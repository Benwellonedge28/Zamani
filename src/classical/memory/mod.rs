
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Memory Subsystem
//!
//! Path: `src/classical/memory/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the composition root for classical memory semantics and
//! provider-neutral memory services in Zamani.
//!
//! It exposes the existing modules responsible for:
//!
//! - logical address spaces and target-specific space bindings;
//! - provider-neutral allocation requests, budgets, and allocation handles;
//! - borrow declarations and their deterministic index;
//! - named lifetime constraints and lifetime-to-region bindings;
//! - adapters to canonical ZUIR ownership contracts;
//! - semantic region scopes and enclosing-scope relationships.
//!
//! This module declares subsystem boundaries. It does not implement memory
//! management, perform semantic analysis, or execute memory operations.
//!
//! # Authority
//!
//! The responsibilities of the subsystem are divided among authoritative
//! repository components:
//!
//! - `grammar/memory/memory.g4` owns common memory source syntax.
//! - `grammar/memory/allocation.g4` owns allocation-intent syntax.
//! - `grammar/memory/deallocation.g4` owns deallocation-intent syntax.
//! - `grammar/memory/borrowing.g4` owns borrowing-related source syntax.
//! - `grammar/memory/lifetimes.g4` owns lifetime source syntax.
//! - `src/zuir/ids.rs` owns canonical ZUIR identifiers, including `RegionId`.
//! - `src/zuir/ir/ownership.rs` owns canonical ownership contracts and uses.
//! - `src/zuir/ir/region.rs` owns canonical structural ZUIR regions.
//! - The normative semantic specification owns language-level semantics.
//! - Semantic analysis and verification own whole-program ownership,
//!   borrowing, lifetime, aliasing, and control-flow correctness.
//! - Allocation providers and runtime backends own physical realization.
//!
//! These responsibilities must not be duplicated in this module.
//!
//! # Module ownership
//!
//! ## `address_space`
//!
//! Owns logical address-space descriptors, open-world space identifiers,
//! target bindings, and capability requirements associated with those spaces.
//! It does not own physical addresses, allocation, or memory contents.
//!
//! ## `allocation`
//!
//! Owns the provider-neutral classical allocation boundary, allocation
//! requests, explicit resource budgets, allocation accounting, backend
//! contracts, and allocation handles. It does not own lifetime inference,
//! borrow checking, or physical-memory policies for every possible target.
//!
//! ## `borrowing`
//!
//! Owns classical-facing borrow records and their local consistency checks,
//! using canonical ownership contracts and identifiers. Recording a borrow
//! is not proof that all uses and control-flow paths are valid.
//!
//! ## `lifetime`
//!
//! Owns named and anonymous lifetime references, explicit outlives
//! constraints, optional bindings to canonical `RegionId` values, and local
//! lifetime-environment queries. It does not implement the complete borrow
//! checker or lifetime inference for an entire program.
//!
//! ## `ownership`
//!
//! Adapts and re-exports canonical ZUIR ownership types and supplies
//! classical-facing constructors. It must not define a competing ownership
//! model or identifier type.
//!
//! ## `regions`
//!
//! Owns the semantic scope relationship index used by lifetime and borrow
//! analysis. It must not duplicate the canonical structural IR region model.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! Source grammar
//!      |
//!      v
//! Frontend / domain-neutral AST
//!      |
//!      v
//! Semantic analysis and verification
//!      |
//!      +------------------------------+
//!      |                              |
//!      v                              v
//! Canonical ZUIR identities     Canonical ownership / IR regions
//!      |                              |
//!      +---------------+--------------+
//!                      |
//!                      v
//!              classical::memory
//!       +--------------+----------------+
//!       |              |                |
//!       v              v                v
//!   ownership       regions          borrowing
//!       |              |                |
//!       +--------------+----------------+
//!                      |
//!                      v
//!                   lifetime
//!
//! Allocation and logical address-space descriptions provide separate,
//! provider-neutral interfaces for downstream execution and realization.
//! ```
//!
//! This diagram describes responsibilities, not permission to introduce
//! cyclic imports. Individual modules must depend only on the contracts they
//! actually need.
//!
//! # Public API policy
//!
//! The child modules are public so callers can use their documented,
//! authoritative interfaces:
//!
//! - `memory::address_space`
//! - `memory::allocation`
//! - `memory::borrowing`
//! - `memory::lifetime`
//! - `memory::ownership`
//! - `memory::regions`
//!
//! This module intentionally does not glob-re-export their contents. Callers
//! should use qualified paths so type ownership, error sources, and API
//! evolution remain explicit. A convenience re-export may be added only
//! after its name, compatibility, and ambiguity implications are reviewed.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no universal maximum for:
//!
//! - allocation size or allocation count;
//! - memory capacity or address-space count;
//! - the number of regions, borrows, or lifetime constraints;
//! - the number of devices, workers, or execution targets;
//! - nesting depth, collection size, or program size.
//!
//! No finite implementation can promise literal infinity. Actual operation
//! remains subject to available resources, representable identifiers,
//! explicit policies and budgets, and the capabilities of the selected
//! provider or target.
//!
//! Resource exhaustion and unsupported target capabilities must be reported
//! through the relevant module's structured errors. They must not silently
//! truncate data, change language semantics, or masquerade as universal
//! language restrictions.
//!
//! # Cross-domain integration
//!
//! Classical memory services must remain independent of any particular CPU,
//! GPU, FPGA, QPU, accelerator, operating system, distributed-memory system,
//! or vendor API.
//!
//! Quantum and HDL subsystems may interoperate through their canonical
//! semantic and IR boundaries. This module must not import quantum-specific
//! state representations, create a second classical IR, or assume that all
//! memory resources have interchangeable ownership or lifetime semantics.
//!
//! # Runtime and compiler boundary
//!
//! The frontend and semantic analyzer resolve source constructs and verify
//! their correctness. Canonical IR and its verifier remain authoritative for
//! IR structure and invariants. Lowering, execution planning, and backends
//! preserve validated semantics while realizing them on compatible targets.
//!
//! This module must not:
//!
//! - parse source code or define AST nodes;
//! - create a second ownership, lifetime, type, or IR system;
//! - perform whole-program borrow checking;
//! - select hardware or schedule work;
//! - expose raw physical addresses as portable semantic identities;
//! - assume allocation success merely because a request is valid;
//! - promise that a target supports a capability it has not advertised.
//!
//! # Error and safety policy
//!
//! This subsystem uses safe Rust. No `unsafe` implementation is permitted.
//! Each child module documents its own error types and failure behavior.
//! Errors should preserve enough context for callers to distinguish invalid
//! semantic requests, inconsistent analysis state, exhausted budgets,
//! unavailable capabilities, and provider failures.
//!
//! User-controlled input, identifier values, dimensions, sizes, and resource
//! requests must be validated at their owning boundaries. Arithmetic used
//! for sizes and offsets must be checked where overflow could affect safety
//! or correctness. Fallible allocation must be handled explicitly wherever
//! the underlying API supports it.
//!
//! # Integration contract
//!
//! The parent `src/classical/mod.rs` must declare `pub mod memory;` once it
//! has been repaired as valid Rust source.
//!
//! The compiler and semantic analyzer should construct and consume the
//! relevant child-module interfaces after resolving source constructs to
//! canonical semantic identities. The canonical verifier remains responsible
//! for whole-program correctness; lowering and runtime code must not treat a
//! local memory index as proof of semantic validity.
//!
//! The module list below intentionally contains only files present in the
//! inspected repository. New modules must be added here only after their
//! contracts, implementations, and tests have been established.
//!
//! # Completion criteria
//!
//! This composition root is integrated when:
//!
//! - every declared child module exists and compiles;
//! - the parent classical module declares this module exactly once;
//! - all public paths resolve without duplicate definitions or cyclic
//!   dependency workarounds;
//! - canonical ZUIR identity and ownership types remain authoritative;
//! - the repository's supported formatting, checking, test, and lint
//!   commands pass, or any pre-existing failures are explicitly documented.
//!
//! This file's successful compilation alone does not establish that the
//! entire classical memory subsystem or repository is production-ready.

// Logical memory spaces and target/backend bindings.
pub mod address_space;

// Provider-neutral allocation contracts and allocation handles.
pub mod allocation;

// Local borrow declarations and deterministic borrow indexing.
pub mod borrowing;

// Named lifetime constraints and canonical region bindings.
pub mod lifetime;

// Classical-facing adapters to canonical ZUIR ownership contracts.
pub mod ownership;

// Semantic region scopes and outlives relationships.
pub mod regions;
