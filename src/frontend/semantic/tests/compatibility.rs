#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Compatibility tests for the Zamani semantic-analysis migration.
//!
//! File: src/frontend/semantic/tests/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Protect compatibility between the existing `crate::semantic` analyzer
//! and the native `crate::frontend::semantic` subsystem.
//!
//! These tests define a shared, normalized observation contract so that
//! legacy and native implementations can be compared without requiring
//! either implementation to expose its private AST, symbol table, inference
//! state, or internal semantic-model representation.
//!
//! # AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/semantic.rs`
//! - `src/lib.rs`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/api.rs`
//! - `src/frontend/semantic/config.rs`
//! - `src/frontend/semantic/pipeline.rs`
//!
//! Normative language specifications define program meaning. Implementation
//! details must not silently become new language rules.
//!
//! # OWNS
//!
//! - A normalized observation format for cross-implementation comparisons.
//! - Exact comparison of observable semantic facts and diagnostics.
//! - Compatibility-result invariants.
//! - Tests ensuring incomplete analysis cannot certify validity.
//! - Tests ensuring target feasibility is not confused with source validity.
//! - Tests for deterministic semantic observations.
//!
//! # DOES_NOT_OWN
//!
//! - Production name resolution, type checking, or semantic analysis.
//! - A replacement AST, symbol table, diagnostic system, or semantic model.
//! - Parsing or grammar definitions.
//! - Quantum IR, classical IR, HDL IR, lowering, or execution.
//! - The implementation of the legacy-to-native migration adapter.
//!
//! # INTEGRATION CONTRACT
//!
//! The canonical API owns `SemanticResult`, `SemanticStatus`,
//! `SemanticDiagnostic`, and `SemanticModelState`.
//!
//! This file deliberately imports those actual API contracts rather than
//! inventing a second analyzer API. Its observation types are test adapters:
//! each analyzer must normalize its public results into the same observable
//! contract before the parity helpers can compare them.
//!
//! The migration adapter should:
//!
//! 1. Run the legacy analyzer through its existing supported entry point.
//! 2. Run the native analyzer through `frontend::semantic::api`.
//! 3. Normalize comparable source-level facts into `SemanticObservation`.
//! 4. Normalize diagnostics to stable codes, severity, and comparable source
//!    locations.
//! 5. Compare observations only for language features whose behavior is
//!    specified and implemented by both analyzers.
//!
//! The legacy and native analyzers may use different internal ASTs and
//! different internal type representations. Those implementation details
//! must not appear in the normalized observation.
//!
//! # COMPATIBILITY POLICY
//!
//! - Preserve specified behavior, not accidental legacy bugs.
//! - Record intentional semantic corrections as explicit migration changes.
//! - Never treat an unresolved or unknown type as proof of validity.
//! - Never treat a partial analysis as a complete result.
//! - Compare diagnostics in their returned order; do not sort them in the
//!   test harness, because sorting could conceal nondeterministic producers.
//! - Compare source spans only after both adapters map them to the same
//!   documented source-coordinate convention.
//! - Do not compare physical target decisions as source-level semantics.
//!
//! # SCALABILITY
//!
//! This file establishes no fixed language-level ceiling for program size,
//! declaration count, type parameters, resource quantities, tensor dimensions,
//! quantum resources, hardware widths, or computational domains.
//!
//! Test inputs are finite because test processes use finite resources.
//! Explicit operational budgets are tested as analysis policies, not as
//! limits on what a Zamani program is permitted to describe.
//!
//! No finite implementation can process literally infinite input. Practical
//! analysis remains bounded by available resources, representational limits,
//! implementation capability, and explicitly configured operational policy.
//!
//! # FREEZE CRITERIA
//!
//! Freeze this file when:
//!
//! - The native semantic module is exported and builds.
//! - The legacy and native adapters are connected to real analyzer entry
//!   points.
//! - Shared conformance cases produce matching normalized observations,
//!   except for explicitly approved semantic corrections.
//! - Diagnostics retain stable codes and deterministic ordering.
//! - Incomplete, aborted, failed, or invalid analysis cannot certify validity.
//! - The full workspace and relevant integration tests pass.

use crate::frontend::semantic::api::{
    SemanticDiagnostic,
    SemanticModelState,
    SemanticResult,
    SemanticStatus,
};

/// Source-level validity, kept separate from analysis completion and
/// target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityValidity {
    /// The analysis establishes that the checked program is semantically valid.
    Valid,

    /// The analysis completed and established one or more semantic errors.
    Invalid,

    /// Required semantic facts remain unresolved.
    Unresolved,

    /// The analysis did not complete sufficiently to establish validity.
    Incomplete,
}

/// Stable, implementation-independent representation of a diagnostic.
///
/// `code` must be a stable machine-readable identifier, not a localized
/// human-readable message.
///
/// `source_start` and `source_end` must use one shared coordinate convention
/// established by the compatibility adapter. If a location is unavailable,
/// both values must be `None`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticObservation {
    /// Stable diagnostic code.
    pub code: String,

    /// Whether the diagnostic represents a semantic error.
    pub is_error: bool,

    /// Normalized inclusive or exclusive source start, as documented by the
    /// adapter. Both implementations must use the same convention.
    pub source_start: Option<u64>,

    /// Normalized source end, using the same convention as `source_start`.
    pub source_end: Option<u64>,
}

impl DiagnosticObservation {
    /// Construct a normalized diagnostic observation.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        is_error: bool,
        source_start: Option<u64>,
        source_end: Option<u64>,
    ) -> Self {
        Self {
            code: code.into(),
            is_error,
            source_start,
            source_end,
        }
    }

    /// Validate the normalized source range.
    ///
    /// A missing range is permitted. A partially missing or reversed range
    /// is not, because it would make parity comparisons ambiguous.
    pub fn validate(&self) -> Result<(), ObservationError> {
        if self.code.trim().is_empty() {
            return Err(ObservationError::EmptyDiagnosticCode);
        }

        match (self.source_start, self.source_end) {
            (Some(start), Some(end)) if start <= end => Ok(()),
            (Some(_), Some(_)) => Err(ObservationError::ReversedSourceRange),
            (None, None) => Ok(()),
            _ => Err(ObservationError::PartialSourceRange),
        }
    }
}

/// A normalized semantic fact suitable for compatibility comparisons.
///
/// `key` identifies the kind and stable subject of a fact. `value` is its
/// canonical semantic representation. Neither field should contain pointer
/// addresses, allocation order, randomized hash values, or target-specific
/// runtime handles.
///
/// The adapter must not use `Debug` formatting as a persistent canonical
/// encoding. It should serialize values according to their actual semantic
/// contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactObservation {
    /// Stable semantic fact key.
    pub key: String,

    /// Canonical representation of the fact's meaning.
    pub value: String,
}

impl FactObservation {
    /// Construct a normalized semantic fact.
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Validate that the fact has a meaningful key.
    pub fn validate(&self) -> Result<(), ObservationError> {
        if self.key.trim().is_empty() {
            return Err(ObservationError::EmptyFactKey);
        }

        Ok(())
    }
}

/// Observable semantic outcome produced by one analyzer.
///
/// This is a testing boundary, not a replacement for the production semantic
/// model. It deliberately excludes implementation-private data.
///
/// `facts` and `diagnostics` are ordered sequences. Their order is observable
/// and must not be normalized by silently sorting them here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObservation {
    /// Whether the analysis completed all requested work.
    pub status: SemanticStatus,

    /// Source-level semantic validity, distinct from target feasibility.
    pub validity: CompatibilityValidity,

    /// Whether all correctness-relevant diagnostics are present.
    pub diagnostics_complete: bool,

    /// Canonical semantic facts visible to downstream consumers.
    pub facts: Vec<FactObservation>,

    /// Diagnostics in the order emitted by the analyzer.
    pub diagnostics: Vec<DiagnosticObservation>,
}

impl SemanticObservation {
    /// Construct a normalized observation.
    #[must_use]
    pub fn new(
        status: SemanticStatus,
        validity: CompatibilityValidity,
        diagnostics_complete: bool,
        facts: Vec<FactObservation>,
        diagnostics: Vec<DiagnosticObservation>,
    ) -> Self {
        Self {
            status,
            validity,
            diagnostics_complete,
            facts,
            diagnostics,
        }
    }

    /// Validate the structural integrity of an observation.
    ///
    /// This does not prove that an analyzer's semantic reasoning is correct.
    /// It catches malformed normalization output before comparing results.
    pub fn validate(&self) -> Result<(), ObservationError> {
        for fact in &self.facts {
            fact.validate()?;
        }

        for diagnostic in &self.diagnostics {
            diagnostic.validate()?;
        }

        if self.validity == CompatibilityValidity::Valid {
            if self.status != SemanticStatus::Complete {
                return Err(ObservationError::ValidButNotComplete);
            }

            if !self.diagnostics_complete {
                return Err(ObservationError::ValidWithIncompleteDiagnostics);
            }

            if self.diagnostics.iter().any(|diagnostic| diagnostic.is_error) {
                return Err(ObservationError::ValidWithErrorDiagnostic);
            }
        }

        if self.status != SemanticStatus::Complete
            && self.validity != CompatibilityValidity::Incomplete
        {
            return Err(ObservationError::NonCompleteAnalysisHasFinalValidity);
        }

        if self.diagnostics.iter().any(|diagnostic| diagnostic.is_error)
            && self.validity == CompatibilityValidity::Valid
        {
            return Err(ObservationError::ValidWithErrorDiagnostic);
        }

        Ok(())
    }

    /// Return whether the observation is allowed to certify semantic validity.
    #[must_use]
    pub fn establishes_validity(&self) -> bool {
        self.status == SemanticStatus::Complete
            && self.validity == CompatibilityValidity::Valid
            && self.diagnostics_complete
            && !self.diagnostics.iter().any(|diagnostic| diagnostic.is_error)
    }
}

/// Errors in a normalized compatibility observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationError {
    EmptyDiagnosticCode,
    EmptyFactKey,
    PartialSourceRange,
    ReversedSourceRange,
    ValidButNotComplete,
    ValidWithIncompleteDiagnostics,
    ValidWithErrorDiagnostic,
    NonCompleteAnalysisHasFinalValidity,
}

impl core::fmt::Display for ObservationError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::EmptyDiagnosticCode => "diagnostic code must not be empty",
            Self::EmptyFactKey => "semantic fact key must not be empty",
            Self::PartialSourceRange => {
                "source range must either have both endpoints or neither"
            }
            Self::ReversedSourceRange => "source range start exceeds its end",
            Self::ValidButNotComplete => {
                "incomplete analysis cannot establish semantic validity"
            }
            Self::ValidWithIncompleteDiagnostics => {
                "incomplete diagnostics cannot establish semantic validity"
            }
            Self::ValidWithErrorDiagnostic => {
                "an observation with error diagnostics cannot be valid"
            }
            Self::NonCompleteAnalysisHasFinalValidity => {
                "non-complete analysis must report incomplete validity"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ObservationError {}

/// Report the first observable difference between two analyzer results.
///
/// This comparison is deliberately exact. Callers must normalize only
/// implementation details that are outside the compatibility contract.
/// They must not sort diagnostics or erase semantic differences to make
/// the analyzers appear equivalent.
pub fn compare_observations(
    legacy: &SemanticObservation,
    native: &SemanticObservation,
) -> Result<(), CompatibilityMismatch> {
    legacy
        .validate()
        .map_err(CompatibilityMismatch::InvalidLegacyObservation)?;

    native
        .validate()
        .map_err(CompatibilityMismatch::InvalidNativeObservation)?;

    if legacy.status != native.status {
        return Err(CompatibilityMismatch::Status {
            legacy: legacy.status,
            native: native.status,
        });
    }

    if legacy.validity != native.validity {
        return Err(CompatibilityMismatch::Validity {
            legacy: legacy.validity,
            native: native.validity,
        });
    }

    if legacy.diagnostics_complete != native.diagnostics_complete {
        return Err(CompatibilityMismatch::DiagnosticCompleteness {
            legacy: legacy.diagnostics_complete,
            native: native.diagnostics_complete,
        });
    }

    if legacy.facts != native.facts {
        return Err(CompatibilityMismatch::Facts {
            legacy_count: legacy.facts.len(),
            native_count: native.facts.len(),
        });
    }

    if legacy.diagnostics != native.diagnostics {
        return Err(CompatibilityMismatch::Diagnostics {
            legacy_count: legacy.diagnostics.len(),
            native_count: native.diagnostics.len(),
        });
    }

    Ok(())
}

/// A mismatch between normalized legacy and native semantic outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityMismatch {
    InvalidLegacyObservation(ObservationError),
    InvalidNativeObservation(ObservationError),
    Status {
        legacy: SemanticStatus,
        native: SemanticStatus,
    },
    Validity {
        legacy: CompatibilityValidity,
        native: CompatibilityValidity,
    },
    DiagnosticCompleteness {
        legacy: bool,
        native: bool,
    },
    Facts {
        legacy_count: usize,
        native_count: usize,
    },
    Diagnostics {
        legacy_count: usize,
        native_count: usize,
    },
}

impl core::fmt::Display for CompatibilityMismatch {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLegacyObservation(error) => {
                write!(formatter, "invalid legacy observation: {error}")
            }
            Self::InvalidNativeObservation(error) => {
                write!(formatter, "invalid native observation: {error}")
            }
            Self::Status { legacy, native } => {
                write!(formatter, "analysis status differs: {legacy} vs {native}")
            }
            Self::Validity { legacy, native } => {
                write!(formatter, "semantic validity differs: {legacy:?} vs {native:?}")
            }
            Self::DiagnosticCompleteness { legacy, native } => write!(
                formatter,
                "diagnostic completeness differs: legacy={legacy}, native={native}"
            ),
            Self::Facts {
                legacy_count,
                native_count,
            } => write!(
                formatter,
                "semantic facts differ (legacy count {legacy_count}, native count {native_count})"
            ),
            Self::Diagnostics {
                legacy_count,
                native_count,
            } => write!(
                formatter,
                "diagnostics differ (legacy count {legacy_count}, native count {native_count})"
            ),
        }
    }
}

impl std::error::Error for CompatibilityMismatch {}

/// Assert parity between normalized results from the two analyzers.
///
/// Use this helper for each shared conformance case after the repository's
/// migration adapters are connected to actual analyzer entry points.
pub fn assert_compatible(
    legacy: &SemanticObservation,
    native: &SemanticObservation,
) {
    if let Err(mismatch) = compare_observations(legacy, native) {
        panic!("semantic compatibility failure: {mismatch}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestDiagnostic {
        error: bool,
    }

    impl SemanticDiagnostic for TestDiagnostic {
        fn is_error(&self) -> bool {
            self.error
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestModel {
        complete: bool,
        valid: bool,
    }

    impl SemanticModelState for TestModel {
        fn establishes_completion(&self) -> bool {
            self.complete
        }

        fn establishes_validity(&self) -> bool {
            self.valid
        }
    }

    fn valid_model() -> TestModel {
        TestModel {
            complete: true,
            valid: true,
        }
    }

    fn valid_observation() -> SemanticObservation {
        SemanticObservation::new(
            SemanticStatus::Complete,
            CompatibilityValidity::Valid,
            true,
            vec![
                FactObservation::new("declaration:function:main", "resolved"),
                FactObservation::new("type:main:return", "unit"),
            ],
            Vec::new(),
        )
    }

    #[test]
    fn identical_normalized_results_are_compatible() {
        let legacy = valid_observation();
        let native = valid_observation();

        assert_eq!(compare_observations(&legacy, &native), Ok(()));
        assert_compatible(&legacy, &native);
    }

    #[test]
    fn semantic_fact_difference_is_not_hidden() {
        let legacy = valid_observation();

        let mut native = valid_observation();
        native.facts[1].value = "integer".to_owned();

        assert!(matches!(
            compare_observations(&legacy, &native),
            Err(CompatibilityMismatch::Facts { .. })
        ));
    }

    #[test]
    fn diagnostic_order_is_part_of_the_observable_contract() {
        let mut legacy = valid_observation();
        legacy.validity = CompatibilityValidity::Invalid;
        legacy.diagnostics = vec![
            DiagnosticObservation::new("SEM001", true, Some(2), Some(3)),
            DiagnosticObservation::new("SEM002", true, Some(8), Some(9)),
        ];

        let mut native = legacy.clone();
        native.diagnostics.reverse();

        assert!(matches!(
            compare_observations(&legacy, &native),
            Err(CompatibilityMismatch::Diagnostics { .. })
        ));
    }

    #[test]
    fn diagnostic_code_and_source_location_are_compared() {
        let mut legacy = valid_observation();
        legacy.validity = CompatibilityValidity::Invalid;
        legacy.diagnostics = vec![
            DiagnosticObservation::new("SEM_UNRESOLVED_NAME", true, Some(4), Some(9)),
        ];

        let mut native = legacy.clone();
        native.diagnostics[0].source_start = Some(5);

        assert!(matches!(
            compare_observations(&legacy, &native),
            Err(CompatibilityMismatch::Diagnostics { .. })
        ));
    }

    #[test]
    fn malformed_diagnostic_ranges_are_rejected() {
        let diagnostic =
            DiagnosticObservation::new("SEM001", true, Some(10), Some(4));

        assert_eq!(
            diagnostic.validate(),
            Err(ObservationError::ReversedSourceRange)
        );
    }

    #[test]
    fn partial_diagnostic_ranges_are_rejected() {
        let diagnostic =
            DiagnosticObservation::new("SEM001", true, Some(10), None);

        assert_eq!(
            diagnostic.validate(),
            Err(ObservationError::PartialSourceRange)
        );
    }

    #[test]
    fn valid_observation_requires_complete_analysis() {
        let observation = SemanticObservation::new(
            SemanticStatus::Incomplete,
            CompatibilityValidity::Valid,
            true,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            observation.validate(),
            Err(ObservationError::ValidButNotComplete)
        );
        assert!(!observation.establishes_validity());
    }

    #[test]
    fn valid_observation_requires_complete_diagnostics() {
        let observation = SemanticObservation::new(
            SemanticStatus::Complete,
            CompatibilityValidity::Valid,
            false,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            observation.validate(),
            Err(ObservationError::ValidWithIncompleteDiagnostics)
        );
        assert!(!observation.establishes_validity());
    }

    #[test]
    fn error_diagnostic_prevents_validity() {
        let observation = SemanticObservation::new(
            SemanticStatus::Complete,
            CompatibilityValidity::Valid,
            true,
            Vec::new(),
            vec![DiagnosticObservation::new("SEM001", true, None, None)],
        );

        assert_eq!(
            observation.validate(),
            Err(ObservationError::ValidWithErrorDiagnostic)
        );
        assert!(!observation.establishes_validity());
    }

    #[test]
    fn noncomplete_status_requires_incomplete_validity() {
        let observation = SemanticObservation::new(
            SemanticStatus::Failed,
            CompatibilityValidity::Invalid,
            true,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            observation.validate(),
            Err(ObservationError::NonCompleteAnalysisHasFinalValidity)
        );
    }

    #[test]
    fn native_api_result_never_certifies_partial_analysis() {
        let result = SemanticResult::incomplete(
            Some(TestModel {
                complete: false,
                valid: true,
            }),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Incomplete);
        assert!(result.has_model());
        assert!(!result.is_valid());
    }

    #[test]
    fn native_api_result_requires_model_completion_and_validity() {
        let incomplete_model = TestModel {
            complete: false,
            valid: true,
        };

        let result =
            SemanticResult::complete(incomplete_model, Vec::<TestDiagnostic>::new());

        assert!(!result.is_valid());

        let invalid_model = TestModel {
            complete: true,
            valid: false,
        };

        let result =
            SemanticResult::complete(invalid_model, Vec::<TestDiagnostic>::new());

        assert!(!result.is_valid());
    }

    #[test]
    fn native_api_result_rejects_incomplete_diagnostic_collection() {
        let result = SemanticResult::from_parts_with_diagnostic_completeness(
            SemanticStatus::Complete,
            Some(valid_model()),
            Vec::<TestDiagnostic>::new(),
            false,
        );

        assert!(!result.diagnostics_complete());
        assert!(!result.is_valid());
    }

    #[test]
    fn native_api_result_rejects_error_diagnostics() {
        let result = SemanticResult::complete(
            valid_model(),
            vec![TestDiagnostic { error: true }],
        );

        assert!(result.has_errors());
        assert!(!result.is_valid());
    }

    #[test]
    fn native_api_result_accepts_complete_valid_model_without_errors() {
        let result = SemanticResult::complete(
            valid_model(),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Complete);
        assert!(result.is_complete());
        assert!(result.diagnostics_complete());
        assert!(result.is_valid());
    }

    #[test]
    fn warnings_do_not_automatically_invalidate_a_result() {
        let result = SemanticResult::complete(
            valid_model(),
            vec![TestDiagnostic { error: false }],
        );

        assert!(!result.has_errors());
        assert!(result.is_valid());
    }

    #[test]
    fn target_feasibility_is_not_part_of_semantic_parity() {
        // Target selection and feasibility are downstream concerns. These
        // normalized source-level observations must remain identical even
        // when downstream realizations differ.
        let legacy = valid_observation();
        let native = valid_observation();

        assert_eq!(compare_observations(&legacy, &native), Ok(()));
    }

    #[test]
    fn symbolic_requirements_are_compared_as_semantic_facts() {
        let mut legacy = valid_observation();
        legacy.facts.push(FactObservation::new(
            "resource:logical_qubits",
            "symbolic:required_qubits",
        ));

        let mut native = legacy.clone();
        native.facts[2].value = "constant:64".to_owned();

        assert!(matches!(
            compare_observations(&legacy, &native),
            Err(CompatibilityMismatch::Facts { .. })
        ));
    }

    #[test]
    fn arbitrary_fact_keys_are_supported_without_domain_enumeration() {
        let mut observation = valid_observation();

        observation.facts.push(FactObservation::new(
            "extension:future_domain:semantic_fact",
            "canonical-value",
        ));

        assert_eq!(observation.validate(), Ok(()));
    }

    #[test]
    fn observation_validation_rejects_empty_fact_keys() {
        let fact = FactObservation::new("  ", "value");

        assert_eq!(fact.validate(), Err(ObservationError::EmptyFactKey));
    }

    #[test]
    fn observation_validation_rejects_empty_diagnostic_codes() {
        let diagnostic = DiagnosticObservation::new("", true, None, None);

        assert_eq!(
            diagnostic.validate(),
            Err(ObservationError::EmptyDiagnosticCode)
        );
    }
}