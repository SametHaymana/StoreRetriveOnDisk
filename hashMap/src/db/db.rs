use std::fs::File;
use std::{collections::HashMap, io, path::PathBuf};

// Remove unused import for `hash::Hash`
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::io::BufRead;

// Import Index struct from your previous code
use super::index::Index;


#[derive(Serialize, Deserialize, Clone,Debug)]
struct KV
{
    collections: HashMap<String, Index>,
}




impl KV
{
    // Constructor for KV struct
    pub fn new() -> Self {
        let mut k = KV {
            collections: HashMap::new(),
        };
        k.read_config().unwrap();

        k
    }

    fn read_config(&mut self) -> io::Result<()> {
        // Read the config file
        // Config file location is ".data/config.txt"
        /*
            Format

            Collection_name: path of serialized Index struct
        */

        // Check if folder exists
        std::fs::create_dir_all(".data")?;

        // Check if config file exists
        let file = File::open(".data/config.txt").or_else(|_| File::create(".data/config.txt"))?;


        let reader = io::BufReader::new(file);

        reader.lines().for_each(|line| {
            let line = line.unwrap();
            let mut iter = line.split(":");
            let collection_name = iter.next().unwrap().to_string();
            let path = iter.next().unwrap().to_string();
            let index: Index = super::io::deserialize_from_file(PathBuf::from(path)).unwrap();
            self.collections.insert(collection_name, index);
        });


        if(self.collections.len() == 0)
        {
            self.create_collection("default".to_string());
        }

        Ok(())
    }


    fn create_collection(&mut self, collection_name: String) {
        let index = Index::new(Some(collection_name.clone()));
        self.collections.insert(collection_name, index);
    }

    fn get_collection(&self, collection_name: &String) -> &Index {
        self.collections.get(collection_name).unwrap()
    }

    fn get_collection_mut(&mut self, collection_name: &String) -> &mut Index {
        self.collections.get_mut(collection_name).unwrap()
    }
}



// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_new() {
        let kv = KV::new();
        println!("{:?}", kv);
        assert_eq!(kv.collections.len(), 1);
    }

    #[test]
    fn test_kv_create_collection() {
        let mut kv = KV::new();
        kv.create_collection("test".to_string());
        assert_eq!(kv.collections.len(), 2);
    }

    #[test]
    fn test_kv_get_collection() {
        let kv = KV::new();
        let index = kv.get_collection(&"default".to_string());
        assert_eq!(index.collection(), "default");
    }

    #[test]
    fn test_write_read() {
        let mut kv = KV::new();
        kv.create_collection("test".to_string());
        
        // Adjusted to get a mutable reference
        let index = kv.get_collection_mut(&"test".to_string()); // Assuming the method is now get_collection_mut
        index.set("test".to_string(), "test".to_string());
    
        assert_eq!(index.collection(), "test");
    }

}