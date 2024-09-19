use std::fs;

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListState, Paragraph, Row},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let [path_bar, content] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());
    let sub_paths = match fs::read_dir(&app.current_path) {
        Ok(dir) => dir,
        Err(err) => panic!("How did you get here?: {err}"),
    }
    .map(|x| x.unwrap().path())
    .map(|file| {
        format!(
            "{}{}",
            match file.file_name() {
                Some(j) => j.to_string_lossy().to_string(),
                None => String::new(),
            },
            match file.is_dir() {
                true => "/",
                false => "",
            }
        )
    })
    .collect::<Vec<String>>();
    let mut list_state = ListState::default().with_selected(Some(app.selected));
    let list = List::new(sub_paths).block(Block::bordered()).scroll_padding(5).highlight_style(Style::new().on_red());
    frame.render_stateful_widget(list, content, &mut list_state);
}