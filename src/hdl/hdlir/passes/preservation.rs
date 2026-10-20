
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Transformation Preservation
//!
//! Path: `src/hdl/hdlir/passes/preservation.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define a deterministic, extensible boundary for checking the preservation
//! obligations declared by an HDLIR transformation pass.
//!
//! This module separates:
//!
//! - The preservation guarantees declared by `PassContract`.
//! - The evidence produced by an appropriate verification provider.
//! - The final determination of whether the available evidence is sufficient.
//!
//! It does not implement hardware equivalence checking, interpret arbitrary
//! operation semantics, or assume that structural validity proves behavioral
//! equivalence. Those tasks belong to implementations that understand the
//! applicable IR, dialect, and semantic contracts.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars.
//! - `grammar/hdl/sequential.g4`.
//! - `grammar/hdl/processes.g4`.
//! - `grammar/hdl/clocks.g4`.
//! - `grammar/hdl/timing.g4`.
//! - `grammar/specification/semantic-model.md`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/config.rs`.
//! - `src/hdl/hdlir/fingerprint.rs`.
//! - `src/hdl/hdlir/passes/contracts.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! The grammar and frontend own source-language meaning. ZUIR owns the
//! universal computational representation. HDLIR owns hardware-domain
//! representation and its transformation contracts. Target realization owns
//! physical implementation and timing closure.
//!
//! ## Ownership
//!
//! This module owns preservation evidence, reports, outcomes, and the
//! orchestration boundary that checks declared guarantees.
//!
//! It does not own:
//!
//! - The HDLIR graph or its canonical IDs.
//! - Pass registration, scheduling, or execution.
//! - Operation, type, clock, reset, process, or memory semantics.
//! - Structural verification or individual analysis implementations.
//! - Formal equivalence algorithms or proof engines.
//! - Source parsing, semantic analysis, or ZUIR.
//! - Physical allocation, placement, routing, synthesis, or timing closure.
//!
//! ## Soundness rule
//!
//! A provider must report `Preserved` only when its documented checking
//! method establishes the requested property for the supplied before/after
//! representations. A hash match, successful parsing, structural validity,
//! or an unverified pass declaration is not by itself proof of equivalence.
//!
//! `Inconclusive` is not success. A violated guarantee cannot be converted
//! into success by another guarantee passing. Missing required guarantees
//! are rejected before evidence collection begins.
//!
//! ## Transaction rule
//!
//! The checker does not mutate either representation. A pass manager should
//! run a transformation on an isolated candidate or retain a rollback point,
//! then commit the candidate only after required verification and preservation
//! obligations have completed successfully.
//!
//! ## Scalability
//!
//! No universal hardware-size or guarantee-count limits are defined here.
//! Collections grow with the actual contract and available resources.
//! A caller processing untrusted inputs must apply explicit operational
//! budgets through the pass manager or evidence provider. Exhausted or
//! incomplete verification must never be reported as success.
//!
//! ## Integration
//!
//! - `passes/contracts.rs` declares preservation guarantees.
//! - `passes/manager.rs` should invoke this checker before committing a
//!   transformed candidate and propagate incomplete or violated outcomes.
//! - `verify/` provides structural and semantic verification mechanisms.
//! - `analysis/` supplies applicable analysis facts.
//! - `transform/` declares and implements transformations.
//! - `fingerprint.rs` may identify canonical content, but fingerprints are
//!   not equivalence proofs.
//! - `source_map.rs` owns provenance associations; providers must check the
//!   provenance guarantee according to its declared preservation contract.
//!
//! These integration points must be registered only after their actual
//! public APIs have been inspected and reconciled.
//!
//! ## Freeze criteria
//!
//! - Declared guarantees and requested guarantees cannot be silently omitted.
//! - Every returned evidence record identifies its guarantee and provider.
//! - Violated and inconclusive results cannot pass the success gate.
//! - Evidence ordering is deterministic.
//! - Tests cover missing declarations, violations, inconclusive checks,
//!   mismatched evidence, extension guarantees, and successful checks.
//! - The actual repository module tree compiles and all applicable checks pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::passes::contracts::{
    ContractKey, PassContract, PassIdentity, PreservationGuarantee,
};

/// Result of checking one preservation obligation.
///
/// `Preserved` means the provider's documented method established the
/// requested property. `Violated` means the provider found a counterexample
/// or otherwise established that the property does not hold. `Inconclusive`
/// means the provider could not establish either conclusion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PreservationStatus {
    /// The requested property was established by the provider.
    Preserved,

    /// The requested property was established to be violated.
    Violated,

    /// The check did not establish preservation or a definite violation.
    Inconclusive,
}

/// Overall result of evaluating a preservation report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PreservationOutcome {
    /// Every required guarantee has conclusive preserving evidence.
    Preserved,

    /// At least one required guarantee has conclusive violating evidence.
    Violated,

    /// No violation was established, but at least one required check is
    /// inconclusive or its evidence is otherwise insufficient.
    Inconclusive,
}

/// Evidence produced by one preservation-checking provider.
///
/// The provider identity identifies the implementation responsible for the
/// check. It does not, by itself, certify that the provider is trustworthy
/// or that its algorithm is sound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationEvidence {
    guarantee: PreservationGuarantee,
    status: PreservationStatus,
    provider: ContractKey,
    detail: String,
}

impl PreservationEvidence {
    /// Construct evidence for a specific guarantee.
    ///
    /// The checker verifies that the guarantee matches the requested
    /// obligation. The provider remains responsible for the validity of
    /// its stated result and checking method.
    #[must_use]
    pub fn new(
        guarantee: PreservationGuarantee,
        status: PreservationStatus,
        provider: ContractKey,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            guarantee,
            status,
            provider,
            detail: detail.into(),
        }
    }

    /// Return the guarantee evaluated by this evidence record.
    #[must_use]
    pub const fn guarantee(&self) -> &PreservationGuarantee {
        &self.guarantee
    }

    /// Return the result of the check.
    #[must_use]
    pub const fn status(&self) -> PreservationStatus {
        self.status
    }

    /// Return the qualified identity of the evidence provider.
    #[must_use]
    pub const fn provider(&self) -> &ContractKey {
        &self.provider
    }

    /// Return the provider's explanation of the result.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Result of evaluating one preservation guarantee.
///
/// A provider should return `Inconclusive` when a required analysis is
/// missing, a proof cannot be completed, unsupported semantics are
/// encountered, or an operational budget prevents a conclusive result.
/// Operational failures that prevent the provider from returning a result
/// may instead be returned as `HdlirError`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationEvaluation {
    status: PreservationStatus,
    detail: String,
}

impl PreservationEvaluation {
    /// Construct a preservation evaluation.
    #[must_use]
    pub fn new(
        status: PreservationStatus,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            status,
            detail: detail.into(),
        }
    }

    /// Return the evaluation status.
    #[must_use]
    pub const fn status(&self) -> PreservationStatus {
        self.status
    }

    /// Return the explanation for the evaluation.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Provider that checks preservation obligations for a particular IR type.
///
/// Implementations must understand the semantics relevant to each guarantee
/// they evaluate. They must not report preservation merely because a pass
/// declared it, because the output is structurally valid, or because
/// fingerprints happen to match.
///
/// The `contract` argument includes the pass's semantic guarantee. Providers
/// handling `SemanticGuarantee::RefinesUnderProof` must discharge the named
/// proof obligation before reporting success for properties that depend on
/// that refinement.
///
/// Implementations should apply explicit work and verification budgets and
/// return `Inconclusive` when those budgets prevent a conclusive result.
pub trait PreservationEvidenceProvider<I> {
    /// Evaluate one requested guarantee for the before/after representations.
    ///
    /// `before` and `after` must refer to the same transformation boundary.
    /// The provider must not mutate either representation.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if the provider cannot perform the
    /// requested check because of an operational or contract failure.
    /// A valid but inconclusive check should normally return an evaluation
    /// with `PreservationStatus::Inconclusive`.
    fn evaluate(
        &mut self,
        contract: &PassContract,
        guarantee: &PreservationGuarantee,
        before: &I,
        after: &I,
    ) -> HdlirResult<PreservationEvaluation>;

    /// Return the stable, qualified identity of this provider.
    fn provider_key(&self) -> &ContractKey;
}

/// Complete record of preservation checks for one pass application.
///
/// Evidence is stored in guarantee order. The report records which
/// guarantees were required, so callers can distinguish a fully checked
/// candidate from one that was checked against only a subset of obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationReport {
    pass: PassIdentity,
    required: BTreeSet<PreservationGuarantee>,
    evidence: BTreeMap<PreservationGuarantee, PreservationEvidence>,
}

impl PreservationReport {
    fn new(
        pass: PassIdentity,
        required: BTreeSet<PreservationGuarantee>,
        evidence: BTreeMap<PreservationGuarantee, PreservationEvidence>,
    ) -> Self {
        Self {
            pass,
            required,
            evidence,
        }
    }

    /// Return the identity of the pass whose candidate was checked.
    #[must_use]
    pub const fn pass(&self) -> &PassIdentity {
        &self.pass
    }

    /// Return all required preservation guarantees in deterministic order.
    #[must_use]
    pub fn required_guarantees(
        &self,
    ) -> &BTreeSet<PreservationGuarantee> {
        &self.required
    }

    /// Return all collected evidence in deterministic guarantee order.
    #[must_use]
    pub fn evidence(
        &self,
    ) -> &BTreeMap<PreservationGuarantee, PreservationEvidence> {
        &self.evidence
    }

    /// Return evidence for a particular guarantee, if present.
    #[must_use]
    pub fn evidence_for(
        &self,
        guarantee: &PreservationGuarantee,
    ) -> Option<&PreservationEvidence> {
        self.evidence.get(guarantee)
    }

    /// Return whether every required guarantee has evidence.
    ///
    /// Completeness alone does not mean preservation succeeded. A complete
    /// report may contain a conclusive violation.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.required
            .iter()
            .all(|guarantee| self.evidence.contains_key(guarantee))
    }

    /// Determine the aggregate result of the collected evidence.
    ///
    /// A definite violation takes precedence over inconclusive checks.
    /// Otherwise, any missing or inconclusive evidence makes the report
    /// inconclusive. Only a complete report with every required guarantee
    /// preserved returns `Preserved`.
    #[must_use]
    pub fn outcome(&self) -> PreservationOutcome {
        if self.evidence.values().any(|evidence| {
            evidence.status() == PreservationStatus::Violated
        }) {
            return PreservationOutcome::Violated;
        }

        let all_preserved = self.is_complete()
            && self.required.iter().all(|guarantee| {
                self.evidence
                    .get(guarantee)
                    .is_some_and(|evidence| {
                        evidence.status() == PreservationStatus::Preserved
                    })
            });

        if all_preserved {
            PreservationOutcome::Preserved
        } else {
            PreservationOutcome::Inconclusive
        }
    }

    /// Require conclusive evidence that every required guarantee is preserved.
    ///
    /// A caller should use this gate before committing a transformed
    /// candidate to the authoritative IR.
    ///
    /// # Errors
    ///
    /// Returns an invariant error if any guarantee was violated. Returns an
    /// incomplete-processing error if any guarantee is missing or
    /// inconclusive. Neither condition may be treated as success.
    pub fn require_preserved(&self) -> HdlirResult<()> {
        match self.outcome() {
            PreservationOutcome::Preserved => Ok(()),

            PreservationOutcome::Violated => {
                let violations = self
                    .evidence
                    .values()
                    .filter(|evidence| {
                        evidence.status() == PreservationStatus::Violated
                    })
                    .map(|evidence| {
                        format!(
                            "{:?} by {}: {}",
                            evidence.guarantee(),
                            evidence.provider(),
                            evidence.detail()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ");

                Err(HdlirError::invalid_invariant(
                    "pass_preservation",
                    format!(
                        "pass `{}` violated preservation obligations: {violations}",
                        self.pass.key()
                    ),
                ))
            }

            PreservationOutcome::Inconclusive => {
                let unresolved = self
                    .required
                    .iter()
                    .filter_map(|guarantee| match self.evidence.get(guarantee) {
                        None => Some(format!("{guarantee:?}: no evidence")),
                        Some(evidence)
                            if evidence.status()
                                == PreservationStatus::Inconclusive =>
                        {
                            Some(format!(
                                "{guarantee:?}: {}",
                                evidence.detail()
                            ))
                        }
                        Some(_) => None,
                    })
                    .collect::<Vec<_>>()
                    .join("; ");

                Err(HdlirError::incomplete(
                    "pass_preservation",
                    format!(
                        "preservation for pass `{}` is incomplete: {unresolved}",
                        self.pass.key()
                    ),
                ))
            }
        }
    }
}

/// Stateless orchestration for preservation evidence collection.
///
/// The checker does not claim that its provider is a formal proof engine.
/// Its result is only as strong as the checking methods implemented by
/// the provider for the requested guarantees.
#[derive(Clone, Copy, Debug, Default)]
pub struct PreservationChecker;

impl PreservationChecker {
    /// Check every guarantee declared by the pass and every additional
    /// guarantee required by the calling pipeline.
    ///
    /// Additional requirements allow a pipeline or validation policy to
    /// demand properties beyond the pass's own declaration. The union is
    /// evaluated exactly once per distinct guarantee.
    ///
    /// # Errors
    ///
    /// Returns an error if the contract is internally inconsistent, if no
    /// preservation guarantees are declared or required, if the provider
    /// returns an error, or if an evaluation cannot be associated with the
    /// requested guarantee.
    ///
    /// A returned report may be inconclusive or may contain violations.
    /// Call [`PreservationReport::require_preserved`] before trusting the
    /// candidate.
    pub fn check<I, P, G>(
        &self,
        contract: &PassContract,
        before: &I,
        after: &I,
        additional_required: G,
        provider: &mut P,
    ) -> HdlirResult<PreservationReport>
    where
        P: PreservationEvidenceProvider<I>,
        G: IntoIterator<Item = PreservationGuarantee>,
    {
        contract.validate_declaration()?;

        let mut required = contract.preservation().clone();
        required.extend(additional_required);

        if required.is_empty() {
            return Err(HdlirError::invalid_invariant(
                "preservation_contract",
                format!(
                    "pass `{}` has no declared or pipeline-required preservation guarantees",
                    contract.identity().key()
                ),
            ));
        }

        let mut evidence = BTreeMap::new();

        for guarantee in &required {
            let evaluation =
                provider.evaluate(contract, guarantee, before, after)?;

            let record = PreservationEvidence::new(
                guarantee.clone(),
                evaluation.status(),
                provider.provider_key().clone(),
                evaluation.detail().to_owned(),
            );

            // The key is drawn from `required`, so duplicate evidence cannot
            // be inserted by a provider or silently replace earlier evidence.
            if evidence.insert(guarantee.clone(), record).is_some() {
                return Err(HdlirError::invalid_invariant(
                    "preservation_evidence_uniqueness",
                    format!("duplicate evidence for {guarantee:?}"),
                ));
            }
        }

        Ok(PreservationReport::new(
            contract.identity().clone(),
            required,
            evidence,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::passes::contracts::{
        PassContract, PassIdentity, PassVersion,
    };

    struct TestProvider {
        key: ContractKey,
        status: PreservationStatus,
    }

    impl PreservationEvidenceProvider<u32> for TestProvider {
        fn evaluate(
            &mut self,
            _contract: &PassContract,
            _guarantee: &PreservationGuarantee,
            before: &u32,
            after: &u32,
        ) -> HdlirResult<PreservationEvaluation> {
            if before == after && self.status == PreservationStatus::Preserved {
                Ok(PreservationEvaluation::new(
                    PreservationStatus::Preserved,
                    "test representations match",
                ))
            } else {
                Ok(PreservationEvaluation::new(
                    self.status,
                    "test provider result",
                ))
            }
        }

        fn provider_key(&self) -> &ContractKey {
            &self.key
        }
    }

    fn key(value: &str) -> ContractKey {
        ContractKey::new(value).expect("test key should be valid")
    }

    fn contract() -> PassContract {
        PassContract::builder(PassIdentity::new(
            key("org.zamani.test_pass"),
            PassVersion::new(1, 0, 0),
        ))
        .preserves(PreservationGuarantee::CombinationalBehavior)
        .preserves(PreservationGuarantee::StateTransitions)
        .build()
        .expect("test contract should be valid")
    }

    fn provider(status: PreservationStatus) -> TestProvider {
        TestProvider {
            key: key("org.zamani.test_provider"),
            status,
        }
    }

    #[test]
    fn all_declared_guarantees_must_be_preserved() {
        let contract = contract();
        let mut provider = provider(PreservationStatus::Preserved);

        let report = PreservationChecker
            .check(&contract, &7_u32, &7_u32, [], &mut provider)
            .expect("preservation checking should complete");

        assert_eq!(report.outcome(), PreservationOutcome::Preserved);
        assert!(report.is_complete());
        assert!(report.require_preserved().is_ok());
        assert_eq!(report.evidence().len(), 2);
    }

    #[test]
    fn a_violation_cannot_be_reported_as_success() {
        let contract = contract();
        let mut provider = provider(PreservationStatus::Violated);

        let report = PreservationChecker
            .check(&contract, &7_u32, &8_u32, [], &mut provider)
            .expect("the checker should return the evidence report");

        assert_eq!(report.outcome(), PreservationOutcome::Violated);
        assert!(report.require_preserved().is_err());
    }

    #[test]
    fn inconclusive_evidence_cannot_be_reported_as_success() {
        let contract = contract();
        let mut provider = provider(PreservationStatus::Inconclusive);

        let report = PreservationChecker
            .check(&contract, &7_u32, &8_u32, [], &mut provider)
            .expect("the checker should return an inconclusive report");

        assert_eq!(report.outcome(), PreservationOutcome::Inconclusive);
        assert!(report.is_complete());
        assert!(report.require_preserved().is_err());
    }

    #[test]
    fn additional_pipeline_requirements_are_checked() {
        let contract = contract();
        let mut provider = provider(PreservationStatus::Preserved);

        let report = PreservationChecker
            .check(
                &contract,
                &7_u32,
                &7_u32,
                [PreservationGuarantee::Provenance],
                &mut provider,
            )
            .expect("additional preservation should be checked");

        assert_eq!(report.required_guarantees().len(), 3);
        assert_eq!(report.evidence().len(), 3);
        assert!(report.require_preserved().is_ok());
    }

    #[test]
    fn a_contract_without_guarantees_is_rejected() {
        let contract = PassContract::builder(PassIdentity::new(
            key("org.zamani.empty_pass"),
            PassVersion::new(1, 0, 0),
        ))
        .build()
        .expect("empty declaration is structurally valid");

        let mut provider = provider(PreservationStatus::Preserved);
        let result =
            PreservationChecker.check(&contract, &1_u32, &1_u32, [], &mut provider);

        assert!(result.is_err());
    }
}
