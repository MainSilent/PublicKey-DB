use std::fs;

use crate::config::config;

pub fn find(value: &str) -> &[u8] {
    if value.len() != 64 {
        return "The public key must be 64 bytes long".as_bytes()
    }
    let path = config("storage");
    let last_index_file = format!("{}/last_index", path);
    let last_index = fs::read_to_string(&last_index_file).unwrap().parse::<u8>().unwrap();

    "1".as_bytes()
}