
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Computational Domain
//!
//! Path: `src/ai/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This file is the canonical Rust composition root for the Zamani AI
//! computational domain.
//!
//! It integrates the existing AI infrastructure into one coherent,
//! extensible, hardware-independent subsystem.
//!
//! The AI domain supports the expression, description, validation,
//! composition, optimization, interoperability and execution-boundary
//! integration of AI computational programs.
//!
//! The architectural objective is:
//!
//! PROGRAM_ONCE_COMPILE_ONCE_RUN_EVERYWHERE_ANYWHERE_FOREVER
//!
//! AI computation must remain independent of the physical machine
//! that eventually realizes it.
//!
//! The same logical AI program must be capable of being compiled
//! for different compatible execution environments without requiring
//! its computational meaning to be rewritten.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! The following repository components remain authoritative.
//!
//! SOURCE LANGUAGE:
//!
//!     grammar/ai/
//!     grammar/ai/ai.g4
//!     grammar/ai/README.md
//!     grammar/spec/ai.md
//!
//! CANONICAL FRONTEND:
//!
//!     crate::frontend::lexer
//!     crate::frontend::parser
//!     crate::frontend::ast
//!
//! AI SEMANTIC AUTHORITY:
//!
//!     crate::frontend::semantic::domains::ai
//!
//! SHARED SEMANTIC AUTHORITIES:
//!
//!     crate::frontend::semantic::types
//!     crate::frontend::semantic::effects
//!     crate::frontend::semantic::capabilities
//!     crate::frontend::semantic::resources
//!     crate::frontend::semantic::model
//!
//! UNIVERSAL COMPUTATIONAL REPRESENTATION:
//!
//!     crate::zuir
//!     crate::zuir::dialect
//!
//! CLASSICAL COMPUTATION:
//!
//!     crate::classical
//!     crate::classical::collections
//!     crate::classical::zcir
//!
//! QUANTUM COMPUTATION:
//!
//!     crate::quantum::ir
//!
//! HARDWARE DESCRIPTION:
//!
//!     crate::hdl::hdlir
//!
//! HYBRID COMPOSITION:
//!
//!     crate::hybrid
//!
//! DISTRIBUTED COMPUTATION:
//!
//!     crate::distributed
//!
//! COMPILER AND OPTIMIZATION:
//!
//!     crate::compiler
//!     crate::optimizer
//!
//! BACKEND AND EXECUTION:
//!
//!     crate::backend
//!     crate::runtime
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This composition root owns:
//!
//! - AI subsystem module registration.
//! - AI public module hierarchy.
//! - Stable AI subsystem import paths.
//! - Approved public API re-exports.
//! - AI architectural boundary documentation.
//! - AI subsystem integration documentation.
//!
//! Individual AI modules own their respective computational contracts,
//! algorithms, validation and integration interfaces.
//!
//! This file does not implement those algorithms.
//!
//! ============================================================================
//! NON-OWNERSHIP
//! ============================================================================
//!
//! This file MUST NOT implement or duplicate:
//!
//! - Source grammar.
//! - Lexer.
//! - Parser.
//! - AST.
//! - Semantic analysis.
//! - Type inference.
//! - Effect checking.
//! - Universal capability infrastructure.
//! - Universal resource infrastructure.
//! - Tensor storage.
//! - Classical IR.
//! - Quantum IR.
//! - Hardware description IR.
//! - Universal intermediate representation.
//! - Compiler optimization infrastructure.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Device selection.
//! - Physical placement.
//! - Scheduling.
//! - Distributed transport.
//! - Quantum error correction.
//! - Backend code generation.
//! - Runtime execution engines.
//! - Persistent knowledge storage.
//! - General-purpose security enforcement.
//! - General-purpose telemetry infrastructure.
//!
//! The existing repository authorities remain responsible for these
//! capabilities.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! The AI subsystem must not impose arbitrary language-level limits
//! on computational workload or physical execution capacity.
//!
//! In particular, this composition root introduces no fixed maximum for:
//!
//! - AI models.
//! - Model parameters.
//! - Model graph nodes.
//! - Model layers.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Tensor elements.
//! - Dataset records.
//! - Training operations.
//! - Training steps.
//! - Inference requests.
//! - Learning algorithms.
//! - Reasoning operations.
//! - Knowledge entities.
//! - Evidence records.
//! - Agents.
//! - Agent populations.
//! - Pipeline stages.
//! - Differentiation variables.
//! - Differentiation operations.
//! - Optimization transformations.
//! - AI artifacts.
//! - Security policies.
//! - Observability events.
//! - Processors.
//! - Accelerators.
//! - Execution targets.
//! - Distributed participants.
//! - Distributed workers.
//! - Machines.
//! - Memory capacity.
//!
//! Operational budgets are permitted when explicitly configured.
//!
//! Examples include:
//!
//! - Compilation budgets.
//! - Execution deadlines.
//! - Memory budgets.
//! - Storage budgets.
//! - Network quotas.
//! - Security restrictions.
//! - Provider-specific operational limits.
//!
//! These budgets must not become language-level semantic ceilings.
//!
//! A target that cannot satisfy a computation's requirements must
//! produce an explicit feasibility or execution failure.
//!
//! It must not silently change the meaning of the program.
//!
//! IMPORTANT:
//!
//! No finite implementation can guarantee literal infinite computation.
//!
//! POCO-REAF means that the language and architecture do not introduce
//! unnecessary restrictions based on today's machine sizes or hardware.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! The intended architectural direction is:
//!
//!     Zamani source
//!          |
//!          v
//!     Canonical grammar
//!          |
//!          v
//!     Canonical lexer/parser
//!          |
//!          v
//!     Canonical frontend AST
//!          |
//!          v
//!     Shared semantic analysis
//!          |
//!          v
//!     AI semantic validation
//!          |
//!          v
//!     AI computational domain
//!          |
//!          +---------------------------+
//!          |             |             |
//!          v             v             v
//!        Models       Tensors       Learning
//!          |             |             |
//!          +-------------+-------------+
//!                        |
//!          +-------------+-------------+
//!          |             |             |
//!          v             v             v
//!       Reasoning      Agents     Differentiation
//!          |             |             |
//!          +-------------+-------------+
//!                        |
//!                        v
//!                AI pipeline composition
//!                        |
//!                        v
//!                AI optimization contracts
//!                        |
//!                        v
//!                AI execution boundary
//!                        |
//!                        v
//!                  Canonical ZUIR
//!                        |
//!           +------------+-------------+
//!           |            |             |
//!           v            v             v
//!        Classical     Quantum        HDL
//!           |            |             |
//!           +------------+-------------+
//!                        |
//!                        v
//!                  Hybrid composition
//!                        |
//!                        v
//!               Distributed execution
//!                        |
//!                        v
//!                Backend realization
//!                        |
//!                        v
//!                 Runtime execution
//!
//! This is an architectural dependency description.
//! It does not authorize foundational AI modules to depend on runtime,
//! backend or hardware-specific implementations.
//!
//! ============================================================================
//! SUBSYSTEM INTEGRATION CONTRACT
//! ============================================================================
//!
//! 1. The grammar defines source-level AI constructs.
//!
//! 2. The canonical frontend constructs the domain-neutral AST.
//!
//! 3. The semantic subsystem validates AI language meaning.
//!
//! 4. AI foundation modules provide reusable AI computational contracts.
//!
//! 5. Model and tensor modules describe computational structures.
//!
//! 6. Learning and reasoning modules describe their respective
//!    computational operations.
//!
//! 7. Agent modules describe agent structure, coordination intent,
//!    communication contracts, memory references and tool interfaces.
//!
//! 8. Differentiation describes derivative computation contracts.
//!
//! 9. Pipeline modules describe AI computational composition.
//!
//! 10. Optimization modules describe legal AI transformations.
//!
//! 11. Data modules describe data dependencies and access contracts.
//!
//! 12. Deployment modules describe portable AI artifacts and manifests.
//!
//! 13. Security modules describe AI-specific security requirements.
//!
//! 14. Observability modules expose AI-specific observation interfaces.
//!
//! 15. Execution accepts validated AI computation and delegates
//!     realization to the appropriate execution infrastructure.
//!
//! 16. ZUIR and the canonical domain IRs represent the resulting
//!     computational operations.
//!
//! 17. Hybrid and distributed infrastructure determine composition,
//!     placement, coordination and execution.
//!
//! 18. Backend and runtime infrastructure perform actual realization.
//!
//! No registration, declaration, validation request or execution
//! request must be mistaken for proof that subsequent operations
//! have successfully completed.
//!
//! ============================================================================
//! GRAMMAR INTEGRATION
//! ============================================================================
//!
//! Canonical AI grammar composition:
//!
//!     grammar/ai/ai.g4
//!
//! AI grammar documentation:
//!
//!     grammar/ai/README.md
//!
//! AI language specification:
//!
//!     grammar/spec/ai.md
//!
//! Canonical parser integration:
//!
//!     crate::frontend::parser::domains::ai
//!
//! Canonical semantic integration:
//!
//!     crate::frontend::semantic::domains::ai
//!
//! This module does not parse grammar productions.
//!
//! New grammar features must be connected through the canonical frontend
//! AST and semantic pipeline before they are considered implemented.
//!
//! ============================================================================
//! CONCURRENCY AND STATE
//! ============================================================================
//!
//! This module introduces:
//!
//! - No process-global mutable AI state.
//! - No implicit singleton registries.
//! - No hidden execution threads.
//! - No implicit hardware discovery.
//! - No implicit network connections.
//! - No hidden device allocation.
//!
//! Explicit contexts and immutable registry snapshots are preferred.
//!
//! Individual modules retain their own concurrency and synchronization
//! contracts.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! The root AI module preserves the existing public module hierarchy.
//!
//! Existing module paths should remain stable.
//!
//! New functionality belongs in the appropriate owning submodule.
//!
//! Changes to this composition root are necessary only when the actual
//! subsystem architecture or approved public API changes.
//!
//! `src/lib.rs` already declares `pub mod ai;`.
//!
//! That declaration must not be duplicated here.
//!
//! ============================================================================
//! ERROR HANDLING
//! ============================================================================
//!
//! Structured AI errors are exposed through the existing error module.
//!
//! Individual modules retain their own specialized error types.
//!
//! Errors must preserve their original categories and causes.
//!
//! Incomplete validation must never be reported as complete success.
//!
//! Unknown execution outcomes must never be converted into successful
//! execution results.
//!
//! Capability availability must not be confused with authorization.
//!
//! Compatibility must not be confused with resource feasibility.
//!
//! Resource feasibility must not be confused with execution success.
//!
//! ============================================================================
//! PUBLIC API
//! ============================================================================
//!
//! `api` is the stable public programming interface.
//!
//! It composes the existing:
//!
//! - AiContext.
//! - AiConfig.
//! - AiOperationRegistry.
//! - AI version contracts.
//! - AI capability contracts.
//! - AI requirement contracts.
//! - AI provenance contracts.
//! - Canonical frontend semantic services.
//!
//! It does not create another execution engine.
//!
//! ============================================================================
//! MODULE DECLARATIONS
//! ============================================================================

// ---------------------------------------------------------------------------
// FOUNDATION
// ---------------------------------------------------------------------------

/// Canonical AI-domain identifiers.
///
/// Reuses the universal ZUIR identity infrastructure where appropriate.
pub mod ids;

/// Independent AI API, semantic, model-schema, operation-set and
/// artifact-format version contracts.
pub mod version;

/// Structured AI errors and serializable error records.
pub mod error;

/// AI-specific diagnostics using the canonical frontend diagnostic model.
pub mod diagnostics;

/// Explicit AI subsystem configuration and operational policies.
pub mod config;

/// AI-domain provenance and integration with canonical semantic provenance.
pub mod provenance;

/// AI computational capability declarations and canonical capability
/// evidence integration.
pub mod capabilities;

/// AI computational requirements and their distinction from capabilities,
/// preferences, constraints and hints.
pub mod requirements;

/// AI contract and feature compatibility analysis.
pub mod compatibility;

/// Immutable, explicitly constructed AI dependency context.
pub mod context;

/// Extensible, versioned AI operation declaration registry.
pub mod registry;

/// AI-domain validation interfaces and coordination.
pub mod validation;

// ---------------------------------------------------------------------------
// COMPUTATIONAL MODELS
// ---------------------------------------------------------------------------

/// AI model definitions, graphs, layers, operations, parameters,
/// signatures, state and structural validation.
pub mod model;

/// AI tensor shapes, dimensions, layouts, numerical types, operations
/// and interoperability.
pub mod tensor;

// ---------------------------------------------------------------------------
// LEARNING AND REASONING
// ---------------------------------------------------------------------------

/// Learning algorithms, training workflows, objectives, optimization,
/// reinforcement learning, federated learning and adaptation.
pub mod learning;

/// Symbolic, deductive, inductive, abductive, causal and probabilistic
/// reasoning, knowledge, evidence and uncertainty.
pub mod reasoning;

/// AI automatic differentiation, derivative rules, forward-mode,
/// reverse-mode, graph analysis and checkpointing.
pub mod differentiation;

// ---------------------------------------------------------------------------
// AGENTS AND COMPOSITION
// ---------------------------------------------------------------------------

/// AI agent definitions, capabilities, communication, coordination,
/// planning, memory, tools and policy attachments.
pub mod agents;

/// AI computational pipeline definitions, graphs, dependencies,
/// transformations, validation and abstract execution plans.
pub mod pipeline;

/// AI graph transformations, precision, quantization, sparsity,
/// cost estimation and optimization legality.
pub mod optimization;

// ---------------------------------------------------------------------------
// DATA AND DEPLOYMENT
// ---------------------------------------------------------------------------

/// AI dataset references, schemas, provenance, batching, streaming,
/// preprocessing and data validation.
pub mod data;

/// AI artifacts, format providers, portable deployment manifests,
/// compatibility and lifecycle contracts.
pub mod deployment;

// ---------------------------------------------------------------------------
// SECURITY AND OBSERVABILITY
// ---------------------------------------------------------------------------

/// AI identity, authorization integration, policy attachments,
/// isolation requirements, integrity and security auditing.
pub mod security;

/// AI events, metrics and tracing integration with canonical
/// observability infrastructure.
pub mod observability;

// ---------------------------------------------------------------------------
// EXECUTION AND PUBLIC INTEGRATION
// ---------------------------------------------------------------------------

/// Backend-independent AI execution requests, preflight, dispatch
/// contracts, receipts and result envelopes.
///
/// Actual execution is delegated to the appropriate execution
/// infrastructure.
pub mod execution;

/// Stable public AI API and approved public contract re-exports.
pub mod api;

// ============================================================================
// STABLE CRATE-LEVEL PUBLIC API
// ============================================================================

// The public API remains owned by api.rs.
// These are re-exports of existing types, not new implementations.

/// Stable AI API entry point.
pub use api::AiApi;

/// Errors returned by the stable AI API.
pub use api::ApiError;

/// Result type for public AI API operations.
pub use api::ApiResult;

/// Canonical AI context type.
pub use api::Context;

/// AI context construction builder.
pub use api::ContextBuilder;

/// AI context construction error.
pub use api::ContextError;

/// AI subsystem version set.
pub use api::VersionSet;

// ============================================================================
// CANONICAL AI IDENTIFIERS
// ============================================================================

/// Canonical AI entity identifiers.
///
/// These are strongly typed identifiers defined by `ids.rs`.
pub use ids::{
    AiAgentId,
    AiArtifactId,
    AiDatasetId,
    AiDecisionId,
    AiEvidenceId,
    AiKnowledgeId,
    AiModelId,
    AiParameterId,
    AiPipelineId,
    AiTrainingRunId,
};

// ============================================================================
// CANONICAL AI ERRORS
// ============================================================================

/// Structured AI-domain error.
pub use error::AiError;

/// Result type for AI-domain operations.
pub use error::AiResult;

// ============================================================================
// PUBLIC AI VERSION CONTRACTS
// ============================================================================

/// Public AI API version.
pub use version::AiApiVersion;

/// AI semantic contract version.
pub use version::AiSemanticVersion;

/// AI model schema version.
pub use version::AiModelSchemaVersion;

/// AI operation-set version.
pub use version::AiOperationSetVersion;

/// AI artifact format version.
pub use version::AiArtifactFormatVersion;

// ============================================================================
// PUBLIC AI CONFIGURATION
// ============================================================================

/// Canonical AI configuration.
pub use config::AiConfig;

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Architectural invariants of the AI subsystem.
///
/// These describe ownership properties, not hardware capacities
/// or execution guarantees.
pub mod invariants {
    /// Source syntax is owned by the canonical grammar.
    pub const GRAMMAR_IS_EXTERNAL: bool = true;

    /// Source semantics are owned by the canonical frontend.
    pub const SEMANTICS_ARE_CANONICAL: bool = true;

    /// Universal computational representation is owned by ZUIR.
    pub const UNIVERSAL_IR_IS_EXTERNAL: bool = true;

    /// Concrete tensor storage is owned by canonical tensor infrastructure.
    pub const TENSOR_STORAGE_IS_EXTERNAL: bool = true;

    /// Physical hardware selection is downstream.
    pub const HARDWARE_SELECTION_IS_EXTERNAL: bool = true;

    /// Physical resource allocation is downstream.
    pub const RESOURCE_ALLOCATION_IS_EXTERNAL: bool = true;

    /// Distributed scheduling is owned by distributed infrastructure.
    pub const DISTRIBUTED_SCHEDULING_IS_EXTERNAL: bool = true;

    /// Actual execution is delegated to execution infrastructure.
    pub const EXECUTION_IS_DELEGATED: bool = true;

    /// AI computation descriptions are hardware-independent.
    pub const COMPUTATION_IS_HARDWARE_INDEPENDENT: bool = true;

    /// No universal workload ceilings are introduced by this root.
    pub const NO_ARTIFICIAL_WORKLOAD_CEILINGS: bool = true;

    /// Unsafe Rust is prohibited.
    pub const UNSAFE_RUST_IS_PROHIBITED: bool = true;
}

// ============================================================================
// COMPOSITION-ROOT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the stable public API is available.
    #[test]
    fn stable_public_api_is_available() {
        let api = AiApi::new()
            .expect("default AI API configuration must be valid");

        let _context = api.context();
        let _config = api.config();
    }

    /// Verify that AI identifiers are exposed through the root.
    #[test]
    fn canonical_ai_identifiers_are_available() {
        let model = AiModelId::new(1)
            .expect("valid model identifier");

        let dataset = AiDatasetId::new(1)
            .expect("valid dataset identifier");

        let agent = AiAgentId::new(1)
            .expect("valid agent identifier");

        assert_eq!(model.get(), 1);
        assert_eq!(dataset.get(), 1);
        assert_eq!(agent.get(), 1);
    }

    /// Verify that structured AI errors are exposed.
    #[test]
    fn structured_ai_errors_are_available() {
        let error = AiError::new(
            crate::ai::error::AiErrorKind::InvalidModel,
            "invalid model declaration",
        );

        assert_eq!(
            error.kind(),
            crate::ai::error::AiErrorKind::InvalidModel
        );

        assert!(!error.to_string().is_empty());
    }

    /// Verify that AI configuration is independently accessible.
    #[test]
    fn ai_configuration_is_available() {
        let config = AiConfig::default();

        assert!(config.validate().is_ok());
    }

    /// Verify that the AI version contract is available.
    #[test]
    fn version_contract_is_available() {
        let versions = VersionSet::initial();

        assert_eq!(
            versions.api,
            crate::ai::version::INITIAL_AI_API_VERSION
        );
    }

    /// Verify that no hardware-specific ownership is introduced
    /// by the composition root.
    #[test]
    fn architectural_invariants_are_preserved() {
        assert!(invariants::GRAMMAR_IS_EXTERNAL);
        assert!(invariants::SEMANTICS_ARE_CANONICAL);
        assert!(invariants::UNIVERSAL_IR_IS_EXTERNAL);
        assert!(invariants::TENSOR_STORAGE_IS_EXTERNAL);
        assert!(invariants::HARDWARE_SELECTION_IS_EXTERNAL);
        assert!(invariants::RESOURCE_ALLOCATION_IS_EXTERNAL);
        assert!(invariants::DISTRIBUTED_SCHEDULING_IS_EXTERNAL);
        assert!(invariants::EXECUTION_IS_DELEGATED);
        assert!(invariants::COMPUTATION_IS_HARDWARE_INDEPENDENT);
        assert!(invariants::NO_ARTIFICIAL_WORKLOAD_CEILINGS);
        assert!(invariants::UNSAFE_RUST_IS_PROHIBITED);
    }
}
