
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Distributed Resilience
//!
//! Path: `src/distributed/resilience/mod.rs`
//!
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! Provides the public composition boundary for distributed resilience
//! in the Zamani programming language.
//!
//! Distributed resilience coordinates the preservation and restoration
//! of valid distributed computation when execution environments,
//! resources, capabilities, participants, communication paths, or
//! execution providers experience failures or changes.
//!
//! It supports the POCO-REAF principle:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The same logical computation must remain independent of the
//! particular machines, processors, accelerators, networks, and
//! execution providers used to realize it.
//!
//! ## Architectural ownership
//!
//! This module composes resilience functionality.
//!
//! It does not implement:
//!
//! - distributed semantic analysis;
//! - AST or canonical IR representations;
//! - task partitioning;
//! - placement;
//! - scheduling;
//! - physical routing;
//! - network transport;
//! - backend selection;
//! - physical hardware allocation;
//! - quantum error correction;
//! - quantum fault/noise modeling;
//! - checkpoint storage engines;
//! - execution-provider implementations;
//! - authentication or authorization implementations.
//!
//! Those responsibilities remain with their existing repository
//! authorities.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Lexer / Parser / AST
//!      |
//!      v
//! Semantic validation
//!      |
//!      v
//! Canonical semantic representation
//!      |
//!      v
//! Canonical and domain IR
//!      |
//!      v
//! Distributed planning
//!      |
//!      v
//! Placement / Routing / Scheduling
//!      |
//!      v
//! Distributed execution
//!      |
//!      v
//! Distributed resilience
//!      |
//!      +--> Retry evaluation
//!      +--> Checkpoint coordination
//!      +--> Execution reconciliation
//!      +--> Recovery orchestration
//!      +--> Idempotency coordination
//!      +--> Recovery verification
//!      |
//!      v
//! Runtime / HAL / Domain providers
//! ```
//!
//! Resilience may request replanning, rescheduling, remapping,
//! recompilation, or provider adaptation.
//!
//! It must not independently implement those operations.
//!
//! ## Fundamental invariants
//!
//! 1. Recovery must preserve the original program's required semantics.
//!
//! 2. An operational failure must not automatically authorize retry.
//!
//! 3. Unknown execution outcomes must be reconciled before resubmission.
//!
//! 4. Partially executed operations must not be blindly repeated.
//!
//! 5. Recovery must respect cancellation, deadlines, authorization,
//!    resource budgets, and capability requirements.
//!
//! 6. A successful recovery operation must not be confused with
//!    successful execution or verified computation results.
//!
//! 7. Checkpointing must be explicitly supported by the relevant
//!    domain/runtime provider.
//!
//! 8. Quantum state must never be assumed to be freely serializable,
//!    transferable, or reconstructible.
//!
//! 9. Logical identities must remain stable across physical migration
//!    and execution-provider changes.
//!
//! 10. An operation must not be considered successfully recovered
//!     until the authoritative execution and verification boundaries
//!     establish that result.
//!
//! ## Scalability
//!
//! This composition boundary introduces no fixed limits on:
//!
//! - participants;
//! - machines;
//! - workers;
//! - tasks;
//! - retries;
//! - checkpoints;
//! - recovery operations;
//! - execution contexts;
//! - computational domains;
//! - hardware devices;
//! - recovery history;
//! - distributed resources.
//!
//! Actual limits may arise from:
//!
//! - available memory;
//! - storage capacity;
//! - address-space representation;
//! - provider capabilities;
//! - configured operational budgets;
//! - operating-system constraints;
//! - network availability;
//! - execution deadlines.
//!
//! Such limits belong to their respective authorities and must be
//! reported explicitly.
//!
//! An unlimited retry policy must never become an uncontrolled
//! infinite retry loop. Cancellation, deadlines, execution state,
//! semantic safety, and external policy remain authoritative.
//!
//! ## Dependency direction
//!
//! ```text
//! distributed::error
//! distributed::config
//! distributed::ids
//! distributed::model
//!        |
//!        v
//! distributed::resilience
//!        |
//!        +--> planning
//!        +--> execution
//!        +--> membership
//!        +--> transport
//!        +--> runtime
//!        +--> observability
//! ```
//!
//! The resilience subsystem consumes the existing distributed
//! foundations. Those foundations must not depend on this composition
//! root merely to define their own fundamental types.
//!
//! ## Existing implementation ownership
//!
//! `checkpoint.rs`:
//!     Owns checkpoint manifests, checkpoint safety evidence,
//!     integrity verification, storage-provider contracts,
//!     restoration verification, and checkpoint coordination.
//!
//! `retry.rs`:
//!     Owns deterministic retry evaluation, retry safety evidence,
//!     execution-state assessment, retry-policy evaluation,
//!     and retry decisions.
//!
//! Future recovery mechanisms must be implemented in dedicated
//! child modules rather than copied into this root.
//!
//! ## Integration with distributed configuration
//!
//! `crate::distributed::config` remains the authority for:
//!
//! - retry policies;
//! - backoff configuration;
//! - operational budgets;
//! - execution timeouts;
//! - configurable runtime policies.
//!
//! This module must not introduce a competing configuration model.
//!
//! ## Integration with distributed errors
//!
//! `crate::distributed::error` remains the authority for:
//!
//! - distributed execution errors;
//! - retry advice;
//! - delivery state;
//! - canonical error classification.
//!
//! Resilience must preserve the distinction between permanent,
//! transient, unsafe-to-repeat, and outcome-unknown failures.
//!
//! ## Integration with distributed identities
//!
//! `crate::distributed::ids` remains the identity authority.
//!
//! Checkpoint, task, operation, and execution-context identities
//! must remain consistent across retries and recovery.
//!
//! Physical process IDs, machine addresses, hardware identifiers,
//! and transport endpoints must never replace logical identities.
//!
//! ## Integration with canonical IR
//!
//! Resilience consumes the existing canonical representations.
//!
//! It must not create:
//!
//! - DistributedResilienceIR;
//! - DistributedRecoveryIR;
//! - QuantumRecoveryIR;
//! - a competing task IR;
//! - a competing execution-state IR.
//!
//! The existing ZUIR, ZCIR, quantum::ir, and HDLIR authorities
//! remain unchanged.
//!
//! ## Quantum integration
//!
//! The canonical quantum identity boundary remains:
//!
//! `crate::quantum::ir::qubit`.
//!
//! No resilience-local QubitId or PhysicalQubitId may be introduced.
//!
//! Quantum execution recovery must respect:
//!
//! - quantum-state legality;
//! - measurement boundaries;
//! - entanglement dependencies;
//! - QEC ownership;
//! - ZQN fault information;
//! - hardware capabilities;
//! - provider-supported recovery operations.
//!
//! A quantum operation must not be restored from an arbitrary
//! classical byte snapshot unless its authoritative provider
//! explicitly establishes that the representation is valid.
//!
//! ## Integration with runtime
//!
//! The distributed runtime owns actual execution and lifecycle.
//!
//! Resilience provides decisions and coordinates recovery through
//! explicit runtime interfaces.
//!
//! It must not directly construct hidden global runtimes, access
//! mutable process-wide state, or report simulated success.
//!
//! ## Integration with grammar
//!
//! The normative language contract is:
//!
//! `grammar/spec/distributed.md`
//!
//! Resilience intent is expressed through:
//!
//! `grammar/execution/resilience.g4`
//!
//! Recovery intent is expressed through:
//!
//! `grammar/execution/recovery.g4`
//!
//! Checkpoint intent is expressed through:
//!
//! `grammar/execution/checkpointing.g4`
//!
//! Distributed fault-tolerance intent is expressed through:
//!
//! `grammar/distributed/fault-tolerance.g4`
//!
//! Grammar defines portable intent, not implementation.
//!
//! This module must never attempt to interpret source syntax directly.
//!
//! ## Determinism
//!
//! This composition root performs no:
//!
//! - filesystem operations;
//! - network operations;
//! - hardware operations;
//! - clock access;
//! - random-number generation;
//! - environment inspection;
//! - background execution;
//! - global-state mutation;
//! - provider initialization.
//!
//! Child modules receive all decision inputs explicitly.
//!
//! ## Security
//!
//! This module does not bypass:
//!
//! - participant authentication;
//! - workload authorization;
//! - checkpoint integrity verification;
//! - recovery authorization;
//! - execution-provider security;
//! - resource access policies.
//!
//! Discovery does not establish trust.
//!
//! A recovered execution must remain subject to the same security
//! requirements as its original execution.
//!
//! ## Public namespace policy
//!
//! Child modules are exposed through qualified paths.
//!
//! Wildcard re-exports are intentionally prohibited.
//!
//! For example:
//!
//! `crate::distributed::resilience::retry::RetryEvaluator`
//!
//! `crate::distributed::resilience::checkpoint::CheckpointManager`
//!
//! Qualified namespaces prevent collisions as the subsystem grows.
//!
//! ## Extension policy
//!
//! New resilience functionality must be introduced through dedicated
//! modules with independently frozen contracts.
//!
//! A new module should be declared here only after its implementation
//! file exists and its ownership, dependencies, API, errors, tests,
//! and integration boundaries have been established.
//!
//! The root must not declare speculative modules or placeholder
//! implementations.
//!
//! ## Completion criteria
//!
//! This file is complete when:
//!
//! - existing resilience implementations are exposed exactly once;
//! - public module ownership is unambiguous;
//! - no duplicate implementation is introduced;
//! - no wildcard re-exports exist;
//! - no fixed scalability ceilings are introduced;
//! - no provider-specific behavior is introduced;
//! - no global state or I/O is accessed;
//! - no unsafe Rust is permitted;
//! - downstream modules can depend on stable qualified paths.
//!
//! Once completed, changes to retry algorithms, checkpoint storage,
//! recovery strategies, idempotency providers, and runtime integrations
//! should not require modifying this composition root unless a genuine
//! top-level ownership boundary is added or removed.

// =============================================================================
// Existing production modules
// =============================================================================

/// Durable, domain-aware checkpoint contracts and coordination.
///
/// This module owns checkpoint manifests, integrity validation,
/// provider capabilities, durable storage contracts, and verified
/// restoration.
pub mod checkpoint;

/// Deterministic retry-policy evaluation.
///
/// This module determines whether another attempt may be considered.
/// It does not execute retries, schedule work, or perform network I/O.
pub mod retry;
