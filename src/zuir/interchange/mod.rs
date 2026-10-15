#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Interchange
//!
//! Canonical module boundary for importing and exporting representations
//! through explicitly registered, versioned interchange formats.
//!
//! Path: `src/zuir/interchange/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later
//! Safety: Safe Rust only; `unsafe` is forbidden.
//!
//! ## PURPOSE
//!
//! Provide one stable public entry point for ZUIR interchange contracts,
//! format negotiation, importing, and exporting.
//!
//! The interchange layer connects external representations with a
//! caller-selected in-memory representation. It does not define a competing
//! IR, duplicate source parsing, or assume that all external formats can
//! express the same semantics.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/interchange/format.rs`
//! - `src/zuir/interchange/import.rs`
//! - `src/zuir/interchange/export.rs`
//! - `src/zuir/version.rs`, where applicable
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/scalability-model.md`
//!
//! Repository-wide architectural authority remains with `DESIGN.md` and
//! the repository's formally established specification hierarchy.
//!
//! ## OWNS
//!
//! - The public interchange module boundary.
//! - Public re-exports of the supported format, import, and export APIs.
//! - The documented separation of responsibilities between those modules.
//!
//! ## DOES_NOT_OWN
//!
//! - Zamani lexical rules, grammar, parsing, AST construction, or name
//!   resolution.
//! - The canonical ZUIR program, operation, type, or value definitions.
//! - Format-specific encoding or decoding algorithms.
//! - Semantic conversion algorithms for individual external formats.
//! - Schema migration or version compatibility inference.
//! - Domain IR definitions for classical, quantum, HDL, tensor, AI,
//!   distributed, networking, cryptographic, or future computing domains.
//! - Physical hardware discovery, allocation, routing, scheduling,
//!   execution, or runtime behavior.
//!
//! ## DEPENDENCY DIRECTION
//!
//! `format` defines format identities, versions, capabilities, descriptors,
//! and exact negotiation contracts.
//!
//! `import` consumes those contracts to select an import adapter and read
//! an input stream.
//!
//! `export` consumes those contracts to select an export adapter and write
//! an output stream.
//!
//! Future migration functionality belongs in a separate `migration` module.
//! It must be added to this module only after its source file and public API
//! exist and their contracts have been verified.
//!
//! The parent `src/zuir/mod.rs` owns integration into the wider ZUIR crate.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module imposes no universal maximum on the number of formats,
//! versions, capabilities, or registered adapters.
//!
//! Resource limits are explicit operational policies of the relevant
//! import/export operations. Actual address-space, memory, I/O, and
//! processing constraints remain applicable.
//!
//! Format registration order must not determine format selection.
//! Callers must request the intended format and version explicitly.
//!
//! A format's declared capabilities are contractual claims, not proof that
//! the implementation preserves every possible semantic feature. Adapters
//! must reject conversions they cannot perform without unacceptable
//! semantic loss.
//!
//! ## GRAMMAR BOUNDARY
//!
//! The ANTLR grammar, lexer, parser, and source AST belong to the frontend.
//! This module must not import grammar-generated types or parse source text.
//! A source-language importer, if required, must be an explicit adapter that
//! respects the frontend's ownership and semantic-validation contracts.
//!
//! ## SAFETY AND DETERMINISM
//!
//! No unsafe Rust, global mutable registry, implicit format fallback, or
//! registration-order-dependent selection is introduced here.
//!
//! Ordered registries and explicit format/version requirements are used by
//! the implementation modules.
//!
//! ## FREEZE CRITERIA
//!
//! - All declared modules exist and compile.
//! - Public re-exports match the actual implementation APIs.
//! - Duplicate registrations and unsupported format/version requests fail
//!   explicitly in the relevant implementation.
//! - Import/export resource budgets are enforced by their respective
//!   implementations.
//! - Unsupported semantic conversions do not silently discard meaning.
//! - Unit, integration, compatibility, and determinism tests pass.
//! - `cargo fmt`, `cargo check`, and the applicable test suite pass.
//!
//! This module is not evidence of production readiness by itself.
//! Readiness requires successful validation of the actual integrated crate.

// =============================================================================
// Canonical submodules
// =============================================================================

/// Format identities, versions, capability declarations, descriptors,
/// requirements, and exact format negotiation.
pub mod format;

/// Import adapters, import registry, streaming input, and import budgets.
pub mod import;

/// Export adapters, export registry, output budgets, and writer error
/// propagation.
pub mod export;

// =============================================================================
// Public API
// =============================================================================

/// Public format and negotiation contracts.
pub use self::format::{
    negotiate_exact,
    CapabilityId,
    FormatCapabilities,
    FormatDescriptor,
    FormatError,
    FormatId,
    FormatRequirement,
    FormatVersion,
    InterchangeFormat,
};

/// Public import API.
pub use self::import::{
    ImportAdapter,
    ImportContext,
    ImportError,
    ImportRegistry,
    ImportResult,
};

/// Public export API.
pub use self::export::{
    ExportAdapter,
    ExportContext,
    ExportError,
    ExportRegistry,
};

// =============================================================================
// Module-boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_format_contracts_are_available_from_the_module_boundary() {
        let format_id =
            FormatId::new("zamani.zuir").expect("valid format identifier");

        let version = FormatVersion::new(1, 0, 0);

        let descriptor =
            FormatDescriptor::new(format_id.clone(), version);

        let requirement =
            FormatRequirement::new(format_id.clone(), version);

        requirement
            .matches_exact(&descriptor)
            .expect("the exact format and version must match");

        assert_eq!(format_id.as_str(), "zamani.zuir");
        assert_eq!(descriptor.version(), version);
    }

    #[test]
    fn import_and_export_registries_are_available_from_the_module_boundary() {
        let imports = ImportRegistry::<String>::new();
        let exports = ExportRegistry::<String>::new();

        assert!(imports.is_empty());
        assert!(exports.is_empty());
    }

    #[test]
    fn format_capabilities_are_extensible() {
        let capability =
            CapabilityId::new("semantics.quantum")
                .expect("valid capability identifier");

        let mut capabilities = FormatCapabilities::new();

        assert!(capabilities.insert(capability.clone()));
        assert!(!capabilities.insert(capability.clone()));
        assert!(capabilities.contains(&capability));
    }

    #[test]
    fn format_negotiation_does_not_infer_an_unregistered_version() {
        let format_id =
            FormatId::new("zamani.zuir").expect("valid format identifier");

        let requested_version = FormatVersion::new(2, 0, 0);
        let available_version = FormatVersion::new(1, 0, 0);

        let requirement =
            FormatRequirement::new(format_id.clone(), requested_version);

        let candidates = vec![
            FormatDescriptor::new(format_id, available_version),
        ];

        assert!(matches!(
            negotiate_exact(&requirement, &candidates),
            Err(FormatError::UnsupportedFormat { .. })
        ));
    }
}