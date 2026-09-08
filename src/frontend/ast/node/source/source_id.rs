//! Canonical source-unit identity infrastructure for the Zamani frontend AST.
//!
//! # Architectural role
//!
//! `SourceId` identifies one source unit in the native Zamani frontend.
//!
//! A source unit may represent:
//!
//! - a source file;
//! - an in-memory source buffer;
//! - a generated source unit;
//! - an imported source unit;
//! - a macro/generated expansion source;
//! - a virtual module;
//! - another source representation managed by the compiler's source-map layer.
//!
//! `SourceId` deliberately identifies the source *unit*, not its path,
//! contents, location, URI, filesystem inode, memory address, or execution
//! resource.
//!
//! The source-map layer is responsible for resolving a `SourceId` into source
//! metadata and source text.
//!
//! # Architectural position
//!
//! ```text
//!                         SOURCE INFRASTRUCTURE
//!
//! source registry / source map
//!             │
//!             ├───────────────┐
//!             │               │
//!             ▼               ▼
//!        SourceId          source text
//!             │               │
//!             │               ▼
//!             │          SourceOffset
//!             │               │
//!             └───────┬───────┘
//!                     ▼
//!                    Span
//!                     │
//!                     ▼
//!              SourceLocation
//!                     │
//!                     ▼
//!                 AST Node
//!                     │
//!                     ▼
//!             semantic analysis
//!                     │
//!                     ▼
//!                    ZUIR
//!                     │
//!                     ▼
//!              domain / target
//!                     │
//!                     ▼
//!                  hardware
//! ```
//!
//! `SourceId` therefore belongs to the foundational AST/source layer and must
//! remain independent of every downstream compilation domain.
//!
//! # POCO-REAF
//!
//! `SourceId` contains no information about:
//!
//! - quantum computers;
//! - qubit counts;
//! - logical qubits;
//! - physical qubits;
//! - hardware topology;
//! - gate sets;
//! - backend vendors;
//! - CPU architectures;
//! - GPU architectures;
//! - FPGA architectures;
//! - schedulers;
//! - routing;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime state;
//! - execution jobs;
//! - machine size.
//!
//! Consequently the same source identity abstraction can be used for tiny
//! programs and extremely large programs without changing its representation.
//!
//! "Infinity" in the Zamani POCO-REAF architecture means that the language and
//! AST do not impose an artificial finite source-count limit. Actual operation
//! remains bounded by the representable coordinate domain and resources
//! available to the compiler process.
//!
//! # Identity versus metadata
//!
//! `SourceId` is intentionally only an identity.
//!
//! It must NOT contain:
//!
//! - `String` paths;
//! - URLs;
//! - source contents;
//! - timestamps;
//! - hashes;
//! - line numbers;
//! - columns;
//! - byte offsets;
//! - file descriptors;
//! - operating-system handles;
//! - memory addresses.
//!
//! Those are separate concerns.
//!
//! This separation is important because source metadata can change while the
//! identity used by an individual compilation session remains stable.
//!
//! # Zero/default identity
//!
//! `0` is retained as a valid representable raw identifier because the current
//! Zamani source infrastructure and parser-facing code already use
//! `SourceId::from_raw(0)` as a neutral/default value.
//!
//! Therefore this module does NOT reserve zero and does NOT silently redefine
//! existing source-ID semantics.
//!
//! Code that requires an explicitly registered source must validate registration
//! through the source-map/registry layer rather than through `SourceId` itself.
//!
//! # Scalability
//!
//! The canonical representation is `u64`.
//!
//! This is deliberately independent of host `usize` width so serialized AST
//! data has a stable coordinate width across compiler hosts.
//!
//! No artificial constants such as:
//!
//! ```text
//! MAX_SOURCE_FILES
//! MAX_MODULES
//! MAX_SOURCES
//! ```
//!
//! are defined here.
//!
//! Operational limits belong in configurable compiler/source-map policy.
//!
//! # Determinism
//!
//! `SourceId` is a value type.
//!
//! It does not:
//!
//! - allocate IDs globally;
//! - use randomness;
//! - use wall-clock time;
//! - inspect memory addresses;
//! - access global mutable state;
//! - depend on thread scheduling;
//! - depend on hash-map iteration order.
//!
//! Allocation policy belongs to the source registry.
//!
//! This makes `SourceId` deterministic and suitable for reproducible
//! compilation, serialization, caching, diagnostics, and incremental
//! compilation.
//!
//! # Serialization
//!
//! Serialization represents the stable numeric identity only.
//!
//! The source registry/source-map layer is responsible for determining how a
//! serialized source ID is resolved in a particular compilation environment.
//!
//! A serialized `SourceId` therefore does not imply that the corresponding
//! source exists in the receiving process.
//!
//! # Error handling
//!
//! This type itself has no fallible constructor because every `u64` value is a
//! representable source identity.
//!
//! Fallible operations are provided only where conversion to a narrower
//! representation, such as `usize`, could overflow.
//!
//! Source registration, lookup, existence, path validation, source loading,
//! and source-map consistency are deliberately outside this type.
//!
//! # Thread safety
//!
//! `SourceId` contains only a `u64` and therefore has no internal mutable state.
//!
//! It is safe to copy, compare, hash, serialize, send, and share between
//! threads. Source registry synchronization remains the responsibility of the
//! registry implementation.
//!
//! # Security
//!
//! A `SourceId` must never be interpreted as:
//!
//! - a filesystem path;
//! - a pointer;
//! - a file descriptor;
//! - a process ID;
//! - an operating-system resource handle;
//! - an authorization token;
//! - a backend identifier.
//!
//! In particular, accepting an arbitrary raw `SourceId` must never grant access
//! to a source or resource. Authorization and source lookup belong to the
//! source registry/compiler session.
//!
//! # Dependency contract
//!
//! This file intentionally depends only on:
//!
//! - Rust standard-library facilities;
//! - Serde.
//!
//! It must never depend on:
//!
//! - parser implementation;
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend APIs.
//!
//! # Integration contract
//!
//! ```text
//! source_id.rs
//!      │
//!      ├──────────────► span.rs
//!      │                    │
//!      │                    ▼
//!      │              SourceLocation
//!      │                    │
//!      │                    ▼
//!      └──────────────► AST Node
//!                           │
//!                           ├── parser
//!                           ├── diagnostics
//!                           ├── validation
//!                           ├── visitors
//!                           ├── serialization
//!                           ├── semantic analysis
//!                           └── ZUIR provenance
//! ```
//!
//! `SourceId` is deliberately completed before `Span` and `SourceLocation`
//! depend on it. This prevents the foundational identity type from depending
//! on higher-level source-coordinate abstractions.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Edition 2021
//!
//! No nightly features are used.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Stable identity of one source unit in the Zamani frontend.
///
/// `SourceId` is intentionally opaque: callers can compare, copy, hash, and
/// serialize it, but the meaning of the underlying number is owned by the
/// source registry/source-map layer.
///
/// # Examples
///
/// ```
/// use zamani::frontend::ast::node::source::source_id::SourceId;
///
/// let source = SourceId::from_raw(42);
///
/// assert_eq!(source.as_raw(), 42);
/// assert!(!source.is_zero());
/// ```
///
/// The exact crate path in the example depends on the public crate name and
/// frontend module re-exports.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct SourceId(u64);

impl SourceId {
    /// Creates a source identity from its stable raw representation.
    ///
    /// This operation does not verify that the ID is registered.
    ///
    /// Registration/existence validation belongs to the source registry.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the stable raw representation.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns whether this is the neutral/default source identity.
    ///
    /// This does not mean that source ID zero is necessarily registered.
    /// Registry lookup must be used to determine whether it refers to an
    /// actual source.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether this source identity is non-zero.
    ///
    /// This is only a representation-level check. It does not prove that the
    /// source is registered.
    #[must_use]
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

    /// Returns the next representable source identity.
    ///
    /// This helper performs only arithmetic. It does not allocate or register
    /// the resulting identity.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Adds a raw identity offset using checked arithmetic.
    ///
    /// This is useful to deterministic ID-allocation infrastructure but does
    /// not itself perform allocation or registration.
    #[must_use]
    pub const fn checked_add(self, amount: u64) -> Option<Self> {
        match self.0.checked_add(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Subtracts a raw identity offset using checked arithmetic.
    ///
    /// This is useful for deterministic source-ID calculations without
    /// permitting integer underflow.
    #[must_use]
    pub const fn checked_sub(self, amount: u64) -> Option<Self> {
        match self.0.checked_sub(amount) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Converts a platform-sized integer into the canonical source identity.
    ///
    /// This conversion is checked so the canonical representation never
    /// silently truncates a platform value.
    #[must_use]
    pub fn try_from_usize(value: usize) -> Result<Self, SourceIdConversionError> {
        u64::try_from(value)
            .map(Self)
            .map_err(|_| SourceIdConversionError::UsizeOverflow)
    }

    /// Converts the source identity to `usize` when the identity is
    /// representable on the current compilation host.
    ///
    /// The canonical AST representation remains `u64`; this conversion is only
    /// for APIs whose platform representation is necessarily `usize`.
    #[must_use]
    pub fn try_as_usize(self) -> Result<usize, SourceIdConversionError> {
        usize::try_from(self.0).map_err(|_| SourceIdConversionError::UsizeOverflow)
    }
}

impl From<u64> for SourceId {
    /// Converts a `u64` into a source identity.
    ///
    /// Every `u64` is representable, so this conversion cannot fail.
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<SourceId> for u64 {
    /// Extracts the stable raw source identity.
    fn from(value: SourceId) -> Self {
        value.as_raw()
    }
}

impl TryFrom<usize> for SourceId {
    type Error = SourceIdConversionError;

    /// Converts a platform-sized integer without truncation.
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from_usize(value)
    }
}

impl TryFrom<SourceId> for usize {
    type Error = SourceIdConversionError;

    /// Converts to the host representation without truncation.
    fn try_from(value: SourceId) -> Result<Self, Self::Error> {
        value.try_as_usize()
    }
}

impl fmt::Display for SourceId {
    /// Formats the identity in an unambiguous diagnostic form.
    ///
    /// Example:
    ///
    /// ```text
    /// source#42
    /// ```
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "source#{}", self.0)
    }
}

/// Errors produced when converting [`SourceId`] to or from a narrower
/// platform-dependent representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SourceIdConversionError {
    /// The source identity cannot be represented by the destination/source
    /// platform-sized integer.
    UsizeOverflow,
}

impl fmt::Display for SourceIdConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UsizeOverflow => {
                formatter.write_str(
                    "source ID cannot be represented by the platform usize type",
                )
            }
        }
    }
}

impl std::error::Error for SourceIdConversionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_round_trip_is_exact() {
        let id = SourceId::from_raw(42);

        assert_eq!(id.as_raw(), 42);
        assert_eq!(u64::from(id), 42);
    }

    #[test]
    fn zero_is_explicitly_supported() {
        let id = SourceId::from_raw(0);

        assert!(id.is_zero());
        assert!(!id.is_non_zero());
    }

    #[test]
    fn non_zero_identity_is_detected() {
        let id = SourceId::from_raw(1);

        assert!(!id.is_zero());
        assert!(id.is_non_zero());
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(SourceId::default(), SourceId::from_raw(0));
    }

    #[test]
    fn checked_arithmetic_does_not_overflow() {
        let id = SourceId::from_raw(u64::MAX);

        assert_eq!(id.checked_next(), None);
        assert_eq!(id.checked_add(1), None);
        assert_eq!(
            SourceId::from_raw(10).checked_add(5),
            Some(SourceId::from_raw(15))
        );
        assert_eq!(
            SourceId::from_raw(10).checked_sub(5),
            Some(SourceId::from_raw(5))
        );
        assert_eq!(SourceId::from_raw(0).checked_sub(1), None);
    }

    #[test]
    fn display_is_stable() {
        assert_eq!(
            SourceId::from_raw(42).to_string(),
            "source#42"
        );
    }

    #[test]
    fn ordering_is_numeric_and_deterministic() {
        let low = SourceId::from_raw(1);
        let high = SourceId::from_raw(2);

        assert!(low < high);
        assert_eq!(low, SourceId::from_raw(1));
    }

    #[test]
    fn usize_conversion_round_trips_when_representable() {
        let value = 123usize;

        let id = SourceId::try_from_usize(value)
            .expect("small usize must fit in u64");

        let recovered = id
            .try_as_usize()
            .expect("small source ID must fit in usize");

        assert_eq!(recovered, value);
    }

    #[test]
    fn u64_conversion_never_truncates() {
        let id = SourceId::from_raw(u64::MAX);

        assert_eq!(id.as_raw(), u64::MAX);
    }

    #[test]
    fn serde_round_trip_preserves_identity() {
        let id = SourceId::from_raw(987_654_321);

        let encoded =
            serde_json::to_string(&id).expect("SourceId must serialize");

        let decoded: SourceId =
            serde_json::from_str(&encoded).expect("SourceId must deserialize");

        assert_eq!(decoded, id);
    }
}