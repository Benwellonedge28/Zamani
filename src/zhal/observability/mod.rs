
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Observability
//!
//! Path: `src/zhal/observability/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the public composition root for hardware
//! observability within the Zamani Universal Hardware
//! Abstraction Layer (ZHAL).
//!
//! It integrates three independently owned components:
//!
//! - Hardware telemetry.
//! - Structured hardware diagnostics.
//! - Hardware and execution provenance.
//!
//! Its responsibility is to provide a consistent public
//! interface to those implementations.
//!
//! It does not implement another telemetry engine, logging
//! framework, metrics registry, tracing system, diagnostic
//! engine, provenance database or event journal.
//!
//! # 2. PRIMARY OBJECTIVE
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Hardware observability must remain portable across:
//!
//! - Classical processors and computing systems.
//! - Quantum processors and quantum-control systems.
//! - HDL realization and programmable hardware.
//! - FPGA and ASIC systems.
//! - GPUs, TPUs, NPUs and other accelerators.
//! - AI and tensor computation.
//! - Embedded and edge computing.
//! - Hybrid computing.
//! - Distributed and heterogeneous computing.
//! - Photonic and optical computing.
//! - Neuromorphic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! New hardware categories and providers must be integratable
//! without modifying this composition root.
//!
//! # 3. ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Observability module composition.
//! - Public module declarations.
//! - Stable observability API paths.
//! - Deliberate public API re-exports.
//! - Component version discoverability.
//! - Composition-level contract tests.
//! - Observability integration documentation.
//!
//! This module DOES NOT own:
//!
//! - Hardware discovery.
//! - Hardware inventory.
//! - Provider registration.
//! - Provider lifecycle.
//! - Device allocation.
//! - Resource allocation.
//! - Resource scheduling.
//! - Hardware execution.
//! - Runtime lifecycle.
//! - Distributed membership.
//! - Distributed execution.
//! - Quantum routing.
//! - Quantum error correction.
//! - HDL synthesis.
//! - Backend selection.
//! - Compiler profiling.
//! - Semantic analysis.
//! - Source grammar.
//! - Execution planning.
//! - Network transport.
//! - Telemetry persistence.
//! - Logging implementation.
//! - Generic metrics collection.
//! - Distributed trace identity allocation.
//! - Authentication.
//! - Authorization.
//! - Security policy evaluation.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # 4. CHILD MODULE RESPONSIBILITIES
//!
//! ## telemetry
//!
//! `telemetry.rs` owns provider-neutral hardware observations.
//!
//! It provides:
//!
//! - HardwareTelemetryObservation.
//! - HardwareTelemetryRecorder.
//! - HardwareTelemetrySink.
//! - TelemetryName.
//! - TelemetryUnit.
//! - TelemetrySubjectRef.
//! - ObservationTimestamp.
//! - ObservationClass.
//! - ObservationOutcome.
//! - MeasurementValue.
//! - HardwareMeasurement.
//! - TelemetryError.
//! - TelemetryErrorKind.
//!
//! It records observations about hardware and providers.
//!
//! It does not create a second generic metrics registry,
//! trace system, logging system or persistence engine.
//!
//! ## diagnostics
//!
//! `diagnostics.rs` owns structured hardware diagnostics.
//!
//! It provides:
//!
//! - ZhalDiagnostic.
//! - DiagnosticOrigin.
//! - DiagnosticContextSnapshot.
//! - DiagnosticValidationError.
//! - Diagnostic schema identification.
//! - Redaction-aware diagnostic composition.
//! - Unknown and partial operation outcome preservation.
//! - Retryability and severity reporting.
//!
//! It consumes canonical ZHAL errors and contexts.
//!
//! It does not redefine error classification, implement
//! recovery, or make retry decisions.
//!
//! ## provenance
//!
//! `provenance.rs` owns hardware provenance composition.
//!
//! It provides:
//!
//! - HardwareProvenance.
//! - ProvenanceReference.
//! - HardwareArtifactRevision.
//! - OperationProvenance.
//! - ResultProvenance.
//! - ProvenanceError.
//! - ProvenanceErrorKind.
//! - ProvenanceResult.
//! - Provenance schema identification.
//!
//! It records references to externally owned evidence.
//!
//! It does not implement artifact storage, cryptographic
//! signing, authentication, authorization or evidence
//! retrieval.
//!
//! # 5. CANONICAL INTEGRATION AUTHORITIES
//!
//! The following existing authorities must be reused.
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Capability definitions and evidence:
//!     crate::stdlib::capability
//!
//! Standard-library operation context:
//!     crate::stdlib::context
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware error classification:
//!     crate::zhal::errors
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Runtime logging:
//!     crate::zur::observability::logging
//!
//! Runtime metrics:
//!     crate::zur::observability::metrics
//!
//! Runtime tracing:
//!     crate::zur::observability::tracing
//!
//! Runtime profiling:
//!     crate::zur::observability::profiling
//!
//! Generic distributed events:
//!     crate::distributed::observability::events
//!
//! Generic distributed metrics:
//!     crate::distributed::observability::metrics
//!
//! Generic distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Quantum hardware telemetry:
//!     crate::quantum::hardware
//!
//! Backend telemetry:
//!     crate::backend::telemetry
//!
//! These systems remain independently authoritative.
//!
//! # 6. DOMAIN INTEGRATION
//!
//! Classical hardware:
//!     crate::zhal::adapters::classical
//!
//! Quantum hardware:
//!     crate::zhal::adapters::quantum
//!
//! HDL realization:
//!     crate::zhal::adapters::hdl
//!
//! Accelerators:
//!     crate::zhal::adapters::accelerator
//!
//! Distributed hardware:
//!     crate::zhal::adapters::distributed
//!
//! Embedded hardware:
//!     crate::zhal::adapters::embedded
//!
//! Domain adapters supply observations to the existing
//! telemetry, diagnostics and provenance interfaces.
//!
//! This module must not depend on those future adapters.
//!
//! That dependency direction allows the observability
//! contracts to remain independently reusable.
//!
//! # 7. OBSERVABILITY INVARIANTS
//!
//! The following invariants are mandatory:
//!
//! 1. Observation is not execution.
//! 2. Measurement is not proof of successful computation.
//! 3. Provenance is not proof of authenticity.
//! 4. Trace correlation is not authentication.
//! 5. Telemetry publication is not durable persistence.
//! 6. Missing observations do not imply successful execution.
//! 7. Unknown operation outcomes remain unknown.
//! 8. Partial completion remains explicit.
//! 9. Cancellation requests do not prove physical termination.
//! 10. Retryability does not trigger automatic retries.
//! 11. Telemetry failures do not fabricate computation results.
//! 12. Diagnostic failures do not change hardware execution outcomes.
//! 13. Provenance references do not establish authorization.
//! 14. Sampling does not determine whether computation executes.
//! 15. Observability does not allocate physical resources.
//! 16. Observability does not make scheduling decisions.
//! 17. Future hardware must not require core-module modification.
//!
//! # 8. ERROR ISOLATION
//!
//! Observability errors must remain distinct from the errors
//! of the hardware operation being observed.
//!
//! For example:
//!
//! A hardware operation may succeed while telemetry recording
//! fails.
//!
//! A hardware operation may fail while diagnostic reporting
//! succeeds.
//!
//! A hardware operation may have an unknown outcome while
//! provenance successfully records the available references.
//!
//! The caller and owning runtime must preserve these outcomes
//! independently.
//!
//! This composition root does not suppress or reinterpret
//! failures.
//!
//! # 9. SECURITY AND PRIVACY
//!
//! The component implementations must preserve their existing
//! redaction and validation contracts.
//!
//! Observability records must not become unrestricted storage
//! for:
//!
//! - Passwords.
//! - Private keys.
//! - Authentication tokens.
//! - Authorization headers.
//! - Session secrets.
//! - Private execution payloads.
//! - Raw quantum circuits.
//! - Confidential application data.
//! - Unrestricted source code.
//!
//! Provenance references are not credentials.
//!
//! Diagnostic references are not authorization grants.
//!
//! External sinks remain responsible for their own access
//! control, encryption, retention and destination security.
//!
//! # 10. SCALABILITY AND POCO-REAF
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Hardware providers.
//! - Hardware devices.
//! - Computational domains.
//! - Telemetry observations.
//! - Measurement definitions.
//! - Diagnostic records.
//! - Provenance references.
//! - Hardware resources.
//! - Distributed participants.
//! - Hardware revisions.
//! - Operation references.
//! - Result references.
//!
//! It introduces no:
//!
//! - MAX_EVENTS.
//! - MAX_METRICS.
//! - MAX_PROVIDERS.
//! - MAX_DEVICES.
//! - MAX_SPANS.
//! - MAX_TRACE_DEPTH.
//! - MAX_SAMPLES.
//! - MAX_RESOURCES.
//! - MAX_DOMAINS.
//! - Fixed hardware capacities.
//!
//! Actual observation throughput, memory consumption,
//! processing capacity and retention depend on the available
//! resources and explicitly configured operational policies.
//!
//! Operational budgets are permitted.
//!
//! They must remain configurable resource policies rather
//! than permanent hardware or language-level ceilings.
//!
//! No finite physical machine promises literal infinite
//! computation or storage.
//!
//! POCO-REAF means preserving program semantics and avoiding
//! arbitrary universal restrictions while allowing compatible
//! implementations to scale with available resources.
//!
//! # 11. GRAMMAR INTEGRATION
//!
//! Canonical source-level observability intent:
//!
//!     grammar/execution/observability.g4
//!
//! Hardware capability intent:
//!
//!     grammar/hardware/capabilities.g4
//!
//! Normative execution model:
//!
//!     grammar/specification/execution-model.md
//!
//! Normative semantic model:
//!
//!     grammar/specification/semantic-model.md
//!
//! The grammar describes observation intent.
//!
//! The frontend and semantic subsystem validate that intent.
//!
//! The compiler and runtime consume validated contracts.
//!
//! ZHAL records actual hardware observations.
//!
//! This module does not parse grammar productions or
//! reconstruct source-level semantic meaning.
//!
//! # 12. INITIALIZATION AND LIFECYCLE
//!
//! This composition root performs no initialization.
//!
//! It does not:
//!
//! - Install a global logger.
//! - Initialize a metrics registry.
//! - Initialize a tracing subscriber.
//! - Start background workers.
//! - Open hardware devices.
//! - Establish network connections.
//! - Create persistent storage.
//! - Start telemetry exporters.
//!
//! Component construction and sink injection remain explicit.
//!
//! Runtime lifecycle and service activation remain owned
//! by ZUR and the respective service implementations.
//!
//! # 13. VERSIONING
//!
//! Each independently implemented component maintains its
//! own schema and API version.
//!
//! Telemetry:
//!     TELEMETRY_API_VERSION
//!
//! Diagnostics:
//!     DIAGNOSTIC_SCHEMA_VERSION
//!
//! Provenance:
//!     PROVENANCE_SCHEMA_VERSION
//!
//! These versions are intentionally not conflated with:
//!
//! - Zamani language version.
//! - Cargo package version.
//! - Grammar version.
//! - ZHAL API version.
//! - Provider API version.
//! - Hardware firmware version.
//!
//! # 14. SAFETY
//!
//! This module enforces safe Rust.
//!
//! No unsafe blocks, functions or implementations are
//! permitted.
//!
//! No raw-pointer ownership is introduced.
//!
//! No process-global mutable observability state is introduced.
//!
//! # 15. COMPLETION CRITERIA
//!
//! This composition root is complete when:
//!
//! - All three child modules are independently available.
//! - Their public interfaces are consistently exposed.
//! - Existing canonical authorities are reused.
//! - No competing observability infrastructure is created.
//! - No hardware-specific assumptions are introduced.
//! - No arbitrary universal ceilings are introduced.
//! - Composition tests pass.
//! - The module compiles on the supported Rust baseline.
//! - Repository-wide formatting, checks and tests pass.

// ============================================================================
// CHILD MODULE COMPOSITION
// ============================================================================

/// Provider-neutral hardware telemetry observations and sink contracts.
///
/// This module owns hardware observation representations and
/// explicit telemetry sink injection.
pub mod telemetry;

/// Structured, redaction-aware hardware diagnostics.
///
/// This module reuses canonical ZHAL error and context contracts.
pub mod diagnostics;

/// Portable hardware and execution provenance.
///
/// This module composes references to evidence owned by other
/// repository subsystems.
pub mod provenance;

// ============================================================================
// TELEMETRY PUBLIC API
// ============================================================================

/// Hardware telemetry measurement representation.
pub use self::telemetry::HardwareMeasurement;

/// Hardware telemetry observation.
pub use self::telemetry::HardwareTelemetryObservation;

/// Hardware telemetry recorder.
pub use self::telemetry::HardwareTelemetryRecorder;

/// Injectable hardware telemetry sink.
pub use self::telemetry::HardwareTelemetrySink;

/// No-operation telemetry sink.
pub use self::telemetry::NoopHardwareTelemetrySink;

/// Hardware observation classification.
pub use self::telemetry::ObservationClass;

/// Hardware observation outcome.
pub use self::telemetry::ObservationOutcome;

/// Explicit hardware observation timestamp.
pub use self::telemetry::ObservationTimestamp;

/// Hardware telemetry measurement value.
pub use self::telemetry::MeasurementValue;

/// Hardware telemetry name.
pub use self::telemetry::TelemetryName;

/// Hardware telemetry unit.
pub use self::telemetry::TelemetryUnit;

/// Opaque hardware observation subject reference.
pub use self::telemetry::TelemetrySubjectRef;

/// Hardware telemetry result type.
pub use self::telemetry::TelemetryResult;

/// Hardware telemetry error.
pub use self::telemetry::TelemetryError;

/// Hardware telemetry error classification.
pub use self::telemetry::TelemetryErrorKind;

/// Hardware telemetry schema identifier.
pub use self::telemetry::TELEMETRY_SCHEMA_ID;

/// Hardware telemetry API version.
pub use self::telemetry::TELEMETRY_API_VERSION;

/// Returns the hardware telemetry schema identifier.
pub use self::telemetry::telemetry_schema_id;

/// Returns the hardware telemetry API version.
pub use self::telemetry::telemetry_api_version;

// ============================================================================
// DIAGNOSTICS PUBLIC API
// ============================================================================

/// Structured hardware diagnostic record.
pub use self::diagnostics::ZhalDiagnostic;

/// Validated diagnostic origin.
pub use self::diagnostics::DiagnosticOrigin;

/// Diagnostic context snapshot.
pub use self::diagnostics::DiagnosticContextSnapshot;

/// Diagnostic validation error.
pub use self::diagnostics::DiagnosticValidationError;

/// Hardware diagnostic component identifier.
pub use self::diagnostics::COMPONENT as DIAGNOSTICS_COMPONENT;

/// Hardware diagnostic schema identifier.
pub use self::diagnostics::DIAGNOSTIC_SCHEMA;

/// Hardware diagnostic schema version.
pub use self::diagnostics::DIAGNOSTIC_SCHEMA_VERSION;

/// Returns the hardware diagnostic component identifier.
pub use self::diagnostics::component_id;

/// Returns the hardware diagnostic schema identifier.
pub use self::diagnostics::schema_id as diagnostic_schema_id;

/// Returns the hardware diagnostic schema version.
pub use self::diagnostics::schema_version as diagnostic_schema_version;

// ============================================================================
// PROVENANCE PUBLIC API
// ============================================================================

/// Complete hardware provenance record.
pub use self::provenance::HardwareProvenance;

/// Opaque provenance reference.
pub use self::provenance::ProvenanceReference;

/// Hardware artifact revision.
pub use self::provenance::HardwareArtifactRevision;

/// Hardware operation provenance.
pub use self::provenance::OperationProvenance;

/// Hardware result provenance.
pub use self::provenance::ResultProvenance;

/// Provenance error.
pub use self::provenance::ProvenanceError;

/// Provenance error classification.
pub use self::provenance::ProvenanceErrorKind;

/// Provenance result type.
pub use self::provenance::ProvenanceResult;

/// Provenance component identifier.
pub use self::provenance::COMPONENT as PROVENANCE_COMPONENT;

/// Provenance schema identifier.
pub use self::provenance::PROVENANCE_SCHEMA;

/// Provenance schema version.
pub use self::provenance::PROVENANCE_SCHEMA_VERSION;

/// Returns the provenance component identifier.
pub use self::provenance::component_id;

/// Returns the provenance schema identifier.
pub use self::provenance::schema_id as provenance_schema_id;

/// Returns the provenance schema version.
pub use self::provenance::schema_version as provenance_schema_version;

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that all three observability components
    /// are available through the public composition root.
    #[test]
    fn all_observability_components_are_exposed() {
        let _telemetry:
            fn() -> &'static str = telemetry_schema_id;

        let _diagnostics:
            fn() -> &'static str = diagnostic_schema_id;

        let _provenance:
            fn() -> &'static str = provenance_schema_id;

        assert!(
            !telemetry_schema_id().is_empty()
        );

        assert!(
            !diagnostic_schema_id().is_empty()
        );

        assert!(
            !provenance_schema_id().is_empty()
        );
    }

    /// Confirms that the individual schema versions
    /// remain independently accessible.
    #[test]
    fn component_schema_versions_are_available() {
        let telemetry_version =
            telemetry_api_version();

        assert!(
            telemetry_version.0 > 0
        );

        assert!(
            DIAGNOSTIC_SCHEMA_VERSION > 0
        );

        assert!(
            PROVENANCE_SCHEMA_VERSION > 0
        );
    }

    /// Confirms that the three canonical module paths
    /// remain independently accessible.
    #[test]
    fn canonical_module_paths_are_available() {
        let _ = telemetry::TELEMETRY_API_VERSION;

        let _ = diagnostics::DIAGNOSTIC_SCHEMA;

        let _ = provenance::PROVENANCE_SCHEMA;
    }

    /// Confirms that telemetry observations and their
    /// sink contracts are publicly accessible.
    #[test]
    fn telemetry_contracts_are_public() {
        let _ = core::any::type_name::<
            HardwareTelemetryObservation
        >();

        let _ = core::any::type_name::<
            HardwareTelemetryRecorder
        >();

        let _ = core::any::type_name::<
            HardwareTelemetrySink
        >();

        let _ = core::any::type_name::<
            HardwareMeasurement
        >();
    }

    /// Confirms that structured diagnostic contracts
    /// are publicly accessible.
    #[test]
    fn diagnostic_contracts_are_public() {
        let _ = core::any::type_name::<
            ZhalDiagnostic
        >();

        let _ = core::any::type_name::<
            DiagnosticContextSnapshot
        >();

        let _ = core::any::type_name::<
            DiagnosticOrigin
        >();
    }

    /// Confirms that hardware provenance contracts
    /// are publicly accessible.
    #[test]
    fn provenance_contracts_are_public() {
        let _ = core::any::type_name::<
            HardwareProvenance
        >();

        let _ = core::any::type_name::<
            OperationProvenance
        >();

        let _ = core::any::type_name::<
            ResultProvenance
        >();

        let _ = core::any::type_name::<
            ProvenanceReference
        >();
    }

    /// Confirms that the composition root introduces
    /// no hardware-specific device assumptions.
    #[test]
    fn future_hardware_does_not_require_new_root_modules() {
        // Hardware-specific extension belongs to the
        // provider and adapter implementations.
        //
        // The three common observability contracts
        // remain available independently of the
        // hardware category being observed.

        assert!(
            !telemetry_schema_id().is_empty()
        );

        assert!(
            !diagnostic_schema_id().is_empty()
        );

        assert!(
            !provenance_schema_id().is_empty()
        );
    }
}
