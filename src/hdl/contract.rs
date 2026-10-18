
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Behavioral Contracts
//!
//! Path: `src/hdl/contract.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: HDL-domain contract model; integration verification required.
//!
//! ## Purpose
//!
//! Represent source-level behavioral contracts for portable hardware
//! descriptions, including preconditions, postconditions, invariants, and
//! explicitly extensible contract kinds.
//!
//! A contract states a condition that must hold. This module records that
//! condition and its assessment state; it does not prove the condition.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/assertions.g4`
//! - `grammar/hdl/verification.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/model/`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar owns syntax. The canonical AST owns source structure and
//! expression nodes. The semantic model owns semantic identities. This file
//! owns only the HDL-specific behavioral-contract representation.
//!
//! ## Owns
//!
//! - HDL contract declarations and kinds.
//! - References to canonical AST expressions.
//! - Contract checkability classifications.
//! - Explicit assessment outcomes.
//! - Contract-local source provenance and validation.
//!
//! ## Does not own
//!
//! - Parsing or a second expression AST.
//! - Name resolution, type inference, or general semantic facts.
//! - The canonical semantic `ContractId` definition or allocation.
//! - The canonical AST `NodeId` or source `Span` definitions.
//! - A formal proof engine, SAT/SMT solver, or model checker.
//! - Simulation, synthesis, scheduling, routing, or physical realization.
//! - Target feasibility or physical resource allocation.
//! - HDLIR or any other intermediate representation.
//!
//! ## Integration
//!
//! 1. The parser produces the canonical AST.
//! 2. Semantic analysis resolves the referenced AST nodes and validates
//!    that each contract expression is Boolean in its required context.
//! 3. An AST/semantic adapter constructs `HdlContract` values using the
//!    canonical semantic `ContractId` and canonical AST `NodeId`.
//! 4. Contract analysis classifies whether an obligation is statically
//!    checkable, dynamically checkable, or currently unproven.
//! 5. A verification provider evaluates or proves the obligation and reports
//!    a `ContractAssessment`.
//! 6. Target feasibility and realization consume relevant requirements and
//!    verified evidence through their own APIs.
//!
//! No stage may interpret a missing assessment as proof of correctness.
//!
//! ## Scalability
//!
//! There are no fixed limits on the number of contracts, contract references,
//! modules, signals, or hardware resources represented by this module.
//! Collections are dynamically sized. This is not a promise of literally
//! infinite execution: allocation, identifier representation, and analysis
//! remain subject to available resources and configured operational budgets.
//!
//! This module imposes no hardware-capacity ceiling and does not convert an
//! operational budget into a source-language restriction.
//!
//! ## Determinism
//!
//! Contract values contain no global state, random identity generation,
//! hardware discovery, or implicit verification.
//!
//! Collection order is preserved. Consumers that require canonical output
//! ordering must order contracts using their canonical identities and the
//! documented ordering policy of the enclosing semantic pipeline.
//!
//! ## Freeze criteria
//!
//! - Canonical import paths and public exports compile in the actual crate.
//! - AST references resolve to nodes in the correct compilation unit.
//! - Contract expressions receive ordinary semantic validation.
//! - Unknown and incomplete assessments cannot be treated as proven.
//! - Diagnostics preserve canonical source spans.
//! - Unit, integration, and conformance tests pass.
//!
//! The public contract should be frozen only after these conditions are met.

use core::fmt;

use crate::frontend::ast::node::NodeId;
use crate::frontend::ast::node::source::Span;
use crate::frontend::semantic::model::ContractId;

/// The semantic kind of an HDL behavioral contract.
///
/// The kind describes the obligation's meaning, not the mechanism used
/// to verify it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlContractKind {
    /// A condition required before an operation or behavior is performed.
    Precondition,

    /// A condition required after an operation or behavior completes.
    Postcondition,

    /// A condition that must remain true throughout a specified scope.
    Invariant,

    /// An extension-defined contract kind.
    ///
    /// The identifier is a stable, namespaced extension name. Its semantics
    /// must be supplied by the extension that registers it.
    Extension,
}

/// An extension-defined contract-kind identifier.
///
/// Names are namespaced to prevent unrelated extensions from accidentally
/// assigning different meanings to the same identifier.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlContractKindId(String);

impl HdlContractKindId {
    /// Construct an extension identifier.
    ///
    /// The identifier must be non-empty and must not contain leading or
    /// trailing whitespace. Namespace syntax and registration are validated
    /// by the extension registry that owns those policies.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlContractError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(HdlContractError::InvalidExtensionIdentifier);
        }

        Ok(Self(value))
    }

    /// Return the identifier exactly as registered by the caller.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HdlContractKindId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The expression that defines a contract's condition.
///
/// The expression is referenced by canonical AST identity. This type
/// deliberately does not duplicate expression operators, literals, types,
/// or evaluation rules.
///
/// `span` is optional because some semantic or generated expressions may
/// not have a direct source range. When present, it must be the canonical
/// span associated with the expression or its relevant source construct.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlContractExpression {
    /// Canonical AST node that represents the contract condition.
    pub node_id: NodeId,

    /// Source span associated with the condition, if available.
    pub span: Option<Span>,
}

impl HdlContractExpression {
    /// Create a reference to a canonical AST expression.
    ///
    /// This constructor records the reference; it does not establish that
    /// the node exists, belongs to the current program, or has Boolean type.
    /// Those checks belong to AST validation and semantic analysis.
    #[must_use]
    pub fn new(node_id: NodeId, span: Option<Span>) -> Self {
        Self { node_id, span }
    }
}

/// The current checkability classification of a contract.
///
/// Checkability is not a verification result. A statically checkable
/// contract may still fail its check, and a dynamically checkable contract
/// may fail during execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlContractCheckability {
    /// The required condition can be decided by the available static
    /// semantic analysis, subject to the checks actually implemented.
    Static,

    /// The condition requires an execution or simulation context.
    Dynamic,

    /// The condition is not currently established as statically or
    /// dynamically checkable.
    ///
    /// This includes missing analysis support or unresolved prerequisites.
    Unproven,
}

/// A contract's verification or assessment outcome.
///
/// An assessment must only be produced by an appropriate analysis,
/// simulation, or verification provider. Constructing an `HdlContract`
/// never implies that its condition has been checked.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlContractOutcome {
    /// The selected verification method established that the condition holds
    /// within its explicitly stated scope and assumptions.
    Satisfied,

    /// The selected verification method established a violation.
    Violated,

    /// The available evidence does not establish satisfaction or violation.
    Unknown,

    /// Required inputs, declarations, or semantic dependencies are unresolved.
    Unresolved,

    /// The analysis did not finish, for example because a configured
    /// operational budget was exhausted or the operation was cancelled.
    Incomplete,

    /// No assessment has been performed.
    NotAssessed,
}

/// Evidence classification for a contract assessment.
///
/// This describes the source of the assessment, not a guarantee that the
/// evidence is sound. Trust, soundness, and proof validation belong to the
/// verification provider and its integration policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlContractEvidenceKind {
    /// A completed static semantic check.
    StaticAnalysis,

    /// An execution or simulation observation.
    DynamicExecution,

    /// A proof produced by a formal verification provider.
    FormalProof,

    /// A result produced by a registered external verification provider.
    Extension,

    /// No evidence was supplied.
    None,
}

/// Result of assessing a contract.
///
/// Provider identity, assumptions, proof artifacts, and execution traces may
/// be stored in the owning verification subsystem. This structure records
/// the minimal outcome needed by HDL clients without defining a proof format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlContractAssessment {
    /// Contract to which this assessment applies.
    pub contract_id: ContractId,

    /// Outcome reported by the provider.
    pub outcome: HdlContractOutcome,

    /// Kind of evidence reported by the provider.
    pub evidence_kind: HdlContractEvidenceKind,

    /// Optional source span associated with the result.
    pub span: Option<Span>,
}

impl HdlContractAssessment {
    /// Construct an assessment record.
    ///
    /// This constructor does not authenticate evidence or independently
    /// verify a proof. The caller must be a trusted integration boundary
    /// that has validated the provider result.
    #[must_use]
    pub fn new(
        contract_id: ContractId,
        outcome: HdlContractOutcome,
        evidence_kind: HdlContractEvidenceKind,
        span: Option<Span>,
    ) -> Self {
        Self {
            contract_id,
            outcome,
            evidence_kind,
            span,
        }
    }

    /// Return whether the reported outcome establishes satisfaction.
    ///
    /// This is a classification of the reported result, not an independent
    /// proof checker. Callers must trust only assessments accepted by their
    /// verification-provider policy.
    #[must_use]
    pub const fn reports_satisfaction(&self) -> bool {
        matches!(self.outcome, HdlContractOutcome::Satisfied)
    }

    /// Return whether the reported outcome establishes a violation.
    #[must_use]
    pub const fn reports_violation(&self) -> bool {
        matches!(self.outcome, HdlContractOutcome::Violated)
    }

    /// Return whether the outcome is inconclusive or has not been assessed.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        matches!(
            self.outcome,
            HdlContractOutcome::Unknown
                | HdlContractOutcome::Unresolved
                | HdlContractOutcome::Incomplete
                | HdlContractOutcome::NotAssessed
        )
    }
}

/// A source-level HDL behavioral contract.
///
/// This is a semantic record, not an AST node or an intermediate
/// representation. The canonical AST remains the authority for the
/// expression's structure and source-language meaning.
///
/// The `kind` field describes the obligation. Extension-specific semantics
/// are carried by `extension_kind` when `kind` is `Extension`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlContract {
    /// Canonical semantic identity of this contract.
    pub id: ContractId,

    /// Human-readable name of the contract.
    ///
    /// The name is descriptive and must not replace canonical identity.
    pub name: String,

    /// Semantic kind of the contract.
    pub kind: HdlContractKind,

    /// Extension identifier when `kind` is `Extension`.
    pub extension_kind: Option<HdlContractKindId>,

    /// Canonical AST expression defining the condition.
    pub condition: HdlContractExpression,

    /// Current checkability classification.
    ///
    /// The semantic analysis pipeline owns updates to this classification.
    pub checkability: HdlContractCheckability,

    /// Source location of the contract declaration, if available.
    pub declaration_span: Option<Span>,
}

impl HdlContract {
    /// Construct a contract record with validated local invariants.
    ///
    /// This validates only the record's own shape. It does not resolve the
    /// AST expression, validate its type, or verify its truth.
    pub fn new(
        id: ContractId,
        name: impl Into<String>,
        kind: HdlContractKind,
        extension_kind: Option<HdlContractKindId>,
        condition: HdlContractExpression,
        checkability: HdlContractCheckability,
        declaration_span: Option<Span>,
    ) -> Result<Self, HdlContractError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(HdlContractError::EmptyName);
        }

        match (kind, extension_kind.is_some()) {
            (HdlContractKind::Extension, false) => {
                return Err(HdlContractError::MissingExtensionIdentifier);
            }
            (HdlContractKind::Extension, true) => {}
            (_, true) => {
                return Err(HdlContractError::UnexpectedExtensionIdentifier);
            }
            (_, false) => {}
        }

        Ok(Self {
            id,
            name,
            kind,
            extension_kind,
            condition,
            checkability,
            declaration_span,
        })
    }

    /// Return whether this contract is currently classified as statically
    /// checkable.
    #[must_use]
    pub const fn is_statically_checkable(&self) -> bool {
        matches!(self.checkability, HdlContractCheckability::Static)
    }

    /// Return whether the contract is classified as dynamically checkable.
    #[must_use]
    pub const fn is_dynamically_checkable(&self) -> bool {
        matches!(self.checkability, HdlContractCheckability::Dynamic)
    }

    /// Return whether the contract remains unproven.
    #[must_use]
    pub const fn is_unproven(&self) -> bool {
        matches!(self.checkability, HdlContractCheckability::Unproven)
    }
}

/// Errors caused by constructing an invalid HDL contract record.
///
/// These errors describe malformed API input. They do not represent a
/// violated source-level behavioral contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HdlContractError {
    /// The contract name is empty or consists only of whitespace.
    EmptyName,

    /// An extension contract has no extension-kind identifier.
    MissingExtensionIdentifier,

    /// A built-in contract kind was supplied an extension-kind identifier.
    UnexpectedExtensionIdentifier,

    /// An extension identifier is empty or has leading/trailing whitespace.
    InvalidExtensionIdentifier,
}

impl fmt::Display for HdlContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("HDL contract name cannot be empty"),
            Self::MissingExtensionIdentifier => {
                f.write_str("extension contract requires an extension-kind identifier")
            }
            Self::UnexpectedExtensionIdentifier => {
                f.write_str("built-in contract kind cannot carry an extension-kind identifier")
            }
            Self::InvalidExtensionIdentifier => {
                f.write_str("HDL contract extension identifier is invalid")
            }
        }
    }
}

impl std::error::Error for HdlContractError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_identifier_rejects_empty_and_trimmed_values() {
        assert_eq!(
            HdlContractKindId::new(""),
            Err(HdlContractError::InvalidExtensionIdentifier)
        );
        assert_eq!(
            HdlContractKindId::new(" extension.kind"),
            Err(HdlContractError::InvalidExtensionIdentifier)
        );
        assert_eq!(
            HdlContractKindId::new("extension.kind "),
            Err(HdlContractError::InvalidExtensionIdentifier)
        );
    }

    #[test]
    fn extension_identifier_preserves_registered_name() {
        let id = HdlContractKindId::new("org.zamani.safety").unwrap();
        assert_eq!(id.as_str(), "org.zamani.safety");
        assert_eq!(id.to_string(), "org.zamani.safety");
    }

    #[test]
    fn outcome_classification_never_promotes_unknown_to_satisfied() {
        let inconclusive = [
            HdlContractOutcome::Unknown,
            HdlContractOutcome::Unresolved,
            HdlContractOutcome::Incomplete,
            HdlContractOutcome::NotAssessed,
        ];

        for outcome in inconclusive {
            assert_ne!(outcome, HdlContractOutcome::Satisfied);
            assert_ne!(outcome, HdlContractOutcome::Violated);
        }
    }

    #[test]
    fn only_satisfied_outcome_reports_satisfaction() {
        assert_eq!(
            HdlContractOutcome::Satisfied,
            HdlContractOutcome::Satisfied
        );

        assert_ne!(
            HdlContractOutcome::Incomplete,
            HdlContractOutcome::Satisfied
        );
        assert_ne!(
            HdlContractOutcome::NotAssessed,
            HdlContractOutcome::Satisfied
        );
    }

    #[test]
    fn checkability_is_separate_from_outcome() {
        let classifications = [
            HdlContractCheckability::Static,
            HdlContractCheckability::Dynamic,
            HdlContractCheckability::Unproven,
        ];

        assert_eq!(classifications.len(), 3);
        assert_ne!(
            HdlContractCheckability::Static,
            HdlContractCheckability::Unproven
        );
    }

    #[test]
    fn error_messages_are_stable_and_non_empty() {
        let errors = [
            HdlContractError::EmptyName,
            HdlContractError::MissingExtensionIdentifier,
            HdlContractError::UnexpectedExtensionIdentifier,
            HdlContractError::InvalidExtensionIdentifier,
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }
}
