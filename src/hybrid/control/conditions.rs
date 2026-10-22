#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Conditional Execution Requirements
//!
//! Path: `src/hybrid/control/conditions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! Represent the requirement that a hybrid graph node executes only when a
//! referenced canonical predicate has a specified Boolean outcome.
//!
//! This module provides a bridge between canonical predicate references and
//! hybrid execution requirements. It does not parse, evaluate, type-check,
//! simplify, or otherwise reinterpret predicate expressions.
//!
//! ## Ownership
//!
//! This module owns:
//! - The expected outcome of a conditional execution requirement.
//! - The association between a target hybrid node and a canonical predicate.
//! - Structural validation of conditional execution requirements.
//! - An adapter contract for resolving predicates through the canonical
//!   semantic or IR subsystem.
//!
//! This module does not own:
//! - Expression syntax or expression AST nodes.
//! - Name resolution, type inference, or Boolean type checking.
//! - Predicate evaluation or expression simplification.
//! - Graph nodes, graph edges, or dependency identity.
//! - Runtime predicate values or execution state.
//! - Quantum measurement semantics or classical control-flow semantics.
//! - Hardware selection, scheduling, routing, or resource allocation.
//!
//! ## Canonical integration
//!
//! - `graph/model.rs` owns `HybridGraph` and `CanonicalIrRef`.
//! - `graph/edges.rs` owns dependency edges, including `EdgeKind::Control`.
//! - `ids.rs` owns graph-scoped `HybridNodeId` values.
//! - `errors.rs` owns structured hybrid errors.
//! - The frontend owns the canonical expression and AST representation.
//! - The semantic layer owns predicate resolution and Boolean type checking.
//! - Compiler and runtime adapters preserve the requirement when realizing
//!   conditional execution.
//! - `quantum::ir` remains the canonical quantum semantic boundary.
//!
//! A condition reference is not proof that the predicate exists, is Boolean,
//! has been evaluated, or is available before the target operation executes.
//! Those properties require validation by the canonical semantic resolver
//! and preservation of the relevant dependencies by the graph/compiler/runtime.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on graph nodes, predicates, domains, or
//! conditional requirements. The data structures in this module represent
//! one requirement at a time and do not impose a global registry or count.
//!
//! Actual memory, address-space, and execution limits remain operational
//! constraints. A caller may apply an explicit operational budget through
//! the owning subsystem.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.

use serde::{Deserialize, Serialize};

use crate::hybrid::errors::HybridError;
use crate::hybrid::graph::model::{CanonicalIrRef, HybridGraph};
use crate::hybrid::ids::HybridNodeId;

/// The Boolean outcome required for a conditional operation to execute.
///
/// This describes the expected result of an already-defined canonical
/// predicate. It does not represent a predicate expression or evaluate one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PredicateOutcome {
    /// The target may execute when the canonical predicate evaluates to true.
    True,

    /// The target may execute when the canonical predicate evaluates to false.
    False,
}

impl PredicateOutcome {
    /// Return the required Boolean value.
    ///
    /// This method exposes the requirement as data. It does not evaluate
    /// the predicate or establish that a value is available.
    #[must_use]
    pub const fn expected_value(self) -> bool {
        matches!(self, Self::True)
    }
}

/// A conditional execution requirement for one hybrid graph node.
///
/// `predicate` references an entity in a canonical representation, such as
/// the canonical semantic model or an authoritative domain IR. Its `entity`
/// component must be interpreted by the owning subsystem; this module does
/// not assume that it contains source text or a serialized expression.
///
/// The target node and predicate reference are intentionally kept separate:
/// the hybrid layer associates the predicate with the computation whose
/// execution it constrains without copying either computation or expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ConditionalExecutionRequirementWire")]
pub struct ConditionalExecutionRequirement {
    target: HybridNodeId,
    predicate: CanonicalIrRef,
    outcome: PredicateOutcome,
}

/// Deserialization representation for a conditional requirement.
///
/// Conversion through `TryFrom` ensures that local predicate-reference
/// invariants are checked when a requirement is deserialized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ConditionalExecutionRequirementWire {
    target: HybridNodeId,
    predicate: CanonicalIrRef,
    outcome: PredicateOutcome,
}

impl TryFrom<ConditionalExecutionRequirementWire>
    for ConditionalExecutionRequirement
{
    type Error = HybridError;

    fn try_from(
        wire: ConditionalExecutionRequirementWire,
    ) -> Result<Self, Self::Error> {
        Self::new(wire.target, wire.predicate, wire.outcome)
    }
}

impl ConditionalExecutionRequirement {
    /// Construct a conditional execution requirement.
    ///
    /// This checks the local structure of the canonical predicate reference.
    /// It cannot establish that the referenced entity exists or has Boolean
    /// type; those checks belong to the canonical semantic resolver.
    pub fn new(
        target: HybridNodeId,
        predicate: CanonicalIrRef,
        outcome: PredicateOutcome,
    ) -> Result<Self, HybridError> {
        predicate
            .validate()
            .map_err(|error| error.in_component("hybrid conditional requirement"))?;

        Ok(Self {
            target,
            predicate,
            outcome,
        })
    }

    /// Return the node whose execution is constrained.
    #[must_use]
    pub const fn target(&self) -> HybridNodeId {
        self.target
    }

    /// Return the referenced canonical predicate.
    #[must_use]
    pub const fn predicate(&self) -> &CanonicalIrRef {
        &self.predicate
    }

    /// Return the outcome required for execution.
    #[must_use]
    pub const fn outcome(&self) -> PredicateOutcome {
        self.outcome
    }

    /// Return the Boolean value required by this condition.
    #[must_use]
    pub const fn expected_value(&self) -> bool {
        self.outcome.expected_value()
    }

    /// Validate the requirement's graph-local references.
    ///
    /// This verifies that the target belongs to `graph` and exists in that
    /// graph, and that the predicate reference is locally well-formed.
    ///
    /// It deliberately does not resolve the predicate, type-check it,
    /// evaluate it, or prove that a control/data dependency makes its value
    /// available before execution.
    pub fn validate_against_graph(
        &self,
        graph: &HybridGraph,
    ) -> Result<(), HybridError> {
        if self.target.graph() != graph.id() {
            return Err(HybridError::invalid_reference(format!(
                "conditional requirement target {} belongs to graph {}, \
                 not graph {}",
                self.target,
                self.target.graph(),
                graph.id(),
            ))
            .in_component("hybrid conditional requirement"));
        }

        if graph.node(self.target).is_none() {
            return Err(HybridError::invalid_reference(format!(
                "conditional requirement references missing target node {}",
                self.target,
            ))
            .in_component("hybrid conditional requirement"));
        }

        self.predicate
            .validate()
            .map_err(|error| error.in_component("hybrid conditional requirement"))?;

        Ok(())
    }

    /// Validate the graph reference and resolve the predicate through the
    /// canonical semantic or IR subsystem.
    ///
    /// The resolver must establish that the reference identifies a valid
    /// predicate with the Boolean semantics required by the source language.
    /// It must not evaluate the predicate merely to validate this contract.
    pub fn validate_with<R>(
        &self,
        graph: &HybridGraph,
        resolver: &R,
    ) -> Result<(), HybridError>
    where
        R: CanonicalPredicateResolver + ?Sized,
    {
        self.validate_against_graph(graph)?;

        resolver
            .validate_boolean_predicate(&self.predicate)
            .map_err(|error| error.in_component("hybrid conditional requirement"))
    }
}

/// Adapter contract for resolving a condition against the canonical
/// semantic model or the authoritative representation that owns it.
///
/// Implementations belong in the relevant integration/semantic or
/// domain-adapter layer. This trait does not define a new expression model,
/// type system, or evaluator.
///
/// A successful result means that the reference resolves to a valid
/// predicate whose canonical type and semantics are suitable for a Boolean
/// execution condition. It does not mean the predicate has been evaluated
/// or that its runtime value is currently available.
pub trait CanonicalPredicateResolver {
    /// Resolve and validate a canonical Boolean predicate reference.
    fn validate_boolean_predicate(
        &self,
        predicate: &CanonicalIrRef,
    ) -> Result<(), HybridError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hybrid::ids::{
        GraphScopedId,
        HybridGraphId,
        HybridNodeId,
        IdComponent,
    };

    fn test_target() -> HybridNodeId {
        let graph = HybridGraphId::new(1)
            .expect("non-zero graph identity must be valid");
        let local = IdComponent::new(1)
            .expect("non-zero node identity must be valid");

        HybridNodeId::new(GraphScopedId::new(graph, local))
    }

    fn test_predicate() -> CanonicalIrRef {
        CanonicalIrRef::new("semantic.model", "predicate:ready")
            .expect("predicate reference must be structurally valid")
    }

    #[test]
    fn preserves_target_predicate_and_outcome() {
        let target = test_target();
        let predicate = test_predicate();

        let requirement = ConditionalExecutionRequirement::new(
            target,
            predicate.clone(),
            PredicateOutcome::True,
        )
        .expect("valid conditional requirement must be accepted");

        assert_eq!(requirement.target(), target);
        assert_eq!(requirement.predicate(), &predicate);
        assert_eq!(requirement.outcome(), PredicateOutcome::True);
        assert!(requirement.expected_value());
    }

    #[test]
    fn represents_false_without_rewriting_the_predicate() {
        let predicate = test_predicate();

        let requirement = ConditionalExecutionRequirement::new(
            test_target(),
            predicate.clone(),
            PredicateOutcome::False,
        )
        .expect("valid conditional requirement must be accepted");

        assert_eq!(requirement.predicate(), &predicate);
        assert_eq!(requirement.outcome(), PredicateOutcome::False);
        assert!(!requirement.expected_value());
    }

    #[test]
    fn exposes_outcomes_as_plain_boolean_requirements() {
        assert!(PredicateOutcome::True.expected_value());
        assert!(!PredicateOutcome::False.expected_value());
    }

    #[test]
    fn rejects_empty_canonical_predicate_reference() {
        let result = CanonicalIrRef::new("", "predicate:ready");

        assert!(result.is_err());
    }
}