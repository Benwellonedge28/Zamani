
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Stage Contract.
//!
//! Path: src/compiler/pipeline/stage.rs
//!
//! Rust: Edition 2021, Rust 1.97+
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines the canonical, domain-independent contract implemented by
//! individual compiler pipeline stages.
//!
//! A stage represents one independently executable unit of compiler work.
//!
//! This module owns:
//!
//! - Stage identity.
//! - Stage metadata.
//! - Dependency declarations.
//! - Determinism declarations.
//! - Concurrency declarations.
//! - Type-erased input and output transport.
//! - Conditional execution decisions.
//! - Shared execution context.
//! - Structured stage errors.
//! - Stage invocation contracts.
//!
//! This module does not own:
//!
//! - Pipeline registration.
//! - Dependency graph construction.
//! - Pipeline ordering.
//! - Pipeline orchestration.
//! - Recovery execution.
//! - Compilation lifecycle.
//! - Source semantics.
//! - AST.
//! - Canonical IR.
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
//! CompilationContext:
//!     compiler::context::compilation_context
//!
//! CompilationPhase:
//!     compiler::context::compilation_state
//!
//! BudgetDimension:
//!     compiler::configuration::budget
//!
//! Stage implementations:
//!     compiler::pipeline::stages
//!
//! Dependency ordering:
//!     compiler::pipeline::ordering
//!
//! Stage execution:
//!     compiler::pipeline::execution
//!
//! Pipeline failures:
//!     compiler::pipeline::failure
//!
//! Recovery:
//!     compiler::pipeline::recovery
//!
//! Orchestration:
//!     compiler::pipeline::orchestration
//!
//! # POCO-REAF
//!
//! No universal ceilings are imposed on stages, dependencies,
//! hardware, devices, memory, qubits, tensors, modules or operations.
//!
//! Operational constraints are managed by CompilationContext.
//!
//! Physical constraints are discovered and evaluated by the
//! appropriate target and runtime authorities.
//!
//! An unlimited budget does not imply infinite physical resources.
//!
//! # Grammar boundary
//!
//! Source-language compilation intent belongs to:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/compile/compile.g4
//!
//! Source semantics belong to the frontend semantic authority.
//!
//! A source-level compilation declaration is not a StageId.
//!
//! Stage activation must be based on validated compiler input.
//!
//! # Determinism
//!
//! Stage metadata and dependency declarations are stable.
//!
//! Actual deterministic execution is coordinated by the pipeline executor.
//!
//! # Thread safety
//!
//! Stage implementations must be Send + Sync.
//!
//! Payloads transported between stages must be Send + Sync + 'static.
//!
//! # Error handling
//!
//! Stage errors remain explicit and typed.
//!
//! Retryability is advisory evidence for recovery policy.
//!
//! This module never retries a stage automatically.

use std::any::Any;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::configuration::budget::BudgetDimension;

use crate::compiler::context::compilation_context::CompilationContext;

use crate::compiler::context::compilation_state::CompilationPhase;

// ============================================================================
// API VERSION
// ============================================================================

/// Public version of the canonical pipeline-stage contract.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic-model version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
pub const STAGE_API_VERSION: u16 = 1;

// ============================================================================
// STAGE IDENTITY
// ============================================================================

/// Identifies one logical compiler pipeline stage.
///
/// Identifiers are dynamically sized and extensible.
///
/// They are not hardware identifiers.
///
/// Examples:
///
/// zamani.compiler.frontend.parse
/// zamani.compiler.semantic.analysis
/// zamani.compiler.ir.verification
/// zamani.compiler.optimization
/// zamani.compiler.lowering
/// zamani.domain.quantum.specialization
/// zamani.extension.example.custom_stage
///
/// No universal identifier-length limit is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StageId(Arc<str>);

/// Errors associated with stage identity construction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageIdError {
    /// Identifier is empty or whitespace-only.
    Empty,

    /// Identifier contains control characters.
    InvalidControlCharacter,
}

impl fmt::Display for StageIdError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str(
                    "stage identifier cannot be empty",
                )
            }

            Self::InvalidControlCharacter => {
                formatter.write_str(
                    "stage identifier cannot contain control characters",
                )
            }
        }
    }
}

impl Error for StageIdError {}

impl StageId {
    /// Creates a validated logical stage identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, StageIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StageIdError::Empty);
        }

        if value.chars().any(char::is_control) {
            return Err(
                StageIdError::InvalidControlCharacter,
            );
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns a shared identifier representation.
    #[must_use]
    pub fn as_arc(&self) -> Arc<str> {
        Arc::clone(&self.0)
    }
}

impl fmt::Display for StageId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// STAGE DETERMINISM
// ============================================================================

/// Declares the reproducibility characteristics of a stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageDeterminism {
    /// Identical declared inputs, configuration and environment
    /// produce identical observable outputs.
    ///
    /// External inputs must be included in the declared input identity.
    Deterministic,

    /// Results depend on explicitly declared external measurements,
    /// environmental inputs or randomized decisions.
    ///
    /// These dependencies must be represented in provenance.
    InputDependent,

    /// Reproducibility cannot be guaranteed.
    ///
    /// Such outputs must not be treated as deterministic cache results.
    Nondeterministic,
}

// ============================================================================
// STAGE CONCURRENCY
// ============================================================================

/// Declares the concurrency guarantees of a stage implementation.
///
/// These declarations do not create threads.
///
/// The execution subsystem is responsible for concurrency admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageConcurrency {
    /// No concurrent invocation of this logical stage is permitted,
    /// including invocations through different instances.
    Exclusive,

    /// One stage instance cannot be invoked concurrently.
    ///
    /// Different instances may be invoked concurrently.
    InstanceExclusive,

    /// Concurrent invocation of the same stage instance is supported.
    Concurrent,
}

// ============================================================================
// STAGE DESCRIPTOR
// ============================================================================

/// Immutable metadata describing one compiler stage.
///
/// Dependencies are kept in deterministic identifier order.
///
/// This descriptor does not contain implementation-specific hardware
/// assumptions or source-language semantic information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageDescriptor {
    id: StageId,

    name: Arc<str>,

    version: Arc<str>,

    description: Arc<str>,

    phase: Option<CompilationPhase>,

    dependencies: Vec<StageId>,

    determinism: StageDeterminism,

    concurrency: StageConcurrency,

    budget_dimension: Option<BudgetDimension>,
}

impl StageDescriptor {
    /// Creates a stage descriptor.
    ///
    /// Name and version must be non-empty.
    ///
    /// Description may be empty.
    ///
    /// Dependency existence and cycle detection are performed
    /// by pipeline::ordering.
    pub fn new(
        id: StageId,
        name: impl Into<Arc<str>>,
        version: impl Into<Arc<str>>,
        description: impl Into<Arc<str>>,
    ) -> Result<Self, StageDescriptorError> {
        let name = name.into();
        let version = version.into();
        let description = description.into();

        validate_descriptor_text(&name, "name")?;

        validate_descriptor_text(&version, "version")?;

        if description.chars().any(char::is_control) {
            return Err(
                StageDescriptorError::InvalidText {
                    field: "description",
                },
            );
        }

        Ok(Self {
            id,
            name,
            version,
            description,
            phase: None,
            dependencies: Vec::new(),
            determinism: StageDeterminism::Deterministic,
            concurrency: StageConcurrency::InstanceExclusive,
            budget_dimension: None,
        })
    }

    /// Associates a standard compilation phase.
    #[must_use]
    pub fn with_phase(
        mut self,
        phase: CompilationPhase,
    ) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Declares one prerequisite stage.
    ///
    /// A prerequisite must complete successfully before this stage
    /// can consume its output.
    ///
    /// The dependency must be registered in the active pipeline.
    pub fn with_dependency(
        mut self,
        dependency: StageId,
    ) -> Result<Self, StageDescriptorError> {
        if dependency == self.id {
            return Err(
                StageDescriptorError::SelfDependency {
                    stage: self.id.clone(),
                },
            );
        }

        if self.dependencies.contains(&dependency) {
            return Err(
                StageDescriptorError::DuplicateDependency {
                    stage: self.id.clone(),
                    dependency,
                },
            );
        }

        self.dependencies.push(dependency);

        // Dependency metadata must not depend on the order in which
        // callers happened to declare the prerequisites.
        self.dependencies.sort_unstable();

        Ok(self)
    }

    /// Declares multiple prerequisite stages.
    ///
    /// Duplicate and self-dependencies are rejected.
    pub fn with_dependencies(
        mut self,
        dependencies: impl IntoIterator<Item = StageId>,
    ) -> Result<Self, StageDescriptorError> {
        for dependency in dependencies {
            self = self.with_dependency(dependency)?;
        }

        Ok(self)
    }

    /// Declares the actual determinism guarantee.
    #[must_use]
    pub fn with_determinism(
        mut self,
        determinism: StageDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Declares the concurrency guarantee.
    #[must_use]
    pub fn with_concurrency(
        mut self,
        concurrency: StageConcurrency,
    ) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Associates the default operational budget dimension.
    ///
    /// This is descriptive metadata.
    ///
    /// Explicit budget accounting remains the responsibility of
    /// the executing stage.
    #[must_use]
    pub fn with_budget_dimension(
        mut self,
        dimension: BudgetDimension,
    ) -> Self {
        self.budget_dimension = Some(dimension);
        self
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn id(&self) -> &StageId {
        &self.id
    }

    /// Returns the stage name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the stage version.
    #[must_use]
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Returns the stage description.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    /// Returns the associated compilation phase.
    #[must_use]
    pub const fn phase(&self) -> Option<CompilationPhase> {
        self.phase
    }

    /// Returns dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &[StageId] {
        &self.dependencies
    }

    /// Returns the determinism guarantee.
    #[must_use]
    pub const fn determinism(&self) -> StageDeterminism {
        self.determinism
    }

    /// Returns the concurrency guarantee.
    #[must_use]
    pub const fn concurrency(&self) -> StageConcurrency {
        self.concurrency
    }

    /// Returns the optional default budget dimension.
    #[must_use]
    pub fn budget_dimension(&self) -> Option<&BudgetDimension> {
        self.budget_dimension.as_ref()
    }

    /// Revalidates descriptor invariants.
    ///
    /// Useful at extension-registration and pipeline-construction
    /// boundaries.
    pub fn validate(&self) -> Result<(), StageDescriptorError> {
        validate_descriptor_text(&self.name, "name")?;

        validate_descriptor_text(&self.version, "version")?;

        if self.description.chars().any(char::is_control) {
            return Err(
                StageDescriptorError::InvalidText {
                    field: "description",
                },
            );
        }

        for dependency in &self.dependencies {
            if dependency == &self.id {
                return Err(
                    StageDescriptorError::SelfDependency {
                        stage: self.id.clone(),
                    },
                );
            }
        }

        for pair in self.dependencies.windows(2) {
            if pair[0] == pair[1] {
                return Err(
                    StageDescriptorError::DuplicateDependency {
                        stage: self.id.clone(),
                        dependency: pair[0].clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// DESCRIPTOR VALIDATION
// ============================================================================

fn validate_descriptor_text(
    value: &str,
    field: &'static str,
) -> Result<(), StageDescriptorError> {
    if value.trim().is_empty() {
        return Err(
            StageDescriptorError::EmptyText {
                field,
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            StageDescriptorError::InvalidText {
                field,
            },
        );
    }

    Ok(())
}

/// Errors encountered when constructing or validating stage metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageDescriptorError {
    EmptyText {
        field: &'static str,
    },

    InvalidText {
        field: &'static str,
    },

    SelfDependency {
        stage: StageId,
    },

    DuplicateDependency {
        stage: StageId,
        dependency: StageId,
    },
}

impl fmt::Display for StageDescriptorError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(
                    formatter,
                    "stage descriptor {field} cannot be empty"
                )
            }

            Self::InvalidText { field } => {
                write!(
                    formatter,
                    "stage descriptor {field} contains control characters"
                )
            }

            Self::SelfDependency { stage } => {
                write!(
                    formatter,
                    "stage {stage} cannot depend on itself"
                )
            }

            Self::DuplicateDependency {
                stage,
                dependency,
            } => {
                write!(
                    formatter,
                    "stage {stage} declares duplicate dependency {dependency}"
                )
            }
        }
    }
}

impl Error for StageDescriptorError {}

// ============================================================================
// TYPE-ERASED INPUT
// ============================================================================

/// Shared, type-erased input transported between pipeline stages.
///
/// Domain-specific input types remain owned by their canonical
/// repository subsystems.
///
/// Values must implement Send + Sync + 'static.
#[derive(Clone, Default)]
pub struct StageInput {
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl StageInput {
    /// Creates an empty input.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            payload: None,
        }
    }

    /// Creates an input containing a typed value.
    pub fn new<T>(value: T) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            payload: Some(Arc::new(value)),
        }
    }

    /// Returns whether the input is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.is_none()
    }

    /// Returns a typed reference to the payload.
    #[must_use]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.payload.as_ref()?.downcast_ref::<T>()
    }

    /// Returns a shared reference-counted payload.
    #[must_use]
    pub fn shared_payload(
        &self,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        self.payload.as_ref().map(Arc::clone)
    }
}

impl fmt::Debug for StageInput {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("StageInput")
            .field("has_payload", &self.payload.is_some())
            .finish()
    }
}

// ============================================================================
// TYPE-ERASED OUTPUT
// ============================================================================

/// Shared, type-erased output produced by one compiler stage.
///
/// Outputs preserve the ownership of the domain-specific representation.
///
/// Examples include:
///
/// - Frontend results.
/// - Validated semantic models.
/// - Canonical ZUIR modules.
/// - Classical IR.
/// - Quantum IR.
/// - HDLIR.
/// - AI computation representations.
/// - Lowered representations.
/// - Artifact references.
///
/// This type does not define those representations.
#[derive(Clone, Default)]
pub struct StageOutput {
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl StageOutput {
    /// Creates an empty successful output.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            payload: None,
        }
    }

    /// Creates an output containing a typed value.
    pub fn new<T>(value: T) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            payload: Some(Arc::new(value)),
        }
    }

    /// Returns whether the output is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.is_none()
    }

    /// Returns a typed reference to the output.
    #[must_use]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.payload.as_ref()?.downcast_ref::<T>()
    }

    /// Returns a shared reference-counted payload.
    #[must_use]
    pub fn shared_payload(
        &self,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        self.payload.as_ref().map(Arc::clone)
    }
}

impl fmt::Debug for StageOutput {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("StageOutput")
            .field("has_payload", &self.payload.is_some())
            .finish()
    }
}

// ============================================================================
// CONDITIONAL EXECUTION DECISION
// ============================================================================

/// Decision made before executing a stage.
///
/// Domain-specific stages may be conditional.
///
/// For example, quantum lowering must not be forced upon a program
/// that does not contain quantum computation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageDecision {
    /// Execute the stage.
    Run,

    /// Explicitly skip the stage.
    ///
    /// A skipped stage is not a completed stage.
    Skip {
        reason: Arc<str>,
    },
}

impl StageDecision {
    /// Creates a validated skip decision.
    pub fn skip(
        reason: impl Into<Arc<str>>,
    ) -> Result<Self, StageError> {
        let reason = reason.into();

        if reason.trim().is_empty() {
            return Err(
                StageError::new(
                    StageErrorKind::InvalidDecision,
                    "a skipped stage must provide a non-empty reason",
                ),
            );
        }

        if reason.chars().any(char::is_control) {
            return Err(
                StageError::new(
                    StageErrorKind::InvalidDecision,
                    "a skip reason cannot contain control characters",
                ),
            );
        }

        Ok(Self::Skip {
            reason,
        })
    }

    /// Returns whether the stage should execute.
    #[must_use]
    pub const fn should_run(&self) -> bool {
        matches!(self, Self::Run)
    }
}

// ============================================================================
// SHARED STAGE EXECUTION CONTEXT
// ============================================================================

/// Shared context for one stage invocation.
///
/// All stages receive the same compilation context.
///
/// Consequently, they share:
///
/// - Compiler configuration.
/// - Operational budget accounting.
/// - Cancellation state.
///
/// This module does not construct replacement compilation contexts.
pub struct StageExecutionContext<'a> {
    compilation: &'a CompilationContext,

    descriptor: &'a StageDescriptor,
}

impl<'a> StageExecutionContext<'a> {
    /// Creates an execution context.
    #[must_use]
    pub fn new(
        compilation: &'a CompilationContext,
        descriptor: &'a StageDescriptor,
    ) -> Self {
        Self {
            compilation,
            descriptor,
        }
    }

    /// Returns the shared compilation context.
    #[must_use]
    pub fn compilation(&self) -> &'a CompilationContext {
        self.compilation
    }

    /// Returns the current stage identity.
    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        self.descriptor.id()
    }

    /// Returns the current stage descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &'a StageDescriptor {
        self.descriptor
    }

    /// Returns the default budget dimension, if configured.
    #[must_use]
    pub fn budget_dimension(&self) -> Option<&BudgetDimension> {
        self.descriptor.budget_dimension()
    }

    /// Checks cooperative cancellation.
    pub fn check_cancelled(&self) -> Result<(), StageError> {
        self.compilation
            .check_cancelled()
            .map_err(|error| {
                StageError::new(
                    StageErrorKind::Cancelled,
                    error.to_string(),
                )
            })
    }

    /// Accounts for explicitly measured compiler work.
    ///
    /// Accounting uses the authoritative shared CompilationContext.
    ///
    /// The amount is expressed in the unit defined by BudgetDimension.
    ///
    /// A failed accounting operation is never treated as successful work.
    pub fn consume_budget(
        &self,
        dimension: BudgetDimension,
        amount: u128,
    ) -> Result<u128, StageError> {
        self.check_cancelled()?;

        self.compilation
            .consume_budget(dimension, amount)
            .map_err(|error| {
                let kind = match &error {
                    crate::compiler::context::compilation_context::ContextError::Cancelled => {
                        StageErrorKind::Cancelled
                    }

                    crate::compiler::context::compilation_context::ContextError::Budget(_) => {
                        StageErrorKind::BudgetExceeded
                    }

                    _ => {
                        StageErrorKind::ExecutionContext
                    }
                };

                StageError::new(
                    kind,
                    error.to_string(),
                )
            })
    }

    /// Accounts for work using the descriptor's declared default
    /// budget dimension.
    ///
    /// Returns an explicit error if no default dimension was declared.
    ///
    /// Use consume_budget when accounting across multiple dimensions.
    pub fn consume_declared_budget(
        &self,
        amount: u128,
    ) -> Result<u128, StageError> {
        let dimension = self
            .descriptor
            .budget_dimension()
            .cloned()
            .ok_or_else(|| {
                StageError::new(
                    StageErrorKind::MissingBudgetDimension,
                    "stage has no declared default budget dimension",
                )
            })?;

        self.consume_budget(
            dimension,
            amount,
        )
    }
}

// ============================================================================
// STAGE ERROR CLASSIFICATION
// ============================================================================

/// Classification of errors returned by a compiler stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageErrorKind {
    /// Stage execution failed.
    Execution,

    /// Compilation was cancelled.
    Cancelled,

    /// Configured operational budget was exhausted.
    BudgetExceeded,

    /// Input did not satisfy the stage contract.
    InvalidInput,

    /// Required operation is unsupported.
    UnsupportedOperation,

    /// Stage cannot execute under the supplied configuration.
    IncompatibleConfiguration,

    /// Stage returned an invalid execution decision.
    InvalidDecision,

    /// Stage attempted an operation that requires an undeclared
    /// default budget dimension.
    MissingBudgetDimension,

    /// An execution-context operation failed.
    ExecutionContext,

    /// An internal stage invariant was violated.
    InternalInvariant,
}

// ============================================================================
// STRUCTURED STAGE ERROR
// ============================================================================

/// Structured stage failure.
///
/// Recovery policy is external to this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageError {
    kind: StageErrorKind,

    message: Arc<str>,

    retryable: bool,
}

impl StageError {
    /// Creates a non-retryable stage error.
    pub fn new(
        kind: StageErrorKind,
        message: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
        }
    }

    /// Creates a stage error that declares potential retryability.
    ///
    /// This is not permission to retry.
    ///
    /// pipeline::recovery must independently determine whether replay
    /// is safe and permitted.
    pub fn retryable(
        kind: StageErrorKind,
        message: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: true,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> StageErrorKind {
        self.kind
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    /// Returns the declared retryability.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        self.retryable
    }
}

impl fmt::Display for StageError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "compiler stage failed ({:?}): {}",
            self.kind,
            self.message,
        )
    }
}

impl Error for StageError {}

// ============================================================================
// STAGE OUTCOME
// ============================================================================

/// Result of a stage invocation.
///
/// Skipping and successful completion are separate outcomes.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StageOutcome {
    /// Stage completed successfully.
    Completed(StageOutput),

    /// Stage deliberately skipped execution.
    Skipped {
        reason: Arc<str>,
    },
}

// ============================================================================
// STAGE EXECUTION REQUEST
// ============================================================================

/// Complete immutable input to one stage invocation.
pub struct StageExecution<'a> {
    context: StageExecutionContext<'a>,

    input: StageInput,
}

impl<'a> StageExecution<'a> {
    /// Creates a stage execution request.
    #[must_use]
    pub fn new(
        compilation: &'a CompilationContext,
        descriptor: &'a StageDescriptor,
        input: StageInput,
    ) -> Self {
        Self {
            context: StageExecutionContext::new(
                compilation,
                descriptor,
            ),
            input,
        }
    }

    /// Returns the stage execution context.
    #[must_use]
    pub fn context(&self) -> &StageExecutionContext<'a> {
        &self.context
    }

    /// Returns the stage input.
    #[must_use]
    pub fn input(&self) -> &StageInput {
        &self.input
    }
}

// ============================================================================
// CANONICAL STAGE TRAIT
// ============================================================================

/// Contract implemented by independently executable compiler stages.
///
/// Implementations must:
///
/// 1. Return stable, validated descriptors.
/// 2. Preserve descriptor identity throughout registration.
/// 3. Declare truthful determinism guarantees.
/// 4. Declare truthful concurrency guarantees.
/// 5. Check cancellation before expensive operations.
/// 6. Account for measurable work.
/// 7. Validate incoming payloads.
/// 8. Preserve source-language semantics.
/// 9. Preserve canonical IR ownership.
/// 10. Report unsupported operations explicitly.
/// 11. Never silently truncate required work.
/// 12. Never report failed work as successful.
/// 13. Avoid introducing arbitrary hardware ceilings.
///
/// Implementations must not:
///
/// - Redefine semantic authority.
/// - Create competing IR representations.
/// - Perform physical allocation without the appropriate authority.
/// - Implement runtime scheduling.
/// - Silently bypass verification.
/// - Automatically retry failed operations.
///
/// This trait is object-safe.
pub trait Stage: Send + Sync {
    /// Returns immutable stage metadata.
    fn descriptor(&self) -> &StageDescriptor;

    /// Determines whether the stage applies to this compilation input.
    ///
    /// The default behavior is execution.
    ///
    /// Conditional domain-specific stages may override this method.
    ///
    /// Implementations must not interpret unvalidated source syntax
    /// as authoritative semantic information.
    fn should_execute(
        &self,
        _context: &StageExecutionContext<'_>,
        _input: &StageInput,
    ) -> Result<StageDecision, StageError> {
        Ok(StageDecision::Run)
    }

    /// Executes the actual stage computation.
    ///
    /// Dependency validation and output delivery belong to the executor.
    ///
    /// This method performs only the work owned by this stage.
    fn execute(
        &self,
        execution: &StageExecution<'_>,
    ) -> Result<StageOutput, StageError>;

    /// Performs a complete cooperative stage invocation.
    ///
    /// This default implementation guarantees:
    ///
    /// - Cancellation is checked before applicability evaluation.
    /// - Cancellation is checked after applicability evaluation.
    /// - Skipped stages have explicit reasons.
    /// - Cancellation is checked before expensive execution.
    /// - Execution errors propagate unchanged.
    /// - Cancellation is checked after execution.
    /// - Successful outputs are returned only when cancellation
    ///   has not been observed.
    ///
    /// It never retries automatically.
    fn run(
        &self,
        compilation: &CompilationContext,
        input: StageInput,
    ) -> Result<StageOutcome, StageError> {
        let descriptor = self.descriptor();

        // Validate immutable metadata at the invocation boundary.
        descriptor.validate().map_err(|error| {
            StageError::new(
                StageErrorKind::InternalInvariant,
                error.to_string(),
            )
        })?;

        // Check cancellation before evaluating applicability.
        compilation
            .check_cancelled()
            .map_err(|error| {
                StageError::new(
                    StageErrorKind::Cancelled,
                    error.to_string(),
                )
            })?;

        let execution = StageExecution::new(
            compilation,
            descriptor,
            input,
        );

        let decision = self.should_execute(
            execution.context(),
            execution.input(),
        )?;

        // Cancellation may occur during applicability evaluation.
        execution.context().check_cancelled()?;

        match decision {
            StageDecision::Skip { reason } => {
                if reason.trim().is_empty()
                    || reason.chars().any(char::is_control)
                {
                    return Err(
                        StageError::new(
                            StageErrorKind::InvalidDecision,
                            "stage returned an invalid skip reason",
                        ),
                    );
                }

                Ok(StageOutcome::Skipped {
                    reason,
                })
            }

            StageDecision::Run => {
                // A stage must not begin expensive work after cancellation.
                execution.context().check_cancelled()?;

                let output = self.execute(&execution)?;

                // Cancellation may race with stage execution.
                // Never publish success after observing cancellation.
                execution.context().check_cancelled()?;

                Ok(StageOutcome::Completed(output))
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::budget::BudgetDimension;

    use crate::compiler::context::compilation_context::CompilationContext;

    use crate::compiler_types::CompilerConfig;

    struct TestStage {
        descriptor: StageDescriptor,
        should_skip: bool,
    }

    impl TestStage {
        fn new(
            id: &str,
            should_skip: bool,
        ) -> Self {
            Self {
                descriptor: StageDescriptor::new(
                    StageId::new(id).expect("valid stage ID"),
                    "Test stage",
                    "1",
                    "Stage used by contract tests",
                )
                .expect("valid descriptor"),
                should_skip,
            }
        }
    }

    impl Stage for TestStage {
        fn descriptor(&self) -> &StageDescriptor {
            &self.descriptor
        }

        fn should_execute(
            &self,
            _context: &StageExecutionContext<'_>,
            _input: &StageInput,
        ) -> Result<StageDecision, StageError> {
            if self.should_skip {
                StageDecision::skip(
                    "not required by this test program",
                )
            } else {
                Ok(StageDecision::Run)
            }
        }

        fn execute(
            &self,
            execution: &StageExecution<'_>,
        ) -> Result<StageOutput, StageError> {
            execution.context().check_cancelled()?;

            Ok(StageOutput::new(
                String::from("completed"),
            ))
        }
    }

    fn context() -> CompilationContext {
        CompilationContext::new(
            CompilerConfig::default(),
        )
        .expect("valid test configuration")
    }

    fn id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage identifier")
    }

    // ------------------------------------------------------------------------
    // IDENTITIES
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_identifiers() {
        assert!(matches!(
            StageId::new(""),
            Err(StageIdError::Empty)
        ));
    }

    #[test]
    fn rejects_control_characters() {
        assert!(matches!(
            StageId::new("stage\ninvalid"),
            Err(StageIdError::InvalidControlCharacter)
        ));
    }

    #[test]
    fn supports_future_extension_identifiers() {
        let identifier = id(
            "zamani.extension.future_computing.specialization",
        );

        assert_eq!(
            identifier.as_str(),
            "zamani.extension.future_computing.specialization",
        );
    }

    #[test]
    fn stage_identifiers_have_deterministic_ordering() {
        let first = id("a.stage");
        let second = id("b.stage");

        assert!(first < second);
    }

    // ------------------------------------------------------------------------
    // DESCRIPTORS
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_stage_names() {
        assert!(matches!(
            StageDescriptor::new(
                id("test.stage"),
                "",
                "1",
                "",
            ),
            Err(StageDescriptorError::EmptyText {
                field: "name"
            })
        ));
    }

    #[test]
    fn rejects_empty_versions() {
        assert!(matches!(
            StageDescriptor::new(
                id("test.stage"),
                "Test",
                "",
                "",
            ),
            Err(StageDescriptorError::EmptyText {
                field: "version"
            })
        ));
    }

    #[test]
    fn rejects_self_dependencies() {
        let identifier = id("test.stage");

        let descriptor = StageDescriptor::new(
            identifier.clone(),
            "Test",
            "1",
            "",
        )
        .expect("valid descriptor");

        assert!(matches!(
            descriptor.with_dependency(identifier),
            Err(StageDescriptorError::SelfDependency { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_dependencies() {
        let dependency = id("frontend.parse");

        let descriptor = StageDescriptor::new(
            id("semantic.analysis"),
            "Semantic analysis",
            "1",
            "",
        )
        .expect("valid descriptor")
        .with_dependency(dependency.clone())
        .expect("first dependency");

        assert!(matches!(
            descriptor.with_dependency(dependency),
            Err(StageDescriptorError::DuplicateDependency { .. })
        ));
    }

    #[test]
    fn dependency_metadata_is_deterministic() {
        let descriptor = StageDescriptor::new(
            id("semantic.analysis"),
            "Semantic analysis",
            "1",
            "",
        )
        .expect("valid descriptor")
        .with_dependencies([
            id("z.stage"),
            id("a.stage"),
            id("m.stage"),
        ])
        .expect("valid dependencies");

        assert_eq!(
            descriptor.dependencies(),
            &[id("a.stage"), id("m.stage"), id("z.stage")]
        );
    }

    #[test]
    fn preserves_stage_metadata() {
        let descriptor = StageDescriptor::new(
            id("ir.verification"),
            "IR verification",
            "1",
            "Verifies canonical representations",
        )
        .expect("valid descriptor")
        .with_phase(CompilationPhase::IrVerification)
        .with_determinism(StageDeterminism::Deterministic)
        .with_concurrency(StageConcurrency::Concurrent);

        assert_eq!(
            descriptor.phase(),
            Some(CompilationPhase::IrVerification),
        );

        assert_eq!(
            descriptor.determinism(),
            StageDeterminism::Deterministic,
        );

        assert_eq!(
            descriptor.concurrency(),
            StageConcurrency::Concurrent,
        );

        assert!(descriptor.validate().is_ok());
    }

    // ------------------------------------------------------------------------
    // INPUT AND OUTPUT
    // ------------------------------------------------------------------------

    #[test]
    fn input_supports_typed_payloads() {
        let input = StageInput::new(
            String::from("portable IR"),
        );

        assert_eq!(
            input.downcast_ref::<String>().map(String::as_str),
            Some("portable IR"),
        );

        assert!(input.downcast_ref::<u64>().is_none());
    }

    #[test]
    fn output_supports_typed_payloads() {
        let output = StageOutput::new(42_u64);

        assert_eq!(
            output.downcast_ref::<u64>(),
            Some(&42_u64),
        );
    }

    #[test]
    fn empty_input_and_output_are_supported() {
        assert!(StageInput::empty().is_empty());
        assert!(StageOutput::empty().is_empty());
    }

    #[test]
    fn shared_payloads_preserve_identity() {
        let input = StageInput::new(String::from("source"));

        let shared = input.shared_payload().expect("payload");

        assert_eq!(
            shared.downcast_ref::<String>().map(String::as_str),
            Some("source"),
        );
    }

    // ------------------------------------------------------------------------
    // BUDGET ACCOUNTING
    // ------------------------------------------------------------------------

    #[test]
    fn stage_can_consume_shared_budget() {
        let compilation = context();

        let dimension = BudgetDimension::new(
            "compiler_stage_work",
            "units",
        )
        .expect("valid dimension");

        let descriptor = StageDescriptor::new(
            id("test.budget"),
            "Budget test",
            "1",
            "",
        )
        .expect("valid descriptor")
        .with_budget_dimension(dimension.clone());

        let execution = StageExecutionContext::new(
            &compilation,
            &descriptor,
        );

        assert_eq!(
            execution.consume_declared_budget(1).expect("budget"),
            1,
        );
    }

    #[test]
    fn missing_default_budget_dimension_is_explicit() {
        let compilation = context();

        let descriptor = StageDescriptor::new(
            id("test.budget"),
            "Budget test",
            "1",
            "",
        )
        .expect("valid descriptor");

        let execution = StageExecutionContext::new(
            &compilation,
            &descriptor,
        );

        assert_eq!(
            execution
                .consume_declared_budget(1)
                .expect_err("dimension required")
                .kind(),
            StageErrorKind::MissingBudgetDimension,
        );
    }

    // ------------------------------------------------------------------------
    // EXECUTION
    // ------------------------------------------------------------------------

    #[test]
    fn stage_executes_successfully() {
        let compilation = context();

        let stage = TestStage::new(
            "test.execution",
            false,
        );

        match stage
            .run(&compilation, StageInput::empty())
            .expect("stage execution")
        {
            StageOutcome::Completed(output) => {
                assert_eq!(
                    output.downcast_ref::<String>().map(String::as_str),
                    Some("completed"),
                );
            }

            StageOutcome::Skipped { .. } => {
                panic!("stage should not have been skipped");
            }
        }
    }

    #[test]
    fn stage_can_be_conditionally_skipped() {
        let compilation = context();

        let stage = TestStage::new(
            "test.optional",
            true,
        );

        assert!(matches!(
            stage.run(&compilation, StageInput::empty()),
            Ok(StageOutcome::Skipped { .. }),
        ));
    }

    #[test]
    fn invalid_skip_decision_is_rejected() {
        assert!(StageDecision::skip("").is_err());

        assert!(StageDecision::skip(
            "not applicable",
        ).is_ok());
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_prevents_execution() {
        let compilation = context();

        compilation.cancellation_token().cancel();

        let stage = TestStage::new(
            "test.cancelled",
            false,
        );

        assert_eq!(
            stage
                .run(&compilation, StageInput::empty())
                .expect_err("must be cancelled")
                .kind(),
            StageErrorKind::Cancelled,
        );
    }

    // ------------------------------------------------------------------------
    // ERROR CONTRACT
    // ------------------------------------------------------------------------

    #[test]
    fn retryability_remains_advisory() {
        let error = StageError::retryable(
            StageErrorKind::Execution,
            "temporary provider failure",
        );

        assert!(error.is_retryable());

        assert_eq!(
            error.kind(),
            StageErrorKind::Execution,
        );
    }

    #[test]
    fn error_display_preserves_classification() {
        let error = StageError::new(
            StageErrorKind::InvalidInput,
            "unexpected representation",
        );

        assert_eq!(
            error.to_string(),
            "compiler stage failed (InvalidInput): unexpected representation",
        );
    }
}
