
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Universal Intermediate Representation — Lowering
//!
//! Canonical path: `src/zuir/lowering/mod.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97 or later.
//! Safety policy: `unsafe` Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR lowering.
//! Compose the lowering API, context, legality analysis, diagnostics
//! adapter, and provenance adapter without duplicating their logic.
//!
//! Lowering converts an explicitly described computational meaning into
//! another representation only when the applicable semantic contracts
//! permit the conversion.
//!
//! ## STATUS
//!
//! Proposed replacement facade. Do not mark production-verified until
//! the parent ZUIR module, all declared child modules, and repository
//! integration tests compile and pass.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/`
//! - `src/zuir/verify/`
//! - `src/zuir/lowering/api.rs`
//! - `src/zuir/lowering/context.rs`
//! - `src/zuir/lowering/legality.rs`
//! - `src/zuir/lowering/diagnostics.rs`
//! - `src/zuir/lowering/provenance.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - Lowering submodule declarations.
//! - The public composition boundary for lowering.
//! - Deliberate re-exports of supported lowering APIs.
//! - Documentation of module ownership and dependency direction.
//!
//! ## DOES_NOT_OWN
//!
//! - Legality algorithms or semantic-preservation proofs.
//! - Lowering context state or mapping algorithms.
//! - Backend implementations or domain-specific conversion algorithms.
//! - Canonical diagnostic construction or source-map storage.
//! - IR verification, type inference, or dialect registration.
//! - Target discovery, physical allocation, routing, scheduling,
//!   synthesis, code generation, or runtime execution.
//!
//! ## MODULE CONTRACTS
//!
//! - `api` owns lowering orchestration and backend invocation.
//! - `context` owns lowering-local state, entity mappings, requirements,
//!   provenance records, policies, and lowering notes.
//! - `legality` owns the decision about whether direct lowering is
//!   justified by the evidence supplied by the caller.
//! - `diagnostics` adapts lowering records to canonical ZUIR diagnostics.
//! - `provenance` bridges lowering-local origins to the canonical
//!   source map through explicit identity resolution.
//!
//! Each child module owns its implementation and error contracts.
//! This facade must not recreate those types or their algorithms.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! ZUIR specification, verified IR, and dialect contracts
//!                         |
//!                         v
//!                 lowering::legality
//!                         |
//!                         v
//!                  lowering::api
//!                         |
//!                         v
//!              Domain-specific backends
//!
//! lowering::context --------> lowering::api
//! lowering::diagnostics -----> canonical diagnostics
//! lowering::provenance ------> canonical source map
//! ```
//!
//! The diagram describes ownership and intended dependency direction;
//! it does not require unrelated child modules to depend on this facade.
//!
//! ## SEMANTIC SAFETY
//!
//! A legality decision authorizing direct lowering does not, by itself,
//! prove physical target feasibility or successful execution.
//!
//! Callers must provide authoritative evidence from the relevant semantic
//! analysis, verifier, and dialect contracts. Missing or unresolved
//! evidence must not be interpreted as proof of legality.
//!
//! Unsupported, conditional, deferred, or expansion-required conversions
//! must not be reported as completed direct lowerings.
//!
//! Lowering must not silently discard effects, ownership constraints,
//! numerical guarantees, control dependencies, quantum measurement
//! semantics, HDL semantics, resource requirements, or provenance.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Source syntax, lexical rules, parsing, and AST construction remain
//! owned by the grammar and frontend.
//!
//! The semantic model establishes what the program means. ZUIR lowering
//! consumes that meaning and does not parse grammar productions or
//! introduce domain-specific source keywords.
//!
//! Adding a lowering backend must not require this facade to know every
//! operation, gate, instruction, device, or hardware family.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This facade introduces no fixed limits on operations, qubits, registers,
//! processors, devices, nodes, memory, tensor rank, or domain count.
//!
//! The facade delegates collection management and work accounting to its
//! owning modules. Real memory, address-space, time, policy, and hardware
//! constraints remain explicit operational constraints.
//!
//! No finite implementation can guarantee literal infinity. Resource
//! exhaustion or unsupported target capabilities must be reported rather
//! than handled by truncating input or silently changing program meaning.
//!
//! ## COMPATIBILITY
//!
//! Public re-exports are part of the Rust API. Removing or changing an
//! existing re-export requires an explicit compatibility review.
//!
//! Internal implementation details must remain accessible through their
//! owning child modules without automatically becoming part of this
//! facade's stable public surface.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared child module exists exactly once.
//! - Every public re-export refers to an existing public item.
//! - The parent ZUIR module declares `lowering` correctly.
//! - No duplicate types, registries, or verification algorithms are added.
//! - Unsupported conversions cannot be mistaken for successful lowering.
//! - Source-map and diagnostic ownership remain canonical.
//! - Existing callers retain their intended public API.
//! - `cargo fmt`, `cargo check`, relevant tests, and Clippy pass.
//! - No unsafe code or fixed hardware-capacity ceilings are introduced.

// -----------------------------------------------------------------------------
// Canonical child modules
// -----------------------------------------------------------------------------

/// Stable lowering invocation API and backend orchestration.
pub mod api;

/// Adapter from lowering-local records to canonical ZUIR diagnostics.
pub mod diagnostics;

/// Shared lowering state, identity mappings, requirements, and policies.
pub mod context;

/// Legality contracts and analysis for proposed lowering conversions.
pub mod legality;

/// Adapter between lowering provenance and the canonical source map.
pub mod provenance;

// -----------------------------------------------------------------------------
// Stable lowering invocation API
// -----------------------------------------------------------------------------

/// Public lowering entry points, backend contracts, outcomes, and errors.
pub use self::api::{
    lower_with,
    permits_direct_lowering,
    BackendOutput,
    LoweringApiError,
    LoweringApiResult,
    LoweringBackend,
    LoweringOutcome,
};

// -----------------------------------------------------------------------------
// Lowering context API
// -----------------------------------------------------------------------------

/// Lowering-local entity identities, context state, policies, requirements,
/// dispositions, provenance records, and context errors.
pub use self::context::{
    EntityKey,
    EntityKind,
    LoweringContext,
    LoweringContextError,
    LoweringDisposition,
    LoweringNote,
    LoweringPolicy,
    ProvenanceOrigin,
    RequirementRelation,
    ResourceRequirement,
};

// -----------------------------------------------------------------------------
// Legality analysis API
// -----------------------------------------------------------------------------

/// Legality inputs, reports, statuses, semantic contracts, and analysis.
pub use self::legality::{
    analyze_legality,
    DestinationSemantics,
    LegalityCandidate,
    LegalityError,
    LegalityReport,
    LegalityResult,
    LegalityStatus,
    OperationSemantics,
};

// -----------------------------------------------------------------------------
// Diagnostics adapter API
// -----------------------------------------------------------------------------

/// Stable metadata keys and functions for adapting lowering records to
/// canonical ZUIR diagnostics.
pub use self::diagnostics::{
    lowering_context_diagnostics,
    lowering_error_to_diagnostic,
    lowering_note_to_diagnostic,
    lowering_note_to_diagnostic_with_span,
    LoweringDiagnosticError,
    DATA_ENTITY_EXTENSION,
    DATA_ENTITY_IDENTITY,
    DATA_ENTITY_KIND,
    DATA_ENTITY_NAMESPACE,
    DATA_LOWERING_CODE,
    LOWERING_NOTE_DIAGNOSTIC_CODE,
};

// -----------------------------------------------------------------------------
// Provenance adapter API
// -----------------------------------------------------------------------------

/// Explicit identity resolution, provenance attachment, and reporting.
pub use self::provenance::{
    attach_provenance,
    ProvenanceError,
    ProvenanceReport,
    ProvenanceResolver,
    SourceIdentityMap,
};
