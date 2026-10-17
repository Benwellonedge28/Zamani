#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Zamani classical scalar lowering.
//!
//! Path: `src/classical/lowering/scalar.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Converts validated classical scalar descriptors into target-independent
//! lowering plans. The plans preserve scalar categories and width
//! specifications without selecting a physical representation.
//!
//! # Ownership
//!
//! This module owns:
//! - ScalarLoweringPlan and its derived lowering metadata.
//! - ScalarRepresentation, describing the semantic category to be lowered.
//! - LoweringObligation, describing work required by downstream adapters.
//! - ScalarLoweringError and local plan validation.
//!
//! It does not own:
//! - Source syntax, ASTs, name resolution, or semantic type inference.
//! - Canonical semantic types or scalar descriptors.
//! - Canonical classical values or arithmetic.
//! - Canonical ZUIR types, values, operations, or identities.
//! - Hardware formats, register widths, device allocation, or scheduling.
//! - Numeric algorithms, runtime execution, or ABI serialization.
//!
//! # Authority
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/classical/types/scalar.rs`
//! - `src/quantum/ir/classical/`
//! - `src/zuir/ids.rs`
//! - `src/zuir/lowering/context.rs`
//!
//! The grammar defines source syntax; semantic analysis determines the
//! meaning of a scalar type. This module preserves that meaning for later
//! lowering. It must not infer source semantics from spelling or guess a
//! target representation.
//!
//! # Scalability and POCO-REAF
//!
//! There are no universal limits on scalar width, number of scalar types,
//! or number of lowering plans. Symbolic and arbitrary widths are preserved.
//!
//! A fixed width is a semantic property of the descriptor, not a promise
//! that every target supports it. Target feasibility belongs to a downstream
//! capability and representation analysis.
//!
//! Host-sized integers are not substitutes for arbitrary-width integers.
//! In particular, this module must never truncate a width or silently replace
//! a symbolic width with `usize`, `u64`, or a machine-native register width.
//!
//! Practical memory, execution, and target limits remain real. They must be
//! reported by the subsystem responsible for enforcing them.
//!
//! # Integration contract
//!
//! 1. `src/classical/lowering/mod.rs` declares `pub mod scalar;`.
//! 2. `src/classical/types/mod.rs` exports the canonical descriptor types.
//! 3. `src/classical/lowering/api.rs` may call `ScalarLoweringPlan::from_descriptor`
//!    after semantic analysis has validated the source type.
//! 4. `src/classical/lowering/context.rs` records entity mappings and lowering
//!    metadata; it does not own this plan or the canonical type descriptor.
//! 5. A ZUIR type adapter translates the plan to canonical ZUIR type IDs and
//!    type definitions using the actual interfaces in `src/zuir/`.
//! 6. Classical IR adapters preserve canonical value semantics in
//!    `src/quantum/ir/classical/`; this file does not create a second IR.
//! 7. Target-specific code checks real format, width, and capability support.
//! 8. Unsupported semantics must produce explicit diagnostics or a structured
//!    lowering error, never a silent narrowing conversion.
//!
//! # Independence
//!
//! This file depends only on the established scalar descriptor contract and
//! the Rust standard library. It does not assume that unfinished lowering
//! adapters, target registries, or diagnostics modules already exist.
//!
//! # Completion contract
//!
//! Plans must preserve descriptors exactly, report required follow-up work,
//! validate their own invariants, and remain deterministic. Unit tests here
//! establish local behavior; repository-wide compilation and integration
//! tests are separate release gates.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::classical::types::scalar::{
    ScalarKind,
    ScalarTypeDescriptor,
    ScalarTypeError,
    ScalarWidth,
};

/// Result type for scalar lowering operations.
pub type ScalarLoweringResult<T> = Result<T, ScalarLoweringError>;

/// Semantic scalar category represented by a lowering plan.
///
/// This enum describes source-level scalar meaning. It does not identify a
/// machine register format, ABI type, or hardware instruction operand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ScalarRepresentation {
    /// Logical boolean value.
    Boolean,
    /// One logical classical bit.
    Bit,
    /// Signed integer with the descriptor's width semantics.
    SignedInteger,
    /// Unsigned integer with the descriptor's width semantics.
    UnsignedInteger,
    /// Binary floating-point category.
    BinaryFloat,
    /// Decimal numeric category.
    Decimal,
    /// Exact rational numeric category.
    Rational,
    /// Complex numeric category.
    Complex,
    /// Character scalar category.
    Character,
    /// Extension-defined category, preserved without reinterpretation.
    Extension(String),
}

impl ScalarRepresentation {
    /// Derives a representation category from a canonical scalar kind.
    #[must_use]
    pub fn from_kind(kind: &ScalarKind) -> Self {
        match kind {
            ScalarKind::Boolean => Self::Boolean,
            ScalarKind::Bit => Self::Bit,
            ScalarKind::SignedInteger => Self::SignedInteger,
            ScalarKind::UnsignedInteger => Self::UnsignedInteger,
            ScalarKind::BinaryFloat => Self::BinaryFloat,
            ScalarKind::Decimal => Self::Decimal,
            ScalarKind::Rational => Self::Rational,
            ScalarKind::Complex => Self::Complex,
            ScalarKind::Character => Self::Character,
            ScalarKind::Extension(name) => Self::Extension(name.clone()),
        }
    }
}

/// A reason that a lowering plan needs additional processing.
///
/// Obligations are descriptive. They do not themselves execute conversions,
/// resolve symbolic expressions, or prove that a target is capable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LoweringObligation {
    /// A symbolic width must be resolved by the semantic or type-resolution
    /// layer before any operation requiring a concrete width.
    ResolveSymbolicWidth {
        /// Original symbolic expression, preserved verbatim.
        expression: String,
    },

    /// The descriptor has not specified a width.
    ///
    /// A downstream consumer must determine whether width information is
    /// necessary for its operation; it must not guess a width from the host.
    ResolveUnspecifiedWidth,

    /// The category's width is intentionally not a single fixed value.
    ///
    /// The downstream type adapter must preserve arbitrary-width semantics
    /// or explicitly diagnose target infeasibility.
    PreserveArbitraryWidth,

    /// The decimal category requires an implementation that defines its
    /// precision, rounding, range, and observable error behavior.
    DecimalSemanticsRequired,

    /// The rational category requires an implementation that preserves the
    /// specified exactness and checked resource behavior.
    RationalSemanticsRequired,

    /// An extension-specific adapter must interpret this category.
    ExtensionAdapterRequired {
        /// Exact extension category name from the descriptor.
        name: String,
    },

    /// The target adapter must prove that the requested fixed width and
    /// category can be represented without changing semantic meaning.
    CheckTargetRepresentation,
}

/// A target-independent scalar lowering plan.
///
/// The original descriptor is retained so downstream adapters cannot lose
/// symbolic width expressions or confuse an unspecified width with an
/// arbitrary-width type.
///
/// A plan is not a target-specific type, machine instruction, ABI value,
/// executable operation, or proof of target feasibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarLoweringPlan {
    descriptor: ScalarTypeDescriptor,
    representation: ScalarRepresentation,
    obligations: BTreeSet<LoweringObligation>,
}

impl ScalarLoweringPlan {
    /// Creates a scalar lowering plan from a validated descriptor.
    ///
    /// The function does not choose a target representation. It records
    /// obligations required by the width and category contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the descriptor fails its canonical local
    /// validation contract.
    pub fn from_descriptor(
        descriptor: &ScalarTypeDescriptor,
    ) -> ScalarLoweringResult<Self> {
        descriptor
            .validate()
            .map_err(ScalarLoweringError::InvalidDescriptor)?;

        let representation = ScalarRepresentation::from_kind(descriptor.kind());
        let mut obligations = BTreeSet::new();

        match descriptor.width() {
            ScalarWidth::Unspecified => {
                obligations.insert(LoweringObligation::ResolveUnspecifiedWidth);
            }
            ScalarWidth::Fixed(_) => {
                obligations.insert(LoweringObligation::CheckTargetRepresentation);
            }
            ScalarWidth::Arbitrary => {
                obligations.insert(LoweringObligation::PreserveArbitraryWidth);
            }
            ScalarWidth::Symbolic(expression) => {
                obligations.insert(LoweringObligation::ResolveSymbolicWidth {
                    expression: expression.clone(),
                });
            }
        }

        match descriptor.kind() {
            ScalarKind::Decimal => {
                obligations.insert(LoweringObligation::DecimalSemanticsRequired);
            }
            ScalarKind::Rational => {
                obligations.insert(LoweringObligation::RationalSemanticsRequired);
            }
            ScalarKind::Extension(name) => {
                obligations.insert(LoweringObligation::ExtensionAdapterRequired {
                    name: name.clone(),
                });
            }
            _ => {}
        }

        Ok(Self {
            descriptor: descriptor.clone(),
            representation,
            obligations,
        })
    }

    /// Returns the exact validated descriptor preserved by this plan.
    #[must_use]
    pub fn descriptor(&self) -> &ScalarTypeDescriptor {
        &self.descriptor
    }

    /// Returns the target-independent semantic category.
    #[must_use]
    pub fn representation(&self) -> &ScalarRepresentation {
        &self.representation
    }

    /// Returns the ordered set of outstanding lowering obligations.
    #[must_use]
    pub fn obligations(&self) -> &BTreeSet<LoweringObligation> {
        &self.obligations
    }

    /// Returns whether the plan has any outstanding obligations.
    ///
    /// An empty set would mean that this module has no additional work to
    /// report. It would still not prove target feasibility or IR legality.
    #[must_use]
    pub fn has_obligations(&self) -> bool {
        !self.obligations.is_empty()
    }

    /// Validates that derived metadata is consistent with the descriptor.
    ///
    /// This recomputes the plan from the preserved descriptor and compares
    /// the result. It is useful at trust boundaries and after deserialization
    /// performed by a future adapter.
    pub fn validate(&self) -> ScalarLoweringResult<()> {
        let expected = Self::from_descriptor(&self.descriptor)?;

        if expected.representation != self.representation
            || expected.obligations != self.obligations
        {
            return Err(ScalarLoweringError::InconsistentPlan);
        }

        Ok(())
    }

    /// Returns whether the plan preserves an unresolved symbolic width.
    #[must_use]
    pub fn has_symbolic_width(&self) -> bool {
        matches!(self.descriptor.width(), ScalarWidth::Symbolic(_))
    }

    /// Returns whether the plan requests an arbitrary-width representation.
    #[must_use]
    pub fn requires_arbitrary_width(&self) -> bool {
        matches!(self.descriptor.width(), ScalarWidth::Arbitrary)
    }

    /// Returns whether the category is extension-defined.
    #[must_use]
    pub fn is_extension_category(&self) -> bool {
        matches!(self.descriptor.kind(), ScalarKind::Extension(_))
    }
}

/// Errors produced while constructing or validating scalar lowering plans.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScalarLoweringError {
    /// The canonical descriptor failed validation.
    InvalidDescriptor(ScalarTypeError),

    /// Derived metadata disagrees with the preserved descriptor.
    InconsistentPlan,
}

impl fmt::Display for ScalarLoweringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDescriptor(error) => {
                write!(formatter, "invalid scalar descriptor for lowering: {error}")
            }
            Self::InconsistentPlan => {
                formatter.write_str("scalar lowering plan metadata is inconsistent")
            }
        }
    }
}

impl Error for ScalarLoweringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDescriptor(error) => Some(error),
            Self::InconsistentPlan => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(
        kind: ScalarKind,
        width: ScalarWidth,
    ) -> ScalarLoweringPlan {
        let descriptor = ScalarTypeDescriptor::new(kind, width)
            .expect("test descriptor must be valid");

        ScalarLoweringPlan::from_descriptor(&descriptor)
            .expect("test lowering plan must be valid")
    }

    #[test]
    fn preserves_signed_integer_width() {
        let lowering = plan(
            ScalarKind::SignedInteger,
            ScalarWidth::fixed(257).expect("positive width"),
        );

        assert_eq!(
            lowering.representation(),
            &ScalarRepresentation::SignedInteger
        );
        assert_eq!(
            lowering.descriptor().width().as_fixed(),
            Some(257)
        );
        assert!(lowering
            .obligations()
            .contains(&LoweringObligation::CheckTargetRepresentation));
        assert!(lowering.validate().is_ok());
    }

    #[test]
    fn preserves_symbolic_width_without_evaluating_it() {
        let expression = "width_of(input) * 3".to_owned();
        let lowering = plan(
            ScalarKind::UnsignedInteger,
            ScalarWidth::symbolic(expression.clone())
                .expect("non-empty symbolic expression"),
        );

        assert!(lowering.has_symbolic_width());
        assert!(lowering.obligations().contains(
            &LoweringObligation::ResolveSymbolicWidth {
                expression,
            }
        ));
        assert!(lowering.validate().is_ok());
    }

    #[test]
    fn arbitrary_width_is_not_replaced_with_host_width() {
        let lowering = plan(
            ScalarKind::UnsignedInteger,
            ScalarWidth::Arbitrary,
        );

        assert!(lowering.requires_arbitrary_width());
        assert!(lowering.obligations().contains(
            &LoweringObligation::PreserveArbitraryWidth
        ));
        assert_eq!(
            lowering.descriptor().width(),
            &ScalarWidth::Arbitrary
        );
    }

    #[test]
    fn unspecified_width_differs_from_arbitrary_width() {
        let unspecified = plan(
            ScalarKind::SignedInteger,
            ScalarWidth::Unspecified,
        );
        let arbitrary = plan(
            ScalarKind::SignedInteger,
            ScalarWidth::Arbitrary,
        );

        assert_ne!(
            unspecified.descriptor().width(),
            arbitrary.descriptor().width()
        );
        assert!(unspecified.obligations().contains(
            &LoweringObligation::ResolveUnspecifiedWidth
        ));
        assert!(arbitrary.obligations().contains(
            &LoweringObligation::PreserveArbitraryWidth
        ));
    }

    #[test]
    fn decimal_and_rational_keep_their_semantic_obligations() {
        let decimal = plan(
            ScalarKind::Decimal,
            ScalarWidth::Unspecified,
        );
        let rational = plan(
            ScalarKind::Rational,
            ScalarWidth::Unspecified,
        );

        assert!(decimal.obligations().contains(
            &LoweringObligation::DecimalSemanticsRequired
        ));
        assert!(rational.obligations().contains(
            &LoweringObligation::RationalSemanticsRequired
        ));
    }

    #[test]
    fn extension_category_is_preserved_exactly() {
        let name = "vendor_or_future_domain.scalar_v2".to_owned();
        let lowering = plan(
            ScalarKind::Extension(name.clone()),
            ScalarWidth::Symbolic("extension_width".to_owned()),
        );

        assert_eq!(
            lowering.representation(),
            &ScalarRepresentation::Extension(name.clone())
        );
        assert!(lowering.obligations().contains(
            &LoweringObligation::ExtensionAdapterRequired { name }
        ));
    }

    #[test]
    fn fixed_width_does_not_claim_target_support() {
        let lowering = plan(
            ScalarKind::BinaryFloat,
            ScalarWidth::fixed(128).expect("positive width"),
        );

        assert!(lowering.obligations().contains(
            &LoweringObligation::CheckTargetRepresentation
        ));
        assert!(lowering.has_obligations());
    }

    #[test]
    fn built_in_scalar_categories_map_deterministically() {
        let cases = [
            (ScalarKind::Boolean, ScalarRepresentation::Boolean),
            (ScalarKind::Bit, ScalarRepresentation::Bit),
            (
                ScalarKind::SignedInteger,
                ScalarRepresentation::SignedInteger,
            ),
            (
                ScalarKind::UnsignedInteger,
                ScalarRepresentation::UnsignedInteger,
            ),
            (
                ScalarKind::BinaryFloat,
                ScalarRepresentation::BinaryFloat,
            ),
            (ScalarKind::Decimal, ScalarRepresentation::Decimal),
            (ScalarKind::Rational, ScalarRepresentation::Rational),
            (ScalarKind::Complex, ScalarRepresentation::Complex),
            (ScalarKind::Character, ScalarRepresentation::Character),
        ];

        for (kind, expected) in cases {
            assert_eq!(ScalarRepresentation::from_kind(&kind), expected);
        }
    }

    #[test]
    fn zero_fixed_width_is_rejected_by_the_descriptor_contract() {
        assert!(ScalarWidth::fixed(0).is_err());
    }

    #[test]
    fn plan_validation_detects_inconsistent_derived_metadata() {
        let descriptor = ScalarTypeDescriptor::new(
            ScalarKind::SignedInteger,
            ScalarWidth::fixed(32).expect("positive width"),
        )
        .expect("valid descriptor");

        let mut lowering = ScalarLoweringPlan::from_descriptor(&descriptor)
            .expect("valid plan");

        lowering.obligations.clear();

        assert_eq!(
            lowering.validate(),
            Err(ScalarLoweringError::InconsistentPlan)
        );
    }

    #[test]
    fn many_plans_do_not_use_a_fixed_semantic_capacity() {
        let mut plans = Vec::new();

        for index in 1_usize..=2_000 {
            let descriptor = ScalarTypeDescriptor::fixed_integer(
                index % 2 == 0,
                index,
            )
            .expect("positive width");

            plans.push(
                ScalarLoweringPlan::from_descriptor(&descriptor)
                    .expect("valid plan"),
            );
        }

        assert_eq!(plans.len(), 2_000);
        assert_eq!(
            plans.last().expect("plans are not empty")
                .descriptor().width().as_fixed(),
            Some(2_000)
        );
    }
}