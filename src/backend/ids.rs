
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Backend — Strongly Typed Identifiers
//!
//! Path: `src/backend/ids.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define validated, strongly typed identities used by the backend layer.
//! This module is independent of target emitters, compiler orchestration,
//! artifact storage, external toolchains, and domain-specific IRs.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Backend, target, toolchain, artifact, and operation identity types.
//! - Namespace validation and canonical identifier formatting.
//! - Parsing and serialization of identifiers.
//! - Per-namespace monotonic ID allocation.
//! - Explicit invalid-input and exhaustion errors.
//!
//! It does not own:
//!
//! - Source-language names, AST node IDs, or semantic symbol IDs.
//! - ZUIR or domain IR operation IDs.
//! - Physical device, qubit, FPGA, register, or runtime-handle identities.
//! - Target capability definitions, target selection, or scheduling.
//! - Content fingerprints, artifact hashes, or reproducibility policy.
//! - Global identity coordination between independent processes.
//!
//! ## Identity contract
//!
//! Each category is a distinct Rust type. Equal numeric values in different
//! categories do not make identifiers interchangeable.
//!
//! An identifier is scoped by its namespace. Its numeric value is opaque;
//! it must not be interpreted as an index, ordering of execution, source
//! position, resource capacity, or content fingerprint.
//!
//! Allocators guarantee uniqueness only within one allocator instance.
//! Combining identities from separate allocators requires an explicit owner
//! to establish the appropriate namespace and collision policy.
//!
//! ## Canonical representation
//!
//! The canonical textual form is:
//!
//!     namespace:nonzero-decimal-u128
//!
//! The namespace starts with an ASCII lowercase letter and continues with
//! ASCII lowercase letters, digits, `.`, `_`, or `-`.
//! Numeric values use decimal digits without a sign or leading zeroes.
//! Namespace comparison is case-sensitive.
//!
//! Parsing rejects noncanonical spellings rather than silently normalizing
//! them. Display and serialization therefore produce stable canonical text.
//!
//! ## Scalability and exhaustion
//!
//! This module imposes no limit on the number of targets, artifacts, devices,
//! operations, or computational domains.
//!
//! Each individual numeric identity uses a nonzero `u128`, so the identifier
//! space is large but finite. Exhaustion is reported explicitly; values are
//! never wrapped or reused by an allocator.
//!
//! A namespace does not itself guarantee global uniqueness. Persistent,
//! cross-process identity requires a documented namespace-allocation policy.
//!
//! ## Integration
//!
//! The backend registry uses `BackendId`.
//! Target descriptions and selection use `TargetId`.
//! Tool discovery and execution use `ToolchainId`.
//! Artifact manifests and output collections use `ArtifactId`.
//! Backend-local operation tracking may use `BackendOperationId`.
//!
//! Use canonical AST, semantic, ZUIR, quantum, and HDL identifiers at their
//! respective boundaries. Do not convert unrelated ID categories merely
//! because they share the same numeric representation.
//!
//! ## Dependencies
//!
//! Uses the existing `serde` dependency from `Cargo.toml`.
//! Defines its own small error types to avoid a dependency on a future
//! backend `errors.rs`, preventing a foundational dependency cycle.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt,
    marker::PhantomData,
    num::NonZeroU128,
    str::FromStr,
    sync::Mutex,
};

/// Errors encountered when validating a namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidNamespace {
    /// The namespace is empty.
    Empty,
    /// The namespace does not follow the canonical ASCII grammar.
    InvalidSyntax,
}

impl fmt::Display for InvalidNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("identifier namespace must not be empty"),
            Self::InvalidSyntax => f.write_str(
                "namespace must start with a lowercase ASCII letter and contain \
                 only lowercase ASCII letters, digits, '.', '_' or '-'",
            ),
        }
    }
}

impl std::error::Error for InvalidNamespace {}

/// Errors encountered when parsing or constructing an identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidId {
    /// The numeric component is zero.
    Zero,
    /// The textual representation does not follow the canonical grammar.
    InvalidSyntax,
    /// The namespace is invalid.
    InvalidNamespace(InvalidNamespace),
    /// The numeric component is outside the supported `u128` range.
    NumericOverflow,
    /// The text uses a noncanonical numeric representation.
    NonCanonicalNumber,
}

impl fmt::Display for InvalidId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("identifier value must be nonzero"),
            Self::InvalidSyntax => f.write_str(
                "identifier must use the canonical namespace:decimal-value form",
            ),
            Self::InvalidNamespace(error) => error.fmt(f),
            Self::NumericOverflow => {
                f.write_str("identifier numeric component exceeds u128")
            }
            Self::NonCanonicalNumber => f.write_str(
                "identifier numeric component must contain decimal digits \
                 without a sign or leading zeroes",
            ),
        }
    }
}

impl std::error::Error for InvalidId {}

impl From<InvalidNamespace> for InvalidId {
    fn from(value: InvalidNamespace) -> Self {
        Self::InvalidNamespace(value)
    }
}

/// Returned when an allocator has exhausted the nonzero `u128` value space.
///
/// Exhaustion is permanent for that allocator. It never wraps or reuses an ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdSpaceExhausted;

impl fmt::Display for IdSpaceExhausted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("backend identifier allocator exhausted its u128 value space")
    }
}

impl std::error::Error for IdSpaceExhausted {}

/// A validated namespace for backend-owned identifiers.
///
/// Namespace strings are case-sensitive and stored in canonical form.
/// Construction and deserialization both validate the namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdNamespace(String);

impl IdNamespace {
    /// Construct a namespace after validating its canonical ASCII syntax.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidNamespace> {
        let value = value.into();
        validate_namespace(&value)?;
        Ok(Self(value))
    }

    /// Return the canonical namespace text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_namespace(value: &str) -> Result<(), InvalidNamespace> {
    let mut chars = value.chars();

    match chars.next() {
        None => return Err(InvalidNamespace::Empty),
        Some(first) if first.is_ascii_lowercase() => {}
        Some(_) => return Err(InvalidNamespace::InvalidSyntax),
    }

    if chars.all(|ch| {
        ch.is_ascii_lowercase()
            || ch.is_ascii_digit()
            || matches!(ch, '.' | '_' | '-')
    }) {
        Ok(())
    } else {
        Err(InvalidNamespace::InvalidSyntax)
    }
}

impl fmt::Display for IdNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
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

/// A validated namespace-qualified numeric identity.
///
/// This representation is private so callers cannot bypass validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ScopedId {
    namespace: IdNamespace,
    value: NonZeroU128,
}

impl ScopedId {
    fn new(namespace: IdNamespace, value: u128) -> Result<Self, InvalidId> {
        let value = NonZeroU128::new(value).ok_or(InvalidId::Zero)?;
        Ok(Self { namespace, value })
    }

    fn parse(text: &str) -> Result<Self, InvalidId> {
        let (namespace_text, number_text) =
            text.split_once(':').ok_or(InvalidId::InvalidSyntax)?;

        if number_text.is_empty() || number_text.bytes().any(|b| !b.is_ascii_digit()) {
            return Err(InvalidId::InvalidSyntax);
        }

        if number_text.len() > 1 && number_text.starts_with('0') {
            return Err(InvalidId::NonCanonicalNumber);
        }

        let value = number_text
            .parse::<u128>()
            .map_err(|_| InvalidId::NumericOverflow)?;

        let namespace = IdNamespace::new(namespace_text)?;
        Self::new(namespace, value)
    }

    fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    fn value(&self) -> u128 {
        self.value.get()
    }
}

impl fmt::Display for ScopedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}

/// Common operations shared by backend identifier categories.
pub trait BackendTypedId:
    Copy + Eq + Ord + std::hash::Hash + fmt::Debug + fmt::Display + Send + Sync + 'static
{
    /// Construct from a namespace and a nonzero raw value.
    fn try_new(namespace: IdNamespace, value: u128) -> Result<Self, InvalidId>;

    /// Return the namespace of this identity.
    fn namespace(&self) -> &IdNamespace;

    /// Return the opaque numeric component.
    fn value(self) -> u128;

    /// Construct from an already validated internal representation.
    #[doc(hidden)]
    fn from_scoped_id(id: ScopedId) -> Self;

    /// Borrow the internal representation for canonical formatting.
    #[doc(hidden)]
    fn as_scoped_id(&self) -> &ScopedId;
}

macro_rules! define_backend_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy)]
        pub struct $name(ScopedId);

        impl $name {
            /// Construct an identifier from a namespace and nonzero value.
            pub fn new(
                namespace: IdNamespace,
                value: u128,
            ) -> Result<Self, InvalidId> {
                Ok(Self(ScopedId::new(namespace, value)?))
            }

            /// Parse a canonical `namespace:decimal-value` identifier.
            pub fn parse(text: &str) -> Result<Self, InvalidId> {
                Ok(Self(ScopedId::parse(text)?))
            }

            /// Return the identifier namespace.
            pub fn namespace(&self) -> &IdNamespace {
                self.0.namespace()
            }

            /// Return the opaque nonzero numeric component.
            pub fn value(self) -> u128 {
                self.0.value()
            }
        }

        impl BackendTypedId for $name {
            fn try_new(
                namespace: IdNamespace,
                value: u128,
            ) -> Result<Self, InvalidId> {
                Self::new(namespace, value)
            }

            fn namespace(&self) -> &IdNamespace {
                self.0.namespace()
            }

            fn value(self) -> u128 {
                self.0.value()
            }

            fn from_scoped_id(id: ScopedId) -> Self {
                Self(id)
            }

            fn as_scoped_id(&self) -> &ScopedId {
                &self.0
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = InvalidId;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::parse(text)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let text = String::deserialize(deserializer)?;
                Self::parse(&text).map_err(de::Error::custom)
            }
        }
    };
}

define_backend_id!(
    /// Identifies one registered backend implementation.
    ///
    /// It is not a target identity or a target's display name.
    BackendId
);

define_backend_id!(
    /// Identifies a compilation target descriptor or target registration.
    ///
    /// Target identity must not be inferred from the selected emitter.
    TargetId
);

define_backend_id!(
    /// Identifies an external or embedded toolchain registration.
    ToolchainId
);

define_backend_id!(
    /// Identifies an artifact within the owning artifact namespace.
    ///
    /// This ID is not a checksum or content fingerprint.
    ArtifactId
);

define_backend_id!(
    /// Identifies a backend-local operation or tracked backend action.
    ///
    /// This must not replace the canonical operation ID owned by ZUIR or a
    /// domain-specific IR.
    BackendOperationId
);

/// A monotonic allocator for one backend ID category and namespace.
///
/// The allocator is safe for concurrent calls. Different allocator instances
/// are independent; their IDs can collide if their namespaces also match.
/// The owner must choose namespaces and allocator scopes accordingly.
pub struct IdAllocator<T: BackendTypedId> {
    namespace: IdNamespace,
    next: Mutex<Option<u128>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: BackendTypedId> IdAllocator<T> {
    /// Create an allocator starting at one for the supplied namespace.
    pub fn new(namespace: IdNamespace) -> Self {
        Self {
            namespace,
            next: Mutex::new(Some(1)),
            marker: PhantomData,
        }
    }

    /// Return the allocator namespace.
    pub fn namespace(&self) -> &IdNamespace {
        &self.namespace
    }

    /// Allocate the next ID without wrapping or reusing a value.
    pub fn try_next(&self) -> Result<T, IdSpaceExhausted> {
        let mut guard = match self.next.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let current = guard.ok_or(IdSpaceExhausted)?;

        // `None` is a permanent exhausted state. In particular, issuing
        // u128::MAX does not wrap to zero or restart the allocator.
        *guard = current.checked_add(1);

        let id = ScopedId {
            namespace: self.namespace.clone(),
            value: NonZeroU128::new(current).ok_or(IdSpaceExhausted)?,
        };

        Ok(T::from_scoped_id(id))
    }

    /// Report whether the allocator has exhausted its value space.
    ///
    /// Concurrent allocation can change the result immediately afterward.
    pub fn is_exhausted(&self) -> bool {
        let guard = match self.next.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.is_none()
    }
}

impl<T: BackendTypedId> fmt::Debug for IdAllocator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdAllocator")
            .field("id_type", &std::any::type_name::<T>())
            .field("namespace", &self.namespace)
            .field("exhausted", &self.is_exhausted())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashSet,
        sync::Arc,
        thread,
    };

    fn namespace(value: &str) -> IdNamespace {
        IdNamespace::new(value).expect("test namespace must be valid")
    }

    #[test]
    fn namespaces_validate_and_round_trip() {
        assert_eq!(IdNamespace::new(""), Err(InvalidNamespace::Empty));
        assert!(IdNamespace::new("Backend").is_err());
        assert!(IdNamespace::new("backend/target").is_err());

        let original = namespace("zamani.backend-v1");
        let encoded = serde_json::to_string(&original).expect("serialize");
        let decoded: IdNamespace =
            serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, original);
    }

    #[test]
    fn zero_is_rejected() {
        assert_eq!(
            BackendId::new(namespace("zamani.backend"), 0),
            Err(InvalidId::Zero)
        );
        assert_eq!(
            TargetId::parse("zamani.target:0"),
            Err(InvalidId::Zero)
        );
    }

    #[test]
    fn canonical_parse_display_round_trip() {
        let original =
            BackendId::new(namespace("zamani.backend"), 42).expect("valid ID");

        assert_eq!(original.to_string(), "zamani.backend:42");
        assert_eq!(
            BackendId::parse(&original.to_string()).expect("parse"),
            original
        );
        assert_eq!(
            "zamani.backend:42".parse::<BackendId>().expect("parse"),
            original
        );
    }

    #[test]
    fn parser_rejects_noncanonical_numbers() {
        assert_eq!(
            BackendId::parse("zamani.backend:042"),
            Err(InvalidId::NonCanonicalNumber)
        );
        assert_eq!(
            BackendId::parse("zamani.backend:+42"),
            Err(InvalidId::InvalidSyntax)
        );
        assert_eq!(
            BackendId::parse("zamani.backend:-42"),
            Err(InvalidId::InvalidSyntax)
        );
        assert_eq!(
            BackendId::parse("zamani.backend:"),
            Err(InvalidId::InvalidSyntax)
        );
        assert_eq!(
            BackendId::parse("zamani.backend:340282366920938463463374607431768211456"),
            Err(InvalidId::NumericOverflow)
        );
    }

    #[test]
    fn categories_are_distinct_types() {
        let backend =
            BackendId::new(namespace("zamani"), 7).expect("valid ID");
        let target =
            TargetId::new(namespace("zamani"), 7).expect("valid ID");

        assert_eq!(backend.value(), target.value());
        assert_ne!(backend.to_string(), target.to_string());
        assert_ne!(format!("{backend:?}"), format!("{target:?}"));
    }

    #[test]
    fn namespaces_are_part_of_identity() {
        let first =
            BackendId::new(namespace("provider.alpha"), 1).expect("valid ID");
        let second =
            BackendId::new(namespace("provider.beta"), 1).expect("valid ID");

        assert_ne!(first, second);
        assert_ne!(first.to_string(), second.to_string());
    }

    #[test]
    fn ordering_and_hashing_are_consistent() {
        let first =
            ArtifactId::new(namespace("zamani.artifact"), 1).expect("valid");
        let second =
            ArtifactId::new(namespace("zamani.artifact"), 2).expect("valid");

        assert!(first < second);

        let set: HashSet<_> = [first, second].into_iter().collect();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn allocator_is_monotonic_and_namespace_scoped() {
        let allocator =
            IdAllocator::<TargetId>::new(namespace("zamani.target"));

        let first = allocator.try_next().expect("first");
        let second = allocator.try_next().expect("second");

        assert_eq!(first.value(), 1);
        assert_eq!(second.value(), 2);
        assert_eq!(first.namespace(), second.namespace());
        assert!(!allocator.is_exhausted());
    }

    #[test]
    fn allocator_instances_do_not_claim_global_uniqueness() {
        let first =
            IdAllocator::<ArtifactId>::new(namespace("build.one"));
        let second =
            IdAllocator::<ArtifactId>::new(namespace("build.two"));

        assert_eq!(first.try_next().expect("first").value(), 1);
        assert_eq!(second.try_next().expect("first").value(), 1);
    }

    #[test]
    fn concurrent_allocations_are_unique_within_allocator() {
        let allocator = Arc::new(IdAllocator::<BackendOperationId>::new(
            namespace("zamani.backend.operation"),
        ));

        let workers = thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let allocator = Arc::clone(&allocator);
            handles.push(thread::spawn(move || {
                (0..256)
                    .map(|_| allocator.try_next().expect("ID space available"))
                    .collect::<Vec<_>>()
            }));
        }

        let mut values = Vec::new();
        for handle in handles {
            values.extend(handle.join().expect("worker must finish"));
        }

        let unique: HashSet<_> = values.iter().copied().collect();
        assert_eq!(unique.len(), values.len());
    }

    #[test]
    fn serde_round_trip_preserves_canonical_identity() {
        let original =
            ToolchainId::new(namespace("toolchain.llvm"), 123).expect("valid");

        let encoded = serde_json::to_string(&original).expect("serialize");
        assert_eq!(encoded, "\"toolchain.llvm:123\"");

        let decoded: ToolchainId =
            serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded, original);
    }

    #[test]
    fn serde_rejects_invalid_identity() {
        assert!(
            serde_json::from_str::<BackendId>("\"Backend:1\"").is_err()
        );
        assert!(
            serde_json::from_str::<BackendId>("\"zamani.backend:0\"").is_err()
        );
        assert!(
            serde_json::from_str::<BackendId>("\"zamani.backend:01\"").is_err()
        );
    }

    #[test]
    fn maximum_nonzero_value_is_valid() {
        let id = ArtifactId::new(namespace("zamani.artifact"), u128::MAX)
            .expect("maximum nonzero value is valid");

        assert_eq!(id.value(), u128::MAX);
        assert_eq!(
            ArtifactId::parse(&id.to_string()).expect("round trip"),
            id
        );
    }
}
