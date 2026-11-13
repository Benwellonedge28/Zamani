
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Computational Domains
//!
//! Path: `src/zur/domains/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! This module is the public composition root for ZUR's computational-domain
//! provider integration.
//!
//! It connects the independently owned domain subsystems:
//!
//! - Domain descriptors.
//! - Domain-provider contracts.
//! - Provider registration and lookup.
//! - Provider discovery.
//! - Domain and provider validation.
//! - Domain/provider compatibility.
//! - Hybrid composition integration.
//!
//! This module provides stable module paths and selective public re-exports.
//! It does not implement or duplicate the functionality owned by its child
//! modules.
//!
//! # Architectural ownership
//!
//! The child modules retain exclusive ownership of their respective APIs.
//!
//! | Module | Authoritative responsibility |
//! |--------|------------------------------|
//! | `descriptor` | Runtime-facing domain descriptors and external contract references |
//! | `provider` | Domain-provider integration and structural provider validation |
//! | `registry` | Explicit provider registration, indexing and lookup |
//! | `discovery` | Provider discovery sources, queries and reports |
//! | `validation` | Domain/provider validation and capability-evidence resolution |
//! | `compatibility` | Provider-interface and contract-version compatibility |
//! | `composition` | ZUR-to-Hybrid adaptation and explicit provider composition |
//!
//! # Canonical authorities
//!
//! The following repository components remain authoritative:
//!
//! - `frontend::ast::node::domains::domain::Domain`:
//!   canonical source-level domain identity.
//! - `stdlib::provider`:
//!   provider identity and common provider lifecycle.
//! - `stdlib::capability`:
//!   capability identity, semantics and status.
//! - `zur::contracts::provider`:
//!   execution-provider interfaces, contracts and operations.
//! - `zur::version`:
//!   service and contract versions.
//! - `zur::error`:
//!   operational error representation.
//! - `hybrid::domain::composition`:
//!   canonical hybrid domain composition and instances.
//! - `hybrid::graph`:
//!   hybrid graph and dependency semantics.
//!
//! This module must not establish competing definitions of these concepts.
//!
//! # Responsibility boundaries
//!
//! The domain integration subsystem does not own:
//!
//! - Source-language grammar or parsing.
//! - AST construction or semantic analysis.
//! - Domain-specific computation semantics.
//! - Requirement definitions.
//! - Capability definitions.
//! - Preference, constraint or hint semantics.
//! - Capability-evidence authentication.
//! - Physical resource discovery.
//! - Resource allocation or reservation.
//! - Feasibility analysis.
//! - Hardware selection.
//! - Provider authentication or authorization.
//! - Backend registration or code generation.
//! - Compiler planning or optimization.
//! - Classical, quantum, HDL, AI or tensor IR.
//! - ZUIR.
//! - Hybrid graph construction or graph planning.
//! - Distributed execution planning.
//! - Scheduling, routing or placement.
//! - Physical device interaction.
//! - Runtime execution.
//!
//! These responsibilities remain with their existing repository owners.
//!
//! # Architectural invariants
//!
//! The following distinctions are mandatory throughout this subsystem:
//!
//! - Requirement != Capability.
//! - Capability != Preference.
//! - Preference != Constraint.
//! - Constraint != Hint.
//! - Hint != Feasibility.
//! - Capability != CapabilityEvidence.
//! - CapabilityEvidence != CapabilityAvailability.
//! - Feasibility != ResourceReservation.
//! - ResourceReservation != Execution.
//! - Provider registration != Provider initialization.
//! - Provider initialization != Provider availability.
//! - Provider availability != Capability availability.
//! - Execution submission != Successful completion.
//!
//! A successful registry lookup establishes registration only.
//!
//! A successful discovery operation establishes a discovered candidate only.
//!
//! A successful structural validation establishes structural validity only.
//!
//! A successful compatibility check establishes contract compatibility only.
//!
//! A successful hybrid composition resolution establishes structural provider
//! coverage only.
//!
//! None of these operations independently proves that physical resources are
//! available, reserved, or capable of completing a particular computation.
//!
//! # Open-world domain model
//!
//! Computational domains use the canonical frontend `Domain` identity.
//!
//! This module deliberately introduces no closed computational-domain enum.
//!
//! Domains may include existing and future computational technologies,
//! including classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! accelerator, photonic and other independently defined domains.
//!
//! These examples are descriptive, not an exhaustive inventory.
//!
//! A new domain must be integrated through its canonical semantic,
//! representation, provider and execution contracts.
//!
//! Merely registering a new provider must not require changing this module.
//!
//! # Scalability and POCO-REAF
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Computational domains.
//! - Domain descriptors.
//! - Domain-provider implementations.
//! - Providers per domain.
//! - Domains per provider.
//! - Provider interfaces.
//! - Discovery sources.
//! - Discovery observations.
//! - Capability-evidence references.
//! - Resource-requirement references.
//! - Hybrid composition instances.
//! - Provider assignments.
//!
//! Collections and operations are limited only by their actual representation,
//! implementation, configured operational policies and available resources.
//!
//! No finite machine promises literal infinite storage or computation.
//!
//! The architectural objective is to avoid artificial limits that prevent
//! the same program intent from being realized across compatible environments.
//!
//! # Concurrency and ownership
//!
//! Each registry instance is explicitly owned by its caller.
//!
//! Registry synchronization and provider-reference ownership remain inside
//! `registry`.
//!
//! Discovery-source execution remains inside `discovery`.
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No singleton registry.
//! - No implicit provider initialization.
//! - No hidden provider selection.
//! - No implicit resource allocation.
//! - No additional synchronization layer.
//!
//! # Grammar and semantic boundary
//!
//! Source-level domain syntax is owned by the grammar subsystem.
//!
//! Relevant authorities include:
//!
//! - `grammar/declarations/domains.g4`
//! - `grammar/specification/domains.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/spec/resources.md`
//! - `grammar/spec/interoperability.md`
//! - `grammar/spec/hybrid.md`
//! - `grammar/hybrid/`
//!
//! The architectural flow remains:
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Canonical grammar
//!       |
//!       v
//! Frontend AST
//!       |
//!       v
//! Semantic analysis
//!       |
//!       v
//! Canonical domain identity and validated contracts
//!       |
//!       v
//! Compiler / hybrid planning
//!       |
//!       v
//! ZUR domain-provider integration
//!       |
//!       v
//! Validated execution request
//!       |
//!       v
//! Runtime execution and resource authorities
//! ```
//!
//! This module consumes validated representations.
//!
//! It does not parse grammar, interpret source declarations, or make
//! compilation and hardware-realization decisions.
//!
//! # Hybrid integration
//!
//! `composition` is the integration boundary between ZUR domain providers
//! and the canonical Hybrid composition model.
//!
//! The canonical hybrid subsystem retains ownership of:
//!
//! - Hybrid domain composition.
//! - Domain instances.
//! - Domain dependencies.
//! - Graph structure.
//! - Synchronization semantics.
//! - Hybrid planning.
//!
//! ZUR domain composition only resolves compatible provider candidates and
//! explicitly supplied provider assignments for existing hybrid instances.
//!
//! It must not construct another hybrid graph or duplicate hybrid planning.
//!
//! # Public API policy
//!
//! Child modules are publicly accessible through their stable module paths.
//!
//! Selected primary types are re-exported for convenient use.
//!
//! Glob re-exports are deliberately avoided because they can introduce
//! accidental API collisions and obscure ownership as the subsystem grows.
//!
//! Operations and functions that belong to a particular authority should
//! normally be called through that module's explicit path.
//!
//! # Integration policy
//!
//! Adding a future computational domain should normally require changes only
//! to that domain's semantic, representation, provider and execution
//! integration.
//!
//! This composition root should change only when a genuinely new,
//! independently owned domain-integration subsystem is introduced.
//!
//! A new provider implementation must not require modifications to this
//! module merely to become discoverable or registrable.
//!
//! # Completion criteria
//!
//! This module is complete when:
//!
//! - Every declared child module exists.
//! - Every public re-export resolves to an existing public item.
//! - No canonical identity or contract is duplicated.
//! - No closed domain inventory is introduced.
//! - No unsafe Rust or global mutable state is introduced.
//! - No hardware or domain-size ceiling is introduced.
//! - Hybrid composition remains owned by `crate::hybrid`.
//! - Provider registration remains owned by `registry`.
//! - Provider discovery remains owned by `discovery`.
//! - Validation and compatibility remain separate.
//! - The parent ZUR module exposes this module.
//! - The repository's supported build and test configurations pass.

// ============================================================================
// CANONICAL DOMAIN DESCRIPTORS
// ============================================================================

/// Runtime-facing computational-domain descriptors.
///
/// Owns domain descriptor metadata and references to externally owned
/// interfaces, capability evidence, resource requirements, execution models
/// and interoperability contracts.
pub mod descriptor;

// ============================================================================
// DOMAIN PROVIDER CONTRACTS
// ============================================================================

/// Integration contract for independently implemented computational-domain
/// providers.
///
/// Extends the canonical ZUR execution-provider contract without introducing
/// a second provider lifecycle or execution model.
pub mod provider;

// ============================================================================
// DOMAIN PROVIDER REGISTRY
// ============================================================================

/// Explicit registry for domain-provider registration, indexing and lookup.
///
/// Registration does not imply initialization, availability, feasibility,
/// resource reservation or successful execution.
pub mod registry;

// ============================================================================
// DOMAIN PROVIDER DISCOVERY
// ============================================================================

/// Provider-neutral discovery sources, queries, observations and reports.
///
/// Discovery does not implicitly register, initialize, authorize or execute
/// discovered providers.
pub mod discovery;

// ============================================================================
// DOMAIN VALIDATION
// ============================================================================

/// Structural validation of domain descriptors and provider relationships.
///
/// Capability-evidence resolution uses the supplied canonical evidence
/// authority and does not fabricate availability.
pub mod validation;

// ============================================================================
// DOMAIN PROVIDER COMPATIBILITY
// ============================================================================

/// Contract compatibility evaluation between domain descriptors and
/// execution-provider contracts.
///
/// Compatibility does not establish physical feasibility or resource
/// reservation.
pub mod compatibility;

// ============================================================================
// ZUR-TO-HYBRID AND MULTI-PROVIDER COMPOSITION
// ============================================================================

/// Strictly separated integration between ZUR providers and canonical
/// Hybrid composition.
///
/// Also provides explicit multi-provider assignment and binding.
///
/// Does not create another provider registry, hybrid graph, scheduler,
/// resource allocator or execution engine.
pub mod composition;

// ============================================================================
// SELECTIVE PUBLIC RE-EXPORTS
// ============================================================================

/// Domain descriptor and externally owned execution-model references.
pub use descriptor::{
    DomainDescriptor,
    ExecutionModelId,
    ExecutionModelRef,
    InteroperabilityContractRef,
    InteroperabilityDeclaration,
};

/// Domain-provider integration contract.
pub use provider::DomainProvider;

/// Provider registration receipt and registry.
pub use registry::{
    DomainProviderRegistry,
    ProviderRegistration,
};

/// Provider discovery interfaces and result types.
pub use discovery::{
    DiscoveredProvider,
    DiscoveryEngine,
    DiscoveryIssue,
    DiscoveryQuery,
    DiscoveryReport,
    DiscoverySource,
};

/// Domain validation reports and evidence-resolution interfaces.
pub use validation::{
    CapabilityEvidenceObservation,
    CapabilityEvidenceResolver,
    DomainValidationReport,
    ValidatedDomain,
};

/// Domain/provider compatibility reports and classifications.
pub use compatibility::{
    DomainCompatibilityReport,
    DomainCompatibilityStatus,
    InterfaceCompatibility,
    InterfaceCompatibilityStatus,
};

/// Hybrid composition adaptation and explicit provider assignment types.
pub use composition::{
    DomainIdentityMap,
    HybridCompositionAdapter,
    HybridCompositionResolution,
    HybridDomainIdentityResolver,
    HybridInstanceResolution,
    HybridProviderCandidate,
    ProviderAssignment,
    ProviderBinding,
    ProviderComposition,
    ProviderCompositionBuilder,
    QualifiedDomainIdentityResolver,
    ResolvedDomainInterface,
};
