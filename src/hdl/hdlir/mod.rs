
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hardware Description Language Intermediate Representation
//!
//! Path: `src/hdl/hdlir/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for Zamani's Hardware Description
//! Language Intermediate Representation (HDLIR).
//!
//! It registers the existing HDLIR foundations, schema, hardware types,
//! symbolic values, graph representation, builders, verification,
//! analysis, transformations, passes, lowering, serialization, dialects,
//! and interchange modules.
//!
//! This file owns module composition and the architectural boundaries
//! documented below. It does not implement a second IR, duplicate child
//! implementations, or claim that every registered subsystem is complete.
//!
//! ## Normative authority
//!
//! HDLIR must conform to the repository's actual language and architecture
//! authorities, including:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/frontend/ast/`
//! - `src/frontend/semantic/`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//!
//! The specifications define the intended architecture and semantics.
//! The actual public APIs and implementations determine which features
//! are currently available and how they can be integrated.
//!
//! ## Representation ownership
//!
//! The authoritative pipeline is:
//!
//! ```text
//! Zamani source / imported HDL
//!             |
//!             v
//! Canonical frontend AST and source locations
//!             |
//!             v
//! Canonical semantic model and HDL validation
//!             |
//!             v
//! ZUIR: universal computational representation
//!             |
//!             v
//! HDLIR: hardware-domain representation
//!             |
//!             v
//! Hardware optimization and lowering
//!             |
//!             v
//! Target-specific implementation and realization
//! ```
//!
//! This describes the intended dependency direction, not a claim that every
//! stage or conversion is already complete.
//!
//! The ownership boundaries are:
//!
//! - The grammar and frontend own source syntax, AST construction,
//!   source locations, name resolution, and source-level semantic analysis.
//! - The canonical semantic model owns source-level types, effects,
//!   capabilities, requirements, contracts, and provenance.
//! - ZUIR owns the universal computational representation.
//! - HDLIR owns hardware-domain graph structures and hardware-specific
//!   representation contracts.
//! - Downstream implementation stages own target selection, synthesis,
//!   scheduling, physical allocation, placement, routing, and realization.
//!
//! HDLIR must not become a replacement AST, a second semantic model,
//! a competing universal IR, or a physical hardware allocation system.
//!
//! ## Module responsibilities
//!
//! The foundational modules define shared HDLIR contracts:
//!
//! - `config`: explicit operational budgets and processing policies.
//! - `version`: HDLIR version and compatibility semantics.
//! - `errors`: structured HDLIR failures and result types.
//! - `diagnostics`: structured HDLIR diagnostics.
//! - `ids`: typed HDLIR identities.
//! - `source_map`: source and transformation provenance.
//! - `fingerprint`: deterministic structural fingerprints.
//!
//! The schema and type modules define the extensible representation:
//!
//! - `schema`: operation, type, interface, and dialect registration.
//! - `types`: hardware-domain types, widths, clocks, logic, and memories.
//! - `values`: constants, symbolic expressions, and parameters.
//!
//! The graph and construction modules define the representation itself:
//!
//! - `ir`: designs, modules, ports, operations, values, regions, blocks,
//!   control flow, connectivity, processes, clocks, and memories.
//! - `builder`: checked construction APIs for supported graph objects.
//!
//! Verification and analysis are distinct responsibilities:
//!
//! - `verify`: structural, type, connectivity, process, clock, and resource
//!   contract verification.
//! - `analysis`: derived dependency, effect, clock, resource, and width facts.
//!
//! Transformations and orchestration are kept separate:
//!
//! - `passes`: pass contracts, execution, and preservation obligations.
//! - `transform`: individual canonicalization, simplification,
//!   dead-code planning, and specialization implementations.
//!
//! Integration boundaries are explicitly registered:
//!
//! - `lowering`: conversions between supported semantic, ZUIR, and HDLIR
//!   representations.
//! - `serialization`: versioned HDLIR artifact encoding, decoding, and
//!   migration.
//! - `dialects`: structural, behavioral, sequential, memory, and
//!   physical-intent dialect contracts.
//! - `interchange`: supported external-format import and export.
//!
//! The responsibilities above do not imply that every operation is
//! supported by every subsystem or target. Each owning module's documented
//! contract remains authoritative.
//!
//! ## Public API policy
//!
//! Subsystems are accessed through their named module paths, for example:
//!
//! - `crate::hdl::hdlir::ir::design`
//! - `crate::hdl::hdlir::schema::registry`
//! - `crate::hdl::hdlir::verify`
//! - `crate::hdl::hdlir::lowering`
//!
//! This composition root intentionally does not glob-re-export every child
//! type. Explicit paths preserve ownership, avoid ambiguous names, and
//! prevent accidental expansion of the root API.
//!
//! Existing child APIs remain owned by their respective modules. Any future
//! convenience re-export must be deliberate, documented, compatible with
//! existing consumers, and covered by tests.
//!
//! ## Dependency rules
//!
//! 1. Foundational contracts must not depend on builders, graph verification,
//!    lowering, or target realization.
//! 2. Schema definitions must not depend on concrete graph instances merely
//!    to describe an operation or type.
//! 3. IR nodes must use canonical HDLIR identities and shared error contracts.
//! 4. Builders must construct the canonical IR rather than duplicate it.
//! 5. Verification must consume the actual graph and required schema/context
//!    contracts; it must not silently skip unavailable checks.
//! 6. Analysis must preserve uncertainty and distinguish estimates from
//!    verified facts.
//! 7. Transformations must declare their preconditions and preservation
//!    obligations.
//! 8. Lowering must use actual frontend and ZUIR APIs and report unsupported
//!    or lossy conversions explicitly.
//! 9. Serialization must validate versions, definitions, and untrusted
//!    input before accepting an artifact.
//! 10. Target-specific realization must not be pulled into the portable
//!     representation merely for implementation convenience.
//!
//! Child modules must not introduce dependency cycles through this
//! composition root. Shared types belong in their designated owners.
//!
//! ## POCO-REAF and scalability
//!
//! The architectural objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! HDLIR must not impose arbitrary universal hardware ceilings for signal
//! widths, ports, modules, instances, operations, clock domains, memories,
//! pipeline stages, or physical resources.
//!
//! Symbolic quantities must remain symbolic until their owning stage is
//! authorized to evaluate or specialize them. Concrete arithmetic and
//! integer conversions must be checked.
//!
//! Operational budgets are permitted as explicit controls on memory,
//! execution work, verification, decoding, and transformations. Such budgets
//! are not hardware-language limits. Exhaustion must be reported as an
//! error or incomplete result, never as successful completion.
//!
//! No finite implementation can process literally infinite input. Actual
//! processing remains bounded by available resources, representable values,
//! supported semantics, and explicit operational budgets.
//!
//! A target that cannot satisfy a design's requirements must produce an
//! explicit feasibility or unsupported-capability result. It must not
//! silently change the design's meaning.
//!
//! ## Determinism and compatibility
//!
//! Where promised by the owning contracts, equivalent designs, schema
//! versions, configurations, and pass versions must produce deterministic
//! results and canonical artifacts.
//!
//! Module registration must not depend on filesystem traversal, hardware
//! discovery order, hidden global state, or unspecified iteration order.
//!
//! HDLIR artifact compatibility, schema compatibility, public Rust API
//! compatibility, and Zamani language compatibility are separate contracts.
//! Changes must follow the corresponding specifications and migrations.
//!
//! ## Error and safety boundaries
//!
//! Parser errors, semantic diagnostics, invalid HDLIR, unsupported
//! operations, target infeasibility, incompatible artifacts, and operational
//! budget exhaustion are distinct outcomes.
//!
//! This composition root must not collapse those outcomes into strings or
//! treat incomplete verification as successful verification.
//!
//! Unsafe Rust is forbidden throughout this module and its descendants.
//! Imported designs and serialized artifacts must be treated as untrusted.
//! Individual owning modules are responsible for validating inputs,
//! checking arithmetic, avoiding unchecked assumptions, and reporting
//! failures through their documented APIs.
//!
//! ## Registration policy
//!
//! Only source modules that exist in the repository are declared below.
//! A module's presence in this tree does not certify its implementation as
//! production-ready.
//!
//! New modules must be registered only after their ownership, dependencies,
//! public API, failure behavior, and tests have been established.
//!
//! ## Production-readiness criteria
//!
//! The composition root is integrated when:
//!
//! - Every declared source module exists.
//! - The registered modules resolve against the actual crate tree.
//! - Existing HDLIR implementations remain under `src/hdl/hdlir/`.
//! - The canonical frontend and ZUIR boundaries remain intact.
//! - No placeholder IR API is needed to make module registration compile.
//! - No duplicate IR, semantic model, or source-span system is introduced.
//! - Repository formatting, compilation, tests, and Clippy pass.
//!
//! Registering these modules alone does not establish that the complete
//! HDLIR pipeline is production-ready.

/// Operational budgets and processing policies.
pub mod config;

/// Structured HDLIR diagnostics.
pub mod diagnostics;

/// Structured HDLIR errors and result contracts.
pub mod errors;

/// Deterministic structural fingerprints.
pub mod fingerprint;

/// Typed HDLIR identities.
pub mod ids;

/// Source and transformation provenance.
pub mod source_map;

/// HDLIR schema and compatibility versioning.
pub mod version;

/// Extensible hardware operation, type, interface, and dialect schemas.
pub mod schema;

/// Hardware-domain types and symbolic width descriptions.
pub mod types;

/// Concrete constants, symbolic expressions, and parameters.
pub mod values;

/// Canonical hardware-domain design graph.
pub mod ir;

/// Checked HDLIR construction APIs.
pub mod builder;

/// Whole-graph and domain-specific verification.
pub mod verify;

/// Read-only analyses of HDLIR structures and properties.
pub mod analysis;

/// Declarative transformation contracts and pass orchestration.
pub mod passes;

/// Individual HDLIR transformation implementations.
pub mod transform;

/// Hardware dialect definitions and registration.
pub mod dialects;

/// Semantic-model and ZUIR conversion boundaries.
pub mod lowering;

/// Versioned HDLIR artifact encoding, decoding, and migration.
pub mod serialization;

/// Supported external-format import and export.
pub mod interchange;

/// Cross-module HDLIR tests; excluded from production builds.
#[cfg(test)]
mod tests;
