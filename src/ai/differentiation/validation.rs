#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Differentiation Validation
//!
//! Path: `src/ai/differentiation/validation.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only; no `unsafe`.
//!
//! ## Purpose
//!
//! Validate differentiation requests and differentiation dependency graphs
//! before differentiation is lowered or executed.
//!
//! This module distinguishes:
//!
//! - structural graph validity;
//! - operation differentiability;
//! - derivative-rule availability;
//! - request validity;
//! - dependency validity;
//! - unresolved semantic information;
//! - resource and target feasibility.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - differentiation validation policy;
//! - validation issue representation;
//! - graph validation coordination;
//! - differentiation-request consistency checks;
//! - operation-level differentiability checks through a supplied resolver;
//! - deterministic validation reporting;
//! - explicit completeness status.
//!
//! It does not own:
//!
//! - source parsing or AST construction;
//! - canonical semantic types;
//! - canonical model graph construction;
//! - differentiation rule registration;
//! - derivative graph construction;
//! - tensor storage or tensor kernels;
//! - ZUIR construction;
//! - hardware selection;
//! - resource allocation;
//! - scheduling or runtime execution.
//!
//! ## Integration
//!
//! `graph.rs` supplies `DifferentiationGraph` and its structural validation.
//! `rules.rs` supplies differentiation rule identities and contracts.
//! `learning/gradients.rs` supplies gradient-request semantics.
//! `frontend/semantic/domains/ai.rs` remains the AI semantic authority.
//! ZUIR remains the canonical computational IR.
//!
//! The caller adapts canonical semantic facts to `DifferentiationValidationInput`
//! and provides a `DifferentiationOperationResolver` backed by the canonical
//! operation and differentiation-rule registries.
//!
//! This avoids creating a second semantic type system or operation registry.
//!
//! ## Scalability
//!
//! No fixed language-level limit is imposed on graph nodes, graph edges,
//! derivative order, variables, outputs, or operation count.
//!
//! The implementation is bounded by available memory, the chosen identifier
//! representations, and the host's representational limits.
//!
//! Operational limits must be supplied through validation policy. They are
//! not language-level semantic ceilings.
//!
//! ## Completeness
//!
//! A validation result is not considered complete if required semantic
//! information was unavailable or a configured analysis budget was exhausted.
//! Incomplete validation must never be interpreted as proof of validity.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::hash::Hash;

use crate::ai::differentiation::graph::{
    DifferentiationGraph,
    DifferentiationNodeId,
    DifferentiationOperationId,
    DifferentiationNodeKind,
    DifferentiationGraphError,
};
use crate::ai::differentiation::rules::{
    DifferentiableOperationId,
    DifferentiationMode,
};
use crate::ai::learning::gradients::GradientResultKind;

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Controls the strictness and operational behavior of differentiation
/// validation.
///
/// This policy does not define language semantics. It determines how the
/// validator responds when information is unavailable or when a caller's
/// operational budget is reached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationValidationPolicy {
    /// Whether unresolved operation semantics make validation incomplete.
    pub require_resolved_operations: bool,

    /// Whether a derivative rule must be resolved for each differentiable
    /// operation.
    pub require_derivative_rules: bool,

    /// Whether conditionally differentiable operations require their
    /// conditions to be established.
    pub require_conditions_established: bool,

    /// Whether unknown or extension-defined semantics are rejected.
    ///
    /// When false, unknown semantics are reported as unresolved rather than
    /// accepted as valid.
    pub reject_unknown_semantics: bool,

    /// Optional operational work budget.
    ///
    /// `None` means that this validator does not impose a local work budget.
    /// A caller may still enforce a session-wide or compiler-wide budget.
    pub work_budget: Option<u64>,
}

impl Default for DifferentiationValidationPolicy {
    fn default() -> Self {
        Self {
            require_resolved_operations: true,
            require_derivative_rules: true,
            require_conditions_established: true,
            reject_unknown_semantics: false,
            work_budget: None,
        }
    }
}

// ============================================================================
// VALIDATION IDENTIFIERS
// ============================================================================

/// Stable machine-readable differentiation validation issue code.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationValidationCode {
    /// The graph contains a structural error.
    GraphStructure,

    /// The graph contains a cycle.
    GraphCycle,

    /// A graph node refers to an invalid operation.
    InvalidOperation,

    /// An operation is not differentiable under the requested semantics.
    NotDifferentiable,

    /// Differentiability depends on a condition that is not established.
    UnresolvedCondition,

    /// Operation semantics could not be resolved.
    UnresolvedOperation,

    /// A derivative rule is not available.
    MissingDerivativeRule,

    /// The operation has no derivative rule for the requested mode.
    UnsupportedMode,

    /// A differentiation variable is missing or invalid.
    InvalidVariable,

    /// A requested output is missing or invalid.
    InvalidOutput,

    /// The differentiation request is internally inconsistent.
    InvalidRequest,

    /// The requested derivative result is incompatible with the request.
    IncompatibleDerivative,

    /// The analysis could not be completed.
    IncompleteAnalysis,

    /// An operational validation budget was exhausted.
    BudgetExhausted,

    /// An extension-defined validation issue.
    Extension,
}

impl fmt::Display for DifferentiationValidationCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::GraphStructure => "differentiation.graph.structure",
            Self::GraphCycle => "differentiation.graph.cycle",
            Self::InvalidOperation => "differentiation.operation.invalid",
            Self::NotDifferentiable => "differentiation.operation.not_differentiable",
            Self::UnresolvedCondition => "differentiation.condition.unresolved",
            Self::UnresolvedOperation => "differentiation.operation.unresolved",
            Self::MissingDerivativeRule => "differentiation.rule.missing",
            Self::UnsupportedMode => "differentiation.mode.unsupported",
            Self::InvalidVariable => "differentiation.variable.invalid",
            Self::InvalidOutput => "differentiation.output.invalid",
            Self::InvalidRequest => "differentiation.request.invalid",
            Self::IncompatibleDerivative => "differentiation.result.incompatible",
            Self::IncompleteAnalysis => "differentiation.analysis.incomplete",
            Self::BudgetExhausted => "differentiation.analysis.budget_exhausted",
            Self::Extension => "differentiation.extension",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// VALIDATION ISSUES
// ============================================================================

/// Severity of a differentiation validation issue.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub enum DifferentiationValidationSeverity {
    /// The request cannot be considered valid.
    Error,

    /// The issue prevents a complete validation conclusion.
    Incomplete,

    /// The request may be valid, but the issue should be surfaced.
    Warning,

    /// Informational validation detail.
    Information,
}

/// A single deterministic differentiation validation issue.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DifferentiationValidationIssue {
    code: DifferentiationValidationCode,
    severity: DifferentiationValidationSeverity,
    subject: Option<String>,
    message: String,
    details: BTreeMap<String, String>,
}

impl DifferentiationValidationIssue {
    /// Creates a validation issue.
    pub fn new(
        code: DifferentiationValidationCode,
        severity: DifferentiationValidationSeverity,
        subject: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            subject,
            message: message.into(),
            details: BTreeMap::new(),
        }
    }

    /// Adds deterministic structured detail to the issue.
    #[must_use]
    pub fn with_detail(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    /// Returns the issue code.
    #[must_use]
    pub const fn code(&self) -> DifferentiationValidationCode {
        self.code
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> DifferentiationValidationSeverity {
        self.severity
    }

    /// Returns the subject identity, if available.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Returns the issue message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured details.
    #[must_use]
    pub fn details(&self) -> &BTreeMap<String, String> {
        &self.details
    }
}

// ============================================================================
// VALIDATION COMPLETENESS
// ============================================================================

/// Completeness state of a differentiation validation run.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationValidationCompleteness {
    /// All requested checks completed with the available information.
    Complete,

    /// Some required semantic information was not available.
    Incomplete,

    /// A caller-supplied operational budget prevented completion.
    BudgetExhausted,
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic result of differentiation validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationValidationReport {
    completeness: DifferentiationValidationCompleteness,
    issues: Vec<DifferentiationValidationIssue>,
    checked_operations: u64,
    checked_dependencies: u64,
}

impl DifferentiationValidationReport {
    /// Returns the validation completeness state.
    #[must_use]
    pub const fn completeness(
        &self,
    ) -> DifferentiationValidationCompleteness {
        self.completeness
    }

    /// Returns all issues in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[DifferentiationValidationIssue] {
        &self.issues
    }

    /// Returns whether no error-level issue was reported and validation
    /// completed.
    ///
    /// This is a validation result, not proof of resource feasibility or
    /// successful runtime execution.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.completeness == DifferentiationValidationCompleteness::Complete
            && !self.issues.iter().any(|issue| {
                issue.severity == DifferentiationValidationSeverity::Error
            })
    }

    /// Returns whether validation completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completeness == DifferentiationValidationCompleteness::Complete
    }

    /// Returns the number of operation checks performed.
    #[must_use]
    pub const fn checked_operations(&self) -> u64 {
        self.checked_operations
    }

    /// Returns the number of dependency checks performed.
    #[must_use]
    pub const fn checked_dependencies(&self) -> u64 {
        self.checked_dependencies
    }
}

// ============================================================================
// DIFFERENTIABILITY SEMANTICS
// ============================================================================

/// Operation differentiability status resolved from canonical semantic
/// information and the differentiation-rule registry.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OperationDifferentiability {
    /// The operation is differentiable for the established input domain.
    Differentiable,

    /// The operation is differentiable only when the supplied condition
    /// has been established.
    Conditional {
        /// Condition identity or explanatory reference.
        condition: String,

        /// Whether the condition has been established by semantic analysis.
        established: bool,
    },

    /// The operation has a generalized or almost-everywhere derivative.
    Generalized,

    /// The operation is not differentiable for the requested semantics.
    NotDifferentiable,

    /// The operation's differentiability is not yet known.
    Unknown,
}

/// Result of resolving operation differentiation semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationDifferentiabilityInfo {
    /// Mathematical differentiability status.
    pub differentiability: OperationDifferentiability,

    /// Whether a derivative rule is available for the requested mode.
    pub derivative_rule_available: bool,

    /// Optional stable identity of the selected derivative rule.
    pub derivative_rule_identity: Option<String>,

    /// Optional explanation or provenance.
    pub explanation: Option<String>,
}

impl OperationDifferentiabilityInfo {
    /// Creates an unconditional differentiability result.
    #[must_use]
    pub fn differentiable(rule_identity: impl Into<String>) -> Self {
        Self {
            differentiability: OperationDifferentiability::Differentiable,
            derivative_rule_available: true,
            derivative_rule_identity: Some(rule_identity.into()),
            explanation: None,
        }
    }

    /// Creates an unknown result.
    #[must_use]
    pub fn unknown(explanation: impl Into<String>) -> Self {
        Self {
            differentiability: OperationDifferentiability::Unknown,
            derivative_rule_available: false,
            derivative_rule_identity: None,
            explanation: Some(explanation.into()),
        }
    }
}

/// Resolves operation semantics through the canonical operation and
/// differentiation-rule registries.
///
/// Implementations must not report a rule as available merely because an
/// operation is known. The rule must apply to the requested mode and
/// semantic domain.
pub trait DifferentiationOperationResolver<OperationId> {
    /// Resolves differentiability and derivative-rule availability.
    fn resolve(
        &self,
        operation: &OperationId,
        mode: &DifferentiationMode,
    ) -> OperationDifferentiabilityInfo;
}

// ============================================================================
// VALIDATION INPUT
// ============================================================================

/// One requested differentiation variable.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DifferentiationVariable<NodeId> {
    /// Graph node that represents the independent variable.
    pub node: NodeId,

    /// Optional semantic identity of the variable.
    pub semantic_identity: Option<String>,
}

/// One requested differentiation output.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DifferentiationOutput<NodeId> {
    /// Graph node whose derivative is requested.
    pub node: NodeId,

    /// Optional semantic identity of the output.
    pub semantic_identity: Option<String>,
}

/// A differentiation validation request.
///
/// This is a validation boundary record, not a replacement for the
/// canonical gradient request. The frontend or gradient adapter maps its
/// canonical request into this structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationValidationInput<NodeId> {
    /// Requested differentiation strategy.
    pub mode: DifferentiationMode,

    /// Mathematical derivative result kind.
    pub result_kind: GradientResultKind,

    /// Independent variables.
    pub variables: Vec<DifferentiationVariable<NodeId>>,

    /// Requested outputs.
    pub outputs: Vec<DifferentiationOutput<NodeId>>,

    /// Derivative order as a symbolic identity.
    ///
    /// This is deliberately not restricted to a machine-sized integer.
    /// The adapter must separately establish whether the order is known and
    /// valid for the request.
    pub derivative_order: Option<String>,
}

// ============================================================================
// VALIDATION ERROR
// ============================================================================

/// Fatal failure to execute the validation procedure itself.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DifferentiationValidationError {
    /// The graph's structural validation failed.
    Graph(String),

    /// A validation input was malformed.
    InvalidInput(String),

    /// A validation counter exceeded its representational range.
    CounterOverflow,
}

impl fmt::Display for DifferentiationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Graph(message) => {
                write!(formatter, "differentiation graph validation failed: {message}")
            }
            Self::InvalidInput(message) => {
                write!(formatter, "invalid differentiation validation input: {message}")
            }
            Self::CounterOverflow => {
                formatter.write_str("differentiation validation counter overflow")
            }
        }
    }
}

impl Error for DifferentiationValidationError {}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Coordinates differentiation validation.
#[derive(Clone, Debug)]
pub struct DifferentiationValidator {
    policy: DifferentiationValidationPolicy,
}

impl Default for DifferentiationValidator {
    fn default() -> Self {
        Self::new(DifferentiationValidationPolicy::default())
    }
}

impl DifferentiationValidator {
    /// Creates a validator with the supplied policy.
    #[must_use]
    pub fn new(policy: DifferentiationValidationPolicy) -> Self {
        Self { policy }
    }

    /// Returns the validator policy.
    #[must_use]
    pub fn policy(&self) -> &DifferentiationValidationPolicy {
        &self.policy
    }

    /// Validates graph structure and the differentiation request.
    ///
    /// Graph structure is checked first. A structurally invalid graph cannot
    /// be used as the basis for operation-level validation.
    pub fn validate<NodeId, OperationId, Resolver>(
        &self,
        graph: &DifferentiationGraph<NodeId, OperationId>,
        request: &DifferentiationValidationInput<NodeId>,
        resolver: &Resolver,
    ) -> Result<DifferentiationValidationReport, DifferentiationValidationError>
    where
        NodeId: DifferentiationNodeId,
        OperationId: DifferentiationOperationId,
        Resolver: DifferentiationOperationResolver<OperationId>,
    {
        let structural = graph
            .validate()
            .map_err(|error| {
                DifferentiationValidationError::Graph(error.to_string())
            })?;

        let mut state = ValidationState::new(&self.policy);

        state.validate_request(graph, request)?;

        let mut operation_count = 0_u64;

        for node in graph.nodes() {
            state.consume_work()?;

            if let DifferentiationNodeKind::Operation(operation) = node.kind() {
                operation_count = operation_count
                    .checked_add(1)
                    .ok_or(DifferentiationValidationError::CounterOverflow)?;

                state.validate_operation(operation, resolver, &request.mode);
            }
        }

        let dependency_count = u64::try_from(structural.edge_count())
            .map_err(|_| DifferentiationValidationError::CounterOverflow)?;

        state.checked_dependencies = dependency_count;
        state.checked_operations = operation_count;

        Ok(state.finish())
    }
}

// ============================================================================
// INTERNAL VALIDATION STATE
// ============================================================================

struct ValidationState<'a> {
    policy: &'a DifferentiationValidationPolicy,
    issues: Vec<DifferentiationValidationIssue>,
    completeness: DifferentiationValidationCompleteness,
    work_consumed: u64,
    checked_operations: u64,
    checked_dependencies: u64,
}

impl<'a> ValidationState<'a> {
    fn new(policy: &'a DifferentiationValidationPolicy) -> Self {
        Self {
            policy,
            issues: Vec::new(),
            completeness: DifferentiationValidationCompleteness::Complete,
            work_consumed: 0,
            checked_operations: 0,
            checked_dependencies: 0,
        }
    }

    fn consume_work(
        &mut self,
    ) -> Result<(), DifferentiationValidationError> {
        self.work_consumed = self
            .work_consumed
            .checked_add(1)
            .ok_or(DifferentiationValidationError::CounterOverflow)?;

        if let Some(budget) = self.policy.work_budget {
            if self.work_consumed > budget {
                self.completeness =
                    DifferentiationValidationCompleteness::BudgetExhausted;

                self.issues.push(
                    DifferentiationValidationIssue::new(
                        DifferentiationValidationCode::BudgetExhausted,
                        DifferentiationValidationSeverity::Incomplete,
                        None,
                        "Differentiation validation exhausted its configured work budget",
                    ),
                );

                return Err(
                    DifferentiationValidationError::InvalidInput(
                        "validation budget exhausted".to_owned(),
                    ),
                );
            }
        }

        Ok(())
    }

    fn validate_request<NodeId, OperationId>(
        &mut self,
        graph: &DifferentiationGraph<NodeId, OperationId>,
        request: &DifferentiationValidationInput<NodeId>,
    ) -> Result<(), DifferentiationValidationError>
    where
        NodeId: DifferentiationNodeId,
        OperationId: DifferentiationOperationId,
    {
        if request.variables.is_empty() {
            self.push_error(
                DifferentiationValidationCode::InvalidVariable,
                None,
                "The differentiation request has no independent variables",
            );
        }

        if request.outputs.is_empty() {
            self.push_error(
                DifferentiationValidationCode::InvalidOutput,
                None,
                "The differentiation request has no requested outputs",
            );
        }

        let mut variables = BTreeSet::new();

        for variable in &request.variables {
            self.consume_work()?;

            if !graph.contains_node(&variable.node) {
                self.push_error(
                    DifferentiationValidationCode::InvalidVariable,
                    Some(variable.node.to_string()),
                    "A differentiation variable does not exist in the graph",
                );
            }

            if !variables.insert(variable.node.clone()) {
                self.push_error(
                    DifferentiationValidationCode::InvalidVariable,
                    Some(variable.node.to_string()),
                    "The differentiation request repeats an independent variable",
                );
            }
        }

        let mut outputs = BTreeSet::new();

        for output in &request.outputs {
            self.consume_work()?;

            if !graph.contains_node(&output.node) {
                self.push_error(
                    DifferentiationValidationCode::InvalidOutput,
                    Some(output.node.to_string()),
                    "A requested differentiation output does not exist in the graph",
                );
            }

            if !outputs.insert(output.node.clone()) {
                self.push_error(
                    DifferentiationValidationCode::InvalidOutput,
                    Some(output.node.to_string()),
                    "The differentiation request repeats an output",
                );
            }
        }

        if let Some(order) = &request.derivative_order {
            if order.trim().is_empty()
                || order.chars().any(char::is_control)
            {
                self.push_error(
                    DifferentiationValidationCode::InvalidRequest,
                    None,
                    "The derivative order reference is malformed",
                );
            }
        }

        request.result_kind.validate().map_err(|error| {
            DifferentiationValidationError::InvalidInput(error.to_string())
        })?;

        Ok(())
    }

    fn validate_operation<OperationId, Resolver>(
        &mut self,
        operation: &OperationId,
        resolver: &Resolver,
        mode: &DifferentiationMode,
    )
    where
        OperationId: DifferentiationOperationId,
        Resolver: DifferentiationOperationResolver<OperationId>,
    {
        let info = resolver.resolve(operation, mode);

        match info.differentiability {
            OperationDifferentiability::Differentiable
            | OperationDifferentiability::Generalized => {
                if self.policy.require_derivative_rules
                    && !info.derivative_rule_available
                {
                    self.push_error(
                        DifferentiationValidationCode::MissingDerivativeRule,
                        Some(operation.to_string()),
                        info.explanation.unwrap_or_else(|| {
                            "No derivative rule is available for this operation and mode"
                                .to_owned()
                        }),
                    );
                }
            }

            OperationDifferentiability::Conditional {
                condition,
                established,
            } => {
                if self.policy.require_conditions_established && !established {
                    self.push_error(
                        DifferentiationValidationCode::UnresolvedCondition,
                        Some(operation.to_string()),
                        format!(
                            "Differentiability condition '{condition}' has not been established"
                        ),
                    );
                }

                if self.policy.require_derivative_rules
                    && !info.derivative_rule_available
                {
                    self.push_error(
                        DifferentiationValidationCode::MissingDerivativeRule,
                        Some(operation.to_string()),
                        "No derivative rule is available for the conditional operation",
                    );
                }
            }

            OperationDifferentiability::NotDifferentiable => {
                self.push_error(
                    DifferentiationValidationCode::NotDifferentiable,
                    Some(operation.to_string()),
                    info.explanation.unwrap_or_else(|| {
                        "The operation is not differentiable under the requested semantics"
                            .to_owned()
                    }),
                );
            }

            OperationDifferentiability::Unknown => {
                if self.policy.reject_unknown_semantics {
                    self.push_error(
                        DifferentiationValidationCode::UnresolvedOperation,
                        Some(operation.to_string()),
                        info.explanation.unwrap_or_else(|| {
                            "The operation's differentiability semantics are unresolved"
                                .to_owned()
                        }),
                    );
                } else {
                    self.push_incomplete(
                        DifferentiationValidationCode::UnresolvedOperation,
                        Some(operation.to_string()),
                        info.explanation.unwrap_or_else(|| {
                            "The operation's differentiability semantics are unresolved"
                                .to_owned()
                        }),
                    );
                }
            }
        }
    }

    fn push_error(
        &mut self,
        code: DifferentiationValidationCode,
        subject: Option<String>,
        message: impl Into<String>,
    ) {
        self.issues.push(
            DifferentiationValidationIssue::new(
                code,
                DifferentiationValidationSeverity::Error,
                subject,
                message,
            ),
        );
    }

    fn push_incomplete(
        &mut self,
        code: DifferentiationValidationCode,
        subject: Option<String>,
        message: impl Into<String>,
    ) {
        self.completeness = DifferentiationValidationCompleteness::Incomplete;

        self.issues.push(
            DifferentiationValidationIssue::new(
                code,
                DifferentiationValidationSeverity::Incomplete,
                subject,
                message,
            ),
        );
    }

    fn finish(mut self) -> DifferentiationValidationReport {
        self.issues.sort();
        self.issues.dedup();

        DifferentiationValidationReport {
            completeness: self.completeness,
            issues: self.issues,
            checked_operations: self.checked_operations,
            checked_dependencies: self.checked_dependencies,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::differentiation::graph::{
        DifferentiationDependencyKind,
        DifferentiationEdge,
        DifferentiationNode,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    struct TestNodeId(&'static str);

    impl fmt::Display for TestNodeId {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.0)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    struct TestOperationId(&'static str);

    impl fmt::Display for TestOperationId {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.0)
        }
    }

    struct TestResolver {
        info: OperationDifferentiabilityInfo,
    }

    impl DifferentiationOperationResolver<TestOperationId> for TestResolver {
        fn resolve(
            &self,
            _operation: &TestOperationId,
            _mode: &DifferentiationMode,
        ) -> OperationDifferentiabilityInfo {
            self.info.clone()
        }
    }

    fn sample_graph(
    ) -> DifferentiationGraph<TestNodeId, TestOperationId> {
        let mut graph = DifferentiationGraph::new();

        graph.add_node(
            DifferentiationNode::new(
                TestNodeId("x"),
                DifferentiationNodeKind::Input,
            ),
        ).expect("insert input");

        graph.add_node(
            DifferentiationNode::new(
                TestNodeId("y"),
                DifferentiationNodeKind::Operation(
                    TestOperationId("op.multiply"),
                ),
            ),
        ).expect("insert operation");

        graph.add_dependency(
            TestNodeId("x"),
            TestNodeId("y"),
            DifferentiationDependencyKind::Value,
        ).expect("insert dependency");

        graph
    }

    fn sample_request() -> DifferentiationValidationInput<TestNodeId> {
        DifferentiationValidationInput {
            mode: DifferentiationMode::Reverse,
            result_kind: GradientResultKind::Gradient,
            variables: vec![
                DifferentiationVariable {
                    node: TestNodeId("x"),
                    semantic_identity: None,
                },
            ],
            outputs: vec![
                DifferentiationOutput {
                    node: TestNodeId("y"),
                    semantic_identity: None,
                },
            ],
            derivative_order: Some("1".to_owned()),
        }
    }

    #[test]
    fn validates_differentiable_graph() {
        let validator = DifferentiationValidator::default();
        let resolver = TestResolver {
            info: OperationDifferentiabilityInfo::differentiable(
                "rule.multiply.reverse",
            ),
        };

        let report = validator
            .validate(&sample_graph(), &sample_request(), &resolver)
            .expect("validation succeeds");

        assert!(report.is_valid());
        assert!(report.is_complete());
    }

    #[test]
    fn unresolved_operation_is_incomplete_not_valid() {
        let validator = DifferentiationValidator::default();
        let resolver = TestResolver {
            info: OperationDifferentiabilityInfo::unknown(
                "operation semantics unavailable",
            ),
        };

        let report = validator
            .validate(&sample_graph(), &sample_request(), &resolver)
            .expect("validation report");

        assert!(!report.is_valid());
        assert!(!report.is_complete());
    }

    #[test]
    fn missing_derivative_rule_is_an_error() {
        let validator = DifferentiationValidator::default();
        let resolver = TestResolver {
            info: OperationDifferentiabilityInfo {
                differentiability: OperationDifferentiability::Differentiable,
                derivative_rule_available: false,
                derivative_rule_identity: None,
                explanation: None,
            },
        };

        let report = validator
            .validate(&sample_graph(), &sample_request(), &resolver)
            .expect("validation report");

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == DifferentiationValidationCode::MissingDerivativeRule
        }));
    }

    #[test]
    fn empty_variable_set_is_rejected() {
        let validator = DifferentiationValidator::default();
        let resolver = TestResolver {
            info: OperationDifferentiabilityInfo::differentiable(
                "rule.multiply.reverse",
            ),
        };

        let mut request = sample_request();
        request.variables.clear();

        let report = validator
            .validate(&sample_graph(), &request, &resolver)
            .expect("validation report");

        assert!(!report.is_valid());
    }

    #[test]
    fn operation_resolver_is_not_hardware_selection() {
        let resolver = TestResolver {
            info: OperationDifferentiabilityInfo::differentiable(
                "rule.multiply.reverse",
            ),
        };

        let info = resolver.resolve(
            &TestOperationId("op.multiply"),
            &DifferentiationMode::Reverse,
        );

        assert!(info.derivative_rule_available);
    }
}