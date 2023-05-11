mod handler;
mod router;
mod server;

use server::Server;

fn main() {
    let socket_addr = "localhost:5000";
    let server = Server::new(socket_addr);
    server.run();
}
