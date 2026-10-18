
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Pass Subsystem
//!
//! Path: `src/classical/zcir/passes/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//!
//! ## Purpose
//!
//! Compose the ZCIR pass subsystem and provide its stable module boundary.
//! ZCIR pass implementations adapt classical-computing transformations to
//! Zamani's canonical IR and pass infrastructure.
//!
//! ## Architectural authority
//!
//! This module must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/contracts.rs`
//!
//! The grammar owns source syntax. The semantic model owns program meaning.
//! The canonical IR owns its representation and invariants. This module
//! composes pass implementations; it does not replace those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of ZCIR pass-related modules.
//! - The namespace boundary for classical pass adapters.
//! - Documentation of dependencies and integration expectations.
//!
//! This module does not own:
//!
//! - Pass identity, descriptors, versions, outcomes, or canonical errors.
//! - The universal pass manager or registry implementation.
//! - IR operations, values, blocks, regions, or verification algorithms.
//! - Source parsing, type checking, or source-language semantic rules.
//! - Target selection, scheduling, hardware allocation, or runtime execution.
//! - Canonical serialization or persistent artifact formats.
//!
//! ## Dependency direction
//!
//! ```text
//! Zamani frontend and semantic model
//!                 |
//!                 v
//!       Canonical IR / ZUIR
//!                 |
//!                 v
//!       Canonical pass contracts
//!                 |
//!                 v
//!          ZCIR pass adapters
//!           /      |       \
//!          v       v        v
//!     Manager   Legality  Canonicalize
//!          \       |       /
//!           v      v      v
//!        Canonical IR verification
//!                 |
//!                 v
//!       Subsequent compiler stages
//! ```
//!
//! The diagram describes intended integration, not proof that every stage
//! is already implemented. The manager, registry, and adapters must share
//! canonical ZUIR pass contracts. They must not introduce parallel pass
//! identity or descriptor types.
//!
//! ## Pass contracts
//!
//! `crate::zuir::pass::contracts` is authoritative for pass IDs, descriptors,
//! versions, outcomes, and structured pass errors. This module must not
//! shadow or redefine those contracts.
//!
//! The current ZUIR pass subsystem contains a migration inconsistency:
//! `src/zuir/pass/manager.rs` defines legacy copies of several contract types,
//! and `src/zuir/pass/registry.rs` imports those legacy types. ZCIR must not
//! silently treat those duplicate types as interchangeable with the
//! canonical definitions. Complete that migration in the ZUIR subsystem.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no fixed limits on pass count, operation count,
//! IR size, processor count, memory capacity, or computational domains.
//!
//! Individual passes may use explicit operational budgets for work, time,
//! cancellation, or memory. Exhausting a budget must be reported as an
//! operational failure, not as invalid source semantics.
//!
//! Deterministic passes must not rely on unspecified map iteration order,
//! memory addresses, hardware inventory, or nondeterministic scheduling.
//! Transformations must preserve the program's specified observable behavior.
//!
//! ## Integration contract
//!
//! Before this module is integrated into the repository:
//!
//! 1. Create `canonicalize.rs`, `legality.rs`, and `manager.rs` with the
//!    public contracts documented in their respective file headers.
//! 2. Ensure every child module compiles against the actual canonical ZUIR
//!    contracts and existing repository APIs.
//! 3. Ensure `canonicalize.rs` uses a concrete adapter over the authoritative
//!    IR representation before claiming to implement real rewrites.
//! 4. Ensure `legality.rs` distinguishes declarations and evidence from
//!    actual IR verification or semantic-preservation proofs.
//! 5. Ensure `manager.rs` does not define competing canonical pass types or
//!    rely on incompatible legacy contracts.
//! 6. Register this module from `src/classical/zcir/mod.rs` only when this
//!    file and all declared children exist.
//! 7. Register ZCIR from `src/classical/mod.rs` only when its parent module
//!    has a valid Rust implementation.
//!
//! This module intentionally does not declare speculative child modules for
//! builders, analyses, lowering, serialization, or other planned subsystems.
//! Add each such module when its file and contract have been reviewed.
//!
//! ## Compatibility
//!
//! Module paths are public integration surfaces once downstream code depends
//! on them. Renaming or removing a published path requires a compatibility
//! decision. This module does not itself define a persistent pass-pipeline
//! format or promise that independently compiled artifacts are interchangeable.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden throughout this module.
//!
//! ## Freeze criteria
//!
//! - All declared child files exist.
//! - Every child uses the canonical pass contract types.
//! - No duplicate manager, registry, descriptor, or error authority remains
//!   in the ZCIR boundary.
//! - The canonicalizer has a concrete IR adapter and semantic-preservation
//!   tests, not only an adapter trait.
//! - Legality checks are integrated with actual verification.
//! - Public APIs and error propagation are documented.
//! - Determinism, budget exhaustion, and large-input behavior are tested.
//! - Formatting, compilation, tests, and Clippy pass at the declared baseline.

// -----------------------------------------------------------------------------
// Pass modules
// -----------------------------------------------------------------------------

/// Canonicalization policy and orchestration.
///
/// Concrete rewrite rules must be implemented through an adapter over the
/// repository's authoritative IR types. This module does not own IR entities.
pub mod canonicalize;

/// Pass-legality contracts and assessment.
///
/// Legality assessment must not substitute for canonical IR verification or
/// a proof that a transformation preserves program semantics.
pub mod legality;

/// ZCIR pass-manager integration.
///
/// The implementation must reuse canonical ZUIR contracts and must not
/// create an independent pass identity, descriptor, outcome, or error model.
pub mod manager;
