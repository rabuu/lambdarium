mod definitions;
mod input;
mod interaction;
mod term_info;

pub use definitions::DefinitionsWidget;
pub use input::InputWidget;
pub use interaction::InteractionWidget;
pub use term_info::TermInfoWidget;

use ratatui::widgets::Wrap;
const DEFAULT_WRAP: Wrap = Wrap { trim: true };
