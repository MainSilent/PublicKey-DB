use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn add(value: &str) -> &[u8] {
    let path = "/home/silent/projects/data.pdb";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open database file");

    file.write_all(value.as_bytes()).expect("Failed to append to database");

    "1".as_bytes()
}