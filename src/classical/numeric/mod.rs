
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani — Classical Numerical Computing
//!
//! File: `src/classical/numeric/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module is the public composition boundary for concrete classical
//! numerical operations implemented by Zamani.
//!
//! It exposes arithmetic, elementary mathematical functions, linear algebra,
//! statistics, and numerical reproducibility through their owning modules.
//!
//! # Architectural authority
//!
//! Normative semantics:
//!   `grammar/spec/classical.md`
//!
//! Numerical-domain syntax:
//!   `grammar/classical/numeric.g4`
//!   `grammar/classical/numerical.g4`
//!
//! Arithmetic-expression syntax and precedence:
//!   `grammar/expressions/arithmetic.g4`
//!   `grammar/expressions/expressions.g4`
//!
//! Canonical floating-point representation:
//!   `crate::quantum::ir::classical::float`
//!
//! Canonical integer representation:
//!   `crate::quantum::ir::classical::integer`
//!
//! Canonical collection ownership:
//!   `crate::classical::collections`
//!
//! # Responsibilities
//!
//! This module owns:
//! - public registration of the numerical implementation modules;
//! - the numerical module namespace;
//! - module-level documentation and integration boundaries;
//! - the invariant that numerical implementations remain independently owned.
//!
//! It does not own:
//! - numeric representations, precision, or integer-width semantics;
//! - implicit conversions, promotion, or rounding policy;
//! - grammar, tokens, AST construction, or expression precedence;
//! - a second classical or quantum IR;
//! - symbolic-expression representation;
//! - memory allocation policy or resource discovery;
//! - hardware-specific implementations or target selection;
//! - execution scheduling or distributed reduction orchestration.
//!
//! # Module map
//!
//! - `arithmetic`: checked concrete arithmetic.
//! - `elementary`: checked elementary mathematical functions.
//! - `linear_algebra`: concrete linear-algebra algorithms.
//! - `statistics`: descriptive and inferential statistics.
//! - `reproducibility`: numerical reproducibility policies and reductions.
//!
//! Each child module owns its operation-specific API and error types.
//! This module deliberately does not flatten those APIs into a single
//! namespace, avoiding collisions between independently meaningful types
//! and preserving the existing public paths.
//!
//! # POCO-REAF and scalability
//!
//! The numerical namespace introduces no universal semantic limits on:
//! - hardware, processors, accelerators, or distributed workers;
//! - numeric widths supported by the canonical numeric representation;
//! - collection lengths or matrix dimensions;
//! - the number of numerical operations in a program;
//! - future numeric algorithms or execution targets.
//!
//! Concrete computations remain bounded by representable dimensions,
//! available memory, computation time, canonical numeric semantics, and
//! explicit compiler/runtime resource policies.
//!
//! This module does not promise physically infinite computation or storage.
//! It avoids imposing artificial architectural ceilings on valid programs.
//!
//! # Integration contract
//!
//! 1. `src/classical/mod.rs` must expose this module with `pub mod numeric;`
//!    while preserving its other valid module declarations.
//! 2. Semantic analysis must validate types, conversions, effects, and
//!    operation legality before concrete evaluation or lowering.
//! 3. Frontend operator nodes must be mapped to the appropriate operation
//!    APIs; grammar rules must not be reimplemented here.
//! 4. Concrete floating-point operations must use the canonical
//!    `ClassicalFloat` representation.
//! 5. Concrete integer operations must use the canonical integer types.
//! 6. Matrix operations must use the existing collection implementation.
//! 7. Symbolic expressions must remain in their existing semantic/IR owner.
//! 8. Lowering and optimization must preserve the language's observable
//!    numerical semantics and applicable reproducibility guarantees.
//! 9. Resource failures must be explicit; implementations must not silently
//!    truncate data or wrap size calculations.
//! 10. A child module may be registered here only when its implementation
//!     exists at the declared path and its API contract is established.
//!
//! # Numerical error ownership
//!
//! Operation-specific errors remain owned by their implementation modules.
//! A shared numerical error facade may be added only after its file exists,
//! its error-code contract is frozen, and conversions from the actual child
//! error types are verified. This module must not reference a nonexistent
//! `numerical_error.rs`.
//!
//! # Compatibility and freeze rule
//!
//! Existing child-module paths are public integration points. Renaming,
//! removing, or flattening those paths requires an explicit compatibility
//! review and downstream migration plan.
//!
//! Adding a new child module requires its own contract, tests, and completed
//! implementation before it is registered here.

/// Checked concrete arithmetic over canonical Zamani numeric types.
pub mod arithmetic;

/// Checked elementary mathematical functions over canonical numeric values.
pub mod elementary;

/// Concrete linear-algebra algorithms using the existing collection layer.
pub mod linear_algebra;

/// Descriptive and inferential statistics over canonical numeric values.
pub mod statistics;

/// Explicit numerical reproducibility policies and deterministic reductions.
pub mod reproducibility;
