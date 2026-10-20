
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR aggregate types
//!
//! Path: `src/hdl/hdlir/types/aggregate.rs`
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Represent target-independent aggregate types for HDLIR: arrays, tuples,
//! and records. Aggregate elements may be logic types, signal types, named
//! semantic types, or further nested aggregates.
//!
//! ## Authority and ownership
//!
//! Source syntax and source-level meaning belong to the grammar, canonical
//! AST, and semantic model. This module owns only the HDLIR aggregate-type
//! descriptor and its local invariants.
//!
//! This module reuses:
//!
//! - `BitWidth`'s sibling-owned `WidthSymbol` for symbolic extents.
//! - `LogicType` for digital logic type semantics.
//! - `SignalType` for signal type categories and named semantic identities.
//!
//! It does not own parsing, name resolution, concrete runtime values,
//! aggregate initialization, physical layouts, memory allocation, pin
//! assignment, synthesis, placement, routing, or target feasibility.
//!
//! ## Scalability
//!
//! No fixed array-length, tuple-arity, record-field-count, or nesting-depth
//! ceiling is imposed by this API. Collections grow dynamically.
//!
//! Aggregate lengths may remain symbolic. Explicit evaluation uses checked
//! arithmetic and a caller-supplied node budget. Budget exhaustion is an
//! incomplete evaluation, not evidence that a source program is invalid.
//!
//! This is not a claim that infinite inputs can be processed. Actual work is
//! bounded by available resources, representable values, and explicit
//! operational budgets.
//!
//! ## Compatibility
//!
//! Aggregate compatibility is conservative. It returns `Indeterminate`
//! whenever the available symbolic information cannot prove compatibility
//! or incompatibility. Named semantic types are compared by canonical
//! identity; this module does not resolve their definitions.
//!
//! Record field declaration order is retained and participates in structural
//! compatibility. Changing this policy requires a specification-level
//! decision, not a local implementation change.
//!
//! ## Integration contract
//!
//! - `types/mod.rs` registers `aggregate` and re-exports its intended API.
//! - `verify/types.rs` consumes compatibility results and reports
//!   indeterminate cases rather than treating them as verified.
//! - `values/parameters.rs` or the lowering layer supplies parameter bindings
//!   to `AggregateLength::evaluate`.
//! - `lowering/from_semantic.rs` converts validated source aggregate types
//!   into these descriptors without redoing name resolution.
//! - Serialization must preserve every variant, field order, symbolic
//!   expression, and named identity.
//!
//! This module intentionally does not depend on future builders, graph
//! structures, verification passes, or serialization implementations.

use core::fmt;

use crate::frontend::semantic::domains::hdl::HdlId;

use super::bit_width::WidthSymbol;
use super::logic::{LogicType, LogicTypeCompatibility};
use super::signal::{SignalType, SignalTypeCompatibility};

/// An aggregate's element count or extent.
///
/// Unlike [`super::bit_width::BitWidth`], an aggregate extent is not
/// inherently a bit width. `Known(0)` is representable so the type system
/// does not silently impose a positive-size rule. Whether zero-length
/// aggregates are legal in a particular source context is decided by the
/// canonical semantic rules and HDLIR verification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AggregateLength {
    /// A concrete element count, including zero where the language permits it.
    Known(u128),

    /// A symbolic extent whose value may be supplied later.
    Symbol(WidthSymbol),

    /// The sum of two extent expressions.
    Add(Box<AggregateLength>, Box<AggregateLength>),

    /// The product of two extent expressions.
    Multiply(Box<AggregateLength>, Box<AggregateLength>),

    /// An extent whose defining expression or binding was not available when
    /// the descriptor was constructed.
    Unresolved(WidthSymbol),
}

impl AggregateLength {
    /// Construct a concrete aggregate extent.
    #[must_use]
    pub const fn known(value: u128) -> Self {
        Self::Known(value)
    }

    /// Construct a symbolic aggregate extent.
    ///
    /// Empty or whitespace-only names are rejected by `WidthSymbol`.
    pub fn symbol(name: impl Into<String>) -> Result<Self, AggregateLengthError> {
        WidthSymbol::new(name)
            .map(Self::Symbol)
            .map_err(AggregateLengthError::InvalidSymbol)
    }

    /// Construct an explicitly unresolved aggregate extent.
    pub fn unresolved(
        name: impl Into<String>,
    ) -> Result<Self, AggregateLengthError> {
        WidthSymbol::new(name)
            .map(Self::Unresolved)
            .map_err(AggregateLengthError::InvalidSymbol)
    }

    /// Construct a symbolic addition without evaluating it.
    #[must_use]
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    /// Construct a symbolic multiplication without evaluating it.
    #[must_use]
    pub fn multiply(lhs: Self, rhs: Self) -> Self {
        Self::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// Return the concrete value if this expression is a single known value.
    #[must_use]
    pub const fn as_known(&self) -> Option<u128> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Symbol(_)
            | Self::Add(_, _)
            | Self::Multiply(_, _)
            | Self::Unresolved(_) => None,
        }
    }

    /// Evaluate an extent with explicit symbol bindings and a work budget.
    ///
    /// `resolve` receives a symbol's original spelling and returns its
    /// concrete value when a binding is available. A missing binding is an
    /// explicit error. The resolver should be deterministic and should not
    /// rely on side effects: symbol resolution order follows expression
    /// traversal order.
    ///
    /// Evaluation is iterative to avoid recursive evaluation proportional to
    /// expression depth. The node budget counts every visited expression
    /// node, including known values and symbol references.
    pub fn evaluate<F>(
        &self,
        budget: AggregateLengthBudget,
        mut resolve: F,
    ) -> Result<u128, AggregateLengthError>
    where
        F: FnMut(&str) -> Option<u128>,
    {
        enum Visit<'a> {
            Enter(&'a AggregateLength),
            ExitAdd,
            ExitMultiply,
        }

        let mut pending = vec![Visit::Enter(self)];
        let mut values: Vec<u128> = Vec::new();
        let mut visited = 0usize;

        while let Some(item) = pending.pop() {
            match item {
                Visit::Enter(expression) => {
                    visited = visited.checked_add(1).ok_or(
                        AggregateLengthError::EvaluationCounterOverflow,
                    )?;

                    if visited > budget.max_nodes {
                        return Err(AggregateLengthError::BudgetExceeded {
                            limit: budget.max_nodes,
                            visited,
                        });
                    }

                    match expression {
                        Self::Known(value) => values.push(*value),

                        Self::Symbol(symbol) => {
                            let value = resolve(symbol.as_str()).ok_or_else(|| {
                                AggregateLengthError::UnboundSymbol(
                                    symbol.as_str().to_owned(),
                                )
                            })?;
                            values.push(value);
                        }

                        Self::Unresolved(symbol) => {
                            let value = resolve(symbol.as_str()).ok_or_else(|| {
                                AggregateLengthError::UnresolvedSymbol(
                                    symbol.as_str().to_owned(),
                                )
                            })?;
                            values.push(value);
                        }

                        Self::Add(lhs, rhs) => {
                            pending.push(Visit::ExitAdd);
                            pending.push(Visit::Enter(rhs));
                            pending.push(Visit::Enter(lhs));
                        }

                        Self::Multiply(lhs, rhs) => {
                            pending.push(Visit::ExitMultiply);
                            pending.push(Visit::Enter(rhs));
                            pending.push(Visit::Enter(lhs));
                        }
                    }
                }

                Visit::ExitAdd => {
                    let rhs = values.pop().ok_or(
                        AggregateLengthError::InvalidEvaluationState,
                    )?;
                    let lhs = values.pop().ok_or(
                        AggregateLengthError::InvalidEvaluationState,
                    )?;

                    values.push(lhs.checked_add(rhs).ok_or(
                        AggregateLengthError::ArithmeticOverflow,
                    )?);
                }

                Visit::ExitMultiply => {
                    let rhs = values.pop().ok_or(
                        AggregateLengthError::InvalidEvaluationState,
                    )?;
                    let lhs = values.pop().ok_or(
                        AggregateLengthError::InvalidEvaluationState,
                    )?;

                    values.push(lhs.checked_mul(rhs).ok_or(
                        AggregateLengthError::ArithmeticOverflow,
                    )?);
                }
            }
        }

        if values.len() != 1 {
            return Err(AggregateLengthError::InvalidEvaluationState);
        }

        values
            .pop()
            .ok_or(AggregateLengthError::InvalidEvaluationState)
    }
}

/// Explicit operational budget for aggregate-extent evaluation.
///
/// This limits compiler work, not the legal size of an HDL design.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AggregateLengthBudget {
    max_nodes: usize,
}

impl AggregateLengthBudget {
    /// Create a budget allowing at most `max_nodes` expression nodes.
    #[must_use]
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }

    /// Create a budget limited only by the maximum representable `usize`.
    ///
    /// This does not guarantee that sufficient memory or processing time is
    /// available to complete an evaluation.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_nodes: usize::MAX,
        }
    }

    /// Return the configured node budget.
    #[must_use]
    pub const fn max_nodes(self) -> usize {
        self.max_nodes
    }
}

/// Errors produced while constructing or evaluating an aggregate extent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AggregateLengthError {
    /// The symbolic extent name was empty or whitespace-only.
    InvalidSymbol(super::bit_width::WidthEvaluationError),

    /// No binding was available for a regular symbolic extent.
    UnboundSymbol(String),

    /// No binding was available for an explicitly unresolved extent.
    UnresolvedSymbol(String),

    /// Addition or multiplication overflowed `u128`.
    ArithmeticOverflow,

    /// Evaluation exceeded its explicit node budget.
    BudgetExceeded {
        /// Maximum number of nodes permitted.
        limit: usize,
        /// Number of nodes reached when the limit was exceeded.
        visited: usize,
    },

    /// An internal evaluation invariant was violated.
    InvalidEvaluationState,

    /// The visited-node counter overflowed.
    EvaluationCounterOverflow,
}

impl fmt::Display for AggregateLengthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSymbol(error) => {
                write!(formatter, "invalid aggregate extent symbol: {error}")
            }
            Self::UnboundSymbol(name) => {
                write!(formatter, "unbound aggregate extent symbol `{name}`")
            }
            Self::UnresolvedSymbol(name) => {
                write!(formatter, "unresolved aggregate extent `{name}`")
            }
            Self::ArithmeticOverflow => {
                formatter.write_str("aggregate extent arithmetic overflowed u128")
            }
            Self::BudgetExceeded { limit, visited } => write!(
                formatter,
                "aggregate extent evaluation budget exceeded: \
                 limit {limit} nodes, visited {visited}"
            ),
            Self::InvalidEvaluationState => {
                formatter.write_str("invalid aggregate extent evaluation state")
            }
            Self::EvaluationCounterOverflow => {
                formatter.write_str("aggregate extent evaluation counter overflowed")
            }
        }
    }
}

impl std::error::Error for AggregateLengthError {}

/// A value type that may appear as an aggregate element or field.
///
/// `Named` retains the canonical semantic identity without resolving the
/// referenced type. `Aggregate` permits nested arrays, tuples, and records.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AggregateValueType {
    /// A digital logic scalar or vector.
    Logic(LogicType),

    /// A logical signal type, including clock, reset, or named signal types.
    Signal(SignalType),

    /// A nested aggregate type.
    Aggregate(Box<AggregateType>),

    /// A reference to a canonical named semantic type.
    Named(HdlId),
}

impl AggregateValueType {
    /// Construct an aggregate value type from a logic type.
    #[must_use]
    pub const fn logic(value: LogicType) -> Self {
        Self::Logic(value)
    }

    /// Construct an aggregate value type from a signal type.
    #[must_use]
    pub const fn signal(value: SignalType) -> Self {
        Self::Signal(value)
    }

    /// Construct an aggregate value type from a nested aggregate.
    #[must_use]
    pub fn aggregate(value: AggregateType) -> Self {
        Self::Aggregate(Box::new(value))
    }

    /// Construct a reference to a canonical named semantic type.
    #[must_use]
    pub const fn named(value: HdlId) -> Self {
        Self::Named(value)
    }
}

/// One named field in an aggregate record.
///
/// The field name is retained exactly as supplied. This constructor checks
/// only that it is non-empty; normalization, scope resolution, and language
/// identifier legality belong to the canonical frontend semantic model.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AggregateField {
    name: String,
    value_type: AggregateValueType,
}

impl AggregateField {
    /// Construct a field with a non-empty name and its value type.
    pub fn new(
        name: impl Into<String>,
        value_type: AggregateValueType,
    ) -> Result<Self, AggregateTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AggregateTypeError::EmptyFieldName);
        }

        Ok(Self { name, value_type })
    }

    /// Return the retained field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the field's type.
    #[must_use]
    pub const fn value_type(&self) -> &AggregateValueType {
        &self.value_type
    }
}

/// A target-independent aggregate type descriptor.
///
/// Array lengths may be concrete or symbolic. Tuple element order and record
/// field declaration order are preserved. No physical packing, padding,
/// alignment, byte offsets, or target-specific layout is implied.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AggregateType {
    /// A homogeneous array of `length` elements.
    Array {
        /// Type of every element.
        element: Box<AggregateValueType>,
        /// Element count, which may remain symbolic.
        length: AggregateLength,
    },

    /// A heterogeneous, positionally typed tuple.
    Tuple {
        /// Tuple element types in declaration order.
        elements: Vec<AggregateValueType>,
    },

    /// A heterogeneous record with named fields.
    Record {
        /// Fields in source/semantic declaration order.
        fields: Vec<AggregateField>,
    },
}

/// Errors from aggregate-type construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AggregateTypeError {
    /// A record field name was empty or whitespace-only.
    EmptyFieldName,

    /// A record contains the same retained field name more than once.
    DuplicateFieldName(String),
}

impl fmt::Display for AggregateTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFieldName => {
                formatter.write_str("aggregate record field name must not be empty")
            }
            Self::DuplicateFieldName(name) => {
                write!(formatter, "duplicate aggregate record field `{name}`")
            }
        }
    }
}

impl std::error::Error for AggregateTypeError {}

/// Result of comparing aggregate types using the information currently known.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AggregateTypeCompatibility {
    /// The available information proves structural compatibility.
    Compatible,

    /// The available information proves incompatibility.
    Incompatible,

    /// Symbolic extents or nested types prevent a definitive conclusion.
    Indeterminate,
}

impl AggregateType {
    /// Construct an array aggregate.
    #[must_use]
    pub fn array(
        element: AggregateValueType,
        length: AggregateLength,
    ) -> Self {
        Self::Array {
            element: Box::new(element),
            length,
        }
    }

    /// Construct a tuple aggregate.
    #[must_use]
    pub fn tuple(elements: Vec<AggregateValueType>) -> Self {
        Self::Tuple { elements }
    }

    /// Construct a record aggregate and reject duplicate field names.
    ///
    /// Field order is preserved. Name resolution and language-specific
    /// identifier normalization must already have been handled upstream.
    pub fn record(
        fields: Vec<AggregateField>,
    ) -> Result<Self, AggregateTypeError> {
        let mut names: std::collections::HashSet<&str> =
            std::collections::HashSet::with_capacity(fields.len());

        for field in &fields {
            if !names.insert(field.name()) {
                return Err(AggregateTypeError::DuplicateFieldName(
                    field.name().to_owned(),
                ));
            }
        }

        Ok(Self::Record { fields })
    }

    /// Return the array element type, or `None` for non-array aggregates.
    #[must_use]
    pub fn array_element(&self) -> Option<&AggregateValueType> {
        match self {
            Self::Array { element, .. } => Some(element),
            Self::Tuple { .. } | Self::Record { .. } => None,
        }
    }

    /// Return the array extent, or `None` for non-array aggregates.
    #[must_use]
    pub fn array_length(&self) -> Option<&AggregateLength> {
        match self {
            Self::Array { length, .. } => Some(length),
            Self::Tuple { .. } | Self::Record { .. } => None,
        }
    }

    /// Return tuple elements in declaration order, or `None` otherwise.
    #[must_use]
    pub fn tuple_elements(&self) -> Option<&[AggregateValueType]> {
        match self {
            Self::Tuple { elements } => Some(elements),
            Self::Array { .. } | Self::Record { .. } => None,
        }
    }

    /// Return record fields in declaration order, or `None` otherwise.
    #[must_use]
    pub fn record_fields(&self) -> Option<&[AggregateField]> {
        match self {
            Self::Record { fields } => Some(fields),
            Self::Array { .. } | Self::Tuple { .. } => None,
        }
    }

    /// Compare two aggregate types conservatively.
    ///
    /// Exact symbolic-expression equality is sufficient to establish equal
    /// extents. Different symbolic expressions are indeterminate unless both
    /// are concrete and unequal. This function does not resolve parameters.
    ///
    /// The traversal is iterative so comparisons do not recurse through the
    /// aggregate graph on the Rust call stack.
    #[must_use]
    pub fn compatibility_with(
        &self,
        other: &Self,
    ) -> AggregateTypeCompatibility {
        enum TypePair<'a> {
            Aggregate(&'a AggregateType, &'a AggregateType),
            Value(&'a AggregateValueType, &'a AggregateValueType),
        }

        let mut pending = vec![TypePair::Aggregate(self, other)];
        let mut indeterminate = false;

        while let Some(pair) = pending.pop() {
            match pair {
                TypePair::Aggregate(left, right) => match (left, right) {
                    (
                        Self::Array {
                            element: left_element,
                            length: left_length,
                        },
                        Self::Array {
                            element: right_element,
                            length: right_length,
                        },
                    ) => {
                        match length_compatibility(left_length, right_length) {
                            AggregateTypeCompatibility::Compatible => {}
                            AggregateTypeCompatibility::Incompatible => {
                                return AggregateTypeCompatibility::Incompatible;
                            }
                            AggregateTypeCompatibility::Indeterminate => {
                                indeterminate = true;
                            }
                        }

                        pending.push(TypePair::Value(left_element, right_element));
                    }

                    (
                        Self::Tuple { elements: left },
                        Self::Tuple { elements: right },
                    ) => {
                        if left.len() != right.len() {
                            return AggregateTypeCompatibility::Incompatible;
                        }

                        pending.extend(
                            left.iter()
                                .zip(right.iter())
                                .map(|(l, r)| TypePair::Value(l, r)),
                        );
                    }

                    (
                        Self::Record { fields: left },
                        Self::Record { fields: right },
                    ) => {
                        if left.len() != right.len() {
                            return AggregateTypeCompatibility::Incompatible;
                        }

                        for (left_field, right_field) in left.iter().zip(right.iter()) {
                            if left_field.name != right_field.name {
                                return AggregateTypeCompatibility::Incompatible;
                            }

                            pending.push(TypePair::Value(
                                &left_field.value_type,
                                &right_field.value_type,
                            ));
                        }
                    }

                    _ => return AggregateTypeCompatibility::Incompatible,
                },

                TypePair::Value(left, right) => match (left, right) {
                    (AggregateValueType::Logic(l), AggregateValueType::Logic(r)) => {
                        match l.compatibility_with(r) {
                            LogicTypeCompatibility::Compatible => {}
                            LogicTypeCompatibility::Incompatible => {
                                return AggregateTypeCompatibility::Incompatible;
                            }
                            LogicTypeCompatibility::Indeterminate => {
                                indeterminate = true;
                            }
                        }
                    }

                    (AggregateValueType::Signal(l), AggregateValueType::Signal(r)) => {
                        match l.compatibility_with(r) {
                            SignalTypeCompatibility::Compatible => {}
                            SignalTypeCompatibility::Incompatible => {
                                return AggregateTypeCompatibility::Incompatible;
                            }
                            SignalTypeCompatibility::Indeterminate => {
                                indeterminate = true;
                            }
                        }
                    }

                    (
                        AggregateValueType::Aggregate(l),
                        AggregateValueType::Aggregate(r),
                    ) => pending.push(TypePair::Aggregate(l, r)),

                    (AggregateValueType::Named(l), AggregateValueType::Named(r)) => {
                        if l != r {
                            return AggregateTypeCompatibility::Incompatible;
                        }
                    }

                    _ => return AggregateTypeCompatibility::Incompatible,
                },
            }
        }

        if indeterminate {
            AggregateTypeCompatibility::Indeterminate
        } else {
            AggregateTypeCompatibility::Compatible
        }
    }

    /// Return `true` only when compatibility is proven.
    ///
    /// An indeterminate symbolic comparison returns `false`.
    #[must_use]
    pub fn is_proven_compatible_with(&self, other: &Self) -> bool {
        self.compatibility_with(other) == AggregateTypeCompatibility::Compatible
    }
}

fn length_compatibility(
    left: &AggregateLength,
    right: &AggregateLength,
) -> AggregateTypeCompatibility {
    if left == right {
        return AggregateTypeCompatibility::Compatible;
    }

    match (left.as_known(), right.as_known()) {
        (Some(left), Some(right)) if left != right => {
            AggregateTypeCompatibility::Incompatible
        }
        (Some(_), Some(_)) => AggregateTypeCompatibility::Compatible,
        _ => AggregateTypeCompatibility::Indeterminate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;
    use crate::hdl::hdlir::types::logic::{
        LogicSignedness, LogicStateModel,
    };
    use crate::hdl::hdlir::types::bit_width::BitWidth;

    fn logic_bit() -> AggregateValueType {
        AggregateValueType::logic(
            LogicType::vector(
                BitWidth::known(1).expect("one is a positive width"),
                LogicSignedness::Unsigned,
                LogicStateModel::TwoState,
            )
            .expect("valid one-bit logic type"),
        )
    }

    #[test]
    fn supports_zero_and_large_concrete_extents() {
        assert_eq!(AggregateLength::known(0).as_known(), Some(0));
        assert_eq!(
            AggregateLength::known(u128::MAX).as_known(),
            Some(u128::MAX)
        );
    }

    #[test]
    fn evaluates_symbolic_extent_with_checked_arithmetic() {
        let expression = AggregateLength::multiply(
            AggregateLength::add(
                AggregateLength::known(2),
                AggregateLength::symbol("N").expect("valid symbol"),
            ),
            AggregateLength::known(3),
        );

        let value = expression
            .evaluate(AggregateLengthBudget::new(5), |name| {
                (name == "N").then_some(4)
            })
            .expect("extent should evaluate");

        assert_eq!(value, 18);
    }

    #[test]
    fn reports_missing_symbols_and_overflow() {
        let missing = AggregateLength::symbol("N")
            .expect("valid symbol")
            .evaluate(AggregateLengthBudget::new(1), |_| None);

        assert!(matches!(
            missing,
            Err(AggregateLengthError::UnboundSymbol(_))
        ));

        let overflow = AggregateLength::add(
            AggregateLength::known(u128::MAX),
            AggregateLength::known(1),
        )
        .evaluate(AggregateLengthBudget::new(3), |_| None);

        assert_eq!(
            overflow,
            Err(AggregateLengthError::ArithmeticOverflow)
        );
    }

    #[test]
    fn reports_budget_exhaustion_without_partial_success() {
        let expression = AggregateLength::add(
            AggregateLength::known(1),
            AggregateLength::known(2),
        );

        assert!(matches!(
            expression.evaluate(AggregateLengthBudget::new(1), |_| None),
            Err(AggregateLengthError::BudgetExceeded {
                limit: 1,
                visited: 2
            })
        ));
    }

    #[test]
    fn constructs_arrays_and_nested_tuples() {
        let array = AggregateType::array(
            logic_bit(),
            AggregateLength::symbol("ELEMENTS").expect("valid symbol"),
        );

        let tuple = AggregateType::tuple(vec![
            logic_bit(),
            AggregateValueType::aggregate(array.clone()),
        ]);

        assert!(array.array_element().is_some());
        assert!(array.array_length().is_some());
        assert_eq!(tuple.tuple_elements().map(<[_]>::len), Some(2));
    }

    #[test]
    fn rejects_duplicate_record_fields() {
        let fields = vec![
            AggregateField::new("data", logic_bit()).expect("valid field"),
            AggregateField::new("data", logic_bit()).expect("valid field"),
        ];

        assert_eq!(
            AggregateType::record(fields),
            Err(AggregateTypeError::DuplicateFieldName(
                "data".to_owned()
            ))
        );
    }

    #[test]
    fn compares_records_and_preserves_field_order() {
        let first = AggregateType::record(vec![
            AggregateField::new("a", logic_bit()).expect("valid field"),
            AggregateField::new("b", logic_bit()).expect("valid field"),
        ])
        .expect("unique fields");

        let same = AggregateType::record(vec![
            AggregateField::new("a", logic_bit()).expect("valid field"),
            AggregateField::new("b", logic_bit()).expect("valid field"),
        ])
        .expect("unique fields");

        let reordered = AggregateType::record(vec![
            AggregateField::new("b", logic_bit()).expect("valid field"),
            AggregateField::new("a", logic_bit()).expect("valid field"),
        ])
        .expect("unique fields");

        assert_eq!(
            first.compatibility_with(&same),
            AggregateTypeCompatibility::Compatible
        );
        assert_eq!(
            first.compatibility_with(&reordered),
            AggregateTypeCompatibility::Incompatible
        );
    }

    #[test]
    fn different_symbolic_lengths_are_indeterminate() {
        let left = AggregateType::array(
            logic_bit(),
            AggregateLength::symbol("LEFT_LEN").expect("valid symbol"),
        );
        let right = AggregateType::array(
            logic_bit(),
            AggregateLength::symbol("RIGHT_LEN").expect("valid symbol"),
        );

        assert_eq!(
            left.compatibility_with(&right),
            AggregateTypeCompatibility::Indeterminate
        );
        assert!(!left.is_proven_compatible_with(&right));
    }

    #[test]
    fn concrete_length_mismatch_is_incompatible() {
        let left = AggregateType::array(
            logic_bit(),
            AggregateLength::known(8),
        );
        let right = AggregateType::array(
            logic_bit(),
            AggregateLength::known(16),
        );

        assert_eq!(
            left.compatibility_with(&right),
            AggregateTypeCompatibility::Incompatible
        );
    }

    #[test]
    fn named_types_use_canonical_identity() {
        let left = AggregateType::tuple(vec![
            AggregateValueType::named(
                HdlId::new("type_a").expect("non-empty identity"),
            ),
        ]);
        let right = AggregateType::tuple(vec![
            AggregateValueType::named(
                HdlId::new("type_b").expect("non-empty identity"),
            ),
        ]);

        assert_eq!(
            left.compatibility_with(&right),
            AggregateTypeCompatibility::Incompatible
        );
    }
}
