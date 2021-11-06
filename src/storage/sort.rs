use std::fs;
use glob::glob;
use std::io::Write;
use fs::OpenOptions;
use crate::config::config;

pub fn sort() -> &'static [u8] {
    let storage_path = config("storage");
    let files = glob(&format!("{}/*.pdb", storage_path)).expect("Failed to read the pdb file");

    if files.count() != 0 {
        let last_index_file = format!("{}/last_index", storage_path);
        let last_index = fs::read_to_string(&last_index_file).unwrap().parse::<u8>().unwrap() + 1;

        for i in 1..last_index {
            let path = format!("{}/{}.pdb", storage_path, i);

            if fs::metadata(path.to_owned()).is_ok() {
                let file = fs::read(path.to_owned()).unwrap();
                let mut keys: Vec<&[u8]> = file.chunks(32).collect();
                keys.sort();
                
                let new_keys = keys.iter()
                    .flat_map(|arr| arr.iter())
                    .cloned()
                    .collect::<Vec<u8>>();
        
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .expect("Failed to open database file");
            
                file.write_all(&new_keys).expect("Failed to append to database");
            }
        }
    }

    "1".as_bytes()
}