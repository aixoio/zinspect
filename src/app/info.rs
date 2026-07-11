use std::fs::File;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Widget},
};
use zip::ZipArchive;

pub struct InfoWidget<'a> {
    archive: &'a ZipArchive<File>,
    filename: &'a str,
}

impl<'a> InfoWidget<'a> {
    pub fn new(archive: &'a ZipArchive<File>, filename: &'a str) -> Self {
        Self { archive, filename }
    }
}

impl<'a> Widget for InfoWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().padding(Padding::new(4, 0, 1, 0));

        let text = Line::from(vec![
            Span::styled("Filename    ", Style::default().blue().bold()),
            Span::raw(self.filename),
        ]);

        let text = Paragraph::new(text).block(block);

        text.render(area, buf);
    }
}
