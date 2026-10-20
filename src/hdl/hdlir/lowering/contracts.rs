
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Lowering Contracts
//!
//! Path: `src/hdl/hdlir/lowering/contracts.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define reusable, deterministic contracts for conversions between the
//! canonical semantic model, ZUIR, HDLIR, and explicitly supported external
//! representations.
//!
//! A declared mapping is not proof that a conversion preserves meaning.
//! A lossless result requires completed processing, satisfied preconditions,
//! preserved obligations, acceptable provenance, and no unsupported or
//! failed requirements.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/contracts/ir-mappings.yaml`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ir/`
//! - `src/hdl/hdlir/types/`
//! - `src/hdl/hdlir/values/`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/`
//!
//! Grammar owns syntax. The frontend semantic model owns validated source
//! meaning. ZUIR owns the universal computational representation. HDLIR owns
//! hardware-domain representation. Target realization owns physical
//! allocation, synthesis, placement, routing, and implementation.
//!
//! ## This file owns
//!
//! - Stable, extensible identifiers for mapping features and obligations.
//! - Mapping direction and fidelity classifications.
//! - Declarative mapping contracts and their validation.
//! - A deterministic mapping-contract registry.
//! - Evidence records and conservative assessment of conversion results.
//!
//! ## This file does not own
//!
//! - Source parsing, AST construction, semantic validation, or name resolution.
//! - ZUIR or HDLIR graph structures, IDs, types, or schema registration.
//! - Conversion algorithms or graph mutation.
//! - A proof engine or an equivalence checker.
//! - Source-span definitions or the canonical diagnostic system.
//! - Target discovery, physical allocation, synthesis, scheduling, routing,
//!   timing closure, or hardware execution.
//!
//! ## Safety, scalability, and determinism
//!
//! No universal hardware-capacity ceilings are defined here. Registry size
//! and obligation counts grow with available memory and the caller's
//! operational budget. This module does not claim literal infinite capacity.
//!
//! Identifiers are validated strings so future dialects and features can be
//! added without extending a closed enum. Ordered maps and sets make registry
//! iteration and assessment deterministic.
//!
//! This module does not allocate hardware resources or force symbolic widths
//! into concrete host-sized integers.
//!
//! ## Integration
//!
//! `lowering/mod.rs` should register this module and re-export only the
//! contract types intended as public API.
//!
//! `from_semantic.rs`, `from_zuir.rs`, and `to_zuir.rs` should declare the
//! mapping contracts they use and provide evidence from their actual checks.
//! They must not manufacture preservation evidence merely because conversion
//! returned `Ok`.
//!
//! The owning conversion adapter remains responsible for validating source
//! input, checking the destination graph, preserving provenance, and returning
//! a failure when a required mapping is unsupported or incomplete.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};

/// Stable identifier for a source or destination feature.
///
/// Identifiers are extensible, namespaced strings such as
/// `semantic.hdl.process`, `zuir.hdl.register`, or `hdlir.memory.port`.
/// This type identifies a feature; it does not prove that the feature is
/// implemented or registered.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FeatureId(String);

impl FeatureId {
    /// Construct a validated feature identifier.
    ///
    /// The accepted character set is ASCII letters, digits, `.`, `_`, `-`,
    /// `/`, and `:`. The identifier must start and end with an alphanumeric
    /// character. Namespaces are convention, not a closed registry.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        let valid = !value.is_empty()
            && value.as_bytes().first().is_some_and(u8::is_ascii_alphanumeric)
            && value.as_bytes().last().is_some_and(u8::is_ascii_alphanumeric)
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'.' | b'_' | b'-' | b'/' | b':')
            });

        if !valid {
            return Err(HdlirError::InvalidInput {
                field: "feature_id",
                reason: format!(
                    "feature identifier must be non-empty, use the supported \
                     ASCII identifier characters, and begin and end with \
                     an alphanumeric character: {value:?}"
                ),
            });
        }

        Ok(Self(value))
    }

    /// Return the exact identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable identifier for a mapping precondition or preservation obligation.
///
/// An obligation is a named requirement. It is not evidence that the
/// requirement has been checked or proved.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObligationId(String);

impl ObligationId {
    /// Construct a validated obligation identifier.
    ///
    /// Uses the same character and boundary rules as [`FeatureId`].
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        FeatureId::new(value.into()).map(|feature| Self(feature.0))
    }

    /// Return the exact identifier string.
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

/// Direction of a representation conversion.
///
/// The direction names express architectural boundaries, not a promise
/// that an adapter implementing that direction already exists.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MappingDirection {
    /// Convert validated frontend semantic meaning into HDLIR.
    SemanticToHdlir,
    /// Convert ZUIR operations or regions into HDLIR.
    ZuirToHdlir,
    /// Convert HDLIR constructs into ZUIR.
    HdlirToZuir,
    /// Import a supported external representation into HDLIR.
    ExternalToHdlir,
    /// Export HDLIR into a supported external representation.
    HdlirToExternal,
}

impl MappingDirection {
    /// Return the stable machine-readable direction identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticToHdlir => "semantic_to_hdlir",
            Self::ZuirToHdlir => "zuir_to_hdlir",
            Self::HdlirToZuir => "hdlir_to_zuir",
            Self::ExternalToHdlir => "external_to_hdlir",
            Self::HdlirToExternal => "hdlir_to_external",
        }
    }
}

impl fmt::Display for MappingDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Declared mapping fidelity.
///
/// `Exact` means the contract requires preservation of all declared
/// obligations. `ExtensionPreserving` allows a destination extension to
/// carry semantics not represented by the destination's base feature set,
/// but does not permit those semantics to be discarded.
///
/// `Unsupported` explicitly records a known mapping gap.
/// `LossyForbidden` records a conversion that would lose required meaning
/// and therefore must not be reported as successful.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MappingFidelity {
    /// The destination representation expresses the required meaning
    /// without requiring an opaque extension for that meaning.
    Exact,
    /// The destination can preserve additional meaning through an approved,
    /// versioned extension contract.
    ExtensionPreserving,
    /// The mapping is not implemented or cannot be established.
    Unsupported,
    /// The mapping is known to lose required meaning and must be rejected.
    LossyForbidden,
}

impl MappingFidelity {
    /// Return whether the declared mapping is eligible for a lossless result.
    #[must_use]
    pub const fn permits_lossless_result(self) -> bool {
        matches!(self, Self::Exact | Self::ExtensionPreserving)
    }

    /// Return the stable machine-readable fidelity identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ExtensionPreserving => "extension_preserving",
            Self::Unsupported => "unsupported",
            Self::LossyForbidden => "lossy_forbidden",
        }
    }
}

/// Contract for how source provenance must be handled by a mapping.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ProvenancePolicy {
    /// Source origins must be retained for every mapped entity to which
    /// provenance applies.
    MustPreserve,
    /// Original origins may be combined with explicit derived-origin records,
    /// but may not be silently dropped.
    PreserveOrDerive,
    /// The mapping's declared feature has no source-origin requirement.
    ///
    /// This option must be justified by the owning feature contract. It must
    /// not be selected merely to avoid implementing source maps.
    NotRequired,
}

/// Evidence describing how a required obligation was handled.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ObligationOutcome {
    /// The required check completed and passed.
    Satisfied,
    /// The required check completed and failed.
    Failed,
    /// The check did not complete or no adequate evidence was supplied.
    Unresolved,
}

/// Evidence about provenance handling for one conversion.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ProvenanceOutcome {
    /// Required source origins were preserved.
    Preserved,
    /// Origins were explicitly derived and the derivation was recorded.
    DerivedWithRecord,
    /// Required source origins could not be preserved or derived.
    Unavailable,
    /// The applicable contract explicitly requires no provenance.
    NotRequired,
}

/// Declarative contract for one feature mapping.
///
/// Contracts are immutable after construction. Adapters provide assessment
/// evidence separately; they must not mutate the contract to make a failed
/// conversion appear valid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingContract {
    source: FeatureId,
    destination: FeatureId,
    direction: MappingDirection,
    fidelity: MappingFidelity,
    required_preconditions: BTreeSet<ObligationId>,
    preservation_obligations: BTreeSet<ObligationId>,
    provenance_policy: ProvenancePolicy,
}

impl MappingContract {
    /// Create a mapping contract with explicit source, destination,
    /// direction, fidelity, and provenance policy.
    #[must_use]
    pub fn new(
        source: FeatureId,
        destination: FeatureId,
        direction: MappingDirection,
        fidelity: MappingFidelity,
        provenance_policy: ProvenancePolicy,
    ) -> Self {
        Self {
            source,
            destination,
            direction,
            fidelity,
            required_preconditions: BTreeSet::new(),
            preservation_obligations: BTreeSet::new(),
            provenance_policy,
        }
    }

    /// Add a precondition that must be checked before conversion is accepted.
    #[must_use]
    pub fn requiring(mut self, obligation: ObligationId) -> Self {
        self.required_preconditions.insert(obligation);
        self
    }

    /// Add a semantic property that must be preserved by the mapping.
    #[must_use]
    pub fn preserving(mut self, obligation: ObligationId) -> Self {
        self.preservation_obligations.insert(obligation);
        self
    }

    /// Return the source feature identifier.
    #[must_use]
    pub fn source(&self) -> &FeatureId {
        &self.source
    }

    /// Return the destination feature identifier.
    #[must_use]
    pub fn destination(&self) -> &FeatureId {
        &self.destination
    }

    /// Return the mapping direction.
    #[must_use]
    pub const fn direction(&self) -> MappingDirection {
        self.direction
    }

    /// Return the declared fidelity.
    #[must_use]
    pub const fn fidelity(&self) -> MappingFidelity {
        self.fidelity
    }

    /// Return the required preconditions in deterministic order.
    #[must_use]
    pub fn required_preconditions(&self) -> &BTreeSet<ObligationId> {
        &self.required_preconditions
    }

    /// Return the required preservation obligations in deterministic order.
    #[must_use]
    pub fn preservation_obligations(&self) -> &BTreeSet<ObligationId> {
        &self.preservation_obligations
    }

    /// Return the provenance policy.
    #[must_use]
    pub const fn provenance_policy(&self) -> ProvenancePolicy {
        self.provenance_policy
    }

    /// Return a stable key for registry lookup and diagnostics.
    ///
    /// The key is a display identifier, not a cryptographic fingerprint.
    #[must_use]
    pub fn key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.direction,
            self.source.as_str(),
            self.destination.as_str()
        )
    }

    /// Validate the internal consistency of the contract.
    ///
    /// An unsupported or lossy-forbidden contract must not advertise a
    /// lossless result. The fidelity itself is sufficient to prevent that
    /// claim; this method also rejects self-mappings that have not explicitly
    /// been represented as a distinct feature/version boundary.
    pub fn validate(&self) -> HdlirResult<()> {
        if self.source == self.destination {
            return Err(HdlirError::InvalidInvariant {
                invariant: "lowering.contract.distinct_boundaries",
                detail: format!(
                    "source and destination are identical for mapping {}",
                    self.key()
                ),
            });
        }

        if !self.fidelity.permits_lossless_result()
            && self.preservation_obligations.is_empty()
            && self.required_preconditions.is_empty()
            && self.fidelity != MappingFidelity::Unsupported
        {
            return Err(HdlirError::InvalidInvariant {
                invariant: "lowering.contract.lossy_mapping_declared",
                detail: format!(
                    "mapping {} declares forbidden loss without documenting \
                     any affected precondition or preservation obligation",
                    self.key()
                ),
            });
        }

        Ok(())
    }
}

/// Deterministic registry of explicitly declared mapping contracts.
///
/// Registration rejects duplicate keys. It never replaces an existing
/// contract silently. The registry is an in-memory declaration catalogue;
/// it does not establish that an adapter implements a registered mapping.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MappingCatalog {
    contracts: BTreeMap<String, MappingContract>,
}

impl MappingCatalog {
    /// Create an empty mapping catalogue.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a contract after validating its invariants.
    ///
    /// Duplicate registration returns an error and leaves the existing
    /// contract unchanged.
    pub fn register(&mut self, contract: MappingContract) -> HdlirResult<()> {
        contract.validate()?;
        let key = contract.key();

        if self.contracts.contains_key(&key) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "lowering_mapping_contract",
                identity: key.clone(),
                scope: Some("HDLIR mapping catalogue".to_owned()),
            });
        }

        self.contracts.insert(key, contract);
        Ok(())
    }

    /// Look up a contract by direction and feature pair.
    #[must_use]
    pub fn get(
        &self,
        direction: MappingDirection,
        source: &FeatureId,
        destination: &FeatureId,
    ) -> Option<&MappingContract> {
        let key = format!(
            "{}:{}:{}",
            direction,
            source.as_str(),
            destination.as_str()
        );
        self.contracts.get(&key)
    }

    /// Iterate over contracts in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &MappingContract)> {
        self.contracts
            .iter()
            .map(|(key, contract)| (key.as_str(), contract))
    }

    /// Return the number of registered contracts as `u64`, if representable.
    pub fn len(&self) -> HdlirResult<u64> {
        u64::try_from(self.contracts.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.contracts.len().to_string(),
                destination: "u64",
            }
        })
    }

    /// Return whether the catalogue contains no contracts.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }
}

/// Evidence collected by an actual conversion adapter.
///
/// This record is deliberately separate from [`MappingContract`]. A contract
/// declares what must be true; evidence records what the adapter actually
/// checked. An adapter must not mark an obligation satisfied without a
/// corresponding real check or approved verification result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingEvidence {
    preconditions: BTreeMap<ObligationId, ObligationOutcome>,
    preservation: BTreeMap<ObligationId, ObligationOutcome>,
    provenance: ProvenanceOutcome,
    processing_complete: bool,
    unsupported_features: BTreeSet<FeatureId>,
    notes: BTreeSet<String>,
}

impl MappingEvidence {
    /// Start evidence collection for a mapping.
    ///
    /// Processing is incomplete until the adapter explicitly marks it
    /// complete after all required conversion and verification stages finish.
    #[must_use]
    pub fn new(provenance: ProvenanceOutcome) -> Self {
        Self {
            preconditions: BTreeMap::new(),
            preservation: BTreeMap::new(),
            provenance,
            processing_complete: false,
            unsupported_features: BTreeSet::new(),
            notes: BTreeSet::new(),
        }
    }

    /// Record the outcome of one precondition check.
    ///
    /// Re-recording an obligation replaces its previous outcome. The final
    /// assessment reflects the latest recorded result.
    pub fn record_precondition(
        &mut self,
        obligation: ObligationId,
        outcome: ObligationOutcome,
    ) {
        self.preconditions.insert(obligation, outcome);
    }

    /// Record the outcome of one preservation check.
    pub fn record_preservation(
        &mut self,
        obligation: ObligationId,
        outcome: ObligationOutcome,
    ) {
        self.preservation.insert(obligation, outcome);
    }

    /// Record a feature the adapter could not support.
    pub fn record_unsupported(&mut self, feature: FeatureId) {
        self.unsupported_features.insert(feature);
    }

    /// Record a deterministic explanatory note.
    ///
    /// Notes are diagnostic context only and must never be used as proof.
    pub fn record_note(&mut self, note: impl Into<String>) {
        self.notes.insert(note.into());
    }

    /// Mark all required adapter work as completed.
    ///
    /// Call this only after all required conversion and verification stages
    /// have actually completed. It does not itself prove preservation.
    pub fn mark_processing_complete(&mut self) {
        self.processing_complete = true;
    }

    /// Return whether the adapter completed all of its declared work.
    #[must_use]
    pub const fn processing_complete(&self) -> bool {
        self.processing_complete
    }

    /// Return recorded provenance handling.
    #[must_use]
    pub const fn provenance(&self) -> ProvenanceOutcome {
        self.provenance
    }

    /// Return unsupported features in deterministic order.
    #[must_use]
    pub fn unsupported_features(&self) -> &BTreeSet<FeatureId> {
        &self.unsupported_features
    }

    /// Return explanatory notes in deterministic order.
    #[must_use]
    pub fn notes(&self) -> &BTreeSet<String> {
        &self.notes
    }
}

/// Result classification for assessing a mapping against its contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MappingAssessmentStatus {
    /// Every required check passed and processing completed.
    Lossless,
    /// Every required check passed and processing completed, using an
    /// approved extension-preserving mapping.
    LosslessWithExtension,
    /// A required precondition or preservation obligation failed.
    Failed,
    /// A required check or conversion stage remains unresolved.
    Incomplete,
    /// At least one required feature is unsupported.
    Unsupported,
    /// The declared mapping cannot be accepted as lossless.
    LossyMappingRejected,
}

/// Assessment of the evidence supplied for a mapping.
///
/// An assessment is a deterministic report. It is not a formal-equivalence
/// proof unless the supplied evidence itself comes from a valid proof system
/// whose contract is independently established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingAssessment {
    status: MappingAssessmentStatus,
    missing_preconditions: BTreeSet<ObligationId>,
    failed_obligations: BTreeSet<ObligationId>,
    unresolved_obligations: BTreeSet<ObligationId>,
    unsupported_features: BTreeSet<FeatureId>,
    provenance_accepted: bool,
}

impl MappingAssessment {
    /// Assess evidence against a declared contract.
    ///
    /// The precedence is conservative: unsupported features, failed
    /// obligations, unresolved work, provenance failures, and forbidden
    /// fidelity prevent a lossless status.
    pub fn assess(
        contract: &MappingContract,
        evidence: &MappingEvidence,
    ) -> HdlirResult<Self> {
        contract.validate()?;

        let mut missing_preconditions = BTreeSet::new();
        let mut failed_obligations = BTreeSet::new();
        let mut unresolved_obligations = BTreeSet::new();

        for obligation in contract.required_preconditions() {
            match evidence.preconditions.get(obligation) {
                Some(ObligationOutcome::Satisfied) => {}
                Some(ObligationOutcome::Failed) => {
                    failed_obligations.insert(obligation.clone());
                }
                Some(ObligationOutcome::Unresolved) | None => {
                    missing_preconditions.insert(obligation.clone());
                    unresolved_obligations.insert(obligation.clone());
                }
            }
        }

        for obligation in contract.preservation_obligations() {
            match evidence.preservation.get(obligation) {
                Some(ObligationOutcome::Satisfied) => {}
                Some(ObligationOutcome::Failed) => {
                    failed_obligations.insert(obligation.clone());
                }
                Some(ObligationOutcome::Unresolved) | None => {
                    unresolved_obligations.insert(obligation.clone());
                }
            }
        }

        let provenance_accepted = match contract.provenance_policy() {
            ProvenancePolicy::MustPreserve => {
                evidence.provenance == ProvenanceOutcome::Preserved
            }
            ProvenancePolicy::PreserveOrDerive => matches!(
                evidence.provenance,
                ProvenanceOutcome::Preserved
                    | ProvenanceOutcome::DerivedWithRecord
            ),
            ProvenancePolicy::NotRequired => {
                evidence.provenance == ProvenanceOutcome::NotRequired
            }
        };

        let status = if !evidence.unsupported_features.is_empty()
            || contract.fidelity() == MappingFidelity::Unsupported
        {
            MappingAssessmentStatus::Unsupported
        } else if !failed_obligations.is_empty() {
            MappingAssessmentStatus::Failed
        } else if !evidence.processing_complete()
            || !unresolved_obligations.is_empty()
            || !provenance_accepted
        {
            MappingAssessmentStatus::Incomplete
        } else if !contract.fidelity().permits_lossless_result() {
            MappingAssessmentStatus::LossyMappingRejected
        } else if contract.fidelity() == MappingFidelity::ExtensionPreserving {
            MappingAssessmentStatus::LosslessWithExtension
        } else {
            MappingAssessmentStatus::Lossless
        };

        Ok(Self {
            status,
            missing_preconditions,
            failed_obligations,
            unresolved_obligations,
            unsupported_features: evidence.unsupported_features.clone(),
            provenance_accepted,
        })
    }

    /// Return the assessment status.
    #[must_use]
    pub const fn status(&self) -> MappingAssessmentStatus {
        self.status
    }

    /// Return whether this assessment permits the caller to report a
    /// lossless conversion.
    #[must_use]
    pub const fn is_lossless(&self) -> bool {
        matches!(
            self.status,
            MappingAssessmentStatus::Lossless
                | MappingAssessmentStatus::LosslessWithExtension
        )
    }

    /// Return preconditions that were not established as satisfied.
    #[must_use]
    pub fn missing_preconditions(&self) -> &BTreeSet<ObligationId> {
        &self.missing_preconditions
    }

    /// Return obligations that were explicitly failed.
    #[must_use]
    pub fn failed_obligations(&self) -> &BTreeSet<ObligationId> {
        &self.failed_obligations
    }

    /// Return obligations that remain unresolved.
    #[must_use]
    pub fn unresolved_obligations(&self) -> &BTreeSet<ObligationId> {
        &self.unresolved_obligations
    }

    /// Return features that the adapter reported as unsupported.
    #[must_use]
    pub fn unsupported_features(&self) -> &BTreeSet<FeatureId> {
        &self.unsupported_features
    }

    /// Return whether provenance handling satisfies the contract.
    #[must_use]
    pub const fn provenance_accepted(&self) -> bool {
        self.provenance_accepted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feature(name: &str) -> FeatureId {
        FeatureId::new(name).expect("valid test feature")
    }

    fn obligation(name: &str) -> ObligationId {
        ObligationId::new(name).expect("valid test obligation")
    }

    fn contract(fidelity: MappingFidelity) -> MappingContract {
        MappingContract::new(
            feature("semantic.hdl.register"),
            feature("hdlir.sequential.register"),
            MappingDirection::SemanticToHdlir,
            fidelity,
            ProvenancePolicy::MustPreserve,
        )
        .requiring(obligation("semantic.input.validated"))
        .preserving(obligation("state.transition.preserved"))
        .preserving(obligation("clock.reset.preserved"))
    }

    fn complete_evidence() -> MappingEvidence {
        let mut evidence =
            MappingEvidence::new(ProvenanceOutcome::Preserved);

        evidence.record_precondition(
            obligation("semantic.input.validated"),
            ObligationOutcome::Satisfied,
        );
        evidence.record_preservation(
            obligation("state.transition.preserved"),
            ObligationOutcome::Satisfied,
        );
        evidence.record_preservation(
            obligation("clock.reset.preserved"),
            ObligationOutcome::Satisfied,
        );
        evidence.mark_processing_complete();
        evidence
    }

    #[test]
    fn rejects_invalid_feature_identifiers() {
        assert!(FeatureId::new("").is_err());
        assert!(FeatureId::new(" leading").is_err());
        assert!(FeatureId::new("feature with spaces").is_err());
        assert!(FeatureId::new("feature/").is_err());
        assert!(FeatureId::new("semantic.hdl.register").is_ok());
    }

    #[test]
    fn accepts_a_complete_exact_mapping() {
        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &complete_evidence(),
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Lossless);
        assert!(assessment.is_lossless());
    }

    #[test]
    fn does_not_treat_unfinished_work_as_success() {
        let mut evidence = complete_evidence();
        // Construct fresh evidence without the completion signal.
        evidence.processing_complete = false;

        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &evidence,
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Incomplete);
        assert!(!assessment.is_lossless());
    }

    #[test]
    fn rejects_missing_preservation_evidence() {
        let mut evidence = complete_evidence();
        evidence.preservation.remove(&obligation("clock.reset.preserved"));

        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &evidence,
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Incomplete);
        assert!(!assessment.is_lossless());
        assert!(
            assessment
                .unresolved_obligations()
                .contains(&obligation("clock.reset.preserved"))
        );
    }

    #[test]
    fn rejects_a_failed_semantic_obligation() {
        let mut evidence = complete_evidence();
        evidence.record_preservation(
            obligation("state.transition.preserved"),
            ObligationOutcome::Failed,
        );

        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &evidence,
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Failed);
        assert!(!assessment.is_lossless());
    }

    #[test]
    fn rejects_unsupported_features() {
        let mut evidence = complete_evidence();
        evidence.record_unsupported(feature("hdlir.unsupported.process"));

        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &evidence,
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Unsupported);
        assert!(!assessment.is_lossless());
    }

    #[test]
    fn requires_recorded_provenance() {
        let mut evidence =
            MappingEvidence::new(ProvenanceOutcome::Unavailable);

        evidence.record_precondition(
            obligation("semantic.input.validated"),
            ObligationOutcome::Satisfied,
        );
        evidence.record_preservation(
            obligation("state.transition.preserved"),
            ObligationOutcome::Satisfied,
        );
        evidence.record_preservation(
            obligation("clock.reset.preserved"),
            ObligationOutcome::Satisfied,
        );
        evidence.mark_processing_complete();

        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::Exact),
            &evidence,
        )
        .expect("contract assessment");

        assert_eq!(assessment.status(), MappingAssessmentStatus::Incomplete);
        assert!(!assessment.provenance_accepted());
        assert!(!assessment.is_lossless());
    }

    #[test]
    fn permits_a_fully_preserved_extension_mapping() {
        let assessment = MappingAssessment::assess(
            &contract(MappingFidelity::ExtensionPreserving),
            &complete_evidence(),
        )
        .expect("contract assessment");

        assert_eq!(
            assessment.status(),
            MappingAssessmentStatus::LosslessWithExtension
        );
        assert!(assessment.is_lossless());
    }

    #[test]
    fn catalog_rejects_duplicate_contracts_without_replacement() {
        let mut catalog = MappingCatalog::new();
        let first = contract(MappingFidelity::Exact);
        let duplicate = contract(MappingFidelity::Unsupported);

        catalog.register(first.clone()).expect("first registration");
        assert!(catalog.register(duplicate).is_err());

        let found = catalog
            .get(
                first.direction(),
                first.source(),
                first.destination(),
            )
            .expect("original contract remains registered");

        assert_eq!(found.fidelity(), MappingFidelity::Exact);
    }

    #[test]
    fn iteration_is_deterministic() {
        let mut catalog = MappingCatalog::new();
        catalog
            .register(contract(MappingFidelity::Exact))
            .expect("registration");

        let keys: Vec<_> = catalog.iter().map(|(key, _)| key).collect();
        assert_eq!(keys, vec!["semantic_to_hdlir:semantic.hdl.register:hdlir.sequential.register"]);
    }

    #[test]
    fn rejects_unjustified_self_mapping() {
        let self_mapping = MappingContract::new(
            feature("hdlir.register"),
            feature("hdlir.register"),
            MappingDirection::HdlirToZuir,
            MappingFidelity::Exact,
            ProvenancePolicy::MustPreserve,
        );

        assert!(self_mapping.validate().is_err());
    }
}
