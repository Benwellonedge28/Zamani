
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Canonical Pipeline Stages.
//!
//! Path:
//!     src/compiler/pipeline/stages.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Defines canonical identifiers and dependency contracts for standard
//! Zamani compiler pipeline stages.
//!
//! Individual stage implementations implement the Stage trait defined in:
//!
//!     compiler::pipeline::stage
//!
//! This module provides the stable names and construction contracts used
//! by pipeline registration, ordering, execution, and orchestration.
//!
//! It does not implement the stages themselves.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Standard compiler-stage identifiers.
//! - Standard stage descriptor construction.
//! - Standard stage dependency declarations.
//! - Stable identifiers for domain-extension integration points.
//! - The standard stage API version.
//!
//! This module does NOT own:
//!
//! - Stage implementations.
//! - Stage registration.
//! - Stage ordering algorithms.
//! - Pipeline orchestration.
//! - Stage execution.
//! - Stage recovery.
//! - Compilation lifecycle.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Effect semantics.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - AST definitions.
//! - ZUIR.
//! - ZCIR.
//! - Quantum IR.
//! - HDLIR.
//! - AI computation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Backend implementation.
//!
//! ============================================================================
//! REPOSITORY OWNERSHIP
//! ============================================================================
//!
//! The canonical authorities remain:
//!
//! Source grammar:
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!
//! Semantic model:
//!     grammar/specification/semantic-model.md
//!
//! Frontend:
//!     src/frontend/
//!
//! Semantic analysis:
//!     Existing canonical semantic subsystem.
//!
//! Universal IR:
//!     src/zuir/
//!
//! Classical IR:
//!     src/classical/
//!
//! Quantum IR:
//!     src/quantum/ir/
//!
//! Hardware IR:
//!     src/hdl/hdlir/
//!
//! AI:
//!     src/ai/
//!
//! Hybrid:
//!     src/hybrid/
//!
//! Distributed:
//!     src/distributed/
//!
//! Backend:
//!     src/backend/
//!
//! Runtime:
//!     src/runtime/
//!
//! Compiler orchestration:
//!     src/compiler/
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! This module depends on:
//!
//!     compiler::pipeline::stage
//!         StageId
//!         StageDescriptor
//!         StageDescriptorError
//!         StageDeterminism
//!         StageConcurrency
//!
//!     compiler::context::compilation_state
//!         CompilationPhase
//!
//!     compiler::configuration::budget
//!         BudgetDimension
//!
//! Future consumers:
//!
//!     pipeline::ordering
//!     pipeline::execution
//!     pipeline::orchestration
//!     pipeline::mod
//!
//! No dependency on future pipeline modules is required.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Stage identifiers are logical compiler identities.
//!
//! They are not physical hardware identifiers.
//!
//! This module imposes no universal maximum on:
//!
//! - CPUs.
//! - Cores.
//! - Threads.
//! - Workers.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Qubits.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Modules.
//! - Functions.
//! - Operations.
//! - Compilation units.
//! - Compiler stages.
//! - Domain extensions.
//!
//! Operational constraints are managed by the existing compilation
//! configuration and budget authorities.
//!
//! No finite implementation can promise infinite physical resources.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! The grammar composition establishes source-language constructs.
//!
//! In particular:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/compile/compile.g4
//!
//! Compilation declarations are source-language intent.
//!
//! They must be interpreted by frontend and semantic analysis before
//! influencing actual compilation policy.
//!
//! A source-level COMPILE declaration is not a StageId.
//!
//! A source-level TARGET declaration is not a backend implementation.
//!
//! A source-level RESOURCE requirement is not a physical allocation.
//!
//! A source-level quantum declaration does not automatically require
//! every quantum compiler stage to execute.
//!
//! Domain-specific stages are activated by validated compilation input,
//! registered implementations, and applicable compilation policy.
//!
//! ============================================================================

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::compiler::configuration::budget::BudgetDimension;

use crate::compiler::context::compilation_state::CompilationPhase;

use crate::compiler::pipeline::stage::{
    StageConcurrency,
    StageDescriptor,
    StageDescriptorError,
    StageDeterminism,
    StageId,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for canonical compiler-stage definitions.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic-model version.
/// - AST version.
/// - ZUIR version.
/// - Quantum IR version.
/// - Backend ABI version.
pub const PIPELINE_STAGES_API_VERSION: u16 = 1;

// ============================================================================
// STANDARD STAGE IDENTIFIERS
// ============================================================================

/// Stable identifiers for the standard compiler pipeline.
///
/// These identifiers describe compiler responsibilities.
///
/// They do not identify a particular implementation.
///
/// Custom extensions must use their own namespaced identifiers.
///
/// Identifiers are strings rather than closed hardware-specific enums.
///
/// This allows future computing domains to introduce new stages without
/// changing this module.
pub mod ids {
    // ------------------------------------------------------------------------
    // Initialization
    // ------------------------------------------------------------------------

    /// Compiler initialization and compilation preparation.
    pub const INITIALIZATION: &str =
        "zamani.compiler.initialization";

    // ------------------------------------------------------------------------
    // Frontend
    // ------------------------------------------------------------------------

    /// Lexical analysis and parsing.
    pub const FRONTEND_PARSE: &str =
        "zamani.compiler.frontend.parse";

    /// Structural validation of frontend output.
    pub const STRUCTURAL_VALIDATION: &str =
        "zamani.compiler.frontend.structural_validation";

    // ------------------------------------------------------------------------
    // Semantic analysis
    // ------------------------------------------------------------------------

    /// Name and symbol resolution.
    pub const NAME_RESOLUTION: &str =
        "zamani.compiler.semantic.name_resolution";

    /// Canonical semantic analysis integration.
    pub const SEMANTIC_ANALYSIS: &str =
        "zamani.compiler.semantic.analysis";

    /// Ownership and borrow-analysis integration.
    pub const OWNERSHIP_ANALYSIS: &str =
        "zamani.compiler.semantic.ownership";

    /// Effect and dependency analysis integration.
    pub const EFFECT_DEPENDENCY_ANALYSIS: &str =
        "zamani.compiler.semantic.effects_dependencies";

    // ------------------------------------------------------------------------
    // Intermediate representation
    // ------------------------------------------------------------------------

    /// Construction of canonical intermediate representations.
    pub const IR_CONSTRUCTION: &str =
        "zamani.compiler.ir.construction";

    /// Verification of canonical intermediate representations.
    pub const IR_VERIFICATION: &str =
        "zamani.compiler.ir.verification";

    // ------------------------------------------------------------------------
    // Optimization
    // ------------------------------------------------------------------------

    /// Optimization of canonical representations.
    pub const OPTIMIZATION: &str =
        "zamani.compiler.optimization";

    // ------------------------------------------------------------------------
    // Capability and feasibility analysis
    // ------------------------------------------------------------------------

    /// Capability compatibility analysis.
    pub const CAPABILITY_ANALYSIS: &str =
        "zamani.compiler.analysis.capabilities";

    /// Compilation and realization feasibility analysis.
    pub const FEASIBILITY_ANALYSIS: &str =
        "zamani.compiler.analysis.feasibility";

    // ------------------------------------------------------------------------
    // Planning
    // ------------------------------------------------------------------------

    /// Heterogeneous compilation planning.
    pub const HETEROGENEOUS_PLANNING: &str =
        "zamani.compiler.planning.heterogeneous";

    // ------------------------------------------------------------------------
    // Lowering
    // ------------------------------------------------------------------------

    /// Target-independent and target-specific lowering.
    pub const LOWERING: &str =
        "zamani.compiler.lowering";

    // ------------------------------------------------------------------------
    // Linking
    // ------------------------------------------------------------------------

    /// Module composition and linking.
    pub const LINKING: &str =
        "zamani.compiler.linking";

    // ------------------------------------------------------------------------
    // Backend
    // ------------------------------------------------------------------------

    /// Target-specific code generation.
    pub const CODE_GENERATION: &str =
        "zamani.compiler.backend.code_generation";

    // ------------------------------------------------------------------------
    // Artifacts
    // ------------------------------------------------------------------------

    /// Artifact integrity and validity verification.
    pub const ARTIFACT_VALIDATION: &str =
        "zamani.compiler.artifacts.validation";

    // ------------------------------------------------------------------------
    // Finalization
    // ------------------------------------------------------------------------

    /// Final compilation-result preparation.
    pub const FINALIZATION: &str =
        "zamani.compiler.finalization";
}

// ============================================================================
// DOMAIN EXTENSION IDENTIFIERS
// ============================================================================

/// Namespaced identifiers for common computing-domain integration points.
///
/// These are extension identifiers, not a closed inventory.
///
/// Additional domains must be able to introduce their own identifiers
/// without modifying this module.
///
/// The presence of an identifier does not imply that an implementation
/// exists or that the stage must execute.
pub mod domains {

    /// Classical computation integration.
    pub const CLASSICAL: &str =
        "zamani.domain.classical";

    /// Quantum computation integration.
    pub const QUANTUM: &str =
        "zamani.domain.quantum";

    /// Artificial intelligence integration.
    pub const AI: &str =
        "zamani.domain.ai";

    /// Tensor computation integration.
    pub const TENSOR: &str =
        "zamani.domain.tensor";

    /// HDL and hardware-description integration.
    pub const HDL: &str =
        "zamani.domain.hdl";

    /// Hardware acceleration integration.
    pub const HARDWARE: &str =
        "zamani.domain.hardware";

    /// Hybrid computation integration.
    pub const HYBRID: &str =
        "zamani.domain.hybrid";

    /// Distributed computation integration.
    pub const DISTRIBUTED: &str =
        "zamani.domain.distributed";

    /// Networking integration.
    pub const NETWORKING: &str =
        "zamani.domain.networking";

    /// Cryptographic computation integration.
    pub const CRYPTOGRAPHY: &str =
        "zamani.domain.cryptography";

    /// Neuromorphic computation integration.
    pub const NEUROMORPHIC: &str =
        "zamani.domain.neuromorphic";

    /// Photonic computation integration.
    pub const PHOTONIC: &str =
        "zamani.domain.photonic";
}

// ============================================================================
// DOMAIN STAGE IDENTIFIERS
// ============================================================================

/// Canonical extension points for domain-specific compilation stages.
///
/// Each domain remains responsible for its own implementation.
///
/// These identifiers allow independently developed domain subsystems
/// to integrate into the universal compiler pipeline.
///
/// A domain may introduce additional identifiers using StageId::new().
pub mod domain_stages {

    /// Classical representation integration.
    pub const CLASSICAL_IR: &str =
        "zamani.compiler.domain.classical.ir";

    /// Quantum representation integration.
    pub const QUANTUM_IR: &str =
        "zamani.compiler.domain.quantum.ir";

    /// AI representation integration.
    pub const AI_IR: &str =
        "zamani.compiler.domain.ai.ir";

    /// Tensor representation integration.
    pub const TENSOR_IR: &str =
        "zamani.compiler.domain.tensor.ir";

    /// HDL representation integration.
    pub const HDL_IR: &str =
        "zamani.compiler.domain.hdl.ir";

    /// Hybrid dependency integration.
    pub const HYBRID_ANALYSIS: &str =
        "zamani.compiler.domain.hybrid.analysis";

    /// Distributed computation integration.
    pub const DISTRIBUTED_ANALYSIS: &str =
        "zamani.compiler.domain.distributed.analysis";

    /// Networking integration.
    pub const NETWORKING_ANALYSIS: &str =
        "zamani.compiler.domain.networking.analysis";

    /// Cryptographic computation integration.
    pub const CRYPTOGRAPHY_ANALYSIS: &str =
        "zamani.compiler.domain.cryptography.analysis";
}

// ============================================================================
// STANDARD STAGE METADATA
// ============================================================================

/// Metadata describing the role of a standard pipeline stage.
///
/// This type is organizational.
///
/// It does not replace StageDescriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StandardStageRole {

    /// Compiler initialization.
    Initialization,

    /// Frontend processing.
    Frontend,

    /// Semantic analysis.
    Semantic,

    /// IR construction.
    IntermediateRepresentation,

    /// IR verification.
    Verification,

    /// Optimization.
    Optimization,

    /// Capability analysis.
    CapabilityAnalysis,

    /// Feasibility analysis.
    FeasibilityAnalysis,

    /// Compilation planning.
    Planning,

    /// Lowering.
    Lowering,

    /// Linking.
    Linking,

    /// Code generation.
    CodeGeneration,

    /// Artifact validation.
    ArtifactValidation,

    /// Finalization.
    Finalization,
}

/// Describes whether a stage is part of the normal compiler path
/// or is activated conditionally.
///
/// Domain-specific implementations should generally be conditional.
///
/// A conditional stage is activated only when the validated compilation
/// input and registered implementation make it applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageActivation {

    /// Part of the standard compilation contract.
    Required,

    /// Executed only when its prerequisites and applicability conditions
    /// are satisfied.
    Conditional,

    /// Activated by an explicit compilation or deployment policy.
    PolicyControlled,
}

// ============================================================================
// STANDARD STAGE DEFINITION
// ============================================================================

/// A standard stage definition.
///
/// The descriptor contains the executable-stage contract.
///
/// The role and activation classification provide additional
/// pipeline-planning metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardStageDefinition {

    descriptor: StageDescriptor,

    role: StandardStageRole,

    activation: StageActivation,
}

impl StandardStageDefinition {

    /// Creates a standard stage definition.
    #[must_use]
    pub fn new(
        descriptor: StageDescriptor,
        role: StandardStageRole,
        activation: StageActivation,
    ) -> Self {
        Self {
            descriptor,
            role,
            activation,
        }
    }

    /// Returns the stage descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &StageDescriptor {
        &self.descriptor
    }

    /// Returns the stage role.
    #[must_use]
    pub const fn role(&self) -> StandardStageRole {
        self.role
    }

    /// Returns the activation classification.
    #[must_use]
    pub const fn activation(&self) -> StageActivation {
        self.activation
    }

    /// Consumes this definition and returns its descriptor.
    #[must_use]
    pub fn into_descriptor(self) -> StageDescriptor {
        self.descriptor
    }
}

// ============================================================================
// STANDARD STAGE CONSTRUCTION ERRORS
// ============================================================================

/// Errors encountered while constructing standard stage definitions.
#[derive(Debug)]
#[non_exhaustive]
pub enum StandardStageError {

    /// Invalid stage identifier.
    InvalidIdentifier {
        identifier: String,
        message: String,
    },

    /// Invalid stage descriptor.
    InvalidDescriptor {
        identifier: String,
        source: StageDescriptorError,
    },

    /// A standard stage identifier was declared more than once.
    DuplicateIdentifier {
        identifier: String,
    },

    /// A standard stage depends on an identifier that is absent
    /// from the standard definition set.
    ///
    /// This check is performed only for dependencies declared as
    /// standard dependencies.
    MissingStandardDependency {
        stage: String,
        dependency: String,
    },
}

impl fmt::Display for StandardStageError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidIdentifier {
                identifier,
                message,
            } => write!(
                formatter,
                "invalid standard stage identifier `{identifier}`: {message}"
            ),

            Self::InvalidDescriptor {
                identifier,
                source,
            } => write!(
                formatter,
                "invalid descriptor for stage `{identifier}`: {source}"
            ),

            Self::DuplicateIdentifier {
                identifier,
            } => write!(
                formatter,
                "duplicate standard stage identifier `{identifier}`"
            ),

            Self::MissingStandardDependency {
                stage,
                dependency,
            } => write!(
                formatter,
                "standard stage `{stage}` references missing dependency `{dependency}`"
            ),
        }
    }
}

impl Error for StandardStageError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {

        match self {

            Self::InvalidDescriptor {
                source,
                ..
            } => Some(source),

            _ => None,
        }
    }
}

// ============================================================================
// STANDARD STAGE BUILDER
// ============================================================================

fn create_stage(
    identifier: &str,
    name: &str,
    description: &str,
    phase: CompilationPhase,
    dependencies: &[&str],
    role: StandardStageRole,
    activation: StageActivation,
    determinism: StageDeterminism,
    concurrency: StageConcurrency,
    budget_dimension: Option<(&str, &str)>,
) -> Result<StandardStageDefinition, StandardStageError> {

    let id = StageId::new(identifier)
        .map_err(|error| {
            StandardStageError::InvalidIdentifier {
                identifier: identifier.to_owned(),
                message: error.to_string(),
            }
        })?;

    let mut descriptor = StageDescriptor::new(
        id,
        name,
        "1",
        description,
    )
    .map_err(|source| {
        StandardStageError::InvalidDescriptor {
            identifier: identifier.to_owned(),
            source,
        }
    })?;

    descriptor = descriptor
        .with_phase(phase)
        .with_determinism(determinism)
        .with_concurrency(concurrency);

    for dependency in dependencies {

        let dependency_id = StageId::new(*dependency)
            .map_err(|error| {
                StandardStageError::InvalidIdentifier {
                    identifier: (*dependency).to_owned(),
                    message: error.to_string(),
                }
            })?;

        descriptor = descriptor
            .with_dependency(dependency_id)
            .map_err(|source| {
                StandardStageError::InvalidDescriptor {
                    identifier: identifier.to_owned(),
                    source,
                }
            })?;
    }

    if let Some((name, unit)) = budget_dimension {

        let dimension = BudgetDimension::new(name, unit)
            .map_err(|error| {
                StandardStageError::InvalidIdentifier {
                    identifier: identifier.to_owned(),
                    message: error.to_string(),
                }
            })?;

        descriptor = descriptor.with_budget_dimension(dimension);
    }

    Ok(StandardStageDefinition::new(
        descriptor,
        role,
        activation,
    ))
}

// ============================================================================
// STANDARD STAGE DEFINITIONS
// ============================================================================

/// Constructs the canonical standard compiler-stage definitions.
///
/// The returned collection describes the standard compilation path.
///
/// It does not instantiate Stage implementations.
///
/// It does not register stages.
///
/// It does not execute stages.
///
/// It does not perform dependency ordering.
///
/// All stage implementations are supplied by their authoritative
/// compiler subsystems.
///
/// The definitions are constructed in dependency order for readability,
/// but actual scheduling is owned by pipeline::ordering.
///
/// Domain-specific stages are registered separately and are not forced
/// into every compilation.
///
/// # Determinism
///
/// Identical calls return identical stage identifiers, metadata,
/// activation classifications, and dependency declarations.
///
/// # Scalability
///
/// The returned collection has no language-level or hardware-level
/// stage-count restriction.
///
/// Extensions can register additional StageDescriptor values.
pub fn standard_stage_definitions(
) -> Result<Vec<StandardStageDefinition>, StandardStageError> {

    use ids::*;

    let mut definitions = Vec::new();

    // ------------------------------------------------------------------------
    // 1. Initialization
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        INITIALIZATION,
        "Compiler initialization",
        "Prepare shared compilation state, configuration and services.",
        CompilationPhase::Initialization,
        &[],
        StandardStageRole::Initialization,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 2. Frontend
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        FRONTEND_PARSE,
        "Frontend parsing",
        "Consume the canonical lexer and parser output and construct frontend representations.",
        CompilationPhase::Parsing,
        &[INITIALIZATION],
        StandardStageRole::Frontend,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(("source_bytes_processed", "bytes")),
    )?);

    definitions.push(create_stage(
        STRUCTURAL_VALIDATION,
        "Structural validation",
        "Validate the frontend structure without redefining grammar or semantic authority.",
        CompilationPhase::Parsing,
        &[FRONTEND_PARSE],
        StandardStageRole::Frontend,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 3. Name resolution
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        NAME_RESOLUTION,
        "Name resolution",
        "Resolve names and symbols using the canonical frontend and semantic contracts.",
        CompilationPhase::NameAnalysis,
        &[STRUCTURAL_VALIDATION],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 4. Semantic analysis
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        SEMANTIC_ANALYSIS,
        "Semantic analysis",
        "Integrate authoritative semantic analysis, types, effects, capabilities and program meaning.",
        CompilationPhase::SemanticAnalysis,
        &[NAME_RESOLUTION],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        OWNERSHIP_ANALYSIS,
        "Ownership analysis",
        "Validate ownership and borrowing using the authoritative language ownership model.",
        CompilationPhase::OwnershipAnalysis,
        &[SEMANTIC_ANALYSIS],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        EFFECT_DEPENDENCY_ANALYSIS,
        "Effect and dependency analysis",
        "Analyze semantic effects and program dependencies without duplicating their domain authorities.",
        CompilationPhase::DependencyAnalysis,
        &[OWNERSHIP_ANALYSIS],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 5. Canonical IR construction
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        IR_CONSTRUCTION,
        "Canonical IR construction",
        "Construct and integrate canonical intermediate representations owned by their domain subsystems.",
        CompilationPhase::IrGeneration,
        &[EFFECT_DEPENDENCY_ANALYSIS],
        StandardStageRole::IntermediateRepresentation,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(("ir_entities_processed", "entities")),
    )?);

    // ------------------------------------------------------------------------
    // 6. IR verification
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        IR_VERIFICATION,
        "Canonical IR verification",
        "Verify canonical IR invariants and domain-specific representation contracts.",
        CompilationPhase::IrVerification,
        &[IR_CONSTRUCTION],
        StandardStageRole::Verification,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::Concurrent,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 7. Optimization
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        OPTIMIZATION,
        "Program optimization",
        "Apply registered, legal and semantics-preserving optimization transformations.",
        CompilationPhase::Optimization,
        &[IR_VERIFICATION],
        StandardStageRole::Optimization,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        Some(("optimization_work", "operations")),
    )?);

    // ------------------------------------------------------------------------
    // 8. Capability analysis
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        CAPABILITY_ANALYSIS,
        "Capability analysis",
        "Match validated program requirements against available abstract target capabilities.",
        CompilationPhase::CapabilityAnalysis,
        &[OPTIMIZATION],
        StandardStageRole::CapabilityAnalysis,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::Concurrent,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 9. Feasibility analysis
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        FEASIBILITY_ANALYSIS,
        "Compilation feasibility analysis",
        "Determine whether candidate realizations satisfy requirements, constraints and available resources.",
        CompilationPhase::FeasibilityAnalysis,
        &[CAPABILITY_ANALYSIS],
        StandardStageRole::FeasibilityAnalysis,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::Concurrent,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 10. Heterogeneous planning
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        HETEROGENEOUS_PLANNING,
        "Heterogeneous computation planning",
        "Construct abstract execution and partition plans for compatible computational domains.",
        CompilationPhase::Planning,
        &[FEASIBILITY_ANALYSIS],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 11. Lowering
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        LOWERING,
        "Representation lowering",
        "Perform legal IR conversion, decomposition and target-specific lowering.",
        CompilationPhase::Lowering,
        &[FEASIBILITY_ANALYSIS],
        StandardStageRole::Lowering,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        Some(("lowering_work", "operations")),
    )?);

    // ------------------------------------------------------------------------
    // 12. Linking
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        LINKING,
        "Compilation linking",
        "Resolve symbols and compose compatible compilation artifacts and modules.",
        CompilationPhase::Linking,
        &[LOWERING],
        StandardStageRole::Linking,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 13. Code generation
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        CODE_GENERATION,
        "Target code generation",
        "Invoke registered backend implementations to generate genuine target artifacts.",
        CompilationPhase::CodeGeneration,
        &[LINKING],
        StandardStageRole::CodeGeneration,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        Some(("generated_artifact_bytes", "bytes")),
    )?);

    // ------------------------------------------------------------------------
    // 14. Artifact validation
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        ARTIFACT_VALIDATION,
        "Artifact validation",
        "Validate artifact integrity, compatibility, provenance and declared portability.",
        CompilationPhase::ArtifactValidation,
        &[CODE_GENERATION],
        StandardStageRole::ArtifactValidation,
        StageActivation::Conditional,
        StageDeterminism::Deterministic,
        StageConcurrency::Concurrent,
        None,
    )?);

    // ------------------------------------------------------------------------
    // 15. Finalization
    // ------------------------------------------------------------------------

    definitions.push(create_stage(
        FINALIZATION,
        "Compilation finalization",
        "Prepare the final compilation result and its verified output metadata.",
        CompilationPhase::Finalization,
        &[ARTIFACT_VALIDATION],
        StandardStageRole::Finalization,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    validate_standard_definitions(&definitions)?;

    Ok(definitions)
}

// ============================================================================
// STANDARD DEFINITION VALIDATION
// ============================================================================

/// Validates uniqueness and standard dependency integrity.
///
/// This function does not perform topological sorting.
///
/// Topological ordering and cycle detection belong to pipeline::ordering.
pub fn validate_standard_definitions(
    definitions: &[StandardStageDefinition],
) -> Result<(), StandardStageError> {

    let mut identifiers = BTreeSet::new();

    for definition in definitions {

        let identifier = definition
            .descriptor()
            .id()
            .as_str();

        if !identifiers.insert(identifier.to_owned()) {

            return Err(
                StandardStageError::DuplicateIdentifier {
                    identifier: identifier.to_owned(),
                }
            );
        }
    }

    for definition in definitions {

        let stage = definition
            .descriptor()
            .id()
            .as_str();

        for dependency in definition
            .descriptor()
            .dependencies()
        {

            if !identifiers.contains(dependency.as_str()) {

                return Err(
                    StandardStageError::MissingStandardDependency {
                        stage: stage.to_owned(),
                        dependency: dependency.as_str().to_owned(),
                    }
                );
            }
        }
    }

    Ok(())
}

// ============================================================================
// DOMAIN EXTENSION DESCRIPTOR CONSTRUCTION
// ============================================================================

/// Constructs a descriptor for an independently-owned domain stage.
///
/// Domain implementations use this function to construct stable,
/// namespaced integration descriptors.
///
/// The caller supplies the dependencies because domain dependencies
/// are determined by the actual validated program and domain contracts.
///
/// The function does not require modification when a new computing
/// domain is introduced.
///
/// # Example
///
/// A quantum subsystem may construct a descriptor using:
///
///     domains::QUANTUM
///
/// or:
///
///     domain_stages::QUANTUM_IR
///
/// A future photonic subsystem may use:
///
///     zamani.compiler.domain.photonic.lowering
///
/// No central enum modification is required.
pub fn domain_stage_descriptor(
    identifier: &str,
    name: &str,
    version: &str,
    description: &str,
    phase: CompilationPhase,
    dependencies: impl IntoIterator<Item = StageId>,
    determinism: StageDeterminism,
    concurrency: StageConcurrency,
    budget_dimension: Option<BudgetDimension>,
) -> Result<StageDescriptor, StandardStageError> {

    let id = StageId::new(identifier)
        .map_err(|error| {
            StandardStageError::InvalidIdentifier {
                identifier: identifier.to_owned(),
                message: error.to_string(),
            }
        })?;

    let mut descriptor = StageDescriptor::new(
        id,
        name,
        version,
        description,
    )
    .map_err(|source| {
        StandardStageError::InvalidDescriptor {
            identifier: identifier.to_owned(),
            source,
        }
    })?;

    descriptor = descriptor
        .with_phase(phase)
        .with_determinism(determinism)
        .with_concurrency(concurrency);

    for dependency in dependencies {

        descriptor = descriptor
            .with_dependency(dependency)
            .map_err(|source| {
                StandardStageError::InvalidDescriptor {
                    identifier: identifier.to_owned(),
                    source,
                }
            })?;
    }

    if let Some(dimension) = budget_dimension {
        descriptor = descriptor.with_budget_dimension(dimension);
    }

    Ok(descriptor)
}

// ============================================================================
// STAGE IDENTIFIER CONVENIENCE
// ============================================================================

/// Constructs a validated StageId from a standard identifier.
pub fn standard_stage_id(
    identifier: &str,
) -> Result<StageId, StandardStageError> {

    StageId::new(identifier)
        .map_err(|error| {
            StandardStageError::InvalidIdentifier {
                identifier: identifier.to_owned(),
                message: error.to_string(),
            }
        })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    // ------------------------------------------------------------------------
    // Standard definition construction
    // ------------------------------------------------------------------------

    #[test]
    fn standard_definitions_are_constructible() {

        let definitions = standard_stage_definitions()
            .expect("standard definitions must be valid");

        assert!(!definitions.is_empty());
    }

    #[test]
    fn standard_identifiers_are_unique() {

        let definitions = standard_stage_definitions()
            .expect("standard definitions must be valid");

        validate_standard_definitions(&definitions)
            .expect("identifiers must be unique");
    }

    #[test]
    fn every_standard_dependency_exists() {

        let definitions = standard_stage_definitions()
            .expect("standard definitions must be valid");

        let identifiers: BTreeSet<String> = definitions
            .iter()
            .map(|definition| {
                definition.descriptor().id().as_str().to_owned()
            })
            .collect();

        for definition in &definitions {

            for dependency in definition
                .descriptor()
                .dependencies()
            {

                assert!(
                    identifiers.contains(dependency.as_str()),
                    "missing dependency {} for {}",
                    dependency,
                    definition.descriptor().id()
                );
            }
        }
    }

    #[test]
    fn standard_definitions_are_deterministic() {

        let first = standard_stage_definitions()
            .expect("first construction");

        let second = standard_stage_definitions()
            .expect("second construction");

        assert_eq!(first, second);
    }

    // ------------------------------------------------------------------------
    // Stage identity
    // ------------------------------------------------------------------------

    #[test]
    fn standard_stage_ids_are_stable() {

        let id = standard_stage_id(ids::SEMANTIC_ANALYSIS)
            .expect("valid standard identifier");

        assert_eq!(
            id.as_str(),
            "zamani.compiler.semantic.analysis"
        );
    }

    #[test]
    fn custom_domain_identifiers_are_supported() {

        let id = standard_stage_id(
            "zamani.extension.future_computing.specialization"
        )
        .expect("valid extension identifier");

        assert_eq!(
            id.as_str(),
            "zamani.extension.future_computing.specialization"
        );
    }

    #[test]
    fn identifiers_are_not_hardware_specific() {

        let id = standard_stage_id(ids::LOWERING)
            .expect("valid identifier");

        assert!(
            !id.as_str().contains("cpu")
        );

        assert!(
            !id.as_str().contains("gpu")
        );
    }

    // ------------------------------------------------------------------------
    // Domain integration
    // ------------------------------------------------------------------------

    #[test]
    fn domain_stage_descriptor_supports_quantum_integration() {

        let dependency = standard_stage_id(ids::IR_VERIFICATION)
            .expect("valid dependency");

        let descriptor = domain_stage_descriptor(
            domain_stages::QUANTUM_IR,
            "Quantum IR integration",
            "1",
            "Integrates canonical quantum IR.",
            CompilationPhase::IrGeneration,
            [dependency],
            StageDeterminism::Deterministic,
            StageConcurrency::InstanceExclusive,
            None,
        )
        .expect("valid quantum descriptor");

        assert_eq!(
            descriptor.id().as_str(),
            domain_stages::QUANTUM_IR
        );
    }

    #[test]
    fn domain_stage_descriptor_supports_future_domains() {

        let dependency = standard_stage_id(ids::IR_VERIFICATION)
            .expect("valid dependency");

        let descriptor = domain_stage_descriptor(
            "zamani.compiler.domain.photonic.lowering",
            "Photonic lowering",
            "1",
            "Integrates future photonic computation.",
            CompilationPhase::Lowering,
            [dependency],
            StageDeterminism::InputDependent,
            StageConcurrency::InstanceExclusive,
            None,
        )
        .expect("future domain descriptor");

        assert_eq!(
            descriptor.id().as_str(),
            "zamani.compiler.domain.photonic.lowering"
        );
    }

    // ------------------------------------------------------------------------
    // Activation
    // ------------------------------------------------------------------------

    #[test]
    fn heterogeneous_planning_is_conditional() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        let definition = definitions
            .iter()
            .find(|definition| {
                definition.descriptor().id().as_str()
                    == ids::HETEROGENEOUS_PLANNING
            })
            .expect("heterogeneous planning stage");

        assert_eq!(
            definition.activation(),
            StageActivation::Conditional
        );
    }

    #[test]
    fn backend_codegen_is_conditional() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        let definition = definitions
            .iter()
            .find(|definition| {
                definition.descriptor().id().as_str()
                    == ids::CODE_GENERATION
            })
            .expect("code generation stage");

        assert_eq!(
            definition.activation(),
            StageActivation::Conditional
        );
    }

    // ------------------------------------------------------------------------
    // No artificial string limits
    // ------------------------------------------------------------------------

    #[test]
    fn long_extension_identifiers_are_supported() {

        let identifier = format!(
            "zamani.extension.{}",
            "future".repeat(1024)
        );

        let id = standard_stage_id(&identifier)
            .expect("valid dynamically sized identifier");

        assert_eq!(id.as_str(), identifier);
    }

    // ------------------------------------------------------------------------
    // Validation failures
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_standard_definitions_are_rejected() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        let mut duplicated = definitions.clone();

        duplicated.push(
            definitions[0].clone()
        );

        assert!(matches!(
            validate_standard_definitions(&duplicated),
            Err(StandardStageError::DuplicateIdentifier { .. })
        ));
    }

    #[test]
    fn invalid_stage_identifiers_are_rejected() {

        assert!(
            standard_stage_id("")
                .is_err()
        );

        assert!(
            standard_stage_id("invalid\nstage")
                .is_err()
        );
    }

    #[test]
    fn invalid_domain_descriptors_are_rejected() {

        let result = domain_stage_descriptor(
            "",
            "Invalid stage",
            "1",
            "Invalid identifier",
            CompilationPhase::Lowering,
            [],
            StageDeterminism::Deterministic,
            StageConcurrency::InstanceExclusive,
            None,
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Semantic ownership
    // ------------------------------------------------------------------------

    #[test]
    fn quantum_is_not_a_required_global_stage() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        assert!(
            !definitions.iter().any(|definition| {
                definition.descriptor().id().as_str()
                    == domain_stages::QUANTUM_IR
            })
        );
    }

    #[test]
    fn ai_is_not_a_required_global_stage() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        assert!(
            !definitions.iter().any(|definition| {
                definition.descriptor().id().as_str()
                    == domain_stages::AI_IR
            })
        );
    }

    #[test]
    fn standard_definitions_do_not_own_domain_ir() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        for definition in &definitions {

            assert!(
                !definition.descriptor().id().as_str()
                    .contains("quantum.ir")
            );

            assert!(
                !definition.descriptor().id().as_str()
                    .contains("ai.ir")
            );

            assert!(
                !definition.descriptor().id().as_str()
                    .contains("hdl.ir")
            );
        }
    }

    // ------------------------------------------------------------------------
    // Phase integration
    // ------------------------------------------------------------------------

    #[test]
    fn stages_have_compilation_phases() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        for definition in &definitions {

            assert!(
                definition.descriptor().phase().is_some(),
                "stage {} must declare its phase",
                definition.descriptor().id()
            );
        }
    }

    #[test]
    fn initialization_has_no_dependencies() {

        let definitions = standard_stage_definitions()
            .expect("valid definitions");

        let initialization = definitions
            .iter()
            .find(|definition| {
                definition.descriptor().id().as_str()
                    == ids::INITIALIZATION
            })
            .expect("initialization stage");

        assert!(
            initialization.descriptor().dependencies().is_empty()
        );
    }
}
