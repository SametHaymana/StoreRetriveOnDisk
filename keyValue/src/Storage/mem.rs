use std::collections::HashMap;

use super::utils::Storable;

#[derive(Default, Clone)]
pub struct Mem<T> {
    map: HashMap<String, T>,
}

impl<T: Clone> Storable<T> for Mem<T> {
    fn write(&mut self, key: String, value: T) -> Result<(), std::io::Error> {
        self.map.insert(key, value);
        Result::Ok(())
    }

    fn retrive(&self, key: &String) -> Result<T, std::io::Error> {
        self.map.get(key).cloned().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Key not found",
        ))
    }

    fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
}
