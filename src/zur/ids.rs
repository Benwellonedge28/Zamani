
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Identity Infrastructure
//!
//! Path: `src/zur/ids.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines strongly typed identities for entities whose identity is owned
//! by the Zamani Universal Runtime (ZUR).
//!
//! ZUR identities identify runtime instances, executions, sessions,
//! service instances, resource leases and runtime operations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - RuntimeInstanceId.
//! - ExecutionId.
//! - SessionId.
//! - ServiceInstanceId.
//! - ResourceLeaseId.
//! - RuntimeOperationId.
//! - Runtime identity namespaces.
//! - Runtime identity representation and validation.
//! - Runtime identity serialization.
//! - Explicitly scoped identity allocation.
//!
//! This module does NOT own:
//!
//! - Source-language identifiers.
//! - AST node identities.
//! - Semantic symbol identities.
//! - ZUIR value, operation or block identities.
//! - Classical IR identities.
//! - Quantum QubitId or quantum circuit identities.
//! - HDL design identities.
//! - AI model or tensor identities.
//! - Distributed NodeId, TaskId or MessageId.
//! - Hybrid DomainId or graph identities.
//! - Backend, target, toolchain or artifact identities.
//! - Physical device identities.
//! - Operating-system process identities.
//! - Content hashes or fingerprints.
//! - Authentication or authorization.
//! - Resource availability or capability evidence.
//!
//! Those identities remain owned by their respective subsystems.
//!
//! ## Identity invariants
//!
//! 1. Every identity category is a distinct Rust type.
//! 2. Identity equality includes its namespace and complete sequence.
//! 3. Numeric identity values are opaque.
//! 4. Identity allocation never wraps.
//! 5. An allocator never intentionally reuses an issued identity.
//! 6. Independent allocators do not automatically share uniqueness.
//! 7. Namespaces are case-sensitive.
//! 8. Parsing and deserialization validate all identity components.
//! 9. Identity existence does not imply entity existence or availability.
//! 10. Identity values do not encode hardware capacities.
//!
//! ## Representation
//!
//! Canonical textual representation:
//!
//!     namespace:lowercase-hexadecimal-sequence
//!
//! Example:
//!
//!     zamani.runtime.instance:01
//!     zamani.runtime.execution:02
//!     zamani.runtime.session:03
//!
//! The sequence is an unsigned, variable-length, big-endian byte sequence.
//!
//! Its hexadecimal representation:
//!
//! - Is lowercase.
//! - Contains an even number of hexadecimal characters.
//! - Contains at least one byte.
//! - Does not contain unnecessary leading zero bytes.
//!
//! The sequence is not restricted to u32, u64, u128 or machine-word width.
//!
//! ## Scalability
//!
//! There is no fixed maximum identifier count, namespace length, namespace
//! depth or numeric sequence width imposed by this module.
//!
//! Sequence storage grows as required and is ultimately constrained by
//! available memory and the allocator's ability to reserve storage.
//!
//! No finite machine promises literal infinite storage.
//!
//! ## Allocation scope
//!
//! An allocator guarantees unique sequential values only within its own
//! lifetime and allocation history.
//!
//! Two independently constructed allocators using the same namespace can
//! issue equal identifiers.
//!
//! Applications requiring persistent or cross-process uniqueness must
//! establish an explicit namespace-allocation and persistence policy.
//!
//! This module does not pretend to provide distributed uniqueness.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable identity state.
//! No fixed-width numeric exhaustion.
//!
//! ## Integration
//!
//! `zur/core/context.rs`:
//!     RuntimeInstanceId
//!
//! `zur/execution/`:
//!     ExecutionId
//!     SessionId
//!
//! `zur/core/service_registry.rs`:
//!     ServiceInstanceId
//!
//! `zur/resources/`:
//!     ResourceLeaseId
//!
//! `zur/observability/`:
//!     RuntimeOperationId
//!
//! Existing subsystem identities must be reused at their respective
//! integration boundaries.

use core::fmt;
use core::hash::Hash;
use core::str::FromStr;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::sync::{Arc, Mutex};

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered when validating a runtime identity namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvalidNamespace {
    /// The namespace is empty.
    Empty,

    /// A namespace contains an empty segment.
    EmptySegment,

    /// A namespace segment begins with an invalid character.
    InvalidSegmentStart,

    /// A namespace contains an unsupported character.
    InvalidCharacter,

    /// A namespace segment ends with a separator.
    InvalidSegmentBoundary,
}

impl fmt::Display for InvalidNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("runtime identity namespace cannot be empty")
            }

            Self::EmptySegment => {
                f.write_str("runtime identity namespace contains an empty segment")
            }

            Self::InvalidSegmentStart => {
                f.write_str("runtime identity namespace has an invalid segment start")
            }

            Self::InvalidCharacter => {
                f.write_str("runtime identity namespace contains an invalid character")
            }

            Self::InvalidSegmentBoundary => {
                f.write_str("runtime identity namespace has an invalid segment boundary")
            }
        }
    }
}

impl std::error::Error for InvalidNamespace {}

/// Errors encountered when constructing or parsing a runtime identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvalidId {
    /// The identity has no numeric sequence.
    EmptySequence,

    /// The identity contains no namespace separator.
    MissingSeparator,

    /// The identity contains an invalid namespace.
    InvalidNamespace(InvalidNamespace),

    /// The sequence contains an invalid hexadecimal character.
    InvalidHexadecimal,

    /// The hexadecimal sequence has an invalid length.
    InvalidSequenceLength,

    /// The sequence contains unnecessary leading zero bytes.
    NonCanonicalSequence,

    /// The sequence is zero.
    Zero,
}

impl fmt::Display for InvalidId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySequence => {
                f.write_str("runtime identity sequence cannot be empty")
            }

            Self::MissingSeparator => {
                f.write_str("runtime identity must contain a namespace separator")
            }

            Self::InvalidNamespace(error) => error.fmt(f),

            Self::InvalidHexadecimal => {
                f.write_str("runtime identity contains invalid hexadecimal data")
            }

            Self::InvalidSequenceLength => {
                f.write_str("runtime identity sequence must contain complete bytes")
            }

            Self::NonCanonicalSequence => {
                f.write_str("runtime identity contains unnecessary leading zero bytes")
            }

            Self::Zero => {
                f.write_str("runtime identity sequence must be nonzero")
            }
        }
    }
}

impl std::error::Error for InvalidId {}

impl From<InvalidNamespace> for InvalidId {
    fn from(value: InvalidNamespace) -> Self {
        Self::InvalidNamespace(value)
    }
}

/// Errors encountered when allocating a runtime identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IdAllocationError {
    /// Available memory or allocation capacity could not accommodate
    /// the next identity sequence.
    CapacityUnavailable,
}

impl fmt::Display for IdAllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityUnavailable => {
                f.write_str("insufficient allocation capacity for runtime identity")
            }
        }
    }
}

impl std::error::Error for IdAllocationError {}

// ============================================================================
// NAMESPACE
// ============================================================================

/// A validated, extensible runtime identity namespace.
///
/// Namespaces are case-sensitive and dot-separated.
///
/// Each segment:
///
/// - Starts with an ASCII lowercase letter or digit.
/// - Continues with ASCII lowercase letters, digits, hyphens or underscores.
/// - Cannot end with a hyphen or underscore.
///
/// There is no fixed namespace length or depth limit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdNamespace(Arc<str>);

impl IdNamespace {
    /// Constructs a validated namespace.
    pub fn new(value: impl AsRef<str>) -> Result<Self, InvalidNamespace> {
        let value = value.as_ref();

        validate_namespace(value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the canonical namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the number of dot-separated namespace segments.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.0.bytes().filter(|byte| *byte == b'.').count() + 1
    }

    /// Returns the final namespace segment.
    #[must_use]
    pub fn leaf(&self) -> &str {
        self.0
            .rsplit('.')
            .next()
            .expect("validated namespaces always have a final segment")
    }

    /// Returns the parent namespace, if present.
    #[must_use]
    pub fn parent(&self) -> Option<&str> {
        self.0.rsplit_once('.').map(|(parent, _)| parent)
    }
}

fn validate_namespace(value: &str) -> Result<(), InvalidNamespace> {
    if value.is_empty() {
        return Err(InvalidNamespace::Empty);
    }

    let mut segment_start = true;
    let mut previous_separator = false;

    for byte in value.bytes() {
        if byte == b'.' {
            if segment_start {
                return Err(InvalidNamespace::EmptySegment);
            }

            if previous_separator {
                return Err(InvalidNamespace::InvalidSegmentBoundary);
            }

            segment_start = true;
            previous_separator = true;
            continue;
        }

        if segment_start {
            if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() {
                return Err(InvalidNamespace::InvalidSegmentStart);
            }

            segment_start = false;
            previous_separator = false;
            continue;
        }

        let valid = byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'-'
            || byte == b'_';

        if !valid {
            return Err(InvalidNamespace::InvalidCharacter);
        }

        if previous_separator && (byte == b'-' || byte == b'_') {
            return Err(InvalidNamespace::InvalidSegmentBoundary);
        }

        previous_separator = byte == b'-' || byte == b'_';
    }

    if segment_start {
        return Err(InvalidNamespace::EmptySegment);
    }

    if previous_separator {
        return Err(InvalidNamespace::InvalidSegmentBoundary);
    }

    Ok(())
}

impl AsRef<str> for IdNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for IdNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for IdNamespace {
    type Err = InvalidNamespace;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for IdNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for IdNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(de::Error::custom)
    }
}

// ============================================================================
// INTERNAL ID REPRESENTATION
// ============================================================================

/// Shared validated identity representation.
///
/// The byte sequence is unsigned, nonzero and big-endian.
///
/// Arc allows identity values to be cloned without copying an arbitrarily
/// large sequence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ScopedId {
    namespace: IdNamespace,
    sequence: Arc<[u8]>,
}

impl ScopedId {
    fn new(
        namespace: IdNamespace,
        sequence: Vec<u8>,
    ) -> Result<Self, InvalidId> {
        validate_sequence(&sequence)?;

        Ok(Self {
            namespace,
            sequence: Arc::from(sequence),
        })
    }

    fn parse(value: &str) -> Result<Self, InvalidId> {
        let (namespace, encoded) = value
            .split_once(':')
            .ok_or(InvalidId::MissingSeparator)?;

        if encoded.is_empty() {
            return Err(InvalidId::EmptySequence);
        }

        if encoded.len() % 2 != 0 {
            return Err(InvalidId::InvalidSequenceLength);
        }

        let mut sequence = Vec::new();

        sequence
            .try_reserve_exact(encoded.len() / 2)
            .map_err(|_| InvalidId::InvalidSequenceLength)?;

        let bytes = encoded.as_bytes();

        for pair in bytes.chunks_exact(2) {
            let high = decode_hex(pair[0])
                .ok_or(InvalidId::InvalidHexadecimal)?;

            let low = decode_hex(pair[1])
                .ok_or(InvalidId::InvalidHexadecimal)?;

            sequence.push((high << 4) | low);
        }

        let namespace = IdNamespace::new(namespace)?;

        Self::new(namespace, sequence)
    }

    fn sequence(&self) -> &[u8] {
        &self.sequence
    }

    fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    fn write_canonical(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        write!(f, "{}:", self.namespace)?;

        for byte in self.sequence.iter() {
            let high = HEX[(byte >> 4) as usize];
            let low = HEX[(byte & 0x0f) as usize];

            write!(f, "{}{}", high as char, low as char)?;
        }

        Ok(())
    }
}

fn validate_sequence(sequence: &[u8]) -> Result<(), InvalidId> {
    if sequence.is_empty() {
        return Err(InvalidId::EmptySequence);
    }

    if sequence.iter().all(|byte| *byte == 0) {
        return Err(InvalidId::Zero);
    }

    if sequence.len() > 1 && sequence.first() == Some(&0) {
        return Err(InvalidId::NonCanonicalSequence);
    }

    Ok(())
}

fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

impl fmt::Display for ScopedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_canonical(f)
    }
}

// ============================================================================
// TYPED IDENTITY CONTRACT
// ============================================================================

mod sealed {
    use super::{IdNamespace, InvalidId, ScopedId};

    pub trait Sealed: Sized {
        fn from_scoped(id: ScopedId) -> Self;

        fn scoped(&self) -> &ScopedId;

        fn create(
            namespace: IdNamespace,
            sequence: Vec<u8>,
        ) -> Result<Self, InvalidId> {
            Ok(Self::from_scoped(ScopedId::new(
                namespace,
                sequence,
            )?))
        }
    }
}

/// Common behavior for strongly typed ZUR identities.
///
/// This trait is sealed: consumers cannot introduce new runtime identity
/// categories that bypass ZUR's identity ownership contract.
pub trait RuntimeId:
    sealed::Sealed
    + Clone
    + Eq
    + Ord
    + Hash
    + fmt::Debug
    + fmt::Display
    + Send
    + Sync
    + 'static
{
    /// Returns the validated identity namespace.
    #[must_use]
    fn namespace(&self) -> &IdNamespace {
        self.scoped().namespace()
    }

    /// Returns the complete unsigned big-endian identity sequence.
    #[must_use]
    fn sequence(&self) -> &[u8] {
        self.scoped().sequence()
    }

    /// Returns the canonical identity string.
    #[must_use]
    fn canonical(&self) -> String {
        self.to_string()
    }

    /// Parses a canonical identity.
    fn parse(value: &str) -> Result<Self, InvalidId>;

    #[doc(hidden)]
    fn scoped(&self) -> &ScopedId;
}

// ============================================================================
// IDENTITY CATEGORY DEFINITIONS
// ============================================================================

macro_rules! define_runtime_id {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(ScopedId);

        impl $name {
            /// Constructs an identity from a validated namespace and
            /// unsigned big-endian sequence.
            pub fn new(
                namespace: IdNamespace,
                sequence: Vec<u8>,
            ) -> Result<Self, InvalidId> {
                <Self as sealed::Sealed>::create(namespace, sequence)
            }

            /// Parses a canonical identity representation.
            pub fn parse(value: &str) -> Result<Self, InvalidId> {
                Ok(Self(ScopedId::parse(value)?))
            }

            /// Returns the identity namespace.
            #[must_use]
            pub fn namespace(&self) -> &IdNamespace {
                self.0.namespace()
            }

            /// Returns the complete unsigned big-endian sequence.
            #[must_use]
            pub fn sequence(&self) -> &[u8] {
                self.0.sequence()
            }

            /// Returns the canonical identity string.
            #[must_use]
            pub fn canonical(&self) -> String {
                self.to_string()
            }
        }

        impl sealed::Sealed for $name {
            fn from_scoped(id: ScopedId) -> Self {
                Self(id)
            }

            fn scoped(&self) -> &ScopedId {
                &self.0
            }
        }

        impl RuntimeId for $name {
            fn parse(value: &str) -> Result<Self, InvalidId> {
                Self::parse(value)
            }

            fn scoped(&self) -> &ScopedId {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = InvalidId;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;

                Self::parse(&value).map_err(de::Error::custom)
            }
        }
    };
}

define_runtime_id!(
    /// Identifies one ZUR runtime instance.
    ///
    /// It does not identify an operating-system process or prove that the
    /// runtime instance is currently active.
    RuntimeInstanceId
);

define_runtime_id!(
    /// Identifies one runtime-managed program execution.
    ///
    /// It does not replace a compiler, ZUIR or domain-specific operation ID.
    ExecutionId
);

define_runtime_id!(
    /// Identifies one runtime execution session.
    ///
    /// A session identity does not prove that its session is still active.
    SessionId
);

define_runtime_id!(
    /// Identifies one instantiated runtime service.
    ///
    /// It is distinct from the stable service descriptor or service contract
    /// identity.
    ServiceInstanceId
);

define_runtime_id!(
    /// Identifies one runtime resource lease.
    ///
    /// It does not identify the physical resource itself and does not prove
    /// that a lease remains valid.
    ResourceLeaseId
);

define_runtime_id!(
    /// Identifies one ZUR-owned operational action.
    ///
    /// This must not replace the operation identities owned by ZUIR or
    /// domain-specific intermediate representations.
    RuntimeOperationId
);

// ============================================================================
// VARIABLE-LENGTH SEQUENCE ALLOCATION
// ============================================================================

/// Thread-safe allocator for one ZUR identity category and namespace.
///
/// The internal sequence is an arbitrary-length unsigned big-endian byte
/// vector. It grows when required and does not wrap at a machine integer
/// boundary.
///
/// The allocator is independent of all other allocator instances.
///
/// It is not a persistent or distributed uniqueness authority.
pub struct IdAllocator<T: RuntimeId> {
    namespace: IdNamespace,
    next: Mutex<Vec<u8>>,
    marker: core::marker::PhantomData<fn() -> T>,
}

impl<T: RuntimeId> IdAllocator<T> {
    /// Creates a new allocator whose first identity sequence is one.
    ///
    /// The namespace should be unique within the identity scope in which
    /// allocated identities will be combined.
    #[must_use]
    pub fn new(namespace: IdNamespace) -> Self {
        Self {
            namespace,
            next: Mutex::new(vec![1]),
            marker: core::marker::PhantomData,
        }
    }

    /// Returns the allocator namespace.
    #[must_use]
    pub fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    /// Allocates the next identity without wrapping or reusing a value.
    ///
    /// If the sequence needs to grow but the required storage cannot be
    /// reserved, the operation returns an explicit allocation error.
    ///
    /// A failed allocation does not intentionally consume an identity.
    pub fn try_next(&self) -> Result<T, IdAllocationError> {
        let mut current = self
            .next
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut issued = Vec::new();

        issued
            .try_reserve_exact(current.len())
            .map_err(|_| IdAllocationError::CapacityUnavailable)?;

        issued.extend_from_slice(&current);

        increment_sequence(&mut current)?;

        let sequence = Arc::<[u8]>::from(issued);

        let id = ScopedId {
            namespace: self.namespace.clone(),
            sequence,
        };

        Ok(T::from_scoped(id))
    }

    /// Returns a snapshot of the next sequence to be allocated.
    ///
    /// This is diagnostic information, not a reservation. Another thread
    /// may allocate the sequence immediately after this method returns.
    #[must_use]
    pub fn next_sequence(&self) -> Vec<u8> {
        self.next
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}

fn increment_sequence(
    sequence: &mut Vec<u8>,
) -> Result<(), IdAllocationError> {
    for index in (0..sequence.len()).rev() {
        if sequence[index] != u8::MAX {
            sequence[index] += 1;
            return Ok(());
        }

        sequence[index] = 0;
    }

    // Every byte was 0xff. Extend the unsigned big-endian sequence.
    //
    // This is the operation that removes the fixed-width numeric ceiling.
    sequence
        .try_reserve(1)
        .map_err(|_| IdAllocationError::CapacityUnavailable)?;

    sequence.insert(0, 1);

    Ok(())
}

impl<T: RuntimeId> fmt::Debug for IdAllocator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdAllocator")
            .field("identity_type", &core::any::type_name::<T>())
            .field("namespace", &self.namespace)
            .field("next_sequence", &self.next_sequence())
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;

    fn namespace(value: &str) -> IdNamespace {
        IdNamespace::new(value).expect("valid test namespace")
    }

    #[test]
    fn namespace_validation() {
        assert_eq!(
            IdNamespace::new(""),
            Err(InvalidNamespace::Empty)
        );

        assert!(IdNamespace::new("zamani.runtime").is_ok());

        assert!(IdNamespace::new("org.example.runtime").is_ok());

        assert!(IdNamespace::new("runtime-v2").is_ok());

        assert!(IdNamespace::new("Runtime").is_err());

        assert!(IdNamespace::new(".runtime").is_err());

        assert!(IdNamespace::new("runtime.").is_err());

        assert!(IdNamespace::new("runtime..session").is_err());

        assert!(IdNamespace::new("runtime/session").is_err());

        assert!(IdNamespace::new("runtime:session").is_err());
    }

    #[test]
    fn namespace_metadata_is_deterministic() {
        let value = namespace("org.example.runtime.session");

        assert_eq!(value.segment_count(), 4);

        assert_eq!(value.leaf(), "session");

        assert_eq!(
            value.parent(),
            Some("org.example.runtime")
        );
    }

    #[test]
    fn constructs_and_formats_canonical_identity() {
        let id = RuntimeInstanceId::new(
            namespace("zamani.runtime.instance"),
            vec![1],
        )
        .expect("valid identity");

        assert_eq!(
            id.to_string(),
            "zamani.runtime.instance:01"
        );
    }

    #[test]
    fn parses_canonical_identity() {
        let id = RuntimeInstanceId::parse(
            "zamani.runtime.instance:01",
        )
        .expect("valid identity");

        assert_eq!(id.sequence(), &[1]);

        assert_eq!(
            id.namespace().as_str(),
            "zamani.runtime.instance"
        );
    }

    #[test]
    fn rejects_invalid_identity_sequences() {
        assert_eq!(
            RuntimeInstanceId::new(
                namespace("zamani.runtime"),
                vec![],
            ),
            Err(InvalidId::EmptySequence)
        );

        assert_eq!(
            RuntimeInstanceId::new(
                namespace("zamani.runtime"),
                vec![0],
            ),
            Err(InvalidId::Zero)
        );

        assert_eq!(
            RuntimeInstanceId::new(
                namespace("zamani.runtime"),
                vec![0, 1],
            ),
            Err(InvalidId::NonCanonicalSequence)
        );
    }

    #[test]
    fn rejects_noncanonical_text() {
        assert_eq!(
            RuntimeInstanceId::parse("invalid"),
            Err(InvalidId::MissingSeparator)
        );

        assert_eq!(
            RuntimeInstanceId::parse("runtime:"),
            Err(InvalidId::EmptySequence)
        );

        assert_eq!(
            RuntimeInstanceId::parse("runtime:0"),
            Err(InvalidId::InvalidSequenceLength)
        );

        assert_eq!(
            RuntimeInstanceId::parse("runtime:00"),
            Err(InvalidId::Zero)
        );

        assert_eq!(
            RuntimeInstanceId::parse("runtime:0001"),
            Err(InvalidId::NonCanonicalSequence)
        );

        assert_eq!(
            RuntimeInstanceId::parse("runtime:0G"),
            Err(InvalidId::InvalidHexadecimal)
        );

        assert!(
            RuntimeInstanceId::parse("runtime:0A").is_err()
        );
    }

    #[test]
    fn identity_categories_are_distinct() {
        let namespace = namespace("zamani.runtime");

        let execution = ExecutionId::new(
            namespace.clone(),
            vec![1],
        )
        .unwrap();

        let session = SessionId::new(
            namespace,
            vec![1],
        )
        .unwrap();

        assert_eq!(execution.sequence(), session.sequence());

        assert_ne!(
            execution.to_string(),
            session.to_string()
        );
    }

    #[test]
    fn namespace_is_part_of_identity_equality() {
        let first = ExecutionId::new(
            namespace("runtime.first"),
            vec![1],
        )
        .unwrap();

        let second = ExecutionId::new(
            namespace("runtime.second"),
            vec![1],
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn allocation_is_monotonic() {
        let allocator = IdAllocator::<ExecutionId>::new(
            namespace("zamani.runtime.execution"),
        );

        let first = allocator.try_next().unwrap();
        let second = allocator.try_next().unwrap();
        let third = allocator.try_next().unwrap();

        assert_eq!(first.to_string(), "zamani.runtime.execution:01");

        assert_eq!(second.to_string(), "zamani.runtime.execution:02");

        assert_eq!(third.to_string(), "zamani.runtime.execution:03");
    }

    #[test]
    fn allocation_grows_beyond_one_byte() {
        let allocator = IdAllocator::<ExecutionId>::new(
            namespace("zamani.runtime.execution"),
        );

        for _ in 0..255 {
            allocator.try_next().unwrap();
        }

        let next = allocator.try_next().unwrap();

        assert_eq!(
            next.to_string(),
            "zamani.runtime.execution:0100"
        );
    }

    #[test]
    fn allocation_grows_beyond_two_bytes() {
        let allocator = IdAllocator::<ExecutionId>::new(
            namespace("zamani.runtime.execution"),
        );

        for _ in 0..65_535 {
            allocator.try_next().unwrap();
        }

        let next = allocator.try_next().unwrap();

        assert_eq!(
            next.to_string(),
            "zamani.runtime.execution:010000"
        );
    }

    #[test]
    fn allocator_does_not_claim_cross_instance_uniqueness() {
        let first = IdAllocator::<ExecutionId>::new(
            namespace("runtime.execution"),
        );

        let second = IdAllocator::<ExecutionId>::new(
            namespace("runtime.execution"),
        );

        assert_eq!(
            first.try_next().unwrap(),
            second.try_next().unwrap()
        );
    }

    #[test]
    fn different_namespaces_produce_distinct_identities() {
        let first = IdAllocator::<ExecutionId>::new(
            namespace("runtime.execution.first"),
        );

        let second = IdAllocator::<ExecutionId>::new(
            namespace("runtime.execution.second"),
        );

        assert_ne!(
            first.try_next().unwrap(),
            second.try_next().unwrap()
        );
    }

    #[test]
    fn allocator_sequence_grows_without_integer_overflow() {
        let mut sequence = vec![u8::MAX];

        increment_sequence(&mut sequence).unwrap();

        assert_eq!(sequence, vec![1, 0]);

        increment_sequence(&mut sequence).unwrap();

        assert_eq!(sequence, vec![1, 1]);
    }

    #[test]
    fn allocator_handles_arbitrarily_wide_existing_sequences() {
        let mut sequence = vec![u8::MAX; 32];

        increment_sequence(&mut sequence).unwrap();

        assert_eq!(sequence.len(), 33);

        assert_eq!(sequence[0], 1);

        assert!(sequence[1..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn clones_preserve_identity() {
        let original = ResourceLeaseId::new(
            namespace("runtime.resources"),
            vec![1, 2, 3, 4],
        )
        .unwrap();

        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn ordering_and_hashing_are_consistent() {
        let first = ExecutionId::new(
            namespace("runtime.execution"),
            vec![1],
        )
        .unwrap();

        let second = ExecutionId::new(
            namespace("runtime.execution"),
            vec![2],
        )
        .unwrap();

        assert!(first < second);

        let set = HashSet::from([
            first.clone(),
            second.clone(),
            first.clone(),
        ]);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn serde_round_trip_preserves_identity() {
        let original = SessionId::new(
            namespace("zamani.runtime.session"),
            vec![1, 2, 3, 4],
        )
        .unwrap();

        let encoded = serde_json::to_string(&original)
            .unwrap();

        assert_eq!(
            encoded,
            "\"zamani.runtime.session:01020304\""
        );

        let decoded: SessionId =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn serde_rejects_invalid_identity() {
        assert!(
            serde_json::from_str::<SessionId>(
                "\"runtime:0001\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<SessionId>(
                "\"runtime:00\""
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<SessionId>(
                "\"Runtime:01\""
            )
            .is_err()
        );
    }

    #[test]
    fn concurrent_allocation_is_unique_within_allocator() {
        let allocator = Arc::new(
            IdAllocator::<RuntimeOperationId>::new(
                namespace("zamani.runtime.operation"),
            ),
        );

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let allocator = Arc::clone(&allocator);

            handles.push(thread::spawn(move || {
                (0..128)
                    .map(|_| allocator.try_next().unwrap())
                    .collect::<Vec<_>>()
            }));
        }

        let mut identities = Vec::new();

        for handle in handles {
            identities.extend(handle.join().unwrap());
        }

        let unique: HashSet<_> =
            identities.iter().cloned().collect();

        assert_eq!(unique.len(), identities.len());
    }

    #[test]
    fn all_six_categories_have_independent_allocators() {
        let runtime = IdAllocator::<RuntimeInstanceId>::new(
            namespace("runtime.instance"),
        );

        let execution = IdAllocator::<ExecutionId>::new(
            namespace("runtime.execution"),
        );

        let session = IdAllocator::<SessionId>::new(
            namespace("runtime.session"),
        );

        let service = IdAllocator::<ServiceInstanceId>::new(
            namespace("runtime.service"),
        );

        let lease = IdAllocator::<ResourceLeaseId>::new(
            namespace("runtime.lease"),
        );

        let operation = IdAllocator::<RuntimeOperationId>::new(
            namespace("runtime.operation"),
        );

        assert_eq!(runtime.try_next().unwrap().sequence(), &[1]);

        assert_eq!(execution.try_next().unwrap().sequence(), &[1]);

        assert_eq!(session.try_next().unwrap().sequence(), &[1]);

        assert_eq!(service.try_next().unwrap().sequence(), &[1]);

        assert_eq!(lease.try_next().unwrap().sequence(), &[1]);

        assert_eq!(operation.try_next().unwrap().sequence(), &[1]);
    }
}
