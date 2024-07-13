/// monitor layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Chart, Dataset, GraphType, List, ListState, Padding, Paragraph, Row, Table,
        Tabs,
    },
    Frame,
};

use crate::app::app::App;

use super::config::{MonitorPageArea, TABS};

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
        .block(Block::bordered().border_style(Style::default().white()))
        .style(Style::default())
        .highlight_style(Style::default().yellow())
        .select(1)
        .padding("  ", "  ");
    frame.render_widget(tabs, full_layout[0]);
    // ------------------------ full layout 0 area start ------------------------
    // ------------------------ full layout 1 area start ------------------------
    // network device list
    let network_device_list_items = app.monitor_page_name_list.clone();
    let network_device_list = List::new(network_device_list_items)
        .block(Block::bordered().title("Network Devices").border_style(
            if app.monitor_page_selecting_area == MonitorPageArea::Area_1 {
                Style::default().green()
            } else {
                Style::default().gray()
            },
        ))
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
        .block(Block::bordered().title("Network Packet").border_style(
            if app.monitor_page_selecting_area == MonitorPageArea::Area_2 {
                Style::default().green()
            } else {
                Style::default().gray()
            },
        ))
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
        .block(Block::bordered().title("Data Details").border_style(
            if app.monitor_page_selecting_area == MonitorPageArea::Area_3 {
                Style::default().green()
            } else {
                Style::default().gray()
            },
        ))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    frame.render_widget(list, full_layout_2_split[0]);
    // chart
    // Create the datasets to fill the chart with
    let datasets = vec![
        // Scatter chart
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().cyan())
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        // Line chart
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().magenta())
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("X Axis".red())
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Y Axis".red())
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(
            Block::bordered()
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 0,
                })
                .title("Chart")
                .border_style(
                    if app.monitor_page_selecting_area == MonitorPageArea::Area_4 {
                        Style::default().green()
                    } else {
                        Style::default().gray()
                    },
                ),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);
    frame.render_widget(chart, full_layout_2_split[1]);
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
    let paragraph: Paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), no wrap"));

    frame.render_widget(paragraph, full_layout[3]);
    // ------------------------ input area end ------------------------
}
