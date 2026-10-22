
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Resource Subsystem
//!
//! Path: `src/hybrid/resources/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the public composition root for resource-related contracts
//! consumed by Zamani's hybrid-computing subsystem.
//!
//! It integrates four independently owned components:
//!
//! - [`requirements`]: collection of canonical semantic resource requirements;
//! - [`capabilities`]: scope-specific views of canonical capability evidence;
//! - [`constraints`]: evaluation through the canonical semantic resource
//!   constraint evaluator;
//! - [`feasibility`]: structured assessment of whether supplied evidence
//!   establishes feasibility, infeasibility, or an indeterminate result.
//!
//! This module organizes and selectively re-exports those APIs. It does not
//! implement resource analysis, capability discovery, constraint solving,
//! target selection, physical allocation, or execution.
//!
//! ## Authority and repository integration
//!
//! The implementation must conform to the applicable contracts in:
//!
//! - `grammar/Zamani.g4`;
//! - `grammar/hybrid/hybrid.g4`;
//! - `grammar/hybrid/hybrid-resources.g4`;
//! - `grammar/hybrid/README.md`;
//! - `grammar/spec/hybrid.md`;
//! - `grammar/resources/`;
//! - `src/frontend/ast/node/resources/`;
//! - `src/frontend/semantic/resources/`;
//! - `src/frontend/semantic/capabilities/`.
//!
//! The grammar owns source syntax. The canonical frontend AST owns source
//! structure. The frontend semantic resource model owns resource identities,
//! quantities, units, requirements, and their semantic meaning. The canonical
//! capability subsystem owns capability identities, requirements, and
//! evidence semantics.
//!
//! This module does not create a competing resource language, AST, semantic
//! model, capability ontology, constraint solver, or hardware inventory.
//!
//! ## Ownership boundaries
//!
//! | Component | Authoritative responsibility |
//! |-----------|-----------------------------|
//! | `requirements` | Hybrid-facing collection and completeness of canonical resource requirements |
//! | `capabilities` | Scope-specific integration of canonical capability evidence |
//! | `constraints` | Hybrid-facing entry points to canonical resource constraint evaluation |
//! | `feasibility` | Structured hybrid feasibility assessment from supplied evidence |
//! | Frontend semantic resources | Canonical resource semantics and requirement/constraint evaluation |
//! | Frontend semantic capabilities | Canonical capability semantics and evidence evaluation |
//! | Compiler and planning | Target-specific realization and planning decisions |
//! | Runtime and hardware facilities | Resource discovery, allocation, and execution |
//!
//! No child module may silently take ownership of a concept assigned to
//! another canonical subsystem.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical grammar
//!       |
//!       v
//! Canonical frontend AST
//!       |
//!       v
//! Frontend semantic resources and capabilities
//!       |
//!       v
//! hybrid::resources
//!       |
//!       v
//! Hybrid compiler integration and planning
//!       |
//!       v
//! Backend / runtime / hardware realization
//! ```
//!
//! This module must not depend on physical devices, vendor SDKs, backend
//! implementations, runtime sessions, or hardware discovery.
//!
//! The hybrid compiler pipeline should consume these public contracts rather
//! than implement a competing resource or feasibility subsystem.
//!
//! ## Requirements, capabilities, constraints, and feasibility
//!
//! These concepts are deliberately distinct:
//!
//! - A requirement describes what a program needs.
//! - Capability evidence describes a claim about what a particular scope
//!   supports.
//! - A constraint expresses a condition that must be evaluated according to
//!   its declared semantic intent.
//! - Feasibility assesses what can be established from the supplied
//!   requirements, constraints, evidence, and evaluation results.
//!
//! A preference or hint must not be silently promoted into a mandatory
//! requirement. Missing evidence must not be treated as proof of support or
//! proof of incompatibility. Incomplete analysis must not be reported as
//! successful feasibility.
//!
//! A satisfied resource constraint alone does not prove that an entire
//! computation is feasible.
//!
//! ## POCO-REAF and scalability
//!
//! The subsystem supports an open-ended set of resource kinds, capabilities,
//! computational domains, and resource requirements. Adding a new resource
//! kind or computational domain must not require extending a closed,
//! hardware-specific enumeration in this module.
//!
//! No universal hardware-capacity ceiling is introduced here. In particular,
//! this module defines no fixed maximum number of CPUs, GPUs, FPGAs, QPUs,
//! devices, nodes, qubits, operations, tensor dimensions, memory units, or
//! resource requirements.
//!
//! Actual implementations remain bounded by available memory, address space,
//! processing time, and explicitly configured operational or security
//! budgets. Such exhaustion must be reported explicitly; it must not silently
//! change program semantics or become a language-level hardware restriction.
//!
//! Literal infinity is not a claim about finite physical machines. The
//! architectural requirement is to avoid arbitrary implementation ceilings
//! and preserve valid program intent as available resources and compatible
//! execution environments change.
//!
//! ## Determinism
//!
//! This composition root introduces no global mutable registry, target
//! discovery, random identifiers, timestamps, or target-dependent ordering.
//!
//! Determinism is governed by the documented contracts of the child modules
//! and the canonical semantic implementations they call. Re-exporting an API
//! does not itself guarantee deterministic behavior if a caller supplies
//! nondeterministic inputs.
//!
//! ## Error handling
//!
//! Errors from child modules retain their original types. Callers can
//! distinguish incomplete requirement collection, invalid capability
//! evidence, invalid constraint assessments, and feasibility-evaluation
//! failures without parsing display strings.
//!
//! An indeterminate feasibility result is an assessment outcome, not
//! necessarily a malformed-input error. It must not be converted into
//! success merely because no explicit contradiction was found.
//!
//! ## Public API policy
//!
//! The child modules remain available for callers that need their detailed
//! APIs. The root re-exports commonly shared types to provide a convenient
//! integration boundary without duplicating their definitions.
//!
//! This file contains no business logic. Changes to a child module's internal
//! implementation should not require changes here unless its supported
//! public contract changes.
//!
//! ## Completion criteria
//!
//! This module is complete when:
//!
//! 1. Every declared child module exists.
//! 2. Every re-export resolves to a public item in its authoritative module.
//! 3. No resource, capability, or feasibility type is reimplemented here.
//! 4. The parent `src/hybrid/mod.rs` declares `pub mod resources;`.
//! 5. The crate exposes the hybrid subsystem through one canonical module path.
//! 6. Formatting, compilation, tests, and Clippy pass for the integrated
//!    repository on the declared Rust baseline.

/// Canonical resource requirement collection for hybrid computations.
///
/// This module does not define resource kinds, quantities, units, or
/// requirement-extraction semantics.
pub mod requirements;

/// Hybrid-facing integration of canonical capability requirements and
/// scope-specific capability evidence.
///
/// This module does not discover hardware or allocate physical resources.
pub mod capabilities;

/// Hybrid-facing entry points to canonical resource constraint evaluation.
///
/// This module does not implement a competing constraint solver.
pub mod constraints;

/// Structured assessment of hybrid resource and capability feasibility.
///
/// This module consumes explicit evidence and assessment inputs; it does not
/// discover targets or guarantee that an uninspected target is feasible.
pub mod feasibility;

// ---------------------------------------------------------------------------
// Resource requirements
// ---------------------------------------------------------------------------

/// Canonical semantic resource requirement type.
///
/// Re-exported from its semantic owner; not redefined by the hybrid subsystem.
pub use crate::frontend::semantic::resources::model::ResourceRequirement;

/// Hybrid-facing requirement collection and completeness state.
pub use requirements::{
    HybridRequirementsError,
    HybridResourceRequirements,
    RequirementCollectionState,
};

// ---------------------------------------------------------------------------
// Capability evidence
// ---------------------------------------------------------------------------

/// Canonical capability identity.
pub use capabilities::CapabilityId;

/// Canonical capability requirement under a hybrid-facing alias.
pub use capabilities::SemanticCapabilityRequirement;

/// Canonical capability evidence status.
pub use capabilities::CapabilityEvidenceStatus;

/// Error raised while integrating hybrid capability evidence.
pub use capabilities::HybridCapabilityError;

/// Scope-specific view over canonical capability evidence.
pub use capabilities::HybridCapabilityView;

/// Stable identity of the capability integration contract.
pub use capabilities::HYBRID_CAPABILITIES_MODULE_NAME;

/// Version of the in-memory capability integration contract.
pub use capabilities::HYBRID_CAPABILITIES_SCHEMA_VERSION;

// ---------------------------------------------------------------------------
// Resource constraints
// ---------------------------------------------------------------------------

/// Canonical resource-constraint assessment kind.
pub use constraints::AssessmentKind;

/// Canonical constraint enforcement classification.
pub use constraints::ConstraintEnforcement;

/// Canonical resource constraint evaluation error.
pub use constraints::ConstraintEvaluationError;

/// Canonical constraint outcome under the hybrid-facing alias.
pub use constraints::HybridConstraintOutcome;

/// Canonical resource constraint assessment report under a hybrid-facing
/// alias.
pub use constraints::HybridConstraintAssessmentReport;

/// Canonical constraint operand.
pub use constraints::ConstraintOperand;

/// Canonical constraint operator.
pub use constraints::ConstraintOperator;

/// Exact resource quantity representation owned by semantic resources.
pub use constraints::ExactQuantity;

/// Canonical resource intent classification.
pub use constraints::ResourceIntent;

/// Canonical resource quantity representation.
pub use constraints::ResourceQuantity;

/// Canonical resource unit representation.
pub use constraints::ResourceUnit;

/// Hybrid-facing constraint evaluator.
pub use constraints::HybridResourceConstraintEvaluator;

/// Combined hybrid resource constraint and requirement report.
pub use constraints::HybridResourceConstraintReport;

/// Stable identity of the resource-constraint integration contract.
pub use constraints::HYBRID_RESOURCE_CONSTRAINTS_MODULE_NAME;

/// Version of the in-memory resource-constraint integration contract.
pub use constraints::HYBRID_RESOURCE_CONSTRAINTS_SCHEMA_VERSION;

// ---------------------------------------------------------------------------
// Feasibility
// ---------------------------------------------------------------------------

/// Result classification for hybrid feasibility assessment.
pub use feasibility::FeasibilityStatus;

/// Category of a finding in a hybrid feasibility report.
pub use feasibility::FeasibilityFindingKind;

/// Structured finding produced during feasibility assessment.
pub use feasibility::FeasibilityFinding;

/// Input contract for a hybrid feasibility assessment.
pub use feasibility::HybridFeasibilityInput;

/// Structured hybrid feasibility report.
pub use feasibility::HybridFeasibilityReport;

/// Capability-specific feasibility assessment.
pub use feasibility::CapabilityFeasibilityAssessment;

/// Hybrid feasibility evaluator.
pub use feasibility::HybridFeasibilityEvaluator;

/// Error returned when hybrid feasibility evaluation cannot be completed.
pub use feasibility::HybridFeasibilityError;

/// Stable identity of the hybrid feasibility contract.
pub use feasibility::HYBRID_FEASIBILITY_MODULE_NAME;

/// Version of the in-memory hybrid feasibility contract.
pub use feasibility::HYBRID_FEASIBILITY_SCHEMA_VERSION;
