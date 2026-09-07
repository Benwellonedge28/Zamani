//! Zamani quantum resilience — semantic verification.
//!
//! Path:
//!     src/quantum/resilience/verification/semantic.rs
//!
//! This module is the semantic-integrity boundary of resilience.
//!
//! Its responsibility is to answer one question:
//!
//!     Did adaptation/recovery change WHAT the Zamani program means?
//!
//! It deliberately does not answer:
//!
//!     Which physical qubits were used?
//!     When was an operation executed?
//!     Which route was selected?
//!     Which provider executed the program?
//!     Which pulse implementation was selected?
//!     Which QEC decoder was used?
//!
//! Those concerns belong to their owning subsystems.
//!
//! # Canonical IR
//!
//! The authoritative quantum IR remains:
//!
//!     crate::quantum::ir
//!
//! Logical qubit identity is always:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Operation identity is always the canonical:
//!
//!     crate::quantum::ir::core::identity::OperationId
//!
//! This module does not define replacement quantum IR types.
//!
//! # Write once, scale everywhere
//!
//! The verifier has no architectural limit on:
//!
//! - qubits;
//! - operations;
//! - circuit depth;
//! - classical values;
//! - parameters;
//! - dependencies;
//! - machine size;
//! - topology;
//! - backend count.
//!
//! The implementation uses indexed comparison rather than an all-pairs
//! comparison matrix.
//!
//! For N operations, Q quantum references, C classical references, P
//! parameters, and D dependency references, comparison is approximately:
//!
//!     O(N + Q + C + P + D)
//!
//! plus the cost of caller-provided containers.
//!
//! No timeline proportional to execution duration is constructed.
//!
//! # Semantic versus physical identity
//!
//! A valid adaptation may transform:
//!
//!     logical q0 -> physical q17
//!
//! without changing:
//!
//!     X(q0)
//!
//! Therefore physical qubit IDs must never be inserted into the semantic
//! fingerprint merely because routing changed.
//!
//! # Dynamic circuits
//!
//! Dynamic classical conditions are represented by a canonical condition
//! fingerprint supplied by the IR adapter.
//!
//! Example:
//!
//!     measure(q0) -> c0
//!     if c0 == 1:
//!         X(q1)
//!
//! The verifier checks preservation of the condition, but does not implement
//! classical control itself.
//!
//! # Parameters
//!
//! Floating-point equivalence is intentionally not approximated here.
//!
//! An upstream canonical-IR adapter must convert parameters to deterministic
//! semantic fingerprints. If approximate equivalence is required, it must be
//! an explicit higher-level verification policy rather than an implicit
//! tolerance hidden in this module.
//!
//! # Recovery safety
//!
//! A matching operation ID alone is insufficient.
//!
//! The verifier also compares all semantic fields. This prevents corrupted
//! recovery/adaptation from retaining an operation ID while changing its
//! meaning.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! A canonical-IR adapter should convert source and adapted representations
//! into `SemanticProgram` values.
//!
//! The resilience verification pipeline should compose this verifier with:
//!
//! - invariant verification;
//! - result verification;
//! - confidence verification;
//! - provenance verification.
//!
//! This module does not depend on those modules so it can be completed first.
//!
//! The intended flow is:
//!
//!     canonical IR
//!         |
//!         v
//!     semantic adapter
//!         |
//!         +----------------------+
//!         |                      |
//!         v                      v
//!     source view            adapted view
//!         |                      |
//!         +----------+-----------+
//!                    |
//!                    v
//!             SemanticVerifier
//!                    |
//!                    v
//!              verification
//!
//! `routing`, `scheduling`, `hardware`, `QEC`, `optimization`, and runtime
//! implementations can evolve independently as long as their adapters
//! preserve this contract.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration controlling semantic verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticVerificationConfig {
    /// Stop after the first violation.
    pub fail_fast: bool,

    /// Require canonical operation identities.
    ///
    /// This should normally remain enabled in production.
    pub require_operation_identity: bool,

    /// Require identical adapter-provided operation ordering.
    ///
    /// This is normally disabled because legal scheduling may reorder
    /// independent operations.
    pub require_sequence_order: bool,

    /// Optional bound on retained diagnostics.
    ///
    /// This is a protection against pathological diagnostic amplification.
    /// It is not a quantum-machine-size limit.
    pub max_diagnostics: Option<usize>,
}

impl Default for SemanticVerificationConfig {
    fn default() -> Self {
        Self {
            fail_fast: false,
            require_operation_identity: true,
            require_sequence_order: false,
            max_diagnostics: None,
        }
    }
}

// ============================================================================
// Semantic parameters
// ============================================================================

/// Canonical semantic parameter representation.
///
/// Numerical canonicalization belongs to the IR adapter. This verifier
/// performs exact equality on the resulting representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticParameter {
    /// Signed integer.
    Integer(i128),

    /// Unsigned integer.
    Unsigned(u128),

    /// Canonical real-number representation.
    Real(String),

    /// Canonical symbolic identifier.
    Symbol(String),

    /// Canonical expression fingerprint.
    Expression(String),

    /// Extension/dialect-defined semantic fingerprint.
    Opaque(String),
}

impl SemanticParameter {
    /// Creates a canonical real parameter.
    #[must_use]
    pub fn real(value: impl Into<String>) -> Self {
        Self::Real(value.into())
    }

    /// Creates a symbolic parameter.
    #[must_use]
    pub fn symbol(value: impl Into<String>) -> Self {
        Self::Symbol(value.into())
    }

    /// Creates an expression fingerprint.
    #[must_use]
    pub fn expression(value: impl Into<String>) -> Self {
        Self::Expression(value.into())
    }

    /// Creates an opaque semantic fingerprint.
    #[must_use]
    pub fn opaque(value: impl Into<String>) -> Self {
        Self::Opaque(value.into())
    }
}

impl fmt::Display for SemanticParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Unsigned(value) => write!(f, "{value}"),
            Self::Real(value)
            | Self::Symbol(value)
            | Self::Expression(value)
            | Self::Opaque(value) => f.write_str(value),
        }
    }
}

// ============================================================================
// Classical semantic identity
// ============================================================================

/// Canonical adapter-level identity for a classical semantic value.
///
/// The canonical classical IR owns the actual classical identifier type.
/// This wrapper prevents this verifier from becoming coupled to one particular
/// classical representation while still requiring deterministic equality.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalValueId(String);

impl ClassicalValueId {
    /// Creates an adapter-provided canonical classical identity.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClassicalValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// Semantic conditions
// ============================================================================

/// Canonical fingerprint for a dynamic control condition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticCondition(String);

impl SemanticCondition {
    /// Creates a canonical condition fingerprint.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the canonical fingerprint.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SemanticCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// Semantic attributes
// ============================================================================

/// A deterministic computation-affecting semantic attribute.
///
/// Hardware-only metadata must not be placed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticAttribute {
    key: String,
    value: String,
}

impl SemanticAttribute {
    /// Creates a semantic attribute.
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// Semantic operation
// ============================================================================

/// Adapter-level semantic fingerprint for one canonical operation.
///
/// This is NOT a replacement for `quantum::ir::QuantumOperation`.
///
/// It is a compact verification view of the portions of that operation that
/// determine computation semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOperation {
    operation_id: OperationId,
    kind: String,
    qubits: Vec<QubitId>,
    classical_inputs: Vec<ClassicalValueId>,
    classical_outputs: Vec<ClassicalValueId>,
    condition: Option<SemanticCondition>,
    parameters: Vec<SemanticParameter>,
    attributes: Vec<SemanticAttribute>,
    dependencies: Vec<OperationId>,
}

impl SemanticOperation {
    /// Creates a semantic operation fingerprint.
    #[must_use]
    pub fn new(
        operation_id: OperationId,
        kind: impl Into<String>,
        qubits: Vec<QubitId>,
        classical_inputs: Vec<ClassicalValueId>,
        classical_outputs: Vec<ClassicalValueId>,
        condition: Option<SemanticCondition>,
        parameters: Vec<SemanticParameter>,
        attributes: Vec<SemanticAttribute>,
        dependencies: Vec<OperationId>,
    ) -> Self {
        Self {
            operation_id,
            kind: kind.into(),
            qubits,
            classical_inputs,
            classical_outputs,
            condition,
            parameters,
            attributes,
            dependencies,
        }
    }

    /// Canonicalizes collections whose ordering is not semantic.
    ///
    /// Quantum operand order is deliberately preserved because control/target
    /// ordering can change computation semantics.
    pub fn canonicalize(&mut self) {
        self.classical_inputs.sort();
        self.classical_outputs.sort();
        self.parameters.sort();
        self.attributes.sort();
        self.attributes.dedup();
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the semantic operation kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns logical qubit operands in semantic order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns classical inputs.
    #[must_use]
    pub fn classical_inputs(&self) -> &[ClassicalValueId] {
        &self.classical_inputs
    }

    /// Returns classical outputs.
    #[must_use]
    pub fn classical_outputs(&self) -> &[ClassicalValueId] {
        &self.classical_outputs
    }

    /// Returns the dynamic control condition.
    #[must_use]
    pub fn condition(&self) -> Option<&SemanticCondition> {
        self.condition.as_ref()
    }

    /// Returns semantic parameters.
    #[must_use]
    pub fn parameters(&self) -> &[SemanticParameter] {
        &self.parameters
    }

    /// Returns semantic attributes.
    #[must_use]
    pub fn attributes(&self) -> &[SemanticAttribute] {
        &self.attributes
    }

    /// Returns semantic dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[OperationId] {
        &self.dependencies
    }
}

// ============================================================================
// Semantic program
// ============================================================================

/// Ordered semantic view produced by a canonical-IR adapter.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SemanticProgram {
    operations: Vec<SemanticOperation>,
}

impl SemanticProgram {
    /// Creates a semantic program view.
    #[must_use]
    pub fn new(operations: Vec<SemanticOperation>) -> Self {
        Self { operations }
    }

    /// Creates an empty semantic program.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns operations in adapter-provided order.
    #[must_use]
    pub fn operations(&self) -> &[SemanticOperation] {
        &self.operations
    }

    /// Returns the operation count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether there are no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// ============================================================================
// Violations
// ============================================================================

/// A semantic verification violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticViolation {
    /// Source and adapted operation counts differ.
    OperationCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// An operation identity occurs more than once.
    DuplicateOperationId {
        operation_id: OperationId,
    },

    /// A source operation does not exist in the adapted program.
    MissingOperation {
        operation_id: OperationId,
    },

    /// An adapted operation does not exist in the source program.
    UnexpectedOperation {
        operation_id: OperationId,
    },

    /// Operation kind changed.
    KindMismatch {
        operation_id: OperationId,
        expected: String,
        actual: String,
    },

    /// Logical quantum operands changed.
    QubitOperandsMismatch {
        operation_id: OperationId,
        expected: Vec<QubitId>,
        actual: Vec<QubitId>,
    },

    /// Classical inputs changed.
    ClassicalInputsMismatch {
        operation_id: OperationId,
        expected: Vec<ClassicalValueId>,
        actual: Vec<ClassicalValueId>,
    },

    /// Classical outputs changed.
    ClassicalOutputsMismatch {
        operation_id: OperationId,
        expected: Vec<ClassicalValueId>,
        actual: Vec<ClassicalValueId>,
    },

    /// Dynamic control condition changed.
    ConditionMismatch {
        operation_id: OperationId,
        expected: Option<SemanticCondition>,
        actual: Option<SemanticCondition>,
    },

    /// Parameters changed.
    ParameterMismatch {
        operation_id: OperationId,
        expected: Vec<SemanticParameter>,
        actual: Vec<SemanticParameter>,
    },

    /// Computation-affecting attributes changed.
    AttributeMismatch {
        operation_id: OperationId,
        expected: Vec<SemanticAttribute>,
        actual: Vec<SemanticAttribute>,
    },

    /// Semantic dependencies changed.
    DependencyMismatch {
        operation_id: OperationId,
        expected: Vec<OperationId>,
        actual: Vec<OperationId>,
    },

    /// Adapter-provided semantic ordering changed.
    SequenceOrderMismatch {
        position: usize,
        expected: OperationId,
        actual: OperationId,
    },

    /// The configured diagnostic capacity was reached.
    DiagnosticLimitReached {
        limit: usize,
    },
}

impl fmt::Display for SemanticViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperationCountMismatch { expected, actual } => {
                write!(
                    f,
                    "semantic operation count mismatch: expected {expected}, got {actual}"
                )
            }
            Self::DuplicateOperationId { operation_id } => {
                write!(f, "duplicate semantic operation identity: {operation_id:?}")
            }
            Self::MissingOperation { operation_id } => {
                write!(f, "missing semantic operation: {operation_id:?}")
            }
            Self::UnexpectedOperation { operation_id } => {
                write!(f, "unexpected semantic operation: {operation_id:?}")
            }
            Self::KindMismatch {
                operation_id,
                expected,
                actual,
            } => write!(
                f,
                "operation {operation_id:?} kind mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::QubitOperandsMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} logical qubit operands changed")
            }
            Self::ClassicalInputsMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} classical inputs changed")
            }
            Self::ClassicalOutputsMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} classical outputs changed")
            }
            Self::ConditionMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} control condition changed")
            }
            Self::ParameterMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} parameters changed")
            }
            Self::AttributeMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} semantic attributes changed")
            }
            Self::DependencyMismatch { operation_id, .. } => {
                write!(f, "operation {operation_id:?} semantic dependencies changed")
            }
            Self::SequenceOrderMismatch {
                position,
                expected,
                actual,
            } => write!(
                f,
                "semantic sequence mismatch at position {position}: expected {expected:?}, got {actual:?}"
            ),
            Self::DiagnosticLimitReached { limit } => {
                write!(f, "semantic diagnostic limit reached: {limit}")
            }
        }
    }
}

// ============================================================================
// Report
// ============================================================================

/// Immutable result of semantic verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticVerificationReport {
    valid: bool,
    compared_operations: usize,
    diagnostics: Vec<SemanticViolation>,
    diagnostics_truncated: bool,
}

impl SemanticVerificationReport {
    fn new(compared_operations: usize) -> Self {
        Self {
            valid: true,
            compared_operations,
            diagnostics: Vec::new(),
            diagnostics_truncated: false,
        }
    }

    /// Returns whether semantic equivalence was established.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns the number of source operations considered.
    #[must_use]
    pub fn compared_operations(&self) -> usize {
        self.compared_operations
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[SemanticViolation] {
        &self.diagnostics
    }

    /// Returns whether diagnostic collection was truncated.
    #[must_use]
    pub fn diagnostics_truncated(&self) -> bool {
        self.diagnostics_truncated
    }

    /// Returns the first diagnostic.
    #[must_use]
    pub fn first_diagnostic(&self) -> Option<&SemanticViolation> {
        self.diagnostics.first()
    }
}

// ============================================================================
// Verifier
// ============================================================================

/// Production semantic verifier.
#[derive(Debug, Clone, Copy)]
pub struct SemanticVerifier {
    config: SemanticVerificationConfig,
}

impl SemanticVerifier {
    /// Creates a semantic verifier.
    #[must_use]
    pub const fn new(config: SemanticVerificationConfig) -> Self {
        Self { config }
    }

    /// Returns the active configuration.
    #[must_use]
    pub const fn config(&self) -> SemanticVerificationConfig {
        self.config
    }

    /// Verifies semantic equivalence.
    ///
    /// When operation identity is enabled, operations are indexed by their
    /// canonical IDs. This permits legal reordering of independent operations.
    #[must_use]
    pub fn verify(
        &self,
        expected: &SemanticProgram,
        actual: &SemanticProgram,
    ) -> SemanticVerificationReport {
        let mut report = SemanticVerificationReport::new(expected.len());

        if expected.len() != actual.len() {
            if !self.push(
                &mut report,
                SemanticViolation::OperationCountMismatch {
                    expected: expected.len(),
                    actual: actual.len(),
                },
            ) {
                return report;
            }
        }

        if self.config.require_operation_identity {
            self.verify_by_identity(expected, actual, &mut report);
        } else {
            self.verify_positionally(expected, actual, &mut report);
        }

        if self.config.require_sequence_order {
            self.verify_sequence_order(expected, actual, &mut report);
        }

        report
    }

    fn verify_by_identity(
        &self,
        expected: &SemanticProgram,
        actual: &SemanticProgram,
        report: &mut SemanticVerificationReport,
    ) {
        let Some(expected_index) =
            Self::build_index(expected, self.config.max_diagnostics, report)
        else {
            return;
        };

        let Some(actual_index) =
            Self::build_index(actual, self.config.max_diagnostics, report)
        else {
            return;
        };

        for (operation_id, expected_operation) in &expected_index {
            match actual_index.get(operation_id) {
                Some(actual_operation) => {
                    self.compare_operation(
                        expected_operation,
                        actual_operation,
                        report,
                    );

                    if self.should_stop(report) {
                        return;
                    }
                }
                None => {
                    if !self.push(
                        report,
                        SemanticViolation::MissingOperation {
                            operation_id: *operation_id,
                        },
                    ) {
                        return;
                    }
                }
            }
        }

        for operation_id in actual_index.keys() {
            if !expected_index.contains_key(operation_id)
                && !self.push(
                    report,
                    SemanticViolation::UnexpectedOperation {
                        operation_id: *operation_id,
                    },
                )
            {
                return;
            }
        }
    }

    fn verify_positionally(
        &self,
        expected: &SemanticProgram,
        actual: &SemanticProgram,
        report: &mut SemanticVerificationReport,
    ) {
        let common_len = expected.len().min(actual.len());

        for index in 0..common_len {
            self.compare_operation(
                &expected.operations()[index],
                &actual.operations()[index],
                report,
            );

            if self.should_stop(report) {
                return;
            }
        }
    }

    fn verify_sequence_order(
        &self,
        expected: &SemanticProgram,
        actual: &SemanticProgram,
        report: &mut SemanticVerificationReport,
    ) {
        let common_len = expected.len().min(actual.len());

        for position in 0..common_len {
            let expected_id = expected.operations()[position].operation_id();
            let actual_id = actual.operations()[position].operation_id();

            if expected_id != actual_id
                && !self.push(
                    report,
                    SemanticViolation::SequenceOrderMismatch {
                        position,
                        expected: expected_id,
                        actual: actual_id,
                    },
                )
            {
                return;
            }

            if self.should_stop(report) {
                return;
            }
        }
    }

    fn compare_operation(
        &self,
        expected: &SemanticOperation,
        actual: &SemanticOperation,
        report: &mut SemanticVerificationReport,
    ) {
        if expected.kind != actual.kind
            && !self.push(
                report,
                SemanticViolation::KindMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.kind.clone(),
                    actual: actual.kind.clone(),
                },
            )
        {
            return;
        }

        if expected.qubits != actual.qubits
            && !self.push(
                report,
                SemanticViolation::QubitOperandsMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.qubits.clone(),
                    actual: actual.qubits.clone(),
                },
            )
        {
            return;
        }

        if expected.classical_inputs != actual.classical_inputs
            && !self.push(
                report,
                SemanticViolation::ClassicalInputsMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.classical_inputs.clone(),
                    actual: actual.classical_inputs.clone(),
                },
            )
        {
            return;
        }

        if expected.classical_outputs != actual.classical_outputs
            && !self.push(
                report,
                SemanticViolation::ClassicalOutputsMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.classical_outputs.clone(),
                    actual: actual.classical_outputs.clone(),
                },
            )
        {
            return;
        }

        if expected.condition != actual.condition
            && !self.push(
                report,
                SemanticViolation::ConditionMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.condition.clone(),
                    actual: actual.condition.clone(),
                },
            )
        {
            return;
        }

        if expected.parameters != actual.parameters
            && !self.push(
                report,
                SemanticViolation::ParameterMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.parameters.clone(),
                    actual: actual.parameters.clone(),
                },
            )
        {
            return;
        }

        if expected.attributes != actual.attributes
            && !self.push(
                report,
                SemanticViolation::AttributeMismatch {
                    operation_id: expected.operation_id,
                    expected: expected.attributes.clone(),
                    actual: actual.attributes.clone(),
                },
            )
        {
            return;
        }

        if expected.dependencies != actual.dependencies {
            let mut expected_dependencies = expected.dependencies.clone();
            let mut actual_dependencies = actual.dependencies.clone();

            expected_dependencies.sort();
            actual_dependencies.sort();

            if expected_dependencies != actual_dependencies
                && !self.push(
                    report,
                    SemanticViolation::DependencyMismatch {
                        operation_id: expected.operation_id,
                        expected: expected.dependencies.clone(),
                        actual: actual.dependencies.clone(),
                    },
                )
            {
                return;
            }
        }
    }

    fn build_index<'a>(
        program: &'a SemanticProgram,
        max_diagnostics: Option<usize>,
        report: &mut SemanticVerificationReport,
    ) -> Option<BTreeMap<OperationId, &'a SemanticOperation>> {
        let mut index = BTreeMap::new();

        for operation in program.operations() {
            let operation_id = operation.operation_id();

            if index.insert(operation_id, operation).is_some() {
                report.valid = false;

                match max_diagnostics {
                    Some(limit) if report.diagnostics.len() >= limit => {
                        report.diagnostics_truncated = true;
                        return None;
                    }
                    _ => {
                        report
                            .diagnostics
                            .push(SemanticViolation::DuplicateOperationId {
                                operation_id,
                            });
                    }
                }
            }
        }

        Some(index)
    }

    fn push(
        &self,
        report: &mut SemanticVerificationReport,
        violation: SemanticViolation,
    ) -> bool {
        report.valid = false;

        if self.config.fail_fast {
            report.diagnostics.push(violation);
            return false;
        }

        match self.config.max_diagnostics {
            Some(limit) if report.diagnostics.len() >= limit => {
                report.diagnostics_truncated = true;
                false
            }
            _ => {
                report.diagnostics.push(violation);
                true
            }
        }
    }

    fn should_stop(&self, report: &SemanticVerificationReport) -> bool {
        self.config.fail_fast || report.diagnostics_truncated
    }
}

impl Default for SemanticVerifier {
    fn default() -> Self {
        Self::new(SemanticVerificationConfig::default())
    }
}

// ============================================================================
// Public convenience functions
// ============================================================================

/// Verifies semantic equivalence using production defaults.
#[must_use]
pub fn verify_semantics(
    expected: &SemanticProgram,
    actual: &SemanticProgram,
) -> SemanticVerificationReport {
    SemanticVerifier::default().verify(expected, actual)
}

/// Verifies semantic equivalence using an explicit configuration.
#[must_use]
pub fn verify_semantics_with_config(
    expected: &SemanticProgram,
    actual: &SemanticProgram,
    config: SemanticVerificationConfig,
) -> SemanticVerificationReport {
    SemanticVerifier::new(config).verify(expected, actual)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(
        id: usize,
        kind: &str,
        qubits: &[usize],
    ) -> SemanticOperation {
        SemanticOperation::new(
            OperationId::new(id),
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    #[test]
    fn empty_programs_are_equal() {
        let report =
            verify_semantics(&SemanticProgram::empty(), &SemanticProgram::empty());

        assert!(report.is_valid());
        assert!(report.diagnostics().is_empty());
    }

    #[test]
    fn identical_programs_are_equal() {
        let expected =
            SemanticProgram::new(vec![operation(0, "x", &[0])]);

        let actual =
            SemanticProgram::new(vec![operation(0, "x", &[0])]);

        assert!(verify_semantics(&expected, &actual).is_valid());
    }

    #[test]
    fn logical_qubit_changes_are_rejected() {
        let expected =
            SemanticProgram::new(vec![operation(0, "x", &[0])]);

        let actual =
            SemanticProgram::new(vec![operation(0, "x", &[1])]);

        let report = verify_semantics(&expected, &actual);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::QubitOperandsMismatch { .. }
            )
        }));
    }

    #[test]
    fn operation_kind_changes_are_rejected() {
        let expected =
            SemanticProgram::new(vec![operation(0, "x", &[0])]);

        let actual =
            SemanticProgram::new(vec![operation(0, "h", &[0])]);

        let report = verify_semantics(&expected, &actual);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::KindMismatch { .. }
            )
        }));
    }

    #[test]
    fn independent_operations_may_be_reordered() {
        let expected = SemanticProgram::new(vec![
            operation(0, "x", &[0]),
            operation(1, "h", &[1]),
        ]);

        let actual = SemanticProgram::new(vec![
            operation(1, "h", &[1]),
            operation(0, "x", &[0]),
        ]);

        let report = verify_semantics(&expected, &actual);

        assert!(report.is_valid());
    }

    #[test]
    fn sequence_order_can_be_required() {
        let expected = SemanticProgram::new(vec![
            operation(0, "x", &[0]),
            operation(1, "h", &[1]),
        ]);

        let actual = SemanticProgram::new(vec![
            operation(1, "h", &[1]),
            operation(0, "x", &[0]),
        ]);

        let config = SemanticVerificationConfig {
            require_sequence_order: true,
            ..SemanticVerificationConfig::default()
        };

        let report =
            verify_semantics_with_config(&expected, &actual, config);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::SequenceOrderMismatch { .. }
            )
        }));
    }

    #[test]
    fn duplicate_operation_ids_are_rejected() {
        let program = SemanticProgram::new(vec![
            operation(0, "x", &[0]),
            operation(0, "h", &[0]),
        ]);

        let report = verify_semantics(&program, &program);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::DuplicateOperationId { .. }
            )
        }));
    }

    #[test]
    fn parameter_changes_are_rejected() {
        let expected = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "rx",
                vec![QubitId::new(0)],
                Vec::new(),
                Vec::new(),
                None,
                vec![SemanticParameter::real("0.5")],
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let actual = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "rx",
                vec![QubitId::new(0)],
                Vec::new(),
                Vec::new(),
                None,
                vec![SemanticParameter::real("0.6")],
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let report = verify_semantics(&expected, &actual);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::ParameterMismatch { .. }
            )
        }));
    }

    #[test]
    fn conditions_are_semantic() {
        let expected = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "x",
                vec![QubitId::new(0)],
                vec![ClassicalValueId::new("c0")],
                Vec::new(),
                Some(SemanticCondition::new("c0 == 1")),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let actual = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "x",
                vec![QubitId::new(0)],
                vec![ClassicalValueId::new("c0")],
                Vec::new(),
                Some(SemanticCondition::new("c0 == 0")),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let report = verify_semantics(&expected, &actual);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::ConditionMismatch { .. }
            )
        }));
    }

    #[test]
    fn classical_outputs_are_semantic() {
        let expected = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "measure",
                vec![QubitId::new(0)],
                Vec::new(),
                vec![ClassicalValueId::new("c0")],
                None,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let actual = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(0),
                "measure",
                vec![QubitId::new(0)],
                Vec::new(),
                vec![ClassicalValueId::new("c1")],
                None,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ]);

        let report = verify_semantics(&expected, &actual);

        assert!(!report.is_valid());

        assert!(report.diagnostics().iter().any(|diagnostic| {
            matches!(
                diagnostic,
                SemanticViolation::ClassicalOutputsMismatch { .. }
            )
        }));
    }

    #[test]
    fn dependencies_are_semantic_but_order_is_not() {
        let expected = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(1),
                "x",
                vec![QubitId::new(0)],
                Vec::new(),
                Vec::new(),
                None,
                Vec::new(),
                Vec::new(),
                vec![OperationId::new(0), OperationId::new(2)],
            ),
        ]);

        let actual = SemanticProgram::new(vec![
            SemanticOperation::new(
                OperationId::new(1),
                "x",
                vec![QubitId::new(0)],
                Vec::new(),
                Vec::new(),
                None,
                Vec::new(),
                Vec::new(),
                vec![OperationId::new(2), OperationId::new(0)],
            ),
        ]);

        let report = verify_semantics(&expected, &actual);

        assert!(report.is_valid());
    }

    #[test]
    fn fail_fast_limits_diagnostics() {
        let expected = SemanticProgram::new(vec![
            operation(0, "x", &[0]),
        ]);

        let actual = SemanticProgram::new(vec![
            operation(0, "h", &[1]),
        ]);

        let config = SemanticVerificationConfig {
            fail_fast: true,
            ..SemanticVerificationConfig::default()
        };

        let report =
            verify_semantics_with_config(&expected, &actual, config);

        assert!(!report.is_valid());
        assert_eq!(report.diagnostics().len(), 1);
    }

    #[test]
    fn bounded_diagnostics_do_not_create_unbounded_storage() {
        let expected = SemanticProgram::new(vec![
            operation(0, "x", &[0]),
        ]);

        let actual = SemanticProgram::new(vec![
            operation(0, "h", &[1]),
        ]);

        let config = SemanticVerificationConfig {
            max_diagnostics: Some(1),
            ..SemanticVerificationConfig::default()
        };

        let report =
            verify_semantics_with_config(&expected, &actual, config);

        assert!(!report.is_valid());
        assert_eq!(report.diagnostics().len(), 1);
    }

    #[test]
    fn canonicalization_does_not_reorder_quantum_operands() {
        let mut operation = SemanticOperation::new(
            OperationId::new(0),
            "cx",
            vec![QubitId::new(1), QubitId::new(0)],
            vec![
                ClassicalValueId::new("c1"),
                ClassicalValueId::new("c0"),
            ],
            Vec::new(),
            None,
            vec![
                SemanticParameter::symbol("theta"),
                SemanticParameter::real("1"),
            ],
            vec![
                SemanticAttribute::new("b", "2"),
                SemanticAttribute::new("a", "1"),
            ],
            Vec::new(),
        );

        operation.canonicalize();

        assert_eq!(
            operation.qubits(),
            &[QubitId::new(1), QubitId::new(0)]
        );

        assert_eq!(
            operation.classical_inputs()[0].as_str(),
            "c0"
        );
    }
}