#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Optimization — Transformation Legality
//!
//! Path: src/ai/optimization/legality.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Establishes a fail-closed legality boundary for AI optimization
//! transformations. It validates structural edit consistency and
//! coordinates evidence-based semantic preservation assessment.
//!
//! This module does not create another graph, semantic model, IR,
//! optimizer, tensor system, execution engine, or hardware abstraction.
//!
//! OWNERSHIP
//!
//! This module owns legality evidence contracts, preservation obligation
//! identities, legality outcomes, structural plan verification,
//! evidence completeness verification, diagnostics, and the implementation
//! of the existing TransformationLegality interface.
//!
//! AUTHORITIES
//!
//! Transformation contracts:
//!     crate::ai::optimization::transformations
//!
//! Graph structure:
//!     crate::ai::optimization::graph
//!
//! AI semantic meaning:
//!     crate::frontend::semantic::domains::ai
//!
//! Numerical precision:
//!     crate::ai::optimization::precision
//!
//! Quantization:
//!     crate::ai::optimization::quantization
//!
//! Sparsity:
//!     crate::ai::optimization::sparsity
//!
//! Canonical tensor types:
//!     crate::ai::tensor::dtypes
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! POCO-REAF
//!
//! No universal limits are imposed on graph nodes, edges, edits,
//! obligations, transformations, parameters, tensor dimensions,
//! or execution targets.
//!
//! UNKNOWN OR MALFORMED EVIDENCE NEVER PROVES LEGALITY.
//!
//! Evidence references provide traceability, not cryptographic
//! authentication. Trusted semantic and IR verification adapters must
//! establish the truth of supplied proof statuses.
//!
//! This module validates plans before TransformationEngine commits them.
//! The canonical graph adapter remains responsible for atomic mutation.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::optimization::transformations::{
    GraphEdit, GraphRevision, LegalityResult, PreservationRequirements,
    TransformationGraph, TransformationLegality, TransformationPlan,
};

// ============================================================================
// OBLIGATION IDENTITIES
// ============================================================================

/// Stable, extensible identity of a semantic preservation obligation.
///
/// Namespaces allow future computational domains and extensions to
/// introduce obligations without modifying this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObligationId(String);

impl ObligationId {
    /// Creates an obligation identity.
    pub fn new(value: impl Into<String>) -> Result<Self, LegalityError> {
        let value = value.into();

        if value.trim().is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(LegalityError::InvalidObligationIdentity);
        }

        Ok(Self(value))
    }

    /// Returns the canonical identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObligationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// STANDARD OBLIGATIONS
// ============================================================================

/// Standard semantic preservation obligation identities.
///
/// These are open-ended identifiers rather than a closed enumeration.
pub mod obligations {
    /// Observable program behavior must remain equivalent.
    pub const OBSERVABLE_BEHAVIOR: &str =
        "zamani.semantic.observable_behavior";

    /// Canonical type and shape contracts must be preserved.
    pub const TYPES_AND_SHAPES: &str =
        "zamani.semantic.types_and_shapes";

    /// Observable effects must remain valid.
    pub const EFFECTS: &str = "zamani.semantic.effects";

    /// Required numerical semantics must be preserved.
    pub const NUMERICAL_SEMANTICS: &str =
        "zamani.ai.numerical_semantics";

    /// Required effect ordering must be preserved.
    pub const EFFECT_ORDERING: &str =
        "zamani.semantic.effect_ordering";

    /// State dependencies must remain valid.
    pub const STATE_DEPENDENCIES: &str =
        "zamani.semantic.state_dependencies";

    /// Provenance obligations must be preserved.
    pub const PROVENANCE: &str = "zamani.semantic.provenance";

    /// Security obligations must be preserved.
    pub const SECURITY: &str = "zamani.security.obligations";

    /// Determinism requirements must be preserved.
    pub const DETERMINISM: &str = "zamani.semantic.determinism";

    /// Resource requirements must be preserved.
    pub const RESOURCE_REQUIREMENTS: &str =
        "zamani.semantic.resource_requirements";

    /// Reproducibility requirements must be preserved.
    pub const REPRODUCIBILITY: &str =
        "zamani.semantic.reproducibility";

    /// Differentiation semantics must be preserved when applicable.
    pub const DIFFERENTIATION: &str = "zamani.ai.differentiation";

    /// Aliasing and ownership contracts must be preserved.
    pub const ALIASING: &str = "zamani.semantic.aliasing";

    /// Control and data dependency semantics must be preserved.
    pub const DEPENDENCIES: &str = "zamani.semantic.dependencies";
}

// ============================================================================
// EVIDENCE STATUS
// ============================================================================

/// Status of a preservation proof supplied by an authoritative verifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ProofStatus {
    /// The authoritative verifier established preservation.
    Proven,

    /// The authoritative verifier established a violation.
    Disproven,

    /// Preservation has not been established.
    Unknown,
}

// ============================================================================
// PRESERVATION EVIDENCE
// ============================================================================

/// Evidence for one semantic preservation obligation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalityEvidence {
    obligation: ObligationId,
    status: ProofStatus,
    references: BTreeSet<String>,
    explanation: String,
}

impl LegalityEvidence {
    /// Constructs preservation evidence.
    pub fn new(
        obligation: ObligationId,
        status: ProofStatus,
        references: BTreeSet<String>,
        explanation: impl Into<String>,
    ) -> Result<Self, LegalityError> {
        let explanation = explanation.into();

        if explanation.trim().is_empty() {
            return Err(LegalityError::MissingExplanation);
        }

        for reference in &references {
            validate_reference(reference)?;
        }

        if status == ProofStatus::Proven && references.is_empty() {
            return Err(LegalityError::MissingEvidenceReference);
        }

        Ok(Self {
            obligation,
            status,
            references,
            explanation,
        })
    }

    /// Returns the obligation identity.
    #[must_use]
    pub fn obligation(&self) -> &ObligationId {
        &self.obligation
    }

    /// Returns the proof status.
    #[must_use]
    pub const fn status(&self) -> ProofStatus {
        self.status
    }

    /// Returns the evidence references.
    #[must_use]
    pub fn references(&self) -> &BTreeSet<String> {
        &self.references
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

fn validate_reference(reference: &str) -> Result<(), LegalityError> {
    if reference.trim().is_empty()
        || reference.trim() != reference
        || reference.chars().any(char::is_control)
    {
        return Err(LegalityError::InvalidEvidenceReference);
    }

    Ok(())
}

// ============================================================================
// ASSESSMENT OUTCOME
// ============================================================================

/// Final legality classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LegalityStatus {
    /// All required obligations have been proven.
    Legal,

    /// At least one required obligation is disproven.
    Illegal,

    /// Required evidence is missing, unknown, or incomplete.
    Indeterminate,
}

// ============================================================================
// DIAGNOSTICS
// ============================================================================

/// Classification of a legality diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LegalityDiagnosticKind {
    /// Structural graph or edit inconsistency.
    Structural,

    /// A semantic preservation obligation was disproven.
    PreservationViolation,

    /// Required proof evidence is missing.
    MissingEvidence,

    /// Proof status is unknown.
    UnknownEvidence,

    /// The evidence provider failed.
    EvidenceProviderFailure,

    /// An operational resource failure occurred.
    ResourceFailure,

    /// A stale transformation plan was supplied.
    StalePlan,

    /// An unsupported or incomplete validation occurred.
    IncompleteValidation,
}

/// One deterministic legality diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalityDiagnostic {
    kind: LegalityDiagnosticKind,
    obligation: Option<ObligationId>,
    message: String,
}

impl LegalityDiagnostic {
    /// Returns the diagnostic kind.
    #[must_use]
    pub const fn kind(&self) -> LegalityDiagnosticKind {
        self.kind
    }

    /// Returns the associated obligation, if applicable.
    #[must_use]
    pub fn obligation(&self) -> Option<&ObligationId> {
        self.obligation.as_ref()
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// LEGALITY ASSESSMENT
// ============================================================================

/// Complete, immutable result of legality assessment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalityAssessment {
    status: LegalityStatus,
    diagnostics: Vec<LegalityDiagnostic>,
    evidence_references: BTreeSet<String>,
}

impl LegalityAssessment {
    fn new(
        status: LegalityStatus,
        diagnostics: Vec<LegalityDiagnostic>,
        evidence_references: BTreeSet<String>,
    ) -> Self {
        Self {
            status,
            diagnostics,
            evidence_references,
        }
    }

    /// Returns the final legality status.
    #[must_use]
    pub const fn status(&self) -> LegalityStatus {
        self.status
    }

    /// Returns whether the transformation is certified legal.
    #[must_use]
    pub const fn is_legal(&self) -> bool {
        matches!(self.status, LegalityStatus::Legal)
    }

    /// Returns whether the transformation is rejected.
    #[must_use]
    pub const fn is_illegal(&self) -> bool {
        matches!(self.status, LegalityStatus::Illegal)
    }

    /// Returns whether validation is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.status, LegalityStatus::Indeterminate)
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[LegalityDiagnostic] {
        &self.diagnostics
    }

    /// Returns evidence references.
    #[must_use]
    pub fn evidence_references(&self) -> &BTreeSet<String> {
        &self.evidence_references
    }

    /// Converts the assessment to the existing transformation contract.
    #[must_use]
    pub fn into_transformation_result(self) -> LegalityResult {
        let diagnostics = self
            .diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.message)
            .collect();

        match self.status {
            LegalityStatus::Legal => LegalityResult {
                legal: true,
                complete: true,
                diagnostics,
            },
            LegalityStatus::Illegal => LegalityResult {
                legal: false,
                complete: true,
                diagnostics,
            },
            LegalityStatus::Indeterminate => LegalityResult {
                legal: false,
                complete: false,
                diagnostics,
            },
        }
    }
}

// ============================================================================
// REQUIRED OBLIGATIONS
// ============================================================================

/// Resolves the obligations that must be preserved.
///
/// Observable behavior, types, effects, reproducibility, and dependency
/// semantics are mandatory. Other obligations are activated by the
/// existing preservation policy.
pub fn required_obligations(
    requirements: &PreservationRequirements,
) -> Result<BTreeSet<ObligationId>, LegalityError> {
    let mut result = BTreeSet::new();

    let mandatory = [
        obligations::OBSERVABLE_BEHAVIOR,
        obligations::TYPES_AND_SHAPES,
        obligations::EFFECTS,
        obligations::REPRODUCIBILITY,
        obligations::DEPENDENCIES,
        obligations::ALIASING,
    ];

    result
        .try_reserve(mandatory.len())
        .map_err(|_| LegalityError::AllocationFailed)?;

    for name in mandatory {
        result.insert(ObligationId::new(name)?);
    }

    let conditional = [
        (
            requirements.numerical_semantics,
            obligations::NUMERICAL_SEMANTICS,
        ),
        (
            requirements.effect_ordering,
            obligations::EFFECT_ORDERING,
        ),
        (
            requirements.state_dependencies,
            obligations::STATE_DEPENDENCIES,
        ),
        (requirements.provenance, obligations::PROVENANCE),
        (requirements.security, obligations::SECURITY),
        (requirements.determinism, obligations::DETERMINISM),
        (
            requirements.resource_requirements,
            obligations::RESOURCE_REQUIREMENTS,
        ),
    ];

    for (enabled, name) in conditional {
        if enabled {
            result.insert(ObligationId::new(name)?);
        }
    }

    Ok(result)
}

// ============================================================================
// EVIDENCE PROVIDER
// ============================================================================

/// Authoritative source of semantic preservation evidence.
///
/// Implementations must delegate to canonical semantic, numerical,
/// security, provenance, and IR verification services.
///
/// Returning `Proven` asserts that authoritative verification completed.
/// A reference alone does not establish proof. Providers must not convert
/// unavailable information into success.
pub trait LegalityEvidenceProvider<G: TransformationGraph> {
    /// Produces evidence for a proposed transformation.
    fn assess(
        &self,
        graph: &G,
        plan: &TransformationPlan<G::Node, G::Edge>,
        requirements: &PreservationRequirements,
        required: &BTreeSet<ObligationId>,
    ) -> Result<Vec<LegalityEvidence>, String>;
}

// ============================================================================
// LEGALITY ERROR
// ============================================================================

/// Errors encountered while performing legality validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LegalityError {
    /// Invalid obligation identity.
    InvalidObligationIdentity,

    /// Missing diagnostic explanation.
    MissingExplanation,

    /// Invalid evidence reference.
    InvalidEvidenceReference,

    /// A proven result has no traceable evidence reference.
    MissingEvidenceReference,

    /// An allocation reservation failed.
    AllocationFailed,

    /// The graph contains duplicate node identities.
    DuplicateNode,

    /// An edge references an undeclared node.
    MissingEndpoint,

    /// An edit refers to a nonexistent object.
    MissingEditTarget,

    /// The same edit is declared more than once.
    DuplicateEdit,

    /// A node is removed while incident edges remain.
    DanglingEdge,

    /// A new edge references a nonexistent node.
    InvalidNewEdge,

    /// An affected node is not part of the original graph.
    InvalidAffectedNode,

    /// A plan was constructed against an obsolete graph revision.
    StalePlan,

    /// A plan contains no edits.
    EmptyPlan,

    /// A required obligation has duplicate evidence.
    DuplicateEvidence,

    /// An evidence provider failed.
    EvidenceProviderFailure(String),
}

impl fmt::Display for LegalityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidObligationIdentity => {
                formatter.write_str("Invalid preservation obligation identity.")
            }
            Self::MissingExplanation => {
                formatter.write_str("Legality evidence requires an explanation.")
            }
            Self::InvalidEvidenceReference => {
                formatter.write_str("Invalid legality evidence reference.")
            }
            Self::MissingEvidenceReference => {
                formatter.write_str(
                    "Proven evidence requires a traceable reference.",
                )
            }
            Self::AllocationFailed => {
                formatter.write_str("Legality validation allocation failed.")
            }
            Self::DuplicateNode => {
                formatter.write_str(
                    "The canonical graph contains duplicate nodes.",
                )
            }
            Self::MissingEndpoint => {
                formatter.write_str(
                    "A graph edge references an undeclared endpoint.",
                )
            }
            Self::MissingEditTarget => {
                formatter.write_str(
                    "A graph edit references a nonexistent object.",
                )
            }
            Self::DuplicateEdit => {
                formatter.write_str(
                    "The transformation plan contains duplicate edits.",
                )
            }
            Self::DanglingEdge => {
                formatter.write_str(
                    "A removed node retains an incident edge.",
                )
            }
            Self::InvalidNewEdge => {
                formatter.write_str("A new edge has invalid endpoints.")
            }
            Self::InvalidAffectedNode => {
                formatter.write_str(
                    "A plan identifies a node outside the original graph.",
                )
            }
            Self::StalePlan => {
                formatter.write_str(
                    "The transformation plan uses a stale graph revision.",
                )
            }
            Self::EmptyPlan => {
                formatter.write_str(
                    "The transformation plan contains no edits.",
                )
            }
            Self::DuplicateEvidence => {
                formatter.write_str(
                    "The evidence provider returned duplicate obligations.",
                )
            }
            Self::EvidenceProviderFailure(reason) => {
                write!(
                    formatter,
                    "Legality evidence provider failed: {reason}",
                )
            }
        }
    }
}

impl std::error::Error for LegalityError {}

// ============================================================================
// STRUCTURAL EDIT IDENTITY
// ============================================================================

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum EditIdentity<N, E> {
    RemoveNode(N),
    RemoveEdge(E),
    AddEdge(E),
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

/// Validates the structural consistency of a proposed graph edit plan.
///
/// This function does not prove semantic equivalence.
pub fn validate_structure<G>(
    graph: &G,
    plan: &TransformationPlan<G::Node, G::Edge>,
) -> Result<(), LegalityError>
where
    G: TransformationGraph,
{
    if graph.revision() != plan.source_revision() {
        return Err(LegalityError::StalePlan);
    }

    if plan.is_empty() {
        return Err(LegalityError::EmptyPlan);
    }

    let nodes = graph.nodes();
    let edges = graph.edges();

    let mut original_nodes = BTreeSet::new();
    let mut original_edges = BTreeSet::new();

    for node in nodes {
        if !original_nodes.insert(node) {
            return Err(LegalityError::DuplicateNode);
        }
    }

    for edge in edges {
        let source = graph.source(&edge);
        let destination = graph.destination(&edge);

        if !original_nodes.contains(&source)
            || !original_nodes.contains(&destination)
        {
            return Err(LegalityError::MissingEndpoint);
        }

        original_edges.insert(edge);
    }

    let mut remaining_nodes = original_nodes.clone();
    let mut remaining_edges = original_edges.clone();
    let mut added_edges = BTreeSet::new();
    let mut identities = BTreeSet::new();

    for edit in plan.edits() {
        let identity = match edit {
            GraphEdit::RemoveNode(node) => {
                EditIdentity::RemoveNode(node.clone())
            }
            GraphEdit::RemoveEdge(edge) => {
                EditIdentity::RemoveEdge(edge.clone())
            }
            GraphEdit::AddEdge(edge) => {
                EditIdentity::AddEdge(edge.clone())
            }
        };

        if !identities.insert(identity) {
            return Err(LegalityError::DuplicateEdit);
        }

        match edit {
            GraphEdit::RemoveNode(node) => {
                if !original_nodes.contains(node)
                    || !remaining_nodes.remove(node)
                {
                    return Err(LegalityError::MissingEditTarget);
                }
            }
            GraphEdit::RemoveEdge(edge) => {
                if !original_edges.contains(edge)
                    || !remaining_edges.remove(edge)
                {
                    return Err(LegalityError::MissingEditTarget);
                }
            }
            GraphEdit::AddEdge(edge) => {
                added_edges.insert(edge.clone());
            }
        }
    }

    for edge in &added_edges {
        let source = graph.source(edge);
        let destination = graph.destination(edge);

        if !remaining_nodes.contains(&source)
            || !remaining_nodes.contains(&destination)
        {
            return Err(LegalityError::InvalidNewEdge);
        }

        if remaining_edges.contains(edge) {
            return Err(LegalityError::DuplicateEdit);
        }
    }

    for edge in remaining_edges.iter().chain(added_edges.iter()) {
        let source = graph.source(edge);
        let destination = graph.destination(edge);

        if !remaining_nodes.contains(&source)
            || !remaining_nodes.contains(&destination)
        {
            return Err(LegalityError::DanglingEdge);
        }
    }

    for node in plan.affected_nodes() {
        if !original_nodes.contains(node) {
            return Err(LegalityError::InvalidAffectedNode);
        }
    }

    Ok(())
}

// ============================================================================
// EVIDENCE-BASED VALIDATOR
// ============================================================================

/// Evidence-based transformation legality validator.
///
/// Semantic proof is delegated to an injected authoritative provider.
/// Structural consistency alone never establishes legality.
pub struct LegalityValidator<P> {
    provider: P,
}

impl<P> LegalityValidator<P> {
    /// Creates a validator with an authoritative evidence provider.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Returns the evidence provider.
    #[must_use]
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Returns ownership of the evidence provider.
    #[must_use]
    pub fn into_provider(self) -> P {
        self.provider
    }
}

impl<P> LegalityValidator<P> {
    /// Performs a complete legality assessment.
    pub fn assess<G>(
        &self,
        graph: &G,
        plan: &TransformationPlan<G::Node, G::Edge>,
        requirements: &PreservationRequirements,
    ) -> LegalityAssessment
    where
        G: TransformationGraph,
        P: LegalityEvidenceProvider<G>,
    {
        if let Err(error) = validate_structure(graph, plan) {
            let kind = match error {
                LegalityError::StalePlan => LegalityDiagnosticKind::StalePlan,
                LegalityError::AllocationFailed => {
                    LegalityDiagnosticKind::ResourceFailure
                }
                _ => LegalityDiagnosticKind::Structural,
            };

            return LegalityAssessment::new(
                LegalityStatus::Illegal,
                vec![LegalityDiagnostic {
                    kind,
                    obligation: None,
                    message: error.to_string(),
                }],
                BTreeSet::new(),
            );
        }

        let required = match required_obligations(requirements) {
            Ok(required) => required,
            Err(error) => {
                return LegalityAssessment::new(
                    LegalityStatus::Indeterminate,
                    vec![LegalityDiagnostic {
                        kind: LegalityDiagnosticKind::IncompleteValidation,
                        obligation: None,
                        message: error.to_string(),
                    }],
                    BTreeSet::new(),
                );
            }
        };

        let evidence = match self.provider.assess(
            graph,
            plan,
            requirements,
            &required,
        ) {
            Ok(evidence) => evidence,
            Err(reason) => {
                return LegalityAssessment::new(
                    LegalityStatus::Indeterminate,
                    vec![LegalityDiagnostic {
                        kind:
                            LegalityDiagnosticKind::EvidenceProviderFailure,
                        obligation: None,
                        message: reason,
                    }],
                    BTreeSet::new(),
                );
            }
        };

        evaluate_evidence(&required, evidence)
    }
}

// ============================================================================
// EVIDENCE EVALUATION
// ============================================================================

/// Evaluates provider evidence against the exact required obligation set.
///
/// Duplicate and unexpected evidence make the assessment indeterminate
/// unless a required obligation is disproven, in which case the result
/// is illegal. Every evidence item is inspected; duplicates cannot
/// overwrite a disproven result.
fn evaluate_evidence(
    required: &BTreeSet<ObligationId>,
    evidence: Vec<LegalityEvidence>,
) -> LegalityAssessment {
    let mut by_obligation =
        BTreeMap::<ObligationId, Vec<LegalityEvidence>>::new();

    let mut diagnostics = Vec::new();
    let mut references = BTreeSet::new();
    let mut malformed_evidence = false;

    for item in evidence {
        let obligation = item.obligation().clone();

        if !required.contains(&obligation) {
            malformed_evidence = true;

            diagnostics.push(LegalityDiagnostic {
                kind: LegalityDiagnosticKind::IncompleteValidation,
                obligation: Some(obligation.clone()),
                message: format!(
                    "Evidence was supplied for an obligation not requested \
                     by the current preservation policy: {obligation}",
                ),
            });

            continue;
        }

        let entries = by_obligation.entry(obligation.clone()).or_default();

        if !entries.is_empty() {
            malformed_evidence = true;

            diagnostics.push(LegalityDiagnostic {
                kind: LegalityDiagnosticKind::IncompleteValidation,
                obligation: Some(obligation.clone()),
                message: format!(
                    "Duplicate evidence for preservation obligation: \
                     {obligation}",
                ),
            });
        }

        entries.push(item);
    }

    let mut illegal = false;
    let mut indeterminate = malformed_evidence;

    for obligation in required {
        match by_obligation.get(obligation) {
            None => {
                indeterminate = true;

                diagnostics.push(LegalityDiagnostic {
                    kind: LegalityDiagnosticKind::MissingEvidence,
                    obligation: Some(obligation.clone()),
                    message: format!(
                        "Required preservation evidence is missing: \
                         {obligation}",
                    ),
                });
            }

            Some(items) => {
                for item in items {
                    references.extend(item.references().iter().cloned());

                    match item.status() {
                        ProofStatus::Proven => {}

                        ProofStatus::Disproven => {
                            illegal = true;

                            diagnostics.push(LegalityDiagnostic {
                                kind:
                                    LegalityDiagnosticKind::PreservationViolation,
                                obligation: Some(obligation.clone()),
                                message: format!(
                                    "Preservation was disproven for \
                                     {obligation}: {}",
                                    item.explanation(),
                                ),
                            });
                        }

                        ProofStatus::Unknown => {
                            indeterminate = true;

                            diagnostics.push(LegalityDiagnostic {
                                kind:
                                    LegalityDiagnosticKind::UnknownEvidence,
                                obligation: Some(obligation.clone()),
                                message: format!(
                                    "Preservation remains unverified for \
                                     {obligation}: {}",
                                    item.explanation(),
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    diagnostics.sort_by(|left, right| {
        left.obligation
            .cmp(&right.obligation)
            .then(left.kind.cmp(&right.kind))
            .then(left.message.cmp(&right.message))
    });

    let status = if illegal {
        LegalityStatus::Illegal
    } else if indeterminate {
        LegalityStatus::Indeterminate
    } else {
        LegalityStatus::Legal
    };

    LegalityAssessment::new(status, diagnostics, references)
}

// ============================================================================
// EXISTING TRANSFORMATION INTEGRATION
// ============================================================================

/// Implements the existing canonical transformation legality interface.
///
/// The existing TransformationEngine can consume this implementation
/// without creating another transformation engine or graph abstraction.
impl<G, P> TransformationLegality<G> for LegalityValidator<P>
where
    G: TransformationGraph,
    P: LegalityEvidenceProvider<G>,
{
    fn validate(
        &self,
        graph: &G,
        plan: &TransformationPlan<G::Node, G::Edge>,
        requirements: &PreservationRequirements,
    ) -> LegalityResult {
        self.assess(graph, plan, requirements)
            .into_transformation_result()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::optimization::transformations::{
        TransformationId, TransformationPhase, TransformationPlanBuilder,
        TransformationVersion,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Node(&'static str);

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Edge {
        source: Node,
        destination: Node,
    }

    struct TestGraph {
        revision: GraphRevision,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    }

    impl TransformationGraph for TestGraph {
        type Node = Node;
        type Edge = Edge;

        fn revision(&self) -> GraphRevision {
            self.revision
        }

        fn nodes(&self) -> Vec<Node> {
            self.nodes.clone()
        }

        fn edges(&self) -> Vec<Edge> {
            self.edges.clone()
        }

        fn source(&self, edge: &Edge) -> Node {
            edge.source.clone()
        }

        fn destination(&self, edge: &Edge) -> Node {
            edge.destination.clone()
        }

        fn is_transparent(&self, _: &Node) -> bool {
            false
        }

        fn compose_edges(
            &self,
            _: &Edge,
            _: &Edge,
            _: &Node,
        ) -> Option<Edge> {
            None
        }
    }

    struct TestEvidenceProvider {
        status: ProofStatus,
    }

    impl LegalityEvidenceProvider<TestGraph> for TestEvidenceProvider {
        fn assess(
            &self,
            _: &TestGraph,
            _: &TransformationPlan<Node, Edge>,
            _: &PreservationRequirements,
            required: &BTreeSet<ObligationId>,
        ) -> Result<Vec<LegalityEvidence>, String> {
            let mut result = Vec::new();

            for obligation in required {
                result.push(
                    LegalityEvidence::new(
                        obligation.clone(),
                        self.status,
                        BTreeSet::from([format!(
                            "test:evidence:{}",
                            obligation.as_str(),
                        )]),
                        "Test verification evidence",
                    )
                    .map_err(|error| error.to_string())?,
                );
            }

            Ok(result)
        }
    }

    fn graph() -> TestGraph {
        TestGraph {
            revision: GraphRevision::new(1),
            nodes: vec![Node("a"), Node("b"), Node("c")],
            edges: vec![
                Edge {
                    source: Node("a"),
                    destination: Node("b"),
                },
                Edge {
                    source: Node("b"),
                    destination: Node("c"),
                },
            ],
        }
    }

    fn plan(graph: &TestGraph) -> TransformationPlan<Node, Edge> {
        let mut builder = TransformationPlanBuilder::new(
            TransformationId::new("zamani.test.rewrite")
                .expect("valid identity"),
            TransformationVersion::new(1, 0, 0),
            graph.revision,
            TransformationPhase::GraphSimplification,
        )
        .with_description("Test edge removal");

        builder
            .push(GraphEdit::RemoveEdge(graph.edges[0].clone()))
            .expect("valid edit");

        builder.affect(Node("a")).expect("affected node");
        builder.affect(Node("b")).expect("affected node");

        builder.build().expect("valid plan")
    }

    fn evidence(
        obligation: &ObligationId,
        status: ProofStatus,
        reference: &str,
    ) -> LegalityEvidence {
        LegalityEvidence::new(
            obligation.clone(),
            status,
            BTreeSet::from([reference.to_string()]),
            "Regression test evidence",
        )
        .expect("valid evidence")
    }

    #[test]
    fn proven_evidence_certifies_legality() {
        let graph = graph();
        let plan = plan(&graph);

        let validator = LegalityValidator::new(TestEvidenceProvider {
            status: ProofStatus::Proven,
        });

        let result = validator.assess(
            &graph,
            &plan,
            &PreservationRequirements::default(),
        );

        assert_eq!(result.status(), LegalityStatus::Legal);
    }

    #[test]
    fn unknown_evidence_is_never_legal() {
        let graph = graph();
        let plan = plan(&graph);

        let validator = LegalityValidator::new(TestEvidenceProvider {
            status: ProofStatus::Unknown,
        });

        let result = validator.assess(
            &graph,
            &plan,
            &PreservationRequirements::default(),
        );

        assert!(result.is_indeterminate());
        assert!(!result.is_legal());
    }

    #[test]
    fn disproven_evidence_rejects_transformation() {
        let graph = graph();
        let plan = plan(&graph);

        let validator = LegalityValidator::new(TestEvidenceProvider {
            status: ProofStatus::Disproven,
        });

        let result = validator.assess(
            &graph,
            &plan,
            &PreservationRequirements::default(),
        );

        assert!(result.is_illegal());
    }

    #[test]
    fn structural_validation_rejects_stale_plans() {
        let graph = graph();
        let plan = plan(&graph);

        let mut changed = graph;
        changed.revision = GraphRevision::new(2);

        assert_eq!(
            validate_structure(&changed, &plan),
            Err(LegalityError::StalePlan),
        );
    }

    #[test]
    fn structural_validation_rejects_dangling_edges() {
        let graph = graph();

        let mut builder = TransformationPlanBuilder::new(
            TransformationId::new("zamani.test.remove_node")
                .expect("valid identity"),
            TransformationVersion::new(1, 0, 0),
            graph.revision,
            TransformationPhase::GraphSimplification,
        )
        .with_description("Invalid node removal");

        builder
            .push(GraphEdit::RemoveNode(Node("b")))
            .expect("valid edit");
        builder.affect(Node("b")).expect("affected node");

        let plan = builder.build().expect("valid plan");

        assert_eq!(
            validate_structure(&graph, &plan),
            Err(LegalityError::DanglingEdge),
        );
    }

    #[test]
    fn missing_evidence_is_indeterminate() {
        let required = required_obligations(
            &PreservationRequirements::default(),
        )
        .expect("required obligations");

        let assessment = evaluate_evidence(&required, Vec::new());

        assert!(assessment.is_indeterminate());
        assert!(!assessment.is_legal());
    }

    #[test]
    fn mandatory_semantic_obligations_cannot_be_disabled() {
        let requirements = PreservationRequirements {
            observable_behavior: false,
            numerical_semantics: false,
            effect_ordering: false,
            state_dependencies: false,
            provenance: false,
            security: false,
            determinism: false,
            resource_requirements: false,
        };

        let required =
            required_obligations(&requirements).expect("required obligations");

        assert!(required.contains(
            &ObligationId::new(obligations::OBSERVABLE_BEHAVIOR)
                .expect("valid obligation"),
        ));

        assert!(required.contains(
            &ObligationId::new(obligations::TYPES_AND_SHAPES)
                .expect("valid obligation"),
        ));
    }

    #[test]
    fn proven_evidence_requires_a_reference() {
        let result = LegalityEvidence::new(
            ObligationId::new(obligations::OBSERVABLE_BEHAVIOR)
                .expect("valid obligation"),
            ProofStatus::Proven,
            BTreeSet::new(),
            "No evidence",
        );

        assert_eq!(
            result,
            Err(LegalityError::MissingEvidenceReference),
        );
    }

    #[test]
    fn structural_validation_rejects_duplicate_nodes() {
        let mut graph = graph();
        graph.nodes.push(Node("a"));

        let plan = plan(&graph);

        assert_eq!(
            validate_structure(&graph, &plan),
            Err(LegalityError::DuplicateNode),
        );
    }

    #[test]
    fn structural_validation_rejects_missing_edge_endpoints() {
        let mut graph = graph();

        graph.edges.push(Edge {
            source: Node("missing"),
            destination: Node("a"),
        });

        let plan = plan(&graph);

        assert_eq!(
            validate_structure(&graph, &plan),
            Err(LegalityError::MissingEndpoint),
        );
    }

    #[test]
    fn standard_obligation_identities_are_extensible() {
        let custom = ObligationId::new(
            "zamani.extension.custom_preservation",
        )
        .expect("valid extension obligation");

        assert_eq!(
            custom.as_str(),
            "zamani.extension.custom_preservation",
        );
    }

    #[test]
    fn unexpected_evidence_makes_assessment_indeterminate() {
        let required = required_obligations(
            &PreservationRequirements::default(),
        )
        .expect("required obligations");

        let mut evidence = required
            .iter()
            .map(|obligation| {
                evidence(
                    obligation,
                    ProofStatus::Proven,
                    "test:required",
                )
            })
            .collect::<Vec<_>>();

        let unexpected = ObligationId::new(
            "zamani.extension.unrequested_obligation",
        )
        .expect("valid obligation");

        evidence.push(evidence(
            &unexpected,
            ProofStatus::Proven,
            "test:unexpected",
        ));

        let assessment = evaluate_evidence(&required, evidence);

        assert!(assessment.is_indeterminate());
        assert!(!assessment.is_legal());
        assert!(assessment.diagnostics().iter().any(|diagnostic| {
            diagnostic.kind()
                == LegalityDiagnosticKind::IncompleteValidation
        }));
    }

    #[test]
    fn duplicate_proven_evidence_makes_assessment_indeterminate() {
        let required = required_obligations(
            &PreservationRequirements::default(),
        )
        .expect("required obligations");

        let obligation = required
            .iter()
            .next()
            .expect("at least one required obligation")
            .clone();

        let mut evidence = required
            .iter()
            .map(|item| {
                evidence(item, ProofStatus::Proven, "test:proven")
            })
            .collect::<Vec<_>>();

        evidence.push(evidence(
            &obligation,
            ProofStatus::Proven,
            "test:duplicate",
        ));

        let assessment = evaluate_evidence(&required, evidence);

        assert!(assessment.is_indeterminate());
        assert!(!assessment.is_legal());
    }

    #[test]
    fn duplicate_disproven_evidence_cannot_be_overwritten() {
        let required = required_obligations(
            &PreservationRequirements::default(),
        )
        .expect("required obligations");

        let obligation = required
            .iter()
            .next()
            .expect("at least one required obligation")
            .clone();

        let mut evidence = required
            .iter()
            .map(|item| {
                evidence(item, ProofStatus::Proven, "test:proven")
            })
            .collect::<Vec<_>>();

        // The later Proven item must not overwrite the Disproven item.
        evidence.push(evidence(
            &obligation,
            ProofStatus::Disproven,
            "test:disproven",
        ));
        evidence.push(evidence(
            &obligation,
            ProofStatus::Proven,
            "test:later-proven",
        ));

        let assessment = evaluate_evidence(&required, evidence);

        assert!(assessment.is_illegal());
        assert!(!assessment.is_legal());
        assert!(assessment.diagnostics().iter().any(|diagnostic| {
            diagnostic.kind()
                == LegalityDiagnosticKind::PreservationViolation
        }));
    }
}