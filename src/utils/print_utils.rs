use std::io::stdout;

use crossterm::{cursor::MoveToNextLine, event::read, execute, style::{Color, Print, SetAttribute, SetForegroundColor}};

use crate::models::env_variable::EnvVariable;

pub fn press_to_continue() {
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print("Press enter for continue..."),
        SetAttribute(crossterm::style::Attribute::Reset),
        MoveToNextLine(1),
    );
    read().unwrap();
}


pub fn print_bolt_line_with_color(string: &str, color: Option<Color>) {
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Bold),
        SetForegroundColor(color.unwrap_or(Color::Reset)),
        Print(string),
        MoveToNextLine(1),
        SetForegroundColor(crossterm::style::Color::Reset),
        SetAttribute(crossterm::style::Attribute::Reset)
    );
}

pub fn print_error_action(msg: &str) {
    print_bolt_line_with_color(msg, Some(Color::Red));
}

pub fn print_success_var_action(msg: &str, java_version: &EnvVariable) {
    let _ = execute!(
        stdout(),
        SetAttribute(crossterm::style::Attribute::Bold),
        Print(format!("[{}]", java_version.get_variable_name())),
        SetAttribute(crossterm::style::Attribute::Reset),
        Print(format!(": {}", msg)),
        MoveToNextLine(1),
    );
}

pub fn simple_print_line(msg: &str) {
    let _ = execute!(
        stdout(),
        Print(msg),
        MoveToNextLine(1),
    );
}
