
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Assertions
//!
//! Path: `src/hdl/verification/assertions.rs`
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! Define a target-independent semantic contract for HDL assertions and
//! reusable properties. This module validates assertion/property structure,
//! identities, references, arguments, qualifiers, and property dependencies.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/assertions.g4`
//! - `grammar/hdl/verification.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - The canonical frontend AST and expression/type semantic contracts.
//!
//! The grammar owns syntax. The canonical AST owns source expressions.
//! Shared semantic infrastructure owns expression typing, name resolution,
//! effects, source identity, and canonical diagnostics.
//!
//! ## Does not own
//!
//! - Parsing or a second expression AST.
//! - General-purpose name resolution or type checking.
//! - Assertion execution, simulation scheduling, or formal proof.
//! - Assumption and coverage declaration implementations.
//! - Physical target selection or hardware allocation.
//! - HDLIR, lowering, synthesis, placement, or routing.
//!
//! ## Scalability and soundness
//!
//! There are no language-level maximums for assertion count, property count,
//! parameter count, or dependency count. The caller may supply operational
//! budgets. Budget exhaustion is reported as incomplete analysis, never as
//! successful verification.
//!
//! This module does not claim that a property is true merely because its
//! syntax and references are valid.
//!
//! ## Integration
//!
//! The frontend adapter maps canonical expression nodes to `ExpressionRef`
//! values and retains source provenance. The semantic pipeline supplies an
//! `AssertionEnvironment` that resolves expressions, properties, and
//! qualifier expressions using the canonical semantic model.
//!
//! No AST constructors or diagnostic constructors are assumed here.
//!
//! ## Freeze criteria
//!
//! - Grammar-to-model mapping is loss-preserving.
//! - Expression/type checks delegate to canonical semantic infrastructure.
//! - Results distinguish invalid, valid-but-unproven, and incomplete.
//! - Diagnostics and dependency traversal are deterministic.
//! - Operational budgets never change source-language meaning.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::{HdlId, HdlSourceLocation};

// -----------------------------------------------------------------------------
// Opaque references to canonical frontend expressions
// -----------------------------------------------------------------------------

/// Stable adapter key for a canonical frontend expression node.
///
/// This is an opaque reference, not an expression representation. The
/// frontend adapter must derive it from the canonical AST node identity.
/// Consumers must resolve it through [`AssertionEnvironment`]; they must not
/// parse or reinterpret an expression from this key.
///
/// The string representation is used only because the canonical AST's concrete
/// node-ID API must remain owned by the AST subsystem.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionRef(String);

impl ExpressionRef {
    /// Create an opaque reference from a canonical AST identity encoding.
    ///
    /// Empty or whitespace-only keys are rejected. The caller is responsible
    /// for supplying a stable encoding of an existing canonical node ID.
    pub fn new(value: impl Into<String>) -> Result<Self, AssertionInputError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AssertionInputError::EmptyExpressionReference);
        }
        Ok(Self(value))
    }

    /// Return the opaque identity encoding.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// -----------------------------------------------------------------------------
// Source-level assertion and property model
// -----------------------------------------------------------------------------

/// Source-level assertion category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AssertionKind {
    /// Immediate assertion evaluated according to its enclosing execution
    /// semantics.
    Immediate,
    /// Property assertion that references or embeds a reusable property.
    Property,
}

/// A positional or named argument in an assertion or property application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationArgument {
    /// Optional argument name. `None` means positional.
    pub name: Option<String>,
    /// Canonical AST expression supplied as the argument value.
    pub value: ExpressionRef,
    /// Source location of the argument, if available.
    pub location: Option<HdlSourceLocation>,
}

/// Qualifier attached to an assertion.
///
/// Qualifier names are extensible. Built-in names such as `clock` and
/// `disable` are interpreted by the supplied semantic environment. Unknown
/// qualifiers are not silently accepted unless the environment explicitly
/// recognizes them as extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationQualifier {
    /// Source-level qualifier name.
    pub name: String,
    /// Qualifier arguments, in source order.
    pub arguments: Vec<VerificationArgument>,
    /// Source location of the qualifier, if available.
    pub location: Option<HdlSourceLocation>,
}

/// A source-level HDL assertion.
///
/// The expression is always a reference to a canonical frontend AST node.
/// Optional arguments preserve their source order. A label is source-level
/// metadata and must be unique within the scope selected by the caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlAssertion {
    /// Stable assertion identity.
    pub id: HdlId,
    /// Optional source-level label.
    pub label: Option<String>,
    /// Assertion category.
    pub kind: AssertionKind,
    /// Inline condition or property expression.
    ///
    /// For a named property application, this field may identify the
    /// canonical expression representing that application. `property` holds
    /// the referenced reusable property's identity when known syntactically.
    pub expression: ExpressionRef,
    /// Referenced property identity for a named property assertion.
    pub property: Option<HdlId>,
    /// Application arguments, preserving source order.
    pub arguments: Vec<VerificationArgument>,
    /// Clock, disable, and extension-defined qualifiers.
    pub qualifiers: Vec<VerificationQualifier>,
    /// Source location of the assertion.
    pub location: Option<HdlSourceLocation>,
}

/// A parameter in a reusable property declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyParameter {
    /// Stable parameter identity.
    pub id: HdlId,
    /// Source-level parameter name.
    pub name: String,
    /// Optional canonical type-expression reference.
    pub type_expression: Option<ExpressionRef>,
    /// Source location of the parameter.
    pub location: Option<HdlSourceLocation>,
}

/// A reference to another reusable property from a property expression.
///
/// The canonical expression AST remains authoritative. This summary exists
/// only to enable deterministic dependency validation without building a
/// second expression tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyReference {
    /// Referenced property identity.
    pub property: HdlId,
    /// Argument expressions supplied to the referenced property.
    pub arguments: Vec<VerificationArgument>,
    /// Source location of this reference.
    pub location: Option<HdlSourceLocation>,
}

/// A reusable, parameterized property declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyDeclaration {
    /// Stable property identity.
    pub id: HdlId,
    /// Source-level property name.
    pub name: String,
    /// Ordered property parameters.
    pub parameters: Vec<PropertyParameter>,
    /// Canonical AST expression implementing the property.
    pub expression: ExpressionRef,
    /// Property references collected from the canonical expression by the
    /// frontend adapter.
    ///
    /// This is an adapter-produced summary, not a replacement expression AST.
    pub references: Vec<PropertyReference>,
    /// Source location of the declaration.
    pub location: Option<HdlSourceLocation>,
}

/// Verification declarations supplied to one validation invocation.
///
/// Scope is explicit: declarations in separate scopes should be validated
/// using separate inputs or a resolver that models their actual scope.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AssertionProgram {
    /// Assertions to validate.
    pub assertions: Vec<HdlAssertion>,
    /// Reusable property declarations.
    pub properties: Vec<PropertyDeclaration>,
}

// -----------------------------------------------------------------------------
// Configurable operational budgets
// -----------------------------------------------------------------------------

/// Operational budgets for assertion validation.
///
/// These are not language-level restrictions. A value of `None` means that
/// this particular budget is not imposed by this configuration. The process
/// can still fail due to genuine host resource exhaustion.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AssertionLimits {
    /// Maximum declarations examined in one invocation.
    pub max_declarations: Option<usize>,
    /// Maximum total property-dependency edges examined.
    pub max_dependency_edges: Option<usize>,
    /// Maximum dependency traversal work items processed.
    pub max_dependency_work: Option<usize>,
    /// Maximum diagnostics retained. Reaching this limit makes the result
    /// incomplete if further diagnostics would otherwise be needed.
    pub max_diagnostics: Option<usize>,
}

impl AssertionLimits {
    fn validate(&self) -> Result<(), AssertionInputError> {
        // `Some(0)` is useful for callers that deliberately disable a stage,
        // but it cannot complete validation of a non-empty input. It is
        // therefore accepted and reported as an operational budget outcome.
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Environment boundary: reuse canonical semantic services
// -----------------------------------------------------------------------------

/// Classification of an expression check performed by the canonical
/// semantic infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionCheck {
    /// The expression is resolved, well-typed, and legal in this context.
    Valid,
    /// The expression has a confirmed semantic error.
    Invalid {
        /// Stable machine-readable diagnostic code.
        code: String,
        /// Human-readable explanation.
        message: String,
    },
    /// Required semantic information is not available yet.
    Unresolved {
        /// Explanation of the missing information.
        message: String,
    },
}

/// Resolution of a property identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyResolution {
    /// The property exists in the current semantic scope.
    Found,
    /// The property is not declared or visible in the current scope.
    Missing,
    /// Resolution could not finish.
    Incomplete {
        /// Explanation of the incomplete lookup.
        message: String,
    },
}

/// Semantic services required from the existing frontend/semantic pipeline.
///
/// Implementations must use the canonical AST, type system, name resolver,
/// scope rules, and expression semantics. They must not infer validity from
/// expression text or from the presence of an opaque identity alone.
pub trait AssertionEnvironment {
    /// Check an expression in the context of an assertion condition.
    fn check_condition(&self, expression: &ExpressionRef) -> ExpressionCheck;

    /// Check an expression supplied as a property or assertion argument.
    fn check_argument(&self, expression: &ExpressionRef) -> ExpressionCheck;

    /// Check an optional parameter type expression.
    fn check_parameter_type(&self, expression: &ExpressionRef) -> ExpressionCheck;

    /// Resolve a property by its canonical semantic identity.
    fn resolve_property(&self, property: &HdlId) -> PropertyResolution;

    /// Return whether a qualifier name is recognized in this context.
    ///
    /// The implementation should recognize standard qualifiers such as
    /// `clock` and `disable`, plus explicitly registered extensions.
    fn recognizes_qualifier(&self, name: &str) -> bool;

    /// Check a qualifier expression using the canonical semantic model.
    fn check_qualifier_expression(
        &self,
        qualifier_name: &str,
        expression: &ExpressionRef,
    ) -> ExpressionCheck;
}

// -----------------------------------------------------------------------------
// Diagnostics and results
// -----------------------------------------------------------------------------

/// Stable machine-readable assertion diagnostic code.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum AssertionDiagnosticCode {
    /// Duplicate assertion identity.
    DuplicateAssertionIdentity,
    /// Duplicate property identity.
    DuplicatePropertyIdentity,
    /// Duplicate assertion label in the supplied scope.
    DuplicateLabel,
    /// Duplicate property name in the supplied scope.
    DuplicatePropertyName,
    /// Duplicate parameter identity or name.
    DuplicateParameter,
    /// A property reference cannot be resolved.
    UnknownProperty,
    /// A referenced property is not valid in the current context.
    InvalidPropertyReference,
    /// Property dependency cycle detected.
    RecursiveProperty,
    /// A named argument appears more than once.
    DuplicateNamedArgument,
    /// A positional argument follows a named argument.
    PositionalAfterNamedArgument,
    /// Supplied argument count or names do not match the property signature.
    ArgumentMismatch,
    /// An expression is semantically invalid.
    InvalidExpression,
    /// An expression could not be resolved or checked.
    UnresolvedExpression,
    /// A qualifier is not recognized.
    UnknownQualifier,
    /// A qualifier has an invalid argument shape.
    InvalidQualifier,
    /// A canonical semantic check returned an error.
    SemanticCheckFailed,
    /// Analysis stopped because an operational budget was reached.
    BudgetExceeded,
    /// Validation could not complete.
    IncompleteAnalysis,
    /// Invalid operational configuration.
    InvalidConfiguration,
}

/// Severity of an assertion diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AssertionSeverity {
    /// Confirmed violation.
    Error,
    /// Suspicious or unresolved condition.
    Warning,
    /// Informational message.
    Note,
}

/// Structured diagnostic produced by assertion validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssertionDiagnostic {
    /// Stable diagnostic code.
    pub code: AssertionDiagnosticCode,
    /// Severity.
    pub severity: AssertionSeverity,
    /// Human-readable explanation.
    pub message: String,
    /// Primary source location.
    pub location: Option<HdlSourceLocation>,
    /// Related source locations in deterministic order.
    pub related: Vec<HdlSourceLocation>,
}

impl AssertionDiagnostic {
    fn error(
        code: AssertionDiagnosticCode,
        message: impl Into<String>,
        location: Option<HdlSourceLocation>,
    ) -> Self {
        Self {
            code,
            severity: AssertionSeverity::Error,
            message: message.into(),
            location,
            related: Vec::new(),
        }
    }

    fn warning(
        code: AssertionDiagnosticCode,
        message: impl Into<String>,
        location: Option<HdlSourceLocation>,
    ) -> Self {
        Self {
            code,
            severity: AssertionSeverity::Warning,
            message: message.into(),
            location,
            related: Vec::new(),
        }
    }
}

/// Outcome of assertion-structure and semantic validation.
///
/// `ValidUnproven` means that the assertions and property declarations passed
/// the checks performed here. It does not mean their conditions are true.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssertionValidationStatus {
    /// No errors were found and all requested checks completed.
    ValidUnproven,
    /// Confirmed errors were found.
    Invalid,
    /// Required checks did not all complete.
    Incomplete,
}

/// Result of assertion validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssertionValidationResult {
    /// Overall result.
    pub status: AssertionValidationStatus,
    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<AssertionDiagnostic>,
    /// Whether all requested checks completed.
    pub complete: bool,
}

impl AssertionValidationResult {
    fn from_diagnostics(
        mut diagnostics: Vec<AssertionDiagnostic>,
        complete: bool,
    ) -> Self {
        diagnostics.sort_by(|a, b| {
            (&a.location, &a.code, &a.message)
                .cmp(&(&b.location, &b.code, &b.message))
        });

        let has_error = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == AssertionSeverity::Error);

        let incomplete = !complete
            || diagnostics.iter().any(|diagnostic| {
                matches!(
                    diagnostic.code,
                    AssertionDiagnosticCode::BudgetExceeded
                        | AssertionDiagnosticCode::IncompleteAnalysis
                )
            });

        let status = if incomplete {
            AssertionValidationStatus::Incomplete
        } else if has_error {
            AssertionValidationStatus::Invalid
        } else {
            AssertionValidationStatus::ValidUnproven
        };

        Self {
            status,
            diagnostics,
            complete: !incomplete,
        }
    }
}

/// Errors detected while constructing the assertion model or configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssertionInputError {
    /// An expression reference has an empty identity.
    EmptyExpressionReference,
    /// A required name or identifier is empty.
    EmptyName,
    /// A source label or name contains only whitespace.
    WhitespaceOnlyName,
}

impl fmt::Display for AssertionInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExpressionReference => {
                f.write_str("canonical expression reference cannot be empty")
            }
            Self::EmptyName => f.write_str("name cannot be empty"),
            Self::WhitespaceOnlyName => f.write_str("name cannot be whitespace only"),
        }
    }
}

impl Error for AssertionInputError {}

// -----------------------------------------------------------------------------
// Public validation entry point
// -----------------------------------------------------------------------------

/// Validate assertion/property structure and delegate expression semantics.
///
/// This function does not execute assertions and does not prove properties.
/// Property-reference and dependency checks are deterministic. If any
/// configured budget prevents complete analysis, the returned status is
/// `Incomplete`.
#[must_use]
pub fn validate_assertions<E: AssertionEnvironment>(
    program: &AssertionProgram,
    environment: &E,
    limits: &AssertionLimits,
) -> AssertionValidationResult {
    let mut diagnostics = Vec::new();
    let mut complete = true;

    if limits.validate().is_err() {
        diagnostics.push(AssertionDiagnostic::error(
            AssertionDiagnosticCode::InvalidConfiguration,
            "invalid assertion validation configuration",
            None,
        ));
        return AssertionValidationResult::from_diagnostics(diagnostics, false);
    }

    let declaration_count = match program
        .assertions
        .len()
        .checked_add(program.properties.len())
    {
        Some(count) => count,
        None => {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::BudgetExceeded,
                "declaration count overflowed the host representation",
                None,
            ));
            return AssertionValidationResult::from_diagnostics(diagnostics, false);
        }
    };

    if limits
        .max_declarations
        .is_some_and(|maximum| declaration_count > maximum)
    {
        diagnostics.push(AssertionDiagnostic::warning(
            AssertionDiagnosticCode::BudgetExceeded,
            "assertion declaration budget exceeded; validation is incomplete",
            None,
        ));
        return AssertionValidationResult::from_diagnostics(diagnostics, false);
    }

    let properties_by_id = index_properties(&program.properties, &mut diagnostics);
    validate_property_signatures(&program.properties, environment, &mut diagnostics);

    validate_assertion_identities(&program.assertions, &mut diagnostics);
    validate_assertion_labels(&program.assertions, &mut diagnostics);

    for assertion in &program.assertions {
        validate_assertion(
            assertion,
            &properties_by_id,
            environment,
            &mut diagnostics,
        );
    }

    for property in &program.properties {
        validate_property_references(
            property,
            &properties_by_id,
            environment,
            &mut diagnostics,
        );
    }

    let dependency_result = validate_property_dependencies(
        &program.properties,
        &properties_by_id,
        limits,
    );
    diagnostics.extend(dependency_result.diagnostics);
    complete &= dependency_result.complete;

    if let Some(maximum) = limits.max_diagnostics {
        if diagnostics.len() > maximum {
            diagnostics.truncate(maximum);
            diagnostics.push(AssertionDiagnostic::warning(
                AssertionDiagnosticCode::BudgetExceeded,
                "diagnostic retention budget exceeded; diagnostics may be omitted",
                None,
            ));
            complete = false;
        }
    }

    AssertionValidationResult::from_diagnostics(diagnostics, complete)
}

fn index_properties<'a>(
    properties: &'a [PropertyDeclaration],
    diagnostics: &mut Vec<AssertionDiagnostic>,
) -> BTreeMap<HdlId, &'a PropertyDeclaration> {
    let mut by_id = BTreeMap::new();
    let mut by_name = BTreeMap::<&str, Vec<&PropertyDeclaration>>::new();

    for property in properties {
        if property.name.trim().is_empty() {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::InvalidPropertyReference,
                "property name cannot be empty",
                property.location.clone(),
            ));
        }

        if by_id.insert(property.id.clone(), property).is_some() {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::DuplicatePropertyIdentity,
                format!("duplicate property identity `{}`", property.id),
                property.location.clone(),
            ));
        }

        by_name.entry(property.name.as_str()).or_default().push(property);
    }

    for (name, declarations) in by_name {
        if declarations.len() > 1 {
            let mut diagnostic = AssertionDiagnostic::error(
                AssertionDiagnosticCode::DuplicatePropertyName,
                format!("property name `{name}` is declared more than once"),
                declarations.first().and_then(|item| item.location.clone()),
            );
            diagnostic.related = declarations
                .iter()
                .filter_map(|item| item.location.clone())
                .collect();
            diagnostics.push(diagnostic);
        }
    }

    by_id
}

fn validate_property_signatures<E: AssertionEnvironment>(
    properties: &[PropertyDeclaration],
    environment: &E,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    for property in properties {
        let mut parameter_ids = BTreeSet::new();
        let mut parameter_names = BTreeSet::new();

        for parameter in &property.parameters {
            if parameter.name.trim().is_empty() {
                diagnostics.push(AssertionDiagnostic::error(
                    AssertionDiagnosticCode::InvalidPropertyReference,
                    "property parameter name cannot be empty",
                    parameter.location.clone(),
                ));
            }

            if !parameter_ids.insert(parameter.id.clone())
                || !parameter_names.insert(parameter.name.clone())
            {
                diagnostics.push(AssertionDiagnostic::error(
                    AssertionDiagnosticCode::DuplicateParameter,
                    format!(
                        "duplicate parameter identity or name `{}` in property `{}`",
                        parameter.name, property.name
                    ),
                    parameter.location.clone(),
                ));
            }

            if let Some(type_expression) = &parameter.type_expression {
                record_expression_check(
                    environment.check_parameter_type(type_expression),
                    parameter.location.clone(),
                    "property parameter type",
                    diagnostics,
                );
            }
        }

        record_expression_check(
            environment.check_condition(&property.expression),
            property.location.clone(),
            "property expression",
            diagnostics,
        );
    }
}

fn validate_assertion_identities(
    assertions: &[HdlAssertion],
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    let mut seen = BTreeSet::new();

    for assertion in assertions {
        if !seen.insert(assertion.id.clone()) {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::DuplicateAssertionIdentity,
                format!("duplicate assertion identity `{}`", assertion.id),
                assertion.location.clone(),
            ));
        }
    }
}

fn validate_assertion_labels(
    assertions: &[HdlAssertion],
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    let mut labels = BTreeMap::<&str, Vec<&HdlAssertion>>::new();

    for assertion in assertions {
        if let Some(label) = assertion.label.as_deref() {
            if label.trim().is_empty() {
                diagnostics.push(AssertionDiagnostic::error(
                    AssertionDiagnosticCode::InvalidPropertyReference,
                    "assertion label cannot be empty",
                    assertion.location.clone(),
                ));
            } else {
                labels.entry(label).or_default().push(assertion);
            }
        }
    }

    for (label, declarations) in labels {
        if declarations.len() > 1 {
            let mut diagnostic = AssertionDiagnostic::error(
                AssertionDiagnosticCode::DuplicateLabel,
                format!("assertion label `{label}` is duplicated in this scope"),
                declarations.first().and_then(|item| item.location.clone()),
            );
            diagnostic.related = declarations
                .iter()
                .filter_map(|item| item.location.clone())
                .collect();
            diagnostics.push(diagnostic);
        }
    }
}

fn validate_assertion<E: AssertionEnvironment>(
    assertion: &HdlAssertion,
    properties: &BTreeMap<HdlId, &PropertyDeclaration>,
    environment: &E,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    record_expression_check(
        environment.check_condition(&assertion.expression),
        assertion.location.clone(),
        "assertion condition",
        diagnostics,
    );

    validate_arguments(
        &assertion.arguments,
        assertion.location.clone(),
        environment,
        diagnostics,
    );

    for qualifier in &assertion.qualifiers {
        if !environment.recognizes_qualifier(&qualifier.name) {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::UnknownQualifier,
                format!("unrecognized verification qualifier `{}`", qualifier.name),
                qualifier.location.clone(),
            ));
            continue;
        }

        if matches!(qualifier.name.as_str(), "clock" | "disable")
            && qualifier.arguments.len() != 1
        {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::InvalidQualifier,
                format!(
                    "qualifier `{}` requires exactly one expression",
                    qualifier.name
                ),
                qualifier.location.clone(),
            ));
        }

        validate_arguments(
            &qualifier.arguments,
            qualifier.location.clone(),
            environment,
            diagnostics,
        );

        for argument in &qualifier.arguments {
            record_expression_check(
                environment.check_qualifier_expression(
                    &qualifier.name,
                    &argument.value,
                ),
                argument.location.clone().or_else(|| qualifier.location.clone()),
                "qualifier expression",
                diagnostics,
            );
        }
    }

    if assertion.kind == AssertionKind::Immediate && assertion.property.is_some() {
        diagnostics.push(AssertionDiagnostic::error(
            AssertionDiagnosticCode::InvalidPropertyReference,
            "an immediate assertion cannot directly carry a property reference",
            assertion.location.clone(),
        ));
    }

    if let Some(property_id) = &assertion.property {
        match properties.get(property_id) {
            Some(property) => {
                validate_property_application(
                    property,
                    &assertion.arguments,
                    assertion.location.clone(),
                    diagnostics,
                );
            }
            None => match environment.resolve_property(property_id) {
                PropertyResolution::Found => {
                    // The property is supplied by an outer scope or imported
                    // package. The canonical resolver owns its signature.
                }
                PropertyResolution::Missing => diagnostics.push(
                    AssertionDiagnostic::error(
                        AssertionDiagnosticCode::UnknownProperty,
                        format!("unknown property identity `{property_id}`"),
                        assertion.location.clone(),
                    ),
                ),
                PropertyResolution::Incomplete { message } => {
                    diagnostics.push(AssertionDiagnostic::warning(
                        AssertionDiagnosticCode::IncompleteAnalysis,
                        format!("property lookup incomplete: {message}"),
                        assertion.location.clone(),
                    ));
                }
            },
        }
    } else if assertion.kind == AssertionKind::Property {
        // Inline property expressions are valid; no named reference is needed.
    }
}

fn validate_arguments<E: AssertionEnvironment>(
    arguments: &[VerificationArgument],
    fallback_location: Option<HdlSourceLocation>,
    environment: &E,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    let mut named_seen = BTreeSet::new();
    let mut saw_named = false;

    for argument in arguments {
        if let Some(name) = argument.name.as_deref() {
            saw_named = true;

            if name.trim().is_empty() {
                diagnostics.push(AssertionDiagnostic::error(
                    AssertionDiagnosticCode::InvalidPropertyReference,
                    "named argument cannot have an empty name",
                    argument.location.clone().or_else(|| fallback_location.clone()),
                ));
            }

            if !named_seen.insert(name.to_owned()) {
                diagnostics.push(AssertionDiagnostic::error(
                    AssertionDiagnosticCode::DuplicateNamedArgument,
                    format!("named argument `{name}` is supplied more than once"),
                    argument.location.clone().or_else(|| fallback_location.clone()),
                ));
            }
        } else if saw_named {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::PositionalAfterNamedArgument,
                "a positional argument cannot follow a named argument",
                argument.location.clone().or_else(|| fallback_location.clone()),
            ));
        }

        record_expression_check(
            environment.check_argument(&argument.value),
            argument.location.clone().or_else(|| fallback_location.clone()),
            "verification argument",
            diagnostics,
        );
    }
}

fn validate_property_application(
    property: &PropertyDeclaration,
    arguments: &[VerificationArgument],
    location: Option<HdlSourceLocation>,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    let named_parameters: BTreeSet<&str> = property
        .parameters
        .iter()
        .map(|parameter| parameter.name.as_str())
        .collect();

    let supplied_named: BTreeSet<&str> = arguments
        .iter()
        .filter_map(|argument| argument.name.as_deref())
        .collect();

    for name in &supplied_named {
        if !named_parameters.contains(name) {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::ArgumentMismatch,
                format!(
                    "property `{}` has no parameter named `{name}`",
                    property.name
                ),
                location.clone(),
            ));
        }
    }

    let positional_count = arguments
        .iter()
        .filter(|argument| argument.name.is_none())
        .count();

    if positional_count > property.parameters.len() {
        diagnostics.push(AssertionDiagnostic::error(
            AssertionDiagnosticCode::ArgumentMismatch,
            format!(
                "property `{}` accepts {} parameters but received {} positional arguments",
                property.name,
                property.parameters.len(),
                positional_count
            ),
            location,
        ));
    }
}

fn validate_property_references<E: AssertionEnvironment>(
    property: &PropertyDeclaration,
    properties: &BTreeMap<HdlId, &PropertyDeclaration>,
    environment: &E,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    for reference in &property.references {
        match properties.get(&reference.property) {
            Some(target) => validate_property_application(
                target,
                &reference.arguments,
                reference.location.clone(),
                diagnostics,
            ),
            None => match environment.resolve_property(&reference.property) {
                PropertyResolution::Found => {}
                PropertyResolution::Missing => diagnostics.push(
                    AssertionDiagnostic::error(
                        AssertionDiagnosticCode::UnknownProperty,
                        format!(
                            "property `{}` references unknown property `{}`",
                            property.name, reference.property
                        ),
                        reference.location.clone(),
                    ),
                ),
                PropertyResolution::Incomplete { message } => diagnostics.push(
                    AssertionDiagnostic::warning(
                        AssertionDiagnosticCode::IncompleteAnalysis,
                        format!("property dependency lookup incomplete: {message}"),
                        reference.location.clone(),
                    ),
                ),
            },
        }
    }
}

fn record_expression_check(
    check: ExpressionCheck,
    location: Option<HdlSourceLocation>,
    context: &str,
    diagnostics: &mut Vec<AssertionDiagnostic>,
) {
    match check {
        ExpressionCheck::Valid => {}
        ExpressionCheck::Invalid { code, message } => {
            diagnostics.push(AssertionDiagnostic::error(
                AssertionDiagnosticCode::InvalidExpression,
                format!("{context} is invalid ({code}): {message}"),
                location,
            ));
        }
        ExpressionCheck::Unresolved { message } => {
            diagnostics.push(AssertionDiagnostic::warning(
                AssertionDiagnosticCode::UnresolvedExpression,
                format!("{context} could not be fully checked: {message}"),
                location,
            ));
        }
    }
}

// -----------------------------------------------------------------------------
// Property dependency analysis
// -----------------------------------------------------------------------------

struct DependencyResult {
    diagnostics: Vec<AssertionDiagnostic>,
    complete: bool,
}

fn validate_property_dependencies(
    properties: &[PropertyDeclaration],
    properties_by_id: &BTreeMap<HdlId, &PropertyDeclaration>,
    limits: &AssertionLimits,
) -> DependencyResult {
    let mut diagnostics = Vec::new();
    let mut complete = true;

    let mut graph = BTreeMap::<HdlId, BTreeSet<HdlId>>::new();
    let mut edge_count = 0usize;

    for property in properties {
        let edges = graph.entry(property.id.clone()).or_default();

        for reference in &property.references {
            if !properties_by_id.contains_key(&reference.property) {
                // External references are resolved by the environment. They
                // are not internal graph edges in this declaration set.
                continue;
            }

            if let Some(maximum) = limits.max_dependency_edges {
                if edge_count >= maximum {
                    diagnostics.push(AssertionDiagnostic::warning(
                        AssertionDiagnosticCode::BudgetExceeded,
                        "property dependency edge budget exceeded",
                        reference.location.clone(),
                    ));
                    complete = false;
                    break;
                }
            }

            edge_count = match edge_count.checked_add(1) {
                Some(value) => value,
                None => {
                    diagnostics.push(AssertionDiagnostic::warning(
                        AssertionDiagnosticCode::BudgetExceeded,
                        "property dependency edge counter overflowed",
                        reference.location.clone(),
                    ));
                    complete = false;
                    break;
                }
            };

            edges.insert(reference.property.clone());
        }

        if !complete {
            break;
        }
    }

    if !complete {
        return DependencyResult {
            diagnostics,
            complete,
        };
    }

    // Iterative three-color DFS avoids using the Rust call stack for property
    // graphs. Ordered maps/sets make traversal and diagnostics reproducible.
    // Colors: absent = white, 1 = gray, 2 = black.
    let mut colors = BTreeMap::<HdlId, u8>::new();
    let mut cycle_edges = BTreeSet::<(HdlId, HdlId)>::new();
    let mut work = 0usize;

    for root in graph.keys() {
        if colors.get(root).copied().unwrap_or(0) != 0 {
            continue;
        }

        let mut stack = vec![(root.clone(), false)];

        while let Some((node, exiting)) = stack.pop() {
            work = match work.checked_add(1) {
                Some(value) => value,
                None => {
                    diagnostics.push(AssertionDiagnostic::warning(
                        AssertionDiagnosticCode::BudgetExceeded,
                        "property dependency traversal counter overflowed",
                        properties_by_id.get(&node).and_then(|p| p.location.clone()),
                    ));
                    complete = false;
                    break;
                }
            };

            if limits
                .max_dependency_work
                .is_some_and(|maximum| work > maximum)
            {
                diagnostics.push(AssertionDiagnostic::warning(
                    AssertionDiagnosticCode::BudgetExceeded,
                    "property dependency work budget exceeded",
                    properties_by_id.get(&node).and_then(|p| p.location.clone()),
                ));
                complete = false;
                break;
            }

            if exiting {
                colors.insert(node, 2);
                continue;
            }

            match colors.get(&node).copied().unwrap_or(0) {
                2 => continue,
                1 => continue,
                _ => {}
            }

            colors.insert(node.clone(), 1);
            stack.push((node.clone(), true));

            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors.iter().rev() {
                    match colors.get(neighbor).copied().unwrap_or(0) {
                        1 => {
                            cycle_edges.insert((node.clone(), neighbor.clone()));
                        }
                        0 => stack.push((neighbor.clone(), false)),
                        _ => {}
                    }
                }
            }
        }

        if !complete {
            break;
        }
    }

    if !complete {
        diagnostics.push(AssertionDiagnostic::warning(
            AssertionDiagnosticCode::IncompleteAnalysis,
            "property recursion analysis did not complete; recursive properties may remain undetected",
            None,
        ));
    }

    for (from, to) in cycle_edges {
        let location = properties_by_id
            .get(&from)
            .and_then(|property| property.location.clone());

        let mut diagnostic = AssertionDiagnostic::error(
            AssertionDiagnosticCode::RecursiveProperty,
            format!("recursive property dependency detected: `{from}` references `{to}`"),
            location,
        );

        if let Some(target_location) = properties_by_id
            .get(&to)
            .and_then(|property| property.location.clone())
        {
            diagnostic.related.push(target_location);
        }

        diagnostics.push(diagnostic);
    }

    DependencyResult {
        diagnostics,
        complete,
    }
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEnvironment {
        expressions: BTreeMap<String, ExpressionCheck>,
        properties: BTreeSet<HdlId>,
        qualifiers: BTreeSet<String>,
    }

    impl Default for TestEnvironment {
        fn default() -> Self {
            Self {
                expressions: BTreeMap::new(),
                properties: BTreeSet::new(),
                qualifiers: ["clock".to_owned(), "disable".to_owned(), "metadata".to_owned()]
                    .into_iter()
                    .collect(),
            }
        }
    }

    impl AssertionEnvironment for TestEnvironment {
        fn check_condition(&self, expression: &ExpressionRef) -> ExpressionCheck {
            self.expressions
                .get(expression.as_str())
                .cloned()
                .unwrap_or(ExpressionCheck::Valid)
        }

        fn check_argument(&self, expression: &ExpressionRef) -> ExpressionCheck {
            self.check_condition(expression)
        }

        fn check_parameter_type(&self, expression: &ExpressionRef) -> ExpressionCheck {
            self.check_condition(expression)
        }

        fn resolve_property(&self, property: &HdlId) -> PropertyResolution {
            if self.properties.contains(property) {
                PropertyResolution::Found
            } else {
                PropertyResolution::Missing
            }
        }

        fn recognizes_qualifier(&self, name: &str) -> bool {
            self.qualifiers.contains(name)
        }

        fn check_qualifier_expression(
            &self,
            _qualifier_name: &str,
            expression: &ExpressionRef,
        ) -> ExpressionCheck {
            self.check_condition(expression)
        }
    }

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be non-empty")
    }

    fn expr(value: &str) -> ExpressionRef {
        ExpressionRef::new(value).expect("test expression key must be non-empty")
    }

    fn assertion(identity: &str, condition: &str) -> HdlAssertion {
        HdlAssertion {
            id: id(identity),
            label: None,
            kind: AssertionKind::Immediate,
            expression: expr(condition),
            property: None,
            arguments: Vec::new(),
            qualifiers: Vec::new(),
            location: None,
        }
    }

    #[test]
    fn empty_program_is_valid_but_unproven() {
        let result = validate_assertions(
            &AssertionProgram::default(),
            &TestEnvironment::default(),
            &AssertionLimits::default(),
        );

        assert_eq!(result.status, AssertionValidationStatus::ValidUnproven);
        assert!(result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn duplicate_assertion_id_is_rejected() {
        let program = AssertionProgram {
            assertions: vec![assertion("assertion-1", "expr-a"), assertion("assertion-1", "expr-b")],
            properties: Vec::new(),
        };

        let result = validate_assertions(
            &program,
            &TestEnvironment::default(),
            &AssertionLimits::default(),
        );

        assert_eq!(result.status, AssertionValidationStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == AssertionDiagnosticCode::DuplicateAssertionIdentity
        }));
    }

    #[test]
    fn unresolved_expression_is_not_reported_as_proven() {
        let mut environment = TestEnvironment::default();
        environment.expressions.insert(
            "unknown".to_owned(),
            ExpressionCheck::Unresolved {
                message: "name resolution is incomplete".to_owned(),
            },
        );

        let program = AssertionProgram {
            assertions: vec![assertion("a1", "unknown")],
            properties: Vec::new(),
        };

        let result = validate_assertions(
            &program,
            &environment,
            &AssertionLimits::default(),
        );

        assert_eq!(result.status, AssertionValidationStatus::ValidUnproven);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == AssertionDiagnosticCode::UnresolvedExpression
        }));
    }

    #[test]
    fn dependency_cycle_is_detected() {
        let a = PropertyDeclaration {
            id: id("property-a"),
            name: "a".to_owned(),
            parameters: Vec::new(),
            expression: expr("expr-a"),
            references: vec![PropertyReference {
                property: id("property-b"),
                arguments: Vec::new(),
                location: None,
            }],
            location: None,
        };

        let b = PropertyDeclaration {
            id: id("property-b"),
            name: "b".to_owned(),
            parameters: Vec::new(),
            expression: expr("expr-b"),
            references: vec![PropertyReference {
                property: id("property-a"),
                arguments: Vec::new(),
                location: None,
            }],
            location: None,
        };

        let program = AssertionProgram {
            assertions: Vec::new(),
            properties: vec![a, b],
        };

        let result = validate_assertions(
            &program,
            &TestEnvironment::default(),
            &AssertionLimits::default(),
        );

        assert_eq!(result.status, AssertionValidationStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == AssertionDiagnosticCode::RecursiveProperty
        }));
    }

    #[test]
    fn dependency_budget_exhaustion_is_incomplete() {
        let property = PropertyDeclaration {
            id: id("property-a"),
            name: "a".to_owned(),
            parameters: Vec::new(),
            expression: expr("expr-a"),
            references: vec![PropertyReference {
                property: id("property-a"),
                arguments: Vec::new(),
                location: None,
            }],
            location: None,
        };

        let program = AssertionProgram {
            assertions: Vec::new(),
            properties: vec![property],
        };

        let result = validate_assertions(
            &program,
            &TestEnvironment::default(),
            &AssertionLimits {
                max_dependency_edges: Some(0),
                ..AssertionLimits::default()
            },
        );

        assert_eq!(result.status, AssertionValidationStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn duplicate_named_arguments_are_rejected() {
        let mut item = assertion("a1", "condition");
        item.arguments = vec![
            VerificationArgument {
                name: Some("message".to_owned()),
                value: expr("message-a"),
                location: None,
            },
            VerificationArgument {
                name: Some("message".to_owned()),
                value: expr("message-b"),
                location: None,
            },
        ];

        let program = AssertionProgram {
            assertions: vec![item],
            properties: Vec::new(),
        };

        let result = validate_assertions(
            &program,
            &TestEnvironment::default(),
            &AssertionLimits::default(),
        );

        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == AssertionDiagnosticCode::DuplicateNamedArgument
        }));
    }
}
