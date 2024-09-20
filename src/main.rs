use std::{
    fs,
    io::{self, Write},
};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    theme::Theme,
    tui::Tui,
};
use ratatui::style::Color;
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

pub mod app;
pub mod event;
pub mod handler;
pub mod theme;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    let theme_string = fs::read_to_string("./theme.json");
    let theme: Theme = serde_json::from_str(&theme_string.unwrap().as_str()).unwrap();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app, theme.clone())?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    let _ = std::io::stdout().write_all(format!("{}\n", app.current_path).as_bytes());

    Ok(())
}
