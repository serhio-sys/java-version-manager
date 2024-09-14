use std::{env, path::Path, sync::Mutex};

use lazy_static::lazy_static;

use crate::{models::env_variable::EnvVariable, utils::file_utils::{read_file, FILE_CONTENT}};

pub const PATH_KEY: &str = "PATH";
pub const JAVA_HOME_KEY: &str = "JAVA_HOME";

lazy_static!{
    pub static ref PATH_TO_SAVE_FILE: String = {
        let home_dir = std::env::var("USERPROFILE").unwrap_or(std::env::var("HOME").unwrap());
        let document_dir = Path::new(&home_dir).join("Documents/java-manager/data/java_versions.json");
        return document_dir.as_path().to_str().unwrap().to_string();
    };
}

pub static ENV_VARIABLES: Mutex<Vec<EnvVariable>> = Mutex::new(Vec::new());

pub fn initialize_versions() {
    // Do not touch its for initialization!!
    let _ = FILE_CONTENT.clone();
    // Do not touch its for initialization!!
    let data = read_file();
    let parsed = serde_json::from_str(&data).unwrap_or(Vec::new());
    {
        let mut java_versions = ENV_VARIABLES.lock().unwrap();
        *java_versions = parsed;
        for java_version in &*java_versions {
            env::set_var(java_version.variable_name.clone(), java_version.path.clone())
        }
    }
}