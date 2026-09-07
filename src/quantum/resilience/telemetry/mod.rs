//! Zamani Quantum Resilience — Telemetry
//!
//! Path:
//!     src/quantum/resilience/telemetry/mod.rs
//!
//! # Purpose
//!
//! This module is the public module boundary for the resilience telemetry
//! subsystem.
//!
//! Telemetry is responsible for representing and transporting immutable
//! observations about quantum computation and the resources involved in it.
//! It is deliberately separated into four semantic layers:
//!
//! - [`event`]  — discrete observations and lifecycle facts;
//! - [`metric`] — quantitative observations;
//! - [`trace`]  — execution/correlation tracing;
//! - [`health`] — observability-facing resource health.
//!
//! This module performs no telemetry collection, detection, diagnosis,
//! aggregation, persistence, exporting, recovery, hardware access, scheduling,
//! routing, QEC, mitigation, or execution.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!
//!     Zamani Quantum Runtime / Hardware / QEC / Compiler
//!                         │
//!                         │ observations
//!                         ▼
//!              ┌───────────────────────┐
//!              │ quantum::resilience   │
//!              │     ::telemetry       │
//!              └───────────┬───────────┘
//!                          │
//!          ┌───────────────┼────────────────┐
//!          │               │                │
//!          ▼               ▼                ▼
//!       event            metric            trace
//!          │               │                │
//!          └───────────────┼────────────────┘
//!                          ▼
//!                        health
//!                          │
//!          ┌───────────────┼─────────────────────┐
//!          ▼               ▼                     ▼
//!       detection       diagnosis              history
//!          │               │                     │
//!          └───────────────┼─────────────────────┘
//!                          ▼
//!                       planning
//!                          │
//!                          ▼
//!                       recovery
//!                          │
//!                          ▼
//!                      verification
//! ```
//!
//! The telemetry module is therefore a **contract boundary**, not an
//! orchestration engine.
//!
//! # Design principles
//!
//! ## 1. Write once, scale everywhere
//!
//! This module contains no machine-size assumptions.
//!
//! It MUST NOT define universal limits for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - executions;
//! - events;
//! - metrics;
//! - traces;
//! - labels;
//! - samples;
//! - resources;
//! - telemetry sources.
//!
//! Concrete resource and execution limits belong to the applicable policy,
//! capability, resource, security, storage, or deployment layer.
//!
//! Consequently, this module is suitable for workloads ranging from a single
//! quantum degree of freedom to arbitrarily large systems subject only to the
//! resources and policies of the deployment.
//!
//! ## 2. Canonical quantum identity
//!
//! Telemetry submodules that need quantum-qubit identity MUST use the
//! canonical types from:
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
//! No telemetry-local `QubitId`, `PhysicalQubitId`, or equivalent identity
//! type is introduced by this module.
//!
//! This prevents logical and physical quantum identities from diverging across
//! resilience subsystems.
//!
//! ## 3. Provider neutrality
//!
//! The telemetry module MUST NOT contain provider-specific branches such as:
//!
//! ```text
//! if backend == ...
//! ```
//!
//! or provider-specific telemetry models.
//!
//! Provider adapters belong below the hardware/provider integration boundary.
//! They translate provider observations into the canonical telemetry models.
//!
//! ## 4. Immutable data contracts
//!
//! Telemetry observations should be constructed by producers and consumed by
//! downstream components without mutation.
//!
//! Detection and diagnosis interpret telemetry; they do not rewrite historical
//! observations.
//!
//! ## 5. Determinism
//!
//! The telemetry module itself does not:
//!
//! - read the system clock;
//! - generate random identifiers;
//! - access global mutable state;
//! - access environment variables;
//! - access the filesystem;
//! - access the network;
//! - select a backend;
//! - select a recovery strategy.
//!
//! Producer-controlled timestamps, sequences, correlation identifiers and
//! trace identifiers remain explicit inputs to the telemetry data contracts.
//!
//! This permits deterministic replay when the same producer inputs are
//! supplied.
//!
//! ## 6. Telemetry is evidence, not truth
//!
//! An observation may be:
//!
//! - delayed;
//! - duplicated;
//! - incomplete;
//! - stale;
//! - corrupted;
//! - produced by a degraded subsystem;
//! - produced by an untrusted source.
//!
//! Therefore telemetry must not itself be treated as an authoritative
//! resilience decision.
//!
//! Detection and diagnosis are responsible for interpreting evidence,
//! correlating observations, evaluating confidence, and determining whether
//! action is justified.
//!
//! ## 7. Separation of concerns
//!
//! The four child modules have deliberately different responsibilities:
//!
//! ```text
//! event  = discrete observation/fact
//! metric = quantitative observation
//! trace  = causal/correlated execution context
//! health = resource-health observation
//! ```
//!
//! They must not become interchangeable representations.
//!
//! For example, a metric must not silently become an event, and a health
//! observation must not become a recovery decision.
//!
//! # Integration contracts
//!
//! ## `event`
//!
//! [`event`] contains the canonical resilience telemetry event representation.
//!
//! Producers from hardware, runtime, compiler, routing, scheduling,
//! optimization, QEC, simulation, benchmarking, detection, diagnosis,
//! recovery, verification, and application layers may construct events through
//! its public API.
//!
//! Consumers include:
//!
//! - detection;
//! - diagnosis;
//! - history;
//! - trace processing;
//! - telemetry exporters;
//! - deterministic replay;
//! - provenance;
//! - verification.
//!
//! The event module explicitly owns the event data contract; this module only
//! exposes it through the `event` namespace.
//!
//! ## `metric`
//!
//! [`metric`] contains the canonical quantitative telemetry representation.
//!
//! Metrics may describe:
//!
//! - execution behaviour;
//! - hardware behaviour;
//! - logical resources;
//! - physical resources;
//! - workload characteristics;
//! - resilience behaviour;
//! - QEC observations;
//! - mitigation observations;
//! - latency;
//! - reliability;
//! - fidelity;
//! - error-related quantities;
//! - application-defined measurements.
//!
//! Metric names and values remain extensible and must not be tied to a fixed
//! hardware generation.
//!
//! Consumers include:
//!
//! - detection;
//! - diagnosis;
//! - planning;
//! - health evaluation;
//! - benchmarking;
//! - history;
//! - learning;
//! - exporters.
//!
//! ## `trace`
//!
//! [`trace`] provides correlation and execution-trace semantics.
//!
//! Trace identity remains separate from event and metric identity.
//!
//! It supports reconstructing the lifecycle of a computation across:
//!
//! - compilation;
//! - lowering;
//! - routing;
//! - scheduling;
//! - execution;
//! - QEC;
//! - mitigation;
//! - detection;
//! - recovery;
//! - verification.
//!
//! Trace processing must not be confused with recovery orchestration.
//!
//! ## `health`
//!
//! [`health`] provides the telemetry-facing representation of resource health.
//!
//! Canonical health semantics belong to:
//!
//! ```text
//! crate::quantum::resilience::model::health
//! ```
//!
//! The telemetry health layer is therefore an observability representation,
//! not a competing health state machine.
//!
//! This preserves the architecture:
//!
//! ```text
//! model::health
//!       │
//!       │ canonical semantics
//!       ▼
//! telemetry::health
//!       │
//!       ├── event
//!       ├── metric
//!       └── trace
//! ```
//!
//! # Integration with detection
//!
//! Detection consumes telemetry as evidence.
//!
//! Conceptually:
//!
//! ```text
//! telemetry::event
//! telemetry::metric
//! telemetry::health
//! telemetry::trace
//!          │
//!          ▼
//!      detection
//!          │
//!          ▼
//!       diagnosis
//! ```
//!
//! Detection implementations must not depend on private implementation
//! details of the telemetry child modules.
//!
//! They should consume the public contracts exposed by those modules.
//!
//! # Integration with diagnosis
//!
//! Diagnosis combines telemetry with:
//!
//! - fault information;
//! - execution state;
//! - hardware capability information;
//! - calibration information;
//! - history;
//! - QEC observations;
//! - routing/scheduling information.
//!
//! Telemetry remains observational. Diagnosis owns interpretation.
//!
//! # Integration with history
//!
//! History may persist events, metrics, traces, and health observations
//! according to its own retention and storage policies.
//!
//! Telemetry MUST NOT assume:
//!
//! - an in-memory lifetime;
//! - a filesystem;
//! - a database;
//! - a cloud service;
//! - a specific exporter;
//! - infinite retention.
//!
//! # Integration with exporters
//!
//! Exporters consume the canonical telemetry types.
//!
//! This module intentionally does not select or require a monitoring system,
//! vendor, wire format, metrics platform, or tracing provider.
//!
//! Adapter-specific formats belong in the exporter/integration layer.
//!
//! # Integration with serialization
//!
//! Canonical persistence and interchange serialization belongs under:
//!
//! ```text
//! crate::quantum::resilience::serialization
//! ```
//!
//! The telemetry module therefore does not establish a competing serialization
//! format.
//!
//! Schema versioning must be handled by the resilience serialization layer.
//!
//! # Integration with hardware
//!
//! Hardware providers and the hardware HAL may produce telemetry, but
//! telemetry does not access hardware directly.
//!
//! The dependency direction is:
//!
//! ```text
//! hardware / provider
//!        │
//!        ▼
//! telemetry producer
//!        │
//!        ▼
//! resilience telemetry
//! ```
//!
//! Never reverse this dependency by placing hardware access inside this module.
//!
//! # Integration with QEC
//!
//! QEC may emit telemetry describing:
//!
//! - syndrome observations;
//! - decoder confidence;
//! - logical-error indicators;
//! - leakage;
//! - erasure;
//! - loss;
//! - code performance;
//! - recovery performance.
//!
//! The QEC subsystem remains authoritative for QEC semantics.
//!
//! Telemetry only represents observations supplied by QEC.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may emit telemetry about:
//!
//! - mapping changes;
//! - route failures;
//! - resource contention;
//! - schedule changes;
//! - timing observations;
//! - execution latency.
//!
//! Telemetry does not implement routing or scheduling.
//!
//! # Integration with optimization
//!
//! Optimization may emit telemetry about:
//!
//! - pass execution;
//! - transformation counts;
//! - cost changes;
//! - compilation latency;
//! - optimization outcomes.
//!
//! Telemetry does not execute optimization passes.
//!
//! # Integration with verification
//!
//! Verification may use telemetry as supporting evidence.
//!
//! Telemetry MUST NOT by itself establish semantic correctness.
//!
//! The final acceptance of a recovered quantum computation belongs to the
//! verification subsystem.
//!
//! # Integration with provenance
//!
//! Telemetry may participate in provenance chains through explicit event,
//! metric, trace, execution, and correlation identifiers.
//!
//! Provenance ownership remains outside this module.
//!
//! # Public API policy
//!
//! The child modules are exposed by namespace rather than flattening every
//! symbol into this module's namespace.
//!
//! This is intentional.
//!
//! For example:
//!
//! ```text
//! crate::quantum::resilience::telemetry::event::TelemetryEvent
//! crate::quantum::resilience::telemetry::metric::Metric
//! crate::quantum::resilience::telemetry::trace::...
//! crate::quantum::resilience::telemetry::health::...
//! ```
//!
//! Namespace-qualified access:
//!
//! - prevents accidental name collisions;
//! - preserves ownership boundaries;
//! - makes APIs easier to evolve;
//! - avoids coupling callers to implementation details;
//! - supports future child modules without requiring global re-export changes.
//!
//! # Future extensibility
//!
//! Additional telemetry capabilities may be introduced as separate modules
//! when their contracts become sufficiently mature.
//!
//! Examples include:
//!
//! ```text
//! collector
//! exporter
//! aggregation
//! sampling
//! replay
//! redaction
//! ingestion
//!````
//!
//! Such modules must remain independent of this module's orchestration
//! boundary and must not introduce hardware-size constants into the telemetry
//! namespace.
//!
//! A future module should be added here only when it represents a stable,
//! independently testable contract.
//!
//! # No hidden limits
//!
//! This module intentionally contains no:
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
//! Any concrete limits must come from:
//!
//! ```text
//! quantum::resilience::limits
//! quantum::resilience::policy
//! quantum::quantum::hardware capability contracts
//! deployment configuration
//! execution budgets
//! storage policy
//! security policy
//! ```
//!
//! The telemetry module must remain valid regardless of the number of
//! resources represented by the surrounding system.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! canonical quantum IR
//!          │
//!          ├──────────────────────┐
//!          ▼                      ▼
//!   quantum::ir::qubit       hardware/QEC/runtime
//!          │                      │
//!          └──────────┬───────────┘
//!                     ▼
//!             resilience telemetry
//!                     │
//!          ┌──────────┼───────────┐
//!          ▼          ▼           ▼
//!      detection   history    exporters
//!          │
//!          ▼
//!      diagnosis
//!          │
//!          ▼
//!      planning/recovery
//!          │
//!          ▼
//!      verification
//! ```
//!
//! `telemetry/mod.rs` must remain a leaf-style module boundary in this graph:
//! it declares the telemetry modules but does not create an orchestration
//! dependency back into detection, recovery, or hardware.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code so the guarantee remains enforced
//! even if this file is changed later.
//!
//! # Testing
//!
//! Module-level integration tests belong primarily in:
//!
//! ```text
//! src/quantum/resilience/tests/
//! ```
//!
//! Child modules should contain their own focused unit tests where useful.
//!
//! This module only needs lightweight boundary tests to ensure that the
//! declared telemetry modules remain reachable.
//!
//! No test in this module should depend on a particular QPU size, provider,
//! backend, clock, filesystem, network, or random source.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

/// Discrete resilience telemetry observations and lifecycle facts.
///
/// This is the canonical event representation for the telemetry subsystem.
///
/// Integration consumers include detection, diagnosis, history, exporters,
/// replay, provenance, and verification.
pub mod event;

/// Observability-facing resource health telemetry.
///
/// Canonical health semantics remain owned by
/// `crate::quantum::resilience::model::health`.
pub mod health;

/// Quantitative resilience telemetry.
///
/// This module defines metric semantics and values without imposing universal
/// hardware or workload limits.
pub mod metric;

/// Execution and correlation tracing.
///
/// Trace identity remains distinct from event and metric identity.
pub mod trace;

/// Stable namespace for the telemetry subsystem.
///
/// This alias intentionally does not duplicate or flatten child-module APIs.
/// It exists only as a compile-time-visible marker for documentation and
/// downstream tooling that needs to reason about the telemetry boundary.
pub const TELEMETRY_MODULE_PATH: &str =
    "crate::quantum::resilience::telemetry";

/// Current telemetry module contract version.
///
/// This is a module/API contract marker, not a hardware capability or resource
/// limit. Individual telemetry data schemas remain owned by their respective
/// child modules and the canonical resilience serialization layer.
pub const TELEMETRY_MODULE_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_child_modules_are_reachable() {
        // These references deliberately test the public module boundary
        // without depending on concrete telemetry implementation details.
        let _ = event::TELEMETRY_EVENT_SCHEMA_ID;
        let _ = metric::TELEMETRY_METRIC_SCHEMA_ID;

        // The modules themselves are the API boundary for health and trace.
        let _health_module = health::module_path;
        let _trace_module = trace::module_path;
    }

    #[test]
    fn telemetry_module_contract_is_stable() {
        assert_eq!(
            TELEMETRY_MODULE_PATH,
            "crate::quantum::resilience::telemetry"
        );
        assert_eq!(TELEMETRY_MODULE_VERSION, 1);
    }
}