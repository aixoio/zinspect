use std::fs::File;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
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

        let mut lines = vec![Line::from(vec![
            Span::styled("Filename    ", Style::default().blue().bold()),
            Span::raw(self.filename),
        ])];

        if let Some(size) = self.archive.decompressed_size() {
            lines.push(Line::from(vec![
                Span::styled("Decompressed size    ", Style::default().blue().bold()),
                Span::raw(size.to_string()),
            ]));
        }

        let text = Text::from(lines);

        let text = Paragraph::new(text).block(block);

        text.render(area, buf);
    }
}
