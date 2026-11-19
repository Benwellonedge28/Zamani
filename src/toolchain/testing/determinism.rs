
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Determinism and Repeated-Build Verification
//!
//! Path: `src/toolchain/testing/determinism.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify determinism at the toolchain build boundary by comparing
//! independently executed builds that declare identical canonical inputs.
//!
//! This module consumes canonical input and artifact fingerprints. It does
//! not implement another fingerprint algorithm, compiler determinism policy,
//! dependency resolver, cache, artifact store, or build executor.
//!
//! ## Canonical authorities
//!
//! Language-level determinism:
//!     grammar/spec/determinism.md
//!
//! Source-level deterministic-build intent:
//!     grammar/compile/deterministic-builds.g4
//!
//! Source-level reproducibility intent:
//!     grammar/compile/reproducibility.g4
//!
//! Language architecture and ownership:
//!     grammar/DESIGN.md
//!     grammar/validation/semantic-boundaries.md
//!
//! POCO-REAF:
//!     grammar/specification/poco-reaf.md
//!
//! Compiler component determinism:
//!     crate::compiler::reproducibility::determinism
//!
//! Canonical build-input identity:
//!     crate::toolchain::build::inputs::BuildInputs
//!
//! Canonical fingerprint representation:
//!     crate::zuir::fingerprint::stable_hash::Fingerprint
//!
//! Artifact production and verification:
//!     crate::toolchain::build::outputs
//!     crate::compiler::artifacts
//!     crate::backend
//!
//! Build orchestration:
//!     crate::toolchain::build
//!
//! Provenance and trust:
//!     crate::compiler::reproducibility::provenance
//!     crate::compiler::security
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Repeated-build observation records.
//! - Stable artifact comparison keys.
//! - Validation of repeated-build evidence records.
//! - Deterministic comparison of build observations.
//! - Structured determinism findings and reports.
//! - Fail-closed classification of repeated-build outcomes.
//!
//! This module does not own:
//!
//! - Source-language determinism semantics.
//! - Determinism policy construction or interpretation.
//! - Compiler component inventories or declarations.
//! - Source discovery or manifest parsing.
//! - Dependency resolution or dependency graph construction.
//! - Build-input fingerprint construction.
//! - Artifact payload hashing or artifact verification.
//! - Compiler or backend execution.
//! - Cache admission, storage, eviction or invalidation.
//! - Numerical reproducibility algorithms.
//! - Runtime determinism or distributed consensus.
//! - Evidence authentication or digital signatures.
//!
//! ## Determinism and reproducibility
//!
//! Deterministic ordering is not equivalent to deterministic compilation.
//!
//! Deterministic compilation is not automatically equivalent to bit-for-bit
//! reproducible artifacts under different environments.
//!
//! Matching artifacts from repeated builds do not independently prove:
//!
//! - Semantic correctness.
//! - Compiler correctness.
//! - Artifact authenticity.
//! - Cross-machine portability.
//! - Runtime determinism.
//! - Numerical reproducibility.
//! - Physical target feasibility.
//!
//! A `Deterministic` report means only that:
//!
//! 1. The supplied compiler determinism assessment has status `Satisfied`.
//! 2. At least two distinct run references were supplied.
//! 3. Every run declares the same canonical build-input fingerprint.
//! 4. Every run reports the same non-empty set of artifact comparison keys.
//! 5. Every corresponding artifact has the same canonical content fingerprint.
//!
//! The caller is responsible for ensuring that run references identify
//! genuinely independent executions and that the supplied fingerprints
//! originate from the canonical authorities.
//!
//! ## Deterministic comparison
//!
//! Runs are ordered by their stable evidence reference.
//!
//! Artifacts are ordered by:
//!
//!     target identity -> artifact format -> logical artifact name
//!
//! Comparison is independent of:
//!
//! - Input iterator order.
//! - Artifact iterator order.
//! - Worker completion order.
//! - Hash-map iteration order.
//! - Thread identity.
//! - Process-local identifiers.
//! - Wall-clock timestamps.
//! - Hostname and incidental machine identity.
//!
//! Run references are evidence locators, not build inputs. They are never
//! incorporated into build-input identity or artifact comparison.
//!
//! ## POCO-REAF and open-world domains
//!
//! Artifact comparison keys use open-ended strings. No closed enumeration
//! of classical, quantum, HDL, AI, hybrid, distributed, networking,
//! cryptographic, accelerator or future domains is introduced.
//!
//! Target-specific artifacts are compared separately. Different target
//! realizations are not required to produce identical bytes; repeated builds
//! of the same declared realization are compared against each other.
//!
//! ## Scalability
//!
//! No application-defined universal maximum is imposed on:
//!
//! - Build runs.
//! - Artifacts.
//! - Targets.
//! - Artifact formats.
//! - Computing domains.
//! - Providers.
//! - Build-input dimensions.
//!
//! Dynamic collections grow according to supplied evidence and available
//! resources. Fallible vector reservations report resource exhaustion.
//! Rust representation limits, address-space limits, execution time and
//! caller-configured operational budgets remain applicable.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - Run references and artifact keys are validated.
//! - Duplicate run references are rejected.
//! - Duplicate artifact comparison keys are rejected.
//! - Fingerprints are treated as untrusted claims until checked by their
//!   canonical producer.
//! - This module does not authenticate evidence.
//! - Secrets and credentials must never be placed in references or keys.
//! - Missing evidence never produces a passing report.
//! - Resource exhaustion is reported rather than silently truncating data.
//!
//! ## Concurrency
//!
//! This module creates no threads, workers, locks, schedulers or global state.
//!
//! Independent reports can be evaluated concurrently. The caller controls
//! build execution and must provide completed, independently identified runs.
//!
//! ## Integration lifecycle
//!
//! ```text
//! Validated build request
//!         |
//!         v
//! Canonical BuildInputs
//!         |
//!         v
//! Actual build executor
//!         |
//!         v
//! Canonical artifact verification
//!         |
//!         v
//! Independent build-run evidence
//!         |
//!         v
//! Toolchain determinism assessment
//!         |
//!         +--> compiler determinism assessment
//!         +--> input identity comparison
//!         +--> artifact-set comparison
//!         +--> artifact-content comparison
//!         |
//!         v
//! Structured determinism report
//! ```
//!
//! This module verifies supplied observations. It does not itself execute
//! builds or claim that a test harness actually performed independent runs.

use std::cmp::Ordering;
use std::fmt;

use crate::compiler::reproducibility::determinism::{
    DeterminismAssessment as CompilerDeterminismAssessment,
    DeterminismAssessmentStatus as CompilerDeterminismStatus,
};

use crate::toolchain::build::inputs::BuildInputs;
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::ToolchainResult;

use crate::zuir::fingerprint::stable_hash::Fingerprint;

/// Public API version of the toolchain determinism-test contract.
pub const DETERMINISM_TESTING_API_VERSION: u16 = 1;

/// Stable operation identifier used in structured errors.
const OPERATION: &str = "testing.determinism";

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn invalid_input(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(OPERATION)
}

fn resource_exhausted(
    description: &'static str,
    source: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ResourceExhausted,
        format!("unable to reserve memory for {description}"),
    )
    .with_operation(OPERATION)
    .with_source(source)
}

fn reserve_one<T>(
    values: &mut Vec<T>,
    description: &'static str,
) -> ToolchainResult<()> {
    values
        .try_reserve(1)
        .map_err(|error| resource_exhausted(description, error))
}

fn validate_text(
    field: &'static str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(format!(
            "{field} must not be empty"
        )));
    }

    if value.trim() != value {
        return Err(invalid_input(format!(
            "{field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// ARTIFACT COMPARISON IDENTITY
// ============================================================================

/// Stable logical key used to match corresponding artifacts across builds.
///
/// The key deliberately contains no closed artifact-kind or target enum.
/// Its values are supplied by the canonical artifact and target authorities.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactComparisonKey {
    logical_name: String,
    target_identity: String,
    format_identity: String,
}

impl ArtifactComparisonKey {
    /// Construct an artifact comparison key.
    ///
    /// `logical_name` identifies the artifact within the logical build.
    ///
    /// `target_identity` identifies the exact target realization.
    ///
    /// `format_identity` identifies the canonical artifact representation.
    pub fn new(
        logical_name: impl Into<String>,
        target_identity: impl Into<String>,
        format_identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let logical_name = logical_name.into();
        let target_identity = target_identity.into();
        let format_identity = format_identity.into();

        validate_text("artifact logical name", &logical_name)?;
        validate_text("artifact target identity", &target_identity)?;
        validate_text("artifact format identity", &format_identity)?;

        Ok(Self {
            logical_name,
            target_identity,
            format_identity,
        })
    }

    /// Return the logical artifact name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    /// Return the exact target-realization identity.
    #[must_use]
    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    /// Return the canonical artifact-format identity.
    #[must_use]
    pub fn format_identity(&self) -> &str {
        &self.format_identity
    }
}

impl fmt::Display for ArtifactComparisonKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} @ {} [{}]",
            self.logical_name,
            self.target_identity,
            self.format_identity,
        )
    }
}

// ============================================================================
// ARTIFACT OBSERVATION
// ============================================================================

/// A verified artifact observation supplied by the canonical artifact layer.
///
/// `content_fingerprint` must be the canonical fingerprint of the actual
/// artifact payload or canonical artifact content. This module does not
/// calculate or authenticate that fingerprint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactObservation {
    key: ArtifactComparisonKey,
    content_fingerprint: Fingerprint,
}

impl ArtifactObservation {
    /// Construct an artifact observation.
    pub fn new(
        key: ArtifactComparisonKey,
        content_fingerprint: Fingerprint,
    ) -> Self {
        Self {
            key,
            content_fingerprint,
        }
    }

    /// Return the stable artifact comparison key.
    #[must_use]
    pub fn key(&self) -> &ArtifactComparisonKey {
        &self.key
    }

    /// Return the canonical content fingerprint.
    #[must_use]
    pub const fn content_fingerprint(&self) -> Fingerprint {
        self.content_fingerprint
    }
}

// ============================================================================
// BUILD-RUN EVIDENCE
// ============================================================================

/// Immutable observation of one completed build execution.
///
/// A run reference must identify a distinct execution record, such as a
/// CI job, test-run record or independently retained build report.
/// The reference is not proof that the execution was independent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismRun {
    reference: String,
    input_fingerprint: Fingerprint,
    artifacts: Vec<ArtifactObservation>,
}

impl DeterminismRun {
    /// Construct a run observation from a canonical build-input fingerprint.
    ///
    /// Artifact observations are sorted into canonical comparison-key order.
    /// Duplicate artifact keys are rejected.
    ///
    /// An empty artifact list is accepted as input evidence, but it cannot
    /// establish a deterministic artifact result.
    pub fn new(
        reference: impl Into<String>,
        input_fingerprint: Fingerprint,
        artifacts: impl IntoIterator<Item = ArtifactObservation>,
    ) -> ToolchainResult<Self> {
        let reference = reference.into();
        validate_text("build-run reference", &reference)?;

        let mut ordered_artifacts = Vec::new();

        for artifact in artifacts {
            reserve_one(
                &mut ordered_artifacts,
                "build-run artifact observations",
            )?;
            ordered_artifacts.push(artifact);
        }

        ordered_artifacts.sort_by(|left, right| {
            left.key.cmp(&right.key)
        });

        for pair in ordered_artifacts.windows(2) {
            if pair[0].key == pair[1].key {
                return Err(invalid_input(format!(
                    "duplicate artifact comparison key: {}",
                    pair[0].key
                )));
            }
        }

        Ok(Self {
            reference,
            input_fingerprint,
            artifacts: ordered_artifacts,
        })
    }

    /// Construct a run observation from canonical toolchain build inputs.
    ///
    /// This uses `BuildInputs::fingerprint()` and does not introduce another
    /// build-input fingerprint implementation.
    pub fn from_build_inputs(
        reference: impl Into<String>,
        inputs: &BuildInputs,
        artifacts: impl IntoIterator<Item = ArtifactObservation>,
    ) -> ToolchainResult<Self> {
        Self::new(
            reference,
            inputs.fingerprint(),
            artifacts,
        )
    }

    /// Return the stable evidence reference for this run.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Return the canonical build-input fingerprint.
    #[must_use]
    pub const fn input_fingerprint(&self) -> Fingerprint {
        self.input_fingerprint
    }

    /// Return artifact observations in canonical key order.
    #[must_use]
    pub fn artifacts(&self) -> &[ArtifactObservation] {
        &self.artifacts
    }

    /// Return the number of observed artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }
}

// ============================================================================
// REPORT STATUS AND FINDINGS
// ============================================================================

/// Overall result of repeated-build determinism verification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeterminismStatus {
    /// All required evidence and comparisons establish matching results.
    ///
    /// This is limited to the declared build-input and artifact scope.
    Deterministic,

    /// At least one artifact comparison differs despite matching inputs.
    NonDeterministic,

    /// Evidence is missing, incomparable or insufficient to establish a
    /// deterministic result.
    Inconclusive,
}

/// Classification of a repeated-build determinism finding.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeterminismFindingKind {
    /// No build-run observations were supplied.
    NoRuns,

    /// Fewer than two distinct build-run observations were supplied.
    InsufficientRuns,

    /// A run contains no artifact observations.
    NoArtifacts,

    /// The compiler's canonical determinism assessment is not satisfied.
    CompilerContractNotSatisfied {
        /// Actual status from the compiler-owned assessment.
        status: CompilerDeterminismStatus,
    },

    /// Build runs do not have the same canonical input identity.
    InputIdentityMismatch,

    /// Corresponding builds produced different artifact key sets.
    ArtifactSetMismatch,

    /// The same artifact key has different content fingerprints.
    ArtifactContentMismatch {
        /// Artifact whose content differed.
        artifact: ArtifactComparisonKey,
    },
}

/// One finding produced by a determinism assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismFinding {
    kind: DeterminismFindingKind,
}

impl DeterminismFinding {
    fn new(kind: DeterminismFindingKind) -> Self {
        Self { kind }
    }

    /// Return the finding classification.
    #[must_use]
    pub fn kind(&self) -> &DeterminismFindingKind {
        &self.kind
    }
}

// ============================================================================
// IMMUTABLE REPORT
// ============================================================================

/// Immutable result of comparing repeated build observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminismReport {
    status: DeterminismStatus,
    compiler_status: CompilerDeterminismStatus,
    run_count: usize,
    artifact_count: usize,
    input_fingerprint: Option<Fingerprint>,
    findings: Vec<DeterminismFinding>,
}

impl DeterminismReport {
    fn new(
        status: DeterminismStatus,
        compiler_status: CompilerDeterminismStatus,
        run_count: usize,
        artifact_count: usize,
        input_fingerprint: Option<Fingerprint>,
        findings: Vec<DeterminismFinding>,
    ) -> Self {
        Self {
            status,
            compiler_status,
            run_count,
            artifact_count,
            input_fingerprint,
            findings,
        }
    }

    /// Return the overall determinism result.
    #[must_use]
    pub const fn status(&self) -> DeterminismStatus {
        self.status
    }

    /// Return the status supplied by the canonical compiler assessment.
    #[must_use]
    pub const fn compiler_status(&self) -> CompilerDeterminismStatus {
        self.compiler_status
    }

    /// Return whether the repeated builds passed this assessment.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        matches!(self.status, DeterminismStatus::Deterministic)
    }

    /// Return the number of distinct run records evaluated.
    #[must_use]
    pub const fn run_count(&self) -> usize {
        self.run_count
    }

    /// Return the number of artifacts in the comparison baseline.
    #[must_use]
    pub const fn artifact_count(&self) -> usize {
        self.artifact_count
    }

    /// Return the common build-input fingerprint, when comparable.
    #[must_use]
    pub const fn input_fingerprint(&self) -> Option<Fingerprint> {
        self.input_fingerprint
    }

    /// Return findings in deterministic comparison order.
    #[must_use]
    pub fn findings(&self) -> &[DeterminismFinding] {
        &self.findings
    }

    /// Return whether the report contains findings.
    #[must_use]
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }
}

// ============================================================================
// DETERMINISM EVALUATION
// ============================================================================

/// Evaluate repeated builds against the canonical compiler determinism result.
///
/// The compiler assessment must have status `Satisfied`. A `NotRequired`,
/// `NoScope`, `BestEffort` or `Unsatisfied` compiler assessment cannot
/// establish the required compiler contract for this verification.
///
/// At least two distinct run references are required. Their input
/// fingerprints must match before artifact outputs are compared.
///
/// Duplicate run references are rejected as invalid evidence rather than
/// silently counted as independent executions.
pub fn evaluate(
    compiler_assessment: &CompilerDeterminismAssessment,
    runs: impl IntoIterator<Item = DeterminismRun>,
) -> ToolchainResult<DeterminismReport> {
    let mut ordered_runs = Vec::new();

    for run in runs {
        reserve_one(
            &mut ordered_runs,
            "determinism build-run observations",
        )?;
        ordered_runs.push(run);
    }

    ordered_runs.sort_by(|left, right| {
        left.reference.cmp(&right.reference)
    });

    for pair in ordered_runs.windows(2) {
        if pair[0].reference == pair[1].reference {
            return Err(invalid_input(format!(
                "duplicate build-run reference: {}",
                pair[0].reference
            )));
        }
    }

    let compiler_status = compiler_assessment.status();

    if ordered_runs.is_empty() {
        return Ok(DeterminismReport::new(
            DeterminismStatus::Inconclusive,
            compiler_status,
            0,
            0,
            None,
            vec![DeterminismFinding::new(
                DeterminismFindingKind::NoRuns,
            )],
        ));
    }

    if ordered_runs.len() < 2 {
        return Ok(DeterminismReport::new(
            DeterminismStatus::Inconclusive,
            compiler_status,
            ordered_runs.len(),
            ordered_runs[0].artifact_count(),
            Some(ordered_runs[0].input_fingerprint),
            vec![DeterminismFinding::new(
                DeterminismFindingKind::InsufficientRuns,
            )],
        ));
    }

    let baseline = &ordered_runs[0];
    let baseline_fingerprint = baseline.input_fingerprint;

    if baseline.artifacts.is_empty() {
        return Ok(DeterminismReport::new(
            DeterminismStatus::Inconclusive,
            compiler_status,
            ordered_runs.len(),
            0,
            Some(baseline_fingerprint),
            vec![DeterminismFinding::new(
                DeterminismFindingKind::NoArtifacts,
            )],
        ));
    }

    // Different input identities cannot be used to infer output
    // nondeterminism: they represent different declared build conditions.
    if ordered_runs
        .iter()
        .any(|run| run.input_fingerprint != baseline_fingerprint)
    {
        return Ok(DeterminismReport::new(
            DeterminismStatus::Inconclusive,
            compiler_status,
            ordered_runs.len(),
            baseline.artifact_count(),
            None,
            vec![DeterminismFinding::new(
                DeterminismFindingKind::InputIdentityMismatch,
            )],
        ));
    }

    let mut findings = Vec::new();

    for run in ordered_runs.iter().skip(1) {
        if run.artifacts.len() != baseline.artifacts.len() {
            reserve_one(
                &mut findings,
                "determinism findings",
            )?;
            findings.push(DeterminismFinding::new(
                DeterminismFindingKind::ArtifactSetMismatch,
            ));
            continue;
        }

        for (expected, actual) in
            baseline.artifacts.iter().zip(run.artifacts.iter())
        {
            match expected.key.cmp(&actual.key) {
                Ordering::Equal => {
                    if expected.content_fingerprint
                        != actual.content_fingerprint
                    {
                        reserve_one(
                            &mut findings,
                            "determinism findings",
                        )?;
                        findings.push(DeterminismFinding::new(
                            DeterminismFindingKind::ArtifactContentMismatch {
                                artifact: expected.key.clone(),
                            },
                        ));
                    }
                }
                Ordering::Less | Ordering::Greater => {
                    reserve_one(
                        &mut findings,
                        "determinism findings",
                    )?;
                    findings.push(DeterminismFinding::new(
                        DeterminismFindingKind::ArtifactSetMismatch,
                    ));
                }
            }
        }
    }

    if !findings.is_empty() {
        return Ok(DeterminismReport::new(
            DeterminismStatus::NonDeterministic,
            compiler_status,
            ordered_runs.len(),
            baseline.artifact_count(),
            Some(baseline_fingerprint),
            findings,
        ));
    }

    if compiler_status != CompilerDeterminismStatus::Satisfied {
        return Ok(DeterminismReport::new(
            DeterminismStatus::Inconclusive,
            compiler_status,
            ordered_runs.len(),
            baseline.artifact_count(),
            Some(baseline_fingerprint),
            vec![DeterminismFinding::new(
                DeterminismFindingKind::CompilerContractNotSatisfied {
                    status: compiler_status,
                },
            )],
        ));
    }

    Ok(DeterminismReport::new(
        DeterminismStatus::Deterministic,
        compiler_status,
        ordered_runs.len(),
        baseline.artifact_count(),
        Some(baseline_fingerprint),
        Vec::new(),
    ))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::concurrency::determinism::DeterminismPolicy;

    use crate::compiler::reproducibility::determinism::{
        ComponentId,
        DeterminismEvidence as CompilerEvidence,
        DeterminismObservation,
        EvidenceKind,
    };

    fn fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    fn artifact(
        name: &str,
        target: &str,
        format: &str,
        digest: u8,
    ) -> ArtifactObservation {
        ArtifactObservation::new(
            ArtifactComparisonKey::new(name, target, format)
                .expect("valid artifact key"),
            fingerprint(digest),
        )
    }

    fn compiler_assessment(
        satisfied: bool,
    ) -> CompilerDeterminismAssessment {
        let component = ComponentId::new("compiler.test")
            .expect("valid component identity");

        let observations = if satisfied {
            let evidence = CompilerEvidence::new(
                EvidenceKind::ConformanceTest,
                "tests/compiler/determinism",
            )
            .expect("valid compiler evidence");

            vec![
                DeterminismObservation::new(
                    component.clone(),
                    DeterminismPolicy::deterministic(),
                )
                .with_evidence(evidence),
            ]
        } else {
            Vec::new()
        };

        CompilerDeterminismAssessment::evaluate(
            DeterminismPolicy::deterministic(),
            [component],
            observations,
        )
        .expect("valid compiler assessment")
    }

    fn run(
        reference: &str,
        input: u8,
        artifact_digest: u8,
    ) -> DeterminismRun {
        DeterminismRun::new(
            reference,
            fingerprint(input),
            [artifact(
                "application",
                "target.example",
                "executable.example",
                artifact_digest,
            )],
        )
        .expect("valid build run")
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(DETERMINISM_TESTING_API_VERSION, 1);
    }

    #[test]
    fn artifact_keys_are_validated() {
        assert!(ArtifactComparisonKey::new(
            "",
            "target.example",
            "format.example",
        )
        .is_err());

        assert!(ArtifactComparisonKey::new(
            "application",
            " ",
            "format.example",
        )
        .is_err());
    }

    #[test]
    fn artifact_order_is_canonical() {
        let first = DeterminismRun::new(
            "run-a",
            fingerprint(1),
            [
                artifact("zeta", "target", "format", 2),
                artifact("alpha", "target", "format", 3),
            ],
        )
        .expect("valid run");

        assert_eq!(
            first.artifacts()[0].key().logical_name(),
            "alpha",
        );
        assert_eq!(
            first.artifacts()[1].key().logical_name(),
            "zeta",
        );
    }

    #[test]
    fn duplicate_artifact_keys_are_rejected() {
        let result = DeterminismRun::new(
            "run-a",
            fingerprint(1),
            [
                artifact("application", "target", "format", 2),
                artifact("application", "target", "format", 2),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_run_references_are_rejected() {
        let assessment = compiler_assessment(true);

        let result = evaluate(
            &assessment,
            [
                run("same-run", 1, 2),
                run("same-run", 1, 2),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn no_runs_are_inconclusive() {
        let assessment = compiler_assessment(true);
        let report = evaluate(&assessment, [])
            .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Inconclusive,
        );
        assert!(matches!(
            report.findings()[0].kind(),
            DeterminismFindingKind::NoRuns,
        ));
    }

    #[test]
    fn one_run_is_inconclusive() {
        let assessment = compiler_assessment(true);
        let report = evaluate(
            &assessment,
            [run("run-a", 1, 2)],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Inconclusive,
        );
    }

    #[test]
    fn different_inputs_are_incomparable() {
        let assessment = compiler_assessment(true);
        let report = evaluate(
            &assessment,
            [
                run("run-a", 1, 2),
                run("run-b", 3, 2),
            ],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Inconclusive,
        );
        assert!(matches!(
            report.findings()[0].kind(),
            DeterminismFindingKind::InputIdentityMismatch,
        ));
    }

    #[test]
    fn matching_inputs_and_artifacts_are_deterministic() {
        let assessment = compiler_assessment(true);
        let report = evaluate(
            &assessment,
            [
                run("run-b", 1, 2),
                run("run-a", 1, 2),
            ],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Deterministic,
        );
        assert!(report.is_deterministic());
        assert_eq!(report.run_count(), 2);
        assert_eq!(report.artifact_count(), 1);
        assert!(!report.has_findings());
    }

    #[test]
    fn differing_artifact_content_is_non_deterministic() {
        let assessment = compiler_assessment(true);
        let report = evaluate(
            &assessment,
            [
                run("run-a", 1, 2),
                run("run-b", 1, 9),
            ],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::NonDeterministic,
        );
        assert!(matches!(
            report.findings()[0].kind(),
            DeterminismFindingKind::ArtifactContentMismatch { .. },
        ));
    }

    #[test]
    fn differing_artifact_sets_are_non_deterministic() {
        let assessment = compiler_assessment(true);

        let first = DeterminismRun::new(
            "run-a",
            fingerprint(1),
            [artifact("application", "target", "format", 2)],
        )
        .expect("first run");

        let second = DeterminismRun::new(
            "run-b",
            fingerprint(1),
            [artifact("library", "target", "format", 2)],
        )
        .expect("second run");

        let report = evaluate(
            &assessment,
            [first, second],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::NonDeterministic,
        );
        assert!(report.findings().iter().any(|finding| {
            matches!(
                finding.kind(),
                DeterminismFindingKind::ArtifactSetMismatch,
            )
        }));
    }

    #[test]
    fn unsatisfied_compiler_contract_cannot_pass() {
        let assessment = compiler_assessment(false);
        let report = evaluate(
            &assessment,
            [
                run("run-a", 1, 2),
                run("run-b", 1, 2),
            ],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Inconclusive,
        );
        assert!(matches!(
            report.findings()[0].kind(),
            DeterminismFindingKind::CompilerContractNotSatisfied { .. },
        ));
    }

    #[test]
    fn empty_artifact_outputs_are_inconclusive() {
        let assessment = compiler_assessment(true);

        let first = DeterminismRun::new(
            "run-a",
            fingerprint(1),
            [],
        )
        .expect("first run");

        let second = DeterminismRun::new(
            "run-b",
            fingerprint(1),
            [],
        )
        .expect("second run");

        let report = evaluate(
            &assessment,
            [first, second],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Inconclusive,
        );
        assert!(matches!(
            report.findings()[0].kind(),
            DeterminismFindingKind::NoArtifacts,
        ));
    }

    #[test]
    fn artifact_iterator_order_does_not_change_comparison() {
        let assessment = compiler_assessment(true);

        let first = DeterminismRun::new(
            "run-a",
            fingerprint(1),
            [
                artifact("alpha", "target", "format", 4),
                artifact("beta", "target", "format", 5),
            ],
        )
        .expect("first run");

        let second = DeterminismRun::new(
            "run-b",
            fingerprint(1),
            [
                artifact("beta", "target", "format", 5),
                artifact("alpha", "target", "format", 4),
            ],
        )
        .expect("second run");

        let report = evaluate(
            &assessment,
            [second, first],
        )
        .expect("evaluation succeeds");

        assert_eq!(
            report.status(),
            DeterminismStatus::Deterministic,
        );
    }
}
