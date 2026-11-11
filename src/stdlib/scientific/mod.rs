
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Scientific Computing Standard Library
//!
//! File:
//!     src/stdlib/scientific/mod.rs
//!
//! Language:
//!     Zamani
//!
//! Rust edition:
//!     2021
//!
//! Minimum Rust version:
//!     1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! # 1. Purpose
//!
//! This module is the public composition root for scientific computing
//! within the Zamani Standard Library.
//!
//! It exposes portable scientific-computing APIs for:
//!
//! - Classical numerical computing.
//! - Linear algebra.
//! - Statistical analysis.
//! - Probability and stochastic computation.
//! - Mathematical optimization.
//! - Scientific simulation.
//! - Numerical and mathematical modeling.
//! - Reproducible scientific computation.
//! - Scientific applications integrated with AI.
//! - Quantum-classical scientific applications.
//! - Hardware/software co-design.
//! - Distributed scientific computation.
//! - Future scientific-computing domains.
//!
//! The module provides stable public paths to the existing scientific
//! implementations without introducing competing mathematical,
//! numerical, storage, semantic, or execution authorities.
//!
//! # 2. Architectural position
//!
//! The scientific standard-library subsystem participates in the
//! following architecture:
//!
//!     Zamani source
//!          |
//!          v
//!     Canonical lexer and parser
//!          |
//!          v
//!     Frontend AST
//!          |
//!          v
//!     Semantic analysis
//!          |
//!          +-------------------------------+
//!          |               |               |
//!          v               v               v
//!        Types           Effects        Resources
//!          |               |               |
//!          +---------------+---------------+
//!                          |
//!                          v
//!                Scientific semantic model
//!                          |
//!          +---------------+---------------+
//!          |               |               |
//!          v               v               v
//!     Classical IR     Tensor/data IR   Domain metadata
//!          |               |               |
//!          +---------------+---------------+
//!                          |
//!                          v
//!                 Compiler optimization
//!                          |
//!                          v
//!               Feasibility and planning
//!                          |
//!                          v
//!                  Lowering and scheduling
//!                          |
//!             +------------+-------------+
//!             |            |             |
//!             v            v             v
//!           CPU          GPU/FPGA      Distributed
//!             |            |             |
//!             +------------+-------------+
//!                          |
//!                          v
//!                 Runtime and HAL
//!                          |
//!                          v
//!                Actual execution
//!
//! Quantum, HDL, AI and hybrid computation integrate through their
//! respective authoritative subsystems.
//!
//! This module does not implement any of those compiler or runtime
//! stages.
//!
//! # 3. Canonical ownership
//!
//! Scientific computation MUST reuse the existing repository authorities.
//!
//! ## Numerical algorithms
//!
//! Classical numerical algorithms:
//!     crate::classical::numeric
//!
//! Linear algebra:
//!     crate::classical::numeric::linear_algebra
//!
//! Statistics:
//!     crate::classical::numeric::statistics
//!
//! ## Numeric representations
//!
//! Canonical classical floating-point representation:
//!     crate::quantum::ir::classical::float
//!
//! Canonical integer representation:
//!     crate::quantum::ir::classical::integer
//!
//! ## Data and storage
//!
//! Canonical collections:
//!     crate::classical::collections
//!
//! Canonical matrix storage:
//!     crate::classical::collections::matrix
//!
//! Canonical vector storage:
//!     crate::classical::collections::vector
//!
//! Canonical tensor storage:
//!     crate::classical::collections::tensor
//!
//! ## Compiler and execution
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Universal intermediate representation:
//!     crate::zuir
//!
//! Compiler optimization:
//!     crate::compiler::optimization
//!
//! Runtime execution:
//!     crate::runtime
//!
//! These authorities must remain independent of this module.
//!
//! # 4. Module ownership
//!
//! This module owns:
//!
//! - Scientific module registration.
//! - Scientific namespace composition.
//! - Scientific public API organization.
//! - Scientific module-level documentation.
//! - Integration boundaries between scientific modules.
//! - Compatibility of scientific standard-library paths.
//!
//! This module MUST NOT own:
//!
//! - Another numeric type system.
//! - Another floating-point representation.
//! - Another integer representation.
//! - Another matrix implementation.
//! - Another vector implementation.
//! - Another tensor storage implementation.
//! - Another collection framework.
//! - Another scientific AST.
//! - Another scientific semantic model.
//! - Another intermediate representation.
//! - Another compiler optimization system.
//! - Another automatic differentiation engine.
//! - Another runtime.
//! - Another hardware abstraction layer.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Execution scheduling.
//! - Device selection.
//! - Distributed execution.
//! - Quantum circuit execution.
//! - HDL synthesis or hardware placement.
//!
//! Each of these responsibilities belongs to an existing authoritative
//! subsystem or a separately specified future subsystem.
//!
//! # 5. POCO-REAF and scalability
//!
//! Scientific APIs MUST remain independent of the machine on which
//! a program is compiled or executed.
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Matrix dimensions.
//! - Vector lengths.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Statistical observations.
//! - Optimization variables.
//! - Optimization objectives.
//! - Optimization constraints.
//! - Simulation steps.
//! - Simulation events.
//! - Numerical operations.
//! - CPU counts.
//! - GPU counts.
//! - FPGA counts.
//! - QPU counts.
//! - Distributed participants.
//! - Memory capacity.
//! - Scientific providers.
//! - Future computational domains.
//!
//! Concrete operations remain subject to:
//!
//! - Representable integer and numeric dimensions.
//! - Available memory.
//! - Available computation time.
//! - Actual provider capabilities.
//! - Numeric precision.
//! - Explicit execution budgets.
//! - Semantic and operational constraints.
//!
//! No finite implementation can promise literal infinite physical
//! memory or computation.
//!
//! The objective is to avoid artificial architectural ceilings and
//! preserve the same program meaning across supported execution scales.
//!
//! # 6. Resource and capability separation
//!
//! Scientific computation must preserve the distinction between:
//!
//! Requirement:
//!     What an operation needs to be performed correctly.
//!
//! Capability:
//!     What an available implementation or provider supports.
//!
//! Preference:
//!     What execution characteristics are desirable.
//!
//! Constraint:
//!     What execution must not violate.
//!
//! Hint:
//!     Optional information that may improve optimization.
//!
//! Feasibility:
//!     Whether the requested operation can actually be performed.
//!
//! These concepts are established by the semantic, compiler and
//! runtime authorities.
//!
//! Scientific algorithms MUST NOT silently weaken requirements,
//! reinterpret constraints as preferences, or claim unsupported
//! operations succeeded.
//!
//! Optional execution budgets are operational policies, not
//! universal language-level limits.
//!
//! # 7. Module integration contracts
//!
//! The following child modules own their respective scientific APIs.
//!
//! linear_algebra:
//!     Canonical scientific linear-algebra facade.
//!     Reuses classical numeric algorithms and matrix storage.
//!
//! statistics:
//!     Canonical scientific statistical facade.
//!     Reuses classical statistics and numeric representations.
//!
//! optimization:
//!     Mathematical optimization problems, objectives, constraints,
//!     solver contracts, execution budgets and results.
//!
//! probability:
//!     Probability mathematics, distributions, sampling and
//!     stochastic computation contracts.
//!
//! simulation:
//!     Scientific simulation models, requests, execution contexts,
//!     cancellation, budgets and simulation engine integration.
//!
//! The child modules retain their individual error types and
//! public APIs.
//!
//! This root deliberately does not flatten all child exports into
//! one namespace. This prevents ambiguous names and preserves
//! independent error and type ownership.
//!
//! # 8. Cross-domain integration
//!
//! Scientific APIs may be consumed by:
//!
//! Classical computing:
//!     crate::classical
//!
//! Artificial intelligence:
//!     crate::ai
//!
//! Quantum computing:
//!     crate::quantum
//!
//! Hardware description:
//!     crate::hdl
//!
//! Hybrid computing:
//!     crate::hybrid
//!
//! Distributed computing:
//!     crate::distributed
//!
//! Universal intermediate representation:
//!     crate::zuir
//!
//! Compiler:
//!     crate::compiler
//!
//! Runtime:
//!     crate::runtime
//!
//! These integrations must use the canonical interfaces owned by
//! their respective subsystems.
//!
//! For example, an optimization problem may be used by a quantum
//! application, but this module must not implement quantum circuit
//! optimization.
//!
//! A simulation may use distributed execution, but this module
//! must not create a distributed scheduler.
//!
//! A scientific tensor may be processed by an AI model, but this
//! module must not introduce competing tensor storage.
//!
//! # 9. Grammar integration
//!
//! Scientific source syntax is governed by the canonical Zamani
//! grammar and its composed domain grammar.
//!
//! Relevant grammar specifications include:
//!
//!     grammar/classical/scientific-computing.g4
//!     grammar/classical/linear-algebra.g4
//!     grammar/classical/numeric.g4
//!     grammar/classical/tensor.g4
//!     grammar/spec/classical.md
//!
//! Scientific operations normally use ordinary Zamani expressions
//! and function calls.
//!
//! Their meaning is determined by semantic analysis, operation
//! resolution, operand types, shapes, capabilities, effects,
//! resource requirements and applicable policies.
//!
//! This module does not:
//!
//! - Parse Zamani source.
//! - Define grammar rules.
//! - Define lexical tokens.
//! - Define another expression hierarchy.
//! - Construct AST nodes.
//! - Perform semantic validation.
//! - Perform type inference.
//! - Generate IR.
//!
//! Grammar and semantic ownership remain separate from the
//! executable scientific library.
//!
//! # 10. Compatibility
//!
//! Existing child-module public paths are preserved:
//!
//!     stdlib::scientific::linear_algebra
//!     stdlib::scientific::statistics
//!     stdlib::scientific::optimization
//!     stdlib::scientific::probability
//!     stdlib::scientific::simulation
//!
//! Child modules are registered without changing their internal
//! implementations or introducing duplicate algorithms.
//!
//! Existing legacy APIs under stdlib::numeric and
//! stdlib::math_foundations remain separate compatibility concerns.
//!
//! Their migration must be performed through explicit adapters
//! and compatibility reviews, not by introducing duplicate
//! scientific implementations here.
//!
//! # 11. Lifecycle
//!
//! This module performs no global initialization or shutdown.
//!
//! Scientific algorithms and data structures should be usable
//! without initializing unrelated AI, quantum, robotics, GUI,
//! networking or distributed subsystems.
//!
//! Any provider or execution lifecycle belongs to the appropriate
//! runtime and provider integration.
//!
//! # 12. Testing
//!
//! Individual child modules own their algorithm and contract tests.
//!
//! Their tests must cover:
//!
//! - Numerical correctness.
//! - Dimension validation.
//! - Invalid input.
//! - Non-finite numeric values where applicable.
//! - Arithmetic overflow.
//! - Singular systems.
//! - Insufficient observations.
//! - Invalid probability parameters.
//! - Optimization feasibility.
//! - Cancellation.
//! - Configured budget exhaustion.
//! - Simulation outcome validation.
//! - Unsupported provider operations.
//! - Resource exhaustion.
//! - Reproducibility where promised.
//!
//! The repository integration suite must additionally verify:
//!
//! - All child modules compile through this composition root.
//! - Existing public API paths remain available.
//! - No duplicate numeric or tensor authorities are introduced.
//! - No unsafe Rust is introduced.
//! - Rust 1.97 compatibility is preserved.
//!
//! # 13. Freeze criteria
//!
//! This composition root is ready to freeze when:
//!
//! - All declared child modules exist.
//! - All declared child modules compile.
//! - Their public contracts are documented.
//! - Their canonical dependencies resolve.
//! - Their individual tests pass.
//! - The standard-library root registers this module.
//! - No competing mathematical or storage authorities exist.
//! - Repository formatting, compilation, testing and lint pass.
//!
//! Registering this module alone does not establish that every
//! scientific algorithm or execution provider is production-ready.

// =============================================================================
// Scientific computing modules
// =============================================================================

/// Canonical linear-algebra APIs.
///
/// Reuses the existing classical matrix and numerical authorities.
pub mod linear_algebra;

/// Canonical statistical APIs.
///
/// Reuses the existing classical statistical implementation.
pub mod statistics;

/// Mathematical optimization problems and solver integration.
///
/// Defines mathematical optimization contracts without taking
/// ownership of compiler optimization or execution scheduling.
pub mod optimization;

/// Probability mathematics, probability distributions and sampling.
///
/// Provides extensible probability APIs for classical and
/// cross-domain scientific computation.
pub mod probability;

/// Scientific simulation contracts and engine integration.
///
/// Supports domain-specific simulation engines without owning
/// their execution schedulers or hardware implementations.
pub mod simulation;
