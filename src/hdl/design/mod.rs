
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Design Model
//!
//! Path: `src/hdl/design/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for the target-independent HDL design
//! model. It exposes the existing module and package declaration models
//! through a stable, explicit Rust module boundary.
//!
//! It coordinates module visibility only. It does not implement another AST,
//! parser, semantic analyzer, symbol table, package registry, hardware
//! realization layer, or intermediate representation.
//!
//! ## Authority
//!
//! The design model must remain consistent with:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `grammar/modules/packages.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//!
//! The grammar defines source syntax. The canonical AST owns source nodes,
//! their identities, and source spans. The frontend semantic layer owns
//! name resolution and resolved source-level meaning. The HDL design model
//! retains structured, target-independent references to those declarations.
//!
//! ## Ownership
//!
//! This composition root owns:
//!
//! - registration of the HDL design submodules;
//! - their public Rust module paths;
//! - the boundary between design declarations and downstream concerns.
//!
//! `module.rs` owns the logical HDL module declaration record.
//!
//! `package.rs` owns the logical HDL package declaration record.
//!
//! Each child module remains the sole owner of its own types, validation,
//! invariants, and tests.
//!
//! ## Explicit exclusions
//!
//! This module does not own:
//!
//! - source syntax, lexing, parsing, or AST construction;
//! - canonical AST nodes, `NodeId`, or `Span` implementations;
//! - symbol resolution, visibility resolution, or dependency solving;
//! - port, signal, instance, interface, or hierarchy implementations;
//! - hardware types, widths, values, or expression evaluation;
//! - physical resource allocation or target selection;
//! - synthesis, placement, routing, or timing closure;
//! - HDL intermediate representation, builders, verifiers, passes, or
//!   lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is outside
//! the implementation scope of this module.
//!
//! ## Integration contract
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical grammar
//!       |
//!       v
//! frontend parser and canonical AST
//!       |
//!       v
//! frontend semantic analysis
//!       |
//!       v
//! src/hdl/design
//!       |
//!       +--> module
//!       +--> package
//!       |
//!       v
//! structural analysis and validation
//!       |
//!       v
//! target feasibility and realization interfaces
//!       |
//!       v
//! src/hdl/hdlir/  (deferred)
//! ```
//!
//! Parser and semantic code must not depend on this composition root merely
//! to construct or validate canonical AST nodes. An explicit adapter may
//! create these derived design records after obtaining the required canonical
//! AST references.
//!
//! The design model must not import target realization or HDLIR merely to
//! expose its declaration types.
//!
//! ## Public API policy
//!
//! Child modules are exposed by their explicit module paths:
//!
//! - `crate::hdl::design::module`
//! - `crate::hdl::design::package`
//!
//! This composition root deliberately avoids wildcard re-exports. That
//! prevents unrelated names from colliding as the design subsystem grows
//! and lets each child module evolve its own API deliberately.
//!
//! Adding a new design submodule requires its implementation and contract
//! to exist before it is registered here. Do not register placeholder
//! modules or claim capabilities that have not been implemented.
//!
//! ## Scalability and POCO-REAF
//!
//! This file introduces no limits on the number of modules, packages,
//! declarations, instances, or hardware resources. It owns no capacity
//! counters and makes no assumptions about machine size, memory capacity,
//! device counts, or vendor-specific resources.
//!
//! Scalability constraints belong to the relevant algorithms and explicit
//! operational policies, not to this composition root. Actual processing
//! remains bounded by available resources and representable values; no
//! physical system is assumed to have infinite capacity.
//!
//! ## Determinism
//!
//! This module has no mutable global state, generated identifiers, runtime
//! discovery, or order-dependent registration logic. It simply declares
//! the child-module boundaries.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden in this module. Child modules independently
//! retain their own safety restrictions.
//!
//! ## Completion criteria
//!
//! This module is integrated when:
//!
//! - both declared child modules exist;
//! - their public paths are reachable through the crate's HDL module;
//! - canonical frontend dependencies resolve through the crate's actual
//!   module tree;
//! - the crate compiles and the workspace checks pass;
//! - no HDLIR implementation has been introduced outside `hdlir/`.

/// Target-independent HDL module declaration model.
///
/// Contains the logical module name, canonical declaration identity,
/// source span, and references to canonical AST nodes. It does not own
/// parsing, name resolution, or physical realization.
pub mod module;

/// Target-independent HDL package declaration model.
///
/// Contains the logical package name, canonical declaration identity,
/// source span, and ordered references to canonical AST metadata and
/// attribute nodes. It does not own package resolution or registry policy.
pub mod package;

#[cfg(test)]
mod tests {
    #[test]
    fn declaration_modules_are_registered() {
        // These type references ensure the intended child-module paths
        // remain usable without creating wildcard re-exports.
        let _: Option<super::module::HdlModule> = None;
        let _: Option<super::package::HdlPackage> = None;
    }

    #[test]
    fn child_schema_versions_are_available() {
        assert_eq!(super::module::HDL_MODULE_SCHEMA_VERSION, 1);
        assert_eq!(super::package::HDL_PACKAGE_SCHEMA_VERSION, 1);
    }
}
