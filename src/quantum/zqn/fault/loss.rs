//! Zamani Quantum Noise (ZQN) — Loss Faults.
//!
//! This module provides the specialized semantic API for realized loss faults.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "How do I construct, validate, inspect, and consume a canonical ZQN
//! > loss fault without duplicating the canonical fault model?"
//!
//! A loss fault represents the semantic loss of a quantum resource or
//! excitation from the model in which the computation is being represented.
//!
//! Loss is intentionally distinct from:
//!
//! - erasure;
//! - leakage;
//! - measurement error;
//! - reset error;
//! - a generic Pauli fault;
//! - a quantum channel;
//! - a probability distribution.
//!
//! The canonical realized fault remains [`Fault`] from `fault.rs`.
//!
//! This module is therefore a specialized facade/adapter over:
//!
//! ```text
//! crate::quantum::zqn::fault::fault::Fault
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - `LossFault`;
//! - loss-specific construction;
//! - loss-specific structural validation;
//! - loss-specific inspection;
//! - conversion between `LossFault` and the canonical `Fault`;
//! - loss-specific predicates;
//! - loss-specific documentation and invariants.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - `Fault`;
//! - `FaultId`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `FaultLocation`;
//! - `FaultEffect`;
//! - probability distributions;
//! - random-number generation;
//! - loss-rate estimation;
//! - noise-model generation;
//! - quantum channels;
//! - calibration;
//! - hardware APIs;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - serialization formats;
//! - execution;
//! - resource allocation.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical fault model
//!
//! The repository already defines:
//!
//! ```text
//! FaultClassification::Loss
//! FaultEffect::Loss
//! ```
//!
//! and provides canonical fault constructors.
//!
//! This module MUST use those definitions rather than introducing:
//!
//! ```text
//! LossEffect
//! LossEvent
//! LossClassification
//! ```
//!
//! as competing canonical representations.
//!
//! `LossFault` is a typed semantic view over the canonical `Fault`.
//!
//! # Canonical quantum identities
//!
//! Quantum-resource identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file MUST NOT define another logical or physical qubit identifier.
//!
//! A loss fault can therefore refer directly to:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! through the canonical [`FaultLocation`] representation.
//!
//! # Loss versus erasure
//!
//! ZQN deliberately distinguishes loss from erasure:
//!
//! ```text
//! Loss
//!     The physical/resource carrier is lost from the represented system.
//!
//! Erasure
//!     The resource/value is treated as erased/unknown while the semantic
//!     resource may remain represented.
//! ```
//!
//! A backend or noise model may map a physical loss into an erasure-like
//! observation, but that mapping belongs to the appropriate integration layer.
//!
//! `loss.rs` MUST NOT silently convert loss into erasure.
//!
//! # Loss versus leakage
//!
//! Loss and leakage are also distinct:
//!
//! ```text
//! Loss
//!     Resource/excitation leaves the represented system.
//!
//! Leakage
//!     Resource remains present but leaves the intended computational or
//!     modeled subspace.
//! ```
//!
//! A physical technology may have transitions between these phenomena, but
//! such modeling belongs to the noise/channel layer.
//!
//! # Loss versus probability
//!
//! A `LossFault` is a realized event.
//!
//! It therefore does NOT contain a probability.
//!
//! A loss probability/rate belongs to:
//!
//! ```text
//! probability
//! noise
//! characterization
//! calibration
//! ```
//!
//! For example:
//!
//! ```text
//! Loss probability
//!       │
//!       ▼
//! NoiseModel
//!       │
//!       ▼
//! realized LossFault
//! ```
//!
//! This prevents a realized event from being confused with the stochastic law
//! that generated it.
//!
//! # Write once, scale everywhere
//!
//! This module contains no machine-size constants.
//!
//! It contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_LOSS_EVENTS
//! MAX_LOST_QUBITS
//! MAX_CORRELATED_LOSS
//! MAX_DEVICE_SIZE
//! ```
//!
//! A single `LossFault` describes one loss event regardless of whether the
//! computation contains one resource or an arbitrarily large number of
//! resources.
//!
//! Large collections of loss faults MUST be represented by the caller using
//! streaming, batching, partitioning, distributed execution, or another
//! resource-appropriate representation.
//!
//! `LossFault` itself does not materialize a collection of events.
//!
//! Therefore:
//!
//! ```text
//! tiny machine
//!      │
//!      ▼
//! one LossFault
//!
//! large machine
//!      │
//!      ▼
//! streamed LossFault values
//! ```
//!
//! The semantic API is unchanged.
//!
//! "Infinity" means that this module introduces no artificial finite semantic
//! machine-size ceiling. Actual computation remains bounded by available
//! memory, CPU/GPU resources, storage, distributed capacity, runtime policy,
//! target capabilities, and execution limits.
//!
//! # Determinism
//!
//! This module performs no random sampling.
//!
//! It does not:
//!
//! - access a global RNG;
//! - access thread-local RNG state;
//! - read system time implicitly;
//! - inspect process IDs;
//! - inspect memory addresses;
//! - use global mutable state;
//! - depend on hash-map iteration order.
//!
//! A `LossFault` is deterministic once constructed.
//!
//! Stochastic loss generation belongs to the ZQN noise/sampling subsystem.
//!
//! # Resource safety
//!
//! Constructing a `LossFault` creates only the canonical fault object and any
//! explicitly supplied metadata/annotation.
//!
//! This module does not allocate based on machine size.
//!
//! Composite locations are owned by the canonical `FaultLocation` model.
//!
//! This module does not recursively expand composite resources.
//!
//! Callers processing untrusted data MUST apply the appropriate ZQN resource
//! policy before materializing arbitrarily large fault structures.
//!
//! # Numerical safety
//!
//! This module performs no floating-point arithmetic.
//!
//! A loss event does not store a loss probability.
//!
//! Probability/rate estimation remains owned by the probability,
//! characterization, calibration, and noise layers.
//!
//! # Serialization
//!
//! This module does not define a wire format.
//!
//! The canonical ZQN IO subsystem owns serialization.
//!
//! `LossFault` serializes through its underlying canonical [`Fault`] semantics
//! when the IO layer chooses to expose this specialized type.
//!
//! The serialized representation MUST preserve the canonical distinction:
//!
//! ```text
//! FaultClassification::Loss
//! FaultEffect::Loss
//! ```
//!
//! It MUST NOT silently serialize a loss as erasure or leakage.
//!
//! # Thread safety
//!
//! `LossFault` contains no global state or interior mutability.
//!
//! It is an ordinary value type and may be shared across threads when the
//! surrounding ownership model permits it.
//!
//! # Security
//!
//! A loss fault is data, not a capability.
//!
//! A `FaultId`, `QubitId`, or `PhysicalQubitId` does not grant:
//!
//! - hardware access;
//! - QPU access;
//! - calibration write access;
//! - execution permission;
//! - credentials;
//! - filesystem access;
//! - network access.
//!
//! Authorization belongs to the surrounding security/capability subsystem.
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
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ├── QubitId
//!          └── PhysicalQubitId
//!
//! zqn::core::ids
//!          │
//!          └── FaultId
//!
//! zqn::fault::fault
//!          │
//!          ├── Fault
//!          ├── FaultClassification
//!          ├── FaultLocation
//!          └── FaultEffect
//!                  │
//!                  ▼
//!          zqn::fault::loss
//!                  │
//!                  └── LossFault
//!                  │
//!          ┌───────┼────────┬───────────┐
//!          ▼       ▼        ▼           ▼
//!        noise    QEC    simulation   analysis
//!          │       │        │           │
//!          └───────┴────────┴───────────┘
//!                          │
//!                          ▼
//!                       runtime
//! ```
//!
//! # Integration with noise
//!
//! A noise model may generate a canonical `Fault` with:
//!
//! ```text
//! FaultClassification::Loss
//! FaultEffect::Loss
//! ```
//!
//! The loss-specific API can then validate/view it as `LossFault`.
//!
//! `loss.rs` does not own the stochastic generation process.
//!
//! # Integration with QEC
//!
//! QEC may consume `LossFault` or the underlying `Fault`.
//!
//! QEC remains responsible for:
//!
//! - syndrome extraction;
//! - decoder behavior;
//! - correction;
//! - code-specific loss/erasure handling;
//! - logical error analysis.
//!
//! `loss.rs` does not know which QEC code is being used.
//!
//! # Integration with simulation
//!
//! Simulation may consume the underlying `Fault` and apply technology-
//! appropriate state/resource semantics.
//!
//! This module does not mutate simulator state.
//!
//! # Integration with routing
//!
//! Routing may inspect the location of a loss fault or derived loss statistics.
//!
//! It remains responsible for placement decisions.
//!
//! # Integration with scheduling
//!
//! Scheduling may associate a loss fault with an operation or explicit timing
//! information already present in the canonical `Fault`.
//!
//! This module does not schedule anything.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may count and classify loss faults.
//!
//! Loss rates and confidence intervals are calculated by the appropriate
//! statistical/characterization layers rather than by this file.
//!
//! # Integration with hardware
//!
//! Hardware adapters may convert backend-specific observations into canonical
//! `FaultClassification::Loss` / `FaultEffect::Loss` faults.
//!
//! This module contains no vendor-specific code.
//!
//! # Integration with correlated faults
//!
//! `CorrelatedFault` remains owned by `correlated.rs`.
//!
//! A correlated loss population should be represented as a correlated
//! collection of canonical faults rather than by adding a fixed-size
//! `LossFault2`, `LossFault3`, or similar structure here.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it uses the canonical `Fault` representation;
//! 2. it uses the canonical `FaultClassification::Loss`;
//! 3. it uses the canonical `FaultEffect::Loss`;
//! 4. it uses canonical IR logical/physical qubit identities through
//!    `FaultLocation`;
//! 5. it introduces no competing qubit IDs;
//! 6. it introduces no competing fault representation;
//! 7. it contains no machine-size ceiling;
//! 8. it contains no probability representation;
//! 9. it contains no RNG;
//! 10. it contains no global mutable state;
//! 11. it provides fallible validation;
//! 12. it rejects non-loss faults when converting from `Fault`;
//! 13. it preserves timing and operation association;
//! 14. it does not silently convert loss to erasure or leakage;
//! 15. it requires no unsafe code;
//! 16. it requires no external dependency;
//! 17. it remains compatible with Rust 1.97/1.97.1;
//! 18. it can be integrated into noise, QEC, simulation, routing, scheduling,
//!     hardware and benchmarking without changing the canonical `Fault` model.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::FaultId;
use crate::quantum::zqn::fault::fault::{
    Fault,
    FaultClassification,
    FaultEffect,
    FaultLocation,
    FaultOperationId,
    FaultTiming,
};

// ============================================================================
// LossFault
// ============================================================================

/// Specialized validated view of a canonical ZQN loss fault.
///
/// `LossFault` does not replace [`Fault`].
///
/// It guarantees that the underlying canonical fault has:
///
/// ```text
/// classification == FaultClassification::Loss
/// effect         == FaultEffect::Loss
/// ```
///
/// This gives loss-aware consumers a type-level boundary without creating a
/// second canonical fault representation.
///
/// # Invariants
///
/// A valid `LossFault` always satisfies:
///
/// 1. classification is `Loss`;
/// 2. effect is `Loss`;
/// 3. the underlying fault is structurally valid;
/// 4. no hidden randomness exists;
/// 5. no hardware handle is contained;
/// 6. no probability is contained;
/// 7. the referenced resource identity remains owned by the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LossFault {
    fault: Fault,
}

impl LossFault {
    /// Creates a loss fault for a specified location.
    ///
    /// The supplied [`FaultId`] is caller-owned identity.
    ///
    /// This function does not allocate or register the ID globally.
    pub fn new(
        id: FaultId,
        location: FaultLocation,
    ) -> ZqnResult<Self> {
        let fault = Fault::new(
            id,
            FaultClassification::Loss,
            location,
            FaultEffect::Loss,
        )?;

        Self::from_fault(fault)
    }

    /// Creates a loss fault with explicit timing.
    ///
    /// The timing semantics remain those of the canonical `Fault`.
    pub fn with_timing(
        id: FaultId,
        location: FaultLocation,
        timing: FaultTiming,
    ) -> ZqnResult<Self> {
        let fault = Fault::with_timing(
            id,
            FaultClassification::Loss,
            location,
            FaultEffect::Loss,
            timing,
        )?;

        Self::from_fault(fault)
    }

    /// Creates a loss fault associated with a canonical fault operation.
    ///
    /// The operation identifier is intentionally the canonical ZQN fault
    /// operation association type rather than a vendor/backend operation ID.
    pub fn with_operation(
        id: FaultId,
        location: FaultLocation,
        operation: FaultOperationId,
    ) -> ZqnResult<Self> {
        let fault = Fault::with_operation(
            id,
            FaultClassification::Loss,
            location,
            FaultEffect::Loss,
            operation,
        )?;

        Self::from_fault(fault)
    }

    /// Creates a fully specified loss fault.
    ///
    /// This constructor preserves the complete canonical fault metadata while
    /// enforcing loss semantics.
    pub fn with_details(
        id: FaultId,
        location: FaultLocation,
        timing: FaultTiming,
        operation: Option<FaultOperationId>,
        annotation: Option<crate::quantum::zqn::fault::fault::FaultAnnotation>,
    ) -> ZqnResult<Self> {
        let fault = Fault::with_details(
            id,
            FaultClassification::Loss,
            location,
            FaultEffect::Loss,
            timing,
            operation,
            annotation,
        )?;

        Self::from_fault(fault)
    }

    /// Converts a canonical fault into a validated loss fault.
    ///
    /// This is the primary integration boundary for noise generators,
    /// hardware adapters, simulation, characterization, and QEC.
    ///
    /// A non-loss fault is rejected rather than silently reclassified.
    pub fn from_fault(fault: Fault) -> ZqnResult<Self> {
        if fault.classification() != &FaultClassification::Loss {
            return Err(ZqnError::invalid_fault(
                "cannot construct LossFault from a fault whose classification is not loss",
            ));
        }

        if fault.effect() != &FaultEffect::Loss {
            return Err(ZqnError::invalid_fault(
                "cannot construct LossFault from a fault whose effect is not loss",
            ));
        }

        Ok(Self { fault })
    }

    /// Returns a shared reference to the canonical fault.
    ///
    /// This is the preferred integration method for consumers that operate on
    /// the general ZQN fault abstraction.
    #[must_use]
    pub fn as_fault(&self) -> &Fault {
        &self.fault
    }

    /// Consumes this specialized view and returns the canonical fault.
    ///
    /// No information is discarded.
    #[must_use]
    pub fn into_fault(self) -> Fault {
        self.fault
    }

    /// Returns the canonical fault identity.
    #[must_use]
    pub const fn id(&self) -> FaultId {
        self.fault.id()
    }

    /// Returns the canonical loss classification.
    ///
    /// This is always `FaultClassification::Loss`.
    #[must_use]
    pub const fn classification(&self) -> FaultClassification {
        FaultClassification::Loss
    }

    /// Returns the canonical loss effect.
    ///
    /// This is always `FaultEffect::Loss`.
    #[must_use]
    pub const fn effect(&self) -> FaultEffect {
        FaultEffect::Loss
    }

    /// Returns the fault location.
    #[must_use]
    pub fn location(&self) -> &FaultLocation {
        self.fault.location()
    }

    /// Returns explicit fault timing.
    #[must_use]
    pub const fn timing(&self) -> FaultTiming {
        self.fault.timing()
    }

    /// Returns the associated operation, if one exists.
    #[must_use]
    pub const fn operation(&self) -> Option<FaultOperationId> {
        self.fault.operation()
    }

    /// Returns the optional annotation.
    #[must_use]
    pub fn annotation(
        &self,
    ) -> Option<&crate::quantum::zqn::fault::fault::FaultAnnotation> {
        self.fault.annotation()
    }

    /// Returns the logical qubit when this is a direct logical-qubit loss.
    ///
    /// Returns `None` for physical, composite, external, or other locations.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        self.fault.logical_qubit()
    }

    /// Returns the physical qubit when this is a direct physical-qubit loss.
    ///
    /// Returns `None` for logical, composite, external, or other locations.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        self.fault.physical_qubit()
    }

    /// Returns whether this loss directly affects a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit_loss(&self) -> bool {
        self.logical_qubit().is_some()
    }

    /// Returns whether this loss directly affects a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit_loss(&self) -> bool {
        self.physical_qubit().is_some()
    }

    /// Returns whether the loss is associated with an operation.
    #[must_use]
    pub const fn is_operation_associated(&self) -> bool {
        self.operation().is_some()
    }

    /// Returns whether explicit timing is attached.
    #[must_use]
    pub const fn has_timing(&self) -> bool {
        self.timing().is_specified()
    }

    /// Validates the specialized loss invariant.
    ///
    /// This re-validates the canonical fault and then verifies the two
    /// loss-specific semantic conditions.
    pub fn validate(&self) -> ZqnResult<()> {
        self.fault.validate()?;

        if self.fault.classification() != &FaultClassification::Loss {
            return Err(ZqnError::invalid_fault(
                "LossFault classification invariant violated",
            ));
        }

        if self.fault.effect() != &FaultEffect::Loss {
            return Err(ZqnError::invalid_fault(
                "LossFault effect invariant violated",
            ));
        }

        Ok(())
    }

    /// Returns the stable semantic category.
    ///
    /// Always returns `"loss"`.
    #[must_use]
    pub const fn category() -> &'static str {
        "loss"
    }

    /// Returns `true`.
    ///
    /// This method exists to make generic fault-analysis code able to inspect
    /// a loss-specific value without pattern matching on the canonical fault.
    #[must_use]
    pub const fn is_loss(&self) -> bool {
        true
    }

    /// Returns whether the loss represents a direct resource location.
    ///
    /// A direct resource is currently a logical or physical qubit.
    ///
    /// Composite and technology-specific resources remain valid loss
    /// locations, but are not classified as direct qubit losses by this helper.
    #[must_use]
    pub const fn is_direct_qubit_loss(&self) -> bool {
        self.logical_qubit().is_some() || self.physical_qubit().is_some()
    }

    /// Returns whether this loss is a composite-resource loss.
    #[must_use]
    pub fn is_composite_loss(&self) -> bool {
        matches!(self.location(), FaultLocation::Composite(_))
    }

    /// Returns whether this loss is associated with a ZQN-owned resource.
    #[must_use]
    pub fn is_zqn_resource_loss(&self) -> bool {
        matches!(self.location(), FaultLocation::ZqnResource(_))
    }

    /// Returns whether this loss is associated with an external resource.
    #[must_use]
    pub fn is_external_resource_loss(&self) -> bool {
        matches!(
            self.location(),
            FaultLocation::ExternalResource(_)
        )
    }
}

// ============================================================================
// Trait implementations
// ============================================================================

impl AsRef<Fault> for LossFault {
    fn as_ref(&self) -> &Fault {
        self.as_fault()
    }
}

impl From<LossFault> for Fault {
    fn from(value: LossFault) -> Self {
        value.into_fault()
    }
}

impl fmt::Display for LossFault {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "loss-fault:{}@{}",
            self.id(),
            self.location()
        )
    }
}

// ============================================================================
// Loss constructors for canonical qubit identities
// ============================================================================

/// Creates a logical-qubit loss using the canonical Quantum IR identity.
///
/// This helper exists only as ergonomic integration glue. It does not define
/// another qubit identity type.
pub fn logical_qubit_loss(
    id: FaultId,
    qubit: QubitId,
) -> ZqnResult<LossFault> {
    LossFault::new(
        id,
        FaultLocation::logical_qubit(qubit),
    )
}

/// Creates a physical-qubit loss using the canonical Quantum IR identity.
///
/// The physical identifier is the canonical
/// `crate::quantum::ir::qubit::PhysicalQubitId`.
pub fn physical_qubit_loss(
    id: FaultId,
    qubit: PhysicalQubitId,
) -> ZqnResult<LossFault> {
    LossFault::new(
        id,
        FaultLocation::physical_qubit(qubit),
    )
}

/// Creates a loss for an arbitrary canonical ZQN fault location.
///
/// This is the preferred generic constructor when the resource is not
/// necessarily a qubit.
pub fn resource_loss(
    id: FaultId,
    location: FaultLocation,
) -> ZqnResult<LossFault> {
    LossFault::new(id, location)
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Returns whether a canonical fault is exactly a loss fault.
///
/// This function intentionally checks both classification and effect.
///
/// Checking only classification would allow malformed canonical values to
/// cross a specialized integration boundary unnoticed.
#[must_use]
pub fn is_loss_fault(fault: &Fault) -> bool {
    fault.classification() == &FaultClassification::Loss
        && fault.effect() == &FaultEffect::Loss
}

/// Validates that a canonical fault can be viewed as a `LossFault`.
///
/// Unlike `is_loss_fault`, this function returns an explicit error describing
/// the failed semantic boundary.
pub fn validate_loss_fault(
    fault: &Fault,
) -> ZqnResult<()> {
    fault.validate()?;

    if fault.classification() != &FaultClassification::Loss {
        return Err(ZqnError::invalid_fault(
            "fault classification is not loss",
        ));
    }

    if fault.effect() != &FaultEffect::Loss {
        return Err(ZqnError::invalid_fault(
            "fault effect is not loss",
        ));
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fault_id(value: u64) -> FaultId {
        FaultId::new(value)
    }

    #[test]
    fn creates_logical_qubit_loss() {
        let fault = logical_qubit_loss(
            fault_id(1),
            QubitId::new(7),
        )
        .expect("logical loss should be valid");

        assert_eq!(
            fault.classification(),
            FaultClassification::Loss
        );
        assert_eq!(
            fault.effect(),
            FaultEffect::Loss
        );
        assert_eq!(
            fault.logical_qubit(),
            Some(QubitId::new(7))
        );
        assert!(fault.is_logical_qubit_loss());
        assert!(!fault.is_physical_qubit_loss());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn creates_physical_qubit_loss() {
        let fault = physical_qubit_loss(
            fault_id(2),
            PhysicalQubitId::new(13),
        )
        .expect("physical loss should be valid");

        assert_eq!(
            fault.classification(),
            FaultClassification::Loss
        );
        assert_eq!(
            fault.effect(),
            FaultEffect::Loss
        );
        assert_eq!(
            fault.physical_qubit(),
            Some(PhysicalQubitId::new(13))
        );
        assert!(fault.is_physical_qubit_loss());
        assert!(!fault.is_logical_qubit_loss());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn creates_generic_resource_loss() {
        let fault = resource_loss(
            fault_id(3),
            FaultLocation::zqn_resource(42),
        )
        .expect("ZQN resource loss should be valid");

        assert!(fault.is_zqn_resource_loss());
        assert!(!fault.is_direct_qubit_loss());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn creates_external_resource_loss() {
        let fault = resource_loss(
            fault_id(4),
            FaultLocation::external_resource(99),
        )
        .expect("external resource loss should be valid");

        assert!(fault.is_external_resource_loss());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn creates_composite_loss_without_fixed_arity() {
        let location = FaultLocation::Composite(vec![
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(7),
            ),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(31),
            ),
        ]);

        let fault = LossFault::new(
            fault_id(5),
            location,
        )
        .expect("composite loss should be valid");

        assert!(fault.is_composite_loss());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn rejects_non_loss_classification() {
        let fault = Fault::new(
            fault_id(6),
            FaultClassification::Gate,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultEffect::Loss,
        )
        .expect("canonical fault should be structurally valid");

        assert!(!is_loss_fault(&fault));
        assert!(LossFault::from_fault(fault).is_err());
    }

    #[test]
    fn rejects_non_loss_effect() {
        let fault = Fault::new(
            fault_id(7),
            FaultClassification::Loss,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultEffect::Erasure,
        )
        .expect("canonical fault should be structurally valid");

        assert!(!is_loss_fault(&fault));
        assert!(LossFault::from_fault(fault).is_err());
    }

    #[test]
    fn rejects_both_wrong() {
        let fault = Fault::new(
            fault_id(8),
            FaultClassification::Gate,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultEffect::Erasure,
        )
        .expect("canonical fault should be structurally valid");

        assert!(!is_loss_fault(&fault));
        assert!(validate_loss_fault(&fault).is_err());
    }

    #[test]
    fn preserves_timing() {
        let timing = FaultTiming::interval(100, 250)
            .expect("valid interval");

        let fault = LossFault::with_timing(
            fault_id(9),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(4),
            ),
            timing,
        )
        .expect("timed loss should be valid");

        assert_eq!(fault.timing(), timing);
        assert!(fault.has_timing());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn preserves_operation_association() {
        let operation = FaultOperationId::new(123);

        let fault = LossFault::with_operation(
            fault_id(10),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(4),
            ),
            operation,
        )
        .expect("operation-associated loss should be valid");

        assert_eq!(
            fault.operation(),
            Some(operation)
        );
        assert!(fault.is_operation_associated());
        assert!(fault.validate().is_ok());
    }

    #[test]
    fn round_trip_to_canonical_fault_preserves_semantics() {
        let original = LossFault::new(
            fault_id(11),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(8),
            ),
        )
        .expect("loss should be valid");

        let canonical = original.clone().into_fault();

        assert!(is_loss_fault(&canonical));

        let restored = LossFault::from_fault(canonical)
            .expect("canonical loss should restore");

        assert_eq!(original, restored);
    }

    #[test]
    fn as_ref_exposes_canonical_fault() {
        let loss = LossFault::new(
            fault_id(12),
            FaultLocation::logical_qubit(
                QubitId::new(2),
            ),
        )
        .expect("loss should be valid");

        let fault: &Fault = loss.as_ref();

        assert_eq!(
            fault.classification(),
            &FaultClassification::Loss
        );
        assert_eq!(
            fault.effect(),
            &FaultEffect::Loss
        );
    }

    #[test]
    fn canonical_predicate_requires_both_dimensions() {
        let classification_only = Fault::new(
            fault_id(13),
            FaultClassification::Loss,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultEffect::Erasure,
        )
        .expect("canonical fault should be structurally valid");

        let effect_only = Fault::new(
            fault_id(14),
            FaultClassification::Gate,
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
            FaultEffect::Loss,
        )
        .expect("canonical fault should be structurally valid");

        assert!(!is_loss_fault(&classification_only));
        assert!(!is_loss_fault(&effect_only));
    }

    #[test]
    fn no_machine_size_is_encoded() {
        // This test deliberately constructs identifiers with arbitrary values.
        // The loss abstraction does not impose a machine-size limit.
        let logical = LossFault::new(
            fault_id(u64::MAX),
            FaultLocation::logical_qubit(
                QubitId::new(usize::MAX),
            ),
        )
        .expect("identifier value itself is not a semantic machine-size limit");

        assert_eq!(
            logical.logical_qubit(),
            Some(QubitId::new(usize::MAX))
        );
        assert!(logical.validate().is_ok());
    }

    #[test]
    fn display_is_deterministic() {
        let loss = LossFault::new(
            fault_id(15),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(21),
            ),
        )
        .expect("loss should be valid");

        assert_eq!(
            loss.to_string(),
            "loss-fault:fault:15@p21"
        );
    }

    #[test]
    fn category_is_stable() {
        assert_eq!(
            LossFault::category(),
            "loss"
        );
    }

    #[test]
    fn loss_is_not_erasure() {
        let loss = LossFault::new(
            fault_id(16),
            FaultLocation::physical_qubit(
                PhysicalQubitId::new(1),
            ),
        )
        .expect("loss should be valid");

        assert_eq!(
            loss.effect(),
            FaultEffect::Loss
        );
        assert_ne!(
            loss.effect(),
            FaultEffect::Erasure
        );
    }
}