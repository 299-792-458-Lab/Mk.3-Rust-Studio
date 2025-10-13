use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;
use tokio::sync::Notify;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod simulation;

use simulation::{ObserverSnapshot, SimulationConfig, SimulationWorld, WorldEvent};

#[derive(Clone)]
struct AppState {
    observer: Arc<RwLock<ObserverSnapshot>>,
}

#[derive(Debug, Deserialize)]
struct EventsQuery {
    limit: Option<usize>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_tracing();

    let mut config = SimulationConfig::default();
    config.tick_duration = Duration::from_secs(1);
    let tick_duration = config.tick_duration;

    let observer = Arc::new(RwLock::new(ObserverSnapshot::default()));
    let shutdown_notify = Arc::new(Notify::new());

    let observer_for_sim = observer.clone();
    let notify_for_simulation = shutdown_notify.clone();

    let mut simulation = SimulationWorld::with_observer(config, observer_for_sim);
    let simulation_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tick_duration);
        loop {
            tokio::select! {
                _ = interval.tick() => simulation.tick(),
                _ = notify_for_simulation.notified() => break,
            }
        }
    });

    let app_state = AppState {
        observer: observer.clone(),
    };

    let router = Router::new()
        .route("/world/state", get(world_state_handler))
        .route("/world/events", get(world_events_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("TERA API listening on http://127.0.0.1:8080");

    let notify_for_server = shutdown_notify.clone();
    let server_result = axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            if let Err(err) = tokio::signal::ctrl_c().await {
                warn!("ctrl-c signal listener failed: {err}");
            }
            notify_for_server.notify_waiters();
        })
        .await;

    // Ensure the simulation loop halts even if the server exits unexpectedly.
    shutdown_notify.notify_waiters();
    simulation_task.await?;
    server_result?;

    Ok(())
}

async fn world_state_handler(State(state): State<AppState>) -> Json<ObserverSnapshot> {
    let snapshot = {
        let guard = state
            .observer
            .read()
            .expect("observer snapshot lock poisoned");
        guard.clone()
    };
    Json(snapshot)
}

async fn world_events_handler(
    State(state): State<AppState>,
    Query(params): Query<EventsQuery>,
) -> Json<Vec<WorldEvent>> {
    let mut events = {
        let guard = state
            .observer
            .read()
            .expect("observer snapshot lock poisoned");
        guard.events.clone()
    };

    if let Some(limit) = params.limit {
        if events.len() > limit {
            events = events.split_off(events.len() - limit);
        }
    }

    Json(events)
}

fn initialize_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();
}
