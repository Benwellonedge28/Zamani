#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Hardware Description Language Intermediate Representation
//!
//! Path: `src/hdl/hdlir/ir/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Compose the hardware-domain IR graph implemented under this directory.
//! This module registers the existing IR components and establishes their
//! namespace. It does not duplicate their definitions or implement graph
//! construction, verification, transformation, serialization, or lowering.
//!
//! ## Architectural authority
//!
//! The implementation must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and the applicable HDL delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/schema/`
//! - `src/hdl/hdlir/types/`
//! - `src/hdl/hdlir/values/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! Grammar and the canonical frontend own source syntax and source-level
//! meaning. ZUIR owns the universal computational representation. This IR
//! owns hardware-domain graph structures and their local invariants.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the IR graph submodules.
//! - The public namespace through which IR components are accessed.
//! - Documentation of the relationships between the graph components.
//!
//! The individual submodules own their data structures, construction rules,
//! and local invariants.
//!
//! This module does not own:
//!
//! - Source parsing, AST nodes, name resolution, or semantic analysis.
//! - A replacement for ZUIR or another universal intermediate representation.
//! - A second ID, source-span, diagnostic, schema, or type system.
//! - Graph-wide verification or transformation-preservation proofs.
//! - Target discovery, physical resource allocation, synthesis, placement,
//!   routing, timing closure, or runtime hardware control.
//!
//! ## Graph responsibilities
//!
//! - `design` owns the top-level design and logical module registry.
//! - `module` owns logical module declarations and their references.
//! - `ports` owns logical module-boundary port declarations.
//! - `operations` owns operation instances and their registered attributes.
//! - `values` owns operation/value relationships and value-use indexing.
//! - `regions` owns region structure and ordered block references.
//! - `blocks` owns block arguments, operation ordering, and terminators.
//! - `control_flow` owns explicit control-flow edges and graph relationships.
//! - `connectivity` owns logical connection and driver/sink descriptions.
//! - `processes` owns behavioral process and event declarations.
//! - `clocks` owns logical clock domains, edges, and reset descriptions.
//! - `memories` owns abstract memory instances and logical port bindings.
//!
//! These responsibilities are complementary. The existence of a local
//! structure does not prove that its references are valid in the complete
//! design. Cross-object invariants must be checked by the appropriate
//! graph verifier before the IR is trusted by downstream stages.
//!
//! ## Dependency and integration contract
//!
//! The IR components use the shared contracts under `src/hdl/hdlir/`:
//!
//! - `ids.rs` supplies HDLIR-owned typed identities.
//! - `errors.rs` supplies structured errors and `HdlirResult`.
//! - `schema/` supplies registered operation and interface definitions.
//! - `types/` supplies hardware-domain type descriptions.
//! - `values/` supplies constants, symbolic expressions, and parameters.
//!
//! These shared contracts remain authoritative. This module must not define
//! competing IDs, errors, types, schemas, or value representations.
//!
//! The graph components may refer to one another through typed IDs. A typed
//! reference is not proof that the referenced object exists. Whole-graph
//! verification must establish reference resolution, ownership, uniqueness,
//! type consistency, connectivity legality, process/clock compatibility,
//! and other applicable invariants.
//!
//! The intended integration direction is:
//!
//! ```text
//! frontend semantic model / ZUIR
//!               |
//!               v
//!       HDLIR lowering boundary
//!               |
//!               v
//!      ir::design and ir::module
//!               |
//!       +-------+--------+
//!       |       |        |
//!       v       v        v
//!    values  operations connectivity
//!       |       |        |
//!       +-------+--------+
//!               |
//!       regions / blocks / control flow
//!               |
//!       processes / clocks / memories
//!               |
//!               v
//!       HDLIR graph verification
//!               |
//!               v
//!       downstream hardware stages
//! ```
//!
//! This diagram describes architectural responsibilities, not a claim that
//! all lowering, graph-wide verification, or downstream stages are already
//! implemented.
//!
//! ## Public API policy
//!
//! Components are accessed through their named submodules, for example:
//!
//! - `ir::design::HdlirDesign`
//! - `ir::module::HdlirModule`
//! - `ir::operations::HdlirOperation`
//! - `ir::clocks::HdlirClockDomains`
//! - `ir::connectivity::HdlirConnectivity`
//!
//! This deliberate namespacing prevents conflicting names from different
//! owners from being silently merged into a single API namespace. In
//! particular, `design::ModuleName` and `module::ModuleName` are distinct
//! declarations in the current source tree and are not glob-re-exported.
//!
//! A future convenience re-export must be deliberate, documented, and
//! covered by compatibility tests. It must not obscure ownership or create
//! ambiguous public names.
//!
//! ## Scalability and determinism
//!
//! This module imposes no limits on the number of modules, ports, operations,
//! values, blocks, regions, connections, processes, clock domains, or memory
//! declarations.
//!
//! Actual storage and processing remain bounded by available resources,
//! representable IDs, collection representation, supported semantics, and
//! explicit operational budgets. No finite implementation can process
//! literally infinite input.
//!
//! This composition module performs no graph traversal, size calculation,
//! or allocation. It therefore introduces no host-sized hardware limits,
//! implicit capacity assumptions, or nondeterministic initialization.
//!
//! Ordering, equality, reference validation, and fingerprint semantics are
//! defined by the owning component and the applicable HDLIR specifications.
//!
//! ## Failure behavior
//!
//! This module performs no fallible runtime operation. It must not hide
//! missing modules, substitute placeholders for missing implementations,
//! or register speculative components.
//!
//! A missing source file or unresolved dependency is a compile-time
//! integration failure and must be fixed at its actual ownership boundary.
//!
//! ## Completion criteria
//!
//! - Every registered module exists in the repository.
//! - Every declared dependency resolves against the actual crate tree.
//! - No component has been implemented twice under different paths.
//! - Public names remain unambiguous and intentional.
//! - The parent HDLIR composition root registers this module.
//! - The obsolete `hdlir::Placeholder` test reference is removed or replaced
//!   with a reference to an actual public HDLIR item.
//! - Formatting, compilation, tests, and Clippy pass in the actual checkout.

/// Basic ordered blocks and block arguments.
pub mod blocks;

/// Logical clocks, clock domains, edges, and reset descriptions.
pub mod clocks;

/// Logical connectivity, connections, drivers, and sinks.
pub mod connectivity;

/// Control-flow edges, terminators, and control-flow graphs.
pub mod control_flow;

/// Top-level designs, design names, and logical module registries.
pub mod design;

/// Abstract memories and logical memory-port bindings.
pub mod memories;

/// Logical module declarations and their parameter/port/region references.
pub mod module;

/// Registered operation instances, operands, results, and attributes.
pub mod operations;

/// Logical module-boundary ports and ordered port collections.
pub mod ports;

/// Behavioral processes and their event declarations.
pub mod processes;

/// Region structure and ordered block references.
pub mod regions;

/// Value definitions, operation uses, and value-use indexing.
pub mod values;