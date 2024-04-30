use crate::{block::{self, Block}, hashable::Hashable};
use crate::prelude::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> Result<()> {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
               return Err("Index Mismatch".into())
            } else if !block::chech_difficulty(&block.hash(), block.difficulty) {
                return Err("Difficulty Fail".into())
            } else if i != 0 {
                // Not genesis block
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    return Err("Time did not increase".into())
                } else if block.prev_block_hash != prev_block.hash {
                    return Err("Hash mismatch".into())
                }
            } else {    
                // Genesis block
                if block.prev_block_hash != [0; 32] {
                    return Err("Genesis block prev_block_hash invalid".into())
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{block, hashable::Hashable, utils::now};

    use super::*;

    const DIFFICULTY: u128 = 0x0000ffffffffffffffffffffffffffff;

    fn create_genesis_block () -> Block {
        let mut block = Block::new(0, now(), [0; 32], 0, "Genesis Block".to_owned(), DIFFICULTY);
        
        block.mine();

        block
    }

    fn create_blockchain_with_genesis_block () -> Blockchain {
        let block = create_genesis_block();

        Blockchain {
            blocks: vec![block],
        }
    }

    fn create_blockchain() -> Blockchain {
        let block = create_genesis_block();

        let mut last_hash = block.hash;

        let mut blockchain = Blockchain {
            blocks: vec![block],
        };

        for i in 1..=10 {
            let mut block = Block::new(i, now(), last_hash, 0, "Another Block".to_owned(), DIFFICULTY);

            block.mine();

            last_hash = block.hash;

            blockchain.blocks.push(block);
        }

        blockchain
    }

    #[test]
    fn verify_genesis_block() {
        let mut blockchain = create_blockchain_with_genesis_block();

        assert_eq!((), blockchain.verify().unwrap());
    }

    #[test]
    fn verify_index_of_block() {
        let mut blockchain = create_blockchain();

        blockchain.blocks[3].index = 4;

        let err = blockchain.verify().unwrap_err();

        assert_eq!(err.to_string(), "Index Mismatch");
    }

    #[test]
    fn verify_hash_of_block() {
        let mut blockchain = create_blockchain();

        blockchain.blocks[3].hash[17] += 1;

        let err = blockchain.verify().unwrap_err();

        assert_eq!(err.to_string(), "Hash mismatch");
    }

    #[test]
    fn verify_difficult_of_block() {
        let mut blockchain = create_blockchain();

        blockchain.blocks[3].payload = "Hello World".to_owned();

        let err = blockchain.verify().unwrap_err();

        assert_eq!(err.to_string(), "Difficulty Fail");
    }

    #[test]
    fn verify_prev_hash_of_block() {
        let mut blockchain = create_blockchain();

        blockchain.blocks[3].prev_block_hash[8] += 1;

        let err = blockchain.verify().unwrap_err();

        assert_eq!(err.to_string(), "Difficulty Fail");
    }

}