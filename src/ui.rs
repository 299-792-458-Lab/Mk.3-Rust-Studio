use crate::simulation::events::{Sentiment, WorldEventKind};
use crate::simulation::ObserverSnapshot;
use ratatui::{
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
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

    // Inner layout for content - Adjusted panel widths
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
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

    // Event Log Panel - Using a Table for alignment
    let header_cells = [
        "Tick",
        "Category",
        "Actor/Source",
        "Details",
        "Impact/Level",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::White).bold()));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows: Vec<Row> = snapshot
        .events
        .iter()
        .rev()
        .take(20)
        .map(|event| {
            let color = match event.sentiment() {
                Sentiment::Positive => Color::Green,
                Sentiment::Neutral => Color::Yellow,
                Sentiment::Negative => Color::Red,
            };
            let style = Style::default().fg(color);

            let (actor, details, impact) = match &event.kind {
                WorldEventKind::Trade { actor, trade_focus, market_pressure } => (
                    actor.name.clone(),
                    trade_focus.clone(),
                    market_pressure.clone(),
                ),
                WorldEventKind::Social { convener, gathering_theme, cohesion_level } => (
                    convener.name.clone(),
                    gathering_theme.clone(),
                    cohesion_level.clone(),
                ),
                WorldEventKind::MacroShock { stressor, catalyst, projected_impact } => (
                    stressor.clone(),
                    catalyst.clone(),
                    projected_impact.clone(),
                ),
            };

            let cells = vec![
                Cell::from(event.tick.to_string()),
                Cell::from(event.category()),
                Cell::from(actor),
                Cell::from(details),
                Cell::from(impact),
            ];

            Row::new(cells).height(1).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Min(20),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(Block::default().title("Event Log").borders(Borders::ALL));

    frame.render_widget(table, inner_layout[1]);
}