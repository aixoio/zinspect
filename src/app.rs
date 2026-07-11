use std::fs::File;

use zip::ZipArchive;

pub struct App {
    state: AppState,
    archive: ZipArchive<File>,
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
        }
    }
}
