//! Zamani Quantum Hardware Abstraction Layer.
//!
//! This module is the authoritative composition boundary for
//! `quantum::hardware`.
//!
//! # Mission
//!
//! `quantum::hardware` provides the provider-neutral Hardware Abstraction
//! Layer (HAL) used by Zamani's quantum compiler, runtime, routing,
//! scheduling, error-correction, benchmarking, and future Danga integration.
//!
//! The HAL is deliberately broader than gate-model QPUs. It is designed to
//! represent:
//!
//! - superconducting processors;
//! - trapped-ion processors;
//! - neutral-atom processors;
//! - photonic processors;
//! - spin/semiconductor processors;
//! - topological processors;
//! - analog quantum processors;
//! - quantum annealers;
//! - distributed/networked quantum systems;
//! - logical/fault-tolerant processors;
//! - simulators;
//! - hardware-oriented emulators;
//! - future quantum execution technologies.
//!
//! # Architectural position
//!
//! The authoritative dependency direction is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├──────────► algorithms
//!      │
//!      ├──────────► optimization
//!      │
//!      ├──────────► error_correction
//!      │
//!      ├──────────► routing
//!      │
//!      └──────────► scheduling
//!                    │
//!                    ▼
//!             quantum::hardware
//!                    │
//!        ┌───────────┼────────────┐
//!        ▼           ▼            ▼
//!     simulator   emulator    adapters
//!                                  │
//!                    ┌─────────────┼──────────────┐
//!                    ▼             ▼              ▼
//!                   IBM           IonQ         Braket
//!                    │             │              │
//!                    └─────────────┼──────────────┘
//!                                  ▼
//!                              QPU/device
//! ```
//!
//! `quantum::benchmarking` consumes the hardware layer.
//!
//! The hardware layer MUST NOT depend on benchmarking.
//!
//! # Canonical semantic boundary
//!
//! The canonical semantic representation remains `quantum::ir`.
//!
//! OpenQASM and QIR are interoperability/compilation representations.
//! Neither replaces the Zamani Quantum IR.
//!
//! ```text
//! Zamani Quantum IR
//!        │
//!        ├──────────────► OpenQASM
//!        │
//!        └──────────────► QIR
//! ```
//!
//! # Workload model
//!
//! Hardware MUST NOT assume every quantum workload is a gate circuit.
//!
//! The child modules collectively support:
//!
//! ```text
//! Quantum workload
//! ├── gate circuit
//! ├── dynamic circuit
//! ├── pulse program
//! ├── analog program
//! ├── annealing problem
//! ├── sampling workload
//! └── logical/fault-tolerant workload
//! ```
//!
//! # Physical technology versus backend kind
//!
//! These concepts remain independent:
//!
//! ```text
//! technology     = superconducting
//! backend kind   = qpu
//! workload       = gate circuit
//!
//! technology     = neutral atom
//! backend kind   = qpu
//! workload       = analog program
//!
//! technology     = software
//! backend kind   = simulator
//! workload       = gate circuit
//! ```
//!
//! # Execution lifecycle
//!
//! Remote execution is asynchronous by design:
//!
//! ```text
//! workload
//!    │
//!    ▼
//! validation
//!    │
//!    ▼
//! compatibility
//!    │
//!    ▼
//! routing / scheduling
//!    │
//!    ▼
//! adapter translation
//!    │
//!    ▼
//! submit
//!    │
//!    ▼
//! JobId
//!    │
//!    ├── status
//!    ├── queue
//!    ├── cancellation
//!    └── result
//! ```
//!
//! Local simulators may expose synchronous convenience operations, but the
//! provider-neutral contract remains asynchronous-capable.
//!
//! # Provider isolation
//!
//! Provider-specific behavior belongs exclusively below `adapters`.
//!
//! Adding a provider MUST NOT require modifications to the core hardware
//! abstractions merely to accommodate provider-specific concepts.
//!
//! Core modules MUST NOT import provider SDK types.
//!
//! # Security boundary
//!
//! This module performs no credential handling and stores no secrets.
//!
//! Credentials and authentication are represented by dedicated child modules.
//!
//! Backend descriptors and metadata MUST NOT contain:
//!
//! - API keys;
//! - bearer tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - secret environment values.
//!
//! # Calibration boundary
//!
//! Calibration is hardware state, not benchmarking state.
//!
//! Execution provenance may preserve:
//!
//! - calibration snapshot identity;
//! - calibration timestamp;
//! - calibration validity;
//! - hardware revision;
//! - firmware version;
//! - topology version;
//! - instruction-set version;
//! - adapter version;
//! - provider API version.
//!
//! Stale calibration MUST NOT silently satisfy a requirement for current
//! calibration.
//!
//! # Capability negotiation
//!
//! A workload MUST be checked against backend capabilities before submission.
//!
//! Experimental capabilities MUST remain distinguishable from stable
//! capabilities and MUST NOT silently satisfy stable requirements.
//!
//! # Error boundary
//!
//! Hardware errors are represented by the structured error taxonomy in
//! `errors.rs`.
//!
//! Provider-specific errors may be retained as provider metadata but MUST NOT
//! leak provider SDK error types into the core API.
//!
//! # Reproducibility
//!
//! The hardware API is designed to preserve execution provenance including:
//!
//! - workload identity;
//! - backend identity;
//! - job identity;
//! - calibration provenance;
//! - execution options;
//! - seed where supported;
//! - compiler version;
//! - adapter version;
//! - hardware revision;
//! - result provenance.
//!
//! # Thread safety
//!
//! This composition module owns no mutable global state and performs no I/O.
//!
//! Child implementations define their own `Send`/`Sync` guarantees.
//! Registries and adapters must explicitly document concurrency semantics.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! The hardware namespace does not permit unsafe Rust.
//!
//! Provider adapters must not introduce undocumented unsafe execution paths.
//!
//! # Integration contract
//!
//! Other Zamani subsystems consume this namespace as follows:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! quantum::hardware
//!      │
//!      ├── capabilities
//!      ├── topology
//!      ├── calibration
//!      ├── instruction_set
//!      ├── timing
//!      │
//!      ├── compatibility
//!      ├── validation
//!      │
//!      ├── execution
//!      ├── job
//!      ├── queue
//!      └── result
//! ```
//!
//! Routing consumes topology/instruction/calibration constraints.
//!
//! Scheduling consumes timing/resource/calibration constraints.
//!
//! QEC consumes logical-hardware capabilities.
//!
//! Benchmarking consumes backend/execution/job/result/provenance contracts.
//!
//! Danga consumes the stable public hardware API rather than implementing a
//! second hardware abstraction.
//!
//! # Module ownership
//!
//! Each child module owns one architectural responsibility:
//!
//! | Module | Responsibility |
//! |---|---|
//! | `identity` | Stable provider/device/backend identity |
//! | `technology` | Physical technology and execution models |
//! | `capabilities` | Hardware capability declarations |
//! | `instruction_set` | Native hardware instructions |
//! | `timing` | Timing, duration and latency constraints |
//! | `topology` | Physical connectivity |
//! | `calibration` | Calibration and hardware characterization |
//! | `backend_status` | Operational state |
//! | `backend_config` | Backend policy/configuration |
//! | `errors` | Structured hardware errors |
//! | `backend_trait` | Executable backend adapter contract |
//! | `backend` | Backend descriptor/aggregate |
//! | `compatibility` | Workload/backend compatibility |
//! | `validation` | Hardware validation |
//! | `execution` | Execution request contract |
//! | `job` | Job lifecycle |
//! | `queue` | Queue information |
//! | `result` | Normalized execution results |
//! | `cancellation` | Cancellation semantics |
//! | `provider` | Provider abstraction |
//! | `provider_registry` | Provider registration |
//! | `device_registry` | Device/backend registration |
//! | `discovery` | Device discovery |
//! | `credentials` | Non-secret credential references |
//! | `authentication` | Authentication contracts |
//! | `health` | Health checks |
//! | `telemetry` | Observability |
//! | `serialization` | Serialization contracts |
//! | `routing` | Hardware routing constraints |
//! | `scheduling` | Hardware scheduling constraints |
//! | `resource_estimator` | Resource/time/cost estimates |
//! | `pulse` | Pulse-level workloads |
//! | `analog` | Analog workloads |
//! | `annealing` | Annealing workloads |
//! | `logical` | Logical/FTQC workloads |
//! | `simulator` | Simulator hardware abstraction |
//! | `emulator` | Hardware-oriented emulation |
//! | `adapters` | External-provider/interoperability adapters |
//!
//! This file owns only namespace composition and stable boundary metadata.
//!
//! It MUST NOT duplicate child-module implementation logic.
//!
//! # File completion invariant
//!
//! This file is considered complete when:
//!
//! 1. every hardware child module is declared exactly once;
//! 2. every adapter is declared exactly once;
//! 3. no implementation logic is duplicated here;
//! 4. no provider SDK is imported here;
//! 5. no credentials are handled here;
//! 6. no network operation occurs here;
//! 7. no benchmarking code occurs here;
//! 8. no Danga code occurs here;
//! 9. no quantum algorithm is implemented here;
//! 10. no QEC algorithm is implemented here;
//! 11. the public hardware namespace is stable;
//! 12. the module remains compatible with Rust 1.97/1.97.1;
//! 13. adding a provider does not require changing core hardware semantics.
//!
//! -----------------------------------------------------------------------------
//! Module declarations
//! -----------------------------------------------------------------------------
//
// The declarations are grouped by dependency role for readability.
// Rust does not require declaration order to match dependency order.
//
// No child module is implemented through inline module bodies here. This keeps
// every source file independently testable and prevents composition logic from
// becoming coupled to implementation details.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Foundation
// =============================================================================

/// Stable identities for hardware, devices, providers, architectures,
// firmware and revisions.
pub mod identity;

/// Physical quantum technologies, encodings and execution models.
pub mod technology;

/// Backend capability declarations and capability-status semantics.
pub mod capabilities;

/// Native hardware instruction-set representation.
pub mod instruction_set;

/// Hardware timing, latency, synchronization and duration constraints.
pub mod timing;

/// Physical hardware topology and connectivity.
pub mod topology;

/// Calibration snapshots and hardware-characterization state.
pub mod calibration;

// =============================================================================
// Backend model and validation
// =============================================================================

/// Operational backend/device status.
pub mod backend_status;

/// Backend configuration and execution policy.
pub mod backend_config;

/// Structured provider-neutral hardware errors.
pub mod errors;

/// Provider-neutral executable backend adapter contract.
pub mod backend_trait;

/// Canonical backend descriptor and hardware aggregate.
pub mod backend;

/// Workload/backend capability negotiation.
pub mod compatibility;

/// Deterministic hardware validation.
pub mod validation;

// =============================================================================
// Execution lifecycle
// =============================================================================

/// Provider-neutral execution requests and execution semantics.
pub mod execution;

/// Quantum job lifecycle and job identity.
pub mod job;

/// Queue state and queue metadata.
pub mod queue;

/// Normalized execution results and result provenance.
pub mod result;

/// Job cancellation semantics.
pub mod cancellation;

// =============================================================================
// Provider and device management
// =============================================================================

/// Provider abstraction.
pub mod provider;

/// Provider registration and lookup.
pub mod provider_registry;

/// Device/backend registration and lookup.
pub mod device_registry;

/// Backend/device discovery.
pub mod discovery;

// =============================================================================
// Security and operations
// =============================================================================

/// Non-secret credential references.
pub mod credentials;

/// Authentication abstraction.
pub mod authentication;

/// Backend/device health checks.
pub mod health;

/// Hardware execution telemetry and observability.
pub mod telemetry;

// =============================================================================
// Persistence, compilation constraints and resource planning
// =============================================================================

/// Stable serialization/deserialization contracts.
pub mod serialization;

/// Hardware-facing routing constraints.
pub mod routing;

/// Hardware-facing scheduling constraints.
pub mod scheduling;

/// Hardware resource, duration and cost estimation.
pub mod resource_estimator;

// =============================================================================
// Quantum execution technologies
// =============================================================================

/// Pulse-level quantum-control representation.
pub mod pulse;

/// Analog Hamiltonian/control representation.
pub mod analog;

/// Quantum annealing, Ising and QUBO representation.
pub mod annealing;

/// Logical-qubit and fault-tolerant hardware representation.
pub mod logical;

/// Simulator hardware abstraction.
pub mod simulator;

/// Hardware-oriented emulator abstraction.
pub mod emulator;

// =============================================================================
// External adapters
// =============================================================================

/// External provider and interoperability adapters.
///
/// Provider-specific implementations are isolated below this namespace.
///
/// The adapter namespace is intentionally composed here so that this file
/// remains the single authoritative hardware composition boundary.
//
// IMPORTANT:
// The `#[path]` attributes are deliberately avoided. Each adapter source is
// declared using its normal path relative to the adapters module once the
// adapters directory has its own module boundary.
//
// If the repository's adapter directory does not yet contain `adapters/mod.rs`,
// the adapter directory must be given that module boundary before enabling
// these declarations. The parent hardware module must never duplicate the
// adapter implementation files.

pub mod adapters;

// =============================================================================
// Stable HAL metadata
// =============================================================================

/// Stable schema identifier for the Zamani quantum hardware namespace.
pub const HARDWARE_SCHEMA_ID: &str = "zamani.quantum.hardware";

/// Major version of the public hardware HAL composition contract.
///
/// Breaking semantic/API changes increment this value.
pub const HARDWARE_API_MAJOR: u16 = 1;

/// Minor version of the public hardware HAL composition contract.
///
/// Backwards-compatible additions increment this value.
pub const HARDWARE_API_MINOR: u16 = 0;

/// Patch version of the public hardware HAL composition contract.
///
/// Non-semantic corrections increment this value.
pub const HARDWARE_API_PATCH: u16 = 0;

/// Complete public hardware HAL version.
pub const HARDWARE_API_VERSION: (u16, u16, u16) = (
    HARDWARE_API_MAJOR,
    HARDWARE_API_MINOR,
    HARDWARE_API_PATCH,
);

/// Rust edition targeted by this module.
pub const HARDWARE_RUST_EDITION: &str = "2021";

/// Minimum supported Rust compiler version.
pub const HARDWARE_MIN_RUST_VERSION: &str = "1.97";

/// Returns the stable hardware HAL schema identifier.
#[inline]
pub const fn hardware_schema_id() -> &'static str {
    HARDWARE_SCHEMA_ID
}

/// Returns the public hardware HAL version.
#[inline]
pub const fn hardware_api_version() -> (u16, u16, u16) {
    HARDWARE_API_VERSION
}

// =============================================================================
// Architectural invariants
// =============================================================================

/// Compile-time identifiers for the non-negotiable hardware architecture
/// boundaries.
///
/// These are descriptive architectural contracts, not runtime configuration.
pub mod invariants {
    /// Hardware must not depend on benchmarking.
    pub const NO_BENCHMARKING_DEPENDENCY: &str =
        "hardware-does-not-depend-on-benchmarking";

    /// Hardware must not depend on Danga.
    pub const NO_DANGA_DEPENDENCY: &str =
        "hardware-does-not-depend-on-danga";

    /// Core hardware types remain provider neutral.
    pub const PROVIDER_NEUTRAL_CORE: &str =
        "provider-neutral-core";

    /// Provider-specific behavior belongs below `adapters`.
    pub const PROVIDERS_ISOLATED_IN_ADAPTERS: &str =
        "providers-isolated-in-adapters";

    /// Zamani Quantum IR remains canonical.
    pub const ZAMANI_IR_IS_CANONICAL: &str =
        "zamani-quantum-ir-is-canonical";

    /// OpenQASM is an interoperability representation.
    pub const OPENQASM_IS_INTEROPERABILITY: &str =
        "openqasm-is-interoperability";

    /// QIR is an interoperability representation.
    pub const QIR_IS_INTEROPERABILITY: &str =
        "qir-is-interoperability";

    /// Credentials do not form part of backend identity.
    pub const CREDENTIALS_NOT_IN_BACKEND_IDENTITY: &str =
        "credentials-not-in-backend-identity";

    /// Remote execution supports asynchronous job lifecycle.
    pub const ASYNC_JOB_LIFECYCLE: &str =
        "asynchronous-job-lifecycle";

    /// Experimental capabilities remain explicit.
    pub const EXPERIMENTAL_CAPABILITIES_EXPLICIT: &str =
        "experimental-capabilities-explicit";

    /// Calibration provenance is preserved.
    pub const CALIBRATION_PROVENANCE: &str =
        "calibration-provenance";

    /// Result provenance is preserved.
    pub const RESULT_PROVENANCE: &str =
        "result-provenance";

    /// Provider additions must not alter core provider-neutral semantics.
    pub const PROVIDER_ADDITIONS_ARE_NON_BREAKING: &str =
        "provider-additions-are-non-breaking";
}

// =============================================================================
// Controlled public prelude
// =============================================================================

/// Stable high-level hardware namespace.
///
/// The prelude exposes subsystem boundaries rather than flattening every
/// implementation type into one namespace. This protects callers from future
/// file-layout changes.
pub mod prelude {
    pub use super::adapters;
    pub use super::analog;
    pub use super::annealing;
    pub use super::authentication;
    pub use super::backend;
    pub use super::backend_config;
    pub use super::backend_status;
    pub use super::backend_trait;
    pub use super::calibration;
    pub use super::cancellation;
    pub use super::capabilities;
    pub use super::compatibility;
    pub use super::credentials;
    pub use super::device_registry;
    pub use super::discovery;
    pub use super::emulator;
    pub use super::errors;
    pub use super::execution;
    pub use super::health;
    pub use super::identity;
    pub use super::instruction_set;
    pub use super::job;
    pub use super::logical;
    pub use super::provider;
    pub use super::provider_registry;
    pub use super::pulse;
    pub use super::queue;
    pub use super::resource_estimator;
    pub use super::result;
    pub use super::routing;
    pub use super::scheduling;
    pub use super::serialization;
    pub use super::simulator;
    pub use super::technology;
    pub use super::telemetry;
    pub use super::timing;
    pub use super::topology;
    pub use super::validation;
}

// =============================================================================
// Namespace-level compile-time smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardware_schema_is_stable() {
        assert_eq!(
            HARDWARE_SCHEMA_ID,
            "zamani.quantum.hardware"
        );
    }

    #[test]
    fn hardware_api_version_is_valid() {
        let (major, minor, patch) = HARDWARE_API_VERSION;

        assert!(major >= 1);
        assert!(minor < 1_000);
        assert!(patch < 1_000);
    }

    #[test]
    fn rust_compatibility_contract_is_explicit() {
        assert_eq!(HARDWARE_RUST_EDITION, "2021");
        assert_eq!(HARDWARE_MIN_RUST_VERSION, "1.97");
    }

    #[test]
    fn schema_helpers_are_consistent() {
        assert_eq!(
            hardware_schema_id(),
            HARDWARE_SCHEMA_ID
        );

        assert_eq!(
            hardware_api_version(),
            HARDWARE_API_VERSION
        );
    }

    #[test]
    fn architectural_invariants_are_non_empty() {
        let values = [
            invariants::NO_BENCHMARKING_DEPENDENCY,
            invariants::NO_DANGA_DEPENDENCY,
            invariants::PROVIDER_NEUTRAL_CORE,
            invariants::PROVIDERS_ISOLATED_IN_ADAPTERS,
            invariants::ZAMANI_IR_IS_CANONICAL,
            invariants::OPENQASM_IS_INTEROPERABILITY,
            invariants::QIR_IS_INTEROPERABILITY,
            invariants::CREDENTIALS_NOT_IN_BACKEND_IDENTITY,
            invariants::ASYNC_JOB_LIFECYCLE,
            invariants::EXPERIMENTAL_CAPABILITIES_EXPLICIT,
            invariants::CALIBRATION_PROVENANCE,
            invariants::RESULT_PROVENANCE,
            invariants::PROVIDER_ADDITIONS_ARE_NON_BREAKING,
        ];

        for value in values {
            assert!(!value.is_empty());
        }
    }

    #[test]
    fn public_prelude_exposes_all_hardware_boundaries() {
        let _ = &prelude::identity;
        let _ = &prelude::technology;
        let _ = &prelude::capabilities;
        let _ = &prelude::instruction_set;
        let _ = &prelude::timing;
        let _ = &prelude::topology;
        let _ = &prelude::calibration;

        let _ = &prelude::backend_status;
        let _ = &prelude::backend_config;
        let _ = &prelude::errors;
        let _ = &prelude::backend_trait;
        let _ = &prelude::backend;
        let _ = &prelude::compatibility;
        let _ = &prelude::validation;

        let _ = &prelude::execution;
        let _ = &prelude::job;
        let _ = &prelude::queue;
        let _ = &prelude::result;
        let _ = &prelude::cancellation;

        let _ = &prelude::provider;
        let _ = &prelude::provider_registry;
        let _ = &prelude::device_registry;
        let _ = &prelude::discovery;

        let _ = &prelude::credentials;
        let _ = &prelude::authentication;
        let _ = &prelude::health;
        let _ = &prelude::telemetry;

        let _ = &prelude::serialization;
        let _ = &prelude::routing;
        let _ = &prelude::scheduling;
        let _ = &prelude::resource_estimator;

        let _ = &prelude::pulse;
        let _ = &prelude::analog;
        let _ = &prelude::annealing;
        let _ = &prelude::logical;
        let _ = &prelude::simulator;
        let _ = &prelude::emulator;
        let _ = &prelude::adapters;
    }
}