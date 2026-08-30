//! Zamani Quantum Optimization — Serialization Boundary
//!
//! Production serialization namespace for the quantum optimization subsystem.
//!
//! # Architectural role
//!
//! `quantum::optimization::serialization` is the interchange boundary for
//! optimization configuration, optimization results/reports, and optimization
//! provenance. It owns serialization composition and public namespace
//! stability; it does not own optimization semantics.
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ├───────────────┬──────────────────┐
//!      ▼               ▼                  ▼
//! config           result             provenance
//!      │               │                  │
//!      ▼               ▼                  ▼
//! serialization::config
//! serialization::report
//! serialization::provenance
//! ```
//!
//! Serialization is therefore a consumer of canonical optimization models.
//! It must never become the owner of those models.
//!
//! # Canonical representation
//!
//! The canonical quantum representation remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! In particular, this module must never introduce a second circuit, operation,
//! qubit, parameter, gate, or identifier representation merely for
//! serialization. If a serialized optimization object contains an actual
//! quantum qubit identity, the canonical type is:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! No `serialization::QubitId`, `serialization::Qubit`, or equivalent shadow
//! type is permitted.
//!
//! The current serialization modules normally operate on aggregate metadata,
//! so they do not need to import `QubitId` directly. That is deliberate: a
//! type should only be imported when it is semantically required.
//!
//! # Submodules
//!
//! ## [`config`]
//!
//! Serializes and deserializes the canonical
//! `optimization::config::OptimizationConfig` using a stable, versioned
//! envelope. It provides compact JSON, pretty JSON, and TOML interfaces.
//!
//! ## [`report`]
//!
//! Serializes the canonical `optimization::result::OptimizationResult` as a
//! stable optimization report. It also provides writer-based APIs so large
//! reports can be streamed without first constructing a complete JSON value.
//!
//! ## [`provenance`]
//!
//! Serializes and deserializes the canonical
//! `optimization::provenance::OptimizationProvenanceSnapshot` using explicit
//! schema/version validation, JSON/TOML support, reader/writer APIs, and
//! deterministic SHA-256 fingerprints.
//!
//! # Ownership boundaries
//!
//! This module owns:
//!
//! - serialization module composition;
//! - serialization namespace stability;
//! - cross-format naming/documentation boundaries;
//! - serialization-level compatibility policy at the namespace level.
//!
//! The child modules own:
//!
//! - `config.rs`: configuration serialization;
//! - `report.rs`: result/report serialization;
//! - `provenance.rs`: provenance serialization.
//!
//! The parent optimization subsystem owns:
//!
//! - optimizer configuration semantics;
//! - pass semantics;
//! - target semantics;
//! - optimization results;
//! - provenance collection;
//! - circuit transformations;
//! - verification;
//! - resource accounting.
//!
//! `quantum::ir` owns quantum semantics and canonical qubit identities.
//!
//! `routing`, `scheduling`, `hardware`, `benchmarking`, and `error_correction`
//! remain separate subsystem owners and are not dependencies of this module.
//!
//! # Stable schema boundaries
//!
//! Each serialized artifact has its own schema namespace and version. The
//! namespace intentionally does not derive its identity from Rust file
//! locations. Consequently, internal file moves do not silently change a
//! persisted format.
//!
//! Current stable schemas are defined by their owning child modules:
//!
//! - `zamani.quantum.optimization.config`;
//! - `zamani.quantum.optimization.report`;
//! - `zamani.quantum.optimization.provenance`.
//!
//! Schema-version decisions belong to those artifact modules, not to this
//! namespace file. Adding a new optimization pass, analysis, target, or
//! verification method therefore does not require this file to change.
//!
//! # Format policy
//!
//! JSON is the primary machine-readable interchange representation.
//!
//! TOML is provided where the owning serializer supports human-authored
//! configuration/interchange use.
//!
//! The namespace deliberately does not claim that all formats are semantically
//! interchangeable byte-for-byte. Each child serializer defines its canonical
//! representation and compatibility rules.
//!
//! # Scalability
//!
//! Zamani is intended to scale from tiny workloads to workloads limited only by
//! available computational and memory resources.
//!
//! This namespace therefore imposes no artificial quantum-workload limit.
//! In particular, this file does not define fixed maxima for:
//!
//! - qubits;
//! - operations;
//! - passes;
//! - diagnostics;
//! - provenance events;
//! - report records;
//! - configuration entries.
//!
//! Resource limits belong to the optimization and IR limit systems, while
//! serialization input/output limits belong to the individual serialization
//! APIs where they can be selected by the caller.
//!
//! For large artifacts, callers should prefer the streaming reader/writer APIs
//! exposed by the child modules instead of converting an entire artifact into
//! a `String` first.
//!
//! This namespace itself performs no allocation proportional to a serialized
//! artifact merely by being imported.
//!
//! # Determinism and reproducibility
//!
//! Serialization composition introduces no timestamps, random values, process
//! IDs, memory addresses, environment variables, filesystem paths, network
//! state, or hidden global state.
//!
//! Deterministic/canonical serialization is defined by the individual artifact
//! serializers. Provenance fingerprints likewise belong to
//! `serialization::provenance`, because that module owns the exact bytes that
//! are hashed.
//!
//! # Security
//!
//! This namespace:
//!
//! - contains no `unsafe` code;
//! - forbids `unsafe` code explicitly;
//! - performs no filesystem I/O;
//! - performs no network I/O;
//! - performs no hardware/QPU I/O;
//! - executes no external processes;
//! - evaluates no executable content;
//! - owns no global mutable state;
//! - does not silently migrate unknown schemas.
//!
//! Child serializers may operate on caller-supplied `Read`/`Write` streams;
//! ownership of those streams remains with the caller.
//!
//! Untrusted-input byte limits are likewise an application concern and are
//! exposed by serializers that parse streams. A caller should always select an
//! appropriate bound when accepting untrusted serialized data.
//!
//! # Integration contract
//!
//! The complete optimization integration is:
//!
//! ```text
//! Zamani source / external quantum format
//!                 │
//!                 ▼
//!         quantum::frontend
//!                 │
//!                 ▼
//!          quantum::ir
//!                 │
//!                 ▼
//!       quantum::optimization
//!          │      │       │
//!          │      │       └── provenance
//!          │      └────────── result
//!          └───────────────── config
//!                 │
//!                 ▼
//! optimization::serialization
//!       ┌─────────┼─────────┐
//!       ▼         ▼         ▼
//!    config     report   provenance
//! ```
//!
//! The optimized canonical circuit continues downstream independently:
//!
//! ```text
//! OptimizationResult.circuit
//!          │
//!          ▼
//! quantum::routing
//!          │
//!          ▼
//! quantum::scheduling
//!          │
//!          ▼
//! quantum::hardware / runtime
//! ```
//!
//! Serialization does not insert itself into that execution path. A compiler
//! may serialize an artifact at any point for persistence, diagnostics,
//! reproducibility, caching, or interchange, but serialization is not a
//! semantic compiler stage.
//!
//! Benchmarking may consume serialized optimization artifacts, but
//! `serialization` must not depend on benchmarking.
//!
//! # Future-proofing
//!
//! New optimization subsystems do not require this file to be edited merely
//! because they are added. For example, adding:
//!
//! - `local.rotation`;
//! - `algebra.phase_polynomial`;
//! - `synthesis.two_qubit`;
//! - `fault_tolerant.t_depth`;
//! - a new target profile;
//! - a new verification strategy;
//!
//! requires changes only to the canonical optimization model and, if that
//! model's externally persisted shape changes, its owning serializer.
//!
//! A future serializer for a genuinely new top-level artifact should be added
//! as a new child module and declared here. It should not modify the existing
//! artifact contracts or duplicate their types.
//!
//! # Public namespace
//!
//! The authoritative paths are:
//!
//! ```text
//! quantum::optimization::serialization::config
//! quantum::optimization::serialization::report
//! quantum::optimization::serialization::provenance
//! ```
//!
//! Keeping these as explicit child modules makes the API discoverable while
//! avoiding a large prelude that would couple callers to implementation types.
//!
//! Callers should import the concrete APIs they need from the relevant child
//! module. This prevents accidental name collisions such as multiple generic
//! `serialize_json` functions being glob-imported into the same scope.
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
//! - no `unsafe` code.
//!
//! It intentionally uses only Rust module composition and therefore introduces
//! no additional Cargo dependency.
//!
//! # Integration with `quantum::ir::qubit`
//!
//! The canonical IR declares the qubit implementation as `quantum::ir::qubit`
//! rather than `quantum::ir::qubits`. Serialization must follow that canonical
//! name whenever an individual qubit identity is eventually serialized.
//!
//! This is a namespace-level invariant and is documented here in advance so
//! later serialization work cannot accidentally recreate the repository's
//! historical `qubits` naming inconsistency.
//!
//! # No circular dependency
//!
//! The intended dependency graph is:
//!
//! ```text
//! optimization model ───────► serialization
//!          │                       │
//!          ▼                       ▼
//!       quantum::ir            formats
//! ```
//!
//! Never introduce:
//!
//! ```text
//! serialization ───► optimization::serialization
//! optimization ────► serialization model ───► optimization
//! ```
//!
//! The child serializers must import canonical optimization types directly;
//! this `mod.rs` must remain a composition boundary only.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Authoritative serialization modules
// =============================================================================

/// Serialization of `OptimizationConfig`.
pub mod config;

/// Serialization of `OptimizationResult` as an optimization report.
pub mod report;

/// Serialization of `OptimizationProvenanceSnapshot`.
pub mod provenance;

// =============================================================================
// Controlled prelude
// =============================================================================

/// Stable module-level prelude.
///
/// The prelude deliberately re-exports modules rather than every function and
/// type from those modules. This avoids collisions between similarly named
/// operations such as `serialize_json` while still giving callers a concise,
/// stable entry point.
pub mod prelude {
    pub use super::{config, provenance, report};
}

// =============================================================================
// Architectural smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_serialization_boundaries_are_reachable() {
        assert_eq!(
            config::CONFIG_SCHEMA,
            "zamani.quantum.optimization.config"
        );

        assert_eq!(
            report::REPORT_SCHEMA,
            "zamani.quantum.optimization.report"
        );

        assert_eq!(
            provenance::PROVENANCE_SCHEMA,
            "zamani.quantum.optimization.provenance"
        );
    }

    #[test]
    fn serialization_schemas_have_current_versions() {
        assert!(config::CURRENT_SCHEMA_VERSION >= 1);
        assert!(config::FORMAT_VERSION >= 1);

        assert!(report::CURRENT_SCHEMA_VERSION >= 1);
        assert!(report::FORMAT_VERSION >= 1);

        assert!(provenance::CURRENT_SCHEMA_VERSION >= 1);
        assert!(provenance::FORMAT_VERSION >= 1);
    }

    #[test]
    fn serialization_namespace_has_no_runtime_side_effects() {
        // This test intentionally performs no I/O and constructs no global
        // state. Its purpose is to keep this module a pure namespace boundary.
        let _ = prelude::config::ConfigSerializationFormat::Json;
        let _ = prelude::provenance::ProvenanceSerializationFormat::Json;
        let _ = prelude::report::REPORT_KIND;
    }
}