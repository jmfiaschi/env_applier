use std::env;

pub trait EnvApply<'a> {
    /// Apply environment variables on a string.
    fn apply(&'a self) -> String;

    /// Apply environment variables by prepending a prefix to the variable names.
    fn apply_with_prefix(&'a self, prefix: &str) -> String;
}

/// Apply environment variables on a string.
impl<'a> EnvApply<'a> for String {
    fn apply(&self) -> String {
        apply(self, "")
    }

    fn apply_with_prefix(&self, prefix: &str) -> String {
        apply(self, prefix)
    }
}

impl<'a> EnvApply<'a> for str {
    fn apply(&'a self) -> String {
        apply(self, "")
    }

    fn apply_with_prefix(&'a self, prefix: &str) -> String {
        apply(self, prefix)
    }
}

/// Function to replace placeholders with environment variables
/// Handles both normal and prefixed environment variable lookups
fn apply(text: &str, prefix: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut start = 0;

    while let Some(open) = text[start..].find("{{") {
        let open = start + open;
        if let Some(close) = text[open..].find("}}") {
            let close = open + close + 2;

            // Add the part before the placeholder
            result.push_str(&text[start..open]);

            // Extract the key and trim it
            let key = text[open + 2..close - 2].trim();

            // Prepend the prefix and get the environment variable value
            let full_key = format!("{}{}", prefix, key);
            let value = env::var(&full_key).unwrap_or_else(|_| text[open..close].to_string());

            // Add the value
            result.push_str(&value);

            // Move the start position
            start = close;
        } else {
            break; // If there's no closing "}}", stop processing
        }
    }

    // Add the remaining part of the string
    result.push_str(&text[start..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_apply_env_var_on_string() {
        let config = r#"{"field1":"{{ ENV_KEY1 }}","field2":"{{ENV_KEY2}}"}"#;
        env::set_var("ENV_KEY1", "VALUE1");
        env::set_var("ENV_KEY2", "VALUE2");
        assert_eq!(r#"{"field1":"VALUE1","field2":"VALUE2"}"#, config.apply());
    }

    #[test]
    fn it_should_apply_env_var_by_prefix() {
        let config = r#"{"field1":"{{ ENV_KEY1 }}","field2":"{{ENV_KEY2}}"}"#;
        env::set_var("DEV_ENV_KEY1", "DEV_VALUE1");
        env::set_var("DEV_ENV_KEY2", "DEV_VALUE2");
        assert_eq!(
            r#"{"field1":"DEV_VALUE1","field2":"DEV_VALUE2"}"#,
            config.apply_with_prefix("DEV_")
        );
    }

    #[test]
    fn it_should_handle_missing_placeholders_gracefully() {
        let config =
            r#"{"field1":"{{ ENV_KEY1 }}","field2":"{{ENV_KEY2}}","field3":"{{ MISSING_KEY }}"}"#;
        env::set_var("ENV_KEY1", "VALUE1");
        env::set_var("ENV_KEY2", "VALUE2");
        assert_eq!(
            r#"{"field1":"VALUE1","field2":"VALUE2","field3":"{{ MISSING_KEY }}"}"#,
            config.apply()
        );
    }
}
