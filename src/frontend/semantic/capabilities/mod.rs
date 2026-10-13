//! # Zamani Frontend — Semantic Capabilities
//!
//! Path: `src/frontend/semantic/capabilities/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97+
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the public integration boundary for target-independent capability
//! semantics in the Zamani frontend.
//!
//! This module composes four independently owned components:
//!
//! - `model`: canonical semantic capability identities, versions, properties,
//!   and requirements.
//! - `requirements`: conversion of AST capability constructs into semantic
//!   requirements while preserving source provenance.
//! - `evidence`: scoped claims about capability support and their evaluation.
//! - `compatibility`: deterministic reports evaluating requirements against
//!   evidence for an explicitly selected scope.
//!
//! This module is an aggregation boundary. It must not implement a second
//! capability model or duplicate the logic owned by its child modules.
//!
//! ## AUTHORITY
//!
//! Syntax and source structure are governed by the canonical grammar and AST.
//! Semantic meaning is governed by the applicable normative specifications,
//! including:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/spec/quantum.md`
//! - `grammar/spec/hdl.md`
//!
//! The scalability specification is subject to its stated conformance status.
//! An implementation must not silently promote a proposed specification to
//! normative authority.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - declaration and organization of the capability submodules;
//! - the supported public re-exports of their APIs;
//! - documentation of the cross-module integration contract.
//!
//! It does not own:
//!
//! - source syntax, AST definitions, parsing, or name resolution;
//! - capability-definition registration or extension discovery;
//! - authentication, trust verification, or authorization policy;
//! - resource arithmetic or resource allocation;
//! - hardware discovery, target selection, placement, routing, or scheduling;
//! - classical IR, `quantum::ir`, HDL IR, or runtime execution.
//!
//! ## PUBLIC DATA FLOW
//!
//! ```text
//! Canonical frontend AST
//!         |
//!         v
//! requirements
//!         |
//!         v
//! model::CapabilityRequirement
//!         |
//!         +---------------------------+
//!         |                           |
//!         v                           v
//! semantic facts                evidence
//!                                     |
//!                                     v
//!                              compatibility
//!                                     |
//!                                     v
//!                         scoped compatibility report
//!                                     |
//!                                     v
//!                      downstream target-feasibility work
//! ```
//!
//! This diagram describes ownership and data flow. It does not imply that
//! target discovery or feasibility is implemented by this module.
//!
//! ## SEMANTIC INVARIANTS
//!
//! 1. A capability requirement expresses program intent; it does not prove
//!    that a capability exists or is available on a target.
//! 2. Capability evidence is a scoped claim. Recording evidence does not
//!    authenticate the claim or establish that it is trustworthy.
//! 3. Missing evidence is not proof of incompatibility.
//! 4. Contradictory evidence must remain observable.
//! 5. Unknown or incomplete evidence must not be converted into a successful
//!    compatibility result.
//! 6. Compatibility reports must preserve the originating AST node and span.
//! 7. Capability identity is open-world. New domains, namespaces, and
//!    capability names must not require editing a closed Rust enumeration.
//! 8. Capability support and resource sufficiency are distinct questions.
//! 9. A source requirement must not cause physical resources to be allocated.
//! 10. Capability semantics must not introduce a competing quantum IR.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! The capability subsystem must support programs intended for tiny,
//! large, heterogeneous, distributed, classical, quantum, HDL, and future
//! computational environments without imposing arbitrary language-wide
//! hardware-capacity ceilings.
//!
//! No universal maximum capability count, namespace count, device count,
//! or resource capacity is defined here.
//!
//! Actual Rust collections, address spaces, memory, processing time, and
//! serialized representations are finite. The subsystem therefore makes no
//! literal-infinity guarantee. It must preserve open-ended semantic intent
//! within available implementation resources.
//!
//! Operational limits, if needed, belong to explicit compiler or session
//! configuration. Exhausting an operational budget must not silently turn
//! incomplete analysis into a successful semantic or compatibility result.
//!
//! ## DETERMINISM
//!
//! Determinism is provided by the child modules' documented ordering
//! contracts. This aggregation module introduces no global mutable registry,
//! random identity, timestamps, or target-dependent ordering.
//!
//! Deterministic re-exports do not by themselves guarantee deterministic
//! analysis. Consumers must also preserve the child modules' validation and
//! ordering requirements.
//!
//! ## ERROR HANDLING
//!
//! Child-module errors remain distinct so callers can determine whether a
//! failure arose from:
//!
//! - invalid semantic capability data;
//! - invalid AST-to-semantic conversion;
//! - invalid evidence;
//! - or compatibility-report preparation.
//!
//! A compatibility status such as `Incompatible`, `NoEvidence`,
//! `Indeterminate`, or `ConflictingEvidence` is a report outcome, not
//! interchangeable with a malformed-input error.
//!
//! ## INTEGRATION CONTRACT
//!
//! Inputs:
//! - canonical semantic capability values;
//! - validated source-level AST capability constructs;
//! - explicitly scoped evidence.
//!
//! Outputs:
//! - canonical capability requirements;
//! - deterministic evidence collections;
//! - evidence-based compatibility reports with source provenance.
//!
//! Consumers:
//! - semantic facts and program finalization;
//! - domain validators;
//! - compiler diagnostics and reporting;
//! - downstream feasibility and realization services.
//!
//! The semantic model does not depend on a concrete target or on any IR.
//! Target-specific consumers must use explicit adapters at their own
//! architectural boundary.
//!
//! ## FREEZE CRITERIA
//!
//! This module is complete when:
//!
//! - all four declared child modules exist and compile;
//! - every re-export below resolves to an actual public item;
//! - the AST capability path used by `requirements.rs` is exported correctly;
//! - public API tests verify the intended import paths;
//! - missing and conflicting evidence remain distinguishable;
//! - source provenance survives requirement collection and evaluation;
//! - determinism and invalid-input tests pass;
//! - the supported workspace builds and tests pass on Rust 1.97+;
//! - no unsafe code is introduced.
//!
//! This module is an integration boundary, not proof that the complete
//! semantic subsystem or compiler is production-ready.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical child modules
// =============================================================================

/// Canonical, target-independent semantic capability types.
///
/// Owns capability identity, version constraints, property values, and
/// semantic requirements. Does not own source syntax or target evidence.
pub mod model;

/// Conversion of AST capability constructs into semantic requirements.
///
/// Preserves source node identity and span. Does not establish target support.
pub mod requirements;

/// Scoped evidence describing claims about capability support.
///
/// Records and evaluates evidence without claiming to authenticate it.
pub mod evidence;

/// Deterministic evaluation of semantic requirements against scoped evidence.
///
/// Does not allocate resources, select targets, or perform hardware discovery.
pub mod compatibility;

// =============================================================================
// Canonical semantic model API
// =============================================================================

pub use model::{
    CapabilityComponent,
    CapabilityId,
    CapabilityModelError,
    CapabilityPropertyValue,
    CapabilityRequirement,
    CapabilityVersion,
    CapabilityVersionConstraint,
};

// =============================================================================
// AST-to-semantic requirement API
// =============================================================================

pub use requirements::{
    CapabilityRequirementError,
    CapabilityRequirementRecord,
    CapabilityRequirements,
    CAPABILITY_REQUIREMENTS_MODULE_NAME,
    CAPABILITY_REQUIREMENTS_SCHEMA_VERSION,
};

// =============================================================================
// Scoped evidence API
// =============================================================================

pub use evidence::{
    CapabilityEvidence,
    CapabilityEvidenceError,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    CapabilityEvidenceStatus,
    EvidenceEvaluation,
    EvidenceOrigin,
    EvidenceSubject,
    CAPABILITY_EVIDENCE_SCHEMA_VERSION,
};

// =============================================================================
// Compatibility evaluation API
// =============================================================================

pub use compatibility::{
    evaluate_compatibility,
    CapabilityCompatibilityError,
    CapabilityCompatibilityRecord,
    CapabilityCompatibilityReport,
    CapabilityCompatibilityStatus,
    CapabilityCompatibilitySummary,
    CAPABILITY_COMPATIBILITY_MODULE_NAME,
    CAPABILITY_COMPATIBILITY_SCHEMA_VERSION,
};