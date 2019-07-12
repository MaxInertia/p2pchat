use crate::client::UdpClient;
use crate::server::UdpServer;

use std::{thread, time};

pub fn run(_client: &mut UdpClient, server: &mut UdpServer, break_out: bool) {
  server.run();
  if !break_out {
    loop {
      println!("running...",);
      thread::sleep(time::Duration::from_millis(1000));
    }
  }
}

#[cfg(test)]
mod tests {

  use super::run;
  use super::UdpClient;
  use super::UdpServer;

  #[test]
  fn test_server_runs() {
    let mut mock_client = UdpClient::new();
    let mut mock_server = UdpServer::new();
    run(&mut mock_client, &mut mock_server, true);
    assert_eq!(mock_server.is_running(), true);
  }
}
