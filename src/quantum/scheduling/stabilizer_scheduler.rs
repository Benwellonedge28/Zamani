//! Zamani Quantum Scheduling — Stabilizer Scheduler Compatibility Facade.
//!
//! # Purpose
//!
//! This file is the compatibility boundary for stabilizer/QEC scheduling.
//!
//! It deliberately does NOT implement a second scheduling algorithm.
//!
//! The production architecture is:
//!
//! ```text
//! canonical quantum IR
//!        │
//!        ▼
//! quantum::error_correction
//!        │
//!        ▼
//! scheduling::qec
//!        │
//!        ▼
//! StabilizerScheduler                 <-- this facade
//!        │
//!        ▼
//! scheduling::adapters::qec
//!        │
//!        ▼
//! scheduling::ir
//!        │
//!        ├── dependency analysis
//!        ├── resource analysis
//!        ├── timing analysis
//!        └── constraints
//!        │
//!        ▼
//! scheduling::planners / algorithms
//!        │
//!        ▼
//! scheduling::verification
//!        │
//!        ▼
//! scheduled quantum program
//! ```
//!
//! # Architectural contract
//!
//! The canonical quantum IR answers:
//!
//! > What does the program mean?
//!
//! QEC planning answers:
//!
//! > Which fault-tolerance operations, rounds, measurements and
//! > dependencies are required?
//!
//! Routing answers:
//!
//! > Where can those operations execute?
//!
//! Generic scheduling answers:
//!
//! > When can those operations execute?
//!
//! Hardware answers:
//!
//! > Can the target execute the resulting operations?
//!
//! This file only connects the QEC scheduling domain to the generic scheduler.
//!
//! # Important change from the historical implementation
//!
//! The historical implementation directly mutated `ir_gen::IrFunction` and
//! emitted synthetic H/Measure/Reset operations plus comments representing
//! CNOTs. That representation was not an executable stabilizer schedule and
//! was coupled to an assumed surface-code interaction pattern.
//!
//! This implementation intentionally removes that behaviour.
//!
//! In particular, this file contains:
//!
//! - no synthetic quantum registers;
//! - no synthetic qubit names;
//! - no hard-coded gate sequence;
//! - no fixed stabilizer weight;
//! - no fixed number of ancillas;
//! - no fixed topology;
//! - no fixed number of rounds;
//! - no fixed number of qubits;
//! - no fixed hardware timing;
//! - no vendor assumptions;
//! - no scheduling time-slot matrix;
//! - no unsafe code.
//!
//! # Canonical qubit identity
//!
//! Qubit identity MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines a second qubit identity.
//!
//! A logical qubit is not silently converted to a physical qubit.
//!
//! Logical-to-physical mapping remains the responsibility of routing.
//!
//! # Scalability
//!
//! "Infinity" means that this file introduces no artificial machine-size
//! ceiling.
//!
//! Actual compilation is naturally bounded by:
//!
//! - available memory;
//! - address space;
//! - compilation time;
//! - explicit caller limits;
//! - target resources;
//! - operating-system/process limits.
//!
//! No `MAX_QUBITS`, `MAX_ROUNDS`, `MAX_STABILIZERS`, `MAX_ANCILLAS` or similar
//! architectural constant exists here.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Ownership
//!
//! The actual stabilizer model is owned by:
//!
//! ```text
//! crate::quantum::scheduling::qec::stabilizer
//! ```
//!
//! The QEC scheduling request contract is owned by:
//!
//! ```text
//! crate::quantum::scheduling::qec::interface
//! ```
//!
//! Scheduling algorithms are owned by:
//!
//! ```text
//! crate::quantum::scheduling::planners
//! crate::quantum::scheduling::algorithms
//! ```
//!
//! Verification is owned by:
//!
//! ```text
//! crate::quantum::scheduling::verification
//! ```
//!
//! This facade must remain thin.
//!
//! # Compatibility
//!
//! Historical callers used:
//!
//! ```text
//! StabilizerScheduler::new(patch_name, distance)
//! ```
//!
//! That API described a surface-code configuration rather than a complete
//! scheduling problem. It is therefore retained only as a compatibility
//! configuration object.
//!
//! New production code should construct a complete QEC scheduling request and
//! pass it to the generic scheduling pipeline.
//!
//! The legacy `IrFunction` mutation API is intentionally NOT reproduced here.
//! It would force this compatibility layer to continue generating an obsolete
//! second IR representation and would undermine the canonical
//! `quantum::ir` architecture.
//!
//! # Integration invariant
//!
//! A stabilizer scheduler request must eventually satisfy:
//!
//! ```text
//! QEC requirements
//!        =
//! semantic scheduling constraints
//! ```
//!
//! It must never mean:
//!
//! ```text
//! QEC requirements
//!        =
//! hard-coded hardware instructions
//! ```
//!
//! # No algorithm duplication
//!
//! Do not add ASAP, ALAP, list scheduling, RCPSP, critical-path scheduling or
//! resource scheduling logic here.
//!
//! Those algorithms already belong to the generic scheduling architecture.
//!
//! # No QEC implementation
//!
//! Do not add:
//!
//! - syndrome decoding;
//! - recovery selection;
//! - stabilizer synthesis;
//! - surface-code geometry;
//! - lattice generation;
//! - hardware routing;
//! - pulse generation.
//!
//! Such functionality belongs to the corresponding subsystem.
//!
//! # Thread safety
//!
//! `StabilizerScheduler` contains immutable configuration only.
//!
//! It owns no global mutable state, no hardware handles and no synchronization
//! primitives.
//!
//! Therefore instances can be freely constructed per compilation request.
//!
//! # Determinism
//!
//! This facade performs no unordered algorithmic selection.
//!
//! Any deterministic scheduling requirement is passed to the downstream
//! scheduling configuration/planner.
//!
//! # Serialization
//!
//! Serialization of the actual schedule belongs to the scheduling serialization
//! subsystem. This facade does not create a second schedule format.
//!
//! # Error handling
//!
//! Invalid stabilizer/QEC data must be rejected by the QEC scheduling model and
//! its adapter before generic scheduling.
//!
//! This facade does not silently manufacture missing qubits, ancillas,
//! durations, resources or dependencies.
//!
//! # Migration
//!
//! Historical:
//!
//! ```text
//! stabilizer_scheduler.rs
//!       │
//!       └── directly emits legacy IrInstruction
//! ```
//!
//! Production:
//!
//! ```text
//! stabilizer_scheduler.rs
//!       │
//!       └── compatibility/configuration facade
//!               │
//!               ▼
//!          QEC scheduling model
//!               │
//!               ▼
//!          generic scheduler
//! ```
//!
//! # Safety boundary
//!
//! Unsafe Rust is forbidden explicitly.
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! These attributes make the safety contract compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// ============================================================================
// Compatibility configuration
// ============================================================================

/// Compatibility configuration for a stabilizer scheduling request.
///
/// # Important
///
/// `patch_name` and `distance` are retained only because older Zamani callers
/// used them to describe a surface-code patch.
///
/// They do NOT define the scheduler's machine size, topology, number of
/// qubits, number of ancillas, timing or resource capacity.
///
/// New code should prefer a concrete QEC scheduling request from
/// `crate::quantum::scheduling::qec`.
///
/// # Scalability
///
/// No scheduler-wide limit is derived from `distance`.
///
/// A distance is merely QEC metadata supplied by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerScheduler {
    /// Optional human-readable QEC/patch identifier.
    ///
    /// This is metadata only. It is never interpreted as a topology.
    patch_name: String,

    /// Optional QEC code-distance metadata.
    ///
    /// `None` means that no code distance has been specified.
    ///
    /// This value is never converted into a fixed qubit count or stabilizer
    /// topology by this module.
    distance: Option<usize>,
}

impl StabilizerScheduler {
    /// Creates a compatibility stabilizer scheduler configuration.
    ///
    /// This constructor does not construct a schedule and does not allocate
    /// qubits.
    ///
    /// The historical two-argument API is preserved so existing callers can
    /// migrate incrementally.
    ///
    /// # Arguments
    ///
    /// * `patch_name` - caller-defined QEC metadata.
    /// * `distance` - caller-defined QEC code-distance metadata.
    ///
    /// # Panics
    ///
    /// Never.
    #[must_use]
    pub fn new(patch_name: impl Into<String>, distance: usize) -> Self {
        Self {
            patch_name: patch_name.into(),
            distance: Some(distance),
        }
    }

    /// Creates a scheduler configuration without code-distance metadata.
    ///
    /// This is the preferred constructor when the QEC implementation is not
    /// surface-code based.
    #[must_use]
    pub fn without_distance(patch_name: impl Into<String>) -> Self {
        Self {
            patch_name: patch_name.into(),
            distance: None,
        }
    }

    /// Returns the caller-supplied QEC/patch name.
    #[must_use]
    pub fn patch_name(&self) -> &str {
        &self.patch_name
    }

    /// Returns the optional caller-supplied code distance.
    ///
    /// This value is metadata only.
    #[must_use]
    pub const fn distance(&self) -> Option<usize> {
        self.distance
    }

    /// Returns true when a code distance was supplied.
    #[must_use]
    pub const fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    /// Returns immutable metadata describing this compatibility configuration.
    ///
    /// The returned object contains no hardware-derived assumptions.
    #[must_use]
    pub fn metadata(&self) -> StabilizerSchedulerMetadata<'_> {
        StabilizerSchedulerMetadata {
            patch_name: &self.patch_name,
            distance: self.distance,
        }
    }
}

// ============================================================================
// Metadata view
// ============================================================================

/// Immutable metadata view for a stabilizer scheduler configuration.
///
/// This type exists to make configuration inspection explicit without
/// exposing mutable scheduler state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StabilizerSchedulerMetadata<'a> {
    patch_name: &'a str,
    distance: Option<usize>,
}

impl<'a> StabilizerSchedulerMetadata<'a> {
    /// Returns the patch/QEC identifier.
    #[must_use]
    pub const fn patch_name(self) -> &'a str {
        self.patch_name
    }

    /// Returns the optional code distance.
    #[must_use]
    pub const fn distance(self) -> Option<usize> {
        self.distance
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for StabilizerScheduler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.distance {
            Some(distance) => write!(
                formatter,
                "StabilizerScheduler(patch={:?}, distance={})",
                self.patch_name, distance
            ),
            None => write!(
                formatter,
                "StabilizerScheduler(patch={:?}, distance=unspecified)",
                self.patch_name
            ),
        }
    }
}

// ============================================================================
// Validation
// ============================================================================

/// Errors validating compatibility stabilizer scheduler configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StabilizerSchedulerConfigError {
    /// The patch name is empty.
    EmptyPatchName,

    /// The patch name contains a prohibited NUL byte.
    InvalidPatchName,

    /// The caller supplied a distance of zero.
    ///
    /// A zero distance is not a meaningful code-distance value. The scheduler
    /// does not infer any other value.
    InvalidDistance,
}

impl fmt::Display for StabilizerSchedulerConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPatchName => {
                formatter.write_str("stabilizer scheduler patch name must not be empty")
            }
            Self::InvalidPatchName => {
                formatter.write_str(
                    "stabilizer scheduler patch name must not contain a NUL byte",
                )
            }
            Self::InvalidDistance => {
                formatter.write_str("stabilizer scheduler distance must be greater than zero")
            }
        }
    }
}

impl std::error::Error for StabilizerSchedulerConfigError {}

/// Result type for compatibility configuration validation.
pub type StabilizerSchedulerConfigResult<T> =
    Result<T, StabilizerSchedulerConfigError>;

impl StabilizerScheduler {
    /// Validates this compatibility configuration.
    ///
    /// Validation is intentionally limited to configuration integrity.
    ///
    /// It does not validate:
    ///
    /// - QEC topology;
    /// - stabilizer participation;
    /// - qubit availability;
    /// - hardware resources;
    /// - durations;
    /// - timing;
    /// - routing;
    /// - scheduling feasibility.
    ///
    /// Those belong to the corresponding subsystems.
    pub fn validate(&self) -> StabilizerSchedulerConfigResult<()> {
        if self.patch_name.is_empty() {
            return Err(StabilizerSchedulerConfigError::EmptyPatchName);
        }

        if self.patch_name.as_bytes().contains(&0) {
            return Err(StabilizerSchedulerConfigError::InvalidPatchName);
        }

        if matches!(self.distance, Some(0)) {
            return Err(StabilizerSchedulerConfigError::InvalidDistance);
        }

        Ok(())
    }
}

// ============================================================================
// Production integration contract
// ============================================================================

/// Documents the production scheduling boundary.
///
/// This trait deliberately contains no algorithm implementation.
///
/// A QEC adapter can use the configuration metadata while constructing the
/// actual `QecSchedulingRequest`.
///
/// The returned values are descriptive only.
///
/// This avoids coupling this compatibility facade to one concrete QEC code.
pub trait StabilizerSchedulingMetadataProvider {
    /// Returns optional patch metadata.
    fn patch_name(&self) -> &str;

    /// Returns optional code distance.
    fn distance(&self) -> Option<usize>;
}

impl StabilizerSchedulingMetadataProvider for StabilizerScheduler {
    fn patch_name(&self) -> &str {
        self.patch_name()
    }

    fn distance(&self) -> Option<usize> {
        self.distance()
    }
}

// ============================================================================
// Canonical qubit helper
// ============================================================================
//
// This helper intentionally accepts the canonical QubitId type.
//
// It does not create another QubitId.
//
// It exists so downstream compatibility/migration code can explicitly state
// that a stabilizer participant is a canonical Zamani IR qubit.
//
// Physical mapping remains outside this file.

use crate::quantum::ir::qubit::QubitId;

/// Returns the canonical qubit identity unchanged.
///
/// This function has no scheduling side effects and performs no mapping.
///
/// It is intentionally trivial: its purpose is to provide an explicit
/// compatibility boundary for callers migrating from legacy scheduler code.
///
/// # Important
///
/// A `QubitId` remains a logical/canonical identity. It must not be interpreted
/// as a physical hardware index by this module.
#[must_use]
pub const fn canonical_qubit(qubit: QubitId) -> QubitId {
    qubit
}

// ============================================================================
// Legacy API migration marker
// ============================================================================

/// Describes the old scheduling API and its replacement.
///
/// This is a zero-sized marker type used only for documentation and migration
/// tooling. It has no runtime behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StabilizerSchedulerMigration;

impl StabilizerSchedulerMigration {
    /// Returns the production migration path.
    ///
    /// ```text
    /// old:
    ///
    /// StabilizerScheduler
    ///     -> IrFunction
    ///     -> synthetic IrInstruction
    ///
    /// new:
    ///
    /// QEC model
    ///     -> QecSchedulingRequest
    ///     -> adapters::qec
    ///     -> generic scheduler
    ///     -> verified schedule
    /// ```
    #[must_use]
    pub const fn production_path() -> &'static str {
        "QEC model -> qec scheduling request -> adapters::qec -> generic scheduler -> verification"
    }
}

// ============================================================================
// Architectural guarantees
// ============================================================================

/// Returns whether this compatibility facade performs hardware scheduling.
///
/// This deliberately returns false.
///
/// The function exists as an explicit machine-readable architectural marker
/// for integration tests and documentation tooling.
#[must_use]
pub const fn owns_hardware_scheduling() -> bool {
    false
}

/// Returns whether this compatibility facade owns QEC decoding.
///
/// This deliberately returns false.
#[must_use]
pub const fn owns_qec_decoding() -> bool {
    false
}

/// Returns whether this compatibility facade owns logical-to-physical routing.
///
/// This deliberately returns false.
#[must_use]
pub const fn owns_routing() -> bool {
    false
}

/// Returns whether this compatibility facade owns scheduling algorithms.
///
/// This deliberately returns false.
///
/// ASAP, ALAP, list scheduling, critical-path scheduling, resource-constrained
/// scheduling and adaptive scheduling belong to the generic scheduling
/// subsystem.
#[must_use]
pub const fn owns_scheduling_algorithm() -> bool {
    false
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_preserves_metadata() {
        let scheduler = StabilizerScheduler::new("surface-code", 7);

        assert_eq!(scheduler.patch_name(), "surface-code");
        assert_eq!(scheduler.distance(), Some(7));
        assert!(scheduler.has_distance());
    }

    #[test]
    fn constructor_without_distance_is_generic() {
        let scheduler = StabilizerScheduler::without_distance("generic-qec");

        assert_eq!(scheduler.patch_name(), "generic-qec");
        assert_eq!(scheduler.distance(), None);
        assert!(!scheduler.has_distance());
    }

    #[test]
    fn valid_configuration_passes_validation() {
        let scheduler = StabilizerScheduler::new("surface-code", 3);

        assert!(scheduler.validate().is_ok());
    }

    #[test]
    fn empty_patch_name_is_rejected() {
        let scheduler = StabilizerScheduler::new("", 3);

        assert_eq!(
            scheduler.validate(),
            Err(StabilizerSchedulerConfigError::EmptyPatchName)
        );
    }

    #[test]
    fn nul_patch_name_is_rejected() {
        let scheduler = StabilizerScheduler::new("surface\0code", 3);

        assert_eq!(
            scheduler.validate(),
            Err(StabilizerSchedulerConfigError::InvalidPatchName)
        );
    }

    #[test]
    fn zero_distance_is_rejected() {
        let scheduler = StabilizerScheduler::new("surface-code", 0);

        assert_eq!(
            scheduler.validate(),
            Err(StabilizerSchedulerConfigError::InvalidDistance)
        );
    }

    #[test]
    fn metadata_view_is_immutable() {
        let scheduler = StabilizerScheduler::new("surface-code", 5);
        let metadata = scheduler.metadata();

        assert_eq!(metadata.patch_name(), "surface-code");
        assert_eq!(metadata.distance(), Some(5));
    }

    #[test]
    fn metadata_provider_matches_scheduler() {
        let scheduler = StabilizerScheduler::new("surface-code", 9);

        assert_eq!(
            StabilizerSchedulingMetadataProvider::patch_name(&scheduler),
            "surface-code"
        );

        assert_eq!(
            StabilizerSchedulingMetadataProvider::distance(&scheduler),
            Some(9)
        );
    }

    #[test]
    fn canonical_qubit_identity_is_unchanged() {
        let qubit = QubitId::new(42);

        assert_eq!(canonical_qubit(qubit), qubit);
    }

    #[test]
    fn migration_path_is_explicit() {
        assert!(StabilizerSchedulerMigration::production_path().contains(
            "generic scheduler"
        ));
    }

    #[test]
    fn facade_does_not_own_hardware_scheduling() {
        assert!(!owns_hardware_scheduling());
    }

    #[test]
    fn facade_does_not_own_qec_decoding() {
        assert!(!owns_qec_decoding());
    }

    #[test]
    fn facade_does_not_own_routing() {
        assert!(!owns_routing());
    }

    #[test]
    fn facade_does_not_own_algorithm() {
        assert!(!owns_scheduling_algorithm());
    }
}