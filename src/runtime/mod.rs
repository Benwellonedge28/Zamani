
//! Zenith Universal Meta-Compiler (UMC) Runtime System
//!
//! This module defines the conceptual runtime environment for executing Zenith programs.
//! It orchestrates the various specialized runtimes (classical, quantum, nano, MTS, Sankofa)
//! and provides the foundational services required for Zenith's multi-paradigm execution model.
//!
//! Key responsibilities include:
//! - **Runtime Initialization & Shutdown:** Managing the lifecycle of all integrated runtimes.
//! - **Resource Management:** Orchestrating CPU, memory, QPU, and nano-agent resources.
//! - **Inter-paradigm Communication:** Facilitating seamless interaction between classical,
//!   quantum, nano-agent, MTS, and Sankofa components.
//! - **Error & Exception Handling:** Propagating runtime errors and managing global panic states.
//! - **Global State Management:** Maintaining a consistent view of the overall system state.

pub mod core;
pub mod quantum;
pub mod nano;
pub mod mts;
pub mod sankofa;
pub mod nimbus_os_interface;
pub mod core_lang_primitives;
pub mod memory_manager;
pub mod distributed; // New module for distributed computing

use std::sync::{Arc, Mutex};
use crate::nimbus_os::mod_rs::NimbusMicrokernel;

// Conceptual global runtime state handles
pub static mut QUANTUM_PROCESSOR_HANDLE: Option<Arc<Mutex<quantum::QuantumProcessor>>> = None;
pub static mut NANO_ORCHESTRATOR_HANDLE: Option<Arc<Mutex<nano::NanoAgentOrchestrator>>> = None;
pub static mut MTS_ORCHESTRATOR_HANDLE: Option<Arc<Mutex<mts::MultiTimelineOrchestrator>>> = None;
pub static mut SANKOFA_RUNTIME_STATE_HANDLE: Option<Arc<Mutex<sankofa::SankofaRuntimeState>>> = None;
pub static mut NIMBUS_MICROKERNEL_HANDLE: Option<Arc<Mutex<NimbusMicrokernel>>> = None;
pub static mut MEMORY_MANAGER_HANDLE: Option<Arc<Mutex<memory_manager::MemoryManager>>> = None;
pub static mut DISTRIBUTED_ORCHESTRATOR_HANDLE: Option<Arc<Mutex<distributed::DistributedOrchestrator>>> = None; // New handle for distributed orchestrator

/// Initializes all integrated runtimes required for Zenith program execution.
pub fn init_runtime() {
    println!("Initializing Zenith UMC Runtime System...");
    
    // Initialize core language primitives (memory, concurrency, Nimbus syscalls)
    core_lang_primitives::init_core_lang_primitives();

    // Initialize specialized runtimes. Order matters for dependencies.
    unsafe {
        NIMBUS_MICROKERNEL_HANDLE = Some(nimbus_os_interface::init_nimbus_os_interface());
        MEMORY_MANAGER_HANDLE = Some(memory_manager::init_memory_manager()); 
        QUANTUM_PROCESSOR_HANDLE = Some(quantum::init_quantum_runtime()); 
        NANO_ORCHESTRATOR_HANDLE = Some(nano::init_nano_runtime());
        MTS_ORCHESTRATOR_HANDLE = Some(mts::init_mts_runtime());
        SANKOFA_RUNTIME_STATE_HANDLE = Some(sankofa::init_sankofa_runtime());
        DISTRIBUTED_ORCHESTRATOR_HANDLE = Some(distributed::init_distributed_runtime()); // Initialize distributed runtime
    }
    
    println!("Zenith UMC Runtime System initialized.");
}

/// Shuts down all integrated runtimes.
pub fn shutdown_runtime() {
    println!("Shutting down Zenith UMC Runtime System...");
    // Shutdown specialized runtimes in reverse order of initialization if dependencies exist
    distributed::shutdown_distributed_runtime(); // Shutdown distributed runtime
    sankofa::shutdown_sankofa_runtime();
    mts::shutdown_mts_runtime();
    nano::shutdown_nano_runtime();
    quantum::shutdown_quantum_runtime();
    memory_manager::shutdown_memory_manager(); 
    nimbus_os_interface::shutdown_nimbus_os_interface();
    
    // Shutdown core language primitives
    core_lang_primitives::shutdown_core_lang_primitives(); 

    println!("Zenith UMC Runtime System shut down.");
}
