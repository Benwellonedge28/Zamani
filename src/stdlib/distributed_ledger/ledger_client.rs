#![allow(unused_imports, dead_code, unused_variables)]
//! Zamani — distributed ledger client implementation

use std::collections::HashMap;

pub fn init_ledger_client() {}
pub fn shutdown_ledger_client() {}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
}

pub struct LedgerClient {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl LedgerClient {
    pub fn new() -> Self {
        let mut client = LedgerClient {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
        };
        client.create_genesis_block();
        client
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            previous_hash: "0".to_string(),
            hash: "genesis_hash_zamani".to_string(),
            transactions: Vec::new(),
            timestamp: 0,
        };
        self.chain.push(genesis_block);
    }

    pub fn submit_transaction(&mut self, tx: Transaction) -> Result<String, String> {
        let tx_id = tx.id.clone();
        self.pending_transactions.push(tx);
        Ok(tx_id)
    }

    pub fn mine_block(&mut self) -> Block {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block {
            index: prev_block.index + 1,
            previous_hash: prev_block.hash.clone(),
            hash: format!("block_hash_{}", prev_block.index + 1),
            transactions: std::mem::take(&mut self.pending_transactions),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.chain.push(new_block.clone());
        new_block
    }
}
