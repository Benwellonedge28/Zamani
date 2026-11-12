
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT
//!
//! Path: `src/zur/jit/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the public composition boundary for the Zamani Universal
//! Runtime's just-in-time compilation subsystem.
//!
//! It exposes the existing JIT components while preserving their individual
//! responsibilities and integration boundaries.
//!
//! The JIT subsystem coordinates:
//!
//! - Compiler-owned compilation requests.
//! - Canonical compiler pipeline integration.
//! - JIT artifact identity.
//! - Verified artifact caching.
//! - Artifact verification.
//! - Runtime execution profiling.
//! - Evidence-based hotness detection.
//! - Specialization validation.
//! - Deoptimization coordination.
//! - Artifact invalidation.
//!
//! # Architecture
//!
//! ```text
//!       Canonical compiler representations
//!                    |
//!                    v
//!             JIT compilation unit
//!                    |
//!                    v
//!               JIT engine
//!                    |
//!          +---------+----------+
//!          |         |          |
//!          v         v          v
//!       Compiler  Verification  Cache
//!       bridge    authority     |
//!          |         |          |
//!          +---------+----------+
//!                    |
//!                    v
//!             Verified artifact
//!                    |
//!                    v
//!          Runtime execution layer
//! ```
//!
//! Profiling and hotness analysis provide operational observations and
//! recommendations. Specialization, deoptimization and invalidation
//! coordinate their respective lifecycle operations.
//!
//! None of these components replaces the canonical compiler, backend,
//! intermediate representations, semantic layer or execution subsystem.
//!
//! # Ownership boundaries
//!
//! This module owns only JIT subsystem composition and public API exposure.
//!
//! It does not own:
//!
//! - Source grammar or parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Requirements, capabilities, preferences or constraints.
//! - Classical IR.
//! - Quantum IR.
//! - HDL or hardware IR.
//! - AI or tensor IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Backend registration or selection.
//! - Machine-code generation.
//! - Artifact manifest construction.
//! - Executable memory management.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Domain-specific execution.
//! - Distributed execution planning.
//! - Hybrid execution planning.
//!
//! Those responsibilities remain with their existing repository authorities.
//!
//! # Existing repository integration
//!
//! ## ZUR configuration
//!
//! `crate::zur::config` owns:
//!
//! - `RuntimeConfig`.
//! - `JitPolicy`.
//! - `ExecutionFallbackPolicy`.
//!
//! ## ZUR operational policy
//!
//! `crate::zur::policy` owns execution strategy preferences and operational
//! policy.
//!
//! ## ZUR errors
//!
//! `crate::zur::error` owns `ZurError` and `ZurResult`.
//!
//! ## Compiler
//!
//! `crate::compiler` and its existing IR, optimization, passes, lowering and
//! artifact subsystems remain authoritative for compilation.
//!
//! ## Backend
//!
//! `crate::backend` remains authoritative for backend registration,
//! capabilities, target compatibility and code generation.
//!
//! ## Execution
//!
//! The ZUR execution subsystem remains responsible for loading and executing
//! verified artifacts.
//!
//! # Grammar boundary
//!
//! The JIT subsystem consumes validated compiler and runtime contracts.
//! It does not parse source grammar or reinterpret AST nodes.
//!
//! Relevant grammar authorities include:
//!
//! - `grammar/execution/runtime.g4`
//! - `grammar/execution/runtime-capabilities.g4`
//! - `grammar/compile/optimization.g4`
//! - `grammar/compile/specialization.g4`
//! - `grammar/resources/portability.g4`
//! - `grammar/core/requirements.g4`
//! - `grammar/core/constraints.g4`
//! - `grammar/core/hints.g4`
//!
//! Their responsibility is to express source-level intent and constraints.
//! Semantic analysis validates that intent. The compiler establishes the
//! canonical compilation inputs. ZUR consumes those validated inputs.
//!
//! In particular:
//!
//! - A requirement is not a capability.
//! - A capability declaration is not proof of availability.
//! - A preference is not a mandatory constraint.
//! - A hint is not an execution guarantee.
//! - Feasibility is not resource reservation.
//! - A specialization request is not proof of semantic preservation.
//! - A cache hit is not proof of artifact validity.
//! - Successful compilation is not proof of successful execution.
//!
//! # POCO-REAF and scalability
//!
//! JIT compilation must preserve the validated semantics of the original
//! program.
//!
//! Different compatible execution environments may produce different
//! artifacts while preserving the same program behavior.
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Computational domains.
//! - Compilation units.
//! - Functions or modules.
//! - IR operations.
//! - Providers.
//! - Devices.
//! - Processors.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Compilation requests.
//! - Artifact identities.
//! - Profile observations.
//!
//! Operational budgets belong to explicit runtime configuration and
//! resource-management authorities.
//!
//! Physical memory, processing capacity, device availability and execution
//! policies remain real constraints. No finite machine provides literal
//! infinite computation.
//!
//! # Safety and correctness invariants
//!
//! 1. Safe Rust only.
//! 2. No raw pointers or global mutable state.
//! 3. No duplicate compiler or intermediate representation.
//! 4. No implicit backend selection.
//! 5. No fabricated compilation or execution success.
//! 6. No unverified artifact publication.
//! 7. No assumption that a cache hit is valid.
//! 8. No assumption that a registered provider is available.
//! 9. No automatic semantic degradation during fallback.
//! 10. No hard-coded hardware or computational-domain ceilings.
//! 11. JIT remains optional.
//! 12. Compiler, verification and cache dependencies are explicit.
//!
//! # Module integration
//!
//! Every declared child module corresponds to an existing implementation
//! in this directory. This composition module deliberately does not declare
//! future modules until their source files and public contracts exist.

// =============================================================================
// JIT ENGINE
// =============================================================================

/// JIT compilation orchestration, compilation requests, artifact contracts,
/// cancellation and validated AOT fallback.
pub mod engine;

// =============================================================================
// COMPILER INTEGRATION
// =============================================================================

/// Adapter between the JIT engine and the canonical Zamani compiler pipeline.
pub mod compiler_bridge;

// =============================================================================
// VERIFIED ARTIFACT CACHE
// =============================================================================

/// Concurrent storage, lookup, publication and invalidation of opaque
/// compiled JIT artifacts.
pub mod code_cache;

// =============================================================================
// ARTIFACT VERIFICATION
// =============================================================================

/// Artifact identity validation, verification authority contracts, evidence
/// binding and verification receipts.
pub mod verification;

// =============================================================================
// EXECUTION PROFILING
// =============================================================================

/// Runtime execution observations and profile aggregation for JIT
/// compilation units.
pub mod profiling;

// =============================================================================
// HOTNESS ANALYSIS
// =============================================================================

/// Configurable, evidence-based hotness analysis using runtime profiling
/// snapshots.
pub mod hotness;

// =============================================================================
// SPECIALIZATION
// =============================================================================

/// Validation and preparation of specialization requests using compiler
/// evidence and hotness decisions.
pub mod specialization;

// =============================================================================
// DEOPTIMIZATION
// =============================================================================

/// Validated coordination for transferring specialized execution to a
/// compatible baseline execution path.
pub mod deoptimization;

// =============================================================================
// INVALIDATION
// =============================================================================

/// Dependency-aware artifact invalidation and publication fencing.
pub mod invalidation;

// =============================================================================
// ENGINE PUBLIC API
// =============================================================================

pub use engine::{
    CompiledJitArtifact,
    JitArtifactVerifier,
    JitCacheKey,
    JitCacheStatus,
    JitCancellation,
    JitCodeCache,
    JitCompilationOutcome,
    JitCompilationReason,
    JitCompilationRequest,
    JitCompilationUnit,
    JitCompilerBridge,
    JitEngine,
    JitFallbackReason,
    NeverCancelled,
    ValidatedAotFallbackAuthority,
};

// =============================================================================
// COMPILER BRIDGE PUBLIC API
// =============================================================================

pub use compiler_bridge::{
    CanonicalCompilerOutput,
    CanonicalJitCompilerPipeline,
    CompilerBridge,
    CompilerJitArtifact,
    JitArtifactPayload,
};

// =============================================================================
// CODE CACHE PUBLIC API
// =============================================================================

pub use code_cache::{
    CacheEvictionPolicy,
    InMemoryJitCodeCache,
    JitCodeCacheConfig,
    JitCodeCacheStatistics,
};

// =============================================================================
// VERIFICATION PUBLIC API
// =============================================================================

pub use verification::{
    JitArtifactVerificationAuthority,
    JitVerifier,
    VerificationEvidence,
    VerificationReceipt,
    VerificationSubject,
};

// =============================================================================
// PROFILING PUBLIC API
// =============================================================================

pub use profiling::{
    JitInvocationOutcome,
    JitInvocationToken,
    JitProfileCollector,
    JitProfileConfig,
    JitProfileEntrySnapshot,
    JitProfileError,
    JitProfileObservation,
    JitProfileResult,
    JitProfileSnapshot,
    JitSamplingPolicy,
    ProfileOverflowPolicy,
    JIT_PROFILING_API_VERSION,
};

// =============================================================================
// HOTNESS PUBLIC API
// =============================================================================

pub use hotness::{
    HotnessDecision,
    HotnessMatchMode,
    HotnessPolicy,
    HotnessReason,
    HotnessStatus,
    HotnessTracker,
    HotnessWindowMetrics,
    JIT_HOTNESS_API_VERSION,
};

// =============================================================================
// SPECIALIZATION PUBLIC API
// =============================================================================

pub use specialization::{
    JIT_SPECIALIZATION_API_VERSION,
    PreparedSpecialization,
    SpecializationAuthority,
    SpecializationCandidate,
    SpecializationCertificate,
    SpecializationCoordinator,
    SpecializationEvidence,
};

// =============================================================================
// DEOPTIMIZATION PUBLIC API
// =============================================================================

pub use deoptimization::{
    BaselineArtifactResolver,
    BaselineHandoffExecutor,
    BaselineHandoffReceipt,
    DeoptimizationAuthority,
    DeoptimizationCoordinator,
    DeoptimizationEvidence,
    DeoptimizationOutcome,
    DeoptimizationReason,
    DeoptimizationRequest,
    DeoptimizationState,
    ValidatedDeoptimization,
    JIT_DEOPTIMIZATION_API_VERSION,
};

// =============================================================================
// INVALIDATION PUBLIC API
// =============================================================================

pub use invalidation::{
    InvalidationReason,
    InvalidationSubject,
    JitInvalidationCoordinator,
    JitInvalidationPublication,
    JitInvalidationReport,
    JitInvalidationSnapshot,
    JIT_INVALIDATION_API_VERSION,
};
