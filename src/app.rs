use std::{error, fs};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// selected
    pub selected: usize,
    /// current_path
    pub current_path: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            selected: 0,
            current_path: "/workspaces/tui-file-explorer".to_string(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_selected(&mut self) {
        let len = fs::read_dir(&self.current_path)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
            .len();

        if len != 0 {
            if let Some(res) = self.selected.checked_add(1) {
                if res != len {
                    self.selected = res;
                }
            }
        }
    }

    pub fn decrement_selected(&mut self) {
        if let Some(res) = self.selected.checked_sub(1) {
            self.selected = res;
        }
    }
}
