
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Compiler Bridge
//!
//! Path: src/zur/jit/compiler_bridge.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97+
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Integrates the Zamani Universal Compiler with the ZUR JIT engine.
//!
//! This module is an adapter between:
//!
//!     ZUR JIT engine
//!           |
//!           v
//!     JIT compiler bridge
//!           |
//!           v
//!     Canonical compiler pipeline
//!           |
//!           +-----------------------+
//!           |                       |
//!           v                       v
//!      Compiler IR             Optimization
//!           |                       |
//!           +-----------+-----------+
//!                       |
//!                       v
//!                Compiler passes
//!                       |
//!                       v
//!                    Lowering
//!                       |
//!                       v
//!                Backend integration
//!                       |
//!                       v
//!                Generated artifact
//!                       |
//!                       v
//!                 ZUR JIT engine
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - The compiler-to-JIT integration boundary.
//! - Compiler pipeline dependency injection.
//! - Compiler output adaptation.
//! - Artifact metadata validation.
//! - Opaque artifact ownership.
//! - Cancellation propagation.
//! - Compiler bridge construction.
//!
//! It does NOT own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Classical IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - AI or tensor IR.
//! - Optimization algorithms.
//! - Compiler passes.
//! - Lowering algorithms.
//! - Backend registration.
//! - Backend selection.
//! - Machine-code generation.
//! - Executable-memory management.
//! - Physical resource allocation.
//! - Program execution.
//!
//! These remain with their existing repository authorities.
//!
//! INTEGRATION
//!
//! Existing engine:
//!     crate::zur::jit::engine
//!
//! Existing compiler:
//!     crate::compiler
//!
//! Existing compiler IR integration:
//!     crate::compiler::ir
//!
//! Existing optimization:
//!     crate::compiler::optimization
//!
//! Existing passes:
//!     crate::compiler::passes
//!
//! Existing pipeline:
//!     crate::compiler::pipeline
//!
//! Existing lowering:
//!     crate::compiler::lowering
//!
//! Existing backend integration:
//!     crate::compiler::backends
//!
//! Existing backend implementations:
//!     crate::backend
//!
//! Existing artifacts:
//!     crate::compiler::artifacts
//!
//! Existing universal IR:
//!     crate::zuir
//!
//! GRAMMAR BOUNDARY
//!
//! The following grammar authorities establish source-level intent:
//!
//!     grammar/execution/runtime.g4
//!     grammar/compile/optimization.g4
//!     grammar/resources/portability.g4
//!
//! Their corresponding semantic specifications establish:
//!
//! - Runtime requirements.
//! - Optimization requirements.
//! - Capabilities.
//! - Constraints.
//! - Preferences.
//! - Hints.
//! - Portability.
//! - Execution intent.
//!
//! This bridge does not parse grammar or reinterpret AST nodes.
//!
//! The compiler must supply already validated compilation inputs.
//!
//! POCO-REAF
//!
//! Different targets may require different generated artifacts.
//!
//! Their executable behavior must preserve the same validated
//! semantic contract.
//!
//! This bridge imposes no arbitrary maximum on:
//!
//! - Computational domains.
//! - IR operations.
//! - Program size.
//! - Functions.
//! - Modules.
//! - Devices.
//! - Processors.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Backend providers.
//! - Compilation requests.
//! - Artifact payload size.
//!
//! Actual operational limits are determined by available resources,
//! implementation capabilities and explicit operational policies.
//!
//! No finite implementation promises literal infinite computation.
//!
//! SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No fabricated compilation success.
//! - No implicit backend selection.
//! - No fabricated artifact metadata.
//! - No duplicate compiler.
//! - No duplicate IR.
//!
//! IMPORTANT INTEGRATION LIMIT
//!
//! The current repository's legacy backend pipeline accepts IrModule
//! and returns String. That interface does not establish a universal
//! typed JIT artifact pipeline for all canonical IR domains.
//!
//! Consequently, CanonicalJitCompilerPipeline is an explicit adapter
//! contract. Its implementation must call the actual canonical
//! compiler, optimization, lowering and backend APIs appropriate
//! to the supplied compiler-owned representation.
//!
//! This module does not pretend that an unavailable universal
//! compilation API already exists.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitCancellation,
    JitCompilationRequest,
    JitCompilerBridge,
    JitCacheKey,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.jit.compiler_bridge";

// ============================================================================
// VALIDATION
// ============================================================================

/// Validate a namespaced compiler/backend identity.
///
/// Identifiers are open-ended.
///
/// No arbitrary identifier-length or namespace-depth ceiling
/// is introduced.
fn validate_identity(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(bridge_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(bridge_error(
            format!("{description} contains surrounding whitespace"),
        ));
    }

    let mut segment_count = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(bridge_error(
                format!("{description} contains an empty namespace segment"),
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(bridge_error(
                format!("{description} contains an invalid character"),
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(bridge_error(
                format!("{description} contains an invalid segment boundary"),
            ));
        }

        segment_count += 1;
    }

    if segment_count < 2 {
        return Err(bridge_error(
            format!("{description} must be namespaced"),
        ));
    }

    Ok(())
}

/// Construct a structured compiler bridge error.
fn bridge_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::configuration(message)
        .with_component(COMPONENT)
}

/// Construct a structured compiler compatibility error.
fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_compiler_output")
}

/// Construct a cancellation error.
fn cancellation_error() -> ZurError {
    ZurError::cancelled(
        "JIT compilation was cancelled",
    )
    .with_component(COMPONENT)
    .with_operation("compile")
}

// ============================================================================
// OPAQUE ARTIFACT PAYLOAD
// ============================================================================

/// Opaque payload produced by an authoritative compiler/backend.
///
/// The payload may contain:
///
/// - Native executable representations.
/// - Portable executable representations.
/// - LLVM-related output.
/// - WebAssembly modules.
/// - Quantum execution packages.
/// - HDL deployment artifacts.
/// - Accelerator-specific executable representations.
/// - Other future-domain executable products.
///
/// The concrete representation remains owned by its producer.
///
/// This bridge never interprets the payload as another IR.
///
/// The payload is not automatically executable merely because
/// it implements this trait.
pub trait JitArtifactPayload:
    Any + fmt::Debug + Send + Sync
{
    /// Expose the concrete payload for safe typed inspection.
    fn as_any(&self) -> &dyn Any;
}

/// Automatically implement the payload contract for concrete
/// thread-safe, debuggable, static types.
impl<T> JitArtifactPayload for T
where
    T: Any + fmt::Debug + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================================
// COMPILER OUTPUT
// ============================================================================

/// Output returned by the canonical compiler integration.
///
/// The compiler/backend must supply genuine provider and format
/// identities.
///
/// This type does not construct artifact manifests or duplicate
/// the compiler artifact model.
///
/// Artifact provenance, integrity and verification remain with
/// their respective authoritative systems.
pub struct CanonicalCompilerOutput {
    provider_identity: String,

    format_identity: String,

    payload: Arc<dyn JitArtifactPayload>,
}

impl fmt::Debug for CanonicalCompilerOutput {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("CanonicalCompilerOutput")
            .field(
                "provider_identity",
                &self.provider_identity,
            )
            .field(
                "format_identity",
                &self.format_identity,
            )
            .field("payload", &self.payload)
            .finish()
    }
}

impl CanonicalCompilerOutput {
    /// Construct output from a real compiler/backend result.
    ///
    /// The provider and format identities must be supplied by the
    /// actual producing implementation.
    ///
    /// Empty or invalid identities are rejected.
    pub fn new(
        provider_identity: impl Into<String>,
        format_identity: impl Into<String>,
        payload: Arc<dyn JitArtifactPayload>,
    ) -> ZurResult<Self> {
        let provider_identity = provider_identity.into();
        let format_identity = format_identity.into();

        validate_identity(
            &provider_identity,
            "backend provider identity",
        )?;

        validate_identity(
            &format_identity,
            "artifact format identity",
        )?;

        Ok(Self {
            provider_identity,
            format_identity,
            payload,
        })
    }

    /// Return the actual producing backend/provider identity.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    /// Return the actual artifact format identity.
    #[must_use]
    pub fn format_identity(&self) -> &str {
        &self.format_identity
    }

    /// Borrow the opaque artifact payload.
    #[must_use]
    pub fn payload(&self) -> &Arc<dyn JitArtifactPayload> {
        &self.payload
    }

    /// Clone the shared payload handle.
    #[must_use]
    pub fn clone_payload(
        &self,
    ) -> Arc<dyn JitArtifactPayload> {
        Arc::clone(&self.payload)
    }
}

// ============================================================================
// CANONICAL COMPILER PIPELINE CONTRACT
// ============================================================================

/// Canonical compiler pipeline integration contract.
///
/// An implementation must delegate to the actual repository
/// authorities rather than introducing replacement implementations.
///
/// Required integration responsibilities:
///
/// 1. Consume the supplied compiler-owned compilation unit.
/// 2. Preserve its validated semantic contract.
/// 3. Validate the applicable IR representation.
/// 4. Resolve the appropriate compiler pipeline.
/// 5. Apply permitted compiler optimization passes.
/// 6. Apply the appropriate domain-specific lowering.
/// 7. Invoke an explicitly selected, registered backend.
/// 8. Obtain actual backend output.
/// 9. Preserve compiler and backend provenance.
/// 10. Return a typed opaque output.
///
/// The implementation must not:
///
/// - Parse source grammar.
/// - Construct another AST.
/// - Perform independent semantic analysis.
/// - Introduce another IR.
/// - Create another optimizer.
/// - Create another backend registry.
/// - Guess the hardware target.
/// - Claim a successful compilation after an error.
///
/// This trait is the dependency-injection boundary for the
/// canonical compiler pipeline.
///
/// The compiler's concrete API is intentionally not guessed:
/// the existing repository does not yet expose one universal
/// typed JIT compilation entry point for every IR domain.
pub trait CanonicalJitCompilerPipeline:
    Send + Sync
{
    /// Compile one validated compiler-owned unit.
    ///
    /// The returned output must originate from an actual
    /// successful compiler/backend operation.
    ///
    /// Cancellation must be propagated into the underlying
    /// compiler stages wherever their contracts support it.
    fn compile_unit(
        &self,
        request: &JitCompilationRequest,
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<CanonicalCompilerOutput>;
}

// ============================================================================
// JIT ARTIFACT
// ============================================================================

/// Compiler-produced artifact adapted to the ZUR JIT engine.
///
/// This is an opaque adapter, not a competing compiler artifact
/// representation.
///
/// Canonical artifact manifests, payload integrity, serialization
/// and provenance remain owned by compiler/artifacts and backend.
pub struct CompilerJitArtifact {
    cache_key: JitCacheKey,

    domain_identity: String,

    provider_identity: String,

    format_identity: String,

    compiler_contract_version: String,

    payload: Arc<dyn JitArtifactPayload>,
}

impl fmt::Debug for CompilerJitArtifact {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("CompilerJitArtifact")
            .field("cache_key", &self.cache_key)
            .field("domain_identity", &self.domain_identity)
            .field("provider_identity", &self.provider_identity)
            .field("format_identity", &self.format_identity)
            .field(
                "compiler_contract_version",
                &self.compiler_contract_version,
            )
            .field("payload", &self.payload)
            .finish()
    }
}

impl CompilerJitArtifact {
    /// Construct an artifact using the exact identity of its
    /// originating compilation request.
    ///
    /// This constructor is intentionally private.
    ///
    /// Only the compiler bridge can associate a compiler output
    /// with the original request.
    fn from_compilation(
        request: &JitCompilationRequest,
        output: CanonicalCompilerOutput,
    ) -> ZurResult<Self> {
        validate_identity(
            request.domain_identity(),
            "computational domain identity",
        )?;

        let compiler_contract_version =
            request.unit().compiler_contract_version();

        if compiler_contract_version.is_empty()
            || compiler_contract_version.trim()
                != compiler_contract_version
        {
            return Err(compatibility_error(
                "invalid compiler contract version",
            ));
        }

        Ok(Self {
            cache_key: request.cache_key().clone(),

            domain_identity: request.domain_identity().to_owned(),

            provider_identity: output.provider_identity,

            format_identity: output.format_identity,

            compiler_contract_version:
                compiler_contract_version.to_owned(),

            payload: output.payload,
        })
    }

    /// Borrow the opaque compiler-produced payload.
    #[must_use]
    pub fn payload(&self) -> &Arc<dyn JitArtifactPayload> {
        &self.payload
    }

    /// Return the cache identity.
    #[must_use]
    pub fn cache_key(&self) -> &JitCacheKey {
        &self.cache_key
    }

    /// Return the computational-domain identity.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Return the producing backend identity.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    /// Return the artifact format identity.
    #[must_use]
    pub fn format_identity(&self) -> &str {
        &self.format_identity
    }

    /// Return the compiler contract version.
    #[must_use]
    pub fn compiler_contract_version(&self) -> &str {
        &self.compiler_contract_version
    }
}

impl CompiledJitArtifact for CompilerJitArtifact {
    fn cache_key(&self) -> &JitCacheKey {
        &self.cache_key
    }

    fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    fn format_identity(&self) -> &str {
        &self.format_identity
    }

    fn compiler_contract_version(&self) -> &str {
        &self.compiler_contract_version
    }
}

// ============================================================================
// COMPILER BRIDGE
// ============================================================================

/// Concrete compiler bridge for the ZUR JIT engine.
///
/// This type delegates all compilation work to an explicitly
/// supplied canonical compiler pipeline.
///
/// It is immutable and safe to share between concurrent requests,
/// provided the injected pipeline satisfies its Send/Sync contract.
///
/// It does not create threads, choose targets, allocate resources
/// or execute generated code.
pub struct CompilerBridge {
    pipeline: Arc<dyn CanonicalJitCompilerPipeline>,
}

impl fmt::Debug for CompilerBridge {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("CompilerBridge")
            .finish_non_exhaustive()
    }
}

impl CompilerBridge {
    /// Construct the bridge using an explicitly supplied
    /// canonical compiler pipeline.
    #[must_use]
    pub fn new(
        pipeline: Arc<dyn CanonicalJitCompilerPipeline>,
    ) -> Self {
        Self { pipeline }
    }

    /// Borrow the configured canonical compiler pipeline.
    #[must_use]
    pub fn pipeline(
        &self,
    ) -> &Arc<dyn CanonicalJitCompilerPipeline> {
        &self.pipeline
    }
}

impl JitCompilerBridge for CompilerBridge {
    /// Compile one JIT request through the canonical compiler.
    ///
    /// The bridge checks cancellation before and after compilation.
    ///
    /// It associates the returned output with the original request
    /// and produces the opaque artifact consumed by JitEngine.
    fn compile(
        &self,
        request: &JitCompilationRequest,
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<Arc<dyn CompiledJitArtifact>> {
        if cancellation.is_cancelled() {
            return Err(cancellation_error());
        }

        let output = self.pipeline.compile_unit(
            request,
            cancellation,
        )?;

        if cancellation.is_cancelled() {
            return Err(cancellation_error());
        }

        let artifact = CompilerJitArtifact::from_compilation(
            request,
            output,
        )?;

        if cancellation.is_cancelled() {
            return Err(cancellation_error());
        }

        Ok(Arc::new(artifact))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::jit::engine::{
        JitCompilationReason,
        JitCompilationUnit,
        JitCacheKey,
        NeverCancelled,
    };

    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    #[derive(Debug)]
    struct TestCompilationUnit;

    impl JitCompilationUnit for TestCompilationUnit {
        fn stable_identity(&self) -> &str {
            "zamani.test.module"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }
    }

    #[derive(Debug)]
    struct TestCompilerPayload {
        content: Vec<u8>,
    }

    #[derive(Debug)]
    struct TestPipeline {
        cancelled_during_compilation: AtomicBool,
    }

    impl CanonicalJitCompilerPipeline for TestPipeline {
        fn compile_unit(
            &self,
            _request: &JitCompilationRequest,
            cancellation: &dyn JitCancellation,
        ) -> ZurResult<CanonicalCompilerOutput> {
            if cancellation.is_cancelled() {
                return Err(cancellation_error());
            }

            if self.cancelled_during_compilation.load(
                Ordering::Acquire,
            ) {
                return Err(cancellation_error());
            }

            CanonicalCompilerOutput::new(
                "zamani.test.backend",
                "zamani.test.executable",
                Arc::new(TestCompilerPayload {
                    content: vec![1, 2, 3],
                }),
            )
        }
    }

    #[derive(Debug)]
    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl JitCancellation for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }
    }

    fn request() -> JitCompilationRequest {
        JitCompilationRequest::new(
            Arc::new(TestCompilationUnit),
            JitCacheKey::new(
                "sha256:0123456789abcdef",
            )
            .expect("valid test cache key"),
            JitCompilationReason::Baseline,
        )
        .expect("valid compilation request")
    }

    fn pipeline() -> Arc<TestPipeline> {
        Arc::new(TestPipeline {
            cancelled_during_compilation:
                AtomicBool::new(false),
        })
    }

    #[test]
    fn validates_namespaced_provider_identity() {
        assert!(
            validate_identity(
                "zamani.native.backend",
                "provider",
            )
            .is_ok()
        );

        assert!(
            validate_identity(
                "vendor.extension.backend",
                "provider",
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_unqualified_provider_identity() {
        assert!(
            validate_identity(
                "backend",
                "provider",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_provider_identity() {
        assert!(
            validate_identity(
                "",
                "provider",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_artifact_format() {
        let result = CanonicalCompilerOutput::new(
            "zamani.test.backend",
            "",
            Arc::new(vec![1u8]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_open_ended_domain_identifiers() {
        assert!(
            validate_identity(
                "future.photonics",
                "domain",
            )
            .is_ok()
        );

        assert!(
            validate_identity(
                "vendor.neuromorphic.extension",
                "domain",
            )
            .is_ok()
        );
    }

    #[test]
    fn creates_artifact_with_original_request_identity() {
        let request = request();

        let output = CanonicalCompilerOutput::new(
            "zamani.test.backend",
            "zamani.test.executable",
            Arc::new(TestCompilerPayload {
                content: vec![1, 2, 3],
            }),
        )
        .expect("valid compiler output");

        let artifact = CompilerJitArtifact::from_compilation(
            &request,
            output,
        )
        .expect("valid artifact");

        assert_eq!(
            artifact.cache_key(),
            request.cache_key(),
        );

        assert_eq!(
            artifact.domain_identity(),
            request.domain_identity(),
        );

        assert_eq!(
            artifact.provider_identity(),
            "zamani.test.backend",
        );

        assert_eq!(
            artifact.format_identity(),
            "zamani.test.executable",
        );

        assert_eq!(
            artifact.compiler_contract_version(),
            "1.0.0",
        );
    }

    #[test]
    fn bridge_delegates_to_canonical_pipeline() {
        let pipeline = pipeline();

        let bridge = CompilerBridge::new(
            pipeline.clone(),
        );

        let result = bridge.compile(
            &request(),
            &NeverCancelled,
        );

        assert!(result.is_ok());

        let artifact = result.expect("compilation succeeds");

        assert_eq!(
            artifact.provider_identity(),
            "zamani.test.backend",
        );
    }

    #[test]
    fn bridge_rejects_pre_cancelled_compilation() {
        let bridge = CompilerBridge::new(
            pipeline(),
        );

        let cancellation = TestCancellation {
            cancelled: AtomicBool::new(true),
        };

        let result = bridge.compile(
            &request(),
            &cancellation,
        );

        assert!(result.is_err());
    }

    #[test]
    fn bridge_rejects_cancellation_after_pipeline_returns() {
        struct CancellingPipeline;

        impl CanonicalJitCompilerPipeline for CancellingPipeline {
            fn compile_unit(
                &self,
                _request: &JitCompilationRequest,
                _cancellation: &dyn JitCancellation,
            ) -> ZurResult<CanonicalCompilerOutput> {
                CanonicalCompilerOutput::new(
                    "zamani.test.backend",
                    "zamani.test.executable",
                    Arc::new(vec![1u8]),
                )
            }
        }

        struct CancelAfterPipeline;

        impl JitCancellation for CancelAfterPipeline {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let bridge = CompilerBridge::new(
            Arc::new(CancellingPipeline),
        );

        let result = bridge.compile(
            &request(),
            &CancelAfterPipeline,
        );

        assert!(result.is_err());
    }

    #[test]
    fn payload_remains_typed_and_opaque() {
        let payload = TestCompilerPayload {
            content: vec![10, 20, 30],
        };

        let output = CanonicalCompilerOutput::new(
            "zamani.test.backend",
            "zamani.test.executable",
            Arc::new(payload),
        )
        .expect("valid output");

        let concrete = output
            .payload()
            .as_any()
            .downcast_ref::<TestCompilerPayload>()
            .expect("original payload type is preserved");

        assert_eq!(
            concrete.content,
            vec![10, 20, 30],
        );
    }

    #[test]
    fn compiler_output_is_not_fabricated_by_bridge() {
        struct FailingPipeline;

        impl CanonicalJitCompilerPipeline for FailingPipeline {
            fn compile_unit(
                &self,
                _request: &JitCompilationRequest,
                _cancellation: &dyn JitCancellation,
            ) -> ZurResult<CanonicalCompilerOutput> {
                Err(
                    ZurError::configuration(
                        "canonical compiler failed",
                    )
                )
            }
        }

        let bridge = CompilerBridge::new(
            Arc::new(FailingPipeline),
        );

        assert!(
            bridge.compile(
                &request(),
                &NeverCancelled,
            ).is_err()
        );
    }
}
