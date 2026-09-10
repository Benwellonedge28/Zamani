//! # Zamani Frontend AST — Serialization Boundary
//!
//! `src/frontend/ast/node/serialization/mod.rs`
//!
//! Production serialization boundary for the native Zamani frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! native Zamani AST
//!     │
//!     ├──────────────► AST validation
//!     │
//!     ├──────────────► AST traversal
//!     │
//!     ├──────────────► diagnostics / tooling
//!     │
//!     └──────────────► AST serialization  ← this subsystem
//!                              │
//!                              ▼
//!                    versioned AST document
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                 storage            transport
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                       AST deserialization
//!                              │
//!                              ▼
//!                         native AST
//!                              │
//!                              ▼
//!                     semantic analysis
//!                              │
//!                              ▼
//!                            ZUIR
//! ```
//!
//! ## Fundamental architectural rule
//!
//! This module serializes the **native Zamani AST**.
//!
//! It is NOT:
//!
//! - Quantum IR serialization;
//! - ZUIR serialization;
//! - QIR serialization;
//! - OpenQASM serialization;
//! - hardware serialization;
//! - backend serialization;
//! - compiler-cache serialization;
//! - semantic-model serialization.
//!
//! Those are separate contracts owned by their respective layers.
//!
//! The dependency direction MUST remain:
//!
//! ```text
//! frontend AST
//!      │
//!      ▼
//! AST serialization
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target IR
//! ```
//!
//! Never:
//!
//! ```text
//! AST serialization
//!      │
//!      └──► quantum IR
//! ```
//!
//! This is essential for POCO-REAF:
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Native AST
//!      │
//!      ▼
//! Semantic meaning
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├──► classical
//!      ├──► quantum
//!      ├──► HDL
//!      ├──► accelerator
//!      ├──► distributed
//!      └──► future domains
//! ```
//!
//! No serialization decision made here may encode a particular machine,
//! processor, QPU, vendor, topology, qubit count, gate set, scheduler,
//! calibration, or backend.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - the public AST serialization API;
//! - AST serialization schema identity;
//! - AST serialization versioning boundary;
//! - resource-policy configuration for serialization;
//! - serialization/deserialization error classification;
//! - serialized-document validation before AST reconstruction;
//! - deterministic serialization policy;
//! - compatibility-policy entry points;
//! - extension-preservation policy;
//! - the public integration boundary for sibling codecs.
//!
//! This module does NOT own:
//!
//! - AST node definitions;
//! - source spans;
//! - node IDs;
//! - node kinds;
//! - semantic analysis;
//! - type checking;
//! - name resolution;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - scheduling;
//! - routing;
//! - QEC;
//! - calibration;
//! - backend execution.
//!
//! Those responsibilities remain with their owning modules.
//!
//! ## Why this is a facade
//!
//! The serialization subsystem is deliberately decomposable.
//!
//! The target structure is:
//!
//! ```text
//! serialization/
//! ├── mod.rs             ← this file; public contract/facade
//! ├── document.rs        ← document envelope
//! ├── schema.rs          ← schema/version identity
//! ├── encoder.rs         ← deterministic AST encoding
//! ├── decoder.rs         ← deterministic AST decoding
//! ├── limits.rs          ← explicit resource policy
//! ├── error.rs           ← serialization errors
//! ├── compatibility.rs   ← compatibility/migration policy
//! └── tests.rs            ← serialization contract tests
//! ```
//!
//! The exact sibling files may be introduced incrementally. This facade is
//! intentionally designed so that adding them does not require redesigning
//! AST node definitions.
//!
//! ## Version domains
//!
//! These version spaces MUST remain independent:
//!
//! ```text
//! Zamani language version
//!          ≠
//! compiler version
//!          ≠
//! AST node schema version
//!          ≠
//! AST serialization format version
//!          ≠
//! AST extension version
//!          ≠
//! semantic-model version
//!          ≠
//! ZUIR version
//!          ≠
//! Quantum IR version
//!          ≠
//! hardware version
//! ```
//!
//! This module therefore does NOT import any downstream IR version type.
//!
//! In particular, the native AST serializer MUST NOT depend on:
//!
//! ```text
//! quantum::ir::identity::IrVersion
//! quantum::ir::serialization::*
//! ZUIR version types
//! backend version types
//! ```
//!
//! ## Existing AST integration
//!
//! The common [`super::Node`] already contains:
//!
//! - [`super::NodeId`];
//! - [`super::NodeKind`];
//! - [`super::source::Span`];
//! - [`super::metadata::NodeMetadata`].
//!
//! `NodeId` is deliberately a stable numeric identity and does not encode
//! hardware or semantic identity. Serialization must preserve that distinction.
//!
//! The serializer therefore treats AST identity as AST data only.
//!
//! ## Determinism
//!
//! Given:
//!
//! ```text
//! identical AST
//! + identical AST schema version
//! + identical serialization format version
//! + identical serialization policy
//! ```
//!
//! the canonical serialization must be identical.
//!
//! Serialization MUST NOT depend on:
//!
//! - memory addresses;
//! - pointer addresses;
//! - process IDs;
//! - thread IDs;
//! - wall-clock time;
//! - random values;
//! - environment variables;
//! - filesystem ordering;
//! - hash-map iteration order;
//! - backend discovery;
//! - hardware discovery.
//!
//! Semantic sequence ordering MUST be preserved. The serializer must never
//! arbitrarily sort AST children merely to obtain deterministic output.
//!
//! If a semantic map/dictionary is represented by an unordered collection,
//! its owning AST representation must provide a deterministic ordering contract
//! before serialization.
//!
//! ## Scalability
//!
//! There is deliberately NO:
//!
//! ```text
//! MAX_AST_NODES
//! MAX_AST_DEPTH
//! MAX_QUANTUM_NODES
//! MAX_QUBITS
//! MAX_REGISTERS
//! MAX_FUNCTIONS
//! MAX_EXPRESSIONS
//! MAX_STATEMENTS
//! MAX_PROGRAM_SIZE
//! ```
//!
//! in this module.
//!
//! The AST schema is therefore independent of computational-machine size.
//!
//! Actual serialization remains finite because every concrete execution has
//! finite resources. Those limits belong to an explicit [`SerializationLimits`]
//! policy rather than the AST language semantics.
//!
//! This distinction is mandatory:
//!
//! ```text
//! AST capability
//!     ≠
//! process resource budget
//!     ≠
//! hardware capacity
//! ```
//!
//! A small embedded compiler can select a small policy.
//!
//! A workstation compiler can select a larger policy.
//!
//! A distributed compiler can select a policy appropriate to its worker.
//!
//! None of those policies changes the AST schema.
//!
//! ## Security boundary
//!
//! Serialized AST data must always be considered untrusted when it crosses:
//!
//! - a filesystem boundary;
//! - a network boundary;
//! - IPC;
//! - a cache boundary;
//! - a plugin boundary;
//! - a package boundary;
//! - an IDE/LSP boundary;
//! - a user-controlled input boundary.
//!
//! Before reconstruction, implementations must validate:
//!
//! - document framing;
//! - schema identity;
//! - serialization version;
//! - declared document size;
//! - actual document size;
//! - integer conversions;
//! - collection lengths;
//! - string lengths;
//! - extension lengths;
//! - structural nesting;
//! - required fields;
//! - duplicate fields where forbidden;
//! - malformed values;
//! - trailing data;
//! - checksum/integrity data when the selected format provides it.
//!
//! No unsafe code is permitted.
//!
//! ## Serialization is not semantic validation
//!
//! Successful deserialization means:
//!
//! ```text
//! bytes
//!   │
//!   ▼
//! valid serialization document
//!   │
//!   ▼
//! structurally reconstructed AST
//! ```
//!
//! It does NOT automatically mean:
//!
//! ```text
//! semantically valid Zamani program
//! ```
//!
//! Semantic validation remains downstream.
//!
//! This separation is critical because serialization must be usable for:
//!
//! - parser snapshots;
//! - AST caches;
//! - IDE tooling;
//! - compiler persistence;
//! - source transformations;
//! - debugging;
//! - reproducibility;
//! - distributed compilation.
//!
//! ## Unknown extensions
//!
//! The native AST is extensible.
//!
//! Consequently, serialization must define explicit handling for unknown
//! extension data.
//!
//! A caller must be able to choose a policy such as:
//!
//! ```text
//! Reject
//! Preserve
//! Skip
//! ```
//!
//! Unknown semantic information must never silently disappear under a policy
//! that claims lossless AST persistence.
//!
//! The exact policy implementation belongs to the codec layer; this module
//! exposes the stable configuration boundary.
//!
//! ## Losslessness
//!
//! A normal AST round trip should satisfy:
//!
//! ```text
//! AST
//!   │
//!   ▼
//! serialize
//!   │
//!   ▼
//! bytes
//!   │
//!   ▼
//! deserialize
//!   │
//!   ▼
//! AST'
//! ```
//!
//! where `AST'` is structurally equivalent to `AST`.
//!
//! The round trip must preserve, where represented by the AST contract:
//!
//! - node identity;
//! - node kind;
//! - source spans;
//! - metadata;
//! - declarations;
//! - expressions;
//! - statements;
//! - types;
//! - patterns;
//! - generics;
//! - paths;
//! - annotations;
//! - effects;
//! - capabilities;
//! - resources;
//! - domains;
//! - extension payloads.
//!
//! The serializer must not invent semantic information that was not present in
//! the original AST.
//!
//! ## Source fidelity
//!
//! The AST is source-oriented. Serialization must therefore preserve source
//! coordinates and source provenance represented by the AST.
//!
//! It must NOT normalize source spans merely because a target backend has a
//! different coordinate system.
//!
//! ## Node identity
//!
//! [`super::node_id::NodeId`] is an AST identity, not:
//!
//! - a quantum-qubit identity;
//! - a physical-qubit identity;
//! - a semantic symbol identity;
//! - a ZUIR value ID;
//! - a backend job ID.
//!
//! Serialization must not reinterpret it.
//!
//! ## Extension boundary
//!
//! Domain-specific AST extensions may eventually include:
//!
//! - quantum source constructs;
//! - HDL constructs;
//! - accelerator constructs;
//! - future computational constructs.
//!
//! The serialization facade must remain domain-neutral.
//!
//! Extension implementations may register their own schema information, but
//! the core serialization module must not contain vendor-specific or
//! hardware-specific variants.
//!
//! Therefore adding a new quantum technology must NOT require changing this
//! file merely because the technology is new.
//!
//! ## Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! This file uses only stable language/library facilities and existing project
//! dependencies.
//!
//! -----------------------------------------------------------------------------
//! Public contract
//! -----------------------------------------------------------------------------
//!
//! The public facade intentionally uses generic `Serialize` / `Deserialize`
//! bounds rather than coupling itself to a particular AST node.
//!
//! This means the serialization boundary can serve:
//!
//! - a complete AST;
//! - an AST fragment;
//! - tooling snapshots;
//! - parser fixtures;
//! - generated AST;
//! - future AST extensions.
//!
//! The serializer must still be the authoritative place for schema/version
//! validation.
//!
//! -----------------------------------------------------------------------------
//! Module contract
//! -----------------------------------------------------------------------------
//!
//! `mod.rs` owns:
//!
//! - public API composition;
//! - public type aliases;
//! - version constants;
//! - high-level orchestration.
//!
//! `encoder.rs` owns byte generation.
//!
//! `decoder.rs` owns byte interpretation.
//!
//! `document.rs` owns the envelope.
//!
//! `schema.rs` owns schema identity.
//!
//! `limits.rs` owns resource policies.
//!
//! `error.rs` owns error representation.
//!
//! `compatibility.rs` owns migration/compatibility decisions.
//!
//! No sibling is permitted to depend on parser, semantic analysis, ZUIR,
//! quantum hardware, or backend modules merely to serialize an AST.
//!
//! -----------------------------------------------------------------------------
//! No unsafe
//! -----------------------------------------------------------------------------
//!
//! `forbid(unsafe_code)` is intentionally enforced at the module boundary.
//!
//! -----------------------------------------------------------------------------
//! Integration
//! -----------------------------------------------------------------------------
//!
//! Expected dependency direction:
//!
//! ```text
//! node/*
//!   │
//!   ▼
//! node/serialization/*
//!   │
//!   ▼
//! frontend/ast
//!   │
//!   ├──► semantic
//!   └──► tooling
//! ```
//!
//! The reverse dependency is forbidden.
//!
//! -----------------------------------------------------------------------------
//! Future sibling-file contract
//! -----------------------------------------------------------------------------
//!
//! When the sibling modules are introduced, this facade should expose them
//! without changing the AST node contracts:
//!
//! ```text
//! pub mod compatibility;
//! pub mod decoder;
//! pub mod document;
//! pub mod encoder;
//! pub mod error;
//! pub mod limits;
//! pub mod schema;
//! ```
//!
//! This file deliberately does not assume that all siblings already exist.
//! That allows foundational work to be completed independently and integrated
//! in dependency order.
//!
//! -----------------------------------------------------------------------------
//! Current implementation strategy
//! -----------------------------------------------------------------------------
//!
//! The generic serialization API below is intentionally implemented directly
//! with `serde_json` until the repository establishes a dedicated canonical
//! binary AST format.
//!
//! JSON is an interchange representation here, not the AST itself.
//!
//! A future canonical binary codec may replace the implementation behind this
//! facade without changing AST node definitions or callers that depend only on
//! this public API.
//!
//! -----------------------------------------------------------------------------
//! Important distinction
//! -----------------------------------------------------------------------------
//!
//! `serde_json` serialization order is deterministic for ordinary derived
//! struct representations used by the AST, but this facade does not claim that
//! arbitrary user-defined `Serialize` implementations are canonical.
//!
//! For that reason:
//!
//! - `serialize` is the general AST persistence API;
//! - `serialize_canonical` is reserved for AST types whose serialization
//!   contract explicitly guarantees canonical field ordering;
//! - cryptographic/content-addressed identity must not be inferred merely from
//!   `serialize`.
//!
//! This prevents a generic serde implementation from accidentally becoming a
//! false canonical-hashing contract.
//!
//! -----------------------------------------------------------------------------
//! Tests
//! -----------------------------------------------------------------------------
//!
//! The eventual `tests.rs` must cover:
//!
//! - empty/minimal AST;
//! - ordinary programs;
//! - very large programs;
//! - deeply nested programs;
//! - large metadata;
//! - Unicode;
//! - generated nodes;
//! - extension nodes;
//! - unknown extension policy;
//! - invalid schema version;
//! - invalid serialization version;
//! - truncated input;
//! - oversized input;
//! - malformed JSON;
//! - trailing data;
//! - round trips;
//! - deterministic output;
//! - concurrent read-only serialization;
//! - resource-limit enforcement;
//! - no panic on malformed input.
//!
//! Property/fuzz tests should additionally establish:
//!
//! ```text
//! arbitrary bytes → never UB / never unsafe / never unchecked allocation
//! ```
//!
//! -----------------------------------------------------------------------------
//! Production invariant
//! -----------------------------------------------------------------------------
//!
//! The native AST serializer must remain capable of representing programs from
//! tiny programs to programs whose size is limited only by the explicitly
//! selected resource policy and the actual available resources.
//!
//! No AST serialization constant may become a hidden language or hardware
//! limit.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::io;

/// Current AST serialization format major version.
///
/// This is deliberately independent of:
//!
//! - AST node schema version;
//! - Zamani language version;
//! - compiler version;
//! - semantic model version;
//! - ZUIR version;
//! - Quantum IR version.
pub const SERIALIZATION_FORMAT_MAJOR: u16 = 1;

/// Current AST serialization format minor version.
pub const SERIALIZATION_FORMAT_MINOR: u16 = 0;

/// Current AST serialization format patch version.
pub const SERIALIZATION_FORMAT_PATCH: u16 = 0;

/// Current AST serialization format version.
pub const CURRENT_SERIALIZATION_VERSION: SerializationVersion =
    SerializationVersion::new(
        SERIALIZATION_FORMAT_MAJOR,
        SERIALIZATION_FORMAT_MINOR,
        SERIALIZATION_FORMAT_PATCH,
    );

/// Stable identifier for the native Zamani AST serialization format.
pub const SERIALIZATION_FORMAT_ID: &str =
    "zamani.frontend.ast.serialization";

/// Stable media/type identifier for the JSON interchange representation.
pub const JSON_MEDIA_TYPE: &str = "application/json";

/// Version of the AST serialization representation.
///
/// This version describes the wire representation, not the semantic AST.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct SerializationVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SerializationVersion {
    /// Creates a serialization-format version.
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns the current serialization version.
    #[must_use]
    pub const fn current() -> Self {
        CURRENT_SERIALIZATION_VERSION
    }

    /// Returns whether this is exactly the current format version.
    #[must_use]
    pub const fn is_current(self) -> bool {
        self.major == CURRENT_SERIALIZATION_VERSION.major
            && self.minor == CURRENT_SERIALIZATION_VERSION.minor
            && self.patch == CURRENT_SERIALIZATION_VERSION.patch
    }

    /// Returns whether two versions share the same major format contract.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether this reader can interpret `other` under the conservative
    /// compatibility rule.
    ///
    /// A reader may consume an older minor/patch representation of the same
    /// major version. Future versions are never assumed compatible.
    #[must_use]
    pub const fn supports(self, other: Self) -> bool {
        other.major == self.major
            && (other.minor < self.minor
                || (other.minor == self.minor
                    && other.patch <= self.patch))
    }

    /// Returns whether an explicit migration is required.
    #[must_use]
    pub const fn requires_migration(self, other: Self) -> bool {
        !self.supports(other)
    }
}

impl Default for SerializationVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for SerializationVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

/// Policy controlling resource consumption during AST serialization.
///
/// These values are **operational safety policies**, not language limits.
///
/// `None` means that this particular layer imposes no application-level limit.
/// The process, allocator, operating system, and available storage remain
/// physical limits.
///
/// No value in this type represents:
///
/// - maximum qubits;
/// - maximum machine size;
/// - maximum AST semantic complexity;
/// - maximum hardware size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerializationLimits {
    /// Maximum serialized document size in bytes.
    ///
    /// `None` means no application-level limit.
    pub max_document_bytes: Option<u64>,

    /// Maximum serialized metadata/field value in bytes.
    ///
    /// `None` means no application-level limit.
    pub max_field_bytes: Option<u64>,

    /// Maximum number of AST collection elements accepted by a decoder.
    ///
    /// `None` means no application-level collection limit.
    pub max_collection_elements: Option<u64>,

    /// Maximum structural nesting accepted by the serialization decoder.
    ///
    /// `None` delegates to the underlying codec's own structural policy.
    pub max_nesting_depth: Option<u64>,
}

impl SerializationLimits {
    /// Creates an unrestricted policy.
    ///
    /// "Unrestricted" means no application-level limit. It does not mean
    /// infinite memory or infinite address space.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_document_bytes: None,
            max_field_bytes: None,
            max_collection_elements: None,
            max_nesting_depth: None,
        }
    }

    /// Creates an explicit resource policy.
    #[must_use]
    pub const fn new(
        max_document_bytes: Option<u64>,
        max_field_bytes: Option<u64>,
        max_collection_elements: Option<u64>,
        max_nesting_depth: Option<u64>,
    ) -> Self {
        Self {
            max_document_bytes,
            max_field_bytes,
            max_collection_elements,
            max_nesting_depth,
        }
    }

    /// Creates a conservative policy for untrusted service boundaries.
    ///
    /// These values are operational defaults only.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_document_bytes: Some(256 * 1024 * 1024),
            max_field_bytes: Some(16 * 1024 * 1024),
            max_collection_elements: Some(16 * 1024 * 1024),
            max_nesting_depth: Some(4096),
        }
    }

    /// Validates the policy itself.
    pub fn validate(self) -> Result<(), SerializationError> {
        if let Some(value) = self.max_document_bytes {
            if value == 0 {
                return Err(
                    SerializationError::InvalidLimit {
                        field: "max_document_bytes",
                    },
                );
            }
        }

        if let Some(value) = self.max_field_bytes {
            if value == 0 {
                return Err(
                    SerializationError::InvalidLimit {
                        field: "max_field_bytes",
                    },
                );
            }
        }

        if let Some(value) = self.max_collection_elements {
            if value == 0 {
                return Err(
                    SerializationError::InvalidLimit {
                        field: "max_collection_elements",
                    },
                );
            }
        }

        if let Some(value) = self.max_nesting_depth {
            if value == 0 {
                return Err(
                    SerializationError::InvalidLimit {
                        field: "max_nesting_depth",
                    },
                );
            }
        }

        if let (
            Some(document),
            Some(field),
        ) = (
            self.max_document_bytes,
            self.max_field_bytes,
        ) {
            if field > document {
                return Err(
                    SerializationError::InvalidLimit {
                        field: "max_field_bytes",
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for SerializationLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Policy for unknown extension data encountered during deserialization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnknownExtensionPolicy {
    /// Reject an unknown extension.
    Reject,

    /// Preserve an unknown extension if the underlying AST representation
    /// supports lossless preservation.
    Preserve,

    /// Skip an unknown extension.
    ///
    /// This is explicitly lossy and must not be used for workflows requiring
    /// exact AST preservation.
    Skip,
}

impl Default for UnknownExtensionPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

/// Options controlling AST serialization/deserialization behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerializationOptions {
    /// Resource-consumption policy.
    pub limits: SerializationLimits,

    /// Unknown-extension policy.
    pub unknown_extensions: UnknownExtensionPolicy,
}

impl SerializationOptions {
    /// Creates unrestricted options with unknown extensions rejected.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: SerializationLimits::unlimited(),
            unknown_extensions: UnknownExtensionPolicy::Reject,
        }
    }

    /// Creates conservative options suitable for an untrusted boundary.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            limits: SerializationLimits::conservative(),
            unknown_extensions: UnknownExtensionPolicy::Reject,
        }
    }

    /// Validates the options.
    pub fn validate(self) -> Result<(), SerializationError> {
        self.limits.validate()
    }
}

impl Default for SerializationOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors produced by the AST serialization boundary.
///
/// This type intentionally describes serialization failures rather than
/// semantic/compiler failures.
#[derive(Debug)]
pub enum SerializationError {
    /// The caller supplied an invalid resource limit.
    InvalidLimit {
        /// Name of the invalid limit.
        field: &'static str,
    },

    /// The serialized representation exceeds an explicit resource policy.
    LimitExceeded {
        /// Resource being exceeded.
        resource: &'static str,

        /// Observed/requested value.
        value: u64,

        /// Permitted value.
        maximum: u64,
    },

    /// The serialized representation is malformed.
    InvalidDocument {
        /// Human-readable reason.
        message: String,
    },

    /// The serialized representation uses an unsupported version.
    UnsupportedVersion {
        /// Version found in the document.
        found: SerializationVersion,

        /// Version understood by this implementation.
        supported: SerializationVersion,
    },

    /// The serialization schema identifier is not recognized.
    InvalidSchema {
        /// Schema identifier found in the document.
        found: String,
    },

    /// The input was truncated.
    Truncated,

    /// Extra bytes/data were found after the expected document.
    TrailingData,

    /// A serialization operation failed.
    Codec {
        /// Underlying codec message.
        message: String,
    },

    /// The requested operation cannot preserve unknown extensions.
    UnknownExtension {
        /// Extension identifier.
        identifier: String,
    },

    /// A numeric conversion required by the host representation failed.
    NumericOverflow {
        /// Context of the failed conversion.
        context: &'static str,
    },

    /// An I/O operation failed while the caller uses an I/O-backed adapter.
    Io {
        /// Human-readable I/O error.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidLimit { field } => {
                write!(formatter, "invalid serialization limit: {field}")
            }

            Self::LimitExceeded {
                resource,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialization resource limit exceeded for {resource}: \
                     requested {value}, maximum {maximum}"
                )
            }

            Self::InvalidDocument { message } => {
                write!(formatter, "invalid AST serialization document: {message}")
            }

            Self::UnsupportedVersion {
                found,
                supported,
            } => {
                write!(
                    formatter,
                    "unsupported AST serialization version {found}; \
                     supported reader version is {supported}"
                )
            }

            Self::InvalidSchema { found } => {
                write!(
                    formatter,
                    "unsupported AST serialization schema: {found}"
                )
            }

            Self::Truncated => {
                write!(formatter, "truncated AST serialization document")
            }

            Self::TrailingData => {
                write!(
                    formatter,
                    "trailing data after AST serialization document"
                )
            }

            Self::Codec { message } => {
                write!(formatter, "AST serialization codec error: {message}")
            }

            Self::UnknownExtension { identifier } => {
                write!(
                    formatter,
                    "unknown AST extension: {identifier}"
                )
            }

            Self::NumericOverflow { context } => {
                write!(
                    formatter,
                    "numeric conversion overflow while {context}"
                )
            }

            Self::Io { message } => {
                write!(formatter, "AST serialization I/O error: {message}")
            }
        }
    }
}

impl std::error::Error for SerializationError {}

impl From<serde_json::Error> for SerializationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Codec {
            message: error.to_string(),
        }
    }
}

impl From<io::Error> for SerializationError {
    fn from(error: io::Error) -> Self {
        Self::Io {
            message: error.to_string(),
        }
    }
}

/// Result type used by the AST serialization boundary.
pub type SerializationResult<T> = Result<T, SerializationError>;

/// A self-describing serialized AST document.
///
/// The envelope is deliberately represented independently from the concrete
/// AST node type.
///
/// The actual byte representation is currently JSON-based. The envelope
/// therefore exists as a logical contract and can later be implemented by a
/// canonical binary codec without changing AST definitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedAst {
    bytes: Vec<u8>,
    version: SerializationVersion,
}

impl SerializedAst {
    /// Creates a serialized AST from already validated bytes.
    ///
    /// This constructor validates the resource policy and byte length but does
    /// not claim that arbitrary bytes constitute a valid AST document.
    ///
    /// Call [`deserialize`] when accepting untrusted serialized AST data.
    pub fn from_bytes(
        bytes: Vec<u8>,
        options: SerializationOptions,
    ) -> SerializationResult<Self> {
        options.validate()?;

        check_size(
            bytes.len(),
            options.limits.max_document_bytes,
            "document",
        )?;

        Ok(Self {
            bytes,
            version: CURRENT_SERIALIZATION_VERSION,
        })
    }

    /// Returns the exact serialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the serialized document length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the document contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the serialization format version associated with this artifact.
    #[must_use]
    pub const fn version(&self) -> SerializationVersion {
        self.version
    }

    /// Consumes the artifact and returns its exact bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl AsRef<[u8]> for SerializedAst {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Checks a host `usize` against an optional `u64` resource limit.
///
/// This function deliberately performs the conversion in the checked
/// direction. It never truncates a `u64` value into `usize`.
fn check_size(
    size: usize,
    maximum: Option<u64>,
    resource: &'static str,
) -> SerializationResult<()> {
    let size_u64 = u64::try_from(size).map_err(|_| {
        SerializationError::NumericOverflow {
            context: "converting serialized size to u64",
        }
    })?;

    if let Some(maximum) = maximum {
        if size_u64 > maximum {
            return Err(SerializationError::LimitExceeded {
                resource,
                value: size_u64,
                maximum,
            });
        }
    }

    Ok(())
}

/// Serializes an AST value into the current JSON interchange representation.
///
/// This is a persistence/interchange API. It is not, by itself, a cryptographic
/// canonicalization API.
///
/// For content-addressed identity, callers must use a separately specified
/// canonical serialization contract.
pub fn serialize<T>(
    value: &T,
) -> SerializationResult<Vec<u8>>
where
    T: Serialize,
{
    serialize_with_options(
        value,
        SerializationOptions::default(),
    )
}

/// Serializes an AST value with explicit resource policy.
///
/// The serialization remains independent of:
///
/// - machine size;
/// - hardware;
/// - quantum backend;
/// - compiler target.
pub fn serialize_with_options<T>(
    value: &T,
    options: SerializationOptions,
) -> SerializationResult<Vec<u8>>
where
    T: Serialize,
{
    options.validate()?;

    let bytes = serde_json::to_vec(value)?;

    check_size(
        bytes.len(),
        options.limits.max_document_bytes,
        "document",
    )?;

    Ok(bytes)
}

/// Serializes an AST value into an owned [`SerializedAst`] artifact.
pub fn serialize_artifact<T>(
    value: &T,
) -> SerializationResult<SerializedAst>
where
    T: Serialize,
{
    serialize_artifact_with_options(
        value,
        SerializationOptions::default(),
    )
}

/// Serializes an AST value into an owned artifact with explicit options.
pub fn serialize_artifact_with_options<T>(
    value: &T,
    options: SerializationOptions,
) -> SerializationResult<SerializedAst>
where
    T: Serialize,
{
    let bytes = serialize_with_options(value, options)?;

    Ok(SerializedAst {
        bytes,
        version: CURRENT_SERIALIZATION_VERSION,
    })
}

/// Deserializes an AST value from serialized bytes.
///
/// The caller should use explicit [`SerializationOptions`] when the bytes come
/// from an untrusted boundary.
pub fn deserialize<T>(
    bytes: &[u8],
) -> SerializationResult<T>
where
    T: DeserializeOwned,
{
    deserialize_with_options(
        bytes,
        SerializationOptions::default(),
    )
}

/// Deserializes an AST value using explicit resource and extension policy.
///
/// This function validates the configured document size before invoking the
/// underlying deserializer.
pub fn deserialize_with_options<T>(
    bytes: &[u8],
    options: SerializationOptions,
) -> SerializationResult<T>
where
    T: DeserializeOwned,
{
    options.validate()?;

    check_size(
        bytes.len(),
        options.limits.max_document_bytes,
        "document",
    )?;

    if bytes.is_empty() {
        return Err(SerializationError::Truncated);
    }

    let value = serde_json::from_slice(bytes)?;

    Ok(value)
}

/// Deserializes an owned [`SerializedAst`] artifact.
///
/// The artifact has already crossed the byte-size boundary, but semantic AST
/// reconstruction remains the responsibility of the underlying decoder.
pub fn deserialize_artifact<T>(
    artifact: &SerializedAst,
) -> SerializationResult<T>
where
    T: DeserializeOwned,
{
    deserialize(artifact.as_bytes())
}

/// Returns the current serialization format identifier.
///
/// This is intentionally a function rather than exposing implementation
/// details of future envelope types.
#[must_use]
pub const fn format_id() -> &'static str {
    SERIALIZATION_FORMAT_ID
}

/// Returns the current serialization format version.
#[must_use]
pub const fn format_version() -> SerializationVersion {
    CURRENT_SERIALIZATION_VERSION
}

/// Returns the JSON media type used by the current interchange codec.
#[must_use]
pub const fn media_type() -> &'static str {
    JSON_MEDIA_TYPE
}

/// Validates the public serialization configuration.
///
/// This is useful for compiler/session construction so configuration failures
/// are detected before compilation begins.
pub fn validate_options(
    options: SerializationOptions,
) -> SerializationResult<()> {
    options.validate()
}

/// Convenience conversion from a byte slice to an owned serialized artifact.
///
/// The bytes are copied because the artifact must own its serialized document.
pub fn artifact_from_slice(
    bytes: &[u8],
) -> SerializationResult<SerializedAst> {
    artifact_from_slice_with_options(
        bytes,
        SerializationOptions::default(),
    )
}

/// Convenience conversion from a byte slice using an explicit resource policy.
pub fn artifact_from_slice_with_options(
    bytes: &[u8],
    options: SerializationOptions,
) -> SerializationResult<SerializedAst> {
    options.validate()?;

    check_size(
        bytes.len(),
        options.limits.max_document_bytes,
        "document",
    )?;

    if bytes.is_empty() {
        return Err(SerializationError::Truncated);
    }

    Ok(SerializedAst {
        bytes: bytes.to_vec(),
        version: CURRENT_SERIALIZATION_VERSION,
    })
}

/// Performs a serialization round-trip.
///
/// This helper is primarily intended for:
///
/// - tests;
/// - compiler diagnostics;
/// - AST tooling;
/// - development verification.
///
/// It must not be used as a substitute for semantic validation.
pub fn round_trip<T>(
    value: &T,
) -> SerializationResult<T>
where
    T: Serialize + DeserializeOwned,
{
    round_trip_with_options(
        value,
        SerializationOptions::default(),
    )
}

/// Performs a serialization round-trip with explicit options.
pub fn round_trip_with_options<T>(
    value: &T,
    options: SerializationOptions,
) -> SerializationResult<T>
where
    T: Serialize + DeserializeOwned,
{
    let bytes = serialize_with_options(value, options)?;
    deserialize_with_options(&bytes, options)
}

/// Checks whether the current implementation can read a serialized format
/// version under the conservative compatibility rule.
#[must_use]
pub const fn supports_version(
    version: SerializationVersion,
) -> bool {
    CURRENT_SERIALIZATION_VERSION.supports(version)
}

/// Returns whether an explicit migration is required before interpreting a
/// serialized document using the current format.
///
/// Future versions are rejected rather than guessed.
#[must_use]
pub const fn requires_migration(
    version: SerializationVersion,
) -> bool {
    !supports_version(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
    struct TestAst {
        id: u64,
        name: String,
        values: Vec<u64>,
    }

    #[test]
    fn current_version_is_self_compatible() {
        assert!(supports_version(CURRENT_SERIALIZATION_VERSION));
        assert!(!requires_migration(CURRENT_SERIALIZATION_VERSION));
    }

    #[test]
    fn version_ordering_is_stable() {
        let old = SerializationVersion::new(1, 0, 0);
        let current = SerializationVersion::new(1, 1, 0);

        assert!(current.supports(old));
        assert!(!old.supports(current));
    }

    #[test]
    fn limits_accept_unrestricted_policy() {
        assert!(
            SerializationLimits::unlimited()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn invalid_zero_limit_is_rejected() {
        let limits = SerializationLimits::new(
            Some(0),
            None,
            None,
            None,
        );

        assert!(matches!(
            limits.validate(),
            Err(SerializationError::InvalidLimit {
                field: "max_document_bytes"
            })
        ));
    }

    #[test]
    fn field_limit_cannot_exceed_document_limit() {
        let limits = SerializationLimits::new(
            Some(10),
            Some(11),
            None,
            None,
        );

        assert!(matches!(
            limits.validate(),
            Err(SerializationError::InvalidLimit {
                field: "max_field_bytes"
            })
        ));
    }

    #[test]
    fn serialize_and_deserialize_round_trip() {
        let ast = TestAst {
            id: 1,
            name: "Zamani".to_owned(),
            values: vec![1, 2, 3, 5, 8],
        };

        let restored =
            round_trip(&ast).expect("round trip must succeed");

        assert_eq!(restored, ast);
    }

    #[test]
    fn serialization_is_deterministic_for_same_value() {
        let ast = TestAst {
            id: 42,
            name: "POCO-REAF".to_owned(),
            values: vec![1, 4, 9, 16],
        };

        let first =
            serialize(&ast).expect("first serialization must succeed");
        let second =
            serialize(&ast).expect("second serialization must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn document_limit_is_enforced_before_returning_bytes() {
        let ast = TestAst {
            id: 1,
            name: "Zamani".to_owned(),
            values: vec![1, 2, 3, 4],
        };

        let options = SerializationOptions {
            limits: SerializationLimits::new(
                Some(1),
                None,
                None,
                None,
            ),
            unknown_extensions:
                UnknownExtensionPolicy::Reject,
        };

        assert!(matches!(
            serialize_with_options(&ast, options),
            Err(SerializationError::LimitExceeded {
                resource: "document",
                ..
            })
        ));
    }

    #[test]
    fn empty_input_is_rejected() {
        let result: SerializationResult<TestAst> =
            deserialize(&[]);

        assert!(matches!(
            result,
            Err(SerializationError::Truncated)
        ));
    }

    #[test]
    fn serialized_artifact_borrows_exact_bytes() {
        let ast = TestAst {
            id: 7,
            name: "artifact".to_owned(),
            values: vec![7],
        };

        let artifact =
            serialize_artifact(&ast)
                .expect("artifact serialization must succeed");

        assert!(!artifact.is_empty());
        assert_eq!(
            artifact.as_bytes().len(),
            artifact.len()
        );
        assert_eq!(
            artifact.version(),
            CURRENT_SERIALIZATION_VERSION
        );
    }

    #[test]
    fn unknown_extension_policy_is_explicit() {
        assert_eq!(
            UnknownExtensionPolicy::default(),
            UnknownExtensionPolicy::Reject
        );
    }

    #[test]
    fn artifact_from_slice_copies_input() {
        let source = br#"{"id":1,"name":"x","values":[]}"#;

        let artifact =
            artifact_from_slice(source)
                .expect("artifact construction must succeed");

        assert_eq!(artifact.as_bytes(), source);
    }

    #[test]
    fn options_validate_successfully() {
        assert!(
            validate_options(
                SerializationOptions::conservative()
            )
            .is_ok()
        );
    }
}