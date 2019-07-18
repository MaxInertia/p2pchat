// Receiver of messages
pub trait Server {
    fn listen(&self) -> Vec<u8>;
}

// Sender of messages
pub trait Client {
    // send a message
    fn send(&self, content: &Vec<u8>) -> bool;//message: &Message) -> bool;
}

pub trait Message {
    fn bytes(&self) -> &Vec<u8>;
}
