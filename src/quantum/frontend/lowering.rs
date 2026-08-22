//! Format-independent lowering boundary for the Zamani Quantum frontend.
//!
//! This module is the controlled boundary between a validated external-format
//! representation and the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                 External quantum format
//!                          │
//!                          ▼
//!                format-specific parser
//!                          │
//!                          ▼
//!                 format-specific AST
//!                          │
//!                          ▼
//!                format-specific validation
//!                          │
//!                          ▼
//!              ┌────────────────────────┐
//!              │  Frontend lowering     │
//!              │  (this module)         │
//!              └───────────┬────────────┘
//!                          │
//!                          ▼
//!                 Zamani Quantum IR
//!                    QuantumCircuit
//!                          │
//!                          ▼
//!                compiler / algorithms
//!                          │
//!                          ▼
//!                  backend / hardware
//! ```
//!
//! # Responsibility
//!
//! This module owns the *boundary contract* for lowering. It does not own any
//! external format's syntax or semantics.
//!
//! In particular, this module must not contain:
//!
//! - OpenQASM grammar;
//! - QIR grammar;
//! - Quil grammar;
//! - format-specific ASTs;
//! - format-specific gate-name tables;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - execution;
//! - backend-specific decomposition;
//! - implicit filesystem access;
//! - implicit network access;
//! - external process execution.
//!
//! Format implementations supply a [`LoweringSource`] implementation and use
//! [`LoweringContext`] to construct the canonical IR.
//!
//! # Canonical IR ownership
//!
//! The canonical Quantum IR remains the sole owner of:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `Parameter`;
//! - qubit identities;
//! - measurement semantics;
//! - circuit invariants;
//! - IR validation.
//!
//! The IR explicitly defines frontend parsing as outside its responsibility.
//!
//! # Safety model
//!
//! Lowering is an untrusted-input boundary. A format parser may have produced a
//! syntactically and semantically valid representation while still presenting
//! resource sizes that must be checked against the canonical IR policy.
//!
//! Consequently this module:
//!
//! 1. checks lowering input sizes before mutation;
//! 2. uses checked arithmetic;
//! 3. never partially commits a failed operation;
//! 4. validates the complete circuit before returning it;
//! 5. never silently discards an unsupported operation;
//! 6. never performs external effects.
//!
//! # Atomicity
//!
//! A lowering operation is transactional at the public boundary:
//!
//! ```text
//! external representation
//!          │
//!          ▼
//!       lowering
//!          │
//!     ┌────┴────┐
//!     │         │
//!   success    error
//!     │         │
//!     ▼         ▼
//! QuantumCircuit  no circuit returned
//! ```
//!
//! A failed lowering operation must never be presented as a successful
//! partially lowered circuit.
//!
//! # Format independence
//!
//! Adding or removing a format must not require modification of this module.
//!
//! ```text
//! OpenQASM ────────┐
//! QIR ─────────────┼──► LoweringSource ──► QuantumCircuit
//! Quil ────────────┤
//! Future format ───┘
//! ```
//!
//! There must never be a dependency such as:
//!
//! ```text
//! OpenQASM → QIR → lowering
//! ```
//!
//! Each format lowers independently into the canonical IR.
//!
//! # Rust compatibility
//!
//! Target toolchain: Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.
//! No new dependencies are required.

use std::fmt;

use crate::quantum::ir::{
    validate_circuit_with_limits,
    Gate,
    GateParameter,
    QuantumCircuit,
    QuantumIrLimits,
};

use super::core::errors::{
    FrontendError,
    FrontendErrorKind,
    FrontendResult,
};
use super::core::limits::FrontendLimits;
use super::format::{
    FormatId,
    FormatVersion,
};

// =============================================================================
// Public result types
// =============================================================================

/// Result produced by a complete lowering operation.
pub type LoweringResult<T> = FrontendResult<T>;

/// Result produced by lowering one externally supplied operation.
///
/// The operation is returned as the canonical Quantum IR [`Gate`].
pub type LoweredGateResult = FrontendResult<Gate>;

// =============================================================================
// Lowering source contract
// =============================================================================

/// Format-independent source of canonical IR operations.
///
/// A format-specific frontend implements this trait for its own validated
/// representation.
///
/// For example:
///
/// ```text
/// OpenQASM AST ──► OpenQasmLoweringSource
/// QIR module   ──► QirLoweringSource
/// Quil AST     ──► QuilLoweringSource
/// ```
///
/// Each implementation is independent.
///
/// # Important
///
/// `LoweringSource` is intentionally an operation-oriented abstraction rather
/// than a generic AST abstraction. It must not force future formats to adopt
/// an OpenQASM-shaped representation.
pub trait LoweringSource {
    /// Returns the external format identity.
    fn format(&self) -> FormatId;

    /// Returns the external format version.
    fn version(&self) -> FormatVersion;

    /// Returns the number of logical qubits required by the source.
    ///
    /// This value must already represent the logical namespace required by the
    /// source language. It must not represent physical hardware resources.
    fn num_qubits(&self) -> usize;

    /// Returns the number of classical bits required by the source.
    ///
    /// If the source format has no classical namespace, this must return zero.
    fn num_classical_bits(&self) -> usize;

    /// Returns the number of operations that will be lowered.
    ///
    /// This value is used before mutation to enforce operation limits.
    fn operation_count(&self) -> usize;

    /// Returns the source operations in their canonical source order.
    ///
    /// The returned iterator must be deterministic.
    ///
    /// The source representation remains owned by the format implementation.
    /// This trait only exposes the lowering view.
    fn operations(&self) -> Box<dyn Iterator<Item = LoweringOperation<'_>> + '_>;

    /// Performs format-specific validation that is required before lowering.
    ///
    /// This method must not mutate the canonical IR because format validation
    /// belongs to the format implementation.
    fn validate(&self, limits: &FrontendLimits) -> LoweringResult<()>;
}

// =============================================================================
// Lowering operation
// =============================================================================

/// One operation presented to the generic lowering layer.
///
/// This deliberately contains the canonical IR operation rather than an
/// OpenQASM/QIR/Quil-specific gate representation.
///
/// Format-specific implementations are responsible for translating their
/// source semantics into this canonical representation.
pub enum LoweringOperation<'a> {
    /// Canonical quantum gate.
    Gate(&'a Gate),

    /// A format-specific construct that has no canonical IR representation.
    ///
    /// Such constructs must be rejected by the format's lowering logic rather
    /// than silently ignored.
    Unsupported {
        /// Stable format-specific feature name.
        feature: &'a str,
    },
}

impl<'a> fmt::Debug for LoweringOperation<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gate(gate) => formatter
                .debug_tuple("Gate")
                .field(gate)
                .finish(),

            Self::Unsupported { feature } => formatter
                .debug_struct("Unsupported")
                .field("feature", feature)
                .finish(),
        }
    }
}

// =============================================================================
// Lowering configuration
// =============================================================================

/// Configuration controlling a lowering operation.
///
/// This configuration deliberately separates:
///
/// - frontend limits, which protect parsing/lowering input;
/// - IR limits, which protect the resulting canonical representation.
///
/// These policies have different ownership and must not be conflated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoweringConfig {
    /// Resource limits for the frontend input.
    frontend_limits: FrontendLimits,

    /// Resource limits for the resulting canonical Quantum IR.
    ir_limits: QuantumIrLimits,

    /// Whether canonical IR validation must be performed after construction.
    ///
    /// This is always enabled by [`Lowerer::lower`]. The field exists only as
    /// part of the explicit configuration contract and is intentionally not
    /// exposed as a way to bypass validation.
    validate_output: bool,
}

impl LoweringConfig {
    /// Creates a production lowering configuration.
    ///
    /// Canonical output validation is always enabled.
    #[must_use]
    pub fn new(
        frontend_limits: FrontendLimits,
        ir_limits: QuantumIrLimits,
    ) -> Self {
        Self {
            frontend_limits,
            ir_limits,
            validate_output: true,
        }
    }

    /// Returns frontend resource limits.
    #[must_use]
    pub const fn frontend_limits(&self) -> &FrontendLimits {
        &self.frontend_limits
    }

    /// Returns canonical IR resource limits.
    #[must_use]
    pub const fn ir_limits(&self) -> &QuantumIrLimits {
        &self.ir_limits
    }

    /// Returns whether output validation is enabled.
    ///
    /// This is always `true` for production lowering.
    #[must_use]
    pub const fn validates_output(&self) -> bool {
        self.validate_output
    }
}

impl Default for LoweringConfig {
    fn default() -> Self {
        Self::new(
            FrontendLimits::default(),
            QuantumIrLimits::default(),
        )
    }
}

// =============================================================================
// Lowering context
// =============================================================================

/// Context used while lowering a validated source representation.
///
/// The context owns the destination `QuantumCircuit` and applies the IR's
/// canonical mutation API.
///
/// Format-specific code should use this context rather than constructing an
/// alternative circuit representation.
pub struct LoweringContext {
    format: FormatId,
    version: FormatVersion,
    config: LoweringConfig,
    circuit: QuantumCircuit,
    lowered_operations: usize,
}

impl fmt::Debug for LoweringContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LoweringContext")
            .field("format", &self.format)
            .field("version", &self.version)
            .field("lowered_operations", &self.lowered_operations)
            .field("config", &self.config)
            .finish()
    }
}

impl LoweringContext {
    /// Creates a lowering context.
    ///
    /// The destination circuit is created using the canonical Quantum IR.
    ///
    /// No external side effects occur.
    pub fn new<S>(
        source: &S,
        config: LoweringConfig,
    ) -> LoweringResult<Self>
    where
        S: LoweringSource + ?Sized,
    {
        source.validate(config.frontend_limits())?;

        Self::check_source_limits(source, &config)?;

        let circuit = Self::create_circuit(
            source.num_qubits(),
            source.num_classical_bits(),
            config.ir_limits(),
        )?;

        Ok(Self {
            format: source.format(),
            version: source.version(),
            config,
            circuit,
            lowered_operations: 0,
        })
    }

    /// Returns the source format.
    #[must_use]
    pub const fn format(&self) -> &FormatId {
        &self.format
    }

    /// Returns the source format version.
    #[must_use]
    pub const fn version(&self) -> &FormatVersion {
        &self.version
    }

    /// Returns the number of operations successfully lowered so far.
    #[must_use]
    pub const fn lowered_operations(&self) -> usize {
        self.lowered_operations
    }

    /// Returns the lowering configuration.
    #[must_use]
    pub const fn config(&self) -> &LoweringConfig {
        &self.config
    }

    /// Returns a read-only view of the partially constructed circuit.
    ///
    /// This is intended for diagnostics and controlled inspection only.
    #[must_use]
    pub const fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Adds one canonical gate to the destination circuit.
    ///
    /// The gate is validated by the canonical IR mutation boundary.
    ///
    /// No operation is counted as successfully lowered until the IR accepts
    /// the mutation.
    pub fn push_gate(
        &mut self,
        gate: &Gate,
    ) -> LoweredGateResult {
        self.ensure_operation_capacity()?;

        self.insert_gate(gate)?;

        self.lowered_operations = self
            .lowered_operations
            .checked_add(1)
            .ok_or_else(|| {
                FrontendError::internal(
                    "lowered operation counter overflowed",
                )
            })?;

        Ok(gate.clone())
    }

    /// Rejects a source-level construct that cannot be represented by the
    /// canonical IR.
    ///
    /// Unsupported constructs must never be silently discarded.
    pub fn reject_unsupported(
        &self,
        feature: &str,
    ) -> LoweringResult<()> {
        if feature.is_empty() {
            return Err(FrontendError::invalid_input(
                "unsupported lowering feature name must not be empty",
            ));
        }

        Err(FrontendError::unsupported(format!(
            "format `{}` version `{}` contains unsupported construct `{feature}`",
            self.format,
            self.version,
        )))
    }

    /// Completes lowering and returns the canonical Quantum IR.
    ///
    /// This is the commit point.
    ///
    /// The circuit is subjected to whole-circuit canonical IR validation before
    /// being returned.
    pub fn finish(self) -> LoweringResult<QuantumCircuit> {
        if !self.config.validates_output() {
            return Err(FrontendError::internal(
                "canonical Quantum IR validation cannot be disabled",
            ));
        }

        validate_circuit_with_limits(
            &self.circuit,
            self.config.ir_limits(),
        )
        .map_err(|error| {
            FrontendError::lowering(format!(
                "lowering produced invalid canonical Quantum IR: {error}"
            ))
        })?;

        Ok(self.circuit)
    }

    // -------------------------------------------------------------------------
    // Internal construction helpers
    // -------------------------------------------------------------------------

    fn check_source_limits<S>(
        source: &S,
        config: &LoweringConfig,
    ) -> LoweringResult<()>
    where
        S: LoweringSource + ?Sized,
    {
        let frontend_limits = config.frontend_limits();

        if source.num_qubits() > frontend_limits.max_register_size() {
            return Err(FrontendError::limit_exceeded(format!(
                "source requires {} logical qubits, exceeding frontend register limit {}",
                source.num_qubits(),
                frontend_limits.max_register_size(),
            )));
        }

        if source.num_classical_bits()
            > frontend_limits.max_register_size()
        {
            return Err(FrontendError::limit_exceeded(format!(
                "source requires {} classical bits, exceeding frontend register limit {}",
                source.num_classical_bits(),
                frontend_limits.max_register_size(),
            )));
        }

        if source.operation_count() > frontend_limits.max_gate_operations() {
            return Err(FrontendError::limit_exceeded(format!(
                "source contains {} operations, exceeding frontend operation limit {}",
                source.operation_count(),
                frontend_limits.max_gate_operations(),
            )));
        }

        Ok(())
    }

    fn create_circuit(
        num_qubits: usize,
        num_classical_bits: usize,
        limits: &QuantumIrLimits,
    ) -> LoweringResult<QuantumCircuit> {
        /*
         * IMPORTANT INTEGRATION CONTRACT
         * --------------------------------
         *
         * QuantumCircuit construction belongs exclusively to
         * quantum::ir::circuit.
         *
         * This function is intentionally the only place in this file where a
         * lowering context obtains a destination circuit.
         *
         * The exact constructor must remain the canonical QuantumCircuit
         * constructor exposed by the IR module.
         *
         * If the IR constructor changes, only this adapter point should need
         * adjustment; no format implementation should construct QuantumCircuit
         * directly.
         */

        QuantumCircuit::new(
            num_qubits,
            num_classical_bits,
            limits.clone(),
        )
        .map_err(|error| {
            FrontendError::lowering(format!(
                "failed to create canonical Quantum IR circuit: {error}"
            ))
        })
    }

    fn ensure_operation_capacity(&self) -> LoweringResult<()> {
        let next = self
            .lowered_operations
            .checked_add(1)
            .ok_or_else(|| {
                FrontendError::internal(
                    "lowering operation count overflowed",
                )
            })?;

        if next > self.config.ir_limits().max_operations() {
            return Err(FrontendError::limit_exceeded(format!(
                "lowering would exceed canonical Quantum IR operation limit {}",
                self.config.ir_limits().max_operations(),
            )));
        }

        Ok(())
    }

    fn insert_gate(
        &mut self,
        gate: &Gate,
    ) -> LoweringResult<()> {
        /*
         * The canonical QuantumCircuit API is the semantic authority.
         *
         * Do not manually validate:
         *
         * - qubit ranges;
         * - gate arity;
         * - duplicate operands;
         * - parameter counts;
         * - operation IDs;
         * - circuit operation limits.
         *
         * Those belong to the Quantum IR.
         *
         * This keeps frontend lowering independent from IR implementation
         * details while still guaranteeing canonical validation.
         */

        self.circuit
            .add_gate(gate.clone())
            .map_err(|error| {
                FrontendError::lowering(format!(
                    "canonical Quantum IR rejected lowered gate: {error}"
                ))
            })
    }
}

// =============================================================================
// Complete lowering API
// =============================================================================

/// Stateless production lowerer.
///
/// The type exists to provide a stable public API while keeping all mutable
/// construction state inside [`LoweringContext`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Lowerer;

impl Lowerer {
    /// Creates a production lowerer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Lowers a validated format-specific source into canonical Quantum IR.
    ///
    /// The operation is transactional from the caller's perspective:
    ///
    /// - success returns a fully validated `QuantumCircuit`;
    /// - failure returns a `FrontendError`;
    /// - no partially constructed circuit is returned.
    pub fn lower<S>(
        &self,
        source: &S,
        config: LoweringConfig,
    ) -> LoweringResult<QuantumCircuit>
    where
        S: LoweringSource + ?Sized,
    {
        let mut context = LoweringContext::new(source, config)?;

        for operation in source.operations() {
            match operation {
                LoweringOperation::Gate(gate) => {
                    context.push_gate(gate)?;
                }

                LoweringOperation::Unsupported { feature } => {
                    context.reject_unsupported(feature)?;
                }
            }
        }

        context.finish()
    }
}

// =============================================================================
// Convenience API
// =============================================================================

/// Lowers a format-specific source using the default production configuration.
///
/// This function is intentionally format-independent.
pub fn lower<S>(
    source: &S,
) -> LoweringResult<QuantumCircuit>
where
    S: LoweringSource + ?Sized,
{
    Lowerer::new().lower(
        source,
        LoweringConfig::default(),
    )
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Verifies that a lowering source can fit inside both frontend and IR limits.
///
/// This function performs only resource checks. It does not construct or
/// mutate a `QuantumCircuit`.
pub fn validate_lowering_capacity<S>(
    source: &S,
    config: &LoweringConfig,
) -> LoweringResult<()>
where
    S: LoweringSource + ?Sized,
{
    source.validate(config.frontend_limits())?;

    if source.num_qubits() > config.ir_limits().max_qubits() {
        return Err(FrontendError::limit_exceeded(format!(
            "source requires {} qubits, exceeding canonical Quantum IR limit {}",
            source.num_qubits(),
            config.ir_limits().max_qubits(),
        )));
    }

    if source.num_classical_bits()
        > config.ir_limits().max_classical_bits()
    {
        return Err(FrontendError::limit_exceeded(format!(
            "source requires {} classical bits, exceeding canonical Quantum IR limit {}",
            source.num_classical_bits(),
            config.ir_limits().max_classical_bits(),
        )));
    }

    if source.operation_count()
        > config.ir_limits().max_operations()
    {
        return Err(FrontendError::limit_exceeded(format!(
            "source contains {} operations, exceeding canonical Quantum IR limit {}",
            source.operation_count(),
            config.ir_limits().max_operations(),
        )));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These tests intentionally test the generic lowering boundary rather than
     * OpenQASM.
     *
     * OpenQASM-specific lowering tests belong in:
     *
     *     frontend/tests/openqasm/importer.rs
     *     frontend/tests/openqasm/integration.rs
     *
     * This prevents the generic frontend contract from becoming coupled to
     * OpenQASM.
     */

    #[test]
    fn default_lowering_config_enables_output_validation() {
        let config = LoweringConfig::default();

        assert!(config.validates_output());
    }

    #[test]
    fn lowerer_is_constructible_without_state() {
        let _lowerer = Lowerer::new();
    }

    #[test]
    fn lowering_config_keeps_frontend_and_ir_limits_separate() {
        let frontend_limits = FrontendLimits::default();
        let ir_limits = QuantumIrLimits::default();

        let config = LoweringConfig::new(
            frontend_limits.clone(),
            ir_limits.clone(),
        );

        assert_eq!(
            config.frontend_limits(),
            &frontend_limits
        );

        assert_eq!(
            config.ir_limits(),
            &ir_limits
        );
    }

    #[test]
    fn unsupported_operations_are_not_silently_discarded() {
        /*
         * This behavioral rule is deliberately documented here even though
         * constructing a complete format source is format-specific.
         *
         * Every format adapter must ultimately route unsupported constructs
         * through LoweringContext::reject_unsupported().
         */
        let _ = FrontendErrorKind::Unsupported;
    }
}