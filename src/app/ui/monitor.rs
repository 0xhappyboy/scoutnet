/// monitor layout
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::*,
    Frame,
};
use tui_tree_widget::Tree;

use crate::app::app::App;

use super::config::{MonitorPageArea, TABS};

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
    let mut device_list: Vec<Line> = vec![];
    for (i, v) in app.monitor_page_device_list.iter().enumerate() {
        let line: Line = vec![
            v.name.clone().green().bold(),
            "   ".into(),
            v.mac.clone().unwrap().to_string().yellow(),
            "   ".into(),
            v.flags.clone().to_string().cyan(),
        ]
        .into();
        device_list.push(line);
    }
    let network_device_list = List::new(device_list)
        .block(Block::bordered().title("Network Devices").border_style(
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
        ))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    // real-time network pack list
    // build row
    let mut rows: Vec<Row> = vec![];
    for (i, v) in app
        .monitor_page_real_time_net_pack_table_data
        .iter()
        .enumerate()
    {
        rows.push(Row::new(vec![
            v.get("k1").unwrap().to_string(),
            v.get("k2").unwrap().to_string(),
            v.get("k3").unwrap().to_string(),
        ]))
    }
    // table width
    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(50),
    ];
    // table header
    let header = ["Name", "Address", "Email"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);
    let table = Table::new(rows, widths)
        .header(header)
        .footer(Row::new(vec!["aaa", "bbb", "ccc"]))
        .block(Block::bordered().title("Network Packet").border_style(
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
        ))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

    let full_layout_1_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(full_layout[1]);
    frame.render_widget(network_device_list, full_layout_1_split[0]);

    frame.render_stateful_widget(
        table,
        full_layout_1_split[1],
        &mut app.monitor_page_real_time_net_pack_table_state,
    );
    // ------------------------ full layout 1 area end ------------------------
    // ------------------------ full layout 2 area start ------------------------
    let full_layout_2_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(full_layout[2]);
    // net pack tree info
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
        full_layout_2_split[0],
        &mut app.monitor_page_net_pack_info_tree_state,
    );

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
