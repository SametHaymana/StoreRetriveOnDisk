use self::utils::Storable;

pub mod disk;
pub mod mem;
pub mod utils;



pub struct  StorageKV<T>{
    strg: Box<dyn Storable<T>>
}


impl<T> StorageKV<T> {
    pub fn new(strg: Box<dyn Storable<T>>    ) -> Self {
        StorageKV { strg}
    }

    pub fn write(&mut self, key: String, value: T) -> Result<(), std::io::Error> {
        self.strg.write(key, value)
    }

    pub fn retrive(&self, key: &String) -> Result<T, std::io::Error> {
        self.strg.retrive(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.strg.keys()
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use self::disk::Disk;
    use self::mem::Mem;
    use self::utils::Storable;

    #[test]
    fn test_mem(){
        let mut mem = StorageKV::new(Box::new(Mem::<String>::default()));
        mem.write("key1".to_string(), "value1".to_string()).unwrap();
        mem.write("key2".to_string(), "value2".to_string()).unwrap();
        assert_eq!(mem.retrive(&"key1".to_string()).unwrap(), "value1".to_string());
        assert_eq!(mem.retrive(&"key2".to_string()).unwrap(), "value2".to_string());
        assert_eq!(mem.keys(), vec!["key1".to_string(), "key2".to_string()]);
    }

    #[test]
    fn test_disk(){
        let mut disk = StorageKV::new(Box::new(Disk::new("testDir".to_string())));
        disk.write("key1".to_string(), "value1".to_string()).unwrap();
        disk.write("key2".to_string(), "value2".to_string()).unwrap();
        assert_eq!(disk.retrive(&"key1".to_string()).unwrap(), "value1".to_string());
        assert_eq!(disk.retrive(&"key2".to_string()).unwrap(), "value2".to_string());
        assert_eq!(disk.keys(), vec!["key1".to_string(), "key2".to_string()]);
    }

}