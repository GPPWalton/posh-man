
use crossterm::event::{self, Event as crossEvent, KeyCode, KeyEvent, KeyEventKind};
use std::io;
pub mod main_handlers;
pub mod input_handlers;

pub use main_handlers::*;
pub use input_handlers::*;

use crate::app::{App};
use crate::ui::{PALETTES, ITEM_HEIGHT};
pub fn handle_events<F: Fn(&mut App,KeyEvent)>(app: &mut App,handler: F) -> io::Result<()> {
        match event::read()? {
            crossEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
               handler(app,key_event)
            }
            _ => {}
        };
        Ok(())
    }