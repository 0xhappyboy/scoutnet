/// welcome layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::app::app::{self, App};

pub fn layout(app: &App, f: &mut Frame) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::new().black();
    f.render_widget(block, size);

    let layout = Layout::vertical([Constraint::Ratio(1, 4); 4]).split(size);

    let text = vec![
        Line::from(app.input_text.to_string()),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_blue()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.on_green()),
        Line::from("This is a line".green().italic()),
        Line::from(vec![
            "Masked text: ".into(),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
    ];

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
    f.render_widget(paragraph, layout[0]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), with wrap"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout[1]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Right alignment, with wrap"))
        .right_aligned()
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout[2]);
}
