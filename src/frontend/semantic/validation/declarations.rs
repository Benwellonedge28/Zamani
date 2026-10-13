
#![forbid(unsafe_code)]

//! # Zamani Semantic Declaration Validation
//!
//! Path: src/frontend/semantic/validation/declarations.rs
//!
//! Status: Proposed implementation; repository integration and compilation
//! against the complete workspace are still required.
//!
//! ## Purpose
//!
//! Validate declaration-level semantic invariants after parsing and before
//! type checking, effect analysis, capability validation, or domain-specific
//! analysis.
//!
//! This module owns:
//! - declaration-fact validation;
//! - declaration identity collision detection;
//! - duplicate-name detection within a semantic scope and namespace;
//! - configurable overload handling;
//! - compatible declaration/definition pair handling;
//! - deterministic declaration indexing;
//! - structured declaration diagnostics.
//!
//! This module does not own:
//! - the canonical AST or its node identities;
//! - parsing, lexical validation, or grammar rules;
//! - import resolution or module loading;
//! - type inference, type compatibility, or generic solving;
//! - effect, capability, resource, or contract analysis;
//! - hardware discovery or target feasibility;
//! - IR generation, lowering, scheduling, routing, or execution.
//!
//! ## Integration boundary
//!
//! The semantic frontend adapts canonical AST declarations into
//! `DeclarationRecord` values. This module validates those facts without
//! inventing another AST or assuming that all AST declaration types share
//! the same Rust representation.
//!
//! The adapter must preserve:
//! - the stable AST node identity;
//! - the original source name;
//! - the semantic scope and namespace;
//! - the canonical declaration kind;
//! - the source span;
//! - the declaration/definition distinction;
//! - the canonical overload signature, when applicable.
//!
//! A future adapter must not invent a signature merely to make declarations
//! appear distinct. Signature construction belongs to the appropriate
//! declaration/type-analysis contract.
//!
//! ## Scalability and safety
//!
//! There are no language-level limits on declaration count, overload count,
//! generic arity, computational domains, hardware resources, or machine size.
//!
//! Work and memory scale with the declarations supplied to this pass and
//! available resources. Configured operational budgets belong to the semantic
//! session and must be enforced before or during collection by that session.
//!
//! This module does not silently truncate its input or claim that an
//! incompletely collected program has been fully validated.
//!
//! All maps use deterministic ordering. No global mutable state is used.
//! No unsafe Rust is used.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Stable source location using UTF-8 byte offsets and half-open ranges.
///
/// `source_id` is an opaque identifier assigned by the frontend source layer.
/// It is not assumed to be a filesystem path.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocation {
    /// Stable identity of the source document.
    pub source_id: String,

    /// Inclusive starting byte offset.
    pub start: u64,

    /// Exclusive ending byte offset.
    pub end: u64,
}

impl SourceLocation {
    /// Construct a source location.
    ///
    /// Validation of ordering and source identity is performed by this
    /// module's declaration validation pass so malformed adapter data can
    /// produce a structured diagnostic instead of being silently accepted.
    #[must_use]
    pub fn new(
        source_id: impl Into<String>,
        start: u64,
        end: u64,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            start,
            end,
        }
    }

    /// Whether this location has a non-empty source identity and a valid
    /// half-open range.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.source_id.trim().is_empty() && self.start <= self.end
    }
}

/// Declaration category supplied by the canonical AST adapter.
///
/// `name` is a stable, language-level category identifier such as
/// `"function"`, `"variable"`, `"type"`, or a namespaced extension category.
/// It is deliberately a string rather than a closed enum so new declaration
/// categories do not require modifying this validator.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclarationKind {
    name: String,
}

impl DeclarationKind {
    /// Create a declaration kind.
    ///
    /// Empty values are reported by declaration validation.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Return the stable category identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DeclarationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// Namespace in which a declaration participates in name lookup.
///
/// Examples include value, type, module, and macro namespaces. The actual
/// namespace rules must come from Zamani's normative language specification
/// and the AST adapter; this type does not impose a closed namespace list.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespaceId(String);

impl NamespaceId {
    /// Create a namespace identifier.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the namespace identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable semantic identity for a declaration.
///
/// The AST adapter must derive this from the canonical AST identity. It must
/// not derive identities from pointer addresses, hash-map iteration order,
/// randomness, or process-global counters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclarationId(String);

impl DeclarationId {
    /// Create a declaration identity from the adapter's stable representation.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the stable identity representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Whether a declaration introduces a symbol or defines its implementation.
///
/// The parser/AST adapter must only choose `Definition` when the source
/// construct actually supplies a definition under the language specification.
/// The validator does not infer this from the spelling of the declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeclarationForm {
    /// A declaration that introduces or describes a symbol without defining
    /// its implementation.
    Declaration,

    /// A definition of the symbol.
    Definition,
}

/// Normalized declaration facts consumed by this pass.
///
/// This is a semantic input record, not a replacement AST. Child expressions,
/// type expressions, generic parameter nodes, attributes, and source metadata
/// remain owned by the canonical AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationRecord {
    /// Canonical AST-derived declaration identity.
    pub id: DeclarationId,

    /// Original, unqualified declaration name.
    ///
    /// Qualified-path resolution belongs to the import/name-resolution layer.
    pub name: String,

    /// Declaration category from the canonical AST adapter.
    pub kind: DeclarationKind,

    /// Namespace in which this declaration participates.
    pub namespace: NamespaceId,

    /// Stable identity of the containing semantic scope.
    ///
    /// The root scope must also have an explicit identity.
    pub scope_id: String,

    /// Source location of the declaration name or declaration node, according
    /// to the adapter's documented span policy.
    pub location: SourceLocation,

    /// Whether the declaration is a declaration or definition.
    pub form: DeclarationForm,

    /// Canonical overload signature, when the language and type-analysis
    /// layer have established one.
    ///
    /// This must not be an unstable debug string or a hash that can collide.
    /// `None` means the declaration cannot yet be distinguished by signature.
    pub overload_key: Option<String>,
}

impl DeclarationRecord {
    /// Construct a normalized declaration record.
    #[must_use]
    pub fn new(
        id: DeclarationId,
        name: impl Into<String>,
        kind: DeclarationKind,
        namespace: NamespaceId,
        scope_id: impl Into<String>,
        location: SourceLocation,
        form: DeclarationForm,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind,
            namespace,
            scope_id: scope_id.into(),
            location,
            form,
            overload_key: None,
        }
    }

    /// Attach a canonical overload signature.
    ///
    /// Only use this for declaration categories that the language permits
    /// to overload and after a canonical signature has been established.
    #[must_use]
    pub fn with_overload_key(
        mut self,
        overload_key: impl Into<String>,
    ) -> Self {
        self.overload_key = Some(overload_key.into());
        self
    }
}

/// Configuration for declaration validation.
///
/// This configuration controls semantic policy, not the capacity of any
/// hardware or the maximum size of a valid Zamani program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationPolicy {
    /// Declaration categories for which distinct canonical signatures may
    /// share a name in the same scope and namespace.
    ///
    /// Empty by default: overloads are not presumed legal. The integration
    /// layer must populate this from normative language rules.
    pub overloadable_kinds: BTreeSet<DeclarationKind>,

    /// Permit exactly one declaration and one definition for the same
    /// declaration key, when their kinds and overload signatures agree.
    ///
    /// This does not permit duplicate definitions or repeated declarations.
    pub allow_declaration_definition_pair: bool,
}

impl Default for DeclarationPolicy {
    fn default() -> Self {
        Self {
            overloadable_kinds: BTreeSet::new(),
            allow_declaration_definition_pair: true,
        }
    }
}

impl DeclarationPolicy {
    /// Register a declaration category that supports overloading according
    /// to the normative language specification.
    pub fn allow_overloading(&mut self, kind: DeclarationKind) {
        self.overloadable_kinds.insert(kind);
    }

    /// Whether this declaration category may be overloaded.
    #[must_use]
    pub fn is_overloadable(&self, kind: &DeclarationKind) -> bool {
        self.overloadable_kinds.contains(kind)
    }
}

/// Stable diagnostic codes emitted by this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeclarationDiagnosticCode {
    /// Declaration identity is empty.
    EmptyDeclarationIdentity,

    /// Multiple declarations claim the same canonical identity.
    DuplicateDeclarationIdentity,

    /// Declaration name is empty or structurally malformed.
    InvalidDeclarationName,

    /// Declaration category is empty or malformed.
    InvalidDeclarationKind,

    /// Namespace identity is empty.
    InvalidNamespace,

    /// Containing scope identity is empty.
    InvalidScopeIdentity,

    /// Source identity is missing or the byte range is reversed.
    InvalidSourceLocation,

    /// An overload signature was supplied but is empty.
    EmptyOverloadSignature,

    /// A declaration conflicts with another declaration in its namespace.
    DuplicateDeclaration,

    /// A name is reused by incompatible declaration categories.
    ConflictingDeclarationKinds,

    /// More than one definition exists for the same declaration key.
    DuplicateDefinition,
}

impl DeclarationDiagnosticCode {
    /// Stable machine-readable code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDeclarationIdentity => "ZSEM-DECL-001",
            Self::DuplicateDeclarationIdentity => "ZSEM-DECL-002",
            Self::InvalidDeclarationName => "ZSEM-DECL-003",
            Self::InvalidDeclarationKind => "ZSEM-DECL-004",
            Self::InvalidNamespace => "ZSEM-DECL-005",
            Self::InvalidScopeIdentity => "ZSEM-DECL-006",
            Self::InvalidSourceLocation => "ZSEM-DECL-007",
            Self::EmptyOverloadSignature => "ZSEM-DECL-008",
            Self::DuplicateDeclaration => "ZSEM-DECL-009",
            Self::ConflictingDeclarationKinds => "ZSEM-DECL-010",
            Self::DuplicateDefinition => "ZSEM-DECL-011",
        }
    }
}

impl fmt::Display for DeclarationDiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A structured declaration diagnostic.
///
/// Diagnostics are data, not preformatted terminal output. Consumers such as
/// the compiler, IDE, test runner, or language server can render them
/// differently without parsing messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationDiagnostic {
    /// Stable diagnostic code.
    pub code: DeclarationDiagnosticCode,

    /// Human-readable explanation.
    pub message: String,

    /// Primary source location.
    pub primary: SourceLocation,

    /// Related declarations or locations, in deterministic order.
    pub related: Vec<SourceLocation>,
}

impl DeclarationDiagnostic {
    fn new(
        code: DeclarationDiagnosticCode,
        message: impl Into<String>,
        primary: SourceLocation,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            primary,
            related: Vec::new(),
        }
    }

    fn with_related(
        mut self,
        mut related: Vec<SourceLocation>,
    ) -> Self {
        related.sort();
        related.dedup();
        self.related = related;
        self
    }
}

/// Canonical declaration lookup key.
///
/// The key preserves scope and namespace. Consequently, a type and a value
/// with the same spelling are not automatically treated as duplicates if
/// Zamani defines those as separate namespaces.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclarationKey {
    /// Containing semantic scope.
    pub scope_id: String,

    /// Namespace of the declaration.
    pub namespace: NamespaceId,

    /// Unqualified declaration name.
    pub name: String,
}

/// Deterministic declaration index produced by this pass.
///
/// Duplicate declarations remain represented in the index so later analysis
/// can recover and report additional errors. Consumers must check the
/// validation result before treating the index as an authoritative symbol
/// environment.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeclarationIndex {
    entries: BTreeMap<DeclarationKey, Vec<DeclarationRecord>>,
}

impl DeclarationIndex {
    /// Look up declarations for a scope, namespace, and name.
    #[must_use]
    pub fn lookup(
        &self,
        scope_id: &str,
        namespace: &NamespaceId,
        name: &str,
    ) -> Option<&[DeclarationRecord]> {
        let key = DeclarationKey {
            scope_id: scope_id.to_owned(),
            namespace: namespace.clone(),
            name: name.to_owned(),
        };

        self.entries.get(&key).map(Vec::as_slice)
    }

    /// Iterate over indexed declaration groups in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&DeclarationKey, &[DeclarationRecord])> {
        self.entries
            .iter()
            .map(|(key, records)| (key, records.as_slice()))
    }

    /// Number of distinct scope/namespace/name keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the index contains no declaration groups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Result of declaration validation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeclarationValidationResult {
    /// Deterministically ordered declaration diagnostics.
    pub diagnostics: Vec<DeclarationDiagnostic>,

    /// Collected declaration facts, including conflicting declarations.
    pub index: DeclarationIndex,

    /// Number of records supplied to this validation pass.
    pub declarations_seen: usize,
}

impl DeclarationValidationResult {
    /// Whether this pass reported no declaration errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Whether this pass reported at least one declaration error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}

/// Validate a stream of normalized declaration facts.
///
/// The iterator is consumed exactly once. The validator retains the
/// declaration facts needed for indexing and diagnostics, so memory use is
/// proportional to the input size and available resources.
///
/// This function validates the supplied stream completely. If the semantic
/// session stopped collecting declarations because a configured budget was
/// exhausted, the session must not call this function and present its result
/// as complete analysis. Budget accounting and completeness belong to the
/// caller's session/result contract.
///
/// # Determinism
///
/// For the same records, policy, and canonical source identities, the result
/// is deterministic. Input iteration order does not affect diagnostic order.
///
/// # Errors
///
/// Invalid records produce diagnostics rather than panics. This function
/// does not silently discard invalid declarations.
#[must_use]
pub fn validate_declarations<I>(
    declarations: I,
    policy: &DeclarationPolicy,
) -> DeclarationValidationResult
where
    I: IntoIterator<Item = DeclarationRecord>,
{
    let mut result = DeclarationValidationResult::default();
    let mut identities: BTreeMap<DeclarationId, Vec<DeclarationRecord>> =
        BTreeMap::new();

    for declaration in declarations {
        result.declarations_seen = result.declarations_seen.saturating_add(1);

        validate_record(&declaration, &mut result.diagnostics);

        identities
            .entry(declaration.id.clone())
            .or_default()
            .push(declaration.clone());

        // Invalid keys cannot be used safely for symbol lookup. Preserve the
        // diagnostic but do not create a misleading index entry.
        if declaration.scope_id.trim().is_empty()
            || declaration.namespace.as_str().trim().is_empty()
            || !is_valid_name(&declaration.name)
        {
            continue;
        }

        let key = DeclarationKey {
            scope_id: declaration.scope_id.clone(),
            namespace: declaration.namespace.clone(),
            name: declaration.name.clone(),
        };

        result
            .index
            .entries
            .entry(key)
            .or_default()
            .push(declaration);
    }

    validate_identity_uniqueness(&identities, &mut result.diagnostics);

    for (key, records) in &mut result.index.entries {
        records.sort_by(|left, right| {
            left.location
                .cmp(&right.location)
                .then_with(|| left.id.cmp(&right.id))
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.form.cmp(&right.form))
                .then_with(|| left.overload_key.cmp(&right.overload_key))
        });

        validate_name_group(
            key,
            records,
            policy,
            &mut result.diagnostics,
        );
    }

    sort_diagnostics(&mut result.diagnostics);
    result
}

fn validate_record(
    declaration: &DeclarationRecord,
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    if declaration.id.as_str().trim().is_empty() {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::EmptyDeclarationIdentity,
            "declaration has an empty canonical identity",
            declaration.location.clone(),
        ));
    }

    if !is_valid_name(&declaration.name) {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::InvalidDeclarationName,
            "declaration name is empty or contains whitespace/control characters",
            declaration.location.clone(),
        ));
    }

    if declaration.kind.as_str().trim().is_empty()
        || contains_whitespace_or_control(declaration.kind.as_str())
    {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::InvalidDeclarationKind,
            "declaration kind is empty or malformed",
            declaration.location.clone(),
        ));
    }

    if declaration.namespace.as_str().trim().is_empty()
        || contains_whitespace_or_control(declaration.namespace.as_str())
    {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::InvalidNamespace,
            "declaration namespace is empty or malformed",
            declaration.location.clone(),
        ));
    }

    if declaration.scope_id.trim().is_empty()
        || contains_whitespace_or_control(&declaration.scope_id)
    {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::InvalidScopeIdentity,
            "declaration has an empty or malformed scope identity",
            declaration.location.clone(),
        ));
    }

    if !declaration.location.is_valid() {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::InvalidSourceLocation,
            "declaration has an invalid source identity or byte range",
            declaration.location.clone(),
        ));
    }

    if declaration
        .overload_key
        .as_ref()
        .is_some_and(|key| key.trim().is_empty())
    {
        diagnostics.push(DeclarationDiagnostic::new(
            DeclarationDiagnosticCode::EmptyOverloadSignature,
            "an overload signature was supplied but is empty",
            declaration.location.clone(),
        ));
    }
}

fn validate_identity_uniqueness(
    identities: &BTreeMap<DeclarationId, Vec<DeclarationRecord>>,
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    for (id, records) in identities {
        if id.as_str().trim().is_empty() || records.len() < 2 {
            continue;
        }

        let mut locations: Vec<SourceLocation> = records
            .iter()
            .map(|record| record.location.clone())
            .collect();

        locations.sort();
        locations.dedup();

        for record in records {
            let related = locations
                .iter()
                .filter(|location| **location != record.location)
                .cloned()
                .collect();

            diagnostics.push(
                DeclarationDiagnostic::new(
                    DeclarationDiagnosticCode::DuplicateDeclarationIdentity,
                    format!(
                        "canonical declaration identity '{}' is assigned to multiple records",
                        id.as_str()
                    ),
                    record.location.clone(),
                )
                .with_related(related),
            );
        }
    }
}

fn validate_name_group(
    key: &DeclarationKey,
    records: &[DeclarationRecord],
    policy: &DeclarationPolicy,
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    if records.len() < 2 {
        return;
    }

    let all_same_kind = records
        .iter()
        .all(|record| record.kind == records[0].kind);

    if !all_same_kind {
        report_conflict(
            DeclarationDiagnosticCode::ConflictingDeclarationKinds,
            format!(
                "name '{}' is used by incompatible declaration kinds in the same scope and namespace",
                key.name
            ),
            records,
            diagnostics,
        );
        return;
    }

    let kind = &records[0].kind;

    if policy.is_overloadable(kind) {
        validate_overload_group(key, records, policy, diagnostics);
    } else {
        validate_single_signature_group(key, records, policy, diagnostics);
    }
}

fn validate_overload_group(
    key: &DeclarationKey,
    records: &[DeclarationRecord],
    policy: &DeclarationPolicy,
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    let mut overloads: BTreeMap<Option<String>, Vec<DeclarationRecord>> =
        BTreeMap::new();

    for record in records {
        overloads
            .entry(record.overload_key.clone())
            .or_default()
            .push(record.clone());
    }

    // A missing signature is not evidence that two declarations are
    // different overloads. Keep all missing signatures in the same bucket.
    for overload_records in overloads.values() {
        if overload_records.len() < 2 {
            continue;
        }

        validate_single_signature_group(
            key,
            overload_records,
            policy,
            diagnostics,
        );
    }
}

fn validate_single_signature_group(
    key: &DeclarationKey,
    records: &[DeclarationRecord],
    policy: &DeclarationPolicy,
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    if records.len() < 2 {
        return;
    }

    if policy.allow_declaration_definition_pair
        && is_exact_declaration_definition_pair(records)
    {
        return;
    }

    let definitions = records
        .iter()
        .filter(|record| record.form == DeclarationForm::Definition)
        .count();

    let code = if definitions > 1 {
        DeclarationDiagnosticCode::DuplicateDefinition
    } else {
        DeclarationDiagnosticCode::DuplicateDeclaration
    };

    report_conflict(
        code,
        format!(
            "duplicate declaration '{}' in the same scope and namespace",
            key.name
        ),
        records,
        diagnostics,
    );
}

fn is_exact_declaration_definition_pair(
    records: &[DeclarationRecord],
) -> bool {
    if records.len() != 2 {
        return false;
    }

    let declarations = records
        .iter()
        .filter(|record| record.form == DeclarationForm::Declaration)
        .count();

    let definitions = records
        .iter()
        .filter(|record| record.form == DeclarationForm::Definition)
        .count();

    declarations == 1 && definitions == 1
}

fn report_conflict(
    code: DeclarationDiagnosticCode,
    message: String,
    records: &[DeclarationRecord],
    diagnostics: &mut Vec<DeclarationDiagnostic>,
) {
    let mut locations: Vec<SourceLocation> = records
        .iter()
        .map(|record| record.location.clone())
        .collect();

    locations.sort();
    locations.dedup();

    for record in records {
        let related = locations
            .iter()
            .filter(|location| **location != record.location)
            .cloned()
            .collect();

        diagnostics.push(
            DeclarationDiagnostic::new(
                code,
                message.clone(),
                record.location.clone(),
            )
            .with_related(related),
        );
    }
}

fn sort_diagnostics(diagnostics: &mut Vec<DeclarationDiagnostic>) {
    diagnostics.sort_by(|left, right| {
        left.primary
            .cmp(&right.primary)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.message.cmp(&right.message))
            .then_with(|| left.related.cmp(&right.related))
    });

    diagnostics.dedup();
}

fn is_valid_name(name: &str) -> bool {
    !name.trim().is_empty() && !contains_whitespace_or_control(name)
}

fn contains_whitespace_or_control(value: &str) -> bool {
    value.chars().any(|character| {
        character.is_whitespace() || character.is_control()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration(
        id: &str,
        name: &str,
        kind: &str,
        namespace: &str,
        scope: &str,
        start: u64,
        form: DeclarationForm,
    ) -> DeclarationRecord {
        DeclarationRecord::new(
            DeclarationId::new(id),
            name,
            DeclarationKind::new(kind),
            NamespaceId::new(namespace),
            scope,
            SourceLocation::new("test.zm", start, start + 1),
            form,
        )
    }

    fn policy_with_function_overloads() -> DeclarationPolicy {
        let mut policy = DeclarationPolicy::default();
        policy.allow_overloading(DeclarationKind::new("function"));
        policy
    }

    #[test]
    fn accepts_distinct_names() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "alpha", "variable", "value", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "beta", "variable", "value", "root", 10,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.is_valid());
        assert_eq!(result.declarations_seen, 2);
        assert_eq!(result.index.len(), 2);
    }

    #[test]
    fn detects_duplicate_names_in_the_same_scope_and_namespace() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "answer", "variable", "value", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "answer", "variable", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.has_errors());
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DeclarationDiagnosticCode::DuplicateDefinition
        }));
    }

    #[test]
    fn permits_same_spelling_in_different_scopes() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "value", "variable", "value", "scope-a", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "value", "variable", "value", "scope-b", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.is_valid());
    }

    #[test]
    fn keeps_namespaces_separate() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "Item", "type", "type", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "Item", "variable", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.is_valid());
    }

    #[test]
    fn permits_overloads_with_distinct_signatures() {
        let first = declaration(
            "n1", "convert", "function", "value", "root", 0,
            DeclarationForm::Definition,
        )
        .with_overload_key("(Integer)->Text");

        let second = declaration(
            "n2", "convert", "function", "value", "root", 20,
            DeclarationForm::Definition,
        )
        .with_overload_key("(Text)->Integer");

        let result = validate_declarations(
            [first, second],
            &policy_with_function_overloads(),
        );

        assert!(result.is_valid());
    }

    #[test]
    fn rejects_duplicate_overload_signatures() {
        let first = declaration(
            "n1", "convert", "function", "value", "root", 0,
            DeclarationForm::Definition,
        )
        .with_overload_key("(Integer)->Text");

        let second = declaration(
            "n2", "convert", "function", "value", "root", 20,
            DeclarationForm::Definition,
        )
        .with_overload_key("(Integer)->Text");

        let result = validate_declarations(
            [first, second],
            &policy_with_function_overloads(),
        );

        assert!(result.has_errors());
    }

    #[test]
    fn permits_one_declaration_and_one_definition() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "compute", "function", "value", "root", 0,
                    DeclarationForm::Declaration,
                ),
                declaration(
                    "n2", "compute", "function", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.is_valid());
    }

    #[test]
    fn rejects_two_definitions() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "compute", "function", "value", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "compute", "function", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DeclarationDiagnosticCode::DuplicateDefinition
        }));
    }

    #[test]
    fn detects_conflicting_declaration_kinds() {
        let result = validate_declarations(
            [
                declaration(
                    "n1", "entry", "variable", "value", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "n2", "entry", "function", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &policy_with_function_overloads(),
        );

        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == DeclarationDiagnosticCode::ConflictingDeclarationKinds
        }));
    }

    #[test]
    fn rejects_malformed_records_without_panicking() {
        let malformed = DeclarationRecord::new(
            DeclarationId::new(""),
            "bad name",
            DeclarationKind::new(""),
            NamespaceId::new(""),
            "",
            SourceLocation::new("", 10, 2),
            DeclarationForm::Definition,
        );

        let result = validate_declarations(
            [malformed],
            &DeclarationPolicy::default(),
        );

        assert!(result.has_errors());
        assert!(result.diagnostics.len() >= 5);
    }

    #[test]
    fn detects_reused_declaration_identity() {
        let result = validate_declarations(
            [
                declaration(
                    "same-id", "alpha", "variable", "value", "root", 0,
                    DeclarationForm::Definition,
                ),
                declaration(
                    "same-id", "beta", "variable", "value", "root", 20,
                    DeclarationForm::Definition,
                ),
            ],
            &DeclarationPolicy::default(),
        );

        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == DeclarationDiagnosticCode::DuplicateDeclarationIdentity
        }));
    }

    #[test]
    fn diagnostic_order_is_independent_of_input_order() {
        let first = declaration(
            "n1", "duplicate", "variable", "value", "root", 10,
            DeclarationForm::Definition,
        );
        let second = declaration(
            "n2", "duplicate", "variable", "value", "root", 30,
            DeclarationForm::Definition,
        );

        let forward = validate_declarations(
            [first.clone(), second.clone()],
            &DeclarationPolicy::default(),
        );

        let reverse = validate_declarations(
            [second, first],
            &DeclarationPolicy::default(),
        );

        assert_eq!(forward.diagnostics, reverse.diagnostics);
        assert_eq!(forward.index, reverse.index);
    }

    #[test]
    fn large_resource_requirements_are_not_relevant_to_declarations() {
        // This pass does not impose a program-size or hardware-size ceiling.
        // A stress test can generate declarations according to the configured
        // test budget without changing language-level semantic rules.
        let result = validate_declarations(
            std::iter::empty(),
            &DeclarationPolicy::default(),
        );

        assert!(result.is_valid());
        assert!(result.index.is_empty());
    }
}
