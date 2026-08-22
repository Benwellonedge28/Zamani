//! Zamani Quantum IR — Identity and Version Contracts
//!
//! This module defines the identity and versioning primitives used by the
//! hardware-independent quantum IR.
//!
//! # Architectural boundary
//!
//! Identity is deliberately separated from the representation of gates,
//! measurements, qubits, and circuits. This module must remain independent of
//! those higher-level IR structures so it can be adopted by them without
//! creating dependency cycles.
//!
//! The distinction between logical and physical qubit identity remains owned
//! by `qubits.rs` (`QubitId` and `PhysicalQubitId`). This module does not
//! duplicate those types.
//!
//! `ClassicalBitId` likewise remains owned by the measurement IR until that
//! type is intentionally migrated into a shared identity namespace.
//!
//! # Versioning contract
//!
//! `IrVersion` identifies the schema and semantic contract of persisted or
//! exchanged IR. It is not a compiler version and it is not a hardware,
//! backend, or device version.
//!
//! Major versions may contain breaking IR changes.
//! Minor versions add compatible capabilities within the same major contract.
//! Patch versions are reserved for contract-preserving corrections.
//!
//! A consumer must never silently interpret a future IR version that it does
//! not explicitly understand.

use std::fmt;

// -----------------------------------------------------------------------------
// IR version
// -----------------------------------------------------------------------------

/// Version of the Zamani Quantum IR contract.
///
/// The version belongs to the IR itself rather than to the compiler package.
/// This allows persisted, exchanged, replayed, or cached IR to be validated
/// independently of the compiler release that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl IrVersion {
    /// Current stable Quantum IR contract.
    ///
    /// `1.0.0` is the initial production contract established by this IR
    /// boundary. Breaking representation or semantic changes require a new
    /// major version.
    pub const CURRENT: Self = Self::new(1, 0, 0);

    /// Creates an IR version.
    ///
    /// Version validation is structural because every component is represented
    /// by an unsigned integer. There are therefore no invalid negative or
    /// non-numeric versions to reject.
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

    /// Returns true when both versions belong to the same major contract.
    ///
    /// This only establishes that they belong to the same compatibility
    /// family. It does not mean that a consumer understands every feature
    /// introduced by the other version.
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns true when this is exactly the current production contract.
    pub const fn is_current(self) -> bool {
        self.major == Self::CURRENT.major
            && self.minor == Self::CURRENT.minor
            && self.patch == Self::CURRENT.patch
    }

    /// Returns true when this version is understood by the current IR
    /// implementation without requiring a major-version migration.
    ///
    /// Future minor versions are intentionally rejected. A consumer must not
    /// silently accept fields or semantics introduced after the version it
    /// understands.
    pub const fn is_supported_by_current(self) -> bool {
        self.major == Self::CURRENT.major
            && self.minor <= Self::CURRENT.minor
    }

    /// Returns true when this version is newer than `other`.
    pub const fn is_newer_than(
        self,
        other: Self,
    ) -> bool {
        self > other
    }

    /// Returns true when this version is older than `other`.
    pub const fn is_older_than(
        self,
        other: Self,
    ) -> bool {
        self < other
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

// -----------------------------------------------------------------------------
// Circuit identity
// -----------------------------------------------------------------------------

/// Stable opaque identity for a quantum circuit/program instance.
///
/// `CircuitId` identifies an IR object. It does not identify the contents of
/// the circuit and must not be treated as a content hash.
///
/// The IR deliberately does not generate global identifiers. Identifier
/// allocation belongs to the owning compiler session, workspace, persistence
/// layer, or application so that deterministic compilation does not depend on
/// hidden global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CircuitId(u64);

impl CircuitId {
    /// Creates a circuit identity from an application-controlled value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric representation.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for CircuitId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<CircuitId> for u64 {
    fn from(id: CircuitId) -> u64 {
        id.value()
    }
}

impl fmt::Display for CircuitId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "c{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Operation identity
// -----------------------------------------------------------------------------

/// Stable opaque identity for an operation in a quantum circuit.
///
/// Operation identity is intentionally independent of operation position.
/// Inserting or removing another operation must not change the identity of an
/// existing operation.
///
/// This is important for future optimization, diagnostics, replay, tracing,
/// provenance, and transformation passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationId(u64);

impl OperationId {
    /// Creates an operation identity from an application-controlled value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric representation.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for OperationId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<OperationId> for u64 {
    fn from(id: OperationId) -> u64 {
        id.value()
    }
}

impl fmt::Display for OperationId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "op{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Namespace identity
// -----------------------------------------------------------------------------

/// Stable opaque identity for an IR namespace.
///
/// A namespace identity identifies the scope containing logical resources;
/// it does not replace `QubitId`, `PhysicalQubitId`, or classical-bit IDs.
///
/// Most circuits can use a single implicit namespace. This type exists so
/// future multi-namespace or imported-module IR can introduce explicit scope
/// identity without changing the identity model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamespaceId(u64);

impl NamespaceId {
    /// Creates a namespace identity from an application-controlled value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric representation.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for NamespaceId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<NamespaceId> for u64 {
    fn from(id: NamespaceId) -> u64 {
        id.value()
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "ns{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Public contract helpers
// -----------------------------------------------------------------------------

/// Returns the current stable Quantum IR version.
#[inline]
pub const fn current_ir_version() -> IrVersion {
    IrVersion::CURRENT
}

/// Returns whether the supplied version can be consumed by the current IR
/// implementation without a major-version migration.
///
/// Future minor versions are rejected deliberately. This prevents an older
/// implementation from silently accepting a newer contract whose semantics it
/// may not understand.
#[inline]
pub const fn is_supported_ir_version(
    version: IrVersion,
) -> bool {
    version.is_supported_by_current()
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_is_stable() {
        let version = IrVersion::CURRENT;

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 0);
        assert_eq!(version.patch(), 0);

        assert!(version.is_current());
        assert_eq!(version.to_string(), "1.0.0");
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
        let v2 = IrVersion::new(1, 1, 0);
        let v3 = IrVersion::new(2, 0, 0);

        assert!(v1.is_older_than(v2));
        assert!(v2.is_older_than(v3));
        assert!(v3.is_newer_than(v1));
    }

    #[test]
    fn major_compatibility_is_explicit() {
        let current = IrVersion::CURRENT;
        let same_major = IrVersion::new(1, 0, 1);
        let breaking = IrVersion::new(2, 0, 0);

        assert!(current.same_major(same_major));
        assert!(!current.same_major(breaking));
    }

    #[test]
    fn current_version_is_supported() {
        assert!(
            is_supported_ir_version(
                IrVersion::CURRENT
            )
        );
    }

    #[test]
    fn future_minor_version_is_rejected() {
        let future = IrVersion::new(1, 1, 0);

        assert!(
            !is_supported_ir_version(future)
        );
    }

    #[test]
    fn future_major_version_is_rejected() {
        let future = IrVersion::new(2, 0, 0);

        assert!(
            !is_supported_ir_version(future)
        );
    }

    #[test]
    fn circuit_identity_is_typed_and_opaque() {
        let id = CircuitId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(u64::from(id), 42);
        assert_eq!(id.to_string(), "c42");
    }

    #[test]
    fn operation_identity_is_independent_of_position() {
        let first = OperationId::new(100);
        let second = OperationId::new(101);

        assert_ne!(first, second);
        assert_eq!(first.value(), 100);
        assert_eq!(second.value(), 101);
        assert_eq!(first.to_string(), "op100");
        assert_eq!(second.to_string(), "op101");
    }

    #[test]
    fn namespace_identity_is_typed() {
        let namespace = NamespaceId::new(7);

        assert_eq!(namespace.value(), 7);
        assert_eq!(u64::from(namespace), 7);
        assert_eq!(namespace.to_string(), "ns7");
    }

    #[test]
    fn identity_values_are_orderable() {
        assert!(
            CircuitId::new(1)
                < CircuitId::new(2)
        );

        assert!(
            OperationId::new(1)
                < OperationId::new(2)
        );

        assert!(
            NamespaceId::new(1)
                < NamespaceId::new(2)
        );
    }
}