//! Zamani Quantum Resilience — Telemetry boundary.
//!
//! This module is the stable public boundary for provider-neutral telemetry
//! used by `crate::quantum::resilience`. It owns module composition and API
//! visibility only; telemetry semantics live in the child modules.
//!
//! # Responsibilities
//!
//! The telemetry subsystem represents evidence produced by quantum hardware,
//! the hardware abstraction layer, runtime/execution, compiler and lowering,
//! routing, scheduling, optimization, QEC, simulation, benchmarking, and the
//! resilience subsystem itself. Evidence is consumed by detection, diagnosis,
//! planning, recovery, verification, history, learning, replay, and external
//! observability adapters.
//!
//! This module does NOT:
//!
//! - access hardware;
//! - read clocks;
//! - generate identifiers;
//! - persist telemetry;
//! - perform network I/O;
//! - detect faults;
//! - diagnose incidents;
//! - select recovery strategies;
//! - execute recovery;
//! - implement QEC;
//! - implement mitigation;
//! - implement routing;
//! - implement scheduling;
//! - implement optimization.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Architectural rule: write once, scale everywhere
//!
//! No telemetry module may encode a universal number of:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - events;
//! - metrics;
//! - traces;
//! - labels;
//! - samples;
//! - resources;
//! - execution stages.
//!
//! Concrete limits are supplied by the applicable capability, policy,
//! resource, storage, security, or deployment layer.
//!
//! A concrete process is necessarily finite, but the telemetry API itself
//! imposes no machine-size assumption. This allows the same Zamani program
//! and telemetry contracts to operate from the smallest supported quantum
//! resource to arbitrarily large systems, subject only to available resources,
//! discovered capabilities, and explicit policies.
//!
//! # Canonical quantum identity
//!
//! When telemetry contracts need quantum-qubit identity, the authoritative
//! definitions are under:
//!
//! `crate::quantum::ir::qubit`
//!
//! In particular, logical and physical identities must remain distinct through
//! the canonical `QubitId` and `PhysicalQubitId` types.
//!
//! This module intentionally does not import or redefine those types because
//! this boundary only composes telemetry modules. Child telemetry modules that
//! require quantum identities must depend directly on the canonical IR module.
//!
//! Telemetry must never introduce a competing resilience-local qubit identity.
//!
//! # Evidence is not truth
//!
//! Telemetry is evidence rather than an authoritative resilience decision.
//!
//! An observation may be:
//!
//! - delayed;
//! - duplicated;
//! - stale;
//! - incomplete;
//! - malformed;
//! - correlated with other observations;
//! - produced by degraded infrastructure;
//! - produced by an untrusted source.
//!
//! Detection and diagnosis are responsible for interpreting evidence,
//! correlating observations, evaluating confidence, and determining whether
//! action is justified.
//!
//! # Determinism
//!
//! This module performs no implicit time, randomness, environment, filesystem,
//! or network access.
//!
//! Producer-controlled values such as timestamps, sequence numbers, event
//! identifiers, trace identifiers, and correlation identifiers remain explicit
//! inputs to telemetry contracts.
//!
//! This is required for deterministic replay and reproducible resilience
//! decisions.
//!
//! # Dependency direction
//!
//! ```text
//! hardware / runtime / compiler / routing / scheduling / QEC / simulation
//!                              │
//!                              ▼
//!                         telemetry
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!         detection         history          exporter
//!             │
//!             ▼
//!         diagnosis
//!             │
//!             ▼
//!          planning
//!             │
//!             ▼
//!          recovery
//!             │
//!             ▼
//!        verification
//! ```
//!
//! `telemetry::mod` remains a composition boundary. It must not create
//! orchestration dependencies back into detection, diagnosis, recovery,
//! hardware implementations, or provider adapters.
//!
//! # Child-module contracts
//!
//! ## `event`
//!
//! [`event`] contains immutable discrete observations and lifecycle facts.
//!
//! Producers include hardware, runtime, compiler, IR, routing, scheduling,
//! optimization, QEC, simulation, benchmarking, resilience, and applications.
//!
//! Consumers include detection, diagnosis, history, exporters, replay,
//! provenance, and verification.
//!
//! ## `metric`
//!
//! [`metric`] contains quantitative observations.
//!
//! Metrics can describe execution, resources, hardware behaviour, QEC,
//! mitigation, resilience, latency, reliability, fidelity, and application-
//! defined quantities.
//!
//! Metric definitions must remain extensible and must not assume a particular
//! hardware generation.
//!
//! ## `trace`
//!
//! [`trace`] represents execution and causal/correlation context.
//!
//! Trace identity is separate from event and metric identity. Trace data may
//! connect compilation, lowering, routing, scheduling, execution, QEC,
//! mitigation, detection, recovery, and verification.
//!
//! Trace representation must not become a recovery engine.
//!
//! ## `health`
//!
//! [`health`] provides telemetry-facing resource-health observations.
//!
//! Canonical resilience health semantics remain owned by:
//!
//! `crate::quantum::resilience::model::health`
//!
//! The telemetry health module must not become a second resilience health state
//! machine.
//!
//! ## `collector`
//!
//! [`collector`] provides configurable ingestion/collection of canonical
//! telemetry.
//!
//! Collection capacity, overflow behaviour, duplicate handling, batching,
//! retention, and lifecycle policy are configuration concerns. This boundary
//! imposes no universal telemetry capacity.
//!
//! ## `exporter`
//!
//! [`exporter`] provides provider-neutral export contracts.
//!
//! Export destinations, formats, monitoring systems, and external observability
//! providers must remain integration concerns rather than dependencies of the
//! telemetry model.
//!
//! # Integration with hardware
//!
//! Hardware and provider adapters may emit telemetry through the canonical
//! telemetry contracts.
//!
//! The dependency direction is:
//!
//! ```text
//! hardware / provider adapter
//!             │
//!             ▼
//!       telemetry producer
//!             │
//!             ▼
//!       canonical telemetry
//! ```
//!
//! Telemetry must never directly access provider APIs or hardware.
//!
//! # Integration with runtime and execution
//!
//! Runtime and execution may emit:
//!
//! - execution lifecycle events;
//! - latency observations;
//! - resource observations;
//! - timeout events;
//! - execution failures;
//! - correlation information;
//! - recovery-related observations.
//!
//! Telemetry only represents those observations. Retry, restart, resume,
//! rollback, and migration remain recovery responsibilities.
//!
//! # Integration with QEC
//!
//! QEC may emit:
//!
//! - syndrome observations;
//! - decoder confidence;
//! - logical-error indicators;
//! - leakage;
//! - loss;
//! - erasure;
//! - code performance;
//! - correction performance.
//!
//! QEC remains authoritative for QEC semantics. Telemetry does not implement
//! decoders, codes, syndrome extraction, or correction.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may emit:
//!
//! - mapping changes;
//! - topology observations;
//! - route failures;
//! - resource contention;
//! - schedule changes;
//! - timing observations;
//! - execution latency.
//!
//! Telemetry does not implement routing or scheduling.
//!
//! # Integration with compiler and optimization
//!
//! Compiler and optimization components may emit:
//!
//! - compilation lifecycle;
//! - pass execution;
//! - transformation counts;
//! - cost observations;
//! - compilation latency;
//! - optimization outcomes.
//!
//! Canonical quantum IR remains authoritative for computation semantics.
//!
//! # Integration with detection
//!
//! Detection consumes telemetry as evidence:
//!
//! ```text
//! event
//! metric
//! health
//! trace
//!   │
//!   ▼
//! detection
//!   │
//!   ▼
//! diagnosis
//! ```
//!
//! Detection must consume the public child-module contracts rather than rely
//! on private implementation details.
//!
//! # Integration with diagnosis
//!
//! Diagnosis combines telemetry with other authoritative information,
//! including:
//!
//! - fault semantics;
//! - hardware capabilities;
//! - calibration;
//! - execution state;
//! - history;
//! - QEC state;
//! - routing state;
//! - scheduling state.
//!
//! Telemetry remains observational; diagnosis owns interpretation.
//!
//! # Integration with planning and recovery
//!
//! Planning consumes interpreted evidence and produces recovery plans.
//!
//! Recovery executes those plans.
//!
//! Telemetry does not select or execute recovery actions.
//!
//! This separation prevents an observation from silently becoming an action.
//!
//! # Integration with history and learning
//!
//! History may persist telemetry according to its own storage and retention
//! policy.
//!
//! Learning may consume historical telemetry to estimate future conditions.
//!
//! Neither subsystem may require telemetry to own a particular database,
//! filesystem, cloud service, or retention duration.
//!
//! # Integration with verification
//!
//! Verification may use telemetry as supporting evidence.
//!
//! Telemetry alone must never establish semantic correctness of a recovered
//! quantum computation.
//!
//! Final acceptance belongs to the verification subsystem.
//!
//! # Integration with provenance
//!
//! Telemetry can participate in provenance chains through explicit:
//!
//! - event identifiers;
//! - metric identifiers;
//! - trace identifiers;
//! - execution identifiers;
//! - correlation identifiers;
//! - producer/source identifiers;
//! - timestamps;
//! - sequence numbers.
//!
//! Provenance ownership remains outside this module.
//!
//! # Integration with serialization
//!
//! Canonical resilience serialization and schema compatibility belong under:
//!
//! `crate::quantum::resilience::serialization`
//!
//! This module therefore does not define a competing persistence or wire
//! format.
//!
//! # Collection and export
//!
//! The existing telemetry directory contains both `collector.rs` and
//! `exporter.rs`. They must be explicitly declared here so that Rust includes
//! them in the module tree.
//!
//! Omitting them from this boundary would leave their implementations
//! unreachable through `crate::quantum::resilience::telemetry`.
//!
//! An unbounded collector configuration means that this module imposes no
//! architectural capacity ceiling. It does not claim that a physical process
//! has infinite memory or storage.
//!
//! # Public API policy
//!
//! Child APIs remain namespace-qualified.
//!
//! Prefer:
//!
//! ```text
//! crate::quantum::resilience::telemetry::event::TelemetryEvent
//! crate::quantum::resilience::telemetry::metric::Metric
//! crate::quantum::resilience::telemetry::trace::...
//! crate::quantum::resilience::telemetry::health::...
//! crate::quantum::resilience::telemetry::collector::...
//! crate::quantum::resilience::telemetry::exporter::...
//! ```
//!
//! The boundary intentionally does not flatten every child symbol into one
//! namespace. This prevents naming collisions and keeps module ownership
//! explicit.
//!
//! # No provider-specific branching
//!
//! Core telemetry must not contain logic such as:
//!
//! ```text
//! if backend == ...
//! ```
//!
//! or fixed provider-specific resource assumptions.
//!
//! Provider-specific observations must be translated by provider/HAL adapters
//! before entering the canonical telemetry boundary.
//!
//! # Security
//!
//! Telemetry is not a secret transport.
//!
//! Credentials, private keys, access tokens, authentication headers, and other
//! secret material must not be placed in telemetry metadata.
//!
//! Authentication, integrity, trust, redaction, authorization, and secure
//! transport belong to the applicable security/integration layers.
//!
//! # Scalability
//!
//! The telemetry boundary deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_EVENTS
//! MAX_METRICS
//! MAX_TRACES
//! MAX_DEVICES
//! MAX_BACKENDS
//! MAX_RESOURCES
//! ```
//!
//! Any concrete resource limit must come from the relevant:
//!
//! - hardware capability;
//! - execution policy;
//! - resilience policy;
//! - resource model;
//! - storage policy;
//! - deployment configuration;
//! - security policy;
//! - runtime budget.
//!
//! This allows the same telemetry contracts to operate across heterogeneous
//! quantum systems without rewriting the telemetry API for each machine size.
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
//! - no `unsafe` code.
//!
//! `unsafe` is explicitly forbidden at this module boundary so a future edit
//! cannot silently weaken that guarantee.
//\n
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(rust_2018_idioms)]

/// Discrete telemetry observations and lifecycle facts.
pub mod event;

/// Quantitative telemetry observations.
pub mod metric;

/// Execution, causal, and correlation tracing.
pub mod trace;

/// Observability-facing resource health observations.
pub mod health;

/// Configurable ingestion of canonical telemetry.
pub mod collector;

/// Provider-neutral telemetry export contracts.
pub mod exporter;

/// Stable module namespace identifier.
///
/// This identifies the Rust module boundary only. It is not a hardware,
/// backend, resource, schema, or execution limit.
pub const TELEMETRY_MODULE_ID: &str = "zamani.quantum.resilience.telemetry";

/// Stable module/API boundary version.
///
/// Individual child schemas own their own schema/version contracts. This value
/// only describes the composition boundary represented by this file.
pub const TELEMETRY_MODULE_VERSION: u32 = 1;

/// Returns the canonical Rust-facing telemetry module path.
///
/// This is a compile-time/static diagnostic contract and performs no runtime
/// I/O or state access.
#[must_use]
pub const fn module_path() -> &'static str {
    "quantum::resilience::telemetry"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_child_modules_are_reachable() {
        // Event and metric expose their own stable schema identifiers.
        // Referencing them verifies that the child modules are wired into
        // this module without constructing backend-dependent state.
        let _ = event::TELEMETRY_EVENT_SCHEMA_ID;
        let _ = metric::TELEMETRY_METRIC_SCHEMA_ID;

        // The remaining modules are deliberately checked through their Rust
        // module paths rather than through implementation-specific symbols.
        //
        // `module_path!()` is a built-in compile-time macro and therefore does
        // not access runtime state, hardware, clocks, randomness, or storage.
        let _ = module_path!();

        let _ = TELEMETRY_MODULE_ID;
        let _ = TELEMETRY_MODULE_VERSION;
    }

    #[test]
    fn boundary_contract_is_stable() {
        assert_eq!(
            module_path(),
            "quantum::resilience::telemetry"
        );

        assert_eq!(
            TELEMETRY_MODULE_ID,
            "zamani.quantum.resilience.telemetry"
        );

        assert_eq!(TELEMETRY_MODULE_VERSION, 1);
    }
}