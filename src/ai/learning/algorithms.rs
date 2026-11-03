
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Learning Algorithm Contracts
//!
//! Path: `src/ai/learning/algorithms.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines open-world, target-independent contracts for learning algorithms.
//!
//! This module describes:
//!
//! - Learning algorithm identities and versions.
//! - Algorithm families and semantic classifications.
//! - Algorithm input and output contracts.
//! - Algorithm parameter declarations.
//! - Algorithm state semantics.
//! - Determinism declarations.
//! - Canonical AI requirements.
//! - Extensible algorithm metadata.
//! - Exact, versioned learning algorithm registration.
//!
//! It does not implement numerical algorithms or execute training.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Learning algorithm declarations.
//! - Learning algorithm contract validation.
//! - Learning algorithm signature validation.
//! - Learning algorithm parameter schemas.
//! - Learning algorithm-specific registry entries.
//! - Exact versioned lookup and registration.
//!
//! ## Does not own
//!
//! - Source syntax or AST construction.
//! - Semantic name resolution.
//! - The canonical type system.
//! - Universal operation identity.
//! - General AI operation registration.
//! - Tensor storage.
//! - Automatic differentiation.
//! - Objective and loss implementations.
//! - Optimizer execution.
//! - Numerical kernels.
//! - Training workflow execution.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling or placement.
//! - Distributed communication.
//! - Runtime execution.
//! - Backend-specific implementations.
//!
//! ## Canonical integration
//!
//! Frontend semantic names:
//!     `frontend::semantic::domains::ai::QualifiedName`
//!
//! AI requirements:
//!     `ai::requirements::AiRequirements`
//!
//! AI errors:
//!     `ai::error::AiError`
//!
//! Canonical version representation:
//!     `zuir::version::VersionTriple`
//!
//! Training syntax:
//!     `grammar/ai/training.g4`
//!
//! Learning syntax:
//!     `grammar/ai/learning.g4`
//!
//! Training workflow descriptions:
//!     `ai::learning::training`
//!
//! Learning-specific optimization:
//!     `ai::learning::optimization`
//!
//! Gradient interfaces:
//!     `ai::learning::gradients`
//!
//! Automatic differentiation:
//!     `ai::differentiation`
//!
//! Universal operation representation:
//!     `zuir`
//!
//! Actual execution:
//!     compiler / backend / runtime
//!
//! ## POCO-REAF
//!
//! Algorithm declarations describe computational meaning, not hardware.
//!
//! The same algorithm contract may be realized by different compatible
//! implementations on:
//!
//! - Embedded systems.
//! - CPUs.
//! - Multicore systems.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - AI accelerators.
//! - Quantum-assisted systems.
//! - Neuromorphic systems.
//! - Heterogeneous systems.
//! - Distributed systems.
//! - Future computational architectures.
//!
//! This module introduces no language-level maximum for:
//!
//! - Registered algorithms.
//! - Algorithm versions.
//! - Parameters.
//! - Inputs or outputs.
//! - Algorithm metadata.
//! - Model parameters.
//! - Training steps.
//! - Workers.
//! - Accelerators.
//! - Devices.
//! - Distributed nodes.
//! - Tensor dimensions or rank.
//!
//! Collection capacity is constrained by the actual implementation
//! resources and externally supplied operational policies.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important semantic distinction
//!
//! An algorithm declaration is not proof that an implementation exists.
//!
//! Registration establishes that a contract is known to the AI subsystem.
//! It does not establish:
//!
//! - Target support.
//! - Implementation availability.
//! - Numerical equivalence.
//! - Capability satisfaction.
//! - Resource feasibility.
//! - Authorization.
//! - Execution success.
//!
//! Those decisions belong to the appropriate downstream authorities.
//!
//! ## Registry policy
//!
//! Registration identity is:
//!
//!     (qualified algorithm name, exact algorithm version)
//!
//! Multiple versions may coexist.
//!
//! Lookup is exact. No automatic version substitution or implicit
//! selection of the newest version occurs.
//!
//! Batch registration is atomic.
//!
//! ## Freeze criteria
//!
//! - Algorithm names reuse canonical semantic names.
//! - Algorithm versions are independently typed.
//! - Algorithm families are open-world.
//! - Parameter and signature validation is deterministic.
//! - Requirements reuse the canonical AI requirements model.
//! - Incomplete requirements are never promoted to complete.
//! - Duplicate registrations are rejected.
//! - Batch registration is atomic.
//! - Exact version lookup is deterministic.
//! - No algorithm executes in this module.
//! - No hardware assumptions or arbitrary workload ceilings exist.
//! - No unsafe Rust is used.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::sync::Arc;

use crate::ai::error::{AiError, AiErrorKind};
use crate::ai::requirements::AiRequirements;

use crate::frontend::semantic::domains::ai::QualifiedName;

use crate::zuir::version::VersionTriple;

// ============================================================================
// CANONICAL IDENTITIES
// ============================================================================

/// Canonical identity of a learning algorithm.
///
/// This is an alias for the frontend's qualified semantic name.
///
/// It is deliberately not a second algorithm identity system.
///
/// Names are open-world. New learning methods do not require extending
/// a Rust enumeration or changing this module.
pub type LearningAlgorithmId = QualifiedName;

/// Canonical identity of an algorithm family.
///
/// A family is descriptive semantic metadata. It is not a closed list
/// of supported algorithm implementations.
pub type LearningAlgorithmFamily = QualifiedName;

// ============================================================================
// ALGORITHM VERSION
// ============================================================================

/// Version of an individual learning algorithm contract.
///
/// This is independent of:
///
/// - Zamani language version.
/// - AI API version.
/// - AI semantic version.
/// - AI operation-set version.
/// - Model schema version.
/// - ZUIR schema version.
/// - Backend version.
/// - Runtime version.
///
/// It reuses the canonical ZUIR version representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LearningAlgorithmVersion(VersionTriple);

impl LearningAlgorithmVersion {
    /// Construct an algorithm contract version.
    #[must_use]
    pub const fn new(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self(VersionTriple::new(major, minor, patch))
    }

    /// Construct from the canonical version representation.
    #[must_use]
    pub const fn from_components(
        version: VersionTriple,
    ) -> Self {
        Self(version)
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.0.major()
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.0.minor()
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.0.patch()
    }

    /// Return the canonical version representation.
    #[must_use]
    pub const fn components(self) -> VersionTriple {
        self.0
    }
}

impl fmt::Display for LearningAlgorithmVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major(),
            self.minor(),
            self.patch()
        )
    }
}

// ============================================================================
// OPEN-WORLD METADATA
// ============================================================================

/// Extensible metadata value used by learning algorithm declarations.
///
/// This is a metadata representation, not another expression language.
///
/// Symbolic values are references. They are not evaluated here.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AlgorithmMetadataValue {
    /// Boolean metadata.
    Boolean(bool),

    /// Signed integer metadata.
    Signed(i128),

    /// Unsigned integer metadata.
    Unsigned(u128),

    /// Exact numerical representation.
    ///
    /// The owning extension defines its interpretation.
    Number(String),

    /// Text metadata.
    Text(String),

    /// Qualified semantic identity.
    Identity(QualifiedName),

    /// Opaque symbolic expression reference.
    Symbolic(String),

    /// Ordered sequence of metadata values.
    Sequence(Vec<AlgorithmMetadataValue>),

    /// Deterministically ordered metadata object.
    Object(BTreeMap<String, AlgorithmMetadataValue>),
}

impl AlgorithmMetadataValue {
    /// Validate the local metadata representation.
    ///
    /// This does not evaluate expressions or establish the validity
    /// of extension-specific metadata schemas.
    ///
    /// Iterative traversal avoids introducing recursion-depth limits
    /// into metadata validation.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        let mut pending = vec![self];

        while let Some(value) = pending.pop() {
            match value {
                Self::Boolean(_)
                | Self::Signed(_)
                | Self::Unsigned(_) => {}

                Self::Number(number) => {
                    validate_identity_component(
                        "metadata number",
                        number,
                    )?;
                }

                Self::Text(text) => {
                    validate_text("metadata text", text)?;
                }

                Self::Identity(identity) => {
                    validate_qualified_name(identity)?;
                }

                Self::Symbolic(expression) => {
                    validate_identity_component(
                        "symbolic metadata reference",
                        expression,
                    )?;
                }

                Self::Sequence(values) => {
                    pending.extend(values.iter());
                }

                Self::Object(values) => {
                    for (key, nested) in values {
                        validate_identity_component(
                            "metadata key",
                            key,
                        )?;

                        pending.push(nested);
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// ALGORITHM DETERMINISM
// ============================================================================

/// Determinism contract declared by a learning algorithm.
///
/// This describes semantic expectations. It does not implement random
/// number generation, seed management, or deterministic execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AlgorithmDeterminism {
    /// The algorithm contract requires deterministic behavior.
    Deterministic,

    /// Determinism depends on an explicitly supplied seed or equivalent
    /// deterministic input.
    SeedControlled,

    /// The algorithm permits nondeterministic behavior.
    Nondeterministic,

    /// Determinism depends on the selected compatible implementation.
    ImplementationDependent,

    /// The declaration does not establish a determinism guarantee.
    Unspecified,
}

// ============================================================================
// ALGORITHM STATE MODEL
// ============================================================================

/// State semantics of a learning algorithm.
///
/// This describes the logical state contract, not physical state storage.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AlgorithmStateModel {
    /// The algorithm declares no persistent logical state.
    Stateless,

    /// The algorithm maintains logical state across operations.
    Stateful,

    /// State is maintained by an external semantic or runtime service.
    External,

    /// An extension-defined state model.
    Custom(QualifiedName),
}

impl AlgorithmStateModel {
    /// Validate the state declaration.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        if let Self::Custom(identity) = self {
            validate_qualified_name(identity)?;
        }

        Ok(())
    }
}

// ============================================================================
// PORT CARDINALITY
// ============================================================================

/// Cardinality of an algorithm input or output port.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AlgorithmPortCardinality {
    /// Exactly one value is associated with the port.
    One,

    /// The value may be absent.
    Optional,

    /// The port accepts or produces a sequence of values.
    Variadic,
}

// ============================================================================
// ALGORITHM PORT
// ============================================================================

/// A typed input or output port in an algorithm contract.
///
/// Type identities reuse canonical semantic qualified names.
///
/// This declaration contains no tensor storage or executable values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlgorithmPort {
    name: QualifiedName,
    value_type: QualifiedName,
    cardinality: AlgorithmPortCardinality,
    description: Option<String>,
}

impl AlgorithmPort {
    /// Construct an algorithm port.
    pub fn new(
        name: QualifiedName,
        value_type: QualifiedName,
        cardinality: AlgorithmPortCardinality,
    ) -> Result<Self, LearningAlgorithmError> {
        validate_qualified_name(&name)?;
        validate_qualified_name(&value_type)?;

        Ok(Self {
            name,
            value_type,
            cardinality,
            description: None,
        })
    }

    /// Attach a description to the port.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, LearningAlgorithmError> {
        let description = description.into();
        validate_text("port description", &description)?;

        self.description = Some(description);
        Ok(self)
    }

    /// Return the port identity.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the canonical semantic value-type identity.
    #[must_use]
    pub fn value_type(&self) -> &QualifiedName {
        &self.value_type
    }

    /// Return the port cardinality.
    #[must_use]
    pub const fn cardinality(&self) -> AlgorithmPortCardinality {
        self.cardinality
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Revalidate this port.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        validate_qualified_name(&self.name)?;
        validate_qualified_name(&self.value_type)?;

        if let Some(description) = &self.description {
            validate_text("port description", description)?;
        }

        Ok(())
    }
}

// ============================================================================
// ALGORITHM SIGNATURE
// ============================================================================

/// Input and output contract of a learning algorithm.
///
/// Port ordering is preserved because it may be significant to the
/// owning semantic operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AlgorithmSignature {
    inputs: Vec<AlgorithmPort>,
    outputs: Vec<AlgorithmPort>,
}

impl AlgorithmSignature {
    /// Construct a signature.
    #[must_use]
    pub fn new(
        inputs: Vec<AlgorithmPort>,
        outputs: Vec<AlgorithmPort>,
    ) -> Self {
        Self {
            inputs,
            outputs,
        }
    }

    /// Return input ports in declaration order.
    #[must_use]
    pub fn inputs(&self) -> &[AlgorithmPort] {
        &self.inputs
    }

    /// Return output ports in declaration order.
    #[must_use]
    pub fn outputs(&self) -> &[AlgorithmPort] {
        &self.outputs
    }

    /// Validate port declarations and uniqueness.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        let mut identities = BTreeSet::new();

        for port in self.inputs.iter().chain(self.outputs.iter()) {
            port.validate()?;

            if !identities.insert(port.name().clone()) {
                return Err(LearningAlgorithmError::DuplicatePort {
                    name: qualified_name_string(port.name()),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// ALGORITHM PARAMETERS
// ============================================================================

/// A declared algorithm parameter.
///
/// The parameter's type is a canonical semantic identity.
///
/// The parameter declaration does not contain a runtime parameter value,
/// tensor allocation, optimizer state, or hardware representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlgorithmParameter {
    name: QualifiedName,
    value_type: QualifiedName,
    required: bool,
    description: Option<String>,
}

impl AlgorithmParameter {
    /// Construct an algorithm parameter declaration.
    pub fn new(
        name: QualifiedName,
        value_type: QualifiedName,
        required: bool,
    ) -> Result<Self, LearningAlgorithmError> {
        validate_qualified_name(&name)?;
        validate_qualified_name(&value_type)?;

        Ok(Self {
            name,
            value_type,
            required,
            description: None,
        })
    }

    /// Attach a parameter description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, LearningAlgorithmError> {
        let description = description.into();
        validate_text("parameter description", &description)?;

        self.description = Some(description);
        Ok(self)
    }

    /// Return the parameter identity.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the canonical semantic value type.
    #[must_use]
    pub fn value_type(&self) -> &QualifiedName {
        &self.value_type
    }

    /// Return whether the parameter is mandatory.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the parameter declaration.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        validate_qualified_name(&self.name)?;
        validate_qualified_name(&self.value_type)?;

        if let Some(description) = &self.description {
            validate_text("parameter description", description)?;
        }

        Ok(())
    }
}

// ============================================================================
// LEARNING ALGORITHM DESCRIPTOR
// ============================================================================

/// Immutable semantic declaration of a learning algorithm.
///
/// The descriptor records what an algorithm contract requires and
/// guarantees. It does not contain executable implementation code.
///
/// An algorithm implementation may be supplied by a library, compiler
/// extension, dialect, or backend. The responsible integration layer
/// must establish that the implementation preserves this contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningAlgorithmDescriptor {
    identity: LearningAlgorithmId,
    version: LearningAlgorithmVersion,
    family: LearningAlgorithmFamily,
    signature: AlgorithmSignature,
    parameters: Vec<AlgorithmParameter>,
    requirements: AiRequirements,
    determinism: AlgorithmDeterminism,
    state_model: AlgorithmStateModel,
    description: Option<String>,
    metadata: BTreeMap<String, AlgorithmMetadataValue>,
}

impl LearningAlgorithmDescriptor {
    /// Construct a learning algorithm declaration.
    ///
    /// The algorithm family is an open-world qualified identity.
    ///
    /// No built-in algorithm list is required.
    pub fn new(
        identity: LearningAlgorithmId,
        version: LearningAlgorithmVersion,
        family: LearningAlgorithmFamily,
        signature: AlgorithmSignature,
    ) -> Result<Self, LearningAlgorithmError> {
        validate_qualified_name(&identity)?;
        validate_qualified_name(&family)?;
        signature.validate()?;

        Ok(Self {
            identity,
            version,
            family,
            signature,
            parameters: Vec::new(),
            requirements: AiRequirements::new(),
            determinism: AlgorithmDeterminism::Unspecified,
            state_model: AlgorithmStateModel::Stateless,
            description: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Replace the algorithm's canonical AI requirement envelope.
    ///
    /// Incomplete requirements are retained as incomplete and rejected
    /// during descriptor validation.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Set the determinism declaration.
    #[must_use]
    pub fn with_determinism(
        mut self,
        determinism: AlgorithmDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Set the logical state model.
    #[must_use]
    pub fn with_state_model(
        mut self,
        state_model: AlgorithmStateModel,
    ) -> Self {
        self.state_model = state_model;
        self
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, LearningAlgorithmError> {
        let description = description.into();
        validate_text("algorithm description", &description)?;

        self.description = Some(description);
        Ok(self)
    }

    /// Add a parameter declaration.
    ///
    /// Duplicate parameter identities are rejected.
    pub fn add_parameter(
        &mut self,
        parameter: AlgorithmParameter,
    ) -> Result<(), LearningAlgorithmError> {
        parameter.validate()?;

        if self.parameters
            .iter()
            .any(|existing| existing.name() == parameter.name())
        {
            return Err(LearningAlgorithmError::DuplicateParameter {
                name: qualified_name_string(parameter.name()),
            });
        }

        self.parameters.push(parameter);
        Ok(())
    }

    /// Add an extensible metadata entry.
    ///
    /// Existing keys are not silently overwritten.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: AlgorithmMetadataValue,
    ) -> Result<(), LearningAlgorithmError> {
        let key = key.into();

        validate_identity_component("metadata key", &key)?;
        value.validate()?;

        if self.metadata.contains_key(&key) {
            return Err(LearningAlgorithmError::DuplicateMetadata {
                key,
            });
        }

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Explicitly replace or insert a metadata entry.
    ///
    /// Replacement is visible to the caller through the returned value.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: AlgorithmMetadataValue,
    ) -> Result<Option<AlgorithmMetadataValue>, LearningAlgorithmError> {
        let key = key.into();

        validate_identity_component("metadata key", &key)?;
        value.validate()?;

        Ok(self.metadata.insert(key, value))
    }

    /// Return the algorithm identity.
    #[must_use]
    pub fn identity(&self) -> &LearningAlgorithmId {
        &self.identity
    }

    /// Return the exact algorithm contract version.
    #[must_use]
    pub const fn version(&self) -> LearningAlgorithmVersion {
        self.version
    }

    /// Return the algorithm family.
    #[must_use]
    pub fn family(&self) -> &LearningAlgorithmFamily {
        &self.family
    }

    /// Return the algorithm signature.
    #[must_use]
    pub fn signature(&self) -> &AlgorithmSignature {
        &self.signature
    }

    /// Return parameter declarations in declaration order.
    #[must_use]
    pub fn parameters(&self) -> &[AlgorithmParameter] {
        &self.parameters
    }

    /// Return canonical AI requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Return the determinism contract.
    #[must_use]
    pub const fn determinism(&self) -> AlgorithmDeterminism {
        self.determinism
    }

    /// Return the logical state model.
    #[must_use]
    pub fn state_model(&self) -> &AlgorithmStateModel {
        &self.state_model
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return deterministic extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, AlgorithmMetadataValue> {
        &self.metadata
    }

    /// Validate every locally owned algorithm contract invariant.
    ///
    /// This does not establish implementation availability, semantic
    /// compatibility with a particular model, or target feasibility.
    pub fn validate(&self) -> Result<(), LearningAlgorithmError> {
        validate_qualified_name(&self.identity)?;
        validate_qualified_name(&self.family)?;

        self.signature.validate()?;
        self.state_model.validate()?;

        if !self.requirements.is_complete() {
            return Err(
                LearningAlgorithmError::IncompleteRequirements
            );
        }

        if let Some(description) = &self.description {
            validate_text("algorithm description", description)?;
        }

        let mut parameter_names = BTreeSet::new();

        for parameter in &self.parameters {
            parameter.validate()?;

            if !parameter_names.insert(parameter.name().clone()) {
                return Err(LearningAlgorithmError::DuplicateParameter {
                    name: qualified_name_string(parameter.name()),
                });
            }
        }

        for (key, value) in &self.metadata {
            validate_identity_component("metadata key", key)?;
            value.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// REGISTRY IDENTITY
// ============================================================================

/// Exact identity of one registered algorithm contract.
///
/// Different versions of the same algorithm are independent entries.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct LearningAlgorithmKey {
    identity: LearningAlgorithmId,
    version: LearningAlgorithmVersion,
}

impl LearningAlgorithmKey {
    fn from_descriptor(
        descriptor: &LearningAlgorithmDescriptor,
    ) -> Self {
        Self {
            identity: descriptor.identity.clone(),
            version: descriptor.version,
        }
    }
}

// ============================================================================
// REGISTRY ERRORS
// ============================================================================

/// Errors produced by learning algorithm declarations and registration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LearningAlgorithmError {
    /// A qualified identity is invalid.
    InvalidIdentity {
        /// The identity component.
        field: &'static str,
    },

    /// A textual metadata value is invalid.
    InvalidText {
        /// The affected field.
        field: &'static str,
    },

    /// An algorithm signature contains duplicate port identities.
    DuplicatePort {
        /// Duplicate port identity.
        name: String,
    },

    /// An algorithm declares a parameter more than once.
    DuplicateParameter {
        /// Duplicate parameter identity.
        name: String,
    },

    /// Metadata contains a duplicate key.
    DuplicateMetadata {
        /// Duplicate metadata key.
        key: String,
    },

    /// The algorithm's requirements analysis is incomplete.
    IncompleteRequirements,

    /// The exact algorithm identity and version are already registered.
    DuplicateAlgorithm {
        /// Qualified algorithm identity.
        identity: String,

        /// Exact algorithm version.
        version: LearningAlgorithmVersion,
    },

    /// The exact requested algorithm identity and version are not registered.
    UnknownAlgorithm {
        /// Qualified algorithm identity.
        identity: String,

        /// Exact requested algorithm version.
        version: LearningAlgorithmVersion,
    },
}

impl fmt::Display for LearningAlgorithmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { field } => {
                write!(formatter, "invalid learning algorithm {field}")
            }

            Self::InvalidText { field } => {
                write!(formatter, "invalid learning algorithm {field}")
            }

            Self::DuplicatePort { name } => {
                write!(formatter, "duplicate algorithm port {name}")
            }

            Self::DuplicateParameter { name } => {
                write!(formatter, "duplicate algorithm parameter {name}")
            }

            Self::DuplicateMetadata { key } => {
                write!(formatter, "duplicate algorithm metadata key {key}")
            }

            Self::IncompleteRequirements => {
                formatter.write_str(
                    "learning algorithm requirements analysis is incomplete"
                )
            }

            Self::DuplicateAlgorithm { identity, version } => {
                write!(
                    formatter,
                    "learning algorithm {identity} version {version} is already registered"
                )
            }

            Self::UnknownAlgorithm { identity, version } => {
                write!(
                    formatter,
                    "learning algorithm {identity} version {version} is not registered"
                )
            }
        }
    }
}

impl Error for LearningAlgorithmError {}

impl From<LearningAlgorithmError> for AiError {
    fn from(error: LearningAlgorithmError) -> Self {
        let kind = match error {
            LearningAlgorithmError::DuplicateAlgorithm { .. }
            | LearningAlgorithmError::DuplicatePort { .. }
            | LearningAlgorithmError::DuplicateParameter { .. }
            | LearningAlgorithmError::DuplicateMetadata { .. } => {
                AiErrorKind::Conflict
            }

            LearningAlgorithmError::UnknownAlgorithm { .. } => {
                AiErrorKind::NotFound
            }

            LearningAlgorithmError::IncompleteRequirements => {
                AiErrorKind::RequirementUnsatisfied
            }

            LearningAlgorithmError::InvalidIdentity { .. }
            | LearningAlgorithmError::InvalidText { .. } => {
                AiErrorKind::InvalidOperation
            }
        };

        AiError::from_source(
            kind,
            error.to_string(),
            error,
        )
    }
}

// ============================================================================
// LEARNING ALGORITHM REGISTRY
// ============================================================================

/// Deterministic registry of learning algorithm declarations.
///
/// This registry stores immutable semantic declarations. It does not
/// load executable plugins or invoke algorithm implementations.
///
/// Registration and lookup are exact and version-aware.
///
/// Cloning a registry produces an independent index containing shared
/// immutable declaration snapshots.
#[derive(Clone, Debug, Default)]
pub struct LearningAlgorithmRegistry {
    algorithms: BTreeMap<
        LearningAlgorithmKey,
        Arc<LearningAlgorithmDescriptor>,
    >,
}

impl LearningAlgorithmRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of exact algorithm registrations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.algorithms.len()
    }

    /// Return whether the registry contains no registrations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.algorithms.is_empty()
    }

    /// Register one validated algorithm descriptor.
    ///
    /// Exact duplicate registrations are rejected.
    ///
    /// The returned `Arc` is an immutable snapshot.
    pub fn register(
        &mut self,
        descriptor: LearningAlgorithmDescriptor,
    ) -> Result<Arc<LearningAlgorithmDescriptor>, LearningAlgorithmError> {
        descriptor.validate()?;

        let key = LearningAlgorithmKey::from_descriptor(&descriptor);

        if self.algorithms.contains_key(&key) {
            return Err(LearningAlgorithmError::DuplicateAlgorithm {
                identity: qualified_name_string(&descriptor.identity),
                version: descriptor.version,
            });
        }

        let descriptor = Arc::new(descriptor);

        self.algorithms.insert(
            key,
            Arc::clone(&descriptor),
        );

        Ok(descriptor)
    }

    /// Atomically register multiple algorithm descriptors.
    ///
    /// All descriptors are validated against a staged registry index.
    /// The original registry remains unchanged if any entry fails.
    ///
    /// An empty batch succeeds without changing the registry.
    pub fn register_batch<I>(
        &mut self,
        descriptors: I,
    ) -> Result<(), LearningAlgorithmError>
    where
        I: IntoIterator<Item = LearningAlgorithmDescriptor>,
    {
        let mut staged = self.algorithms.clone();

        for descriptor in descriptors {
            descriptor.validate()?;

            let key = LearningAlgorithmKey::from_descriptor(&descriptor);

            if staged.contains_key(&key) {
                return Err(LearningAlgorithmError::DuplicateAlgorithm {
                    identity: qualified_name_string(&descriptor.identity),
                    version: descriptor.version,
                });
            }

            staged.insert(
                key,
                Arc::new(descriptor),
            );
        }

        self.algorithms = staged;

        Ok(())
    }

    /// Resolve an exact algorithm identity and version.
    ///
    /// No version substitution or automatic upgrade occurs.
    pub fn resolve(
        &self,
        identity: &LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> Result<Arc<LearningAlgorithmDescriptor>, LearningAlgorithmError> {
        self.get(identity, version)
            .cloned()
            .ok_or_else(|| LearningAlgorithmError::UnknownAlgorithm {
                identity: qualified_name_string(identity),
                version,
            })
    }

    /// Look up an exact algorithm registration.
    #[must_use]
    pub fn get(
        &self,
        identity: &LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> Option<&Arc<LearningAlgorithmDescriptor>> {
        self.algorithms.get(&LearningAlgorithmKey {
            identity: identity.clone(),
            version,
        })
    }

    /// Return whether an exact algorithm registration exists.
    #[must_use]
    pub fn contains(
        &self,
        identity: &LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> bool {
        self.algorithms.contains_key(&LearningAlgorithmKey {
            identity: identity.clone(),
            version,
        })
    }

    /// Iterate over all descriptors in deterministic order.
    ///
    /// Ordering is by qualified identity and then exact version.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Arc<LearningAlgorithmDescriptor>> {
        self.algorithms.values()
    }

    /// Iterate over all registered versions of one algorithm.
    pub fn versions_for<'a>(
        &'a self,
        identity: &'a LearningAlgorithmId,
    ) -> impl Iterator<Item = LearningAlgorithmVersion> + 'a {
        self.algorithms
            .keys()
            .filter(move |key| &key.identity == identity)
            .map(|key| key.version)
    }

    /// Remove one exact registration.
    ///
    /// Previously returned `Arc` snapshots remain valid.
    pub fn remove(
        &mut self,
        identity: &LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> Option<Arc<LearningAlgorithmDescriptor>> {
        self.algorithms.remove(&LearningAlgorithmKey {
            identity: identity.clone(),
            version,
        })
    }

    /// Return an immutable snapshot of the current registry.
    #[must_use]
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_identity_component(
    field: &'static str,
    value: &str,
) -> Result<(), LearningAlgorithmError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(LearningAlgorithmError::InvalidIdentity {
            field,
        });
    }

    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), LearningAlgorithmError> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(LearningAlgorithmError::InvalidText {
            field,
        });
    }

    Ok(())
}

fn validate_qualified_name(
    name: &QualifiedName,
) -> Result<(), LearningAlgorithmError> {
    validate_identity_component("namespace", &name.namespace)?;
    validate_identity_component("name", &name.name)?;

    Ok(())
}

fn qualified_name_string(
    name: &QualifiedName,
) -> String {
    format!("{}::{}", name.namespace, name.name)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(namespace: &str, value: &str) -> QualifiedName {
        QualifiedName::new(namespace, value)
            .expect("valid qualified name")
    }

    fn version(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> LearningAlgorithmVersion {
        LearningAlgorithmVersion::new(
            major,
            minor,
            patch,
        )
    }

    fn descriptor(
        algorithm: &str,
        algorithm_version: LearningAlgorithmVersion,
    ) -> LearningAlgorithmDescriptor {
        LearningAlgorithmDescriptor::new(
            name("zamani.ai.learning", algorithm),
            algorithm_version,
            name("zamani.ai.learning.family", "general"),
            AlgorithmSignature::default(),
        )
        .expect("valid algorithm descriptor")
    }

    #[test]
    fn algorithm_identity_reuses_canonical_qualified_name() {
        let identity: LearningAlgorithmId =
            name("future.ai.learning", "new_method");

        assert_eq!(identity.namespace, "future.ai.learning");
        assert_eq!(identity.name, "new_method");
    }

    #[test]
    fn algorithm_versions_are_independent_and_ordered() {
        let first = version(1, 0, 0);
        let second = version(1, 1, 0);

        assert!(first < second);
        assert_eq!(first.to_string(), "1.0.0");
        assert_eq!(second.to_string(), "1.1.0");
    }

    #[test]
    fn arbitrary_algorithm_families_are_supported() {
        let algorithm = LearningAlgorithmDescriptor::new(
            name("future.photonic.learning", "coherent_update"),
            version(1, 0, 0),
            name("future.photonic", "coherent_learning"),
            AlgorithmSignature::default(),
        )
        .expect("open-world family is valid");

        assert_eq!(
            algorithm.family().namespace,
            "future.photonic"
        );
    }

    #[test]
    fn signature_preserves_input_and_output_order() {
        let first = AlgorithmPort::new(
            name("port", "data"),
            name("type", "tensor"),
            AlgorithmPortCardinality::One,
        )
        .expect("valid port");

        let second = AlgorithmPort::new(
            name("port", "objective"),
            name("type", "scalar"),
            AlgorithmPortCardinality::One,
        )
        .expect("valid port");

        let signature = AlgorithmSignature::new(
            vec![first.clone(), second.clone()],
            vec![second],
        );

        // Port identities must be unique across the complete signature.
        assert!(matches!(
            signature.validate(),
            Err(LearningAlgorithmError::DuplicatePort { .. })
        ));

        let valid = AlgorithmSignature::new(
            vec![first],
            vec![
                AlgorithmPort::new(
                    name("port", "updated_model"),
                    name("type", "model"),
                    AlgorithmPortCardinality::One,
                )
                .expect("valid output"),
            ],
        );

        assert!(valid.validate().is_ok());
    }

    #[test]
    fn duplicate_parameters_are_rejected() {
        let mut algorithm = descriptor(
            "custom_method",
            version(1, 0, 0),
        );

        let parameter = AlgorithmParameter::new(
            name("parameter", "rate"),
            name("type", "real"),
            true,
        )
        .expect("valid parameter");

        algorithm
            .add_parameter(parameter.clone())
            .expect("first parameter");

        assert!(matches!(
            algorithm.add_parameter(parameter),
            Err(LearningAlgorithmError::DuplicateParameter { .. })
        ));
    }

    #[test]
    fn parameter_type_is_a_canonical_semantic_identity() {
        let parameter = AlgorithmParameter::new(
            name("parameter", "regularization"),
            name("numeric", "real"),
            false,
        )
        .expect("valid parameter");

        assert_eq!(
            parameter.value_type().namespace,
            "numeric"
        );
    }

    #[test]
    fn incomplete_requirements_cannot_be_registered() {
        let mut requirements = AiRequirements::new();

        requirements
            .mark_incomplete("unresolved algorithm requirement")
            .expect("valid incomplete reason");

        let algorithm = descriptor(
            "incomplete_method",
            version(1, 0, 0),
        )
        .with_requirements(requirements);

        assert_eq!(
            algorithm.validate(),
            Err(LearningAlgorithmError::IncompleteRequirements)
        );
    }

    #[test]
    fn metadata_rejects_duplicate_keys() {
        let mut algorithm = descriptor(
            "metadata_method",
            version(1, 0, 0),
        );

        algorithm
            .add_metadata(
                "domain",
                AlgorithmMetadataValue::Text("statistical".into()),
            )
            .expect("first metadata value");

        assert!(matches!(
            algorithm.add_metadata(
                "domain",
                AlgorithmMetadataValue::Text("symbolic".into()),
            ),
            Err(LearningAlgorithmError::DuplicateMetadata { .. })
        ));
    }

    #[test]
    fn metadata_rejects_invalid_nested_values() {
        let invalid = AlgorithmMetadataValue::Object(
            [(
                "valid_key".to_owned(),
                AlgorithmMetadataValue::Symbolic(String::new()),
            )]
            .into_iter()
            .collect(),
        );

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn metadata_supports_nested_open_world_values() {
        let value = AlgorithmMetadataValue::Object(
            [(
                "future_property".to_owned(),
                AlgorithmMetadataValue::Sequence(vec![
                    AlgorithmMetadataValue::Boolean(true),
                    AlgorithmMetadataValue::Identity(
                        name("future.domain", "identity"),
                    ),
                    AlgorithmMetadataValue::Number(
                        "1.234567890123456789".to_owned(),
                    ),
                ]),
            )]
            .into_iter()
            .collect(),
        );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn registry_starts_empty() {
        let registry = LearningAlgorithmRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_registers_and_resolves_exact_version() {
        let mut registry = LearningAlgorithmRegistry::new();

        let algorithm = descriptor(
            "custom_method",
            version(1, 0, 0),
        );

        registry
            .register(algorithm)
            .expect("registration succeeds");

        let resolved = registry
            .resolve(
                &name("zamani.ai.learning", "custom_method"),
                version(1, 0, 0),
            )
            .expect("exact version exists");

        assert_eq!(resolved.version(), version(1, 0, 0));
    }

    #[test]
    fn registry_does_not_substitute_versions() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register(descriptor(
                "custom_method",
                version(1, 0, 0),
            ))
            .expect("registration succeeds");

        assert!(registry
            .resolve(
                &name("zamani.ai.learning", "custom_method"),
                version(2, 0, 0),
            )
            .is_err());
    }

    #[test]
    fn different_versions_can_coexist() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register(descriptor(
                "custom_method",
                version(1, 0, 0),
            ))
            .expect("first version");

        registry
            .register(descriptor(
                "custom_method",
                version(2, 0, 0),
            ))
            .expect("second version");

        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn duplicate_exact_registration_is_rejected() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register(descriptor(
                "custom_method",
                version(1, 0, 0),
            ))
            .expect("first registration");

        assert!(matches!(
            registry.register(descriptor(
                "custom_method",
                version(1, 0, 0),
            )),
            Err(LearningAlgorithmError::DuplicateAlgorithm { .. })
        ));
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register(descriptor(
                "existing",
                version(1, 0, 0),
            ))
            .expect("initial registration");

        let result = registry.register_batch(vec![
            descriptor("new_algorithm", version(1, 0, 0)),
            descriptor("existing", version(1, 0, 0)),
        ]);

        assert!(result.is_err());

        // The staged first entry must not leak into the live registry.
        assert_eq!(registry.len(), 1);

        assert!(!registry.contains(
            &name("zamani.ai.learning", "new_algorithm"),
            version(1, 0, 0),
        ));
    }

    #[test]
    fn batch_registration_accepts_distinct_versions() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register_batch(vec![
                descriptor("method", version(1, 0, 0)),
                descriptor("method", version(2, 0, 0)),
            ])
            .expect("distinct versions");

        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn versions_are_returned_in_deterministic_order() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register_batch(vec![
                descriptor("method", version(2, 0, 0)),
                descriptor("method", version(1, 0, 0)),
                descriptor("method", version(1, 1, 0)),
            ])
            .expect("registrations");

        let versions: Vec<_> = registry
            .versions_for(&name("zamani.ai.learning", "method"))
            .collect();

        assert_eq!(
            versions,
            vec![
                version(1, 0, 0),
                version(1, 1, 0),
                version(2, 0, 0),
            ]
        );
    }

    #[test]
    fn immutable_snapshots_survive_removal() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register(descriptor(
                "method",
                version(1, 0, 0),
            ))
            .expect("registration");

        let snapshot = registry
            .resolve(
                &name("zamani.ai.learning", "method"),
                version(1, 0, 0),
            )
            .expect("snapshot");

        registry.remove(
            &name("zamani.ai.learning", "method"),
            version(1, 0, 0),
        );

        assert_eq!(snapshot.version(), version(1, 0, 0));
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register_batch(vec![
                descriptor("z_method", version(1, 0, 0)),
                descriptor("a_method", version(1, 0, 0)),
            ])
            .expect("registrations");

        let names: Vec<_> = registry
            .iter()
            .map(|entry| entry.identity().name.clone())
            .collect();

        assert_eq!(names, vec!["a_method", "z_method"]);
    }

    #[test]
    fn removal_affects_only_the_exact_version() {
        let mut registry = LearningAlgorithmRegistry::new();

        registry
            .register_batch(vec![
                descriptor("method", version(1, 0, 0)),
                descriptor("method", version(2, 0, 0)),
            ])
            .expect("registrations");

        registry.remove(
            &name("zamani.ai.learning", "method"),
            version(1, 0, 0),
        );

        assert!(!registry.contains(
            &name("zamani.ai.learning", "method"),
            version(1, 0, 0),
        ));

        assert!(registry.contains(
            &name("zamani.ai.learning", "method"),
            version(2, 0, 0),
        ));
    }

    #[test]
    fn algorithm_errors_integrate_with_ai_error_contract() {
        let error = AiError::from(
            LearningAlgorithmError::IncompleteRequirements
        );

        assert_eq!(
            error.kind(),
            AiErrorKind::RequirementUnsatisfied
        );
    }
}
