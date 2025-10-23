use std::path::{Path, PathBuf};

use crate::as_str::AsStr;

impl AsStr for PathBuf {
    fn as_str(&self) -> &str {
        self.as_os_str().to_str().unwrap_or_default()
    }
}

impl AsStr for Path {
    fn as_str(&self) -> &str {
        self.as_os_str().to_str().unwrap_or_default()
    }
}
