
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Module Ports
//!
//! Path: `src/hdl/design/port.rs`
//! Rust baseline: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent logical ports on reusable HDL modules.
//!
//! A port records its canonical semantic identity, source-level name,
//! direction, semantic value type, optional logical interface association,
//! and optional source provenance. It does not duplicate the AST or the
//! shared HDL semantic type system.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/hdl/interfaces.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/module.rs`
//! - `src/hdl/design/interface.rs` (when implemented)
//!
//! Grammar owns source syntax. The canonical AST owns source structure.
//! The shared HDL semantic module owns the canonical direction, type,
//! width, declaration identity, and source-location vocabulary.
//! This module owns the design-layer port contract and its local invariants.
//!
//! ## Owns
//!
//! - Logical module-port declarations.
//! - Port direction and source-level name.
//! - References to canonical semantic type and identity values.
//! - Optional logical interface association.
//! - Optional source provenance.
//! - Local port validation and duplicate-name checking.
//! - Conservative, target-independent port compatibility.
//!
//! ## Does not own
//!
//! - Parsing, lexing, or a replacement AST.
//! - Name resolution or a second semantic type system.
//! - A second width-expression representation.
//! - Module-instance connection elaboration.
//! - Interface declaration or protocol implementation.
//! - Physical pins, package balls, board locations, or vendor resources.
//! - Hardware allocation, synthesis, placement, or routing.
//! - Simulation or runtime behavior.
//! - HDLIR data structures, builders, passes, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is
//! outside this file's scope.
//!
//! ## Portability and scalability
//!
//! A port describes a logical boundary, not a physical connector.
//! Symbolic widths remain symbolic. No maximum port count, vector width,
//! interface count, or hierarchy size is introduced here.
//!
//! Collections grow according to available resources. Concrete execution
//! remains finite and may be subject to explicit operational budgets.
//! Such budgets must not become language-level restrictions.
//!
//! ## Integration
//!
//! `design/module.rs` owns the logical module record. Its AST adapter or
//! later design-model integration associates validated ports with that
//! module. `design/instance.rs` and connectivity analysis consume the
//! port contract when validating logical connections. `design/interface.rs`
//! resolves interface references and checks interface-level compatibility.
//! Interoperability adapters translate external port declarations into this
//! contract while retaining provenance and reporting information lost by
//! conversion.
//!
//! This module deliberately does not depend on any of those consumers.
//!
//! ## Validation boundary
//!
//! Local construction validates only facts available in this value:
//! non-empty names and identities, legal port direction, valid type-width
//! structure, and valid source-location ranges.
//!
//! It cannot establish that a referenced interface exists, that a name is
//! unique in a module, or that a connected port is semantically valid.
//! Those checks require the owning design, symbol resolver, or semantic
//! environment and must remain explicit downstream operations.
//!
//! ## Freeze criteria
//!
//! - Existing canonical semantic types are reused.
//! - Symbolic widths are not evaluated or truncated during construction.
//! - Duplicate names are detected deterministically by the port collection.
//! - Compatibility never guesses equivalence between unrelated symbolic
//!   widths.
//! - No target-specific or HDLIR dependency is introduced.
//! - Unit tests and repository workspace checks pass.

use core::fmt;
use std::collections::BTreeSet;

use crate::frontend::semantic::domains::hdl::{
    HdlDirection, HdlId, HdlSourceLocation, HdlValueType, HdlWidth,
};

/// Schema version for the logical port contract.
///
/// This is an API/model version. It is not the Zamani language version,
/// AST schema version, or HDLIR schema version.
pub const HDL_PORT_SCHEMA_VERSION: u16 = 1;

/// A validated logical port on an HDL module.
///
/// `id` is the canonical semantic identity supplied by the frontend adapter.
/// `name` is the source-level port name. `direction` is relative to the
/// module boundary: an input is driven from outside the module, and an
/// output is driven by the module.
///
/// `value_type` uses the canonical HDL semantic type. It may contain
/// symbolic widths or named type references that require later resolution.
///
/// `interface` is an optional reference to a logical interface identity.
/// This reference is not proof that the interface exists or is compatible.
///
/// `location` preserves the source declaration range when available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlPort {
    id: HdlId,
    name: String,
    direction: HdlDirection,
    value_type: HdlValueType,
    interface: Option<HdlId>,
    location: Option<HdlSourceLocation>,
}

impl HdlPort {
    /// Construct a logical port and validate its local invariants.
    ///
    /// This does not resolve the type or interface, check uniqueness within
    /// a module, or validate connections to another port.
    ///
    /// # Errors
    ///
    /// Returns [`HdlPortError`] if the name is empty or has surrounding
    /// whitespace, the direction is `Internal`, the type contains a
    /// concrete zero width, or the source location is invalid.
    pub fn new(
        id: HdlId,
        name: impl Into<String>,
        direction: HdlDirection,
        value_type: HdlValueType,
        interface: Option<HdlId>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, HdlPortError> {
        let port = Self {
            id,
            name: name.into(),
            direction,
            value_type,
            interface,
            location,
        };

        port.validate()?;
        Ok(port)
    }

    /// Return the canonical semantic identity of this port.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the source-level port name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the direction relative to the containing module.
    #[must_use]
    pub const fn direction(&self) -> HdlDirection {
        self.direction
    }

    /// Return the canonical semantic value type.
    #[must_use]
    pub fn value_type(&self) -> &HdlValueType {
        &self.value_type
    }

    /// Return the optional logical interface identity.
    ///
    /// The returned identity is a reference, not a resolved interface
    /// declaration. Resolution belongs to the design/semantic layer.
    #[must_use]
    pub fn interface(&self) -> Option<&HdlId> {
        self.interface.as_ref()
    }

    /// Return the optional source declaration location.
    #[must_use]
    pub fn location(&self) -> Option<&HdlSourceLocation> {
        self.location.as_ref()
    }

    /// Validate this value, including values constructed by future
    /// deserialization or external adapters.
    ///
    /// The method validates local invariants only. It cannot check the
    /// existence of referenced identities or uniqueness in a module.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic order.
    pub fn validate(&self) -> Result<(), HdlPortError> {
        if self.name.trim().is_empty() {
            return Err(HdlPortError::EmptyName);
        }

        if self.name.trim() != self.name {
            return Err(HdlPortError::SurroundingWhitespaceInName);
        }

        if self.direction == HdlDirection::Internal {
            return Err(HdlPortError::InternalDirection);
        }

        validate_value_type(&self.value_type)?;

        if let Some(location) = &self.location {
            if location.source.trim().is_empty() {
                return Err(HdlPortError::EmptySourceIdentity);
            }

            if location.end < location.start {
                return Err(HdlPortError::InvalidSourceRange {
                    start: location.start,
                    end: location.end,
                });
            }
        }

        Ok(())
    }

    /// Compare this port with another port as a pair of module-boundary
    /// declarations.
    ///
    /// This is a conservative declaration-compatibility check, not a
    /// complete connection check. Directions must match because the method
    /// compares declarations in the same boundary role. Instance connection
    /// validation must account for the direction reversal between a parent
    /// expression and a child module's formal port.
    ///
    /// Concrete widths are compared directly. Identical symbolic width
    /// expressions are considered structurally compatible. Different
    /// symbolic expressions are indeterminate unless a semantic resolver
    /// proves their relationship.
    ///
    /// The method does not infer that two named types or interface
    /// references are equivalent merely because their names look similar.
    #[must_use]
    pub fn compatibility_with(&self, other: &Self) -> PortCompatibility {
        if self.direction != other.direction {
            return PortCompatibility::Incompatible(
                PortIncompatibility::DirectionMismatch {
                    expected: self.direction,
                    actual: other.direction,
                },
            );
        }

        match (&self.interface, &other.interface) {
            (Some(left), Some(right)) if left != right => {
                return PortCompatibility::Incompatible(
                    PortIncompatibility::InterfaceMismatch,
                );
            }
            (Some(_), None) | (None, Some(_)) => {
                return PortCompatibility::Indeterminate(
                    PortIndeterminacy::InterfaceAssociationIncomplete,
                );
            }
            _ => {}
        }

        compare_value_types(&self.value_type, &other.value_type)
    }
}

/// An ordered collection of logical ports.
///
/// The collection preserves declaration order and rejects duplicate
/// source-level names. It does not impose a maximum length.
///
/// This type is useful to the module design model, but it does not replace
/// module declarations or the canonical AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlPortList {
    ports: Vec<HdlPort>,
}

impl HdlPortList {
    /// Create an empty port list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a port list from declarations in source order.
    ///
    /// # Errors
    ///
    /// Returns [`HdlPortListError::DuplicateName`] if two ports have the
    /// same source-level name, or [`HdlPortListError::InvalidPort`] if a
    /// port violates its local invariants.
    pub fn from_ports(ports: Vec<HdlPort>) -> Result<Self, HdlPortListError> {
        let list = Self { ports };
        list.validate()?;
        Ok(list)
    }

    /// Append a port, preserving declaration order.
    ///
    /// The list is unchanged if validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid port or duplicate source-level name.
    pub fn push(&mut self, port: HdlPort) -> Result<(), HdlPortListError> {
        port.validate().map_err(HdlPortListError::InvalidPort)?;

        if self.ports.iter().any(|existing| existing.name == port.name) {
            return Err(HdlPortListError::DuplicateName {
                name: port.name.clone(),
            });
        }

        self.ports.push(port);
        Ok(())
    }

    /// Return all ports in declaration order.
    #[must_use]
    pub fn as_slice(&self) -> &[HdlPort] {
        &self.ports
    }

    /// Return the number of ports.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ports.len()
    }

    /// Return whether this list has no ports.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    /// Find a port by its exact source-level name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&HdlPort> {
        self.ports.iter().find(|port| port.name == name)
    }

    /// Validate all ports and ensure source-level names are unique.
    ///
    /// The first error follows declaration order. Name equality is exact;
    /// any language-specific identifier normalization must be performed by
    /// the canonical name-resolution layer before this collection is built.
    ///
    /// # Errors
    ///
    /// Returns the first invalid port or duplicate name.
    pub fn validate(&self) -> Result<(), HdlPortListError> {
        let mut names = BTreeSet::new();

        for port in &self.ports {
            port.validate().map_err(HdlPortListError::InvalidPort)?;

            if !names.insert(port.name.as_str()) {
                return Err(HdlPortListError::DuplicateName {
                    name: port.name.clone(),
                });
            }
        }

        Ok(())
    }
}

/// A reason why two ports are incompatible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortIncompatibility {
    /// The ports have different boundary directions.
    DirectionMismatch {
        /// Direction required by the comparison.
        expected: HdlDirection,
        /// Direction found on the other port.
        actual: HdlDirection,
    },
    /// Their type categories differ.
    TypeCategoryMismatch,
    /// Concrete widths differ.
    WidthMismatch {
        /// Width of the first port.
        left: u128,
        /// Width of the second port.
        right: u128,
    },
    /// Signedness differs.
    SignednessMismatch,
    /// One port permits a different logic-state model.
    ///
    /// The semantic `HdlValueType` does not encode a two-state/four-state
    /// distinction, so this variant is reserved for future adapters that
    /// carry that information into a richer comparison contract.
    LogicStateModelMismatch,
    /// The ports refer to different logical interfaces.
    InterfaceMismatch,
    /// The named semantic types differ.
    NamedTypeMismatch {
        /// Identity of the first named type.
        left: HdlId,
        /// Identity of the second named type.
        right: HdlId,
    },
}

impl fmt::Display for PortIncompatibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectionMismatch { expected, actual } => {
                write!(
                    f,
                    "port direction mismatch: expected {expected:?}, found {actual:?}"
                )
            }
            Self::TypeCategoryMismatch => {
                f.write_str("port value-type categories differ")
            }
            Self::WidthMismatch { left, right } => {
                write!(f, "port widths differ: {left} versus {right}")
            }
            Self::SignednessMismatch => {
                f.write_str("port signedness differs")
            }
            Self::LogicStateModelMismatch => {
                f.write_str("port logic-state models differ")
            }
            Self::InterfaceMismatch => {
                f.write_str("ports refer to different logical interfaces")
            }
            Self::NamedTypeMismatch { left, right } => {
                write!(f, "named port types differ: `{left}` versus `{right}`")
            }
        }
    }
}

/// A reason why port compatibility cannot yet be established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortIndeterminacy {
    /// At least one width is symbolic and the expressions are not identical.
    SymbolicWidthRelationshipUnknown,
    /// Named type identity or its definition needs semantic resolution.
    NamedTypeNeedsResolution,
    /// One or both ports have incomplete interface association information.
    InterfaceAssociationIncomplete,
}

impl fmt::Display for PortIndeterminacy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolicWidthRelationshipUnknown => {
                f.write_str("symbolic width relationship is unresolved")
            }
            Self::NamedTypeNeedsResolution => {
                f.write_str("named port type requires semantic resolution")
            }
            Self::InterfaceAssociationIncomplete => {
                f.write_str("interface association is incomplete")
            }
        }
    }
}

/// Result of conservative port declaration compatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortCompatibility {
    /// Compatibility is established from the available declaration facts.
    Compatible,
    /// A known conflict makes the declarations incompatible.
    Incompatible(PortIncompatibility),
    /// More semantic information is required to decide compatibility.
    Indeterminate(PortIndeterminacy),
}

impl PortCompatibility {
    /// Return whether compatibility has been established.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Return whether a definite incompatibility has been found.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible(_))
    }

    /// Return whether additional semantic information is required.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate(_))
    }
}

/// Errors found while constructing or validating one port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlPortError {
    /// The source-level port name is empty or whitespace-only.
    EmptyName,
    /// The source-level port name has leading or trailing whitespace.
    SurroundingWhitespaceInName,
    /// An internal declaration was supplied where a module port is required.
    InternalDirection,
    /// A concrete width is zero.
    ZeroWidth,
    /// The source identity in a source location is empty.
    EmptySourceIdentity,
    /// A source location has an invalid half-open range.
    InvalidSourceRange {
        /// Inclusive start offset.
        start: u64,
        /// Exclusive end offset.
        end: u64,
    },
}

impl fmt::Display for HdlPortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("HDL port name cannot be empty"),
            Self::SurroundingWhitespaceInName => {
                f.write_str("HDL port name cannot have surrounding whitespace")
            }
            Self::InternalDirection => {
                f.write_str("an internal declaration cannot be a module port")
            }
            Self::ZeroWidth => f.write_str("HDL port type contains a zero width"),
            Self::EmptySourceIdentity => {
                f.write_str("HDL port source identity cannot be empty")
            }
            Self::InvalidSourceRange { start, end } => {
                write!(
                    f,
                    "invalid HDL port source range: start {start} exceeds end {end}"
                )
            }
        }
    }
}

impl std::error::Error for HdlPortError {}

/// Errors found while constructing or validating a port list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlPortListError {
    /// A port failed its local validation.
    InvalidPort(HdlPortError),
    /// Two ports use the same source-level name.
    DuplicateName {
        /// Repeated source-level name.
        name: String,
    },
}

impl fmt::Display for HdlPortListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort(error) => write!(f, "invalid HDL port: {error}"),
            Self::DuplicateName { name } => {
                write!(f, "duplicate HDL port name `{name}`")
            }
        }
    }
}

impl std::error::Error for HdlPortListError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPort(error) => Some(error),
            Self::DuplicateName { .. } => None,
        }
    }
}

/// Validate the canonical HDL value type without resolving symbolic widths.
///
/// An explicit stack avoids recursively traversing arbitrarily deep
/// width-expression trees on the Rust call stack.
fn validate_value_type(value_type: &HdlValueType) -> Result<(), HdlPortError> {
    let mut pending: Vec<&HdlWidth> = Vec::new();

    if let HdlValueType::BitVector { width, .. } = value_type {
        pending.push(width);
    }

    while let Some(width) = pending.pop() {
        match width {
            HdlWidth::Known(0) => return Err(HdlPortError::ZeroWidth),
            HdlWidth::Known(_) | HdlWidth::Symbol(_) | HdlWidth::Unresolved(_) => {}
            HdlWidth::Add(left, right) | HdlWidth::Multiply(left, right) => {
                pending.push(right);
                pending.push(left);
            }
        }
    }

    Ok(())
}

/// Compare canonical semantic value types conservatively.
fn compare_value_types(
    left: &HdlValueType,
    right: &HdlValueType,
) -> PortCompatibility {
    match (left, right) {
        (HdlValueType::Bit, HdlValueType::Bit)
        | (HdlValueType::Clock, HdlValueType::Clock)
        | (HdlValueType::Reset, HdlValueType::Reset) => PortCompatibility::Compatible,

        (
            HdlValueType::BitVector {
                width: left_width,
                signedness: left_signedness,
            },
            HdlValueType::BitVector {
                width: right_width,
                signedness: right_signedness,
            },
        ) => {
            if left_signedness != right_signedness {
                return PortCompatibility::Incompatible(
                    PortIncompatibility::SignednessMismatch,
                );
            }

            match compare_width(left_width, right_width) {
                WidthRelation::Equal => PortCompatibility::Compatible,
                WidthRelation::Different(left, right) => {
                    PortCompatibility::Incompatible(
                        PortIncompatibility::WidthMismatch { left, right },
                    )
                }
                WidthRelation::Unknown => PortCompatibility::Indeterminate(
                    PortIndeterminacy::SymbolicWidthRelationshipUnknown,
                ),
            }
        }

        (HdlValueType::Named(left), HdlValueType::Named(right)) if left == right => {
            PortCompatibility::Compatible
        }

        (HdlValueType::Named(left), HdlValueType::Named(right)) => {
            PortCompatibility::Incompatible(
                PortIncompatibility::NamedTypeMismatch {
                    left: left.clone(),
                    right: right.clone(),
                },
            )
        }

        (HdlValueType::Named(_), _) | (_, HdlValueType::Named(_)) => {
            PortCompatibility::Indeterminate(
                PortIndeterminacy::NamedTypeNeedsResolution,
            )
        }

        _ => PortCompatibility::Incompatible(
            PortIncompatibility::TypeCategoryMismatch,
        ),
    }
}

enum WidthRelation {
    Equal,
    Different(u128, u128),
    Unknown,
}

/// Compare widths without guessing the meaning of symbolic expressions.
fn compare_width(left: &HdlWidth, right: &HdlWidth) -> WidthRelation {
    if left == right {
        return WidthRelation::Equal;
    }

    match (left, right) {
        (HdlWidth::Known(left), HdlWidth::Known(right)) => {
            WidthRelation::Different(*left, *right)
        }
        _ => WidthRelation::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity should be valid")
    }

    fn port(
        name: &str,
        direction: HdlDirection,
        value_type: HdlValueType,
    ) -> HdlPort {
        HdlPort::new(
            id(&format!("port::{name}")),
            name,
            direction,
            value_type,
            None,
            None,
        )
        .expect("test port should be valid")
    }

    fn vector(width: HdlWidth, signedness: super::super::super::super::frontend::semantic::domains::hdl::HdlSignedness) -> HdlValueType {
        HdlValueType::BitVector { width, signedness }
    }

    #[test]
    fn constructs_a_valid_input_port() {
        let value = port("data", HdlDirection::Input, HdlValueType::Bit);

        assert_eq!(value.name(), "data");
        assert_eq!(value.direction(), HdlDirection::Input);
        assert_eq!(value.value_type(), &HdlValueType::Bit);
        assert!(value.interface().is_none());
        assert!(value.location().is_none());
    }

    #[test]
    fn rejects_empty_and_whitespace_names() {
        assert_eq!(
            HdlPort::new(
                id("p"),
                "   ",
                HdlDirection::Input,
                HdlValueType::Bit,
                None,
                None,
            ),
            Err(HdlPortError::EmptyName)
        );

        assert_eq!(
            HdlPort::new(
                id("p"),
                " data ",
                HdlDirection::Input,
                HdlValueType::Bit,
                None,
                None,
            ),
            Err(HdlPortError::SurroundingWhitespaceInName)
        );
    }

    #[test]
    fn rejects_internal_direction() {
        assert_eq!(
            HdlPort::new(
                id("p"),
                "data",
                HdlDirection::Internal,
                HdlValueType::Bit,
                None,
                None,
            ),
            Err(HdlPortError::InternalDirection)
        );
    }

    #[test]
    fn rejects_zero_width_even_when_the_enum_is_constructed_directly() {
        assert_eq!(
            HdlPort::new(
                id("p"),
                "data",
                HdlDirection::Input,
                HdlValueType::BitVector {
                    width: HdlWidth::Known(0),
                    signedness:
                        crate::frontend::semantic::domains::hdl::HdlSignedness::Unsigned,
                },
                None,
                None,
            ),
            Err(HdlPortError::ZeroWidth)
        );
    }

    #[test]
    fn accepts_symbolic_widths_without_resolving_them() {
        let value = port(
            "payload",
            HdlDirection::Input,
            HdlValueType::BitVector {
                width: HdlWidth::Symbol(id("payload_width")),
                signedness:
                    crate::frontend::semantic::domains::hdl::HdlSignedness::Unsigned,
            },
        );

        assert!(matches!(
            value.value_type(),
            HdlValueType::BitVector {
                width: HdlWidth::Symbol(_),
                ..
            }
        ));
    }

    #[test]
    fn compatibility_detects_direction_mismatch() {
        let input = port("data", HdlDirection::Input, HdlValueType::Bit);
        let output = port("data", HdlDirection::Output, HdlValueType::Bit);

        assert_eq!(
            input.compatibility_with(&output),
            PortCompatibility::Incompatible(
                PortIncompatibility::DirectionMismatch {
                    expected: HdlDirection::Input,
                    actual: HdlDirection::Output,
                }
            )
        );
    }

    #[test]
    fn compatibility_detects_concrete_width_mismatch() {
        use crate::frontend::semantic::domains::hdl::HdlSignedness;

        let left = port(
            "data",
            HdlDirection::Input,
            vector(HdlWidth::Known(8), HdlSignedness::Unsigned),
        );
        let right = port(
            "data",
            HdlDirection::Input,
            vector(HdlWidth::Known(16), HdlSignedness::Unsigned),
        );

        assert_eq!(
            left.compatibility_with(&right),
            PortCompatibility::Incompatible(
                PortIncompatibility::WidthMismatch {
                    left: 8,
                    right: 16,
                }
            )
        );
    }

    #[test]
    fn identical_symbolic_widths_are_compatible() {
        use crate::frontend::semantic::domains::hdl::HdlSignedness;

        let width = HdlWidth::Symbol(id("width"));
        let left = port(
            "data",
            HdlDirection::Input,
            vector(width.clone(), HdlSignedness::Unsigned),
        );
        let right = port(
            "data",
            HdlDirection::Input,
            vector(width, HdlSignedness::Unsigned),
        );

        assert_eq!(
            left.compatibility_with(&right),
            PortCompatibility::Compatible
        );
    }

    #[test]
    fn unrelated_symbolic_widths_are_indeterminate() {
        use crate::frontend::semantic::domains::hdl::HdlSignedness;

        let left = port(
            "data",
            HdlDirection::Input,
            vector(
                HdlWidth::Symbol(id("left_width")),
                HdlSignedness::Unsigned,
            ),
        );
        let right = port(
            "data",
            HdlDirection::Input,
            vector(
                HdlWidth::Symbol(id("right_width")),
                HdlSignedness::Unsigned,
            ),
        );

        assert_eq!(
            left.compatibility_with(&right),
            PortCompatibility::Indeterminate(
                PortIndeterminacy::SymbolicWidthRelationshipUnknown
            )
        );
    }

    #[test]
    fn port_list_preserves_order_and_rejects_duplicate_names() {
        let first = port("clock", HdlDirection::Input, HdlValueType::Clock);
        let second = port("ready", HdlDirection::Output, HdlValueType::Bit);

        let list = HdlPortList::from_ports(vec![first.clone(), second.clone()])
            .expect("unique ports should be accepted");

        assert_eq!(list.len(), 2);
        assert_eq!(list.as_slice()[0], first);
        assert_eq!(list.as_slice()[1], second);
        assert_eq!(list.get("ready"), Some(&second));

        assert_eq!(
            HdlPortList::from_ports(vec![
                port("data", HdlDirection::Input, HdlValueType::Bit),
                port("data", HdlDirection::Output, HdlValueType::Bit),
            ]),
            Err(HdlPortListError::DuplicateName {
                name: "data".to_owned(),
            })
        );
    }

    #[test]
    fn failed_push_does_not_mutate_the_list() {
        let existing = port("data", HdlDirection::Input, HdlValueType::Bit);
        let duplicate = port("data", HdlDirection::Output, HdlValueType::Bit);
        let mut list = HdlPortList::from_ports(vec![existing])
            .expect("initial list should be valid");

        assert!(matches!(
            list.push(duplicate),
            Err(HdlPortListError::DuplicateName { .. })
        ));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn empty_port_list_is_valid() {
        let list = HdlPortList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert!(list.validate().is_ok());
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(HDL_PORT_SCHEMA_VERSION, 1);
    }
}
