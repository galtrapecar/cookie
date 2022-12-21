#![allow(unused_doc_comments)]

use crate::chain::Block;
use crate::database::Database;
pub struct Miner {

}

impl Miner {
    fn fetch_prev_block() -> Block {
        Block::new()
    }

    fn create_new_block() {
        
    }

    pub fn new() -> Miner {
        /**
         * Setup
         */
        let prev_block = Miner::fetch_prev_block();
        let hash_prev_block = prev_block.get_hash();


        Miner {}
    }
}