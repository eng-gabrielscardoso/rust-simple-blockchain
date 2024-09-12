use core::{block::Block, blockchain::Blockchain};

mod core;

fn main() {
    let mut new_blockchain = Blockchain::new();

    new_blockchain.generate_genesis_block();

    println!("{:?}", new_blockchain);

    let new_block = Block::new(2, new_blockchain.blocks[0].hash.to_owned(), String::from("All the governments have fallen"));

    new_blockchain.try_add_block(new_block);

    println!("{:?}", new_blockchain);
}
