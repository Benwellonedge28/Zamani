
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Width Analysis
//!
//! Path: `src/hdl/hdlir/analysis/widths.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Analyze concrete and symbolic HDLIR bit widths without replacing the
//! canonical frontend width model or imposing hardware-capacity limits.
//!
//! This module reports resolved widths, unresolved symbolic expressions,
//! invalid arithmetic, and incomplete analysis separately. It never guesses
//! a missing binding, truncates an arithmetic result, or reports incomplete
//! work as successful analysis.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/types/bit_width.rs`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//!
//! The frontend owns source-level semantics. `BitWidth` owns the HDLIR
//! representation and checked evaluation of width expressions. This module
//! owns orchestration of width analysis and the status of its results.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, name resolution, or source type checking.
//! - A replacement for `HdlWidth` or `BitWidth`.
//! - HDLIR graph definitions, operation schemas, or signal ownership.
//! - Physical device capacity, resource allocation, placement, or routing.
//! - A claim that a resolved width is feasible on a particular target.
//!
//! ## Scalability
//!
//! No maximum hardware width or fixed number of width queries is imposed by
//! default. Optional budgets constrain analysis work, not HDL semantics.
//! Concrete widths follow the existing `u128` contract. Arithmetic overflow
//! is explicit; symbolic expressions are retained in their original form.
//!
//! Traversals are iterative. Collection allocation and work remain subject
//! to available resources and representational limits. No finite program
//! can process literally infinite input.
//!
//! ## Integration
//!
//! - `types/bit_width.rs` provides `BitWidth` and `WidthEvaluationError`.
//! - `analysis/mod.rs` must register this module and re-export its intended
//!   public API.
//! - `verify/types.rs` may consume these results, but must not treat an
//!   unresolved width as a proven-compatible width.
//! - `ir/ports.rs`, `ir/connectivity.rs`, and `ir/memories.rs` may submit
//!   width queries using stable, caller-supplied labels.
//! - `lowering/from_semantic.rs` must map canonical `HdlWidth` variants to
//!   `BitWidth` without discarding symbolic expression structure.
//! - `config.rs` remains the authority for shared HDLIR configuration.
//!
//! ## Completion contract
//!
//! All required queries must be analyzed, resolved, or explicitly reported
//! unresolved for the result to be complete. Budget exhaustion is incomplete,
//! not success. A resolved width establishes only its numeric value; it does
//! not establish target feasibility or semantic compatibility.

use core::fmt;

use crate::hdl::hdlir::types::bit_width::{
    BitWidth,
    WidthEvaluationBudget,
    WidthEvaluationError,
};

/// Stable identifier supplied by the caller for one width-analysis query.
///
/// This is a diagnostic label, not an HDLIR ID, source symbol, or hardware
/// resource identity. Callers should use deterministic labels when stable
/// output is required.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WidthQueryId(String);

impl WidthQueryId {
    /// Construct a query identifier from a non-empty label.
    ///
    /// The label is retained without normalization. Its uniqueness within
    /// an analysis batch is checked by [`WidthAnalysis::analyze`].
    pub fn new(label: impl Into<String>) -> Result<Self, WidthAnalysisError> {
        let label = label.into();

        if label.trim().is_empty() {
            return Err(WidthAnalysisError::EmptyQueryId);
        }

        Ok(Self(label))
    }

    /// Return the retained label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WidthQueryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A request to analyze one HDLIR width expression.
///
/// The expression is borrowed by the analysis request. Its symbolic form is
/// not modified, specialized, or replaced by a computed result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidthQuery {
    /// Caller-supplied stable label for this query.
    pub id: WidthQueryId,
    /// Width expression to analyze.
    pub width: BitWidth,
}

impl WidthQuery {
    /// Construct a width-analysis request.
    #[must_use]
    pub fn new(id: WidthQueryId, width: BitWidth) -> Self {
        Self { id, width }
    }
}

/// Explicit operational limits for one analysis invocation.
///
/// `None` means this analysis configuration does not impose a limit for that
/// unit of work. It does not promise unlimited memory or execution time.
///
/// The query count and expression-node count are independent budgets.
/// Neither represents a maximum hardware width, signal count, or design size.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WidthAnalysisBudget {
    /// Maximum number of queries to begin processing.
    ///
    /// `None` imposes no explicit query-count budget.
    pub max_queries: Option<u64>,

    /// Maximum total expression nodes to inspect across all queries.
    ///
    /// `None` imposes no explicit expression-node budget.
    pub max_expression_nodes: Option<u64>,
}

impl WidthAnalysisBudget {
    /// Create a budget without explicit query or expression-node limits.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_queries: None,
            max_expression_nodes: None,
        }
    }

    /// Create a budget with explicit query and expression-node limits.
    #[must_use]
    pub const fn new(
        max_queries: Option<u64>,
        max_expression_nodes: Option<u64>,
    ) -> Self {
        Self {
            max_queries,
            max_expression_nodes,
        }
    }
}

/// Classification of one width-analysis result.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthStatus {
    /// The expression was evaluated to a concrete positive width.
    Resolved {
        /// Evaluated width in bits.
        bits: u128,
    },

    /// The expression is validly represented but lacks one or more bindings.
    ///
    /// The original expression remains available in the corresponding query.
    /// This status is not proof of compatibility with another width.
    Symbolic {
        /// Symbol or unresolved-expression name whose binding was unavailable.
        missing_symbol: String,
    },

    /// Concrete arithmetic overflowed the existing `u128` representation.
    ArithmeticOverflow {
        /// Arithmetic operation that failed.
        operation: &'static str,
        /// Left operand.
        lhs: u128,
        /// Right operand.
        rhs: u128,
    },

    /// A resolver supplied a zero width, which is invalid for `BitWidth`.
    InvalidResolvedWidth {
        /// Symbol that resolved to zero.
        symbol: String,
    },

    /// Evaluation failed because the width expression or evaluator state
    /// violated an invariant.
    InvalidExpression {
        /// Explanation of the failure.
        reason: String,
    },
}

impl WidthStatus {
    /// Return the concrete width when evaluation succeeded.
    #[must_use]
    pub const fn resolved_bits(&self) -> Option<u128> {
        match self {
            Self::Resolved { bits } => Some(*bits),
            Self::Symbolic { .. }
            | Self::ArithmeticOverflow { .. }
            | Self::InvalidResolvedWidth { .. }
            | Self::InvalidExpression { .. } => None,
        }
    }

    /// Return whether the query produced a concrete width.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved { .. })
    }
}

/// Result for a single width-analysis request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidthResult {
    /// Identifier supplied by the caller.
    pub id: WidthQueryId,

    /// Number of expression nodes inspected for this query.
    ///
    /// This is zero if the query was not started.
    pub inspected_nodes: u64,

    /// The query's analysis result, if processing began.
    ///
    /// `None` means the query was not processed because a budget was
    /// exhausted before it began.
    pub status: Option<WidthStatus>,
}

/// Why a width-analysis batch did not complete.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthIncompleteReason {
    /// The configured query-count budget was exhausted.
    QueryBudgetExceeded {
        /// Configured query limit.
        limit: u64,
        /// Number of queries begun.
        processed: u64,
    },

    /// The configured expression-node budget prevented a query from starting
    /// or prevented inspection of the complete expression.
    ExpressionNodeBudgetExceeded {
        /// Configured node limit.
        limit: u64,
        /// Nodes already inspected.
        inspected: u64,
        /// Query that could not be fully inspected.
        query: WidthQueryId,
    },

    /// A counter exceeded its representable range.
    CounterOverflow,
}

/// Whether every query in the requested batch was processed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidthAnalysisCompletion {
    /// Every query was processed, including queries with symbolic or invalid
    /// expressions. Completion does not imply every width was resolved.
    Complete,

    /// One or more queries were not fully processed.
    Incomplete(WidthIncompleteReason),
}

/// Aggregate result for a batch of width-analysis requests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidthAnalysisReport {
    /// Per-query results in input order.
    pub results: Vec<WidthResult>,

    /// Whether every requested query was processed.
    pub completion: WidthAnalysisCompletion,

    /// Total number of expression nodes inspected.
    pub inspected_nodes: u64,

    /// Number of queries whose processing began.
    pub processed_queries: u64,
}

impl WidthAnalysisReport {
    /// Return whether the batch completed all requested work.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completion, WidthAnalysisCompletion::Complete)
    }

    /// Return the number of queries that produced concrete widths.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                result
                    .status
                    .as_ref()
                    .is_some_and(WidthStatus::is_resolved)
            })
            .count()
    }

    /// Return the number of queries whose width remains symbolic.
    #[must_use]
    pub fn symbolic_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| {
                matches!(
                    result.status,
                    Some(WidthStatus::Symbolic { .. })
                )
            })
            .count()
    }

    /// Return whether every processed query has a concrete result.
    ///
    /// An incomplete report never satisfies this predicate, even when every
    /// query that happened to run was resolved.
    #[must_use]
    pub fn all_resolved(&self) -> bool {
        self.is_complete()
            && self.results.iter().all(|result| {
                result
                    .status
                    .as_ref()
                    .is_some_and(WidthStatus::is_resolved)
            })
    }

    /// Find a result by its caller-supplied identifier.
    #[must_use]
    pub fn get(&self, id: &WidthQueryId) -> Option<&WidthResult> {
        self.results.iter().find(|result| &result.id == id)
    }
}

/// Errors in width-analysis request construction or batch validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WidthAnalysisError {
    /// A query identifier is empty.
    EmptyQueryId,

    /// Two requests have the same identifier.
    DuplicateQueryId {
        /// Duplicated identifier.
        id: WidthQueryId,
    },

    /// A query count or work counter cannot be represented.
    CounterOverflow,
}

impl fmt::Display for WidthAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQueryId => {
                formatter.write_str("width-analysis query identifier cannot be empty")
            }
            Self::DuplicateQueryId { id } => {
                write!(formatter, "duplicate width-analysis query identifier `{id}`")
            }
            Self::CounterOverflow => {
                formatter.write_str("width-analysis counter overflowed")
            }
        }
    }
}

impl std::error::Error for WidthAnalysisError {}

/// Stateless width-analysis engine.
///
/// The engine owns no global state, caches, resolver, hardware target, or
/// mutable design graph. Bindings are supplied for each invocation so
/// equivalent inputs and a deterministic resolver can yield deterministic
/// results.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WidthAnalysis;

impl WidthAnalysis {
    /// Create a width-analysis engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Analyze a batch of symbolic width expressions.
    ///
    /// `resolve` maps a symbol name to a concrete positive width. Returning
    /// `None` leaves that expression symbolic. Resolver behavior must be
    /// deterministic for deterministic results; it should not depend on
    /// external mutable state or iteration order.
    ///
    /// All query identifiers are validated before processing begins.
    /// Results preserve input order. The input expressions are not modified.
    ///
    /// Expression traversal is iterative. The node budget is checked before
    /// each expression is evaluated. If a query cannot be fully inspected,
    /// the report is marked incomplete and does not claim that query was
    /// resolved.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate query IDs or invalid request identity.
    /// Width-specific evaluation failures are represented in each result.
    pub fn analyze<F>(
        &self,
        queries: &[WidthQuery],
        budget: WidthAnalysisBudget,
        mut resolve: F,
    ) -> Result<WidthAnalysisReport, WidthAnalysisError>
    where
        F: FnMut(&str) -> Option<u128>,
    {
        Self::validate_unique_ids(queries)?;

        let mut results = Vec::new();
        let mut inspected_nodes = 0_u64;
        let mut processed_queries = 0_u64;

        for (index, query) in queries.iter().enumerate() {
            if let Some(limit) = budget.max_queries {
                if processed_queries >= limit {
                    let remaining = &queries[index..];
                    results.extend(remaining.iter().map(|pending| WidthResult {
                        id: pending.id.clone(),
                        inspected_nodes: 0,
                        status: None,
                    }));

                    return Ok(WidthAnalysisReport {
                        results,
                        completion: WidthAnalysisCompletion::Incomplete(
                            WidthIncompleteReason::QueryBudgetExceeded {
                                limit,
                                processed: processed_queries,
                            },
                        ),
                        inspected_nodes,
                        processed_queries,
                    });
                }
            }

            let remaining_nodes = match budget.max_expression_nodes {
                Some(limit) => limit.checked_sub(inspected_nodes).unwrap_or(0),
                None => u64::MAX,
            };

            let node_count = match Self::count_nodes(&query.width, remaining_nodes) {
                Ok(count) => count,
                Err(CountNodesError::BudgetExceeded) => {
                    results.push(WidthResult {
                        id: query.id.clone(),
                        inspected_nodes: remaining_nodes,
                        status: None,
                    });

                    let pending = &queries[index + 1..];
                    results.extend(pending.iter().map(|next| WidthResult {
                        id: next.id.clone(),
                        inspected_nodes: 0,
                        status: None,
                    }));

                    return Ok(WidthAnalysisReport {
                        results,
                        completion: WidthAnalysisCompletion::Incomplete(
                            WidthIncompleteReason::ExpressionNodeBudgetExceeded {
                                limit: budget.max_expression_nodes.unwrap_or(u64::MAX),
                                inspected: inspected_nodes,
                                query: query.id.clone(),
                            },
                        ),
                        inspected_nodes,
                        processed_queries,
                    });
                }
                Err(CountNodesError::CounterOverflow) => {
                    results.push(WidthResult {
                        id: query.id.clone(),
                        inspected_nodes: 0,
                        status: Some(WidthStatus::InvalidExpression {
                            reason: "expression node counter overflowed".to_owned(),
                        }),
                    });

                    return Ok(WidthAnalysisReport {
                        results,
                        completion: WidthAnalysisCompletion::Incomplete(
                            WidthIncompleteReason::CounterOverflow,
                        ),
                        inspected_nodes,
                        processed_queries,
                    });
                }
            };

            // Count first, then evaluate with exactly enough node budget for
            // the complete expression. This prevents a partial evaluation
            // from being mistaken for a complete result.
            inspected_nodes = inspected_nodes
                .checked_add(node_count)
                .ok_or(WidthAnalysisError::CounterOverflow)?;

            processed_queries = processed_queries
                .checked_add(1)
                .ok_or(WidthAnalysisError::CounterOverflow)?;

            let eval_limit = usize::try_from(node_count)
                .map_err(|_| WidthAnalysisError::CounterOverflow)?;

            let status = Self::evaluate_status(
                &query.width,
                WidthEvaluationBudget::new(eval_limit),
                &mut resolve,
            );

            results.push(WidthResult {
                id: query.id.clone(),
                inspected_nodes: node_count,
                status: Some(status),
            });
        }

        Ok(WidthAnalysisReport {
            results,
            completion: WidthAnalysisCompletion::Complete,
            inspected_nodes,
            processed_queries,
        })
    }

    fn validate_unique_ids(queries: &[WidthQuery]) -> Result<(), WidthAnalysisError> {
        let mut ids = std::collections::BTreeSet::new();

        for query in queries {
            if query.id.as_str().trim().is_empty() {
                return Err(WidthAnalysisError::EmptyQueryId);
            }

            if !ids.insert(query.id.clone()) {
                return Err(WidthAnalysisError::DuplicateQueryId {
                    id: query.id.clone(),
                });
            }
        }

        Ok(())
    }

    fn count_nodes(
        width: &BitWidth,
        limit: u64,
    ) -> Result<u64, CountNodesError> {
        let mut pending = vec![width];
        let mut count = 0_u64;

        while let Some(node) = pending.pop() {
            count = count
                .checked_add(1)
                .ok_or(CountNodesError::CounterOverflow)?;

            if count > limit {
                return Err(CountNodesError::BudgetExceeded);
            }

            match node {
                BitWidth::Known(_) | BitWidth::Symbol(_) | BitWidth::Unresolved(_) => {}
                BitWidth::Add(lhs, rhs) | BitWidth::Multiply(lhs, rhs) => {
                    pending.push(rhs);
                    pending.push(lhs);
                }
            }
        }

        Ok(count)
    }

    fn evaluate_status<F>(
        width: &BitWidth,
        budget: WidthEvaluationBudget,
        resolve: &mut F,
    ) -> WidthStatus
    where
        F: FnMut(&str) -> Option<u128>,
    {
        match width.evaluate_with(budget, |symbol| resolve(symbol)) {
            Ok(bits) => WidthStatus::Resolved { bits },

            Err(WidthEvaluationError::UnboundSymbol(symbol))
            | Err(WidthEvaluationError::UnresolvedWidth(symbol)) => {
                WidthStatus::Symbolic {
                    missing_symbol: symbol.to_string(),
                }
            }

            Err(WidthEvaluationError::ArithmeticOverflow {
                operation,
                lhs,
                rhs,
            }) => WidthStatus::ArithmeticOverflow {
                operation: match operation {
                    crate::hdl::hdlir::types::bit_width::WidthArithmetic::Add => "addition",
                    crate::hdl::hdlir::types::bit_width::WidthArithmetic::Multiply => {
                        "multiplication"
                    }
                },
                lhs,
                rhs,
            },

            Err(WidthEvaluationError::ZeroResolvedWidth(symbol)) => {
                WidthStatus::InvalidResolvedWidth {
                    symbol: symbol.to_string(),
                }
            }

            Err(error) => WidthStatus::InvalidExpression {
                reason: error.to_string(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CountNodesError {
    BudgetExceeded,
    CounterOverflow,
}

#[cfg(test)]
mod tests {
    use super::{
        WidthAnalysis,
        WidthAnalysisBudget,
        WidthAnalysisCompletion,
        WidthAnalysisError,
        WidthIncompleteReason,
        WidthQuery,
        WidthQueryId,
        WidthStatus,
    };
    use crate::hdl::hdlir::types::bit_width::BitWidth;

    fn query(id: &str, width: BitWidth) -> WidthQuery {
        WidthQuery::new(
            WidthQueryId::new(id).expect("test query ID must be valid"),
            width,
        )
    }

    #[test]
    fn concrete_width_resolves_without_a_binding() {
        let queries = [query("bus.data", BitWidth::known(32).unwrap())];

        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |_| None)
            .unwrap();

        assert!(report.is_complete());
        assert!(report.all_resolved());
        assert_eq!(report.inspected_nodes, 1);
        assert_eq!(
            report.results[0].status,
            Some(WidthStatus::Resolved { bits: 32 })
        );
    }

    #[test]
    fn symbolic_width_is_preserved_when_binding_is_missing() {
        let queries = [query(
            "bus.payload",
            BitWidth::symbol("DATA_WIDTH").unwrap(),
        )];

        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |_| None)
            .unwrap();

        assert!(report.is_complete());
        assert!(!report.all_resolved());
        assert_eq!(
            report.results[0].status,
            Some(WidthStatus::Symbolic {
                missing_symbol: "DATA_WIDTH".to_owned()
            })
        );
    }

    #[test]
    fn symbolic_expression_resolves_with_bindings() {
        let width = BitWidth::multiply(
            BitWidth::symbol("LANES").unwrap(),
            BitWidth::known(8).unwrap(),
        );

        let queries = [query("bus.payload", width)];
        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |symbol| {
                (symbol == "LANES").then_some(4)
            })
            .unwrap();

        assert!(report.all_resolved());
        assert_eq!(
            report.results[0].status,
            Some(WidthStatus::Resolved { bits: 32 })
        );
    }

    #[test]
    fn arithmetic_overflow_is_reported_not_wrapped() {
        let width = BitWidth::add(
            BitWidth::known(u128::MAX).unwrap(),
            BitWidth::known(1).unwrap(),
        );

        let queries = [query("bus.overflow", width)];
        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |_| None)
            .unwrap();

        assert!(matches!(
            report.results[0].status,
            Some(WidthStatus::ArithmeticOverflow {
                operation: "addition",
                ..
            })
        ));
    }

    #[test]
    fn zero_from_resolver_is_invalid() {
        let queries = [query(
            "bus.data",
            BitWidth::symbol("DATA_WIDTH").unwrap(),
        )];

        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |_| Some(0))
            .unwrap();

        assert!(matches!(
            report.results[0].status,
            Some(WidthStatus::InvalidResolvedWidth { .. })
        ));
    }

    #[test]
    fn expression_budget_exhaustion_is_incomplete() {
        let width = BitWidth::add(
            BitWidth::known(8).unwrap(),
            BitWidth::known(8).unwrap(),
        );

        let queries = [query("bus.data", width)];
        let report = WidthAnalysis::new()
            .analyze(
                &queries,
                WidthAnalysisBudget::new(None, Some(2)),
                |_| None,
            )
            .unwrap();

        assert!(!report.is_complete());
        assert!(!report.all_resolved());
        assert!(matches!(
            report.completion,
            WidthAnalysisCompletion::Incomplete(
                WidthIncompleteReason::ExpressionNodeBudgetExceeded { .. }
            )
        ));
    }

    #[test]
    fn query_budget_marks_unprocessed_queries() {
        let queries = [
            query("a", BitWidth::known(1).unwrap()),
            query("b", BitWidth::known(2).unwrap()),
        ];

        let report = WidthAnalysis::new()
            .analyze(
                &queries,
                WidthAnalysisBudget::new(Some(1), None),
                |_| None,
            )
            .unwrap();

        assert!(!report.is_complete());
        assert_eq!(report.processed_queries, 1);
        assert!(report.results[0].status.is_some());
        assert!(report.results[1].status.is_none());
        assert!(matches!(
            report.completion,
            WidthAnalysisCompletion::Incomplete(
                WidthIncompleteReason::QueryBudgetExceeded { .. }
            )
        ));
    }

    #[test]
    fn duplicate_query_ids_are_rejected_before_analysis() {
        let queries = [
            query("same", BitWidth::known(1).unwrap()),
            query("same", BitWidth::known(2).unwrap()),
        ];

        assert!(matches!(
            WidthAnalysis::new().analyze(
                &queries,
                WidthAnalysisBudget::unbounded(),
                |_| None
            ),
            Err(WidthAnalysisError::DuplicateQueryId { .. })
        ));
    }

    #[test]
    fn input_order_is_preserved() {
        let queries = [
            query("z", BitWidth::known(1).unwrap()),
            query("a", BitWidth::known(2).unwrap()),
        ];

        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::unbounded(), |_| None)
            .unwrap();

        assert_eq!(report.results[0].id.as_str(), "z");
        assert_eq!(report.results[1].id.as_str(), "a");
    }

    #[test]
    fn zero_query_budget_completes_no_queries() {
        let queries = [query("data", BitWidth::known(8).unwrap())];

        let report = WidthAnalysis::new()
            .analyze(&queries, WidthAnalysisBudget::new(Some(0), None), |_| None)
            .unwrap();

        assert!(!report.is_complete());
        assert_eq!(report.processed_queries, 0);
        assert!(report.results[0].status.is_none());
    }
}
