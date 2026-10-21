
#![forbid(unsafe_code)]

//! Zamani WebAssembly backend module.
//!
//! Path: `src/backend/wasm/mod.rs`
//!
//! # Responsibility
//!
//! Define the public module boundary for Zamani's WebAssembly backend and
//! expose its backend-facing emission API.
//!
//! # Ownership
//!
//! - `emitter`: public WAT and WebAssembly binary emission facade.
//! - `crate::compiler::wasm_backend`: canonical existing WebAssembly lowering
//!   and binary assembly implementation.
//! - `crate::compiler::wasm_cfg`: WebAssembly control-flow preparation and
//!   CFG validation.
//! - `crate::ir_gen`: existing input IR representation.
//! - `crate::compiler::linker`: existing linking implementation.
//!
//! This module does not implement a second IR, instruction selector, CFG,
//! memory allocator, linker, or runtime.
//!
//! # Integration contract
//!
//! The emitter consumes the existing `IrModule` and delegates lowering to
//! the established compiler backend. It returns generated artifacts to its
//! caller; persistence, deployment, and execution remain separate stages.
//!
//! Target capability negotiation must be completed before claiming that an
//! artifact is executable in a particular runtime.
//!
//! # Scalability
//!
//! This module imposes no additional universal limits on the number of
//! functions, instructions, locals, modules, or source declarations.
//! WebAssembly format constraints and operational resource exhaustion remain
//! explicit. This module does not promise unbounded physical memory.
//!
//! # Safety
//!
//! Safe Rust only. Unsafe code is forbidden throughout this module.

/// Backend-facing WebAssembly emission API.
pub mod emitter;

/// Stable public emission types.
pub use emitter::{
    WasmEmissionStage,
    WasmEmitter,
    WasmEmitterConfig,
    WasmEmitterError,
};
