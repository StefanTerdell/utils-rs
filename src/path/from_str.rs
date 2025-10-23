use std::path::PathBuf;

use crate::from_str::FromStr;

impl FromStr for PathBuf {
    fn from_str(str: &str) -> Self {
        str.into()
    }
}
