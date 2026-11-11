#![forbid(unsafe_code)]

//! # Zamani Standard Library — Computational Domains
//!
//! Path: `src/stdlib/domains/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public aggregation boundary for Zamani's standard-library
//! computational-domain facades.
//!
//! A domain facade exposes the appropriate canonical repository subsystem
//! through a stable standard-library namespace. It does not create a
//! competing implementation of that subsystem.
//!
//! The standard library supports the Zamani portability objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! A program expresses computational intent independently of the physical
//! platform that eventually realizes it. The compiler, semantic authorities,
//! execution planner, runtime, HAL, and providers determine how that intent
//! can be realized using available capabilities and resources.
//!
//! This module introduces no hardware-specific assumptions or universal
//! resource ceilings.
//!
//! ## Public domain namespaces
//!
//! The following modules expose the existing domain facades:
//!
//! - [`ai`] — Artificial intelligence and AI-oriented computation.
//! - [`classical`] — Classical computation.
//! - [`distributed`] — Distributed computation.
//! - [`embedded`] — Embedded and resource-constrained computation.
//! - [`future`] — Integration points for future computational domains.
//! - [`hdl`] — Hardware-description functionality.
//! - [`hybrid`] — Hybrid and heterogeneous computation.
//! - [`quantum`] — Quantum computation.
//! - [`robotics`] — Robotics and robotic-control computation.
//!
//! These are public API namespaces, not a closed enumeration of all
//! computational domains that Zamani can support.
//!
//! New domains may be introduced through the appropriate grammar,
//! frontend, semantic, IR, compiler, runtime, and provider authorities.
//! A new standard-library facade can then be exposed here when one is
//! appropriate.
//!
//! ## Architectural ownership
//!
//! This module owns only:
//!
//! - The standard-library computational-domain namespace.
//! - Public declarations of domain facade modules.
//! - The organization of standard-library domain imports.
//! - The safe-Rust boundary for this module and its descendants.
//! - Documentation of domain-facade responsibilities.
//!
//! This module does not own:
//!
//! - Source grammar, tokens, parsing, or AST construction.
//! - Language-level type checking or semantic validation.
//! - Domain-specific semantic authorities.
//! - Capability or resource requirement definitions.
//! - Requirement, Preference, Constraint, or Hint semantics.
//! - Feasibility assessment.
//! - Classical, quantum, HDL, AI, or distributed IR.
//! - Tensor storage or foundational collections.
//! - Compiler transformations or lowering.
//! - Backend selection or code generation.
//! - Physical hardware discovery or allocation.
//! - Device drivers or hardware access.
//! - Provider registration or provider selection.
//! - Execution scheduling or routing.
//! - Runtime initialization or shutdown.
//! - Operating-system services.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ## Canonical integration authorities
//!
//! The domain facades integrate with the following repository subsystems
//! where applicable:
//!
//! - `crate::frontend` — source-language frontend and AST.
//! - `crate::frontend::semantic` — semantic validation and domain meaning.
//! - `crate::classical` — classical computation and canonical classical IR.
//! - `crate::quantum` — quantum computation and quantum IR.
//! - `crate::hdl` — hardware-description functionality and HDLIR.
//! - `crate::ai` — AI computation and AI integration.
//! - `crate::hybrid` — hybrid computation.
//! - `crate::distributed` — distributed computation.
//! - `crate::zuir` — universal intermediate representation.
//! - `crate::compiler` — compilation and lowering.
//! - `crate::backend` — backend implementations.
//! - `crate::runtime` — execution and resource management.
//! - HAL and platform providers — actual hardware and platform access.
//! - `crate::stdlib::provider` — common standard-library provider contracts.
//! - `crate::stdlib::capability` — standard-library capability descriptions.
//! - `crate::stdlib::context` — operation context.
//! - `crate::stdlib::error` — standard-library error contracts.
//!
//! A facade must use the relevant canonical authority rather than introduce
//! a second implementation of an existing concept.
//!
//! ## Domain boundaries
//!
//! A domain facade may expose portable APIs and deliberately re-export
//! canonical domain functionality.
//!
//! A domain facade must not independently redefine:
//!
//! - AST nodes or source-language semantics.
//! - Domain identities already owned by semantic authorities.
//! - Canonical intermediate representations.
//! - Generic provider or capability registries.
//! - Tensor or collection storage.
//! - Runtime execution engines.
//! - Hardware allocation or device selection.
//! - Scheduling, placement, or routing.
//!
//! Domain-to-domain composition must occur through the established
//! semantic and canonical IR boundaries, not through dependencies on
//! another facade's private implementation.
//!
//! ## Capability, resource, and policy separation
//!
//! Domain facades must preserve the distinction between:
//!
//! - **Requirement:** what an operation must have to be validly realized.
//! - **Capability:** what an environment or provider can actually supply.
//! - **Preference:** which valid realization is desirable.
//! - **Constraint:** what a valid realization must obey.
//! - **Hint:** non-binding implementation guidance.
//! - **Feasibility:** whether a requested realization is supported by
//!   available evidence and resources.
//!
//! A domain facade must not turn a preference or hint into a mandatory
//! requirement, or claim feasibility merely because an API exists.
//!
//! The semantic layer owns source-language meaning. The compiler and
//! runtime evaluate realization feasibility using authoritative evidence.
//!
//! ## POCO-REAF and scalability
//!
//! This module imposes no universal maximum on:
//!
//! - Computational domains.
//! - Domain operations.
//! - Functions or modules.
//! - Classical processors or threads.
//! - Accelerators or accelerator instances.
//! - Quantum processors or qubits.
//! - HDL signals or hardware resources.
//! - Embedded devices.
//! - Distributed nodes or participants.
//! - Tensor ranks or dimensions.
//! - Memory or storage capacity.
//! - Network participants.
//!
//! This module contains no fixed hardware inventory, vendor list,
//! physical topology, device count, or execution-target assumption.
//!
//! Actual execution remains subject to:
//!
//! - Program semantics.
//! - Representation and implementation limits.
//! - Explicit policies and operational budgets.
//! - Available provider capabilities.
//! - Available memory, processing, storage, and other resources.
//! - Target and runtime support.
//!
//! Operational budgets are configurable execution policies, not
//! universal language-level capability ceilings.
//!
//! No finite implementation promises literal infinite computation.
//! The portability objective is to avoid arbitrary semantic and
//! hardware-capacity restrictions so that programs can scale as
//! implementation support and available resources permit.
//!
//! ## Initialization and lifecycle
//!
//! This module performs no initialization and has no shutdown procedure.
//!
//! Declaring a domain module must not:
//!
//! - Initialize a runtime.
//! - Discover hardware.
//! - Register providers through global side effects.
//! - Allocate physical resources.
//! - Start threads or executors.
//! - Establish network connections.
//! - Initialize every computational domain.
//!
//! Domain-specific lifecycle operations, where required, belong to
//! explicit lifecycle and runtime authorities.
//!
//! A program using classical functionality must not be forced to initialize
//! quantum, robotics, distributed, or other unrelated subsystems.
//!
//! ## Compatibility
//!
//! Existing domain facades retain their own public APIs and compatibility
//! responsibilities.
//!
//! This module deliberately does not glob-re-export their contents.
//! Consumers use explicit paths such as:
//!
//! - `crate::stdlib::domains::classical`
//! - `crate::stdlib::domains::quantum`
//! - `crate::stdlib::domains::embedded`
//! - `crate::stdlib::domains::distributed`
//!
//! This prevents unrelated domain APIs from silently colliding in a
//! shared namespace.
//!
//! Existing legacy paths under `crate::stdlib` remain separate until
//! their migration and compatibility contracts are completed.
//!
//! ## Testing
//!
//! This aggregation module has no independent runtime behavior to test.
//! Its compilation verifies that the declared domain modules are present
//! and can be included by the standard-library crate.
//!
//! Each domain facade remains responsible for its own unit tests,
//! integration tests, API compatibility tests, and domain-specific
//! conformance tests.

// -----------------------------------------------------------------------------
// Domain facade declarations
// -----------------------------------------------------------------------------
//
// Keep this section declarative.
// Do not add runtime initialization, provider discovery, hardware selection,
// resource allocation, or domain-specific implementation here.

/// Artificial intelligence and AI-oriented computation.
pub mod ai;

/// Classical computation.
pub mod classical;

/// Distributed computation.
pub mod distributed;

/// Embedded and resource-constrained computation.
pub mod embedded;

/// Extensibility for computational domains not yet represented by a
/// dedicated standard-library facade.
pub mod future;

/// Hardware-description functionality.
pub mod hdl;

/// Hybrid and heterogeneous computation.
pub mod hybrid;

/// Quantum computation.
pub mod quantum;

/// Robotics and robotic-control computation.
pub mod robotics;