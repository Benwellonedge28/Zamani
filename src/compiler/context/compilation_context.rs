
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
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides immutable, shareable, compilation-wide context for one
//! logical compilation operation.
//!
//! The context coordinates:
//!
//! - The canonical CompilerConfig.
//! - Optional source identity.
//! - Shared operational budget accounting.
//! - Cooperative cancellation.
//! - Configuration validation.
//! - Context derivation for additional source units.
//! - Shared context construction for independently created consumers.
//!
//! # Architectural ownership
//!
//! Canonical owners:
//!
//! crate::compiler_types:
//!     CompilerConfig
//!
//! compiler::configuration::config:
//!     Configuration validation and validation modes.
//!
//! compiler::configuration::budget:
//!     CompilationBudget
//!     BudgetTracker
//!     BudgetDimension
//!     BudgetError
//!
//! compiler::context::compilation_state:
//!     Compilation lifecycle and legal state transitions.
//!
//! compiler::context::dependency_context:
//!     Compilation units and dependency relationships.
//!
//! compiler::context::provenance_context:
//!     Compilation provenance and transformation lineage.
//!
//! frontend:
//!     Canonical source maps, source spans and source locations.
//!
//! This module owns only the shared compilation context and its
//! cooperative cancellation token.
//!
//! It does NOT own:
//!
//! - Source parsing.
//! - AST construction.
//! - Name resolution.
//! - Type inference.
//! - Semantic validation.
//! - Ownership or borrow analysis.
//! - Effect analysis.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Compilation lifecycle transitions.
//! - Dependency graph construction.
//! - Provenance lineage.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Backend implementation.
//! - Linking.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal hardware-capacity ceilings.
//!
//! It does not establish fixed limits for:
//!
//! - CPUs, cores, threads or workers;
//! - GPUs, FPGAs, ASICs, QPUs or accelerators;
//! - Nodes, devices or network participants;
//! - Memory or storage;
//! - Qubits or logical qubits;
//! - Tensor dimensions or tensor rank;
//! - Register widths;
//! - Modules, functions or IR operations;
//! - Compilation stages;
//! - Source files or source units;
//! - Backends, providers or computing domains.
//!
//! Budgets are explicit operational accounting policies.
//!
//! An unlimited budget imposes no configured limit for the relevant
//! budget dimension. It does not promise infinite physical resources.
//!
//! The budget implementation uses its own documented numeric
//! representation and reports accounting overflow explicitly.
//!
//! # Sharing and concurrency
//!
//! Cloning CompilationContext shares:
//!
//! - Immutable compiler configuration;
//! - Budget accounting;
//! - Cancellation state.
//!
//! Additional-source contexts also share the same budget and
//! cancellation state.
//!
//! Configuration and source identity cannot be mutated through a
//! shared context.
//!
//! Budget accounting and cancellation are synchronized by their
//! authoritative implementations.
//!
//! # Cancellation semantics
//!
//! Cancellation is cooperative.
//!
//! A cancellation request is visible to all clones sharing the token.
//!
//! Cancellation does not forcibly terminate a thread, interrupt an
//! operating-system operation or roll back already-accounted work.
//!
//! A stage must check cancellation at safe interruption points.
//!
//! Cancellation may race with an already-running budget operation.
//! A successful budget operation is accounted even if cancellation
//! is requested concurrently.
//!
//! # Dependency direction
//!
//! compiler_types
//!       |
//!       v
//! configuration
//!       |
//!       v
//! compilation_context
//!       |
//!       +----> API and sessions
//!       +----> pipeline
//!       +----> analysis
//!       +----> optimization
//!       +----> parallel compilation
//!       +----> incremental compilation
//!
//! This module does not depend on pipeline orchestration.
//!
//! # Grammar boundary
//!
//! Source-level compilation syntax is owned by the grammar and frontend.
//!
//! Relevant grammar authorities include:
//!
//!     grammar/Zamani.g4
//!     grammar/compile/compile.g4
//!     grammar/compile/provenance.g4
//!     grammar/spec/provenance.md
//!
//! Those files describe source-language declarations and semantics.
//!
//! This Rust module represents runtime compiler-operation context.
//!
//! It does not parse grammar productions, construct AST nodes or
//! interpret source-level provenance declarations.

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

/// Public API version of the compilation-context contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - Grammar version;
/// - Compiler release version;
/// - AST version;
/// - Semantic-model version;
/// - IR version;
/// - Backend ABI version.
///
/// Increment when this module's public contract changes incompatibly.
pub const COMPILATION_CONTEXT_API_VERSION: u16 = 1;

// ============================================================================
// SOURCE IDENTITY
// ============================================================================

/// Describes the origin of a logical source unit.
///
/// This is descriptive compilation metadata.
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

/// Identifies the logical origin of source associated with a
/// compilation context.
///
/// The canonical frontend source-map implementation remains
/// responsible for:
///
/// - FileId;
/// - SourceFile;
/// - Span;
/// - byte positions;
/// - source mapping;
/// - source-location diagnostics.
///
/// This structure does not introduce a competing source map.
///
/// Filesystem paths are retained as PathBuf values. The logical
/// display name is descriptive and must not be used as a replacement
/// for the original filesystem path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceIdentity {
    logical_name: Arc<str>,
    path: Option<Arc<PathBuf>>,
    origin: SourceOrigin,
}

/// Errors produced while constructing source identities.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SourceIdentityError {
    /// The logical name or path is empty.
    EmptyName,

    /// The logical name contains control characters.
    InvalidName,
}

impl fmt::Display for SourceIdentityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str(
                "source identity requires a non-empty logical name",
            ),

            Self::InvalidName => formatter.write_str(
                "source identity contains control characters",
            ),
        }
    }
}

impl Error for SourceIdentityError {}

impl SourceIdentity {
    /// Creates an identity for an in-memory source unit.
    pub fn new(
        logical_name: impl Into<Arc<str>>,
    ) -> Result<Self, SourceIdentityError> {
        Self::with_origin(logical_name, SourceOrigin::InMemory)
    }

    /// Creates an identity with an explicit origin.
    ///
    /// The logical name must be non-empty and must not contain
    /// control characters.
    pub fn with_origin(
        logical_name: impl Into<Arc<str>>,
        origin: SourceOrigin,
    ) -> Result<Self, SourceIdentityError> {
        let logical_name = logical_name.into();

        Self::validate_logical_name(&logical_name)?;

        Ok(Self {
            logical_name,
            path: None,
            origin,
        })
    }

    /// Creates an identity for a filesystem-backed source.
    ///
    /// This constructor does not access the filesystem.
    ///
    /// The original PathBuf is preserved, including non-UTF-8
    /// platform paths.
    ///
    /// The display name is descriptive metadata. It is not used
    /// to reopen or identify the original filesystem object.
    pub fn from_path(
        path: impl Into<PathBuf>,
    ) -> Result<Self, SourceIdentityError> {
        let path = path.into();

        if path.as_os_str().is_empty() {
            return Err(SourceIdentityError::EmptyName);
        }

        let logical_name: Arc<str> =
            Arc::from(path.display().to_string());

        Self::validate_logical_name(&logical_name)?;

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
        Self::with_origin(logical_name, SourceOrigin::Generated)
    }

    /// Validates a logical source name.
    fn validate_logical_name(
        logical_name: &str,
    ) -> Result<(), SourceIdentityError> {
        if logical_name.trim().is_empty() {
            return Err(SourceIdentityError::EmptyName);
        }

        if logical_name.chars().any(char::is_control) {
            return Err(SourceIdentityError::InvalidName);
        }

        Ok(())
    }

    /// Returns the logical source name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        self.logical_name.as_ref()
    }

    /// Returns the original filesystem path, when available.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().map(|path| path.as_path())
    }

    /// Returns the source origin.
    #[must_use]
    pub const fn origin(&self) -> SourceOrigin {
        self.origin
    }

    /// Returns whether this identity is filesystem-backed.
    #[must_use]
    pub fn is_filesystem_backed(&self) -> bool {
        self.path.is_some()
    }
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Thread-safe cooperative cancellation token.
///
/// Clones share the same cancellation state.
///
/// A session, API caller or orchestrator can request cancellation
/// while compiler stages or parallel workers are executing.
///
/// Cancellation is cooperative:
///
/// - It is not thread termination.
/// - It is not an operating-system interrupt.
/// - It does not roll back completed work.
/// - It does not automatically update CompilationStateTracker.
///
/// The lifecycle owner observes cancellation and performs the
/// appropriate lifecycle transition.
///
/// A token cannot be reset. Create a new token for a new independent
/// compilation operation.
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
    /// Returns true only when this call changes the token from
    /// uncancelled to cancelled.
    ///
    /// Repeated cancellation requests are idempotent.
    pub fn cancel(&self) -> bool {
        !self.cancelled.swap(true, Ordering::AcqRel)
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Checks cancellation.
    ///
    /// Returns ContextError::Cancelled if cancellation has
    /// already been requested.
    pub fn check(&self) -> Result<(), ContextError> {
        if self.is_cancelled() {
            Err(ContextError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Returns whether another token shares this cancellation state.
    ///
    /// This is an identity check, not a synchronization operation.
    #[must_use]
    pub fn shares_state_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.cancelled, &other.cancelled)
    }
}

// ============================================================================
// CONTEXT ERRORS
// ============================================================================

/// Errors produced by compilation-context operations.
///
/// The original structured error is preserved.
///
/// This module does not flatten errors into strings or create a
/// competing diagnostic representation.
///
/// Callers may translate these errors into the canonical compiler
/// or frontend diagnostic system.
#[derive(Debug)]
#[non_exhaustive]
pub enum ContextError {
    /// The supplied compiler configuration is invalid.
    Configuration(ConfigurationError),

    /// A budget operation failed.
    Budget(BudgetError),

    /// A source identity could not be constructed.
    SourceIdentity(SourceIdentityError),

    /// Compilation cancellation has been requested.
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
// INTERNAL STORAGE
// ============================================================================

/// Immutable data shared by clones of one compilation context.
#[derive(Debug)]
struct CompilationContextInner {
    config: Arc<CompilerConfig>,
    source: Option<SourceIdentity>,
    budget: BudgetTracker,
    cancellation: CancellationToken,
}

/// Immutable, shareable context for one logical compilation operation.
///
/// Cloning a context is inexpensive and shares its operational state.
///
/// Shared state:
///
/// - Compiler configuration;
/// - Budget accounting;
/// - Cancellation.
///
/// Source identity is immutable within one context instance.
///
/// A derived additional-source context shares the original
/// configuration, budget and cancellation while associating a
/// different source identity.
#[derive(Debug, Clone)]
pub struct CompilationContext {
    inner: Arc<CompilationContextInner>,
}

impl CompilationContext {
    /// Creates a context using the canonical CompilerConfig.
    ///
    /// Defaults:
    ///
    /// - No source identity;
    /// - Unlimited explicit compiler budget;
    /// - Fresh cancellation token;
    /// - Standard configuration validation.
    ///
    /// The target already present in CompilerConfig is preserved.
    ///
    /// This constructor does not discover hardware, select a backend,
    /// allocate resources or execute compilation.
    pub fn new(
        config: CompilerConfig,
    ) -> Result<Self, ContextError> {
        CompilationContextBuilder::new(config).build()
    }

    /// Creates a context associated with one source unit.
    pub fn for_source(
        config: CompilerConfig,
        source: SourceIdentity,
    ) -> Result<Self, ContextError> {
        CompilationContextBuilder::new(config)
            .source(source)
            .build()
    }

    /// Creates a context with an explicit optional source and budget.
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

    /// Returns the canonical immutable compiler configuration.
    #[must_use]
    pub fn config(&self) -> &CompilerConfig {
        self.inner.config.as_ref()
    }

    /// Returns a shared reference-counted compiler configuration.
    ///
    /// Cloning this Arc does not clone the underlying configuration.
    #[must_use]
    pub fn shared_config(&self) -> Arc<CompilerConfig> {
        Arc::clone(&self.inner.config)
    }

    /// Returns the source identity associated with this context.
    #[must_use]
    pub fn source(&self) -> Option<&SourceIdentity> {
        self.inner.source.as_ref()
    }

    /// Returns the shared budget tracker.
    ///
    /// All clones and derived additional-source contexts use the
    /// same accounting ledger.
    #[must_use]
    pub fn budget_tracker(&self) -> &BudgetTracker {
        &self.inner.budget
    }

    /// Returns a shared clone of the budget tracker.
    ///
    /// The returned tracker uses the same accounting ledger.
    #[must_use]
    pub fn shared_budget_tracker(&self) -> BudgetTracker {
        self.inner.budget.shared()
    }

    /// Returns a clone of the shared cancellation token.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.inner.cancellation.clone()
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancellation.is_cancelled()
    }

    /// Checks whether this compilation may continue.
    pub fn check_cancelled(&self) -> Result<(), ContextError> {
        self.inner.cancellation.check()
    }

    /// Accounts for compiler work against the shared budget.
    ///
    /// The budget tracker performs its own synchronized accounting.
    ///
    /// Cancellation is checked before accounting. Cancellation may
    /// still race with the accounting operation; if accounting
    /// succeeds, the work remains accounted.
    ///
    /// Budget exhaustion and accounting errors are returned
    /// explicitly. Work is never silently truncated.
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

    /// Derives a context for another source unit belonging to the
    /// same logical compilation operation.
    ///
    /// The derived context shares:
    ///
    /// - The canonical compiler configuration;
    /// - The original budget accounting ledger;
    /// - The original cancellation state.
    ///
    /// Only the source identity changes.
    ///
    /// This is suitable for multi-file compilation and parallel
    /// compilation units.
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

    /// Returns the number of strong references to this context's
    /// immutable storage.
    ///
    /// This is an observability method only. It must not be used
    /// to make correctness or lifecycle decisions.
    #[must_use]
    pub fn context_reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Returns the number of strong references to the shared
    /// compiler configuration.
    ///
    /// This is an observability method only.
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
/// The builder ensures that configuration validation and shared
/// service association happen before a context becomes available.
///
/// It does not define a competing CompilerConfig.
///
/// A caller may supply either:
///
/// - A CompilationBudget from which a new tracker is constructed; or
/// - An existing BudgetTracker whose accounting ledger is shared.
///
/// Supplying a budget after a shared tracker replaces the tracker
/// selection. Supplying a shared tracker after a budget replaces
/// the budget selection.
#[derive(Debug)]
pub struct CompilationContextBuilder {
    config: CompilerConfig,
    source: Option<SourceIdentity>,
    budget: CompilationBudget,
    shared_budget: Option<BudgetTracker>,
    cancellation: CancellationToken,
    validation_mode: ConfigurationValidationMode,
}

impl CompilationContextBuilder {
    /// Creates a builder from the canonical compiler configuration.
    ///
    /// Defaults:
    ///
    /// - No source identity;
    /// - Unlimited explicit budget;
    /// - No externally supplied budget tracker;
    /// - Fresh cancellation token;
    /// - Standard configuration validation.
    pub fn new(config: CompilerConfig) -> Self {
        Self {
            config,
            source: None,
            budget: CompilationBudget::unlimited(),
            shared_budget: None,
            cancellation: CancellationToken::new(),
            validation_mode: ConfigurationValidationMode::Standard,
        }
    }

    /// Associates a source identity with the context.
    #[must_use]
    pub fn source(
        mut self,
        source: SourceIdentity,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Associates an optional source identity with the context.
    #[must_use]
    pub fn optional_source(
        mut self,
        source: Option<SourceIdentity>,
    ) -> Self {
        self.source = source;
        self
    }

    /// Supplies a budget configuration.
    ///
    /// A new tracker is created when build() is called.
    ///
    /// The budget controls compiler-accounted work. It does not
    /// allocate or reserve physical resources.
    #[must_use]
    pub fn budget(
        mut self,
        budget: CompilationBudget,
    ) -> Self {
        self.budget = budget;
        self.shared_budget = None;
        self
    }

    /// Supplies an existing shared budget tracker.
    ///
    /// The constructed context will use the same accounting ledger
    /// as the supplied tracker.
    ///
    /// This supports multiple independently constructed context
    /// consumers participating in one compilation operation.
    #[must_use]
    pub fn shared_budget_tracker(
        mut self,
        tracker: BudgetTracker,
    ) -> Self {
        self.shared_budget = Some(tracker);
        self
    }

    /// Supplies an existing cancellation token.
    ///
    /// This allows API sessions, orchestration and independently
    /// constructed context consumers to share cancellation.
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
    /// Standard validation preserves compatibility while checking
    /// structural validity.
    ///
    /// Production validation applies the restrictions defined by
    /// the canonical configuration subsystem.
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
    /// Configuration validation occurs before the context is
    /// returned.
    ///
    /// This method performs no:
    ///
    /// - Filesystem access;
    /// - Parsing;
    /// - Semantic analysis;
    /// - Hardware discovery;
    /// - Physical resource allocation;
    /// - Backend invocation;
    /// - Runtime execution.
    ///
    /// An already-cancelled token is preserved. The caller can
    /// observe it through check_cancelled().
    pub fn build(self) -> Result<CompilationContext, ContextError> {
        validate_compiler_config(
            &self.config,
            self.validation_mode,
        )?;

        let budget = match self.shared_budget {
            Some(tracker) => tracker,
            None => BudgetTracker::new(self.budget),
        };

        let inner = CompilationContextInner {
            config: Arc::new(self.config),
            source: self.source,
            budget,
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

    fn test_dimension() -> BudgetDimension {
        BudgetDimension::new(
            "compiler_work",
            "units",
        )
        .expect("test budget dimension should be valid")
    }

    #[test]
    fn context_preserves_existing_configuration() {
        let config = test_config();

        let context = CompilationContext::new(config.clone())
            .expect("context construction should succeed");

        assert_eq!(context.config().target, config.target);
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
            context
                .budget_tracker()
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

        assert_eq!(context.source(), Some(&source));
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
        .expect("generated source identity should be valid");

        assert_eq!(
            source.origin(),
            SourceOrigin::Generated
        );
    }

    #[test]
    fn unspecified_source_origin_is_supported() {
        let source = SourceIdentity::with_origin(
            "module.zm",
            SourceOrigin::Unspecified,
        )
        .expect("source identity should be valid");

        assert_eq!(
            source.origin(),
            SourceOrigin::Unspecified
        );
    }

    #[test]
    fn empty_source_identity_is_rejected() {
        assert!(matches!(
            SourceIdentity::new("   "),
            Err(SourceIdentityError::EmptyName)
        ));
    }

    #[test]
    fn control_characters_in_logical_names_are_rejected() {
        assert!(matches!(
            SourceIdentity::new("module\nname.zm"),
            Err(SourceIdentityError::InvalidName)
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
    fn cancellation_tokens_can_report_shared_identity() {
        let first = CancellationToken::new();
        let second = first.clone();
        let independent = CancellationToken::new();

        assert!(first.shares_state_with(&second));
        assert!(!first.shares_state_with(&independent));
    }

    #[test]
    fn pre_cancelled_token_remains_cancelled_after_build() {
        let token = CancellationToken::new();
        token.cancel();

        let context = CompilationContextBuilder::new(test_config())
            .cancellation_token(token)
            .build()
            .expect("context construction should succeed");

        assert!(context.is_cancelled());

        assert!(matches!(
            context.check_cancelled(),
            Err(ContextError::Cancelled)
        ));
    }

    #[test]
    fn budget_accounting_is_shared_between_context_clones() {
        let dimension = test_dimension();

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

        context
            .consume_budget(dimension.clone(), 40)
            .expect("budget consumption should succeed");

        assert_eq!(
            clone
                .budget_tracker()
                .consumed(&dimension)
                .expect("accounting should succeed"),
            40
        );
    }

    #[test]
    fn independently_constructed_contexts_can_share_budget() {
        let dimension = test_dimension();

        let budget = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            );

        let tracker = BudgetTracker::new(budget);

        let first = CompilationContextBuilder::new(test_config())
            .shared_budget_tracker(tracker.shared())
            .build()
            .expect("first context should build");

        let second = CompilationContextBuilder::new(test_config())
            .shared_budget_tracker(tracker.shared())
            .build()
            .expect("second context should build");

        first
            .consume_budget(dimension.clone(), 35)
            .expect("first context should consume budget");

        assert_eq!(
            second
                .budget_tracker()
                .consumed(&dimension)
                .expect("shared accounting should succeed"),
            35
        );
    }

    #[test]
    fn budget_exhaustion_is_reported() {
        let dimension = test_dimension();

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

        context
            .consume_budget(dimension.clone(), 10)
            .expect("configured budget should permit this amount");

        let result = context.consume_budget(dimension, 1);

        assert!(matches!(
            result,
            Err(ContextError::Budget(_))
        ));
    }

    #[test]
    fn cancellation_prevents_new_budget_consumption() {
        let dimension = test_dimension();

        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        context.cancellation_token().cancel();

        assert!(matches!(
            context.consume_budget(dimension, 1),
            Err(ContextError::Cancelled)
        ));
    }

    #[test]
    fn additional_source_context_shares_budget() {
        let dimension = test_dimension();

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

        let source = SourceIdentity::new("module_b.zm")
            .expect("source identity should be valid");

        let child = context.for_additional_source(source);

        context
            .consume_budget(dimension.clone(), 25)
            .expect("budget consumption should succeed");

        assert_eq!(
            child
                .budget_tracker()
                .consumed(&dimension)
                .expect("accounting should succeed"),
            25
        );
    }

    #[test]
    fn additional_source_context_shares_cancellation() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let source = SourceIdentity::new("module_b.zm")
            .expect("source identity should be valid");

        let child = context.for_additional_source(source);

        context.cancellation_token().cancel();

        assert!(child.is_cancelled());
    }

    #[test]
    fn additional_source_context_preserves_configuration() {
        let context = CompilationContext::new(test_config())
            .expect("context construction should succeed");

        let source = SourceIdentity::new("module_b.zm")
            .expect("source identity should be valid");

        let child = context.for_additional_source(source);

        assert_eq!(
            child.config().target,
            context.config().target
        );

        assert_eq!(
            child.config().opt_level,
            context.config().opt_level
        );
    }

    #[test]
    fn production_validation_rejects_unsupported_optimization() {
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
    fn standard_validation_preserves_legacy_configuration() {
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
