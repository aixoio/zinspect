use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::app::AppState;

pub struct StatusBarWidget<'a> {
    status: &'a AppState,
}

impl<'a> StatusBarWidget<'a> {
    pub fn new(status: &'a AppState) -> Self {
        Self { status }
    }
}

impl<'a> Widget for StatusBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .padding(Padding::horizontal(4))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().blue());

        let text = Line::from(match self.status {
            AppState::InfoPage => {
                vec![
                    Span::styled("Mode: ", Style::default().bold()),
                    Span::styled("Info  ", Style::default().blue()),
                    Span::styled("Files", Style::default()),
                    Span::raw("     "),
                    Span::styled(
                        "tab: swtich modes, ctrl+c: quit",
                        Style::default().gray().italic(),
                    ),
                ]
            }
            AppState::FilesPage => {
                vec![
                    Span::styled("Mode: ", Style::default().bold()),
                    Span::styled("Info  ", Style::default()),
                    Span::styled("Files", Style::default().blue()),
                    Span::raw("     "),
                    Span::styled(
                        "tab: swtich modes, ctrl+c: quit",
                        Style::default().gray().italic(),
                    ),
                ]
            }
        });

        let text = Paragraph::new(text).block(block);

        text.render(area, buf);
    }
}
