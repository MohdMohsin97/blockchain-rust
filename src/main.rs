#![allow(unused)]

use crate::{block::Block, hashable::Hashable, prelude::*, utils::now};

mod error;
mod prelude;
mod utils;
mod block;
mod hashable;

fn main() -> Result<()>{
    let mut  block = Block::new(0, now(), [0;32], 0,"Genesis block!".to_owned(), 0x00ffffffffffffffffffffffffffffff);

        block.hash = block.hash();
       
        block.mine();

        dbg!(block);

    Ok(())
}
