#[cfg(windows)]
use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind };

#[cfg(windows)]
use winreg::{ enums::{ HKEY_CURRENT_USER, KEY_READ, KEY_WRITE }, RegKey };

use crate::program::models::env_variable::EnvVariable;
#[cfg(windows)]
use crate::program::{
    config::{ ENV_VARIABLES, JAVA_HOME_KEY, PATH_KEY },
    models::env_variable::{ self },
    utils,
};
use super::BaseCommands;
use windows_sys::Win32::{Foundation::{LRESULT}, UI::WindowsAndMessaging::{HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE}};
use windows_sys::Win32::Foundation::LPARAM;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, sync::RwLockReadGuard};

pub struct WinVariation();

impl BaseCommands for WinVariation {
    #[cfg(windows)]
    fn print_current_version(&self) {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let cur_ver = hklm
            .open_subkey("Environment")
            .unwrap();
        let java_home: Result<String, std::io::Error> = cur_ver.get_value(JAVA_HOME_KEY);
        if let Ok(value) = java_home {
            let java_versions = ENV_VARIABLES.read().unwrap();
            if
                let Some(unwrapped) = env_variable::get_java_version_by_path(
                    &value.trim_end_matches("\\bin"),
                    &java_versions
                )
            {
                utils::print_utils::print_success_var_action("is currently setted", &unwrapped);
            }
        } else {
            utils::print_utils::print_error_action("Java Home variable is not setted.");
            return;
        }
    }

    #[cfg(windows)]
    fn set_java_version(&self) {
        let mut selected: i32 = 0;
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let cur_ver = hklm
            .open_subkey_with_flags(
                "Environment",
                KEY_READ | KEY_WRITE
            )
            .unwrap();
        let java_versions = ENV_VARIABLES.read().unwrap();
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
                    KeyEvent { code: KeyCode::Enter, modifiers: _, kind: _, state: _ } => {
                        do_set_java_version(selected, &java_versions, &cur_ver);
                        broadcast_environment_change();
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

}


fn do_set_java_version(selected: i32, java_versions: &RwLockReadGuard<'_,Vec<EnvVariable>>, cur_ver: &RegKey) {
    let java_version = java_versions.get(selected as usize).unwrap();
    let java_home_old: Result<String, std::io::Error> = cur_ver.get_value(
        JAVA_HOME_KEY
    );
    let _ = cur_ver.set_value(
        JAVA_HOME_KEY,
        &(java_version.get_path().to_owned())
    );
    let path: Result<String, std::io::Error> = cur_ver.get_value(PATH_KEY);
    if let Ok(value) = path {
        let mut new_value = value.clone();
        if let Ok(value_java_home) = java_home_old {
            for pattern in [
                format!(";{}\\bin", value_java_home),
                format!("{}\\bin;", value_java_home),
                format!("{}\\bin", value_java_home),
            ] {
                new_value = new_value.replace(&pattern, "");
            }

            new_value = new_value.trim_end_matches(';').to_string();

            let new_java_bin = format!("{}\\bin", java_version.get_path());
            if !new_value.contains(&new_java_bin) {
                new_value.push(';');
                new_value.push_str(&new_java_bin);
            }
        }

        let _ = cur_ver.set_value(PATH_KEY, &new_value);
    }
}

fn broadcast_environment_change() {
    unsafe {
        let param: Vec<u16> = OsStr::new("Environment")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let result: LRESULT = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            param.as_ptr() as LPARAM,
            SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );

        if result == 0 {
            eprintln!("⚠️ SendMessageTimeoutW failed (GetLastError may give more info)");
        }
    }
    utils::print_utils::simple_print_line(
        "✅ Java version was setted successfully"
    );
}
