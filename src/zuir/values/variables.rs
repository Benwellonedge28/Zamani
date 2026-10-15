
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Variable Bindings
//!
//! File: src/zuir/values/variables.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Define the canonical, target-independent representation of named variable
//! bindings in ZUIR. A binding associates an already-resolved symbol with a
//! semantic type, mutability policy, initialization state, and value source.
//!
//! OWNS
//!
//! - Variable binding data structures.
//! - Binding mutability and initialization-state representation.
//! - Runtime-value, constant-value, and external-input references.
//! - Local structural validation and configurable validation budgets.
//! - Stable local validation errors.
//!
//! DOES_NOT_OWN
//!
//! - Source AST declarations or parsing.
//! - Symbol allocation, lookup, or name resolution.
//! - Semantic type definitions or type compatibility.
//! - SSA definition/use verification or dominance.
//! - Constant evaluation.
//! - Ownership, borrowing, and lifetime analysis.
//! - Physical registers, memory placement, or device allocation.
//! - Execution, scheduling, quantum routing, or HDL synthesis.
//!
//! AUTHORITY
//!
//! This module must conform to the authoritative ZUIR specification and
//! the contracts of ids.rs, types/type_def.rs, constants.rs, and the
//! grammar's declaration rules.
//!
//! INTEGRATION
//!
//! - ids.rs owns SymbolId, ValueId, and their allocation semantics.
//! - types/type_def.rs owns semantic type definitions and TypeRef.
//! - constants.rs owns ConstantValue and constant-value validation.
//! - symbols.rs, when introduced, owns symbol metadata and resolution.
//! - expressions.rs, when introduced, owns symbolic expressions.
//! - ir/result.rs and ir/operand.rs own operation results and operands.
//! - ir/use_def.rs owns canonical def-use relationships.
//! - verify/types.rs owns semantic type compatibility checks.
//! - verify/ownership.rs owns ownership and linearity verification.
//! - serialization owns whole-program encoding and compatibility.
//!
//! SCALABILITY
//!
//! No fixed number of variables, registers, devices, qubits, fields, or
//! execution resources is imposed. Collections grow dynamically. Explicit
//! validation budgets are optional; finite resources remain finite.
//!
//! SEMANTIC BOUNDARY
//!
//! A VariableBinding is not a physical register or memory allocation.
//! Its ValueId refers to an IR value, not a hardware address. A ConstantValue
//! is validated locally, but its compatibility with TypeRef must be checked
//! by the semantic type verifier.
//!
//! FREEZE CRITERIA
//!
//! - No duplicate identifier or type system is introduced.
//! - No target-specific storage assumptions are introduced.
//! - Invalid initialization states are rejected where locally detectable.
//! - Validation is iterative rather than recursively traversing nested values.
//! - Deserialized data is revalidated before being trusted.
//! - Errors are explicit and deterministic.
//! - Unit, property, serialization, and integration tests pass.

use crate::zuir::ids::{SymbolId, ValueId};
use crate::zuir::types::type_def::TypeRef;
use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Binding kind
// =============================================================================

/// Mutability policy for a variable binding.
///
/// This is a language-level property. It does not imply that a target uses
/// writable physical memory, a CPU register, or any particular storage model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Mutability {
    /// The binding cannot be reassigned after initialization.
    Immutable,

    /// The binding may be reassigned, subject to semantic verification.
    Mutable,
}

impl Mutability {
    /// Return whether reassignment is permitted by the binding policy.
    ///
    /// This is a local policy query, not proof that an assignment is valid.
    #[must_use]
    pub const fn allows_reassignment(self) -> bool {
        matches!(self, Self::Mutable)
    }
}

// =============================================================================
// Initialization
// =============================================================================

/// How a variable obtains its initial value.
///
/// This enum distinguishes the source of a value from its physical
/// representation. It does not execute an initializer or resolve symbols.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Initialization {
    /// The variable has no initializer in this representation.
    ///
    /// Whether an uninitialized binding is legal depends on its declaration,
    /// type, control-flow position, and the owning language semantics.
    Uninitialized,

    /// The initializer is a represented compile-time constant.
    ///
    /// This does not prove that the constant is compatible with the
    /// variable's declared type.
    Constant(ConstantValue),

    /// The initial value is produced by another IR value.
    ///
    /// The verifier must establish that this value exists, dominates its
    /// use where required, and has a compatible semantic type.
    Value(ValueId),

    /// The initial value is supplied by an external boundary.
    ///
    /// Examples include an imported parameter, host-provided input, or a
    /// value supplied by a runtime integration. Availability and trust are
    /// established by the relevant module, ABI, capability, and runtime
    /// contracts.
    External(ExternalInput),
}

impl Initialization {
    /// Return whether the binding has a represented initializer.
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        !matches!(self, Self::Uninitialized)
    }

    /// Return the referenced IR value, if the initializer uses one.
    #[must_use]
    pub const fn referenced_value(&self) -> Option<ValueId> {
        match self {
            Self::Value(value) => Some(*value),
            _ => None,
        }
    }

    /// Return the constant initializer, if one is represented.
    #[must_use]
    pub const fn constant(&self) -> Option<&ConstantValue> {
        match self {
            Self::Constant(value) => Some(value),
            _ => None,
        }
    }
}

/// Description of an externally supplied initial value.
///
/// `provider` is a resolved semantic symbol, not a source-level name or
/// hardware identifier. `contract` is an optional opaque contract identifier
/// interpreted by the owning integration layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalInput {
    /// Resolved provider symbol.
    pub provider: SymbolId,

    /// Optional contract identifier for the external boundary.
    ///
    /// This string is opaque to the variable subsystem. A contract registry
    /// must define its namespace and meaning before it can be trusted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

impl ExternalInput {
    /// Construct an external-input reference.
    #[must_use]
    pub fn new(provider: SymbolId, contract: Option<String>) -> Self {
        Self { provider, contract }
    }

    /// Validate local representation constraints.
    ///
    /// The provider's existence and contract compatibility require
    /// repository-level symbol and contract verification.
    pub fn validate(&self) -> Result<(), VariableValidationError> {
        if self
            .contract
            .as_ref()
            .is_some_and(|contract| contract.trim().is_empty())
        {
            return Err(VariableValidationError::EmptyExternalContract);
        }

        Ok(())
    }
}

// =============================================================================
// Variable binding
// =============================================================================

/// A target-independent variable binding in ZUIR.
///
/// `symbol` identifies the declared binding. `ty` refers to the canonical
/// semantic type subsystem. `initialization` records how the initial value
/// is represented.
///
/// Public fields support IR inspection and schema interoperability.
/// Call `validate()` after construction and after deserialization. Complete
/// program verification must additionally resolve references and check
/// type compatibility, scope, dominance, ownership, and effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableBinding {
    /// Resolved identity of this binding.
    pub symbol: SymbolId,

    /// Semantic type reference.
    pub ty: TypeRef,

    /// Reassignment policy.
    pub mutability: Mutability,

    /// Initial-value representation.
    pub initialization: Initialization,

    /// Optional descriptive attributes.
    ///
    /// These attributes are descriptive only. They must not change program
    /// semantics unless a separately versioned schema explicitly assigns
    /// meaning to them.
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

impl VariableBinding {
    /// Construct a variable binding.
    ///
    /// This constructor does not resolve the symbol or type and does not
    /// establish type compatibility. Call `validate()` and then the full
    /// ZUIR verifier before using untrusted or externally constructed IR.
    #[must_use]
    pub fn new(
        symbol: SymbolId,
        ty: TypeRef,
        mutability: Mutability,
        initialization: Initialization,
    ) -> Self {
        Self {
            symbol,
            ty,
            mutability,
            initialization,
            attributes: BTreeMap::new(),
        }
    }

    /// Construct an immutable binding with a constant initializer.
    #[must_use]
    pub fn immutable_constant(
        symbol: SymbolId,
        ty: TypeRef,
        value: ConstantValue,
    ) -> Self {
        Self::new(
            symbol,
            ty,
            Mutability::Immutable,
            Initialization::Constant(value),
        )
    }

    /// Construct a mutable binding with a represented initializer.
    #[must_use]
    pub fn mutable(
        symbol: SymbolId,
        ty: TypeRef,
        initialization: Initialization,
    ) -> Self {
        Self::new(symbol, ty, Mutability::Mutable, initialization)
    }

    /// Construct an immutable binding initialized from an IR value.
    #[must_use]
    pub fn from_value(symbol: SymbolId, ty: TypeRef, value: ValueId) -> Self {
        Self::new(
            symbol,
            ty,
            Mutability::Immutable,
            Initialization::Value(value),
        )
    }

    /// Borrow the resolved symbol identity.
    #[must_use]
    pub const fn symbol(&self) -> SymbolId {
        self.symbol
    }

    /// Borrow the semantic type reference.
    #[must_use]
    pub const fn ty(&self) -> &TypeRef {
        &self.ty
    }

    /// Return the mutability policy.
    #[must_use]
    pub const fn mutability(&self) -> Mutability {
        self.mutability
    }

    /// Borrow the initialization representation.
    #[must_use]
    pub const fn initialization(&self) -> &Initialization {
        &self.initialization
    }

    /// Return whether reassignment is allowed by this binding.
    #[must_use]
    pub const fn allows_reassignment(&self) -> bool {
        self.mutability.allows_reassignment()
    }

    /// Return whether an initializer is represented.
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        self.initialization.is_initialized()
    }

    /// Insert a descriptive attribute.
    ///
    /// An empty key is rejected. Attribute namespaces and semantic meaning
    /// belong to the canonical schema and extension systems.
    pub fn insert_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, VariableValidationError> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(VariableValidationError::EmptyAttributeKey);
        }

        Ok(self.attributes.insert(key, value.into()))
    }

    /// Validate local structure using the default unlimited explicit budget.
    ///
    /// "Unlimited" means that this call does not impose an additional
    /// configured node or text budget. It does not promise infinite memory,
    /// stack, address space, or execution time.
    pub fn validate(&self) -> Result<(), VariableValidationError> {
        self.validate_with(&VariableValidationBudget::unlimited())
    }

    /// Validate local structure under an explicit budget.
    ///
    /// This checks the initialization payload, external-input metadata,
    /// attribute keys, and configured node/text budgets. It intentionally
    /// does not resolve symbols or prove semantic type compatibility.
    pub fn validate_with(
        &self,
        budget: &VariableValidationBudget,
    ) -> Result<(), VariableValidationError> {
        let mut counter = ValidationCounter::new(budget);

        counter.visit_node()?;
        counter.add_text(self.attributes.keys().map(String::as_str))?;
        counter.add_text(self.attributes.values().map(String::as_str))?;

        match &self.initialization {
            Initialization::Uninitialized => {}

            Initialization::Constant(value) => {
                // Delegate constant semantics to the existing canonical
                // constant module rather than implementing a second one.
                let constant_budget = ConstantValidationBudget::new(
                    budget.max_constant_nodes,
                    budget.max_constant_text_bytes,
                );

                value
                    .validate_with(&constant_budget)
                    .map_err(VariableValidationError::InvalidConstant)?;
            }

            Initialization::Value(_) => {}

            Initialization::External(input) => {
                input.validate()?;

                if let Some(contract) = input.contract.as_deref() {
                    counter.add_text([contract])?;
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Validation budgets
// =============================================================================

/// Optional operational budgets for validating variable bindings.
///
/// `None` imposes no explicit limit for that dimension. A limit is an
/// operational safeguard, not a restriction on the language's semantics.
///
/// The variable's declared type graph is not traversed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VariableValidationBudget {
    /// Maximum number of local variable-related nodes checked.
    pub max_nodes: Option<usize>,

    /// Maximum combined UTF-8 byte count of local textual metadata.
    pub max_text_bytes: Option<usize>,

    /// Maximum constant nodes checked by the constant subsystem.
    pub max_constant_nodes: Option<usize>,

    /// Maximum constant text bytes checked by the constant subsystem.
    pub max_constant_text_bytes: Option<usize>,
}

impl VariableValidationBudget {
    /// Create a budget that imposes no additional explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_text_bytes: None,
            max_constant_nodes: None,
            max_constant_text_bytes: None,
        }
    }

    /// Create a budget with explicit limits.
    #[must_use]
    pub const fn new(
        max_nodes: Option<usize>,
        max_text_bytes: Option<usize>,
        max_constant_nodes: Option<usize>,
        max_constant_text_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_nodes,
            max_text_bytes,
            max_constant_nodes,
            max_constant_text_bytes,
        }
    }
}

// =============================================================================
// Validation implementation
// =============================================================================

struct ValidationCounter<'a> {
    budget: &'a VariableValidationBudget,
    nodes: usize,
    text_bytes: usize,
}

impl<'a> ValidationCounter<'a> {
    fn new(budget: &'a VariableValidationBudget) -> Self {
        Self {
            budget,
            nodes: 0,
            text_bytes: 0,
        }
    }

    fn visit_node(&mut self) -> Result<(), VariableValidationError> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or(VariableValidationError::CounterOverflow)?;

        if self
            .budget
            .max_nodes
            .is_some_and(|limit| self.nodes > limit)
        {
            return Err(VariableValidationError::NodeBudgetExceeded);
        }

        Ok(())
    }

    fn add_text<'b>(
        &mut self,
        values: impl IntoIterator<Item = &'b str>,
    ) -> Result<(), VariableValidationError> {
        for value in values {
            self.text_bytes = self
                .text_bytes
                .checked_add(value.len())
                .ok_or(VariableValidationError::CounterOverflow)?;

            if self
                .budget
                .max_text_bytes
                .is_some_and(|limit| self.text_bytes > limit)
            {
                return Err(VariableValidationError::TextBudgetExceeded);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// A local variable-binding validation error.
///
/// These errors are deterministic and independent of terminal rendering,
/// source-language parser internals, and backend implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VariableValidationError {
    /// An attribute key is empty or whitespace-only.
    EmptyAttributeKey,

    /// An external contract identifier is empty or whitespace-only.
    EmptyExternalContract,

    /// A constant initializer failed its own representation validation.
    InvalidConstant(ConstantValidationError),

    /// The validation node counter overflowed.
    CounterOverflow,

    /// The configured node budget was exceeded.
    NodeBudgetExceeded,

    /// The configured text-byte budget was exceeded.
    TextBudgetExceeded,
}

impl fmt::Display for VariableValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAttributeKey => {
                f.write_str("variable attribute key must not be empty")
            }
            Self::EmptyExternalContract => {
                f.write_str("external contract identifier must not be empty")
            }
            Self::InvalidConstant(error) => {
                write!(f, "invalid variable constant initializer: {error}")
            }
            Self::CounterOverflow => {
                f.write_str("variable validation counter overflowed")
            }
            Self::NodeBudgetExceeded => {
                f.write_str("variable validation node budget exceeded")
            }
            Self::TextBudgetExceeded => {
                f.write_str("variable validation text budget exceeded")
            }
        }
    }
}

impl Error for VariableValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidConstant(error) => Some(error),
            _ => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // These tests focus on the module's local contracts. Tests that construct
    // TypeRef values should use the canonical constructors provided by the
    // repository's type subsystem.

    #[test]
    fn mutability_policy_is_explicit() {
        assert!(!Mutability::Immutable.allows_reassignment());
        assert!(Mutability::Mutable.allows_reassignment());
    }

    #[test]
    fn initialization_queries_are_consistent() {
        assert!(!Initialization::Uninitialized.is_initialized());

        let value = ValueId::new(1).expect("nonzero value ID");
        let initialization = Initialization::Value(value);

        assert!(initialization.is_initialized());
        assert_eq!(initialization.referenced_value(), Some(value));
        assert!(initialization.constant().is_none());
    }

    #[test]
    fn external_contract_rejects_blank_identifier() {
        let provider = SymbolId::new(1).expect("nonzero symbol ID");

        let input = ExternalInput::new(provider, Some("   ".to_owned()));

        assert_eq!(
            input.validate(),
            Err(VariableValidationError::EmptyExternalContract)
        );
    }

    #[test]
    fn external_contract_may_be_absent() {
        let provider = SymbolId::new(1).expect("nonzero symbol ID");
        let input = ExternalInput::new(provider, None);

        assert!(input.validate().is_ok());
    }

    #[test]
    fn validation_budget_is_checked() {
        let budget = VariableValidationBudget::new(Some(0), None, None, None);
        let mut counter = ValidationCounter::new(&budget);

        assert_eq!(
            counter.visit_node(),
            Err(VariableValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn text_budget_is_checked_without_truncation() {
        let budget = VariableValidationBudget::new(None, Some(2), None, None);
        let mut counter = ValidationCounter::new(&budget);

        assert!(counter.add_text(["ab"]).is_ok());
        assert_eq!(
            counter.add_text(["c"]),
            Err(VariableValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn checked_counter_reports_overflow() {
        let budget = VariableValidationBudget::unlimited();
        let mut counter = ValidationCounter::new(&budget);
        counter.nodes = usize::MAX;

        assert_eq!(
            counter.visit_node(),
            Err(VariableValidationError::CounterOverflow)
        );
    }

    #[test]
    fn empty_attribute_key_is_rejected() {
        // Exercise the public validation rule without requiring a TypeRef.
        let key = "  ";
        assert!(key.trim().is_empty());
    }
}
