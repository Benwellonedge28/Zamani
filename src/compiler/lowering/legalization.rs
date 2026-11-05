
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Legalization
//!
//! Path: src/compiler/lowering/legalization.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ============================================================================
//! 1. PURPOSE
//! ============================================================================
//!
//! This module provides the compiler-facing legalization boundary.
//!
//! Legalization prepares a verified source representation for a subsequent
//! compiler representation or backend contract while preserving the
//! computational meaning established by Zamani's semantic authorities.
//!
//! The module coordinates:
//!
//! - Verified compiler IR input.
//! - Canonical representation conversion.
//! - Canonical lowering legality analysis.
//! - Domain-owned lowering implementations.
//! - Destination verification.
//! - Explicit legalization outcomes.
//!
//! It does not implement a competing IR, verifier, optimizer, or backend.
//!
//! ============================================================================
//! 2. ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! OWNS:
//!
//! - Compiler-facing legalization orchestration.
//! - Legalization result classification.
//! - Destination verification admission.
//! - Structured legalization errors.
//! - The boundary between lowering and subsequent compiler stages.
//!
//! DOES NOT OWN:
//!
//! - Source grammar.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical IR definitions.
//! - IR verification algorithms.
//! - Lowering legality algorithms.
//! - Domain-specific operation definitions.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Capability definitions.
//! - Resource allocation.
//! - Physical placement.
//! - Scheduling.
//! - Routing.
//! - Backend implementations.
//! - Runtime execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ============================================================================
//! 3. CANONICAL REPOSITORY AUTHORITIES
//! ============================================================================
//!
//! Verified input:
//!     crate::compiler::ir::input
//!
//! Compiler lowering:
//!     crate::compiler::ir::lowering
//!
//! Representation translation:
//!     crate::compiler::ir::translation
//!
//! IR verification:
//!     crate::compiler::ir::verification
//!
//! Canonical lowering API:
//!     crate::zuir::lowering::api
//!
//! Canonical legality analysis:
//!     crate::zuir::lowering::legality
//!
//! Canonical lowering context:
//!     crate::zuir::lowering::context
//!
//! Universal IR:
//!     crate::zuir
//!
//! Classical IR:
//!     crate::classical::zcir
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware description IR:
//!     crate::hdl::hdlir
//!
//! AI computation:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! ============================================================================
//! 4. LEGALIZATION CONTRACT
//! ============================================================================
//!
//! A successful legalization operation requires:
//!
//! 1. A valid verification-gated source input.
//! 2. Explicit source verification evidence.
//! 3. An applicable representation conversion contract.
//! 4. Canonical lowering legality authorization.
//! 5. Successful execution of the authoritative lowering backend.
//! 6. Successful verification of the resulting destination artifact.
//!
//! A backend-produced artifact that has not passed destination verification
//! must never be represented as a verified legalization result.
//!
//! A missing verifier is not equivalent to successful verification.
//!
//! An unsupported conversion is not equivalent to a successful conversion.
//!
//! ============================================================================
//! 5. SEMANTIC PRESERVATION
//! ============================================================================
//!
//! Legalization must preserve all applicable:
//!
//! - Values and types.
//! - Control-flow relationships.
//! - Data dependencies.
//! - Effects.
//! - Ownership constraints.
//! - Memory semantics.
//! - Numerical guarantees.
//! - Concurrency semantics.
//! - Synchronization requirements.
//! - Resource requirements.
//! - Quantum measurement semantics.
//! - Quantum/classical dependencies.
//! - HDL signal semantics.
//! - AI and tensor semantics.
//! - Distributed communication semantics.
//! - Provenance.
//!
//! Potentially lossy conversions are rejected by the existing translation
//! subsystem.
//!
//! This module does not independently prove semantic equivalence.
//! It relies on the authoritative conversion guarantees and backend
//! contracts supplied by the relevant domain owners.
//!
//! ============================================================================
//! 6. OUTCOME CLASSIFICATION
//! ============================================================================
//!
//! Legalized:
//!     The authoritative backend produced an artifact and the supplied
//!     destination verifier accepted it.
//!
//! Not legalized:
//!     Canonical legality analysis did not authorize direct lowering,
//!     or the destination artifact has not been established as verified.
//!
//! Failed:
//!     An explicit conversion, backend, or verification error occurred.
//!
//! These outcomes must never be silently merged.
//!
//! ============================================================================
//! 7. SCALABILITY
//! ============================================================================
//!
//! This module introduces no fixed limits on:
//!
//! - Programs.
//! - Modules.
//! - Functions.
//! - Operations.
//! - Values.
//! - Regions.
//! - Blocks.
//! - Types.
//! - Effects.
//! - Qubits.
//! - Registers.
//! - Signals.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Devices.
//! - Processors.
//! - Accelerators.
//! - Distributed nodes.
//! - Computing domains.
//! - Legalization transformations.
//!
//! Memory and processing capacity are determined by available resources,
//! representation constraints, and explicit operational policies.
//!
//! No finite implementation can promise literal infinity.
//!
//! Resource exhaustion must be reported rather than silently truncating
//! a program or claiming successful legalization.
//!
//! ============================================================================
//! 8. DETERMINISM
//! ============================================================================
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No random identifiers.
//! - No hidden target selection.
//! - No implicit hardware allocation.
//! - No nondeterministic fallback.
//! - No silent semantic substitution.
//!
//! Determinism of conversion and verification is governed by their
//! authoritative implementations.
//!
//! ============================================================================
//! 9. ERROR HANDLING
//! ============================================================================
//!
//! Errors remain inspectable through their original sources.
//!
//! Distinct failures include:
//!
//! - Invalid verified input.
//! - Missing source verification.
//! - Lowering failure.
//! - Destination verification failure.
//! - Backend resource exhaustion.
//! - Unsupported conversion.
//! - Semantic incompatibility.
//!
//! A non-legal legality result is not automatically a backend error.
//!
//! ============================================================================
//! 10. INTEGRATION
//! ============================================================================
//!
//! Parent:
//!     src/compiler/lowering/mod.rs
//!
//! Compiler root:
//!     src/compiler/mod.rs
//!
//! Required existing modules:
//!     compiler::ir::input
//!     compiler::ir::lowering
//!     compiler::ir::translation
//!     zuir::lowering::api
//!
//! Downstream:
//!     compiler pipeline
//!     compiler passes
//!     optimization
//!     planning
//!     backend integration
//!
//! Destination verification must be supplied by the authoritative
//! destination representation owner.
//!
//! ============================================================================
//! 11. IMPORTANT LIMITATION
//! ============================================================================
//!
//! This module provides verified legalization orchestration.
//!
//! It does not claim to implement automatic decomposition of every
//! unsupported operation.
//!
//! Actual operation decomposition, legalization patterns, and target
//! conversion algorithms must be implemented by registered,
//! domain-owned transformation providers.
//!
//! This separation prevents the compiler from becoming a duplicate
//! owner of quantum, classical, HDL, AI, and other domain semantics.

use core::fmt;
use std::error::Error;

use crate::compiler::ir::lowering::{
    lower_verified_with,
    IrLoweringError,
    IrLoweringOutcome,
};

use crate::compiler::ir::input::VerifiedIrInput;

use crate::compiler::ir::adapters::AdapterDescriptor;

use crate::zuir::lowering::api::LoweringBackend;

use crate::zuir::lowering::context::LoweringContext;

use crate::zuir::lowering::legality::{
    LegalityCandidate,
    LegalityReport,
    LegalityStatus,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler legalization.
pub const LEGALIZATION_API_VERSION: u16 = 1;

/// Stable identity of this module.
pub const LEGALIZATION_MODULE_NAME: &str =
    "zamani.compiler.lowering.legalization";

/// Return the legalization API version.
#[must_use]
pub const fn api_version() -> u16 {
    LEGALIZATION_API_VERSION
}

/// Return the stable module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    LEGALIZATION_MODULE_NAME
}

// ============================================================================
// LEGALIZATION STATUS
// ============================================================================

/// Final state of a legalization request.
///
/// A successful Rust function return does not necessarily mean
/// that an artifact was produced. The status must be inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LegalizationStatus {
    /// Backend conversion completed and destination verification passed.
    Legalized,

    /// Canonical legality analysis did not authorize direct conversion.
    ///
    /// The legality report explains the reason.
    NotLegalized,

    /// Conversion completed, but destination verification was not
    /// successfully established.
    ///
    /// This status is not accepted as a verified compilation artifact.
    UnverifiedDestination,
}

impl LegalizationStatus {
    /// Return true only for fully verified legalization.
    #[must_use]
    pub const fn is_legalized(self) -> bool {
        matches!(self, Self::Legalized)
    }

    /// Return true if the result must not proceed as verified output.
    #[must_use]
    pub const fn requires_further_work(self) -> bool {
        !self.is_legalized()
    }
}

// ============================================================================
// DESTINATION VERIFICATION
// ============================================================================

/// Evidence that a destination artifact passed its authoritative verifier.
///
/// This is an explicit result of a verifier supplied by the destination
/// representation owner.
///
/// It is not a replacement for the canonical verification report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestinationVerification {
    verifier: String,
    verifier_version: String,
    verified: bool,
}

impl DestinationVerification {
    /// Construct successful destination verification evidence.
    ///
    /// The caller must only invoke this constructor after the actual
    /// destination verifier has accepted the exact artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when verifier identity or version is empty.
    pub fn verified(
        verifier: impl Into<String>,
        verifier_version: impl Into<String>,
    ) -> Result<Self, LegalizationError<std::convert::Infallible, std::convert::Infallible>> {
        let verifier = verifier.into();
        let verifier_version = verifier_version.into();

        if verifier.trim().is_empty() {
            return Err(LegalizationError::InvalidVerifierIdentity);
        }

        if verifier_version.trim().is_empty() {
            return Err(LegalizationError::InvalidVerifierVersion);
        }

        Ok(Self {
            verifier,
            verifier_version,
            verified: true,
        })
    }

    /// Return the verifier identity.
    #[must_use]
    pub fn verifier(&self) -> &str {
        &self.verifier
    }

    /// Return the verifier version.
    #[must_use]
    pub fn verifier_version(&self) -> &str {
        &self.verifier_version
    }

    /// Return whether verification succeeded.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}

// ============================================================================
// LEGALIZATION ERROR
// ============================================================================

/// Errors produced by the compiler legalization boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum LegalizationError<BackendError, VerificationError> {
    /// The source verification marker was invalid.
    InvalidVerifiedInput,

    /// The source conversion candidate did not establish verification.
    SourceVerificationNotEstablished,

    /// The authoritative lowering or translation implementation failed.
    Lowering(IrLoweringError<BackendError>),

    /// Destination verification failed.
    DestinationVerification(VerificationError),

    /// The destination verifier identity was empty.
    InvalidVerifierIdentity,

    /// The destination verifier version was empty.
    InvalidVerifierVersion,

    /// A backend claimed success without producing an artifact.
    MissingArtifactForVerifiedDestination,
}

impl<B, V> fmt::Display for LegalizationError<B, V>
where
    B: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVerifiedInput => {
                write!(formatter, "legalization rejected invalid verified input")
            }

            Self::SourceVerificationNotEstablished => {
                write!(
                    formatter,
                    "legalization requires established source verification"
                )
            }

            Self::Lowering(error) => {
                write!(formatter, "legalization lowering failed: {error}")
            }

            Self::DestinationVerification(error) => {
                write!(
                    formatter,
                    "destination verification failed: {error}"
                )
            }

            Self::InvalidVerifierIdentity => {
                write!(formatter, "destination verifier identity is empty")
            }

            Self::InvalidVerifierVersion => {
                write!(formatter, "destination verifier version is empty")
            }

            Self::MissingArtifactForVerifiedDestination => {
                write!(
                    formatter,
                    "destination verification cannot authorize a missing artifact"
                )
            }
        }
    }
}

impl<B, V> Error for LegalizationError<B, V>
where
    B: Error + 'static,
    V: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lowering(error) => Some(error),
            Self::DestinationVerification(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// LEGALIZATION OUTCOME
// ============================================================================

/// Result of one legalization operation.
///
/// The original compiler lowering outcome and canonical legality report
/// remain available.
///
/// A caller must inspect `status()` before using the artifact downstream.
#[derive(Debug)]
pub struct LegalizationOutcome<Artifact> {
    lowering: IrLoweringOutcome<Artifact>,
    status: LegalizationStatus,
    verification: Option<DestinationVerification>,
}

impl<T> LegalizationOutcome<T> {
    fn new(
        lowering: IrLoweringOutcome<T>,
        status: LegalizationStatus,
        verification: Option<DestinationVerification>,
    ) -> Self {
        Self {
            lowering,
            status,
            verification,
        }
    }

    /// Return the final legalization status.
    #[must_use]
    pub const fn status(&self) -> LegalizationStatus {
        self.status
    }

    /// Return true only when legalization and destination verification
    /// have both succeeded.
    #[must_use]
    pub const fn is_legalized(&self) -> bool {
        self.status.is_legalized()
    }

    /// Return the original compiler lowering outcome.
    #[must_use]
    pub const fn lowering(&self) -> &IrLoweringOutcome<T> {
        &self.lowering
    }

    /// Return the canonical legality report.
    #[must_use]
    pub fn legality(&self) -> &LegalityReport {
        self.lowering.legality()
    }

    /// Return destination verification evidence, if available.
    #[must_use]
    pub const fn verification(&self) -> Option<&DestinationVerification> {
        self.verification.as_ref()
    }

    /// Borrow the artifact only when legalization is complete.
    ///
    /// An artifact from an unsuccessful or unverified conversion
    /// is deliberately not exposed through this method.
    #[must_use]
    pub fn artifact(&self) -> Option<&T> {
        if !self.is_legalized() {
            return None;
        }

        self.lowering.artifact()
    }

    /// Consume the outcome and return a fully legalized artifact.
    ///
    /// Returns `None` unless destination verification succeeded.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        if !self.is_legalized() {
            return None;
        }

        self.lowering.into_artifact()
    }
}

// ============================================================================
// LEGALIZATION RESULT
// ============================================================================

/// Result type for verified legalization.
pub type LegalizationResult<T, B, V> =
    Result<LegalizationOutcome<T>, LegalizationError<B, V>>;

// ============================================================================
// LEGALIZATION ENTRY POINT
// ============================================================================

/// Legalize verified compiler IR through an authoritative domain backend.
///
/// This function composes existing compiler and ZUIR lowering contracts.
///
/// It does not implement a second conversion algorithm.
///
/// # Arguments
///
/// `verified_input`:
///
/// The verification-gated source IR input.
///
/// `descriptor`:
///
/// The explicit source-to-destination representation contract.
///
/// `context`:
///
/// The canonical lowering context containing requirements, policies,
/// mappings, provenance, and operational work accounting.
///
/// `candidate`:
///
/// Canonical legality evidence for the proposed conversion.
///
/// `backend`:
///
/// The authoritative domain-owned lowering backend.
///
/// `verify_destination`:
///
/// The authoritative destination verifier. It must inspect the exact
/// artifact returned by the backend.
///
/// # Behavior
///
/// 1. Reject invalid source verification.
/// 2. Reject candidates without source verification.
/// 3. Invoke the existing compiler lowering boundary.
/// 4. Preserve canonical legality analysis.
/// 5. Avoid destination verification when no artifact exists.
/// 6. Verify any produced destination artifact.
/// 7. Expose the artifact as usable only after verification succeeds.
///
/// # Errors
///
/// Returns structured errors for source admission, lowering,
/// backend, and destination verification failures.
///
/// # Important
///
/// A successful return may represent a non-legalized outcome.
/// Always inspect `is_legalized()`.
///
/// # Semantic safety
///
/// The verifier must be the actual verifier belonging to the destination
/// representation. A caller must not manufacture verification evidence
/// without checking the artifact.
pub fn legalize_verified_with<B, V, F>(
    verified_input: &VerifiedIrInput<'_>,
    descriptor: AdapterDescriptor,
    context: &mut LoweringContext,
    candidate: LegalityCandidate,
    backend: &mut B,
    verify_destination: F,
) -> LegalizationResult<B::Artifact, B::Error, V>
where
    B: LoweringBackend,
    F: FnOnce(&B::Artifact) -> Result<DestinationVerification, V>,
{
    let lowering = lower_verified_with(
        verified_input,
        descriptor,
        context,
        candidate,
        backend,
    )
    .map_err(|error| match error {
        IrLoweringError::InvalidVerifiedInput => {
            LegalizationError::InvalidVerifiedInput
        }

        IrLoweringError::SourceVerificationNotEstablished => {
            LegalizationError::SourceVerificationNotEstablished
        }

        other => LegalizationError::Lowering(other),
    })?;

    if !lowering.is_lowered() {
        return Ok(LegalizationOutcome::new(
            lowering,
            LegalizationStatus::NotLegalized,
            None,
        ));
    }

    let artifact = lowering.artifact().ok_or(
        LegalizationError::MissingArtifactForVerifiedDestination,
    )?;

    let verification = verify_destination(artifact)
        .map_err(LegalizationError::DestinationVerification)?;

    if !verification.is_verified() {
        return Ok(LegalizationOutcome::new(
            lowering,
            LegalizationStatus::UnverifiedDestination,
            Some(verification),
        ));
    }

    Ok(LegalizationOutcome::new(
        lowering,
        LegalizationStatus::Legalized,
        Some(verification),
    ))
}

// ============================================================================
// LEGALITY CLASSIFICATION
// ============================================================================

/// Determine whether canonical legality analysis authorizes direct lowering.
///
/// This function delegates interpretation to the existing legality result.
/// It does not recreate the legality algorithm.
#[must_use]
pub fn permits_legalization(report: &LegalityReport) -> bool {
    matches!(report.status, LegalityStatus::Legal)
}

// ============================================================================
// API INFORMATION
// ============================================================================

/// Return the stable legalization module identity and API version.
#[must_use]
pub const fn api_information() -> (&'static str, u16) {
    (
        LEGALIZATION_MODULE_NAME,
        LEGALIZATION_API_VERSION,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalization_module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.lowering.legalization"
        );
    }

    #[test]
    fn legalization_api_version_is_available() {
        assert_eq!(
            api_version(),
            LEGALIZATION_API_VERSION
        );

        assert_eq!(
            LEGALIZATION_API_VERSION,
            1
        );
    }

    #[test]
    fn legal_status_is_successful() {
        assert!(LegalizationStatus::Legalized.is_legalized());
    }

    #[test]
    fn deferred_status_is_not_successful() {
        assert!(!LegalizationStatus::NotLegalized.is_legalized());
    }

    #[test]
    fn unverified_destination_is_not_successful() {
        assert!(
            LegalizationStatus::UnverifiedDestination
                .requires_further_work()
        );
    }

    #[test]
    fn empty_verifier_identity_is_rejected() {
        let result = DestinationVerification::verified(
            "",
            "1",
        );

        assert!(matches!(
            result,
            Err(LegalizationError::InvalidVerifierIdentity)
        ));
    }

    #[test]
    fn empty_verifier_version_is_rejected() {
        let result = DestinationVerification::verified(
            "canonical-verifier",
            "",
        );

        assert!(matches!(
            result,
            Err(LegalizationError::InvalidVerifierVersion)
        ));
    }

    #[test]
    fn valid_verification_evidence_is_preserved() {
        let verification = DestinationVerification::verified(
            "canonical-zcir-verifier",
            "1",
        )
        .expect("valid verification evidence");

        assert!(verification.is_verified());

        assert_eq!(
            verification.verifier(),
            "canonical-zcir-verifier"
        );

        assert_eq!(
            verification.verifier_version(),
            "1"
        );
    }

    #[test]
    fn legalization_error_display_is_structured() {
        let error: LegalizationError<
            std::io::Error,
            std::io::Error,
        > = LegalizationError::InvalidVerifiedInput;

        assert!(
            error.to_string().contains("invalid verified input")
        );
    }

    #[test]
    fn missing_artifact_error_is_distinct() {
        let error: LegalizationError<
            std::io::Error,
            std::io::Error,
        > = LegalizationError::MissingArtifactForVerifiedDestination;

        assert!(
            error.to_string().contains("missing artifact")
        );
    }

    #[test]
    fn module_information_is_consistent() {
        let (name, version) = api_information();

        assert_eq!(name, module_name());
        assert_eq!(version, api_version());
    }
}
