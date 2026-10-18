#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Alias Analysis
//!
//! Canonical path: `src/classical/zcir/analysis/alias.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide deterministic, conservative alias analysis for classical ZCIR
//! consumers without introducing a competing IR or identifier system.
//!
//! The analysis determines whether two registered SSA values are known to
//! refer to the same abstract storage, known to refer to disjoint storage,
//! or might refer to overlapping storage.
//!
//! ## Architectural authority
//!
//! - `src/zuir/ids.rs` owns canonical `ValueId` identities.
//! - `src/zuir/ir/use_def.rs` owns canonical definition-use indexing.
//! - `src/zuir/ir/operation.rs` owns operation-local operands and results.
//! - `src/zuir/ir/block.rs` owns block-local structure and terminators.
//! - `src/zuir/ir/region.rs` owns region membership and capture declarations.
//! - Trusted operation and dialect contracts define memory-related semantics.
//! - The semantic model defines source-language meaning.
//! - Whole-IR verification establishes the validity of the underlying IR.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Alias relation queries over registered values.
//! - Explicit must-alias equivalence classes.
//! - Explicit no-alias facts.
//! - Conflict detection between those facts.
//! - Deterministic query behavior and local error reporting.
//!
//! This module does not own:
//!
//! - Value IDs, operation IDs, types, or canonical IR nodes.
//! - Pointer representation, memory layout, or allocation.
//! - Operation semantics or dialect registration.
//! - Definition-use indexing or SSA verification.
//! - Lifetime, ownership, borrow, escape, or concurrency verification.
//! - Interprocedural summaries or points-to inference.
//! - Physical addresses, registers, caches, or target memory.
//! - Source parsing, AST construction, or semantic analysis.
//!
//! ## Soundness contract
//!
//! The caller must register only values whose storage-reference meaning is
//! relevant to the analysis. Registering an arbitrary scalar does not make
//! it a pointer.
//!
//! Unknown relationships default to `MayAlias`, never `NoAlias`.
//!
//! `MustAlias` facts assert equivalence of abstract storage identity.
//! `NoAlias` facts assert disjointness of the represented storage locations.
//! Those facts must be justified by trusted semantics or a sound upstream
//! analysis. This module cannot establish their truth from IDs alone.
//!
//! A conflicting must-alias and no-alias assertion is rejected.
//!
//! A successful query does not prove dominance, value visibility, lifetime
//! safety, race freedom, effect ordering, or operation legality.
//!
//! ## Scalability and determinism
//!
//! There are no fixed limits on the number of registered values, alias
//! classes, or explicit no-alias facts. Storage grows with the supplied
//! facts and available resources.
//!
//! Ordered collections make query behavior and internal iteration
//! deterministic. This module does not promise constant-time operations;
//! ordered-map and ordered-set operations generally depend on collection size.
//!
//! No finite implementation promises literal infinity. POCO-REAF requires
//! avoiding arbitrary universal ceilings while reporting actual constraints.
//!
//! ## Lifecycle
//!
//! This analysis is a derived fact set, not canonical IR state.
//!
//! Rebuild or update it when relevant definitions, reference semantics,
//! escape information, memory effects, or trusted alias facts change.
//! Do not reuse stale facts after IR mutation.
//!
//! The analysis is not automatically derived from `UseDefIndex`: use-def
//! relationships describe value references, not memory aliasing.
//!
//! ## Integration
//!
//! - Import `crate::zuir::ids::ValueId`.
//! - Register reference-bearing values explicitly.
//! - Supply must-alias/no-alias facts from trusted analyses or contracts.
//! - Query before applying optimizations that depend on memory independence.
//! - Preserve conservative behavior when the source of a fact is unknown.
//! - Invoke canonical whole-IR verification independently.
//!
//! Register this module from `src/classical/zcir/analysis/mod.rs` once that
//! module is established and its parent ZCIR module is valid Rust.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::ValueId;

/// Result type for alias-analysis operations.
pub type AliasResult<T> = Result<T, AliasError>;

/// Relationship between two registered values.
///
/// `NoAlias` is a positive proof of disjointness, not a default assumption.
/// `MayAlias` is the conservative answer when neither stronger relationship
/// has been established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AliasRelation {
    /// Both values are known to designate the same abstract storage.
    MustAlias,

    /// The values may designate overlapping storage, or the relationship
    /// has not been established.
    MayAlias,

    /// A trusted fact establishes that the storage locations are disjoint.
    NoAlias,
}

/// Errors returned by alias-analysis operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AliasError {
    /// A value has not been registered with this analysis instance.
    UnknownValue {
        value: ValueId,
    },

    /// A must-alias assertion conflicts with an existing no-alias fact.
    MustAliasConflictsWithNoAlias {
        first: ValueId,
        second: ValueId,
    },

    /// A no-alias assertion conflicts with an existing must-alias class.
    NoAliasConflictsWithMustAlias {
        first: ValueId,
        second: ValueId,
    },
}

impl fmt::Display for AliasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue { value } => {
                write!(
                    f,
                    "value {value} is not registered with alias analysis"
                )
            }
            Self::MustAliasConflictsWithNoAlias { first, second } => {
                write!(
                    f,
                    "must-alias assertion for values {first} and {second} \
                     conflicts with an existing no-alias fact"
                )
            }
            Self::NoAliasConflictsWithMustAlias { first, second } => {
                write!(
                    f,
                    "no-alias assertion for values {first} and {second} \
                     conflicts with an existing must-alias class"
                )
            }
        }
    }
}

impl std::error::Error for AliasError {}

/// An unordered pair of distinct values.
///
/// The lower ID is always stored first so that a relationship has one
/// deterministic representation regardless of argument order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ValuePair {
    first: ValueId,
    second: ValueId,
}

impl ValuePair {
    fn new(first: ValueId, second: ValueId) -> Self {
        if first <= second {
            Self { first, second }
        } else {
            Self {
                first: second,
                second: first,
            }
        }
    }

    fn contains(self, value: ValueId) -> bool {
        self.first == value || self.second == value
    }
}

/// Conservative alias facts over canonical ZUIR value identities.
///
/// This is a derived analysis structure. It is not an alternative value
/// representation or a replacement for canonical IR ownership.
///
/// Must-alias classes are maintained as equivalence classes. Explicit
/// no-alias facts are stored as unordered pairs. All other relationships
/// between registered values conservatively return `MayAlias`.
#[derive(Debug, Clone, Default)]
pub struct AliasAnalysis {
    /// Values deliberately selected for reference/alias analysis.
    registered: BTreeSet<ValueId>,

    /// Maps each registered value to its canonical alias-class representative.
    ///
    /// The representative is the smallest `ValueId` in its class. This
    /// choice is deterministic and does not assign semantic meaning to IDs.
    class_of: BTreeMap<ValueId, ValueId>,

    /// Maps each class representative to all values in that class.
    members: BTreeMap<ValueId, BTreeSet<ValueId>>,

    /// Explicit proofs that two registered values do not alias.
    no_alias: BTreeSet<ValuePair>,
}

impl AliasAnalysis {
    /// Create an empty alias-analysis instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a value whose reference or storage relationship is relevant.
    ///
    /// Registering the same value more than once is idempotent.
    ///
    /// Registration alone establishes no relationship with any other value.
    /// Unproven relationships remain `MayAlias`.
    pub fn register_value(&mut self, value: ValueId) {
        if !self.registered.insert(value) {
            return;
        }

        self.class_of.insert(value, value);

        let mut members = BTreeSet::new();
        members.insert(value);
        self.members.insert(value, members);
    }

    /// Return whether a value has been registered.
    #[must_use]
    pub fn contains_value(&self, value: ValueId) -> bool {
        self.registered.contains(&value)
    }

    /// Declare that two registered values designate the same abstract storage.
    ///
    /// The assertion merges their entire alias classes. If any existing
    /// no-alias fact would become contradictory, this method returns an error
    /// and leaves the analysis unchanged.
    ///
    /// A value aliases itself, so declaring a value must-alias itself is a
    /// valid no-op when that value is registered.
    pub fn declare_must_alias(
        &mut self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<()> {
        self.ensure_registered(first)?;
        self.ensure_registered(second)?;

        if first == second {
            return Ok(());
        }

        let first_class = self.class_of[&first];
        let second_class = self.class_of[&second];

        if first_class == second_class {
            return Ok(());
        }

        // Check all explicit disjointness facts before mutating either class.
        // This avoids a partially applied merge on conflict.
        for pair in &self.no_alias {
            let left_class = self.class_of[&pair.first];
            let right_class = self.class_of[&pair.second];

            let crosses_classes =
                (left_class == first_class && right_class == second_class)
                    || (left_class == second_class
                        && right_class == first_class);

            if crosses_classes {
                return Err(AliasError::MustAliasConflictsWithNoAlias {
                    first,
                    second,
                });
            }
        }

        let representative = first_class.min(second_class);
        let removed_class = first_class.max(second_class);

        // Both classes exist because registration initializes them.
        let mut merged = self
            .members
            .remove(&representative)
            .expect("registered alias class must exist");

        let removed_members = self
            .members
            .remove(&removed_class)
            .expect("registered alias class must exist");

        merged.extend(removed_members.iter().copied());

        for member in &merged {
            self.class_of.insert(*member, representative);
        }

        self.members.insert(representative, merged);

        Ok(())
    }

    /// Declare that two registered values designate disjoint storage.
    ///
    /// If both values are already in the same must-alias class, the assertion
    /// is contradictory and rejected. Otherwise the fact is recorded.
    ///
    /// Repeating an existing no-alias assertion is idempotent.
    pub fn declare_no_alias(
        &mut self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<()> {
        self.ensure_registered(first)?;
        self.ensure_registered(second)?;

        if self.class_of[&first] == self.class_of[&second] {
            return Err(AliasError::NoAliasConflictsWithMustAlias {
                first,
                second,
            });
        }

        self.no_alias.insert(ValuePair::new(first, second));

        Ok(())
    }

    /// Query the established relationship between two registered values.
    ///
    /// The result is symmetric. For registered values:
    ///
    /// - Equal values return `MustAlias`.
    /// - Members of the same must-alias class return `MustAlias`.
    /// - An explicit no-alias fact returns `NoAlias`.
    /// - Every other pair returns `MayAlias`.
    ///
    /// Unknown values return an error rather than silently implying that
    /// the values have been included in the analysis.
    pub fn relation(
        &self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<AliasRelation> {
        self.ensure_registered(first)?;
        self.ensure_registered(second)?;

        if self.class_of[&first] == self.class_of[&second] {
            return Ok(AliasRelation::MustAlias);
        }

        if self.no_alias.contains(&ValuePair::new(first, second)) {
            return Ok(AliasRelation::NoAlias);
        }

        Ok(AliasRelation::MayAlias)
    }

    /// Return whether two registered values may refer to overlapping storage.
    ///
    /// This is the conservative predicate to use when deciding whether an
    /// optimization may reorder or combine memory operations.
    pub fn may_alias(
        &self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<bool> {
        Ok(self.relation(first, second)? != AliasRelation::NoAlias)
    }

    /// Return whether two registered values are proven to designate the
    /// same abstract storage.
    pub fn must_alias(
        &self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<bool> {
        Ok(self.relation(first, second)? == AliasRelation::MustAlias)
    }

    /// Return whether two registered values are proven to designate disjoint
    /// storage.
    pub fn is_no_alias(
        &self,
        first: ValueId,
        second: ValueId,
    ) -> AliasResult<bool> {
        Ok(self.relation(first, second)? == AliasRelation::NoAlias)
    }

    /// Number of registered values.
    #[must_use]
    pub fn value_count(&self) -> usize {
        self.registered.len()
    }

    /// Number of must-alias equivalence classes.
    #[must_use]
    pub fn alias_class_count(&self) -> usize {
        self.members.len()
    }

    /// Number of explicitly recorded no-alias pairs.
    #[must_use]
    pub fn no_alias_fact_count(&self) -> usize {
        self.no_alias.len()
    }

    /// Return the values belonging to the must-alias class of a registered
    /// value, in deterministic `ValueId` order.
    pub fn alias_class(
        &self,
        value: ValueId,
    ) -> AliasResult<impl Iterator<Item = ValueId> + '_> {
        self.ensure_registered(value)?;

        let representative = self.class_of[&value];
        let values = self
            .members
            .get(&representative)
            .expect("registered alias class must exist");

        Ok(values.iter().copied())
    }

    /// Remove all facts and registered values.
    ///
    /// This is useful when discarding an analysis snapshot. It does not
    /// mutate the underlying IR.
    pub fn clear(&mut self) {
        self.registered.clear();
        self.class_of.clear();
        self.members.clear();
        self.no_alias.clear();
    }

    fn ensure_registered(&self, value: ValueId) -> AliasResult<()> {
        if self.registered.contains(&value) {
            Ok(())
        } else {
            Err(AliasError::UnknownValue { value })
        }
    }

    /// Check consistency between registered values, equivalence classes,
    /// and explicit no-alias facts.
    ///
    /// This is a local consistency check, not proof that the supplied facts
    /// correctly model the underlying program's memory behavior.
    pub fn verify_consistency(&self) -> AliasResult<()> {
        if self.registered.len() != self.class_of.len() {
            return Err(AliasError::UnknownValue {
                value: self
                    .registered
                    .iter()
                    .find(|value| !self.class_of.contains_key(value))
                    .copied()
                    .unwrap_or_else(|| {
                        // This branch is only reachable for a malformed
                        // internal state. The structure's public methods
                        // maintain these maps together.
                        self.registered
                            .iter()
                            .next()
                            .copied()
                            .expect("a length mismatch requires a registered value")
                    }),
            });
        }

        for value in &self.registered {
            let representative = self.class_of[value];
            let Some(class_members) = self.members.get(&representative) else {
                return Err(AliasError::UnknownValue { value: *value });
            };

            if !class_members.contains(value) {
                return Err(AliasError::UnknownValue { value: *value });
            }
        }

        for (representative, class_members) in &self.members {
            if class_members.is_empty()
                || !class_members.contains(representative)
            {
                let value = class_members
                    .iter()
                    .next()
                    .copied()
                    .unwrap_or(*representative);

                return Err(AliasError::UnknownValue { value });
            }

            for value in class_members {
                if self.class_of.get(value) != Some(representative) {
                    return Err(AliasError::UnknownValue { value: *value });
                }
            }
        }

        for pair in &self.no_alias {
            self.ensure_registered(pair.first)?;
            self.ensure_registered(pair.second)?;

            if self.class_of[&pair.first] == self.class_of[&pair.second] {
                return Err(AliasError::NoAliasConflictsWithMustAlias {
                    first: pair.first,
                    second: pair.second,
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn registered(values: &[ValueId]) -> AliasAnalysis {
        let mut analysis = AliasAnalysis::new();

        for value in values {
            analysis.register_value(*value);
        }

        analysis
    }

    #[test]
    fn new_analysis_is_empty() {
        let analysis = AliasAnalysis::new();

        assert_eq!(analysis.value_count(), 0);
        assert_eq!(analysis.alias_class_count(), 0);
        assert_eq!(analysis.no_alias_fact_count(), 0);
        assert!(analysis.verify_consistency().is_ok());
    }

    #[test]
    fn registration_is_idempotent() {
        let first = value(1);
        let mut analysis = AliasAnalysis::new();

        analysis.register_value(first);
        analysis.register_value(first);

        assert_eq!(analysis.value_count(), 1);
        assert_eq!(analysis.alias_class_count(), 1);
    }

    #[test]
    fn unproven_relationship_is_may_alias() {
        let first = value(1);
        let second = value(2);
        let analysis = registered(&[first, second]);

        assert_eq!(
            analysis.relation(first, second).unwrap(),
            AliasRelation::MayAlias
        );
        assert!(analysis.may_alias(first, second).unwrap());
        assert!(!analysis.must_alias(first, second).unwrap());
        assert!(!analysis.is_no_alias(first, second).unwrap());
    }

    #[test]
    fn a_value_must_alias_itself() {
        let first = value(1);
        let analysis = registered(&[first]);

        assert_eq!(
            analysis.relation(first, first).unwrap(),
            AliasRelation::MustAlias
        );
    }

    #[test]
    fn must_alias_merges_equivalence_classes() {
        let first = value(1);
        let second = value(2);
        let third = value(3);
        let mut analysis = registered(&[first, second, third]);

        analysis.declare_must_alias(first, second).unwrap();
        analysis.declare_must_alias(second, third).unwrap();

        assert_eq!(analysis.alias_class_count(), 1);
        assert!(analysis.must_alias(first, third).unwrap());
        assert!(analysis.must_alias(second, first).unwrap());
        assert_eq!(
            analysis.alias_class(third).unwrap().collect::<Vec<_>>(),
            vec![first, second, third]
        );
        assert!(analysis.verify_consistency().is_ok());
    }

    #[test]
    fn no_alias_is_symmetric() {
        let first = value(1);
        let second = value(2);
        let mut analysis = registered(&[first, second]);

        analysis.declare_no_alias(first, second).unwrap();

        assert!(analysis.is_no_alias(first, second).unwrap());
        assert!(analysis.is_no_alias(second, first).unwrap());
        assert!(!analysis.may_alias(first, second).unwrap());
        assert_eq!(analysis.no_alias_fact_count(), 1);
    }

    #[test]
    fn duplicate_no_alias_fact_is_idempotent() {
        let first = value(1);
        let second = value(2);
        let mut analysis = registered(&[first, second]);

        analysis.declare_no_alias(first, second).unwrap();
        analysis.declare_no_alias(second, first).unwrap();

        assert_eq!(analysis.no_alias_fact_count(), 1);
    }

    #[test]
    fn conflicting_no_alias_fact_is_rejected() {
        let first = value(1);
        let second = value(2);
        let mut analysis = registered(&[first, second]);

        analysis.declare_must_alias(first, second).unwrap();

        assert_eq!(
            analysis.declare_no_alias(first, second),
            Err(AliasError::NoAliasConflictsWithMustAlias {
                first,
                second,
            })
        );

        assert!(analysis.must_alias(first, second).unwrap());
        assert!(analysis.verify_consistency().is_ok());
    }

    #[test]
    fn conflicting_must_alias_merge_is_rejected_without_mutation() {
        let first = value(1);
        let second = value(2);
        let third = value(3);
        let fourth = value(4);
        let mut analysis = registered(&[first, second, third, fourth]);

        analysis.declare_must_alias(first, second).unwrap();
        analysis.declare_must_alias(third, fourth).unwrap();
        analysis.declare_no_alias(second, fourth).unwrap();

        let before_class_count = analysis.alias_class_count();
        let before_fact_count = analysis.no_alias_fact_count();

        assert_eq!(
            analysis.declare_must_alias(first, third),
            Err(AliasError::MustAliasConflictsWithNoAlias {
                first,
                second: third,
            })
        );

        assert_eq!(analysis.alias_class_count(), before_class_count);
        assert_eq!(analysis.no_alias_fact_count(), before_fact_count);
        assert!(!analysis.must_alias(first, third).unwrap());
        assert!(analysis.verify_consistency().is_ok());
    }

    #[test]
    fn unknown_values_are_not_silently_accepted() {
        let first = value(1);
        let unknown = value(2);
        let analysis = registered(&[first]);

        assert_eq!(
            analysis.relation(first, unknown),
            Err(AliasError::UnknownValue { value: unknown })
        );
        assert_eq!(
            analysis.declare_no_alias(first, unknown),
            Err(AliasError::UnknownValue { value: unknown })
        );
    }

    #[test]
    fn a_conflicting_merge_does_not_partially_change_classes() {
        let first = value(1);
        let second = value(2);
        let third = value(3);
        let mut analysis = registered(&[first, second, third]);

        analysis.declare_no_alias(second, third).unwrap();

        let before_classes = analysis.alias_class_count();

        assert!(analysis.declare_must_alias(first, second).is_ok());
        assert_eq!(analysis.alias_class_count(), before_classes - 1);

        assert_eq!(
            analysis.declare_must_alias(first, third),
            Err(AliasError::MustAliasConflictsWithNoAlias {
                first,
                second: third,
            })
        );

        assert!(analysis.must_alias(first, second).unwrap());
        assert_eq!(
            analysis.relation(first, third).unwrap(),
            AliasRelation::MayAlias
        );
        assert!(analysis.verify_consistency().is_ok());
    }

    #[test]
    fn clearing_removes_all_facts() {
        let first = value(1);
        let second = value(2);
        let mut analysis = registered(&[first, second]);

        analysis.declare_must_alias(first, second).unwrap();
        analysis.clear();

        assert_eq!(analysis.value_count(), 0);
        assert_eq!(analysis.alias_class_count(), 0);
        assert_eq!(analysis.no_alias_fact_count(), 0);
        assert!(analysis.verify_consistency().is_ok());
        assert!(matches!(
            analysis.relation(first, second),
            Err(AliasError::UnknownValue { .. })
        ));
    }

    #[test]
    fn fact_insertion_order_does_not_change_query_results() {
        let first = value(1);
        let second = value(2);
        let third = value(3);

        let mut left = registered(&[first, second, third]);
        left.declare_must_alias(first, second).unwrap();
        left.declare_no_alias(first, third).unwrap();

        let mut right = registered(&[third, second, first]);
        right.declare_no_alias(third, first).unwrap();
        right.declare_must_alias(second, first).unwrap();

        for a in [first, second, third] {
            for b in [first, second, third] {
                assert_eq!(
                    left.relation(a, b).unwrap(),
                    right.relation(a, b).unwrap()
                );
            }
        }
    }
}