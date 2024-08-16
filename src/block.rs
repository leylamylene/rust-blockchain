pub struct Block {
  timestamp : i64, // time when the block was created
  pre_block_hash : String, //haash value of the previous block
  hash : String, // hash of the current block,
  transactions : Vec<Transaction>, // transactions included int the block,
  nonce : i64,
  height : usize, // the position of the current block within the blockchain
}


impl Block {
  pub fn new_block(pre_block_hash : String , transactions : &[Transaction] , height : usize) ->Block {
    let mut block = Block{
      timestamp : crate::current_timestamp(),
      pre_block_hash,
      hash : String::new(""),
      transactions : transactions.to_vec(),
      nonce :0,
      height,
    };
   

    let pow = ProofOfWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    block.nonce = nonce;
    block.hash = hash;
    return block;
  }
}