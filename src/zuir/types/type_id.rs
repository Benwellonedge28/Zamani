#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR canonical type identity
//!
//! Path: `src/zuir/types/type_id.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provides type-identity utilities for the ZUIR type subsystem while
//! reusing the canonical identifier implementation in `crate::zuir::ids`.
//!
//! OWNS
//!
//! - The semantic qualified-name lookup key (`TypeKey`).
//! - Validation of type-name lookup keys.
//! - The type-specific allocator facade.
//! - Explicit identity-allocation error propagation.
//!
//! DOES NOT OWN
//!
//! - The canonical `TypeId` representation.
//! - Type definitions or type inference.
//! - Structural type equivalence or compatibility.
//! - Type interning, registry storage, or name resolution.
//! - Structural fingerprints or persistent content hashes.
//! - Target layout, ABI, physical allocation, or code generation.
//!
//! INTEGRATION
//!
//! - `crate::zuir::ids` owns `TypeId` and `IdAllocator`.
//! - `crate::zuir::schema::types::QualifiedName` owns qualified-name
//!   representation and its identifier validation rules.
//! - `types/type_def.rs` owns `TypeDef` and `TypeRef`.
//! - A future type registry owns name-to-ID mappings, uniqueness,
//!   interning policy, and registry lifetime.
//! - `types/compatibility.rs` owns type equivalence and assignability.
//! - A future fingerprint module owns stable content fingerprints.
//!
//! IDENTITY RULES
//!
//! A `TypeId` identifies a type entity within the allocation scope chosen
//! by its owner. It is not a content hash and does not imply cross-build
//! stability.
//!
//! A `TypeKey` is a validated qualified-name lookup key. It is not a
//! proof of type equivalence. The registry must reject duplicate
//! definitions in a namespace where uniqueness is required.
//!
//! Structurally equivalent types may have different nominal identities.
//! Conversely, equal names in different namespaces or registry scopes
//! must not be assumed to denote the same type.
//!
//! SCALABILITY
//!
//! No machine-specific limit is imposed on the number of types, type-name
//! components, integer widths, tensor dimensions, or domain resources.
//! Allocation and representation remain bounded by available resources
//! and the finite identifier representation.
//!
//! Identifier exhaustion is returned explicitly. IDs are never silently
//! wrapped or reused by this module.
//!
//! SERIALIZATION
//!
//! `TypeId` uses the canonical serialization contract of `ids.rs`.
//! `TypeKey` serializes its qualified name. Because deserialization can
//! bypass constructors, consumers must call `validate()` on untrusted
//! deserialized keys before using them as registry keys.
//!
//! FREEZE CRITERIA
//!
//! - No duplicate TypeId type is defined here.
//! - Qualified names are validated before lookup-key construction.
//! - Identifier exhaustion remains explicit.
//! - No registry, verifier, or backend dependency is introduced.
//! - Unit tests cover valid names, invalid deserialized keys, identity
//!   allocation, equality, ordering, and exhaustion behavior.

use crate::zuir::ids::{IdAllocator, IdSpaceExhausted};
pub use crate::zuir::ids::TypeId;
use crate::zuir::schema::types::{QualifiedName, SchemaTypeError};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated qualified-name key for looking up a semantic type.
///
/// The key preserves the name's component boundaries. It does not flatten
/// the name into a delimiter-encoded string, so names remain consistent
/// with the canonical `QualifiedName` representation.
///
/// # Identity
///
/// `TypeKey` is a lookup key, not the identity of an allocated type entity.
/// The owning registry determines whether a name is unique and which
/// `TypeId` it resolves to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypeKey(QualifiedName);

impl TypeKey {
    /// Construct a lookup key from a qualified name.
    ///
    /// Validation is performed even if the name was obtained by
    /// deserializing data rather than by calling `QualifiedName::new`.
    pub fn new(name: QualifiedName) -> Result<Self, TypeIdentityError> {
        name.validate()
            .map_err(TypeIdentityError::InvalidQualifiedName)?;

        Ok(Self(name))
    }

    /// Parse a canonical `::`-separated qualified name.
    ///
    /// Empty names, empty components, and invalid identifier components
    /// are rejected by the canonical qualified-name implementation.
    pub fn parse(name: &str) -> Result<Self, TypeIdentityError> {
        let name = QualifiedName::parse(name)
            .map_err(TypeIdentityError::InvalidQualifiedName)?;

        Self::new(name)
    }

    /// Return the qualified name represented by this key.
    #[must_use]
    pub fn qualified_name(&self) -> &QualifiedName {
        &self.0
    }

    /// Return the canonical textual form of the qualified name.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        self.0.canonical_string()
    }

    /// Validate the key, including keys created by deserialization.
    pub fn validate(&self) -> Result<(), TypeIdentityError> {
        self.0
            .validate()
            .map_err(TypeIdentityError::InvalidQualifiedName)
    }

    /// Consume the key and return its qualified name.
    #[must_use]
    pub fn into_qualified_name(self) -> QualifiedName {
        self.0
    }
}

impl fmt::Display for TypeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Allocates canonical type identifiers using the repository-wide ID system.
///
/// This facade exists to give the type subsystem a focused API. It does
/// not introduce a second ID representation or a separate counter model.
///
/// The owner must decide the allocator's scope. A single allocator provides
/// unique IDs within its own sequence; independently created allocators
/// are not globally coordinated.
#[derive(Debug)]
pub struct TypeIdAllocator {
    inner: IdAllocator<TypeId>,
}

impl TypeIdAllocator {
    /// Create a new allocator for a distinct type-identity scope.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: IdAllocator::new(),
        }
    }

    /// Allocate the next type identifier.
    ///
    /// Returns an error if the canonical identifier space is exhausted.
    /// The allocator never silently wraps or reuses an issued ID.
    pub fn try_next(&self) -> Result<TypeId, IdSpaceExhausted> {
        self.inner.try_next()
    }
}

impl Default for TypeIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors produced by type-identity utilities.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypeIdentityError {
    /// A qualified name is malformed or violates the canonical name rules.
    #[error("invalid qualified type name: {0}")]
    InvalidQualifiedName(#[source] SchemaTypeError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_qualified_type_name() {
        let key = TypeKey::parse("core::numeric::Integer")
            .expect("valid qualified name");

        assert_eq!(key.canonical_name(), "core::numeric::Integer");
        assert_eq!(key.qualified_name().depth(), 3);
        assert_eq!(key.qualified_name().local_name(), "Integer");
    }

    #[test]
    fn rejects_an_empty_type_name() {
        assert!(TypeKey::parse("").is_err());
    }

    #[test]
    fn rejects_empty_qualified_name_components() {
        assert!(TypeKey::parse("core::::Integer").is_err());
        assert!(TypeKey::parse("::Integer").is_err());
        assert!(TypeKey::parse("core::").is_err());
    }

    #[test]
    fn equivalent_names_produce_equal_lookup_keys() {
        let first = TypeKey::parse("core::Integer").expect("valid name");
        let second = TypeKey::parse("core::Integer").expect("valid name");

        assert_eq!(first, second);
        assert_eq!(first.cmp(&second), std::cmp::Ordering::Equal);
    }

    #[test]
    fn distinct_names_produce_distinct_lookup_keys() {
        let first = TypeKey::parse("core::Integer").expect("valid name");
        let second = TypeKey::parse("numeric::Integer").expect("valid name");

        assert_ne!(first, second);
    }

    #[test]
    fn type_ids_are_allocated_by_the_canonical_allocator() {
        let allocator = TypeIdAllocator::new();

        let first = allocator.try_next().expect("first type ID");
        let second = allocator.try_next().expect("second type ID");

        assert_ne!(first, second);
        assert!(first.get() > 0);
        assert!(second.get() > first.get());
    }

    #[test]
    fn separate_allocators_are_not_assumed_to_be_globally_unique() {
        let first_allocator = TypeIdAllocator::new();
        let second_allocator = TypeIdAllocator::new();

        let first = first_allocator.try_next().expect("first type ID");
        let second = second_allocator.try_next().expect("second type ID");

        // Both allocators begin their own sequence at one.
        // Registry owners must establish the appropriate allocation scope.
        assert_eq!(first, second);
    }

    #[test]
    fn type_id_round_trips_through_serde_json() {
        let allocator = TypeIdAllocator::new();
        let id = allocator.try_next().expect("type ID");

        let encoded = serde_json::to_string(&id).expect("serialize type ID");
        let decoded: TypeId =
            serde_json::from_str(&encoded).expect("deserialize type ID");

        assert_eq!(id, decoded);
    }

    #[test]
    fn type_key_round_trips_through_serde_json() {
        let key = TypeKey::parse("core::numeric::Integer")
            .expect("valid qualified name");

        let encoded = serde_json::to_string(&key).expect("serialize key");
        let decoded: TypeKey =
            serde_json::from_str(&encoded).expect("deserialize key");

        decoded.validate().expect("validate deserialized key");
        assert_eq!(key, decoded);
    }

    #[test]
    fn rejects_malformed_deserialized_type_keys() {
        // QualifiedName is transparent over its segment vector. Deserializing
        // does not necessarily run its constructor, so validate the key
        // before trusting it.
        let decoded: Result<TypeKey, _> =
            serde_json::from_str(r#"["core","","Integer"]"#);

        if let Ok(key) = decoded {
            assert!(key.validate().is_err());
        }
    }
}