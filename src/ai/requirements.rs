
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Requirements
//!
//! Path: src/ai/requirements.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the target-independent computational requirements of AI programs.
//!
//! This module preserves AI-specific requirements and aggregates references
//! to the canonical semantic resource and capability models.
//!
//! It is an intent and validation boundary, not an execution planner.
//!
//! ## Authority
//!
//! - grammar/ai/README.md
//! - grammar/ai/ai-capabilities.g4
//! - grammar/ai/ai-accelerators.g4
//! - grammar/ai/tensors.g4
//! - grammar/ai/training.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - src/frontend/semantic/domains/ai.rs
//! - src/frontend/semantic/resources/model.rs
//! - src/frontend/semantic/capabilities/model.rs
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI-specific computational requirement descriptions;
//! - AI numerical precision requirements;
//! - AI operation semantic requirements;
//! - symbolic tensor shape requirements;
//! - abstract memory-access requirements;
//! - automatic differentiation requirements;
//! - numerical reproducibility requirements;
//! - AI security requirement descriptions;
//! - abstract execution requirements;
//! - aggregation and completeness of AI requirements;
//! - requirement-level validation;
//! - explicit distinction between mandatory and advisory intent.
//!
//! ## Does not own
//!
//! - source grammar;
//! - AST construction;
//! - name resolution;
//! - canonical type checking;
//! - canonical resource quantities;
//! - canonical resource constraints;
//! - canonical capability identities;
//! - capability evidence;
//! - authorization enforcement;
//! - target discovery;
//! - physical memory allocation;
//! - device selection;
//! - placement;
//! - scheduling;
//! - routing;
//! - feasibility computation;
//! - ZUIR, ZCIR, HDLIR or quantum::ir;
//! - runtime execution.
//!
//! ## Integration
//!
//! Canonical resources:
//!   frontend::semantic::resources::model
//!
//! Canonical capabilities:
//!   frontend::semantic::capabilities::model
//!
//! Canonical symbolic AI shapes:
//!   frontend::semantic::domains::ai
//!
//! Canonical AST provenance:
//!   frontend::ast::node::node_id
//!
//! Distributed aggregation:
//!   distributed::requirements
//!
//! Compiler, IR and runtime adapters consume validated requirements
//! through explicit integration boundaries.
//!
//! ## POCO-REAF
//!
//! No universal limits are imposed on:
//!
//! - model size;
//! - parameter count;
//! - tensor rank;
//! - tensor dimensions;
//! - memory capacity;
//! - accelerator count;
//! - processor count;
//! - distributed node count;
//! - training steps;
//! - inference operations;
//! - pipeline size;
//! - agent count;
//! - computational workload.
//!
//! Collections grow according to available implementation resources.
//!
//! Operational budgets are external policies, not semantic ceilings.
//!
//! This module does not promise literal infinite physical capacity.
//!
//! ## Important semantic distinctions
//!
//! Requirement:
//!   A mandatory condition for preserving program semantics.
//!
//! Constraint:
//!   A restriction on admissible realizations.
//!
//! Preference:
//!   A non-mandatory optimization criterion.
//!
//! Hint:
//!   Optional implementation guidance.
//!
//! Capability:
//!   An ability that an execution environment may provide.
//!
//! Feasibility:
//!   A downstream conclusion based on requirements, evidence,
//!   policies and actual available resources.
//!
//! These concepts must never be silently interchanged.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

use crate::frontend::semantic::capabilities::model::{
    CapabilityRequirement,
};

use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceRequirement,
};

use crate::frontend::semantic::domains::ai::{
    DimensionExpr,
    TensorShape,
};

/// Version of the in-memory AI requirements API.
///
/// This is not a language version, model version, IR version,
/// or serialized artifact version.
pub const AI_REQUIREMENTS_API_VERSION: u16 = 1;

// ============================================================================
// Namespaced identities
// ============================================================================

/// An open-world, namespaced AI requirement identity.
///
/// New AI domains and extension requirements do not require adding
/// variants to a closed enumeration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementKey {
    namespace: String,
    name: String,
}

impl RequirementKey {
    /// Creates a validated requirement identity.
    ///
    /// Names are preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns an error when either component is empty or contains
    /// control characters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(&namespace)?;
        validate_identity_component(&name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the qualified display name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for RequirementKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

fn validate_identity_component(
    value: &str,
) -> Result<(), RequirementError> {
    if value.trim().is_empty() {
        return Err(RequirementError::EmptyIdentityComponent);
    }

    if value.chars().any(char::is_control) {
        return Err(RequirementError::ControlCharacter);
    }

    Ok(())
}

// ============================================================================
// Requirement classification
// ============================================================================

/// The semantic role of an AI requirement.
///
/// Mandatory and advisory declarations must remain distinguishable.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RequirementKind {
    /// A mandatory semantic requirement.
    Required,

    /// A mandatory restriction on admissible realizations.
    Constraint,

    /// A non-mandatory optimization preference.
    Preference,

    /// Non-binding implementation guidance.
    Hint,
}

impl RequirementKind {
    /// Returns whether the declaration is binding.
    #[must_use]
    pub const fn is_binding(self) -> bool {
        matches!(
            self,
            Self::Required | Self::Constraint
        )
    }

    /// Returns whether the declaration is advisory.
    #[must_use]
    pub const fn is_advisory(self) -> bool {
        matches!(
            self,
            Self::Preference | Self::Hint
        )
    }
}

impl fmt::Display for RequirementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Required => "required",
            Self::Constraint => "constraint",
            Self::Preference => "preference",
            Self::Hint => "hint",
        })
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Provenance for an AI requirement.
///
/// This identifies the source AST node when one exists.
/// It is not evidence that a requirement has been validated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RequirementProvenance {
    source_node: Option<NodeId>,
}

impl RequirementProvenance {
    /// Creates provenance from a canonical AST node.
    #[must_use]
    pub const fn from_node(node: NodeId) -> Self {
        Self {
            source_node: Some(node),
        }
    }

    /// Creates provenance for a derived semantic requirement.
    #[must_use]
    pub const fn derived() -> Self {
        Self {
            source_node: None,
        }
    }

    /// Returns the originating AST node.
    #[must_use]
    pub const fn source_node(self) -> Option<NodeId> {
        self.source_node
    }
}

impl Default for RequirementProvenance {
    fn default() -> Self {
        Self::derived()
    }
}

// ============================================================================
// Extensible requirement values
// ============================================================================

/// A generic value for open-world AI extension requirements.
///
/// This is a data representation, not another expression language.
/// Symbolic values must be resolved by their owning semantic service.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum RequirementValue {
    /// Boolean value.
    Boolean(bool),

    /// Signed integer.
    Signed(i128),

    /// Unsigned integer.
    Unsigned(u128),

    /// Exact decimal or domain-defined numerical representation.
    Number(String),

    /// Text value.
    Text(String),

    /// Opaque symbolic expression identity.
    Symbolic(String),

    /// Sequence of values.
    Sequence(Vec<RequirementValue>),

    /// Deterministically ordered object.
    Object(BTreeMap<String, RequirementValue>),
}

impl RequirementValue {
    /// Validates local representation invariants.
    ///
    /// This does not evaluate symbolic expressions or validate
    /// a value against an extension-specific schema.
    pub fn validate(&self) -> Result<(), RequirementError> {
        match self {
            Self::Boolean(_)
            | Self::Signed(_)
            | Self::Unsigned(_) => Ok(()),

            Self::Number(value)
            | Self::Symbolic(value) => {
                validate_identity_component(value)
            }

            Self::Text(value) => {
                if value.chars().any(char::is_control) {
                    return Err(RequirementError::ControlCharacter);
                }

                Ok(())
            }

            Self::Sequence(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Object(values) => {
                for (key, value) in values {
                    validate_identity_component(key)?;
                    value.validate()?;
                }

                Ok(())
            }
        }
    }
}

/// An open-world AI extension requirement.
///
/// The namespace owner defines the meaning, schema, compatibility
/// and versioning of the extension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionRequirement {
    key: RequirementKey,
    value: RequirementValue,
}

impl ExtensionRequirement {
    /// Creates an extension requirement.
    pub fn new(
        key: RequirementKey,
        value: RequirementValue,
    ) -> Result<Self, RequirementError> {
        value.validate()?;

        Ok(Self {
            key,
            value,
        })
    }

    /// Returns the extension identity.
    #[must_use]
    pub fn key(&self) -> &RequirementKey {
        &self.key
    }

    /// Returns the extension value.
    #[must_use]
    pub fn value(&self) -> &RequirementValue {
        &self.value
    }
}

// ============================================================================
// Numerical precision requirements
// ============================================================================

/// A numerical format identity.
///
/// The identity is open-world. It does not imply that every execution
/// target supports that numerical representation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NumericalFormat {
    key: RequirementKey,
}

impl NumericalFormat {
    /// Creates a numerical format identity.
    #[must_use]
    pub fn new(key: RequirementKey) -> Self {
        Self { key }
    }

    /// Returns the format identity.
    #[must_use]
    pub fn key(&self) -> &RequirementKey {
        &self.key
    }
}

/// Rounding behavior required by a computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoundingRequirement {
    /// Rounding behavior is determined by the numerical operation.
    OperationDefined,

    /// An explicitly identified rounding policy.
    Named(RequirementKey),
}

/// Numerical precision requirements.
///
/// These describe mathematical and numerical intent, not physical
/// register widths or a particular processor's implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionRequirement {
    format: NumericalFormat,
    rounding: RoundingRequirement,
    accumulation_format: Option<NumericalFormat>,
    allow_lossy_conversion: bool,
}

impl PrecisionRequirement {
    /// Creates a precision requirement.
    #[must_use]
    pub fn new(
        format: NumericalFormat,
        rounding: RoundingRequirement,
        accumulation_format: Option<NumericalFormat>,
        allow_lossy_conversion: bool,
    ) -> Self {
        Self {
            format,
            rounding,
            accumulation_format,
            allow_lossy_conversion,
        }
    }

    /// Returns the required numerical format.
    #[must_use]
    pub fn format(&self) -> &NumericalFormat {
        &self.format
    }

    /// Returns the rounding requirement.
    #[must_use]
    pub fn rounding(&self) -> &RoundingRequirement {
        &self.rounding
    }

    /// Returns the optional accumulation format.
    #[must_use]
    pub fn accumulation_format(&self) -> Option<&NumericalFormat> {
        self.accumulation_format.as_ref()
    }

    /// Returns whether lossy conversion is permitted.
    #[must_use]
    pub const fn allows_lossy_conversion(&self) -> bool {
        self.allow_lossy_conversion
    }
}

// ============================================================================
// Operation semantics
// ============================================================================

/// Describes the semantic equivalence required of an operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationEquivalence {
    /// Exact preservation of specified observable semantics.
    Exact,

    /// Equivalent within a specified, externally validated tolerance.
    Tolerance {
        absolute: Option<String>,
        relative: Option<String>,
    },

    /// Equivalent under a named semantic contract.
    Contract(RequirementKey),
}

/// An AI operation semantic requirement.
///
/// The operation name is open-world and independent of its
/// implementation on any particular device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationRequirement {
    operation: RequirementKey,
    equivalence: OperationEquivalence,
}

impl OperationRequirement {
    /// Creates an operation requirement.
    #[must_use]
    pub fn new(
        operation: RequirementKey,
        equivalence: OperationEquivalence,
    ) -> Self {
        Self {
            operation,
            equivalence,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &RequirementKey {
        &self.operation
    }

    /// Returns the equivalence requirement.
    #[must_use]
    pub fn equivalence(&self) -> &OperationEquivalence {
        &self.equivalence
    }
}

// ============================================================================
// Symbolic shape requirements
// ============================================================================

/// A symbolic shape requirement.
///
/// The underlying shape representation is owned by the existing
/// frontend AI semantic module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShapeRequirement {
    shape: TensorShape,
    require_compatible_dimensions: bool,
}

impl ShapeRequirement {
    /// Creates a shape requirement.
    #[must_use]
    pub fn new(
        shape: TensorShape,
        require_compatible_dimensions: bool,
    ) -> Self {
        Self {
            shape,
            require_compatible_dimensions,
        }
    }

    /// Returns the symbolic tensor shape.
    #[must_use]
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    /// Returns whether dimensions must be compatible.
    #[must_use]
    pub const fn requires_compatible_dimensions(&self) -> bool {
        self.require_compatible_dimensions
    }
}

/// Returns symbolic dimensions without evaluating them.
///
/// Evaluation belongs to the canonical semantic constraint services.
#[must_use]
pub fn symbolic_dimensions(
    shape: &ShapeRequirement,
) -> &[DimensionExpr] {
    &shape.shape.dimensions
}

// ============================================================================
// Memory access requirements
// ============================================================================

/// Abstract memory access behavior.
///
/// These are logical access semantics, not physical memory locations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryAccessMode {
    /// Read-only access.
    Read,

    /// Write-only access.
    Write,

    /// Read and write access.
    ReadWrite,

    /// Atomic access semantics.
    Atomic,
}

/// Abstract memory consistency requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryConsistency {
    /// Operation-defined consistency.
    OperationDefined,

    /// A named consistency contract.
    Contract(RequirementKey),
}

/// AI memory access requirements.
///
/// Does not specify a physical address, device, memory bank,
/// processor, or accelerator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryAccessRequirement {
    mode: MemoryAccessMode,
    consistency: MemoryConsistency,
    aliasing_contract: Option<RequirementKey>,
}

impl MemoryAccessRequirement {
    /// Creates a memory access requirement.
    #[must_use]
    pub fn new(
        mode: MemoryAccessMode,
        consistency: MemoryConsistency,
        aliasing_contract: Option<RequirementKey>,
    ) -> Self {
        Self {
            mode,
            consistency,
            aliasing_contract,
        }
    }

    /// Returns the access mode.
    #[must_use]
    pub const fn mode(&self) -> MemoryAccessMode {
        self.mode
    }

    /// Returns the consistency contract.
    #[must_use]
    pub fn consistency(&self) -> &MemoryConsistency {
        &self.consistency
    }

    /// Returns the optional aliasing contract.
    #[must_use]
    pub fn aliasing_contract(&self) -> Option<&RequirementKey> {
        self.aliasing_contract.as_ref()
    }
}

// ============================================================================
// Automatic differentiation requirements
// ============================================================================

/// Differentiation mode required by an AI computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DifferentiationMode {
    /// Forward-mode differentiation.
    Forward,

    /// Reverse-mode differentiation.
    Reverse,

    /// Both modes are acceptable or required by the enclosing contract.
    Both,

    /// An extension-defined differentiation mode.
    Custom(RequirementKey),
}

/// Automatic differentiation requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DifferentiationRequirement {
    mode: DifferentiationMode,
    higher_order: bool,
    preserve_gradient_semantics: bool,
    checkpointing_permitted: bool,
}

impl DifferentiationRequirement {
    /// Creates a differentiation requirement.
    #[must_use]
    pub fn new(
        mode: DifferentiationMode,
        higher_order: bool,
        preserve_gradient_semantics: bool,
        checkpointing_permitted: bool,
    ) -> Self {
        Self {
            mode,
            higher_order,
            preserve_gradient_semantics,
            checkpointing_permitted,
        }
    }

    /// Returns the differentiation mode.
    #[must_use]
    pub fn mode(&self) -> &DifferentiationMode {
        &self.mode
    }

    /// Returns whether higher-order differentiation is required.
    #[must_use]
    pub const fn requires_higher_order(&self) -> bool {
        self.higher_order
    }

    /// Returns whether gradient semantics must be preserved.
    #[must_use]
    pub const fn preserves_gradient_semantics(&self) -> bool {
        self.preserve_gradient_semantics
    }

    /// Returns whether checkpointing is permitted.
    #[must_use]
    pub const fn checkpointing_permitted(&self) -> bool {
        self.checkpointing_permitted
    }
}

// ============================================================================
// Reproducibility requirements
// ============================================================================

/// Numerical reproducibility level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReproducibilityLevel {
    /// Bitwise identical results are required.
    Bitwise,

    /// Deterministic results under the declared semantic contract.
    Deterministic,

    /// Statistical equivalence is sufficient.
    Statistical,

    /// An extension-defined reproducibility contract.
    Custom(RequirementKey),
}

/// Reproducibility requirements for AI computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReproducibilityRequirement {
    level: ReproducibilityLevel,
    seed: Option<String>,
    preserve_operation_order: bool,
}

impl ReproducibilityRequirement {
    /// Creates a reproducibility requirement.
    #[must_use]
    pub fn new(
        level: ReproducibilityLevel,
        seed: Option<String>,
        preserve_operation_order: bool,
    ) -> Self {
        Self {
            level,
            seed,
            preserve_operation_order,
        }
    }

    /// Returns the reproducibility level.
    #[must_use]
    pub fn level(&self) -> &ReproducibilityLevel {
        &self.level
    }

    /// Returns the optional symbolic or literal seed representation.
    #[must_use]
    pub fn seed(&self) -> Option<&str> {
        self.seed.as_deref()
    }

    /// Returns whether operation ordering must be preserved.
    #[must_use]
    pub const fn preserves_operation_order(&self) -> bool {
        self.preserve_operation_order
    }
}

// ============================================================================
// Security requirements
// ============================================================================

/// An AI security requirement.
///
/// Security identities and policy semantics are open-world and remain
/// integrated with the canonical security and policy subsystems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityRequirement {
    policy: RequirementKey,
    mandatory: bool,
}

impl SecurityRequirement {
    /// Creates a security requirement.
    #[must_use]
    pub fn new(
        policy: RequirementKey,
        mandatory: bool,
    ) -> Self {
        Self {
            policy,
            mandatory,
        }
    }

    /// Returns the security policy identity.
    #[must_use]
    pub fn policy(&self) -> &RequirementKey {
        &self.policy
    }

    /// Returns whether the policy is mandatory.
    #[must_use]
    pub const fn is_mandatory(&self) -> bool {
        self.mandatory
    }
}

// ============================================================================
// Abstract execution requirements
// ============================================================================

/// Abstract AI execution mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionMode {
    /// Execution without a parallelism requirement.
    Unspecified,

    /// Parallel execution semantics are required.
    Parallel,

    /// Incremental or streaming execution is required.
    Streaming,

    /// Batch-oriented execution is required.
    Batch,

    /// Interactive execution is required.
    Interactive,

    /// Extension-defined execution mode.
    Custom(RequirementKey),
}

/// AI execution requirements.
///
/// This describes execution intent. It does not perform scheduling,
/// placement, resource allocation or hardware discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionRequirement {
    mode: ExecutionMode,
    cancellation_supported: bool,
    checkpoint_recovery_required: bool,
}

impl ExecutionRequirement {
    /// Creates an execution requirement.
    #[must_use]
    pub fn new(
        mode: ExecutionMode,
        cancellation_supported: bool,
        checkpoint_recovery_required: bool,
    ) -> Self {
        Self {
            mode,
            cancellation_supported,
            checkpoint_recovery_required,
        }
    }

    /// Returns the requested execution mode.
    #[must_use]
    pub fn mode(&self) -> &ExecutionMode {
        &self.mode
    }

    /// Returns whether cancellation support is required.
    #[must_use]
    pub const fn cancellation_supported(&self) -> bool {
        self.cancellation_supported
    }

    /// Returns whether checkpoint recovery is required.
    #[must_use]
    pub const fn checkpoint_recovery_required(&self) -> bool {
        self.checkpoint_recovery_required
    }
}

// ============================================================================
// AI requirement subjects
// ============================================================================

/// A typed AI requirement subject.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AiRequirementSubject {
    /// Numerical precision.
    Precision(PrecisionRequirement),

    /// Operation semantics.
    Operation(OperationRequirement),

    /// Symbolic tensor shape.
    Shape(ShapeRequirement),

    /// Memory access semantics.
    MemoryAccess(MemoryAccessRequirement),

    /// Automatic differentiation.
    Differentiation(DifferentiationRequirement),

    /// Numerical reproducibility.
    Reproducibility(ReproducibilityRequirement),

    /// Security and policy intent.
    Security(SecurityRequirement),

    /// Abstract execution behavior.
    Execution(ExecutionRequirement),

    /// Open-world extension requirement.
    Extension(ExtensionRequirement),
}

/// An AI requirement with classification and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiRequirement {
    kind: RequirementKind,
    subject: AiRequirementSubject,
    provenance: RequirementProvenance,
}

impl AiRequirement {
    /// Creates an AI requirement.
    #[must_use]
    pub fn new(
        kind: RequirementKind,
        subject: AiRequirementSubject,
        provenance: RequirementProvenance,
    ) -> Self {
        Self {
            kind,
            subject,
            provenance,
        }
    }

    /// Returns the requirement classification.
    #[must_use]
    pub const fn kind(&self) -> RequirementKind {
        self.kind
    }

    /// Returns the requirement subject.
    #[must_use]
    pub fn subject(&self) -> &AiRequirementSubject {
        &self.subject
    }

    /// Returns the requirement provenance.
    #[must_use]
    pub const fn provenance(&self) -> RequirementProvenance {
        self.provenance
    }
}

// ============================================================================
// Canonical capability integration
// ============================================================================

/// Strength assigned to a canonical capability predicate.
///
/// Capability identity and version semantics remain owned by the
/// canonical semantic capability model.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CapabilityStrength {
    /// The capability must be available.
    Required,

    /// The capability is preferred.
    Preferred,

    /// The capability may be used.
    Allowed,

    /// The capability must not be used.
    Forbidden,
}

/// A canonical capability predicate with AI-specific classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiCapabilityRequirement {
    predicate: CapabilityRequirement,
    strength: CapabilityStrength,
    provenance: RequirementProvenance,
}

impl AiCapabilityRequirement {
    /// Wraps an existing canonical capability requirement.
    #[must_use]
    pub const fn new(
        predicate: CapabilityRequirement,
        strength: CapabilityStrength,
        provenance: RequirementProvenance,
    ) -> Self {
        Self {
            predicate,
            strength,
            provenance,
        }
    }

    /// Returns the canonical capability predicate.
    #[must_use]
    pub fn predicate(&self) -> &CapabilityRequirement {
        &self.predicate
    }

    /// Returns the capability strength.
    #[must_use]
    pub const fn strength(&self) -> CapabilityStrength {
        self.strength
    }

    /// Returns its provenance.
    #[must_use]
    pub const fn provenance(&self) -> RequirementProvenance {
        self.provenance
    }
}

// ============================================================================
// Analysis completeness
// ============================================================================

/// Status of AI requirement analysis.
///
/// Incomplete analysis must never be promoted to certified success.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementAnalysisStatus {
    /// Every declared requirement source was analyzed.
    Complete,

    /// One or more requirement sources remain unresolved.
    Incomplete {
        /// Reasons analysis is incomplete.
        reasons: Vec<String>,
    },
}

impl RequirementAnalysisStatus {
    /// Returns whether the analysis is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

// ============================================================================
// Downstream feasibility result vocabulary
// ============================================================================

/// A feasibility outcome produced by a downstream feasibility service.
///
/// This module does not calculate or certify feasibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeasibilityStatus {
    /// Feasibility has not been evaluated.
    NotEvaluated,

    /// Available evidence establishes feasibility for a specified scope.
    Feasible,

    /// Available evidence establishes infeasibility for a specified scope.
    Infeasible,

    /// Available evidence is insufficient for a definitive conclusion.
    Indeterminate,
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by AI requirement construction and aggregation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementError {
    /// An identity component is empty.
    EmptyIdentityComponent,

    /// A textual component contains control characters.
    ControlCharacter,

    /// An incomplete requirement set was used as a complete set.
    IncompleteAnalysis {
        /// Reasons analysis is incomplete.
        reasons: Vec<String>,
    },

    /// An incomplete-analysis reason is empty.
    EmptyIncompleteReason,
}

impl fmt::Display for RequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentityComponent => {
                f.write_str("requirement identity component cannot be empty")
            }

            Self::ControlCharacter => {
                f.write_str("requirement text contains a control character")
            }

            Self::IncompleteAnalysis { reasons } => {
                write!(
                    f,
                    "AI requirement analysis is incomplete: {}",
                    reasons.join("; ")
                )
            }

            Self::EmptyIncompleteReason => {
                f.write_str("incomplete-analysis reason cannot be empty")
            }
        }
    }
}

impl Error for RequirementError {}

// ============================================================================
// Complete AI requirement envelope
// ============================================================================

/// The target-independent requirement envelope for AI computation.
///
/// Canonical resources and capabilities are retained directly rather
/// than copied into competing AI-specific representations.
///
/// Insertion order is preserved for provenance and source diagnostics.
/// Consumers requiring canonical presentation ordering must explicitly
/// sort using their semantic ordering contract.
///
/// Duplicate declarations are preserved. Their semantic combination
/// belongs to the appropriate semantic service.
///
/// No universal entry-count limit exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiRequirements {
    requirements: Vec<AiRequirement>,
    capabilities: Vec<AiCapabilityRequirement>,
    resources: Vec<ResourceRequirement>,
    resource_constraints: Vec<ResourceConstraint>,
    status: RequirementAnalysisStatus,
}

impl Default for AiRequirements {
    fn default() -> Self {
        Self::new()
    }
}

impl AiRequirements {
    /// Creates an empty, complete AI requirement collection.
    ///
    /// The caller must only use this constructor when all relevant
    /// requirement sources are known to have been examined.
    #[must_use]
    pub fn new() -> Self {
        Self {
            requirements: Vec::new(),
            capabilities: Vec::new(),
            resources: Vec::new(),
            resource_constraints: Vec::new(),
            status: RequirementAnalysisStatus::Complete,
        }
    }

    /// Returns AI-specific requirements.
    #[must_use]
    pub fn requirements(&self) -> &[AiRequirement] {
        &self.requirements
    }

    /// Returns canonical capability requirements.
    #[must_use]
    pub fn capabilities(&self) -> &[AiCapabilityRequirement] {
        &self.capabilities
    }

    /// Returns canonical resource requirements.
    #[must_use]
    pub fn resources(&self) -> &[ResourceRequirement] {
        &self.resources
    }

    /// Returns canonical resource constraints.
    #[must_use]
    pub fn resource_constraints(&self) -> &[ResourceConstraint] {
        &self.resource_constraints
    }

    /// Returns the analysis status.
    #[must_use]
    pub fn status(&self) -> &RequirementAnalysisStatus {
        &self.status
    }

    /// Returns whether analysis is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Adds an AI-specific requirement.
    pub fn add_requirement(
        &mut self,
        requirement: AiRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Adds a canonical capability requirement.
    pub fn add_capability(
        &mut self,
        capability: AiCapabilityRequirement,
    ) {
        self.capabilities.push(capability);
    }

    /// Adds an existing canonical resource requirement.
    pub fn add_resource(
        &mut self,
        resource: ResourceRequirement,
    ) {
        self.resources.push(resource);
    }

    /// Adds an existing canonical resource constraint.
    pub fn add_resource_constraint(
        &mut self,
        constraint: ResourceConstraint,
    ) {
        self.resource_constraints.push(constraint);
    }

    /// Marks requirement analysis incomplete.
    ///
    /// Previous reasons are never discarded.
    ///
    /// # Errors
    ///
    /// Returns an error when the reason is empty or contains
    /// control characters.
    pub fn mark_incomplete(
        &mut self,
        reason: impl Into<String>,
    ) -> Result<(), RequirementError> {
        let reason = reason.into();

        if reason.trim().is_empty() {
            return Err(RequirementError::EmptyIncompleteReason);
        }

        if reason.chars().any(char::is_control) {
            return Err(RequirementError::ControlCharacter);
        }

        match &mut self.status {
            RequirementAnalysisStatus::Complete => {
                self.status = RequirementAnalysisStatus::Incomplete {
                    reasons: vec![reason],
                };
            }

            RequirementAnalysisStatus::Incomplete { reasons } => {
                reasons.push(reason);
            }
        }

        Ok(())
    }

    /// Merges another AI requirement envelope.
    ///
    /// Entries and incomplete-analysis reasons are preserved.
    /// No requirements are silently removed or promoted.
    pub fn merge(&mut self, other: Self) {
        self.requirements.extend(other.requirements);
        self.capabilities.extend(other.capabilities);
        self.resources.extend(other.resources);
        self.resource_constraints.extend(other.resource_constraints);

        if let RequirementAnalysisStatus::Incomplete { reasons } = other.status {
            for reason in reasons {
                let _ = self.mark_incomplete(reason);
            }
        }
    }

    /// Returns whether the collection contains no requirement entries.
    ///
    /// Check `is_complete()` separately before interpreting an empty
    /// collection as evidence that no requirements exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
            && self.capabilities.is_empty()
            && self.resources.is_empty()
            && self.resource_constraints.is_empty()
    }

    /// Consumes the collection, returning it only if analysis is complete.
    ///
    /// # Errors
    ///
    /// Returns an error if any requirement source remains unresolved.
    pub fn into_complete(self) -> Result<Self, RequirementError> {
        match &self.status {
            RequirementAnalysisStatus::Complete => Ok(self),

            RequirementAnalysisStatus::Incomplete { reasons } => {
                Err(RequirementError::IncompleteAnalysis {
                    reasons: reasons.clone(),
                })
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(namespace: &str, name: &str) -> RequirementKey {
        RequirementKey::new(namespace, name)
            .expect("valid requirement identity")
    }

    #[test]
    fn namespaced_identities_are_open_world() {
        let identity = key(
            "future.photonic.ai",
            "coherent_inference",
        );

        assert_eq!(
            identity.qualified_name(),
            "future.photonic.ai:coherent_inference"
        );
    }

    #[test]
    fn invalid_identities_are_rejected() {
        assert!(RequirementKey::new("", "inference").is_err());
        assert!(RequirementKey::new("ai", "").is_err());
        assert!(RequirementKey::new("ai", "bad\nname").is_err());
    }

    #[test]
    fn mandatory_and_advisory_intent_are_distinct() {
        assert!(RequirementKind::Required.is_binding());
        assert!(RequirementKind::Constraint.is_binding());

        assert!(RequirementKind::Preference.is_advisory());
        assert!(RequirementKind::Hint.is_advisory());
    }

    #[test]
    fn precision_requirements_preserve_numerical_intent() {
        let precision = PrecisionRequirement::new(
            NumericalFormat::new(key("numeric", "custom_format")),
            RoundingRequirement::OperationDefined,
            Some(NumericalFormat::new(
                key("numeric", "accumulation_format"),
            )),
            false,
        );

        assert_eq!(
            precision.format().key().name(),
            "custom_format"
        );

        assert!(!precision.allows_lossy_conversion());
        assert!(precision.accumulation_format().is_some());
    }

    #[test]
    fn symbolic_shapes_are_preserved_without_evaluation() {
        let shape = ShapeRequirement::new(
            TensorShape::new(vec![
                DimensionExpr::Constant(128),
            ]),
            true,
        );

        assert_eq!(symbolic_dimensions(&shape).len(), 1);
        assert!(shape.requires_compatible_dimensions());
    }

    #[test]
    fn memory_requirements_are_logical_not_physical() {
        let access = MemoryAccessRequirement::new(
            MemoryAccessMode::ReadWrite,
            MemoryConsistency::OperationDefined,
            None,
        );

        assert_eq!(
            access.mode(),
            MemoryAccessMode::ReadWrite
        );
    }

    #[test]
    fn differentiation_requirements_preserve_gradient_contracts() {
        let differentiation = DifferentiationRequirement::new(
            DifferentiationMode::Reverse,
            true,
            true,
            true,
        );

        assert!(differentiation.requires_higher_order());
        assert!(differentiation.preserves_gradient_semantics());
        assert!(differentiation.checkpointing_permitted());
    }

    #[test]
    fn reproducibility_is_explicit() {
        let reproducibility = ReproducibilityRequirement::new(
            ReproducibilityLevel::Bitwise,
            Some("training-seed".to_owned()),
            true,
        );

        assert_eq!(
            reproducibility.level(),
            &ReproducibilityLevel::Bitwise
        );

        assert!(reproducibility.preserves_operation_order());
    }

    #[test]
    fn extension_requirements_accept_future_domains() {
        let extension = ExtensionRequirement::new(
            key("future.neuromorphic", "event_precision"),
            RequirementValue::Text("adaptive".to_owned()),
        )
        .expect("valid extension");

        assert_eq!(
            extension.key().namespace(),
            "future.neuromorphic"
        );
    }

    #[test]
    fn empty_requirements_are_complete() {
        let requirements = AiRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.is_complete());
        assert!(requirements.into_complete().is_ok());
    }

    #[test]
    fn incomplete_requirements_cannot_be_certified() {
        let mut requirements = AiRequirements::new();

        requirements
            .mark_incomplete("symbolic requirement unresolved")
            .expect("valid reason");

        assert!(!requirements.is_complete());

        assert!(matches!(
            requirements.into_complete(),
            Err(RequirementError::IncompleteAnalysis { .. })
        ));
    }

    #[test]
    fn incomplete_reasons_are_preserved() {
        let mut requirements = AiRequirements::new();

        requirements
            .mark_incomplete("unresolved capability")
            .expect("valid reason");

        requirements
            .mark_incomplete("unresolved resource expression")
            .expect("valid reason");

        match requirements.status() {
            RequirementAnalysisStatus::Incomplete { reasons } => {
                assert_eq!(reasons.len(), 2);
            }

            RequirementAnalysisStatus::Complete => {
                panic!("expected incomplete analysis");
            }
        }
    }

    #[test]
    fn merge_preserves_incomplete_analysis() {
        let mut left = AiRequirements::new();
        let mut right = AiRequirements::new();

        right
            .mark_incomplete("unresolved AI capability")
            .expect("valid reason");

        left.merge(right);

        assert!(!left.is_complete());

        assert!(matches!(
            left.status(),
            RequirementAnalysisStatus::Incomplete { reasons }
                if reasons.len() == 1
        ));
    }

    #[test]
    fn symbolic_values_are_not_silently_evaluated() {
        let value = RequirementValue::Symbolic(
            "dynamic_batch_dimension".to_owned(),
        );

        assert!(value.validate().is_ok());

        assert_eq!(
            value,
            RequirementValue::Symbolic(
                "dynamic_batch_dimension".to_owned()
            )
        );
    }

    #[test]
    fn feasibility_is_not_assumed() {
        assert_eq!(
            FeasibilityStatus::NotEvaluated,
            FeasibilityStatus::NotEvaluated
        );
    }
}
