use glob::glob;
use crate::config::config;

pub fn add(value: &str) -> &[u8] {
    let size_limit = 50; // size in MB
    let path = config("storage");

    let files = glob(&(path+"/*.pdb")).expect("Failed to read the pdb file");
    
    for entry in files {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }

    "1".as_bytes()
}