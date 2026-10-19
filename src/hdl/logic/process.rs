
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Process Contracts
//!
//! Path: `src/hdl/logic/process.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent contracts for HDL processes. A process is a
//! source-level behavioral construct whose body is represented by references
//! to the canonical frontend AST.
//!
//! The model preserves:
//!
//! - the canonical process identity;
//! - the original process keyword and optional name;
//! - the semantic process classification;
//! - optional sensitivity expressions in source order;
//! - process-level attributes;
//! - ordered body items, including statements, bindings, nested blocks,
//!   and attributes;
//! - canonical AST references for downstream validation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/combinational.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/expressions.rs`
//! - `src/hdl/logic/sequential.rs`
//! - `src/hdl/logic/assignment.rs`
//!
//! The grammar owns process syntax. The canonical AST owns syntax nodes,
//! node identity, and source spans. The shared semantic layer owns name
//! resolution and source-language meaning. This module owns the process
//! integration contract and does not replace those authorities.
//!
//! ## Does not own
//!
//! - Parsing or a replacement AST.
//! - A second general-purpose statement or expression representation.
//! - Name resolution, type checking, or width resolution.
//! - Clock and reset declarations or clock-domain analysis.
//! - Sequential event semantics or assignment compatibility.
//! - Simulation scheduling, synthesis, placement, or routing.
//! - Physical resources, hardware allocation, or target selection.
//! - HDLIR structures, builders, verification, passes, serialization,
//!   or lowering. All HDLIR implementation remains in `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The parser/AST adapter maps each parsed process into this contract.
//! Statement, nested-block, and attribute references must point to nodes in
//! the canonical AST. Process bindings preserve their property spelling and
//! canonical expression reference; their legality and meaning are decided by
//! semantic analysis.
//!
//! The adapter maps a process to the existing `HdlProcessKind`. A semantic
//! checker must resolve process keywords, sensitivity expressions, bindings,
//! and referenced AST nodes using canonical semantic services.
//!
//! Sequential event validation remains in `sequential.rs`; assignment
//! validation remains in `assignment.rs`. This module does not duplicate
//! either validator or infer process behavior from a keyword's spelling.
//!
//! ## Scalability
//!
//! This module imposes no language-level maximum on processes, body items,
//! sensitivity expressions, attributes, bindings, or nested blocks.
//! Collections are dynamically sized. Operational budgets, cancellation,
//! and incomplete-analysis reporting belong to the caller.
//!
//! The model does not recursively traverse nested AST nodes. A downstream
//! traversal must use the repository's canonical traversal facilities or an
//! explicitly budgeted iterative traversal.
//!
//! ## Freeze criteria
//!
//! - AST references are canonical and validated by the AST integration layer.
//! - Source ordering is preserved wherever ordering is observable.
//! - Process classification is supplied by semantic analysis.
//! - Unknown process semantics remain indeterminate rather than valid.
//! - Shared statement and expression ownership is not duplicated.
//! - No target-specific behavior or HDLIR implementation is introduced.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::{HdlId, HdlProcessKind};
use crate::hdl::expressions::{HdlExpressionRole, HdlExpressionUse};

/// Version of the public HDL process-contract API.
///
/// This version is independent of the language version and HDLIR schema
/// version.
pub const HDL_PROCESS_CONTRACT_VERSION: u16 = 1;

/// Errors detected while constructing a local HDL process contract.
///
/// These errors concern the shape of this module's API inputs. They do not
/// replace parser diagnostics, canonical AST validation, or semantic errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProcessError {
    /// A process keyword is empty or contains a control character.
    InvalidProcessKeyword,

    /// A sensitivity expression has an incompatible HDL expression role.
    InvalidSensitivityExpressionRole,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProcessKeyword => {
                formatter.write_str("HDL process keyword is invalid")
            }
            Self::InvalidSensitivityExpressionRole => formatter.write_str(
                "HDL process sensitivity entries must use the \
                 EventOrSensitivity expression role",
            ),
        }
    }
}

impl std::error::Error for ProcessError {}

/// A process sensitivity list referencing canonical expression nodes.
///
/// The list is dynamically sized and preserves source order. An empty list
/// is permitted because the grammar permits an empty parenthesized
/// sensitivity specification. Whether that form is meaningful for a
/// particular process kind is a semantic question.
///
/// This type stores expression references, not evaluated signal values or
/// physical event sources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessSensitivity {
    expressions: Vec<HdlExpressionUse>,
}

impl ProcessSensitivity {
    /// Construct a process sensitivity list.
    ///
    /// Every expression must use the `EventOrSensitivity` role. This local
    /// check does not prove that an expression exists in the AST or resolves
    /// to a legal event source.
    ///
    /// # Errors
    ///
    /// Returns `InvalidSensitivityExpressionRole` if an expression uses
    /// another role.
    pub fn new(
        expressions: Vec<HdlExpressionUse>,
    ) -> Result<Self, ProcessError> {
        if expressions.iter().any(|expression| {
            !matches!(
                expression.role(),
                HdlExpressionRole::EventOrSensitivity
            )
        }) {
            return Err(ProcessError::InvalidSensitivityExpressionRole);
        }

        Ok(Self { expressions })
    }

    /// Return the expression references in source order.
    #[must_use]
    pub fn expressions(&self) -> &[HdlExpressionUse] {
        &self.expressions
    }

    /// Return the number of sensitivity expressions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    /// Return whether the sensitivity list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.expressions.is_empty()
    }
}

/// A process-local configuration binding.
///
/// The grammar represents a binding as an identifier followed by an
/// expression, for example `clock: clk;` or `mode: sequential;`.
///
/// `property` preserves the source-level property name. `expression` points
/// to the canonical AST expression. This structure intentionally does not
/// define a closed list of binding names or decide whether a property is
/// supported, may repeat, or conflicts with another property.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessBinding {
    property: String,
    expression: NodeId,
}

impl ProcessBinding {
    /// Construct a process binding from its property name and canonical
    /// expression identity.
    ///
    /// Syntax validity, expression existence, name resolution, and property
    /// legality must be checked by the parser and semantic integration.
    #[must_use]
    pub fn new(property: impl Into<String>, expression: NodeId) -> Self {
        Self {
            property: property.into(),
            expression,
        }
    }

    /// Return the original process property name.
    #[must_use]
    pub fn property(&self) -> &str {
        &self.property
    }

    /// Return the canonical AST identity of the binding expression.
    #[must_use]
    pub fn expression(&self) -> &NodeId {
        &self.expression
    }
}

/// One item in a process body, preserving its source order.
///
/// Each variant either references a canonical AST node or retains the
/// process-binding contract. The enum does not contain copies of statement,
/// block, or attribute AST structures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProcessItem {
    /// A statement owned by the shared statement grammar and canonical AST.
    Statement(NodeId),

    /// A process-local property binding.
    Binding(ProcessBinding),

    /// A nested process block represented by a canonical AST node.
    NestedBlock(NodeId),

    /// A process-local attribute list or attribute construct represented
    /// by a canonical AST node.
    Attribute(NodeId),
}

impl ProcessItem {
    /// Return the canonical AST identity when this item is represented
    /// directly by a node.
    ///
    /// A binding exposes its expression through `ProcessBinding::expression`
    /// instead; it is not itself a canonical AST node.
    #[must_use]
    pub fn node_id(&self) -> Option<&NodeId> {
        match self {
            Self::Statement(node)
            | Self::NestedBlock(node)
            | Self::Attribute(node) => Some(node),
            Self::Binding(_) => None,
        }
    }

    /// Return the binding when this item is a process-local binding.
    #[must_use]
    pub fn binding(&self) -> Option<&ProcessBinding> {
        match self {
            Self::Binding(binding) => Some(binding),
            Self::Statement(_)
            | Self::NestedBlock(_)
            | Self::Attribute(_) => None,
        }
    }
}

/// A target-independent HDL process declaration contract.
///
/// This is not an AST node. It is a compact integration object that retains
/// references to the canonical AST and the classification assigned by
/// semantic analysis.
///
/// `keyword` preserves the process keyword's source spelling. It is not
/// interpreted here as a closed enum because the grammar delegates the
/// keyword to the shared HDL keyword contract.
///
/// `kind` uses the repository's existing semantic process classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessDeclaration {
    id: HdlId,
    keyword: String,
    name: Option<String>,
    kind: HdlProcessKind,
    sensitivity: Option<ProcessSensitivity>,
    attributes: Vec<NodeId>,
    items: Vec<ProcessItem>,
}

impl ProcessDeclaration {
    /// Construct a process declaration contract.
    ///
    /// The constructor checks the local keyword invariant. It deliberately
    /// does not resolve AST references, validate process attributes, infer
    /// clock/reset behavior, or decide whether the body is legal for `kind`.
    ///
    /// An empty body is retained if supplied; the source grammar and semantic
    /// rules determine whether it is valid in context.
    ///
    /// # Errors
    ///
    /// Returns `InvalidProcessKeyword` if `keyword` is empty or contains a
    /// control character.
    pub fn new(
        id: HdlId,
        keyword: impl Into<String>,
        name: Option<String>,
        kind: HdlProcessKind,
        sensitivity: Option<ProcessSensitivity>,
        attributes: Vec<NodeId>,
        items: Vec<ProcessItem>,
    ) -> Result<Self, ProcessError> {
        let keyword = keyword.into();

        if keyword.trim().is_empty()
            || keyword.chars().any(char::is_control)
        {
            return Err(ProcessError::InvalidProcessKeyword);
        }

        Ok(Self {
            id,
            keyword,
            name,
            kind,
            sensitivity,
            attributes,
            items,
        })
    }

    /// Return the stable semantic identity of this process.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the original process keyword spelling.
    #[must_use]
    pub fn keyword(&self) -> &str {
        &self.keyword
    }

    /// Return the optional source-level process name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the semantic process classification.
    #[must_use]
    pub const fn kind(&self) -> HdlProcessKind {
        self.kind
    }

    /// Return the optional process sensitivity list.
    #[must_use]
    pub fn sensitivity(&self) -> Option<&ProcessSensitivity> {
        self.sensitivity.as_ref()
    }

    /// Return process-level attribute references in source order.
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Return body items in source order.
    ///
    /// The order is preserved even when semantic rules later determine that
    /// some categories of items cannot appear in that order.
    #[must_use]
    pub fn items(&self) -> &[ProcessItem] {
        &self.items
    }

    /// Return the number of items in the process body.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Return whether the process body has no items.
    #[must_use]
    pub fn is_body_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Result of source-level process semantic validation.
///
/// `Indeterminate` means that the semantic checker could not establish
/// validity. It must not be treated as a successful validation result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ProcessSemanticStatus {
    /// The checker established that the process is valid in context.
    Valid,

    /// The checker established a source-level semantic violation.
    Invalid,

    /// The checker lacks sufficient information to decide validity.
    Indeterminate,
}

/// Integration point for validating a process against canonical semantic
/// services.
///
/// Implementations should resolve canonical AST references and apply the
/// rules in the normative HDL specification. They should reuse existing
/// sequential-event, assignment, clock/reset, and shared semantic services.
///
/// The implementation must not infer meaning solely from a process keyword
/// or signal spelling. It must return `Indeterminate` when the information
/// needed for a sound decision is unavailable.
pub trait ProcessSemanticChecker {
    /// Validate the supplied process in its enclosing semantic context.
    ///
    /// This method is intentionally abstract: name resolution, AST graph
    /// validation, process-kind rules, and diagnostics are owned by the
    /// repository's semantic integration layer.
    fn check_process(
        &self,
        process: &ProcessDeclaration,
    ) -> ProcessSemanticStatus;
}

/// Validate a process using the supplied semantic integration.
///
/// This function does not parse source or mutate the process. It returns the
/// semantic checker's result unchanged, preserving the distinction between
/// invalid and indeterminate analysis.
#[must_use]
pub fn validate_process(
    process: &ProcessDeclaration,
    checker: &impl ProcessSemanticChecker,
) -> ProcessSemanticStatus {
    checker.check_process(process)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_id() -> HdlId {
        HdlId::new("test.process")
            .expect("the test process identity should be valid")
    }

    #[test]
    fn rejects_empty_process_keyword() {
        let result = ProcessDeclaration::new(
            process_id(),
            "  ",
            None,
            HdlProcessKind::Sequential,
            None,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(result, Err(ProcessError::InvalidProcessKeyword));
    }

    #[test]
    fn rejects_control_characters_in_process_keyword() {
        let result = ProcessDeclaration::new(
            process_id(),
            "process\ninvalid",
            None,
            HdlProcessKind::Sequential,
            None,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(result, Err(ProcessError::InvalidProcessKeyword));
    }

    #[test]
    fn permits_empty_sensitivity_list() {
        let sensitivity = ProcessSensitivity::new(Vec::new())
            .expect("an empty list is permitted by the grammar");

        assert!(sensitivity.is_empty());
        assert_eq!(sensitivity.len(), 0);
    }

    #[test]
    fn preserves_process_keyword_and_optional_name() {
        let process = ProcessDeclaration::new(
            process_id(),
            "process",
            Some("controller".to_owned()),
            HdlProcessKind::Sequential,
            None,
            Vec::new(),
            Vec::new(),
        )
        .expect("the process contract should be valid");

        assert_eq!(process.keyword(), "process");
        assert_eq!(process.name(), Some("controller"));
        assert_eq!(process.kind(), HdlProcessKind::Sequential);
        assert!(process.is_body_empty());
    }
}
