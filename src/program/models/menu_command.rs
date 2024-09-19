use std::{ io::stdout, sync::Mutex };

use crossterm::{ cursor, execute, terminal::{ Clear, ClearType } };
use lazy_static::lazy_static;

use crate::program::variation::BaseCommands;
#[allow(unused_imports)]
use crate::program::{utils::print_utils::press_to_continue, variation::{linux_variation::LinuxVariation, win_variation::WinVariation, Commands}};


lazy_static! {
    static ref COMMANDS: Commands = {
        #[cfg(windows)]
        {
            return Commands::WinCommands(WinVariation {});
        }
        #[cfg(target_os = "linux")]
        {
            return Commands::LinuxCommands(LinuxVariation {});
        }
    };
    pub static ref MENU_COMMANDS: Mutex<Vec<MenuCommand>> = {
        let mut commands: Vec<MenuCommand> = Vec::new();
        commands.push(
            MenuCommand::create_instance("Print saved versions", || COMMANDS.print_saved_versions())
        );
        commands.push(
            MenuCommand::create_instance("Print current version", ||
                COMMANDS.print_current_version()
            )
        );
        commands.push(
            MenuCommand::create_instance("Add java version", || COMMANDS.add_java_version())
        );
        commands.push(
            MenuCommand::create_instance("Remove java version", || COMMANDS.remove_java_version())
        );
        commands.push(
            MenuCommand::create_instance("Set java version", || COMMANDS.set_java_version())
        );
        return Mutex::new(commands);
    };
}

#[derive(Clone)]
pub struct MenuCommand {
    name: String,
    func: fn(),
}

impl MenuCommand {
    pub fn create_instance(name: &str, func: fn()) -> MenuCommand {
        return MenuCommand { name: name.to_string(), func: func };
    }

    pub fn get_command_name(&self) -> &str {
        return &self.name;
    }

    pub fn call_func(&self) {
        handle_command(self.func);
    }
}

fn handle_command(func: fn()) {
    let _ = execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0));
    func();
    press_to_continue();
}
