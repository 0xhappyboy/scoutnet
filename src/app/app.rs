use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
    Frame,
};

use crate::net::device::Device;

use super::ui::{self, config::MonitorPageArea};
use super::{config::PageIndex, init};

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
    pub monitor_page_name_list: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            input_text: String::from(""),
            input_mode: InputMode::Normal,
            page_index: PageIndex::Monitor,
            exit: false,
            monitor_page_selecting_area: MonitorPageArea::Area_1,
            monitor_page_selected_area: MonitorPageArea::None,
            monitor_page_name_list: vec![],
        }
    }

    pub fn run(&mut self, terminal: &mut init::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        match self.page_index {
            PageIndex::Welcome => {
                ui::welcome::layout(self, frame);
            }
            PageIndex::Monitor => {
                self.handle_monitor_page_data();
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
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.handle_global_basic_events(key_event);
        match self.page_index {
            PageIndex::Welcome => {}
            PageIndex::Monitor => {
                self.handle_monitor_page_basic_events(key_event);
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

    fn handle_global_basic_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.exit();
            }
            KeyCode::Char('1') => {
                self.page_index = PageIndex::Welcome;
            }
            KeyCode::Char('2') => {
                self.page_index = PageIndex::Monitor;
            }
            KeyCode::Char('3') => {
                self.page_index = PageIndex::Safe;
            }
            KeyCode::Char('4') => {
                self.page_index = PageIndex::Http;
            }
            _ => {}
        }
    }

    fn handle_monitor_page_data(&mut self) {
        if (self.monitor_page_name_list.is_empty()) {
            self.monitor_page_name_list = Device::get_device_name_list();
        }
    }

    fn handle_monitor_page_basic_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                if (self.monitor_page_selecting_area == MonitorPageArea::Area_3) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_1;
                } else if (self.monitor_page_selecting_area == MonitorPageArea::Area_4) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_2;
                }
            }
            KeyCode::Down => {
                if (self.monitor_page_selecting_area == MonitorPageArea::Area_1) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_3;
                } else if (self.monitor_page_selecting_area == MonitorPageArea::Area_2) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_4;
                }
            }
            KeyCode::Left => {
                if (self.monitor_page_selecting_area == MonitorPageArea::Area_2) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_1;
                } else if (self.monitor_page_selecting_area == MonitorPageArea::Area_4) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_3;
                }
            }
            KeyCode::Right => {
                if (self.monitor_page_selecting_area == MonitorPageArea::Area_1) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_2;
                } else if (self.monitor_page_selecting_area == MonitorPageArea::Area_3) {
                    self.monitor_page_selecting_area = MonitorPageArea::Area_4;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
