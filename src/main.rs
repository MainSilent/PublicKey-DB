use server::Server;

mod socket;
mod server;
mod storage;

fn main() {
    let socket_path = "/tmp/pubdb.sock";
    let server = Server::new(socket_path.to_string());
    server.run();
}
