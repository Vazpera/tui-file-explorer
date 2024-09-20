use ratatui::style::Color;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Theme {
    pub border: Color,
    pub background: Color,
    pub text: Color,
    pub header_background: Color,
    pub header_text: Color,
    pub highlight_background: Color,
    pub highlight_text: Color,
    pub path_background: Color,
    pub path_text: Color,
}
