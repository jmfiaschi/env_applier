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
        let key_pattern_regex = Regex::new("[{]{2}[ ]*([a-zA-Z0-9_-]+)[ ]*[}]{2}").unwrap();
        let mut result = text.clone();
        for key_pattern in key_pattern_regex.find_iter(text.as_ref()) {
            let key_regex = Regex::new("[{]{2}[ ]*([a-zA-Z0-9_-]+)[ ]*[}]{2}").unwrap();
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
        let key_pattern_regex = Regex::new("[{]{2}[ ]*([a-zA-Z0-9_-]+)[ ]*[}]{2}").unwrap();
        let mut result = text.clone();
        for key_pattern in key_pattern_regex.find_iter(text.as_ref()) {
            let key_regex = Regex::new("[{]{2}[ ]*([a-zA-Z0-9_-]+)[ ]*[}]{2}").unwrap();
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
mod tests {
    use super::*;
    #[test]
    fn it_should_apply_env_var_on_string() {
        let string = r#"{"field1":"{{ ENV_KEY1 }}","field2":"{{ENV_KEY2}}"}"#;
        set_var("ENV_KEY1", "ENV_VALUE1");
        set_var("ENV_KEY2", "ENV_VALUE2");
        let result = Vars::apply(string.to_string());
        assert_eq!(r#"{"field1":"ENV_VALUE1","field2":"ENV_VALUE2"}"#, result);
    }
    #[test]
    fn it_should_apply_env_var_os_on_string() {
        let string = r#"{"field1":"{{ ENV_KEY1 }}","field2":"{{ENV_KEY2}}"}"#;
        set_var("ENV_KEY1", "ENV_VALUE1");
        set_var("ENV_KEY2", "ENV_VALUE2");
        let result = VarsOs::apply(string.to_string());
        assert_eq!(r#"{"field1":"ENV_VALUE1","field2":"ENV_VALUE2"}"#, result);
    }
}
