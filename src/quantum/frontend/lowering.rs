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
//!              │    Frontend lowering   │
//!              │      this module       │
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
//! This module owns the format-independent lowering contract.
//!
//! It does NOT own:
//!
//! - OpenQASM grammar;
//! - QIR grammar;
//! - Quil grammar;
//! - format-specific ASTs;
//! - format-specific gate tables;
//! - hardware topology;
//! - qubit routing;
//! - scheduling;
//! - optimization;
//! - backend decomposition;
//! - execution;
//! - filesystem access;
//! - network access;
//! - external process execution.
//!
//! Format implementations are responsible for translating their own validated
//! representations into canonical `quantum::ir` objects and then passing those
//! objects through this boundary.
//!
//! # Canonical IR ownership
//!
//! The Quantum IR remains the sole owner of:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `GateKind`;
//! - `Parameter`;
//! - logical qubit identity;
//! - measurement semantics;
//! - operation invariants;
//! - IR resource limits;
//! - canonical IR validation.
//!
//! The frontend must never create a competing semantic circuit model.
//!
//! # Transactional guarantee
//!
//! The public [`Lowerer::lower`] operation is transactional:
//!
//! ```text
//! validated source
//!       │
//!       ▼
//!   lowering context
//!       │
//!       ├──────────────► error
//!       │                  │
//!       │                  ▼
//!       │             no circuit returned
//!       │
//!       ▼
//! canonical QuantumCircuit
//!       │
//!       ▼
//! whole-circuit validation
//!       │
//!       ▼
//! successful result
//! ```
//!
//! A caller never receives a partially lowered circuit from `lower()`.
//!
//! # Format independence
//!
//! Every format lowers directly into the canonical Quantum IR:
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
//! Adding or removing a format therefore does not require changing this module.
//!
//! # Resource safety
//!
//! Lowering is an untrusted-input boundary.
//!
//! This module therefore:
//!
//! 1. validates the frontend limit configuration;
//! 2. validates the format-specific source before mutation;
//! 3. checks source resource declarations against frontend limits;
//! 4. checks source resource declarations against IR limits;
//! 5. uses checked arithmetic for counters;
//! 6. validates every gate through the canonical IR mutation API;
//! 7. verifies the declared operation count against the actual operation stream;
//! 8. performs whole-circuit canonical IR validation before success;
//! 9. never silently discards unsupported constructs;
//! 10. performs no external effects.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no new dependencies

use std::fmt;

use crate::quantum::ir::{
    validate_circuit_with_limits,
    Gate,
    QuantumCircuit,
    QuantumIrLimits,
};

use super::core::errors::{
    FrontendError,
    FrontendErrorCode,
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

/// Result type used by the lowering subsystem.
pub type LoweringResult<T> = FrontendResult<T>;

/// Result of successfully lowering one canonical gate.
///
/// The returned gate is cloned from the validated source operation. This alias
/// exists as part of the stable API for callers that lower individual
/// operations.
pub type LoweredGateResult = LoweringResult<Gate>;

// =============================================================================
// Lowering source contract
// =============================================================================

/// Format-independent source of already-translated canonical operations.
///
/// A concrete format owns its parser, AST, symbol table, semantic validation,
/// and source-to-IR translation.
///
/// For example:
///
/// ```text
/// OpenQASM AST ──► OpenQasmLoweringSource
/// QIR module   ──► QirLoweringSource
/// Quil AST     ──► QuilLoweringSource
/// ```
///
/// The generic lowering layer never needs to know the structure of those
/// formats.
///
/// # Contract
///
/// Implementations MUST:
///
/// - return deterministic metadata;
/// - return logical, not physical, qubit counts;
/// - return the exact number of operations exposed by `operations()`;
/// - expose operations in canonical source order;
/// - perform format-specific semantic validation in `validate()`;
/// - avoid external side effects.
///
/// Implementations MUST NOT:
///
/// - mutate the canonical IR from `validate()`;
/// - access the network;
/// - access arbitrary files;
/// - execute programs;
/// - silently discard unsupported semantics.
pub trait LoweringSource {
    /// Returns the external format identity.
    fn format(&self) -> FormatId;

    /// Returns the external format version.
    fn version(&self) -> FormatVersion;

    /// Returns the number of logical qubits required by the source.
    ///
    /// This represents the source's logical namespace, not physical hardware
    /// resources.
    fn num_qubits(&self) -> usize;

    /// Returns the number of classical bits required by the source.
    ///
    /// Formats without a classical namespace must return zero.
    fn num_classical_bits(&self) -> usize;

    /// Returns the number of operations exposed by [`Self::operations`].
    ///
    /// This value MUST exactly match the number of yielded operations.
    fn operation_count(&self) -> usize;

    /// Returns operations in deterministic source order.
    ///
    /// The iterator must not reorder operations.
    fn operations(
        &self,
    ) -> Box<dyn Iterator<Item = LoweringOperation<'_>> + '_>;

    /// Performs format-specific validation before generic lowering begins.
    ///
    /// This is deliberately separate from canonical IR validation.
    fn validate(
        &self,
        limits: &FrontendLimits,
    ) -> LoweringResult<()>;
}

// =============================================================================
// Lowering operation
// =============================================================================

/// One operation presented to the generic lowering layer.
///
/// The canonical IR owns quantum operation semantics. Format-specific
/// implementations therefore translate their own source constructs before
/// presenting them here.
pub enum LoweringOperation<'a> {
    /// A canonical validated Quantum IR gate.
    Gate(&'a Gate),

    /// A source construct that cannot currently be represented by the
    /// canonical IR.
    ///
    /// This variant exists specifically to make accidental semantic loss
    /// impossible. It must never be ignored.
    Unsupported {
        /// Stable description of the unsupported construct.
        feature: &'a str,
    },
}

impl<'a> fmt::Debug for LoweringOperation<'a> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

/// Configuration for one lowering transaction.
///
/// Frontend and IR limits intentionally remain separate:
///
/// ```text
/// FrontendLimits
///     │
///     ├── protects parsing/lowering complexity
///     │
///     ▼
/// lowering
///     │
///     ▼
/// QuantumIrLimits
///     │
///     └── constrains canonical IR
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoweringConfig {
    frontend_limits: FrontendLimits,
    ir_limits: QuantumIrLimits,
}

impl LoweringConfig {
    /// Creates a lowering configuration.
    ///
    /// Validation of the supplied policies occurs when lowering starts, so an
    /// invalid configuration can be reported through the normal
    /// `FrontendResult` API without introducing another public error type.
    #[must_use]
    pub const fn new(
        frontend_limits: FrontendLimits,
        ir_limits: QuantumIrLimits,
    ) -> Self {
        Self {
            frontend_limits,
            ir_limits,
        }
    }

    /// Returns frontend resource limits.
    #[must_use]
    pub const fn frontend_limits(
        &self,
    ) -> &FrontendLimits {
        &self.frontend_limits
    }

    /// Returns canonical Quantum IR limits.
    #[must_use]
    pub const fn ir_limits(
        &self,
    ) -> &QuantumIrLimits {
        &self.ir_limits
    }
}

impl Default for LoweringConfig {
    fn default() -> Self {
        Self::new(
            FrontendLimits::default(),
            QuantumIrLimits::production(),
        )
    }
}

// =============================================================================
// Lowering context
// =============================================================================

/// Mutable transaction state used during lowering.
///
/// `LoweringContext` is intentionally not the public result of lowering.
/// It may contain a partially constructed circuit while an operation is being
/// processed. Only [`LoweringContext::finish`] can commit it to the caller.
///
/// The public [`Lowerer::lower`] API never exposes this intermediate state.
pub struct LoweringContext {
    format: FormatId,
    version: FormatVersion,
    config: LoweringConfig,
    circuit: QuantumCircuit,
    lowered_operations: usize,
}

impl fmt::Debug for LoweringContext {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("LoweringContext")
            .field("format", &self.format)
            .field("version", &self.version)
            .field(
                "lowered_operations",
                &self.lowered_operations,
            )
            .field("config", &self.config)
            .field("circuit", &self.circuit)
            .finish()
    }
}

impl LoweringContext {
    /// Creates a lowering context for a validated source.
    ///
    /// This method performs all pre-mutation checks before creating the
    /// destination circuit.
    pub fn new<S>(
        source: &S,
        config: LoweringConfig,
    ) -> LoweringResult<Self>
    where
        S: LoweringSource + ?Sized,
    {
        Self::validate_configuration(&config)?;

        source.validate(config.frontend_limits())?;

        Self::check_source_capacity(
            source,
            &config,
        )?;

        let circuit = QuantumCircuit::try_new_with_limits(
            source.num_qubits(),
            source.num_classical_bits(),
            config.ir_limits().clone(),
        )
        .map_err(|error| {
            FrontendError::with_code(
                FrontendErrorKind::Lowering,
                FrontendErrorCode::new("LOWER-E001"),
                format!(
                    "failed to construct canonical Quantum IR circuit: {error}"
                ),
            )
            .context("format", source.format().to_string())
            .context("version", source.version().to_string())
        })?;

        Ok(Self {
            format: source.format(),
            version: source.version(),
            config,
            circuit,
            lowered_operations: 0,
        })
    }

    /// Returns the source format identity.
    #[must_use]
    pub const fn format(
        &self,
    ) -> &FormatId {
        &self.format
    }

    /// Returns the source format version.
    #[must_use]
    pub const fn version(
        &self,
    ) -> &FormatVersion {
        &self.version
    }

    /// Returns the number of successfully inserted operations.
    #[must_use]
    pub const fn lowered_operations(
        &self,
    ) -> usize {
        self.lowered_operations
    }

    /// Returns the active lowering configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> &LoweringConfig {
        &self.config
    }

    /// Returns a read-only view of the current circuit.
    ///
    /// This is intended for diagnostics and controlled inspection.
    #[must_use]
    pub const fn circuit(
        &self,
    ) -> &QuantumCircuit {
        &self.circuit
    }

    /// Lowers and inserts one canonical gate.
    ///
    /// The canonical `QuantumCircuit::push` method remains the final local
    /// mutation authority. This prevents the frontend from duplicating IR
    /// validation logic.
    pub fn push_gate(
        &mut self,
        gate: &Gate,
    ) -> LoweredGateResult {
        self.ensure_operation_capacity()?;

        self.circuit
            .push(gate.clone())
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new("LOWER-E002"),
                    format!(
                        "canonical Quantum IR rejected lowered operation: {error}"
                    ),
                )
                .context("format", self.format.to_string())
                .context("version", self.version.to_string())
                .context("stage", "canonical-ir-insertion")
            })?;

        self.lowered_operations = self
            .lowered_operations
            .checked_add(1)
            .ok_or_else(|| {
                FrontendError::with_code(
                    FrontendErrorKind::Internal,
                    FrontendErrorCode::new("LOWER-I001"),
                    "lowering operation counter overflowed",
                )
            })?;

        Ok(gate.clone())
    }

    /// Rejects a construct that cannot be represented by the canonical IR.
    ///
    /// Unsupported semantics are never silently discarded.
    pub fn reject_unsupported(
        &self,
        feature: &str,
    ) -> LoweringResult<()> {
        if feature.trim().is_empty() {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::InvalidInput,
                    FrontendErrorCode::new("LOWER-E003"),
                    "unsupported feature name must not be empty",
                )
                .context("stage", "lowering"),
            );
        }

        Err(
            FrontendError::with_code(
                FrontendErrorKind::Unsupported,
                FrontendErrorCode::new("LOWER-U001"),
                format!(
                    "format `{}` version `{}` contains unsupported construct `{feature}`",
                    self.format,
                    self.version,
                ),
            )
            .context("format", self.format.to_string())
            .context("version", self.version.to_string())
            .context("feature", feature),
        )
    }

    /// Commits the lowering transaction.
    ///
    /// Whole-circuit canonical validation is mandatory.
    pub fn finish(
        self,
    ) -> LoweringResult<QuantumCircuit> {
        validate_circuit_with_limits(
            &self.circuit,
            self.config.ir_limits(),
        )
        .map_err(|error| {
            FrontendError::with_code(
                FrontendErrorKind::Lowering,
                FrontendErrorCode::new("LOWER-E004"),
                format!(
                    "lowering produced invalid canonical Quantum IR: {error}"
                ),
            )
            .context("format", self.format.to_string())
            .context("version", self.version.to_string())
            .context("stage", "canonical-ir-validation")
        })?;

        Ok(self.circuit)
    }

    // -------------------------------------------------------------------------
    // Internal validation
    // -------------------------------------------------------------------------

    fn validate_configuration(
        config: &LoweringConfig,
    ) -> LoweringResult<()> {
        config
            .frontend_limits()
            .validate()
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::InvalidInput,
                    FrontendErrorCode::new("LOWER-C001"),
                    format!(
                        "invalid frontend limit configuration: {error}"
                    ),
                )
                .context("stage", "lowering-configuration")
            })?;

        config
            .ir_limits()
            .validate()
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::InvalidInput,
                    FrontendErrorCode::new("LOWER-C002"),
                    format!(
                        "invalid Quantum IR limit configuration: {error}"
                    ),
                )
                .context("stage", "lowering-configuration")
            })?;

        Ok(())
    }

    fn check_source_capacity<S>(
        source: &S,
        config: &LoweringConfig,
    ) -> LoweringResult<()>
    where
        S: LoweringSource + ?Sized,
    {
        let num_qubits = source.num_qubits();
        let num_classical_bits =
            source.num_classical_bits();
        let operation_count =
            source.operation_count();

        let qubits_u64 =
            u64::try_from(num_qubits).map_err(|_| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L001"),
                    "logical qubit count cannot be represented by frontend limits",
                )
            })?;

        let classical_bits_u64 =
            u64::try_from(num_classical_bits).map_err(|_| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L002"),
                    "classical-bit count cannot be represented by frontend limits",
                )
            })?;

        let operations_u64 =
            u64::try_from(operation_count).map_err(|_| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L003"),
                    "operation count cannot be represented by frontend limits",
                )
            })?;

        let frontend_limits =
            config.frontend_limits();

        if qubits_u64
            > frontend_limits.max_register_size()
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L004"),
                    format!(
                        "source requires {num_qubits} logical qubits, exceeding frontend register limit {}",
                        frontend_limits.max_register_size(),
                    ),
                )
                .context("resource", "logical-qubits"),
            );
        }

        if classical_bits_u64
            > frontend_limits.max_register_size()
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L005"),
                    format!(
                        "source requires {num_classical_bits} classical bits, exceeding frontend register limit {}",
                        frontend_limits.max_register_size(),
                    ),
                )
                .context("resource", "classical-bits"),
            );
        }

        if operations_u64
            > frontend_limits.max_gate_operations()
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L006"),
                    format!(
                        "source contains {operation_count} operations, exceeding frontend operation limit {}",
                        frontend_limits.max_gate_operations(),
                    ),
                )
                .context("resource", "operations"),
            );
        }

        // Check the same declarations against the canonical IR policy.
        //
        // This is intentionally delegated to QuantumIrLimits so the frontend
        // does not duplicate IR resource semantics.
        config
            .ir_limits()
            .check_qubits(num_qubits)
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L007"),
                    format!(
                        "source logical-qubit count violates canonical Quantum IR limits: {error}"
                    ),
                )
                .context("resource", "logical-qubits")
            })?;

        config
            .ir_limits()
            .check_classical_bits(
                num_classical_bits,
            )
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L008"),
                    format!(
                        "source classical-bit count violates canonical Quantum IR limits: {error}"
                    ),
                )
                .context("resource", "classical-bits")
            })?;

        config
            .ir_limits()
            .check_operations(operation_count)
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L009"),
                    format!(
                        "source operation count violates canonical Quantum IR limits: {error}"
                    ),
                )
                .context("resource", "operations")
            })?;

        Ok(())
    }

    fn ensure_operation_capacity(
        &self,
    ) -> LoweringResult<()> {
        let next_count = self
            .lowered_operations
            .checked_add(1)
            .ok_or_else(|| {
                FrontendError::with_code(
                    FrontendErrorKind::Internal,
                    FrontendErrorCode::new("LOWER-I002"),
                    "lowering operation counter overflowed",
                )
            })?;

        self.config
            .ir_limits()
            .check_operations(next_count)
            .map_err(|error| {
                FrontendError::with_code(
                    FrontendErrorKind::LimitExceeded,
                    FrontendErrorCode::new("LOWER-L010"),
                    format!(
                        "lowering would exceed canonical Quantum IR operation limit: {error}"
                    ),
                )
                .context("resource", "operations")
            })?;

        Ok(())
    }
}

// =============================================================================
// Stateless lowerer
// =============================================================================

/// Stateless production lowerer.
///
/// All mutable state belongs to [`LoweringContext`], allowing the lowerer to
/// remain cheap to construct and safely share between independent operations.
#[derive(Clone, Copy, Debug, Default)]
pub struct Lowerer;

impl Lowerer {
    /// Creates a stateless lowerer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Lowers a complete validated source into canonical Quantum IR.
    ///
    /// The result is transactional:
    ///
    /// - success returns a fully validated circuit;
    /// - failure returns a frontend error;
    /// - no partial circuit escapes.
    pub fn lower<S>(
        &self,
        source: &S,
        config: LoweringConfig,
    ) -> LoweringResult<QuantumCircuit>
    where
        S: LoweringSource + ?Sized,
    {
        let expected_operations =
            source.operation_count();

        let mut context =
            LoweringContext::new(source, config)?;

        let mut observed_operations =
            0usize;

        for operation in source.operations() {
            observed_operations =
                observed_operations
                    .checked_add(1)
                    .ok_or_else(|| {
                        FrontendError::with_code(
                            FrontendErrorKind::Internal,
                            FrontendErrorCode::new("LOWER-I003"),
                            "source operation iterator counter overflowed",
                        )
                    })?;

            match operation {
                LoweringOperation::Gate(gate) => {
                    context.push_gate(gate)?;
                }

                LoweringOperation::Unsupported {
                    feature,
                } => {
                    context
                        .reject_unsupported(feature)?;
                }
            }
        }

        if observed_operations
            != expected_operations
        {
            return Err(
                FrontendError::with_code(
                    FrontendErrorKind::Lowering,
                    FrontendErrorCode::new("LOWER-E005"),
                    format!(
                        "source operation-count contract violated: declared {expected_operations}, yielded {observed_operations}",
                    ),
                )
                .context("format", context.format().to_string())
                .context("version", context.version().to_string()),
            );
        }

        context.finish()
    }
}

// =============================================================================
// Convenience API
// =============================================================================

/// Lowers a source using the default production configuration.
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
// Capacity validation helper
// =============================================================================

/// Validates whether a source can fit inside the configured frontend and
/// canonical IR resource policies.
///
/// This performs no circuit construction and no mutation.
pub fn validate_lowering_capacity<S>(
    source: &S,
    config: &LoweringConfig,
) -> LoweringResult<()>
where
    S: LoweringSource + ?Sized,
{
    LoweringContext::validate_configuration(
        config,
    )?;

    source.validate(
        config.frontend_limits(),
    )?;

    LoweringContext::check_source_capacity(
        source,
        config,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptySource {
        format: FormatId,
        version: FormatVersion,
    }

    impl EmptySource {
        fn new() -> Self {
            Self {
                format: FormatId::new(
                    "test-format",
                )
                .expect(
                    "test format identifier must be valid",
                ),
                version: FormatVersion::new(
                    1,
                    0,
                    0,
                ),
            }
        }
    }

    impl LoweringSource for EmptySource {
        fn format(&self) -> FormatId {
            self.format.clone()
        }

        fn version(&self) -> FormatVersion {
            self.version
        }

        fn num_qubits(&self) -> usize {
            0
        }

        fn num_classical_bits(&self) -> usize {
            0
        }

        fn operation_count(&self) -> usize {
            0
        }

        fn operations(
            &self,
        ) -> Box<
            dyn Iterator<
                    Item = LoweringOperation<'_>,
                > + '_,
        > {
            Box::new(
                std::iter::empty(),
            )
        }

        fn validate(
            &self,
            _limits: &FrontendLimits,
        ) -> LoweringResult<()> {
            Ok(())
        }
    }

    struct MismatchedCountSource;

    impl LoweringSource
        for MismatchedCountSource
    {
        fn format(&self) -> FormatId {
            FormatId::new(
                "test-mismatch",
            )
            .expect(
                "test format identifier must be valid",
            )
        }

        fn version(&self) -> FormatVersion {
            FormatVersion::new(
                1,
                0,
                0,
            )
        }

        fn num_qubits(&self) -> usize {
            0
        }

        fn num_classical_bits(&self) -> usize {
            0
        }

        fn operation_count(&self) -> usize {
            1
        }

        fn operations(
            &self,
        ) -> Box<
            dyn Iterator<
                    Item = LoweringOperation<'_>,
                > + '_,
        > {
            Box::new(
                std::iter::empty(),
            )
        }

        fn validate(
            &self,
            _limits: &FrontendLimits,
        ) -> LoweringResult<()> {
            Ok(())
        }
    }

    #[test]
    fn default_config_uses_production_ir_limits() {
        let config =
            LoweringConfig::default();

        assert_eq!(
            config.ir_limits(),
            &QuantumIrLimits::production(),
        );
    }

    #[test]
    fn empty_source_lowers_to_valid_circuit() {
        let source =
            EmptySource::new();

        let circuit =
            lower(&source)
                .expect(
                    "empty source should lower successfully",
                );

        assert_eq!(
            circuit.num_qubits(),
            0,
        );

        assert_eq!(
            circuit.num_classical_bits(),
            0,
        );

        assert!(
            circuit.is_empty(),
        );

        circuit
            .validate()
            .expect(
                "lowered circuit must pass canonical IR validation",
            );
    }

    #[test]
    fn lowerer_is_stateless() {
        let lowerer =
            Lowerer::new();

        let source =
            EmptySource::new();

        let circuit =
            lowerer
                .lower(
                    &source,
                    LoweringConfig::default(),
                )
                .expect(
                    "source should lower successfully",
                );

        assert!(
            circuit.is_empty(),
        );
    }

    #[test]
    fn capacity_validation_does_not_construct_a_circuit() {
        let source =
            EmptySource::new();

        validate_lowering_capacity(
            &source,
            &LoweringConfig::default(),
        )
        .expect(
            "empty source should fit all limits",
        );
    }

    #[test]
    fn operation_count_contract_is_enforced() {
        let source =
            MismatchedCountSource;

        let result =
            lower(&source);

        assert!(
            result.is_err(),
            "declared and yielded operation counts must agree",
        );
    }

    #[test]
    fn lowering_configuration_preserves_distinct_policies() {
        let frontend =
            FrontendLimits::strict();

        let ir =
            QuantumIrLimits::production();

        let config =
            LoweringConfig::new(
                frontend,
                ir.clone(),
            );

        assert_eq!(
            config.frontend_limits(),
            &frontend,
        );

        assert_eq!(
            config.ir_limits(),
            &ir,
        );
    }
}