//! Zamani Quantum IR — Core Identity Contracts
//!
//! This module defines stable, strongly typed identities used throughout the
//! hardware-independent Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `core::identity` owns:
//!
//! - IR object identities;
//! - stable numeric identity representation;
//! - IR schema/semantic versioning;
//! - conservative version compatibility predicates;
//! - deterministic identity formatting;
//! - explicit conversion between identity wrappers and their stable `u64`
//!   representation.
//!
//! It deliberately does NOT own:
//!
//! - logical qubit identity;
//! - physical qubit identity;
//! - qubit registers;
//! - hardware topology;
//! - hardware allocation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - backend execution;
//! - simulation state;
//! - optimization;
//! - frontend parsing;
//! - serialization formats;
//! - cryptographic content hashing.
//!
//! # Canonical qubit identity boundary
//!
//! Logical and physical qubit identities are owned exclusively by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module intentionally does NOT define or re-export replacement qubit
//! identity types.
//!
//! Any module that needs a logical or physical qubit must depend on:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! This prevents the catastrophic class of bugs where two independent
//! `QubitId` definitions become accidentally interchangeable or semantically
//! divergent.
//!
//! # Identity model
//!
//! All identities owned by this module are opaque newtypes around `u64`.
//!
//! A numeric identity is an identifier token. It is NOT:
//!
//! - a collection index;
//! - a qubit count;
//! - a hardware capacity;
//! - a memory address;
//! - a process-local pointer;
//! - a machine-size limit.
//!
//! `u64` is used rather than `usize` because semantic identities must have a
//! stable width independent of the host architecture.
//!
//! The use of `u64` does not mean that Zamani Quantum IR supports only a
//! particular number of qubits. Qubit identity remains owned by
//! `quantum::ir::qubit`, while resource limits remain explicit policy values.
//!
//! # No global allocator
//!
//! This module intentionally does not contain a global mutable ID allocator.
//!
//! Global allocation would introduce hidden process state and could make:
//!
//! - deterministic compilation;
//! - reproducible builds;
//! - distributed compilation;
//! - serialization;
//! - caching;
//! - incremental compilation;
//! - parallel compilation
//!
//! unnecessarily difficult.
//!
//! Identity allocation belongs to the owning compilation/session/program
//! builder. This module only defines the identity contract.
//!
//! # Identity stability
//!
//! An identity must remain stable for the lifetime of the semantic object it
//! identifies.
//!
//! In particular:
//!
//! - inserting another operation must not inherently renumber existing
//!   `OperationId`s;
//! - moving an operation between collections must not inherently change its
//!   identity;
//! - changing a Rust container implementation must not change an object's
//!   semantic identity;
//! - serialization/deserialization must preserve identity;
//! - cloning an IR object must preserve identity only when the clone is
//!   semantically the same object; independently created objects require
//!   independently allocated identities.
//!
//! # Versioning
//!
//! `IrVersion` identifies the Quantum IR schema and semantic contract.
//!
//! It is independent of:
//!
//! - the Zamani language version;
//! - the compiler version;
//! - the Danga version;
//! - the hardware version;
//! - the backend version;
//! - the calibration version;
//! - the simulator version.
//!
//! Version compatibility is deliberately conservative:
//!
//! - major-version changes may be breaking;
//! - minor-version changes are additive within a major contract;
//! - patch-version changes are corrections within the same contract;
//! - unknown future versions must never be silently interpreted.
//!
//! # Serialization
//!
//! This module does not define the canonical serialization format.
//!
//! `quantum::ir::serialization` owns encoding and decoding.
//!
//! The identity types here are nevertheless serialization-friendly because
//! their semantic representation is a fixed-width unsigned integer.
//!
//! # Hashing
//!
//! These identity types implement Rust's `Hash` for use in deterministic data
//! structures and maps/sets.
//!
//! They do NOT define canonical cryptographic content hashing.
//!
//! Canonical content hashing belongs to `quantum::ir::hash`.
//!
//! # Security
//!
//! No arithmetic in this module is allowed to panic through integer overflow.
//!
//! Operations that can increment identities expose checked APIs.
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! # Integration contract
//!
//! This module is intentionally foundational.
//!
//! Dependency direction:
//!
//! ```text
//! core::identity
//!       │
//!       ├── program
//!       ├── operation
//!       ├── region
//!       ├── gate
//!       ├── pulse
//!       ├── timing
//!       ├── resource
//!       ├── scheduling
//!       ├── validation
//!       ├── serialization
//!       ├── hashing
//!       ├── provenance
//!       └── dialects
//! ```
//!
//! The dependency never points in the opposite direction.
//!
//! In particular, this file must never import:
//!
//! ```text
//! quantum::ir::gate
//! quantum::ir::operation
//! quantum::ir::program
//! quantum::ir::hardware
//! quantum::ir::routing
//! quantum::ir::optimization
//! quantum::ir::frontend
//! ```
//!
//! This makes the file independently stable and prevents downstream
//! architectural changes from requiring changes here.
//!
//! # Identity categories
//!
//! The identity types are intentionally separated even when they have the
//! same underlying representation.
//!
//! For example:
//!
//! ```text
//! OperationId(7)
//! RegionId(7)
//! QubitId(7)
//! ```
//!
//! are three different semantic values.
//!
//! Rust's type system prevents accidental interchange between these domains.
//!
//! -----------------------------------------------------------------------------
//! No domain algorithms belong in this file.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// =============================================================================
// IR version
// =============================================================================

/// Version of the Zamani Quantum IR semantic/schema contract.
///
/// `IrVersion` is deliberately independent from all implementation and
/// hardware versions.
///
/// # Compatibility policy
///
/// Within a major version:
///
/// - lower minor versions may be consumed by a newer implementation when the
///   implementation explicitly supports them;
/// - a future minor version is not silently accepted;
/// - a future patch version is not silently accepted.
///
/// A major-version mismatch always requires an explicit compatibility or
/// migration decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl IrVersion {
    /// Current stable Zamani Quantum IR contract.
    ///
    /// This constant belongs to the IR contract, not to the Zamani language
    /// version or compiler version.
    pub const CURRENT: Self = Self::new(1, 0, 0);

    /// Creates an IR version.
    ///
    /// The components are unsigned and therefore structurally representable.
    /// Semantic compatibility is determined by the compatibility methods.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns the current stable IR version.
    #[must_use]
    pub const fn current() -> Self {
        Self::CURRENT
    }

    /// Returns whether two versions have the same major contract.
    #[must_use]
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether two versions are exactly identical.
    #[must_use]
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether this version is the current implementation version.
    #[must_use]
    pub const fn is_current(self) -> bool {
        self.is_exactly(Self::CURRENT)
    }

    /// Returns whether this version precedes the current version.
    #[must_use]
    pub const fn is_older_than_current(self) -> bool {
        self < Self::CURRENT
    }

    /// Returns whether this version follows the current version.
    #[must_use]
    pub const fn is_newer_than_current(self) -> bool {
        self > Self::CURRENT
    }

    /// Returns whether this version can be consumed by the current IR
    /// implementation under the conservative compatibility policy.
    ///
    /// A version is supported when:
    ///
    /// - the major version is exactly the current major;
    /// - the version's minor component is not newer than the current minor;
    /// - when the minor components are equal, the patch is not newer than the
    ///   current patch.
    ///
    /// This intentionally rejects future contracts instead of silently
    /// interpreting them.
    #[must_use]
    pub const fn is_supported_by_current(self) -> bool {
        self.major == Self::CURRENT.major
            && self.minor <= Self::CURRENT.minor
            && (self.minor < Self::CURRENT.minor
                || self.patch <= Self::CURRENT.patch)
    }

    /// Returns whether `other` can be consumed by an implementation whose
    /// contract is represented by `self`.
    ///
    /// This method is useful when comparing arbitrary producer/consumer
    /// versions rather than always comparing against `CURRENT`.
    #[must_use]
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && (other.minor < self.minor
                || other.patch <= self.patch)
    }

    /// Returns whether the two versions belong to the same major contract.
    ///
    /// This answers the *contract-family* question, not the stronger question
    /// of whether a specific implementation can actually consume the other
    /// version.
    ///
    /// Consumers that need an executable compatibility decision should use
    /// `supports` or `is_supported_by_current`.
    #[must_use]
    pub const fn is_compatible_with(
        self,
        other: Self,
    ) -> bool {
        self.same_major(other)
    }

    /// Returns whether moving from `self` to `other` crosses a major boundary.
    #[must_use]
    pub const fn requires_major_migration(
        self,
        other: Self,
    ) -> bool {
        self.major != other.major
    }

    /// Returns whether the two versions differ only within the same major
    /// contract.
    #[must_use]
    pub const fn same_major_contract(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }
}

impl Default for IrVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for IrVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Stable identity macro
// =============================================================================

/// Defines one strongly typed semantic identity.
///
/// Every identity produced by this macro:
///
/// - uses a stable `u64` representation;
/// - is opaque outside its own implementation;
/// - implements equality and ordering;
/// - implements deterministic hashing;
/// - can be converted to/from `u64`;
/// - has deterministic human-readable formatting;
/// - has no implicit relationship to collection indexes;
/// - has no relationship to hardware capacity.
///
/// The macro is private to this module. It is not part of the public API.
macro_rules! define_identity {
    (
        $(#[$meta:meta])*
        $name:ident,
        $prefix:literal
    ) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord
        )]
        pub struct $name(u64);

        impl $name {
            /// Creates an identity from an explicitly supplied stable value.
            ///
            /// This does not allocate an identity and does not check whether
            /// the value is currently in use. Allocation/uniqueness belongs to
            /// the owning compiler/session/program builder.
            #[must_use]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the stable numeric representation.
            #[must_use]
            pub const fn value(self) -> u64 {
                self.0
            }

            /// Returns the stable textual prefix used by `Display`.
            #[must_use]
            pub const fn prefix() -> &'static str {
                $prefix
            }

            /// Returns the next identity if the underlying numeric value can
            /// be incremented without overflow.
            ///
            /// This does not imply that the returned identity is allocated.
            #[must_use]
            pub const fn checked_next(self) -> Option<Self> {
                match self.0.checked_add(1) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            fn from(identity: $name) -> Self {
                identity.value()
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                write!(
                    formatter,
                    "{}{}",
                    $prefix,
                    self.0
                )
            }
        }
    };
}

// =============================================================================
// Program identity
// =============================================================================

define_identity!(
    /// Stable identity for a complete Quantum IR program.
    ///
    /// `ProgramId` identifies the semantic program object. It is not a
    /// cryptographic content hash.
    ProgramId,
    "program:"
);

// =============================================================================
// Circuit identity
// =============================================================================

define_identity!(
    /// Stable identity for a gate-oriented quantum circuit.
    ///
    /// A circuit identity is independent of the circuit's position inside a
    /// program.
    CircuitId,
    "circuit:"
);

// =============================================================================
// Module identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR module/compilation unit.
    ModuleId,
    "module:"
);

// =============================================================================
// Namespace identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR namespace.
    ///
    /// A namespace provides semantic scope identity. It is not a qubit,
    /// register, hardware device, or filesystem namespace.
    NamespaceId,
    "namespace:"
);

// =============================================================================
// Region identity
// =============================================================================

define_identity!(
    /// Stable identity for a structured IR region.
    ///
    /// Regions can contain blocks and may represent functions, control-flow
    /// bodies, loops, branches, pulse sequences, or other nested constructs.
    RegionId,
    "region:"
);

// =============================================================================
// Block identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR block.
    ///
    /// A block identity is independent of its position in a region.
    BlockId,
    "block:"
);

// =============================================================================
// Operation identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR operation.
    ///
    /// This covers quantum, classical, pulse, control-flow, and extension
    /// operations at the universal operation layer.
    OperationId,
    "op:"
);

// =============================================================================
// Value identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR value.
    ///
    /// A `ValueId` identifies an IR value-producing entity. It does not contain
    /// the value itself.
    ValueId,
    "value:"
);

// =============================================================================
// Parameter identity
// =============================================================================

define_identity!(
    /// Stable identity for a symbolic/runtime parameter.
    ///
    /// Parameter expressions and parameter values belong to the parameter/value
    /// layers, not to this identity type.
    ParameterId,
    "param:"
);

// =============================================================================
// Pulse identity
// =============================================================================

define_identity!(
    /// Stable identity for a semantic pulse operation/object.
    ///
    /// This does not identify a DAC, laser, microwave generator, control card,
    /// or other physical device.
    PulseId,
    "pulse:"
);

// =============================================================================
// Waveform identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR waveform definition.
    ///
    /// This is independent of the physical sampling hardware that eventually
    /// realizes the waveform.
    WaveformId,
    "waveform:"
);

// =============================================================================
// Channel identity
// =============================================================================

define_identity!(
    /// Stable identity for an abstract IR control/acquisition channel.
    ///
    /// This is intentionally not a physical hardware channel number.
    ChannelId,
    "channel:"
);

// =============================================================================
// Frame identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR control frame.
    ///
    /// A frame may carry semantic phase/frequency context. Physical oscillator
    /// identity remains a hardware concern.
    FrameId,
    "frame:"
);

// =============================================================================
// Schedule identity
// =============================================================================

define_identity!(
    /// Stable identity for a semantic schedule.
    ///
    /// Scheduling algorithms are outside this module.
    ScheduleId,
    "schedule:"
);

// =============================================================================
// Resource identity
// =============================================================================

define_identity!(
    /// Stable identity for an abstract IR resource declaration/requirement.
    ///
    /// This does not identify an actual physical hardware resource.
    ResourceId,
    "resource:"
);

// =============================================================================
// Capability identity
// =============================================================================

define_identity!(
    /// Stable identity for an abstract capability requirement.
    ///
    /// A capability describes required or supported behavior. The actual
    /// hardware implementation remains outside this module.
    CapabilityId,
    "capability:"
);

// =============================================================================
// Calibration identity
// =============================================================================

define_identity!(
    /// Stable identity/reference for calibration metadata.
    ///
    /// Calibration execution and physical calibration data remain outside the
    /// canonical semantic identity layer.
    CalibrationId,
    "calibration:"
);

// =============================================================================
// Function identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR-level function/subroutine.
    ///
    /// This is deliberately independent of the Zamani frontend AST.
    FunctionId,
    "function:"
);

// =============================================================================
// Type identity
// =============================================================================

define_identity!(
    /// Stable identity for an extensible IR type declaration.
    TypeId,
    "type:"
);

// =============================================================================
// Attribute identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR attribute declaration.
    AttributeId,
    "attribute:"
);

// =============================================================================
// Extension identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR extension object.
    ///
    /// An extension identity does not bypass validation or versioning.
    ExtensionId,
    "extension:"
);

// =============================================================================
// Provenance identity
// =============================================================================

define_identity!(
    /// Stable identity for a provenance record.
    ProvenanceId,
    "provenance:"
);

// =============================================================================
// Utility functions
// =============================================================================

/// Returns the current stable Quantum IR version.
///
/// This function exists as a small convenience for callers that do not need
/// to name the `IrVersion` associated constant.
#[must_use]
#[inline]
pub const fn current_ir_version() -> IrVersion {
    IrVersion::CURRENT
}

/// Returns whether an IR version can be consumed by the current
/// implementation under the conservative compatibility policy.
#[must_use]
#[inline]
pub const fn is_supported_ir_version(
    version: IrVersion,
) -> bool {
    version.is_supported_by_current()
}

/// Returns whether two versions belong to the same major contract family.
///
/// This does not mean that the current implementation can consume either
/// version. Use `IrVersion::supports` for that stronger decision.
#[must_use]
#[inline]
pub const fn is_same_ir_major(
    left: IrVersion,
    right: IrVersion,
) -> bool {
    left.same_major(right)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_is_stable() {
        assert_eq!(
            IrVersion::CURRENT,
            IrVersion::new(1, 0, 0)
        );

        assert!(IrVersion::CURRENT.is_current());
        assert!(
            IrVersion::CURRENT.is_supported_by_current()
        );
    }

    #[test]
    fn version_display_is_deterministic() {
        assert_eq!(
            IrVersion::new(1, 2, 3).to_string(),
            "1.2.3"
        );
    }

    #[test]
    fn version_accessors_are_stable() {
        let version = IrVersion::new(7, 11, 19);

        assert_eq!(version.major(), 7);
        assert_eq!(version.minor(), 11);
        assert_eq!(version.patch(), 19);
    }

    #[test]
    fn_same_major_versions_are_same_contract_family() {
        let left = IrVersion::new(1, 0, 0);
        let right = IrVersion::new(1, 99, 99);

        assert!(left.same_major(right));
        assert!(left.is_compatible_with(right));
        assert!(!left.requires_major_migration(right));
    }

    #[test]
    fn_different_major_versions_require_migration() {
        let left = IrVersion::new(1, 0, 0);
        let right = IrVersion::new(2, 0, 0);

        assert!(!left.same_major(right));
        assert!(!left.is_compatible_with(right));
        assert!(left.requires_major_migration(right));
    }

    #[test]
    fn_future_minor_is_not_silently_supported() {
        let future = IrVersion::new(
            IrVersion::CURRENT.major(),
            IrVersion::CURRENT.minor() + 1,
            0,
        );

        assert!(!future.is_supported_by_current());
    }

    #[test]
    fn future_major_is_not_silently_supported() {
        let future = IrVersion::new(
            IrVersion::CURRENT.major() + 1,
            0,
            0,
        );

        assert!(!future.is_supported_by_current());
    }

    #[test]
    fn future_patch_is_not_silently_supported() {
        let future = IrVersion::new(
            IrVersion::CURRENT.major(),
            IrVersion::CURRENT.minor(),
            IrVersion::CURRENT.patch() + 1,
        );

        assert!(!future.is_supported_by_current());
    }

    #[test]
    fn older_patch_is_supported() {
        let older = IrVersion::new(
            IrVersion::CURRENT.major(),
            IrVersion::CURRENT.minor(),
            0,
        );

        assert!(older.is_supported_by_current());
    }

    #[test]
    fn identity_is_opaque_and_strongly_typed() {
        let operation = OperationId::new(42);
        let region = RegionId::new(42);

        assert_eq!(operation.value(), 42);
        assert_eq!(region.value(), 42);

        assert_ne!(
            operation.to_string(),
            region.to_string()
        );
    }

    #[test]
    fn identity_round_trips_through_u64() {
        let original = OperationId::new(u64::MAX);

        let raw: u64 = original.into();
        let restored = OperationId::from(raw);

        assert_eq!(original, restored);
        assert_eq!(restored.value(), u64::MAX);
    }

    #[test]
    fn identity_display_is_deterministic() {
        assert_eq!(
            ProgramId::new(0).to_string(),
            "program:0"
        );

        assert_eq!(
            OperationId::new(42).to_string(),
            "op:42"
        );

        assert_eq!(
            RegionId::new(9001).to_string(),
            "region:9001"
        );
    }

    #[test]
    fn identity_prefix_is_stable() {
        assert_eq!(
            ProgramId::prefix(),
            "program:"
        );

        assert_eq!(
            OperationId::prefix(),
            "op:"
        );

        assert_eq!(
            CapabilityId::prefix(),
            "capability:"
        );
    }

    #[test]
    fn checked_next_handles_normal_values() {
        let identity = OperationId::new(41);

        assert_eq!(
            identity.checked_next(),
            Some(OperationId::new(42))
        );
    }

    #[test]
    fn checked_next_rejects_overflow() {
        let identity = OperationId::new(u64::MAX);

        assert_eq!(
            identity.checked_next(),
            None
        );
    }

    #[test]
    fn all_identity_domains_are_independently_typed() {
        let program = ProgramId::new(1);
        let circuit = CircuitId::new(1);
        let module = ModuleId::new(1);
        let namespace = NamespaceId::new(1);
        let region = RegionId::new(1);
        let block = BlockId::new(1);
        let operation = OperationId::new(1);
        let value = ValueId::new(1);
        let parameter = ParameterId::new(1);
        let pulse = PulseId::new(1);
        let waveform = WaveformId::new(1);
        let channel = ChannelId::new(1);
        let frame = FrameId::new(1);
        let schedule = ScheduleId::new(1);
        let resource = ResourceId::new(1);
        let capability = CapabilityId::new(1);
        let calibration = CalibrationId::new(1);
        let function = FunctionId::new(1);
        let type_id = TypeId::new(1);
        let attribute = AttributeId::new(1);
        let extension = ExtensionId::new(1);
        let provenance = ProvenanceId::new(1);

        assert_eq!(program.value(), 1);
        assert_eq!(circuit.value(), 1);
        assert_eq!(module.value(), 1);
        assert_eq!(namespace.value(), 1);
        assert_eq!(region.value(), 1);
        assert_eq!(block.value(), 1);
        assert_eq!(operation.value(), 1);
        assert_eq!(value.value(), 1);
        assert_eq!(parameter.value(), 1);
        assert_eq!(pulse.value(), 1);
        assert_eq!(waveform.value(), 1);
        assert_eq!(channel.value(), 1);
        assert_eq!(frame.value(), 1);
        assert_eq!(schedule.value(), 1);
        assert_eq!(resource.value(), 1);
        assert_eq!(capability.value(), 1);
        assert_eq!(calibration.value(), 1);
        assert_eq!(function.value(), 1);
        assert_eq!(type_id.value(), 1);
        assert_eq!(attribute.value(), 1);
        assert_eq!(extension.value(), 1);
        assert_eq!(provenance.value(), 1);
    }

    #[test]
    fn maximum_u64_identity_is_valid() {
        let identity = OperationId::new(u64::MAX);

        assert_eq!(
            identity.value(),
            u64::MAX
        );

        assert_eq!(
            u64::from(identity),
            u64::MAX
        );
    }

    #[test]
    fn utility_version_functions_match_type_methods() {
        let version = IrVersion::CURRENT;

        assert_eq!(
            current_ir_version(),
            version
        );

        assert!(
            is_supported_ir_version(version)
        );

        assert!(
            is_same_ir_major(
                version,
                IrVersion::new(
                    version.major(),
                    version.minor() + 1,
                    0,
                )
            )
        );
    }
}