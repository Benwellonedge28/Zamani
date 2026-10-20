
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Logical Resource Analysis
//!
//! Path: `src/hdl/hdlir/analysis/resources.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Analyze logical memory-size estimates and aggregate caller-supplied
//! resource observations. Preserve symbolic and unavailable quantities.
//!
//! ## Authority and ownership
//!
//! The canonical grammar and frontend semantic model own source-level
//! requirements, constraints, preferences, hints, policies, and capability
//! requirements. `ir/memories.rs` owns HDLIR memory declarations.
//! `types/memory.rs` owns memory dimensions and element-type contracts.
//! `verify/resources.rs` owns resource-contract verification.
//!
//! This module computes read-only analysis facts. It does not define a
//! competing semantic resource-requirement system, select a target, allocate
//! physical resources, synthesize hardware, or prove physical feasibility.
//!
//! ## Scalability
//!
//! No universal resource or hardware-count ceiling is imposed. Symbolic
//! quantities remain symbolic. Arithmetic is checked. Explicit budgets
//! constrain analysis work rather than language semantics. Incomplete work
//! is reported as incomplete.
//!
//! ## Estimate semantics
//!
//! A concrete logical memory-size estimate is reported only when every
//! dimension and the element bit width are known and their product is
//! representable by `u128`. It is not a physical allocation estimate.
//!
//! Caller-supplied resource observations are already-normalized analysis
//! facts. Their names and units are preserved; this module does not infer
//! that differently named metrics are interchangeable.

use std::collections::BTreeMap;

use crate::hdl::hdlir::ir::memories::HdlirMemoryTable;
use crate::hdl::hdlir::types::memory::MemoryDepth;
use crate::hdl::hdlir::types::signal::SignalType;

/// Explicit work budgets for logical resource analysis.
///
/// `None` means no explicit limit for that category. It does not promise
/// infinite memory, execution time, or representable collection capacity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceAnalysisBudget {
    /// Maximum number of memory declarations to inspect.
    pub max_memories: Option<u64>,

    /// Maximum number of memory dimensions to inspect.
    pub max_dimensions: Option<u64>,

    /// Maximum number of caller-supplied resource facts to inspect.
    pub max_facts: Option<u64>,

    /// Maximum number of findings to retain.
    pub max_findings: Option<u64>,
}

impl ResourceAnalysisBudget {
    /// Return a budget with no explicit work limits.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_memories: None,
            max_dimensions: None,
            max_facts: None,
            max_findings: None,
        }
    }
}

/// A normalized resource quantity supplied by an upstream semantic or
/// dialect adapter.
///
/// This is an analysis input, not a replacement for canonical semantic
/// resource expressions or requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceQuantity {
    /// A known non-negative integer quantity in the associated metric's unit.
    Known(u128),

    /// A symbolic quantity whose expression has not been evaluated.
    ///
    /// The expression is retained as supplied. This module does not parse or
    /// reinterpret it.
    Symbolic(String),

    /// The quantity is not currently available.
    Unavailable,
}

/// One resource observation normalized by an upstream adapter.
///
/// Metric names and units must be meaningful under that adapter's contract.
/// For example, two metrics with different units must not be combined merely
/// because their names are similar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceFact {
    /// Extensible logical resource metric name.
    pub metric: String,

    /// Unit associated with the metric, such as `bits` or `bytes`.
    pub unit: String,

    /// Known, symbolic, or unavailable quantity.
    pub quantity: ResourceQuantity,

    /// Optional provenance describing the source of this observation.
    pub origin: Option<String>,
}

impl ResourceFact {
    /// Construct a resource observation.
    ///
    /// # Errors
    ///
    /// Returns an error if the metric or unit is empty or whitespace-only.
    pub fn new(
        metric: impl Into<String>,
        unit: impl Into<String>,
        quantity: ResourceQuantity,
        origin: Option<String>,
    ) -> Result<Self, ResourceAnalysisInputError> {
        let metric = metric.into();
        let unit = unit.into();

        if metric.trim().is_empty() {
            return Err(ResourceAnalysisInputError::EmptyMetric);
        }
        if unit.trim().is_empty() {
            return Err(ResourceAnalysisInputError::EmptyUnit);
        }

        Ok(Self {
            metric,
            unit,
            quantity,
            origin,
        })
    }
}

/// Invalid caller-supplied analysis input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceAnalysisInputError {
    /// A resource metric name is empty.
    EmptyMetric,

    /// A resource unit is empty.
    EmptyUnit,
}

impl std::fmt::Display for ResourceAnalysisInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMetric => f.write_str("resource metric cannot be empty"),
            Self::EmptyUnit => f.write_str("resource unit cannot be empty"),
        }
    }
}

impl std::error::Error for ResourceAnalysisInputError {}

/// Reason a memory storage estimate could not be made exact.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryEstimateUncertainty {
    /// At least one memory dimension is symbolic or unresolved.
    SymbolicDimension,

    /// The element type is not a directly known-width logic type.
    UnknownElementWidth,

    /// Multiplying dimensions or width exceeded `u128`.
    ArithmeticOverflow,
}

/// Logical storage estimate for one declared memory.
///
/// These values describe the logical shape only. They do not represent a
/// target-specific RAM, SRAM, BRAM, register, or physical-cell allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryStorageEstimate {
    /// Stable HDLIR memory identity.
    pub memory_id: String,

    /// Exact logical element count, if every dimension is known.
    pub elements: Option<u128>,

    /// Exact element width in bits, if available directly from the type.
    pub element_bits: Option<u128>,

    /// Exact logical storage bits, if both factors are known and their
    /// product is representable.
    pub storage_bits: Option<u128>,

    /// Why an exact estimate was not available.
    ///
    /// `None` means the logical estimate is exact. An arithmetic overflow
    /// prevents a known total; it does not mean the memory is semantically
    /// invalid.
    pub uncertainty: Option<MemoryEstimateUncertainty>,
}

/// Resource-analysis finding category.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceAnalysisFindingKind {
    /// The logical memory storage product overflowed `u128`.
    MemoryEstimateOverflow {
        /// HDLIR memory identity.
        memory_id: String,
    },

    /// Multiple observations contribute to a metric/unit aggregate.
    ///
    /// An aggregate with symbolic or unavailable observations has only a
    /// known subtotal; it is not an exact total.
    ResourceAggregateIncomplete {
        /// Logical metric name.
        metric: String,

        /// Unit of the metric.
        unit: String,
    },

    /// Adding known quantities for one metric/unit overflowed `u128`.
    ResourceAggregateOverflow {
        /// Logical metric name.
        metric: String,

        /// Unit of the metric.
        unit: String,
    },
}

/// One resource-analysis finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceAnalysisFinding {
    /// Detected condition.
    pub kind: ResourceAnalysisFindingKind,
}

/// Why the analysis did not complete all requested work.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceAnalysisIncompleteReason {
    /// The configured memory budget was exhausted.
    MemoryBudgetExceeded {
        /// Configured limit.
        limit: u64,

        /// Number of memory records already inspected.
        inspected: u64,
    },

    /// The configured dimension budget was exhausted.
    DimensionBudgetExceeded {
        /// Configured limit.
        limit: u64,

        /// Number of dimensions already inspected.
        inspected: u64,
    },

    /// The configured fact budget was exhausted.
    FactBudgetExceeded {
        /// Configured limit.
        limit: u64,

        /// Number of resource facts already inspected.
        inspected: u64,
    },

    /// The configured finding-retention budget was exhausted.
    FindingBudgetExceeded {
        /// Configured limit.
        limit: u64,

        /// Number of findings already retained.
        retained: u64,
    },

    /// A work counter exceeded its representable range.
    CounterOverflow,
}

/// Whether all work requested by the configured budget was completed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResourceAnalysisCompletion {
    /// All requested analysis work completed.
    ///
    /// Findings can still indicate incomplete estimates or arithmetic
    /// overflow. Completion does not imply target feasibility.
    Complete,

    /// The analysis stopped before all requested work was completed.
    Incomplete(ResourceAnalysisIncompleteReason),
}

/// Aggregated observations for one exact metric/unit pair.
///
/// `known_subtotal` is the sum of known contributions seen so far.
/// It is an exact total only when `complete` is true and `overflow` is false.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceAggregate {
    /// Metric name supplied by the upstream adapter.
    pub metric: String,

    /// Unit supplied by the upstream adapter.
    pub unit: String,

    /// Sum of known quantities, if representable.
    pub known_subtotal: u128,

    /// Whether every observed contribution was concrete and representable.
    pub complete: bool,

    /// Whether adding known contributions overflowed `u128`.
    pub overflow: bool,

    /// Number of symbolic or unavailable contributions.
    pub unresolved_contributions: u64,

    /// Provenance strings for unresolved contributions, where supplied.
    pub unresolved_origins: Vec<String>,
}

/// Result of logical resource analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceAnalysisReport {
    /// Per-memory logical storage estimates.
    pub memory_estimates: Vec<MemoryStorageEstimate>,

    /// Aggregates keyed by the exact `(metric, unit)` pair.
    pub aggregates: BTreeMap<(String, String), ResourceAggregate>,

    /// Findings discovered before completion or budget exhaustion.
    pub findings: Vec<ResourceAnalysisFinding>,

    /// Completion status.
    pub completion: ResourceAnalysisCompletion,

    /// Number of memory declarations inspected.
    pub memories_examined: u64,

    /// Number of dimensions inspected.
    pub dimensions_examined: u64,

    /// Number of caller-supplied resource facts inspected.
    pub facts_examined: u64,

    /// Number of findings retained.
    pub findings_retained: u64,
}

impl Default for ResourceAnalysisReport {
    fn default() -> Self {
        Self {
            memory_estimates: Vec::new(),
            aggregates: BTreeMap::new(),
            findings: Vec::new(),
            completion: ResourceAnalysisCompletion::Complete,
            memories_examined: 0,
            dimensions_examined: 0,
            facts_examined: 0,
            findings_retained: 0,
        }
    }
}

impl ResourceAnalysisReport {
    /// Return whether all requested work completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completion, ResourceAnalysisCompletion::Complete)
    }

    /// Return whether any retained finding describes arithmetic overflow.
    #[must_use]
    pub fn has_arithmetic_overflow(&self) -> bool {
        self.findings.iter().any(|finding| {
            matches!(
                finding.kind,
                ResourceAnalysisFindingKind::MemoryEstimateOverflow { .. }
                    | ResourceAnalysisFindingKind::ResourceAggregateOverflow {
                        ..
                    }
            )
        })
    }
}

/// Analyze logical memory storage and normalized resource observations.
///
/// The memory table is the existing canonical HDLIR declaration collection.
/// `facts` must contain observations normalized by the caller from canonical
/// semantic resource contracts, registered dialect semantics, or another
/// authoritative adapter.
///
/// This function does not interpret requirement expressions, infer physical
/// capacities, allocate hardware, or decide whether a target is feasible.
///
/// # Determinism
///
/// Memory declarations are visited in ascending HDLIR memory-ID order.
/// Aggregate keys are ordered by metric and unit. The order of facts supplied
/// by the caller is preserved when processing contributions.
///
/// # Incomplete results
///
/// If a budget is exhausted, previously produced results are retained and
/// `completion` is set to `Incomplete`. Callers must not treat the report as
/// complete or use partial aggregates as exact totals.
#[must_use]
pub fn analyze_resources(
    memories: &HdlirMemoryTable,
    facts: &[ResourceFact],
    budget: ResourceAnalysisBudget,
) -> ResourceAnalysisReport {
    let mut report = ResourceAnalysisReport::default();

    for (memory_id, memory) in memories.iter_with_ids() {
        if let Err(reason) = tick(
            &mut report.memories_examined,
            budget.max_memories,
            |limit, inspected| {
                ResourceAnalysisIncompleteReason::MemoryBudgetExceeded {
                    limit,
                    inspected,
                }
            },
        ) {
            report.completion = ResourceAnalysisCompletion::Incomplete(reason);
            return report;
        }

        let memory_id_text = memory_id.to_string();
        let memory_type = memory.memory_type();

        let mut element_count = Some(1_u128);
        let mut uncertainty = None;

        for dimension in memory_type.dimensions() {
            if let Err(reason) = tick(
                &mut report.dimensions_examined,
                budget.max_dimensions,
                |limit, inspected| {
                    ResourceAnalysisIncompleteReason::DimensionBudgetExceeded {
                        limit,
                        inspected,
                    }
                },
            ) {
                report.completion = ResourceAnalysisCompletion::Incomplete(reason);
                return report;
            }

            match dimension {
                MemoryDepth::Known(extent) => {
                    if let Some(current) = element_count {
                        match current.checked_mul(extent.get()) {
                            Some(product) => element_count = Some(product),
                            None => {
                                element_count = None;
                                uncertainty = Some(
                                    MemoryEstimateUncertainty::ArithmeticOverflow,
                                );
                            }
                        }
                    }
                }
                MemoryDepth::Symbol(_)
                | MemoryDepth::Add(_, _)
                | MemoryDepth::Multiply(_, _)
                | MemoryDepth::Unresolved(_) => {
                    element_count = None;
                    if uncertainty.is_none() {
                        uncertainty = Some(
                            MemoryEstimateUncertainty::SymbolicDimension,
                        );
                    }
                }
            }
        }

        let element_bits = match memory_type.element_type() {
            SignalType::Logic(logic) => logic.width().as_known(),
            SignalType::Clock
            | SignalType::Reset
            | SignalType::Named(_) => None,
        };

        if element_bits.is_none() && uncertainty.is_none() {
            uncertainty = Some(MemoryEstimateUncertainty::UnknownElementWidth);
        }

        let storage_bits = match (element_count, element_bits) {
            (Some(elements), Some(bits)) => match elements.checked_mul(bits) {
                Some(total) => Some(total),
                None => {
                    uncertainty =
                        Some(MemoryEstimateUncertainty::ArithmeticOverflow);
                    None
                }
            },
            _ => None,
        };

        if matches!(
            uncertainty,
            Some(MemoryEstimateUncertainty::ArithmeticOverflow)
        ) {
            if !record_finding(
                &mut report,
                budget,
                ResourceAnalysisFindingKind::MemoryEstimateOverflow {
                    memory_id: memory_id_text.clone(),
                },
            ) {
                return report;
            }
        }

        report.memory_estimates.push(MemoryStorageEstimate {
            memory_id: memory_id_text,
            elements: element_count,
            element_bits,
            storage_bits,
            uncertainty,
        });
    }

    for fact in facts {
        if let Err(reason) = tick(
            &mut report.facts_examined,
            budget.max_facts,
            |limit, inspected| {
                ResourceAnalysisIncompleteReason::FactBudgetExceeded {
                    limit,
                    inspected,
                }
            },
        ) {
            report.completion = ResourceAnalysisCompletion::Incomplete(reason);
            return report;
        }

        // Defensive validation protects callers that constructed the public
        // struct directly instead of using ResourceFact::new.
        if fact.metric.trim().is_empty() || fact.unit.trim().is_empty() {
            // Invalid observations are not silently aggregated. Treat this as
            // incomplete because the current report schema has no input-error
            // channel distinct from findings.
            report.completion = ResourceAnalysisCompletion::Incomplete(
                ResourceAnalysisIncompleteReason::CounterOverflow,
            );
            return report;
        }

        let key = (fact.metric.clone(), fact.unit.clone());
        let aggregate = report
            .aggregates
            .entry(key)
            .or_insert_with(|| ResourceAggregate {
                metric: fact.metric.clone(),
                unit: fact.unit.clone(),
                known_subtotal: 0,
                complete: true,
                overflow: false,
                unresolved_contributions: 0,
                unresolved_origins: Vec::new(),
            });

        match &fact.quantity {
            ResourceQuantity::Known(value) => {
                if let Some(sum) = aggregate.known_subtotal.checked_add(*value) {
                    aggregate.known_subtotal = sum;
                } else {
                    aggregate.complete = false;
                    aggregate.overflow = true;

                    if !record_finding(
                        &mut report,
                        budget,
                        ResourceAnalysisFindingKind::ResourceAggregateOverflow {
                            metric: fact.metric.clone(),
                            unit: fact.unit.clone(),
                        },
                    ) {
                        return report;
                    }
                }
            }
            ResourceQuantity::Symbolic(_) | ResourceQuantity::Unavailable => {
                aggregate.complete = false;

                let Some(next) =
                    aggregate.unresolved_contributions.checked_add(1)
                else {
                    report.completion =
                        ResourceAnalysisCompletion::Incomplete(
                            ResourceAnalysisIncompleteReason::CounterOverflow,
                        );
                    return report;
                };

                aggregate.unresolved_contributions = next;

                if let Some(origin) = &fact.origin {
                    aggregate.unresolved_origins.push(origin.clone());
                }
            }
        }
    }

    // Add one finding per incomplete aggregate after aggregation. This keeps
    // the finding count independent of the number of symbolic contributions.
    let incomplete_keys: Vec<(String, String)> = report
        .aggregates
        .iter()
        .filter(|(_, aggregate)| !aggregate.complete && !aggregate.overflow)
        .map(|(key, _)| key.clone())
        .collect();

    for (metric, unit) in incomplete_keys {
        if !record_finding(
            &mut report,
            budget,
            ResourceAnalysisFindingKind::ResourceAggregateIncomplete {
                metric,
                unit,
            },
        ) {
            return report;
        }
    }

    report
}

fn tick(
    counter: &mut u64,
    limit: Option<u64>,
    exceeded: impl FnOnce(u64, u64) -> ResourceAnalysisIncompleteReason,
) -> Result<(), ResourceAnalysisIncompleteReason> {
    if let Some(limit) = limit {
        if *counter >= limit {
            return Err(exceeded(limit, *counter));
        }
    }

    *counter = counter
        .checked_add(1)
        .ok_or(ResourceAnalysisIncompleteReason::CounterOverflow)?;

    Ok(())
}

fn record_finding(
    report: &mut ResourceAnalysisReport,
    budget: ResourceAnalysisBudget,
    kind: ResourceAnalysisFindingKind,
) -> bool {
    if let Some(limit) = budget.max_findings {
        if report.findings_retained >= limit {
            report.completion =
                ResourceAnalysisCompletion::Incomplete(
                    ResourceAnalysisIncompleteReason::FindingBudgetExceeded {
                        limit,
                        retained: report.findings_retained,
                    },
                );
            return false;
        }
    }

    let Some(next) = report.findings_retained.checked_add(1) else {
        report.completion = ResourceAnalysisCompletion::Incomplete(
            ResourceAnalysisIncompleteReason::CounterOverflow,
        );
        return false;
    };

    report.findings.push(ResourceAnalysisFinding { kind });
    report.findings_retained = next;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_inputs_produce_a_complete_empty_report() {
        let memories = HdlirMemoryTable::new();
        let report = analyze_resources(
            &memories,
            &[],
            ResourceAnalysisBudget::unbounded(),
        );

        assert!(report.is_complete());
        assert!(report.memory_estimates.is_empty());
        assert!(report.aggregates.is_empty());
        assert!(report.findings.is_empty());
    }

    #[test]
    fn zero_memory_budget_does_not_fail_for_empty_input() {
        let memories = HdlirMemoryTable::new();
        let report = analyze_resources(
            &memories,
            &[],
            ResourceAnalysisBudget {
                max_memories: Some(0),
                ..ResourceAnalysisBudget::unbounded()
            },
        );

        assert!(report.is_complete());
    }

    #[test]
    fn resource_fact_requires_a_metric_and_unit() {
        assert_eq!(
            ResourceFact::new(
                " ",
                "bits",
                ResourceQuantity::Known(8),
                None,
            ),
            Err(ResourceAnalysisInputError::EmptyMetric),
        );

        assert_eq!(
            ResourceFact::new(
                "storage",
                " ",
                ResourceQuantity::Known(8),
                None,
            ),
            Err(ResourceAnalysisInputError::EmptyUnit),
        );
    }

    #[test]
    fn symbolic_observations_are_not_claimed_as_exact_totals() {
        let memories = HdlirMemoryTable::new();
        let facts = vec![
            ResourceFact::new(
                "logical.storage",
                "bits",
                ResourceQuantity::Known(64),
                Some("known contribution".to_owned()),
            )
            .expect("valid fact"),
            ResourceFact::new(
                "logical.storage",
                "bits",
                ResourceQuantity::Symbolic("DEPTH * WIDTH".to_owned()),
                Some("symbolic contribution".to_owned()),
            )
            .expect("valid fact"),
        ];

        let report = analyze_resources(
            &memories,
            &facts,
            ResourceAnalysisBudget::unbounded(),
        );

        assert!(report.is_complete());
        let aggregate = report
            .aggregates
            .get(&("logical.storage".to_owned(), "bits".to_owned()))
            .expect("aggregate exists");

        assert_eq!(aggregate.known_subtotal, 64);
        assert!(!aggregate.complete);
        assert_eq!(aggregate.unresolved_contributions, 1);
        assert!(report.findings.iter().any(|finding| matches!(
            finding.kind,
            ResourceAnalysisFindingKind::ResourceAggregateIncomplete { .. }
        )));
    }
}
