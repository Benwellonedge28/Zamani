
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Target Constraints
//!
//! Path: `src/hdl/targets/constraints.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Initial implementation contract. Integration is complete only after this
//! module is registered and the repository's actual workspace checks pass.
//!
//! ## Purpose
//!
//! Store and validate constraints scoped to one explicit HDL target
//! description. Preserve target-specific conditions without changing
//! portable source semantics or duplicating the shared property model.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/hdl/requirements.rs`
//! - `src/hdl/targets/model.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar owns syntax. The canonical semantic model owns source meaning.
//! `requirements.rs` owns property relations, values, units, and intent.
//! `targets/model.rs` owns target identity and target-description validation.
//! This module composes those existing contracts.
//!
//! ## Owns
//!
//! - A collection of constraints associated with one target description.
//! - Optional textual provenance for individual constraint records.
//! - Validation that a constraint collection is being used with its target.
//! - Preservation of declaration order, duplicate entries, and potentially
//!   conflicting constraints for later analysis.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, name resolution, or type checking.
//! - A second property, value, relation, unit, or intent model.
//! - Evaluation of symbolic expressions or unit conversion.
//! - Conflict resolution, feasibility decisions, or capability evaluation.
//! - Target discovery, authentication, authorization, or device allocation.
//! - Physical pin assignment algorithms, placement, routing, or timing closure.
//! - Vendor APIs, synthesis, backend code generation, or deployment.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Semantics
//!
//! A target-scoped constraint describes a condition, preference, or hint
//! associated with a particular target description. Its property key and
//! value use the existing `HdlPropertyRequirement` contract.
//!
//! This module deliberately does not evaluate a property. A property's
//! meaning, dimensional compatibility, symbolic resolution, and comparison
//! rules must be established by the relevant semantic or target-analysis
//! service.
//!
//! Repeated entries are preserved. They may represent distinct declarations,
//! provenance, or conditions. This module does not silently deduplicate them
//! or decide that conflicting entries cancel one another out.
//!
//! Matching a constraint collection to a target establishes scope identity
//! only. It does not prove that the target satisfies any constraint.
//!
//! ## Scalability and POCO-REAF
//!
//! No fixed maximum number of constraints, properties, targets, or hardware
//! resources is defined here. Collections grow subject to available memory
//! and any explicit operational budget imposed by their caller.
//!
//! This does not promise literal infinity. Rust values, address spaces,
//! execution time, and available memory remain finite. Resource exhaustion
//! must be reported by the owning operation and must not be interpreted as
//! successful validation or feasibility.
//!
//! ## Integration
//!
//! 1. An upstream parser and semantic adapter construct a canonical
//!    `HdlPropertyRequirement`.
//! 2. Target-specific configuration adds it to an `HdlTargetConstraints`
//!    collection associated with a `TargetId`.
//! 3. The consumer validates the collection against the intended
//!    `HdlTargetDescription`.
//! 4. Later target analysis interprets property semantics and determines
//!    whether constraints are satisfied, contradicted, or unresolved.
//! 5. Realization consumes those results. This module does not implement
//!    realization or HDLIR.
//!
//! The target model currently has no constraints field. This module therefore
//! keeps the collection independent instead of requiring an uncoordinated
//! change to `HdlTargetDescription`. A later aggregate API may compose the
//! two contracts after target-analysis interfaces are frozen.
//!
//! ## Freeze criteria
//!
//! - Shared property and target identity contracts are reused.
//! - Constraint scope mismatches return structured errors.
//! - Duplicate and conflicting records are preserved.
//! - No constraint is declared satisfied merely because it was stored.
//! - The module is registered by `src/hdl/targets/mod.rs`.
//! - The actual repository formatting, compilation, test, and Clippy checks
//!   pass before production readiness is claimed.

use core::fmt;
use std::error::Error;

use crate::hdl::requirements::HdlPropertyRequirement;
use crate::hdl::targets::model::{
    HdlTargetDescription,
    HdlTargetModelError,
    TargetId,
};

/// Schema version of the in-memory target-constraints contract.
///
/// This is not the Zamani language version, target-model schema version,
/// provider version, or HDLIR schema version.
pub const HDL_TARGET_CONSTRAINTS_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of this module.
pub const HDL_TARGET_CONSTRAINTS_MODULE_NAME: &str =
    "zamani.hdl.targets.constraints";

/// One property condition associated with a target-constraint collection.
///
/// The property reuses the canonical `HdlPropertyRequirement` model. Its
/// `HdlPropertyIntent` determines whether it is mandatory, a constraint,
/// a preference, or a hint. This record does not evaluate that property.
///
/// `origin` is optional descriptive provenance, such as a configuration
/// source or provider declaration identifier. It is not authenticated and
/// does not replace canonical source spans or semantic provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTargetConstraint {
    property: HdlPropertyRequirement,
    origin: Option<String>,
}

impl HdlTargetConstraint {
    /// Construct a target-scoped constraint record.
    ///
    /// The property must already have passed construction through the
    /// canonical `HdlPropertyRequirement` API. The optional origin is checked
    /// for a non-empty value and control characters.
    ///
    /// This constructor does not validate property registration, resolve
    /// symbolic values, check units, or establish target feasibility.
    pub fn new(
        property: HdlPropertyRequirement,
        origin: Option<String>,
    ) -> Result<Self, HdlTargetConstraintsError> {
        if let Some(value) = origin.as_deref() {
            validate_text(value, "constraint origin")?;
        }

        Ok(Self { property, origin })
    }

    /// Return the canonical property requirement.
    #[must_use]
    pub const fn property(&self) -> &HdlPropertyRequirement {
        &self.property
    }

    /// Return the optional provenance label.
    #[must_use]
    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }
}

/// A collection of constraints associated with exactly one target identity.
///
/// Entries are retained in insertion order. Repeated properties are not
/// automatically duplicates: their provenance or intent may differ, and
/// even conflicting entries must remain visible to downstream analysis.
///
/// The collection is independent of `HdlTargetDescription` so that adding
/// target constraints does not require changing the existing target model's
/// representation or creating a second target-description type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTargetConstraints {
    target_id: TargetId,
    entries: Vec<HdlTargetConstraint>,
}

impl HdlTargetConstraints {
    /// Create an empty constraint collection for a validated target identity.
    ///
    /// `TargetId` can only be constructed through its validating constructor.
    /// An empty collection means that no target constraints were supplied;
    /// it does not prove that the target satisfies any requirements.
    #[must_use]
    pub const fn new(target_id: TargetId) -> Self {
        Self {
            target_id,
            entries: Vec::new(),
        }
    }

    /// Return the identity of the target to which these constraints apply.
    #[must_use]
    pub const fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether the collection contains no entries.
    ///
    /// An empty collection provides no evidence that a target is feasible.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over constraint records in insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &HdlTargetConstraint> {
        self.entries.iter()
    }

    /// Append a constraint without deduplicating or evaluating it.
    ///
    /// The collection owns the target scope; callers cannot accidentally
    /// attach an individual entry to a different target through this method.
    pub fn push(&mut self, constraint: HdlTargetConstraint) {
        self.entries.push(constraint);
    }

    /// Validate this collection against a target description.
    ///
    /// Validation checks the target description's local invariants and then
    /// verifies that its identity matches this collection's target identity.
    ///
    /// Success establishes only that the collection is structurally scoped
    /// to the supplied target. It does not establish that any property is
    /// satisfied, that the target exists physically, or that realization is
    /// possible.
    pub fn validate_for(
        &self,
        target: &HdlTargetDescription,
    ) -> Result<(), HdlTargetConstraintsError> {
        target
            .validate()
            .map_err(HdlTargetConstraintsError::InvalidTarget)?;

        if target.id() != &self.target_id {
            return Err(HdlTargetConstraintsError::TargetMismatch {
                expected: self.target_id.as_str().to_owned(),
                actual: target.id().as_str().to_owned(),
            });
        }

        for entry in &self.entries {
            if let Some(origin) = entry.origin() {
                validate_text(origin, "constraint origin")?;
            }
        }

        Ok(())
    }

    /// Return the stable architectural module name.
    #[must_use]
    pub const fn module_name() -> &'static str {
        HDL_TARGET_CONSTRAINTS_MODULE_NAME
    }

    /// Return the schema version of this in-memory contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        HDL_TARGET_CONSTRAINTS_SCHEMA_VERSION
    }
}

/// Errors produced by target-constraint construction or scope validation.
///
/// These errors describe malformed representation or incorrect target
/// association. They are not feasibility outcomes.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlTargetConstraintsError {
    /// A textual field is empty or consists only of whitespace.
    EmptyField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A textual field contains a Unicode control character.
    ControlCharacter {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// The supplied target description is locally invalid.
    InvalidTarget(HdlTargetModelError),

    /// The constraints were checked against a different target identity.
    TargetMismatch {
        /// Target identity the collection was created for.
        expected: String,
        /// Target identity supplied for validation.
        actual: String,
    },
}

impl fmt::Display for HdlTargetConstraintsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} must not contain control characters"
                )
            }
            Self::InvalidTarget(error) => {
                write!(formatter, "invalid target description: {error}")
            }
            Self::TargetMismatch { expected, actual } => {
                write!(
                    formatter,
                    "target constraints belong to `{expected}`, \
                     not `{actual}`"
                )
            }
        }
    }
}

impl Error for HdlTargetConstraintsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTarget(error) => Some(error),
            Self::EmptyField { .. }
            | Self::ControlCharacter { .. }
            | Self::TargetMismatch { .. } => None,
        }
    }
}

/// Validate a non-empty textual field without normalizing its value.
///
/// Exact strings are preserved to avoid silently changing provider-defined
/// identities or provenance. This is representation validation, not a
/// security or authentication check.
fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), HdlTargetConstraintsError> {
    if value.trim().is_empty() {
        return Err(HdlTargetConstraintsError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlTargetConstraintsError::ControlCharacter { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::requirements::{
        HdlPropertyIntent,
        HdlPropertyRelation,
        HdlPropertyRequirement,
        HdlPropertyValue,
    };

    fn target(id: &str) -> HdlTargetDescription {
        HdlTargetDescription::new(
            TargetId::new(id).expect("test target ID should be valid"),
            "Test target",
            "test.target",
        )
        .expect("test target description should be valid")
    }

    fn property(
        key: &str,
        value: &str,
        intent: HdlPropertyIntent,
    ) -> HdlPropertyRequirement {
        HdlPropertyRequirement::new(
            key,
            HdlPropertyRelation::Equal,
            HdlPropertyValue::Text(value.to_owned()),
            None,
            intent,
        )
        .expect("test property should be valid")
    }

    #[test]
    fn empty_collection_matches_only_its_target_identity() {
        let description = target("test.target");
        let constraints = HdlTargetConstraints::new(
            TargetId::new("test.target").expect("valid target ID"),
        );

        assert!(constraints.is_empty());
        assert_eq!(constraints.len(), 0);
        assert_eq!(constraints.target_id().as_str(), "test.target");
        assert!(constraints.validate_for(&description).is_ok());
    }

    #[test]
    fn rejects_validation_against_a_different_target() {
        let constraints = HdlTargetConstraints::new(
            TargetId::new("target.alpha").expect("valid target ID"),
        );

        let error = constraints
            .validate_for(&target("target.beta"))
            .expect_err("a different target must not match");

        assert!(matches!(
            error,
            HdlTargetConstraintsError::TargetMismatch {
                expected,
                actual,
            } if expected == "target.alpha" && actual == "target.beta"
        ));
    }

    #[test]
    fn preserves_constraints_in_insertion_order() {
        let mut constraints = HdlTargetConstraints::new(
            TargetId::new("target.alpha").expect("valid target ID"),
        );

        constraints.push(
            HdlTargetConstraint::new(
                property(
                    "physical.pin",
                    "provider-defined-pin-A",
                    HdlPropertyIntent::Constraint,
                ),
                Some("board-profile".to_owned()),
            )
            .expect("first constraint should be valid"),
        );

        constraints.push(
            HdlTargetConstraint::new(
                property(
                    "timing.period",
                    "provider-defined-period",
                    HdlPropertyIntent::Required,
                ),
                Some("timing-profile".to_owned()),
            )
            .expect("second constraint should be valid"),
        );

        let keys = constraints
            .iter()
            .map(|entry| entry.property().key())
            .collect::<Vec<_>>();

        assert_eq!(keys, vec!["physical.pin", "timing.period"]);
        assert_eq!(constraints.len(), 2);
    }

    #[test]
    fn preserves_repeated_properties_instead_of_silently_merging_them() {
        let mut constraints = HdlTargetConstraints::new(
            TargetId::new("target.alpha").expect("valid target ID"),
        );

        for origin in ["profile-a", "profile-b"] {
            constraints.push(
                HdlTargetConstraint::new(
                    property(
                        "timing.period",
                        "same-property",
                        HdlPropertyIntent::Constraint,
                    ),
                    Some(origin.to_owned()),
                )
                .expect("constraint should be valid"),
            );
        }

        assert_eq!(constraints.len(), 2);

        let origins = constraints
            .iter()
            .map(|entry| entry.origin().expect("origin should be present"))
            .collect::<Vec<_>>();

        assert_eq!(origins, vec!["profile-a", "profile-b"]);
    }

    #[test]
    fn rejects_empty_or_control_character_origins() {
        let valid_property = property(
            "physical.pin",
            "provider-defined-pin-A",
            HdlPropertyIntent::Constraint,
        );

        assert!(matches!(
            HdlTargetConstraint::new(
                valid_property.clone(),
                Some(" \t ".to_owned()),
            ),
            Err(HdlTargetConstraintsError::EmptyField { .. })
        ));

        assert!(matches!(
            HdlTargetConstraint::new(
                valid_property,
                Some("profile\ninjected".to_owned()),
            ),
            Err(HdlTargetConstraintsError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn supports_missing_provenance_without_inventing_an_origin() {
        let entry = HdlTargetConstraint::new(
            property(
                "implementation.preference",
                "provider-defined-choice",
                HdlPropertyIntent::Preference,
            ),
            None,
        )
        .expect("missing optional provenance is valid");

        assert_eq!(entry.origin(), None);
        assert_eq!(entry.property().key(), "implementation.preference");
    }

    #[test]
    fn target_scope_validation_does_not_claim_feasibility() {
        let description = target("target.alpha");
        let mut constraints = HdlTargetConstraints::new(
            TargetId::new("target.alpha").expect("valid target ID"),
        );

        constraints.push(
            HdlTargetConstraint::new(
                property(
                    "physical.pin",
                    "provider-defined-pin-A",
                    HdlPropertyIntent::Constraint,
                ),
                None,
            )
            .expect("constraint should be valid"),
        );

        // This validates only structure and target identity. Whether the
        // target supports or can satisfy the property is not evaluated here.
        assert!(constraints.validate_for(&description).is_ok());
    }

    #[test]
    fn schema_contract_is_explicit() {
        assert_eq!(
            HdlTargetConstraints::module_name(),
            "zamani.hdl.targets.constraints"
        );
        assert_eq!(HdlTargetConstraints::schema_version(), 1);
    }
}
