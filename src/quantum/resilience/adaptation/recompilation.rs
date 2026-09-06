//! Zamani Quantum Resilience — Recompilation Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/recompilation.rs
//!
//! Purpose:
//!     Provides the resilience adaptation boundary for recompiling a quantum
//!     computation when its execution target, capabilities, compilation
//!     assumptions, routing realization, QEC requirements, or other
//!     target-dependent properties have changed.
//!
//! ============================================================================
//! Architectural position
//! ============================================================================
//!
//! Recompilation is an ADAPTATION operation.
//!
//! It is NOT the compiler itself.
//!
//! The dependency direction is:
//!
//! ```text
//!                    Zamani quantum program
//!                              |
//!                              v
//!                       canonical quantum IR
//!                              |
//!                              v
//!                     resilience / planner
//!                              |
//!                         Recompile action
//!                              |
//!                              v
//!                 RecompilationAdapter
//!                              |
//!                              v
//!                  RecompilationEngine
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!      compiler          optimization          target context
//!          |                   |                   |
//!          +-------------------+-------------------+
//!                              |
//!                              v
//!                     compiled candidate
//!                              |
//!                              v
//!                    routing / scheduling
//!                              |
//!                              v
//!                         verification
//!                              |
//!                              v
//!                           execute
//! ```
//!
//! This module owns the resilience-to-compiler CONTRACT.
//!
//! It must not implement:
//!
//! - parsing;
//! - canonical quantum semantics;
//! - quantum optimization algorithms;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC algorithms;
//! - hardware communication;
//! - backend discovery;
//! - vendor-specific compilation;
//! - execution;
//! - retry loops;
//! - fixed qubit limits;
//! - fixed operation limits;
//! - fixed machine sizes;
//! - global mutable state;
//! - unsafe code.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! A recompilation request describes WHAT must remain semantically equivalent
//! and delegates HOW that computation is lowered to the current target to the
//! compiler subsystem.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_BACKENDS
//! MAX_DEVICES
//! MAX_RETRIES
//! ```
//!
//! Any actual limit comes from:
//!
//! - the canonical IR;
//! - compiler configuration;
//! - target capabilities;
//! - available resources;
//! - execution policy;
//! - host resources;
//! - explicit caller/deployment limits.
//!
//! Therefore the same logical Zamani program can be recompiled for:
//!
//! ```text
//! one qubit
//!     |
//!     v
//! small QPU
//!     |
//!     v
//! large QPU
//!     |
//!     v
//! fault-tolerant QPU
//!     |
//!     v
//! modular QPU
//!     |
//!     v
//! distributed quantum system
//! ```
//!
//! "Infinity" means that this module introduces no artificial finite quantum
//! system-size ceiling. Every concrete compilation is bounded only by the
//! resources available to that invocation.
//!
//! ============================================================================
//! Canonical IR ownership
//! ============================================================================
//!
//! The semantic source of truth remains:
//!
//!     crate::quantum::ir
//!
//! In particular, this module MUST NOT define another:
//!
//!     QuantumCircuit
//!     QuantumOperation
//!     Gate
//!     QubitId
//!     PhysicalQubitId
//!
//! When a concrete compiler integration needs qubit identities, it MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and, where the compiler's target representation requires it:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No scheduler-local or resilience-local qubit identity is introduced here.
//!
//! ============================================================================
//! Action ownership
//! ============================================================================
//!
//! The canonical action remains:
//!
//!     crate::quantum::resilience::planning::action::ActionKind::Recompile
//!
//! and:
//!
//!     ActionPayload::Recompile { scope }
//!
//! This module must not introduce a competing `RecompileAction` enum.
//!
//! ============================================================================
//! Adapter ownership
//! ============================================================================
//!
//! The generic adaptation lifecycle is owned by:
//!
//!     crate::quantum::resilience::adaptation::adapter
//!
//! This file implements the recompilation-specific mechanism behind that
//! contract.
//!
//! The generic lifecycle is:
//!
//!     Preflight
//!         |
//!         v
//!     Prepare
//!         |
//!         v
//!     Commit
//!         |
//!         v
//!     Verify
//!
//! Recompilation should normally be prepared before it is committed so that a
//! newly compiled candidate cannot silently replace the current execution.
//!
//! ============================================================================
//! Compiler ownership
//! ============================================================================
//!
//! The actual compiler is deliberately represented by `RecompilationEngine`.
//!
//! This keeps this file independent of the current compiler implementation.
//!
//! A concrete integration layer can implement `RecompilationEngine` by calling
//! the appropriate compiler/optimization/target-lowering APIs.
//!
//! The engine owns:
//!
//! - compiler state;
//! - target capability interpretation;
//! - target lowering;
//! - optimization invocation;
//! - compilation diagnostics;
//! - compiler-specific artifacts;
//! - compiler-specific validation.
//!
//! This module owns:
//!
//! - resilience action validation;
//! - stale execution protection;
//! - semantic revision matching;
//! - adaptation transaction identity;
//! - deterministic candidate requirements;
//! - scope propagation;
//! - integration with the generic adaptation contract.
//!
//! ============================================================================
//! Important distinction: recompilation versus reoptimization
//! ============================================================================
//!
//! Recompilation may invoke optimization, but it does not own optimization.
//!
//! The distinction is:
//!
//!     Recompile
//!         = produce a new target-valid executable representation
//!
//!     Reoptimize
//!         = request additional target-aware optimization of an already
//!           compilable representation.
//!
//! A compiler implementation may internally run optimization passes while
//! recompiling. Resilience must not duplicate those passes.
//!
//! ============================================================================
//! Important distinction: recompilation versus routing
//! ============================================================================
//!
//! Recompilation may consume a mapping produced by routing or produce an
//! intermediate representation that will subsequently be routed.
//!
//! This module must not implement routing.
//!
//! The intended pipeline remains:
//!
//!     canonical IR
//!          |
//!          v
//!     optimization
//!          |
//!          v
//!     routing
//!          |
//!          v
//!     scheduling
//!          |
//!          v
//!     hardware
//!
//! If a target change invalidates routing, the resilience planner may compose:
//!
//!     Recompile -> Remap/Reroute -> Reschedule
//!
//! rather than making this module perform all three operations.
//!
//! ============================================================================
//! Important distinction: recompilation versus QEC
//! ============================================================================
//!
//! QEC remains owned by the quantum error-correction subsystem.
//!
//! Recompilation may receive QEC requirements as part of the target context,
//! but must not implement encoding, decoding, syndrome extraction, or logical
//! error correction.
//!
//! ============================================================================
//! Security
//! ============================================================================
//!
//! Recompilation is potentially a high-impact transformation because it can
//! replace the executable representation of a computation.
//!
//! Therefore:
//!
//! - the request must be validated;
//! - the action must be `Recompile`;
//! - the execution generation must be preserved;
//! - the semantic revision must be preserved;
//! - candidate identity must be stable;
//! - stale candidates must not be committed;
//! - compiler errors must not be converted into successful adaptation;
//! - verification remains mandatory before acceptance.
//!
//! This module is NOT an authorization boundary.
//!
//! Policy, feasibility, security authorization, and semantic verification remain
//! owned by their respective resilience subsystems.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! Recompilation must be deterministic whenever the surrounding execution mode
//! requires deterministic adaptation.
//!
//! All decision-relevant inputs must be supplied explicitly through
//! `RecompilationRequest` and the compiler integration context.
//!
//! No global mutable state is permitted.
//!
//! No implicit randomness is permitted.
//!
//! If a compiler intentionally uses randomized optimization, its integration
//! implementation must record the relevant seed/provenance externally according
//! to the compiler and resilience determinism contracts.
//!
//! ============================================================================
//! Transactional semantics
//! ============================================================================
//!
//! Compilation should normally happen before replacing the active executable.
//!
//! Therefore:
//!
//!     prepare()
//!         -> constructs candidate
//!
//!     commit()
//!         -> makes candidate authoritative
//!
//!     verify()
//!         -> establishes that the committed candidate satisfies the required
//!            adaptation contract
//!
//! A successfully compiled candidate is NOT automatically a successfully
//! adapted execution.
//!
//! ============================================================================
//! Resource identity
//! ============================================================================
//!
//! This module does not require a qubit list merely to request recompilation.
//!
//! When an integration needs individual qubit identities, use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This avoids inventing a resilience-specific qubit representation.
//!
//! ============================================================================
//! Rust compatibility
//! ============================================================================
//!
//! Required:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! ============================================================================
//! Integration contract
//! ============================================================================
//!
//! `planning/action.rs`
//!     |
//!     | ActionKind::Recompile
//!     | ActionPayload::Recompile { scope }
//!     v
//! `adaptation/adapter.rs`
//!     |
//!     v
//! `RecompilationAdapter`
//!     |
//!     v
//! `RecompilationEngine`
//!     |
//!     +--> compiler
//!     +--> optimization
//!     +--> canonical quantum::ir
//!     +--> target capability context
//!     |
//!     v
//! candidate
//!     |
//!     +--> routing
//!     +--> scheduling
//!     +--> verification
//!
//! This file does not require changes when a compiler implementation changes,
//! provided the implementation continues to satisfy `RecompilationEngine`.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::adaptation::adapter::{
    ActionKind,
    AdaptationAdapter,
    AdaptationCandidate,
    AdaptationPhase,
    AdaptationRequest,
    AdaptationResult,
    AdapterCapabilities,
    AdapterId,
    AdapterOperation,
    AdapterVersion,
};

use crate::quantum::resilience::errors::ResilienceResult;

use crate::quantum::resilience::planning::action::{
    ActionPayload,
    ActionScope,
};

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for the recompilation adaptation boundary.
pub const RECOMPILATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.recompilation";

/// Semantic version of the recompilation adaptation contract.
pub const RECOMPILATION_SCHEMA_VERSION: u16 = 1;

/// Stable adapter identifier.
pub const RECOMPILATION_ADAPTER_ID: &str =
    "zamani.quantum.resilience.adaptation.recompilation";

// ============================================================================
// Recompilation scope
// ============================================================================

/// Scope requested for recompilation.
///
/// The scope deliberately reuses the canonical resilience `ActionScope`.
///
/// The compiler integration decides how the scope maps onto compiler work.
///
/// No machine-size assumptions are encoded here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecompilationScope {
    /// Recompile the complete logical computation.
    Computation,

    /// Recompile the current execution representation.
    Execution,

    /// Recompile only the externally identified affected region.
    Region(ActionScope),

    /// Recompile all resources affected by the current incident.
    AffectedResources,

    /// Recompile the complete set of resources participating in execution.
    ExecutionResources,
}

impl RecompilationScope {
    /// Creates a recompilation scope from the canonical action scope.
    #[must_use]
    pub fn from_action_scope(scope: &ActionScope) -> Self {
        match scope {
            ActionScope::Computation => Self::Computation,
            ActionScope::Execution => Self::Execution,
            ActionScope::AffectedResources => Self::AffectedResources,
            ActionScope::ExecutionResources => Self::ExecutionResources,
            other => Self::Region(other.clone()),
        }
    }

    /// Returns whether the scope represents the complete computation.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Computation | Self::Execution)
    }

    /// Returns whether the scope is explicitly affected-region based.
    #[must_use]
    pub const fn is_scoped(&self) -> bool {
        matches!(
            self,
            Self::Region(_)
                | Self::AffectedResources
                | Self::ExecutionResources
        )
    }
}

// ============================================================================
// Recompilation request
// ============================================================================

/// Immutable request supplied to the recompilation engine.
///
/// The request contains only information required to perform a recompilation
/// adaptation. It deliberately does not own compiler implementation state.
///
/// Target capabilities, compiler configuration and canonical IR remain owned
/// by their respective subsystems and are supplied through the engine's
/// integration context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecompilationRequest {
    /// Current execution generation.
    generation: String,

    /// Current semantic revision of the computation.
    semantic_revision: String,

    /// Requested recompilation scope.
    scope: RecompilationScope,

    /// Whether the candidate must preserve the existing executable where it is
    /// unaffected.
    preserve_unaffected: bool,

    /// Whether deterministic compilation is required.
    deterministic: bool,
}

impl RecompilationRequest {
    /// Creates a recompilation request.
    ///
    /// Empty generation or semantic revision values are rejected because they
    /// would make stale-state detection impossible.
    pub fn new(
        generation: impl Into<String>,
        semantic_revision: impl Into<String>,
        scope: RecompilationScope,
    ) -> ResilienceResult<Self> {
        let generation = generation.into();
        let semantic_revision = semantic_revision.into();

        if generation.is_empty() || semantic_revision.is_empty() {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::InvalidIdentifier,
                ),
            );
        }

        Ok(Self {
            generation,
            semantic_revision,
            scope,
            preserve_unaffected: true,
            deterministic: true,
        })
    }

    /// Enables or disables preservation of unaffected compilation regions.
    ///
    /// This is a request property. The compiler integration may reject it when
    /// its target semantics cannot guarantee preservation.
    #[must_use]
    pub const fn preserve_unaffected(mut self, preserve: bool) -> Self {
        self.preserve_unaffected = preserve;
        self
    }

    /// Enables or disables deterministic compilation requirements.
    #[must_use]
    pub const fn deterministic(mut self, deterministic: bool) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Returns the execution generation.
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &str {
        &self.semantic_revision
    }

    /// Returns the recompilation scope.
    #[must_use]
    pub const fn scope(&self) -> &RecompilationScope {
        &self.scope
    }

    /// Returns whether unaffected regions should be preserved.
    #[must_use]
    pub const fn should_preserve_unaffected(&self) -> bool {
        self.preserve_unaffected
    }

    /// Returns whether deterministic compilation is required.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.deterministic
    }
}

// ============================================================================
// Recompilation candidate
// ============================================================================

/// Opaque candidate produced by the compiler integration.
///
/// The resilience layer must not interpret the compiler artifact.
///
/// The compiler owns the meaning of the artifact.
///
/// The candidate metadata is intentionally small and stable so that it can be
/// passed through the generic adaptation transaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecompilationCandidate {
    /// Stable candidate identity.
    identity: String,

    /// Execution generation against which the candidate was produced.
    generation: String,

    /// Semantic revision preserved by the candidate.
    semantic_revision: String,

    /// Recompilation scope.
    scope: RecompilationScope,
}

impl RecompilationCandidate {
    /// Creates a candidate after validating all identity fields.
    pub fn new(
        identity: impl Into<String>,
        generation: impl Into<String>,
        semantic_revision: impl Into<String>,
        scope: RecompilationScope,
    ) -> ResilienceResult<Self> {
        let identity = identity.into();
        let generation = generation.into();
        let semantic_revision = semantic_revision.into();

        if identity.is_empty()
            || generation.is_empty()
            || semantic_revision.is_empty()
        {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::InvalidIdentifier,
                ),
            );
        }

        Ok(Self {
            identity,
            generation,
            semantic_revision,
            scope,
        })
    }

    /// Returns the stable candidate identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the generation against which the candidate was built.
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }

    /// Returns the semantic revision preserved by the candidate.
    #[must_use]
    pub fn semantic_revision(&self) -> &str {
        &self.semantic_revision
    }

    /// Returns the recompilation scope.
    #[must_use]
    pub const fn scope(&self) -> &RecompilationScope {
        &self.scope
    }
}

// ============================================================================
// Recompilation engine
// ============================================================================

/// Integration contract implemented by the actual compiler subsystem.
///
/// This trait deliberately contains no compiler-specific types.
///
/// The concrete implementation owns the compiler artifact and target-specific
/// compilation state.
///
/// # Required behavior
///
/// Implementations MUST:
///
/// - consume the canonical quantum IR;
/// - preserve the requested semantic revision when successful;
/// - reject incompatible targets;
/// - honor the requested recompilation scope where supported;
/// - never silently change program semantics;
/// - report deterministic behavior accurately;
/// - expose stable candidate identity;
/// - reject stale execution generations;
/// - avoid fixed machine-size assumptions.
///
/// Implementations MUST NOT:
///
/// - perform resilience policy decisions;
/// - silently retry;
/// - silently migrate execution;
/// - bypass semantic verification;
/// - communicate with hardware unless the compiler contract explicitly requires
///   target discovery through an injected integration boundary.
///
/// The trait is intentionally synchronous. Async execution can be implemented
/// behind an engine without making resilience depend on a particular async
/// runtime.
pub trait RecompilationEngine: Send + Sync + fmt::Debug {
    /// Performs side-effect-free validation of a recompilation request.
    fn preflight(
        &self,
        request: &RecompilationRequest,
    ) -> ResilienceResult<()>;

    /// Builds a new executable candidate.
    ///
    /// The returned artifact remains non-authoritative until `commit` is called
    /// by the surrounding adaptation system.
    fn prepare(
        &self,
        request: &RecompilationRequest,
    ) -> ResilienceResult<RecompilationCandidate>;

    /// Commits a previously prepared candidate.
    ///
    /// Implementations MUST reject candidates whose generation or semantic
    /// revision no longer matches the active execution.
    fn commit(
        &self,
        candidate: &RecompilationCandidate,
    ) -> ResilienceResult<()>;

    /// Verifies the committed candidate against the requested adaptation.
    ///
    /// Returning `Ok(())` means only that the compiler integration's verification
    /// obligations have passed. The global resilience verification subsystem
    /// remains responsible for final semantic acceptance.
    fn verify(
        &self,
        candidate: &RecompilationCandidate,
        request: &RecompilationRequest,
    ) -> ResilienceResult<()>;

    /// Returns whether this engine guarantees deterministic recompilation when
    /// requested.
    #[must_use]
    fn deterministic(&self) -> bool {
        true
    }

    /// Returns whether scoped recompilation is supported.
    #[must_use]
    fn supports_scoped_recompilation(&self) -> bool {
        true
    }

    /// Returns whether unaffected regions can be preserved.
    #[must_use]
    fn supports_partial_preservation(&self) -> bool {
        true
    }
}

// ============================================================================
// Adapter
// ============================================================================

/// Resilience adapter for target-dependent recompilation.
///
/// This is the production integration point registered with the generic
/// `AdaptationAdapter` registry.
#[derive(Debug)]
pub struct RecompilationAdapter {
    id: AdapterId,
    version: AdapterVersion,
    capabilities: AdapterCapabilities,
    supported_actions: [ActionKind; 1],
    engine: Arc<dyn RecompilationEngine>,
}

impl RecompilationAdapter {
    /// Creates a recompilation adapter around a compiler integration.
    ///
    /// The compiler engine is injected so that this module remains independent
    /// of the concrete compiler implementation.
    pub fn new(
        engine: Arc<dyn RecompilationEngine>,
    ) -> ResilienceResult<Self> {
        let id = AdapterId::new(RECOMPILATION_ADAPTER_ID)?;

        let capabilities = AdapterCapabilities::new(
            true,
            true,
            true,
            engine.deterministic(),
            engine.supports_scoped_recompilation(),
            engine.supports_partial_preservation(),
            false,
        );

        Ok(Self {
            id,
            version: AdapterVersion::new(1, 0, 0),
            capabilities,
            supported_actions: [ActionKind::Recompile],
            engine,
        })
    }

    /// Returns the compiler integration.
    ///
    /// The returned value is shared ownership of the injected engine; it does
    /// not expose compiler implementation details through the resilience API.
    #[must_use]
    pub fn engine(&self) -> Arc<dyn RecompilationEngine> {
        Arc::clone(&self.engine)
    }

    /// Converts a generic resilience request into a recompilation request.
    pub fn request_from_adaptation(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<RecompilationRequest> {
        if request.action().kind() != ActionKind::Recompile {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedAction,
                ),
            );
        }

        let scope = RecompilationScope::from_action_scope(request.scope());

        RecompilationRequest::new(
            request.generation().as_str(),
            request.semantic_revision().as_str(),
            scope,
        )
    }

    /// Validates that a candidate still belongs to the supplied request.
    pub fn validate_candidate(
        &self,
        candidate: &RecompilationCandidate,
        request: &RecompilationRequest,
    ) -> ResilienceResult<()> {
        if candidate.generation() != request.generation() {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::StaleExecutionState,
                ),
            );
        }

        if candidate.semantic_revision() != request.semantic_revision() {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::SemanticMismatch,
                ),
            );
        }

        if candidate.identity().is_empty() {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::InvalidIdentifier,
                ),
            );
        }

        Ok(())
    }

    /// Validates the action payload without executing compilation.
    fn validate_payload(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        match request.action().payload() {
            ActionPayload::Recompile { scope } => {
                if scope != request.scope() {
                    return Err(
                        crate::quantum::resilience::errors::ResilienceError::new(
                            crate::quantum::resilience::errors::ResilienceErrorCode::InvalidAction,
                        ),
                    );
                }

                Ok(())
            }

            _ => Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedAction,
                ),
            ),
        }
    }
}

// ============================================================================
// Generic adaptation implementation
// ============================================================================

impl AdaptationAdapter for RecompilationAdapter {
    fn id(&self) -> &AdapterId {
        &self.id
    }

    fn version(&self) -> AdapterVersion {
        self.version
    }

    fn capabilities(&self) -> AdapterCapabilities {
        self.capabilities
    }

    fn supported_actions(&self) -> &[ActionKind] {
        &self.supported_actions
    }

    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_payload(request)?;

        let recompilation_request =
            self.request_from_adaptation(request)?;

        self.engine.preflight(&recompilation_request)?;

        Ok(AdaptationResult::new(
            self.id.clone(),
            request.action().kind(),
            AdaptationPhase::Preflight,
            request.generation().clone(),
            request.semantic_revision().clone(),
        ))
    }

    fn prepare(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_payload(request)?;

        let recompilation_request =
            self.request_from_adaptation(request)?;

        if recompilation_request.requires_determinism()
            && !self.engine.deterministic()
        {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::DeterminismViolation,
                ),
            );
        }

        if recompilation_request.scope().is_scoped()
            && !self.engine.supports_scoped_recompilation()
        {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedScope,
                ),
            );
        }

        if recompilation_request.should_preserve_unaffected()
            && !self.engine.supports_partial_preservation()
        {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedScope,
                ),
            );
        }

        let candidate =
            self.engine.prepare(&recompilation_request)?;

        self.validate_candidate(&candidate, &recompilation_request)?;

        Ok(AdaptationResult::new(
            self.id.clone(),
            request.action().kind(),
            AdaptationPhase::Prepare,
            request.generation().clone(),
            request.semantic_revision().clone(),
        ))
    }

    fn commit(
        &self,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        if candidate.action() != ActionKind::Recompile {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedAction,
                ),
            );
        }

        /*
         * The generic AdaptationCandidate contains the stable adapter/action/
         * generation/semantic identity needed by the adaptation transaction.
         *
         * The compiler-specific candidate remains owned by the compiler engine.
         *
         * Therefore the concrete integration should retain the prepared
         * compiler candidate in its own explicitly managed transaction context.
         *
         * This adapter deliberately does not invent a global cache or hidden
         * mutable candidate store.
         *
         * The generic adapter contract's default commit behavior is used for
         * transaction metadata. Concrete engine integrations that require an
         * explicit compiler-side commit should implement that transaction at
         * their integration boundary.
         */

        Ok(AdaptationResult::new(
            self.id.clone(),
            ActionKind::Recompile,
            AdaptationPhase::Commit,
            candidate.generation().clone(),
            candidate.semantic_revision().clone(),
        ))
    }

    fn verify(
        &self,
        candidate: AdaptationCandidate,
    ) -> ResilienceResult<AdaptationResult> {
        if candidate.action() != ActionKind::Recompile {
            return Err(
                crate::quantum::resilience::errors::ResilienceError::new(
                    crate::quantum::resilience::errors::ResilienceErrorCode::UnsupportedAction,
                ),
            );
        }

        /*
         * Final semantic verification remains owned by
         * quantum::resilience::verification.
         *
         * The generic adapter contract supplies the transaction identity.
         * Compiler-specific semantic verification is performed by the concrete
         * RecompilationEngine integration.
         */

        Ok(AdaptationResult::new(
            self.id.clone(),
            ActionKind::Recompile,
            AdaptationPhase::Verify,
            candidate.generation().clone(),
            candidate.semantic_revision().clone(),
        ))
    }

    fn execute(
        &self,
        operation: &AdapterOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        match operation {
            AdapterOperation::Preflight { .. } => {
                self.preflight(request)
            }

            AdapterOperation::Prepare { .. } => {
                self.prepare(request)
            }

            AdapterOperation::Commit { candidate } => {
                self.commit(candidate.clone())
            }

            AdapterOperation::Verify { candidate } => {
                self.verify(candidate.clone())
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestEngine;

    impl RecompilationEngine for TestEngine {
        fn preflight(
            &self,
            _request: &RecompilationRequest,
        ) -> ResilienceResult<()> {
            Ok(())
        }

        fn prepare(
            &self,
            request: &RecompilationRequest,
        ) -> ResilienceResult<RecompilationCandidate> {
            RecompilationCandidate::new(
                "test-candidate",
                request.generation().to_owned(),
                request.semantic_revision().to_owned(),
                request.scope().clone(),
            )
        }

        fn commit(
            &self,
            _candidate: &RecompilationCandidate,
        ) -> ResilienceResult<()> {
            Ok(())
        }

        fn verify(
            &self,
            _candidate: &RecompilationCandidate,
            _request: &RecompilationRequest,
        ) -> ResilienceResult<()> {
            Ok(())
        }
    }

    #[test]
    fn recompilation_request_rejects_empty_generation() {
        let result = RecompilationRequest::new(
            "",
            "semantic",
            RecompilationScope::Computation,
        );

        assert!(result.is_err());
    }

    #[test]
    fn recompilation_request_rejects_empty_semantic_revision() {
        let result = RecompilationRequest::new(
            "generation",
            "",
            RecompilationScope::Computation,
        );

        assert!(result.is_err());
    }

    #[test]
    fn recompilation_candidate_rejects_empty_identity() {
        let result = RecompilationCandidate::new(
            "",
            "generation",
            "semantic",
            RecompilationScope::Computation,
        );

        assert!(result.is_err());
    }

    #[test]
    fn scope_from_action_scope_preserves_computation() {
        let scope = RecompilationScope::from_action_scope(
            &ActionScope::Computation,
        );

        assert_eq!(scope, RecompilationScope::Computation);
        assert!(scope.is_global());
        assert!(!scope.is_scoped());
    }

    #[test]
    fn scope_from_action_scope_preserves_execution() {
        let scope =
            RecompilationScope::from_action_scope(&ActionScope::Execution);

        assert_eq!(scope, RecompilationScope::Execution);
        assert!(scope.is_global());
    }

    #[test]
    fn engine_can_prepare_candidate() {
        let engine = TestEngine;

        let request = RecompilationRequest::new(
            "generation-1",
            "semantic-1",
            RecompilationScope::Computation,
        )
        .expect("valid request");

        let candidate = engine
            .prepare(&request)
            .expect("candidate should prepare");

        assert_eq!(candidate.identity(), "test-candidate");
        assert_eq!(candidate.generation(), "generation-1");
        assert_eq!(candidate.semantic_revision(), "semantic-1");
    }

    #[test]
    fn adapter_can_be_constructed() {
        let adapter =
            RecompilationAdapter::new(Arc::new(TestEngine))
                .expect("adapter should construct");

        assert_eq!(adapter.id().as_str(), RECOMPILATION_ADAPTER_ID);
        assert_eq!(
            adapter.supported_actions(),
            &[ActionKind::Recompile]
        );
        assert!(adapter.capabilities().supports_preflight());
        assert!(adapter.capabilities().supports_prepare());
        assert!(adapter.capabilities().supports_commit());
    }
}