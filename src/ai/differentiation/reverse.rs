
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI — Reverse-Mode Automatic Differentiation
//!
//! Path: src/ai/differentiation/reverse.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides target-independent reverse-mode automatic differentiation
//! orchestration.
//!
//! Reverse-mode differentiation propagates cotangents backwards through
//! an ordered computational execution trace.
//!
//! For a computation:
//!
//!     y = f(x)
//!
//! and an output cotangent:
//!
//!     y_bar = dL/dy
//!
//! reverse-mode propagation computes:
//!
//!     x_bar = J_f(x)^T * y_bar
//!
//! This module coordinates:
//!
//! - Reverse-mode differentiation rule resolution.
//! - Ordered operation trace validation.
//! - Output cotangent initialization.
//! - Reverse traversal.
//! - Vector-Jacobian product delegation.
//! - Cotangent accumulation.
//! - Requested input-gradient extraction.
//! - Rule version tracking.
//! - Verification-policy enforcement.
//! - Explicit differentiation failures.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! Source differentiation syntax:
//!     grammar/ai/differentiation.g4
//!
//! Differentiability contracts:
//!     grammar/ai/differentiable.g4
//!
//! Gradient requests:
//!     crate::ai::learning::gradients
//!
//! Differentiation rules:
//!     crate::ai::differentiation::rules
//!
//! Forward-mode differentiation:
//!     crate::ai::differentiation::forward
//!
//! Canonical AI model operations:
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
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Reverse-mode execution trace contracts.
//! - Reverse-mode orchestration.
//! - Reverse-mode provider interfaces.
//! - Reverse-mode cotangent propagation.
//! - Reverse-mode gradient accumulation.
//! - Reverse-mode result metadata.
//! - Reverse-mode error classification.
//! - Reverse-mode rule provenance references.
//!
//! This module does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic name resolution.
//! - Canonical computational graph construction.
//! - Canonical ZUIR operations.
//! - Tensor storage.
//! - Tensor arithmetic kernels.
//! - Mathematical derivative rule registration.
//! - Forward-mode differentiation.
//! - Symbolic algebra.
//! - Numerical differentiation.
//! - Optimizer algorithms.
//! - Physical memory allocation.
//! - Device discovery.
//! - Hardware selection.
//! - Distributed scheduling.
//! - Runtime kernel execution.
//!
//! ============================================================================
//! EXECUTION TRACE CONTRACT
//! ============================================================================
//!
//! Reverse mode needs operations in their original execution order.
//!
//! This module accepts that order from an upstream canonical execution
//! trace or an adapter.
//!
//! It does not construct another model graph or IR.
//!
//! Every recorded value identity must identify one logical value version.
//!
//! In-place mutations must therefore be represented by the upstream
//! canonical value/versioning mechanism.
//!
//! External input values are values consumed by the trace but not produced
//! by any operation inside that trace.
//!
//! The trace is validated for:
//!
//! - Unique output definitions.
//! - Nonempty operation outputs.
//! - Valid operation ordering.
//! - Absence of forward references.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! No artificial maximum is imposed on:
//!
//! - Number of operations.
//! - Number of inputs.
//! - Number of outputs.
//! - Number of differentiable values.
//! - Number of differentiation variables.
//! - Number of model parameters.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Number of accumulated contributions.
//! - Number of registered rules.
//! - Number of derivative providers.
//! - Number of devices.
//! - Number of workers.
//! - Memory capacity.
//!
//! Collections grow according to available resources and the host's
//! representational limits.
//!
//! This module does not promise literal infinite computation.
//!
//! ============================================================================
//! IMPORTANT CORRECTNESS RULES
//! ============================================================================
//!
//! 1. Missing derivative rules never produce successful gradients.
//!
//! 2. Ambiguous rules are never silently selected.
//!
//! 3. Verification-required policies fail closed.
//!
//! 4. Exact rule versions are respected.
//!
//! 5. Contributions to the same value are accumulated.
//!
//! 6. Multiple operation outputs are supported.
//!
//! 7. Unused branches do not require derivative execution.
//!
//! 8. Derivative execution is delegated to an explicit provider.
//!
//! 9. The provider must preserve mathematical semantics.
//!
//! 10. Resource feasibility is not inferred from successful rule lookup.
//!
//! ============================================================================

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
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
// REVERSE OPERATION
// ============================================================================

/// One operation recorded in the original execution order.
///
/// The operation contains references to canonical values.
///
/// It does not own their storage or numerical representation.
///
/// Inputs and outputs must follow the canonical operation signature.
///
/// The operation's output identities must be unique within the trace.
///
/// If the operation has multiple outputs, each output is represented
/// independently.
///
/// An operation with no inputs is permitted because constant-producing
/// operations may legitimately have zero inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseOperation<ValueId> {
    operation: DifferentiableOperationId,

    inputs: Vec<ValueId>,

    outputs: Vec<ValueId>,

    rule_version: Option<DifferentiationRuleVersion>,
}

impl<ValueId> ReverseOperation<ValueId> {
    /// Creates an operation trace entry.
    ///
    /// Inputs and outputs retain their declared order.
    ///
    /// The constructor does not perform canonical semantic validation.
    pub fn new(
        operation: DifferentiableOperationId,
        inputs: Vec<ValueId>,
        outputs: Vec<ValueId>,
    ) -> Self {
        Self {
            operation,
            inputs,
            outputs,
            rule_version: None,
        }
    }

    /// Requires an exact differentiation-rule version.
    ///
    /// This prevents an unversioned lookup from selecting a different
    /// mathematical derivative contract.
    #[must_use]
    pub fn with_rule_version(
        mut self,
        version: DifferentiationRuleVersion,
    ) -> Self {
        self.rule_version = Some(version);
        self
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub fn operation(&self) -> &DifferentiableOperationId {
        &self.operation
    }

    /// Returns the operation's ordered inputs.
    #[must_use]
    pub fn inputs(&self) -> &[ValueId] {
        &self.inputs
    }

    /// Returns the operation's ordered outputs.
    #[must_use]
    pub fn outputs(&self) -> &[ValueId] {
        &self.outputs
    }

    /// Returns the optional exact differentiation-rule version.
    #[must_use]
    pub const fn rule_version(
        &self,
    ) -> Option<DifferentiationRuleVersion> {
        self.rule_version
    }
}


// ============================================================================
// REVERSE TRACE ERRORS
// ============================================================================

/// Structural failures detected while validating a reverse execution trace.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReverseTapeError {

    /// A recorded operation has no outputs.
    EmptyOperationOutputs {
        /// Position in the original execution trace.
        step_index: usize,
    },

    /// An output value is defined more than once.
    DuplicateValueDefinition {
        /// Position of the conflicting definition.
        step_index: usize,
    },

    /// An operation consumes a value produced by itself or a later step.
    ///
    /// The trace is not in valid execution order.
    NonTopologicalInput {
        /// Position of the consuming operation.
        step_index: usize,

        /// Position of the operation producing the consumed value.
        producer_index: usize,
    },
}

impl fmt::Display for ReverseTapeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyOperationOutputs { step_index } => {
                write!(
                    formatter,
                    "reverse trace operation at step {step_index} \
                     has no outputs"
                )
            }

            Self::DuplicateValueDefinition { step_index } => {
                write!(
                    formatter,
                    "reverse trace contains a duplicate value definition \
                     at step {step_index}"
                )
            }

            Self::NonTopologicalInput {
                step_index,
                producer_index,
            } => {
                write!(
                    formatter,
                    "reverse trace operation at step {step_index} \
                     consumes a value produced at step {producer_index}"
                )
            }
        }
    }
}

impl Error for ReverseTapeError {}


// ============================================================================
// VALIDATED REVERSE TAPE
// ============================================================================

/// A validated, ordered reverse-mode execution trace.
///
/// This is a trace adapter, not a second computational graph.
///
/// The canonical compiler or execution infrastructure supplies the
/// operation records.
///
/// The structure validates their execution order and value definitions.
///
/// It does not contain tensor data, numerical values, or executable kernels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseTape<ValueId> {
    operations: Vec<ReverseOperation<ValueId>>,

    producers: BTreeMap<ValueId, usize>,

    external_inputs: BTreeSet<ValueId>,

    known_values: BTreeSet<ValueId>,
}

impl<ValueId> ReverseTape<ValueId>
where
    ValueId: Clone + Ord,
{
    /// Creates and validates a reverse execution trace.
    ///
    /// The input order is the original forward execution order.
    ///
    /// No arbitrary operation-count limit is imposed.
    pub fn new(
        operations: Vec<ReverseOperation<ValueId>>,
    ) -> Result<Self, ReverseTapeError> {

        let mut producers = BTreeMap::new();

        let mut known_values = BTreeSet::new();

        // -----------------------------------------------------------------
        // Pass 1: establish unique output definitions.
        // -----------------------------------------------------------------

        for (step_index, operation) in operations.iter().enumerate() {

            if operation.outputs.is_empty() {
                return Err(
                    ReverseTapeError::EmptyOperationOutputs {
                        step_index,
                    }
                );
            }

            let mut local_outputs = BTreeSet::new();

            for output in &operation.outputs {

                if !local_outputs.insert(output.clone()) {
                    return Err(
                        ReverseTapeError::DuplicateValueDefinition {
                            step_index,
                        }
                    );
                }

                if producers
                    .insert(output.clone(), step_index)
                    .is_some()
                {
                    return Err(
                        ReverseTapeError::DuplicateValueDefinition {
                            step_index,
                        }
                    );
                }

                known_values.insert(output.clone());
            }
        }

        // -----------------------------------------------------------------
        // Pass 2: validate operation ordering and external inputs.
        // -----------------------------------------------------------------

        let mut external_inputs = BTreeSet::new();

        for (step_index, operation) in operations.iter().enumerate() {

            for input in &operation.inputs {

                match producers.get(input) {

                    Some(producer_index) => {

                        if *producer_index >= step_index {
                            return Err(
                                ReverseTapeError::NonTopologicalInput {
                                    step_index,
                                    producer_index: *producer_index,
                                }
                            );
                        }

                    }

                    None => {

                        external_inputs.insert(input.clone());

                        known_values.insert(input.clone());

                    }
                }
            }
        }

        Ok(Self {
            operations,
            producers,
            external_inputs,
            known_values,
        })
    }

    /// Returns the original forward execution order.
    #[must_use]
    pub fn operations(&self) -> &[ReverseOperation<ValueId>] {
        &self.operations
    }

    /// Returns the values consumed from outside this trace.
    ///
    /// The returned iterator is deterministic.
    pub fn external_inputs(
        &self,
    ) -> impl Iterator<Item = &ValueId> {
        self.external_inputs.iter()
    }

    /// Returns whether a value is produced by the trace.
    #[must_use]
    pub fn contains_produced_value(
        &self,
        value: &ValueId,
    ) -> bool {
        self.producers.contains_key(value)
    }

    /// Returns whether a value is known to the trace.
    #[must_use]
    pub fn contains_value(
        &self,
        value: &ValueId,
    ) -> bool {
        self.known_values.contains(value)
    }

    /// Returns the producing operation index, if one exists.
    #[must_use]
    pub fn producer_index(
        &self,
        value: &ValueId,
    ) -> Option<usize> {
        self.producers.get(value).copied()
    }

    /// Returns the number of recorded operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the trace contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}


// ============================================================================
// REVERSE-MODE EXECUTION PROVIDER
// ============================================================================

/// Executes the vector-Jacobian product for a resolved reverse rule.
///
/// The provider is the boundary between reverse-mode orchestration and
/// canonical mathematical execution.
///
/// It is responsible for accessing the primal values associated with
/// the supplied canonical value identities.
///
/// Primal storage may be owned by:
///
/// - The canonical execution trace.
/// - A runtime value store.
/// - A tensor execution provider.
/// - A symbolic computation provider.
/// - A quantum differentiation provider.
/// - A hybrid execution provider.
///
/// This module does not duplicate that storage.
///
/// # Cotangent contract
///
/// `output_cotangents` has exactly one entry for each operation output.
///
/// `None` means that no cotangent has reached that output.
///
/// The provider must correctly handle partially active multi-output
/// operations.
///
/// The returned input contributions must contain exactly one entry
/// for each operation input.
///
/// # Accumulation contract
///
/// Multiple downstream paths can contribute to the same input value.
///
/// `accumulate` combines these contributions according to the canonical
/// numerical, tensor, symbolic, or domain-specific addition semantics.
///
/// The provider must preserve the declared mathematical meaning.
///
/// It must report incompatibility and numerical failures explicitly.
pub trait ReverseModeExecutor<ValueId, Cotangent> {

    /// Error returned by the provider.
    type Error: Error + Send + Sync + 'static;

    /// Executes the vector-Jacobian product for one operation.
    ///
    /// The provider receives the resolved derivative rule and canonical
    /// input/output value identities.
    ///
    /// It must return one optional cotangent contribution per input.
    fn apply_vjp(
        &mut self,
        rule: &DifferentiationRule,
        inputs: &[ValueId],
        outputs: &[ValueId],
        output_cotangents: &[Option<&Cotangent>],
    ) -> Result<Vec<Option<Cotangent>>, Self::Error>;

    /// Accumulates one additional cotangent contribution.
    ///
    /// For example, two branches may contribute to the gradient of the
    /// same input value.
    ///
    /// The implementation must use the correct canonical addition
    /// semantics for the supplied cotangent representation.
    fn accumulate(
        &mut self,
        accumulated: &mut Cotangent,
        contribution: Cotangent,
    ) -> Result<(), Self::Error>;
}


// ============================================================================
// RULE EXECUTION METADATA
// ============================================================================

/// Identifies a differentiation rule used during reverse propagation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseRuleUse {

    step_index: usize,

    rule_id: DifferentiationRuleId,

    rule_version: DifferentiationRuleVersion,

    derivative_operation: DerivativeOperationRef,
}

impl ReverseRuleUse {

    fn new(
        step_index: usize,
        rule: &DifferentiationRule,
    ) -> Self {
        Self {
            step_index,
            rule_id: rule.id().clone(),
            rule_version: rule.version(),
            derivative_operation: rule.derivative().clone(),
        }
    }

    /// Returns the original forward operation index.
    #[must_use]
    pub const fn step_index(&self) -> usize {
        self.step_index
    }

    /// Returns the differentiation rule identity.
    #[must_use]
    pub fn rule_id(&self) -> &DifferentiationRuleId {
        &self.rule_id
    }

    /// Returns the exact differentiation rule version.
    #[must_use]
    pub const fn rule_version(
        &self,
    ) -> DifferentiationRuleVersion {
        self.rule_version
    }

    /// Returns the canonical derivative operation reference.
    #[must_use]
    pub fn derivative_operation(
        &self,
    ) -> &DerivativeOperationRef {
        &self.derivative_operation
    }
}


// ============================================================================
// REVERSE-MODE RESULT
// ============================================================================

/// Successful result of reverse-mode propagation.
///
/// `adjoints` contains the cotangent accumulated for every value
/// that received a derivative contribution or output seed.
///
/// Requested inputs are preserved in declaration order.
///
/// A requested input without a gradient is represented by the absence
/// of its entry in `adjoints`.
///
/// Rule usage is recorded in reverse execution order.
#[derive(Clone, Debug, PartialEq)]
pub struct ReverseModeResult<ValueId, Cotangent> {

    adjoints: BTreeMap<ValueId, Cotangent>,

    requested_inputs: Vec<ValueId>,

    rules_used: Vec<ReverseRuleUse>,
}

impl<ValueId, Cotangent> ReverseModeResult<ValueId, Cotangent>
where
    ValueId: Ord,
{
    /// Creates a reverse-mode result.
    fn new(
        adjoints: BTreeMap<ValueId, Cotangent>,
        requested_inputs: Vec<ValueId>,
        rules_used: Vec<ReverseRuleUse>,
    ) -> Self {
        Self {
            adjoints,
            requested_inputs,
            rules_used,
        }
    }

    /// Returns all accumulated value cotangents.
    #[must_use]
    pub fn adjoints(&self) -> &BTreeMap<ValueId, Cotangent> {
        &self.adjoints
    }

    /// Returns the requested differentiation variables.
    ///
    /// Their order matches the original request.
    #[must_use]
    pub fn requested_inputs(&self) -> &[ValueId] {
        &self.requested_inputs
    }

    /// Returns the gradient associated with a requested variable.
    ///
    /// Returns `None` if the variable has no derivative contribution.
    #[must_use]
    pub fn gradient_for(
        &self,
        variable: &ValueId,
    ) -> Option<&Cotangent> {
        self.adjoints.get(variable)
    }

    /// Returns the derivative rules used during propagation.
    ///
    /// Entries are ordered by reverse execution.
    #[must_use]
    pub fn rules_used(&self) -> &[ReverseRuleUse] {
        &self.rules_used
    }

    /// Returns the number of derivative rules executed.
    #[must_use]
    pub fn executed_rule_count(&self) -> usize {
        self.rules_used.len()
    }
}


// ============================================================================
// REVERSE-MODE ERRORS
// ============================================================================

/// Failures produced by reverse-mode orchestration.
#[derive(Debug)]
#[non_exhaustive]
pub enum ReverseModeError<ProviderError> {

    /// The reverse execution trace is structurally invalid.
    InvalidTape(ReverseTapeError),

    /// No output cotangent seeds were supplied.
    EmptyOutputSeeds,

    /// An output seed refers to a value unknown to the execution trace.
    UnknownOutputSeed,

    /// No differentiation variables were requested.
    EmptyRequestedInputs,

    /// A requested differentiation variable is not an external input.
    RequestedInputNotExternal,

    /// The same differentiation variable was requested more than once.
    DuplicateRequestedInput,

    /// The differentiation rule registry failed.
    RuleRegistry(DifferentiationRuleError),

    /// No reverse-mode differentiation rule exists.
    RuleNotFound {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Computational domain.
        domain: DifferentiationDomain,
    },

    /// Multiple rules matched the request.
    AmbiguousRules {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Candidate rule identities.
        candidates: Vec<DifferentiationRuleId>,
    },

    /// A matching rule did not satisfy the verification policy.
    VerificationRequired {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,
    },

    /// The exact requested rule version was unavailable.
    VersionUnavailable {
        /// Operation whose derivative was requested.
        operation: DifferentiableOperationId,

        /// Requested version.
        version: DifferentiationRuleVersion,
    },

    /// The derivative provider failed.
    Provider(ProviderError),

    /// The provider returned an incorrect number of input contributions.
    ProviderArityMismatch {
        /// Original forward operation index.
        step_index: usize,

        /// Number of operation inputs.
        expected: usize,

        /// Number of contributions returned.
        actual: usize,
    },
}

impl<ProviderError> fmt::Display
    for ReverseModeError<ProviderError>
where
    ProviderError: fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidTape(error) => {
                write!(
                    formatter,
                    "invalid reverse execution trace: {error}"
                )
            }

            Self::EmptyOutputSeeds => {
                write!(
                    formatter,
                    "reverse differentiation requires output cotangent seeds"
                )
            }

            Self::UnknownOutputSeed => {
                write!(
                    formatter,
                    "an output cotangent seed references an unknown value"
                )
            }

            Self::EmptyRequestedInputs => {
                write!(
                    formatter,
                    "reverse differentiation requires at least one requested input"
                )
            }

            Self::RequestedInputNotExternal => {
                write!(
                    formatter,
                    "a requested differentiation variable is not an external trace input"
                )
            }

            Self::DuplicateRequestedInput => {
                write!(
                    formatter,
                    "a differentiation variable was requested more than once"
                )
            }

            Self::RuleRegistry(error) => {
                write!(
                    formatter,
                    "reverse-mode differentiation registry failed: {error}"
                )
            }

            Self::RuleNotFound {
                operation,
                domain,
            } => {
                write!(
                    formatter,
                    "no reverse differentiation rule exists for \
                     operation '{operation}' in domain '{domain:?}'"
                )
            }

            Self::AmbiguousRules {
                operation,
                candidates,
            } => {
                write!(
                    formatter,
                    "ambiguous reverse differentiation rules for \
                     operation '{operation}': {candidates:?}"
                )
            }

            Self::VerificationRequired { operation } => {
                write!(
                    formatter,
                    "reverse differentiation rule for '{operation}' \
                     does not satisfy the verification policy"
                )
            }

            Self::VersionUnavailable {
                operation,
                version,
            } => {
                write!(
                    formatter,
                    "reverse differentiation rule version '{version}' \
                     is unavailable for operation '{operation}'"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "reverse differentiation provider failed: {error}"
                )
            }

            Self::ProviderArityMismatch {
                step_index,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "reverse differentiation provider returned {actual} \
                     input contributions for operation at step {step_index}; \
                     expected {expected}"
                )
            }
        }
    }
}

impl<ProviderError> Error for ReverseModeError<ProviderError>
where
    ProviderError: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTape(error) => Some(error),
            Self::RuleRegistry(error) => Some(error),
            Self::Provider(error) => Some(error),

            Self::EmptyOutputSeeds
            | Self::UnknownOutputSeed
            | Self::EmptyRequestedInputs
            | Self::RequestedInputNotExternal
            | Self::DuplicateRequestedInput
            | Self::RuleNotFound { .. }
            | Self::AmbiguousRules { .. }
            | Self::VerificationRequired { .. }
            | Self::VersionUnavailable { .. }
            | Self::ProviderArityMismatch { .. } => None,
        }
    }
}


// ============================================================================
// REVERSE-MODE ENGINE
// ============================================================================

/// Target-independent reverse-mode differentiation coordinator.
///
/// The engine:
///
/// 1. Validates the execution trace.
/// 2. Validates output cotangent seeds.
/// 3. Validates requested differentiation inputs.
/// 4. Traverses operations in reverse execution order.
/// 5. Resolves reverse differentiation rules.
/// 6. Enforces the configured verification policy.
/// 7. Delegates vector-Jacobian products to the provider.
/// 8. Accumulates contributions to shared values.
/// 9. Returns the requested input gradients and execution metadata.
///
/// The engine does not:
///
/// - Build another computational graph.
/// - Perform tensor arithmetic.
/// - Store primal tensor values.
/// - Allocate physical device memory.
/// - Select hardware.
/// - Schedule distributed work.
///
/// It shares the canonical differentiation rule registry.
#[derive(Clone, Debug)]
pub struct ReverseModeEngine {

    registry: DifferentiationRuleRegistry,

    domain: DifferentiationDomain,

    require_verified_rules: bool,
}

impl ReverseModeEngine {

    /// Creates a reverse-mode engine.
    ///
    /// The registry and computational domain must be supplied by the
    /// surrounding AI context or integration layer.
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

    /// Requires all resolved derivative rules to be verified.
    ///
    /// This policy does not independently verify mathematical rules.
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

    /// Returns the shared differentiation rule registry.
    #[must_use]
    pub fn registry(&self) -> &DifferentiationRuleRegistry {
        &self.registry
    }

    /// Computes reverse-mode gradients for the supplied execution trace.
    ///
    /// `tape` contains operations in their original forward execution order.
    ///
    /// `output_seeds` maps output value identities to their initial
    /// cotangents.
    ///
    /// `requested_inputs` identifies the external inputs for which
    /// gradients are required.
    ///
    /// Multiple output seeds and multiple requested inputs are supported.
    ///
    /// A disconnected requested input is valid and has no gradient entry.
    ///
    /// The provider is responsible for retrieving primal values and
    /// implementing mathematically correct vector-Jacobian products.
    ///
    /// No implicit fallback to another differentiation strategy occurs.
    pub fn differentiate<ValueId, Cotangent, Provider>(
        &self,
        tape: &ReverseTape<ValueId>,
        output_seeds: BTreeMap<ValueId, Cotangent>,
        requested_inputs: &[ValueId],
        provider: &mut Provider,
    ) -> Result<
        ReverseModeResult<ValueId, Cotangent>,
        ReverseModeError<Provider::Error>,
    >
    where
        ValueId: Clone + Ord,
        Provider: ReverseModeExecutor<ValueId, Cotangent>,
    {

        // -------------------------------------------------------------
        // Validate output seeds.
        // -------------------------------------------------------------

        if output_seeds.is_empty() {
            return Err(
                ReverseModeError::EmptyOutputSeeds
            );
        }

        if output_seeds
            .keys()
            .any(|value| !tape.contains_value(value))
        {
            return Err(
                ReverseModeError::UnknownOutputSeed
            );
        }

        // -------------------------------------------------------------
        // Validate requested inputs.
        // -------------------------------------------------------------

        if requested_inputs.is_empty() {
            return Err(
                ReverseModeError::EmptyRequestedInputs
            );
        }

        let mut requested_set = BTreeSet::new();

        for input in requested_inputs {

            if !requested_set.insert(input.clone()) {
                return Err(
                    ReverseModeError::DuplicateRequestedInput
                );
            }

            if !tape.external_inputs.contains(input) {
                return Err(
                    ReverseModeError::RequestedInputNotExternal
                );
            }
        }

        // -------------------------------------------------------------
        // Initialize cotangent accumulation.
        // -------------------------------------------------------------

        let mut adjoints = output_seeds;

        let mut rules_used = Vec::new();

        // -------------------------------------------------------------
        // Reverse traversal.
        // -------------------------------------------------------------

        for (step_index, operation) in
            tape.operations.iter().enumerate().rev()
        {

            // Collect output cotangents in canonical output order.
            //
            // A missing cotangent is represented by None.
            //
            // An operation with no active output gradient does not
            // need derivative rule resolution or execution.

            let output_cotangents = operation
                .outputs
                .iter()
                .map(|output| adjoints.get(output))
                .collect::<Vec<_>>();

            if output_cotangents.iter().all(Option::is_none) {
                continue;
            }

            // ---------------------------------------------------------
            // Resolve the reverse-mode derivative rule.
            // ---------------------------------------------------------

            let mut request = DifferentiationRuleRequest::new(
                operation.operation.clone(),
                DifferentiationMode::Reverse,
                self.domain.clone(),
            );

            if let Some(version) = operation.rule_version {
                request = request.with_version(version);
            }

            if self.require_verified_rules {
                request = request.requiring_verified_rule();
            }

            let rule = match self
                .registry
                .resolve(&request)
                .map_err(ReverseModeError::RuleRegistry)?
            {

                DifferentiationRuleResolution::Resolved(rule) => {
                    rule
                }

                DifferentiationRuleResolution::NotFound => {
                    return Err(
                        ReverseModeError::RuleNotFound {
                            operation: operation.operation.clone(),
                            domain: self.domain.clone(),
                        }
                    );
                }

                DifferentiationRuleResolution::Ambiguous(candidates) => {
                    return Err(
                        ReverseModeError::AmbiguousRules {
                            operation: operation.operation.clone(),
                            candidates,
                        }
                    );
                }

                DifferentiationRuleResolution::VerificationRequired => {
                    return Err(
                        ReverseModeError::VerificationRequired {
                            operation: operation.operation.clone(),
                        }
                    );
                }

                DifferentiationRuleResolution::VersionUnavailable => {

                    return Err(
                        ReverseModeError::VersionUnavailable {
                            operation: operation.operation.clone(),
                            version: operation
                                .rule_version
                                .ok_or(
                                    ReverseModeError::RuleNotFound {
                                        operation: operation.operation.clone(),
                                        domain: self.domain.clone(),
                                    }
                                )?,
                        }
                    );
                }
            };

            // ---------------------------------------------------------
            // Execute vector-Jacobian product.
            // ---------------------------------------------------------

            let contributions = provider
                .apply_vjp(
                    &rule,
                    &operation.inputs,
                    &operation.outputs,
                    &output_cotangents,
                )
                .map_err(ReverseModeError::Provider)?;

            // ---------------------------------------------------------
            // Validate provider output arity.
            // ---------------------------------------------------------

            if contributions.len() != operation.inputs.len() {
                return Err(
                    ReverseModeError::ProviderArityMismatch {
                        step_index,
                        expected: operation.inputs.len(),
                        actual: contributions.len(),
                    }
                );
            }

            // ---------------------------------------------------------
            // Accumulate input cotangents.
            // ---------------------------------------------------------

            for (input, contribution) in
                operation.inputs.iter().zip(contributions.into_iter())
            {

                let Some(contribution) = contribution else {
                    continue;
                };

                match adjoints.get_mut(input) {

                    Some(existing) => {

                        provider
                            .accumulate(existing, contribution)
                            .map_err(ReverseModeError::Provider)?;

                    }

                    None => {

                        adjoints.insert(
                            input.clone(),
                            contribution,
                        );

                    }
                }
            }

            // Record successful derivative execution.
            rules_used.push(
                ReverseRuleUse::new(
                    step_index,
                    &rule,
                )
            );
        }

        Ok(
            ReverseModeResult::new(
                adjoints,
                requested_inputs.to_vec(),
                rules_used,
            )
        )
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use std::convert::Infallible;

    use crate::ai::differentiation::rules::{
        Differentiability,
        DifferentiationRegularity,
        DifferentiationRuleId,
        DifferentiationRuleProvenance,
        DerivativeOperationRef,
    };


    // ---------------------------------------------------------------------
    // Test provider
    // ---------------------------------------------------------------------

    #[derive(Debug)]
    struct TestProviderError;

    impl fmt::Display for TestProviderError {
        fn fmt(
            &self,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            write!(formatter, "test provider failure")
        }
    }

    impl Error for TestProviderError {}


    struct ScalarProvider {

        values: BTreeMap<&'static str, f64>,
    }

    impl ReverseModeExecutor<&'static str, f64>
        for ScalarProvider
    {
        type Error = TestProviderError;

        fn apply_vjp(
            &mut self,
            rule: &DifferentiationRule,
            inputs: &[&'static str],
            _outputs: &[&'static str],
            output_cotangents: &[Option<&f64>],
        ) -> Result<Vec<Option<f64>>, Self::Error> {

            let incoming = output_cotangents
                .iter()
                .filter_map(|value| *value)
                .copied()
                .sum::<f64>();

            match rule.operation().as_str() {

                "test::add" => {
                    Ok(
                        inputs
                            .iter()
                            .map(|_| Some(incoming))
                            .collect()
                    )
                }

                "test::multiply" => {

                    if inputs.len() != 2 {
                        return Err(TestProviderError);
                    }

                    let left = *self
                        .values
                        .get(inputs[0])
                        .ok_or(TestProviderError)?;

                    let right = *self
                        .values
                        .get(inputs[1])
                        .ok_or(TestProviderError)?;

                    Ok(vec![
                        Some(incoming * right),
                        Some(incoming * left),
                    ])
                }

                _ => Err(TestProviderError),
            }
        }

        fn accumulate(
            &mut self,
            accumulated: &mut f64,
            contribution: f64,
        ) -> Result<(), Self::Error> {

            *accumulated += contribution;

            Ok(())
        }
    }


    // ---------------------------------------------------------------------
    // Test rule construction
    // ---------------------------------------------------------------------

    fn make_rule(
        operation: &str,
        derivative: &str,
    ) -> DifferentiationRule {

        DifferentiationRule::new(
            DifferentiationRuleId::new(
                format!("test::rule::{operation}")
            )
            .unwrap(),

            DifferentiableOperationId::new(operation)
                .unwrap(),

            DerivativeOperationRef::new(derivative)
                .unwrap(),

            DifferentiationRuleVersion::new(1, 0, 0),

            vec![
                DifferentiationMode::Reverse,
            ],

            vec![
                DifferentiationDomain::Classical,
            ],

            Differentiability::Differentiable,

            DifferentiationRegularity::Smooth,

            DifferentiationRuleProvenance::new(
                "test::source",
                "test::authority",
                "Reverse differentiation test rule",
            )
            .unwrap(),
        )
        .unwrap()
    }


    fn registry() -> DifferentiationRuleRegistry {

        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            make_rule(
                "test::add",
                "test::add_vjp",
            )
        )
        .unwrap();

        registry.register(
            make_rule(
                "test::multiply",
                "test::multiply_vjp",
            )
        )
        .unwrap();

        registry
    }


    // ---------------------------------------------------------------------
    // Trace tests
    // ---------------------------------------------------------------------

    #[test]
    fn validates_forward_execution_order() {

        let operations = vec![
            ReverseOperation::new(
                DifferentiableOperationId::new("test::add").unwrap(),
                vec!["x", "y"],
                vec!["z"],
            )
        ];

        let tape = ReverseTape::new(operations).unwrap();

        assert_eq!(tape.operation_count(), 1);

        assert!(tape.contains_value(&"x"));
        assert!(tape.contains_value(&"y"));
        assert!(tape.contains_produced_value(&"z"));
    }


    #[test]
    fn rejects_empty_operation_outputs() {

        let operation = ReverseOperation::new(
            DifferentiableOperationId::new("test::add").unwrap(),
            vec!["x"],
            Vec::<&str>::new(),
        );

        let result = ReverseTape::new(vec![operation]);

        assert!(matches!(
            result,
            Err(ReverseTapeError::EmptyOperationOutputs { .. })
        ));
    }


    #[test]
    fn rejects_duplicate_output_definitions() {

        let operations = vec![
            ReverseOperation::new(
                DifferentiableOperationId::new("test::add").unwrap(),
                vec!["x", "y"],
                vec!["z"],
            ),

            ReverseOperation::new(
                DifferentiableOperationId::new("test::add").unwrap(),
                vec!["z", "x"],
                vec!["z"],
            ),
        ];

        assert!(matches!(
            ReverseTape::new(operations),
            Err(ReverseTapeError::DuplicateValueDefinition { .. })
        ));
    }


    #[test]
    fn rejects_forward_references() {

        let operations = vec![
            ReverseOperation::new(
                DifferentiableOperationId::new("test::add").unwrap(),
                vec!["future", "x"],
                vec!["first"],
            ),

            ReverseOperation::new(
                DifferentiableOperationId::new("test::add").unwrap(),
                vec!["x", "y"],
                vec!["future"],
            ),
        ];

        assert!(matches!(
            ReverseTape::new(operations),
            Err(ReverseTapeError::NonTopologicalInput { .. })
        ));
    }


    // ---------------------------------------------------------------------
    // Reverse propagation tests
    // ---------------------------------------------------------------------

    #[test]
    fn propagates_gradient_through_operation_chain() {

        // z = x * y
        // result = z + x
        //
        // d(result)/dx = y + 1
        // d(result)/dy = x

        let tape = ReverseTape::new(vec![

            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::multiply"
                ).unwrap(),
                vec!["x", "y"],
                vec!["z"],
            ),

            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["z", "x"],
                vec!["result"],
            ),

        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("result", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::from([
                ("x", 3.0),
                ("y", 4.0),
                ("z", 12.0),
                ("result", 15.0),
            ]),
        };

        let result = engine
            .differentiate(
                &tape,
                seeds,
                &["x", "y"],
                &mut provider,
            )
            .unwrap();

        assert_eq!(
            result.gradient_for(&"x"),
            Some(&5.0)
        );

        assert_eq!(
            result.gradient_for(&"y"),
            Some(&3.0)
        );

        assert_eq!(
            result.executed_rule_count(),
            2
        );
    }


    #[test]
    fn accumulates_contributions_from_multiple_paths() {

        // z = x + x
        //
        // dz/dx = 2

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "x"],
                vec!["z"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("z", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine
            .differentiate(
                &tape,
                seeds,
                &["x"],
                &mut provider,
            )
            .unwrap();

        assert_eq!(
            result.gradient_for(&"x"),
            Some(&2.0)
        );
    }


    #[test]
    fn supports_multiple_output_seeds() {

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "y"],
                vec!["a", "b"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("a", 2.0);
        seeds.insert("b", 3.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine
            .differentiate(
                &tape,
                seeds,
                &["x", "y"],
                &mut provider,
            )
            .unwrap();

        assert_eq!(
            result.gradient_for(&"x"),
            Some(&5.0)
        );

        assert_eq!(
            result.gradient_for(&"y"),
            Some(&5.0)
        );
    }


    #[test]
    fn disconnected_input_has_no_gradient() {

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "y"],
                vec!["z"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("z", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine
            .differentiate(
                &tape,
                seeds,
                &["x", "unused"],
                &mut provider,
            );

        assert!(matches!(
            result,
            Err(ReverseModeError::RequestedInputNotExternal)
        ));
    }


    // ---------------------------------------------------------------------
    // Failure and policy tests
    // ---------------------------------------------------------------------

    #[test]
    fn rejects_empty_output_seeds() {

        let tape = ReverseTape::<&str>::new(Vec::new()).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine.differentiate(
            &tape,
            BTreeMap::new(),
            &["x"],
            &mut provider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::EmptyOutputSeeds)
        ));
    }


    #[test]
    fn rejects_unknown_output_seed() {

        let tape = ReverseTape::<&str>::new(Vec::new()).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("unknown", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine.differentiate(
            &tape,
            seeds,
            &["x"],
            &mut provider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::UnknownOutputSeed)
        ));
    }


    #[test]
    fn fails_when_reverse_rule_is_missing() {

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::unknown"
                ).unwrap(),
                vec!["x"],
                vec!["y"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            DifferentiationRuleRegistry::new(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("y", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine.differentiate(
            &tape,
            seeds,
            &["x"],
            &mut provider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::RuleNotFound { .. })
        ));
    }


    #[test]
    fn respects_exact_rule_version() {

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "y"],
                vec!["z"],
            )
            .with_rule_version(
                DifferentiationRuleVersion::new(2, 0, 0)
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("z", 1.0);

        let mut provider = ScalarProvider {
            values: BTreeMap::new(),
        };

        let result = engine.differentiate(
            &tape,
            seeds,
            &["x"],
            &mut provider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::VersionUnavailable { .. })
        ));
    }


    #[test]
    fn provider_errors_are_not_reported_as_success() {

        struct FailingProvider;

        impl ReverseModeExecutor<&'static str, f64>
            for FailingProvider
        {
            type Error = TestProviderError;

            fn apply_vjp(
                &mut self,
                _rule: &DifferentiationRule,
                _inputs: &[&'static str],
                _outputs: &[&'static str],
                _cotangents: &[Option<&f64>],
            ) -> Result<Vec<Option<f64>>, Self::Error> {
                Err(TestProviderError)
            }

            fn accumulate(
                &mut self,
                _accumulated: &mut f64,
                _contribution: f64,
            ) -> Result<(), Self::Error> {
                Err(TestProviderError)
            }
        }

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "y"],
                vec!["z"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("z", 1.0);

        let result = engine.differentiate(
            &tape,
            seeds,
            &["x"],
            &mut FailingProvider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::Provider(_))
        ));
    }


    #[test]
    fn provider_arity_mismatch_is_rejected() {

        struct InvalidProvider;

        impl ReverseModeExecutor<&'static str, f64>
            for InvalidProvider
        {
            type Error = Infallible;

            fn apply_vjp(
                &mut self,
                _rule: &DifferentiationRule,
                _inputs: &[&'static str],
                _outputs: &[&'static str],
                _cotangents: &[Option<&f64>],
            ) -> Result<Vec<Option<f64>>, Self::Error> {
                Ok(Vec::new())
            }

            fn accumulate(
                &mut self,
                _accumulated: &mut f64,
                _contribution: f64,
            ) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        let tape = ReverseTape::new(vec![
            ReverseOperation::new(
                DifferentiableOperationId::new(
                    "test::add"
                ).unwrap(),
                vec!["x", "y"],
                vec!["z"],
            )
        ]).unwrap();

        let engine = ReverseModeEngine::new(
            registry(),
            DifferentiationDomain::Classical,
        );

        let mut seeds = BTreeMap::new();
        seeds.insert("z", 1.0);

        let result = engine.differentiate(
            &tape,
            seeds,
            &["x"],
            &mut InvalidProvider,
        );

        assert!(matches!(
            result,
            Err(ReverseModeError::ProviderArityMismatch {
                expected: 2,
                actual: 0,
                ..
            })
        ));
    }
}
