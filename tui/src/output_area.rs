use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

#[derive(Debug)]
pub struct OutputArea {
    pub title: String,
    pub lines: Vec<String>,
}

impl OutputArea {
    pub fn new(title: String, lines: Vec<String>) -> Self {
        Self { title, lines }
    }
}

impl Widget for &OutputArea {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title(self.title.as_str());

        Paragraph::new(self.lines.join("\n"))
            .block(block)
            .render(area, buf);
    }
}
