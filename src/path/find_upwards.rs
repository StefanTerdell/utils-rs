use crate::find_upwards::FindUpwards;

impl FindUpwards for std::path::Path {
    fn find_upwards(&self) -> Result<Option<std::path::PathBuf>, std::io::Error> {
        let Some(file_or_dir_name) = self.file_name() else {
            return Ok(None);
        };

        let mut curr_dir = if let Some(parent_dir) = self.parent()
            && !parent_dir.as_os_str().is_empty()
        {
            if parent_dir.is_relative() {
                &std::env::current_dir()?.join(parent_dir)
            } else {
                parent_dir
            }
        } else {
            &std::env::current_dir()?
        };

        let mut curr_path = curr_dir.join(file_or_dir_name);

        loop {
            if std::fs::exists(&curr_path)? {
                return Ok(Some(curr_path));
            } else if let Some(parent) = curr_dir.parent() {
                curr_dir = parent;
                curr_path = curr_dir.join(file_or_dir_name);
            } else {
                return Ok(None);
            }
        }
    }
}

impl<T: AsRef<std::path::Path>> FindUpwards for T {
    fn find_upwards(&self) -> Result<Option<std::path::PathBuf>, std::io::Error> {
        self.as_ref().find_upwards()
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

        let result_from_file_name_only = (file_name).find_upwards();

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
            (current_dir().expect("No current dir").join(file_name)).find_upwards();

        remove_file(&file_path).expect("Could not remove temp file");

        assert_eq!(result_from_absolue_path.expect("io error"), Some(file_path));
    }

    #[test]
    fn it_should_find_a_given_file_from_relative_dir() {
        let file_name = "utils-rs-tmp-file-3";
        let file_path = home_dir().expect("No home dir").join(file_name);

        File::create_new(&file_path).expect("Could not create file to find");

        let result_from_absolue_path =
            (std::path::PathBuf::from("target").join(file_name)).find_upwards();

        remove_file(&file_path).expect("Could not remove temp file");

        assert_eq!(result_from_absolue_path.expect("io error"), Some(file_path));
    }
}
