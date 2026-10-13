
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Statement Validation
//!
//! Path: `src/frontend/semantic/validation/statements.rs`
//! Rust: 2021 edition; Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Validate statement-level AST references and control-flow context before
//! downstream semantic results are treated as complete.
//!
//! ## Owns
//!
//! - normalized statement-validation input contracts;
//! - statement-reference integrity checks;
//! - required child-role checks;
//! - duplicate child-reference detection;
//! - statement-graph cycle detection;
//! - return/break/continue context checks;
//! - deterministic statement diagnostics;
//! - delegation to an authoritative statement semantic checker.
//!
//! ## Does not own
//!
//! - the canonical AST, NodeId, source Span, or grammar;
//! - parsing, declaration collection, name resolution, or import resolution;
//! - expression inference, type unification, or ownership analysis;
//! - effect, capability, resource, policy, or contract implementations;
//! - domain-specific semantic rules;
//! - target feasibility, resource allocation, scheduling, routing, or IR.
//!
//! ## Integration contract
//!
//! An adapter over `crate::frontend::ast::node::statements::statement`
//! supplies normalized records while preserving canonical NodeId identity,
//! source spans, child order, and child categories. It must not derive
//! semantics from debug output or display strings.
//!
//! `StatementSemanticChecker` delegates additional rules to the canonical
//! semantic services. It must not treat missing or unresolved information as
//! proof that a statement is valid.
//!
//! ## Scalability
//!
//! No language-level statement-count, nesting-depth, loop-count, hardware-size,
//! or computational-domain ceiling is introduced. Graph traversal is
//! iterative. Memory and work are proportional to the graph and context states
//! actually supplied. An outer semantic session may enforce explicit,
//! configurable operational budgets; budget exhaustion must mark analysis
//! incomplete rather than valid.
//!
//! ## Source coordinates
//!
//! Locations use UTF-8 byte offsets and half-open ranges `[start, end)`.
//! The source-map adapter is responsible for ensuring that offsets refer to
//! the correct source document.
//!
//! ## Diagnostics
//!
//! Diagnostic codes are stable identifiers. Human-readable messages are
//! informational and must not be parsed by consumers.
//!
//! The semantic diagnostics subsystem may adapt these records into its shared
//! diagnostic representation after that API is frozen.

use std::collections::{BTreeMap, BTreeSet};

/// Stable identity of a statement in one canonical AST.
///
/// The adapter must derive this from canonical NodeId, never from pointer
/// addresses, source spelling, or hash-map iteration order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatementId(pub String);

impl StatementId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

/// Canonical source location expressed in UTF-8 byte offsets.
///
/// `end` is exclusive. This boundary record can be converted to the AST's
/// canonical Span by the adapter or shared-diagnostic adapter.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocation {
    pub source_id: String,
    pub start: u64,
    pub end: u64,
}

impl SourceLocation {
    #[must_use]
    pub fn new(source_id: impl Into<String>, start: u64, end: u64) -> Self {
        Self {
            source_id: source_id.into(),
            start,
            end,
        }
    }

    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        !self.source_id.trim().is_empty() && self.start <= self.end
    }
}

/// Semantic category of a referenced AST node.
///
/// This is metadata supplied by the AST adapter, not a replacement AST enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeClass {
    Statement,
    Expression,
    Pattern,
    Type,
    Declaration,
    MatchArm,
    Other,
}

/// Meaningful structural role of a statement child.
///
/// Repeated `BodyStatement` and `MatchArm` roles are permitted for blocks and
/// matches. The adapter must preserve the canonical source order in `children`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChildRole {
    Expression,
    BindingPattern,
    TypeAnnotation,
    Initializer,
    ReturnValue,
    ControlLabel,
    Condition,
    ThenBranch,
    ElseBranch,
    LoopPattern,
    Iterable,
    LoopBody,
    BodyStatement,
    Scrutinee,
    MatchArm,
    Declaration,
    RecoveryNode,
    ExtensionChild,
    Other,
}

/// Statement-level categories needed for generic validation.
///
/// Domain-specific statements use `Extension`; they do not require a new
/// central variant for every quantum, HDL, accelerator, or future operation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatementKind {
    Expression,
    Let,
    Const,
    Return,
    Break,
    Continue,
    If,
    While,
    For,
    Block,
    Match,
    Declaration,
    Error,
    Extension {
        namespace: String,
        name: String,
    },
    Other(String),
}

impl StatementKind {
    fn label(&self) -> &str {
        match self {
            Self::Expression => "expression",
            Self::Let => "let",
            Self::Const => "const",
            Self::Return => "return",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::If => "if",
            Self::While => "while",
            Self::For => "for",
            Self::Block => "block",
            Self::Match => "match",
            Self::Declaration => "declaration",
            Self::Error => "error-recovery",
            Self::Extension { name, .. } => name,
            Self::Other(name) => name,
        }
    }

    fn is_loop(&self) -> bool {
        matches!(self, Self::While | Self::For)
    }

    fn required_roles(&self) -> &'static [ChildRole] {
        use ChildRole as R;
        use StatementKind as K;

        match self {
            K::Expression => &[R::Expression],
            K::Let => &[R::BindingPattern],
            K::Const => &[R::BindingPattern, R::Initializer],
            K::Return => &[],
            K::Break => &[],
            K::Continue => &[],
            K::If => &[R::Condition, R::ThenBranch],
            K::While => &[R::Condition, R::LoopBody],
            K::For => &[R::LoopPattern, R::Iterable, R::LoopBody],
            K::Block => &[],
            K::Match => &[R::Scrutinee],
            K::Declaration => &[R::Declaration],
            K::Error => &[],
            K::Extension { .. } | K::Other(_) => &[],
        }
    }
}

/// One edge from a statement to a canonical AST node.
///
/// Child references are typed by the adapter so a missing expression cannot
/// accidentally be treated as a valid statement body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementChild {
    pub id: StatementId,
    pub role: ChildRole,
    pub class: NodeClass,
}

impl StatementChild {
    #[must_use]
    pub fn new(id: StatementId, role: ChildRole, class: NodeClass) -> Self {
        Self { id, role, class }
    }
}

/// One normalized statement from the canonical AST.
///
/// `opens_callable_scope` is true only when this statement introduces a
/// callable body whose statement children are in that callable's context.
/// It is usually false for ordinary statements. Loop context is derived from
/// `While` and `For` kinds.
///
/// The adapter must not infer this flag from the source spelling alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementNode {
    pub id: StatementId,
    pub kind: StatementKind,
    pub location: SourceLocation,
    pub children: Vec<StatementChild>,
    pub opens_callable_scope: bool,
}

impl StatementNode {
    #[must_use]
    pub fn new(
        id: StatementId,
        kind: StatementKind,
        location: SourceLocation,
        children: Vec<StatementChild>,
    ) -> Self {
        Self {
            id,
            kind,
            location,
            children,
            opens_callable_scope: false,
        }
    }

    #[must_use]
    pub fn with_callable_scope(mut self, opens_callable_scope: bool) -> Self {
        self.opens_callable_scope = opens_callable_scope;
        self
    }
}

/// Metadata for any AST node referenced by a statement.
///
/// `StatementProvider::node_info` must include non-statement nodes as well,
/// because statement children can refer to expressions, patterns, types,
/// declarations, and match arms.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstNodeInfo {
    pub id: StatementId,
    pub class: NodeClass,
    pub location: SourceLocation,
}

/// Read-only view over the canonical AST graph.
///
/// For a given analysis invocation, each ID must refer to stable node data.
/// `roots()` must return the statements the caller intends to validate.
/// `get_statement()` returns only statement nodes. `node_info()` resolves
/// every referenced AST node regardless of its class.
pub trait StatementProvider {
    fn roots(&self) -> Vec<StatementId>;

    fn get_statement(&self, id: &StatementId) -> Option<StatementNode>;

    fn node_info(&self, id: &StatementId) -> Option<AstNodeInfo>;
}

/// Context available while validating one statement.
///
/// This is deliberately small. More advanced flow state—definite assignment,
/// return-type compatibility, reachability, effects, and ownership—belongs to
/// the canonical semantic services.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatementContext {
    pub in_callable: bool,
    pub in_loop: bool,
}

/// Additional semantic issue returned by an authoritative semantic checker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementCheckIssue {
    pub code: String,
    pub message: String,
    pub related: Vec<SourceLocation>,
}

/// Integration interface for statement rules owned by other semantic modules.
///
/// Implementations may use name resolution, type checking, effects, contracts,
/// and domain validators. Return issues for violations or unresolved facts;
/// do not silently accept unknown information.
pub trait StatementSemanticChecker {
    fn check(
        &self,
        statement: &StatementNode,
        context: StatementContext,
    ) -> Vec<StatementCheckIssue>;
}

/// Machine-readable diagnostic categories produced by this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatementDiagnosticCode {
    EmptyStatementIdentity,
    DuplicateStatementIdentity,
    MissingStatement,
    MissingReferencedNode,
    ReferencedNodeIdentityMismatch,
    InvalidSourceLocation,
    EmptyExtensionNamespace,
    EmptyExtensionName,
    MissingRequiredChild,
    WrongChildClass,
    DuplicateChildReference,
    DuplicateChildRole,
    StatementCycle,
    ReturnOutsideCallable,
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ErrorRecoveryNode,
    SemanticCheckFailure,
}

impl StatementDiagnosticCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStatementIdentity => "ZSEM-STMT-001",
            Self::DuplicateStatementIdentity => "ZSEM-STMT-002",
            Self::MissingStatement => "ZSEM-STMT-003",
            Self::MissingReferencedNode => "ZSEM-STMT-004",
            Self::ReferencedNodeIdentityMismatch => "ZSEM-STMT-005",
            Self::InvalidSourceLocation => "ZSEM-STMT-006",
            Self::EmptyExtensionNamespace => "ZSEM-STMT-007",
            Self::EmptyExtensionName => "ZSEM-STMT-008",
            Self::MissingRequiredChild => "ZSEM-STMT-009",
            Self::WrongChildClass => "ZSEM-STMT-010",
            Self::DuplicateChildReference => "ZSEM-STMT-011",
            Self::DuplicateChildRole => "ZSEM-STMT-012",
            Self::StatementCycle => "ZSEM-STMT-013",
            Self::ReturnOutsideCallable => "ZSEM-STMT-014",
            Self::BreakOutsideLoop => "ZSEM-STMT-015",
            Self::ContinueOutsideLoop => "ZSEM-STMT-016",
            Self::ErrorRecoveryNode => "ZSEM-STMT-017",
            Self::SemanticCheckFailure => "ZSEM-STMT-018",
        }
    }
}

/// One structured statement diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatementDiagnostic {
    pub code: StatementDiagnosticCode,
    pub message: String,
    pub primary: Option<SourceLocation>,
    pub related: Vec<SourceLocation>,
    pub statement: Option<StatementId>,
}

impl StatementDiagnostic {
    fn new(
        code: StatementDiagnosticCode,
        message: impl Into<String>,
        statement: Option<StatementId>,
        primary: Option<SourceLocation>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            primary,
            related: Vec::new(),
            statement,
        }
    }

    fn with_related(mut self, mut related: Vec<SourceLocation>) -> Self {
        related.sort();
        related.dedup();
        self.related = related;
        self
    }
}

/// Result of validating a statement graph.
///
/// `complete` is false when a requested statement or a required referenced
/// node could not be inspected, or when an invalid statement graph prevents
/// full semantic checking. A result with errors must never be certified as
/// semantically valid by a later pipeline stage.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StatementValidationResult {
    pub diagnostics: Vec<StatementDiagnostic>,
    pub statements_seen: usize,
    pub complete: bool,
}

impl StatementValidationResult {
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.complete && self.diagnostics.is_empty()
    }
}

/// Validate statement structure and delegate language-specific semantic rules.
///
/// The graph is discovered iteratively, then checked for cycles and local
/// invariants, and finally checked under the callable/loop contexts reachable
/// from the roots. Root and diagnostic order are deterministic.
///
/// The provider must represent a consistent snapshot of one canonical AST.
/// An outer semantic session should enforce cancellation and configurable
/// work/memory/diagnostic budgets when processing untrusted input.
#[must_use]
pub fn validate_statements<P, C>(
    provider: &P,
    checker: &C,
) -> StatementValidationResult
where
    P: StatementProvider,
    C: StatementSemanticChecker,
{
    let mut result = StatementValidationResult {
        complete: true,
        ..StatementValidationResult::default()
    };

    let mut roots = provider.roots();
    roots.sort();
    roots.dedup();

    let mut nodes: BTreeMap<StatementId, StatementNode> = BTreeMap::new();
    let mut malformed = BTreeSet::<StatementId>::new();
    let mut pending = roots.clone();
    let mut scheduled = BTreeSet::<StatementId>::new();

    // Snapshot reachable statement nodes once. Non-statement references are
    // checked through node_info(), while nested statement references are
    // traversed iteratively.
    while let Some(id) = pending.pop() {
        if !scheduled.insert(id.clone()) {
            continue;
        }

        if !id.is_valid() {
            result.diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::EmptyStatementIdentity,
                "Statement identity must not be empty.",
                Some(id.clone()),
                None,
            ));
            malformed.insert(id);
            result.complete = false;
            continue;
        }

        let Some(node) = provider.get_statement(&id) else {
            result.diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::MissingStatement,
                "A statement reference does not identify a statement in the canonical AST.",
                Some(id.clone()),
                None,
            ));
            malformed.insert(id);
            result.complete = false;
            continue;
        };

        if node.id != id {
            result.diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::ReferencedNodeIdentityMismatch,
                "The statement provider returned a node whose identity differs from the requested identity.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(id.clone());
            malformed.insert(node.id.clone());
            result.complete = false;
            continue;
        }

        result.statements_seen = result.statements_seen.saturating_add(1);

        if !node.location.is_well_formed() {
            result.diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::InvalidSourceLocation,
                "Statement source location has an empty source identity or reversed byte range.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(id.clone());
            result.complete = false;
        }

        if let StatementKind::Extension { namespace, name } = &node.kind {
            if namespace.trim().is_empty() {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::EmptyExtensionNamespace,
                    "An extension statement must have a non-empty namespace.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                malformed.insert(id.clone());
                result.complete = false;
            }

            if name.trim().is_empty() {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::EmptyExtensionName,
                    "An extension statement must have a non-empty local name.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                malformed.insert(id.clone());
                result.complete = false;
            }
        }

        validate_child_roles(&node, &mut result.diagnostics, &mut malformed);

        let mut seen_child_ids = BTreeSet::new();
        for child in &node.children {
            if !seen_child_ids.insert(child.id.clone()) {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::DuplicateChildReference,
                    "A statement references the same child node more than once.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                malformed.insert(id.clone());
                result.complete = false;
            }

            match provider.node_info(&child.id) {
                None => {
                    result.diagnostics.push(StatementDiagnostic::new(
                        StatementDiagnosticCode::MissingReferencedNode,
                        format!(
                            "Child role {:?} references a node absent from the canonical AST.",
                            child.role
                        ),
                        Some(id.clone()),
                        Some(node.location.clone()),
                    ));
                    malformed.insert(id.clone());
                    result.complete = false;
                }
                Some(info) => {
                    if info.id != child.id {
                        result.diagnostics.push(StatementDiagnostic::new(
                            StatementDiagnosticCode::ReferencedNodeIdentityMismatch,
                            "The AST provider returned child metadata with a mismatched identity.",
                            Some(id.clone()),
                            Some(node.location.clone()),
                        ));
                        malformed.insert(id.clone());
                        result.complete = false;
                    } else if info.class != child.class {
                        result.diagnostics.push(
                            StatementDiagnostic::new(
                                StatementDiagnosticCode::WrongChildClass,
                                format!(
                                    "Child role {:?} was declared as {:?}, but the canonical AST reports {:?}.",
                                    child.role, child.class, info.class
                                ),
                                Some(id.clone()),
                                Some(node.location.clone()),
                            )
                            .with_related(vec![info.location]),
                        );
                        malformed.insert(id.clone());
                        result.complete = false;
                    }
                }
            }

            if child.class == NodeClass::Statement {
                pending.push(child.id.clone());
            }
        }

        if matches!(node.kind, StatementKind::Error) {
            result.diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::ErrorRecoveryNode,
                "A parser-recovery statement cannot be certified as semantically valid.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(id.clone());
            result.complete = false;
        }

        nodes.insert(id, node);
    }

    detect_statement_cycles(&nodes, &mut malformed, &mut result);

    // Propagate only the minimal context needed for universal control-transfer
    // checks. A statement shared by multiple parents can be checked under
    // multiple contexts; each context state is visited once.
    let mut work: Vec<(StatementId, StatementContext)> = roots
        .into_iter()
        .map(|id| (id, StatementContext::default()))
        .collect();
    let mut visited = BTreeSet::<(StatementId, StatementContext)>::new();

    while let Some((id, inherited)) = work.pop() {
        if !visited.insert((id.clone(), inherited)) {
            continue;
        }

        let Some(node) = nodes.get(&id) else {
            continue;
        };

        if malformed.contains(&id) {
            continue;
        }

        let context = StatementContext {
            in_callable: inherited.in_callable || node.opens_callable_scope,
            in_loop: inherited.in_loop || node.kind.is_loop(),
        };

        match node.kind {
            StatementKind::Return if !context.in_callable => {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::ReturnOutsideCallable,
                    "`return` is not enclosed by a callable body.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                result.complete = false;
            }
            StatementKind::Break if !context.in_loop => {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::BreakOutsideLoop,
                    "`break` is not enclosed by a loop context.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                result.complete = false;
            }
            StatementKind::Continue if !context.in_loop => {
                result.diagnostics.push(StatementDiagnostic::new(
                    StatementDiagnosticCode::ContinueOutsideLoop,
                    "`continue` is not enclosed by a loop context.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                result.complete = false;
            }
            _ => {}
        }

        for child in node.children.iter().rev() {
            if child.class == NodeClass::Statement {
                work.push((child.id.clone(), context));
            }
        }

        for issue in checker.check(node, context) {
            let diagnostic = StatementDiagnostic::new(
                StatementDiagnosticCode::SemanticCheckFailure,
                format!("{}: {}", issue.code, issue.message),
                Some(id.clone()),
                Some(node.location.clone()),
            )
            .with_related(issue.related);
            result.diagnostics.push(diagnostic);
        }
    }

    sort_diagnostics(&mut result.diagnostics);
    result
}

fn validate_child_roles(
    node: &StatementNode,
    diagnostics: &mut Vec<StatementDiagnostic>,
    malformed: &mut BTreeSet<StatementId>,
) {
    let mut roles = BTreeMap::<ChildRole, usize>::new();

    for child in &node.children {
        *roles.entry(child.role).or_insert(0) += 1;
    }

    for required in node.kind.required_roles() {
        if !roles.contains_key(required) {
            diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::MissingRequiredChild,
                format!(
                    "Statement kind `{}` is missing required child role {:?}.",
                    node.kind.label(),
                    required
                ),
                Some(node.id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(node.id.clone());
        }
    }

    // Roles representing lists may occur more than once. All other roles are
    // single-valued; repeated singleton roles are likely adapter mistakes.
    for (role, count) in roles {
        let is_repeated_role = matches!(role, ChildRole::BodyStatement
            | ChildRole::MatchArm
            | ChildRole::ExtensionChild);

        if count > 1 && !is_repeated_role {
            diagnostics.push(StatementDiagnostic::new(
                StatementDiagnosticCode::DuplicateChildRole,
                format!(
                    "Child role {:?} occurs {count} times but is a singleton role.",
                    role
                ),
                Some(node.id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(node.id.clone());
        }
    }
}

fn detect_statement_cycles(
    nodes: &BTreeMap<StatementId, StatementNode>,
    malformed: &mut BTreeSet<StatementId>,
    result: &mut StatementValidationResult,
) {
    // Iterative depth-first search with explicit enter/exit events. Colors:
    // absent = unseen, 1 = active, 2 = finished.
    let mut color = BTreeMap::<StatementId, u8>::new();

    for start in nodes.keys() {
        if color.get(start).copied().unwrap_or(0) == 2 {
            continue;
        }

        let mut stack: Vec<(StatementId, bool)> = vec![(start.clone(), false)];

        while let Some((id, exiting)) = stack.pop() {
            if exiting {
                color.insert(id, 2);
                continue;
            }

            match color.get(&id).copied().unwrap_or(0) {
                2 => continue,
                1 => {
                    malformed.insert(id.clone());
                    result.diagnostics.push(StatementDiagnostic::new(
                        StatementDiagnosticCode::StatementCycle,
                        "The statement-child graph contains a cycle.",
                        Some(id.clone()),
                        nodes.get(&id).map(|node| node.location.clone()),
                    ));
                    result.complete = false;
                    continue;
                }
                _ => {}
            }

            color.insert(id.clone(), 1);
            stack.push((id.clone(), true));

            if let Some(node) = nodes.get(&id) {
                for child in node.children.iter().rev() {
                    if child.class != NodeClass::Statement {
                        continue;
                    }

                    match color.get(&child.id).copied().unwrap_or(0) {
                        1 => {
                            malformed.insert(id.clone());
                            malformed.insert(child.id.clone());
                            result.diagnostics.push(
                                StatementDiagnostic::new(
                                    StatementDiagnosticCode::StatementCycle,
                                    "A statement child points to an active ancestor, forming a cycle.",
                                    Some(id.clone()),
                                    Some(node.location.clone()),
                                )
                                .with_related(
                                    nodes
                                        .get(&child.id)
                                        .map(|entry| vec![entry.location.clone()])
                                        .unwrap_or_default(),
                                ),
                            );
                            result.complete = false;
                        }
                        2 => {}
                        _ => stack.push((child.id.clone(), false)),
                    }
                }
            }
        }
    }
}

fn sort_diagnostics(diagnostics: &mut Vec<StatementDiagnostic>) {
    for diagnostic in diagnostics.iter_mut() {
        diagnostic.related.sort();
        diagnostic.related.dedup();
    }

    diagnostics.sort_by(|left, right| {
        left.primary
            .cmp(&right.primary)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.statement.cmp(&right.statement))
            .then_with(|| left.message.cmp(&right.message))
            .then_with(|| left.related.cmp(&right.related))
    });

    diagnostics.dedup();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestProvider {
        roots: Vec<StatementId>,
        statements: BTreeMap<StatementId, StatementNode>,
        other_nodes: BTreeMap<StatementId, AstNodeInfo>,
    }

    impl TestProvider {
        fn insert_statement(&mut self, node: StatementNode) {
            self.other_nodes.insert(
                node.id.clone(),
                AstNodeInfo {
                    id: node.id.clone(),
                    class: NodeClass::Statement,
                    location: node.location.clone(),
                },
            );
            self.statements.insert(node.id.clone(), node);
        }

        fn insert_other(
            &mut self,
            id: &str,
            class: NodeClass,
            location: SourceLocation,
        ) {
            let id = StatementId::new(id);
            self.other_nodes.insert(
                id.clone(),
                AstNodeInfo {
                    id,
                    class,
                    location,
                },
            );
        }
    }

    impl StatementProvider for TestProvider {
        fn roots(&self) -> Vec<StatementId> {
            self.roots.clone()
        }

        fn get_statement(&self, id: &StatementId) -> Option<StatementNode> {
            self.statements.get(id).cloned()
        }

        fn node_info(&self, id: &StatementId) -> Option<AstNodeInfo> {
            self.other_nodes.get(id).cloned()
        }
    }

    struct NoopChecker;

    impl StatementSemanticChecker for NoopChecker {
        fn check(
            &self,
            _statement: &StatementNode,
            _context: StatementContext,
        ) -> Vec<StatementCheckIssue> {
            Vec::new()
        }
    }

    fn loc(start: u64, end: u64) -> SourceLocation {
        SourceLocation::new("test.zm", start, end)
    }

    fn node(id: &str, kind: StatementKind, children: Vec<StatementChild>) -> StatementNode {
        StatementNode::new(StatementId::new(id), kind, loc(0, 1), children)
    }

    #[test]
    fn empty_program_is_valid_and_complete() {
        let provider = TestProvider::default();
        let result = validate_statements(&provider, &NoopChecker);
        assert!(result.is_valid());
        assert_eq!(result.statements_seen, 0);
    }

    #[test]
    fn expression_statement_requires_expression_child() {
        let mut provider = TestProvider::default();
        let statement = node("s1", StatementKind::Expression, vec![]);
        provider.roots.push(statement.id.clone());
        provider.insert_statement(statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::MissingRequiredChild
        }));
        assert!(!result.is_valid());
    }

    #[test]
    fn return_outside_callable_is_rejected() {
        let mut provider = TestProvider::default();
        let statement = node("s1", StatementKind::Return, vec![]);
        provider.roots.push(statement.id.clone());
        provider.insert_statement(statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::ReturnOutsideCallable
        }));
    }

    #[test]
    fn break_outside_loop_is_rejected() {
        let mut provider = TestProvider::default();
        let statement = node("s1", StatementKind::Break, vec![]);
        provider.roots.push(statement.id.clone());
        provider.insert_statement(statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::BreakOutsideLoop
        }));
    }

    #[test]
    fn break_inside_loop_is_accepted_by_context_check() {
        let mut provider = TestProvider::default();

        let break_statement = node("break", StatementKind::Break, vec![]);
        let break_id = break_statement.id.clone();

        let body = node(
            "body",
            StatementKind::Block,
            vec![StatementChild::new(
                break_id.clone(),
                ChildRole::BodyStatement,
                NodeClass::Statement,
            )],
        );

        let loop_statement = node(
            "loop",
            StatementKind::While,
            vec![
                StatementChild::new(
                    StatementId::new("condition"),
                    ChildRole::Condition,
                    NodeClass::Expression,
                ),
                StatementChild::new(
                    body.id.clone(),
                    ChildRole::LoopBody,
                    NodeClass::Statement,
                ),
            ],
        );

        provider.insert_other("condition", NodeClass::Expression, loc(1, 2));
        provider.insert_statement(break_statement);
        provider.insert_statement(body);
        provider.roots.push(loop_statement.id.clone());
        provider.insert_statement(loop_statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(!result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::BreakOutsideLoop
        }));
    }

    #[test]
    fn missing_referenced_node_is_reported() {
        let mut provider = TestProvider::default();
        let statement = node(
            "s1",
            StatementKind::Expression,
            vec![StatementChild::new(
                StatementId::new("missing"),
                ChildRole::Expression,
                NodeClass::Expression,
            )],
        );
        provider.roots.push(statement.id.clone());
        provider.insert_statement(statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::MissingReferencedNode
        }));
    }

    #[test]
    fn invalid_recovery_statement_is_never_certified() {
        let mut provider = TestProvider::default();
        let statement = node("s1", StatementKind::Error, vec![]);
        provider.roots.push(statement.id.clone());
        provider.insert_statement(statement);

        let result = validate_statements(&provider, &NoopChecker);
        assert!(!result.is_valid());
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == StatementDiagnosticCode::ErrorRecoveryNode
        }));
    }

    #[test]
    fn diagnostics_are_deterministically_ordered() {
        let mut provider = TestProvider::default();

        let a = node("a", StatementKind::Return, vec![]);
        let b = node("b", StatementKind::Continue, vec![]);
        provider.roots = vec![b.id.clone(), a.id.clone()];
        provider.insert_statement(a);
        provider.insert_statement(b);

        let first = validate_statements(&provider, &NoopChecker);
        let second = validate_statements(&provider, &NoopChecker);
        assert_eq!(first, second);
    }
}
