
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Semantic Validation
//!
//! Path: `src/frontend/semantic/domains/hdl.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Validate source-level hardware-description semantics without choosing a
//! physical target, allocating hardware, or lowering to an HDL-specific IR.
//!
//! ## AUTHORITY
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/ast/node/domains/`
//! - `src/frontend/semantic/model/`
//!
//! The grammar owns syntax. The canonical AST owns source structure. Shared
//! semantic infrastructure owns name resolution, core types, effects,
//! capabilities, resources, diagnostics, and provenance. This module owns
//! HDL-specific validation rules over an explicit semantic input contract.
//!
//! ## DOES NOT OWN
//!
//! - A replacement AST or parser.
//! - General-purpose symbol resolution or type inference.
//! - Physical clock synthesis, placement, routing, or timing closure.
//! - Target capability discovery or resource allocation.
//! - Hardware vendor APIs or device handles.
//! - Classical IR, `quantum::ir`, or a competing universal IR.
//! - Simulation, synthesis, scheduling, or runtime execution.
//!
//! ## SCALABILITY
//!
//! No architectural maximum is imposed on signals, ports, processes, clock
//! domains, dependency edges, or declared bit widths. Widths can remain
//! symbolic. Collections use dynamically sized standard-library containers.
//!
//! Concrete compilation remains finite and is subject to available resources
//! and the caller's configured operational budgets. This module does not
//! introduce a hidden language-level capacity limit.
//!
//! ## INTEGRATION CONTRACT
//!
//! The AST-to-semantic adapter maps canonical HDL AST nodes into `HdlProgram`.
//! It must preserve source identity, symbolic expressions, declaration
//! direction, assignment intent, process kind, and clock/reset relationships.
//! The shared semantic pipeline invokes [`validate_hdl`] and translates
//! [`HdlDiagnostic`] values into the repository's canonical diagnostic type.
//!
//! This module intentionally does not assume unverified constructors or field
//! names in the repository's AST and diagnostic APIs.
//!
//! ## FREEZE CRITERIA
//!
//! - Unit and property tests cover each validation rule.
//! - AST adapter mapping is loss-preserving.
//! - Diagnostics are deterministic.
//! - Symbolic widths are not truncated to a machine-sized integer.
//! - Validation does not silently certify incomplete analysis.
//! - The semantic pipeline converts diagnostics into its canonical format.
//! - The repository workspace and HDL conformance corpus pass.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// Stable identity for a source-level HDL declaration.
///
/// The AST adapter should derive this identity from canonical AST node
/// identity, or from a deterministic source-level identity scheme. It must
/// not use physical signal addresses or target-specific identifiers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlId(String);

impl HdlId {
    /// Create an identifier from a non-empty, non-whitespace value.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlInputError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(HdlInputError::EmptyIdentifier);
        }
        Ok(Self(value))
    }

    /// Return the stable identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HdlId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Input construction errors, distinct from semantic program errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlInputError {
    /// An identifier was empty or contained only whitespace.
    EmptyIdentifier,
    /// A declaration repeated an identifier within one collection.
    DuplicateIdentifier(HdlId),
    /// A value required by the input contract was absent.
    MissingRequiredValue(&'static str),
}

impl fmt::Display for HdlInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => f.write_str("HDL identifier cannot be empty"),
            Self::DuplicateIdentifier(id) => {
                write!(f, "duplicate HDL input identity `{id}`")
            }
            Self::MissingRequiredValue(name) => {
                write!(f, "missing required HDL input value `{name}`")
            }
        }
    }
}

impl std::error::Error for HdlInputError {}

/// A symbolic, target-independent bit-width expression.
///
/// This type deliberately preserves symbolic expressions instead of forcing
/// them into `usize`. `Add` and `Multiply` represent expression structure,
/// not evaluated values. The AST adapter should preserve the actual source
/// expression in its canonical AST and use this type for the normalized
/// semantic relationship.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlWidth {
    /// A known, positive bit width.
    Known(u128),
    /// A symbolic width parameter, constant, or resolved expression identity.
    Symbol(HdlId),
    /// A symbolic sum of widths.
    Add(Box<HdlWidth>, Box<HdlWidth>),
    /// A symbolic product of widths.
    Multiply(Box<HdlWidth>, Box<HdlWidth>),
    /// A width expression that has not yet been resolved.
    Unresolved(HdlId),
}

impl HdlWidth {
    /// Create a positive concrete width.
    ///
    /// A zero width is rejected here. If the language permits zero-sized
    /// hardware objects in a particular context, that rule should be
    /// represented explicitly by the owning specification and adapter.
    pub fn known(value: u128) -> Result<Self, HdlInputError> {
        if value == 0 {
            return Err(HdlInputError::MissingRequiredValue(
                "positive HDL bit width",
            ));
        }
        Ok(Self::Known(value))
    }

    /// Return the concrete width if this value is already a literal width.
    #[must_use]
    pub const fn as_known(&self) -> Option<u128> {
        match self {
            Self::Known(value) => Some(*value),
            _ => None,
        }
    }

    /// Return whether the width is unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        matches!(self, Self::Unresolved(_))
    }
}

/// Source-level signedness of a bit vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlSignedness {
    /// Unsigned bit-vector interpretation.
    Unsigned,
    /// Signed bit-vector interpretation.
    Signed,
    /// Signedness is not applicable or not yet established.
    NotApplicable,
}

/// Semantic category of an HDL signal's value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlValueType {
    /// A one-bit Boolean or logic value.
    Bit,
    /// A vector with symbolic or concrete width.
    BitVector {
        /// Number of bits in the vector.
        width: HdlWidth,
        /// Interpretation of the vector.
        signedness: HdlSignedness,
    },
    /// A clock signal.
    Clock,
    /// A reset signal.
    Reset,
    /// An extension-defined source-level type.
    ///
    /// The shared type checker remains responsible for resolving and
    /// validating the referenced type identity.
    Named(HdlId),
}

/// Source-level port or signal direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlDirection {
    /// Driven from outside the current module.
    Input,
    /// Driven by the current module and observed from outside.
    Output,
    /// Both read and driven according to the language's inout rules.
    InOut,
    /// Internal declaration with no module-port direction.
    Internal,
}

/// Source location of a semantic item.
///
/// Offsets use the canonical AST's UTF-8 byte-offset, half-open convention.
/// The source identifier should correspond to the repository's canonical
/// source-map identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlSourceLocation {
    /// Canonical source identity, rendered by the AST adapter.
    pub source: String,
    /// Inclusive UTF-8 byte offset.
    pub start: u64,
    /// Exclusive UTF-8 byte offset.
    pub end: u64,
}

impl HdlSourceLocation {
    /// Construct a validated half-open source range.
    pub fn new(
        source: impl Into<String>,
        start: u64,
        end: u64,
    ) -> Result<Self, HdlInputError> {
        if end < start {
            return Err(HdlInputError::MissingRequiredValue(
                "valid half-open source range",
            ));
        }
        Ok(Self {
            source: source.into(),
            start,
            end,
        })
    }
}

/// Source-level declaration of an HDL signal or port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlSignal {
    /// Stable semantic identity.
    pub id: HdlId,
    /// Source-level name used in diagnostics.
    pub name: String,
    /// Signal or port direction.
    pub direction: HdlDirection,
    /// Resolved or symbolic value type.
    pub value_type: HdlValueType,
    /// Source declaration location.
    pub location: Option<HdlSourceLocation>,
}

/// Source-level clock declaration.
///
/// Frequency and period are kept symbolic. Physical frequency feasibility
/// and timing closure belong to target-aware hardware analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClock {
    /// Stable clock declaration identity.
    pub id: HdlId,
    /// Signal representing this clock.
    pub signal: HdlId,
    /// Optional symbolic period expression identity.
    pub period: Option<HdlId>,
    /// Source declaration location.
    pub location: Option<HdlSourceLocation>,
}

/// Source-level reset declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlReset {
    /// Stable reset declaration identity.
    pub id: HdlId,
    /// Signal representing this reset.
    pub signal: HdlId,
    /// Active polarity.
    pub polarity: HdlResetPolarity,
    /// Whether assertion is synchronous or asynchronous.
    pub behavior: HdlResetBehavior,
    /// Source declaration location.
    pub location: Option<HdlSourceLocation>,
}

/// Reset polarity as declared by the source program.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlResetPolarity {
    /// Reset is asserted by a logical high value.
    ActiveHigh,
    /// Reset is asserted by a logical low value.
    ActiveLow,
}

/// Reset behavior declared by the source program.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlResetBehavior {
    /// Reset assertion is synchronized to a clock.
    Synchronous,
    /// Reset assertion does not require a clock edge.
    Asynchronous,
}

/// Source-level kind of an HDL process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlProcessKind {
    /// Combinational process.
    Combinational,
    /// Clocked or sequential process.
    Sequential,
    /// Process whose semantics are supplied by a registered extension.
    Extension,
}

/// One source-level assignment or drive relationship.
///
/// `reads` identifies declarations read by the expression producing the
/// assigned value. It is used to build the combinational dependency graph.
/// Expression typing itself remains owned by the shared type checker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlAssignment {
    /// Target signal identity.
    pub target: HdlId,
    /// Signal identities read by the right-hand-side expression.
    pub reads: Vec<HdlId>,
    /// Whether the assignment is a continuous drive.
    pub continuous: bool,
    /// Source location.
    pub location: Option<HdlSourceLocation>,
}

/// Source-level HDL process summary.
///
/// The AST adapter should populate this summary only after mapping the
/// canonical process AST. It must not infer missing event semantics from
/// a target or silently convert malformed syntax into a valid process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlProcess {
    /// Stable process identity.
    pub id: HdlId,
    /// Process kind.
    pub kind: HdlProcessKind,
    /// Assignments performed by the process.
    pub assignments: Vec<HdlAssignment>,
    /// Clock declarations controlling this process.
    pub clocks: Vec<HdlId>,
    /// Reset declarations associated with this process.
    pub resets: Vec<HdlId>,
    /// Source location.
    pub location: Option<HdlSourceLocation>,
}

/// A complete normalized input to HDL semantic validation.
///
/// This is a semantic input contract, not a competing syntax AST. The
/// canonical AST remains authoritative; an adapter builds this value for the
/// validator and retains the mapping back to source node identities.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlProgram {
    /// Signals and ports declared in the HDL design.
    pub signals: Vec<HdlSignal>,
    /// Clock declarations.
    pub clocks: Vec<HdlClock>,
    /// Reset declarations.
    pub resets: Vec<HdlReset>,
    /// Processes and their summarized assignments.
    pub processes: Vec<HdlProcess>,
}

/// Severity of an HDL semantic diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlSeverity {
    /// A semantic violation that prevents a valid result.
    Error,
    /// A suspicious or incomplete condition that requires review.
    Warning,
    /// Informational semantic feedback.
    Note,
}

/// Stable HDL diagnostic identifier.
///
/// The owning integration layer may map these identifiers to the repository's
/// central diagnostic-code registry. String rendering is not the identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlDiagnosticCode {
    /// Two declarations use the same semantic identity.
    DuplicateIdentity,
    /// A declaration uses a duplicate name in the supplied scope.
    DuplicateName,
    /// A reference names no declared signal.
    UnknownSignal,
    /// A signal is assigned despite its declared direction.
    InvalidDirection,
    /// A concrete bit width is invalid.
    InvalidWidth,
    /// A required clock declaration is absent or invalid.
    InvalidClock,
    /// A reset declaration is absent or invalid.
    InvalidReset,
    /// A process refers to a missing clock or reset.
    UnknownClockOrReset,
    /// A process has an invalid clock/reset configuration.
    InvalidProcess,
    /// A signal has multiple incompatible drivers.
    MultipleDrivers,
    /// A combinational dependency graph contains a cycle.
    CombinationalCycle,
    /// A width relation cannot yet be established.
    UnresolvedWidth,
    /// A value or relation could not be checked because analysis was incomplete.
    IncompleteAnalysis,
}

/// A structured HDL semantic diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlDiagnostic {
    /// Stable machine-readable code.
    pub code: HdlDiagnosticCode,
    /// Diagnostic severity.
    pub severity: HdlSeverity,
    /// Human-readable explanation.
    pub message: String,
    /// Primary source location, if known.
    pub location: Option<HdlSourceLocation>,
    /// Related source locations, ordered deterministically.
    pub related: Vec<HdlSourceLocation>,
}

impl HdlDiagnostic {
    fn error(
        code: HdlDiagnosticCode,
        message: impl Into<String>,
        location: Option<HdlSourceLocation>,
    ) -> Self {
        Self {
            code,
            severity: HdlSeverity::Error,
            message: message.into(),
            location,
            related: Vec::new(),
        }
    }

    fn warning(
        code: HdlDiagnosticCode,
        message: impl Into<String>,
        location: Option<HdlSourceLocation>,
    ) -> Self {
        Self {
            code,
            severity: HdlSeverity::Warning,
            message: message.into(),
            location,
            related: Vec::new(),
        }
    }
}

/// Final status of HDL semantic validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HdlValidationStatus {
    /// All checks implemented by this validator completed without errors.
    Valid,
    /// One or more semantic errors were detected.
    Invalid,
    /// Validation was deliberately stopped before all checks completed.
    Incomplete,
}

/// Result of HDL semantic validation.
///
/// `Valid` means only that this module's declared checks succeeded. It is not
/// a substitute for whole-program type checking, effect checking, contract
/// checking, capability analysis, or target feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlValidationResult {
    /// Overall status.
    pub status: HdlValidationStatus,
    /// Diagnostics in deterministic order.
    pub diagnostics: Vec<HdlDiagnostic>,
    /// Whether all checks represented by this input completed.
    pub complete: bool,
}

impl HdlValidationResult {
    fn from_diagnostics(diagnostics: Vec<HdlDiagnostic>) -> Self {
        let has_errors = diagnostics
            .iter()
            .any(|d| d.severity == HdlSeverity::Error);
        let incomplete = diagnostics.iter().any(|d| {
            d.code == HdlDiagnosticCode::IncompleteAnalysis
        });

        let status = if incomplete {
            HdlValidationStatus::Incomplete
        } else if has_errors {
            HdlValidationStatus::Invalid
        } else {
            HdlValidationStatus::Valid
        };

        Self {
            status,
            diagnostics,
            complete: !incomplete,
        }
    }
}

/// Validate source-level HDL declarations and process relationships.
///
/// The function is deterministic for equivalent input values. It does not
/// mutate the input, discover hardware, resolve physical capabilities, or
/// select a target. Callers should run shared structural validation and
/// ordinary type/name resolution as separate pipeline stages.
///
/// This entry point is intentionally non-panicking for malformed semantic
/// input. Operational budgets should be enforced by the calling session; if a
/// budget prevents completion, the caller must mark the result incomplete.
#[must_use]
pub fn validate_hdl(program: &HdlProgram) -> HdlValidationResult {
    let mut diagnostics = Vec::new();

    let mut signals_by_id = BTreeMap::new();
    let mut signals_by_name = BTreeMap::<String, Vec<HdlId>>::new();

    for signal in &program.signals {
        if signals_by_id.insert(signal.id.clone(), signal).is_some() {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::DuplicateIdentity,
                format!("duplicate signal identity `{}`", signal.id),
                signal.location.clone(),
            ));
        }
        signals_by_name
            .entry(signal.name.clone())
            .or_default()
            .push(signal.id.clone());

        match &signal.value_type {
            HdlValueType::BitVector {
                width: HdlWidth::Known(0),
                ..
            } => diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::InvalidWidth,
                format!("signal `{}` has a zero bit width", signal.name),
                signal.location.clone(),
            )),
            HdlValueType::BitVector {
                width: HdlWidth::Unresolved(id),
                ..
            } => diagnostics.push(HdlDiagnostic::warning(
                HdlDiagnosticCode::UnresolvedWidth,
                format!(
                    "width for signal `{}` remains unresolved ({id})",
                    signal.name
                ),
                signal.location.clone(),
            )),
            _ => {}
        }
    }

    for (name, ids) in &signals_by_name {
        if ids.len() > 1 {
            let mut diagnostic = HdlDiagnostic::error(
                HdlDiagnosticCode::DuplicateName,
                format!("signal name `{name}` is declared more than once"),
                ids.first()
                    .and_then(|id| signals_by_id.get(id))
                    .and_then(|signal| signal.location.clone()),
            );
            diagnostic.related = ids
                .iter()
                .filter_map(|id| signals_by_id.get(id))
                .filter_map(|signal| signal.location.clone())
                .collect();
            diagnostics.push(diagnostic);
        }
    }

    let mut clocks_by_id = BTreeMap::new();
    for clock in &program.clocks {
        if clocks_by_id.insert(clock.id.clone(), clock).is_some() {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::DuplicateIdentity,
                format!("duplicate clock identity `{}`", clock.id),
                clock.location.clone(),
            ));
        }

        match signals_by_id.get(&clock.signal) {
            None => diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::InvalidClock,
                format!(
                    "clock `{}` references unknown signal `{}`",
                    clock.id, clock.signal
                ),
                clock.location.clone(),
            )),
            Some(signal) if !matches!(signal.value_type, HdlValueType::Clock) => {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::InvalidClock,
                    format!(
                        "clock `{}` must reference a signal declared with clock type",
                        clock.id
                    ),
                    clock.location.clone(),
                ));
            }
            Some(_) => {}
        }
    }

    let mut resets_by_id = BTreeMap::new();
    for reset in &program.resets {
        if resets_by_id.insert(reset.id.clone(), reset).is_some() {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::DuplicateIdentity,
                format!("duplicate reset identity `{}`", reset.id),
                reset.location.clone(),
            ));
        }

        match signals_by_id.get(&reset.signal) {
            None => diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::InvalidReset,
                format!(
                    "reset `{}` references unknown signal `{}`",
                    reset.id, reset.signal
                ),
                reset.location.clone(),
            )),
            Some(signal) if !matches!(signal.value_type, HdlValueType::Reset) => {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::InvalidReset,
                    format!(
                        "reset `{}` must reference a signal declared with reset type",
                        reset.id
                    ),
                    reset.location.clone(),
                ));
            }
            Some(_) => {}
        }
    }

    let mut process_ids = BTreeSet::new();
    let mut drivers = BTreeMap::<HdlId, Vec<(HdlId, Option<HdlSourceLocation>)>>::new();
    let mut combinational_edges = BTreeMap::<HdlId, BTreeSet<HdlId>>::new();

    for process in &program.processes {
        if !process_ids.insert(process.id.clone()) {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::DuplicateIdentity,
                format!("duplicate process identity `{}`", process.id),
                process.location.clone(),
            ));
        }

        if process.kind == HdlProcessKind::Sequential && process.clocks.is_empty() {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::InvalidProcess,
                format!(
                    "sequential process `{}` must declare at least one clock",
                    process.id
                ),
                process.location.clone(),
            ));
        }

        if process.kind == HdlProcessKind::Combinational && !process.clocks.is_empty() {
            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::InvalidProcess,
                format!(
                    "combinational process `{}` cannot declare sequential clock events",
                    process.id
                ),
                process.location.clone(),
            ));
        }

        for clock_id in &process.clocks {
            if !clocks_by_id.contains_key(clock_id) {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::UnknownClockOrReset,
                    format!(
                        "process `{}` references unknown clock `{clock_id}`",
                        process.id
                    ),
                    process.location.clone(),
                ));
            }
        }

        for reset_id in &process.resets {
            if !resets_by_id.contains_key(reset_id) {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::UnknownClockOrReset,
                    format!(
                        "process `{}` references unknown reset `{reset_id}`",
                        process.id
                    ),
                    process.location.clone(),
                ));
            }
        }

        for assignment in &process.assignments {
            let Some(target) = signals_by_id.get(&assignment.target) else {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::UnknownSignal,
                    format!(
                        "process `{}` assigns unknown signal `{}`",
                        process.id, assignment.target
                    ),
                    assignment.location.clone().or_else(|| process.location.clone()),
                ));
                continue;
            };

            let can_drive = match target.direction {
                HdlDirection::Input => false,
                HdlDirection::Output | HdlDirection::InOut | HdlDirection::Internal => true,
            };

            if !can_drive {
                diagnostics.push(HdlDiagnostic::error(
                    HdlDiagnosticCode::InvalidDirection,
                    format!(
                        "process `{}` cannot drive input signal `{}`",
                        process.id, target.name
                    ),
                    assignment.location.clone().or_else(|| target.location.clone()),
                ));
            }

            drivers
                .entry(assignment.target.clone())
                .or_default()
                .push((process.id.clone(), assignment.location.clone()));

            for read_id in &assignment.reads {
                if !signals_by_id.contains_key(read_id) {
                    diagnostics.push(HdlDiagnostic::error(
                        HdlDiagnosticCode::UnknownSignal,
                        format!(
                            "assignment in process `{}` reads unknown signal `{read_id}`",
                            process.id
                        ),
                        assignment.location.clone().or_else(|| process.location.clone()),
                    ));
                    continue;
                }

                if process.kind == HdlProcessKind::Combinational {
                    combinational_edges
                        .entry(assignment.target.clone())
                        .or_default()
                        .insert(read_id.clone());
                }
            }
        }
    }

    for (signal_id, signal_drivers) in &drivers {
        let distinct_processes: BTreeSet<&HdlId> =
            signal_drivers.iter().map(|(process_id, _)| process_id).collect();

        let Some(signal) = signals_by_id.get(signal_id) else {
            continue;
        };

        // Multiple assignments within one process are not automatically
        // illegal: procedural control flow may establish exclusivity or
        // ordering. This check detects distinct process drivers only.
        if distinct_processes.len() > 1 {
            let mut diagnostic = HdlDiagnostic::error(
                HdlDiagnosticCode::MultipleDrivers,
                format!(
                    "signal `{}` is driven by multiple processes",
                    signal.name
                ),
                signal.location.clone(),
            );
            diagnostic.related = signal_drivers
                .iter()
                .filter_map(|(_, location)| location.clone())
                .collect();
            diagnostics.push(diagnostic);
        }
    }

    diagnostics.extend(find_combinational_cycles(&combinational_edges, &signals_by_id));

    // A stable order independent of insertion order makes diagnostics easier
    // to compare, cache, test, and consume from IDE tooling.
    diagnostics.sort_by(|left, right| {
        (
            left.location.as_ref().map(|l| (&l.source, l.start, l.end)),
            left.code,
            &left.message,
        )
            .cmp(&(
                right.location.as_ref().map(|l| (&l.source, l.start, l.end)),
                right.code,
                &right.message,
            ))
    });

    HdlValidationResult::from_diagnostics(diagnostics)
}

/// Find cycles in a signal dependency graph using iterative strongly connected
/// component discovery.
///
/// An edge `A -> B` means the value assigned to signal `A` reads signal `B`.
/// Cycles in this graph are reported as combinational cycles. Sequential
/// feedback should not be included by the caller when building this graph.
///
/// This algorithm avoids recursive DFS, so graph depth does not consume the
/// Rust call stack. Its storage is proportional to the graph size.
fn find_combinational_cycles(
    graph: &BTreeMap<HdlId, BTreeSet<HdlId>>,
    signals: &BTreeMap<HdlId, &HdlSignal>,
) -> Vec<HdlDiagnostic> {
    let mut nodes = BTreeSet::new();
    for (node, edges) in graph {
        nodes.insert(node.clone());
        nodes.extend(edges.iter().cloned());
    }

    // Iterative DFS finishing order.
    let mut visited = BTreeSet::new();
    let mut finish_order = Vec::new();

    for start in &nodes {
        if visited.contains(start) {
            continue;
        }

        let mut stack = vec![(start.clone(), false)];
        while let Some((node, exiting)) = stack.pop() {
            if exiting {
                finish_order.push(node);
                continue;
            }
            if !visited.insert(node.clone()) {
                continue;
            }

            stack.push((node.clone(), true));
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors.iter().rev() {
                    if !visited.contains(neighbor) {
                        stack.push((neighbor.clone(), false));
                    }
                }
            }
        }
    }

    // Build the reversed graph for the second Kosaraju pass.
    let mut reverse = BTreeMap::<HdlId, BTreeSet<HdlId>>::new();
    for node in &nodes {
        reverse.entry(node.clone()).or_default();
    }
    for (node, edges) in graph {
        for edge in edges {
            reverse.entry(edge.clone()).or_default().insert(node.clone());
        }
    }

    let mut assigned = BTreeSet::new();
    let mut diagnostics = Vec::new();

    for start in finish_order.into_iter().rev() {
        if !assigned.insert(start.clone()) {
            continue;
        }

        let mut component = Vec::new();
        let mut queue = VecDeque::from([start.clone()]);

        while let Some(node) = queue.pop_front() {
            component.push(node.clone());
            if let Some(neighbors) = reverse.get(&node) {
                for neighbor in neighbors {
                    if assigned.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        let self_loop = component.len() == 1
            && graph
                .get(&component[0])
                .is_some_and(|edges| edges.contains(&component[0]));

        if component.len() > 1 || self_loop {
            component.sort();
            let names = component
                .iter()
                .map(|id| {
                    signals
                        .get(id)
                        .map(|signal| signal.name.as_str())
                        .unwrap_or(id.as_str())
                })
                .collect::<Vec<_>>()
                .join(" -> ");

            let location = component
                .first()
                .and_then(|id| signals.get(id))
                .and_then(|signal| signal.location.clone());

            diagnostics.push(HdlDiagnostic::error(
                HdlDiagnosticCode::CombinationalCycle,
                format!("combinational dependency cycle detected: {names}"),
                location,
            ));
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier is valid")
    }

    fn signal(
        name: &str,
        direction: HdlDirection,
        value_type: HdlValueType,
    ) -> HdlSignal {
        HdlSignal {
            id: id(name),
            name: name.to_owned(),
            direction,
            value_type,
            location: None,
        }
    }

    #[test]
    fn empty_program_is_valid() {
        let result = validate_hdl(&HdlProgram::default());
        assert_eq!(result.status, HdlValidationStatus::Valid);
        assert!(result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn zero_width_is_rejected() {
        let program = HdlProgram {
            signals: vec![signal(
                "data",
                HdlDirection::Internal,
                HdlValueType::BitVector {
                    width: HdlWidth::Known(0),
                    signedness: HdlSignedness::Unsigned,
                },
            )],
            ..HdlProgram::default()
        };

        let result = validate_hdl(&program);
        assert_eq!(result.status, HdlValidationStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == HdlDiagnosticCode::InvalidWidth
        }));
    }

    #[test]
    fn symbolic_width_is_preserved() {
        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("lanes"))),
            Box::new(HdlWidth::Known(8)),
        );

        assert_eq!(width.as_known(), None);

        let program = HdlProgram {
            signals: vec![signal(
                "payload",
                HdlDirection::Internal,
                HdlValueType::BitVector {
                    width,
                    signedness: HdlSignedness::Unsigned,
                },
            )],
            ..HdlProgram::default()
        };

        assert_eq!(validate_hdl(&program).status, HdlValidationStatus::Valid);
    }

    #[test]
    fn input_signal_cannot_be_driven_by_a_process() {
        let program = HdlProgram {
            signals: vec![signal(
                "external",
                HdlDirection::Input,
                HdlValueType::Bit,
            )],
            processes: vec![HdlProcess {
                id: id("comb"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("external"),
                    reads: Vec::new(),
                    continuous: false,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_hdl(&program);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == HdlDiagnosticCode::InvalidDirection
        }));
    }

    #[test]
    fn combinational_self_dependency_is_reported() {
        let program = HdlProgram {
            signals: vec![signal(
                "feedback",
                HdlDirection::Internal,
                HdlValueType::Bit,
            )],
            processes: vec![HdlProcess {
                id: id("comb"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("feedback"),
                    reads: vec![id("feedback")],
                    continuous: true,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_hdl(&program);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == HdlDiagnosticCode::CombinationalCycle
        }));
    }

    #[test]
    fn sequential_process_requires_a_clock() {
        let program = HdlProgram {
            processes: vec![HdlProcess {
                id: id("seq"),
                kind: HdlProcessKind::Sequential,
                assignments: Vec::new(),
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_hdl(&program);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == HdlDiagnosticCode::InvalidProcess
        }));
    }

    #[test]
    fn diagnostics_have_stable_order() {
        let program = HdlProgram {
            signals: vec![
                signal("z", HdlDirection::Internal, HdlValueType::Bit),
                signal("a", HdlDirection::Internal, HdlValueType::Bit),
            ],
            processes: vec![HdlProcess {
                id: id("p"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("missing"),
                    reads: vec![id("also_missing")],
                    continuous: false,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let first = validate_hdl(&program);
        let second = validate_hdl(&program);
        assert_eq!(first, second);
    }
}
