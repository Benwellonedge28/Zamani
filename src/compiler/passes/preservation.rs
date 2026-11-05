
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Preservation
//!
//! Path: src/compiler/passes/preservation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide a compiler-facing semantic-preservation verification boundary
//! for transformations operating on canonical and domain-specific IR.
//!
//! This module distinguishes:
//!
//! 1. Preservation guarantees.
//! 2. Preservation evidence.
//! 3. Conclusive preservation verification.
//!
//! A pass declaration is not proof of correctness.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! Canonical pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Canonical IR:
//!     crate::zuir
//!
//! Canonical IR verification:
//!     crate::zuir::verify
//!
//! Compiler pass orchestration:
//!     crate::compiler::passes::manager
//!
//! Compiler pass pipeline:
//!     crate::compiler::passes::pipeline
//!
//! Analysis invalidation:
//!     crate::zuir::pass::invalidation
//!
//! DOMAIN AUTHORITIES
//!
//! Classical:
//!     crate::classical
//!
//! Quantum:
//!     crate::quantum::ir
//!
//! Hardware:
//!     crate::hdl::hdlir
//!
//! AI:
//!     crate::ai
//!
//! Hybrid:
//!     crate::hybrid
//!
//! Distributed:
//!     crate::distributed
//!
//! OWNERSHIP
//!
//! OWNS:
//! - Preservation property identifiers.
//! - Preservation guarantee declarations.
//! - Preservation evidence representation.
//! - Preservation verification reports.
//! - Verification result completeness checks.
//! - Rejection of unsupported or inconclusive proof claims.
//!
//! DOES NOT OWN:
//! - Pass registration.
//! - Pass execution.
//! - Pass identities or descriptors.
//! - Canonical IR definitions.
//! - Domain IR definitions.
//! - Semantic analysis.
//! - Type checking.
//! - Ownership analysis.
//! - Analysis result storage.
//! - Analysis invalidation.
//! - IR verification algorithms.
//! - Hardware allocation.
//! - Physical scheduling or routing.
//! - Runtime execution.
//!
//! SEMANTIC CONTRACT
//!
//! A preservation guarantee identifies properties that a transformation
//! must preserve between its input and output.
//!
//! A verifier must provide exactly one evidence result for every required
//! property.
//!
//! Missing, duplicate, unexpected, unsupported, inconclusive, or violated
//! evidence must never be interpreted as successful preservation.
//!
//! A successful report establishes only the properties explicitly
//! requested and conclusively checked by the configured verifier.
//!
//! SCALABILITY
//!
//! No fixed preservation-property count or computational-domain count.
//! Property identifiers are extensible and namespaced.
//! Dynamically sized standard-library collections are used.
//! No machine-specific capacity is embedded in this module.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::zuir::pass::contracts::{
    PassDescriptor,
    PassError,
    PassId,
};

// -----------------------------------------------------------------------------
// Preservation property identifiers
// -----------------------------------------------------------------------------

/// Stable, extensible identifier for a semantic-preservation property.
///
/// Examples:
///
/// - semantic.observable_behavior
/// - semantic.values
/// - semantic.control_flow
/// - classical.numeric_behavior
/// - quantum.measurement_behavior
/// - quantum.state_transitions
/// - hdl.clock_reset_behavior
/// - hdl.connectivity
/// - tensor.shape_semantics
/// - distributed.communication_order
/// - concurrency.synchronization
///
/// These examples are not a closed vocabulary.
///
/// New computational domains may introduce additional namespaced
/// preservation properties without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreservationProperty(String);

impl PreservationProperty {
    /// Construct and validate a namespaced preservation property.
    ///
    /// Segments must contain ASCII letters, digits, underscores or
    /// hyphens and must start with an ASCII alphanumeric character.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, PreservationError> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || !value.contains('.')
            || !value.split('.').all(valid_identifier_segment)
        {
            return Err(PreservationError::new(
                PreservationErrorKind::InvalidProperty,
                format!(
                    "invalid preservation property identifier: {value:?}"
                ),
            ));
        }

        Ok(Self(value))
    }

    /// Return the stable textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PreservationProperty {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn valid_identifier_segment(segment: &str) -> bool {
    let mut chars = segment.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    first.is_ascii_alphanumeric()
        && chars.all(|character| {
            character.is_ascii_alphanumeric()
                || character == '_'
                || character == '-'
        })
}

// -----------------------------------------------------------------------------
// Standard preservation property identifiers
// -----------------------------------------------------------------------------

/// Standard property identifiers.
///
/// These are convenience constants, not a closed property registry.
/// Additional properties are always permitted.
pub mod properties {
    pub const OBSERVABLE_BEHAVIOR: &str =
        "semantic.observable_behavior";

    pub const VALUES: &str =
        "semantic.values";

    pub const CONTROL_FLOW: &str =
        "semantic.control_flow";

    pub const TYPES: &str =
        "semantic.types";

    pub const EFFECTS: &str =
        "semantic.effects";

    pub const OWNERSHIP: &str =
        "semantic.ownership";

    pub const MEMORY: &str =
        "semantic.memory";

    pub const PROVENANCE: &str =
        "semantic.provenance";

    pub const CONCURRENCY: &str =
        "concurrency.synchronization";

    pub const NUMERIC_BEHAVIOR: &str =
        "classical.numeric_behavior";

    pub const QUANTUM_STATE: &str =
        "quantum.state_transitions";

    pub const QUANTUM_MEASUREMENT: &str =
        "quantum.measurement_behavior";

    pub const HDL_CLOCK_RESET: &str =
        "hdl.clock_reset_behavior";

    pub const HDL_CONNECTIVITY: &str =
        "hdl.connectivity";

    pub const HDL_TIMING: &str =
        "hdl.timing_constraints";

    pub const TENSOR_SHAPES: &str =
        "tensor.shape_semantics";

    pub const DISTRIBUTED_COMMUNICATION: &str =
        "distributed.communication_order";
}

// -----------------------------------------------------------------------------
// Preservation guarantee
// -----------------------------------------------------------------------------

/// Declares the properties a transformation is required to preserve.
///
/// This is a contract, not a proof.
///
/// A guarantee may contain properties from multiple computational
/// domains, making it suitable for heterogeneous transformations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationGuarantee {
    /// Pass responsible for the transformation.
    pub pass_id: PassId,

    /// Required preservation properties.
    pub required_properties: BTreeSet<PreservationProperty>,

    /// Human-readable explanation of the guarantee.
    pub description: String,
}

impl PreservationGuarantee {
    /// Construct an empty guarantee.
    ///
    /// Properties must be added before validation.
    #[must_use]
    pub fn new(
        pass_id: PassId,
        description: impl Into<String>,
    ) -> Self {
        Self {
            pass_id,
            required_properties: BTreeSet::new(),
            description: description.into(),
        }
    }

    /// Add a required preservation property.
    pub fn require(
        &mut self,
        property: PreservationProperty,
    ) -> &mut Self {
        self.required_properties.insert(property);
        self
    }

    /// Add multiple required preservation properties.
    pub fn require_all(
        &mut self,
        properties: impl IntoIterator<Item = PreservationProperty>,
    ) -> &mut Self {
        self.required_properties.extend(properties);
        self
    }

    /// Validate the guarantee before verification.
    pub fn validate(&self) -> Result<(), PreservationError> {
        if self.description.trim().is_empty() {
            return Err(PreservationError::for_pass(
                PreservationErrorKind::InvalidGuarantee,
                self.pass_id.clone(),
                "preservation guarantee description must not be empty",
            ));
        }

        if self.required_properties.is_empty() {
            return Err(PreservationError::for_pass(
                PreservationErrorKind::EmptyGuarantee,
                self.pass_id.clone(),
                "preservation guarantee must specify at least one property",
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Evidence status
// -----------------------------------------------------------------------------

/// The epistemic status of a preservation assessment.
///
/// Only Proven is accepted as conclusive preservation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreservationStatus {
    /// The verifier conclusively established preservation.
    Proven,

    /// The verifier established that preservation was violated.
    Violated,

    /// The verifier could not establish either preservation or violation.
    Inconclusive,

    /// No verifier implementation supports this property.
    Unsupported,
}

impl PreservationStatus {
    /// Whether the status constitutes conclusive positive evidence.
    #[must_use]
    pub const fn is_proven(self) -> bool {
        matches!(self, Self::Proven)
    }
}

// -----------------------------------------------------------------------------
// Preservation evidence
// -----------------------------------------------------------------------------

/// Evidence returned by an authoritative preservation verifier.
///
/// The verifier must assess the exact before and after IR snapshots
/// supplied by the caller.
///
/// An explanation is diagnostic information, not proof by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationEvidence {
    /// Property being assessed.
    pub property: PreservationProperty,

    /// Assessment result.
    pub status: PreservationStatus,

    /// Explanation supplied by the verifier.
    pub explanation: String,
}

impl PreservationEvidence {
    /// Construct preservation evidence.
    #[must_use]
    pub fn new(
        property: PreservationProperty,
        status: PreservationStatus,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            property,
            status,
            explanation: explanation.into(),
        }
    }

    /// Construct conclusive positive evidence.
    #[must_use]
    pub fn proven(
        property: PreservationProperty,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            property,
            PreservationStatus::Proven,
            explanation,
        )
    }

    /// Construct evidence of a semantic violation.
    #[must_use]
    pub fn violated(
        property: PreservationProperty,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            property,
            PreservationStatus::Violated,
            explanation,
        )
    }

    /// Construct inconclusive evidence.
    #[must_use]
    pub fn inconclusive(
        property: PreservationProperty,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            property,
            PreservationStatus::Inconclusive,
            explanation,
        )
    }

    /// Construct unsupported-property evidence.
    #[must_use]
    pub fn unsupported(
        property: PreservationProperty,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            property,
            PreservationStatus::Unsupported,
            explanation,
        )
    }
}

// -----------------------------------------------------------------------------
// Verification provider
// -----------------------------------------------------------------------------

/// Interface implemented by an authoritative preservation verifier.
///
/// `I` is deliberately generic.
///
/// This allows verification of canonical ZUIR and domain-specific IR
/// representations without creating duplicate IR types.
///
/// Implementations may delegate to:
///
/// - ZUIR verification.
/// - Classical equivalence checking.
/// - Quantum semantic verification.
/// - HDL formal equivalence.
/// - AI/tensor semantic verification.
/// - Domain-specific verification providers.
///
/// The provider must assess the supplied input and output snapshots.
/// It must not simply repeat the pass's declared guarantee.
///
/// A successful function return does not automatically imply that the
/// returned evidence is conclusive.
pub trait PreservationVerifier<I>: Send + Sync {
    /// Assess all requested preservation properties.
    ///
    /// Implementations must return exactly one evidence item for each
    /// requested property.
    ///
    /// Returning an error means that verification infrastructure failed.
    ///
    /// Unsupported checks must be reported as Unsupported evidence,
    /// not silently omitted.
    fn assess(
        &self,
        pass: &PassId,
        before: &I,
        after: &I,
        guarantee: &PreservationGuarantee,
    ) -> Result<Vec<PreservationEvidence>, PreservationError>;
}

// -----------------------------------------------------------------------------
// Preservation report
// -----------------------------------------------------------------------------

/// Complete preservation assessment for one transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationReport {
    /// Transformation being assessed.
    pub pass_id: PassId,

    /// Required properties.
    pub required_properties: BTreeSet<PreservationProperty>,

    /// Evidence, in deterministic property order.
    pub evidence: Vec<PreservationEvidence>,
}

impl PreservationReport {
    /// Whether every required property has conclusive positive evidence.
    #[must_use]
    pub fn is_fully_preserved(&self) -> bool {
        !self.required_properties.is_empty()
            && self.evidence.len() == self.required_properties.len()
            && self.evidence.iter().all(|item| {
                item.status.is_proven()
                    && self.required_properties.contains(&item.property)
            })
    }

    /// Return the number of properties conclusively proven.
    #[must_use]
    pub fn proven_count(&self) -> usize {
        self.evidence
            .iter()
            .filter(|item| item.status.is_proven())
            .count()
    }

    /// Return the number of properties not conclusively proven.
    #[must_use]
    pub fn unproven_count(&self) -> usize {
        self.required_properties
            .len()
            .saturating_sub(self.proven_count())
    }

    /// Return whether the report contains a violation.
    #[must_use]
    pub fn contains_violation(&self) -> bool {
        self.evidence.iter().any(|item| {
            item.status == PreservationStatus::Violated
        })
    }

    /// Return unsupported properties.
    pub fn unsupported_properties(
        &self,
    ) -> impl Iterator<Item = &PreservationEvidence> {
        self.evidence.iter().filter(|item| {
            item.status == PreservationStatus::Unsupported
        })
    }

    /// Return inconclusive properties.
    pub fn inconclusive_properties(
        &self,
    ) -> impl Iterator<Item = &PreservationEvidence> {
        self.evidence.iter().filter(|item| {
            item.status == PreservationStatus::Inconclusive
        })
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Machine-readable preservation verification error categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreservationErrorKind {
    InvalidProperty,
    InvalidGuarantee,
    EmptyGuarantee,
    PassMismatch,
    VerificationFailed,
    MissingEvidence,
    DuplicateEvidence,
    UnexpectedEvidence,
    PreservationViolated,
    PreservationInconclusive,
    PreservationUnsupported,
}

/// Structured preservation verification failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationError {
    pub kind: PreservationErrorKind,
    pub pass: Option<PassId>,
    pub property: Option<PreservationProperty>,
    pub message: String,
}

impl PreservationError {
    /// Construct an unattributed error.
    #[must_use]
    pub fn new(
        kind: PreservationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: None,
            property: None,
            message: message.into(),
        }
    }

    /// Construct a pass-attributed error.
    #[must_use]
    pub fn for_pass(
        kind: PreservationErrorKind,
        pass: PassId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            property: None,
            message: message.into(),
        }
    }

    /// Construct a property-attributed error.
    #[must_use]
    pub fn for_property(
        kind: PreservationErrorKind,
        pass: PassId,
        property: PreservationProperty,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            property: Some(property),
            message: message.into(),
        }
    }
}

impl fmt::Display for PreservationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{:?}: ", self.kind)?;

        if let Some(pass) = &self.pass {
            write!(formatter, "pass {pass}: ")?;
        }

        if let Some(property) = &self.property {
            write!(formatter, "property {property}: ")?;
        }

        formatter.write_str(&self.message)
    }
}

impl Error for PreservationError {}

// -----------------------------------------------------------------------------
// Canonical pass integration
// -----------------------------------------------------------------------------

/// Verify a transformation against its preservation guarantee.
///
/// The caller must supply the exact input and output IR snapshots.
///
/// This function:
///
/// 1. Validates the canonical pass descriptor.
/// 2. Validates the preservation guarantee.
/// 3. Checks that both identify the same pass.
/// 4. Invokes the authoritative verifier.
/// 5. Rejects missing, duplicate, and unexpected evidence.
/// 6. Produces a deterministic preservation report.
///
/// Important:
///
/// This function does not infer semantic equivalence from
/// `PassDescriptor::preserves_all_analyses`.
///
/// Analysis preservation and semantic preservation are different
/// contracts and must remain separate.
pub fn verify_pass_preservation<I>(
    descriptor: &PassDescriptor,
    guarantee: &PreservationGuarantee,
    before: &I,
    after: &I,
    verifier: &dyn PreservationVerifier<I>,
) -> Result<PreservationReport, PreservationError> {
    descriptor.validate().map_err(|error: PassError| {
        PreservationError::for_pass(
            PreservationErrorKind::InvalidGuarantee,
            descriptor.id.clone(),
            error.to_string(),
        )
    })?;

    guarantee.validate()?;

    if descriptor.id != guarantee.pass_id {
        return Err(PreservationError::for_pass(
            PreservationErrorKind::PassMismatch,
            descriptor.id.clone(),
            format!(
                "descriptor identifies {}, but guarantee identifies {}",
                descriptor.id,
                guarantee.pass_id,
            ),
        ));
    }

    let evidence = verifier.assess(
        &descriptor.id,
        before,
        after,
        guarantee,
    )?;

    validate_evidence(
        &descriptor.id,
        &guarantee.required_properties,
        evidence,
    )
}

// -----------------------------------------------------------------------------
// Evidence completeness and consistency
// -----------------------------------------------------------------------------

fn validate_evidence(
    pass_id: &PassId,
    required: &BTreeSet<PreservationProperty>,
    evidence: Vec<PreservationEvidence>,
) -> Result<PreservationReport, PreservationError> {
    let mut received = BTreeSet::new();

    for item in &evidence {
        if !required.contains(&item.property) {
            return Err(PreservationError::for_property(
                PreservationErrorKind::UnexpectedEvidence,
                pass_id.clone(),
                item.property.clone(),
                "verifier returned evidence for an unrequested property",
            ));
        }

        if !received.insert(item.property.clone()) {
            return Err(PreservationError::for_property(
                PreservationErrorKind::DuplicateEvidence,
                pass_id.clone(),
                item.property.clone(),
                "verifier returned duplicate evidence for a property",
            ));
        }
    }

    for property in required {
        if !received.contains(property) {
            return Err(PreservationError::for_property(
                PreservationErrorKind::MissingEvidence,
                pass_id.clone(),
                property.clone(),
                "verifier did not provide evidence for a required property",
            ));
        }
    }

    // Normalize evidence order independently of the provider's collection
    // order. This makes reports reproducible.
    let mut evidence = evidence;
    evidence.sort_by(|left, right| {
        left.property.cmp(&right.property)
    });

    Ok(PreservationReport {
        pass_id: pass_id.clone(),
        required_properties: required.clone(),
        evidence,
    })
}

// -----------------------------------------------------------------------------
// Strict preservation requirement
// -----------------------------------------------------------------------------

/// Require complete preservation of every requested property.
///
/// Unlike `verify_pass_preservation`, this function returns an error
/// whenever any property is violated, unsupported, or inconclusive.
///
/// The report remains available through the lower-level verification API
/// when callers need to inspect incomplete evidence.
pub fn require_preservation<I>(
    descriptor: &PassDescriptor,
    guarantee: &PreservationGuarantee,
    before: &I,
    after: &I,
    verifier: &dyn PreservationVerifier<I>,
) -> Result<PreservationReport, PreservationError> {
    let report = verify_pass_preservation(
        descriptor,
        guarantee,
        before,
        after,
        verifier,
    )?;

    if report.is_fully_preserved() {
        return Ok(report);
    }

    for item in &report.evidence {
        let kind = match item.status {
            PreservationStatus::Proven => continue,
            PreservationStatus::Violated => {
                PreservationErrorKind::PreservationViolated
            }
            PreservationStatus::Inconclusive => {
                PreservationErrorKind::PreservationInconclusive
            }
            PreservationStatus::Unsupported => {
                PreservationErrorKind::PreservationUnsupported
            }
        };

        return Err(PreservationError::for_property(
            kind,
            report.pass_id.clone(),
            item.property.clone(),
            item.explanation.clone(),
        ));
    }

    Err(PreservationError::for_pass(
        PreservationErrorKind::MissingEvidence,
        report.pass_id,
        "preservation report is incomplete",
    ))
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::pass::contracts::{
        PassId,
        PassVersion,
    };

    #[derive(Debug)]
    struct TestVerifier {
        status: PreservationStatus,
    }

    impl PreservationVerifier<u64> for TestVerifier {
        fn assess(
            &self,
            _pass: &PassId,
            before: &u64,
            after: &u64,
            guarantee: &PreservationGuarantee,
        ) -> Result<Vec<PreservationEvidence>, PreservationError> {
            Ok(guarantee
                .required_properties
                .iter()
                .map(|property| {
                    let status = if before == after {
                        self.status
                    } else {
                        PreservationStatus::Violated
                    };

                    PreservationEvidence::new(
                        property.clone(),
                        status,
                        "Test verifier assessment",
                    )
                })
                .collect())
        }
    }

    fn descriptor() -> PassDescriptor {
        PassDescriptor::new(
            PassId::new("core.test").unwrap(),
            PassVersion::new(1, 0, 0),
            "Test transformation",
        )
    }

    fn property(name: &str) -> PreservationProperty {
        PreservationProperty::new(name).unwrap()
    }

    fn guarantee() -> PreservationGuarantee {
        let mut guarantee = PreservationGuarantee::new(
            descriptor().id,
            "Preserve observable behavior",
        );

        guarantee.require(property(
            properties::OBSERVABLE_BEHAVIOR,
        ));

        guarantee
    }

    #[test]
    fn accepts_namespaced_properties_from_future_domains() {
        for name in [
            "semantic.observable_behavior",
            "classical.numeric_behavior",
            "quantum.measurement_behavior",
            "hdl.clock_reset_behavior",
            "tensor.shape_semantics",
            "distributed.communication_order",
            "future_domain.new_property",
        ] {
            assert_eq!(
                PreservationProperty::new(name).unwrap().as_str(),
                name,
            );
        }
    }

    #[test]
    fn rejects_malformed_property_identifiers() {
        for name in [
            "",
            "observable_behavior",
            ".semantic",
            "semantic.",
            "semantic..behavior",
            "semantic.bad name",
            "semantic/bad",
            " semantic.behavior",
        ] {
            assert!(
                PreservationProperty::new(name).is_err(),
                "unexpectedly accepted {name:?}",
            );
        }
    }

    #[test]
    fn rejects_empty_guarantees() {
        let guarantee = PreservationGuarantee::new(
            descriptor().id,
            "Empty guarantee",
        );

        assert_eq!(
            guarantee.validate().unwrap_err().kind,
            PreservationErrorKind::EmptyGuarantee,
        );
    }

    #[test]
    fn rejects_blank_guarantee_descriptions() {
        let mut guarantee = guarantee();
        guarantee.description = "  ".to_owned();

        assert_eq!(
            guarantee.validate().unwrap_err().kind,
            PreservationErrorKind::InvalidGuarantee,
        );
    }

    #[test]
    fn accepts_complete_positive_evidence() {
        let descriptor = descriptor();
        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Proven,
        };

        let report = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap();

        assert!(report.is_fully_preserved());
        assert_eq!(report.proven_count(), 1);
        assert_eq!(report.unproven_count(), 0);
    }

    #[test]
    fn rejects_semantic_violations_in_strict_mode() {
        let descriptor = descriptor();
        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Violated,
        };

        let error = require_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::PreservationViolated,
        );
    }

    #[test]
    fn rejects_inconclusive_evidence_in_strict_mode() {
        let descriptor = descriptor();
        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Inconclusive,
        };

        let error = require_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::PreservationInconclusive,
        );
    }

    #[test]
    fn rejects_unsupported_properties_in_strict_mode() {
        let descriptor = descriptor();
        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Unsupported,
        };

        let error = require_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::PreservationUnsupported,
        );
    }

    #[test]
    fn rejects_descriptor_guarantee_pass_mismatch() {
        let descriptor = descriptor();

        let mut guarantee = guarantee();
        guarantee.pass_id = PassId::new("quantum.other").unwrap();

        let verifier = TestVerifier {
            status: PreservationStatus::Proven,
        };

        let error = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::PassMismatch,
        );
    }

    #[test]
    fn rejects_incomplete_verifier_results() {
        struct MissingVerifier;

        impl PreservationVerifier<u64> for MissingVerifier {
            fn assess(
                &self,
                _pass: &PassId,
                _before: &u64,
                _after: &u64,
                _guarantee: &PreservationGuarantee,
            ) -> Result<Vec<PreservationEvidence>, PreservationError> {
                Ok(Vec::new())
            }
        }

        let descriptor = descriptor();
        let guarantee = guarantee();

        let error = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &MissingVerifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::MissingEvidence,
        );
    }

    #[test]
    fn rejects_duplicate_evidence() {
        struct DuplicateVerifier;

        impl PreservationVerifier<u64> for DuplicateVerifier {
            fn assess(
                &self,
                _pass: &PassId,
                _before: &u64,
                _after: &u64,
                guarantee: &PreservationGuarantee,
            ) -> Result<Vec<PreservationEvidence>, PreservationError> {
                let property = guarantee
                    .required_properties
                    .iter()
                    .next()
                    .unwrap()
                    .clone();

                Ok(vec![
                    PreservationEvidence::proven(
                        property.clone(),
                        "First",
                    ),
                    PreservationEvidence::proven(
                        property,
                        "Duplicate",
                    ),
                ])
            }
        }

        let descriptor = descriptor();
        let guarantee = guarantee();

        let error = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &DuplicateVerifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::DuplicateEvidence,
        );
    }

    #[test]
    fn rejects_unexpected_evidence() {
        struct UnexpectedVerifier;

        impl PreservationVerifier<u64> for UnexpectedVerifier {
            fn assess(
                &self,
                _pass: &PassId,
                _before: &u64,
                _after: &u64,
                _guarantee: &PreservationGuarantee,
            ) -> Result<Vec<PreservationEvidence>, PreservationError> {
                Ok(vec![
                    PreservationEvidence::proven(
                        PreservationProperty::new(
                            "future.extra_property",
                        ).unwrap(),
                        "Unexpected",
                    ),
                ])
            }
        }

        let descriptor = descriptor();
        let guarantee = guarantee();

        let error = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &UnexpectedVerifier,
        ).unwrap_err();

        assert_eq!(
            error.kind,
            PreservationErrorKind::UnexpectedEvidence,
        );
    }

    #[test]
    fn reports_are_deterministically_ordered() {
        let descriptor = descriptor();

        let mut guarantee = guarantee();
        guarantee.require(property("quantum.state_transitions"));
        guarantee.require(property("classical.numeric_behavior"));

        let verifier = TestVerifier {
            status: PreservationStatus::Proven,
        };

        let report = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap();

        let names: Vec<&str> = report.evidence
            .iter()
            .map(|item| item.property.as_str())
            .collect();

        assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn changed_input_output_is_not_assumed_preserved() {
        let descriptor = descriptor();
        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Proven,
        };

        let report = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &11,
            &verifier,
        ).unwrap();

        assert!(!report.is_fully_preserved());
        assert!(report.contains_violation());
    }

    #[test]
    fn analysis_preservation_is_not_semantic_preservation() {
        let mut descriptor = descriptor();
        descriptor.preserves_all_analyses = true;

        let guarantee = guarantee();

        let verifier = TestVerifier {
            status: PreservationStatus::Unsupported,
        };

        let report = verify_pass_preservation(
            &descriptor,
            &guarantee,
            &10,
            &10,
            &verifier,
        ).unwrap();

        assert!(!report.is_fully_preserved());
    }
}
