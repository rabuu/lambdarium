use std::{borrow::Cow, io};

use reedline::{Prompt, Reedline, Signal};

use lambda_calculator::LAMBDA;

struct MainPrompt;
impl Prompt for MainPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<'_, str> {
        Cow::Owned(format!("{LAMBDA}-calculator"))
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<'_, str> {
        Cow::Owned(String::new())
    }

    fn render_prompt_indicator(&self, _: reedline::PromptEditMode) -> std::borrow::Cow<'_, str> {
        Cow::Owned(String::from("> "))
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<'_, str> {
        Cow::Owned(String::from("> "))
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<'_, str> {
        let prefix = match history_search.status {
            reedline::PromptHistorySearchStatus::Passing => "",
            reedline::PromptHistorySearchStatus::Failing => "failing ",
        };
        Cow::Owned(format!("({prefix}search: {})", history_search.term))
    }
}

fn main() -> io::Result<()> {
    let mut line_editor = Reedline::create();

    let mut first = Some(());

    loop {
        if first.take().is_none() {
            println!();
        }

        let line = match line_editor.read_line(&MainPrompt)? {
            Signal::Success(line) => line,
            Signal::CtrlC | Signal::CtrlD => return Ok(()),
        };

        let Some(term) = lambda_calculator::Term::parse(line) else {
            eprintln!("Invalid {LAMBDA}-term.");
            continue;
        };

        let Some(typ) = term.get_type() else {
            eprintln!("Not typable in STLC.");
            continue;
        };

        println!("{typ}");
    }
}
