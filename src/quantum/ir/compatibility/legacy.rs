//! Zamani Quantum IR — Legacy Compatibility Boundary
//!
//! This module contains compatibility-only API for historical Quantum IR
//! spellings and APIs.
//!
//! # Architectural purpose
//!
//! `legacy.rs` exists to let older Zamani code migrate to the canonical
//! Quantum IR without duplicating semantic types.
//!
//! The canonical architecture is:
//!
//! ```text
//! historical API
//!       │
//!       ▼
//! compatibility::legacy
//!       │
//!       ▼
//! canonical quantum::ir
//!       │
//!       ├── qubit
//!       ├── gate
//!       ├── operation
//!       ├── program
//!       ├── classical
//!       └── ...
//! ```
//!
//! Compatibility is intentionally one-way:
//!
//! ```text
//! legacy spelling ───────► canonical type
//! ```
//!
//! It MUST NOT create a second semantic representation.
//!
//! # Canonical qubit rule
//!
//! The authoritative logical-qubit identity is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! The authoritative physical-qubit identity is:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module therefore aliases those exact types.
//!
//! It must never define:
//!
//! ```text
//! struct LegacyQubitId(...)
//! struct LegacyPhysicalQubitId(...)
//! ```
//!
//! or any other duplicate identity representation.
//!
//! # Why this module exists
//!
//! The repository historically contains consumers using:
//!
//! ```text
//! quantum::ir::qubits::QubitId
//! ```
//!
//! while the canonical repository path is now:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Existing repository code has been identified using the historical spelling,
//! including parts of the optimization subsystem. Keeping the compatibility
//! boundary explicit prevents that historical spelling from propagating into
//! new IR code.
//!
//! # Compatibility philosophy
//!
//! Compatibility must preserve semantics, not preserve accidental
//! implementation details.
//!
//! Therefore this module:
//!
//! - aliases canonical types;
//! - re-exports canonical constructors/types where safe;
//! - documents migration targets;
//! - provides compile-time deprecation guidance;
//! - does not mutate programs;
//! - does not perform lowering;
//! - does not perform optimization;
//! - does not perform routing;
//! - does not perform scheduling;
//! - does not select hardware;
//! - does not perform serialization;
//! - does not perform execution.
//!
//! # Universal-program/scalability rule
//!
//! Legacy compatibility MUST NOT introduce a fixed machine-size assumption.
//!
//! In particular, this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_OPERATIONS
//! 63
//! 64
//! 127
//! 128
//! 4096
//! ```
//!
//! as semantic limits.
//!
//! The canonical IR already establishes that concrete limits are resource or
//! security policies rather than architectural quantum-machine limits.
//!
//! A legacy API must therefore resolve to the same canonical types whether a
//! program contains one qubit or a very large finite number of qubits subject
//! to available resources.
//!
//! # Information-preservation rule
//!
//! Compatibility aliases are lossless because they are aliases to the exact
//! canonical types.
//!
//! This is preferable to compatibility wrappers that copy fields or convert
//! through smaller integer types.
//!
//! In particular, this module MUST NOT perform conversions such as:
//!
//! ```text
//! u64 -> u32
//! usize -> u16
//! usize -> u8
//! ```
//!
//! merely to support historical APIs.
//!
//! # Dependency boundary
//!
//! This module may depend only on canonical IR modules.
//!
//! It MUST NOT depend on:
//!
//! - `quantum::frontend`;
//! - `quantum::optimization`;
//! - `quantum::routing`;
//! - `quantum::scheduling`;
//! - `quantum::hardware`;
//! - `quantum::simulator`;
//! - `quantum::qec`;
//! - backend execution;
//! - external service APIs;
//! - source-language ASTs.
//!
//! Downstream systems may use this module during migration, but canonical IR
//! modules must never depend on this compatibility layer.
//!
//! # Stable integration contract
//!
//! Canonical modules remain the source of truth.
//!
//! ```text
//! compatibility::legacy
//!          │
//!          ├────► qubit.rs
//!          ├────► gate.rs
//!          ├────► circuit.rs
//!          ├────► parameter.rs
//!          ├────► measurement.rs
//!          └────► identity.rs
//! ```
//!
//! No reverse dependency is permitted.
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
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler
//! enforced.
//!
//! # Deprecation policy
//!
//! Compatibility symbols are deprecated rather than immediately removed.
//!
//! This allows the repository to migrate incrementally:
//!
//! ```text
//! old consumer
//!     │
//!     ▼
//! compatibility alias
//!     │
//!     ▼
//! canonical implementation
//! ```
//!
//! The deprecation message always names the canonical replacement.
//!
//! Compatibility code must not emit runtime warnings, print to stdout, or
//! alter program execution.
//!
//! # Important distinction
//!
//! This module is source/API compatibility.
//!
//! It is NOT IR schema migration.
//!
//! Serialized IR-version migration belongs to the serialization/versioning
//! boundary. Semantic transformations between incompatible IR versions must
//! never be hidden inside these aliases.
//!
//! # Public API
//!
//! The compatibility surface is deliberately conservative.
//!
//! Only symbols for which the canonical replacement is unambiguous are exposed.
//!
//! New code MUST use canonical module paths directly.
//!
//! -----------------------------------------------------------------------------
//! No domain logic belongs in this module.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical qubit compatibility
// =============================================================================
//
// IMPORTANT:
//
// These are aliases to the canonical types.
//
// There is exactly one QubitId type in the semantic IR:
//
//     quantum::ir::qubit::QubitId
//
// Therefore:
//
//     compatibility::legacy::QubitId
//
// and:
//
//     quantum::ir::qubit::QubitId
//
// are the same Rust type.

/// Historical compatibility alias for the canonical logical-qubit identity.
///
/// # Migration
///
/// Replace:
///
/// ```text
/// quantum::ir::compatibility::legacy::QubitId
/// ```
///
/// with:
///
/// ```text
/// quantum::ir::qubit::QubitId
/// ```
///
/// The alias does not create a second qubit identity type.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitId; this compatibility alias will be removed in a future breaking release"
)]
pub type QubitId = super::super::qubit::QubitId;

/// Historical compatibility alias for the canonical physical-qubit identity.
///
/// # Migration
///
/// Replace:
///
/// ```text
/// quantum::ir::compatibility::legacy::PhysicalQubitId
/// ```
///
/// with:
///
/// ```text
/// quantum::ir::qubit::PhysicalQubitId
/// ```
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::PhysicalQubitId; this compatibility alias will be removed in a future breaking release"
)]
pub type PhysicalQubitId = super::super::qubit::PhysicalQubitId;

/// Historical compatibility alias for the canonical logical/physical qubit
/// reference.
///
/// New code should use `quantum::ir::qubit::QubitRef`.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitRef"
)]
pub type QubitRef = super::super::qubit::QubitRef;

/// Historical compatibility alias for the canonical logical qubit value.
///
/// New code should use `quantum::ir::qubit::Qubit`.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::Qubit"
)]
pub type Qubit = super::super::qubit::Qubit;

/// Historical compatibility alias for the canonical logical-qubit state
/// bookkeeping type.
///
/// This is compiler/IR bookkeeping and is not a quantum state-vector type.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitState"
)]
pub type QubitState = super::super::qubit::QubitState;

/// Historical compatibility alias for the canonical logical-qubit register.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::qubit::QubitRegister"
)]
pub type QubitRegister = super::super::qubit::QubitRegister;

// =============================================================================
// Gate compatibility
// =============================================================================
//
// Gate compatibility aliases deliberately point at the canonical gate module.
// They do not create a legacy gate implementation.

/// Historical compatibility alias for the canonical gate representation.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::gate::Gate"
)]
pub type Gate = super::super::gate::Gate;

/// Historical compatibility alias for the standard gate-kind vocabulary.
///
/// `GateKind` represents the standard semantic gate vocabulary. It is not the
/// complete universe of possible Zamani quantum operations.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::gate::GateKind"
)]
pub type GateKind = super::super::gate::GateKind;

/// Historical compatibility alias for canonical gate parameters.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::gate::GateParameter"
)]
pub type GateParameter = super::super::gate::GateParameter;

/// Historical compatibility alias for canonical gate errors.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::gate::GateError"
)]
pub type GateError = super::super::gate::GateError;

// =============================================================================
// Circuit compatibility
// =============================================================================

/// Historical compatibility alias for the canonical gate-oriented circuit.
///
/// `QuantumCircuit` is a specialized circuit representation. It is not the
/// complete canonical quantum-program model.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::circuit::QuantumCircuit"
)]
pub type QuantumCircuit = super::super::circuit::QuantumCircuit;

/// Historical compatibility alias for circuit errors.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::circuit::CircuitError"
)]
pub type CircuitError = super::super::circuit::CircuitError;

/// Historical compatibility alias for circuit metadata.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::circuit::CircuitMetadata"
)]
pub type CircuitMetadata = super::super::circuit::CircuitMetadata;

// =============================================================================
// Parameter compatibility
// =============================================================================

/// Historical compatibility alias for canonical symbolic/numerical parameters.
///
/// Parameters remain target-independent and may be resolved later by
/// downstream compilation stages.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::parameter::Parameter"
)]
pub type Parameter = super::super::parameter::Parameter;

// =============================================================================
// Measurement compatibility
// =============================================================================

/// Historical compatibility alias for canonical measurement semantics.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::measurement::Measurement"
)]
pub type Measurement = super::super::measurement::Measurement;

/// Historical compatibility alias for canonical measurement bases.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::measurement::MeasurementBasis"
)]
pub type MeasurementBasis = super::super::measurement::MeasurementBasis;

/// Historical compatibility alias for canonical measurement modes.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::measurement::MeasurementMode"
)]
pub type MeasurementMode = super::super::measurement::MeasurementMode;

/// Historical compatibility alias for canonical classical-bit identity.
///
/// The exact canonical definition remains owned by the measurement/classical
/// IR boundary used by the current repository.
#[deprecated(
    since = "1.1.0",
    note = "use the canonical ClassicalBitId exported by quantum::ir"
)]
pub type ClassicalBitId = super::super::measurement::ClassicalBitId;

// =============================================================================
// Identity compatibility
// =============================================================================

/// Historical compatibility alias for the canonical IR version.
///
/// Compatibility code must never define a second version type.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::IrVersion"
)]
pub type IrVersion = super::super::identity::IrVersion;

/// Historical compatibility alias for canonical operation identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::OperationId"
)]
pub type OperationId = super::super::identity::OperationId;

/// Historical compatibility alias for canonical program identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ProgramId"
)]
pub type ProgramId = super::super::identity::ProgramId;

/// Historical compatibility alias for canonical module identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ModuleId"
)]
pub type ModuleId = super::super::identity::ModuleId;

/// Historical compatibility alias for canonical region identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::RegionId"
)]
pub type RegionId = super::super::identity::RegionId;

/// Historical compatibility alias for canonical block identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::BlockId"
)]
pub type BlockId = super::super::identity::BlockId;

/// Historical compatibility alias for canonical value identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ValueId"
)]
pub type ValueId = super::super::identity::ValueId;

/// Historical compatibility alias for canonical parameter identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ParameterId"
)]
pub type ParameterId = super::super::identity::ParameterId;

/// Historical compatibility alias for canonical resource identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ResourceId"
)]
pub type ResourceId = super::super::identity::ResourceId;

/// Historical compatibility alias for canonical capability identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::CapabilityId"
)]
pub type CapabilityId = super::super::identity::CapabilityId;

/// Historical compatibility alias for canonical channel identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ChannelId"
)]
pub type ChannelId = super::super::identity::ChannelId;

/// Historical compatibility alias for canonical frame identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::FrameId"
)]
pub type FrameId = super::super::identity::FrameId;

/// Historical compatibility alias for canonical pulse identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::PulseId"
)]
pub type PulseId = super::super::identity::PulseId;

/// Historical compatibility alias for canonical waveform identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::WaveformId"
)]
pub type WaveformId = super::super::identity::WaveformId;

/// Historical compatibility alias for canonical schedule identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ScheduleId"
)]
pub type ScheduleId = super::super::identity::ScheduleId;

/// Historical compatibility alias for canonical extension identity.
#[deprecated(
    since = "1.1.0",
    note = "use quantum::ir::identity::ExtensionId"
)]
pub type ExtensionId = super::super::identity::ExtensionId;

// =============================================================================
// Canonical migration constants
// =============================================================================

/// Canonical module path for logical qubit identity.
///
/// Keeping this as a string is useful to migration tooling without requiring
/// migration tooling to depend on a Rust type.
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

// =============================================================================
// Compatibility metadata
// =============================================================================

/// Stable description of the legacy qubit-module migration.
///
/// This is intentionally plain data. It performs no source rewriting and no
/// semantic conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LegacyPathMapping {
    /// Historical source path.
    pub legacy: &'static str,

    /// Canonical replacement path.
    pub canonical: &'static str,
}

impl LegacyPathMapping {
    /// Creates a compatibility mapping.
    pub const fn new(
        legacy: &'static str,
        canonical: &'static str,
    ) -> Self {
        Self {
            legacy,
            canonical,
        }
    }
}

/// Returns the canonical mapping for the historical logical-qubit identity.
#[must_use]
pub const fn qubit_id_mapping() -> LegacyPathMapping {
    LegacyPathMapping::new(
        LEGACY_QUBIT_ID,
        CANONICAL_QUBIT_ID,
    )
}

/// Returns the canonical mapping for the historical physical-qubit identity.
#[must_use]
pub const fn physical_qubit_id_mapping() -> LegacyPathMapping {
    LegacyPathMapping::new(
        LEGACY_PHYSICAL_QUBIT_ID,
        CANONICAL_PHYSICAL_QUBIT_ID,
    )
}

// =============================================================================
// Compatibility classification
// =============================================================================

/// Classification of compatibility work.
///
/// This deliberately distinguishes source/API compatibility from serialized
/// IR migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompatibilityKind {
    /// Historical Rust/API spelling resolves to the same canonical type.
    Alias,

    /// A historical spelling needs a source-level rename.
    SourceRename,

    /// A serialized representation needs an explicit migration.
    SchemaMigration,

    /// A semantic change requires an explicit compiler transformation.
    SemanticMigration,
}

/// Returns the compatibility class for the historical `qubits` module path.
///
/// The historical logical/physical qubit spelling is an API/source rename,
/// not a semantic migration.
#[must_use]
pub const fn qubits_compatibility_kind() -> CompatibilityKind {
    CompatibilityKind::SourceRename
}

/// Returns the compatibility class represented by this module itself.
///
/// `legacy.rs` primarily provides aliases; it does not migrate serialized IR.
#[must_use]
pub const fn compatibility_kind() -> CompatibilityKind {
    CompatibilityKind::Alias
}

// =============================================================================
// Canonicalization guidance
// =============================================================================

/// Returns the canonical Rust import path for logical qubit identity.
///
/// This helper is intentionally static and allocation-free so tooling and
/// diagnostics can use it without depending on a compiler service.
#[must_use]
pub const fn canonical_qubit_id_path() -> &'static str {
    CANONICAL_QUBIT_ID
}

/// Returns the canonical Rust import path for physical qubit identity.
#[must_use]
pub const fn canonical_physical_qubit_id_path() -> &'static str {
    CANONICAL_PHYSICAL_QUBIT_ID
}

/// Returns the historical Rust import path for logical qubit identity.
#[must_use]
pub const fn legacy_qubit_id_path() -> &'static str {
    LEGACY_QUBIT_ID
}

/// Returns the historical Rust import path for physical qubit identity.
#[must_use]
pub const fn legacy_physical_qubit_id_path() -> &'static str {
    LEGACY_PHYSICAL_QUBIT_ID
}

// =============================================================================
// Compile-time type-identity guarantees
// =============================================================================
//
// These functions intentionally accept and return the canonical types.
// They prove that compatibility aliases do not introduce wrappers.
//
// No runtime conversion occurs.

/// Identity-preserving compatibility boundary for logical qubit IDs.
///
/// This function exists primarily for integration tests and migration code.
/// It returns the exact same canonical `QubitId` value.
#[inline]
#[must_use]
pub const fn canonicalize_qubit_id(
    qubit: super::super::qubit::QubitId,
) -> super::super::qubit::QubitId {
    qubit
}

/// Identity-preserving compatibility boundary for physical qubit IDs.
#[inline]
#[must_use]
pub const fn canonicalize_physical_qubit_id(
    qubit: super::super::qubit::PhysicalQubitId,
) -> super::super::qubit::PhysicalQubitId {
    qubit
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_qubit_id_is_the_canonical_type() {
        let canonical =
            super::super::qubit::QubitId::new(17);

        let through_legacy =
            canonicalize_qubit_id(canonical);

        assert_eq!(canonical, through_legacy);
    }

    #[test]
    fn legacy_physical_qubit_id_is_the_canonical_type() {
        let canonical =
            super::super::qubit::PhysicalQubitId::new(23);

        let through_legacy =
            canonicalize_physical_qubit_id(canonical);

        assert_eq!(canonical, through_legacy);
    }

    #[test]
    fn canonical_paths_are_stable() {
        assert_eq!(
            canonical_qubit_id_path(),
            "quantum::ir::qubit::QubitId"
        );

        assert_eq!(
            canonical_physical_qubit_id_path(),
            "quantum::ir::qubit::PhysicalQubitId"
        );
    }

    #[test]
    fn legacy_paths_are_stable() {
        assert_eq!(
            legacy_qubit_id_path(),
            "quantum::ir::qubits::QubitId"
        );

        assert_eq!(
            legacy_physical_qubit_id_path(),
            "quantum::ir::qubits::PhysicalQubitId"
        );
    }

    #[test]
    fn qubit_mapping_points_to_canonical_identity() {
        let mapping = qubit_id_mapping();

        assert_eq!(
            mapping.legacy,
            LEGACY_QUBIT_ID
        );

        assert_eq!(
            mapping.canonical,
            CANONICAL_QUBIT_ID
        );
    }

    #[test]
    fn physical_qubit_mapping_points_to_canonical_identity() {
        let mapping =
            physical_qubit_id_mapping();

        assert_eq!(
            mapping.legacy,
            LEGACY_PHYSICAL_QUBIT_ID
        );

        assert_eq!(
            mapping.canonical,
            CANONICAL_PHYSICAL_QUBIT_ID
        );
    }

    #[test]
    fn compatibility_kind_is_alias_based() {
        assert_eq!(
            compatibility_kind(),
            CompatibilityKind::Alias
        );
    }

    #[test]
    fn qubits_are_source_rename_compatibility() {
        assert_eq!(
            qubits_compatibility_kind(),
            CompatibilityKind::SourceRename
        );
    }
}