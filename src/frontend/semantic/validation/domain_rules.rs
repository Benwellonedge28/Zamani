
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Domain Rules
//!
//! Path: `src/frontend/semantic/validation/domain_rules.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Validate domain-related semantic facts after the canonical AST has been
//! structurally validated and domain identities have been extracted by the
//! AST-to-semantic adapter.
//!
//! This module establishes:
//!
//! - whether a referenced computational domain is registered;
//! - whether semantic rules are available for that domain;
//! - whether a domain is disabled by the active semantic policy;
//! - whether a cross-domain boundary is permitted;
//! - whether a boundary requires an explicit source-level declaration;
//! - whether a boundary remains unresolved and therefore prevents complete
//!   semantic validation;
//! - whether configured operational budgets were exhausted.
//!
//! It is the shared cross-domain validation boundary for classical, quantum,
//! HDL, hybrid, AI/data, distributed, accelerator, and future domains.
//!
//! ## AUTHORITY
//!
//! Syntax and domain constructs are governed by:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/syntax.md`
//! - `grammar/specification/domains.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/hybrid/quantum-classical.g4`
//! - `grammar/hybrid/classical-quantum.g4`
//! - `grammar/quantum/quantum.g4`
//! - `grammar/hdl/hdl.g4`
//! - `src/frontend/ast/node/domains/`
//!
//! The canonical AST owns source structure and source-level domain identities.
//! `domains/registry.rs` owns registered domain definitions.
//! Individual `domains/*.rs` validators own domain-specific semantic rules.
//! The shared diagnostic subsystem owns repository-wide diagnostic conversion.
//!
//! ## OWNS
//!
//! - The cross-domain validation input contract.
//! - The interface for querying domain registration and boundary policies.
//! - Cross-domain boundary validation.
//! - Domain-rule availability checks.
//! - Stable module-local issue identifiers and categories.
//! - Explicit completeness and operational-budget reporting.
//!
//! ## DOES_NOT_OWN
//!
//! - Grammar, lexing, parsing, or AST construction.
//! - A second AST or authoritative domain registry.
//! - Ordinary name resolution or the core type system.
//! - Quantum operation validation.
//! - HDL elaboration or signal-width validation.
//! - Classical expression validation.
//! - Effect inference or effect authorization.
//! - Capability evidence or target discovery.
//! - Resource allocation or target feasibility.
//! - Hardware placement, routing, scheduling, or execution.
//! - Classical IR, `quantum::ir`, or another intermediate representation.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The AST adapter creates `DomainRuleProgram` from the canonical AST and
//!    preserves source node identities.
//! 2. The semantic session supplies a `DomainRuleCatalog` backed by the
//!    authoritative domain registry and boundary/extension metadata.
//! 3. `validate_domain_rules` checks the normalized input without mutating
//!    the AST, registry, or semantic model.
//! 4. The caller converts `DomainRuleIssue` records to canonical diagnostics.
//! 5. The pipeline merges the report into its semantic result.
//! 6. Domain-specific validators remain responsible for their own rules.
//! 7. Incomplete validation must never be reported as successful certification.
//! 8. Target feasibility and physical resource allocation remain downstream.
//!
//! The adapter must not infer that an unknown construct is valid merely
//! because its domain identity is syntactically well-formed.
//!
//! ## SCALABILITY
//!
//! There are no fixed domain-count, boundary-count, machine-size, qubit-count,
//! tensor-rank, device-count, or operation-count ceilings.
//!
//! Collections grow according to available memory and the caller's explicit
//! operational policy. If a configured budget is exhausted, the report is
//! marked incomplete. Unprocessed input is never silently treated as valid.
//!
//! Literal infinite input cannot be processed by a finite compiler process.
//! POCO-REAF here means that this module imposes no arbitrary language-level
//! capacity ceiling and preserves target-independent semantic meaning.
//!
//! ## DETERMINISM
//!
//! - Domain identities use the canonical AST `Domain` type.
//! - Catalog implementations must be stable for the duration of one analysis.
//! - Diagnostics are ordered by node identity, rule, related node, and message.
//! - No global mutable state, randomness, hardware queries, or I/O is used.
//!
//! ## FREEZE CRITERIA
//!
//! - The AST adapter is loss-preserving.
//! - The catalog is backed by the authoritative registry and policy metadata.
//! - Unknown or unvalidated domains cannot be certified as valid.
//! - Implicit cross-domain transitions obey the registered boundary policy.
//! - Budget exhaustion produces an incomplete report.
//! - Domain-specific validators are integrated by the semantic pipeline.
//! - Unit, integration, determinism, fuzz, and stress tests pass.
//! - Workspace formatting, linting, and tests pass.

use core::fmt;
use std::collections::BTreeMap;

use crate::frontend::ast::node::domains::domain::Domain;
use crate::frontend::ast::node::node_id::NodeId;

/// Schema version for this module's input and output contract.
///
/// This is independent of the language version, AST schema, domain registry
/// revision, and downstream IR versions.
pub const DOMAIN_RULES_SCHEMA_VERSION: u16 = 1;

/// Result of validating cross-domain semantic rules.
pub type DomainRulesResult<T> = Result<T, DomainRulesInputError>;

/// A namespaced identity for a source construct or boundary kind.
///
/// This identity is deliberately open-ended. Adding a computational domain,
/// conversion kind, synchronization construct, or extension does not require
/// extending a central enum.
///
/// Namespace and name are source-level metadata, not hardware identities.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainRuleIdentity {
    namespace: String,
    name: String,
}

impl DomainRuleIdentity {
    /// Construct a validated namespaced identity.
    ///
    /// Empty, surrounding-whitespace, and control-character components are
    /// rejected. No arbitrary maximum length or namespace depth is imposed.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> DomainRulesResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(&namespace, "namespace")?;
        validate_identity_component(&name, "name")?;

        Ok(Self { namespace, name })
    }

    /// Return the namespace component.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the local name component.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical display spelling.
    ///
    /// Semantic equality is based on the structured identity, not this
    /// allocated display string.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for DomainRuleIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.namespace, self.name)
    }
}

/// Input-construction errors.
///
/// These errors indicate malformed adapter input, not a semantic error in
/// the source program.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomainRulesInputError {
    /// An identity component is empty or consists only of whitespace.
    EmptyIdentityComponent {
        /// Name of the rejected component.
        component: &'static str,
    },

    /// An identity component has surrounding whitespace.
    SurroundingWhitespace {
        /// Name of the rejected component.
        component: &'static str,
    },

    /// An identity component contains a Unicode control character.
    ControlCharacter {
        /// Name of the rejected component.
        component: &'static str,

        /// The rejected character.
        character: char,
    },
}

impl fmt::Display for DomainRulesInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentityComponent { component } => {
                write!(formatter, "{component} must not be empty")
            }
            Self::SurroundingWhitespace { component } => {
                write!(
                    formatter,
                    "{component} must not contain surrounding whitespace"
                )
            }
            Self::ControlCharacter {
                component,
                character,
            } => {
                write!(
                    formatter,
                    "{component} contains control character U+{:04X}",
                    u32::from(*character)
                )
            }
        }
    }
}

impl std::error::Error for DomainRulesInputError {}

fn validate_identity_component(
    value: &str,
    component: &'static str,
) -> DomainRulesResult<()> {
    if value.trim().is_empty() {
        return Err(DomainRulesInputError::EmptyIdentityComponent {
            component,
        });
    }

    if value.trim() != value {
        return Err(DomainRulesInputError::SurroundingWhitespace {
            component,
        });
    }

    if let Some(character) = value.chars().find(char::is_control) {
        return Err(DomainRulesInputError::ControlCharacter {
            component,
            character,
        });
    }

    Ok(())
}

/// The kind of source construct associated with a domain use.
///
/// This is metadata used for diagnostics and registry policy. It does not
/// replace the canonical AST construct or its semantic validator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainUse {
    /// AST node containing or introducing the domain use.
    pub node: NodeId,

    /// Canonical source-level domain identity.
    pub domain: Domain,

    /// Namespaced identity of the source construct using the domain.
    pub construct: DomainRuleIdentity,
}

impl DomainUse {
    /// Construct a domain-use record.
    #[must_use]
    pub fn new(
        node: NodeId,
        domain: Domain,
        construct: DomainRuleIdentity,
    ) -> Self {
        Self {
            node,
            domain,
            construct,
        }
    }
}

/// A source-level boundary between two computational domains.
///
/// `from_node` and `to_node` identify the source constructs on either side of
/// the boundary. They are AST identities, not runtime values or hardware IDs.
///
/// `explicit` means that the source explicitly declares the relevant boundary
/// or conversion according to the adapter's interpretation of the grammar.
/// It must not be inferred merely because two constructs have different types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainBoundary {
    /// Source construct at the origin of the boundary.
    pub from_node: NodeId,

    /// Source construct at the destination of the boundary.
    pub to_node: NodeId,

    /// Domain of the originating construct.
    pub from_domain: Domain,

    /// Domain of the receiving construct.
    pub to_domain: Domain,

    /// Namespaced identity of the boundary construct.
    pub kind: DomainRuleIdentity,

    /// Whether the source explicitly declares this boundary.
    pub explicit: bool,
}

impl DomainBoundary {
    /// Construct a cross-domain boundary record.
    #[must_use]
    pub fn new(
        from_node: NodeId,
        to_node: NodeId,
        from_domain: Domain,
        to_domain: Domain,
        kind: DomainRuleIdentity,
        explicit: bool,
    ) -> Self {
        Self {
            from_node,
            to_node,
            from_domain,
            to_domain,
            kind,
            explicit,
        }
    }
}

/// Normalized, target-independent input for domain-rule validation.
///
/// This is an adapter contract, not an AST. The AST adapter preserves source
/// information and produces these records; the canonical AST remains the
/// source of truth.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DomainRuleProgram {
    /// Domain uses in deterministic source/traversal order.
    pub uses: Vec<DomainUse>,

    /// Cross-domain boundaries in deterministic source/traversal order.
    pub boundaries: Vec<DomainBoundary>,
}

impl DomainRuleProgram {
    /// Create an empty input program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a domain use.
    pub fn push_use(&mut self, domain_use: DomainUse) {
        self.uses.push(domain_use);
    }

    /// Append a domain boundary.
    pub fn push_boundary(&mut self, boundary: DomainBoundary) {
        self.boundaries.push(boundary);
    }

    /// Return whether no domain facts were supplied.
    ///
    /// An empty input is not by itself proof that a complete AST was
    /// semantically validated. The caller must establish that extraction from
    /// the AST completed successfully.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.uses.is_empty() && self.boundaries.is_empty()
    }
}

/// Registration state of a source-level domain.
///
/// `RegisteredWithoutRules` is intentionally distinct from `Registered`:
/// knowing that a domain exists does not mean the compiler has implemented
/// enough semantics to certify every construct in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomainRegistration {
    /// No matching domain definition is available in the active registry.
    Unknown,

    /// The domain is registered, but no applicable semantic validator exists.
    RegisteredWithoutRules,

    /// The domain is registered and its applicable semantic rules are
    /// available for the current analysis.
    Registered,

    /// The domain exists but is disallowed by the active language or extension
    /// policy.
    Disabled,
}

/// Policy for a particular kind of domain boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BoundaryPolicy {
    /// The boundary is permitted without an explicit conversion declaration.
    Allowed,

    /// The boundary is permitted only when explicitly represented in source.
    RequiresExplicit,

    /// The boundary is prohibited by the applicable semantic contract.
    Forbidden,

    /// No applicable rule is known; validation cannot establish correctness.
    Unspecified,
}

/// Registry/policy interface required by this validator.
///
/// Implement this interface using the authoritative domain registry and the
/// applicable language/extension policy. Do not create a second authoritative
/// registry in this module.
///
/// Implementations must return stable answers for the duration of a single
/// analysis invocation. They must not perform hardware discovery or select a
/// target.
pub trait DomainRuleCatalog {
    /// Resolve the registration and semantic-rule availability of a domain.
    fn domain_registration(&self, domain: &Domain) -> DomainRegistration;

    /// Return the boundary policy for a directed domain crossing and its
    /// namespaced boundary kind.
    ///
    /// `from` and `to` are source-level domain identities. They do not refer
    /// to devices, backends, physical resources, or runtime locations.
    fn boundary_policy(
        &self,
        from: &Domain,
        to: &Domain,
        kind: &DomainRuleIdentity,
    ) -> BoundaryPolicy;
}

/// Optional operational budgets for this validation pass.
///
/// `None` means this pass does not impose a limit for that dimension. These
/// are compiler-service safeguards, not source-language or hardware limits.
///
/// When a budget is exceeded, the report is incomplete even if no other issue
/// was found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DomainRulesBudget {
    /// Maximum domain-use records to inspect.
    pub max_domain_uses: Option<usize>,

    /// Maximum domain-boundary records to inspect.
    pub max_boundaries: Option<usize>,
}

impl DomainRulesBudget {
    /// Create a policy with no additional per-pass limits.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_domain_uses: None,
            max_boundaries: None,
        }
    }
}

/// Stable category of a domain-rule issue.
///
/// The repository-wide diagnostic registry should map these categories and
/// their `rule` identifiers to canonical diagnostic codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DomainRuleIssueKind {
    /// A domain identity is not registered.
    UnknownDomain,

    /// A registered domain has no applicable semantic validator.
    MissingDomainRules,

    /// The domain is disallowed by the active policy.
    DomainDisabled,

    /// A boundary requires an explicit declaration that is missing.
    ExplicitBoundaryRequired,

    /// A boundary is prohibited by the registered policy.
    ForbiddenBoundary,

    /// No applicable rule is known for a domain boundary.
    UnspecifiedBoundary,

    /// An operational budget prevented complete validation.
    OperationalBudgetExceeded,
}

impl fmt::Display for DomainRuleIssueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::UnknownDomain => "unknown domain",
            Self::MissingDomainRules => "missing domain rules",
            Self::DomainDisabled => "domain disabled",
            Self::ExplicitBoundaryRequired => "explicit boundary required",
            Self::ForbiddenBoundary => "forbidden domain boundary",
            Self::UnspecifiedBoundary => "unspecified domain boundary",
            Self::OperationalBudgetExceeded => "domain validation budget exceeded",
        };

        formatter.write_str(text)
    }
}

/// One structured issue produced by this validator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainRuleIssue {
    /// Category of the issue.
    pub kind: DomainRuleIssueKind,

    /// Primary AST node, if one is available.
    pub node: Option<NodeId>,

    /// Related AST node, such as the other side of a domain boundary.
    pub related_node: Option<NodeId>,

    /// Stable module-local rule identifier.
    ///
    /// Consumers must not classify diagnostics by parsing `message`.
    pub rule: &'static str,

    /// Human-readable explanation.
    pub message: String,
}

impl DomainRuleIssue {
    fn new(
        kind: DomainRuleIssueKind,
        node: Option<NodeId>,
        related_node: Option<NodeId>,
        rule: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            node,
            related_node,
            rule,
            message: message.into(),
        }
    }
}

/// Completion state for the domain-rule pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainRulesCompletion {
    /// All supplied facts were inspected and all applicable rules were known.
    Complete,

    /// One or more facts could not be fully checked.
    ///
    /// This includes unknown domains, missing validators, unspecified boundary
    /// policies, and operational budget exhaustion.
    Incomplete,
}

/// Result of validating domain uses and cross-domain boundaries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainRulesReport {
    /// Completion state of the pass.
    pub completion: DomainRulesCompletion,

    /// Deterministically ordered issues.
    pub issues: Vec<DomainRuleIssue>,

    /// Number of domain-use records actually inspected.
    pub checked_domain_uses: usize,

    /// Number of boundary records actually inspected.
    pub checked_boundaries: usize,
}

impl DomainRulesReport {
    /// Return whether the pass completed without reporting any issue.
    ///
    /// This is a local result only. The complete semantic pipeline must also
    /// verify that all other required passes completed successfully.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.completion == DomainRulesCompletion::Complete
            && self.issues.is_empty()
    }

    /// Return whether all supplied domain facts were inspected.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completion == DomainRulesCompletion::Complete
    }
}

/// Validate registered domain uses and cross-domain boundaries.
///
/// This function does not mutate the input or registry. It does not perform
/// type inference, effect checking, capability evidence checks, resource
/// feasibility analysis, or domain-specific operation validation.
///
/// Those responsibilities remain with their authoritative subsystems.
///
/// # Completeness
///
/// A report is incomplete if:
///
/// - a referenced domain is unknown;
/// - a registered domain has no applicable semantic validator;
/// - a boundary policy is unspecified; or
/// - an operational budget prevents inspection of all supplied records.
///
/// A disabled domain or a forbidden boundary is a semantic issue. If all
/// facts and policies were successfully examined, such errors do not by
/// themselves make the analysis incomplete.
///
/// # Determinism
///
/// Issues are sorted by primary node ID, rule identifier, related node ID,
/// and message. Catalog implementations must provide stable answers during
/// the call.
pub fn validate_domain_rules(
    program: &DomainRuleProgram,
    catalog: &impl DomainRuleCatalog,
    budget: DomainRulesBudget,
) -> DomainRulesReport {
    let mut issues = Vec::new();
    let mut complete = true;

    // Cache domain lookups within this invocation. The AST Domain type has
    // value-based ordering, so cache behavior is deterministic and does not
    // depend on hash-map iteration.
    let mut registrations: BTreeMap<Domain, DomainRegistration> =
        BTreeMap::new();

    let mut checked_domain_uses = 0usize;
    let use_limit = budget.max_domain_uses.unwrap_or(usize::MAX);
    let uses_to_check = program.uses.len().min(use_limit);

    for domain_use in program.uses.iter().take(uses_to_check) {
        checked_domain_uses += 1;

        let registration = *registrations
            .entry(domain_use.domain.clone())
            .or_insert_with(|| catalog.domain_registration(&domain_use.domain));

        match registration {
            DomainRegistration::Registered => {}
            DomainRegistration::Unknown => {
                complete = false;
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::UnknownDomain,
                    Some(domain_use.node),
                    None,
                    "domain.unknown",
                    format!(
                        "domain `{}` used by construct `{}` is not registered",
                        domain_use.domain, domain_use.construct
                    ),
                ));
            }
            DomainRegistration::RegisteredWithoutRules => {
                complete = false;
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::MissingDomainRules,
                    Some(domain_use.node),
                    None,
                    "domain.rules.missing",
                    format!(
                        "domain `{}` is registered but has no applicable semantic validator",
                        domain_use.domain
                    ),
                ));
            }
            DomainRegistration::Disabled => {
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::DomainDisabled,
                    Some(domain_use.node),
                    None,
                    "domain.disabled",
                    format!(
                        "domain `{}` is disabled by the active semantic policy",
                        domain_use.domain
                    ),
                ));
            }
        }
    }

    if uses_to_check < program.uses.len() {
        complete = false;
        let first_unchecked = program.uses.get(uses_to_check);

        issues.push(DomainRuleIssue::new(
            DomainRuleIssueKind::OperationalBudgetExceeded,
            first_unchecked.map(|item| item.node),
            None,
            "domain.budget.uses",
            format!(
                "domain-use validation budget exhausted: inspected {} of {} records",
                uses_to_check,
                program.uses.len()
            ),
        ));
    }

    let mut checked_boundaries = 0usize;
    let boundary_limit = budget.max_boundaries.unwrap_or(usize::MAX);
    let boundaries_to_check = program.boundaries.len().min(boundary_limit);

    for boundary in program.boundaries.iter().take(boundaries_to_check) {
        checked_boundaries += 1;

        // A same-domain edge does not cross a domain boundary. Its ordinary
        // type/effect/control-flow rules are owned by other validators.
        if boundary.from_domain == boundary.to_domain {
            continue;
        }

        let from_registration = lookup_registration(
            &boundary.from_domain,
            catalog,
            &mut registrations,
        );
        let to_registration = lookup_registration(
            &boundary.to_domain,
            catalog,
            &mut registrations,
        );

        if !registration_is_validated(from_registration)
            || !registration_is_validated(to_registration)
        {
            complete = false;

            // Domain-use records ordinarily provide the more specific
            // unknown-domain/missing-validator diagnostics. If this boundary
            // is the only record for a domain, still report its unresolved
            // endpoints here.
            report_unvalidated_boundary_endpoint(
                &mut issues,
                &boundary.from_domain,
                boundary.from_node,
                boundary.to_node,
                from_registration,
                "from",
            );

            report_unvalidated_boundary_endpoint(
                &mut issues,
                &boundary.to_domain,
                boundary.to_node,
                boundary.from_node,
                to_registration,
                "to",
            );

            // Do not infer a boundary policy when either endpoint's semantic
            // definition is unresolved.
            continue;
        }

        match catalog.boundary_policy(
            &boundary.from_domain,
            &boundary.to_domain,
            &boundary.kind,
        ) {
            BoundaryPolicy::Allowed => {}
            BoundaryPolicy::RequiresExplicit if boundary.explicit => {}
            BoundaryPolicy::RequiresExplicit => {
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::ExplicitBoundaryRequired,
                    Some(boundary.to_node),
                    Some(boundary.from_node),
                    "domain.boundary.explicit_required",
                    format!(
                        "boundary `{}` from `{}` to `{}` requires an explicit source-level conversion or boundary declaration",
                        boundary.kind,
                        boundary.from_domain,
                        boundary.to_domain
                    ),
                ));
            }
            BoundaryPolicy::Forbidden => {
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::ForbiddenBoundary,
                    Some(boundary.to_node),
                    Some(boundary.from_node),
                    "domain.boundary.forbidden",
                    format!(
                        "boundary `{}` from `{}` to `{}` is prohibited by the semantic contract",
                        boundary.kind,
                        boundary.from_domain,
                        boundary.to_domain
                    ),
                ));
            }
            BoundaryPolicy::Unspecified => {
                complete = false;
                issues.push(DomainRuleIssue::new(
                    DomainRuleIssueKind::UnspecifiedBoundary,
                    Some(boundary.to_node),
                    Some(boundary.from_node),
                    "domain.boundary.unspecified",
                    format!(
                        "no semantic boundary rule is registered for `{}` from `{}` to `{}`",
                        boundary.kind,
                        boundary.from_domain,
                        boundary.to_domain
                    ),
                ));
            }
        }
    }

    if boundaries_to_check < program.boundaries.len() {
        complete = false;
        let first_unchecked = program.boundaries.get(boundaries_to_check);

        issues.push(DomainRuleIssue::new(
            DomainRuleIssueKind::OperationalBudgetExceeded,
            first_unchecked.map(|item| item.to_node),
            first_unchecked.map(|item| item.from_node),
            "domain.budget.boundaries",
            format!(
                "domain-boundary validation budget exhausted: inspected {} of {} records",
                boundaries_to_check,
                program.boundaries.len()
            ),
        ));
    }

    issues.sort_by(|left, right| {
        left.node
            .map(NodeId::get)
            .cmp(&right.node.map(NodeId::get))
            .then_with(|| left.rule.cmp(right.rule))
            .then_with(|| {
                left.related_node
                    .map(NodeId::get)
                    .cmp(&right.related_node.map(NodeId::get))
            })
            .then_with(|| left.message.cmp(&right.message))
    });

    DomainRulesReport {
        completion: if complete {
            DomainRulesCompletion::Complete
        } else {
            DomainRulesCompletion::Incomplete
        },
        issues,
        checked_domain_uses,
        checked_boundaries,
    }
}

fn lookup_registration(
    domain: &Domain,
    catalog: &impl DomainRuleCatalog,
    cache: &mut BTreeMap<Domain, DomainRegistration>,
) -> DomainRegistration {
    *cache
        .entry(domain.clone())
        .or_insert_with(|| catalog.domain_registration(domain))
}

fn registration_is_validated(registration: DomainRegistration) -> bool {
    matches!(registration, DomainRegistration::Registered)
}

fn report_unvalidated_boundary_endpoint(
    issues: &mut Vec<DomainRuleIssue>,
    domain: &Domain,
    node: NodeId,
    related_node: NodeId,
    registration: DomainRegistration,
    endpoint: &'static str,
) {
    match registration {
        DomainRegistration::Unknown => {
            issues.push(DomainRuleIssue::new(
                DomainRuleIssueKind::UnknownDomain,
                Some(node),
                Some(related_node),
                "domain.boundary.endpoint_unknown",
                format!(
                    "the {endpoint} endpoint of a domain boundary refers to unregistered domain `{domain}`"
                ),
            ));
        }
        DomainRegistration::RegisteredWithoutRules => {
            issues.push(DomainRuleIssue::new(
                DomainRuleIssueKind::MissingDomainRules,
                Some(node),
                Some(related_node),
                "domain.boundary.endpoint_rules_missing",
                format!(
                    "the {endpoint} endpoint uses domain `{domain}`, which has no applicable semantic validator"
                ),
            ));
        }
        DomainRegistration::Disabled => {
            issues.push(DomainRuleIssue::new(
                DomainRuleIssueKind::DomainDisabled,
                Some(node),
                Some(related_node),
                "domain.boundary.endpoint_disabled",
                format!(
                    "the {endpoint} endpoint uses domain `{domain}`, which is disabled by policy"
                ),
            ));
        }
        DomainRegistration::Registered => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[derive(Default)]
    struct TestCatalog {
        domains: BTreeMap<Domain, DomainRegistration>,
        boundaries: BTreeMap<(Domain, Domain, DomainRuleIdentity), BoundaryPolicy>,
        default_boundary: BoundaryPolicy,
    }

    impl DomainRuleCatalog for TestCatalog {
        fn domain_registration(&self, domain: &Domain) -> DomainRegistration {
            self.domains
                .get(domain)
                .copied()
                .unwrap_or(DomainRegistration::Unknown)
        }

        fn boundary_policy(
            &self,
            from: &Domain,
            to: &Domain,
            kind: &DomainRuleIdentity,
        ) -> BoundaryPolicy {
            self.boundaries
                .get(&(from.clone(), to.clone(), kind.clone()))
                .copied()
                .unwrap_or(self.default_boundary)
        }
    }

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn domain(namespace: &str, name: &str) -> Domain {
        Domain::new(namespace, name).expect("test domain identity must be valid")
    }

    fn identity(namespace: &str, name: &str) -> DomainRuleIdentity {
        DomainRuleIdentity::new(namespace, name)
            .expect("test semantic identity must be valid")
    }

    fn use_of(node_id: u64, domain: Domain) -> DomainUse {
        DomainUse::new(node(node_id), domain, identity("zamani.test", "call"))
    }

    fn boundary(
        from_node: u64,
        to_node: u64,
        from: Domain,
        to: Domain,
        explicit: bool,
    ) -> DomainBoundary {
        DomainBoundary::new(
            node(from_node),
            node(to_node),
            from,
            to,
            identity("zamani.test", "conversion"),
            explicit,
        )
    }

    #[test]
    fn registered_domain_with_rules_is_accepted() {
        let classical = domain("zamani", "classical");
        let mut catalog = TestCatalog::default();
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_use(use_of(1, classical));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert!(report.is_clean());
        assert_eq!(report.checked_domain_uses, 1);
    }

    #[test]
    fn unknown_domain_is_not_certified() {
        let mut program = DomainRuleProgram::new();
        program.push_use(use_of(1, domain("future.example", "compute")));

        let report = validate_domain_rules(
            &program,
            &TestCatalog::default(),
            DomainRulesBudget::unrestricted(),
        );

        assert!(!report.is_complete());
        assert_eq!(report.issues.len(), 1);
        assert_eq!(
            report.issues[0].kind,
            DomainRuleIssueKind::UnknownDomain
        );
    }

    #[test]
    fn registered_domain_without_validator_is_incomplete() {
        let quantum = domain("zamani", "quantum");
        let mut catalog = TestCatalog::default();
        catalog
            .domains
            .insert(quantum.clone(), DomainRegistration::RegisteredWithoutRules);

        let mut program = DomainRuleProgram::new();
        program.push_use(use_of(7, quantum));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert_eq!(
            report.issues[0].kind,
            DomainRuleIssueKind::MissingDomainRules
        );
        assert!(!report.is_complete());
    }

    #[test]
    fn implicit_boundary_is_rejected_when_explicit_conversion_is_required() {
        let classical = domain("zamani", "classical");
        let quantum = domain("zamani", "quantum");
        let kind = identity("zamani.test", "conversion");

        let mut catalog = TestCatalog {
            default_boundary: BoundaryPolicy::RequiresExplicit,
            ..TestCatalog::default()
        };
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);
        catalog
            .domains
            .insert(quantum.clone(), DomainRegistration::Registered);
        catalog.boundaries.insert(
            (classical.clone(), quantum.clone(), kind.clone()),
            BoundaryPolicy::RequiresExplicit,
        );

        let mut program = DomainRuleProgram::new();
        program.push_boundary(DomainBoundary::new(
            node(10),
            node(11),
            classical,
            quantum,
            kind,
            false,
        ));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert!(report.is_complete());
        assert_eq!(
            report.issues[0].kind,
            DomainRuleIssueKind::ExplicitBoundaryRequired
        );
    }

    #[test]
    fn explicit_boundary_satisfies_explicit_boundary_policy() {
        let classical = domain("zamani", "classical");
        let quantum = domain("zamani", "quantum");
        let mut catalog = TestCatalog {
            default_boundary: BoundaryPolicy::RequiresExplicit,
            ..TestCatalog::default()
        };
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);
        catalog
            .domains
            .insert(quantum.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_boundary(boundary(
            20,
            21,
            classical,
            quantum,
            true,
        ));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert!(report.is_clean());
    }

    #[test]
    fn forbidden_boundary_is_reported() {
        let classical = domain("zamani", "classical");
        let hdl = domain("zamani", "hdl");
        let mut catalog = TestCatalog {
            default_boundary: BoundaryPolicy::Forbidden,
            ..TestCatalog::default()
        };
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);
        catalog
            .domains
            .insert(hdl.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_boundary(boundary(30, 31, classical, hdl, true));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert_eq!(
            report.issues[0].kind,
            DomainRuleIssueKind::ForbiddenBoundary
        );
        assert!(report.is_complete());
    }

    #[test]
    fn unspecified_boundary_policy_makes_analysis_incomplete() {
        let classical = domain("zamani", "classical");
        let future = domain("future.example", "compute");
        let mut catalog = TestCatalog::default();
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);
        catalog
            .domains
            .insert(future.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_boundary(boundary(40, 41, classical, future, true));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert!(!report.is_complete());
        assert_eq!(
            report.issues[0].kind,
            DomainRuleIssueKind::UnspecifiedBoundary
        );
    }

    #[test]
    fn same_domain_relationship_does_not_require_boundary_registration() {
        let classical = domain("zamani", "classical");
        let mut catalog = TestCatalog::default();
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_boundary(boundary(
            50,
            51,
            classical.clone(),
            classical,
            false,
        ));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert!(report.is_clean());
        assert_eq!(report.checked_boundaries, 1);
    }

    #[test]
    fn exhausting_a_domain_use_budget_marks_report_incomplete() {
        let classical = domain("zamani", "classical");
        let mut catalog = TestCatalog::default();
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_use(use_of(60, classical.clone()));
        program.push_use(use_of(61, classical));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget {
                max_domain_uses: Some(1),
                max_boundaries: None,
            },
        );

        assert!(!report.is_complete());
        assert_eq!(report.checked_domain_uses, 1);
        assert!(report.issues.iter().any(|issue| {
            issue.kind == DomainRuleIssueKind::OperationalBudgetExceeded
        }));
    }

    #[test]
    fn exhausting_a_boundary_budget_marks_report_incomplete() {
        let classical = domain("zamani", "classical");
        let quantum = domain("zamani", "quantum");
        let mut catalog = TestCatalog {
            default_boundary: BoundaryPolicy::Allowed,
            ..TestCatalog::default()
        };
        catalog
            .domains
            .insert(classical.clone(), DomainRegistration::Registered);
        catalog
            .domains
            .insert(quantum.clone(), DomainRegistration::Registered);

        let mut program = DomainRuleProgram::new();
        program.push_boundary(boundary(
            70,
            71,
            classical.clone(),
            quantum.clone(),
            true,
        ));
        program.push_boundary(boundary(
            72,
            73,
            classical,
            quantum,
            true,
        ));

        let report = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget {
                max_domain_uses: None,
                max_boundaries: Some(1),
            },
        );

        assert!(!report.is_complete());
        assert_eq!(report.checked_boundaries, 1);
        assert!(report.issues.iter().any(|issue| {
            issue.kind == DomainRuleIssueKind::OperationalBudgetExceeded
        }));
    }

    #[test]
    fn diagnostics_have_stable_order() {
        let mut catalog = TestCatalog::default();
        let mut program = DomainRuleProgram::new();

        program.push_use(use_of(9, domain("unknown", "z")));
        program.push_use(use_of(2, domain("unknown", "a")));

        let first = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );
        let second = validate_domain_rules(
            &program,
            &catalog,
            DomainRulesBudget::unrestricted(),
        );

        assert_eq!(first, second);
        assert_eq!(
            first.issues[0].node.map(NodeId::get),
            Some(2)
        );

        // Keep the mutable binding useful for test catalogs that grow
        // additional policy scenarios without requiring global state.
        catalog.default_boundary = BoundaryPolicy::Allowed;
    }

    #[test]
    fn namespaced_identity_rejects_invalid_components() {
        assert!(DomainRuleIdentity::new("", "conversion").is_err());
        assert!(DomainRuleIdentity::new("zamani", " ").is_err());
        assert!(DomainRuleIdentity::new(" zamani", "conversion").is_err());
        assert!(DomainRuleIdentity::new("zamani", "con\ntrol").is_err());

        assert!(
            DomainRuleIdentity::new(
                "future.example.deep.namespace",
                "new_boundary"
            )
            .is_ok()
        );
    }
}
