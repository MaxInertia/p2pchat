use crate::core;

use std::net::UdpSocket;

//use std::fmt::Debug;

#[derive(Debug)]
pub struct Client{
    pub address: String,
    pub client_port: u16,
    pub server_port: u16,
}

impl Client {
    pub fn new(address: &str, client_port: u16, server_port: u16) -> impl core::Client {
        Client {
            address: String::from(address),
            client_port,
            server_port,
        }
    }
}

/// Implementation of trait client::Client for udp::Client
impl core::Client for Client {
    fn send(&self, content: &[u8; 10]/*&core::Message*/) -> bool {
        // TODO: Implement

        let dest_addr = format!("{}:{}", self.address, self.server_port); //"127.0.0.1:42336";
        println!("dest_addr = {}", dest_addr);
        let bind_addr = format!("{}:{}", self.address, self.client_port);
        println!("bind_addr = {}", bind_addr);

        //let socket =
        match UdpSocket::bind(bind_addr) { //format!("{}:{}", self.address, self.port)) {
            Ok(s) => {
                println!("Client Bind returned OK({:?})", s);
                match s.send_to(&content[..10], dest_addr) {
                    Ok(data) => {
                        println!("Client Send returned OK({:?})", data);
                        return true
                    },
                    Err(e) => {
                        println!("Client Send returned Err({:?})", e);
                        return false
                    }
                }
            },
            Err(e) => {
                println!("Client Send returned Err({:?})", e);
                return false
            }
        }
    }
}
