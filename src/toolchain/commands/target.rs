
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Command
//!
//! Path: `src/toolchain/commands/target.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the command/API boundary for inspecting and validating
//! canonical backend targets, inspecting target-profile scope, and
//! selecting among candidates that have already received authoritative
//! compatibility assessments.
//!
//! This module is an orchestration adapter. It reuses the existing
//! `crate::toolchain::targets` implementations and does not introduce
//! another target-management subsystem.
//!
//! ## Architectural authorities
//!
//! Source-level target intent:
//!     grammar/compile/target.g4
//!
//! Source-level target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! Source-level compilation profiles:
//!     grammar/compile/profiles.g4
//!
//! Backend target identity and descriptors:
//!     crate::backend::target
//!
//! Target registration and discovery:
//!     crate::compiler::planning::target_discovery
//!     crate::compiler::backends::discovery
//!     crate::backend::registry
//!
//! Capability and resource compatibility:
//!     crate::backend::compatibility
//!     crate::toolchain::targets::compatibility
//!
//! Target-profile configuration:
//!     crate::toolchain::targets::profiles
//!
//! Target selection:
//!     crate::toolchain::targets::selection
//!
//! Target validation:
//!     crate::toolchain::targets::validation
//!
//! Multi-target build coordination:
//!     crate::toolchain::targets::matrix
//!
//! Structured toolchain errors:
//!     crate::toolchain::errors
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The target command service boundary.
//! - Deterministic listing of explicitly supplied canonical descriptors.
//! - Exact target inspection.
//! - Target-profile scope inspection.
//! - Delegation to canonical target validation.
//! - Delegation to canonical assessed-candidate selection.
//! - Command-level error context.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Target identifiers or target descriptors.
//! - Target registration.
//! - Target discovery or hardware probing.
//! - Backend registration or backend construction.
//! - Capability declarations or evidence.
//! - Requirement or constraint interpretation.
//! - Compatibility algorithms.
//! - Feasibility algorithms.
//! - Preference scoring or selection algorithms.
//! - Resource accounting or physical allocation.
//! - Compiler target lowering.
//! - Code generation or artifact verification.
//! - Quantum routing, scheduling, or error correction.
//! - HDL synthesis, placement, or timing closure.
//! - Distributed scheduling or deployment.
//! - Runtime execution.
//!
//! ## Semantic invariants
//!
//! The command preserves these distinct concepts:
//!
//! - Requirement: a mandatory condition.
//! - Capability: a declared or evidenced implementation property.
//! - Preference: a non-mandatory ordering preference.
//! - Constraint: a condition an acceptable realization must satisfy.
//! - Hint: non-binding implementation guidance.
//! - Feasibility: an authoritative assessment based on available evidence.
//!
//! The command never weakens a requirement, converts a hint into a
//! requirement, or promotes unknown evidence into positive evidence.
//!
//! Compatibility and selection are not proof of physical allocation,
//! successful compilation, artifact validity, or runtime availability.
//!
//! ## POCO-REAF and scalability
//!
//! Target identities and computational domains are open-world.
//!
//! This module introduces no universal ceilings on:
//!
//! - Targets or providers.
//! - CPUs, cores, or threads.
//! - GPUs, FPGAs, ASICs, or other accelerators.
//! - QPUs, qubits, or quantum resources.
//! - Distributed nodes or participants.
//! - Memory, storage, or network capacity.
//! - Tensor dimensions or rank.
//! - Register widths.
//! - Artifact formats.
//! - Computational domains.
//!
//! Collection sizes are determined by the supplied data and available
//! implementation resources. Operational budgets belong to the
//! configuration and execution authorities.
//!
//! Literal infinite physical resources or execution are not promised.
//!
//! ## Determinism
//!
//! Target listings are sorted by canonical backend target identity.
//! Duplicate canonical identities are rejected.
//!
//! Selection ordering and selection decisions are delegated to the
//! existing deterministic target-selection service.
//!
//! This module does not introduce implicit target ordering or fallback.
//!
//! ## Security
//!
//! This module performs no filesystem, network, environment, process,
//! credential, hardware, or runtime operations.
//!
//! Descriptors and assessment reports must originate from their
//! authoritative registries and providers. This module does not
//! authenticate provider evidence.
//!
//! ## Integration contract
//!
//! 1. Discovery obtains descriptors from the canonical discovery and
//!    registration authorities.
//! 2. The caller supplies those descriptors to `list_targets` or
//!    `inspect_target`.
//! 3. Compatibility assessment is performed by the canonical backend
//!    compatibility evaluator through `targets::compatibility`.
//! 4. Selection consumes only candidates with their matching
//!    authoritative compatibility reports.
//! 5. Validation delegates to `targets::validation`.
//! 6. Multi-target build orchestration consumes the selected targets
//!    and assessments through `targets::matrix`.
//! 7. Compilation, resource acquisition, and execution remain downstream.
//!
//! This command does not claim that a listed or selected target is
//! universally usable.

use std::collections::BTreeMap;

use crate::backend::target::{
    TargetDescriptor as BackendTargetDescriptor,
    TargetId as BackendTargetId,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::targets::profiles::TargetProfileSettings;

use crate::toolchain::targets::selection::{
    select_target as select_assessed_target,
    SelectionOutcome,
    TargetSelectionCandidate,
    TargetSelectionPolicy,
};

use crate::toolchain::targets::validation::{
    validate_target as validate_canonical_target,
    TargetValidationReport,
    TargetValidationRequest,
};

/// Stable architectural identity of this command module.
pub const TARGET_COMMAND_MODULE_NAME: &str =
    "zamani.toolchain.commands.target";

/// Public API version of the target command.
pub const TARGET_COMMAND_API_VERSION: u16 = 1;

/// Operation identifier used for command-level errors.
const OPERATION_LIST: &str = "toolchain.command.target.list";

/// Operation identifier used for target-inspection errors.
const OPERATION_INSPECT: &str = "toolchain.command.target.inspect";

/// Operation identifier used for target-profile inspection.
const OPERATION_PROFILE: &str =
    "toolchain.command.target.profile.inspect";

/// Operation identifier used for target validation.
const OPERATION_VALIDATE: &str =
    "toolchain.command.target.validate";

/// Operation identifier used for target selection.
const OPERATION_SELECT: &str = "toolchain.command.target.select";

/// Stateless target command service.
///
/// The service owns no registry, target inventory, cache, provider,
/// mutable configuration, hardware state, or global state.
///
/// It is safe to construct independently for each command invocation.
/// Its operations are deterministic for the same validated inputs.
#[derive(Debug, Clone, Copy, Default)]
pub struct TargetCommandService;

impl TargetCommandService {
    /// Construct the stateless target command service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// List explicitly supplied canonical backend target descriptors.
    ///
    /// The caller must obtain `targets` from the authoritative backend
    /// or compiler target-discovery integration. This method does not
    /// perform discovery or query a second registry.
    ///
    /// The returned references are sorted by canonical target identity.
    /// The descriptors themselves are not copied or modified.
    ///
    /// A descriptor's presence in this list does not establish that:
    ///
    /// - A compatible emitter is registered.
    /// - Required capabilities are available.
    /// - Required resources are feasible.
    /// - A physical device is currently available.
    /// - Compilation or execution will succeed.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a descriptor is invalid or if
    /// duplicate canonical target identities are supplied.
    pub fn list_targets<'a>(
        &self,
        targets: &'a [BackendTargetDescriptor],
    ) -> ToolchainResult<Vec<&'a BackendTargetDescriptor>> {
        let mut ordered = BTreeMap::new();

        for target in targets {
            target.validate().map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    format!(
                        "target descriptor `{}` failed validation: {error}",
                        target.id(),
                    ),
                )
                .with_operation(OPERATION_LIST)
                .with_subject(target.id().to_string())
                .with_source(error)
            })?;

            let target_id = target.id().as_str();

            if ordered.insert(target_id, target).is_some() {
                return Err(ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!(
                        "duplicate canonical target descriptor `{target_id}`",
                    ),
                )
                .with_operation(OPERATION_LIST)
                .with_subject(target_id.to_owned()));
            }
        }

        Ok(ordered.into_values().collect())
    }

    /// Inspect one target by its exact canonical backend target identity.
    ///
    /// The supplied descriptors must originate from the authoritative
    /// backend/compiler discovery boundary.
    ///
    /// This method performs exact identity matching. It does not resolve
    /// aliases, infer architectures, select a replacement, or silently
    /// fall back to another target.
    ///
    /// The returned descriptor exposes the canonical target metadata
    /// and its declared capability profile. A declaration is not proof
    /// of current capability evidence or physical availability.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is invalid, a supplied
    /// descriptor is invalid, duplicate descriptors exist, or the
    /// requested target is not present in the supplied inventory.
    pub fn inspect_target<'a>(
        &self,
        targets: &'a [BackendTargetDescriptor],
        target_id: &str,
    ) -> ToolchainResult<&'a BackendTargetDescriptor> {
        let canonical_id =
            BackendTargetId::new(target_id.to_owned()).map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::InvalidInput,
                    format!("invalid target identifier `{target_id}`: {error}"),
                )
                .with_operation(OPERATION_INSPECT)
                .with_subject(target_id.to_owned())
                .with_source(error)
            })?;

        let ordered = self.list_targets(targets)?;

        ordered
            .into_iter()
            .find(|target| target.id() == &canonical_id)
            .ok_or_else(|| {
                ToolchainError::new(
                    ToolchainErrorKind::NotFound,
                    format!(
                        "target `{target_id}` was not present in the supplied \
                         canonical target inventory",
                    ),
                )
                .with_operation(OPERATION_INSPECT)
                .with_subject(target_id.to_owned())
            })
    }

    /// Inspect whether a target belongs to an explicit target-profile scope.
    ///
    /// The profile is the existing `TargetProfileSettings` object.
    /// This method does not create or resolve generic profile identities,
    /// perform profile inheritance, or register profiles.
    ///
    /// Returns:
    ///
    /// - `Some(true)` if the profile explicitly references the target.
    /// - `Some(false)` if the profile has an explicit target scope but
    ///   does not reference the target.
    /// - `None` if the profile does not declare a target scope.
    ///
    /// `None` must not be interpreted as universal target support.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the target identifier is invalid
    /// under the canonical compiler target-ID contract.
    pub fn inspect_profile_scope(
        &self,
        profile: &TargetProfileSettings,
        target_id: &str,
    ) -> ToolchainResult<Option<bool>> {
        profile.references_target(target_id).map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!(
                    "could not inspect target-profile scope for `{target_id}`: \
                     {error}",
                ),
            )
            .with_operation(OPERATION_PROFILE)
            .with_subject(target_id.to_owned())
            .with_source(error)
        })
    }

    /// Validate a target using the canonical target-validation service.
    ///
    /// The request must contain the canonical backend descriptor,
    /// compatibility request, and target-scoped capability/resource
    /// evidence. Emitter-registration evidence must only be supplied
    /// when it has been obtained from the canonical backend registry.
    ///
    /// This method does not manufacture missing evidence.
    ///
    /// The returned report distinguishes:
    ///
    /// - `Ready`: all checks requested by this validation request passed.
    /// - `Rejected`: available evidence establishes a failed condition.
    /// - `Unresolved`: available evidence is insufficient to establish
    ///   readiness.
    ///
    /// Even `Ready` does not prove compilation, resource reservation,
    /// artifact verification, or runtime execution.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error if the authoritative
    /// validation service cannot evaluate the request.
    pub fn validate_target(
        &self,
        request: &TargetValidationRequest<'_>,
    ) -> ToolchainResult<TargetValidationReport> {
        validate_canonical_target(request).map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!("canonical target validation failed: {error}"),
            )
            .with_operation(OPERATION_VALIDATE)
            .with_subject(request.target().id().to_string())
            .with_source(error)
        })
    }

    /// Select one candidate from explicitly supplied, assessed targets.
    ///
    /// Each candidate must already contain:
    ///
    /// - A validated canonical backend target descriptor.
    /// - The authoritative compatibility report for that exact target.
    ///
    /// The existing selection service applies exclusions, explicit
    /// profile scope, compatibility status, preferences, fallback order,
    /// and deterministic canonical identity ordering.
    ///
    /// Requirements and constraints must already have been evaluated
    /// by the authoritative compatibility/planning layers. This method
    /// cannot weaken them.
    ///
    /// An indeterminate candidate is never selected.
    ///
    /// A missing preferred or fallback target remains unresolved in
    /// the returned selection outcome. It is not silently treated as
    /// available.
    ///
    /// An outcome with no selected target is a valid structured result.
    /// The caller must inspect `has_selection()` before proceeding.
    ///
    /// # Errors
    ///
    /// Returns a structured error for invalid policy data, duplicate
    /// candidates, invalid descriptors, mismatched target/report
    /// identities, or invalid profile references.
    pub fn select_target<'a>(
        &self,
        candidates: &'a [TargetSelectionCandidate<'a>],
        policy: &TargetSelectionPolicy,
        profile: Option<&TargetProfileSettings>,
    ) -> ToolchainResult<SelectionOutcome<'a>> {
        select_assessed_target(candidates, policy, profile).map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                format!("canonical target selection failed: {error}"),
            )
            .with_operation(OPERATION_SELECT)
            .with_source(error)
        })
    }
}

/// List explicitly supplied canonical backend target descriptors.
///
/// This is a convenience function with the same semantics as
/// `TargetCommandService::list_targets`.
///
/// The caller remains responsible for obtaining descriptors from the
/// canonical discovery/registration authority.
pub fn list_targets<'a>(
    targets: &'a [BackendTargetDescriptor],
) -> ToolchainResult<Vec<&'a BackendTargetDescriptor>> {
    TargetCommandService::new().list_targets(targets)
}

/// Inspect a target by exact canonical backend target identity.
///
/// This is a convenience function with the same semantics as
/// `TargetCommandService::inspect_target`.
pub fn inspect_target<'a>(
    targets: &'a [BackendTargetDescriptor],
    target_id: &str,
) -> ToolchainResult<&'a BackendTargetDescriptor> {
    TargetCommandService::new().inspect_target(targets, target_id)
}

/// Inspect a target's membership in an existing target-profile scope.
///
/// This is a convenience function with the same semantics as
/// `TargetCommandService::inspect_profile_scope`.
pub fn inspect_profile_scope(
    profile: &TargetProfileSettings,
    target_id: &str,
) -> ToolchainResult<Option<bool>> {
    TargetCommandService::new().inspect_profile_scope(profile, target_id)
}

/// Validate a target through the canonical target-validation service.
///
/// This is a convenience function with the same semantics as
/// `TargetCommandService::validate_target`.
pub fn validate_target(
    request: &TargetValidationRequest<'_>,
) -> ToolchainResult<TargetValidationReport> {
    TargetCommandService::new().validate_target(request)
}

/// Select an assessed target through the canonical selection service.
///
/// This is a convenience function with the same semantics as
/// `TargetCommandService::select_target`.
pub fn select_target<'a>(
    candidates: &'a [TargetSelectionCandidate<'a>],
    policy: &TargetSelectionPolicy,
    profile: Option<&TargetProfileSettings>,
) -> ToolchainResult<SelectionOutcome<'a>> {
    TargetCommandService::new().select_target(candidates, policy, profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::backend::target::ExecutionModel;

    fn descriptor(
        target_id: &str,
        display_name: &str,
    ) -> BackendTargetDescriptor {
        BackendTargetDescriptor::from_name(
            target_id,
            display_name,
            ExecutionModel::Other("test.execution-model".to_owned()),
        )
        .expect("test descriptor must be valid")
    }

    #[test]
    fn command_identity_is_stable() {
        assert_eq!(
            TARGET_COMMAND_MODULE_NAME,
            "zamani.toolchain.commands.target",
        );

        assert_ne!(TARGET_COMMAND_API_VERSION, 0);
    }

    #[test]
    fn service_is_stateless_and_constructible() {
        let _service = TargetCommandService::new();
    }

    #[test]
    fn empty_inventory_returns_an_empty_listing() {
        let targets = [];

        let listed = list_targets(&targets)
            .expect("an empty discovered inventory is valid");

        assert!(listed.is_empty());
    }

    #[test]
    fn listing_is_sorted_by_canonical_target_identity() {
        let targets = [
            descriptor("vendor:zeta", "Zeta"),
            descriptor("vendor:alpha", "Alpha"),
            descriptor("vendor:middle", "Middle"),
        ];

        let listed = list_targets(&targets)
            .expect("valid unique descriptors should list");

        let identifiers: Vec<&str> = listed
            .iter()
            .map(|target| target.id().as_str())
            .collect();

        assert_eq!(
            identifiers,
            vec![
                "vendor:alpha",
                "vendor:middle",
                "vendor:zeta",
            ],
        );
    }

    #[test]
    fn duplicate_canonical_target_identities_are_rejected() {
        let targets = [
            descriptor("vendor:duplicate", "First"),
            descriptor("vendor:duplicate", "Second"),
        ];

        let error = list_targets(&targets)
            .expect_err("duplicate target identities must be rejected");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::AlreadyExists,
        );
    }

    #[test]
    fn inspection_uses_exact_canonical_identity() {
        let targets = [
            descriptor("vendor:target-a", "Target A"),
            descriptor("vendor:target-b", "Target B"),
        ];

        let target = inspect_target(&targets, "vendor:target-b")
            .expect("exact target should be found");

        assert_eq!(target.id().as_str(), "vendor:target-b");
    }

    #[test]
    fn inspection_does_not_fall_back_to_another_target() {
        let targets = [
            descriptor("vendor:target-a", "Target A"),
        ];

        let error = inspect_target(&targets, "vendor:missing")
            .expect_err("missing target must not silently fall back");

        assert_eq!(error.kind(), ToolchainErrorKind::NotFound);
    }

    #[test]
    fn invalid_target_identity_is_rejected() {
        let targets = [
            descriptor("vendor:target-a", "Target A"),
        ];

        assert!(
            inspect_target(&targets, "invalid target").is_err()
        );
    }

    #[test]
    fn profile_scope_does_not_claim_universal_support() {
        let profile = TargetProfileSettings::new();

        let scope = inspect_profile_scope(
            &profile,
            "vendor:future-target",
        )
        .expect("valid target identity should be inspectable");

        assert_eq!(scope, None);
    }

    #[test]
    fn explicit_profile_scope_is_distinguished_from_unspecified_scope() {
        let mut profile = TargetProfileSettings::new();

        profile
            .add_target_reference("vendor:target-a")
            .expect("target reference should be valid");

        assert_eq!(
            inspect_profile_scope(&profile, "vendor:target-a")
                .expect("scope inspection should succeed"),
            Some(true),
        );

        assert_eq!(
            inspect_profile_scope(&profile, "vendor:target-b")
                .expect("scope inspection should succeed"),
            Some(false),
        );
    }
}
