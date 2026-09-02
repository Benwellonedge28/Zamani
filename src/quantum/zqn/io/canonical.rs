//! Zamani Quantum Noise (ZQN) — Canonical Representation.
//!
//! # Ownership
//!
//! This file owns the canonical semantic representation of ZQN
//! serialization values.
//!
//! It owns:
//!
//! - deterministic JSON object-key ordering;
//! - recursive canonicalization of JSON values;
//! - preservation of array ordering;
//! - canonical UTF-8 JSON encoding;
//! - canonical byte generation;
//! - canonical SHA-256 digest generation;
//! - canonical-byte validation;
//! - canonicalization resource policies;
//! - canonical representation equality;
//! - canonical representation invariants;
//! - canonical representation documentation.
//!
//! It does NOT own:
//!
//! - binary ZQN document framing;
//! - serialization format headers;
//! - payload lengths;
//! - serialization format versions;
//! - schema versions;
//! - semantic ZQN versioning;
//! - compatibility migrations;
//! - quantum channels;
//! - faults;
//! - noise models;
//! - calibration;
//! - characterization;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - runtime execution;
//! - vendor APIs;
//! - quantum resource identity.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                 ZQN semantic object
//!                         │
//!                         ▼
//!                   Serde encoding
//!                         │
//!                         ▼
//!                 serde_json::Value
//!                         │
//!                         ▼
//!                 canonicalize_value
//!                         │
//!                         ▼
//!              deterministic JSON bytes
//!                         │
//!              ┌──────────┴──────────┐
//!              ▼                     ▼
//!        SHA-256 digest          serialization.rs
//!                                      │
//!                                      ▼
//!                              ZQN binary envelope
//! ```
//!
//! The dependency direction is deliberately one-way:
//!
//! ```text
//! canonical.rs
//!      ▲
//!      │
//! serialization.rs
//!      │
//!      ├── deserialization.rs
//!      └── compatibility.rs
//! ```
//!
//! `canonical.rs` must not depend on `serialization.rs`.
//!
//! # Why canonicalization is separate from serialization
//!
//! Binary serialization answers:
//!
//! > How is a ZQN artifact framed for storage or transport?
//!
//! Canonicalization answers:
//!
//! > Which exact bytes represent a semantic value deterministically?
//!
//! These are different concerns.
//!
//! Canonical bytes are useful for:
//!
//! - content addressing;
//! - reproducibility;
//! - cache keys;
//! - provenance;
//! - model identity;
//! - calibration identity;
//! - distributed execution;
//! - regression tests;
//! - integrity verification;
//! - deterministic artifact comparison.
//!
//! Binary framing must therefore consume canonical bytes rather than redefine
//! canonicalization itself.
//!
//! # Canonicalization contract
//!
//! ZQN canonical JSON follows these rules:
//!
//! 1. JSON object members are ordered lexicographically by their UTF-8 key
//!    bytes.
//! 2. JSON arrays preserve their original order.
//! 3. JSON scalar values are not semantically transformed.
//! 4. JSON strings are encoded as valid UTF-8 JSON strings by `serde_json`.
//! 5. Whitespace outside JSON strings is removed by canonical encoding.
//! 6. Object-key ordering is recursive.
//! 7. Nested arrays and objects are canonicalized recursively.
//! 8. No semantic sorting of arrays is performed.
//! 9. No quantum-resource ordering is invented here.
//! 10. No qubit IDs are interpreted here.
//! 11. No floating-point approximation is introduced here.
//! 12. Non-finite floating-point values are rejected by the Serde/JSON layer.
//!
//! This means:
//!
//! ```text
//! object key order
//!     -> canonicalized
//!
//! array order
//!     -> preserved
//!
//! semantic array contents
//!     -> never reordered
//! ```
//!
//! # Important numeric rule
//!
//! Canonicalization does not silently turn numerically different JSON
//! representations into one another.
//!
//! For example, a semantic type whose Serde representation distinguishes
//! different numeric representations must retain that distinction unless its
//! owning semantic type explicitly defines a normalization before reaching
//! this layer.
//!
//! Canonicalization is therefore a representation-level operation, not a
//! quantum mathematical simplifier.
//!
//! # Quantum identity boundary
//!
//! This module deliberately does not define a qubit identifier.
//!
//! If a canonicalized ZQN object contains quantum resource identities, those
//! identities are owned by the semantic module that defines the object.
//!
//! Where qubit identities are required, the canonical repository boundary is:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module treats them as serialized data and never creates replacement
//! identifiers.
//!
//! # Scalability
//!
//! There is no semantic upper bound on:
//!
//! - qubits;
//! - qudits;
//! - modes;
//! - channels;
//! - operations;
//! - faults;
//! - calibration entries;
//! - noise locations;
//! - circuit depth;
//! - distributed resources;
//! - quantum technologies.
//!
//! Canonicalization complexity is determined by the supplied representation.
//!
//! Object canonicalization is O(n log n) in the number of members of an
//! individual JSON object because keys must be deterministically ordered.
//!
//! Arrays remain O(n) because their order is preserved.
//!
//! Recursive traversal is O(n) in the number of JSON nodes, excluding object
//! key sorting.
//!
//! This module does not impose a fixed maximum quantum-system size.
//!
//! Resource policies are explicit and caller-controlled.
//!
//! # Resource safety
//!
//! Canonicalization can allocate because converting arbitrary Serde values into
//! canonical JSON requires materializing JSON representation.
//!
//! This module therefore provides explicit limits for callers that need
//! defensive canonicalization.
//!
//! `CanonicalLimits::unbounded()` removes ZQN-imposed finite limits, but does
//! not remove operating-system, allocator, address-space, or physical-memory
//! limits.
//!
//! There is no hidden global allocation policy.
//!
//! # Determinism
//!
//! For the same Serde semantic representation:
//!
//! ```text
//! same input
//!     + same canonicalization rules
//!     = same canonical JSON bytes
//!     = same SHA-256 digest
//! ```
//!
//! Canonicalization is stateless and contains no random behavior.
//!
//! It is therefore safe to use concurrently.
//!
//! # Security
//!
//! Canonicalization must not:
//!
//! - execute code;
//! - access the network;
//! - access the filesystem;
//! - invoke vendor APIs;
//! - use global mutable state;
//! - invoke `unsafe`;
//! - silently normalize invalid values;
//! - silently drop object members;
//! - silently reorder arrays.
//!
//! Canonical-byte validation additionally allows callers to reject noncanonical
//! input instead of silently rewriting it.
//!
//! # Serialization boundary
//!
//! `serialization.rs` owns:
//!
//! - ZQN magic;
//! - format version;
//! - payload length;
//! - digest field placement;
//! - binary envelope;
//! - reader/writer framing.
//!
//! It must call this module for canonical payload bytes.
//!
//! It must not maintain a second canonicalization algorithm.
//!
//! # Deserialization boundary
//!
//! `deserialization.rs` should:
//!
//! 1. validate the binary envelope;
//! 2. extract canonical payload bytes;
//! 3. verify the stored digest;
//! 4. parse the semantic payload;
//! 5. optionally require canonical payload bytes;
//! 6. deserialize into the requested semantic type.
//!
//! It should call this module for canonical-byte validation instead of
//! duplicating object-ordering logic.
//!
//! # Compatibility boundary
//!
//! Canonicalization does not decide whether two schema versions are compatible.
//!
//! That responsibility belongs to:
//!
//! `io::compatibility`
//!
//! Canonicalization must not perform migrations.
//!
//! # Provenance boundary
//!
//! The digest returned by [`canonical_digest`] can be recorded by
//! `core::provenance` as an artifact identity.
//!
//! A digest identifies bytes.
//!
//! It does not establish authorship, authorization, or authenticity.
//!
//! # Cryptographic boundary
//!
//! SHA-256 is used here as a deterministic content digest because the existing
//! Zamani workspace already provides the `sha2` dependency.
//!
//! A digest provides integrity/content identity.
//!
//! It does not provide authentication.
//!
//! Signatures belong to the security/artifact layer.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. canonical object ordering is defined here;
//! 2. array ordering is preserved;
//! 3. canonical encoding is deterministic;
//! 4. canonical hashing is deterministic;
//! 5. no binary framing is implemented here;
//! 6. no semantic quantum type is implemented here;
//! 7. no vendor dependency exists;
//! 8. no qubit identifier is duplicated;
//! 9. no machine-size constant exists;
//! 10. resource limits are explicit;
//! 11. malformed values return errors;
//! 12. canonical validation can distinguish canonical from merely valid JSON;
//! 13. the implementation is thread-safe because it has no mutable global state;
//! 14. adding a new ZQN semantic type does not require modifying this file;
//! 15. serialization consumes this file rather than duplicating it;
//! 16. deserialization consumes this file rather than duplicating it;
//! 17. compatibility remains outside this file.
//!
//! # External architecture alignment
//!
//! The separation of canonicalization from target-specific execution is
//! consistent with the broader quantum compiler architecture used by QIR,
//! which separates language-independent representation from backend profiles
//! and quantum instruction sets. QIR explicitly supports dynamic allocation
//! capabilities so representations need not encode one fixed hardware size.
//! 3
//!
//! The separation also follows compiler canonicalization practice: canonical
//! transformations should preserve semantics and should not be relied upon as
//! a correctness requirement of unrelated passes. 4
//!
//! # No unsafe
//!
//! This module explicitly forbids unsafe Rust.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

// =============================================================================
// Constants
// =============================================================================

/// SHA-256 digest length in bytes.
pub const DIGEST_LEN: usize = 32;

// =============================================================================
// Resource policy
// =============================================================================

/// Resource limits for canonicalization and canonical-byte validation.
///
/// These limits are security/resource policies, not quantum-system limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalLimits {
    /// Maximum number of JSON nodes traversed.
    pub max_nodes: u64,

    /// Maximum number of members in one JSON object.
    pub max_object_members: u64,

    /// Maximum number of elements in one JSON array.
    pub max_array_elements: u64,

    /// Maximum UTF-8 byte length of one JSON string.
    pub max_string_bytes: u64,

    /// Maximum recursive JSON depth.
    pub max_nesting_depth: u64,
}

impl CanonicalLimits {
    /// Creates an explicit canonicalization policy.
    #[must_use]
    pub const fn new(
        max_nodes: u64,
        max_object_members: u64,
        max_array_elements: u64,
        max_string_bytes: u64,
        max_nesting_depth: u64,
    ) -> Self {
        Self {
            max_nodes,
            max_object_members,
            max_array_elements,
            max_string_bytes,
            max_nesting_depth,
        }
    }

    /// Returns a conservative policy suitable for ordinary untrusted data.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_nodes: 64 * 1024 * 1024,
            max_object_members: 16 * 1024 * 1024,
            max_array_elements: 16 * 1024 * 1024,
            max_string_bytes: 16 * 1024 * 1024,
            max_nesting_depth: 4096,
        }
    }

    /// Removes ZQN-imposed finite canonicalization limits.
    ///
    /// The process remains constrained by actual available resources.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_nodes: u64::MAX,
            max_object_members: u64::MAX,
            max_array_elements: u64::MAX,
            max_string_bytes: u64::MAX,
            max_nesting_depth: u64::MAX,
        }
    }

    /// Validates the policy itself.
    pub fn validate(self) -> Result<(), CanonicalError> {
        if self.max_nodes == 0 {
            return Err(CanonicalError::InvalidLimits {
                field: "max_nodes",
            });
        }

        if self.max_object_members == 0 {
            return Err(CanonicalError::InvalidLimits {
                field: "max_object_members",
            });
        }

        if self.max_array_elements == 0 {
            return Err(CanonicalError::InvalidLimits {
                field: "max_array_elements",
            });
        }

        if self.max_string_bytes == 0 {
            return Err(CanonicalError::InvalidLimits {
                field: "max_string_bytes",
            });
        }

        if self.max_nesting_depth == 0 {
            return Err(CanonicalError::InvalidLimits {
                field: "max_nesting_depth",
            });
        }

        Ok(())
    }
}

impl Default for CanonicalLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by canonical representation operations.
#[derive(Debug)]
pub enum CanonicalError {
    /// The caller supplied an invalid resource policy.
    InvalidLimits {
        /// Invalid policy field.
        field: &'static str,
    },

    /// The canonicalization traversal exceeded its node budget.
    NodeLimitExceeded {
        /// Number of nodes requested.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// An object contains too many members.
    ObjectMemberLimitExceeded {
        /// Number of members requested.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// An array contains too many elements.
    ArrayElementLimitExceeded {
        /// Number of elements requested.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// A JSON string is too large.
    StringLimitExceeded {
        /// String byte length.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// Canonical nesting is too deep.
    NestingLimitExceeded {
        /// Requested nesting depth.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// Serde could not convert a value into JSON.
    JsonSerialization {
        /// Human-readable error.
        message: String,
    },

    /// JSON could not be encoded as canonical bytes.
    JsonEncoding {
        /// Human-readable error.
        message: String,
    },

    /// Supplied bytes are valid JSON but are not canonical ZQN JSON.
    NonCanonicalBytes,

    /// A checked resource counter overflowed.
    CounterOverflow {
        /// Counter name.
        counter: &'static str,
    },
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits { field } => {
                write!(formatter, "invalid canonicalization limit `{field}`")
            }
            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "canonicalization node limit exceeded: {requested} > {maximum}"
            ),
            Self::ObjectMemberLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "canonicalization object-member limit exceeded: {requested} > {maximum}"
            ),
            Self::ArrayElementLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "canonicalization array-element limit exceeded: {requested} > {maximum}"
            ),
            Self::StringLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "canonicalization string limit exceeded: {requested} > {maximum}"
            ),
            Self::NestingLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "canonicalization nesting limit exceeded: {requested} > {maximum}"
            ),
            Self::JsonSerialization { message } => {
                write!(formatter, "JSON value serialization failed: {message}")
            }
            Self::JsonEncoding { message } => {
                write!(formatter, "canonical JSON encoding failed: {message}")
            }
            Self::NonCanonicalBytes => {
                write!(formatter, "input bytes are not canonical ZQN JSON")
            }
            Self::CounterOverflow { counter } => {
                write!(formatter, "canonicalization counter overflow: {counter}")
            }
        }
    }
}

impl std::error::Error for CanonicalError {}

// =============================================================================
// Canonicalization
// =============================================================================

/// Recursively canonicalizes a JSON value.
///
/// Object members are sorted lexicographically by key.
///
/// Arrays retain their original order.
///
/// This function is infallible with respect to resource limits because it
/// intentionally does not impose hidden limits. Use
/// [`canonicalize_value_with_limits`] when processing data under an explicit
/// resource policy.
#[must_use]
pub fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries: Vec<(String, Value)> = object.into_iter().collect();

            entries.sort_by(|left, right| left.0.cmp(&right.0));

            let mut canonical = Map::with_capacity(entries.len());

            for (key, value) in entries {
                canonical.insert(key, canonicalize_value(value));
            }

            Value::Object(canonical)
        }

        Value::Array(values) => {
            Value::Array(values.into_iter().map(canonicalize_value).collect())
        }

        scalar => scalar,
    }
}

/// Canonicalizes a JSON value while enforcing explicit resource limits.
pub fn canonicalize_value_with_limits(
    value: Value,
    limits: CanonicalLimits,
) -> Result<Value, CanonicalError> {
    limits.validate()?;

    let mut state = TraversalState::new(limits);
    canonicalize_inner(value, 0, &mut state)
}

fn canonicalize_inner(
    value: Value,
    depth: u64,
    state: &mut TraversalState,
) -> Result<Value, CanonicalError> {
    let current_depth = depth
        .checked_add(1)
        .ok_or(CanonicalError::CounterOverflow {
            counter: "depth",
        })?;

    if current_depth > state.limits.max_nesting_depth {
        return Err(CanonicalError::NestingLimitExceeded {
            requested: current_depth,
            maximum: state.limits.max_nesting_depth,
        });
    }

    state.increment_nodes()?;

    match value {
        Value::Object(object) => {
            let member_count =
                u64::try_from(object.len()).map_err(|_| CanonicalError::CounterOverflow {
                    counter: "object_members",
                })?;

            if member_count > state.limits.max_object_members {
                return Err(CanonicalError::ObjectMemberLimitExceeded {
                    requested: member_count,
                    maximum: state.limits.max_object_members,
                });
            }

            let mut entries: Vec<(String, Value)> = object.into_iter().collect();

            entries.sort_by(|left, right| left.0.cmp(&right.0));

            let mut canonical = Map::with_capacity(entries.len());

            for (key, value) in entries {
                check_string_len(&key, state.limits.max_string_bytes)?;

                let canonical_value =
                    canonicalize_inner(value, current_depth, state)?;

                canonical.insert(key, canonical_value);
            }

            Ok(Value::Object(canonical))
        }

        Value::Array(values) => {
            let element_count =
                u64::try_from(values.len()).map_err(|_| CanonicalError::CounterOverflow {
                    counter: "array_elements",
                })?;

            if element_count > state.limits.max_array_elements {
                return Err(CanonicalError::ArrayElementLimitExceeded {
                    requested: element_count,
                    maximum: state.limits.max_array_elements,
                });
            }

            let mut canonical = Vec::with_capacity(values.len());

            for value in values {
                canonical.push(canonicalize_inner(
                    value,
                    current_depth,
                    state,
                )?);
            }

            Ok(Value::Array(canonical))
        }

        Value::String(string) => {
            check_string_len(&string, state.limits.max_string_bytes)?;
            Ok(Value::String(string))
        }

        scalar => Ok(scalar),
    }
}

fn check_string_len(string: &str, maximum: u64) -> Result<(), CanonicalError> {
    let length =
        u64::try_from(string.len()).map_err(|_| CanonicalError::CounterOverflow {
            counter: "string_bytes",
        })?;

    if length > maximum {
        return Err(CanonicalError::StringLimitExceeded {
            requested: length,
            maximum,
        });
    }

    Ok(())
}

struct TraversalState {
    limits: CanonicalLimits,
    nodes: u64,
}

impl TraversalState {
    fn new(limits: CanonicalLimits) -> Self {
        Self { limits, nodes: 0 }
    }

    fn increment_nodes(&mut self) -> Result<(), CanonicalError> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or(CanonicalError::CounterOverflow {
                counter: "nodes",
            })?;

        if self.nodes > self.limits.max_nodes {
            return Err(CanonicalError::NodeLimitExceeded {
                requested: self.nodes,
                maximum: self.limits.max_nodes,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Serde-facing canonicalization
// =============================================================================

/// Converts any Serde-serializable value into a canonical JSON value.
///
/// This is the main generic entry point for semantic ZQN objects.
pub fn to_canonical_value<T>(
    value: &T,
) -> Result<Value, CanonicalError>
where
    T: Serialize,
{
    let json = serde_json::to_value(value).map_err(|error| {
        CanonicalError::JsonSerialization {
            message: error.to_string(),
        }
    })?;

    Ok(canonicalize_value(json))
}

/// Converts a Serde value into a canonical JSON value under explicit limits.
pub fn to_canonical_value_with_limits<T>(
    value: &T,
    limits: CanonicalLimits,
) -> Result<Value, CanonicalError>
where
    T: Serialize,
{
    let json = serde_json::to_value(value).map_err(|error| {
        CanonicalError::JsonSerialization {
            message: error.to_string(),
        }
    })?;

    canonicalize_value_with_limits(json, limits)
}

/// Returns canonical compact JSON bytes for a Serde value.
///
/// The resulting bytes are the canonical semantic payload used by
/// `serialization.rs`.
pub fn serialize_canonical<T>(
    value: &T,
) -> Result<Vec<u8>, CanonicalError>
where
    T: Serialize,
{
    let canonical = to_canonical_value(value)?;

    serde_json::to_vec(&canonical).map_err(|error| {
        CanonicalError::JsonEncoding {
            message: error.to_string(),
        }
    })
}

/// Returns canonical compact JSON bytes under explicit limits.
pub fn serialize_canonical_with_limits<T>(
    value: &T,
    limits: CanonicalLimits,
) -> Result<Vec<u8>, CanonicalError>
where
    T: Serialize,
{
    let canonical = to_canonical_value_with_limits(value, limits)?;

    serde_json::to_vec(&canonical).map_err(|error| {
        CanonicalError::JsonEncoding {
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Digest
// =============================================================================

/// Computes SHA-256 over canonical bytes.
#[must_use]
pub fn digest_bytes(bytes: &[u8]) -> [u8; DIGEST_LEN] {
    let digest = Sha256::digest(bytes);

    let mut output = [0_u8; DIGEST_LEN];
    output.copy_from_slice(&digest);

    output
}

/// Computes the SHA-256 digest of a canonical JSON value.
pub fn canonical_digest<T>(
    value: &T,
) -> Result<[u8; DIGEST_LEN], CanonicalError>
where
    T: Serialize,
{
    let bytes = serialize_canonical(value)?;
    Ok(digest_bytes(&bytes))
}

/// Computes the digest of a value using explicit canonicalization limits.
pub fn canonical_digest_with_limits<T>(
    value: &T,
    limits: CanonicalLimits,
) -> Result<[u8; DIGEST_LEN], CanonicalError>
where
    T: Serialize,
{
    let bytes = serialize_canonical_with_limits(value, limits)?;
    Ok(digest_bytes(&bytes))
}

// =============================================================================
// Canonical-byte validation
// =============================================================================

/// Parses JSON bytes and verifies that they are already canonical.
///
/// This function does not silently rewrite the input.
///
/// A JSON document is accepted only when:
//!
//! 1. it is valid JSON;
//! 2. its semantic value can be canonicalized;
//! 3. re-encoding the canonical value produces exactly the supplied bytes.
pub fn validate_canonical_bytes(
    bytes: &[u8],
) -> Result<(), CanonicalError> {
    validate_canonical_bytes_with_limits(bytes, CanonicalLimits::unbounded())
}

/// Validates canonical JSON bytes under explicit resource limits.
pub fn validate_canonical_bytes_with_limits(
    bytes: &[u8],
    limits: CanonicalLimits,
) -> Result<(), CanonicalError> {
    limits.validate()?;

    let value: Value =
        serde_json::from_slice(bytes).map_err(|error| CanonicalError::JsonEncoding {
            message: error.to_string(),
        })?;

    let canonical = canonicalize_value_with_limits(value, limits)?;

    let canonical_bytes =
        serde_json::to_vec(&canonical).map_err(|error| CanonicalError::JsonEncoding {
            message: error.to_string(),
        })?;

    if canonical_bytes != bytes {
        return Err(CanonicalError::NonCanonicalBytes);
    }

    Ok(())
}

/// Returns `true` if bytes are valid canonical ZQN JSON.
///
/// This is intentionally a convenience predicate; callers needing the exact
/// reason should use [`validate_canonical_bytes`].
#[must_use]
pub fn is_canonical_bytes(bytes: &[u8]) -> bool {
    validate_canonical_bytes(bytes).is_ok()
}

// =============================================================================
// Canonical equality
// =============================================================================

/// Compares two Serde values by their canonical representation.
///
/// This does not claim mathematical equivalence of arbitrary quantum objects.
/// It only establishes equality of their canonical serialized representation.
pub fn canonical_eq<T, U>(
    left: &T,
    right: &U,
) -> Result<bool, CanonicalError>
where
    T: Serialize,
    U: Serialize,
{
    let left_bytes = serialize_canonical(left)?;
    let right_bytes = serialize_canonical(right)?;

    Ok(left_bytes == right_bytes)
}

/// Compares two Serde values by their canonical digest.
///
/// This is useful for artifact identity but should not be used as a substitute
/// for byte comparison when cryptographic collision resistance is not an
/// acceptable assumption for the caller.
pub fn canonical_digest_eq<T, U>(
    left: &T,
    right: &U,
) -> Result<bool, CanonicalError>
where
    T: Serialize,
    U: Serialize,
{
    Ok(canonical_digest(left)? == canonical_digest(right)?)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn object_keys_are_sorted() {
        let value = json!({
            "z": 1,
            "a": 2,
            "m": 3
        });

        let bytes =
            serialize_canonical(&value).expect("canonical serialization must succeed");

        assert_eq!(
            bytes,
            br#"{"a":2,"m":3,"z":1}"#
        );
    }

    #[test]
    fn nested_objects_are_sorted() {
        let value = json!({
            "outer": {
                "z": 1,
                "a": 2
            }
        });

        let bytes =
            serialize_canonical(&value).expect("canonical serialization must succeed");

        assert_eq!(
            bytes,
            br#"{"outer":{"a":2,"z":1}}"#
        );
    }

    #[test]
    fn arrays_are_not_reordered() {
        let value = json!({
            "values": [3, 1, 2]
        });

        let bytes =
            serialize_canonical(&value).expect("canonical serialization must succeed");

        assert_eq!(
            bytes,
            br#"{"values":[3,1,2]}"#
        );
    }

    #[test]
    fn canonicalization_is_idempotent() {
        let value = json!({
            "z": {
                "b": 2,
                "a": 1
            },
            "a": [3, 2, 1]
        });

        let first =
            canonicalize_value(value.clone());

        let second =
            canonicalize_value(first.clone());

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_bytes_are_stable() {
        let value = json!({
            "z": 1,
            "a": 2,
            "nested": {
                "y": 3,
                "x": 4
            }
        });

        let first =
            serialize_canonical(&value).expect("first serialization must succeed");

        let second =
            serialize_canonical(&value).expect("second serialization must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_digest_is_stable() {
        let value = json!({
            "z": 1,
            "a": 2
        });

        let first =
            canonical_digest(&value).expect("digest must succeed");

        let second =
            canonical_digest(&value).expect("digest must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn different_object_insertion_order_has_same_canonical_bytes() {
        let first = json!({
            "a": 1,
            "b": 2,
            "c": 3
        });

        let second = json!({
            "c": 3,
            "a": 1,
            "b": 2
        });

        assert_eq!(
            serialize_canonical(&first).unwrap(),
            serialize_canonical(&second).unwrap()
        );
    }

    #[test]
    fn array_order_changes_canonical_bytes() {
        let first = json!([1, 2, 3]);
        let second = json!([3, 2, 1]);

        assert_ne!(
            serialize_canonical(&first).unwrap(),
            serialize_canonical(&second).unwrap()
        );
    }

    #[test]
    fn canonical_bytes_validate() {
        let bytes = br#"{"a":1,"b":{"c":2}}"#;

        assert!(
            validate_canonical_bytes(bytes).is_ok()
        );
    }

    #[test]
    fn noncanonical_object_order_is_rejected() {
        let bytes = br#"{"b":2,"a":1}"#;

        assert_eq!(
            validate_canonical_bytes(bytes)
                .expect_err("noncanonical bytes must fail")
                .to_string(),
            "input bytes are not canonical ZQN JSON"
        );
    }

    #[test]
    fn noncanonical_whitespace_is_rejected() {
        let bytes = br#"{ "a": 1 }"#;

        assert_eq!(
            validate_canonical_bytes(bytes)
                .expect_err("whitespace must be rejected")
                .to_string(),
            "input bytes are not canonical ZQN JSON"
        );
    }

    #[test]
    fn canonical_arrays_remain_semantic() {
        let first = br#"{"items":[1,2,3]}"#;
        let second = br#"{"items":[3,2,1]}"#;

        assert!(validate_canonical_bytes(first).is_ok());
        assert!(validate_canonical_bytes(second).is_ok());
        assert_ne!(first, second);
    }

    #[test]
    fn node_limit_is_enforced() {
        let value = json!({
            "a": {
                "b": {
                    "c": 1
                }
            }
        });

        let limits = CanonicalLimits::new(
            2,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
        );

        let result =
            canonicalize_value_with_limits(value, limits);

        assert!(matches!(
            result,
            Err(CanonicalError::NodeLimitExceeded { .. })
        ));
    }

    #[test]
    fn object_limit_is_enforced() {
        let value = json!({
            "a": 1,
            "b": 2,
            "c": 3
        });

        let limits = CanonicalLimits::new(
            u64::MAX,
            2,
            u64::MAX,
            u64::MAX,
            u64::MAX,
        );

        let result =
            canonicalize_value_with_limits(value, limits);

        assert!(matches!(
            result,
            Err(CanonicalError::ObjectMemberLimitExceeded { .. })
        ));
    }

    #[test]
    fn array_limit_is_enforced() {
        let value = json!([1, 2, 3]);

        let limits = CanonicalLimits::new(
            u64::MAX,
            u64::MAX,
            2,
            u64::MAX,
            u64::MAX,
        );

        let result =
            canonicalize_value_with_limits(value, limits);

        assert!(matches!(
            result,
            Err(CanonicalError::ArrayElementLimitExceeded { .. })
        ));
    }

    #[test]
    fn string_limit_is_enforced() {
        let value = json!({
            "value": "abcdef"
        });

        let limits = CanonicalLimits::new(
            u64::MAX,
            u64::MAX,
            u64::MAX,
            3,
            u64::MAX,
        );

        let result =
            canonicalize_value_with_limits(value, limits);

        assert!(matches!(
            result,
            Err(CanonicalError::StringLimitExceeded { .. })
        ));
    }

    #[test]
    fn depth_limit_is_enforced() {
        let value = json!({
            "a": {
                "b": {
                    "c": 1
                }
            }
        });

        let limits = CanonicalLimits::new(
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            2,
        );

        let result =
            canonicalize_value_with_limits(value, limits);

        assert!(matches!(
            result,
            Err(CanonicalError::NestingLimitExceeded { .. })
        ));
    }

    #[test]
    fn btreemap_serialization_is_stable() {
        let mut value = BTreeMap::new();

        value.insert("z", 1_u64);
        value.insert("a", 2_u64);
        value.insert("m", 3_u64);

        let bytes =
            serialize_canonical(&value)
                .expect("BTreeMap serialization must succeed");

        assert_eq!(
            bytes,
            br#"{"a":2,"m":3,"z":1}"#
        );
    }

    #[derive(Debug, Serialize)]
    struct Sample {
        z: u64,
        a: u64,
        nested: Vec<u64>,
    }

    #[test]
    fn generic_serde_values_are_supported() {
        let value = Sample {
            z: 10,
            a: 20,
            nested: vec![3, 2, 1],
        };

        let bytes =
            serialize_canonical(&value)
                .expect("Serde value must serialize");

        assert_eq!(
            bytes,
            br#"{"a":20,"nested":[3,2,1],"z":10}"#
        );
    }

    #[test]
    fn canonical_equality_is_representation_based() {
        let first = json!({
            "b": 2,
            "a": 1
        });

        let second = json!({
            "a": 1,
            "b": 2
        });

        assert!(
            canonical_eq(&first, &second)
                .expect("canonical equality must succeed")
        );
    }

    #[test]
    fn canonical_digest_equality_is_stable() {
        let first = json!({
            "b": 2,
            "a": 1
        });

        let second = json!({
            "a": 1,
            "b": 2
        });

        assert!(
            canonical_digest_eq(&first, &second)
                .expect("digest equality must succeed")
        );
    }

    #[test]
    fn digest_bytes_is_deterministic() {
        let first = digest_bytes(b"zamani");
        let second = digest_bytes(b"zamani");

        assert_eq!(first, second);
    }

    #[test]
    fn different_bytes_have_different_digest_for_normal_inputs() {
        let first = digest_bytes(b"zamani-a");
        let second = digest_bytes(b"zamani-b");

        assert_ne!(first, second);
    }
}