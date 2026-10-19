
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Analysis
//!
//! File: `src/hdl/analysis/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! This module is the public composition root for target-independent HDL
//! analyses. It registers the existing analysis implementations without
//! duplicating their algorithms, semantic inputs, graph representations,
//! identities, or result types.
//!
//! ## Architectural authority
//!
//! The analysis subsystem follows the contracts established by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/`
//!
//! The grammar owns HDL syntax. The canonical frontend owns parsing, AST
//! construction, and source-level semantic validation. The HDL design
//! modules own their logical declaration records. This module exposes
//! analyses over those existing contracts.
//!
//! ## Responsibilities
//!
//! This composition root owns:
//!
//! - registration of implemented analysis modules;
//! - their stable, explicit Rust module paths;
//! - the public boundary between independent analyses;
//! - the rule that future analyses must have an implementation and an
//!   integration contract before being registered.
//!
//! The individual analysis modules own their algorithms, configurations,
//! results, findings, and operational-budget behavior.
//!
//! ## Analysis inventory
//!
//! - `connectivity`: source-level signal readers, drivers, and connections.
//! - `cycles`: directed cycles and cyclic strongly connected components.
//! - `dependencies`: the canonical combinational dependency graph and its
//!   cycle witnesses.
//! - `drivers`: source-level signal-driver ownership and driver findings.
//! - `hierarchy`: logical module-instance relationships, reachability,
//!   unresolved references, and recursive-instantiation findings.
//! - `widths`: concrete and symbolic HDL signal-width analysis.
//!
//! These analyses have distinct responsibilities. In particular, cycle
//! analysis consumes the dependency graph rather than defining a competing
//! graph representation.
//!
//! ## Public API policy
//!
//! APIs remain available through their owning module paths, for example:
//!
//! - `crate::hdl::analysis::dependencies::analyze_dependencies`
//! - `crate::hdl::analysis::cycles`
//! - `crate::hdl::analysis::drivers`
//! - `crate::hdl::analysis::widths`
//! - `crate::hdl::analysis::connectivity`
//! - `crate::hdl::analysis::hierarchy`
//!
//! This module intentionally does not wildcard-re-export child APIs or
//! flatten their types into one namespace. This avoids collisions between
//! similarly named configurations, statuses, errors, and result types.
//! Callers should use the types defined by the specific analysis they run.
//!
//! ## Semantic and implementation boundaries
//!
//! This module does not own or implement:
//!
//! - an alternative parser, AST, or semantic analyzer;
//! - a replacement source-span, declaration-ID, or diagnostic system;
//! - general-purpose name resolution or type checking;
//! - a universal HDL graph that replaces the existing dependency graph;
//! - target selection, physical allocation, synthesis, placement, routing,
//!   or timing closure;
//! - HDL intermediate representation, IR builders, verifiers, passes,
//!   serialization, or lowering.
//!
//! All HDLIR implementation is deferred to `src/hdl/hdlir/`. Registering
//! analyses here does not authorize HDLIR implementation elsewhere.
//!
//! ## Scalability and POCO-REAF
//!
//! This composition root imposes no language-level limits on signal counts,
//! module counts, instances, widths, graph sizes, or hierarchy depth.
//!
//! Operational budgets belong to the relevant analysis configuration.
//! Analyses must report when configured budgets prevent completion and must
//! not treat partial results as proof that no issue exists.
//!
//! No configured budget means that this composition root imposes no limit;
//! it does not promise unlimited execution time, memory, address space, or
//! host resources. The subsystem must scale with available resources
//! without adding arbitrary universal hardware ceilings.
//!
//! ## Determinism
//!
//! Module registration has no runtime discovery, generated identities,
//! mutable global state, or order-dependent initialization. Deterministic
//! findings and traversal are the responsibility of each analysis module.
//!
//! ## Integration contract
//!
//! The crate must expose `frontend` and `hdl` through its library root.
//! `crate::hdl::design` must register any design submodules required by the
//! hierarchy analysis. The frontend semantic module remains the authority
//! for source-level validation.
//!
//! Register a new analysis here only after its source file, ownership,
//! dependencies, public API, incomplete-result behavior, and tests exist.
//! Do not register planned or placeholder modules.
//!
//! ## Completion criteria
//!
//! This composition root is integrated when:
//!
//! - every registered module exists;
//! - sibling-module imports resolve through the crate's actual module tree;
//! - public APIs remain available under their documented paths;
//! - analyses do not duplicate semantic or IR ownership;
//! - configured budget exhaustion is represented explicitly by each
//!   affected analysis;
//! - the workspace formatting, compilation, test, and lint checks pass.

// Signal connectivity analysis.
pub mod connectivity;

// Directed dependency-cycle and strongly connected component analysis.
pub mod cycles;

// Canonical source-level dependency graph construction.
pub mod dependencies;

// Signal-driver ownership and driver findings.
pub mod drivers;

// Logical module hierarchy and reachability analysis.
pub mod hierarchy;

// Concrete and symbolic signal-width analysis.
pub mod widths;
