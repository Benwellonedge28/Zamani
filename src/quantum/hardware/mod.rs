//! Zamani Quantum Hardware Abstraction Layer.
//!
//! This module is the authoritative public boundary for quantum-hardware
//! interaction in Zamani.
//!
//! # Purpose
//!
//! `quantum::hardware` provides a provider-neutral Hardware Abstraction Layer
//! (HAL) for all quantum execution technologies supported by Zamani.
//!
//! The hardware layer is intentionally broader than gate-model QPUs. It is
//! designed to represent:
//!
//! - superconducting quantum processors;
//! - trapped-ion processors;
//! - neutral-atom processors;
//! - photonic processors;
//! - spin/semiconductor processors;
//! - topological processors;
//! - analog quantum processors;
//! - quantum annealers;
//! - distributed/networked quantum systems;
//! - logical/fault-tolerant quantum processors;
//! - quantum simulators;
//! - hardware-specific emulators;
//! - future quantum execution technologies.
//!
//! # Architectural position
//!
//! The canonical dependency direction is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                ┌─────────────┼─────────────┐
//!                │             │             │
//!                ▼             ▼             ▼
//!           algorithms    optimization      QEC
//!                │             │             │
//!                └─────────────┼─────────────┘
//!                              ▼
//!                         compatibility
//!                              │
//!                 ┌────────────┴────────────┐
//!                 ▼                         ▼
//!              routing                  scheduling
//!                 │                         │
//!                 └────────────┬────────────┘
//!                              ▼
//!                    quantum::hardware
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          simulator        emulator         providers
//!                                               │
//!                  ┌────────────┬────────────────┤
//!                  ▼            ▼                ▼
//!                 IBM         IonQ             Braket
//!                  │            │                │
//!                  └────────────┴────────────────┘
//!                               │
//!                               ▼
//!                           physical QPU
//!
//! benchmarking consumes hardware; hardware never depends on benchmarking.
//! ```
//!
//! # Ownership
//!
//! This module owns the **hardware namespace and public composition**.
//!
//! Individual child modules own their respective semantics:
//!
//! | Module | Responsibility |
//! |---|---|
//! | `identity` | Stable provider/backend/device/architecture identity |
//! | `technology` | Physical quantum technology and execution models |
//! | `capabilities` | Backend capability declarations |
//! | `instruction_set` | Native hardware instructions |
//! | `timing` | Hardware timing and latency constraints |
//! | `topology` | Physical connectivity |
//! | `calibration` | Calibration snapshots and hardware characteristics |
//! | `backend_status` | Operational backend state |
//! | `backend_config` | Backend configuration policy |
//! | `errors` | Hardware error taxonomy |
//! | `backend_trait` | Provider-neutral backend execution contract |
//! | `backend` | Canonical backend descriptor/aggregate |
//! | `compatibility` | Workload/backend capability negotiation |
//! | `validation` | Deterministic hardware validation |
//! | `execution` | Provider-neutral execution contract |
//! | `job` | Quantum job lifecycle |
//! | `queue` | Queue and scheduling metadata |
//! | `result` | Normalized execution results |
//! | `cancellation` | Job cancellation semantics |
//! | `provider` | Provider abstraction |
//! | `provider_registry` | Provider registration and lookup |
//! | `device_registry` | Device/backend registration and lookup |
//! | `discovery` | Backend/device discovery |
//! | `credentials` | Credential references without owning secrets |
//! | `authentication` | Authentication abstraction |
//! | `health` | Backend health checks |
//! | `telemetry` | Hardware observability |
//! | `serialization` | Stable hardware serialization |
//! | `routing` | Hardware routing constraints |
//! | `scheduling` | Hardware scheduling constraints |
//! | `resource_estimator` | Resource/time/cost estimation |
//! | `pulse` | Pulse-level execution |
//! | `analog` | Analog Hamiltonian/control execution |
//! | `annealing` | Annealing/QUBO/Ising execution |
//! | `logical` | Logical/fault-tolerant hardware |
//! | `simulator` | Simulator hardware abstraction |
//! | `emulator` | Hardware-oriented emulation |
//! | `adapters` | External-provider/interoperability adapters |
//!
//! The parent module must not duplicate those responsibilities.
//!
//! # Critical dependency rule
//!
//! The hardware layer is below the canonical quantum IR and above concrete
//! provider implementations.
//!
//! ```text
//! frontend ───────► ir
//! algorithms ─────► ir
//! optimization ───► ir
//! error_correction ► ir
//!
//! ir ─────────────► hardware
//! routing ─────────► hardware
//! scheduling ──────► hardware
//!
//! hardware ───────► provider adapters
//! hardware ───────► runtime/execution integration
//!
//! benchmarking ───► hardware
//! Danga ───────────► hardware
//! ```
//!
//! The following dependencies are forbidden:
//!
//! ```text
//! hardware ─X─► benchmarking
//! hardware ─X─► Danga
//! hardware ─X─► CLI
//! hardware ─X─► application code
//! hardware ─X─► provider-specific SDK semantics in core types
//! ```
//!
//! # Provider independence
//!
//! Provider-specific functionality belongs under [`adapters`].
//!
//! Adding a provider must not require modifying:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `capabilities.rs`;
//! - `technology.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `result.rs`;
//! - `topology.rs`;
//! - `calibration.rs`.
//!
//! A provider adapter consumes the stable hardware contracts.
//!
//! The intended model is:
//!
//! ```text
//! Hardware core
//!      │
//!      ├── generic adapter contract
//!      │
//!      ├── OpenQASM interoperability
//!      ├── QIR interoperability
//!      │
//!      └── provider adapters
//!             ├── IBM
//!             ├── IonQ
//!             ├── AWS Braket
//!             ├── Rigetti
//!             ├── IQM
//!             ├── Quantinuum
//!             └── QuEra
//! ```
//!
//! # Canonical execution model
//!
//! Hardware execution is asynchronous by default:
//!
//! ```text
//! Quantum workload
//!       │
//!       ▼
//! validate
//!       │
//!       ▼
//! compatibility
//!       │
//!       ▼
//! routing / scheduling
//!       │
//!       ▼
//! adapter translation
//!       │
//!       ▼
//! submit
//!       │
//!       ▼
//! JobId
//!       │
//!       ├── status
//!       ├── queue
//!       ├── cancellation
//!       └── result
//! ```
//!
//! Local simulators may provide synchronous convenience APIs, but the
//! provider-neutral contract must remain capable of representing remote,
//! queued, long-running quantum jobs.
//!
//! # Workload independence
//!
//! Hardware must not assume that every quantum workload is a gate circuit.
//!
//! The architecture supports:
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
//! The authoritative semantic representation remains `quantum::ir`.
//! Interoperability representations such as OpenQASM and QIR are translation
//! targets rather than replacements for the Zamani Quantum IR.
//!
//! # Physical technology versus backend kind
//!
//! These concepts must remain distinct.
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
//! # Capability negotiation
//!
//! A workload must never be submitted blindly.
//!
//! The expected pipeline is:
//!
//! ```text
//! Program requirements
//!          │
//!          ▼
//! Backend capabilities
//!          │
//!          ▼
//! Compatibility analysis
//!          │
//!     ┌────┴─────┐
//!     ▼          ▼
//! compatible   incompatible
//!     │
//!     ▼
//! required transformations
//!     │
//!     ▼
//! executable workload
//! ```
//!
//! Experimental provider capabilities are deliberately distinct from stable
//! capabilities. An experimental feature must never silently satisfy a stable
//! hardware requirement.
//!
//! # Calibration contract
//!
//! Calibration is first-class hardware state.
//!
//! Execution metadata should be capable of preserving:
//!
//! - calibration snapshot identity;
//! - calibration timestamp;
//! - calibration validity interval;
//! - hardware revision;
//! - firmware version;
//! - instruction-set version;
//! - topology version;
//! - adapter version;
//! - provider API version.
//!
//! Hardware code must not silently substitute stale calibration where current
//! calibration is required.
//!
//! # Reproducibility
//!
//! A production execution should be reproducible or auditable to the extent
//! supported by the provider.
//!
//! The hardware boundary therefore exposes contracts for preserving:
//!
//! - workload identity;
//! - backend identity;
//! - job identity;
//! - calibration provenance;
//! - execution options;
//! - deterministic seed where supported;
//! - compiler/adapter versions;
//! - hardware revision;
//! - result provenance.
//!
//! # Security boundary
//!
//! The hardware core never stores plaintext credentials as part of a backend
//! descriptor.
//!
//! Credential handling is divided into:
//!
//! ```text
//! credentials.rs
//!       │
//!       ▼
//! authentication.rs
//!       │
//!       ▼
//! provider adapter
//! ```
//!
//! Hardware metadata and diagnostics must not expose:
//!
//! - API keys;
//! - bearer tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - secret environment values.
//!
//! Secret management remains outside the hardware metadata model.
//!
//! # Error contract
//!
//! Hardware errors must remain structured and machine-readable.
//!
//! Consumers should be able to distinguish:
//!
//! - identity errors;
//! - capability errors;
//! - topology errors;
//! - calibration errors;
//! - validation errors;
//! - authentication errors;
//! - authorization errors;
//! - transport errors;
//! - submission errors;
//! - queue errors;
//! - execution errors;
//! - result errors;
//! - cancellation errors;
//! - serialization errors;
//! - provider errors;
//! - timeout errors.
//!
//! Provider-specific error codes may be preserved as metadata, but provider
//! error types must not leak into the core abstraction.
//!
//! # Determinism
//!
//! Metadata, capability lists, instruction identifiers, registries and other
//! externally observable collections should use deterministic ordering where
//! practical.
//!
//! The module composition itself contains no runtime global state and performs
//! no I/O.
//!
//! # Thread safety
//!
//! This module does not impose a global mutable singleton.
//!
//! Child modules are responsible for their own `Send`/`Sync` guarantees.
//! Registries and execution implementations must explicitly document their
//! concurrency behavior.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! The hardware namespace forbids unsafe Rust.
//!
//! Provider adapters must not bypass the safety boundary through undocumented
//! unsafe abstractions.
//!
//! # Integration with `quantum::mod`
//!
//! The quantum root currently contains an inline hardware module. Once this
//! file exists, the authoritative declaration must be changed from the inline
//! composition to:
//!
//! ```text
//! pub mod hardware;
//! ```
//!
//! No hardware child modules should remain declared inline in
//! `quantum/mod.rs`.
//!
//! This file then becomes the single authoritative composition point for
//! `quantum::hardware`.
//!
//! # Integration with benchmarking
//!
//! `quantum::benchmarking` consumes this namespace.
//!
//! Benchmarking should use:
//!
//! - backend descriptors;
//! - capabilities;
//! - topology;
//! - calibration;
//! - execution;
//! - jobs;
//! - normalized results;
//! - provider/device discovery.
//!
//! The hardware namespace must never import the benchmarking namespace.
//!
//! # Integration with routing
//!
//! `hardware::topology`, `hardware::instruction_set`,
//! `hardware::calibration` and `hardware::timing` provide hardware constraints.
//!
//! The canonical routing algorithms remain in `quantum::routing`.
//!
//! `hardware::routing` exists only for hardware-facing routing constraints and
//! representations.
//!
//! # Integration with scheduling
//!
//! Hardware timing and resource constraints are exposed through:
//!
//! - `timing`;
//! - `calibration`;
//! - `topology`;
//! - `capabilities`;
//! - `scheduling`.
//!
//! The canonical scheduling algorithms remain outside this namespace.
//!
//! # Integration with QEC
//!
//! `logical` exposes hardware-facing logical-qubit capabilities.
//!
//! Quantum error-correction semantics remain owned by
//! `quantum::error_correction`.
//!
//! The hardware layer must not reimplement QEC algorithms.
//!
//! # Integration with Danga
//!
//! Danga may eventually expose commands such as:
//!
//! ```text
//! danga quantum devices
//! danga quantum backends
//! danga quantum discover
//! danga quantum check
//! danga quantum compile
//! danga quantum run
//! danga quantum jobs
//! danga quantum cancel
//! danga quantum results
//! danga quantum benchmark
//! ```
//!
//! Danga must consume this stable hardware API rather than creating a second
//! quantum hardware abstraction.
//!
//! # Interoperability
//!
//! The adapter namespace includes both OpenQASM and QIR interoperability.
//!
//! The intended relationship is:
//!
//! ```text
//!                   Zamani Quantum IR
//!                         │
//!               ┌─────────┴─────────┐
//!               ▼                   ▼
//!           OpenQASM 3             QIR
//!               │                   │
//!               ▼                   ▼
//!         hardware/provider    compiler/provider
//!         interoperability    interoperability
//! ```
//!
//! Neither format becomes the canonical Zamani IR.
//!
//! # API stability policy
//!
//! This module distinguishes three levels:
//!
//! 1. **Stable hardware contracts** — intended for use by the rest of Zamani.
//! 2. **Internal implementation modules** — may evolve without external API
//!    guarantees.
//! 3. **Provider adapters** — provider-specific and independently versioned.
//!
//! Provider additions must not force breaking changes to stable core contracts.
//!
//! # File completion rule
//!
//! This file is complete when:
//!
//! - every hardware implementation file has exactly one declaration here;
//! - adapter modules have exactly one declaration here;
//! - no implementation logic is duplicated here;
//! - no provider SDK is imported here;
//! - no network operation occurs here;
//! - no credentials are handled here;
//! - no benchmark logic occurs here;
//! - the module hierarchy is discoverable from this file;
//! - the public namespace is stable;
//! - Rust 1.97/1.97.1 compatibility is preserved.
//!
//! Individual child modules must be completed independently according to their
//! own contracts. Completing another child module must not require reopening
//! this file merely to add its implementation logic.
//!
//! -----------------------------------------------------------------------------
//! Module graph
//! -----------------------------------------------------------------------------
//!
//! The declarations below are intentionally grouped by architectural role.
//! They contain no runtime initialization and no provider-specific logic.
//!
//! The order is documentation/dependency order; Rust does not require modules
//! to be declared in dependency order.
//!
//! -----------------------------------------------------------------------------
//! Core identity and technology
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Stable identity for hardware, devices, providers, architectures, firmware
/// and revisions.
pub mod identity;

/// Physical quantum technologies and execution models.
pub mod technology;

/// Backend capability declarations and capability status.
pub mod capabilities;

/// Native hardware instruction-set representation.
pub mod instruction_set;

/// Hardware timing, latency and synchronization constraints.
pub mod timing;

/// Physical hardware topology and connectivity.
pub mod topology;

/// Calibration snapshots and hardware-characterization data.
pub mod calibration;

// -----------------------------------------------------------------------------
// Backend contract
// -----------------------------------------------------------------------------

/// Operational backend status.
pub mod backend_status;

/// Backend configuration and execution policy.
pub mod backend_config;

/// Provider-neutral hardware error taxonomy.
pub mod errors;

/// Provider-neutral backend trait and execution contract.
pub mod backend_trait;

/// Canonical backend descriptor and aggregate hardware model.
pub mod backend;

/// Workload/backend compatibility analysis.
pub mod compatibility;

/// Deterministic hardware validation.
pub mod validation;

// -----------------------------------------------------------------------------
// Execution lifecycle
// -----------------------------------------------------------------------------

/// Provider-neutral quantum execution request and execution semantics.
pub mod execution;

/// Quantum job identity and lifecycle.
pub mod job;

/// Queue state and queue metadata.
pub mod queue;

/// Normalized quantum execution results.
pub mod result;

/// Quantum job cancellation semantics.
pub mod cancellation;

// -----------------------------------------------------------------------------
// Provider and device management
// -----------------------------------------------------------------------------

/// Provider abstraction independent of concrete vendor implementations.
pub mod provider;

/// Provider registration and lookup.
pub mod provider_registry;

/// Device/backend registration and lookup.
pub mod device_registry;

/// Hardware/device discovery.
pub mod discovery;

// -----------------------------------------------------------------------------
// Security and operations
// -----------------------------------------------------------------------------

/// Non-secret credential references.
pub mod credentials;

/// Provider-neutral authentication contracts.
pub mod authentication;

/// Backend/device health checks.
pub mod health;

/// Hardware execution telemetry and observability.
pub mod telemetry;

// -----------------------------------------------------------------------------
// Persistence and compilation/execution constraints
// -----------------------------------------------------------------------------

/// Stable hardware-model serialization and deserialization.
pub mod serialization;

/// Hardware-specific routing constraints and representations.
pub mod routing;

/// Hardware-specific scheduling constraints and representations.
pub mod scheduling;

/// Hardware resource, duration and cost estimation.
pub mod resource_estimator;

// -----------------------------------------------------------------------------
// Quantum technology execution models
// -----------------------------------------------------------------------------

/// Pulse-level quantum hardware representation.
pub mod pulse;

/// Analog quantum-control representation.
pub mod analog;

/// Quantum annealing, Ising and QUBO representation.
pub mod annealing;

/// Logical-qubit and fault-tolerant hardware representation.
pub mod logical;

/// Simulator hardware abstraction.
pub mod simulator;

/// Hardware-oriented emulator abstraction.
pub mod emulator;

// -----------------------------------------------------------------------------
// External adapters
// -----------------------------------------------------------------------------
// There is intentionally no dependency on an `adapters/mod.rs` file here.
// Keeping the adapter namespace composed explicitly makes this top-level
// hardware module authoritative and avoids another public composition layer.
//
// All adapters consume the stable contracts above.
// No adapter is allowed to redefine the core hardware model.

/// External provider and interoperability adapters.
///
/// Provider-specific behavior is isolated here. The core hardware modules
/// above must never import these concrete adapters.
pub mod adapters {
    /// Provider-neutral adapter/transport foundation.
    #[path = "adapters/generic.rs"]
    pub mod generic;

    /// OpenQASM interoperability adapter.
    #[path = "adapters/openqasm.rs"]
    pub mod openqasm;

    /// QIR interoperability adapter.
    #[path = "adapters/qir.rs"]
    pub mod qir;

    /// IBM Quantum adapter.
    #[path = "adapters/ibm.rs"]
    pub mod ibm;

    /// IonQ adapter.
    #[path = "adapters/ionq.rs"]
    pub mod ionq;

    /// Amazon Braket adapter.
    #[path = "adapters/aws_braket.rs"]
    pub mod aws_braket;

    /// Rigetti adapter.
    #[path = "adapters/rigetti.rs"]
    pub mod rigetti;

    /// IQM adapter.
    #[path = "adapters/iqm.rs"]
    pub mod iqm;

    /// Quantinuum adapter.
    #[path = "adapters/quantinuum.rs"]
    pub mod quantinuum;

    /// QuEra adapter.
    #[path = "adapters/quera.rs"]
    pub mod quera;
}

// -----------------------------------------------------------------------------
// Controlled public prelude
// -----------------------------------------------------------------------------

/// Stable high-level hardware prelude.
///
/// Consumers that need a concise hardware API may import this namespace
/// instead of depending on the physical file layout.
///
/// The prelude deliberately exposes module boundaries rather than attempting
/// to flatten every hardware type into one namespace.
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

// -----------------------------------------------------------------------------
// Compile-time architectural contracts
// -----------------------------------------------------------------------------

/// Hardware HAL schema identifier.
///
/// This identifier belongs to the namespace boundary rather than to any
/// provider implementation.
pub const HARDWARE_SCHEMA_ID: &str = "zamani.quantum.hardware";

/// Major hardware HAL API version.
///
/// This version describes the composition/API boundary. Individual serialized
/// schemas and providers have their own versions.
pub const HARDWARE_API_MAJOR: u16 = 1;

/// Minor hardware HAL API version.
///
/// Additive, backwards-compatible public functionality increments this value.
pub const HARDWARE_API_MINOR: u16 = 0;

/// Patch hardware HAL API version.
///
/// Bug fixes that do not alter the public semantic contract increment this
/// value.
pub const HARDWARE_API_PATCH: u16 = 0;

/// Complete hardware HAL API version as a static tuple.
///
/// This avoids requiring a version-parsing dependency merely for the module
/// boundary.
pub const HARDWARE_API_VERSION: (u16, u16, u16) = (
    HARDWARE_API_MAJOR,
    HARDWARE_API_MINOR,
    HARDWARE_API_PATCH,
);

/// Rust language edition targeted by this module.
pub const HARDWARE_RUST_EDITION: &str = "2021";

/// Minimum supported Rust version for this module.
pub const HARDWARE_MIN_RUST_VERSION: &str = "1.97";

/// Returns the stable hardware HAL schema identifier.
#[inline]
pub const fn hardware_schema_id() -> &'static str {
    HARDWARE_SCHEMA_ID
}

/// Returns the hardware HAL API version.
#[inline]
pub const fn hardware_api_version() -> (u16, u16, u16) {
    HARDWARE_API_VERSION
}

// -----------------------------------------------------------------------------
// Architectural invariants
// -----------------------------------------------------------------------------

/// Architectural invariant identifiers.
///
/// These are intentionally constants rather than runtime configuration.
/// Changing one represents an architectural decision and therefore requires
/// an explicit source change.
pub mod invariants {
    /// Hardware does not depend on benchmarking.
    pub const NO_BENCHMARKING_DEPENDENCY: &str =
        "hardware-does-not-depend-on-benchmarking";

    /// Hardware does not depend on Danga.
    pub const NO_DANGA_DEPENDENCY: &str =
        "hardware-does-not-depend-on-danga";

    /// Core hardware types are provider neutral.
    pub const PROVIDER_NEUTRAL_CORE: &str =
        "provider-neutral-core";

    /// Provider-specific behavior belongs under adapters.
    pub const PROVIDERS_ISOLATED_IN_ADAPTERS: &str =
        "providers-isolated-in-adapters";

    /// Zamani Quantum IR remains canonical.
    pub const ZAMANI_IR_IS_CANONICAL: &str =
        "zamani-quantum-ir-is-canonical";

    /// OpenQASM is interoperability, not canonical IR.
    pub const OPENQASM_IS_INTEROPERABILITY: &str =
        "openqasm-is-interoperability";

    /// QIR is interoperability, not canonical IR.
    pub const QIR_IS_INTEROPERABILITY: &str =
        "qir-is-interoperability";

    /// Credentials are never part of backend identity.
    pub const CREDENTIALS_NOT_IN_BACKEND_IDENTITY: &str =
        "credentials-not-in-backend-identity";

    /// Hardware execution is capable of asynchronous job lifecycle.
    pub const ASYNC_JOB_LIFECYCLE: &str =
        "asynchronous-job-lifecycle";

    /// Experimental capabilities cannot silently satisfy stable requirements.
    pub const EXPERIMENTAL_CAPABILITIES_EXPLICIT: &str =
        "experimental-capabilities-explicit";

    /// Calibration provenance is preserved.
    pub const CALIBRATION_PROVENANCE: &str =
        "calibration-provenance";

    /// Results preserve execution provenance.
    pub const RESULT_PROVENANCE: &str =
        "result-provenance";
}

// -----------------------------------------------------------------------------
// Namespace smoke tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardware_schema_is_stable() {
        assert_eq!(HARDWARE_SCHEMA_ID, "zamani.quantum.hardware");
    }

    #[test]
    fn hardware_api_version_is_valid() {
        let (major, minor, patch) = HARDWARE_API_VERSION;

        assert!(major >= 1);
        assert!(minor < 1_000);
        assert!(patch < 1_000);
    }

    #[test]
    fn supported_rust_contract_is_explicit() {
        assert_eq!(HARDWARE_RUST_EDITION, "2021");
        assert_eq!(HARDWARE_MIN_RUST_VERSION, "1.97");
    }

    #[test]
    fn architectural_invariants_are_non_empty() {
        let invariants = [
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
        ];

        for invariant in invariants {
            assert!(!invariant.is_empty());
        }
    }

    #[test]
    fn public_prelude_reaches_all_hardware_boundaries() {
        let _ = HARDWARE_SCHEMA_ID;

        // These references intentionally exercise the namespace composition
        // without coupling this module to implementation-specific structs.
        let _ = &prelude::identity;
        let _ = &prelude::technology;
        let _ = &prelude::capabilities;
        let _ = &prelude::instruction_set;
        let _ = &prelude::timing;
        let _ = &prelude::topology;
        let _ = &prelude::calibration;
        let _ = &prelude::backend;
        let _ = &prelude::backend_trait;
        let _ = &prelude::execution;
        let _ = &prelude::job;
        let _ = &prelude::result;
        let _ = &prelude::provider;
        let _ = &prelude::adapters;
    }
}