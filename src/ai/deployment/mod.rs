
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Deployment
//!
//! Path: `src/ai/deployment/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the canonical composition root for AI deployment
//! contracts and their integration with Zamani's existing backend,
//! compatibility, distributed execution, and runtime infrastructure.
//!
//! It exposes the independently implemented AI deployment components
//! through one stable module boundary.
//!
//! AI deployment describes the portable intent and lifecycle of
//! deploying AI computational artifacts. It does not implement a
//! competing compiler, backend, artifact store, scheduler, or runtime.
//!
//! ## Architecture
//!
//! AI source
//!     |
//!     v
//! Canonical frontend AST
//!     |
//!     v
//! AI semantic validation
//!     |
//!     v
//! AI model and computational descriptions
//!     |
//!     v
//! AI deployment artifacts
//!     |
//!     v
//! AI deployment manifest
//!     |
//!     v
//! Compatibility evaluation
//!     |
//!     v
//! Distributed / backend / runtime realization
//!     |
//!     v
//! Provider-confirmed lifecycle transitions
//!
//! ## Module ownership
//!
//! This composition root exposes five independently owned components:
//!
//! - `artifacts`: AI-specific artifact descriptors and associations.
//! - `formats`: Extensible model-format providers and conversions.
//! - `manifests`: Portable AI deployment manifests.
//! - `compatibility`: Deployment compatibility evaluation.
//! - `lifecycle`: Logical AI deployment lifecycle and transition journal.
//!
//! Each component retains its own implementation and validation rules.
//!
//! ## Canonical ownership boundaries
//!
//! AI artifact descriptors:
//!     `crate::ai::deployment::artifacts`
//!
//! Canonical artifact payloads, encoding and integrity:
//!     `crate::backend::artifact`
//!
//! Canonical backend artifact identities:
//!     `crate::backend::ids`
//!
//! Persistent backend artifact manifests:
//!     `crate::backend::manifest`
//!
//! AI deployment manifest:
//!     `crate::ai::deployment::manifests`
//!
//! AI format-provider registry:
//!     `crate::ai::deployment::formats`
//!
//! AI compatibility:
//!     `crate::ai::compatibility`
//!
//! Backend capability and resource compatibility:
//!     `crate::backend::compatibility`
//!
//! AI deployment compatibility:
//!     `crate::ai::deployment::compatibility`
//!
//! AI deployment lifecycle:
//!     `crate::ai::deployment::lifecycle`
//!
//! Distributed execution lifecycle:
//!     `crate::distributed::execution::lifecycle`
//!
//! Physical deployment and execution:
//!     Canonical distributed, backend and runtime infrastructure.
//!
//! ## Ownership restrictions
//!
//! This module MUST NOT implement:
//!
//! - A second artifact payload representation.
//! - A second artifact identity system.
//! - A second persistent manifest system.
//! - A second format-provider registry.
//! - A second compatibility evaluator.
//! - A second deployment lifecycle authority.
//! - Model training or inference.
//! - Model serialization implementations.
//! - Filesystem or object-storage persistence.
//! - Cryptographic signing or verification.
//! - Device discovery.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Placement or scheduling.
//! - Network provisioning.
//! - Process or container management.
//! - Deployment orchestration.
//! - Model execution.
//!
//! These responsibilities remain with their established repository
//! authorities.
//!
//! ## POCO-REAF and scalability
//!
//! Deployment descriptions are independent of the size and topology
//! of the execution environment.
//!
//! No universal maximum is imposed on:
//!
//! - AI models.
//! - Model parameters.
//! - Artifacts.
//! - Artifact dependencies.
//! - Deployment instances.
//! - Replicas.
//! - Workers.
//! - Nodes.
//! - Accelerators.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Tensor dimensions.
//! - Deployment revisions.
//! - Lifecycle events.
//!
//! Actual feasibility is determined by declared requirements,
//! implementation capabilities, operational budgets, available
//! resources and physical constraints.
//!
//! This architecture supports scaling from resource-constrained
//! embedded environments to large heterogeneous and distributed
//! systems without changing the source-level computational meaning.
//!
//! It does not promise physically infinite resources.
//!
//! ## Dependency direction
//!
//! The deployment components depend on established AI and backend
//! contracts. The deployment composition root does not introduce
//! reverse dependencies into those foundational modules.
//!
//! The dependency relationships are:
//!
//! `artifacts`
//!     -> backend artifact infrastructure
//!
//! `formats`
//!     -> artifacts
//!     -> backend artifact formats
//!
//! `manifests`
//!     -> artifacts
//!     -> AI identities and versions
//!     -> backend manifests
//!
//! `compatibility`
//!     -> manifests
//!     -> AI compatibility
//!     -> backend compatibility
//!
//! `lifecycle`
//!     -> manifests
//!
//! Lifecycle state changes do not execute deployment operations.
//!
//! ## Integration contract
//!
//! 1. Construct and validate AI artifact descriptors.
//! 2. Validate payloads using registered format providers.
//! 3. Associate AI artifacts with canonical backend artifacts.
//! 4. Construct and validate portable AI deployment manifests.
//! 5. Evaluate compatibility using supplied target evidence.
//! 6. Preserve indeterminate and migration-required outcomes.
//! 7. Submit accepted deployment intent to the responsible
//!    distributed or runtime infrastructure.
//! 8. Record lifecycle transitions only when supported by the
//!    appropriate operation evidence.
//!
//! No step implicitly performs a subsequent step.
//!
//! In particular, compatibility does not mean deployment success,
//! and a requested deployment does not mean an active deployment.
//!
//! ## Error handling
//!
//! Every submodule retains its structured error types.
//!
//! Errors must propagate without:
//!
//! - silently discarding their causes;
//! - converting unknown outcomes into success;
//! - treating missing compatibility evidence as approval;
//! - treating an unverified artifact as verified;
//! - treating a requested transition as a confirmed operation.
//!
//! ## Concurrency
//!
//! This module introduces no global mutable state, locks, threads,
//! task executors, or hidden registries.
//!
//! Concurrency and synchronization remain owned by the individual
//! components and their callers.
//!
//! ## Serialization and portability
//!
//! Portable deployment descriptions use the versioned contracts
//! defined by the manifest and backend artifact infrastructure.
//!
//! Machine-local paths, device identities, physical placement,
//! and implicit timestamps must not be introduced into portable
//! deployment descriptions.
//!
//! ## Testing
//!
//! Each child module owns its implementation-specific unit tests.
//!
//! Integration tests should additionally verify:
//!
//! - All five deployment modules are publicly accessible.
//! - AI artifact identities remain distinct from backend identities.
//! - Canonical backend payloads are reused.
//! - Format validation is provider-driven.
//! - Manifest validation preserves version and integrity contracts.
//! - Unknown compatibility evidence is not accepted as success.
//! - Lifecycle transitions preserve their revision invariants.
//! - No deployment operation is claimed without provider confirmation.
//! - No artificial hardware or workload ceilings are introduced.
//!
//! ## Freeze criteria
//!
//! This module is complete when:
//!
//! - Every deployment component is declared exactly once.
//! - Existing component ownership is preserved.
//! - Existing cross-module paths resolve.
//! - No duplicate implementation is introduced.
//! - All public child modules are accessible.
//! - Rust 1.97+ compatibility is maintained.
//! - No unsafe Rust is used.
//! - The repository's relevant compilation and test checks pass.

// ============================================================================
// AI DEPLOYMENT COMPONENTS
// ============================================================================

/// AI artifact descriptors and canonical backend artifact associations.
///
/// Owns AI-specific artifact classification, identity association,
/// versions, metadata and provenance references.
///
/// Payload storage and integrity implementation remain in
/// `crate::backend::artifact`.
pub mod artifacts;

/// Extensible AI model-format descriptors and provider interfaces.
///
/// Owns format registration, format validation, conversion contracts
/// and conversion fidelity reporting.
///
/// Actual payload storage and encoding remain with the backend artifact
/// infrastructure.
pub mod formats;

/// Portable, versioned AI deployment manifests.
///
/// Owns deployment artifact records, AI-to-backend identity associations,
/// dependency relationships, manifest validation and deterministic
/// manifest fingerprints.
///
/// Persistent backend artifact manifests remain owned by
/// `crate::backend::manifest`.
pub mod manifests;

/// AI deployment compatibility evaluation.
///
/// Coordinates AI semantic compatibility, deployment manifest
/// validation, backend capability compatibility and resource
/// feasibility evidence.
///
/// It does not discover hardware, allocate resources, schedule work
/// or perform deployment.
pub mod compatibility;

/// Logical AI deployment lifecycle and transition journaling.
///
/// Owns lifecycle states, legal transitions, revisions and in-process
/// lifecycle snapshots.
///
/// Physical deployment operations and distributed execution lifecycle
/// remain owned by their respective infrastructure.
pub mod lifecycle;
