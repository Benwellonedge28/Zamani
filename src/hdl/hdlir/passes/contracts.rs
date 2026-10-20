
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Transformation Contracts
//!
//! Path: `src/hdl/hdlir/passes/contracts.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define declarative, versioned contracts for HDLIR passes. A contract
//! describes the conditions a pass requires, the conditions it must establish,
//! the analyses it consumes and invalidates, the properties it guarantees to
//! preserve, and its determinism and semantic-preservation obligations.
//!
//! This module does not implement transformations, inspect the IR graph,
//! execute analyses, prove equivalence, or certify physical realizability.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and applicable HDL delegate grammars
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`, when established
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/config.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! ## Ownership
//!
//! This module owns declarative pass-contract data types and validation of
//! their internal consistency. It does not own the pass manager, preservation
//! checker, IR verifier, analyses, transformations, source semantics, ZUIR,
//! target selection, or physical implementation.
//!
//! ## Scalability
//!
//! No hardware-size limits or fixed maximum number of conditions, analyses,
//! passes, modules, signals, widths, or preservation properties are imposed.
//! Collections grow with the supplied contract and available resources.
//! Callers that process untrusted or generated contracts must apply the
//! repository's explicit operational budgets at the consuming boundary.
//!
//! ## Semantic-safety rule
//!
//! A declared preservation guarantee is a promise the pass implementation
//! must honor, not evidence that it has honored it. The pass manager must
//! verify preconditions before execution, verify postconditions after
//! execution, and enforce preservation obligations using the applicable
//! verification or equivalence mechanism. Incomplete checks are not success.
//!
//! ## Integration
//!
//! The intended consumers are:
//!
//! - `passes/mod.rs`: public pass-contract exports.
//! - `passes/manager.rs`: contract evaluation and pass execution ordering.
//! - `passes/preservation.rs`: verification of preservation guarantees.
//! - `analysis/*`: providers for required analysis facts.
//! - `verify/*`: providers for structural and semantic checks.
//! - `transform/*`: pass declarations.
//!
//! These are planned integration points; their existence and compatibility
//! must be verified before the module is registered in the composition root.

use std::collections::BTreeSet;
use std::fmt;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};

// =============================================================================
// Qualified names
// =============================================================================

/// A stable, namespaced identifier for a pass, analysis, condition, or
/// preservation extension.
///
/// Names are lowercase ASCII and use dot-separated namespaces. The final
/// segment identifies the entity. Segments may contain ASCII lowercase
/// letters, digits, underscores, and hyphens.
///
/// Examples: `org.zamani.canonicalize`,
/// `org.zamani.analysis.clock_dependencies`.
///
/// Names are identifiers, not human-readable descriptions. Their meanings
/// must be documented and versioned by their owning extension.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContractKey(String);

impl ContractKey {
    /// Create a validated namespaced identifier.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR invalid-input error if the identifier is empty,
    /// unqualified, contains an empty segment, has surrounding whitespace,
    /// or contains characters outside the permitted ASCII alphabet.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        let valid = !value.is_empty()
            && value.trim() == value
            && value.contains('.')
            && !value.starts_with('.')
            && !value.ends_with('.')
            && value.split('.').all(|segment| {
                !segment.is_empty()
                    && segment.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'_' | b'-')
                    })
            });

        if !valid {
            return Err(HdlirError::invalid_input(
                "contract_key",
                format!(
                    "invalid qualified identifier `{value}`; expected \
                     lowercase ASCII dot-separated segments"
                ),
            ));
        }

        Ok(Self(value))
    }

    /// Return the stable identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContractKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Pass identity and versioning
// =============================================================================

/// A version of a pass contract.
///
/// Contract versions are distinct from the Zamani language version, crate
/// version, HDL API version, HDLIR artifact version, and dialect versions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PassVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl PassVersion {
    /// Construct a pass-contract version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Return the patch version.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for PassVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Stable identity of a registered transformation pass.
///
/// Two implementations with the same key and version must satisfy the same
/// published contract. A semantic contract change requires an appropriate
/// version change under the repository's compatibility policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassIdentity {
    key: ContractKey,
    version: PassVersion,
}

impl PassIdentity {
    /// Construct a pass identity.
    #[must_use]
    pub const fn new(key: ContractKey, version: PassVersion) -> Self {
        Self { key, version }
    }

    /// Return the namespaced pass key.
    #[must_use]
    pub const fn key(&self) -> &ContractKey {
        &self.key
    }

    /// Return the contract version.
    #[must_use]
    pub const fn version(&self) -> PassVersion {
        self.version
    }
}

// =============================================================================
// Conditions and analyses
// =============================================================================

/// States whether a condition must hold or is advisory.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionStrength {
    /// Failure prevents the pass from being considered contract-compliant.
    Required,

    /// Failure is reported to the caller but does not, by itself, block
    /// execution. Advisory conditions must never override a required
    /// condition or a mandatory verification invariant.
    Advisory,
}

/// A named condition a pass requires before execution or establishes
/// after execution.
///
/// The evaluator that owns the condition's namespace supplies its meaning.
/// Descriptions are for humans and diagnostics; they are not executable
/// predicates and must not be used as machine-readable identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractCondition {
    key: ContractKey,
    strength: ConditionStrength,
    description: String,
}

impl ContractCondition {
    /// Construct a condition declaration.
    #[must_use]
    pub fn new(
        key: ContractKey,
        strength: ConditionStrength,
        description: impl Into<String>,
    ) -> Self {
        Self {
            key,
            strength,
            description: description.into(),
        }
    }

    /// Return the stable condition identifier.
    #[must_use]
    pub const fn key(&self) -> &ContractKey {
        &self.key
    }

    /// Return the condition's enforcement strength.
    #[must_use]
    pub const fn strength(&self) -> ConditionStrength {
        self.strength
    }

    /// Return the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Declares whether a required analysis must be available and current.
///
/// `Current` means that the provider has established validity for the exact
/// IR revision being examined. A result from an earlier revision is not
/// current merely because its analysis key matches.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AnalysisFreshness {
    /// A valid analysis result for the current IR revision is required.
    Current,

    /// A provider may recompute the result before the pass executes.
    RecomputeIfStale,
}

/// A named analysis dependency declared by a pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisRequirement {
    key: ContractKey,
    freshness: AnalysisFreshness,
}

impl AnalysisRequirement {
    /// Construct a required analysis declaration.
    #[must_use]
    pub const fn new(
        key: ContractKey,
        freshness: AnalysisFreshness,
    ) -> Self {
        Self { key, freshness }
    }

    /// Return the analysis identifier.
    #[must_use]
    pub const fn key(&self) -> &ContractKey {
        &self.key
    }

    /// Return the required freshness behavior.
    #[must_use]
    pub const fn freshness(&self) -> AnalysisFreshness {
        self.freshness
    }
}

// =============================================================================
// Preservation guarantees
// =============================================================================

/// A semantic or representational property a pass promises to preserve.
///
/// The enum provides stable core properties while `Extension` allows
/// registered dialects to declare additional properties without requiring
/// every new hardware technology to modify this module.
///
/// These declarations do not themselves prove preservation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PreservationGuarantee {
    /// Preserve observable combinational behavior.
    CombinationalBehavior,

    /// Preserve sequential state-transition behavior.
    StateTransitions,

    /// Preserve clock and reset meaning, including relevant edge and polarity
    /// semantics represented by the input IR.
    ClockAndResetSemantics,

    /// Preserve memory access, ordering, and declared collision behavior.
    MemorySemantics,

    /// Preserve externally observable effects and side-effect ordering.
    ObservableEffects,

    /// Preserve required logical connectivity and externally visible ports.
    ConnectivityAndPorts,

    /// Preserve type meaning, signedness, and exact or symbolic width rules.
    TypesAndWidths,

    /// Preserve declared requirements, constraints, and contracts.
    RequirementsAndConstraints,

    /// Preserve source provenance to the extent promised by the provenance
    /// contract, including multi-origin information when present.
    Provenance,

    /// Preserve the declared numerical and ordering semantics.
    NumericalAndOrderingSemantics,

    /// Preserve concurrency, synchronization, and event-ordering semantics.
    ConcurrencyAndEvents,

    /// Preserve an extension-defined property.
    Extension(ContractKey),
}

/// Describes the semantic obligation a pass claims to satisfy.
///
/// Ordinary optimizations and canonicalization passes should normally use
/// `PreservesObservableBehavior`. A pass that refines an implementation
/// under a proof obligation must name that obligation explicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SemanticGuarantee {
    /// Observable behavior and required source-level contracts are preserved.
    PreservesObservableBehavior,

    /// A refinement is permitted only if the named proof obligation is
    /// discharged by an appropriate verifier before the result is trusted.
    RefinesUnderProof(ContractKey),
}

/// Describes the determinism promised by a pass.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeterminismGuarantee {
    /// The pass must produce the same canonical result for the same IR,
    /// schema, contract version, and relevant configuration.
    Deterministic,

    /// The pass may vary incidental representation details but must preserve
    /// all declared semantic guarantees. Consumers must not use its output
    /// directly as a deterministic fingerprint input without canonicalizing.
    SemanticOnly,
}

// =============================================================================
// Contract declaration
// =============================================================================

/// Declarative contract for an HDLIR transformation.
///
/// A contract is immutable after construction. Its ordered condition lists
/// preserve declaration order for diagnostics; set-like dependencies and
/// preservation guarantees have deterministic ordering.
///
/// The contract does not contain executable closures, pointers, global state,
/// target handles, or references to a particular IR allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassContract {
    identity: PassIdentity,
    preconditions: Vec<ContractCondition>,
    postconditions: Vec<ContractCondition>,
    required_analyses: Vec<AnalysisRequirement>,
    invalidated_analyses: BTreeSet<ContractKey>,
    preservation: BTreeSet<PreservationGuarantee>,
    semantic_guarantee: SemanticGuarantee,
    determinism: DeterminismGuarantee,
}

impl PassContract {
    /// Create a contract builder for a pass.
    #[must_use]
    pub fn builder(identity: PassIdentity) -> PassContractBuilder {
        PassContractBuilder::new(identity)
    }

    /// Return the pass identity.
    #[must_use]
    pub const fn identity(&self) -> &PassIdentity {
        &self.identity
    }

    /// Return preconditions in declaration order.
    #[must_use]
    pub fn preconditions(&self) -> &[ContractCondition] {
        &self.preconditions
    }

    /// Return postconditions in declaration order.
    #[must_use]
    pub fn postconditions(&self) -> &[ContractCondition] {
        &self.postconditions
    }

    /// Return required analyses in declaration order.
    #[must_use]
    pub fn required_analyses(&self) -> &[AnalysisRequirement] {
        &self.required_analyses
    }

    /// Return analyses that become invalid after this pass changes the IR.
    #[must_use]
    pub fn invalidated_analyses(&self) -> &BTreeSet<ContractKey> {
        &self.invalidated_analyses
    }

    /// Return guaranteed preservation properties.
    #[must_use]
    pub fn preservation(&self) -> &BTreeSet<PreservationGuarantee> {
        &self.preservation
    }

    /// Return the semantic-preservation obligation.
    #[must_use]
    pub const fn semantic_guarantee(&self) -> &SemanticGuarantee {
        &self.semantic_guarantee
    }

    /// Return the determinism promise.
    #[must_use]
    pub const fn determinism(&self) -> DeterminismGuarantee {
        self.determinism
    }

    /// Validate declaration consistency before registry insertion.
    ///
    /// This validates the contract metadata only. It does not inspect an IR,
    /// execute any condition, check whether an analysis provider exists, or
    /// prove a preservation claim.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate condition declarations or duplicate
    /// analysis requirements. Repeating a preservation guarantee is already
    /// prevented by its set representation.
    pub fn validate_declaration(&self) -> HdlirResult<()> {
        validate_unique_conditions("preconditions", &self.preconditions)?;
        validate_unique_conditions("postconditions", &self.postconditions)?;
        validate_unique_analyses(&self.required_analyses)?;

        Ok(())
    }
}

/// Builder for constructing and validating a [`PassContract`].
///
/// The builder is not tied to a specific transformation implementation.
#[derive(Clone, Debug)]
pub struct PassContractBuilder {
    identity: PassIdentity,
    preconditions: Vec<ContractCondition>,
    postconditions: Vec<ContractCondition>,
    required_analyses: Vec<AnalysisRequirement>,
    invalidated_analyses: BTreeSet<ContractKey>,
    preservation: BTreeSet<PreservationGuarantee>,
    semantic_guarantee: SemanticGuarantee,
    determinism: DeterminismGuarantee,
}

impl PassContractBuilder {
    /// Start a contract declaration for the supplied pass identity.
    #[must_use]
    pub fn new(identity: PassIdentity) -> Self {
        Self {
            identity,
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            required_analyses: Vec::new(),
            invalidated_analyses: BTreeSet::new(),
            preservation: BTreeSet::new(),
            semantic_guarantee: SemanticGuarantee::PreservesObservableBehavior,
            determinism: DeterminismGuarantee::Deterministic,
        }
    }

    /// Declare a precondition.
    #[must_use]
    pub fn precondition(mut self, condition: ContractCondition) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Declare a postcondition.
    #[must_use]
    pub fn postcondition(mut self, condition: ContractCondition) -> Self {
        self.postconditions.push(condition);
        self
    }

    /// Declare an analysis required before execution.
    #[must_use]
    pub fn requires_analysis(
        mut self,
        requirement: AnalysisRequirement,
    ) -> Self {
        self.required_analyses.push(requirement);
        self
    }

    /// Declare an analysis invalidated by this pass.
    #[must_use]
    pub fn invalidates_analysis(mut self, key: ContractKey) -> Self {
        self.invalidated_analyses.insert(key);
        self
    }

    /// Declare a preservation guarantee.
    #[must_use]
    pub fn preserves(mut self, guarantee: PreservationGuarantee) -> Self {
        self.preservation.insert(guarantee);
        self
    }

    /// Declare the semantic-preservation obligation.
    #[must_use]
    pub fn semantic_guarantee(
        mut self,
        guarantee: SemanticGuarantee,
    ) -> Self {
        self.semantic_guarantee = guarantee;
        self
    }

    /// Declare the determinism promise.
    #[must_use]
    pub fn determinism(
        mut self,
        guarantee: DeterminismGuarantee,
    ) -> Self {
        self.determinism = guarantee;
        self
    }

    /// Finish and validate the contract declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if any declaration is internally inconsistent.
    pub fn build(self) -> HdlirResult<PassContract> {
        let contract = PassContract {
            identity: self.identity,
            preconditions: self.preconditions,
            postconditions: self.postconditions,
            required_analyses: self.required_analyses,
            invalidated_analyses: self.invalidated_analyses,
            preservation: self.preservation,
            semantic_guarantee: self.semantic_guarantee,
            determinism: self.determinism,
        };

        contract.validate_declaration()?;
        Ok(contract)
    }
}

// =============================================================================
// Contract evaluation boundary
// =============================================================================

/// The outcome of evaluating one declared condition.
///
/// `satisfied == false` for an advisory condition is reportable but does not
/// independently block execution. A required failure must block execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConditionEvaluation {
    /// Condition that was evaluated.
    pub key: ContractKey,

    /// Whether the condition is satisfied.
    pub satisfied: bool,

    /// Optional explanation for the result.
    pub detail: Option<String>,
}

/// A report of evaluated conditions in declaration order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContractEvaluationReport {
    /// Completed condition evaluations.
    pub conditions: Vec<ConditionEvaluation>,
}

impl ContractEvaluationReport {
    /// Return whether every required condition in the report passed.
    ///
    /// This is a report-local query. Callers must also establish that all
    /// required checks were actually performed before trusting the report.
    #[must_use]
    pub fn required_conditions_satisfied(
        &self,
        declared: &[ContractCondition],
    ) -> bool {
        declared.iter().all(|condition| {
            if condition.strength() == ConditionStrength::Advisory {
                return true;
            }

            self.conditions.iter().any(|result| {
                result.key == *condition.key() && result.satisfied
            })
        })
    }
}

/// Supplies executable meaning for contract conditions and analysis keys.
///
/// Implementations belong to the pass manager and analysis/verification
/// integration. They must return an error if evaluation is incomplete or
/// cannot safely establish the requested fact. They must not report an
/// unevaluated condition as satisfied.
pub trait ContractEvaluator {
    /// Evaluate one named condition against the specified IR revision.
    fn evaluate_condition(
        &mut self,
        condition: &ContractCondition,
        after_pass: bool,
    ) -> HdlirResult<ConditionEvaluation>;

    /// Establish whether a required analysis is current or can be recomputed.
    ///
    /// `Ok(true)` means the analysis is ready for the exact IR revision.
    /// `Ok(false)` means it is not ready. An inability to finish evaluation
    /// must be returned as an error, not `Ok(true)`.
    fn ensure_analysis(
        &mut self,
        requirement: &AnalysisRequirement,
    ) -> HdlirResult<bool>;
}

/// Evaluate all declared preconditions and required analyses.
///
/// Required conditions and analyses must be satisfied before the pass runs.
/// Advisory condition failures are retained in the report. A failure of a
/// required condition or analysis returns an error and blocks execution.
///
/// # Errors
///
/// Returns an HDLIR error when a required condition is false, an analysis is
/// unavailable, or evaluation cannot complete.
pub fn evaluate_before(
    contract: &PassContract,
    evaluator: &mut impl ContractEvaluator,
) -> HdlirResult<ContractEvaluationReport> {
    let mut report = ContractEvaluationReport::default();

    for condition in contract.preconditions() {
        let result = evaluator.evaluate_condition(condition, false)?;

        if result.key != *condition.key() {
            return Err(HdlirError::invalid_invariant(
                "condition_evaluator_identity",
                format!(
                    "pass `{}` declared condition `{}`, but evaluator \
                     returned `{}`",
                    contract.identity().key(),
                    condition.key(),
                    result.key
                ),
            ));
        }

        if !result.satisfied
            && condition.strength() == ConditionStrength::Required
        {
            return Err(HdlirError::invalid_invariant(
                "pass_precondition",
                format!(
                    "pass `{}` requires `{}`: {}",
                    contract.identity().key(),
                    condition.key(),
                    result.detail.as_deref().unwrap_or(
                        "required precondition was not satisfied"
                    )
                ),
            ));
        }

        report.conditions.push(result);
    }

    for requirement in contract.required_analyses() {
        if !evaluator.ensure_analysis(requirement)? {
            return Err(HdlirError::invalid_invariant(
                "required_analysis",
                format!(
                    "pass `{}` requires unavailable analysis `{}`",
                    contract.identity().key(),
                    requirement.key()
                ),
            ));
        }
    }

    Ok(report)
}

/// Evaluate all declared postconditions after the pass has completed.
///
/// The caller must invoke this only after a pass has reported successful
/// execution. It must not use postcondition evaluation to convert a failed
/// or incomplete transformation into success.
///
/// # Errors
///
/// Returns an HDLIR error if a required postcondition fails or evaluation
/// cannot complete.
pub fn evaluate_after(
    contract: &PassContract,
    evaluator: &mut impl ContractEvaluator,
) -> HdlirResult<ContractEvaluationReport> {
    let mut report = ContractEvaluationReport::default();

    for condition in contract.postconditions() {
        let result = evaluator.evaluate_condition(condition, true)?;

        if result.key != *condition.key() {
            return Err(HdlirError::invalid_invariant(
                "condition_evaluator_identity",
                format!(
                    "pass `{}` declared postcondition `{}`, but evaluator \
                     returned `{}`",
                    contract.identity().key(),
                    condition.key(),
                    result.key
                ),
            ));
        }

        if !result.satisfied
            && condition.strength() == ConditionStrength::Required
        {
            return Err(HdlirError::TransformationFailed {
                pass: contract.identity().key().to_string(),
                detail: format!(
                    "required postcondition `{}` failed: {}",
                    condition.key(),
                    result.detail.as_deref().unwrap_or(
                        "required postcondition was not satisfied"
                    )
                ),
            });
        }

        report.conditions.push(result);
    }

    Ok(report)
}

// =============================================================================
// Internal declaration validation
// =============================================================================

fn validate_unique_conditions(
    field: &'static str,
    conditions: &[ContractCondition],
) -> HdlirResult<()> {
    let mut seen = BTreeSet::new();

    for condition in conditions {
        if !seen.insert(condition.key()) {
            return Err(HdlirError::invalid_input(
                field,
                format!("duplicate condition `{}`", condition.key()),
            ));
        }
    }

    Ok(())
}

fn validate_unique_analyses(
    requirements: &[AnalysisRequirement],
) -> HdlirResult<()> {
    let mut seen = BTreeSet::new();

    for requirement in requirements {
        if !seen.insert(requirement.key()) {
            return Err(HdlirError::invalid_input(
                "required_analyses",
                format!(
                    "duplicate analysis requirement `{}`",
                    requirement.key()
                ),
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> ContractKey {
        ContractKey::new(value).expect("test key must be valid")
    }

    fn identity() -> PassIdentity {
        PassIdentity::new(
            key("org.zamani.test_pass"),
            PassVersion::new(1, 0, 0),
        )
    }

    struct TestEvaluator {
        conditions_satisfied: bool,
        analyses_available: bool,
    }

    impl ContractEvaluator for TestEvaluator {
        fn evaluate_condition(
            &mut self,
            condition: &ContractCondition,
            _after_pass: bool,
        ) -> HdlirResult<ConditionEvaluation> {
            Ok(ConditionEvaluation {
                key: condition.key().clone(),
                satisfied: self.conditions_satisfied,
                detail: None,
            })
        }

        fn ensure_analysis(
            &mut self,
            _requirement: &AnalysisRequirement,
        ) -> HdlirResult<bool> {
            Ok(self.analyses_available)
        }
    }

    #[test]
    fn rejects_unqualified_or_invalid_keys() {
        assert!(ContractKey::new("").is_err());
        assert!(ContractKey::new("canonicalize").is_err());
        assert!(ContractKey::new(".org.pass").is_err());
        assert!(ContractKey::new("org..pass").is_err());
        assert!(ContractKey::new("org.Pass").is_err());
        assert!(ContractKey::new("org.pass ").is_err());
        assert!(ContractKey::new("org.pass-name").is_ok());
        assert!(ContractKey::new("org.pass_name").is_ok());
    }

    #[test]
    fn contract_builder_preserves_declared_contract() {
        let precondition = ContractCondition::new(
            key("org.zamani.condition.input_verified"),
            ConditionStrength::Required,
            "Input IR has passed required verification",
        );

        let contract = PassContract::builder(identity())
            .precondition(precondition)
            .requires_analysis(AnalysisRequirement::new(
                key("org.zamani.analysis.dependencies"),
                AnalysisFreshness::RecomputeIfStale,
            ))
            .invalidates_analysis(key("org.zamani.analysis.dependencies"))
            .preserves(PreservationGuarantee::StateTransitions)
            .preserves(PreservationGuarantee::ClockAndResetSemantics)
            .build()
            .expect("contract must be valid");

        assert_eq!(contract.preconditions().len(), 1);
        assert_eq!(contract.required_analyses().len(), 1);
        assert_eq!(contract.invalidated_analyses().len(), 1);
        assert!(
            contract
                .preservation()
                .contains(&PreservationGuarantee::StateTransitions)
        );
        assert_eq!(
            contract.semantic_guarantee(),
            &SemanticGuarantee::PreservesObservableBehavior
        );
    }

    #[test]
    fn duplicate_preconditions_are_rejected() {
        let condition = || {
            ContractCondition::new(
                key("org.zamani.condition.verified"),
                ConditionStrength::Required,
                "Verified",
            )
        };

        let result = PassContract::builder(identity())
            .precondition(condition())
            .precondition(condition())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_analysis_requirements_are_rejected() {
        let requirement = || {
            AnalysisRequirement::new(
                key("org.zamani.analysis.widths"),
                AnalysisFreshness::Current,
            )
        };

        let result = PassContract::builder(identity())
            .requires_analysis(requirement())
            .requires_analysis(requirement())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn required_precondition_failure_blocks_execution() {
        let contract = PassContract::builder(identity())
            .precondition(ContractCondition::new(
                key("org.zamani.condition.verified"),
                ConditionStrength::Required,
                "Input must be verified",
            ))
            .build()
            .expect("contract must be valid");

        let mut evaluator = TestEvaluator {
            conditions_satisfied: false,
            analyses_available: true,
        };

        assert!(evaluate_before(&contract, &mut evaluator).is_err());
    }

    #[test]
    fn advisory_precondition_failure_is_reported_without_blocking() {
        let contract = PassContract::builder(identity())
            .precondition(ContractCondition::new(
                key("org.zamani.condition.profitable"),
                ConditionStrength::Advisory,
                "Optimization is expected to be profitable",
            ))
            .build()
            .expect("contract must be valid");

        let mut evaluator = TestEvaluator {
            conditions_satisfied: false,
            analyses_available: true,
        };

        let report = evaluate_before(&contract, &mut evaluator)
            .expect("advisory failure must not block execution");

        assert_eq!(report.conditions.len(), 1);
        assert!(!report.conditions[0].satisfied);
    }

    #[test]
    fn unavailable_analysis_blocks_execution() {
        let contract = PassContract::builder(identity())
            .requires_analysis(AnalysisRequirement::new(
                key("org.zamani.analysis.clocks"),
                AnalysisFreshness::Current,
            ))
            .build()
            .expect("contract must be valid");

        let mut evaluator = TestEvaluator {
            conditions_satisfied: true,
            analyses_available: false,
        };

        assert!(evaluate_before(&contract, &mut evaluator).is_err());
    }

    #[test]
    fn required_postcondition_failure_is_transformation_failure() {
        let contract = PassContract::builder(identity())
            .postcondition(ContractCondition::new(
                key("org.zamani.condition.valid_output"),
                ConditionStrength::Required,
                "Output must be valid",
            ))
            .build()
            .expect("contract must be valid");

        let mut evaluator = TestEvaluator {
            conditions_satisfied: false,
            analyses_available: true,
        };

        let result = evaluate_after(&contract, &mut evaluator);

        assert!(matches!(
            result,
            Err(HdlirError::TransformationFailed { .. })
        ));
    }

    #[test]
    fn pass_version_ordering_is_explicit() {
        let v1 = PassVersion::new(1, 0, 0);
        let v2 = PassVersion::new(2, 0, 0);

        assert!(v1 < v2);
        assert_eq!(v1.to_string(), "1.0.0");
    }

    #[test]
    fn preservation_guarantees_are_deterministically_ordered() {
        let contract = PassContract::builder(identity())
            .preserves(PreservationGuarantee::StateTransitions)
            .preserves(PreservationGuarantee::CombinationalBehavior)
            .preserves(PreservationGuarantee::StateTransitions)
            .build()
            .expect("contract must be valid");

        assert_eq!(contract.preservation().len(), 2);
    }
}
