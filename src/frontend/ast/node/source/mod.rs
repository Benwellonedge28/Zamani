//! Canonical source infrastructure for the Zamani frontend AST.
//!
//! This module is the public aggregation boundary for source-related
//! information attached to the native Zamani AST.
//!
//! # Architectural role
//!
//! The source subsystem provides the compiler with stable, source-oriented
//! information without coupling the AST to:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - hardware topology;
//! - qubit counts;
//! - gate sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime execution;
//! - backend vendors;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - operating-system resources.
//!
//! The intended dependency direction is:
//!
//! ```text
//! source text
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! frontend::ast::node::source
//!     │
//!     ├── SourceId
//!     ├── SourceOffset
//!     ├── Span
//!     ├── SourceLocation
//!     └── SourceOrigin
//!     │
//!     ▼
//! AST nodes
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//!     │
//!     ▼
//! hardware
//! ```
//!
//! The source subsystem is therefore a foundational dependency and must not
//! depend on any later compilation stage.
//!
//! # POCO-REAF
//!
//! Source information is deliberately independent of computational scale.
//!
//! Nothing in this module assumes:
//!
//! - a particular number of AST nodes;
//! - a particular number of source files;
//! - a particular program size;
//! - a particular number of modules;
//! - a particular number of qubits;
//! - a particular machine size;
//! - a particular hardware topology;
//! - a particular backend;
//! - a particular quantum technology.
//!
//! A tiny program and an extremely large program use the same source model.
//!
//! "Infinity" in the Zamani architecture means that this layer introduces no
//! artificial finite language or machine-size bound. Actual compilation is,
//! of course, bounded by the representable coordinate domain and resources
//! available to the compiler process.
//!
//! Operational limits must therefore be configured by compiler/source-map
//! policy rather than encoded as language semantics in these source types.
//!
//! # Coordinate model
//!
//! The canonical source coordinate model is:
//!
//! ```text
//! SourceId + UTF-8 byte offset
//! ```
//!
//! `Span` uses the half-open interval:
//!
//! ```text
//! [start, end)
//! ```
//!
//! `SourceLocation` provides derived human-readable coordinates for diagnostics
//! and tooling.
//!
//! The canonical coordinate must remain byte-based. Line, column, UTF-16
//! offsets, grapheme positions, editor coordinates, and other presentation
//! coordinate systems are derived representations and must not replace the
//! canonical span representation.
//!
//! # Source identity versus source metadata
//!
//! A source identity identifies a source unit.
//!
//! It must not itself encode:
//!
//! - a filesystem path;
//! - URL;
//! - source contents;
//! - timestamp;
//! - memory address;
//! - file descriptor;
//! - operating-system handle;
//! - authorization token;
//! - backend identifier.
//!
//! Such information belongs to the source registry/source-map layer.
//!
//! # Module responsibilities
//!
//! ## [`source_id`]
//!
//! Provides stable source-unit identity.
//!
//! ## [`span`]
//!
//! Provides canonical source ranges and offsets.
//!
//! ## [`location`]
//!
//! Provides human-readable source positions derived from the canonical
//! source-coordinate model.
//!
//! ## [`source_origin`]
//!
//! Describes where a source/AST construct originated, such as source text,
//! generated code, macro expansion, imported representation, or another
//! explicitly supported provenance category.
//!
//! # Public API policy
//!
//! This module intentionally acts as an aggregation boundary.
//!
//! Consumers outside the source subsystem should normally import source types
//! through this module rather than reaching into implementation files:
//!
//! ```text
//! frontend::ast::node::source::Span
//! frontend::ast::node::source::SourceId
//! frontend::ast::node::source::SourceOffset
//! frontend::ast::node::source::SourceLocation
//! frontend::ast::node::source::SourceOrigin
//! ```
//!
//! This keeps internal file organization replaceable without forcing parser,
//! AST, diagnostic, semantic, or tooling code to depend on individual source
//! implementation files.
//!
//! # Dependency contract
//!
//! The source subsystem may depend on:
//!
//! - Rust standard-library facilities;
//! - Serde, where required by the existing source value types.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend implementations.
//!
//! # Integration contract
//!
//! ```text
//! source_id.rs ──────────────┐
//!                            │
//! span.rs ───────────────────┼──► source/mod.rs ──► AST nodes
//!                            │                         │
//! location.rs ───────────────┤                         ├──► diagnostics
//!                            │                         ├──► parser
//! source_origin.rs ──────────┘                         ├──► validation
//!                                                      ├──► visitors
//!                                                      ├──► serialization
//!                                                      ├──► semantic analysis
//!                                                      └──► ZUIR provenance
//! ```
//!
//! The aggregation module itself contains no source-state storage and performs
//! no source registration. It only establishes the public module boundary.
//!
//! # Important compatibility note
//!
//! The repository currently contains `SourceId` definitions in both the
//! standalone `source_id.rs` implementation and the existing `span.rs`
//! implementation. `location.rs` currently consumes the `SourceId` and
//! `SourceOffset` exported by `span.rs`.
//!
//! This module deliberately does **not** silently create another `SourceId`
//! representation or pretend those two existing definitions are identical.
//!
//! The canonicalization sequence is:
//!
//! ```text
//! 1. source_id.rs
//!       ↓
//! 2. span.rs uses source_id::SourceId
//!       ↓
//! 3. location.rs consumes the canonical source types
//!       ↓
//! 4. source/mod.rs publicly re-exports the canonical types
//! ```
//!
//! Until step 2 is completed, compatibility aliases are exposed explicitly
//! rather than silently conflating distinct Rust types.
//!
//! This prevents downstream code from accidentally compiling against the
//! wrong source-identity type and makes the migration mechanically verifiable.
//!
//! # Scalability
//!
//! This module introduces no collections, global state, counters, locks,
//! registries, caches, or fixed capacities.
//!
//! Consequently:
//!
//! - module loading scale is determined by the source registry;
//! - AST scale is determined by the AST representation;
//! - source-coordinate scale is determined by the coordinate types;
//! - compiler resource limits remain configurable outside this module.
//!
//! # Determinism
//!
//! Importing this module has no runtime side effects.
//!
//! It does not:
//!
//! - allocate IDs;
//! - access the filesystem;
//! - read environment variables;
//! - use randomness;
//! - access clocks;
//! - depend on thread scheduling;
//! - initialize global mutable state.
//!
//! The source subsystem therefore remains compatible with reproducible and
//! deterministic compilation.
//!
//! # Security
//!
//! Source values are compiler input and may ultimately originate from
//! untrusted programs.
//!
//! This module itself performs no unchecked memory access and contains no
//! `unsafe` code.
//!
//! Validation of source coordinates against actual source buffers belongs to
//! the source-map/validation layer.
//!
//! Malformed spans must never be allowed to cause:
//!
//! - integer overflow;
//! - out-of-bounds slicing;
//! - memory corruption;
//! - uncontrolled allocation;
//! - compiler panics.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021.
//!
//! No nightly features are required.
//!
//! # Public re-exports
//!
//! The explicit re-exports below are the stable integration surface.
//!
//! Keep implementation modules public enough for internal crate integration,
//! while encouraging external consumers to use this module's re-exported API.

pub mod location;
pub mod source_id;
pub mod source_origin;
pub mod span;

// -----------------------------------------------------------------------------
// Canonical public source API
// -----------------------------------------------------------------------------
//
// The current repository state has the source-coordinate implementation in
// `span.rs`, while `source_id.rs` independently provides the dedicated source
// identity abstraction. Until `span.rs` is migrated to consume
// `source_id::SourceId`, exporting both under the same public name would be
// incorrect because Rust treats them as distinct types.
//
// Therefore the current compatibility surface is intentionally explicit.
//
// `SourceId` below is the identity type currently used by `Span` and
// `SourceLocation`. The dedicated standalone implementation is temporarily
// exposed as `SourceUnitId`.
//
// The eventual canonical state should remove this compatibility distinction
// after `span.rs` is migrated:
//
//     pub use source_id::SourceId;
//     pub use span::{Span, SourceOffset};
// -----------------------------------------------------------------------------

pub use location::SourceLocation;
pub use source_id::{
    SourceId as SourceUnitId,
    SourceIdConversionError,
};
pub use source_origin::{
    SourceOrigin,
    SourceOriginKind,
};
pub use span::{
    SourceId,
    SourceOffset,
    Span,
    SpanError,
};

// -----------------------------------------------------------------------------
// Stable semantic aliases
// -----------------------------------------------------------------------------
//
// These aliases describe intent without creating additional Rust types.
// They are useful at integration boundaries where the distinction between a
// source unit identity and a source range matters.
//
// They intentionally remain aliases rather than wrapper structs so they do not
// create unnecessary conversion boundaries or duplicate representations.

/// Stable identity of the source unit associated with a source coordinate.
///
/// This alias follows the currently canonical `Span` representation.
/// During the source-ID consolidation migration, `SourceId` will become a
/// direct re-export of `source_id::SourceId`.
pub type SourceCoordinateSourceId = SourceId;

/// Canonical byte offset used by source spans and locations.
pub type SourceByteOffset = SourceOffset;

// -----------------------------------------------------------------------------
// Compile-time API assertions
// -----------------------------------------------------------------------------
//
// These assertions intentionally remain small. The source module is an
// aggregation boundary and must not duplicate the implementation tests already
// owned by its child modules.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_source_api_is_reachable() {
        let source = SourceId::from_raw(0);
        let offset = SourceOffset::from_raw(0);
        let span = Span::point(source, offset);

        assert_eq!(span.source(), source);
        assert_eq!(span.start(), offset);
        assert_eq!(span.end(), offset);
        assert!(span.is_empty());
    }

    #[test]
    fn public_source_coordinate_aliases_are_type_preserving() {
        let source: SourceCoordinateSourceId = SourceId::from_raw(7);
        let offset: SourceByteOffset = SourceOffset::from_raw(11);

        let span = Span::point(source, offset);

        assert_eq!(span.source().as_raw(), 7);
        assert_eq!(span.start().as_raw(), 11);
        assert_eq!(span.end().as_raw(), 11);
    }

    #[test]
    fn source_unit_identity_is_explicitly_distinct_during_migration() {
        let source_unit = SourceUnitId::from_raw(7);

        assert_eq!(source_unit.as_raw(), 7);
    }

    #[test]
    fn empty_source_span_is_valid() {
        let source = SourceId::from_raw(0);
        let span = Span::point(source, SourceOffset::from_raw(0));

        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
    }

    #[test]
    fn source_module_has_no_machine_specific_semantics() {
        // This test intentionally verifies only the public source-coordinate
        // model. Machine/hardware semantics must not appear in this module.
        let source = SourceId::from_raw(u64::MAX);
        let offset = SourceOffset::from_raw(u64::MAX);

        let span = Span::point(source, offset);

        assert_eq!(span.source().as_raw(), u64::MAX);
        assert_eq!(span.start().as_raw(), u64::MAX);
        assert_eq!(span.end().as_raw(), u64::MAX);
    }
}