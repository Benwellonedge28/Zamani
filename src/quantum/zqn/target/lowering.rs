//! Zamani Quantum Noise (ZQN) — Target Lowering
//!
//! # Purpose
//!
//! This module defines the target-lowering contract for ZQN.
//!
//! Lowering transforms a target-independent ZQN semantic description into a
//! target-specific realization without changing the meaning of the canonical
//! quantum program or silently discarding physical/noise semantics.
//!
//! The intended pipeline is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! ZQN semantic model
//!      │
//!      ▼
//! TargetRequirements
//!      │
//!      ▼
//! target capabilities
//!      │
//!      ▼
//! compatibility
//!      │
//!      ▼
//! ┌──────────────────────┐
//! │      LOWERING        │
//! │                      │
//! │ semantic → target    │
//! │ realization          │
//! └──────────┬───────────┘
//!            │
//!            ▼
//! target-specific execution representation
//!            │
//!            ▼
//! runtime / simulator / hardware adapter
//! ```
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - the generic ZQN target-lowering contract;
//! - lowering requests;
//! - lowering policies;
//! - lowering modes;
//! - lowering outcomes;
//! - lowering diagnostics;
//! - explicit approximation/loss contracts;
//! - lowering provenance;
//! - deterministic lowering metadata;
//! - target-independent lowering orchestration;
//! - the interface implemented by target-specific lowerers.
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - quantum source-language parsing;
//! - quantum channels;
//! - noise models;
//! - faults;
//! - target discovery;
//! - hardware inventory;
//! - vendor APIs;
//! - credentials;
//! - QPU transport;
//! - routing;
//! - scheduling;
//! - QEC;
//! - simulation algorithms;
//! - benchmarking methodology;
//! - resource allocation;
//! - target-specific instruction definitions.
//!
//! Those concerns remain owned by their respective subsystems.
//!
//! # Critical architectural rule
//!
//! Lowering is NOT compilation into a second quantum IR.
//!
//! ZQN lowering is a semantic realization step:
//!
//! ```text
//! abstract ZQN semantics
//!          │
//!          ▼
//! compatibility contract
//!          │
//!          ▼
//! target realization
//! ```
//!
//! `quantum::ir` remains the canonical quantum semantic representation.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! - `MAX_QUBITS`;
//! - `MAX_PHYSICAL_QUBITS`;
//! - `MAX_OPERATIONS`;
//! - fixed gate arities;
//! - fixed topology;
//! - vendor identifiers;
//! - simulator-specific limits;
//! - hardware-specific qubit counts;
//! - machine-size branches;
//! - compile-time target capacity assumptions.
//!
//! A lowering implementation may impose resource limits through an explicit
//! resource policy supplied by the caller, but those limits are not semantic
//! properties of ZQN.
//!
//! Therefore the same source-level program and semantic ZQN model can be
//! lowered for small, large, distributed, simulated, fault-tolerant, analog,
//! bosonic, continuous-variable, photonic, fermionic, or future quantum
//! targets without rewriting the program.
//!
//! # Canonical quantum identities
//!
//! This file does not define `QubitId` or `PhysicalQubitId`.
//!
//! When a lowering implementation needs resource identity, it must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This follows the repository-wide canonical identity boundary. The IR
//! explicitly requires new code to use those canonical types rather than
//! introducing a second identity system. 
//!
//! # Approximation rule
//!
//! Lowering MUST NOT silently change semantics.
//!
//! A lowerer has only four legitimate outcomes:
//!
//! 1. exact realization;
//! 2. explicitly declared approximation;
//! 3. explicitly declared bounded/statistical realization;
//! 4. rejection.
//!
//! For an approximation, the result must preserve:
//!
//! - requested semantics;
//! - realized semantics;
//! - approximation mode;
//! - declared error information;
//! - assumptions;
//! - provenance.
//!
//! A lowerer must never silently convert:
//!
//! ```text
//! Unsupported → Supported
//! Approximate → Exact
//! Unknown → Exact
//! ```
//!
//! # Capability negotiation
//!
//! Capability matching belongs to the target capability/compatibility layers.
//!
//! This file consumes the resulting lowering authorization through the
//! `LoweringTarget` trait.
//!
//! That separation prevents this module from depending on a concrete hardware
//! implementation.
//!
//! # Determinism
//!
//! Lowering is deterministic for a fixed:
//!
//! - semantic input;
//! - target;
//! - target requirements;
//! - lowering policy;
//! - capability decision;
//! - lowering configuration.
//!
//! This module does not own random state and does not access:
//!
//! - global RNGs;
//! - clocks;
//! - environment variables;
//! - process IDs;
//! - memory addresses;
//! - global mutable state.
//!
//! A target lowerer requiring stochastic behavior must receive an explicit
//! deterministic execution context from the runtime/simulation subsystem.
//!
//! # Security
//!
//! Lowering input is untrusted data at architectural boundaries.
//!
//! Implementations must guard against:
//!
//! - pathological target descriptions;
//! - uncontrolled allocation;
//! - non-finite numerical values;
//! - recursive target representations;
//! - non-terminating lowering;
//! - malicious capability data;
//! - resource-exhaustion attacks.
//!
//! Resource limits belong to explicit caller-supplied policy rather than
//! hidden constants in this module.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! Producers:
//!
//! - ZQN semantic analysis;
//! - `target/requirements.rs`;
//! - `target/compatibility.rs`;
//! - IR analysis;
//! - calibration-aware compilation;
//! - simulation planning;
//! - hardware planning.
//!
//! Consumers:
//!
//! - target adapters;
//! - hardware integration;
//! - simulator integration;
//! - runtime integration;
//! - future execution backends.
//!
//! Dependency direction:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! ZQN semantics
//!      │
//!      ▼
//! TargetRequirements
//!      │
//!      ▼
//! TargetCapabilities / compatibility
//!      │
//!      ▼
//! this lowering contract
//!      │
//!      ▼
//! target adapter
//!      │
//!      ▼
//! runtime / simulator / hardware
//! ```
//!
//! This file does not import or depend on concrete hardware providers.
//!
//! # Future-file stability
//!
//! This module intentionally does not require a concrete
//! `TargetCapabilities` implementation.
//!
//! The capability layer can evolve independently as long as it implements
//! `LoweringTarget`.
//!
//! Likewise, target-specific representations remain outside this file.
//!
//! This allows the file to be completed and stabilized before concrete
//! simulator and hardware adapters are implemented.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::fmt;
use std::marker::PhantomData;

use crate::quantum::zqn::target::requirements::TargetRequirements;

// =============================================================================
// Lowering mode
// =============================================================================

/// The semantic strictness permitted during target lowering.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LoweringMode {
    /// Only an exact realization is accepted.
    Exact,

    /// An explicitly declared approximation may be selected.
    Approximate,

    /// A formally declared error bound may be selected.
    Bounded,

    /// A statistically characterized realization may be selected.
    Statistical,
}

impl Default for LoweringMode {
    fn default() -> Self {
        Self::Exact
    }
}

impl LoweringMode {
    /// Returns whether only exact realization is permitted.
    #[must_use]
    pub const fn requires_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether approximation may be selected.
    #[must_use]
    pub const fn allows_approximation(self) -> bool {
        matches!(self, Self::Approximate)
    }

    /// Returns whether bounded realization may be selected.
    #[must_use]
    pub const fn allows_bounded(self) -> bool {
        matches!(self, Self::Bounded)
    }

    /// Returns whether statistical realization may be selected.
    #[must_use]
    pub const fn allows_statistical(self) -> bool {
        matches!(self, Self::Statistical)
    }
}

// =============================================================================
// Approximation contract
// =============================================================================

/// Explicit information describing a non-exact lowering.
#[derive(Clone, Debug, PartialEq)]
pub struct ApproximationContract {
    /// Human-readable identifier of the approximation strategy.
    ///
    /// This is descriptive metadata and must not be used as authorization.
    strategy: String,

    /// Optional absolute error bound.
    absolute_error: Option<f64>,

    /// Optional relative error bound.
    relative_error: Option<f64>,

    /// Optional statistical confidence associated with the declared bound.
    confidence: Option<f64>,

    /// Assumptions under which the approximation contract is valid.
    assumptions: Vec<String>,
}

impl ApproximationContract {
    /// Creates an approximation contract.
    ///
    /// At least one quantitative guarantee must be supplied.
    pub fn new<S>(
        strategy: S,
        absolute_error: Option<f64>,
        relative_error: Option<f64>,
        confidence: Option<f64>,
        assumptions: Vec<String>,
    ) -> Result<Self, LoweringError>
    where
        S: Into<String>,
    {
        let strategy = strategy.into();

        validate_non_empty_text(&strategy, "approximation strategy")?;

        validate_optional_non_negative(
            absolute_error,
            "absolute approximation error",
        )?;

        validate_optional_non_negative(
            relative_error,
            "relative approximation error",
        )?;

        validate_optional_probability(confidence, "approximation confidence")?;

        if absolute_error.is_none()
            && relative_error.is_none()
            && confidence.is_none()
        {
            return Err(LoweringError::MissingApproximationGuarantee);
        }

        validate_text_collection(&assumptions, "approximation assumption")?;

        Ok(Self {
            strategy,
            absolute_error,
            relative_error,
            confidence,
            assumptions,
        })
    }

    /// Returns the approximation strategy.
    #[must_use]
    pub fn strategy(&self) -> &str {
        &self.strategy
    }

    /// Returns the absolute error bound.
    #[must_use]
    pub const fn absolute_error(&self) -> Option<f64> {
        self.absolute_error
    }

    /// Returns the relative error bound.
    #[must_use]
    pub const fn relative_error(&self) -> Option<f64> {
        self.relative_error
    }

    /// Returns the statistical confidence.
    #[must_use]
    pub const fn confidence(&self) -> Option<f64> {
        self.confidence
    }

    /// Returns the assumptions.
    #[must_use]
    pub fn assumptions(&self) -> &[String] {
        &self.assumptions
    }
}

// =============================================================================
// Lowering policy
// =============================================================================

/// Policy controlling what a lowerer may do.
#[derive(Clone, Debug, PartialEq)]
pub struct LoweringPolicy {
    /// Maximum semantic lowering mode permitted.
    mode: LoweringMode,

    /// Whether information loss must cause lowering to fail.
    reject_information_loss: bool,

    /// Whether an explicitly approximate target realization is permitted.
    allow_approximation: bool,

    /// Whether bounded target realizations are permitted.
    allow_bounded: bool,

    /// Whether statistical target realizations are permitted.
    allow_statistical: bool,

    /// Whether the lowerer must preserve provenance information.
    preserve_provenance: bool,
}

impl Default for LoweringPolicy {
    fn default() -> Self {
        Self {
            mode: LoweringMode::Exact,
            reject_information_loss: true,
            allow_approximation: false,
            allow_bounded: false,
            allow_statistical: false,
            preserve_provenance: true,
        }
    }
}

impl LoweringPolicy {
    /// Creates the strict production default.
    ///
    /// Exact lowering is required and semantic information loss is rejected.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            mode: LoweringMode::Exact,
            reject_information_loss: true,
            allow_approximation: false,
            allow_bounded: false,
            allow_statistical: false,
            preserve_provenance: true,
        }
    }

    /// Creates a policy allowing explicit approximation.
    #[must_use]
    pub const fn approximate() -> Self {
        Self {
            mode: LoweringMode::Approximate,
            reject_information_loss: true,
            allow_approximation: true,
            allow_bounded: false,
            allow_statistical: false,
            preserve_provenance: true,
        }
    }

    /// Creates a policy allowing explicit bounded realizations.
    #[must_use]
    pub const fn bounded() -> Self {
        Self {
            mode: LoweringMode::Bounded,
            reject_information_loss: true,
            allow_approximation: false,
            allow_bounded: true,
            allow_statistical: false,
            preserve_provenance: true,
        }
    }

    /// Creates a policy allowing statistical realizations.
    #[must_use]
    pub const fn statistical() -> Self {
        Self {
            mode: LoweringMode::Statistical,
            reject_information_loss: true,
            allow_approximation: false,
            allow_bounded: false,
            allow_statistical: true,
            preserve_provenance: true,
        }
    }

    /// Returns the selected lowering mode.
    #[must_use]
    pub const fn mode(&self) -> LoweringMode {
        self.mode
    }

    /// Returns whether information loss is rejected.
    #[must_use]
    pub const fn rejects_information_loss(&self) -> bool {
        self.reject_information_loss
    }

    /// Returns whether approximation is allowed.
    #[must_use]
    pub const fn allows_approximation(&self) -> bool {
        self.allow_approximation
    }

    /// Returns whether bounded realization is allowed.
    #[must_use]
    pub const fn allows_bounded(&self) -> bool {
        self.allow_bounded
    }

    /// Returns whether statistical realization is allowed.
    #[must_use]
    pub const fn allows_statistical(&self) -> bool {
        self.allow_statistical
    }

    /// Returns whether provenance must be preserved.
    #[must_use]
    pub const fn preserves_provenance(&self) -> bool {
        self.preserve_provenance
    }

    /// Enables or disables rejection of semantic information loss.
    #[must_use]
    pub const fn with_reject_information_loss(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_information_loss = reject;
        self
    }

    /// Enables or disables explicit approximation.
    #[must_use]
    pub const fn with_approximation(mut self, enabled: bool) -> Self {
        self.allow_approximation = enabled;
        self
    }

    /// Enables or disables bounded realization.
    #[must_use]
    pub const fn with_bounded(mut self, enabled: bool) -> Self {
        self.allow_bounded = enabled;
        self
    }

    /// Enables or disables statistical realization.
    #[must_use]
    pub const fn with_statistical(mut self, enabled: bool) -> Self {
        self.allow_statistical = enabled;
        self
    }

    /// Enables or disables provenance preservation.
    #[must_use]
    pub const fn with_provenance(mut self, enabled: bool) -> Self {
        self.preserve_provenance = enabled;
        self
    }

    /// Validates the policy.
    pub fn validate(&self) -> Result<(), LoweringError> {
        match self.mode {
            LoweringMode::Exact => {
                if self.allow_approximation
                    || self.allow_bounded
                    || self.allow_statistical
                {
                    return Err(LoweringError::InvalidPolicy(
                        "exact mode cannot enable non-exact realization"
                            .to_owned(),
                    ));
                }
            }

            LoweringMode::Approximate => {
                if !self.allow_approximation {
                    return Err(LoweringError::InvalidPolicy(
                        "approximate mode requires approximation support"
                            .to_owned(),
                    ));
                }
            }

            LoweringMode::Bounded => {
                if !self.allow_bounded {
                    return Err(LoweringError::InvalidPolicy(
                        "bounded mode requires bounded realization support"
                            .to_owned(),
                    ));
                }
            }

            LoweringMode::Statistical => {
                if !self.allow_statistical {
                    return Err(LoweringError::InvalidPolicy(
                        "statistical mode requires statistical realization support"
                            .to_owned(),
                    ));
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Lowering request
// =============================================================================

/// A target-lowering request.
///
/// `S` is intentionally generic because this module must not define another
/// quantum IR or depend on a concrete target representation.
#[derive(Clone, Debug)]
pub struct LoweringRequest<S> {
    /// Semantic source being lowered.
    source: S,

    /// Target-independent requirements.
    requirements: TargetRequirements,

    /// Lowering policy.
    policy: LoweringPolicy,
}

impl<S> LoweringRequest<S> {
    /// Creates a lowering request.
    pub fn new(
        source: S,
        requirements: TargetRequirements,
        policy: LoweringPolicy,
    ) -> Result<Self, LoweringError> {
        policy.validate()?;

        requirements
            .validate()
            .map_err(|error| LoweringError::InvalidRequirements(error.to_string()))?;

        Ok(Self {
            source,
            requirements,
            policy,
        })
    }

    /// Returns the semantic source.
    #[must_use]
    pub fn source(&self) -> &S {
        &self.source
    }

    /// Consumes the request and returns its source.
    #[must_use]
    pub fn into_source(self) -> S {
        self.source
    }

    /// Returns the target-independent requirements.
    #[must_use]
    pub fn requirements(&self) -> &TargetRequirements {
        &self.requirements
    }

    /// Returns the lowering policy.
    #[must_use]
    pub fn policy(&self) -> &LoweringPolicy {
        &self.policy
    }

    /// Returns the semantic source and requirements without cloning.
    #[must_use]
    pub fn into_parts(self) -> (S, TargetRequirements, LoweringPolicy) {
        (self.source, self.requirements, self.policy)
    }
}

// =============================================================================
// Lowering realization
// =============================================================================

/// Classification of a successful lowering result.
#[derive(Clone, Debug, PartialEq)]
pub enum LoweringRealization {
    /// The requested semantics were preserved exactly.
    Exact,

    /// The requested semantics were approximated under an explicit contract.
    Approximate(ApproximationContract),

    /// The requested semantics were realized with an explicit formal bound.
    Bounded(ApproximationContract),

    /// The requested semantics were realized statistically.
    Statistical(ApproximationContract),
}

impl LoweringRealization {
    /// Returns whether the realization is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the realization is approximate.
    #[must_use]
    pub const fn is_approximate(&self) -> bool {
        matches!(self, Self::Approximate(_))
    }

    /// Returns whether the realization is bounded.
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        matches!(self, Self::Bounded(_))
    }

    /// Returns whether the realization is statistical.
    #[must_use]
    pub const fn is_statistical(&self) -> bool {
        matches!(self, Self::Statistical(_))
    }

    /// Returns the approximation contract, when one exists.
    #[must_use]
    pub const fn contract(&self) -> Option<&ApproximationContract> {
        match self {
            Self::Exact => None,
            Self::Approximate(contract)
            | Self::Bounded(contract)
            | Self::Statistical(contract) => Some(contract),
        }
    }
}

// =============================================================================
// Lowering provenance
// =============================================================================

/// Immutable provenance attached to a lowering result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoweringProvenance {
    /// Stable semantic-layer identifier.
    source_kind: String,

    /// Stable target-layer identifier.
    target_kind: String,

    /// Lowering implementation identifier.
    lowerer: String,

    /// Optional lowerer version.
    lowerer_version: Option<String>,
}

impl LoweringProvenance {
    /// Creates lowering provenance.
    pub fn new<S1, S2, S3>(
        source_kind: S1,
        target_kind: S2,
        lowerer: S3,
        lowerer_version: Option<String>,
    ) -> Result<Self, LoweringError>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let source_kind = source_kind.into();
        let target_kind = target_kind.into();
        let lowerer = lowerer.into();

        validate_non_empty_text(&source_kind, "source kind")?;
        validate_non_empty_text(&target_kind, "target kind")?;
        validate_non_empty_text(&lowerer, "lowerer")?;

        if let Some(version) = &lowerer_version {
            validate_non_empty_text(version, "lowerer version")?;
        }

        Ok(Self {
            source_kind,
            target_kind,
            lowerer,
            lowerer_version,
        })
    }

    /// Returns the source-kind identifier.
    #[must_use]
    pub fn source_kind(&self) -> &str {
        &self.source_kind
    }

    /// Returns the target-kind identifier.
    #[must_use]
    pub fn target_kind(&self) -> &str {
        &self.target_kind
    }

    /// Returns the lowerer identifier.
    #[must_use]
    pub fn lowerer(&self) -> &str {
        &self.lowerer
    }

    /// Returns the lowerer version.
    #[must_use]
    pub fn lowerer_version(&self) -> Option<&str> {
        self.lowerer_version.as_deref()
    }
}

// =============================================================================
// Lowering result
// =============================================================================

/// Successful target-lowering result.
///
/// `T` is the target-specific representation owned by the target adapter.
#[derive(Clone, Debug)]
pub struct LoweringResult<T> {
    /// Target-specific realization.
    realization: T,

    /// Semantic classification of the lowering.
    classification: LoweringRealization,

    /// Provenance of the transformation.
    provenance: Option<LoweringProvenance>,
}

impl<T> LoweringResult<T> {
    /// Creates an exact lowering result.
    #[must_use]
    pub fn exact(
        realization: T,
        provenance: Option<LoweringProvenance>,
    ) -> Self {
        Self {
            realization,
            classification: LoweringRealization::Exact,
            provenance,
        }
    }

    /// Creates an approximate lowering result.
    pub fn approximate(
        realization: T,
        contract: ApproximationContract,
        provenance: Option<LoweringProvenance>,
    ) -> Self {
        Self {
            realization,
            classification: LoweringRealization::Approximate(contract),
            provenance,
        }
    }

    /// Creates a bounded lowering result.
    pub fn bounded(
        realization: T,
        contract: ApproximationContract,
        provenance: Option<LoweringProvenance>,
    ) -> Self {
        Self {
            realization,
            classification: LoweringRealization::Bounded(contract),
            provenance,
        }
    }

    /// Creates a statistical lowering result.
    pub fn statistical(
        realization: T,
        contract: ApproximationContract,
        provenance: Option<LoweringProvenance>,
    ) -> Self {
        Self {
            realization,
            classification: LoweringRealization::Statistical(contract),
            provenance,
        }
    }

    /// Returns the target-specific realization.
    #[must_use]
    pub fn realization(&self) -> &T {
        &self.realization
    }

    /// Returns mutable access to the target realization.
    ///
    /// Mutation is deliberately limited to the result owned by the caller.
    /// This module does not provide global mutable target state.
    #[must_use]
    pub fn realization_mut(&mut self) -> &mut T {
        &mut self.realization
    }

    /// Consumes the result and returns the target realization.
    #[must_use]
    pub fn into_realization(self) -> T {
        self.realization
    }

    /// Returns the realization classification.
    #[must_use]
    pub fn classification(&self) -> &LoweringRealization {
        &self.classification
    }

    /// Returns whether lowering is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.classification.is_exact()
    }

    /// Returns whether lowering is non-exact.
    #[must_use]
    pub const fn is_non_exact(&self) -> bool {
        !self.classification.is_exact()
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> Option<&LoweringProvenance> {
        self.provenance.as_ref()
    }

    /// Returns the result components without cloning.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (T, LoweringRealization, Option<LoweringProvenance>) {
        (self.realization, self.classification, self.provenance)
    }
}

// =============================================================================
// Lowering diagnostics
// =============================================================================

/// Severity of a lowering diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LoweringDiagnosticSeverity {
    /// Informational diagnostic.
    Info,

    /// Warning that does not necessarily prevent execution.
    Warning,

    /// Error that prevents a valid lowering.
    Error,
}

/// A structured lowering diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweringDiagnostic {
    severity: LoweringDiagnosticSeverity,
    code: String,
    message: String,
}

impl LoweringDiagnostic {
    /// Creates a diagnostic.
    pub fn new<C, M>(
        severity: LoweringDiagnosticSeverity,
        code: C,
        message: M,
    ) -> Result<Self, LoweringError>
    where
        C: Into<String>,
        M: Into<String>,
    {
        let code = code.into();
        let message = message.into();

        validate_non_empty_text(&code, "diagnostic code")?;
        validate_non_empty_text(&message, "diagnostic message")?;

        Ok(Self {
            severity,
            code,
            message,
        })
    }

    /// Returns diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> LoweringDiagnosticSeverity {
        self.severity
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns whether this diagnostic is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(
            self.severity,
            LoweringDiagnosticSeverity::Error
        )
    }
}

// =============================================================================
// Lowering plan
// =============================================================================

/// A target-independent lowering plan.
///
/// The plan deliberately contains no target-specific instruction objects.
/// It records the decisions that the target lowerer is authorized to realize.
#[derive(Clone, Debug)]
pub struct LoweringPlan {
    mode: LoweringMode,
    preserve_provenance: bool,
    diagnostics: Vec<LoweringDiagnostic>,
}

impl LoweringPlan {
    /// Creates an empty lowering plan.
    #[must_use]
    pub fn new(policy: &LoweringPolicy) -> Self {
        Self {
            mode: policy.mode(),
            preserve_provenance: policy.preserves_provenance(),
            diagnostics: Vec::new(),
        }
    }

    /// Returns the permitted lowering mode.
    #[must_use]
    pub const fn mode(&self) -> LoweringMode {
        self.mode
    }

    /// Returns whether provenance must be preserved.
    #[must_use]
    pub const fn preserves_provenance(&self) -> bool {
        self.preserve_provenance
    }

    /// Adds a diagnostic.
    pub fn push_diagnostic(
        &mut self,
        diagnostic: LoweringDiagnostic,
    ) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[LoweringDiagnostic] {
        &self.diagnostics
    }

    /// Returns whether the plan contains an error diagnostic.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(LoweringDiagnostic::is_error)
    }

    /// Validates the plan before realization.
    pub fn validate(&self) -> Result<(), LoweringError> {
        if self.has_errors() {
            return Err(LoweringError::PlanRejected);
        }

        Ok(())
    }
}

// =============================================================================
// Target-lowering abstraction
// =============================================================================

/// Target-specific lowering boundary.
///
/// A target adapter implements this trait to transform ZQN semantic input into
/// its own execution representation.
///
/// The target representation `T` is associated with the implementation and is
/// therefore never standardized by ZQN.
///
/// # Design requirements
///
/// Implementations MUST:
///
/// - validate the request before producing target output;
/// - honor the supplied lowering policy;
/// - never silently discard semantics;
/// - never silently approximate;
/// - never access credentials through this interface;
/// - never perform target discovery implicitly;
/// - never introduce fixed machine-size assumptions;
/// - return deterministic output for deterministic input;
/// - return structured errors on failure.
///
/// Implementations SHOULD:
///
/// - remain independent of source-language syntax;
/// - remain independent of frontend ASTs;
/// - avoid embedding routing or scheduling;
/// - preserve provenance;
/// - use canonical IR resource identities when resource-scoped behavior is
///   required.
pub trait TargetLowering {
    /// Semantic input accepted by this lowerer.
    type Source;

    /// Target-specific representation produced by this lowerer.
    type Target;

    /// Returns a stable source-kind identifier.
    fn source_kind(&self) -> &str;

    /// Returns a stable target-kind identifier.
    fn target_kind(&self) -> &str;

    /// Returns a stable lowerer identifier.
    fn lowerer_id(&self) -> &str;

    /// Returns the lowerer implementation version, when available.
    fn lowerer_version(&self) -> Option<&str> {
        None
    }

    /// Checks whether the lowerer can attempt the supplied request.
    ///
    /// This is deliberately separate from actual lowering so incompatible
    /// targets can be rejected before expensive target representation is
    /// constructed.
    fn prepare(
        &self,
        request: &LoweringRequest<Self::Source>,
    ) -> Result<LoweringPlan, LoweringError>;

    /// Performs target realization after preparation succeeds.
    fn realize(
        &self,
        request: &LoweringRequest<Self::Source>,
        plan: &LoweringPlan,
    ) -> Result<LoweringResult<Self::Target>, LoweringError>;

    /// Performs complete lowering.
    ///
    /// The default implementation enforces the required ordering:
    ///
    /// ```text
    /// request
    ///   ↓
    /// prepare
    ///   ↓
    /// validate plan
    ///   ↓
    /// realize
    ///   ↓
    /// validate result
    /// ```
    fn lower(
        &self,
        request: &LoweringRequest<Self::Source>,
    ) -> Result<LoweringResult<Self::Target>, LoweringError> {
        let plan = self.prepare(request)?;

        plan.validate()?;

        let result = self.realize(request, &plan)?;

        validate_result_against_policy(&result, request.policy())?;

        if request.policy().preserves_provenance()
            && result.provenance().is_none()
        {
            return Err(LoweringError::MissingProvenance);
        }

        Ok(result)
    }
}

// =============================================================================
// Capability-to-lowering adapter
// =============================================================================

/// A target capability boundary used by lowering implementations.
///
/// This small trait prevents `lowering.rs` from depending on a concrete
/// `TargetCapabilities` representation.
///
/// `target/capabilities.rs` should implement this trait for its public target
/// capability object.
///
/// The compatibility subsystem remains responsible for deciding whether a
/// requested capability is acceptable; this trait exposes only the final
/// authorization required by a lowerer.
pub trait LoweringTarget {
    /// Returns a stable target identity.
    fn target_kind(&self) -> &str;

    /// Validates that the target can realize the supplied requirements under
    /// the selected lowering policy.
    fn authorize_lowering<S>(
        &self,
        request: &LoweringRequest<S>,
    ) -> Result<(), LoweringError>;
}

// =============================================================================
// Generic target-authorized lowerer
// =============================================================================

/// Wrapper that combines a target capability object and a concrete lowerer.
///
/// This keeps capability authorization and realization separate.
pub struct AuthorizedLowerer<'a, C, L> {
    capabilities: &'a C,
    lowerer: &'a L,
}

impl<'a, C, L> AuthorizedLowerer<'a, C, L> {
    /// Creates an authorized lowering facade.
    #[must_use]
    pub fn new(capabilities: &'a C, lowerer: &'a L) -> Self {
        Self {
            capabilities,
            lowerer,
        }
    }
}

impl<'a, C, L> AuthorizedLowerer<'a, C, L>
where
    C: LoweringTarget,
    L: TargetLowering,
{
    /// Returns the target capability object.
    #[must_use]
    pub fn capabilities(&self) -> &C {
        self.capabilities
    }

    /// Returns the target lowerer.
    #[must_use]
    pub fn lowerer(&self) -> &L {
        self.lowerer
    }

    /// Performs capability-authorized lowering.
    pub fn lower(
        &self,
        request: &LoweringRequest<L::Source>,
    ) -> Result<LoweringResult<L::Target>, LoweringError> {
        self.capabilities.authorize_lowering(request)?;

        if self.lowerer.target_kind() != self.capabilities.target_kind() {
            return Err(LoweringError::TargetMismatch {
                expected: self.capabilities.target_kind().to_owned(),
                actual: self.lowerer.target_kind().to_owned(),
            });
        }

        self.lowerer.lower(request)
    }
}

// =============================================================================
// Type-erased planning marker
// =============================================================================

/// Marker type used where an integration layer needs to carry a source type
/// without constructing a target-specific representation.
///
/// This is intentionally zero-sized.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PlanningMarker<S>(PhantomData<fn() -> S>);

impl<S> PlanningMarker<S> {
    /// Creates a planning marker.
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

// =============================================================================
// Lowering errors
// =============================================================================

/// Errors produced by the target-lowering layer.
#[derive(Clone, Debug, PartialEq)]
pub enum LoweringError {
    /// The target does not satisfy the required semantics.
    IncompatibleTarget,

    /// The supplied requirements are invalid.
    InvalidRequirements(String),

    /// The lowering policy is invalid.
    InvalidPolicy(String),

    /// The lowerer does not implement the required semantics.
    UnsupportedSemantics(String),

    /// A target-specific representation cannot be constructed safely.
    RepresentationFailure(String),

    /// Lowering would discard information required by the request.
    InformationLoss(String),

    /// An approximation was required but no explicit guarantee was supplied.
    MissingApproximationGuarantee,

    /// The selected realization violates the caller's lowering policy.
    PolicyViolation(String),

    /// The lowerer did not provide required provenance.
    MissingProvenance,

    /// The preparation plan was rejected.
    PlanRejected,

    /// The selected lowerer belongs to a different target.
    TargetMismatch {
        /// Expected target kind.
        expected: String,

        /// Actual lowerer target kind.
        actual: String,
    },

    /// A required numerical value was invalid.
    NonFiniteValue {
        /// Name of the invalid value.
        field: String,

        /// Invalid numerical value.
        value: f64,
    },

    /// A numerical value was outside its valid domain.
    InvalidNumericRange {
        /// Name of the invalid value.
        field: String,

        /// Invalid numerical value.
        value: f64,
    },

    /// Textual metadata was invalid.
    InvalidText {
        /// Name of the invalid field.
        field: String,
    },
}

impl fmt::Display for LoweringError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IncompatibleTarget => {
                formatter.write_str("target is incompatible with lowering requirements")
            }

            Self::InvalidRequirements(message) => {
                write!(formatter, "invalid target requirements: {message}")
            }

            Self::InvalidPolicy(message) => {
                write!(formatter, "invalid lowering policy: {message}")
            }

            Self::UnsupportedSemantics(message) => {
                write!(formatter, "unsupported lowering semantics: {message}")
            }

            Self::RepresentationFailure(message) => {
                write!(formatter, "target representation failure: {message}")
            }

            Self::InformationLoss(message) => {
                write!(formatter, "lowering would lose required information: {message}")
            }

            Self::MissingApproximationGuarantee => {
                formatter.write_str(
                    "non-exact lowering requires an explicit approximation guarantee",
                )
            }

            Self::PolicyViolation(message) => {
                write!(formatter, "lowering policy violation: {message}")
            }

            Self::MissingProvenance => {
                formatter.write_str(
                    "lowering policy requires provenance but the lowerer supplied none",
                )
            }

            Self::PlanRejected => {
                formatter.write_str("lowering plan was rejected")
            }

            Self::TargetMismatch { expected, actual } => {
                write!(
                    formatter,
                    "target mismatch: expected `{expected}`, got `{actual}`"
                )
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "non-finite lowering value for `{field}`: {value}"
                )
            }

            Self::InvalidNumericRange { field, value } => {
                write!(
                    formatter,
                    "invalid numerical range for `{field}`: {value}"
                )
            }

            Self::InvalidText { field } => {
                write!(formatter, "invalid textual value for `{field}`")
            }
        }
    }
}

impl std::error::Error for LoweringError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_non_empty_text(
    value: &str,
    field: &str,
) -> Result<(), LoweringError> {
    if value.trim().is_empty() {
        return Err(LoweringError::InvalidText {
            field: field.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(LoweringError::InvalidText {
            field: field.to_owned(),
        });
    }

    Ok(())
}

fn validate_text_collection(
    values: &[String],
    field: &str,
) -> Result<(), LoweringError> {
    for value in values {
        validate_non_empty_text(value, field)?;
    }

    Ok(())
}

fn validate_optional_non_negative(
    value: Option<f64>,
    field: &str,
) -> Result<(), LoweringError> {
    if let Some(value) = value {
        if !value.is_finite() {
            return Err(LoweringError::NonFiniteValue {
                field: field.to_owned(),
                value,
            });
        }

        if value < 0.0 {
            return Err(LoweringError::InvalidNumericRange {
                field: field.to_owned(),
                value,
            });
        }
    }

    Ok(())
}

fn validate_optional_probability(
    value: Option<f64>,
    field: &str,
) -> Result<(), LoweringError> {
    if let Some(value) = value {
        if !value.is_finite() {
            return Err(LoweringError::NonFiniteValue {
                field: field.to_owned(),
                value,
            });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(LoweringError::InvalidNumericRange {
                field: field.to_owned(),
                value,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Result/policy validation
// =============================================================================

fn validate_result_against_policy<T>(
    result: &LoweringResult<T>,
    policy: &LoweringPolicy,
) -> Result<(), LoweringError> {
    match result.classification() {
        LoweringRealization::Exact => Ok(()),

        LoweringRealization::Approximate(_) => {
            if policy.mode() == LoweringMode::Exact
                || !policy.allows_approximation()
            {
                return Err(LoweringError::PolicyViolation(
                    "approximate realization is not permitted".to_owned(),
                ));
            }

            Ok(())
        }

        LoweringRealization::Bounded(_) => {
            if policy.mode() == LoweringMode::Exact
                || !policy.allows_bounded()
            {
                return Err(LoweringError::PolicyViolation(
                    "bounded realization is not permitted".to_owned(),
                ));
            }

            Ok(())
        }

        LoweringRealization::Statistical(_) => {
            if policy.mode() == LoweringMode::Exact
                || !policy.allows_statistical()
            {
                return Err(LoweringError::PolicyViolation(
                    "statistical realization is not permitted".to_owned(),
                ));
            }

            Ok(())
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_policy_is_strict() {
        let policy = LoweringPolicy::exact();

        assert_eq!(policy.mode(), LoweringMode::Exact);
        assert!(policy.rejects_information_loss());
        assert!(!policy.allows_approximation());
        assert!(!policy.allows_bounded());
        assert!(!policy.allows_statistical());
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn approximate_policy_is_valid() {
        let policy = LoweringPolicy::approximate();

        assert_eq!(policy.mode(), LoweringMode::Approximate);
        assert!(policy.allows_approximation());
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn invalid_exact_policy_is_rejected() {
        let policy = LoweringPolicy::exact().with_approximation(true);

        assert!(policy.validate().is_err());
    }

    #[test]
    fn approximation_requires_guarantee() {
        let result = ApproximationContract::new(
            "test",
            None,
            None,
            None,
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(LoweringError::MissingApproximationGuarantee)
        ));
    }

    #[test]
    fn approximation_rejects_nan() {
        let result = ApproximationContract::new(
            "test",
            Some(f64::NAN),
            None,
            None,
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(LoweringError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn approximation_rejects_negative_error() {
        let result = ApproximationContract::new(
            "test",
            Some(-1.0),
            None,
            None,
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(LoweringError::InvalidNumericRange { .. })
        ));
    }

    #[test]
    fn confidence_must_be_a_probability() {
        let result = ApproximationContract::new(
            "test",
            None,
            None,
            Some(1.1),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(LoweringError::InvalidNumericRange { .. })
        ));
    }

    #[test]
    fn provenance_rejects_empty_values() {
        let result = LoweringProvenance::new(
            "",
            "target",
            "lowerer",
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn result_classification_is_exact() {
        let result = LoweringResult::exact(
            42_u32,
            Some(
                LoweringProvenance::new(
                    "zqn",
                    "test",
                    "test-lowerer",
                    Some("1".to_owned()),
                )
                .expect("valid provenance"),
            ),
        );

        assert!(result.is_exact());
        assert!(!result.is_non_exact());
        assert!(result.classification().contract().is_none());
    }

    #[test]
    fn approximate_result_requires_approximate_policy() {
        let contract = ApproximationContract::new(
            "test",
            Some(0.01),
            None,
            None,
            Vec::new(),
        )
        .expect("valid contract");

        let provenance = LoweringProvenance::new(
            "zqn",
            "test",
            "test-lowerer",
            None,
        )
        .expect("valid provenance");

        let result = LoweringResult::approximate(
            42_u32,
            contract,
            Some(provenance),
        );

        assert!(
            validate_result_against_policy(
                &result,
                &LoweringPolicy::exact(),
            )
            .is_err()
        );

        assert!(
            validate_result_against_policy(
                &result,
                &LoweringPolicy::approximate(),
            )
            .is_ok()
        );
    }

    #[test]
    fn plan_rejects_error_diagnostics() {
        let mut plan =
            LoweringPlan::new(&LoweringPolicy::exact());

        let diagnostic = LoweringDiagnostic::new(
            LoweringDiagnosticSeverity::Error,
            "ZQN-TARGET-001",
            "target cannot realize required semantics",
        )
        .expect("valid diagnostic");

        plan.push_diagnostic(diagnostic);

        assert!(plan.has_errors());
        assert!(plan.validate().is_err());
    }

    #[test]
    fn plan_accepts_information_diagnostic() {
        let mut plan =
            LoweringPlan::new(&LoweringPolicy::exact());

        let diagnostic = LoweringDiagnostic::new(
            LoweringDiagnosticSeverity::Info,
            "ZQN-TARGET-INFO",
            "exact realization selected",
        )
        .expect("valid diagnostic");

        plan.push_diagnostic(diagnostic);

        assert!(!plan.has_errors());
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn lowering_mode_is_explicit() {
        assert!(LoweringMode::Exact.requires_exact());
        assert!(!LoweringMode::Approximate.requires_exact());

        assert!(LoweringMode::Approximate.allows_approximation());
        assert!(LoweringMode::Bounded.allows_bounded());
        assert!(LoweringMode::Statistical.allows_statistical());
    }
}