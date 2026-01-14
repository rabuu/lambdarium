use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

use lambda_calculator::meta;

#[derive(Debug, Default)]
pub struct InputArea {
    input: Input,
    item: Option<meta::Item>,
}

impl InputArea {
    pub fn update(&mut self, mut event: &Event) {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('\\'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) = event
        {
            event = &Event::Key(KeyEvent {
                code: KeyCode::Char('λ'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            })
        }
        let _ = self.input.handle_event(event);

        let text = self.input.value();
        self.item = meta::Item::parse(text).ok();
    }

    pub fn get_cursor_position(&self, area: Rect) -> (u16, u16) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        let x = self.input.visual_cursor().max(scroll) - scroll + 1;
        (area.x + x as u16, area.y + 1)
    }
}

impl Widget for &InputArea {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        let text = self.input.value();
        let mut block = Block::bordered().title("Input");

        if !text.is_empty() && self.item.is_none() {
            block = block.red();
        }

        Paragraph::new(text)
            .scroll((0, scroll as u16))
            .block(block)
            .render(area, buf);
    }
}
