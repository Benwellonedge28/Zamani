
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution
//!
//! Path: `src/classical/execution/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Compose the classical execution-planning subsystem through stable,
//! explicitly owned modules. This module is the public module boundary;
//! individual modules retain responsibility for their own data models,
//! validation rules, and algorithms.
//!
//! ## Architectural authority
//!
//! The execution subsystem follows these repository contracts:
//!
//! - `grammar/Zamani.g4`: canonical root grammar composition.
//! - `grammar/specification/semantic-model.md`: normative language meaning.
//! - `grammar/specification/execution-model.md`: execution boundaries.
//! - `grammar/specification/scalability-model.md`: resource-parametric scaling.
//! - `grammar/spec/resources.md`: resource and capability semantics.
//! - `src/frontend/semantic/`: canonical semantic requirements and facts.
//! - `src/zuir/`: universal computation IR.
//! - `src/quantum/ir/`: canonical quantum IR.
//! - `src/classical/config.rs`: classical execution policies and budgets.
//! - `src/classical/error.rs`: classical error vocabulary.
//!
//! Grammar defines syntax. Semantic specifications define meaning.
//! Canonical semantic models define the meaning of requirements and
//! capabilities. Canonical IR represents established computation semantics.
//! This module composes execution-facing adapters without replacing those
//! authorities.
//!
//! ## Module ownership
//!
//! - [`plan`]: target-independent execution plans, step identities,
//!   dependency validation, and deterministic dependency ordering.
//! - [`requirements`]: aggregation and completeness tracking for canonical
//!   resource and capability requirements.
//! - [`capabilities`]: assessment of canonical capability requirements
//!   against externally supplied, scope-specific evidence.
//! - [`placement`]: abstract placement intent and structural validation.
//! - [`partitioning`]: logical partition assignments and partition
//!   dependency validation.
//! - [`scheduling`]: dependency-respecting schedules based on explicit
//!   duration, usage, and capacity inputs.
//! - [`fallback`]: ordered fallback orchestration and retained attempt
//!   outcomes. This module does not establish semantic equivalence between
//!   arbitrary alternatives.
//!
//! These modules have distinct responsibilities. This composition module
//! does not define a second execution plan, requirement model, capability
//! registry, placement model, partition representation, schedule, or
//! fallback policy.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Grammar -> AST -> semantic analysis
//!                         |
//!                         v
//!                canonical semantic model
//!                         |
//!                         v
//!                  canonical ZUIR
//!                         |
//!                         v
//!              classical execution adapters
//!                         |
//!          +--------------+---------------+
//!          |              |               |
//!          v              v               v
//!      requirements   capabilities     plan/placement
//!          |              |               |
//!          +--------------+---------------+
//!                         |
//!                         v
//!                 feasibility checks
//!                         |
//!                         v
//!             partitioning and scheduling
//!                         |
//!                         v
//!             target-specific realization
//!                         |
//!                         v
//!                  runtime execution
//! ```
//!
//! This diagram describes the intended integration flow, not a claim that
//! every downstream service or connection is already implemented.
//!
//! Fallback may be used by an execution coordinator when an attempt fails.
//! It must not bypass feasibility checks, mandatory requirements, security
//! policies, or semantic-preservation requirements.
//!
//! ## Public API policy
//!
//! Modules are exposed explicitly with `pub mod` so consumers can use
//! qualified paths such as `execution::plan` and `execution::scheduling`.
//!
//! This module intentionally avoids wildcard re-exports. Public types remain
//! under their owning module until a separately reviewed API facade defines
//! stable names, compatibility guarantees, and any required conversions.
//!
//! Adding a new execution component requires an explicit module declaration
//! here, a review of ownership and dependencies, and the corresponding
//! compilation and integration tests.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no fixed maximum on the number of plans, steps,
//! requirements, capabilities, partitions, schedule entries, or fallback
//! candidates.
//!
//! Actual limits arise from available memory, representable values,
//! operational budgets, target capabilities, policies, and the work required
//! by the algorithms involved. Such limits must be reported explicitly;
//! they must not silently change the meaning of a Zamani program.
//!
//! POCO-REAF requires semantic portability, not a guarantee that every target
//! can execute every program. A target that cannot satisfy a mandatory
//! requirement must be rejected or reported infeasible unless an explicitly
//! permitted, semantics-preserving realization exists.
//!
//! ## Error and diagnostic policy
//!
//! Each component owns its structured validation and operational errors.
//! Integrating callers must preserve error causes and relevant provenance
//! when converting them into `crate::classical::error` or the repository's
//! canonical diagnostic infrastructure.
//!
//! Human-readable error messages must not be used as machine-readable error
//! codes. Missing or conflicting capability evidence must not be interpreted
//! as proof of compatibility.
//!
//! ## Runtime boundary
//!
//! This module does not itself discover hardware, allocate physical
//! resources, spawn threads, dispatch operations, retry irreversible side
//! effects, or execute a plan.
//!
//! Runtime and target adapters are responsible for those operations.
//! Fallback and scheduling must be coordinated with cancellation, execution
//! state, idempotency, and resource accounting.
//!
//! ## Safety
//!
//! Unsafe Rust is prohibited. Foreign-function boundaries and operating-system
//! operations must be implemented behind separately reviewed safe interfaces.
//!
//! ## Completion status
//!
//! This file establishes module composition only. It does not establish that
//! the entire execution subsystem has compiled, that all components are
//! integrated, or that any target is production-qualified.
//!
//! Integration is complete only when every declared module exists, all
//! internal references resolve, public API checks pass, and the repository's
//! supported build and test suites pass.

/// Target-independent execution plans and dependency validation.
///
/// This module owns logical plan representation. It does not own canonical
/// ZUIR operations, target selection, or physical execution.
pub mod plan;

/// Aggregation and completeness tracking for execution requirements.
///
/// Canonical resource and capability semantics remain owned by the frontend
/// semantic subsystem. This module adapts them for execution planning.
pub mod requirements;

/// Assessment of capability requirements against supplied evidence.
///
/// This module does not discover hardware, authenticate evidence, allocate
/// resources, or decide that an unverified capability is available.
pub mod capabilities;

/// Target-independent placement requests.
///
/// This module represents placement intent and does not assign physical
/// devices, nodes, addresses, or workers.
pub mod placement;

/// Logical partition assignments and dependency relationships.
///
/// This module does not perform physical distribution, data movement,
/// placement, or runtime dispatch.
pub mod partitioning;

/// Deterministic, dependency-respecting execution scheduling.
///
/// This module computes schedules from explicit inputs and does not itself
/// execute operations or guarantee runtime performance.
pub mod scheduling;

/// Ordered fallback orchestration.
///
/// Fallback candidates must preserve the required program semantics and
/// satisfy mandatory requirements. This module does not prove arbitrary
/// implementation equivalence or independently establish target feasibility.
pub mod fallback;
