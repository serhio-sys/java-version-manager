use crossterm::{
    event::{ self, Event, KeyCode, KeyEvent, KeyEventKind },
    terminal::{ disable_raw_mode, enable_raw_mode },
};

mod program;

fn main() {
    program::config::initialize_versions();
    program();
}

fn program() {
    enable_raw_mode().unwrap();
    let mut selected: i32 = 0;
    loop {
        let selected_item = program::utils::menu_utils::print_main_menu(selected);
        if let Event::Key(event) = event::read().unwrap() {
            if let KeyEventKind::Release = event.kind {
                continue;
            }
            match event {
                KeyEvent { code: KeyCode::Up, modifiers: _, kind: _, state: _ } => {
                    if selected - 1 >= 0 {
                        selected -= 1;
                    }
                }
                KeyEvent { code: KeyCode::Down, modifiers: _, kind: _, state: _ } => {
                    {
                        if
                            selected + 1 <
                            (
                                program::models::menu_command::MENU_COMMANDS
                                    .lock()
                                    .unwrap()
                                    .len() as i32
                            )
                        {
                            selected += 1;
                        }
                    }
                }
                KeyEvent { code: KeyCode::Enter, modifiers: _, kind: _, state: _ } => {
                    if let Some(unwrapped) = selected_item {
                        unwrapped.call_func();
                    }
                }
                KeyEvent { code: KeyCode::Esc, modifiers: _, kind: _, state: _ } => {
                    disable_raw_mode().unwrap();
                    break;
                }
                _ => {}
            }
        }
    }
}
