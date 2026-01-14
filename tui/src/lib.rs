use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use lambda_calculator::{core, meta};

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
    defs: Vec<meta::Definition>,
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
        let title = Line::from(" λ-calculator ".bold());
        let help = Line::from(vec![" Quit ".into(), "<C-q> ".yellow().bold()]);
        let outer_block = Block::bordered()
            .title(title.centered())
            .title_bottom(help.centered())
            .border_set(border::THICK);

        let area = outer_block.inner(frame.area());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let output = self.term.clone().map_or(vec![], |term| {
            let nf = term.clone().normalize();
            let stlc_typ = term.get_type();
            [
                Some(term.to_string()),
                nf.map(|t| t.to_string()),
                stlc_typ.map(|t| t.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect()
        });

        let output = OutputArea::new(output);

        frame.render_widget(outer_block, frame.area());
        frame.render_widget(&self.input, layout[0]);
        frame.render_widget(&output, layout[1]);

        let cursor = self.input.get_cursor_position(layout[0]);
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
                        self.defs.push(def);
                    }
                    Some(meta::Item::Term(meta_term)) => {
                        if let Ok(core_term) = meta_term.lower(&self.defs) {
                            self.term = Some(core_term);
                        }
                    }
                    None => (),
                },
                _ => {
                    let _ = self.input.update(&event);
                }
            }
        }

        Ok(())
    }
}
