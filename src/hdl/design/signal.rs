
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Signal Declarations
//!
//! Path: `src/hdl/design/signal.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represents validated, target-independent logical HDL signal declarations.
//! Signals carry a canonical semantic identity, source-level name, semantic
//! value type, declaration kind, mutability, optional alias reference,
//! extensible qualifiers, and optional source provenance.
//!
//! This is a design-layer model, not an AST, symbol resolver, connectivity
//! graph, register model, physical wire, simulator value, or intermediate
//! representation.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/wires.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/hdl/registers.g4`, where present
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/module.rs`
//! - `src/hdl/design/port.rs`
//!
//! Grammar owns syntax. The canonical AST owns source nodes and their
//! identities. The shared semantic layer owns canonical HDL identities,
//! types, widths, directions, and source-location representations.
//! This module owns the design-layer signal contract and local validation.
//!
//! ## Owns
//!
//! - Logical signal declaration records.
//! - Ordinary and alias declaration distinctions.
//! - Mutability and extensible signal qualifiers.
//! - Validation of local declaration invariants.
//! - A per-scope signal table with deterministic identity/name lookup.
//! - Alias-reference and alias-cycle validation.
//!
//! ## Does not own
//!
//! - Parsing, lexing, or a replacement AST.
//! - A second semantic identity, type, width, or source-span system.
//! - General name resolution or expression evaluation.
//! - Module-port declarations or physical pin mappings.
//! - Wire/net connectivity, routing, or driver resolution.
//! - Register storage semantics or state-machine behavior.
//! - Clock synthesis, placement, synthesis, or timing closure.
//! - Simulation, runtime execution, or target allocation.
//! - HDLIR structures, builders, passes, lowering, or serialization.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on signal declarations, identifier
//! lengths, qualifier counts, or symbolic widths. Collections grow according
//! to available resources. Width expressions remain symbolic.
//!
//! Rust's address space, allocator, and representable values are finite.
//! POCO-REAF means this module adds no arbitrary universal hardware ceiling.
//!
//! ## Integration
//!
//! The canonical AST adapter constructs `HdlSignal` values using canonical
//! semantic IDs, types, widths, and locations. A containing module or design
//! owns an `HdlSignalTable` for its signal declarations. The table checks
//! duplicate identities and names within that scope and validates alias
//! references after all declarations have been collected.
//!
//! Assignment and process analysis consume signal identities and types.
//! Driver and connectivity analysis remain separate consumers. Port
//! declarations continue to use `design/port.rs`; register state remains
//! owned by the register/sequential model.
//!
//! This module intentionally does not depend on those consumers.
//!
//! ## Validation boundary
//!
//! Construction checks only local invariants. It does not prove that a named
//! type is resolvable, an expression is well typed, an alias is semantically
//! compatible with its target, or a signal has legal drivers.
//!
//! Alias-reference validation checks target existence and alias cycles.
//! Full type compatibility and scope-sensitive language rules remain with
//! semantic analysis.
//!
//! ## Freeze criteria
//!
//! - Existing semantic types are reused.
//! - Alias targets remain references, not copied declarations.
//! - Symbolic widths are not evaluated or truncated.
//! - Duplicate names are checked within the owning table.
//! - No physical or HDLIR dependency is introduced.
//! - Unit tests and workspace checks pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::hdl::{
    HdlId, HdlSourceLocation, HdlValueType, HdlWidth,
};

/// Schema version of the in-memory logical signal contract.
///
/// This is independent of the Zamani language, AST, semantic-model, and
/// HDLIR schema versions.
pub const HDL_SIGNAL_SCHEMA_VERSION: u16 = 1;

/// Kind of logical signal declaration.
///
/// This enum deliberately does not represent ports, physical nets, wires,
/// registers, or other declaration kinds owned by separate HDL components.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SignalDeclarationKind {
    /// A directly declared logical signal.
    Value,
    /// An alias referring to another logical signal declaration.
    Alias,
}

/// Source-level mutability of a logical signal.
///
/// This describes a language-level declaration property. It does not imply
/// physical storage, a hardware register, or a particular implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SignalMutability {
    /// Mutability is not explicitly specified by this design record.
    Unspecified,
    /// Assignments may update the signal where otherwise permitted.
    Mutable,
    /// The signal is immutable after its permitted initialization.
    Immutable,
}

/// A validated logical HDL signal declaration.
///
/// Canonical semantic types are reused directly. The optional alias target
/// refers to another declaration by identity and is not copied into this
/// value. Qualifier keys are extension-friendly strings; the set of
/// recognized qualifiers is not closed by this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlSignal {
    id: HdlId,
    name: String,
    value_type: HdlValueType,
    kind: SignalDeclarationKind,
    mutability: SignalMutability,
    alias_target: Option<HdlId>,
    qualifiers: BTreeMap<String, String>,
    location: Option<HdlSourceLocation>,
}

impl HdlSignal {
    /// Construct a signal declaration and validate its local invariants.
    ///
    /// The caller supplies a canonical semantic identity and type. Symbolic
    /// widths and named types are preserved without resolution.
    ///
    /// An alias must provide `alias_target`; an ordinary value declaration
    /// must not provide one.
    ///
    /// # Errors
    ///
    /// Returns an error if the name, type width, alias configuration,
    /// qualifier keys, or source location violates a local invariant.
    pub fn new(
        id: HdlId,
        name: impl Into<String>,
        value_type: HdlValueType,
        kind: SignalDeclarationKind,
        mutability: SignalMutability,
        alias_target: Option<HdlId>,
        qualifiers: BTreeMap<String, String>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, SignalError> {
        let signal = Self {
            id,
            name: name.into(),
            value_type,
            kind,
            mutability,
            alias_target,
            qualifiers,
            location,
        };

        signal.validate()?;
        Ok(signal)
    }

    /// Return the canonical semantic identity.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the source-level signal name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical HDL semantic value type.
    #[must_use]
    pub fn value_type(&self) -> &HdlValueType {
        &self.value_type
    }

    /// Return the declaration kind.
    #[must_use]
    pub const fn kind(&self) -> SignalDeclarationKind {
        self.kind
    }

    /// Return the declared mutability.
    #[must_use]
    pub const fn mutability(&self) -> SignalMutability {
        self.mutability
    }

    /// Return the optional target identity for an alias.
    ///
    /// This is a reference to a declaration, not proof that the declaration
    /// exists or that its type is compatible.
    #[must_use]
    pub fn alias_target(&self) -> Option<&HdlId> {
        self.alias_target.as_ref()
    }

    /// Return the complete qualifier map in deterministic key order.
    #[must_use]
    pub fn qualifiers(&self) -> &BTreeMap<String, String> {
        &self.qualifiers
    }

    /// Look up a qualifier by its exact key.
    #[must_use]
    pub fn qualifier(&self, key: &str) -> Option<&str> {
        self.qualifiers.get(key).map(String::as_str)
    }

    /// Return the optional source declaration location.
    #[must_use]
    pub fn location(&self) -> Option<&HdlSourceLocation> {
        self.location.as_ref()
    }

    /// Validate local invariants, including for values obtained from
    /// external adapters or future deserialization implementations.
    ///
    /// This method does not perform cross-declaration name resolution.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic order.
    pub fn validate(&self) -> Result<(), SignalError> {
        validate_name(&self.name)?;
        validate_value_type(&self.value_type)?;

        match (self.kind, self.alias_target.as_ref()) {
            (SignalDeclarationKind::Value, Some(_)) => {
                return Err(SignalError::UnexpectedAliasTarget);
            }
            (SignalDeclarationKind::Alias, None) => {
                return Err(SignalError::MissingAliasTarget);
            }
            (SignalDeclarationKind::Alias, Some(target)) if target == &self.id => {
                return Err(SignalError::SelfReferentialAlias(self.id.clone()));
            }
            _ => {}
        }

        for key in self.qualifiers.keys() {
            validate_qualifier_key(key)?;
        }

        if let Some(location) = &self.location {
            if location.source.trim().is_empty() {
                return Err(SignalError::EmptySourceIdentity);
            }

            if location.end < location.start {
                return Err(SignalError::InvalidSourceRange {
                    start: location.start,
                    end: location.end,
                });
            }
        }

        Ok(())
    }
}

/// A scope-local table of logical signal declarations.
///
/// This table owns neither the containing module nor global name resolution.
/// Names are unique within this table; the containing design determines
/// whether a table represents a module, package, or another legal scope.
///
/// Ordered maps make lookup results and iteration deterministic. Insertions
/// reject duplicate identities and duplicate names rather than silently
/// replacing declarations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlSignalTable {
    by_id: BTreeMap<HdlId, HdlSignal>,
    by_name: BTreeMap<String, HdlId>,
}

impl HdlSignalTable {
    /// Create an empty signal table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of declarations in this table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Return whether the table contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Look up a declaration by canonical semantic identity.
    #[must_use]
    pub fn get(&self, id: &HdlId) -> Option<&HdlSignal> {
        self.by_id.get(id)
    }

    /// Look up a declaration by its exact source-level name.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&HdlSignal> {
        self.by_name
            .get(name)
            .and_then(|id| self.by_id.get(id))
    }

    /// Iterate over declarations in deterministic identity order.
    pub fn iter(&self) -> impl Iterator<Item = &HdlSignal> {
        self.by_id.values()
    }

    /// Insert one declaration without replacing an existing declaration.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid declaration, duplicate identity, or
    /// duplicate name in this table.
    pub fn insert(&mut self, signal: HdlSignal) -> Result<(), SignalError> {
        signal.validate()?;

        if self.by_id.contains_key(signal.id()) {
            return Err(SignalError::DuplicateIdentity(signal.id().clone()));
        }

        if self.by_name.contains_key(signal.name()) {
            return Err(SignalError::DuplicateName(signal.name().to_owned()));
        }

        self.by_name
            .insert(signal.name().to_owned(), signal.id().clone());
        self.by_id.insert(signal.id().clone(), signal);

        Ok(())
    }

    /// Validate all alias references after declarations have been collected.
    ///
    /// This operation checks that every alias target exists and that alias
    /// chains do not form cycles. Forward references are supported because
    /// validation is performed against the complete table.
    ///
    /// Alias type compatibility is deliberately left to semantic analysis.
    ///
    /// # Errors
    ///
    /// Returns the first missing-target or cycle error in deterministic
    /// signal-identity order.
    pub fn validate_aliases(&self) -> Result<(), SignalError> {
        for signal in self.by_id.values() {
            if signal.kind() != SignalDeclarationKind::Alias {
                continue;
            }

            let mut current = signal;
            let mut visited = BTreeSet::new();

            loop {
                if !visited.insert(current.id().clone()) {
                    return Err(SignalError::AliasCycle(current.id().clone()));
                }

                let Some(target_id) = current.alias_target() else {
                    break;
                };

                let Some(target) = self.by_id.get(target_id) else {
                    return Err(SignalError::UnknownAliasTarget {
                        alias: current.id().clone(),
                        target: target_id.clone(),
                    });
                };

                if target.kind() != SignalDeclarationKind::Alias {
                    break;
                }

                current = target;
            }
        }

        Ok(())
    }

    /// Remove a declaration by identity.
    ///
    /// Removing a declaration does not automatically remove or rewrite
    /// aliases that refer to it. Call [`Self::validate_aliases`] after
    /// structural edits to detect dangling references.
    pub fn remove(&mut self, id: &HdlId) -> Option<HdlSignal> {
        let signal = self.by_id.remove(id)?;
        self.by_name.remove(signal.name());
        Some(signal)
    }

    /// Validate all local declaration invariants and alias relationships.
    ///
    /// # Errors
    ///
    /// Returns the first invalid declaration or alias relationship.
    pub fn validate(&self) -> Result<(), SignalError> {
        if self.by_id.len() != self.by_name.len() {
            return Err(SignalError::InconsistentTable);
        }

        for (id, signal) in &self.by_id {
            signal.validate()?;

            if id != signal.id() {
                return Err(SignalError::InconsistentTable);
            }

            if self.by_name.get(signal.name()) != Some(id) {
                return Err(SignalError::InconsistentTable);
            }
        }

        self.validate_aliases()
    }
}

/// Errors arising from logical signal construction or table validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignalError {
    /// The source-level signal name is empty or whitespace.
    EmptyName,
    /// The source-level signal name has leading or trailing whitespace.
    SurroundingWhitespaceInName,
    /// A concrete bit-vector width is zero.
    ZeroWidth,
    /// An alias declaration has no target.
    MissingAliasTarget,
    /// A non-alias declaration supplied an alias target.
    UnexpectedAliasTarget,
    /// An alias refers directly to itself.
    SelfReferentialAlias(HdlId),
    /// An alias target does not exist in the completed table.
    UnknownAliasTarget {
        /// Identity of the alias declaration.
        alias: HdlId,
        /// Identity of the missing target.
        target: HdlId,
    },
    /// An alias chain contains a cycle.
    AliasCycle(HdlId),
    /// A declaration identity already exists in the table.
    DuplicateIdentity(HdlId),
    /// A source-level name already exists in the table.
    DuplicateName(String),
    /// A qualifier key is empty, whitespace-padded, or contains a control
    /// character.
    InvalidQualifierKey(String),
    /// A source identity is empty.
    EmptySourceIdentity,
    /// A source range has an end preceding its start.
    InvalidSourceRange {
        /// Inclusive start offset.
        start: u64,
        /// Exclusive end offset.
        end: u64,
    },
    /// The table's identity and name indexes disagree.
    InconsistentTable,
}

impl fmt::Display for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("HDL signal name cannot be empty"),
            Self::SurroundingWhitespaceInName => {
                f.write_str("HDL signal name cannot have surrounding whitespace")
            }
            Self::ZeroWidth => f.write_str("HDL signal width cannot be zero"),
            Self::MissingAliasTarget => {
                f.write_str("HDL alias declaration requires a target")
            }
            Self::UnexpectedAliasTarget => {
                f.write_str("ordinary HDL signal cannot specify an alias target")
            }
            Self::SelfReferentialAlias(id) => {
                write!(f, "HDL signal `{id}` cannot alias itself")
            }
            Self::UnknownAliasTarget { alias, target } => {
                write!(f, "HDL alias `{alias}` references unknown signal `{target}`")
            }
            Self::AliasCycle(id) => {
                write!(f, "HDL alias chain contains a cycle at `{id}`")
            }
            Self::DuplicateIdentity(id) => {
                write!(f, "duplicate HDL signal identity `{id}`")
            }
            Self::DuplicateName(name) => {
                write!(f, "duplicate HDL signal name `{name}` in the same scope")
            }
            Self::InvalidQualifierKey(key) => {
                write!(f, "invalid HDL signal qualifier key `{key}`")
            }
            Self::EmptySourceIdentity => {
                f.write_str("HDL signal source identity cannot be empty")
            }
            Self::InvalidSourceRange { start, end } => {
                write!(f, "invalid HDL source range {start}..{end}")
            }
            Self::InconsistentTable => {
                f.write_str("HDL signal table indexes are inconsistent")
            }
        }
    }
}

impl std::error::Error for SignalError {}

fn validate_name(name: &str) -> Result<(), SignalError> {
    if name.trim().is_empty() {
        return Err(SignalError::EmptyName);
    }

    if name.trim() != name {
        return Err(SignalError::SurroundingWhitespaceInName);
    }

    Ok(())
}

fn validate_qualifier_key(key: &str) -> Result<(), SignalError> {
    if key.trim().is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
    {
        return Err(SignalError::InvalidQualifierKey(key.to_owned()));
    }

    Ok(())
}

/// Validate width invariants without recursively traversing the expression
/// tree. This avoids introducing a separate recursion-depth ceiling into
/// local validation. It does not evaluate symbolic expressions.
fn validate_value_type(value_type: &HdlValueType) -> Result<(), SignalError> {
    let HdlValueType::BitVector { width, .. } = value_type else {
        return Ok(());
    };

    let mut pending = vec![width];

    while let Some(current) = pending.pop() {
        match current {
            HdlWidth::Known(0) => return Err(SignalError::ZeroWidth),
            HdlWidth::Add(left, right) | HdlWidth::Multiply(left, right) => {
                pending.push(right);
                pending.push(left);
            }
            HdlWidth::Known(_)
            | HdlWidth::Symbol(_)
            | HdlWidth::Unresolved(_) => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::{
        HdlDirection, HdlSignedness,
    };

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be valid")
    }

    fn signal(
        identity: &str,
        name: &str,
        value_type: HdlValueType,
    ) -> HdlSignal {
        HdlSignal::new(
            id(identity),
            name,
            value_type,
            SignalDeclarationKind::Value,
            SignalMutability::Unspecified,
            None,
            BTreeMap::new(),
            None,
        )
        .expect("test signal must be valid")
    }

    fn bit_vector(width: HdlWidth) -> HdlValueType {
        HdlValueType::BitVector {
            width,
            signedness: HdlSignedness::Unsigned,
        }
    }

    #[test]
    fn preserves_canonical_identity_and_symbolic_width() {
        let width = HdlWidth::Symbol(id("DATA_WIDTH"));
        let value = signal("signal:data", "data", bit_vector(width.clone()));

        assert_eq!(value.id(), &id("signal:data"));
        assert_eq!(value.name(), "data");
        assert_eq!(
            value.value_type(),
            &bit_vector(HdlWidth::Symbol(id("DATA_WIDTH")))
        );
        assert_eq!(value.kind(), SignalDeclarationKind::Value);
        assert_eq!(value.validate(), Ok(()));
    }

    #[test]
    fn rejects_empty_and_whitespace_padded_names() {
        assert_eq!(
            HdlSignal::new(
                id("signal:empty"),
                "  ",
                HdlValueType::Bit,
                SignalDeclarationKind::Value,
                SignalMutability::Unspecified,
                None,
                BTreeMap::new(),
                None,
            )
            .unwrap_err(),
            SignalError::EmptyName
        );

        assert_eq!(
            HdlSignal::new(
                id("signal:padded"),
                " data ",
                HdlValueType::Bit,
                SignalDeclarationKind::Value,
                SignalMutability::Unspecified,
                None,
                BTreeMap::new(),
                None,
            )
            .unwrap_err(),
            SignalError::SurroundingWhitespaceInName
        );
    }

    #[test]
    fn rejects_zero_width_even_inside_symbolic_expression() {
        let width = HdlWidth::Add(
            Box::new(HdlWidth::Known(4)),
            Box::new(HdlWidth::Known(0)),
        );

        assert_eq!(
            HdlSignal::new(
                id("signal:bad-width"),
                "bad_width",
                bit_vector(width),
                SignalDeclarationKind::Value,
                SignalMutability::Unspecified,
                None,
                BTreeMap::new(),
                None,
            )
            .unwrap_err(),
            SignalError::ZeroWidth
        );
    }

    #[test]
    fn preserves_open_ended_qualifiers() {
        let mut qualifiers = BTreeMap::new();
        qualifiers.insert(
            "vendor.example:retention".to_owned(),
            "required".to_owned(),
        );
        qualifiers.insert(
            "zamani:domain".to_owned(),
            "control".to_owned(),
        );

        let value = HdlSignal::new(
            id("signal:control"),
            "control",
            HdlValueType::Bit,
            SignalDeclarationKind::Value,
            SignalMutability::Mutable,
            None,
            qualifiers,
            None,
        )
        .expect("qualified signal must be valid");

        assert_eq!(
            value.qualifier("vendor.example:retention"),
            Some("required")
        );
        assert_eq!(value.qualifiers().len(), 2);
    }

    #[test]
    fn rejects_duplicate_names_and_identities_without_replacement() {
        let mut table = HdlSignalTable::new();

        table
            .insert(signal("signal:a", "a", HdlValueType::Bit))
            .expect("first signal must insert");

        assert_eq!(
            table.insert(signal("signal:a", "b", HdlValueType::Bit)),
            Err(SignalError::DuplicateIdentity(id("signal:a")))
        );

        assert_eq!(
            table.insert(signal("signal:b", "a", HdlValueType::Bit)),
            Err(SignalError::DuplicateName("a".to_owned()))
        );

        assert_eq!(table.len(), 1);
        assert!(table.get_by_name("a").is_some());
        assert!(table.get_by_name("b").is_none());
    }

    #[test]
    fn accepts_forward_alias_reference_after_all_declarations_are_added() {
        let mut table = HdlSignalTable::new();

        let alias = HdlSignal::new(
            id("signal:alias"),
            "alias_data",
            bit_vector(HdlWidth::Symbol(id("DATA_WIDTH"))),
            SignalDeclarationKind::Alias,
            SignalMutability::Unspecified,
            Some(id("signal:target")),
            BTreeMap::new(),
            None,
        )
        .expect("alias declaration must be locally valid");

        table
            .insert(alias)
            .expect("alias may be inserted before its target");

        assert!(matches!(
            table.validate_aliases(),
            Err(SignalError::UnknownAliasTarget { .. })
        ));

        table
            .insert(signal(
                "signal:target",
                "target_data",
                bit_vector(HdlWidth::Symbol(id("DATA_WIDTH"))),
            ))
            .expect("target must insert");

        assert_eq!(table.validate_aliases(), Ok(()));
        assert_eq!(table.validate(), Ok(()));
    }

    #[test]
    fn detects_alias_cycles() {
        let mut table = HdlSignalTable::new();

        for (identity, name, target) in [
            ("signal:a", "a", "signal:b"),
            ("signal:b", "b", "signal:a"),
        ] {
            let alias = HdlSignal::new(
                id(identity),
                name,
                HdlValueType::Bit,
                SignalDeclarationKind::Alias,
                SignalMutability::Unspecified,
                Some(id(target)),
                BTreeMap::new(),
                None,
            )
            .expect("alias must be locally valid");

            table.insert(alias).expect("identity must be unique");
        }

        assert!(matches!(
            table.validate_aliases(),
            Err(SignalError::AliasCycle(_))
        ));
    }

    #[test]
    fn lookup_is_deterministic_and_remove_keeps_indexes_consistent() {
        let mut table = HdlSignalTable::new();

        table
            .insert(signal("signal:z", "z", HdlValueType::Bit))
            .expect("signal must insert");
        table
            .insert(signal("signal:a", "a", HdlValueType::Bit))
            .expect("signal must insert");

        let identities: Vec<_> = table.iter().map(|item| item.id().as_str()).collect();
        assert_eq!(identities, vec!["signal:a", "signal:z"]);

        assert!(table.remove(&id("signal:a")).is_some());
        assert!(table.get_by_name("a").is_none());
        assert_eq!(table.validate(), Ok(()));
    }

    #[test]
    fn clock_and_reset_types_are_preserved_without_physical_assumptions() {
        let clock = signal("signal:clock", "clock", HdlValueType::Clock);
        let reset = signal("signal:reset", "reset", HdlValueType::Reset);

        assert_eq!(clock.value_type(), &HdlValueType::Clock);
        assert_eq!(reset.value_type(), &HdlValueType::Reset);
        assert_eq!(clock.validate(), Ok(()));
        assert_eq!(reset.validate(), Ok(()));
    }

    #[test]
    fn imports_existing_direction_contract_without_owning_ports() {
        // A signal is not a module port. The canonical direction type remains
        // available to consumers through the semantic module.
        let _direction = HdlDirection::Internal;
        let _signedness = HdlSignedness::Signed;
    }
}
