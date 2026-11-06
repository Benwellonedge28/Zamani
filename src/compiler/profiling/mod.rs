#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Profiling.
//!
//! Path:
//!     src/compiler/profiling/mod.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the composition root for compiler profiling.
//!
//! It exposes the existing compiler profiling implementations
//! through one stable namespace:
//!
//! - `events`: structured compiler stage observations;
//! - `measurements`: individual typed measurement records;
//! - `aggregation`: streaming measurement aggregation.
//!
//! It establishes module composition and public namespace boundaries.
//! It does not introduce competing implementations of those
//! responsibilities.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - profiling child-module composition;
//! - profiling public module boundaries;
//! - profiling API version coordination;
//! - profiling integration documentation;
//! - module-level safety requirements;
//! - composition-level tests.
//!
//! This module does not own:
//!
//! - compiler stage identity;
//! - compiler pipeline orchestration;
//! - compiler lifecycle state;
//! - event transport;
//! - distributed event persistence;
//! - distributed tracing;
//! - metric registration;
//! - generic metric counters, gauges, or histograms;
//! - measurement construction;
//! - measurement aggregation;
//! - profiling storage;
//! - hardware performance counters;
//! - hardware discovery;
//! - physical resource allocation;
//! - optimization decisions;
//! - runtime execution;
//! - canonical intermediate representations.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # Child-module responsibilities
//!
//! ## events
//!
//! `events.rs` owns compiler-specific profiling event construction
//! and classification.
//!
//! It uses the canonical compiler pipeline stage identity and
//! distributed observability event infrastructure.
//!
//! Events describe established facts. They do not initiate,
//! schedule, or modify compilation.
//!
//! ## measurements
//!
//! `measurements.rs` owns individual typed compiler measurement
//! records, measurement provenance, stability classification,
//! validation, and conversion from completed stage observations.
//!
//! It does not collect hardware counters or retain measurement
//! histories.
//!
//! ## aggregation
//!
//! `aggregation.rs` owns streaming aggregation of measurement
//! records, aggregate snapshots, grouping, numeric accumulation,
//! configurable aggregation budgets, and adaptation into the
//! canonical distributed metrics registry.
//!
//! It does not retain raw measurement histories or create another
//! metrics registry.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! compiler::pipeline::stage
//!             |
//!             v
//!      profiling::events
//!             |
//!             v
//!   profiling::measurements
//!             |
//!             v
//!   profiling::aggregation
//!             |
//!             v
//! distributed::observability::metrics
//! ```
//!
//! The actual implementations may also consume shared distributed
//! event timestamps, event values, and correlation context.
//!
//! This composition root must not reverse those dependencies or
//! introduce a circular dependency.
//!
//! # Canonical repository authorities
//!
//! Compiler stage identity:
//!     crate::compiler::pipeline::stage
//!
//! Distributed event infrastructure:
//!     crate::distributed::observability::events
//!
//! Distributed metrics infrastructure:
//!     crate::distributed::observability::metrics
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Compiler profiling events:
//!     crate::compiler::profiling::events
//!
//! Compiler profiling measurements:
//!     crate::compiler::profiling::measurements
//!
//! Compiler profiling aggregation:
//!     crate::compiler::profiling::aggregation
//!
//! Optimization telemetry consumer:
//!     crate::compiler::sro
//!
//! # POCO-REAF
//!
//! Compiler profiling is independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - accelerator vendors;
//! - operating systems;
//! - physical memory capacity;
//! - processor count;
//! - worker count;
//! - machine count;
//! - device count;
//! - execution topology;
//! - computational domain;
//! - backend implementation.
//!
//! Profiling records describe compiler observations rather than
//! defining how a particular physical machine must execute a program.
//!
//! Profiling must remain compatible with classical, quantum,
//! hybrid, AI, tensor, HDL, distributed, networking, cryptographic,
//! accelerator, and future computational domains.
//!
//! Adding a new computational domain must not require changing
//! this composition root merely to introduce its measurements.
//!
//! # Scalability
//!
//! This module imposes no universal maximum on:
//!
//! - profiling events;
//! - measurement records;
//! - aggregate groups;
//! - compiler stages;
//! - compilation sessions;
//! - metric names;
//! - metric units;
//! - computational domains;
//! - targets;
//! - hardware devices;
//! - workers;
//! - operations.
//!
//! It creates no global measurement history, unbounded queue,
//! hidden worker pool, or process-global registry.
//!
//! Configured operational budgets belong to the implementations
//! that consume the corresponding resources.
//!
//! Physical memory, processing capacity, integer representation,
//! and configured resource policies remain real implementation
//! boundaries. This module does not promise mathematically
//! infinite storage or computation.
//!
//! # Determinism
//!
//! Determinism guarantees belong to the child modules:
//!
//! - events preserve explicitly supplied timestamps;
//! - measurements distinguish deterministic from
//!   environment-dependent observations;
//! - aggregation provides deterministic grouping and snapshot
//!   ordering.
//!
//! Environment-dependent profiling observations must not silently
//! become semantic identity, program meaning, or deterministic
//! compilation-cache inputs.
//!
//! # Failure semantics
//!
//! Profiling is optional instrumentation.
//!
//! Failure to construct, publish, or aggregate a profiling record
//! must not silently change the meaning of the program.
//!
//! Profiling failures must remain distinguishable from compilation
//! stage failures.
//!
//! Callers decide how to handle observability failures under their
//! configured operational policies.
//!
//! # Security
//!
//! Profiling records must contain operational metadata rather than
//! unrestricted source code, credentials, private keys, authentication
//! tokens, private computational state, or execution payloads.
//!
//! Child modules validate their own inputs.
//!
//! Providers and callers remain responsible for data classification,
//! redaction, access control, retention, and secure transport.
//!
//! # Compatibility
//!
//! Existing child-module APIs remain authoritative.
//!
//! The composition root exposes child modules without redefining
//! their types or introducing ambiguous convenience aliases.
//!
//! Consumers can use explicit paths:
//!
//! - `profiling::events::CompilerProfileEvent`;
//! - `profiling::measurements::Measurement`;
//! - `profiling::aggregation::ProfileAggregator`.
//!
//! The child modules retain independently versioned public APIs.
//! Their API versions must not be assumed to be identical forever.
//!
//! # Completion criteria
//!
//! This composition root is complete when:
//!
//! - all three existing profiling modules are declared;
//! - all three public namespaces are accessible;
//! - their canonical dependencies remain intact;
//! - no duplicate profiling implementation is introduced;
//! - no circular dependency is introduced;
//! - no unsafe Rust is used;
//! - no global mutable profiling state is introduced;
//! - no arbitrary scalability ceiling is introduced;
//! - composition tests pass;
//! - repository-wide integration checks pass.
//!
//! Declaring this module does not by itself establish that the
//! complete compiler profiling subsystem or compiler is production
//! ready.

// =============================================================================
// CHILD MODULES
// =============================================================================

/// Structured compiler profiling events and stage observations.
pub mod events;

/// Individual typed compiler profiling measurements.
pub mod measurements;

/// Streaming compiler profiling aggregation.
pub mod aggregation;

// =============================================================================
// COMPOSITION API VERSION
// =============================================================================

/// Public composition API version for the compiler profiling namespace.
///
/// Individual child modules retain their own API versions:
///
/// - [`events::COMPILER_PROFILE_EVENTS_API_VERSION`];
/// - [`measurements::COMPILER_PROFILE_MEASUREMENTS_API_VERSION`];
/// - [`aggregation::COMPILER_PROFILE_AGGREGATION_API_VERSION`].
///
/// This version describes the composition contract, not the
/// compatibility version of every child implementation.
pub const COMPILER_PROFILING_API_VERSION: u16 = 1;

// =============================================================================
// COMPOSITION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// All profiling child modules must remain publicly accessible.
    #[test]
    fn child_modules_are_available() {
        let _events = core::any::type_name::<
            events::CompilerProfileEvent,
        >();

        let _measurements = core::any::type_name::<
            measurements::Measurement,
        >();

        let _aggregation = core::any::type_name::<
            aggregation::ProfileAggregator,
        >();
    }

    /// Child modules retain independently versioned APIs.
    #[test]
    fn child_api_versions_are_available() {
        assert_ne!(
            events::COMPILER_PROFILE_EVENTS_API_VERSION,
            0
        );

        assert_ne!(
            measurements::COMPILER_PROFILE_MEASUREMENTS_API_VERSION,
            0
        );

        assert_ne!(
            aggregation::COMPILER_PROFILE_AGGREGATION_API_VERSION,
            0
        );
    }

    /// The composition root exposes its own version.
    #[test]
    fn composition_api_version_is_available() {
        assert_eq!(
            COMPILER_PROFILING_API_VERSION,
            1
        );
    }

    /// Measurement types remain accessible through their own
    /// canonical namespace.
    #[test]
    fn measurement_api_is_not_redefined() {
        let _: Option<measurements::Measurement> = None;
        let _: Option<measurements::MeasurementValue> = None;
        let _: Option<measurements::MeasurementSource> = None;
        let _: Option<measurements::MeasurementStability> = None;
    }

    /// Aggregation types remain accessible through their own
    /// canonical namespace.
    #[test]
    fn aggregation_api_is_not_redefined() {
        let _: Option<aggregation::ProfileAggregator> = None;
        let _: Option<aggregation::AggregationConfig> = None;
        let _: Option<aggregation::AggregationSnapshot> = None;
        let _: Option<aggregation::AggregateSummary> = None;
    }

    /// Event types remain accessible through their own
    /// canonical namespace.
    #[test]
    fn event_api_is_not_redefined() {
        let _: Option<events::CompilerProfileEvent> = None;
        let _: Option<events::StageEventKind> = None;
    }
}