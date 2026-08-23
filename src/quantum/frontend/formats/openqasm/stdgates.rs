//! OpenQASM 3 standard-library gate catalogue and Zamani IR capability map.
//!
//! This module is the single source of truth for the names, arities,
//! parameter counts, aliases, and version availability of gates supplied by
//! `include "stdgates.inc";`.
//!
//! Architectural boundary:
//!
//! ```text
//! OpenQASM source
//!       │
//!       ▼
//! openqasm::lexer / parser
//!       │
//!       ▼
//! openqasm::validation
//!       │
//!       ▼
//! this module ──────► canonical quantum::ir::GateKind
//!       │              when directly representable
//!       │
//!       └────────────► explicit Unsupported result
//!                      when the IR lacks the required operation
//! ```
//!
//! The catalogue deliberately does not:
//!
//! - parse OpenQASM source;
//! - construct `Gate` values;
//! - perform qubit mapping;
//! - perform optimization;
//! - silently decompose unsupported gates;
//! - silently discard unsupported operations;
//! - depend on another frontend format.
//!
//! This keeps OpenQASM-specific knowledge isolated from the canonical
//! Quantum IR while allowing the importer and validator to query exactly
//! what the standard library defines and what Zamani can currently represent.
//!
//! Rust compatibility: Rust 1.97.1.
//! No nightly features.
//! No external dependencies.

use crate::quantum::ir::GateKind;

/// Canonical include name for the OpenQASM 3 standard library.
pub const STANDARD_LIBRARY_INCLUDE: &str = "stdgates.inc";

/// OpenQASM major version represented by this standard-library catalogue.
pub const OPENQASM_MAJOR_VERSION: u8 = 3;

/// Minimum OpenQASM major version represented by this catalogue.
pub const STANDARD_LIBRARY_MIN_MAJOR: u8 = 3;

/// Minimum OpenQASM minor version represented by this catalogue.
pub const STANDARD_LIBRARY_MIN_MINOR: u8 = 0;

/// Describes how a standard-library gate maps into the canonical Quantum IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardGateLowering {
    /// The standard-library operation has a direct canonical IR
    /// representation.
    Direct(GateKind),

    /// The operation is part of OpenQASM's standard library but the current
    /// canonical IR does not contain a semantically equivalent operation.
    ///
    /// The reason is deliberately static and machine-readable. The importer
    /// or validator is responsible for attaching source spans and frontend
    /// diagnostic codes.
    Unsupported(&'static str),
}

impl StandardGateLowering {
    /// Returns the canonical IR gate when directly representable.
    #[must_use]
    pub const fn gate_kind(self) -> Option<GateKind> {
        match self {
            Self::Direct(kind) => Some(kind),
            Self::Unsupported(_) => None,
        }
    }

    /// Returns whether this operation is directly representable.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        matches!(self, Self::Direct(_))
    }

    /// Returns the stable reason for unsupported lowering.
    #[must_use]
    pub const fn unsupported_reason(self) -> Option<&'static str> {
        match self {
            Self::Direct(_) => None,
            Self::Unsupported(reason) => Some(reason),
        }
    }
}

/// Immutable metadata describing one OpenQASM standard-library gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardGate {
    /// Exact OpenQASM source spelling.
    ///
    /// OpenQASM identifiers are case-sensitive.
    name: &'static str,

    /// Canonical standard-library spelling.
    ///
    /// Compatibility aliases point at their canonical gate.
    canonical_name: &'static str,

    /// Number of quantum operands.
    qubit_count: usize,

    /// Number of angle parameters.
    parameter_count: usize,

    /// First OpenQASM major version containing this entry.
    introduced_major: u8,

    /// First OpenQASM minor version containing this entry.
    introduced_minor: u8,

    /// Current lowering capability.
    lowering: StandardGateLowering,
}

impl StandardGate {
    const fn new(
        name: &'static str,
        canonical_name: &'static str,
        qubit_count: usize,
        parameter_count: usize,
        introduced_major: u8,
        introduced_minor: u8,
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

    /// Exact OpenQASM source spelling.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Canonical standard-library spelling.
    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        self.canonical_name
    }

    /// Required number of quantum operands.
    #[must_use]
    pub const fn qubit_count(self) -> usize {
        self.qubit_count
    }

    /// Required number of angle parameters.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameter_count
    }

    /// First OpenQASM major version containing this entry.
    #[must_use]
    pub const fn introduced_major(self) -> u8 {
        self.introduced_major
    }

    /// First OpenQASM minor version containing this entry.
    #[must_use]
    pub const fn introduced_minor(self) -> u8 {
        self.introduced_minor
    }

    /// Returns the lowering capability.
    #[must_use]
    pub const fn lowering(self) -> StandardGateLowering {
        self.lowering
    }

    /// Returns the direct canonical IR representation.
    #[must_use]
    pub const fn gate_kind(self) -> Option<GateKind> {
        self.lowering.gate_kind()
    }

    /// Returns whether this operation can currently be represented directly.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.lowering.is_supported()
    }

    /// Returns whether this entry is available in an OpenQASM version.
    #[must_use]
    pub const fn available_in(self, major: u8, minor: u8) -> bool {
        if major != self.introduced_major {
            return major > self.introduced_major;
        }

        minor >= self.introduced_minor
    }
}

/// Complete OpenQASM 3 standard-library gate catalogue.
///
/// This includes:
///
/// - standard OpenQASM 3 gates;
/// - OpenQASM 2 compatibility aliases reproduced by `stdgates.inc`;
/// - gates that are currently unsupported by the Zamani IR.
///
/// Unsupported gates intentionally remain in this table. They must not
/// disappear from the frontend merely because the IR cannot yet represent
/// them.
pub const STANDARD_GATES: &[StandardGate] = &[
    // ---------------------------------------------------------------------
    // Single-qubit gates.
    // ---------------------------------------------------------------------

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
            "canonical Quantum IR has no SX gate kind",
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

    // ---------------------------------------------------------------------
    // Two-qubit gates.
    // ---------------------------------------------------------------------

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
            "canonical Quantum IR has no controlled-phase gate kind",
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
            "canonical Quantum IR has no controlled-U gate kind",
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

    // ---------------------------------------------------------------------
    // Three-qubit gates.
    // ---------------------------------------------------------------------

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

    // ---------------------------------------------------------------------
    // OpenQASM 2 compatibility aliases reproduced by stdgates.inc.
    // ---------------------------------------------------------------------

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
            "controlled-phase alias requires unsupported CP operation",
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

/// Number of entries in the standard-library catalogue.
pub const STANDARD_GATE_COUNT: usize = STANDARD_GATES.len();

/// Looks up an exact, case-sensitive OpenQASM standard-library name.
///
/// OpenQASM identifiers are case-sensitive. Therefore `cx` and `CX` are
/// deliberately separate entries.
#[must_use]
pub fn lookup(name: &str) -> Option<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .find(|gate| gate.name == name)
}

/// Looks up the canonical entry for a source spelling.
///
/// For example:
///
/// ```text
/// CX      -> cx
/// phase   -> p
/// ```
#[must_use]
pub fn lookup_canonical(name: &str) -> Option<StandardGate> {
    let entry = lookup(name)?;

    lookup_exact_canonical(entry.canonical_name)
}

/// Returns the complete deterministic catalogue.
#[must_use]
pub const fn all() -> &'static [StandardGate] {
    STANDARD_GATES
}

/// Returns all currently directly representable standard-library entries.
#[must_use]
pub fn supported() -> Vec<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .filter(|gate| gate.is_supported())
        .collect()
}

/// Returns all standard-library entries that cannot currently be lowered to
/// the canonical Quantum IR.
#[must_use]
pub fn unsupported() -> Vec<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .filter(|gate| !gate.is_supported())
        .collect()
}

/// Returns whether a source name is part of the standard library for the
/// specified OpenQASM language version.
#[must_use]
pub fn is_available(name: &str, major: u8, minor: u8) -> bool {
    lookup(name).is_some_and(|gate| gate.available_in(major, minor))
}

/// Returns the direct canonical IR gate for a source name.
///
/// `None` means either:
///
/// - the source name is unknown; or
/// - the source name is known but currently unsupported by the IR.
///
/// Callers that must distinguish those cases should use [`lookup`] first.
#[must_use]
pub fn gate_kind(name: &str) -> Option<GateKind> {
    lookup(name).and_then(StandardGate::gate_kind)
}

fn lookup_exact_canonical(name: &str) -> Option<StandardGate> {
    STANDARD_GATES
        .iter()
        .copied()
        .find(|gate| gate.name == name && gate.canonical_name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_library_include_name_is_stable() {
        assert_eq!(STANDARD_LIBRARY_INCLUDE, "stdgates.inc");
    }

    #[test]
    fn catalogue_is_non_empty_and_deterministic() {
        assert!(!STANDARD_GATES.is_empty());
        assert_eq!(STANDARD_GATES.len(), STANDARD_GATE_COUNT);

        for pair in STANDARD_GATES.windows(2) {
            assert_ne!(pair[0].name(), pair[1].name());
        }
    }

    #[test]
    fn standard_library_names_are_case_sensitive() {
        assert_eq!(lookup("cx").map(StandardGate::name), Some("cx"));
        assert_eq!(lookup("CX").map(StandardGate::name), Some("CX"));
        assert!(lookup("Cx").is_none());
    }

    #[test]
    fn compatibility_aliases_resolve_to_canonical_entries() {
        let cx = lookup("CX").expect("CX must be present");
        assert_eq!(cx.canonical_name(), "cx");
        assert_eq!(cx.gate_kind(), Some(GateKind::CX));

        let phase = lookup("phase").expect("phase must be present");
        assert_eq!(phase.canonical_name(), "p");
        assert_eq!(phase.gate_kind(), Some(GateKind::Phase));

        let canonical =
            lookup_canonical("CX").expect("canonical CX entry must exist");

        assert_eq!(canonical.name(), "cx");
        assert_eq!(canonical.canonical_name(), "cx");
    }

    #[test]
    fn standard_gate_arities_are_correct() {
        let expected = [
            ("x", 1, 0),
            ("rx", 1, 1),
            ("cx", 2, 0),
            ("cp", 2, 1),
            ("cu", 2, 4),
            ("ccx", 3, 0),
            ("cswap", 3, 0),
            ("u1", 1, 1),
            ("u2", 1, 2),
            ("u3", 1, 3),
        ];

        for (name, qubits, parameters) in expected {
            let gate =
                lookup(name).expect("expected standard-library gate");

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
    fn supported_gates_map_to_existing_ir_kinds() {
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
                "wrong lowering for {name}"
            );
        }
    }

    #[test]
    fn unsupported_gates_are_explicit() {
        for name in ["sx", "cp", "cu"] {
            let gate =
                lookup(name).expect("gate must exist in standard library");

            assert!(
                !gate.is_supported(),
                "{name} must not be silently lowered"
            );

            assert!(gate.lowering().unsupported_reason().is_some());
            assert!(gate_kind(name).is_none());
        }

        let cphase =
            lookup("cphase").expect("cphase must exist as an alias");

        assert!(!cphase.is_supported());
    }

    #[test]
    fn version_availability_is_explicit() {
        assert!(is_available("x", 3, 0));
        assert!(is_available("x", 3, 1));
        assert!(is_available("x", 3, 2));

        assert!(!is_available("x", 2, 0));

        // CX is supplied by stdgates.inc in OpenQASM 3.
        // It was a builtin in OpenQASM 2 rather than an entry in stdgates.inc.
        assert!(is_available("CX", 3, 0));
        assert!(!is_available("CX", 2, 0));
    }

    #[test]
    fn unknown_names_are_not_standard_gates() {
        assert!(lookup("cnot").is_none());
        assert!(lookup("CNOT").is_none());
        assert!(lookup("custom_gate").is_none());
        assert!(gate_kind("custom_gate").is_none());
    }

    #[test]
    fn unsupported_entries_remain_visible() {
        let unsupported_gates = unsupported();

        let names: Vec<&str> = unsupported_gates
            .iter()
            .map(|gate| gate.name())
            .collect();

        assert!(names.contains(&"sx"));
        assert!(names.contains(&"cp"));
        assert!(names.contains(&"cu"));
        assert!(names.contains(&"cphase"));
    }
}