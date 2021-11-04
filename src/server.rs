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
                Ok(mut stream) => {
                    let mut response = String::new();
                    let mut buffer = BufReader::new(&stream);
                    buffer.read_line(&mut response).expect("Failed to read");

                    loop {
                        match stream.write_all(b"&mut response\n") {
                            Ok(v) => v,
                            Err(_e) => {
                                eprintln!("Failed to write, client disconnected");
                                break
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to connect: {}", err);                    
                    break;
                }
            }
        }
    }
}