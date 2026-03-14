use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use lambdarium::{Context, meta};

use crate::widgets::DefinitionsWidget;
use crate::widgets::InputWidget;
use crate::widgets::InteractionWidget;
use crate::widgets::TermInfoWidget;

mod widgets;

pub fn run_app() -> io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    context: Context,
    exit: bool,

    // stateful widgets
    input: InputWidget,
    interaction: InteractionWidget,
    term_info: TermInfoWidget,
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

        let [input_area, interaction_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(middle);

        let definitions = DefinitionsWidget::new(&self.context);

        frame.render_widget(outer_block, frame.area());
        frame.render_widget(&self.input, input_area);
        frame.render_widget(&self.interaction, interaction_area);
        frame.render_widget(&self.term_info, left);
        frame.render_widget(&definitions, right);

        let cursor = self.input.get_cursor_position(input_area);
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
                        if let Ok(term) = meta_term.lower(&self.context) {
                            self.term_info.set_term(term.clone());
                            self.interaction.reset(term);
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
