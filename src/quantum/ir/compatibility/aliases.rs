//! Zamani Quantum IR — Compatibility Aliases
//!
//! This module provides lossless source/API aliases for canonical Quantum IR
//! items.
//!
//! # Architectural role
//!
//! `aliases.rs` is deliberately a very small compatibility boundary.
//!
//! Its only responsibility is:
//!
//! ```text
//! historical/public spelling
//!          │
//!          ▼
//! canonical Quantum IR item
//! ```
//!
//! It MUST NOT:
//!
//! - define a second semantic IR;
//! - define duplicate qubit identity types;
//! - perform source rewriting;
//! - perform schema migration;
//! - perform semantic migration;
//! - perform lowering;
//! - perform optimization;
//! - perform routing;
//! - perform scheduling;
//! - select hardware;
//! - perform calibration;
//! - execute quantum programs;
//! - allocate quantum hardware;
//! - impose a fixed qubit-count limit.
//!
//! # Canonical qubit authority
//!
//! The canonical qubit module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! quantum::ir::qubit::QubitRef
//! quantum::ir::qubit::Qubit
//! quantum::ir::qubit::QubitState
//! quantum::ir::qubit::QubitRegister
//! quantum::ir::qubit::QubitRange
//! ```
//!
//! This module never defines replacements for those types.
//!
//! # Compatibility philosophy
//!
//! An alias must be lossless.
//!
//! Therefore a compatibility alias must resolve to exactly the canonical Rust
//! type rather than introducing:
//!
//! ```text
//! wrapper -> conversion -> canonical type
//! ```
//!
//! The desired relationship is:
//!
//! ```text
//! old name ──────────────► canonical name
//!              same type
//! ```
//!
//! This prevents compatibility code from becoming a second implementation
//! that can diverge from the canonical IR.
//!
//! # Module-path compatibility
//!
//! Historical repository code has used:
//!
//! ```text
//! quantum::ir::qubits::*
//! ```
//!
//! while new code must use:
//!
//! ```text
//! quantum::ir::qubit::*
//! ```
//!
//! The `qubits` module alias below preserves the old path while pointing
//! directly to the canonical module.
//!
//! # Type aliases versus wrappers
//!
//! These are true Rust type aliases:
//!
//! ```text
//! compatibility::aliases::QubitId
//!     == quantum::ir::qubit::QubitId
//! ```
//!
//! No allocation, conversion, copying, runtime dispatch, or runtime metadata
//! is introduced by an alias.
//!
//! # Scalability
//!
//! This module introduces no machine-size assumptions.
//!
//! In particular, it contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! 63
//! 64
//! 127
//! 128
//! 4096
//! ```
//!
//! as architectural limits.
//!
//! A compatibility alias works identically for any finite number of logical
//! qubits representable by the canonical IR and available compilation/runtime
//! resources.
//!
//! Resource/security limits remain the responsibility of the explicit IR
//! limits/resource policy. Physical capacity remains the responsibility of the
//! hardware layer.
//!
//! # Dependency direction
//!
//! ```text
//! compatibility::aliases
//!          │
//!          ▼
//! quantum::ir::qubit
//! ```
//!
//! Never the reverse.
//!
//! Canonical IR modules MUST NOT import this module.
//!
//! # Relationship with `legacy.rs`
//!
//! `legacy.rs` owns broader historical compatibility APIs and migrations.
//!
//! `aliases.rs` is intentionally limited to direct aliases and the historical
//! qubit-module path bridge.
//!
//! Schema/version migration does NOT belong here.
//!
//! Semantic migration does NOT belong here.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! -----------------------------------------------------------------------------
//! No domain logic belongs below this point.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical module-path compatibility
// =============================================================================

/// Historical plural spelling of the canonical qubit module.
///
/// # Historical path
///
/// ```text
/// quantum::ir::qubits
/// ```
///
/// # Canonical path
///
/// ```text
/// quantum::ir::qubit
/// ```
///
/// This is a module-path alias only. It does not create a second implementation
/// or a second qubit namespace.
///
/// New code MUST use `quantum::ir::qubit`.
pub use crate::quantum::ir::qubit as qubits;

// =============================================================================
// Canonical qubit aliases
// =============================================================================

/// Historical compatibility alias for the canonical logical-qubit identity.
///
/// Canonical type:
///
/// ```text
/// quantum::ir::qubit::QubitId
/// ```
///
/// This alias is exactly the canonical type and therefore introduces no
/// conversion or scalability restriction.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitId"
)]
pub type QubitId = crate::quantum::ir::qubit::QubitId;

/// Historical compatibility alias for the canonical physical-qubit identity.
///
/// Canonical type:
///
/// ```text
/// quantum::ir::qubit::PhysicalQubitId
/// ```
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::PhysicalQubitId"
)]
pub type PhysicalQubitId = crate::quantum::ir::qubit::PhysicalQubitId;

/// Historical compatibility alias for the canonical logical/physical qubit
/// reference.
///
/// Canonical type:
///
/// ```text
/// quantum::ir::qubit::QubitRef
/// ```
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitRef"
)]
pub type QubitRef = crate::quantum::ir::qubit::QubitRef;

/// Historical compatibility alias for the canonical logical qubit value.
///
/// Canonical type:
///
/// ```text
/// quantum::ir::qubit::Qubit
/// ```
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::Qubit"
)]
pub type Qubit = crate::quantum::ir::qubit::Qubit;

/// Historical compatibility alias for canonical IR qubit bookkeeping state.
///
/// This type is NOT a quantum state-vector or simulator state.
///
/// Canonical type:
///
/// ```text
/// quantum::ir::qubit::QubitState
/// ```
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitState"
)]
pub type QubitState = crate::quantum::ir::qubit::QubitState;

/// Historical compatibility alias for the canonical logical-qubit register.
///
/// Register allocation/resource policy remains owned by the canonical qubit
/// and resource-policy layers. This alias introduces no register-size limit.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitRegister"
)]
pub type QubitRegister = crate::quantum::ir::qubit::QubitRegister;

/// Historical compatibility alias for the canonical logical-qubit range.
///
/// A range is a semantic range representation and does not require
/// materializing every qubit identifier.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitRange"
)]
pub type QubitRange = crate::quantum::ir::qubit::QubitRange;

// =============================================================================
// Compile-time identity guarantees
// =============================================================================
//
// These functions intentionally perform no runtime work.
//
// Their purpose is to make the type relationship compiler-checked: if the
// canonical type changes incompatibly, these assignments fail to compile.

#[allow(dead_code)]
fn assert_qubit_id_is_canonical(
    value: QubitId,
) -> crate::quantum::ir::qubit::QubitId {
    value
}

#[allow(dead_code)]
fn assert_physical_qubit_id_is_canonical(
    value: PhysicalQubitId,
) -> crate::quantum::ir::qubit::PhysicalQubitId {
    value
}

#[allow(dead_code)]
fn assert_qubit_ref_is_canonical(
    value: QubitRef,
) -> crate::quantum::ir::qubit::QubitRef {
    value
}

#[allow(dead_code)]
fn assert_qubit_is_canonical(
    value: Qubit,
) -> crate::quantum::ir::qubit::Qubit {
    value
}

#[allow(dead_code)]
fn assert_qubit_state_is_canonical(
    value: QubitState,
) -> crate::quantum::ir::qubit::QubitState {
    value
}

#[allow(dead_code)]
fn assert_qubit_register_is_canonical(
    value: QubitRegister,
) -> crate::quantum::ir::qubit::QubitRegister {
    value
}

#[allow(dead_code)]
fn assert_qubit_range_is_canonical(
    value: QubitRange,
) -> crate::quantum::ir::qubit::QubitRange {
    value
}

// =============================================================================
// Compatibility metadata
// =============================================================================

/// Canonical module path for logical and physical qubit identities.
///
/// This is intentionally plain compile-time data. It does not perform source
/// rewriting or semantic migration.
pub const CANONICAL_QUBIT_MODULE: &str = "quantum::ir::qubit";

/// Canonical logical-qubit identity path.
pub const CANONICAL_QUBIT_ID: &str = "quantum::ir::qubit::QubitId";

/// Canonical physical-qubit identity path.
pub const CANONICAL_PHYSICAL_QUBIT_ID: &str =
    "quantum::ir::qubit::PhysicalQubitId";

/// Historical logical-qubit identity path.
pub const LEGACY_QUBIT_ID: &str = "quantum::ir::qubits::QubitId";

/// Historical physical-qubit identity path.
pub const LEGACY_PHYSICAL_QUBIT_ID: &str =
    "quantum::ir::qubits::PhysicalQubitId";

/// Describes a lossless source/API path mapping.
///
/// This structure does not perform migration. It simply describes the
/// canonical replacement so tooling can consume the compatibility contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AliasMapping {
    /// Historical source/API path.
    pub legacy: &'static str,

    /// Canonical replacement path.
    pub canonical: &'static str,
}

impl AliasMapping {
    /// Creates a path mapping.
    pub const fn new(
        legacy: &'static str,
        canonical: &'static str,
    ) -> Self {
        Self { legacy, canonical }
    }
}

/// Returns the logical-qubit identity compatibility mapping.
#[must_use]
pub const fn qubit_id_mapping() -> AliasMapping {
    AliasMapping::new(
        LEGACY_QUBIT_ID,
        CANONICAL_QUBIT_ID,
    )
}

/// Returns the physical-qubit identity compatibility mapping.
#[must_use]
pub const fn physical_qubit_id_mapping() -> AliasMapping {
    AliasMapping::new(
        LEGACY_PHYSICAL_QUBIT_ID,
        CANONICAL_PHYSICAL_QUBIT_ID,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_qubit_alias_is_exactly_canonical() {
        let canonical = crate::quantum::ir::qubit::QubitId::new(17);

        #[allow(deprecated)]
        let compatibility: QubitId = canonical;

        let round_trip: crate::quantum::ir::qubit::QubitId = compatibility;

        assert_eq!(round_trip, canonical);
    }

    #[test]
    fn physical_qubit_alias_is_exactly_canonical() {
        let canonical =
            crate::quantum::ir::qubit::PhysicalQubitId::new(23);

        #[allow(deprecated)]
        let compatibility: PhysicalQubitId = canonical;

        let round_trip:
            crate::quantum::ir::qubit::PhysicalQubitId =
            compatibility;

        assert_eq!(round_trip, canonical);
    }

    #[test]
    fn qubit_ref_alias_is_exactly_canonical() {
        let canonical =
            crate::quantum::ir::qubit::QubitRef::Logical(
                crate::quantum::ir::qubit::QubitId::new(31),
            );

        #[allow(deprecated)]
        let compatibility: QubitRef = canonical;

        let round_trip:
            crate::quantum::ir::qubit::QubitRef =
            compatibility;

        assert_eq!(round_trip, canonical);
    }

    #[test]
    fn qubit_alias_is_exactly_canonical() {
        let canonical =
            crate::quantum::ir::qubit::Qubit::new(
                crate::quantum::ir::qubit::QubitId::new(5),
            );

        #[allow(deprecated)]
        let compatibility: Qubit = canonical;

        let round_trip: crate::quantum::ir::qubit::Qubit =
            compatibility;

        assert_eq!(round_trip.id(), canonical.id());
        assert_eq!(round_trip.state(), canonical.state());
    }

    #[test]
    fn qubit_state_alias_is_exactly_canonical() {
        let canonical =
            crate::quantum::ir::qubit::QubitState::Measured;

        #[allow(deprecated)]
        let compatibility: QubitState = canonical;

        let round_trip:
            crate::quantum::ir::qubit::QubitState =
            compatibility;

        assert_eq!(round_trip, canonical);
    }

    #[test]
    fn qubit_range_alias_is_exactly_canonical() {
        let canonical =
            crate::quantum::ir::qubit::QubitRange::new(10, 100)
                .expect("valid half-open range");

        #[allow(deprecated)]
        let compatibility: QubitRange = canonical;

        let round_trip:
            crate::quantum::ir::qubit::QubitRange =
            compatibility;

        assert_eq!(round_trip, canonical);
    }

    #[test]
    fn module_alias_resolves_to_canonical_qubit_module() {
        let canonical =
            crate::quantum::ir::qubit::QubitId::new(41);

        let through_alias: qubits::QubitId = canonical;

        assert_eq!(through_alias, canonical);
    }

    #[test]
    fn logical_mapping_is_stable() {
        let mapping = qubit_id_mapping();

        assert_eq!(
            mapping.legacy,
            "quantum::ir::qubits::QubitId"
        );
        assert_eq!(
            mapping.canonical,
            "quantum::ir::qubit::QubitId"
        );
    }

    #[test]
    fn physical_mapping_is_stable() {
        let mapping = physical_qubit_id_mapping();

        assert_eq!(
            mapping.legacy,
            "quantum::ir::qubits::PhysicalQubitId"
        );
        assert_eq!(
            mapping.canonical,
            "quantum::ir::qubit::PhysicalQubitId"
        );
    }

    #[test]
    fn compatibility_does_not_change_large_identifier_values() {
        let value = usize::MAX;

        let canonical =
            crate::quantum::ir::qubit::QubitId::new(value);

        let through_alias: qubits::QubitId = canonical;

        assert_eq!(through_alias.index(), value);
    }
}