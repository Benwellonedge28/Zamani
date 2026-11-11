
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Hybrid Computing Domain
//!
//! Path: `src/stdlib/domains/hybrid.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a stable standard-library entry point for hybrid and
//! heterogeneous computation in Zamani.
//!
//! Hybrid computation allows one program to compose computation
//! across classical, quantum, HDL, accelerator, AI, tensor,
//! distributed, networking, data, security, and future domains.
//!
//! This module is a facade over the canonical hybrid subsystem.
//! It does not implement a second hybrid language, AST, semantic
//! model, graph, IR, compiler, planner, scheduler, runtime,
//! hardware manager, or provider registry.
//!
//! ## Canonical authorities
//!
//! Source syntax:
//!     grammar/hybrid/
//!
//! Normative hybrid specification:
//!     grammar/spec/hybrid.md
//!
//! Canonical frontend:
//!     crate::frontend
//!
//! Hybrid semantic validation:
//!     crate::frontend::semantic::domains::hybrid
//!
//! Canonical classical computation:
//!     crate::classical
//!
//! Canonical quantum representation:
//!     crate::quantum::ir
//!
//! Canonical HDL subsystem:
//!     crate::hdl
//!
//! Canonical hybrid composition:
//!     crate::hybrid
//!
//! Universal IR:
//!     crate::zuir
//!
//! Compiler:
//!     crate::compiler
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! Hardware abstraction and physical providers:
//!     Their respective canonical repository subsystems.
//!
//! ## Ownership
//!
//! This file owns only:
//!
//! - The standard-library hybrid namespace.
//! - Stable access to the canonical hybrid subsystem.
//! - Explicit public re-exports.
//! - Standard-library facade identity and API version.
//! - Facade-level API construction.
//! - Facade-level compatibility tests.
//!
//! It does not own:
//!
//! - Hybrid source grammar or parsing.
//! - AST construction or source locations.
//! - Semantic validation or type checking.
//! - Domain identities or domain registration.
//! - Hybrid contracts or domain interfaces.
//! - Hybrid graphs, nodes, edges, or graph algorithms.
//! - Cross-domain data and control-flow semantics.
//! - Synchronization semantics.
//! - Resource requirements or capability evidence.
//! - Constraint evaluation or feasibility algorithms.
//! - Hybrid realization plans.
//! - Classical IR, quantum IR, HDLIR, ZUIR, or ZCIR.
//! - Compiler passes or target-specific lowering.
//! - Hardware discovery or physical allocation.
//! - Scheduling, routing, placement, or QEC.
//! - Backend selection or code generation.
//! - Runtime execution or device communication.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! ## POCO-REAF
//!
//! The standard-library interface supports:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! A hybrid program describes computational meaning and
//! cross-domain relationships independently of unnecessary
//! physical machine assumptions.
//!
//! Compatible implementations may realize the same program
//! across different combinations of:
//!
//! - Classical processors.
//! - Quantum processors and simulators.
//! - GPUs and other accelerators.
//! - FPGAs and ASICs.
//! - Embedded systems.
//! - AI and tensor processors.
//! - Distributed systems.
//! - Heterogeneous systems.
//! - Future computational domains.
//!
//! This facade introduces no universal maximum for:
//!
//! - Computational domains.
//! - Domain interfaces.
//! - Hybrid graph nodes or edges.
//! - Operations or dependencies.
//! - Domain crossings.
//! - Values or data transfers.
//! - Synchronization relationships.
//! - Devices or execution nodes.
//! - Processors or accelerators.
//! - Qubits or registers.
//! - Tensor dimensions or tensor rank.
//! - Memory or storage quantities.
//! - Hybrid composition depth.
//!
//! Actual execution remains subject to available resources,
//! representational limits, implementation support, target
//! capabilities, and explicitly configured operational policies.
//!
//! Literal infinite computation is not physically realizable.
//! The requirement is to avoid arbitrary language-level or
//! facade-level capacity ceilings.
//!
//! ## Requirement and realization separation
//!
//! These concepts must remain distinct:
//!
//! - Requirement: what a computation needs.
//! - Capability: what an implementation or execution scope provides.
//! - Preference: what is desirable but not mandatory.
//! - Constraint: what must be respected.
//! - Hint: non-binding implementation guidance.
//! - Feasibility: what can be established from the available evidence.
//!
//! This module does not redefine these concepts. Their canonical
//! contracts remain in the hybrid and semantic subsystems.
//!
//! Missing evidence must not be interpreted as proof of support.
//! Unknown feasibility must not be reported as successful execution.
//! An operational budget must not become a language-level limit.
//!
//! ## Integration pipeline
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical grammar and lexer
//!      |
//!      v
//! Frontend parser and AST
//!      |
//!      v
//! Shared semantic validation
//!      |
//!      v
//! Hybrid semantic composition
//!      |
//!      v
//! Canonical domain representations
//!      |
//!      v
//! Hybrid graph and cross-domain contracts
//!      |
//!      v
//! Compiler and target-independent planning
//!      |
//!      v
//! Target feasibility and lowering
//!      |
//!      v
//! Backend / runtime / HAL / providers
//!      |
//!      v
//! Physical execution
//! ```
//!
//! This facade does not implement or certify every stage in
//! this pipeline.
//!
//! ## Error behavior
//!
//! Canonical hybrid errors and reports must remain unchanged.
//!
//! This facade must not:
//!
//! - Suppress semantic errors.
//! - Convert incomplete analysis into successful validation.
//! - Treat an unknown capability as available.
//! - Treat an infeasible plan as executable.
//! - Conceal unsupported domain crossings.
//! - Silently discard dependencies.
//! - Weaken program requirements.
//! - Report conceptual or simulated execution as physical completion.
//!
//! Callers should use the structured errors and reports supplied
//! by the canonical hybrid APIs.
//!
//! ## Initialization
//!
//! This module is stateless.
//!
//! Constructing or accessing the facade does not:
//!
//! - Initialize the entire standard library.
//! - Initialize the hybrid runtime.
//! - Discover physical devices.
//! - Register global providers.
//! - Allocate hardware resources.
//! - Start background tasks.
//!
//! Lifecycle and execution remain with their canonical owners.
//!
//! ## Compatibility
//!
//! Every hybrid subsystem module and public type exposed here is
//! owned by crate::hybrid.
//!
//! This facade does not copy canonical types or implementations.
//!
//! The facade API version is independent of the language version,
//! Cargo package version, grammar version, semantic schema version,
//! IR version, and hybrid contract version.
//!
//! ## Completion criteria
//!
//! - The canonical hybrid module is exposed at the crate root.
//! - The canonical frontend is exposed at the crate root.
//! - The standard-library domains module is registered.
//! - Every re-export resolves to an existing canonical module.
//! - The public API uses the canonical HybridApi.
//! - No competing hybrid authority is introduced.
//! - No arbitrary hardware-size ceiling is introduced.
//! - Safe Rust is maintained.
//! - Facade tests pass.
//! - Repository formatting, compilation, tests, and Clippy pass.

// =============================================================================
// Standard-library facade identity
// =============================================================================

/// Stable standard-library identity for the hybrid domain.
pub const HYBRID_STDLIB_DOMAIN_NAME: &str = "zamani.stdlib.hybrid";

/// Version of the standard-library hybrid facade contract.
///
/// This is not the version of the language, grammar, hybrid
/// semantic schema, hybrid graph, or hybrid IR.
pub const HYBRID_STDLIB_DOMAIN_API_VERSION: u16 = 1;

/// Returns the stable standard-library hybrid domain identity.
#[must_use]
pub const fn domain_name() -> &'static str {
    HYBRID_STDLIB_DOMAIN_NAME
}

/// Returns the standard-library hybrid facade API version.
#[must_use]
pub const fn domain_api_version() -> u16 {
    HYBRID_STDLIB_DOMAIN_API_VERSION
}

// =============================================================================
// Canonical hybrid subsystem namespaces
// =============================================================================

/// Canonical hybrid public API module.
pub use crate::hybrid::api;

/// Canonical hybrid configuration and operational-budget policies.
pub use crate::hybrid::config;

/// Canonical hybrid domain-independent contracts.
pub use crate::hybrid::contract;

/// Canonical hybrid diagnostic facilities.
pub use crate::hybrid::diagnostics;

/// Canonical structured hybrid errors.
pub use crate::hybrid::errors;

/// Canonical hybrid identity definitions.
pub use crate::hybrid::ids;

/// Canonical hybrid contract and representation versions.
pub use crate::hybrid::version;

/// Canonical computational-domain identities, interfaces,
/// registries, and composition.
pub use crate::hybrid::domain;

/// Canonical hybrid dependency graph and graph analysis.
pub use crate::hybrid::graph;

/// Canonical hybrid data exchange and value contracts.
pub use crate::hybrid::data;

/// Canonical hybrid control-flow contracts.
pub use crate::hybrid::control;

/// Canonical hybrid synchronization contracts.
pub use crate::hybrid::sync;

/// Canonical hybrid resource, capability, constraint,
/// and feasibility integration.
pub use crate::hybrid::resources;

/// Canonical target-independent hybrid planning contracts.
pub use crate::hybrid::planning;

/// Canonical hybrid representation interchange.
pub use crate::hybrid::interchange;

/// Canonical integration adapters for the existing repository
/// frontend, semantic, classical, and quantum subsystems.
pub use crate::hybrid::integration;

/// Canonical hybrid architectural invariants and compatibility rules.
pub use crate::hybrid::spec;

// =============================================================================
// Canonical public API
// =============================================================================

/// Canonical hybrid API for contract validation.
///
/// This is the existing API from `crate::hybrid::api`.
/// No replacement API or validation engine is defined here.
pub use crate::hybrid::HybridApi;

/// Canonical report returned by hybrid contract validation.
pub use crate::hybrid::ContractValidationReport;

/// Constructs the canonical hybrid API with its documented defaults.
///
/// This function does not initialize a runtime, discover hardware,
/// register providers, or allocate physical resources.
#[must_use]
pub fn new_api() -> HybridApi {
    HybridApi::default()
}

// =============================================================================
// Facade contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_library_domain_identity_is_stable() {
        assert_eq!(domain_name(), "zamani.stdlib.hybrid");
        assert_eq!(domain_name(), HYBRID_STDLIB_DOMAIN_NAME);
    }

    #[test]
    fn standard_library_api_version_is_nonzero() {
        assert_ne!(domain_api_version(), 0);
        assert_eq!(
            domain_api_version(),
            HYBRID_STDLIB_DOMAIN_API_VERSION
        );
    }

    #[test]
    fn facade_uses_the_canonical_hybrid_api_type() {
        fn accepts_canonical_api(_: &crate::hybrid::HybridApi) {}

        let standard_library_api = new_api();

        accepts_canonical_api(&standard_library_api);
    }

    #[test]
    fn facade_exposes_the_canonical_validation_report_type() {
        let _ = core::any::type_name::<ContractValidationReport>();
        let _ = core::any::type_name::<crate::hybrid::ContractValidationReport>();
    }

    #[test]
    fn default_api_uses_the_canonical_configuration() {
        let standard_library_api = new_api();

        let _: &crate::hybrid::config::HybridConfig =
            standard_library_api.config();
    }

    #[test]
    fn canonical_hybrid_subsystems_are_reachable() {
        let _ = core::any::type_name::<
            crate::hybrid::graph::model::HybridGraph
        >();

        let _ = core::any::type_name::<
            crate::hybrid::planning::HybridPlan
        >();

        let _ = core::any::type_name::<
            crate::hybrid::domain::DomainId
        >();
    }
}
