use crate::core;

//use std::fmt::Debug;

#[derive(Debug)]
pub struct Message {
    content: [u8; 10]//&'static [u8]
}

impl core::Message for Message {
    fn bytes(&self) -> [u8; 10] {//&'static [u8] {
        return self.content
    }
}