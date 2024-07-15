use std::{
    collections::HashMap,
    io::{self, Stdout},
    thread,
    time::Duration,
};

use pnet::datalink::NetworkInterface;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{self, Event, KeyEvent},
    },
    widgets::{ScrollbarState, TableState},
    Frame, Terminal,
};
use tokio::{
    join,
    sync::mpsc::{self, Receiver, Sender},
};
use tui_tree_widget::{TreeItem, TreeState};

use super::{
    config::PageIndex,
    events::globalevent::handle_global_basic_events,
    init::{self, Tui},
    ui::monitor::{DEVICE_TABLE_ITEM_HEIGHT, REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT},
};
use super::{
    events::monitorevent::handle_monitor_page_basic_events,
    ui::{self, config::MonitorPageArea},
};
use crate::{
    net::{
        device::{self, Device},
        pack::NetPack,
    },
    task::monitortask::{test1, test2, test3},
};

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App {
    // app
    pub input_text: String,
    pub input_mode: InputMode,
    pub page_index: PageIndex,
    exit: bool,
    // monitor page
    pub monitor_page_selecting_area: MonitorPageArea,
    pub monitor_page_selected_area: MonitorPageArea,
    pub monitor_page_net_pack_info_tree_state: TreeState<&'static str>,
    pub monitor_page_net_pack_info_tree_items: Vec<TreeItem<'static, &'static str>>,
    pub monitor_page_device_table_state: TableState,
    pub monitor_page_device_table_selected_index: Option<usize>,
    pub monitor_page_device_table_data: Vec<NetworkInterface>,
    pub monitor_page_device_table_scroll_bar_state: ScrollbarState,
    pub monitor_page_real_time_net_pack_table_state: TableState,
    pub monitor_page_real_time_net_pack_table_selected_index: Option<usize>,
    pub monitor_page_real_time_net_pack_table_data: Vec<HashMap<String, String>>,
    pub monitor_page_real_time_net_pack_table_scroll_bar_state: ScrollbarState,
    // test
    pub test1: u64,
    pub t_tx: Sender<HashMap<String, String>>,
    pub t_rx: Receiver<HashMap<String, String>>,
}

impl App {
    pub fn new() -> Self {
        let (mut tx, mut rx) = mpsc::channel::<HashMap<String, String>>(32);

        let mut m = HashMap::new();
        m.insert(String::from("k1"), String::from("v1"));
        m.insert(String::from("k2"), String::from("v2"));
        m.insert(String::from("k3"), String::from("v3"));
        m.insert(String::from("k4"), String::from("v3"));
        m.insert(String::from("k5"), String::from("v3"));
        m.insert(String::from("k6"), String::from("v3"));
        m.insert(String::from("k7"), String::from("v3"));

        let mut m2 = HashMap::new();
        m2.insert(String::from("k1"), String::from("vvvvvv1"));
        m2.insert(String::from("k2"), String::from("vvvvvv2"));
        m2.insert(String::from("k3"), String::from("vvvvvv3"));
        m2.insert(String::from("k4"), String::from("vvvvvv4"));
        m2.insert(String::from("k5"), String::from("vvvvvv5"));
        m2.insert(String::from("k6"), String::from("vvvvvqwerqwvv6"));
        m2.insert(
            String::from("k7"),
            String::from("vvvqvqwerqwwerqwerqwervvv7"),
        );
        m2.insert(String::from("k8"), String::from("vvvvvvqwerqwvqwvvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8vvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8vvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8erqwvqwerqwvqwerqwv8"));

        let t_data: Vec<HashMap<String, String>> = vec![m.clone(), m2.clone()];
        let device_name_list = Device::get_device_name_list();
        let device_list: Vec<NetworkInterface> = Device::get_device_list();

        App {
            input_text: String::from(""),
            input_mode: InputMode::Normal,
            page_index: PageIndex::Monitor,
            exit: false,
            monitor_page_selecting_area: MonitorPageArea::Area_1,
            monitor_page_selected_area: MonitorPageArea::None,
            monitor_page_real_time_net_pack_table_state: TableState::default(),
            monitor_page_real_time_net_pack_table_selected_index: Some(0),
            monitor_page_real_time_net_pack_table_scroll_bar_state: ScrollbarState::new(
                (t_data.len() - 1) * REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT,
            ),
            monitor_page_real_time_net_pack_table_data: t_data,
            monitor_page_net_pack_info_tree_items: vec![
                TreeItem::new_leaf("a", "Alfa"),
                TreeItem::new(
                    "b",
                    "Bravo",
                    vec![
                        TreeItem::new_leaf("c", "Charlie"),
                        TreeItem::new(
                            "d",
                            "Delta",
                            vec![
                                TreeItem::new_leaf("e", "Echo"),
                                TreeItem::new_leaf("f", "Foxtrot"),
                            ],
                        )
                        .expect("all item identifiers are unique"),
                        TreeItem::new_leaf("g", "Golf"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("h", "Hotel"),
                TreeItem::new(
                    "i",
                    "India",
                    vec![
                        TreeItem::new_leaf("j", "Juliett"),
                        TreeItem::new_leaf("k", "Kilo"),
                        TreeItem::new_leaf("l", "Lima"),
                        TreeItem::new_leaf("m", "Mike"),
                        TreeItem::new_leaf("n", "November"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("o", "Oscar"),
                TreeItem::new(
                    "p",
                    "Papa",
                    vec![
                        TreeItem::new_leaf("q", "Quebec"),
                        TreeItem::new_leaf("r", "Romeo"),
                        TreeItem::new_leaf("s", "Sierra"),
                        TreeItem::new_leaf("t", "Tango"),
                        TreeItem::new_leaf("u", "Uniform"),
                        TreeItem::new(
                            "v",
                            "Victor",
                            vec![
                                TreeItem::new_leaf("w", "Whiskey"),
                                TreeItem::new_leaf("x", "Xray"),
                                TreeItem::new_leaf("y", "Yankee"),
                            ],
                        )
                        .expect("all item identifiers are unique"),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("z", "Zulu"),
            ],
            monitor_page_net_pack_info_tree_state: TreeState::default(),
            monitor_page_device_table_state: TableState::default(),
            monitor_page_device_table_selected_index: Some(0),
            monitor_page_device_table_scroll_bar_state: ScrollbarState::new(
                (device_list.len() - 1) * DEVICE_TABLE_ITEM_HEIGHT,
            ),
            monitor_page_device_table_data: device_list,
            test1: 1,
            t_tx: tx,
            t_rx: rx,
        }
    }

    pub async fn run(&mut self, terminal: &mut init::Tui) -> io::Result<()> {
        tokio::spawn(test1());
        tokio::spawn(test2());
        tokio::spawn(test3());

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            let _ = self.handle_events();
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        match self.page_index {
            PageIndex::Welcome => {
                ui::welcome::layout(self, frame);
            }
            PageIndex::Monitor => {
                ui::monitor::layout(self, frame);
            }
            PageIndex::Safe => {
                ui::safe::layout(self, frame);
            }
            PageIndex::Http => {
                ui::http::layout(self, frame);
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read().unwrap() {
            Event::Key(mut key_event) => self.handle_key_event(&mut key_event),
            _ => return Ok(()),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &mut KeyEvent) {
        handle_global_basic_events(self, key_event);
        match self.page_index {
            PageIndex::Welcome => {}
            PageIndex::Monitor => {
                handle_monitor_page_basic_events(self, key_event);
            }
            PageIndex::Safe => {}
            PageIndex::Http => {}
        }

        // match self.input_mode {
        //     InputMode::Normal => match key_event.code {
        //         KeyCode::Char('e') => self.input_mode = InputMode::Editing,
        //         KeyCode::Char('q') => self.exit(),
        //         KeyCode::Char('1') => self.page_index = PageIndex::Welcome,
        //         KeyCode::Char('2') => self.page_index = PageIndex::Home,
        //         KeyCode::Char('3') => self.page_index = PageIndex::Safe,
        //         KeyCode::Char('4') => self.page_index = PageIndex::Test,
        //         KeyCode::Esc => self.exit(),
        //         _ => {}
        //     },
        //     InputMode::Editing => match key_event.code {
        //         KeyCode::Esc => {
        //             self.input_mode = InputMode::Normal;
        //             self.input_text.clear();
        //         }
        //         KeyCode::Char(insert) => self.input_text.push(insert),
        //         KeyCode::Backspace => {
        //             if self.input_text.len() > 0 {
        //                 let removed = &self.input_text[0..self.input_text.len() - 1];
        //                 self.input_text = removed.to_owned();
        //             }
        //         }
        //         _ => {}
        //     },
        // }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
