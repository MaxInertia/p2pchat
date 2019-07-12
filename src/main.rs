mod app;
mod client;
mod server;

use client::UdpClient;
use server::UdpServer;

fn main() {
    let mut client = UdpClient::new();
    let mut server = UdpServer::new();
    app::run(&mut client, &mut server, false);
}
