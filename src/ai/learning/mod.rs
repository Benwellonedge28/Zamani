
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Learning Subsystem
//!
//! Path: `src/ai/learning/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the canonical composition root for Zamani's AI learning
//! computational domain.
//!
//! The learning subsystem describes target-independent learning
//! computation and exposes the contracts required to express,
//! validate, compose, optimize, and integrate learning operations.
//!
//! It supports extensible learning paradigms, including:
//!
//! - Machine learning.
//! - Deep learning.
//! - Statistical learning.
//! - Neural learning.
//! - Symbolic learning.
//! - Probabilistic learning.
//! - Reinforcement learning.
//! - Federated learning.
//! - Distributed learning.
//! - Transfer learning.
//! - Continual and adaptive learning.
//! - Differentiable learning.
//! - Neuro-symbolic learning.
//! - Quantum-assisted learning.
//! - Hybrid learning.
//! - Future learning paradigms.
//!
//! These capabilities are provided through independently owned
//! learning-domain modules and their integration contracts.
//!
//! This file does not implement learning algorithms, numerical
//! computation, model execution, or hardware management.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Learning subsystem composition.
//! - Public learning module registration.
//! - Learning module visibility boundaries.
//! - The canonical Rust module path for AI learning.
//!
//! Individual modules own their respective learning contracts.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Source grammar.
//! - Lexer or parser implementation.
//! - AST construction.
//! - Semantic name resolution.
//! - Canonical type checking.
//! - AI model definitions.
//! - Tensor storage.
//! - Universal intermediate representation.
//! - Numerical algorithms.
//! - Automatic differentiation implementation.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Device selection.
//! - Execution scheduling.
//! - Distributed communication.
//! - Quantum execution.
//! - Runtime execution.
//! - Backend code generation.
//!
//! These responsibilities remain with their authoritative
//! repository subsystems.
//!
//! ## Canonical integration architecture
//!
//! Source grammar:
//!     grammar/ai/learning.g4
//!     grammar/ai/training.g4
//!     grammar/ai/reinforcement.g4
//!     grammar/ai/federated-learning.g4
//!     grammar/ai/transfer-learning.g4
//!     grammar/ai/adaptation.g4
//!     grammar/ai/optimization.g4
//!
//! Grammar composition:
//!     grammar/ai/ai.g4
//!
//! Frontend semantic authority:
//!     crate::frontend::semantic::domains::ai
//!
//! Canonical AI identifiers:
//!     crate::ai::ids
//!
//! Canonical AI errors:
//!     crate::ai::error
//!
//! Canonical AI requirements:
//!     crate::ai::requirements
//!
//! Model definitions:
//!     crate::ai::model
//!
//! Tensor computation:
//!     crate::ai::tensor
//!
//! Automatic differentiation:
//!     crate::ai::differentiation
//!
//! Universal AI representation:
//!     crate::zuir::dialects::ai
//!
//! Universal tensor representation:
//!     crate::zuir::dialects::tensor
//!
//! Classical computation:
//!     crate::classical
//!
//! Quantum computation:
//!     crate::quantum::ir
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Hybrid composition:
//!     crate::hybrid
//!
//! Actual execution:
//!     compiler / backend / runtime
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//!     Canonical frontend and semantic services
//!                     |
//!                     v
//!             AI learning contracts
//!                     |
//!                     v
//!             AI domain composition
//!                     |
//!                     v
//!             Canonical ZUIR
//!                     |
//!                     v
//!          Optimization and lowering
//!                     |
//!                     v
//!        Execution planning and scheduling
//!                     |
//!                     v
//!             Runtime / backend
//!
//! This composition root must not introduce reverse dependencies
//! from foundational learning contracts into runtime or backend
//! implementations.
//!
//! ## POCO-REAF
//!
//! Learning declarations must remain independent of physical
//! execution targets.
//!
//! The same learning computation may be realized using:
//!
//! - Embedded processors.
//! - CPUs.
//! - Multicore processors.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - AI accelerators.
//! - Quantum-assisted systems.
//! - Neuromorphic systems.
//! - Heterogeneous systems.
//! - Distributed systems.
//! - Future computational architectures.
//!
//! This module introduces no language-level maximum for:
//!
//! - Learning algorithms.
//! - Algorithm versions.
//! - Training workflows.
//! - Training stages.
//! - Training operations.
//! - Objectives.
//! - Optimizers.
//! - Parameters.
//! - Gradient variables.
//! - Reinforcement-learning states.
//! - Actions.
//! - Episodes.
//! - Federated participants.
//! - Transfer mappings.
//! - Adaptation strategies.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Workers.
//! - Devices.
//! - Accelerators.
//! - Distributed nodes.
//!
//! Actual execution capacity is determined by available resources,
//! declared requirements, implementation capabilities, operational
//! policies, and physical feasibility.
//!
//! Operational budgets are not language-level semantic ceilings.
//!
//! Literal infinite computation is not promised.
//!
//! ## Integration invariants
//!
//! 1. Every learning feature must have one authoritative owner.
//!
//! 2. Learning algorithms describe computational intent and contracts;
//!    they do not execute numerical kernels.
//!
//! 3. Training describes workflows and their dependencies;
//!    it does not implement the training runtime.
//!
//! 4. Optimizers describe update semantics;
//!    they do not allocate or update physical tensor storage.
//!
//! 5. Objectives describe optimization intent;
//!    they do not evaluate mathematical expressions.
//!
//! 6. Gradient contracts describe differentiation requests;
//!    automatic differentiation belongs to the differentiation subsystem.
//!
//! 7. Reinforcement learning describes policies, environments,
//!    transitions, and learning intent;
//!    it does not own environment execution.
//!
//! 8. Federated learning describes logical participants,
//!    aggregation intent, privacy requirements, and recovery contracts;
//!    distributed infrastructure owns communication and coordination.
//!
//! 9. Transfer learning describes reusable model knowledge and
//!    compatibility requirements;
//!    model and artifact subsystems own their respective storage.
//!
//! 10. Adaptive learning describes adaptation intent;
//!     runtime and semantic authorities determine whether changes
//!     are valid, authorized, and executable.
//!
//! 11. Canonical frontend semantic services remain authoritative
//!     for source-level meaning.
//!
//! 12. ZUIR remains authoritative for universal computational
//!     representation.
//!
//! 13. No learning module may silently report execution success
//!     when only a declaration or validation has succeeded.
//!
//! ## Compatibility policy
//!
//! Individual learning modules may evolve independently while
//! preserving their documented public contracts.
//!
//! New learning paradigms should be introduced through additional
//! independently owned modules or extensions.
//!
//! Existing module identities must not be silently reassigned
//! to unrelated functionality.
//!
//! This composition root should only change when the subsystem's
//! actual module architecture changes.
//!
//! ## Freeze criteria
//!
//! - Every declared child module exists.
//! - Every child module has one authoritative responsibility.
//! - Existing public module paths are preserved.
//! - Cross-module dependencies remain acyclic at the architectural
//!   ownership level.
//! - Canonical AI types and error contracts are reused.
//! - No competing semantic or IR authority is introduced.
//! - No hardware-specific implementation is introduced.
//! - No arbitrary workload ceilings are introduced.
//! - No unsafe Rust is permitted.
//! - Rust 1.97+ compatibility is maintained.
//! - Module-level documentation describes the actual architecture.

// ============================================================================
// LEARNING ALGORITHM CONTRACTS
// ============================================================================

/// Learning algorithm declarations, identities, versions, signatures,
/// parameters, state semantics, and algorithm registration.
///
/// This module describes algorithms; it does not execute them.
pub mod algorithms;

// ============================================================================
// TRAINING WORKFLOWS
// ============================================================================

/// Training workflow definitions, phases, operations, dependencies,
/// objectives, metrics, checkpoints, and reproducibility contracts.
///
/// This module describes training workflows; execution belongs to
/// the runtime and downstream execution infrastructure.
pub mod training;

// ============================================================================
// LEARNING OBJECTIVES
// ============================================================================

/// Learning objectives, objective composition, optimization
/// constraints, and loss-function contracts.
///
/// Mathematical evaluation belongs to the appropriate computational
/// and execution subsystems.
pub mod objectives;

// ============================================================================
// LEARNING OPTIMIZATION
// ============================================================================

/// Learning optimizer identities, versions, parameter groups,
/// update semantics, logical optimizer state, and optimizer contracts.
///
/// Numerical optimizer execution is owned by downstream implementations.
pub mod optimization;

// ============================================================================
// GRADIENT COMPUTATION CONTRACTS
// ============================================================================

/// Gradient computation requests, differentiation targets,
/// gradient bindings, and gradient contracts.
///
/// Actual differentiation is owned by `crate::ai::differentiation`.
pub mod gradients;

// ============================================================================
// REINFORCEMENT LEARNING
// ============================================================================

/// Reinforcement-learning problem definitions, environments,
/// policies, states, actions, rewards, trajectories, and update contracts.
///
/// Environment execution and numerical learning are external responsibilities.
pub mod reinforcement;

// ============================================================================
// FEDERATED LEARNING
// ============================================================================

/// Federated-learning declarations, logical participants,
/// aggregation contracts, communication requirements, privacy intent,
/// recovery contracts, and federation validation.
///
/// Communication and distributed execution belong to
/// `crate::distributed`.
pub mod federated;

// ============================================================================
// TRANSFER LEARNING
// ============================================================================

/// Transfer-learning contracts, source and target model relationships,
/// parameter mappings, compatibility, and transfer lineage.
///
/// Model storage and numerical transfer belong to their authoritative
/// model, artifact, and execution subsystems.
pub mod transfer;

// ============================================================================
// ADAPTIVE LEARNING
// ============================================================================

/// Adaptive-learning contracts, adaptation targets, feedback sources,
/// strategies, triggers, constraints, and adaptation lineage.
///
/// Actual adaptation execution belongs to downstream authorities.
pub mod adaptation;
