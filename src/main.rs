use std::{env, io::stdout, path::Path};

use config::{ENV_VARIABLES, JAVA_HOME_KEY};
use crossterm::{cursor::{self, MoveToNextLine}, event::{read, Event, KeyCode, KeyEvent}, execute, style::{Color, Print, SetAttribute}, terminal::{enable_raw_mode, Clear, ClearType}};
use models::env_variable::EnvVariable;
use utils::{file_utils::GLOBAL_VARIABLES, print_utils::press_to_continue};

mod config;
mod models;
mod utils;

fn main() {    
    config::initialize_versions();
    program();
}

fn program() {
    enable_raw_mode().unwrap();
    loop {
        print_menu_header();
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('1'),
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                handle_command(print_saved_versions);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                handle_command(print_current_version);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('3'),
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                handle_command(add_java_version);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('4'),
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                handle_command(remove_java_version);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('5'),
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                handle_command(set_java_version);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                break;
            }
            _ => {}
        }
    }
}

fn handle_command(func: fn()) {
    let _ = execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0,0),
    );
    func();
    press_to_continue();
}

fn print_saved_versions() {
    {
        let java_versions = ENV_VARIABLES.lock().unwrap();
        for index in 0..java_versions.len() {
            let java_version = java_versions.get(index).unwrap();
            let _ = execute!(
                stdout(),
                SetAttribute(crossterm::style::Attribute::Bold),
                Print(format!("{}", java_version.variable_name)),
                SetAttribute(crossterm::style::Attribute::Reset),
                Print(format!(": {}", java_version.path)),
                MoveToNextLine(1)
            );
        }
    }
}

fn print_current_version() {
    let java_home: EnvVariable;
    {
        let env_variables = GLOBAL_VARIABLES.lock().unwrap();
        let index = models::env_variable::get_java_version_index_by_name(JAVA_HOME_KEY, &env_variables);
        if index != -1 {
            java_home = env_variables.get(index as usize).unwrap().clone();
        } else {
            utils::print_utils::print_error_action("Java Home variable is not setted.");
            return;
        }
        let java_versions = ENV_VARIABLES.lock().unwrap();
        if let Some(unwrapped) = models::env_variable::get_java_version_by_path(&java_home.path.trim_end_matches("/bin"), &java_versions) {
            utils::print_utils::print_success_var_action("is currently setted", &unwrapped);
        }
    }
}

fn add_java_version() {
    utils::print_utils::simple_print_line("Enter the path variable name: ");
    let var_name = utils::read_line();
    let java_version = env::var(var_name.as_str());
    if let Ok(unwrapped_java_version) = java_version {
        let _ = execute!(
            stdout(),
            Print("Variable was found here the java path:"),
            MoveToNextLine(1),
            Print(format!("{}", unwrapped_java_version.as_str())),
            MoveToNextLine(1),
        );
    } else {
        utils::print_utils::print_error_action("Variable was not found");
    }
    utils::print_utils::simple_print_line("Enter valid path: ");
    let path = utils::read_line();
    if Path::new(path.as_str()).exists() {
        env::set_var(var_name.as_str(), path.as_str());
        let var_name_msg = &format!{"[{}] was setted successfully.", var_name.as_str()};
        save_java_version(EnvVariable{variable_name:var_name.clone(), path: path});
        utils::print_utils::print_bolt_line_with_color(var_name_msg.as_str(), None);
    } else {
        utils::print_utils::print_bolt_line_with_color("Please enter valid path in next time.", Some(Color::Red));
    }
}

fn remove_java_version() {
    utils::print_utils::simple_print_line("Enter the path variable name: ");
    let var_name = utils::read_line();
    {
        let mut java_versions = ENV_VARIABLES.lock().unwrap();
        let index = models::env_variable::get_java_version_index_by_name(var_name.as_str(), &java_versions);
        if index == -1 {
            utils::print_utils::print_bolt_line_with_color("The variable was not found by name.", Some(Color::Red));
        } else {
            let indx: usize = index.try_into().unwrap();
            let java_version = java_versions.swap_remove(indx);
            env::remove_var(java_version.variable_name.clone());
            utils::print_utils::print_success_var_action("variable was successfully removed", &java_version);
        }
    }
    utils::file_utils::save_to_file();
}

fn set_java_version() {
    utils::print_utils::simple_print_line("Enter the path variable name: ");
    let var_name = utils::read_line();
    let java_var: EnvVariable;
    {
        let java_versions = ENV_VARIABLES.lock().unwrap();
        let index = models::env_variable::get_java_version_index_by_name(var_name.as_str(), &java_versions);
        if index != -1 {
            java_var = java_versions.get(index as usize).unwrap().clone();
        } else {
            panic!("");
        }
    }
    utils::file_utils::set_java_home(&java_var);
}

fn save_java_version(java_version: EnvVariable) {
    {
        let mut java_versions = match ENV_VARIABLES.try_lock() {
            Ok(guard) => guard,
            Err(_) => panic!("Failed to acquire lock on JAVA_VERSIONS"),
        };
        let index: i32 = models::env_variable::get_java_version_index_by_name(&java_version.variable_name, &java_versions).try_into().unwrap();
        if index != -1 {
            let registered_java_version = java_versions.get_mut(index as usize).unwrap();
            registered_java_version.path = java_version.path;
        } else {
            java_versions.push(java_version);
        }
    }
    utils::file_utils::save_to_file();
}


fn print_menu_header() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print("Java Version Manager"),
        SetAttribute(crossterm::style::Attribute::Reset),
        MoveToNextLine(1),
        Print("1) Print saved versions"),
        MoveToNextLine(1),
        Print("2) Print current version"),
        MoveToNextLine(1),
        Print("3) Add java version"),
        MoveToNextLine(1),
        Print("4) Remove java version"),
        MoveToNextLine(1),
        Print("5) Set java version"),
        MoveToNextLine(1),
        Print("To exit the program please press - "),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print("ESC"),
        MoveToNextLine(1),
        SetAttribute(crossterm::style::Attribute::Reset)
    );
}
