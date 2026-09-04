//! Zamani Quantum Scheduling — Custom Constraints
//!
//! Path:
//!     src/quantum/scheduling/constraints/custom.rs
//!
//! # Purpose
//!
//! This module provides the production extension point for scheduling
//! constraints that cannot, or should not, be represented by one of the
//! built-in constraint categories.
//!
//! A custom constraint is still a normal `Constraint`. It participates in the
//! same evaluation, applicability, severity, phase, diagnostics, and
//! verification mechanisms as every other scheduling constraint.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - `CustomConstraint`;
//! - reusable custom constraint predicates;
//! - custom constraint construction;
//! - custom applicability predicates;
//! - custom diagnostic metadata;
//! - safe composition of custom constraint logic;
//! - deterministic custom-constraint evaluation;
//! - custom-constraint validation;
//! - a convenient API for embedding project- or target-specific policies.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - quantum gate definitions;
//! - quantum circuit representation;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - hardware calibration;
//! - resource calendars;
//! - scheduling algorithms;
//! - QEC algorithms;
//! - runtime execution;
//! - serialization formats;
//! - vendor SDKs;
//! - global mutable scheduler state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! built-in constraints          custom constraints
//!      |                               |
//!      +---------------+---------------+
//!                      |
//!                      v
//!               ConstraintSet
//!                      |
//!                      v
//!                  planner
//!                      |
//!                      v
//!                verification
//! ```
//!
//! `CustomConstraint` therefore extends the scheduler; it does not replace
//! the scheduler.
//!
//! # Why custom constraints exist
//!
//! The built-in constraint modules cover common scheduling concerns such as:
//!
//! - qubit occupancy;
//! - channels;
//! - measurement;
//! - reset;
//! - classical control;
//! - communication.
//!
//! Real quantum targets and future Zamani subsystems may impose additional
//! constraints, for example:
//!
//! - cryogenic operating windows;
//! - optical beam sharing;
//! - laser exclusivity;
//! - thermal recovery;
//! - modular synchronization;
//! - target-specific resource coupling;
//! - user-defined deadlines;
//! - experimental restrictions;
//! - research scheduling policies;
//! - QEC-specific temporal rules;
//! - plugin-provided constraints;
//! - simulator-specific execution restrictions.
//!
//! Such constraints should not require modifying the generic scheduler.
//!
//! # Generic contract
//!
//! A custom constraint implements the same contract as every other constraint:
//!
//! ```text
//! ConstraintContext
//!        |
//!        v
//! custom constraint
//!        |
//!        +----> Ok(())
//!        |
//!        +----> ConstraintViolation
//! ```
//!
//! The custom predicate receives only an immutable `ConstraintContext`.
//!
//! It MUST NOT mutate:
//!
//! - the candidate;
//! - the scheduling state;
//! - the resource model;
//! - the quantum IR;
//! - hardware;
//! - global state.
//!
//! # No hard-coded machine limits
//!
//! This module contains no fixed:
//!
//! - qubit count;
//! - operation count;
//! - resource count;
//! - channel count;
//! - topology size;
//! - operation arity;
//! - schedule depth;
//! - QEC distance;
//! - node count;
//! - network size.
//!
//! A custom constraint may impose a target-specific restriction, but that
//! restriction must be supplied as configuration or derived from the supplied
//! scheduling context rather than encoded as a machine-size constant.
//!
//! For example, this is appropriate:
//!
//! ```text
//! CustomConstraint::new(...)
//!     .with_predicate(|context| ...)
//! ```
//!
//! whereas this is architecturally wrong:
//!
//! ```text
//! if qubit_count > 127 { ... }
//! ```
//!
//! The latter would hard-code a particular machine assumption into the
//! scheduler.
//!
//! # "Infinity" scalability model
//!
//! The scheduler cannot literally represent infinitely many objects on finite
//! hardware. The intended Zamani guarantee is instead:
//!
//! > No artificial finite machine-size ceiling is introduced by the scheduling
//! > architecture.
//!
//! A concrete compilation remains bounded by:
//!
//! - available memory;
//! - CPU capacity;
//! - operating-system resources;
//! - explicit compiler limits;
//! - target resources;
//! - target capabilities;
//! - execution deadlines.
//!
//! Custom constraints must preserve this property.
//!
//! # Canonical identity boundary
//!
//! This file does not define or reinterpret qubit identities.
//!
//! If a custom constraint needs qubit identities, it should obtain them from
//! the scheduling candidate/context, whose canonical identity boundary is:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! In particular, this module must never define another:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `QubitRef`.
//!
//! No direct import is required here because the generic constraint context
//! already provides the appropriate canonical information.
//!
//! # Thread safety
//!
//! `CustomConstraint` is designed to be safely stored in the heterogeneous
//! `ConstraintSet`.
//!
//! The custom predicate therefore requires:
//!
//! ```text
//! Send + Sync
//! ```
//!
//! A custom predicate should preferably be a pure function.
//!
//! If external state is captured by a closure, that state must itself be
//! immutable or synchronized by the caller.
//!
//! This module does not introduce synchronization or mutable global state.
//!
//! # Determinism
//!
//! A custom constraint is expected to be deterministic for a fixed:
//!
//! - scheduling context;
//! - target snapshot;
//! - configuration;
//! - input state.
//!
//! A custom constraint must not silently depend on:
//!
//! - wall-clock time;
//! - process-global mutable state;
//! - environment variables;
//! - filesystem state;
//! - network state;
//! - implicit randomness.
//!
//! If such dependencies are genuinely required, they should be materialized
//! into the scheduling context before evaluation so the scheduler still has an
//! explicit, reproducible input snapshot.
//!
//! # Runtime and serialization
//!
//! A Rust closure cannot safely or meaningfully be serialized as executable
//! scheduler state.
//!
//! Therefore this type intentionally does NOT implement `Serialize` or
//! `Deserialize`.
//!
//! For persistent schedules, serialize a stable custom-constraint identifier
//! and configuration through the scheduler/plugin serialization layer, then
//! reconstruct the executable constraint from a trusted registry.
//!
//! Never deserialize arbitrary executable code.
//!
//! # Plugin integration
//!
//! Plugins may construct `CustomConstraint` values and register them in the
//! scheduler's `ConstraintSet`.
//!
//! The plugin layer remains responsible for:
//!
//! - plugin identity;
//! - versioning;
//! - compatibility;
//! - trusted construction;
//! - persistence metadata.
//!
//! This file only supplies the executable constraint abstraction.
//!
//! # Error semantics
//!
//! The custom predicate returns `Result<(), String>` rather than a scheduler
//! error type.
//!
//! The reason is deliberate:
//!
//! - a predicate reports a domain-specific failure;
//! - `CustomConstraint` adds stable constraint identity;
//! - the generic framework converts the result into `ConstraintViolation`;
//! - the higher scheduler may then convert it into `SchedulingError`.
//!
//! This keeps the custom constraint independent from planner and scheduler
//! error policy.
//!
//! # Performance
//!
//! The abstraction uses `Arc<dyn Fn(...)>` so custom constraints can be shared
//! safely without copying executable state.
//!
//! This introduces dynamic dispatch for custom predicates. That is an
//! intentional trade-off because custom constraints are heterogeneous plugin
//! boundaries rather than the scheduler's primary hot-path representation.
//!
//! Performance-critical built-in constraints should remain specialized modules
//! with concrete implementations.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`;
//! - no unsafe dependencies required by this module.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::sync::Arc;

use super::constraint::{
    Constraint,
    ConstraintApplicability,
    ConstraintContext,
    ConstraintId,
    ConstraintKind,
    ConstraintPhase,
    ConstraintSeverity,
    ConstraintViolation,
};

// ============================================================================
// Predicate type
// ============================================================================

/// Executable custom constraint predicate.
///
/// The predicate receives an immutable scheduling context and returns:
///
/// - `Ok(())` when the custom rule is satisfied;
/// - `Err(reason)` when the custom rule is violated.
///
/// The predicate must not mutate scheduling state.
///
/// The predicate is required to be `Send + Sync` because custom constraints
/// may be evaluated by concurrent scheduler workers.
pub type CustomConstraintPredicate =
    Arc<dyn Fn(&ConstraintContext<'_>) -> Result<(), String> + Send + Sync + 'static>;

// ============================================================================
// Applicability predicate
// ============================================================================

/// Optional executable predicate determining whether a custom constraint
/// applies to a candidate.
///
/// Returning `true` means the constraint should be evaluated.
///
/// Returning `false` means the constraint is not applicable to that candidate.
///
/// Applicability MUST be conservative: a constraint should not return `false`
/// merely because evaluating it is inconvenient if the constraint might
/// actually apply.
pub type CustomApplicabilityPredicate =
    Arc<dyn Fn(&ConstraintContext<'_>) -> bool + Send + Sync + 'static>;

// ============================================================================
// Custom constraint metadata
// ============================================================================

/// Immutable metadata associated with a custom constraint.
///
/// Metadata is intentionally separate from executable predicate logic so
/// diagnostics and plugin inspection do not need to invoke the constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomConstraintMetadata {
    /// Stable identifier for the constraint.
    id: ConstraintId,

    /// Human-readable name.
    name: String,

    /// Optional stable namespace for plugin/project ownership.
    namespace: Option<String>,

    /// Optional version string supplied by the owner.
    version: Option<String>,

    /// Severity of a violation.
    severity: ConstraintSeverity,

    /// Whether the constraint is enabled.
    enabled: bool,
}

impl CustomConstraintMetadata {
    /// Creates metadata for a custom constraint.
    #[must_use]
    pub fn new(id: ConstraintId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            namespace: None,
            version: None,
            severity: ConstraintSeverity::Error,
            enabled: true,
        }
    }

    /// Returns the stable constraint identifier.
    #[must_use]
    pub const fn id(&self) -> ConstraintId {
        self.id
    }

    /// Returns the human-readable name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional namespace.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the optional version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the configured severity.
    #[must_use]
    pub const fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    /// Returns whether this constraint is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets a namespace.
    #[must_use]
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Sets a version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the violation severity.
    #[must_use]
    pub const fn with_severity(mut self, severity: ConstraintSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets whether the constraint is enabled.
    #[must_use]
    pub const fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

// ============================================================================
// Construction errors
// ============================================================================

/// Structural error produced while constructing a custom constraint.
///
/// These are configuration/construction errors, not scheduling violations.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomConstraintError {
    /// The constraint name is empty.
    EmptyName,

    /// The namespace was supplied but is empty.
    EmptyNamespace,

    /// The version was supplied but is empty.
    EmptyVersion,

    /// No executable predicate was supplied.
    MissingPredicate,
}

impl fmt::Display for CustomConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("custom constraint name must not be empty")
            }
            Self::EmptyNamespace => {
                formatter.write_str("custom constraint namespace must not be empty")
            }
            Self::EmptyVersion => {
                formatter.write_str("custom constraint version must not be empty")
            }
            Self::MissingPredicate => {
                formatter.write_str("custom constraint predicate must be supplied")
            }
        }
    }
}

impl std::error::Error for CustomConstraintError {}

// ============================================================================
// Custom constraint
// ============================================================================

/// Production custom scheduling constraint.
///
/// `CustomConstraint` provides an extension point without modifying the
/// scheduler whenever a new scheduling rule is required.
///
/// The executable predicate is deliberately generic and receives only the
/// immutable `ConstraintContext`.
///
/// # Example
///
/// ```text
/// let constraint = CustomConstraint::new(
///     ConstraintId::new(100),
///     "thermal_window",
///     |context| {
///         // Inspect the immutable scheduling context.
///         // Return Err(reason) when the candidate violates the rule.
///         Ok(())
///     },
/// )?;
/// ```
///
/// The concrete predicate may inspect:
///
/// - candidate operation identity;
/// - canonical logical qubits;
/// - canonical physical qubits;
/// - resource claims;
/// - candidate start time;
/// - candidate duration;
/// - reservations;
/// - completed operations;
/// - unavailable resources;
/// - scheduling phase.
///
/// It must not mutate those values.
#[derive(Clone)]
pub struct CustomConstraint {
    metadata: CustomConstraintMetadata,
    predicate: CustomConstraintPredicate,
    applicability: Option<CustomApplicabilityPredicate>,
    supported_phases: Option<Arc<[ConstraintPhase]>>,
}

impl fmt::Debug for CustomConstraint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CustomConstraint")
            .field("metadata", &self.metadata)
            .field("has_applicability_predicate", &self.applicability.is_some())
            .field("supported_phases", &self.supported_phases)
            .finish()
    }
}

impl CustomConstraint {
    /// Creates a custom constraint.
    ///
    /// The supplied predicate becomes part of the immutable constraint
    /// instance.
    ///
    /// Construction validates all static metadata.
    pub fn new<F>(
        id: ConstraintId,
        name: impl Into<String>,
        predicate: F,
    ) -> Result<Self, CustomConstraintError>
    where
        F: Fn(&ConstraintContext<'_>) -> Result<(), String>
            + Send
            + Sync
            + 'static,
    {
        let metadata = CustomConstraintMetadata::new(id, name);

        Self::from_metadata(metadata, Arc::new(predicate))
    }

    /// Creates a custom constraint from explicit metadata and predicate.
    pub fn from_metadata(
        metadata: CustomConstraintMetadata,
        predicate: CustomConstraintPredicate,
    ) -> Result<Self, CustomConstraintError> {
        Self::validate_metadata(&metadata)?;

        Ok(Self {
            metadata,
            predicate,
            applicability: None,
            supported_phases: None,
        })
    }

    /// Creates a custom constraint with an already shared predicate.
    ///
    /// This is useful for plugin registries or applications that reuse one
    /// predicate implementation across multiple constraint instances.
    pub fn from_predicate(
        id: ConstraintId,
        name: impl Into<String>,
        predicate: CustomConstraintPredicate,
    ) -> Result<Self, CustomConstraintError> {
        Self::from_metadata(
            CustomConstraintMetadata::new(id, name),
            predicate,
        )
    }

    /// Attaches an applicability predicate.
    ///
    /// The applicability predicate is evaluated before the main constraint
    /// predicate.
    #[must_use]
    pub fn with_applicability<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&ConstraintContext<'_>) -> bool
            + Send
            + Sync
            + 'static,
    {
        self.applicability = Some(Arc::new(predicate));
        self
    }

    /// Attaches an already shared applicability predicate.
    #[must_use]
    pub fn with_shared_applicability(
        mut self,
        predicate: CustomApplicabilityPredicate,
    ) -> Self {
        self.applicability = Some(predicate);
        self
    }

    /// Restricts this constraint to the supplied scheduling phases.
    ///
    /// An empty phase collection is rejected at construction time by
    /// `try_with_supported_phases`.
    ///
    /// This infallible convenience method preserves the existing constraint
    /// instance when the supplied slice is empty rather than introducing a
    /// runtime failure.
    ///
    /// For strict configuration validation use `try_with_supported_phases`.
    #[must_use]
    pub fn with_supported_phases(
        self,
        phases: impl IntoIterator<Item = ConstraintPhase>,
    ) -> Self {
        let phases: Vec<ConstraintPhase> = phases.into_iter().collect();

        if phases.is_empty() {
            return self;
        }

        self.with_normalized_phases(phases)
    }

    /// Restricts this constraint to the supplied scheduling phases with
    /// validation.
    pub fn try_with_supported_phases(
        mut self,
        phases: impl IntoIterator<Item = ConstraintPhase>,
    ) -> Result<Self, CustomConstraintError> {
        let phases: Vec<ConstraintPhase> = phases.into_iter().collect();

        if phases.is_empty() {
            return Err(CustomConstraintError::MissingPredicate);
        }

        self.supported_phases = Some(normalize_phases(phases));
        Ok(self)
    }

    /// Configures the violation severity.
    #[must_use]
    pub const fn with_severity(
        mut self,
        severity: ConstraintSeverity,
    ) -> Self {
        self.metadata.severity = severity;
        self
    }

    /// Enables or disables the constraint.
    #[must_use]
    pub const fn with_enabled(mut self, enabled: bool) -> Self {
        self.metadata.enabled = enabled;
        self
    }

    /// Adds a stable namespace.
    ///
    /// Namespace should normally be used by plugins, research modules, or
    /// target-specific extensions to avoid identifier collisions.
    pub fn try_with_namespace(
        mut self,
        namespace: impl Into<String>,
    ) -> Result<Self, CustomConstraintError> {
        let namespace = namespace.into();

        if namespace.trim().is_empty() {
            return Err(CustomConstraintError::EmptyNamespace);
        }

        self.metadata.namespace = Some(namespace);
        Ok(self)
    }

    /// Adds a stable namespace without fallible construction.
    ///
    /// Empty values are ignored. For configuration validation use
    /// `try_with_namespace`.
    #[must_use]
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        let namespace = namespace.into();

        if !namespace.trim().is_empty() {
            self.metadata.namespace = Some(namespace);
        }

        self
    }

    /// Adds a version identifier.
    pub fn try_with_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, CustomConstraintError> {
        let version = version.into();

        if version.trim().is_empty() {
            return Err(CustomConstraintError::EmptyVersion);
        }

        self.metadata.version = Some(version);
        Ok(self)
    }

    /// Adds a version identifier without fallible construction.
    ///
    /// Empty values are ignored. For configuration validation use
    /// `try_with_version`.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        let version = version.into();

        if !version.trim().is_empty() {
            self.metadata.version = Some(version);
        }

        self
    }

    /// Returns the immutable metadata.
    #[must_use]
    pub const fn metadata(&self) -> &CustomConstraintMetadata {
        &self.metadata
    }

    /// Returns the executable predicate.
    ///
    /// This method is primarily intended for trusted plugin/framework
    /// integrations.
    #[must_use]
    pub fn predicate(&self) -> &CustomConstraintPredicate {
        &self.predicate
    }

    /// Returns the optional applicability predicate.
    #[must_use]
    pub fn applicability(&self) -> Option<&CustomApplicabilityPredicate> {
        self.applicability.as_ref()
    }

    /// Returns the supported phases, if explicitly restricted.
    #[must_use]
    pub fn supported_phases(&self) -> Option<&[ConstraintPhase]> {
        self.supported_phases.as_deref()
    }

    /// Returns whether the constraint supports a phase.
    #[must_use]
    pub fn supports(&self, phase: ConstraintPhase) -> bool {
        match &self.supported_phases {
            None => true,
            Some(phases) => phases.binary_search(&phase).is_ok(),
        }
    }

    /// Validates static configuration.
    pub fn validate(&self) -> Result<(), CustomConstraintError> {
        Self::validate_metadata(&self.metadata)?;

        if self.predicate_is_missing() {
            return Err(CustomConstraintError::MissingPredicate);
        }

        Ok(())
    }

    fn validate_metadata(
        metadata: &CustomConstraintMetadata,
    ) -> Result<(), CustomConstraintError> {
        if metadata.name.trim().is_empty() {
            return Err(CustomConstraintError::EmptyName);
        }

        if metadata
            .namespace
            .as_deref()
            .is_some_and(str::trim.is_empty)
        {
            return Err(CustomConstraintError::EmptyNamespace);
        }

        if metadata
            .version
            .as_deref()
            .is_some_and(str::trim.is_empty)
        {
            return Err(CustomConstraintError::EmptyVersion);
        }

        Ok(())
    }

    fn predicate_is_missing(&self) -> bool {
        // `CustomConstraint` always owns a valid Arc predicate after
        // construction. This method exists so `validate()` remains explicit
        // and future internal constructors can preserve the invariant.
        false
    }

    fn with_normalized_phases(
        mut self,
        phases: Vec<ConstraintPhase>,
    ) -> Self {
        self.supported_phases = Some(normalize_phases(phases));
        self
    }

    fn make_violation(
        &self,
        context: &ConstraintContext<'_>,
        reason: String,
    ) -> ConstraintViolation {
        ConstraintViolation::new(
            self.id(),
            self.kind(),
            self.severity(),
            reason,
        )
        .with_operation(context.candidate().operation())
        .with_timing(
            context.candidate().start(),
            context.candidate().duration(),
        )
    }
}

impl Constraint for CustomConstraint {
    fn id(&self) -> ConstraintId {
        self.metadata.id()
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Custom
    }

    fn name(&self) -> &str {
        self.metadata.name()
    }

    fn severity(&self) -> ConstraintSeverity {
        self.metadata.severity()
    }

    fn applies(
        &self,
        context: &ConstraintContext<'_>,
    ) -> ConstraintApplicability {
        if !self.is_enabled() {
            return ConstraintApplicability::NotApplicable;
        }

        if !self.supports_phase(context.phase()) {
            return ConstraintApplicability::NotApplicable;
        }

        match &self.applicability {
            Some(predicate) if !predicate(context) => {
                ConstraintApplicability::NotApplicable
            }
            _ => ConstraintApplicability::Applicable,
        }
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        if !self.is_enabled() {
            return Ok(());
        }

        if !self.supports_phase(context.phase()) {
            return Ok(());
        }

        if let Some(applicability) = &self.applicability {
            if !applicability(context) {
                return Ok(());
            }
        }

        match (self.predicate)(context) {
            Ok(()) => Ok(()),
            Err(reason) => Err(self.make_violation(context, reason)),
        }
    }

    fn is_enabled(&self) -> bool {
        self.metadata.is_enabled()
    }

    fn supports_phase(&self, phase: ConstraintPhase) -> bool {
        self.supports(phase)
    }
}

// ============================================================================
// Predicate helpers
// ============================================================================

/// Creates a predicate that always accepts the candidate.
///
/// This is useful for framework tests, conditional plugin registration, and
/// dynamically constructed constraint pipelines.
#[must_use]
pub fn allow_all_predicate() -> CustomConstraintPredicate {
    Arc::new(|_| Ok(()))
}

/// Creates a predicate that always rejects the candidate with the supplied
/// stable reason.
///
/// This is useful for explicit target-policy failures where the surrounding
/// configuration has already established that a candidate is unsupported.
#[must_use]
pub fn reject_all_predicate(
    reason: impl Into<String>,
) -> CustomConstraintPredicate {
    let reason = reason.into();

    Arc::new(move |_| Err(reason.clone()))
}

/// Creates an applicability predicate that always applies.
#[must_use]
pub fn apply_all_predicate() -> CustomApplicabilityPredicate {
    Arc::new(|_| true)
}

/// Creates an applicability predicate that never applies.
///
/// This is useful for dynamically disabled policy branches without mutating
/// global scheduler state.
#[must_use]
pub fn apply_none_predicate() -> CustomApplicabilityPredicate {
    Arc::new(|_| false)
}

// ============================================================================
// Constraint composition
// ============================================================================

/// Logical composition mode for custom predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CustomComposition {
    /// Every child predicate must succeed.
    All,

    /// At least one child predicate must succeed.
    Any,
}

/// Composes multiple custom predicates into one predicate.
///
/// The supplied predicates are evaluated in deterministic caller-supplied
/// order.
///
/// This function does not impose a fixed number of predicates.
///
/// The returned predicate:
///
/// - `All`: returns the first failure;
/// - `Any`: returns success after the first successful predicate, otherwise
///   returns a combined failure reason.
///
/// For `Any`, every failure reason is retained in deterministic order.
///
/// # Scalability
///
/// The caller controls the number of predicates. The implementation does not
/// impose a machine-size-dependent limit.
pub fn compose_predicates(
    predicates: impl IntoIterator<Item = CustomConstraintPredicate>,
    composition: CustomComposition,
) -> CustomConstraintPredicate {
    let predicates: Arc<[CustomConstraintPredicate]> =
        predicates.into_iter().collect();

    Arc::new(move |context| match composition {
        CustomComposition::All => {
            for predicate in predicates.iter() {
                if let Err(reason) = predicate(context) {
                    return Err(reason);
                }
            }

            Ok(())
        }

        CustomComposition::Any => {
            if predicates.is_empty() {
                return Err(
                    "custom constraint composition `Any` has no predicates"
                        .to_owned(),
                );
            }

            let mut reasons = Vec::new();

            for predicate in predicates.iter() {
                match predicate(context) {
                    Ok(()) => return Ok(()),
                    Err(reason) => reasons.push(reason),
                }
            }

            Err(reasons.join("; "))
        }
    })
}

// ============================================================================
// Phase normalization
// ============================================================================

/// Normalizes phase configuration into deterministic sorted unique storage.
fn normalize_phases(
    mut phases: Vec<ConstraintPhase>,
) -> Arc<[ConstraintPhase]> {
    phases.sort_unstable();
    phases.dedup();
    phases.into()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        allow_all_predicate,
        apply_all_predicate,
        apply_none_predicate,
        compose_predicates,
        reject_all_predicate,
        CustomComposition,
        CustomConstraint,
        CustomConstraintError,
    };

    use crate::quantum::ir::core::identity::OperationId;

    use crate::quantum::scheduling::constraints::constraint::{
        Constraint,
        ConstraintApplicability,
        ConstraintContext,
        ConstraintId,
        ConstraintKind,
        ConstraintPhase,
        ConstraintSeverity,
        ConstraintState,
        SchedulingCandidate,
    };

    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    use crate::quantum::scheduling::constraints::constraint::{
        ConstraintResourceClaim,
        ConstraintReservationView,
    };

    use crate::quantum::scheduling::types::{
        Duration,
        ReservationId,
        TimePoint,
    };

    fn test_context<'a>(
        logical: &'a [QubitId],
        physical: &'a [PhysicalQubitId],
        claims: &'a [ConstraintResourceClaim],
        reservations: &'a [ConstraintReservationView],
        completed: &'a [OperationId],
        unavailable: &'a [crate::quantum::ir::core::identity::ResourceId],
    ) -> ConstraintContext<'a> {
        let candidate = Box::leak(Box::new(SchedulingCandidate::new(
            OperationId::new(1),
            logical,
            physical,
            claims,
            TimePoint::zero(),
            Duration::zero(),
        )));

        let state = Box::leak(Box::new(ConstraintState::new(
            reservations,
            completed,
            unavailable,
        )));

        ConstraintContext::new(
            candidate,
            state,
            ConstraintPhase::Planning,
        )
    }

    #[test]
    fn construction_rejects_empty_name() {
        let result = CustomConstraint::new(
            ConstraintId::new(1),
            "   ",
            |_| Ok(()),
        );

        assert_eq!(
            result.err(),
            Some(CustomConstraintError::EmptyName)
        );
    }

    #[test]
    fn custom_constraint_accepts_success() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(1),
            "always_valid",
            |_| Ok(()),
        )
        .expect("constraint should construct");

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert!(constraint.evaluate(&context).is_ok());
        assert_eq!(constraint.kind(), ConstraintKind::Custom);
    }

    #[test]
    fn custom_constraint_reports_structured_violation() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(2),
            "always_invalid",
            |_| Err("candidate rejected".to_owned()),
        )
        .expect("constraint should construct");

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        let violation = constraint
            .evaluate(&context)
            .expect_err("constraint should fail");

        assert_eq!(violation.constraint(), ConstraintId::new(2));
        assert_eq!(violation.kind(), ConstraintKind::Custom);
        assert_eq!(
            violation.severity(),
            ConstraintSeverity::Error
        );
        assert_eq!(
            violation.operation(),
            Some(OperationId::new(1))
        );
        assert_eq!(violation.reason(), "candidate rejected");
    }

    #[test]
    fn applicability_can_skip_constraint() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(3),
            "conditional",
            |_| Err("must not execute".to_owned()),
        )
        .expect("constraint should construct")
        .with_applicability(|_| false);

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(
            constraint.applies(&context),
            ConstraintApplicability::NotApplicable
        );

        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn phase_filter_is_respected() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(4),
            "planning_only",
            |_| Err("invalid".to_owned()),
        )
        .expect("constraint should construct")
        .with_supported_phases([ConstraintPhase::Planning]);

        let planning = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert!(constraint.evaluate(&planning).is_err());
    }

    #[test]
    fn disabled_constraint_is_not_evaluated() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(5),
            "disabled",
            |_| Err("must not execute".to_owned()),
        )
        .expect("constraint should construct")
        .with_enabled(false);

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(
            constraint.applies(&context),
            ConstraintApplicability::NotApplicable
        );
        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn phase_configuration_is_deterministic() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(6),
            "phases",
            |_| Ok(()),
        )
        .expect("constraint should construct")
        .with_supported_phases([
            ConstraintPhase::Verification,
            ConstraintPhase::Planning,
            ConstraintPhase::Planning,
        ]);

        let phases = constraint
            .supported_phases()
            .expect("phases should exist");

        assert_eq!(
            phases,
            &[
                ConstraintPhase::Planning,
                ConstraintPhase::Verification,
            ]
        );
    }

    #[test]
    fn helper_predicates_work() {
        let allow = allow_all_predicate();
        let reject = reject_all_predicate("rejected");
        let apply = apply_all_predicate();
        let none = apply_none_predicate();

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert!(allow(&context).is_ok());
        assert_eq!(
            reject(&context).expect_err("must reject"),
            "rejected"
        );
        assert!(apply(&context));
        assert!(!none(&context));
    }

    #[test]
    fn all_composition_returns_first_failure() {
        let predicates = vec![
            Arc::new(|_| Ok(())) as super::CustomConstraintPredicate,
            Arc::new(|_| Err("second".to_owned())),
            Arc::new(|_| Err("third".to_owned())),
        ];

        let composed =
            compose_predicates(predicates, CustomComposition::All);

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(
            composed(&context).expect_err("must fail"),
            "second"
        );
    }

    #[test]
    fn any_composition_succeeds_when_one_predicate_succeeds() {
        let predicates = vec![
            Arc::new(|_| Err("first".to_owned()))
                as super::CustomConstraintPredicate,
            Arc::new(|_| Ok(())),
            Arc::new(|_| Err("third".to_owned())),
        ];

        let composed =
            compose_predicates(predicates, CustomComposition::Any);

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert!(composed(&context).is_ok());
    }

    #[test]
    fn any_composition_rejects_empty_predicate_set() {
        let composed =
            compose_predicates([], CustomComposition::Any);

        let context = test_context(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert!(composed(&context).is_err());
    }

    #[test]
    fn metadata_configuration_is_retained() {
        let constraint = CustomConstraint::new(
            ConstraintId::new(7),
            "target_rule",
            |_| Ok(()),
        )
        .expect("constraint should construct")
        .with_namespace("zamani.target")
        .with_version("1.0")
        .with_severity(ConstraintSeverity::Critical)
        .with_enabled(true);

        assert_eq!(
            constraint.metadata().namespace(),
            Some("zamani.target")
        );
        assert_eq!(
            constraint.metadata().version(),
            Some("1.0")
        );
        assert_eq!(
            constraint.severity(),
            ConstraintSeverity::Critical
        );
        assert!(constraint.is_enabled());
    }
}