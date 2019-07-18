// Receiver of messages
pub trait Server {
    fn listen(&self) -> [u8; 10];
}

// Sender of messages
pub trait Client {
    // send a message
    fn send(&self, content: &[u8; 10]) -> bool;//message: &Message) -> bool;
}

pub trait Message {
    fn bytes(&self) -> [u8; 10];
}
