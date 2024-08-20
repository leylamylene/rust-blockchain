use std::sync::{Arc , RwLock};
use crate::{Block, Transaction};
use std::env::current_dir;

use sled::{transaction, Db};
use data_encoding::HEXLOWER;




const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const BLOCKS_TREE: &str = "blocks";

pub struct Blockchain {
  tip_hash : Arc<RwLock<String>> , // Hash of the last block 
  db: Db,
}


impl Blockchain {
  pub fn create_blockchain(genesis_address :&str) ->Blockchain {
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_hash;
    if data.is_none() {
      let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
      let block = Block::generate_genesis_block(&coinbase_tx);
      Self::update_blocks_tree(&blocks_tree , &block);
      tip_hash = String::from(block.get_hash());

    }
    else {
      tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
    }
    Blockchain {
      tip_hash : Arc::new(RwLock::new(tip_hash)),
      db,
    }
  }


  pub fn new_blockchain() -> Blockchain {
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let tip_bytes = blocks_tree
       .get(TIP_BLOCK_HASH_KEY)
       .unwrap()
       .expect("No existing blockchain found. Create one first.");
    let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
    Blockchain {
      tip_hash: Arc::new(RwLock::new(tip_hash)),
      db,
    }
}


  pub fn get_db(&self) ->&Db {
    &self.db
  }

  pub fn get_tip_hash(&self) ->String {
    self.tip_hash.read().unwrap().clone()
  }


  pub fn set_tip_hash(&self , new_tip_hash:&str) {
    let mut tip_hash = self.tip_hash.write().unwrap();
    *tip_hash = String::from(new_tip_hash);
  }

  pub fn iterator(&self) -> BlockchainIterator {
    BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
  }



  pub fn mine_block(&self , transactions: &[Transaction]) -> Block {
    for transaction in transactions {
      if transaction.verify(self) == false {
        panic!("Error invalid transaction");
      }
    }
    let best_height = self.get_best_height();
    let block = Block::new_block(self.get_tip_hash(),transactions ,best_height +1);
    let block_hash = block.get_hash();
    let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
    self::update_blocks_tree(&blocks_tree,&block);
    self.set_tip_hash(block_hash);
    block

  }

}


pub struct BlockchainIterator {
  db: Db,
  current_hash : String,
}

impl BlockchainIterator  {
  pub fn new(tip_hash : String , db:Db)-> BlockchainIterator {
    BlockchainIterator{db, current_hash :tip_hash}
  }

  pub fn next(&mut self) -> Option<Block> {
    let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
    let data = blocks_tree
    .get(self.current_hash.clone())
    .unwrap();
    
    if data.is_none() {
      return None;
    }
    // deserialise
    let block = Block::deserialize(data.unwrap().to_vec().as_slice());
      self.current_hash = block.get_pre_block_hash().clone();
      return Some(block);
   
  }
}