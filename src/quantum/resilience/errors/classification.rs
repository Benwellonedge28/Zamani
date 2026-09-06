//! Zamani Quantum Resilience — Error Classification
//!
//! This module defines the canonical, provider-neutral semantic classification
//! of errors produced by `quantum::resilience`.
//!
//! # Architectural responsibility
//!
//! `ResilienceErrorCategory` answers one question:
//!
//! > Which semantic subsystem/domain does this error belong to?
//!
//! It does NOT decide:
//!
//! - whether an operation should be retried;
//! - whether recovery is safe;
//! - which recovery action should be selected;
//! - which backend should be used;
//! - whether a result should be accepted;
//! - whether a fault is quantum-mechanical or classical;
//! - whether a policy should permit an operation.
//!
//! Those decisions belong to the corresponding resilience layers.
//!
//! # Architecture
//!
//! ```text
//!                         ResilienceError
//!                               |
//!                               v
//!                    ResilienceErrorCategory
//!                               |
//!          +--------------------+--------------------+
//!          |                    |                    |
//!          v                    v                    v
//!      Validation           Detection            Diagnosis
//!          |                    |                    |
//!          +--------------------+--------------------+
//!                               |
//!                               v
//!                            Policy
//!                               |
//!                               v
//!                           Planning
//!                               |
//!                               v
//!                           Adaptation
//!                               |
//!                               v
//!                           Recovery
//!                               |
//!                               v
//!                          Verification
//! ```
//!
//! # Design principles
//!
//! ## 1. Stable semantics
//!
//! Category variants are part of the internal and potentially serialized
//! resilience contract. Their discriminants therefore MUST NOT be reused for a
//! different semantic meaning after release.
//!
//! ## 2. Provider neutrality
//!
//! No category identifies IBM, Google, Quantinuum, Rigetti, IonQ, a simulator,
//! or any other provider.
//!
//! Provider/device identity belongs to the hardware abstraction layer.
//!
//! ## 3. No machine-size assumptions
//!
//! This module contains no qubit count, topology size, retry count, memory
//! limit, device identifier, or backend-specific constant.
//!
//! It therefore scales from a single-qubit execution to arbitrarily large
//! systems subject only to the capabilities and resources supplied by the
//! surrounding execution environment.
//!
//! ## 4. Separation of classification and policy
//!
//! A category describes what kind of failure occurred. It does not prescribe
//! what to do about it.
//!
//! For example:
//!
//! ```text
//! Recovery category != automatically retry
//! Hardware category != automatically migrate
//! Verification category != automatically abort
//! ```
//!
//! The policy/planning layers make those decisions using the category plus
//! evidence, constraints, budgets, capabilities, and execution state.
//!
//! ## 5. Forward compatibility
//!
//! Unknown serialized category values MUST be handled as an error by
//! `from_u8`/`from_str` rather than silently mapped to an unrelated category.
//! This prevents a future category from being misinterpreted as an existing
//! one.
//!
//! ## 6. Determinism
//!
//! All mappings in this module are pure and deterministic.
//!
//! Identical category values always produce identical representations.
//!
//! ## 7. Rust compatibility
//!
//! This file is designed for:
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
//! `error.rs` MUST import this type using:
//!
//! ```rust
//! use super::classification::ResilienceErrorCategory;
//! ```
//!
//! `errors/mod.rs` SHOULD expose it using:
//!
//! ```rust
//! pub mod classification;
//! pub use classification::ResilienceErrorCategory;
//! ```
//!
//! `error.rs` then uses the same canonical type for:
//!
//! ```text
//! ResilienceErrorCode::category()
//! ResilienceError::category()
//! ```
//!
//! No other resilience module should define another error-category enum.
//!
//! # Dependency direction
//!
//! ```text
//! classification.rs
//!        ^
//!        |
//!     error.rs
//!        ^
//!        |
//! detection / diagnosis / policy / planning / recovery / ...
//! ```
//!
//! This module intentionally has no dependency on `error.rs`.
//!
//! That prevents a circular dependency and allows classification to be used
//! by future error, telemetry, serialization, and observability components.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier for resilience error classification.
pub const RESILIENCE_ERROR_CLASSIFICATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.error-classification";

/// Semantic version of the classification schema.
///
/// This version is independent from the numeric discriminants. The version
/// changes when the externally observable classification contract changes,
/// while discriminants remain permanently stable once released.
pub const RESILIENCE_ERROR_CLASSIFICATION_SCHEMA_VERSION: u16 = 1;

/// Number of currently defined categories.
///
/// This is metadata, not a machine-size limit. It MUST NOT be interpreted as
/// a limit on errors, faults, qubits, resources, incidents, or executions.
pub const RESILIENCE_ERROR_CATEGORY_COUNT: usize = 19;

// =============================================================================
// Canonical category
// =============================================================================

/// Broad semantic category of a quantum-resilience error.
///
/// This is the canonical category type for `quantum::resilience`.
///
/// The discriminants are deliberately explicit and stable. Do not renumber
/// existing variants after release.
///
/// Categories describe semantic ownership; they do not prescribe recovery
/// behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ResilienceErrorCategory {
    /// Input, configuration, representation, or invariant validation failed.
    Validation = 1,

    /// Fault or anomaly observation/detection failed.
    Detection = 2,

    /// Fault diagnosis or root-cause analysis failed.
    Diagnosis = 3,

    /// A resilience policy rejected or constrained an operation.
    Policy = 4,

    /// Recovery/adaptation planning failed.
    Planning = 5,

    /// The computation or execution target could not be adapted safely.
    Adaptation = 6,

    /// Recovery execution failed or was aborted.
    Recovery = 7,

    /// Error mitigation failed or was unavailable.
    Mitigation = 8,

    /// Result, semantic, invariant, or provenance verification failed.
    Verification = 9,

    /// A required logical, physical, or execution resource is unavailable
    /// or changed.
    Resource = 10,

    /// Hardware or backend execution infrastructure failed.
    Hardware = 11,

    /// Quantum error-correction integration or logical-error handling failed.
    Qec = 12,

    /// Checkpoint creation, restoration, integrity, or compatibility failed.
    Checkpoint = 13,

    /// Internal resilience state or concurrency management failed.
    State = 14,

    /// Serialization, deserialization, or schema compatibility failed.
    Serialization = 15,

    /// Authentication, authorization, trust, or security enforcement failed.
    Security = 16,

    /// Execution timing, cancellation, deadline, or interruption failed.
    Execution = 17,

    /// An extensible resilience component/extension failed or was unavailable.
    Component = 18,

    /// The failure could not be safely assigned to a more specific category.
    Internal = 19,
}

// =============================================================================
// Static category set
// =============================================================================

impl ResilienceErrorCategory {
    /// All currently defined categories in stable discriminant order.
    ///
    /// This is useful for:
    ///
    /// - validation;
    /// - deterministic iteration;
    /// - telemetry schema generation;
    /// - documentation generation;
    /// - exhaustive compatibility tests.
    ///
    /// The returned slice is immutable and contains no runtime-sized
    /// allocation.
    pub const ALL: &'static [Self] = &[
        Self::Validation,
        Self::Detection,
        Self::Diagnosis,
        Self::Policy,
        Self::Planning,
        Self::Adaptation,
        Self::Recovery,
        Self::Mitigation,
        Self::Verification,
        Self::Resource,
        Self::Hardware,
        Self::Qec,
        Self::Checkpoint,
        Self::State,
        Self::Serialization,
        Self::Security,
        Self::Execution,
        Self::Component,
        Self::Internal,
    ];

    /// Returns the stable numeric discriminant.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the stable machine-readable category identifier.
    ///
    /// These identifiers are suitable for:
    ///
    /// - telemetry;
    /// - logs;
    /// - metrics;
    /// - serialized diagnostics;
    /// - cross-process protocols;
    /// - deterministic tests.
    ///
    /// They are more stable than human-facing display text.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Policy => "policy",
            Self::Planning => "planning",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Mitigation => "mitigation",
            Self::Verification => "verification",
            Self::Resource => "resource",
            Self::Hardware => "hardware",
            Self::Qec => "qec",
            Self::Checkpoint => "checkpoint",
            Self::State => "state",
            Self::Serialization => "serialization",
            Self::Security => "security",
            Self::Execution => "execution",
            Self::Component => "component",
            Self::Internal => "internal",
        }
    }

    /// Returns a stable fully-qualified category identifier.
    ///
    /// This avoids collisions if another subsystem has a category named
    /// `validation`, `hardware`, etc.
    #[must_use]
    pub const fn qualified_name(self) -> &'static str {
        match self {
            Self::Validation => "zamani.quantum.resilience.validation",
            Self::Detection => "zamani.quantum.resilience.detection",
            Self::Diagnosis => "zamani.quantum.resilience.diagnosis",
            Self::Policy => "zamani.quantum.resilience.policy",
            Self::Planning => "zamani.quantum.resilience.planning",
            Self::Adaptation => "zamani.quantum.resilience.adaptation",
            Self::Recovery => "zamani.quantum.resilience.recovery",
            Self::Mitigation => "zamani.quantum.resilience.mitigation",
            Self::Verification => "zamani.quantum.resilience.verification",
            Self::Resource => "zamani.quantum.resilience.resource",
            Self::Hardware => "zamani.quantum.resilience.hardware",
            Self::Qec => "zamani.quantum.resilience.qec",
            Self::Checkpoint => "zamani.quantum.resilience.checkpoint",
            Self::State => "zamani.quantum.resilience.state",
            Self::Serialization => "zamani.quantum.resilience.serialization",
            Self::Security => "zamani.quantum.resilience.security",
            Self::Execution => "zamani.quantum.resilience.execution",
            Self::Component => "zamani.quantum.resilience.component",
            Self::Internal => "zamani.quantum.resilience.internal",
        }
    }

    /// Returns the category represented by a stable numeric discriminant.
    ///
    /// Unknown values return `None`. They MUST NOT be silently mapped to
    /// `Internal`, because doing so would make a future category appear to be
    /// an existing category and could produce an unsafe recovery decision.
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Validation),
            2 => Some(Self::Detection),
            3 => Some(Self::Diagnosis),
            4 => Some(Self::Policy),
            5 => Some(Self::Planning),
            6 => Some(Self::Adaptation),
            7 => Some(Self::Recovery),
            8 => Some(Self::Mitigation),
            9 => Some(Self::Verification),
            10 => Some(Self::Resource),
            11 => Some(Self::Hardware),
            12 => Some(Self::Qec),
            13 => Some(Self::Checkpoint),
            14 => Some(Self::State),
            15 => Some(Self::Serialization),
            16 => Some(Self::Security),
            17 => Some(Self::Execution),
            18 => Some(Self::Component),
            19 => Some(Self::Internal),
            _ => None,
        }
    }

    /// Returns the category represented by its stable machine-readable name.
    ///
    /// Matching is deliberately exact and case-sensitive. This prevents
    /// ambiguous external representations from being accepted silently.
    #[must_use]
    pub const fn from_str(value: &str) -> Option<Self> {
        match value {
            "validation" => Some(Self::Validation),
            "detection" => Some(Self::Detection),
            "diagnosis" => Some(Self::Diagnosis),
            "policy" => Some(Self::Policy),
            "planning" => Some(Self::Planning),
            "adaptation" => Some(Self::Adaptation),
            "recovery" => Some(Self::Recovery),
            "mitigation" => Some(Self::Mitigation),
            "verification" => Some(Self::Verification),
            "resource" => Some(Self::Resource),
            "hardware" => Some(Self::Hardware),
            "qec" => Some(Self::Qec),
            "checkpoint" => Some(Self::Checkpoint),
            "state" => Some(Self::State),
            "serialization" => Some(Self::Serialization),
            "security" => Some(Self::Security),
            "execution" => Some(Self::Execution),
            "component" => Some(Self::Component),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }

    /// Returns the category represented by a qualified machine-readable name.
    #[must_use]
    pub const fn from_qualified_name(value: &str) -> Option<Self> {
        match value {
            "zamani.quantum.resilience.validation" => Some(Self::Validation),
            "zamani.quantum.resilience.detection" => Some(Self::Detection),
            "zamani.quantum.resilience.diagnosis" => Some(Self::Diagnosis),
            "zamani.quantum.resilience.policy" => Some(Self::Policy),
            "zamani.quantum.resilience.planning" => Some(Self::Planning),
            "zamani.quantum.resilience.adaptation" => Some(Self::Adaptation),
            "zamani.quantum.resilience.recovery" => Some(Self::Recovery),
            "zamani.quantum.resilience.mitigation" => Some(Self::Mitigation),
            "zamani.quantum.resilience.verification" => Some(Self::Verification),
            "zamani.quantum.resilience.resource" => Some(Self::Resource),
            "zamani.quantum.resilience.hardware" => Some(Self::Hardware),
            "zamani.quantum.resilience.qec" => Some(Self::Qec),
            "zamani.quantum.resilience.checkpoint" => Some(Self::Checkpoint),
            "zamani.quantum.resilience.state" => Some(Self::State),
            "zamani.quantum.resilience.serialization" => Some(Self::Serialization),
            "zamani.quantum.resilience.security" => Some(Self::Security),
            "zamani.quantum.resilience.execution" => Some(Self::Execution),
            "zamani.quantum.resilience.component" => Some(Self::Component),
            "zamani.quantum.resilience.internal" => Some(Self::Internal),
            _ => None,
        }
    }

    /// Returns whether this category represents an external trust boundary.
    ///
    /// This is descriptive metadata only. It does not authorize or reject
    /// anything.
    #[must_use]
    pub const fn crosses_external_boundary(self) -> bool {
        match self {
            Self::Hardware
            | Self::Checkpoint
            | Self::Serialization
            | Self::Security
            | Self::Component
            | Self::Execution => true,

            Self::Validation
            | Self::Detection
            | Self::Diagnosis
            | Self::Policy
            | Self::Planning
            | Self::Adaptation
            | Self::Recovery
            | Self::Mitigation
            | Self::Verification
            | Self::Resource
            | Self::Qec
            | Self::State
            | Self::Internal => false,
        }
    }

    /// Returns whether the category can represent information originating
    /// outside the resilience decision engine.
    ///
    /// This is intentionally broader than `crosses_external_boundary`.
    #[must_use]
    pub const fn may_have_external_evidence(self) -> bool {
        match self {
            Self::Detection
            | Self::Diagnosis
            | Self::Resource
            | Self::Hardware
            | Self::Qec
            | Self::Checkpoint
            | Self::Serialization
            | Self::Security
            | Self::Execution
            | Self::Component => true,

            Self::Validation
            | Self::Policy
            | Self::Planning
            | Self::Adaptation
            | Self::Recovery
            | Self::Mitigation
            | Self::Verification
            | Self::State
            | Self::Internal => false,
        }
    }

    /// Returns whether this category represents a failure of a stateful
    /// execution/resilience subsystem.
    ///
    /// This classification does not imply that the whole computation must be
    /// aborted.
    #[must_use]
    pub const fn is_state_related(self) -> bool {
        match self {
            Self::State
            | Self::Resource
            | Self::Checkpoint
            | Self::Recovery
            | Self::Execution => true,

            Self::Validation
            | Self::Detection
            | Self::Diagnosis
            | Self::Policy
            | Self::Planning
            | Self::Adaptation
            | Self::Mitigation
            | Self::Verification
            | Self::Hardware
            | Self::Qec
            | Self::Serialization
            | Self::Security
            | Self::Component
            | Self::Internal => false,
        }
    }

    /// Returns whether this category is directly related to correctness
    /// evidence or acceptance of an execution result.
    ///
    /// This is descriptive and does not itself accept or reject a result.
    #[must_use]
    pub const fn is_correctness_related(self) -> bool {
        match self {
            Self::Verification
            | Self::Qec
            | Self::Mitigation
            | Self::Adaptation
            | Self::Recovery
            | Self::Planning => true,

            Self::Validation
            | Self::Detection
            | Self::Diagnosis
            | Self::Policy
            | Self::Resource
            | Self::Hardware
            | Self::Checkpoint
            | Self::State
            | Self::Serialization
            | Self::Security
            | Self::Execution
            | Self::Component
            | Self::Internal => false,
        }
    }

    /// Returns whether the category is security-sensitive.
    ///
    /// Security-sensitive categories should receive appropriate provenance,
    /// authorization, and audit treatment. This function does not implement
    /// those controls.
    #[must_use]
    pub const fn is_security_sensitive(self) -> bool {
        match self {
            Self::Security | Self::Checkpoint | Self::Serialization | Self::Component => true,

            Self::Validation
            | Self::Detection
            | Self::Diagnosis
            | Self::Policy
            | Self::Planning
            | Self::Adaptation
            | Self::Recovery
            | Self::Mitigation
            | Self::Verification
            | Self::Resource
            | Self::Hardware
            | Self::Qec
            | Self::State
            | Self::Execution
            | Self::Internal => false,
        }
    }

    /// Returns whether the category is expected to be useful for telemetry
    /// aggregation.
    ///
    /// Every category is telemetry-safe as a classification identifier.
    #[must_use]
    pub const fn is_telemetry_category(self) -> bool {
        let _ = self;
        true
    }

    /// Returns the broad lifecycle stage represented by this category.
    ///
    /// The value is a stable semantic label and is intentionally represented
    /// as `&'static str` so this module does not introduce another enum whose
    /// compatibility would have to be coordinated with the resilience state
    /// machine.
    #[must_use]
    pub const fn lifecycle_stage(self) -> &'static str {
        match self {
            Self::Validation => "input",
            Self::Detection => "observation",
            Self::Diagnosis => "analysis",
            Self::Policy => "decision",
            Self::Planning => "decision",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Mitigation => "execution-support",
            Self::Verification => "verification",
            Self::Resource => "execution",
            Self::Hardware => "execution",
            Self::Qec => "correction",
            Self::Checkpoint => "state",
            Self::State => "state",
            Self::Serialization => "boundary",
            Self::Security => "security",
            Self::Execution => "execution",
            Self::Component => "extension",
            Self::Internal => "internal",
        }
    }
}

// =============================================================================
// Display
// =============================================================================

impl core::fmt::Display for ResilienceErrorCategory {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_categories_have_unique_stable_discriminants() {
        for (index, category) in ResilienceErrorCategory::ALL.iter().enumerate() {
            assert_eq!(category.as_u8(), (index + 1) as u8);
        }

        assert_eq!(
            ResilienceErrorCategory::ALL.len(),
            RESILIENCE_ERROR_CATEGORY_COUNT
        );
    }

    #[test]
    fn numeric_round_trip_is_exact() {
        for category in ResilienceErrorCategory::ALL {
            assert_eq!(
                ResilienceErrorCategory::from_u8(category.as_u8()),
                Some(*category)
            );
        }
    }

    #[test]
    fn string_round_trip_is_exact() {
        for category in ResilienceErrorCategory::ALL {
            assert_eq!(
                ResilienceErrorCategory::from_str(category.as_str()),
                Some(*category)
            );
        }
    }

    #[test]
    fn qualified_name_round_trip_is_exact() {
        for category in ResilienceErrorCategory::ALL {
            assert_eq!(
                ResilienceErrorCategory::from_qualified_name(category.qualified_name()),
                Some(*category)
            );
        }
    }

    #[test]
    fn unknown_numeric_values_are_rejected() {
        assert_eq!(ResilienceErrorCategory::from_u8(0), None);
        assert_eq!(ResilienceErrorCategory::from_u8(u8::MAX), None);
    }

    #[test]
    fn unknown_string_values_are_rejected() {
        assert_eq!(ResilienceErrorCategory::from_str(""), None);
        assert_eq!(ResilienceErrorCategory::from_str("Hardware"), None);
        assert_eq!(
            ResilienceErrorCategory::from_str("hardware "),
            None
        );
        assert_eq!(
            ResilienceErrorCategory::from_str("zamani.quantum.resilience.hardware"),
            None
        );
    }

    #[test]
    fn category_names_are_unique() {
        for (index, left) in ResilienceErrorCategory::ALL.iter().enumerate() {
            for right in ResilienceErrorCategory::ALL.iter().skip(index + 1) {
                assert_ne!(left.as_str(), right.as_str());
                assert_ne!(left.qualified_name(), right.qualified_name());
                assert_ne!(left.as_u8(), right.as_u8());
            }
        }
    }

    #[test]
    fn display_is_machine_name() {
        for category in ResilienceErrorCategory::ALL {
            assert_eq!(category.to_string(), category.as_str());
        }
    }

    #[test]
    fn internal_is_only_the_fallback_category() {
        assert_eq!(
            ResilienceErrorCategory::from_u8(19),
            Some(ResilienceErrorCategory::Internal)
        );
    }

    #[test]
    fn category_count_matches_canonical_set() {
        assert_eq!(
            ResilienceErrorCategory::ALL.len(),
            RESILIENCE_ERROR_CATEGORY_COUNT
        );
    }
}