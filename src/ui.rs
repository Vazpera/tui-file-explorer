use chrono::prelude::*;
use chrono::TimeZone;
use std::time::SystemTime;
use std::{fs, time::UNIX_EPOCH};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::App;
use crate::theme;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, theme: theme::Theme) {
    let [path_bar, content] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());
    let sub_paths = match fs::read_dir(&app.current_path) {
        Ok(dir) => dir,
        Err(err) => panic!("How did you get here?: {err}, {}", app.current_path),
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
        values.push({
            let file_metadata = file.clone().metadata();
            match file_metadata {
                Ok(metadata) => {
                    let (sec, _) = match metadata
                        .created()
                        .unwrap_or(SystemTime::now())
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
                }
                Err(_) => "N/A".to_string(),
            }
        });
        values.push({
            let file_metadata = file.clone().metadata();
            match file_metadata {
                Ok(metadata) => {
                    let (sec, _) = match metadata
                        .modified()
                        .unwrap_or(SystemTime::now())
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
                }
                Err(_) => "N/A".to_string(),
            }
        });
        values.push(format!(
            "{}",
            match file.is_dir() {
                true => "File Folder",
                false => match file.extension() {
                    Some(extension) => {
                        match extension.to_str().unwrap() {
                            "txt" => "Text File",
                            "mp3" | "wav" | "flak" => "Audio File",
                            "mp4" | "mov" => "Video File",
                            "rs" | "ts" | "js" | "cpp" | "c" | "cs" | "go" => "Source Code",
                            "exe" => "Windows Executable",
                            "deb" => "Debian Package",
                            _ => "",
                        }
                    }
                    _ => "",
                },
            }
        ));
        Row::new(values).fg(theme.text).bg(theme.background)
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
    .block(Block::bordered().border_style(Style::new().fg(theme.border)))
    .bg(theme.background)
    .header(
        Row::new(vec!["Name", "Created", "Modified", "Type"])
            .fg(theme.header_text)
            .bg(theme.header_background),
    )
    .highlight_style(
        Style::new()
            .bold()
            .fg(theme.highlight_text)
            .bg(theme.highlight_background),
    );
    frame.render_stateful_widget(list, content, &mut list_state);
    let path = Paragraph::new(app.current_path.clone())
        .fg(theme.path_text)
        .bg(theme.path_background);
    frame.render_widget(path, path_bar);
}
