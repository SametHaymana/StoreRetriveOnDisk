use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use serde::de::DeserializeOwned;
use serde::Serialize;



pub fn serialize_to_file<T:Serialize>(path: PathBuf, data: T) -> std::io::Result<()>{ 
    println!("Serializing to file: {:?}", path);
    let data = bincode::serialize(&data).unwrap();
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

pub fn deserialize_from_file<T: DeserializeOwned>(path: PathBuf) -> std::io::Result<T>{
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let data = bincode::deserialize_from(reader).unwrap();
    Ok(data)
}

