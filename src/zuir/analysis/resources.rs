#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Resource Analysis
//!
//! Canonical path: `src/zuir/analysis/resources.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! # PURPOSE
//!
//! Derive deterministic, immutable analysis facts from the canonical ZUIR
//! resource-intent model in `crate::zuir::ir::resources`.
//!
//! This module indexes resource assertions, capabilities, and topology
//! relationships. It does not allocate hardware, estimate physical execution
//! cost, discover target capabilities, or decide whether a target is feasible.
//!
//! # AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/resources.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/capabilities.g4`
//! - `grammar/resources/budgets.g4`
//! - `grammar/spec/resources.md`
//! - `grammar/specification/semantic-model.md`
//!
//! # OWNS
//!
//! - Immutable resource-analysis snapshots.
//! - Deterministic indexing of resource assertions by dimension.
//! - Indexed views of capability and topology declarations.
//! - Analysis-local errors and validation of input intents.
//!
//! # DOES NOT OWN
//!
//! - Resource-intent representation or resource-key semantics.
//! - Parsing, AST construction, or source-level name resolution.
//! - Type inference or symbolic expression evaluation.
//! - Contradiction verification across arbitrary symbolic expressions.
//! - Physical resource discovery, allocation, reservation, or release.
//! - Target feasibility, scheduling, placement, routing, or runtime behavior.
//! - Quantum-specific resource semantics or a second quantum IR.
//! - The authoritative whole-program verifier.
//!
//! # SEMANTICS
//!
//! Every input intent is validated before its declarations are indexed.
//! Declaration order is preserved in the fact collections. Dimension lookup
//! uses ordered maps, so dimension iteration is deterministic.
//!
//! Requirement and constraint declarations remain distinguishable from
//! budgets, preferences, and hints. This module never upgrades an advisory
//! declaration into a mandatory requirement.
//!
//! Duplicate declarations are preserved. Deduplication could erase provenance
//! or meaningful declaration occurrences and is therefore not performed.
//!
//! # SCALABILITY
//!
//! This module imposes no fixed maximum on resource dimensions, assertions,
//! capabilities, topology relationships, qubits, processors, devices, memory,
//! tensor dimensions, or distributed nodes.
//!
//! Collections grow dynamically. Fallible reservation is used for the main
//! fact vectors where practical. Standard ordered maps do not provide a
//! general stable fallible-reservation API, so complete recovery from every
//! allocator failure cannot be promised.
//!
//! No finite process can handle literal infinity. The implementation avoids
//! artificial universal machine-size limits; actual work is bounded by
//! available memory, address space, time, and configured process policies.
//!
//! # TRUST BOUNDARY
//!
//! Successful analysis means the supplied resource-intent values passed their
//! local validation contracts and were indexed. It does not prove that the
//! complete IR is valid, that assertions are mutually satisfiable, or that
//! any target can realize the requested computation.
//!
//! Deserialized or imported IR must pass canonical validation and the
//! authoritative verifier before optimization, lowering, or execution.
//!
//! # INVALIDATION
//!
//! This result is an immutable snapshot. Recompute it when any input intent
//! changes. Analysis-cache invalidation belongs to the pass manager.
//!
//! # GRAMMAR BOUNDARY
//!
//! The grammar defines source syntax for resource declarations and constraints.
//! Frontend and semantic adapters lower that syntax into the canonical
//! `ResourceIntent` types. This module does not parse source text.
//!
//! # FREEZE CRITERIA
//!
//! - The canonical resource model remains the only resource-intent owner.
//! - Invalid input intents return deterministic, structured errors.
//! - Duplicate declarations are preserved.
//! - All returned indexes point to the correct fact collection entries.
//! - No hardware capacity or resource dimension is hard-coded.
//! - Unit, integration, determinism, and stress tests pass.
//! - The module is declared by a compilable `analysis/mod.rs`.
//! - The full repository compiles and all relevant tests pass.

use std::collections::BTreeMap;
use std::fmt;

use crate::zuir::ir::resources::{
    CapabilityIntent,
    ResourceAssertion,
    ResourceIntent,
    ResourceIntentKind,
    ResourceKey,
    TopologyIntent,
};

/// Result type for resource analysis.
pub type ResourceAnalysisResult<T> = Result<T, ResourceAnalysisError>;

/// Errors returned while building a resource-analysis snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResourceAnalysisError {
    /// An input intent failed its canonical validation contract.
    InvalidIntent {
        /// Zero-based position of the input intent.
        intent_index: usize,
        /// Explanation from the canonical resource model.
        reason: String,
    },

    /// An input index exceeded the platform's index space.
    IndexExhausted,

    /// A collection could not reserve the requested additional capacity.
    AllocationFailure {
        /// Name of the collection whose reservation failed.
        collection: &'static str,
    },
}

impl ResourceAnalysisError {
    /// Return a stable machine-readable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidIntent { .. } => "ZUIR-RESOURCE-INVALID-INTENT",
            Self::IndexExhausted => "ZUIR-RESOURCE-INDEX-EXHAUSTED",
            Self::AllocationFailure { .. } => "ZUIR-RESOURCE-ALLOCATION-FAILURE",
        }
    }
}

impl fmt::Display for ResourceAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIntent {
                intent_index,
                reason,
            } => write!(
                f,
                "resource intent at input index {intent_index} is invalid: {reason}"
            ),
            Self::IndexExhausted => {
                f.write_str("resource analysis exceeded the platform index space")
            }
            Self::AllocationFailure { collection } => {
                write!(
                    f,
                    "unable to reserve memory for resource-analysis {collection}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceAnalysisError {}

/// One indexed resource assertion.
///
/// `intent_index` identifies the input intent's position in the iterator
/// supplied to [`ResourceAnalysis::analyze`]. It is not a persistent IR ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceAssertionFact {
    intent_index: usize,
    assertion: ResourceAssertion,
}

impl ResourceAssertionFact {
    /// Return the input intent's zero-based position.
    #[must_use]
    pub const fn intent_index(&self) -> usize {
        self.intent_index
    }

    /// Borrow the canonical assertion.
    #[must_use]
    pub fn assertion(&self) -> &ResourceAssertion {
        &self.assertion
    }
}

/// One indexed capability declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCapabilityFact {
    intent_index: usize,
    capability: CapabilityIntent,
}

impl ResourceCapabilityFact {
    /// Return the input intent's zero-based position.
    #[must_use]
    pub const fn intent_index(&self) -> usize {
        self.intent_index
    }

    /// Borrow the canonical capability declaration.
    #[must_use]
    pub fn capability(&self) -> &CapabilityIntent {
        &self.capability
    }

    /// Whether the declaration is mandatory under the canonical model.
    ///
    /// A mandatory declaration is not evidence that a target actually
    /// provides the capability.
    #[must_use]
    pub const fn is_mandatory(&self) -> bool {
        matches!(
            self.capability.kind(),
            ResourceIntentKind::Requirement | ResourceIntentKind::Constraint
        )
    }
}

/// One indexed topology declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceTopologyFact {
    intent_index: usize,
    topology: TopologyIntent,
}

impl ResourceTopologyFact {
    /// Return the input intent's zero-based position.
    #[must_use]
    pub const fn intent_index(&self) -> usize {
        self.intent_index
    }

    /// Borrow the canonical topology declaration.
    #[must_use]
    pub fn topology(&self) -> &TopologyIntent {
        &self.topology
    }

    /// Whether the declaration is mandatory under the canonical model.
    #[must_use]
    pub const fn is_mandatory(&self) -> bool {
        matches!(
            self.topology.kind(),
            ResourceIntentKind::Requirement | ResourceIntentKind::Constraint
        )
    }
}

/// Immutable resource-analysis snapshot.
///
/// Assertions are stored in input-intent order and declaration order.
/// `assertions_by_dimension` contains indexes into `assertions`, not duplicate
/// copies of assertion data.
///
/// Capability and topology facts likewise retain their source intent index.
/// These indexes are local to this snapshot and must not be serialized as
/// stable identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceAnalysis {
    input_intent_count: usize,
    assertions: Vec<ResourceAssertionFact>,
    assertions_by_dimension: BTreeMap<ResourceKey, Vec<usize>>,
    capabilities: Vec<ResourceCapabilityFact>,
    topology: Vec<ResourceTopologyFact>,
}

impl ResourceAnalysis {
    /// Analyze a sequence of canonical resource intents.
    ///
    /// Each input intent is validated before its declarations are copied into
    /// the snapshot. The iterator's order determines `intent_index`.
    ///
    /// This operation indexes declarations only. It does not infer resource
    /// quantities, convert units, solve symbolic constraints, or inspect
    /// physical hardware.
    pub fn analyze<'a, I>(intents: I) -> ResourceAnalysisResult<Self>
    where
        I: IntoIterator<Item = &'a ResourceIntent>,
    {
        let mut analysis = Self::default();

        for (intent_index, intent) in intents.into_iter().enumerate() {
            intent.validate().map_err(|error| {
                ResourceAnalysisError::InvalidIntent {
                    intent_index,
                    reason: error.to_string(),
                }
            })?;

            analysis
                .input_intent_count
                .checked_add(1)
                .ok_or(ResourceAnalysisError::IndexExhausted)?;

            // Reserve before growing the principal fact vectors. Map
            // allocations are governed by the standard allocator.
            analysis
                .assertions
                .try_reserve(intent.assertions().len())
                .map_err(|_| ResourceAnalysisError::AllocationFailure {
                    collection: "assertions",
                })?;

            analysis
                .capabilities
                .try_reserve(intent.capabilities().len())
                .map_err(|_| ResourceAnalysisError::AllocationFailure {
                    collection: "capabilities",
                })?;

            analysis
                .topology
                .try_reserve(intent.topology().len())
                .map_err(|_| ResourceAnalysisError::AllocationFailure {
                    collection: "topology facts",
                })?;

            for assertion in intent.assertions() {
                let fact_index = analysis.assertions.len();

                analysis
                    .assertions
                    .len()
                    .checked_add(1)
                    .ok_or(ResourceAnalysisError::IndexExhausted)?;

                analysis
                    .assertions_by_dimension
                    .entry(assertion.dimension().clone())
                    .or_default()
                    .push(fact_index);

                analysis.assertions.push(ResourceAssertionFact {
                    intent_index,
                    assertion: assertion.clone(),
                });
            }

            for capability in intent.capabilities() {
                analysis.capabilities.push(ResourceCapabilityFact {
                    intent_index,
                    capability: capability.clone(),
                });
            }

            for topology in intent.topology() {
                analysis.topology.push(ResourceTopologyFact {
                    intent_index,
                    topology: topology.clone(),
                });
            }

            analysis.input_intent_count = analysis
                .input_intent_count
                .checked_add(1)
                .ok_or(ResourceAnalysisError::IndexExhausted)?;
        }

        Ok(analysis)
    }

    /// Number of input intents analyzed.
    #[must_use]
    pub const fn input_intent_count(&self) -> usize {
        self.input_intent_count
    }

    /// Borrow all assertion facts in declaration order.
    #[must_use]
    pub fn assertions(&self) -> &[ResourceAssertionFact] {
        &self.assertions
    }

    /// Borrow all capability facts in declaration order.
    #[must_use]
    pub fn capabilities(&self) -> &[ResourceCapabilityFact] {
        &self.capabilities
    }

    /// Borrow all topology facts in declaration order.
    #[must_use]
    pub fn topology(&self) -> &[ResourceTopologyFact] {
        &self.topology
    }

    /// Return all assertion indexes for a resource dimension.
    ///
    /// The returned indexes refer to [`Self::assertions`]. They preserve the
    /// original order of declarations for that dimension.
    #[must_use]
    pub fn assertion_indexes_for(
        &self,
        dimension: &ResourceKey,
    ) -> &[usize] {
        self.assertions_by_dimension
            .get(dimension)
            .map_or(&[], Vec::as_slice)
    }

    /// Iterate over assertion facts for a resource dimension.
    #[must_use]
    pub fn assertions_for<'a>(
        &'a self,
        dimension: &'a ResourceKey,
    ) -> impl Iterator<Item = &'a ResourceAssertionFact> + 'a {
        self.assertion_indexes_for(dimension)
            .iter()
            .filter_map(|&index| self.assertions.get(index))
    }

    /// Borrow the ordered dimension index.
    ///
    /// Each vector contains indexes into [`Self::assertions`].
    #[must_use]
    pub fn dimension_index(&self) -> &BTreeMap<ResourceKey, Vec<usize>> {
        &self.assertions_by_dimension
    }

    /// Return whether any resource declarations were found.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assertions.is_empty()
            && self.capabilities.is_empty()
            && self.topology.is_empty()
    }

    /// Count mandatory resource assertions.
    #[must_use]
    pub fn mandatory_assertion_count(&self) -> usize {
        self.assertions
            .iter()
            .filter(|fact| {
                matches!(
                    fact.assertion.kind(),
                    ResourceIntentKind::Requirement
                        | ResourceIntentKind::Constraint
                )
            })
            .count()
    }

    /// Count budget assertions.
    #[must_use]
    pub fn budget_assertion_count(&self) -> usize {
        self.assertions
            .iter()
            .filter(|fact| {
                fact.assertion.kind() == ResourceIntentKind::Budget
            })
            .count()
    }

    /// Count advisory assertions (preferences and hints).
    #[must_use]
    pub fn advisory_assertion_count(&self) -> usize {
        self.assertions
            .iter()
            .filter(|fact| {
                matches!(
                    fact.assertion.kind(),
                    ResourceIntentKind::Preference | ResourceIntentKind::Hint
                )
            })
            .count()
    }

    /// Iterate over mandatory capability declarations.
    ///
    /// This does not query a target capability registry.
    pub fn mandatory_capabilities(
        &self,
    ) -> impl Iterator<Item = &ResourceCapabilityFact> {
        self.capabilities
            .iter()
            .filter(|fact| fact.is_mandatory())
    }

    /// Revalidate the internal index relationships.
    ///
    /// This is useful at trust boundaries after receiving an analysis
    /// snapshot through an internal cache or future interchange adapter.
    /// It does not revalidate the original IR or prove target feasibility.
    pub fn validate(&self) -> ResourceAnalysisResult<()> {
        for (dimension, indexes) in &self.assertions_by_dimension {
            for &index in indexes {
                let fact = self.assertions.get(index).ok_or_else(|| {
                    ResourceAnalysisError::InvalidIntent {
                        intent_index: 0,
                        reason: format!(
                            "dimension index `{dimension}` references missing assertion {index}"
                        ),
                    }
                })?;

                if fact.assertion.dimension() != dimension {
                    return Err(ResourceAnalysisError::InvalidIntent {
                        intent_index: fact.intent_index,
                        reason: format!(
                            "dimension index `{dimension}` references an assertion for `{}`",
                            fact.assertion.dimension()
                        ),
                    });
                }
            }
        }

        // Every assertion must be indexed exactly once under its dimension.
        for (index, fact) in self.assertions.iter().enumerate() {
            let occurrences = self
                .assertion_indexes_for(fact.assertion.dimension())
                .iter()
                .filter(|&&candidate| candidate == index)
                .count();

            if occurrences != 1 {
                return Err(ResourceAnalysisError::InvalidIntent {
                    intent_index: fact.intent_index,
                    reason: format!(
                        "assertion index {index} occurs {occurrences} times in its dimension index"
                    ),
                });
            }
        }

        if self
            .assertions_by_dimension
            .values()
            .any(Vec::is_empty)
        {
            return Err(ResourceAnalysisError::InvalidIntent {
                intent_index: 0,
                reason: "dimension index contains an empty entry".to_owned(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ir::resources::{
        DecimalMagnitude,
        ResourceQuantity,
        ResourceRelation,
    };

    fn assertion(
        dimension: &str,
        relation: ResourceRelation,
        magnitude: &str,
        unit: &str,
        kind: ResourceIntentKind,
    ) -> ResourceAssertion {
        ResourceAssertion::new(
            ResourceKey::new(dimension).expect("valid dimension"),
            relation,
            ResourceQuantity::new(
                DecimalMagnitude::parse(magnitude).expect("valid magnitude"),
                ResourceKey::new(unit).expect("valid unit"),
            )
            .expect("valid quantity"),
            kind,
        )
        .expect("valid assertion")
    }

    #[test]
    fn empty_input_produces_empty_valid_analysis() {
        let analysis = ResourceAnalysis::analyze(
            std::iter::empty::<&ResourceIntent>(),
        )
        .expect("empty analysis");

        assert_eq!(analysis.input_intent_count(), 0);
        assert!(analysis.is_empty());
        assert!(analysis.validate().is_ok());
    }

    #[test]
    fn indexes_assertions_by_dimension_and_preserves_order() {
        let mut first = ResourceIntent::new();
        first
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "12",
                "GiB",
                ResourceIntentKind::Requirement,
            ))
            .expect("add first assertion");

        let mut second = ResourceIntent::new();
        second
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "24",
                "GiB",
                ResourceIntentKind::Budget,
            ))
            .expect("add second assertion");

        let intents = [&first, &second];
        let analysis =
            ResourceAnalysis::analyze(intents).expect("resource analysis");

        let dimension =
            ResourceKey::new("memory.capacity").expect("valid dimension");
        let facts: Vec<_> = analysis.assertions_for(&dimension).collect();

        assert_eq!(analysis.input_intent_count(), 2);
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].intent_index(), 0);
        assert_eq!(facts[1].intent_index(), 1);
        assert_eq!(analysis.mandatory_assertion_count(), 1);
        assert_eq!(analysis.budget_assertion_count(), 1);
        assert!(analysis.validate().is_ok());
    }

    #[test]
    fn advisory_assertions_are_not_promoted_to_requirements() {
        let mut intent = ResourceIntent::new();
        intent
            .add_assertion(assertion(
                "processing.units",
                ResourceRelation::AtLeast,
                "4",
                "units",
                ResourceIntentKind::Preference,
            ))
            .expect("add preference");

        let analysis =
            ResourceAnalysis::analyze([&intent]).expect("resource analysis");

        assert_eq!(analysis.mandatory_assertion_count(), 0);
        assert_eq!(analysis.advisory_assertion_count(), 1);
    }

    #[test]
    fn repeated_declarations_are_preserved() {
        let mut intent = ResourceIntent::new();

        for _ in 0..2 {
            intent
                .add_assertion(assertion(
                    "quantum.logical_qubits",
                    ResourceRelation::AtLeast,
                    "1",
                    "qubits",
                    ResourceIntentKind::Requirement,
                ))
                .expect("add assertion");
        }

        let analysis =
            ResourceAnalysis::analyze([&intent]).expect("resource analysis");

        let dimension = ResourceKey::new("quantum.logical_qubits")
            .expect("valid dimension");

        assert_eq!(analysis.assertions_for(&dimension).count(), 2);
        assert!(analysis.validate().is_ok());
    }

    #[test]
    fn results_are_deterministic_for_the_same_input() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "network.bandwidth",
                ResourceRelation::AtLeast,
                "10",
                "Gbps",
                ResourceIntentKind::Constraint,
            ))
            .expect("add bandwidth");

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "8",
                "GiB",
                ResourceIntentKind::Requirement,
            ))
            .expect("add memory");

        let first =
            ResourceAnalysis::analyze([&intent]).expect("first analysis");
        let second =
            ResourceAnalysis::analyze([&intent]).expect("second analysis");

        assert_eq!(first, second);
        assert!(first.validate().is_ok());
    }
}