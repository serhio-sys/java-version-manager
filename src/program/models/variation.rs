use std::io::stdout;

use crossterm::{ cursor::MoveToNextLine, execute, style::{ Print, SetAttribute } };

use crate::program::{ config::ENV_VARIABLES, utils };

use super::{
    env_variable::{ self, EnvVariable },
    linux_variation::LinuxVariation,
    win_variation::WinVariation,
};

#[allow(dead_code)]
pub(super) enum Commands {
    LinuxCommands(LinuxVariation),
    WinCommands(WinVariation),
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

    fn add_java_version(&self);

    fn remove_java_version(&self);

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
