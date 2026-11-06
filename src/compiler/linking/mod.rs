
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Linking Subsystem
//!
//! Canonical path:
//! `src/compiler/linking/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! Safety:
//! Safe Rust only. Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Establish the canonical public module boundary for compiler linking.
//!
//! This module integrates the existing linking implementations:
//!
//! - `symbols.rs`
//! - `composition.rs`
//! - `relocation.rs`
//!
//! It does not implement another linker, symbol table, module representation,
//! relocation engine, ABI, object-file format, or backend.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This facade owns:
//!
//! - Linking subsystem module composition.
//! - Public visibility of linking components.
//! - Linking API version identification.
//! - Explicit public integration exports.
//! - Linking subsystem integration tests.
//!
//! It does NOT own:
//!
//! - Source-level name resolution.
//! - Semantic symbol declaration.
//! - Canonical symbol identity allocation.
//! - Canonical ZUIR module representation.
//! - Type inference or type compatibility.
//! - ABI or calling-convention definitions.
//! - Object-file parsing or serialization.
//! - Section placement.
//! - Target-specific relocation encoding.
//! - Backend code generation.
//! - Physical hardware allocation.
//! - Executable loading.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ============================================================================
//! CANONICAL REPOSITORY AUTHORITIES
//! ============================================================================
//!
//! Canonical symbol identity and metadata:
//!     crate::zuir::ids
//!     crate::zuir::values::symbols
//!
//! Canonical module representation:
//!     crate::zuir::ir::module
//!
//! Canonical program representation:
//!     crate::zuir::ir::program
//!
//! Link-time symbol resolution:
//!     crate::compiler::linking::symbols
//!
//! Module composition:
//!     crate::compiler::linking::composition
//!
//! Generic relocation:
//!     crate::compiler::linking::relocation
//!
//! Artifact representation:
//!     crate::compiler::artifacts
//!
//! Target-specific code generation:
//!     crate::backend
//!
//! Runtime loading and execution:
//!     crate::runtime
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! ```text
//!
//!                compiler/linking/mod.rs
//!                          |
//!             +------------+------------+
//!             |            |            |
//!             v            v            v
//!          symbols.rs  composition.rs relocation.rs
//!             |            |            |
//!             |            +----+-------+
//!             |                 |
//!             v                 v
//!        ZUIR symbols       ZUIR modules
//!             |
//!             v
//!       Symbol resolution
//!
//! ```
//!
//! The facade exposes the components but does not become their implementation
//! dependency. Child modules should use explicit sibling paths.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Linking must preserve the meaning of canonical program representations
//! independently of the target architecture or computational domain.
//!
//! This subsystem must accommodate independently compiled units containing:
//!
//! - Classical computation.
//! - Quantum computation.
//! - Classical-quantum hybrid computation.
//! - AI and tensor computation.
//! - HDL and hardware computation.
//! - Distributed computation.
//! - Networking.
//! - Cryptographic computation.
//! - Accelerated computation.
//! - Future registered computational domains.
//!
//! These are examples, not a closed inventory.
//!
//! The linking facade must not introduce target-specific closed inventories.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - Symbols.
//! - Imports.
//! - Exports.
//! - Relocations.
//! - Sections.
//! - Computing domains.
//! - Architectures.
//! - Devices.
//! - Accelerators.
//! - Memory capacity.
//! - Register width.
//! - Tensor rank.
//! - Quantum resources.
//!
//! The underlying collections and operations remain subject to available
//! memory, address space, processing time, operating-system limits, and
//! explicitly configured operational budgets.
//!
//! Literal infinity cannot be guaranteed by a finite implementation.
//!
//! An operational budget must never be presented as a language-level or
//! universal hardware limit.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The child implementations provide their respective deterministic
//! behavior and validation guarantees.
//!
//! This facade does not reorder, mutate, or reinterpret linking inputs.
//!
//! Public API exposure must not introduce nondeterministic behavior.
//!
//! ============================================================================
//! TRANSACTIONAL AND SECURITY CONTRACT
//! ============================================================================
//!
//! Transactional behavior belongs to the operations that perform mutations:
//!
//! - Symbol insertion validates before updating its indexes.
//! - Module composition stages its result before returning success.
//! - Relocation application validates and computes patches before committing.
//!
//! The facade must not bypass these checks.
//!
//! No component exposed here is permission to execute untrusted code.
//!
//! Symbol resolution, module composition, and relocation do not by themselves
//! establish that an artifact is safe to load or execute.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! Source-level module syntax is defined by the grammar subsystem.
//!
//! Relevant grammar authorities include:
//!
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/modules/modules.g4
//! - grammar/modules/imports.g4
//! - grammar/modules/exports.g4
//! - grammar/modules/dependencies.g4
//! - grammar/interoperability/linkage.g4
//!
//! The grammar defines source syntax.
//! The frontend constructs the AST.
//! Semantic analysis resolves source-level declarations and imports.
//! ZUIR represents the resulting canonical modules and symbols.
//! This linking subsystem consumes those resolved representations.
//!
//! This facade must never parse source text or interpret grammar productions.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `symbols.rs`:
//!     Owns LinkUnitId, SymbolDefinition, SymbolIndex, SymbolIndexBudget,
//!     and SymbolLinkError.
//!
//! `composition.rs`:
//!     Owns CompositionUnit, CompositionPolicy, ModuleComposition,
//!     and CompositionError.
//!
//! `relocation.rs`:
//!     Owns relocation identities, field descriptions, relocation records,
//!     address-provider contracts, relocation validation, and application.
//!
//! `compiler/mod.rs`:
//!     Must expose this subsystem using `pub mod linking;`.
//!
//! `compiler/artifacts/`:
//!     Supplies artifact representations and metadata.
//!
//! `backend/`:
//!     Supplies target-specific lowering and encoding integrations.
//!
//! `runtime/`:
//!     Owns loading and execution.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing child module paths are public:
//!
//!     crate::compiler::linking::symbols
//!     crate::compiler::linking::composition
//!     crate::compiler::linking::relocation
//!
//! They are intentionally not flattened into this facade's root namespace.
//!
//! Adding a future child module must not silently change existing public APIs.
//! Public re-exports must be deliberate and documented.
//!
//! ============================================================================
//! FREEZE CONDITIONS
//! ============================================================================
//!
//! - Every declared child module exists.
//! - Every child module compiles.
//! - Existing child module paths remain stable.
//! - No duplicate linking authority is introduced.
//! - No unsafe Rust is introduced.
//! - No arbitrary hardware ceilings are introduced.
//! - Linking tests pass.
//! - Repository integration tests pass.
//! - Rust 1.97 or later is supported.

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compiler linking subsystem.
///
/// This version describes the facade contract. It is independent of the
/// Zamani language, ZUIR, artifact, ABI, and backend versions.
pub const LINKING_API_VERSION: &str = "1.0.0";

// ============================================================================
// CANONICAL LINKING COMPONENTS
// ============================================================================

/// Link-time symbol indexing and symbol resolution.
///
/// This module uses canonical ZUIR symbol definitions and identities.
pub mod symbols;

/// Composition of independently compiled canonical ZUIR modules.
///
/// This module preserves module boundaries and validates cross-module
/// imports, exports, and symbol relationships.
pub mod composition;

/// Generic relocation representation and safe relocation application.
///
/// Target-specific relocation encoding remains with the appropriate
/// backend or object-format implementation.
pub mod relocation;

// ============================================================================
// DELIBERATE PUBLIC API RE-EXPORTS
// ============================================================================

/// Common module-composition interfaces.
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

/// Common symbol-resolution interfaces.
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

/// Relocation APIs remain available through the dedicated `relocation`
/// module. This avoids flattening target-sensitive relocation types into
/// the general linking namespace.
pub mod relocation_api {
    pub use super::relocation::*;
}

// ============================================================================
// FACADE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_api_version_is_present() {
        assert!(!LINKING_API_VERSION.trim().is_empty());
    }

    #[test]
    fn child_api_versions_are_present() {
        assert!(!symbols::LINKING_SYMBOLS_API_VERSION.trim().is_empty());
        assert!(!composition::COMPOSITION_API_VERSION.trim().is_empty());
        assert!(!relocation::RELOCATION_API_VERSION.trim().is_empty());
    }

    #[test]
    fn public_component_modules_are_accessible() {
        // These type references verify that the facade exposes the intended
        // existing implementations without defining competing types.
        let _: Option<symbols::LinkUnitId> = None;
        let _: Option<composition::CompositionPolicy> = None;
        let _: Option<relocation::RelocationKindId> = None;
    }
}
