use std::collections::HashSet;

use crate::{block::{self, Block}, hashable::Hashable};
use crate::prelude::*;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspend_outputs: HashSet<Hash>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspend_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block (& mut self, block : Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if i == 0 {
            // Genesis Block
            if block.prev_block_hash != [0; 32] && !block::chech_difficulty(&block.hash(), block.difficulty) {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat)
            }
        } else if i != block.index as usize{
            return Err(
                BlockValidationErr::MismatchedIndex
            );
        } 
        else {
            let prev_block = &self.blocks[i-1];

            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp)
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            } else if !block::chech_difficulty(&block.hash(), block.difficulty) {
                return Err(BlockValidationErr::InvalidHash);
            }
        }

        

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<[u8; 32]> = HashSet::new();
            let mut block_created: HashSet<[u8; 32]> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspend_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty(){
                        return Err(BlockValidationErr::InvalidInput);
                    }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();    
                
                if output_value > input_value {
                    return  Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());

            }

            if coinbase.output_value() < total_fee {
                return Err(
                    BlockValidationErr::InvalidCoinbaseTransaction
                );
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspend_outputs.retain(|output| !block_spent.contains(output));
            self.unspend_outputs.extend(block_created);

        }
        
        self.blocks.push(block);

        Ok(())
    }
}