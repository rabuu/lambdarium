use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use lambdarium::{Print, core};

#[derive(Debug, Default)]
pub struct InteractionWidget {
    terms: Vec<core::Term>,
    active: Option<usize>,
}

impl InteractionWidget {
    pub fn clear(&mut self) {
        self.terms.clear();
        self.active = None;
    }

    pub fn reset(&mut self, term: core::Term) {
        self.terms.clear();
        self.terms.push(term);
        self.active = Some(0);
    }
}

impl Widget for &InteractionWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("Interaction");

        let rendered: String = self
            .terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if self.active == Some(i) {
                    format!("> {}\n", term.print_pretty())
                } else {
                    format!("  {}\n", term.print_pretty())
                }
            })
            .collect();

        Paragraph::new(rendered).block(block).render(area, buf);
    }
}
