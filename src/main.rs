use crossterm::{
    cursor::MoveToColumn,
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    Result,
};
use std::io::{stdout, Write};

use media_notes::{
    cli::{helpers, input::input_handlers::InputHandlersBuilder},
    read_char,
};

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;

    enable_raw_mode()?;

    let handlers = InputHandlersBuilder::new().build();

    let mut text = String::new();

    loop {
        let result = read_char(&handlers);

        let event = result.unwrap();

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                let _ = &text.pop();
                execute!(stdout(), Clear(ClearType::All), MoveToColumn(0))?;
                print_string(&text);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                let _ = &text.push_str(&c.to_string());
                print_string(&c.to_string());
            }
            _ => (),
        }
    }

    helpers::handle_cleanup();
    Result::Ok(())
}

fn print_string(s: &str) {
    print!("{}", s);
    stdout().flush().unwrap();
}
