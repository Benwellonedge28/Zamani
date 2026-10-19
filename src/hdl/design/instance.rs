
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Module Instances
//!
//! Path: `src/hdl/design/instance.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represents a logical instance of a reusable HDL module. An instance
//! records its canonical AST identity, local source-level name, referenced
//! qualified module name, generic-specialization arguments, logical port
//! connections, and canonical source span.
//!
//! This is a derived design-layer record. It is not an AST, symbol resolver,
//! generic evaluator, elaborator, connectivity graph, physical device
//! instance, netlist, or intermediate representation.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `grammar/hdl/hardware-generics.g4`
//! - `grammar/hdl/ports.g4`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/ast/node/source/span.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/module.rs`
//! - `src/hdl/design/port.rs`
//!
//! Grammar owns syntax. The canonical AST owns source nodes and their
//! identities. Shared semantic analysis owns name resolution, type checking,
//! and resolved meaning. `design/module.rs` owns module definitions and
//! `design/port.rs` owns module-boundary port declarations. This file owns
//! only the derived instance and binding records described below.
//!
//! ## Owns
//!
//! - Logical instance declarations and their local structural invariants.
//! - References to the canonical AST nodes for generic arguments and
//!   connection expressions.
//! - Ordered generic-specialization arguments.
//! - Ordered named or positional port connections.
//! - An ordered per-scope instance collection with identity/name lookup.
//! - Local duplicate-binding and duplicate-instance checks.
//!
//! ## Does not own
//!
//! - Lexing, parsing, or a replacement AST.
//! - Module-name resolution or a global module registry.
//! - Generic substitution, constant evaluation, or elaboration.
//! - Canonical type or width definitions.
//! - Port type/direction compatibility or expression type checking.
//! - Interface/protocol resolution.
//! - Hierarchy-cycle detection.
//! - Physical device selection, allocation, placement, or routing.
//! - Simulation, synthesis, or runtime execution.
//! - HDLIR structures, builders, passes, lowering, or serialization.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Grammar integration
//!
//! `hardware-modules.g4` defines the logical form:
//!
//!     instance counter: Counter;
//!     instance counter: Counter<WIDTH = width>;
//!     instance arithmetic: compute::Arithmetic<WIDTH = width> {
//!         a = lhs,
//!         b = rhs,
//!         result = output
//!     };
//!
//! The canonical AST adapter supplies the instance declaration NodeId and
//! Span, a validated `ModuleName`, and NodeIds for each generic argument and
//! connection expression. This model never parses source text or allocates
//! AST identities.
//!
//! Named and positional bindings are retained in source order. A mixture of
//! binding styles is preserved here because its legality depends on the
//! language's semantic rules. This file does not silently reorder arguments
//! or infer a different binding style.
//!
//! `Option<Vec<_>>` deliberately distinguishes an omitted specialization or
//! connection block from an explicitly present but empty one.
//!
//! ## Scalability and determinism
//!
//! There are no fixed limits on instances, generic arguments, connections,
//! name-segment counts, or hierarchy size. Collections grow according to
//! available resources. Operational budgets belong to compiler policy and
//! must be reported explicitly when they prevent complete analysis.
//!
//! The model preserves source order. Its scope collection uses ordered maps
//! for deterministic duplicate checks and lookup. It does not use physical
//! addresses, vendor IDs, or target resource indices as identities.
//!
//! Rust's address space, allocator, and representable IDs are finite. POCO-REAF
//! means that this module adds no artificial universal hardware ceiling.
//!
//! ## Validation boundary
//!
//! Local validation checks names, source-span ordering, module-name
//! structure, duplicate named bindings, and local AST-reference integrity.
//! It does not prove that referenced AST nodes exist or are expressions of
//! the required kind; those checks require the canonical AST store.
//!
//! It also does not establish that a referenced module exists, that generic
//! arguments match its declaration, or that connections match its ports.
//! Those are explicit semantic/elaboration checks. Unknown information must
//! remain unresolved rather than being guessed.
//!
//! ## Integration direction
//!
//! Canonical AST and parser adapter
//!     -> HdlModuleInstance
//!     -> per-module HdlInstanceList
//!     -> module resolution and generic binding
//!     -> port/connection validation
//!     -> hierarchy validation and elaboration
//!     -> HDLIR (`src/hdl/hdlir/`) -- deferred
//!
//! This module must not depend on semantic analysis, target realization,
//! runtime code, or HDLIR.
//!
//! ## Deserialization
//!
//! Deserialization can bypass constructors. Consumers accepting externally
//! deserialized instances must call `validate`; collections constructed from
//! such records must be rebuilt through `HdlInstanceList::from_instances`.
//!
//! ## Freeze criteria
//!
//! - Canonical AST NodeIds and Span are reused.
//! - Module references reuse `design/module.rs::ModuleName`.
//! - Argument and connection order is preserved.
//! - Omitted and explicitly empty lists remain distinguishable.
//! - Name resolution, type checking, elaboration, and hierarchy validation
//!   remain separate.
//! - No target-specific or HDLIR dependency is introduced.
//! - Unit tests and workspace checks pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;

use super::module::{ModuleName, ModuleNameError};

/// Schema version of the logical module-instance contract.
///
/// This is independent of the language, AST, semantic-model, and future
/// HDLIR schema versions.
pub const HDL_INSTANCE_SCHEMA_VERSION: u16 = 1;

/// One generic-specialization argument.
///
/// `name` is present for a named argument such as `WIDTH = value` and absent
/// for a positional argument such as `32`. `value_node` references the
/// canonical AST node representing the argument value or type-level argument.
/// The AST adapter must preserve the actual source expression in the AST.
///
/// This record does not evaluate the expression or establish whether its
/// name is declared by the referenced module.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HdlGenericArgument {
    name: Option<String>,
    value_node: NodeId,
}

impl HdlGenericArgument {
    /// Construct a generic argument.
    ///
    /// This validates a supplied name locally. It does not resolve the
    /// referenced AST node or validate the argument against a declaration.
    ///
    /// # Errors
    ///
    /// Returns [`HdlInstanceError::EmptyGenericArgumentName`] or
    /// [`HdlInstanceError::SurroundingWhitespaceInGenericArgumentName`]
    /// when a named argument has an invalid name.
    pub fn new(
        name: Option<String>,
        value_node: NodeId,
    ) -> Result<Self, HdlInstanceError> {
        let argument = Self { name, value_node };
        argument.validate()?;
        Ok(argument)
    }

    /// Return the optional formal generic name.
    ///
    /// `None` means the argument is positional.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the canonical AST identity of the argument value.
    #[must_use]
    pub const fn value_node(&self) -> NodeId {
        self.value_node
    }

    /// Validate local invariants, including after deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if the optional name is empty or has surrounding
    /// whitespace.
    pub fn validate(&self) -> Result<(), HdlInstanceError> {
        if let Some(name) = &self.name {
            validate_binding_name(
                name,
                HdlInstanceError::EmptyGenericArgumentName,
                HdlInstanceError::SurroundingWhitespaceInGenericArgumentName,
            )?;
        }

        Ok(())
    }
}

/// One logical connection to a referenced module's port.
///
/// A named connection, such as `data = source`, stores the formal port name.
/// A positional connection stores `None`; the semantic layer later resolves
/// its position against the referenced module's declared port order.
///
/// `expression_node` refers to the canonical AST node for the connected
/// expression. The expression is never copied into this record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HdlInstanceConnection {
    formal_name: Option<String>,
    expression_node: NodeId,
}

impl HdlInstanceConnection {
    /// Construct a named or positional logical connection.
    ///
    /// This does not resolve the formal port or type-check the expression.
    ///
    /// # Errors
    ///
    /// Returns [`HdlInstanceError::EmptyConnectionName`] or
    /// [`HdlInstanceError::SurroundingWhitespaceInConnectionName`]
    /// when a named connection has an invalid name.
    pub fn new(
        formal_name: Option<String>,
        expression_node: NodeId,
    ) -> Result<Self, HdlInstanceError> {
        let connection = Self {
            formal_name,
            expression_node,
        };
        connection.validate()?;
        Ok(connection)
    }

    /// Return the optional formal port name.
    ///
    /// `None` means the connection is positional.
    #[must_use]
    pub fn formal_name(&self) -> Option<&str> {
        self.formal_name.as_deref()
    }

    /// Return the canonical AST identity of the connected expression.
    #[must_use]
    pub const fn expression_node(&self) -> NodeId {
        self.expression_node
    }

    /// Validate local invariants, including after deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if the optional formal name is empty or has
    /// surrounding whitespace.
    pub fn validate(&self) -> Result<(), HdlInstanceError> {
        if let Some(name) = &self.formal_name {
            validate_binding_name(
                name,
                HdlInstanceError::EmptyConnectionName,
                HdlInstanceError::SurroundingWhitespaceInConnectionName,
            )?;
        }

        Ok(())
    }
}

/// A logical instance of a reusable HDL module.
///
/// The instance has an identity distinct from the referenced module's
/// declaration identity. Multiple instances may reference the same module
/// definition while retaining independent names, specializations, and
/// connections.
///
/// `generic_arguments` is `None` when no specialization syntax was supplied.
/// `Some(vec![])` preserves an explicitly supplied empty specialization.
/// `connections` has the corresponding meaning for the optional connection
/// block.
///
/// The AST remains the authority for original syntax and source structure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HdlModuleInstance {
    declaration: NodeId,
    name: String,
    module_reference: ModuleName,
    span: Span,
    generic_arguments: Option<Vec<HdlGenericArgument>>,
    connections: Option<Vec<HdlInstanceConnection>>,
}

impl HdlModuleInstance {
    /// Construct a logical module instance and validate local invariants.
    ///
    /// This does not resolve the module reference, perform generic
    /// substitution, check port compatibility, or elaborate hierarchy.
    ///
    /// # Errors
    ///
    /// Returns [`HdlInstanceError`] for an invalid name or module reference,
    /// an invalid source span, duplicate named bindings, or an argument or
    /// connection that refers to the instance declaration itself.
    pub fn new(
        declaration: NodeId,
        name: impl Into<String>,
        module_reference: ModuleName,
        span: Span,
        generic_arguments: Option<Vec<HdlGenericArgument>>,
        connections: Option<Vec<HdlInstanceConnection>>,
    ) -> Result<Self, HdlInstanceError> {
        let instance = Self {
            declaration,
            name: name.into(),
            module_reference,
            span,
            generic_arguments,
            connections,
        };

        instance.validate()?;
        Ok(instance)
    }

    /// Return the canonical AST identity of this instance declaration.
    #[must_use]
    pub const fn declaration_id(&self) -> NodeId {
        self.declaration
    }

    /// Return the source-level name of this instance within its containing
    /// scope.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified source-level name of the referenced module.
    ///
    /// This is a reference, not proof that the module exists or resolves
    /// uniquely in the active namespace.
    #[must_use]
    pub fn module_reference(&self) -> &ModuleName {
        &self.module_reference
    }

    /// Return the canonical source span of the instance declaration.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return generic arguments in source order, if specialization syntax
    /// was supplied.
    #[must_use]
    pub fn generic_arguments(&self) -> Option<&[HdlGenericArgument]> {
        self.generic_arguments.as_deref()
    }

    /// Return logical connections in source order, if a connection block
    /// was supplied.
    #[must_use]
    pub fn connections(&self) -> Option<&[HdlInstanceConnection]> {
        self.connections.as_deref()
    }

    /// Return whether the source supplied a generic-specialization clause.
    #[must_use]
    pub const fn has_generic_specialization(&self) -> bool {
        self.generic_arguments.is_some()
    }

    /// Return whether the source supplied a connection block.
    #[must_use]
    pub const fn has_connection_block(&self) -> bool {
        self.connections.is_some()
    }

    /// Validate local invariants, including after deserialization.
    ///
    /// This method does not access the canonical AST store and cannot prove
    /// that referenced node IDs exist or refer to the correct AST node kinds.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic order.
    pub fn validate(&self) -> Result<(), HdlInstanceError> {
        validate_binding_name(
            &self.name,
            HdlInstanceError::EmptyInstanceName,
            HdlInstanceError::SurroundingWhitespaceInInstanceName,
        )?;

        self.module_reference
            .validate()
            .map_err(HdlInstanceError::InvalidModuleName)?;

        if self.span.end_raw() < self.span.start_raw() {
            return Err(HdlInstanceError::InvalidSourceSpan {
                start: self.span.start_raw(),
                end: self.span.end_raw(),
            });
        }

        if let Some(arguments) = &self.generic_arguments {
            let mut names = BTreeMap::<&str, usize>::new();

            for (index, argument) in arguments.iter().enumerate() {
                argument.validate()?;

                if argument.value_node == self.declaration {
                    return Err(HdlInstanceError::SelfReference {
                        role: InstanceReferenceRole::GenericArgument,
                        node: self.declaration,
                    });
                }

                if let Some(name) = argument.name() {
                    if names.insert(name, index).is_some() {
                        return Err(HdlInstanceError::DuplicateGenericArgumentName {
                            name: name.to_owned(),
                        });
                    }
                }
            }
        }

        if let Some(connections) = &self.connections {
            let mut names = BTreeMap::<&str, usize>::new();

            for (index, connection) in connections.iter().enumerate() {
                connection.validate()?;

                if connection.expression_node == self.declaration {
                    return Err(HdlInstanceError::SelfReference {
                        role: InstanceReferenceRole::ConnectionExpression,
                        node: self.declaration,
                    });
                }

                if let Some(name) = connection.formal_name() {
                    if names.insert(name, index).is_some() {
                        return Err(HdlInstanceError::DuplicateConnectionName {
                            name: name.to_owned(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

/// An ordered collection of instances belonging to one logical scope.
///
/// This collection detects duplicate instance identities and local names.
/// It does not resolve the referenced module names or replace the canonical
/// AST's ordered module-body membership.
///
/// Each collection is intended to represent one containing scope. Instances
/// with the same local name may exist in different scopes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlInstanceList {
    instances: Vec<HdlModuleInstance>,
    by_id: BTreeMap<NodeId, usize>,
    by_name: BTreeMap<String, usize>,
}

impl HdlInstanceList {
    /// Create an empty instance collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a collection from instances in source order.
    ///
    /// # Errors
    ///
    /// Returns the first invalid instance, duplicate identity, or duplicate
    /// local name. Input order determines which invalid entry is reported.
    pub fn from_instances(
        instances: Vec<HdlModuleInstance>,
    ) -> Result<Self, HdlInstanceListError> {
        let mut list = Self::new();

        for instance in instances {
            list.push(instance)?;
        }

        Ok(list)
    }

    /// Append an instance while preserving source order.
    ///
    /// The collection is unchanged when validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the instance is invalid or its identity/name
    /// already exists in this scope.
    pub fn push(
        &mut self,
        instance: HdlModuleInstance,
    ) -> Result<(), HdlInstanceListError> {
        instance
            .validate()
            .map_err(HdlInstanceListError::InvalidInstance)?;

        let id = instance.declaration_id();
        let name = instance.name().to_owned();

        if self.by_id.contains_key(&id) {
            return Err(HdlInstanceListError::DuplicateIdentity { id });
        }

        if self.by_name.contains_key(&name) {
            return Err(HdlInstanceListError::DuplicateName { name });
        }

        let index = self.instances.len();
        self.instances.push(instance);
        self.by_id.insert(id, index);
        self.by_name.insert(name, index);

        Ok(())
    }

    /// Return instances in source declaration order.
    #[must_use]
    pub fn as_slice(&self) -> &[HdlModuleInstance] {
        &self.instances
    }

    /// Return the number of instances in this scope.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Find an instance by its canonical declaration identity.
    #[must_use]
    pub fn get_by_id(&self, id: NodeId) -> Option<&HdlModuleInstance> {
        self.by_id.get(&id).map(|index| &self.instances[*index])
    }

    /// Find an instance by its exact local source-level name.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&HdlModuleInstance> {
        self.by_name
            .get(name)
            .map(|index| &self.instances[*index])
    }
}

/// The role of an AST reference found in an instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceReferenceRole {
    /// A generic-specialization argument node.
    GenericArgument,
    /// A connected port expression node.
    ConnectionExpression,
}

impl fmt::Display for InstanceReferenceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericArgument => f.write_str("generic argument"),
            Self::ConnectionExpression => f.write_str("connection expression"),
        }
    }
}

/// Errors raised while constructing or validating a logical instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlInstanceError {
    /// The instance name is empty or whitespace-only.
    EmptyInstanceName,
    /// The instance name has leading or trailing whitespace.
    SurroundingWhitespaceInInstanceName,
    /// A named generic argument has an empty name.
    EmptyGenericArgumentName,
    /// A named generic argument has surrounding whitespace.
    SurroundingWhitespaceInGenericArgumentName,
    /// A named connection has an empty formal name.
    EmptyConnectionName,
    /// A named connection has surrounding whitespace.
    SurroundingWhitespaceInConnectionName,
    /// The referenced module name is structurally invalid.
    InvalidModuleName(ModuleNameError),
    /// Two named generic arguments use the same name.
    DuplicateGenericArgumentName {
        /// Repeated generic name.
        name: String,
    },
    /// Two named connections use the same formal port name.
    DuplicateConnectionName {
        /// Repeated formal port name.
        name: String,
    },
    /// An AST reference incorrectly points to the instance declaration itself.
    SelfReference {
        /// The role of the invalid reference.
        role: InstanceReferenceRole,
        /// The invalid AST identity.
        node: NodeId,
    },
    /// A deserialized or externally supplied source span has reversed bounds.
    InvalidSourceSpan {
        /// Inclusive source offset.
        start: u64,
        /// Exclusive source offset.
        end: u64,
    },
}

impl fmt::Display for HdlInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInstanceName => {
                f.write_str("HDL instance name cannot be empty")
            }
            Self::SurroundingWhitespaceInInstanceName => {
                f.write_str("HDL instance name cannot have surrounding whitespace")
            }
            Self::EmptyGenericArgumentName => {
                f.write_str("HDL generic argument name cannot be empty")
            }
            Self::SurroundingWhitespaceInGenericArgumentName => {
                f.write_str(
                    "HDL generic argument name cannot have surrounding whitespace",
                )
            }
            Self::EmptyConnectionName => {
                f.write_str("HDL connection name cannot be empty")
            }
            Self::SurroundingWhitespaceInConnectionName => {
                f.write_str("HDL connection name cannot have surrounding whitespace")
            }
            Self::InvalidModuleName(error) => {
                write!(f, "invalid referenced HDL module name: {error}")
            }
            Self::DuplicateGenericArgumentName { name } => {
                write!(f, "duplicate HDL generic argument `{name}`")
            }
            Self::DuplicateConnectionName { name } => {
                write!(f, "duplicate HDL instance connection `{name}`")
            }
            Self::SelfReference { role, node } => {
                write!(
                    f,
                    "HDL instance declaration refers to itself as a {role} (NodeId {})",
                    node.get()
                )
            }
            Self::InvalidSourceSpan { start, end } => {
                write!(
                    f,
                    "invalid HDL instance source span: start {start} exceeds end {end}"
                )
            }
        }
    }
}

impl Error for HdlInstanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidModuleName(error) => Some(error),
            _ => None,
        }
    }
}

/// Errors raised while constructing or validating an instance collection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlInstanceListError {
    /// An instance failed local validation.
    InvalidInstance(HdlInstanceError),
    /// Two instances in the same scope use the same canonical identity.
    DuplicateIdentity {
        /// Repeated AST identity.
        id: NodeId,
    },
    /// Two instances in the same scope use the same local name.
    DuplicateName {
        /// Repeated source-level name.
        name: String,
    },
}

impl fmt::Display for HdlInstanceListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstance(error) => {
                write!(f, "invalid HDL module instance: {error}")
            }
            Self::DuplicateIdentity { id } => {
                write!(f, "duplicate HDL instance declaration identity (NodeId {})", id.get())
            }
            Self::DuplicateName { name } => {
                write!(f, "duplicate HDL instance name `{name}` in one scope")
            }
        }
    }
}

impl Error for HdlInstanceListError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidInstance(error) => Some(error),
            Self::DuplicateIdentity { .. } | Self::DuplicateName { .. } => None,
        }
    }
}

fn validate_binding_name(
    name: &str,
    empty_error: HdlInstanceError,
    whitespace_error: HdlInstanceError,
) -> Result<(), HdlInstanceError> {
    if name.trim().is_empty() {
        return Err(empty_error);
    }

    if name.trim() != name {
        return Err(whitespace_error);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{SourceId, SourceOffset};

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(10),
        )
        .expect("test span must be valid")
    }

    fn module_name(name: &str) -> ModuleName {
        ModuleName::parse_qualified(name).expect("test module name must be valid")
    }

    fn instance(
        id: u64,
        name: &str,
        module: &str,
    ) -> HdlModuleInstance {
        HdlModuleInstance::new(
            node(id),
            name,
            module_name(module),
            span(),
            None,
            None,
        )
        .expect("test instance must be valid")
    }

    #[test]
    fn preserves_optional_empty_specialization_and_connection_block() {
        let value = HdlModuleInstance::new(
            node(1),
            "child",
            module_name("logic::Child"),
            span(),
            Some(Vec::new()),
            Some(Vec::new()),
        )
        .expect("explicit empty clauses are structurally valid");

        assert!(value.has_generic_specialization());
        assert!(value.has_connection_block());
        assert_eq!(value.generic_arguments(), Some([].as_slice()));
        assert_eq!(value.connections(), Some([].as_slice()));
    }

    #[test]
    fn preserves_named_and_positional_arguments_in_order() {
        let args = vec![
            HdlGenericArgument::new(None, node(2)).expect("positional argument"),
            HdlGenericArgument::new(Some("WIDTH".to_owned()), node(3))
                .expect("named argument"),
        ];

        let value = HdlModuleInstance::new(
            node(1),
            "child",
            module_name("Child"),
            span(),
            Some(args),
            None,
        )
        .expect("binding style is checked by semantic analysis");

        let stored = value.generic_arguments().expect("specialization exists");
        assert_eq!(stored.len(), 2);
        assert_eq!(stored[0].name(), None);
        assert_eq!(stored[1].name(), Some("WIDTH"));
    }

    #[test]
    fn rejects_duplicate_named_generic_arguments() {
        let result = HdlModuleInstance::new(
            node(1),
            "child",
            module_name("Child"),
            span(),
            Some(vec![
                HdlGenericArgument::new(Some("WIDTH".to_owned()), node(2))
                    .expect("first argument"),
                HdlGenericArgument::new(Some("WIDTH".to_owned()), node(3))
                    .expect("second argument"),
            ]),
            None,
        );

        assert!(matches!(
            result,
            Err(HdlInstanceError::DuplicateGenericArgumentName { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_named_connections() {
        let result = HdlModuleInstance::new(
            node(1),
            "child",
            module_name("Child"),
            span(),
            None,
            Some(vec![
                HdlInstanceConnection::new(Some("input_a".to_owned()), node(2))
                    .expect("first connection"),
                HdlInstanceConnection::new(Some("input_a".to_owned()), node(3))
                    .expect("second connection"),
            ]),
        );

        assert!(matches!(
            result,
            Err(HdlInstanceError::DuplicateConnectionName { .. })
        ));
    }

    #[test]
    fn instance_list_preserves_order_and_supports_lookup() {
        let first = instance(1, "left", "Gate");
        let second = instance(2, "right", "Gate");

        let list = HdlInstanceList::from_instances(vec![first, second])
            .expect("distinct instances must be accepted");

        assert_eq!(list.len(), 2);
        assert_eq!(list.as_slice()[0].name(), "left");
        assert_eq!(list.as_slice()[1].name(), "right");
        assert_eq!(list.get_by_name("right").map(HdlModuleInstance::name), Some("right"));
        assert_eq!(list.get_by_id(node(1)).map(HdlModuleInstance::name), Some("left"));
    }

    #[test]
    fn instance_list_rejects_duplicate_local_names() {
        let result = HdlInstanceList::from_instances(vec![
            instance(1, "child", "Gate"),
            instance(2, "child", "Gate"),
        ]);

        assert!(matches!(
            result,
            Err(HdlInstanceListError::DuplicateName { .. })
        ));
    }

    #[test]
    fn instance_list_rejects_duplicate_declaration_ids() {
        let result = HdlInstanceList::from_instances(vec![
            instance(1, "left", "Gate"),
            instance(1, "right", "Gate"),
        ]);

        assert!(matches!(
            result,
            Err(HdlInstanceListError::DuplicateIdentity { .. })
        ));
    }

    #[test]
    fn rejects_invalid_instance_name() {
        let result = HdlModuleInstance::new(
            node(1),
            " child ",
            module_name("Child"),
            span(),
            None,
            None,
        );

        assert_eq!(
            result.err(),
            Some(HdlInstanceError::SurroundingWhitespaceInInstanceName)
        );
    }

    #[test]
    fn rejects_self_reference_as_connection_expression() {
        let result = HdlModuleInstance::new(
            node(1),
            "child",
            module_name("Child"),
            span(),
            None,
            Some(vec![
                HdlInstanceConnection::new(None, node(1))
                    .expect("connection record is locally valid"),
            ]),
        );

        assert!(matches!(
            result,
            Err(HdlInstanceError::SelfReference {
                role: InstanceReferenceRole::ConnectionExpression,
                ..
            })
        ));
    }
}
