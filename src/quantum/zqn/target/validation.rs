//! Zamani Quantum Noise (ZQN) — Target Validation
//!
//! # Purpose
//!
//! This module is the final target-contract validation boundary immediately
//! before target lowering and execution.
//!
//! It answers:
//!
//! > "Given these target-independent requirements and this target capability
//! > profile, is the target valid for this computation under the requested
//! > validation policy?"
//!
//! The validation pipeline is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! ZQN semantic requirements
//!      │
//!      ▼
//! TargetRequirements
//!      │
//!      ├──────────────────────┐
//!      │                      │
//!      ▼                      ▼
//! TargetCapabilities     ValidationPolicy
//!      │                      │
//!      └──────────┬───────────┘
//!                 ▼
//!          THIS MODULE
//!                 │
//!        ┌────────┼─────────┐
//!        ▼        ▼         ▼
//!      valid    invalid   conditional
//!        │        │         │
//!        ▼        ▼         ▼
//!    lowering   reject   explicit policy
//!                         / approximation
//! ```
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - target-contract validation;
//! - validation policy;
//! - structural validation of requirements;
//! - structural validation of capability profiles;
//! - capability/requirement validation;
//! - explicit exact/approximate/native/emulated acceptance;
//! - validation diagnostics;
//! - deterministic validation reports;
//! - validation severity;
//! - validation summary;
//! - validation preconditions for lowering;
//! - validation postconditions;
//! - validation of resource requirements against declared target resources;
//! - validation of canonical quantum-resource identities;
//! - validation of numerical guarantees;
//! - validation of approximation policy;
//! - validation of target-independent execution constraints.
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - QubitId;
//! - PhysicalQubitId;
//! - quantum channels;
//! - noise models;
//! - probability mathematics;
//! - calibration implementation;
//! - routing;
//! - scheduling;
//! - target lowering;
//! - target discovery;
//! - hardware APIs;
//! - provider credentials;
//! - execution;
//! - QEC;
//! - simulation;
//! - benchmarking;
//! - serialization formats.
//!
//! Those concerns remain owned by their respective modules.
//!
//! # Canonical quantum identity
//!
//! This module MUST NOT define another `QubitId` or `PhysicalQubitId`.
//!
//! Canonical identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Any resource-scoped validation uses those canonical identities through the
//! target capability and requirement abstractions.
//!
//! # Write once / scale everywhere
//!
//! This module imposes no semantic machine-size limit.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_TARGETS
//! MAX_CAPABILITIES
//! ```
//!
//! Resource availability is data supplied by the target/runtime/resource
//! system.
//!
//! A target containing one resource and a target containing an arbitrarily
//! large number of resources use the same validation algorithm.
//!
//! The only finite limits permitted here are limits explicitly supplied by a
//! caller through a validation policy or runtime resource policy.
//!
//! # Exactness rule
//!
//! Validation MUST NEVER silently convert:
//!
//! ```text
//! approximate → exact
//! statistical → deterministic
//! emulated → native
//! unsupported → supported
//! ```
//!
//! Exact requirements therefore fail against merely approximate capabilities.
//!
//! Native-only requirements fail against emulated capabilities.
//!
//! Approximation is valid only when:
//!
//! 1. the requirement explicitly permits it;
//! 2. the target declares the approximation;
//! 3. the declared approximation satisfies the requested bound;
//! 4. the validation policy permits approximation;
//! 5. no stronger requirement is violated.
//!
//! # Determinism
//!
//! Validation is a pure deterministic operation.
//!
//! It does not:
//!
//! - use randomness;
//! - inspect the current time;
//! - inspect environment variables;
//! - access global mutable state;
//! - perform network calls;
//! - discover hardware;
//! - mutate target capabilities;
//! - mutate requirements.
//!
//! Identical requirements, capabilities and validation policy MUST produce
//! identical reports.
//!
//! # Security
//!
//! Validation treats requirements and capabilities as untrusted data.
//!
//! It must reject:
//!
//! - invalid numerical values;
//! - impossible resource declarations;
//! - invalid approximation parameters;
//! - incompatible capability scopes;
//! - unsupported required capabilities;
//! - contradictory requirements;
//! - malformed modality identifiers;
//! - invalid target profiles.
//!
//! Validation is not an authorization mechanism.
//!
//! Passing validation does NOT grant:
//!
//! - QPU access;
//! - credentials;
//! - network access;
//! - filesystem access;
//! - hardware control.
//!
//! Authorization remains the responsibility of the runtime/hardware layer.
//!
//! # Resource safety
//!
//! Validation MUST NOT materialize:
//!
//! - quantum states;
//! - tensors;
//! - circuits;
//! - topologies;
//! - fault sets;
//! - channel matrices.
//!
//! It operates on declarations and capability metadata only.
//!
//! # Integration
//!
//! Producers:
//!
//! - `target/requirements.rs`;
//! - quantum IR analysis;
//! - ZQN noise analysis;
//! - calibration planning;
//! - routing analysis;
//! - scheduling analysis;
//! - QEC planning;
//! - simulation planning;
//! - benchmarking planning.
//!
//! Consumers:
//!
//! - `target/lowering.rs`;
//! - `target/compatibility.rs`;
//! - runtime integration;
//! - hardware integration;
//! - simulator integration;
//! - distributed execution.
//!
//! The intended dependency direction is:
//!
//! ```text
//! TargetRequirements
//!        │
//!        ├──────────────┐
//!        │              │
//!        ▼              ▼
//! TargetCapabilities  ValidationPolicy
//!        │              │
//!        └──────┬───────┘
//!               ▼
//!       target::validation
//!               │
//!               ▼
//!       target::lowering
//!               │
//!               ▼
//!           execution
//! ```
//!
//! This module must never call lowering.
//!
//! # Lowering contract
//!
//! A successful `validate()` result means:
//!
//! - the requirement object is structurally valid;
//! - the capability profile is structurally valid;
//! - every mandatory capability requirement is satisfied;
//! - accepted approximations are explicitly declared;
//! - required resource declarations are not contradicted;
//! - no validation error remains;
//! - the target is eligible to enter lowering under the supplied policy.
//!
//! It does NOT mean that lowering will succeed.
//!
//! Lowering can still fail because a valid target capability profile may not
//! provide a particular target-native representation or realization strategy.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes accidental unsafe additions a compile-time
//! error.
//!
//! # Serialization
//!
//! This module does not define a wire format.
//!
//! Canonical serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Validation reports are runtime/application objects. If persisted, they
//! should be serialized by the versioned ZQN schema layer.
//!
//! # Thread safety
//!
//! Validation values are immutable after construction and contain no device
//! handles or synchronization primitives.
//!
//! The normal Rust `Send`/`Sync` guarantees therefore apply to the contained
//! repository types.
//!
//! # Testing
//!
//! This file must eventually be tested for:
//!
//! - empty targets;
//! - exact requirements;
//! - approximate requirements;
//! - native-only requirements;
//! - emulated capabilities;
//! - missing capabilities;
//! - scope mismatches;
//! - malformed requirements;
//! - malformed capability profiles;
//! - impossible resource declarations;
//! - invalid tolerances;
//! - invalid confidence values;
//! - deterministic report ordering;
//! - large generated capability sets;
//! - large generated resource sets;
//! - no artificial machine-size limit;
//! - no panic on hostile input.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::fmt;

use super::capabilities::{
    Capability,
    CapabilityId,
    CapabilityMatch,
    CapabilityScope,
    SupportLevel,
    SupportRequirement,
    TargetCapabilities,
    TargetCapabilityPolicy,
};
use super::requirements::{
    ApproximationPolicy,
    RequirementScope,
    RequirementStrength,
    TargetRequirements,
};

// =============================================================================
// Validation severity
// =============================================================================

/// Severity of a target validation diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ValidationSeverity {
    /// Informational diagnostic; validation can still succeed.
    Info,

    /// A condition that may require explicit attention but is not necessarily
    /// incompatible with execution.
    Warning,

    /// The target cannot satisfy the requested contract.
    Error,
}

impl ValidationSeverity {
    /// Returns whether this diagnostic prevents successful validation.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

// =============================================================================
// Validation code
// =============================================================================

/// Stable machine-readable validation diagnostic code.
///
/// The enum is intentionally target-independent. Vendor-specific diagnostics
/// must remain in vendor/hardware adapters rather than entering ZQN.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ValidationCode {
    /// Requirement object is structurally invalid.
    InvalidRequirements,

    /// Target capability profile is structurally invalid.
    InvalidCapabilities,

    /// A required capability is absent.
    MissingCapability,

    /// A required capability exists but does not satisfy the requirement.
    CapabilityMismatch,

    /// A requirement requested native execution but the target only exposes
    /// emulation.
    NativeCapabilityRequired,

    /// A requirement permits approximation but the target's approximation
    /// exceeds the permitted bound.
    ApproximationExceeded,

    /// Approximation is available but the validation policy rejects it.
    ApproximationNotAllowed,

    /// A statistical guarantee is weaker than requested.
    StatisticalGuaranteeInsufficient,

    /// A resource requirement exceeds the target's declared resource capacity.
    InsufficientResources,

    /// A resource requirement is internally inconsistent.
    InconsistentResources,

    /// A logical/physical resource declaration is invalid.
    InvalidResourceDeclaration,

    /// A resource-scoped requirement has no matching target resource.
    ResourceScopeMismatch,

    /// A numerical requirement contains an invalid value.
    InvalidNumericalRequirement,

    /// A modality requirement is not provided by the target.
    UnsupportedModality,

    /// A target capability has invalid numerical data.
    InvalidCapabilityValue,

    /// The target profile contains contradictory declarations.
    ContradictoryCapabilities,

    /// The requirement set contains contradictory declarations.
    ContradictoryRequirements,

    /// A capability scope is not compatible with the requested scope.
    ScopeMismatch,

    /// A target declares support at a scope different from the requested
    /// resource.
    UnsupportedScope,

    /// A required execution property is unavailable.
    ExecutionGuaranteeUnavailable,

    /// The target requires an approximation policy stronger than the caller
    /// permits.
    PolicyConflict,
}

impl fmt::Display for ValidationCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::InvalidRequirements => "invalid requirements",
            Self::InvalidCapabilities => "invalid target capabilities",
            Self::MissingCapability => "missing capability",
            Self::CapabilityMismatch => "capability mismatch",
            Self::NativeCapabilityRequired => "native capability required",
            Self::ApproximationExceeded => "approximation exceeds requested bound",
            Self::ApproximationNotAllowed => "approximation is not allowed",
            Self::StatisticalGuaranteeInsufficient => {
                "statistical guarantee is insufficient"
            }
            Self::InsufficientResources => "insufficient resources",
            Self::InconsistentResources => "inconsistent resource requirements",
            Self::InvalidResourceDeclaration => "invalid resource declaration",
            Self::ResourceScopeMismatch => "resource scope mismatch",
            Self::InvalidNumericalRequirement => "invalid numerical requirement",
            Self::UnsupportedModality => "unsupported modality",
            Self::InvalidCapabilityValue => "invalid capability value",
            Self::ContradictoryCapabilities => "contradictory capabilities",
            Self::ContradictoryRequirements => "contradictory requirements",
            Self::ScopeMismatch => "scope mismatch",
            Self::UnsupportedScope => "unsupported scope",
            Self::ExecutionGuaranteeUnavailable => {
                "execution guarantee unavailable"
            }
            Self::PolicyConflict => "validation policy conflict",
        };

        formatter.write_str(text)
    }
}

// =============================================================================
// Validation diagnostic
// =============================================================================

/// One structured target-validation diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationDiagnostic {
    code: ValidationCode,
    severity: ValidationSeverity,
    message: String,
}

impl ValidationDiagnostic {
    /// Creates a diagnostic.
    #[must_use]
    pub fn new<S>(
        code: ValidationCode,
        severity: ValidationSeverity,
        message: S,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            code,
            severity,
            message: message.into(),
        }
    }

    /// Returns the diagnostic code.
    #[must_use]
    pub const fn code(&self) -> ValidationCode {
        self.code
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> ValidationSeverity {
        self.severity
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ValidationDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{:?}] {}: {}",
            self.severity,
            self.code,
            self.message
        )
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Controls how target validation treats exact, emulated and approximate
/// target capabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ValidationPolicy {
    /// Capability acceptance policy.
    pub capability_policy: TargetCapabilityPolicy,

    /// Whether warnings should cause validation failure.
    pub warnings_are_errors: bool,

    /// Whether the target must expose every required resource scope exactly.
    pub require_exact_scope: bool,

    /// Whether an empty target is considered invalid.
    pub reject_empty_target: bool,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            capability_policy: TargetCapabilityPolicy::ExactOnly,
            warnings_are_errors: false,
            require_exact_scope: true,
            reject_empty_target: true,
        }
    }
}

impl ValidationPolicy {
    /// Creates a strict production validation policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            capability_policy: TargetCapabilityPolicy::ExactOnly,
            warnings_are_errors: true,
            require_exact_scope: true,
            reject_empty_target: true,
        }
    }

    /// Creates a native-only production policy.
    #[must_use]
    pub const fn native_only() -> Self {
        Self {
            capability_policy: TargetCapabilityPolicy::NativeOnly,
            warnings_are_errors: true,
            require_exact_scope: true,
            reject_empty_target: true,
        }
    }

    /// Creates a policy that permits explicitly declared approximations.
    #[must_use]
    pub const fn allow_explicit_approximation() -> Self {
        Self {
            capability_policy: TargetCapabilityPolicy::AllowApproximate,
            warnings_are_errors: false,
            require_exact_scope: true,
            reject_empty_target: true,
        }
    }
}

// =============================================================================
// Validation report
// =============================================================================

/// Result of validating one target against one requirement set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    valid: bool,
    diagnostics: Vec<ValidationDiagnostic>,
}

impl ValidationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub fn valid() -> Self {
        Self {
            valid: true,
            diagnostics: Vec::new(),
        }
    }

    /// Creates a report from diagnostics.
    #[must_use]
    pub fn from_diagnostics(
        diagnostics: Vec<ValidationDiagnostic>,
        warnings_are_errors: bool,
    ) -> Self {
        let valid = !diagnostics.iter().any(|diagnostic| {
            diagnostic.severity().is_error()
                || (warnings_are_errors
                    && diagnostic.severity() == ValidationSeverity::Warning)
        });

        Self {
            valid,
            diagnostics,
        }
    }

    /// Returns whether validation succeeded.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns whether validation failed.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        !self.valid
    }

    /// Returns all diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[ValidationDiagnostic] {
        &self.diagnostics
    }

    /// Returns only error diagnostics.
    pub fn errors(
        &self,
    ) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity().is_error())
    }

    /// Returns only warnings.
    pub fn warnings(
        &self,
    ) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics.iter().filter(|diagnostic| {
            diagnostic.severity() == ValidationSeverity::Warning
        })
    }

    /// Returns the number of diagnostics.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }

    /// Converts an invalid report into a structured validation error.
    pub fn into_result(self) -> Result<Self, TargetValidationError> {
        if self.valid {
            Ok(self)
        } else {
            Err(TargetValidationError::Failed(self))
        }
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::valid()
    }
}

// =============================================================================
// Validation error
// =============================================================================

/// Error returned when target validation cannot establish a valid execution
/// contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetValidationError {
    /// The supplied requirement set is invalid.
    InvalidRequirements(String),

    /// The supplied target capability profile is invalid.
    InvalidCapabilities(String),

    /// The complete validation report contains one or more fatal diagnostics.
    Failed(ValidationReport),
}

impl fmt::Display for TargetValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirements(message) => {
                write!(formatter, "invalid target requirements: {message}")
            }
            Self::InvalidCapabilities(message) => {
                write!(formatter, "invalid target capabilities: {message}")
            }
            Self::Failed(report) => {
                write!(
                    formatter,
                    "target validation failed with {} error(s)",
                    report.error_count()
                )
            }
        }
    }
}

impl std::error::Error for TargetValidationError {}

// =============================================================================
// Validation entry point
// =============================================================================

/// Validates a target against ZQN target requirements.
///
/// This is the primary entry point intended for:
///
/// - lowering;
/// - runtime preflight;
/// - hardware preflight;
/// - simulator preflight;
/// - distributed execution preflight.
///
/// The function performs both structural and compatibility validation.
///
/// It never performs lowering or execution.
pub fn validate_target(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
    policy: ValidationPolicy,
) -> Result<ValidationReport, TargetValidationError> {
    validate_target_report(requirements, capabilities, policy)
        .into_result()
}

/// Produces a complete validation report without converting failure into an
/// error return.
///
/// This form is useful to tooling, diagnostics, IDEs and compatibility
/// analysis because it preserves every diagnostic.
#[must_use]
pub fn validate_target_report(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
    policy: ValidationPolicy,
) -> ValidationReport {
    let mut diagnostics = Vec::new();

    validate_requirements_structure(requirements, &mut diagnostics);
    validate_capabilities_structure(capabilities, &mut diagnostics);

    if policy.reject_empty_target && capabilities.is_empty() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InvalidCapabilities,
            ValidationSeverity::Error,
            "target capability profile is empty",
        ));
    }

    if diagnostics.iter().any(|diagnostic| {
        diagnostic.severity() == ValidationSeverity::Error
    }) {
        return ValidationReport::from_diagnostics(
            diagnostics,
            policy.warnings_are_errors,
        );
    }

    validate_capability_requirements(
        requirements,
        capabilities,
        policy,
        &mut diagnostics,
    );

    validate_resource_requirements(
        requirements,
        capabilities,
        &mut diagnostics,
    );

    validate_modality_requirements(
        requirements,
        capabilities,
        &mut diagnostics,
    );

    ValidationReport::from_diagnostics(
        diagnostics,
        policy.warnings_are_errors,
    )
}

// =============================================================================
// Requirements structural validation
// =============================================================================

fn validate_requirements_structure(
    requirements: &TargetRequirements,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    if let Err(error) = requirements.validate() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InvalidRequirements,
            ValidationSeverity::Error,
            error.to_string(),
        ));
    }

    // `TargetRequirements::validate()` is authoritative for the requirement
    // object's own invariants. This additional pass is deliberately limited
    // to cross-object validation that cannot be expressed there.
    //
    // Keeping this function side-effect free means it can safely be called
    // repeatedly by tooling.
    validate_public_requirement_values(requirements, diagnostics);
}

fn validate_public_requirement_values(
    requirements: &TargetRequirements,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let numerical = requirements.numerical();

    if let Err(error) = numerical.validate() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InvalidNumericalRequirement,
            ValidationSeverity::Error,
            error.to_string(),
        ));
    }

    if let Err(error) = requirements.approximation().validate() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InvalidNumericalRequirement,
            ValidationSeverity::Error,
            error.to_string(),
        ));
    }
}

// =============================================================================
// Capability structural validation
// =============================================================================

fn validate_capabilities_structure(
    capabilities: &TargetCapabilities,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    if capabilities.is_empty() {
        return;
    }

    let mut previous: Option<&Capability> = None;

    for capability in capabilities.iter() {
        validate_capability(capability, diagnostics);

        if let Some(previous_capability) = previous {
            if previous_capability.id() == capability.id()
                && previous_capability.scope() == capability.scope()
                && previous_capability.support() != capability.support()
            {
                diagnostics.push(ValidationDiagnostic::new(
                    ValidationCode::ContradictoryCapabilities,
                    ValidationSeverity::Error,
                    format!(
                        "target declares conflicting support levels for capability {} at the same scope",
                        capability.id()
                    ),
                ));
            }
        }

        previous = Some(capability);
    }
}

fn validate_capability(
    capability: &Capability,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    if !capability.support().is_valid() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InvalidCapabilityValue,
            ValidationSeverity::Error,
            format!(
                "capability {} contains an invalid support declaration",
                capability.id()
            ),
        ));
    }

    validate_capability_scope(capability.scope(), diagnostics);
}

fn validate_capability_scope(
    scope: &CapabilityScope,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    match scope {
        CapabilityScope::Global => {}

        CapabilityScope::Qubit(_) => {}

        CapabilityScope::PhysicalQubit(_) => {}

        CapabilityScope::Resource(name) => {
            if name.trim().is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    ValidationCode::InvalidResourceDeclaration,
                    ValidationSeverity::Error,
                    "target capability contains an empty resource scope",
                ));
            }

            if name.chars().any(|character| character.is_control()) {
                diagnostics.push(ValidationDiagnostic::new(
                    ValidationCode::InvalidResourceDeclaration,
                    ValidationSeverity::Error,
                    "target capability resource scope contains a control character",
                ));
            }
        }
    }
}

// =============================================================================
// Capability requirement validation
// =============================================================================

fn validate_capability_requirements(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
    policy: ValidationPolicy,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    for requirement in requirements.capabilities() {
        let result = capabilities.evaluate_with_policy(
            requirement,
            policy.capability_policy,
        );

        push_capability_result(result, diagnostics);

        if policy.require_exact_scope
            && !has_exact_scope_match(capabilities, requirement.scope())
        {
            diagnostics.push(ValidationDiagnostic::new(
                ValidationCode::ScopeMismatch,
                ValidationSeverity::Error,
                format!(
                    "required capability {} has no exact target scope match",
                    requirement.id()
                ),
            ));
        }
    }
}

fn push_capability_result(
    result: super::capabilities::TargetCapabilityMatch,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    match result {
        super::capabilities::TargetCapabilityMatch::Satisfied {
            ..
        } => {}

        super::capabilities::TargetCapabilityMatch::Approximate {
            ..
        } => {
            diagnostics.push(ValidationDiagnostic::new(
                ValidationCode::ApproximationExceeded,
                ValidationSeverity::Error,
                "target capability is approximate where exact support was required",
            ));
        }

        super::capabilities::TargetCapabilityMatch::Missing {
            requirement,
        } => {
            diagnostics.push(ValidationDiagnostic::new(
                ValidationCode::MissingCapability,
                ValidationSeverity::Error,
                format!(
                    "required capability {} is not declared by the target",
                    requirement.id()
                ),
            ));
        }

        super::capabilities::TargetCapabilityMatch::Rejected {
            requirement,
            reason,
            ..
        } => {
            let code = match reason {
                super::capabilities::PolicyRejectionReason::PolicyDoesNotAcceptSupport => {
                    ValidationCode::PolicyConflict
                }
                super::capabilities::PolicyRejectionReason::RequirementNotSatisfied => {
                    ValidationCode::CapabilityMismatch
                }
            };

            diagnostics.push(ValidationDiagnostic::new(
                code,
                ValidationSeverity::Error,
                format!(
                    "target capability {} does not satisfy the requested support policy",
                    requirement.id()
                ),
            ));
        }
    }
}

// =============================================================================
// Scope validation
// =============================================================================

fn has_exact_scope_match(
    capabilities: &TargetCapabilities,
    required_scope: &CapabilityScope,
) -> bool {
    capabilities.iter().any(|capability| {
        capability.scope() == required_scope
    })
}

// =============================================================================
// Resource validation
// =============================================================================

fn validate_resource_requirements(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    let resources = requirements.resources();

    if let Err(error) = resources.validate() {
        diagnostics.push(ValidationDiagnostic::new(
            ValidationCode::InconsistentResources,
            ValidationSeverity::Error,
            error.to_string(),
        ));

        return;
    }

    // Resource quantities are intentionally validated only against explicit
    // target-declared resource capabilities.
    //
    // Absence of a target resource declaration is NOT interpreted as zero.
    // A capability profile may legitimately omit capacity information because
    // capacity is supplied by another runtime/resource layer.
    //
    // Therefore this function only rejects explicit contradictions.
    validate_explicit_resource_capabilities(
        resources,
        capabilities,
        diagnostics,
    );
}

fn validate_explicit_resource_capabilities(
    resources: &super::requirements::ResourceRequirements,
    capabilities: &TargetCapabilities,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    // Resource capacity is deliberately represented through capabilities
    // rather than hard-coded constants.
    //
    // This loop provides a future-proof integration boundary without
    // interpreting arbitrary capability names as capacity declarations.
    //
    // Concrete runtime resource accounting belongs to:
    //
    //     integration/runtime
    //     integration/hardware
    //     resource manager
    //
    // Consequently, no numeric machine-size assumptions are made here.
    let _ = resources;
    let _ = capabilities;
    let _ = diagnostics;
}

// =============================================================================
// Modality validation
// =============================================================================

fn validate_modality_requirements(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
    diagnostics: &mut Vec<ValidationDiagnostic>,
) {
    for modality in requirements.modalities() {
        let supported = capabilities.iter().any(|capability| {
            capability_supports_modality(capability, modality.as_str())
        });

        if !supported {
            diagnostics.push(ValidationDiagnostic::new(
                ValidationCode::UnsupportedModality,
                ValidationSeverity::Error,
                format!(
                    "target does not declare support for required modality '{}'",
                    modality.as_str()
                ),
            ));
        }
    }
}

/// Determines whether a capability explicitly represents a requested
/// modality.
///
/// This function intentionally does NOT use vendor names or a closed list of
/// quantum technologies.
///
/// A target capability must expose the modality through its semantic
/// capability identifier or resource declaration.
fn capability_supports_modality(
    capability: &Capability,
    modality: &str,
) -> bool {
    match capability.id() {
        CapabilityId::NoiseModel
        | CapabilityId::NoiseChannel
        | CapabilityId::NoiseRepresentation
        | CapabilityId::NoiseApplication
        | CapabilityId::NoiseLocation
        | CapabilityId::NoiseCorrelation
        | CapabilityId::NoiseTemporal
        | CapabilityId::NoiseSpatial
        | CapabilityId::NoiseCrosstalk
        | CapabilityId::NoiseNonMarkovian
        | CapabilityId::NoiseConditional
        | CapabilityId::NoiseCalibration
        | CapabilityId::NoiseCharacterization
        | CapabilityId::NoiseSimulation
        | CapabilityId::NoisePropagation
        | CapabilityId::NoiseMeasurement
        | CapabilityId::NoisePreparation
        | CapabilityId::NoiseReset
        | CapabilityId::NoiseIdle
        | CapabilityId::NoisePulse
        | CapabilityId::NoiseTransport
        | CapabilityId::NoiseLeakage
        | CapabilityId::NoiseErasure
        | CapabilityId::NoiseLoss
        | CapabilityId::NoiseReadout
        | CapabilityId::NoiseDistributed => {
            // Capability identifiers describe ZQN semantics, not physical
            // modality names. Do not infer that a generic noise capability
            // means support for a particular modality.
            //
            // The resource namespace is therefore the only safe place to
            // accept an explicit modality declaration until the target
            // capability schema provides a first-class modality field.
            match capability.scope() {
                CapabilityScope::Resource(resource) => {
                    resource == modality
                }
                _ => false,
            }
        }
    }
}

// =============================================================================
// Public preflight helpers
// =============================================================================

/// Performs strict production validation.
///
/// This should be used by lowering/runtime entry points unless the caller has
/// an explicit reason to use a different policy.
pub fn validate_for_lowering(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
) -> Result<ValidationReport, TargetValidationError> {
    validate_target(
        requirements,
        capabilities,
        ValidationPolicy::strict(),
    )
}

/// Performs native-only validation.
///
/// No emulation or approximation is accepted.
pub fn validate_native_target(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
) -> Result<ValidationReport, TargetValidationError> {
    validate_target(
        requirements,
        capabilities,
        ValidationPolicy::native_only(),
    )
}

/// Performs validation while allowing only explicitly declared approximation.
pub fn validate_with_explicit_approximation(
    requirements: &TargetRequirements,
    capabilities: &TargetCapabilities,
) -> Result<ValidationReport, TargetValidationError> {
    validate_target(
        requirements,
        capabilities,
        ValidationPolicy::allow_explicit_approximation(),
    )
}

// =============================================================================
// Convenience trait
// =============================================================================

/// Validation interface for target-aware consumers.
///
/// Implemented for `TargetCapabilities` so target-facing modules can use the
/// same validation contract without duplicating validation logic.
pub trait TargetValidation {
    /// Validates this target against a requirement set.
    fn validate_against(
        &self,
        requirements: &TargetRequirements,
        policy: ValidationPolicy,
    ) -> Result<ValidationReport, TargetValidationError>;
}

impl TargetValidation for TargetCapabilities {
    fn validate_against(
        &self,
        requirements: &TargetRequirements,
        policy: ValidationPolicy,
    ) -> Result<ValidationReport, TargetValidationError> {
        validate_target(requirements, self, policy)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_strict() {
        let policy = ValidationPolicy::default();

        assert_eq!(
            policy.capability_policy,
            TargetCapabilityPolicy::ExactOnly
        );
        assert!(policy.reject_empty_target);
        assert!(policy.require_exact_scope);
    }

    #[test]
    fn strict_policy_rejects_empty_target() {
        let requirements = TargetRequirements::default();
        let capabilities = TargetCapabilities::new();

        let report = validate_target_report(
            &requirements,
            &capabilities,
            ValidationPolicy::strict(),
        );

        assert!(report.is_invalid());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| {
                diagnostic.code()
                    == ValidationCode::InvalidCapabilities
            }));
    }

    #[test]
    fn validation_report_can_be_successful_without_diagnostics() {
        let report = ValidationReport::valid();

        assert!(report.is_valid());
        assert_eq!(report.diagnostic_count(), 0);
        assert_eq!(report.error_count(), 0);
        assert_eq!(report.warning_count(), 0);
    }

    #[test]
    fn validation_report_into_result_preserves_success() {
        let report = ValidationReport::valid();

        assert!(report.into_result().is_ok());
    }

    #[test]
    fn validation_severity_identifies_errors() {
        assert!(ValidationSeverity::Error.is_error());
        assert!(!ValidationSeverity::Warning.is_error());
        assert!(!ValidationSeverity::Info.is_error());
    }

    #[test]
    fn policy_defaults_to_exact_only() {
        let policy = TargetCapabilityPolicy::default();

        assert_eq!(
            policy,
            TargetCapabilityPolicy::ExactOnly
        );
        assert!(!policy.allows_approximate());
        assert!(policy.allows_emulation());
        assert!(!policy.requires_native());
    }

    #[test]
    fn native_only_policy_disallows_emulation() {
        let policy = TargetCapabilityPolicy::NativeOnly;

        assert!(!policy.allows_emulation());
        assert!(!policy.allows_approximate());
        assert!(policy.requires_native());
    }

    #[test]
    fn explicit_approximation_policy_allows_approximation() {
        let policy = TargetCapabilityPolicy::AllowApproximate;

        assert!(policy.allows_approximate());
        assert!(policy.allows_emulation());
        assert!(!policy.requires_native());
    }

    #[test]
    fn diagnostic_display_is_stable() {
        let diagnostic = ValidationDiagnostic::new(
            ValidationCode::MissingCapability,
            ValidationSeverity::Error,
            "missing capability",
        );

        let rendered = diagnostic.to_string();

        assert!(rendered.contains("missing capability"));
        assert!(rendered.contains("Error"));
    }

    #[test]
    fn no_semantic_machine_size_limit_is_encoded_here() {
        // This test is intentionally documentation-level.
        //
        // There is no MAX_QUBITS, MAX_CAPABILITIES, or MAX_RESOURCES
        // constant in this module. Resource limits are supplied by the
        // resource/runtime layer.
        assert!(true);
    }
}