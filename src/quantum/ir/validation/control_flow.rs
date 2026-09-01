//! Zamani Quantum IR — Production Control-Flow Validation
//!
//! Canonical validation boundary for structured quantum/classical control
//! flow.
//!
//! # Architectural role
//!
//! This module validates the semantic control-flow model owned by:
//!
//! ```text
//! quantum::ir::control::control_flow
//! ```
//!
//! It validates:
//!
//! - `ControlFlowRegion`;
//! - `ControlFlowBlock`;
//! - `ControlFlowNode`;
//! - classical predicates;
//! - classical-bit namespace references;
//! - logical-qubit loop ranges;
//! - integer loop domains;
//! - repeat domains;
//! - structured transfers;
//! - control-flow nesting depth;
//! - recursive control-flow node count;
//! - operation-reference integrity when an operation registry is supplied;
//! - resource-policy compliance;
//! - deterministic validation work.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - validation of control-flow-specific invariants;
//! - validation policy adaptation from `QuantumIrLimits`;
//! - non-recursive traversal of potentially hostile/deep control-flow IR;
//! - validation diagnostics specific to control flow;
//! - integration helpers for operation-reference validation.
//!
//! This file does NOT own:
//!
//! - the definition of `ControlFlowNode`;
//! - the definition of `ControlFlowRegion`;
//! - the definition of `ClassicalPredicate`;
//! - operation semantics;
//! - gate semantics;
//! - frontend parsing;
//! - routing;
//! - scheduling;
//! - hardware;
//! - pulse generation;
//! - simulation;
//! - QEC decoding;
//! - backend execution.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical dependencies
//!
//! ```text
//! quantum::ir::limits::QuantumIrLimits
//!                 │
//!                 ▼
//! validation::control_flow
//!                 ▲
//!                 │
//! quantum::ir::control::control_flow
//!                 │
//!       ┌─────────┴─────────┐
//!       ▼                   ▼
//!   QubitId            OperationId
//! ```
//!
//! Logical qubits are always the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! No alternate qubit identity is introduced here.
//!
//! # Why this validator does not call `ControlFlowRegion::validate()`
//!
//! The semantic control-flow implementation intentionally contains convenient
//! local validation APIs. However, a production trust-boundary validator must
//! be robust against deeply nested externally supplied IR.
//!
//! Calling recursive validation from this layer would make validation depth
//! proportional to Rust call-stack depth.
//!
//! This implementation therefore performs its own explicit-stack traversal.
//!
//! Consequently:
//!
//! ```text
//! malicious/deep IR
//!       │
//!       ▼
//! explicit Vec validation stack
//!       │
//!       ▼
//! bounded validation
//! ```
//!
//! rather than:
//!
//! ```text
//! malicious/deep IR
//!       │
//!       ▼
//! recursive Rust calls
//!       │
//!       ▼
//! stack exhaustion
//! ```
//!
//! # Scalability
//!
//! There is no architectural maximum number of:
//!
//! - qubits;
//! - operations;
//! - branches;
//! - loops;
//! - iterations;
//! - control-flow nodes;
//! - classical bits.
//!
//! All finite ceilings come from the supplied `QuantumIrLimits` policy.
//!
//! `QuantumIrLimits::unbounded()` therefore removes application-level finite
//! policy ceilings, while the host's actual memory/address space remains the
//! ultimate physical limitation.
//!
//! Critically, this validator NEVER:
//!
//! - allocates one element per declared qubit;
//! - expands a logical-qubit range into individual qubits;
//! - expands a loop into iterations;
//! - expands a repeat count;
//! - expands an integer range;
//! - uses a fixed-size qubit array;
//! - uses a fixed machine topology;
//! - assumes a maximum qubit count.
//!
//! # Determinism
//!
//! Validation is deterministic for identical:
//!
//! - IR;
//! - namespace sizes;
//! - validation policy;
//! - operation registry.
//!
//! No hash-map iteration is required for correctness.
//!
//! # Security
//!
//! This module is intended to run at trust boundaries including:
//!
//! - frontend lowering;
//! - deserialization;
//! - cache replay;
//! - compiler services;
//! - generated IR ingestion;
//! - optimization output;
//! - distributed compilation;
//! - external tooling.
//!
//! Invalid IR is rejected explicitly.
//!
//! No malformed construct is silently ignored.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This file should be exposed by the validation namespace:
//!
//! ```text
//! pub mod control_flow;
//! ```
//!
//! The canonical structured-control implementation remains:
//!
//! ```text
//! quantum::ir::control::control_flow
//! ```
//!
//! The old flat `quantum::ir::control_flow` module is a compatibility layer
//! only and is not the ownership location for new validation code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::control::control_flow::{
    ClassicalBool,
    ClassicalPredicate,
    ControlFlowBlock,
    ControlFlowError,
    ControlFlowNode,
    ControlFlowRegion,
    ControlFlowValidationContext,
    ControlFlowValidationPolicy,
    ControlTransfer,
    IntegerLoopRange,
    LoopDomain,
    LoopVariable,
    QubitLoopRange,
    ReturnValue,
};
use super::super::identity::OperationId;
use super::super::limits::QuantumIrLimits;
use super::super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result returned by control-flow validation.
pub type ControlFlowValidationResult<T> =
    Result<T, ControlFlowValidationError>;

// =============================================================================
// Validation error
// =============================================================================

/// Production validation errors for structured control flow.
///
/// This error type intentionally wraps the semantic `ControlFlowError` only
/// where the semantic type is the appropriate owner. Validation-specific
/// failures remain here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowValidationError {
    /// A semantic control-flow invariant is invalid.
    Semantic(ControlFlowError),

    /// The control-flow node count exceeds the repository-wide policy.
    NodeLimitExceeded {
        /// Number of nodes encountered/requested.
        requested: usize,

        /// Maximum permitted by policy.
        maximum: usize,
    },

    /// The control-flow nesting depth exceeds policy.
    DepthLimitExceeded {
        /// Required nesting depth.
        requested: usize,

        /// Maximum permitted depth.
        maximum: usize,
    },

    /// Predicate node count exceeds policy.
    ConditionLimitExceeded {
        /// Number of predicate nodes encountered.
        requested: usize,

        /// Maximum permitted predicate nodes.
        maximum: usize,
    },

    /// Validation work exceeds the repository-wide validation budget.
    ValidationWorkExceeded {
        /// Number of work units requested.
        requested: usize,

        /// Maximum permitted work units.
        maximum: usize,
    },

    /// A classical-bit reference is outside the enclosing classical namespace.
    ClassicalBitOutOfRange {
        /// Referenced classical bit.
        bit: usize,

        /// Number of classical bits in the namespace.
        count: usize,
    },

    /// A logical-qubit reference is outside the enclosing logical namespace.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Number of logical qubits in the namespace.
        count: usize,
    },

    /// A logical-qubit range is malformed.
    QubitRangeInvalid {
        /// First logical-qubit index.
        start: usize,

        /// Exclusive end index.
        end: usize,

        /// Number of logical qubits in the namespace.
        count: usize,
    },

    /// An operation reference does not exist in the supplied operation table.
    OperationReferenceMissing {
        /// Missing operation identity.
        operation: OperationId,
    },

    /// An operation reference was checked against an operation table that
    /// contained a duplicate identity.
    OperationReferenceAmbiguous {
        /// Ambiguous operation identity.
        operation: OperationId,
    },

    /// A required structured block is empty.
    EmptyRequiredBlock {
        /// Semantic block name.
        block: &'static str,
    },

    /// A structured transfer occurs outside the required context.
    InvalidTransferContext {
        /// Invalid transfer.
        transfer: ControlTransfer,
    },

    /// A loop variable is semantically invalid.
    ///
    /// The validator intentionally does not reserve arbitrary integer
    /// identities such as `u64::MAX` as a sentinel. Loop-variable identity is
    /// opaque and all representable identifiers are valid unless another
    /// symbol/SSA layer explicitly rejects them.
    InvalidLoopVariable,

    /// A loop domain is malformed.
    InvalidLoopDomain {
        /// Static reason.
        reason: &'static str,
    },

    /// Arithmetic overflow occurred while validating/counting.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },
}

impl fmt::Display for ControlFlowValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Semantic(error) => {
                write!(formatter, "control-flow semantic error: {error}")
            }

            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow node limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::DepthLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow depth limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ConditionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow condition-node limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ValidationWorkExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow validation-work limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                count,
            } => {
                write!(
                    formatter,
                    "classical bit {bit} is outside namespace \
                     containing {count} classical bits"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                count,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside namespace \
                     containing {count} qubits"
                )
            }

            Self::QubitRangeInvalid {
                start,
                end,
                count,
            } => {
                write!(
                    formatter,
                    "logical-qubit range [{start}, {end}) is invalid \
                     for namespace containing {count} qubits"
                )
            }

            Self::OperationReferenceMissing { operation } => {
                write!(
                    formatter,
                    "control-flow operation reference {operation} \
                     does not exist"
                )
            }

            Self::OperationReferenceAmbiguous { operation } => {
                write!(
                    formatter,
                    "control-flow operation reference {operation} \
                     is ambiguous"
                )
            }

            Self::EmptyRequiredBlock { block } => {
                write!(
                    formatter,
                    "required control-flow block `{block}` is empty"
                )
            }

            Self::InvalidTransferContext { transfer } => {
                write!(
                    formatter,
                    "control transfer `{transfer}` is invalid \
                     in the current control-flow context"
                )
            }

            Self::InvalidLoopVariable => {
                formatter.write_str("invalid control-flow loop variable")
            }

            Self::InvalidLoopDomain { reason } => {
                write!(
                    formatter,
                    "invalid control-flow loop domain: {reason}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while validating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for ControlFlowValidationError {}

impl From<ControlFlowError> for ControlFlowValidationError {
    fn from(error: ControlFlowError) -> Self {
        Self::Semantic(error)
    }
}

// =============================================================================
// Validation configuration
// =============================================================================

/// Immutable configuration for one control-flow validation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlFlowValidationConfig {
    /// Number of logical qubits in the enclosing logical namespace.
    pub num_qubits: usize,

    /// Number of classical bits in the enclosing classical namespace.
    pub num_classical_bits: usize,

    /// Repository-wide IR resource policy.
    pub limits: QuantumIrLimits,

    /// Whether the containing region is a function body.
    pub in_function: bool,

    /// Whether operation references must be checked against a registry.
    pub validate_operation_references: bool,
}

impl ControlFlowValidationConfig {
    /// Creates a production validation configuration.
    #[must_use]
    pub const fn new(
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
        in_function: bool,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            limits,
            in_function,
            validate_operation_references: false,
        }
    }

    /// Enables operation-reference existence validation.
    #[must_use]
    pub const fn with_operation_reference_validation(
        mut self,
        enabled: bool,
    ) -> Self {
        self.validate_operation_references = enabled;
        self
    }

    /// Creates an unrestricted trusted-workload configuration.
    #[must_use]
    pub const fn unbounded(
        num_qubits: usize,
        num_classical_bits: usize,
        in_function: bool,
    ) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            limits: QuantumIrLimits::unbounded(),
            in_function,
            validate_operation_references: false,
        }
    }

    /// Converts the configuration into the semantic control-flow context.
    #[must_use]
    pub const fn semantic_context(
        self,
    ) -> ControlFlowValidationContext {
        ControlFlowValidationContext::new(
            self.num_qubits,
            self.num_classical_bits,
            ControlFlowValidationPolicy::new(
                self.limits.max_operations(),
                self.limits.max_control_flow_depth(),
                self.limits.max_operations(),
            ),
            self.in_function,
        )
    }
}

// =============================================================================
// Validation statistics
// =============================================================================

/// Deterministic statistics produced by a successful validation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ControlFlowValidationStats {
    /// Number of control-flow nodes inspected.
    pub nodes: usize,

    /// Number of operation references inspected.
    pub operation_references: usize,

    /// Number of condition nodes inspected.
    pub condition_nodes: usize,

    /// Number of classical-bit dependency occurrences inspected.
    pub classical_bit_references: usize,

    /// Number of logical-qubit loop-domain references inspected.
    pub qubit_range_references: usize,

    /// Maximum control-flow nesting depth observed.
    pub maximum_depth: usize,

    /// Number of validation work units consumed.
    pub validation_steps: usize,
}

impl ControlFlowValidationStats {
    fn checked_add_node(
        &mut self,
    ) -> ControlFlowValidationResult<()> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or(
                ControlFlowValidationError::ArithmeticOverflow {
                    calculation: "control-flow node count",
                },
            )?;

        Ok(())
    }

    fn checked_add_operation(
        &mut self,
    ) -> ControlFlowValidationResult<()> {
        self.operation_references = self
            .operation_references
            .checked_add(1)
            .ok_or(
                ControlFlowValidationError::ArithmeticOverflow {
                    calculation: "operation-reference count",
                },
            )?;

        Ok(())
    }

    fn checked_add_condition_node(
        &mut self,
    ) -> ControlFlowValidationResult<()> {
        self.condition_nodes = self
            .condition_nodes
            .checked_add(1)
            .ok_or(
                ControlFlowValidationError::ArithmeticOverflow {
                    calculation: "condition-node count",
                },
            )?;

        Ok(())
    }

    fn checked_add_classical_reference(
        &mut self,
    ) -> ControlFlowValidationResult<()> {
        self.classical_bit_references = self
            .classical_bit_references
            .checked_add(1)
            .ok_or(
                ControlFlowValidationError::ArithmeticOverflow {
                    calculation: "classical-bit reference count",
                },
            )?;

        Ok(())
    }

    fn checked_add_qubit_range_reference(
        &mut self,
    ) -> ControlFlowValidationResult<()> {
        self.qubit_range_references = self
            .qubit_range_references
            .checked_add(1)
            .ok_or(
                ControlFlowValidationError::ArithmeticOverflow {
                    calculation: "qubit-range reference count",
                },
            )?;

        Ok(())
    }

    fn observe_depth(
        &mut self,
        depth: usize,
    ) -> ControlFlowValidationResult<()> {
        if depth > self.maximum_depth {
            self.maximum_depth = depth;
        }

        Ok(())
    }
}

// =============================================================================
// Public validation entry points
// =============================================================================

/// Validates a complete control-flow region using production limits.
///
/// Operation-reference existence is not checked because the operation table
/// belongs to the enclosing program/module.
///
/// Use [`validate_region_with_operation_registry`] when that registry is
/// available.
pub fn validate_region(
    region: &ControlFlowRegion,
    config: &ControlFlowValidationConfig,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_region_internal(region, config, None)
}

/// Validates a complete control-flow region and checks every `OperationId`
/// against a caller-owned operation registry.
///
/// The registry is represented as a `BTreeSet` so validation remains
/// deterministic.
///
/// The validator does not require the entire program representation to be
/// coupled to control-flow validation.
pub fn validate_region_with_operation_registry(
    region: &ControlFlowRegion,
    config: &ControlFlowValidationConfig,
    operations: &BTreeSet<OperationId>,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_region_internal(
        region,
        config,
        Some(operations),
    )
}

/// Validates a region using the repository production resource policy.
pub fn validate_region_production(
    region: &ControlFlowRegion,
    num_qubits: usize,
    num_classical_bits: usize,
    in_function: bool,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    let config = ControlFlowValidationConfig::new(
        num_qubits,
        num_classical_bits,
        QuantumIrLimits::production(),
        in_function,
    );

    validate_region(region, &config)
}

/// Validates a region using an explicitly supplied `QuantumIrLimits`.
pub fn validate_region_with_limits(
    region: &ControlFlowRegion,
    num_qubits: usize,
    num_classical_bits: usize,
    limits: &QuantumIrLimits,
    in_function: bool,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    let config = ControlFlowValidationConfig::new(
        num_qubits,
        num_classical_bits,
        *limits,
        in_function,
    );

    validate_region(region, &config)
}

/// Validates a single control-flow block.
pub fn validate_block(
    block: &ControlFlowBlock,
    config: &ControlFlowValidationConfig,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_block_internal(block, config, None)
}

/// Validates a single control-flow block against an operation registry.
pub fn validate_block_with_operation_registry(
    block: &ControlFlowBlock,
    config: &ControlFlowValidationConfig,
    operations: &BTreeSet<OperationId>,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_block_internal(
        block,
        config,
        Some(operations),
    )
}

/// Validates a single node.
pub fn validate_node(
    node: &ControlFlowNode,
    config: &ControlFlowValidationConfig,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    let mut stats = ControlFlowValidationStats::default();

    let mut stack = Vec::new();

    stack.push(ValidationFrame {
        node,
        depth: 0,
        in_loop: false,
    });

    validate_stack(
        &mut stack,
        config,
        None,
        &mut stats,
    )?;

    Ok(stats)
}

/// Validates a single node against an operation registry.
pub fn validate_node_with_operation_registry(
    node: &ControlFlowNode,
    config: &ControlFlowValidationConfig,
    operations: &BTreeSet<OperationId>,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    let mut stats = ControlFlowValidationStats::default();

    let mut stack = Vec::new();

    stack.push(ValidationFrame {
        node,
        depth: 0,
        in_loop: false,
    });

    validate_stack(
        &mut stack,
        config,
        Some(operations),
        &mut stats,
    )?;

    Ok(stats)
}

// =============================================================================
// Internal validation
// =============================================================================

fn validate_region_internal(
    region: &ControlFlowRegion,
    config: &ControlFlowValidationConfig,
    operations: Option<&BTreeSet<OperationId>>,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_configuration(config)?;

    let mut stats = ControlFlowValidationStats::default();

    let mut stack = Vec::new();

    push_block_frames(
        region.body(),
        0,
        false,
        &mut stack,
    )?;

    validate_stack(
        &mut stack,
        config,
        operations,
        &mut stats,
    )?;

    Ok(stats)
}

fn validate_block_internal(
    block: &ControlFlowBlock,
    config: &ControlFlowValidationConfig,
    operations: Option<&BTreeSet<OperationId>>,
) -> ControlFlowValidationResult<ControlFlowValidationStats> {
    validate_configuration(config)?;

    let mut stats = ControlFlowValidationStats::default();

    let mut stack = Vec::new();

    push_block_frames(
        block,
        0,
        false,
        &mut stack,
    )?;

    validate_stack(
        &mut stack,
        config,
        operations,
        &mut stats,
    )?;

    Ok(stats)
}

fn validate_configuration(
    config: &ControlFlowValidationConfig,
) -> ControlFlowValidationResult<()> {
    config
        .limits
        .validate()
        .map_err(|error| {
            ControlFlowValidationError::Semantic(
                ControlFlowError::InvalidStructure {
                    reason: match error {
                        super::super::limits::LimitsError::InvalidConfiguration {
                            ..
                        } => "invalid QuantumIrLimits configuration",

                        _ => "invalid QuantumIrLimits configuration",
                    },
                },
            )
        })?;

    config
        .limits
        .check_qubits(config.num_qubits)
        .map_err(|_| {
            ControlFlowValidationError::Semantic(
                ControlFlowError::InvalidStructure {
                    reason: "logical-qubit namespace exceeds IR policy",
                },
            )
        })?;

    config
        .limits
        .check_classical_bits(config.num_classical_bits)
        .map_err(|_| {
            ControlFlowValidationError::Semantic(
                ControlFlowError::InvalidStructure {
                    reason:
                        "classical-bit namespace exceeds IR policy",
                },
            )
        })?;

    Ok(())
}

// =============================================================================
// Explicit traversal frame
// =============================================================================

/// One explicit validation frame.
///
/// Keeping the frame on a heap-backed `Vec` avoids recursion through nested
/// control-flow structures.
#[derive(Debug, Clone, Copy)]
struct ValidationFrame<'a> {
    node: &'a ControlFlowNode,
    depth: usize,
    in_loop: bool,
}

/// Adds the nodes of a block to the explicit validation stack.
///
/// The reverse insertion order ensures that the original execution order is
/// preserved when the stack is popped.
fn push_block_frames<'a>(
    block: &'a ControlFlowBlock,
    depth: usize,
    in_loop: bool,
    stack: &mut Vec<ValidationFrame<'a>>,
) -> ControlFlowValidationResult<()> {
    for node in block.nodes().iter().rev() {
        stack.push(ValidationFrame {
            node,
            depth,
            in_loop,
        });
    }

    Ok(())
}

// =============================================================================
// Stack traversal
// =============================================================================

fn validate_stack<'a>(
    stack: &mut Vec<ValidationFrame<'a>>,
    config: &ControlFlowValidationConfig,
    operations: Option<&BTreeSet<OperationId>>,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    while let Some(frame) = stack.pop() {
        consume_validation_step(config, stats)?;

        stats.checked_add_node()?;

        check_node_limit(stats.nodes, config.limits.max_operations())?;

        check_depth_limit(
            frame.depth,
            config.limits.max_control_flow_depth(),
        )?;

        stats.observe_depth(frame.depth)?;

        match frame.node {
            ControlFlowNode::Operation(operation) => {
                stats.checked_add_operation()?;

                if let Some(registry) = operations {
                    if !registry.contains(operation) {
                        return Err(
                            ControlFlowValidationError::OperationReferenceMissing {
                                operation: *operation,
                            },
                        );
                    }
                }
            }

            ControlFlowNode::If {
                condition,
                then_block,
                else_block,
            } => {
                validate_required_block(
                    then_block,
                    "if.then",
                )?;

                if let Some(else_block) = else_block {
                    validate_required_block(
                        else_block,
                        "if.else",
                    )?;
                }

                validate_predicate(
                    condition,
                    config,
                    stats,
                )?;

                let child_depth =
                    checked_child_depth(frame.depth)?;

                if let Some(else_block) = else_block {
                    push_block_frames(
                        else_block,
                        child_depth,
                        frame.in_loop,
                        stack,
                    )?;
                }

                push_block_frames(
                    then_block,
                    child_depth,
                    frame.in_loop,
                    stack,
                )?;
            }

            ControlFlowNode::While {
                condition,
                body,
            } => {
                validate_required_block(
                    body,
                    "while.body",
                )?;

                validate_predicate(
                    condition,
                    config,
                    stats,
                )?;

                let child_depth =
                    checked_child_depth(frame.depth)?;

                push_block_frames(
                    body,
                    child_depth,
                    true,
                    stack,
                )?;
            }

            ControlFlowNode::DoWhile {
                body,
                condition,
            } => {
                validate_required_block(
                    body,
                    "do_while.body",
                )?;

                validate_predicate(
                    condition,
                    config,
                    stats,
                )?;

                let child_depth =
                    checked_child_depth(frame.depth)?;

                push_block_frames(
                    body,
                    child_depth,
                    true,
                    stack,
                )?;
            }

            ControlFlowNode::For {
                variable,
                domain,
                body,
            } => {
                validate_loop_variable(*variable)?;

                validate_required_block(
                    body,
                    "for.body",
                )?;

                validate_loop_domain(
                    domain,
                    config,
                    stats,
                )?;

                let child_depth =
                    checked_child_depth(frame.depth)?;

                push_block_frames(
                    body,
                    child_depth,
                    true,
                    stack,
                )?;
            }

            ControlFlowNode::Repeat {
                count: _,
                body,
            } => {
                validate_required_block(
                    body,
                    "repeat.body",
                )?;

                let child_depth =
                    checked_child_depth(frame.depth)?;

                push_block_frames(
                    body,
                    child_depth,
                    true,
                    stack,
                )?;
            }

            ControlFlowNode::Break => {
                validate_transfer_context(
                    ControlTransfer::Break,
                    frame.in_loop,
                    config.in_function,
                )?;
            }

            ControlFlowNode::Continue => {
                validate_transfer_context(
                    ControlTransfer::Continue,
                    frame.in_loop,
                    config.in_function,
                )?;
            }

            ControlFlowNode::Return(value) => {
                validate_transfer_context(
                    ControlTransfer::Return,
                    frame.in_loop,
                    config.in_function,
                )?;

                validate_return_value(
                    value,
                    operations,
                    stats,
                )?;
            }
        }
    }

    Ok(())
}

// =============================================================================
// Node limits
// =============================================================================

fn check_node_limit(
    requested: usize,
    maximum: usize,
) -> ControlFlowValidationResult<()> {
    if requested > maximum {
        return Err(
            ControlFlowValidationError::NodeLimitExceeded {
                requested,
                maximum,
            },
        );
    }

    Ok(())
}

fn check_depth_limit(
    requested: usize,
    maximum: usize,
) -> ControlFlowValidationResult<()> {
    if requested > maximum {
        return Err(
            ControlFlowValidationError::DepthLimitExceeded {
                requested,
                maximum,
            },
        );
    }

    Ok(())
}

fn check_condition_limit(
    requested: usize,
    maximum: usize,
) -> ControlFlowValidationResult<()> {
    if requested > maximum {
        return Err(
            ControlFlowValidationError::ConditionLimitExceeded {
                requested,
                maximum,
            },
        );
    }

    Ok(())
}

fn consume_validation_step(
    config: &ControlFlowValidationConfig,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    stats.validation_steps = stats
        .validation_steps
        .checked_add(1)
        .ok_or(
            ControlFlowValidationError::ArithmeticOverflow {
                calculation: "validation work count",
            },
        )?;

    let maximum =
        config.limits.max_validation_steps();

    if stats.validation_steps > maximum {
        return Err(
            ControlFlowValidationError::ValidationWorkExceeded {
                requested: stats.validation_steps,
                maximum,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Block validation
// =============================================================================

fn validate_required_block(
    block: &ControlFlowBlock,
    name: &'static str,
) -> ControlFlowValidationResult<()> {
    if block.is_empty() {
        return Err(
            ControlFlowValidationError::EmptyRequiredBlock {
                block: name,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Depth
// =============================================================================

fn checked_child_depth(
    depth: usize,
) -> ControlFlowValidationResult<usize> {
    depth
        .checked_add(1)
        .ok_or(
            ControlFlowValidationError::ArithmeticOverflow {
                calculation: "control-flow nesting depth",
            },
        )
}

// =============================================================================
// Predicate validation
// =============================================================================

/// Validates a `ClassicalPredicate` using an explicit stack.
///
/// The predicate implementation itself owns semantic predicate construction;
/// this function validates the predicate again because externally supplied IR
/// may bypass constructors.
fn validate_predicate(
    root: &ClassicalPredicate,
    config: &ControlFlowValidationConfig,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    let maximum_nodes =
        config.limits.max_operations();

    let mut stack: Vec<PredicateFrame<'_>> =
        Vec::new();

    stack.push(PredicateFrame {
        predicate: root,
        depth: 0,
    });

    while let Some(frame) = stack.pop() {
        consume_validation_step(
            config,
            stats,
        )?;

        stats.checked_add_condition_node()?;

        check_condition_limit(
            stats.condition_nodes,
            maximum_nodes,
        )?;

        match frame.predicate {
            ClassicalPredicate::Constant(_) => {}

            ClassicalPredicate::Bit(bit) => {
                validate_classical_bit(
                    *bit,
                    config.num_classical_bits,
                    stats,
                )?;
            }

            ClassicalPredicate::BitEquals {
                bit,
                value: _,
            } => {
                validate_classical_bit(
                    *bit,
                    config.num_classical_bits,
                    stats,
                )?;
            }

            ClassicalPredicate::Not(child) => {
                let depth =
                    checked_predicate_depth(
                        frame.depth,
                    )?;

                check_depth_limit(
                    depth,
                    config.limits.max_control_flow_depth(),
                )?;

                stack.push(PredicateFrame {
                    predicate: child,
                    depth,
                });
            }

            ClassicalPredicate::And(children)
            | ClassicalPredicate::Or(children)
            | ClassicalPredicate::Xor(children) => {
                if children.is_empty() {
                    return Err(
                        ControlFlowValidationError::Semantic(
                            ControlFlowError::EmptyCondition,
                        ),
                    );
                }

                let depth =
                    checked_predicate_depth(
                        frame.depth,
                    )?;

                check_depth_limit(
                    depth,
                    config.limits.max_control_flow_depth(),
                )?;

                for child in children.iter().rev() {
                    stack.push(PredicateFrame {
                        predicate: child,
                        depth,
                    });
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct PredicateFrame<'a> {
    predicate: &'a ClassicalPredicate,
    depth: usize,
}

fn checked_predicate_depth(
    depth: usize,
) -> ControlFlowValidationResult<usize> {
    depth
        .checked_add(1)
        .ok_or(
            ControlFlowValidationError::ArithmeticOverflow {
                calculation: "classical predicate nesting depth",
            },
        )
}

// =============================================================================
// Classical namespace
// =============================================================================

fn validate_classical_bit(
    bit: super::super::measurement::ClassicalBitId,
    num_classical_bits: usize,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    stats.checked_add_classical_reference()?;

    let index = bit.index();

    if index >= num_classical_bits {
        return Err(
            ControlFlowValidationError::ClassicalBitOutOfRange {
                bit: index,
                count: num_classical_bits,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Loop validation
// =============================================================================

fn validate_loop_variable(
    variable: LoopVariable,
) -> ControlFlowValidationResult<()> {
    // LoopVariable is an opaque identity.
    //
    // Do NOT reserve u64::MAX or any other integer as an artificial invalid
    // sentinel. Identity validity belongs to the symbol/value namespace.
    let _ = variable;

    Ok(())
}

fn validate_loop_domain(
    domain: &LoopDomain,
    config: &ControlFlowValidationConfig,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    match domain {
        LoopDomain::Integer(range) => {
            validate_integer_loop_range(
                range,
                config,
            )?;
        }

        LoopDomain::Qubits(range) => {
            stats.checked_add_qubit_range_reference()?;

            validate_qubit_loop_range(
                range,
                config.num_qubits,
            )?;
        }

        LoopDomain::Repeat(_) => {
            // Repeat counts are semantic values.
            //
            // They are intentionally NOT expanded or simulated here.
        }
    }

    Ok(())
}

fn validate_integer_loop_range(
    range: &IntegerLoopRange,
    _config: &ControlFlowValidationConfig,
) -> ControlFlowValidationResult<()> {
    let step = range.step();

    if step == 0 {
        return Err(
            ControlFlowValidationError::Semantic(
                ControlFlowError::ZeroLoopStep,
            ),
        );
    }

    let start = range.start();
    let end = range.end();

    if step > 0 && start > end {
        return Err(
            ControlFlowValidationError::InvalidLoopDomain {
                reason:
                    "positive-step range has start greater than end",
            },
        );
    }

    if step < 0 && start < end {
        return Err(
            ControlFlowValidationError::InvalidLoopDomain {
                reason:
                    "negative-step range has start less than end",
            },
        );
    }

    // Do not call iteration_count().
    //
    // Validation must not perform unnecessary arithmetic over an enormous
    // iteration domain. The domain is semantic and is not expanded.
    //
    // We nevertheless verify the representational relationships above.
    let _ = range.is_inclusive();

    Ok(())
}

fn validate_qubit_loop_range(
    range: &QubitLoopRange,
    num_qubits: usize,
) -> ControlFlowValidationResult<()> {
    let start = range.start().index();
    let end = range.end().index();

    if start > end {
        return Err(
            ControlFlowValidationError::QubitRangeInvalid {
                start,
                end,
                count: num_qubits,
            },
        );
    }

    if start > num_qubits || end > num_qubits {
        return Err(
            ControlFlowValidationError::QubitRangeInvalid {
                start,
                end,
                count: num_qubits,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Transfer validation
// =============================================================================

fn validate_transfer_context(
    transfer: ControlTransfer,
    in_loop: bool,
    in_function: bool,
) -> ControlFlowValidationResult<()> {
    match transfer {
        ControlTransfer::Break
        | ControlTransfer::Continue => {
            if !in_loop {
                return Err(
                    ControlFlowValidationError::InvalidTransferContext {
                        transfer,
                    },
                );
            }
        }

        ControlTransfer::Return => {
            if !in_function {
                return Err(
                    ControlFlowValidationError::InvalidTransferContext {
                        transfer,
                    },
                );
            }
        }
    }

    Ok(())
}

// =============================================================================
// Return validation
// =============================================================================

fn validate_return_value(
    value: &ReturnValue,
    operations: Option<&BTreeSet<OperationId>>,
    stats: &mut ControlFlowValidationStats,
) -> ControlFlowValidationResult<()> {
    match value {
        ReturnValue::Unit => Ok(()),

        ReturnValue::Value(operation) => {
            stats.checked_add_operation()?;

            if let Some(registry) = operations {
                if !registry.contains(operation) {
                    return Err(
                        ControlFlowValidationError::OperationReferenceMissing {
                            operation: *operation,
                        },
                    );
                }
            }

            Ok(())
        }
    }
}

// =============================================================================
// Public utility validation helpers
// =============================================================================

/// Validates a logical qubit against an enclosing logical namespace.
///
/// This is intentionally independent of a `QubitRegister` allocation.
pub fn validate_qubit(
    qubit: QubitId,
    num_qubits: usize,
) -> ControlFlowValidationResult<()> {
    if qubit.index() >= num_qubits {
        return Err(
            ControlFlowValidationError::QubitOutOfRange {
                qubit,
                count: num_qubits,
            },
        );
    }

    Ok(())
}

/// Validates a logical-qubit range without expanding it.
///
/// This is safe for very large ranges because it performs only endpoint
/// validation.
pub fn validate_qubit_range(
    range: &QubitLoopRange,
    num_qubits: usize,
) -> ControlFlowValidationResult<()> {
    validate_qubit_loop_range(
        range,
        num_qubits,
    )
}

/// Validates a classical bit against its enclosing namespace.
pub fn validate_classical_bit(
    bit: super::super::measurement::ClassicalBitId,
    num_classical_bits: usize,
) -> ControlFlowValidationResult<()> {
    let index = bit.index();

    if index >= num_classical_bits {
        return Err(
            ControlFlowValidationError::ClassicalBitOutOfRange {
                bit: index,
                count: num_classical_bits,
            },
        );
    }

    Ok(())
}

/// Validates a structured transfer without constructing a node.
pub const fn validate_transfer(
    transfer: ControlTransfer,
    in_loop: bool,
    in_function: bool,
) -> ControlFlowValidationResult<()> {
    match transfer {
        ControlTransfer::Break
        | ControlTransfer::Continue => {
            if !in_loop {
                return Err(
                    ControlFlowValidationError::InvalidTransferContext {
                        transfer,
                    },
                );
            }
        }

        ControlTransfer::Return => {
            if !in_function {
                return Err(
                    ControlFlowValidationError::InvalidTransferContext {
                        transfer,
                    },
                );
            }
        }
    }

    Ok(())
}

/// Returns the canonical validation-policy projection used for structured
/// control flow.
///
/// This function is useful when another validation subsystem already owns the
/// `QuantumIrLimits` instance.
#[must_use]
pub const fn policy_from_limits(
    limits: &QuantumIrLimits,
) -> ControlFlowValidationPolicy {
    ControlFlowValidationPolicy::new(
        limits.max_operations(),
        limits.max_control_flow_depth(),
        limits.max_operations(),
    )
}

/// Creates a semantic validation context from repository-wide limits.
///
/// This keeps control-flow validation aligned with the repository-wide
/// resource policy instead of introducing an independent hidden limit.
#[must_use]
pub const fn context_from_limits(
    num_qubits: usize,
    num_classical_bits: usize,
    limits: &QuantumIrLimits,
    in_function: bool,
) -> ControlFlowValidationContext {
    ControlFlowValidationContext::new(
        num_qubits,
        num_classical_bits,
        policy_from_limits(limits),
        in_function,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn classical_bit(
        value: usize,
    ) -> super::super::measurement::ClassicalBitId {
        super::super::measurement::ClassicalBitId::new(value)
    }

    fn production_config() -> ControlFlowValidationConfig {
        ControlFlowValidationConfig::new(
            16,
            16,
            QuantumIrLimits::production()
                .with_max_operations(10_000)
                .with_max_control_flow_depth(128),
            true,
        )
    }

    #[test]
    fn simple_operation_region_validates() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push must succeed");

        let stats =
            validate_region(
                &region,
                &production_config(),
            )
            .expect("region must validate");

        assert_eq!(
            stats.nodes,
            1
        );

        assert_eq!(
            stats.operation_references,
            1
        );
    }

    #[test]
    fn operation_registry_validation_is_deterministic() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(7),
            ))
            .expect("push must succeed");

        let mut registry =
            BTreeSet::new();

        registry.insert(operation(7));

        let stats =
            validate_region_with_operation_registry(
                &region,
                &production_config(),
                &registry,
            )
            .expect("operation must exist");

        assert_eq!(
            stats.operation_references,
            1
        );
    }

    #[test]
    fn missing_operation_reference_is_rejected() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(7),
            ))
            .expect("push must succeed");

        let registry =
            BTreeSet::new();

        let result =
            validate_region_with_operation_registry(
                &region,
                &production_config(),
                &registry,
            );

        assert_eq!(
            result,
            Err(
                ControlFlowValidationError::OperationReferenceMissing {
                    operation: operation(7),
                }
            )
        );
    }

    #[test]
    fn out_of_range_classical_bit_is_rejected() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("push must succeed");

        let node =
            ControlFlowNode::if_then(
                ClassicalPredicate::bit(
                    classical_bit(8),
                ),
                body,
            )
            .expect("construction should succeed");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push must succeed");

        let config =
            ControlFlowValidationConfig::new(
                8,
                8,
                QuantumIrLimits::production(),
                true,
            );

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Err(
                ControlFlowValidationError::ClassicalBitOutOfRange {
                    bit: 8,
                    count: 8,
                }
            )
        ));
    }

    #[test]
    fn qubit_loop_range_is_validated_without_expansion() {
        let range =
            QubitLoopRange::new(
                QubitId::new(1),
                QubitId::new(1_000_000),
            )
            .expect("range construction must succeed");

        assert!(
            validate_qubit_range(
                &range,
                1_000_000,
            )
            .is_ok()
        );
    }

    #[test]
    fn qubit_loop_range_above_namespace_is_rejected() {
        let range =
            QubitLoopRange::new(
                QubitId::new(1),
                QubitId::new(1_000_001),
            )
            .expect("range construction must succeed");

        assert!(matches!(
            validate_qubit_range(
                &range,
                1_000_000,
            ),
            Err(
                ControlFlowValidationError::QubitRangeInvalid {
                    start: 1,
                    end: 1_000_001,
                    count: 1_000_000,
                }
            )
        ));
    }

    #[test]
    fn break_outside_loop_is_rejected() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::break_loop())
            .expect("push must succeed");

        let result =
            validate_region(
                &region,
                &production_config(),
            );

        assert!(matches!(
            result,
            Err(
                ControlFlowValidationError::InvalidTransferContext {
                    transfer: ControlTransfer::Break,
                }
            )
        ));
    }

    #[test]
    fn continue_inside_loop_is_valid() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::continue_loop(),
        )
        .expect("push must succeed");

        let node =
            ControlFlowNode::repeat(
                1_000_000_000_000u128,
                body,
            )
            .expect("repeat construction must succeed");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push must succeed");

        assert!(
            validate_region(
                &region,
                &production_config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn return_requires_function_context() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::return_unit())
            .expect("push must succeed");

        let config =
            ControlFlowValidationConfig::new(
                1,
                1,
                QuantumIrLimits::production(),
                false,
            );

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Err(
                ControlFlowValidationError::InvalidTransferContext {
                    transfer: ControlTransfer::Return,
                }
            )
        ));
    }

    #[test]
    fn return_inside_function_is_valid() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::return_unit())
            .expect("push must succeed");

        assert!(
            validate_region(
                &region,
                &production_config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_required_branch_is_rejected() {
        let node =
            ControlFlowNode::If {
                condition:
                    ClassicalPredicate::always(),
                then_block:
                    ControlFlowBlock::new(),
                else_block: None,
            };

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push must succeed");

        assert!(matches!(
            validate_region(
                &region,
                &production_config(),
            ),
            Err(
                ControlFlowValidationError::EmptyRequiredBlock {
                    block: "if.then",
                }
            )
        ));
    }

    #[test]
    fn zero_step_range_is_rejected() {
        let range =
            IntegerLoopRange::new(
                0,
                10,
                0,
            );

        assert_eq!(
            range,
            Err(
                ControlFlowError::ZeroLoopStep
            )
        );
    }

    #[test]
    fn large_repeat_is_not_expanded() {
        let mut body =
            ControlFlowBlock::new();

        body.push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("push must succeed");

        let node =
            ControlFlowNode::repeat(
                u128::MAX,
                body,
            )
            .expect("repeat construction must succeed");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push must succeed");

        let stats =
            validate_region(
                &region,
                &production_config(),
            )
            .expect("region must validate");

        assert_eq!(
            stats.nodes,
            2
        );

        assert_eq!(
            stats.operation_references,
            1
        );
    }

    #[test]
    fn deep_structure_uses_explicit_validation_stack() {
        let mut block =
            ControlFlowBlock::new();

        block.push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("push must succeed");

        // Construct a deliberately deep semantic structure.
        //
        // This test is intentionally bounded by available memory rather than
        // using a fixed architectural depth.
        for _ in 0..64 {
            block =
                ControlFlowBlock::from_nodes(
                    vec![
                        ControlFlowNode::if_then(
                            ClassicalPredicate::always(),
                            block,
                        )
                        .expect(
                            "if construction must succeed",
                        ),
                    ],
                );
        }

        let region =
            ControlFlowRegion::from_block(
                block,
            );

        let config =
            ControlFlowValidationConfig::new(
                1,
                1,
                QuantumIrLimits::production()
                    .with_max_operations(1_000)
                    .with_max_control_flow_depth(128),
                true,
            );

        let stats =
            validate_region(
                &region,
                &config,
            )
            .expect("deep region must validate");

        assert_eq!(
            stats.maximum_depth,
            64
        );
    }

    #[test]
    fn depth_limit_is_enforced() {
        let mut block =
            ControlFlowBlock::new();

        block.push(
            ControlFlowNode::operation(
                operation(1),
            ),
        )
        .expect("push must succeed");

        for _ in 0..8 {
            block =
                ControlFlowBlock::from_nodes(
                    vec![
                        ControlFlowNode::if_then(
                            ClassicalPredicate::always(),
                            block,
                        )
                        .expect(
                            "if construction must succeed",
                        ),
                    ],
                );
        }

        let region =
            ControlFlowRegion::from_block(
                block,
            );

        let config =
            ControlFlowValidationConfig::new(
                1,
                1,
                QuantumIrLimits::production()
                    .with_max_operations(100)
                    .with_max_control_flow_depth(4),
                true,
            );

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Err(
                ControlFlowValidationError::DepthLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn node_limit_is_enforced() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push must succeed");

        region
            .push(ControlFlowNode::operation(
                operation(2),
            ))
            .expect("push must succeed");

        let config =
            ControlFlowValidationConfig::new(
                1,
                1,
                QuantumIrLimits::production()
                    .with_max_operations(1),
                true,
            );

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Err(
                ControlFlowValidationError::NodeLimitExceeded {
                    requested: 2,
                    maximum: 1,
                }
            )
        ));
    }

    #[test]
    fn validation_work_budget_is_enforced() {
        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push must succeed");

        let config =
            ControlFlowValidationConfig::new(
                1,
                1,
                QuantumIrLimits::production()
                    .with_max_operations(100)
                    .with_max_validation_steps(1),
                true,
            );

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Ok(_)
        ));

        let mut region =
            ControlFlowRegion::new();

        region
            .push(ControlFlowNode::operation(
                operation(1),
            ))
            .expect("push must succeed");

        region
            .push(ControlFlowNode::operation(
                operation(2),
            ))
            .expect("push must succeed");

        assert!(matches!(
            validate_region(
                &region,
                &config,
            ),
            Err(
                ControlFlowValidationError::ValidationWorkExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn loop_variable_max_value_is_not_artificially_invalid() {
        let variable =
            LoopVariable::new(u64::MAX);

        let body = {
            let mut block =
                ControlFlowBlock::new();

            block
                .push(
                    ControlFlowNode::operation(
                        operation(1),
                    ),
                )
                .expect("push must succeed");

            block
        };

        let node =
            ControlFlowNode::for_loop(
                variable,
                LoopDomain::Repeat(1),
                body,
            )
            .expect("loop construction must succeed");

        let mut region =
            ControlFlowRegion::new();

        region
            .push(node)
            .expect("push must succeed");

        assert!(
            validate_region(
                &region,
                &production_config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn production_policy_projection_matches_limits() {
        let limits =
            QuantumIrLimits::production()
                .with_max_operations(1234)
                .with_max_control_flow_depth(77);

        let policy =
            policy_from_limits(&limits);

        assert_eq!(
            policy.max_nodes,
            1234
        );

        assert_eq!(
            policy.max_depth,
            77
        );

        assert_eq!(
            policy.max_condition_nodes,
            1234
        );
    }

    #[test]
    fn qubit_identity_is_canonical() {
        let qubit =
            QubitId::new(42);

        assert!(
            validate_qubit(
                qubit,
                43,
            )
            .is_ok()
        );

        assert!(
            validate_qubit(
                qubit,
                42,
            )
            .is_err()
        );
    }

    #[test]
    fn transfer_helper_is_context_safe() {
        assert!(
            validate_transfer(
                ControlTransfer::Break,
                true,
                false,
            )
            .is_ok()
        );

        assert!(
            validate_transfer(
                ControlTransfer::Break,
                false,
                false,
            )
            .is_err()
        );

        assert!(
            validate_transfer(
                ControlTransfer::Return,
                false,
                true,
            )
            .is_ok()
        );

        assert!(
            validate_transfer(
                ControlTransfer::Return,
                false,
                false,
            )
            .is_err()
        );
    }
}