use std::io::BufRead;
use std::fs;
use std::io::{Write, BufReader};
use std::os::unix::net::UnixListener;

pub struct Server {
    path: String
}

impl Server {
    pub fn new(path: String) -> Self {
        fs::remove_file(&path).ok();

        Self {
            path
        }
    }

    pub fn run(self) {
        let listener = UnixListener::bind(self.path).expect("Failed to bind");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut response = String::new();
                    let mut stream = BufReader::new(stream);
                    stream.read_line(&mut response).unwrap();

                    println!("{}", response);
                }
                Err(err) => {
                    println!("Failed to connect: {}", err);                    
                    break;
                }
            }
        }
    }
}