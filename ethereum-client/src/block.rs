// use chrono;
use crypto::{digest::Digest, sha2::Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use eyre::Result;
use log::info;

const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: u64,
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Result<Block> {
        let block = Block::new_block(String::from("Genesis block"), String::new(), 0).unwrap();
        Ok(block)
    }

    pub fn new_block(
        data: String,
        prev_block_hash: String,
        height: usize
    ) -> Result<Block> {
        let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(); // SystemTime::now()
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the block...");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input_str(&data);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<String> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
        );
        let bytes = bincode::serialize(&content)?;
        let data_string = String::from_utf8(bytes)?;
        Ok(data_string)
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input_str(&data);
        let mut vec1 = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        //println!("{:?}", vec1);
        Ok(hasher.result_str() == String::from_utf8(vec1)?)
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block().unwrap()]
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("First block".to_string()).unwrap();
        blockchain.add_block("Second block".to_string()).unwrap();
        assert_eq!(blockchain.blocks.len(), 3);
    }
}