//! # Zamani Native AST — Source Paths
//!
//! Canonical aggregation boundary for source-level paths in the Zamani
//! frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! frontend::ast::node::paths
//!     │
//!     ├── Path
//!     │    └── PathSegment
//!     │
//!     ├── Import
//!     │
//!     └── future source-path extensions
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── module resolution
//!     ├── import resolution
//!     ├── type resolution
//!     ├── capability resolution
//!     └── resource/domain resolution
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! ## Purpose
//!
//! This module is the public module boundary for source-level paths.
//!
//! It owns:
//!
//! - module declarations;
//! - submodule declarations;
//! - stable public re-exports of path types;
//! - the dependency boundary between path implementations and their
//!   consumers.
//!
//! It does **not** own the implementation of:
//!
//! - `Path`;
//! - `PathSegment`;
//! - imports;
//! - name resolution;
//! - symbol resolution;
//! - type resolution;
//! - semantic analysis;
//! - capability resolution;
//! - resource allocation;
//! - quantum mapping;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend selection;
//! - execution.
//!
//! Those responsibilities belong to their respective modules or downstream
//! compiler phases.
//!
//! ## Domain neutrality
//!
//! A source path is a language-level construct. It must not encode the
//! computational domain of the entity eventually resolved from that path.
//!
//! Therefore this module does not contain domain-specific path types such as:
//!
//! ```text
//! QuantumPath
//! HardwarePath
//! QubitPath
//! BackendPath
//! CpuPath
//! GpuPath
//! HdlPath
//! QirPath
//! MlirPath
//! ```
//!
//! A path such as:
//!
//! ```text
//! quantum::algorithm::qft
//! ```
//!
//! is represented only as source-level path components at this layer.
//!
//! Whether the path denotes a quantum operation, module, type, resource,
//! capability, function, or another entity is determined later by semantic
//! analysis.
//!
//! ## POCO-REAF
//!
//! This module introduces no computational-size assumption.
//!
//! It contains no:
//!
//! - maximum path depth;
//! - maximum number of path segments;
//! - maximum identifier length;
//! - maximum module count;
//! - maximum resource count;
//! - maximum qubit count;
//! - machine-size assumption;
//! - hardware-topology assumption;
//! - vendor assumption;
//! - backend assumption.
//!
//! The underlying path representation therefore scales according to the
//! resources available to the compiler and the representable Rust data model.
//!
//! Operational limits for hostile or resource-exhausting input belong to
//! configurable compiler policy. They must not be encoded here as language
//! semantics.
//!
//! ## Separation from filesystem paths
//!
//! These are **programming-language paths**, not operating-system paths.
//!
//! The path API must never interpret a source path as:
//!
//! - a filesystem path;
//! - a directory;
//! - a URL;
//! - a network address;
//! - a package-registry URL;
//! - a drive path;
//! - an operating-system root.
//!
//! Filesystem/package/source-unit resolution belongs to separate infrastructure.
//!
//! ## Separation from semantic resolution
//!
//! This module exposes unresolved source structures.
//!
//! ```text
//! Path
//!   │
//!   ▼
//! semantic resolver
//!   │
//!   ▼
//! Resolved symbol / semantic entity
//! ```
//!
//! `Path` must not contain:
//!
//! - symbol IDs;
//! - resolved type IDs;
//! - resource IDs;
//! - qubit IDs;
//! - backend IDs;
//! - hardware IDs;
//! - target IDs.
//!
//! Those values belong to downstream semantic or compilation structures.
//!
//! ## Separation from ZUIR
//!
//! The path AST is not an intermediate representation.
//!
//! ```text
//! Native AST
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! Domain IR
//!     │
//!     ▼
//! Target IR
//! ```
//!
//! A path module must therefore never depend on ZUIR or any target IR.
//!
//! ## Separation from external quantum languages
//!
//! OpenQASM, QIR, Quil, Q#, vendor formats, and future quantum formats are
//! external representations. They must not redefine this module.
//!
//! Their frontends may parse into their own format-specific ASTs and then
//! lower into Zamani's source/semantic representation.
//!
//! Conceptually:
//!
//! ```text
//! OpenQASM AST ─────┐
//! QIR              │
//! Quil             ├──► semantic adaptation ───► Zamani semantic model
//! Q#               │
//! vendor format ───┘
//! ```
//!
//! The native `Path` remains independent from all of them.
//!
//! ## Module ownership
//!
//! The current canonical implementation files are:
//!
//! ```text
//! paths/
//! ├── mod.rs       ← this aggregation boundary
//! ├── path.rs      ← complete Path implementation
//! ├── segment.rs   ← complete PathSegment implementation
//! └── import.rs    ← import/path integration
//! ```
//!
//! `mod.rs` must not duplicate definitions from those files.
//!
//! ## Canonical representation rule
//!
//! There must be exactly one authoritative `Path` representation and exactly
//! one authoritative `PathSegment` representation within the native AST.
//!
//! This module therefore deliberately performs only module declaration and
//! public re-export.
//!
//! If an implementation file currently contains a duplicate representation,
//! migration must consolidate it into the canonical implementation rather
//! than adding another compatibility type here.
//!
//! In particular, consumers should eventually use:
//!
//! ```rust
//! use super::paths::{Path, PathSegment};
//! ```
//!
//! or the fully qualified equivalent:
//!
//! ```rust
//! use crate::frontend::ast::node::paths::{Path, PathSegment};
//! ```
//!
//! rather than depending unnecessarily on implementation-file layout.
//!
//! ## Dependency direction
//!
//! ```text
//! source infrastructure
//!        │
//!        ▼
//! PathSegment
//!        │
//!        ▼
//! Path
//!        │
//!        ▼
//! paths::mod
//!        │
//!        ├──► expressions
//!        ├──► declarations
//!        ├──► types
//!        ├──► patterns
//!        ├──► annotations
//!        └──► imports
//!        │
//!        ▼
//! structural validation
//!        │
//!        ▼
//! semantic analysis
//!        │
//!        ▼
//! semantic model
//!        │
//!        ▼
//! ZUIR
//! ```
//!
//! No reverse dependency is permitted.
//!
//! In particular:
//!
//! ```text
//! paths::mod
//!     ✗ semantic
//!     ✗ ZUIR
//!     ✗ quantum::ir
//!     ✗ hardware
//!     ✗ routing
//!     ✗ scheduling
//!     ✗ runtime
//!     ✗ backend
//! ```
//!
//! ## Public API stability
//!
//! The module-level re-exports form the stable consumer-facing path API.
//!
//! Implementation files may evolve internally without forcing every consumer
//! to depend on their physical module location.
//!
//! This is especially important for the POCO-REAF architecture: downstream
//! users of the AST should depend on semantic contracts rather than internal
//! file layout.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser may construct:
//!
//! - `PathSegment`;
//! - `Path`;
//! - import structures;
//!
//! and pass them into the AST.
//!
//! The parser must not use this module to perform:
//!
//! - symbol lookup;
//! - filesystem lookup;
//! - backend selection;
//! - hardware discovery.
//!
//! ### AST nodes
//!
//! Expressions, declarations, types, patterns, annotations, and imports may
//! consume `Path` through this public module boundary.
//!
//! ### Structural validation
//!
//! Structural validation consumes the exported path structures and validates
//! their AST invariants.
//!
//! It must not resolve names.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes paths and resolves them against the appropriate
//! symbol/module/import environment.
//!
//! Resolution results must live outside the native source AST.
//!
//! ### Serialization
//!
//! AST serialization may serialize the public path structures through their
//! canonical Serde implementations.
//!
//! Global AST schema/version policy remains owned by the AST serialization
//! subsystem.
//!
//! ### Visitors and traversal
//!
//! Path traversal is owned by the visitor/traversal infrastructure.
//!
//! This module only exposes the nodes required by that infrastructure.
//!
//! ### ZUIR
//!
//! ZUIR lowering consumes semantic meaning produced from paths.
//!
//! `paths::mod` itself has no ZUIR dependency.
//!
//! ## Migration contract
//!
//! The repository currently contains both `path.rs` and `segment.rs` path
//! representations. The correct migration is:
//!
//! ```text
//! existing Path implementation
//!          │
//!          ├──► canonical Path
//!          │
//! existing PathSegment implementations
//!          │
//!          └──► canonical PathSegment
//!                    │
//!                    ▼
//!                paths/mod.rs
//!                    │
//!                    ▼
//!              all consumers
//! ```
//!
//! `mod.rs` must not preserve competing public definitions merely to avoid
//! fixing consumers. Consumers should be migrated to the canonical exports.
//!
//! This prevents long-term divergence between path representations.
//!
//! ## No hidden compatibility layer
//!
//! Do not introduce:
//!
//! ```text
//! LegacyPath
//! OldPath
//! PathV2
//! GenericPath
//! QuantumPath
//! TypePathAlias
//! ```
//!
//! solely to conceal an incomplete migration.
//!
//! Compatibility belongs at explicit language/schema migration boundaries,
//! not inside this aggregation module.
//!
//! ## Error ownership
//!
//! This module introduces no independent path error type.
//!
//! Errors associated with construction or structural validation remain owned by
//! the implementation that defines the corresponding invariant.
//!
//! Semantic errors remain owned by semantic analysis.
//!
//! Parser diagnostics remain owned by parser/frontend diagnostics.
//!
//! ## Determinism
//!
//! Module declaration and re-export introduce no runtime state.
//!
//! There is:
//!
//! - no global mutable state;
//! - no filesystem discovery;
//! - no environment inspection;
//! - no randomness;
//! - no timestamps;
//! - no process identity;
//! - no thread identity;
//! - no backend discovery.
//!
//! Therefore the module boundary itself is deterministic.
//!
//! ## Security
//!
//! This file performs no:
//!
//! - filesystem access;
//! - network access;
//! - process execution;
//! - dynamic loading;
//! - pointer manipulation;
//! - unsafe operations.
//!
//! It is therefore safe to use as a pure AST module boundary.
//!
//! Untrusted input validation remains the responsibility of the parser,
//! structural validator, deserializer, and compiler resource-policy layers.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Why this file is intentionally small
//!
//! `mod.rs` is an architectural boundary, not a dumping ground for AST logic.
//!
//! Keeping implementation out of this file provides:
//!
//! - clear ownership;
//! - smaller compilation units;
//! - easier testing;
//! - stable import paths;
//! - lower coupling;
//! - easier future extension;
//! - simpler repository-wide migration;
//! - protection against another monolithic AST module.
//!
//! The implementation belongs in `path.rs`, `segment.rs`, and `import.rs`.
//!
//! =============================================================================
//! Public module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

mod import;
mod path;
mod segment;

// =============================================================================
// Stable public API
// =============================================================================

pub use import::*;
pub use path::*;
pub use segment::*;

// =============================================================================
// Compile-time integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_module_exposes_canonical_path_type() {
        fn assert_path_type(_: &Path) {}

        let path = Path::empty();
        assert_path_type(&path);
    }

    #[test]
    fn path_module_exposes_canonical_segment_type() {
        fn assert_segment_type(_: &PathSegment) {}

        let segment = PathSegment::from_str("example");
        assert_segment_type(&segment);
    }

    #[test]
    fn implementation_modules_remain_addressable() {
        // These imports intentionally verify that the public API is available
        // through the stable aggregation boundary without depending on private
        // implementation details in downstream code.
        use super::path::Path;
        use super::segment::PathSegment;

        let segment = PathSegment::from_str("example");
        let path = Path::single(segment);

        assert_eq!(path.len(), 1);
    }
}