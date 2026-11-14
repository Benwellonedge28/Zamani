
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Interoperability
//!
//! Path: `src/zur/interop/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides the public composition root for interoperability
//! between the Zamani Universal Runtime (ZUR) and external
//! computational environments.
//!
//! This module composes the existing interoperability
//! implementations without introducing competing ABI,
//! provider, execution, loading, linking, or transport systems.
//!
//! ## Module ownership
//!
//! The interoperability subsystem is divided into three
//! independently maintained modules.
//!
//! ### `abi`
//!
//! Adapts existing foreign ABI declarations into the canonical
//! ZUR runtime ABI.
//!
//! Source ABI ownership remains with:
//!
//!     crate::classical::interoperability::abi
//!
//! Canonical runtime ABI ownership remains with:
//!
//!     crate::zur::contracts::abi
//!
//! This module does not define another ABI representation,
//! determine physical type layouts, load libraries, or execute
//! foreign functions.
//!
//! ### `ffi`
//!
//! Defines the safe foreign-function invocation boundary.
//!
//! It provides typed foreign-call requests, values, responses,
//! authorization interfaces, and injected execution interfaces.
//!
//! Actual foreign invocation is delegated to an explicitly
//! supplied implementation.
//!
//! It does not expose raw pointers or implement unsafe
//! foreign-function calling sequences.
//!
//! ### `external`
//!
//! Provides interoperability with independently implemented
//! external execution providers.
//!
//! It delegates execution through the canonical ZUR provider
//! contracts and requires explicit authorization.
//!
//! It does not implement another provider registry, provider
//! discovery system, transport protocol, process manager,
//! dynamic-library loader, or distributed execution engine.
//!
//! ## Architectural boundaries
//!
//! Interoperability is a boundary mechanism, not a second
//! programming language, semantic analyzer, intermediate
//! representation, compiler, or runtime.
//!
//! The following authorities remain separate:
//!
//! - `frontend`: syntax, AST and semantic analysis.
//! - `classical::interoperability`: source-level foreign ABI declarations.
//! - `zur::contracts::abi`: canonical runtime ABI.
//! - `zur::contracts::provider`: execution-provider contracts.
//! - `zur::contracts::execution`: execution lifecycle contracts.
//! - `zur::security`: runtime security and authorization policy.
//! - `stdlib::context`: operation context and cancellation.
//! - `compiler`: compilation, lowering and artifact production.
//! - `backend`: code generation and target-specific artifacts.
//! - `distributed`: distributed execution and transport.
//! - `quantum`, `hybrid`, `hdl` and `ai`: their respective
//!   computational authorities.
//!
//! Interoperability must consume these authorities rather than
//! redefining them.
//!
//! ## Important distinctions
//!
//! ABI compatibility is not provider compatibility.
//!
//! Provider compatibility is not provider availability.
//!
//! Provider availability is not authorization.
//!
//! Authorization is not resource reservation.
//!
//! Resource reservation is not execution.
//!
//! Submission is not successful completion.
//!
//! A successful adapter operation must never be interpreted
//! as proof that an external computation has completed.
//!
//! ## Open-world extensibility
//!
//! Interoperability must support independently implemented
//! languages, ABIs, calling conventions, providers and
//! computational domains.
//!
//! This composition root deliberately contains no closed
//! computational-domain enumeration.
//!
//! Future integrations must use the established contracts
//! and must not require changes to this module merely because
//! a new language, provider or computational domain is added.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Foreign functions.
//! - ABI declarations.
//! - ABI parameters.
//! - Foreign types.
//! - Calling conventions.
//! - Providers.
//! - Interfaces.
//! - Operations.
//! - Sessions.
//! - Payload sizes.
//! - Computational domains.
//! - Execution environments.
//!
//! Actual limits are determined by available memory,
//! implementation capabilities, configured resource budgets,
//! provider contracts and physical platform constraints.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No raw-pointer ownership interfaces.
//! No global mutable interoperability state.
//! No implicit authorization.
//! No fabricated execution success.
//! No implicit ABI compatibility.
//!
//! ## Grammar integration
//!
//! The normative interoperability specification is:
//!
//!     grammar/spec/interoperability.md
//!
//! The grammar family is maintained under:
//!
//!     grammar/interoperability/
//!
//! Relevant grammar authorities include:
//!
//! - `ffi.g4`
//! - `abi.g4`
//! - `foreign-functions.g4`
//! - `foreign-types.g4`
//! - `calling-conventions.g4`
//! - `interoperability.g4`
//!
//! These grammars describe source-level declarations and
//! expressions. They do not directly invoke runtime services.
//!
//! The implementation pipeline remains:
//!
//!     Grammar
//!       -> Frontend
//!       -> AST
//!       -> Semantic validation
//!       -> Canonical interoperability contracts
//!       -> Compiler and artifact generation
//!       -> ABI/provider validation
//!       -> Authorized execution
//!
//! Grammar modules must not depend on this runtime module.
//!
//! ## Integration
//!
//! This module is consumed through:
//!
//!     crate::zur::interop
//!
//! Runtime ABI consumers use:
//!
//!     crate::zur::interop::abi
//!
//! Foreign-function consumers use:
//!
//!     crate::zur::interop::ffi
//!
//! External-provider consumers use:
//!
//!     crate::zur::interop::external
//!
//! The parent `zur` module must explicitly declare this module.
//!
//! ## Stability
//!
//! This file owns module composition and public module paths.
//!
//! It does not own the implementation details of its child
//! modules. Changes to ABI, FFI or external-provider behavior
//! belong in their respective files.
//!
//! Adding a new computational domain or external language
//! must not require changing this composition root unless
//! a genuinely new interoperability responsibility is added.

// ============================================================================
// ABI ADAPTATION
// ============================================================================

/// Foreign ABI adaptation into the canonical ZUR runtime ABI.
///
/// This module reuses the authoritative source ABI and runtime
/// ABI contracts. It does not define a competing ABI.
pub mod abi;

// ============================================================================
// FOREIGN-FUNCTION INTEROPERABILITY
// ============================================================================

/// Safe, explicitly delegated foreign-function invocation.
///
/// Actual foreign execution requires an injected executor
/// and explicit authorization.
pub mod ffi;

// ============================================================================
// EXTERNAL PROVIDER INTEROPERABILITY
// ============================================================================

/// Interoperability with external execution providers.
///
/// Execution is delegated through canonical ZUR provider
/// contracts and their declared invocation interfaces.
pub mod external;
