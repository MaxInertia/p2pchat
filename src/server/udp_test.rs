use super::UdpServer;

#[test]
fn test_panic() {
  let mut server = UdpServer::new();
  server.run();
}
