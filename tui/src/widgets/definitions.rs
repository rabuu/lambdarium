use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use lambdarium::{Context, Print};

#[derive(Debug)]
pub struct DefinitionsWidget<'a> {
    context: &'a Context,
}

impl<'a> DefinitionsWidget<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl Widget for &DefinitionsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("Definitions");

        let rendered: String = self
            .context
            .iter()
            .map(|(var, body)| format!("def {} = {}\n", var.print_pretty(), body.print_pretty()))
            .collect();

        Paragraph::new(rendered).block(block).render(area, buf);
    }
}
