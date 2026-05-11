
//! Zenith UMC Runtime: Nimbus OS Interface
//!
//! This module provides the interface layer for the Zenith runtime to interact
//! with the underlying Nimbus Operating System microkernel. It re-exports
//! core Nimbus types and functions, acting as the bridge for Zenith programs.

use std::sync::{Arc, Mutex};
use crate::nimbus_os::mod_rs::{NimbusMicrokernel, NimbusContextId, SandboxPolicy, CapabilityToken, ChannelId, NimbusContext, NimbusContextState}; // Import from new location
use crate::ast::Identifier;
use crate::core_lang_primitives::{Size, MemoryRegion, TimeStamp}; // Use core primitives for types
use crate::runtime::mts::TimelineId; // Import TimelineId

// Re-export core Nimbus types for convenience for other runtime modules
pub use crate::nimbus_os::mod_rs::{NimbusContext, NimbusContextState, CapabilityToken, ChannelId, SandboxPolicy, NimbusContextId};

// Global conceptual Nimbus Microkernel instance.
static mut NIMBUS_MICROKERNEL_INSTANCE: Option<Arc<Mutex<NimbusMicrokernel>>> = None;

/// Initializes the Nimbus OS interface.
pub fn init_nimbus_os_interface() -> Arc<Mutex<NimbusMicrokernel>> {
    println!("  - Initializing Nimbus OS Microkernel Interface (Secure Isolation, IPC, Capabilities)...");
    let microkernel = Arc::new(Mutex::new(NimbusMicrokernel::new()));
    unsafe { NIMBUS_MICROKERNEL_INSTANCE = Some(Arc::clone(&microkernel)); }
    println!("    -> Nimbus OS Microkernel Interface initialized.");
    microkernel
}

/// Shuts down the Nimbus OS interface.
pub fn shutdown_nimbus_os_interface() {
    println!("  - Shutting down Nimbus OS Microkernel Interface...");
    unsafe { NIMBUS_MICROKERNEL_INSTANCE = None; }
    // Conceptual: Terminate all running contexts, clean up resources.
}

/// Conceptual function to get a reference to the global Nimbus Microkernel.
pub fn get_nimbus_microkernel() -> Option<Arc<Mutex<NimbusMicrokernel>>> {
    unsafe { NIMBUS_MICROKERNEL_INSTANCE.as_ref().map(Arc::clone) }
}

// --- Wrapper functions for Nimbus System Calls, now interacting with the global microkernel instance ---
// These would typically be part of `core_lang_primitives::NimbusSystemCall` or directly exposed by the runtime.
// For this conceptual refactoring, we'll keep them here for clarity in the interface layer.

// Example of how a "system call" might be invoked by Zenith runtime components:
pub fn create_isolated_context_via_interface(blueprint_id: Identifier, sandbox_policy: SandboxPolicy) -> NimbusContextId {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        // Assume default parent_id for now
        microkernel.create_context(blueprint_id.0, None, sandbox_policy).unwrap_or(0)
    } else {
        println!("Error: Nimbus Microkernel not initialized.");
        0
    }
}

// The NimbusSystemCall struct in core_lang_primitives will use `get_nimbus_microkernel()`
// to access these functionalities directly from the global instance.
