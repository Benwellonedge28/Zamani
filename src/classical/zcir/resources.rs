
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZCIR target-independent resource requirements and feasibility contracts.
//!
//! Path: `src/classical/zcir/resources.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only; `unsafe` is forbidden.
//!
//! # Purpose
//!
//! Represent the resource intent associated with classical computation
//! without tying its meaning to a particular machine, runtime, operating
//! system, accelerator, deployment, or resource-allocation strategy.
//!
//! This module models:
//! - named resource dimensions;
//! - exact, non-negative decimal quantities of arbitrary representable length;
//! - minimum, maximum, and exact resource bounds;
//! - requirements, preferences, and hints;
//! - optional resource units;
//! - available-resource inventories;
//! - deterministic feasibility assessments;
//! - explicit unknown and unsupported feasibility outcomes.
//!
//! # Ownership
//!
//! This file owns the ZCIR-local resource contract and its pure validation
//! and feasibility algorithms.
//!
//! It does not own:
//! - source-language semantic analysis;
//! - the canonical ZUIR resource representation;
//! - physical resource discovery or allocation;
//! - scheduling, placement, routing, or execution;
//! - hardware capabilities or backend-specific resource models;
//! - global compiler budgets or runtime policy configuration.
//!
//! # Integration
//!
//! The frontend semantic model establishes source-level resource meaning.
//! ZCIR consumes the established meaning through an adapter.
//!
//! The ZUIR integration layer must map canonical resource facts into this
//! contract, or replace this contract with the canonical ZUIR resource type
//! when the actual public API is verified. Do not maintain two competing
//! resource authorities.
//!
//! `crate::classical::execution` owns execution planning.
//! `crate::classical::config` owns operational configuration and budgets.
//! `crate::classical::diagnostics` and `crate::classical::error` own their
//! respective diagnostic and error taxonomies.
//! These modules can adapt this file's structured errors and assessments;
//! this module does not assume their APIs.
//!
//! The quantum IR retains its canonical classical and quantum resource
//! semantics. Physical qubits, device allocation, routing, and scheduling
//! must not be implemented here.
//!
//! # Scalability
//!
//! There is no universal maximum resource quantity or number of resource
//! dimensions. Quantities use canonical decimal strings rather than machine
//! integers, so a quantity need not fit in u64 or u128.
//!
//! The implementation is still subject to available memory and processing
//! time. Callers must apply their own explicit operational budgets when
//! processing untrusted or exceptionally large input.
//!
//! # Determinism
//!
//! Resource dimensions and inventories use ordered maps. Canonical decimal
//! quantities have no leading zeroes except for zero itself.
//!
//! # Feasibility
//!
//! Feasibility is a statement about the supplied inventory, not a promise
//! that execution will succeed. Missing information remains unknown.
//! A preference or hint is never promoted into a hard requirement.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// A validated, portable resource-dimension identifier.
///
/// Examples include `memory.bytes`, `compute.cpu.logical_cores`,
/// `compute.gpu.devices`, `classical.integer.bits`, and
/// `storage.bytes`.
///
/// The registry of recognized dimensions belongs to the integrating
/// specification or dialect. This type deliberately does not contain a
/// closed enumeration of hardware or resource categories.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResourceDimension(String);

impl ResourceDimension {
    /// Construct a resource dimension.
    ///
    /// Identifiers use lowercase ASCII letters, decimal digits, `_`, `-`,
    /// and `.`. They must be non-empty and must not contain empty dotted
    /// segments. No arbitrary identifier-length ceiling is imposed here.
    pub fn new(value: impl Into<String>) -> Result<Self, ResourceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResourceError::EmptyDimension);
        }

        if value
            .bytes()
            .any(|b| !(b.is_ascii_lowercase()
                || b.is_ascii_digit()
                || b == b'_'
                || b == b'-'
                || b == b'.'))
        {
            return Err(ResourceError::InvalidDimension(value));
        }

        if value
            .split('.')
            .any(|segment| segment.is_empty())
        {
            return Err(ResourceError::InvalidDimension(value));
        }

        Ok(Self(value))
    }

    /// Return the canonical dimension name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An exact non-negative integer quantity with arbitrary decimal length.
///
/// The canonical representation is a decimal string:
///
/// - `0` represents zero;
/// - nonzero values have no leading zeroes;
/// - signs, whitespace, separators, exponents, and decimal points are
///   rejected.
///
/// Using a decimal representation avoids silently imposing the capacity of
/// a native integer on resource quantities. It does not imply that the
/// host can physically provide the represented amount.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ResourceAmount(String);

impl ResourceAmount {
    /// Parse and canonicalize a non-negative integer.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ResourceError> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(ResourceError::InvalidAmount(
                value.to_owned(),
            ));
        }

        if !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(ResourceError::InvalidAmount(
                value.to_owned(),
            ));
        }

        let canonical = value.trim_start_matches('0');

        Ok(Self(if canonical.is_empty() {
            "0".to_owned()
        } else {
            canonical.to_owned()
        }))
    }

    /// Construct zero.
    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    /// Construct one.
    pub fn one() -> Self {
        Self("1".to_owned())
    }

    /// Return the canonical decimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compare exact quantities without converting to a bounded integer.
    pub fn cmp_exact(&self, other: &Self) -> Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            ordering => ordering,
        }
    }

    /// Return whether this quantity is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The relationship between a declared quantity and a resource quantity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceBound {
    /// The available quantity must be at least the declared quantity.
    AtLeast,

    /// The available quantity must not exceed the declared quantity.
    AtMost,

    /// The available quantity must equal the declared quantity.
    Exactly,
}

/// The semantic force of a resource declaration.
///
/// Only `Required` can make a target infeasible. Preferences and hints
/// inform planning but must not change program meaning when unmet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceIntent {
    /// A hard requirement whose violation prevents a valid realization.
    Required,

    /// A desired property that may be unmet without invalidating semantics.
    Preferred,

    /// Advisory information for a planner or downstream consumer.
    Hint,
}

/// A target-independent resource declaration.
///
/// `unit` is optional because some dimensions have a canonical unit
/// established by their registry. Where a dimension supports multiple
/// units, integrations must normalize compatible units before comparison.
/// This module intentionally performs no implicit unit conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceRequirement {
    dimension: ResourceDimension,
    bound: ResourceBound,
    amount: ResourceAmount,
    intent: ResourceIntent,
    unit: Option<String>,
}

impl ResourceRequirement {
    /// Create a resource declaration.
    pub fn new(
        dimension: ResourceDimension,
        bound: ResourceBound,
        amount: ResourceAmount,
        intent: ResourceIntent,
        unit: Option<String>,
    ) -> Result<Self, ResourceError> {
        if let Some(ref unit) = unit {
            if unit.is_empty()
                || unit.bytes().any(|b| b.is_ascii_control())
            {
                return Err(ResourceError::InvalidUnit(unit.clone()));
            }
        }

        Ok(Self {
            dimension,
            bound,
            amount,
            intent,
            unit,
        })
    }

    /// The resource dimension.
    pub fn dimension(&self) -> &ResourceDimension {
        &self.dimension
    }

    /// The bound relationship.
    pub fn bound(&self) -> ResourceBound {
        self.bound
    }

    /// The declared quantity.
    pub fn amount(&self) -> &ResourceAmount {
        &self.amount
    }

    /// The semantic force of this declaration.
    pub fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// The optional unit.
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    /// Evaluate this declaration against one known available quantity.
    ///
    /// This method does not allocate resources and does not make any
    /// assumptions about physical topology or target capabilities.
    pub fn evaluate(
        &self,
        available: &ResourceAmount,
    ) -> bool {
        let ordering = available.cmp_exact(&self.amount);

        match self.bound {
            ResourceBound::AtLeast => ordering != Ordering::Less,
            ResourceBound::AtMost => ordering != Ordering::Greater,
            ResourceBound::Exactly => ordering == Ordering::Equal,
        }
    }
}

/// An inventory of known resource quantities.
///
/// Absence of a dimension means "unknown or not reported", not zero.
/// An inventory is a snapshot supplied by a caller; it does not discover
/// devices, reserve resources, or guarantee future availability.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceInventory {
    quantities: BTreeMap<ResourceDimension, InventoryEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryEntry {
    amount: ResourceAmount,
    unit: Option<String>,
}

impl ResourceInventory {
    /// Create an empty inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a known quantity for a dimension.
    ///
    /// Repeating a dimension replaces its previous value explicitly.
    /// Callers that need conflict detection should check `contains`
    /// before inserting.
    pub fn insert(
        &mut self,
        dimension: ResourceDimension,
        amount: ResourceAmount,
        unit: Option<String>,
    ) -> Result<Option<ResourceAmount>, ResourceError> {
        if let Some(ref unit) = unit {
            if unit.is_empty()
                || unit.bytes().any(|b| b.is_ascii_control())
            {
                return Err(ResourceError::InvalidUnit(unit.clone()));
            }
        }

        let previous = self.quantities.insert(
            dimension,
            InventoryEntry { amount, unit },
        );

        Ok(previous.map(|entry| entry.amount))
    }

    /// Check whether a dimension has been reported.
    pub fn contains(&self, dimension: &ResourceDimension) -> bool {
        self.quantities.contains_key(dimension)
    }

    /// Get a known quantity.
    pub fn get(
        &self,
        dimension: &ResourceDimension,
    ) -> Option<&ResourceAmount> {
        self.quantities.get(dimension).map(|entry| &entry.amount)
    }

    /// Get the optional unit reported for a dimension.
    pub fn unit(
        &self,
        dimension: &ResourceDimension,
    ) -> Option<Option<&str>> {
        self.quantities
            .get(dimension)
            .map(|entry| entry.unit.as_deref())
    }

    /// Iterate over known inventory entries in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&ResourceDimension, &ResourceAmount, Option<&str>),
    > {
        self.quantities.iter().map(|(dimension, entry)| {
            (dimension, &entry.amount, entry.unit.as_deref())
        })
    }
}

/// Feasibility of one declaration against an inventory snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RequirementStatus {
    /// The inventory proves the declaration is satisfied.
    Satisfied,

    /// The inventory proves the declaration is not satisfied.
    Unsatisfied,

    /// The inventory does not report this dimension.
    Unknown,

    /// The inventory and requirement use incompatible or unverified units.
    Incomparable,
}

/// A single requirement assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceAssessment {
    pub dimension: ResourceDimension,
    pub intent: ResourceIntent,
    pub status: RequirementStatus,
    pub required: ResourceAmount,
    pub available: Option<ResourceAmount>,
    pub bound: ResourceBound,
}

/// Overall feasibility of the supplied declarations.
///
/// This is a conservative assessment of known inventory facts, not an
/// execution guarantee. `Infeasible` means at least one hard requirement
/// is demonstrably unmet or incomparable. `Indeterminate` means no hard
/// requirement is known to fail, but at least one cannot be established.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Feasibility {
    Feasible,
    Infeasible,
    Indeterminate,
}

/// Evaluate declarations without allocating resources.
///
/// All declarations are assessed, including preferences and hints.
/// A missing inventory entry produces `Unknown`. Unit mismatches produce
/// `Incomparable`; no implicit conversion is attempted.
pub fn assess_resources(
    requirements: &[ResourceRequirement],
    inventory: &ResourceInventory,
) -> Vec<ResourceAssessment> {
    requirements
        .iter()
        .map(|requirement| {
            let entry = inventory.quantities.get(&requirement.dimension);

            let (status, available) = match entry {
                None => (RequirementStatus::Unknown, None),
                Some(entry) => {
                    let compatible_units = requirement.unit.as_deref()
                        == entry.unit.as_deref();

                    if !compatible_units {
                        (
                            RequirementStatus::Incomparable,
                            Some(entry.amount.clone()),
                        )
                    } else if requirement.evaluate(&entry.amount) {
                        (
                            RequirementStatus::Satisfied,
                            Some(entry.amount.clone()),
                        )
                    } else {
                        (
                            RequirementStatus::Unsatisfied,
                            Some(entry.amount.clone()),
                        )
                    }
                }
            };

            ResourceAssessment {
                dimension: requirement.dimension.clone(),
                intent: requirement.intent,
                status,
                required: requirement.amount.clone(),
                available,
                bound: requirement.bound,
            }
        })
        .collect()
}

/// Summarize assessments using hard requirements only.
///
/// An unmet preference or hint does not make the computation infeasible.
/// Unknown hard requirements remain indeterminate unless another hard
/// requirement already proves infeasibility.
pub fn summarize_feasibility(
    assessments: &[ResourceAssessment],
) -> Feasibility {
    let mut indeterminate = false;

    for assessment in assessments {
        if assessment.intent != ResourceIntent::Required {
            continue;
        }

        match assessment.status {
            RequirementStatus::Satisfied => {}
            RequirementStatus::Unsatisfied
            | RequirementStatus::Incomparable => {
                return Feasibility::Infeasible;
            }
            RequirementStatus::Unknown => indeterminate = true,
        }
    }

    if indeterminate {
        Feasibility::Indeterminate
    } else {
        Feasibility::Feasible
    }
}

/// Resource contract errors.
///
/// These errors describe malformed resource declarations, not target
/// infeasibility. Integrations may map them into the repository's canonical
/// diagnostic and error types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResourceError {
    EmptyDimension,
    InvalidDimension(String),
    InvalidAmount(String),
    InvalidUnit(String),
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDimension => {
                f.write_str("resource dimension must not be empty")
            }
            Self::InvalidDimension(value) => {
                write!(f, "invalid resource dimension: {value:?}")
            }
            Self::InvalidAmount(value) => {
                write!(f, "invalid non-negative integer quantity: {value:?}")
            }
            Self::InvalidUnit(value) => {
                write!(f, "invalid resource unit: {value:?}")
            }
        }
    }
}

impl Error for ResourceError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimension(name: &str) -> ResourceDimension {
        ResourceDimension::new(name).expect("valid test dimension")
    }

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::parse(value).expect("valid test quantity")
    }

    fn requirement(
        name: &str,
        bound: ResourceBound,
        quantity: &str,
        intent: ResourceIntent,
        unit: Option<&str>,
    ) -> ResourceRequirement {
        ResourceRequirement::new(
            dimension(name),
            bound,
            amount(quantity),
            intent,
            unit.map(str::to_owned),
        )
        .expect("valid test requirement")
    }

    #[test]
    fn decimal_quantities_are_canonical() {
        assert_eq!(amount("000000").as_str(), "0");
        assert_eq!(amount("00042").as_str(), "42");
        assert_eq!(amount("999").cmp_exact(&amount("1000")), Ordering::Less);
        assert_eq!(
            amount("100000000000000000000000000000000000000000")
                .cmp_exact(&amount("99999999999999999999999999999999999999999")),
            Ordering::Greater
        );
    }

    #[test]
    fn rejects_invalid_quantities() {
        for value in ["", "-1", "+1", "1.0", "1e3", " 1", "1 "] {
            assert!(ResourceAmount::parse(value).is_err(), "{value:?}");
        }
    }

    #[test]
    fn rejects_invalid_dimensions() {
        for value in ["", "Memory.Bytes", ".memory", "memory.", "a..b", "a b"] {
            assert!(ResourceDimension::new(value).is_err(), "{value:?}");
        }
    }

    #[test]
    fn evaluates_all_bound_relationships() {
        let available = amount("10");

        assert!(requirement(
            "memory.bytes",
            ResourceBound::AtLeast,
            "9",
            ResourceIntent::Required,
            Some("bytes"),
        ).evaluate(&available));

        assert!(requirement(
            "memory.bytes",
            ResourceBound::AtMost,
            "10",
            ResourceIntent::Required,
            Some("bytes"),
        ).evaluate(&available));

        assert!(!requirement(
            "memory.bytes",
            ResourceBound::Exactly,
            "11",
            ResourceIntent::Required,
            Some("bytes"),
        ).evaluate(&available));
    }

    #[test]
    fn missing_inventory_is_unknown_not_zero() {
        let requirements = vec![requirement(
            "memory.bytes",
            ResourceBound::AtLeast,
            "100",
            ResourceIntent::Required,
            Some("bytes"),
        )];

        let assessments = assess_resources(
            &requirements,
            &ResourceInventory::new(),
        );

        assert_eq!(assessments[0].status, RequirementStatus::Unknown);
        assert_eq!(
            summarize_feasibility(&assessments),
            Feasibility::Indeterminate
        );
    }

    #[test]
    fn unmet_preference_does_not_invalidate_execution() {
        let requirements = vec![requirement(
            "compute.cpu.logical_cores",
            ResourceBound::AtLeast,
            "8",
            ResourceIntent::Preferred,
            None,
        )];

        let mut inventory = ResourceInventory::new();
        inventory.insert(
            dimension("compute.cpu.logical_cores"),
            amount("2"),
            None,
        ).expect("valid inventory entry");

        let assessments = assess_resources(&requirements, &inventory);

        assert_eq!(assessments[0].status, RequirementStatus::Unsatisfied);
        assert_eq!(
            summarize_feasibility(&assessments),
            Feasibility::Feasible
        );
    }

    #[test]
    fn unmet_hard_requirement_is_infeasible() {
        let requirements = vec![requirement(
            "memory.bytes",
            ResourceBound::AtLeast,
            "1000",
            ResourceIntent::Required,
            Some("bytes"),
        )];

        let mut inventory = ResourceInventory::new();
        inventory.insert(
            dimension("memory.bytes"),
            amount("999"),
            Some("bytes".to_owned()),
        ).expect("valid inventory entry");

        let assessments = assess_resources(&requirements, &inventory);

        assert_eq!(
            summarize_feasibility(&assessments),
            Feasibility::Infeasible
        );
    }

    #[test]
    fn incompatible_units_are_not_silently_converted() {
        let requirements = vec![requirement(
            "memory.bytes",
            ResourceBound::AtLeast,
            "1",
            ResourceIntent::Required,
            Some("bytes"),
        )];

        let mut inventory = ResourceInventory::new();
        inventory.insert(
            dimension("memory.bytes"),
            amount("1000000"),
            Some("kilobytes".to_owned()),
        ).expect("valid inventory entry");

        let assessments = assess_resources(&requirements, &inventory);

        assert_eq!(
            assessments[0].status,
            RequirementStatus::Incomparable
        );
        assert_eq!(
            summarize_feasibility(&assessments),
            Feasibility::Infeasible
        );
    }

    #[test]
    fn zero_is_a_valid_quantity() {
        assert!(amount("0").is_zero());
        assert!(!amount("1").is_zero());
    }

    #[test]
    fn inventory_iteration_is_deterministic() {
        let mut inventory = ResourceInventory::new();

        inventory.insert(
            dimension("memory.bytes"),
            amount("10"),
            Some("bytes".to_owned()),
        ).expect("valid entry");

        inventory.insert(
            dimension("compute.cpu.logical_cores"),
            amount("4"),
            None,
        ).expect("valid entry");

        let names: Vec<_> = inventory
            .iter()
            .map(|(dimension, _, _)| dimension.as_str())
            .collect();

        assert_eq!(
            names,
            vec!["compute.cpu.logical_cores", "memory.bytes"]
        );
    }
}
