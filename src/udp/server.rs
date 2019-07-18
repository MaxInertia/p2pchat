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

impl core::Server for Server {
    fn listen(&self) -> [u8; 10] {
        let mut buf: [u8; 10] = [0; 10];
        let recv_from = format!("{}:{}", self.address, self.port);

        //let socket =
        match UdpSocket::bind(recv_from) {//format!("{}:{}", self.address, self.port))
            Ok(s) => {
                //loop {
                /*let (number_of_bytes, _src_addr) = */
                match s.peek_from(&mut buf) {
                    Ok(data) => {
                        println!("Server Received: {:?}", data);
                        //data
                    },
                    Err(x) => {
                        println!("Server Error on receive: {:?}", x);
                        //x
                    },
                };
                //}
                //let val = &mut buf[..10];
            },
            Err(e) => {
                println!("Server Bind Error: {:?}", e)
            }
        }

        return buf
    }
}

//use std::marker::Sync;
//use std::marker::Send;
//unsafe impl Sync for Server {}
//unsafe impl Send for Server {}