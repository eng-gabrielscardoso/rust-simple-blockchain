use core::blockchain::Blockchain;

mod core;

fn main() {
    let mut new_blockchain = Blockchain::new();

    new_blockchain.generate_genesis_block();

    println!("{:?}", new_blockchain);
}
