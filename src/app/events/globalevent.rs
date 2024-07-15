use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{app::App, config::PageIndex, ui::config::MonitorPageArea},
    data::{
        appdata::page_index,
        monitordata::{
            monitor_page_net_pack_info_tree_state, monitor_page_real_time_net_pack_table_data,
            monitor_page_real_time_net_pack_table_state, monitor_page_selected_area,
        },
    },
};

pub fn handle_global_basic_events(key_event: &mut KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            if *page_index.lock().unwrap() == PageIndex::Monitor {
                if *monitor_page_selected_area.lock().unwrap() != MonitorPageArea::None {
                    monitor_page_net_pack_info_tree_state
                        .lock()
                        .unwrap()
                        .close_all();
                    monitor_page_real_time_net_pack_table_state
                        .lock()
                        .unwrap()
                        .select(Some(
                            monitor_page_real_time_net_pack_table_data
                                .lock()
                                .unwrap()
                                .len()
                                - 1,
                        ));
                    *monitor_page_selected_area.lock().unwrap() = MonitorPageArea::None;
                } else {
                    App::exit();
                }
            } else {
                App::exit();
            }
        }
        KeyCode::Char('1') => {
            *page_index.lock().unwrap() = PageIndex::Welcome;
        }
        KeyCode::Char('2') => {
            *page_index.lock().unwrap() = PageIndex::Monitor;
        }
        KeyCode::Char('3') => {
            *page_index.lock().unwrap() = PageIndex::Safe;
        }
        KeyCode::Char('4') => {
            *page_index.lock().unwrap() = PageIndex::Http;
        }
        _ => {}
    }
}
