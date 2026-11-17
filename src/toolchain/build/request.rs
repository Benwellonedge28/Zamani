
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Validated Build Request
//!
//! Path: `src/toolchain/build/request.rs`
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the immutable build-layer request accepted by the toolchain
//! build subsystem.
//!
//! The public API request is defined by:
//!
//!     crate::toolchain::api::request::BuildRequest
//!
//! This module does not introduce a competing request schema. It converts
//! the public API request into an immutable, structurally validated value
//! that build planning and execution can consume.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The validated build-request boundary.
//! - Immutable access to accepted build intent.
//! - Revalidation before build planning.
//! - Conversion from the public API request.
//!
//! It does not own:
//!
//! - Project or workspace discovery.
//! - Manifest parsing or validation.
//! - Dependency resolution.
//! - Semantic requirements or capability analysis.
//! - Resource-intent interpretation.
//! - Target discovery or target registration.
//! - Target feasibility assessment.
//! - Compiler configuration or compilation.
//! - Backend selection or code generation.
//! - Physical resource discovery or allocation.
//! - Build-task graph construction.
//! - Build execution or scheduling.
//! - Artifact construction or verification.
//! - Runtime execution.
//!
//! These responsibilities remain with their canonical subsystems.
//!
//! ## POCO-REAF
//!
//! One logical build request may produce multiple compatible
//! target-specific realizations.
//!
//! Target selection is intent, not proof of feasibility.
//!
//! The build planner must obtain authoritative compatibility and
//! feasibility evidence from the compiler, target, backend, hardware,
//! and other relevant provider authorities.
//!
//! This module imposes no universal limits on:
//!
//! - Projects or workspace members.
//! - Features or dependencies.
//! - Target selectors.
//! - Computing domains.
//! - CPUs, GPUs, FPGAs, ASICs or QPUs.
//! - Quantum resources.
//! - Memory, storage or tensor dimensions.
//! - Distributed participants.
//!
//! Actual processing is subject to available resources, Rust's
//! representational constraints, and explicitly configured operational
//! budgets. Literal infinite execution or storage is not promised.
//!
//! ## Semantic boundary
//!
//! Requirements, capabilities, preferences, constraints and hints
//! describing program meaning belong to the canonical semantic and
//! resource-intent authorities.
//!
//! Feasibility is an evidence-based downstream assessment.
//!
//! This request must not copy, reinterpret, promote, weaken or replace
//! those semantic declarations.
//!
//! ## Fingerprints and identity
//!
//! Dependency and environment fingerprints are established by their
//! respective resolution and discovery services. They are not accepted
//! as trusted caller-supplied facts here.
//!
//! A BuildId identifies an accepted build operation and is allocated
//! by the build service. It is not an intrinsic property of caller intent.
//!
//! ## Validation
//!
//! Conversion validates the complete public API request.
//!
//! Structural validation does not prove that:
//!
//! - The project exists.
//! - The manifest is valid.
//! - Dependencies can be resolved.
//! - A profile exists.
//! - Requested features are available.
//! - A target is registered or compatible.
//! - Required resources are available.
//! - Compilation will succeed.
//! - An artifact can be executed.
//!
//! Those checks belong to the services that own the corresponding facts.
//!
//! ## Integration
//!
//! Input:
//!     crate::toolchain::api::BuildRequest
//!
//! Output:
//!     crate::toolchain::build::request::BuildRequest
//!
//! Consumers:
//!     build::plan
//!     build::graph
//!     build::executor
//!
//! This module is deliberately independent of those future consumers.
//!
//! ## Serialization
//!
//! This validated internal type is not a wire-format type.
//! Serialization and deserialization remain owned by the public API
//! request contract. A deserialized request must be converted through
//! TryFrom before build planning.

use std::collections::BTreeSet;

use crate::toolchain::api::{
    BuildRequest as ApiBuildRequest,
    BudgetKey,
    OperationalBudgets,
    ProjectLocator,
    ProjectScope,
    RequestValidationError,
    RequestValidationResult,
    TargetSelection,
    TargetSelector,
};

use crate::toolchain::ids::ProfileId;

/// An immutable, structurally validated build request.
///
/// This type is distinct from `toolchain::api::BuildRequest`.
///
/// The API request represents externally supplied caller intent.
/// This type represents that same intent after it has crossed the
/// build subsystem's structural validation boundary.
///
/// Its fields are private. Consumers can inspect the request but cannot
/// mutate it after validation.
///
/// A value of this type does not mean that dependency resolution,
/// target feasibility, compilation, or execution has succeeded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "a validated build request has no effect until submitted to the build service"]
pub struct BuildRequest {
    request: ApiBuildRequest,
}

impl BuildRequest {
    /// Convert and validate a public API build request.
    ///
    /// Validation is repeated deliberately. API requests may have been
    /// deserialized or modified through their public fields after their
    /// original construction.
    ///
    /// This method performs structural validation only. It does not
    /// access the filesystem, resolve dependencies, inspect targets,
    /// or perform compilation.
    ///
    /// # Errors
    ///
    /// Returns the canonical API request-validation error when any
    /// request field violates its structural contract.
    pub fn try_from_api(
        request: ApiBuildRequest,
    ) -> RequestValidationResult<Self> {
        request.validate()?;

        Ok(Self { request })
    }

    /// Revalidate this request.
    ///
    /// The wrapped request is private and cannot be mutated through
    /// this type's public API. Revalidation is nevertheless available
    /// for explicit trust-boundary checks and future integration.
    ///
    /// # Errors
    ///
    /// Returns the canonical request-validation error if the wrapped
    /// request does not satisfy its structural contract.
    pub fn validate(&self) -> RequestValidationResult<()> {
        self.request.validate()
    }

    /// Borrow the original project and workspace scope.
    #[must_use]
    pub fn scope(&self) -> &ProjectScope {
        &self.request.scope
    }

    /// Borrow the requested target-selection intent.
    ///
    /// This value is not evidence that any requested target is
    /// registered, compatible, available or feasible.
    #[must_use]
    pub fn targets(&self) -> &TargetSelection {
        &self.request.targets
    }

    /// Borrow the optional toolchain profile identity.
    ///
    /// Profile existence and profile resolution belong to the
    /// configuration and profile services.
    #[must_use]
    pub fn profile(&self) -> Option<&ProfileId> {
        self.request.profile.as_ref()
    }

    /// Borrow the deterministically ordered feature selection.
    ///
    /// Feature availability and feature-unification rules belong
    /// to manifest and dependency resolution services.
    #[must_use]
    pub fn features(&self) -> &BTreeSet<String> {
        &self.request.features
    }

    /// Borrow the requested operational budgets.
    ///
    /// Budget interpretation and enforcement belong to the
    /// configuration, planning and execution services.
    #[must_use]
    pub fn budgets(&self) -> &OperationalBudgets {
        &self.request.budgets
    }

    /// Borrow the original validated public API request.
    ///
    /// The returned reference is immutable. It does not expose
    /// a mutable route around this module's validation boundary.
    #[must_use]
    pub fn as_api_request(&self) -> &ApiBuildRequest {
        &self.request
    }
}

impl TryFrom<ApiBuildRequest> for BuildRequest {
    type Error = RequestValidationError;

    /// Convert a public API request into an immutable build request.
    ///
    /// This delegates to `BuildRequest::try_from_api`, ensuring that
    /// both conversion entry points enforce the same validation rules.
    fn try_from(
        request: ApiBuildRequest,
    ) -> Result<Self, Self::Error> {
        Self::try_from_api(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    use std::collections::BTreeSet;

    fn project_scope() -> ProjectScope {
        let locator = ProjectLocator::from_path(
            "examples/minimal",
        )
        .expect("a non-empty project path is structurally valid");

        ProjectScope::new(locator)
            .expect("valid project scope")
    }

    fn valid_api_request() -> ApiBuildRequest {
        let targets = TargetSelection::resolve_from_capabilities(
            Vec::new(),
        )
        .expect("empty preferences permit policy-driven resolution");

        ApiBuildRequest::new(project_scope(), targets)
            .expect("valid API build request")
    }

    #[test]
    fn accepts_a_valid_api_request() {
        let api_request = valid_api_request();

        let request = BuildRequest::try_from_api(api_request)
            .expect("valid request must be accepted");

        assert!(request.validate().is_ok());
    }

    #[test]
    fn try_from_uses_the_same_validation_boundary() {
        let api_request = valid_api_request();

        let request = BuildRequest::try_from(api_request)
            .expect("valid request must be accepted");

        assert!(request.validate().is_ok());
    }

    #[test]
    fn preserves_project_scope() {
        let api_request = valid_api_request();

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(
            request.scope().project,
            ProjectLocator::from_path("examples/minimal")
                .expect("valid path"),
        );
    }

    #[test]
    fn permits_capability_driven_target_resolution() {
        let api_request = valid_api_request();

        let request = BuildRequest::try_from(api_request)
            .expect("capability-driven request is valid");

        assert!(matches!(
            request.targets(),
            TargetSelection::ResolveFromCapabilities {
                preferences
            } if preferences.is_empty()
        ));
    }

    #[test]
    fn preserves_explicit_target_intent() {
        let target = TargetSelector::new(
            "portable",
        )
        .expect("valid target selector");

        let targets = TargetSelection::explicit(
            vec![target.clone()],
            true,
        )
        .expect("valid explicit target selection");

        let api_request = ApiBuildRequest::new(
            project_scope(),
            targets,
        )
        .expect("valid API request");

        let request = BuildRequest::try_from(api_request)
            .expect("valid build request");

        assert!(matches!(
            request.targets(),
            TargetSelection::Explicit {
                targets,
                require_all: true,
            } if targets == &vec![target]
        ));
    }

    #[test]
    fn preserves_profile_selection() {
        let profile = ProfileId::new(
            IdNamespace::new("test").expect("valid namespace"),
            7,
        )
        .expect("valid profile identity");

        let api_request = valid_api_request()
            .with_profile(profile.clone());

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(request.profile(), Some(&profile));
    }

    #[test]
    fn preserves_feature_selection() {
        let features = BTreeSet::from([
            "classical.parallel".to_owned(),
            "quantum.simulation".to_owned(),
        ]);

        let api_request = valid_api_request()
            .with_features(features.clone())
            .expect("valid features");

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(request.features(), &features);
    }

    #[test]
    fn preserves_open_ended_operational_budgets() {
        let mut budgets = OperationalBudgets::new();

        let key = BudgetKey::new(
            "build.wall_time_ms",
        )
        .expect("valid namespaced budget key");

        budgets.set(key.clone(), 30_000);

        let api_request = valid_api_request()
            .with_budgets(budgets)
            .expect("valid budgets");

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(
            request.budgets().get(&key),
            Some(30_000),
        );
    }

    #[test]
    fn zero_budget_is_not_reinterpreted_as_unlimited() {
        let mut budgets = OperationalBudgets::new();

        let key = BudgetKey::new(
            "build.wall_time_ms",
        )
        .expect("valid budget key");

        budgets.set(key.clone(), 0);

        let api_request = valid_api_request()
            .with_budgets(budgets)
            .expect("zero is a valid explicit budget");

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(
            request.budgets().get(&key),
            Some(0),
        );
    }

    #[test]
    fn rejects_invalid_request_after_public_field_mutation() {
        let mut api_request = valid_api_request();

        // Public API fields may be changed after construction.
        // The build boundary must not trust the original constructor.
        api_request.features.insert("  ".to_owned());

        assert!(
            BuildRequest::try_from_api(api_request).is_err()
        );
    }

    #[test]
    fn rejects_empty_explicit_target_selection() {
        let mut api_request = valid_api_request();

        api_request.targets = TargetSelection::Explicit {
            targets: Vec::new(),
            require_all: true,
        };

        assert!(
            BuildRequest::try_from_api(api_request).is_err()
        );
    }

    #[test]
    fn validation_does_not_claim_project_existence() {
        // Structural request validation must not perform filesystem
        // discovery or assume that a supplied path exists.
        let locator = ProjectLocator::from_path(
            "path-that-is-not-required-to-exist",
        )
        .expect("non-empty path");

        let scope = ProjectScope::new(locator)
            .expect("valid scope");

        let targets = TargetSelection::resolve_from_capabilities(
            Vec::new(),
        )
        .expect("valid target intent");

        let api_request = ApiBuildRequest::new(
            scope,
            targets,
        )
        .expect("valid API request");

        assert!(
            BuildRequest::try_from(api_request).is_ok()
        );
    }

    #[test]
    fn immutable_wrapper_preserves_original_api_request() {
        let api_request = valid_api_request();

        let expected = api_request.clone();

        let request = BuildRequest::try_from(api_request)
            .expect("valid request");

        assert_eq!(
            request.as_api_request(),
            &expected,
        );
    }
}
