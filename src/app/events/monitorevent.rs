use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::ui::config::MonitorPageArea, data::monitordata::*};

pub fn handle_monitor_page_basic_events(key_event: &mut KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            *selected_area.lock().unwrap() = *selecting_area.lock().unwrap();
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                real_time_net_pack_table_state
                    .lock()
                    .unwrap()
                    .select(*real_time_net_pack_table_selected_index.lock().unwrap());
            }
        }
        KeyCode::Char('\n' | ' ') => {
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                net_pack_info_tree_state.lock().unwrap().toggle_selected();
            }
        }
        KeyCode::Up => {
            if *selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_2
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_3) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_1;
                } else if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_4) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_2;
                }
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                net_pack_info_tree_state.lock().unwrap().key_up();
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                let current_index = real_time_net_pack_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index > 0 {
                    *real_time_net_pack_table_selected_index.lock().unwrap() =
                        Some(current_index - 1);
                    real_time_net_pack_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index - 1));
                    real_time_net_pack_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .prev();
                }
            }

            if *selected_area.lock().unwrap() == MonitorPageArea::Area_1 {
                let current_index = device_table_selected_index.lock().unwrap().unwrap();
                if current_index > 0 {
                    *device_table_selected_index.lock().unwrap() = Some(current_index - 1);
                    device_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index - 1));
                    device_table_scroll_bar_state.lock().unwrap().prev();
                }
            }
        }
        KeyCode::Down => {
            if *selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_2
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_1) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_3;
                } else if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_2) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_4;
                }
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                net_pack_info_tree_state.lock().unwrap().key_down();
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                let current_index = real_time_net_pack_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index < real_time_net_pack_table_data.lock().unwrap().len() - 1 {
                    *real_time_net_pack_table_selected_index.lock().unwrap() =
                        Some(current_index + 1);
                    real_time_net_pack_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index + 1));

                    real_time_net_pack_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .next();
                }
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_1 {
                let current_index = device_table_selected_index.lock().unwrap().unwrap();
                if current_index < device_table_data.lock().unwrap().len() - 1 {
                    *device_table_selected_index.lock().unwrap() = Some(current_index + 1);
                    device_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index + 1));
                    device_table_scroll_bar_state.lock().unwrap().next();
                }
            }
        }
        KeyCode::Left => {
            if *selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_2
            {
                if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_2) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_1;
                } else if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_4) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_3;
                }
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                net_pack_info_tree_state.lock().unwrap().key_left();
            }
        }
        KeyCode::Right => {
            if *selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_1) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_2;
                } else if (*selecting_area.lock().unwrap() == MonitorPageArea::Area_3) {
                    *selecting_area.lock().unwrap() = MonitorPageArea::Area_4;
                }
            }
            if *selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                net_pack_info_tree_state.lock().unwrap().key_right();
            }
        }
        _ => {}
    }
}
