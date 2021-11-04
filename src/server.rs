use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::os::unix::net::UnixListener;

use crate::socket::Request;

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
                Ok(mut stream) => {
                    let mut response = String::new();
                    let mut buffer = BufReader::new(&stream);
                    buffer.read_line(&mut response).expect("Failed to read");

                    let request = Request::new("dsf".to_string());
                }
                Err(err) => {
                    println!("Failed to connect: {}", err);                    
                    break;
                }
            }
        }
    }
}