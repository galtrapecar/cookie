use ring::digest::*;

type Address = u128;

struct Transaction {
    version: u8,
    sender: Address,
    reciever: Address
}
pub struct Block {
    version: u8,
    magic_number: u8,
    hash_prev_block: u128,
    hash_next_block: u128,
    time: u64,
    size: u64,
    height: u128,

    transactions: Vec<Transaction>
}

impl Block {
    pub fn hash() {
        let data = "hello world";
        let hasher = digest(&SHA256, data.as_bytes());
    }

    pub fn new() {
        
    }
}