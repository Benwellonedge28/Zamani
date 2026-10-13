
#![forbid(unsafe_code)]

//! Expression graph validation and semantic-check orchestration.
//!
//! # Responsibility
//!
//! This module validates the integrity of an expression graph and coordinates
//! bottom-up expression type inference. It deliberately does not define
//! Zamani's canonical AST, own the symbol table, implement a second type
//! system, authorize effects, or decide whether a physical target can execute
//! a program.
//!
//! # Integration boundary
//!
//! The canonical AST adapter implements [`ExpressionProvider`] and exposes
//! canonical expression identities, source locations, child relationships,
//! and expression-kind metadata. The existing semantic type system implements
//! [`ExpressionTypeChecker`] using Zamani's normative typing rules.
//!
//! The adapter must preserve canonical AST identity and source spans. It must
//! not synthesize new source nodes or infer semantic meaning from display
//! strings.
//!
//! # Determinism
//!
//! Roots, child references, graph traversal, and diagnostics are processed in
//! stable order. Providers should expose children in their canonical source
//! order. Diagnostic ordering does not depend on hash-map iteration.
//!
//! # Scalability
//!
//! Traversal is iterative rather than recursive, so graph depth does not
//! directly consume the Rust call stack. This module imposes no arbitrary
//! language-level expression-count limit. Callers may enforce configurable
//! operational budgets in the semantic session.
//!
//! # Error behavior
//!
//! Missing nodes, invalid locations, duplicate identities, cycles, missing
//! child types, and type-checking failures are reported explicitly. A failed
//! expression is never represented as successfully type-checked.
//!
//! # Ownership boundaries
//!
//! - AST ownership: `src/frontend/ast/node/`.
//! - Name resolution: `src/frontend/semantic/symbols/`.
//! - Canonical semantic types: `src/frontend/semantic/types/`.
//! - Shared diagnostics: `src/frontend/semantic/diagnostics/`.
//! - Pipeline orchestration: `src/frontend/semantic/pipeline.rs`.
//! - Target-specific lowering and feasibility: downstream compiler stages.
//!
//! The small public types below are boundary records, not replacement AST
//! nodes. When the repository's canonical span, ID, and diagnostic types have
//! been verified, the integration adapter should map them to these interfaces
//! or replace these boundary types with the canonical types.

use std::collections::{BTreeMap, BTreeSet};

/// Stable identity of an expression in one canonical AST.
///
/// The AST adapter must derive this from canonical node identity, not from
/// pointer addresses, source spelling, or traversal order.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExpressionId(pub String);

/// A source location using byte offsets into a named source.
///
/// `end` is exclusive. The source owner is responsible for ensuring that
/// offsets refer to the correct source representation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceLocation {
    pub source_id: String,
    pub start: u64,
    pub end: u64,
}

impl SourceLocation {
    /// Returns whether the location has a non-empty source identity and
    /// ordered, half-open byte offsets.
    pub fn is_well_formed(&self) -> bool {
        !self.source_id.trim().is_empty() && self.start <= self.end
    }
}

/// Minimal expression information required by this validation pass.
///
/// `kind` is a stable semantic category supplied by the AST adapter, such as
/// `"literal"`, `"binary"`, `"call"`, or `"conditional"`. It is descriptive
/// metadata, not a substitute for matching the canonical AST variant.
///
/// `children` must preserve the canonical AST's defined child order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpressionNode {
    pub id: ExpressionId,
    pub kind: String,
    pub location: SourceLocation,
    pub children: Vec<ExpressionId>,
}

/// Read-only access to canonical expression nodes.
///
/// Implement this trait in the AST adapter. `roots()` must return the
/// expressions that the caller intends to validate, in deterministic order.
/// `get()` must return the same node for a given ID throughout one analysis.
pub trait ExpressionProvider {
    fn roots(&self) -> Vec<ExpressionId>;

    fn get(&self, id: &ExpressionId) -> Option<ExpressionNode>;
}

/// Stable reference to a canonical semantic type.
///
/// This is an opaque boundary key, not a serialized type-system definition.
/// The adapter should use the repository's canonical semantic type identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeRef(pub String);

/// Machine-readable categories for errors emitted by this module.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExpressionDiagnosticCode {
    EmptyExpressionId,
    DuplicateExpressionId,
    EmptyExpressionKind,
    InvalidSourceLocation,
    MissingExpressionNode,
    DuplicateChildReference,
    ExpressionCycle,
    MissingChildType,
    TypeCheckFailure,
    EmptyTypeIdentity,
}

/// One structured issue found during expression validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpressionDiagnostic {
    pub code: ExpressionDiagnosticCode,
    pub message: String,
    pub primary: Option<SourceLocation>,
    pub related: Vec<SourceLocation>,
    pub expression: Option<ExpressionId>,
}

impl ExpressionDiagnostic {
    fn new(
        code: ExpressionDiagnosticCode,
        message: impl Into<String>,
        expression: Option<ExpressionId>,
        primary: Option<SourceLocation>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            primary,
            related: Vec::new(),
            expression,
        }
    }

    fn with_related(mut self, location: SourceLocation) -> Self {
        self.related.push(location);
        self.related.sort();
        self.related.dedup();
        self
    }
}

/// Type-checking issue returned by the canonical semantic type checker.
///
/// A repository integration should translate this record into the shared
/// diagnostic representation without parsing its human-readable message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckIssue {
    pub code: String,
    pub message: String,
    pub related: Vec<SourceLocation>,
}

/// Result of checking one expression after its children have been checked.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckOutcome {
    /// `Some(type)` only when the expression's type was established.
    ///
    /// A type checker must not return a successful type merely because it
    /// cannot determine the actual type. Use issues and `None` instead.
    pub ty: Option<TypeRef>,
    pub issues: Vec<TypeCheckIssue>,
}

/// Integration contract for the canonical semantic type checker.
///
/// The implementation should resolve operators, calls, conversions, generic
/// instantiations, and other typing rules using the shared symbol and type
/// systems. It must not treat unknown or unresolved types as automatically
/// compatible.
///
/// Child types are provided in the same order as `node.children`.
pub trait ExpressionTypeChecker {
    fn infer(
        &self,
        node: &ExpressionNode,
        child_types: &[TypeRef],
    ) -> TypeCheckOutcome;
}

/// Whether an expression has a fully established type after this pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionStatus {
    Typed(TypeRef),
    Invalid,
    Incomplete,
}

/// Complete output of expression graph validation and type-check orchestration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExpressionValidationResult {
    /// Successfully established expression types, keyed by canonical ID.
    pub types: BTreeMap<ExpressionId, TypeRef>,

    /// Per-expression status, including invalid and incomplete expressions.
    pub statuses: BTreeMap<ExpressionId, ExpressionStatus>,

    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<ExpressionDiagnostic>,

    /// False when at least one requested root could not be fully checked.
    pub complete: bool,
}

impl ExpressionValidationResult {
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn type_of(&self, id: &ExpressionId) -> Option<&TypeRef> {
        self.types.get(id)
    }

    pub fn status_of(&self, id: &ExpressionId) -> Option<&ExpressionStatus> {
        self.statuses.get(id)
    }
}

/// Validate and type-check all expression nodes reachable from the provider's
/// roots.
///
/// This function validates graph integrity before invoking the type checker.
/// It uses iterative graph traversal, performs inference in child-before-parent
/// order, and never declares a node typed when one of its children has no
/// established type.
///
/// This function does not replace whole-program name resolution, effect
/// checking, contract validation, or domain-specific validation. The supplied
/// type checker should use the already-established symbol environment and
/// canonical type system.
///
/// # Determinism
///
/// For stable provider output and a deterministic type checker, the result is
/// deterministic. Root order is normalized; child order is preserved because
/// it can be semantically significant.
///
/// # Operational budgets
///
/// No fixed graph-size limit is imposed here. A semantic session handling
/// untrusted or exceptionally large inputs should enforce configurable work,
/// memory, cancellation, or diagnostic budgets around this call.
pub fn validate_expressions<P, T>(
    provider: &P,
    type_checker: &T,
) -> ExpressionValidationResult
where
    P: ExpressionProvider,
    T: ExpressionTypeChecker,
{
    let mut result = ExpressionValidationResult {
        complete: true,
        ..ExpressionValidationResult::default()
    };

    let mut roots = provider.roots();
    roots.sort();
    roots.dedup();

    let mut nodes: BTreeMap<ExpressionId, ExpressionNode> = BTreeMap::new();
    let mut malformed = BTreeSet::<ExpressionId>::new();
    let mut graph_edges: BTreeMap<ExpressionId, Vec<ExpressionId>> = BTreeMap::new();

    // Discover nodes iteratively. A discovered node is fetched once, so the
    // provider cannot accidentally change a node between validation passes.
    let mut pending = roots.clone();
    let mut scheduled = BTreeSet::<ExpressionId>::new();

    while let Some(id) = pending.pop() {
        if !scheduled.insert(id.clone()) {
            continue;
        }

        if id.0.trim().is_empty() {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::EmptyExpressionId,
                "Expression identity must not be empty.",
                Some(id.clone()),
                None,
            ));
            malformed.insert(id);
            result.complete = false;
            continue;
        }

        let Some(node) = provider.get(&id) else {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::MissingExpressionNode,
                "An expression reference does not identify a node in the canonical AST.",
                Some(id.clone()),
                None,
            ));
            malformed.insert(id);
            result.complete = false;
            continue;
        };

        if node.id != id {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::DuplicateExpressionId,
                "The expression provider returned a node with an identity different from the requested identity.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            malformed.insert(id.clone());
            malformed.insert(node.id.clone());
            result.complete = false;
            continue;
        }

        let mut node_invalid = false;

        if node.kind.trim().is_empty() {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::EmptyExpressionKind,
                "Expression kind metadata must not be empty.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            node_invalid = true;
        }

        if !node.location.is_well_formed() {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::InvalidSourceLocation,
                "Expression source location has an empty source identity or an invalid byte range.",
                Some(id.clone()),
                Some(node.location.clone()),
            ));
            node_invalid = true;
        }

        let mut seen_children = BTreeSet::new();
        for child in &node.children {
            if !seen_children.insert(child.clone()) {
                result.diagnostics.push(ExpressionDiagnostic::new(
                    ExpressionDiagnosticCode::DuplicateChildReference,
                    "The same child expression is referenced more than once by one expression node.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                node_invalid = true;
            }
        }

        if node_invalid {
            malformed.insert(id.clone());
            result.complete = false;
        }

        graph_edges.insert(id.clone(), node.children.clone());

        // Reverse-push so the first child in canonical order is discovered
        // first. Final inference order is computed independently below.
        for child in node.children.iter().rev() {
            pending.push(child.clone());
        }

        nodes.insert(id, node);
    }

    // Detect cycles and produce a child-before-parent ordering without using
    // recursive Rust calls. The color values are: 0/unseen, 1/active, 2/done.
    let mut colors: BTreeMap<ExpressionId, u8> = BTreeMap::new();
    let mut postorder: Vec<ExpressionId> = Vec::new();

    for root in &roots {
        if !nodes.contains_key(root) || colors.get(root).copied().unwrap_or(0) == 2 {
            continue;
        }

        let mut stack: Vec<(ExpressionId, usize)> = vec![(root.clone(), 0)];
        colors.insert(root.clone(), 1);

        while let Some((current, child_index)) = stack.last_mut() {
            let current_id = current.clone();
            let children = graph_edges.get(&current_id).cloned().unwrap_or_default();

            if *child_index >= children.len() {
                colors.insert(current_id.clone(), 2);
                postorder.push(current_id);
                stack.pop();
                continue;
            }

            let child = children[*child_index].clone();
            *child_index += 1;

            if !nodes.contains_key(&child) {
                // Missing references were diagnosed during discovery.
                continue;
            }

            match colors.get(&child).copied().unwrap_or(0) {
                0 => {
                    colors.insert(child.clone(), 1);
                    stack.push((child, 0));
                }
                1 => {
                    let primary = nodes.get(&current_id).map(|n| n.location.clone());
                    let related = nodes.get(&child).map(|n| n.location.clone());

                    let mut diagnostic = ExpressionDiagnostic::new(
                        ExpressionDiagnosticCode::ExpressionCycle,
                        "The expression graph contains a cycle; expression dependencies must be acyclic.",
                        Some(current_id.clone()),
                        primary,
                    );

                    if let Some(location) = related {
                        diagnostic = diagnostic.with_related(location);
                    }

                    result.diagnostics.push(diagnostic);
                    malformed.insert(current_id);
                    malformed.insert(child);
                    result.complete = false;
                }
                _ => {}
            }
        }
    }

    // A node can be structurally malformed even if its children are valid.
    // Propagate inability to type-check from children to parents, rather than
    // passing invented placeholder types to the type checker.
    for id in postorder {
        let Some(node) = nodes.get(&id) else {
            continue;
        };

        if malformed.contains(&id) {
            result.statuses.insert(id, ExpressionStatus::Invalid);
            continue;
        }

        let mut child_types = Vec::with_capacity(node.children.len());
        let mut missing_child_type = false;

        for child_id in &node.children {
            match result.types.get(child_id) {
                Some(ty) => child_types.push(ty.clone()),
                None => {
                    missing_child_type = true;
                    let related = nodes.get(child_id).map(|n| n.location.clone());

                    let mut diagnostic = ExpressionDiagnostic::new(
                        ExpressionDiagnosticCode::MissingChildType,
                        "The expression cannot be type-checked because a child expression has no established type.",
                        Some(id.clone()),
                        Some(node.location.clone()),
                    );

                    if let Some(location) = related {
                        diagnostic = diagnostic.with_related(location);
                    }

                    result.diagnostics.push(diagnostic);
                }
            }
        }

        if missing_child_type {
            result.statuses.insert(id, ExpressionStatus::Incomplete);
            result.complete = false;
            continue;
        }

        let outcome = type_checker.infer(node, &child_types);

        for issue in outcome.issues {
            result.diagnostics.push(ExpressionDiagnostic::new(
                ExpressionDiagnosticCode::TypeCheckFailure,
                format!("{}: {}", issue.code, issue.message),
                Some(id.clone()),
                Some(node.location.clone()),
            ).with_related_many(issue.related));
        }

        match outcome.ty {
            Some(ty) if !ty.0.trim().is_empty() => {
                result.types.insert(id.clone(), ty.clone());
                result.statuses.insert(id, ExpressionStatus::Typed(ty));
            }
            Some(_) => {
                result.diagnostics.push(ExpressionDiagnostic::new(
                    ExpressionDiagnosticCode::EmptyTypeIdentity,
                    "The type checker returned an empty semantic type identity.",
                    Some(id.clone()),
                    Some(node.location.clone()),
                ));
                result.statuses.insert(id, ExpressionStatus::Invalid);
                result.complete = false;
            }
            None => {
                result.statuses.insert(id, ExpressionStatus::Invalid);
                result.complete = false;
            }
        }
    }

    // Every requested root must have a successfully established type for the
    // overall expression-validation result to be complete.
    for root in roots {
        if !matches!(result.statuses.get(&root), Some(ExpressionStatus::Typed(_))) {
            result.complete = false;
        }
    }

    // Ensure deterministic output independent of map insertion details.
    result.diagnostics.sort_by(|a, b| {
        (
            a.primary.as_ref(),
            a.code,
            a.expression.as_ref(),
            &a.message,
            &a.related,
        )
            .cmp(&(
                b.primary.as_ref(),
                b.code,
                b.expression.as_ref(),
                &b.message,
                &b.related,
            ))
    });
    result.diagnostics.dedup();

    result
}

impl ExpressionDiagnostic {
    fn with_related_many(mut self, locations: Vec<SourceLocation>) -> Self {
        self.related.extend(locations);
        self.related.sort();
        self.related.dedup();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestProvider {
        roots: Vec<ExpressionId>,
        nodes: BTreeMap<ExpressionId, ExpressionNode>,
    }

    impl TestProvider {
        fn add(&mut self, id: &str, children: &[&str]) {
            let node_id = ExpressionId(id.to_owned());
            let node = ExpressionNode {
                id: node_id.clone(),
                kind: "test-expression".to_owned(),
                location: SourceLocation {
                    source_id: "test.zm".to_owned(),
                    start: 0,
                    end: 1,
                },
                children: children
                    .iter()
                    .map(|child| ExpressionId((*child).to_owned()))
                    .collect(),
            };
            self.nodes.insert(node_id, node);
        }
    }

    impl ExpressionProvider for TestProvider {
        fn roots(&self) -> Vec<ExpressionId> {
            self.roots.clone()
        }

        fn get(&self, id: &ExpressionId) -> Option<ExpressionNode> {
            self.nodes.get(id).cloned()
        }
    }

    struct TestTypeChecker;

    impl ExpressionTypeChecker for TestTypeChecker {
        fn infer(
            &self,
            node: &ExpressionNode,
            _child_types: &[TypeRef],
        ) -> TypeCheckOutcome {
            TypeCheckOutcome {
                ty: Some(TypeRef(format!("type:{}", node.kind))),
                issues: Vec::new(),
            }
        }
    }

    #[test]
    fn validates_a_simple_expression() {
        let mut provider = TestProvider::default();
        provider.add("leaf", &[]);
        provider.roots.push(ExpressionId("leaf".to_owned()));

        let result = validate_expressions(&provider, &TestTypeChecker);

        assert!(result.is_complete());
        assert!(!result.has_errors());
        assert!(result.type_of(&ExpressionId("leaf".to_owned())).is_some());
    }

    #[test]
    fn checks_children_before_parent() {
        let mut provider = TestProvider::default();
        provider.add("parent", &["child"]);
        provider.add("child", &[]);
        provider.roots.push(ExpressionId("parent".to_owned()));

        let result = validate_expressions(&provider, &TestTypeChecker);

        assert!(result.is_complete());
        assert!(result.type_of(&ExpressionId("child".to_owned())).is_some());
        assert!(result.type_of(&ExpressionId("parent".to_owned())).is_some());
    }

    #[test]
    fn reports_missing_child_nodes() {
        let mut provider = TestProvider::default();
        provider.add("parent", &["missing"]);
        provider.roots.push(ExpressionId("parent".to_owned()));

        let result = validate_expressions(&provider, &TestTypeChecker);

        assert!(!result.is_complete());
        assert!(result.diagnostics.iter().any(|d| {
            d.code == ExpressionDiagnosticCode::MissingExpressionNode
        }));
        assert!(result.diagnostics.iter().any(|d| {
            d.code == ExpressionDiagnosticCode::MissingChildType
        }));
    }

    #[test]
    fn reports_expression_cycles() {
        let mut provider = TestProvider::default();
        provider.add("a", &["b"]);
        provider.add("b", &["a"]);
        provider.roots.push(ExpressionId("a".to_owned()));

        let result = validate_expressions(&provider, &TestTypeChecker);

        assert!(!result.is_complete());
        assert!(result.diagnostics.iter().any(|d| {
            d.code == ExpressionDiagnosticCode::ExpressionCycle
        }));
    }

    #[test]
    fn rejects_invalid_source_locations() {
        let mut provider = TestProvider::default();
        provider.add("leaf", &[]);
        provider.roots.push(ExpressionId("leaf".to_owned()));

        let node = provider
            .nodes
            .get_mut(&ExpressionId("leaf".to_owned()))
            .expect("test node exists");
        node.location.start = 9;
        node.location.end = 2;

        let result = validate_expressions(&provider, &TestTypeChecker);

        assert!(!result.is_complete());
        assert!(result.diagnostics.iter().any(|d| {
            d.code == ExpressionDiagnosticCode::InvalidSourceLocation
        }));
    }

    #[test]
    fn rejects_empty_type_identities() {
        struct EmptyTypeChecker;

        impl ExpressionTypeChecker for EmptyTypeChecker {
            fn infer(
                &self,
                _node: &ExpressionNode,
                _child_types: &[TypeRef],
            ) -> TypeCheckOutcome {
                TypeCheckOutcome {
                    ty: Some(TypeRef(String::new())),
                    issues: Vec::new(),
                }
            }
        }

        let mut provider = TestProvider::default();
        provider.add("leaf", &[]);
        provider.roots.push(ExpressionId("leaf".to_owned()));

        let result = validate_expressions(&provider, &EmptyTypeChecker);

        assert!(!result.is_complete());
        assert!(result.diagnostics.iter().any(|d| {
            d.code == ExpressionDiagnosticCode::EmptyTypeIdentity
        }));
    }

    #[test]
    fn diagnostics_are_deterministically_ordered() {
        let mut provider = TestProvider::default();
        provider.add("root", &["missing_b", "missing_a"]);
        provider.roots.push(ExpressionId("root".to_owned()));

        let first = validate_expressions(&provider, &TestTypeChecker);
        let second = validate_expressions(&provider, &TestTypeChecker);

        assert_eq!(first, second);
    }
}
