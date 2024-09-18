use crate::program::{
    config::{ ENV_VARIABLES, JAVA_HOME_KEY },
    utils::{ self, file_utils::GLOBAL_VARIABLES },
};

use super::{ env_variable::{ self, EnvVariable }, variation::BaseCommands };

pub(super) struct LinuxVariation();

impl BaseCommands for LinuxVariation {
    fn print_current_version() {
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

    fn add_java_version() {
        todo!()
    }

    fn remove_java_version() {
        todo!()
    }

    fn set_java_version() {
        todo!()
    }
}
