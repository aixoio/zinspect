use std::fs::File;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};
use zip::ZipArchive;

use crate::{
    app::{
        files::{FilesWidget, FilesWidgetState},
        info::InfoWidget,
        status::StatusBarWidget,
    },
    getter,
};

mod files;
mod info;
mod status;

pub struct App {
    state: AppState,
    archive: ZipArchive<File>,
    running: bool,
    filename: Box<str>,
    files_widget: FilesWidget,
    files_widget_state: FilesWidgetState,
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

    pub fn new(mut archive: ZipArchive<File>, filename: Box<str>) -> anyhow::Result<Self> {
        let files_widget = FilesWidget::build(&mut archive)?;

        Ok(Self {
            state: AppState::InfoPage,
            archive,
            running: true,
            filename,
            files_widget,
            files_widget_state: FilesWidgetState::new(),
        })
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
                let mut cycled = false;

                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit()
                    }
                    KeyCode::Tab => {
                        cycled = true;
                        self.state = self.state.cycle();
                    }

                    KeyCode::Up if matches!(self.state(), AppState::FilesPage) => {
                        self.files_widget_state.back()
                    }
                    KeyCode::Down if matches!(self.state(), AppState::FilesPage) => {
                        self.files_widget_state.next()
                    }

                    _ => {}
                }

                if let AppState::FilesPage = &self.state
                    && !cycled
                {
                    self.files_widget_state.input(key_event);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.running = false;
    }

    fn draw(&mut self, frame: &mut Frame) {
        let master = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(6)])
            .split(frame.area());

        let statusbar = StatusBarWidget::new(&self.state);
        frame.render_widget(statusbar, master[0]);

        if let AppState::InfoPage = &self.state {}

        match &self.state {
            AppState::InfoPage => {
                let info = InfoWidget::new(&self.archive, &self.filename);
                frame.render_widget(info, master[1]);
            }
            AppState::FilesPage => frame.render_stateful_widget(
                &self.files_widget,
                master[1],
                &mut self.files_widget_state,
            ),
        }
    }
}
