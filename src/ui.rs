use chrono::prelude::*;
use chrono::TimeZone;
use fs_extra::dir::get_size;
use std::{fs, time::UNIX_EPOCH};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Row, Table, TableState},
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
        let mut values: Vec<String> = Vec::new();
        values.push(format!(
            "{}{}",
            match file.file_name() {
                Some(j) => j.to_string_lossy().to_string(),
                None => String::new(),
            },
            match file.is_dir() {
                true => "/",
                false => "",
            }
        ));
        values.push(match file.clone().is_dir() {
            true => get_size(file.clone()).unwrap().to_string(),
            false => fs::read(file.clone()).unwrap().len().to_string(),
        });
        values.push({
            let (sec, _) = match file
                .clone()
                .metadata()
                .unwrap()
                .created()
                .unwrap()
                .duration_since(UNIX_EPOCH)
            {
                Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
                Err(e) => {
                    // unlikely but should be handled
                    let dur = e.duration();
                    let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
                    if nsec == 0 {
                        (-sec, 0)
                    } else {
                        (-sec - 1, 1_000_000_000 - nsec)
                    }
                }
            };
            Local
                .timestamp_opt(sec, 0)
                .unwrap()
                .format("%d/%m/%Y %H:%M")
                .to_string()
        });
        values.push({
            let (sec, _) = match file
                .clone()
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
            {
                Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
                Err(e) => {
                    // unlikely but should be handled
                    let dur = e.duration();
                    let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
                    if nsec == 0 {
                        (-sec, 0)
                    } else {
                        (-sec - 1, 1_000_000_000 - nsec)
                    }
                }
            };
            Local
                .timestamp_opt(sec, 0)
                .unwrap()
                .format("%d/%m/%Y %H:%M")
                .to_string()
        });
        Row::new(values)
    })
    .collect::<Vec<Row>>();
    let mut list_state = TableState::default().with_selected(Some(app.selected));
    let list = Table::new(
        sub_paths,
        [
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ],
    )
    .block(Block::bordered())
    .on_black()
    .header(Row::new(vec!["Name", "Size (Bytes)", "Created", "Modified"]).on_red())
    .highlight_style(Style::new().bold().white().on_dark_gray());
    frame.render_stateful_widget(list, content, &mut list_state);
    let path = Paragraph::new(app.current_path.clone()).on_black();
    frame.render_widget(path, path_bar);
}
