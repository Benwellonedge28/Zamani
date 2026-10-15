
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Operation Operand
//!
//! Canonical path: `src/zuir/ir/operand.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Represent one ordered input reference consumed by a ZUIR operation.
//!
//! An operand references an existing or provisionally constructed ZUIR value.
//! It may optionally carry a qualified, extension-defined role.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## OWNS
//!
//! - `OperationOperand` and its local invariants.
//! - Optional qualified operand-role metadata.
//! - Structural validation and operand-specific errors.
//!
//! ## DOES_NOT_OWN
//!
//! - Value allocation or global value identity.
//! - Type definitions, type inference, or type compatibility.
//! - SSA dominance, use-def indexing, or reference resolution.
//! - Operation schemas, dialect registration, or effect verification.
//! - Source parsing, AST construction, or name resolution.
//! - Physical registers, memory addresses, device identifiers,
//!   quantum qubit identities, hardware placement, or execution.
//!
//! ## SCALABILITY
//!
//! This type imposes no universal operand-count, machine-size, hardware,
//! qubit-count, register-width, tensor-rank, or device-count limit.
//! Collection sizing is the responsibility of the containing operation.
//! No finite implementation promises literal infinity: actual construction
//! remains bounded by available address space and system resources.
//!
//! ## SERIALIZATION
//!
//! Deserialized operands must be validated before trusted IR processing.
//! Serde deserialization alone is not a substitute for semantic verification.
//!
//! ## INTEGRATION
//!
//! `operation.rs` owns ordered operand collections and translates
//! `OperandError` into its existing `OperationError` API.
//! The whole-IR verifier resolves the referenced `ValueId` and validates
//! its definition, type, scope, dominance, and dialect-specific role.
//!
//! This module deliberately depends only on canonical IDs and qualified
//! schema names, avoiding a dependency on operations or verifier internals.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::zuir::ids::ValueId;
use crate::zuir::schema::types::QualifiedName;

/// Result type for operand-local construction and validation.
pub type OperandResult<T> = Result<T, OperandError>;

/// One ordered input reference consumed by a ZUIR operation.
///
/// Operands are references, not owned values. Repeating the same `ValueId`
/// is permitted because an operation may consume one value in multiple
/// operand positions.
///
/// `role` is optional descriptive metadata qualified by namespace. A valid
/// name does not establish that the role is registered or legal for a
/// particular operation. The owning dialect schema decides that.
///
/// The fields are private so ordinary Rust callers must use the constructors
/// and mutation methods, which validate role metadata. Deserialization can
/// still introduce malformed nested data; call [`Self::validate`] after
/// decoding untrusted or persisted representations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OperationOperand {
    value: ValueId,
    role: Option<QualifiedName>,
}

impl OperationOperand {
    /// Create an operand referencing `value`, without an explicit role.
    ///
    /// The value identity is accepted without lookup. Whether it exists and
    /// is visible at the use site is checked by the whole-IR verifier.
    #[must_use]
    pub const fn new(value: ValueId) -> Self {
        Self { value, role: None }
    }

    /// Create an operand with a validated, qualified role.
    ///
    /// # Errors
    ///
    /// Returns `OperandError::InvalidRole` if the supplied qualified name
    /// violates the canonical qualified-name rules.
    pub fn with_role(
        value: ValueId,
        role: QualifiedName,
    ) -> OperandResult<Self> {
        role.validate().map_err(|error| {
            OperandError::InvalidRole(error.to_string())
        })?;

        Ok(Self {
            value,
            role: Some(role),
        })
    }

    /// Return the referenced value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the optional qualified role.
    #[must_use]
    pub const fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Replace the optional role after validating the new value.
    ///
    /// The existing role is unchanged if validation fails.
    pub fn set_role(
        &mut self,
        role: Option<QualifiedName>,
    ) -> OperandResult<()> {
        if let Some(candidate) = &role {
            candidate.validate().map_err(|error| {
                OperandError::InvalidRole(error.to_string())
            })?;
        }

        self.role = role;
        Ok(())
    }

    /// Validate all invariants that can be checked using this operand alone.
    ///
    /// This method intentionally does not look up the referenced value or
    /// infer operation-specific role semantics.
    pub fn validate(&self) -> OperandResult<()> {
        if let Some(role) = &self.role {
            role.validate().map_err(|error| {
                OperandError::InvalidRole(error.to_string())
            })?;
        }

        Ok(())
    }

    /// Return the referenced value and optional role as borrowed data.
    ///
    /// This convenience method does not resolve the value or interpret the
    /// role; it exposes the existing canonical representation.
    #[must_use]
    pub const fn parts(&self) -> (ValueId, Option<&QualifiedName>) {
        (self.value, self.role.as_ref())
    }

    /// Consume this operand and return its value identity and role.
    #[must_use]
    pub fn into_parts(self) -> (ValueId, Option<QualifiedName>) {
        (self.value, self.role)
    }
}

/// Errors detectable from an individual operand without access to the IR.
///
/// Global reference validity, type compatibility, dominance, and role
/// authorization belong to their respective verifier layers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperandError {
    /// The optional qualified role is malformed.
    InvalidRole(String),
}

impl fmt::Display for OperandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRole(reason) => {
                write!(f, "invalid qualified operand role: {reason}")
            }
        }
    }
}

impl std::error::Error for OperandError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn role(name: &str) -> QualifiedName {
        QualifiedName::parse(name).expect("test role must be valid")
    }

    #[test]
    fn creates_operand_without_role() {
        let operand = OperationOperand::new(value_id(1));

        assert_eq!(operand.value(), value_id(1));
        assert_eq!(operand.role(), None);
        assert_eq!(operand.validate(), Ok(()));
    }

    #[test]
    fn creates_operand_with_qualified_role() {
        let operand = OperationOperand::with_role(
            value_id(2),
            role("quantum::control"),
        )
        .expect("qualified role should be accepted");

        assert_eq!(operand.value(), value_id(2));
        assert_eq!(
            operand.role().map(QualifiedName::canonical_string).as_deref(),
            Some("quantum::control"),
        );
        assert_eq!(operand.validate(), Ok(()));
    }

    #[test]
    fn permits_repeated_value_references() {
        let first = OperationOperand::new(value_id(3));
        let second = OperationOperand::new(value_id(3));

        assert_eq!(first, second);
    }

    #[test]
    fn role_can_be_replaced_and_removed() {
        let mut operand = OperationOperand::with_role(
            value_id(4),
            role("core::input"),
        )
        .expect("role should be valid");

        operand
            .set_role(Some(role("tensor::element")))
            .expect("replacement role should be valid");

        assert_eq!(
            operand.role().map(QualifiedName::canonical_string).as_deref(),
            Some("tensor::element"),
        );

        operand
            .set_role(None)
            .expect("removing role should succeed");

        assert_eq!(operand.role(), None);
        assert_eq!(operand.validate(), Ok(()));
    }

    #[test]
    fn exposes_parts_without_changing_identity() {
        let operand = OperationOperand::with_role(
            value_id(5),
            role("hdl::signal"),
        )
        .expect("role should be valid");

        let (value, role) = operand.parts();

        assert_eq!(value, value_id(5));
        assert_eq!(
            role.map(QualifiedName::canonical_string).as_deref(),
            Some("hdl::signal"),
        );
    }

    #[test]
    fn round_trips_through_serde_json() {
        let operand = OperationOperand::with_role(
            value_id(6),
            role("core::argument"),
        )
        .expect("role should be valid");

        let encoded =
            serde_json::to_string(&operand).expect("serialization should succeed");
        let decoded: OperationOperand =
            serde_json::from_str(&encoded).expect("deserialization should succeed");

        assert_eq!(decoded, operand);
        assert_eq!(decoded.validate(), Ok(()));
    }

    #[test]
    fn detects_malformed_role_after_deserialization() {
        // The representation is deliberately malformed to prove that callers
        // must validate decoded data instead of trusting deserialization.
        let decoded: OperationOperand = serde_json::from_str(
            r#"{"value":7,"role":{"segments":[]}}"#,
        )
        .expect("structurally decodable malformed role");

        assert!(matches!(
            decoded.validate(),
            Err(OperandError::InvalidRole(_))
        ));
    }

    #[test]
    fn value_identity_is_not_a_hardware_address() {
        let operand = OperationOperand::new(value_id(123));

        // The only meaning assigned here is a reference to a typed IR value.
        assert_eq!(operand.value().get(), 123);
        assert_eq!(operand.validate(), Ok(()));
    }
}
