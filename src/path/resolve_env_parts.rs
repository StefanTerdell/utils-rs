//! Contains the `ResolveEnvParts` trait that replaces env-vars in paths with their value, along with its implementations for `Path` and `T: AsRef<Path>`.

use std::{
    env::vars,
    path::{Path, PathBuf},
};

pub trait ResolveEnvParts {
    /// Replaces any environment variable names starting with $ with their values and returns a PathBuf
    /// A leading `~` is treated like `$HOME`
    ///
    /// Example:
    /// ```rust
    /// use std::{path::PathBuf, env::home_dir};
    /// use utils_rs::prefix::*;
    ///
    /// let resolved = PathBuf::from("~/.ssh").resolve_env_parts();
    /// let expected = home_dir().unwrap().join(".ssh");
    ///
    /// assert_eq!(resolved, expected)
    /// ```
    fn resolve_env_parts(&self) -> PathBuf;
}

impl ResolveEnvParts for Path {
    fn resolve_env_parts(&self) -> PathBuf {
        let mut string = self.to_string_lossy().to_string();

        for (key, value) in vars() {
            string = string.replace(&format!("${key}"), &value);

            if key == "HOME"
                && let Some(tail) = string.strip_prefix("~")
            {
                string = format!("{value}{tail}");
            }
        }

        PathBuf::from(string)
    }
}

impl<T: AsRef<Path>> ResolveEnvParts for T {
    fn resolve_env_parts(&self) -> PathBuf {
        self.as_ref().resolve_env_parts()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::var;

    #[test]
    fn it_should_work() {
        let home = var("HOME").unwrap();
        let cargo = var("CARGO").unwrap();

        let home_hello_cargo = PathBuf::from("~/hello/$CARGO/$NOT_VERY_LIKELY_TO_EXIST_IN_ENV");

        assert_eq!(
            home_hello_cargo.resolve_env_parts(),
            PathBuf::from(format!(
                "{home}/hello/{cargo}/$NOT_VERY_LIKELY_TO_EXIST_IN_ENV"
            ))
        );
    }
}
