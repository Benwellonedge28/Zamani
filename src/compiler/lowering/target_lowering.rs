
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Target Lowering
//!
//! Path:
//!     src/compiler/lowering/target_lowering.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provide the compiler's target-specific lowering admission and
//! orchestration boundary.
//!
//! This module connects:
//!
//! - Verification-gated canonical IR input.
//! - Target descriptors.
//! - Target-specific backend providers.
//! - Canonical lowering legality.
//! - Existing compiler IR lowering.
//! - Domain-owned lowering implementations.
//!
//! Actual representation conversion remains owned by the existing
//! domain and backend implementations.
//!
//! This module does not implement another IR or lowering engine.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! Verified ZUIR
//!       |
//!       v
//! Target descriptor validation
//!       |
//!       v
//! Backend identity validation
//!       |
//!       v
//! Backend target-admission validation
//!       |
//!       v
//! Canonical IR lowering
//!       |
//!       v
//! Semantic legality analysis
//!       |
//!       v
//! Target-specific backend
//!       |
//!       v
//! Lowering outcome
//!       |
//!       v
//! Destination verification
//!       |
//!       v
//! Artifact integration
//!
//! Destination verification and artifact integration are subsequent
//! responsibilities. This module does not claim they have occurred.
//!
//! ============================================================================
//! AUTHORITATIVE OWNERS
//! ============================================================================
//!
//! Verified input:
//!     crate::compiler::ir::input
//!
//! Canonical compiler lowering:
//!     crate::compiler::ir::lowering
//!
//! Representation translation:
//!     crate::compiler::ir::translation
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Canonical lowering API:
//!     crate::zuir::lowering::api
//!
//! Lowering context:
//!     crate::zuir::lowering::context
//!
//! Lowering legality:
//!     crate::zuir::lowering::legality
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Target-specific lowering admission.
//! - Backend-to-target identity checking.
//! - Target descriptor validation before lowering.
//! - The target lowering backend contract.
//! - Target lowering result metadata.
//! - Target lowering error classification.
//! - Delegation to canonical verified IR lowering.
//!
//! This module DOES NOT own:
//!
//! - Source grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical IR definitions.
//! - Domain IR definitions.
//! - IR verification algorithms.
//! - Lowering legality algorithms.
//! - Hardware discovery.
//! - Capability measurement.
//! - Physical resource allocation.
//! - Physical routing.
//! - Execution scheduling.
//! - Backend registration.
//! - Native code generation.
//! - Runtime execution.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module introduces no fixed limits on:
//!
//! - Modules.
//! - Functions.
//! - Operations.
//! - Values.
//! - Types.
//! - Effects.
//! - Registers.
//! - Signals.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Devices.
//! - Distributed nodes.
//! - Computing domains.
//! - Hybrid combinations.
//!
//! Actual implementation capacity is bounded by available resources,
//! representation constraints, backend capabilities and configured
//! operational budgets.
//!
//! No finite implementation can promise literal infinity.
//!
//! Resource exhaustion must never be interpreted as successful lowering.
//!
//! ============================================================================
//! SEMANTIC PRESERVATION
//! ============================================================================
//!
//! The backend provider must preserve:
//!
//! - Observable program behavior.
//! - Operation semantics.
//! - Data dependencies.
//! - Control-flow dependencies.
//! - Effects.
//! - Ownership guarantees.
//! - Numerical guarantees.
//! - Synchronization.
//! - Communication semantics.
//! - Resource requirements.
//! - Domain-specific guarantees.
//!
//! The canonical legality subsystem remains authoritative.
//!
//! Potentially lossy translations are rejected by the existing
//! compiler IR translation boundary.
//!
//! ============================================================================
//! TARGET ADMISSION
//! ============================================================================
//!
//! A TargetDescriptor is metadata, not proof that a backend can
//! implement the target.
//!
//! Therefore:
//!
//! 1. The descriptor must be structurally valid.
//! 2. The backend must identify the target it implements.
//! 3. The backend must explicitly validate target compatibility.
//! 4. The verified IR lowering API must authorize the conversion.
//! 5. The backend must report conversion failures explicitly.
//!
//! Target capability evidence must originate from the canonical
//! planning and capability authorities.
//!
//! A backend must not fabricate capability evidence.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Parent:
//!     src/compiler/lowering/mod.rs
//!
//! Compiler root:
//!     src/compiler/mod.rs
//!
//! Existing verified lowering:
//!     src/compiler/ir/lowering.rs
//!
//! No additional src/lib.rs declaration is required.
//!
//! ============================================================================

use core::fmt;
use std::error::Error;

use crate::compiler::configuration::target::{
    TargetDescriptor,
    TargetError,
    TargetId,
};

use crate::compiler::ir::input::VerifiedIrInput;

use crate::compiler::ir::lowering::{
    lower_verified_with,
    IrLoweringError,
    IrLoweringOutcome,
};

use crate::compiler::ir::adapters::AdapterDescriptor;

use crate::zuir::lowering::api::LoweringBackend;

use crate::zuir::lowering::context::LoweringContext;

use crate::zuir::lowering::legality::LegalityCandidate;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for target lowering.
pub const TARGET_LOWERING_API_VERSION: u16 = 1;

// ============================================================================
// BACKEND CONTRACT
// ============================================================================

/// Contract implemented by a target-specific lowering backend.
///
/// This trait extends the canonical ZUIR lowering backend contract.
///
/// The implementation owns actual target-specific conversion.
///
/// The compiler owns target admission and orchestration.
///
/// A provider must validate the target using its actual implementation
/// capabilities and the canonical compatibility contracts.
///
/// It must not treat a matching target identifier as proof that
/// arbitrary operations are supported.
pub trait TargetLoweringBackend: LoweringBackend {

    /// Return the stable identity of the target implemented by
    /// this backend provider.
    fn target_id(&self) -> &TargetId;

    /// Validate whether this backend can realize the supplied target.
    ///
    /// Implementations must inspect the actual target compatibility
    /// and capability contracts available to them.
    ///
    /// A successful result authorizes attempting lowering.
    ///
    /// It does not establish that every operation in the program
    /// is legal or that lowering will succeed.
    ///
    /// The canonical legality subsystem remains authoritative
    /// for semantic conversion legality.
    fn validate_target(
        &self,
        target: &TargetDescriptor,
    ) -> Result<(), Self::Error>;
}

// ============================================================================
// TARGET LOWERING ERROR
// ============================================================================

/// Errors produced by target-specific lowering.
#[derive(Debug)]
pub enum TargetLoweringError<E> {

    /// Target descriptor validation failed.
    InvalidTarget(TargetError),

    /// Backend identity does not match the requested target.
    ///
    /// This prevents accidentally invoking a backend registered
    /// for a different target.
    BackendTargetMismatch {
        /// Requested target identifier.
        requested: String,

        /// Backend target identifier.
        provided: String,
    },

    /// The backend rejected the target.
    ///
    /// This indicates that target admission could not be established.
    TargetAdmission(E),

    /// The canonical verified IR lowering implementation failed.
    Lowering(IrLoweringError<E>),
}

impl<E: fmt::Display> fmt::Display for TargetLoweringError<E> {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidTarget(error) => {
                write!(
                    formatter,
                    "target descriptor validation failed: {error}"
                )
            }

            Self::BackendTargetMismatch {
                requested,
                provided,
            } => {
                write!(
                    formatter,
                    "target/backend identity mismatch: \
                     requested `{requested}`, backend provides `{provided}`"
                )
            }

            Self::TargetAdmission(error) => {
                write!(
                    formatter,
                    "target backend admission failed: {error}"
                )
            }

            Self::Lowering(error) => {
                write!(
                    formatter,
                    "canonical target lowering failed: {error}"
                )
            }
        }
    }
}

impl<E> Error for TargetLoweringError<E>
where
    E: Error + 'static,
{
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {

        match self {

            Self::InvalidTarget(error) => Some(error),

            Self::BackendTargetMismatch { .. } => None,

            Self::TargetAdmission(error) => Some(error),

            Self::Lowering(error) => Some(error),
        }
    }
}

// ============================================================================
// TARGET LOWERING OUTCOME
// ============================================================================

/// Result of a target-specific lowering request.
///
/// This structure preserves the canonical lowering outcome.
///
/// An outcome may be returned without an artifact when the canonical
/// legality subsystem determines that direct lowering is not permitted.
///
/// Callers must inspect is_lowered() before accessing the artifact.
///
/// A successful lowering result does not establish destination
/// verification, linking success or runtime execution.
#[derive(Debug)]
pub struct TargetLoweringOutcome<T> {

    target_id: TargetId,

    target_name: String,

    lowering: IrLoweringOutcome<T>,
}

impl<T> TargetLoweringOutcome<T> {

    /// Construct a target lowering outcome.
    ///
    /// This constructor is private to preserve the admission boundary.
    fn new(
        target: &TargetDescriptor,
        lowering: IrLoweringOutcome<T>,
    ) -> Self {

        Self {
            target_id: target.id().clone(),
            target_name: target.display_name().to_owned(),
            lowering,
        }
    }

    /// Return the target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the target display name.
    #[must_use]
    pub fn target_name(&self) -> &str {
        &self.target_name
    }

    /// Return the canonical lowering outcome.
    #[must_use]
    pub const fn lowering(&self) -> &IrLoweringOutcome<T> {
        &self.lowering
    }

    /// Return whether the backend successfully produced an artifact.
    ///
    /// This is false for non-legal or incomplete lowering outcomes.
    #[must_use]
    pub fn is_lowered(&self) -> bool {
        self.lowering.is_lowered()
    }

    /// Borrow the artifact if lowering completed.
    #[must_use]
    pub fn artifact(&self) -> Option<&T> {
        self.lowering.artifact()
    }

    /// Consume the outcome and return its artifact if available.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        self.lowering.into_artifact()
    }

    /// Return the canonical lowering legality report.
    #[must_use]
    pub fn legality(
        &self,
    ) -> &crate::zuir::lowering::legality::LegalityReport {
        self.lowering.legality()
    }

    /// Return the destination entity identities.
    #[must_use]
    pub fn destinations(
        &self,
    ) -> &[crate::zuir::lowering::context::EntityKey] {
        self.lowering.destinations()
    }
}

// ============================================================================
// TARGET LOWERING RESULT
// ============================================================================

/// Result alias for target-specific lowering.
pub type TargetLoweringResult<T, E> =
    Result<T, TargetLoweringError<E>>;

// ============================================================================
// TARGET LOWERING ENGINE
// ============================================================================

/// Stateless target lowering orchestration service.
///
/// This type does not own a backend registry, hardware inventory,
/// global target state or physical resource manager.
pub struct TargetLowering;

impl TargetLowering {

    /// Return the target lowering module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        "zamani.compiler.lowering.target"
    }

    /// Return the target lowering API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        TARGET_LOWERING_API_VERSION
    }

    /// Lower verified canonical IR for an explicitly supplied target.
    ///
    /// # Arguments
    ///
    /// `verified_input`:
    ///
    /// Canonical IR input that passed the existing full-program
    /// verification boundary.
    ///
    /// `target`:
    ///
    /// Explicit target descriptor.
    ///
    /// `descriptor`:
    ///
    /// Existing representation conversion contract.
    ///
    /// `context`:
    ///
    /// Canonical ZUIR lowering context.
    ///
    /// `candidate`:
    ///
    /// Semantic and capability evidence for the proposed conversion.
    ///
    /// `backend`:
    ///
    /// Target-specific backend implementing the canonical lowering
    /// contract and this module's target admission contract.
    ///
    /// # Behavior
    ///
    /// 1. Validate the supplied target descriptor.
    /// 2. Verify backend identity against the requested target.
    /// 3. Ask the backend to validate target compatibility.
    /// 4. Delegate verified lowering to compiler::ir::lowering.
    /// 5. Preserve the canonical legality report.
    /// 6. Return the artifact only when lowering actually completed.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The target descriptor is invalid.
    /// - The backend belongs to another target.
    /// - The backend cannot admit the target.
    /// - Canonical lowering fails.
    ///
    /// # Important
    ///
    /// A successful function return does not necessarily mean that
    /// lowering produced an artifact.
    ///
    /// The caller must inspect outcome.is_lowered().
    ///
    /// A non-legal lowering decision is preserved as an outcome
    /// rather than converted into false success.
    ///
    /// # Target feasibility
    ///
    /// Target admission is not physical resource allocation.
    ///
    /// The caller must establish the required capability and resource
    /// evidence through the canonical planning architecture.
    ///
    /// This function does not independently discover hardware.
    ///
    /// # Semantic preservation
    ///
    /// The canonical lowering legality subsystem determines whether
    /// the proposed conversion is semantically permitted.
    ///
    /// This module does not override or bypass that decision.
    pub fn lower_for_target<B>(
        verified_input: &VerifiedIrInput<'_>,
        target: &TargetDescriptor,
        descriptor: AdapterDescriptor,
        context: &mut LoweringContext,
        candidate: LegalityCandidate,
        backend: &mut B,
    ) -> TargetLoweringResult<
        TargetLoweringOutcome<B::Artifact>,
        B::Error,
    >
    where
        B: TargetLoweringBackend,
    {

        // ------------------------------------------------------------
        // 1. Validate the complete target descriptor.
        // ------------------------------------------------------------

        target
            .validate()
            .map_err(TargetLoweringError::InvalidTarget)?;

        // ------------------------------------------------------------
        // 2. Validate backend identity.
        //
        // A backend for one target must never silently compile
        // a request intended for another target.
        // ------------------------------------------------------------

        if backend.target_id() != target.id() {

            return Err(
                TargetLoweringError::BackendTargetMismatch {
                    requested: target.id().as_str().to_owned(),
                    provided: backend.target_id().as_str().to_owned(),
                },
            );
        }

        // ------------------------------------------------------------
        // 3. Establish target admission.
        //
        // Descriptor validity alone is insufficient.
        // The backend must explicitly validate compatibility.
        // ------------------------------------------------------------

        backend
            .validate_target(target)
            .map_err(TargetLoweringError::TargetAdmission)?;

        // ------------------------------------------------------------
        // 4. Delegate to the existing canonical verified lowering.
        //
        // Do not duplicate:
        //
        // - Verified-input validation.
        // - Translation legality.
        // - Lowering legality.
        // - Backend invocation.
        // - Destination mapping.
        // ------------------------------------------------------------

        let lowering = lower_verified_with(
            verified_input,
            descriptor,
            context,
            candidate,
            backend,
        )
        .map_err(TargetLoweringError::Lowering)?;

        // ------------------------------------------------------------
        // 5. Preserve the canonical outcome.
        // ------------------------------------------------------------

        Ok(TargetLoweringOutcome::new(
            target,
            lowering,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn api_version_is_available() {

        assert_eq!(
            TargetLowering::api_version(),
            TARGET_LOWERING_API_VERSION,
        );

        assert_ne!(
            TARGET_LOWERING_API_VERSION,
            0,
        );
    }

    #[test]
    fn module_name_is_stable() {

        assert_eq!(
            TargetLowering::module_name(),
            "zamani.compiler.lowering.target",
        );
    }

    #[test]
    fn backend_identity_mismatch_is_reported() {

        let requested = TargetLoweringError::<std::io::Error>::
            BackendTargetMismatch {
                requested: "target:a".to_owned(),
                provided: "target:b".to_owned(),
            };

        assert!(
            requested
                .to_string()
                .contains("target identity mismatch")
        );
    }

    #[test]
    fn error_categories_are_distinct() {

        let mismatch =
            TargetLoweringError::<std::io::Error>::
                BackendTargetMismatch {
                    requested: "target:a".to_owned(),
                    provided: "target:b".to_owned(),
                };

        assert!(
            matches!(
                mismatch,
                TargetLoweringError::BackendTargetMismatch { .. }
            )
        );
    }
}
