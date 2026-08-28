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
//! - adapter inventory and architecture invariants.
//!
//! It does NOT own:
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
//! - Quantum IR semantics;
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
//!                         Zamani Quantum IR
//!                                |
//!                                v
//!                       compatibility
//!                                |
//!                    +-----------+-----------+
//!                    |                       |
//!                    v                       v
//!                 routing                 scheduling
//!                    |                       |
//!                    +-----------+-----------+
//!                                |
//!                                v
//!                         executable workload
//!                                |
//!                                v
//!                    QuantumBackendAdapter
//!                                |
//!                                v
//!                       adapters::generic
//!                                |
//!             +------------------+------------------+
//!             |                  |                  |
//!             v                  v                  v
//!       interoperability     providers            local
//!             |                  |                  |
//!        +----+----+       +-----+-----+            |
//!        |         |       |     |     |            |
//!        v         v       v     v     v            v
//!     OpenQASM    QIR     IBM   IonQ  Braket     simulator
//!                         |     |      |          emulator
//!                         +-----+------+
//!                                |
//!                                v
//!                               QPU
//! ```
//!
//! # Adapter families
//!
//! The adapter namespace has four families:
//!
//! 1. Generic provider-neutral adapter foundation.
//! 2. Interoperability adapters.
//! 3. Local execution adapters.
//! 4. Provider-specific adapters.
//!
//! The families are intentionally separated so that adding a provider does
//! not alter canonical hardware semantics.
//!
//! # Dependency direction
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
//!      +-----------------------+
//!      |                       |
//!      v                       v
//! interoperability        concrete adapters
//!      |                       |
//!      v             +---------+---------+
//!   OpenQASM         |         |         |
//!      QIR          IBM      IonQ     Braket...
//! ```
//!
//! The forbidden direction is:
//!
//! ```text
//! backend.rs -> adapters::ibm
//! ```
//!
//! The valid direction is:
//!
//! ```text
//! adapters::ibm -> backend.rs
//! ```
//!
//! # Provider-independence invariant
//!
//! Adding a provider MUST NOT require modification of the canonical hardware
//! contracts.
//!
//! In particular, adding a provider must not require changing:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `capabilities.rs`;
//! - `technology.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `result.rs`;
//! - `topology.rs`;
//! - `calibration.rs`;
//! - the canonical Quantum IR.
//!
//! A built-in provider requires only:
//!
//! ```text
//! adapters/<provider>.rs
//! ```
//!
//! plus its module declaration here and registry integration where required.
//!
//! # Execution invariant
//!
//! All executable adapters implement the provider-neutral:
//!
//! ```text
//! QuantumBackendAdapter
//! ```
//!
//! lifecycle:
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
//!    +----> status
//!    |
//!    +----> queue
//!    |
//!    +----> cancel
//!    |
//!    +----> result
//! ```
//!
//! Provider-specific lifecycle states must be normalized by the adapter.
//!
//! An adapter must never report `Completed` while the normalized result is
//! unavailable.
//!
//! # Program formats
//!
//! Adapters may operate on:
//!
//! - `zamani-ir`;
//! - `openqasm-3.x`;
//! - `qir`;
//! - `pulse`;
//! - `analog`;
//! - `annealing`;
//! - `logical`;
//! - provider-native formats.
//!
//! These formats are executable representations, not replacements for the
//! canonical Zamani Quantum IR.
//!
//! # Interoperability
//!
//! OpenQASM and QIR are deliberately separate adapters:
//!
//! ```text
//! Zamani Quantum IR
//!       |
//!       +----------> OpenQASM 3.x
//!       |
//!       +----------> QIR
//! ```
//!
//! OpenQASM is a hardware/interoperability representation.
//!
//! QIR is an LLVM-based compiler/interoperability representation.
//!
//! Neither becomes the canonical Zamani Quantum IR.
//!
//! # Security invariant
//!
//! This module contains no credentials and performs no authentication.
//!
//! It must never contain:
//!
//! - API keys;
//! - access tokens;
//! - refresh tokens;
//! - passwords;
//! - private keys;
//! - authorization headers;
//! - cookies;
//! - secret environment values.
//!
//! Credential references belong to:
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
//! Provider adapters may consume authenticated transport abstractions, but
//! must not become credential stores.
//!
//! # Transport invariant
//!
//! `generic` owns the provider-neutral transport boundary.
//!
//! Concrete adapters translate provider semantics into generic transport
//! requests and responses.
//!
//! This module itself performs no network I/O.
//!
//! # Error invariant
//!
//! Provider-specific errors must be normalized before crossing the adapter
//! boundary.
//!
//! ```text
//! provider error
//!       |
//!       v
//! provider adapter
//!       |
//!       v
//! BackendError
//! ```
//!
//! Provider SDK error types must not leak through the canonical hardware API.
//!
//! # Result invariant
//!
//! Provider-specific results must be normalized into the canonical execution
//! result model.
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
//! Normalization should preserve, when available:
//!
//! - backend identity;
//! - provider job identity;
//! - requested shots;
//! - executable format;
//! - adapter version;
//! - provider API version;
//! - calibration provenance;
//! - execution metadata.
//!
//! # Capability invariant
//!
//! Provider capabilities must be mapped to the canonical hardware capability
//! model.
//!
//! Unknown provider capabilities must never silently become supported Zamani
//! capabilities.
//!
//! Stable and experimental capabilities must remain distinguishable.
//!
//! # Topology invariant
//!
//! Provider topology information may be translated into
//! `HardwareTopology`, but topology semantics remain owned by the topology
//! subsystem.
//!
//! Provider adapters must not implement competing topology models.
//!
//! # Calibration invariant
//!
//! Provider calibration information may be translated into the canonical
//! calibration model.
//!
//! Providers must not redefine calibration semantics.
//!
//! Calibration freshness and validity remain governed by the calibration
//! subsystem.
//!
//! # Local adapter invariant
//!
//! The local adapter must work without:
//!
//! - provider credentials;
//! - provider accounts;
//! - cloud access;
//! - provider SDKs;
//! - physical QPU access.
//!
//! This makes it suitable for CI, deterministic testing, emulation,
//! simulation, failure injection and adapter conformance testing.
//!
//! # Registry integration
//!
//! This module does not own provider registration.
//!
//! Registration belongs to:
//!
//! ```text
//! hardware::provider_registry
//! ```
//!
//! Device indexing belongs to:
//!
//! ```text
//! hardware::device_registry
//! ```
//!
//! Discovery belongs to:
//!
//! ```text
//! hardware::discovery
//! ```
//!
//! Importing this module has no side effects.
//!
//! # Benchmarking integration
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
//! Adapters must never depend on benchmarking.
//!
//! Benchmarking consumes the normalized hardware execution boundary.
//!
//! # Danga integration
//!
//! Danga may eventually expose:
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
//! Danga must consume the canonical hardware APIs rather than implementing
//! provider protocols itself.
//!
//! # No-global-state invariant
//!
//! Importing this module must not:
//!
//! - create network clients;
//! - authenticate;
//! - discover hardware;
//! - submit jobs;
//! - spawn worker threads;
//! - modify global registries;
//! - read credentials;
//! - read execution environment variables.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Public API policy
//!
//! The preferred application-facing execution abstraction is:
//!
//! ```text
//! hardware::backend_trait::QuantumBackendAdapter
//! ```
//!
//! Provider modules remain available for advanced integrations, provider
//! configuration and conformance testing.
//!
//! # Module inventory
//!
//! ## Generic foundation
//!
//! `generic`
//!
//! Provider-neutral transport, request, response, error, pagination,
//! idempotency, capability and format primitives.
//!
//! ## Interoperability
//!
//! `openqasm`
//!
//! Zamani Quantum IR ↔ OpenQASM interoperability.
//!
//! `qir`
//!
//! Zamani Quantum IR ↔ QIR interoperability.
//!
//! ## Local
//!
//! `local`
//!
//! Credential-free local/simulator/emulator execution and deterministic test
//! infrastructure.
//!
//! ## Built-in providers
//!
//! `ibm`
//!
//! IBM Quantum integration.
//!
//! `ionq`
//!
//! IonQ integration.
//!
//! `aws_braket`
//!
//! Amazon Braket integration.
//!
//! `rigetti`
//!
//! Rigetti integration.
//!
//! `iqm`
//!
//! IQM integration.
//!
//! `quantinuum`
//!
//! Quantinuum integration.
//!
//! `quera`
//!
//! QuEra integration.
//!
//! # Provider adapter isolation
//!
//! Provider modules must not depend on one another.
//!
//! For example:
//!
//! ```text
//! ibm.rs -> ionq.rs
//! ```
//!
//! is forbidden.
//!
//! Shared provider-neutral behavior belongs in `generic` or another
//! provider-independent hardware module.
//!
//! # Adapter implementation requirements
//!
//! Every executable adapter must:
//!
//! 1. expose immutable adapter identity;
//! 2. expose backend identity;
//! 3. expose capabilities;
//! 4. perform deterministic preflight;
//! 5. reject unsupported workloads before submission;
//! 6. use the generic transport boundary;
//! 7. normalize provider errors;
//! 8. normalize provider job states;
//! 9. normalize provider results;
//! 10. preserve provenance;
//! 11. support cancellation where the provider supports it;
//! 12. accurately report unsupported cancellation;
//! 13. preserve provider API version;
//! 14. preserve adapter version;
//! 15. avoid secret leakage;
//! 16. avoid network side effects during construction;
//! 17. be safe for concurrent use where its trait contract requires `Sync`;
//! 18. pass the common adapter conformance suite.
//!
//! # Provider additions
//!
//! Adding a new provider follows:
//!
//! ```text
//! 1. create adapters/new_provider.rs
//! 2. implement QuantumBackendAdapter
//! 3. use adapters::generic
//! 4. normalize provider capabilities
//! 5. normalize provider errors
//! 6. normalize provider job lifecycle
//! 7. normalize provider results
//! 8. preserve provenance
//! 9. add provider tests
//! 10. register provider
//! 11. declare module here
//! ```
//!
//! No existing provider adapter should require modification.
//!
//! # Stability
//!
//! This file is a namespace/composition boundary, not an implementation
//! container.
//!
//! Provider-specific implementation belongs in the individual adapter files.
//!
//! Changes to provider implementations must not require changing this file
//! unless the provider is being added, removed, or intentionally hidden from
//! the built-in adapter set.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Provider-neutral generic adapter foundation.
///
/// This module must remain independent from all concrete providers.
pub mod generic;

/// OpenQASM interoperability adapter.
///
/// This is not a provider implementation.
pub mod openqasm;

/// QIR interoperability adapter.
///
/// This is not a provider implementation.
pub mod qir;

/// Local execution adapter.
///
/// Provides credential-free local execution/testing infrastructure.
pub mod local;

/// IBM Quantum adapter.
pub mod ibm;

/// IonQ adapter.
pub mod ionq;

/// Amazon Braket adapter.
pub mod aws_braket;

/// Rigetti adapter.
pub mod rigetti;

/// IQM adapter.
pub mod iqm;

/// Quantinuum adapter.
pub mod quantinuum;

/// QuEra adapter.
pub mod quera;

/// Stable provider-neutral adapter prelude.
///
/// This prelude intentionally exposes contracts rather than provider-specific
/// implementation details.
pub mod prelude {
    pub use super::generic::{
        AdapterIdentity,
        AdapterMetadata,
        GenericAdapterError,
        ProviderOperation,
        ProviderTransport,
        TransportMethod,
    };

    pub use super::super::backend_trait::{
        BackendHealth,
        BackendJob,
        BackendJobId,
        BackendJobState,
        BackendJobStatus,
        BackendProgram,
        BackendQueueInfo,
        QuantumBackendAdapter,
    };
}

/// Returns the stable adapter subsystem schema identifier.
pub const ADAPTERS_SCHEMA_ID: &str = "zamani.quantum.hardware.adapters";

/// Stable semantic schema version for this composition boundary.
///
/// This changes only when the public adapter namespace or its composition
/// contract changes incompatibly.
pub const ADAPTERS_SCHEMA_VERSION: u16 = 1;

/// Returns the canonical adapter subsystem identifier.
#[inline]
pub const fn subsystem_id() -> &'static str {
    ADAPTERS_SCHEMA_ID
}

/// Returns the current adapter subsystem schema version.
#[inline]
pub const fn schema_version() -> u16 {
    ADAPTERS_SCHEMA_VERSION
}

/// Returns whether a built-in adapter family is known to this release.
///
/// This function is deliberately pure and performs no discovery or I/O.
///
/// Provider-specific availability still depends on compilation, configuration,
/// credentials and the provider's current service state.
pub fn is_builtin_adapter(adapter_id: &str) -> bool {
    matches!(
        adapter_id,
        "generic"
            | "openqasm"
            | "qir"
            | "local"
            | "ibm"
            | "ionq"
            | "aws_braket"
            | "rigetti"
            | "iqm"
            | "quantinuum"
            | "quera"
    )
}

/// Returns the deterministic list of built-in adapter identifiers.
///
/// The order is stable and must not be changed casually because consumers may
/// use it for deterministic diagnostics and documentation generation.
pub fn builtin_adapter_ids() -> &'static [&'static str] {
    &[
        "generic",
        "openqasm",
        "qir",
        "local",
        "ibm",
        "ionq",
        "aws_braket",
        "rigetti",
        "iqm",
        "quantinuum",
        "quera",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            subsystem_id(),
            "zamani.quantum.hardware.adapters"
        );
        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn built_in_inventory_is_deterministic() {
        assert_eq!(
            builtin_adapter_ids(),
            &[
                "generic",
                "openqasm",
                "qir",
                "local",
                "ibm",
                "ionq",
                "aws_braket",
                "rigetti",
                "iqm",
                "quantinuum",
                "quera",
            ]
        );
    }

    #[test]
    fn built_in_inventory_contains_only_known_adapters() {
        for adapter in builtin_adapter_ids() {
            assert!(is_builtin_adapter(adapter));
        }
    }

    #[test]
    fn unknown_adapter_is_not_reported_as_built_in() {
        assert!(!is_builtin_adapter("unknown-provider"));
        assert!(!is_builtin_adapter(""));
        assert!(!is_builtin_adapter("IBM"));
    }

    #[test]
    fn adapter_namespace_has_no_runtime_side_effect_contract() {
        // This test intentionally performs no construction, I/O, discovery,
        // authentication or registry mutation. The module is composition
        // only.
        assert!(is_builtin_adapter("local"));
    }
}