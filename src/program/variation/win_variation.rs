use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use winreg::{enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE}, RegKey};

use crate::program::{config::{ENV_VARIABLES, JAVA_HOME_KEY, PATH_KEY}, models::env_variable::{self}, utils};

use super::BaseCommands;

pub struct WinVariation();

impl BaseCommands for WinVariation {
    fn print_current_version(&self) {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let cur_ver = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment").unwrap();
        let java_home:Result<String, std::io::Error> = cur_ver.get_value(JAVA_HOME_KEY);
        if let Ok(value) = java_home {
            {
                let java_versions = ENV_VARIABLES.lock().unwrap();
                if
                    let Some(unwrapped) = env_variable::get_java_version_by_path(
                        &value.trim_end_matches("\\bin"),
                        &java_versions
                    )
                {
                    utils::print_utils::print_success_var_action("is currently setted", &unwrapped);
                }
            }
        } else {
            utils::print_utils::print_error_action("Java Home variable is not setted.");
            return;
        }
    }

    fn set_java_version(&self) {
        let mut selected: i32 = 0;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let cur_ver = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_READ | KEY_WRITE).unwrap();
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
                            if selected + 1 < java_versions.len().try_into().unwrap() {
                                selected += 1;
                            }
                        }
                        KeyEvent { code: KeyCode::Enter, modifiers: _, kind: _, state: _ }  => {
                            let java_version = java_versions.get(selected as usize).unwrap();
                            let java_home_old:Result<String, std::io::Error> = cur_ver.get_value(JAVA_HOME_KEY);
                            let _ = cur_ver.set_value(JAVA_HOME_KEY, &(java_version.get_path().to_owned() + "\\bin"));
                            let path:Result<String, std::io::Error> = cur_ver.get_value(PATH_KEY);
                            if let Ok(value) = path {
                                let mut new_value = value.clone();
                                if let Ok(value_java_home) = java_home_old {
                                    new_value = new_value.replace(format!(":{}", value_java_home).as_str(), "");
                                    new_value = new_value.trim_end_matches(":").to_owned()+&java_version.get_path().to_owned() + "\\bin";
                                }
                                let _ = cur_ver.set_value(PATH_KEY, &new_value);
                            }
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
}
