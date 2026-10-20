
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Logical Module Ports
//!
//! Path: `src/hdl/hdlir/ir/ports.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed core implementation; freeze after repository integration.
//!
//! ## Purpose
//!
//! Represent logical module-boundary ports and an ordered, checked collection
//! of those declarations. A port describes an interface boundary; it does not
//! describe a physical pin, package pad, FPGA resource, or vendor handle.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/port.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/types/signal.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//!
//! The grammar owns syntax and the canonical frontend owns source semantics.
//! HDLIR owns this hardware-domain representation. ZUIR remains the universal
//! computational representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Logical port identity, name, direction, signal type, modes and optional
//!   default-value reference.
//! - Validation of local port invariants.
//! - An ordered port collection with unique identities and names.
//!
//! This module does not own:
//!
//! - Parsing, source ASTs, name resolution or source-level semantic checking.
//! - A competing signal type or value representation.
//! - Port-to-net connectivity, driver resolution or instance port mapping.
//! - Clock-tree synthesis, timing closure or physical pin assignment.
//! - Evaluation of symbolic widths or default-value expressions.
//! - Target discovery, synthesis, placement, routing or execution.
//! - A replacement for the repository's canonical source-level `HdlPort`.
//!
//! ## Integration
//!
//! `ir/module.rs` stores ordered `HdlirPortId` references. A containing design
//! graph or builder must ensure those IDs resolve to declarations in this
//! collection and that each declaration belongs to the intended module.
//! `verify/structure.rs` must check reference resolution and ownership.
//! `verify/types.rs` must check type and width constraints.
//! `verify/connectivity.rs` must check connections and driver/sink rules.
//! `lowering/from_semantic.rs` must map canonical semantic port declarations
//! into this representation while preserving source provenance.
//!
//! ## Scalability
//!
//! No universal maximum port count or width is imposed. The collection grows
//! subject to available memory and representable identifiers. Symbolic widths
//! remain inside `SignalType`; this module does not evaluate them prematurely.
//! Host-sized collection indexes are implementation details, not language
//! limits. No finite implementation can process literally infinite input.
//!
//! ## Freeze criteria
//!
//! Local validation, duplicate rejection, declaration ordering, lookup,
//! symbolic type preservation and error behavior must pass unit tests.
//! Full production qualification also requires graph-level reference checks,
//! attribute/constraint preservation, frontend lowering, and repository-wide
//! build, test and lint checks.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{HdlirPortId, HdlirValueId};
use crate::hdl::hdlir::types::signal::SignalType;

/// Direction of a logical port relative to its containing module.
///
/// These directions describe the module boundary. They do not identify a
/// physical pin direction or imply that a target supports the port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirPortDirection {
    /// Information enters the module through this port.
    Input,

    /// Information leaves the module through this port.
    Output,

    /// The port can both receive and drive information under the applicable
    /// HDL semantics.
    InOut,
}

/// Source-level port mode retained in the hardware-domain representation.
///
/// The grammar currently defines `const`, `mut`, and `ref` modes. This enum
/// preserves those declarations; their complete legality and interaction are
/// determined by the normative language and semantic contracts.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirPortMode {
    /// The port is declared with the `const` mode.
    Const,

    /// The port is declared with the `mut` mode.
    Mut,

    /// The port is declared with the `ref` mode.
    Ref,
}

/// A logical module-boundary port declaration.
///
/// `default_value` is a reference to a value owned by the HDLIR value system.
/// This type does not evaluate that value or assume that a default is legal
/// for every port direction. Such checks belong to semantic lowering and
/// verification.
///
/// Port modes are retained in declaration order. The constructor rejects
/// repeated modes but does not invent additional mode-compatibility rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirPort {
    id: HdlirPortId,
    name: String,
    direction: HdlirPortDirection,
    signal_type: SignalType,
    modes: Vec<HdlirPortMode>,
    default_value: Option<HdlirValueId>,
}

impl HdlirPort {
    /// Construct a logical port and validate its local invariants.
    ///
    /// The port name is retained exactly. Empty names, whitespace-only names,
    /// and names with leading or trailing whitespace are rejected. Name
    /// resolution, Unicode normalization and case folding are not performed.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error for an invalid name, or a duplicate-entity
    /// error if the mode list repeats a mode.
    pub fn new(
        id: HdlirPortId,
        name: impl Into<String>,
        direction: HdlirPortDirection,
        signal_type: SignalType,
        modes: Vec<HdlirPortMode>,
        default_value: Option<HdlirValueId>,
    ) -> HdlirResult<Self> {
        let port = Self {
            id,
            name: name.into(),
            direction,
            signal_type,
            modes,
            default_value,
        };

        port.validate_structure()?;
        Ok(port)
    }

    /// Return this port's HDLIR identity.
    #[must_use]
    pub const fn id(&self) -> HdlirPortId {
        self.id
    }

    /// Return the exact logical name of this port.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the direction relative to the containing module.
    #[must_use]
    pub const fn direction(&self) -> HdlirPortDirection {
        self.direction
    }

    /// Return the logical signal type.
    ///
    /// Symbolic widths and unresolved type details remain symbolic according
    /// to the underlying `SignalType` contract.
    #[must_use]
    pub fn signal_type(&self) -> &SignalType {
        &self.signal_type
    }

    /// Return the retained modes in declaration order.
    #[must_use]
    pub fn modes(&self) -> &[HdlirPortMode] {
        &self.modes
    }

    /// Return the optional HDLIR default-value reference.
    ///
    /// The referenced value's existence, ownership and type compatibility
    /// must be checked by the graph verifier.
    #[must_use]
    pub const fn default_value(&self) -> Option<HdlirValueId> {
        self.default_value
    }

    /// Recheck the port's local invariants.
    ///
    /// This is useful at trust boundaries such as deserialization or
    /// migration, where a value may not have been created by `new`.
    ///
    /// This method does not validate external references or perform semantic
    /// type checking.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the name or mode list is invalid.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "port_name",
                "port name must not be empty or whitespace-only",
            ));
        }

        if self.name.trim() != self.name {
            return Err(HdlirError::invalid_input(
                "port_name",
                "port name must not have leading or trailing whitespace",
            ));
        }

        for (index, mode) in self.modes.iter().enumerate() {
            if self.modes[..index].contains(mode) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR port mode",
                    identity: format!("{mode:?}"),
                    scope: Some(format!("port `{}`", self.name)),
                });
            }
        }

        Ok(())
    }
}

/// Ordered collection of logical port declarations.
///
/// The vector is authoritative for declaration order. Ordered lookup maps
/// provide deterministic lookup by identity and exact name. A failed
/// insertion does not modify the collection.
///
/// The collection enforces local uniqueness only. Ownership by a particular
/// module and references from `HdlirModule` are checked by the containing
/// graph or verifier.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirPortList {
    ports: Vec<HdlirPort>,
    by_id: BTreeMap<HdlirPortId, usize>,
    by_name: BTreeMap<String, usize>,
}

impl HdlirPortList {
    /// Create an empty port collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a port collection from declarations in source order.
    ///
    /// # Errors
    ///
    /// Returns the first validation or uniqueness error. No partially
    /// constructed collection is returned.
    pub fn from_ports<I>(ports: I) -> HdlirResult<Self>
    where
        I: IntoIterator<Item = HdlirPort>,
    {
        let mut list = Self::new();

        for port in ports {
            list.insert(port)?;
        }

        Ok(list)
    }

    /// Insert a port while preserving declaration order.
    ///
    /// The port is appended only after its local invariants and both
    /// uniqueness constraints have been checked.
    ///
    /// # Errors
    ///
    /// Returns a duplicate-entity error if another port has the same ID or
    /// exact name. Returns a validation error if the port is malformed.
    pub fn insert(&mut self, port: HdlirPort) -> HdlirResult<()> {
        port.validate_structure()?;

        let id = port.id();
        let name = port.name().to_owned();

        if self.by_id.contains_key(&id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR port ID",
                identity: id.to_string(),
                scope: None,
            });
        }

        if self.by_name.contains_key(&name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR port name",
                identity: name,
                scope: None,
            });
        }

        // The vector is the canonical ordered representation. Its length is
        // a host collection index, not a hardware or language-level limit.
        let index = self.ports.len();

        self.ports.push(port);
        self.by_id.insert(id, index);
        self.by_name.insert(
            self.ports[index].name().to_owned(),
            index,
        );

        Ok(())
    }

    /// Return all declarations in their original insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[HdlirPort] {
        &self.ports
    }

    /// Iterate over declarations in their original insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &HdlirPort> {
        self.ports.iter()
    }

    /// Iterate over port IDs in declaration order.
    pub fn ids(&self) -> impl ExactSizeIterator<Item = HdlirPortId> + '_ {
        self.ports.iter().map(HdlirPort::id)
    }

    /// Look up a declaration by HDLIR identity.
    #[must_use]
    pub fn get(&self, id: HdlirPortId) -> Option<&HdlirPort> {
        self.by_id
            .get(&id)
            .and_then(|index| self.ports.get(*index))
    }

    /// Look up a declaration by exact logical name.
    ///
    /// This performs no case folding, normalization, or source-language name
    /// resolution.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&HdlirPort> {
        self.by_name
            .get(name)
            .and_then(|index| self.ports.get(*index))
    }

    /// Return the number of declared ports.
    ///
    /// This is the host collection's current length, not a language-level
    /// port-count limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ports.len()
    }

    /// Return whether the collection contains no ports.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    /// Validate declarations and ensure the lookup indexes match the ordered
    /// collection.
    ///
    /// This checks local declarations, unique identities/names, and index
    /// consistency. It does not resolve default-value references, verify
    /// widths, check module ownership, or validate connectivity.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a declaration or internal index is
    /// inconsistent.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.by_id.len() != self.ports.len()
            || self.by_name.len() != self.ports.len()
        {
            return Err(HdlirError::invalid_invariant(
                "port_list_indexes_match_declarations",
                "port lookup index cardinality differs from declaration count",
            ));
        }

        for (index, port) in self.ports.iter().enumerate() {
            port.validate_structure()?;

            if self.by_id.get(&port.id()) != Some(&index) {
                return Err(HdlirError::invalid_invariant(
                    "port_id_index_matches_declaration_order",
                    format!(
                        "port ID {} has an inconsistent lookup index",
                        port.id()
                    ),
                ));
            }

            if self.by_name.get(port.name()) != Some(&index) {
                return Err(HdlirError::invalid_invariant(
                    "port_name_index_matches_declaration_order",
                    format!(
                        "port `{}` has an inconsistent lookup index",
                        port.name()
                    ),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HdlirPort, HdlirPortDirection, HdlirPortList, HdlirPortMode,
    };
    use crate::hdl::hdlir::ids::{HdlirPortId, HdlirValueId};
    use crate::hdl::hdlir::types::signal::SignalType;

    fn port_id(value: u128) -> HdlirPortId {
        HdlirPortId::new(value).expect("test port ID must be non-zero")
    }

    fn value_id(value: u128) -> HdlirValueId {
        HdlirValueId::new(value).expect("test value ID must be non-zero")
    }

    fn port(
        id: u128,
        name: &str,
        direction: HdlirPortDirection,
    ) -> HdlirPort {
        HdlirPort::new(
            port_id(id),
            name,
            direction,
            SignalType::Clock,
            Vec::new(),
            None,
        )
        .expect("valid test port")
    }

    #[test]
    fn constructs_a_logical_input_port() {
        let port = port(1, "data_in", HdlirPortDirection::Input);

        assert_eq!(port.id(), port_id(1));
        assert_eq!(port.name(), "data_in");
        assert_eq!(port.direction(), HdlirPortDirection::Input);
        assert!(port.default_value().is_none());
        assert!(port.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_or_whitespace_only_names() {
        for name in ["", " ", "\t\n"] {
            assert!(
                HdlirPort::new(
                    port_id(1),
                    name,
                    HdlirPortDirection::Input,
                    SignalType::Clock,
                    Vec::new(),
                    None,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn rejects_names_with_surrounding_whitespace() {
        assert!(
            HdlirPort::new(
                port_id(1),
                " data ",
                HdlirPortDirection::Input,
                SignalType::Clock,
                Vec::new(),
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn preserves_port_modes_in_declaration_order() {
        let port = HdlirPort::new(
            port_id(1),
            "data",
            HdlirPortDirection::Input,
            SignalType::Clock,
            vec![HdlirPortMode::Const, HdlirPortMode::Ref],
            None,
        )
        .expect("valid modes");

        assert_eq!(
            port.modes(),
            &[HdlirPortMode::Const, HdlirPortMode::Ref]
        );
    }

    #[test]
    fn rejects_repeated_port_modes() {
        assert!(
            HdlirPort::new(
                port_id(1),
                "data",
                HdlirPortDirection::Input,
                SignalType::Clock,
                vec![HdlirPortMode::Const, HdlirPortMode::Const],
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn retains_default_value_reference_without_evaluating_it() {
        let port = HdlirPort::new(
            port_id(1),
            "data",
            HdlirPortDirection::Input,
            SignalType::Clock,
            Vec::new(),
            Some(value_id(9)),
        )
        .expect("valid local declaration");

        assert_eq!(port.default_value(), Some(value_id(9)));
    }

    #[test]
    fn preserves_insertion_order_and_supports_lookup() {
        let mut ports = HdlirPortList::new();

        ports
            .insert(port(2, "second", HdlirPortDirection::Output))
            .expect("first insertion");
        ports
            .insert(port(1, "first", HdlirPortDirection::Input))
            .expect("second insertion");

        assert_eq!(ports.len(), 2);
        assert_eq!(
            ports.ids().collect::<Vec<_>>(),
            vec![port_id(2), port_id(1)]
        );
        assert_eq!(
            ports.get(port_id(1)).map(HdlirPort::name),
            Some("first")
        );
        assert_eq!(
            ports.get_by_name("second").map(HdlirPort::id),
            Some(port_id(2))
        );
        assert!(ports.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_port_ids_without_mutating_collection() {
        let mut ports = HdlirPortList::new();
        ports
            .insert(port(1, "first", HdlirPortDirection::Input))
            .expect("initial insertion");

        assert!(
            ports
                .insert(port(1, "second", HdlirPortDirection::Output))
                .is_err()
        );

        assert_eq!(ports.len(), 1);
        assert!(ports.get_by_name("second").is_none());
        assert!(ports.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_port_names_without_mutating_collection() {
        let mut ports = HdlirPortList::new();
        ports
            .insert(port(1, "data", HdlirPortDirection::Input))
            .expect("initial insertion");

        assert!(
            ports
                .insert(port(2, "data", HdlirPortDirection::Output))
                .is_err()
        );

        assert_eq!(ports.len(), 1);
        assert!(ports.get(port_id(2)).is_none());
        assert!(ports.validate_structure().is_ok());
    }

    #[test]
    fn names_are_compared_exactly() {
        let mut ports = HdlirPortList::new();

        ports
            .insert(port(1, "Data", HdlirPortDirection::Input))
            .expect("first insertion");
        ports
            .insert(port(2, "data", HdlirPortDirection::Output))
            .expect("second insertion");

        assert_eq!(ports.len(), 2);
    }

    #[test]
    fn can_construct_a_list_from_ordered_declarations() {
        let ports = HdlirPortList::from_ports([
            port(1, "clock", HdlirPortDirection::Input),
            port(2, "result", HdlirPortDirection::Output),
        ])
        .expect("valid list");

        assert_eq!(ports.len(), 2);
        assert!(ports.validate_structure().is_ok());
    }

    #[test]
    fn empty_collection_is_a_valid_construction_state() {
        let ports = HdlirPortList::new();

        assert!(ports.is_empty());
        assert_eq!(ports.len(), 0);
        assert!(ports.validate_structure().is_ok());
    }
}
