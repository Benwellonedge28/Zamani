#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Uncertainty
//!
//! Path: `src/ai/reasoning/uncertainty.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent uncertainty descriptions and propagation intent
//! for AI, statistical, scientific, classical, quantum-assisted, distributed,
//! and future computational domains.
//!
//! This module models:
//!
//! - references to canonical uncertainty-bearing AST expressions;
//! - uncertainty classifications and source descriptions;
//! - uncertainty metadata from the canonical uncertainty grammar;
//! - explicit distinctions between uncertainty and numerical error;
//! - uncertainty relationships and correlation assumptions;
//! - propagation requests and propagation method references;
//! - validation of structural uncertainty contracts.
//!
//! ## Authority and integration
//!
//! Universal source syntax:
//!
//!     grammar/expressions/uncertainty.g4
//!
//! AI grammar composition:
//!
//!     grammar/ai/uncertainty.g4
//!     grammar/ai/ai.g4
//!
//! Canonical AST:
//!
//!     crate::frontend::ast
//!
//! Shared AI semantics:
//!
//!     crate::frontend::semantic::domains::ai
//!
//! Probabilistic reasoning:
//!
//!     crate::ai::reasoning::probabilistic
//!
//! Canonical provenance:
//!
//!     crate::ai::provenance
//!
//! Universal AI IR:
//!
//!     crate::zuir::dialects::ai
//!
//! ## Ownership
//!
//! This module owns uncertainty-specific computational-intent descriptors,
//! uncertainty metadata contracts, propagation request descriptions, and
//! structural validation of those descriptions.
//!
//! It does not own:
//!
//! - source parsing or AST construction;
//! - the universal uncertainty expression grammar;
//! - canonical semantic types or semantic analysis;
//! - evidence or knowledge persistence;
//! - probability arithmetic or distribution implementations;
//! - numerical uncertainty estimation;
//! - statistical inference algorithms;
//! - uncertainty propagation algorithms;
//! - tensor storage or a competing IR;
//! - hardware discovery, resource allocation, or scheduling;
//! - runtime execution or physical uncertainty resolution.
//!
//! ## Important distinction
//!
//! Uncertainty and numerical error are not interchangeable.
//!
//! Uncertainty describes incomplete knowledge, variability, measurement
//! uncertainty, model uncertainty, or another declared uncertainty source.
//!
//! Numerical error describes error introduced by finite-precision
//! representation or numerical computation.
//!
//! A value may have both. Neither is silently converted into the other.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on uncertainty records, metadata entries,
//! propagation inputs, relationships, dimensions, or computational targets.
//!
//! Collections grow with the supplied input and available resources.
//! Rust's finite address space, representable values, and allocation behavior
//! remain implementation constraints. Literal infinite computation is not
//! promised.
//!
//! Operational budgets belong to compiler, resource-analysis, and execution
//! layers and must not alter language-level semantics.

use std::collections::BTreeSet;
use std::fmt;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::frontend::ast::node::node_id::NodeId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn uncertainty_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

fn validate_name(
    field: &'static str,
    value: &str,
) -> AiResult<()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(uncertainty_error(
            AiErrorKind::InvalidInput,
            format!(
                "uncertainty {field} must be nonempty \
                 and contain no control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// CANONICAL AST REFERENCES
// ============================================================================

/// A reference to an uncertainty-bearing expression in the canonical AST.
///
/// This wrapper never stores or reconstructs an expression. The referenced
/// node must be resolved against the AST of the current compilation.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct UncertaintyExpressionRef {
    node_id: NodeId,
}

impl UncertaintyExpressionRef {
    /// Create a reference to a canonical AST expression node.
    ///
    /// Existence and expression classification are checked by the AST
    /// adapter or semantic pipeline.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

/// A reference to a canonical uncertainty construct.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct UncertaintyConstructRef {
    node_id: NodeId,
}

impl UncertaintyConstructRef {
    /// Create a reference to a canonical uncertainty construct.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

// ============================================================================
// OPEN-WORLD NAMES
// ============================================================================

/// An open-world uncertainty vocabulary name.
///
/// Names may identify classifications, propagation methods, metadata fields,
/// correlation models, or extension-defined concepts.
///
/// A name is a reference to declared semantics; it does not itself implement
/// a numerical algorithm.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct UncertaintyName {
    namespace: String,
    name: String,
}

impl UncertaintyName {
    /// Construct a qualified uncertainty name.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_name("namespace", &namespace)?;
        validate_name("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified name.
    #[must_use]
    pub fn qualified(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for UncertaintyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// UNCERTAINTY CLASSIFICATION
// ============================================================================

/// Broad uncertainty classes.
///
/// These are descriptive classifications, not an exhaustive taxonomy.
/// Extension-defined classifications use [`UncertaintyName`].
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum UncertaintyClass {
    /// Uncertainty due to incomplete knowledge or information.
    Epistemic,

    /// Uncertainty due to variability in a declared process or population.
    Aleatoric,

    /// Uncertainty associated with a measurement or observation.
    Measurement,

    /// Uncertainty associated with a model or its assumptions.
    Model,

    /// Uncertainty associated with parameters or estimated quantities.
    Parameter,

    /// Uncertainty whose classification has not yet been established.
    Unclassified,
}

/// Classification descriptor with optional extension vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncertaintyClassification {
    class: UncertaintyClass,
    extension: Option<UncertaintyName>,
}

impl UncertaintyClassification {
    /// Create a standard uncertainty classification.
    #[must_use]
    pub const fn standard(class: UncertaintyClass) -> Self {
        Self {
            class,
            extension: None,
        }
    }

    /// Create an extension-defined classification.
    #[must_use]
    pub fn extension(name: UncertaintyName) -> Self {
        Self {
            class: UncertaintyClass::Unclassified,
            extension: Some(name),
        }
    }

    /// Return the broad classification.
    #[must_use]
    pub const fn class(&self) -> UncertaintyClass {
        self.class
    }

    /// Return the extension classification, if present.
    #[must_use]
    pub fn extension_name(&self) -> Option<&UncertaintyName> {
        self.extension.as_ref()
    }
}

// ============================================================================
// NUMERICAL ERROR — DISTINCT FROM UNCERTAINTY
// ============================================================================

/// Categories of numerical error.
///
/// This type is deliberately separate from uncertainty classification.
/// It describes numerical computation or representation error, not
/// epistemic or stochastic uncertainty.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum NumericalErrorKind {
    /// Error from finite-precision representation.
    Representation,

    /// Error accumulated during numerical operations.
    Arithmetic,

    /// Error introduced by an approximation.
    Approximation,

    /// Error from an iterative method or termination criterion.
    Iterative,

    /// Error associated with quantization.
    Quantization,

    /// Error classification is not yet known.
    Unclassified,
}

/// A declared numerical-error reference.
///
/// The optional expression identifies the canonical AST expression that
/// introduced or describes the error. No numerical bound is inferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalErrorDescriptor {
    kind: NumericalErrorKind,
    expression: Option<UncertaintyExpressionRef>,
}

impl NumericalErrorDescriptor {
    /// Construct a numerical-error descriptor.
    #[must_use]
    pub const fn new(
        kind: NumericalErrorKind,
        expression: Option<UncertaintyExpressionRef>,
    ) -> Self {
        Self { kind, expression }
    }

    /// Return the numerical-error category.
    #[must_use]
    pub const fn kind(self) -> NumericalErrorKind {
        self.kind
    }

    /// Return the associated canonical expression, if any.
    #[must_use]
    pub const fn expression(
        self,
    ) -> Option<UncertaintyExpressionRef> {
        self.expression
    }
}

// ============================================================================
// UNCERTAINTY METADATA
// ============================================================================

/// The grammar-level form used for an uncertainty declaration.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum UncertaintyExpressionForm {
    /// Source form `uncertain(value, ...)`.
    Uncertain,

    /// Source form `uncertainty(value, ...)`.
    Uncertainty,
}

/// A named or positional uncertainty metadata argument.
///
/// The expression is always a reference to the canonical AST. Metadata
/// values are not copied into a parallel expression representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UncertaintyArgument {
    name: Option<UncertaintyName>,
    expression: UncertaintyExpressionRef,
}

impl UncertaintyArgument {
    /// Create a positional metadata argument.
    #[must_use]
    pub const fn positional(
        expression: UncertaintyExpressionRef,
    ) -> Self {
        Self {
            name: None,
            expression,
        }
    }

    /// Create a named metadata argument.
    #[must_use]
    pub const fn named(
        name: UncertaintyName,
        expression: UncertaintyExpressionRef,
    ) -> Self {
        Self {
            name: Some(name),
            expression,
        }
    }

    /// Return the optional metadata name.
    #[must_use]
    pub fn name(&self) -> Option<&UncertaintyName> {
        self.name.as_ref()
    }

    /// Return the canonical AST expression reference.
    #[must_use]
    pub const fn expression(
        self,
    ) -> UncertaintyExpressionRef {
        self.expression
    }
}

/// A grammar-aligned uncertainty expression descriptor.
///
/// This structure records source intent only. It does not establish the
/// semantic type, numerical value, distribution, or validity of the
/// referenced expressions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncertaintyExpression {
    source: UncertaintyConstructRef,
    form: UncertaintyExpressionForm,
    value: UncertaintyExpressionRef,
    arguments: Vec<UncertaintyArgument>,
}

impl UncertaintyExpression {
    /// Construct an uncertainty expression descriptor.
    ///
    /// The argument vector is dynamically sized. No arbitrary metadata-count
    /// limit is imposed.
    #[must_use]
    pub fn new(
        source: UncertaintyConstructRef,
        form: UncertaintyExpressionForm,
        value: UncertaintyExpressionRef,
        arguments: Vec<UncertaintyArgument>,
    ) -> Self {
        Self {
            source,
            form,
            value,
            arguments,
        }
    }

    /// Return the source construct reference.
    #[must_use]
    pub const fn source(&self) -> UncertaintyConstructRef {
        self.source
    }

    /// Return the source grammar form.
    #[must_use]
    pub const fn form(&self) -> UncertaintyExpressionForm {
        self.form
    }

    /// Return the canonical value expression reference.
    #[must_use]
    pub const fn value(&self) -> UncertaintyExpressionRef {
        self.value
    }

    /// Return metadata arguments in source order.
    #[must_use]
    pub fn arguments(&self) -> &[UncertaintyArgument] {
        &self.arguments
    }
}

// ============================================================================
// UNCERTAINTY RELATIONSHIPS
// ============================================================================

/// Relationship between uncertainty-bearing quantities.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum UncertaintyRelationKind {
    /// The quantities are declared independent.
    Independent,

    /// The quantities are declared correlated.
    Correlated,

    /// One quantity is conditioned on another.
    Conditional,

    /// One quantity is derived from another.
    DerivedFrom,

    /// The relationship has not been specified.
    Unspecified,
}

/// A relationship between canonical uncertainty constructs.
///
/// Correlation and independence are declarations. This type does not prove
/// them mathematically or statistically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UncertaintyRelation {
    left: UncertaintyConstructRef,
    right: UncertaintyConstructRef,
    kind: UncertaintyRelationKind,
    model: Option<UncertaintyName>,
}

impl UncertaintyRelation {
    /// Construct a relation between two uncertainty references.
    #[must_use]
    pub const fn new(
        left: UncertaintyConstructRef,
        right: UncertaintyConstructRef,
        kind: UncertaintyRelationKind,
        model: Option<UncertaintyName>,
    ) -> Self {
        Self {
            left,
            right,
            kind,
            model,
        }
    }

    /// Return the left-hand uncertainty reference.
    #[must_use]
    pub const fn left(self) -> UncertaintyConstructRef {
        self.left
    }

    /// Return the right-hand uncertainty reference.
    #[must_use]
    pub const fn right(self) -> UncertaintyConstructRef {
        self.right
    }

    /// Return the declared relationship.
    #[must_use]
    pub const fn kind(self) -> UncertaintyRelationKind {
        self.kind
    }

    /// Return the optional relationship model.
    #[must_use]
    pub fn model(&self) -> Option<&UncertaintyName> {
        self.model.as_ref()
    }
}

// ============================================================================
// PROPAGATION INTENT
// ============================================================================

/// Declared uncertainty propagation method.
///
/// The method is an open-world name. The registry/compiler determines
/// whether the method is defined, mathematically applicable, and available
/// for a particular realization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropagationMethod {
    name: UncertaintyName,
}

impl PropagationMethod {
    /// Construct a propagation method reference.
    #[must_use]
    pub const fn new(name: UncertaintyName) -> Self {
        Self { name }
    }

    /// Return the method name.
    #[must_use]
    pub fn name(&self) -> &UncertaintyName {
        &self.name
    }
}

/// A target-independent uncertainty propagation request.
///
/// Inputs and output refer to canonical AST constructs. The request does
/// not execute propagation or assert that the result is feasible.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncertaintyPropagation {
    source: UncertaintyConstructRef,
    inputs: Vec<UncertaintyConstructRef>,
    output: UncertaintyConstructRef,
    method: Option<PropagationMethod>,
    relationships: Vec<UncertaintyRelation>,
}

impl UncertaintyPropagation {
    /// Create a propagation request.
    ///
    /// The caller supplies all input references and relationship declarations.
    /// No fixed input-count or relationship-count limit is imposed.
    #[must_use]
    pub fn new(
        source: UncertaintyConstructRef,
        inputs: Vec<UncertaintyConstructRef>,
        output: UncertaintyConstructRef,
        method: Option<PropagationMethod>,
        relationships: Vec<UncertaintyRelation>,
    ) -> Self {
        Self {
            source,
            inputs,
            output,
            method,
            relationships,
        }
    }

    /// Return the propagation request source.
    #[must_use]
    pub const fn source(&self) -> UncertaintyConstructRef {
        self.source
    }

    /// Return propagation inputs in declared order.
    #[must_use]
    pub fn inputs(&self) -> &[UncertaintyConstructRef] {
        &self.inputs
    }

    /// Return the output uncertainty reference.
    #[must_use]
    pub const fn output(&self) -> UncertaintyConstructRef {
        self.output
    }

    /// Return the optional propagation method.
    #[must_use]
    pub fn method(&self) -> Option<&PropagationMethod> {
        self.method.as_ref()
    }

    /// Return declared relationships.
    #[must_use]
    pub fn relationships(&self) -> &[UncertaintyRelation] {
        &self.relationships
    }
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

/// Structural uncertainty validation result.
///
/// A complete result means only that the local structural checks passed.
/// It does not establish mathematical correctness, probability validity,
/// numerical stability, or execution feasibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncertaintyValidationReport {
    constructs_checked: usize,
    expressions_checked: usize,
    relationships_checked: usize,
    issues: Vec<UncertaintyValidationIssue>,
}

impl UncertaintyValidationReport {
    fn new() -> Self {
        Self {
            constructs_checked: 0,
            expressions_checked: 0,
            relationships_checked: 0,
            issues: Vec::new(),
        }
    }

    /// Number of uncertainty constructs checked.
    #[must_use]
    pub const fn constructs_checked(&self) -> usize {
        self.constructs_checked
    }

    /// Number of expression references checked.
    #[must_use]
    pub const fn expressions_checked(&self) -> usize {
        self.expressions_checked
    }

    /// Number of relationships checked.
    #[must_use]
    pub const fn relationships_checked(&self) -> usize {
        self.relationships_checked
    }

    /// Return structural issues in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[UncertaintyValidationIssue] {
        &self.issues
    }

    /// Whether the local structural validation completed without issues.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.issues.is_empty()
    }
}

/// A structural uncertainty validation issue.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct UncertaintyValidationIssue {
    code: UncertaintyValidationCode,
    message: String,
    source: Option<UncertaintyConstructRef>,
}

impl UncertaintyValidationIssue {
    /// Return the stable issue code.
    #[must_use]
    pub const fn code(&self) -> UncertaintyValidationCode {
        self.code
    }

    /// Return the issue message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the associated uncertainty construct.
    #[must_use]
    pub const fn source(
        &self,
    ) -> Option<UncertaintyConstructRef> {
        self.source
    }
}

/// Stable structural issue categories.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[non_exhaustive]
pub enum UncertaintyValidationCode {
    /// A construct identity is duplicated in the supplied validation set.
    DuplicateConstruct,

    /// An uncertainty expression references an invalid local relationship.
    InvalidExpression,

    /// A propagation request contains no declared input.
    EmptyPropagationInputs,

    /// A propagation input is repeated.
    DuplicatePropagationInput,

    /// A propagation request refers to itself as an input.
    SelfPropagation,
}

fn issue(
    code: UncertaintyValidationCode,
    message: impl Into<String>,
    source: Option<UncertaintyConstructRef>,
) -> UncertaintyValidationIssue {
    UncertaintyValidationIssue {
        code,
        message: message.into(),
        source,
    }
}

/// Validate one uncertainty expression's local structure.
///
/// Canonical AST resolution, semantic typing, metadata meaning, and source
/// span diagnostics remain responsibilities of the frontend adapter and
/// semantic pipeline.
pub fn validate_expression(
    expression: &UncertaintyExpression,
) -> AiResult<UncertaintyValidationReport> {
    let mut report = UncertaintyValidationReport::new();

    report.constructs_checked = 1;
    report.expressions_checked = 1;

    let mut named_arguments = BTreeSet::new();

    for argument in expression.arguments() {
        if let Some(name) = argument.name() {
            if !named_arguments.insert(name.clone()) {
                report.issues.push(issue(
                    UncertaintyValidationCode::InvalidExpression,
                    format!(
                        "duplicate uncertainty metadata field `{name}`"
                    ),
                    Some(expression.source()),
                ));
            }
        }
    }

    report.issues.sort();

    Ok(report)
}

/// Validate a propagation request's local structure.
///
/// This checks only request structure. It does not establish that a selected
/// propagation method exists or is mathematically appropriate.
pub fn validate_propagation(
    propagation: &UncertaintyPropagation,
) -> AiResult<UncertaintyValidationReport> {
    let mut report = UncertaintyValidationReport::new();

    report.constructs_checked = 1;
    report.relationships_checked =
        propagation.relationships().len();

    if propagation.inputs().is_empty() {
        report.issues.push(issue(
            UncertaintyValidationCode::EmptyPropagationInputs,
            "uncertainty propagation requires at least one input",
            Some(propagation.source()),
        ));
    }

    let mut inputs = BTreeSet::new();

    for input in propagation.inputs() {
        if *input == propagation.output() {
            report.issues.push(issue(
                UncertaintyValidationCode::SelfPropagation,
                "uncertainty propagation output cannot be its own input",
                Some(propagation.source()),
            ));
        }

        if !inputs.insert(*input) {
            report.issues.push(issue(
                UncertaintyValidationCode::DuplicatePropagationInput,
                "uncertainty propagation input is repeated",
                Some(propagation.source()),
            ));
        }
    }

    report.issues.sort();

    Ok(report)
}

/// Validate construct identity uniqueness across a supplied sequence.
///
/// The caller owns the collection and its lifecycle. The validator does not
/// persist or register uncertainty constructs.
pub fn validate_unique_constructs(
    constructs: impl IntoIterator<Item = UncertaintyConstructRef>,
) -> AiResult<UncertaintyValidationReport> {
    let mut report = UncertaintyValidationReport::new();
    let mut seen = BTreeSet::new();

    for construct in constructs {
        report.constructs_checked += 1;

        if !seen.insert(construct) {
            report.issues.push(issue(
                UncertaintyValidationCode::DuplicateConstruct,
                "uncertainty construct identity is duplicated",
                Some(construct),
            ));
        }
    }

    report.issues.sort();

    Ok(report)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // These tests exercise pure contract behavior. AST resolution and
    // semantic correctness are tested by the frontend integration suite.

    fn name(namespace: &str, value: &str) -> UncertaintyName {
        UncertaintyName::new(namespace, value)
            .expect("valid uncertainty name")
    }

    #[test]
    fn uncertainty_names_are_qualified_and_open_world() {
        let value = name("extension", "custom_propagation");

        assert_eq!(
            value.qualified(),
            "extension.custom_propagation"
        );
    }

    #[test]
    fn rejects_empty_name_components() {
        assert!(
            UncertaintyName::new("", "method").is_err()
        );

        assert!(
            UncertaintyName::new("namespace", "").is_err()
        );
    }

    #[test]
    fn uncertainty_and_numerical_error_are_distinct_types() {
        assert_ne!(
            std::any::type_name::<UncertaintyClassification>(),
            std::any::type_name::<NumericalErrorDescriptor>()
        );
    }

    #[test]
    fn uncertainty_classification_preserves_extension_name() {
        let extension = name("custom", "sensor_uncertainty");
        let classification =
            UncertaintyClassification::extension(extension.clone());

        assert_eq!(
            classification.extension_name(),
            Some(&extension)
        );
        assert_eq!(
            classification.class(),
            UncertaintyClass::Unclassified
        );
    }

    #[test]
    fn metadata_arguments_preserve_order_and_names() {
        let first = UncertaintyArgument::named(
            name("uncertainty", "confidence"),
            UncertaintyExpressionRef::new(NodeId::new(1)),
        );

        let second = UncertaintyArgument::positional(
            UncertaintyExpressionRef::new(NodeId::new(2)),
        );

        assert!(first.name().is_some());
        assert!(second.name().is_none());
    }

    #[test]
    fn propagation_request_is_structural_and_dynamic() {
        let first = UncertaintyConstructRef::new(NodeId::new(1));
        let second = UncertaintyConstructRef::new(NodeId::new(2));
        let output = UncertaintyConstructRef::new(NodeId::new(3));

        let propagation = UncertaintyPropagation::new(
            first,
            vec![first, second],
            output,
            Some(PropagationMethod::new(
                name("uncertainty", "linearized"),
            )),
            Vec::new(),
        );

        let report = validate_propagation(&propagation)
            .expect("validation report");

        assert!(report.is_complete());
        assert_eq!(report.constructs_checked(), 1);
    }

    #[test]
    fn rejects_empty_propagation_inputs() {
        let source = UncertaintyConstructRef::new(NodeId::new(1));
        let output = UncertaintyConstructRef::new(NodeId::new(2));

        let propagation = UncertaintyPropagation::new(
            source,
            Vec::new(),
            output,
            None,
            Vec::new(),
        );

        let report = validate_propagation(&propagation)
            .expect("validation report");

        assert!(!report.is_complete());
        assert_eq!(
            report.issues()[0].code(),
            UncertaintyValidationCode::EmptyPropagationInputs
        );
    }

    #[test]
    fn rejects_duplicate_propagation_inputs() {
        let source = UncertaintyConstructRef::new(NodeId::new(1));
        let input = UncertaintyConstructRef::new(NodeId::new(2));
        let output = UncertaintyConstructRef::new(NodeId::new(3));

        let propagation = UncertaintyPropagation::new(
            source,
            vec![input, input],
            output,
            None,
            Vec::new(),
        );

        let report = validate_propagation(&propagation)
            .expect("validation report");

        assert!(!report.is_complete());
        assert_eq!(
            report.issues()[0].code(),
            UncertaintyValidationCode::DuplicatePropagationInput
        );
    }
}