use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{app::App, config::PageIndex, ui::config::MonitorPageArea};

pub fn handle_global_basic_events(app: &mut App, key_event: &mut KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            if (app.page_index == PageIndex::Monitor) {
                if app.monitor_page_selected_area != MonitorPageArea::None {
                    app.monitor_page_net_pack_info_tree_state.close_all();
                    app.monitor_page_real_time_net_pack_table_state.select(Some(
                        app.monitor_page_real_time_net_pack_table_data.len() - 1,
                    ));
                    app.monitor_page_selected_area = MonitorPageArea::None;
                } else {
                    app.exit();
                }
            } else {
                app.exit();
            }
        }
        KeyCode::Char('1') => {
            app.page_index = PageIndex::Welcome;
        }
        KeyCode::Char('2') => {
            app.page_index = PageIndex::Monitor;
        }
        KeyCode::Char('3') => {
            app.page_index = PageIndex::Safe;
        }
        KeyCode::Char('4') => {
            app.page_index = PageIndex::Http;
        }
        _ => {}
    }
}
