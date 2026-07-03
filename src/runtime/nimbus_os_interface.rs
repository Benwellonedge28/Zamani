//! Zenith UMC Runtime: Nimbus OS Interface
//!
//! This module provides the interface layer for the Zenith runtime to interact
//! with the underlying Nimbus Operating System microkernel. It re-exports
//! core Nimbus types and functions, acting as the bridge for Zenith programs.

use std::sync::{Arc, Mutex};
// Updated import to reflect changes in mod.rs for ThreadId, ThreadState, GlobalScheduler
use crate::ast::Identifier;
use crate::core_lang_primitives::{MemoryRegion, Size, TimeStamp}; // Use core primitives for types
use crate::runtime::mts::TimelineId; // Import TimelineId

// Re-export core Nimbus types for convenience for other runtime modules.
// (NimbusMicrokernel is used locally only, so it is imported without being
// re-exported here.)
use crate::nimbus_os::NimbusMicrokernel;
pub use crate::nimbus_os::{
    CapabilityToken, ChannelId, GlobalScheduler, NimbusContext, NimbusContextId,
    NimbusContextState, SandboxPolicy, ThreadId, ThreadState,
};

// Global conceptual Nimbus Microkernel instance.
static mut NIMBUS_MICROKERNEL_INSTANCE: Option<Arc<Mutex<NimbusMicrokernel>>> = None;

/// Initializes the Nimbus OS interface.
pub fn init_nimbus_os_interface() -> Arc<Mutex<NimbusMicrokernel>> {
    println!(
        "  - Initializing Nimbus OS Microkernel Interface (Secure Isolation, IPC, Capabilities)..."
    );
    let microkernel = Arc::new(Mutex::new(NimbusMicrokernel::new()));
    unsafe {
        NIMBUS_MICROKERNEL_INSTANCE = Some(Arc::clone(&microkernel));
    }
    println!("    -> Nimbus OS Microkernel Interface initialized.");
    microkernel
}

/// Shuts down the Nimbus OS interface.
pub fn shutdown_nimbus_os_interface() {
    println!("  - Shutting down Nimbus OS Microkernel Interface...");
    unsafe {
        NIMBUS_MICROKERNEL_INSTANCE = None;
    }
    // Conceptual: Terminate all running contexts, clean up resources.
}

/// Conceptual function to get a reference to the global Nimbus Microkernel.
pub fn get_nimbus_microkernel() -> Option<Arc<Mutex<NimbusMicrokernel>>> {
    unsafe { NIMBUS_MICROKERNEL_INSTANCE.as_ref().map(Arc::clone) }
}

// --- Wrapper functions for Nimbus System Calls, now interacting with the global microkernel instance ---

// Example: create_isolated_context_via_interface now needs SandboxPolicy
pub fn create_isolated_context_via_interface(
    blueprint_id: Identifier,
    sandbox_policy: SandboxPolicy,
) -> NimbusContextId {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        microkernel
            .create_context(blueprint_id.0, None, sandbox_policy)
            .unwrap_or(0)
    } else {
        println!("Error: Nimbus Microkernel not initialized.");
        0
    }
}

// New wrapper function for creating a thread
pub fn create_thread_via_interface(
    context_id: NimbusContextId,
    entry_point_fn_ptr: u64,
    stack_size: Size,
) -> Result<ThreadId, String> {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        microkernel.create_thread(context_id, entry_point_fn_ptr, stack_size)
    } else {
        Err("Nimbus Microkernel not initialized.".to_string())
    }
}

// New wrapper function for starting a thread
pub fn start_thread_via_interface(
    context_id: NimbusContextId,
    thread_id: ThreadId,
) -> Result<(), String> {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        microkernel.start_thread(context_id, thread_id)
    } else {
        Err("Nimbus Microkernel not initialized.".to_string())
    }
}

// New wrapper function for suspending a thread
pub fn suspend_thread_via_interface(
    context_id: NimbusContextId,
    thread_id: ThreadId,
) -> Result<(), String> {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        microkernel.suspend_thread(context_id, thread_id)
    } else {
        Err("Nimbus Microkernel not initialized.".to_string())
    }
}

// New wrapper function for terminating a thread
pub fn terminate_thread_via_interface(
    context_id: NimbusContextId,
    thread_id: ThreadId,
) -> Result<(), String> {
    if let Some(microkernel_arc) = get_nimbus_microkernel() {
        let mut microkernel = microkernel_arc.lock().unwrap();
        microkernel.terminate_thread(context_id, thread_id)
    } else {
        Err("Nimbus Microkernel not initialized.".to_string())
    }
}

// The NimbusSystemCall struct in core_lang_primitives will use `get_nimbus_microkernel()`
// to access these functionalities directly from the global instance.
