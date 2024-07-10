/// home layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::app::App;

pub fn layout(app: &App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
        ])
        .split(frame.size());
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
}
