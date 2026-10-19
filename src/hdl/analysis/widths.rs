
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # HDL Width Analysis
//!
//! Path: `src/hdl/analysis/widths.rs`
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! Resolve and classify the widths of HDL signals using the existing semantic
//! input model. Evaluate concrete and parameter-bound symbolic expressions,
//! preserve unresolved dependencies, detect invalid zero widths, and report
//! arithmetic that cannot be represented by the current semantic width type.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md` defines the HDL portability and scalability rules.
//! - `grammar/hdl/` owns HDL source syntax.
//! - `crate::frontend::semantic::domains::hdl` owns `HdlWidth`, `HdlId`,
//!   `HdlProgram`, `HdlSignal`, and `HdlValueType`.
//! - This module owns width-analysis results, evaluation policy, and
//!   width-analysis operational budgets.
//!
//! This module does not create another AST, semantic model, parser, type
//! system, source-span system, or intermediate representation.
//!
//! ## Does not own
//!
//! - Source parsing, name resolution, or general-purpose type checking.
//! - Parameter declaration or parameter-scope resolution.
//! - Target-specific widths, physical layouts, or resource allocation.
//! - HDLIR, builders, verifiers, passes, serialization, or lowering.
//! - Simulation, synthesis, placement, routing, or timing closure.
//!
//! ## Scalability and arithmetic
//!
//! There are no fixed signal-count, expression-depth, or hardware-width
//! ceilings in this analysis API. Optional budgets are caller-configured
//! operational safeguards, not HDL language restrictions.
//!
//! Evaluation uses an explicit stack rather than recursive traversal.
//! Arithmetic uses checked operations and never silently wraps or truncates.
//!
//! The existing semantic `HdlWidth::Known` representation is `u128`. Thus,
//! this module can resolve concrete results representable by that type;
//! overflow is reported as unrepresentable, not as proof that the HDL program
//! is semantically invalid. Arbitrary-precision concrete widths require an
//! upstream change to the canonical semantic width representation.
//!
//! ## Integration
//!
//! The semantic pipeline validates source semantics with `validate_hdl()`.
//! This module then analyzes signal widths using an optional map of resolved
//! parameter identities to positive concrete widths.
//!
//! An unresolved width is not automatically a semantic error. Callers decide
//! whether unresolved values are acceptable at their current compilation
//! stage. Incomplete analysis must never be treated as proof that every width
//! was resolved.
//!
//! `src/hdl/hdlir/` remains entirely deferred.
//!
//! ## Determinism
//!
//! Binding maps and unresolved-symbol sets use ordered collections.
//! Reported signal results are sorted by signal identity and name.
//! The same input and configuration produce the same results.
//!
//! ## Complexity
//!
//! For the analyzed expression nodes, evaluation is O(N) in traversal work,
//! plus ordered-set costs when combining unresolved symbols. Auxiliary memory
//! is O(N) in the worst case. Actual completion depends on available resources
//! and optional caller-provided budgets.

use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::hdl::{
    HdlId, HdlProgram, HdlSourceLocation, HdlValueType, HdlWidth,
};

/// Configuration for width analysis.
///
/// `None` means that this API does not impose a limit for that budget.
/// It does not promise that the host has unlimited resources.
///
/// Budgets limit analysis work, not the legal size of an HDL program.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WidthAnalysisConfig {
    /// Maximum number of signals to analyze.
    pub max_signals: Option<usize>,

    /// Maximum number of width-expression nodes to visit across the analysis.
    pub max_expression_nodes: Option<usize>,

    /// Maximum total work units, including signal processing and expression
    /// node visits.
    pub max_work: Option<usize>,
}

/// A reason width analysis could not finish.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthAnalysisIncompleteReason {
    /// The configured signal budget prevented another signal from being
    /// analyzed.
    SignalBudgetExceeded,

    /// The configured expression-node budget prevented another expression
    /// node from being evaluated.
    ExpressionNodeBudgetExceeded,

    /// The configured total work budget prevented further analysis.
    WorkBudgetExceeded,
}

/// Completion state of width analysis.
///
/// `Complete` means every eligible signal encountered in the input was
/// processed. It does not mean that all symbolic widths became concrete.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidthAnalysisStatus {
    /// Analysis completed within the configured budgets.
    Complete,

    /// Analysis stopped before all eligible signal widths were processed.
    Incomplete(WidthAnalysisIncompleteReason),
}

impl WidthAnalysisStatus {
    /// Returns `true` only if the analysis completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A width evaluation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthIssue {
    /// A concrete width was zero, but this semantic model requires positive
    /// widths.
    ZeroWidth,

    /// Checked addition or multiplication exceeded the current concrete
    /// width representation.
    ArithmeticOverflow,

    /// An internal evaluator invariant could not be maintained.
    ///
    /// This is an implementation failure, not a source-level semantic error.
    InternalInvariant,
}

/// The outcome of evaluating one signal width.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthEvaluation {
    /// The width resolved to a positive concrete value.
    Resolved(u128),

    /// One or more symbolic identities still require values.
    ///
    /// Identities are sorted and deduplicated.
    Unresolved {
        /// Parameter or expression identities not supplied by the binding map.
        symbols: Vec<HdlId>,
    },

    /// The expression represents an invalid width under the current
    /// semantic width contract.
    Invalid(WidthIssue),

    /// The result cannot be represented by the current concrete width type.
    ///
    /// This is distinct from semantic invalidity.
    Unrepresentable(WidthIssue),

    /// Evaluation of this signal did not finish because a budget was reached.
    Incomplete,
}

impl WidthEvaluation {
    /// Returns `true` when the width resolved to a concrete value.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }
}

/// An analyzed signal width and its source context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalWidthResult {
    /// Stable semantic signal identity.
    pub signal_id: HdlId,

    /// Source-level signal name.
    pub signal_name: String,

    /// Evaluated width or the reason it remains unresolved or unavailable.
    pub evaluation: WidthEvaluation,

    /// Original source location, when available.
    pub location: Option<HdlSourceLocation>,
}

/// Complete output of a width-analysis invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidthAnalysisResult {
    /// Whether all eligible signals were processed.
    pub status: WidthAnalysisStatus,

    /// Results for signals reached by the analysis.
    ///
    /// If the status is incomplete, later signals may not be represented.
    /// An incomplete entry identifies a signal whose expression evaluation
    /// itself could not finish.
    pub signals: Vec<SignalWidthResult>,

    /// Number of eligible signals whose processing began.
    pub signals_analyzed: usize,

    /// Number of expression nodes visited.
    pub expression_nodes_visited: usize,

    /// Number of charged work units.
    pub work_units: usize,
}

impl WidthAnalysisResult {
    /// Returns `true` if analysis completed and every reported width is
    /// concrete and representable.
    ///
    /// Signals with named types are not width-evaluated by this module.
    /// This method therefore describes the reported widths, not the
    /// completeness of whole-program type checking.
    #[must_use]
    pub fn all_reported_widths_resolved(&self) -> bool {
        self.status.is_complete()
            && self
                .signals
                .iter()
                .all(|result| result.evaluation.is_resolved())
    }
}

/// Analyze widths in a semantic HDL program.
///
/// `bindings` maps canonical semantic identities to concrete positive widths.
/// Both `HdlWidth::Symbol` and `HdlWidth::Unresolved` expressions can resolve
/// through this map when a binding is available.
///
/// The function does not mutate the program or the binding map. It does not
/// resolve names, infer parameter values, choose a target, or allocate
/// hardware.
///
/// A zero binding is reported as an invalid width. An absent binding remains
/// symbolic. Arithmetic overflow is reported as unrepresentable instead of
/// wrapping or being silently narrowed.
///
/// Only width-bearing built-in signal types (`Bit`, `BitVector`, `Clock`, and
/// `Reset`) are analyzed. `Named` types are left to the canonical type
/// resolution and type-checking pipeline.
#[must_use]
pub fn analyze_widths(
    program: &HdlProgram,
    bindings: &BTreeMap<HdlId, u128>,
    config: &WidthAnalysisConfig,
) -> WidthAnalysisResult {
    let mut result = WidthAnalysisResult {
        status: WidthAnalysisStatus::Complete,
        signals: Vec::new(),
        signals_analyzed: 0,
        expression_nodes_visited: 0,
        work_units: 0,
    };

    for signal in &program.signals {
        let width_expression = match &signal.value_type {
            HdlValueType::Bit | HdlValueType::Clock | HdlValueType::Reset => {
                None
            }
            HdlValueType::BitVector { width, .. } => Some(width),
            HdlValueType::Named(_) => continue,
        };

        if limit_reached(config.max_signals, result.signals_analyzed) {
            result.status = WidthAnalysisStatus::Incomplete(
                WidthAnalysisIncompleteReason::SignalBudgetExceeded,
            );
            break;
        }

        if limit_reached(config.max_work, result.work_units) {
            result.status = WidthAnalysisStatus::Incomplete(
                WidthAnalysisIncompleteReason::WorkBudgetExceeded,
            );
            break;
        }

        result.work_units = result.work_units.saturating_add(1);
        result.signals_analyzed = result.signals_analyzed.saturating_add(1);

        let evaluation = if let Some(expression) = width_expression {
            match evaluate_width(
                expression,
                bindings,
                config,
                &mut result.expression_nodes_visited,
                &mut result.work_units,
            ) {
                EvaluationOutcome::Complete(value) => value.into_public(),
                EvaluationOutcome::Incomplete(reason) => {
                    result.signals.push(SignalWidthResult {
                        signal_id: signal.id.clone(),
                        signal_name: signal.name.clone(),
                        evaluation: WidthEvaluation::Incomplete,
                        location: signal.location.clone(),
                    });
                    result.status = WidthAnalysisStatus::Incomplete(reason);
                    break;
                }
            }
        } else {
            // Bit, Clock, and Reset have scalar logical width.
            WidthEvaluation::Resolved(1)
        };

        result.signals.push(SignalWidthResult {
            signal_id: signal.id.clone(),
            signal_name: signal.name.clone(),
            evaluation,
            location: signal.location.clone(),
        });
    }

    result.signals.sort_by(|left, right| {
        left.signal_id
            .cmp(&right.signal_id)
            .then_with(|| left.signal_name.cmp(&right.signal_name))
    });

    result
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

enum Frame<'a> {
    Visit(&'a HdlWidth),
    Add,
    Multiply,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EvaluatedWidth {
    Resolved(u128),
    Unresolved(BTreeSet<HdlId>),
    Invalid(WidthIssue),
    Unrepresentable(WidthIssue),
}

impl EvaluatedWidth {
    fn into_public(self) -> WidthEvaluation {
        match self {
            Self::Resolved(value) => WidthEvaluation::Resolved(value),
            Self::Unresolved(symbols) => WidthEvaluation::Unresolved {
                symbols: symbols.into_iter().collect(),
            },
            Self::Invalid(issue) => WidthEvaluation::Invalid(issue),
            Self::Unrepresentable(issue) => {
                WidthEvaluation::Unrepresentable(issue)
            }
        }
    }
}

enum EvaluationOutcome {
    Complete(EvaluatedWidth),
    Incomplete(WidthAnalysisIncompleteReason),
}

fn evaluate_width(
    expression: &HdlWidth,
    bindings: &BTreeMap<HdlId, u128>,
    config: &WidthAnalysisConfig,
    expression_nodes_visited: &mut usize,
    work_units: &mut usize,
) -> EvaluationOutcome {
    let mut frames = vec![Frame::Visit(expression)];
    let mut values = Vec::<EvaluatedWidth>::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Visit(node) => {
                if limit_reached(config.max_work, *work_units) {
                    return EvaluationOutcome::Incomplete(
                        WidthAnalysisIncompleteReason::WorkBudgetExceeded,
                    );
                }

                if limit_reached(
                    config.max_expression_nodes,
                    *expression_nodes_visited,
                ) {
                    return EvaluationOutcome::Incomplete(
                        WidthAnalysisIncompleteReason::ExpressionNodeBudgetExceeded,
                    );
                }

                *work_units = work_units.saturating_add(1);
                *expression_nodes_visited =
                    expression_nodes_visited.saturating_add(1);

                match node {
                    HdlWidth::Known(0) => {
                        values.push(EvaluatedWidth::Invalid(
                            WidthIssue::ZeroWidth,
                        ));
                    }
                    HdlWidth::Known(value) => {
                        values.push(EvaluatedWidth::Resolved(*value));
                    }
                    HdlWidth::Symbol(id) | HdlWidth::Unresolved(id) => {
                        values.push(resolve_symbol(id, bindings));
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
                }
            }
            Frame::Add => {
                let Some(right) = values.pop() else {
                    return EvaluationOutcome::Complete(
                        EvaluatedWidth::Unrepresentable(
                            WidthIssue::InternalInvariant,
                        ),
                    );
                };
                let Some(left) = values.pop() else {
                    return EvaluationOutcome::Complete(
                        EvaluatedWidth::Unrepresentable(
                            WidthIssue::InternalInvariant,
                        ),
                    );
                };
                values.push(combine(left, right, Operator::Add));
            }
            Frame::Multiply => {
                let Some(right) = values.pop() else {
                    return EvaluationOutcome::Complete(
                        EvaluatedWidth::Unrepresentable(
                            WidthIssue::InternalInvariant,
                        ),
                    );
                };
                let Some(left) = values.pop() else {
                    return EvaluationOutcome::Complete(
                        EvaluatedWidth::Unrepresentable(
                            WidthIssue::InternalInvariant,
                        ),
                    );
                };
                values.push(combine(left, right, Operator::Multiply));
            }
        }
    }

    match values.pop() {
        Some(value) if values.is_empty() => {
            EvaluationOutcome::Complete(value)
        }
        _ => EvaluationOutcome::Complete(EvaluatedWidth::Unrepresentable(
            WidthIssue::InternalInvariant,
        )),
    }
}

fn resolve_symbol(
    id: &HdlId,
    bindings: &BTreeMap<HdlId, u128>,
) -> EvaluatedWidth {
    match bindings.get(id) {
        Some(0) => EvaluatedWidth::Invalid(WidthIssue::ZeroWidth),
        Some(value) => EvaluatedWidth::Resolved(*value),
        None => EvaluatedWidth::Unresolved(BTreeSet::from([id.clone()])),
    }
}

fn combine(
    left: EvaluatedWidth,
    right: EvaluatedWidth,
    operator: Operator,
) -> EvaluatedWidth {
    use EvaluatedWidth::{
        Invalid, Resolved, Unrepresentable, Unresolved,
    };

    match (left, right) {
        (Invalid(issue), _) | (_, Invalid(issue)) => Invalid(issue),

        (Unrepresentable(issue), _) | (_, Unrepresentable(issue)) => {
            Unrepresentable(issue)
        }

        (Unresolved(mut left_symbols), Unresolved(right_symbols)) => {
            left_symbols.extend(right_symbols);
            Unresolved(left_symbols)
        }

        (Unresolved(symbols), Resolved(_))
        | (Resolved(_), Unresolved(symbols)) => Unresolved(symbols),

        (Resolved(left_value), Resolved(right_value)) => {
            let value = match operator {
                Operator::Add => left_value.checked_add(right_value),
                Operator::Multiply => left_value.checked_mul(right_value),
            };

            match value {
                Some(0) => Invalid(WidthIssue::ZeroWidth),
                Some(value) => Resolved(value),
                None => Unrepresentable(WidthIssue::ArithmeticOverflow),
            }
        }
    }
}

fn limit_reached(limit: Option<usize>, current: usize) -> bool {
    limit.is_some_and(|maximum| current >= maximum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::{
        HdlDirection, HdlId, HdlSignal, HdlSignedness,
    };

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be valid")
    }

    fn vector_signal(name: &str, width: HdlWidth) -> HdlSignal {
        HdlSignal {
            id: id(name),
            name: name.to_owned(),
            direction: HdlDirection::Internal,
            value_type: HdlValueType::BitVector {
                width,
                signedness: HdlSignedness::Unsigned,
            },
            location: None,
        }
    }

    #[test]
    fn scalar_signal_has_width_one() {
        let program = HdlProgram {
            signals: vec![HdlSignal {
                id: id("flag"),
                name: "flag".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::Bit,
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(result.status, WidthAnalysisStatus::Complete);
        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Resolved(1)
        );
    }

    #[test]
    fn concrete_width_is_preserved() {
        let program = HdlProgram {
            signals: vec![vector_signal("data", HdlWidth::Known(128))],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Resolved(128)
        );
        assert!(result.all_reported_widths_resolved());
    }

    #[test]
    fn symbolic_width_resolves_from_binding() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "payload",
                HdlWidth::Symbol(id("width")),
            )],
            ..HdlProgram::default()
        };
        let bindings = BTreeMap::from([(id("width"), 256)]);

        let result = analyze_widths(
            &program,
            &bindings,
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Resolved(256)
        );
    }

    #[test]
    fn unresolved_symbols_are_reported_deterministically() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "payload",
                HdlWidth::Add(
                    Box::new(HdlWidth::Symbol(id("z"))),
                    Box::new(HdlWidth::Symbol(id("a"))),
                ),
            )],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Unresolved {
                symbols: vec![id("a"), id("z")]
            }
        );
        assert!(!result.all_reported_widths_resolved());
    }

    #[test]
    fn symbolic_arithmetic_resolves_after_binding() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "payload",
                HdlWidth::Multiply(
                    Box::new(HdlWidth::Symbol(id("lanes"))),
                    Box::new(HdlWidth::Known(8)),
                ),
            )],
            ..HdlProgram::default()
        };
        let bindings = BTreeMap::from([(id("lanes"), 4)]);

        let result = analyze_widths(
            &program,
            &bindings,
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Resolved(32)
        );
    }

    #[test]
    fn zero_width_is_invalid() {
        let program = HdlProgram {
            signals: vec![vector_signal("invalid", HdlWidth::Known(0))],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Invalid(WidthIssue::ZeroWidth)
        );
    }

    #[test]
    fn arithmetic_overflow_is_not_silently_wrapped() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "too_wide",
                HdlWidth::Add(
                    Box::new(HdlWidth::Known(u128::MAX)),
                    Box::new(HdlWidth::Known(1)),
                ),
            )],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Unrepresentable(
                WidthIssue::ArithmeticOverflow
            )
        );
        assert!(result.status.is_complete());
    }

    #[test]
    fn zero_binding_is_invalid() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "payload",
                HdlWidth::Symbol(id("width")),
            )],
            ..HdlProgram::default()
        };
        let bindings = BTreeMap::from([(id("width"), 0)]);

        let result = analyze_widths(
            &program,
            &bindings,
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Invalid(WidthIssue::ZeroWidth)
        );
    }

    #[test]
    fn zero_work_budget_does_not_claim_completion() {
        let program = HdlProgram {
            signals: vec![vector_signal("data", HdlWidth::Known(8))],
            ..HdlProgram::default()
        };
        let config = WidthAnalysisConfig {
            max_work: Some(0),
            ..WidthAnalysisConfig::default()
        };

        let result = analyze_widths(&program, &BTreeMap::new(), &config);

        assert_eq!(
            result.status,
            WidthAnalysisStatus::Incomplete(
                WidthAnalysisIncompleteReason::WorkBudgetExceeded
            )
        );
        assert!(result.signals.is_empty());
    }

    #[test]
    fn expression_budget_exhaustion_is_explicit() {
        let program = HdlProgram {
            signals: vec![vector_signal(
                "data",
                HdlWidth::Add(
                    Box::new(HdlWidth::Known(8)),
                    Box::new(HdlWidth::Known(8)),
                ),
            )],
            ..HdlProgram::default()
        };
        let config = WidthAnalysisConfig {
            max_expression_nodes: Some(1),
            ..WidthAnalysisConfig::default()
        };

        let result = analyze_widths(&program, &BTreeMap::new(), &config);

        assert_eq!(
            result.status,
            WidthAnalysisStatus::Incomplete(
                WidthAnalysisIncompleteReason::ExpressionNodeBudgetExceeded
            )
        );
        assert_eq!(
            result.signals[0].evaluation,
            WidthEvaluation::Incomplete
        );
    }

    #[test]
    fn signal_budget_exhaustion_is_explicit() {
        let program = HdlProgram {
            signals: vec![
                vector_signal("first", HdlWidth::Known(8)),
                vector_signal("second", HdlWidth::Known(16)),
            ],
            ..HdlProgram::default()
        };
        let config = WidthAnalysisConfig {
            max_signals: Some(1),
            ..WidthAnalysisConfig::default()
        };

        let result = analyze_widths(&program, &BTreeMap::new(), &config);

        assert_eq!(
            result.status,
            WidthAnalysisStatus::Incomplete(
                WidthAnalysisIncompleteReason::SignalBudgetExceeded
            )
        );
        assert_eq!(result.signals.len(), 1);
    }

    #[test]
    fn result_order_is_stable_by_identity() {
        let program = HdlProgram {
            signals: vec![
                vector_signal("z", HdlWidth::Known(8)),
                vector_signal("a", HdlWidth::Known(16)),
            ],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(result.signals[0].signal_id, id("a"));
        assert_eq!(result.signals[1].signal_id, id("z"));
    }

    #[test]
    fn named_types_are_left_to_type_resolution() {
        let program = HdlProgram {
            signals: vec![HdlSignal {
                id: id("custom"),
                name: "custom".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::Named(id("CustomType")),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = analyze_widths(
            &program,
            &BTreeMap::new(),
            &WidthAnalysisConfig::default(),
        );

        assert_eq!(result.status, WidthAnalysisStatus::Complete);
        assert!(result.signals.is_empty());
    }
}
