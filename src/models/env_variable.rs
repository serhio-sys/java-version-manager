use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EnvVariable {
    pub variable_name: String,
    pub path: String
}

impl PartialEq for EnvVariable {
    fn eq(&self, other: &Self) -> bool {
        self.variable_name == other.variable_name && self.path == other.path
    }
}

impl Clone for  EnvVariable {

    fn clone(&self) -> Self {
        Self { variable_name: self.variable_name.clone(), path: self.path.clone() }
    }
}

impl EnvVariable {
    pub fn get_export_string(&self) -> String {
        return format!("export {}={}", self.variable_name, self.path);
    }
}

pub fn get_java_version_index_by_name(var_name: &str, list: &Vec<EnvVariable>) -> i32 {
    for java_version in list {
        if java_version.variable_name.eq(var_name) {
            if let Some(index) = list.iter().position(|item| item == java_version) {
                return index as i32;
            }
        }
    }
    return -1;
}

pub fn get_java_version_by_path(var_path: &str, list: &Vec<EnvVariable>) -> Option<EnvVariable> {
    for java_version in list {
        if java_version.path.eq(var_path) {
            if let Some(index) = list.iter().position(|item| item == java_version) {
                return Some(list.get(index).unwrap().clone());
            }
        }
    }
    return None;
}