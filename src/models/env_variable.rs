use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EnvVariable {
    variable_name: String,
    path: String
}

impl EnvVariable {
    pub fn get_export_string(&self) -> String {
        return format!("export {}={}", self.variable_name, self.path);
    }

    pub fn create_instance(var_name: &str, path: &str) -> EnvVariable {
        return EnvVariable {variable_name: var_name.to_string(), path: path.to_string()};
    }

    pub fn get_variable_name(&self) -> &str {
        return &self.variable_name;   
    }

    pub fn get_path(&self) -> &str {
        return &self.path;   
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();   
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