use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{app::App, ui::config::MonitorPageArea},
    data::monitordata::{
        monitor_page_device_table_data, monitor_page_device_table_scroll_bar_state,
        monitor_page_device_table_selected_index, monitor_page_device_table_state,
        monitor_page_net_pack_info_tree_state, monitor_page_real_time_net_pack_table_data,
        monitor_page_real_time_net_pack_table_scroll_bar_state,
        monitor_page_real_time_net_pack_table_selected_index,
        monitor_page_real_time_net_pack_table_state, monitor_page_selected_area,
        monitor_page_selecting_area,
    },
};

pub fn handle_monitor_page_basic_events(key_event: &mut KeyEvent) {
    match key_event.code {
        KeyCode::Enter => {
            *monitor_page_selected_area.lock().unwrap() =
                *monitor_page_selecting_area.lock().unwrap();
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                monitor_page_real_time_net_pack_table_state
                    .lock()
                    .unwrap()
                    .select(
                        *monitor_page_real_time_net_pack_table_selected_index
                            .lock()
                            .unwrap(),
                    );
            }
        }
        KeyCode::Char('\n' | ' ') => {
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                monitor_page_net_pack_info_tree_state
                    .lock()
                    .unwrap()
                    .toggle_selected();
            }
        }
        KeyCode::Up => {
            if *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_2
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_3) {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_1;
                } else if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_4)
                {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_2;
                }
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                monitor_page_net_pack_info_tree_state
                    .lock()
                    .unwrap()
                    .key_up();
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                let current_index = monitor_page_real_time_net_pack_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index > 0 {
                    *monitor_page_real_time_net_pack_table_selected_index
                        .lock()
                        .unwrap() = Some(current_index - 1);
                    monitor_page_real_time_net_pack_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index - 1));
                    monitor_page_real_time_net_pack_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .prev();
                }
            }

            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_1 {
                let current_index = monitor_page_device_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index > 0 {
                    *monitor_page_device_table_selected_index.lock().unwrap() =
                        Some(current_index - 1);
                    monitor_page_device_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index - 1));
                    monitor_page_device_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .prev();
                }
            }
        }
        KeyCode::Down => {
            if *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_2
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_1) {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_3;
                } else if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_2)
                {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_4;
                }
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                monitor_page_net_pack_info_tree_state
                    .lock()
                    .unwrap()
                    .key_down();
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_2 {
                let current_index = monitor_page_real_time_net_pack_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index
                    < monitor_page_real_time_net_pack_table_data
                        .lock()
                        .unwrap()
                        .len()
                        - 1
                {
                    *monitor_page_real_time_net_pack_table_selected_index
                        .lock()
                        .unwrap() = Some(current_index + 1);
                    monitor_page_real_time_net_pack_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index + 1));

                    monitor_page_real_time_net_pack_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .next();
                }
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_1 {
                let current_index = monitor_page_device_table_selected_index
                    .lock()
                    .unwrap()
                    .unwrap();
                if current_index < monitor_page_device_table_data.lock().unwrap().len() - 1 {
                    *monitor_page_device_table_selected_index.lock().unwrap() =
                        Some(current_index + 1);
                    monitor_page_device_table_state
                        .lock()
                        .unwrap()
                        .select(Some(current_index + 1));
                    monitor_page_device_table_scroll_bar_state
                        .lock()
                        .unwrap()
                        .next();
                }
            }
        }
        KeyCode::Left => {
            if *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_2
            {
                if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_2) {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_1;
                } else if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_4)
                {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_3;
                }
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                monitor_page_net_pack_info_tree_state
                    .lock()
                    .unwrap()
                    .key_left();
            }
        }
        KeyCode::Right => {
            if *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_3
                && *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::Area_1
            {
                if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_1) {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_2;
                } else if (*monitor_page_selecting_area.lock().unwrap() == MonitorPageArea::Area_3)
                {
                    *monitor_page_selecting_area.lock().unwrap() = MonitorPageArea::Area_4;
                }
            }
            if *monitor_page_selected_area.lock().unwrap() == MonitorPageArea::Area_3 {
                monitor_page_net_pack_info_tree_state
                    .lock()
                    .unwrap()
                    .key_right();
            }
        }
        _ => {}
    }
}
