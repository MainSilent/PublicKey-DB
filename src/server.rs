use std::fs;
use std::thread;
use std::io::{Write, BufRead, BufReader};
use std::os::unix::net::{UnixStream, UnixListener};

use crate::storage as Storage;
use crate::socket::{Request, Operation};

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
                    thread::spawn(move || Self::connect(&stream));
                },
                Err(e) => eprintln!("Client failed to connect: {}", e)
            }
        }
    }

    pub fn connect(mut stream: &UnixStream) {
        println!("Client connected");

        loop {
            let mut request = String::new();
            let mut buffer = BufReader::new(stream);
            buffer.read_line(&mut request).expect("Failed to read");
            
            if request == "" {
                println!("Connection closed");
                break
            }

            let request = match Request::new(&request) {
                Ok(request) => request,
                Err(e) => {
                    stream.write_all(e.as_bytes()).ok();
                    stream.write(b"\n").ok();
                    continue
                }
            };

            
            stream.write_all(match request.op {
                Operation::Add => Storage::add(&request.value),
                Operation::Find => Storage::find(&request.value)
            })
            .expect("Failed to send the result");

            stream.write(b"\n").ok();
        }
    }
}