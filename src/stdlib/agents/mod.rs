//! Zenith Universal Meta-Compiler (UMC) Standard Library: AGI Agents Module
//!
//! This module aggregates and manages all components related to the creation,
//! deployment, and orchestration of AGI agents within the Zenith ecosystem.

pub mod agent_communication;
pub mod agent_interface; // Generic Agent Interface
pub mod agent_lifecycle;
pub mod swarm_orchestration; // AGI Swarm Orchestration // Agent Lifecycle Management // Inter-Agent Communication Protocols

/// Initializes all AGI agents components.
pub fn init_agents_lib() {
    println!("Initializing Zenith AGI Agents Module...");
    agent_interface::init_agent_interface();
    agent_lifecycle::init_agent_lifecycle();
    agent_communication::init_agent_communication(); // Initialize Swarm Orchestration
    swarm_orchestration::init_swarm_orchestration();
    println!("Zenith AGI Agents Module initialized.");
}

/// Shuts down all AGI agents components.
pub fn shutdown_agents_lib() {
    println!("Shutting down Zenith AGI Agents Module...");
    swarm_orchestration::shutdown_swarm_orchestration(); // Shutdown Swarm Orchestration
    agent_communication::shutdown_agent_communication();
    agent_lifecycle::shutdown_agent_lifecycle();
    agent_interface::shutdown_agent_interface();
    println!("Zenith AGI Agents Module shut down.");
}
