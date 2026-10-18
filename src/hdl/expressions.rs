
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Expression Integration
//!
//! File: `src/hdl/expressions.rs`
//! Rust: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Identify the HDL-specific role of an expression already represented by
//! the canonical Zamani frontend AST.
//!
//! This module is an integration boundary, not an expression parser, AST,
//! evaluator, type checker, elaborator, or intermediate representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its expression-consuming delegates
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/expressions/`
//!
//! The grammar owns syntax. The canonical AST owns expression structure,
//! node identity, and source information. Semantic analysis owns meaning.
//! HDL-specific lowering and all HDLIR implementation remain under
//! `src/hdl/hdlir/`.
//!
//! ## Ownership
//!
//! This module owns:
//! - the vocabulary of HDL contexts that consume expressions;
//! - an explicit reference from an HDL context to a canonical AST node;
//! - validation of extension-context identifiers.
//!
//! This module does not own:
//! - expression syntax or expression AST nodes;
//! - source spans, source maps, or AST node allocation;
//! - expression parsing or operator precedence;
//! - symbol resolution, types, constant evaluation, or width evaluation;
//! - clock-domain analysis or behavioral validation;
//! - hardware allocation, target selection, synthesis, or HDLIR.
//!
//! ## Integration
//!
//! The HDL AST adapter creates an `HdlExpressionUse` only after it has a
//! canonical expression `NodeId`. The graph validator must subsequently
//! confirm that the referenced node exists and is an expression node.
//! HDL semantic validation interprets the role and obtains the actual
//! expression structure from the canonical AST graph.
//!
//! A reference alone is not proof that the referenced node exists or that
//! the expression is semantically valid.
//!
//! ## Scalability and determinism
//!
//! This module imposes no universal limit on expression count, width,
//! hierarchy, hardware size, or target capacity. It introduces no fixed
//! collection capacity and performs no recursive expression traversal.
//! Any resource budgets belong to the caller's explicit operational policy.
//!
//! The role vocabulary is extensible. New source-level expression forms
//! remain owned by the canonical AST; adding a new HDL usage does not require
//! copying those forms into this module.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;

/// A semantic context in which an HDL construct uses a canonical expression.
///
/// The role describes why the expression is referenced. It does not determine
/// the expression's type, value, evaluation time, or target feasibility.
///
/// Extension roles allow new HDL features to be introduced without making
/// the built-in vocabulary a closed list.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HdlExpressionRole {
    /// An expression used as a hardware parameter or generic argument.
    Parameter,

    /// An expression used to describe a width or dimension.
    WidthOrDimension,

    /// An expression supplying a declaration's initial value.
    InitialValue,

    /// An expression supplying a combinational assignment's value.
    AssignmentValue,

    /// An expression used as a conditional or branch guard.
    Condition,

    /// An expression used to compute an index or address.
    IndexOrAddress,

    /// An expression used in a clock declaration or clock-related property.
    Clock,

    /// An expression used in a reset declaration or reset-related property.
    Reset,

    /// An expression used in a process sensitivity or event specification.
    EventOrSensitivity,

    /// An expression used to describe a timing property or timing constraint.
    Timing,

    /// An expression used in a resource requirement or constraint.
    ResourceRequirement,

    /// An expression used in an assertion, property, or verification contract.
    Verification,

    /// An expression used to control source-level hardware generation.
    GenerateCondition,

    /// An expression used as a memory initialization or access value.
    Memory,

    /// An expression used as a protocol or interface property.
    InterfaceOrProtocol,

    /// A role introduced by an HDL extension.
    ///
    /// The namespace and name identify the extension's contract. Their
    /// interpretation belongs to the extension and semantic layer.
    Extension {
        /// Stable, non-empty extension namespace.
        namespace: String,

        /// Stable, non-empty role name within the namespace.
        name: String,
    },
}

impl HdlExpressionRole {
    /// Construct an extension-defined expression role.
    ///
    /// Namespace and name must be non-empty and must not contain control
    /// characters. This validates identity syntax only; it does not check
    /// whether an extension is registered or supported.
    ///
    /// # Errors
    ///
    /// Returns [`HdlExpressionError::InvalidExtensionIdentity`] when either
    /// component is empty or contains a control character.
    pub fn extension(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, HdlExpressionError> {
        let namespace = namespace.into();
        let name = name.into();

        if !valid_identity_component(&namespace)
            || !valid_identity_component(&name)
        {
            return Err(HdlExpressionError::InvalidExtensionIdentity);
        }

        Ok(Self::Extension { namespace, name })
    }

    /// Return the extension namespace and role name, if this is an extension.
    #[must_use]
    pub fn extension_identity(&self) -> Option<(&str, &str)> {
        match self {
            Self::Extension { namespace, name } => {
                Some((namespace.as_str(), name.as_str()))
            }
            _ => None,
        }
    }
}

/// A reference associating a canonical AST expression with its HDL usage.
///
/// This type stores the canonical node identity rather than copying the
/// expression, its span, or its source text. The canonical AST graph remains
/// authoritative for all of those details.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HdlExpressionUse {
    expression: NodeId,
    role: HdlExpressionRole,
}

impl HdlExpressionUse {
    /// Create an HDL expression-use reference.
    ///
    /// This constructor deliberately does not attempt to resolve the node:
    /// the AST graph owns node existence and node-kind validation.
    #[must_use]
    pub fn new(expression: NodeId, role: HdlExpressionRole) -> Self {
        Self { expression, role }
    }

    /// Borrow the canonical AST node identity.
    #[must_use]
    pub fn expression(&self) -> &NodeId {
        &self.expression
    }

    /// Borrow the HDL usage role.
    #[must_use]
    pub fn role(&self) -> &HdlExpressionRole {
        &self.role
    }

    /// Consume this reference and return its canonical AST node identity.
    #[must_use]
    pub fn into_expression(self) -> NodeId {
        self.expression
    }

    /// Consume this reference and return its HDL usage role.
    #[must_use]
    pub fn into_role(self) -> HdlExpressionRole {
        self.role
    }
}

/// Errors raised while constructing HDL expression integration metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HdlExpressionError {
    /// An extension namespace or role name is empty or contains a control
    /// character.
    InvalidExtensionIdentity,
}

impl fmt::Display for HdlExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExtensionIdentity => formatter.write_str(
                "HDL expression extension namespace and name must be \
                 non-empty and contain no control characters",
            ),
        }
    }
}

impl std::error::Error for HdlExpressionError {}

fn valid_identity_component(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use super::{HdlExpressionError, HdlExpressionRole};

    #[test]
    fn extension_role_preserves_its_identity() {
        let role = HdlExpressionRole::extension(
            "vendor.example.hdl",
            "custom_timing_property",
        )
        .expect("valid extension identity");

        assert_eq!(
            role.extension_identity(),
            Some(("vendor.example.hdl", "custom_timing_property")),
        );
    }

    #[test]
    fn extension_role_rejects_empty_namespace() {
        let result = HdlExpressionRole::extension("", "custom_role");

        assert_eq!(
            result,
            Err(HdlExpressionError::InvalidExtensionIdentity),
        );
    }

    #[test]
    fn extension_role_rejects_whitespace_only_name() {
        let result = HdlExpressionRole::extension("example.hdl", "  ");

        assert_eq!(
            result,
            Err(HdlExpressionError::InvalidExtensionIdentity),
        );
    }

    #[test]
    fn extension_role_rejects_control_characters() {
        let result =
            HdlExpressionRole::extension("example.hdl", "custom\nrole");

        assert_eq!(
            result,
            Err(HdlExpressionError::InvalidExtensionIdentity),
        );
    }

    #[test]
    fn built_in_roles_have_no_extension_identity() {
        assert_eq!(
            HdlExpressionRole::WidthOrDimension.extension_identity(),
            None,
        );
    }
}
