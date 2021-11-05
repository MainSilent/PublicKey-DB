use server::Server;
use config::config;

mod config;
mod socket;
mod server;
mod storage;

fn main() {
    let socket_path = config("socket");
    let server = Server::new(socket_path);
    server.run();
}
