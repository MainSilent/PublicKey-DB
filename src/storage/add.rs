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

    let size_limit = 100; // size in MB
    let mut last_index: u8;
    let path = config("storage");
    let last_index_file = format!("{}/last_index", path);

    let files = glob(&format!("{}/*.pdb", path)).expect("Failed to read the pdb file");
    
    if files.count() == 0 {
        sort_keys(value, &format!("{}/1.pdb", path));
        last_index = 1;
    }
    else {
        last_index = fs::read_to_string(&last_index_file).unwrap().parse::<u8>().unwrap();
        let last_file = format!("{}/{}.pdb", path, last_index);
        let last_file_size = fs::metadata(&last_file).unwrap().len();

        // If exceeds the file size limit create a new file
        if last_file_size >= (size_limit * 1000000) {
            last_index += 1;
        }

        sort_keys(value, &format!("{}/{}.pdb", path, last_index));
    }

    fs::write(&last_index_file, last_index.to_string())
        .expect("Failed to write last index file");
    Ok("1".as_bytes())
}

fn sort_keys(value: &str, path: &str) {
    let mut new_keys = hex::decode(value).unwrap();

    // Sort the data
    if fs::metadata(path).is_ok() {
        let file = fs::read(path).unwrap();
        let mut keys: Vec<&[u8]> = file.chunks(32).collect();
        keys.push(&new_keys);
        keys.sort();
        
        println!("{:?}", keys);
        new_keys = [8];
    } else {
        add_to_file(&new_keys, path);
    }
}

fn add_to_file(new_keys: &[u8], path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open database file");

    file.write_all(&new_keys).expect("Failed to append to database");
}