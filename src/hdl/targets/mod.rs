
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Target Subsystem
//!
//! File: `src/hdl/targets/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Target-subsystem composition root. The individual target modules exist,
//! but this file and the complete crate-level integration must be compiled
//! and tested before the subsystem can be declared production-ready.
//!
//! ## Purpose
//!
//! Register the HDL target-description, capability-assessment,
//! target-constraint, and feasibility modules behind one explicit public
//! module boundary.
//!
//! This file composes existing implementations. It does not implement
//! target discovery, physical allocation, HDLIR, or hardware realization.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## Existing implementation dependencies
//!
//! - `super::capabilities` provides the HDL-facing adapter to canonical
//!   semantic capability identities, requirements, and evidence.
//! - `super::resources` provides the HDL-facing adapter to canonical
//!   semantic resource types and profiles.
//! - `super::requirements` provides HDL property and requirement contracts.
//! - `crate::frontend::semantic::capabilities` owns canonical capability
//!   compatibility semantics.
//! - `crate::frontend::semantic::resources` owns canonical resource
//!   semantics.
//! - `crate::frontend::parser::domains::hdl` owns HDL construct parsing.
//! - `crate::frontend::semantic::domains::hdl` owns source-level HDL
//!   semantic validation.
//!
//! These authorities must remain distinct. Target feasibility must not
//! redefine source-language validity, capability identity, resource units,
//! or the semantics of HDL declarations.
//!
//! ## Responsibilities
//!
//! This composition root:
//!
//! - Registers each implemented target module exactly once.
//! - Makes the modules available through stable, explicit Rust paths.
//! - Keeps target description, capability assessment, constraints, and
//!   feasibility as separate responsibilities.
//! - Provides a small regression test for the registered module contracts.
//!
//! ## Explicit exclusions
//!
//! This file does not:
//!
//! - Define a second AST, semantic model, source-span system, or IR.
//! - Parse HDL grammar or perform source-level semantic validation.
//! - Claim that advertised target capabilities are authenticated.
//! - Treat missing resource information as unlimited capacity.
//! - Treat incomplete analysis as proof of feasibility.
//! - Discover hardware, select devices, or allocate physical resources.
//! - Implement synthesis, scheduling, placement, routing, or deployment.
//! - Define HDLIR structures, builders, verification, passes, or lowering.
//!
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/`
//! and is deferred as requested.
//!
//! ## Module ownership
//!
//! `model`
//! : Owns abstract target identity, description metadata, capability
//!   evidence aggregation, optional resource information, and local
//!   target-description validation.
//!
//! `capabilities`
//! : Assesses canonical capability requirements against explicitly scoped
//!   target evidence. It does not establish overall target feasibility.
//!
//! `constraints`
//! : Stores target-scoped property constraints and validates their target
//!   association. It does not independently evaluate their meaning.
//!
//! `feasibility`
//! : Combines the evidence and information available to its implemented
//!   analysis to report feasible, infeasible, or indeterminate outcomes.
//!   Its result must not claim more certainty than the evidence supports.
//!
//! Each module retains its own public API and schema-version constant.
//! This composition root intentionally avoids wildcard re-exports, which
//! could create name collisions as the target subsystem grows.
//!
//! ## Dependency direction
//!
//! ```text
//! Grammar and canonical frontend
//!              |
//!              v
//! Canonical semantic capability/resource models
//!              |
//!              v
//! src/hdl/capabilities.rs + src/hdl/resources.rs
//!              |
//!              v
//! src/hdl/targets/model.rs
//!       |          |          |
//!       v          v          v
//! capabilities  constraints  feasibility
//!                              |
//!                              v
//!                 realization interfaces (future)
//!                              |
//!                              v
//!                     src/hdl/hdlir/ (deferred)
//! ```
//!
//! The diagram documents ownership and intended integration. It does not
//! assert that all later realization stages are implemented.
//!
//! ## Scalability and POCO-REAF
//!
//! No fixed universal ceiling is introduced here for the number of targets,
//! capabilities, constraints, resources, devices, modules, or hardware units.
//!
//! Scalability follows the data structures and operational policies of the
//! registered modules. Actual work remains bounded by available memory,
//! execution time, address space, representable values, and caller-configured
//! operational budgets. Literal infinity is not promised.
//!
//! A budget-exhausted or incomplete analysis must not be converted into a
//! successful feasibility result. Unknown information must remain explicit.
//!
//! Target-specific facts must not silently alter portable HDL semantics.
//! A program may be considered for another target when that target's
//! capabilities, resources, and constraints are assessed independently.
//!
//! ## Public API policy
//!
//! Consumers should use explicit module paths, for example:
//!
//! - `crate::hdl::targets::model::HdlTargetDescription`
//! - `crate::hdl::targets::capabilities::assess_target_capabilities`
//! - `crate::hdl::targets::constraints::HdlTargetConstraints`
//! - `crate::hdl::targets::feasibility::...`
//!
//! This file intentionally does not flatten all public types into
//! `crate::hdl::targets::*`. New convenience re-exports require an explicit
//! API review and must not create ambiguous ownership.
//!
//! ## Integration requirements
//!
//! The parent `src/hdl/mod.rs` must declare `pub mod targets;` and expose
//! the existing `capabilities`, `requirements`, and `resources` modules
//! required by the target implementations.
//!
//! The crate root must also expose the actual `frontend` and `hdl` module
//! trees if the current source files are to be compiled through their
//! `crate::frontend` and `crate::hdl` paths. Those parent-module changes
//! cannot be completed by this file alone.
//!
//! No source parser or semantic validator should depend on target
//! feasibility. The target subsystem consumes their canonical outputs;
//! it must not become an upstream semantic authority.
//!
//! ## Completion criteria
//!
//! - Every registered path resolves to an existing implementation.
//! - Each child module compiles against the canonical shared types.
//! - The crate-root and HDL-root module declarations are integrated.
//! - Module schema identifiers remain stable and independently versioned.
//! - Target-specific uncertainty and infeasibility remain explicit.
//! - No HDLIR implementation is introduced outside `src/hdl/hdlir/`.
//! - Formatting, compilation, tests, and Clippy pass on Rust 1.97 or later.

/// Abstract target descriptions and their local validation contracts.
///
/// This module does not establish target feasibility or authenticate
/// provider-supplied information.
pub mod model;

/// Target-scoped evaluation of canonical capability requirements.
///
/// Capability compatibility is distinct from resource sufficiency and
/// overall target feasibility.
pub mod capabilities;

/// Storage and target-association validation for target-scoped constraints.
///
/// Constraint storage does not prove that a target satisfies those
/// constraints.
pub mod constraints;

/// Analysis of whether a target can satisfy the available requirements.
///
/// Incomplete evidence must remain indeterminate rather than being
/// silently treated as successful feasibility.
pub mod feasibility;

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure each registered module publishes its expected stable identity.
    #[test]
    fn registered_modules_have_stable_names() {
        assert_eq!(
            model::HDL_TARGET_MODEL_MODULE_NAME,
            "zamani.hdl.targets.model"
        );
        assert_eq!(
            capabilities::HDL_TARGET_CAPABILITIES_MODULE_NAME,
            "zamani.hdl.targets.capabilities"
        );
        assert_eq!(
            constraints::HDL_TARGET_CONSTRAINTS_MODULE_NAME,
            "zamani.hdl.targets.constraints"
        );
        assert_eq!(
            feasibility::HDL_TARGET_FEASIBILITY_MODULE_NAME,
            "zamani.hdl.targets.feasibility"
        );
    }

    /// Ensure each module exposes an explicit schema version.
    ///
    /// The versions are intentionally checked independently: they describe
    /// separate contracts and need not remain numerically identical.
    #[test]
    fn registered_modules_publish_schema_versions() {
        let _model_version = model::HDL_TARGET_MODEL_SCHEMA_VERSION;
        let _capabilities_version =
            capabilities::HDL_TARGET_CAPABILITIES_SCHEMA_VERSION;
        let _constraints_version =
            constraints::HDL_TARGET_CONSTRAINTS_SCHEMA_VERSION;
        let _feasibility_version =
            feasibility::HDL_TARGET_FEASIBILITY_SCHEMA_VERSION;
    }
}
