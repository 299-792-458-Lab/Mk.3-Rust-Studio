use std::time::Duration;

use tracing_subscriber::EnvFilter;

mod simulation;

use simulation::{SimulationConfig, SimulationWorld};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_tracing();

    let mut config = SimulationConfig::default();
    config.tick_duration = Duration::from_secs(1);

    let tick_duration = config.tick_duration;
    let mut simulation = SimulationWorld::new(config);
    let mut interval = tokio::time::interval(tick_duration);

    let max_ticks: u64 = 10;
    while simulation.tick_count() < max_ticks {
        interval.tick().await;
        simulation.tick();
    }

    Ok(())
}

fn initialize_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();
}
