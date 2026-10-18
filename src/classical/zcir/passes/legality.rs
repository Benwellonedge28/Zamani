
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR transformation legality
//!
//! Path: `src/classical/zcir/passes/legality.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//!
//! ## Purpose
//!
//! Define a fail-closed boundary for assessing classical compiler passes and
//! proposed transformations without creating a second IR or pass engine.
//!
//! ## Authority and ownership
//!
//! Canonical pass identities, descriptors, versions, outcomes and errors are
//! owned by `src/zuir/pass/contracts.rs`.
//!
//! Pass execution and operational budgets are owned by the shared ZUIR pass
//! manager. IR structure and type correctness remain the responsibility of
//! the applicable canonical ZUIR verifier.
//!
//! This module owns:
//!
//! - Pass descriptor legality against an explicit execution policy.
//! - Required-analysis availability checks.
//! - Structured semantic-preservation evidence for transformations.
//! - Separation of semantic legality from target feasibility.
//! - Deterministic reporting of legality findings.
//!
//! This module does not own:
//!
//! - IR construction, parsing, type inference, or semantic analysis.
//! - A second pass registry, pass manager, or canonical IR.
//! - Proof of correctness for arbitrary pass implementations.
//! - Hardware discovery, resource allocation, scheduling, or code generation.
//! - A claim that an unavailable target makes a source program invalid.
//!
//! ## Integration contract
//!
//! 1. `passes/mod.rs` declares `pub mod legality;`.
//! 2. The shared pass manager calls `check_pass_legality` before executing a
//!    pass when the integration has access to the canonical descriptor and
//!    current analysis state.
//! 3. Transformation implementations provide explicit preservation evidence
//!    to `assess_transformation`.
//! 4. Canonical ZUIR verifiers continue to verify the resulting IR.
//! 5. Target feasibility is assessed separately; it must never override a
//!    semantic-legality failure.
//! 6. The legacy manager must migrate to canonical pass contracts before
//!    directly consuming these APIs with its own descriptor types.
//!
//! ## Scalability and safety
//!
//! No universal pass-count, graph-size, operation-count, hardware-count,
//! register-width, tensor-rank, or memory-capacity ceiling is introduced.
//! Collections grow dynamically within the resources available to the host.
//!
//! An optional operational budget belongs to the caller's execution policy.
//! This module does not claim that unbounded computation or allocation is
//! physically possible.
//!
//! Unsafe Rust is forbidden.

use std::collections::BTreeSet;

use crate::zuir::pass::contracts::{
    PassDescriptor, PassError, PassErrorKind,
};

/// Policy applied when deciding whether a pass may execute.
///
/// This policy checks declarations and known prerequisites. It does not
/// prove that a pass implementation is correct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LegalityPolicy {
    /// Reject passes that do not declare deterministic behavior.
    ///
    /// Determinism is a pass contract, not a proof of reproducibility.
    pub require_deterministic_passes: bool,

    /// Reject passes whose required analyses are unavailable.
    ///
    /// This must remain enabled in a production pipeline. It is configurable
    /// primarily to support staged integration and diagnostic tooling.
    pub require_available_analyses: bool,
}

/// Snapshot of analysis names available for the exact input IR state.
///
/// The caller is responsible for invalidating or recomputing analysis results
/// whenever the IR changes. This type does not compute analyses or establish
/// that a claimed analysis result is valid.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AvailableAnalyses {
    names: BTreeSet<String>,
}

impl AvailableAnalyses {
    /// Create an empty analysis snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a snapshot from analysis names already computed for this IR.
    ///
    /// Names are deduplicated and kept in deterministic order.
    #[must_use]
    pub fn from_names(
        names: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            names: names.into_iter().collect(),
        }
    }

    /// Return whether an analysis is available.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    /// Iterate over available analysis names in stable order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.names.iter().map(String::as_str)
    }

    /// Return the number of available analyses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return whether no analyses are available.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// Successful assessment of a pass descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassLegalityReport {
    /// Canonical identity of the pass that was assessed.
    pub pass_id: String,

    /// Names of the analyses required by the pass.
    ///
    /// This is descriptive metadata, not proof that those analyses were
    /// correctly computed or remain valid.
    pub required_analyses: BTreeSet<String>,

    /// Whether the pass declares deterministic behavior.
    pub declares_determinism: bool,
}

/// Validate a pass descriptor and its execution prerequisites.
///
/// # Errors
///
/// Returns a canonical `PassError` when the descriptor is malformed, a
/// required analysis is unavailable under the supplied policy, or a
/// nondeterministic pass is prohibited by policy.
///
/// # Important
///
/// A successful result means only that the checked contract requirements
/// passed. It does not prove semantic preservation, IR validity, termination,
/// resource feasibility, or implementation correctness.
pub fn check_pass_legality(
    descriptor: &PassDescriptor,
    available: &AvailableAnalyses,
    policy: LegalityPolicy,
) -> Result<PassLegalityReport, PassError> {
    descriptor.validate()?;

    if policy.require_deterministic_passes && !descriptor.deterministic {
        return Err(PassError::for_pass(
            PassErrorKind::NondeterministicPassRejected,
            descriptor.id.clone(),
            "pass legality policy requires declared determinism",
        ));
    }

    if policy.require_available_analyses {
        for required in &descriptor.required_analyses {
            if !available.contains(required) {
                return Err(PassError::for_pass(
                    PassErrorKind::MissingAnalysis,
                    descriptor.id.clone(),
                    format!("required analysis is unavailable: {required}"),
                ));
            }
        }
    }

    Ok(PassLegalityReport {
        pass_id: descriptor.id.to_string(),
        required_analyses: descriptor.required_analyses.clone(),
        declares_determinism: descriptor.deterministic,
    })
}

/// Semantic properties that a transformation may need to preserve.
///
/// `Unknown` is deliberately distinct from `Preserved`. A transformation
/// must not be accepted merely because nobody has checked a property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreservationStatus {
    /// Evidence establishes preservation of this property.
    Preserved,

    /// The transformation changes this property in a way that may change
    /// observable program behavior.
    Violated,

    /// No sufficient evidence is available to establish preservation.
    Unknown,
}

/// A semantic property relevant to transformation legality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticProperty {
    /// Observable results and externally visible behavior.
    ObservableBehavior,

    /// Ordering and multiplicity of observable side effects.
    EffectOrdering,

    /// Exceptions, traps, and failure behavior covered by the language model.
    ExceptionAndTrapBehavior,

    /// Integer, floating-point, overflow, rounding, and numerical semantics.
    NumericalSemantics,

    /// Aliasing, references, ownership, and memory access behavior.
    MemoryAndAliasing,

    /// Control-flow behavior, including termination where it is guaranteed.
    ControlFlow,

    /// Concurrency, synchronization, and memory-model guarantees.
    ConcurrencySemantics,

    /// Evaluation order and the language's sequencing guarantees.
    EvaluationOrder,
}

/// Evidence supplied by a transformation implementation or legality checker.
///
/// The evidence is declarative. The caller must not fabricate `Preserved`
/// entries merely to make a transformation pass this assessment.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreservationEvidence {
    properties: Vec<(SemanticProperty, PreservationStatus)>,
}

impl PreservationEvidence {
    /// Start with no preservation claims.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the assessment for a semantic property.
    ///
    /// Repeated declarations for the same property are rejected, preventing
    /// conflicting evidence from being silently resolved by insertion order.
    pub fn record(
        &mut self,
        property: SemanticProperty,
        status: PreservationStatus,
    ) -> Result<(), LegalityAssessmentError> {
        if self.properties.iter().any(|(existing, _)| *existing == property) {
            return Err(LegalityAssessmentError::DuplicateProperty(property));
        }

        self.properties.push((property, status));
        self.properties.sort_unstable_by_key(|(key, _)| *key);
        Ok(())
    }

    /// Look up the assessment for a property.
    #[must_use]
    pub fn status(
        &self,
        property: SemanticProperty,
    ) -> PreservationStatus {
        self.properties
            .iter()
            .find_map(|(candidate, status)| {
                (*candidate == property).then_some(*status)
            })
            .unwrap_or(PreservationStatus::Unknown)
    }

    /// Iterate over the recorded properties in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (SemanticProperty, PreservationStatus)> + '_ {
        self.properties.iter().copied()
    }
}

/// Target support is independent of semantic legality.
///
/// A valid transformation may be unavailable on a particular target. The
/// compiler should then select another valid lowering or report a feasibility
/// diagnostic rather than silently change the program's meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetFeasibility {
    /// The relevant target checker established support for this operation.
    Supported,

    /// The target cannot implement this operation under the supplied target
    /// configuration and constraints.
    Unsupported {
        reason: String,
    },

    /// No authoritative target assessment has been performed.
    Unknown,
}

/// Result of a semantic-preservation assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalityAssessment {
    /// Properties that are explicitly known to be violated.
    pub violations: BTreeSet<SemanticProperty>,

    /// Properties for which sufficient preservation evidence is missing.
    pub unknown_properties: BTreeSet<SemanticProperty>,

    /// Target feasibility, assessed independently of semantic legality.
    pub target_feasibility: TargetFeasibility,
}

impl LegalityAssessment {
    /// Whether all required semantic properties are explicitly preserved.
    #[must_use]
    pub fn is_semantically_legal(&self) -> bool {
        self.violations.is_empty() && self.unknown_properties.is_empty()
    }

    /// Whether the transformation is semantically legal and known to be
    /// supported by the selected target.
    #[must_use]
    pub fn is_legal_for_target(&self) -> bool {
        self.is_semantically_legal()
            && matches!(self.target_feasibility, TargetFeasibility::Supported)
    }
}

/// Error returned when transformation evidence cannot be assessed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegalityAssessmentError {
    /// A property was declared more than once.
    DuplicateProperty(SemanticProperty),
}

impl std::fmt::Display for LegalityAssessmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateProperty(property) => {
                write!(f, "duplicate preservation evidence for {property:?}")
            }
        }
    }
}

impl std::error::Error for LegalityAssessmentError {}

/// Assess whether a proposed transformation preserves required semantics.
///
/// Every property in `required_properties` must have explicit
/// `Preserved` evidence. A missing entry is `Unknown`, not a silent success.
///
/// Target feasibility is carried through unchanged and cannot override
/// semantic violations or missing preservation evidence.
#[must_use]
pub fn assess_transformation(
    evidence: &PreservationEvidence,
    required_properties: impl IntoIterator<Item = SemanticProperty>,
    target_feasibility: TargetFeasibility,
) -> LegalityAssessment {
    let mut violations = BTreeSet::new();
    let mut unknown_properties = BTreeSet::new();

    for property in required_properties {
        match evidence.status(property) {
            PreservationStatus::Preserved => {}
            PreservationStatus::Violated => {
                violations.insert(property);
            }
            PreservationStatus::Unknown => {
                unknown_properties.insert(property);
            }
        }
    }

    LegalityAssessment {
        violations,
        unknown_properties,
        target_feasibility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_properties() -> BTreeSet<SemanticProperty> {
        [
            SemanticProperty::ObservableBehavior,
            SemanticProperty::EffectOrdering,
            SemanticProperty::ExceptionAndTrapBehavior,
            SemanticProperty::NumericalSemantics,
            SemanticProperty::MemoryAndAliasing,
            SemanticProperty::ControlFlow,
            SemanticProperty::ConcurrencySemantics,
            SemanticProperty::EvaluationOrder,
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn missing_evidence_fails_closed() {
        let evidence = PreservationEvidence::new();

        let result = assess_transformation(
            &evidence,
            all_properties(),
            TargetFeasibility::Supported,
        );

        assert!(!result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
        assert!(result.violations.is_empty());
        assert_eq!(result.unknown_properties, all_properties());
    }

    #[test]
    fn explicit_violation_cannot_be_overridden_by_target_support() {
        let mut evidence = PreservationEvidence::new();

        evidence
            .record(
                SemanticProperty::EffectOrdering,
                PreservationStatus::Violated,
            )
            .expect("first declaration must succeed");

        let result = assess_transformation(
            &evidence,
            [SemanticProperty::EffectOrdering],
            TargetFeasibility::Supported,
        );

        assert!(!result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
        assert!(
            result
                .violations
                .contains(&SemanticProperty::EffectOrdering)
        );
    }

    #[test]
    fn target_unavailability_does_not_imply_semantic_illegality() {
        let mut evidence = PreservationEvidence::new();

        evidence
            .record(
                SemanticProperty::NumericalSemantics,
                PreservationStatus::Preserved,
            )
            .expect("evidence must be unique");

        let result = assess_transformation(
            &evidence,
            [SemanticProperty::NumericalSemantics],
            TargetFeasibility::Unsupported {
                reason: "target lacks the required operation".to_owned(),
            },
        );

        assert!(result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
    }

    #[test]
    fn unknown_target_is_not_treated_as_supported() {
        let mut evidence = PreservationEvidence::new();

        evidence
            .record(
                SemanticProperty::ObservableBehavior,
                PreservationStatus::Preserved,
            )
            .expect("evidence must be unique");

        let result = assess_transformation(
            &evidence,
            [SemanticProperty::ObservableBehavior],
            TargetFeasibility::Unknown,
        );

        assert!(result.is_semantically_legal());
        assert!(!result.is_legal_for_target());
    }

    #[test]
    fn duplicate_evidence_is_rejected() {
        let mut evidence = PreservationEvidence::new();

        evidence
            .record(
                SemanticProperty::ControlFlow,
                PreservationStatus::Preserved,
            )
            .expect("first declaration must succeed");

        assert_eq!(
            evidence.record(
                SemanticProperty::ControlFlow,
                PreservationStatus::Violated,
            ),
            Err(LegalityAssessmentError::DuplicateProperty(
                SemanticProperty::ControlFlow,
            ))
        );
    }

    #[test]
    fn evidence_iteration_is_deterministic() {
        let mut evidence = PreservationEvidence::new();

        evidence
            .record(
                SemanticProperty::NumericalSemantics,
                PreservationStatus::Preserved,
            )
            .expect("unique evidence");

        evidence
            .record(
                SemanticProperty::ObservableBehavior,
                PreservationStatus::Preserved,
            )
            .expect("unique evidence");

        let actual: Vec<_> = evidence.iter().map(|(property, _)| property).collect();
        let expected: Vec<_> = {
            let mut properties = vec![
                SemanticProperty::NumericalSemantics,
                SemanticProperty::ObservableBehavior,
            ];
            properties.sort_unstable();
            properties
        };

        assert_eq!(actual, expected);
    }
}
