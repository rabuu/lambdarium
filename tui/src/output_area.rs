use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

#[derive(Debug)]
pub struct OutputArea {
    pub lines: Vec<String>,
}

impl OutputArea {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

impl Widget for &OutputArea {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("Output");

        Paragraph::new(self.lines.join("\n"))
            .block(block)
            .render(area, buf);
    }
}
