
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Computing — Normative Invariants
//!
//! Path: `src/hybrid/spec/invariants.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the machine-readable architectural invariants governing Zamani's
//! hybrid-computation subsystem.
//!
//! This module is the canonical catalog of hybrid architectural invariants.
//! It defines what the hybrid implementation must preserve and identifies
//! the subsystem responsible for enforcing each invariant.
//!
//! It does not implement a second semantic analyzer, AST, semantic model,
//! classical IR, quantum IR, universal IR, graph representation, scheduler,
//! resource allocator, runtime, or target-specific compiler.
//!
//! ## Repository authority
//!
//! The applicable authority chain includes:
//!
//! - `grammar/specification/`
//! - `grammar/spec/hybrid.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hybrid/`
//! - `src/frontend/ast/`
//! - `src/frontend/semantic/`
//! - `src/quantum/ir/`
//! - `src/compiler/hybrid_pipeline.rs`
//! - `src/runtime/hybrid_kernel.rs`
//!
//! The language specifications govern source meaning. The canonical AST
//! governs source structure and node identity. Shared semantic services
//! govern types, effects, ownership, capabilities, resources, contracts,
//! policies, and provenance.
//!
//! Hybrid composition preserves cross-domain relationships. It does not
//! replace the authority of those systems.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Stable invariant identifiers.
//! - Normative invariant descriptions.
//! - The enforcement-stage classification of each invariant.
//! - The canonical, deterministic invariant catalog.
//! - Structural validation of the catalog itself.
//!
//! This module does not own:
//!
//! - Program syntax or AST construction.
//! - General semantic analysis or type checking.
//! - Hybrid graph construction or graph validation algorithms.
//! - Quantum operation semantics or quantum IR.
//! - Classical operation semantics or classical IR.
//! - Hardware discovery, allocation, placement, routing, scheduling, QEC,
//!   ZQN, HAL, backend instruction selection, or runtime execution.
//! - Serialization implementation or schema migration.
//!
//! ## Enforcement contract
//!
//! A catalog entry is a normative obligation, not proof that the obligation
//! has already been enforced.
//!
//! The designated enforcement subsystem must implement the invariant and
//! supply tests or other verifiable evidence that it holds.
//!
//! An invariant that has not been enforced, cannot be evaluated, or lacks
//! sufficient evidence must never be reported as satisfied merely because
//! it appears in this catalog.
//!
//! This file intentionally avoids referencing not-yet-frozen graph, domain,
//! resource, or diagnostic APIs. That lets the catalog be completed before
//! those downstream APIs are implemented.
//!
//! ## Scalability contract
//!
//! No invariant establishes a universal maximum for domains, graph nodes,
//! edges, values, dependencies, resources, tensor dimensions, register
//! widths, devices, qubits, processors, accelerators, or operations.
//!
//! Operational limits may be imposed by explicit configuration and security
//! policies. Exhausting a configured budget must be reported explicitly.
//! It must never silently change program semantics or be presented as a
//! universal language restriction.
//!
//! "Scalable without an architectural ceiling" does not mean that a finite
//! machine has infinite memory, address space, execution time, or resources.
//!
//! ## Determinism
//!
//! Invariant identifiers, catalog order, metadata, and catalog-validation
//! results are deterministic and independent of hardware, environment,
//! current time, registration order, and randomized hash iteration.
//!
//! ## Integration order
//!
//! 1. `src/hybrid/spec/mod.rs` declares this module.
//! 2. `src/hybrid/mod.rs` exposes the spec module through the agreed public
//!    module boundary.
//! 3. Hybrid graph verification, semantic integration, planning, interchange,
//!    compiler integration, and conformance tests use these invariant IDs.
//! 4. Existing canonical semantic and IR implementations enforce their own
//!    respective responsibilities through explicit adapters.
//!
//! Adding a catalog entry does not automatically implement its enforcement.
//!
//! ## Completion criteria
//!
//! - Every invariant has a stable identifier and machine-readable code.
//! - Every invariant has an explicit owner and enforcement stage.
//! - All required architectural boundaries are represented.
//! - The catalog contains no duplicate identifiers or codes.
//! - No invariant introduces a universal hardware or workload ceiling.
//! - Tests verify catalog completeness, uniqueness, ordering, and stability.
//! - No unsafe Rust or external dependency is required.

use core::fmt;

/// Stable identifier for a normative hybrid architectural invariant.
///
/// These identifiers are suitable for conformance tests, structured
/// diagnostics, implementation tracking, and compatibility reports.
///
/// Once published, an identifier must not be reassigned to a different
/// invariant. A material change in meaning requires an explicit specification
/// and compatibility review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum InvariantId {
    /// Each language and IR concept has one authoritative owner.
    CanonicalOwnership,

    /// A domain identifies computational meaning, not a physical device.
    DomainIdentity,

    /// All graph references resolve to valid entities.
    GraphIntegrity,

    /// Dependencies preserve the declared control and data semantics.
    DependencySemantics,

    /// Valid feedback and iteration are not prohibited by a blanket DAG rule.
    FeedbackAndCycles,

    /// Cross-domain transformations preserve required program meaning.
    SemanticPreservation,

    /// Hybrid composition is open to future computational domains.
    DomainNeutrality,

    /// Requirements, capabilities, constraints, preferences, and hints differ.
    IntentClassification,

    /// Synchronization and completion guarantees are explicit.
    SynchronizationSemantics,

    /// Canonical results are deterministic under equivalent explicit inputs.
    Determinism,

    /// Domain-specific semantics remain with their canonical IR owners.
    NoDuplicateIr,

    /// Portable composition does not encode target-specific realization.
    TargetIndependence,

    /// No universal machine-size or workload ceiling is introduced.
    NoUniversalLimits,

    /// Operational budgets are explicit and exhaustion is observable.
    OperationalBudgets,

    /// Feasibility reflects evidence about requirements and capabilities.
    FeasibilityHonesty,

    /// Source identity and semantic provenance survive integration.
    ProvenancePreservation,

    /// Failures cannot silently become successful validation.
    FailureTransparency,

    /// Cancellation and asynchronous completion respect declared contracts.
    LifecycleSemantics,

    /// Persisted representations obey explicit version and validation rules.
    InterchangeCompatibility,

    /// Only safe Rust is permitted in this subsystem.
    SafeImplementation,
}

impl InvariantId {
    /// Return the stable machine-readable identifier.
    ///
    /// These identifiers are independent of display text and enum debug
    /// formatting. Do not parse human-readable messages to identify an
    /// invariant.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalOwnership => "HYB-INV-001",
            Self::DomainIdentity => "HYB-INV-002",
            Self::GraphIntegrity => "HYB-INV-003",
            Self::DependencySemantics => "HYB-INV-004",
            Self::FeedbackAndCycles => "HYB-INV-005",
            Self::SemanticPreservation => "HYB-INV-006",
            Self::DomainNeutrality => "HYB-INV-007",
            Self::IntentClassification => "HYB-INV-008",
            Self::SynchronizationSemantics => "HYB-INV-009",
            Self::Determinism => "HYB-INV-010",
            Self::NoDuplicateIr => "HYB-INV-011",
            Self::TargetIndependence => "HYB-INV-012",
            Self::NoUniversalLimits => "HYB-INV-013",
            Self::OperationalBudgets => "HYB-INV-014",
            Self::FeasibilityHonesty => "HYB-INV-015",
            Self::ProvenancePreservation => "HYB-INV-016",
            Self::FailureTransparency => "HYB-INV-017",
            Self::LifecycleSemantics => "HYB-INV-018",
            Self::InterchangeCompatibility => "HYB-INV-019",
            Self::SafeImplementation => "HYB-INV-020",
        }
    }
}

impl fmt::Display for InvariantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Architectural stage responsible for enforcing an invariant.
///
/// The stage identifies the authoritative enforcement boundary. It does
/// not prescribe a particular implementation algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum EnforcementStage {
    /// Canonical AST and shared semantic services.
    SemanticAnalysis,

    /// Hybrid domain composition and graph verification.
    HybridComposition,

    /// Domain-specific canonical IR verification.
    DomainIr,

    /// Cross-domain synchronization and lifecycle contracts.
    Coordination,

    /// Explicit policy and operational budget enforcement.
    Policy,

    /// Target capability evaluation and realization feasibility.
    Feasibility,

    /// Canonical encoding, decoding, and schema compatibility.
    Interchange,

    /// Compiler transformations and target-independent planning.
    Compilation,

    /// Runtime execution and lifecycle integration.
    Runtime,

    /// Language-wide and subsystem-wide safety requirements.
    RepositoryConformance,
}

/// Normative definition of one hybrid invariant.
///
/// All text fields are static strings so the canonical catalog does not
/// require allocation to inspect its definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvariantDefinition {
    /// Stable machine-readable invariant identity.
    pub id: InvariantId,

    /// Short human-readable title.
    pub title: &'static str,

    /// Normative statement describing what must hold.
    pub statement: &'static str,

    /// Authoritative stage responsible for enforcement.
    pub enforcement_stage: EnforcementStage,

    /// Canonical repository boundary responsible for enforcement.
    ///
    /// This is documentation and integration metadata, not a dynamically
    /// resolved module path.
    pub owner: &'static str,

    /// Observable behavior required when the invariant cannot be satisfied.
    pub failure_behavior: &'static str,

    /// Minimum evidence required to consider enforcement complete.
    pub verification: &'static str,
}

/// The canonical invariant catalog.
///
/// Catalog order is stable and intentional. Do not derive this order from a
/// hash map, filesystem traversal, runtime registration, or target discovery.
///
/// A new invariant must receive a new stable identifier and an appropriate
/// enforcement owner. Removing or materially redefining a published
/// invariant requires a compatibility review.
pub const HYBRID_INVARIANTS: &[InvariantDefinition] = &[
    InvariantDefinition {
        id: InvariantId::CanonicalOwnership,
        title: "Single canonical ownership",
        statement: "Every source, semantic, domain-IR, and runtime concept has one authoritative owner. Hybrid composition references or adapts canonical concepts instead of defining competing versions.",
        enforcement_stage: EnforcementStage::SemanticAnalysis,
        owner: "src/frontend/semantic/ and the relevant canonical domain modules",
        failure_behavior: "Reject conflicting or ambiguous ownership through a structured diagnostic; do not silently choose one definition.",
        verification: "Ownership map, adapter tests, dependency-boundary tests, and duplicate-definition audits.",
    },
    InvariantDefinition {
        id: InvariantId::DomainIdentity,
        title: "Semantic domain identity",
        statement: "A domain identity denotes a computational domain or registered semantic extension, not a physical device, vendor, execution session, or hardware instance.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "src/hybrid/domain/ and hybrid semantic integration",
        failure_behavior: "Reject malformed or unresolved domain references; do not infer a physical target from a domain name.",
        verification: "Identity validation, open-domain registration tests, and tests distinguishing domain identity from target identity.",
    },
    InvariantDefinition {
        id: InvariantId::GraphIntegrity,
        title: "Reference and graph integrity",
        statement: "Every graph reference, endpoint, value reference, dependency reference, and domain-instance reference must resolve within its declared identity and lifetime scope.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "Hybrid graph verification and src/frontend/semantic/domains/hybrid.rs",
        failure_behavior: "Return an explicit validation failure for dangling, ambiguous, or incompatible references; never certify an incomplete graph as valid.",
        verification: "Positive and negative reference tests, duplicate-identity tests, malformed-input tests, and property-based graph tests.",
    },
    InvariantDefinition {
        id: InvariantId::DependencySemantics,
        title: "Preservation of dependencies",
        statement: "Data dependencies, control dependencies, ordering requirements, completion dependencies, and feedback dependencies remain distinguishable and preserve their declared semantics.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "Hybrid graph verification and canonical control-flow integration",
        failure_behavior: "Reject invalid or lost required dependencies; never silently remove an edge because a target cannot realize it.",
        verification: "Dependency-kind tests, graph transformation tests, and end-to-end data/control dependency tests.",
    },
    InvariantDefinition {
        id: InvariantId::FeedbackAndCycles,
        title: "Semantics-aware feedback and cycles",
        statement: "A dependency cycle is invalid only when the declared construct semantics prohibit it. Valid loops, iterative computation, and classical feedback controlling subsequent quantum operations must remain representable.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "Hybrid graph verification and canonical quantum/classical control-flow IR",
        failure_behavior: "Reject cycles proven invalid by their semantics; preserve legal feedback and report indeterminate analysis when legality cannot be established.",
        verification: "Legal-loop tests, illegal-cycle tests, measurement-to-classical-feedback tests, and tests for iterative and nested compositions.",
    },
    InvariantDefinition {
        id: InvariantId::SemanticPreservation,
        title: "Semantic preservation",
        statement: "Every accepted transformation preserves the program's required observable behavior, including types, effects, ownership, value meaning, numerical guarantees, concurrency semantics, measurement behavior, and required ordering.",
        enforcement_stage: EnforcementStage::Compilation,
        owner: "Canonical semantic model, domain IR verifiers, and compiler transformation validators",
        failure_behavior: "Reject an invalid transformation or report that semantic preservation cannot be proven; never silently weaken a requirement.",
        verification: "Transformation equivalence tests, negative preservation tests, and cross-target conformance tests.",
    },
    InvariantDefinition {
        id: InvariantId::DomainNeutrality,
        title: "Open-world domain composition",
        statement: "The hybrid composition contract permits new computational domains through explicit, versioned interfaces without requiring a closed universal enumeration of all future domains.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "src/hybrid/domain/ and integration adapters",
        failure_behavior: "Report unsupported or unregistered domain contracts explicitly; do not reinterpret unknown domain operations as known operations.",
        verification: "Custom-domain registration tests, interface-version tests, and tests proving that adding a domain does not require changing unrelated domain semantics.",
    },
    InvariantDefinition {
        id: InvariantId::IntentClassification,
        title: "Distinct intent categories",
        statement: "Requirements, capabilities, mandatory constraints, preferences, hints, policies, and feasibility are distinct concepts. Preferences and hints must not be treated as mandatory requirements, and capabilities must not be mistaken for program requirements.",
        enforcement_stage: EnforcementStage::SemanticAnalysis,
        owner: "Canonical semantic capability, resource, constraint, and policy models",
        failure_behavior: "Report malformed or contradictory intent explicitly; do not silently change the category or force an interpretation that changes program meaning.",
        verification: "Semantic classification tests, requirement/capability mismatch tests, and tests for mandatory versus advisory intent.",
    },
    InvariantDefinition {
        id: InvariantId::SynchronizationSemantics,
        title: "Explicit synchronization semantics",
        statement: "Ordering, barriers, asynchronous dependencies, result availability, completion, and synchronization guarantees must be represented explicitly whenever they affect observable behavior.",
        enforcement_stage: EnforcementStage::Coordination,
        owner: "Hybrid coordination contracts and canonical quantum/classical control-flow facilities",
        failure_behavior: "Reject a plan that violates a required ordering or synchronization guarantee; do not assume submission implies completion.",
        verification: "Ordering tests, asynchronous completion tests, measurement feedback tests, and synchronization regression tests.",
    },
    InvariantDefinition {
        id: InvariantId::Determinism,
        title: "Deterministic canonical results",
        statement: "Where determinism is promised, equivalent canonical inputs, schema versions, and explicit policies produce equivalent canonical representations and stable diagnostic ordering independent of incidental machine or collection iteration order.",
        enforcement_stage: EnforcementStage::Interchange,
        owner: "Canonical hybrid representation, compiler pipeline, and interchange implementation",
        failure_behavior: "Report a determinism violation or fail the operation; never conceal nondeterminism by silently altering semantics.",
        verification: "Repeated-run tests, canonical encoding comparisons, ordering tests, and reproducibility tests under varied registration and insertion orders.",
    },
    InvariantDefinition {
        id: InvariantId::NoDuplicateIr,
        title: "No competing intermediate representation",
        statement: "The hybrid subsystem describes cross-domain composition and dependencies. It must not redefine canonical quantum, classical, HDL, tensor, or universal IR operations and values.",
        enforcement_stage: EnforcementStage::DomainIr,
        owner: "src/quantum/ir/, canonical classical facilities, and their domain adapters",
        failure_behavior: "Reject integration designs that establish conflicting semantic authority; reference canonical domain representations through explicit adapters.",
        verification: "Dependency audits, API ownership tests, and integration tests against canonical IR identifiers and values.",
    },
    InvariantDefinition {
        id: InvariantId::TargetIndependence,
        title: "Target-independent composition",
        statement: "The portable hybrid representation must not require physical device selection, vendor handles, physical qubit placement, hardware routing, calibration, or target-specific scheduling to express program meaning.",
        enforcement_stage: EnforcementStage::Compilation,
        owner: "Hybrid composition core and downstream planning interfaces",
        failure_behavior: "Keep target-specific assumptions in explicit downstream plans; report missing realization information rather than embedding guessed physical choices in the portable representation.",
        verification: "Target-independent construction tests, dependency-boundary tests, and cross-target planning tests.",
    },
    InvariantDefinition {
        id: InvariantId::NoUniversalLimits,
        title: "No universal machine-size ceilings",
        statement: "The hybrid architecture must not impose arbitrary universal ceilings on domains, operations, nodes, edges, qubits, devices, processors, memory, tensor rank, register width, graph size, nesting, or other program-defined quantities.",
        enforcement_stage: EnforcementStage::RepositoryConformance,
        owner: "Hybrid subsystem and repository-wide architecture conformance",
        failure_behavior: "Remove the unjustified universal ceiling or reclassify it as an explicit operational policy with documented scope and observable exhaustion behavior.",
        verification: "Source audits, configurable stress tests, boundary tests, and checks for fixed-size collections or hidden numeric ceilings.",
    },
    InvariantDefinition {
        id: InvariantId::OperationalBudgets,
        title: "Explicit operational budgets",
        statement: "A finite operational budget is permitted only as an explicit, documented policy. Budget exhaustion must be observable and must not alter the semantics of a valid program.",
        enforcement_stage: EnforcementStage::Policy,
        owner: "src/hybrid/config.rs and the operation consuming each budget",
        failure_behavior: "Return a structured budget-exhaustion result or an explicitly incomplete result; do not report partial analysis as complete or valid.",
        verification: "Default and override tests, exact-boundary tests, budget-exhaustion tests, and tests proving that absent budgets do not create hidden language ceilings.",
    },
    InvariantDefinition {
        id: InvariantId::FeasibilityHonesty,
        title: "Evidence-based feasibility",
        statement: "Feasibility decisions must use declared requirements, constraints, available capability evidence, and applicable policies. Insufficient evidence must remain distinguishable from success and proven infeasibility.",
        enforcement_stage: EnforcementStage::Feasibility,
        owner: "Resource/capability analysis and downstream compiler planning",
        failure_behavior: "Return feasible, infeasible, or indeterminate outcomes as appropriate; never invent target capabilities or claim universal executability.",
        verification: "Missing-capability tests, indeterminate-evidence tests, explicit infeasibility tests, and capability-satisfaction tests.",
    },
    InvariantDefinition {
        id: InvariantId::ProvenancePreservation,
        title: "Source and semantic provenance",
        statement: "Where available, source-node identity, source spans, semantic provenance, and transformation history remain traceable through hybrid validation and integration.",
        enforcement_stage: EnforcementStage::SemanticAnalysis,
        owner: "Canonical AST, semantic diagnostics, and hybrid integration adapters",
        failure_behavior: "Report unavailable or lost required provenance explicitly; do not fabricate source locations or claim a lossless mapping when one is unavailable.",
        verification: "AST-to-semantic mapping tests, diagnostic-location tests, and transformation provenance tests.",
    },
    InvariantDefinition {
        id: InvariantId::FailureTransparency,
        title: "Transparent validation and execution failures",
        statement: "Invalid input, incomplete analysis, unsupported semantics, exhausted budgets, and execution failures must remain distinguishable. A failure must not be converted into successful validation by a fallback that changes required meaning.",
        enforcement_stage: EnforcementStage::HybridComposition,
        owner: "Hybrid errors, diagnostics, validators, compiler adapters, and runtime adapters",
        failure_behavior: "Preserve the original failure category and context through adapters; return explicit errors or incomplete outcomes.",
        verification: "Error propagation tests, malformed-input tests, incomplete-analysis tests, and compiler/runtime integration regression tests.",
    },
    InvariantDefinition {
        id: InvariantId::LifecycleSemantics,
        title: "Explicit asynchronous lifecycle",
        statement: "Submission, running, completion, failure, and cancellation are distinct lifecycle states where applicable. Cancellation and resource release must respect the execution contract and cannot imply that already completed external work has been undone.",
        enforcement_stage: EnforcementStage::Runtime,
        owner: "src/runtime/hybrid_kernel.rs and runtime integration adapters",
        failure_behavior: "Expose lifecycle failures and best-effort cancellation explicitly; preserve known completion and partial-result information.",
        verification: "Lifecycle state tests, cancellation races, asynchronous completion tests, and resource-release tests.",
    },
    InvariantDefinition {
        id: InvariantId::InterchangeCompatibility,
        title: "Versioned interchange",
        statement: "Persisted hybrid representations must declare or otherwise unambiguously identify their schema version, validate references and contracts, and apply explicit compatibility rules.",
        enforcement_stage: EnforcementStage::Interchange,
        owner: "Hybrid interchange schema and canonical serialization infrastructure",
        failure_behavior: "Reject unsupported or malformed representations, or apply an explicitly supported migration; never silently reinterpret unknown semantics.",
        verification: "Round-trip tests, unknown-version tests, malformed-input tests, and compatibility/migration tests.",
    },
    InvariantDefinition {
        id: InvariantId::SafeImplementation,
        title: "Safe Rust implementation",
        statement: "The hybrid subsystem must not require or introduce unsafe Rust. Its public and internal APIs must uphold their documented safety contracts using safe Rust.",
        enforcement_stage: EnforcementStage::RepositoryConformance,
        owner: "src/hybrid/ and repository CI",
        failure_behavior: "Reject changes that introduce unsafe code or depend on an undocumented unsafe implementation requirement.",
        verification: "Crate/module unsafe-code lints, code review, and repository CI checks.",
    },
];

/// Return the canonical invariant catalog in its stable declared order.
///
/// The returned slice is immutable and allocation-free to access.
#[must_use]
pub const fn all_invariants() -> &'static [InvariantDefinition] {
    HYBRID_INVARIANTS
}

/// Find an invariant by its stable machine-readable identifier.
///
/// This lookup is independent of catalog ordering and performs no allocation.
#[must_use]
pub fn invariant_by_code(code: &str) -> Option<&'static InvariantDefinition> {
    HYBRID_INVARIANTS
        .iter()
        .find(|definition| definition.id.as_str() == code)
}

/// Return whether the canonical catalog contains the supplied identifier.
///
/// This helper is intended for integration adapters and conformance tests.
#[must_use]
pub fn contains_invariant(id: InvariantId) -> bool {
    HYBRID_INVARIANTS
        .iter()
        .any(|definition| definition.id == id)
}

/// A structural error in the invariant catalog.
///
/// This error concerns the catalog definition itself. It is not a diagnostic
/// about a user's hybrid program and must not be used as a substitute for
/// graph or semantic validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogError {
    /// Two entries use the same stable invariant identifier.
    DuplicateIdentifier {
        /// Identifier that occurs more than once.
        id: InvariantId,
    },

    /// Two entries use the same machine-readable code.
    DuplicateCode {
        /// Conflicting stable machine-readable code.
        code: &'static str,
    },

    /// An entry contains a required empty metadata field.
    EmptyMetadata {
        /// Identifier of the entry with missing metadata.
        id: InvariantId,

        /// Name of the missing metadata field.
        field: &'static str,
    },
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentifier { id } => {
                write!(formatter, "duplicate hybrid invariant identifier: {id}")
            }
            Self::DuplicateCode { code } => {
                write!(formatter, "duplicate hybrid invariant code: {code}")
            }
            Self::EmptyMetadata { id, field } => {
                write!(formatter, "hybrid invariant {id} has empty {field}")
            }
        }
    }
}

impl std::error::Error for CatalogError {}

/// Validate the catalog's structural integrity.
///
/// This checks metadata and uniqueness only. It does not claim that the
/// semantic, graph, compiler, interchange, or runtime invariants are
/// enforced by their respective implementation modules.
///
/// The result is deterministic because the catalog has a fixed declared
/// order and validation returns the first error in that order.
pub fn validate_catalog() -> Result<(), CatalogError> {
    for (index, definition) in HYBRID_INVARIANTS.iter().enumerate() {
        if definition.title.trim().is_empty() {
            return Err(CatalogError::EmptyMetadata {
                id: definition.id,
                field: "title",
            });
        }

        if definition.statement.trim().is_empty() {
            return Err(CatalogError::EmptyMetadata {
                id: definition.id,
                field: "statement",
            });
        }

        if definition.owner.trim().is_empty() {
            return Err(CatalogError::EmptyMetadata {
                id: definition.id,
                field: "owner",
            });
        }

        if definition.failure_behavior.trim().is_empty() {
            return Err(CatalogError::EmptyMetadata {
                id: definition.id,
                field: "failure_behavior",
            });
        }

        if definition.verification.trim().is_empty() {
            return Err(CatalogError::EmptyMetadata {
                id: definition.id,
                field: "verification",
            });
        }

        for earlier in &HYBRID_INVARIANTS[..index] {
            if earlier.id == definition.id {
                return Err(CatalogError::DuplicateIdentifier {
                    id: definition.id,
                });
            }

            if earlier.id.as_str() == definition.id.as_str() {
                return Err(CatalogError::DuplicateCode {
                    code: definition.id.as_str(),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_structurally_valid() {
        assert_eq!(validate_catalog(), Ok(()));
    }

    #[test]
    fn every_invariant_has_a_stable_nonempty_code() {
        for definition in all_invariants() {
            assert!(!definition.id.as_str().is_empty());
            assert!(definition.id.as_str().starts_with("HYB-INV-"));
        }
    }

    #[test]
    fn invariant_codes_are_unique() {
        for (index, definition) in all_invariants().iter().enumerate() {
            assert!(
                !all_invariants()[..index]
                    .iter()
                    .any(|earlier| earlier.id.as_str() == definition.id.as_str()),
                "duplicate invariant code: {}",
                definition.id
            );
        }
    }

    #[test]
    fn invariant_identifiers_are_unique() {
        for (index, definition) in all_invariants().iter().enumerate() {
            assert!(
                !all_invariants()[..index]
                    .iter()
                    .any(|earlier| earlier.id == definition.id),
                "duplicate invariant identifier: {}",
                definition.id
            );
        }
    }

    #[test]
    fn all_required_architectural_boundaries_are_present() {
        let required = [
            InvariantId::CanonicalOwnership,
            InvariantId::DomainIdentity,
            InvariantId::GraphIntegrity,
            InvariantId::DependencySemantics,
            InvariantId::FeedbackAndCycles,
            InvariantId::SemanticPreservation,
            InvariantId::DomainNeutrality,
            InvariantId::IntentClassification,
            InvariantId::SynchronizationSemantics,
            InvariantId::Determinism,
            InvariantId::NoDuplicateIr,
            InvariantId::TargetIndependence,
            InvariantId::NoUniversalLimits,
            InvariantId::OperationalBudgets,
            InvariantId::FeasibilityHonesty,
            InvariantId::ProvenancePreservation,
            InvariantId::FailureTransparency,
            InvariantId::LifecycleSemantics,
            InvariantId::InterchangeCompatibility,
            InvariantId::SafeImplementation,
        ];

        for id in required {
            assert!(
                contains_invariant(id),
                "required invariant missing: {id}"
            );
        }
    }

    #[test]
    fn code_lookup_returns_the_matching_definition() {
        for definition in all_invariants() {
            let found = invariant_by_code(definition.id.as_str())
                .expect("every catalog entry must be discoverable by code");

            assert_eq!(found.id, definition.id);
        }
    }

    #[test]
    fn unknown_codes_do_not_match_an_invariant() {
        assert!(invariant_by_code("HYB-INV-UNKNOWN").is_none());
        assert!(invariant_by_code("").is_none());
    }

    #[test]
    fn catalog_order_is_stable() {
        let first = all_invariants()
            .first()
            .expect("the invariant catalog must not be empty");

        let last = all_invariants()
            .last()
            .expect("the invariant catalog must not be empty");

        assert_eq!(first.id, InvariantId::CanonicalOwnership);
        assert_eq!(last.id, InvariantId::SafeImplementation);
    }

    #[test]
    fn every_invariant_has_an_enforcement_owner_and_verification_contract() {
        for definition in all_invariants() {
            assert!(!definition.owner.trim().is_empty());
            assert!(!definition.statement.trim().is_empty());
            assert!(!definition.failure_behavior.trim().is_empty());
            assert!(!definition.verification.trim().is_empty());
        }
    }

    #[test]
    fn identifiers_have_stable_display_formatting() {
        assert_eq!(
            InvariantId::CanonicalOwnership.to_string(),
            "HYB-INV-001"
        );
        assert_eq!(
            InvariantId::NoUniversalLimits.to_string(),
            "HYB-INV-013"
        );
        assert_eq!(
            InvariantId::SafeImplementation.to_string(),
            "HYB-INV-020"
        );
    }

    #[test]
    fn catalog_is_independent_of_runtime_target_selection() {
        // Catalog definitions are static and require no hardware or runtime
        // discovery. This test protects that dependency-free contract.
        assert!(!all_invariants().is_empty());
        assert_eq!(validate_catalog(), Ok(()));
    }
}
