use serde::{Deserialize, Serialize};
use std::path::Path;

use super::error::{SerdeFormatError, TomlSerdeFormatError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SerdeFormat {
    #[default]
    Json,
    Yaml,
    Toml,
}

impl SerdeFormat {
    pub fn from_file_extension(path: impl AsRef<Path>) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|x| x.to_str())
            .and_then(|x| match x.to_lowercase().as_str() {
                "json" => Some(Self::Json),
                "yaml" | "yml" => Some(Self::Yaml),
                "toml" => Some(Self::Toml),
                _ => None,
            })
    }

    pub fn deserialize_from_slice<'de, T: Deserialize<'de>>(
        &self,
        slice: &'de [u8],
    ) -> Result<T, SerdeFormatError> {
        Ok(match self {
            SerdeFormat::Json => serde_json::from_slice(slice)?,
            SerdeFormat::Yaml => serde_yml::from_slice(slice)?,
            SerdeFormat::Toml => toml::from_slice(slice).map_err(TomlSerdeFormatError::from)?,
        })
    }

    pub fn serialize_to_string<T: Serialize>(&self, value: &T) -> Result<String, SerdeFormatError> {
        Ok(match self {
            SerdeFormat::Json => serde_json::to_string_pretty(value)?,
            SerdeFormat::Yaml => serde_yml::to_string(value)?,
            SerdeFormat::Toml => {
                toml::to_string_pretty(value).map_err(TomlSerdeFormatError::from)?
            }
        })
    }
}
