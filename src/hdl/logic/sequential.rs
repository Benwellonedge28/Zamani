
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Sequential Behavior Contracts
//!
//! Path: `src/hdl/logic/sequential.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Target-independent sequential semantic contracts. Full sequential
//! validation requires an adapter to the canonical AST and semantic model.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/clocking.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/expressions.rs`
//! - `src/hdl/logic/assignment.rs`
//! - `src/hdl/logic/expression.rs`
//! - `src/hdl/design/signal.rs`
//!
//! The grammar owns sequential syntax. The canonical AST owns source nodes
//! and structure. The semantic layer owns name resolution, expression
//! meaning, clock/reset validity, and source-language legality.
//!
//! ## Owns
//!
//! - The sequential event-edge and event-qualifier vocabulary specified by
//!   the current grammar.
//! - A validated reference to a canonical sequential event specification.
//! - The semantic-checking extension point for event sources.
//! - The integration check that an assignment belongs to a sequential
//!   context before delegating assignment validation.
//! - Explicit valid, invalid, and indeterminate event-analysis outcomes.
//!
//! ## Does not own
//!
//! - Parsing, a replacement AST, or a second expression representation.
//! - Clock/reset declarations or a second clock-domain model.
//! - General type checking, width resolution, or assignment compatibility.
//! - State-machine declarations, memory semantics, or general process ASTs.
//! - A new nonblocking assignment operator.
//! - A default assumption about update ordering that the language contract
//!   has not established.
//! - Event scheduling, simulation time, synthesis, or physical registers.
//! - Physical clock generation, placement, routing, or target selection.
//! - HDLIR structures, builders, verification, passes, serialization,
//!   or lowering. All HDLIR implementation remains in `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The AST adapter constructs event references from canonical `NodeId`
//! values and labels them with their actual HDL expression roles. A semantic
//! checker implements `SequentialEventChecker` using canonical name
//! resolution and semantic facts. Sequential assignments are validated
//! through the existing `HdlAssignment` and `AssignmentTypeChecker` APIs.
//!
//! This module intentionally does not infer that an expression is a clock
//! merely because of its spelling or role. The semantic checker must prove
//! the event-source meaning.
//!
//! ## Scalability
//!
//! No language-level ceiling is imposed on events, assignments, signals,
//! clocks, or design size. Collections grow according to available resources.
//! A caller may impose operational budgets in its analysis layer; exhaustion
//! must remain distinguishable from successful validation.
//!
//! ## Freeze criteria
//!
//! - The frontend adapter preserves canonical AST identity and event roles.
//! - Event-source semantics are established by the semantic checker.
//! - Sequential assignment validation delegates to the existing assignment
//!   contract rather than reimplementing type and mutability checks.
//! - Unknown or incomplete semantic facts never become proof of validity.
//! - No physical realization or HDLIR ownership is introduced.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::HdlId;
use crate::hdl::design::signal::HdlSignal;
use crate::hdl::expressions::{HdlExpressionRole, HdlExpressionUse};
use crate::hdl::logic::assignment::{
    AssignmentContext, AssignmentError, AssignmentTypeChecker,
    AssignmentValidation, HdlAssignment,
};

/// Version of the public sequential-contract API.
///
/// This is independent of the Zamani language and HDLIR schema versions.
pub const HDL_SEQUENTIAL_CONTRACT_VERSION: u16 = 1;

/// Edge condition written in a sequential event specification.
///
/// These variants correspond to the edge vocabulary in
/// `grammar/hdl/sequential.g4`. They describe source-level event intent,
/// not a physical clock edge detector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SequentialEdge {
    /// The event occurs on a rising edge.
    Posedge,

    /// The event occurs on a falling edge.
    Negedge,
}

/// Qualifier attached to a sequential event source.
///
/// The semantic layer determines whether a qualifier is legal for the
/// resolved source and the enclosing sequential construct.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SequentialEventQualifier {
    /// Asynchronous event qualification.
    Async,

    /// Synchronous event qualification.
    Sync,
}

/// Errors from local sequential-contract construction and integration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SequentialError {
    /// The expression reference is not marked as an event, clock, or reset
    /// expression.
    InvalidEventExpressionRole,

    /// An event specification was constructed without any events.
    EmptyEventSpecification,

    /// The same qualifier was supplied more than once for one event.
    DuplicateEventQualifier {
        /// Repeated qualifier.
        qualifier: SequentialEventQualifier,
    },

    /// An event was simultaneously marked synchronous and asynchronous.
    ConflictingEventQualifiers,

    /// The assignment does not belong to a sequential construct.
    WrongAssignmentContext {
        /// Context found on the assignment.
        actual: AssignmentContext,
    },

    /// The existing assignment validator rejected the assignment.
    AssignmentValidation(AssignmentError),
}

impl fmt::Display for SequentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEventExpressionRole => formatter.write_str(
                "sequential event source must have an event, clock, or reset role",
            ),
            Self::EmptyEventSpecification => {
                formatter.write_str("sequential event specification cannot be empty")
            }
            Self::DuplicateEventQualifier { qualifier } => {
                write!(formatter, "duplicate sequential event qualifier {qualifier:?}")
            }
            Self::ConflictingEventQualifiers => formatter.write_str(
                "a sequential event cannot be both synchronous and asynchronous",
            ),
            Self::WrongAssignmentContext { actual } => write!(
                formatter,
                "sequential assignment validation received context {actual:?}",
            ),
            Self::AssignmentValidation(error) => {
                write!(formatter, "sequential assignment validation failed: {error}")
            }
        }
    }
}

impl std::error::Error for SequentialError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AssignmentValidation(error) => Some(error),
            _ => None,
        }
    }
}

/// One event source within a sequential event specification.
///
/// The expression is a reference to the canonical frontend AST. Its source
/// text, node, type, and resolved declaration are not copied into this
/// structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequentialEvent {
    source: HdlExpressionUse,
    edge: Option<SequentialEdge>,
    qualifiers: Vec<SequentialEventQualifier>,
}

impl SequentialEvent {
    /// Construct a sequential event reference and validate its local shape.
    ///
    /// This method validates only the expression role and qualifier
    /// combination. It does not prove that the expression resolves to a
    /// clock, reset, or other legal event source.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression role is inappropriate, a qualifier
    /// is repeated, or synchronous and asynchronous qualifiers conflict.
    pub fn new(
        source: HdlExpressionUse,
        edge: Option<SequentialEdge>,
        qualifiers: Vec<SequentialEventQualifier>,
    ) -> Result<Self, SequentialError> {
        match source.role() {
            HdlExpressionRole::EventOrSensitivity
            | HdlExpressionRole::Clock
            | HdlExpressionRole::Reset => {}
            _ => return Err(SequentialError::InvalidEventExpressionRole),
        }

        validate_qualifiers(&qualifiers)?;

        Ok(Self {
            source,
            edge,
            qualifiers,
        })
    }

    /// Return the canonical event-source expression reference.
    #[must_use]
    pub fn source(&self) -> &HdlExpressionUse {
        &self.source
    }

    /// Return the optional edge requirement.
    #[must_use]
    pub const fn edge(&self) -> Option<SequentialEdge> {
        self.edge
    }

    /// Return event qualifiers in their supplied order.
    #[must_use]
    pub fn qualifiers(&self) -> &[SequentialEventQualifier] {
        &self.qualifiers
    }

    /// Return the canonical AST node used as the event source.
    #[must_use]
    pub fn expression_id(&self) -> &NodeId {
        self.source.expression()
    }
}

/// A non-empty list of sequential events.
///
/// The enclosing sequential declaration may omit its event specification;
/// represent that at the caller as `None`. If a specification is present,
/// this type requires at least one event, matching the grammar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequentialEventSpecification {
    events: Vec<SequentialEvent>,
}

impl SequentialEventSpecification {
    /// Construct a sequential event specification.
    ///
    /// The list is dynamically sized. No fixed number of events is imposed.
    ///
    /// # Errors
    ///
    /// Returns `EmptyEventSpecification` if the supplied list is empty.
    pub fn new(events: Vec<SequentialEvent>) -> Result<Self, SequentialError> {
        if events.is_empty() {
            return Err(SequentialError::EmptyEventSpecification);
        }

        Ok(Self { events })
    }

    /// Return the event references in source order.
    #[must_use]
    pub fn events(&self) -> &[SequentialEvent] {
        &self.events
    }

    /// Return the number of event references in this specification.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Return whether this specification contains no events.
    ///
    /// A successfully constructed specification is never empty. This method
    /// is provided for conventional collection ergonomics.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// Semantic validity assigned to a sequential event by the canonical
/// semantic integration layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EventSourceValidity {
    /// The semantic checker established that the event is valid in context.
    Valid,

    /// The semantic checker established that the event is invalid.
    Invalid,

    /// Available semantic information is insufficient to decide validity.
    Indeterminate,
}

/// Extension point for checking sequential event semantics.
///
/// Implementations must use canonical name resolution and semantic facts.
/// They must not accept an event merely because its identifier is named
/// `clock`, `clk`, `reset`, or another conventional spelling.
pub trait SequentialEventChecker {
    /// Check one event in its enclosing sequential context.
    ///
    /// `region` identifies the sequential declaration using the canonical
    /// HDL identity. `event` retains the canonical AST expression reference.
    ///
    /// Return `Indeterminate` when the event's meaning cannot yet be
    /// established. Do not treat that outcome as success.
    fn check_event(
        &self,
        region: &HdlId,
        event: &SequentialEvent,
    ) -> EventSourceValidity;
}

/// Summary of checking a sequential event specification.
///
/// Event indices are zero-based positions in the original source-order list.
/// Separate vectors preserve the distinction between proven invalidity and
/// insufficient semantic information.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SequentialEventValidation {
    invalid_events: Vec<usize>,
    indeterminate_events: Vec<usize>,
}

impl SequentialEventValidation {
    /// Create an empty successful validation summary.
    #[must_use]
    pub fn valid() -> Self {
        Self::default()
    }

    /// Return indices of events established to be invalid.
    #[must_use]
    pub fn invalid_events(&self) -> &[usize] {
        &self.invalid_events
    }

    /// Return indices of events whose validity is indeterminate.
    #[must_use]
    pub fn indeterminate_events(&self) -> &[usize] {
        &self.indeterminate_events
    }

    /// Return whether every event was established to be valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.invalid_events.is_empty() && self.indeterminate_events.is_empty()
    }

    /// Return whether at least one event was established to be invalid.
    #[must_use]
    pub fn has_invalid_events(&self) -> bool {
        !self.invalid_events.is_empty()
    }

    /// Return whether at least one event remains indeterminate.
    #[must_use]
    pub fn has_indeterminate_events(&self) -> bool {
        !self.indeterminate_events.is_empty()
    }
}

/// Validate event sources using the canonical semantic integration.
///
/// This function does not resolve names itself, inspect expression spelling,
/// or choose clock/reset implementation. It preserves each check result and
/// never converts indeterminate results into success.
///
/// # Arguments
///
/// * `region` — canonical identity of the enclosing sequential declaration.
/// * `specification` — the event list to validate.
/// * `checker` — the adapter to canonical semantic services.
#[must_use]
pub fn validate_event_specification(
    region: &HdlId,
    specification: &SequentialEventSpecification,
    checker: &impl SequentialEventChecker,
) -> SequentialEventValidation {
    let mut result = SequentialEventValidation::valid();

    for (index, event) in specification.events().iter().enumerate() {
        match checker.check_event(region, event) {
            EventSourceValidity::Valid => {}
            EventSourceValidity::Invalid => result.invalid_events.push(index),
            EventSourceValidity::Indeterminate => {
                result.indeterminate_events.push(index);
            }
        }
    }

    result
}

/// Validate one assignment belonging to a sequential construct.
///
/// This function checks that the enclosing context is sequential and then
/// delegates all target, mutability, type, and width checks to the existing
/// `HdlAssignment::validate` contract. It does not introduce separate
/// assignment rules or an invented nonblocking assignment operator.
///
/// The caller must already have resolved the assignment's target signal and
/// must ensure that the canonical l-value refers to that target.
///
/// # Errors
///
/// Returns an error if the assignment context is not sequential or if the
/// existing assignment validator rejects the assignment.
pub fn validate_sequential_assignment(
    assignment: &HdlAssignment,
    signal: &HdlSignal,
    checker: &impl AssignmentTypeChecker,
) -> Result<AssignmentValidation, SequentialError> {
    if assignment.context() != AssignmentContext::Sequential {
        return Err(SequentialError::WrongAssignmentContext {
            actual: assignment.context(),
        });
    }

    assignment
        .validate(signal, checker)
        .map_err(SequentialError::AssignmentValidation)
}

fn validate_qualifiers(
    qualifiers: &[SequentialEventQualifier],
) -> Result<(), SequentialError> {
    let mut saw_async = false;
    let mut saw_sync = false;

    for qualifier in qualifiers {
        match qualifier {
            SequentialEventQualifier::Async => {
                if saw_async {
                    return Err(SequentialError::DuplicateEventQualifier {
                        qualifier: *qualifier,
                    });
                }
                saw_async = true;
            }
            SequentialEventQualifier::Sync => {
                if saw_sync {
                    return Err(SequentialError::DuplicateEventQualifier {
                        qualifier: *qualifier,
                    });
                }
                saw_sync = true;
            }
        }
    }

    if saw_async && saw_sync {
        return Err(SequentialError::ConflictingEventQualifiers);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_no_qualifiers() {
        assert_eq!(validate_qualifiers(&[]), Ok(()));
    }

    #[test]
    fn accepts_one_async_qualifier() {
        assert_eq!(
            validate_qualifiers(&[SequentialEventQualifier::Async]),
            Ok(())
        );
    }

    #[test]
    fn accepts_one_sync_qualifier() {
        assert_eq!(
            validate_qualifiers(&[SequentialEventQualifier::Sync]),
            Ok(())
        );
    }

    #[test]
    fn rejects_duplicate_async_qualifier() {
        assert_eq!(
            validate_qualifiers(&[
                SequentialEventQualifier::Async,
                SequentialEventQualifier::Async,
            ]),
            Err(SequentialError::DuplicateEventQualifier {
                qualifier: SequentialEventQualifier::Async,
            })
        );
    }

    #[test]
    fn rejects_duplicate_sync_qualifier() {
        assert_eq!(
            validate_qualifiers(&[
                SequentialEventQualifier::Sync,
                SequentialEventQualifier::Sync,
            ]),
            Err(SequentialError::DuplicateEventQualifier {
                qualifier: SequentialEventQualifier::Sync,
            })
        );
    }

    #[test]
    fn rejects_conflicting_sync_and_async_qualifiers() {
        assert_eq!(
            validate_qualifiers(&[
                SequentialEventQualifier::Sync,
                SequentialEventQualifier::Async,
            ]),
            Err(SequentialError::ConflictingEventQualifiers)
        );
    }

    #[test]
    fn event_validation_summary_requires_complete_proof() {
        let mut result = SequentialEventValidation::valid();
        assert!(result.is_valid());

        result.indeterminate_events.push(0);
        assert!(!result.is_valid());
        assert!(result.has_indeterminate_events());
        assert!(!result.has_invalid_events());

        result.invalid_events.push(1);
        assert!(!result.is_valid());
        assert!(result.has_invalid_events());
        assert_eq!(result.invalid_events(), &[1]);
        assert_eq!(result.indeterminate_events(), &[0]);
    }
}
