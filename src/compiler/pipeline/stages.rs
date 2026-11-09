
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Canonical Pipeline Stages.
//!
//! Path: src/compiler/pipeline/stages.rs
//!
//! Rust: Edition 2021, Rust 1.97+
//!
//! # Purpose
//!
//! Defines stable, extensible identities and dependency contracts for
//! the standard Zamani compiler pipeline.
//!
//! This module describes compiler responsibilities. It does not
//! implement, register, order, execute or orchestrate stages.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Standard stage identifiers.
//! - Standard domain integration identifiers.
//! - Standard stage metadata.
//! - Stage activation classification.
//! - Standard stage descriptor construction.
//! - Standard dependency declarations.
//! - Standard-definition validation.
//! - Standard-stage activation queries.
//!
//! This module does not own:
//!
//! - Stage implementations.
//! - Stage registration.
//! - Extension discovery.
//! - Dependency ordering algorithms.
//! - Pipeline execution.
//! - Pipeline orchestration.
//! - Compilation lifecycle.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership semantics.
//! - Effect semantics.
//! - Capability semantics.
//! - Canonical IR.
//! - IR verification implementation.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Backend implementation.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Source grammar:
//!     grammar/Zamani.g4
//!
//! Parser composition:
//!     grammar/antlr/ZamaniParser.g4
//!
//! Compilation grammar:
//!     grammar/compile/compile.g4
//!
//! Compilation semantics:
//!     grammar/specification/compilation-model.md
//!
//! Semantic authority:
//!     src/frontend/semantic/
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
//! HDL:
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
//! # Dependency direction
//!
//! configuration/budget
//!         |
//! context/compilation_state
//!         |
//! pipeline/stage
//!         |
//! pipeline/stages
//!         |
//! pipeline/ordering
//!         |
//! pipeline/execution
//!         |
//! pipeline/orchestration
//!
//! # POCO-REAF
//!
//! Stage identifiers are logical identities.
//!
//! They are not physical machine identifiers.
//!
//! No universal limits are imposed on:
//!
//! - stages;
//! - dependencies;
//! - CPUs;
//! - cores;
//! - threads;
//! - GPUs;
//! - FPGAs;
//! - QPUs;
//! - qubits;
//! - accelerators;
//! - nodes;
//! - devices;
//! - memory;
//! - storage;
//! - tensor rank;
//! - tensor dimensions;
//! - modules;
//! - functions;
//! - operations;
//! - compilation units;
//! - domain extensions.
//!
//! Operational budgets are owned by compiler configuration.
//!
//! Actual physical capacity is determined by authoritative providers.
//!
//! No implementation can guarantee infinite physical resources.
//!
//! # Grammar boundary
//!
//! A source-level COMPILE declaration is not a StageId.
//!
//! A source-level TARGET declaration is not a backend implementation.
//!
//! A source-level RESOURCE requirement is not a physical allocation.
//!
//! Source-level intent must be parsed and semantically validated before
//! influencing compiler-stage activation.
//!
//! Adding a new grammar production must not automatically add a
//! compiler-stage implementation.

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

/// Public API version of standard pipeline-stage definitions.
pub const PIPELINE_STAGES_API_VERSION: u16 = 2;

// ============================================================================
// STANDARD STAGE IDENTIFIERS
// ============================================================================

/// Stable identifiers for standard compiler responsibilities.
///
/// These are dynamically sized string identifiers rather than closed
/// hardware-specific enumerations.
///
/// Their values are compatibility-sensitive.
///
/// Existing identifiers must not be changed without an explicit
/// compatibility migration.
pub mod ids {

    /// Compilation initialization.
    pub const INITIALIZATION: &str =
        "zamani.compiler.initialization";

    // ------------------------------------------------------------------------
    // Frontend
    // ------------------------------------------------------------------------

    /// Frontend lexing and parsing integration.
    pub const FRONTEND_PARSE: &str =
        "zamani.compiler.frontend.parse";

    /// Structural validation.
    pub const STRUCTURAL_VALIDATION: &str =
        "zamani.compiler.frontend.structural_validation";

    /// Macro expansion and AST transformation.
    pub const MACRO_EXPANSION: &str =
        "zamani.compiler.frontend.macro_expansion";

    /// Name and symbol resolution.
    pub const NAME_RESOLUTION: &str =
        "zamani.compiler.semantic.name_resolution";

    // ------------------------------------------------------------------------
    // Semantic analysis
    // ------------------------------------------------------------------------

    /// Canonical semantic analysis.
    pub const SEMANTIC_ANALYSIS: &str =
        "zamani.compiler.semantic.analysis";

    /// Ownership and borrow analysis.
    pub const OWNERSHIP_ANALYSIS: &str =
        "zamani.compiler.semantic.ownership";

    /// Effect and dependency analysis.
    pub const EFFECT_DEPENDENCY_ANALYSIS: &str =
        "zamani.compiler.semantic.effects_dependencies";

    /// Compile-time evaluation.
    pub const COMPILE_TIME_EVALUATION: &str =
        "zamani.compiler.compile_time.evaluation";

    // ------------------------------------------------------------------------
    // Intermediate representations
    // ------------------------------------------------------------------------

    /// Canonical IR construction and domain IR integration.
    pub const IR_CONSTRUCTION: &str =
        "zamani.compiler.ir.construction";

    /// Canonical IR verification.
    pub const IR_VERIFICATION: &str =
        "zamani.compiler.ir.verification";

    /// Generic and other permitted specialization.
    pub const SPECIALIZATION: &str =
        "zamani.compiler.specialization";

    // ------------------------------------------------------------------------
    // Optimization
    // ------------------------------------------------------------------------

    /// Registered optimization pipeline.
    pub const OPTIMIZATION: &str =
        "zamani.compiler.optimization";

    // ------------------------------------------------------------------------
    // Capability and feasibility
    // ------------------------------------------------------------------------

    /// Capability compatibility analysis.
    pub const CAPABILITY_ANALYSIS: &str =
        "zamani.compiler.analysis.capabilities";

    /// Compilation and realization feasibility.
    pub const FEASIBILITY_ANALYSIS: &str =
        "zamani.compiler.analysis.feasibility";

    // ------------------------------------------------------------------------
    // Planning
    // ------------------------------------------------------------------------

    /// Heterogeneous realization planning.
    pub const HETEROGENEOUS_PLANNING: &str =
        "zamani.compiler.planning.heterogeneous";

    /// Computation partitioning.
    pub const PARTITIONING: &str =
        "zamani.compiler.planning.partitioning";

    /// Data movement planning.
    pub const DATA_MOVEMENT_PLANNING: &str =
        "zamani.compiler.planning.data_movement";

    /// Abstract routing planning.
    pub const ROUTING_PLANNING: &str =
        "zamani.compiler.planning.routing";

    /// Abstract scheduling planning.
    pub const SCHEDULING_PLANNING: &str =
        "zamani.compiler.planning.scheduling";

    // ------------------------------------------------------------------------
    // Lowering
    // ------------------------------------------------------------------------

    /// Domain and target lowering.
    pub const LOWERING: &str =
        "zamani.compiler.lowering";

    // ------------------------------------------------------------------------
    // Linking
    // ------------------------------------------------------------------------

    /// Module and artifact linking.
    pub const LINKING: &str =
        "zamani.compiler.linking";

    // ------------------------------------------------------------------------
    // Backend
    // ------------------------------------------------------------------------

    /// Target or portable artifact generation.
    pub const CODE_GENERATION: &str =
        "zamani.compiler.backend.code_generation";

    // ------------------------------------------------------------------------
    // Artifacts
    // ------------------------------------------------------------------------

    /// Artifact integrity and compatibility validation.
    pub const ARTIFACT_VALIDATION: &str =
        "zamani.compiler.artifacts.validation";

    /// Artifact portability validation.
    pub const PORTABILITY_VALIDATION: &str =
        "zamani.compiler.artifacts.portability_validation";

    // ------------------------------------------------------------------------
    // Finalization
    // ------------------------------------------------------------------------

    /// Compilation result finalization.
    pub const FINALIZATION: &str =
        "zamani.compiler.finalization";
}

// ============================================================================
// DOMAIN IDENTIFIERS
// ============================================================================

/// Common computational-domain namespaces.
///
/// These constants provide convenient identifiers.
///
/// They do not constitute a closed list of supported computing domains.
///
/// New domains can introduce their own namespaces without modifying
/// this module.
pub mod domains {

    pub const CLASSICAL: &str =
        "zamani.domain.classical";

    pub const QUANTUM: &str =
        "zamani.domain.quantum";

    pub const AI: &str =
        "zamani.domain.ai";

    pub const TENSOR: &str =
        "zamani.domain.tensor";

    pub const HDL: &str =
        "zamani.domain.hdl";

    pub const HARDWARE: &str =
        "zamani.domain.hardware";

    pub const HYBRID: &str =
        "zamani.domain.hybrid";

    pub const DISTRIBUTED: &str =
        "zamani.domain.distributed";

    pub const NETWORKING: &str =
        "zamani.domain.networking";

    pub const CRYPTOGRAPHY: &str =
        "zamani.domain.cryptography";

    pub const NEUROMORPHIC: &str =
        "zamani.domain.neuromorphic";

    pub const PHOTONIC: &str =
        "zamani.domain.photonic";
}

// ============================================================================
// DOMAIN STAGE IDENTIFIERS
// ============================================================================

/// Well-known domain integration points.
///
/// These identifiers do not imply that an implementation exists.
///
/// Domain implementations remain owned by their respective subsystems.
pub mod domain_stages {

    pub const CLASSICAL_IR: &str =
        "zamani.compiler.domain.classical.ir";

    pub const QUANTUM_IR: &str =
        "zamani.compiler.domain.quantum.ir";

    pub const AI_IR: &str =
        "zamani.compiler.domain.ai.ir";

    pub const TENSOR_IR: &str =
        "zamani.compiler.domain.tensor.ir";

    pub const HDL_IR: &str =
        "zamani.compiler.domain.hdl.ir";

    pub const HYBRID_ANALYSIS: &str =
        "zamani.compiler.domain.hybrid.analysis";

    pub const DISTRIBUTED_ANALYSIS: &str =
        "zamani.compiler.domain.distributed.analysis";

    pub const NETWORKING_ANALYSIS: &str =
        "zamani.compiler.domain.networking.analysis";

    pub const CRYPTOGRAPHY_ANALYSIS: &str =
        "zamani.compiler.domain.cryptography.analysis";
}

// ============================================================================
// STANDARD STAGE ROLE
// ============================================================================

/// Organizational role of a standard compiler stage.
///
/// This is metadata, not an execution policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StandardStageRole {

    Initialization,

    Frontend,

    Semantic,

    IntermediateRepresentation,

    Verification,

    CompileTime,

    Specialization,

    Optimization,

    CapabilityAnalysis,

    FeasibilityAnalysis,

    Planning,

    Lowering,

    Linking,

    CodeGeneration,

    ArtifactValidation,

    PortabilityValidation,

    Finalization,
}

// ============================================================================
// ACTIVATION POLICY
// ============================================================================

/// Declares the default activation classification of a stage.
///
/// Activation metadata must be interpreted by pipeline orchestration.
///
/// It does not automatically cause the executor to skip or execute a stage.
///
/// A stage marked Conditional can still be mandatory for a particular
/// compilation after its applicability has been established.
///
/// A PolicyControlled stage requires explicit policy authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageActivation {

    /// Required for every standard compilation.
    Required,

    /// Activated when validated program characteristics require it.
    Conditional,

    /// Activated by explicit compilation policy.
    PolicyControlled,
}

impl StageActivation {

    /// Whether the stage is unconditionally required.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Whether the stage is conditionally activated.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional)
    }

    /// Whether explicit policy authorization is necessary.
    #[must_use]
    pub const fn is_policy_controlled(self) -> bool {
        matches!(self, Self::PolicyControlled)
    }
}

// ============================================================================
// STANDARD STAGE DEFINITION
// ============================================================================

/// Immutable definition of a standard compiler stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardStageDefinition {

    descriptor: StageDescriptor,

    role: StandardStageRole,

    activation: StageActivation,
}

impl StandardStageDefinition {

    /// Constructs a standard stage definition.
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

    /// Returns the organizational role.
    #[must_use]
    pub const fn role(&self) -> StandardStageRole {
        self.role
    }

    /// Returns the activation classification.
    #[must_use]
    pub const fn activation(&self) -> StageActivation {
        self.activation
    }

    /// Returns whether the stage is unconditionally required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.activation.is_required()
    }

    /// Returns whether the stage is conditionally activated.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.activation.is_conditional()
    }

    /// Returns whether explicit policy activation is necessary.
    #[must_use]
    pub const fn is_policy_controlled(&self) -> bool {
        self.activation.is_policy_controlled()
    }

    /// Consumes the definition and returns its descriptor.
    #[must_use]
    pub fn into_descriptor(self) -> StageDescriptor {
        self.descriptor
    }
}

// ============================================================================
// STANDARD STAGE ERRORS
// ============================================================================

/// Errors encountered while constructing or validating standard stages.
#[derive(Debug)]
#[non_exhaustive]
pub enum StandardStageError {

    InvalidIdentifier {
        identifier: String,
        message: String,
    },

    InvalidDescriptor {
        identifier: String,
        source: StageDescriptorError,
    },

    DuplicateIdentifier {
        identifier: String,
    },

    MissingStandardDependency {
        stage: String,
        dependency: String,
    },

    /// A required stage depends directly on a stage that is not
    /// unconditionally required.
    ///
    /// This would allow a skipped optional stage to block the
    /// mandatory compilation path.
    RequiredStageDependsOnOptionalDependency {
        stage: String,
        dependency: String,
    },

    EmptyDefinitions,
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
                "invalid stage identifier `{identifier}`: {message}"
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
                "duplicate standard stage `{identifier}`"
            ),

            Self::MissingStandardDependency {
                stage,
                dependency,
            } => write!(
                formatter,
                "stage `{stage}` references missing dependency `{dependency}`"
            ),

            Self::RequiredStageDependsOnOptionalDependency {
                stage,
                dependency,
            } => write!(
                formatter,
                "required stage `{stage}` depends on non-required stage `{dependency}`"
            ),

            Self::EmptyDefinitions => {
                formatter.write_str(
                    "standard stage definitions cannot be empty"
                )
            }
        }
    }
}

impl Error for StandardStageError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {

        match self {

            Self::InvalidDescriptor { source, .. } => {
                Some(source)
            }

            _ => None,
        }
    }
}

// ============================================================================
// INTERNAL STAGE CONSTRUCTION
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
    budget_dimension: Option<BudgetDimension>,
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

    if let Some(dimension) = budget_dimension {

        descriptor = descriptor
            .with_budget_dimension(dimension);
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

/// Constructs the standard Zamani compilation-stage definitions.
///
/// This function creates descriptors only.
///
/// It does not create implementations or register providers.
///
/// The standard mandatory path produces a validated compilation artifact.
/// It does not require every optional heterogeneous-planning stage.
///
/// Stages that have no applicable work must still have a valid,
/// explicit execution outcome. The implementation must not silently
/// fabricate an output or claim that an unperformed operation succeeded.
///
/// The returned graph is deterministic.
pub fn standard_stage_definitions(
) -> Result<Vec<StandardStageDefinition>, StandardStageError> {

    use ids::*;

    let mut definitions = Vec::new();

    // ========================================================================
    // 1. INITIALIZATION
    // ========================================================================

    definitions.push(create_stage(
        INITIALIZATION,
        "Compiler initialization",
        "Prepare compilation state, configuration and required compiler services.",
        CompilationPhase::Initialization,
        &[],
        StandardStageRole::Initialization,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 2. FRONTEND
    // ========================================================================

    definitions.push(create_stage(
        FRONTEND_PARSE,
        "Frontend parsing",
        "Integrate the canonical lexer, parser and domain-neutral AST.",
        CompilationPhase::Parsing,
        &[INITIALIZATION],
        StandardStageRole::Frontend,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::input_bytes()),
    )?);

    definitions.push(create_stage(
        STRUCTURAL_VALIDATION,
        "Structural validation",
        "Validate AST structural invariants without redefining language semantics.",
        CompilationPhase::Parsing,
        &[FRONTEND_PARSE],
        StandardStageRole::Frontend,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        MACRO_EXPANSION,
        "Macro expansion",
        "Integrate hygienic macro expansion using the authoritative frontend AST.",
        CompilationPhase::Parsing,
        &[STRUCTURAL_VALIDATION],
        StandardStageRole::Frontend,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 3. SEMANTIC ANALYSIS
    // ========================================================================

    definitions.push(create_stage(
        NAME_RESOLUTION,
        "Name and symbol resolution",
        "Resolve identifiers, scopes, namespaces and symbol references.",
        CompilationPhase::NameAnalysis,
        &[MACRO_EXPANSION],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        SEMANTIC_ANALYSIS,
        "Canonical semantic analysis",
        "Integrate authoritative types, effects, requirements, capabilities, constraints and program meaning.",
        CompilationPhase::SemanticAnalysis,
        &[NAME_RESOLUTION],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // These two analyses are independent after semantic validation.
    // The IR construction stage consumes both results.

    definitions.push(create_stage(
        OWNERSHIP_ANALYSIS,
        "Ownership and borrow analysis",
        "Integrate the authoritative ownership and lifetime contracts.",
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
        "Integrate semantic effects and dependency evidence without duplicating semantic authority.",
        CompilationPhase::DependencyAnalysis,
        &[SEMANTIC_ANALYSIS],
        StandardStageRole::Semantic,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        COMPILE_TIME_EVALUATION,
        "Compile-time evaluation",
        "Evaluate permitted compile-time expressions under explicit evaluation policies and budgets.",
        CompilationPhase::SemanticAnalysis,
        &[
            SEMANTIC_ANALYSIS,
            OWNERSHIP_ANALYSIS,
            EFFECT_DEPENDENCY_ANALYSIS,
        ],
        StandardStageRole::CompileTime,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::constant_evaluation_work()),
    )?);

    // ========================================================================
    // 4. CANONICAL IR
    // ========================================================================

    definitions.push(create_stage(
        IR_CONSTRUCTION,
        "Canonical IR construction",
        "Integrate ZUIR and authoritative classical, quantum, HDL and other domain IRs.",
        CompilationPhase::IrGeneration,
        &[
            SEMANTIC_ANALYSIS,
            OWNERSHIP_ANALYSIS,
            EFFECT_DEPENDENCY_ANALYSIS,
            COMPILE_TIME_EVALUATION,
        ],
        StandardStageRole::IntermediateRepresentation,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::ir_entities()),
    )?);

    definitions.push(create_stage(
        IR_VERIFICATION,
        "Canonical IR verification",
        "Verify canonical IR invariants and applicable domain representation contracts.",
        CompilationPhase::IrVerification,
        &[IR_CONSTRUCTION],
        StandardStageRole::Verification,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 5. SPECIALIZATION AND OPTIMIZATION
    // ========================================================================

    definitions.push(create_stage(
        SPECIALIZATION,
        "Program specialization",
        "Integrate generic specialization and monomorphization while preserving semantic contracts.",
        CompilationPhase::Optimization,
        &[IR_VERIFICATION],
        StandardStageRole::Specialization,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        OPTIMIZATION,
        "Program optimization",
        "Execute registered legal transformations and semantics-preserving optimization passes.",
        CompilationPhase::Optimization,
        &[SPECIALIZATION],
        StandardStageRole::Optimization,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::pass_applications()),
    )?);

    // ========================================================================
    // 6. CAPABILITY AND FEASIBILITY
    // ========================================================================

    definitions.push(create_stage(
        CAPABILITY_ANALYSIS,
        "Capability analysis",
        "Match validated requirements against registered abstract target and implementation capabilities.",
        CompilationPhase::CapabilityAnalysis,
        &[
            SEMANTIC_ANALYSIS,
            IR_VERIFICATION,
            OPTIMIZATION,
        ],
        StandardStageRole::CapabilityAnalysis,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        FEASIBILITY_ANALYSIS,
        "Compilation feasibility",
        "Assess whether candidate realizations satisfy requirements, capabilities, constraints and available resource evidence.",
        CompilationPhase::FeasibilityAnalysis,
        &[CAPABILITY_ANALYSIS],
        StandardStageRole::FeasibilityAnalysis,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 7. OPTIONAL REALIZATION PLANNING
    // ========================================================================

    // These stages form an optional planning branch.
    //
    // They are deliberately not mandatory prerequisites of the
    // universal compilation path.
    //
    // If a selected realization requires them, orchestration must
    // activate them and enforce their completion before the relevant
    // realization stage consumes their results.

    definitions.push(create_stage(
        HETEROGENEOUS_PLANNING,
        "Heterogeneous realization planning",
        "Construct abstract plans for computation spanning compatible computational domains.",
        CompilationPhase::Planning,
        &[FEASIBILITY_ANALYSIS],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        PARTITIONING,
        "Computation partitioning",
        "Produce abstract computation partitions using semantic dependencies and capability evidence.",
        CompilationPhase::Planning,
        &[HETEROGENEOUS_PLANNING],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        DATA_MOVEMENT_PLANNING,
        "Data movement planning",
        "Produce abstract data movement requirements and transfer plans.",
        CompilationPhase::Planning,
        &[PARTITIONING],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        ROUTING_PLANNING,
        "Routing planning",
        "Coordinate abstract routing requirements through the appropriate domain authorities.",
        CompilationPhase::Planning,
        &[PARTITIONING],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        SCHEDULING_PLANNING,
        "Scheduling planning",
        "Produce abstract scheduling constraints and proposals without performing runtime scheduling.",
        CompilationPhase::Planning,
        &[FEASIBILITY_ANALYSIS],
        StandardStageRole::Planning,
        StageActivation::Conditional,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 8. LOWERING
    // ========================================================================

    definitions.push(create_stage(
        LOWERING,
        "Domain and target lowering",
        "Perform verified representation conversion, legalization and domain-specific lowering.",
        CompilationPhase::Lowering,
        &[
            FEASIBILITY_ANALYSIS,
            OPTIMIZATION,
        ],
        StandardStageRole::Lowering,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::ir_entities()),
    )?);

    // ========================================================================
    // 9. LINKING
    // ========================================================================

    definitions.push(create_stage(
        LINKING,
        "Compilation linking",
        "Resolve symbols and compose compatible modules and compilation artifacts.",
        CompilationPhase::Linking,
        &[LOWERING],
        StandardStageRole::Linking,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 10. BACKEND REALIZATION
    // ========================================================================

    definitions.push(create_stage(
        CODE_GENERATION,
        "Code and artifact generation",
        "Invoke registered backends or portable artifact providers to produce genuine compilation outputs.",
        CompilationPhase::CodeGeneration,
        &[LINKING],
        StandardStageRole::CodeGeneration,
        StageActivation::Required,
        StageDeterminism::InputDependent,
        StageConcurrency::InstanceExclusive,
        Some(BudgetDimension::artifact_bytes()),
    )?);

    // ========================================================================
    // 11. ARTIFACT VALIDATION
    // ========================================================================

    definitions.push(create_stage(
        ARTIFACT_VALIDATION,
        "Artifact validation",
        "Validate output integrity, compatibility, provenance and structural correctness.",
        CompilationPhase::ArtifactValidation,
        &[CODE_GENERATION],
        StandardStageRole::ArtifactValidation,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    definitions.push(create_stage(
        PORTABILITY_VALIDATION,
        "Portability validation",
        "Validate declared source, semantic, IR and artifact portability guarantees.",
        CompilationPhase::ArtifactValidation,
        &[ARTIFACT_VALIDATION],
        StandardStageRole::PortabilityValidation,
        StageActivation::Required,
        StageDeterminism::Deterministic,
        StageConcurrency::InstanceExclusive,
        None,
    )?);

    // ========================================================================
    // 12. FINALIZATION
    // ========================================================================

    definitions.push(create_stage(
        FINALIZATION,
        "Compilation finalization",
        "Assemble the verified compilation result, diagnostics, artifacts and provenance.",
        CompilationPhase::Finalization,
        &[PORTABILITY_VALIDATION],
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

/// Validates standard stage definitions.
///
/// Checks:
///
/// - The definition set is not empty.
/// - Identifiers are unique.
/// - All standard dependencies exist.
/// - Required stages do not directly depend on conditional or
///   policy-controlled stages.
///
/// Cycle detection and topological ordering remain owned by
/// pipeline::ordering.
pub fn validate_standard_definitions(
    definitions: &[StandardStageDefinition],
) -> Result<(), StandardStageError> {

    if definitions.is_empty() {
        return Err(StandardStageError::EmptyDefinitions);
    }

    let mut identifiers = BTreeSet::<String>::new();

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

            let dependency_name = dependency.as_str();

            if !identifiers.contains(dependency_name) {

                return Err(
                    StandardStageError::MissingStandardDependency {
                        stage: stage.to_owned(),
                        dependency: dependency_name.to_owned(),
                    }
                );
            }

            if definition.is_required() {

                let dependency_definition = definitions
                    .iter()
                    .find(|candidate| {
                        candidate.descriptor().id()
                            == dependency
                    })
                    .ok_or_else(|| {
                        StandardStageError::MissingStandardDependency {
                            stage: stage.to_owned(),
                            dependency: dependency_name.to_owned(),
                        }
                    })?;

                if !dependency_definition.is_required() {

                    return Err(
                        StandardStageError::RequiredStageDependsOnOptionalDependency {
                            stage: stage.to_owned(),
                            dependency: dependency_name.to_owned(),
                        }
                    );
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// ACTIVATION QUERIES
// ============================================================================

/// Returns identifiers of all unconditionally required standard stages.
///
/// The returned order is deterministic and follows the definition order.
///
/// This function does not decide whether a conditional stage is
/// applicable to a particular program.
pub fn required_standard_stage_ids(
    definitions: &[StandardStageDefinition],
) -> Vec<StageId> {

    definitions
        .iter()
        .filter(|definition| definition.is_required())
        .map(|definition| {
            definition.descriptor().id().clone()
        })
        .collect()
}

/// Returns identifiers of conditionally activated stages.
pub fn conditional_standard_stage_ids(
    definitions: &[StandardStageDefinition],
) -> Vec<StageId> {

    definitions
        .iter()
        .filter(|definition| definition.is_conditional())
        .map(|definition| {
            definition.descriptor().id().clone()
        })
        .collect()
}

/// Returns identifiers requiring explicit policy authorization.
pub fn policy_controlled_standard_stage_ids(
    definitions: &[StandardStageDefinition],
) -> Vec<StageId> {

    definitions
        .iter()
        .filter(|definition| {
            definition.is_policy_controlled()
        })
        .map(|definition| {
            definition.descriptor().id().clone()
        })
        .collect()
}

// ============================================================================
// DOMAIN STAGE DESCRIPTOR CONSTRUCTION
// ============================================================================

/// Constructs an independently-owned domain stage descriptor.
///
/// The caller supplies its identifier and dependencies.
///
/// The domain subsystem owns:
///
/// - Stage implementation.
/// - Domain semantics.
/// - Domain-specific IR.
/// - Domain-specific verification.
/// - Domain-specific legality.
/// - Domain-specific realization.
///
/// The compiler owns orchestration and integration.
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
// STANDARD IDENTIFIER CONSTRUCTION
// ============================================================================

/// Constructs a validated StageId from a standard or extension identifier.
///
/// This function does not check whether the identifier is registered.
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
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn standard_definitions_are_constructible() {

        let definitions = standard_stage_definitions()
            .expect("standard definitions must be valid");

        assert!(!definitions.is_empty());
    }

    #[test]
    fn standard_definitions_are_deterministic() {

        let first = standard_stage_definitions()
            .expect("first construction");

        let second = standard_stage_definitions()
            .expect("second construction");

        assert_eq!(first, second);
    }

    #[test]
    fn standard_identifiers_are_unique() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        validate_standard_definitions(&definitions)
            .expect("unique identifiers");
    }

    #[test]
    fn every_standard_dependency_exists() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        let identifiers: BTreeSet<String> = definitions
            .iter()
            .map(|definition| {
                definition.descriptor()
                    .id()
                    .as_str()
                    .to_owned()
            })
            .collect();

        for definition in &definitions {

            for dependency in definition.descriptor().dependencies() {

                assert!(
                    identifiers.contains(dependency.as_str()),
                    "missing dependency {} for {}",
                    dependency,
                    definition.descriptor().id()
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Activation
    // ------------------------------------------------------------------------

    #[test]
    fn mandatory_path_does_not_depend_on_optional_stages() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        for definition in &definitions {

            if !definition.is_required() {
                continue;
            }

            for dependency in definition.descriptor().dependencies() {

                let prerequisite = definitions
                    .iter()
                    .find(|candidate| {
                        candidate.descriptor().id() == dependency
                    })
                    .expect("dependency exists");

                assert!(
                    prerequisite.is_required(),
                    "required stage {} depends on optional stage {}",
                    definition.descriptor().id(),
                    dependency
                );
            }
        }
    }

    #[test]
    fn required_stage_query_returns_mandatory_stages() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        let required = required_standard_stage_ids(&definitions);

        assert!(
            required.iter().any(|id| {
                id.as_str() == ids::INITIALIZATION
            })
        );

        assert!(
            required.iter().any(|id| {
                id.as_str() == ids::FINALIZATION
            })
        );

        assert!(
            !required.iter().any(|id| {
                id.as_str() == ids::HETEROGENEOUS_PLANNING
            })
        );
    }

    #[test]
    fn planning_stages_are_conditional() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        let conditional =
            conditional_standard_stage_ids(&definitions);

        assert!(
            conditional.iter().any(|id| {
                id.as_str() == ids::HETEROGENEOUS_PLANNING
            })
        );

        assert!(
            conditional.iter().any(|id| {
                id.as_str() == ids::PARTITIONING
            })
        );
    }

    #[test]
    fn no_standard_stage_requires_policy_authorization_by_default() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        assert!(
            policy_controlled_standard_stage_ids(&definitions).is_empty()
        );
    }

    // ------------------------------------------------------------------------
    // Activation metadata
    // ------------------------------------------------------------------------

    #[test]
    fn activation_classification_is_queryable() {

        assert!(StageActivation::Required.is_required());

        assert!(StageActivation::Conditional.is_conditional());

        assert!(
            StageActivation::PolicyControlled
                .is_policy_controlled()
        );
    }

    // ------------------------------------------------------------------------
    // Stable identifiers
    // ------------------------------------------------------------------------

    #[test]
    fn existing_standard_identifiers_remain_stable() {

        assert_eq!(
            ids::INITIALIZATION,
            "zamani.compiler.initialization"
        );

        assert_eq!(
            ids::FRONTEND_PARSE,
            "zamani.compiler.frontend.parse"
        );

        assert_eq!(
            ids::SEMANTIC_ANALYSIS,
            "zamani.compiler.semantic.analysis"
        );

        assert_eq!(
            ids::IR_CONSTRUCTION,
            "zamani.compiler.ir.construction"
        );

        assert_eq!(
            ids::IR_VERIFICATION,
            "zamani.compiler.ir.verification"
        );

        assert_eq!(
            ids::OPTIMIZATION,
            "zamani.compiler.optimization"
        );

        assert_eq!(
            ids::CODE_GENERATION,
            "zamani.compiler.backend.code_generation"
        );

        assert_eq!(
            ids::FINALIZATION,
            "zamani.compiler.finalization"
        );
    }

    #[test]
    fn extension_identifiers_are_supported() {

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
    fn dynamically_sized_identifiers_are_supported() {

        let identifier = format!(
            "zamani.extension.{}",
            "future".repeat(10_000)
        );

        let id = standard_stage_id(&identifier)
            .expect("valid dynamic identifier");

        assert_eq!(id.as_str(), identifier);
    }

    #[test]
    fn invalid_identifiers_are_rejected() {

        assert!(standard_stage_id("").is_err());

        assert!(
            standard_stage_id("invalid\nstage").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Domain integration
    // ------------------------------------------------------------------------

    #[test]
    fn quantum_domain_descriptor_is_supported() {

        let dependency = standard_stage_id(ids::IR_VERIFICATION)
            .expect("dependency");

        let descriptor = domain_stage_descriptor(
            domain_stages::QUANTUM_IR,
            "Quantum IR integration",
            "1",
            "Integrates the canonical quantum IR authority.",
            CompilationPhase::IrGeneration,
            [dependency],
            StageDeterminism::Deterministic,
            StageConcurrency::InstanceExclusive,
            None,
        )
        .expect("quantum descriptor");

        assert_eq!(
            descriptor.id().as_str(),
            domain_stages::QUANTUM_IR
        );
    }

    #[test]
    fn future_domain_descriptor_is_supported() {

        let dependency = standard_stage_id(ids::IR_VERIFICATION)
            .expect("dependency");

        let descriptor = domain_stage_descriptor(
            "zamani.compiler.domain.photonic.lowering",
            "Photonic lowering",
            "1",
            "Future photonic integration.",
            CompilationPhase::Lowering,
            [dependency],
            StageDeterminism::InputDependent,
            StageConcurrency::InstanceExclusive,
            None,
        )
        .expect("photonic descriptor");

        assert_eq!(
            descriptor.id().as_str(),
            "zamani.compiler.domain.photonic.lowering"
        );
    }

    // ------------------------------------------------------------------------
    // Domain ownership
    // ------------------------------------------------------------------------

    #[test]
    fn quantum_is_not_a_mandatory_global_domain_stage() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        assert!(
            !definitions.iter().any(|definition| {
                definition.descriptor().id().as_str()
                    == domain_stages::QUANTUM_IR
            })
        );
    }

    #[test]
    fn ai_is_not_a_mandatory_global_domain_stage() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        assert!(
            !definitions.iter().any(|definition| {
                definition.descriptor().id().as_str()
                    == domain_stages::AI_IR
            })
        );
    }

    // ------------------------------------------------------------------------
    // Validation failures
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_definitions_are_rejected() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        let mut duplicated = definitions.clone();

        duplicated.push(definitions[0].clone());

        assert!(matches!(
            validate_standard_definitions(&duplicated),
            Err(StandardStageError::DuplicateIdentifier { .. })
        ));
    }

    #[test]
    fn empty_definitions_are_rejected() {

        assert!(matches!(
            validate_standard_definitions(&[]),
            Err(StandardStageError::EmptyDefinitions)
        ));
    }

    #[test]
    fn domain_descriptors_reject_invalid_identifiers() {

        assert!(
            domain_stage_descriptor(
                "",
                "Invalid stage",
                "1",
                "Invalid identifier",
                CompilationPhase::Lowering,
                [],
                StageDeterminism::Deterministic,
                StageConcurrency::InstanceExclusive,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn every_standard_stage_has_a_phase() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        for definition in &definitions {

            assert!(
                definition.descriptor().phase().is_some(),
                "missing phase for {}",
                definition.descriptor().id()
            );
        }
    }

    // ------------------------------------------------------------------------
    // Deterministic dependency metadata
    // ------------------------------------------------------------------------

    #[test]
    fn dependencies_are_deterministically_ordered() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        for definition in &definitions {

            let dependencies =
                definition.descriptor().dependencies();

            let mut sorted = dependencies.to_vec();

            sorted.sort();

            assert_eq!(
                dependencies,
                sorted.as_slice()
            );
        }
    }

    // ------------------------------------------------------------------------
    // POCO-REAF
    // ------------------------------------------------------------------------

    #[test]
    fn standard_identifiers_are_not_hardware_specific() {

        let definitions = standard_stage_definitions()
            .expect("definitions");

        for definition in &definitions {

            let identifier = definition.descriptor()
                .id()
                .as_str();

            assert!(
                !identifier.contains("MAX_CPUS")
            );

            assert!(
                !identifier.contains("MAX_GPUS")
            );

            assert!(
                !identifier.contains("MAX_QUBITS")
            );
        }
    }
}
