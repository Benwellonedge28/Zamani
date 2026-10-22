//! Zamani Hybrid Computing — Canonical Quantum IR Integration.
//!
//! Path: `src/hybrid/integration/quantum.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Define the integration boundary between the canonical quantum IR and the
//! hybrid-composition subsystem.
//!
//! The canonical quantum IR remains owned by `crate::quantum::ir`. This
//! module must adapt its existing semantic representations into hybrid
//! composition without defining a second quantum IR.
//!
//! # Architectural boundaries
//!
//! This module does not:
//!
//! - define quantum gates, measurements, qubits, or quantum instructions;
//! - replace `crate::quantum::ir`;
//! - create a second classical or quantum control-flow representation;
//! - perform quantum routing, scheduling, QEC, calibration, or pulse synthesis;
//! - discover hardware or allocate physical resources;
//! - execute quantum programs or communicate with vendor SDKs;
//! - claim that a target can execute an operation merely because it is
//!   represented in the canonical IR.
//!
//! # Integration contract
//!
//! A concrete bridge must preserve canonical quantum operation identity,
//! operand and result relationships, measurement semantics, effects,
//! dependencies, source provenance, and all semantically relevant control
//! relationships.
//!
//! Unknown or extension-defined quantum operations must not be silently
//! rewritten, discarded, or assigned guessed semantics.
//!
//! # Scalability
//!
//! This module introduces no architectural limit on the number of quantum
//! operations, values, logical qubits, domains, or cross-domain dependencies.
//! Concrete resource and work limits belong to explicit operational policy.
//!
//! No finite implementation can promise infinite physical execution. The
//! portability guarantee is that this adapter does not impose arbitrary
//! machine-size ceilings on valid representations.
//!
//! # Integration status
//!
//! The bridge traits in this file define an integration seam. They do not,
//! by themselves, constitute an implementation of the canonical IR bridge.
//! The repository's concrete quantum IR types and hybrid graph APIs must be
//! connected through an implementation of these traits before this adapter
//! can be considered complete.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

/// Identifies the semantic role of a quantum-to-hybrid relationship.
///
/// This enum describes relationships at the hybrid boundary. It does not
/// define quantum operations or replace the canonical quantum control-flow
/// representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum QuantumBoundaryRole {
    /// A classical value is consumed by a quantum computation.
    ClassicalInput,

    /// A quantum computation produces a result.
    QuantumResult,

    /// A measurement produces classical information.
    MeasurementResult,

    /// A classical predicate controls subsequent quantum computation.
    ClassicalFeedback,

    /// Completion of a quantum computation is a dependency of another step.
    CompletionDependency,

    /// An explicit ordering requirement crosses the domain boundary.
    OrderingDependency,

    /// A relationship whose semantics are defined by an extension contract.
    ExtensionDefined,
}

/// Identifies the strength of a relationship crossing the domain boundary.
///
/// A dependency is not interchangeable with a scheduling hint. The concrete
/// bridge must preserve the canonical semantics associated with each role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantumDependencyStrength {
    /// The relationship is necessary to preserve program semantics.
    Required,

    /// The relationship is advisory and must not change program meaning.
    Advisory,
}

/// A stable error category for failures at the quantum integration boundary.
///
/// These categories are adapter-level classifications. When the repository's
/// canonical `HybridError` API is available, integration code should map
/// these errors into that type rather than establish a competing error system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum QuantumIntegrationErrorKind {
    /// The source representation could not be inspected or resolved.
    InvalidSource,

    /// Required canonical semantic information is missing.
    MissingSemanticFact,

    /// The bridge encountered an unsupported or incompatible contract.
    ContractMismatch,

    /// A required dependency cannot be represented faithfully.
    DependencyPreservation,

    /// The bridge cannot establish measurement-result availability.
    MeasurementAvailability,

    /// An extension-defined operation lacks a valid integration contract.
    UnsupportedExtension,

    /// An explicit operational budget has been exhausted.
    BudgetExhausted,

    /// A canonical source or graph identifier could not be resolved.
    UnresolvedReference,

    /// A canonical representation failed its own validation.
    InvalidCanonicalRepresentation,
}

/// A structured quantum integration error.
///
/// The message is explanatory text. Callers must classify errors using
/// `kind`, not by parsing the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumIntegrationError {
    kind: QuantumIntegrationErrorKind,
    message: String,
    context: Option<String>,
}

impl QuantumIntegrationError {
    /// Construct an integration error.
    #[must_use]
    pub fn new(
        kind: QuantumIntegrationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            context: None,
        }
    }

    /// Attach contextual information without changing the error category.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Return the stable error category.
    #[must_use]
    pub const fn kind(&self) -> QuantumIntegrationErrorKind {
        self.kind
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return optional additional context.
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

impl fmt::Display for QuantumIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.message)?;

        if let Some(context) = &self.context {
            write!(formatter, " (context: {context})")?;
        }

        Ok(())
    }
}

impl std::error::Error for QuantumIntegrationError {}

/// A borrowed reference to a canonical quantum computation.
///
/// Implementations must use the repository's existing quantum IR types.
/// They must not convert the computation into a second, adapter-owned quantum
/// instruction list.
///
/// The associated type intentionally leaves the canonical IR representation
/// under the ownership of `crate::quantum::ir`.
pub trait CanonicalQuantumProgram {
    /// The canonical program, region, or computation reference.
    ///
    /// This must be a type owned by, or directly referencing, the canonical
    /// quantum IR.
    type Program: ?Sized;

    /// Return the canonical quantum program.
    fn program(&self) -> &Self::Program;
}

/// Provides read-only access to a canonical quantum IR representation.
///
/// This trait is an integration seam, not a replacement for the quantum IR.
/// Its implementation belongs at the boundary where the repository's actual
/// canonical quantum IR types and the hybrid graph APIs are both available.
///
/// The trait deliberately does not prescribe a closed list of quantum
/// operations. Extension-defined operations remain governed by their
/// canonical dialect and semantic contracts.
pub trait QuantumIrBridge {
    /// Canonical program or region type owned by the quantum IR.
    type Program: ?Sized;

    /// Canonical quantum operation reference.
    type Operation: ?Sized;

    /// Canonical quantum value reference.
    type Value: ?Sized;

    /// Canonical logical-qubit identity type.
    ///
    /// Implementations must reuse `quantum::ir::qubit::QubitId` or the
    /// appropriate canonical identity supplied by the IR. They must not
    /// introduce a competing qubit identity type.
    type QubitId: ?Sized;

    /// Canonical source/provenance reference, when available.
    type Provenance: ?Sized;

    /// Return the canonical program or region.
    fn program(&self) -> &Self::Program;

    /// Visit operations in a deterministic canonical order.
    ///
    /// The callback receives borrowed canonical operation references.
    /// Implementations must not fabricate operations or silently omit
    /// operations that cannot be interpreted by the hybrid adapter.
    ///
    /// The order must be documented by the concrete implementation. It must
    /// not be presented as an execution schedule unless the canonical source
    /// explicitly defines that ordering.
    fn visit_operations(
        &self,
        visitor: &mut dyn FnMut(&Self::Operation) -> Result<(), QuantumIntegrationError>,
    ) -> Result<(), QuantumIntegrationError>;

    /// Visit values that participate in quantum/classical boundaries.
    ///
    /// This method exposes canonical values; it does not require that every
    /// quantum state be copied into a classical in-memory value.
    fn visit_boundary_values(
        &self,
        visitor: &mut dyn FnMut(&Self::Value) -> Result<(), QuantumIntegrationError>,
    ) -> Result<(), QuantumIntegrationError>;

    /// Return the canonical logical-qubit identity associated with an
    /// operation operand, when that operand denotes a logical qubit.
    ///
    /// `None` means that the operand is not a qubit operand or that the
    /// operation's canonical contract represents it through another form.
    /// It must not be used to hide a missing required semantic fact.
    fn logical_qubit(
        &self,
        operation: &Self::Operation,
        operand_index: usize,
    ) -> Result<Option<&Self::QubitId>, QuantumIntegrationError>;

    /// Return canonical provenance for an operation, when available.
    fn provenance(
        &self,
        operation: &Self::Operation,
    ) -> Option<&Self::Provenance>;

    /// Validate the canonical program using the quantum IR's authoritative
    /// validation facilities.
    ///
    /// The implementation must delegate to those facilities rather than
    /// reimplement whole-IR quantum validation in this adapter.
    fn validate_canonical_program(
        &self,
    ) -> Result<(), QuantumIntegrationError>;
}

/// A destination capable of receiving semantic relationships from a quantum
/// IR bridge.
///
/// A concrete implementation must connect these calls to the existing
/// hybrid graph, dependency, and data-contract APIs. It must not create a
/// second graph or assign new identities to canonical quantum operations
/// without an explicit mapping contract.
pub trait HybridQuantumSink<B: QuantumIrBridge> {
    /// Import a canonical quantum operation into the hybrid representation.
    ///
    /// `operation` remains owned by the canonical IR. The sink should retain
    /// a stable reference or an explicit canonical identity according to the
    /// repository's graph contract.
    fn import_operation(
        &mut self,
        bridge: &B,
        operation: &B::Operation,
    ) -> Result<(), QuantumIntegrationError>;

    /// Import a canonical value that crosses a domain boundary.
    ///
    /// Type, ownership, effects, provenance, and availability semantics must
    /// remain intact. The sink must not assume that the value is serializable,
    /// copyable, or immediately available.
    fn import_boundary_value(
        &mut self,
        bridge: &B,
        value: &B::Value,
    ) -> Result<(), QuantumIntegrationError>;

    /// Record a semantic dependency between canonical entities.
    ///
    /// Implementations must preserve the distinction between data, control,
    /// completion, and ordering dependencies. If the graph contract supports
    /// extension-defined dependency kinds, those must remain explicit.
    fn record_dependency(
        &mut self,
        role: QuantumBoundaryRole,
        strength: QuantumDependencyStrength,
    ) -> Result<(), QuantumIntegrationError>;

    /// Finalize and validate the imported relationships.
    ///
    /// This method must delegate structural validation to the authoritative
    /// hybrid graph verifier. It must not silently repair malformed
    /// relationships or reinterpret their semantics.
    fn validate_import(
        &mut self,
    ) -> Result<(), QuantumIntegrationError>;
}

/// Import canonical quantum semantics into an existing hybrid sink.
///
/// This function intentionally delegates all representation-specific work to
/// `QuantumIrBridge` and `HybridQuantumSink`. It does not manufacture a
/// quantum IR or a hybrid graph.
///
/// # Guarantees
///
/// - Validates the canonical quantum program before importing it.
/// - Visits canonical operations and boundary values by borrowing them.
/// - Propagates the first explicit integration failure.
/// - Delegates final graph validation to the sink.
/// - Does not silently discard unsupported operations.
/// - Does not impose an architectural operation-count or qubit-count limit.
///
/// # Operational policy
///
/// Work and memory budgets must be enforced by the concrete bridge, sink, or
/// their explicitly configured owning subsystems. This function does not
/// invent hidden global ceilings.
///
/// # Important
///
/// This function cannot, on its own, prove semantic equivalence between two
/// quantum implementations. That proof depends on the canonical IR contract,
/// the concrete bridge, the hybrid graph verifier, and downstream lowering
/// validation.
pub fn import_quantum<B, S>(
    bridge: &B,
    sink: &mut S,
) -> Result<(), QuantumIntegrationError>
where
    B: QuantumIrBridge,
    S: HybridQuantumSink<B>,
{
    bridge.validate_canonical_program()?;

    bridge.visit_operations(&mut |operation| {
        sink.import_operation(bridge, operation)
    })?;

    bridge.visit_boundary_values(&mut |value| {
        sink.import_boundary_value(bridge, value)
    })?;

    sink.validate_import()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_retains_structured_category_and_context() {
        let error = QuantumIntegrationError::new(
            QuantumIntegrationErrorKind::MissingSemanticFact,
            "measurement availability is not established",
        )
        .with_context("quantum/classical boundary");

        assert_eq!(
            error.kind(),
            QuantumIntegrationErrorKind::MissingSemanticFact,
        );
        assert_eq!(
            error.message(),
            "measurement availability is not established",
        );
        assert_eq!(
            error.context(),
            Some("quantum/classical boundary"),
        );
    }

    #[test]
    fn boundary_roles_remain_distinct() {
        assert_ne!(
            QuantumBoundaryRole::MeasurementResult,
            QuantumBoundaryRole::CompletionDependency,
        );

        assert_ne!(
            QuantumBoundaryRole::ClassicalInput,
            QuantumBoundaryRole::ClassicalFeedback,
        );
    }

    #[test]
    fn dependency_strength_is_explicit() {
        assert_ne!(
            QuantumDependencyStrength::Required,
            QuantumDependencyStrength::Advisory,
        );
    }
}