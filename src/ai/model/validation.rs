
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Validation
//!
//! File: src/ai/model/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Coordinates structural validation of canonical AI model definitions
//! and their associated model operation contracts.
//!
//! This module provides a model-validation boundary between the canonical
//! frontend/semantic infrastructure and downstream AI model processing.
//!
//! ## Architectural ownership
//!
//! Source grammar:
//!     grammar/ai/models.g4
//!
//! AI grammar composition:
//!     grammar/ai/ai.g4
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! AI semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! Canonical model definitions:
//!     crate::ai::model::definition
//!
//! Canonical model graph:
//!     crate::ai::model::graph
//!
//! Model operation contracts:
//!     crate::ai::model::operations
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! ## Owns
//!
//! - Model-validation coordination.
//! - Model structural validation reporting.
//! - Operation-contract validation coordination.
//! - Duplicate operation identity detection within a supplied model scope.
//! - Deterministic validation issue ordering.
//! - Explicit distinction between successful and unsuccessful validation.
//!
//! ## Does not own
//!
//! - Source parsing.
//! - AST construction.
//! - Name resolution.
//! - Type inference.
//! - Generic substitution.
//! - Semantic type compatibility.
//! - Tensor shape inference.
//! - Effect authorization.
//! - Capability verification.
//! - Resource feasibility.
//! - Model graph execution.
//! - Training or inference.
//! - Hardware selection.
//! - Scheduling or placement.
//! - ZUIR construction or lowering.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! ## Validation levels
//!
//! This module performs structural validation only.
//!
//! Structural validation establishes that the supplied model and operation
//! declarations satisfy the local invariants enforced by their owning
//! definition and operation modules.
//!
//! It does not establish complete language semantic validity.
//!
//! The canonical frontend semantic pipeline must additionally establish:
//!
//! - declaration and reference resolution;
//! - generic-bound satisfaction;
//! - type compatibility;
//! - model connection port existence;
//! - model connection direction;
//! - model connection type compatibility;
//! - shape and dimension constraints;
//! - effects and policies;
//! - capability requirements;
//! - resource requirements;
//! - provenance requirements.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on:
//!
//! - model members;
//! - model connections;
//! - model operations;
//! - operation inputs;
//! - operation outputs;
//! - model nesting;
//! - generic parameters;
//! - model composition;
//! - tensor rank;
//! - model parameters;
//! - accelerators;
//! - execution targets;
//! - distributed participants.
//!
//! Actual implementation work is bounded by available memory,
//! addressability, execution budgets, and explicit operational policies.
//!
//! An operational resource failure must never be interpreted as proof
//! that a model is semantically invalid or valid.
//!
//! ## Determinism
//!
//! - Model declaration order is preserved.
//! - Operation declaration order is preserved.
//! - Validation issues are emitted in deterministic traversal order.
//! - Duplicate identities are detected without relying on hash iteration.
//!
//! ## Integration
//!
//! The canonical frontend adapter constructs ModelDefinition and
//! ModelOperation values from resolved AST and semantic information.
//!
//! This module validates their local structural contracts.
//!
//! The frontend semantic validator remains responsible for language-level
//! semantic correctness.
//!
//! The model graph module remains responsible for graph indexing,
//! traversal, and graph algorithms.
//!
//! ZUIR remains responsible for computational IR verification and lowering.
//!
//! ## Freeze criteria
//!
//! - ModelDefinition structural validation is integrated.
//! - ModelOperation structural validation is integrated.
//! - Duplicate operation identities are rejected.
//! - Validation reports preserve deterministic declaration order.
//! - Structural and semantic validation remain distinct.
//! - Resource failures are not misreported as semantic failures.
//! - Tests cover valid and invalid models.
//! - Workspace compilation and lint checks pass.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use super::definition::{
    ModelDefinition,
    ModelDefinitionError,
};

use super::operations::{
    ModelOperation,
    ModelOperationError,
};

// =============================================================================
// Validation issue location
// =============================================================================

/// Identifies the declaration associated with a validation issue.
///
/// Indices refer to the original source-order collections.
///
/// They are not persistent identities and must not be serialized as
/// canonical model identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ModelValidationLocation {
    /// The model declaration itself.
    Model,

    /// An operation at its original declaration index.
    Operation {
        index: usize,
    },
}

// =============================================================================
// Validation issue
// =============================================================================

/// A structural validation issue.
///
/// The underlying error is preserved rather than converted into an
/// unstructured string.
///
/// This allows callers to inspect the exact failure category and retain
/// structured error handling.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelValidationIssue {
    /// The canonical model definition is structurally invalid.
    InvalidDefinition {
        error: ModelDefinitionError,
    },

    /// An operation contract is structurally invalid.
    InvalidOperation {
        index: usize,
        error: ModelOperationError,
    },

    /// Two operations in the supplied scope have the same identity.
    DuplicateOperationIdentity {
        first_index: usize,
        duplicate_index: usize,
    },
}

impl ModelValidationIssue {
    /// Returns the location associated with this issue.
    #[must_use]
    pub fn location(&self) -> ModelValidationLocation {
        match self {
            Self::InvalidDefinition { .. } => {
                ModelValidationLocation::Model
            }

            Self::InvalidOperation { index, .. } => {
                ModelValidationLocation::Operation {
                    index: *index,
                }
            }

            Self::DuplicateOperationIdentity {
                duplicate_index,
                ..
            } => ModelValidationLocation::Operation {
                index: *duplicate_index,
            },
        }
    }
}

impl fmt::Display for ModelValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDefinition { error } => {
                write!(f, "invalid AI model definition: {error}")
            }

            Self::InvalidOperation { index, error } => {
                write!(
                    f,
                    "invalid AI model operation at index {index}: {error}"
                )
            }

            Self::DuplicateOperationIdentity {
                first_index,
                duplicate_index,
            } => {
                write!(
                    f,
                    "duplicate AI model operation identity: \
                     first declared at index {first_index}, \
                     repeated at index {duplicate_index}"
                )
            }
        }
    }
}

impl Error for ModelValidationIssue {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDefinition { error } => Some(error),

            Self::InvalidOperation { error, .. } => Some(error),

            Self::DuplicateOperationIdentity { .. } => None,
        }
    }
}

// =============================================================================
// Validation report
// =============================================================================

/// Result of structural model validation.
///
/// An empty issue collection means that all structural checks performed
/// by this module succeeded.
///
/// It does not mean that the model has passed complete frontend semantic
/// validation, target compatibility validation, or runtime feasibility
/// analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelValidationReport {
    issues: Vec<ModelValidationIssue>,
}

impl ModelValidationReport {
    /// Creates an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
        }
    }

    /// Returns true when no structural issues were reported.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns true when at least one structural issue was reported.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Returns the number of structural issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Borrows all structural issues in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[ModelValidationIssue] {
        &self.issues
    }

    /// Consumes the report and returns its issues.
    #[must_use]
    pub fn into_issues(self) -> Vec<ModelValidationIssue> {
        self.issues
    }

    /// Adds an issue.
    ///
    /// This is private because report construction and ordering are
    /// controlled by the validator.
    fn push(
        &mut self,
        issue: ModelValidationIssue,
    ) {
        self.issues.push(issue);
    }
}

impl Default for ModelValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Validation execution errors
// =============================================================================

/// Operational errors encountered while constructing a validation report.
///
/// These errors are deliberately separate from model structural errors.
///
/// In particular, inability to allocate report storage must not be
/// reported as a model semantic error.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelValidationError {
    /// The validation report could not reserve the requested storage.
    AllocationFailed,
}

impl fmt::Display for ModelValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed => {
                f.write_str(
                    "unable to reserve AI model validation report storage",
                )
            }
        }
    }
}

impl Error for ModelValidationError {}

// =============================================================================
// Model validator
// =============================================================================

/// Stateless coordinator for AI model structural validation.
///
/// The validator owns no model state, global registry, execution context,
/// hardware state, or mutable singleton.
///
/// It can therefore be used independently by compiler sessions and
/// concurrent compilation tasks.
#[derive(Clone, Copy, Debug, Default)]
pub struct ModelValidator;

impl ModelValidator {
    /// Creates a model validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates a model definition without an additional operation list.
    ///
    /// The model's own canonical structural validator remains authoritative.
    ///
    /// # Errors
    ///
    /// Returns AllocationFailed if the report cannot reserve its storage.
    ///
    /// Structural model failures are returned in the report rather than
    /// being confused with operational validation failures.
    pub fn validate(
        &self,
        model: &ModelDefinition,
    ) -> Result<ModelValidationReport, ModelValidationError> {
        self.validate_with_operations(model, &[])
    }

    /// Validates a model definition and its associated operation contracts.
    ///
    /// The operation slice represents the operation declarations belonging
    /// to the same validation scope.
    ///
    /// The caller is responsible for supplying the correct resolved scope.
    /// This method does not perform name or registry resolution.
    ///
    /// Checks:
    ///
    /// 1. Model structural invariants.
    /// 2. Each operation's structural invariants.
    /// 3. Duplicate operation identities within the supplied scope.
    ///
    /// Declaration order is preserved in the resulting report.
    ///
    /// # Errors
    ///
    /// Returns AllocationFailed if validation report storage cannot be
    /// reserved.
    pub fn validate_with_operations(
        &self,
        model: &ModelDefinition,
        operations: &[ModelOperation],
    ) -> Result<ModelValidationReport, ModelValidationError> {
        let mut report = ModelValidationReport::new();

        // Reserve enough issue capacity for the worst case:
        //
        // - one model-definition issue;
        // - one issue per invalid operation;
        // - one duplicate-identity issue per repeated operation.
        //
        // The upper bound is one plus twice the operation count.
        //
        // Checked arithmetic prevents address-space overflow.
        let operation_capacity = operations
            .len()
            .checked_mul(2)
            .and_then(|count| count.checked_add(1))
            .ok_or(ModelValidationError::AllocationFailed)?;

        report
            .issues
            .try_reserve(operation_capacity)
            .map_err(|_| ModelValidationError::AllocationFailed)?;

        // -------------------------------------------------------------
        // Validate the canonical model definition.
        // -------------------------------------------------------------

        if let Err(error) = model.validate() {
            report.push(
                ModelValidationIssue::InvalidDefinition {
                    error,
                },
            );
        }

        // -------------------------------------------------------------
        // Validate operation contracts.
        //
        // A BTreeMap is intentionally unnecessary here. The identity
        // strings are used as ordered-set keys, and the first declaration
        // index is retained for precise duplicate diagnostics.
        // -------------------------------------------------------------

        let mut operation_identities: BTreeSet<&str> =
            BTreeSet::new();

        let mut first_indices = std::collections::BTreeMap::<
            &str,
            usize,
        >::new();

        for (index, operation) in operations.iter().enumerate() {
            if let Err(error) = operation.validate() {
                report.push(
                    ModelValidationIssue::InvalidOperation {
                        index,
                        error,
                    },
                );
            }

            let identity = operation.id.as_str();

            if !operation_identities.insert(identity) {
                if let Some(first_index) = first_indices.get(identity) {
                    report.push(
                        ModelValidationIssue::DuplicateOperationIdentity {
                            first_index: *first_index,
                            duplicate_index: index,
                        },
                    );
                }
            } else {
                first_indices.insert(identity, index);
            }
        }

        Ok(report)
    }
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Validates a canonical AI model definition.
///
/// This is a convenience wrapper around [`ModelValidator::validate`].
///
/// # Errors
///
/// Returns an operational validation error if report storage cannot
/// be reserved.
pub fn validate_model(
    model: &ModelDefinition,
) -> Result<ModelValidationReport, ModelValidationError> {
    ModelValidator::new().validate(model)
}

/// Validates a canonical AI model definition and its operation contracts.
///
/// # Errors
///
/// Returns an operational validation error if report storage cannot
/// be reserved.
pub fn validate_model_with_operations(
    model: &ModelDefinition,
    operations: &[ModelOperation],
) -> Result<ModelValidationReport, ModelValidationError> {
    ModelValidator::new()
        .validate_with_operations(model, operations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::model::definition::{
        ModelDefinition,
        ModelMember,
        ModelMemberId,
        ModelMemberPayload,
        ModelMemberRole,
    };

    use crate::ai::model::operations::{
        ModelOperation,
        ModelOperationId,
        ModelOperationSignature,
        ModelOperationVersion,
    };

    use crate::frontend::semantic::types::ty::{
        QualifiedName,
        SemanticName,
        SemanticType,
    };

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::single(value)
    }

    fn member_id(value: &str) -> ModelMemberId {
        ModelMemberId::new(value)
            .expect("test member identity must be valid")
    }

    fn valid_model() -> ModelDefinition {
        ModelDefinition::new(qualified_name("Encoder"))
            .with_member(
                ModelMember::new(
                    member_id("input"),
                    SemanticName::new("features"),
                    ModelMemberRole::Input,
                    ModelMemberPayload::Typed {
                        ty: SemanticType::Unit,
                        initializer: None,
                    },
                ),
            )
    }

    fn operation(id: &str) -> ModelOperation {
        ModelOperation::new(
            ModelOperationId::new(id)
                .expect("test operation identity must be valid"),
            ModelOperationVersion::new(1, 0, 0),
            qualified_name("ai.test.operation"),
            ModelOperationSignature::new(
                Vec::new(),
                Vec::new(),
            ),
        )
    }

    #[test]
    fn accepts_a_structurally_valid_model() {
        let report = validate_model(&valid_model())
            .expect("validation report should be created");

        assert!(report.is_valid());
        assert!(!report.has_errors());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn rejects_a_model_with_duplicate_member_identity() {
        let model = ModelDefinition::new(qualified_name("Encoder"))
            .with_member(
                ModelMember::new(
                    member_id("duplicate"),
                    SemanticName::new("first"),
                    ModelMemberRole::Input,
                    ModelMemberPayload::Typed {
                        ty: SemanticType::Unit,
                        initializer: None,
                    },
                ),
            )
            .with_member(
                ModelMember::new(
                    member_id("duplicate"),
                    SemanticName::new("second"),
                    ModelMemberRole::Output,
                    ModelMemberPayload::Typed {
                        ty: SemanticType::Unit,
                        initializer: None,
                    },
                ),
            );

        let report = validate_model(&model)
            .expect("validation report should be created");

        assert!(!report.is_valid());
        assert_eq!(report.issue_count(), 1);

        assert!(matches!(
            report.issues()[0],
            ModelValidationIssue::InvalidDefinition {
                error: ModelDefinitionError::DuplicateMemberIdentity,
            }
        ));
    }

    #[test]
    fn accepts_multiple_distinct_operations() {
        let model = valid_model();
        let operations = [
            operation("operation.first"),
            operation("operation.second"),
        ];

        let report = validate_model_with_operations(
            &model,
            &operations,
        )
        .expect("validation report should be created");

        assert!(report.is_valid());
    }

    #[test]
    fn rejects_duplicate_operation_identities() {
        let model = valid_model();
        let operations = [
            operation("operation.shared"),
            operation("operation.other"),
            operation("operation.shared"),
        ];

        let report = validate_model_with_operations(
            &model,
            &operations,
        )
        .expect("validation report should be created");

        assert!(!report.is_valid());

        assert!(report.issues().iter().any(|issue| {
            matches!(
                issue,
                ModelValidationIssue::DuplicateOperationIdentity {
                    first_index: 0,
                    duplicate_index: 2,
                }
            )
        }));
    }

    #[test]
    fn preserves_operation_declaration_order() {
        let model = valid_model();
        let operations = [
            operation("operation.first"),
            operation("operation.second"),
            operation("operation.third"),
        ];

        let report = validate_model_with_operations(
            &model,
            &operations,
        )
        .expect("validation report should be created");

        assert!(report.is_valid());
        assert_eq!(report.issue_count(), 0);
    }

    #[test]
    fn accepts_an_empty_model_when_structurally_valid() {
        let model = ModelDefinition::new(
            qualified_name("EmptyModel"),
        );

        let report = validate_model(&model)
            .expect("validation report should be created");

        assert!(report.is_valid());
    }

    #[test]
    fn report_default_is_valid() {
        let report = ModelValidationReport::default();

        assert!(report.is_valid());
        assert!(report.issues().is_empty());
    }

    #[test]
    fn validation_issue_exposes_its_location() {
        let issue = ModelValidationIssue::DuplicateOperationIdentity {
            first_index: 1,
            duplicate_index: 4,
        };

        assert_eq!(
            issue.location(),
            ModelValidationLocation::Operation {
                index: 4,
            }
        );
    }
}
