use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm_input::input;


pub mod print_utils;
pub mod file_utils;

pub fn read_line() -> String {
    disable_raw_mode().unwrap();
    let input = input();
    let result = input.read_line().unwrap();
    enable_raw_mode().unwrap();
    return result;
}