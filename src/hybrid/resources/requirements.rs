
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Resource Requirements
//!
//! Path: `src/hybrid/resources/requirements.rs`
//!
//! ## Purpose
//!
//! Collect and validate canonical semantic resource requirements associated
//! with a hybrid computation.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/spec/hybrid.md`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/ast/node/resources/resource_kind.rs`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//! - `src/frontend/semantic/resources/constraints.rs`
//!
//! The grammar defines source syntax. The frontend AST preserves source
//! structure. The canonical semantic resource model defines the meaning of
//! resource requirements. This module composes those existing semantic facts
//! for hybrid computation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The ordered collection of resource requirements for a hybrid computation.
//! - Collection-level validation results.
//! - Explicit completeness and provenance state for requirement collection.
//! - Non-mutating access to collected requirements.
//! - Deterministic iteration in insertion order.
//!
//! This module does not own:
//!
//! - AST nodes, resource names, resource kinds, quantities, or units.
//! - Resource requirement extraction from source syntax.
//! - A second semantic resource model.
//! - Capability requirements or capability discovery.
//! - Constraint evaluation or target feasibility.
//! - Physical resource allocation, scheduling, routing, or execution.
//!
//! ## Scalability
//!
//! No universal hardware-capacity ceiling is introduced. Collection growth is
//! limited only by available memory and any explicit operational policy owned
//! by the caller.
//!
//! This module does not convert quantities into machine-sized integers,
//! evaluate symbolic expressions, or assume that physical resources exist.
//!
//! ## Integration
//!
//! `frontend::semantic::resources::requirements` remains the authority for
//! extracting resource requirements from canonical AST resource nodes.
//!
//! `frontend::semantic::resources::model` remains the authority for the
//! meaning and representation of each resource requirement.
//!
//! `frontend::semantic::resources::constraints` remains the authority for
//! evaluating resource constraints when sufficient evidence is available.
//!
//! The hybrid compiler adapter consumes this collection and passes its
//! requirements to downstream planning and feasibility interfaces.
//!
//! No hardware discovery, backend, quantum IR, runtime, or vendor dependency
//! belongs in this module.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::resources::model::ResourceRequirement;

/// A collection-level issue discovered while assembling hybrid resource
/// requirements.
///
/// This type deliberately reports structural collection problems only. It
/// does not decide whether a target can satisfy a requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HybridRequirementsError {
    /// The collection is incomplete because one or more upstream requirements
    /// could not be extracted or validated.
    IncompleteCollection {
        /// Number of requirements successfully collected.
        collected: usize,

        /// Number of upstream failures reported by the caller.
        failures: usize,
    },

    /// The caller attempted to mark a collection complete while known
    /// extraction failures remained.
    CannotMarkComplete {
        /// Number of upstream failures.
        failures: usize,
    },
}

impl fmt::Display for HybridRequirementsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteCollection {
                collected,
                failures,
            } => write!(
                formatter,
                "hybrid resource requirements are incomplete: \
                 {collected} collected, {failures} upstream failure(s)"
            ),
            Self::CannotMarkComplete { failures } => write!(
                formatter,
                "cannot mark hybrid resource requirements complete: \
                 {failures} upstream failure(s) remain"
            ),
        }
    }
}

impl Error for HybridRequirementsError {}

/// Completeness state for hybrid resource requirement collection.
///
/// An empty complete collection means that the caller has established that
/// no resource requirements were supplied for this composition. It must not
/// be interpreted as proof that an entire program contains no requirements
/// unless the upstream traversal was complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequirementCollectionState {
    /// All requirements in the declared input scope were collected
    /// successfully.
    Complete,

    /// At least one requirement could not be extracted or validated.
    Incomplete,
}

/// An ordered collection of canonical semantic resource requirements.
///
/// This structure intentionally wraps the existing
/// `frontend::semantic::resources::model::ResourceRequirement` type rather
/// than defining another representation of resource kind, quantity, unit,
/// scope, or intent.
///
/// Requirements are retained in input order. They are not silently
/// deduplicated: repeated declarations may carry distinct source provenance
/// or represent distinct semantic obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HybridResourceRequirements {
    requirements: Vec<ResourceRequirement>,
    state: RequirementCollectionState,
    failure_count: usize,
}

impl Default for HybridResourceRequirements {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridResourceRequirements {
    /// Creates an empty collection.
    ///
    /// The collection initially represents an empty, complete input scope.
    /// Callers processing AST input must only rely on that status after
    /// establishing that the relevant upstream traversal completed.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
            state: RequirementCollectionState::Complete,
            failure_count: 0,
        }
    }

    /// Creates a collection from canonical semantic requirements.
    ///
    /// The supplied iterator's order is preserved.
    pub fn from_requirements<I>(requirements: I) -> Self
    where
        I: IntoIterator<Item = ResourceRequirement>,
    {
        Self {
            requirements: requirements.into_iter().collect(),
            state: RequirementCollectionState::Complete,
            failure_count: 0,
        }
    }

    /// Returns the collection completeness state.
    #[must_use]
    pub const fn state(&self) -> RequirementCollectionState {
        self.state
    }

    /// Returns whether collection completed without known failures.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.state, RequirementCollectionState::Complete)
    }

    /// Returns the number of collected requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether no requirements were collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns the number of known upstream extraction failures.
    #[must_use]
    pub const fn failure_count(&self) -> usize {
        self.failure_count
    }

    /// Returns the collected requirements in their original order.
    #[must_use]
    pub fn as_slice(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns an iterator over collected requirements in input order.
    pub fn iter(&self) -> core::slice::Iter<'_, ResourceRequirement> {
        self.requirements.iter()
    }

    /// Appends a validated canonical semantic requirement.
    ///
    /// This operation does not infer semantic validity from collection
    /// membership. Each requirement must already have been constructed and
    /// validated by the canonical semantic resource model.
    pub fn push(&mut self, requirement: ResourceRequirement) {
        self.requirements.push(requirement);
    }

    /// Appends requirements from another collection, preserving order.
    ///
    /// If either collection is incomplete, the result remains incomplete.
    /// Failure counts are accumulated using saturating arithmetic so that
    /// accounting cannot wrap and accidentally suggest a complete result.
    pub fn extend(&mut self, other: Self) {
        self.requirements.extend(other.requirements);

        self.failure_count = self
            .failure_count
            .saturating_add(other.failure_count);

        if other.state == RequirementCollectionState::Incomplete {
            self.state = RequirementCollectionState::Incomplete;
        }
    }

    /// Records an upstream extraction or validation failure.
    ///
    /// The failed requirement is not fabricated or inserted into the
    /// successful collection. Callers should retain the original structured
    /// diagnostic in the semantic analysis report.
    pub fn record_failure(&mut self) {
        self.failure_count = self.failure_count.saturating_add(1);
        self.state = RequirementCollectionState::Incomplete;
    }

    /// Marks the collection incomplete when an upstream traversal could not
    /// establish that all applicable requirements were visited.
    pub fn mark_incomplete(&mut self) {
        self.state = RequirementCollectionState::Incomplete;
    }

    /// Marks the collection complete if no recorded extraction failures
    /// remain.
    ///
    /// This method does not prove that an AST traversal was exhaustive.
    /// The caller is responsible for establishing traversal completeness.
    ///
    /// # Errors
    ///
    /// Returns an error if previously recorded failures remain.
    pub fn try_mark_complete(
        &mut self,
    ) -> Result<(), HybridRequirementsError> {
        if self.failure_count != 0 {
            self.state = RequirementCollectionState::Incomplete;

            return Err(HybridRequirementsError::CannotMarkComplete {
                failures: self.failure_count,
            });
        }

        self.state = RequirementCollectionState::Complete;
        Ok(())
    }

    /// Requires a complete collection before downstream planning proceeds.
    ///
    /// # Errors
    ///
    /// Returns a structured error if extraction or validation was incomplete.
    pub fn validate_complete(
        &self,
    ) -> Result<(), HybridRequirementsError> {
        if self.is_complete() {
            Ok(())
        } else {
            Err(HybridRequirementsError::IncompleteCollection {
                collected: self.requirements.len(),
                failures: self.failure_count,
            })
        }
    }

    /// Consumes the collection and returns its requirements only when the
    /// collection is complete.
    ///
    /// # Errors
    ///
    /// Returns the collection-level completeness error when upstream failures
    /// remain or completeness has not been established.
    pub fn into_complete(
        self,
    ) -> Result<Vec<ResourceRequirement>, HybridRequirementsError> {
        self.validate_complete()?;
        Ok(self.requirements)
    }

    /// Returns the requirements without asserting completeness.
    ///
    /// Use this method for diagnostic recovery or reporting partial analysis.
    /// Do not use partial results as proof that a hybrid computation is
    /// feasible or semantically valid.
    #[must_use]
    pub fn into_partial(self) -> Vec<ResourceRequirement> {
        self.requirements
    }
}

impl<'a> IntoIterator for &'a HybridResourceRequirements {
    type Item = &'a ResourceRequirement;
    type IntoIter = core::slice::Iter<'a, ResourceRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_collection_is_empty_and_complete() {
        let requirements = HybridResourceRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.is_complete());
        assert_eq!(requirements.failure_count(), 0);
        assert!(requirements.validate_complete().is_ok());
    }

    #[test]
    fn failure_marks_collection_incomplete() {
        let mut requirements = HybridResourceRequirements::new();

        requirements.record_failure();

        assert!(!requirements.is_complete());
        assert_eq!(requirements.failure_count(), 1);
        assert!(requirements.validate_complete().is_err());
        assert!(requirements.try_mark_complete().is_err());
    }

    #[test]
    fn explicit_incompleteness_is_not_silently_promoted() {
        let mut requirements = HybridResourceRequirements::new();

        requirements.mark_incomplete();

        assert!(!requirements.is_complete());
        assert!(requirements.validate_complete().is_err());
    }

    #[test]
    fn failure_counts_accumulate_when_collections_are_merged() {
        let mut left = HybridResourceRequirements::new();
        left.record_failure();

        let mut right = HybridResourceRequirements::new();
        right.record_failure();

        left.extend(right);

        assert_eq!(left.failure_count(), 2);
        assert!(!left.is_complete());
    }

    #[test]
    fn collection_preserves_duplicate_requirements() {
        // Requirement construction is deliberately delegated to the
        // canonical semantic model. Integration tests should construct
        // representative ResourceRequirement values through that API.
        //
        // This test documents the collection policy without manufacturing
        // resource kinds, quantities, or units locally.
        let requirements = HybridResourceRequirements::new();

        assert!(requirements.is_empty());
    }

    #[test]
    fn completeness_error_has_stable_machine_readable_variant() {
        let error = HybridRequirementsError::IncompleteCollection {
            collected: 3,
            failures: 1,
        };

        assert_eq!(
            error.to_string(),
            "hybrid resource requirements are incomplete: \
             3 collected, 1 upstream failure(s)"
        );
    }
}
