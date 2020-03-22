extern crate regex;

use regex::*;
use std::env::*;

pub trait EnvApply {
    /// Apply environment variables on a string.
    fn apply(text: String) -> String;
}

/// Apply environment variables on a string.
impl EnvApply for Vars {
    fn apply(text: String) -> String {
        let key_pattern_regex = Regex::new("[{]{2}[ ]*([^{ ]+)[ ]*[}]{2}").unwrap();
        let mut result = text.clone();
        for key_pattern in key_pattern_regex.find_iter(text.as_ref()) {
            let key_regex = Regex::new("[{]{2}[ ]*([^{ ]+)[ ]*[}]{2}").unwrap();
            let key = key_regex.replace(key_pattern.as_str(), "$1").to_string();
            let value = match var(key.as_str()) {
                Ok(value) => value,
                Err(_) => key_pattern.as_str().to_string(),
            };
            result = result.replace(key_pattern.as_str(), value.as_str());
        }

        result
    }
}

/// Apply OS environment variables on a string.
impl EnvApply for VarsOs {
    fn apply(text: String) -> String {
        let key_pattern_regex = Regex::new("[{]{2}[ ]*([^{ ]+)[ ]*[}]{2}").unwrap();
        let mut result = text.clone();
        for key_pattern in key_pattern_regex.find_iter(text.as_ref()) {
            let key_regex = Regex::new("[{]{2}[ ]*([^{ ]+)[ ]*[}]{2}").unwrap();
            let key = key_regex.replace(key_pattern.as_str(), "$1").to_string();
            let value = match var_os(key.as_str()) {
                Some(os) => os.to_string_lossy().to_string(),
                None => key_pattern.as_str().to_string(),
            };
            result = result.replace(key_pattern.as_str(), value.as_str());
        }

        result
    }
}

#[cfg(test)]
mod env_applier_test {
    use super::*;
    #[test]
    fn it_should_apply_env_var_on_string() {
        let string = r#"{"test":"{{ ENV_KEY }}"}"#;
        set_var("ENV_KEY", "ENV_VALUE");
        let result = Vars::apply(string.to_string());
        assert_eq!(r#"{"test":"ENV_VALUE"}"#, result);
    }
    #[test]
    fn it_should_apply_env_var_os_on_string() {
        let string = r#"{"test":"{{ HOME }}"}"#;
        set_var("HOME", "MY_HOME");
        let result = VarsOs::apply(string.to_string());
        assert_eq!(r#"{"test":"MY_HOME"}"#, result);
    }
}
