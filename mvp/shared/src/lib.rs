extern crate borsh;

pub use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct MyStruct {
    x: u32,
    y: u32,
}

impl MyStruct {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
