use std::fmt::Debug;
use hex;
use crate::{hashable::Hashable, prelude::*, utils::{u128_bytes, u32_bytes, u64_bytes}};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String
}   

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, payload: String
    ) -> Self {
        Block { 
            index,
            timestamp,
            hash: [0; 32],
            prev_block_hash,
            nonce: 0,
            payload,
         }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}",
            self.index,
            hex::encode(&self.hash),
            self.timestamp,
            self.payload,
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

        bytes
    }
}

#[cfg(test)]
mod test {
    use crate::utils::now;
    use super::*;

    #[test]
    fn create_block() {
        let mut  block = Block::new(0, now(), [0;32], "Genesis block!".to_owned());

        let hash = block.hash();
        block.hash = hash;

        dbg!(block);
    }
}