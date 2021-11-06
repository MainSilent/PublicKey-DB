use std::fs;
use glob::glob;
use crate::config::config;

pub fn find(value: &str) -> &[u8] {
    if value.len() != 64 {
        return "The public key must be 64 bytes long".as_bytes()
    }
    let storage_path = config("storage");
    let last_index_file = format!("{}/last_index", storage_path);
    let last_index = fs::read_to_string(&last_index_file).unwrap().parse::<u8>().unwrap();
    let files = glob(&format!("{}/*.pdb", storage_path)).expect("Failed to read the pdb file");

    if files.count() == 0 {
        return "0".as_bytes()
    }

    for i in 1..last_index {
        let path = format!("{}/{}.pdb", storage_path, i);
    }

    "1".as_bytes()
}