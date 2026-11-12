
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Dispatch
//!
//! Path: src/zur/execution/dispatch.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides the ZUR dispatch boundary between an already-validated
//! execution plan and an explicitly supplied execution authority.
//!
//! Dispatch coordinates the hand-off of an execution request.
//!
//! It does not implement compilation, semantic analysis, planning,
//! hardware selection, scheduling, routing, resource allocation,
//! artifact loading, or domain-specific execution.
//!
//! # Architectural ownership
//!
//! Existing authorities remain authoritative:
//!
//! - zur::contracts::execution:
//!   ExecutionRequest, ExecutionEngine, ExecutionHandle.
//!
//! - zur::contracts::abi:
//!   Runtime ABI and invocation contracts.
//!
//! - zur::contracts::provider:
//!   Execution-provider contracts.
//!
//! - zur::execution::artifact:
//!   Artifact validation.
//!
//! - zur::execution::loader:
//!   Executable artifact loading.
//!
//! - zur::execution::session:
//!   Execution session and cancellation.
//!
//! - zur::execution::instance:
//!   Execution instance ownership.
//!
//! - compiler:
//!   Execution planning and compilation.
//!
//! - hybrid:
//!   Hybrid execution planning.
//!
//! - distributed:
//!   Distributed execution planning.
//!
//! - quantum:
//!   Quantum execution planning and hardware contracts.
//!
//! - HDL and AI:
//!   Their respective computational authorities.
//!
//! This module introduces no competing authority.
//!
//! # Dispatch responsibilities
//!
//! This module owns:
//!
//! - Validated dispatch evidence.
//! - Explicit execution-authority binding.
//! - Dispatch-plan identity verification.
//! - Artifact and plan binding verification.
//! - Compatibility-evidence association.
//! - Cancellation checks before submission.
//! - Execution-engine hand-off.
//! - Accepted-submission identity verification.
//! - Structured dispatch receipts.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - Interpret source grammar.
//! - Interpret AST or semantic requirements.
//! - Construct execution plans.
//! - Determine computational feasibility.
//! - Discover execution providers.
//! - Select physical hardware.
//! - Allocate or reserve resources.
//! - Choose a backend.
//! - Perform scheduling or routing.
//! - Load artifacts.
//! - Execute provider-specific operations.
//! - Claim successful execution after submission.
//! - Perform distributed recovery.
//!
//! # Important distinctions
//!
//! Plan validation != compatibility validation.
//! Compatibility != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != submission.
//! Submission != completion.
//! Cancellation request != confirmed cancellation.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Computational domains are open-ended.
//!
//! A new domain must integrate through the established planning,
//! provider and execution contracts rather than requiring a new
//! domain-specific branch in this module.
//!
//! There are no artificial maximums on:
//!
//! - Execution requests.
//! - Execution authorities.
//! - Computational domains.
//! - Devices.
//! - Providers.
//! - Resource quantities.
//! - Distributed participants.
//! - Execution plans.
//! - Compatibility evidence records.
//!
//! Actual finite limits arise from available memory, implementation
//! capacity, operating environments and declared operational budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Safety
//!
//! Safe Rust only.
//! No unsafe blocks.
//! No raw pointers.
//! No global mutable state.
//! No fabricated execution success.

// =============================================================================
// IMPORTS
// =============================================================================

use core::fmt;

use std::collections::BTreeSet;

use crate::zur::contracts::execution::{
    ExecutionEngine,
    ExecutionFuture,
    ExecutionHandle,
    ExecutionPlanReference,
    ExecutionRequest,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

/// Canonical component identity used by dispatch diagnostics.
pub const COMPONENT: &str = "zur.execution.dispatch";

/// Stable dispatch contract identifier.
pub const DISPATCH_CONTRACT_ID: &str =
    "zamani.zur.execution.dispatch";

/// Dispatch contract major version.
pub const DISPATCH_CONTRACT_MAJOR: u32 = 1;

/// Dispatch contract minor version.
pub const DISPATCH_CONTRACT_MINOR: u32 = 0;

/// Dispatch contract patch version.
pub const DISPATCH_CONTRACT_PATCH: u32 = 0;

// =============================================================================
// ERROR CONSTRUCTION
// =============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
}

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
}

fn unavailable_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::service_unavailable(message)
        .with_component(COMPONENT)
}

fn cancellation_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::cancelled(message)
        .with_component(COMPONENT)
}

// =============================================================================
// EXECUTION AUTHORITY IDENTITY
// =============================================================================

/// Identifies an explicitly supplied execution authority.
///
/// The identifier is open-ended and namespaced.
///
/// It is not a physical device identity, vendor identity,
/// backend identity or computational-domain enumeration.
///
/// Examples:
///
/// - zamani.classical.native
/// - zamani.classical.jit
/// - zamani.quantum.execution
/// - zamani.hdl.execution
/// - zamani.hybrid.execution
/// - zamani.distributed.execution
/// - future.photonics.execution
///
/// These examples are not a closed inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionAuthorityId(String);

impl ExecutionAuthorityId {
    /// Constructs a validated execution-authority identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(invalid_input(
                "execution authority identifier must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(invalid_input(
                "execution authority identifier contains surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(invalid_input(
                "execution authority identifier contains control characters",
            ));
        }

        // Reuse the canonical namespaced reference validator.
        //
        // This does not introduce a second identifier format.
        crate::zur::contracts::execution::ExecutionReference::new(
            value.clone(),
            "dispatch-authority",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical authority identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionAuthorityId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// VALIDATED DISPATCH EVIDENCE
// =============================================================================

/// Evidence returned by the authoritative execution-plan validator.
///
/// This value binds:
///
/// - The exact artifact reference.
/// - The exact execution-plan reference.
/// - The explicitly selected execution authority.
/// - The plan-validation record.
/// - Compatibility-evidence records.
///
/// It does not contain a second execution-plan representation.
///
/// It does not independently authenticate external evidence.
/// Its producer must be an appropriately trusted planning and
/// compatibility authority.
///
/// The runtime security subsystem remains responsible for
/// authentication and authorization.
///
/// Resource reservation remains the responsibility of the
/// canonical resource-management authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchPlanEvidence {
    artifact: crate::zur::contracts::execution::ArtifactReference,

    plan: ExecutionPlanReference,

    execution_authority: ExecutionAuthorityId,

    validation_record:
        crate::zur::contracts::execution::ExecutionReference,

    compatibility_evidence:
        Vec<crate::zur::contracts::execution::ExecutionReference>,
}

impl DispatchPlanEvidence {
    /// Creates dispatch evidence from records supplied by the
    /// authoritative planning and compatibility subsystem.
    ///
    /// At least one compatibility-evidence reference is required.
    ///
    /// Duplicate compatibility references are rejected.
    ///
    /// This constructor validates structural consistency only.
    /// It does not authenticate evidence or independently establish
    /// plan feasibility.
    pub fn new(
        artifact:
            crate::zur::contracts::execution::ArtifactReference,

        plan: ExecutionPlanReference,

        execution_authority: ExecutionAuthorityId,

        validation_record:
            crate::zur::contracts::execution::ExecutionReference,

        compatibility_evidence:
            Vec<crate::zur::contracts::execution::ExecutionReference>,
    ) -> ZurResult<Self> {
        if compatibility_evidence.is_empty() {
            return Err(invalid_input(
                "dispatch evidence requires compatibility evidence",
            ));
        }

        let mut seen = BTreeSet::new();

        for reference in &compatibility_evidence {
            if !seen.insert(reference) {
                return Err(invalid_input(
                    "dispatch evidence contains duplicate compatibility references",
                ));
            }
        }

        Ok(Self {
            artifact,
            plan,
            execution_authority,
            validation_record,
            compatibility_evidence,
        })
    }

    /// Returns the canonical executable artifact reference.
    #[must_use]
    pub fn artifact(&self)
        -> &crate::zur::contracts::execution::ArtifactReference
    {
        &self.artifact
    }

    /// Returns the canonical execution-plan reference.
    #[must_use]
    pub fn plan(&self) -> &ExecutionPlanReference {
        &self.plan
    }

    /// Returns the explicitly selected execution authority.
    #[must_use]
    pub fn execution_authority(&self) -> &ExecutionAuthorityId {
        &self.execution_authority
    }

    /// Returns the plan-validation evidence reference.
    #[must_use]
    pub fn validation_record(
        &self,
    ) -> &crate::zur::contracts::execution::ExecutionReference {
        &self.validation_record
    }

    /// Returns compatibility-evidence references.
    #[must_use]
    pub fn compatibility_evidence(
        &self,
    ) -> &[crate::zur::contracts::execution::ExecutionReference] {
        &self.compatibility_evidence
    }
}

// =============================================================================
// PLAN VALIDATION AUTHORITY
// =============================================================================

/// Authoritative interface for validating a dispatchable execution plan.
///
/// The implementation must delegate to the canonical owner of the
/// supplied plan and its associated compatibility information.
///
/// Depending on the plan, this may be:
///
/// - Compiler planning.
/// - Hybrid planning.
/// - Distributed planning.
/// - Domain-specific planning.
///
/// The implementation must validate the exact artifact and plan
/// references supplied by the execution request.
///
/// It must also establish that the requested execution authority
/// is the authority associated with the validated plan.
///
/// The implementation must not treat:
///
/// - A registered provider as necessarily available.
/// - A capability declaration as a resource reservation.
/// - A preference as a mandatory requirement.
/// - A hint as a constraint.
/// - Feasibility as successful execution.
///
/// This interface does not perform provider selection.
/// It validates the route already established by the owning
/// planning/composition authority.
pub trait DispatchPlanAuthority: Send + Sync {
    /// Validates the supplied request and returns bound evidence.
    ///
    /// The returned evidence must identify the exact artifact,
    /// execution plan and execution authority.
    ///
    /// Implementations must return an error if:
    ///
    /// - The plan cannot be resolved.
    /// - The plan is invalid or stale.
    /// - The artifact binding is inconsistent.
    /// - Required compatibility evidence is unavailable.
    /// - The execution authority is not the planned authority.
    /// - Required planning validation has not completed.
    ///
    /// This method does not establish physical resource reservation
    /// unless the canonical resource authority separately supplies
    /// that evidence.
    fn validate(
        &self,
        request: &ExecutionRequest<'_>,
        execution_authority: &ExecutionAuthorityId,
    ) -> ZurResult<DispatchPlanEvidence>;
}

// =============================================================================
// EXECUTION AUTHORITY BINDING
// =============================================================================

/// An explicitly supplied execution authority.
///
/// This type binds an authority identity to an existing canonical
/// `ExecutionEngine` implementation.
///
/// It does not discover or select an engine.
///
/// The runtime builder or another trusted composition authority
/// supplies this binding after resolving the validated execution plan.
///
/// The engine remains responsible for actual execution submission,
/// observation and cancellation.
pub struct ExecutionAuthority<'a> {
    id: ExecutionAuthorityId,
    engine: &'a dyn ExecutionEngine,
}

impl<'a> ExecutionAuthority<'a> {
    /// Creates an explicit execution-authority binding.
    pub fn new(
        id: ExecutionAuthorityId,
        engine: &'a dyn ExecutionEngine,
    ) -> Self {
        Self {
            id,
            engine,
        }
    }

    /// Returns the authority identity.
    #[must_use]
    pub fn id(&self) -> &ExecutionAuthorityId {
        &self.id
    }

    /// Returns the bound canonical execution engine.
    ///
    /// Advanced integrations may use this reference to compose
    /// execution operations through the existing execution contract.
    #[must_use]
    pub fn engine(&self) -> &dyn ExecutionEngine {
        self.engine
    }
}

impl fmt::Debug for ExecutionAuthority<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutionAuthority")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

// =============================================================================
// DISPATCH RECEIPT
// =============================================================================

/// Receipt for an execution submission explicitly accepted by
/// the canonical execution authority.
///
/// The receipt establishes submission acceptance only.
///
/// It does not establish:
///
/// - Execution start.
/// - Successful execution.
/// - Successful completion.
/// - Resource retention.
/// - Successful quantum measurement.
/// - Successful hardware programming.
/// - Successful distributed execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchReceipt {
    handle: ExecutionHandle,

    evidence: DispatchPlanEvidence,
}

impl DispatchReceipt {
    /// Creates a receipt after verifying the accepted execution handle.
    ///
    /// The handle must identify the exact artifact, execution plan
    /// and execution identity associated with the original request.
    fn new(
        handle: ExecutionHandle,
        evidence: DispatchPlanEvidence,
        execution_id:
            &crate::zur::ids::ExecutionId,
        artifact:
            &crate::zur::contracts::execution::ArtifactReference,
        plan: &ExecutionPlanReference,
    ) -> ZurResult<Self> {
        if handle.execution_id() != execution_id {
            return Err(compatibility_error(
                "execution authority returned a handle for a different execution",
            )
            .with_execution(execution_id.to_string()));
        }

        if handle.artifact() != artifact {
            return Err(compatibility_error(
                "execution authority returned a handle for a different artifact",
            )
            .with_execution(execution_id.to_string()));
        }

        if handle.plan() != plan {
            return Err(compatibility_error(
                "execution authority returned a handle for a different execution plan",
            )
            .with_execution(execution_id.to_string()));
        }

        Ok(Self {
            handle,
            evidence,
        })
    }

    /// Returns the accepted canonical execution handle.
    #[must_use]
    pub fn handle(&self) -> &ExecutionHandle {
        &self.handle
    }

    /// Returns the dispatch evidence associated with this submission.
    #[must_use]
    pub fn evidence(&self) -> &DispatchPlanEvidence {
        &self.evidence
    }

    /// Returns the canonical execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &crate::zur::ids::ExecutionId {
        self.handle.execution_id()
    }

    /// Returns the planned execution authority.
    #[must_use]
    pub fn execution_authority(&self) -> &ExecutionAuthorityId {
        self.evidence.execution_authority()
    }

    /// Returns the opaque provider-side correlation reference, if supplied.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.handle.provider_reference()
    }
}

// =============================================================================
// DISPATCH COORDINATOR
// =============================================================================

/// Coordinates validated execution dispatch.
///
/// This component is stateless.
///
/// It does not maintain a global provider registry or execution queue.
///
/// The plan authority and execution authority are explicitly injected.
///
/// Multiple coordinators may operate concurrently when their
/// supplied dependencies support concurrent access.
pub struct DispatchCoordinator<'a> {
    plan_authority: &'a dyn DispatchPlanAuthority,
}

impl<'a> DispatchCoordinator<'a> {
    /// Creates a dispatch coordinator with an explicit plan authority.
    #[must_use]
    pub const fn new(
        plan_authority: &'a dyn DispatchPlanAuthority,
    ) -> Self {
        Self {
            plan_authority,
        }
    }

    /// Dispatches an execution request to an explicitly supplied
    /// execution authority.
    ///
    /// Processing order:
    ///
    /// 1. Check request cancellation.
    /// 2. Validate the exact execution plan and its compatibility evidence.
    /// 3. Verify artifact identity.
    /// 4. Verify plan identity.
    /// 5. Verify execution-authority identity.
    /// 6. Submit through the canonical ExecutionEngine.
    /// 7. Verify the accepted execution handle.
    /// 8. Return a structured dispatch receipt.
    ///
    /// The method never searches for another provider or silently
    /// changes the selected execution authority.
    ///
    /// If submission is rejected or indeterminate, the engine's
    /// structured error is propagated.
    ///
    /// If the execution authority returns a mismatched handle,
    /// dispatch returns a compatibility error. The caller must
    /// treat this as an authority-contract violation and investigate
    /// possible external execution side effects.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - Cancellation was requested.
    /// - Plan validation fails.
    /// - Compatibility evidence is inconsistent.
    /// - Artifact identity differs.
    /// - Plan identity differs.
    /// - Execution authority differs.
    /// - Submission fails.
    /// - The returned execution handle is inconsistent.
    pub fn dispatch<'b>(
        &'b self,
        request: ExecutionRequest<'b>,
        authority: &'b ExecutionAuthority<'b>,
    ) -> ExecutionFuture<'b, DispatchReceipt> {
        Box::pin(async move {
            // ----------------------------------------------------------------
            // 1. Check cancellation before any dispatch work.
            // ----------------------------------------------------------------

            if request.context().is_cancelled() {
                return Err(cancellation_error(
                    "execution was cancelled before dispatch",
                )
                .with_execution(
                    request.execution_id().to_string(),
                ));
            }

            // Preserve request identity before transferring ownership
            // of the request to the canonical execution engine.
            let execution_id =
                request.execution_id().clone();

            let artifact =
                request.artifact().clone();

            let plan =
                request.plan().clone();

            // ----------------------------------------------------------------
            // 2. Validate the exact plan through its owner.
            // ----------------------------------------------------------------

            let evidence = self.plan_authority.validate(
                &request,
                authority.id(),
            )?;

            // ----------------------------------------------------------------
            // 3. Verify artifact binding.
            // ----------------------------------------------------------------

            if evidence.artifact() != &artifact {
                return Err(compatibility_error(
                    "validated dispatch evidence identifies a different artifact",
                )
                .with_execution(execution_id.to_string()));
            }

            // ----------------------------------------------------------------
            // 4. Verify plan binding.
            // ----------------------------------------------------------------

            if evidence.plan() != &plan {
                return Err(compatibility_error(
                    "validated dispatch evidence identifies a different execution plan",
                )
                .with_execution(execution_id.to_string()));
            }

            // ----------------------------------------------------------------
            // 5. Verify the explicitly supplied authority.
            // ----------------------------------------------------------------

            if evidence.execution_authority() != authority.id() {
                return Err(compatibility_error(
                    "validated dispatch evidence identifies a different execution authority",
                )
                .with_execution(execution_id.to_string()));
            }

            // ----------------------------------------------------------------
            // 6. Recheck cancellation immediately before submission.
            // ----------------------------------------------------------------

            if request.context().is_cancelled() {
                return Err(cancellation_error(
                    "execution was cancelled before authority submission",
                )
                .with_execution(execution_id.to_string()));
            }

            // ----------------------------------------------------------------
            // 7. Transfer the request to the canonical execution engine.
            // ----------------------------------------------------------------

            let handle = authority.engine.submit(request).await?;

            // ----------------------------------------------------------------
            // 8. Validate actual submission identity.
            // ----------------------------------------------------------------

            DispatchReceipt::new(
                handle,
                evidence,
                &execution_id,
                &artifact,
                &plan,
            )
        })
    }
}

// =============================================================================
// DEBUG IMPLEMENTATION
// =============================================================================

impl fmt::Debug for DispatchCoordinator<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("DispatchCoordinator")
            .field(
                "plan_authority",
                &"explicitly injected",
            )
            .finish()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::contracts::execution::{
        ArtifactReference,
        ExecutionPlanReference,
        ExecutionReference,
    };

    fn test_authority_id() -> ExecutionAuthorityId {
        ExecutionAuthorityId::new(
            "zamani.classical.native",
        )
        .expect("valid authority identity")
    }

    fn test_artifact() -> ArtifactReference {
        ArtifactReference::new(
            "zamani.compiler.artifacts",
            "artifact-001",
        )
        .expect("valid artifact reference")
    }

    fn test_plan() -> ExecutionPlanReference {
        ExecutionPlanReference::new(
            "zamani.compiler.planning",
            "plan-001",
        )
        .expect("valid plan reference")
    }

    fn test_validation_record() -> ExecutionReference {
        ExecutionReference::new(
            "zamani.compiler.validation",
            "validation-001",
        )
        .expect("valid validation reference")
    }

    fn test_compatibility_evidence() -> Vec<ExecutionReference> {
        vec![
            ExecutionReference::new(
                "zamani.backend.compatibility",
                "compatibility-001",
            )
            .expect("valid compatibility reference"),
        ]
    }

    #[test]
    fn accepts_open_ended_authority_identity() {
        let authority = ExecutionAuthorityId::new(
            "future.photonics.execution",
        )
        .expect("future authority should be supported");

        assert_eq!(
            authority.as_str(),
            "future.photonics.execution",
        );
    }

    #[test]
    fn rejects_empty_authority_identity() {
        assert!(
            ExecutionAuthorityId::new("").is_err()
        );
    }

    #[test]
    fn rejects_unqualified_authority_identity() {
        assert!(
            ExecutionAuthorityId::new("classical").is_err()
        );
    }

    #[test]
    fn rejects_invalid_authority_identity() {
        assert!(
            ExecutionAuthorityId::new(
                "zamani..classical",
            )
            .is_err()
        );

        assert!(
            ExecutionAuthorityId::new(
                "zamani.classical\n",
            )
            .is_err()
        );
    }

    #[test]
    fn creates_valid_dispatch_evidence() {
        let evidence = DispatchPlanEvidence::new(
            test_artifact(),
            test_plan(),
            test_authority_id(),
            test_validation_record(),
            test_compatibility_evidence(),
        )
        .expect("valid dispatch evidence");

        assert_eq!(
            evidence.execution_authority().as_str(),
            "zamani.classical.native",
        );

        assert_eq!(
            evidence.compatibility_evidence().len(),
            1,
        );
    }

    #[test]
    fn rejects_missing_compatibility_evidence() {
        let result = DispatchPlanEvidence::new(
            test_artifact(),
            test_plan(),
            test_authority_id(),
            test_validation_record(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_compatibility_evidence() {
        let reference = ExecutionReference::new(
            "zamani.backend.compatibility",
            "compatibility-001",
        )
        .expect("valid reference");

        let result = DispatchPlanEvidence::new(
            test_artifact(),
            test_plan(),
            test_authority_id(),
            test_validation_record(),
            vec![
                reference.clone(),
                reference,
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn evidence_preserves_canonical_artifact_and_plan() {
        let artifact = test_artifact();
        let plan = test_plan();

        let evidence = DispatchPlanEvidence::new(
            artifact.clone(),
            plan.clone(),
            test_authority_id(),
            test_validation_record(),
            test_compatibility_evidence(),
        )
        .expect("valid evidence");

        assert_eq!(
            evidence.artifact(),
            &artifact,
        );

        assert_eq!(
            evidence.plan(),
            &plan,
        );
    }

    #[test]
    fn dispatch_evidence_is_cloneable_and_immutable() {
        let evidence = DispatchPlanEvidence::new(
            test_artifact(),
            test_plan(),
            test_authority_id(),
            test_validation_record(),
            test_compatibility_evidence(),
        )
        .expect("valid evidence");

        let cloned = evidence.clone();

        assert_eq!(
            evidence,
            cloned,
        );
    }

    #[test]
    fn evidence_supports_future_computational_domains() {
        let authority = ExecutionAuthorityId::new(
            "future.optical.execution",
        )
        .expect("future authority identity");

        assert_eq!(
            authority.as_str(),
            "future.optical.execution",
        );
    }

    #[test]
    fn coordinator_is_explicitly_constructible() {
        struct TestPlanAuthority;

        impl DispatchPlanAuthority for TestPlanAuthority {
            fn validate(
                &self,
                _request: &ExecutionRequest<'_>,
                _execution_authority: &ExecutionAuthorityId,
            ) -> ZurResult<DispatchPlanEvidence> {
                Err(
                    ZurError::unsupported(
                        "test authority does not validate execution plans",
                    )
                    .with_component(COMPONENT),
                )
            }
        }

        let plan_authority = TestPlanAuthority;

        let coordinator = DispatchCoordinator::new(
            &plan_authority,
        );

        // Construction has no global initialization side effects.
        let _ = format!("{coordinator:?}");
    }

    #[test]
    fn canonical_contract_types_are_send_sync_where_required() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ExecutionAuthorityId>();

        assert_send_sync::<DispatchPlanEvidence>();
    }
}
