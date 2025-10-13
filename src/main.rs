use std::io::{self, stdout};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, Terminal};
use tokio::sync::Notify;

mod simulation;
mod ui;

use simulation::{ObserverSnapshot, SimulationConfig, SimulationWorld};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Simulation Setup
    let config = SimulationConfig {
        tick_duration: Duration::from_secs(1),
        ..Default::default()
    };
    let tick_duration = config.tick_duration;

    let observer = Arc::new(RwLock::new(ObserverSnapshot::default()));
    let shutdown_notify = Arc::new(Notify::new());

    let mut simulation = SimulationWorld::with_observer(config, observer.clone());
    let notify_for_simulation = shutdown_notify.clone();
    let simulation_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tick_duration);
        loop {
            tokio::select! {
                _ = interval.tick() => simulation.tick(),
                _ = notify_for_simulation.notified() => break,
            }
        }
    });

    // TUI Setup
    let mut terminal = init_terminal()?;
    let mut app_should_run = true;

    while app_should_run {
        terminal.draw(|frame| {
            let snapshot = observer.read().expect("Observer lock is poisoned").clone();
            ui::render(frame, &snapshot);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app_should_run = false;
                }
            }
        }
    }

    // Shutdown
    shutdown_notify.notify_waiters();
    simulation_task.await?;
    restore_terminal()?;

    Ok(())
}

fn init_terminal() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::new(backend)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}