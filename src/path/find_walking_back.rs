use std::{
    env::current_dir,
    fs::exists,
    path::{Path, PathBuf},
};

pub trait FindWalkingBack {
    /// Finds an existing file or directory by walking backwards from its expected path.
    /// If the provided path does not contain a parent the current dir is used as the starting point.
    /// Always returns Ok(None) if the provided path is empty.
    fn find_walking_back(&self) -> Result<Option<PathBuf>, std::io::Error>;
}

impl FindWalkingBack for Path {
    fn find_walking_back(&self) -> Result<Option<PathBuf>, std::io::Error> {
        let Some(file_or_dir_name) = self.file_name() else {
            return Ok(None);
        };

        let mut curr_dir = if let Some(parent_dir) = self.parent()
            && !parent_dir.as_os_str().is_empty()
        {
            if parent_dir.is_relative() {
                &current_dir()?.join(parent_dir)
            } else {
                parent_dir
            }
        } else {
            &current_dir()?
        };

        let mut curr_path = curr_dir.join(file_or_dir_name);

        loop {
            if exists(&curr_path)? {
                return Ok(Some(curr_path));
            } else if let Some(parent) = curr_dir.parent() {
                curr_dir = parent;
                curr_path = curr_dir.join(&file_or_dir_name);
            } else {
                return Ok(None);
            }
        }
    }
}

impl<T: AsRef<Path>> FindWalkingBack for T {
    fn find_walking_back(&self) -> Result<Option<PathBuf>, std::io::Error> {
        self.as_ref().find_walking_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        env::{current_dir, home_dir},
        fs::{File, remove_file},
    };

    #[test]
    fn should_walk_back_from_current_dir_if_given_only_file_name() {
        let file_name = "utils-rs-tmp-file-1";
        let file_path = home_dir().expect("No home dir").join(file_name);

        File::create_new(&file_path).expect("Could not create file to find");

        let result_from_file_name_only = (file_name).find_walking_back();

        remove_file(&file_path).expect("Could not remove temp file");

        assert_eq!(
            result_from_file_name_only.expect("io error"),
            Some(file_path)
        );
    }

    #[test]
    fn it_should_find_a_given_file_from_absolute_dir() {
        let file_name = "utils-rs-tmp-file-2";
        let file_path = home_dir().expect("No home dir").join(file_name);

        File::create_new(&file_path).expect("Could not create file to find");

        let result_from_absolue_path =
            (current_dir().expect("No current dir").join(file_name)).find_walking_back();

        remove_file(&file_path).expect("Could not remove temp file");

        assert_eq!(result_from_absolue_path.expect("io error"), Some(file_path));
    }

    #[test]
    fn it_should_find_a_given_file_from_relative_dir() {
        let file_name = "utils-rs-tmp-file-3";
        let file_path = home_dir().expect("No home dir").join(file_name);

        File::create_new(&file_path).expect("Could not create file to find");

        let result_from_absolue_path =
            (PathBuf::from("target").join(file_name)).find_walking_back();

        remove_file(&file_path).expect("Could not remove temp file");

        assert_eq!(result_from_absolue_path.expect("io error"), Some(file_path));
    }
}
