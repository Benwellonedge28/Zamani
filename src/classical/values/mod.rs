
```rust
//! Zamani — Classical Values Module
//!
//! Path: src/classical/values/mod.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!   Module-boundary implementation.
//!   Workspace compilation and complete integration verification are still
//!   required before this module can be marked production-verified.
//!
//! PURPOSE
//!   Provide the stable classical-facing entry point for value operations,
//!   conversion policies, comparison semantics, and canonical value types.
//!
//! ARCHITECTURAL AUTHORITY
//!   Language semantics and architectural constraints are governed by:
//!     - grammar/DESIGN.md
//!     - grammar/specification/semantic-model.md
//!     - grammar/specification/scalability-model.md
//!     - grammar/specification/poco-reaf.md
//!     - grammar/lexer/literals.md
//!
//! CANONICAL OWNERSHIP
//!   The existing canonical value representation is owned by:
//!     crate::quantum::ir::classical::value
//!
//!   The canonical arbitrary-width integer representation is owned by:
//!     crate::quantum::ir::classical::integer
//!
//!   The canonical floating-point implementation is owned by:
//!     crate::quantum::ir::classical::float
//!
//!   This module exposes those implementations through the classical
//!   subsystem. It does not create a competing value enum, integer engine,
//!   floating-point representation, type system, AST, IR, or runtime.
//!
//! RESPONSIBILITIES
//!   - Register existing classical value adapter modules.
//!   - Expose canonical value types without redefining them.
//!   - Expose explicit conversion policies and checked conversion APIs.
//!   - Expose comparison results and comparison operations.
//!   - Establish one discoverable import boundary for classical consumers.
//!
//! DOES NOT OWN
//!   - Source syntax, tokenization, literal parsing, or AST construction.
//!   - Source-language type inference or implicit-conversion rules.
//!   - Canonical integer or floating-point arithmetic implementations.
//!   - Decimal, rational, fixed-point, or complex arithmetic engines.
//!   - Symbolic algebra or parameter evaluation.
//!   - Collection storage, memory allocation, or ownership checking.
//!   - Execution, scheduling, placement, hardware capabilities, or backends.
//!   - Canonical IR serialization, hashing, or identity.
//!
//! DEPENDENCY DIRECTION
//!
//!   grammar / lexer / parser
//!              |
//!              v
//!       domain-neutral AST
//!              |
//!              v
//!       semantic analysis
//!              |
//!              v
//!       canonical semantic IR
//!              |
//!              v
//!       classical::values
//!              |
//!              v
//!   validation / lowering / execution
//!
//!   The values module must not depend on the frontend or on downstream
//!   execution and hardware implementations.
//!
//! POCO-REAF AND SCALABILITY
//!   This module introduces no universal semantic ceiling for:
//!     - integer width;
//!     - the number of values;
//!     - collection length;
//!     - tensor dimensions or rank;
//!     - processor, accelerator, or device counts;
//!     - operations, tasks, or distributed workers.
//!
//!   Actual canonical value representations, compiler/runtime policies,
//!   target capabilities, and available resources determine what can be
//!   represented and realized. Resource exhaustion and representation
//!   failures must be explicit; they must never cause silent truncation,
//!   corruption, or a change in program meaning.
//!
//!   This is not a promise of physically infinite resources or of arbitrary
//!   computation completing on every target.
//!
//! MODULE REGISTRATION POLICY
//!   Only modules whose source files exist are declared below. A planned
//!   module must not be declared before its implementation file exists.
//!   Add future value modules through a separately reviewed integration
//!   change after their ownership contracts and canonical dependencies are
//!   frozen.
//!
//! INTEGRATION CONTRACT
//!   - src/classical/mod.rs must declare `pub mod values;`.
//!   - Existing canonical IR value types remain the single source of truth.
//!   - Callers must use explicit conversion policies where conversion may
//!     lose precision, round, or overflow.
//!   - Semantic analysis must decide which source-level operations are legal.
//!   - Comparison must not invent implicit numeric promotions.
//!   - Runtime memory and execution modules may consume these APIs but must
//!     not redefine their semantic contracts.
//!   - Grammar changes do not automatically change value semantics.
//!   - New public exports require compatibility and downstream-use review.
//!
//! VERIFICATION
//!   This file is not considered verified until the actual repository passes
//!   the applicable formatting, build, test, and lint checks, and its
//!   consumers have been checked against the exported APIs.
//!
//! FREEZE RULE
//!   Changing canonical ownership, public exports, conversion behavior,
//!   comparison semantics, or compatibility guarantees requires an explicit
//!   dependency and semantic compatibility review.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -------------------------------------------------------------------------
// Classical-facing adapters
// -------------------------------------------------------------------------
//
// These modules must remain adapters around canonical implementations.
// They must not introduce competing numeric representations or semantics.

pub mod comparison;
pub mod conversion;
pub mod float;
pub mod integer;

// -------------------------------------------------------------------------
// Canonical classical value representations
// -------------------------------------------------------------------------
//
// Re-export the exact canonical types used by the existing IR. These are
// aliases through Rust's module system, not new type definitions.
//
// Keeping the canonical representations visible here gives classical
// consumers a concise import path while preserving one source of truth.

pub use crate::quantum::ir::classical::value::{
    BigInt,
    BigUint,
    BitVector,
    ClassicalValue,
    ClassicalValueError,
    ClassicalValueKind,
    ComplexValue,
    FiniteFloat,
};

// -------------------------------------------------------------------------
// Comparison API
// -------------------------------------------------------------------------
//
// The comparison module owns comparison outcomes and policy. The canonical
// value module continues to own the values being compared.

pub use self::comparison::{
    compare,
    semantic_equal,
    ComparisonError,
    ComparisonResult,
    EqualityResult,
};

// -------------------------------------------------------------------------
// Explicit conversion API
// -------------------------------------------------------------------------
//
// Conversion policies make overflow, rounding, and precision-loss choices
// visible. Fixed-width Rust types remain interoperability destinations,
// not universal Zamani value limits.

pub use self::conversion::{
    ConversionError,
    ConversionPolicy,
    OverflowPolicy,
    PrecisionPolicy,
    RoundingMode,
    TryConvertTo,
};

// -------------------------------------------------------------------------
// Canonical numeric adapters
// -------------------------------------------------------------------------
//
// These are separate adapter names from FiniteFloat and the BigInt/BigUint
// representations exposed above. Do not silently substitute one type for
// another; the canonical IR remains authoritative.

pub use self::float::{
    ClassicalFloat,
    ClassicalFloatError,
    FloatFormat,
};

pub use self::integer::{
    IntegerError,
    IntegerValue,
    SignedInteger,
    UnsignedInteger,
};
```
