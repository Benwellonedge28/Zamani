
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Realization
//!
//! Path: `src/hdl/realization/mod.rs`.
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Composition root for the HDL realization subsystem.
//!
//! This module registers the existing realization plan, provider interface,
//! and realization report contracts. Production readiness depends on the
//! integration of these modules with the parent HDL subsystem and on the
//! repository's actual compilation, test, and lint results.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/scalability-model.md`
//! - `src/hdl/realization/plan.rs`
//! - `src/hdl/realization/interface.rs`
//! - `src/hdl/realization/reports.rs`
//! - `src/hdl/targets/mod.rs`
//!
//! The grammar owns syntax. The canonical frontend and semantic model own
//! source-level meaning. The target subsystem owns target descriptions and
//! feasibility assessment. This module composes the realization contracts
//! without replacing those authorities.
//!
//! ## Responsibilities
//!
//! This module:
//!
//! - Registers each existing realization module exactly once.
//! - Establishes stable, explicit module paths for downstream consumers.
//! - Documents the dependency direction between plans, providers, and reports.
//! - Exposes a versioned identity for the composition-root contract.
//! - Provides focused tests for module registration.
//!
//! ## Explicit exclusions
//!
//! This module does not:
//!
//! - Parse source code or construct AST nodes.
//! - Perform source-level semantic validation or name resolution.
//! - Define another requirements, capabilities, resources, or target model.
//! - Evaluate target feasibility.
//! - Discover, authenticate, authorize, or allocate physical hardware.
//! - Implement synthesis, scheduling, placement, routing, or timing closure.
//! - Execute a provider or perform hardware realization by itself.
//! - Define a hardware intermediate representation.
//! - Implement HDLIR builders, operations, verification, passes, serialization,
//!   optimization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is explicitly
//! deferred. This composition root must not become a parallel IR boundary.
//!
//! ## Module ownership
//!
//! `plan`:
//! Owns the declarative realization-plan contract, including the selected
//! target description, requirements, target-scoped constraints, requested
//! provider operations, unresolved decisions, and optional feasibility
//! evidence.
//!
//! `interface`:
//! Owns provider descriptors, capability negotiation, provider invocation
//! contracts, cooperative cancellation, and orchestration-level validation.
//! Provider-advertised capabilities are not proof of physical availability.
//!
//! `reports`:
//! Owns structured realization outcomes, operation results, issue records,
//! and report consistency checks. A provider-reported success is not
//! independent proof of implementation correctness.
//!
//! These responsibilities remain separate. A plan is a request, an interface
//! coordinates a provider, and a report records an outcome.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical grammar, AST, and semantic model
//!                    |
//!                    v
//!       HDL requirements and resource contracts
//!                    |
//!                    v
//!          Target descriptions and feasibility
//!                    |
//!                    v
//!        realization::plan  <----- target evidence
//!                    |
//!                    v
//!       realization::interface
//!                    |
//!                    v
//!        realization::reports
//!                    |
//!                    v
//!        Future implementation integration
//!                    |
//!                    v
//!         src/hdl/hdlir/ (deferred)
//! ```
//!
//! This diagram describes intended ownership and dependency direction; it
//! does not claim that every downstream stage is implemented.
//!
//! ## Portability and POCO-REAF
//!
//! Realization requests must preserve portable HDL semantics across supported
//! targets. A target-specific limitation must be reported as infeasibility,
//! unsupported functionality, or an unresolved implementation decision;
//! it must not silently change the program's meaning.
//!
//! Target IDs identify target descriptions, not necessarily physical devices.
//! Provider IDs and operation IDs remain extensible identifiers rather than
//! a closed list of vendors or hardware technologies.
//!
//! ## Scalability
//!
//! This composition root introduces no fixed maximum number of targets,
//! providers, operations, constraints, resources, modules, or hardware units.
//!
//! Scalability is governed by the underlying data structures, available
//! memory and processing time, representable values, and explicit operational
//! budgets owned by the relevant workflow or provider.
//!
//! Literal infinite storage or computation is not promised. If a provider or
//! analysis exhausts its budget, is cancelled, or lacks necessary evidence,
//! the result must remain explicitly incomplete or indeterminate rather than
//! being reported as successful.
//!
//! ## API policy
//!
//! The canonical public paths are:
//!
//! - `crate::hdl::realization::plan`
//! - `crate::hdl::realization::interface`
//! - `crate::hdl::realization::reports`
//!
//! Types should be accessed through their owning modules. This root
//! intentionally avoids wildcard re-exports and duplicate type aliases.
//! Convenience re-exports require a separate API review.
//!
//! ## Integration requirements
//!
//! 1. The parent HDL module must register `pub mod realization;`.
//! 2. The parent HDL module must register the canonical modules required by
//!    the existing realization and target files.
//! 3. `reports.rs` must import its plan types from
//!    `crate::hdl::realization::plan`, not `crate::hdl::plan`.
//! 4. The crate root and frontend module hierarchy must expose the paths used
//!    by these files before they can compile as part of the library.
//! 5. Provider execution must remain behind the provider interface.
//! 6. Feasibility evidence must not be upgraded by the composition root.
//! 7. HDLIR implementation must remain exclusively within `src/hdl/hdlir/`.
//!
//! ## Freeze criteria
//!
//! - All registered files exist and compile through their canonical paths.
//! - Existing public contracts are reused without competing definitions.
//! - Plan, interface, and report responsibilities remain distinct.
//! - Unknown feasibility and incomplete realization remain explicit.
//! - No unsafe Rust or artificial universal hardware ceilings are introduced.
//! - Unit, integration, conformance, and workspace checks pass.
//!
//! This file does not claim those checks have already passed.

/// Stable architectural name of the HDL realization composition root.
pub const HDL_REALIZATION_MODULE_NAME: &str = "zamani.hdl.realization";

/// Version of this composition-root contract.
///
/// This is independent of the Zamani language version, grammar version,
/// target-description schema, provider interface version, report schema,
/// package version, and future HDLIR version.
pub const HDL_REALIZATION_MODULE_SCHEMA_VERSION: u16 = 1;

/// Declarative realization plans and their structural validation contracts.
///
/// Plans describe requested work; they do not perform physical realization.
pub mod plan;

/// Structured outcomes and issue records for realization attempts.
///
/// Reports record the provider's reported outcome; they do not independently
/// prove that a resulting implementation is correct.
pub mod reports;

/// Portable provider negotiation and realization orchestration contracts.
///
/// This module coordinates the existing plan and report APIs without defining
/// a second target model or an intermediate representation.
pub mod interface;

#[cfg(test)]
mod tests {
    use super::*;

    /// The composition root publishes its stable architectural identity.
    #[test]
    fn composition_root_has_stable_identity() {
        assert_eq!(
            HDL_REALIZATION_MODULE_NAME,
            "zamani.hdl.realization"
        );
        assert_eq!(HDL_REALIZATION_MODULE_SCHEMA_VERSION, 1);
    }

    /// Each child module is registered and exposes its own identity.
    #[test]
    fn registered_modules_have_stable_names() {
        assert_eq!(
            plan::HDL_REALIZATION_PLAN_MODULE_NAME,
            "zamani.hdl.realization.plan"
        );
        assert_eq!(
            reports::HDL_REALIZATION_REPORTS_MODULE_NAME,
            "zamani.hdl.realization.reports"
        );
        assert_eq!(
            interface::HDL_REALIZATION_INTERFACE_MODULE_NAME,
            "zamani.hdl.realization.interface"
        );
    }

    /// The composition root keeps each contract independently versioned.
    #[test]
    fn child_contract_versions_are_nonzero() {
        assert_ne!(plan::HDL_REALIZATION_PLAN_SCHEMA_VERSION, 0);
        assert_ne!(reports::HDL_REALIZATION_REPORT_SCHEMA_VERSION, 0);
        assert_ne!(interface::HDL_REALIZATION_INTERFACE_VERSION, 0);
    }
}
