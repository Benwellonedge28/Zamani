//! Zamani Quantum Resilience — Error Mitigation Subsystem
//!
//! Path:
//!     src/quantum/resilience/mitigation/mod.rs
//!
//! Purpose:
//!     Composition boundary for provider-independent quantum error mitigation.
//!
//! # Architectural role
//!
//! The mitigation subsystem provides mechanisms for reducing the effect of
//! physical noise on quantum-computation results without changing the
//! canonical meaning of the Zamani program.
//!
//! The subsystem is deliberately separated into:
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                    resilience / policy
//!                             │
//!                             ▼
//!                       mitigation
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          │                  │                  │
//!          ▼                  ▼                  ▼
//!      selection          strategies         executor
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             │
//!                             ▼
//!                       quantum runtime
//!                             │
//!                             ▼
//!                         hardware
//!                             │
//!                             ▼
//!                        verification
//! ```
//!
//! This file is intentionally a composition root.
//!
//! It declares and exposes the mitigation subsystem's child modules. It does
//! not implement mitigation algorithms, execution, hardware access, routing,
//! scheduling, QEC, noise modelling, or policy decisions.
//!
//! # Write once, scale everywhere
//!
//! No module in this composition boundary assumes a particular:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - circuit depth;
//! - operation count;
//! - gate arity;
//! - number of shots;
//! - number of circuit variants;
//! - number of devices;
//! - number of backends;
//! - topology size;
//! - quantum technology;
//! - hardware provider.
//!
//! "Infinite scale" means that this namespace imposes no artificial semantic
//! machine-size ceiling. Concrete executions remain bounded only by resources,
//! capabilities, policies, and execution-target constraints supplied by the
//! surrounding quantum architecture.
//!
//! No fixed values such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_SHOTS
//! MAX_VARIANTS
//! MAX_DEVICES
//! DEFAULT_BACKEND
//! DEFAULT_QUBIT_COUNT
//! ```
//!
//! belong in this module.
//!
//! # Canonical quantum identity
//!
//! Mitigation scopes that identify logical qubits use the canonical identity:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! The mitigation subsystem must never define another `QubitId`,
//! `PhysicalQubitId`, or equivalent replacement for the canonical IR types.
//!
//! Physical resource identity remains owned by the hardware/routing layers.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ├──────────────► mitigation::strategy
//!      │
//!      └──────────────► mitigation::selection
//!
//! quantum::hardware ──► mitigation evaluation/execution context
//!
//! quantum::zqn ───────► noise/fault information
//!
//! quantum::routing ───► physical mapping when required
//!
//! quantum::scheduling ► timing/schedule information when required
//!
//! quantum::optimization ► circuit transformation when required
//!
//! mitigation::strategy
//!          │
//!          ├────────► mitigation::selection
//!          │
//!          ├────────► concrete mitigation strategies
//!          │
//!          └────────► mitigation::executor
//!
//! mitigation::selection
//!          │
//!          ▼
//! mitigation::executor
//!          │
//!          ▼
//! quantum runtime / hardware
//!          │
//!          ▼
//! verification
//! ```
//!
//! The composition root does not reverse those dependencies.
//!
//! In particular:
//!
//! ```text
//! quantum::ir       ─X─► mitigation implementation
//! quantum::hardware ─X─► mitigation strategy semantics
//! mitigation        ─X─► vendor SDK directly
//! mitigation        ─X─► provider credentials
//! mitigation        ─X─► global mutable state
//! ```
//!
//! # Child-module ownership
//!
//! ## `strategy`
//!
//! Defines the stable provider-independent contract shared by all mitigation
//! strategies.
//!
//! It owns:
//!
//! - strategy identity;
//! - strategy version;
//! - strategy family;
//! - execution phase;
//! - capability requirements;
//! - expected overhead;
//! - applicability;
//! - mitigation scope;
//! - strategy evaluation;
//! - strategy collections.
//!
//! It does not execute quantum work.
//!
//! Existing strategy implementations consume this contract. The contract also
//! explicitly uses the canonical `quantum::ir::qubit::QubitId` for logical
//! qubit scopes.
//!
//! ## `selection`
//!
//! Determines which registered mitigation strategy is an eligible candidate
//! under the supplied:
//!
//! - mitigation permission;
//! - strategy context;
//! - policy constraints;
//! - strategy priorities;
//! - family priorities;
//! - selection objective;
//! - deterministic selection mode.
//!
//! Selection is not execution.
//!
//! An evaluated strategy that still requires capability or policy validation
//! must not be silently converted into an executable selection.
//!
//! ## `executor`
//!
//! Owns execution orchestration for an already-selected mitigation strategy.
//!
//! It is responsible for connecting the strategy contract to the execution
//! infrastructure.
//!
//! It does not redefine strategy-selection policy.
//!
//! It does not become a hardware-provider abstraction.
//!
//! It must use the authoritative runtime/hardware contracts supplied by the
//! surrounding quantum architecture.
//!
//! ## `readout`
//!
//! Implements measurement/readout error mitigation.
//!
//! Its responsibility is classical correction/reconstruction of measurement
//! results and any required calibration/characterization contracts.
//!
//! It must not redefine canonical quantum semantics.
//!
//! ## `zero_noise`
//!
//! Implements zero-noise extrapolation and related noise-scaling abstractions.
//!
//! Noise amplification factors, extrapolators, execution variants, and
//! statistical requirements remain configuration/policy driven.
//!
//! No fixed number of noise factors belongs in the module boundary.
//!
//! ## `probabilistic`
//!
//! Implements probabilistic error cancellation and related probabilistic
//! mitigation mechanisms.
//!
//! Sampling and overhead are target- and policy-dependent.
//!
//! No fixed sampling count, circuit count, or qubit count belongs here at the
//! composition level.
//!
//! ## `twirling`
//!
//! Implements randomized compiling / twirling mechanisms.
//!
//! Randomness must be explicit and reproducible where deterministic execution
//! is requested.
//!
//! Randomness provenance must remain available to verification and telemetry.
//!
//! ## `dynamical_decoupling`
//!
//! Implements dynamical-decoupling mechanisms.
//!
//! It operates through scheduling/timing/pulse capability contracts rather than
//! directly accessing hardware.
//!
//! Whether dynamical decoupling is useful is target- and workload-dependent.
//! It must therefore be evaluated rather than unconditionally inserted.
//!
//! Current quantum practice confirms this distinction: dynamical decoupling
//! can help during idle periods but may worsen results when pulse insertion is
//! itself detrimental. The strategy therefore belongs behind capability and
//! policy evaluation rather than being a mandatory transformation.
//!
//! ## `custom`
//!
//! Provides extension mechanisms for mitigation strategies not yet represented
//! by the built-in strategy families.
//!
//! Custom strategies must still implement the same stable strategy contract.
//!
//! The module boundary must remain open-ended so adding a new mitigation
//! strategy does not require modifying this file.
//!
//! # Integration contracts
//!
//! ## Resilience API
//!
//! The parent resilience API supplies the execution request and policy context.
//!
//! ```text
//! quantum::resilience::api
//!             │
//!             ▼
//!     mitigation::selection
//!             │
//!             ▼
//!     mitigation::executor
//! ```
//!
//! The mitigation subsystem does not own the overall resilience lifecycle.
//!
//! ## Policy
//!
//! `quantum::resilience::policy` determines whether mitigation is:
//!
//! - disabled;
//! - allowed;
//! - required;
//! - constrained by resource/semantic budgets.
//!
//! The mitigation module must not invent a competing policy system.
//!
//! ## Planning
//!
//! `quantum::resilience::planning` may treat mitigation as one candidate action
//! among other resilience actions such as:
//!
//! - retry;
//! - reroute;
//! - reschedule;
//! - recompile;
//! - migrate;
//! - change QEC;
//! - abort.
//!
//! Mitigation therefore remains one execution-resilience mechanism rather than
//! becoming the entire resilience engine.
//!
//! ## Verification
//!
//! `quantum::resilience::verification` remains authoritative for determining
//! whether the mitigated result satisfies the required semantic and confidence
//! guarantees.
//!
//! Mitigation must never declare its own result correct merely because a
//! mitigation algorithm completed.
//!
//! ## Telemetry
//!
//! `quantum::resilience::telemetry` should be able to observe:
//!
//! - strategy identity;
//! - strategy version;
//! - strategy family;
//! - configuration identity;
//! - target identity;
//! - execution variant identity;
//! - expected overhead;
//! - actual execution outcome;
//! - mitigation diagnostics;
//! - verification outcome.
//!
//! The mitigation module itself should not require a particular telemetry
//! backend.
//!
//! ## History
//!
//! `quantum::resilience::history` may consume mitigation outcomes for later
//! statistical analysis and strategy evaluation.
//!
//! Historical observations must not silently override explicit safety policy.
//!
//! ## Registry
//!
//! `quantum::resilience::registry::strategy` owns dynamic registration and
//! discovery of concrete strategy implementations.
//!
//! This module declares strategy implementations but does not own a global
//! mutable registry.
//!
//! Strategy registration should therefore be explicit and dependency-injected.
//!
//! ## Serialization
//!
//! `quantum::resilience::serialization` may serialize public mitigation
//! descriptors, selection results, execution metadata, and provenance.
//!
//! This module does not define a second serialization format.
//!
//! # Safety
//!
//! The mitigation subsystem is intended to be safe Rust.
//!
//! No `unsafe` implementation is permitted in this module or its child
//! mitigation modules.
//!
//! This composition root therefore explicitly forbids unsafe Rust.
//!
//! The following are prohibited:
//!
//! - `unsafe` blocks;
//! - `unsafe fn`;
//! - raw-pointer execution interfaces;
//! - unsafe global state;
//! - hidden mutable singletons;
//! - provider-specific unsafe FFI in mitigation contracts.
//!
//! Hardware/provider FFI, where unavoidable, belongs behind the hardware HAL
//! boundary and must not leak into mitigation strategy contracts.
//!
//! # Determinism
//!
//! This module performs no stochastic computation itself.
//!
//! Concrete stochastic mitigation strategies must expose enough configuration
//! and provenance to reproduce an execution when deterministic behavior is
//! requested.
//!
//! In particular:
//!
//! ```text
//! strategy identity
//! strategy version
//! configuration
//! randomization seed/provenance
//! execution variant identity
//! target snapshot
//! ```
//!
//! must be available to the appropriate execution and verification layers.
//!
//! The module root must never create hidden randomness.
//!
//! # State ownership
//!
//! This module owns no mutable runtime state.
//!
//! Strategy instances are supplied explicitly by callers/registries.
//!
//! There is no:
//!
//! - global strategy singleton;
//! - global backend;
//! - global RNG;
//! - global device;
//! - global cache;
//! - global telemetry collector.
//!
//! This is necessary for deterministic testing, parallel execution, multi-QPU
//! execution, distributed execution, and long-running Zamani processes.
//!
//! # Scalability
//!
//! The child modules must scale with the resources actually available to an
//! execution target.
//!
//! The composition boundary does not allocate based on a fixed machine size.
//!
//! For example, it must remain valid for:
//!
//! ```text
//! one logical qubit
//!         │
//!         ▼
//! small QPU
//!         │
//!         ▼
//! large QPU
//!         │
//!         ▼
//! fault-tolerant logical system
//!         │
//!         ▼
//! heterogeneous quantum fleet
//!         │
//!         ▼
//! distributed quantum execution
//! ```
//!
//! The module structure itself is therefore independent of machine scale.
//!
//! # No provider coupling
//!
//! Provider-specific identifiers must not be introduced into this module.
//!
//! The mitigation layer operates on capability and execution contracts rather
//! than names of hardware vendors.
//!
//! A new provider should therefore be integrated through:
//!
//! ```text
//! hardware adapter
//!        │
//!        ▼
//! capability model
//!        │
//!        ▼
//! mitigation context
//!        │
//!        ▼
//! mitigation strategy
//! ```
//!
//! and should not require modification of this module.
//!
//! # Semantic boundary
//!
//! Mitigation is not error correction.
//!
//! ```text
//! QEC
//! │
//! ├── encoding
//! ├── syndrome extraction
//! ├── decoding
//! └── logical correction
//!
//! Mitigation
//! │
//! ├── noise suppression
//! ├── noise transformation
//! ├── noise amplification + extrapolation
//! ├── probabilistic reconstruction
//! └── measurement correction
//! ```
//!
//! QEC remains owned by `quantum::error_correction`.
//!
//! Noise/fault semantics remain owned by `quantum::zqn`.
//!
//! Hardware capabilities remain owned by `quantum::hardware`.
//!
//! Canonical computation semantics remain owned by `quantum::ir`.
//!
//! # Why there are no wildcard re-exports
//!
//! This module deliberately does not do:
//!
//! ```text
//! pub use strategy::*;
//! pub use selection::*;
//! pub use executor::*;
//! ```
//!
//! Wildcard re-exports would make ownership ambiguous, create accidental name
//! collisions, and make this composition root change whenever an unrelated
//! child module adds a public item.
//!
//! Consumers should use explicit namespaces:
//!
//! ```text
//! crate::quantum::resilience::mitigation::strategy::MitigationStrategy
//! crate::quantum::resilience::mitigation::selection::SelectionConfig
//! crate::quantum::resilience::mitigation::executor::...
//! ```
//!
//! This keeps the API stable as the subsystem grows.
//!
//! # Public API policy
//!
//! Child modules remain public because they are independently useful contracts.
//!
//! This root only defines the module topology.
//!
//! New mitigation functionality should normally be added as a new child module
//! implementing the existing strategy contract rather than by expanding this
//! file with algorithmic logic.
//!
//! # Rust compatibility
//!
//! This file is compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! The repository's Cargo configuration targets Rust 1.97.1, so this module
//! intentionally uses only long-stable language constructs.
//!
//! # Module declarations
//!
//! The declaration order is intentionally dependency-oriented:
//!
//! 1. `strategy` — foundational contracts;
//! 2. `selection` — consumes strategy contracts;
//! 3. concrete strategy modules — implement the contract;
//! 4. `executor` — orchestrates execution after selection.
//!
//! Rust does not require declaration order for sibling-module resolution, but
//! this ordering documents the architectural dependency direction.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Stable mitigation strategy contracts and shared domain types.
///
/// This module is the foundational dependency for all concrete mitigation
/// implementations.
pub mod strategy;

/// Deterministic, policy-aware strategy selection.
///
/// This module selects candidates but does not execute them.
pub mod selection;

/// Measurement/readout error mitigation.
///
/// Operates on measurement results and associated calibration/characterization
/// contracts.
pub mod readout;

/// Zero-noise extrapolation and related noise-scaling mechanisms.
///
/// Execution variants and extrapolation choices remain configuration- and
/// policy-driven.
pub mod zero_noise;

/// Probabilistic error cancellation and related probabilistic mitigation
/// mechanisms.
///
/// Sampling overhead and reconstruction behavior remain target- and
/// policy-dependent.
pub mod probabilistic;

/// Randomized compiling and twirling mechanisms.
///
/// Randomness and provenance must remain explicit.
pub mod twirling;

/// Dynamical-decoupling mechanisms.
///
/// Scheduling, timing, and pulse capabilities remain owned by their respective
/// quantum subsystems.
pub mod dynamical_decoupling;

/// Extension point for custom/provider-independent mitigation strategies.
///
/// Custom implementations must still satisfy the common `strategy` contract.
pub mod custom;

/// Execution orchestration for an already-selected mitigation strategy.
///
/// Execution is intentionally declared after the domain/strategy modules in
/// this composition boundary. It must consume contracts rather than define
/// them.
pub mod executor;