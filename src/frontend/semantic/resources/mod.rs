#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend Semantic Resources
//!
//! File: `src/frontend/semantic/resources/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Proposed canonical semantic-resource module boundary.
//!
//! ## 1. Purpose
//!
//! This module is the public composition boundary for target-independent
//! semantic resource analysis in the Zamani frontend.
//!
//! It integrates:
//!
//! - `model`: canonical semantic resource types and records;
//! - `symbolic`: symbolic quantities, dimensions, and resource expressions;
//! - `requirements`: extraction of resource requirements from the canonical
//!   frontend AST;
//! - `constraints`: interpretation and evaluation of semantic resource
//!   constraints.
//!
//! This module owns module organization and the resource subsystem's public
//! boundary. It does not implement resource analysis algorithms.
//!
//! ## 2. Architectural position
//!
//! ```text
//! Source
//!   |
//!   v
//! Canonical lexer and parser
//!   |
//!   v
//! frontend::ast::node
//!   |
//!   v
//! frontend::semantic
//!   |
//!   +-- symbols
//!   +-- types
//!   +-- effects
//!   +-- capabilities
//!   +-- resources  <-- this module
//!   +-- contracts
//!   +-- domains
//!   |
//!   v
//! Canonical semantic model
//!   |
//!   +-- Classical IR
//!   +-- quantum::ir
//!   +-- HDL / hardware IR
//!   +-- future domain IRs
//!   |
//!   v
//! Optimization, lowering, target planning, and execution
//! ```
//!
//! This is the intended integration architecture. Each downstream interface
//! must be verified against the actual implementation before being enabled.
//!
//! ## 3. Authority and specifications
//!
//! This subsystem MUST conform to the repository's applicable normative
//! specifications, including:
//!
//! - `grammar/specification/grammar-authority.md`;
//! - `grammar/specification/semantic-model.md`;
//! - `grammar/specification/scalability-model.md`;
//! - `grammar/specification/poco-reaf.md`;
//! - the canonical grammar rooted at `grammar/Zamani.g4`;
//! - the resource grammar composition under `grammar/resources/`;
//! - the native AST resource boundary under
//!   `src/frontend/ast/node/resources/`.
//!
//! The normative specifications define intended behavior. The canonical
//! grammar defines accepted syntax. The native AST defines source structure.
//! This subsystem interprets that structure according to the semantic
//! contract; it MUST NOT independently redefine the language.
//!
//! Conflicts between specifications, grammar, AST, and implementation must
//! be reported and resolved through the repository's change-control process.
//! They must not be hidden by silently choosing whichever interpretation is
//! easiest to implement.
//!
//! ## 4. Module ownership
//!
//! Each resource concept has exactly one implementation owner.
//!
//! ### `model.rs`
//!
//! Owns canonical semantic resource identities, resource kinds, quantities,
//! units, requirements, and other shared resource records explicitly approved
//! by the semantic model.
//!
//! It MUST NOT own symbolic-expression algorithms, AST traversal, constraint
//! solving, target discovery, or physical allocation.
//!
//! ### `symbolic.rs`
//!
//! Owns the representation and operations for symbolic resource quantities,
//! dimensions, and relationships that cannot yet be evaluated to concrete
//! values.
//!
//! It MUST preserve unresolved symbolic relationships and source provenance
//! where the chosen API requires them. It MUST NOT assume that resource
//! quantities fit in `usize`, `u64`, or another host-sized integer.
//!
//! It MUST NOT decide whether a physical target can satisfy a requirement.
//!
//! ### `requirements.rs`
//!
//! Owns extraction and construction of semantic resource requirements from
//! canonical AST resource declarations, annotations, and expressions.
//!
//! It MUST consume the existing AST rather than introduce a competing AST.
//! It MUST preserve the distinction between a malformed requirement, a valid
//! concrete requirement, and a valid but unresolved symbolic requirement.
//!
//! It MUST NOT allocate resources or select a backend.
//!
//! ### `constraints.rs`
//!
//! Owns semantic resource-constraint interpretation and the evaluation
//! boundary for constraints whose required inputs are available.
//!
//! It MUST distinguish a satisfied constraint, a violated constraint, an
//! unresolved constraint, and an evaluation failure. The exact public result
//! types belong to this module's implementation contract.
//!
//! It MUST NOT report an unresolved constraint as satisfied. It MUST NOT
//! claim that a physical target is feasible without sufficient target
//! evidence and an explicitly authorized target-feasibility operation.
//!
//! ## 5. Public module surface
//!
//! The child modules below are the canonical owners of their respective
//! public types. Consumers should use these module-qualified APIs until the
//! shared public types and re-exports have been implemented and frozen.
//!
//! This avoids making this composition root depend on guessed type names or
//! on partially implemented sibling interfaces.
//!
//! Adding a new resource kind, computational domain, resource unit, or
//! backend MUST NOT require adding a new module declaration here unless the
//! change introduces a genuinely new subsystem responsibility.
//!
//! ## 6. Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! Canonical AST and source identity
//!              |
//!              v
//!       resources::model
//!          ^       ^
//!          |       |
//! resources::symbolic
//!          ^
//!          |
//! resources::requirements
//!          |
//!          v
//! resources::constraints
//!          |
//!          v
//! Canonical semantic result
//! ```
//!
//! This diagram expresses ownership boundaries, not a requirement that every
//! module import every module above it. Actual imports must be minimal and
//! acyclic.
//!
//! Required dependency rules:
//!
//! 1. The semantic resource subsystem may depend on canonical AST types and
//!    approved semantic foundations.
//! 2. `model.rs` must not depend on `requirements.rs` or `constraints.rs`.
//! 3. `symbolic.rs` must not depend on target discovery, a backend, a runtime,
//!    or physical resource allocation.
//! 4. `requirements.rs` may use the canonical AST, resource model, and
//!    symbolic-expression API.
//! 5. `constraints.rs` may use the resource model and symbolic-expression
//!    API, together with explicitly defined semantic evidence.
//! 6. No child module may import this composition root to access a sibling.
//!    Siblings should import the owning module directly.
//! 7. No module may import the legacy `crate::semantic` analyzer merely to
//!    reuse its unrelated AST or semantic types.
//! 8. No resource module may depend on `quantum::ir`, classical IR, HDL IR,
//!    scheduling, routing, QEC, ZQN, HAL, or runtime allocation.
//! 9. No reverse dependency from the AST into semantic analysis is permitted.
//!
//! ## 7. Scalability and POCO-REAF
//!
//! This module introduces no language-level ceiling for:
//!
//! - resource quantities;
//! - logical quantum resources;
//! - processors, accelerators, or devices;
//! - memory or storage requirements;
//! - tensor dimensions or dimensional relationships;
//! - distributed resources or topology size;
//! - resource kinds, capability identifiers, or computational domains.
//!
//! The intended architecture supports:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! This does not mean that physical resources or compiler execution are
//! infinite. It means that the language must not acquire arbitrary hardware
//! capacity limits merely because a particular implementation uses a finite
//! representation or runs on a small host.
//!
//! Resource quantities and dimensions should remain symbolic until their
//! semantics authorize evaluation. Concrete arithmetic must detect overflow
//! and invalid operations explicitly. If a quantity cannot be represented or
//! evaluated under a selected policy, the implementation must report that
//! fact rather than wrap, truncate, or silently change the requirement.
//!
//! Operational budgets, where needed for safety, belong in explicit,
//! configurable compiler configuration. They are not resource capacities
//! promised by the language and must not turn budget exhaustion into a
//! successful semantic result.
//!
//! ## 8. Requirements, constraints, preferences, and hints
//!
//! The integrated resource API MUST preserve these distinct meanings:
//!
//! - **Requirement:** a condition that must be satisfied for a realization to
//!   preserve the program's declared semantics.
//! - **Constraint:** a restriction on admissible realizations.
//! - **Preference:** a non-mandatory criterion used when comparing otherwise
//!   acceptable realizations.
//! - **Hint:** optional implementation guidance that cannot change the
//!   program's observable meaning.
//! - **Capability:** an ability that an execution environment may provide;
//!   it is not itself a resource allocation.
//! - **Target feasibility:** a downstream determination based on actual
//!   target information and applicable policies.
//!
//! A preference or hint MUST NOT be promoted silently to a mandatory
//! requirement. An unmet target requirement MUST NOT be converted into a
//! different program meaning to make compilation appear successful.
//!
//! ## 9. Error handling and incomplete analysis
//!
//! All child modules must define explicit outcomes for invalid input,
//! unresolved symbols, unknown units, invalid arithmetic, unsupported
//! expressions, contradictory constraints, and exhausted operational budgets
//! where applicable.
//!
//! They MUST NOT:
//!
//! - panic on ordinary malformed source or AST input;
//! - silently treat unknown quantities as zero;
//! - silently accept incompatible units;
//! - silently overflow or truncate a quantity;
//! - treat an unresolved constraint as proven;
//! - conflate missing target evidence with a proven target failure;
//! - discard relevant source provenance;
//! - turn an incomplete analysis into a certified-valid result.
//!
//! Diagnostics should use the semantic subsystem's canonical diagnostic
//! infrastructure rather than introducing a second diagnostic model.
//!
//! ## 10. Determinism
//!
//! For equivalent canonical ASTs, configuration, semantic environment, and
//! extension definitions, the resource subsystem must produce equivalent
//! semantic outcomes.
//!
//! Where collections or diagnostics are observable, their ordering must be
//! explicitly defined and deterministic. Hash-map iteration order must not
//! accidentally determine diagnostic order, serialized output, or semantic
//! results.
//!
//! Symbolic normalization must preserve meaning. Expressions may only be
//! reordered or simplified where the relevant operations are proven to
//! permit that transformation.
//!
//! ## 11. Integration contract
//!
//! ### AST integration
//!
//! `requirements.rs` consumes the canonical source-level resource
//! representation under `crate::frontend::ast::node::resources` and the
//! canonical expression and node-identity APIs.
//!
//! It must not assume that every resource quantity is a literal integer.
//!
//! ### Type-system integration
//!
//! Symbolic resource expressions may participate in type-level dimensions and
//! other semantic constraints through a deliberately defined interface.
//! The resource subsystem must not create a second constant evaluator or a
//! competing type-constraint solver.
//!
//! Constant evaluation remains owned by the semantic constant-evaluation
//! contract. The resource subsystem may request evaluation through that
//! contract and must preserve an unresolved result when evaluation is not
//! currently possible.
//!
//! ### Capability integration
//!
//! Resource requirements may refer to capabilities, but capability identity,
//! evidence, and compatibility remain owned by the capabilities subsystem.
//! Resource modules must not duplicate the capability registry.
//!
//! ### Diagnostic integration
//!
//! Resource diagnostics must use the shared semantic diagnostic codes,
//! source labels, and ordering rules once those interfaces are available.
//!
//! ### Semantic-result integration
//!
//! The completed semantic model must be able to record resource requirements,
//! symbolic relationships, evaluation states, constraints, provenance, and
//! unresolved obligations without embedding target-specific allocation data.
//!
//! ### Downstream integration
//!
//! Classical IR, `quantum::ir`, HDL/hardware IR, and other downstream
//! consumers may receive validated semantic resource facts through explicit
//! adapters. They must not become dependencies of this module.
//!
//! ## 12. Public API and compatibility policy
//!
//! This file is a composition root, not a convenience layer for arbitrary
//! implementation details.
//!
//! A public type should be re-exported here only after:
//!
//! - its owning module has been implemented;
//! - its behavior and invariants are documented;
//! - its integration contract has been agreed upon;
//! - its public API tests pass;
//! - its compatibility implications have been reviewed.
//!
//! Public re-exports must point to the owning module; this file must never
//! redefine those types.
//!
//! Breaking public API changes require explicit review and coordinated
//! updates to consumers. New resource kinds and extension identifiers should
//! not require breaking changes to this module's API.
//!
//! ## 13. Freeze criteria
//!
//! This file may be marked frozen only when:
//!
//! - all declared child modules exist;
//! - their public APIs and ownership contracts are documented;
//! - module dependencies are acyclic;
//! - no child module imports this composition root to reach a sibling;
//! - canonical AST integration compiles;
//! - resource, capability, type-constraint, and diagnostic integration
//!   contracts are agreed upon;
//! - module-surface tests pass;
//! - the semantic resource conformance suite passes;
//! - determinism, malformed-input, and configured-budget tests pass;
//! - no resource-capacity ceiling has been introduced into language semantics;
//! - the complete workspace builds on the supported Rust toolchain.
//!
//! Until these criteria are met, the subsystem must be treated as under
//! implementation rather than production-verified.

// ---------------------------------------------------------------------------
// Canonical semantic-resource modules
// ---------------------------------------------------------------------------
//
// These modules are intentionally declared here exactly once.
//
// IMPORTANT INTEGRATION REQUIREMENT:
// The following sibling files must exist before this module can compile:
//
//   model.rs
//   symbolic.rs
//   requirements.rs
//   constraints.rs
//
// Do not add placeholder implementations in this file to conceal missing
// siblings. Implement each responsibility in its own file and test it against
// the contracts above.

pub mod model;
pub mod symbolic;
pub mod requirements;
pub mod constraints;