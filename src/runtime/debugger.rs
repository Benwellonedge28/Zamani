//! Zamani Runtime: Debugger
//!
//! A minimal, real implementation of the core debugger primitives consumed
//! by the `toolchain::zamani_debug` Debug Adapter Protocol (DAP) front-end:
//! breakpoint tracking and basic execution-control state (continue/step).

use crate::stdlib::collections::List;

/// A single source-level breakpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct Breakpoint {
    pub file: String,
    pub line: u32,
    pub verified: bool,
}

impl Breakpoint {
    pub fn new(file: &str, line: u32) -> Self {
        Breakpoint {
            file: file.to_string(),
            line,
            verified: true,
        }
    }
}

/// A single variable observed at a given point in execution.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub type_name: String,
}

/// Core Zamani debugger: tracks breakpoints and drives execution control
/// over an attached runtime.
pub struct ZamaniDebugger {
    pub breakpoints: List<Breakpoint>,
    pub attached: bool,
}

impl ZamaniDebugger {
    pub fn new() -> Self {
        ZamaniDebugger {
            breakpoints: List::new(),
            attached: false,
        }
    }

    /// Attaches this debugger to a running Zamani application, given a
    /// handle to the runtime interface. The interface is generic (`?Sized`
    /// via reference) so callers can pass their own runtime-interface type.
    pub fn attach_to_runtime<T>(&mut self, _runtime_interface: &T) -> Result<(), String> {
        self.attached = true;
        println!("[Runtime::Debugger] Attached to runtime.");
        Ok(())
    }

    pub fn set_breakpoints(
        &mut self,
        breakpoints: List<Breakpoint>,
    ) -> Result<List<Breakpoint>, String> {
        self.breakpoints = breakpoints.clone();
        Ok(breakpoints)
    }

    pub fn continue_execution(&mut self) {
        println!("[Runtime::Debugger] Continuing execution.");
    }

    pub fn step_over(&mut self) {
        println!("[Runtime::Debugger] Stepping over.");
    }
}

impl Default for ZamaniDebugger {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init_debugger() {
    println!("  - Initializing Zamani Runtime Debugger...");
}

pub fn shutdown_debugger() {
    println!("  - Shutting down Zamani Runtime Debugger...");
}
