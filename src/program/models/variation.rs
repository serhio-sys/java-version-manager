use std::io::stdout;

use crossterm::{ cursor::MoveToNextLine, execute, style::{ Print, SetAttribute } };

use crate::program::{ config::ENV_VARIABLES, utils };

pub(super) trait BaseCommands {
    fn print_saved_versions() {
        {
            let java_versions = ENV_VARIABLES.lock().unwrap();
            if java_versions.is_empty() {
                utils::print_utils::print_bolt_line_with_color(
                    "There is no available java versions found. Please add any java version.",
                    None
                );
                return;
            }
            for index in 0..java_versions.len() {
                let java_version = java_versions.get(index).unwrap();
                let _ = execute!(
                    stdout(),
                    SetAttribute(crossterm::style::Attribute::Bold),
                    Print(format!("{}", java_version.get_variable_name())),
                    SetAttribute(crossterm::style::Attribute::Reset),
                    Print(format!(": {}", java_version.get_path())),
                    MoveToNextLine(1)
                );
            }
        }
    }

    fn print_current_version();

    fn add_java_version();

    fn remove_java_version();

    fn set_java_version();
}
