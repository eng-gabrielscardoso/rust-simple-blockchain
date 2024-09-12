use chrono::Utc;
use sha256::digest;

use super::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
  pub blocks: Vec<Block>
}

impl Blockchain {
  pub fn new() -> Self {
    Self {
      blocks: vec![],
    }
  }

  pub fn generate_genesis_block(&mut self) {
    let genesis_block = Block {
      id: 1,
      data: String::from("The first block in the chain."),
      nonce: 1,
      hash: digest("To you from dystopia"),
      previous_hash: String::default(),
      timestamp: Utc::now().timestamp(),
    };

    self.blocks.push(genesis_block);
  }
}
