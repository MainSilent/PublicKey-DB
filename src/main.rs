use server::Server;

mod socket;
mod server;

fn main() {
    let socket_path = "/tmp/pubdb";
    let server = Server::new(socket_path.to_string());
    server.run();
}
