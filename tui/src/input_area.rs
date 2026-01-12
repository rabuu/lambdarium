use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct InputArea<'a> {
    text: Paragraph<'a>,
}

impl Default for InputArea<'_> {
    fn default() -> Self {
        let text =
            Paragraph::default().block(Block::default().borders(Borders::ALL).title("Input"));
        Self { text }
    }
}

impl Widget for InputArea<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.text.render(area, buf);
    }
}
