
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Callable Values
//!
//! Canonical path: `src/zuir/values/functions.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Represent first-class callable references in the Zamani Universal
//! Intermediate Representation (ZUIR), independently of the execution
//! target. A callable may refer to a declared function, an indirect callable
//! value, or a registered extension-provided callable.
//!
//! ## STATUS
//!
//! Proposed new module. Freeze after module wiring, compilation, tests,
//! compatibility review, and integration with the canonical verifier.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/values/symbols.rs`
//! - `src/zuir/values/expressions.rs`
//! - `grammar/functions/functions.g4`
//! - `grammar/expressions/calls.g4`
//!
//! ## OWNS
//!
//! - Callable-value references and their target categories.
//! - An associated semantic signature reference.
//! - Explicit generic or specialization arguments.
//! - Local structural validation and validation budgets.
//! - Deterministic local validation errors.
//!
//! ## DOES_NOT_OWN
//!
//! - Source-level function declaration syntax or AST nodes.
//! - Function-body, block, region, or operation storage.
//! - Symbol resolution, overload selection, or linkage resolution.
//! - The semantic type system or callable-signature compatibility.
//! - Generic inference, specialization execution, or constant evaluation.
//! - Effect, capability, ownership, resource, or contract verification.
//! - Call execution, ABI lowering, scheduling, or device selection.
//! - Complete-program serialization or runtime dispatch.
//!
//! ## INTEGRATION
//!
//! - `ids.rs` owns FunctionId, SymbolId, and ValueId.
//! - `schema/types.rs` owns QualifiedName.
//! - `types/type_def.rs` owns TypeRef and TypeArgument.
//! - `values/symbols.rs` owns declaration identity and symbol metadata.
//! - `values/expressions.rs` owns expression and call-expression structure.
//! - `ir/function.rs`, when present, owns function definitions and bodies.
//! - `verify/types.rs` owns signature and type compatibility checks.
//! - `verify/dialects.rs` or the dialect verifier owns extension legality.
//! - `ir/use_def.rs` and control-flow verification own value-reference checks.
//! - Lowering owns target-specific calling conventions and ABI realization.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! No fixed function count, parameter count, generic-argument count, device
//! count, processor count, qubit count, or hardware capacity is introduced.
//! Collections grow dynamically, subject to finite implementation resources.
//! Optional budgets bound validation work without redefining language meaning.
//!
//! ## SEMANTIC BOUNDARY
//!
//! Local validation does not prove that a target exists, a signature resolves,
//! an indirect value is callable, a generic specialization is valid, or an
//! extension is registered. Those decisions belong to the owning verifier
//! and semantic registries.
//!
//! ## FREEZE CRITERIA
//!
//! - Reuses canonical IDs, type references, and qualified names.
//! - Does not duplicate function declarations or function signature types.
//! - Supports direct, indirect, and extension-provided callable references.
//! - Uses checked counters and explicit validation budgets.
//! - Revalidates data after deserialization.
//! - Does not impose hardware-specific assumptions.
//! - Compiles and passes unit, serialization, and integration tests.

use crate::zuir::ids::{FunctionId, SymbolId, ValueId};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::{ConstantArgument, TypeArgument, TypeRef};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fmt;

// =============================================================================
// Callable value
// =============================================================================

/// A target-independent reference to a callable entity.
///
/// `signature` refers to the canonical semantic type of the callable.
/// It should resolve to a function-compatible type according to the
/// semantic type system. This structure does not duplicate that type.
///
/// The constructor deliberately does not claim that the target or signature
/// is valid. Validate the local representation, then invoke the canonical
/// semantic verifier before executing or lowering the callable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionValue {
    /// Identity of the callable entity.
    pub target: CallableTarget,

    /// Semantic signature type reference.
    ///
    /// The referenced type should describe the callable's parameters,
    /// results, effects, and calling semantics where applicable.
    pub signature: TypeRef,

    /// Explicit generic or specialization arguments.
    ///
    /// These arguments preserve their declared order. Their meaning and
    /// compatibility are established by generic/type verification.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_arguments: Vec<TypeArgument>,
}

impl FunctionValue {
    /// Construct a callable reference without asserting semantic validity.
    #[must_use]
    pub fn new(
        target: CallableTarget,
        signature: TypeRef,
        type_arguments: Vec<TypeArgument>,
    ) -> Self {
        Self {
            target,
            signature,
            type_arguments,
        }
    }

    /// Construct a reference to a function declared in the current
    /// compilation identity space.
    #[must_use]
    pub fn direct(
        function: FunctionId,
        symbol: Option<SymbolId>,
        signature: TypeRef,
    ) -> Self {
        Self::new(
            CallableTarget::Direct { function, symbol },
            signature,
            Vec::new(),
        )
    }

    /// Construct a reference to an indirect callable value.
    ///
    /// The verifier must establish that the referenced value exists, is
    /// callable, and has a compatible signature.
    #[must_use]
    pub fn indirect(value: ValueId, signature: TypeRef) -> Self {
        Self::new(
            CallableTarget::Indirect { value },
            signature,
            Vec::new(),
        )
    }

    /// Construct a reference to a callable supplied by an extension.
    ///
    /// This constructor does not register or authorize the extension.
    #[must_use]
    pub fn extension(
        dialect: QualifiedName,
        callable: QualifiedName,
        signature: TypeRef,
    ) -> Self {
        Self::new(
            CallableTarget::Extension { dialect, callable },
            signature,
            Vec::new(),
        )
    }

    /// Return the direct function identity, if this is a direct reference.
    #[must_use]
    pub const fn function_id(&self) -> Option<FunctionId> {
        match &self.target {
            CallableTarget::Direct { function, .. } => Some(*function),
            CallableTarget::Indirect { .. } | CallableTarget::Extension { .. } => None,
        }
    }

    /// Return the associated symbol identity, if one is explicitly recorded.
    #[must_use]
    pub const fn symbol_id(&self) -> Option<SymbolId> {
        match &self.target {
            CallableTarget::Direct { symbol, .. } => *symbol,
            CallableTarget::Indirect { .. } | CallableTarget::Extension { .. } => None,
        }
    }

    /// Return the indirect value identity, if this is an indirect reference.
    #[must_use]
    pub const fn value_id(&self) -> Option<ValueId> {
        match &self.target {
            CallableTarget::Indirect { value } => Some(*value),
            CallableTarget::Direct { .. } | CallableTarget::Extension { .. } => None,
        }
    }

    /// Borrow the callable's semantic signature reference.
    #[must_use]
    pub const fn signature(&self) -> &TypeRef {
        &self.signature
    }

    /// Borrow the specialization arguments in their declared order.
    #[must_use]
    pub fn type_arguments(&self) -> &[TypeArgument] {
        &self.type_arguments
    }

    /// Validate the local representation without an explicit work limit.
    ///
    /// This does not resolve the signature, prove callability, or check
    /// generic argument compatibility.
    pub fn validate(&self) -> Result<(), FunctionValueValidationError> {
        self.validate_with(&FunctionValueValidationBudget::unlimited())
    }

    /// Validate the local representation under an explicit budget.
    ///
    /// Budgets are operational safeguards. They do not impose a semantic
    /// maximum on functions, parameters, generic arguments, or hardware.
    pub fn validate_with(
        &self,
        budget: &FunctionValueValidationBudget,
    ) -> Result<(), FunctionValueValidationError> {
        let mut visited_nodes: usize = 1;
        let mut visited_text_bytes: usize = 0;

        check_node_budget(visited_nodes, budget.max_nodes)?;

        match &self.target {
            CallableTarget::Direct { .. } | CallableTarget::Indirect { .. } => {}

            CallableTarget::Extension { dialect, callable } => {
                dialect
                    .validate()
                    .map_err(|_| FunctionValueValidationError::InvalidQualifiedName)?;

                callable
                    .validate()
                    .map_err(|_| FunctionValueValidationError::InvalidQualifiedName)?;

                add_text_bytes(
                    &mut visited_text_bytes,
                    dialect.canonical_string().len(),
                    budget.max_text_bytes,
                )?;

                add_text_bytes(
                    &mut visited_text_bytes,
                    callable.canonical_string().len(),
                    budget.max_text_bytes,
                )?;
            }
        }

        // Named references contain qualified names whose local structure
        // can be checked here. Inline type definitions and full signature
        // semantics remain the responsibility of the type subsystem.
        if let TypeRef::Named(name) = &self.signature {
            name.validate()
                .map_err(|_| FunctionValueValidationError::InvalidQualifiedName)?;

            add_text_bytes(
                &mut visited_text_bytes,
                name.canonical_string().len(),
                budget.max_text_bytes,
            )?;
        }

        for argument in &self.type_arguments {
            visited_nodes = visited_nodes
                .checked_add(1)
                .ok_or(FunctionValueValidationError::CounterOverflow)?;

            check_node_budget(visited_nodes, budget.max_nodes)?;

            match argument {
                TypeArgument::Type(TypeRef::Named(name)) => {
                    name.validate()
                        .map_err(|_| FunctionValueValidationError::InvalidQualifiedName)?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        name.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;
                }

                TypeArgument::Type(TypeRef::Inline(_)) => {
                    // The type subsystem owns validation of inline type
                    // definitions and recursive type constraints.
                }

                TypeArgument::Constant(ConstantArgument::Name(name)) => {
                    name.validate()
                        .map_err(|_| FunctionValueValidationError::InvalidQualifiedName)?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        name.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;
                }

                TypeArgument::Constant(ConstantArgument::Integer(value))
                | TypeArgument::Constant(ConstantArgument::String(value))
                | TypeArgument::Symbolic(
                    crate::zuir::types::type_def::SymbolicExpression { expression: value },
                ) => {
                    add_text_bytes(
                        &mut visited_text_bytes,
                        value.len(),
                        budget.max_text_bytes,
                    )?;
                }

                TypeArgument::Constant(ConstantArgument::Boolean(_)) => {}
            }
        }

        Ok(())
    }
}

// =============================================================================
// Callable target
// =============================================================================

/// The identity mechanism used to refer to a callable.
///
/// This is an open-ended reference model, not a list of callable kinds in
/// the source language. Additional provider mechanisms should be introduced
/// through a versioned extension contract rather than hardware-specific
/// variants.
///
/// Direct and indirect references are intentionally distinct:
///
/// - A direct reference names a canonical function identity.
/// - An indirect reference uses a previously defined IR value.
/// - An extension reference delegates callable meaning to a registered
///   dialect or extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum CallableTarget {
    /// Reference to a canonical function definition.
    Direct {
        /// Identity of the function definition.
        function: FunctionId,

        /// Optional resolved declaration symbol.
        ///
        /// This is useful when a function definition and its declaration
        /// symbol have separate identities. The verifier owns their
        /// correspondence.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        symbol: Option<SymbolId>,
    },

    /// Reference to a callable value such as a closure or function object.
    Indirect {
        /// Canonical IR value that denotes the callable.
        value: ValueId,
    },

    /// Reference to an extension-provided callable.
    Extension {
        /// Registered dialect or extension namespace.
        dialect: QualifiedName,

        /// Qualified callable identifier within that namespace.
        callable: QualifiedName,
    },
}

// =============================================================================
// Validation budget
// =============================================================================

/// Operational limits for local callable-value validation.
///
/// `None` means no configured ceiling for that dimension. An unlimited
/// budget does not promise infinite memory, address space, or execution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FunctionValueValidationBudget {
    /// Maximum number of callable-value and specialization-argument nodes.
    pub max_nodes: Option<usize>,

    /// Maximum UTF-8 bytes counted for qualified names and textual arguments.
    pub max_text_bytes: Option<usize>,
}

impl FunctionValueValidationBudget {
    /// Create a budget without configured ceilings.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_text_bytes: None,
        }
    }

    /// Create a budget with explicit node and text limits.
    #[must_use]
    pub const fn new(
        max_nodes: Option<usize>,
        max_text_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_nodes,
            max_text_bytes,
        }
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// Local callable-value validation failures.
///
/// These errors do not replace type-checking, symbol-resolution, generic
/// inference, extension registration, or whole-program verification errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FunctionValueValidationError {
    /// A qualified name is structurally invalid.
    InvalidQualifiedName,

    /// A checked validation counter overflowed.
    CounterOverflow,

    /// The configured node budget was exceeded.
    NodeBudgetExceeded,

    /// The configured text-byte budget was exceeded.
    TextBudgetExceeded,
}

impl fmt::Display for FunctionValueValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQualifiedName => {
                f.write_str("callable value contains an invalid qualified name")
            }
            Self::CounterOverflow => {
                f.write_str("callable-value validation counter overflowed")
            }
            Self::NodeBudgetExceeded => {
                f.write_str("callable-value validation node budget exceeded")
            }
            Self::TextBudgetExceeded => {
                f.write_str("callable-value validation text budget exceeded")
            }
        }
    }
}

impl Error for FunctionValueValidationError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn check_node_budget(
    nodes: usize,
    limit: Option<usize>,
) -> Result<(), FunctionValueValidationError> {
    if limit.is_some_and(|limit| nodes > limit) {
        return Err(FunctionValueValidationError::NodeBudgetExceeded);
    }

    Ok(())
}

fn add_text_bytes(
    total: &mut usize,
    bytes: usize,
    limit: Option<usize>,
) -> Result<(), FunctionValueValidationError> {
    *total = total
        .checked_add(bytes)
        .ok_or(FunctionValueValidationError::CounterOverflow)?;

    if limit.is_some_and(|limit| *total > limit) {
        return Err(FunctionValueValidationError::TextBudgetExceeded);
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn signature_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::CallableSignature")
                .expect("valid test type name"),
        )
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid qualified name")
    }

    #[test]
    fn constructs_and_validates_direct_function_reference() {
        let function = FunctionId::new(1).expect("nonzero function ID");
        let symbol = SymbolId::new(2).expect("nonzero symbol ID");

        let value = FunctionValue::direct(
            function,
            Some(symbol),
            signature_type(),
        );

        assert_eq!(value.function_id(), Some(function));
        assert_eq!(value.symbol_id(), Some(symbol));
        assert!(value.validate().is_ok());
    }

    #[test]
    fn constructs_and_validates_indirect_callable_reference() {
        let id = ValueId::new(7).expect("nonzero value ID");
        let value = FunctionValue::indirect(id, signature_type());

        assert_eq!(value.value_id(), Some(id));
        assert_eq!(value.function_id(), None);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn supports_extension_provided_callables() {
        let value = FunctionValue::extension(
            name("example_domain"),
            name("example_domain::invoke"),
            signature_type(),
        );

        assert!(value.validate().is_ok());

        assert!(matches!(
            value.target,
            CallableTarget::Extension { .. }
        ));
    }

    #[test]
    fn preserves_generic_argument_order() {
        let value = FunctionValue::new(
            CallableTarget::Direct {
                function: FunctionId::new(1).expect("nonzero function ID"),
                symbol: None,
            },
            signature_type(),
            vec![
                TypeArgument::Type(TypeRef::Named(name("core::Integer"))),
                TypeArgument::Type(TypeRef::Named(name("core::Boolean"))),
            ],
        );

        assert_eq!(value.type_arguments().len(), 2);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_exceeded_node_budget() {
        let value = FunctionValue::new(
            CallableTarget::Direct {
                function: FunctionId::new(1).expect("nonzero function ID"),
                symbol: None,
            },
            signature_type(),
            vec![TypeArgument::Type(TypeRef::Named(name("core::Integer")))],
        );

        let budget = FunctionValueValidationBudget::new(Some(1), None);

        assert_eq!(
            value.validate_with(&budget),
            Err(FunctionValueValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn rejects_exceeded_text_budget() {
        let value = FunctionValue::extension(
            name("example_domain"),
            name("example_domain::invoke"),
            signature_type(),
        );

        let budget = FunctionValueValidationBudget::new(None, Some(1));

        assert_eq!(
            value.validate_with(&budget),
            Err(FunctionValueValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn serialization_round_trip_preserves_callable_value() {
        let value = FunctionValue::direct(
            FunctionId::new(1).expect("nonzero function ID"),
            Some(SymbolId::new(2).expect("nonzero symbol ID")),
            signature_type(),
        );

        let encoded =
            serde_json::to_string(&value).expect("callable should serialize");

        let decoded: FunctionValue =
            serde_json::from_str(&encoded).expect("callable should deserialize");

        assert_eq!(decoded, value);
        assert!(decoded.validate().is_ok());
    }
}
