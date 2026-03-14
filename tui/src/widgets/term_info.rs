use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use lambdarium::{Print, core};

use super::DEFAULT_WRAP;

#[derive(Debug, Default)]
pub struct TermInfoWidget {
    term: Option<core::Term>,
}

impl TermInfoWidget {
    pub fn set_term(&mut self, term: core::Term) {
        self.term = Some(term);
    }

    pub fn clear(&mut self) {
        self.term = None;
    }

    fn render_inner(&self) -> Option<String> {
        let term = self.term.as_ref()?;

        let term_str = term.print_pretty();
        let underline: String = std::iter::repeat_n("-", term_str.len() - 1).collect();

        let na = String::from("n/a");

        let normal_form = term
            .clone()
            .normalize()
            .map_or(na.clone(), |nf| nf.print_pretty());

        let stlc_type = term.get_type().map_or(na.clone(), |typ| typ.print_pretty());

        let free_vars: Vec<String> = term
            .free_variables()
            .iter()
            .map(|v| v.print_pretty())
            .collect();

        let length = term.length();

        Some(
            format!(
                r#"
Term: {}
------{}
Normal form: {}
STLC type:   {}
Free vars:   {{{}}}
Length:      {}
        "#,
                term_str,
                underline,
                normal_form,
                stlc_type,
                free_vars.join(", "),
                length,
            )
            .trim()
            .to_string(),
        )
    }
}

impl Widget for &TermInfoWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("Term Info");
        let rendered = self.render_inner().unwrap_or(String::new());
        Paragraph::new(rendered)
            .block(block)
            .wrap(DEFAULT_WRAP)
            .render(area, buf);
    }
}
