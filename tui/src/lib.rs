use std::io;

use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use lambdarium::Context;

use crate::widgets::DefinitionsWidget;
use crate::widgets::InputWidget;
use crate::widgets::InteractionWidget;
use crate::widgets::TermInfoWidget;

mod event_handling;
mod widgets;

pub fn run_app() -> io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Mode {
    #[default]
    Input,
    Interaction,
    Exit,
}

#[derive(Debug, Default)]
pub struct App {
    context: Context,
    mode: Mode,

    // stateful widgets
    input: InputWidget,
    interaction: InteractionWidget,
    term_info: TermInfoWidget,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.mode != Mode::Exit {
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

        if self.mode == Mode::Input {
            let cursor = self.input.get_cursor_position(input_area);
            frame.set_cursor_position(cursor);
        }
    }
}
