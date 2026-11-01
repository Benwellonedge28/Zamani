
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Signatures
//!
//! File:
//!     src/ai/model/signatures.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Define target-independent input and output contracts for AI models.
//!
//! A model signature describes:
//!
//! - what information a model accepts;
//! - what information a model produces;
//! - the canonical semantic type of every port;
//! - required and optional input semantics;
//! - port cardinality;
//! - default expression references;
//! - the relationships between model interfaces.
//!
//! Signatures describe computational meaning, not physical execution.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! Source syntax:
//!     grammar/ai/models.g4
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! Canonical type compatibility:
//!     crate::frontend::semantic::types::compatibility
//!
//! Model definitions:
//!     crate::ai::model::definition
//!
//! Model operations and reusable typed ports:
//!     crate::ai::model::operations
//!
//! AI semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - ModelSignature.
//! - ModelSignatureMetadata.
//! - Model-level signature composition.
//! - Signature structural validation.
//! - Signature port lookup.
//! - Signature compatibility coordination.
//! - Explicit compatibility outcomes.
//! - Signature-specific validation errors.
//!
//! ============================================================================
//! DOES NOT OWN
//! ============================================================================
//!
//! - Source grammar.
//! - AST construction.
//! - SemanticType definitions.
//! - Type inference.
//! - Type unification.
//! - Generic substitution.
//! - Tensor storage.
//! - Tensor shape authority.
//! - Model graph execution.
//! - Model parameter allocation.
//! - Training.
//! - Inference.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling.
//! - Backend implementation.
//! - ZUIR operation definitions.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! No fixed maximum number of:
//!
//! - model inputs;
//! - model outputs;
//! - ports;
//! - structured fields;
//! - generic parameters;
//! - tensor dimensions;
//! - model members;
//! - models;
//! - execution targets;
//! - accelerators.
//!
//! Collection sizes are representation and operational concerns.
//!
//! Operational budgets must be supplied by the compiler or execution
//! environment. They must not silently alter model signature semantics.
//!
//! Literal infinite execution is not promised. Execution remains subject
//! to addressability, available resources and explicit operational policies.
//!
//! ============================================================================
//! TYPE COMPATIBILITY
//! ============================================================================
//!
//! This module deliberately delegates semantic type compatibility.
//!
//! Exact Rust equality is sufficient to establish identical type structures,
//! but unequal structures are not automatically incompatible.
//!
//! The canonical semantic compatibility service must determine whether
//! one type can safely substitute for another.
//!
//! Unknown compatibility remains explicitly unknown.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! model/definition.rs:
//!     Model declarations may reference a ModelSignature.
//!
//! model/operations.rs:
//!     Supplies canonical reusable operation ports and port validation.
//!
//! model/layers.rs:
//!     Layer interfaces may reuse model signature contracts.
//!
//! model/composition.rs:
//!     Uses compatibility checking when composing model interfaces.
//!
//! model/validation.rs:
//!     Coordinates signature validation with model structural validation.
//!
//! frontend/semantic/domains/ai.rs:
//!     Validates source-derived semantic relationships.
//!
//! frontend/semantic/types/compatibility.rs:
//!     Authoritative semantic type compatibility.
//!
//! zuir/dialects/ai:
//!     Converts validated signature information into canonical IR contracts.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! - Model signatures reuse canonical semantic types.
//! - Model signatures reuse canonical operation ports.
//! - Required and optional input rules are validated.
//! - Input and output identities are unambiguous.
//! - Compatibility does not guess unresolved type relationships.
//! - Generic and symbolic types remain representable.
//! - Structured types remain representable.
//! - No hardware-dependent limits are introduced.
//! - Frontend and ZUIR adapters consume this contract.
//! - Workspace compilation and tests pass.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::ai::model::operations::{
    ModelOperationPort,
    ModelOperationPortDirection,
    ModelOperationSignature,
};

use crate::frontend::semantic::types::ty::{
    SemanticName,
    SemanticType,
};

// ============================================================================
// COMPATIBILITY RESULT
// ============================================================================

/// Result of checking whether a produced semantic type can satisfy
/// an accepted semantic type.
///
/// This is intentionally not a boolean.
///
/// An unresolved result must never be interpreted as success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TypeCompatibility {
    /// The canonical type system established compatibility.
    Compatible,

    /// The canonical type system established incompatibility.
    Incompatible,

    /// The canonical type system could not establish either result.
    ///
    /// Examples include unresolved generic variables, unresolved aliases,
    /// or incomplete constraint solving.
    Undetermined,
}

// ============================================================================
// CANONICAL TYPE COMPATIBILITY INTERFACE
// ============================================================================

/// Adapter to the repository's authoritative semantic type compatibility
/// implementation.
///
/// Implementations must delegate to the canonical semantic type system.
///
/// The produced type is checked against the accepted type.
///
/// This direction matters: compatibility is not necessarily symmetric.
pub trait SemanticTypeCompatibility {
    /// Check whether `produced` can safely satisfy `accepted`.
    ///
    /// Implementations must account for the language's:
    ///
    /// - nominal typing;
    /// - structural typing;
    /// - generic substitutions;
    /// - type aliases;
    /// - optionality;
    /// - union and intersection types;
    /// - constraints;
    /// - variance rules;
    /// - domain-specific type rules.
    fn is_compatible(
        &self,
        produced: &SemanticType,
        accepted: &SemanticType,
    ) -> TypeCompatibility;
}

// ============================================================================
// SIGNATURE COMPATIBILITY ISSUE
// ============================================================================

/// A specific compatibility issue between two model signatures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureCompatibilityIssue {
    /// An output produced by one model has no matching input in the
    /// receiving model.
    MissingAcceptedInput {
        name: SemanticName,
    },

    /// A receiving model requires an input that the producing model
    /// cannot guarantee to supply.
    MissingRequiredInput {
        name: SemanticName,
    },

    /// The semantic types are incompatible.
    IncompatibleType {
        name: SemanticName,
    },

    /// Type compatibility could not be established.
    UndeterminedType {
        name: SemanticName,
    },

    /// The output cardinality cannot satisfy the receiving input.
    IncompatibleCardinality {
        name: SemanticName,
    },
}

/// Result of a model signature compatibility operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignatureCompatibilityReport {
    /// Whether all checks completed.
    ///
    /// False means at least one required semantic check was unresolved.
    pub complete: bool,

    /// Compatibility issues in deterministic order.
    pub issues: Vec<SignatureCompatibilityIssue>,
}

impl SignatureCompatibilityReport {
    /// Returns true only when all checks completed and no incompatibility
    /// was found.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.complete && self.issues.is_empty()
    }

    /// Returns true if at least one definitive incompatibility was found.
    #[must_use]
    pub fn has_incompatibilities(&self) -> bool {
        self.issues.iter().any(|issue| {
            matches!(
                issue,
                SignatureCompatibilityIssue::MissingAcceptedInput { .. }
                    | SignatureCompatibilityIssue::MissingRequiredInput { .. }
                    | SignatureCompatibilityIssue::IncompatibleType { .. }
                    | SignatureCompatibilityIssue::IncompatibleCardinality { .. }
            )
        })
    }

    /// Returns true if at least one unresolved compatibility check exists.
    #[must_use]
    pub fn is_undetermined(&self) -> bool {
        !self.complete
            || self.issues.iter().any(|issue| {
                matches!(
                    issue,
                    SignatureCompatibilityIssue::UndeterminedType { .. }
                )
            })
    }
}

// ============================================================================
// SIGNATURE METADATA
// ============================================================================

/// Descriptive metadata associated with a model signature.
///
/// This metadata does not redefine semantic types or port behavior.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelSignatureMetadata {
    /// Optional documentation describing the interface.
    pub documentation: Option<String>,

    /// Optional stable schema identity.
    ///
    /// Assigned by the canonical semantic adapter or model registry.
    pub schema_identity: Option<String>,

    /// Extension-defined metadata.
    ///
    /// The caller is responsible for assigning meaning to extension keys
    /// through the appropriate registry.
    pub extensions: Vec<(String, String)>,
}

impl ModelSignatureMetadata {
    /// Creates empty signature metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets documentation.
    #[must_use]
    pub fn with_documentation(
        mut self,
        documentation: impl Into<String>,
    ) -> Self {
        self.documentation = Some(documentation.into());
        self
    }

    /// Sets a schema identity.
    #[must_use]
    pub fn with_schema_identity(
        mut self,
        identity: impl Into<String>,
    ) -> Self {
        self.schema_identity = Some(identity.into());
        self
    }

    /// Adds extension metadata.
    #[must_use]
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.extensions.push((key.into(), value.into()));
        self
    }

    /// Validates metadata structure.
    pub fn validate(&self) -> Result<(), ModelSignatureError> {
        if let Some(identity) = &self.schema_identity {
            validate_text(identity)
                .map_err(|_| ModelSignatureError::InvalidSchemaIdentity)?;
        }

        let mut keys = BTreeSet::new();

        for (key, _) in &self.extensions {
            validate_text(key)
                .map_err(|_| ModelSignatureError::InvalidMetadataKey)?;

            if !keys.insert(key) {
                return Err(ModelSignatureError::DuplicateMetadataKey);
            }
        }

        Ok(())
    }
}

// ============================================================================
// MODEL SIGNATURE
// ============================================================================

/// Target-independent model input/output signature.
///
/// The actual port contracts are delegated to
/// `ModelOperationSignature`.
///
/// This avoids creating a second model-port representation.
///
/// Structured values are expressed using canonical SemanticType variants,
/// including:
///
/// - Tuple.
/// - Named.
/// - Array.
/// - Slice.
/// - Optional.
/// - Union.
/// - Intersection.
/// - Function.
/// - Generic Parameter.
///
/// Tensor types remain canonical named semantic types and are not
/// reimplemented here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelSignature {
    /// Canonical model input/output ports.
    pub ports: ModelOperationSignature,

    /// Interface metadata.
    pub metadata: ModelSignatureMetadata,
}

impl ModelSignature {
    /// Creates a model signature.
    ///
    /// Port order is preserved.
    #[must_use]
    pub fn new(
        inputs: Vec<ModelOperationPort>,
        outputs: Vec<ModelOperationPort>,
    ) -> Self {
        Self {
            ports: ModelOperationSignature::new(inputs, outputs),
            metadata: ModelSignatureMetadata::default(),
        }
    }

    /// Creates a model signature from the existing canonical operation
    /// signature.
    #[must_use]
    pub fn from_operation_signature(
        ports: ModelOperationSignature,
    ) -> Self {
        Self {
            ports,
            metadata: ModelSignatureMetadata::default(),
        }
    }

    /// Sets signature metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: ModelSignatureMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns input ports in declaration order.
    #[must_use]
    pub fn inputs(&self) -> &[ModelOperationPort] {
        &self.ports.inputs
    }

    /// Returns output ports in declaration order.
    #[must_use]
    pub fn outputs(&self) -> &[ModelOperationPort] {
        &self.ports.outputs
    }

    /// Finds an input by semantic name.
    #[must_use]
    pub fn input(
        &self,
        name: &SemanticName,
    ) -> Option<&ModelOperationPort> {
        self.ports.inputs.iter().find(|port| &port.name == name)
    }

    /// Finds an output by semantic name.
    #[must_use]
    pub fn output(
        &self,
        name: &SemanticName,
    ) -> Option<&ModelOperationPort> {
        self.ports.outputs.iter().find(|port| &port.name == name)
    }

    /// Returns the number of input ports.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.ports.inputs.len()
    }

    /// Returns the number of output ports.
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.ports.outputs.len()
    }

    /// Validates local signature structure.
    ///
    /// This delegates port validation to ModelOperationSignature.
    ///
    /// It does not perform type inference, generic substitution,
    /// semantic name resolution or runtime feasibility checks.
    pub fn validate(&self) -> Result<(), ModelSignatureError> {
        self.ports
            .validate()
            .map_err(ModelSignatureError::InvalidPorts)?;

        self.metadata.validate()?;

        Ok(())
    }

    /// Checks whether the outputs of this model can satisfy the inputs
    /// of a receiving model.
    ///
    /// Compatibility is directional:
    ///
    ///     producer.outputs -> receiver.inputs
    ///
    /// A producer may have additional outputs. Those outputs need not
    /// be consumed by the receiver.
    ///
    /// A receiver may have optional inputs that the producer does not
    /// supply.
    ///
    /// A required receiver input must have a compatible producer output.
    ///
    /// Type compatibility is delegated to the canonical semantic service.
    pub fn can_feed_into<C: SemanticTypeCompatibility>(
        &self,
        receiver: &Self,
        compatibility: &C,
    ) -> SignatureCompatibilityReport {
        let mut report = SignatureCompatibilityReport {
            complete: true,
            issues: Vec::new(),
        };

        for accepted in receiver.inputs() {
            let produced = self.output(&accepted.name);

            let Some(produced) = produced else {
                if accepted.required {
                    report.issues.push(
                        SignatureCompatibilityIssue::MissingRequiredInput {
                            name: accepted.name.clone(),
                        },
                    );
                }

                continue;
            };

            match compatibility.is_compatible(
                &produced.ty,
                &accepted.ty,
            ) {
                TypeCompatibility::Compatible => {}

                TypeCompatibility::Incompatible => {
                    report.issues.push(
                        SignatureCompatibilityIssue::IncompatibleType {
                            name: accepted.name.clone(),
                        },
                    );
                }

                TypeCompatibility::Undetermined => {
                    report.complete = false;

                    report.issues.push(
                        SignatureCompatibilityIssue::UndeterminedType {
                            name: accepted.name.clone(),
                        },
                    );
                }
            }

            if !cardinality_can_feed(produced, accepted) {
                report.issues.push(
                    SignatureCompatibilityIssue::IncompatibleCardinality {
                        name: accepted.name.clone(),
                    },
                );
            }
        }

        report
    }

    /// Checks whether this signature can satisfy a declared interface.
    ///
    /// This operation is useful for model implementation and interface
    /// conformance.
    ///
    /// The supplied type compatibility service determines semantic
    /// substitutability.
    pub fn conforms_to<C: SemanticTypeCompatibility>(
        &self,
        contract: &Self,
        compatibility: &C,
    ) -> SignatureCompatibilityReport {
        self.can_feed_into(contract, compatibility)
    }
}

// ============================================================================
// CARDINALITY COMPATIBILITY
// ============================================================================

/// Checks whether a produced output port can supply an accepted input port.
///
/// This check concerns value cardinality, not tensor dimensions or
/// physical execution capacity.
fn cardinality_can_feed(
    produced: &ModelOperationPort,
    accepted: &ModelOperationPort,
) -> bool {
    use crate::ai::model::operations::ModelOperationPortCardinality;

    match (&produced.cardinality, &accepted.cardinality) {
        (
            ModelOperationPortCardinality::One,
            ModelOperationPortCardinality::One,
        ) => true,

        (
            ModelOperationPortCardinality::One,
            ModelOperationPortCardinality::Optional,
        ) => true,

        (
            ModelOperationPortCardinality::Optional,
            ModelOperationPortCardinality::Optional,
        ) => true,

        (
            ModelOperationPortCardinality::Optional,
            ModelOperationPortCardinality::One,
        ) => false,

        (
            ModelOperationPortCardinality::Many { .. },
            ModelOperationPortCardinality::Many { .. },
        ) => true,

        (
            ModelOperationPortCardinality::Many { .. },
            _,
        ) => false,

        (
            ModelOperationPortCardinality::Custom { identity: left, .. },
            ModelOperationPortCardinality::Custom { identity: right, .. },
        ) => left == right,

        _ => false,
    }
}

// ============================================================================
// SIGNATURE ERRORS
// ============================================================================

/// Structural errors produced by model signature construction or validation.
#[derive(Debug)]
#[non_exhaustive]
pub enum ModelSignatureError {
    /// The underlying canonical port signature is invalid.
    InvalidPorts(
        crate::ai::model::operations::ModelOperationError,
    ),

    /// Invalid schema identity.
    InvalidSchemaIdentity,

    /// Invalid metadata key.
    InvalidMetadataKey,

    /// Duplicate metadata key.
    DuplicateMetadataKey,
}

impl fmt::Display for ModelSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPorts(error) => {
                write!(f, "invalid model signature ports: {error}")
            }

            Self::InvalidSchemaIdentity => {
                f.write_str("invalid model signature schema identity")
            }

            Self::InvalidMetadataKey => {
                f.write_str("invalid model signature metadata key")
            }

            Self::DuplicateMetadataKey => {
                f.write_str("duplicate model signature metadata key")
            }
        }
    }
}

impl Error for ModelSignatureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPorts(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_text(value: &str) -> Result<(), ()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(());
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::model::operations::{
        ModelOperationPort,
        ModelOperationPortCardinality,
        ModelOperationPortDirection,
        ModelOperationPortId,
    };

    use crate::frontend::semantic::types::ty::{
        PrimitiveType,
        SemanticName,
    };

    fn name(value: &str) -> SemanticName {
        SemanticName::new(value)
    }

    fn port(
        id: &str,
        port_name: &str,
        direction: ModelOperationPortDirection,
        ty: SemanticType,
    ) -> ModelOperationPort {
        ModelOperationPort::new(
            ModelOperationPortId::new(id)
                .expect("valid port identity"),
            name(port_name),
            direction,
            ty,
        )
    }

    fn float_type() -> SemanticType {
        SemanticType::Primitive(PrimitiveType::Float {
            width: None,
        })
    }

    struct ExactCompatibility;

    impl SemanticTypeCompatibility for ExactCompatibility {
        fn is_compatible(
            &self,
            produced: &SemanticType,
            accepted: &SemanticType,
        ) -> TypeCompatibility {
            if produced == accepted {
                TypeCompatibility::Compatible
            } else {
                TypeCompatibility::Incompatible
            }
        }
    }

    struct UnknownCompatibility;

    impl SemanticTypeCompatibility for UnknownCompatibility {
        fn is_compatible(
            &self,
            _: &SemanticType,
            _: &SemanticType,
        ) -> TypeCompatibility {
            TypeCompatibility::Undetermined
        }
    }

    fn signature(
        inputs: Vec<ModelOperationPort>,
        outputs: Vec<ModelOperationPort>,
    ) -> ModelSignature {
        ModelSignature::new(inputs, outputs)
    }

    #[test]
    fn creates_empty_signature() {
        let signature = signature(Vec::new(), Vec::new());

        assert!(signature.validate().is_ok());
        assert_eq!(signature.input_count(), 0);
        assert_eq!(signature.output_count(), 0);
    }

    #[test]
    fn preserves_input_and_output_order() {
        let signature = signature(
            vec![
                port(
                    "input-a",
                    "first",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
                port(
                    "input-b",
                    "second",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            vec![
                port(
                    "output-a",
                    "result",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
        );

        assert_eq!(signature.inputs()[0].name, name("first"));
        assert_eq!(signature.inputs()[1].name, name("second"));
        assert_eq!(signature.outputs()[0].name, name("result"));
    }

    #[test]
    fn finds_ports_by_name() {
        let signature = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            vec![
                port(
                    "output",
                    "result",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
        );

        assert!(signature.input(&name("features")).is_some());
        assert!(signature.output(&name("result")).is_some());

        assert!(signature.input(&name("missing")).is_none());
        assert!(signature.output(&name("missing")).is_none());
    }

    #[test]
    fn rejects_duplicate_port_names() {
        let signature = signature(
            vec![
                port(
                    "input-a",
                    "value",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            vec![
                port(
                    "output-a",
                    "value",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
        );

        assert!(signature.validate().is_err());
    }

    #[test]
    fn rejects_incorrect_port_direction() {
        let signature = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        assert!(signature.validate().is_err());
    }

    #[test]
    fn accepts_structured_canonical_types() {
        let structured = SemanticType::Tuple(vec![
            float_type(),
            float_type(),
        ]);

        let signature = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    structured,
                ),
            ],
            Vec::new(),
        );

        assert!(signature.validate().is_ok());
    }

    #[test]
    fn supports_optional_input_cardinality() {
        let optional = port(
            "input",
            "context",
            ModelOperationPortDirection::Input,
            float_type(),
        )
        .with_cardinality(ModelOperationPortCardinality::Optional)
        .with_required(false);

        let signature = signature(vec![optional], Vec::new());

        assert!(signature.validate().is_ok());
    }

    #[test]
    fn accepts_matching_producer_and_receiver() {
        let producer = signature(
            Vec::new(),
            vec![
                port(
                    "output",
                    "features",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
        );

        let receiver = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        let report = producer.can_feed_into(
            &receiver,
            &ExactCompatibility,
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn rejects_missing_required_receiver_input() {
        let producer = signature(Vec::new(), Vec::new());

        let receiver = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        let report = producer.can_feed_into(
            &receiver,
            &ExactCompatibility,
        );

        assert!(!report.is_compatible());
        assert!(report.has_incompatibilities());
    }

    #[test]
    fn permits_missing_optional_receiver_input() {
        let producer = signature(Vec::new(), Vec::new());

        let receiver = signature(
            vec![
                port(
                    "input",
                    "context",
                    ModelOperationPortDirection::Input,
                    float_type(),
                )
                .with_required(false)
                .with_cardinality(
                    ModelOperationPortCardinality::Optional,
                ),
            ],
            Vec::new(),
        );

        let report = producer.can_feed_into(
            &receiver,
            &ExactCompatibility,
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn does_not_treat_unknown_type_compatibility_as_success() {
        let producer = signature(
            Vec::new(),
            vec![
                port(
                    "output",
                    "features",
                    ModelOperationPortDirection::Output,
                    float_type(),
                ),
            ],
        );

        let receiver = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        let report = producer.can_feed_into(
            &receiver,
            &UnknownCompatibility,
        );

        assert!(!report.is_compatible());
        assert!(report.is_undetermined());
        assert!(!report.complete);
    }

    #[test]
    fn rejects_optional_output_for_required_input() {
        let producer = signature(
            Vec::new(),
            vec![
                port(
                    "output",
                    "features",
                    ModelOperationPortDirection::Output,
                    float_type(),
                )
                .with_cardinality(
                    ModelOperationPortCardinality::Optional,
                ),
            ],
        );

        let receiver = signature(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        let report = producer.can_feed_into(
            &receiver,
            &ExactCompatibility,
        );

        assert!(report.has_incompatibilities());
    }

    #[test]
    fn validates_metadata() {
        let metadata = ModelSignatureMetadata::new()
            .with_documentation("Model input/output contract")
            .with_schema_identity("model.schema.v1")
            .with_extension("domain", "ai");

        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_metadata_keys() {
        let metadata = ModelSignatureMetadata::new()
            .with_extension("domain", "ai")
            .with_extension("domain", "machine-learning");

        assert_eq!(
            metadata.validate(),
            Err(ModelSignatureError::DuplicateMetadataKey),
        );
    }

    #[test]
    fn supports_signature_conversion_from_operation_signature() {
        let ports = ModelOperationSignature::new(
            vec![
                port(
                    "input",
                    "features",
                    ModelOperationPortDirection::Input,
                    float_type(),
                ),
            ],
            Vec::new(),
        );

        let signature = ModelSignature::from_operation_signature(ports);

        assert_eq!(signature.input_count(), 1);
        assert!(signature.validate().is_ok());
    }
}
