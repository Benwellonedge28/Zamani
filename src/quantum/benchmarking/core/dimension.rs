//! Universal dimensions for Zamani quantum benchmarks.
//!
//! This module defines the stable vocabulary used to describe *what is being
//! varied or measured* by a benchmark. It deliberately does not define metric
//! values, units, statistical uncertainty, or protocol-specific semantics;
//! those belong to `core::metric`, `core::experiment`, and the individual
//! benchmark protocols.
//!
//! # Architectural role
//!
//! ```text
//! benchmark configuration / workload / result
//!                 │
//!                 ▼
//!       BenchmarkDimension / DimensionSet
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!    volumetric  metrics  analysis
//! ```
//!
//! The type is intentionally backend-neutral. In particular, `Qubits` does
//! not imply a physical-qubit implementation, and `Time` does not imply a
//! particular clock or timing source. Backend-specific meaning is attached by
//! the owning hardware/execution layer and recorded in provenance.
//!
//! # Stability contract
//!
//! Built-in dimensions have stable machine identifiers. Do not rename those
//! identifiers after publication; serialized benchmark reports and regression
//! baselines depend on them. New dimensions should normally be added as a new
//! built-in variant only when the concept is universal enough to deserve a
//! stable Zamani-wide identifier. Otherwise use `Custom`.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021. No nightly features are required.

use core::fmt;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::str::FromStr;

#[cfg(feature = "full")]
use serde::{Deserialize, Serialize};

/// Maximum UTF-8 byte length of a custom dimension identifier.
///
/// A bounded identifier prevents malformed benchmark metadata from creating
/// unbounded allocations while still allowing useful domain-specific names.
pub const MAX_CUSTOM_DIMENSION_ID_BYTES: usize = 128;

/// Universal benchmarking dimensions supported by Zamani.
///
/// These are *axes/concepts*, not numerical values. For example, a benchmark
/// may contain `Depth` as an axis and later associate values such as 10, 20,
/// and 40 with that axis. Units and uncertainty belong to the metric/value
/// layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum BenchmarkDimension {
    /// Total logical or physical qubit count represented by the workload.
    Qubits,

    /// Number of logical qubits used by a logical/fault-tolerant workload.
    LogicalQubits,

    /// Number of physical qubits allocated or required by the workload.
    PhysicalQubits,

    /// Total circuit depth according to the canonical circuit-depth
    /// definition used by the workload/IR.
    Depth,

    /// Depth contributed by two-qubit operations.
    TwoQubitDepth,

    /// Total quantum gate/operation count.
    GateCount,

    /// Number of two-qubit gates/operations.
    TwoQubitGateCount,

    /// Number of benchmark shots/samples requested or observed.
    Shots,

    /// Number of distinct circuits/instances executed.
    Circuits,

    /// Elapsed time associated with an explicitly identified timing stage.
    ///
    /// The timing source and stage must be supplied by the execution layer.
    Time,

    /// Memory/storage consumption associated with a benchmark stage.
    ///
    /// The concrete unit belongs to the metric layer.
    Memory,

    /// Energy consumption when the backend can measure it.
    Energy,

    /// Fidelity or fidelity-like quality measure.
    Fidelity,

    /// Error probability/rate or error-like quality measure.
    ErrorRate,

    /// Rate of completed work; the exact numerator/denominator belongs to the
    /// metric definition.
    Throughput,

    /// Latency of a specified operation or pipeline stage.
    Latency,

    /// Domain-specific extension that is not yet part of the stable built-in
    /// vocabulary.
    Custom(String),
}

impl BenchmarkDimension {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Qubits => "qubits",
            Self::LogicalQubits => "logical_qubits",
            Self::PhysicalQubits => "physical_qubits",
            Self::Depth => "depth",
            Self::TwoQubitDepth => "two_qubit_depth",
            Self::GateCount => "gate_count",
            Self::TwoQubitGateCount => "two_qubit_gate_count",
            Self::Shots => "shots",
            Self::Circuits => "circuits",
            Self::Time => "time",
            Self::Memory => "memory",
            Self::Energy => "energy",
            Self::Fidelity => "fidelity",
            Self::ErrorRate => "error_rate",
            Self::Throughput => "throughput",
            Self::Latency => "latency",
            Self::Custom(id) => id.as_str(),
        }
    }

    /// Returns the human-readable name used in reports and diagnostics.
    #[must_use]
    pub fn display_name(&self) -> &str {
        match self {
            Self::Qubits => "Qubits",
            Self::LogicalQubits => "Logical Qubits",
            Self::PhysicalQubits => "Physical Qubits",
            Self::Depth => "Depth",
            Self::TwoQubitDepth => "Two-Qubit Depth",
            Self::GateCount => "Gate Count",
            Self::TwoQubitGateCount => "Two-Qubit Gate Count",
            Self::Shots => "Shots",
            Self::Circuits => "Circuits",
            Self::Time => "Time",
            Self::Memory => "Memory",
            Self::Energy => "Energy",
            Self::Fidelity => "Fidelity",
            Self::ErrorRate => "Error Rate",
            Self::Throughput => "Throughput",
            Self::Latency => "Latency",
            Self::Custom(id) => id.as_str(),
        }
    }

    /// Returns whether this is a user-defined extension dimension.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Creates a validated custom dimension.
    ///
    /// Custom identifiers use the same canonical form as built-in IDs:
    /// lowercase ASCII letters, digits, and underscores, beginning with a
    /// letter. This makes them safe for JSON keys, CLI output, reports, and
    /// future Zamani-language syntax without introducing escaping rules.
    pub fn custom<S: Into<String>>(id: S) -> Result<Self, DimensionError> {
        let id = id.into();
        validate_dimension_id(&id)?;
        Ok(Self::Custom(id))
    }

    /// Returns all stable built-in dimensions.
    #[must_use]
    pub fn builtins() -> &'static [Self] {
        static BUILTINS: [BenchmarkDimension; 16] = [
            BenchmarkDimension::Qubits,
            BenchmarkDimension::LogicalQubits,
            BenchmarkDimension::PhysicalQubits,
            BenchmarkDimension::Depth,
            BenchmarkDimension::TwoQubitDepth,
            BenchmarkDimension::GateCount,
            BenchmarkDimension::TwoQubitGateCount,
            BenchmarkDimension::Shots,
            BenchmarkDimension::Circuits,
            BenchmarkDimension::Time,
            BenchmarkDimension::Memory,
            BenchmarkDimension::Energy,
            BenchmarkDimension::Fidelity,
            BenchmarkDimension::ErrorRate,
            BenchmarkDimension::Throughput,
            BenchmarkDimension::Latency,
        ];

        &BUILTINS
    }

    /// Parses a stable built-in identifier or a validated custom identifier.
    ///
    /// Unknown but valid identifiers are deliberately preserved as `Custom`
    /// rather than rejected. This is required for forward-compatible
    /// benchmark result files: an older Zamani binary must be able to read a
    /// newer benchmark's domain-specific dimensions without silently
    /// dropping them.
    pub fn parse(id: &str) -> Result<Self, DimensionError> {
        validate_dimension_id(id)?;

        Ok(match id {
            "qubits" => Self::Qubits,
            "logical_qubits" => Self::LogicalQubits,
            "physical_qubits" => Self::PhysicalQubits,
            "depth" => Self::Depth,
            "two_qubit_depth" => Self::TwoQubitDepth,
            "gate_count" => Self::GateCount,
            "two_qubit_gate_count" => Self::TwoQubitGateCount,
            "shots" => Self::Shots,
            "circuits" => Self::Circuits,
            "time" => Self::Time,
            "memory" => Self::Memory,
            "energy" => Self::Energy,
            "fidelity" => Self::Fidelity,
            "error_rate" => Self::ErrorRate,
            "throughput" => Self::Throughput,
            "latency" => Self::Latency,
            other => Self::Custom(other.to_owned()),
        })
    }
}

impl fmt::Display for BenchmarkDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

impl FromStr for BenchmarkDimension {
    type Err = DimensionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for BenchmarkDimension {
    type Error = DimensionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for BenchmarkDimension {
    type Error = DimensionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

/// A deterministic, duplicate-free set of benchmark dimensions.
///
/// `BTreeSet` gives stable ordering independent of insertion order. This is
/// important for reproducibility hashes, serialized benchmark specifications,
/// and regression comparisons.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DimensionSet {
    dimensions: BTreeSet<BenchmarkDimension>,
}

impl DimensionSet {
    /// Creates an empty dimension set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a set from an iterator, deduplicating dimensions.
    #[must_use]
    pub fn from_iter<I>(dimensions: I) -> Self
    where
        I: IntoIterator<Item = BenchmarkDimension>,
    {
        Self {
            dimensions: dimensions.into_iter().collect(),
        }
    }

    /// Inserts a dimension and reports whether it was newly inserted.
    pub fn insert(&mut self, dimension: BenchmarkDimension) -> bool {
        self.dimensions.insert(dimension)
    }

    /// Returns whether the set contains a dimension.
    #[must_use]
    pub fn contains(&self, dimension: &BenchmarkDimension) -> bool {
        self.dimensions.contains(dimension)
    }

    /// Returns the number of distinct dimensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dimensions.is_empty()
    }

    /// Returns dimensions in deterministic canonical order.
    pub fn iter(&self) -> impl Iterator<Item = &BenchmarkDimension> {
        self.dimensions.iter()
    }

    /// Returns a deterministic vector suitable for serialization or hashing.
    #[must_use]
    pub fn to_vec(&self) -> Vec<BenchmarkDimension> {
        self.dimensions.iter().cloned().collect()
    }

    /// Returns stable machine identifiers in deterministic order.
    #[must_use]
    pub fn ids(&self) -> Vec<&str> {
        self.dimensions.iter().map(BenchmarkDimension::id).collect()
    }
}

impl IntoIterator for DimensionSet {
    type Item = BenchmarkDimension;
    type IntoIter = std::collections::btree_set::IntoIter<BenchmarkDimension>;

    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

/// Errors raised when constructing or parsing a benchmark dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionError {
    /// The identifier was empty.
    EmptyIdentifier,

    /// The identifier exceeded the bounded metadata size.
    IdentifierTooLong {
        /// Length in UTF-8 bytes.
        length: usize,

        /// Maximum accepted UTF-8 byte length.
        maximum: usize,
    },

    /// The first character was not a lowercase ASCII letter.
    InvalidFirstCharacter,

    /// A later character was outside `[a-z0-9_]`.
    InvalidCharacter {
        /// Zero-based byte position of the offending ASCII byte.
        position: usize,

        /// Offending byte.
        byte: u8,
    },
}

impl fmt::Display for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                f.write_str("benchmark dimension identifier cannot be empty")
            }

            Self::IdentifierTooLong { length, maximum } => write!(
                f,
                "benchmark dimension identifier is {} bytes; maximum is {} bytes",
                length, maximum
            ),

            Self::InvalidFirstCharacter => f.write_str(
                "benchmark dimension identifier must begin with a lowercase ASCII letter",
            ),

            Self::InvalidCharacter { position, byte } => write!(
                f,
                "invalid byte 0x{:02x} at position {} in benchmark dimension identifier",
                byte, position
            ),
        }
    }
}

impl std::error::Error for DimensionError {}

fn validate_dimension_id(id: &str) -> Result<(), DimensionError> {
    if id.is_empty() {
        return Err(DimensionError::EmptyIdentifier);
    }

    if id.len() > MAX_CUSTOM_DIMENSION_ID_BYTES {
        return Err(DimensionError::IdentifierTooLong {
            length: id.len(),
            maximum: MAX_CUSTOM_DIMENSION_ID_BYTES,
        });
    }

    let bytes = id.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(DimensionError::InvalidFirstCharacter);
    }

    for (position, &byte) in bytes.iter().enumerate().skip(1) {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_')
        {
            return Err(DimensionError::InvalidCharacter { position, byte });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_ids_are_stable() {
        let expected = [
            "qubits",
            "logical_qubits",
            "physical_qubits",
            "depth",
            "two_qubit_depth",
            "gate_count",
            "two_qubit_gate_count",
            "shots",
            "circuits",
            "time",
            "memory",
            "energy",
            "fidelity",
            "error_rate",
            "throughput",
            "latency",
        ];

        let actual: Vec<&str> = BenchmarkDimension::builtins()
            .iter()
            .map(BenchmarkDimension::id)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn built_in_round_trip_is_lossless() {
        for dimension in BenchmarkDimension::builtins() {
            let parsed = BenchmarkDimension::parse(dimension.id()).unwrap();

            assert_eq!(&parsed, dimension);
            assert_eq!(parsed.to_string(), dimension.id());
        }
    }

    #[test]
    fn unknown_valid_ids_are_forward_compatible() {
        let dimension =
            BenchmarkDimension::parse("decoder_latency_p99").unwrap();

        assert_eq!(
            dimension,
            BenchmarkDimension::Custom(
                "decoder_latency_p99".into()
            )
        );

        assert!(dimension.is_custom());
        assert_eq!(dimension.id(), "decoder_latency_p99");
    }

    #[test]
    fn custom_dimension_validation_is_strict_and_bounded() {
        assert!(BenchmarkDimension::custom("logical_cycle_count").is_ok());

        assert_eq!(
            BenchmarkDimension::custom("logical_cycle_count")
                .unwrap()
                .id(),
            "logical_cycle_count"
        );

        assert_eq!(
            BenchmarkDimension::custom(""),
            Err(DimensionError::EmptyIdentifier)
        );

        assert_eq!(
            BenchmarkDimension::custom("1qubit"),
            Err(DimensionError::InvalidFirstCharacter)
        );

        assert!(matches!(
            BenchmarkDimension::custom("Qubits"),
            Err(DimensionError::InvalidFirstCharacter)
        ));

        assert!(matches!(
            BenchmarkDimension::custom("two-qubit-depth"),
            Err(DimensionError::InvalidCharacter { .. })
        ));

        assert!(matches!(
            BenchmarkDimension::custom("two.qubit.depth"),
            Err(DimensionError::InvalidCharacter { .. })
        ));

        let too_long =
            "a".repeat(MAX_CUSTOM_DIMENSION_ID_BYTES + 1);

        assert!(matches!(
            BenchmarkDimension::custom(too_long),
            Err(DimensionError::IdentifierTooLong { .. })
        ));
    }

    #[test]
    fn dimension_set_is_deterministic_and_deduplicated() {
        let mut set = DimensionSet::new();

        assert!(set.insert(BenchmarkDimension::Depth));
        assert!(!set.insert(BenchmarkDimension::Depth));
        assert!(set.insert(BenchmarkDimension::Qubits));

        assert!(set.insert(
            BenchmarkDimension::Custom("problem_size".into())
        ));

        assert_eq!(set.len(), 3);

        assert!(set.contains(&BenchmarkDimension::Depth));
        assert!(!set.contains(&BenchmarkDimension::Shots));

        assert_eq!(
            set.ids(),
            vec!["depth", "problem_size", "qubits"]
        );
    }

    #[test]
    fn dimension_set_from_iter_deduplicates() {
        let set = DimensionSet::from_iter([
            BenchmarkDimension::Shots,
            BenchmarkDimension::Shots,
            BenchmarkDimension::Circuits,
        ]);

        assert_eq!(set.len(), 2);
        assert_eq!(set.ids(), vec!["circuits", "shots"]);
    }

    #[test]
    fn identifiers_are_ascii_and_machine_safe() {
        assert!(BenchmarkDimension::parse("qec_rounds_10").is_ok());
        assert!(BenchmarkDimension::parse("QEC_ROUNDS").is_err());
        assert!(BenchmarkDimension::parse("qec-rounds").is_err());
        assert!(BenchmarkDimension::parse("qec rounds").is_err());
        assert!(BenchmarkDimension::parse("qéc_rounds").is_err());
    }
}