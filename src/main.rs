use core::{block::Block, blockchain::Blockchain};

mod core;

fn main() {
    let mut new_blockchain = Blockchain::new();

    new_blockchain.generate_genesis_block();

    println!("{:?}", new_blockchain);

    let new_block = Block::new(2, new_blockchain.blocks[0].hash.to_owned(), String::from("From the deep"));

    new_blockchain.try_add_block(new_block);

    println!("{:?}", new_blockchain);

    let new_block = Block::new(3, new_blockchain.blocks[1].hash.to_owned(), String::from("He who controls the spice"));

    new_blockchain.try_add_block(new_block);

    println!("{:?}", new_blockchain);

    let new_block = Block::new(4, new_blockchain.blocks[2].hash.to_owned(), String::from("Controls the universe"));

    new_blockchain.try_add_block(new_block);

    println!("{:?}", new_blockchain);

    new_blockchain.is_blockchain_valid(&new_blockchain.blocks);

    new_blockchain.blocks = new_blockchain.blockchain_selector(new_blockchain.blocks.to_owned(), new_blockchain.blocks.to_owned()).unwrap();
}
