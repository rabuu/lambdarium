use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lambdarium::meta;

use crate::{App, Mode};

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        let event = crossterm::event::read()?;

        if let Event::Key(key) = event {
            self.handle_key_event(key, event);
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, event: Event) {
        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.mode = Mode::Exit;
                // panic!();
            }
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

                        self.mode = Mode::Interaction;
                    }
                }
                None => (),
            },
            _ => {
                self.input.update(&event);
            }
        }
    }
}
