/// Application.
pub mod app;

/// Terminal user interface
pub mod tui;

/// UI.
pub mod ui;

/// Events handler
pub mod event;

/// Application updater
pub mod update;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    // Create application
    let mut app = App::new();

    // Initialize terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop
    while !app.should_quit {
        // Render the uesr interface
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
