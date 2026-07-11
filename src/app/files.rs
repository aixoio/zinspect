use std::fs;

use human_repr::HumanCount;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Padding, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};
use ratatui_textarea::{Input, Key, TextArea};
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
    textarea: TextArea<'static>,
}

impl FilesWidgetState {
    pub fn new() -> FilesWidgetState {
        let mut table = TableState::default();
        table.select_first();

        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Search...");
        textarea.set_placeholder_style(Style::default().gray());

        FilesWidgetState { table, textarea }
    }

    pub fn next(&mut self) {
        self.table.select_next();
    }

    pub fn back(&mut self) {
        self.table.select_previous();
    }

    pub fn input(&mut self, input: impl Into<Input>) {
        let input = input.into();

        if !matches!(input.key, Key::Enter) {
            self.textarea.input(input);
        }
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
        let [text_layout, table_layout] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .areas(area);

        let [header_area, separator_area, rows_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(table_layout);

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

        let header_table = Table::new(Vec::<Row>::new(), widths)
            .header(Row::new(["Filename", "Size", "%"]).style(Style::default().bold()))
            .column_spacing(1);

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::default().on_blue());

        let search_block = Block::default().padding(Padding::new(2, 2, 1, 1));

        let search_inner = search_block.inner(text_layout);
        search_block.render(text_layout, buf);

        let [prefix_area, textarea_area] =
            Layout::horizontal([Constraint::Length(2), Constraint::Min(1)]).areas(search_inner);

        Paragraph::new("> ").render(prefix_area, buf);
        Widget::render(&state.textarea, textarea_area, buf);

        Widget::render(header_table, header_area, buf);

        Paragraph::new(Span::styled(
            "─".repeat(separator_area.width as usize),
            Style::default().blue(),
        ))
        .render(separator_area, buf);

        StatefulWidget::render(table, rows_area, buf, &mut state.table);
    }
}
