
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # ZCIR Capability Integration
//!
//! Path: `src/classical/zcir/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the classical-facing capability integration boundary for ZCIR
//! while reusing the canonical capability-reference and requirement types
//! defined by ZUIR.
//!
//! This module represents capability requirements and validates their local
//! structure. It does not determine whether a target provides a capability.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/resources/resource-expressions.g4`
//! - `grammar/quantum/resource-requirements.g4`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/values/capability.rs`
//! - `src/zuir/ir/resources.rs`
//! - `src/frontend/ast/node/capabilities/`
//! - `src/frontend/semantic/capabilities/model.rs`
//!
//! The language and semantic specifications govern meaning. ZUIR owns the
//! shared IR-level capability-reference and requirement representations.
//!
//! ## Owns
//!
//! - The ZCIR-facing capability integration API.
//! - Ordered aggregation of classical capability requirements.
//! - Explicit validation budgets for requirement collections.
//! - Deterministic, indexed reporting of local validation failures.
//!
//! ## Does not own
//!
//! - A second capability identity, version, property, or requirement model.
//! - Source syntax, AST nodes, or semantic name resolution.
//! - A closed enumeration of capability names or computational domains.
//! - Target capability inventories or hardware discovery.
//! - Authorization, trust, permissions, or runtime credentials.
//! - Physical resource allocation, scheduling, placement, or routing.
//! - Target feasibility or a claim that a capability is available.
//! - ZUIR's canonical IR, quantum IR, or backend representations.
//!
//! ## Dependency direction
//!
//! ```text
//! source grammar
//!     -> frontend AST
//!     -> semantic capability model
//!     -> canonical ZUIR capability requirements
//!     -> ZCIR classical integration
//!     -> target feasibility and lowering
//! ```
//!
//! This module must not be imported by the frontend AST or semantic model.
//!
//! ## Integration contract
//!
//! - `src/zuir/values/capability.rs` owns `CapabilityRef`,
//!   `CapabilityRequirement`, version constraints, property values,
//!   validation budgets, and their validation errors.
//! - `src/zuir/values/mod.rs` must register and re-export those canonical
//!   definitions before this module is compiled.
//! - `src/classical/zcir/mod.rs` declares `pub mod capabilities;`.
//! - `src/classical/mod.rs` registers `pub mod zcir;` only when the ZCIR
//!   module root is a valid Rust source file and its required children exist.
//! - Frontend adapters convert already-resolved semantic requirements into
//!   canonical ZUIR requirements before passing them to this boundary.
//! - Target-planning code compares requirements with explicit target evidence.
//! - ZCIR verification may validate these records but must not treat local
//!   validation as proof of target support.
//!
//! ## Scalability
//!
//! There is no hard-coded maximum capability count, namespace depth,
//! property count, domain count, or hardware population.
//!
//! The collection grows according to available memory. Optional budgets are
//! operational safeguards, not restrictions on the Zamani language.
//! Exhaustion is reported explicitly; requirements are never silently dropped.
//!
//! This module does not promise literal infinity. Rust representations,
//! process memory, execution time, and physical resources remain finite.
//!
//! ## Determinism
//!
//! Requirements retain insertion order. Validation reports the first failing
//! requirement in that order. No environment probing, hardware discovery,
//! global mutation, randomization, or implicit capability registration occurs.
//!
//! ## Freeze criteria
//!
//! Freeze this file only after the canonical ZUIR capability module is
//! registered, the ZCIR module root is valid, and the repository build,
//! tests, formatting, and lint checks pass.

use std::error::Error;
use std::fmt;

pub use crate::zuir::values::capability::{
    CapabilityRef,
    CapabilityRequirement,
    CapabilityValidationBudget,
    CapabilityValidationError,
    CapabilityValue,
    CapabilityVersion,
    CapabilityVersionConstraint,
};

/// Result type for ZCIR capability collection operations.
pub type ZcirCapabilitiesResult<T> = Result<T, ZcirCapabilitiesError>;

/// Explicit operational budgets for validating a collection of capability
/// requirements.
///
/// `max_requirements` bounds the number of collection entries examined.
/// `per_requirement` bounds the structural validation work for each entry.
/// `None` means no explicit limit for that dimension.
///
/// These limits are caller-selected operational policies, not language-level
/// capability-count limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ZcirCapabilitiesValidationBudget {
    /// Maximum number of requirement entries to inspect.
    pub max_requirements: Option<usize>,

    /// Structural validation budget for each requirement.
    pub per_requirement: CapabilityValidationBudget,
}

impl ZcirCapabilitiesValidationBudget {
    /// Construct a budget without explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_requirements: None,
            per_requirement: CapabilityValidationBudget::unlimited(),
        }
    }

    /// Construct a budget with a maximum entry count and per-requirement
    /// structural validation budgets.
    #[must_use]
    pub const fn bounded(
        max_requirements: usize,
        per_requirement: CapabilityValidationBudget,
    ) -> Self {
        Self {
            max_requirements: Some(max_requirements),
            per_requirement,
        }
    }
}

/// Ordered collection of canonical ZUIR capability requirements.
///
/// This is intentionally not named a mathematical set: duplicate
/// declarations and their original order are preserved for diagnostics and
/// later semantic analysis. Whether duplicates conflict or are redundant is
/// decided by the appropriate semantic or IR verification layer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZcirCapabilityRequirements {
    entries: Vec<CapabilityRequirement>,
}

impl ZcirCapabilityRequirements {
    /// Create an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Construct a collection from existing canonical requirements.
    ///
    /// This constructor preserves the supplied order and duplicates.
    /// Call `validate` before trusting externally supplied entries.
    #[must_use]
    pub fn from_entries(entries: Vec<CapabilityRequirement>) -> Self {
        Self { entries }
    }

    /// Append a requirement without changing its meaning or deduplicating it.
    pub fn push(&mut self, requirement: CapabilityRequirement) {
        self.entries.push(requirement);
    }

    /// Return the number of requirement entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the collection contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Borrow the requirements in declaration/insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[CapabilityRequirement] {
        &self.entries
    }

    /// Consume the collection and return its ordered entries.
    #[must_use]
    pub fn into_entries(self) -> Vec<CapabilityRequirement> {
        self.entries
    }

    /// Append all entries from another collection, preserving order.
    ///
    /// No attempt is made to merge or discard duplicate requirements.
    pub fn append(&mut self, other: &mut Self) {
        self.entries.append(&mut other.entries);
    }

    /// Validate every requirement with no explicit operational budget.
    ///
    /// This checks local structure only. It does not establish that a
    /// capability is registered, supported, authorized, or feasible on any
    /// target.
    pub fn validate(&self) -> ZcirCapabilitiesResult<()> {
        self.validate_with(&ZcirCapabilitiesValidationBudget::unlimited())
    }

    /// Validate entries under an explicit operational budget.
    ///
    /// On failure, the error identifies the first failing entry in stable
    /// insertion order. No entries are removed or rewritten.
    pub fn validate_with(
        &self,
        budget: &ZcirCapabilitiesValidationBudget,
    ) -> ZcirCapabilitiesResult<()> {
        if let Some(limit) = budget.max_requirements {
            if self.entries.len() > limit {
                return Err(ZcirCapabilitiesError::RequirementBudgetExceeded {
                    limit,
                    actual: self.entries.len(),
                });
            }
        }

        for (index, requirement) in self.entries.iter().enumerate() {
            requirement
                .validate_with(&budget.per_requirement)
                .map_err(|source| {
                    ZcirCapabilitiesError::InvalidRequirement {
                        index,
                        source,
                    }
                })?;
        }

        Ok(())
    }

    /// Iterate over requirements in insertion order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &CapabilityRequirement> + DoubleEndedIterator
    {
        self.entries.iter()
    }
}

impl IntoIterator for ZcirCapabilityRequirements {
    type Item = CapabilityRequirement;
    type IntoIter = std::vec::IntoIter<CapabilityRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a ZcirCapabilityRequirements {
    type Item = &'a CapabilityRequirement;
    type IntoIter = std::slice::Iter<'a, CapabilityRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

/// Errors produced by the ZCIR capability integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZcirCapabilitiesError {
    /// The collection exceeded its caller-specified entry budget.
    RequirementBudgetExceeded {
        /// Configured maximum number of entries.
        limit: usize,

        /// Actual number of entries.
        actual: usize,
    },

    /// One requirement failed canonical local validation.
    InvalidRequirement {
        /// Zero-based index in insertion order.
        index: usize,

        /// Canonical ZUIR validation error.
        source: CapabilityValidationError,
    },
}

impl fmt::Display for ZcirCapabilitiesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequirementBudgetExceeded { limit, actual } => {
                write!(
                    f,
                    "ZCIR capability requirement budget exceeded: \
                     limit {limit}, actual {actual}"
                )
            }
            Self::InvalidRequirement { index, source } => {
                write!(
                    f,
                    "invalid ZCIR capability requirement at index {index}: {source}"
                )
            }
        }
    }
}

impl Error for ZcirCapabilitiesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RequirementBudgetExceeded { .. } => None,
            Self::InvalidRequirement { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn requirement(name: &str) -> CapabilityRequirement {
        CapabilityRequirement::new(
            CapabilityRef::parse(name).expect("valid capability name"),
        )
    }

    #[test]
    fn accepts_open_world_capability_names() {
        let item = requirement("future::classical::vector_execution");

        assert_eq!(
            item.capability.canonical_name(),
            "future::classical::vector_execution"
        );
    }

    #[test]
    fn preserves_order_and_duplicates() {
        let first = requirement("zamani::classical::integer");
        let second = requirement("future::classical::vector_execution");
        let third = requirement("zamani::classical::integer");

        let requirements =
            ZcirCapabilityRequirements::from_entries(vec![
                first.clone(),
                second.clone(),
                third.clone(),
            ]);

        assert_eq!(requirements.len(), 3);
        assert_eq!(requirements.as_slice()[0], first);
        assert_eq!(requirements.as_slice()[1], second);
        assert_eq!(requirements.as_slice()[2], third);
    }

    #[test]
    fn accepts_an_empty_collection() {
        let requirements = ZcirCapabilityRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.validate().is_ok());
    }

    #[test]
    fn enforces_only_an_explicit_collection_budget() {
        let requirements =
            ZcirCapabilityRequirements::from_entries(vec![
                requirement("zamani::classical::integer"),
                requirement("zamani::classical::control_flow"),
            ]);

        let budget = ZcirCapabilitiesValidationBudget::bounded(
            1,
            CapabilityValidationBudget::unlimited(),
        );

        assert!(matches!(
            requirements.validate_with(&budget),
            Err(ZcirCapabilitiesError::RequirementBudgetExceeded {
                limit: 1,
                actual: 2
            })
        ));

        assert!(requirements.validate().is_ok());
    }

    #[test]
    fn append_preserves_both_input_orders() {
        let mut left = ZcirCapabilityRequirements::from_entries(vec![
            requirement("zamani::classical::integer"),
        ]);
        let mut right = ZcirCapabilityRequirements::from_entries(vec![
            requirement("zamani::classical::control_flow"),
            requirement("future::classical::vector_execution"),
        ]);

        left.append(&mut right);

        assert_eq!(left.len(), 3);
        assert!(right.is_empty());
        assert_eq!(
            left.as_slice()[0].capability.canonical_name(),
            "zamani::classical::integer"
        );
        assert_eq!(
            left.as_slice()[1].capability.canonical_name(),
            "zamani::classical::control_flow"
        );
        assert_eq!(
            left.as_slice()[2].capability.canonical_name(),
            "future::classical::vector_execution"
        );
    }

    #[test]
    fn rejects_invalid_requirement_property_values() {
        let mut item = requirement("zamani::classical::integer");
        item.properties.insert(
            String::new(),
            CapabilityValue::Boolean(true),
        );

        let requirements =
            ZcirCapabilityRequirements::from_entries(vec![item]);

        assert!(matches!(
            requirements.validate(),
            Err(ZcirCapabilitiesError::InvalidRequirement {
                index: 0,
                ..
            })
        ));
    }

    #[test]
    fn empty_explicit_budget_accepts_only_empty_collection() {
        let budget = ZcirCapabilitiesValidationBudget::bounded(
            0,
            CapabilityValidationBudget::unlimited(),
        );

        assert!(
            ZcirCapabilityRequirements::new()
                .validate_with(&budget)
                .is_ok()
        );

        let nonempty = ZcirCapabilityRequirements::from_entries(vec![
            requirement("zamani::classical::integer"),
        ]);

        assert!(nonempty.validate_with(&budget).is_err());
    }
}
