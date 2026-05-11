
//! Zenith Universal Meta-Compiler (UMC) Standard Library
//!
//! This module aggregates and manages all standard library components for Zenith.
//! It provides foundational services and high-level abstractions that are common
//! across different programming paradigms supported by Zenith.
//!
//! The standard library is structured into modules corresponding to functional
//! areas (e.g., core utilities, collections, specific paradigm APIs).

pub mod core;
pub mod collections; // For List, Map, etc.
pub mod quantum;
pub mod nano;
pub mod mts;
pub mod sankofa;

/// Initializes all standard library components.
pub fn initialize_stdlib() {
    println!("Initializing Zenith UMC Standard Library...");
    core::init_core_lib();
    collections::init_collections_lib(); // New module init
    quantum::init_quantum_lib();
    nano::init_nano_lib();
    mts::init_mts_lib();
    sankofa::init_sankofa_lib();
    println!("Zenith UMC Standard Library initialized.");
}

/// Shuts down all standard library components.
pub fn shutdown_stdlib() {
    println!("Shutting down Zenith UMC Standard Library...");
    sankofa::shutdown_sankofa_lib();
    mts::shutdown_mts_lib();
    nano::shutdown_nano_lib();
    quantum::shutdown_quantum_lib();
    collections::shutdown_collections_lib(); // New module shutdown
    core::shutdown_core_lib();
    println!("Zenith UMC Standard Library shut down.");
}
