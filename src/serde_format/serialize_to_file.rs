use super::{error::SerdeFormatError, format::SerdeFormat};

use serde::Serialize;
use std::path::Path;

pub trait SerializeToFile: Serialize + Sized {
    fn to_file(&self, path: impl AsRef<Path>) -> Result<&Self, SerdeFormatError> {
        let contents = SerdeFormat::from_file_extension(&path)
            .unwrap_or_default()
            .serialize_to_string(self)?;

        std::fs::write(path, contents)?;

        Ok(self)
    }
}

impl<T: Serialize + Sized> SerializeToFile for T {}
