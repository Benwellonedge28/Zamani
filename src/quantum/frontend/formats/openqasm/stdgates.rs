//! OpenQASM 3 standard-library gate catalogue.
//!
//! This module is the single source of truth for the OpenQASM 3
//! `stdgates.inc` namespace as understood by the Zamani frontend.
//!
//! # Architectural boundary
//!
//! ```text
//! OpenQASM source
//!       │
//!       ▼
//! lexer / parser
//!       │
//!       ▼
//! OpenQASM AST
//!       │
//!       ▼
//! validation
//!       │
//!       ├── lookup(name)
//!       │
//!       ├── availability(version)
//!       │
//!       └── lowering capability
//!       │
//!       ▼
//! generic lowering
//!       │
//!       ▼
//! canonical quantum::ir::GateKind
//! ```
//!
//! This module owns only the OpenQASM standard-library catalogue.
//!
//! It deliberately does NOT:
//!
//! - parse OpenQASM;
//! - resolve arbitrary include files;
//! - access the filesystem;
//! - access the network;
//! - execute gates;
//! - construct `Gate` values;
//! - construct `QuantumCircuit` values;
//! - perform optimization;
//! - perform decomposition;
//! - perform routing;
//! - perform scheduling;
//! - communicate with hardware;
//! - silently discard unsupported operations;
//! - define another quantum semantic model.
//!
//! # Standard-library boundary
//!
//! OpenQASM 3 has a standard include file named:
//!
//! ```text
//! include "stdgates.inc";
//! ```
//!
//! The standard library is versioned with OpenQASM itself. Therefore this
//! catalogue must never claim that an entry is available merely because a
//! future language version happens to have a greater numeric version.
//!
//! The current production baseline is OpenQASM 3.0 and 3.1.
//!
//! # Important distinction
//!
//! A gate can be:
//!
//! 1. a real OpenQASM standard-library gate;
//! 2. directly representable by the current Zamani Quantum IR;
//! 3. syntactically/semantically valid but unsupported by the current IR.
//!
//! These are deliberately separate concepts.
//!
//! For example, `sx`, `cp`, and `cu` are real OpenQASM standard-library
//! gates, but the current Zamani IR does not have corresponding canonical
//! `GateKind` variants. They therefore remain visible in this catalogue and
//! return `StandardGateLowering::Unsupported`.
//!
//! They must NOT be silently decomposed into other gates here.
//!
//! # OpenQASM built-ins are intentionally excluded
//!
//! `U` and `gphase` are OpenQASM language built-ins rather than entries of
//! `stdgates.inc`. They therefore do not belong in [`STANDARD_GATES`].
//!
//! The OpenQASM validator already treats these as language-level constructs.
//!
//! # Compatibility aliases
//!
//! The OpenQASM 3 standard library also carries several OpenQASM 2
//! compatibility aliases:
//!
//! - `CX`
//! - `phase`
//! - `cphase`
//! - `id`
//! - `u1`
//! - `u2`
//! - `u3`
//!
//! These are included exactly as standard-library entries and are mapped to
//! their canonical semantic identity where appropriate.
//!
//! # Version policy
//!
//! Every entry currently has an introduction version of OpenQASM 3.0 because
//! the current standard library entries in this catalogue were introduced in
//! OpenQASM 3.0.
//!
//! `available_in()` deliberately rejects:
//!
//! - OpenQASM 2.x;
//! - unknown future OpenQASM 3.x versions;
//! - OpenQASM 4.x and later.
//!
//! This prevents the dangerous assumption that a future language version has
//! exactly the same standard-library namespace.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! Rust 2021.
//! No nightly features.
//! No additional dependencies.
//!
//! # Specification
//!
//! The catalogue follows the OpenQASM 3 standard-library specification:
//!
//! <https://openqasm.com/language/standard_library.html>
//!
//! The OpenQASM specification defines `stdgates.inc` as the standard library
//! and requires implementations to make available exactly the gates belonging
//! to the selected language version.
//!
//! # Integration contract
//!
//! `validation.rs` may use:
//!
//! - [`lookup`]
//! - [`lookup_canonical`]
//! - [`is_available`]
//! - [`gate_kind`]
//! - [`StandardGate::qubit_count`]
//! - [`StandardGate::parameter_count`]
//! - [`StandardGate::lowering`]
//!
//! `importer.rs` may use:
//!
//! - [`lookup`]
//! - [`lookup_canonical`]
//! - [`gate_kind`]
//!
//! Generic frontend code must not depend on this module.
//!
//! Future format frontends must not depend on this module.
//!
//! This file is therefore independently completable and frozen as the
//! OpenQASM standard-library contract.

use crate::quantum::ir::GateKind;

/// Canonical OpenQASM standard-library include name.
pub const STANDARD_LIBRARY_INCLUDE: &str = "stdgates.inc";

/// OpenQASM major version implemented by this catalogue.
pub const OPENQASM_MAJOR_VERSION: u16 = 3;

/// First supported OpenQASM minor version.
pub const OPENQASM_MIN_MINOR_VERSION: u16 = 0;

/// Last OpenQASM minor version for which this catalogue has an explicit
/// standard-library definition.
pub const OPENQASM_MAX_MINOR_VERSION: u16 = 1;

/// Minimum language version represented by this catalogue.
pub const STANDARD_LIBRARY_MIN_MAJOR: u16 = 3;

/// Minimum language minor version represented by this catalogue.
pub const STANDARD_LIBRARY_MIN_MINOR: u16 = 0;

/// Maximum language version represented by this catalogue.
pub const STANDARD_LIBRARY_MAX_MAJOR: u16 = 3;

/// Maximum language minor version represented by this catalogue.
pub const STANDARD_LIBRARY_MAX_MINOR: u16 = 1;

/// Describes how an OpenQASM standard-library gate maps into the canonical
/// Zamani Quantum IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardGateLowering {
    /// The OpenQASM operation has a directly corresponding canonical IR gate.
    ///
    /// This does not imply that the importer may ignore other semantic
    /// properties such as parameters, operands, modifiers, or global phase.
    Direct(GateKind),

    /// The operation is part of the OpenQASM standard library but the current
    /// canonical IR has no semantically equivalent operation.
    ///
    /// The reason is static and machine-readable. Source spans and diagnostic
    /// codes belong to the frontend diagnostic layer.
    Unsupported(&'static str),
}

impl StandardGateLowering {
    /// Returns the canonical IR gate when the operation is directly
    /// representable.
    #[must_use]
    pub const fn gate_kind(self) -> Option<GateKind> {
        match self {
            Self::Direct(kind) => Some(kind),
            Self::Unsupported(_) => None,
        }
    }

    /// Returns whether the operation is directly representable.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        matches!(self, Self::Direct(_))
    }

    /// Returns the static reason why lowering is unsupported.
    #[must_use]
    pub const fn unsupported_reason(self) -> Option<&'static str> {
        match self {
            Self::Direct(_) => None,
            Self::Unsupported(reason) => Some(reason),
        }
    }
}

/// Immutable description of one OpenQASM standard-library gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardGate {
    /// Exact OpenQASM source spelling.
    ///
    /// OpenQASM identifiers are case-sensitive.
    name: &'static str,

    /// Canonical standard-library spelling.
    ///
    /// An alias points to its canonical gate name.
    canonical_name: &'static str,

    /// Number of quantum operands.
    qubit_count: usize,

    /// Number of source-level parameters.
    parameter_count: usize,

    /// First OpenQASM major version containing this entry.
    introduced_major: u16,

    /// First OpenQASM minor version containing this entry.
    introduced_minor: u16,

    /// Canonical IR lowering capability.
    lowering: StandardGateLowering,
}

impl StandardGate {
    /// Constructs immutable standard-gate metadata.
    const fn new(
        name: &'static str,
        canonical_name: &'static str,
        qubit_count: usize,
        parameter_count: usize,
        introduced_major: u16,
        introduced_minor: u16,
        lowering: StandardGateLowering,
    ) -> Self {
        Self {
            name,
            canonical_name,
            qubit_count,
            parameter_count,
            introduced_major,
            introduced_minor,
            lowering,
        }
    }

    /// Exact OpenQASM spelling.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Canonical standard-library spelling.
    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        self.canonical_name
    }

    /// Required quantum operand count.
    #[must_use]
    pub const fn qubit_count(self) -> usize {
        self.qubit_count
    }

    /// Required source-level parameter count.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameter_count
    }

    /// First OpenQASM major version containing the gate.
    #[must_use]
    pub const fn introduced_major(self) -> u16 {
        self.introduced_major
    }

    /// First OpenQASM minor version containing the gate.
    #[must_use]
    pub const fn introduced_minor(self) -> u16 {
        self.introduced_minor
    }

    /// Returns the lowering capability.
    #[must_use]
    pub const fn lowering(self) -> StandardGateLowering {
        self.lowering
    }

    /// Returns the direct canonical IR representation, if one exists.
    #[must_use]
    pub const fn gate_kind(self) -> Option<GateKind> {
        self.lowering.gate_kind()
    }

    /// Returns whether this gate can currently be represented directly by
    /// the canonical Quantum IR.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.lowering.is_supported()
    }

    /// Returns whether this entry is a compatibility alias.
    #[must_use]
    pub const fn is_alias(self) -> bool {
        !const_str_eq(self.name, self.canonical_name)
    }

    /// Returns whether this entry is available in the explicitly supported
    /// OpenQASM language version.
    ///
    /// Future language versions are rejected until their standard-library
    /// contents have been explicitly audited and added to this catalogue.
    #[must_use]
    pub const fn available_in(self, major: u16, minor: u16) -> bool {
        if major != OPENQASM_MAJOR_VERSION {
            return false;
        }

        if minor < self.introduced_minor {
            return false;
        }

        minor <= OPENQASM_MAX_MINOR_VERSION
    }
}

/// Complete OpenQASM 3.0/3.1 `stdgates.inc` catalogue.
///
/// The entries correspond to the standard-library namespace, not to every
/// quantum operation understood by OpenQASM itself.
///
/// In particular, the following are deliberately NOT present here:
///
/// - `U` — language built-in;
/// - `gphase` — language built-in;
/// - `measure` — language operation;
/// - `reset` — language operation;
/// - `barrier` — language operation;
/// - `delay` — language operation.
///
/// Those constructs belong to the AST/semantic layer and are handled by the
/// appropriate OpenQASM frontend modules.
///
/// The catalogue is intentionally kept as a static table:
///
/// - no runtime initialization;
/// - no global mutable state;
/// - deterministic lookup;
/// - no allocation for normal lookup;
/// - no filesystem dependency;
/// - no network dependency.
pub const STANDARD_GATES: &[StandardGate] = &[
    // =====================================================================
    // Single-qubit standard gates.
    // =====================================================================

    StandardGate::new(
        "p",
        "p",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Phase),
    ),
    StandardGate::new(
        "x",
        "x",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::X),
    ),
    StandardGate::new(
        "y",
        "y",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Y),
    ),
    StandardGate::new(
        "z",
        "z",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Z),
    ),
    StandardGate::new(
        "h",
        "h",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::H),
    ),
    StandardGate::new(
        "s",
        "s",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::S),
    ),
    StandardGate::new(
        "sdg",
        "sdg",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Sdg),
    ),
    StandardGate::new(
        "t",
        "t",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::T),
    ),
    StandardGate::new(
        "tdg",
        "tdg",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Tdg),
    ),
    StandardGate::new(
        "sx",
        "sx",
        1,
        0,
        3,
        0,
        StandardGateLowering::Unsupported(
            "OpenQASM sx is not directly represented by the canonical Quantum IR",
        ),
    ),
    StandardGate::new(
        "rx",
        "rx",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::RX),
    ),
    StandardGate::new(
        "ry",
        "ry",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::RY),
    ),
    StandardGate::new(
        "rz",
        "rz",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::RZ),
    ),

    // =====================================================================
    // Two-qubit standard gates.
    // =====================================================================

    StandardGate::new(
        "cx",
        "cx",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CX),
    ),
    StandardGate::new(
        "cy",
        "cy",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CY),
    ),
    StandardGate::new(
        "cz",
        "cz",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CZ),
    ),
    StandardGate::new(
        "cp",
        "cp",
        2,
        1,
        3,
        0,
        StandardGateLowering::Unsupported(
            "OpenQASM cp is not directly represented by the canonical Quantum IR",
        ),
    ),
    StandardGate::new(
        "crx",
        "crx",
        2,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CRX),
    ),
    StandardGate::new(
        "cry",
        "cry",
        2,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CRY),
    ),
    StandardGate::new(
        "crz",
        "crz",
        2,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CRZ),
    ),
    StandardGate::new(
        "ch",
        "ch",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CH),
    ),
    StandardGate::new(
        "cu",
        "cu",
        2,
        4,
        3,
        0,
        StandardGateLowering::Unsupported(
            "OpenQASM cu is not directly represented by the canonical Quantum IR",
        ),
    ),
    StandardGate::new(
        "swap",
        "swap",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::SWAP),
    ),

    // =====================================================================
    // Three-qubit standard gates.
    // =====================================================================

    StandardGate::new(
        "ccx",
        "ccx",
        3,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CCX),
    ),
    StandardGate::new(
        "cswap",
        "cswap",
        3,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CSWAP),
    ),

    // =====================================================================
    // OpenQASM 2 compatibility aliases supplied by stdgates.inc.
    // =====================================================================

    StandardGate::new(
        "CX",
        "cx",
        2,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::CX),
    ),
    StandardGate::new(
        "phase",
        "p",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::Phase),
    ),
    StandardGate::new(
        "cphase",
        "cp",
        2,
        1,
        3,
        0,
        StandardGateLowering::Unsupported(
            "OpenQASM cphase aliases cp, which is not directly represented by the canonical Quantum IR",
        ),
    ),
    StandardGate::new(
        "id",
        "id",
        1,
        0,
        3,
        0,
        StandardGateLowering::Direct(GateKind::I),
    ),
    StandardGate::new(
        "u1",
        "u1",
        1,
        1,
        3,
        0,
        StandardGateLowering::Direct(GateKind::U1),
    ),
    StandardGate::new(
        "u2",
        "u2",
        1,
        2,
        3,
        0,
        StandardGateLowering::Direct(GateKind::U2),
    ),
    StandardGate::new(
        "u3",
        "u3",
        1,
        3,
        3,
        0,
        StandardGateLowering::Direct(GateKind::U3),
    ),
];

/// Number of entries in [`STANDARD_GATES`].
pub const STANDARD_GATE_COUNT: usize = STANDARD_GATES.len();

/// Returns the complete immutable standard-library catalogue.
///
/// The returned slice has deterministic ordering and requires no allocation.
#[must_use]
pub const fn all() -> &'static [StandardGate] {
    STANDARD_GATES
}

/// Looks up an exact, case-sensitive OpenQASM standard-library name.
///
/// OpenQASM identifiers are case-sensitive:
///
/// ```text
/// cx  != CX
/// ```
#[must_use]
pub fn lookup(name: &str) -> Option<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .find(|gate| gate.name() == name)
}

/// Looks up the canonical standard-library entry corresponding to a source
/// spelling.
///
/// Examples:
///
/// ```text
/// CX      -> cx
/// phase   -> p
/// cphase  -> cp
/// ```
#[must_use]
pub fn lookup_canonical(name: &str) -> Option<StandardGate> {
    let entry = lookup(name)?;

    STANDARD_GATES
        .iter()
        .copied()
        .find(|gate| gate.name() == entry.canonical_name())
}

/// Returns all currently directly representable standard-library entries.
///
/// The returned vector is deterministic and follows the catalogue order.
#[must_use]
pub fn supported() -> Vec<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .filter(|gate| gate.is_supported())
        .collect()
}

/// Returns all standard-library entries that are currently unsupported by
/// the canonical Quantum IR.
///
/// Unsupported entries remain visible so that validation/import can produce
/// an explicit capability error instead of an unknown-gate error.
#[must_use]
pub fn unsupported() -> Vec<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .filter(|gate| !gate.is_supported())
        .collect()
}

/// Returns whether the named entry belongs to `stdgates.inc` for the supplied
/// explicitly supported OpenQASM language version.
///
/// Unknown future versions return `false`.
#[must_use]
pub fn is_available(name: &str, major: u16, minor: u16) -> bool {
    lookup(name).is_some_and(|gate| gate.available_in(major, minor))
}

/// Returns the direct canonical IR gate kind for a source spelling.
///
/// `None` means either:
///
/// - the name is not a standard-library gate; or
/// - the name is a standard-library gate but has no direct canonical IR
///   representation.
///
/// Use [`lookup`] first when the caller must distinguish those cases.
#[must_use]
pub fn gate_kind(name: &str) -> Option<GateKind> {
    lookup(name).and_then(StandardGate::gate_kind)
}

/// Returns the canonical name for a standard-library source spelling.
///
/// This is useful when diagnostics, symbol tables, or lowering need to
/// preserve the distinction between the source spelling and semantic
/// identity.
#[must_use]
pub fn canonical_name(name: &str) -> Option<&'static str> {
    lookup(name).map(StandardGate::canonical_name)
}

/// Returns whether a name is a standard-library compatibility alias.
#[must_use]
pub fn is_alias(name: &str) -> bool {
    lookup(name).is_some_and(StandardGate::is_alias)
}

/// Returns whether the supplied version is explicitly supported by this
/// standard-library catalogue.
#[must_use]
pub const fn is_supported_language_version(major: u16, minor: u16) -> bool {
    major == OPENQASM_MAJOR_VERSION
        && minor >= OPENQASM_MIN_MINOR_VERSION
        && minor <= OPENQASM_MAX_MINOR_VERSION
}

/// Performs a deterministic internal consistency check over the catalogue.
///
/// This is intentionally public so integration tests and future build/test
/// tooling can validate the static catalogue without exposing its internal
/// representation.
///
/// Returns `true` only when:
///
/// - every name is unique;
/// - every canonical name resolves to a canonical entry;
/// - every canonical entry points to itself;
/// - every gate has a valid version;
/// - directly lowered entries have a corresponding IR kind;
/// - unsupported entries carry a reason;
/// - aliases have a different source spelling from their canonical spelling.
#[must_use]
pub fn catalogue_is_consistent() -> bool {
    for gate in STANDARD_GATES {
        if gate.name().is_empty() || gate.canonical_name().is_empty() {
            return false;
        }

        if gate.introduced_major() != OPENQASM_MAJOR_VERSION {
            return false;
        }

        if gate.introduced_minor() > OPENQASM_MAX_MINOR_VERSION {
            return false;
        }

        if gate.qubit_count() == 0 {
            return false;
        }

        if gate.is_supported() {
            if gate.gate_kind().is_none() {
                return false;
            }
        } else if gate.lowering().unsupported_reason().is_none() {
            return false;
        }

        let canonical = match lookup(gate.canonical_name()) {
            Some(value) => value,
            None => return false,
        };

        if canonical.canonical_name() != gate.canonical_name() {
            return false;
        }
    }

    for (index, left) in STANDARD_GATES.iter().enumerate() {
        for right in STANDARD_GATES.iter().skip(index + 1) {
            if left.name() == right.name() {
                return false;
            }
        }
    }

    true
}

/// Tiny `const` string equality helper.
///
/// The standard library catalogue uses only ASCII identifiers, so comparing
/// bytes is sufficient and avoids introducing an external dependency.
const fn const_str_eq(left: &str, right: &str) -> bool {
    let left_bytes = left.as_bytes();
    let right_bytes = right.as_bytes();

    if left_bytes.len() != right_bytes.len() {
        return false;
    }

    let mut index = 0;

    while index < left_bytes.len() {
        if left_bytes[index] != right_bytes[index] {
            return false;
        }

        index += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_name_is_exact() {
        assert_eq!(STANDARD_LIBRARY_INCLUDE, "stdgates.inc");
    }

    #[test]
    fn supported_language_versions_are_explicit() {
        assert!(is_supported_language_version(3, 0));
        assert!(is_supported_language_version(3, 1));

        assert!(!is_supported_language_version(2, 0));
        assert!(!is_supported_language_version(3, 2));
        assert!(!is_supported_language_version(4, 0));
    }

    #[test]
    fn catalogue_is_non_empty() {
        assert!(!STANDARD_GATES.is_empty());
        assert_eq!(STANDARD_GATES.len(), STANDARD_GATE_COUNT);
    }

    #[test]
    fn catalogue_is_consistent() {
        assert!(catalogue_is_consistent());
    }

    #[test]
    fn names_are_unique() {
        for (index, left) in STANDARD_GATES.iter().enumerate() {
            for right in STANDARD_GATES.iter().skip(index + 1) {
                assert_ne!(
                    left.name(),
                    right.name(),
                    "duplicate standard-library name: {}",
                    left.name()
                );
            }
        }
    }

    #[test]
    fn identifiers_are_case_sensitive() {
        assert!(lookup("cx").is_some());
        assert!(lookup("CX").is_some());

        assert!(lookup("Cx").is_none());
        assert!(lookup("cX").is_none());
    }

    #[test]
    fn official_standard_library_members_are_present() {
        let expected = [
            "p",
            "x",
            "y",
            "z",
            "h",
            "s",
            "sdg",
            "t",
            "tdg",
            "sx",
            "rx",
            "ry",
            "rz",
            "cx",
            "cy",
            "cz",
            "cp",
            "crx",
            "cry",
            "crz",
            "ch",
            "cu",
            "swap",
            "ccx",
            "cswap",
            "CX",
            "phase",
            "cphase",
            "id",
            "u1",
            "u2",
            "u3",
        ];

        for name in expected {
            assert!(
                lookup(name).is_some(),
                "missing OpenQASM standard-library entry: {name}"
            );
        }
    }

    #[test]
    fn non_library_builtins_are_not_catalogued() {
        // U and gphase are OpenQASM language built-ins, not stdgates.inc
        // entries.
        assert!(lookup("U").is_none());
        assert!(lookup("u").is_none());
        assert!(lookup("gphase").is_none());

        // These are language operations rather than stdgates.inc gates.
        assert!(lookup("measure").is_none());
        assert!(lookup("reset").is_none());
        assert!(lookup("barrier").is_none());
        assert!(lookup("delay").is_none());
    }

    #[test]
    fn standard_gate_arities_are_correct() {
        let expected = [
            ("p", 1, 1),
            ("x", 1, 0),
            ("y", 1, 0),
            ("z", 1, 0),
            ("h", 1, 0),
            ("s", 1, 0),
            ("sdg", 1, 0),
            ("t", 1, 0),
            ("tdg", 1, 0),
            ("sx", 1, 0),
            ("rx", 1, 1),
            ("ry", 1, 1),
            ("rz", 1, 1),
            ("cx", 2, 0),
            ("cy", 2, 0),
            ("cz", 2, 0),
            ("cp", 2, 1),
            ("crx", 2, 1),
            ("cry", 2, 1),
            ("crz", 2, 1),
            ("ch", 2, 0),
            ("cu", 2, 4),
            ("swap", 2, 0),
            ("ccx", 3, 0),
            ("cswap", 3, 0),
            ("CX", 2, 0),
            ("phase", 1, 1),
            ("cphase", 2, 1),
            ("id", 1, 0),
            ("u1", 1, 1),
            ("u2", 1, 2),
            ("u3", 1, 3),
        ];

        for (name, qubits, parameters) in expected {
            let gate = lookup(name)
                .unwrap_or_else(|| panic!("missing standard gate: {name}"));

            assert_eq!(
                gate.qubit_count(),
                qubits,
                "wrong qubit count for {name}"
            );

            assert_eq!(
                gate.parameter_count(),
                parameters,
                "wrong parameter count for {name}"
            );
        }
    }

    #[test]
    fn canonical_aliases_resolve_correctly() {
        let aliases = [
            ("CX", "cx"),
            ("phase", "p"),
            ("cphase", "cp"),
        ];

        for (source, canonical) in aliases {
            let gate = lookup(source).expect("alias must exist");

            assert_eq!(gate.canonical_name(), canonical);
            assert!(gate.is_alias());

            let canonical_gate =
                lookup_canonical(source).expect("canonical gate must exist");

            assert_eq!(canonical_gate.name(), canonical);
            assert_eq!(canonical_gate.canonical_name(), canonical);
        }
    }

    #[test]
    fn canonical_entries_are_not_aliases() {
        for gate in STANDARD_GATES {
            if gate.name() == gate.canonical_name() {
                assert!(!gate.is_alias());
            }
        }
    }

    #[test]
    fn direct_ir_lowerings_match_existing_gate_kinds() {
        let expected = [
            ("id", GateKind::I),
            ("x", GateKind::X),
            ("y", GateKind::Y),
            ("z", GateKind::Z),
            ("h", GateKind::H),
            ("s", GateKind::S),
            ("sdg", GateKind::Sdg),
            ("t", GateKind::T),
            ("tdg", GateKind::Tdg),
            ("rx", GateKind::RX),
            ("ry", GateKind::RY),
            ("rz", GateKind::RZ),
            ("p", GateKind::Phase),
            ("cx", GateKind::CX),
            ("cy", GateKind::CY),
            ("cz", GateKind::CZ),
            ("crx", GateKind::CRX),
            ("cry", GateKind::CRY),
            ("crz", GateKind::CRZ),
            ("ch", GateKind::CH),
            ("swap", GateKind::SWAP),
            ("ccx", GateKind::CCX),
            ("cswap", GateKind::CSWAP),
            ("u1", GateKind::U1),
            ("u2", GateKind::U2),
            ("u3", GateKind::U3),
        ];

        for (name, expected_kind) in expected {
            assert_eq!(
                gate_kind(name),
                Some(expected_kind),
                "incorrect canonical IR lowering for {name}"
            );
        }
    }

    #[test]
    fn unsupported_standard_gates_are_explicit() {
        for name in ["sx", "cp", "cu"] {
            let gate =
                lookup(name).expect("standard-library gate must exist");

            assert!(
                !gate.is_supported(),
                "{name} must not be silently lowered"
            );

            assert!(
                gate.lowering().unsupported_reason().is_some(),
                "{name} must have an explicit unsupported reason"
            );

            assert!(
                gate_kind(name).is_none(),
                "{name} must not produce a fake GateKind"
            );
        }
    }

    #[test]
    fn cphase_is_an_explicit_unsupported_alias() {
        let gate = lookup("cphase").expect("cphase must exist");

        assert_eq!(gate.canonical_name(), "cp");
        assert!(gate.is_alias());
        assert!(!gate.is_supported());
        assert!(gate.lowering().unsupported_reason().is_some());
    }

    #[test]
    fn standard_gates_are_available_in_openqasm_3_0_and_3_1() {
        for gate in STANDARD_GATES {
            assert!(
                gate.available_in(3, 0),
                "{} must be available in OpenQASM 3.0",
                gate.name()
            );

            assert!(
                gate.available_in(3, 1),
                "{} must be available in OpenQASM 3.1",
                gate.name()
            );
        }
    }

    #[test]
    fn unsupported_future_versions_are_rejected() {
        assert!(!is_available("x", 3, 2));
        assert!(!is_available("x", 4, 0));
        assert!(!is_available("x", 99, 99));
    }

    #[test]
    fn openqasm_2_is_not_claimed_as_supported() {
        assert!(!is_available("x", 2, 0));
        assert!(!is_available("CX", 2, 0));
        assert!(!is_available("u3", 2, 0));
    }

    #[test]
    fn unknown_names_are_not_standard_gates() {
        for name in [
            "",
            "cnot",
            "CNOT",
            "foo",
            "custom_gate",
            "sxdag",
            "measure",
            "reset",
            "gphase",
            "U",
        ] {
            assert!(
                lookup(name).is_none(),
                "{name:?} must not be a standard-library entry"
            );
        }
    }

    #[test]
    fn supported_and_unsupported_partitions_cover_catalogue() {
        let supported_count = STANDARD_GATES
            .iter()
            .filter(|gate| gate.is_supported())
            .count();

        let unsupported_count = STANDARD_GATES
            .iter()
            .filter(|gate| !gate.is_supported())
            .count();

        assert_eq!(
            supported_count + unsupported_count,
            STANDARD_GATES.len()
        );
    }

    #[test]
    fn every_canonical_name_resolves_to_itself() {
        for gate in STANDARD_GATES {
            let canonical = lookup(gate.canonical_name())
                .unwrap_or_else(|| {
                    panic!(
                        "canonical name {} has no catalogue entry",
                        gate.canonical_name()
                    )
                });

            assert_eq!(
                canonical.canonical_name(),
                gate.canonical_name(),
                "canonical chain must terminate at itself"
            );
        }
    }

    #[test]
    fn aliases_and_canonical_entries_share_semantic_lowering_when_supported() {
        for gate in STANDARD_GATES {
            if !gate.is_alias() {
                continue;
            }

            let canonical = lookup_canonical(gate.name())
                .expect("alias must resolve to canonical entry");

            if gate.is_supported() {
                assert_eq!(
                    gate.gate_kind(),
                    canonical.gate_kind(),
                    "alias {} must lower like {}",
                    gate.name(),
                    canonical.name()
                );
            }
        }
    }
}