use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::io::stdout;

pub fn handle_cleanup() {
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn handle_program_exit() -> ! {
    handle_cleanup();
    // TODO: maybe make code be something passed through ?
    std::process::exit(0);
}
