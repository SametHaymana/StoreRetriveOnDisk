use Storage::{disk::Disk, StorageKV};

mod Storage;

fn main() {
    let  disk:StorageKV<String> = StorageKV::new(Box::new(Disk::new("testDir".to_string())));

    println!("{:?}", disk.retrive(&"key1".to_string()).unwrap());
}
