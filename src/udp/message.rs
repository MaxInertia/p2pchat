use crate::core;

//use std::fmt::Debug;

#[derive(Debug)]
pub struct Message {
    content: Vec<u8>
}

impl core::Message for Message {
    fn bytes(&self) -> &Vec<u8> {
        return &self.content
    }
}
