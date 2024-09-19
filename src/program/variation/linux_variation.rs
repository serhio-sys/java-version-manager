use crossterm::event::{ read, Event, KeyCode, KeyEvent };

use crate::program::{
    config::{ ENV_VARIABLES, JAVA_HOME_KEY },
    utils::{ self, file_utils::GLOBAL_VARIABLES },
};

use super::{ env_variable::{ self, EnvVariable }, BaseCommands };

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
