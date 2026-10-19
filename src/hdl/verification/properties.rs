
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Property Specifications
//!
//! Path: `src/hdl/verification/properties.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, source-level HDL properties and the structural
//! validation required before properties are submitted to a verification
//! engine.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its verification-related delegates
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - The canonical frontend AST and semantic model
//!
//! The grammar owns syntax. The canonical AST owns source structure.
//! Shared semantic infrastructure owns expression typing, name resolution,
//! effects, resources, capabilities, contracts, and provenance.
//!
//! ## Owns
//!
//! - Stable property identifiers and property metadata.
//! - References to canonical semantic expressions.
//! - Portable temporal-property structure.
//! - Explicit temporal bounds and clock references.
//! - Property declaration validation.
//! - Structural validation outcomes and actionable validation errors.
//!
//! ## Does not own
//!
//! - A replacement AST, expression parser, or expression evaluator.
//! - Expression type checking or symbol resolution.
//! - A formal proof engine, SAT/SMT solver, or simulation engine.
//! - HDL intermediate representation or its transformations.
//! - Physical clocks, clock-tree synthesis, placement, or routing.
//! - A claim that a structurally valid property has been proven.
//!
//! ## Integration
//!
//! The AST adapter maps source expressions to `ExpressionRef` values derived
//! from canonical frontend identities. The semantic pipeline resolves those
//! references and checks expression types before verification.
//!
//! `assertions.rs` may use `Property` as the common declarative input for
//! assertions. `invariants.rs` may consume properties classified as
//! invariants. A future verification engine interprets `TemporalFormula`
//! according to the language's normative temporal semantics.
//!
//! These modules must not create competing property, expression, or source
//! identity systems.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum on property count, expression
//! reference length, temporal bounds, or formula size. It does not impose a
//! fixed machine-word width on temporal bounds.
//!
//! Callers may enforce configurable operational budgets while parsing,
//! validating, or proving properties. Exhausting a budget must produce an
//! incomplete result, never a successful proof.
//!
//! ## Completion contract
//!
//! A property is structurally valid only when its identifiers, formula
//! structure, temporal bounds, and optional clock/reset references satisfy
//! this module's rules. Structural validity does not imply that expressions
//! resolve, that the formula is satisfiable, or that a design satisfies it.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

/// A validated, stable identifier used by the HDL property subsystem.
///
/// The identifier is a portable semantic name, not a physical resource
/// number, memory address, or target-specific handle.
///
/// The frontend adapter should derive IDs from canonical declaration
/// identities or another documented, deterministic identity scheme.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PropertyId(String);

impl PropertyId {
    /// Creates an identifier from a non-empty string.
    ///
    /// This checks only the identifier's basic representation. Language
    /// identifier syntax and scope rules remain the responsibility of the
    /// canonical parser and semantic analyzer.
    pub fn new(value: impl Into<String>) -> Result<Self, PropertyError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PropertyError::EmptyIdentifier {
                field: "property identifier",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier's stable textual representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PropertyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A reference to an expression owned by the canonical frontend AST or
/// semantic model.
///
/// The referenced expression is not copied into this module. The adapter
/// must guarantee that the reference resolves in the compilation context
/// in which the property is validated.
///
/// The string representation is intentionally opaque to this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionRef(String);

impl ExpressionRef {
    /// Creates an expression reference from a non-empty identity.
    pub fn new(value: impl Into<String>) -> Result<Self, PropertyError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PropertyError::EmptyIdentifier {
                field: "expression reference",
            });
        }

        Ok(Self(value))
    }

    /// Returns the opaque expression identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExpressionRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A reference to a clock declaration in the canonical HDL semantic model.
///
/// This represents logical clock identity, not a physical clock pin,
/// device clock index, or synthesized clock resource.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClockRef(String);

impl ClockRef {
    /// Creates a clock reference from a non-empty identity.
    pub fn new(value: impl Into<String>) -> Result<Self, PropertyError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PropertyError::EmptyIdentifier {
                field: "clock reference",
            });
        }

        Ok(Self(value))
    }

    /// Returns the opaque clock identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClockRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A reference to a reset or disable condition represented by a canonical
/// frontend expression.
///
/// This is an expression reference, not a physical reset-network identity.
pub type DisableConditionRef = ExpressionRef;

/// An inclusive range of temporal distances measured in the property's
/// declared logical clock domain.
///
/// The values are `u128` so that the API does not assume temporal bounds
/// fit into `usize` or the width of a hardware register. The type is still
/// finite: bounds larger than `u128::MAX` require a future arbitrary-precision
/// representation and must not be truncated to fit this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TemporalRange {
    /// Earliest permitted temporal distance, inclusive.
    pub minimum: u128,

    /// Latest permitted temporal distance, inclusive.
    pub maximum: u128,
}

impl TemporalRange {
    /// Creates an inclusive range whose minimum does not exceed its maximum.
    pub fn new(minimum: u128, maximum: u128) -> Result<Self, PropertyError> {
        if minimum > maximum {
            return Err(PropertyError::InvalidTemporalRange {
                minimum,
                maximum,
            });
        }

        Ok(Self { minimum, maximum })
    }

    /// Returns a range containing exactly one temporal distance.
    #[must_use]
    pub const fn exact(distance: u128) -> Self {
        Self {
            minimum: distance,
            maximum: distance,
        }
    }
}

/// A temporal bound.
///
/// `Unbounded` means that the formula has no finite semantic upper bound.
/// It does not mean that an implementation must search forever or that
/// verification is guaranteed to terminate.
///
/// Operational proof limits belong to the verification session, not to
/// this language-level representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TemporalBound {
    /// The temporal interval is bounded and inclusive.
    Bounded(TemporalRange),

    /// The formula has no finite semantic upper bound.
    Unbounded,
}

impl TemporalBound {
    /// Constructs a bounded temporal interval.
    pub fn bounded(
        minimum: u128,
        maximum: u128,
    ) -> Result<Self, PropertyError> {
        Ok(Self::Bounded(TemporalRange::new(minimum, maximum)?))
    }

    /// Returns the bounded interval, or `None` for an unbounded interval.
    #[must_use]
    pub const fn as_range(self) -> Option<TemporalRange> {
        match self {
            Self::Bounded(range) => Some(range),
            Self::Unbounded => None,
        }
    }
}

/// The semantic classification of a declared property.
///
/// This classification describes the property's intent. It does not
/// determine which proof algorithm a backend must use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PropertyKind {
    /// A condition that must hold for every applicable observation.
    Assertion,

    /// A condition that must hold at least once in the applicable behavior.
    Cover,

    /// A condition describing a required design invariant.
    Invariant,

    /// A condition describing an assumption about the environment.
    Assumption,

    /// A condition describing a required relationship between behaviors.
    Equivalence,

    /// A property classification supplied by an extension.
    Extension,
}

/// The declared severity of a property violation.
///
/// Severity controls reporting and policy integration. It does not change
/// the logical meaning of the formula.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PropertySeverity {
    /// A violation invalidates the applicable verification obligation.
    Error,

    /// A violation requires attention but is not automatically a build error.
    Warning,

    /// A violation is informational under the selected reporting policy.
    Informational,
}

/// A portable temporal formula.
///
/// Formula nodes reference canonical expressions instead of embedding a
/// second expression AST. Boolean compatibility, type checking, symbol
/// resolution, and temporal-operator legality are checked by later semantic
/// and verification stages.
///
/// The exact meaning of each operator must be defined by the normative HDL
/// specification and the selected verification semantics. This data type
/// does not silently select a vendor's assertion dialect.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TemporalFormula {
    /// A Boolean expression evaluated in the applicable observation context.
    Atom(ExpressionRef),

    /// Logical negation.
    Not(Box<TemporalFormula>),

    /// Logical conjunction.
    And(Box<TemporalFormula>, Box<TemporalFormula>),

    /// Logical disjunction.
    Or(Box<TemporalFormula>, Box<TemporalFormula>),

    /// Logical implication.
    Implies(Box<TemporalFormula>, Box<TemporalFormula>),

    /// The child formula must hold at every applicable observation.
    Always(Box<TemporalFormula>),

    /// The child formula must eventually hold within the declared interval.
    ///
    /// `Unbounded` expresses unbounded eventuality. A bounded interval is
    /// interpreted relative to the applicable logical clock domain.
    Eventually {
        /// Temporal interval for eventual satisfaction.
        bound: TemporalBound,

        /// Formula whose eventual satisfaction is required.
        formula: Box<TemporalFormula>,
    },

    /// The child formula must hold at the next applicable observation.
    Next(Box<TemporalFormula>),

    /// The left formula must hold until the right formula holds, according
    /// to the selected normative temporal semantics.
    Until {
        /// Temporal interval in which the right formula must be satisfied.
        bound: TemporalBound,

        /// Formula required to hold until the right formula is satisfied.
        left: Box<TemporalFormula>,

        /// Formula that terminates the until obligation when satisfied.
        right: Box<TemporalFormula>,
    },
}

impl TemporalFormula {
    /// Creates an atomic formula referencing a canonical expression.
    #[must_use]
    pub fn atom(expression: ExpressionRef) -> Self {
        Self::Atom(expression)
    }

    /// Creates a negated formula.
    #[must_use]
    pub fn not(formula: Self) -> Self {
        Self::Not(Box::new(formula))
    }

    /// Creates a conjunction.
    #[must_use]
    pub fn and(left: Self, right: Self) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    /// Creates a disjunction.
    #[must_use]
    pub fn or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    /// Creates an implication.
    #[must_use]
    pub fn implies(antecedent: Self, consequent: Self) -> Self {
        Self::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Creates an always-holds temporal formula.
    #[must_use]
    pub fn always(formula: Self) -> Self {
        Self::Always(Box::new(formula))
    }

    /// Creates an eventually-satisfied temporal formula.
    #[must_use]
    pub fn eventually(bound: TemporalBound, formula: Self) -> Self {
        Self::Eventually {
            bound,
            formula: Box::new(formula),
        }
    }

    /// Creates a next-observation temporal formula.
    #[must_use]
    pub fn next(formula: Self) -> Self {
        Self::Next(Box::new(formula))
    }

    /// Creates an until temporal formula.
    #[must_use]
    pub fn until(
        bound: TemporalBound,
        left: Self,
        right: Self,
    ) -> Self {
        Self::Until {
            bound,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Returns the number of formula nodes using an iterative traversal.
    ///
    /// This operation avoids recursive traversal of deeply nested formulas.
    /// Its running time and auxiliary memory are proportional to the number
    /// of formula nodes visited.
    #[must_use]
    pub fn node_count(&self) -> usize {
        let mut pending = vec![self];
        let mut count = 0_usize;

        while let Some(formula) = pending.pop() {
            count = count.saturating_add(1);

            match formula {
                Self::Atom(_) => {}
                Self::Not(child)
                | Self::Always(child)
                | Self::Next(child) => pending.push(child),
                Self::And(left, right)
                | Self::Or(left, right)
                | Self::Implies(left, right) => {
                    pending.push(right);
                    pending.push(left);
                }
                Self::Eventually { formula, .. } => pending.push(formula),
                Self::Until { left, right, .. } => {
                    pending.push(right);
                    pending.push(left);
                }
            }
        }

        count
    }

    /// Validates the formula's structural representation.
    ///
    /// Because expression references are validated when constructed, this
    /// traversal checks structural invariants and reports formula size
    /// overflow defensively. It does not resolve expressions or prove the
    /// formula.
    pub fn validate_structure(&self) -> Result<(), PropertyError> {
        let mut pending = vec![self];

        while let Some(formula) = pending.pop() {
            match formula {
                Self::Atom(reference) => {
                    if reference.as_str().trim().is_empty() {
                        return Err(PropertyError::EmptyIdentifier {
                            field: "expression reference",
                        });
                    }
                }
                Self::Not(child)
                | Self::Always(child)
                | Self::Next(child) => pending.push(child),
                Self::And(left, right)
                | Self::Or(left, right)
                | Self::Implies(left, right) => {
                    pending.push(right);
                    pending.push(left);
                }
                Self::Eventually { bound, formula } => {
                    validate_bound(*bound)?;
                    pending.push(formula);
                }
                Self::Until { bound, left, right } => {
                    validate_bound(*bound)?;
                    pending.push(right);
                    pending.push(left);
                }
            }
        }

        Ok(())
    }
}

fn validate_bound(bound: TemporalBound) -> Result<(), PropertyError> {
    if let TemporalBound::Bounded(range) = bound {
        if range.minimum > range.maximum {
            return Err(PropertyError::InvalidTemporalRange {
                minimum: range.minimum,
                maximum: range.maximum,
            });
        }
    }

    Ok(())
}

/// A portable HDL property declaration.
///
/// `name` is a human-readable label, while `id` is the stable identity.
/// `clock` selects the logical observation domain for temporal operators.
/// Properties without an explicit clock may be used only where the language
/// specification or verification adapter defines an unambiguous default.
///
/// `disable_condition` references an expression that disables or masks the
/// property under the semantics defined by the HDL specification. This
/// module does not assume a particular vendor-specific `disable iff` rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Property {
    /// Stable property identity.
    pub id: PropertyId,

    /// Human-readable property name.
    pub name: String,

    /// Classification of the property.
    pub kind: PropertyKind,

    /// Declared reporting severity.
    pub severity: PropertySeverity,

    /// Optional logical clock domain for temporal evaluation.
    pub clock: Option<ClockRef>,

    /// Optional expression that disables or masks evaluation.
    pub disable_condition: Option<DisableConditionRef>,

    /// The property formula.
    pub formula: TemporalFormula,
}

impl Property {
    /// Creates a property declaration.
    ///
    /// This constructor validates the basic name and formula structure.
    /// It does not claim that the property is semantically well-typed,
    /// resolvable, provable, or satisfied by any particular design.
    pub fn new(
        id: PropertyId,
        name: impl Into<String>,
        kind: PropertyKind,
        severity: PropertySeverity,
        clock: Option<ClockRef>,
        disable_condition: Option<DisableConditionRef>,
        formula: TemporalFormula,
    ) -> Result<Self, PropertyError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(PropertyError::EmptyIdentifier {
                field: "property name",
            });
        }

        formula.validate_structure()?;

        Ok(Self {
            id,
            name,
            kind,
            severity,
            clock,
            disable_condition,
            formula,
        })
    }

    /// Returns the stable identity of this property.
    #[must_use]
    pub fn id(&self) -> &PropertyId {
        &self.id
    }

    /// Returns the human-readable property name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A collection of properties belonging to one semantic verification scope.
///
/// Insertion rejects duplicate property IDs and duplicate names within this
/// collection. The collection does not claim that names are globally unique
/// across packages or modules; the owning semantic scope decides that policy.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PropertySet {
    properties: Vec<Property>,
    ids: BTreeSet<PropertyId>,
    names: BTreeSet<String>,
}

impl PropertySet {
    /// Creates an empty property collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a property, rejecting duplicate IDs or names.
    ///
    /// The property is not inserted if a duplicate is detected.
    pub fn insert(&mut self, property: Property) -> Result<(), PropertyError> {
        if self.ids.contains(&property.id) {
            return Err(PropertyError::DuplicateId(property.id));
        }

        if self.names.contains(&property.name) {
            return Err(PropertyError::DuplicateName(property.name));
        }

        self.ids.insert(property.id.clone());
        self.names.insert(property.name.clone());
        self.properties.push(property);

        Ok(())
    }

    /// Returns the number of properties in this collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether the collection contains no properties.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns an iterator over properties in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &Property> {
        self.properties.iter()
    }

    /// Finds a property by its stable identity.
    #[must_use]
    pub fn get(&self, id: &PropertyId) -> Option<&Property> {
        self.properties.iter().find(|property| &property.id == id)
    }

    /// Validates every property's structural representation.
    ///
    /// This checks structure only. Semantic expression resolution and
    /// verification remain separate stages.
    pub fn validate(&self) -> Result<(), PropertyError> {
        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();

        for property in &self.properties {
            if !ids.insert(property.id.clone()) {
                return Err(PropertyError::DuplicateId(property.id.clone()));
            }

            if !names.insert(property.name.clone()) {
                return Err(PropertyError::DuplicateName(property.name.clone()));
            }

            if property.name.trim().is_empty() {
                return Err(PropertyError::EmptyIdentifier {
                    field: "property name",
                });
            }

            if let Some(clock) = &property.clock {
                if clock.as_str().trim().is_empty() {
                    return Err(PropertyError::EmptyIdentifier {
                        field: "clock reference",
                    });
                }
            }

            if let Some(condition) = &property.disable_condition {
                if condition.as_str().trim().is_empty() {
                    return Err(PropertyError::EmptyIdentifier {
                        field: "disable condition",
                    });
                }
            }

            property.formula.validate_structure()?;
        }

        Ok(())
    }
}

/// Structural property errors.
///
/// These errors describe malformed property declarations, not failed
/// verification obligations or invalid HDL programs in general.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyError {
    /// An identifier or required textual field was empty.
    EmptyIdentifier {
        /// Field that was empty.
        field: &'static str,
    },

    /// A property ID already exists in the current property scope.
    DuplicateId(PropertyId),

    /// A property name already exists in the current property scope.
    DuplicateName(String),

    /// A temporal range has its minimum greater than its maximum.
    InvalidTemporalRange {
        /// Declared inclusive minimum.
        minimum: u128,

        /// Declared inclusive maximum.
        maximum: u128,
    },
}

impl fmt::Display for PropertyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }
            Self::DuplicateId(id) => {
                write!(formatter, "duplicate HDL property ID `{id}`")
            }
            Self::DuplicateName(name) => {
                write!(formatter, "duplicate HDL property name `{name}`")
            }
            Self::InvalidTemporalRange { minimum, maximum } => {
                write!(
                    formatter,
                    "invalid temporal range: minimum {minimum} exceeds maximum {maximum}"
                )
            }
        }
    }
}

impl Error for PropertyError {}

/// A verification outcome, deliberately separate from structural validation.
///
/// A structurally valid property may remain unproven because the engine lacks
/// a supported proof method, a resource budget is exhausted, or the result
/// cannot be determined from available evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyOutcome {
    /// The verification engine established that the property holds under
    /// the stated assumptions and verification model.
    Proven,

    /// The verification engine established a violation.
    ///
    /// A production verification adapter should attach a counterexample or
    /// other evidence in its richer engine-specific result type.
    Violated,

    /// The engine could not determine whether the property holds.
    Inconclusive,

    /// Verification did not finish, for example because a configured
    /// operational budget or cancellation request stopped the analysis.
    Incomplete,

    /// No compatible verification procedure is available.
    Unsupported,
}

impl PropertyOutcome {
    /// Returns whether the property has been proven to hold.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        matches!(self, Self::Proven)
    }

    /// Returns whether the outcome must not be treated as a successful proof.
    #[must_use]
    pub const fn is_non_success(&self) -> bool {
        !matches!(self, Self::Proven)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expression(value: &str) -> ExpressionRef {
        ExpressionRef::new(value).expect("test expression identity is valid")
    }

    fn property(id: &str, name: &str) -> Property {
        Property::new(
            PropertyId::new(id).expect("test property ID is valid"),
            name,
            PropertyKind::Assertion,
            PropertySeverity::Error,
            Some(ClockRef::new("clock.main").expect("clock ID is valid")),
            None,
            TemporalFormula::always(TemporalFormula::atom(expression("expr.ready"))),
        )
        .expect("test property is structurally valid")
    }

    #[test]
    fn rejects_empty_property_identity() {
        assert_eq!(
            PropertyId::new("  "),
            Err(PropertyError::EmptyIdentifier {
                field: "property identifier",
            })
        );
    }

    #[test]
    fn rejects_empty_expression_reference() {
        assert_eq!(
            ExpressionRef::new(""),
            Err(PropertyError::EmptyIdentifier {
                field: "expression reference",
            })
        );
    }

    #[test]
    fn rejects_inverted_temporal_range() {
        assert_eq!(
            TemporalRange::new(8, 3),
            Err(PropertyError::InvalidTemporalRange {
                minimum: 8,
                maximum: 3,
            })
        );
    }

    #[test]
    fn accepts_equal_temporal_bounds() {
        assert_eq!(
            TemporalRange::new(7, 7),
            Ok(TemporalRange::exact(7))
        );
    }

    #[test]
    fn represents_unbounded_eventuality_explicitly() {
        let formula = TemporalFormula::eventually(
            TemporalBound::Unbounded,
            TemporalFormula::atom(expression("expr.done")),
        );

        assert_eq!(formula.node_count(), 2);
        assert!(formula.validate_structure().is_ok());
    }

    #[test]
    fn counts_nested_formula_nodes() {
        let formula = TemporalFormula::implies(
            TemporalFormula::atom(expression("expr.request")),
            TemporalFormula::eventually(
                TemporalBound::bounded(0, 12).expect("valid interval"),
                TemporalFormula::atom(expression("expr.response")),
            ),
        );

        assert_eq!(formula.node_count(), 4);
        assert!(formula.validate_structure().is_ok());
    }

    #[test]
    fn property_set_rejects_duplicate_ids() {
        let mut properties = PropertySet::new();

        properties
            .insert(property("p.ready", "ready"))
            .expect("first insertion succeeds");

        assert_eq!(
            properties.insert(property("p.ready", "ready_again")),
            Err(PropertyError::DuplicateId(
                PropertyId::new("p.ready").expect("valid ID")
            ))
        );

        assert_eq!(properties.len(), 1);
    }

    #[test]
    fn property_set_rejects_duplicate_names() {
        let mut properties = PropertySet::new();

        properties
            .insert(property("p.first", "ready"))
            .expect("first insertion succeeds");

        assert_eq!(
            properties.insert(property("p.second", "ready")),
            Err(PropertyError::DuplicateName("ready".to_owned()))
        );

        assert_eq!(properties.len(), 1);
    }

    #[test]
    fn property_set_lookup_uses_stable_identity() {
        let mut properties = PropertySet::new();
        properties
            .insert(property("p.ready", "ready"))
            .expect("insertion succeeds");

        let id = PropertyId::new("p.ready").expect("valid ID");

        assert_eq!(properties.get(&id).map(Property::name), Some("ready"));
        assert!(properties.validate().is_ok());
    }

    #[test]
    fn inconclusive_and_incomplete_are_not_proofs() {
        assert!(PropertyOutcome::Proven.is_proven());
        assert!(!PropertyOutcome::Inconclusive.is_proven());
        assert!(!PropertyOutcome::Incomplete.is_proven());
        assert!(PropertyOutcome::Unsupported.is_non_success());
        assert!(PropertyOutcome::Violated.is_non_success());
    }

    #[test]
    fn temporal_bounds_support_values_beyond_usize_on_narrow_targets() {
        let value = u128::MAX;

        assert_eq!(
            TemporalRange::new(value, value),
            Ok(TemporalRange::exact(value))
        );
    }

    #[test]
    fn formula_validation_handles_deep_iterative_traversal() {
        let mut formula = TemporalFormula::atom(expression("expr.signal"));

        for _ in 0..10_000 {
            formula = TemporalFormula::not(formula);
        }

        assert_eq!(formula.node_count(), 10_001);
        assert!(formula.validate_structure().is_ok());
    }

    #[test]
    fn empty_property_names_are_rejected() {
        let result = Property::new(
            PropertyId::new("p.invalid").expect("valid ID"),
            " ",
            PropertyKind::Assertion,
            PropertySeverity::Error,
            None,
            None,
            TemporalFormula::atom(expression("expr.signal")),
        );

        assert_eq!(
            result,
            Err(PropertyError::EmptyIdentifier {
                field: "property name",
            })
        );
    }
}
