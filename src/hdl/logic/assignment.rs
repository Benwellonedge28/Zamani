
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Assignment Semantics
//!
//! Path: `src/hdl/logic/assignment.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the target-independent contract for HDL assignments, including
//! target identity, assignment context, mutability checks, expression-type
//! compatibility, and conservative driver-conflict detection.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/combinational.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/signals.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/expressions.rs`
//! - `src/hdl/logic/expression.rs`
//! - `src/hdl/design/signal.rs`
//!
//! Grammar owns assignment syntax. The canonical AST owns expression and
//! l-value nodes. Semantic analysis owns name resolution, expression typing,
//! width compatibility, and source-language legality. This module consumes
//! those contracts and does not replace them.
//!
//! ## Important grammar rule
//!
//! The current combinational and sequential grammars both use the canonical
//! `ASSIGN` token. Their enclosing constructs determine the assignment
//! context. This module does not invent a separate nonblocking operator.
//!
//! ## Owns
//!
//! - The HDL assignment request and context.
//! - Local assignment-target and mutability checks.
//! - The semantic type-checking extension point.
//! - Explicit compatible, incompatible, and indeterminate outcomes.
//! - A deterministic registry of assignment drivers.
//!
//! ## Does not own
//!
//! - Parsing or a replacement AST.
//! - Name resolution or a second type/width system.
//! - Expression evaluation or symbolic-width resolution.
//! - Full l-value decomposition, member/index resolution, or alias analysis.
//! - Process scheduling, event scheduling, or simulation time.
//! - Combinational completeness, dependency cycles, or full connectivity.
//! - Physical wires, register allocation, synthesis, placement, or routing.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The AST adapter supplies a canonical target l-value `NodeId`, its resolved
//! `HdlSignal`, and an `HdlExpressionUse` whose role is `AssignmentValue`.
//! The semantic adapter implements `AssignmentTypeChecker` using the existing
//! AST graph and semantic services. The driver-analysis stage registers the
//! owning process or continuous-driver identity for each target.
//!
//! The target signal is supplied by reference, so this module does not
//! maintain a second signal table or perform name lookup.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on assignments, signals, widths,
//! drivers, or design size. Ordered dynamic collections provide deterministic
//! results. Operational resource limits belong to the caller.
//!
//! Distinct drivers of the same signal are conservatively treated as a
//! conflict. This module does not pretend to prove that partial selections
//! are disjoint: precise bit/member/index overlap analysis belongs to the
//! canonical l-value and connectivity analysis integration.
//!
//! ## Freeze criteria
//!
//! - The frontend adapter resolves canonical target and expression nodes.
//! - The semantic adapter establishes type and width compatibility.
//! - Assignment context is derived from the parsed enclosing construct.
//! - Driver identity represents the actual owning process or continuous
//!   driver, not a source-order index or a physical resource identifier.
//! - Inconclusive checks never become successful proofs.
//! - No HDLIR implementation is introduced.

use core::fmt;
use std::collections::BTreeMap;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::{
    HdlId, HdlValueType,
};
use crate::hdl::design::signal::{
    HdlSignal, SignalDeclarationKind, SignalMutability,
};
use crate::hdl::expressions::{
    HdlExpressionRole, HdlExpressionUse,
};

/// Version of the public assignment contract.
///
/// This is independent of the language version and HDLIR schema version.
pub const HDL_ASSIGNMENT_CONTRACT_VERSION: u16 = 1;

/// Assignment context determined by the enclosing HDL construct.
///
/// The context does not introduce an operator or override grammar semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum AssignmentContext {
    /// Assignment in a combinational hardware construct.
    ///
    /// Completeness, combinational-cycle detection, and inferred-storage
    /// analysis are performed by their respective analysis modules.
    Combinational,

    /// Assignment in a sequential hardware construct.
    ///
    /// The enclosing sequential construct determines clocked/state-update
    /// semantics. This variant does not imply a particular physical register.
    Sequential,

    /// Initialization permitted by the source-language declaration rules.
    ///
    /// This context is not a general exemption from immutability checks:
    /// the semantic adapter must establish that initialization is legal.
    Initialization,

    /// An assignment-like operation supplied by an explicitly defined
    /// extension or another validated HDL context.
    Extension,
}

/// Classification of the expression-to-target type relationship.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AssignmentCompatibility {
    /// The semantic checker established that the assignment is compatible.
    Compatible,

    /// The semantic checker established a type or width incompatibility.
    Incompatible,

    /// Compatibility could not be established with available information.
    ///
    /// Examples include unresolved symbolic widths and unavailable semantic
    /// facts. This result must not be treated as successful validation.
    Indeterminate,
}

/// Result of checking an assignment's target and expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssignmentValidation {
    /// The canonical identity of the target declaration.
    pub target: HdlId,

    /// The semantic compatibility result.
    pub compatibility: AssignmentCompatibility,

    /// Whether the target is legally mutable in the requested context.
    pub target_is_writable: bool,
}

/// Errors detected by this module's local contract checks.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AssignmentError {
    /// The expression reference does not have the required assignment-value
    /// role.
    WrongExpressionRole,

    /// The supplied target declaration is an alias rather than a directly
    /// assignable value declaration.
    AliasTargetNotResolved,

    /// The target is immutable in the requested assignment context.
    ImmutableTarget {
        /// Target identity.
        target: HdlId,
        /// Requested assignment context.
        context: AssignmentContext,
    },

    /// The type checker established a type or width incompatibility.
    IncompatibleTypes {
        /// Target identity.
        target: HdlId,
    },

    /// Compatibility could not be established.
    IndeterminateCompatibility {
        /// Target identity.
        target: HdlId,
    },

    /// The same driver identity was registered with contradictory kinds.
    ConflictingDriverKind {
        /// Target signal identity.
        target: HdlId,
        /// Driver identity.
        driver: HdlId,
    },
}

impl fmt::Display for AssignmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongExpressionRole => formatter.write_str(
                "assignment expression does not have the AssignmentValue role",
            ),
            Self::AliasTargetNotResolved => formatter.write_str(
                "assignment target alias must be resolved before validation",
            ),
            Self::ImmutableTarget { target, context } => {
                write!(
                    formatter,
                    "target `{target}` is not writable in context {context:?}"
                )
            }
            Self::IncompatibleTypes { target } => {
                write!(
                    formatter,
                    "assignment expression is incompatible with target `{target}`"
                )
            }
            Self::IndeterminateCompatibility { target } => {
                write!(
                    formatter,
                    "assignment compatibility for target `{target}` \
                     could not be established"
                )
            }
            Self::ConflictingDriverKind { target, driver } => {
                write!(
                    formatter,
                    "driver `{driver}` has conflicting driver kinds for \
                     target `{target}`"
                )
            }
        }
    }
}

impl std::error::Error for AssignmentError {}

/// Semantic integration point for assignment type checking.
///
/// Implementations must query the canonical AST and semantic services.
/// They must not infer expression types from source spelling, silently
/// truncate values, or assume unresolved widths are compatible.
///
/// This trait deliberately accepts the canonical expression node identity
/// instead of defining a second expression representation.
pub trait AssignmentTypeChecker {
    /// Establish compatibility between a target type and a canonical
    /// assignment-value expression.
    ///
    /// # Errors
    ///
    /// Return an error only when the semantic service itself cannot complete
    /// its operation. A source-level mismatch should normally be represented
    /// by `Ok(AssignmentCompatibility::Incompatible)`, and unavailable
    /// semantic information by `Ok(AssignmentCompatibility::Indeterminate)`.
    fn check_assignment(
        &self,
        target_type: &HdlValueType,
        expression: &NodeId,
    ) -> Result<AssignmentCompatibility, AssignmentError>;
}

/// A validated reference to an assignment's source-level target and value.
///
/// This is a semantic request, not an AST node or an executable assignment.
/// The canonical l-value node is retained for later l-value, projection, and
/// connectivity analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlAssignment {
    target: HdlId,
    target_lvalue: NodeId,
    value: HdlExpressionUse,
    context: AssignmentContext,
}

impl HdlAssignment {
    /// Construct an assignment request from canonical frontend references.
    ///
    /// This checks only the expression-use role. Target existence, l-value
    /// legality, name resolution, and expression-node validity remain the
    /// responsibility of the AST and semantic adapters.
    ///
    /// # Errors
    ///
    /// Returns `WrongExpressionRole` if the expression reference is not
    /// explicitly marked as an assignment value.
    pub fn new(
        target: HdlId,
        target_lvalue: NodeId,
        value: HdlExpressionUse,
        context: AssignmentContext,
    ) -> Result<Self, AssignmentError> {
        if value.role() != &HdlExpressionRole::AssignmentValue {
            return Err(AssignmentError::WrongExpressionRole);
        }

        Ok(Self {
            target,
            target_lvalue,
            value,
            context,
        })
    }

    /// Return the target declaration identity.
    #[must_use]
    pub fn target(&self) -> &HdlId {
        &self.target
    }

    /// Return the canonical AST node representing the target l-value.
    #[must_use]
    pub fn target_lvalue(&self) -> &NodeId {
        &self.target_lvalue
    }

    /// Return the canonical expression reference for the assigned value.
    #[must_use]
    pub fn value(&self) -> &HdlExpressionUse {
        &self.value
    }

    /// Return the enclosing assignment context.
    #[must_use]
    pub const fn context(&self) -> AssignmentContext {
        self.context
    }

    /// Validate the assignment against its already-resolved target signal.
    ///
    /// The caller must first resolve `self.target()` and verify that the
    /// canonical target l-value actually denotes that declaration. This
    /// method does not perform name resolution or projection analysis.
    ///
    /// An alias must be resolved to its canonical writable target before
    /// calling this method. That avoids accidentally creating independent
    /// driver identities for two names referring to the same signal.
    ///
    /// # Errors
    ///
    /// Returns an error if the signal identity does not match, the target is
    /// not directly assignable, the target is immutable in this context, or
    /// compatibility is invalid or indeterminate. A semantic-service failure
    /// is propagated unchanged.
    pub fn validate(
        &self,
        signal: &HdlSignal,
        checker: &impl AssignmentTypeChecker,
    ) -> Result<AssignmentValidation, AssignmentError> {
        if signal.id() != &self.target {
            return Err(AssignmentError::AliasTargetNotResolved);
        }

        if signal.kind() != SignalDeclarationKind::Value {
            return Err(AssignmentError::AliasTargetNotResolved);
        }

        let writable = match signal.mutability() {
            SignalMutability::Mutable => true,
            SignalMutability::Unspecified => false,
            SignalMutability::Immutable => {
                // Initialization is not automatically authorized merely
                // because the caller selected the initialization context.
                // The declaration/semantic adapter must encode legal
                // initialization as a permitted semantic operation.
                false
            }
        };

        if !writable {
            return Err(AssignmentError::ImmutableTarget {
                target: self.target.clone(),
                context: self.context,
            });
        }

        let compatibility = checker.check_assignment(
            signal.value_type(),
            self.value.expression(),
        )?;

        match compatibility {
            AssignmentCompatibility::Compatible => Ok(AssignmentValidation {
                target: self.target.clone(),
                compatibility,
                target_is_writable: true,
            }),
            AssignmentCompatibility::Incompatible => {
                Err(AssignmentError::IncompatibleTypes {
                    target: self.target.clone(),
                })
            }
            AssignmentCompatibility::Indeterminate => {
                Err(AssignmentError::IndeterminateCompatibility {
                    target: self.target.clone(),
                })
            }
        }
    }
}

/// Category of an entity that drives a logical signal.
///
/// These are source-level driver categories, not physical hardware resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum DriverKind {
    /// A continuous or concurrent assignment driver.
    Continuous,

    /// A combinational process or construct.
    CombinationalProcess,

    /// A sequential process or construct.
    SequentialProcess,

    /// A driver defined by a registered HDL extension.
    Extension,
}

/// One driver entry in the registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverEntry {
    target: HdlId,
    driver: HdlId,
    kind: DriverKind,
}

impl DriverEntry {
    /// Return the driven signal identity.
    #[must_use]
    pub fn target(&self) -> &HdlId {
        &self.target
    }

    /// Return the owning driver identity.
    #[must_use]
    pub fn driver(&self) -> &HdlId {
        &self.driver
    }

    /// Return the driver's category.
    #[must_use]
    pub const fn kind(&self) -> DriverKind {
        self.kind
    }
}

/// A deterministic registry for signal-driver relationships.
///
/// Driver identity should represent the owning process or continuous driver.
/// Multiple assignments within one process may register the same driver
/// repeatedly without being mistaken for independent drivers.
///
/// The registry deliberately treats distinct drivers of one signal as
/// potentially conflicting. It does not claim that separate bit slices or
/// aggregate members are disjoint; precise overlap checks require resolved
/// l-value projections and belong to connectivity analysis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DriverRegistry {
    by_target: BTreeMap<HdlId, BTreeMap<HdlId, DriverKind>>,
}

impl DriverRegistry {
    /// Create an empty driver registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a driver for a target signal.
    ///
    /// Re-registering the same target, driver, and kind is idempotent.
    /// Re-registering the same driver with another kind is rejected.
    ///
    /// # Errors
    ///
    /// Returns `ConflictingDriverKind` when one driver identity is assigned
    /// contradictory categories for the same target.
    pub fn register(
        &mut self,
        target: HdlId,
        driver: HdlId,
        kind: DriverKind,
    ) -> Result<(), AssignmentError> {
        let drivers = self.by_target.entry(target.clone()).or_default();

        if let Some(existing) = drivers.get(&driver) {
            if *existing != kind {
                return Err(AssignmentError::ConflictingDriverKind {
                    target,
                    driver,
                });
            }

            return Ok(());
        }

        drivers.insert(driver, kind);
        Ok(())
    }

    /// Return all registered drivers for a target in deterministic order.
    #[must_use]
    pub fn drivers_for(&self, target: &HdlId) -> Vec<DriverEntry> {
        self.by_target
            .get(target)
            .map(|drivers| {
                drivers
                    .iter()
                    .map(|(driver, kind)| DriverEntry {
                        target: target.clone(),
                        driver: driver.clone(),
                        kind: *kind,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Return whether a target has more than one distinct registered driver.
    ///
    /// This is a conservative conflict indicator, not a complete language
    /// legality proof. A language rule or connectivity analysis may need to
    /// distinguish legal resolved nets, mutually exclusive drivers, or
    /// disjoint target projections.
    #[must_use]
    pub fn has_multiple_drivers(&self, target: &HdlId) -> bool {
        self.by_target
            .get(target)
            .is_some_and(|drivers| drivers.len() > 1)
    }

    /// Return targets with more than one distinct registered driver.
    ///
    /// The returned identities are sorted deterministically.
    #[must_use]
    pub fn conflicting_targets(&self) -> Vec<HdlId> {
        self.by_target
            .iter()
            .filter(|(_, drivers)| drivers.len() > 1)
            .map(|(target, _)| target.clone())
            .collect()
    }

    /// Return the number of targets with registered drivers.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.by_target.len()
    }

    /// Return whether no drivers have been registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_target.is_empty()
    }

    /// Remove all driver registrations.
    pub fn clear(&mut self) {
        self.by_target.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AssignmentError, DriverKind, DriverRegistry,
    };
    use crate::frontend::semantic::domains::hdl::HdlId;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifiers are non-empty")
    }

    #[test]
    fn one_process_can_register_multiple_assignments_to_one_signal() {
        let mut registry = DriverRegistry::new();
        let target = id("signal.result");
        let process = id("process.compute");

        registry
            .register(
                target.clone(),
                process.clone(),
                DriverKind::CombinationalProcess,
            )
            .expect("first registration succeeds");

        registry
            .register(
                target.clone(),
                process,
                DriverKind::CombinationalProcess,
            )
            .expect("duplicate registration is idempotent");

        assert!(!registry.has_multiple_drivers(&target));
        assert_eq!(registry.drivers_for(&target).len(), 1);
    }

    #[test]
    fn distinct_processes_are_reported_as_potential_conflicts() {
        let mut registry = DriverRegistry::new();
        let target = id("signal.result");

        registry
            .register(
                target.clone(),
                id("process.left"),
                DriverKind::CombinationalProcess,
            )
            .expect("first driver registers");

        registry
            .register(
                target.clone(),
                id("process.right"),
                DriverKind::SequentialProcess,
            )
            .expect("second driver registers");

        assert!(registry.has_multiple_drivers(&target));
        assert_eq!(registry.conflicting_targets(), vec![target]);
    }

    #[test]
    fn a_driver_cannot_change_kind_for_the_same_target() {
        let mut registry = DriverRegistry::new();
        let target = id("signal.result");
        let driver = id("process.compute");

        registry
            .register(
                target.clone(),
                driver.clone(),
                DriverKind::CombinationalProcess,
            )
            .expect("initial registration succeeds");

        let result = registry.register(
            target.clone(),
            driver.clone(),
            DriverKind::SequentialProcess,
        );

        assert_eq!(
            result,
            Err(AssignmentError::ConflictingDriverKind {
                target,
                driver,
            })
        );
    }

    #[test]
    fn an_empty_registry_has_no_conflicts() {
        let registry = DriverRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.target_count(), 0);
        assert!(registry.conflicting_targets().is_empty());
    }
}
