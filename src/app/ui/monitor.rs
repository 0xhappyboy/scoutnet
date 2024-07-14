/// monitor layout
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    symbols,
    text::{Line, Span},
    widgets::*,
    Frame,
};
use tui_tree_widget::Tree;

use crate::app::app::App;

use super::config::{MonitorPageArea, TABS};

// real time net pack table item height
pub const REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT: usize = 1;
pub const DEVICE_TABLE_ITEM_HEIGHT: usize = 1;

const REAL_TIME_PACK_TABLE_HEADER: [&str; 7] = [
    "No",
    "Time",
    "Source",
    "Destination",
    "Protocol",
    "Length",
    "Info",
];

const DEVICE_TABLE_HEADER: [&str; 3] = ["Name", "Mac", "Flag"];

pub fn layout(app: &mut App, frame: &mut Frame) {
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
    render_tab_area(app, frame, full_layout[0]);
    // ------------------------ full layout 0 area start ------------------------
    // ------------------------ full layout 1 area start ------------------------
    let full_layout_1_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(full_layout[1]);
    // render net device list area
    render_network_device_table_area(app, frame, full_layout_1_split[0]);
    // render real time net pack area
    render_real_time_net_pack_area(app, frame, full_layout_1_split[1]);
    // ------------------------ full layout 1 area end ------------------------
    // ------------------------ full layout 2 area start ------------------------
    let full_layout_2_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(full_layout[2]);
    // net pack tree info
    render_net_pack_tree_info_area(app, frame, full_layout_2_split[0]);
    // chart hex area
    render_hex_area(app, frame, full_layout_2_split[1]);
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

fn render_hex_area(app: &mut App, frame: &mut Frame, area: Rect) {
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
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_4 {
                            style.green()
                        } else {
                            style
                        }
                    } else {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_4 {
                            style.green()
                        } else {
                            style.gray()
                        }
                    },
                ),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);
    frame.render_widget(chart, area);
}

fn render_tab_area(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(TABS)
        .block(Block::bordered().border_style(Style::default().white()))
        .style(Style::default())
        .highlight_style(Style::default().yellow())
        .select(1)
        .padding("  ", "  ");
    frame.render_widget(tabs, area);
}

fn render_net_pack_tree_info_area(app: &mut App, frame: &mut Frame, area: Rect) {
    let net_pack_tree_info = Tree::new(&app.monitor_page_net_pack_info_tree_items)
        .expect("all item identifiers are unique")
        .block(
            Block::bordered()
                .border_style(
                    if app.monitor_page_selecting_area == MonitorPageArea::Area_3 {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                            style.green()
                        } else {
                            style
                        }
                    } else {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                            style.green()
                        } else {
                            style.gray()
                        }
                    },
                )
                .title("Pack Info"),
        )
        .experimental_scrollbar(Some(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .track_symbol(None)
                .end_symbol(None),
        ))
        .highlight_style(
            Style::new()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    frame.render_stateful_widget(
        net_pack_tree_info,
        area,
        &mut app.monitor_page_net_pack_info_tree_state,
    );
}

fn render_real_time_net_pack_area(app: &mut App, frame: &mut Frame, area: Rect) {
    // build row
    let mut rows: Vec<Row> = vec![];
    for (i, v) in app
        .monitor_page_real_time_net_pack_table_data
        .iter()
        .enumerate()
    {
        rows.push(
            Row::new(vec![
                v.get("k1").unwrap().to_string(),
                v.get("k2").unwrap().to_string(),
                v.get("k3").unwrap().to_string(),
                v.get("k4").unwrap().to_string(),
                v.get("k5").unwrap().to_string(),
                v.get("k6").unwrap().to_string(),
                v.get("k7").unwrap().to_string(),
            ])
            .height(REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT.try_into().unwrap()),
        )
    }
    // table width
    let widths = [
        Constraint::Percentage(8),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(8),
        Constraint::Percentage(8),
        Constraint::Percentage(46),
    ];
    // table header
    let header = REAL_TIME_PACK_TABLE_HEADER
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT.try_into().unwrap())
        .style(Style::default().bg(Color::LightBlue));
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::bordered()
                .title(format!(
                    "Network Packet ({})",
                    app.monitor_page_real_time_net_pack_table_data.len()
                ))
                .border_style(
                    if app.monitor_page_selecting_area == MonitorPageArea::Area_2 {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_2 {
                            style.green()
                        } else {
                            style
                        }
                    } else {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_2 {
                            style.green()
                        } else {
                            style.gray()
                        }
                    },
                )
                .padding(Padding {
                    left: 0,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        )
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");
    frame.render_stateful_widget(
        table,
        area,
        &mut app.monitor_page_real_time_net_pack_table_state,
    );
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.monitor_page_real_time_net_pack_table_scroll_bar_state,
    );
}

fn render_network_device_table_area(app: &mut App, frame: &mut Frame, area: Rect) {
    // build row
    let mut rows: Vec<Row> = vec![];
    for (i, v) in app.monitor_page_device_table_data.iter().enumerate() {
        rows.push(
            Row::new(vec![
                v.name
                    .clone()
                    .to_string()
                    .set_style(Style::default().green()),
                v.mac
                    .clone()
                    .unwrap()
                    .to_string()
                    .set_style(Style::default().yellow()),
                v.flags
                    .clone()
                    .to_string()
                    .set_style(Style::default().cyan()),
            ])
            .height(DEVICE_TABLE_ITEM_HEIGHT.try_into().unwrap()),
        )
    }
    // table width
    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ];
    // table header
    let header = DEVICE_TABLE_HEADER
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(DEVICE_TABLE_ITEM_HEIGHT.try_into().unwrap())
        .style(Style::default().bg(Color::LightBlue));
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::bordered()
                .title(format!(
                    "Network Packet ({}/{})",
                    (app.monitor_page_device_table_selected_index.unwrap() + 1),
                    app.monitor_page_device_table_data.len()
                ))
                .border_style(
                    if app.monitor_page_selecting_area == MonitorPageArea::Area_1 {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_1 {
                            style.green()
                        } else {
                            style
                        }
                    } else {
                        let style = Style::default().bold();
                        if app.monitor_page_selected_area == MonitorPageArea::Area_1 {
                            style.green()
                        } else {
                            style.gray()
                        }
                    },
                )
                .padding(Padding {
                    left: 0,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        )
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");
    frame.render_stateful_widget(table, area, &mut app.monitor_page_device_table_state);
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.monitor_page_device_table_scroll_bar_state,
    );
}
