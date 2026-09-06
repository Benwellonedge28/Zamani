//! Zamani Quantum Resilience — Degradation Model.
//!
//! Path:
//!     src/quantum/resilience/model/degradation.rs
//!
//! Purpose:
//!     Provides the provider-independent semantic model for graceful
//!     degradation of quantum execution resources and capabilities.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! Degradation answers:
//!
//!     "What capability or resource has become less available or less capable
//!      than its declared/reference state, and by how much?"
//!
//! It does NOT answer:
//!
//!     "Is the resource healthy?"
//!     "How severe is the incident?"
//!     "What caused the degradation?"
//!     "Should the system recover?"
//!     "Which recovery action should be selected?"
//!
//! Those concerns belong to:
//!
//!     model/health.rs
//!     model/severity.rs
//!     model/fault.rs
//!     model/incident.rs
//!     diagnosis/
//!     policy/
//!     planning/
//!     recovery/
//!
//! ============================================================================
//! CORE SEPARATION
//! ============================================================================
//!
//! The resilience architecture distinguishes:
//!
//!     Fault
//!         = something went wrong or was observed.
//!
//!     Health
//!         = current operational condition.
//!
//!     Severity
//!         = operational consequence.
//!
//!     Degradation
//!         = reduction/restriction of available capability.
//!
//!     Recovery
//!         = action taken to restore or preserve execution.
//!
//! Therefore:
//!
//!     fault != health != severity != degradation != recovery
//!
//! A resource can be:
//!
//!     Healthy + degraded relative to its nominal/reference capability
//!
//! or:
//!
//!     Degraded health + no measured capacity loss
//!
//! or:
//!
//!     Unavailable health + complete capability degradation.
//!
//! These dimensions MUST remain independently representable.
//!
//! ============================================================================
//! WRITE ONCE, SCALE EVERYWHERE
//! ============================================================================
//!
//! This module deliberately contains no:
//!
//!     MAX_QUBITS
//!     MAX_DEVICES
//!     MAX_BACKENDS
//!     MAX_CAPACITY
//!     MAX_DEGRADATION
//!     MAX_OPERATIONS
//!     provider-specific thresholds
//!     fixed percentages
//!     fixed machine sizes
//!
//! No resource identifier is assumed to be numeric, contiguous, or bounded by
//! this module.
//!
//! A degradation may therefore describe:
//!
//!     one qubit
//!     one logical qubit
//!     a logical register
//!     a coupling
//!     a gate family
//!     a control channel
//!     a QPU
//!     a backend
//!     a distributed quantum resource
//!
//! The size of the resource collection belongs to the resource/capability
//! layers, not this type.
//!
//! "Infinity" means that this semantic model introduces no artificial
//! machine-size ceiling. Actual execution remains bounded by the resources
//! available to the surrounding system.
//!
//! ============================================================================
//! NO QUANTUM-ID DUPLICATION
//! ============================================================================
//!
//! This file intentionally does not import:
//!
//!     quantum::ir::qubit::QubitId
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! That is deliberate.
//!
//! Degradation describes capability, not identity.
//!
//! When a degradation is associated with a quantum resource, the future
//! `model/resource.rs` layer owns that association and MUST use the canonical
//! identities from:
//!
//!     crate::quantum::ir::qubit
//!
//! No resilience-local qubit identity is permitted.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! The intended dependency direction is:
//!
//!     quantum::hardware
//!            │
//!            ├── discovered capabilities
//!            ├── observed capacity
//!            └── operational state
//!                    │
//!                    ▼
//!     quantum::zqn
//!            │
//!            └── fault/noise evidence
//!                    │
//!                    ▼
//!     quantum::resilience::model::degradation
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      diagnosis   policy    planning
//!                              │
//!                              ▼
//!                          adaptation
//!                              │
//!                              ▼
//!                           recovery
//!
//! `Degradation` is data.
//!
//! It performs no hardware access, telemetry collection, recovery, routing,
//! scheduling, optimization, QEC, or backend selection.
//!
//! ============================================================================
//! DESIGN PRINCIPLES
//! ============================================================================
//!
//! 1. No hard-coded machine limits.
//! 2. No provider-specific terminology.
//! 3. No implicit clock.
//! 4. No randomness.
//! 5. No global mutable state.
//! 6. No I/O.
//! 7. No authorization.
//! 8. No recovery side effects.
//! 9. Deterministic value semantics.
//! 10. Exact integer quantities where exactness matters.
//! 11. Explicit distinction between unavailable measurements and zero.
//! 12. No silent clamping of invalid input.
//! 13. No floating-point threshold decisions.
//! 14. Reference and available capacity are caller-defined.
//! 15. Degradation is descriptive; policy decides whether it is acceptable.
//!
//! ============================================================================
//! QUANTITATIVE MODEL
//! ============================================================================
//!
//! For a measurable capacity:
//!
//!     reference = capability available before degradation
//!     available = capability available after degradation
//!
//! The model records both values rather than storing a hard-coded percentage.
//!
//! This permits callers to derive:
//!
//!     lost = reference - available
//!
//! and, where meaningful:
//!
//!     remaining ratio = available / reference
//!
//! without this module imposing a policy threshold.
//!
//! Example:
//!
//!     reference = 1000 physical qubits
//!     available = 940 physical qubits
//!
//! means:
//!
//!     60 units of capacity were lost.
//!
//! It does NOT mean:
//!
//!     "execution must stop"
//!
//! because that is a policy question.
//!
//! ============================================================================
//! ZERO AND UNKNOWN
//! ============================================================================
//!
//! These states are deliberately different:
//!
//!     known zero
//!         = the caller explicitly knows that available capacity is zero.
//!
//!     unknown
//!         = the caller cannot establish a quantitative capacity.
//!
//! An unknown capacity MUST NOT be silently interpreted as zero.
//!
//! ============================================================================
//! COMPOSABILITY
//! ============================================================================
//!
//! A single `Degradation` describes one degradation observation.
//!
//! Larger systems may hold many such observations:
//!
//!     qubit capability
//!     gate capability
//!     connectivity
//!     timing
//!     readout
//!     logical capacity
//!     execution throughput
//!
//! Aggregation belongs to higher-level resource/capability models because the
//! meaning of combining independent dimensions is domain-specific.
//!
//! This prevents invalid arithmetic such as treating:
//!
//!     20 lost qubits
//!
//! as directly equivalent to:
//!
//!     20% slower execution.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Construction and access are deterministic.
//!
//! This module does not:
//!
//!     read the system clock;
//!     generate randomness;
//!     inspect environment variables;
//!     access hardware;
//!     perform network calls;
//!     inspect memory addresses;
//!     mutate global state.
//!
//! Equal inputs produce equal values.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! `Degradation` is descriptive data.
//!
//! Possessing a degradation value MUST NOT grant:
//!
//!     hardware access
//!     backend credentials
//!     migration authority
//!     recovery authority
//!     QEC authority
//!     filesystem access
//!     network access
//!
//! An external component may report a false degradation. Trust, provenance,
//! authentication and authorization therefore remain outside this module.
//!
//! ============================================================================
//! SERIALIZATION
//! ============================================================================
//!
//! This module does not define a wire format.
//!
//! The future:
//!
//!     resilience::serialization
//!
//! subsystem owns encoding, decoding, schema versions and migrations.
//!
//! The public representation here is intentionally composed of stable scalar
//! values suitable for deterministic serialization.
//!
//! ============================================================================
//! RUST COMPATIBILITY
//! ============================================================================
//!
//! Target:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021 edition
//!     stable Rust
//!     no nightly features
//!     no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::str::FromStr;

// ============================================================================
// Degradation dimension
// ============================================================================

/// A provider-independent identifier for the capability dimension that has
/// degraded.
///
/// The identifier is intentionally opaque to this module.
///
/// This avoids hard-coding a finite list of quantum resource dimensions while
/// still allowing callers to use stable machine-readable names.
///
/// Examples of caller-defined dimensions include:
///
///     physical_qubit_capacity
///     logical_qubit_capacity
///     connectivity
///     gate_availability
///     measurement_capacity
///     readout_capacity
///     execution_throughput
///     timing_capacity
///     control_channel_capacity
///
/// These examples are documentation only; they are not an enum and therefore
/// do not constrain future quantum architectures.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DegradationDimension(String);

impl DegradationDimension {
    /// Creates a degradation dimension from a non-empty identifier.
    ///
    /// The identifier is treated as opaque. This type does not impose
    /// provider-specific vocabulary.
    pub fn new<S>(value: S) -> Result<Self, InvalidDegradationDimension>
    where
        S: Into<String>,
    {
        let value = value.into();

        if value.is_empty() {
            return Err(InvalidDegradationDimension);
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the dimension and returns its identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for DegradationDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for DegradationDimension {
    type Err = InvalidDegradationDimension;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Error returned when a degradation dimension is structurally invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidDegradationDimension;

impl fmt::Display for InvalidDegradationDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("degradation dimension must not be empty")
    }
}

impl std::error::Error for InvalidDegradationDimension {}

// ============================================================================
// Capacity
// ============================================================================

/// Exact measurable capacity before and after degradation.
///
/// This type deliberately stores absolute quantities rather than a percentage.
///
/// That avoids:
///
/// - floating-point rounding;
/// - arbitrary percentage thresholds;
/// - hard-coded normalization assumptions.
///
/// The quantity is dimensionless. The associated
/// [`DegradationDimension`] determines its meaning.
///
/// For example:
///
///     dimension = "physical_qubit_capacity"
///     reference = 1000
///     available = 940
///
/// The same representation can be used for any caller-defined countable
/// capability.
///
/// `u128` is used because this is a value-model boundary rather than a
/// machine-size declaration. It introduces no quantum-specific maximum and
/// provides substantially more range than ordinary machine indexing.
///
/// The surrounding resource model remains responsible for mapping its native
/// resource quantities into this representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Capacity {
    reference: u128,
    available: u128,
}

impl Capacity {
    /// Creates an exact capacity measurement.
    ///
    /// `available` may not exceed `reference`.
    pub const fn new(
        reference: u128,
        available: u128,
    ) -> Result<Self, InvalidCapacity> {
        if available > reference {
            return Err(InvalidCapacity::AvailableExceedsReference);
        }

        Ok(Self {
            reference,
            available,
        })
    }

    /// Creates a capacity where the reference capacity is fully available.
    pub const fn full(reference: u128) -> Self {
        Self {
            reference,
            available: reference,
        }
    }

    /// Creates a capacity where no reference capacity remains available.
    pub const fn empty(reference: u128) -> Self {
        Self {
            reference,
            available: 0,
        }
    }

    /// Returns the reference capacity.
    #[must_use]
    pub const fn reference(self) -> u128 {
        self.reference
    }

    /// Returns the currently available capacity.
    #[must_use]
    pub const fn available(self) -> u128 {
        self.available
    }

    /// Returns the amount of capacity lost.
    #[must_use]
    pub const fn lost(self) -> u128 {
        self.reference - self.available
    }

    /// Returns whether the measured capacity is fully available.
    #[must_use]
    pub const fn is_full(self) -> bool {
        self.available == self.reference
    }

    /// Returns whether no capacity remains available.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.available == 0
    }

    /// Returns whether some, but not all, capacity remains available.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        self.available > 0 && self.available < self.reference
    }

    /// Returns whether a meaningful positive reference exists.
    ///
    /// A reference of zero is valid data but does not define a useful
    /// normalized capacity ratio.
    #[must_use]
    pub const fn has_positive_reference(self) -> bool {
        self.reference != 0
    }

    /// Returns the exact remaining ratio as a rational pair.
    ///
    /// The result is `(available, reference)`.
    ///
    /// No floating-point conversion is performed.
    ///
    /// Returns `None` when the reference capacity is zero.
    #[must_use]
    pub const fn remaining_ratio(self) -> Option<(u128, u128)> {
        if self.reference == 0 {
            None
        } else {
            Some((self.available, self.reference))
        }
    }

    /// Returns the exact lost ratio as a rational pair.
    ///
    /// The result is `(lost, reference)`.
    ///
    /// Returns `None` when the reference capacity is zero.
    #[must_use]
    pub const fn lost_ratio(self) -> Option<(u128, u128)> {
        if self.reference == 0 {
            None
        } else {
            Some((self.lost(), self.reference))
        }
    }

    /// Returns the greatest common divisor of two quantities.
    ///
    /// This is provided so callers can normalize the exact ratio without
    /// converting it to floating point.
    #[must_use]
    pub const fn gcd(mut left: u128, mut right: u128) -> u128 {
        while right != 0 {
            let remainder = left % right;
            left = right;
            right = remainder;
        }

        left
    }

    /// Returns the normalized remaining ratio.
    ///
    /// For example:
    ///
    ///     50 / 100 -> 1 / 2
    ///
    /// Returns `None` for a zero reference.
    #[must_use]
    pub const fn normalized_remaining_ratio(self) -> Option<(u128, u128)> {
        match self.remaining_ratio() {
            Some((numerator, denominator)) => {
                let divisor = Self::gcd(numerator, denominator);

                if divisor == 0 {
                    Some((0, 1))
                } else {
                    Some((numerator / divisor, denominator / divisor))
                }
            }
            None => None,
        }
    }

    /// Returns the normalized lost ratio.
    ///
    /// Returns `None` for a zero reference.
    #[must_use]
    pub const fn normalized_lost_ratio(self) -> Option<(u128, u128)> {
        match self.lost_ratio() {
            Some((numerator, denominator)) => {
                let divisor = Self::gcd(numerator, denominator);

                if divisor == 0 {
                    Some((0, 1))
                } else {
                    Some((numerator / divisor, denominator / divisor))
                }
            }
            None => None,
        }
    }
}

/// Error returned when a capacity measurement violates its structural
/// invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidCapacity {
    /// Available capacity cannot exceed reference capacity.
    AvailableExceedsReference,
}

impl fmt::Display for InvalidCapacity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AvailableExceedsReference => {
                formatter.write_str("available capacity exceeds reference capacity")
            }
        }
    }
}

impl std::error::Error for InvalidCapacity {}

// ============================================================================
// Degradation status
// ============================================================================

/// Semantic status of a degradation measurement.
///
/// This is deliberately independent of [`crate::quantum::resilience::model::health::HealthState`].
///
/// `NoDegradation` means the measured capability equals its reference.
///
/// `Partial` means some capability remains but the available quantity is below
/// the reference.
///
/// `Complete` means the measured capability is zero.
///
/// `Unknown` means no quantitative capacity is available for the observation.
///
/// The status is descriptive. It does not determine whether execution should
/// continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DegradationStatus {
    /// No degradation is present in the measured capability.
    NoDegradation,

    /// Some capability remains, but less than the reference capability.
    Partial,

    /// The measured capability is completely unavailable.
    Complete,

    /// Quantitative degradation could not be established.
    Unknown,
}

impl DegradationStatus {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDegradation => "none",
            Self::Partial => "partial",
            Self::Complete => "complete",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether degradation is known to be present.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Partial | Self::Complete)
    }

    /// Returns whether the measured capability is completely unavailable.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether no degradation is present.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::NoDegradation)
    }

    /// Returns whether the quantitative state is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for DegradationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DegradationStatus {
    type Err = InvalidDegradationStatus;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::NoDegradation),
            "partial" => Ok(Self::Partial),
            "complete" => Ok(Self::Complete),
            "unknown" => Ok(Self::Unknown),
            _ => Err(InvalidDegradationStatus),
        }
    }
}

/// Error returned for an unknown degradation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidDegradationStatus;

impl fmt::Display for InvalidDegradationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unknown degradation status")
    }
}

impl std::error::Error for InvalidDegradationStatus {}

// ============================================================================
// Degradation
// ============================================================================

/// Immutable provider-independent degradation observation.
///
/// A `Degradation` associates a caller-defined capability dimension with its
/// current/reference capacity.
///
/// The dimension remains opaque so this type can support future quantum
/// architectures without modifying the enum every time a new resource class
/// appears.
///
/// Examples:
///
///     physical qubit capacity
///     logical qubit capacity
///     connectivity capacity
///     measurement capacity
///     gate availability
///     execution throughput
///     control-channel capacity
///
/// The object is intentionally descriptive only.
///
/// It does not:
///
/// - change hardware;
/// - select another backend;
/// - reroute circuits;
/// - reschedule work;
/// - alter QEC;
/// - perform mitigation;
/// - retry execution;
/// - authorize recovery.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Degradation {
    dimension: DegradationDimension,
    capacity: Option<Capacity>,
}

impl Degradation {
    /// Creates a degradation observation with an exact capacity measurement.
    pub fn measured(
        dimension: DegradationDimension,
        reference: u128,
        available: u128,
    ) -> Result<Self, InvalidCapacity> {
        let capacity = Capacity::new(reference, available)?;

        Ok(Self {
            dimension,
            capacity: Some(capacity),
        })
    }

    /// Creates an observation for which quantitative capacity is unknown.
    ///
    /// Unknown is distinct from zero.
    pub const fn unknown(dimension: DegradationDimension) -> Self {
        Self {
            dimension,
            capacity: None,
        }
    }

    /// Creates a non-degraded observation with full reference capacity.
    pub const fn full(
        dimension: DegradationDimension,
        reference: u128,
    ) -> Self {
        Self {
            dimension,
            capacity: Some(Capacity::full(reference)),
        }
    }

    /// Creates a completely degraded observation.
    pub const fn empty(
        dimension: DegradationDimension,
        reference: u128,
    ) -> Self {
        Self {
            dimension,
            capacity: Some(Capacity::empty(reference)),
        }
    }

    /// Returns the capability dimension.
    #[must_use]
    pub fn dimension(&self) -> &DegradationDimension {
        &self.dimension
    }

    /// Returns the exact capacity measurement, if available.
    #[must_use]
    pub const fn capacity(&self) -> Option<Capacity> {
        self.capacity
    }

    /// Returns the current available capacity, if measured.
    #[must_use]
    pub const fn available(&self) -> Option<u128> {
        match self.capacity {
            Some(capacity) => Some(capacity.available()),
            None => None,
        }
    }

    /// Returns the reference capacity, if measured.
    #[must_use]
    pub const fn reference(&self) -> Option<u128> {
        match self.capacity {
            Some(capacity) => Some(capacity.reference()),
            None => None,
        }
    }

    /// Returns the amount of capacity lost, if measured.
    #[must_use]
    pub const fn lost(&self) -> Option<u128> {
        match self.capacity {
            Some(capacity) => Some(capacity.lost()),
            None => None,
        }
    }

    /// Returns the semantic degradation status.
    #[must_use]
    pub const fn status(&self) -> DegradationStatus {
        match self.capacity {
            None => DegradationStatus::Unknown,
            Some(capacity) if capacity.is_full() => {
                DegradationStatus::NoDegradation
            }
            Some(capacity) if capacity.is_empty() => {
                DegradationStatus::Complete
            }
            Some(_) => DegradationStatus::Partial,
        }
    }

    /// Returns whether the observation is quantitatively measured.
    #[must_use]
    pub const fn is_measured(&self) -> bool {
        self.capacity.is_some()
    }

    /// Returns whether this observation describes no degradation.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.status().is_none()
    }

    /// Returns whether this observation describes partial degradation.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self.status(), DegradationStatus::Partial)
    }

    /// Returns whether this observation describes complete degradation.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.status(), DegradationStatus::Complete)
    }

    /// Returns whether quantitative degradation is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.status().is_unknown()
    }

    /// Returns the exact remaining ratio.
    ///
    /// The result is `(available, reference)`.
    ///
    /// No floating-point approximation is introduced.
    #[must_use]
    pub const fn remaining_ratio(&self) -> Option<(u128, u128)> {
        match self.capacity {
            Some(capacity) => capacity.remaining_ratio(),
            None => None,
        }
    }

    /// Returns the exact lost ratio.
    ///
    /// The result is `(lost, reference)`.
    #[must_use]
    pub const fn lost_ratio(&self) -> Option<(u128, u128)> {
        match self.capacity {
            Some(capacity) => capacity.lost_ratio(),
            None => None,
        }
    }

    /// Returns the normalized remaining ratio.
    #[must_use]
    pub const fn normalized_remaining_ratio(&self) -> Option<(u128, u128)> {
        match self.capacity {
            Some(capacity) => capacity.normalized_remaining_ratio(),
            None => None,
        }
    }

    /// Returns the normalized lost ratio.
    #[must_use]
    pub const fn normalized_lost_ratio(&self) -> Option<(u128, u128)> {
        match self.capacity {
            Some(capacity) => capacity.normalized_lost_ratio(),
            None => None,
        }
    }

    /// Returns whether this observation represents at least as much loss as
    /// another observation of the same dimension.
    ///
    /// This comparison is intentionally only available when both observations
    /// have measurable capacities and the dimensions match.
    ///
    /// Cross-dimension comparisons are invalid because, for example, qubit
    /// capacity and timing capacity are not commensurable.
    #[must_use]
    pub const fn has_at_least_as_much_loss_as(
        &self,
        other: &Self,
    ) -> Option<bool> {
        if self.dimension.0 != other.dimension.0 {
            return None;
        }

        match (self.capacity, other.capacity) {
            (Some(left), Some(right)) => {
                // Compare:
                //
                // left.lost / left.reference
                //     >=
                // right.lost / right.reference
                //
                // without floating-point arithmetic.
                //
                // Cross multiplication is safe here because the product may
                // overflow u128 for sufficiently large caller values. We
                // therefore use checked multiplication and return `None`
                // rather than silently overflowing.
                let left_product = left.lost().checked_mul(right.reference());
                let right_product = right.lost().checked_mul(left.reference());

                match (left_product, right_product) {
                    (Some(left_value), Some(right_value)) => {
                        Some(left_value >= right_value)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Validates the internal semantic invariants.
    ///
    /// The constructor already enforces the capacity invariant. This method is
    /// intentionally provided as a stable boundary for callers and future
    /// model composition.
    pub const fn validate(&self) -> Result<(), InvalidDegradation> {
        if let Some(capacity) = self.capacity {
            if capacity.available() > capacity.reference() {
                return Err(InvalidDegradation::InvalidCapacity);
            }
        }

        if self.dimension.0.is_empty() {
            return Err(InvalidDegradation::EmptyDimension);
        }

        Ok(())
    }
}

/// Structural validation error for [`Degradation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidDegradation {
    /// The capability dimension is empty.
    EmptyDimension,

    /// The capacity invariant was violated.
    InvalidCapacity,
}

impl fmt::Display for InvalidDegradation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDimension => {
                formatter.write_str("degradation dimension must not be empty")
            }
            Self::InvalidCapacity => {
                formatter.write_str("degradation contains invalid capacity")
            }
        }
    }
}

impl std::error::Error for InvalidDegradation {}

// ============================================================================
// Standard conversions
// ============================================================================

impl TryFrom<(DegradationDimension, u128, u128)> for Degradation {
    type Error = InvalidCapacity;

    fn try_from(
        value: (DegradationDimension, u128, u128),
    ) -> Result<Self, Self::Error> {
        Self::measured(value.0, value.1, value.2)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dimension(name: &str) -> DegradationDimension {
        DegradationDimension::new(name).expect("test dimension must be valid")
    }

    #[test]
    fn dimension_rejects_empty_identifier() {
        assert!(DegradationDimension::new("").is_err());
    }

    #[test]
    fn dimension_preserves_identifier() {
        let value = dimension("physical_qubit_capacity");

        assert_eq!(value.as_str(), "physical_qubit_capacity");
        assert_eq!(value.to_string(), "physical_qubit_capacity");
    }

    #[test]
    fn dimension_parses() {
        let value: DegradationDimension = "connectivity".parse().unwrap();

        assert_eq!(value.as_str(), "connectivity");
    }

    #[test]
    fn capacity_accepts_full_capacity() {
        let capacity = Capacity::new(100, 100).unwrap();

        assert_eq!(capacity.reference(), 100);
        assert_eq!(capacity.available(), 100);
        assert_eq!(capacity.lost(), 0);
        assert!(capacity.is_full());
        assert!(!capacity.is_partial());
        assert!(!capacity.is_empty());
    }

    #[test]
    fn capacity_accepts_partial_capacity() {
        let capacity = Capacity::new(100, 75).unwrap();

        assert_eq!(capacity.lost(), 25);
        assert!(capacity.is_partial());
        assert_eq!(capacity.remaining_ratio(), Some((75, 100)));
        assert_eq!(capacity.lost_ratio(), Some((25, 100)));
    }

    #[test]
    fn capacity_accepts_zero_available_capacity() {
        let capacity = Capacity::new(100, 0).unwrap();

        assert_eq!(capacity.lost(), 100);
        assert!(capacity.is_empty());
        assert_eq!(capacity.remaining_ratio(), Some((0, 100)));
    }

    #[test]
    fn capacity_rejects_available_above_reference() {
        assert_eq!(
            Capacity::new(10, 11),
            Err(InvalidCapacity::AvailableExceedsReference)
        );
    }

    #[test]
    fn zero_reference_is_valid_but_has_no_ratio() {
        let capacity = Capacity::new(0, 0).unwrap();

        assert_eq!(capacity.lost(), 0);
        assert!(!capacity.has_positive_reference());
        assert_eq!(capacity.remaining_ratio(), None);
        assert_eq!(capacity.lost_ratio(), None);
    }

    #[test]
    fn normalized_ratios_are_exact() {
        let capacity = Capacity::new(100, 50).unwrap();

        assert_eq!(capacity.normalized_remaining_ratio(), Some((1, 2)));
        assert_eq!(capacity.normalized_lost_ratio(), Some((1, 2)));
    }

    #[test]
    fn normalized_ratio_handles_zero_numerator() {
        let capacity = Capacity::new(100, 0).unwrap();

        assert_eq!(capacity.normalized_remaining_ratio(), Some((0, 1)));
        assert_eq!(capacity.normalized_lost_ratio(), Some((1, 1)));
    }

    #[test]
    fn status_for_full_capacity_is_none() {
        let degradation = Degradation::full(dimension("capacity"), 100);

        assert_eq!(
            degradation.status(),
            DegradationStatus::NoDegradation
        );
        assert!(degradation.is_none());
        assert!(!degradation.is_partial());
        assert!(!degradation.is_complete());
    }

    #[test]
    fn status_for_partial_capacity_is_partial() {
        let degradation =
            Degradation::measured(dimension("capacity"), 100, 60).unwrap();

        assert_eq!(degradation.status(), DegradationStatus::Partial);
        assert!(degradation.is_partial());
        assert_eq!(degradation.available(), Some(60));
        assert_eq!(degradation.reference(), Some(100));
        assert_eq!(degradation.lost(), Some(40));
    }

    #[test]
    fn status_for_zero_capacity_is_complete() {
        let degradation = Degradation::empty(dimension("capacity"), 100);

        assert_eq!(degradation.status(), DegradationStatus::Complete);
        assert!(degradation.is_complete());
    }

    #[test]
    fn unknown_is_not_zero() {
        let degradation = Degradation::unknown(dimension("capacity"));

        assert!(degradation.is_unknown());
        assert!(!degradation.is_complete());
        assert_eq!(degradation.available(), None);
        assert_eq!(degradation.reference(), None);
    }

    #[test]
    fn ratios_are_available_from_degradation() {
        let degradation =
            Degradation::measured(dimension("capacity"), 1000, 940).unwrap();

        assert_eq!(degradation.remaining_ratio(), Some((940, 1000)));
        assert_eq!(degradation.lost_ratio(), Some((60, 1000)));

        assert_eq!(
            degradation.normalized_remaining_ratio(),
            Some((47, 50))
        );

        assert_eq!(
            degradation.normalized_lost_ratio(),
            Some((3, 50))
        );
    }

    #[test]
    fn same_dimension_loss_can_be_compared() {
        let greater =
            Degradation::measured(dimension("capacity"), 100, 50).unwrap();

        let lesser =
            Degradation::measured(dimension("capacity"), 100, 80).unwrap();

        assert_eq!(
            greater.has_at_least_as_much_loss_as(&lesser),
            Some(true)
        );

        assert_eq!(
            lesser.has_at_least_as_much_loss_as(&greater),
            Some(false)
        );
    }

    #[test]
    fn different_dimensions_are_not_comparable() {
        let qubits =
            Degradation::measured(dimension("qubit_capacity"), 100, 50)
                .unwrap();

        let timing =
            Degradation::measured(dimension("timing_capacity"), 100, 50)
                .unwrap();

        assert_eq!(
            qubits.has_at_least_as_much_loss_as(&timing),
            None
        );
    }

    #[test]
    fn unknown_measurements_are_not_comparable() {
        let known =
            Degradation::measured(dimension("capacity"), 100, 50).unwrap();

        let unknown = Degradation::unknown(dimension("capacity"));

        assert_eq!(
            known.has_at_least_as_much_loss_as(&unknown),
            None
        );
    }

    #[test]
    fn degradation_validation_succeeds_for_valid_values() {
        let degradation =
            Degradation::measured(dimension("capacity"), 100, 50).unwrap();

        assert_eq!(degradation.validate(), Ok(()));
    }

    #[test]
    fn status_text_is_stable() {
        assert_eq!(DegradationStatus::NoDegradation.as_str(), "none");
        assert_eq!(DegradationStatus::Partial.as_str(), "partial");
        assert_eq!(DegradationStatus::Complete.as_str(), "complete");
        assert_eq!(DegradationStatus::Unknown.as_str(), "unknown");
    }

    #[test]
    fn status_parsing_is_strict_and_deterministic() {
        assert_eq!(
            "none".parse::<DegradationStatus>(),
            Ok(DegradationStatus::NoDegradation)
        );

        assert_eq!(
            "partial".parse::<DegradationStatus>(),
            Ok(DegradationStatus::Partial)
        );

        assert_eq!(
            "complete".parse::<DegradationStatus>(),
            Ok(DegradationStatus::Complete)
        );

        assert_eq!(
            "unknown".parse::<DegradationStatus>(),
            Ok(DegradationStatus::Unknown)
        );

        assert!("Partial".parse::<DegradationStatus>().is_err());
        assert!("invalid".parse::<DegradationStatus>().is_err());
    }

    #[test]
    fn tuple_conversion_works() {
        let degradation = Degradation::try_from((
            dimension("physical_qubit_capacity"),
            1000_u128,
            900_u128,
        ))
        .unwrap();

        assert_eq!(degradation.lost(), Some(100));
    }

    #[test]
    fn very_large_capacity_does_not_use_fixed_machine_limits() {
        let reference = u128::MAX;
        let available = u128::MAX - 1;

        let degradation =
            Degradation::measured(dimension("arbitrary_capacity"), reference, available)
                .unwrap();

        assert_eq!(degradation.reference(), Some(reference));
        assert_eq!(degradation.available(), Some(available));
        assert_eq!(degradation.lost(), Some(1));
    }

    #[test]
    fn equality_is_value_based() {
        let first =
            Degradation::measured(dimension("capacity"), 100, 80).unwrap();

        let second =
            Degradation::measured(dimension("capacity"), 100, 80).unwrap();

        assert_eq!(first, second);
    }
}