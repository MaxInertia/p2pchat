use std::net::UdpSocket;

pub struct Server {
  pub address: String,
  pub port: u16,
  running: bool,
}

impl Server {
  pub fn new() -> Server {
    let server_addr = "127.0.0.1";
    Server {
      address: String::from(server_addr),
      port: 42069,
      running: false,
    }
  }

  pub fn is_running(&self) -> bool {
    self.running
  }

  pub fn run(&mut self) -> &mut [u8] {
    if self.is_running() {
      panic!();
    }
    self.running = true;
    let socket =
      UdpSocket::bind(format!("{}:{}", self.address, self.port)).expect("couldnt bind server addr");
    let buf: &mut [u8] = &mut [0; 10];
    loop {
      let (number_of_bytes, _src_addr) = match socket.peek_from(buf) {
        Ok(data) => data,
        Err(_) => continue,
      };
      let val = &mut buf[..number_of_bytes];
    }
  }
}
