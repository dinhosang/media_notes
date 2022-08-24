use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub mod cli;

use cli::input::input_handlers::InputHandlers;

pub fn read_char(handlers: &InputHandlers) -> Result<Event> {
    let event = event::read()?;

    match event {
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('c'),
            ..
        }) => handlers.ctrl_c_handler(),
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => handlers.escape_handler(),
        _ => (),
    }

    Ok(event)
}
