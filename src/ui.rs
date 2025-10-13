use crate::simulation::events::{Sentiment, WorldEventKind};
use crate::simulation::{Nation, NationMetrics, ObserverSnapshot};
use ratatui::{
    prelude::*,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use std::time::Duration;

pub fn render(frame: &mut Frame, snapshot: &ObserverSnapshot, tick_duration: Duration) {
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
    frame.render_widget(render_world_state_panel(snapshot, tick_duration), inner_layout[0]);

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

use crate::simulation::{Nation, NationMetrics};

fn render_world_state_panel(snapshot: &ObserverSnapshot, tick_duration: Duration) -> Paragraph {
    let total_entities = snapshot.entities.len();
    let tick = snapshot.tick;

    let mut world_state_lines = vec![
        Line::from(format!("Total Entities: {}", total_entities)),
        Line::from(format!("Tick: {}", tick)),
        Line::from(""),
    ];

    let mut nations: Vec<_> = snapshot.all_metrics.keys().collect();
    nations.sort_by_key(|a| a.name()); // Sort for consistent order

    for nation in nations {
        if let Some(metrics) = snapshot.all_metrics.get(nation) {
            world_state_lines.push(Line::from(Span::styled(
                nation.name(),
                Style::default().bold().underlined(),
            )));

            if metrics.is_destroyed {
                world_state_lines.push(Line::from(Span::styled(
                    "-- 멸망 --",
                    Style::default().fg(Color::Red).italic(),
                )));
            } else {
                world_state_lines.push(Line::from(Span::styled("  경제", Style::default())));
                world_state_lines.push(create_bar(metrics.economy, 100.0, 18));
                world_state_lines.push(Line::from(Span::styled("  만족도", Style::default())));
                world_state_lines.push(create_bar(metrics.satisfaction, 100.0, 18));
                world_state_lines.push(Line::from(Span::styled("  치안", Style::default())));
                world_state_lines.push(create_bar(metrics.security, 100.0, 18));
                world_state_lines.push(Line::from(Span::styled("  군사력", Style::default())));
                world_state_lines.push(create_bar(metrics.military, 100.0, 18));
                world_state_lines.push(Line::from(Span::styled("  영토", Style::default())));
                world_state_lines.push(create_bar(metrics.territory, 100.0, 18));
            }
            world_state_lines.push(Line::from(""));
        }
    }

    world_state_lines.push(Line::from(Span::styled(
        "Tick Speed",
        Style::default().bold(),
    )));
    world_state_lines.push(Line::from(format!("{} ms/tick", tick_duration.as_millis())));
    world_state_lines.push(Line::from(vec![
        Span::from("["),
        Span::styled("-", Style::default().fg(Color::Red).bold()),
        Span::from("] ["),
        Span::styled("+", Style::default().fg(Color::Green).bold()),
        Span::from("] ["),
        Span::styled("R", Style::default().fg(Color::Yellow).bold()),
        Span::from("]"),
    ]));

    Paragraph::new(world_state_lines)
        .block(Block::default().title("World State").borders(Borders::ALL))
}

fn create_bar(value: f32, max_value: f32, max_width: usize) -> Line<'static> {
    let percentage = (value / max_value).clamp(0.0, 1.0);
    let width = (percentage * max_width as f32) as usize;
    let bar_text = "█".repeat(width);
    let padding = " ".repeat(max_width - width);

    let color = if percentage > 0.66 {
        Color::Green
    } else if percentage > 0.33 {
        Color::Yellow
    } else {
        Color::Red
    };

    let bar_span = Span::styled(bar_text, Style::default().fg(color));
    let padding_span = Span::raw(padding);
    let text_span = Span::from(format!(" {:.1}%", percentage * 100.0));

    Line::from(vec![
        Span::raw("["),
        bar_span,
        padding_span,
        Span::raw("]"),
        text_span,
    ])
}