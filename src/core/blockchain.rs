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

  pub fn try_add_block(&mut self, block: Block) {
    match self.blocks.last() {
      None => {
        print!("The blockchain does not have at least one block. Aborting.");
        return;
      },
      Some(latest_block) => {
        if self.is_block_valid(&block, latest_block) {
          self.blocks.push(block);

          println!("Block has been successfully added");
        } else {
          println!("Could not add block, check the data and try again.");
        }
      }
    }
  }

  pub fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
    if new_block.previous_hash != latest_block.hash {
      println!("Block with ID {} has an invalid previous hash", new_block.id);
      return false;
    }

    if !new_block.hash.starts_with("0000") {
      println!("Block with ID {} has an invalid nonce", new_block.id);
      return false;
    }

    if new_block.id != latest_block.id + 1 {
      println!("Block with ID {} has an invalid ID", new_block.id);
      return false;
    }

    if digest(format!("{}{}{}{}{}", new_block.id, &new_block.previous_hash, &new_block.data, new_block.timestamp, new_block.nonce)) != new_block.hash {
      println!("Block with ID {} has an invalid hash", new_block.id);
      return false;
    }

    true
  }
}
