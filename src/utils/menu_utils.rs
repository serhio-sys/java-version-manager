use std::io::stdout;

use crossterm::{cursor::{self, MoveToNextLine}, execute, style::{Print, SetAttribute, SetForegroundColor}, terminal::{Clear, ClearType}};

use crate::models::{self, env_variable::EnvVariable, menu_command::MenuCommand};

use super::print_utils;

fn print_selected_menu_item(first_part: String, second_part: String) {
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Underlined),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print(first_part),
        SetAttribute(crossterm::style::Attribute::Reset),
        SetAttribute(crossterm::style::Attribute::Underlined),
        Print(second_part),
        SetAttribute(crossterm::style::Attribute::Reset),
        MoveToNextLine(1)
    );
}

fn print_menu_item(first_part: String, second_part: String) {
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print(first_part),
        SetAttribute(crossterm::style::Attribute::Reset),
        Print(second_part),
        MoveToNextLine(1)
    );
}

pub fn print_available_java_versions(java_versions: Vec<EnvVariable>, selected: usize) {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    print_utils::simple_print_line("Select available java version: ");
    for index in 0..java_versions.len() {
        let java_version = java_versions.get(index).unwrap();
        if index == selected {
            print_selected_menu_item(format!("-> {}", java_version.get_variable_name()), format!(": {}", java_version.get_path()));
        } else {  
            print_menu_item(format!("{}", java_version.get_variable_name()), format!(": {}", java_version.get_path()));
        }
    }
}

pub fn print_main_menu(selected: i32) -> Option<MenuCommand>{
    execute!(
        stdout(),
        Clear(crossterm::terminal::ClearType::All),
        cursor::MoveTo(0,0),
        SetForegroundColor(crossterm::style::Color::Green),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print("Java Version Manager"),
        MoveToNextLine(2),
        SetAttribute(crossterm::style::Attribute::Reset),
        SetForegroundColor(crossterm::style::Color::Reset),
    ).unwrap();
    {
        let menu_items = models::menu_command::MENU_COMMANDS.lock().unwrap();
        let selected_menu_item = menu_items.get(selected as usize).unwrap();
        for index in 0..menu_items.len() {
            let menu_item = menu_items.get(index).unwrap();
            if index == selected as usize {
                print_selected_menu_item("-> ".to_string(), menu_item.get_command_name().to_string());
            } else {
                print_menu_item(String::new(), menu_item.get_command_name().to_string())
            }
        }
        execute!(
            stdout(),
            Print("To exit program just press - ["),
            SetAttribute(crossterm::style::Attribute::Bold),
            Print("ESC"),
            SetAttribute(crossterm::style::Attribute::Reset),
            SetForegroundColor(crossterm::style::Color::Reset),
            Print("]"),
            MoveToNextLine(1),
        ).unwrap();
        return Some(selected_menu_item.clone());
    }
}