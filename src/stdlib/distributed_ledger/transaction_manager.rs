#![allow(unused_imports, dead_code, unused_variables)]

//! Zamani Distributed Ledger — Transaction Manager

use std::collections::VecDeque;

/// Initialize transaction_manager
pub fn init_transaction_manager() {
    println!("[StdLib::Ledger] Initializing Transaction Manager...");
}

/// Shutdown transaction_manager
pub fn shutdown_transaction_manager() {
    println!("[StdLib::Ledger] Shutting down Transaction Manager...");
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

pub struct TransactionManager {
    pub pending_txs: VecDeque<Transaction>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            pending_txs: VecDeque::new(),
        }
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.pending_txs.push_back(tx);
    }

    pub fn process_next_transaction(&mut self) -> Option<Transaction> {
        self.pending_txs.pop_front()
    }
}
