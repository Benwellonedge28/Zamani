//! Zamani Quantum — Hardware Adapter Boundary
//!
//! Production-grade composition boundary for:
//!
//! `crate::quantum::hardware::adapters`
//!
//! # Responsibility
//!
//! This module is the authoritative namespace and composition boundary for
//! quantum-hardware adapters in Zamani.
//!
//! It owns:
//!
//! - adapter module composition;
//! - stable adapter namespace organization;
//! - provider-neutral adapter exports;
//! - interoperability adapter organization;
//! - provider adapter organization;
//! - adapter-family documentation;
//! - compile-time visibility boundaries;
//! - the public adapter prelude;
//! - architectural invariants for adapter dependencies.
//!
//! It deliberately does NOT own:
//!
//! - backend semantics;
//! - backend identity;
//! - provider registries;
//! - device registries;
//! - job orchestration;
//! - execution orchestration;
//! - credentials;
//! - authentication;
//! - HTTP/TLS implementation;
//! - provider SDK clients;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - topology algorithms;
//! - benchmarking;
//! - quantum IR semantics;
//! - frontend parsing;
//! - optimization;
//! - error-correction algorithms;
//! - simulator implementation;
//! - emulator implementation.
//!
//! Those responsibilities belong to the surrounding hardware and quantum
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                     Zamani Quantum IR
//!                            |
//!                            v
//!                 compatibility analysis
//!                            |
//!                   +--------+--------+
//!                   |                 |
//!                   v                 v
//!                routing          scheduling
//!                   |                 |
//!                   +--------+--------+
//!                            |
//!                            v
//!                    BackendProgram
//!                            |
//!                            v
//!              QuantumBackendAdapter
//!                            |
//!                            v
//!              hardware::adapters::generic
//!                            |
//!          +-----------------+------------------+
//!          |                 |                  |
//!          v                 v                  v
//!     interoperability   provider adapters    local
//!       adapters                              adapter
//!          |                 |                  |
//!          |        +--------+--------+         |
//!          |        |        |        |         |
//!          v        v        v        v         v
//!       OpenQASM   IBM     IonQ   Braket     simulator
//!       QIR        Rigetti IQM    Quantinuum emulator
//!                  QuEra
//! ```
//!
//! # Adapter families
//!
//! The adapter namespace is deliberately divided conceptually into four
//! families.
//!
//! ## 1. Generic adapter foundation
//!
//! `generic`
//!
//! Defines reusable provider-neutral transport and adapter primitives.
//!
//! Concrete adapters depend on it.
//!
//! `generic` MUST NOT depend on concrete adapters.
//!
//! ## 2. Interoperability adapters
//!
//! `openqasm`
//!
//! `qir`
//!
//! These adapters translate between Zamani's canonical quantum representation
//! and external interoperability formats.
//!
//! They are NOT quantum providers.
//!
//! In particular:
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        +---------> OpenQASM 3.x
//!        |
//!        +---------> QIR
//! ```
//!
//! Neither OpenQASM nor QIR becomes the canonical Zamani Quantum IR.
//!
//! ## 3. Local execution adapter
//!
//! `local`
//!
//! Provides the provider-independent local execution boundary used for:
//!
//! - local development;
//! - CI;
//! - deterministic testing;
//! - simulator execution;
//! - emulator execution;
//! - failure injection;
//! - adapter conformance testing;
//! - development without provider credentials.
//!
//! The local adapter is particularly important because the hardware subsystem
//! must be testable without access to physical QPUs.
//!
//! ## 4. Provider adapters
//!
//! Provider-specific adapters currently include:
//!
//! - `ibm`;
//! - `ionq`;
//! - `aws_braket`;
//! - `rigetti`;
//! - `iqm`;
//! - `quantinuum`;
//! - `quera`.
//!
//! Provider-specific behavior MUST remain inside its provider module.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! hardware core
//!      |
//!      v
//! backend_trait
//!      |
//!      v
//! adapters::generic
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! interoperability                provider/local
//! adapters                        adapters
//!      |                               |
//!      v                               v
//! openqasm / qir              IBM / IonQ / Braket / ...
//! ```
//!
//! Concrete provider adapters MUST NOT become dependencies of the canonical
//! backend model.
//!
//! The following dependency is forbidden:
//!
//! ```text
//! backend.rs -> adapters::ibm
//! ```
//!
//! The correct direction is:
//!
//! ```text
//! adapters::ibm -> backend.rs
//! ```
//!
//! # Provider-independence invariant
//!
//! Adding a provider MUST NOT require modifying:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `generic.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `result.rs`;
//! - `topology.rs`;
//! - `calibration.rs`;
//! - the canonical Quantum IR.
//!
//! A new provider should normally require:
//!
//! ```text
//! adapters/new_provider.rs
//! ```
//!
//! plus registration/discovery configuration in the appropriate registry
//! subsystem.
//!
//! This module then receives one additional module declaration when the
//! provider is intentionally shipped as a built-in adapter.
//!
//! # Stable execution boundary
//!
//! All executable adapters ultimately target the provider-neutral contract:
//!
//! ```text
//! QuantumBackendAdapter
//! ```
//!
//! The lifecycle is:
//!
//! ```text
//! preflight
//!    |
//!    v
//! submit
//!    |
//!    v
//! BackendJobId
//!    |
//!    +------> status
//!    |
//!    +------> cancel
//!    |
//!    +------> result
//! ```
//!
//! An adapter MUST NOT invent an independent lifecycle model.
//!
//! Provider-specific lifecycle states must be normalized by the provider
//! adapter into the provider-neutral hardware execution contract.
//!
//! # Program-format boundary
//!
//! Adapters may consume several executable representations:
//!
//! ```text
//! zamani-ir
//! openqasm-3.x
//! qir
//! pulse
//! analog
//! annealing
//! logical
//! provider-native
//! ```
//!
//! The adapter namespace does not define the semantic meaning of those
//! representations. Their owning modules define those semantics.
//!
//! # Security invariant
//!
//! This module contains no credential material.
//!
//! In particular, this module must never introduce:
//!
//! ```text
//! api_key
//! access_token
//! refresh_token
//! password
//! private_key
//! authorization_header
//! cookie
//! ```
//!
//! Credentials belong to:
//!
//! ```text
//! hardware::credentials
//! ```
//!
//! Authentication belongs to:
//!
//! ```text
//! hardware::authentication
//! ```
//!
//! Provider adapters may consume authenticated transport/session abstractions,
//! but they must not become credential stores.
//!
//! # Transport invariant
//!
//! `adapters::generic` owns the provider-neutral transport boundary.
//!
//! Concrete provider adapters translate provider semantics into generic
//! requests/responses.
//!
//! The provider adapter itself must not become an HTTP client.
//!
//! Conceptually:
//!
//! ```text
//! provider adapter
//!       |
//!       v
//! generic transport contract
//!       |
//!       v
//! HTTP / SDK / RPC / local transport
//! ```
//!
//! This allows the actual transport implementation to evolve independently
//! from provider semantic mappings.
//!
//! # Error invariant
//!
//! Provider-specific errors must be normalized before they cross the adapter
//! boundary.
//!
//! The desired direction is:
//!
//! ```text
//! Provider error
//!      |
//!      v
//! provider adapter
//!      |
//!      v
//! provider-neutral BackendError
//! ```
//!
//! Provider SDK error types must never become part of the public canonical
//! hardware API.
//!
//! # Result invariant
//!
//! Provider-specific result formats must be normalized into the canonical
//! execution-result model.
//!
//! The direction is:
//!
//! ```text
//! provider result
//!       |
//!       v
//! adapter normalization
//!       |
//!       v
//! ExecutionResult
//! ```
//!
//! Normalization must preserve provenance where supported, including:
//!
//! - backend identity;
//! - provider job identity;
//! - requested shots;
//! - executable format;
//! - adapter version;
//! - provider API version;
//! - calibration information;
//! - execution metadata.
//!
//! # Capability invariant
//!
//! Provider-specific capabilities must be translated into the canonical
//! hardware capability model.
//!
//! Unknown provider capabilities MUST NOT silently become supported Zamani
//! capabilities.
//!
//! Experimental capabilities must remain distinguishable from stable
//! capabilities.
//!
//! Conceptually:
//!
//! ```text
//! provider capability
//!       |
//!       v
//! adapter normalization
//!       |
//!       +---- stable
//!       |
//!       +---- experimental
//!       |
//!       +---- unknown
//!       |
//!       v
//! BackendCapabilities
//! ```
//!
//! # Topology invariant
//!
//! Adapters may translate provider topology information into the canonical
//! `HardwareTopology` representation.
//!
//! They must not redefine topology semantics.
//!
//! Topology algorithms remain owned by the hardware topology/routing
//! subsystems.
//!
//! # Calibration invariant
//!
//! Adapters may retrieve and normalize provider calibration information.
//!
//! They must not redefine the canonical calibration model.
//!
//! Calibration freshness, provenance, validity and policy remain controlled by
//! the hardware calibration subsystem.
//!
//! # OpenQASM invariant
//!
//! `openqasm` is an interoperability adapter.
//!
//! It must not become the canonical quantum language representation for
//! Zamani.
//!
//! The intended direction is:
//!
//! ```text
//! Zamani Quantum IR <-> OpenQASM 3.x
//! ```
//!
//! Provider adapters may consume the OpenQASM representation when their
//! provider protocol supports it.
//!
//! # QIR invariant
//!
//! `qir` is an interoperability/compiler boundary.
//!
//! It must not replace Zamani Quantum IR.
//!
//! The intended architecture is:
//!
//! ```text
//! Zamani Quantum IR
//!          |
//!          v
//!         QIR
//!          |
//!     +----+----+
//!     |         |
//!     v         v
//! providers   compiler ecosystem
//! ```
//!
//! # Local adapter invariant
//!
//! The local adapter must remain usable without:
//!
//! - cloud credentials;
//! - provider accounts;
//! - provider network access;
//! - provider SDKs;
//! - physical QPU access.
//!
//! This makes it the foundation for deterministic CI and adapter conformance
//! testing.
//!
//! # Provider adapter isolation
//!
//! Provider modules must not import one another merely to share provider
//! behavior.
//!
//! For example:
//!
//! ```text
//! ibm.rs -> ionq.rs
//! ```
//!
//! is forbidden unless there is a genuinely provider-neutral abstraction that
//! belongs in `generic.rs` or another appropriate core module.
//!
//! Provider-specific shared behavior must not be hidden inside an unrelated
//! provider module.
//!
//! # Registry integration
//!
//! This module does NOT own provider registration.
//!
//! The intended architecture is:
//!
//! ```text
//! adapters
//!     |
//!     v
//! provider_registry
//!     |
//!     v
//! device_registry
//! ```
//!
//! `provider_registry.rs` may register concrete adapter implementations.
//!
//! `device_registry.rs` may index discovered backend/device identities.
//!
//! Neither registry should require provider-specific types in its public
//! canonical API.
//!
//! # Discovery integration
//!
//! Discovery is owned by:
//!
//! ```text
//! hardware::discovery
//! ```
//!
//! Adapters may implement provider-specific discovery operations, but the
//! adapter namespace itself does not perform discovery automatically.
//!
//! Importing this module MUST have no network side effects.
//!
//! # Execution integration
//!
//! Execution orchestration is owned by:
//!
//! ```text
//! hardware::execution
//! ```
//!
//! The adapter namespace only provides implementations of the executable
//! adapter contract.
//!
//! The adapter must never start network execution merely because its module is
//! imported.
//!
//! # Benchmarking integration
//!
//! Benchmarking consumes adapters through the provider-neutral hardware
//! execution boundary.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::benchmarking
//!          |
//!          v
//! quantum::hardware
//!          |
//!          v
//! quantum::hardware::adapters
//! ```
//!
//! Hardware adapters MUST NOT depend on benchmarking.
//!
//! This preserves the repository's architectural rule that benchmarking is a
//! consumer/orchestration subsystem rather than a dependency of lower-level
//! quantum execution.
//!
//! # Danga integration
//!
//! Danga, the Zamani project/package/toolchain manager, may expose quantum
//! hardware operations such as:
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
//! ```
//!
//! Danga must access adapters through the canonical hardware/backend APIs.
//!
//! Danga must not directly implement provider protocols.
//!
//! # No-global-state invariant
//!
//! Merely importing this module must not:
//!
//! - create network clients;
//! - authenticate;
//! - discover devices;
//! - submit jobs;
//! - spawn worker threads;
//! - modify global registries;
//! - read credentials;
//! - read environment variables for execution.
//!
//! Construction and lifecycle remain explicit.
//!
//! # Thread-safety
//!
//! This module itself contains no mutable shared state.
//!
//! Concrete adapter implementations are responsible for satisfying the
//! `Send`/`Sync` requirements of `QuantumBackendAdapter` where applicable.
//!
//! # Rust compatibility
//!
//! Supported toolchains:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Safety
//!
//! No unsafe Rust is permitted in this module.
//!
//! # Public API policy
//!
//! The adapter modules themselves are public so that provider registries,
//! conformance suites, tests and advanced integrations can reference them.
//!
//! The preferred high-level execution API remains the provider-neutral
//! `QuantumBackendAdapter` contract.
//!
//! Consumers should therefore prefer:
//!
//! ```text
//! quantum::hardware::backend_trait
//! ```
//!
//! rather than coupling application code directly to a provider module.
//!
//! # Adding a new provider
//!
//! A new built-in provider adapter should follow this sequence:
//!
//! 1. Create `new_provider.rs`.
//! 2. Implement `QuantumBackendAdapter`.
//! 3. Reuse `generic` transport/adapter primitives.
//! 4. Normalize provider errors into the canonical error model.
//! 5. Normalize capabilities into the canonical capability model.
//! 6. Normalize backend metadata.
//! 7. Normalize job lifecycle.
//! 8. Normalize results.
//! 9. Preserve execution provenance.
//! 10. Add provider-specific conformance tests.
//! 11. Register it through `provider_registry`.
//! 12. Add one module declaration here.
//!
//! No existing provider module should need modification merely because a new
//! provider was introduced.
//!
//! # Built-in adapter inventory
//!
//! The current built-in adapter set is:
//!
//! ```text
//! generic
//! openqasm
//! qir
//! local
//! ibm
//! ionq
//! aws_braket
//! rigetti
//! iqm
//! quantinuum
//! quera
//! ```
//!
//! # Module declarations
//!
//! The order below is intentional for documentation and dependency clarity.
//! Rust does not require modules to be declared in dependency order, but the
//! namespace is organized from foundational abstractions toward concrete
//! implementations.
//!
//! =============================================================================
//! Foundational adapter contract
//! =============================================================================

/// Provider-neutral adapter and transport foundation.
///
/// Concrete adapters should build on this module rather than implementing
/// duplicate request, response, error, identifier and transport primitives.
pub mod generic;

// =============================================================================
// Interoperability adapters
// =============================================================================

/// OpenQASM interoperability adapter.
///
/// This is a format adapter, not a quantum hardware provider.
pub mod openqasm;

/// QIR interoperability adapter.
///
/// This is a compiler/intermediate-representation adapter, not a hardware
/// provider.
pub mod qir;

// =============================================================================
// Local execution
// =============================================================================

/// Local/simulator/emulator execution adapter.
///
/// This adapter is intentionally provider-independent and is suitable for
/// deterministic development and CI.
pub mod local;

// =============================================================================
// Built-in provider adapters
// =============================================================================

/// IBM Quantum provider adapter.
pub mod ibm;

/// IonQ provider adapter.
pub mod ionq;

/// Amazon Braket provider adapter.
///
/// Braket is represented as one provider adapter because it exposes multiple
/// heterogeneous quantum device families behind its task model.
pub mod aws_braket;

/// Rigetti provider adapter.
pub mod rigetti;

/// IQM provider adapter.
pub mod iqm;

/// Quantinuum provider adapter.
pub mod quantinuum;

/// QuEra provider adapter.
pub mod quera;

// =============================================================================
// Stable adapter prelude
// =============================================================================

/// Stable adapter namespace for advanced hardware integrations.
///
/// The prelude intentionally exports adapter modules rather than blindly
/// re-exporting every provider-specific type. This prevents provider-specific
/// implementation details from becoming part of the canonical hardware API.
///
/// High-level consumers should normally use:
///
/// ```text
/// crate::quantum::hardware::backend_trait
/// ```
///
/// and use this prelude only when adapter-level access is actually required.
pub mod prelude {
    pub use super::aws_braket;
    pub use super::generic;
    pub use super::ibm;
    pub use super::ionq;
    pub use super::iqm;
    pub use super::local;
    pub use super::openqasm;
    pub use super::qir;
    pub use super::quera;
    pub use super::quantinuum;
    pub use super::rigetti;
}

// =============================================================================
// Architectural documentation constants
// =============================================================================

/// Stable namespace identifier for the adapter subsystem.
pub const ADAPTERS_NAMESPACE_ID: &str =
    "zamani.quantum.hardware.adapters";

/// Semantic version of the adapter namespace contract.
///
/// This version describes the namespace/composition contract, not individual
/// provider API versions. Provider adapters maintain their own versions.
pub const ADAPTERS_NAMESPACE_VERSION: u16 = 1;

/// Stable identifiers for built-in adapter families.
///
/// These identifiers are intentionally data-only and have no execution side
/// effects.
pub mod family {
    /// Generic provider/transport adapter family.
    pub const GENERIC: &str = "generic";

    /// OpenQASM interoperability family.
    pub const OPENQASM: &str = "openqasm";

    /// QIR interoperability family.
    pub const QIR: &str = "qir";

    /// Local execution family.
    pub const LOCAL: &str = "local";

    /// IBM provider family.
    pub const IBM: &str = "ibm";

    /// IonQ provider family.
    pub const IONQ: &str = "ionq";

    /// Amazon Braket provider family.
    pub const AWS_BRAKET: &str = "aws_braket";

    /// Rigetti provider family.
    pub const RIGETTI: &str = "rigetti";

    /// IQM provider family.
    pub const IQM: &str = "iqm";

    /// Quantinuum provider family.
    pub const QUANTINUUM: &str = "quantinuum";

    /// QuEra provider family.
    pub const QUERA: &str = "quera";
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================

/// Returns the canonical adapter namespace identifier.
///
/// This is deliberately a pure function so callers and integration tests can
/// verify the namespace contract without constructing an adapter or causing
/// any I/O.
#[inline]
pub const fn namespace_id() -> &'static str {
    ADAPTERS_NAMESPACE_ID
}

/// Returns the semantic version of the adapter namespace contract.
#[inline]
pub const fn namespace_version() -> u16 {
    ADAPTERS_NAMESPACE_VERSION
}

/// Returns the stable identifier for a built-in adapter family.
///
/// Unknown families return `None` rather than being guessed.
///
/// This function is intentionally pure and performs no registration,
/// discovery, authentication or network activity.
pub const fn family_id(name: &str) -> Option<&'static str> {
    match name {
        family::GENERIC => Some(family::GENERIC),
        family::OPENQASM => Some(family::OPENQASM),
        family::QIR => Some(family::QIR),
        family::LOCAL => Some(family::LOCAL),
        family::IBM => Some(family::IBM),
        family::IONQ => Some(family::IONQ),
        family::AWS_BRAKET => Some(family::AWS_BRAKET),
        family::RIGETTI => Some(family::RIGETTI),
        family::IQM => Some(family::IQM),
        family::QUANTINUUM => Some(family::QUANTINUUM),
        family::QUERA => Some(family::QUERA),
        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_contract_is_stable() {
        assert_eq!(
            namespace_id(),
            "zamani.quantum.hardware.adapters"
        );

        assert_eq!(namespace_version(), 1);
    }

    #[test]
    fn all_builtin_families_have_stable_identifiers() {
        let families = [
            family::GENERIC,
            family::OPENQASM,
            family::QIR,
            family::LOCAL,
            family::IBM,
            family::IONQ,
            family::AWS_BRAKET,
            family::RIGETTI,
            family::IQM,
            family::QUANTINUUM,
            family::QUERA,
        ];

        for family_name in families {
            assert_eq!(family_id(family_name), Some(family_name));
        }
    }

    #[test]
    fn unknown_family_is_not_silently_accepted() {
        assert_eq!(family_id("unknown-provider"), None);
    }
}