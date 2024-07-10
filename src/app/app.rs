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

use super::init;
use super::ui;

#[derive(Debug)]
enum PageIndex {
    Welcome,
    Home,
    Safe,
    Test,
}

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    pub input_text: String,
    pub input_mode: InputMode,
    pub page_index: PageIndex,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            input_text: String::from(""),
            input_mode: InputMode::Normal,
            page_index: PageIndex::Welcome,
        }
    }

    pub fn run(&mut self, terminal: &mut init::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        match self.page_index {
            PageIndex::Welcome => {
                ui::welcome::layout(self, frame);
            }
            PageIndex::Home => {
                ui::home::layout(self, frame);
            }
            PageIndex::Safe => {
                ui::safe::layout(self, frame);
            }
            PageIndex::Test => {
                ui::test::layout(self, frame);
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('1') => self.page_index = PageIndex::Welcome,
                KeyCode::Char('2') => self.page_index = PageIndex::Home,
                KeyCode::Char('3') => self.page_index = PageIndex::Safe,
                KeyCode::Char('4') => self.page_index = PageIndex::Test,
                KeyCode::Esc => self.exit(),
                _ => {}
            },
            InputMode::Editing => match key_event.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_text.clear();
                }
                KeyCode::Char(insert) => self.input_text.push(insert),
                KeyCode::Backspace => {
                    if self.input_text.len() > 0 {
                        let removed = &self.input_text[0..self.input_text.len() - 1];
                        self.input_text = removed.to_owned();
                    }
                }
                _ => {}
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
