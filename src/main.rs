mod udp;
mod core;

use crate::core::{Client, Server};
use std::thread;
//use std::sync::Mutex;
use std::time::Duration;

pub fn main() {
    let address: &str = "127.0.0.1"; // "localhost" also works
    let server_port: u16 = 42534;
    let client_port: u16 = server_port + 1;

    let server = udp::Server::new(address, server_port);
    let client = udp::Client::new(address, client_port, server_port);

    run(client, server)
}

pub fn run(client: impl Client, server: impl Server + std::marker::Send  + 'static) {
    // spawn server thread
    let join_handle = thread::spawn(move || {
        println!("Server thread started");
        let received = server.listen();
        println!("server.received: {:?}", received);
    });

    // send content via client
    let content: &[u8; 10] = b"hello pop!";
    //let msg = udp::Message{content};
    println!("Client sending data");
    let ok = client.send(content);
    println!("client.send({:?}) == {}", content, ok);

    // wait and join
    thread::sleep(Duration::from_secs(2));
    match join_handle.join() {
        Ok(_) => {},
        Err(_) => panic!()
    };
}