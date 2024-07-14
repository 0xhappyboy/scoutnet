use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{app::App, ui::config::MonitorPageArea};

pub fn handle_monitor_page_basic_events(app: &mut App, key_event: &mut KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            app.monitor_page_selected_area = app.monitor_page_selecting_area;
            if app.monitor_page_selected_area == MonitorPageArea::Area_2 {
                app.monitor_page_real_time_net_pack_table_state
                    .select(app.monitor_page_real_time_net_pack_table_selected_index);
            }
        }
        KeyCode::Char('\n' | ' ') => {
            if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                app.monitor_page_net_pack_info_tree_state.toggle_selected();
            }
        }
        KeyCode::Up => {
            if app.monitor_page_selected_area != MonitorPageArea::Area_3
                && app.monitor_page_selected_area != MonitorPageArea::Area_2
                && app.monitor_page_selected_area != MonitorPageArea::Area_1
            {
                if (app.monitor_page_selecting_area == MonitorPageArea::Area_3) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_1;
                } else if (app.monitor_page_selecting_area == MonitorPageArea::Area_4) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_2;
                }
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                app.monitor_page_net_pack_info_tree_state.key_up();
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_2 {
                let current_index = app
                    .monitor_page_real_time_net_pack_table_selected_index
                    .unwrap();
                if current_index > 0 {
                    app.monitor_page_real_time_net_pack_table_selected_index =
                        Some(current_index - 1);
                    app.monitor_page_real_time_net_pack_table_state
                        .select(Some(current_index - 1));
                    app.monitor_page_real_time_net_pack_table_scroll_bar_state
                        .prev();
                }
            }

            if app.monitor_page_selected_area == MonitorPageArea::Area_1 {
                let current_index = app.monitor_page_device_table_selected_index.unwrap();
                if current_index > 0 {
                    app.monitor_page_device_table_selected_index = Some(current_index - 1);
                    app.monitor_page_device_table_state
                        .select(Some(current_index - 1));
                    app.monitor_page_device_table_scroll_bar_state.prev();
                }
            }
        }
        KeyCode::Down => {
            if app.monitor_page_selected_area != MonitorPageArea::Area_3
                && app.monitor_page_selected_area != MonitorPageArea::Area_2
                && app.monitor_page_selected_area != MonitorPageArea::Area_1
            {
                if (app.monitor_page_selecting_area == MonitorPageArea::Area_1) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_3;
                } else if (app.monitor_page_selecting_area == MonitorPageArea::Area_2) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_4;
                }
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                app.monitor_page_net_pack_info_tree_state.key_down();
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_2 {
                let current_index = app
                    .monitor_page_real_time_net_pack_table_selected_index
                    .unwrap();
                if current_index < app.monitor_page_real_time_net_pack_table_data.len() - 1 {
                    app.monitor_page_real_time_net_pack_table_selected_index =
                        Some(current_index + 1);
                    app.monitor_page_real_time_net_pack_table_state
                        .select(Some(current_index + 1));

                    app.monitor_page_real_time_net_pack_table_scroll_bar_state
                        .next();
                }
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_1 {
                let current_index = app.monitor_page_device_table_selected_index.unwrap();
                if current_index < app.monitor_page_device_table_data.len() - 1 {
                    app.monitor_page_device_table_selected_index = Some(current_index + 1);
                    app.monitor_page_device_table_state
                        .select(Some(current_index + 1));
                    app.monitor_page_device_table_scroll_bar_state.next();
                }
            }
        }
        KeyCode::Left => {
            if app.monitor_page_selected_area != MonitorPageArea::Area_3
                && app.monitor_page_selected_area != MonitorPageArea::Area_2
            {
                if (app.monitor_page_selecting_area == MonitorPageArea::Area_2) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_1;
                } else if (app.monitor_page_selecting_area == MonitorPageArea::Area_4) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_3;
                }
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                app.monitor_page_net_pack_info_tree_state.key_left();
            }
        }
        KeyCode::Right => {
            if app.monitor_page_selected_area != MonitorPageArea::Area_3
                && app.monitor_page_selected_area != MonitorPageArea::Area_1
            {
                if (app.monitor_page_selecting_area == MonitorPageArea::Area_1) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_2;
                } else if (app.monitor_page_selecting_area == MonitorPageArea::Area_3) {
                    app.monitor_page_selecting_area = MonitorPageArea::Area_4;
                }
            }
            if app.monitor_page_selected_area == MonitorPageArea::Area_3 {
                app.monitor_page_net_pack_info_tree_state.key_right();
            }
        }
        _ => {}
    }
}
