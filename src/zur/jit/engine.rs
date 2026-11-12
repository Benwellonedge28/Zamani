
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Engine
//!
//! Path: src/zur/jit/engine.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Coordinates just-in-time compilation requests for the Zamani Universal
//! Runtime (ZUR).
//!
//! This module is the orchestration boundary between:
//!
//!     validated compiler representation
//!                 |
//!                 v
//!          JIT engine
//!                 |
//!       +---------+---------+
//!       |         |         |
//!       v         v         v
//!    compiler  verifier   code cache
//!       |         |         |
//!       +---------+---------+
//!                 |
//!                 v
//!        verified JIT artifact
//!                 |
//!                 v
//!       runtime execution dispatch
//!
//! The engine does not implement a second compiler.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - JIT request orchestration.
//! - JIT activation and strategy checks.
//! - Compilation lifecycle coordination.
//! - Cache lookup and publication coordination.
//! - Artifact identity checks.
//! - Verification-before-publication.
//! - Cancellation checkpoints.
//! - Explicit AOT fallback eligibility.
//! - JIT compilation outcomes.
//! - JIT engine dependency injection.
//!
//! It does NOT own:
//!
//! - Source parsing.
//! - Grammar interpretation.
//! - AST construction.
//! - Semantic analysis.
//! - Compiler IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Backend registration.
//! - Backend selection.
//! - Machine-code generation.
//! - Executable memory management.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Runtime scheduling.
//! - Task execution.
//! - Domain-specific execution.
//! - Artifact manifest construction.
//! - Artifact serialization.
//! - Runtime execution results.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ============================================================================
//! EXISTING REPOSITORY INTEGRATION
//! ============================================================================
//!
//! Canonical configuration:
//!
//!     crate::zur::config
//!         RuntimeConfig
//!         JitPolicy
//!         ExecutionFallbackPolicy
//!
//! Canonical operational policy:
//!
//!     crate::zur::policy
//!         RuntimePolicy
//!         ExecutionStrategyPreference
//!
//! Canonical errors:
//!
//!     crate::zur::error
//!         ZurError
//!         ZurResult
//!
//! Compiler integration:
//!
//!     crate::compiler
//!     crate::compiler::ir
//!     crate::compiler::optimization
//!     crate::compiler::passes
//!     crate::compiler::artifacts
//!
//! Backend integration:
//!
//!     crate::backend
//!
//! Runtime integration:
//!
//!     crate::zur::execution::dispatch
//!     crate::zur::execution::artifact
//!     crate::zur::execution::loader
//!     crate::zur::security
//!     crate::zur::observability
//!
//! The compiler bridge, verifier and cache are explicitly injected.
//! This avoids assuming APIs that have not yet been implemented.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! Relevant source grammar:
//!
//!     grammar/execution/runtime.g4
//!     grammar/execution/runtime-capabilities.g4
//!     grammar/compile/optimization.g4
//!     grammar/resources/portability.g4
//!
//! Relevant specifications:
//!
//!     grammar/spec/portability.md
//!     grammar/specification/execution-model.md
//!     grammar/specification/semantic-model.md
//!
//! The frontend and semantic layer interpret source-level declarations.
//!
//! The compiler establishes validated optimization and execution inputs.
//!
//! ZUR receives those established contracts.
//!
//! This module never parses grammar or interprets AST nodes.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! JIT compilation must preserve the semantic meaning of the program.
//!
//! A JIT specialization is valid only when its compiler and verification
//! authorities establish compatibility with the original execution
//! contract.
//!
//! Different machines may produce different compiled artifacts while
//! preserving the same program semantics.
//!
//! No arbitrary hardware, device, domain, task, memory or provider limits
//! are imposed here.
//!
//! Literal infinite physical computation is not promised.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No fabricated compilation success.
//! - No fabricated execution success.
//! - No unverified artifact publication.
//! - No automatic semantic degradation.
//! - No implicit hardware discovery.
//! - No implicit backend selection.
//!
//! Executable memory and native code execution are delegated to explicitly
//! supplied implementations that must satisfy the surrounding safe API
//! contracts.
//!
//! ============================================================================
//! INTEGRATION INVARIANTS
//! ============================================================================
//!
//! 1. Compiler-owned representations remain compiler-owned.
//! 2. A JIT request is not proof of JIT availability.
//! 3. A cache hit is not proof of artifact validity.
//! 4. Successful compilation is not proof of successful execution.
//! 5. A fallback preference is not proof of fallback compatibility.
//! 6. A verifier must approve an artifact before publication.
//! 7. Cache identities must include all compatibility-relevant inputs.
//! 8. Cancellation must be checked at compilation boundaries.
//! 9. Runtime configuration cannot bypass security verification.
//! 10. JIT must remain optional.
//!
//! ============================================================================

use std::fmt;
use std::sync::Arc;

use crate::zur::config::{
    ExecutionFallbackPolicy,
    JitPolicy,
    RuntimeConfig,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::policy::{
    ExecutionStrategyPreference,
    RuntimePolicy,
};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

const COMPONENT: &str = "zur.jit.engine";

// =============================================================================
// IDENTIFIER VALIDATION
// =============================================================================

/// Validates a namespaced domain identifier.
///
/// Format:
///
///     namespace.segment.identifier
///
/// Allowed characters:
///
/// - ASCII lowercase letters.
/// - ASCII digits.
/// - Hyphens.
/// - Underscores.
/// - Dots separating nonempty segments.
///
/// There is no arbitrary identifier-length or namespace-depth ceiling.
///
/// This validation establishes identifier structure only.
/// It does not establish that a domain is supported or executable.
fn validate_domain_identifier(value: &str) -> ZurResult<()> {
    if value.is_empty() {
        return Err(engine_error(
            "domain identifier must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(engine_error(
            "domain identifier contains surrounding whitespace",
        ));
    }

    let mut count = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(engine_error(
                "domain identifier contains an empty namespace segment",
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(engine_error(
                "domain identifier contains an invalid character",
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(engine_error(
                "domain identifier has an invalid segment boundary",
            ));
        }

        count += 1;
    }

    if count < 2 {
        return Err(engine_error(
            "domain identifier must contain a namespace",
        ));
    }

    Ok(())
}

/// Creates a structured JIT engine configuration error.
fn engine_error(message: impl Into<String>) -> ZurError {
    ZurError::configuration(message)
        .with_component(COMPONENT)
}

/// Creates a structured JIT compatibility error.
fn compatibility_error(message: impl Into<String>) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_artifact")
}

/// Creates a structured JIT cancellation error.
fn cancellation_error() -> ZurError {
    ZurError::cancelled(
        "JIT compilation was cancelled",
    )
    .with_component(COMPONENT)
    .with_operation("compile")
}

// =============================================================================
// COMPILATION UNIT
// =============================================================================

/// Opaque reference to a compiler-owned compilation unit.
///
/// Implementations must wrap an existing canonical compiler
/// representation or an explicitly validated compiler input.
///
/// Examples include adapters around:
///
/// - Classical compiler IR.
/// - Quantum IR.
/// - HDL IR.
/// - ZUIR.
/// - AI/tensor compiler representations.
///
/// This trait does not define another intermediate representation.
///
/// The JIT engine cannot inspect or reinterpret the underlying IR.
pub trait JitCompilationUnit: fmt::Debug + Send + Sync {
    /// Returns the stable identity of the compiler-owned unit.
    ///
    /// This identity must remain stable for the lifetime of the unit.
    fn stable_identity(&self) -> &str;

    /// Returns the canonical namespaced computational-domain identity.
    ///
    /// This is an extensible identifier, not a closed domain enum.
    fn domain_identity(&self) -> &str;

    /// Returns the compiler contract version associated with the unit.
    fn compiler_contract_version(&self) -> &str;
}

// =============================================================================
// CACHE IDENTITY
// =============================================================================

/// Stable identity for one JIT compilation result.
///
/// The identity is supplied by the compiler integration.
///
/// It must account for every compilation input that can affect
/// artifact correctness or compatibility, including as applicable:
///
/// - Compiler representation identity.
/// - Source semantic identity.
/// - Compiler version.
/// - Optimization pipeline identity.
/// - Optimization configuration.
/// - Target compatibility identity.
/// - Runtime ABI version.
/// - Domain contract version.
/// - Code-generation provider identity.
/// - Specialization assumptions.
///
/// The JIT engine deliberately does not invent a hashing algorithm.
///
/// The compiler bridge must supply a collision-resistant canonical
/// identity appropriate to its artifact and cache implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JitCacheKey(String);

impl JitCacheKey {
    /// Constructs a cache key from a canonical compiler fingerprint.
    ///
    /// Empty or whitespace-padded identifiers are rejected.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(engine_error(
                "JIT cache key must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(engine_error(
                "JIT cache key contains surrounding whitespace",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical cache-key representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// COMPILATION REASON
// =============================================================================

/// Describes why a JIT compilation was requested.
///
/// These values describe operational compilation requests.
///
/// They do not determine the semantic meaning of a program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum JitCompilationReason {
    /// Initial baseline JIT compilation.
    Baseline,

    /// Compilation of a frequently executed region.
    ///
    /// Hotness thresholds belong to the profiling and adaptation
    /// subsystems, not this engine.
    HotPath,

    /// Compilation using explicitly supplied specialization assumptions.
    Specialization,

    /// Recompilation following an invalidation or deoptimization event.
    Recompilation,
}

// =============================================================================
// COMPILATION REQUEST
// =============================================================================

/// An immutable JIT compilation request.
///
/// The request carries an opaque compiler-owned unit.
///
/// It does not carry a duplicate AST, semantic model, IR or hardware
/// inventory.
pub struct JitCompilationRequest {
    unit: Arc<dyn JitCompilationUnit>,

    cache_key: JitCacheKey,

    reason: JitCompilationReason,
}

impl fmt::Debug for JitCompilationRequest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("JitCompilationRequest")
            .field(
                "unit_identity",
                &self.unit.stable_identity(),
            )
            .field(
                "domain_identity",
                &self.unit.domain_identity(),
            )
            .field(
                "compiler_contract_version",
                &self.unit.compiler_contract_version(),
            )
            .field("cache_key", &self.cache_key)
            .field("reason", &self.reason)
            .finish()
    }
}

impl JitCompilationRequest {
    /// Creates a validated JIT compilation request.
    pub fn new(
        unit: Arc<dyn JitCompilationUnit>,
        cache_key: JitCacheKey,
        reason: JitCompilationReason,
    ) -> ZurResult<Self> {
        if unit.stable_identity().is_empty() {
            return Err(engine_error(
                "compiler unit identity must not be empty",
            ));
        }

        if unit.compiler_contract_version().is_empty() {
            return Err(engine_error(
                "compiler contract version must not be empty",
            ));
        }

        validate_domain_identifier(
            unit.domain_identity(),
        )?;

        Ok(Self {
            unit,
            cache_key,
            reason,
        })
    }

    /// Returns the compiler-owned compilation unit.
    #[must_use]
    pub fn unit(&self) -> &Arc<dyn JitCompilationUnit> {
        &self.unit
    }

    /// Returns the canonical cache key.
    #[must_use]
    pub fn cache_key(&self) -> &JitCacheKey {
        &self.cache_key
    }

    /// Returns the compilation reason.
    #[must_use]
    pub const fn reason(&self) -> JitCompilationReason {
        self.reason
    }

    /// Returns the computational-domain identity.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        self.unit.domain_identity()
    }
}

// =============================================================================
// COMPILED ARTIFACT CONTRACT
// =============================================================================

/// Opaque verified-code artifact.
///
/// The actual representation belongs to the compiler/backend integration.
///
/// It may represent:
///
/// - Native machine code.
/// - A portable executable representation.
/// - A provider-specific executable object.
/// - Another supported runtime compilation product.
///
/// This trait does not promise that the artifact has been executed.
///
/// Implementations must expose stable identity and compatibility metadata.
///
/// The concrete implementation must not expose unsafe execution operations
/// through this engine's API.
pub trait CompiledJitArtifact: fmt::Debug + Send + Sync {
    /// Returns the cache identity associated with this artifact.
    fn cache_key(&self) -> &JitCacheKey;

    /// Returns the namespaced domain identity.
    fn domain_identity(&self) -> &str;

    /// Returns the code-generation provider identity.
    fn provider_identity(&self) -> &str;

    /// Returns the generated artifact-format identity.
    fn format_identity(&self) -> &str;

    /// Returns the compiler contract version.
    fn compiler_contract_version(&self) -> &str;
}

// =============================================================================
// COMPILER BRIDGE
// =============================================================================

/// Compiler integration boundary for JIT compilation.
///
/// The implementation must reuse the canonical Zamani compiler.
///
/// It must not implement an independent parser, semantic analyzer,
/// optimization engine, lowering system or backend registry.
///
/// The compiler bridge is responsible for:
///
/// - Accepting canonical compiler-owned inputs.
/// - Running the existing compilation pipeline.
/// - Applying compatible optimization passes.
/// - Lowering through existing backend infrastructure.
/// - Producing an opaque JIT artifact.
/// - Reporting genuine compilation failures.
///
/// The engine performs verification and cache publication afterward.
pub trait JitCompilerBridge: Send + Sync {
    /// Compiles the supplied compiler-owned unit.
    fn compile(
        &self,
        request: &JitCompilationRequest,
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<Arc<dyn CompiledJitArtifact>>;
}

// =============================================================================
// VERIFICATION
// =============================================================================

/// Verifies JIT-generated artifacts.
///
/// The verifier must validate all applicable integrity and compatibility
/// requirements before an artifact is accepted.
///
/// Depending on the artifact, this may include:
///
/// - Artifact integrity.
/// - Compiler provenance.
/// - ABI compatibility.
/// - Domain compatibility.
/// - Target compatibility.
/// - Security policy.
/// - Specialization assumptions.
/// - Executable-format validity.
///
/// The verifier must fail closed.
///
/// A successful verifier call is the verification authority's result.
/// It does not mean that the program has executed successfully.
pub trait JitArtifactVerifier: Send + Sync {
    /// Verifies an artifact against its original compilation request.
    fn verify(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()>;
}

// =============================================================================
// CODE CACHE
// =============================================================================

/// JIT code-cache integration.
///
/// Cache implementations own storage, synchronization, eviction,
/// persistence and cache resource budgets.
///
/// This engine does not impose a fixed cache capacity.
///
/// Cache publication must be atomic with respect to concurrent writers.
///
/// `insert_if_absent` must return the canonical artifact associated
/// with the key, including when another compilation wins a race.
pub trait JitCodeCache: Send + Sync {
    /// Looks up an artifact by its complete canonical identity.
    fn get(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<Option<Arc<dyn CompiledJitArtifact>>>;

    /// Atomically publishes an artifact if the key is not already present.
    ///
    /// Returns the canonical artifact associated with the key.
    fn insert_if_absent(
        &self,
        key: &JitCacheKey,
        artifact: Arc<dyn CompiledJitArtifact>,
    ) -> ZurResult<Arc<dyn CompiledJitArtifact>>;

    /// Invalidates an artifact.
    ///
    /// Returns true when an entry was removed.
    fn invalidate(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<bool>;
}

// =============================================================================
// CANCELLATION
// =============================================================================

/// Cooperative JIT cancellation interface.
///
/// The authoritative runtime cancellation mechanism should be adapted
/// to this interface by the integration layer.
pub trait JitCancellation: Send + Sync {
    /// Returns whether cancellation has been requested.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation for operations without an external
/// cancellation request.
///
/// This is useful for explicitly synchronous compilation requests.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancelled;

impl JitCancellation for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// FALLBACK AUTHORIZATION
// =============================================================================

/// Authority for validating a fallback to an existing AOT execution path.
///
/// This interface must be implemented by an integration that has access
/// to the canonical validated execution plan.
///
/// It must establish that AOT fallback preserves:
///
/// - Program semantics.
/// - Required effects.
/// - Execution constraints.
/// - Required guarantees.
/// - Domain contracts.
/// - Relevant ABI requirements.
///
/// A JIT preference, provider registration or cache failure alone
/// cannot authorize fallback.
pub trait ValidatedAotFallbackAuthority: Send + Sync {
    /// Validates whether the original request can use its AOT alternative.
    fn authorize_aot_fallback(
        &self,
        request: &JitCompilationRequest,
    ) -> ZurResult<()>;
}

// =============================================================================
// FALLBACK REASONS
// =============================================================================

/// Explains why JIT was not selected or completed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JitFallbackReason {
    /// JIT is disabled by runtime configuration.
    Disabled,

    /// AOT is preferred by runtime policy.
    AotPreferred,

    /// The JIT implementation is unavailable.
    Unavailable,

    /// JIT compilation failed.
    CompilationFailed {
        /// Human-readable diagnostic from the failed compilation.
        diagnostic: String,
    },
}

// =============================================================================
// CACHE STATUS
// =============================================================================

/// Describes how a compiled artifact was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum JitCacheStatus {
    /// Artifact was obtained from an existing cache entry.
    Hit,

    /// Artifact was compiled and successfully published.
    Compiled,

    /// Another concurrent operation published the canonical artifact.
    ConcurrentPublication,
}

// =============================================================================
// COMPILATION OUTCOME
// =============================================================================

/// Result of a JIT selection and compilation request.
///
/// This is not a program execution result.
///
/// Execution remains the responsibility of the execution dispatcher.
pub enum JitCompilationOutcome {
    /// AOT was preferred and JIT was not attempted.
    AotPreferred,

    /// A validated AOT alternative is eligible for dispatch.
    ///
    /// The engine does not itself execute the AOT artifact.
    FallbackEligible {
        /// Reason JIT was not selected or completed.
        reason: JitFallbackReason,
    },

    /// A verified JIT artifact is ready for downstream execution handling.
    Compiled {
        /// Verified compiler/backend artifact.
        artifact: Arc<dyn CompiledJitArtifact>,

        /// How the artifact was obtained.
        cache_status: JitCacheStatus,
    },
}

impl fmt::Debug for JitCompilationOutcome {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::AotPreferred => {
                formatter.write_str("AotPreferred")
            }

            Self::FallbackEligible { reason } => {
                formatter
                    .debug_struct("FallbackEligible")
                    .field("reason", reason)
                    .finish()
            }

            Self::Compiled {
                artifact,
                cache_status,
            } => {
                formatter
                    .debug_struct("Compiled")
                    .field("artifact", artifact)
                    .field("cache_status", cache_status)
                    .finish()
            }
        }
    }
}

// =============================================================================
// JIT ENGINE
// =============================================================================

/// Dependency-injected JIT compilation coordinator.
///
/// The engine is immutable after construction.
///
/// Concurrent compilation requests are supported when the injected
/// compiler, verifier and cache implementations satisfy their Send/Sync
/// contracts.
///
/// The engine does not create threads or impose concurrency limits.
///
/// Those responsibilities belong to the runtime concurrency and
/// scheduling systems.
pub struct JitEngine {
    jit_policy: JitPolicy,

    fallback_policy: ExecutionFallbackPolicy,

    execution_preference: ExecutionStrategyPreference,

    compiler: Arc<dyn JitCompilerBridge>,

    verifier: Arc<dyn JitArtifactVerifier>,

    cache: Arc<dyn JitCodeCache>,
}

impl fmt::Debug for JitEngine {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("JitEngine")
            .field("jit_policy", &self.jit_policy)
            .field("fallback_policy", &self.fallback_policy)
            .field(
                "execution_preference",
                &self.execution_preference,
            )
            .finish_non_exhaustive()
    }
}

impl JitEngine {
    /// Constructs a JIT engine using canonical ZUR configuration.
    ///
    /// The engine can be constructed even when JIT is disabled.
    ///
    /// This allows runtime composition to remain independent of whether
    /// JIT is enabled for a particular execution.
    ///
    /// No compiler, verifier or cache implementation is selected
    /// implicitly.
    pub fn new(
        configuration: &RuntimeConfig,
        policy: &RuntimePolicy,
        compiler: Arc<dyn JitCompilerBridge>,
        verifier: Arc<dyn JitArtifactVerifier>,
        cache: Arc<dyn JitCodeCache>,
    ) -> ZurResult<Self> {
        configuration.validate()?;

        Ok(Self {
            jit_policy: configuration.jit_policy(),

            fallback_policy: configuration.fallback_policy(),

            execution_preference: policy.execution_preference(),

            compiler,

            verifier,

            cache,
        })
    }

    /// Returns the effective JIT activation policy.
    #[must_use]
    pub const fn jit_policy(&self) -> JitPolicy {
        self.jit_policy
    }

    /// Returns the configured execution preference.
    #[must_use]
    pub const fn execution_preference(
        &self,
    ) -> ExecutionStrategyPreference {
        self.execution_preference
    }

    /// Returns whether this engine may attempt JIT compilation.
    ///
    /// This reports configuration and policy only.
    ///
    /// It does not establish provider availability or execution feasibility.
    #[must_use]
    pub const fn is_jit_permitted(&self) -> bool {
        matches!(self.jit_policy, JitPolicy::Allow)
            && !matches!(
                self.execution_preference,
                ExecutionStrategyPreference::PreferAot
            )
    }

    /// Compiles or resolves a verified JIT artifact.
    ///
    /// The method:
    ///
    /// 1. Checks cancellation.
    /// 2. Checks runtime JIT policy.
    /// 3. Checks execution strategy preference.
    /// 4. Looks up the canonical cache identity.
    /// 5. Verifies cached artifacts.
    /// 6. Invokes the canonical compiler bridge when necessary.
    /// 7. Verifies newly compiled artifacts.
    /// 8. Publishes only verified artifacts.
    /// 9. Verifies the canonical artifact returned by publication.
    /// 10. Returns a structured compilation outcome.
    ///
    /// It does not invoke the generated program.
    pub fn compile(
        &self,
        request: &JitCompilationRequest,
        cancellation: &dyn JitCancellation,
        fallback_authority: Option<
            &dyn ValidatedAotFallbackAuthority,
        >,
    ) -> ZurResult<JitCompilationOutcome> {
        self.check_cancelled(cancellation)?;

        if matches!(
            self.execution_preference,
            ExecutionStrategyPreference::PreferAot
        ) {
            return Ok(
                JitCompilationOutcome::AotPreferred,
            );
        }

        if matches!(self.jit_policy, JitPolicy::Disabled) {
            return self.fallback_or_error(
                request,
                JitFallbackReason::Disabled,
                None,
                fallback_authority,
            );
        }

        self.check_cancelled(cancellation)?;

        // -----------------------------------------------------------------
        // CACHE LOOKUP
        // -----------------------------------------------------------------

        if let Some(cached) = self.cache.get(
            request.cache_key(),
        )? {
            self.check_cancelled(cancellation)?;

            self.validate_artifact_identity(
                request,
                cached.as_ref(),
            )?;

            self.verifier.verify(
                request,
                cached.as_ref(),
            )?;

            self.check_cancelled(cancellation)?;

            return Ok(
                JitCompilationOutcome::Compiled {
                    artifact: cached,
                    cache_status: JitCacheStatus::Hit,
                },
            );
        }

        // -----------------------------------------------------------------
        // COMPILATION
        // -----------------------------------------------------------------

        self.check_cancelled(cancellation)?;

        let compiled = match self.compiler.compile(
            request,
            cancellation,
        ) {
            Ok(artifact) => artifact,

            Err(error) => {
                if cancellation.is_cancelled() {
                    return Err(cancellation_error());
                }

                return self.fallback_or_error(
                    request,
                    JitFallbackReason::CompilationFailed {
                        diagnostic: error.to_string(),
                    },
                    Some(error),
                    fallback_authority,
                );
            }
        };

        self.check_cancelled(cancellation)?;

        // -----------------------------------------------------------------
        // ARTIFACT VALIDATION
        // -----------------------------------------------------------------

        self.validate_artifact_identity(
            request,
            compiled.as_ref(),
        )?;

        // -----------------------------------------------------------------
        // VERIFY BEFORE CACHE PUBLICATION
        // -----------------------------------------------------------------

        self.verifier.verify(
            request,
            compiled.as_ref(),
        )?;

        self.check_cancelled(cancellation)?;

        // -----------------------------------------------------------------
        // ATOMIC CACHE PUBLICATION
        // -----------------------------------------------------------------

        let canonical = self.cache.insert_if_absent(
            request.cache_key(),
            Arc::clone(&compiled),
        )?;

        self.check_cancelled(cancellation)?;

        self.validate_artifact_identity(
            request,
            canonical.as_ref(),
        )?;

        // Another compiler request may have published the canonical
        // artifact first. Therefore verify the actual returned object,
        // not merely the artifact produced by this compilation.
        self.verifier.verify(
            request,
            canonical.as_ref(),
        )?;

        self.check_cancelled(cancellation)?;

        let cache_status = if Arc::ptr_eq(
            &compiled,
            &canonical,
        ) {
            JitCacheStatus::Compiled
        } else {
            JitCacheStatus::ConcurrentPublication
        };

        Ok(JitCompilationOutcome::Compiled {
            artifact: canonical,
            cache_status,
        })
    }

    /// Invalidates a previously compiled artifact.
    ///
    /// Invalidation is delegated to the canonical cache implementation.
    ///
    /// This operation does not imply that an already running execution
    /// has been terminated.
    pub fn invalidate(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<bool> {
        self.cache.invalidate(key)
    }

    /// Validates artifact identity and compatibility metadata.
    fn validate_artifact_identity(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()> {
        if artifact.cache_key() != request.cache_key() {
            return Err(compatibility_error(
                "compiled artifact cache identity does not match request",
            ));
        }

        if artifact.domain_identity()
            != request.domain_identity()
        {
            return Err(compatibility_error(
                "compiled artifact domain does not match request",
            ));
        }

        if artifact.provider_identity().is_empty() {
            return Err(compatibility_error(
                "compiled artifact has no provider identity",
            ));
        }

        if artifact.format_identity().is_empty() {
            return Err(compatibility_error(
                "compiled artifact has no format identity",
            ));
        }

        if artifact.compiler_contract_version()
            != request.unit.compiler_contract_version()
        {
            return Err(compatibility_error(
                "compiled artifact compiler contract version mismatch",
            ));
        }

        Ok(())
    }

    /// Performs a cooperative cancellation checkpoint.
    fn check_cancelled(
        &self,
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<()> {
        if cancellation.is_cancelled() {
            return Err(cancellation_error());
        }

        Ok(())
    }

    /// Determines whether a validated AOT alternative may be dispatched.
    ///
    /// The engine never independently declares an alternative feasible.
    ///
    /// AOT fallback requires:
    ///
    /// - An explicitly permissive fallback policy.
    /// - An independently supplied execution-plan authority.
    /// - Successful validation by that authority.
    fn fallback_or_error(
        &self,
        request: &JitCompilationRequest,
        reason: JitFallbackReason,
        original_error: Option<ZurError>,
        authority: Option<
            &dyn ValidatedAotFallbackAuthority,
        >,
    ) -> ZurResult<JitCompilationOutcome> {
        if matches!(
            self.fallback_policy,
            ExecutionFallbackPolicy::ValidatedAlternativesOnly
        ) {
            if let Some(authority) = authority {
                authority.authorize_aot_fallback(
                    request,
                )?;

                return Ok(
                    JitCompilationOutcome::FallbackEligible {
                        reason,
                    },
                );
            }
        }

        match original_error {
            Some(error) => Err(error
                .with_component(COMPONENT)
                .with_operation("compile")),

            None => Err(
                ZurError::service_unavailable(
                    "JIT is unavailable and no validated AOT fallback was authorized",
                )
                .with_component(COMPONENT)
                .with_operation("select_execution_strategy"),
            ),
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::sync::Mutex;

    // ------------------------------------------------------------------------
    // TEST COMPILATION UNIT
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestUnit;

    impl JitCompilationUnit for TestUnit {
        fn stable_identity(&self) -> &str {
            "test.function"
        }

        fn domain_identity(&self) -> &str {
            "classical.general"
        }

        fn compiler_contract_version(&self) -> &str {
            "1"
        }
    }

    // ------------------------------------------------------------------------
    // TEST ARTIFACT
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            "classical.general"
        }

        fn provider_identity(&self) -> &str {
            "test.compiler"
        }

        fn format_identity(&self) -> &str {
            "test.executable"
        }

        fn compiler_contract_version(&self) -> &str {
            "1"
        }
    }

    // ------------------------------------------------------------------------
    // TEST COMPILER
    // ------------------------------------------------------------------------

    struct TestCompiler;

    impl JitCompilerBridge for TestCompiler {
        fn compile(
            &self,
            request: &JitCompilationRequest,
            cancellation: &dyn JitCancellation,
        ) -> ZurResult<Arc<dyn CompiledJitArtifact>> {
            if cancellation.is_cancelled() {
                return Err(cancellation_error());
            }

            Ok(Arc::new(TestArtifact {
                key: request.cache_key().clone(),
            }))
        }
    }

    // ------------------------------------------------------------------------
    // TEST VERIFIER
    // ------------------------------------------------------------------------

    struct TestVerifier;

    impl JitArtifactVerifier for TestVerifier {
        fn verify(
            &self,
            _request: &JitCompilationRequest,
            _artifact: &dyn CompiledJitArtifact,
        ) -> ZurResult<()> {
            Ok(())
        }
    }

    // ------------------------------------------------------------------------
    // TEST CACHE
    // ------------------------------------------------------------------------

    #[derive(Default)]
    struct TestCache {
        entries: Mutex<
            BTreeMap<
                JitCacheKey,
                Arc<dyn CompiledJitArtifact>,
            >,
        >,
    }

    impl JitCodeCache for TestCache {
        fn get(
            &self,
            key: &JitCacheKey,
        ) -> ZurResult<Option<Arc<dyn CompiledJitArtifact>>> {
            let entries = self.entries.lock().map_err(|_| {
                ZurError::internal(
                    "test cache lock poisoned",
                )
            })?;

            Ok(entries.get(key).cloned())
        }

        fn insert_if_absent(
            &self,
            key: &JitCacheKey,
            artifact: Arc<dyn CompiledJitArtifact>,
        ) -> ZurResult<Arc<dyn CompiledJitArtifact>> {
            let mut entries = self.entries.lock().map_err(|_| {
                ZurError::internal(
                    "test cache lock poisoned",
                )
            })?;

            Ok(Arc::clone(
                entries
                    .entry(key.clone())
                    .or_insert(artifact),
            ))
        }

        fn invalidate(
            &self,
            key: &JitCacheKey,
        ) -> ZurResult<bool> {
            let mut entries = self.entries.lock().map_err(|_| {
                ZurError::internal(
                    "test cache lock poisoned",
                )
            })?;

            Ok(entries.remove(key).is_some())
        }
    }

    // ------------------------------------------------------------------------
    // TEST FALLBACK AUTHORITY
    // ------------------------------------------------------------------------

    struct TestFallbackAuthority;

    impl ValidatedAotFallbackAuthority for TestFallbackAuthority {
        fn authorize_aot_fallback(
            &self,
            _request: &JitCompilationRequest,
        ) -> ZurResult<()> {
            Ok(())
        }
    }

    // ------------------------------------------------------------------------
    // TEST FIXTURES
    // ------------------------------------------------------------------------

    fn request() -> JitCompilationRequest {
        JitCompilationRequest::new(
            Arc::new(TestUnit),
            JitCacheKey::new(
                "test.compiler.v1.target.general.abi.v1",
            )
            .expect("valid test cache key"),
            JitCompilationReason::Baseline,
        )
        .expect("valid test request")
    }

    fn engine() -> JitEngine {
        let configuration = RuntimeConfig::default()
            .with_jit_policy(JitPolicy::Allow);

        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferJit,
            );

        JitEngine::new(
            &configuration,
            &policy,
            Arc::new(TestCompiler),
            Arc::new(TestVerifier),
            Arc::new(TestCache::default()),
        )
        .expect("valid engine")
    }

    // ------------------------------------------------------------------------
    // TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn cache_key_rejects_empty_identity() {
        assert!(
            JitCacheKey::new("").is_err()
        );
    }

    #[test]
    fn cache_key_rejects_surrounding_whitespace() {
        assert!(
            JitCacheKey::new(" test.key ").is_err()
        );
    }

    #[test]
    fn domain_identifier_requires_namespace() {
        assert!(
            validate_domain_identifier("classical.general").is_ok()
        );

        assert!(
            validate_domain_identifier("classical").is_err()
        );
    }

    #[test]
    fn domain_identifier_is_open_ended() {
        assert!(
            validate_domain_identifier(
                "future.photonic.computing"
            )
            .is_ok()
        );

        assert!(
            validate_domain_identifier(
                "future.neuromorphic.computing"
            )
            .is_ok()
        );
    }

    #[test]
    fn compilation_unit_requires_valid_domain() {
        let result = JitCompilationRequest::new(
            Arc::new(TestUnit),
            JitCacheKey::new("test.key")
                .expect("valid key"),
            JitCompilationReason::Baseline,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn successful_compilation_returns_verified_artifact() {
        let engine = engine();

        let outcome = engine.compile(
            &request(),
            &NeverCancelled,
            None,
        );

        assert!(matches!(
            outcome,
            Ok(JitCompilationOutcome::Compiled {
                cache_status: JitCacheStatus::Compiled,
                ..
            })
        ));
    }

    #[test]
    fn second_compilation_uses_cache() {
        let configuration = RuntimeConfig::default()
            .with_jit_policy(JitPolicy::Allow);

        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferJit,
            );

        let engine = JitEngine::new(
            &configuration,
            &policy,
            Arc::new(TestCompiler),
            Arc::new(TestVerifier),
            Arc::new(TestCache::default()),
        )
        .expect("valid engine");

        let first = engine.compile(
            &request(),
            &NeverCancelled,
            None,
        );

        assert!(matches!(
            first,
            Ok(JitCompilationOutcome::Compiled { .. })
        ));

        let second = engine.compile(
            &request(),
            &NeverCancelled,
            None,
        );

        assert!(matches!(
            second,
            Ok(JitCompilationOutcome::Compiled {
                cache_status: JitCacheStatus::Hit,
                ..
            })
        ));
    }

    #[test]
    fn cancellation_prevents_compilation() {
        struct Cancelled;

        impl JitCancellation for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let result = engine().compile(
            &request(),
            &Cancelled,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn aot_preference_does_not_invoke_jit() {
        let configuration = RuntimeConfig::default()
            .with_jit_policy(JitPolicy::Allow);

        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferAot,
            );

        let engine = JitEngine::new(
            &configuration,
            &policy,
            Arc::new(TestCompiler),
            Arc::new(TestVerifier),
            Arc::new(TestCache::default()),
        )
        .expect("valid engine");

        let result = engine.compile(
            &request(),
            &NeverCancelled,
            None,
        );

        assert!(matches!(
            result,
            Ok(JitCompilationOutcome::AotPreferred)
        ));
    }

    #[test]
    fn disabled_jit_requires_validated_fallback() {
        let configuration = RuntimeConfig::default();

        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferJit,
            );

        let engine = JitEngine::new(
            &configuration,
            &policy,
            Arc::new(TestCompiler),
            Arc::new(TestVerifier),
            Arc::new(TestCache::default()),
        )
        .expect("valid engine");

        let denied = engine.compile(
            &request(),
            &NeverCancelled,
            None,
        );

        assert!(denied.is_err());

        let permitted = engine.compile(
            &request(),
            &NeverCancelled,
            Some(&TestFallbackAuthority),
        );

        assert!(matches!(
            permitted,
            Ok(JitCompilationOutcome::FallbackEligible {
                reason: JitFallbackReason::Disabled
            })
        ));
    }

    #[test]
    fn cache_invalidation_removes_entry() {
        let engine = engine();
        let request = request();

        let _ = engine.compile(
            &request,
            &NeverCancelled,
            None,
        );

        assert!(
            engine.invalidate(request.cache_key()).unwrap()
        );

        assert!(
            !engine.invalidate(request.cache_key()).unwrap()
        );
    }
}
