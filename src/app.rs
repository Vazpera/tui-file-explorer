use std::{
    error, fs,
    path::{Path, PathBuf},
};

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
            current_path: PathBuf::from("./")
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string(),
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
    pub fn zoom(&mut self) {
        let files = fs::read_dir(&self.current_path)
            .unwrap()
            .map(|x| x.unwrap().path())
            .collect::<Vec<_>>();
        match files[self.selected].is_dir() {
            true => {
                self.current_path = files[self.selected].as_path().to_string_lossy().to_string()
            }
            false => {
                self.current_path = files[self.selected].as_path().to_string_lossy().to_string();
                self.quit();
            }
        }
        self.selected = 0;
    }
    pub fn unzoom(&mut self) {
        self.current_path = match Path::new(&self.current_path).parent() {
            Some(j) => j.to_string_lossy().to_string(),
            None => self.current_path.clone(),
        }
    }
}
