//! Zamani Quantum IR — Identity and Version Contracts
//!
//! This module defines the stable identity and IR-version primitives shared by
//! the hardware-independent Zamani Quantum IR.
//!
//! # Architectural boundary
//!
//! `identity.rs` owns identities for IR objects and the IR schema version.
//!
//! It deliberately does NOT own:
//!
//! - logical qubit identity;
//! - physical qubit identity;
//! - hardware topology;
//! - hardware allocation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - backend execution;
//! - simulation;
//! - optimization;
//! - frontend parsing.
//!
//! Logical and physical qubit identities remain owned by `qubit.rs`:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module therefore does not import or duplicate those types.
//!
//! # Identity model
//!
//! IR identities are opaque, strongly typed identifiers backed by `u64`.
//!
//! A `u64` identity is intentionally chosen instead of `usize` because:
//!
//! - it has a stable width across 32-bit and 64-bit platforms;
//! - it is straightforward to serialize;
//! - it does not encode a platform-dependent collection index;
//! - it is large enough for practical compiler, distributed, and persistent
//!   workloads;
//! - it does not impose a 63-qubit or similar machine-size limit.
//!
//! A `u64` identifier is an identity token, not a declaration of how many
//! quantum resources a machine contains.
//!
//! # Scalability
//!
//! Zamani Quantum IR has no architectural fixed qubit-count limit.
//!
//! This module must therefore never introduce constants such as:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as limits on quantum-machine size.
//!
//! Actual resource limits belong to `limits.rs`, while actual hardware
//! capacity belongs to `quantum::hardware`.
//!
//! # Identifier allocation
//!
//! This module deliberately does not contain a global ID allocator.
//!
//! Global mutable allocation would introduce hidden state, make deterministic
//! compilation harder, and complicate distributed compilation.
//!
//! IDs are supplied by the owning compiler/session/program builder,
//! persistence layer, or other explicitly controlled owner.
//!
//! # Versioning
//!
//! `IrVersion` identifies the Quantum IR schema and semantic contract.
//!
//! It is NOT:
//!
//! - a compiler version;
//! - a Zamani language version;
//! - a hardware version;
//! - a backend version;
//! - a calibration version.
//!
//! Major versions may contain breaking changes.
//! Minor versions add compatible capabilities.
//! Patch versions represent contract-preserving corrections.
//!
//! An implementation must never silently interpret an unknown future major or
//! minor IR contract.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.
//! No `unsafe` code is used.

use std::fmt;

// =============================================================================
// IR version
// =============================================================================

/// Version of the Zamani Quantum IR schema and semantic contract.
///
/// The version is independent from compiler, language, hardware, backend, and
/// calibration versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl IrVersion {
    /// Current stable Quantum IR contract.
    ///
    /// This is the initial stable IR contract. Future breaking changes must
    /// increment the major version.
    pub const CURRENT: Self = Self::new(1, 0, 0);

    /// Creates an IR version.
    ///
    /// All components are unsigned and therefore structurally valid.
    pub const const fn new(
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
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns whether both versions belong to the same major contract.
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether both versions are exactly equal.
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether this is the current IR version.
    pub const fn is_current(self) -> bool {
        self.is_exactly(Self::CURRENT)
    }

    /// Returns whether this version is older than the current version.
    pub const fn is_older_than_current(self) -> bool {
        self < Self::CURRENT
    }

    /// Returns whether this version is newer than the current version.
    pub const fn is_newer_than_current(self) -> bool {
        self > Self::CURRENT
    }

    /// Returns whether this version can be consumed by the current IR
    /// implementation.
    ///
    /// Compatibility policy:
    ///
    /// - an older major version is accepted only if its major is the current
    ///   major;
    /// - an older minor version is accepted within the same major contract;
    /// - a future minor version is rejected;
    /// - a future major version is rejected;
    /// - future patch versions are rejected because the implementation does
    ///   not know what contract-preserving changes they contain.
    ///
    /// This conservative policy prevents silent interpretation of an IR
    /// contract that this implementation does not explicitly recognize.
    pub const fn is_supported_by_current(self) -> bool {
        self.major == Self::CURRENT.major
            && self.minor <= Self::CURRENT.minor
            && !(self.minor == Self::CURRENT.minor
                && self.patch > Self::CURRENT.patch)
    }

    /// Returns whether `other` can be consumed by this version.
    ///
    /// This is useful when a compiler/backend explicitly negotiates a version
    /// boundary instead of always comparing against `CURRENT`.
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && !(other.minor == self.minor
                && other.patch > self.patch)
    }

    /// Returns whether this version is compatible with another version under
    /// the conservative same-major/supported-release policy.
    pub const fn is_compatible_with(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && (self == other
                || self < other
                || other < self)
    }

    /// Returns the current stable IR version.
    pub const fn current() -> Self {
        Self::CURRENT
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
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Identity macro
// =============================================================================

/// Defines an opaque stable IR identity type.
///
/// Every generated identity:
///
/// - is strongly typed;
/// - is `Copy`;
/// - is hashable;
/// - is orderable;
/// - has deterministic formatting;
/// - can be converted to/from `u64`;
/// - cannot accidentally be confused with another identity type.
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
            /// Creates an identity from an application-controlled value.
            ///
            /// The numeric value has no architectural relationship to the
            /// number of qubits, operations, or physical hardware resources.
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the underlying stable numeric identity.
            pub const fn value(self) -> u64 {
                self.0
            }

            /// Returns the identity's stable display prefix.
            pub const fn prefix() -> &'static str {
                $prefix
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u64 {
            fn from(id: $name) -> u64 {
                id.value()
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                write!(
                    f,
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
    /// `ProgramId` identifies an IR program instance. It is not itself a
    /// cryptographic content hash. Content identity belongs to the hashing
    /// infrastructure.
    ProgramId,
    "program"
);

// =============================================================================
// Circuit identity
// =============================================================================

define_identity!(
    /// Stable identity for a quantum circuit contained in an IR program.
    ///
    /// A circuit identity is independent of its position in a program.
    CircuitId,
    "c"
);

// =============================================================================
// Module identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR module or namespace-level compilation unit.
    ModuleId,
    "module"
);

// =============================================================================
// Namespace identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR namespace.
    ///
    /// A namespace provides scope identity. It does not replace logical qubit
    /// identity, physical qubit identity, or classical-bit identity.
    NamespaceId,
    "ns"
);

// =============================================================================
// Region identity
// =============================================================================

define_identity!(
    /// Stable identity for a structured IR region.
    ///
    /// Regions are used by higher-level IR structures for control flow,
    /// functions, loops, branches, and other nested program constructs.
    RegionId,
    "region"
);

// =============================================================================
// Block identity
// =============================================================================

define_identity!(
    /// Stable identity for a basic/structured IR block.
    BlockId,
    "block"
);

// =============================================================================
// Operation identity
// =============================================================================

define_identity!(
    /// Stable identity for a quantum or classical IR operation.
    ///
    /// Operation identity is independent of operation position.
    ///
    /// Inserting another operation before an existing operation must not
    /// inherently change the existing operation's identity.
    OperationId,
    "op"
);

// =============================================================================
// Value identity
// =============================================================================

define_identity!(
    /// Stable identity for an SSA-like or otherwise named IR value.
    ///
    /// This is distinct from a literal value. A `ValueId` identifies an IR
    /// value-producing entity.
    ValueId,
    "value"
);

// =============================================================================
// Parameter identity
// =============================================================================

define_identity!(
    /// Stable identity for a symbolic or runtime IR parameter.
    ///
    /// The actual parameter expression/value belongs to parameter/value
    /// infrastructure, not to this identity type.
    ParameterId,
    "param"
);

// =============================================================================
// Pulse identity
// =============================================================================

define_identity!(
    /// Stable identity for a pulse-level IR operation.
    ///
    /// This identifies the semantic pulse object. It does not identify a DAC,
    /// microwave generator, laser, physical control line, or hardware device.
    PulseId,
    "pulse"
);

// =============================================================================
// Waveform identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR waveform definition.
    ///
    /// A waveform identity is hardware-independent. Hardware waveform
    /// realization belongs to downstream hardware/backend layers.
    WaveformId,
    "wave"
);

// =============================================================================
// Channel identity
// =============================================================================

define_identity!(
    /// Stable identity for an abstract IR control/acquisition channel.
    ///
    /// This is not a physical hardware channel number.
    ChannelId,
    "channel"
);

// =============================================================================
// Frame identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR control frame.
    ///
    /// A frame is a semantic pulse/control concept. Physical oscillator,
    /// microwave source, laser, or electronics identity belongs to hardware.
    FrameId,
    "frame"
);

// =============================================================================
// Schedule identity
// =============================================================================

define_identity!(
    /// Stable identity for a schedule representation.
    ///
    /// The scheduling subsystem computes schedules; this identity only
    /// identifies the resulting IR schedule object.
    ScheduleId,
    "schedule"
);

// =============================================================================
// Resource identity
// =============================================================================

define_identity!(
    /// Stable identity for an abstract IR resource requirement.
    ///
    /// This does not identify an actual hardware resource.
    ResourceId,
    "resource"
);

// =============================================================================
// Capability identity
// =============================================================================

define_identity!(
    /// Stable identity for a capability requirement/reference.
    ///
    /// Actual hardware capabilities remain owned by the hardware layer.
    CapabilityId,
    "cap"
);

// =============================================================================
// Calibration identity
// =============================================================================

define_identity!(
    /// Stable identity/reference for calibration metadata.
    ///
    /// The identity may be recorded by IR provenance, but calibration data and
    /// calibration execution remain outside the canonical semantic IR.
    CalibrationId,
    "cal"
);

// =============================================================================
// Function identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR-level quantum/classical function.
    ///
    /// This supports structured programs without making the IR dependent on
    /// the Zamani frontend's source-language AST.
    FunctionId,
    "fn"
);

// =============================================================================
// Type identity
// =============================================================================

define_identity!(
    /// Stable identity for an extensible IR type declaration.
    TypeId,
    "type"
);

// =============================================================================
// Attribute identity
// =============================================================================

define_identity!(
    /// Stable identity for an IR attribute declaration.
    AttributeId,
    "attr"
);

// =============================================================================
// Extension identity
// =============================================================================

define_identity!(
    /// Stable identity for an extensible IR extension object.
    ///
    /// Extension identities do not bypass validation or the canonical IR
    /// compatibility rules.
    ExtensionId,
    "ext"
);

// =============================================================================
// Provenance identity
// =============================================================================

define_identity!(
    /// Stable identity for a provenance record.
    ProvenanceId,
    "prov"
);

// =============================================================================
// Identity utilities
// =============================================================================

/// Returns the current stable Quantum IR version.
#[inline]
pub const fn current_ir_version() -> IrVersion {
    IrVersion::CURRENT
}

/// Returns whether the supplied IR version is supported by this implementation.
///
/// This function is intentionally conservative and delegates to
/// `IrVersion::is_supported_by_current`.
#[inline]
pub const fn is_supported_ir_version(
    version: IrVersion,
) -> bool {
    version.is_supported_by_current()
}

/// Returns whether two IR versions belong to the same major contract.
#[inline]
pub const fn same_ir_major(
    left: IrVersion,
    right: IrVersion,
) -> bool {
    left.major() == right.major()
}

/// Returns whether an IR version is newer than the current implementation.
#[inline]
pub const fn is_future_ir_version(
    version: IrVersion,
) -> bool {
    version > IrVersion::CURRENT
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Version tests
    // -------------------------------------------------------------------------

    #[test]
    fn current_version_is_stable() {
        let version = IrVersion::CURRENT;

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 0);
        assert_eq!(version.patch(), 0);

        assert!(version.is_current());
        assert_eq!(
            version.to_string(),
            "1.0.0"
        );
    }

    #[test]
    fn default_version_is_current() {
        assert_eq!(
            IrVersion::default(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn version_ordering_is_deterministic() {
        let v1 = IrVersion::new(1, 0, 0);
        let v2 = IrVersion::new(1, 0, 1);
        let v3 = IrVersion::new(1, 1, 0);
        let v4 = IrVersion::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);

        assert!(v4.is_newer_than_current());
    }

    #[test]
    fn same_major_is_explicit() {
        let current = IrVersion::CURRENT;
        let patch = IrVersion::new(1, 0, 1);
        let minor = IrVersion::new(1, 1, 0);
        let major = IrVersion::new(2, 0, 0);

        assert!(current.same_major(patch));
        assert!(current.same_major(minor));
        assert!(!current.same_major(major));
    }

    #[test]
    fn older_same_major_version_is_supported() {
        let old = IrVersion::new(1, 0, 0);

        assert!(old.is_supported_by_current());
        assert!(is_supported_ir_version(old));
    }

    #[test]
    fn future_minor_version_is_rejected() {
        let future = IrVersion::new(1, 1, 0);

        assert!(!future.is_supported_by_current());
        assert!(!is_supported_ir_version(future));
    }

    #[test]
    fn future_major_version_is_rejected() {
        let future = IrVersion::new(2, 0, 0);

        assert!(!future.is_supported_by_current());
        assert!(!is_supported_ir_version(future));
    }

    #[test]
    fn future_patch_version_is_rejected() {
        let future = IrVersion::new(1, 0, 1);

        assert!(!future.is_supported_by_current());
        assert!(!is_supported_ir_version(future));
    }

    #[test]
    fn current_version_is_not_future() {
        assert!(
            !is_future_ir_version(
                IrVersion::CURRENT
            )
        );
    }

    #[test]
    fn future_version_is_detected() {
        assert!(
            is_future_ir_version(
                IrVersion::new(1, 1, 0)
            )
        );

        assert!(
            is_future_ir_version(
                IrVersion::new(2, 0, 0)
            )
        );
    }

    #[test]
    fn version_support_is_directional() {
        let current = IrVersion::CURRENT;
        let older = IrVersion::new(1, 0, 0);
        let future = IrVersion::new(1, 1, 0);

        assert!(current.supports(older));
        assert!(!older.supports(current));
        assert!(!current.supports(future));
    }

    // -------------------------------------------------------------------------
    // Program identity
    // -------------------------------------------------------------------------

    #[test]
    fn program_identity_is_typed() {
        let id = ProgramId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(u64::from(id), 42);
        assert_eq!(
            id.to_string(),
            "program42"
        );
        assert_eq!(
            ProgramId::prefix(),
            "program"
        );
    }

    // -------------------------------------------------------------------------
    // Circuit identity
    // -------------------------------------------------------------------------

    #[test]
    fn circuit_identity_is_typed_and_opaque() {
        let id = CircuitId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(u64::from(id), 42);
        assert_eq!(
            id.to_string(),
            "c42"
        );
    }

    // -------------------------------------------------------------------------
    // Operation identity
    // -------------------------------------------------------------------------

    #[test]
    fn operation_identity_is_independent_of_position() {
        let first = OperationId::new(100);
        let second = OperationId::new(101);

        assert_ne!(first, second);

        assert_eq!(
            first.value(),
            100
        );

        assert_eq!(
            second.value(),
            101
        );

        assert_eq!(
            first.to_string(),
            "op100"
        );

        assert_eq!(
            second.to_string(),
            "op101"
        );
    }

    // -------------------------------------------------------------------------
    // Namespace identity
    // -------------------------------------------------------------------------

    #[test]
    fn namespace_identity_is_typed() {
        let id = NamespaceId::new(7);

        assert_eq!(
            id.value(),
            7
        );

        assert_eq!(
            u64::from(id),
            7
        );

        assert_eq!(
            id.to_string(),
            "ns7"
        );
    }

    // -------------------------------------------------------------------------
    // Additional identity types
    // -------------------------------------------------------------------------

    #[test]
    fn all_core_identity_types_are_independent() {
        let program = ProgramId::new(1);
        let circuit = CircuitId::new(1);
        let operation = OperationId::new(1);
        let region = RegionId::new(1);
        let block = BlockId::new(1);
        let value = ValueId::new(1);
        let parameter = ParameterId::new(1);
        let pulse = PulseId::new(1);
        let waveform = WaveformId::new(1);
        let channel = ChannelId::new(1);
        let frame = FrameId::new(1);
        let schedule = ScheduleId::new(1);
        let resource = ResourceId::new(1);
        let capability = CapabilityId::new(1);

        assert_eq!(
            program.value(),
            1
        );

        assert_eq!(
            circuit.value(),
            1
        );

        assert_eq!(
            operation.value(),
            1
        );

        assert_eq!(
            region.value(),
            1
        );

        assert_eq!(
            block.value(),
            1
        );

        assert_eq!(
            value.value(),
            1
        );

        assert_eq!(
            parameter.value(),
            1
        );

        assert_eq!(
            pulse.value(),
            1
        );

        assert_eq!(
            waveform.value(),
            1
        );

        assert_eq!(
            channel.value(),
            1
        );

        assert_eq!(
            frame.value(),
            1
        );

        assert_eq!(
            schedule.value(),
            1
        );

        assert_eq!(
            resource.value(),
            1
        );

        assert_eq!(
            capability.value(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Large identity tests
    // -------------------------------------------------------------------------

    #[test]
    fn identities_support_large_u64_values() {
        let maximum = u64::MAX;

        let program = ProgramId::new(maximum);
        let operation = OperationId::new(maximum);
        let pulse = PulseId::new(maximum);

        assert_eq!(
            program.value(),
            maximum
        );

        assert_eq!(
            operation.value(),
            maximum
        );

        assert_eq!(
            pulse.value(),
            maximum
        );
    }

    #[test]
    fn identity_zero_is_valid() {
        assert_eq!(
            ProgramId::new(0).value(),
            0
        );

        assert_eq!(
            CircuitId::new(0).value(),
            0
        );

        assert_eq!(
            OperationId::new(0).value(),
            0
        );
    }

    // -------------------------------------------------------------------------
    // Ordering tests
    // -------------------------------------------------------------------------

    #[test]
    fn identities_are_deterministically_orderable() {
        assert!(
            ProgramId::new(1)
                < ProgramId::new(2)
        );

        assert!(
            CircuitId::new(1)
                < CircuitId::new(2)
        );

        assert!(
            OperationId::new(1)
                < OperationId::new(2)
        );

        assert!(
            RegionId::new(1)
                < RegionId::new(2)
        );

        assert!(
            BlockId::new(1)
                < BlockId::new(2)
        );
    }

    // -------------------------------------------------------------------------
    // Hash/equality contract
    // -------------------------------------------------------------------------

    #[test]
    fn equal_identity_values_are_equal() {
        let first = OperationId::new(123);
        let second = OperationId::new(123);
        let third = OperationId::new(124);

        assert_eq!(
            first,
            second
        );

        assert_ne!(
            first,
            third
        );
    }

    // -------------------------------------------------------------------------
    // Display tests
    // -------------------------------------------------------------------------

    #[test]
    fn identity_display_is_deterministic() {
        assert_eq!(
            ProgramId::new(5).to_string(),
            "program5"
        );

        assert_eq!(
            CircuitId::new(5).to_string(),
            "c5"
        );

        assert_eq!(
            OperationId::new(5).to_string(),
            "op5"
        );

        assert_eq!(
            RegionId::new(5).to_string(),
            "region5"
        );

        assert_eq!(
            BlockId::new(5).to_string(),
            "block5"
        );

        assert_eq!(
            PulseId::new(5).to_string(),
            "pulse5"
        );

        assert_eq!(
            WaveformId::new(5).to_string(),
            "wave5"
        );

        assert_eq!(
            ChannelId::new(5).to_string(),
            "channel5"
        );

        assert_eq!(
            FrameId::new(5).to_string(),
            "frame5"
        );
    }

    // -------------------------------------------------------------------------
    // Architectural boundary tests
    // -------------------------------------------------------------------------

    #[test]
    fn identity_is_not_a_machine_size_limit() {
        let very_large_id = OperationId::new(
            u64::MAX
        );

        assert_eq!(
            very_large_id.value(),
            u64::MAX
        );

        // The identity itself does not impose a 63, 64, 4096, or other
        // machine-size boundary.
    }

    #[test]
    fn current_ir_version_helper_matches_constant() {
        assert_eq!(
            current_ir_version(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn major_helper_matches_version_fields() {
        let left = IrVersion::new(7, 2, 3);
        let right = IrVersion::new(7, 9, 10);
        let different = IrVersion::new(8, 0, 0);

        assert!(
            same_ir_major(left, right)
        );

        assert!(
            !same_ir_major(left, different)
        );
    }
}