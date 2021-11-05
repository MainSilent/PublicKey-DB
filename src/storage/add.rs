use std::fs;
use std::u64;
use glob::glob;
use fs::OpenOptions;
use std::io::prelude::*;
use crate::config::config;

pub fn add(value: &str) -> Result<&[u8], std::io::Error> {
    let size_limit = 50; // size in MB
    let path = config("storage");

    let files = glob(&(path.to_string()+"/*.pdb")).expect("Failed to read the pdb file");
    
    if files.count() == 0 {
        add_to_file(value, &format!("{}/1.pdb", path));
    }
    else {
        
    }

    Ok("1".as_bytes())
}

fn add_to_file(value: &str, path: &str) {
    let number = u128::from_str_radix(value, 16).unwrap();
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open database file");

    file.write_all(&number.to_be_bytes()).expect("Failed to append to database");
}