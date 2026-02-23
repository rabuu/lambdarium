use std::{borrow::Cow, fmt};

use inquire::Confirm;
use inquire_derive::Selectable;
use nu_ansi_term::{Color, Style};
use reedline::{
    EditCommand, Emacs, Highlighter, KeyCode, KeyModifiers, Prompt, Reedline, ReedlineEvent,
    Signal, StyledText, ValidationResult, Validator, default_emacs_keybindings,
};

use lambdarium::{Context, EvaluationStrategy, Print, meta};

use crate::error::AppError;

const PROMPT: &str = "λambdarium";

const COLOR_LAMBDA: Color = Color::Purple;
const COLOR_PUNCTUATION: Color = Color::DarkGray;

pub fn run() -> Result<(), AppError> {
    let edit_mode = {
        let mut keybindings = default_emacs_keybindings();
        keybindings.add_binding(
            KeyModifiers::empty(),
            KeyCode::Char('\\'),
            ReedlineEvent::Edit(vec![EditCommand::InsertChar('λ')]),
        );

        Emacs::new(keybindings)
    };

    let mut line_editor = Reedline::create()
        .with_validator(Box::new(MatchingParensValidator))
        .with_edit_mode(Box::new(edit_mode))
        .with_highlighter(Box::new(TermHighlighter));

    let mut context = Context::default();

    let mut first = Some(());
    loop {
        if first.take().is_none() {
            println!();
        }

        let line = match line_editor.read_line(&MainPrompt)? {
            Signal::Success(line) => line,
            Signal::CtrlC | Signal::CtrlD => return Ok(()),
        };

        let item = match meta::Item::parse(&line) {
            Ok(item) => item,
            Err(err) => {
                render_report_with_source(err, line);
                continue;
            }
        };

        let term = match item {
            meta::Item::Def(def) => {
                if let Err(err) = context.add_definition(def) {
                    render_report_with_source(err, line);
                }
                continue;
            }
            meta::Item::Term(term) => term,
        };

        let term = match term.lower(&context) {
            Ok(term) => term,
            Err(err) => {
                render_report_with_source(err, line);
                continue;
            }
        };

        println!("Term: {}", term.print_pretty());

        let action = match TermAction::select("Action:").prompt() {
            Ok(action) => action,
            Err(_) => continue,
        };

        match action {
            TermAction::TypeInference => {
                let Some(typ) = term.get_type() else {
                    eprintln!("Not typable in STLC.");
                    continue;
                };

                println!("{}", typ.print_pretty());
            }
            TermAction::BetaContraction => {
                let mut term = term;

                let mut first = Some(());
                while let Some(contracted) = term.beta_contract(EvaluationStrategy::NormalOrder) {
                    if first.take().is_none() {
                        let confirm_again = Confirm::new("Continue?").with_default(true).prompt();
                        match confirm_again {
                            Ok(true) => (),
                            Ok(false) | Err(_) => break,
                        }
                    }

                    term = contracted;
                    println!("{}", term.print_pretty());
                }

                println!("The term is in β normal form.");
            }
        }
    }
}

struct MainPrompt;
impl Prompt for MainPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<'_, str> {
        Cow::Owned(String::from(PROMPT))
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
        Cow::Owned(format!(" ({prefix}search: {})> ", history_search.term))
    }
}

struct TermHighlighter;
impl Highlighter for TermHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> reedline::StyledText {
        let mut styled_text = StyledText::new();

        for c in line.chars() {
            let style = match c {
                'λ' | '\\' => Style::new().bold().fg(COLOR_LAMBDA),
                '.' | '(' | ')' => Style::new().fg(COLOR_PUNCTUATION),
                _ => Style::new(),
            };
            styled_text.push((style, c.to_string()));
        }

        styled_text
    }
}

struct MatchingParensValidator;
impl Validator for MatchingParensValidator {
    fn validate(&self, line: &str) -> reedline::ValidationResult {
        let mut counter = 0;

        for c in line.chars() {
            match c {
                '(' => counter += 1,
                ')' => counter -= 1,
                _ => (),
            }

            if counter < 0 {
                return ValidationResult::Incomplete;
            }
        }

        if counter == 0 {
            ValidationResult::Complete
        } else {
            ValidationResult::Incomplete
        }
    }
}

#[derive(Debug, Clone, Copy, Selectable)]
enum TermAction {
    BetaContraction,
    TypeInference,
}

impl fmt::Display for TermAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermAction::BetaContraction => write!(f, "β-contract"),
            TermAction::TypeInference => write!(f, "Infer type (in STLC)"),
        }
    }
}

fn render_report_with_source<D: Into<miette::Report>>(err: D, source: String) {
    let report: miette::Report = err.into();
    let report = report.with_source_code(source);
    eprintln!("{report:?}")
}
