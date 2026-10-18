
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Width Resolution
//!
//! Path: `src/hdl/widths.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide checked resolution and comparison operations for the canonical
//! source-level HDL width expression defined by the existing HDL semantic
//! validator.
//!
//! This module does not define another width-expression enum. It consumes
//! `HdlWidth` and `HdlId` from the existing semantic HDL module.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`, particularly width semantics and arbitrary
//!   precision and large values.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/signals.g4`.
//! - `grammar/hdl/hardware-generics.g4`.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - The canonical frontend AST and semantic expression contracts.
//!
//! The grammar defines accepted syntax. The semantic layer determines whether
//! a width is valid. Target-specific code determines whether a valid width
//! can be realized on a particular target.
//!
//! ## Owns
//!
//! - An explicit environment for binding width symbols to concrete values.
//! - Checked evaluation of existing `HdlWidth` expressions.
//! - Conservative comparison of widths.
//! - Typed errors for unbound symbols, invalid zero widths, arithmetic
//!   overflow, and conflicting bindings.
//!
//! ## Does not own
//!
//! - A second AST, expression parser, width-expression enum, or symbol table.
//! - Generic parameter declaration or name resolution.
//! - Target capacity, physical layout, resource allocation, synthesis,
//!   placement, routing, or timing closure.
//! - HDLIR structures or transformations; those belong under
//!   `src/hdl/hdlir/`.
//!
//! ## Scalability and numeric semantics
//!
//! There is no configured or hard-coded maximum hardware width in this
//! module. Symbolic expressions remain symbolic until callers provide the
//! bindings needed for evaluation.
//!
//! The current canonical `HdlWidth::Known` representation uses `u128`.
//! Consequently, a concrete result that exceeds `u128` cannot be represented
//! by this existing contract. This is a representation boundary, not a
//! claim that the HDL language or a target has a `u128`-bit hardware limit.
//! Such values must remain symbolic or require a separately specified
//! arbitrary-precision representation before they can be resolved concretely.
//!
//! Evaluation uses an explicit work stack, avoiding recursive evaluation of
//! deeply nested width expressions. Actual work and memory use remain
//! dependent on input size and available resources.
//!
//! ## Integration contract
//!
//! The AST and semantic adapters continue to construct the existing
//! `HdlWidth`. Generic/name resolution supplies bindings to `WidthEnvironment`.
//! Type and design validation may use `resolve_width` when a concrete value is
//! required and `compare_widths` when compatibility must be checked.
//! Target feasibility remains a downstream responsibility.
//!
//! Before compiling this module, the existing HDL semantic module must be
//! reachable through `crate::frontend::semantic::domains::hdl`, and this
//! module must be registered by `src/hdl/mod.rs`. See the integration notes
//! accompanying this file.
//!
//! ## Freeze criteria
//!
//! - The existing `HdlWidth` remains the single canonical width expression.
//! - Missing bindings never become guessed values.
//! - Zero widths and arithmetic overflow never silently succeed.
//! - Symbolic comparisons can report indeterminate results.
//! - Unit, boundary, determinism, and large-expression tests pass.
//! - The public module paths are wired into the actual crate and the workspace
//!   passes its required format, build, test, and lint checks.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::{HdlId, HdlWidth};

/// Schema version for the width-resolution API.
///
/// This version is independent of the language, AST, and HDLIR versions.
pub const HDL_WIDTHS_SCHEMA_VERSION: u16 = 1;

/// A concrete binding environment for symbolic HDL widths.
///
/// The environment is deliberately separate from the canonical expression
/// tree. It can be built after generic parameters and constants have been
/// resolved, without mutating source expressions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WidthEnvironment {
    bindings: BTreeMap<HdlId, u128>,
}

impl WidthEnvironment {
    /// Create an empty width environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind a symbolic identity to a positive concrete width.
    ///
    /// Repeating an identical binding is idempotent. Attempting to bind the
    /// same identity to a different value returns an error and preserves the
    /// original binding.
    ///
    /// Zero is rejected because this API resolves bit widths, whose existing
    /// HDL semantic contract treats zero width as invalid.
    pub fn bind(
        &mut self,
        id: HdlId,
        value: u128,
    ) -> Result<(), WidthResolutionError> {
        if value == 0 {
            return Err(WidthResolutionError::ZeroWidth);
        }

        match self.bindings.get(&id) {
            Some(existing) if *existing == value => Ok(()),
            Some(existing) => Err(WidthResolutionError::ConflictingBinding {
                id,
                existing: *existing,
                attempted: value,
            }),
            None => {
                self.bindings.insert(id, value);
                Ok(())
            }
        }
    }

    /// Return the concrete binding for an identity, if present.
    #[must_use]
    pub fn get(&self, id: &HdlId) -> Option<u128> {
        self.bindings.get(id).copied()
    }

    /// Return the number of bindings in this environment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Return whether the environment contains no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Remove every binding.
    ///
    /// This affects only the environment, not any source-level width
    /// expressions.
    pub fn clear(&mut self) {
        self.bindings.clear();
    }
}

/// The kind of symbolic reference that failed to resolve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidthReferenceKind {
    /// A declared symbol or parameter has no concrete binding in the
    /// supplied environment.
    Symbol,

    /// A width expression is explicitly marked unresolved and has no
    /// concrete binding in the supplied environment.
    Unresolved,
}

/// Errors produced while resolving concrete HDL widths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidthResolutionError {
    /// A concrete width or binding was zero.
    ZeroWidth,

    /// A symbolic width reference had no concrete binding.
    UnboundSymbol {
        /// The unresolved identity.
        id: HdlId,

        /// The kind of reference in the source-level width expression.
        kind: WidthReferenceKind,
    },

    /// Concrete arithmetic exceeded the numeric representation of the
    /// existing `HdlWidth::Known(u128)` variant.
    ArithmeticOverflow {
        /// The operation that overflowed.
        operation: WidthArithmeticOperation,
    },

    /// A binding attempted to redefine an identity with a different value.
    ConflictingBinding {
        /// The symbolic identity.
        id: HdlId,

        /// The value already present in the environment.
        existing: u128,

        /// The conflicting value.
        attempted: u128,
    },

    /// The expression evaluator encountered an inconsistent internal
    /// evaluation stack.
    ///
    /// This indicates an implementation invariant failure, not invalid
    /// user syntax.
    EvaluationInvariantViolation,
}

impl fmt::Display for WidthResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => {
                f.write_str("HDL bit width must be greater than zero")
            }
            Self::UnboundSymbol { id, kind } => match kind {
                WidthReferenceKind::Symbol => {
                    write!(f, "HDL width symbol `{id}` has no binding")
                }
                WidthReferenceKind::Unresolved => {
                    write!(f, "HDL width expression `{id}` remains unresolved")
                }
            },
            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "HDL width arithmetic overflow during {operation}"
                )
            }
            Self::ConflictingBinding {
                id,
                existing,
                attempted,
            } => write!(
                f,
                "conflicting HDL width binding for `{id}`: \
                 existing value {existing}, attempted value {attempted}"
            ),
            Self::EvaluationInvariantViolation => {
                f.write_str("HDL width evaluator invariant was violated")
            }
        }
    }
}

impl Error for WidthResolutionError {}

/// Concrete arithmetic operation used in a width expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidthArithmeticOperation {
    /// Addition of two concrete widths.
    Addition,

    /// Multiplication of two concrete widths.
    Multiplication,
}

impl fmt::Display for WidthArithmeticOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Addition => f.write_str("addition"),
            Self::Multiplication => f.write_str("multiplication"),
        }
    }
}

/// Result of comparing two HDL widths.
///
/// `Indeterminate` means the supplied environment did not establish the
/// relationship. It does not mean the widths are equal, unequal, or invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidthComparison {
    /// Both expressions resolved to the same concrete width.
    Equal,

    /// Both expressions resolved to different concrete widths.
    Unequal,

    /// A missing binding prevented the comparison from being established.
    Indeterminate,
}

/// Resolve a canonical HDL width expression to a positive concrete width.
///
/// `Known` values are validated even though the enum permits direct
/// construction of `Known(0)`. `Symbol` and `Unresolved` references are
/// resolved only through the supplied environment. `Add` and `Multiply` use
/// checked arithmetic; overflow is reported rather than wrapping or
/// truncating.
///
/// The evaluator uses an explicit stack rather than recursive calls.
pub fn resolve_width(
    width: &HdlWidth,
    environment: &WidthEnvironment,
) -> Result<u128, WidthResolutionError> {
    enum Frame<'a> {
        Visit(&'a HdlWidth),
        Add,
        Multiply,
    }

    let mut frames = vec![Frame::Visit(width)];
    let mut values: Vec<u128> = Vec::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Visit(expression) => match expression {
                HdlWidth::Known(value) => {
                    if *value == 0 {
                        return Err(WidthResolutionError::ZeroWidth);
                    }
                    values.push(*value);
                }
                HdlWidth::Symbol(id) => {
                    let value = environment.get(id).ok_or_else(|| {
                        WidthResolutionError::UnboundSymbol {
                            id: id.clone(),
                            kind: WidthReferenceKind::Symbol,
                        }
                    })?;

                    if value == 0 {
                        return Err(WidthResolutionError::ZeroWidth);
                    }

                    values.push(value);
                }
                HdlWidth::Unresolved(id) => {
                    let value = environment.get(id).ok_or_else(|| {
                        WidthResolutionError::UnboundSymbol {
                            id: id.clone(),
                            kind: WidthReferenceKind::Unresolved,
                        }
                    })?;

                    if value == 0 {
                        return Err(WidthResolutionError::ZeroWidth);
                    }

                    values.push(value);
                }
                HdlWidth::Add(left, right) => {
                    frames.push(Frame::Add);
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }
                HdlWidth::Multiply(left, right) => {
                    frames.push(Frame::Multiply);
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }
            },
            Frame::Add => {
                let right = values
                    .pop()
                    .ok_or(WidthResolutionError::EvaluationInvariantViolation)?;
                let left = values
                    .pop()
                    .ok_or(WidthResolutionError::EvaluationInvariantViolation)?;

                let value = left.checked_add(right).ok_or(
                    WidthResolutionError::ArithmeticOverflow {
                        operation: WidthArithmeticOperation::Addition,
                    },
                )?;

                if value == 0 {
                    return Err(WidthResolutionError::ZeroWidth);
                }

                values.push(value);
            }
            Frame::Multiply => {
                let right = values
                    .pop()
                    .ok_or(WidthResolutionError::EvaluationInvariantViolation)?;
                let left = values
                    .pop()
                    .ok_or(WidthResolutionError::EvaluationInvariantViolation)?;

                let value = left.checked_mul(right).ok_or(
                    WidthResolutionError::ArithmeticOverflow {
                        operation: WidthArithmeticOperation::Multiplication,
                    },
                )?;

                if value == 0 {
                    return Err(WidthResolutionError::ZeroWidth);
                }

                values.push(value);
            }
        }
    }

    if values.len() != 1 {
        return Err(WidthResolutionError::EvaluationInvariantViolation);
    }

    values
        .pop()
        .ok_or(WidthResolutionError::EvaluationInvariantViolation)
}

/// Compare two width expressions using the supplied bindings.
///
/// The comparison is based on resolved concrete widths. If a required symbol
/// is unbound, the result is `Indeterminate`. Invalid zero widths and
/// arithmetic overflow are errors and are not downgraded to indeterminate.
///
/// No assumption is made about a target's ability to realize either width.
pub fn compare_widths(
    left: &HdlWidth,
    right: &HdlWidth,
    environment: &WidthEnvironment,
) -> Result<WidthComparison, WidthResolutionError> {
    let left_result = resolve_width(left, environment);
    let right_result = resolve_width(right, environment);

    match (left_result, right_result) {
        (Ok(left), Ok(right)) if left == right => Ok(WidthComparison::Equal),
        (Ok(_), Ok(_)) => Ok(WidthComparison::Unequal),

        (Err(WidthResolutionError::UnboundSymbol { .. }), _)
        | (_, Err(WidthResolutionError::UnboundSymbol { .. })) => {
            // Do not hide a concrete invalid expression merely because the
            // other expression is unresolved.
            match (
                resolve_non_binding_error(left, environment),
                resolve_non_binding_error(right, environment),
            ) {
                (Some(error), _) | (_, Some(error)) => Err(error),
                (None, None) => Ok(WidthComparison::Indeterminate),
            }
        }

        (Err(error), _) | (_, Err(error)) => Err(error),
    }
}

/// Return an error only when resolving the expression fails for a reason
/// other than an unbound symbolic reference.
fn resolve_non_binding_error(
    width: &HdlWidth,
    environment: &WidthEnvironment,
) -> Option<WidthResolutionError> {
    match resolve_width(width, environment) {
        Ok(_) | Err(WidthResolutionError::UnboundSymbol { .. }) => None,
        Err(error) => Some(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity should be valid")
    }

    #[test]
    fn resolves_a_known_width() {
        let environment = WidthEnvironment::new();
        let width = HdlWidth::Known(8);

        assert_eq!(resolve_width(&width, &environment), Ok(8));
    }

    #[test]
    fn resolves_symbolic_addition_and_multiplication() {
        let mut environment = WidthEnvironment::new();
        environment
            .bind(id("lanes"), 4)
            .expect("positive binding");

        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("lanes"))),
            Box::new(HdlWidth::Add(
                Box::new(HdlWidth::Known(2)),
                Box::new(HdlWidth::Known(1)),
            )),
        );

        assert_eq!(resolve_width(&width, &environment), Ok(12));
    }

    #[test]
    fn preserves_unresolved_symbols_without_guessing() {
        let environment = WidthEnvironment::new();
        let width = HdlWidth::Symbol(id("payload_width"));

        assert_eq!(
            resolve_width(&width, &environment),
            Err(WidthResolutionError::UnboundSymbol {
                id: id("payload_width"),
                kind: WidthReferenceKind::Symbol,
            })
        );
    }

    #[test]
    fn an_unresolved_expression_can_be_resolved_by_an_explicit_binding() {
        let mut environment = WidthEnvironment::new();
        environment
            .bind(id("computed_width"), 128)
            .expect("positive binding");

        let width = HdlWidth::Unresolved(id("computed_width"));

        assert_eq!(resolve_width(&width, &environment), Ok(128));
    }

    #[test]
    fn rejects_zero_width_even_if_constructed_directly() {
        let environment = WidthEnvironment::new();

        assert_eq!(
            resolve_width(&HdlWidth::Known(0), &environment),
            Err(WidthResolutionError::ZeroWidth)
        );
    }

    #[test]
    fn rejects_zero_width_bindings() {
        let mut environment = WidthEnvironment::new();

        assert_eq!(
            environment.bind(id("width"), 0),
            Err(WidthResolutionError::ZeroWidth)
        );

        assert!(environment.is_empty());
    }

    #[test]
    fn identical_bindings_are_idempotent() {
        let mut environment = WidthEnvironment::new();

        environment
            .bind(id("width"), 32)
            .expect("initial binding");

        environment
            .bind(id("width"), 32)
            .expect("identical binding");

        assert_eq!(environment.len(), 1);
        assert_eq!(environment.get(&id("width")), Some(32));
    }

    #[test]
    fn conflicting_bindings_do_not_replace_the_original() {
        let mut environment = WidthEnvironment::new();

        environment
            .bind(id("width"), 32)
            .expect("initial binding");

        assert_eq!(
            environment.bind(id("width"), 64),
            Err(WidthResolutionError::ConflictingBinding {
                id: id("width"),
                existing: 32,
                attempted: 64,
            })
        );

        assert_eq!(environment.get(&id("width")), Some(32));
    }

    #[test]
    fn addition_overflow_is_reported() {
        let environment = WidthEnvironment::new();
        let width = HdlWidth::Add(
            Box::new(HdlWidth::Known(u128::MAX)),
            Box::new(HdlWidth::Known(1)),
        );

        assert_eq!(
            resolve_width(&width, &environment),
            Err(WidthResolutionError::ArithmeticOverflow {
                operation: WidthArithmeticOperation::Addition,
            })
        );
    }

    #[test]
    fn multiplication_overflow_is_reported() {
        let environment = WidthEnvironment::new();
        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Known(u128::MAX)),
            Box::new(HdlWidth::Known(2)),
        );

        assert_eq!(
            resolve_width(&width, &environment),
            Err(WidthResolutionError::ArithmeticOverflow {
                operation: WidthArithmeticOperation::Multiplication,
            })
        );
    }

    #[test]
    fn compares_concrete_widths() {
        let environment = WidthEnvironment::new();

        assert_eq!(
            compare_widths(
                &HdlWidth::Known(16),
                &HdlWidth::Known(16),
                &environment,
            ),
            Ok(WidthComparison::Equal)
        );

        assert_eq!(
            compare_widths(
                &HdlWidth::Known(16),
                &HdlWidth::Known(32),
                &environment,
            ),
            Ok(WidthComparison::Unequal)
        );
    }

    #[test]
    fn comparison_is_indeterminate_when_a_binding_is_missing() {
        let environment = WidthEnvironment::new();

        assert_eq!(
            compare_widths(
                &HdlWidth::Symbol(id("width")),
                &HdlWidth::Known(32),
                &environment,
            ),
            Ok(WidthComparison::Indeterminate)
        );
    }

    #[test]
    fn comparison_propagates_invalid_widths_even_if_the_other_is_unbound() {
        let environment = WidthEnvironment::new();

        assert_eq!(
            compare_widths(
                &HdlWidth::Symbol(id("unknown")),
                &HdlWidth::Known(0),
                &environment,
            ),
            Err(WidthResolutionError::ZeroWidth)
        );
    }

    #[test]
    fn symbolic_arithmetic_is_not_evaluated_without_bindings() {
        let environment = WidthEnvironment::new();
        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("lanes"))),
            Box::new(HdlWidth::Known(8)),
        );

        assert!(matches!(
            resolve_width(&width, &environment),
            Err(WidthResolutionError::UnboundSymbol {
                kind: WidthReferenceKind::Symbol,
                ..
            })
        ));
    }

    #[test]
    fn results_are_deterministic() {
        let mut environment = WidthEnvironment::new();
        environment
            .bind(id("lanes"), 8)
            .expect("positive binding");

        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("lanes"))),
            Box::new(HdlWidth::Known(16)),
        );

        let first = resolve_width(&width, &environment);
        let second = resolve_width(&width, &environment);

        assert_eq!(first, second);
        assert_eq!(first, Ok(128));
    }
}
