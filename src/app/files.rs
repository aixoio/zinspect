use std::fs;

use human_repr::HumanCount;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Row, StatefulWidget, Table, TableState},
};
use zip::ZipArchive;

struct File {
    filename: Box<str>,
    size: u64,
    compressed_size: u64,
}

pub struct FilesWidget {
    files: Box<[File]>,
}

pub struct FilesWidgetState {
    table: TableState,
}

impl FilesWidgetState {
    pub fn new() -> FilesWidgetState {
        let mut table = TableState::default();
        table.select_first();

        FilesWidgetState { table }
    }

    pub fn next(&mut self) {
        self.table.select_next();
    }

    pub fn back(&mut self) {
        self.table.select_previous();
    }
}

impl FilesWidget {
    pub fn build(archive: &mut ZipArchive<fs::File>) -> anyhow::Result<FilesWidget> {
        let mut files = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            files.push(File {
                filename: file.name().into(),
                compressed_size: file.compressed_size(),
                size: file.size(),
            });
        }

        Ok(Self {
            files: files.into_boxed_slice(),
        })
    }
}

impl StatefulWidget for &FilesWidget {
    type State = FilesWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header = Row::new(["Filename", "Size", "%"])
            .style(Style::default().bold())
            .bottom_margin(1);

        let rows: Vec<_> = self
            .files
            .iter()
            .map(|f| {
                Row::new([
                    f.filename.to_string(),
                    f.size.human_count_bytes().to_string(),
                    format!("{}", ((f.size - f.compressed_size) / f.size) * 100),
                ])
            })
            .collect();

        let widths = [
            Constraint::Percentage(70),
            Constraint::Percentage(15),
            Constraint::Percentage(5),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::default().on_blue());

        table.render(area, buf, &mut state.table);
    }
}
