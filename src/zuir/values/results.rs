
#![forbid(unsafe_code)]

//! # ZUIR value results
//!
//! PURPOSE:
//! Represent values defined by operations in the canonical Zamani ZUIR.
//!
//! STATUS:
//! Proposed implementation. Freeze only after integration with the repository's
//! authoritative ID, type, operation, and error contracts.
//!
//! AUTHORITY:
//! The normative ZUIR specification and the contracts for identifiers,
//! types, operations, and SSA/value semantics.
//!
//! OWNS:
//! - Result definitions.
//! - The association between a result, its defining operation, its type,
//!   and its position in that operation's result sequence.
//! - Ordered result collections and their derived value-ID index.
//! - Local structural validation of result collections.
//!
//! DOES NOT OWN:
//! - Global ID allocation or ID uniqueness across an entire program.
//! - Type definition, type compatibility, or type inference.
//! - Operation semantics, dominance, use-def verification, or control flow.
//! - Physical registers, memory addresses, quantum hardware resources,
//!   HDL signal allocation, or runtime storage.
//! - Serialization formats or source-level name resolution.
//!
//! INTEGRATION:
//! Use the repository's existing ValueId, OperationId, and TypeId as the
//! generic parameters. Do not define duplicate global identifier types here.
//!
//! SCALABILITY:
//! Uses dynamically sized collections and checked construction rules.
//! It defines no fixed operation-result count or hardware-size limit.
//! Actual growth remains subject to available memory and configured policies.
//!
//! THREAD SAFETY:
//! This module does not introduce shared mutable global state.
//!
//! SAFETY:
//! No unsafe Rust is used.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// The position of a result in its defining operation's result sequence.
///
/// This is a logical index, not a hardware register number.
pub type ResultIndex = usize;

/// A single value definition produced by an operation.
///
/// `V`, `O`, and `T` should be the repository's canonical value ID,
/// operation ID, and type ID respectively.
///
/// The IDs are generic deliberately: this module does not allocate IDs
/// or establish a competing identity system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultDefinition<V, O, T> {
    value_id: V,
    defining_operation: O,
    type_id: T,
    index: ResultIndex,
}

impl<V, O, T> ResultDefinition<V, O, T> {
    /// Construct a result definition.
    ///
    /// This constructor records the supplied facts. Structural consistency
    /// is enforced when the definition is inserted into a result collection.
    pub const fn new(
        value_id: V,
        defining_operation: O,
        type_id: T,
        index: ResultIndex,
    ) -> Self {
        Self {
            value_id,
            defining_operation,
            type_id,
            index,
        }
    }

    /// Return the identity of the value defined by this result.
    pub const fn value_id(&self) -> &V {
        &self.value_id
    }

    /// Return the operation that defines this result.
    pub const fn defining_operation(&self) -> &O {
        &self.defining_operation
    }

    /// Return the declared type identity.
    ///
    /// This does not prove that the type exists or that it is semantically
    /// valid. Those checks belong to the type system and verifier.
    pub const fn type_id(&self) -> &T {
        &self.type_id
    }

    /// Return this result's position within its defining operation.
    pub const fn index(&self) -> ResultIndex {
        self.index
    }
}

/// Failures detected while constructing or validating result collections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultCollectionError {
    /// The result claims an operation other than the collection's owner.
    DefiningOperationMismatch,

    /// The result position is not the next expected position.
    ///
    /// This implementation requires result positions to be contiguous and
    /// zero-based within each operation.
    UnexpectedResultIndex {
        expected: ResultIndex,
        actual: ResultIndex,
    },

    /// The same value identity was inserted more than once into a collection.
    DuplicateValueId,

    /// A collection's derived index does not match its stored definitions.
    IndexInconsistency,

    /// The collection contains an entry that violates its structural rules.
    InvalidCollection,
}

impl fmt::Display for ResultCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefiningOperationMismatch => {
                write!(
                    f,
                    "result definition belongs to a different operation"
                )
            }
            Self::UnexpectedResultIndex { expected, actual } => {
                write!(
                    f,
                    "unexpected result index: expected {expected}, got {actual}"
                )
            }
            Self::DuplicateValueId => {
                write!(f, "duplicate value identity in operation results")
            }
            Self::IndexInconsistency => {
                write!(f, "result collection index is inconsistent")
            }
            Self::InvalidCollection => {
                write!(f, "result collection violates structural invariants")
            }
        }
    }
}

impl Error for ResultCollectionError {}

/// An ordered collection of results produced by one operation.
///
/// The vector is the canonical ordered representation. The map is a derived
/// lookup index; callers cannot mutate either structure independently.
///
/// Invariants:
/// - Every result belongs to `owner`.
/// - Result indices are contiguous, starting at zero.
/// - Every value ID occurs at most once in this collection.
/// - The lookup index agrees with the ordered definitions.
///
/// These are local invariants. Program-wide uniqueness, type validity,
/// definition/use integrity, and dominance must be checked by the verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationResults<V, O, T> {
    owner: O,
    definitions: Vec<ResultDefinition<V, O, T>>,
    by_value: BTreeMap<V, ResultIndex>,
}

impl<V, O, T> OperationResults<V, O, T>
where
    V: Ord + Clone,
    O: Eq,
{
    /// Create an empty result collection for an operation.
    pub fn new(owner: O) -> Self {
        Self {
            owner,
            definitions: Vec::new(),
            by_value: BTreeMap::new(),
        }
    }

    /// Return the operation that owns this collection.
    pub const fn owner(&self) -> &O {
        &self.owner
    }

    /// Return the number of results.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Return whether the operation has no results.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Insert a result while preserving all collection invariants.
    ///
    /// Results must be inserted in index order. This makes the stored order
    /// explicit and deterministic rather than silently sorting malformed data.
    pub fn insert(
        &mut self,
        definition: ResultDefinition<V, O, T>,
    ) -> Result<(), ResultCollectionError> {
        if definition.defining_operation != self.owner {
            return Err(ResultCollectionError::DefiningOperationMismatch);
        }

        let expected = self.definitions.len();

        if definition.index != expected {
            return Err(ResultCollectionError::UnexpectedResultIndex {
                expected,
                actual: definition.index,
            });
        }

        if self.by_value.contains_key(&definition.value_id) {
            return Err(ResultCollectionError::DuplicateValueId);
        }

        // Reserve the next vector slot before modifying the index. If reserve
        // fails due to allocation exhaustion, Rust's allocation behavior
        // applies; this API does not silently truncate the collection.
        self.definitions.reserve(1);

        // `expected` is the position the definition must occupy.
        self.by_value
            .insert(definition.value_id.clone(), expected);
        self.definitions.push(definition);

        Ok(())
    }

    /// Look up a result by its value identity.
    pub fn get_by_value(&self, value_id: &V) -> Option<&ResultDefinition<V, O, T>> {
        let index = *self.by_value.get(value_id)?;
        self.definitions.get(index)
    }

    /// Look up a result by its operation-local position.
    pub fn get_by_index(
        &self,
        index: ResultIndex,
    ) -> Option<&ResultDefinition<V, O, T>> {
        self.definitions.get(index)
    }

    /// Iterate over results in their canonical operation-local order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &ResultDefinition<V, O, T>> {
        self.definitions.iter()
    }

    /// Iterate over value identities in result order.
    pub fn value_ids(&self) -> impl ExactSizeIterator<Item = &V> {
        self.definitions.iter().map(ResultDefinition::value_id)
    }

    /// Validate all local structural invariants.
    ///
    /// Run this at trust boundaries, after decoding external data, and from
    /// the authoritative IR verifier. Normal insertion already enforces
    /// these rules for collections built exclusively through this API.
    pub fn validate(&self) -> Result<(), ResultCollectionError> {
        if self.definitions.len() != self.by_value.len() {
            return Err(ResultCollectionError::IndexInconsistency);
        }

        for (expected, definition) in self.definitions.iter().enumerate() {
            if definition.defining_operation != self.owner {
                return Err(ResultCollectionError::DefiningOperationMismatch);
            }

            if definition.index != expected {
                return Err(ResultCollectionError::UnexpectedResultIndex {
                    expected,
                    actual: definition.index,
                });
            }

            match self.by_value.get(&definition.value_id) {
                Some(index) if *index == expected => {}
                _ => return Err(ResultCollectionError::IndexInconsistency),
            }
        }

        Ok(())
    }

    /// Consume the collection and return its ordered definitions.
    ///
    /// The derived lookup index is discarded because it can be rebuilt.
    pub fn into_definitions(self) -> Vec<ResultDefinition<V, O, T>> {
        self.definitions
    }

    /// Build a collection from ordered definitions, validating every entry.
    pub fn try_from_ordered(
        owner: O,
        definitions: impl IntoIterator<Item = ResultDefinition<V, O, T>>,
    ) -> Result<Self, ResultCollectionError> {
        let mut results = Self::new(owner);

        for definition in definitions {
            results.insert(definition)?;
        }

        results.validate()?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-only identifier types. Production callers should use the
    // repository's canonical ID types.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestValueId(u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestOperationId(u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestTypeId(u64);

    fn result(
        value: u64,
        operation: u64,
        ty: u64,
        index: usize,
    ) -> ResultDefinition<TestValueId, TestOperationId, TestTypeId> {
        ResultDefinition::new(
            TestValueId(value),
            TestOperationId(operation),
            TestTypeId(ty),
            index,
        )
    }

    #[test]
    fn accepts_ordered_results_and_looks_them_up() {
        let mut results = OperationResults::new(TestOperationId(7));

        results.insert(result(10, 7, 100, 0)).unwrap();
        results.insert(result(11, 7, 101, 1)).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results.get_by_index(1).unwrap().value_id(), &TestValueId(11));
        assert_eq!(
            results.get_by_value(&TestValueId(10)).unwrap().index(),
            0
        );
        assert!(results.validate().is_ok());
    }

    #[test]
    fn rejects_a_result_from_another_operation() {
        let mut results = OperationResults::new(TestOperationId(7));

        let error = results.insert(result(10, 8, 100, 0)).unwrap_err();

        assert_eq!(
            error,
            ResultCollectionError::DefiningOperationMismatch
        );
        assert!(results.is_empty());
    }

    #[test]
    fn rejects_non_contiguous_result_positions() {
        let mut results = OperationResults::new(TestOperationId(7));

        let error = results.insert(result(10, 7, 100, 1)).unwrap_err();

        assert_eq!(
            error,
            ResultCollectionError::UnexpectedResultIndex {
                expected: 0,
                actual: 1,
            }
        );
    }

    #[test]
    fn rejects_duplicate_value_identities() {
        let mut results = OperationResults::new(TestOperationId(7));

        results.insert(result(10, 7, 100, 0)).unwrap();

        // The next position is correct, but the value identity is duplicated.
        let error = results.insert(result(10, 7, 101, 1)).unwrap_err();

        assert_eq!(error, ResultCollectionError::DuplicateValueId);
        assert_eq!(results.len(), 1);
        assert!(results.validate().is_ok());
    }

    #[test]
    fn supports_operations_with_no_results() {
        let results: OperationResults<TestValueId, TestOperationId, TestTypeId> =
            OperationResults::new(TestOperationId(7));

        assert!(results.is_empty());
        assert!(results.validate().is_ok());
        assert!(results.get_by_index(0).is_none());
    }

    #[test]
    fn reconstructs_a_collection_from_ordered_definitions() {
        let results = OperationResults::try_from_ordered(
            TestOperationId(7),
            vec![result(10, 7, 100, 0), result(11, 7, 101, 1)],
        )
        .unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.validate().is_ok());
    }

    #[test]
    fn rejects_malformed_ordered_input() {
        let error = OperationResults::try_from_ordered(
            TestOperationId(7),
            vec![result(10, 7, 100, 0), result(11, 7, 101, 2)],
        )
        .unwrap_err();

        assert_eq!(
            error,
            ResultCollectionError::UnexpectedResultIndex {
                expected: 1,
                actual: 2,
            }
        );
    }
}
