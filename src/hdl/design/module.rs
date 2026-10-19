
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Module Definition
//!
//! Path: `src/hdl/design/module.rs`
//! Rust baseline: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represents a validated, target-independent HDL module definition in the
//! HDL design layer. It records the identity and location of the canonical
//! source declaration and references its generic-parameter and body-member
//! AST nodes without copying those nodes.
//!
//! This type is a derived design-layer record. It is not a replacement AST,
//! parser representation, semantic symbol table, or intermediate
//! representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `grammar/hdl/hardware-generics.g4`, where present in the active grammar
//! - `src/frontend/ast/node/program/module.rs`
//! - `src/frontend/ast/node/program/item.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar owns source syntax. The canonical AST owns source nodes and
//! their identities. Shared semantic analysis owns name resolution and
//! resolved meaning. This module owns only the HDL design-layer record and
//! its local structural invariants.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - the logical qualified name retained for this design record;
//! - the canonical AST identity of the module declaration;
//! - references to generic-parameter declaration nodes;
//! - ordered references to module-body member nodes;
//! - the canonical source span associated with the declaration;
//! - local reference-integrity validation.
//!
//! It does not own or duplicate the referenced AST nodes.
//!
//! ## Does not own
//!
//! This module does not implement:
//!
//! - lexing, parsing, or a second AST;
//! - name resolution or a global module registry;
//! - generic substitution or elaboration;
//! - port, signal, memory, process, or instance semantics;
//! - type checking or symbolic expression evaluation;
//! - physical device selection or resource allocation;
//! - synthesis, placement, routing, or timing closure;
//! - HDLIR, builders, passes, lowering, or serialization formats for HDLIR.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Grammar integration
//!
//! `grammar/hdl/hardware-modules.g4` defines a module declaration as a
//! logical qualified name, optional generic parameters, and a module body.
//! The body is an ordered sequence of members dispatched to their respective
//! grammar owners.
//!
//! Consequently:
//!
//! - qualified names are represented as ordered identifier segments;
//! - generic parameters are references to canonical AST nodes;
//! - body members are references to canonical AST nodes in source order;
//! - member kinds remain open-ended and are not encoded in a closed enum.
//!
//! The parser/AST adapter must supply the references after constructing the
//! canonical AST nodes. This module does not parse text or allocate NodeIds.
//!
//! ## Scalability
//!
//! There are no fixed limits on module-name segments, generic parameters,
//! or body members. Collections grow according to available memory and the
//! caller's operational policy. No machine, vendor, resource, or target
//! capacity is inferred from a module definition.
//!
//! Rust's address space, allocator, and representable identifiers are finite.
//! POCO-REAF means this type introduces no additional artificial
//! language-level ceiling.
//!
//! ## Determinism
//!
//! Generic-parameter and body-member ordering is preserved exactly.
//! Validation reports the first structural error in deterministic traversal
//! order. No global mutable state, random IDs, or unordered public collections
//! are used.
//!
//! ## Integration direction
//!
//! ```text
//! HDL grammar
//!     |
//!     v
//! HDL parser / canonical AST adapter
//!     |
//!     v
//! canonical AST nodes and NodeIds
//!     |
//!     v
//! HdlModule
//!     |
//!     v
//! structural and semantic validation
//!     |
//!     v
//! hardware semantic model
//!     |
//!     v
//! HDLIR (`src/hdl/hdlir/`) -- deferred
//!     |
//!     v
//! target realization
//! ```
//!
//! This module must not import semantic analysis, compiler orchestration,
//! target realization, runtime code, or HDLIR.
//!
//! ## Deserialization contract
//!
//! Deserialization can construct a value without running `new`. Consumers
//! accepting deserialized data must call `validate` before relying on local
//! invariants. Cross-node checks still require the canonical AST store.
//!
//! ## Freeze criteria
//!
//! - The canonical AST adapter supplies declaration identity, name, span,
//!   generic references, and body references without copying AST nodes.
//! - Every referenced NodeId resolves in the canonical AST store.
//! - Structural and semantic validation remain separate.
//! - The public API compiles with the repository's actual module exports.
//! - Unit tests and workspace checks pass.

use core::fmt;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;

/// Schema version for this in-memory HDL design record.
///
/// This is independent of the Zamani language version, the AST serialization
/// schema, and any future HDLIR schema.
pub const HDL_MODULE_SCHEMA_VERSION: u16 = 1;

/// A qualified logical HDL module name.
///
/// Each element is one identifier segment. The separator `::` is structural
/// and is never stored inside a segment.
///
/// This type preserves the source-level qualified name. It does not establish
/// whether the name resolves to a declaration in a particular namespace.
///
/// Lexical identifier validity remains the responsibility of the canonical
/// lexer and grammar.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ModuleName {
    segments: Vec<String>,
}

impl ModuleName {
    /// Construct a qualified module name from its identifier segments.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleNameError`] if there are no segments or if a segment
    /// is empty, consists only of whitespace, contains surrounding
    /// whitespace, or contains the qualified-name separator `::`.
    pub fn new<I, S>(segments: I) -> Result<Self, ModuleNameError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let segments: Vec<String> = segments.into_iter().map(Into::into).collect();

        if segments.is_empty() {
            return Err(ModuleNameError::EmptyName);
        }

        for (index, segment) in segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(ModuleNameError::EmptySegment { index });
            }

            if segment.trim() != segment {
                return Err(ModuleNameError::SurroundingWhitespace { index });
            }

            if segment.contains("::") {
                return Err(ModuleNameError::EmbeddedSeparator { index });
            }
        }

        Ok(Self { segments })
    }

    /// Parse the structural form of a qualified name such as
    /// `library::arithmetic::adder`.
    ///
    /// This splits the name on `::`; it does not replace or bypass the
    /// canonical lexer/parser's identifier validation.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleNameError`] for an empty name or invalid segment.
    pub fn parse_qualified(value: &str) -> Result<Self, ModuleNameError> {
        Self::new(value.split("::"))
    }

    /// Return the ordered identifier segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Return the number of segments in the qualified name.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Return the final identifier segment.
    ///
    /// A valid `ModuleName` always contains at least one segment.
    #[must_use]
    pub fn leaf(&self) -> &str {
        // Construction and validation guarantee a non-empty segment list.
        self.segments
            .last()
            .map(String::as_str)
            .unwrap_or("")
    }

    /// Return the canonical structural representation using `::`.
    #[must_use]
    pub fn as_qualified_string(&self) -> String {
        self.segments.join("::")
    }

    /// Revalidate this value, including values obtained through deserialization.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleNameError`] if the segments violate the constructor
    /// invariants.
    pub fn validate(&self) -> Result<(), ModuleNameError> {
        if self.segments.is_empty() {
            return Err(ModuleNameError::EmptyName);
        }

        for (index, segment) in self.segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(ModuleNameError::EmptySegment { index });
            }

            if segment.trim() != segment {
                return Err(ModuleNameError::SurroundingWhitespace { index });
            }

            if segment.contains("::") {
                return Err(ModuleNameError::EmbeddedSeparator { index });
            }
        }

        Ok(())
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                f.write_str("::")?;
            }
            f.write_str(segment)?;
        }

        Ok(())
    }
}

/// Errors raised while constructing or validating a qualified module name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModuleNameError {
    /// The name contains no identifier segments.
    EmptyName,
    /// A segment is empty or contains only whitespace.
    EmptySegment {
        /// Zero-based index of the invalid segment.
        index: usize,
    },
    /// A segment contains leading or trailing whitespace.
    SurroundingWhitespace {
        /// Zero-based index of the invalid segment.
        index: usize,
    },
    /// A segment contains `::`, which is reserved as the path separator.
    EmbeddedSeparator {
        /// Zero-based index of the invalid segment.
        index: usize,
    },
}

impl fmt::Display for ModuleNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("HDL module name must contain a segment"),
            Self::EmptySegment { index } => {
                write!(f, "HDL module name segment {index} is empty")
            }
            Self::SurroundingWhitespace { index } => {
                write!(
                    f,
                    "HDL module name segment {index} contains surrounding whitespace"
                )
            }
            Self::EmbeddedSeparator { index } => {
                write!(
                    f,
                    "HDL module name segment {index} contains the reserved `::` separator"
                )
            }
        }
    }
}

impl std::error::Error for ModuleNameError {}

/// The category of a canonical AST reference held by an HDL module record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleReferenceRole {
    /// A generic-parameter declaration in the module header.
    GenericParameter,
    /// A declaration or construct in the ordered module body.
    BodyMember,
}

impl fmt::Display for ModuleReferenceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericParameter => f.write_str("generic parameter"),
            Self::BodyMember => f.write_str("body member"),
        }
    }
}

/// Errors raised by local HDL module construction or validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlModuleError {
    /// The qualified name is invalid.
    InvalidName(ModuleNameError),
    /// A reference points to the module declaration itself.
    SelfReference {
        /// The role in which the invalid reference occurs.
        role: ModuleReferenceRole,
        /// The offending canonical AST node identity.
        node: NodeId,
    },
    /// A reference occurs more than once in the same role.
    DuplicateReference {
        /// The role in which the duplicate occurs.
        role: ModuleReferenceRole,
        /// The repeated canonical AST node identity.
        node: NodeId,
    },
    /// A single AST node is listed both as a generic parameter and a body member.
    ReferenceInMultipleRoles {
        /// The canonical AST node identity used in both roles.
        node: NodeId,
    },
}

impl fmt::Display for HdlModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => write!(f, "invalid HDL module name: {error}"),
            Self::SelfReference { role, node } => {
                write!(
                    f,
                    "HDL module declaration references itself as a {role} (NodeId {})",
                    node.get()
                )
            }
            Self::DuplicateReference { role, node } => {
                write!(
                    f,
                    "duplicate HDL module {role} reference (NodeId {})",
                    node.get()
                )
            }
            Self::ReferenceInMultipleRoles { node } => {
                write!(
                    f,
                    "HDL module AST node is used as both a generic parameter and a body member (NodeId {})",
                    node.get()
                )
            }
        }
    }
}

impl std::error::Error for HdlModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidName(error) => Some(error),
            _ => None,
        }
    }
}

/// A target-independent HDL module definition.
///
/// This is a derived design-layer record, not an AST node. The canonical AST
/// store remains the sole owner of the source nodes identified by `declaration`,
/// `generic_parameters`, and `body_members`.
///
/// `declaration` is the canonical AST identity of the module declaration.
/// It is stable within that AST graph, not a globally persistent identity
/// across unrelated compilations.
///
/// `generic_parameters` preserves header declaration order.
/// `body_members` preserves source order and may reference any HDL construct
/// admitted by the grammar. This structure deliberately does not enumerate
/// all possible member kinds.
///
/// The stored span uses the canonical AST's source-coordinate system.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HdlModule {
    declaration: NodeId,
    name: ModuleName,
    span: Span,
    generic_parameters: Vec<NodeId>,
    body_members: Vec<NodeId>,
}

impl HdlModule {
    /// Construct a module record and validate its local structural invariants.
    ///
    /// This function does not resolve names, inspect the AST store, perform
    /// type checking, elaborate generics, or determine target feasibility.
    ///
    /// # Errors
    ///
    /// Returns [`HdlModuleError`] for an invalid name, self-reference,
    /// duplicate reference, or a reference used in multiple roles.
    pub fn new(
        declaration: NodeId,
        name: ModuleName,
        span: Span,
        generic_parameters: Vec<NodeId>,
        body_members: Vec<NodeId>,
    ) -> Result<Self, HdlModuleError> {
        let module = Self {
            declaration,
            name,
            span,
            generic_parameters,
            body_members,
        };

        module.validate()?;
        Ok(module)
    }

    /// Return the canonical AST identity of this module declaration.
    #[must_use]
    pub const fn declaration_id(&self) -> NodeId {
        self.declaration
    }

    /// Return the logical qualified module name.
    #[must_use]
    pub fn name(&self) -> &ModuleName {
        &self.name
    }

    /// Return the canonical source span associated with the declaration.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return generic-parameter AST references in declaration order.
    #[must_use]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Return module-body AST references in source order.
    ///
    /// The references are not copies of the referenced nodes. Resolve them
    /// through the canonical AST store.
    #[must_use]
    pub fn body_members(&self) -> &[NodeId] {
        &self.body_members
    }

    /// Return the number of generic-parameter references.
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Return the number of body-member references.
    #[must_use]
    pub fn body_member_count(&self) -> usize {
        self.body_members.len()
    }

    /// Validate local invariants, including after deserialization.
    ///
    /// This check does not verify that the referenced IDs exist in the
    /// canonical AST store or that their concrete node kinds are appropriate.
    /// Those checks require the AST store and belong to structural validation.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic order.
    pub fn validate(&self) -> Result<(), HdlModuleError> {
        self.name
            .validate()
            .map_err(HdlModuleError::InvalidName)?;

        let mut seen = BTreeSet::new();

        for node in &self.generic_parameters {
            if *node == self.declaration {
                return Err(HdlModuleError::SelfReference {
                    role: ModuleReferenceRole::GenericParameter,
                    node: *node,
                });
            }

            if !seen.insert(*node) {
                return Err(HdlModuleError::DuplicateReference {
                    role: ModuleReferenceRole::GenericParameter,
                    node: *node,
                });
            }
        }

        for node in &self.body_members {
            if *node == self.declaration {
                return Err(HdlModuleError::SelfReference {
                    role: ModuleReferenceRole::BodyMember,
                    node: *node,
                });
            }

            if !seen.insert(*node) {
                if self.generic_parameters.contains(node) {
                    return Err(HdlModuleError::ReferenceInMultipleRoles {
                        node: *node,
                    });
                }

                return Err(HdlModuleError::DuplicateReference {
                    role: ModuleReferenceRole::BodyMember,
                    node: *node,
                });
            }
        }

        Ok(())
    }

    /// Return whether the module references the supplied AST node as a
    /// generic parameter or body member.
    #[must_use]
    pub fn references(&self, node: NodeId) -> bool {
        self.generic_parameters.contains(&node) || self.body_members.contains(&node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::SourceId;

    fn node_id(raw: u64) -> NodeId {
        NodeId::new(raw).expect("test NodeId must be non-zero")
    }

    fn test_span() -> Span {
        Span::point_usize(SourceId::from_raw(1), 0)
            .expect("zero offset is representable")
    }

    fn module(
        generic_parameters: Vec<NodeId>,
        body_members: Vec<NodeId>,
    ) -> Result<HdlModule, HdlModuleError> {
        HdlModule::new(
            node_id(1),
            ModuleName::parse_qualified("math::adder")
                .expect("test name is valid"),
            test_span(),
            generic_parameters,
            body_members,
        )
    }

    #[test]
    fn qualified_names_preserve_segment_order() {
        let name = ModuleName::parse_qualified("library::arithmetic::adder")
            .expect("qualified name should be valid");

        assert_eq!(
            name.segments(),
            &[
                "library".to_owned(),
                "arithmetic".to_owned(),
                "adder".to_owned()
            ]
        );
        assert_eq!(name.leaf(), "adder");
        assert_eq!(name.to_string(), "library::arithmetic::adder");
    }

    #[test]
    fn rejects_empty_or_malformed_qualified_names() {
        assert_eq!(
            ModuleName::parse_qualified(""),
            Err(ModuleNameError::EmptySegment { index: 0 })
        );
        assert_eq!(
            ModuleName::parse_qualified("library::::adder"),
            Err(ModuleNameError::EmptySegment { index: 1 })
        );
        assert_eq!(
            ModuleName::new(["library", " adder"]),
            Err(ModuleNameError::SurroundingWhitespace { index: 1 })
        );
    }

    #[test]
    fn accepts_empty_module_body() {
        let result = module(Vec::new(), Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().body_member_count(), 0);
    }

    #[test]
    fn preserves_generic_and_body_order() {
        let result = module(
            vec![node_id(2), node_id(3)],
            vec![node_id(4), node_id(5), node_id(6)],
        )
        .expect("module should be valid");

        assert_eq!(
            result.generic_parameters(),
            &[node_id(2), node_id(3)]
        );
        assert_eq!(
            result.body_members(),
            &[node_id(4), node_id(5), node_id(6)]
        );
    }

    #[test]
    fn rejects_self_references() {
        assert_eq!(
            module(vec![node_id(1)], Vec::new()),
            Err(HdlModuleError::SelfReference {
                role: ModuleReferenceRole::GenericParameter,
                node: node_id(1),
            })
        );

        assert_eq!(
            module(Vec::new(), vec![node_id(1)]),
            Err(HdlModuleError::SelfReference {
                role: ModuleReferenceRole::BodyMember,
                node: node_id(1),
            })
        );
    }

    #[test]
    fn rejects_duplicate_generic_parameter_references() {
        assert_eq!(
            module(vec![node_id(2), node_id(2)], Vec::new()),
            Err(HdlModuleError::DuplicateReference {
                role: ModuleReferenceRole::GenericParameter,
                node: node_id(2),
            })
        );
    }

    #[test]
    fn rejects_duplicate_body_member_references() {
        assert_eq!(
            module(Vec::new(), vec![node_id(2), node_id(2)]),
            Err(HdlModuleError::DuplicateReference {
                role: ModuleReferenceRole::BodyMember,
                node: node_id(2),
            })
        );
    }

    #[test]
    fn rejects_a_node_reused_across_roles() {
        assert_eq!(
            module(vec![node_id(2)], vec![node_id(2)]),
            Err(HdlModuleError::ReferenceInMultipleRoles {
                node: node_id(2),
            })
        );
    }

    #[test]
    fn reference_queries_are_consistent() {
        let result = module(vec![node_id(2)], vec![node_id(3)])
            .expect("module should be valid");

        assert!(result.references(node_id(2)));
        assert!(result.references(node_id(3)));
        assert!(!result.references(node_id(4)));
    }

    #[test]
    fn schema_version_is_stable_and_explicit() {
        assert_eq!(HDL_MODULE_SCHEMA_VERSION, 1);
    }
}
