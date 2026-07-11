use std::fs::File;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};
use zip::ZipArchive;

use crate::{
    app::{info::InfoWidget, status::StatusBarWidget},
    getter,
};

mod info;
mod status;

pub struct App {
    state: AppState,
    archive: ZipArchive<File>,
    running: bool,
    filename: Box<str>,
}

pub enum AppState {
    InfoPage,
    FilesPage,
}

impl AppState {
    pub fn cycle(&self) -> Self {
        match self {
            Self::FilesPage => Self::InfoPage,
            Self::InfoPage => Self::FilesPage,
        }
    }
}

impl App {
    getter!(state, AppState);
    getter!(archive, ZipArchive<File>);

    pub fn new(archive: ZipArchive<File>, filename: Box<str>) -> Self {
        Self {
            state: AppState::InfoPage,
            archive,
            running: true,
            filename,
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
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.exit();
                }

                if key_event.code == KeyCode::Tab {
                    self.state = self.state.cycle();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.running = false;
    }

    fn draw(&self, frame: &mut Frame) {
        let master = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(6)])
            .split(frame.area());

        let statusbar = StatusBarWidget::new(&self.state);
        frame.render_widget(statusbar, master[0]);

        if let AppState::InfoPage = &self.state {
            let info = InfoWidget::new(&self.archive, &self.filename);
            frame.render_widget(info, master[1]);
        }
    }
}
