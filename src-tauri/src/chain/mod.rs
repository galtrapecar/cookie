use ring::digest::*;

type Address = u128;

/**
 * Default values for Transaction should be
 * version: 1 (as of 2023)
 * sender: 0
 * reciever: 0
 */
struct Transaction {
    version: u8,
    sender: Address,
    reciever: Address
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction { 
            version: 1,
            sender: 0, 
            reciever: 0
        }
    }
}

/**
 * Default values for Block should be
 * version: 1 (as of 2023)
 * magic_number: 0xC0041E00
 * hash_prev_block: 0
 * time: 0
 * size: 0
 * height: 0
 * transactions: Vec::new()
 */
pub struct Block {
    version: u8,
    magic_number: u32,
    hash_prev_block: u128,
    hash: u128,
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

    pub fn get_hash(&self) -> u128 {
        self.hash
    }

    pub fn new() -> Block {
        Block { 
            version: 1u8, 
            magic_number: 0xC0041E00, 
            hash_prev_block: 0, 
            hash: 0,
            time: 0, 
            size: 0, 
            height: 0,
            transactions: Vec::new()
        }
    }
}