
//! Nimbus OS: Core Operating System Components
//!
//! This module aggregates and manages the core components of the Nimbus Operating System,
//! providing foundational services for Zenith's AGI capabilities. This includes process
//! management, secure execution, resource scheduling, and ethical governance.

pub mod process_manager; // Process and Task Management
pub mod security_kernel; // Secure Execution Environment (Sandboxing, Isolation)
pub mod resource_scheduler; // Optimal Resource Allocation (CPU, Memory, I/O)
pub mod evas;             // Ethical, Verifiable, Autonomous, Secure filter
pub mod mod_rs;           // Modular OS components
pub mod admin_interface;  // New: Zenith Administration Interface
pub mod nimbus_rpc; // Nimbus OS RPC for secure inter-process and inter-system communication

/// Initializes the entire Nimbus OS.
pub fn initialize_nimbus_os() {
    println!("Initializing Nimbus OS...");
    process_manager::init_process_manager();
    security_kernel::init_security_kernel();
    resource_scheduler::init_resource_scheduler();
    evas::init_evas();
    mod_rs::init_mod_rs();
    nimbus_rpc::init_nimbus_rpc(); // Initialize Nimbus RPC
    admin_interface::init_admin_interface(); // Initialize Admin Interface
    println!("Nimbus OS initialized.");
}

/// Shuts down the entire Nimbus OS.
pub fn shutdown_nimbus_os() {
    println!("Shutting down Nimbus OS...");
    admin_interface::shutdown_admin_interface(); // Shutdown Admin Interface
    nimbus_rpc::shutdown_nimbus_rpc(); // Shutdown Nimbus RPC
    mod_rs::shutdown_mod_rs();
    evas::shutdown_evas();
    resource_scheduler::shutdown_resource_scheduler();
    security_kernel::shutdown_security_kernel();
    process_manager::shutdown_process_manager();
    println!("Nimbus OS shut down.");
}
