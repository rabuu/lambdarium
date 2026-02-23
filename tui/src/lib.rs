use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use lambdarium::{Context, Print, core, meta};

use crate::input_area::InputArea;
use crate::output_area::OutputArea;

mod input_area;
mod output_area;

pub fn run_app() -> io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    input: InputArea,
    exit: bool,
    context: Context,
    term: Option<core::Term>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from(" λambdarium ".bold());
        let help = Line::from(vec![" Quit ".into(), "<C-q> ".yellow().bold()]);

        let outer_block = Block::bordered()
            .title(title.centered())
            .title_bottom(help.centered())
            .border_set(border::THICK);

        let area = outer_block.inner(frame.area());

        let [left, middle, right] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [input, interaction] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(middle);

        let term_info = self.term.clone().map_or(vec![], |term| {
            let nf = term.clone().normalize();
            let stlc_typ = term.get_type();
            vec![
                term.print_pretty(),
                String::new(),
                nf.map(|nf| format!("Normal form: {}", nf.print_pretty()))
                    .unwrap_or(String::from("n/a")),
                stlc_typ
                    .map(|ty| format!("STLC type:   {}", ty.print_pretty()))
                    .unwrap_or(String::from("n/a")),
            ]
        });
        let term_info = OutputArea::new("Term Info".to_owned(), term_info);

        let definitions = self
            .context
            .iter()
            .map(|(metavar, body)| {
                format!("def {} = {}", metavar.print_pretty(), body.print_pretty())
            })
            .collect();
        let definitions = OutputArea::new("Definitions".to_owned(), definitions);

        let interaction_content = OutputArea::new("Interaction Area".to_owned(), vec![]);

        frame.render_widget(outer_block, frame.area());
        frame.render_widget(&self.input, input);
        frame.render_widget(&term_info, left);
        frame.render_widget(&interaction_content, interaction);
        frame.render_widget(&definitions, right);

        let cursor = self.input.get_cursor_position(input);
        frame.set_cursor_position(cursor);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = crossterm::event::read()?;

        if let Event::Key(key) = event {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => self.exit = true,
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => match self.input.item.clone() {
                    Some(meta::Item::Def(def)) => {
                        let _ = self.context.add_definition(def);
                    }
                    Some(meta::Item::Term(meta_term)) => {
                        if let Ok(core_term) = meta_term.lower(&self.context) {
                            self.term = Some(core_term);
                        }
                    }
                    None => (),
                },
                _ => {
                    self.input.update(&event);
                }
            }
        }

        Ok(())
    }
}
