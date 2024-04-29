use std::fmt::Debug;
use hex;
use crate::{hashable::Hashable, prelude::*, utils::{difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes}};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}   

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64, payload: String, difficulty: u128
    ) -> Self {
        Block { 
            index,
            timestamp,
            hash: [0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty
         }
    }

    pub fn mine (&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if chech_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
            self.index,
            hex::encode(&self.hash),
            self.timestamp,
            self.payload,
            self.nonce,
        )
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(u32_bytes(&self.index));
        bytes.extend(u128_bytes(&self.timestamp));
        bytes.extend(self.prev_block_hash);
        bytes.extend(u64_bytes(&self.nonce));
        bytes.ends_with(self.payload.as_bytes());
        bytes.extend(u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn chech_difficulty (hash: &BlockHash, difficulty: u128) ->bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

#[cfg(test)]
mod test {
    use crate::utils::now;
    use super::*;

    #[test]
    fn create_block() {
        let mut  block = Block::new(0, now(), [0;32], 0,"Genesis block!".to_owned(), 0x00ffffffffffffffffffffffffffffff);

        let hash = block.hash();
        block.hash = hash;

        dbg!(block);
    }

    #[test]
    fn mine_block() {
        let mut  block = Block::new(0, now(), [0;32], 0,"Genesis block!".to_owned(), 0x00ffffffffffffffffffffffffffffff);

        block.hash = block.hash();
       
        dbg!(&block);

        block.mine();

        dbg!(block);
    }
}