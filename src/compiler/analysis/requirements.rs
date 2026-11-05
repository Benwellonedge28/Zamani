#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Requirement Analysis
//!
//! Path: `src/compiler/analysis/requirements.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the compiler-side representation of requirements that must be
//! considered when determining possible realizations of a Zamani program.
//!
//! Requirements describe what a computation needs. They do not describe
//! what a target provides, what a target is preferred, or whether a
//! particular realization is feasible.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-side requirement identifiers.
//! - Requirement strength.
//! - Requirement values.
//! - Requirement collections.
//! - Requirement provenance references.
//! - Requirement validation.
//! - Deterministic requirement ordering.
//!
//! This module does not own:
//!
//! - Source-language semantic meaning.
//! - Canonical AST declarations.
//! - Semantic type inference.
//! - Semantic resource quantities.
//! - Hardware capabilities.
//! - Target discovery.
//! - Backend compatibility decisions.
//! - Physical resource allocation.
//! - Scheduling, routing, or execution.
//!
//! ## Integration
//!
//! The canonical semantic layer remains authoritative.
//!
//! `src/frontend/semantic/` produces semantic facts. This module may consume
//! those facts through adapters, but must not redefine their meaning.
//!
//! `src/backend/compatibility.rs` owns existing compatibility primitives.
//! This module must not introduce a second compatibility engine.
//!
//! Compiler planning consumes validated requirement sets and compares them
//! against independently supplied capability evidence.
//!
//! ## Scalability
//!
//! This module imposes no language-level ceiling on the number of
//! requirements or hardware resources.
//!
//! Its collections grow according to available memory. Operational limits
//! belong to explicit compilation budgets, not semantic requirement types.
//!
//! A finite implementation cannot process literally infinite input.
//! The intended guarantee is that the language does not impose arbitrary
//! hardware-count ceilings.
//!
//! ## Safety
//!
//! No unsafe code, global mutable state, hardware inspection, or I/O is used.
//!
//! ## Determinism
//!
//! Requirement identifiers and values have canonical representations.
//! Ordered collections are used where stable ordering is required.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

/// Schema version of this in-memory requirement model.
///
/// This is not the Zamani language version, AST version, or artifact version.
pub const REQUIREMENT_MODEL_SCHEMA_VERSION: u16 = 1;

/// A stable, open, namespaced requirement identifier.
///
/// Identifiers are case-sensitive.
///
/// Examples:
///
/// - `classical.integer_arithmetic`
/// - `quantum.measurement`
/// - `quantum.mid_circuit_measurement`
/// - `ai.tensor_operation`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `networking.communication`
///
/// These examples are not a closed registry.
///
/// New computing domains may introduce identifiers without changing this
/// type or adding variants to a central enum.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementId(String);

impl RequirementId {
    /// Creates a validated requirement identifier.
    ///
    /// The identifier must contain at least one non-whitespace character.
    ///
    /// The identifier is not restricted to a predefined set of computing
    /// domains.
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RequirementError::EmptyIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(RequirementError::ControlCharacter {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the canonical identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RequirementId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Requirement strength.
///
/// Strength describes the semantic importance of a requirement.
///
/// It does not describe how difficult a requirement is to implement.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementStrength {
    /// The requirement must be satisfied.
    ///
    /// An implementation must not silently weaken or discard it.
    Hard,

    /// The property is desirable but may be traded off.
    ///
    /// A preference is not a correctness requirement.
    Preferred,

    /// The property may guide implementation decisions.
    ///
    /// A hint does not establish correctness or feasibility.
    Advisory,
}

/// A canonical exact, non-negative decimal number.
///
/// The representation is a string to avoid introducing a fixed integer-width
/// ceiling into the requirement model.
///
/// Accepted syntax:
///
/// `DIGITS`
///
/// or
///
/// `DIGITS "." DIGITS`
///
/// Scientific notation and signs are intentionally excluded.
///
/// Values are normalized:
///
/// - Leading integer zeroes are removed.
/// - Trailing fractional zeroes are removed.
/// - Zero is represented as `"0"`.
///
/// This type does not perform floating-point arithmetic.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DecimalQuantity(String);

impl DecimalQuantity {
    /// Creates a validated decimal quantity.
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RequirementError::InvalidDecimal {
                value,
            });
        }

        let mut parts = value.split('.');

        let integer = parts.next().unwrap_or_default();
        let fraction = parts.next();

        if parts.next().is_some() {
            return Err(RequirementError::InvalidDecimal {
                value,
            });
        }

        if integer.is_empty()
            || !integer.chars().all(|c| c.is_ascii_digit())
        {
            return Err(RequirementError::InvalidDecimal {
                value,
            });
        }

        if let Some(fraction) = fraction {
            if fraction.is_empty()
                || !fraction.chars().all(|c| c.is_ascii_digit())
            {
                return Err(RequirementError::InvalidDecimal {
                    value,
                });
            }
        }

        let normalized_integer = integer.trim_start_matches('0');

        let normalized_integer = if normalized_integer.is_empty() {
            "0"
        } else {
            normalized_integer
        };

        let normalized_fraction = fraction
            .map(|fraction| fraction.trim_end_matches('0'))
            .unwrap_or("");

        let normalized = if normalized_fraction.is_empty() {
            normalized_integer.to_owned()
        } else {
            format!(
                "{}.{}",
                normalized_integer,
                normalized_fraction
            )
        };

        Ok(Self(normalized))
    }

    /// Returns the canonical decimal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this quantity is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }
}

impl fmt::Display for DecimalQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A requirement value.
///
/// Values are intentionally open to future extensions.
///
/// A requirement may represent a boolean property, exact quantity, textual
/// identity, or symbolic expression.
///
/// Symbolic expressions must be supplied through a canonical semantic
/// expression adapter rather than reconstructed from source text.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RequirementValue {
    /// A boolean property.
    Boolean(bool),

    /// An exact decimal quantity.
    Quantity(DecimalQuantity),

    /// A stable textual identity.
    ///
    /// This is not a source-language expression.
    Text(String),

    /// An opaque symbolic expression reference.
    ///
    /// The identifier must be supplied by the semantic authority.
    Symbolic {
        /// Semantic expression identifier.
        expression_id: String,
    },
}

impl RequirementValue {
    /// Creates a textual requirement value.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a symbolic requirement value.
    pub fn symbolic(
        expression_id: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let expression_id = expression_id.into();

        if expression_id.trim().is_empty() {
            return Err(RequirementError::EmptyExpressionId);
        }

        Ok(Self::Symbolic {
            expression_id,
        })
    }
}

/// Provenance of a requirement.
///
/// Provenance is descriptive metadata. It does not establish that the
/// requirement is valid or feasible.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RequirementProvenance {
    /// Optional originating semantic node identifier.
    pub semantic_node_id: Option<String>,

    /// Optional originating module identity.
    pub module_id: Option<String>,

    /// Optional explanation for the requirement.
    pub explanation: Option<String>,
}

impl RequirementProvenance {
    /// Creates empty provenance.
    pub fn empty() -> Self {
        Self {
            semantic_node_id: None,
            module_id: None,
            explanation: None,
        }
    }
}

impl Default for RequirementProvenance {
    fn default() -> Self {
        Self::empty()
    }
}

/// One compiler-side requirement.
///
/// A requirement identifies a property that a realization must establish.
///
/// This structure does not claim that any target satisfies the requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requirement {
    /// Stable requirement identifier.
    pub id: RequirementId,

    /// Requirement strength.
    pub strength: RequirementStrength,

    /// Required property value.
    pub value: RequirementValue,

    /// Requirement provenance.
    pub provenance: RequirementProvenance,
}

impl Requirement {
    /// Creates a requirement.
    pub fn new(
        id: RequirementId,
        strength: RequirementStrength,
        value: RequirementValue,
    ) -> Self {
        Self {
            id,
            strength,
            value,
            provenance: RequirementProvenance::empty(),
        }
    }

    /// Attaches provenance to the requirement.
    pub fn with_provenance(
        mut self,
        provenance: RequirementProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Returns whether the requirement is mandatory.
    pub fn is_hard(&self) -> bool {
        self.strength == RequirementStrength::Hard
    }
}

/// A validated collection of compiler requirements.
///
/// Requirements are stored in deterministic identifier order.
///
/// Duplicate identifiers are rejected. A caller must resolve duplicate
/// declarations explicitly rather than silently choosing one.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequirementSet {
    requirements: BTreeMap<RequirementId, Requirement>,
}

impl RequirementSet {
    /// Creates an empty requirement set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a requirement.
    ///
    /// Returns an error if the identifier is already present.
    pub fn insert(
        &mut self,
        requirement: Requirement,
    ) -> Result<(), RequirementError> {
        let id = requirement.id.clone();

        if self.requirements.contains_key(&id) {
            return Err(RequirementError::DuplicateRequirement {
                id,
            });
        }

        self.requirements.insert(id, requirement);

        Ok(())
    }

    /// Returns a requirement by identifier.
    pub fn get(
        &self,
        id: &RequirementId,
    ) -> Option<&Requirement> {
        self.requirements.get(id)
    }

    /// Returns the number of requirements.
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns requirements in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements.values()
    }

    /// Returns only hard requirements.
    pub fn hard_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements
            .values()
            .filter(|requirement| requirement.is_hard())
    }

    /// Returns only preferred requirements.
    pub fn preferred_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements.values().filter(|requirement| {
            requirement.strength == RequirementStrength::Preferred
        })
    }

    /// Returns only advisory requirements.
    pub fn advisory_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements.values().filter(|requirement| {
            requirement.strength == RequirementStrength::Advisory
        })
    }
}

/// Errors produced by requirement construction and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequirementError {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains a control character.
    ControlCharacter {
        /// Invalid identifier.
        value: String,
    },

    /// A decimal quantity is malformed.
    InvalidDecimal {
        /// Invalid decimal value.
        value: String,
    },

    /// A symbolic expression identifier is empty.
    EmptyExpressionId,

    /// A requirement identifier is duplicated.
    DuplicateRequirement {
        /// Duplicate requirement identifier.
        id: RequirementId,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                write!(formatter, "requirement identifier cannot be empty")
            }

            Self::ControlCharacter { value } => {
                write!(
                    formatter,
                    "requirement identifier contains a control character: {value:?}"
                )
            }

            Self::InvalidDecimal { value } => {
                write!(
                    formatter,
                    "invalid non-negative decimal quantity: {value:?}"
                )
            }

            Self::EmptyExpressionId => {
                write!(
                    formatter,
                    "symbolic expression identifier cannot be empty"
                )
            }

            Self::DuplicateRequirement { id } => {
                write!(
                    formatter,
                    "requirement `{id}` is already present"
                )
            }
        }
    }
}

impl Error for RequirementError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_identifiers_are_open_and_namespaced() {
        let quantum = RequirementId::new(
            "quantum.mid_circuit_measurement",
        )
        .unwrap();

        let future_domain = RequirementId::new(
            "future.photonic.coherent_operation",
        )
        .unwrap();

        assert_ne!(quantum, future_domain);
    }

    #[test]
    fn empty_identifier_is_rejected() {
        assert_eq!(
            RequirementId::new(" "),
            Err(RequirementError::EmptyIdentifier)
        );
    }

    #[test]
    fn decimal_quantity_is_canonicalized() {
        assert_eq!(
            DecimalQuantity::new("00012.3400")
                .unwrap()
                .as_str(),
            "12.34"
        );

        assert_eq!(
            DecimalQuantity::new("0000.000")
                .unwrap()
                .as_str(),
            "0"
        );
    }

    #[test]
    fn decimal_quantity_does_not_use_machine_integer_width() {
        let value = "999999999999999999999999999999999999999999999999999999";

        assert_eq!(
            DecimalQuantity::new(value)
                .unwrap()
                .as_str(),
            value
        );
    }

    #[test]
    fn malformed_decimal_is_rejected() {
        assert!(
            DecimalQuantity::new("1.2.3").is_err()
        );

        assert!(
            DecimalQuantity::new("-1").is_err()
        );

        assert!(
            DecimalQuantity::new("1e10").is_err()
        );
    }

    #[test]
    fn requirement_set_rejects_duplicate_ids() {
        let id = RequirementId::new(
            "classical.integer_arithmetic",
        )
        .unwrap();

        let mut set = RequirementSet::new();

        set.insert(Requirement::new(
            id.clone(),
            RequirementStrength::Hard,
            RequirementValue::Boolean(true),
        ))
        .unwrap();

        assert!(
            set.insert(Requirement::new(
                id,
                RequirementStrength::Preferred,
                RequirementValue::Boolean(true),
            ))
            .is_err()
        );
    }

    #[test]
    fn requirements_are_returned_in_deterministic_order() {
        let mut set = RequirementSet::new();

        for name in [
            "quantum.measurement",
            "classical.integer_arithmetic",
            "ai.tensor_operation",
        ] {
            set.insert(Requirement::new(
                RequirementId::new(name).unwrap(),
                RequirementStrength::Hard,
                RequirementValue::Boolean(true),
            ))
            .unwrap();
        }

        let ids: Vec<_> = set
            .iter()
            .map(|requirement| requirement.id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "ai.tensor_operation",
                "classical.integer_arithmetic",
                "quantum.measurement",
            ]
        );
    }

    #[test]
    fn requirement_strengths_remain_distinct() {
        let mut set = RequirementSet::new();

        for (name, strength) in [
            ("required", RequirementStrength::Hard),
            ("preferred", RequirementStrength::Preferred),
            ("advisory", RequirementStrength::Advisory),
        ] {
            set.insert(Requirement::new(
                RequirementId::new(name).unwrap(),
                strength,
                RequirementValue::Boolean(true),
            ))
            .unwrap();
        }

        assert_eq!(set.hard_requirements().count(), 1);
        assert_eq!(set.preferred_requirements().count(), 1);
        assert_eq!(set.advisory_requirements().count(), 1);
    }

    #[test]
    fn symbolic_requirements_preserve_semantic_references() {
        let value = RequirementValue::symbolic(
            "semantic.expression.123",
        )
        .unwrap();

        assert_eq!(
            value,
            RequirementValue::Symbolic {
                expression_id: "semantic.expression.123".to_owned(),
            }
        );
    }

    #[test]
    fn empty_symbolic_reference_is_rejected() {
        assert_eq!(
            RequirementValue::symbolic(" "),
            Err(RequirementError::EmptyExpressionId)
        );
    }
}