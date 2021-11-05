use glob::glob;
use crate::config::config;

pub fn add(value: &str) -> Result<&[u8], std::io::Error> {
    let path = config("storage");

    let files = glob(&(path.to_string()+"/*.pdb")).expect("Failed to read the pdb file");
    
    if files.count() == 0 {
        add_to_file(value, &format!("{}/1.pdb", path));
    }
    else {
        add_to_file(value, &format!("{}/1.pdb", path));
    }

    Ok("1".as_bytes())
}

fn add_to_file(value: &str, filepath: &str) {
    let size_limit = 50; // size in MB


}