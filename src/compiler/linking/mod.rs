
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Canonical Linking Subsystem
//!
//! Path: src/compiler/linking/mod.rs
//!
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//!
//! # Purpose
//!
//! Establish the canonical public integration boundary for compiler linking.
//!
//! This module coordinates the existing linking components:
//!
//! - symbols.rs
//! - composition.rs
//! - relocation.rs
//! - validation.rs
//!
//! It does not implement another linker, symbol table, IR, relocation
//! engine, ABI, object-file format, backend, or runtime.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Linking subsystem composition.
//! - Public linking API boundaries.
//! - Linking API version.
//! - Explicit integration exports.
//! - Linking facade conformance tests.
//!
//! It does not own:
//!
//! - Source-level name resolution.
//! - Semantic declaration analysis.
//! - Canonical symbol identity allocation.
//! - Canonical ZUIR representation.
//! - Type inference or semantic type checking.
//! - ABI and calling conventions.
//! - Object-file parsing and serialization.
//! - Section placement.
//! - Target-specific relocation encoding.
//! - Backend code generation.
//! - Physical hardware allocation.
//! - Executable loading.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Symbol identity and metadata:
//!     crate::zuir::ids
//!     crate::zuir::values::symbols
//!
//! Canonical modules:
//!     crate::zuir::ir::module
//!
//! Canonical programs:
//!     crate::zuir::ir::program
//!
//! Symbol indexing:
//!     crate::compiler::linking::symbols
//!
//! Module composition:
//!     crate::compiler::linking::composition
//!
//! Relocation:
//!     crate::compiler::linking::relocation
//!
//! Cross-input validation:
//!     crate::compiler::linking::validation
//!
//! Artifact metadata and representation:
//!     crate::compiler::artifacts
//!
//! Target-specific code generation:
//!     crate::backend
//!
//! Loading and execution:
//!     crate::runtime
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Linking must preserve canonical program meaning independently of
//! target architecture, computational domain, or deployment topology.
//!
//! Supported computational domains are open-ended.
//!
//! Examples include:
//!
//! - Classical computation.
//! - Quantum computation.
//! - Hybrid quantum-classical computation.
//! - AI and tensor computation.
//! - HDL and hardware computation.
//! - Distributed computation.
//! - Networking.
//! - Cryptographic computation.
//! - Accelerated computation.
//! - Future registered computational domains.
//!
//! These examples are not a closed target or domain inventory.
//!
//! # Scalability
//!
//! This facade imposes no arbitrary maximum on:
//!
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - Symbols.
//! - Imports.
//! - Exports.
//! - Relocations.
//! - Sections.
//! - Computational domains.
//! - Architectures.
//! - Devices.
//! - Accelerators.
//! - Memory capacity.
//! - Hardware resource counts.
//!
//! The underlying implementations are subject to available memory,
//! address space, processing time, operating-system constraints,
//! implementation representation and explicitly configured budgets.
//!
//! Literal infinity cannot be guaranteed by a finite implementation.
//!
//! Resource budgets are operational controls, not language-level limits.
//!
//! # Dependency direction
//!
//! ```text
//!                  compiler/linking
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!       symbols       composition    relocation
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!                     validation
//!                         |
//!                         v
//!                 linking integration
//!                         |
//!             +-----------+-----------+
//!             |                       |
//!          artifacts                backend
//!             |                       |
//!             +-----------+-----------+
//!                         |
//!                  target realization
//!                         |
//!                       runtime
//! ```
//!
//! Child modules must use explicit sibling paths rather than depending
//! on facade re-exports.
//!
//! # Grammar boundary
//!
//! Source-level module, import, export and linkage syntax belongs to:
//!
//! - grammar/modules/modules.g4
//! - grammar/modules/imports.g4
//! - grammar/modules/exports.g4
//! - grammar/modules/dependencies.g4
//! - grammar/interoperability/linkage.g4
//!
//! The grammar defines syntax, not linking behavior.
//!
//! The frontend constructs the AST.
//! Semantic analysis resolves source-level declarations.
//! Canonical IR represents resolved modules and symbols.
//! This subsystem consumes those canonical representations.
//!
//! No grammar parsing or source-level semantic resolution belongs here.
//!
//! # Transactional behavior
//!
//! The facade must not bypass the guarantees of its components.
//!
//! Symbol indexing validates before updating its indexes.
//! Composition stages results before returning success.
//! Relocation validates and computes patches before committing mutations.
//! Cross-input validation does not mutate its inputs.
//!
//! # Security
//!
//! Successful symbol resolution or module composition does not establish
//! that an executable artifact is safe to load or execute.
//!
//! Artifact integrity, provenance, trust verification, security policy,
//! backend validation and runtime loading remain separate responsibilities.
//!
//! # Compatibility
//!
//! Existing public child paths are preserved:
//!
//! - compiler::linking::symbols
//! - compiler::linking::composition
//! - compiler::linking::relocation
//! - compiler::linking::validation
//!
//! Existing child APIs are not replaced by duplicate facade types.
//!
//! # Integration requirements
//!
//! compiler/mod.rs must declare:
//!
//!     pub mod linking;
//!
//! backend/linker.rs must delegate to the authoritative linker
//! implementation through a separately established compatibility API.
//!
//! The facade does not manufacture missing linker implementations.
//!
//! # Completion conditions
//!
//! - Every child module exists.
//! - Every child module compiles.
//! - All four components are publicly accessible.
//! - Existing child paths remain stable.
//! - No duplicate linking authority is introduced.
//! - No unsafe Rust is introduced.
//! - No arbitrary hardware ceilings are introduced.
//! - Facade tests pass.
//! - Repository integration tests pass.

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compiler linking subsystem.
///
/// Independent of the Zamani language, ZUIR, artifact, ABI and backend
/// version numbers.
pub const LINKING_API_VERSION: &str = "1.0.0";

// ============================================================================
// CANONICAL IMPLEMENTATION MODULES
// ============================================================================

/// Link-time symbol indexing and resolution.
///
/// Owns compilation-unit symbol associations, symbol indexing,
/// duplicate-definition detection and external symbol resolution.
pub mod symbols;

/// Composition of independently compiled canonical ZUIR modules.
///
/// Owns composition-unit association and composition-level validation.
pub mod composition;

/// Generic relocation representation and safe relocation application.
///
/// Target-specific relocation encoding remains with the appropriate
/// backend or object-format implementation.
pub mod relocation;

/// Cross-input linking validation.
///
/// Owns validation of module relationships, symbol ownership,
/// imports, exports and relocation references.
///
/// This module consumes the canonical symbol and relocation implementations.
/// It does not introduce duplicate implementations of those components.
pub mod validation;

// ============================================================================
// COMPOSITION API
// ============================================================================

/// Explicit public API for module composition.
///
/// This namespace preserves the implementation ownership of composition.rs.
pub mod composition_api {
    pub use super::composition::{
        CompositionError,
        CompositionPolicy,
        CompositionResult,
        CompositionUnit,
        ModuleComposition,
        COMPOSITION_API_VERSION,
    };
}

// ============================================================================
// SYMBOL API
// ============================================================================

/// Explicit public API for link-time symbol resolution.
///
/// Canonical symbol metadata remains owned by ZUIR.
pub mod symbol_api {
    pub use super::symbols::{
        LinkUnitId,
        SymbolDefinition,
        SymbolIndex,
        SymbolIndexBudget,
        SymbolLinkError,
        SymbolLinkResult,
        LINKING_SYMBOLS_API_VERSION,
    };
}

// ============================================================================
// RELOCATION API
// ============================================================================

/// Explicit public API for generic relocation.
///
/// Target-specific relocation encoding is deliberately excluded.
pub mod relocation_api {
    pub use super::relocation::{
        apply_relocation,
        apply_relocations,
        Relocation,
        RelocationAddressProvider,
        RelocationByteOrder,
        RelocationError,
        RelocationExpression,
        RelocationField,
        RelocationKindId,
        RelocationOverflowPolicy,
        RelocationResult,
        RelocationSectionId,
        RelocationSignedness,
        RELOCATION_API_VERSION,
    };
}

// ============================================================================
// VALIDATION API
// ============================================================================

/// Explicit public API for cross-input linking validation.
///
/// Validation is separate from symbol-index construction, module
/// composition and relocation application.
pub mod validation_api {
    pub use super::validation::{
        validate_modules,
        validate_relocations,
        LinkValidationError,
        LinkValidationModule,
        LinkValidationPolicy,
        LinkValidationReport,
        LinkValidationResult,
        UnresolvedImport,
        LINK_VALIDATION_API_VERSION,
    };
}

// ============================================================================
// FACADE CONFORMANCE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_api_version_is_present() {
        assert_eq!(LINKING_API_VERSION, "1.0.0");
    }

    #[test]
    fn every_component_has_an_api_version() {
        assert!(!symbols::LINKING_SYMBOLS_API_VERSION.is_empty());
        assert!(!composition::COMPOSITION_API_VERSION.is_empty());
        assert!(!relocation::RELOCATION_API_VERSION.is_empty());
        assert!(!validation::LINK_VALIDATION_API_VERSION.is_empty());
    }

    #[test]
    fn public_component_modules_are_accessible() {
        let _: Option<symbols::LinkUnitId> = None;
        let _: Option<composition::CompositionPolicy> = None;
        let _: Option<relocation::RelocationKindId> = None;
        let _: Option<validation::LinkValidationPolicy> = None;
    }

    #[test]
    fn public_api_namespaces_are_accessible() {
        let _: Option<symbol_api::SymbolIndex> = None;
        let _: Option<composition_api::ModuleComposition<()>> = None;
        let _: Option<relocation_api::Relocation> = None;
        let _: Option<validation_api::LinkValidationReport> = None;
    }

    #[test]
    fn validation_api_exposes_module_validation() {
        let _: fn(
            std::iter::Empty<
                validation::LinkValidationModule<'static, ()>
            >,
            &symbols::SymbolIndex,
            validation::LinkValidationPolicy,
        ) -> validation::LinkValidationResult<
            validation::LinkValidationReport
        > = validation::validate_modules;
    }

    #[test]
    fn validation_api_exposes_relocation_validation() {
        let _: fn(
            std::iter::Empty<&'static relocation::Relocation>,
            &symbols::SymbolIndex,
        ) -> validation::LinkValidationResult<usize> =
            validation::validate_relocations;
    }
}
