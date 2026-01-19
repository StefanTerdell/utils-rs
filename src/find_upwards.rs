pub trait FindUpwards {
    /// Finds an existing file or directory by walking backwards from its expected path.
    /// If the provided path does not contain a parent the current dir is used as the starting point.
    /// Always returns Ok(None) if the provided path is empty.
    fn find_upwards(&self) -> Result<Option<std::path::PathBuf>, std::io::Error>;
}
