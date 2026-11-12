
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Executable-Code Management
//!
//! Path: `src/zur/jit/executable.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the safe ownership and provider-integration boundary between
//! verified JIT artifacts and executable-code objects.
//!
//! This module does not implement a machine-code generator, executable-memory
//! allocator, instruction-set encoder, relocation engine, operating-system
//! loader, or execution ABI.
//!
//! Those responsibilities belong to compiler, backend, platform and
//! execution-provider implementations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Executable-provider integration contracts.
//! - Executable-code object contracts.
//! - Verification-before-loading orchestration.
//! - Executable artifact identity validation.
//! - Shared executable-code handles.
//! - Invocation admission leases.
//! - Executable-code retirement state.
//!
//! It does not own:
//!
//! - Source parsing or grammar interpretation.
//! - Semantic analysis.
//! - Classical, quantum, HDL or AI intermediate representations.
//! - Compiler optimization or lowering.
//! - Backend registration or code generation.
//! - Artifact manifests or serialization.
//! - Executable memory allocation.
//! - Native instruction execution.
//! - Hardware discovery or resource reservation.
//! - JIT cache storage or invalidation policy.
//! - Scheduling or execution dispatch.
//! - Domain-specific invocation ABIs.
//!
//! ## Existing repository integration
//!
//! `crate::zur::jit::engine`:
//!     JitCompilationRequest
//!     JitCacheKey
//!     CompiledJitArtifact
//!     JitArtifactVerifier
//!
//! `crate::zur::error`:
//!     ZurError
//!     ZurResult
//!
//! `crate::zur::jit::code_cache`:
//!     Existing artifact cache and cache identity.
//!
//! `crate::zur::jit::invalidation`:
//!     Cache invalidation and invalidation-triggered retirement.
//!
//! `crate::zur::jit::deoptimization`:
//!     Specialized-to-baseline execution handoff.
//!
//! `crate::compiler` and `crate::backend`:
//!     Canonical compilation and artifact production.
//!
//! ## Safety contract
//!
//! - This module contains no unsafe Rust.
//! - It exposes no raw pointers or function pointers.
//! - It does not assume that an artifact is executable merely because it
//!   implements `CompiledJitArtifact`.
//! - It verifies artifacts before asking a provider to load them.
//! - Provider implementations must own and release their executable
//!   resources through their returned object's lifetime.
//! - An invocation lease prevents retirement from completing while that
//!   lease is held.
//! - Retirement does not forcibly terminate already admitted invocations.
//!
//! A platform provider requiring unsafe Rust internally must not introduce
//! unsafe code into this crate. Any external dependency used by such a
//! provider requires a separate safety and dependency audit.
//!
//! ## Scalability
//!
//! This module imposes no fixed limits on:
//!
//! - Computational domains.
//! - Artifact formats.
//! - Executable providers.
//! - Executable objects.
//! - Compilation units.
//! - Execution sessions.
//! - Hardware architectures.
//! - Device counts.
//! - Resource dimensions.
//!
//! Actual capacity is determined by the supplied provider, operating
//! environment, available resources and explicitly configured budgets.
//!
//! Literal infinite physical computation is not promised.

use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitArtifactVerifier,
    JitCacheKey,
    JitCompilationRequest,
};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

const COMPONENT: &str = "zur.jit.executable";

// =============================================================================
// ERROR HELPERS
// =============================================================================

/// Creates a structured executable-management input error.
fn executable_input_error(message: impl Into<String>) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
}

/// Creates a structured executable compatibility error.
fn executable_compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("load_verified_artifact")
}

/// Creates a structured executable-provider availability error.
fn executable_provider_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::unsupported(message)
        .with_component(COMPONENT)
        .with_operation("load_executable")
}

/// Validates a required identity without imposing an arbitrary length limit.
fn validate_identity(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(executable_input_error(format!(
            "{description} must not be empty"
        )));
    }

    if value.trim() != value {
        return Err(executable_input_error(format!(
            "{description} contains surrounding whitespace"
        )));
    }

    Ok(())
}

// =============================================================================
// EXECUTABLE-CODE OBJECT
// =============================================================================

/// Provider-owned executable-code resource.
///
/// An implementation represents an executable resource prepared by an
/// explicitly selected executable provider.
///
/// Examples include:
///
/// - A native executable-code allocation.
/// - A JIT engine's compiled-code object.
/// - A provider-managed executable module.
/// - A portable executable object accepted by an interpreter or virtual
///   machine.
/// - A domain-specific executable resource.
///
/// The implementation owns its underlying resource.
///
/// When the last strong reference to the implementation is dropped, the
/// provider must release or safely retire its owned resources according to
/// its platform contract.
///
/// This trait deliberately does not expose a raw entry-point address or
/// function pointer.
///
/// Actual invocation belongs to the appropriate execution-provider and
/// execution-ABI integration.
pub trait ExecutableCode: fmt::Debug + Send + Sync {
    /// Returns the exact JIT cache identity from which this object originated.
    fn cache_key(&self) -> &JitCacheKey;

    /// Returns the identity of the provider that loaded this object.
    ///
    /// This is the executable-loading provider, not necessarily the compiler
    /// or code-generation provider that produced the artifact.
    fn loader_identity(&self) -> &str;

    /// Returns the executable format accepted by this object.
    ///
    /// This must match the verified artifact's format identity.
    fn format_identity(&self) -> &str;

    /// Returns the provider's stable identity for this executable object.
    ///
    /// This identifier is useful for diagnostics and observability.
    fn executable_identity(&self) -> &str;
}

// =============================================================================
// EXECUTABLE PROVIDER
// =============================================================================

/// Safe integration boundary for executable-code providers.
///
/// An implementation is responsible for converting an already verified
/// compiler/backend artifact into a provider-owned executable-code object.
///
/// Implementations may delegate to:
///
/// - A safe JIT library API.
/// - A safe virtual-machine API.
/// - A safe portable-code execution engine.
/// - A platform execution service.
/// - Another explicitly supported executable provider.
///
/// The provider must not report success unless it has actually prepared
/// the executable object.
///
/// This trait does not authorize a provider to bypass verification,
/// security policy, ABI validation or execution compatibility checks.
pub trait JitExecutableProvider: Send + Sync {
    /// Returns the stable identity of this executable-loading provider.
    fn provider_identity(&self) -> &str;

    /// Returns whether this provider supports the supplied artifact format.
    ///
    /// This is a format-support declaration, not proof that a particular
    /// artifact is valid or executable.
    fn supports_format(
        &self,
        format_identity: &str,
    ) -> bool;

    /// Loads a verified artifact into a provider-owned executable object.
    ///
    /// The implementation must:
    ///
    /// - Validate provider-specific format requirements.
    /// - Enforce its platform and security requirements.
    /// - Establish executable-resource ownership.
    /// - Return a genuine executable object.
    /// - Release partially created resources if loading fails.
    ///
    /// It must not reinterpret source semantics or generate a new artifact.
    fn load(
        &self,
        request: &JitCompilationRequest,
        artifact: Arc<dyn CompiledJitArtifact>,
    ) -> ZurResult<Arc<dyn ExecutableCode>>;
}

// =============================================================================
// EXECUTABLE-CODE LIFECYCLE STATE
// =============================================================================

/// Shared state controlling admission of new executable invocations.
#[derive(Debug, Default)]
struct ExecutableLifecycle {
    retired: bool,
}

// =============================================================================
// EXECUTABLE-CODE HANDLE
// =============================================================================

/// Shared, safely owned handle to a provider-owned executable object.
///
/// Cloning this handle shares both:
///
/// - The underlying executable resource.
/// - The retirement state.
///
/// Dropping one clone does not destroy the executable object while another
/// clone or invocation lease still owns it.
///
/// Retirement prevents new invocation leases from being acquired.
///
/// Existing invocation leases remain valid until they are dropped.
#[derive(Clone)]
pub struct ExecutableCodeHandle {
    code: Arc<dyn ExecutableCode>,
    lifecycle: Arc<RwLock<ExecutableLifecycle>>,
}

impl fmt::Debug for ExecutableCodeHandle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutableCodeHandle")
            .field("cache_key", self.code.cache_key())
            .field("loader_identity", &self.code.loader_identity())
            .field("format_identity", &self.code.format_identity())
            .field(
                "executable_identity",
                &self.code.executable_identity(),
            )
            .field("retired", &self.is_retired().ok())
            .finish()
    }
}

impl ExecutableCodeHandle {
    /// Constructs a handle from a provider-owned executable object.
    ///
    /// This constructor is private because executable objects must pass
    /// through the manager's verification and identity-validation boundary.
    fn new(code: Arc<dyn ExecutableCode>) -> Self {
        Self {
            code,
            lifecycle: Arc::new(RwLock::new(
                ExecutableLifecycle::default(),
            )),
        }
    }

    /// Returns the canonical JIT cache identity.
    #[must_use]
    pub fn cache_key(&self) -> &JitCacheKey {
        self.code.cache_key()
    }

    /// Returns the executable-loading provider identity.
    #[must_use]
    pub fn loader_identity(&self) -> &str {
        self.code.loader_identity()
    }

    /// Returns the loaded executable format identity.
    #[must_use]
    pub fn format_identity(&self) -> &str {
        self.code.format_identity()
    }

    /// Returns the provider's executable-object identity.
    #[must_use]
    pub fn executable_identity(&self) -> &str {
        self.code.executable_identity()
    }

    /// Returns whether the executable object has been retired.
    ///
    /// A poisoned lifecycle lock is reported as an error.
    pub fn is_retired(&self) -> ZurResult<bool> {
        let lifecycle = self.read_lifecycle()?;
        Ok(lifecycle.retired)
    }

    /// Acquires permission to begin an invocation using this executable.
    ///
    /// The returned lease must be held for the complete duration of the
    /// invocation.
    ///
    /// Retirement obtains an exclusive lifecycle lock. Therefore:
    ///
    /// - An invocation admitted before retirement may finish.
    /// - Retirement waits for admitted invocation leases to be dropped.
    /// - Once retirement completes, new invocation leases are rejected.
    ///
    /// This method does not itself execute code. The execution subsystem
    /// must hold the lease while using the executable object.
    pub fn begin_invocation(
        &self,
    ) -> ZurResult<ExecutableInvocationLease<'_>> {
        let lifecycle = self.read_lifecycle()?;

        if lifecycle.retired {
            return Err(
                ZurError::unsupported(
                    "executable object has been retired",
                )
                .with_component(COMPONENT)
                .with_operation("begin_invocation"),
            );
        }

        Ok(ExecutableInvocationLease {
            code: self.code.as_ref(),
            _lifecycle: lifecycle,
        })
    }

    /// Retires this executable object for future invocations.
    ///
    /// This operation is idempotent.
    ///
    /// It waits for existing invocation leases to finish. It does not
    /// terminate, interrupt or roll back already admitted execution.
    ///
    /// Retirement is distinct from:
    ///
    /// - Removing an artifact from the JIT cache.
    /// - Invalidating compilation assumptions.
    /// - Deoptimizing an executing computation.
    /// - Releasing a physical resource while it is still in use.
    ///
    /// Those operations remain owned by their respective subsystems.
    ///
    /// Returns `true` only when this call changes the object from active
    /// to retired.
    pub fn retire(&self) -> ZurResult<bool> {
        let mut lifecycle = self.write_lifecycle()?;

        if lifecycle.retired {
            return Ok(false);
        }

        lifecycle.retired = true;

        Ok(true)
    }

    /// Returns the provider-owned executable object while an invocation
    /// lease is held.
    ///
    /// This is the only public route from the handle to the executable
    /// object. Callers must retain the lease for the entire period during
    /// which the object is used.
    fn read_lifecycle(
        &self,
    ) -> ZurResult<RwLockReadGuard<'_, ExecutableLifecycle>> {
        self.lifecycle.read().map_err(|_| {
            ZurError::internal(
                "executable lifecycle lock was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("read_lifecycle")
        })
    }

    /// Obtains exclusive access to the lifecycle state.
    fn write_lifecycle(
        &self,
    ) -> ZurResult<RwLockWriteGuard<'_, ExecutableLifecycle>> {
        self.lifecycle.write().map_err(|_| {
            ZurError::internal(
                "executable lifecycle lock was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("write_lifecycle")
        })
    }
}

// =============================================================================
// INVOCATION LEASE
// =============================================================================

/// Admission lease for one use of an executable object.
///
/// The lease holds a shared lifecycle lock for its lifetime. This prevents
/// retirement from completing while the executable is being used.
///
/// The execution subsystem must keep this lease alive until the actual
/// invocation has finished or has been safely abandoned.
pub struct ExecutableInvocationLease<'a> {
    code: &'a dyn ExecutableCode,
    _lifecycle: RwLockReadGuard<'a, ExecutableLifecycle>,
}

impl fmt::Debug for ExecutableInvocationLease<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutableInvocationLease")
            .field("cache_key", self.code.cache_key())
            .field("loader_identity", &self.code.loader_identity())
            .field("executable_identity", &self.code.executable_identity())
            .finish()
    }
}

impl ExecutableInvocationLease<'_> {
    /// Returns the executable object protected by this lease.
    ///
    /// The returned reference must not be retained beyond the lease.
    #[must_use]
    pub fn executable(&self) -> &dyn ExecutableCode {
        self.code
    }
}

// =============================================================================
// EXECUTABLE MANAGER
// =============================================================================

/// Coordinates verification and loading of JIT executable objects.
///
/// The manager is immutable after construction. It may be shared across
/// threads when its injected verifier and provider implementations satisfy
/// their `Send + Sync` contracts.
///
/// It does not create threads, compile code, select hardware, manage cache
/// capacity or execute program entry points.
pub struct JitExecutableManager {
    verifier: Arc<dyn JitArtifactVerifier>,
    provider: Arc<dyn JitExecutableProvider>,
}

impl fmt::Debug for JitExecutableManager {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("JitExecutableManager")
            .field(
                "provider_identity",
                &self.provider.provider_identity(),
            )
            .finish_non_exhaustive()
    }
}

impl JitExecutableManager {
    /// Constructs an executable manager with explicit verification and
    /// executable-provider dependencies.
    ///
    /// No provider is discovered or selected implicitly.
    pub fn new(
        verifier: Arc<dyn JitArtifactVerifier>,
        provider: Arc<dyn JitExecutableProvider>,
    ) -> ZurResult<Self> {
        validate_identity(
            provider.provider_identity(),
            "executable provider identity",
        )?;

        Ok(Self {
            verifier,
            provider,
        })
    }

    /// Returns the configured executable-provider identity.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        self.provider.provider_identity()
    }

    /// Verifies and loads a compiled JIT artifact.
    ///
    /// The order is intentional:
    ///
    /// 1. Validate request/artifact identity.
    /// 2. Validate artifact metadata.
    /// 3. Verify the artifact using the canonical verifier.
    /// 4. Check provider format support.
    /// 5. Ask the provider to load the artifact.
    /// 6. Validate the returned executable object's identity.
    /// 7. Return a shared executable handle.
    ///
    /// The artifact must already have been produced by the canonical
    /// compiler/backend integration.
    ///
    /// This method does not assume that a cache hit is valid. The caller
    /// must pass the artifact through the verifier regardless of whether
    /// it originated from a cache or a fresh compilation.
    pub fn load_verified(
        &self,
        request: &JitCompilationRequest,
        artifact: Arc<dyn CompiledJitArtifact>,
    ) -> ZurResult<ExecutableCodeHandle> {
        self.validate_artifact_identity(request, artifact.as_ref())?;

        // Fail closed: no executable provider is called before verification.
        self.verifier.verify(request, artifact.as_ref())?;

        let format_identity = artifact.format_identity();

        if !self.provider.supports_format(format_identity) {
            return Err(executable_provider_error(format!(
                "provider '{}' does not support artifact format '{}'",
                self.provider.provider_identity(),
                format_identity,
            )));
        }

        let executable = self.provider.load(
            request,
            Arc::clone(&artifact),
        )?;

        self.validate_executable_identity(
            request,
            artifact.as_ref(),
            executable.as_ref(),
        )?;

        Ok(ExecutableCodeHandle::new(executable))
    }

    /// Validates the artifact against the original compilation request.
    fn validate_artifact_identity(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()> {
        if request.cache_key() != artifact.cache_key() {
            return Err(executable_compatibility_error(
                "compiled artifact cache identity does not match the request",
            ));
        }

        if request.domain_identity() != artifact.domain_identity() {
            return Err(executable_compatibility_error(
                "compiled artifact domain does not match the request",
            ));
        }

        validate_identity(
            artifact.provider_identity(),
            "artifact code-generation provider identity",
        )?;

        validate_identity(
            artifact.format_identity(),
            "artifact format identity",
        )?;

        validate_identity(
            artifact.compiler_contract_version(),
            "artifact compiler contract version",
        )?;

        Ok(())
    }

    /// Validates the executable object returned by the provider.
    fn validate_executable_identity(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
        executable: &dyn ExecutableCode,
    ) -> ZurResult<()> {
        if request.cache_key() != executable.cache_key() {
            return Err(executable_compatibility_error(
                "loaded executable cache identity does not match the request",
            ));
        }

        if artifact.format_identity() != executable.format_identity() {
            return Err(executable_compatibility_error(
                "loaded executable format does not match the verified artifact",
            ));
        }

        if self.provider.provider_identity()
            != executable.loader_identity()
        {
            return Err(executable_compatibility_error(
                "loaded executable identifies a different loading provider",
            ));
        }

        validate_identity(
            executable.loader_identity(),
            "executable loader identity",
        )?;

        validate_identity(
            executable.executable_identity(),
            "executable object identity",
        )?;

        Ok(())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use crate::zur::jit::engine::{
        JitCompilationReason,
        JitCompilationUnit,
        JitCacheKey,
    };

    #[derive(Debug)]
    struct TestCompilationUnit;

    impl JitCompilationUnit for TestCompilationUnit {
        fn stable_identity(&self) -> &str {
            "zamani.test.module.function"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0"
        }
    }

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
        domain: String,
        provider: String,
        format: String,
        compiler_version: String,
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            &self.domain
        }

        fn provider_identity(&self) -> &str {
            &self.provider
        }

        fn format_identity(&self) -> &str {
            &self.format
        }

        fn compiler_contract_version(&self) -> &str {
            &self.compiler_version
        }
    }

    #[derive(Debug, Default)]
    struct TestVerifier {
        called: AtomicBool,
        reject: AtomicBool,
    }

    impl JitArtifactVerifier for TestVerifier {
        fn verify(
            &self,
            _request: &JitCompilationRequest,
            _artifact: &dyn CompiledJitArtifact,
        ) -> ZurResult<()> {
            self.called.store(true, Ordering::SeqCst);

            if self.reject.load(Ordering::SeqCst) {
                return Err(
                    ZurError::invalid_data(
                        "test verifier rejected the artifact",
                    )
                    .with_component(COMPONENT)
                    .with_operation("verify"),
                );
            }

            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestExecutable {
        key: JitCacheKey,
        loader: String,
        format: String,
        identity: String,
    }

    impl ExecutableCode for TestExecutable {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn loader_identity(&self) -> &str {
            &self.loader
        }

        fn format_identity(&self) -> &str {
            &self.format
        }

        fn executable_identity(&self) -> &str {
            &self.identity
        }
    }

    #[derive(Debug)]
    struct TestProvider {
        identity: String,
        supported_format: String,
        load_count: AtomicUsize,
        wrong_key: AtomicBool,
    }

    impl JitExecutableProvider for TestProvider {
        fn provider_identity(&self) -> &str {
            &self.identity
        }

        fn supports_format(&self, format: &str) -> bool {
            format == self.supported_format
        }

        fn load(
            &self,
            request: &JitCompilationRequest,
            artifact: Arc<dyn CompiledJitArtifact>,
        ) -> ZurResult<Arc<dyn ExecutableCode>> {
            self.load_count.fetch_add(1, Ordering::SeqCst);

            let key = if self.wrong_key.load(Ordering::SeqCst) {
                JitCacheKey::new("zamani.test.wrong-key")?
            } else {
                request.cache_key().clone()
            };

            Ok(Arc::new(TestExecutable {
                key,
                loader: self.identity.clone(),
                format: artifact.format_identity().to_owned(),
                identity: "test.executable.object".to_owned(),
            }))
        }
    }

    fn test_request() -> JitCompilationRequest {
        let unit: Arc<dyn JitCompilationUnit> =
            Arc::new(TestCompilationUnit);

        JitCompilationRequest::new(
            unit,
            JitCacheKey::new("zamani.test.compilation")?,
            JitCompilationReason::Baseline,
        )
        .expect("test request must be valid")
    }

    fn test_artifact(
        request: &JitCompilationRequest,
    ) -> Arc<dyn CompiledJitArtifact> {
        Arc::new(TestArtifact {
            key: request.cache_key().clone(),
            domain: request.domain_identity().to_owned(),
            provider: "zamani.test.compiler".to_owned(),
            format: "zamani.test.executable-format".to_owned(),
            compiler_version: "1.0".to_owned(),
        })
    }

    fn test_manager(
        verifier: Arc<TestVerifier>,
        provider: Arc<TestProvider>,
    ) -> JitExecutableManager {
        JitExecutableManager::new(verifier, provider)
            .expect("test manager must be valid")
    }

    #[test]
    fn verified_artifact_is_loaded() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(
            Arc::clone(&verifier),
            Arc::clone(&provider),
        );

        let handle = manager
            .load_verified(&request, test_artifact(&request))
            .expect("verified artifact should load");

        assert!(verifier.called.load(Ordering::SeqCst));
        assert_eq!(provider.load_count.load(Ordering::SeqCst), 1);
        assert_eq!(
            handle.cache_key(),
            request.cache_key(),
        );
        assert_eq!(
            handle.loader_identity(),
            "zamani.test.loader",
        );
        assert!(!handle.is_retired().unwrap());
    }

    #[test]
    fn rejected_artifact_is_never_loaded() {
        let request = test_request();

        let verifier = Arc::new(TestVerifier::default());
        verifier.reject.store(true, Ordering::SeqCst);

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(
            Arc::clone(&verifier),
            Arc::clone(&provider),
        );

        assert!(
            manager
                .load_verified(&request, test_artifact(&request))
                .is_err()
        );

        assert_eq!(provider.load_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unsupported_format_is_rejected_before_provider_loading() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format: "zamani.test.other-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(verifier, Arc::clone(&provider));

        assert!(
            manager
                .load_verified(&request, test_artifact(&request))
                .is_err()
        );

        assert_eq!(provider.load_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn mismatched_provider_output_is_rejected() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(true),
        });

        let manager = test_manager(verifier, provider);

        assert!(
            manager
                .load_verified(&request, test_artifact(&request))
                .is_err()
        );
    }

    #[test]
    fn retirement_is_idempotent() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(verifier, provider);

        let handle = manager
            .load_verified(&request, test_artifact(&request))
            .expect("artifact should load");

        assert!(handle.retire().unwrap());
        assert!(!handle.retire().unwrap());
        assert!(handle.is_retired().unwrap());
        assert!(handle.begin_invocation().is_err());
    }

    #[test]
    fn cloned_handles_share_retirement_state() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(verifier, provider);

        let original = manager
            .load_verified(&request, test_artifact(&request))
            .expect("artifact should load");

        let clone = original.clone();

        original.retire().unwrap();

        assert!(clone.is_retired().unwrap());
        assert!(clone.begin_invocation().is_err());
    }

    #[test]
    fn invocation_lease_exposes_the_executable_object() {
        let request = test_request();
        let verifier = Arc::new(TestVerifier::default());

        let provider = Arc::new(TestProvider {
            identity: "zamani.test.loader".to_owned(),
            supported_format:
                "zamani.test.executable-format".to_owned(),
            load_count: AtomicUsize::new(0),
            wrong_key: AtomicBool::new(false),
        });

        let manager = test_manager(verifier, provider);

        let handle = manager
            .load_verified(&request, test_artifact(&request))
            .expect("artifact should load");

        let lease = handle
            .begin_invocation()
            .expect("active executable should admit invocation");

        assert_eq!(
            lease.executable().cache_key(),
            request.cache_key(),
        );
    }
}
