use hex;
use std::fs;
use glob::glob;
use fs::OpenOptions;
use std::io::prelude::*;
use crate::config::config;

pub fn add(value: &str) -> Result<&[u8], std::io::Error> {
    if value.len() != 64 {
        return Ok("The public key must be 64 bytes long".as_bytes())
    }

    let mut last_index: u8;
    let size_limit = 50; // size in MB
    let path = config("storage");

    let files = glob(&format!("{}/*.pdb", path)).expect("Failed to read the pdb file");
    
    if files.count() == 0 {
        add_to_file(value, &format!("{}/1.pdb", path));
        last_index = 1;
    }
    else {
        last_index = 1;
    }

    fs::write(&format!("{}/last_index", path), last_index.to_string())
        .expect("Failed to write last index file");
    Ok("1".as_bytes())
}

fn add_to_file(value: &str, path: &str) {
    let number = hex::decode(value).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open database file");

    file.write_all(&number).expect("Failed to append to database");
}