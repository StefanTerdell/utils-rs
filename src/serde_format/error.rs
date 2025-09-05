#[derive(Debug, thiserror::Error)]
pub enum SerdeFormatError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yml::Error),
    #[error(transparent)]
    Toml(#[from] TomlSerdeFormatError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TomlSerdeFormatError {
    #[error(transparent)]
    De(#[from] toml::de::Error),
    #[error(transparent)]
    Ser(#[from] toml::ser::Error),
}
