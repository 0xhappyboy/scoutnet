use std::{
    collections::HashMap,
    io::{self, Stdout},
    thread,
    time::Duration,
};

use pnet::datalink::NetworkInterface;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent},
    Frame,
};

use super::{
    config::PageIndex,
    events::globalevent::handle_global_basic_events,
    init::{self},
};
use super::{
    events::monitorevent::handle_monitor_page_basic_events,
    ui::{self},
};
use crate::data::monitordata::real_time_net_pack_table_data;
use crate::{
    data::{
        appdata::{exit, page_index},
        monitordata::device_table_data,
    },
    net::{device::Device, pack::NetPack},
    task::monitortask::get_real_time_net_pack,
};
#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App {
    // app
    pub input_mode: InputMode,
    // monitor page
}

impl App {
    pub fn new() -> Self {
        let device_name_list = Device::get_device_name_list();
        let device_list: Vec<NetworkInterface> = Device::get_device_list();
        *device_table_data.lock().unwrap() = device_list;
        App {
            input_mode: InputMode::Normal,
        }
    }

    pub async fn run(&mut self, terminal: &mut init::Tui) -> io::Result<()> {
        tokio::spawn(async {
            NetPack::listen("en0".to_string()).await;
        });
        tokio::spawn(async {
            while !*exit.lock().unwrap() {
                let _ = App::handle_events();
            }
        });
        while !*exit.lock().unwrap() {
            let _ = terminal.draw(|frame| App::render_frame(frame));
        }
        Ok(())
    }

    fn render_frame(frame: &mut Frame) {
        match *page_index.lock().unwrap() {
            PageIndex::Welcome => {
                ui::welcome::layout(frame);
            }
            PageIndex::Monitor => {
                ui::monitor::layout(frame);
            }
            PageIndex::Safe => {
                ui::safe::layout(frame);
            }
            PageIndex::Http => {
                ui::http::layout(frame);
            }
        }
    }

    fn handle_events() -> io::Result<()> {
        match event::read().unwrap() {
            Event::Key(mut key_event) => App::handle_key_event(&mut key_event),
            _ => return Ok(()),
        };
        Ok(())
    }

    fn handle_key_event(key_event: &mut KeyEvent) {
        handle_global_basic_events(key_event);
        match *page_index.lock().unwrap() {
            PageIndex::Welcome => {}
            PageIndex::Monitor => {
                handle_monitor_page_basic_events(key_event);
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

    pub fn exit() {
        *exit.lock().unwrap() = true;
    }
}
