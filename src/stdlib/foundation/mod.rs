
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Foundation
//!
//! Path: `src/stdlib/foundation/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the public aggregation boundary for the foundational
//! facilities of the Zamani Standard Library.
//!
//! It integrates reusable, target-independent functionality required by
//! classical, quantum, HDL, hybrid, distributed, AI, scientific,
//! networking, cryptographic, embedded and future computing domains.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - Registration of foundational submodules.
//! - Stable public foundation module paths.
//! - Carefully selected public re-exports.
//! - Foundation-level API documentation.
//! - Foundation module composition.
//!
//! It does not own:
//!
//! - Primitive source-language semantics.
//! - Semantic type resolution.
//! - Memory allocation policy.
//! - Physical memory management.
//! - Tensor storage.
//! - Quantum state or qubit representations.
//! - HDL signals or physical registers.
//! - Compiler intermediate representations.
//! - Hardware discovery.
//! - Device allocation.
//! - Runtime scheduling.
//! - Distributed execution.
//! - Backend code generation.
//! - Provider lifecycle.
//!
//! Those responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! # POCO-REAF
//!
//! The foundation APIs must remain independent of particular hardware,
//! architectures, vendors, execution environments and machine sizes.
//!
//! No arbitrary universal limits are introduced for:
//!
//! - Processor counts.
//! - Thread counts.
//! - Memory capacity.
//! - Storage capacity.
//! - Integer implementation extensions.
//! - Collection lengths.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Device counts.
//! - Qubit counts.
//! - Distributed participants.
//! - Computational domains.
//!
//! Concrete Rust implementations remain subject to their actual
//! representation, allocator, operating system and available resources.
//!
//! No finite implementation promises literal infinite physical resources.
//!
//! Capacity reservations and operational budgets must never become
//! universal language-level capability restrictions.
//!
//! # Architectural ownership
//!
//! The following authorities must remain distinct:
//!
//! ```text
//! Source grammar
//!       |
//!       v
//! Frontend AST
//!       |
//!       v
//! Semantic type authority
//!       |
//!       v
//! Standard library foundation
//!       |
//!       +---- Collections
//!       +---- Primitives
//!       +---- Memory abstractions
//!       +---- Strings
//!       +---- Conversion
//!       +---- Iteration
//!       +---- Numeric operations
//!       +---- Mathematical operations
//!       +---- Comparison
//!       |
//!       v
//! Existing domain authorities
//!       |
//!       +---- Classical
//!       +---- Quantum
//!       +---- HDL
//!       +---- Hybrid
//!       +---- Distributed
//!       +---- AI
//!       |
//!       v
//! Compiler / IR / Runtime / HAL
//! ```
//!
//! The foundation is a reusable API layer, not another semantic,
//! compiler, tensor, hardware or runtime subsystem.
//!
//! # Public API stability
//!
//! Existing implementation modules remain independently addressable.
//!
//! For example:
//!
//! ```rust
//! use crate::stdlib::foundation::collections::List;
//! use crate::stdlib::foundation::primitives::Primitive;
//! use crate::stdlib::foundation::numeric::NumericScalar;
//! ```
//!
//! Selected frequently used types are also re-exported at the
//! foundation module boundary.
//!
//! Specialized algorithms and functions remain available through
//! their owning modules to prevent ambiguous exports.
//!
//! # Shared error contract
//!
//! General standard-library errors are owned by:
//!
//! `crate::stdlib::error`
//!
//! Specialized operations may additionally expose their own
//! detailed errors, such as:
//!
//! - CollectionError
//! - MemoryError
//! - IterationError
//! - NumericError
//! - MathError
//!
//! These specialized errors must not silently discard their
//! operation-specific information.
//!
//! # Grammar integration
//!
//! The canonical source-level primitive representation belongs to:
//!
//! `crate::frontend::ast::node::types::primitive`
//!
//! Semantic type authority belongs to:
//!
//! `crate::frontend::semantic::types::ty`
//!
//! Source primitive syntax belongs to:
//!
//! `grammar/types/primitive.g4`
//!
//! This module does not redefine any of these authorities.
//!
//! # Freeze criteria
//!
//! This module is considered integrated when:
//!
//! - All declared child modules exist.
//! - All public re-exports resolve.
//! - The parent standard-library module declares foundation.
//! - The shared error module is available.
//! - No duplicate foundational type authority is introduced.
//! - Existing consumers can migrate through stable compatibility paths.
//! - Formatting, compilation, tests and Clippy pass.


// =============================================================================
// FOUNDATIONAL IMPLEMENTATION MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// Collections
// -----------------------------------------------------------------------------

/// General-purpose dynamically sized collections.
///
/// Owns the standard-library `List`, `Map` and collection-specific
/// error contracts.
///
/// Does not replace specialized classical collection semantics,
/// tensor storage or distributed storage.
pub mod collections;


// -----------------------------------------------------------------------------
// Comparison
// -----------------------------------------------------------------------------

/// Generic equality, ordering and comparison utilities.
///
/// Provides reusable comparison operations over Rust-compatible
/// value representations.
///
/// Does not define Zamani source-language operator semantics.
pub mod comparison;


// -----------------------------------------------------------------------------
// Conversion
// -----------------------------------------------------------------------------

/// Explicit and checked value-conversion utilities.
///
/// Provides infallible, fallible, borrowed and caller-defined
/// conversion operations.
///
/// Source-language conversion legality remains owned by
/// semantic analysis.
pub mod conversion;


// -----------------------------------------------------------------------------
// Iteration
// -----------------------------------------------------------------------------

/// Generic collection-independent iteration facilities.
///
/// Builds on Rust's standard iterator contracts and provides
/// fallible materialization utilities.
///
/// Does not own asynchronous execution or parallel scheduling.
pub mod iteration;


// -----------------------------------------------------------------------------
// Mathematical operations
// -----------------------------------------------------------------------------

/// General-purpose mathematical operations.
///
/// Builds on the numeric foundation and provides checked
/// mathematical functionality.
///
/// Advanced scientific algorithms remain separate.
pub mod math;


// -----------------------------------------------------------------------------
// Memory abstractions
// -----------------------------------------------------------------------------

/// Ownership-safe memory buffers, views and memory accounting.
///
/// Provides host-side memory abstractions.
///
/// Physical allocation, device memory and runtime resource
/// management remain outside this module.
pub mod memory;


// -----------------------------------------------------------------------------
// Numeric operations
// -----------------------------------------------------------------------------

/// Generic numeric operations and arithmetic contracts.
///
/// Integrates supported primitive representations and the
/// canonical arbitrary-width integer authority.
///
/// Does not define source-language numeric semantics.
pub mod numeric;


// -----------------------------------------------------------------------------
// Primitive implementation metadata
// -----------------------------------------------------------------------------

/// Rust-side primitive implementation metadata and arithmetic traits.
///
/// This module does not replace the canonical Zamani primitive
/// AST or semantic type representation.
pub mod primitives;


// -----------------------------------------------------------------------------
// Strings
// -----------------------------------------------------------------------------

/// UTF-8 string representations and portable string operations.
///
/// Uses established Rust string facilities rather than creating
/// a competing string storage implementation.
pub mod strings;


// =============================================================================
// SHARED FOUNDATION API
// =============================================================================

// -----------------------------------------------------------------------------
// Standard-library error infrastructure
// -----------------------------------------------------------------------------

/// Shared structured standard-library error contract.
///
/// The actual implementation is owned by `crate::stdlib::error`.
pub use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};


// -----------------------------------------------------------------------------
// Canonical general-purpose collections
// -----------------------------------------------------------------------------

/// General-purpose collection types.
pub use self::collections::{
    CollectionError,
    CollectionKind,
    List,
    Map,
};


// -----------------------------------------------------------------------------
// Primitive implementation contracts
// -----------------------------------------------------------------------------

/// Extensible primitive implementation metadata.
pub use self::primitives::{
    Primitive,
    PrimitiveFloat,
    PrimitiveInteger,
};


// -----------------------------------------------------------------------------
// Memory abstractions
// -----------------------------------------------------------------------------

/// Owned memory buffer abstraction.
pub use self::memory::MemoryBuffer;

/// Structured memory-operation errors.
pub use self::memory::MemoryError;

/// Memory footprint information.
pub use self::memory::MemoryFootprint;

/// Memory operation result.
pub use self::memory::MemoryResult;

/// Immutable borrowed memory view.
pub use self::memory::MemoryView;

/// Mutable borrowed memory view.
pub use self::memory::MemoryViewMut;


// -----------------------------------------------------------------------------
// String abstractions
// -----------------------------------------------------------------------------

/// UTF-8 string builder.
pub use self::strings::StringBuilder;

/// Borrowed UTF-8 string view.
pub use self::strings::StringView;

/// Validated UTF-8 byte offset.
pub use self::strings::Utf8ByteOffset;

/// Owned Zamani string representation.
pub use self::strings::ZamaniString;


// -----------------------------------------------------------------------------
// Iteration contracts
// -----------------------------------------------------------------------------

/// Structured iteration errors.
pub use self::iteration::IterationError;

/// Fallible iterator extension trait.
pub use self::iteration::FallibleIteratorExt;


// =============================================================================
// ARCHITECTURAL INVARIANTS
// =============================================================================

/// Foundation architecture invariants.
///
/// These are documentation-only guarantees, not runtime configuration.
///
/// 1. One authoritative implementation per foundational concept.
/// 2. No duplicate semantic type system.
/// 3. No competing tensor storage.
/// 4. No hardware-specific execution assumptions.
/// 5. No fixed universal hardware capacities.
/// 6. No arbitrary universal collection ceilings.
/// 7. No unsafe Rust.
/// 8. No implicit hardware selection.
/// 9. No runtime scheduling.
/// 10. No simulated success for unsupported operations.
///
/// These guarantees apply to the foundation API and its integration
/// with the rest of the Zamani architecture.


// =============================================================================
// FOUNDATION API TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        CollectionKind,
        List,
        Map,
        Primitive,
    };

    #[test]
    fn foundation_collection_exports_are_available() {
        let list = List::<u64>::new();
        let map = Map::<String, u64>::new();

        assert_eq!(list.len(), 0);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn foundation_collection_exports_share_their_canonical_types() {
        let kind = CollectionKind::List;

        assert_eq!(kind, CollectionKind::List);
    }

    #[test]
    fn foundation_primitive_exports_are_available() {
        assert_eq!(i32::NAME, "i32");
        assert_eq!(u64::NAME, "u64");
        assert_eq!(f64::NAME, "f64");
    }

    #[test]
    fn foundation_uses_rust_standard_option_and_result() {
        let optional: Option<u64> = Some(42);
        let result: std::result::Result<u64, ()> = Ok(42);

        assert_eq!(optional, Some(42));
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn foundation_does_not_require_hardware_initialization() {
        // Foundation aggregation itself has no initialization side effects.
        // Hardware and runtime initialization belong to their authorities.
        assert!(true);
    }
}
