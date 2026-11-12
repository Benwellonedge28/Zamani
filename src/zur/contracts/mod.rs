
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime (ZUR) — Contracts
//!
//! Path: `src/zur/contracts/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! This module is the canonical composition root for the contracts
//! defining the boundaries between ZUR and the other Zamani
//! architectural authorities.
//!
//! It exposes four independently owned contract families:
//!
//! - `abi`: Runtime application binary interface contracts.
//! - `execution`: Runtime execution request and lifecycle contracts.
//! - `provider`: Execution-provider integration contracts.
//! - `service`: Optional runtime service contracts.
//!
//! This module composes those contracts without implementing,
//! redefining, or duplicating their functionality.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Public composition of ZUR contract modules.
//! - Stable module paths for contract consumers.
//! - Documentation of the relationships between contract families.
//!
//! This module does NOT own:
//!
//! - Source-language syntax or semantic meaning.
//! - AST or semantic-model definitions.
//! - Requirements or capability definitions.
//! - Preferences, constraints, or implementation hints.
//! - Feasibility analysis.
//! - Resource allocation or reservation.
//! - Compiler planning or optimization.
//! - Compiler artifacts or artifact manifests.
//! - Backend registration or code generation.
//! - Domain-specific intermediate representations.
//! - Physical hardware or device discovery.
//! - Runtime service registration.
//! - Runtime service initialization.
//! - Provider discovery or authentication.
//! - Execution scheduling or placement.
//! - Domain-specific execution implementations.
//!
//! These responsibilities remain with their existing
//! authoritative subsystems.
//!
//! ## Contract boundaries
//!
//! The four modules have deliberately separate responsibilities.
//!
//! ### ABI
//!
//! `abi` defines how compatible executable entry points,
//! parameters, results, ownership, lifetimes, calling conventions,
//! and runtime service calls cross the execution boundary.
//!
//! It does not load artifacts or execute instructions.
//!
//! ### Execution
//!
//! `execution` defines how already-compiled executable artifacts
//! are referenced, submitted, observed, cancelled, and reported.
//!
//! It does not define compiler planning or provider registration.
//!
//! ### Provider
//!
//! `provider` defines how independently implemented execution
//! providers expose compatible interfaces and accept execution
//! operations.
//!
//! It does not discover physical devices, reserve resources,
//! or establish provider authorization.
//!
//! ### Service
//!
//! `service` defines declarative contracts for optional runtime
//! services, their versions, dependencies, and lifecycle
//! requirements.
//!
//! It does not register, initialize, or execute services.
//!
//! ## Fundamental architectural distinctions
//!
//! The contract system MUST preserve these distinctions:
//!
//! - Requirement != Capability.
//! - Capability != Preference.
//! - Preference != Constraint.
//! - Constraint != Hint.
//! - Hint != Feasibility.
//! - Feasibility != Resource reservation.
//! - Resource reservation != Execution.
//! - Provider registration != Provider availability.
//! - Provider availability != Successful execution.
//! - Execution submission != Execution completion.
//! - Cancellation request != Confirmed cancellation.
//! - ABI compatibility != Artifact integrity.
//! - Contract validity != Implementation correctness.
//!
//! No contract declaration may independently establish facts
//! owned by another subsystem.
//!
//! ## Open-world extensibility
//!
//! Contract interfaces must support independently developed
//! implementations and future computational domains.
//!
//! No closed inventory of computational domains, providers,
//! hardware architectures, devices, or resource types is
//! established here.
//!
//! Future integrations may include:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware execution.
//! - AI and tensor computation.
//! - Accelerators.
//! - Hybrid computation.
//! - Distributed computation.
//! - Photonic computation.
//! - Future computational technologies.
//!
//! New domains must integrate through the existing contract
//! boundaries rather than requiring changes to this composition
//! root merely to become recognizable.
//!
//! Unknown or unimplemented functionality must never be
//! interpreted as supported functionality.
//!
//! ## POCO-REAF and scalability
//!
//! The contract composition imposes no arbitrary maximum on:
//!
//! - ABI entry points.
//! - ABI parameters.
//! - Execution requests.
//! - Execution sessions.
//! - Runtime services.
//! - Service dependencies.
//! - Provider implementations.
//! - Provider interfaces.
//! - Computational domains.
//! - Devices.
//! - Resource quantities.
//!
//! Collections and representations grow subject to available
//! memory, implementation capabilities, and actual platform limits.
//!
//! Literal infinite physical computation is not promised.
//!
//! The objective is to avoid artificial architectural ceilings
//! while preserving portability across compatible execution
//! environments.
//!
//! ## Runtime independence
//!
//! These contracts do not require every Zamani program to use
//! a separately installed runtime.
//!
//! Runtime-free AOT execution remains a supported architectural
//! objective.
//!
//! Runtime-enabled and hybrid execution may consume these
//! contracts when their execution requirements need them.
//!
//! ## Safety
//!
//! All implementations must comply with the crate-wide safe-Rust
//! requirement.
//!
//! No unsafe code, unsafe implementations, or global mutable
//! contract registries are introduced by this module.
//!
//! ## Integration
//!
//! Upstream authorities:
//!
//! - `frontend/semantic/`: semantic meaning and requirements.
//! - `compiler/`: compilation and execution planning.
//! - `compiler/artifacts/`: canonical executable artifacts.
//! - `backend/`: code generation and artifact compatibility.
//! - `zuir/`: unified target-independent representation.
//! - `quantum/`: quantum computation and hardware contracts.
//! - `hybrid/`: hybrid composition and execution planning.
//! - `distributed/`: distributed execution contracts.
//! - `stdlib/`: common provider and operation-context contracts.
//!
//! ZUR consumers:
//!
//! - `zur/core/`: runtime composition and service management.
//! - `zur/execution/`: artifact loading and execution.
//! - `zur/domains/`: domain-provider integration.
//! - `zur/services/`: optional service implementations.
//! - `zur/security/`: execution security.
//! - `zur/observability/`: operational observations.
//!
//! Integration must preserve the ownership boundaries of all
//! upstream and downstream subsystems.
//!
//! ## Stability
//!
//! Each contract family is independently versioned where
//! its owning implementation requires versioning.
//!
//! Adding a new computational domain must not require changing
//! the existing contract modules unless a genuine shared
//! contract evolution is necessary.

// ============================================================================
// ABI CONTRACTS
// ============================================================================

/// Runtime application binary interface contracts.
///
/// Defines executable entry-point declarations, invocation modes,
/// calling conventions, value-transfer contracts, ownership,
/// lifetime, error-transfer, and runtime service invocation
/// declarations.
///
/// ABI definitions do not implement executable loading or
/// instruction execution.
///
/// See [`abi`] for the complete contract.
pub mod abi;

// ============================================================================
// EXECUTION CONTRACTS
// ============================================================================

/// Runtime execution request and lifecycle contracts.
///
/// Defines external artifact and plan references, execution
/// requests, execution handles, lifecycle observations,
/// cancellation dispositions, and execution-engine interfaces.
///
/// Execution contracts do not implement compilation, resource
/// reservation, or physical provider execution.
///
/// See [`execution`] for the complete contract.
pub mod execution;

// ============================================================================
// PROVIDER CONTRACTS
// ============================================================================

/// Runtime execution-provider integration contracts.
///
/// Defines provider-facing interfaces, operation requests,
/// execution outcomes, session interactions, streaming,
/// and provider contract validation.
///
/// Provider contracts do not own provider registration,
/// physical device discovery, or resource allocation.
///
/// See [`provider`] for the complete contract.
pub mod provider;

// ============================================================================
// SERVICE CONTRACTS
// ============================================================================

/// Optional runtime service contracts.
///
/// Defines service identities, service versions, dependencies,
/// startup requirements, readiness requirements, and declarative
/// service descriptors.
///
/// Service contracts do not perform service registration,
/// initialization, lifecycle execution, or dependency
/// graph execution.
///
/// See [`service`] for the complete contract.
pub mod service;
