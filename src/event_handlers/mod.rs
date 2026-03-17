
use crossterm::event::{self, Event as crossEvent, KeyCode, KeyEvent, KeyEventKind};
use std::io;

/// Contains the event handling and logic used by the main page.
pub mod main_handlers;
/// Conts the event handling and logic used by the input page
pub mod input_handlers;

pub use main_handlers::*;
pub use input_handlers::*;

use crate::app::{App};
use crate::ui::{PALETTES, ITEM_HEIGHT};

/// Helper function for handling events for each page.
pub fn handle_events<F: Fn(&mut App,KeyEvent)>(app: &mut App,handler: F) -> io::Result<()> {
        match event::read()? {
            crossEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
               handler(app,key_event)
            }
            _ => {}
        };
        Ok(())
    }