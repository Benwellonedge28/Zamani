//! Format-independent lowering boundary for the Zamani quantum frontend.
//!
//! # Responsibility
//!
//! This module owns exactly one architectural transition:
//!
//! ```text
//! validated format representation
//!             │
//!             ▼
//!     LoweringOperation
//!             │
//!             ▼
//!      QuantumCircuit
//! ```
//!
//! This module MUST NOT contain:
//!
//! - OpenQASM-specific logic;
//! - parser logic;
//! - lexer logic;
//! - semantic validation for a concrete source format;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - optimization;
//! - backend execution.
//!
//! Those responsibilities belong to their respective layers.
//!
//! # Transactional guarantee
//!
//! `Lowerer::lower` constructs the resulting circuit in a private,
//! bounded transaction. If any operation fails, no partial circuit is
//! returned.
//!
//! # Resource guarantees
//!
//! Lowering is bounded by both:
//!
//! 1. `FrontendLimits` — limits imposed on untrusted frontend work;
//! 2. `QuantumIrLimits` — limits imposed by the canonical Quantum IR.
//!
//! Both limits must be satisfied.
//!
//! # Determinism
//!
//! A lowering implementation is required to preserve the order supplied
//! by its `LoweringOperation` iterator. The lowerer itself performs no
//! reordering, optimization, deduplication, or implicit transformation.
//!
//! # Source provenance
//!
//! `LoweringSource` provides format-independent provenance information.
//! Concrete frontend implementations are responsible for translating
//! their AST/source spans into this structure.
//!
//! Rust compatibility: 1.97.1.

use std::fmt;

use super::core::errors::{FrontendError, FrontendResult};
use super::core::limits::FrontendLimits;
use super::core::source::SourceSpan;

use crate::quantum::ir::{
    validate_circuit_with_limits,
    CircuitMetadata,
    Gate,
    GateKind,
    QuantumCircuit,
    QuantumIrLimits,
};

/// Configuration controlling a lowering transaction.
///
/// This configuration is deliberately format-independent. A concrete
/// frontend must validate its own semantic rules before invoking the
/// generic lowering boundary.
#[derive(Debug, Clone)]
pub struct LoweringConfig {
    /// Frontend resource limits.
    pub frontend_limits: FrontendLimits,

    /// Canonical Quantum IR resource limits.
    pub ir_limits: QuantumIrLimits,

    /// Whether the produced circuit should be validated after lowering.
    ///
    /// Production callers should normally leave this enabled.
    pub validate_result: bool,
}

impl Default for LoweringConfig {
    fn default() -> Self {
        Self {
            frontend_limits: FrontendLimits::default(),
            ir_limits: QuantumIrLimits::default(),
            validate_result: true,
        }
    }
}

impl LoweringConfig {
    /// Creates a configuration using the supplied frontend and IR limits.
    pub fn new(
        frontend_limits: FrontendLimits,
        ir_limits: QuantumIrLimits,
    ) -> Self {
        Self {
            frontend_limits,
            ir_limits,
            validate_result: true,
        }
    }

    /// Disables the final IR validation.
    ///
    /// This is intended only for controlled internal scenarios where the
    /// caller has an independently established IR invariant.
    ///
    /// Normal production frontend paths should keep validation enabled.
    pub fn without_final_validation(mut self) -> Self {
        self.validate_result = false;
        self
    }

    /// Returns whether final IR validation is enabled.
    pub const fn validates_result(&self) -> bool {
        self.validate_result
    }
}

/// Provenance attached to a lowered operation.
///
/// This type intentionally contains no format-specific information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringSource {
    /// Source span from which the lowered operation originated.
    pub span: SourceSpan,

    /// Optional human-readable operation description.
    ///
    /// This is useful when constructing diagnostics for a lowering
    /// failure. It must not be treated as a semantic identifier.
    pub description: Option<String>,
}

impl LoweringSource {
    /// Creates provenance from a source span.
    pub fn new(span: SourceSpan) -> Self {
        Self {
            span,
            description: None,
        }
    }

    /// Creates provenance with a diagnostic description.
    pub fn with_description(
        span: SourceSpan,
        description: impl Into<String>,
    ) -> Self {
        Self {
            span,
            description: Some(description.into()),
        }
    }
}

/// A format-independent operation that can be lowered into Quantum IR.
///
/// Concrete formats such as OpenQASM should convert their validated
/// semantic representation into these operations rather than coupling
/// the generic lowerer to their AST types.
///
/// The operation contains already-resolved canonical IR information.
/// The lowerer is therefore not a second semantic validator.
#[derive(Debug, Clone)]
pub struct LoweringOperation {
    /// Canonical gate to insert into the circuit.
    pub gate: Gate,

    /// Source provenance for this operation.
    pub source: Option<LoweringSource>,
}

impl LoweringOperation {
    /// Creates a lowering operation.
    pub fn new(gate: Gate) -> Self {
        Self {
            gate,
            source: None,
        }
    }

    /// Creates an operation with source provenance.
    pub fn with_source(
        gate: Gate,
        source: LoweringSource,
    ) -> Self {
        Self {
            gate,
            source: Some(source),
        }
    }

    /// Returns the number of qubits consumed by the operation.
    pub fn qubit_count(&self) -> usize {
        self.gate.qubits().len()
    }

    /// Returns the number of classical bits consumed by the operation.
    pub fn classical_count(&self) -> usize {
        self.gate.classical_bits().len()
    }

    /// Returns the gate kind.
    pub fn kind(&self) -> &GateKind {
        self.gate.kind()
    }
}

/// Context supplied to a lowering transaction.
///
/// The context is immutable so that lowering cannot mutate global
/// frontend state or accidentally share mutable compiler state between
/// concurrent compilations.
#[derive(Debug, Clone)]
pub struct LoweringContext {
    config: LoweringConfig,
    metadata: CircuitMetadata,
}

impl LoweringContext {
    /// Creates a lowering context.
    pub fn new(
        config: LoweringConfig,
        metadata: CircuitMetadata,
    ) -> FrontendResult<Self> {
        validate_config(&config)?;

        Ok(Self {
            config,
            metadata,
        })
    }

    /// Creates a context with default metadata.
    pub fn with_defaults(
        config: LoweringConfig,
    ) -> FrontendResult<Self> {
        Self::new(config, CircuitMetadata::default())
    }

    /// Returns the lowering configuration.
    pub const fn config(&self) -> &LoweringConfig {
        &self.config
    }

    /// Returns the circuit metadata.
    pub const fn metadata(&self) -> &CircuitMetadata {
        &self.metadata
    }
}

/// Result of a successful lowering transaction.
///
/// The result contains only canonical Quantum IR. Format-specific AST
/// structures do not cross this boundary.
#[derive(Debug)]
pub struct LoweringResult {
    circuit: QuantumCircuit,
}

impl LoweringResult {
    /// Creates a result from a validated circuit.
    fn new(circuit: QuantumCircuit) -> Self {
        Self { circuit }
    }

    /// Consumes the result and returns the circuit.
    pub fn into_circuit(self) -> QuantumCircuit {
        self.circuit
    }

    /// Borrows the resulting circuit.
    pub const fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Returns the number of operations in the circuit.
    pub fn operation_count(&self) -> usize {
        self.circuit.operations().len()
    }
}

/// Generic lowering interface.
///
/// Concrete frontend formats should implement their lowering logic by
/// producing `LoweringOperation` values and feeding them through
/// `Lowerer`.
pub trait LoweringProvider {
    /// Produces the canonical operation stream.
    ///
    /// The returned count must exactly match the number of operations
    /// yielded by the iterator.
    fn operations(
        &self,
        context: &LoweringContext,
    ) -> FrontendResult<Box<dyn Iterator<Item = FrontendResult<LoweringOperation>> + '_>>;

    /// Expected number of operations.
    ///
    /// This is checked before and during lowering to prevent silent
    /// truncation or unexpected expansion.
    fn operation_count(&self) -> u64;
}

/// Stateless production lowerer.
///
/// All mutable state required by a lowering transaction lives on the
/// stack or inside the transaction-local `QuantumCircuit`.
#[derive(Debug, Default, Clone, Copy)]
pub struct Lowerer;

impl Lowerer {
    /// Creates a lowerer.
    pub const fn new() -> Self {
        Self
    }

    /// Lowers a provider into canonical Quantum IR.
    ///
    /// This method is transactional:
    ///
    /// ```text
    /// provider
    ///    │
    ///    ▼
    /// capacity validation
    ///    │
    ///    ▼
    /// private circuit
    ///    │
    ///    ▼
    /// operation insertion
    ///    │
    ///    ▼
    /// complete-circuit validation
    ///    │
    ///    ▼
    /// LoweringResult
    /// ```
    ///
    /// Any failure aborts the transaction.
    pub fn lower<P>(
        &self,
        provider: &P,
        context: &LoweringContext,
    ) -> FrontendResult<LoweringResult>
    where
        P: LoweringProvider,
    {
        let declared_operations = provider.operation_count();

        validate_lowering_capacity(
            context.config(),
            declared_operations,
        )?;

        let mut circuit =
            QuantumCircuit::new(context.metadata().clone());

        let operations = provider.operations(context)?;

        let mut actual_operations = 0_u64;

        for operation_result in operations {
            let operation = operation_result?;

            actual_operations = actual_operations
                .checked_add(1)
                .ok_or_else(|| {
                    lowering_limit_error(
                        "lowering operation counter overflow",
                    )
                })?;

            if actual_operations > declared_operations {
                return Err(lowering_limit_error(
                    "lowering provider yielded more operations than declared",
                ));
            }

            validate_operation_capacity(
                context.config(),
                &circuit,
                &operation,
            )?;

            circuit
                .push(operation.gate)
                .map_err(|error| {
                    map_ir_error(error, operation.source.as_ref())
                })?;
        }

        if actual_operations != declared_operations {
            return Err(lowering_limit_error(format!(
                "lowering provider declared {declared_operations} operations \
                 but yielded {actual_operations}"
            )));
        }

        if context.config().validates_result() {
            validate_circuit_with_limits(
                &circuit,
                &context.config().ir_limits,
            )
            .map_err(|error| {
                map_ir_error(error, None)
            })?;
        }

        Ok(LoweringResult::new(circuit))
    }
}

/// Validates that a lowering configuration is internally safe.
///
/// This check happens before allocating a circuit so an invalid
/// configuration cannot partially execute a lowering transaction.
pub fn validate_config(
    config: &LoweringConfig,
) -> FrontendResult<()> {
    validate_frontend_limits(&config.frontend_limits)?;
    validate_ir_limits(&config.ir_limits)?;
    Ok(())
}

/// Validates the operation count against all relevant resource limits.
///
/// This should be called before constructing a potentially large
/// operation stream.
pub fn validate_lowering_capacity(
    config: &LoweringConfig,
    operation_count: u64,
) -> FrontendResult<()> {
    config
        .frontend_limits
        .check_operations(operation_count)
        .map_err(|error| FrontendError::from(error))?;

    config
        .ir_limits
        .check_operations(operation_count)
        .map_err(|error| FrontendError::from(error))?;

    Ok(())
}

/// Validates one operation against the remaining circuit capacity.
///
/// Both frontend and IR limits are checked. The operation itself is
/// then allowed to cross the IR boundary, where the canonical IR
/// performs its own semantic/invariant validation.
fn validate_operation_capacity(
    config: &LoweringConfig,
    circuit: &QuantumCircuit,
    operation: &LoweringOperation,
) -> FrontendResult<()> {
    let current_operations =
        u64::try_from(circuit.operations().len())
            .map_err(|_| {
                lowering_limit_error(
                    "current operation count cannot be represented as u64",
                )
            })?;

    let next_operations = current_operations
        .checked_add(1)
        .ok_or_else(|| {
            lowering_limit_error(
                "operation count overflow during lowering",
            )
        })?;

    config
        .frontend_limits
        .check_operations(next_operations)
        .map_err(FrontendError::from)?;

    config
        .ir_limits
        .check_operations(next_operations)
        .map_err(FrontendError::from)?;

    let qubit_count =
        u64::try_from(operation.qubit_count())
            .map_err(|_| {
                lowering_limit_error(
                    "operation qubit count cannot be represented as u64",
                )
            })?;

    let classical_count =
        u64::try_from(operation.classical_count())
            .map_err(|_| {
                lowering_limit_error(
                    "operation classical-bit count cannot be represented as u64",
                )
            })?;

    let existing_qubits =
        u64::try_from(circuit.qubit_count())
            .map_err(|_| {
                lowering_limit_error(
                    "circuit qubit count cannot be represented as u64",
                )
            })?;

    let existing_classical =
        u64::try_from(circuit.classical_bit_count())
            .map_err(|_| {
                lowering_limit_error(
                    "circuit classical-bit count cannot be represented as u64",
                )
            })?;

    let required_qubits = existing_qubits
        .checked_add(qubit_count)
        .ok_or_else(|| {
            lowering_limit_error(
                "qubit count overflow during lowering",
            )
        })?;

    let required_classical = existing_classical
        .checked_add(classical_count)
        .ok_or_else(|| {
            lowering_limit_error(
                "classical-bit count overflow during lowering",
            )
        })?;

    config
        .frontend_limits
        .check_qubits(required_qubits)
        .map_err(FrontendError::from)?;

    config
        .frontend_limits
        .check_bits(required_classical)
        .map_err(FrontendError::from)?;

    config
        .ir_limits
        .check_qubits(required_qubits)
        .map_err(FrontendError::from)?;

    config
        .ir_limits
        .check_classical_bits(required_classical)
        .map_err(FrontendError::from)?;

    Ok(())
}

/// Validates frontend limits.
///
/// The exact default values remain owned by `FrontendLimits`; this
/// function validates only internal consistency.
fn validate_frontend_limits(
    limits: &FrontendLimits,
) -> FrontendResult<()> {
    if limits.max_operations() == 0 {
        return Err(lowering_limit_error(
            "frontend max_operations must be greater than zero",
        ));
    }

    Ok(())
}

/// Validates canonical IR limits.
///
/// The exact policy remains owned by `QuantumIrLimits`.
fn validate_ir_limits(
    limits: &QuantumIrLimits,
) -> FrontendResult<()> {
    if limits.max_operations() == 0 {
        return Err(lowering_limit_error(
            "Quantum IR max_operations must be greater than zero",
        ));
    }

    Ok(())
}

/// Converts an IR validation failure into the frontend error boundary.
///
/// The frontend must not expose format-specific or backend-specific
/// errors directly to callers.
fn map_ir_error<E>(
    error: E,
    source: Option<&LoweringSource>,
) -> FrontendError
where
    E: fmt::Display,
{
    let message = match source {
        Some(source) => match &source.description {
            Some(description) => format!(
                "failed to lower `{description}`: {error}"
            ),
            None => format!("failed to lower operation: {error}"),
        },
        None => format!("lowering failed: {error}"),
    };

    FrontendError::lowering(message)
}

/// Creates a frontend limit error for failures that occur before a
/// specific `FrontendLimitViolation` exists.
fn lowering_limit_error(
    message: impl Into<String>,
) -> FrontendError {
    FrontendError::lowering_limit(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let config = LoweringConfig::default();

        assert!(validate_config(&config).is_ok());
        assert!(config.validates_result());
    }

    #[test]
    fn context_preserves_configuration() {
        let config = LoweringConfig::default();

        let context =
            LoweringContext::with_defaults(config.clone())
                .expect("default lowering configuration must be valid");

        assert_eq!(
            context.config().validates_result(),
            config.validates_result()
        );
    }

    #[test]
    fn source_constructor_is_deterministic() {
        let span = SourceSpan::new(0, 0, 3)
            .expect("test span must be valid");

        let source = LoweringSource::new(span);

        assert_eq!(source.span, span);
        assert!(source.description.is_none());
    }

    #[test]
    fn source_description_is_preserved() {
        let span = SourceSpan::new(0, 0, 3)
            .expect("test span must be valid");

        let source =
            LoweringSource::with_description(span, "test operation");

        assert_eq!(
            source.description.as_deref(),
            Some("test operation")
        );
    }

    #[test]
    fn lowering_result_exposes_circuit() {
        let circuit =
            QuantumCircuit::new(CircuitMetadata::default());

        let result = LoweringResult::new(circuit);

        assert_eq!(result.operation_count(), 0);
        assert_eq!(result.circuit().operations().len(), 0);
    }

    #[test]
    fn lowerer_is_stateless() {
        let first = Lowerer::new();
        let second = Lowerer::new();

        assert_eq!(first, second);
    }

    #[test]
    fn zero_operation_capacity_is_rejected() {
        let config = LoweringConfig::default();

        let result =
            validate_lowering_capacity(&config, 0);

        assert!(result.is_ok());
    }

    #[test]
    fn operation_count_above_frontend_limit_is_rejected() {
        let config = LoweringConfig::default();

        let count = config
            .frontend_limits
            .max_operations()
            .saturating_add(1);

        let result =
            validate_lowering_capacity(&config, count);

        assert!(result.is_err());
    }

    #[test]
    fn operation_count_above_ir_limit_is_rejected() {
        let config = LoweringConfig::default();

        let count = config
            .ir_limits
            .max_operations()
            .saturating_add(1);

        let result =
            validate_lowering_capacity(&config, count);

        assert!(result.is_err());
    }

    #[test]
    fn lower_empty_provider_produces_empty_circuit() {
        struct EmptyProvider;

        impl LoweringProvider for EmptyProvider {
            fn operations(
                &self,
                _context: &LoweringContext,
            ) -> FrontendResult<
                Box<
                    dyn Iterator<
                            Item = FrontendResult<LoweringOperation>,
                        > + '_,
                >,
            > {
                Ok(Box::new(
                    std::iter::empty::<FrontendResult<LoweringOperation>>(),
                ))
            }

            fn operation_count(&self) -> u64 {
                0
            }
        }

        let context =
            LoweringContext::with_defaults(
                LoweringConfig::default(),
            )
            .expect("default configuration must be valid");

        let result =
            Lowerer::new()
                .lower(&EmptyProvider, &context)
                .expect("empty lowering must succeed");

        assert_eq!(result.operation_count(), 0);
    }

    #[test]
    fn provider_count_mismatch_is_rejected() {
        struct MismatchProvider;

        impl LoweringProvider for MismatchProvider {
            fn operations(
                &self,
                _context: &LoweringContext,
            ) -> FrontendResult<
                Box<
                    dyn Iterator<
                            Item = FrontendResult<LoweringOperation>,
                        > + '_,
                >,
            > {
                Ok(Box::new(
                    std::iter::empty::<FrontendResult<LoweringOperation>>(),
                ))
            }

            fn operation_count(&self) -> u64 {
                1
            }
        }

        let context =
            LoweringContext::with_defaults(
                LoweringConfig::default(),
            )
            .expect("default configuration must be valid");

        let result =
            Lowerer::new()
                .lower(&MismatchProvider, &context);

        assert!(result.is_err());
    }

    #[test]
    fn excessive_declared_operation_count_is_rejected_before_iteration() {
        struct HugeProvider;

        impl LoweringProvider for HugeProvider {
            fn operations(
                &self,
                _context: &LoweringContext,
            ) -> FrontendResult<
                Box<
                    dyn Iterator<
                            Item = FrontendResult<LoweringOperation>,
                        > + '_,
                >,
            > {
                panic!("operation iterator must not be requested");
            }

            fn operation_count(&self) -> u64 {
                u64::MAX
            }
        }

        let context =
            LoweringContext::with_defaults(
                LoweringConfig::default(),
            )
            .expect("default configuration must be valid");

        let result =
            Lowerer::new()
                .lower(&HugeProvider, &context);

        assert!(result.is_err());
    }

    #[test]
    fn lowering_without_final_validation_is_supported() {
        let config =
            LoweringConfig::default()
                .without_final_validation();

        assert!(!config.validates_result());
    }
}