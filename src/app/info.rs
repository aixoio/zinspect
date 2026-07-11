use std::fs::File;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use zip::ZipArchive;

pub struct InfoWidget<'a> {
    archive: &'a ZipArchive<File>,
}

impl<'a> InfoWidget<'a> {
    pub fn new(archive: &'a ZipArchive<File>) -> Self {
        Self { archive }
    }
}

impl<'a> Widget for InfoWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
