#[derive(Debug, Clone)]
pub struct Block {
  pub id: u64,
  pub nonce: u64,
  pub data: String,
  pub hash: String,
  pub previous_hash: String,
  pub timestamp: i64,
}
