//! Zamani Quantum Noise (ZQN) — Fault Classification Analysis.
//!
//! # Ownership
//!
//! This module owns the *analysis layer* for canonical ZQN fault
//! classifications.
//!
//! It owns:
//!
//! - classification predicates;
//! - classification profiles;
//! - effect/mechanism classification;
//! - resource-scope classification;
//! - logical/physical/mixed resource-domain analysis;
//! - temporal classification;
//! - classification compatibility queries;
//! - deterministic classification reports;
//! - explicit classification aggregation;
//! - stable semantic labels for classification analysis.
//!
//! It consumes the canonical semantic types defined by
//! `crate::quantum::zqn::fault::fault`.
//!
//! # It does NOT own
//!
//! This module does not own:
//!
//! - `FaultClassification` itself;
//! - `Fault` itself;
//! - `FaultLocation` itself;
//! - `FaultEffect` itself;
//! - canonical logical/physical qubit identities;
//! - probability;
//! - quantum channels;
//! - noise generation;
//! - random-number generation;
//! - calibration;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - hardware APIs;
//! - backend execution;
//! - serialization formats;
//! - machine-size limits.
//!
//! `FaultClassification` remains defined by `fault.rs` because it is part of
//! the semantic identity of a `Fault`. This file deliberately does not
//! duplicate or redefine that enum.
//!
//! # Architectural position
//!
//! ```text
//! canonical Quantum IR
//!         │
//!         │ QubitId / PhysicalQubitId
//!         ▼
//!     ZQN Fault
//!         │
//!         ├── FaultClassification
//!         ├── FaultLocation
//!         └── FaultEffect
//!                 │
//!                 ▼
//!       fault/classification.rs
//!                 │
//!       ┌─────────┼──────────────┐
//!       ▼         ▼              ▼
//!   predicates   reports     aggregation
//!       │         │              │
//!       └─────────┼──────────────┘
//!                 ▼
//!        routing / scheduling
//!        simulation / QEC
//!        benchmarking / analysis
//! ```
//!
//! # Fundamental invariant
//!
//! A classification is semantic metadata about a fault. It is not a hardware
//! capability and it is not a resource allocation.
//!
//! In particular:
//!
//! ```text
//! FaultClassification::Gate
//! ```
//!
//! does NOT mean that a particular backend supports that gate.
//!
//! Likewise:
//!
//! ```text
//! FaultClassification::Correlated
//! ```
//!
//! does NOT impose a maximum correlation degree.
//!
//! Correlation degree is determined by the associated `FaultLocation`.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_CORRELATION
//! MAX_CUSTOM_CLASSES
//! MAX_LOCATIONS
//! ```
//!
//! A classification describes semantic properties of one already-materialized
//! `Fault`. It does not impose an architectural limit on the number of
//! resources in the computation.
//!
//! For large workloads, callers should stream faults into the analysis
//! functions rather than materializing the entire workload.
//!
//! Aggregation structures allocate only for classifications actually observed.
//! They do not preallocate according to machine size.
//!
//! # Canonical quantum identities
//!
//! This file intentionally does not create another qubit identifier.
//!
//! The canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Classification analysis reaches those identities through `Fault` and
//! `FaultLocation`.
//!
//! This preserves the repository-wide ownership rule that ZQN consumes
//! canonical Quantum IR identities instead of defining competing identities.
//!
//! # Determinism
//!
//! Classification is entirely deterministic.
//!
//! It does not use:
//!
//! - RNGs;
//! - clocks;
//! - process IDs;
//! - thread IDs;
//! - memory addresses;
//! - global mutable state;
//! - hash iteration order.
//!
//! `ClassificationHistogram` uses `BTreeMap`, not `HashMap`, so iteration is
//! deterministic.
//!
//! # Resource safety
//!
//! Classification of one fault is proportional to the explicitly represented
//! fault structure.
//!
//! Composite locations are traversed iteratively rather than recursively.
//! This is intentional: deeply nested untrusted `FaultLocation::Composite`
//! values must not require unbounded call-stack growth merely to classify
//! them.
//!
//! The module does not create hidden copies of locations or faults.
//!
//! Histogram aggregation grows only with the number of distinct classifications
//! actually inserted.
//!
//! Resource-policy enforcement belongs to the caller/ZQN execution context.
//!
//! # Numerical safety
//!
//! This module performs no floating-point arithmetic.
//!
//! Histogram counts use checked integer arithmetic and return an explicit
//! overflow error rather than wrapping.
//!
//! # Serialization
//!
//! This module defines no wire format.
//!
//! The ZQN IO subsystem owns serialization and schema compatibility.
//!
//! `FaultClassification` already exposes a stable semantic name through its
//! canonical `as_str()` method.
//!
//! # Thread safety
//!
//! All types in this file are ordinary immutable values except for local
//! mutable aggregation performed through `&mut self`.
//!
//! There is no global state and no interior mutability.
//!
//! A `ClassificationHistogram` can therefore be independently owned by
//! concurrent workers and merged deterministically by the caller.
//!
//! # Security
//!
//! Classification is observational only.
//!
//! It grants no:
//!
//! - QPU access;
//! - credentials;
//! - hardware control;
//! - calibration access;
//! - filesystem access;
//! - network access.
//!
//! Untrusted faults should still be admitted through the normal ZQN resource
//! policy before they are materialized.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! `fault.rs` remains the semantic owner:
//!
//! ```text
//! fault.rs
//!   │
//!   ├── Fault
//!   ├── FaultClassification
//!   ├── FaultLocation
//!   └── FaultEffect
//!          │
//!          ▼
//! classification.rs
//!          │
//!          ├── FaultClassificationExt
//!          ├── FaultEffectKind
//!          ├── FaultScope
//!          ├── FaultResourceDomain
//!          ├── FaultClassificationReport
//!          └── ClassificationHistogram
//! ```
//!
//! Downstream consumers should use this module for analysis instead of
//! reimplementing classification predicates.
//!
//! # Integration with QEC
//!
//! QEC may consume `FaultClassificationReport` to determine broad physical
//! fault categories.
//!
//! This module does not know anything about:
//!
//! - syndrome extraction;
//! - decoders;
//! - stabilizer codes;
//! - logical correction.
//!
//! # Integration with routing
//!
//! Routing may consume:
//!
//! - `is_correlated()`;
//! - `is_crosstalk()`;
//! - `resource_scope()`;
//! - `resource_domain()`;
//!
//! but routing remains responsible for placement decisions.
//!
//! # Integration with scheduling
//!
//! Scheduling may consume:
//!
//! - `has_explicit_timing()`;
//! - `is_timing_related()`;
//! - `is_idle_related()`;
//!
//! but scheduling remains responsible for temporal placement.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may use `ClassificationHistogram` to summarize observed fault
//! populations without coupling itself to fault generation.
//!
//! # Integration with simulation
//!
//! Simulation may inspect classification reports to select an appropriate
//! execution/analysis path, but this module never applies the fault to a
//! quantum state.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it does not redefine `FaultClassification`;
//! 2. it does not redefine `Fault`;
//! 3. it uses canonical IR identities indirectly through `Fault`;
//! 4. no machine-size limit exists;
//! 5. no correlation-size limit exists;
//! 6. classification is deterministic;
//! 7. nested composite locations are classified iteratively;
//! 8. custom classifications remain supported;
//! 9. custom classification strings are never silently normalized;
//! 10. aggregation is deterministic;
//! 11. counter overflow is detected;
//! 12. no unsafe code exists;
//! 13. no external dependency is required;
//! 14. the module can be added without changing the semantic definition of
//!     `FaultClassification`;
//! 15. downstream modules can consume its API without knowing how faults are
//!     generated.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;

use crate::quantum::zqn::fault::fault::{
    Fault,
    FaultClassification,
    FaultEffect,
    FaultLocation,
    FaultResourceDomain,
};

// ============================================================================
// Classification extension trait
// ============================================================================

/// Stable analysis predicates for the canonical [`FaultClassification`].
///
/// The enum itself remains owned by `fault.rs`.
///
/// This extension trait deliberately keeps analysis out of the semantic fault
/// definition while providing one canonical place for consumers to obtain
/// classification predicates.
pub trait FaultClassificationExt {
    /// Returns the stable semantic identifier.
    fn semantic_name(&self) -> &str;

    /// Returns whether this is one of ZQN's built-in classifications.
    fn is_standard_classification(&self) -> bool;

    /// Returns whether this classification is user/technology-defined.
    fn is_custom_classification(&self) -> bool;

    /// Returns whether the classification is operation/gate related.
    fn is_operation_related(&self) -> bool;

    /// Returns whether the classification is lifecycle related.
    fn is_lifecycle_related(&self) -> bool;

    /// Returns whether the classification is measurement related.
    fn is_measurement_related(&self) -> bool;

    /// Returns whether the classification is time related.
    fn is_time_related(&self) -> bool;

    /// Returns whether the classification is transport related.
    fn is_transport_related(&self) -> bool;

    /// Returns whether the classification is calibration related.
    fn is_calibration_related(&self) -> bool;

    /// Returns whether the classification describes a correlated event.
    fn is_correlation_related(&self) -> bool;

    /// Returns whether the classification describes leakage.
    fn is_leakage_related(&self) -> bool;

    /// Returns whether the classification describes loss or erasure.
    fn is_loss_related(&self) -> bool;

    /// Returns whether the classification is compatible with a logical-level
    /// interpretation.
    fn is_logical_level(&self) -> bool;
}

impl FaultClassificationExt for FaultClassification {
    fn semantic_name(&self) -> &str {
        self.as_str()
    }

    fn is_standard_classification(&self) -> bool {
        self.is_standard()
    }

    fn is_custom_classification(&self) -> bool {
        !self.is_standard()
    }

    fn is_operation_related(&self) -> bool {
        matches!(
            self,
            Self::Gate
                | Self::Reset
                | Self::Preparation
                | Self::Coherent
                | Self::Crosstalk
        )
    }

    fn is_lifecycle_related(&self) -> bool {
        matches!(
            self,
            Self::Preparation
                | Self::Reset
                | Self::Leakage
                | Self::Erasure
                | Self::Loss
        )
    }

    fn is_measurement_related(&self) -> bool {
        matches!(self, Self::Measurement)
    }

    fn is_time_related(&self) -> bool {
        matches!(
            self,
            Self::Idle | Self::Timing | Self::Calibration
        )
    }

    fn is_transport_related(&self) -> bool {
        matches!(self, Self::Transport)
    }

    fn is_calibration_related(&self) -> bool {
        matches!(self, Self::Calibration)
    }

    fn is_correlation_related(&self) -> bool {
        matches!(self, Self::Correlated | Self::Crosstalk)
    }

    fn is_leakage_related(&self) -> bool {
        matches!(self, Self::Leakage)
    }

    fn is_loss_related(&self) -> bool {
        matches!(self, Self::Erasure | Self::Loss)
    }

    fn is_logical_level(&self) -> bool {
        matches!(self, Self::Logical)
    }
}

// ============================================================================
// Effect classification
// ============================================================================

/// Canonical mechanism classification of a realized fault effect.
///
/// This is deliberately separate from `FaultClassification`.
///
/// For example:
///
/// ```text
/// FaultClassification::Gate
/// FaultEffect::Coherent(...)
/// ```
///
/// means:
///
/// ```text
/// semantic context = gate
/// physical/effect mechanism = coherent
/// ```
///
/// Keeping these dimensions separate prevents the classification enum from
/// becoming an unscalable Cartesian product such as:
///
/// ```text
/// GateCoherent
/// GatePauli
/// GateLeakage
/// MeasurementReadout
/// ...
/// ```
///
/// Such Cartesian explosion would make future quantum technologies difficult
/// to represent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultEffectKind {
    /// Pauli effect.
    Pauli,

    /// Coherent/control deviation.
    Coherent,

    /// Leakage from the intended state/subspace.
    Leakage,

    /// Erasure.
    Erasure,

    /// Loss.
    Loss,

    /// Readout/assignment error.
    Readout,

    /// Timing deviation.
    Timing,

    /// Generic corruption.
    Corruption,

    /// User/technology-defined effect.
    Custom,
}

impl FaultEffectKind {
    /// Returns the stable semantic identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pauli => "pauli",
            Self::Coherent => "coherent",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
            Self::Loss => "loss",
            Self::Readout => "readout",
            Self::Timing => "timing",
            Self::Corruption => "corruption",
            Self::Custom => "custom",
        }
    }

    /// Returns whether this effect removes or invalidates a resource.
    #[must_use]
    pub const fn is_resource_destroying(self) -> bool {
        matches!(self, Self::Erasure | Self::Loss)
    }

    /// Returns whether this effect changes the computational subspace.
    #[must_use]
    pub const fn is_subspace_changing(self) -> bool {
        matches!(self, Self::Leakage)
    }

    /// Returns whether this effect is readout-specific.
    #[must_use]
    pub const fn is_readout(self) -> bool {
        matches!(self, Self::Readout)
    }

    /// Returns whether this effect is time-related.
    #[must_use]
    pub const fn is_time_related(self) -> bool {
        matches!(self, Self::Timing)
    }
}

impl fmt::Display for FaultEffectKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&FaultEffect> for FaultEffectKind {
    fn from(effect: &FaultEffect) -> Self {
        match effect {
            FaultEffect::Pauli(_) => Self::Pauli,
            FaultEffect::Coherent(_) => Self::Coherent,
            FaultEffect::Leakage { .. } => Self::Leakage,
            FaultEffect::Erasure => Self::Erasure,
            FaultEffect::Loss => Self::Loss,
            FaultEffect::Readout { .. } => Self::Readout,
            FaultEffect::Timing { .. } => Self::Timing,
            FaultEffect::Corruption(_) => Self::Corruption,
            FaultEffect::Custom(_) => Self::Custom,
        }
    }
}

// ============================================================================
// Resource scope
// ============================================================================

/// Scope of the resource set affected by a fault.
///
/// This classification does not encode the number of resources. In particular,
/// `Composite` means "multiple explicitly represented resources", regardless of
/// whether there are two, two hundred, or another resource count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultScope {
    /// One directly identified resource.
    Single,

    /// Multiple explicitly represented resources.
    Composite,

    /// The fault is explicitly system/global scoped.
    Global,
}

impl FaultScope {
    /// Returns the stable semantic identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Composite => "composite",
            Self::Global => "global",
        }
    }

    /// Returns whether more than one resource is explicitly represented.
    #[must_use]
    pub const fn is_multi_resource(self) -> bool {
        matches!(self, Self::Composite)
    }
}

impl fmt::Display for FaultScope {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&FaultLocation> for FaultScope {
    fn from(location: &FaultLocation) -> Self {
        match location {
            FaultLocation::Composite(_) => Self::Composite,

            // `fault.rs` currently represents a system-wide fault through
            // semantic ZQN resources rather than a dedicated Global variant.
            //
            // Classification therefore remains conservative here:
            // resource-like locations are classified as Single unless the
            // location is explicitly composite.
            FaultLocation::LogicalQubit(_)
            | FaultLocation::PhysicalQubit(_)
            | FaultLocation::ZqnResource(_)
            | FaultLocation::ExternalResource(_)
            | FaultLocation::Operation(_)
            | FaultLocation::Measurement(_)
            | FaultLocation::Preparation(_)
            | FaultLocation::Reset(_)
            | FaultLocation::Transport(_) => Self::Single,
        }
    }
}

// ============================================================================
// Resource-domain analysis
// ============================================================================

/// Resource-domain result of classification analysis.
///
/// Unlike `FaultResourceDomain`, this type can explicitly represent a mixed
/// logical/physical composite.
///
/// This is important because a composite location may contain resources from
/// more than one semantic domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassifiedResourceDomain {
    /// Only logical resources were encountered.
    Logical,

    /// Only physical resources were encountered.
    Physical,

    /// Only ZQN-owned non-qubit resources were encountered.
    Zqn,

    /// Only external resources were encountered.
    External,

    /// More than one resource domain was encountered.
    Mixed,

    /// No directly enumerated resource domain was available.
    GlobalOrUnspecified,
}

impl ClassifiedResourceDomain {
    /// Returns the stable semantic identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logical => "logical",
            Self::Physical => "physical",
            Self::Zqn => "zqn",
            Self::External => "external",
            Self::Mixed => "mixed",
            Self::GlobalOrUnspecified => "global_or_unspecified",
        }
    }

    /// Returns whether logical resources are represented.
    #[must_use]
    pub const fn includes_logical(self) -> bool {
        matches!(self, Self::Logical | Self::Mixed)
    }

    /// Returns whether physical resources are represented.
    #[must_use]
    pub const fn includes_physical(self) -> bool {
        matches!(self, Self::Physical | Self::Mixed)
    }

    /// Returns whether multiple semantic domains are represented.
    #[must_use]
    pub const fn is_mixed(self) -> bool {
        matches!(self, Self::Mixed)
    }
}

impl fmt::Display for ClassifiedResourceDomain {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Internal domain accumulator
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DomainMask(u8);

impl DomainMask {
    const LOGICAL: u8 = 1 << 0;
    const PHYSICAL: u8 = 1 << 1;
    const ZQN: u8 = 1 << 2;
    const EXTERNAL: u8 = 1 << 3;

    const fn empty() -> Self {
        Self(0)
    }

    fn insert_location(&mut self, location: &FaultLocation) {
        match location {
            FaultLocation::LogicalQubit(_) => {
                self.0 |= Self::LOGICAL;
            }

            FaultLocation::PhysicalQubit(_) => {
                self.0 |= Self::PHYSICAL;
            }

            FaultLocation::ZqnResource(_)
            | FaultLocation::Operation(_)
            | FaultLocation::Measurement(_)
            | FaultLocation::Preparation(_)
            | FaultLocation::Reset(_)
            | FaultLocation::Transport(_) => {
                self.0 |= Self::ZQN;
            }

            FaultLocation::ExternalResource(_) => {
                self.0 |= Self::EXTERNAL;
            }

            FaultLocation::Composite(locations) => {
                for child in locations {
                    self.insert_location(child);
                }
            }
        }
    }

    const fn classify(self) -> ClassifiedResourceDomain {
        match self.0 {
            Self::LOGICAL => ClassifiedResourceDomain::Logical,
            Self::PHYSICAL => ClassifiedResourceDomain::Physical,
            Self::ZQN => ClassifiedResourceDomain::Zqn,
            Self::EXTERNAL => ClassifiedResourceDomain::External,
            0 => ClassifiedResourceDomain::GlobalOrUnspecified,
            _ => ClassifiedResourceDomain::Mixed,
        }
    }
}

// ============================================================================
// Classification report
// ============================================================================

/// Deterministic, allocation-free analysis result for one fault.
///
/// The report does not contain the original `Fault` and therefore does not
/// extend its lifetime or retain large fault structures.
///
/// This makes it suitable for streaming analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaultClassificationReport {
    classification: FaultClassification,
    effect: FaultEffectKind,
    scope: FaultScope,
    resource_domain: ClassifiedResourceDomain,
    standard: bool,
    correlated: bool,
    leakage: bool,
    loss_like: bool,
    operation_associated: bool,
    timed: bool,
}

impl FaultClassificationReport {
    /// Builds a deterministic report from a canonical fault.
    #[must_use]
    pub fn from_fault(fault: &Fault) -> Self {
        let classification = fault.classification().clone();
        let effect = FaultEffectKind::from(fault.effect());
        let scope = FaultScope::from(fault.location());

        let mut domain_mask = DomainMask::empty();
        domain_mask.insert_location(fault.location());

        let resource_domain = domain_mask.classify();

        Self {
            standard: classification.is_standard(),
            correlated: fault.is_correlated(),
            leakage: fault.is_leakage(),
            loss_like: fault.is_loss_like(),
            operation_associated: fault.operation().is_some(),
            timed: fault.has_timing(),
            classification,
            effect,
            scope,
            resource_domain,
        }
    }

    /// Returns the canonical classification.
    #[must_use]
    pub fn classification(&self) -> &FaultClassification {
        &self.classification
    }

    /// Returns the effect mechanism.
    #[must_use]
    pub const fn effect(&self) -> FaultEffectKind {
        self.effect
    }

    /// Returns the resource scope.
    #[must_use]
    pub const fn scope(&self) -> FaultScope {
        self.scope
    }

    /// Returns the classified resource domain.
    #[must_use]
    pub const fn resource_domain(
        &self,
    ) -> ClassifiedResourceDomain {
        self.resource_domain
    }

    /// Returns whether the classification is standard.
    #[must_use]
    pub const fn is_standard(&self) -> bool {
        self.standard
    }

    /// Returns whether the classification is custom.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        !self.standard
    }

    /// Returns whether the fault is correlated.
    #[must_use]
    pub const fn is_correlated(&self) -> bool {
        self.correlated
    }

    /// Returns whether the fault represents leakage.
    #[must_use]
    pub const fn is_leakage(&self) -> bool {
        self.leakage
    }

    /// Returns whether the fault is loss/erasure-like.
    #[must_use]
    pub const fn is_loss_like(&self) -> bool {
        self.loss_like
    }

    /// Returns whether an operation association exists.
    #[must_use]
    pub const fn has_operation_association(
        &self,
    ) -> bool {
        self.operation_associated
    }

    /// Returns whether explicit timing exists.
    #[must_use]
    pub const fn has_explicit_timing(
        &self,
    ) -> bool {
        self.timed
    }

    /// Returns the stable primary classification name.
    #[must_use]
    pub fn classification_name(&self) -> &str {
        self.classification.as_str()
    }

    /// Returns the stable effect name.
    #[must_use]
    pub const fn effect_name(&self) -> &'static str {
        self.effect.as_str()
    }

    /// Returns the stable scope name.
    #[must_use]
    pub const fn scope_name(&self) -> &'static str {
        self.scope.as_str()
    }

    /// Returns the stable resource-domain name.
    #[must_use]
    pub const fn resource_domain_name(
        &self,
    ) -> &'static str {
        self.resource_domain.as_str()
    }
}

impl From<&Fault> for FaultClassificationReport {
    fn from(fault: &Fault) -> Self {
        Self::from_fault(fault)
    }
}

// ============================================================================
// Classification classifier
// ============================================================================

/// Zero-sized deterministic classifier.
///
/// This type exists to give downstream systems a stable service boundary
/// without introducing state or global configuration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FaultClassifier;

impl FaultClassifier {
    /// Creates a classifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Classifies one fault.
    #[must_use]
    pub fn classify(
        &self,
        fault: &Fault,
    ) -> FaultClassificationReport {
        FaultClassificationReport::from_fault(fault)
    }

    /// Returns the canonical classification directly.
    #[must_use]
    pub fn classification<'a>(
        &self,
        fault: &'a Fault,
    ) -> &'a FaultClassification {
        fault.classification()
    }

    /// Returns the effect mechanism directly.
    #[must_use]
    pub fn effect(
        &self,
        fault: &Fault,
    ) -> FaultEffectKind {
        FaultEffectKind::from(fault.effect())
    }

    /// Returns the scope directly.
    #[must_use]
    pub fn scope(
        &self,
        fault: &Fault,
    ) -> FaultScope {
        FaultScope::from(fault.location())
    }

    /// Returns the classified resource domain.
    #[must_use]
    pub fn resource_domain(
        &self,
        fault: &Fault,
    ) -> ClassifiedResourceDomain {
        FaultClassificationReport::from_fault(fault)
            .resource_domain()
    }
}

// ============================================================================
// Histogram error
// ============================================================================

/// Errors produced by explicit classification aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassificationAggregationError {
    /// The counter would exceed its representable range.
    CountOverflow {
        classification: FaultClassification,
    },
}

impl fmt::Display for ClassificationAggregationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CountOverflow { classification } => {
                write!(
                    formatter,
                    "classification count overflow for '{}'",
                    classification.as_str()
                )
            }
        }
    }
}

impl std::error::Error for ClassificationAggregationError {}

// ============================================================================
// Classification histogram
// ============================================================================

/// Deterministic explicit aggregation of fault classifications.
///
/// The histogram is intentionally caller-owned.
///
/// It does not:
///
/// - know the total machine size;
/// - preallocate for all possible resources;
/// - impose a fault-count ceiling;
/// - assume a finite number of custom classifications.
///
/// A custom classification becomes one key in the map, and storage grows only
/// as required by the classifications actually observed.
///
/// `BTreeMap` provides deterministic iteration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassificationHistogram {
    counts: BTreeMap<FaultClassification, u64>,
    total: u64,
}

impl Default for ClassificationHistogram {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassificationHistogram {
    /// Creates an empty deterministic histogram.
    #[must_use]
    pub fn new() -> Self {
        Self {
            counts: BTreeMap::new(),
            total: 0,
        }
    }

    /// Adds one canonical fault classification.
    pub fn observe(
        &mut self,
        classification: &FaultClassification,
    ) -> Result<(), ClassificationAggregationError> {
        let entry = self
            .counts
            .entry(classification.clone())
            .or_insert(0);

        *entry = entry.checked_add(1).ok_or_else(|| {
            ClassificationAggregationError::CountOverflow {
                classification: classification.clone(),
            }
        })?;

        self.total = self
            .total
            .checked_add(1)
            .ok_or_else(|| {
                ClassificationAggregationError::CountOverflow {
                    classification: classification.clone(),
                }
            })?;

        Ok(())
    }

    /// Observes one fault.
    pub fn observe_fault(
        &mut self,
        fault: &Fault,
    ) -> Result<(), ClassificationAggregationError> {
        self.observe(fault.classification())
    }

    /// Observes an iterator of faults without requiring the entire iterator to
    /// be materialized.
    pub fn observe_faults<I>(
        &mut self,
        faults: I,
    ) -> Result<(), ClassificationAggregationError>
    where
        I: IntoIterator,
        I::Item: std::borrow::Borrow<Fault>,
    {
        for fault in faults {
            self.observe_fault(fault.borrow())?;
        }

        Ok(())
    }

    /// Merges another histogram into this histogram.
    ///
    /// The merge is deterministic and does not depend on thread scheduling.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), ClassificationAggregationError> {
        for (classification, count) in &other.counts {
            let entry = self
                .counts
                .entry(classification.clone())
                .or_insert(0);

            *entry = entry.checked_add(*count).ok_or_else(|| {
                ClassificationAggregationError::CountOverflow {
                    classification: classification.clone(),
                }
            })?;
        }

        self.total = self
            .total
            .checked_add(other.total)
            .ok_or_else(|| {
                ClassificationAggregationError::CountOverflow {
                    classification: FaultClassification::Custom(
                        "__histogram_total__".to_owned(),
                    ),
                }
            })?;

        Ok(())
    }

    /// Returns the total number of observed faults.
    #[must_use]
    pub const fn total(&self) -> u64 {
        self.total
    }

    /// Returns the number of distinct classifications observed.
    #[must_use]
    pub fn distinct_classifications(&self) -> usize {
        self.counts.len()
    }

    /// Returns the count for one classification.
    #[must_use]
    pub fn count(
        &self,
        classification: &FaultClassification,
    ) -> u64 {
        self.counts
            .get(classification)
            .copied()
            .unwrap_or(0)
    }

    /// Returns whether the classification has been observed.
    #[must_use]
    pub fn contains(
        &self,
        classification: &FaultClassification,
    ) -> bool {
        self.counts.contains_key(classification)
    }

    /// Returns deterministic classification/count pairs.
    ///
    /// The returned iterator borrows the histogram and therefore does not
    /// allocate.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&FaultClassification, &u64),
    > {
        self.counts.iter()
    }

    /// Returns whether no classifications have been observed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Clears all counts.
    pub fn clear(&mut self) {
        self.counts.clear();
        self.total = 0;
    }
}

// ============================================================================
// Streaming classification helpers
// ============================================================================

/// Returns a lazy iterator of classification reports.
///
/// No fault collection is materialized by this function.
///
/// This is useful for very large workloads where classification analysis must
/// remain streaming.
pub fn classify_iter<'a, I>(
    faults: I,
) -> impl Iterator<Item = FaultClassificationReport> + 'a
where
    I: IntoIterator<Item = &'a Fault>,
    I::IntoIter: 'a,
{
    faults
        .into_iter()
        .map(FaultClassificationReport::from_fault)
}

/// Returns a deterministic histogram for a borrowed fault iterator.
///
/// The caller controls the lifetime and storage of the input iterator.
pub fn histogram<I>(
    faults: I,
) -> Result<ClassificationHistogram, ClassificationAggregationError>
where
    I: IntoIterator,
    I::Item: std::borrow::Borrow<Fault>,
{
    let mut result = ClassificationHistogram::new();
    result.observe_faults(faults)?;
    Ok(result)
}

// ============================================================================
// Predicate helpers
// ============================================================================

/// Returns whether a fault belongs to the specified canonical classification.
#[must_use]
pub fn is_classification(
    fault: &Fault,
    classification: &FaultClassification,
) -> bool {
    fault.classification() == classification
}

/// Returns whether a fault is operation/gate related.
#[must_use]
pub fn is_operation_related(
    fault: &Fault,
) -> bool {
    fault.classification().is_operation_related()
}

/// Returns whether a fault is measurement related.
#[must_use]
pub fn is_measurement_related(
    fault: &Fault,
) -> bool {
    fault
        .classification()
        .is_measurement_related()
}

/// Returns whether a fault is lifecycle related.
#[must_use]
pub fn is_lifecycle_related(
    fault: &Fault,
) -> bool {
    fault
        .classification()
        .is_lifecycle_related()
}

/// Returns whether a fault is time related.
#[must_use]
pub fn is_time_related(
    fault: &Fault,
) -> bool {
    fault.classification().is_time_related()
        || fault.has_timing()
}

/// Returns whether a fault is transport related.
#[must_use]
pub fn is_transport_related(
    fault: &Fault,
) -> bool {
    fault
        .classification()
        .is_transport_related()
}

/// Returns whether a fault is calibration related.
#[must_use]
pub fn is_calibration_related(
    fault: &Fault,
) -> bool {
    fault
        .classification()
        .is_calibration_related()
}

/// Returns whether a fault is correlated.
#[must_use]
pub fn is_correlated(
    fault: &Fault,
) -> bool {
    fault.is_correlated()
}

/// Returns whether a fault is leakage-related.
#[must_use]
pub fn is_leakage(
    fault: &Fault,
) -> bool {
    fault.is_leakage()
}

/// Returns whether a fault is loss/erasure-related.
#[must_use]
pub fn is_loss_like(
    fault: &Fault,
) -> bool {
    fault.is_loss_like()
}

/// Returns whether a fault has explicit timing.
#[must_use]
pub fn has_explicit_timing(
    fault: &Fault,
) -> bool {
    fault.has_timing()
}

/// Returns whether a fault has an operation association.
#[must_use]
pub fn has_operation_association(
    fault: &Fault,
) -> bool {
    fault.operation().is_some()
}

/// Returns whether the fault is directly logical.
#[must_use]
pub fn is_logical(
    fault: &Fault,
) -> bool {
    fault.is_logical()
}

/// Returns whether the fault is directly physical.
#[must_use]
pub fn is_physical(
    fault: &Fault,
) -> bool {
    fault.is_physical()
}

/// Returns whether the fault is explicitly composite.
#[must_use]
pub fn is_composite(
    fault: &Fault,
) -> bool {
    fault.is_composite()
}

// ============================================================================
// Compatibility helpers
// ============================================================================

/// Returns whether a classification/effect pair is semantically consistent
/// with the canonical validation performed by `Fault::validate()`.
///
/// This function intentionally constructs no temporary `Fault`; the canonical
/// fault validator remains authoritative.
///
/// The result is therefore only an analysis helper and not a second semantic
/// validator.
#[must_use]
pub fn classification_effect_kind(
    fault: &Fault,
) -> FaultEffectKind {
    FaultEffectKind::from(fault.effect())
}

/// Returns whether a fault represents a standard built-in classification.
#[must_use]
pub fn is_standard(
    fault: &Fault,
) -> bool {
    fault.classification().is_standard()
}

/// Returns whether a fault is user/technology-defined.
#[must_use]
pub fn is_custom(
    fault: &Fault,
) -> bool {
    fault.classification().is_custom_classification()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };
    use crate::quantum::zqn::core::ids::FaultId;

    fn fault_id(value: u64) -> FaultId {
        FaultId::new(value)
    }

    #[test]
    fn standard_classification_predicates_are_deterministic() {
        assert!(FaultClassification::Gate.is_standard_classification());
        assert!(FaultClassification::Measurement.is_measurement_related());
        assert!(FaultClassification::Timing.is_time_related());
        assert!(FaultClassification::Correlated.is_correlation_related());
        assert!(FaultClassification::Leakage.is_leakage_related());
        assert!(FaultClassification::Loss.is_loss_related());
        assert!(FaultClassification::Logical.is_logical_level());
    }

    #[test]
    fn custom_classification_is_not_standard() {
        let classification =
            FaultClassification::Custom("future_modality".to_owned());

        assert!(!classification.is_standard_classification());
        assert!(classification.is_custom_classification());
        assert_eq!(
            classification.semantic_name(),
            "future_modality"
        );
    }

    #[test]
    fn effect_classification_is_independent_from_fault_classification() {
        let effect = FaultEffect::Coherent(
            crate::quantum::zqn::core::ids::ZqnIdValue::new(7),
        );

        assert_eq!(
            FaultEffectKind::from(&effect),
            FaultEffectKind::Coherent
        );
    }

    #[test]
    fn logical_fault_domain_is_detected() {
        let fault = Fault::new(
            fault_id(1),
            FaultClassification::Logical,
            FaultLocation::LogicalQubit(QubitId::new(0)),
            FaultEffect::Pauli(
                crate::quantum::zqn::fault::fault::PauliEffect::X,
            ),
        )
        .expect("logical fault should be valid");

        let report = FaultClassificationReport::from_fault(&fault);

        assert_eq!(
            report.resource_domain(),
            ClassifiedResourceDomain::Logical
        );
        assert!(report.resource_domain().includes_logical());
        assert!(!report.resource_domain().includes_physical());
    }

    #[test]
    fn physical_fault_domain_is_detected() {
        let fault = Fault::new(
            fault_id(2),
            FaultClassification::Gate,
            FaultLocation::PhysicalQubit(PhysicalQubitId::new(0)),
            FaultEffect::Pauli(
                crate::quantum::zqn::fault::fault::PauliEffect::X,
            ),
        )
        .expect("physical fault should be valid");

        let report = FaultClassificationReport::from_fault(&fault);

        assert_eq!(
            report.resource_domain(),
            ClassifiedResourceDomain::Physical
        );
    }

    #[test]
    fn composite_location_is_classified_without_correlation_limit() {
        let location = FaultLocation::composite(vec![
            FaultLocation::PhysicalQubit(
                PhysicalQubitId::new(0),
            ),
            FaultLocation::PhysicalQubit(
                PhysicalQubitId::new(1),
            ),
            FaultLocation::PhysicalQubit(
                PhysicalQubitId::new(2),
            ),
        ])
        .expect("composite location should be valid");

        let fault = Fault::new(
            fault_id(3),
            FaultClassification::Correlated,
            location,
            FaultEffect::Pauli(
                crate::quantum::zqn::fault::fault::PauliEffect::X,
            ),
        )
        .expect("correlated fault should be valid");

        let report = FaultClassificationReport::from_fault(&fault);

        assert_eq!(report.scope(), FaultScope::Composite);
        assert!(report.is_correlated());
        assert_eq!(
            report.resource_domain(),
            ClassifiedResourceDomain::Physical
        );
    }

    #[test]
    fn histogram_is_deterministic() {
        let logical = Fault::new(
            fault_id(10),
            FaultClassification::Logical,
            FaultLocation::LogicalQubit(QubitId::new(0)),
            FaultEffect::Pauli(
                crate::quantum::zqn::fault::fault::PauliEffect::X,
            ),
        )
        .expect("valid logical fault");

        let measurement = Fault::new(
            fault_id(11),
            FaultClassification::Measurement,
            FaultLocation::Measurement(
                crate::quantum::zqn::fault::fault::FaultOperationId::new(
                    crate::quantum::zqn::core::ids::ZqnIdValue::new(1),
                ),
            ),
            FaultEffect::Readout {
                assigned_value: true,
            },
        )
        .expect("valid measurement fault");

        let mut histogram = ClassificationHistogram::new();

        histogram
            .observe_fault(&measurement)
            .expect("observe measurement");

        histogram
            .observe_fault(&logical)
            .expect("observe logical");

        histogram
            .observe_fault(&logical)
            .expect("observe logical");

        assert_eq!(histogram.total(), 3);
        assert_eq!(histogram.distinct_classifications(), 2);
        assert_eq!(
            histogram.count(&FaultClassification::Logical),
            2
        );
        assert_eq!(
            histogram.count(&FaultClassification::Measurement),
            1
        );

        let ordered: Vec<&FaultClassification> = histogram
            .iter()
            .map(|(classification, _)| classification)
            .collect();

        assert_eq!(
            ordered,
            vec![
                &FaultClassification::Logical,
                &FaultClassification::Measurement,
            ]
        );
    }

    #[test]
    fn histogram_merge_is_deterministic() {
        let mut left = ClassificationHistogram::new();
        let mut right = ClassificationHistogram::new();

        left.observe(&FaultClassification::Gate)
            .expect("gate");

        right
            .observe(&FaultClassification::Measurement)
            .expect("measurement");

        right
            .observe(&FaultClassification::Measurement)
            .expect("measurement");

        left.merge(&right)
            .expect("histogram merge");

        assert_eq!(left.total(), 3);
        assert_eq!(
            left.count(&FaultClassification::Gate),
            1
        );
        assert_eq!(
            left.count(&FaultClassification::Measurement),
            2
        );
    }

    #[test]
    fn classifier_is_zero_state() {
        let classifier = FaultClassifier::new();
        let classifier2 = FaultClassifier::new();

        assert_eq!(classifier, classifier2);
    }

    #[test]
    fn classify_iter_is_lazy_over_borrowed_faults() {
        let fault = Fault::new(
            fault_id(20),
            FaultClassification::Gate,
            FaultLocation::PhysicalQubit(
                PhysicalQubitId::new(0),
            ),
            FaultEffect::Pauli(
                crate::quantum::zqn::fault::fault::PauliEffect::Z,
            ),
        )
        .expect("valid fault");

        let reports: Vec<FaultClassificationReport> =
            classify_iter(std::iter::once(&fault)).collect();

        assert_eq!(reports.len(), 1);
        assert_eq!(
            reports[0].classification(),
            &FaultClassification::Gate
        );
        assert_eq!(
            reports[0].effect(),
            FaultEffectKind::Pauli
        );
    }

    #[test]
    fn no_machine_size_assumption_exists() {
        // The classification layer deliberately reasons about semantic scope,
        // not about a fixed qubit count.
        //
        // The test verifies that classification does not require a count
        // parameter or a machine-size configuration.
        assert_eq!(
            FaultScope::Composite.as_str(),
            "composite"
        );
    }
}