use std::fs;

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use human_repr::HumanCount;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Clear, Padding, Paragraph, Row, StatefulWidget, Table, TableState,
        Widget,
    },
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
    inspect: bool,
}

impl FilesWidgetState {
    pub fn new() -> FilesWidgetState {
        let mut table = TableState::default();
        table.select_first();

        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Search...");
        textarea.set_placeholder_style(Style::default().gray());

        FilesWidgetState {
            table,
            textarea,
            inspect: false,
        }
    }

    pub fn next(&mut self) {
        self.table.select_next();
    }

    pub fn back(&mut self) {
        self.table.select_previous();
    }

    pub fn inspect(&mut self) {
        self.inspect = !self.inspect;
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

        let query = state.textarea.lines().join(" ");
        let matcher = SkimMatcherV2::default();

        let mut files: Vec<_> = self
            .files
            .iter()
            .filter_map(|file| {
                if query.is_empty() {
                    Some((file, 0))
                } else {
                    matcher
                        .fuzzy_match(&file.filename, &query)
                        .map(|score| (file, score))
                }
            })
            .collect();

        files.sort_by_key(|(_, score)| std::cmp::Reverse(*score));

        if files.is_empty() {
            state.table.select(None);
        } else if state
            .table
            .selected()
            .is_none_or(|index| index >= files.len())
        {
            state.table.select_first();
        }

        let rows = files.iter().map(|(file, _)| {
            let percent = if file.size == 0 {
                0.0
            } else {
                100.0 - (file.compressed_size as f64 / file.size as f64 * 100.0)
            };

            Row::new([
                file.filename.to_string(),
                file.size.human_count_bytes().to_string(),
                format!("{percent:.1}%"),
            ])
        });

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

        if let Some(item) = state.table.selected()
            && state.inspect
        {
            let item = files[item].0;

            let percent = if item.size == 0 {
                0.0
            } else {
                100.0 - (item.compressed_size as f64 / item.size as f64 * 100.0)
            };

            let popup_block = Block::bordered()
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().cyan());

            let centered_area =
                area.centered(Constraint::Percentage(70), Constraint::Percentage(30));

            Widget::render(Clear, centered_area, buf);

            let text = Paragraph::new(Text::from(vec![
                Line::from(vec![
                    Span::styled("Filename ", Style::default().bold().cyan()),
                    Span::raw(format!("{}", item.filename)),
                ]),
                Line::from(vec![
                    Span::styled("Size ", Style::default().bold().cyan()),
                    Span::raw(format!("{}", item.size.human_count_bytes())),
                ]),
                Line::from(vec![
                    Span::styled("Compressed Size ", Style::default().bold().cyan()),
                    Span::raw(format!("{}", item.compressed_size.human_count_bytes())),
                ]),
                Line::from(vec![
                    Span::styled("Percent ", Style::default().bold().cyan()),
                    Span::raw(format!("{:.2}%", percent)),
                ]),
            ]))
            .block(popup_block);

            Widget::render(text, centered_area, buf);
        }
    }
}
