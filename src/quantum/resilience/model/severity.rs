//! Zamani Quantum Resilience — Severity Model.
//!
//! This module defines the canonical, backend-independent severity vocabulary
//! used by the quantum resilience subsystem.
//!
//! # Architectural role
//!
//! Severity answers:
//!
//! > "How serious is this resilience-domain condition in the current
//! > execution context?"
//!
//! It does NOT answer:
//!
//! > "What recovery action should be performed?"
//!
//! Recovery decisions belong to:
//!
//! ```text
//! quantum::resilience::policy
//! quantum::resilience::planning
//! quantum::resilience::recovery
//! ```
//!
//! Severity is therefore an input to policy and planning, not an implicit
//! command.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the stable resilience severity vocabulary;
//! - severity comparison;
//! - deterministic severity ordering;
//! - human-readable representation;
//! - machine-readable representation;
//! - conservative severity composition;
//! - conversion to/from stable numeric ranks;
//! - severity predicates;
//! - serialization-independent inspection APIs.
//!
//! This module does NOT own:
//!
//! - quantum fault semantics;
//! - noise models;
//! - QEC semantics;
//! - hardware health;
//! - retry policy;
//! - recovery actions;
//! - resource limits;
//! - confidence;
//! - timestamps;
//! - telemetry;
//! - authorization;
//! - backend/provider identities;
//! - machine-size limits.
//!
//! # Relationship to other resilience models
//!
//! ```text
//! model/fault.rs
//!       │
//!       │ observed fact
//!       ▼
//! model/severity.rs
//!       │
//!       │ resilience interpretation
//!       ├──────────────► diagnosis
//!       ├──────────────► policy
//!       ├──────────────► planning
//!       └──────────────► observability
//! ```
//!
//! Severity is intentionally separate from:
//!
//! - [`crate::quantum::resilience::model::confidence`] because uncertainty and
//!   seriousness are different dimensions;
//! - `model::health` because resource health and incident severity are not
//!   equivalent;
//! - `model::fault` because a fault is an observed/represented condition while
//!   severity is an interpretation of its operational consequence.
//!
//! # Critical semantic distinction
//!
//! Severity MUST NOT be interpreted as:
//!
//! - probability;
//! - confidence;
//! - priority;
//! - retry count;
//! - recovery cost;
//! - resource quantity;
//! - execution deadline;
//! - logical error rate;
//! - physical error rate;
//! - hardware fidelity;
//! - authorization level.
//!
//! For example:
//!
//! ```text
//! Critical + low confidence
//! ```
//!
//! is different from:
//!
//! ```text
//! Critical + high confidence
//! ```
//!
//! Diagnosis/policy must consider both dimensions.
//!
//! # Stable vocabulary
//!
//! The canonical severity levels are:
//!
//! ```text
//! Informational
//! Degraded
//! Major
//! Critical
//! Fatal
//! ```
//!
//! The ordering is intentionally semantic:
//!
//! ```text
//! Informational < Degraded < Major < Critical < Fatal
//! ```
//!
//! This ordering means "greater operational consequence", not "greater
//! probability" and not "greater execution priority."
//!
//! # Why no `Unknown` severity?
//!
//! Unknown is not a severity.
//!
//! Unknown information should be represented by the absence of a valid
//! severity, or by a separate uncertainty/confidence model.
//!
//! Introducing an `Unknown` severity value would make it ambiguous whether
//! "unknown" means:
//!
//! - missing observation;
//! - insufficient confidence;
//! - genuinely unknown impact;
//! - invalid data;
//! - lowest severity.
//!
//! Those are materially different states.
//!
//! `Option<Severity>` can represent absence without corrupting the severity
//! ordering.
//!
//! # Why no `Emergency`, `Warning`, `Error`, etc.?
//!
//! The vocabulary deliberately remains small and stable.
//!
//! Additional dimensions such as:
//!
//! - alert priority;
//! - operational urgency;
//! - incident class;
//! - security criticality;
//! - safety criticality;
//! - confidence;
//! - recoverability;
//!
//! belong to their respective domain models.
//!
//! This prevents severity from becoming a catch-all enum that must change
//! whenever another subsystem evolves.
//!
//! # Write once, scale everywhere
//!
//! This module contains no machine-specific values.
//!
//! It has no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_BACKENDS
//! MAX_RETRIES
//! MAX_OPERATIONS
//! ```
//!
//! Severity is independent of system size.
//!
//! A single-qubit failure and a distributed failure affecting an arbitrarily
//! large resource set can both be represented by the same severity vocabulary.
//!
//! The number of affected resources is NOT encoded in the severity itself.
//!
//! For example:
//!
//! ```text
//! one qubit  -> Critical
//! many qubits -> Critical
//! entire backend -> Critical
//! ```
//!
//! The difference belongs to fault/incident scope, health, resource and
//! diagnosis models.
//!
//! # Determinism
//!
//! This type is a pure value.
//!
//! It does not:
//!
//! - access the clock;
//! - generate randomness;
//! - inspect hardware;
//! - access global state;
//! - access environment variables;
//! - perform I/O;
//! - query a backend;
//! - inspect thread-local state.
//!
//! Given the same severity value, all methods return the same result.
//!
//! # Security
//!
//! Severity is descriptive data, not authority.
//!
//! A severity value MUST NOT grant:
//!
//! - hardware control;
//! - QPU access;
//! - credentials;
//! - recovery authorization;
//! - administrative privileges.
//!
//! A compromised component must not be able to obtain authority merely by
//! producing `Fatal`.
//!
//! Authorization is handled elsewhere.
//!
//! # Policy integration
//!
//! Policy may use severity as one input:
//!
//! ```text
//! diagnosis
//!    │
//!    ├── severity
//!    ├── confidence
//!    ├── scope
//!    ├── capabilities
//!    ├── constraints
//!    └── history
//!          │
//!          ▼
//!       policy
//!          │
//!          ▼
//!       plan
//! ```
//!
//! Severity alone must never select a recovery action.
//!
//! # Ordering semantics
//!
//! `Ord` is provided so deterministic consumers can:
//!
//! - sort incidents;
//! - select the most severe observed condition;
//! - compare policy thresholds;
//! - build deterministic reports;
//! - aggregate observations.
//!
//! Ordering MUST NOT be interpreted as:
//!
//! - temporal ordering;
//! - causal ordering;
//! - recovery priority;
//! - execution priority.
//!
//! Those semantics belong elsewhere.
//!
//! # Aggregation
//!
//! `max()` semantics are useful when aggregating independent observations into
//! a conservative incident severity:
//!
//! ```text
//! Informational + Major + Degraded = Major
//! ```
//!
//! This does NOT imply that every incident must use maximum severity.
//!
//! More sophisticated incident aggregation may consider:
//!
//! - scope;
//! - confidence;
//! - correlation;
//! - affected logical resources;
//! - redundancy;
//! - policy;
//! - temporal persistence.
//!
//! Such aggregation belongs to `model::incident` or diagnosis.
//!
//! # Numeric representation
//!
//! Numeric ranks are intended only for:
//!
//! - deterministic ordering;
//! - compact internal representations;
//! - schema adapters;
//! - interoperability boundaries.
//!
//! They are NOT:
//!
//! - probabilities;
//! - percentages;
//! - error rates;
//! - externally imposed universal severity standards.
//!
//! The mapping is stable within the Zamani resilience semantic contract:
//!
//! ```text
//! Informational = 0
//! Degraded      = 1
//! Major         = 2
//! Critical      = 3
//! Fatal         = 4
//! ```
//!
//! Callers MUST NOT infer quantitative meaning from the difference between
//! ranks.
//!
//! # Serialization
//!
//! This module does not implement a wire format.
//!
//! The resilience serialization layer owns schema/version handling.
//!
//! `as_str()` and `from_str()` provide a stable semantic representation that
//! serialization adapters may use.
//!
//! Serialization code should prefer the stable string representation when
//! compatibility across versions is more important than compactness.
//!
//! Numeric representation may be used where the surrounding schema explicitly
//! defines it.
//!
//! # Forward compatibility
//!
//! Exhaustive matching on `Severity` is intentional inside this module.
//!
//! External serialized data must be validated by the serialization layer
//! before conversion to this enum.
//!
//! Unknown future wire values must not be silently mapped to an existing
//! severity because doing so could lower or otherwise alter safety-relevant
//! meaning.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This module intentionally has no dependency on higher-level resilience
//! modules.
//!
//! Consumers:
//!
//! ```text
//! model/fault.rs
//! model/incident.rs
//! model/health.rs
//! model/degradation.rs
//!
//!        │
//!        ▼
//! model/severity.rs
//!        │
//!        ├── diagnosis
//!        ├── policy
//!        ├── planning
//!        ├── telemetry
//!        ├── history
//!        └── verification
//! ```
//!
//! The dependency direction must remain downward toward this foundational
//! model, never upward from this file into policy/recovery.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. severity has a stable vocabulary;
//! 2. severity is independent of hardware/provider identity;
//! 3. severity is independent of machine size;
//! 4. severity is independent of confidence;
//! 5. severity is independent of recovery action;
//! 6. deterministic ordering is defined;
//! 7. deterministic aggregation is available;
//! 8. stable textual representation is available;
//! 9. no serialization framework is required;
//! 10. no global state exists;
//! 11. no clock or RNG is accessed;
//! 12. no unsafe Rust exists;
//! 13. no hard-coded machine resource limits exist;
//! 14. the module can be consumed by later resilience modules without changing
//!     its semantic foundation.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::str::FromStr;

/// Canonical resilience severity.
///
/// Severity represents the operational consequence of a condition as
/// interpreted by the resilience subsystem.
///
/// It is deliberately independent of:
//!
//! - fault probability;
//! - confidence;
//! - resource count;
//! - recovery priority;
//! - recovery action;
//! - backend identity.
//!
//! The variants are ordered from least to most severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severity {
    /// Informational condition with no currently established material impact
    /// on continued execution.
    ///
    /// An informational condition may still be useful for telemetry,
    /// diagnosis, historical analysis, or future prediction.
    Informational,

    /// Execution or resource condition is degraded but remains materially
    /// usable under the current execution constraints.
    Degraded,

    /// Material operational impact exists and normal execution may require
    /// adaptation or intervention.
    Major,

    /// Severe condition that materially threatens correctness, availability,
    /// or a required execution invariant.
    ///
    /// `Critical` does not itself authorize recovery or termination.
    Critical,

    /// The condition is considered incompatible with safe continuation under
    /// the applicable resilience interpretation.
    ///
    /// `Fatal` does not itself perform an abort. Policy and the verification
    /// boundary remain responsible for deciding whether continuation is
    /// permissible.
    Fatal,
}

impl Severity {
    /// Returns the least severe canonical level.
    #[must_use]
    pub const fn minimum() -> Self {
        Self::Informational
    }

    /// Returns the greatest canonical level.
    #[must_use]
    pub const fn maximum() -> Self {
        Self::Fatal
    }

    /// Returns the stable semantic rank.
    ///
    /// The rank exists for deterministic ordering and interoperability only.
    /// It is not a probability, percentage, or quantitative impact measure.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Informational => 0,
            Self::Degraded => 1,
            Self::Major => 2,
            Self::Critical => 3,
            Self::Fatal => 4,
        }
    }

    /// Constructs a severity from its stable semantic rank.
    ///
    /// Returns `None` for values that are not part of the canonical Zamani
    /// severity vocabulary.
    ///
    /// Unknown values are rejected rather than silently mapped to another
    /// severity. This is important for forward-compatible serialized data.
    #[must_use]
    pub const fn from_rank(rank: u8) -> Option<Self> {
        match rank {
            0 => Some(Self::Informational),
            1 => Some(Self::Degraded),
            2 => Some(Self::Major),
            3 => Some(Self::Critical),
            4 => Some(Self::Fatal),
            _ => None,
        }
    }

    /// Returns the stable machine-readable textual representation.
    ///
    /// These values are deliberately provider-independent.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Degraded => "degraded",
            Self::Major => "major",
            Self::Critical => "critical",
            Self::Fatal => "fatal",
        }
    }

    /// Returns whether this severity represents at least the supplied level.
    #[must_use]
    pub const fn is_at_least(self, minimum: Self) -> bool {
        self.rank() >= minimum.rank()
    }

    /// Returns whether this severity is below the supplied level.
    #[must_use]
    pub const fn is_below(self, threshold: Self) -> bool {
        self.rank() < threshold.rank()
    }

    /// Returns whether this severity is informational.
    #[must_use]
    pub const fn is_informational(self) -> bool {
        matches!(self, Self::Informational)
    }

    /// Returns whether this severity represents a degraded condition.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Returns whether this severity is major or greater.
    #[must_use]
    pub const fn is_major_or_greater(self) -> bool {
        self.is_at_least(Self::Major)
    }

    /// Returns whether this severity is critical or greater.
    #[must_use]
    pub const fn is_critical_or_greater(self) -> bool {
        self.is_at_least(Self::Critical)
    }

    /// Returns whether this severity is fatal.
    #[must_use]
    pub const fn is_fatal(self) -> bool {
        matches!(self, Self::Fatal)
    }

    /// Returns the more severe of two values.
    ///
    /// This is useful for conservative aggregation of independent severity
    /// observations.
    ///
    /// It does not perform incident diagnosis and does not account for
    /// confidence, correlation, scope, or policy.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }

    /// Returns the less severe of two values.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    /// Saturating escalation by a number of severity levels.
    ///
    /// Escalation stops at [`Severity::Fatal`].
    ///
    /// This operation changes only the severity value. It does not represent
    /// or execute a recovery action.
    ///
    /// `levels` is interpreted as a number of semantic severity steps, not as
    /// a machine/resource quantity.
    #[must_use]
    pub const fn escalate(self, levels: u8) -> Self {
        let rank = self.rank().saturating_add(levels);

        match Self::from_rank(rank) {
            Some(value) => value,
            None => Self::Fatal,
        }
    }

    /// Returns the stable textual name suitable for diagnostics.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.as_str()
    }

    /// Returns whether a severity transition is an escalation.
    #[must_use]
    pub const fn is_escalation(from: Self, to: Self) -> bool {
        to.rank() > from.rank()
    }

    /// Returns whether a severity transition is a de-escalation.
    #[must_use]
    pub const fn is_deescalation(from: Self, to: Self) -> bool {
        to.rank() < from.rank()
    }

    /// Returns whether two severities have equal semantic rank.
    #[must_use]
    pub const fn same_level(self, other: Self) -> bool {
        self.rank() == other.rank()
    }
}

impl Default for Severity {
    /// The default severity is informational.
    ///
    /// This default represents the least severe *known* severity. It must not
    /// be used to represent missing or uncertain data; callers should use
    /// `Option<Severity>` for absence and `model::confidence` for uncertainty.
    fn default() -> Self {
        Self::Informational
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error returned when parsing a severity from external textual data.
///
/// This error intentionally contains the original input so callers can
/// diagnose malformed configuration or serialized data without requiring a
/// dependency on the resilience error subsystem.
///
/// It does not expose backend credentials, hardware state, or other
/// capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidSeverity {
    input: String,
}

impl InvalidSeverity {
    /// Returns the input that could not be parsed.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for InvalidSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid resilience severity: {:?}", self.input)
    }
}

impl std::error::Error for InvalidSeverity {}

impl FromStr for Severity {
    type Err = InvalidSeverity;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "informational" | "Information" | "information" => {
                Ok(Self::Informational)
            }
            "degraded" | "Degraded" => Ok(Self::Degraded),
            "major" | "Major" => Ok(Self::Major),
            "critical" | "Critical" => Ok(Self::Critical),
            "fatal" | "Fatal" => Ok(Self::Fatal),
            _ => Err(InvalidSeverity {
                input: value.to_owned(),
            }),
        }
    }
}

impl TryFrom<u8> for Severity {
    type Error = InvalidSeverity;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_rank(value).ok_or_else(|| InvalidSeverity {
            input: value.to_string(),
        })
    }
}

impl From<Severity> for u8 {
    fn from(value: Severity) -> Self {
        value.rank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_order_is_stable() {
        assert!(Severity::Informational < Severity::Degraded);
        assert!(Severity::Degraded < Severity::Major);
        assert!(Severity::Major < Severity::Critical);
        assert!(Severity::Critical < Severity::Fatal);
    }

    #[test]
    fn ranks_are_stable() {
        assert_eq!(Severity::Informational.rank(), 0);
        assert_eq!(Severity::Degraded.rank(), 1);
        assert_eq!(Severity::Major.rank(), 2);
        assert_eq!(Severity::Critical.rank(), 3);
        assert_eq!(Severity::Fatal.rank(), 4);
    }

    #[test]
    fn rank_round_trip_is_lossless() {
        let severities = [
            Severity::Informational,
            Severity::Degraded,
            Severity::Major,
            Severity::Critical,
            Severity::Fatal,
        ];

        for severity in severities {
            assert_eq!(Severity::from_rank(severity.rank()), Some(severity));
        }
    }

    #[test]
    fn invalid_rank_is_rejected() {
        assert_eq!(Severity::from_rank(5), None);
        assert_eq!(Severity::from_rank(u8::MAX), None);
    }

    #[test]
    fn textual_representation_is_stable() {
        assert_eq!(
            Severity::Informational.as_str(),
            "informational"
        );
        assert_eq!(Severity::Degraded.as_str(), "degraded");
        assert_eq!(Severity::Major.as_str(), "major");
        assert_eq!(Severity::Critical.as_str(), "critical");
        assert_eq!(Severity::Fatal.as_str(), "fatal");
    }

    #[test]
    fn textual_round_trip_is_lossless() {
        let severities = [
            Severity::Informational,
            Severity::Degraded,
            Severity::Major,
            Severity::Critical,
            Severity::Fatal,
        ];

        for severity in severities {
            let encoded = severity.as_str();
            let decoded: Severity = encoded.parse().expect("canonical severity");
            assert_eq!(decoded, severity);
        }
    }

    #[test]
    fn invalid_text_is_rejected() {
        let result = "unknown".parse::<Severity>();

        assert!(result.is_err());

        let error = result.expect_err("unknown severity must be rejected");
        assert_eq!(error.input(), "unknown");
    }

    #[test]
    fn predicates_are_semantically_correct() {
        assert!(Severity::Critical.is_critical_or_greater());
        assert!(Severity::Fatal.is_critical_or_greater());

        assert!(!Severity::Major.is_critical_or_greater());

        assert!(Severity::Major.is_major_or_greater());
        assert!(!Severity::Degraded.is_major_or_greater());

        assert!(Severity::Fatal.is_fatal());
        assert!(!Severity::Critical.is_fatal());

        assert!(Severity::Informational.is_informational());
        assert!(Severity::Degraded.is_degraded());
    }

    #[test]
    fn threshold_comparison_is_deterministic() {
        assert!(Severity::Major.is_at_least(Severity::Degraded));
        assert!(Severity::Major.is_at_least(Severity::Major));
        assert!(!Severity::Major.is_at_least(Severity::Critical));

        assert!(Severity::Degraded.is_below(Severity::Major));
        assert!(!Severity::Major.is_below(Severity::Major));
    }

    #[test]
    fn max_is_conservative() {
        assert_eq!(
            Severity::Degraded.max(Severity::Critical),
            Severity::Critical
        );

        assert_eq!(
            Severity::Fatal.max(Severity::Informational),
            Severity::Fatal
        );
    }

    #[test]
    fn min_is_correct() {
        assert_eq!(
            Severity::Degraded.min(Severity::Critical),
            Severity::Degraded
        );

        assert_eq!(
            Severity::Fatal.min(Severity::Major),
            Severity::Major
        );
    }

    #[test]
    fn escalation_saturates_at_fatal() {
        assert_eq!(
            Severity::Informational.escalate(1),
            Severity::Degraded
        );

        assert_eq!(
            Severity::Major.escalate(1),
            Severity::Critical
        );

        assert_eq!(
            Severity::Critical.escalate(1),
            Severity::Fatal
        );

        assert_eq!(
            Severity::Fatal.escalate(1),
            Severity::Fatal
        );

        assert_eq!(
            Severity::Informational.escalate(u8::MAX),
            Severity::Fatal
        );
    }

    #[test]
    fn zero_escalation_preserves_value() {
        let severities = [
            Severity::Informational,
            Severity::Degraded,
            Severity::Major,
            Severity::Critical,
            Severity::Fatal,
        ];

        for severity in severities {
            assert_eq!(severity.escalate(0), severity);
        }
    }

    #[test]
    fn transition_helpers_are_correct() {
        assert!(Severity::is_escalation(
            Severity::Degraded,
            Severity::Critical
        ));

        assert!(!Severity::is_escalation(
            Severity::Critical,
            Severity::Degraded
        ));

        assert!(Severity::is_deescalation(
            Severity::Critical,
            Severity::Degraded
        ));

        assert!(!Severity::is_deescalation(
            Severity::Degraded,
            Severity::Critical
        ));

        assert!(Severity::same_level(
            Severity::Major,
            Severity::Major
        ));
    }

    #[test]
    fn default_is_informational() {
        assert_eq!(Severity::default(), Severity::Informational);
    }

    #[test]
    fn display_is_stable() {
        assert_eq!(
            Severity::Critical.to_string(),
            "critical"
        );
    }

    #[test]
    fn numeric_conversion_is_lossless() {
        let severities = [
            Severity::Informational,
            Severity::Degraded,
            Severity::Major,
            Severity::Critical,
            Severity::Fatal,
        ];

        for severity in severities {
            let encoded: u8 = severity.into();
            let decoded =
                Severity::try_from(encoded).expect("valid severity rank");

            assert_eq!(decoded, severity);
        }
    }
}