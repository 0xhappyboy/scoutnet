/// welcome layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::app::{self, App};

pub fn layout(app: &App, frame: &mut Frame) {
    // full layout
    let full_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(frame.size());
    // ------------------------ full layout 0 area start ------------------------
    let tabs = Tabs::new(vec!["Home", "Monitor", "Safe", "Test"])
        .block(Block::bordered())
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(0)
        .padding("  ", "  ");
    frame.render_widget(tabs, full_layout[0]);
    // ------------------------ full layout 0 area start ------------------------
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        full_layout[1],
    );
    // ------------------------ input area start ------------------------
    let text: Vec<Line> = vec![Line::from(app.input_text.to_string())];
    let create_block = |title| {
        Block::bordered()
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), no wrap"));
    frame.render_widget(paragraph, full_layout[2]);
    // ------------------------ input area end ------------------------
}
