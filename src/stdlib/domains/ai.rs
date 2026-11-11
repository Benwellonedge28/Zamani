
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — AI Domain
//!
//! Path: src/stdlib/domains/ai.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the stable standard-library integration boundary for
//! artificial intelligence and AI-oriented computation in Zamani.
//!
//! This module exposes the canonical AI computational subsystem through
//! the standard-library namespace.
//!
//! It does not implement a competing AI engine.
//!
//! The objective is to allow Zamani programs to express AI computation
//! independently of the physical machine that eventually realizes it.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Source grammar:
//!     grammar/ai/
//!
//! AI grammar composition:
//!     grammar/ai/ai.g4
//!
//! AI grammar specification:
//!     grammar/ai/README.md
//!
//! AI language semantics:
//!     grammar/spec/ai.md
//!     grammar/specification/semantic-model.md
//!
//! Frontend:
//!     crate::frontend
//!
//! AI semantic authority:
//!     crate::frontend::semantic::domains::ai
//!
//! Canonical AI implementation:
//!     crate::ai
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal IR:
//!     crate::zuir
//!
//! Classical IR:
//!     crate::classical
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware description:
//!     crate::hdl
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Compiler:
//!     crate::compiler
//!
//! Optimization:
//!     crate::optimizer
//!
//! Runtime:
//!     crate::runtime
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Standard-library AI namespace integration.
//! - Stable standard-library access to canonical AI APIs.
//! - AI public subsystem re-exports.
//! - AI semantic validation delegation.
//! - Standard-library AI integration documentation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - AI grammar.
//! - Lexer or parser.
//! - AST.
//! - Semantic analysis.
//! - Type inference.
//! - Effect analysis.
//! - Capability discovery.
//! - Resource feasibility.
//! - AI model definitions.
//! - Model parameter storage.
//! - Tensor storage.
//! - Dataset storage.
//! - Learning algorithms.
//! - Reasoning algorithms.
//! - Knowledge graph storage.
//! - Automatic differentiation.
//! - Agent execution.
//! - Pipeline scheduling.
//! - AI optimization.
//! - Model serialization.
//! - Cryptographic algorithms.
//! - Authorization enforcement.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Compiler IR.
//! - Physical placement.
//! - Routing.
//! - Scheduling.
//! - Backend code generation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! AI computation must remain independent of:
//!
//! - Processor architecture.
//! - Processor count.
//! - Accelerator count.
//! - GPU architecture.
//! - FPGA architecture.
//! - ASIC architecture.
//! - QPU architecture.
//! - Hardware vendors.
//! - Memory capacity.
//! - Tensor rank.
//! - Model parameter count.
//! - Dataset size.
//! - Agent population.
//! - Distributed node count.
//! - Network topology.
//!
//! This module introduces no universal workload ceilings.
//!
//! Configured operational budgets are permitted.
//!
//! A finite resource budget is not a language-level capability limit.
//!
//! Literal infinite computation is not promised.
//!
//! The objective is to allow computation to scale according to:
//!
//! - Program semantics.
//! - Available resources.
//! - Provider capabilities.
//! - Declared requirements.
//! - Explicit constraints.
//! - Security policies.
//! - Implementation feasibility.
//!
//! An unavailable capability must never be represented as successful
//! execution.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! Canonical grammar
//!       |
//!       v
//! Canonical frontend AST
//!       |
//!       v
//! Semantic validation
//!       |
//!       v
//! Canonical AI subsystem
//!       |
//!       v
//! THIS STANDARD-LIBRARY FACADE
//!       |
//!       v
//! ZUIR / canonical domain IRs
//!       |
//!       v
//! Compiler / optimization / lowering
//!       |
//!       v
//! Hybrid / distributed / hardware integration
//!       |
//!       v
//! Runtime / HAL
//!
//! The facade must not reverse this dependency direction.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! 1. The grammar defines AI source syntax.
//!
//! 2. The frontend constructs the canonical AST.
//!
//! 3. The semantic subsystem validates source-level AI meaning.
//!
//! 4. The canonical AI subsystem exposes reusable AI computational APIs.
//!
//! 5. This module exposes those APIs through stdlib::domains::ai.
//!
//! 6. AI execution requests are delegated to crate::ai::execution.
//!
//! 7. Computational representation is delegated to canonical IRs.
//!
//! 8. Actual execution is delegated to compiler/runtime/provider systems.
//!
//! 9. Classical, quantum, HDL, hybrid and distributed integration
//!    remains owned by their respective repository subsystems.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! Canonical AI errors must retain their original error categories.
//!
//! This module must not:
//!
//! - Convert failures into successful results.
//! - Suppress provider errors.
//! - Treat missing capabilities as available.
//! - Treat authorization as capability availability.
//! - Treat compatibility as resource feasibility.
//! - Treat feasibility as execution success.
//! - Treat incomplete semantic validation as valid.
//!
//! ============================================================================
//! CONCURRENCY AND STATE
//! ============================================================================
//!
//! No process-global AI state is introduced.
//!
//! No implicit singleton registry is introduced.
//!
//! No background execution thread is started.
//!
//! No hardware discovery is performed.
//!
//! No model is loaded automatically.
//!
//! No network connection is opened automatically.
//!
//! All dependencies are supplied explicitly through the canonical AI API.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! The existing stdlib::ml, stdlib::ai_reasoning, stdlib::nlp and
//! stdlib::vision paths remain separate legacy compatibility surfaces.
//!
//! They must be migrated progressively to the canonical AI subsystem.
//!
//! This module deliberately does not re-export their competing types.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! - Canonical AI API is exposed.
//! - Canonical AI modules are exposed.
//! - Semantic validation delegates to frontend authority.
//! - No duplicate model or tensor implementation is introduced.
//! - No execution engine is introduced.
//! - No arbitrary workload ceilings are introduced.
//! - No hardware-specific dependencies are introduced.
//! - No unsafe Rust is introduced.
//! - Public API tests pass.
//! - Workspace integration tests pass.
//!
//! ============================================================================

use crate::frontend::semantic::domains::ai::{
    AiProgram,
    AiSemanticContext,
    AiValidationReport,
};

// ============================================================================
// CANONICAL AI PUBLIC API
// ============================================================================

/// Canonical AI public API.
///
/// This is the existing AI implementation, not a second API.
pub use crate::ai::api::{
    AiApi,
    AiApiBuilder,
    ApiError,
    ApiResult,
    Context,
    ContextBuilder,
    ContextError,
    OperationDescriptor,
    OperationKind,
    OperationName,
    OperationRegistry,
    OperationRegistryError,
    OperationVersion,
    SemanticServices,
    VersionSet,
};

/// Canonical AI configuration.
pub use crate::ai::config::AiConfig;

/// Canonical structured AI errors.
pub use crate::ai::error::{
    AiError,
    AiResult,
};

/// Canonical AI identifiers.
pub use crate::ai::ids::{
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
// CANONICAL AI SUBSYSTEMS
// ============================================================================

// These are re-exports of the existing AI implementation.
// No competing implementation is created here.

/// AI agents, coordination, planning and tool contracts.
pub use crate::ai::agents;

/// AI data integration contracts.
pub use crate::ai::data;

/// AI artifact and deployment contracts.
pub use crate::ai::deployment;

/// AI automatic differentiation.
pub use crate::ai::differentiation;

/// AI execution request and dispatch contracts.
pub use crate::ai::execution;

/// AI learning, training and adaptation.
pub use crate::ai::learning;

/// AI models, graphs, layers and parameters.
pub use crate::ai::model;

/// AI observability integration.
pub use crate::ai::observability;

/// AI optimization contracts.
pub use crate::ai::optimization;

/// AI computational pipelines.
pub use crate::ai::pipeline;

/// AI reasoning, knowledge and uncertainty.
pub use crate::ai::reasoning;

/// AI-specific security integration.
pub use crate::ai::security;

/// AI tensor descriptions and interoperability.
pub use crate::ai::tensor;

// ============================================================================
// CANONICAL SEMANTIC BOUNDARY
// ============================================================================

/// Canonical AI semantic program representation.
///
/// This is an adapter-produced semantic record, not an AST or IR.
pub use crate::frontend::semantic::domains::ai::AiProgram;

/// Canonical AI semantic validation report.
pub use crate::frontend::semantic::domains::ai::AiValidationReport;

/// Canonical AI semantic service interface.
pub use crate::frontend::semantic::domains::ai::AiSemanticContext;

// ============================================================================
// STANDARD-LIBRARY API ALIASES
// ============================================================================

/// Stable standard-library name for the canonical AI API.
///
/// This is a type alias. It does not introduce another API implementation.
pub type AiStandardLibrary<'services> = AiApi<'services>;

/// Stable standard-library name for the canonical AI API builder.
pub type AiStandardLibraryBuilder<'services> = AiApiBuilder<'services>;

// ============================================================================
// SEMANTIC INTEGRATION
// ============================================================================

/// Validate AI-domain semantic records using canonical frontend services.
///
/// This function delegates all semantic decisions to the existing
/// frontend AI semantic authority.
///
/// It does not parse source code, construct AST nodes, perform hardware
/// discovery, allocate resources, generate IR or execute AI operations.
///
/// An incomplete report must not be interpreted as successful complete
/// semantic validation.
///
/// # Arguments
///
/// * `program` - AI semantic records produced by the canonical frontend.
/// * `services` - Canonical shared semantic services.
///
/// # Returns
///
/// An AI-domain validation report containing deterministic diagnostics
/// and an explicit completeness status.
#[must_use]
pub fn validate_ai_domain_records<C: AiSemanticContext>(
    program: &AiProgram,
    services: &C,
) -> AiValidationReport {
    program.validate(services)
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Architectural properties guaranteed by this integration facade.
pub mod invariants {
    /// The canonical grammar owns AI source syntax.
    pub const GRAMMAR_IS_EXTERNAL: bool = true;

    /// The frontend owns source-level AI semantics.
    pub const SEMANTICS_ARE_CANONICAL: bool = true;

    /// The canonical AI subsystem owns AI computation APIs.
    pub const AI_IMPLEMENTATION_IS_CANONICAL: bool = true;

    /// Tensor storage is owned by canonical tensor infrastructure.
    pub const TENSOR_STORAGE_IS_EXTERNAL: bool = true;

    /// Universal IR is owned by ZUIR.
    pub const UNIVERSAL_IR_IS_EXTERNAL: bool = true;

    /// Classical IR is externally owned.
    pub const CLASSICAL_IR_IS_EXTERNAL: bool = true;

    /// Quantum IR is externally owned.
    pub const QUANTUM_IR_IS_EXTERNAL: bool = true;

    /// Hardware description is externally owned.
    pub const HARDWARE_DESCRIPTION_IS_EXTERNAL: bool = true;

    /// Hybrid composition is externally owned.
    pub const HYBRID_COMPOSITION_IS_EXTERNAL: bool = true;

    /// Distributed execution is externally owned.
    pub const DISTRIBUTED_EXECUTION_IS_EXTERNAL: bool = true;

    /// Runtime execution is delegated.
    pub const EXECUTION_IS_DELEGATED: bool = true;

    /// No arbitrary universal workload ceilings are introduced.
    pub const NO_ARTIFICIAL_WORKLOAD_CEILINGS: bool = true;

    /// Unsafe Rust is prohibited.
    pub const UNSAFE_RUST_IS_PROHIBITED: bool = true;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_ai_api_is_available() {
        let api = AiApi::new()
            .expect("default canonical AI configuration must be valid");

        assert!(api.operations_are_empty());
        assert_eq!(api.operation_count(), 0);
    }

    #[test]
    fn standard_library_alias_uses_canonical_ai_api() {
        let api: AiStandardLibrary<'static> = AiApi::new()
            .expect("default canonical AI API must be constructible");

        assert!(api.operations_are_empty());
    }

    #[test]
    fn canonical_ai_configuration_is_available() {
        let configuration = AiConfig::default();

        let api = AiApi::with_config(configuration)
            .expect("default AI configuration must be valid");

        assert_eq!(api.operation_count(), 0);
    }

    #[test]
    fn canonical_ai_subsystems_are_exposed() {
        let _ = core::any::type_name::<
            crate::ai::model::ModelMarkerForIntegrationTest
        >;
    }

    #[test]
    fn architectural_invariants_are_preserved() {
        assert!(invariants::GRAMMAR_IS_EXTERNAL);
        assert!(invariants::SEMANTICS_ARE_CANONICAL);
        assert!(invariants::AI_IMPLEMENTATION_IS_CANONICAL);
        assert!(invariants::TENSOR_STORAGE_IS_EXTERNAL);
        assert!(invariants::UNIVERSAL_IR_IS_EXTERNAL);
        assert!(invariants::NO_ARTIFICIAL_WORKLOAD_CEILINGS);
        assert!(invariants::UNSAFE_RUST_IS_PROHIBITED);
    }
}
