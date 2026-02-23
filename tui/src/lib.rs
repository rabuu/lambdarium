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

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(5), Constraint::Fill(1)])
            .split(area);

        let main_section = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(columns[0]);

        let input_section = main_section[0];
        let output_section = main_section[1];

        let output = self.term.clone().map_or(vec![], |term| {
            let nf = term.clone().normalize();
            let stlc_typ = term.get_type();
            vec![
                term.print_pretty(),
                nf.map(|nf| format!("{} (normal form)", nf.print_pretty()))
                    .unwrap_or(String::from("No normal form")),
                stlc_typ
                    .map(|ty| format!("{} (STLC type)", ty.print_pretty()))
                    .unwrap_or(String::from("No type in STLC")),
            ]
        });

        let output = OutputArea::new("Output".to_owned(), output);

        let definitions = OutputArea::new(
            "Definitions".to_owned(),
            self.context
                .iter()
                .map(|(metavar, body)| {
                    format!("def {} = {}", metavar.print_pretty(), body.print_pretty())
                })
                .collect(),
        );

        frame.render_widget(outer_block, frame.area());
        frame.render_widget(&self.input, input_section);
        frame.render_widget(&output, output_section);
        frame.render_widget(&definitions, columns[1]);

        let cursor = self.input.get_cursor_position(input_section);
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
