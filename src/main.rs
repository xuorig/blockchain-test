pub mod blockchain;
pub mod byte_helpers;

use blockchain::{Block, Blockchain, Hashable};

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, 0, vec![0; 32], 0, difficulty, "Genesis block".into());
    block.mine();
    println!("Mined: {:?}", &block);

    let mut last_hash = block.hash().clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, difficulty, "Genesis block".into());
        block.mine();
        println!("Mined: {:?}", &block);

        last_hash = block.hash().clone();
        blockchain.blocks.push(block);
    }
}
