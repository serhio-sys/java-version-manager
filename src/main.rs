use crossterm::{event::{read, Event, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode}};

mod config;
mod models;
mod utils;

fn main() {    
    config::initialize_versions();
    program();
}

fn program() {
    enable_raw_mode().unwrap();
    let mut selected: i32 = 0;
    loop {
        let selected_item = utils::menu_utils::print_main_menu(selected);
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                if selected - 1 >= 0 {
                    selected -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                {
                    if selected + 1 < models::menu_command::MENU_COMMANDS.lock().unwrap().len() as i32 {
                        selected += 1;
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                if let Some(unwrapped) = selected_item {
                    unwrapped.call_func();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                disable_raw_mode().unwrap();
                break;
            }
            _ => {}
        }
    }
}


