
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Observability
//!
//! Path: `src/ai/observability/mod.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! This module is the composition root for AI-domain observability.
//!
//! It connects AI-specific observation interfaces to the canonical
//! observability infrastructure without implementing another telemetry
//! engine or changing AI computation semantics.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - AI observability module composition.
//! - The public AI observability API boundary.
//! - AI-domain event, metric, and tracing exports.
//! - Documentation of observability integration contracts.
//!
//! It does not own:
//!
//! - AI model execution.
//! - Training or inference execution.
//! - Model lifecycle state.
//! - AI semantic analysis.
//! - AST or IR representations.
//! - Generic event transport.
//! - Generic metrics collection.
//! - Distributed tracing infrastructure.
//! - Hardware discovery.
//! - Resource allocation.
//! - Execution scheduling.
//! - Event persistence.
//! - Security policy enforcement.
//! - Telemetry exporters.
//! - Global mutable telemetry state.
//!
//! # Canonical authorities
//!
//! AI events:
//! `crate::ai::observability::events`
//!
//! AI metrics:
//! `crate::ai::observability::metrics`
//!
//! AI tracing:
//! `crate::ai::observability::tracing`
//!
//! Generic distributed event delivery:
//! `crate::distributed::observability::events`
//!
//! Generic distributed metrics:
//! `crate::distributed::observability::metrics`
//!
//! Generic distributed tracing:
//! `crate::distributed::observability::tracing`
//!
//! AI semantic authority:
//! `crate::frontend::semantic::domains::ai`
//!
//! AI execution authority:
//! The appropriate AI, hybrid, distributed, and runtime subsystems.
//!
//! # POCO-REAF
//!
//! Observability must remain independent of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - FPGA architecture.
//! - ASIC architecture.
//! - QPU architecture.
//! - Accelerator vendors.
//! - Operating systems.
//! - Physical memory size.
//! - Network topology.
//! - Machine count.
//! - Worker count.
//! - Model size.
//! - Tensor rank.
//!
//! Observability describes computation; it does not determine its
//! physical realization.
//!
//! # Scalability
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - AI events.
//! - Metrics.
//! - Metric series.
//! - Traces.
//! - Spans.
//! - Models.
//! - Agents.
//! - Pipelines.
//! - Workers.
//! - Devices.
//! - Machines.
//!
//! Actual resource consumption depends on available resources,
//! configured operational budgets, and the selected observability
//! providers.
//!
//! No implementation can guarantee mathematically infinite memory,
//! storage, or throughput.
//!
//! A resource limit must remain an explicit operational constraint,
//! not a language-level semantic ceiling.
//!
//! # Semantic independence
//!
//! Observability is optional instrumentation.
//!
//! A missing observer, disabled tracing provider, unavailable exporter,
//! or telemetry delivery failure must not silently change the meaning
//! of an AI computation.
//!
//! Telemetry failures must remain distinguishable from computational
//! failures.
//!
//! Applications may enforce separate operational policies for required
//! telemetry delivery. Such policies must be explicit.
//!
//! # Security
//!
//! Observability metadata must not be used as an unrestricted payload
//! storage mechanism.
//!
//! Producers remain responsible for classifying and sanitizing:
//!
//! - Model inputs.
//! - Model outputs.
//! - Prompts.
//! - Dataset contents.
//! - Checkpoint contents.
//! - Credentials.
//! - Authentication tokens.
//! - Private keys.
//! - Sensitive personal information.
//!
//! AI observability interfaces must not implicitly disclose private
//! computational data.
//!
//! # Determinism
//!
//! Event ordering, metric snapshots, and trace propagation are governed
//! by their canonical infrastructure.
//!
//! This composition root does not invent a second identity system,
//! timestamp authority, or distributed causality mechanism.
//!
//! # Integration contract
//!
//! AI computation subsystems may emit observations through the
//! AI-specific interfaces exported here.
//!
//! The interfaces delegate generic delivery and collection to the
//! canonical distributed observability subsystem.
//!
//! The following integration pathway is intended:
//!
//! AI computation
//!     |
//!     v
//! AI event / metric / trace interfaces
//!     |
//!     v
//! Canonical distributed observability
//!     |
//!     v
//! Configured observability provider
//!
//! The composition root does not initialize providers, start background
//! workers, open network connections, or create process-global registries.
//!
//! # Integration status
//!
//! This module composes the existing AI observability source modules.
//!
//! It does not claim that all runtime instrumentation, telemetry export,
//! or provider integrations are complete.
//!
//! Those capabilities must be verified independently.

// ============================================================================
// AI OBSERVABILITY SUBMODULES
// ============================================================================

/// AI-specific structured event infrastructure.
///
/// Event construction and delivery contracts are defined by this module.
/// Generic event transport remains owned by distributed observability.
pub mod events;

/// AI-domain metric interfaces.
///
/// Generic metric storage, collection, and series management remain
/// owned by distributed observability.
pub mod metrics;

/// AI-specific tracing contracts.
///
/// Distributed trace propagation, transport, and export remain owned
/// by the canonical distributed tracing infrastructure.
pub mod tracing;

// ============================================================================
// PUBLIC AI OBSERVABILITY API
// ============================================================================

/// Public AI event interfaces.
///
/// These exports preserve the implementation and ownership of the
/// canonical AI event subsystem.
pub use events::{
    AiEventDomain,
    AiEventError,
    AiEventResult,
};

/// Public AI metrics interfaces.
///
/// Metric collection and storage are delegated to the canonical
/// distributed metrics infrastructure.
pub use metrics::{
    AiMetricName,
    AiMetricUnit,
    AiMetricsError,
    AiMetricsResult,
};

/// Public AI tracing interfaces.
///
/// Trace identity, propagation, and provider behavior remain governed
/// by the AI tracing contracts and distributed infrastructure.
pub use tracing::{
    AiTraceContext,
    AiTraceError,
    AiTraceId,
    AiTraceResult,
};
