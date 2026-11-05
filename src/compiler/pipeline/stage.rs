
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Stage Contract.
//!
//! Path:
//!     src/compiler/pipeline/stage.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Defines the foundational contract implemented by individual compiler
//! pipeline stages.
//!
//! A stage is one independently executable unit of compiler work.
//!
//! Examples include:
//!
//! - Frontend integration.
//! - Structural validation.
//! - Semantic analysis integration.
//! - Ownership analysis.
//! - Dependency analysis.
//! - Canonical IR construction.
//! - IR verification.
//! - Optimization.
//! - Capability analysis.
//! - Feasibility analysis.
//! - Heterogeneous planning.
//! - IR lowering.
//! - Legalization.
//! - Linking.
//! - Backend invocation.
//! - Artifact validation.
//!
//! This module defines contracts only.
//!
//! It does not implement the compiler pipeline itself.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - StageId.
//! - StageDescriptor.
//! - StageExecution.
//! - StageExecutionContext.
//! - StageInput.
//! - StageOutput.
//! - StageDecision.
//! - StageOutcome.
//! - StageError.
//! - StageErrorKind.
//! - StageDeterminism.
//! - StageConcurrency.
//! - The Stage trait.
//!
//! It does NOT own:
//!
//! - Pipeline ordering.
//! - Dependency graph construction.
//! - Pipeline orchestration.
//! - Stage registration.
//! - Pipeline recovery.
//! - Compilation lifecycle tracking.
//! - Compiler diagnostics.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Backend implementation.
//! - Runtime execution.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! Existing foundational dependencies:
//!
//!     compiler::context::compilation_context
//!         CompilationContext
//!
//!     compiler::context::compilation_state
//!         CompilationPhase
//!
//!     compiler::configuration::budget
//!         BudgetDimension
//!
//! Future consumers:
//!
//!     pipeline::stages
//!     pipeline::ordering
//!     pipeline::failure
//!     pipeline::execution
//!     pipeline::orchestration
//!
//! This module does not depend on those future consumers.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module imposes no universal limits on:
//!
//! - CPUs.
//! - Cores.
//! - Workers.
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
//! - IR operations.
//! - Modules.
//! - Functions.
//! - Pipeline stages.
//! - Stage dependencies.
//! - Compilation units.
//!
//! Stage identifiers and metadata use dynamically sized strings.
//!
//! Operational resource constraints are supplied by CompilationContext.
//!
//! Unlimited configured budgets do not promise infinite physical resources.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! The authoritative grammar roots are:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/core/program.g4
//!     grammar/compile/compile.g4
//!
//! The grammar describes source-level compilation intent.
//!
//! This module describes compiler implementation stages.
//!
//! A source-level compilation declaration is not itself a Stage object.
//!
//! Semantic analysis must interpret source declarations before they can
//! influence legal compiler-stage execution.
//!
//! ============================================================================

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

/// Public API version for the pipeline-stage contract.
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

/// Identifies one compiler pipeline stage.
///
/// Stage identifiers are logical identifiers.
///
/// They are not hardware identifiers.
///
/// A stage identifier must be non-empty and must not contain control
/// characters.
///
/// No arbitrary identifier-length limit is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StageId(Arc<str>);

/// Errors associated with stage identity construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageIdError {
    Empty,
    InvalidControlCharacter,
}

impl fmt::Display for StageIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("stage identifier cannot be empty")
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
    /// Creates a validated stage identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, StageIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StageIdError::Empty);
        }

        if value.chars().any(char::is_control) {
            return Err(StageIdError::InvalidControlCharacter);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for StageId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// STAGE DETERMINISM
// ============================================================================

/// Describes the determinism guarantees of a stage.
///
/// Determinism is important for:
///
/// - Reproducible compilation.
/// - Incremental compilation.
/// - Distributed compilation.
/// - Build caching.
/// - Differential testing.
/// - Portable artifacts.
///
/// A stage must declare its actual behavior rather than claiming
/// determinism merely because it is executed sequentially.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageDeterminism {
    /// Identical declared inputs and configuration produce identical
    /// observable stage outputs.
    Deterministic,

    /// Output may depend on explicitly declared external measurements,
    /// randomized decisions or environmental inputs.
    ///
    /// Such inputs must be represented in provenance and cache identity.
    InputDependent,

    /// The stage cannot guarantee reproducible output.
    ///
    /// Its outputs must not be cached as deterministic artifacts.
    Nondeterministic,
}

// ============================================================================
// STAGE CONCURRENCY
// ============================================================================

/// Declares the concurrency guarantees of a stage.
///
/// This declaration does not start threads.
///
/// Parallel execution belongs to the pipeline executor and the existing
/// parallel-build subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageConcurrency {
    /// The same stage instance must not execute concurrently.
    Exclusive,

    /// Different instances may execute concurrently, but one instance
    /// must not be invoked concurrently.
    InstanceExclusive,

    /// The stage implementation supports concurrent invocations,
    /// including concurrent invocation of the same instance.
    Concurrent,
}

// ============================================================================
// STAGE DESCRIPTOR
// ============================================================================

/// Immutable descriptive metadata for a compiler stage.
///
/// The descriptor allows pipeline infrastructure to inspect a stage
/// without executing it.
///
/// It contains no implementation-specific hardware assumptions.
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
    /// The stage name and version are required.
    ///
    /// The description may be empty.
    ///
    /// Dependency existence and cycle detection belong to pipeline
    /// ordering and registration.
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
            return Err(StageDescriptorError::InvalidText {
                field: "description",
            });
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

    /// Associates a standard high-level compilation phase.
    #[must_use]
    pub fn with_phase(mut self, phase: CompilationPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Declares a prerequisite stage.
    ///
    /// The dependency must complete successfully before this stage
    /// is eligible for execution.
    ///
    /// This method does not verify that the dependency is registered.
    ///
    /// Global dependency validation belongs to pipeline::ordering.
    pub fn with_dependency(
        mut self,
        dependency: StageId,
    ) -> Result<Self, StageDescriptorError> {
        if dependency == self.id {
            return Err(StageDescriptorError::SelfDependency {
                stage: self.id.clone(),
            });
        }

        if self.dependencies.contains(&dependency) {
            return Err(StageDescriptorError::DuplicateDependency {
                stage: self.id.clone(),
                dependency,
            });
        }

        self.dependencies.push(dependency);

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

    /// Declares stage determinism.
    #[must_use]
    pub fn with_determinism(
        mut self,
        determinism: StageDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Declares stage concurrency guarantees.
    #[must_use]
    pub fn with_concurrency(
        mut self,
        concurrency: StageConcurrency,
    ) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Associates an operational budget dimension with the stage.
    ///
    /// The stage must explicitly report consumption through its execution
    /// context.
    ///
    /// Merely declaring a dimension does not automatically account for work.
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

    /// Returns declared dependencies.
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

    /// Returns the optional operational budget dimension.
    #[must_use]
    pub fn budget_dimension(&self) -> Option<&BudgetDimension> {
        self.budget_dimension.as_ref()
    }
}

fn validate_descriptor_text(
    value: &str,
    field: &'static str,
) -> Result<(), StageDescriptorError> {
    if value.trim().is_empty() {
        return Err(StageDescriptorError::EmptyText { field });
    }

    if value.chars().any(char::is_control) {
        return Err(StageDescriptorError::InvalidText { field });
    }

    Ok(())
}

/// Stage descriptor construction errors.
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(formatter, "stage descriptor {field} cannot be empty")
            }

            Self::InvalidText { field } => {
                write!(
                    formatter,
                    "stage descriptor {field} contains control characters"
                )
            }

            Self::SelfDependency { stage } => {
                write!(formatter, "stage {stage} cannot depend on itself")
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
// TYPE-ERASED STAGE INPUT
// ============================================================================

/// Type-erased, reference-counted stage input.
///
/// The pipeline can transfer values between independently implemented
/// stages without owning or duplicating their domain-specific types.
///
/// Payload types must be Send + Sync + 'static so that pipeline
/// infrastructure can safely share them across worker threads.
#[derive(Clone, Default)]
pub struct StageInput {
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl StageInput {
    /// Creates an empty input.
    #[must_use]
    pub const fn empty() -> Self {
        Self { payload: None }
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

    /// Returns whether the input contains a payload.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.is_none()
    }

    /// Returns a typed reference when the payload has the requested type.
    #[must_use]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.payload.as_ref()?.downcast_ref::<T>()
    }

    /// Returns a cloned reference-counted payload.
    #[must_use]
    pub fn shared_payload(
        &self,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        self.payload.as_ref().map(Arc::clone)
    }
}

impl fmt::Debug for StageInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StageInput")
            .field("has_payload", &self.payload.is_some())
            .finish()
    }
}

// ============================================================================
// STAGE OUTPUT
// ============================================================================

/// Type-erased stage output.
///
/// Outputs preserve domain ownership.
///
/// For example:
///
/// - A semantic-analysis stage may return a semantic result.
/// - An IR stage may return a canonical IR module.
/// - An optimization stage may return a verified IR result.
/// - A backend stage may return an artifact reference.
///
/// This module does not define those types.
#[derive(Clone, Default)]
pub struct StageOutput {
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl StageOutput {
    /// Creates an empty successful output.
    #[must_use]
    pub const fn empty() -> Self {
        Self { payload: None }
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

    /// Returns whether the output contains a payload.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.is_none()
    }

    /// Returns a typed reference to the output.
    #[must_use]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.payload.as_ref()?.downcast_ref::<T>()
    }

    /// Returns a cloned reference-counted payload.
    #[must_use]
    pub fn shared_payload(
        &self,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        self.payload.as_ref().map(Arc::clone)
    }
}

impl fmt::Debug for StageOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StageOutput")
            .field("has_payload", &self.payload.is_some())
            .finish()
    }
}

// ============================================================================
// STAGE DECISION
// ============================================================================

/// Decision made before executing a stage.
///
/// Conditional execution is necessary because not every program requires
/// every compiler domain.
///
/// For example, a purely classical program should not be forced through
/// quantum-specific lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageDecision {
    /// Execute the stage.
    Run,

    /// Skip the stage with an explicit explanation.
    ///
    /// Skipping is not the same as successful execution.
    ///
    /// The pipeline executor must record the skipped state and apply
    /// the pipeline's dependency policy.
    Skip {
        reason: Arc<str>,
    },
}

impl StageDecision {
    /// Creates a skip decision.
    pub fn skip(
        reason: impl Into<Arc<str>>,
    ) -> Result<Self, StageError> {
        let reason = reason.into();

        if reason.trim().is_empty() {
            return Err(StageError::new(
                StageErrorKind::InvalidDecision,
                "a skipped stage must provide a non-empty reason",
            ));
        }

        Ok(Self::Skip { reason })
    }

    /// Returns whether the stage should execute.
    #[must_use]
    pub const fn should_run(&self) -> bool {
        matches!(self, Self::Run)
    }
}

// ============================================================================
// STAGE EXECUTION CONTEXT
// ============================================================================

/// Context supplied to one stage invocation.
///
/// The compilation context is borrowed rather than reconstructed.
///
/// Consequently, all stages share the same:
///
/// - Configuration.
/// - Operational budget accounting.
/// - Cancellation state.
///
/// A stage cannot silently obtain a fresh unlimited budget by cloning
/// or reconstructing an independent context.
pub struct StageExecutionContext<'a> {
    compilation: &'a CompilationContext,

    descriptor: &'a StageDescriptor,
}

impl<'a> StageExecutionContext<'a> {
    /// Creates an execution context.
    ///
    /// Pipeline execution infrastructure should use this constructor.
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

    /// Returns the current stage identifier.
    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        self.descriptor.id()
    }

    /// Returns the current stage descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &'a StageDescriptor {
        self.descriptor
    }

    /// Checks whether compilation has been cancelled.
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

    /// Accounts for stage work against the shared compilation budget.
    ///
    /// The supplied dimension is explicit because stages may account for
    /// different kinds of measurable work.
    ///
    /// The returned value is the cumulative consumption reported by the
    /// authoritative budget tracker.
    pub fn consume_budget(
        &self,
        dimension: BudgetDimension,
        amount: u128,
    ) -> Result<u128, StageError> {
        self.check_cancelled()?;

        self.compilation
            .consume_budget(dimension, amount)
            .map_err(|error| {
                let kind = match error {
                    crate::compiler::context::compilation_context::ContextError::Cancelled => {
                        StageErrorKind::Cancelled
                    }

                    crate::compiler::context::compilation_context::ContextError::Budget(_) => {
                        StageErrorKind::BudgetExceeded
                    }

                    _ => StageErrorKind::ExecutionContext,
                };

                StageError::new(kind, error.to_string())
            })
    }
}

// ============================================================================
// STAGE ERROR
// ============================================================================

/// Classification of stage execution errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageErrorKind {
    /// Stage execution failed.
    Execution,

    /// Compilation was cancelled.
    Cancelled,

    /// An operational compiler budget was exhausted.
    BudgetExceeded,

    /// Stage input did not match the expected contract.
    InvalidInput,

    /// The stage encountered an unsupported operation.
    UnsupportedOperation,

    /// The stage cannot execute under the supplied configuration.
    IncompatibleConfiguration,

    /// The stage reported an invalid execution decision.
    InvalidDecision,

    /// An execution-context operation failed.
    ExecutionContext,

    /// An internal stage invariant was violated.
    InternalInvariant,
}

/// Structured error returned by a pipeline stage.
///
/// Stage failures must not be silently converted into successful output.
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

    /// Creates a retryable stage error.
    ///
    /// Retryability is a declaration, not an instruction to retry.
    ///
    /// Recovery policy belongs to pipeline::recovery.
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

    /// Returns whether the stage declares the error retryable.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        self.retryable
    }
}

impl fmt::Display for StageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "compiler stage failed ({:?}): {}",
            self.kind,
            self.message
        )
    }
}

impl Error for StageError {}

// ============================================================================
// STAGE OUTCOME
// ============================================================================

/// Result of a stage invocation.
///
/// A skipped stage is represented separately from a completed stage.
///
/// This prevents the pipeline from confusing:
///
/// - successful execution;
/// - conditional skipping;
/// - execution failure.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StageOutcome {
    /// Stage completed successfully.
    Completed(StageOutput),

    /// Stage was deliberately skipped.
    Skipped {
        reason: Arc<str>,
    },
}

// ============================================================================
// STAGE EXECUTION REQUEST
// ============================================================================

/// Complete request for one stage invocation.
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

    /// Returns the execution context.
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
// STAGE TRAIT
// ============================================================================

/// Contract implemented by an independently executable compiler stage.
///
/// Implementations must:
///
/// 1. Return stable, validated metadata.
/// 2. Declare actual determinism guarantees.
/// 3. Declare actual concurrency guarantees.
/// 4. Check cancellation before expensive work.
/// 5. Account for applicable compiler work.
/// 6. Validate input before processing it.
/// 7. Preserve the semantics of their input.
/// 8. Return explicit errors for unsupported operations.
/// 9. Never silently truncate compilation work.
/// 10. Return output owned by the appropriate domain subsystem.
///
/// A stage must not:
///
/// - Select physical devices without the appropriate planner/runtime.
/// - Allocate hardware resources.
/// - redefine canonical IR.
/// - Modify source semantics.
/// - silently skip required verification.
/// - claim successful execution after failure.
///
/// The trait is object-safe and can therefore be used with registered
/// heterogeneous stage implementations.
pub trait Stage: Send + Sync {
    /// Returns immutable stage metadata.
    fn descriptor(&self) -> &StageDescriptor;

    /// Determines whether this stage should execute for the supplied input.
    ///
    /// The default is to execute.
    ///
    /// Implementations may override this for domain-specific conditional
    /// execution.
    ///
    /// A skip decision must include a meaningful explanation.
    fn should_execute(
        &self,
        _context: &StageExecutionContext<'_>,
        _input: &StageInput,
    ) -> Result<StageDecision, StageError> {
        Ok(StageDecision::Run)
    }

    /// Executes the stage.
    ///
    /// The pipeline executor is responsible for:
    ///
    /// - dependency validation;
    /// - execution ordering;
    /// - concurrency admission;
    /// - lifecycle tracking;
    /// - diagnostics;
    /// - failure propagation;
    /// - recovery policy.
    ///
    /// This method is responsible for the stage's actual computation.
    fn execute(
        &self,
        execution: &StageExecution<'_>,
    ) -> Result<StageOutput, StageError>;

    /// Performs one complete stage invocation.
    ///
    /// This method centralizes:
    ///
    /// - cancellation checking;
    /// - conditional execution;
    /// - execution;
    /// - cancellation checking after execution.
    ///
    /// It does not swallow failures or automatically retry stages.
    fn run(
        &self,
        compilation: &CompilationContext,
        input: StageInput,
    ) -> Result<StageOutcome, StageError> {
        let descriptor = self.descriptor();

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

        match self.should_execute(
            execution.context(),
            execution.input(),
        )? {
            StageDecision::Skip { reason } => {
                compilation
                    .check_cancelled()
                    .map_err(|error| {
                        StageError::new(
                            StageErrorKind::Cancelled,
                            error.to_string(),
                        )
                    })?;

                Ok(StageOutcome::Skipped { reason })
            }

            StageDecision::Run => {
                let output = self.execute(&execution)?;

                compilation
                    .check_cancelled()
                    .map_err(|error| {
                        StageError::new(
                            StageErrorKind::Cancelled,
                            error.to_string(),
                        )
                    })?;

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
                StageDecision::skip("not required by this test program")
            } else {
                Ok(StageDecision::Run)
            }
        }

        fn execute(
            &self,
            execution: &StageExecution<'_>,
        ) -> Result<StageOutput, StageError> {
            execution.context().check_cancelled()?;

            Ok(StageOutput::new(String::from("completed")))
        }
    }

    fn context() -> CompilationContext {
        CompilationContext::new(CompilerConfig::default())
            .expect("valid test compiler configuration")
    }

    #[test]
    fn stage_id_rejects_empty_identifiers() {
        assert!(matches!(
            StageId::new(""),
            Err(StageIdError::Empty)
        ));
    }

    #[test]
    fn stage_id_rejects_control_characters() {
        assert!(matches!(
            StageId::new("stage\ninvalid"),
            Err(StageIdError::InvalidControlCharacter)
        ));
    }

    #[test]
    fn stage_id_accepts_dynamic_identifiers() {
        let id = StageId::new(
            "zamani.extension.future_computing.specialization",
        )
        .expect("valid identifier");

        assert_eq!(
            id.as_str(),
            "zamani.extension.future_computing.specialization"
        );
    }

    #[test]
    fn descriptor_rejects_empty_name() {
        let id = StageId::new("test.stage").unwrap();

        assert!(matches!(
            StageDescriptor::new(id, "", "1", ""),
            Err(StageDescriptorError::EmptyText {
                field: "name"
            })
        ));
    }

    #[test]
    fn descriptor_rejects_empty_version() {
        let id = StageId::new("test.stage").unwrap();

        assert!(matches!(
            StageDescriptor::new(id, "Test", "", ""),
            Err(StageDescriptorError::EmptyText {
                field: "version"
            })
        ));
    }

    #[test]
    fn descriptor_rejects_self_dependency() {
        let id = StageId::new("test.stage").unwrap();

        let descriptor = StageDescriptor::new(
            id.clone(),
            "Test",
            "1",
            "",
        )
        .unwrap();

        assert!(matches!(
            descriptor.with_dependency(id),
            Err(StageDescriptorError::SelfDependency { .. })
        ));
    }

    #[test]
    fn descriptor_rejects_duplicate_dependencies() {
        let dependency = StageId::new("frontend.parse").unwrap();

        let descriptor = StageDescriptor::new(
            StageId::new("semantic.analysis").unwrap(),
            "Semantic analysis",
            "1",
            "",
        )
        .unwrap()
        .with_dependency(dependency.clone())
        .unwrap();

        assert!(matches!(
            descriptor.with_dependency(dependency),
            Err(StageDescriptorError::DuplicateDependency { .. })
        ));
    }

    #[test]
    fn descriptor_preserves_declared_metadata() {
        let descriptor = StageDescriptor::new(
            StageId::new("ir.verification").unwrap(),
            "IR verification",
            "1",
            "Verifies canonical intermediate representations",
        )
        .unwrap()
        .with_phase(CompilationPhase::IrVerification)
        .with_determinism(StageDeterminism::Deterministic)
        .with_concurrency(StageConcurrency::Concurrent);

        assert_eq!(
            descriptor.phase(),
            Some(CompilationPhase::IrVerification)
        );

        assert_eq!(
            descriptor.determinism(),
            StageDeterminism::Deterministic
        );

        assert_eq!(
            descriptor.concurrency(),
            StageConcurrency::Concurrent
        );
    }

    #[test]
    fn stage_input_supports_typed_payloads() {
        let input = StageInput::new(String::from("portable IR"));

        assert_eq!(
            input.downcast_ref::<String>().map(String::as_str),
            Some("portable IR")
        );

        assert!(input.downcast_ref::<u64>().is_none());
    }

    #[test]
    fn stage_output_supports_typed_payloads() {
        let output = StageOutput::new(42_u64);

        assert_eq!(
            output.downcast_ref::<u64>(),
            Some(&42_u64)
        );
    }

    #[test]
    fn empty_stage_input_is_supported() {
        let input = StageInput::empty();

        assert!(input.is_empty());
        assert!(input.downcast_ref::<String>().is_none());
    }

    #[test]
    fn empty_stage_output_is_supported() {
        let output = StageOutput::empty();

        assert!(output.is_empty());
    }

    #[test]
    fn stage_executes_successfully() {
        let compilation = context();
        let stage = TestStage::new("test.execution", false);

        let result = stage.run(
            &compilation,
            StageInput::empty(),
        );

        match result.expect("stage execution should succeed") {
            StageOutcome::Completed(output) => {
                assert_eq!(
                    output.downcast_ref::<String>().map(String::as_str),
                    Some("completed")
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
        let stage = TestStage::new("test.optional", true);

        let result = stage
            .run(&compilation, StageInput::empty())
            .expect("skip should be successful");

        assert!(matches!(
            result,
            StageOutcome::Skipped { .. }
        ));
    }

    #[test]
    fn cancellation_is_checked_before_execution() {
        let compilation = context();

        compilation.cancellation_token().cancel();

        let stage = TestStage::new("test.cancelled", false);

        let result = stage.run(
            &compilation,
            StageInput::empty(),
        );

        assert!(matches!(
            result,
            Err(StageError {
                kind: StageErrorKind::Cancelled,
                ..
            })
        ));
    }

    #[test]
    fn stage_can_account_for_shared_budget() {
        let compilation = context();

        let dimension = BudgetDimension::new(
            "compiler_stage_work",
            "units",
        )
        .expect("valid budget dimension");

        let stage = StageDescriptor::new(
            StageId::new("test.budget").unwrap(),
            "Budget test",
            "1",
            "",
        )
        .unwrap()
        .with_budget_dimension(dimension.clone());

        let execution = StageExecutionContext::new(
            &compilation,
            &stage,
        );

        assert_eq!(
            execution
                .consume_budget(dimension, 1)
                .expect("unlimited budget should permit work"),
            1
        );
    }

    #[test]
    fn stage_error_preserves_retryability() {
        let error = StageError::retryable(
            StageErrorKind::Execution,
            "temporary external service failure",
        );

        assert!(error.is_retryable());

        assert_eq!(
            error.kind(),
            StageErrorKind::Execution
        );
    }

    #[test]
    fn stage_error_display_is_structured() {
        let error = StageError::new(
            StageErrorKind::InvalidInput,
            "unexpected IR representation",
        );

        assert_eq!(
            error.to_string(),
            "compiler stage failed (InvalidInput): unexpected IR representation"
        );
    }

    #[test]
    fn skip_requires_an_explanation() {
        assert!(StageDecision::skip("").is_err());

        assert!(StageDecision::skip(
            "quantum lowering is unnecessary"
        ).is_ok());
    }
}
