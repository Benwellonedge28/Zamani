
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Type System
//!
//! Path: `src/hdl/hdlir/types/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Compose and expose the type descriptors owned by Zamani's hardware-domain
//! intermediate representation (HDLIR).
//!
//! This module is the public type-system boundary for `src/hdl/hdlir/`.
//! It registers the individual type modules and provides deliberate,
//! discoverable re-exports for downstream HDLIR components.
//!
//! ## Authority
//!
//! The implementation must remain consistent with:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/Zamani.g4`.
//! - `grammar/hdl/hdl.g4` and the applicable HDL delegate grammars.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/types/logic.rs`.
//! - `src/hdl/hdlir/types/signal.rs`.
//! - `src/hdl/hdlir/types/aggregate.rs`.
//! - `src/hdl/hdlir/types/clock.rs`.
//! - `src/hdl/hdlir/types/memory.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//!
//! Grammar and the canonical frontend semantic model remain authoritative
//! for source syntax and source-level meaning. This module exposes HDLIR
//! representations; it does not replace those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the HDLIR type-system submodules.
//! - The public aggregation of their intended APIs.
//! - The separation between type descriptors and their implementations.
//!
//! The individual sibling modules own their type definitions and invariants.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, name resolution, or semantic analysis.
//! - A second copy of canonical frontend types or source spans.
//! - Design graphs, module ports, operations, builders, or verification.
//! - Runtime values, simulation execution, synthesis, or physical allocation.
//! - Vendor-specific types unless introduced through an explicitly defined
//!   and registered extension contract.
//!
//! ## Module responsibilities
//!
//! - `bit_width`: concrete and symbolic bit-width expressions and evaluation.
//! - `logic`: logic type, signedness, state-model, and type compatibility.
//! - `signal`: logical signal-type classification and compatibility.
//! - `aggregate`: aggregate extents, element types, fields, and compatibility.
//! - `clock`: logical clock and clock-domain descriptors.
//! - `memory`: symbolic memory dimensions, port contracts, and memory intent.
//!
//! These modules may depend on each other's public contracts. Their
//! definitions must not be copied into this file or into downstream modules.
//!
//! ## Scalability and resource policy
//!
//! This module introduces no universal hardware-capacity constants and no
//! fixed maximum for widths, aggregate dimensions, fields, clocks, memory
//! dimensions, or memory ports.
//!
//! Symbolic quantities remain symbolic until an appropriate stage evaluates
//! them. Concrete arithmetic and conversions must use the checked behavior
//! defined by the owning module.
//!
//! Operational budgets limit compiler work; they must not silently redefine
//! source-language semantics or impose universal hardware ceilings.
//! An exhausted budget or unrepresentable concrete value must remain an
//! explicit incomplete result or error.
//!
//! No finite implementation can process literally infinite input. The
//! design goal is to support representable designs within available resources,
//! supported semantics, and explicit operational budgets.
//!
//! ## Integration contract
//!
//! - `src/hdl/hdlir/mod.rs` may register this module once the parent module
//!   has valid Rust contents.
//! - HDLIR graph types consume these descriptors through this module's
//!   public API or through explicit sibling-module paths.
//! - `lowering/from_semantic.rs` maps validated canonical semantic facts
//!   into these types using the actual frontend APIs.
//! - `verify/types.rs` consumes type compatibility and evaluation results.
//!   Indeterminate compatibility must not be treated as proven compatibility.
//! - `ir/ports.rs`, `ir/connectivity.rs`, `ir/clocks.rs`, and
//!   `ir/memories.rs` consume the relevant descriptors when those modules
//!   are implemented.
//! - Serialization must preserve symbolic expressions and all semantically
//!   relevant variants rather than flattening or guessing their meaning.
//!
//! The re-exports here are not a promise that these future consumers already
//! exist or that frontend-to-HDLIR lowering is complete.
//!
//! ## API stability
//!
//! Re-exports intentionally expose the existing sibling APIs without
//! inventing convenience constructors or alternate representations.
//! Adding a new public type requires defining its owner and contract first.
//! Removing or changing a public re-export must follow the HDLIR
//! compatibility policy.
//!
//! ## Completion criteria
//!
//! - Every declared module exists and is included exactly once.
//! - Public re-exports match the actual sibling declarations.
//! - No duplicate type definitions or competing semantic authorities exist.
//! - The module compiles with the repository's Rust 1.97+ toolchain.
//! - Formatting, focused tests, repository tests, and Clippy pass after the
//!   parent module and sibling integration have been corrected.

// -----------------------------------------------------------------------------
// Type implementation modules
// -----------------------------------------------------------------------------

/// Concrete and symbolic bit-width expressions and checked evaluation.
pub mod bit_width;

/// Logic types, signedness, state models, and compatibility.
pub mod logic;

/// Logical signal-type descriptors and compatibility.
pub mod signal;

/// Aggregate extents, fields, element types, and aggregate compatibility.
pub mod aggregate;

/// Logical clock and clock-domain descriptors.
pub mod clock;

/// Symbolic memory dimensions, memory ports, and memory semantics.
pub mod memory;

// -----------------------------------------------------------------------------
// Bit widths
// -----------------------------------------------------------------------------

/// Canonical HDLIR bit-width types and evaluation contracts.
pub use self::bit_width::{
    BitWidth,
    WidthArithmetic,
    WidthEvaluationBudget,
    WidthEvaluationError,
    WidthSymbol,
};

// -----------------------------------------------------------------------------
// Logic types
// -----------------------------------------------------------------------------

/// Canonical HDLIR logic-type descriptors and compatibility contracts.
pub use self::logic::{
    LogicSignedness,
    LogicStateModel,
    LogicType,
    LogicTypeCompatibility,
    LogicTypeError,
    WidthConversionBudget,
};

// -----------------------------------------------------------------------------
// Signal types
// -----------------------------------------------------------------------------

/// Canonical HDLIR logical signal-type descriptors and compatibility.
pub use self::signal::{
    SignalType,
    SignalTypeCompatibility,
    SignalTypeError,
};

// -----------------------------------------------------------------------------
// Aggregate types
// -----------------------------------------------------------------------------

/// Canonical HDLIR aggregate descriptors and extent-evaluation contracts.
pub use self::aggregate::{
    AggregateField,
    AggregateLength,
    AggregateLengthBudget,
    AggregateLengthError,
    AggregateType,
    AggregateTypeCompatibility,
    AggregateTypeError,
    AggregateValueType,
};

// -----------------------------------------------------------------------------
// Clock types
// -----------------------------------------------------------------------------

/// Canonical HDLIR logical clock and clock-domain descriptors.
pub use self::clock::{
    ClockDomain,
    ClockDomainError,
    ClockEdge,
    ClockType,
    ClockTypeError,
};

// -----------------------------------------------------------------------------
// Memory types
// -----------------------------------------------------------------------------

/// Canonical HDLIR memory descriptors and dimension-evaluation contracts.
pub use self::memory::{
    MemoryClockEdge,
    MemoryCollisionBehavior,
    MemoryDepth,
    MemoryDepthArithmetic,
    MemoryDepthBudget,
    MemoryDepthError,
    MemoryInitialization,
    MemoryPortAccess,
    MemoryPortClocking,
    MemoryPortType,
    MemoryType,
    MemoryTypeError,
};
