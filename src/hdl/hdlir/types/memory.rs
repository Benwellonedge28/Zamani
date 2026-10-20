
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR logical memory types
//!
//! Path: `src/hdl/hdlir/types/memory.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent logical memory types, shapes, ports, access
//! modes, clocking contracts, initialization intent, and read/write collision
//! semantics for HDLIR.
//!
//! ## Authority
//!
//! The normative language contract is `grammar/spec/hdl.md`. Memory syntax is
//! owned by `grammar/hdl/memories.g4`; array syntax and dimensions are owned
//! by `grammar/hdl/arrays.g4`; grammar composition is owned by
//! `grammar/hdl/hdl.g4` and `grammar/Zamani.g4`.
//!
//! The canonical frontend semantic layer remains authoritative for source
//! meaning. This module represents validated hardware-domain type information;
//! it does not parse source or replace semantic validation.
//!
//! ## Owns
//!
//! - Symbolic and concrete memory extents.
//! - Checked, budgeted extent evaluation.
//! - Logical memory element types and multidimensional shapes.
//! - Logical read/write port contracts.
//! - Port access modes and clocking intent.
//! - Initialization and read-during-write collision contracts.
//! - Construction-time invariants and structured errors.
//!
//! ## Does not own
//!
//! - Source parsing, AST nodes, name resolution, or source-level type truth.
//! - Memory allocation, physical addresses, banks, caches, controllers,
//!   DMA, placement, routing, synthesis, or technology selection.
//! - Runtime reads/writes, simulation scheduling, or memory contents.
//! - Physical port counts or vendor-specific memory primitives.
//! - The core HDLIR graph, builders, verification passes, or serialization.
//!
//! ## Scalability
//!
//! There are no universal limits on depth, dimensions, or port count.
//! Symbolic extents remain symbolic until evaluation is explicitly requested.
//! Concrete arithmetic is checked and never wraps. Evaluation budgets bound
//! compiler work, not the set of legal designs. All processing is finite and
//! subject to available resources and the caller's chosen budget.
//!
//! Concrete extents use `u128`, consistent with the current HDL semantic
//! width/count conventions. This is a representational boundary, not a claim
//! that all possible hardware sizes fit in `u128`; larger or unknown extents
//! must remain symbolic or be reported as unrepresentable by a later adapter.
//!
//! ## Integration contract
//!
//! - `types/mod.rs` declares `pub mod memory;` and exports the public types
//!   intended for stable HDLIR use.
//! - `types/signal.rs` remains the owner of logical signal value types.
//! - `values/parameters.rs` or the lowering layer supplies symbolic bindings.
//! - `lowering/from_semantic.rs` maps validated semantic memory declarations
//!   into this representation while preserving dimensions, access intent,
//!   clock relationships, initialization and collision behavior.
//! - `verify/types.rs`, `verify/processes.rs`, and `verify/resources.rs`
//!   validate contextual legality and resource feasibility; this module's
//!   constructors do not claim that a memory is realizable on a target.
//! - `ir/memories.rs` associates these types with logical memory instances.
//! - Serialization must preserve every variant and symbolic expression.
//!
//! This module deliberately does not import APIs from those not-yet-verified
//! downstream modules.
//!
//! ## Freeze criteria
//!
//! Constructors, arithmetic, symbolic evaluation, duplicate-port detection,
//! equality, and boundary behavior must have focused tests. The module must
//! be registered in the actual module tree and pass the repository's
//! formatting, build, test and Clippy checks before being considered frozen.

use core::fmt;
use core::num::NonZeroU128;

use super::bit_width::WidthSymbol;
use super::signal::SignalType;

/// A symbolic or concrete extent of a logical memory dimension.
///
/// Extents are counts, not bit widths. A concrete extent must be positive.
/// Symbolic expressions are preserved until their bindings are available.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryDepth {
    /// A positive, concrete number of elements in this dimension.
    Known(NonZeroU128),

    /// A named extent parameter.
    Symbol(WidthSymbol),

    /// The sum of two extent expressions.
    Add(Box<MemoryDepth>, Box<MemoryDepth>),

    /// The product of two extent expressions.
    Multiply(Box<MemoryDepth>, Box<MemoryDepth>),

    /// An extent whose definition or binding is not currently available.
    Unresolved(WidthSymbol),
}

impl MemoryDepth {
    /// Construct a positive concrete extent.
    ///
    /// Zero-sized dimensions are rejected by this type. If the language
    /// later defines zero-sized memories as legal, that must be introduced
    /// through an explicit specification and API change.
    pub fn known(value: u128) -> Result<Self, MemoryDepthError> {
        NonZeroU128::new(value)
            .map(Self::Known)
            .ok_or(MemoryDepthError::ZeroExtent)
    }

    /// Construct a named symbolic extent.
    pub fn symbol(name: impl Into<String>) -> Result<Self, MemoryDepthError> {
        WidthSymbol::new(name)
            .map(Self::Symbol)
            .map_err(|_| MemoryDepthError::EmptySymbol)
    }

    /// Construct an explicitly unresolved extent.
    pub fn unresolved(name: impl Into<String>) -> Result<Self, MemoryDepthError> {
        WidthSymbol::new(name)
            .map(Self::Unresolved)
            .map_err(|_| MemoryDepthError::EmptySymbol)
    }

    /// Construct a symbolic addition without evaluating it.
    #[must_use]
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    /// Construct a symbolic multiplication without evaluating it.
    #[must_use]
    pub fn multiply(lhs: Self, rhs: Self) -> Self {
        Self::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// Return the concrete extent if this expression is already known.
    #[must_use]
    pub const fn as_known(&self) -> Option<u128> {
        match self {
            Self::Known(value) => Some(value.get()),
            _ => None,
        }
    }

    /// Evaluate the expression using the supplied symbol resolver and budget.
    ///
    /// The resolver returns a positive concrete binding for a symbol, or
    /// `None` when no binding is available. Both ordinary symbols and
    /// explicitly unresolved symbols are passed to the resolver.
    ///
    /// Evaluation uses an explicit work stack rather than recursive
    /// expression traversal. Arithmetic overflow, zero bindings, missing
    /// bindings and budget exhaustion are reported as errors.
    pub fn evaluate_with_budget(
        &self,
        budget: MemoryDepthBudget,
        mut resolve: impl FnMut(&str) -> Option<u128>,
    ) -> Result<u128, MemoryDepthError> {
        enum Task<'a> {
            Visit(&'a MemoryDepth),
            Add,
            Multiply,
        }

        let mut tasks = vec![Task::Visit(self)];
        let mut values: Vec<u128> = Vec::new();
        let mut visited = 0usize;

        while let Some(task) = tasks.pop() {
            match task {
                Task::Visit(node) => {
                    visited = visited
                        .checked_add(1)
                        .ok_or(MemoryDepthError::NodeCountOverflow)?;

                    if visited > budget.max_nodes {
                        return Err(MemoryDepthError::BudgetExceeded {
                            limit: budget.max_nodes,
                            visited,
                        });
                    }

                    match node {
                        Self::Known(value) => values.push(value.get()),

                        Self::Symbol(symbol) | Self::Unresolved(symbol) => {
                            let value = resolve(symbol.as_str()).ok_or_else(|| {
                                MemoryDepthError::UnboundSymbol(symbol.clone())
                            })?;

                            if value == 0 {
                                return Err(MemoryDepthError::ZeroBinding(
                                    symbol.clone(),
                                ));
                            }

                            values.push(value);
                        }

                        Self::Add(lhs, rhs) => {
                            tasks.push(Task::Add);
                            tasks.push(Task::Visit(rhs));
                            tasks.push(Task::Visit(lhs));
                        }

                        Self::Multiply(lhs, rhs) => {
                            tasks.push(Task::Multiply);
                            tasks.push(Task::Visit(rhs));
                            tasks.push(Task::Visit(lhs));
                        }
                    }
                }

                Task::Add | Task::Multiply => {
                    let rhs = values
                        .pop()
                        .ok_or(MemoryDepthError::InvalidEvaluationState)?;
                    let lhs = values
                        .pop()
                        .ok_or(MemoryDepthError::InvalidEvaluationState)?;

                    let result = match task {
                        Task::Add => lhs.checked_add(rhs).ok_or(
                            MemoryDepthError::ArithmeticOverflow {
                                operation: MemoryDepthArithmetic::Add,
                                lhs,
                                rhs,
                            },
                        )?,
                        Task::Multiply => lhs.checked_mul(rhs).ok_or(
                            MemoryDepthError::ArithmeticOverflow {
                                operation: MemoryDepthArithmetic::Multiply,
                                lhs,
                                rhs,
                            },
                        )?,
                        Task::Visit(_) => {
                            return Err(MemoryDepthError::InvalidEvaluationState);
                        }
                    };

                    if result == 0 {
                        return Err(MemoryDepthError::ZeroExtent);
                    }

                    values.push(result);
                }
            }
        }

        if values.len() != 1 {
            return Err(MemoryDepthError::InvalidEvaluationState);
        }

        values
            .pop()
            .ok_or(MemoryDepthError::InvalidEvaluationState)
    }
}

/// Operational budget for evaluating a memory extent expression.
///
/// This limits expression nodes visited during one evaluation. It does not
/// limit legal memory sizes or the number of dimensions in a memory type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryDepthBudget {
    /// Maximum number of expression nodes that may be visited.
    pub max_nodes: usize,
}

impl MemoryDepthBudget {
    /// Construct an explicit evaluation budget.
    #[must_use]
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }
}

/// Arithmetic operation that failed while evaluating a memory extent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryDepthArithmetic {
    /// Addition overflowed.
    Add,

    /// Multiplication overflowed.
    Multiply,
}

/// Failures while constructing or evaluating a memory extent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryDepthError {
    /// A concrete extent or resolved expression evaluated to zero.
    ZeroExtent,

    /// A symbol name was empty or whitespace-only.
    EmptySymbol,

    /// A symbol has no binding in the supplied resolver.
    UnboundSymbol(WidthSymbol),

    /// A resolver returned zero for a named extent.
    ZeroBinding(WidthSymbol),

    /// Checked extent arithmetic overflowed `u128`.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: MemoryDepthArithmetic,
        /// Left operand.
        lhs: u128,
        /// Right operand.
        rhs: u128,
    },

    /// Evaluation exceeded the configured node budget.
    BudgetExceeded {
        /// Configured maximum number of nodes.
        limit: usize,
        /// Number of nodes visited when exhaustion was detected.
        visited: usize,
    },

    /// The node counter exceeded its representable range.
    NodeCountOverflow,

    /// An internal evaluation stack invariant was violated.
    ///
    /// This indicates an implementation defect, not invalid user source.
    InvalidEvaluationState,
}

impl fmt::Display for MemoryDepthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroExtent => {
                formatter.write_str("memory dimension extent must be positive")
            }
            Self::EmptySymbol => {
                formatter.write_str("memory extent symbol cannot be empty")
            }
            Self::UnboundSymbol(symbol) => {
                write!(formatter, "no memory extent binding for `{symbol}`")
            }
            Self::ZeroBinding(symbol) => {
                write!(formatter, "memory extent binding `{symbol}` resolved to zero")
            }
            Self::ArithmeticOverflow {
                operation,
                lhs,
                rhs,
            } => {
                let operator = match operation {
                    MemoryDepthArithmetic::Add => "+",
                    MemoryDepthArithmetic::Multiply => "*",
                };
                write!(
                    formatter,
                    "memory extent arithmetic overflow: {lhs} {operator} {rhs}"
                )
            }
            Self::BudgetExceeded { limit, visited } => write!(
                formatter,
                "memory extent evaluation budget exceeded: limit {limit}, visited {visited}"
            ),
            Self::NodeCountOverflow => {
                formatter.write_str("memory extent evaluation node counter overflowed")
            }
            Self::InvalidEvaluationState => {
                formatter.write_str("invalid internal memory extent evaluation state")
            }
        }
    }
}

impl std::error::Error for MemoryDepthError {}

/// Logical access permissions of a memory port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryPortAccess {
    /// The port may read memory but may not write it.
    ReadOnly,

    /// The port may write memory but may not read it.
    WriteOnly,

    /// The port may both read and write memory.
    ReadWrite,
}

/// Clock edge used by a synchronous memory port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryClockEdge {
    /// Sample synchronous behavior on the rising edge.
    Rising,

    /// Sample synchronous behavior on the falling edge.
    Falling,
}

/// Clocking contract of a logical memory port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryPortClocking {
    /// The port has no clocked access contract.
    ///
    /// Whether asynchronous access is legal depends on the memory dialect
    /// and later verification.
    Asynchronous,

    /// The port is synchronous to a named logical clock.
    ///
    /// The name is an unresolved logical reference, not a physical clock pin.
    Synchronous {
        /// Name or semantic reference used to identify the logical clock.
        clock: String,

        /// Edge on which the synchronous port samples behavior.
        edge: MemoryClockEdge,
    },
}

impl MemoryPortClocking {
    /// Construct a synchronous port clocking contract.
    ///
    /// An empty clock reference is rejected. Name resolution is performed by
    /// the semantic/lowering layer, not by this type.
    pub fn synchronous(
        clock: impl Into<String>,
        edge: MemoryClockEdge,
    ) -> Result<Self, MemoryTypeError> {
        let clock = clock.into();
        if clock.trim().is_empty() {
            return Err(MemoryTypeError::EmptyClockReference);
        }

        Ok(Self::Synchronous { clock, edge })
    }
}

/// A named logical memory-port contract.
///
/// This describes the operations a port permits. It does not model a
/// physical pin, implementation-specific port count, or bus allocation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryPortType {
    name: String,
    access: MemoryPortAccess,
    clocking: MemoryPortClocking,
}

impl MemoryPortType {
    /// Construct a memory port with a name, access mode, and clocking contract.
    pub fn new(
        name: impl Into<String>,
        access: MemoryPortAccess,
        clocking: MemoryPortClocking,
    ) -> Result<Self, MemoryTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(MemoryTypeError::EmptyPortName);
        }

        if let MemoryPortClocking::Synchronous { clock, .. } = &clocking {
            if clock.trim().is_empty() {
                return Err(MemoryTypeError::EmptyClockReference);
            }
        }

        Ok(Self {
            name,
            access,
            clocking,
        })
    }

    /// Return the logical port name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the access permissions of this port.
    #[must_use]
    pub const fn access(&self) -> MemoryPortAccess {
        self.access
    }

    /// Return the clocking contract of this port.
    #[must_use]
    pub fn clocking(&self) -> &MemoryPortClocking {
        &self.clocking
    }
}

/// Read-during-write behavior when accesses collide at the same logical
/// memory location.
///
/// The selected behavior is part of the memory contract. It must not be
/// silently replaced by a target's default behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryCollisionBehavior {
    /// The prior stored value is observed.
    ReadFirst,

    /// The newly written value is observed.
    WriteFirst,

    /// The read output retains its prior value.
    NoChange,

    /// The behavior is unspecified by this type and must be resolved by
    /// the applicable language, dialect, or downstream contract.
    Unspecified,

    /// A colliding access is semantically invalid and must be diagnosed
    /// by the relevant verifier or execution model.
    ErrorOnCollision,
}

/// Initialization intent for a logical memory.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryInitialization {
    /// No initialization behavior is specified by this type.
    Unspecified,

    /// Every logical element is initialized to the all-zero value.
    ///
    /// The element type and downstream semantics must define how this
    /// initialization is represented and whether a target can implement it.
    ZeroInitialized,

    /// Initialization is supplied by a named semantic parameter or artifact.
    ///
    /// This reference is not resolved by the type constructor.
    Symbolic(String),
}

impl MemoryInitialization {
    /// Construct symbolic initialization intent using a non-empty reference.
    pub fn symbolic(reference: impl Into<String>) -> Result<Self, MemoryTypeError> {
        let reference = reference.into();
        if reference.trim().is_empty() {
            return Err(MemoryTypeError::EmptyInitializationReference);
        }
        Ok(Self::Symbolic(reference))
    }
}

/// Target-independent logical memory type.
///
/// `dimensions` describes one or more extents. A one-dimensional memory has
/// one dimension; a multidimensional memory retains each dimension separately
/// instead of forcing its total element count into a fixed-size product.
///
/// This is a type contract, not a physical allocation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryType {
    element_type: SignalType,
    dimensions: Vec<MemoryDepth>,
    ports: Vec<MemoryPortType>,
    initialization: MemoryInitialization,
    collision_behavior: MemoryCollisionBehavior,
}

impl MemoryType {
    /// Construct a logical memory type.
    ///
    /// The dimension list must not be empty. Port names must be unique within
    /// this memory type. No universal dimension or port-count maximum is
    /// imposed. The order of dimensions and ports is preserved.
    pub fn new(
        element_type: SignalType,
        dimensions: Vec<MemoryDepth>,
        ports: Vec<MemoryPortType>,
        initialization: MemoryInitialization,
        collision_behavior: MemoryCollisionBehavior,
    ) -> Result<Self, MemoryTypeError> {
        if dimensions.is_empty() {
            return Err(MemoryTypeError::MissingDimensions);
        }

        for (index, port) in ports.iter().enumerate() {
            if ports[..index]
                .iter()
                .any(|previous| previous.name == port.name)
            {
                return Err(MemoryTypeError::DuplicatePortName(
                    port.name.clone(),
                ));
            }
        }

        if let MemoryInitialization::Symbolic(reference) = &initialization {
            if reference.trim().is_empty() {
                return Err(MemoryTypeError::EmptyInitializationReference);
            }
        }

        Ok(Self {
            element_type,
            dimensions,
            ports,
            initialization,
            collision_behavior,
        })
    }

    /// Return the logical type of each memory element.
    #[must_use]
    pub const fn element_type(&self) -> &SignalType {
        &self.element_type
    }

    /// Return the ordered symbolic or concrete dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[MemoryDepth] {
        &self.dimensions
    }

    /// Return the ordered logical port contracts.
    #[must_use]
    pub fn ports(&self) -> &[MemoryPortType] {
        &self.ports
    }

    /// Return the memory initialization intent.
    #[must_use]
    pub const fn initialization(&self) -> &MemoryInitialization {
        &self.initialization
    }

    /// Return the declared read-during-write collision behavior.
    #[must_use]
    pub const fn collision_behavior(&self) -> MemoryCollisionBehavior {
        self.collision_behavior
    }

    /// Return whether this type contains any symbolic or unresolved dimension.
    #[must_use]
    pub fn has_symbolic_dimensions(&self) -> bool {
        self.dimensions
            .iter()
            .any(|dimension| !matches!(dimension, MemoryDepth::Known(_)))
    }

    /// Evaluate each dimension independently using a shared resolver and
    /// per-dimension node budget.
    ///
    /// The result preserves dimensionality. It intentionally does not
    /// multiply dimensions into a total element count, because that product
    /// may overflow even when each individual dimension is representable.
    pub fn evaluate_dimensions(
        &self,
        budget: MemoryDepthBudget,
        mut resolve: impl FnMut(&str) -> Option<u128>,
    ) -> Result<Vec<u128>, MemoryDepthError> {
        self.dimensions
            .iter()
            .map(|dimension| {
                dimension.evaluate_with_budget(budget, |symbol| resolve(symbol))
            })
            .collect()
    }
}

/// Errors constructing a logical memory type or one of its contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryTypeError {
    /// A memory type must declare at least one dimension.
    MissingDimensions,

    /// A port name is empty or whitespace-only.
    EmptyPortName,

    /// Two ports have the same name in one memory type.
    DuplicatePortName(String),

    /// A synchronous port refers to an empty clock name.
    EmptyClockReference,

    /// A symbolic initialization reference is empty.
    EmptyInitializationReference,
}

impl fmt::Display for MemoryTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDimensions => {
                formatter.write_str("a logical memory type requires at least one dimension")
            }
            Self::EmptyPortName => {
                formatter.write_str("memory port name cannot be empty")
            }
            Self::DuplicatePortName(name) => {
                write!(formatter, "duplicate logical memory port name `{name}`")
            }
            Self::EmptyClockReference => {
                formatter.write_str("synchronous memory port clock reference cannot be empty")
            }
            Self::EmptyInitializationReference => {
                formatter.write_str("memory initialization reference cannot be empty")
            }
        }
    }
}

impl std::error::Error for MemoryTypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_element_type() -> SignalType {
        // A named semantic type avoids assumptions about a concrete logic
        // constructor while exercising memory shape and port contracts.
        SignalType::named(
            crate::frontend::semantic::domains::hdl::HdlId::new("Word")
                .expect("test identifier is non-empty"),
        )
    }

    #[test]
    fn concrete_depth_must_be_positive() {
        assert!(matches!(
            MemoryDepth::known(0),
            Err(MemoryDepthError::ZeroExtent)
        ));
        assert_eq!(MemoryDepth::known(8).unwrap().as_known(), Some(8));
    }

    #[test]
    fn symbolic_depth_is_evaluated_with_checked_arithmetic() {
        let depth = MemoryDepth::multiply(
            MemoryDepth::symbol("ROWS").unwrap(),
            MemoryDepth::known(4).unwrap(),
        );

        let evaluated = depth
            .evaluate_with_budget(MemoryDepthBudget::new(8), |name| {
                (name == "ROWS").then_some(16)
            })
            .unwrap();

        assert_eq!(evaluated, 64);
    }

    #[test]
    fn missing_symbol_is_an_explicit_error() {
        let depth = MemoryDepth::symbol("DEPTH").unwrap();

        assert!(matches!(
            depth.evaluate_with_budget(MemoryDepthBudget::new(8), |_| None),
            Err(MemoryDepthError::UnboundSymbol(_))
        ));
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let depth = MemoryDepth::multiply(
            MemoryDepth::known(u128::MAX).unwrap(),
            MemoryDepth::known(2).unwrap(),
        );

        assert!(matches!(
            depth.evaluate_with_budget(MemoryDepthBudget::new(8), |_| None),
            Err(MemoryDepthError::ArithmeticOverflow {
                operation: MemoryDepthArithmetic::Multiply,
                ..
            })
        ));
    }

    #[test]
    fn evaluation_budget_is_enforced() {
        let depth = MemoryDepth::add(
            MemoryDepth::known(2).unwrap(),
            MemoryDepth::known(3).unwrap(),
        );

        assert!(matches!(
            depth.evaluate_with_budget(MemoryDepthBudget::new(1), |_| None),
            Err(MemoryDepthError::BudgetExceeded { .. })
        ));
    }

    #[test]
    fn port_names_must_be_unique() {
        let read_port = MemoryPortType::new(
            "data",
            MemoryPortAccess::ReadOnly,
            MemoryPortClocking::Asynchronous,
        )
        .unwrap();

        let write_port = MemoryPortType::new(
            "data",
            MemoryPortAccess::WriteOnly,
            MemoryPortClocking::Asynchronous,
        )
        .unwrap();

        let result = MemoryType::new(
            test_element_type(),
            vec![MemoryDepth::known(32).unwrap()],
            vec![read_port, write_port],
            MemoryInitialization::Unspecified,
            MemoryCollisionBehavior::Unspecified,
        );

        assert!(matches!(
            result,
            Err(MemoryTypeError::DuplicatePortName(_))
        ));
    }

    #[test]
    fn symbolic_dimensions_are_preserved() {
        let memory = MemoryType::new(
            test_element_type(),
            vec![
                MemoryDepth::symbol("ROWS").unwrap(),
                MemoryDepth::symbol("COLUMNS").unwrap(),
            ],
            Vec::new(),
            MemoryInitialization::ZeroInitialized,
            MemoryCollisionBehavior::ReadFirst,
        )
        .unwrap();

        assert!(memory.has_symbolic_dimensions());
        assert_eq!(memory.dimensions().len(), 2);
    }

    #[test]
    fn dimensions_are_evaluated_without_forcing_a_product() {
        let memory = MemoryType::new(
            test_element_type(),
            vec![
                MemoryDepth::symbol("ROWS").unwrap(),
                MemoryDepth::symbol("COLUMNS").unwrap(),
            ],
            Vec::new(),
            MemoryInitialization::Unspecified,
            MemoryCollisionBehavior::Unspecified,
        )
        .unwrap();

        let dimensions = memory
            .evaluate_dimensions(MemoryDepthBudget::new(4), |name| match name {
                "ROWS" => Some(1_000),
                "COLUMNS" => Some(2_000),
                _ => None,
            })
            .unwrap();

        assert_eq!(dimensions, vec![1_000, 2_000]);
    }

    #[test]
    fn synchronous_port_requires_a_clock_reference() {
        assert!(matches!(
            MemoryPortClocking::synchronous("  ", MemoryClockEdge::Rising),
            Err(MemoryTypeError::EmptyClockReference)
        ));

        assert!(MemoryPortClocking::synchronous("core_clk", MemoryClockEdge::Rising).is_ok());
    }

    #[test]
    fn empty_dimension_list_is_rejected() {
        assert!(matches!(
            MemoryType::new(
                test_element_type(),
                Vec::new(),
                Vec::new(),
                MemoryInitialization::Unspecified,
                MemoryCollisionBehavior::Unspecified,
            ),
            Err(MemoryTypeError::MissingDimensions)
        ));
    }
}
