
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Cross-Domain Data API
//!
//! Path: `src/hybrid/data/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the canonical public composition root for data contracts
//! used by Zamani's hybrid-computing subsystem.
//!
//! It exposes the existing implementations for:
//!
//! - references to canonical values, types, and provenance;
//! - cross-domain exchange contracts;
//! - cross-domain ownership and lifetime obligations;
//! - abstract data-layout requirements;
//! - target-independent transfer contracts.
//!
//! This module contains no independent data model or execution logic. Its
//! responsibility is to establish one discoverable API over the existing
//! implementations and keep their ownership boundaries explicit.
//!
//! ## Canonical ownership
//!
//! The child modules have separate, non-overlapping responsibilities:
//!
//! - [`values`] identifies canonical entities and cross-domain values.
//! - [`exchange`] declares the conditions under which values may cross
//!   computational-domain boundaries.
//! - [`ownership`] declares cross-domain ownership, access, lifetime,
//!   completion, cancellation, release, and coherence obligations.
//! - [`layout`] describes abstract representation and data-layout
//!   requirements.
//! - [`transfer`] declares abstract transfer modes, completion, ordering,
//!   integrity, cancellation, and failure obligations.
//!
//! The modules remain authoritative for their respective contracts.
//! Re-exporting a type does not transfer its ownership to this module.
//!
//! ## Integration boundaries
//!
//! This module does not replace or duplicate:
//!
//! - the canonical semantic type, effect, or ownership system;
//! - classical computation or classical IR;
//! - `quantum::ir` or any other domain-specific IR;
//! - the hybrid graph and dependency verifier;
//! - compiler lowering, optimization, placement, or scheduling;
//! - backend code generation;
//! - runtime execution, physical memory allocation, or resource release;
//! - hardware discovery, device selection, or network transport.
//!
//! Canonical references are declarations of identity, not proof that the
//! referenced entity exists or is compatible. Structural validation must be
//! followed by validation against the authoritative graph, semantic model,
//! domain interface, or runtime wherever that additional evidence is needed.
//!
//! ## Portability and scalability
//!
//! This module imposes no fixed maximum number of domains, values, exchanges,
//! transfers, dimensions, metadata entries, or layout descriptors.
//!
//! It does not impose universal limits on tensor rank, register width, memory
//! size, device count, or hardware capacity. Those properties are described
//! by canonical semantics, target capabilities, and explicit operational
//! policies rather than hidden limits in this API.
//!
//! Concrete Rust values and collections remain subject to available memory,
//! address space, processing time, and allocator behavior. This API does not
//! promise infinite physical resources or guaranteed execution on an
//! incompatible target.
//!
//! ## Grammar and semantic conformance
//!
//! The hybrid language specification and the existing grammar components
//! remain authoritative for source-language syntax and hybrid semantics.
//! This module introduces no source syntax, parser, AST, type system, or
//! second intermediate representation.
//!
//! Implementation and integration must remain consistent with
//! `grammar/spec/hybrid.md` and `grammar/hybrid/README.md`, including the
//! distinction between portable computational intent and target-specific
//! realization.
//!
//! ## Serialization
//!
//! Serialization and structural deserialization validation are implemented
//! by the owning child types. Re-exporting those types does not bypass their
//! constructors or validation rules.
//!
//! A structurally valid deserialized contract does not prove that referenced
//! entities exist, that a target supports the required representation, or
//! that a transfer has completed. Those claims require validation by the
//! relevant authoritative subsystem.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden throughout this module and its descendants.

/// Canonical references, cross-domain values, and value provenance.
///
/// Use this module when identifying values or semantic entities without
/// copying or redefining their domain-owned representations.
pub mod values;

/// Contracts describing cross-domain value exchanges.
///
/// Exchange declarations describe compatibility and dependencies; they do
/// not execute conversions or move physical data.
pub mod exchange;

/// Abstract ownership, access, lifetime, completion, and release contracts.
///
/// This module does not replace the language's ownership checker or manage
/// physical resources.
pub mod ownership;

/// Abstract data-layout and representation requirements.
///
/// Layout descriptors do not allocate memory, select devices, or choose
/// physical memory locations.
pub mod layout;

/// Target-independent transfer requirements and guarantees.
///
/// Transfer contracts do not implement data movement, transport, hardware
/// submission, or runtime lifecycle management.
pub mod transfer;

// -------------------------------------------------------------------------
// Canonical value references
// -------------------------------------------------------------------------

pub use self::values::{
    CanonicalEntityRef,
    CanonicalTypeRef,
    CrossDomainValue,
    ValueProvenance,
};

// -------------------------------------------------------------------------
// Cross-domain exchange contracts
// -------------------------------------------------------------------------

pub use self::exchange::{
    CrossDomainExchange,
    ExchangeOwnership,
    RepresentationCompatibility,
    TransferSemantics,
    ValueAvailability,
};

// -------------------------------------------------------------------------
// Cross-domain ownership contracts
// -------------------------------------------------------------------------

pub use self::ownership::{
    AccessRequirement,
    CancellationRequirement,
    CoherenceRequirement,
    CompletionRequirement,
    LifetimeKind,
    OwnershipContract,
    OwnershipOperation,
    ReleaseResponsibility,
};

// -------------------------------------------------------------------------
// Abstract data-layout contracts
// -------------------------------------------------------------------------

pub use self::layout::{
    AlignmentConstraint,
    ByteOrder,
    DataLayout,
    Dimension,
    DimensionExpr,
    DimensionInstruction,
    DimensionOrder,
    LayoutMapping,
    NamespacedKey,
    RankSpec,
};

// -------------------------------------------------------------------------
// Abstract transfer contracts
// -------------------------------------------------------------------------

pub use self::transfer::{
    TransferCancellation,
    TransferCompletion,
    TransferContract,
    TransferFailurePolicy,
    TransferIntegrity,
    TransferMode,
    TransferOrdering,
};
