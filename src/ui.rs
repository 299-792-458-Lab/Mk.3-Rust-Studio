use crate::simulation::ObserverSnapshot;
use ratatui::
{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, snapshot: &ObserverSnapshot) {
    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());

    // Header
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(" Mk.3 Rust Studio - TERA "),
        main_layout[0],
    );

    // Inner layout for content
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout[1]);

    // World State Panel
    let world_state_text = format!(
        "Total Entities: {}\nTick: {}",
        snapshot.entities.len(),
        snapshot.tick,
    );
    let state_widget = Paragraph::new(world_state_text)
        .block(Block::default().title("World State").borders(Borders::ALL));
    frame.render_widget(state_widget, inner_layout[0]);

    // Event Log Panel
    let events: Vec<String> = snapshot
        .events
        .iter()
        .rev() // Show latest events at the top
        .take(20) // Limit number of events shown
        .map(|e| format!("{:?}", e))
        .collect();

    let event_log = Paragraph::new(events.join("\n"))
        .block(Block::default().title("Event Log").borders(Borders::ALL));
    frame.render_widget(event_log, inner_layout[1]);
}
