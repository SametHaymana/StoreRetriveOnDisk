use std::{fs, path::{Path, PathBuf}};

use serde::{de::DeserializeOwned, Serialize};

use super::utils::{self, Storable};

#[derive(Debug)]
pub struct Disk {
    main_dir: PathBuf,
}

impl Disk {
    pub fn new(main_dir: String) -> Self {
        let path = PathBuf::from(main_dir);
        utils::create_dir(&path).unwrap();

        Disk {
            main_dir: path
        }
    }

}

impl<T: Serialize + DeserializeOwned> Storable<T> for Disk {
    fn write(&mut self, key: String, value: T) -> Result<(), std::io::Error> {
        let binding = (self.main_dir.join(key.clone()));
        let path = Path::new(&binding);

        utils::write_file(path, &value)?;

        Ok(())
    }

    fn retrive(&self, key: &String) -> Result<T, std::io::Error> {
        let binding = self.main_dir.clone().join(key.clone());
        let path = Path::new(&binding);

        utils::read_file(path)
    }

    fn keys(&self) -> Vec<String> {
        // List all files in the directory
        let paths = std::fs::read_dir(&self.main_dir).unwrap();
        paths
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    }
}
