/// home layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListState, Paragraph, RenderDirection, Row, Sparkline, Table, Tabs,
    },
    Frame,
};

use crate::app::app::App;
use crate::app::ui::config;

use super::config::TABS;

pub fn layout(app: &App, frame: &mut Frame) {
    // full layout
    let full_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ])
        .split(frame.size());
    // ------------------------ full layout 0 area start ------------------------
    let tabs = Tabs::new(TABS)
        .block(Block::bordered())
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(3)
        .padding("  ", "  ");
    frame.render_widget(tabs, full_layout[0]);
    // ------------------------ full layout 0 area start ------------------------
    // ------------------------ full layout 1 area start ------------------------
    // network device list
    let mut network_device_list_state = ListState::default();
    let network_device_list_items = ["device 1", "device 2", "device 3", "device 4", "device 5"];
    let network_device_list = List::new(network_device_list_items)
        .block(Block::bordered().title("Network Devices"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    // real-time network packet list
    let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
    // Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1)
        // You can set the style of the entire Table.
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::new().bold())
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        .footer(Row::new(vec!["Updated on Dec 28"]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::bordered().title("Network Packet"))
        // The selected row and its content can also be styled.
        .highlight_style(Style::new().reversed())
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");

    let full_layout_1_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(full_layout[1]);
    frame.render_widget(network_device_list, full_layout_1_split[0]);
    frame.render_widget(table, full_layout_1_split[1]);
    // ------------------------ full layout 1 area end ------------------------
    // ------------------------ full layout 2 area start ------------------------
    let full_layout_2_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(full_layout[2]);
    // net package info
    let mut state = ListState::default();
    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(Block::bordered().title("Data Details"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    frame.render_widget(list, full_layout_2_split[0]);
    // Flow
    let flow = Sparkline::default()
        .block(Block::bordered().title("Flow"))
        .data(&[0, 2, 3, 4, 1, 4, 10])
        .max(5)
        .direction(RenderDirection::RightToLeft)
        .style(Style::default().red().on_white());
    frame.render_widget(flow, full_layout_2_split[1]);
    // ------------------------ full layout 2 area start ------------------------
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
    frame.render_widget(paragraph, full_layout[3]);
    // ------------------------ input area end ------------------------
}
