
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Validation
//!
//! File: `src/frontend/semantic/validation/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! This module defines the stable orchestration boundary for semantic
//! validation of the canonical Zamani frontend AST.
//!
//! It provides:
//!
//! - deterministic registration and execution of validation passes;
//! - explicit pass phases and dependency ordering;
//! - structured pass failures;
//! - configurable operational budgets;
//! - diagnostic collection, ordering and deduplication;
//! - explicit reporting of incomplete validation;
//! - extension points for future and domain-specific validators;
//! - a stable integration contract for the semantic pipeline.
//!
//! It does not implement parsing, AST construction, name resolution, type
//! inference, domain semantics, resource allocation, IR generation, or
//! target-specific feasibility analysis.
//!
//! ## Architecture
//!
//! ```text
//! Canonical frontend AST
//!          |
//!          v
//! Existing AST structural validation
//!          |
//!          v
//! Semantic validation pipeline
//!          |
//!          +-- declaration validation
//!          +-- expression validation
//!          +-- statement validation
//!          +-- effect validation
//!          +-- domain-rule validation
//!          +-- semantic invariant validation
//!          |
//!          v
//! Structured semantic result and diagnostics
//!          |
//!          v
//! Compiler / tooling / semantic-model consumers
//! ```
//!
//! The existing `frontend::ast::node::validation` module remains the
//! authority for AST structural validation. This module must not create a
//! competing structural validator.
//!
//! ## Language authority
//!
//! Semantic rules are governed by the repository's normative language
//! specifications and grammar authority. In particular:
//!
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - the applicable normative type, effect, resource, capability, contract,
//!   domain, and compatibility specifications.
//!
//! The grammar defines syntax. Semantic specifications define meaning.
//! This module must not silently introduce syntax or semantic rules that
//! conflict with those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - validation pass contracts;
//! - validation phase ordering;
//! - pass registration and duplicate detection;
//! - orchestration-level operational budgets;
//! - diagnostic sink behavior;
//! - deterministic result finalization;
//! - explicit incomplete-analysis reporting.
//!
//! It does not own:
//!
//! - canonical AST nodes, identities, or source spans;
//! - canonical semantic types or semantic facts;
//! - the repository-wide diagnostic schema;
//! - symbol tables or name resolution;
//! - type inference or type compatibility;
//! - effects, capabilities, resources, or contracts;
//! - domain-specific semantic rules;
//! - physical hardware discovery or allocation;
//! - classical IR, `quantum::ir`, HDL IR, or backend representations.
//!
//! ## Dependency direction
//!
//! ```text
//! canonical AST and source infrastructure
//!                  |
//!                  v
//! shared semantic model and diagnostics
//!                  |
//!                  v
//! validation pass implementations
//!                  |
//!                  v
//! this orchestration contract
//!                  |
//!                  v
//! semantic pipeline and compiler integration
//! ```
//!
//! Pass implementations may use the canonical AST and shared semantic
//! services. They must not make this facade depend on backend-specific
//! implementations.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no language-level or hardware-capacity ceilings.
//! In particular, it defines no maximum number of:
//!
//! - qubits, registers, processors, accelerators, or devices;
//! - declarations, expressions, statements, or AST nodes;
//! - tensor dimensions, tensor rank, or distributed nodes;
//! - computational domains or registered operations.
//!
//! Operational budgets are optional, explicit service policies. An exhausted
//! budget produces an incomplete report, never an assertion that analysis
//! succeeded.
//!
//! No finite implementation can promise to process literally infinite input.
//! The architectural requirement is to avoid arbitrary language-level
//! ceilings and scale with available resources and representable values.
//!
//! ## Determinism
//!
//! For equivalent inputs, configuration, registered pass identities, and
//! equivalent pass behavior, this module guarantees:
//!
//! - deterministic pass ordering by phase and stable pass identifier;
//! - duplicate pass identifier rejection;
//! - deterministic diagnostic sorting and deduplication when diagnostic
//!   values implement a deterministic `Ord` contract;
//! - deterministic report status for the same execution outcomes.
//!
//! Individual passes are responsible for deterministic semantic behavior.
//! They must not depend on unordered iteration, wall-clock time, randomness,
//! hardware discovery, or hidden global mutable state when computing source
//! semantics.
//!
//! ## Error handling
//!
//! A pass may report ordinary semantic diagnostics through `DiagnosticSink`.
//! A pass may also return an orchestration-level failure. Any such failure
//! makes the report incomplete. Later passes are not run after a failure,
//! because their prerequisites may no longer hold.
//!
//! Budget exhaustion also makes the report incomplete. Diagnostic collection
//! may continue to count omitted diagnostics without storing them.
//!
//! ## Integration contract
//!
//! The semantic pipeline supplies a stable analysis context `C` containing
//! references to the canonical AST, configuration, symbol-resolution
//! services, semantic model, and other required shared services.
//!
//! Passes consume that context and report diagnostics of type `D`.
//! `D` must be the repository's canonical semantic diagnostic type when that
//! type is available. This module deliberately does not redefine it.
//!
//! A pass must not certify semantic validity merely because it found no
//! errors in the subset it understands. Completeness is determined by the
//! pipeline and the execution status of all required passes.
//!
//! ## Completion contract
//!
//! This file is complete when:
//!
//! - the pass-registration contract is stable;
//! - phase ordering and failure behavior are tested;
//! - budget exhaustion cannot be mistaken for successful validation;
//! - diagnostics have deterministic finalization;
//! - domain extensions do not require changes to this facade;
//! - integration uses the canonical semantic context and diagnostic type;
//! - no duplicate AST, semantic model, or diagnostic owner is introduced;
//! - all tests in this file pass under the repository's supported toolchain.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

// =============================================================================
// Stable module identity
// =============================================================================

/// Stable architectural name for the semantic validation subsystem.
pub const VALIDATION_MODULE_NAME: &str = "zamani.frontend.semantic.validation";

/// Version of this module's public orchestration contract.
///
/// This is independent of the Zamani language edition, AST schema, semantic
/// model schema, and compiler release version.
pub const VALIDATION_MODULE_SCHEMA_VERSION: u16 = 1;

/// Returns the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    VALIDATION_MODULE_NAME
}

/// Returns the validation contract schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    VALIDATION_MODULE_SCHEMA_VERSION
}

// =============================================================================
// Validation phases
// =============================================================================

/// Logical validation phase.
///
/// Phase order expresses prerequisites, not a fixed inventory of domains.
/// New passes may be registered in an existing phase without changing this
/// enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ValidationPhase {
    /// Check that the semantic input and analysis context are usable.
    ///
    /// This is not a replacement for canonical AST structural validation.
    Input,

    /// Validate declarations and declaration-level relationships.
    Declarations,

    /// Validate expressions after the prerequisites required by a pass.
    Expressions,

    /// Validate statements, control flow, and statement-level relationships.
    Statements,

    /// Validate effect rules and effect-related obligations.
    Effects,

    /// Validate capability and symbolic resource requirements.
    ///
    /// These checks concern source-level meaning, not physical allocation.
    Requirements,

    /// Validate domain-specific semantics.
    ///
    /// Classical, quantum, HDL, hybrid, AI/data, concurrency, distributed,
    /// and future domains may register passes in this phase.
    Domains,

    /// Check cross-cutting semantic invariants and finalize obligations.
    Finalization,
}

// =============================================================================
// Operational policy
// =============================================================================

/// Optional operational budgets for one validation run.
///
/// `None` means this orchestration layer imposes no corresponding budget.
/// The absence of a budget does not imply infinite physical memory, execution
/// time, address space, or processing capacity.
///
/// These values are service-safety policies, never hardware-capacity limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationConfig {
    /// Optional maximum number of passes to start in one run.
    pub max_passes: Option<usize>,

    /// Optional maximum number of diagnostics retained in the report.
    ///
    /// Additional diagnostics are counted but not retained. Reaching this
    /// budget makes the report incomplete because the diagnostic set may
    /// no longer be complete.
    pub max_diagnostics: Option<usize>,
}

impl ValidationConfig {
    /// Creates an unrestricted orchestration policy.
    ///
    /// The caller remains responsible for configuring service-level budgets
    /// where untrusted input or operational policy requires them.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_passes: None,
            max_diagnostics: None,
        }
    }

    /// Sets the optional pass execution budget.
    #[must_use]
    pub const fn with_max_passes(mut self, limit: Option<usize>) -> Self {
        self.max_passes = limit;
        self
    }

    /// Sets the optional retained-diagnostic budget.
    #[must_use]
    pub const fn with_max_diagnostics(mut self, limit: Option<usize>) -> Self {
        self.max_diagnostics = limit;
        self
    }
}

// =============================================================================
// Pass contract
// =============================================================================

/// A stable identifier for a validation pass.
///
/// Identifiers must be nonempty, namespaced where appropriate, and stable
/// across equivalent builds. They must not depend on memory addresses or
/// registration order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationPassId(String);

impl ValidationPassId {
    /// Creates a pass identifier.
    ///
    /// Empty or whitespace-only identifiers are rejected. Identifier syntax
    /// beyond this basic invariant belongs to the extension-registration
    /// contract and can be tightened without changing pass execution.
    pub fn new(value: impl Into<String>) -> Result<Self, RegistrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RegistrationError::EmptyPassId);
        }

        if value.trim() != value {
            return Err(RegistrationError::SurroundingWhitespace {
                identifier: value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidationPassId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Metadata required to register a validation pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationPassDescriptor {
    /// Stable unique identifier.
    pub id: ValidationPassId,

    /// Phase in which the pass runs.
    pub phase: ValidationPhase,

    /// Stable identifiers of prerequisite passes.
    ///
    /// Every prerequisite must be registered. A prerequisite must run in an
    /// earlier phase, or in the same phase with a lower identifier ordering.
    /// Registration checks these conditions when a complete pipeline is
    /// finalized.
    pub prerequisites: Vec<ValidationPassId>,
}

impl ValidationPassDescriptor {
    /// Creates a descriptor with no explicit pass prerequisites.
    #[must_use]
    pub fn new(id: ValidationPassId, phase: ValidationPhase) -> Self {
        Self {
            id,
            phase,
            prerequisites: Vec::new(),
        }
    }

    /// Declares prerequisite pass identifiers.
    #[must_use]
    pub fn with_prerequisites(
        mut self,
        prerequisites: impl IntoIterator<Item = ValidationPassId>,
    ) -> Self {
        self.prerequisites = prerequisites.into_iter().collect();
        self.prerequisites.sort();
        self.prerequisites.dedup();
        self
    }
}

/// Error returned by a pass when it cannot complete its work.
///
/// Semantic errors in the source program should normally be diagnostics,
/// not pass failures. A pass failure indicates that its analysis did not
/// complete reliably, for example because a required service was unavailable
/// or an internal invariant was violated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationPassError {
    message: String,
}

impl ValidationPassError {
    /// Creates a nonempty orchestration-level failure.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        let message = message.into();
        let message = if message.trim().is_empty() {
            "validation pass failed without an explanation".to_owned()
        } else {
            message
        };

        Self { message }
    }

    /// Returns the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ValidationPassError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ValidationPassError {}

/// A semantic validation pass.
///
/// `C` is the pipeline's shared semantic context. It must provide read access
/// to the canonical AST and the services required by the pass.
///
/// `D` is the repository's canonical diagnostic type. It must implement
/// `Ord` with stable ordering for deterministic report finalization.
///
/// Pass implementations must not store per-analysis mutable state in global
/// variables. Mutable state belongs to the validation session or the pass
/// invocation itself.
pub trait ValidationPass<C, D>: Send + Sync
where
    D: Ord,
{
    /// Returns immutable registration metadata.
    fn descriptor(&self) -> &ValidationPassDescriptor;

    /// Validates the program using the supplied shared context.
    ///
    /// Return `Ok(())` after completing this pass, even if source errors were
    /// found and reported to the sink. Return `Err` only when the pass could
    /// not complete reliably.
    fn validate(
        &self,
        context: &C,
        diagnostics: &mut DiagnosticSink<D>,
    ) -> Result<(), ValidationPassError>;
}

// =============================================================================
// Registration errors
// =============================================================================

/// Errors detected while registering or finalizing validation passes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegistrationError {
    /// A pass identifier was empty or whitespace-only.
    EmptyPassId,

    /// A pass identifier had leading or trailing whitespace.
    SurroundingWhitespace {
        /// Invalid identifier supplied by the caller.
        identifier: String,
    },

    /// A pass identifier was registered more than once.
    DuplicatePassId {
        /// Identifier already registered.
        identifier: ValidationPassId,
    },

    /// A declared prerequisite was not registered.
    MissingPrerequisite {
        /// Pass whose descriptor contains the missing prerequisite.
        pass: ValidationPassId,

        /// Missing prerequisite identifier.
        prerequisite: ValidationPassId,
    },

    /// A prerequisite is scheduled after the pass that needs it.
    PrerequisiteOrder {
        /// Pass that declares the dependency.
        pass: ValidationPassId,

        /// Prerequisite that cannot be guaranteed to run first.
        prerequisite: ValidationPassId,
    },

    /// A dependency cycle prevents a valid pass order.
    DependencyCycle {
        /// Pass identifiers participating in or blocked by the cycle.
        passes: Vec<ValidationPassId>,
    },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPassId => {
                formatter.write_str("validation pass identifier must not be empty")
            }
            Self::SurroundingWhitespace { identifier } => write!(
                formatter,
                "validation pass identifier has surrounding whitespace: {identifier:?}"
            ),
            Self::DuplicatePassId { identifier } => {
                write!(formatter, "duplicate validation pass identifier: {identifier}")
            }
            Self::MissingPrerequisite { pass, prerequisite } => write!(
                formatter,
                "validation pass {pass} requires unregistered pass {prerequisite}"
            ),
            Self::PrerequisiteOrder { pass, prerequisite } => write!(
                formatter,
                "validation pass {pass} is ordered before its prerequisite {prerequisite}"
            ),
            Self::DependencyCycle { passes } => {
                write!(formatter, "validation pass dependency cycle: ")?;
                for (index, pass) in passes.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "{pass}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for RegistrationError {}

// =============================================================================
// Diagnostic sink
// =============================================================================

/// Result of attempting to retain a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticPush {
    /// The diagnostic was retained.
    Retained,

    /// The diagnostic duplicated an already retained diagnostic.
    Duplicate,

    /// The diagnostic was not retained because the configured budget was
    /// already exhausted.
    BudgetExceeded,
}

/// Collects diagnostics under an optional retained-item budget.
///
/// This type intentionally does not define a diagnostic schema. The canonical
/// semantic diagnostic type remains owned by the semantic diagnostics module.
#[derive(Debug)]
pub struct DiagnosticSink<D>
where
    D: Ord,
{
    diagnostics: BTreeSet<D>,
    max_diagnostics: Option<usize>,
    omitted_diagnostics: usize,
}

impl<D> DiagnosticSink<D>
where
    D: Ord,
{
    /// Creates a diagnostic sink with the specified retention budget.
    #[must_use]
    pub fn new(max_diagnostics: Option<usize>) -> Self {
        Self {
            diagnostics: BTreeSet::new(),
            max_diagnostics,
            omitted_diagnostics: 0,
        }
    }

    /// Attempts to retain a diagnostic.
    ///
    /// Identical diagnostics are deduplicated. When the retention budget is
    /// exhausted, additional diagnostics are counted without being stored.
    ///
    /// The omitted count saturates instead of overflowing.
    pub fn push(&mut self, diagnostic: D) -> DiagnosticPush {
        if self.diagnostics.contains(&diagnostic) {
            return DiagnosticPush::Duplicate;
        }

        if self
            .max_diagnostics
            .is_some_and(|limit| self.diagnostics.len() >= limit)
        {
            self.omitted_diagnostics = self.omitted_diagnostics.saturating_add(1);
            return DiagnosticPush::BudgetExceeded;
        }

        self.diagnostics.insert(diagnostic);
        DiagnosticPush::Retained
    }

    /// Returns whether the retention budget has been reached.
    ///
    /// This does not mean that more diagnostics necessarily exist. It means
    /// that the sink cannot guarantee that any additional distinct
    /// diagnostic will be retained.
    #[must_use]
    pub fn is_at_capacity(&self) -> bool {
        self.max_diagnostics
            .is_some_and(|limit| self.diagnostics.len() >= limit)
    }

    /// Returns the number of retained distinct diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether no diagnostics have been retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns the number of diagnostics that could not be retained.
    #[must_use]
    pub fn omitted_count(&self) -> usize {
        self.omitted_diagnostics
    }

    /// Returns retained diagnostics in deterministic order.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &D> {
        self.diagnostics.iter()
    }

    fn into_parts(self) -> (Vec<D>, usize) {
        (
            self.diagnostics.into_iter().collect(),
            self.omitted_diagnostics,
        )
    }
}

// =============================================================================
// Pipeline result
// =============================================================================

/// Completion status of semantic validation.
///
/// A complete run means all registered passes selected for that run completed.
/// It does not mean the source is semantically valid: source errors may still
/// be present in the diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValidationStatus {
    /// All selected passes completed and diagnostic collection was complete.
    Complete,

    /// A configured operational budget prevented complete analysis.
    BudgetExceeded,

    /// A validation pass failed to complete reliably.
    PassFailed,
}

/// Details of a pass that failed during execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedPass {
    /// Stable identity of the failed pass.
    pub pass: ValidationPassId,

    /// Explanation supplied by the pass.
    pub error: ValidationPassError,
}

/// Final report from one validation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport<D> {
    /// Whether analysis completed reliably.
    pub status: ValidationStatus,

    /// Retained diagnostics in deterministic order.
    pub diagnostics: Vec<D>,

    /// Number of diagnostics omitted because the retention budget was reached.
    pub omitted_diagnostics: usize,

    /// Number of passes actually started.
    pub passes_started: usize,

    /// Number of passes that returned successfully.
    pub passes_completed: usize,

    /// Failed pass, if a pass did not complete reliably.
    pub failed_pass: Option<FailedPass>,
}

impl<D> ValidationReport<D> {
    /// Returns true only if validation completed and no diagnostics were
    /// omitted. This is not a substitute for checking semantic error severity.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status == ValidationStatus::Complete && self.omitted_diagnostics == 0
    }

    /// Returns whether the pipeline completed without a pass failure.
    #[must_use]
    pub fn passes_completed_reliably(&self) -> bool {
        self.failed_pass.is_none()
    }

    /// Returns whether any diagnostics were retained or omitted.
    #[must_use]
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty() || self.omitted_diagnostics != 0
    }
}

// =============================================================================
// Pipeline
// =============================================================================

/// A deterministic semantic validation pipeline.
///
/// Passes are stored in registration-independent order: phase first, then
/// stable pass identifier. Dependencies are checked when the pipeline is
/// finalized. Domain growth therefore does not require a central exhaustive
/// match over every supported computing domain.
///
/// `C` must be the canonical shared semantic context supplied by the parent
/// semantic subsystem. `D` must be its canonical diagnostic type.
pub struct ValidationPipeline<C, D>
where
    D: Ord,
{
    config: ValidationConfig,
    passes: Vec<Box<dyn ValidationPass<C, D>>>,
    identifiers: BTreeSet<ValidationPassId>,
    finalized: bool,
}

impl<C, D> fmt::Debug for ValidationPipeline<C, D>
where
    D: Ord,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidationPipeline")
            .field("config", &self.config)
            .field("registered_passes", &self.passes.len())
            .field("finalized", &self.finalized)
            .finish()
    }
}

impl<C, D> ValidationPipeline<C, D>
where
    D: Ord,
{
    /// Creates an empty pipeline.
    #[must_use]
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            config,
            passes: Vec::new(),
            identifiers: BTreeSet::new(),
            finalized: false,
        }
    }

    /// Returns the current operational policy.
    #[must_use]
    pub const fn config(&self) -> ValidationConfig {
        self.config
    }

    /// Registers a validation pass.
    ///
    /// Registration is permitted only before finalization. Duplicate IDs are
    /// rejected immediately; dependency completeness and ordering are checked
    /// by `finalize`.
    pub fn register<P>(&mut self, pass: P) -> Result<(), RegistrationError>
    where
        P: ValidationPass<C, D> + 'static,
    {
        if self.finalized {
            // A finalized pipeline has a stable execution plan. Rebuilding
            // it is explicit and avoids silently invalidating dependencies.
            return Err(RegistrationError::PrerequisiteOrder {
                pass: pass.descriptor().id.clone(),
                prerequisite: pass.descriptor().id.clone(),
            });
        }

        let identifier = pass.descriptor().id.clone();

        if !self.identifiers.insert(identifier.clone()) {
            return Err(RegistrationError::DuplicatePassId { identifier });
        }

        self.passes.push(Box::new(pass));
        Ok(())
    }

    /// Validates dependencies and freezes the pipeline's execution order.
    ///
    /// This operation is deterministic and does not execute any validation.
    pub fn finalize(&mut self) -> Result<(), RegistrationError> {
        if self.finalized {
            return Ok(());
        }

        self.passes.sort_by(|left, right| {
            left.descriptor()
                .phase
                .cmp(&right.descriptor().phase)
                .then_with(|| left.descriptor().id.cmp(&right.descriptor().id))
        });

        let mut ordered = BTreeSet::new();

        for pass in &self.passes {
            let descriptor = pass.descriptor();

            for prerequisite in &descriptor.prerequisites {
                let Some(prerequisite_pass) = self
                    .passes
                    .iter()
                    .find(|candidate| candidate.descriptor().id == *prerequisite)
                else {
                    return Err(RegistrationError::MissingPrerequisite {
                        pass: descriptor.id.clone(),
                        prerequisite: prerequisite.clone(),
                    });
                };

                let prerequisite_descriptor = prerequisite_pass.descriptor();

                if prerequisite_descriptor.phase > descriptor.phase
                    || (prerequisite_descriptor.phase == descriptor.phase
                        && prerequisite_descriptor.id >= descriptor.id)
                {
                    return Err(RegistrationError::PrerequisiteOrder {
                        pass: descriptor.id.clone(),
                        prerequisite: prerequisite.clone(),
                    });
                }

                if !ordered.contains(prerequisite) {
                    return Err(RegistrationError::DependencyCycle {
                        passes: vec![descriptor.id.clone(), prerequisite.clone()],
                    });
                }
            }

            ordered.insert(descriptor.id.clone());
        }

        self.finalized = true;
        Ok(())
    }

    /// Returns the registered pass descriptors in their current order.
    ///
    /// Before finalization this is registration order. After finalization it
    /// is deterministic execution order.
    #[must_use]
    pub fn descriptors(&self) -> Vec<ValidationPassDescriptor> {
        self.passes
            .iter()
            .map(|pass| pass.descriptor().clone())
            .collect()
    }

    /// Executes the finalized pipeline.
    ///
    /// If the pipeline has not been finalized, it is finalized first.
    ///
    /// A pass failure stops execution immediately. Exhausting the configured
    /// pass budget stops before starting the next pass. Exhausting the
    /// diagnostic retention budget marks the result incomplete.
    pub fn run(
        &mut self,
        context: &C,
    ) -> Result<ValidationReport<D>, RegistrationError> {
        self.finalize()?;

        let mut sink = DiagnosticSink::new(self.config.max_diagnostics);
        let mut passes_started = 0usize;
        let mut passes_completed = 0usize;
        let mut failed_pass = None;
        let mut status = ValidationStatus::Complete;

        for pass in &self.passes {
            if self
                .config
                .max_passes
                .is_some_and(|limit| passes_started >= limit)
            {
                status = ValidationStatus::BudgetExceeded;
                break;
            }

            // A zero diagnostic budget cannot retain diagnostics. It does
            // not, by itself, prove that any diagnostic exists, so passes
            // are still allowed to run and determine their semantic result.
            passes_started = passes_started.saturating_add(1);

            match pass.validate(context, &mut sink) {
                Ok(()) => {
                    passes_completed = passes_completed.saturating_add(1);
                }
                Err(error) => {
                    status = ValidationStatus::PassFailed;
                    failed_pass = Some(FailedPass {
                        pass: pass.descriptor().id.clone(),
                        error,
                    });
                    break;
                }
            }

            if sink.omitted_count() != 0 {
                status = ValidationStatus::BudgetExceeded;
                break;
            }
        }

        let (diagnostics, omitted_diagnostics) = sink.into_parts();

        if omitted_diagnostics != 0 {
            status = ValidationStatus::BudgetExceeded;
        }

        Ok(ValidationReport {
            status,
            diagnostics,
            omitted_diagnostics,
            passes_started,
            passes_completed,
            failed_pass,
        })
    }
}

// =============================================================================
// Contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestDiagnostic(&'static str);

    struct TestPass {
        descriptor: ValidationPassDescriptor,
        emitted: Vec<TestDiagnostic>,
        fail: bool,
    }

    impl TestPass {
        fn new(
            id: &str,
            phase: ValidationPhase,
            emitted: Vec<TestDiagnostic>,
        ) -> Self {
            Self {
                descriptor: ValidationPassDescriptor::new(
                    ValidationPassId::new(id).expect("test ID is valid"),
                    phase,
                ),
                emitted,
                fail: false,
            }
        }

        fn failing(id: &str, phase: ValidationPhase) -> Self {
            let mut pass = Self::new(id, phase, Vec::new());
            pass.fail = true;
            pass
        }
    }

    impl ValidationPass<(), TestDiagnostic> for TestPass {
        fn descriptor(&self) -> &ValidationPassDescriptor {
            &self.descriptor
        }

        fn validate(
            &self,
            _context: &(),
            diagnostics: &mut DiagnosticSink<TestDiagnostic>,
        ) -> Result<(), ValidationPassError> {
            for diagnostic in &self.emitted {
                diagnostics.push(diagnostic.clone());
            }

            if self.fail {
                return Err(ValidationPassError::new("intentional test failure"));
            }

            Ok(())
        }
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.frontend.semantic.validation"
        );
        assert_eq!(schema_version(), VALIDATION_MODULE_SCHEMA_VERSION);
        assert!(schema_version() > 0);
    }

    #[test]
    fn unrestricted_configuration_has_no_artificial_budgets() {
        let config = ValidationConfig::unrestricted();

        assert_eq!(config.max_passes, None);
        assert_eq!(config.max_diagnostics, None);
    }

    #[test]
    fn empty_pass_identifiers_are_rejected() {
        assert!(matches!(
            ValidationPassId::new("  "),
            Err(RegistrationError::EmptyPassId)
        ));
    }

    #[test]
    fn surrounding_identifier_whitespace_is_rejected() {
        assert!(matches!(
            ValidationPassId::new(" semantic.types "),
            Err(RegistrationError::SurroundingWhitespace { .. })
        ));
    }

    #[test]
    fn duplicate_pass_ids_are_rejected() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        pipeline
            .register(TestPass::new(
                "core.declarations",
                ValidationPhase::Declarations,
                Vec::new(),
            ))
            .expect("first registration succeeds");

        assert!(matches!(
            pipeline.register(TestPass::new(
                "core.declarations",
                ValidationPhase::Declarations,
                Vec::new(),
            )),
            Err(RegistrationError::DuplicatePassId { .. })
        ));
    }

    #[test]
    fn passes_run_in_stable_phase_and_identifier_order() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        pipeline
            .register(TestPass::new(
                "z.expressions",
                ValidationPhase::Expressions,
                vec![TestDiagnostic("expression")],
            ))
            .expect("registration succeeds");

        pipeline
            .register(TestPass::new(
                "a.declarations",
                ValidationPhase::Declarations,
                vec![TestDiagnostic("declaration")],
            ))
            .expect("registration succeeds");

        pipeline
            .register(TestPass::new(
                "a.expressions",
                ValidationPhase::Expressions,
                vec![TestDiagnostic("expression-a")],
            ))
            .expect("registration succeeds");

        let report = pipeline.run(&()).expect("pipeline finalizes");

        assert_eq!(report.status, ValidationStatus::Complete);
        assert_eq!(report.passes_started, 3);
        assert_eq!(report.passes_completed, 3);

        let descriptors = pipeline.descriptors();
        let ids: Vec<_> = descriptors
            .iter()
            .map(|descriptor| descriptor.id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec!["a.declarations", "a.expressions", "z.expressions"]
        );
    }

    #[test]
    fn diagnostics_are_sorted_and_deduplicated() {
        let mut sink = DiagnosticSink::new(None);

        assert_eq!(
            sink.push(TestDiagnostic("z")),
            DiagnosticPush::Retained
        );
        assert_eq!(
            sink.push(TestDiagnostic("a")),
            DiagnosticPush::Retained
        );
        assert_eq!(
            sink.push(TestDiagnostic("z")),
            DiagnosticPush::Duplicate
        );

        let (diagnostics, omitted) = sink.into_parts();

        assert_eq!(
            diagnostics,
            vec![TestDiagnostic("a"), TestDiagnostic("z")]
        );
        assert_eq!(omitted, 0);
    }

    #[test]
    fn diagnostic_budget_is_reported_as_incomplete() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted().with_max_diagnostics(Some(1)),
        );

        pipeline
            .register(TestPass::new(
                "core.expressions",
                ValidationPhase::Expressions,
                vec![
                    TestDiagnostic("first"),
                    TestDiagnostic("second"),
                ],
            ))
            .expect("registration succeeds");

        let report = pipeline.run(&()).expect("pipeline finalizes");

        assert_eq!(report.status, ValidationStatus::BudgetExceeded);
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(report.omitted_diagnostics, 1);
        assert!(!report.is_complete());
    }

    #[test]
    fn pass_budget_is_reported_as_incomplete() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted().with_max_passes(Some(1)),
        );

        pipeline
            .register(TestPass::new(
                "core.declarations",
                ValidationPhase::Declarations,
                Vec::new(),
            ))
            .expect("registration succeeds");

        pipeline
            .register(TestPass::new(
                "core.expressions",
                ValidationPhase::Expressions,
                Vec::new(),
            ))
            .expect("registration succeeds");

        let report = pipeline.run(&()).expect("pipeline finalizes");

        assert_eq!(report.status, ValidationStatus::BudgetExceeded);
        assert_eq!(report.passes_started, 1);
        assert_eq!(report.passes_completed, 1);
        assert!(!report.is_complete());
    }

    #[test]
    fn pass_failure_stops_later_passes() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        pipeline
            .register(TestPass::failing(
                "core.declarations",
                ValidationPhase::Declarations,
            ))
            .expect("registration succeeds");

        pipeline
            .register(TestPass::new(
                "core.expressions",
                ValidationPhase::Expressions,
                Vec::new(),
            ))
            .expect("registration succeeds");

        let report = pipeline.run(&()).expect("pipeline finalizes");

        assert_eq!(report.status, ValidationStatus::PassFailed);
        assert_eq!(report.passes_started, 1);
        assert_eq!(report.passes_completed, 0);
        assert_eq!(
            report.failed_pass.as_ref().map(|failure| failure.pass.as_str()),
            Some("core.declarations")
        );
    }

    #[test]
    fn missing_prerequisites_are_rejected() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        let missing = ValidationPassId::new("core.declarations")
            .expect("test ID is valid");

        let descriptor = ValidationPassDescriptor::new(
            ValidationPassId::new("core.expressions").expect("test ID is valid"),
            ValidationPhase::Expressions,
        )
        .with_prerequisites([missing.clone()]);

        let pass = TestPass {
            descriptor,
            emitted: Vec::new(),
            fail: false,
        };

        pipeline.register(pass).expect("registration succeeds");

        assert!(matches!(
            pipeline.finalize(),
            Err(RegistrationError::MissingPrerequisite {
                prerequisite,
                ..
            }) if prerequisite == missing
        ));
    }

    #[test]
    fn prerequisites_must_precede_the_dependent_pass() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        pipeline
            .register(TestPass::new(
                "core.z_prerequisite",
                ValidationPhase::Expressions,
                Vec::new(),
            ))
            .expect("registration succeeds");

        let prerequisite = ValidationPassId::new("core.z_prerequisite")
            .expect("test ID is valid");

        let descriptor = ValidationPassDescriptor::new(
            ValidationPassId::new("core.a_dependent").expect("test ID is valid"),
            ValidationPhase::Expressions,
        )
        .with_prerequisites([prerequisite.clone()]);

        pipeline
            .register(TestPass {
                descriptor,
                emitted: Vec::new(),
                fail: false,
            })
            .expect("registration succeeds");

        assert!(matches!(
            pipeline.finalize(),
            Err(RegistrationError::PrerequisiteOrder { .. })
        ));
    }

    #[test]
    fn finalized_pipeline_rejects_new_registration() {
        let mut pipeline = ValidationPipeline::<(), TestDiagnostic>::new(
            ValidationConfig::unrestricted(),
        );

        pipeline.finalize().expect("empty pipeline finalizes");

        assert!(pipeline
            .register(TestPass::new(
                "core.declarations",
                ValidationPhase::Declarations,
                Vec::new(),
            ))
            .is_err());
    }
}
