
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Differentiation Rules
//!
//! Path: src/ai/differentiation/rules.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines extensible, target-independent differentiation
//! rule contracts for the Zamani AI computational domain.
//!
//! A differentiation rule establishes the mathematical relationship
//! between an operation and its derivative operation.
//!
//! It does not execute the derivative.
//!
//! It does not construct a competing computational graph.
//!
//! It does not define another intermediate representation.
//!
//! It does not allocate tensor storage.
//!
//! It does not select physical hardware.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! Source syntax:
//!     grammar/ai/differentiation.g4
//!
//! Differentiability declarations:
//!     grammar/ai/differentiable.g4
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! AI semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! AI model operation contracts:
//!     crate::ai::model::operations
//!
//! AI tensor descriptions:
//!     crate::ai::tensor
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! Quantum computational representation:
//!     crate::quantum::ir
//!
//! Execution and resource realization:
//!     crate::runtime
//!     crate::distributed
//!     crate::hybrid
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Differentiation rule identities.
//! - Differentiation rule versions.
//! - Differentiation mode descriptors.
//! - Differentiability classifications.
//! - Derivative operation references.
//! - Rule applicability declarations.
//! - Rule provenance.
//! - Rule registration.
//! - Rule lookup.
//! - Rule compatibility.
//! - Deterministic rule selection.
//! - Ambiguity reporting.
//!
//! This module does not own:
//!
//! - Derivative graph construction.
//! - Forward-mode propagation.
//! - Reverse-mode propagation.
//! - Symbolic algebra evaluation.
//! - Numerical differentiation kernels.
//! - Tensor element storage.
//! - Canonical semantic type definitions.
//! - ZUIR operations.
//! - Quantum IR.
//! - Hardware placement.
//! - Resource allocation.
//! - Runtime execution.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! No universal maximum is imposed on:
//!
//! - Number of differentiation rules.
//! - Number of operations.
//! - Number of registered providers.
//! - Number of differentiation modes.
//! - Number of rule versions.
//! - Number of derivative relationships.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Model size.
//! - Number of parameters.
//! - Number of devices.
//! - Number of workers.
//! - Memory capacity.
//!
//! Collections grow according to available resources and the host's
//! representational limits.
//!
//! This module does not promise literal infinite computation.
//!
//! It guarantees that it does not introduce arbitrary language-level
//! capacity ceilings.
//!
//! ============================================================================
//! IMPORTANT SEMANTIC DISTINCTIONS
//! ============================================================================
//!
//! A registered rule is not proof that a particular operation is
//! differentiable for every possible input.
//!
//! A rule applicability declaration is not a completed semantic check.
//!
//! A differentiation mode is not a hardware selection.
//!
//! A derivative operation reference is not an executable kernel.
//!
//! A registered rule is not proof that a target can execute it.
//!
//! A successful rule lookup is not proof of resource feasibility.
//!
//! These checks belong to the appropriate semantic, compiler and
//! execution subsystems.
//!
//! ============================================================================

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates an externally supplied identity or qualified name.
///
/// Empty strings and control characters are rejected.
///
/// Whitespace inside a nonempty identity is preserved because the
/// canonical frontend may define qualified-name normalization rules.
fn validate_identity(
    value: &str,
) -> Result<(), DifferentiationRuleError> {
    if value.trim().is_empty() {
        return Err(DifferentiationRuleError::EmptyIdentity);
    }

    if value.chars().any(char::is_control) {
        return Err(DifferentiationRuleError::ControlCharacterInIdentity);
    }

    Ok(())
}

/// Validates a descriptive metadata string.
///
/// Unlike identities, descriptive strings may be empty.
fn validate_metadata(
    value: &str,
) -> Result<(), DifferentiationRuleError> {
    if value.chars().any(char::is_control) {
        return Err(DifferentiationRuleError::InvalidMetadata);
    }

    Ok(())
}

// ============================================================================
// RULE IDENTITIES
// ============================================================================

/// Stable identity of a differentiation rule.
///
/// The identity is independent of:
///
/// - Hardware.
/// - Compiler backend.
/// - Execution provider.
/// - Tensor storage.
/// - Runtime implementation.
///
/// A rule identity must be resolved through the differentiation registry.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DifferentiationRuleId(Arc<str>);

impl DifferentiationRuleId {
    /// Creates a differentiation rule identity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let value = value.as_ref();

        validate_identity(value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the underlying identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DifferentiationRuleId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// OPERATION IDENTITIES
// ============================================================================

/// Identity of the operation to which a differentiation rule applies.
///
/// The identity is intended to correspond to an operation resolved by
/// the canonical AI operation/model registry or another canonical
/// computational operation registry.
///
/// This module intentionally stores the identity as an opaque string
/// rather than introducing a second operation type system.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DifferentiableOperationId(Arc<str>);

impl DifferentiableOperationId {
    /// Creates an operation identity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let value = value.as_ref();

        validate_identity(value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DifferentiableOperationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DERIVATIVE OPERATION REFERENCES
// ============================================================================

/// Reference to the canonical operation implementing a derivative
/// transformation.
///
/// This is a semantic reference.
///
/// It is not an implementation pointer, executable function, kernel,
/// graph node, or IR operation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DerivativeOperationRef(Arc<str>);

impl DerivativeOperationRef {
    /// Creates a derivative operation reference.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let value = value.as_ref();

        validate_identity(value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the canonical operation reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DerivativeOperationRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RULE VERSIONS
// ============================================================================

/// Version of a differentiation rule contract.
///
/// This is distinct from:
///
/// - Zamani language version.
/// - AI subsystem version.
/// - ZUIR dialect version.
/// - Backend implementation version.
/// - Runtime version.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct DifferentiationRuleVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl DifferentiationRuleVersion {
    /// Creates a rule version.
    #[must_use]
    pub const fn new(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for DifferentiationRuleVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// DIFFERENTIATION MODES
// ============================================================================

/// Requested or supported differentiation strategy.
///
/// The built-in variants describe common strategies.
///
/// `Custom` permits future differentiation strategies without requiring
/// a permanent modification of this enum.
///
/// A mode is a computational strategy descriptor, not a device choice.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationMode {
    /// Automatically select a semantically valid differentiation strategy.
    Automatic,

    /// Forward-mode automatic differentiation.
    Forward,

    /// Reverse-mode automatic differentiation.
    Reverse,

    /// Symbolic differentiation.
    Symbolic,

    /// Numerical differentiation.
    Numerical,

    /// Adjoint differentiation.
    Adjoint,

    /// Parameter-shift differentiation.
    ParameterShift,

    /// Finite-difference differentiation.
    FiniteDifference,

    /// An extension-defined differentiation strategy.
    Custom(Arc<str>),
}

impl DifferentiationMode {
    /// Creates an extension-defined differentiation mode.
    pub fn custom(
        identity: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let identity = identity.as_ref();

        validate_identity(identity)?;

        Ok(Self::Custom(Arc::from(identity)))
    }

    /// Returns the stable textual mode identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        match self {
            Self::Automatic => "automatic",
            Self::Forward => "forward",
            Self::Reverse => "reverse",
            Self::Symbolic => "symbolic",
            Self::Numerical => "numerical",
            Self::Adjoint => "adjoint",
            Self::ParameterShift => "parameter_shift",
            Self::FiniteDifference => "finite_difference",
            Self::Custom(identity) => identity,
        }
    }
}

// ============================================================================
// DIFFERENTIABILITY CLASSIFICATION
// ============================================================================

/// Mathematical differentiability classification of a rule.
///
/// These classifications describe the conditions under which the
/// derivative relationship is mathematically valid.
///
/// The actual input-dependent validation is performed by the canonical
/// semantic and differentiation validation infrastructure.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum Differentiability {
    /// Differentiable under the operation's declared mathematical domain.
    Differentiable,

    /// Differentiable only when additional conditions are established.
    Conditional,

    /// The operation has a generalized or almost-everywhere derivative.
    Generalized,

    /// Differentiability depends on runtime or semantic information.
    RuntimeDependent,

    /// The operation has no valid derivative under this rule.
    NotDifferentiable,

    /// An extension-defined differentiability classification.
    Custom(Arc<str>),
}

impl Differentiability {
    /// Creates an extension-defined classification.
    pub fn custom(
        identity: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let identity = identity.as_ref();

        validate_identity(identity)?;

        Ok(Self::Custom(Arc::from(identity)))
    }

    /// Returns whether the classification establishes unconditional
    /// differentiability.
    ///
    /// This does not replace input-specific semantic validation.
    #[must_use]
    pub fn is_unconditionally_differentiable(&self) -> bool {
        matches!(self, Self::Differentiable)
    }
}

// ============================================================================
// MATHEMATICAL REGULARITY
// ============================================================================

/// Regularity requirements associated with a differentiation rule.
///
/// Regularity is separate from the requested derivative order.
///
/// This module does not impose a universal maximum derivative order.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationRegularity {
    /// No additional regularity declaration is required by the rule.
    Unspecified,

    /// The operation is continuous under the applicable domain.
    Continuous,

    /// The operation is once continuously differentiable.
    ContinuouslyDifferentiable,

    /// The operation is repeatedly differentiable.
    Smooth,

    /// The operation is analytic within its declared domain.
    Analytic,

    /// The operation is piecewise differentiable.
    PiecewiseDifferentiable,

    /// Regularity is established through a semantic condition.
    Conditional(Arc<str>),

    /// Extension-defined regularity.
    Custom(Arc<str>),
}

impl DifferentiationRegularity {
    /// Creates a conditional regularity declaration.
    pub fn conditional(
        condition: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let condition = condition.as_ref();

        validate_identity(condition)?;

        Ok(Self::Conditional(Arc::from(condition)))
    }

    /// Creates an extension-defined regularity declaration.
    pub fn custom(
        identity: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let identity = identity.as_ref();

        validate_identity(identity)?;

        Ok(Self::Custom(Arc::from(identity)))
    }
}

// ============================================================================
// RULE APPLICABILITY
// ============================================================================

/// Domain in which a differentiation rule is applicable.
///
/// Domains describe computational semantics rather than physical
/// execution locations.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationDomain {
    Classical,
    Tensor,
    Symbolic,
    Probabilistic,
    Quantum,
    Hybrid,
    Custom(Arc<str>),
}

impl DifferentiationDomain {
    /// Creates an extension-defined domain.
    pub fn custom(
        identity: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let identity = identity.as_ref();

        validate_identity(identity)?;

        Ok(Self::Custom(Arc::from(identity)))
    }
}

// ============================================================================
// RULE PROVENANCE
// ============================================================================

/// Provenance associated with a differentiation rule.
///
/// This information identifies the origin and authority of a rule.
/// It does not itself prove that the rule has been verified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationRuleProvenance {
    source: Arc<str>,
    authority: Arc<str>,
    description: Arc<str>,
}

impl DifferentiationRuleProvenance {
    /// Creates rule provenance.
    pub fn new(
        source: impl AsRef<str>,
        authority: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<Self, DifferentiationRuleError> {
        let source = source.as_ref();
        let authority = authority.as_ref();
        let description = description.as_ref();

        validate_identity(source)?;
        validate_identity(authority)?;
        validate_metadata(description)?;

        Ok(Self {
            source: Arc::from(source),
            authority: Arc::from(authority),
            description: Arc::from(description),
        })
    }

    /// Returns the rule origin.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the rule authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the rule description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// ============================================================================
// RULE VERIFICATION STATUS
// ============================================================================

/// Verification state of a differentiation rule.
///
/// Registration must not be confused with mathematical verification.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum DifferentiationRuleVerification {
    /// The rule has been declared but not independently verified.
    Unverified,

    /// The rule has passed the applicable mathematical verification.
    Verified,

    /// Verification is incomplete.
    Pending,

    /// The rule has been rejected.
    Rejected,

    /// An extension-defined verification state.
    Custom(Arc<str>),
}

impl DifferentiationRuleVerification {
    /// Returns whether the rule is approved for use by a policy that
    /// requires verified rules.
    #[must_use]
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// DIFFERENTIATION RULE CONTRACT
// ============================================================================

/// Complete target-independent differentiation rule description.
///
/// A rule associates an operation with a derivative operation and
/// specifies the mathematical and semantic conditions under which
/// the relationship may be used.
///
/// The derivative operation reference must resolve through the
/// canonical operation infrastructure.
///
/// No executable implementation is embedded in this structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationRule {
    id: DifferentiationRuleId,

    operation: DifferentiableOperationId,

    derivative: DerivativeOperationRef,

    version: DifferentiationRuleVersion,

    modes: Vec<DifferentiationMode>,

    domains: Vec<DifferentiationDomain>,

    differentiability: Differentiability,

    regularity: DifferentiationRegularity,

    verification: DifferentiationRuleVerification,

    provenance: DifferentiationRuleProvenance,
}

impl DifferentiationRule {
    /// Creates a differentiation rule.
    ///
    /// A rule must declare at least one supported differentiation mode
    /// and one applicable semantic domain.
    pub fn new(
        id: DifferentiationRuleId,
        operation: DifferentiableOperationId,
        derivative: DerivativeOperationRef,
        version: DifferentiationRuleVersion,
        modes: Vec<DifferentiationMode>,
        domains: Vec<DifferentiationDomain>,
        differentiability: Differentiability,
        regularity: DifferentiationRegularity,
        provenance: DifferentiationRuleProvenance,
    ) -> Result<Self, DifferentiationRuleError> {
        if modes.is_empty() {
            return Err(DifferentiationRuleError::MissingMode);
        }

        if domains.is_empty() {
            return Err(DifferentiationRuleError::MissingDomain);
        }

        if modes
            .iter()
            .any(|mode| mode.identity().trim().is_empty())
        {
            return Err(DifferentiationRuleError::InvalidMode);
        }

        if matches!(
            differentiability,
            Differentiability::NotDifferentiable
        ) {
            return Err(
                DifferentiationRuleError::RuleForNonDifferentiableOperation
            );
        }

        let mut normalized_modes = modes;
        normalized_modes.sort();
        normalized_modes.dedup();

        let mut normalized_domains = domains;
        normalized_domains.sort();
        normalized_domains.dedup();

        Ok(Self {
            id,
            operation,
            derivative,
            version,
            modes: normalized_modes,
            domains: normalized_domains,
            differentiability,
            regularity,
            verification: DifferentiationRuleVerification::Unverified,
            provenance,
        })
    }

    /// Sets the verification status.
    ///
    /// The caller is responsible for supplying evidence through the
    /// repository's verification and provenance infrastructure.
    #[must_use]
    pub fn with_verification(
        mut self,
        verification: DifferentiationRuleVerification,
    ) -> Self {
        self.verification = verification;
        self
    }

    /// Returns the rule identity.
    #[must_use]
    pub fn id(&self) -> &DifferentiationRuleId {
        &self.id
    }

    /// Returns the differentiated operation.
    #[must_use]
    pub fn operation(&self) -> &DifferentiableOperationId {
        &self.operation
    }

    /// Returns the derivative operation reference.
    #[must_use]
    pub fn derivative(&self) -> &DerivativeOperationRef {
        &self.derivative
    }

    /// Returns the rule version.
    #[must_use]
    pub const fn version(&self) -> DifferentiationRuleVersion {
        self.version
    }

    /// Returns supported differentiation modes.
    #[must_use]
    pub fn modes(&self) -> &[DifferentiationMode] {
        &self.modes
    }

    /// Returns applicable computational domains.
    #[must_use]
    pub fn domains(&self) -> &[DifferentiationDomain] {
        &self.domains
    }

    /// Returns the differentiability classification.
    #[must_use]
    pub fn differentiability(&self) -> &Differentiability {
        &self.differentiability
    }

    /// Returns the mathematical regularity.
    #[must_use]
    pub fn regularity(&self) -> &DifferentiationRegularity {
        &self.regularity
    }

    /// Returns the verification status.
    #[must_use]
    pub fn verification(&self) -> &DifferentiationRuleVerification {
        &self.verification
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> &DifferentiationRuleProvenance {
        &self.provenance
    }

    /// Returns whether this rule supports the supplied mode.
    #[must_use]
    pub fn supports_mode(
        &self,
        mode: &DifferentiationMode,
    ) -> bool {
        self.modes.contains(mode)
    }

    /// Returns whether this rule applies to the supplied domain.
    #[must_use]
    pub fn supports_domain(
        &self,
        domain: &DifferentiationDomain,
    ) -> bool {
        self.domains.contains(domain)
    }
}

// ============================================================================
// RULE LOOKUP REQUEST
// ============================================================================

/// A request to locate a differentiation rule.
///
/// The request contains semantic intent only.
///
/// It does not specify a physical target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationRuleRequest {
    operation: DifferentiableOperationId,
    mode: DifferentiationMode,
    domain: DifferentiationDomain,
    required_version: Option<DifferentiationRuleVersion>,
    require_verified: bool,
}

impl DifferentiationRuleRequest {
    /// Creates a rule lookup request.
    #[must_use]
    pub fn new(
        operation: DifferentiableOperationId,
        mode: DifferentiationMode,
        domain: DifferentiationDomain,
    ) -> Self {
        Self {
            operation,
            mode,
            domain,
            required_version: None,
            require_verified: false,
        }
    }

    /// Requires an exact rule version.
    #[must_use]
    pub fn with_version(
        mut self,
        version: DifferentiationRuleVersion,
    ) -> Self {
        self.required_version = Some(version);
        self
    }

    /// Requires mathematical verification.
    #[must_use]
    pub fn requiring_verified_rule(mut self) -> Self {
        self.require_verified = true;
        self
    }

    /// Returns the requested operation.
    #[must_use]
    pub fn operation(&self) -> &DifferentiableOperationId {
        &self.operation
    }

    /// Returns the requested differentiation mode.
    #[must_use]
    pub fn mode(&self) -> &DifferentiationMode {
        &self.mode
    }

    /// Returns the requested domain.
    #[must_use]
    pub fn domain(&self) -> &DifferentiationDomain {
        &self.domain
    }
}

// ============================================================================
// RULE RESOLUTION
// ============================================================================

/// Outcome of differentiation rule resolution.
///
/// Multiple matching rules are explicitly reported as ambiguous.
/// The registry never silently chooses a potentially incompatible rule.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DifferentiationRuleResolution {
    /// Exactly one rule matches.
    Resolved(DifferentiationRule),

    /// No registered rule matches.
    NotFound,

    /// Multiple compatible rule versions match.
    Ambiguous(Vec<DifferentiationRuleId>),

    /// Matching rules exist but do not meet verification policy.
    VerificationRequired,

    /// Matching rules exist but their requested version is unavailable.
    VersionUnavailable,
}

// ============================================================================
// REGISTRY KEYS
// ============================================================================

/// Stable registry key.
///
/// Multiple versions may coexist.
///
/// A different operation, mode or version is a distinct registration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct DifferentiationRuleKey {
    operation: DifferentiableOperationId,
    mode: DifferentiationMode,
    version: DifferentiationRuleVersion,
}

// ============================================================================
// REGISTRY ERRORS
// ============================================================================

/// Errors produced by differentiation rule registration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DifferentiationRuleError {
    EmptyIdentity,
    ControlCharacterInIdentity,
    InvalidMetadata,
    MissingMode,
    MissingDomain,
    InvalidMode,
    DuplicateRule,
    RuleIdentityConflict,
    RuleForNonDifferentiableOperation,
    RegistryPoisoned,
}

impl fmt::Display for DifferentiationRuleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentity => {
                write!(formatter, "differentiation identity cannot be empty")
            }

            Self::ControlCharacterInIdentity => {
                write!(formatter, "identity contains a control character")
            }

            Self::InvalidMetadata => {
                write!(formatter, "invalid differentiation metadata")
            }

            Self::MissingMode => {
                write!(formatter, "differentiation rule has no mode")
            }

            Self::MissingDomain => {
                write!(formatter, "differentiation rule has no domain")
            }

            Self::InvalidMode => {
                write!(formatter, "invalid differentiation mode")
            }

            Self::DuplicateRule => {
                write!(formatter, "duplicate differentiation rule")
            }

            Self::RuleIdentityConflict => {
                write!(formatter, "differentiation rule identity conflict")
            }

            Self::RuleForNonDifferentiableOperation => {
                write!(
                    formatter,
                    "cannot register a derivative rule for an operation \
                     declared non-differentiable"
                )
            }

            Self::RegistryPoisoned => {
                write!(formatter, "differentiation registry lock poisoned")
            }
        }
    }
}

impl Error for DifferentiationRuleError {}

// ============================================================================
// DIFFERENTIATION RULE REGISTRY
// ============================================================================

/// Thread-safe differentiation rule registry.
///
/// Responsibilities:
///
/// - Atomic rule registration.
/// - Duplicate detection.
/// - Version coexistence.
/// - Deterministic enumeration.
/// - Exact operation/mode/domain lookup.
/// - Ambiguity detection.
/// - Verification policy.
/// - Rule removal by identity and version.
///
/// The registry contains descriptions, not executable kernels.
///
/// A read/write lock protects registry state. Lock poisoning is reported
/// explicitly rather than silently recovered.
///
/// This registry is local to the AI differentiation domain. It does not
/// replace the repository-wide dialect, capability, or implementation
/// registries.
#[derive(Clone, Debug, Default)]
pub struct DifferentiationRuleRegistry {
    rules: Arc<RwLock<BTreeMap<DifferentiationRuleKey, DifferentiationRule>>>,
}

impl DifferentiationRuleRegistry {
    /// Creates an empty differentiation rule registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a differentiation rule.
    ///
    /// Registration is atomic.
    ///
    /// An existing operation/mode/version entry cannot be silently
    /// overwritten.
    pub fn register(
        &self,
        rule: DifferentiationRule,
    ) -> Result<(), DifferentiationRuleError> {
        let mut rules = self
            .rules
            .write()
            .map_err(|_| DifferentiationRuleError::RegistryPoisoned)?;

        for mode in rule.modes() {
            let key = DifferentiationRuleKey {
                operation: rule.operation().clone(),
                mode: mode.clone(),
                version: rule.version(),
            };

            if rules.contains_key(&key) {
                return Err(DifferentiationRuleError::DuplicateRule);
            }
        }

        // Validate all identity conflicts before inserting anything.
        if rules.values().any(|existing| {
            existing.id() == rule.id()
                && (existing.operation() != rule.operation()
                    || existing.version() != rule.version())
        }) {
            return Err(DifferentiationRuleError::RuleIdentityConflict);
        }

        for mode in rule.modes() {
            let key = DifferentiationRuleKey {
                operation: rule.operation().clone(),
                mode: mode.clone(),
                version: rule.version(),
            };

            rules.insert(key, rule.clone());
        }

        Ok(())
    }

    /// Resolves a differentiation rule.
    ///
    /// When multiple versions match an unversioned request, the outcome
    /// is explicitly ambiguous.
    ///
    /// No implicit "latest version" behavior is used because different
    /// versions may have different mathematical or semantic contracts.
    pub fn resolve(
        &self,
        request: &DifferentiationRuleRequest,
    ) -> Result<DifferentiationRuleResolution, DifferentiationRuleError> {
        let rules = self
            .rules
            .read()
            .map_err(|_| DifferentiationRuleError::RegistryPoisoned)?;

        let mut matching = rules
            .values()
            .filter(|rule| {
                rule.operation() == request.operation()
                    && rule.supports_mode(request.mode())
                    && rule.supports_domain(request.domain())
            })
            .filter(|rule| {
                request
                    .required_version
                    .is_none_or(|version| rule.version() == version)
            })
            .cloned()
            .collect::<Vec<_>>();

        matching.sort_by(|left, right| {
            left.version()
                .cmp(&right.version())
                .then_with(|| left.id().cmp(right.id()))
        });

        if matching.is_empty() {
            let operation_has_rules = rules.values().any(|rule| {
                rule.operation() == request.operation()
                    && rule.supports_mode(request.mode())
            });

            if request.required_version.is_some() && operation_has_rules {
                return Ok(
                    DifferentiationRuleResolution::VersionUnavailable
                );
            }

            return Ok(DifferentiationRuleResolution::NotFound);
        }

        if request.require_verified
            && matching.iter().any(|rule| !rule.verification().is_verified())
        {
            return Ok(
                DifferentiationRuleResolution::VerificationRequired
            );
        }

        if matching.len() == 1 {
            return Ok(
                DifferentiationRuleResolution::Resolved(
                    matching.remove(0)
                )
            );
        }

        let identities = matching
            .iter()
            .map(|rule| rule.id().clone())
            .collect();

        Ok(DifferentiationRuleResolution::Ambiguous(identities))
    }

    /// Returns the number of unique registered rule identities and
    /// versions.
    ///
    /// A rule supporting multiple modes is counted once.
    pub fn len(
        &self,
    ) -> Result<usize, DifferentiationRuleError> {
        let rules = self
            .rules
            .read()
            .map_err(|_| DifferentiationRuleError::RegistryPoisoned)?;

        let identities = rules
            .values()
            .map(|rule| (rule.id().clone(), rule.version()))
            .collect::<std::collections::BTreeSet<_>>();

        Ok(identities.len())
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(
        &self,
    ) -> Result<bool, DifferentiationRuleError> {
        Ok(self.len()? == 0)
    }

    /// Returns registered rules in deterministic order.
    ///
    /// A rule supporting multiple modes is returned once.
    pub fn rules(
        &self,
    ) -> Result<Vec<DifferentiationRule>, DifferentiationRuleError> {
        let rules = self
            .rules
            .read()
            .map_err(|_| DifferentiationRuleError::RegistryPoisoned)?;

        let mut unique = BTreeMap::new();

        for rule in rules.values() {
            unique.insert(
                (rule.id().clone(), rule.version()),
                rule.clone(),
            );
        }

        Ok(unique.into_values().collect())
    }

    /// Removes a rule by identity and exact version.
    ///
    /// All mode registrations associated with that rule are removed
    /// atomically.
    pub fn unregister(
        &self,
        id: &DifferentiationRuleId,
        version: DifferentiationRuleVersion,
    ) -> Result<bool, DifferentiationRuleError> {
        let mut rules = self
            .rules
            .write()
            .map_err(|_| DifferentiationRuleError::RegistryPoisoned)?;

        let keys = rules
            .iter()
            .filter(|(_, rule)| {
                rule.id() == id && rule.version() == version
            })
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();

        if keys.is_empty() {
            return Ok(false);
        }

        for key in keys {
            rules.remove(&key);
        }

        Ok(true)
    }
}

// ============================================================================
// STANDARD RULE IDENTITIES
// ============================================================================

/// Canonical operation identity namespace used by standard mathematical
/// differentiation rules.
///
/// These identifiers are semantic references. The canonical operation
/// registry must provide the corresponding operations.
pub mod standard_operations {
    pub const ADD: &str = "math::add";
    pub const SUBTRACT: &str = "math::subtract";
    pub const MULTIPLY: &str = "math::multiply";
    pub const DIVIDE: &str = "math::divide";
    pub const NEGATE: &str = "math::negate";
    pub const EXPONENTIAL: &str = "math::exp";
    pub const LOGARITHM: &str = "math::log";
    pub const SINE: &str = "math::sin";
    pub const COSINE: &str = "math::cos";
    pub const POWER: &str = "math::power";
}

/// Canonical derivative operation identity namespace.
///
/// Implementations are supplied through canonical operation registration
/// and the appropriate ZUIR lowering adapters.
pub mod standard_derivatives {
    pub const ADD: &str = "math::derivative_add";
    pub const SUBTRACT: &str = "math::derivative_subtract";
    pub const MULTIPLY: &str = "math::derivative_multiply";
    pub const DIVIDE: &str = "math::derivative_divide";
    pub const NEGATE: &str = "math::derivative_negate";
    pub const EXPONENTIAL: &str = "math::derivative_exp";
    pub const LOGARITHM: &str = "math::derivative_log";
    pub const SINE: &str = "math::derivative_sin";
    pub const COSINE: &str = "math::derivative_cos";
    pub const POWER: &str = "math::derivative_power";
}

// ============================================================================
// STANDARD RULE CONSTRUCTION
// ============================================================================

/// Creates a standard rule provenance record.
///
/// This identifies the standard mathematical rule collection.
/// Verification remains a separate obligation.
fn standard_provenance(
    description: &'static str,
) -> Result<DifferentiationRuleProvenance, DifferentiationRuleError> {
    DifferentiationRuleProvenance::new(
        "zamani::standard_differentiation",
        "zamani::mathematical_operation_specification",
        description,
    )
}

/// Constructs a standard mathematical differentiation rule.
///
/// Standard rules are initially unverified. They must not be treated
/// as mathematically certified merely because they are built in.
fn standard_rule(
    operation: &'static str,
    derivative: &'static str,
    modes: Vec<DifferentiationMode>,
    description: &'static str,
) -> Result<DifferentiationRule, DifferentiationRuleError> {
    DifferentiationRule::new(
        DifferentiationRuleId::new(
            format!("standard::{operation}")
        )?,
        DifferentiableOperationId::new(operation)?,
        DerivativeOperationRef::new(derivative)?,
        DifferentiationRuleVersion::new(1, 0, 0),
        modes,
        vec![
            DifferentiationDomain::Classical,
            DifferentiationDomain::Tensor,
            DifferentiationDomain::Symbolic,
        ],
        Differentiability::Differentiable,
        DifferentiationRegularity::Smooth,
        standard_provenance(description)?,
    )
}

/// Creates an empty registry populated with standard mathematical
/// derivative contracts.
///
/// This function registers descriptions only.
///
/// It does not assert that the corresponding derivative operations
/// have already been implemented by every backend.
pub fn standard_rule_registry(
) -> Result<DifferentiationRuleRegistry, DifferentiationRuleError> {
    let registry = DifferentiationRuleRegistry::new();

    let modes = vec![
        DifferentiationMode::Automatic,
        DifferentiationMode::Forward,
        DifferentiationMode::Reverse,
        DifferentiationMode::Symbolic,
    ];

    let rules = [
        standard_rule(
            standard_operations::ADD,
            standard_derivatives::ADD,
            modes.clone(),
            "Linearity of addition.",
        )?,
        standard_rule(
            standard_operations::SUBTRACT,
            standard_derivatives::SUBTRACT,
            modes.clone(),
            "Linearity of subtraction.",
        )?,
        standard_rule(
            standard_operations::MULTIPLY,
            standard_derivatives::MULTIPLY,
            modes.clone(),
            "Product differentiation rule.",
        )?,
        standard_rule(
            standard_operations::DIVIDE,
            standard_derivatives::DIVIDE,
            modes.clone(),
            "Quotient differentiation rule under a nonzero denominator.",
        )?,
        standard_rule(
            standard_operations::NEGATE,
            standard_derivatives::NEGATE,
            modes.clone(),
            "Linearity of negation.",
        )?,
        standard_rule(
            standard_operations::EXPONENTIAL,
            standard_derivatives::EXPONENTIAL,
            modes.clone(),
            "Derivative of the exponential function.",
        )?,
        standard_rule(
            standard_operations::LOGARITHM,
            standard_derivatives::LOGARITHM,
            modes.clone(),
            "Derivative of the logarithm under its valid domain.",
        )?,
        standard_rule(
            standard_operations::SINE,
            standard_derivatives::SINE,
            modes.clone(),
            "Derivative of the sine function.",
        )?,
        standard_rule(
            standard_operations::COSINE,
            standard_derivatives::COSINE,
            modes.clone(),
            "Derivative of the cosine function.",
        )?,
        standard_rule(
            standard_operations::POWER,
            standard_derivatives::POWER,
            modes,
            "Power differentiation under its declared domain.",
        )?,
    ];

    for rule in rules {
        registry.register(rule)?;
    }

    Ok(registry)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> DifferentiationRuleProvenance {
        DifferentiationRuleProvenance::new(
            "test::source",
            "test::authority",
            "Test differentiation rule.",
        )
        .unwrap()
    }

    fn rule(
        id: &str,
        operation: &str,
        version: DifferentiationRuleVersion,
    ) -> DifferentiationRule {
        DifferentiationRule::new(
            DifferentiationRuleId::new(id).unwrap(),
            DifferentiableOperationId::new(operation).unwrap(),
            DerivativeOperationRef::new("math::derivative").unwrap(),
            version,
            vec![
                DifferentiationMode::Forward,
                DifferentiationMode::Reverse,
            ],
            vec![DifferentiationDomain::Classical],
            Differentiability::Differentiable,
            DifferentiationRegularity::Smooth,
            provenance(),
        )
        .unwrap()
    }

    fn request(
        operation: &str,
        mode: DifferentiationMode,
    ) -> DifferentiationRuleRequest {
        DifferentiationRuleRequest::new(
            DifferentiableOperationId::new(operation).unwrap(),
            mode,
            DifferentiationDomain::Classical,
        )
    }

    #[test]
    fn creates_valid_rule() {
        let value = rule(
            "test::rule",
            "math::test",
            DifferentiationRuleVersion::new(1, 0, 0),
        );

        assert_eq!(value.id().as_str(), "test::rule");
        assert_eq!(value.operation().as_str(), "math::test");
        assert_eq!(
            value.derivative().as_str(),
            "math::derivative"
        );
    }

    #[test]
    fn rejects_empty_rule_identity() {
        assert!(
            DifferentiationRuleId::new(" ").is_err()
        );
    }

    #[test]
    fn rejects_control_characters() {
        assert!(
            DifferentiableOperationId::new("math::add\n")
                .is_err()
        );
    }

    #[test]
    fn rejects_rule_without_modes() {
        let result = DifferentiationRule::new(
            DifferentiationRuleId::new("test::rule").unwrap(),
            DifferentiableOperationId::new("math::add").unwrap(),
            DerivativeOperationRef::new("math::dadd").unwrap(),
            DifferentiationRuleVersion::new(1, 0, 0),
            Vec::new(),
            vec![DifferentiationDomain::Classical],
            Differentiability::Differentiable,
            DifferentiationRegularity::Smooth,
            provenance(),
        );

        assert_eq!(
            result.unwrap_err(),
            DifferentiationRuleError::MissingMode
        );
    }

    #[test]
    fn rejects_rule_without_domains() {
        let result = DifferentiationRule::new(
            DifferentiationRuleId::new("test::rule").unwrap(),
            DifferentiableOperationId::new("math::add").unwrap(),
            DerivativeOperationRef::new("math::dadd").unwrap(),
            DifferentiationRuleVersion::new(1, 0, 0),
            vec![DifferentiationMode::Forward],
            Vec::new(),
            Differentiability::Differentiable,
            DifferentiationRegularity::Smooth,
            provenance(),
        );

        assert_eq!(
            result.unwrap_err(),
            DifferentiationRuleError::MissingDomain
        );
    }

    #[test]
    fn rejects_rule_for_non_differentiable_operation() {
        let result = DifferentiationRule::new(
            DifferentiationRuleId::new("test::rule").unwrap(),
            DifferentiableOperationId::new("math::floor").unwrap(),
            DerivativeOperationRef::new("math::dfloor").unwrap(),
            DifferentiationRuleVersion::new(1, 0, 0),
            vec![DifferentiationMode::Forward],
            vec![DifferentiationDomain::Classical],
            Differentiability::NotDifferentiable,
            DifferentiationRegularity::Unspecified,
            provenance(),
        );

        assert_eq!(
            result.unwrap_err(),
            DifferentiationRuleError::RuleForNonDifferentiableOperation
        );
    }

    #[test]
    fn registry_registers_and_resolves_rule() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        let result = registry
            .resolve(&request(
                "math::add",
                DifferentiationMode::Forward,
            ))
            .unwrap();

        assert!(matches!(
            result,
            DifferentiationRuleResolution::Resolved(_)
        ));
    }

    #[test]
    fn registry_registers_all_modes_atomically() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        assert_eq!(registry.len().unwrap(), 1);

        let result = registry.resolve(
            &request(
                "math::add",
                DifferentiationMode::Reverse,
            )
        ).unwrap();

        assert!(matches!(
            result,
            DifferentiationRuleResolution::Resolved(_)
        ));
    }

    #[test]
    fn registry_rejects_duplicate_rule_registration() {
        let registry = DifferentiationRuleRegistry::new();

        let first = rule(
            "test::rule",
            "math::add",
            DifferentiationRuleVersion::new(1, 0, 0),
        );

        let second = first.clone();

        registry.register(first).unwrap();

        assert_eq!(
            registry.register(second).unwrap_err(),
            DifferentiationRuleError::DuplicateRule
        );
    }

    #[test]
    fn different_rule_versions_can_coexist() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(2, 0, 0),
            )
        ).unwrap();

        assert_eq!(registry.len().unwrap(), 2);
    }

    #[test]
    fn unversioned_multiple_matches_are_ambiguous() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(2, 0, 0),
            )
        ).unwrap();

        let result = registry
            .resolve(&request(
                "math::add",
                DifferentiationMode::Forward,
            ))
            .unwrap();

        assert!(matches!(
            result,
            DifferentiationRuleResolution::Ambiguous(_)
        ));
    }

    #[test]
    fn exact_version_resolves_unambiguously() {
        let registry = DifferentiationRuleRegistry::new();

        let version = DifferentiationRuleVersion::new(2, 0, 0);

        registry.register(
            rule("test::rule", "math::add", version)
        ).unwrap();

        let result = registry
            .resolve(
                &request(
                    "math::add",
                    DifferentiationMode::Forward,
                )
                .with_version(version)
            )
            .unwrap();

        assert!(matches!(
            result,
            DifferentiationRuleResolution::Resolved(_)
        ));
    }

    #[test]
    fn unsupported_mode_returns_not_found() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::rule",
                "math::add",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        let result = registry
            .resolve(&request(
                "math::add",
                DifferentiationMode::Symbolic,
            ))
            .unwrap();

        assert_eq!(
            result,
            DifferentiationRuleResolution::NotFound
        );
    }

    #[test]
    fn rule_unregistration_removes_every_mode() {
        let registry = DifferentiationRuleRegistry::new();

        let version = DifferentiationRuleVersion::new(1, 0, 0);

        registry.register(
            rule("test::rule", "math::add", version)
        ).unwrap();

        assert!(
            registry.unregister(
                &DifferentiationRuleId::new("test::rule").unwrap(),
                version,
            ).unwrap()
        );

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn custom_modes_are_supported() {
        let mode = DifferentiationMode::custom(
            "extension::special_gradient"
        ).unwrap();

        assert_eq!(
            mode.identity(),
            "extension::special_gradient"
        );
    }

    #[test]
    fn custom_differentiability_is_supported() {
        let classification = Differentiability::custom(
            "extension::piecewise"
        ).unwrap();

        assert!(matches!(
            classification,
            Differentiability::Custom(_)
        ));
    }

    #[test]
    fn standard_registry_is_extensible() {
        let registry = standard_rule_registry().unwrap();

        assert!(!registry.is_empty().unwrap());

        let result = registry
            .resolve(&request(
                standard_operations::ADD,
                DifferentiationMode::Forward,
            ))
            .unwrap();

        assert!(matches!(
            result,
            DifferentiationRuleResolution::Resolved(_)
        ));
    }

    #[test]
    fn standard_rules_are_not_automatically_certified() {
        let registry = standard_rule_registry().unwrap();

        let result = registry
            .resolve(
                &request(
                    standard_operations::ADD,
                    DifferentiationMode::Forward,
                )
                .requiring_verified_rule()
            )
            .unwrap();

        assert_eq!(
            result,
            DifferentiationRuleResolution::VerificationRequired
        );
    }

    #[test]
    fn standard_rule_registry_has_no_hardware_dependency() {
        let registry = standard_rule_registry().unwrap();

        let rules = registry.rules().unwrap();

        assert!(
            rules.iter().all(|rule| {
                !rule.operation().as_str().contains("gpu")
                    && !rule.operation().as_str().contains("cpu")
            })
        );
    }

    #[test]
    fn rule_enumeration_is_deterministic() {
        let registry = DifferentiationRuleRegistry::new();

        registry.register(
            rule(
                "test::z",
                "math::z",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        registry.register(
            rule(
                "test::a",
                "math::a",
                DifferentiationRuleVersion::new(1, 0, 0),
            )
        ).unwrap();

        let first = registry.rules().unwrap();
        let second = registry.rules().unwrap();

        assert_eq!(first, second);
    }
}
