use std::fs;
use std::io::prelude::*;

use bincode;
use serde::Serialize;

use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub trait Storable<T> {
    fn write(&mut self, key: String, value: T) -> Result<(), std::io::Error>;
    fn retrive(&self, key: &String) -> Result<T, std::io::Error>;
    fn keys(&self) -> Vec<String>;
}

// File Ops

pub fn create_dir(path: &PathBuf) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    fs::create_dir(path)
}

pub fn write_file<T: Serialize, P: AsRef<Path>>(path: P, content: &T) -> io::Result<()> {
    let serilized = bincode::serialize(content)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut file = File::create(&path).unwrap_or(File::open(&path)?);

    file.write_all(&serilized)?;

    Ok(())
}

pub fn read_file<T, P>(path: P) -> io::Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let deserialized = bincode::deserialize(&buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(deserialized)
}
