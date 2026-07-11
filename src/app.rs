use std::fs::File;

use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use zip::ZipArchive;

pub struct App {
    state: AppState,
    archive: ZipArchive<File>,
    running: bool,
}

pub enum AppState {
    InfoPage,
    FilesPage,
}

impl AppState {
    pub fn cycle(self) -> Self {
        match self {
            Self::FilesPage => Self::InfoPage,
            Self::InfoPage => Self::FilesPage,
        }
    }
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl App {
    getter!(state, AppState);
    getter!(archive, ZipArchive<File>);

    pub fn new(archive: ZipArchive<File>) -> Self {
        Self {
            state: AppState::InfoPage,
            archive,
            running: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.exit();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.running = false;
    }

    fn draw(&self, frame: &mut Frame) {}
}
