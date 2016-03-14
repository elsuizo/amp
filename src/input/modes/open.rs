use commands::{Command, application, open_mode};
use rustbox::keyboard::Key;

pub fn handle(input: Key) -> Option<Command> {
    match input {
        Key::Char('i') => Some(open_mode::enable_insert),
        Key::Char('j') => Some(open_mode::select_next_path),
        Key::Char('k') => Some(open_mode::select_previous_path),
        Key::Enter | Key::Char(' ') => Some(open_mode::open),
        Key::Esc => Some(application::switch_to_normal_mode),
        Key::Ctrl('z') => Some(application::suspend),
        Key::Ctrl('c') => Some(application::exit),
        _ => None,
    }
}
