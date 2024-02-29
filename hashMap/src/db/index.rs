use std::{collections::HashMap, hash::Hash, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::utils::calculate_hash;


#[derive(Serialize,Deserialize, Clone,Debug)]
pub struct Index{
    map: HashMap<String, PathBuf>,
    collection: String,
}

impl Index{
    pub fn new(collection_name: Option<String>) -> Self{
        let name = collection_name.unwrap_or_else(|| "default".to_string());
        Index{
            map: HashMap::new(),
            collection: name,
        }
    }

    pub fn collection(&self) -> &String {
        &self.collection
    }

    pub fn get(&self, key: &String) -> String{
        super::io::deserialize_from_file(self.map.get(key).unwrap().clone()).unwrap()
    }

    pub fn set(&mut self, key: String, val: String) {
        let _ = super::io::serialize_to_file(
            format!(".data/{}/{}", self.collection, calculate_hash(&key.clone())).into(), val);
        
        self.map.insert(key.clone(), format!(".data/{}/{}", self.collection, calculate_hash(&key)).into());
    }
    
}

