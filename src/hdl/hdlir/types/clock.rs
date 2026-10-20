#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR clock types
//!
//! Path: `src/hdl/hdlir/types/clock.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: HDLIR clock-type implementation; repository integration and
//! conformance tests are required before the API can be considered stable.
//!
//! ## Purpose
//!
//! Represent logical clocks and clock domains in the hardware-domain
//! intermediate representation without coupling their meaning to a physical
//! oscillator, clock pin, clock tree, PLL, FPGA, ASIC, or vendor implementation.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/clocks.g4`.
//! - `grammar/hdl/clocking.g4`.
//! - `grammar/hdl/reset.g4`.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/types/signal.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! The grammar owns clock syntax. The canonical semantic model owns
//! source-level clock meaning. This module owns the HDLIR representation
//! of logical clock declarations and clock-domain associations.
//!
//! ## Owns
//!
//! - Logical clock declaration descriptors.
//! - Optional edge intent when explicitly supplied by a validated producer.
//! - Logical clock-domain descriptors.
//! - Explicit association between a logical clock and a clock domain.
//! - Conversion of the existing canonical `HdlClock` summary.
//!
//! ## Does not own
//!
//! - Parsing or source-level name resolution.
//! - A replacement for the canonical HDL semantic model.
//! - Physical oscillators, clock pins, clock trees, PLLs, or DLLs.
//! - Clock generation, gating, synchronization, or clock-domain-crossing
//!   implementation.
//! - Timing analysis, timing closure, synthesis, placement, or routing.
//! - Reset semantics; those belong to a separately specified reset contract.
//! - Target discovery, physical resource allocation, or vendor APIs.
//!
//! ## Scalability and determinism
//!
//! This module imposes no universal maximum on the number of clocks,
//! domains, relationships, or consumers. Collections are owned by callers
//! and grow according to available resources.
//!
//! It does not evaluate timing expressions or convert symbolic timing
//! quantities to machine integers. The existing semantic API represents
//! the period as an optional `HdlId`; that identity is preserved as an
//! opaque reference rather than being misinterpreted as a numeric duration.
//!
//! Identities are logical identifiers scoped by their owning compilation
//! or design context. They are not persistent IDs unless a separate
//! serialization contract explicitly makes them persistent.
//!
//! ## Integration
//!
//! - `types/mod.rs` should register `pub mod clock;` and export the intended
//!   public API.
//! - `types/signal.rs` continues to represent the signal category as
//!   `SignalType::Clock`; `ClockType` adds declaration-level information.
//! - `src/hdl/ids.rs` remains the owner of `HdlClockDomainId`.
//! - `lowering/from_semantic.rs` converts validated `HdlClock` values using
//!   `ClockType::from_semantic_clock`.
//! - `ir/clocks.rs` may associate clock descriptors with processes and
//!   sequential operations after its own contracts are established.
//! - `verify/clocks.rs` must check declaration references, domain membership,
//!   and any clocking rules required by the relevant dialect.
//! - `analysis/clocks.rs` owns clock relationship analysis and must not infer
//!   missing edge or timing facts from the target.
//! - Serialization must preserve declaration IDs, signal IDs, domain IDs,
//!   period-expression references, and optional edge intent.
//!
//! This module does not depend on future HDLIR graph, builder, verification,
//! analysis, or serialization modules.
//!
//! ## Completion criteria
//!
//! - Canonical identities are reused rather than duplicated.
//! - Conversion preserves every field exposed by `HdlClock`.
//! - Missing edge information remains missing rather than being guessed.
//! - Clock domains cannot be constructed with an absent logical clock ID.
//! - Physical implementation decisions remain outside this module.
//! - Focused tests and repository-wide formatting, build, test, and Clippy
//!   checks pass after integration.

use core::fmt;

use crate::frontend::semantic::domains::hdl::{HdlClock, HdlId};
use crate::hdl::ids::HdlClockDomainId;

/// The logical edge event associated with a clock.
///
/// This is declarative edge intent. It does not define the implementation
/// of a clock source, nor does it prove that a target can realize the edge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClockEdge {
    /// A transition from logical low to logical high.
    Rising,

    /// A transition from logical high to logical low.
    Falling,

    /// Either rising or falling transitions are relevant.
    ///
    /// Whether both edges are legal for a particular operation is determined
    /// by that operation's dialect contract and verification rules.
    Both,
}

impl ClockEdge {
    /// Return the stable lowercase spelling of this edge variant.
    ///
    /// These strings are suitable for diagnostics and stable interchange
    /// fields when the serialization contract adopts the same spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::Both => "both",
        }
    }
}

impl fmt::Display for ClockEdge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A target-independent descriptor for a logical HDL clock declaration.
///
/// `declaration_id` identifies the semantic clock declaration, while
/// `signal_id` identifies the logical signal representing that clock.
/// They are intentionally separate: a declaration identity is not a signal
/// identity.
///
/// `period_expression` is an opaque reference to the semantic expression
/// describing the period, when the canonical semantic model provides one.
/// It is not a numeric duration and must not be interpreted as one.
///
/// The optional edge remains `None` when the producer has not established
/// edge intent. Callers must not silently replace missing information with
/// a presumed rising edge.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClockType {
    declaration_id: HdlId,
    signal_id: HdlId,
    period_expression: Option<HdlId>,
    domain_id: Option<HdlClockDomainId>,
    edge: Option<ClockEdge>,
}

/// Errors produced while constructing or associating HDLIR clock descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClockTypeError {
    /// A clock declaration ID is empty or contains only whitespace.
    EmptyDeclarationId,

    /// A logical signal ID is empty or contains only whitespace.
    EmptySignalId,

    /// A clock-domain ID was absent or invalid.
    InvalidDomainId,

    /// The requested clock does not match the clock associated with a domain.
    ClockDomainMismatch {
        /// Expected clock declaration identity.
        expected: HdlId,

        /// Actual clock declaration identity.
        actual: HdlId,
    },
}

impl fmt::Display for ClockTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDeclarationId => {
                formatter.write_str("clock declaration identity cannot be empty")
            }
            Self::EmptySignalId => {
                formatter.write_str("clock signal identity cannot be empty")
            }
            Self::InvalidDomainId => {
                formatter.write_str("clock-domain identity is invalid")
            }
            Self::ClockDomainMismatch { expected, actual } => {
                write!(
                    formatter,
                    "clock-domain association mismatch: expected clock \
                     `{expected}`, received `{actual}`"
                )
            }
        }
    }
}

impl std::error::Error for ClockTypeError {}

impl ClockType {
    /// Construct a logical clock descriptor from validated semantic IDs.
    ///
    /// The constructor preserves symbolic period information and does not
    /// infer an edge or domain. Domain membership and edge intent can be
    /// supplied explicitly through the corresponding builder methods.
    ///
    /// # Errors
    ///
    /// Returns an error if either required identity is empty or whitespace.
    pub fn new(
        declaration_id: HdlId,
        signal_id: HdlId,
        period_expression: Option<HdlId>,
    ) -> Result<Self, ClockTypeError> {
        if declaration_id.as_str().trim().is_empty() {
            return Err(ClockTypeError::EmptyDeclarationId);
        }

        if signal_id.as_str().trim().is_empty() {
            return Err(ClockTypeError::EmptySignalId);
        }

        Ok(Self {
            declaration_id,
            signal_id,
            period_expression,
            domain_id: None,
            edge: None,
        })
    }

    /// Convert an existing canonical semantic clock summary to HDLIR.
    ///
    /// All fields exposed by `HdlClock` are retained. The semantic summary
    /// currently does not expose edge or domain membership, so those fields
    /// remain unset after conversion.
    ///
    /// The source location remains the responsibility of the canonical
    /// source-map/provenance integration layer.
    ///
    /// # Errors
    ///
    /// Returns an error if the required semantic identities violate the
    /// constructor's invariants.
    pub fn from_semantic_clock(
        clock: &HdlClock,
    ) -> Result<Self, ClockTypeError> {
        Self::new(
            clock.id.clone(),
            clock.signal.clone(),
            clock.period.clone(),
        )
    }

    /// Return the canonical semantic identity of the clock declaration.
    #[must_use]
    pub fn declaration_id(&self) -> &HdlId {
        &self.declaration_id
    }

    /// Return the identity of the logical signal representing this clock.
    #[must_use]
    pub fn signal_id(&self) -> &HdlId {
        &self.signal_id
    }

    /// Return the period-expression identity, if one is available.
    ///
    /// The returned ID is opaque. Resolving and evaluating the expression
    /// belongs to semantic analysis or a separately specified timing analysis.
    #[must_use]
    pub fn period_expression(&self) -> Option<&HdlId> {
        self.period_expression.as_ref()
    }

    /// Return the logical clock-domain identity, if assigned.
    #[must_use]
    pub const fn domain_id(&self) -> Option<HdlClockDomainId> {
        self.domain_id
    }

    /// Return the explicitly established clock edge, if known.
    #[must_use]
    pub const fn edge(&self) -> Option<ClockEdge> {
        self.edge
    }

    /// Attach explicit edge intent to this descriptor.
    ///
    /// This does not prove that every operation using this clock supports
    /// the selected edge. Such legality is checked by the operation dialect
    /// and HDLIR verification.
    #[must_use]
    pub fn with_edge(mut self, edge: ClockEdge) -> Self {
        self.edge = Some(edge);
        self
    }

    /// Associate this clock with a logical clock domain.
    ///
    /// The domain identity is a logical HDL identity, not a physical clock
    /// domain allocation. The association is declarative and does not
    /// perform clock-domain-crossing analysis.
    #[must_use]
    pub fn in_domain(mut self, domain_id: HdlClockDomainId) -> Self {
        self.domain_id = Some(domain_id);
        self
    }

    /// Return whether the descriptor has an explicit clock edge.
    #[must_use]
    pub const fn has_explicit_edge(&self) -> bool {
        self.edge.is_some()
    }

    /// Return whether the descriptor has been associated with a domain.
    #[must_use]
    pub const fn has_domain(&self) -> bool {
        self.domain_id.is_some()
    }
}

/// A logical clock-domain descriptor.
///
/// A domain associates sequential behavior with one logical clock
/// declaration. It does not describe a physical clock tree, clock source,
/// clock-generation circuit, or target-specific allocation.
///
/// Reset membership and clock relationships are intentionally not stored
/// here until their own HDLIR contracts are defined.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClockDomain {
    id: HdlClockDomainId,
    clock_declaration_id: HdlId,
}

/// Errors produced while validating a clock-domain association.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClockDomainError {
    /// The descriptor's clock identity does not match the requested clock.
    ClockMismatch {
        /// Clock identity recorded by the domain.
        domain_clock: HdlId,

        /// Clock identity supplied by the caller.
        supplied_clock: HdlId,
    },
}

impl fmt::Display for ClockDomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClockMismatch {
                domain_clock,
                supplied_clock,
            } => write!(
                formatter,
                "clock-domain mismatch: domain uses clock \
                 `{domain_clock}`, but `{supplied_clock}` was supplied"
            ),
        }
    }
}

impl std::error::Error for ClockDomainError {}

impl ClockDomain {
    /// Construct a clock domain from an allocated domain identity and a
    /// canonical clock declaration identity.
    ///
    /// The domain ID must come from the owning compilation/design context's
    /// `clock_domain_id_allocator`. This constructor does not allocate IDs
    /// or establish global uniqueness.
    ///
    /// # Errors
    ///
    /// Returns an error if the clock declaration identity is empty.
    pub fn new(
        id: HdlClockDomainId,
        clock_declaration_id: HdlId,
    ) -> Result<Self, ClockTypeError> {
        if clock_declaration_id.as_str().trim().is_empty() {
            return Err(ClockTypeError::EmptyDeclarationId);
        }

        Ok(Self {
            id,
            clock_declaration_id,
        })
    }

    /// Return the logical clock-domain identity.
    #[must_use]
    pub const fn id(&self) -> HdlClockDomainId {
        self.id
    }

    /// Return the canonical identity of the clock assigned to this domain.
    #[must_use]
    pub fn clock_declaration_id(&self) -> &HdlId {
        &self.clock_declaration_id
    }

    /// Check whether this domain is associated with the supplied clock.
    ///
    /// This checks identity equality only. It does not prove that the clock
    /// declaration exists in a design or that its timing properties are valid.
    #[must_use]
    pub fn matches_clock(&self, clock: &ClockType) -> bool {
        self.clock_declaration_id == *clock.declaration_id()
    }

    /// Require the supplied clock descriptor to match this domain's clock.
    ///
    /// # Errors
    ///
    /// Returns `ClockDomainError::ClockMismatch` when the identities differ.
    pub fn require_clock(
        &self,
        clock: &ClockType,
    ) -> Result<(), ClockDomainError> {
        if self.matches_clock(clock) {
            Ok(())
        } else {
            Err(ClockDomainError::ClockMismatch {
                domain_clock: self.clock_declaration_id.clone(),
                supplied_clock: clock.declaration_id().clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test ID must be non-empty")
    }

    fn clock() -> ClockType {
        ClockType::new(
            id("clock_decl"),
            id("clock_signal"),
            Some(id("period_expr")),
        )
        .expect("valid test clock")
    }

    #[test]
    fn clock_preserves_declaration_signal_and_period_identities() {
        let clock = clock();

        assert_eq!(clock.declaration_id().as_str(), "clock_decl");
        assert_eq!(clock.signal_id().as_str(), "clock_signal");
        assert_eq!(
            clock.period_expression().map(HdlId::as_str),
            Some("period_expr")
        );
    }

    #[test]
    fn clock_does_not_invent_edge_or_domain_information() {
        let clock = clock();

        assert_eq!(clock.edge(), None);
        assert_eq!(clock.domain_id(), None);
        assert!(!clock.has_explicit_edge());
        assert!(!clock.has_domain());
    }

    #[test]
    fn clock_can_record_explicit_edge_and_domain() {
        let domain_id =
            HdlClockDomainId::new(7).expect("non-zero domain ID");

        let clock = clock()
            .with_edge(ClockEdge::Falling)
            .in_domain(domain_id);

        assert_eq!(clock.edge(), Some(ClockEdge::Falling));
        assert_eq!(clock.domain_id(), Some(domain_id));
        assert!(clock.has_explicit_edge());
        assert!(clock.has_domain());
    }

    #[test]
    fn clock_edges_have_stable_spellings() {
        assert_eq!(ClockEdge::Rising.as_str(), "rising");
        assert_eq!(ClockEdge::Falling.as_str(), "falling");
        assert_eq!(ClockEdge::Both.as_str(), "both");
    }

    #[test]
    fn clock_domain_matches_only_its_declared_clock() {
        let domain_id =
            HdlClockDomainId::new(1).expect("non-zero domain ID");

        let domain = ClockDomain::new(
            domain_id,
            id("clock_decl"),
        )
        .expect("valid domain");

        assert!(domain.matches_clock(&clock()));
        assert!(domain.require_clock(&clock()).is_ok());
    }

    #[test]
    fn clock_domain_rejects_a_different_clock() {
        let domain_id =
            HdlClockDomainId::new(2).expect("non-zero domain ID");

        let domain = ClockDomain::new(
            domain_id,
            id("expected_clock"),
        )
        .expect("valid domain");

        let other_clock = ClockType::new(
            id("different_clock"),
            id("different_signal"),
            None,
        )
        .expect("valid clock");

        assert!(!domain.matches_clock(&other_clock));
        assert!(matches!(
            domain.require_clock(&other_clock),
            Err(ClockDomainError::ClockMismatch { .. })
        ));
    }

    #[test]
    fn empty_clock_signal_identity_is_rejected() {
        let result = ClockType::new(
            id("clock_decl"),
            id("   "),
            None,
        );

        assert_eq!(result, Err(ClockTypeError::EmptySignalId));
    }

    #[test]
    fn empty_domain_clock_identity_is_rejected() {
        let domain_id =
            HdlClockDomainId::new(3).expect("non-zero domain ID");

        let result = ClockDomain::new(domain_id, id(" "));

        assert_eq!(result, Err(ClockTypeError::EmptyDeclarationId));
    }

    #[test]
    fn semantic_clock_conversion_preserves_available_fields() {
        let semantic_clock = HdlClock {
            id: id("semantic_clock"),
            signal: id("semantic_signal"),
            period: Some(id("semantic_period")),
            location: None,
        };

        let converted = ClockType::from_semantic_clock(&semantic_clock)
            .expect("valid semantic clock");

        assert_eq!(
            converted.declaration_id().as_str(),
            "semantic_clock"
        );
        assert_eq!(
            converted.signal_id().as_str(),
            "semantic_signal"
        );
        assert_eq!(
            converted.period_expression().map(HdlId::as_str),
            Some("semantic_period")
        );

        // HdlClock currently has no edge/domain fields; conversion must not
        // guess them.
        assert_eq!(converted.edge(), None);
        assert_eq!(converted.domain_id(), None);
    }
}