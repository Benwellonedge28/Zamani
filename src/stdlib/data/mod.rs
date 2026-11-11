
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Data Subsystem
//!
//! Path: `src/stdlib/data/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the canonical public composition boundary for the
//! Zamani standard-library data subsystem.
//!
//! It exposes reusable, portable, target-independent data APIs that
//! can participate in classical, quantum, HDL, hybrid, distributed,
//! AI, scientific, embedded, and future computational domains.
//!
//! The subsystem follows the principle:
//!
//!     Program Once
//!     Compile Once
//!     Run Everywhere
//!     Anywhere
//!     Forever
//!
//! (POCO-REAF).
//!
//! The data subsystem describes and manipulates data through stable
//! contracts. It does not select the physical machine or determine
//! where an operation executes.
//!
//! # Architecture
//!
//! The public subsystem is composed of five independently owned
//! implementation modules:
//!
//! ```text
//!                   stdlib::data
//!                        |
//!          +-------------+-------------+
//!          |             |             |
//!          v             v             v
//!       buffers       storage    serialization
//!          |             |             |
//!          |             |             v
//!          |             |          formats
//!          |             |
//!          +-------------+-------------+
//!                        |
//!                        v
//!                    provenance
//!
//! ```
//!
//! The modules have distinct responsibilities.
//!
//! ## buffers
//!
//! Owns:
//!
//! - Contiguous byte buffers.
//! - Segmented byte buffers.
//! - Immutable byte views.
//! - Mutable byte views.
//! - Checked byte access.
//! - Fallible allocation and resizing.
//!
//! Does not own:
//!
//! - Tensor storage.
//! - Classical array semantics.
//! - Quantum registers.
//! - Device memory.
//! - DMA.
//! - Physical memory allocation.
//! - Distributed placement.
//!
//! ## storage
//!
//! Owns:
//!
//! - Logical storage identifiers.
//! - Storage metadata.
//! - Version and conditional-write contracts.
//! - Streaming read/write interfaces.
//! - Provider-neutral persistence contracts.
//! - Storage-provider interfaces.
//!
//! Does not own:
//!
//! - Filesystem implementations.
//! - Database implementations.
//! - Object-store implementations.
//! - Physical storage placement.
//! - Distributed consensus.
//! - Replication scheduling.
//! - Runtime resource allocation.
//!
//! ## serialization
//!
//! Owns:
//!
//! - Serialization and deserialization contracts.
//! - Open-world format identifiers.
//! - Serialization options.
//! - Codec-provider interfaces.
//! - Serialization provider registration.
//! - Serialization provider lookup.
//! - Streaming encoding and decoding.
//! - Serialization operation reports.
//!
//! Does not own:
//!
//! - Source-language parsing.
//! - Semantic type definitions.
//! - Schema semantics.
//! - Tensor representations.
//! - Storage placement.
//! - Cryptographic implementation.
//! - Physical execution.
//!
//! ## formats
//!
//! Owns:
//!
//! - The public format catalog facade.
//! - Format discovery.
//! - Format descriptor inspection.
//! - Media-type matching.
//! - Delegation to the canonical serialization registry.
//!
//! Does not own another format registry or provider implementation.
//!
//! ## provenance
//!
//! Owns:
//!
//! - Data-oriented provenance access.
//! - Provenance integration interfaces.
//! - Validated provenance operations.
//! - Provenance interoperability.
//!
//! The authoritative provenance types and graph remain owned by:
//!
//! `crate::frontend::semantic::model::provenance`
//!
//! This module does not introduce another provenance identity system,
//! graph implementation, or semantic record representation.
//!
//! # Canonical repository authorities
//!
//! The standard-library data subsystem integrates with the following
//! existing repository authorities.
//!
//! ## Frontend and semantics
//!
//! - `crate::frontend::ast`
//! - `crate::frontend::semantic`
//! - `crate::frontend::semantic::model::provenance`
//! - `crate::frontend::semantic::model::ids`
//!
//! These own source representation, semantic meaning, canonical
//! identities, and provenance invariants.
//!
//! ## Classical computation
//!
//! - `crate::classical`
//! - `crate::classical::collections`
//!
//! Classical tensor and collection implementations remain authoritative.
//!
//! The data subsystem must not create competing tensor storage.
//!
//! ## Quantum computation
//!
//! - `crate::quantum`
//! - `crate::quantum::ir`
//!
//! Quantum values and quantum intermediate representations retain
//! their existing authorities.
//!
//! Serialization may provide interoperability for quantum-related
//! data without taking ownership of quantum semantics.
//!
//! ## Hardware and hybrid computation
//!
//! - `crate::hdl`
//! - `crate::hybrid`
//! - `crate::runtime`
//! - HAL and registered hardware providers.
//!
//! Data APIs must remain independent of physical hardware selection,
//! allocation, placement, and execution.
//!
//! ## Artificial intelligence
//!
//! - `crate::ai`
//! - Existing canonical tensor and model authorities.
//!
//! AI datasets and tensors must reuse their established representations.
//!
//! ## Distributed computing
//!
//! - `crate::distributed`
//! - `crate::stdlib::distributed_ledger`
//!
//! Distributed persistence, replication, transport, and consensus
//! must be implemented by their respective authorities.
//!
//! Ledger provenance must reuse the canonical semantic provenance model.
//!
//! ## Standard-library infrastructure
//!
//! - `crate::stdlib::error`
//! - `crate::stdlib::context`
//! - `crate::stdlib::provider`
//! - `crate::stdlib::registry`
//!
//! These provide shared operation errors, context, and extensibility
//! infrastructure where the corresponding modules require them.
//!
//! # Grammar integration
//!
//! The data subsystem integrates with the canonical data grammar:
//!
//! - `grammar/data/data.g4`
//! - `grammar/data/provenance.g4`
//! - `grammar/data/serialization.g4`
//!
//! The universal grammar composition remains owned by:
//!
//! - `grammar/Zamani.g4`
//! - The canonical parser composition.
//!
//! Grammar responsibilities:
//!
//! - `data.g4` owns data-domain composition.
//! - `provenance.g4` owns data provenance syntax.
//! - `serialization.g4` owns serialization and deserialization syntax.
//!
//! These grammars express source-language intent.
//!
//! They do not instantiate storage providers, open files, allocate
//! buffers, execute serialization operations, or select hardware.
//!
//! The frontend translates source constructs into the canonical
//! AST and semantic representations.
//!
//! This Rust module does not implement another parser or AST.
//!
//! # Integration contracts
//!
//! ## Buffers and storage
//!
//! `buffers` provides owned and borrowed byte representations.
//!
//! `storage` provides provider-neutral streaming persistence contracts.
//!
//! A storage provider may consume or produce byte streams without
//! requiring an entire stored object to fit into one contiguous
//! allocation.
//!
//! ## Serialization and formats
//!
//! `serialization` is the sole authority for codec-provider
//! registration and serialization dispatch.
//!
//! `formats` delegates to that implementation.
//!
//! Format identifiers and provider metadata must not be duplicated.
//!
//! ## Provenance
//!
//! `provenance` delegates identity allocation, semantic records,
//! relationship definitions, graph storage, and graph validation
//! to the frontend semantic provenance authority.
//!
//! Persistence of provenance is an explicit storage operation.
//!
//! Recording provenance must not implicitly persist it.
//!
//! Recording evidence must not be interpreted as cryptographic
//! verification or proof of authenticity.
//!
//! ## Legacy compatibility
//!
//! The existing `crate::stdlib::serialize` module remains a
//! compatibility entry point.
//!
//! It must delegate to `crate::stdlib::data::serialization`.
//!
//! The existing `crate::stdlib::fs` module remains the filesystem
//! compatibility entry point.
//!
//! Filesystem implementations must not be duplicated here.
//!
//! Existing distributed-ledger provenance interfaces must migrate
//! toward the canonical `crate::stdlib::data::provenance` facade
//! and semantic provenance types.
//!
//! # POCO-REAF and scalability
//!
//! This subsystem introduces no arbitrary universal ceilings on:
//!
//! - Buffer sizes.
//! - Storage sizes.
//! - Object counts.
//! - Format counts.
//! - Provider counts.
//! - Serialization input sizes.
//! - Serialization output sizes.
//! - Schema counts.
//! - Records.
//! - Fields.
//! - Provenance records.
//! - Lineage relationships.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Distributed nodes.
//! - Devices.
//! - Computational domains.
//!
//! No universal maximum is defined for CPU, GPU, FPGA, QPU, memory,
//! storage, threads, nodes, qubits, registers, or devices.
//!
//! This does not promise literal infinite physical memory or
//! computation.
//!
//! Actual feasibility depends on:
//!
//! - Available resources.
//! - Platform address-space limits.
//! - Allocation success.
//! - Provider capabilities.
//! - Explicit resource requirements.
//! - Configured operational budgets.
//! - Runtime feasibility decisions.
//!
//! Such constraints must never become arbitrary language-level
//! capacity restrictions.
//!
//! # Requirement, Capability, Preference, Constraint, Hint,
//! # and Feasibility
//!
//! These concepts retain their separate authorities.
//!
//! The data subsystem must preserve their distinctions:
//!
//! - Requirement: What a data operation needs.
//! - Capability: What an implementation can provide.
//! - Preference: What an implementation or operation would prefer.
//! - Constraint: What must not be violated.
//! - Hint: Non-binding optimization information.
//! - Feasibility: Whether the requested operation can be performed.
//!
//! Data APIs must not silently turn preferences into requirements
//! or hints into mandatory constraints.
//!
//! # Concurrency
//!
//! The data subsystem must not introduce a competing global execution
//! engine or scheduler.
//!
//! Individual implementation modules are responsible for their
//! documented thread-safety and synchronization guarantees.
//!
//! Provider callbacks must not be invoked while holding unrelated
//! registry locks unless their documented contract explicitly
//! requires such synchronization.
//!
//! # Error handling
//!
//! Standard-library operations use:
//!
//! `crate::stdlib::error`
//!
//! Errors must preserve:
//!
//! - Failure category.
//! - Operation identity.
//! - Context.
//! - Causality.
//! - Retryability where applicable.
//!
//! Resource exhaustion must not be reported as successful completion.
//!
//! Unsupported capabilities must not be silently substituted with
//! fabricated results.
//!
//! # Safety
//!
//! This subsystem follows the crate's safe-Rust policy.
//!
//! No unsafe Rust is permitted.
//!
//! No implicit hardware selection.
//!
//! No implicit persistence.
//!
//! No fabricated successful operations.
//!
//! No hidden global provider selection.
//!
//! No logging or printing as a substitute for operation results.
//!
//! # Public module stability
//!
//! The following module paths form the public composition surface:
//!
//! - `crate::stdlib::data::buffers`
//! - `crate::stdlib::data::storage`
//! - `crate::stdlib::data::serialization`
//! - `crate::stdlib::data::formats`
//! - `crate::stdlib::data::provenance`
//!
//! Implementation details should remain inside their owning modules.
//!
//! New data functionality must be added to its authoritative module
//! before being exposed through this composition boundary.
//!
//! # Completion criteria
//!
//! This file is complete when:
//!
//! - All five declared modules exist.
//! - Their public APIs compile together.
//! - Their dependencies resolve without circular ownership.
//! - The root standard-library module exports `data`.
//! - Legacy serialization delegates to the canonical engine.
//! - Format registration has exactly one authority.
//! - Provenance has exactly one semantic graph authority.
//! - Tensor storage has exactly one canonical authority.
//! - Grammar and frontend integration tests pass.
//! - Workspace build and test validation pass.
//! - No unsafe Rust is introduced.

// ============================================================================
// DATA SUBSYSTEM MODULES
// ============================================================================

/// Portable owned and borrowed byte-buffer abstractions.
///
/// Owns contiguous and segmented host-side byte storage and views.
///
/// Does not own tensors, device memory, or physical allocation.
pub mod buffers;

/// Provider-neutral persistent storage contracts.
///
/// Owns logical storage keys, metadata, conditional-write contracts,
/// streaming interfaces, and storage-provider abstractions.
///
/// Does not implement a filesystem, database, object store,
/// distributed placement, or physical storage engine.
pub mod storage;

/// Canonical serialization and deserialization infrastructure.
///
/// Owns format identifiers, codec-provider contracts, serialization
/// options, registry, dispatch, and streaming operations.
///
/// This is the sole serialization implementation authority.
pub mod serialization;

/// Public data-format catalog facade.
///
/// Delegates format discovery, inspection, registration, and removal
/// to the canonical serialization infrastructure.
///
/// Does not create a second registry or codec-provider system.
pub mod formats;

/// Canonical semantic provenance integration.
///
/// Reuses the frontend semantic provenance model and exposes its
/// data-oriented standard-library integration surface.
///
/// Does not introduce another provenance graph or identity authority.
pub mod provenance;
