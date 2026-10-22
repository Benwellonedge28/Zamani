
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Interchange
//!
//! Path: `src/hybrid/interchange/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the public module boundary for exchanging, persisting, and
//! restoring hybrid-computation documents.
//!
//! This module composes the existing schema, encoder, and decoder. It
//! intentionally contains no independent serialization implementation,
//! competing hybrid representation, schema migration engine, or
//! target-specific execution logic.
//!
//! ## Canonical ownership
//!
//! - Document structure and structural invariants: [`schema`].
//! - Encoding and encoded-byte budgets: [`encode`].
//! - Decoding, input validation, and compatibility checks:
//!   [`decode`].
//! - Operational budgets and serialization policies:
//!   [`crate::hybrid::config`].
//! - Structured failures: [`crate::hybrid::errors`].
//! - Schema and format versions: [`crate::hybrid::version`].
//! - Hybrid identifiers: [`crate::hybrid::ids`].
//! - Domain-interface contracts: [`crate::hybrid::contract`].
//! - Quantum computation semantics: the existing `quantum::ir` subsystem.
//! - Classical computation semantics: the existing canonical classical
//!   subsystem.
//! - Universal computation representation: the existing ZUIR subsystem.
//! - Target lowering and code generation: compiler and backend subsystems.
//! - Execution, scheduling, and device lifecycle: their existing owners.
//!
//! ## POCO-REAF and scalability
//!
//! The interchange boundary must not impose universal limits on domain
//! instances, graph nodes, graph edges, operations, tensor dimensions,
//! hardware resources, or computational scale.
//!
//! Encoding and decoding honor the caller's explicit operational budgets.
//! An absent budget is not a promise of unlimited physical resources:
//! allocation, address-space, parser, operating-system, and target
//! constraints still apply.
//!
//! Budget exhaustion must be reported explicitly. It must never cause
//! silent truncation, dropped dependencies, weakened requirements, or
//! changed program semantics.
//!
//! ## Interchange contract
//!
//! 1. Encode only documents accepted by the canonical schema validator.
//! 2. Decode untrusted bytes through the canonical decoder.
//! 3. Reject unsupported document kinds and incompatible versions.
//! 4. Preserve canonical document relationships and the ordering represented
//!    by the schema.
//! 5. Do not infer that successful decoding proves semantic validity,
//!    domain availability, target feasibility, or executability.
//! 6. Do not silently discard unknown required semantics.
//! 7. Keep schema evolution and version compatibility in their existing
//!    authoritative modules.
//!
//! ## Integration
//!
//! The compiler pipeline, persistence consumers, and runtime integrations
//! must use this module's canonical document and codec APIs rather than
//! implementing competing interchange formats.
//!
//! Frontend parsing, semantic analysis, graph construction, domain-specific
//! IR, hardware selection, lowering, scheduling, routing, QEC, and execution
//! remain outside this module.
//!
//! ## Safety
//!
//! This module and its descendants forbid unsafe Rust.

/// Canonical decoder for versioned hybrid interchange documents.
pub mod decode;

/// Canonical encoder for hybrid interchange documents.
pub mod encode;

/// Canonical document schema and structural validation.
pub mod schema;

// ---------------------------------------------------------------------------
// Stable convenience API
// ---------------------------------------------------------------------------

/// Decode untrusted bytes into the canonical hybrid document.
///
/// This is an alias for [`decode::decode`]. The decoder applies the
/// configured input budget, checks compatibility, deserializes the canonical
/// schema, and validates its structural invariants.
///
/// Semantic validation, domain resolution, and target feasibility remain
/// responsibilities of their canonical subsystems.
pub use decode::decode as decode_document;

/// Encode a validated hybrid document into deterministic JSON bytes.
///
/// This is an alias for [`encode::encode`]. The encoder applies configured
/// operational budgets and validates the document before returning bytes.
///
/// The input document's represented ordering is preserved; this function
/// does not reorder graph elements or invent a new canonical graph order.
pub use encode::encode as encode_document;

// ---------------------------------------------------------------------------
// Canonical schema exports
// ---------------------------------------------------------------------------

/// Stable identifier for the hybrid interchange document kind.
pub use schema::HYBRID_DOCUMENT_KIND;

/// Current interchange format version, as defined by the canonical schema.
pub use schema::FORMAT_VERSION;

/// Current logical schema version, as defined by the canonical schema.
pub use schema::SCHEMA_VERSION;

/// Versioned hybrid interchange document.
pub use schema::HybridDocument;

/// Document-level identity and provenance.
pub use schema::DocumentMetadata;

/// Optional artifact digest metadata.
pub use schema::ArtifactDigest;

/// Extensible document metadata entries.
pub use schema::MetadataEntry;

/// Domain-neutral hybrid composition.
pub use schema::HybridComposition;

/// A participating domain instance.
pub use schema::DomainInstanceRecord;

/// Reference to an artifact owned by another canonical subsystem.
pub use schema::CanonicalArtifactRef;

/// A node in the serialized hybrid composition.
pub use schema::HybridNodeRecord;

/// A source-location reference.
pub use schema::SourceLocationRef;

/// A dependency edge in the serialized composition.
pub use schema::HybridEdgeRecord;

/// A value exchanged across computational domains.
pub use schema::CrossDomainValueRecord;

/// A synchronization contract in the serialized composition.
pub use schema::SynchronizationRecord;

/// A portable semantic declaration.
pub use schema::SemanticDeclaration;

/// A namespaced extension record.
pub use schema::ExtensionRecord;

/// Errors produced by canonical schema validation.
pub use schema::SchemaError;
