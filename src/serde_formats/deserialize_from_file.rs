use super::{error::SerdeFormatError, format::SerdeFormat};

use serde::de::DeserializeOwned;
use std::path::Path;

pub trait DeserializeFromFile: DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> Result<Self, SerdeFormatError> {
        SerdeFormat::from_file_extension(&path)
            .unwrap_or_default()
            .deserialize_from_slice(&std::fs::read(&path)?)
    }
}

impl<T: DeserializeOwned> DeserializeFromFile for T {}
