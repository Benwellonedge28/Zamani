
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani ZHAL — HDL Adapter
//!
//! Path: `src/zhal/adapters/hdl.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the HDL-domain integration boundary for ZHAL.
//!
//! This adapter connects the universal hardware-abstraction architecture
//! to the existing HDL realization provider contract. It deliberately
//! delegates realization to `crate::hdl::realization::interface`.
//!
//! The adapter does not implement synthesis, technology mapping, placement,
//! routing, timing closure, physical allocation, deployment, or HDLIR.
//!
//! ## Existing authorities reused
//!
//! - `crate::hdl::realization::plan`
//! - `crate::hdl::realization::interface`
//! - `crate::hdl::realization::reports`
//! - `crate::hdl::targets`
//! - `crate::hdl::requirements`
//! - `crate::hdl::resources`
//! - `crate::hdl::hdlir`
//!
//! ## Ownership boundaries
//!
//! The following responsibilities remain outside this adapter:
//!
//! - Grammar and syntax: `grammar/hdl/`.
//! - Source semantics: canonical frontend semantic model.
//! - Universal computation representation: `src/zuir/`.
//! - HDL representation: `src/hdl/hdlir/`.
//! - HDL target descriptions and feasibility: `src/hdl/targets/`.
//! - HDL realization plans: `src/hdl/realization/plan.rs`.
//! - HDL provider execution: `src/hdl/realization/interface.rs`.
//! - HDL reports: `src/hdl/realization/reports.rs`.
//! - Physical resource allocation and orchestration: ZUR/runtime.
//! - Universal provider lifecycle and identity: standard-library provider
//!   contracts and the future ZHAL provider integration.
//!
//! ## Adapter contract
//!
//! `HdlAdapter` is a lightweight, cloneable handle to an existing
//! `HdlRealizationProvider`.
//!
//! It:
//!
//! - Retains provider ownership through `Arc`.
//! - Exposes the existing provider descriptor.
//! - Delegates realization to the canonical HDL realization coordinator.
//! - Preserves the existing policy and cancellation contracts.
//! - Preserves structured realization errors and reports.
//! - Does not silently reinterpret feasibility or unsupported operations.
//!
//! It does not maintain a second provider registry.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no fixed limits on:
//!
//! - HDL targets.
//! - FPGA or ASIC devices.
//! - Logic elements.
//! - Signals or ports.
//! - Modules or instances.
//! - Memory resources.
//! - Clock domains.
//! - Realization operations.
//! - Providers.
//! - Hardware capacity.
//!
//! The adapter does not allocate hardware or define resource ceilings.
//! Its memory use is limited to its shared provider handle and the data
//! owned by the existing HDL realization contracts.
//!
//! Literal infinite computation or storage is not promised. Actual work
//! remains bounded by available resources and explicitly configured
//! operational budgets.
//!
//! ## Failure semantics
//!
//! The adapter preserves the canonical realization interface's failures.
//! In particular:
//!
//! - Infeasible targets remain infeasible.
//! - Unknown feasibility is not silently upgraded to feasible.
//! - Unsupported required operations are rejected.
//! - Unsupported optional operations follow the supplied policy.
//! - Cancellation remains cooperative.
//! - Provider failures remain provider failures.
//! - Provider-reported success is not independent proof of physical
//!   correctness.
//!
//! ## Integration requirements
//!
//! The ZHAL composition must eventually register:
//!
//! `crate::zhal::adapters::hdl`
//!
//! The future universal ZHAL adapter contract must accept or reference
//! this adapter without replacing `HdlRealizationProvider`.
//!
//! The HDL composition root must continue to register:
//!
//! `crate::hdl::realization`
//!
//! No changes to the existing HDL realization provider, plan, report,
//! target, or HDLIR authorities are required by this file.
//!
//! ## Freeze criteria
//!
//! - Uses the existing HDL provider contract.
//! - Uses the existing HDL plan and report contracts.
//! - Does not define a competing provider registry.
//! - Does not define another HDL execution engine.
//! - Does not duplicate target or feasibility models.
//! - Does not perform physical resource allocation.
//! - Preserves cancellation and error semantics.
//! - Contains no unsafe Rust.
//! - Introduces no arbitrary hardware ceilings.
//! - Passes formatting, compilation, tests and Clippy in the actual
//!   repository after the ZHAL module hierarchy is registered.

use std::sync::Arc;

use crate::hdl::realization::interface::{
    realize as realize_hdl_plan,
    HdlProviderDescriptor,
    HdlRealizationContext,
    HdlRealizationInterfaceError,
    HdlRealizationPolicy,
    HdlRealizationProvider,
};
use crate::hdl::realization::plan::HdlRealizationPlan;
use crate::hdl::realization::reports::HdlRealizationReport;

/// Stable architectural name of the ZHAL HDL adapter.
pub const ZHAL_HDL_ADAPTER_NAME: &str = "zamani.zhal.adapters.hdl";

/// Version of this adapter's public contract.
///
/// This is independent of:
///
/// - The Zamani language version.
/// - The grammar version.
/// - The HDLIR schema version.
/// - The HDL target-description schema.
/// - The HDL realization-plan schema.
/// - The provider's own version.
/// - The universal ZHAL API version.
pub const ZHAL_HDL_ADAPTER_VERSION: u16 = 1;

/// A shared handle to an existing HDL realization provider.
///
/// This adapter does not own or duplicate provider lifecycle management.
/// The provider remains the implementation authority defined by
/// `crate::hdl::realization::interface::HdlRealizationProvider`.
///
/// Cloning this adapter clones the `Arc`, not the provider implementation.
///
/// The provider must satisfy the existing `Send + Sync` contract.
#[derive(Clone)]
pub struct HdlAdapter {
    provider: Arc<dyn HdlRealizationProvider>,
}

impl HdlAdapter {
    /// Construct an adapter from an existing HDL realization provider.
    ///
    /// The provider must already expose a valid descriptor through its
    /// canonical provider contract.
    ///
    /// This constructor does not:
    ///
    /// - Initialize the provider.
    /// - Authenticate the provider.
    /// - Discover physical hardware.
    /// - Verify physical device availability.
    /// - Reserve resources.
    /// - Assess target feasibility.
    ///
    /// Those operations remain with their respective authorities.
    #[must_use]
    pub fn new(provider: Arc<dyn HdlRealizationProvider>) -> Self {
        Self { provider }
    }

    /// Return the existing provider descriptor.
    ///
    /// The returned descriptor is borrowed from the provider. It is not
    /// copied into a competing ZHAL provider model.
    #[must_use]
    pub fn descriptor(&self) -> &HdlProviderDescriptor {
        self.provider.descriptor()
    }

    /// Return a shared reference to the underlying HDL provider.
    ///
    /// This is intended for integration code that must interact with the
    /// canonical HDL provider contract.
    ///
    /// The returned reference does not transfer ownership.
    #[must_use]
    pub fn provider(&self) -> &dyn HdlRealizationProvider {
        self.provider.as_ref()
    }

    /// Return a cloned shared handle to the underlying provider.
    ///
    /// Cloning the handle does not clone, initialize, or shut down the
    /// provider.
    #[must_use]
    pub fn provider_handle(&self) -> Arc<dyn HdlRealizationProvider> {
        Arc::clone(&self.provider)
    }

    /// Submit a declarative HDL realization plan to the canonical
    /// realization coordinator.
    ///
    /// This method delegates to the existing HDL realization interface.
    /// It does not implement a second realization algorithm.
    ///
    /// The supplied policy controls the existing coordinator's handling
    /// of unproven feasibility and unsupported optional operations.
    ///
    /// The supplied context provides cooperative cancellation.
    ///
    /// # Errors
    ///
    /// Returns the canonical HDL realization errors, including:
    ///
    /// - Invalid or incompatible provider interface.
    /// - Infeasible target.
    /// - Unestablished feasibility when policy disallows proceeding.
    /// - Blocking unresolved decisions.
    /// - Unsupported operations.
    /// - Cancellation.
    /// - Provider failure.
    /// - Invalid provider report bookkeeping.
    ///
    /// # Important
    ///
    /// A successful report records the realization coordinator's and
    /// provider's reported outcomes. It does not independently verify
    /// a generated bitstream, ASIC layout, physical timing result, or
    /// deployed hardware.
    pub fn realize(
        &self,
        plan: &HdlRealizationPlan,
        policy: HdlRealizationPolicy,
        context: &HdlRealizationContext,
    ) -> Result<HdlRealizationReport, HdlRealizationInterfaceError> {
        realize_hdl_plan(
            self.provider.as_ref(),
            plan,
            policy,
            context,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hdl::realization::interface::{
        HdlOperationResult,
    };
    use crate::hdl::realization::plan::{
        HdlRealizationOperation,
    };
    use crate::hdl::targets::model::{
        HdlTargetDescription,
        TargetId,
    };

    struct TestProvider {
        descriptor: HdlProviderDescriptor,
    }

    impl TestProvider {
        fn new() -> Self {
            Self {
                descriptor: HdlProviderDescriptor::new(
                    "test.hdl.provider",
                    "1.0.0",
                    crate::hdl::realization::interface::
                        HDL_REALIZATION_INTERFACE_VERSION,
                    vec!["test.operation".to_owned()],
                )
                .expect("test provider descriptor must be valid"),
            }
        }
    }

    impl HdlRealizationProvider for TestProvider {
        fn descriptor(&self) -> &HdlProviderDescriptor {
            &self.descriptor
        }

        fn execute_operation(
            &self,
            _plan: &HdlRealizationPlan,
            operation: &HdlRealizationOperation,
            context: &HdlRealizationContext,
        ) -> Result<HdlOperationResult, HdlRealizationInterfaceError> {
            context.check_cancelled()?;

            Ok(
                HdlOperationResult::new(
                    operation.id(),
                    crate::hdl::realization::reports::
                        HdlOperationOutcome::Completed,
                    Some("Test provider completed the operation.".to_owned()),
                )
                .expect("test result must be valid"),
            )
        }
    }

    fn test_plan() -> HdlRealizationPlan {
        let target_id =
            TargetId::new("test.hdl.target")
                .expect("target identity must be valid");

        let target = HdlTargetDescription::new(
            target_id,
            "Test HDL target",
            "test.hdl.target-kind",
        )
        .expect("target description must be valid");

        let mut plan =
            HdlRealizationPlan::new(&target)
                .expect("plan must be valid");

        plan.add_operation(
            HdlRealizationOperation::new(
                "test.operation",
                Some("Test realization operation".to_owned()),
                true,
            )
            .expect("operation must be valid"),
        )
        .expect("operation must be inserted");

        plan
    }

    #[test]
    fn adapter_has_stable_identity() {
        assert_eq!(
            ZHAL_HDL_ADAPTER_NAME,
            "zamani.zhal.adapters.hdl"
        );

        assert_ne!(ZHAL_HDL_ADAPTER_VERSION, 0);
    }

    #[test]
    fn adapter_exposes_the_existing_provider_descriptor() {
        let provider = Arc::new(TestProvider::new());
        let adapter = HdlAdapter::new(provider);

        assert_eq!(
            adapter.descriptor().id(),
            "test.hdl.provider"
        );

        assert_eq!(
            adapter.descriptor().version(),
            "1.0.0"
        );

        assert!(
            adapter
                .descriptor()
                .supports_operation("test.operation")
        );
    }

    #[test]
    fn adapter_returns_the_same_shared_provider() {
        let provider: Arc<dyn HdlRealizationProvider> =
            Arc::new(TestProvider::new());

        let adapter = HdlAdapter::new(Arc::clone(&provider));
        let returned = adapter.provider_handle();

        assert!(Arc::ptr_eq(&provider, &returned));
    }

    #[test]
    fn adapter_clones_share_the_provider() {
        let adapter = HdlAdapter::new(
            Arc::new(TestProvider::new()),
        );

        let cloned = adapter.clone();

        assert_eq!(
            adapter.descriptor().id(),
            cloned.descriptor().id()
        );

        assert!(Arc::ptr_eq(
            &adapter.provider_handle(),
            &cloned.provider_handle(),
        ));
    }

    #[test]
    fn adapter_preserves_canonical_cancellation_behavior() {
        let adapter = HdlAdapter::new(
            Arc::new(TestProvider::new()),
        );

        let plan = test_plan();
        let context = HdlRealizationContext::new();

        context.cancel();

        let result = adapter.realize(
            &plan,
            HdlRealizationPolicy::default(),
            &context,
        );

        assert!(matches!(
            result,
            Err(HdlRealizationInterfaceError::Cancelled)
        ));
    }

    #[test]
    fn adapter_delegates_to_the_canonical_realization_coordinator() {
        let adapter = HdlAdapter::new(
            Arc::new(TestProvider::new()),
        );

        let plan = test_plan();

        // The existing coordinator requires established feasibility by
        // default. This test explicitly permits an attempt with unproven
        // feasibility; it does not claim the target is feasible.
        let policy = HdlRealizationPolicy {
            allow_unproven_feasibility: true,
            allow_unsupported_optional_operations: false,
        };

        let context = HdlRealizationContext::new();

        let report = adapter
            .realize(&plan, policy, &context)
            .expect("test provider should complete the requested operation");

        assert_eq!(
            report.target_id(),
            plan.target_id()
        );

        assert_eq!(
            report.operation_count(),
            1
        );

        assert_eq!(
            report.pending_operation_count(),
            0
        );

        assert!(
            report.all_operations_resolved()
        );

        assert_eq!(
            report.outcome(),
            crate::hdl::realization::reports::
                HdlRealizationOutcome::Partial
        );
    }
}
