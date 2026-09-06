//! Zamani Quantum Resilience — Custom Mitigation Strategy
//!
//! Path:
//!     src/quantum/resilience/mitigation/custom.rs
//!
//! Purpose:
//!     Provides a production-grade, backend-independent extension mechanism
//!     for application-, research-, domain-, and future-specific quantum
//!     error-mitigation strategies.
//!
//! Architectural boundary:
//!
//! ```text
//!                         Canonical Zamani IR
//!                                |
//!                                v
//!                     Resilience / Mitigation
//!                                |
//!                                v
//!                    CustomMitigationStrategy
//!                                |
//!                 +--------------+--------------+
//!                 |                             |
//!                 v                             v
//!          applicability                 declarative plan
//!                 |                             |
//!                 v                             v
//!          MitigationSelection          MitigationExecutor
//!                                               |
//!                                               v
//!                                         Hardware / Runtime
//!                                               |
//!                                               v
//!                                          Verification
//! ```
//!
//! This module deliberately does NOT:
//!
//! - execute quantum programs;
//! - access a backend;
//! - access provider credentials;
//! - perform network or filesystem I/O;
//! - perform routing;
//! - perform scheduling;
//! - mutate canonical IR;
//! - implement QEC;
//! - implement a particular mitigation algorithm;
//! - contain retry loops;
//! - contain fixed qubit counts;
//! - contain fixed circuit-size limits;
//! - contain provider-specific branches;
//! - silently authorize custom strategies;
//! - silently mutate execution state.
//!
//! Actual execution belongs to `mitigation/executor.rs`.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `mitigation/strategy.rs`
//!     Supplies the stable `MitigationStrategy` contract and common strategy
//!     domain types.
//!
//! `mitigation/selection.rs`
//!     Determines whether this strategy is eligible under the current
//!     workload, capabilities, policy and resource constraints.
//!
//! `mitigation/executor.rs`
//!     Interprets the declarative `CustomMitigationPlan` and performs the
//!     target-specific execution through the normal execution stack.
//!
//! `mitigation/zero_noise.rs`
//! `mitigation/probabilistic.rs`
//! `mitigation/twirling.rs`
//! `mitigation/dynamical_decoupling.rs`
//!     May be composed with a custom strategy only when the planner explicitly
//!     permits composition.
//!
//! `registry/strategy.rs`
//!     Registers this strategy exactly like every other mitigation strategy.
//!
//! `planning/*`
//!     Treats the returned plan as a candidate mitigation action.
//!
//! `verification/*`
//!     Remains authoritative for semantic acceptance.
//!
//! `telemetry/*`
//!     Records strategy identity, version, configuration identity and outcome.
//!
//! `history/*`
//!     Records verified outcomes for future selection/learning.
//!
//! `serialization/*`
//!     Can serialize the immutable descriptor, parameters and plan.
//!
//! `quantum::ir::qubit::QubitId`
//!     Remains authoritative for logical qubit identity.
//!
//! `quantum::hardware`
//!     Remains authoritative for target capabilities.
//!
//! `quantum::zqn`
//!     Remains authoritative for quantum fault/noise semantics.
//!
//! -----------------------------------------------------------------------------
//! Scalability
//! -----------------------------------------------------------------------------
//!
//! There is intentionally no:
//!
//! - maximum qubit count;
//! - maximum custom strategy count;
//! - maximum parameter count;
//! - maximum circuit size;
//! - maximum shot count;
//! - maximum backend count;
//! - provider-specific resource identifier;
//! - fixed machine topology.
//!
//! Resource limits belong to the policy/planning/execution layers.
//!
//! Collection sizes are determined by caller-provided resources.
//!
//! -----------------------------------------------------------------------------
//! Security
//! -----------------------------------------------------------------------------
//!
//! A custom mitigation strategy is data and a contract, not executable code.
//!
//! This is intentional. Arbitrary custom executable code must not be injected
//! into the resilience core merely because it implements a mitigation API.
//!
//! If executable extensions are eventually supported, they should be isolated
//! behind a separately authenticated/sandboxed plugin boundary.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! The strategy descriptor and plan are immutable.
//!
//! Parameter ordering is canonicalized during construction.
//! Duplicate parameter names are rejected.
//!
//! No randomness is generated here.
//!
//! Randomized custom strategies must receive randomness/provenance through the
//! executor/runtime contract and must record it there.
//!
//! -----------------------------------------------------------------------------
//! Rust requirements
//! -----------------------------------------------------------------------------
//!
//! Rust 1.97 / 1.97.1
//! Rust 2021
//! No unsafe.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use super::strategy::{
    Applicability, ExpectedOverhead, MitigationScope, MitigationStrategy,
    OverheadDimension, OverheadLevel, StrategyContext, StrategyDescriptor,
    StrategyEvaluation, StrategyFamily, StrategyId, StrategyPhase,
    StrategyRequirement, StrategyVersion,
};

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable identifier for the custom mitigation subsystem.
pub const CUSTOM_MITIGATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.custom";

/// Schema version.
///
/// Increment only when the externally observable custom-strategy contract
/// changes incompatibly.
pub const CUSTOM_MITIGATION_SCHEMA_VERSION: u16 = 1;

/// Default semantic version for a newly authored custom strategy.
///
/// The strategy author may supply another version through
/// `CustomMitigationDefinition::new`.
pub const CUSTOM_STRATEGY_VERSION: StrategyVersion =
    StrategyVersion::new(1, 0, 0);

// =============================================================================
// Error type
// =============================================================================

/// Errors produced while constructing or validating a custom mitigation
/// strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomMitigationError {
    /// The custom strategy identifier is invalid.
    InvalidStrategyId,

    /// A parameter name is empty.
    EmptyParameterName,

    /// A parameter name contains whitespace.
    InvalidParameterName,

    /// A parameter name appears more than once.
    DuplicateParameterName,

    /// The custom strategy contains no declared semantic contract.
    EmptyDescription,

    /// A required field is inconsistent with the supplied strategy family.
    InvalidDescriptor(&'static str),

    /// A floating-point custom parameter is not finite.
    NonFiniteFloatParameter,

    /// The logical-qubit scope was explicitly supplied but is empty.
    EmptyLogicalQubitScope,
}

impl fmt::Display for CustomMitigationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategyId => {
                formatter.write_str("invalid custom mitigation strategy identifier")
            }
            Self::EmptyParameterName => {
                formatter.write_str("custom mitigation parameter name is empty")
            }
            Self::InvalidParameterName => {
                formatter.write_str(
                    "custom mitigation parameter name contains whitespace",
                )
            }
            Self::DuplicateParameterName => {
                formatter.write_str(
                    "custom mitigation parameter name occurs more than once",
                )
            }
            Self::EmptyDescription => {
                formatter.write_str("custom mitigation description is empty")
            }
            Self::InvalidDescriptor(message) => {
                write!(formatter, "invalid custom mitigation descriptor: {message}")
            }
            Self::NonFiniteFloatParameter => {
                formatter.write_str(
                    "custom mitigation floating-point parameter must be finite",
                )
            }
            Self::EmptyLogicalQubitScope => {
                formatter.write_str(
                    "custom mitigation logical-qubit scope cannot be empty",
                )
            }
        }
    }
}

impl std::error::Error for CustomMitigationError {}

/// Result type used by this module.
pub type CustomMitigationResult<T> = Result<T, CustomMitigationError>;

// =============================================================================
// Custom parameter values
// =============================================================================

/// Provider-independent value that can be supplied to a custom mitigation
/// strategy.
///
/// The custom strategy layer intentionally does not use `serde_json::Value` or
/// another provider/application-specific dynamic object. This keeps the
/// domain contract deterministic and serialization-neutral.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomParameterValue {
    /// Boolean value.
    Boolean(bool),

    /// Signed integer.
    Signed(i128),

    /// Unsigned integer.
    Unsigned(u128),

    /// Finite floating-point value represented by its IEEE-754 bit pattern.
    ///
    /// Storing bits rather than relying on textual formatting preserves exact
    /// identity for deterministic hashing/provenance.
    FloatBits(u64),

    /// UTF-8 textual value.
    Text(Arc<str>),

    /// Opaque binary value.
    Bytes(Arc<[u8]>),
}

impl CustomParameterValue {
    /// Creates a finite floating-point parameter.
    pub fn float(value: f64) -> CustomMitigationResult<Self> {
        if !value.is_finite() {
            return Err(CustomMitigationError::NonFiniteFloatParameter);
        }

        Ok(Self::FloatBits(value.to_bits()))
    }

    /// Reconstructs the floating-point value.
    ///
    /// The constructor guarantees that normal values are finite. The bit
    /// representation is retained exactly for deterministic identity.
    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::FloatBits(bits) => Some(f64::from_bits(*bits)),
            _ => None,
        }
    }

    /// Creates a textual parameter.
    pub fn text(value: impl Into<Arc<str>>) -> Self {
        Self::Text(value.into())
    }

    /// Creates a binary parameter.
    pub fn bytes(value: impl Into<Arc<[u8]>>) -> Self {
        Self::Bytes(value.into())
    }
}

// =============================================================================
// Custom parameter
// =============================================================================

/// Named immutable parameter supplied to a custom mitigation strategy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomParameter {
    name: Arc<str>,
    value: CustomParameterValue,
}

impl CustomParameter {
    /// Creates a validated named parameter.
    pub fn new(
        name: impl Into<Arc<str>>,
        value: CustomParameterValue,
    ) -> CustomMitigationResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(CustomMitigationError::EmptyParameterName);
        }

        if name.chars().any(char::is_whitespace) {
            return Err(CustomMitigationError::InvalidParameterName);
        }

        Ok(Self { name, value })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parameter value.
    #[must_use]
    pub fn value(&self) -> &CustomParameterValue {
        &self.value
    }
}

// =============================================================================
// Custom parameter collection
// =============================================================================

/// Immutable, deterministically ordered custom parameter collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomParameters {
    values: Arc<[CustomParameter]>,
}

impl CustomParameters {
    /// Creates a validated parameter collection.
    ///
    /// Parameter names are sorted lexicographically and duplicate names are
    /// rejected. This makes equivalent configurations have the same canonical
    /// ordering regardless of insertion order.
    pub fn new<I>(parameters: I) -> CustomMitigationResult<Self>
    where
        I: IntoIterator<Item = CustomParameter>,
    {
        let mut values: Vec<CustomParameter> = parameters.into_iter().collect();

        values.sort_by(|left, right| left.name().cmp(right.name()));

        for pair in values.windows(2) {
            if pair[0].name() == pair[1].name() {
                return Err(CustomMitigationError::DuplicateParameterName);
            }
        }

        Ok(Self {
            values: values.into(),
        })
    }

    /// Creates an empty parameter collection.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            values: Arc::from([]),
        }
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the parameters.
    #[must_use]
    pub fn as_slice(&self) -> &[CustomParameter] {
        &self.values
    }

    /// Looks up a parameter by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&CustomParameter> {
        self.values
            .binary_search_by(|parameter| parameter.name().cmp(name))
            .ok()
            .and_then(|index| self.values.get(index))
    }
}

impl Default for CustomParameters {
    fn default() -> Self {
        Self::empty()
    }
}

// =============================================================================
// Semantic guarantee
// =============================================================================

/// Declares the semantic contract of a custom mitigation strategy.
///
/// This does not prove correctness. It declares what the strategy claims, so
/// verification can apply the appropriate acceptance rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticGuarantee {
    /// Strategy claims exact preservation of logical program semantics.
    Exact,

    /// Strategy may alter only physically/global-phase-equivalent behavior
    /// according to the verification contract.
    EquivalentUpToDeclaredPhase,

    /// Strategy produces an estimate rather than a directly equivalent
    /// execution result.
    EstimateOnly,

    /// Strategy semantics are supplied externally and therefore require
    /// explicit verification before acceptance.
    RequiresVerification,
}

impl SemanticGuarantee {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::EquivalentUpToDeclaredPhase => "equivalent_up_to_declared_phase",
            Self::EstimateOnly => "estimate_only",
            Self::RequiresVerification => "requires_verification",
        }
    }
}

// =============================================================================
// Custom strategy definition
// =============================================================================

/// Immutable definition of a custom mitigation strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomMitigationDefinition {
    descriptor: StrategyDescriptor,
    semantic_guarantee: SemanticGuarantee,
}

impl CustomMitigationDefinition {
    /// Creates a custom mitigation definition.
    ///
    /// `family` is normalized to `StrategyFamily::Custom`; custom strategies
    /// must never impersonate one of Zamani's built-in mitigation families.
    pub fn new(
        id: impl Into<String>,
        version: StrategyVersion,
        phase: StrategyPhase,
        description: impl Into<Arc<str>>,
        requirements: impl Into<Arc<[StrategyRequirement]>>,
        expected_overhead: impl Into<Arc<[ExpectedOverhead]>>,
        deterministic: bool,
        requires_explicit_authorization: bool,
        semantic_guarantee: SemanticGuarantee,
    ) -> CustomMitigationResult<Self> {
        let id = StrategyId::new(id.into())
            .map_err(|_| CustomMitigationError::InvalidStrategyId)?;

        let description = description.into();

        if description.is_empty() {
            return Err(CustomMitigationError::EmptyDescription);
        }

        let requirements = requirements.into();

        let requires_authorization =
            requires_explicit_authorization
                || requirements
                    .iter()
                    .any(|requirement| {
                        *requirement == StrategyRequirement::ExplicitPolicyAuthorization
                    });

        Ok(Self {
            descriptor: StrategyDescriptor {
                id,
                version,
                family: StrategyFamily::Custom,
                phase,
                description,
                requirements,
                expected_overhead,
                deterministic,
                requires_explicit_authorization: requires_authorization,
            },
            semantic_guarantee,
        })
    }

    /// Returns the immutable strategy descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    /// Returns the declared semantic guarantee.
    #[must_use]
    pub const fn semantic_guarantee(&self) -> SemanticGuarantee {
        self.semantic_guarantee
    }
}

// =============================================================================
// Declarative execution plan
// =============================================================================

/// Immutable request produced by a custom strategy for the mitigation
/// executor.
///
/// This is deliberately declarative. The executor remains responsible for
/// determining whether and how this request can be lowered to the current
/// target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomMitigationPlan {
    /// Strategy identity.
    pub strategy_id: StrategyId,

    /// Strategy version.
    pub strategy_version: StrategyVersion,

    /// Logical/execution/resource scope.
    pub scope: MitigationScope,

    /// Canonical custom parameters.
    pub parameters: CustomParameters,

    /// Semantic guarantee claimed by the strategy.
    pub semantic_guarantee: SemanticGuarantee,

    /// Whether the executor must require verification before acceptance.
    pub verification_required: bool,
}

impl CustomMitigationPlan {
    /// Creates a validated declarative plan.
    pub fn new(
        definition: &CustomMitigationDefinition,
        scope: MitigationScope,
        parameters: CustomParameters,
    ) -> CustomMitigationResult<Self> {
        if let MitigationScope::LogicalQubits(qubits) = &scope {
            if qubits.is_empty() {
                return Err(CustomMitigationError::EmptyLogicalQubitScope);
            }
        }

        Ok(Self {
            strategy_id: definition.descriptor.id.clone(),
            strategy_version: definition.descriptor.version,
            scope,
            parameters,
            semantic_guarantee: definition.semantic_guarantee,
            verification_required: true,
        })
    }
}

// =============================================================================
// Custom mitigation strategy
// =============================================================================

/// Production custom mitigation strategy implementation.
///
/// This type implements the repository's common `MitigationStrategy` contract
/// while exposing a declarative plan to the executor.
#[derive(Debug, Clone)]
pub struct CustomMitigationStrategy {
    definition: CustomMitigationDefinition,
    parameters: CustomParameters,
}

impl CustomMitigationStrategy {
    /// Creates a custom mitigation strategy.
    pub fn new(
        definition: CustomMitigationDefinition,
        parameters: CustomParameters,
    ) -> CustomMitigationResult<Self> {
        Ok(Self {
            definition,
            parameters,
        })
    }

    /// Creates a custom strategy directly from its descriptor inputs.
    pub fn from_parts(
        id: impl Into<String>,
        version: StrategyVersion,
        phase: StrategyPhase,
        description: impl Into<Arc<str>>,
        requirements: impl Into<Arc<[StrategyRequirement]>>,
        expected_overhead: impl Into<Arc<[ExpectedOverhead]>>,
        deterministic: bool,
        requires_explicit_authorization: bool,
        semantic_guarantee: SemanticGuarantee,
        parameters: CustomParameters,
    ) -> CustomMitigationResult<Self> {
        let definition = CustomMitigationDefinition::new(
            id,
            version,
            phase,
            description,
            requirements,
            expected_overhead,
            deterministic,
            requires_explicit_authorization,
            semantic_guarantee,
        )?;

        Self::new(definition, parameters)
    }

    /// Returns the immutable definition.
    #[must_use]
    pub fn definition(&self) -> &CustomMitigationDefinition {
        &self.definition
    }

    /// Returns the immutable custom parameters.
    #[must_use]
    pub fn parameters(&self) -> &CustomParameters {
        &self.parameters
    }

    /// Builds a declarative execution plan.
    pub fn plan(
        &self,
        scope: MitigationScope,
    ) -> CustomMitigationResult<CustomMitigationPlan> {
        CustomMitigationPlan::new(&self.definition, scope, self.parameters.clone())
    }

    /// Evaluates capability requirements without selecting or executing the
    /// strategy.
    fn evaluate_requirements(
        &self,
        context: &StrategyContext,
    ) -> StrategyEvaluation {
        let descriptor = self.descriptor();

        let mut missing = Vec::new();
        let mut insufficient_information = false;

        for requirement in descriptor.requirements.iter().copied() {
            let state = match requirement {
                StrategyRequirement::ClassicalPostProcessing => {
                    context.statistical_analysis_available
                }

                StrategyRequirement::MeasurementResults => {
                    context.measurement_results_available
                }

                StrategyRequirement::RepeatedExecution => {
                    context.repeated_execution_allowed
                }

                StrategyRequirement::NoiseScaling => {
                    context.noise_scaling_available
                }

                StrategyRequirement::ParameterVariation => {
                    context.parameter_variation_available
                }

                StrategyRequirement::RandomizedCompilation => {
                    context.randomized_compilation_available
                }

                StrategyRequirement::RandomnessProvenance => {
                    context.randomness_provenance_available
                }

                StrategyRequirement::ScheduleControl => {
                    context.schedule_control_available
                }

                StrategyRequirement::TimingInformation => {
                    context.timing_information_available
                }

                StrategyRequirement::PulseControl => {
                    context.pulse_control_available
                }

                StrategyRequirement::StatisticalAnalysis => {
                    context.statistical_analysis_available
                }

                StrategyRequirement::Provenance => {
                    context.provenance_available
                }

                StrategyRequirement::CrossExecutionCorrelation => {
                    context.cross_execution_correlation_available
                }

                StrategyRequirement::ScopedExecution => true,

                StrategyRequirement::ExplicitPolicyAuthorization => {
                    context.policy_authorized
                }

                // The current StrategyContext intentionally does not expose
                // a generic `variant_execution_available` field. Do not
                // falsely report support for this requirement.
                StrategyRequirement::VariantExecution => {
                    insufficient_information = true;
                    false
                }
            };

            if !state {
                missing.push(requirement);
            }
        }

        let applicability = if insufficient_information {
            Applicability::InsufficientInformation
        } else if missing.is_empty() {
            Applicability::Applicable
        } else if descriptor.requires_explicit_authorization
            && !context.policy_authorized
        {
            Applicability::RequiresPolicyValidation
        } else {
            Applicability::RequiresCapabilityValidation
        };

        StrategyEvaluation {
            strategy_id: descriptor.id.clone(),
            strategy_version: descriptor.version,
            applicability,
            missing_requirements: missing.into(),
        }
    }
}

impl MitigationStrategy for CustomMitigationStrategy {
    fn descriptor(&self) -> &StrategyDescriptor {
        self.definition.descriptor()
    }

    fn evaluate(&self, context: &StrategyContext) -> StrategyEvaluation {
        self.evaluate_requirements(context)
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates the overhead descriptor for a custom strategy whose actual
/// resource cost is target-dependent.
///
/// This intentionally reports `Unknown` instead of inventing a numerical
/// estimate.
#[must_use]
pub const fn unknown_execution_overhead() -> ExpectedOverhead {
    ExpectedOverhead::new(
        OverheadDimension::Time,
        OverheadLevel::Unknown,
    )
}

/// Creates an overhead descriptor for a custom strategy that is known to add
/// quantum operations but whose quantity is determined only after target
/// lowering.
#[must_use]
pub const fn target_dependent_quantum_operation_overhead() -> ExpectedOverhead {
    ExpectedOverhead::new(
        OverheadDimension::QuantumOperations,
        OverheadLevel::Unknown,
    )
}

/// Creates an overhead descriptor for a custom strategy that requires
/// additional executions determined by the strategy itself.
#[must_use]
pub const fn target_dependent_execution_overhead() -> ExpectedOverhead {
    ExpectedOverhead::new(
        OverheadDimension::Executions,
        OverheadLevel::Unknown,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn definition() -> CustomMitigationDefinition {
        CustomMitigationDefinition::new(
            "example.custom.mitigation",
            StrategyVersion::new(1, 0, 0),
            StrategyPhase::PreExecution,
            "Example custom mitigation strategy",
            Arc::from([
                StrategyRequirement::Provenance,
                StrategyRequirement::ScopedExecution,
            ]),
            Arc::from([
                ExpectedOverhead::new(
                    OverheadDimension::QuantumOperations,
                    OverheadLevel::Unknown,
                ),
            ]),
            true,
            true,
            SemanticGuarantee::RequiresVerification,
        )
        .expect("test definition must be valid")
    }

    #[test]
    fn parameter_collection_is_canonicalized() {
        let first = CustomParameter::new(
            "z",
            CustomParameterValue::Unsigned(1),
        )
        .expect("parameter must be valid");

        let second = CustomParameter::new(
            "a",
            CustomParameterValue::Boolean(true),
        )
        .expect("parameter must be valid");

        let parameters =
            CustomParameters::new([first, second]).expect("parameters must be valid");

        assert_eq!(parameters.as_slice()[0].name(), "a");
        assert_eq!(parameters.as_slice()[1].name(), "z");
    }

    #[test]
    fn duplicate_parameter_names_are_rejected() {
        let first = CustomParameter::new(
            "same",
            CustomParameterValue::Unsigned(1),
        )
        .expect("parameter must be valid");

        let second = CustomParameter::new(
            "same",
            CustomParameterValue::Unsigned(2),
        )
        .expect("parameter must be valid");

        assert_eq!(
            CustomParameters::new([first, second]),
            Err(CustomMitigationError::DuplicateParameterName)
        );
    }

    #[test]
    fn non_finite_float_is_rejected() {
        assert_eq!(
            CustomParameterValue::float(f64::NAN),
            Err(CustomMitigationError::NonFiniteFloatParameter)
        );

        assert_eq!(
            CustomParameterValue::float(f64::INFINITY),
            Err(CustomMitigationError::NonFiniteFloatParameter)
        );
    }

    #[test]
    fn custom_strategy_implements_common_contract() {
        let strategy = CustomMitigationStrategy::new(
            definition(),
            CustomParameters::empty(),
        )
        .expect("strategy must be valid");

        assert_eq!(
            strategy.descriptor().family,
            StrategyFamily::Custom
        );

        assert_eq!(
            strategy.descriptor().id.as_str(),
            "example.custom.mitigation"
        );
    }

    #[test]
    fn policy_authorization_is_not_bypassed() {
        let strategy = CustomMitigationStrategy::new(
            definition(),
            CustomParameters::empty(),
        )
        .expect("strategy must be valid");

        let context = StrategyContext {
            provenance_available: true,
            policy_authorized: false,
            ..StrategyContext::default()
        };

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::RequiresPolicyValidation
        );

        assert!(
            evaluation
                .missing_requirements
                .contains(&StrategyRequirement::ExplicitPolicyAuthorization)
        );
    }

    #[test]
    fn authorized_strategy_can_become_applicable() {
        let strategy = CustomMitigationStrategy::new(
            definition(),
            CustomParameters::empty(),
        )
        .expect("strategy must be valid");

        let context = StrategyContext {
            provenance_available: true,
            policy_authorized: true,
            ..StrategyContext::default()
        };

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::Applicable
        );

        assert!(evaluation.missing_requirements.is_empty());
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_ir_type() {
        let qubit = crate::quantum::ir::qubit::QubitId::new(0)
            .expect("canonical qubit identifier must be constructible");

        let scope = MitigationScope::logical_qubits([qubit]);

        assert_eq!(
            scope.logical_qubits_ref().map(|value| value.len()),
            Some(1)
        );
    }

    #[test]
    fn empty_logical_scope_is_rejected_when_planning() {
        let definition = definition();

        let result = CustomMitigationPlan::new(
            &definition,
            MitigationScope::logical_qubits(
                std::iter::empty::<crate::quantum::ir::qubit::QubitId>(),
            ),
            CustomParameters::empty(),
        );

        assert_eq!(
            result,
            Err(CustomMitigationError::EmptyLogicalQubitScope)
        );
    }

    #[test]
    fn plan_requires_verification() {
        let definition = definition();

        let plan = CustomMitigationPlan::new(
            &definition,
            MitigationScope::program(),
            CustomParameters::empty(),
        )
        .expect("plan must be valid");

        assert!(plan.verification_required);
        assert_eq!(
            plan.semantic_guarantee,
            SemanticGuarantee::RequiresVerification
        );
    }

    #[test]
    fn float_bits_round_trip() {
        let value = 1.25_f64;

        let parameter =
            CustomParameterValue::float(value).expect("finite float");

        assert_eq!(parameter.as_float(), Some(value));
    }
}