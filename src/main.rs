// #![allow(unused)]

use block::Block;
use transaction::Transaction;
use utils::now;

use crate::{blockchain::Blockchain, prelude::*};

mod prelude;
mod utils;
mod block;
mod hashable;
mod blockchain;
mod transaction;

fn main() {
    let difficulty: u128 = 0x0000ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), [0 ; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                }
            ]
        }
    ], difficulty);

    genesis_block.mine();

    let last_hash = genesis_block.hash;

    println!("Mined genesis block {:?}", &genesis_block);

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ]
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 36,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ]
        }
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    blockchain.update_with_block(block).expect("Failed to add block");

}
