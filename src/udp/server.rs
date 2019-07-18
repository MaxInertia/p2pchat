use std::net::UdpSocket;
use crate::core;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

impl Server {
    pub fn new(address: &str, port: u16) -> impl core::Server {
        Server {
            address: String::from(address),
            port,
        }
    }
}

const BUFFER_SIZE: usize = 16;

impl core::Server for Server {
    fn listen(&self) -> Vec<u8> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let recv_from = format!("{}:{}", self.address, self.port);

        //let socket =
        match UdpSocket::bind(recv_from) {
            Ok(s) => {
                //loop {
                /*let (number_of_bytes, _src_addr) = */
                match s.peek_from(&mut buffer) {
                    Ok((len, _)) => {
                        println!("Server received {} bytes!", len);
                        return Vec::from(&mut buffer[..len])
                        //data
                    },
                    Err(x) => {
                        println!("Server Error on receive: {:?}", x);
                        //x
                    },
                };
                //}
            },
            Err(e) => {
                println!("Server Bind Error: {:?}", e)
            }
        }

        return Vec::new()
    }
}

//use std::marker::Sync;
//use std::marker::Send;
//unsafe impl Sync for Server {}
//unsafe impl Send for Server {}
