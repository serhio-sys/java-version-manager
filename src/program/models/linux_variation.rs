use std::{ env, io::stdout, path::Path };

use crossterm::{
    cursor::MoveToNextLine,
    event::{ read, Event, KeyCode, KeyEvent },
    execute,
    style::{ Color, Print },
};

use crate::program::{
    config::{ ENV_VARIABLES, JAVA_HOME_KEY },
    utils::{ self, file_utils::GLOBAL_VARIABLES },
};

use super::{ env_variable::{ self, EnvVariable }, variation::{ save_java_version, BaseCommands } };

pub struct LinuxVariation();

impl BaseCommands for LinuxVariation {
    fn print_current_version(&self) {
        let java_home: EnvVariable;
        {
            let env_variables = GLOBAL_VARIABLES.lock().unwrap();
            let index = env_variable::get_java_version_index_by_name(JAVA_HOME_KEY, &env_variables);
            if index != -1 {
                java_home = env_variables
                    .get(index as usize)
                    .unwrap()
                    .clone();
            } else {
                utils::print_utils::print_error_action("Java Home variable is not setted.");
                return;
            }
            let java_versions = ENV_VARIABLES.lock().unwrap();
            if
                let Some(unwrapped) = env_variable::get_java_version_by_path(
                    &java_home.get_path().trim_end_matches("/bin"),
                    &java_versions
                )
            {
                utils::print_utils::print_success_var_action("is currently setted", &unwrapped);
            }
        }
    }

    fn add_java_version(&self) {
        utils::print_utils::simple_print_line("Enter the path variable name: ");
        let var_name = utils::read_line();
        let java_version = env::var(var_name.as_str());
        if let Ok(unwrapped_java_version) = java_version {
            let _ = execute!(
                stdout(),
                Print("Variable was found here the java path:"),
                MoveToNextLine(1),
                Print(format!("{}", unwrapped_java_version.as_str())),
                MoveToNextLine(1)
            );
        } else {
            utils::print_utils::print_error_action("Variable was not found");
        }
        utils::print_utils::simple_print_line("Enter valid path: ");
        let path = utils::read_line();
        if Path::new(path.as_str()).exists() {
            env::set_var(var_name.as_str(), path.as_str());
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

    fn set_java_version(&self) {
        let mut selected: i32 = 0;
        {
            let java_versions = ENV_VARIABLES.lock().unwrap();
            if java_versions.is_empty() {
                utils::print_utils::print_bolt_line_with_color(
                    "There is no available java versions found. Please add any java version.",
                    None
                );
                return;
            }
            loop {
                utils::menu_utils::print_available_java_versions(
                    java_versions.clone(),
                    selected as usize
                );
                match read().unwrap() {
                    Event::Key(KeyEvent { code: KeyCode::Up, modifiers: _, kind: _, state: _ }) => {
                        if selected - 1 >= 0 {
                            selected -= 1;
                        }
                    }
                    Event::Key(
                        KeyEvent { code: KeyCode::Down, modifiers: _, kind: _, state: _ },
                    ) => {
                        if selected + 1 < java_versions.len().try_into().unwrap() {
                            selected += 1;
                        }
                    }
                    Event::Key(
                        KeyEvent { code: KeyCode::Enter, modifiers: _, kind: _, state: _ },
                    ) => {
                        utils::file_utils::set_java_home(
                            &java_versions.get(selected as usize).unwrap()
                        );
                        utils::print_utils::simple_print_line(
                            "Java version was setted successfully"
                        );
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
