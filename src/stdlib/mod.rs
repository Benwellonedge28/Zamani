
//! Zenith Universal Meta-Compiler (UMC) Standard Library
//!
//! This module aggregates and manages all standard library components for Zenith.
//! It provides foundational services and high-level abstractions that are common
//! across different programming paradigms supported by Zenith.
//!
//! The standard library is structured into modules corresponding to functional
//! areas (e.g., core utilities, collections, specific paradigm APIs).

pub mod core;
pub mod collections;
pub mod quantum;
pub mod nano;
pub mod mts;
pub mod sankofa;
pub mod reflection;
pub mod ml;
pub mod net;
pub mod fs;
pub mod sync;
pub mod crypto;
pub mod serialize;
pub mod gui;
pub mod db;
pub mod time;
pub mod numeric;
pub mod web;
pub mod ai_reasoning;
pub mod nlp;
pub mod vision; // New module for Computer Vision

/// Initializes all standard library components.
pub fn initialize_stdlib() {
    println!("Initializing Zenith UMC Standard Library...");
    core::init_core_lib();
    collections::init_collections_lib();
    quantum::init_quantum_lib();
    nano::init_nano_lib();
    mts::init_mts_lib();
    sankofa::init_sankofa_lib();
    reflection::init_reflection_lib();
    ml::init_ml_lib();
    net::init_net_lib();
    fs::init_fs_lib();
    sync::init_sync_lib();
    crypto::init_crypto_lib();
    serialize::init_serialize_lib();
    gui::init_gui_lib();
    db::init_db_lib();
    time::init_time_lib();
    numeric::init_numeric_lib();
    web::init_web_lib();
    ai_reasoning::init_ai_reasoning_lib();
    nlp::init_nlp_lib();
    vision::init_vision_lib(); // Initialize Vision module
    println!("Zenith UMC Standard Library initialized.");
}

/// Shuts down all standard library components.

pub fn shutdown_stdlib() {
    println!("Shutting down Zenith UMC Standard Library...");
    vision::shutdown_vision_lib(); // Shutdown Vision module
    nlp::shutdown_nlp_lib(); 
    ai_reasoning::shutdown_ai_reasoning_lib(); 
    web::shutdown_web_lib(); 
    numeric::shutdown_numeric_lib(); 
    time::shutdown_time_lib(); 
    db::shutdown_db_lib(); 
    gui::shutdown_gui_lib(); 
    serialize::shutdown_serialize_lib(); 
    crypto::shutdown_crypto_lib(); 
    sync::shutdown_sync_lib(); 
    fs::shutdown_fs_lib(); 
    net::shutdown_net_lib(); 
    ml::shutdown_ml_lib(); 
    reflection::shutdown_reflection_lib(); 
    sankofa::shutdown_sankofa_lib();
    mts::shutdown_mts_lib();
    nano::shutdown_nano_lib();
    quantum::shutdown_quantum_lib();
    collections::shutdown_collections_lib();
    core::shutdown_core_lib();
    println!("Zenith UMC Standard Library shut down.");
}
