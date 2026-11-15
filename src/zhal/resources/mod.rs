
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Physical Hardware Resource Integration
//!
//! Path: `src/zhal/resources/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97 or later
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the composition root and stable public API for ZHAL's
//! physical hardware resource subsystem.
//!
//! This module integrates four independent resource authorities:
//!
//! 1. Physical resource observations.
//! 2. Provider-facing physical resource reservations.
//! 3. Physical reservation inspection and reconciliation.
//! 4. Provider-reported physical resource accounting.
//!
//! These components share the canonical hardware resource model,
//! identity contracts, provider identities and error contracts.
//!
//! # Architectural ownership
//!
//! ZHAL owns the hardware-facing resource integration boundary.
//!
//! ZHAL does NOT own:
//!
//! - Source-language resource requirements.
//! - Semantic resource analysis.
//! - Resource AST representations.
//! - Resource feasibility decisions.
//! - Physical hardware discovery.
//! - Provider registration or provider lifecycle.
//! - Runtime resource allocation.
//! - Runtime resource lease lifecycle.
//! - Runtime scheduling.
//! - Resource placement.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Hardware execution algorithms.
//!
//! These responsibilities remain with their existing
//! authoritative repository subsystems.
//!
//! # Canonical repository authorities
//!
//! Physical resource descriptions:
//!     crate::zhal::model::resource
//!
//! Physical resource identities:
//!     crate::zhal::identity
//!
//! Hardware provider identity:
//!     crate::stdlib::provider
//!
//! Hardware provider contracts:
//!     crate::zhal::providers::contract
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Source-level resource semantics:
//!     crate::frontend::semantic::resources
//!
//! Runtime resource allocation:
//!     crate::zur::resources
//!
//! Runtime resource lease lifecycle:
//!     crate::zur::resources::lease
//!
//! Runtime resource accounting:
//!     crate::zur::resources::accounting
//!
//! Quantum-specific hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! # Fundamental invariants
//!
//! Requirement != Capability.
//! Capability != Availability.
//! Availability != Feasibility.
//! Feasibility != Reservation.
//! Reservation != Acquisition.
//! Acquisition != Allocation.
//! Allocation != Execution.
//!
//! Observation != Guarantee.
//! Unknown != Zero.
//! Stale != Unavailable.
//! Reservation request != Confirmed reservation.
//! Release request != Confirmed release.
//! Provider measurement != Independently verified measurement.
//!
//! ZHAL must never fabricate physical resource availability,
//! reservations, acquisitions or usage measurements.
//!
//! # Resource subsystem composition
//!
//! ```text
//!                  ZHAL RESOURCES
//!                         |
//!           +-------------+-------------+
//!           |             |             |
//!           v             v             v
//!      OBSERVATION    RESERVATION    ACCOUNTING
//!           |             |
//!           |             v
//!           |       RESERVATION
//!           |      RECONCILIATION
//!           |             |
//!           +------+------+
//!                  |
//!                  v
//!        CANONICAL RESOURCE MODEL
//!                  |
//!                  v
//!        HARDWARE PROVIDER CONTRACTS
//!                  |
//!                  v
//!          PHYSICAL HARDWARE
//! ```
//!
//! # POCO-REAF
//!
//! This subsystem supports provider-independent physical resource
//! integration across:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - GPUs and other accelerators.
//! - AI and tensor hardware.
//! - Embedded computing.
//! - Distributed computing.
//! - Hybrid computing.
//! - Networking and interconnects.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! Resource categories, quantities, units, providers and hardware
//! implementations remain extensible.
//!
//! No arbitrary universal hardware ceilings are introduced.
//!
//! Operational limits may be supplied through explicit configuration
//! and externally owned resource-budget policies.
//!
//! Actual operations remain subject to available physical resources,
//! memory, address-space representation, provider capabilities and
//! configured operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Grammar boundary
//!
//! The canonical source-level resource grammar remains:
//!
//!     grammar/resources/resources.g4
//!
//! Its specialized components include:
//!
//!     grammar/resources/resource.g4
//!     grammar/resources/budgets.g4
//!     grammar/declarations/resources.g4
//!
//! The normative resource semantic specification is:
//!
//!     grammar/spec/resources.md
//!
//! These define resource intent, requirements, constraints, budgets,
//! preferences, hints and lifecycle intent.
//!
//! This module does not parse grammar or interpret source-language
//! resource expressions.
//!
//! The frontend semantic resource model remains authoritative.
//!
//! ZHAL receives already established downstream hardware-facing
//! resource information.
//!
//! # Concurrency
//!
//! This module introduces no global mutable state.
//!
//! Each component retains its own documented synchronization policy.
//!
//! Provider callbacks must not execute while internal accounting
//! or observation collection locks are held.
//!
//! # Security
//!
//! Physical resource identities, reservation references and
//! measurement records are not authentication credentials.
//!
//! Secrets must never be stored in resource descriptions,
//! observations, accounting records or public identifiers.
//!
//! Authorization and provider trust remain owned by the
//! existing ZUR and provider-security authorities.
//!
//! # Public API stability
//!
//! Each component is independently versioned.
//!
//! Public exports are centralized here.
//!
//! New hardware technologies should normally integrate through
//! existing provider and domain-specific extension contracts,
//! without modifying this composition module.


// ============================================================================
// COMPONENT MODULES
// ============================================================================

/// Physical resource observations and evidence management.
///
/// Owns:
///
/// - Installed and advertised capacity evidence.
/// - Resource observation records.
/// - Observation freshness.
/// - Observation collection validation.
/// - Immutable observation snapshots.
///
/// Does not allocate or reserve resources.
pub mod observation;

/// Provider-facing physical hardware reservation contracts.
///
/// Owns:
///
/// - Reservation requests.
/// - Reservation references.
/// - Reservation grants.
/// - Reservation validation.
/// - Provider-facing reservation.
/// - Provider-facing renewal.
/// - Provider-facing release.
///
/// ZUR remains responsible for runtime resource acquisition
/// and overall allocation orchestration.
pub mod reservation;

/// Provider-facing inspection and reconciliation of physical
/// hardware reservations.
///
/// Owns:
///
/// - Reservation lookup contracts.
/// - Reservation state observations.
/// - Authoritative provider inspection.
/// - Ambiguous reservation outcome reconciliation.
///
/// ZUR remains the authoritative runtime lease lifecycle owner.
pub mod leases;

/// Provider-reported physical resource usage accounting.
///
/// Owns:
///
/// - Provider usage reports.
/// - Measurement identities.
/// - Measurement classifications.
/// - Measurement uncertainty.
/// - Measurement provenance.
/// - Idempotent report ingestion.
/// - Immutable accounting snapshots.
/// - Exact aggregation of compatible delta measurements.
///
/// ZUR retains authoritative runtime acquisition and release
/// accounting.
pub mod accounting;


// ============================================================================
// OBSERVATION PUBLIC API
// ============================================================================

/// Public physical resource observation contracts.
pub use self::observation::{
    FreshnessAssessment,
    ObservationFreshness,
    ResourceCapacityEvidence,
    ResourceObservationRecord,
    ResourceObservationSet,
    ResourceObservationSetBuilder,
};


// ============================================================================
// RESERVATION PUBLIC API
// ============================================================================

/// Public physical resource reservation contracts.
pub use self::reservation::{
    HardwareReservationGrant,
    HardwareReservationProvider,
    HardwareReservationProviderIdentity,
    HardwareReservationReference,
    HardwareReservationRequest,
    release_validated,
    reserve_validated,
    renew_validated,
    validate_grant,
};


// ============================================================================
// RESERVATION RECONCILIATION PUBLIC API
// ============================================================================

/// Public physical reservation reconciliation contracts.
pub use self::leases::{
    HardwareReservationLeaseObservation,
    HardwareReservationLeaseProvider,
    HardwareReservationLeaseState,
    HardwareReservationLookup,
    inspect_validated,
    is_confirmed_active,
    is_confirmed_released,
    requires_reconciliation,
};


// ============================================================================
// ACCOUNTING PUBLIC API
// ============================================================================

/// Public provider-reported resource accounting contracts.
pub use self::accounting::{
    AccountingSnapshot,
    AccountingUpdate,
    MeasurementInterval,
    MeasurementKind,
    MeasurementQuality,
    MeasurementUncertainty,
    ProviderTimestamp,
    ProviderUsageReport,
    ReportCompleteness,
    ResourceAccounting,
};


// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.resources";

/// Resource subsystem API version.
///
/// This is independent of the Zamani language version,
/// ZHAL package version and individual provider API versions.
pub const API_VERSION: u16 = 1;

/// Returns the canonical resource subsystem component identifier.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the resource subsystem API version.
#[must_use]
pub const fn api_version() -> u16 {
    API_VERSION
}

/// Returns the number of independently composed resource components.
///
/// This describes the composition structure, not a hardware limit.
#[must_use]
pub const fn component_count() -> usize {
    4
}

/// Returns the resource subsystem's safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    "safe-rust-only"
}

/// Returns the resource subsystem's scalability policy.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    "open-ended; bounded only by available resources and explicit operational budgets"
}


// ============================================================================
// ARCHITECTURAL CONTRACT VALIDATION
// ============================================================================

/// Describes the independently versioned resource component contracts.
///
/// These versions identify component schemas, not hardware firmware,
/// provider implementation versions or runtime versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceComponentVersions {
    /// Physical observation schema version.
    pub observation: u16,

    /// Physical reservation schema version.
    pub reservation: u16,

    /// Reservation reconciliation schema version.
    pub reconciliation: u16,

    /// Provider accounting API version.
    pub accounting: u16,
}

/// Returns the versions of the four composed resource components.
///
/// This function provides version visibility without introducing
/// another version authority.
///
/// Individual component modules remain authoritative for their
/// respective schema versions.
#[must_use]
pub const fn component_versions() -> ResourceComponentVersions {
    ResourceComponentVersions {
        observation: observation::OBSERVATION_SCHEMA_VERSION,
        reservation: reservation::schema_version(),
        reconciliation: leases::schema_version(),
        accounting: accounting::ACCOUNTING_API_VERSION,
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Composition
    // ------------------------------------------------------------------------

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            component_id(),
            "zhal.resources"
        );
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(
            api_version(),
            1
        );
    }

    #[test]
    fn all_resource_components_are_composed() {
        assert_eq!(
            component_count(),
            4
        );
    }

    #[test]
    fn component_versions_are_available() {
        let versions = component_versions();

        assert_eq!(
            versions.observation,
            observation::OBSERVATION_SCHEMA_VERSION
        );

        assert_eq!(
            versions.reservation,
            reservation::schema_version()
        );

        assert_eq!(
            versions.reconciliation,
            leases::schema_version()
        );

        assert_eq!(
            versions.accounting,
            accounting::ACCOUNTING_API_VERSION
        );
    }

    // ------------------------------------------------------------------------
    // Architectural policies
    // ------------------------------------------------------------------------

    #[test]
    fn subsystem_uses_safe_rust_policy() {
        assert_eq!(
            safety_policy(),
            "safe-rust-only"
        );
    }

    #[test]
    fn subsystem_uses_open_ended_scalability_policy() {
        assert_eq!(
            scalability_policy(),
            "open-ended; bounded only by available resources and explicit operational budgets"
        );
    }

    // ------------------------------------------------------------------------
    // Public API integration
    // ------------------------------------------------------------------------

    #[test]
    fn observation_types_are_available_from_composition_root() {
        fn assert_observation_api<T>() {}

        assert_observation_api::<ResourceObservationSet>();
        assert_observation_api::<ResourceObservationRecord>();
        assert_observation_api::<ResourceCapacityEvidence>();
        assert_observation_api::<FreshnessAssessment>();
    }

    #[test]
    fn reservation_types_are_available_from_composition_root() {
        fn assert_reservation_api<T>() {}

        assert_reservation_api::<HardwareReservationRequest>();
        assert_reservation_api::<HardwareReservationGrant>();
        assert_reservation_api::<HardwareReservationReference>();
    }

    #[test]
    fn reconciliation_types_are_available_from_composition_root() {
        fn assert_reconciliation_api<T>() {}

        assert_reconciliation_api::<HardwareReservationLookup>();
        assert_reconciliation_api::<HardwareReservationLeaseState>();
        assert_reconciliation_api::<HardwareReservationLeaseObservation>();
    }

    #[test]
    fn accounting_types_are_available_from_composition_root() {
        fn assert_accounting_api<T>() {}

        assert_accounting_api::<ResourceAccounting>();
        assert_accounting_api::<AccountingSnapshot>();
        assert_accounting_api::<ProviderUsageReport>();
        assert_accounting_api::<MeasurementUncertainty>();
    }

    // ------------------------------------------------------------------------
    // Thread-safety contracts
    // ------------------------------------------------------------------------

    #[test]
    fn accounting_handle_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ResourceAccounting>();
    }

    // ------------------------------------------------------------------------
    // Independent component accessibility
    // ------------------------------------------------------------------------

    #[test]
    fn component_modules_remain_independently_accessible() {
        assert_eq!(
            observation::COMPONENT,
            "zhal.resources.observation"
        );

        assert_eq!(
            reservation::COMPONENT,
            "zhal.resources.reservation"
        );

        assert_eq!(
            leases::COMPONENT,
            "zhal.resources.leases"
        );

        assert_eq!(
            accounting::COMPONENT,
            "zhal.resources.accounting"
        );
    }
}
