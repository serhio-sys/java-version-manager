use std::{io::stdout, path::Path};

use crossterm::{ cursor::MoveToNextLine, execute, style::{ Color, Print, SetAttribute } };

use crate::program::{ config::ENV_VARIABLES, utils };

use super::models::env_variable::{self, get_java_version_index_by_name, EnvVariable};

pub(super) mod linux_variation;
pub(super) mod win_variation;

#[allow(dead_code)]
pub(super) enum Commands {
    LinuxCommands(linux_variation::LinuxVariation),
    WinCommands(win_variation::WinVariation),
}

pub(super) trait BaseCommands: Sync {
    fn print_saved_versions(&self) {
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

    fn print_current_version(&self);

    fn add_java_version(&self) {
        utils::print_utils::simple_print_line("Enter the path variable name: ");
        let var_name = utils::read_line();
        {
            let java_versions = ENV_VARIABLES.lock().unwrap();
            let index = get_java_version_index_by_name(var_name.as_str(), &java_versions);
            if index != -1 {
                let unwrapped_java_version = java_versions.get(index as usize).unwrap();
                let _ = execute!(
                    stdout(),
                    Print("Variable was found here the java path:"),
                    MoveToNextLine(1),
                    Print(format!("{}", unwrapped_java_version.get_path())),
                    MoveToNextLine(1)
                );
            } else {
                utils::print_utils::print_error_action("Variable was not found");
            }
        }
        utils::print_utils::simple_print_line("Enter valid path: ");
        let path = utils::read_line();
        if Path::new(path.as_str()).exists() {
            let var_name_msg = &format!("[{}] was added successfully.", var_name.as_str());
            save_java_version(EnvVariable::create_instance(var_name.as_str(), path.as_str()));
            utils::print_utils::print_bolt_line_with_color(var_name_msg.as_str(), None);
        } else {
            utils::print_utils::print_bolt_line_with_color(
                "Please enter valid path in next time.",
                Some(Color::Red)
            );
        }
    }

    fn remove_java_version(&self) {
        {
            let mut java_versions = ENV_VARIABLES.lock().unwrap();
            if java_versions.is_empty() {
                utils::print_utils::print_bolt_line_with_color(
                    "There is no available java versions found. Please add any java version.",
                    None
                );
                return;
            }
            utils::print_utils::simple_print_line("Enter the path variable name: ");
            let var_name = utils::read_line();
            let index = env_variable::get_java_version_index_by_name(
                var_name.as_str(),
                &java_versions
            );
            if index == -1 {
                utils::print_utils::print_bolt_line_with_color(
                    "The variable was not found by name.",
                    Some(Color::Red)
                );
            } else {
                let indx: usize = index.try_into().unwrap();
                let java_version = java_versions.swap_remove(indx);
                utils::print_utils::print_success_var_action(
                    "variable was successfully removed",
                    &java_version
                );
            }
        }
        utils::file_utils::save_to_file();
    }

    fn set_java_version(&self);
}

impl BaseCommands for Commands {
    fn print_current_version(&self) {
        match self {
            Commands::LinuxCommands(linux) => linux.print_current_version(),
            Commands::WinCommands(win) => win.print_current_version(),
        }
    }

    fn add_java_version(&self) {
        match self {
            Commands::LinuxCommands(linux) => linux.add_java_version(),
            Commands::WinCommands(win) => win.add_java_version(),
        }
    }

    fn remove_java_version(&self) {
        match self {
            Commands::LinuxCommands(linux) => linux.remove_java_version(),
            Commands::WinCommands(win) => win.remove_java_version(),
        }
    }

    fn set_java_version(&self) {
        match self {
            Commands::LinuxCommands(linux) => linux.set_java_version(),
            Commands::WinCommands(win) => win.set_java_version(),
        }
    }
}

pub(super) fn save_java_version(java_version: EnvVariable) {
    {
        let mut java_versions = match ENV_VARIABLES.try_lock() {
            Ok(guard) => guard,
            Err(_) => panic!("Failed to acquire lock on JAVA_VERSIONS"),
        };
        let index: i32 = env_variable
            ::get_java_version_index_by_name(&java_version.get_variable_name(), &java_versions)
            .try_into()
            .unwrap();
        if index != -1 {
            let registered_java_version = java_versions.get_mut(index as usize).unwrap();
            registered_java_version.set_path(java_version.get_path());
        } else {
            java_versions.push(java_version);
        }
    }
    utils::file_utils::save_to_file();
}
