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
    frame.render_widget(render_world_state_panel(snapshot), inner_layout[0]);

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

fn render_world_state_panel(snapshot: &ObserverSnapshot) -> Paragraph {
    let total_entities = snapshot.entities.len();
    let tick = snapshot.tick;

    // Calculate metrics
    let avg_wealth = if total_entities > 0 {
        snapshot.entities.iter().map(|e| e.wealth).sum::<f32>() / total_entities as f32
    } else {
        0.0
    };

    let avg_fame = if total_entities > 0 {
        snapshot.entities.iter().map(|e| e.fame).sum::<f32>() / total_entities as f32
    } else {
        0.0
    };

    // Placeholder for security
    let security = 75.0;

    // Create bar graphs
    let max_bar_width = 20;
    let wealth_bar = "█".repeat(((avg_wealth / 100.0) * max_bar_width as f32).clamp(0.0, max_bar_width as f32) as usize);
    let fame_bar = "█".repeat(((avg_fame / 100.0) * max_bar_width as f32).clamp(0.0, max_bar_width as f32) as usize);
    let security_bar = "█".repeat(((security / 100.0) * max_bar_width as f32).clamp(0.0, max_bar_width as f32) as usize);

    let world_state_text = vec![
        Line::from(format!("Total Entities: {}", total_entities)),
        Line::from(format!("Tick: {}", tick)),
        Line::from(""),
        Line::from(Span::styled("경제", Style::default().bold())),
        Line::from(format!("[{:<20}] {:.1}%", wealth_bar, avg_wealth)),
        Line::from(Span::styled("만족도", Style::default().bold())),
        Line::from(format!("[{:<20}] {:.1}%", fame_bar, avg_fame)),
        Line::from(Span::styled("치안", Style::default().bold())),
        Line::from(format!("[{:<20}] {:.1}%", security_bar, security)),
    ];

    Paragraph::new(world_state_text)
        .block(Block::default().title("World State").borders(Borders::ALL))
}