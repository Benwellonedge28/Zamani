
#![forbid(unsafe_code)]

//! AI and machine-learning semantic validation for Zamani.
//!
//! # File contract
//!
//! PURPOSE
//!   Validate AI/ML semantic declarations and relationships that are specific
//!   to AI computation while delegating shared language rules to the canonical
//!   semantic infrastructure.
//!
//! STATUS
//!   Proposed integration module. The repository's actual AST and semantic
//!   service APIs must be connected through `AiSemanticContext` before this
//!   file can be considered integrated and frozen.
//!
//! AUTHORITY
//!   The canonical frontend AST, normative Zamani grammar/specifications,
//!   shared semantic type system, effects, capabilities, resources and
//!   diagnostics are authoritative. This file does not define source syntax.
//!
//! OWNS
//!   AI-specific semantic input records, symbolic tensor-shape expressions,
//!   AI operation metadata contracts, AI-specific structural checks and
//!   deterministic validation reporting.
//!
//! DOES_NOT_OWN
//!   Parsing, AST construction, ordinary name resolution, the canonical type
//!   system, generic inference, effect authorization, capability evidence,
//!   resource feasibility, model training/execution, automatic differentiation
//!   implementation, hardware selection, scheduling, lowering or IR creation.
//!
//! INPUTS
//!   Adapter-produced records derived from canonical AST nodes and shared
//!   semantic facts.
//!
//! OUTPUTS
//!   Deterministically ordered AI semantic issues and an explicit completeness
//!   status. This is not a substitute for the complete program semantic result.
//!
//! INTEGRATION
//!   `domains/mod.rs` declares this module. The semantic pipeline converts
//!   canonical AST nodes into these records, supplies a shared semantic context,
//!   runs validation, and maps issues into the repository's canonical diagnostic
//!   type. Prefer direct canonical-type and source-span references in the
//!   adapter; do not create a competing AST or type system.
//!
//! SCALABILITY
//!   No fixed tensor rank, parameter count, model size, device count, or
//!   accelerator capacity is imposed here. Operational budgets belong to the
//!   semantic session/configuration. A budget failure must be represented as
//!   incomplete analysis, never as proof of validity.
//!
//! DETERMINISM
//!   Issues are sorted by stable fields. Operation identifiers and shape
//!   expressions are compared structurally. The adapter/context must itself
//!   provide deterministic answers.
//!
//! FREEZE CRITERIA
//!   Connect the actual AST, type/constraint solver, effects, capabilities,
//!   resource model, provenance, diagnostics and pipeline; run domain tests,
//!   workspace tests, determinism tests, and the AI conformance corpus.

use std::collections::{BTreeMap, BTreeSet};

/// Stable identity for an AI semantic entity.
///
/// The adapter should use a stable AST or semantic identity where possible.
/// The string representation is intentionally not interpreted by this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AiId(String);

impl AiId {
    /// Creates an identity from a non-empty identifier.
    ///
    /// Whitespace is not silently trimmed: doing so could merge distinct IDs.
    pub fn new(value: impl Into<String>) -> Result<Self, AiInputError> {
        let value = value.into();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(AiInputError::InvalidIdentifier);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Optional source provenance supplied by the canonical AST adapter.
///
/// These strings are references, not replacements for the AST's source-span
/// and node-ID types. The production adapter should preserve native types
/// internally and format them only when constructing this boundary record.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct AiOrigin {
    pub source_name: Option<String>,
    pub node_identity: Option<String>,
}

/// A qualified, extensible domain or operation identity.
///
/// Names are opaque to this module. The registry and language specifications
/// determine whether an identity is defined and what it means.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QualifiedName {
    pub namespace: String,
    pub name: String,
}

impl QualifiedName {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, AiInputError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty()
            || name.is_empty()
            || namespace.chars().any(char::is_control)
            || name.chars().any(char::is_control)
        {
            return Err(AiInputError::InvalidQualifiedName);
        }

        Ok(Self { namespace, name })
    }
}

/// A dimension that can be checked without forcing symbolic values into a
/// fixed machine-sized integer.
///
/// Arithmetic evaluation, overflow policy, constant evaluation and constraint
/// solving belong to shared semantic services. This representation preserves
/// symbolic relationships rather than assuming that every dimension is known
/// during initial analysis.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DimensionExpr {
    Constant(u128),
    Symbol(AiId),
    Add(Box<DimensionExpr>, Box<DimensionExpr>),
    Multiply(Box<DimensionExpr>, Box<DimensionExpr>),
    Subtract(Box<DimensionExpr>, Box<DimensionExpr>),
    Divide(Box<DimensionExpr>, Box<DimensionExpr>),
    /// A dimension expression represented by the shared constraint system.
    ///
    /// The adapter must resolve this identity through that system. Its
    /// presence does not prove that the expression is well-formed.
    Constraint(AiId),
}

/// Tensor shape whose rank and dimensions are not bounded by this module.
///
/// A vector is used as a representation, not as a language-level maximum.
/// Memory and work limits must be controlled by the caller's operational
/// configuration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TensorShape {
    pub dimensions: Vec<DimensionExpr>,
}

impl TensorShape {
    pub fn new(dimensions: Vec<DimensionExpr>) -> Self {
        Self { dimensions }
    }

    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    pub fn is_scalar(&self) -> bool {
        self.dimensions.is_empty()
    }
}

/// Extensible AI value category.
///
/// These categories describe AI-domain roles; they are not substitutes for
/// canonical semantic types. For example, a tensor's element type must still
/// be checked by the shared type system.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AiValueKind {
    Tensor,
    Model,
    Dataset,
    Optimizer,
    Loss,
    Prediction,
    Gradient,
    TrainingState,
    Custom(QualifiedName),
}

/// AI-specific information associated with a value already identified by
/// the canonical semantic layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiValue {
    pub id: AiId,
    pub kind: AiValueKind,
    /// Canonical semantic type identity, serialized by the adapter.
    ///
    /// This module does not parse or reinterpret this string.
    pub type_identity: String,
    pub shape: Option<TensorShape>,
    pub origin: AiOrigin,
}

/// AI operation class.
///
/// `Custom` allows registered operations without extending this enum for
/// every operator, model family, training method, or future domain feature.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AiOperationKind {
    TensorOperation,
    ModelDefinition,
    Inference,
    Training,
    LossEvaluation,
    GradientComputation,
    Optimization,
    DataTransformation,
    Custom(QualifiedName),
}

/// An operation declaration or invocation after adaptation from the canonical
/// AST. The adapter must populate it only with facts actually present in the
/// AST or established by earlier semantic passes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiOperation {
    pub id: AiId,
    pub operation: QualifiedName,
    pub kind: AiOperationKind,
    pub inputs: Vec<AiId>,
    pub outputs: Vec<AiId>,
    pub origin: AiOrigin,
}

/// A relation between values that the AI validator may check.
///
/// This is a semantic boundary record, not an executable computation graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiValueFlow {
    pub producer: AiId,
    pub consumer: AiId,
    pub value: AiId,
    pub expected_type_identity: Option<String>,
    pub origin: AiOrigin,
}

/// An AI-domain semantic input assembled by the frontend adapter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AiProgram {
    pub values: Vec<AiValue>,
    pub operations: Vec<AiOperation>,
    pub flows: Vec<AiValueFlow>,
}

/// Input-record construction failures.
///
/// These failures describe malformed adapter records. They are not a
/// replacement for user-facing source diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiInputError {
    InvalidIdentifier,
    InvalidQualifiedName,
}

impl std::fmt::Display for AiInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIdentifier => {
                f.write_str("identifier must be non-empty and contain no control characters")
            }
            Self::InvalidQualifiedName => {
                f.write_str("qualified name must have non-empty, valid components")
            }
        }
    }
}

impl std::error::Error for AiInputError {}

/// Classification returned by shared semantic services.
///
/// `Incomplete` means that the service could not establish a definitive
/// answer, for example because a type or constraint remains unresolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckStatus {
    Valid,
    Invalid,
    Incomplete,
}

/// Result of a shared semantic check.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextCheck {
    pub status: CheckStatus,
    pub code: String,
    pub message: String,
}

impl ContextCheck {
    pub fn valid() -> Self {
        Self {
            status: CheckStatus::Valid,
            code: String::new(),
            message: String::new(),
        }
    }

    pub fn invalid(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Invalid,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn incomplete(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: CheckStatus::Incomplete,
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Shared semantic services required by AI validation.
///
/// The real frontend integration should implement this trait by delegating to
/// canonical type/constraint, effect, capability, resource, and operation
/// registries. Do not implement those systems again inside this file.
///
/// All methods must be deterministic for the same AST, configuration,
/// extension registry and semantic facts.
pub trait AiSemanticContext {
    /// Check that an operation is registered and valid in the language version
    /// and semantic environment selected for this analysis.
    fn validate_operation(&self, operation: &AiOperation) -> ContextCheck;

    /// Check an AI value against the canonical type system.
    fn validate_value_type(&self, value: &AiValue) -> ContextCheck;

    /// Check an individual symbolic dimension using the shared constraint and
    /// constant-evaluation infrastructure.
    fn validate_dimension(&self, dimension: &DimensionExpr) -> ContextCheck;

    /// Check value flow, including canonical type compatibility, domain
    /// crossing rules, effects and relevant obligations.
    fn validate_flow(
        &self,
        flow: &AiValueFlow,
        value: &AiValue,
        producer: &AiOperation,
        consumer: &AiOperation,
    ) -> ContextCheck;

    /// Check the operation's effect, capability, resource and contract
    /// obligations through the shared semantic services.
    fn validate_obligations(&self, operation: &AiOperation) -> ContextCheck;
}

/// Severity of a domain-specific issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AiSeverity {
    Warning,
    Error,
}

/// A stable AI-domain diagnostic candidate.
///
/// The pipeline adapter should map this to the repository's canonical
/// diagnostic type and use the native source span for labels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiIssue {
    pub severity: AiSeverity,
    pub code: String,
    pub message: String,
    pub entity: Option<AiId>,
    pub origin: AiOrigin,
}

impl Ord for AiIssue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            &self.entity,
            &self.code,
            self.severity,
            &self.message,
            &self.origin,
        )
            .cmp(&(
                &other.entity,
                &other.code,
                other.severity,
                &other.message,
                &other.origin,
            ))
    }
}

impl PartialOrd for AiIssue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Validation result. An incomplete result must never be reported as certified
/// semantic success.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AiValidationReport {
    pub issues: Vec<AiIssue>,
    pub complete: bool,
}

impl AiValidationReport {
    pub fn is_valid(&self) -> bool {
        self.complete
            && !self
                .issues
                .iter()
                .any(|issue| issue.severity == AiSeverity::Error)
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == AiSeverity::Error)
    }

    fn push(
        &mut self,
        severity: AiSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
        entity: Option<AiId>,
        origin: AiOrigin,
    ) {
        self.issues.push(AiIssue {
            severity,
            code: code.into(),
            message: message.into(),
            entity,
            origin,
        });
    }

    fn apply_check(
        &mut self,
        check: ContextCheck,
        entity: Option<AiId>,
        origin: AiOrigin,
    ) {
        match check.status {
            CheckStatus::Valid => {}
            CheckStatus::Invalid => self.push(
                AiSeverity::Error,
                check.code,
                check.message,
                entity,
                origin,
            ),
            CheckStatus::Incomplete => {
                self.complete = false;
                self.push(
                    AiSeverity::Error,
                    check.code,
                    check.message,
                    entity,
                    origin,
                );
            }
        }
    }

    fn finish(&mut self) {
        self.issues.sort();
        self.issues.dedup();
    }
}

impl AiProgram {
    /// Validate AI-specific records using shared semantic services.
    ///
    /// This method does not parse source, resolve names, or establish hardware
    /// feasibility. It checks the integrity of the adapted records, invokes
    /// shared semantic services, and reports unresolved information explicitly.
    pub fn validate<C: AiSemanticContext>(&self, context: &C) -> AiValidationReport {
        let mut report = AiValidationReport {
            issues: Vec::new(),
            complete: true,
        };

        let mut values: BTreeMap<AiId, &AiValue> = BTreeMap::new();
        for value in &self.values {
            if values.insert(value.id.clone(), value).is_some() {
                report.push(
                    AiSeverity::Error,
                    "AI001",
                    "duplicate AI value identity",
                    Some(value.id.clone()),
                    value.origin.clone(),
                );
            }
        }

        let mut operations: BTreeMap<AiId, &AiOperation> = BTreeMap::new();
        for operation in &self.operations {
            if operations
                .insert(operation.id.clone(), operation)
                .is_some()
            {
                report.push(
                    AiSeverity::Error,
                    "AI002",
                    "duplicate AI operation identity",
                    Some(operation.id.clone()),
                    operation.origin.clone(),
                );
            }
        }

        // Validate operation declarations and shared obligations.
        for operation in &self.operations {
            report.apply_check(
                context.validate_operation(operation),
                Some(operation.id.clone()),
                operation.origin.clone(),
            );

            report.apply_check(
                context.validate_obligations(operation),
                Some(operation.id.clone()),
                operation.origin.clone(),
            );

            let mut seen_inputs = BTreeSet::new();
            for input in &operation.inputs {
                if !seen_inputs.insert(input.clone()) {
                    report.push(
                        AiSeverity::Error,
                        "AI003",
                        "operation lists the same input identity more than once",
                        Some(operation.id.clone()),
                        operation.origin.clone(),
                    );
                }

                if !values.contains_key(input) {
                    report.push(
                        AiSeverity::Error,
                        "AI004",
                        "operation input does not identify a declared AI value",
                        Some(operation.id.clone()),
                        operation.origin.clone(),
                    );
                }
            }

            let mut seen_outputs = BTreeSet::new();
            for output in &operation.outputs {
                if !seen_outputs.insert(output.clone()) {
                    report.push(
                        AiSeverity::Error,
                        "AI005",
                        "operation lists the same output identity more than once",
                        Some(operation.id.clone()),
                        operation.origin.clone(),
                    );
                }

                if !values.contains_key(output) {
                    report.push(
                        AiSeverity::Error,
                        "AI006",
                        "operation output does not identify a declared AI value",
                        Some(operation.id.clone()),
                        operation.origin.clone(),
                    );
                }
            }
        }

        // Validate value declarations and all symbolic dimensions.
        for value in &self.values {
            report.apply_check(
                context.validate_value_type(value),
                Some(value.id.clone()),
                value.origin.clone(),
            );

            if let Some(shape) = &value.shape {
                for dimension in &shape.dimensions {
                    report.apply_check(
                        context.validate_dimension(dimension),
                        Some(value.id.clone()),
                        value.origin.clone(),
                    );
                }
            }
        }

        // Validate producer/consumer relationships without assuming that
        // vector order corresponds to execution order.
        let mut seen_flows: BTreeSet<(AiId, AiId, AiId)> = BTreeSet::new();

        for flow in &self.flows {
            let flow_key = (
                flow.producer.clone(),
                flow.consumer.clone(),
                flow.value.clone(),
            );

            if !seen_flows.insert(flow_key) {
                report.push(
                    AiSeverity::Error,
                    "AI007",
                    "duplicate value-flow relationship",
                    Some(flow.consumer.clone()),
                    flow.origin.clone(),
                );
                continue;
            }

            let Some(value) = values.get(&flow.value).copied() else {
                report.push(
                    AiSeverity::Error,
                    "AI008",
                    "value-flow relationship references an unknown value",
                    Some(flow.value.clone()),
                    flow.origin.clone(),
                );
                continue;
            };

            let Some(producer) = operations.get(&flow.producer).copied() else {
                report.push(
                    AiSeverity::Error,
                    "AI009",
                    "value-flow relationship references an unknown producer",
                    Some(flow.producer.clone()),
                    flow.origin.clone(),
                );
                continue;
            };

            let Some(consumer) = operations.get(&flow.consumer).copied() else {
                report.push(
                    AiSeverity::Error,
                    "AI010",
                    "value-flow relationship references an unknown consumer",
                    Some(flow.consumer.clone()),
                    flow.origin.clone(),
                );
                continue;
            };

            if !producer.outputs.contains(&flow.value) {
                report.push(
                    AiSeverity::Error,
                    "AI011",
                    "producer does not declare this value as an output",
                    Some(flow.producer.clone()),
                    flow.origin.clone(),
                );
                continue;
            }

            if !consumer.inputs.contains(&flow.value) {
                report.push(
                    AiSeverity::Error,
                    "AI012",
                    "consumer does not declare this value as an input",
                    Some(flow.consumer.clone()),
                    flow.origin.clone(),
                );
                continue;
            }

            report.apply_check(
                context.validate_flow(flow, value, producer, consumer),
                Some(flow.value.clone()),
                flow.origin.clone(),
            );
        }

        // An output must have a unique producer. This is an integrity check,
        // not a rule about whether a value is physically materialized.
        let mut producer_by_value: BTreeMap<AiId, AiId> = BTreeMap::new();

        for operation in &self.operations {
            for output in &operation.outputs {
                if let Some(previous) =
                    producer_by_value.insert(output.clone(), operation.id.clone())
                {
                    if previous != operation.id {
                        report.push(
                            AiSeverity::Error,
                            "AI013",
                            "AI value has multiple distinct producers",
                            Some(output.clone()),
                            values
                                .get(output)
                                .map(|value| value.origin.clone())
                                .unwrap_or_default(),
                        );
                    }
                }
            }
        }

        report.finish();
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AcceptAllContext;

    impl AiSemanticContext for AcceptAllContext {
        fn validate_operation(&self, _: &AiOperation) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_value_type(&self, _: &AiValue) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_dimension(&self, _: &DimensionExpr) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_flow(
            &self,
            _: &AiValueFlow,
            _: &AiValue,
            _: &AiOperation,
            _: &AiOperation,
        ) -> ContextCheck {
            ContextCheck::valid()
        }

        fn validate_obligations(&self, _: &AiOperation) -> ContextCheck {
            ContextCheck::valid()
        }
    }

    fn id(value: &str) -> AiId {
        AiId::new(value).expect("test identifier must be valid")
    }

    fn qualified(namespace: &str, name: &str) -> QualifiedName {
        QualifiedName::new(namespace, name).expect("test name must be valid")
    }

    fn value(value_id: &str, shape: Option<TensorShape>) -> AiValue {
        AiValue {
            id: id(value_id),
            kind: AiValueKind::Tensor,
            type_identity: "core.tensor".to_owned(),
            shape,
            origin: AiOrigin::default(),
        }
    }

    fn operation(
        operation_id: &str,
        inputs: Vec<&str>,
        outputs: Vec<&str>,
    ) -> AiOperation {
        AiOperation {
            id: id(operation_id),
            operation: qualified("ai", "operation"),
            kind: AiOperationKind::TensorOperation,
            inputs: inputs.into_iter().map(id).collect(),
            outputs: outputs.into_iter().map(id).collect(),
            origin: AiOrigin::default(),
        }
    }

    #[test]
    fn identifiers_reject_empty_and_control_characters() {
        assert!(AiId::new("").is_err());
        assert!(AiId::new("x\n").is_err());
        assert!(AiId::new("tensor_α").is_ok());
    }

    #[test]
    fn symbolic_dimensions_are_preserved() {
        let shape = TensorShape::new(vec![
            DimensionExpr::Constant(128),
            DimensionExpr::Symbol(id("batch")),
            DimensionExpr::Multiply(
                Box::new(DimensionExpr::Symbol(id("channels"))),
                Box::new(DimensionExpr::Constant(2)),
            ),
        ]);

        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.dimensions[1], DimensionExpr::Symbol(id("batch")));
    }

    #[test]
    fn empty_program_is_valid_when_context_has_no_extra_obligations() {
        let report = AiProgram::default().validate(&AcceptAllContext);
        assert!(report.is_valid());
        assert!(report.complete);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn duplicate_value_ids_are_reported() {
        let program = AiProgram {
            values: vec![value("v", None), value("v", None)],
            ..AiProgram::default()
        };

        let report = program.validate(&AcceptAllContext);

        assert!(report.has_errors());
        assert!(report.issues.iter().any(|issue| issue.code == "AI001"));
    }

    #[test]
    fn duplicate_operation_ids_are_reported() {
        let program = AiProgram {
            operations: vec![
                operation("op", vec![], vec![]),
                operation("op", vec![], vec![]),
            ],
            ..AiProgram::default()
        };

        let report = program.validate(&AcceptAllContext);

        assert!(report.issues.iter().any(|issue| issue.code == "AI002"));
    }

    #[test]
    fn undeclared_inputs_are_reported() {
        let program = AiProgram {
            operations: vec![operation("op", vec!["missing"], vec![])],
            ..AiProgram::default()
        };

        let report = program.validate(&AcceptAllContext);

        assert!(report.issues.iter().any(|issue| issue.code == "AI004"));
    }

    #[test]
    fn multiple_producers_are_reported() {
        let program = AiProgram {
            values: vec![value("v", None)],
            operations: vec![
                operation("first", vec![], vec!["v"]),
                operation("second", vec![], vec!["v"]),
            ],
            ..AiProgram::default()
        };

        let report = program.validate(&AcceptAllContext);

        assert!(report.issues.iter().any(|issue| issue.code == "AI013"));
    }

    #[test]
    fn unresolved_shared_constraint_prevents_valid_result() {
        struct IncompleteContext;

        impl AiSemanticContext for IncompleteContext {
            fn validate_operation(&self, _: &AiOperation) -> ContextCheck {
                ContextCheck::valid()
            }

            fn validate_value_type(&self, _: &AiValue) -> ContextCheck {
                ContextCheck::valid()
            }

            fn validate_dimension(&self, _: &DimensionExpr) -> ContextCheck {
                ContextCheck::incomplete(
                    "TYPE001",
                    "symbolic dimension has not been resolved",
                )
            }

            fn validate_flow(
                &self,
                _: &AiValueFlow,
                _: &AiValue,
                _: &AiOperation,
                _: &AiOperation,
            ) -> ContextCheck {
                ContextCheck::valid()
            }

            fn validate_obligations(&self, _: &AiOperation) -> ContextCheck {
                ContextCheck::valid()
            }
        }

        let program = AiProgram {
            values: vec![value(
                "tensor",
                Some(TensorShape::new(vec![DimensionExpr::Symbol(id("batch"))])),
            )],
            ..AiProgram::default()
        };

        let report = program.validate(&IncompleteContext);

        assert!(!report.complete);
        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| issue.code == "TYPE001"));
    }

    #[test]
    fn diagnostics_are_deterministically_ordered() {
        let program = AiProgram {
            values: vec![value("v", None), value("v", None)],
            operations: vec![
                operation("op", vec!["unknown_b"], vec![]),
                operation("op", vec!["unknown_a"], vec![]),
            ],
            ..AiProgram::default()
        };

        let first = program.validate(&AcceptAllContext);
        let second = program.validate(&AcceptAllContext);

        assert_eq!(first.issues, second.issues);
    }
}
