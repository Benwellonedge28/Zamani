
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Context.
//!
//! Path:
//!     src/compiler/context/compilation_context.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ================================================================
//! PURPOSE
//! ================================================================
//!
//! This module defines the immutable, shareable context associated
//! with one compiler operation.
//!
//! The context provides:
//!
//! - Immutable compiler configuration.
//! - Shared configuration access.
//! - Optional source identity.
//! - Shared operational resource-budget accounting.
//! - Shared cancellation.
//! - Configuration validation.
//! - Context derivation for additional source units.
//! - Thread-safe context cloning.
//!
//! The context is intended to be shared by compiler stages and
//! parallel compilation workers.
//!
//! ================================================================
//! ARCHITECTURAL OWNERSHIP
//! ================================================================
//!
//! Existing authoritative owners:
//!
//! crate::compiler_types
//!     CompilerConfig
//!     CompilationTarget
//!     OptimizationLevel
//!
//! compiler::configuration::config
//!     Configuration construction and validation.
//!
//! compiler::configuration::budget
//!     CompilationBudget
//!     BudgetTracker
//!     BudgetDimension
//!     BudgetError
//!
//! compiler::configuration::target
//!     TargetId
//!     TargetDescriptor
//!     Target metadata and structural validation.
//!
//! compiler::diagnostics
//!     Compiler diagnostic definitions.
//!
//! crate::source_map
//!     FileId
//!     BytePos
//!     Span
//!     SourceFile
//!     SourceMap
//!
//! This module owns only the shared compilation context.
//!
//! It does NOT own:
//!
//! - Source parsing.
//! - AST construction.
//! - Name resolution.
//! - Type inference.
//! - Semantic validation.
//! - Ownership analysis.
//! - Borrow checking.
//! - Effect analysis.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Backend implementation.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//!
//! ================================================================
//! POCO-REAF
//! ================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This context introduces no universal hardware-capacity ceilings.
//!
//! It does not impose fixed limits on:
//!
//! - CPUs.
//! - Cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Register width.
//! - IR operations.
//! - Modules.
//! - Functions.
//! - Source size.
//!
//! Resource budgets are configurable.
//!
//! An unlimited budget means that this context imposes no explicit
//! operational budget for the corresponding dimension.
//!
//! It does not promise infinite physical resources.
//!
//! ================================================================
//! CONCURRENCY
//! ================================================================
//!
//! Cloning CompilationContext shares:
//!
//! - Compiler configuration.
//! - Budget accounting.
//! - Cancellation state.
//!
//! This prevents parallel workers from independently receiving
//! fresh copies of the same compilation budget.
//!
//! Configuration is immutable after construction.
//!
//! Operational accounting and cancellation use controlled,
//! thread-safe interior mutability.
//!
//! ================================================================
//! DEPENDENCY DIRECTION
//! ================================================================
//!
//! compiler_types
//!       |
//!       v
//! configuration
//!       |
//!       v
//! compilation_context
//!       |
//!       +----> compiler API
//!       |
//!       +----> compilation sessions
//!       |
//!       +----> pipeline
//!       |
//!       +----> analysis
//!       |
//!       +----> optimization
//!       |
//!       +----> parallel compilation
//!       |
//!       +----> incremental compilation
//!
//! This module does not depend on pipeline orchestration.
//!
//! It can therefore be completed independently of the pipeline.
//!
//! ================================================================

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::compiler::configuration::budget::{
    BudgetDimension,
    BudgetError,
    BudgetTracker,
    CompilationBudget,
};

use crate::compiler::configuration::config::{
    validate_compiler_config,
    ConfigurationError,
    ConfigurationValidationMode,
};

use crate::compiler_types::CompilerConfig;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the compilation-context contract.
///
/// This is independent of:
///
/// - Zamani language version.
/// - Compiler release version.
/// - Grammar version.
/// - AST version.
/// - Semantic-model version.
/// - IR version.
/// - Backend ABI version.
///
/// Increment only when the public context contract changes
/// incompatibly.
pub const COMPILATION_CONTEXT_API_VERSION: u16 = 1;

// ============================================================================
// SOURCE IDENTITY
// ============================================================================

/// Identifies the origin of source associated with a compilation.
///
/// This is descriptive metadata.
///
/// It does not define source-language semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SourceOrigin {
    /// Source supplied directly as in-memory text.
    InMemory,

    /// Source loaded from a filesystem path.
    Filesystem,

    /// Source generated by another compiler subsystem.
    Generated,

    /// Source origin was not supplied.
    Unspecified,
}

/// Identifies a logical source unit.
///
/// The canonical source-map implementation remains responsible for
/// FileId, SourceFile, Span, byte positions, and source mapping.
///
/// This structure does not introduce a competing source map.
///
/// A source identity can be shared between compiler stages without
/// copying its logical name or filesystem path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceIdentity {
    logical_name: Arc<str>,
    path: Option<Arc<PathBuf>>,
    origin: SourceOrigin,
}

/// Errors associated with source identity construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceIdentityError {
    /// The logical source name is empty.
    EmptyName,

    /// The logical source name contains control characters.
    InvalidName,
}

impl fmt::Display for SourceIdentityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "source identity requires a non-empty logical name",
                )
            }

            Self::InvalidName => {
                formatter.write_str(
                    "source identity contains invalid control characters",
                )
            }
        }
    }
}

impl Error for SourceIdentityError {}

impl SourceIdentity {
    /// Creates an identity for an in-memory source unit.
    ///
    /// No arbitrary source-name length limit is imposed.
    pub fn new(
        logical_name: impl Into<Arc<str>>,
    ) -> Result<Self, SourceIdentityError> {
        Self::with_origin(
            logical_name,
            SourceOrigin::InMemory,
        )
    }

    /// Creates an identity with an explicitly supplied origin.
    pub fn with_origin(
        logical_name: impl Into<Arc<str>>,
        origin: SourceOrigin,
    ) -> Result<Self, SourceIdentityError> {
        let logical_name = logical_name.into();

        if logical_name.trim().is_empty() {
            return Err(SourceIdentityError::EmptyName);
        }

        if logical_name.chars().any(char::is_control) {
            return Err(SourceIdentityError::InvalidName);
        }

        Ok(Self {
            logical_name,
            path: None,
            origin,
        })
    }

    /// Creates a filesystem-backed source identity.
    ///
    /// This constructor does not access the filesystem.
    ///
    /// Paths are retained as PathBuf so that non-UTF-8 filesystem
    /// paths are not discarded.
    pub fn from_path(
        path: impl Into<PathBuf>,
    ) -> Result<Self, SourceIdentityError> {
        let path = path.into();

        if path.as_os_str().is_empty() {
            return Err(SourceIdentityError::EmptyName);
        }

        let logical_name: Arc<str> =
            Arc::from(path.display().to_string());

        if logical_name.trim().is_empty() {
            return Err(SourceIdentityError::EmptyName);
        }

        Ok(Self {
            logical_name,
            path: Some(Arc::new(path)),
            origin: SourceOrigin::Filesystem,
        })
    }

    /// Creates an identity for compiler-generated source.
    pub fn generated(
        logical_name: impl Into<Arc<str>>,
    ) -> Result<Self, SourceIdentityError> {
        Self::with_origin(
            logical_name,
            SourceOrigin::Generated,
        )
    }

    /// Returns the logical source name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        self.logical_name.as_ref()
    }

    /// Returns the filesystem path when available.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path
            .as_ref()
            .map(|path| path.as_path())
    }

    /// Returns the source origin.
    #[must_use]
    pub const fn origin(&self) -> SourceOrigin {
        self.origin
    }

    /// Returns whether this identity refers to a filesystem path.
    #[must_use]
    pub fn is_filesystem_backed(&self) -> bool {
        self.path.is_some()
    }
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Thread-safe cancellation token.
///
/// Clones share the same cancellation state.
///
/// This allows a compiler session to request cancellation while
/// compilation stages or parallel workers are executing.
///
/// Cancellation is cooperative.
///
/// Compiler stages must periodically inspect the token and stop
/// according to their own cancellation-safety contracts.
///
/// This type does not forcibly terminate threads or interrupt
/// arbitrary operating-system operations.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a new, uncancelled token.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Requests cancellation.
    ///
    /// Returns true if this invocation changed the state from
    /// not-cancelled to cancelled.
    ///
    /// Returns false if cancellation had already been requested.
    pub fn cancel(&self) -> bool {
        !self.cancelled.swap(true, Ordering::AcqRel)
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Checks cancellation and returns an explicit result.
    pub fn check(&self) -> Result<(), ContextError> {
        if self.is_cancelled() {
            Err(ContextError::Cancelled)
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// CONTEXT ERROR
// ============================================================================

/// Errors associated with compilation-context operations.
#[derive(Debug)]
#[non_exhaustive]
pub enum ContextError {
    /// The supplied compiler configuration is invalid.
    Configuration(ConfigurationError),

    /// The supplied budget operation failed.
    Budget(BudgetError),

    /// A source identity could not be constructed.
    SourceIdentity(SourceIdentityError),

    /// The compilation has been cancelled.
    Cancelled,
}

impl fmt::Display for ContextError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Configuration(error) => {
                write!(
                    formatter,
                    "compilation-context configuration error: {error}"
                )
            }

            Self::Budget(error) => {
                write!(
                    formatter,
                    "compilation-context budget error: {error}"
                )
            }

            Self::SourceIdentity(error) => {
                write!(
                    formatter,
                    "compilation-context source error: {error}"
                )
            }

            Self::Cancelled => {
                formatter.write_str(
                    "compilation has been cancelled",
                )
            }
        }
    }
}

impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Configuration(error) => Some(error),
            Self::Budget(error) => Some(error),
            Self::SourceIdentity(error) => Some(error),
            Self::Cancelled => None,
        }
    }
}

impl From<ConfigurationError> for ContextError {
    fn from(error: ConfigurationError) -> Self {
        Self::Configuration(error)
    }
}

impl From<BudgetError> for ContextError {
    fn from(error: BudgetError) -> Self {
        Self::Budget(error)
    }
}

impl From<SourceIdentityError> for ContextError {
    fn from(error: SourceIdentityError) -> Self {
        Self::SourceIdentity(error)
    }
}

// ============================================================================
// IMMUTABLE CONTEXT STORAGE
// ============================================================================

/// Internal context data.
///
/// All configuration and source metadata is immutable after
/// construction.
///
/// Operational state is shared through its dedicated thread-safe
/// owners.
#[derive(Debug)]
struct CompilationContextInner {
    config: Arc<CompilerConfig>,
    source: Option<SourceIdentity>,
    budget: BudgetTracker,
    cancellation: CancellationToken,
}

/// Immutable, shareable context for one compilation operation.
///
/// Cloning this structure is inexpensive.
///
/// Clones share:
///
/// - Compiler configuration.
/// - Budget accounting.
/// - Cancellation state.
///
/// The context is suitable for use by:
///
/// - Sequential compiler stages.
/// - Parallel compilation workers.
/// - Incremental compilation.
/// - Compiler API sessions.
/// - IDE and language-server integrations.
///
/// The context does not own semantic or hardware state.
#[derive(Debug, Clone)]
pub struct CompilationContext {
    inner: Arc<CompilationContextInner>,
}

impl CompilationContext {
    /// Creates a compilation context using an existing canonical
    /// CompilerConfig.
    ///
    /// The default budget is unlimited.
    ///
    /// No new hardware target is selected by this constructor.
    ///
    /// The target value already contained in CompilerConfig is
    /// preserved exactly.
    ///
    /// Structural configuration validation is performed.
    pub fn new(
        config: CompilerConfig,
    ) -> Result<Self, ContextError> {
        CompilationContextBuilder::new(config).build()
    }

    /// Creates a context from an existing compiler configuration
    /// and an explicit source identity.
    pub fn for_source(
        config: CompilerConfig,
        source: SourceIdentity,
    ) -> Result<Self, ContextError> {
        CompilationContextBuilder::new(config)
            .source(source)
            .build()
    }

    /// Creates a context with an explicit source identity and
    /// resource budget.
    pub fn with_budget(
        config: CompilerConfig,
        source: Option<SourceIdentity>,
        budget: CompilationBudget,
    ) -> Result<Self, ContextError> {
        CompilationContextBuilder::new(config)
            .optional_source(source)
            .budget(budget)
            .build()
    }

    /// Returns the authoritative compiler configuration.
    ///
    /// The returned reference is immutable.
    #[must_use]
    pub fn config(&self) -> &CompilerConfig {
        self.inner.config.as_ref()
    }

    /// Returns a shared reference-counted compiler configuration.
    ///
    /// Cloning the returned Arc does not duplicate the configuration.
    #[must_use]
    pub fn shared_config(&self) -> Arc<CompilerConfig> {
        Arc::clone(&self.inner.config)
    }

    /// Returns the source identity when one was supplied.
    #[must_use]
    pub fn source(&self) -> Option<&SourceIdentity> {
        self.inner.source.as_ref()
    }

    /// Returns the shared budget tracker.
    ///
    /// Every clone of the context uses the same accounting ledger.
    #[must_use]
    pub fn budget_tracker(&self) -> &BudgetTracker {
        &self.inner.budget
    }

    /// Returns a clone of the shared budget tracker.
    ///
    /// The clone shares the original accounting ledger.
    #[must_use]
    pub fn shared_budget_tracker(&self) -> BudgetTracker {
        self.inner.budget.shared()
    }

    /// Returns a clone of the shared cancellation token.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.inner.cancellation.clone()
    }

    /// Returns whether compilation has been cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancellation.is_cancelled()
    }

    /// Checks whether compilation may continue.
    pub fn check_cancelled(&self) -> Result<(), ContextError> {
        self.inner.cancellation.check()
    }

    /// Accounts for compiler work against the shared budget.
    ///
    /// The operation is atomic with respect to other clones of the
    /// same BudgetTracker.
    ///
    /// Budget exhaustion is reported explicitly.
    ///
    /// No work is silently truncated.
    pub fn consume_budget(
        &self,
        dimension: BudgetDimension,
        amount: u128,
    ) -> Result<u128, ContextError> {
        self.check_cancelled()?;

        self.inner
            .budget
            .consume(dimension, amount)
            .map_err(ContextError::Budget)
    }

    /// Derives a context for another source unit in the same
    /// compilation operation.
    ///
    /// The new context shares:
    ///
    /// - Original compiler configuration.
    /// - Original budget accounting.
    /// - Original cancellation state.
    ///
    /// Only the source identity changes.
    ///
    /// This is useful for multi-file compilation.
    #[must_use]
    pub fn for_additional_source(
        &self,
        source: SourceIdentity,
    ) -> Self {
        Self {
            inner: Arc::new(CompilationContextInner {
                config: Arc::clone(&self.inner.config),
                source: Some(source),
                budget: self.inner.budget.shared(),
                cancellation: self.inner.cancellation.clone(),
            }),
        }
    }

    /// Returns the number of strong references to the immutable
    /// context storage.
    ///
    /// This is an observability method, not a synchronization
    /// or lifecycle guarantee.
    #[must_use]
    pub fn context_reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Returns the number of strong references to the shared
    /// compiler configuration.
    #[must_use]
    pub fn configuration_reference_count(&self) -> usize {
        Arc::strong_count(&self.inner.config)
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for CompilationContext.
///
/// The builder centralizes configuration validation and ensures
/// that all context instances are constructed consistently.
///
/// It does not define a competing CompilerConfig.
#[derive(Debug)]
pub struct CompilationContextBuilder {
    config: CompilerConfig,
    source: Option<SourceIdentity>,
    budget: CompilationBudget,
    cancellation: CancellationToken,
    validation_mode: ConfigurationValidationMode,
}

impl CompilationContextBuilder {
    /// Creates a builder from the existing canonical configuration.
    ///
    /// Defaults:
    ///
    /// - No source identity.
    /// - Unlimited explicit compiler budget.
    /// - Fresh cancellation token.
    /// - Standard configuration validation.
    pub fn new(config: CompilerConfig) -> Self {
        Self {
            config,
            source: None,
            budget: CompilationBudget::unlimited(),
            cancellation: CancellationToken::new(),
            validation_mode: ConfigurationValidationMode::Standard,
        }
    }

    /// Supplies the source identity.
    #[must_use]
    pub fn source(
        mut self,
        source: SourceIdentity,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Supplies an optional source identity.
    #[must_use]
    pub fn optional_source(
        mut self,
        source: Option<SourceIdentity>,
    ) -> Self {
        self.source = source;
        self
    }

    /// Supplies the compilation budget.
    ///
    /// The budget controls compiler-accounted work.
    ///
    /// It does not allocate or reserve physical resources.
    #[must_use]
    pub fn budget(
        mut self,
        budget: CompilationBudget,
    ) -> Self {
        self.budget = budget;
        self
    }

    /// Supplies an existing shared cancellation token.
    ///
    /// This allows a compiler session to coordinate cancellation
    /// across independently constructed context consumers.
    #[must_use]
    pub fn cancellation_token(
        mut self,
        cancellation: CancellationToken,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    /// Selects the configuration validation mode.
    ///
    /// Standard validation checks structural compatibility.
    ///
    /// Production validation rejects configuration modes that the
    /// current production pipeline does not support.
    #[must_use]
    pub fn validation_mode(
        mut self,
        mode: ConfigurationValidationMode,
    ) -> Self {
        self.validation_mode = mode;
        self
    }

    /// Requires production configuration validation.
    #[must_use]
    pub fn production_validation(self) -> Self {
        self.validation_mode(
            ConfigurationValidationMode::Production,
        )
    }

    /// Builds the immutable compilation context.
    ///
    /// Configuration is validated before the context is returned.
    ///
    /// No filesystem access, parsing, semantic analysis, target
    /// discovery, or backend invocation occurs here.
    pub fn build(self) -> Result<CompilationContext, ContextError> {
        validate_compiler_config(
            &self.config,
            self.validation_mode,
        )?;

        let inner = CompilationContextInner {
            config: Arc::new(self.config),
            source: self.source,
            budget: BudgetTracker::new(self.budget),
            cancellation: self.cancellation,
        };

        Ok(CompilationContext {
            inner: Arc::new(inner),
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::budget::{
        BudgetDimension,
        BudgetLimit,
    };

    use crate::compiler_types::{
        CompilationTarget,
        OptimizationLevel,
    };

    fn test_config() -> CompilerConfig {
        CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable-ir".to_owned(),
            ),
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: true,
            emit_ir: true,
            parallel: true,
        }
    }

    #[test]
    fn context_preserves_existing_configuration() {
        let config = test_config();

        let context = CompilationContext::new(
            config.clone(),
        )
        .expect("context construction should succeed");

        assert_eq!(
            context.config().target,
            config.target
        );

        assert_eq!(
            context.config().opt_level,
            OptimizationLevel::Basic
        );

        assert!(context.config().verify);
        assert!(context.config().parallel);
    }

    #[test]
    fn context_shares_configuration_between_clones() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let clone = context.clone();

        assert!(
            Arc::ptr_eq(
                &context.shared_config(),
                &clone.shared_config()
            )
        );
    }

    #[test]
    fn context_defaults_to_unlimited_budget() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        assert!(
            context.budget_tracker()
                .budget()
                .is_unlimited()
        );
    }

    #[test]
    fn context_preserves_source_identity() {
        let source = SourceIdentity::new(
            "examples/hybrid.zm",
        )
        .expect("source identity should be valid");

        let context = CompilationContext::for_source(
            test_config(),
            source.clone(),
        )
        .expect("context construction should succeed");

        assert_eq!(
            context.source(),
            Some(&source)
        );
    }

    #[test]
    fn filesystem_source_identity_does_not_require_io() {
        let source = SourceIdentity::from_path(
            "examples/classical.zm",
        )
        .expect("path identity should be valid");

        assert!(source.is_filesystem_backed());

        assert_eq!(
            source.origin(),
            SourceOrigin::Filesystem
        );

        assert_eq!(
            source.path(),
            Some(Path::new("examples/classical.zm"))
        );
    }

    #[test]
    fn generated_source_identity_is_supported() {
        let source = SourceIdentity::generated(
            "generated/module.zm",
        )
        .expect("generated identity should be valid");

        assert_eq!(
            source.origin(),
            SourceOrigin::Generated
        );
    }

    #[test]
    fn empty_source_identity_is_rejected() {
        let result = SourceIdentity::new("   ");

        assert!(matches!(
            result,
            Err(SourceIdentityError::EmptyName)
        ));
    }

    #[test]
    fn source_identity_supports_unicode() {
        let source = SourceIdentity::new(
            "examples/计算/程序.zm",
        )
        .expect("Unicode source identity should be supported");

        assert_eq!(
            source.logical_name(),
            "examples/计算/程序.zm"
        );
    }

    #[test]
    fn cancellation_is_shared_between_context_clones() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let clone = context.clone();

        assert!(!context.is_cancelled());

        assert!(context.cancellation_token().cancel());

        assert!(context.is_cancelled());
        assert!(clone.is_cancelled());

        assert!(matches!(
            clone.check_cancelled(),
            Err(ContextError::Cancelled)
        ));
    }

    #[test]
    fn repeated_cancellation_is_idempotent() {
        let token = CancellationToken::new();

        assert!(token.cancel());
        assert!(!token.cancel());
        assert!(token.is_cancelled());
    }

    #[test]
    fn budget_accounting_is_shared_between_context_clones() {
        let dimension = BudgetDimension::new(
            "compiler_work",
            "units",
        )
        .expect("budget dimension should be valid");

        let budget = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            );

        let context = CompilationContext::with_budget(
            test_config(),
            None,
            budget,
        )
        .expect("context construction should succeed");

        let clone = context.clone();

        context.consume_budget(
            dimension.clone(),
            40,
        )
        .expect("first consumption should succeed");

        assert_eq!(
            clone.budget_tracker()
                .consumed(&dimension)
                .expect("accounting should succeed"),
            40
        );
    }

    #[test]
    fn budget_exhaustion_is_reported() {
        let dimension = BudgetDimension::new(
            "compiler_work",
            "units",
        )
        .expect("budget dimension should be valid");

        let budget = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(10),
            );

        let context = CompilationContext::with_budget(
            test_config(),
            None,
            budget,
        )
        .expect("context construction should succeed");

        context.consume_budget(
            dimension.clone(),
            10,
        )
        .expect("budget should permit the configured amount");

        let result = context.consume_budget(
            dimension,
            1,
        );

        assert!(matches!(
            result,
            Err(ContextError::Budget(_))
        ));
    }

    #[test]
    fn additional_source_context_shares_budget() {
        let dimension = BudgetDimension::new(
            "ir_entities",
            "entities",
        )
        .expect("budget dimension should be valid");

        let budget = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            );

        let context = CompilationContext::with_budget(
            test_config(),
            None,
            budget,
        )
        .expect("context construction should succeed");

        let source = SourceIdentity::new(
            "module_b.zm",
        )
        .expect("source identity should be valid");

        let child = context.for_additional_source(source);

        context.consume_budget(
            dimension.clone(),
            25,
        )
        .expect("consumption should succeed");

        assert_eq!(
            child.budget_tracker()
                .consumed(&dimension)
                .expect("accounting should succeed"),
            25
        );
    }

    #[test]
    fn additional_source_context_shares_cancellation() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let source = SourceIdentity::new(
            "module_b.zm",
        )
        .expect("source identity should be valid");

        let child = context.for_additional_source(source);

        context.cancellation_token().cancel();

        assert!(child.is_cancelled());
    }

    #[test]
    fn production_validation_rejects_experimental_optimization() {
        let config = CompilerConfig {
            opt_level: OptimizationLevel::UltraAGI,
            ..test_config()
        };

        let result = CompilationContextBuilder::new(config)
            .production_validation()
            .build();

        assert!(matches!(
            result,
            Err(ContextError::Configuration(_))
        ));
    }

    #[test]
    fn standard_validation_does_not_change_configuration() {
        let config = CompilerConfig {
            opt_level: OptimizationLevel::UltraAGI,
            ..test_config()
        };

        let context = CompilationContext::new(config)
            .expect("standard validation should preserve compatibility");

        assert_eq!(
            context.config().opt_level,
            OptimizationLevel::UltraAGI
        );
    }

    #[test]
    fn context_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<CompilationContext>();
        assert_send_sync::<CancellationToken>();
        assert_send_sync::<SourceIdentity>();
    }

    #[test]
    fn context_cloning_is_inexpensive() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let clone = context.clone();

        assert_eq!(
            context.context_reference_count(),
            2
        );

        drop(clone);

        assert_eq!(
            context.context_reference_count(),
            1
        );
    }
}
