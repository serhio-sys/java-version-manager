use std::{fs::{self, File, OpenOptions}, io::{read_to_string, Error, Write}, path::Path, sync::Mutex};

use lazy_static::lazy_static;

use crate::{config::{self, JAVA_HOME_KEY, PATH_KEY}, models::env_variable::{get_java_version_index_by_name, EnvVariable}};

use super::print_utils::simple_print_line;

const EMPTY_ENV_VAR: EnvVariable = EnvVariable{ variable_name: String::new(), path: String::new() };

pub static GLOBAL_VARIABLES: Mutex<Vec<EnvVariable>> = Mutex::new(Vec::new());

lazy_static!{
    static ref BASE_VAR_PATH: String = {
        let home_dir = std::env::var("USERPROFILE").unwrap_or(std::env::var("HOME").unwrap());
        let document_dir = Path::new(&home_dir).join(".bashrc");
        return document_dir.as_path().to_str().unwrap().to_string();
    };
    pub static ref FILE_CONTENT: String = extract_variables_from_file();
}


pub fn set_java_home(java_var: &EnvVariable) {
    let mut content = FILE_CONTENT.clone();
    let java_home_var: EnvVariable = EnvVariable{variable_name:JAVA_HOME_KEY.to_string(), path: java_var.path.trim_end_matches('/').to_string() + "/bin"};
    let mut path_var: EnvVariable;
    {
        let mut data = GLOBAL_VARIABLES.lock().unwrap();
        let mut index = get_java_version_index_by_name(PATH_KEY, &data);
        if index == -1 {
            panic!("Path variable not found... Check .bashrc file for containing PATH variable");
        }
        path_var = data.get(index as usize).unwrap().clone();
        index = get_java_version_index_by_name(JAVA_HOME_KEY, &data);
        if index != -1 {    
            data.remove(index as usize);
            data.push(java_home_var.clone());   
        }
    }
    
    path_var.path = format!("{}:${}", path_var.path.replace(format!(":${}", JAVA_HOME_KEY).as_str(), ""), java_home_var.variable_name);
    content += format!("\n{}\n", java_home_var.get_export_string()).as_str();
    content += format!("{}", path_var.get_export_string()).as_str();

    let file = OpenOptions::new().write(true).truncate(true).open(BASE_VAR_PATH.as_str());
    if let Ok(mut opened_file) = file {
        let _ = opened_file.write_all(content.as_bytes());
    }
}

fn extract_variables_from_file() -> String {
    let mut content = read_to_string(File::open(BASE_VAR_PATH.as_str()).unwrap()).unwrap();
    let java_home = get_var_from_content(&content, JAVA_HOME_KEY);
    let path = get_var_from_content(&content, PATH_KEY);
    add_and_remove_from_content(&mut content, &java_home.unwrap_or(EMPTY_ENV_VAR));
    let mut unwrapped_path = path.unwrap_or(EMPTY_ENV_VAR);
    add_and_remove_from_content(&mut content, &unwrapped_path);
    unwrapped_path.path = unwrapped_path.path.clone().replace(format!(":${}", JAVA_HOME_KEY).as_str(), "");
    return content.trim_end_matches("\n").to_string();
}

fn add_and_remove_from_content(content: &mut String, op_var: &EnvVariable) {
    if *op_var == EMPTY_ENV_VAR {
        return;
    }
    add_env_variable(&op_var);
    *content = content.replace(&*op_var.get_export_string(), "");
}

fn add_env_variable(op_var: &EnvVariable) {
    {
        let mut data = GLOBAL_VARIABLES.lock().unwrap();
        data.push(EnvVariable{variable_name: op_var.variable_name.clone(), path: op_var.path.clone()});
    }
}

fn get_var_from_content(content: &str, variable_name: &str) -> Option<EnvVariable>{
    for content_line in content.lines() {
        if content_line.contains(format!("export {}", variable_name).as_str()) {
            let line = content_line.to_string();
            let splited_line: Vec<String> = line.replace("export ", "").split("=").map(|s| s.to_owned()).collect();
            if let Some(val) = splited_line.get(1) {
                return Some(EnvVariable { variable_name: variable_name.to_string(), path: val.clone()});
            }
        }
    }
    return None;
}

pub fn save_to_file() {
    {
        let data = config::ENV_VARIABLES.lock().unwrap();
        let data_string = serde_json::to_string_pretty(&*data);
        let unwrapped_data_string = data_string.unwrap();
        let file = File::create(config::PATH_TO_SAVE_FILE.as_str()).unwrap().write_all(unwrapped_data_string.as_bytes());
        match file {
            Ok(_) => {
                simple_print_line("Data saved to file successfully");
            },
            Err(e) => panic!("Error saving data to file: {}", e),
        }
    }
    config::initialize_versions();
}

pub fn read_file() -> String {
    let content = read_to_string(create_or_get_file_by_path(config::PATH_TO_SAVE_FILE.as_str()).unwrap());
    if let Ok(string_content) = content {
        return string_content;
    } else {
        panic!("Error in time reading file.");
    }
}

fn create_or_get_file_by_path(path: &str) -> Result<File, Error> {
    if check_is_exists(path) {
        return File::open(path);
    } else {
        let _ = create_folder(path);
        return create_file(path);
    }
}

fn create_folder(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let dir_path = path.parent().unwrap();
    fs::create_dir_all(dir_path)?;
    Ok(())
}

fn create_file(path: &str) -> Result<File, Error> {
    let file = File::create(path);
    return file;
}

fn check_is_exists(path: &str) -> bool {
    return Path::new(path).is_file();
}

