
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Forward-Mode Differentiation
//!
//! Path: src/ai/differentiation/forward.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent forward-mode automatic differentiation
//! orchestration.
//!
//! Forward-mode differentiation propagates primal values and their
//! corresponding tangent values through differentiable operations.
//!
//! This module defines:
//!
//! - Primal/tangent value pairs.
//! - Forward-mode operation-provider contracts.
//! - Rule-registry-based derivative resolution.
//! - Forward-mode propagation results.
//! - Explicit differentiation failures.
//! - Verification-policy handling.
//!
//! ## Architectural ownership
//!
//! Source syntax:
//!     grammar/ai/differentiation.g4
//!
//! Differentiability declarations:
//!     grammar/ai/differentiable.g4
//!
//! Gradient requests:
//!     crate::ai::learning::gradients
//!
//! Training references:
//!     crate::ai::learning::training
//!
//! Differentiation rules:
//!     crate::ai::differentiation::rules
//!
//! Model operation contracts:
//!     crate::ai::model::operations
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! Quantum computational representation:
//!     crate::quantum::ir
//!
//! Execution and resource realization:
//!     crate::runtime
//!     crate::distributed
//!     crate::hybrid
//!
//! ## Owns
//!
//! - Forward-mode primal/tangent value pairing.
//! - Forward-mode propagation orchestration.
//! - Forward-mode derivative-rule resolution.
//! - Forward-mode provider contracts.
//! - Forward-mode result metadata.
//! - Forward-mode-specific error classification.
//!
//! ## Does not own
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic name resolution.
//! - Mathematical differentiability analysis.
//! - Gradient request construction.
//! - Differentiation rule registration.
//! - Reverse-mode differentiation.
//! - Symbolic algebra.
//! - Numerical finite differences.
//! - Tensor storage.
//! - Tensor arithmetic kernels.
//! - Model graph construction.
//! - A second differentiation graph.
//! - ZUIR construction.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling.
//! - Distributed execution.
//! - Runtime kernel execution.
//!
//! ## Forward-mode contract
//!
//! For a differentiable operation:
//!
//!     y = f(x)
//!
//! Forward-mode propagation carries:
//!
//!     (x, ẋ)
//!
//! to:
//!
//!     (y, ẏ)
//!
//! where:
//!
//!     ẏ = J_f(x) · ẋ
//!
//! The mathematical operation represented by the Jacobian-vector product
//! is delegated to a registered derivative operation provider.
//!
//! This module does not assume that values are scalars. The primal and
//! tangent types are generic and may represent:
//!
//! - Scalars.
//! - Vectors.
//! - Matrices.
//! - Arbitrary-rank tensors.
//! - Symbolic values.
//! - Structured values.
//! - Quantum-related values.
//! - Hybrid computational values.
//! - Future domain-specific values.
//!
//! The provider is responsible for preserving the mathematical and
//! representation contracts of its values.
//!
//! ## POCO-REAF
//!
//! No fixed limits are imposed on:
//!
//! - Number of propagated values.
//! - Number of operation inputs.
//! - Number of operation outputs.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Model parameters.
//! - Differentiation variables.
//! - Differentiation rules.
//! - Operation providers.
//! - Devices.
//! - Workers.
//! - Memory capacity.
//!
//! Collections and value representations are bounded only by the
//! implementation's representational limits and available resources.
//!
//! Operational budgets and resource feasibility are downstream concerns.
//!
//! Literal infinite computation is not promised.
//!
//! ## Integration
//!
//! `rules.rs`:
//!     Resolves the derivative operation for a requested operation,
//!     differentiation mode and computational domain.
//!
//! `learning/gradients.rs`:
//!     Supplies gradient requests and mathematical result intent.
//!
//! `learning/training.rs`:
//!     Supplies canonical symbolic training references.
//!
//! `model/operations.rs`:
//!     Supplies operation differentiability declarations to the
//!     semantic validation boundary.
//!
//! `tensor/`:
//!     Supplies tensor contracts and interoperability.
//!
//! `zuir/`:
//!     Receives canonical operations through the appropriate lowering
//!     and execution-provider integration.
//!
//! `runtime/`:
//!     Realizes lowered operations using available resources.
//!
//! This module never treats successful rule resolution as proof of
//! semantic validity, target compatibility or resource feasibility.

use core::fmt;
use std::error::Error;

use crate::ai::differentiation::rules::{
    DifferentiableOperationId,
    DifferentiationDomain,
    DifferentiationMode,
    DifferentiationRule,
    DifferentiationRuleError,
    DifferentiationRuleId,
    DifferentiationRuleRegistry,
    DifferentiationRuleRequest,
    DifferentiationRuleResolution,
    DifferentiationRuleVersion,
    DerivativeOperationRef,
};

// ============================================================================
// PRIMAL / TANGENT VALUE
// ============================================================================

/// A value paired with its forward-mode tangent.
///
/// `Primal` represents the ordinary value produced by a computation.
///
/// `Tangent` represents the directional derivative associated with
/// the selected differentiation variable or direction.
///
/// The types are generic because a primal and tangent may be represented
/// by different canonical value types.
///
/// This structure does not allocate tensor storage or evaluate arithmetic.
#[derive(Clone, Debug, PartialEq)]
pub struct ForwardValue<Primal, Tangent> {
    primal: Primal,
    tangent: Tangent,
}

impl<Primal, Tangent> ForwardValue<Primal, Tangent> {
    /// Creates a primal/tangent pair.
    ///
    /// Mathematical compatibility between the two values is established
    /// by the canonical semantic and tensor validation infrastructure.
    #[must_use]
    pub const fn new(primal: Primal, tangent: Tangent) -> Self {
        Self { primal, tangent }
    }

    /// Returns the primal value.
    #[must_use]
    pub const fn primal(&self) -> &Primal {
        &self.primal
    }

    /// Returns the tangent value.
    #[must_use]
    pub const fn tangent(&self) -> &Tangent {
        &self.tangent
    }

    /// Consumes the pair and returns its primal and tangent values.
    #[must_use]
    pub fn into_parts(self) -> (Primal, Tangent) {
        (self.primal, self.tangent)
    }

    /// Maps the primal and tangent representations independently.
    ///
    /// This is a representation transformation, not a differentiation
    /// operation.
    pub fn map<NewPrimal, NewTangent, PrimalMap, TangentMap>(
        self,
        map_primal: PrimalMap,
        map_tangent: TangentMap,
    ) -> ForwardValue<NewPrimal, NewTangent>
    where
        PrimalMap: FnOnce(Primal) -> NewPrimal,
        TangentMap: FnOnce(Tangent) -> NewTangent,
    {
        ForwardValue {
            primal: map_primal(self.primal),
            tangent: map_tangent(self.tangent),
        }
    }
}

// ============================================================================
// FORWARD-MODE EXECUTION PROVIDER
// ============================================================================

/// Executes a resolved forward-mode derivative operation.
///
/// This is the integration boundary between forward-mode differentiation
/// and the canonical computational execution infrastructure.
///
/// An implementation may:
///
/// - Translate the derivative operation into canonical ZUIR.
/// - Resolve a registered ZUIR operation implementation.
/// - Invoke an appropriate tensor or domain-specific implementation.
/// - Delegate execution to the runtime.
/// - Preserve symbolic values without materializing them.
/// - Delegate quantum or hybrid operations to their authoritative
///   computational subsystems.
///
/// It must not silently change the mathematical meaning of the operation.
///
/// It must return an error when the requested derivative operation
/// cannot be realized.
///
/// The provider receives the complete resolved rule so that it can
/// inspect its operation, derivative reference, version, domain,
/// differentiability classification and verification metadata.
///
/// The provider must not assume that the inputs are scalar values.
pub trait ForwardModeExecutor<Primal, Tangent> {
    /// Error returned by this provider.
    type Error: Error + Send + Sync + 'static;

    /// Executes the Jacobian-vector product represented by a resolved rule.
    ///
    /// Each input contains its primal value and tangent value.
    ///
    /// The returned values contain the corresponding primal outputs
    /// and tangent outputs.
    ///
    /// The provider is responsible for:
    ///
    /// - Input/output signature validation.
    /// - Shape and dtype compatibility.
    /// - Numerical correctness.
    /// - Differentiability conditions.
    /// - Domain-specific operation semantics.
    /// - Resource and execution failures.
    ///
    /// This method does not imply a particular physical device.
    fn apply_jvp(
        &mut self,
        rule: &DifferentiationRule,
        inputs: &[ForwardValue<Primal, Tangent>],
    ) -> Result<Vec<ForwardValue<Primal, Tangent>>, Self::Error>;
}

// ============================================================================
// FORWARD-MODE ERROR
// ============================================================================

/// Failure produced by forward-mode differentiation orchestration.
///
/// Provider errors are preserved as their original error type.
///
/// Rule-resolution errors remain distinct from execution-provider errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum ForwardModeError<ProviderError> {
    /// The derivative rule registry could not be accessed.
    RuleRegistry(DifferentiationRuleError),

    /// No forward-mode derivative rule was registered.
    RuleNotFound {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Computational domain of the operation.
        domain: DifferentiationDomain,
    },

    /// More than one rule matched the request.
    ///
    /// The caller must resolve the ambiguity explicitly.
    AmbiguousRules {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Matching rule identities.
        candidates: Vec<DifferentiationRuleId>,
    },

    /// A matching rule exists, but verification is required.
    VerificationRequired {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,
    },

    /// Rules exist for the operation, but not at the requested version.
    VersionUnavailable {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Exact requested rule version.
        version: DifferentiationRuleVersion,
    },

    /// The derivative provider failed.
    Provider(ProviderError),
}

impl<ProviderError> fmt::Display for ForwardModeError<ProviderError>
where
    ProviderError: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RuleRegistry(error) => {
                write!(
                    formatter,
                    "forward-mode rule registry failure: {error}"
                )
            }

            Self::RuleNotFound { operation, domain } => {
                write!(
                    formatter,
                    "no forward-mode derivative rule for operation \
                     '{operation}' in domain '{domain:?}'"
                )
            }

            Self::AmbiguousRules {
                operation,
                candidates,
            } => {
                write!(
                    formatter,
                    "ambiguous forward-mode derivative rules for \
                     operation '{operation}': {candidates:?}"
                )
            }

            Self::VerificationRequired { operation } => {
                write!(
                    formatter,
                    "a verified forward-mode derivative rule is required \
                     for operation '{operation}'"
                )
            }

            Self::VersionUnavailable {
                operation,
                version,
            } => {
                write!(
                    formatter,
                    "forward-mode derivative rule version '{version}' \
                     is unavailable for operation '{operation}'"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "forward-mode derivative provider failed: {error}"
                )
            }
        }
    }
}

impl<ProviderError> Error for ForwardModeError<ProviderError>
where
    ProviderError: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RuleRegistry(error) => Some(error),
            Self::Provider(error) => Some(error),
            Self::RuleNotFound { .. }
            | Self::AmbiguousRules { .. }
            | Self::VerificationRequired { .. }
            | Self::VersionUnavailable { .. } => None,
        }
    }
}

// ============================================================================
// FORWARD-MODE RESULT
// ============================================================================

/// Result of propagating one operation in forward mode.
///
/// This result records the resolved rule identity and version alongside
/// the provider-produced primal/tangent outputs.
///
/// It does not claim that the entire model or gradient request has been
/// validated. That remains the responsibility of the canonical semantic
/// and differentiation validation infrastructure.
#[derive(Clone, Debug, PartialEq)]
pub struct ForwardModeResult<Primal, Tangent> {
    rule_id: DifferentiationRuleId,
    rule_version: DifferentiationRuleVersion,
    derivative_operation: DerivativeOperationRef,
    outputs: Vec<ForwardValue<Primal, Tangent>>,
}

impl<Primal, Tangent> ForwardModeResult<Primal, Tangent> {
    fn new(
        rule_id: DifferentiationRuleId,
        rule_version: DifferentiationRuleVersion,
        derivative_operation: DerivativeOperationRef,
        outputs: Vec<ForwardValue<Primal, Tangent>>,
    ) -> Self {
        Self {
            rule_id,
            rule_version,
            derivative_operation,
            outputs,
        }
    }

    /// Returns the resolved differentiation rule identity.
    #[must_use]
    pub fn rule_id(&self) -> &DifferentiationRuleId {
        &self.rule_id
    }

    /// Returns the resolved differentiation rule version.
    #[must_use]
    pub const fn rule_version(&self) -> DifferentiationRuleVersion {
        self.rule_version
    }

    /// Returns the canonical derivative operation reference.
    #[must_use]
    pub fn derivative_operation(&self) -> &DerivativeOperationRef {
        &self.derivative_operation
    }

    /// Returns the propagated primal/tangent outputs.
    #[must_use]
    pub fn outputs(&self) -> &[ForwardValue<Primal, Tangent>] {
        &self.outputs
    }

    /// Consumes the result and returns its outputs.
    #[must_use]
    pub fn into_outputs(self) -> Vec<ForwardValue<Primal, Tangent>> {
        self.outputs
    }
}

// ============================================================================
// FORWARD-MODE ENGINE
// ============================================================================

/// Target-independent forward-mode differentiation coordinator.
///
/// The engine:
///
/// 1. Resolves the operation's forward-mode derivative rule.
/// 2. Applies the configured verification policy.
/// 3. Delegates the Jacobian-vector product to a provider.
/// 4. Returns the provider's outputs and the rule provenance identifiers.
///
/// The engine does not construct a graph, perform tensor arithmetic,
/// allocate device memory, select hardware or schedule work.
///
/// The registry is shared with the differentiation-rule subsystem.
/// Cloning this engine does not create a separate rule catalogue.
#[derive(Clone, Debug)]
pub struct ForwardModeEngine {
    registry: DifferentiationRuleRegistry,
    domain: DifferentiationDomain,
    require_verified_rules: bool,
}

impl ForwardModeEngine {
    /// Creates a forward-mode engine.
    ///
    /// The registry and computational domain must be supplied by the
    /// surrounding AI context or integration layer.
    ///
    /// Verification is not required by default. Applications that require
    /// verified mathematical rules should use
    /// `requiring_verified_rules`.
    #[must_use]
    pub fn new(
        registry: DifferentiationRuleRegistry,
        domain: DifferentiationDomain,
    ) -> Self {
        Self {
            registry,
            domain,
            require_verified_rules: false,
        }
    }

    /// Requires every resolved derivative rule to be verified.
    ///
    /// This is a policy choice. It does not itself verify a rule.
    #[must_use]
    pub fn requiring_verified_rules(mut self) -> Self {
        self.require_verified_rules = true;
        self
    }

    /// Returns the configured computational domain.
    #[must_use]
    pub fn domain(&self) -> &DifferentiationDomain {
        &self.domain
    }

    /// Returns whether verified rules are mandatory.
    #[must_use]
    pub const fn requires_verified_rules(&self) -> bool {
        self.require_verified_rules
    }

    /// Returns a reference to the shared differentiation rule registry.
    #[must_use]
    pub fn registry(&self) -> &DifferentiationRuleRegistry {
        &self.registry
    }

    /// Propagates primal and tangent values through one operation.
    ///
    /// `operation` must identify the operation using the same canonical
    /// identity registered in the differentiation-rule registry.
    ///
    /// `inputs` are supplied in the operation's canonical input order.
    ///
    /// An empty input list is allowed because operation arity and
    /// constant-operation semantics belong to the operation signature.
    ///
    /// The provider validates the actual operation signature and returns
    /// the corresponding output values.
    ///
    /// No implicit fallback to reverse mode, numerical differentiation,
    /// symbolic differentiation or another strategy occurs.
    pub fn propagate<Primal, Tangent, Provider>(
        &self,
        operation: DifferentiableOperationId,
        inputs: &[ForwardValue<Primal, Tangent>],
        provider: &mut Provider,
    ) -> Result<
        ForwardModeResult<Primal, Tangent>,
        ForwardModeError<Provider::Error>,
    >
    where
        Provider: ForwardModeExecutor<Primal, Tangent>,
    {
        let mut request = DifferentiationRuleRequest::new(
            operation.clone(),
            DifferentiationMode::Forward,
            self.domain.clone(),
        );

        if self.require_verified_rules {
            request = request.requiring_verified_rule();
        }

        self.propagate_with_request(request, inputs, provider)
    }

    /// Propagates values using an explicitly versioned derivative rule.
    ///
    /// Exact rule versioning prevents an unversioned lookup from silently
    /// selecting a different mathematical contract.
    pub fn propagate_with_version<Primal, Tangent, Provider>(
        &self,
        operation: DifferentiableOperationId,
        version: DifferentiationRuleVersion,
        inputs: &[ForwardValue<Primal, Tangent>],
        provider: &mut Provider,
    ) -> Result<
        ForwardModeResult<Primal, Tangent>,
        ForwardModeError<Provider::Error>,
    >
    where
        Provider: ForwardModeExecutor<Primal, Tangent>,
    {
        let mut request = DifferentiationRuleRequest::new(
            operation,
            DifferentiationMode::Forward,
            self.domain.clone(),
        )
        .with_version(version);

        if self.require_verified_rules {
            request = request.requiring_verified_rule();
        }

        self.propagate_with_request(request, inputs, provider)
    }

    fn propagate_with_request<Primal, Tangent, Provider>(
        &self,
        request: DifferentiationRuleRequest,
        inputs: &[ForwardValue<Primal, Tangent>],
        provider: &mut Provider,
    ) -> Result<
        ForwardModeResult<Primal, Tangent>,
        ForwardModeError<Provider::Error>,
    >
    where
        Provider: ForwardModeExecutor<Primal, Tangent>,
    {
        let operation = request.operation().clone();

        let resolution = self
            .registry
            .resolve(&request)
            .map_err(ForwardModeError::RuleRegistry)?;

        let rule = match resolution {
            DifferentiationRuleResolution::Resolved(rule) => rule,

            DifferentiationRuleResolution::NotFound => {
                return Err(ForwardModeError::RuleNotFound {
                    operation,
                    domain: self.domain.clone(),
                });
            }

            DifferentiationRuleResolution::Ambiguous(candidates) => {
                return Err(ForwardModeError::AmbiguousRules {
                    operation,
                    candidates,
                });
            }

            DifferentiationRuleResolution::VerificationRequired => {
                return Err(ForwardModeError::VerificationRequired {
                    operation,
                });
            }

            DifferentiationRuleResolution::VersionUnavailable => {
                let version = request
                    .required_version()
                    .expect(
                        "version-unavailable resolution requires a requested version",
                    );

                return Err(ForwardModeError::VersionUnavailable {
                    operation,
                    version,
                });
            }

            _ => {
                // The resolution enum is non-exhaustive. Future variants
                // must fail closed rather than execute an unknown rule.
                return Err(ForwardModeError::RuleNotFound {
                    operation,
                    domain: self.domain.clone(),
                });
            }
        };

        let rule_id = rule.id().clone();
        let rule_version = rule.version();
        let derivative_operation = rule.derivative().clone();

        let outputs = provider
            .apply_jvp(&rule, inputs)
            .map_err(ForwardModeError::Provider)?;

        Ok(ForwardModeResult::new(
            rule_id,
            rule_version,
            derivative_operation,
            outputs,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::differentiation::rules::{
        Differentiability,
        DifferentiationRegularity,
        DifferentiationRuleProvenance,
        DifferentiationRuleVersion,
        DifferentiationRuleVerification,
    };

    use std::convert::Infallible;

    fn operation_id() -> DifferentiableOperationId {
        DifferentiableOperationId::new("test.scalar.multiply").unwrap()
    }

    fn make_rule(
        version: DifferentiationRuleVersion,
    ) -> DifferentiationRule {
        DifferentiationRule::new(
            DifferentiationRuleId::new("test.scalar.multiply.forward").unwrap(),
            operation_id(),
            DerivativeOperationRef::new(
                "test.scalar.multiply.jvp",
            )
            .unwrap(),
            version,
            vec![DifferentiationMode::Forward],
            vec![DifferentiationDomain::Classical],
            Differentiability::Differentiable,
            DifferentiationRegularity::ContinuouslyDifferentiable,
            DifferentiationRuleProvenance::new(
                "forward-mode-tests",
                "Zamani test suite",
                "Test derivative rule",
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[derive(Debug)]
    struct TestProviderError;

    impl fmt::Display for TestProviderError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("test provider failure")
        }
    }

    impl Error for TestProviderError {}

    struct TestProvider;

    impl ForwardModeExecutor<f64, f64> for TestProvider {
        type Error = TestProviderError;

        fn apply_jvp(
            &mut self,
            rule: &DifferentiationRule,
            inputs: &[ForwardValue<f64, f64>],
        ) -> Result<Vec<ForwardValue<f64, f64>>, Self::Error> {
            if rule.derivative().as_str() != "test.scalar.multiply.jvp" {
                return Err(TestProviderError);
            }

            if inputs.len() != 2 {
                return Err(TestProviderError);
            }

            let left = inputs[0].primal();
            let right = inputs[1].primal();

            let left_tangent = inputs[0].tangent();
            let right_tangent = inputs[1].tangent();

            let primal = left * right;

            // d(x*y) = dx*y + x*dy
            let tangent =
                left_tangent * right + left * right_tangent;

            Ok(vec![ForwardValue::new(primal, tangent)])
        }
    }

    #[test]
    fn propagates_primal_and_tangent_values() {
        let registry = DifferentiationRuleRegistry::new();
        registry
            .register(make_rule(DifferentiationRuleVersion::new(
                1, 0, 0,
            )))
            .unwrap();

        let engine = ForwardModeEngine::new(
            registry,
            DifferentiationDomain::Classical,
        );

        let inputs = vec![
            ForwardValue::new(3.0, 1.0),
            ForwardValue::new(4.0, 0.0),
        ];

        let result = engine
            .propagate(operation_id(), &inputs, &mut TestProvider)
            .unwrap();

        assert_eq!(result.outputs().len(), 1);
        assert_eq!(*result.outputs()[0].primal(), 12.0);
        assert_eq!(*result.outputs()[0].tangent(), 4.0);
        assert_eq!(
            result.derivative_operation().as_str(),
            "test.scalar.multiply.jvp"
        );
    }

    #[test]
    fn reports_missing_derivative_rule() {
        let engine = ForwardModeEngine::new(
            DifferentiationRuleRegistry::new(),
            DifferentiationDomain::Classical,
        );

        let result = engine.propagate(
            operation_id(),
            &[],
            &mut TestProvider,
        );

        assert!(matches!(
            result,
            Err(ForwardModeError::RuleNotFound { .. })
        ));
    }

    #[test]
    fn exact_rule_version_is_respected() {
        let registry = DifferentiationRuleRegistry::new();
        registry
            .register(make_rule(DifferentiationRuleVersion::new(
                1, 0, 0,
            )))
            .unwrap();

        let engine = ForwardModeEngine::new(
            registry,
            DifferentiationDomain::Classical,
        );

        let result = engine.propagate_with_version(
            operation_id(),
            DifferentiationRuleVersion::new(2, 0, 0),
            &[],
            &mut TestProvider,
        );

        assert!(matches!(
            result,
            Err(ForwardModeError::VersionUnavailable { .. })
        ));
    }

    #[test]
    fn verified_rule_policy_fails_closed_for_unverified_rule() {
        let registry = DifferentiationRuleRegistry::new();
        registry
            .register(make_rule(DifferentiationRuleVersion::new(
                1, 0, 0,
            )))
            .unwrap();

        let engine = ForwardModeEngine::new(
            registry,
            DifferentiationDomain::Classical,
        )
        .requiring_verified_rules();

        let result = engine.propagate(
            operation_id(),
            &[],
            &mut TestProvider,
        );

        assert!(matches!(
            result,
            Err(ForwardModeError::VerificationRequired { .. })
        ));
    }

    #[test]
    fn provider_errors_are_not_reported_as_success() {
        struct FailingProvider;

        impl ForwardModeExecutor<f64, f64> for FailingProvider {
            type Error = TestProviderError;

            fn apply_jvp(
                &mut self,
                _rule: &DifferentiationRule,
                _inputs: &[ForwardValue<f64, f64>],
            ) -> Result<
                Vec<ForwardValue<f64, f64>>,
                Self::Error,
            > {
                Err(TestProviderError)
            }
        }

        let registry = DifferentiationRuleRegistry::new();
        registry
            .register(make_rule(DifferentiationRuleVersion::new(
                1, 0, 0,
            )))
            .unwrap();

        let engine = ForwardModeEngine::new(
            registry,
            DifferentiationDomain::Classical,
        );

        let result = engine.propagate(
            operation_id(),
            &[],
            &mut FailingProvider,
        );

        assert!(matches!(
            result,
            Err(ForwardModeError::Provider(_))
        ));
    }

    #[test]
    fn forward_value_maps_primal_and_tangent_independently() {
        let value = ForwardValue::new(5_u32, 2_u32);

        let mapped = value.map(
            |primal| primal.to_string(),
            |tangent| tangent as u64,
        );

        assert_eq!(mapped.primal(), "5");
        assert_eq!(*mapped.tangent(), 2_u64);
    }

    #[test]
    fn provider_error_type_can_be_infallible() {
        struct IdentityProvider;

        impl ForwardModeExecutor<i32, i32> for IdentityProvider {
            type Error = Infallible;

            fn apply_jvp(
                &mut self,
                _rule: &DifferentiationRule,
                inputs: &[ForwardValue<i32, i32>],
            ) -> Result<
                Vec<ForwardValue<i32, i32>>,
                Self::Error,
            > {
                Ok(inputs.to_vec())
            }
        }

        let _provider = IdentityProvider;
    }

    #[test]
    fn rule_version_metadata_is_preserved() {
        let registry = DifferentiationRuleRegistry::new();
        registry
            .register(make_rule(DifferentiationRuleVersion::new(
                3, 2, 1,
            )))
            .unwrap();

        let engine = ForwardModeEngine::new(
            registry,
            DifferentiationDomain::Classical,
        );

        let result = engine
            .propagate(
                operation_id(),
                &[
                    ForwardValue::new(2.0, 1.0),
                    ForwardValue::new(3.0, 0.0),
                ],
                &mut TestProvider,
            )
            .unwrap();

        assert_eq!(
            result.rule_version(),
            DifferentiationRuleVersion::new(3, 2, 1)
        );
    }
}
