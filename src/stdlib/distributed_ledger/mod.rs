//! Zenith Universal Meta-Compiler (UMC) Standard Library: Distributed Ledger Module
//!
//! This module aggregates and manages all distributed ledger and blockchain-related
//! components for Zenith, ensuring secure, transparent, and immutable records.

pub mod contract_engine;
pub mod data_provenance; // Data Provenance & Ethical AI
pub mod ledger_client; // Client for interacting with Zenith ledgers
pub mod transaction_manager; // Transaction creation, signing, and submission // Zenith Smart Contract execution environment

/// Initializes all distributed ledger components.
pub fn init_ledger() {
    println!("Initializing Zenith Distributed Ledger Module...");
    ledger_client::init_ledger_client();
    transaction_manager::init_transaction_manager();
    contract_engine::init_contract_engine(); // Initialize Data Provenance
    data_provenance::init_data_provenance();
    println!("Zenith Distributed Ledger Module initialized.");
}

/// Shuts down all distributed ledger components.
pub fn shutdown_ledger() {
    println!("Shutting down Zenith Distributed Ledger Module...");
    data_provenance::shutdown_data_provenance(); // Shutdown Data Provenance
    contract_engine::shutdown_contract_engine();
    transaction_manager::shutdown_transaction_manager();
    ledger_client::shutdown_ledger_client();
    println!("Zenith Distributed Ledger Module shut down.");
}
