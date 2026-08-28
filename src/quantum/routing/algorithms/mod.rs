//! Zamani Quantum Routing — Routing Algorithms
//!
//! Production algorithm namespace for the Zamani quantum logical-to-physical
//! routing subsystem.
//!
//! # Responsibility
//!
//! This module owns the routing-algorithm abstraction and the public namespace
//! for concrete routing strategies.
//!
//! Concrete algorithms are deliberately separated into independent modules:
//!
//! - [`basic`] — deterministic greedy routing;
//! - [`shortest_path`] — deterministic shortest-path routing;
//! - [`lookahead`] — front-layer/future-interaction lookahead routing;
//! - [`sabre`] — SABRE/LightSABRE-style heuristic routing;
//! - [`noise_aware`] — calibration/error/duration-aware routing;
//! - [`dynamic`] — adaptive routing for changing target conditions.
//!
//! This module does NOT own:
//!
//! - topology storage;
//! - logical/physical mapping storage;
//! - path-finding primitives;
//! - cost-model implementations;
//! - layout selection;
//! - compiler IR;
//! - OpenQASM parsing;
//! - hardware-provider APIs;
//! - scheduling;
//! - pulse generation;
//! - quantum simulation;
//! - QEC decoding;
//! - benchmark execution.
//!
//! Those responsibilities belong to their respective routing or quantum
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Quantum IR
//!                             │
//!                             ▼
//!                    routing::layout
//!                             │
//!                             ▼
//!                    routing::mapping
//!                             │
//!                             ▼
//!                    routing::algorithms
//!                             │
//!             ┌───────────────┼────────────────┐
//!             │               │                │
//!             ▼               ▼                ▼
//!          Basic         ShortestPath        SABRE
//!             │               │                │
//!             ├───────────────┼────────────────┤
//!             │               │                │
//!             ▼               ▼                ▼
//!       Lookahead       NoiseAware          Dynamic
//!             │               │                │
//!             └───────────────┼────────────────┘
//!                             ▼
//!                     routing::router
//!                             │
//!                             ▼
//!                  routing::verification
//!                             │
//!                             ▼
//!                    Hardware lowering
//! ```
//!
//! # Stable contract
//!
//! Concrete algorithms should implement [`RoutingAlgorithm`].
//!
//! The public router should depend on the trait rather than directly encoding
//! algorithm-specific implementation details.
//!
//! This gives Zamani the ability to:
//!
//! - add new routing algorithms;
//! - register experimental algorithms;
//! - run multiple deterministic trials;
//! - compare candidates using a common cost model;
//! - select algorithms automatically;
//! - execute SABRE variants;
//! - add future research algorithms;
//! - support provider-independent routing;
//! - support hardware-specific routing without changing the router contract.
//!
//! # Important ownership rule
//!
//! `RoutingAlgorithm` is a *behavioral contract*.
//!
//! [`RoutingAlgorithm`] must not contain an algorithm registry, global mutable
//! state, provider SDK, or compiler-specific implementation.
//!
//! Algorithm selection belongs to the higher-level router/configuration layer.
//!
//! # Determinism
//!
//! An implementation must honor deterministic routing when requested by the
//! routing configuration.
//!
//! In deterministic mode, equivalent inputs must produce reproducible output.
//!
//! If an implementation uses randomness, it must obtain its seed from the
//! routing input/configuration rather than using process-global randomness.
//!
//! # Transactionality
//!
//! Algorithms must treat routing as speculative until the caller commits the
//! resulting [`RoutingResult`].
//!
//! An algorithm must not mutate caller-owned state through hidden global state.
//!
//! The router owns the authoritative transaction boundary.
//!
//! # Safety
//!
//! This module contains no `unsafe` code and must remain safe Rust.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration
//!
//! The intended dependency direction is:
//!
//! ```text
//! types
//!   │
//!   ├── topology
//!   ├── mapping
//!   ├── cost
//!   ├── config
//!   └── result
//!          │
//!          ▼
//!      algorithms
//!          │
//!          ▼
//!        router
//!          │
//!          ▼
//!      transpiler
//! ```
//!
//! Algorithms must never introduce an upward dependency on `router`,
//! `transpiler`, frontend parsing, benchmarking, or hardware-provider
//! implementations.
//!
//! # Concrete algorithm modules
//!
//! These declarations are intentionally explicit so the routing architecture
//! is discoverable from this file.
//!
//! They also establish the final public paths:
//!
//! ```text
//! quantum::routing::algorithms::basic
//! quantum::routing::algorithms::shortest_path
//! quantum::routing::algorithms::lookahead
//! quantum::routing::algorithms::sabre
//! quantum::routing::algorithms::noise_aware
//! quantum::routing::algorithms::dynamic
//! ```
//!
//! The files themselves are implemented independently against the contracts
//! already established by the other routing foundation modules.
//!
//! # No fallback implementations
//!
//! `mod.rs` intentionally does not provide placeholder algorithm
//! implementations. A missing concrete algorithm file is a build-time
//! integration error rather than a silent runtime fallback.
//!
//! This prevents production builds from accidentally selecting a fake or
//! incomplete routing implementation.
//!
//! # Algorithm selection
//!
//! `RoutingAlgorithm` in `config.rs` is the *configuration identifier*.
//!
//! `RoutingAlgorithm` in this module is the *behavioral trait*.
//!
//! They intentionally have different responsibilities.
//!
//! The higher-level router maps configuration identifiers to implementations.
//!
//! ```text
//! RoutingConfig
//!      │
//!      │ RoutingAlgorithm::Sabre
//!      ▼
//! router selection
//!      │
//!      ▼
//! algorithms::sabre::SabreRouter
//!      │
//!      ▼
//! RoutingResult
//! ```
//!
//! This separation prevents configuration from depending on concrete
//! algorithm implementations.
//!
//! # Public re-exports
//!
//! Only stable algorithm contracts and stable concrete algorithm types should
//! be re-exported here.
//!
//! Internal helper types should remain inside their owning implementation
//! modules unless they become part of the stable public API.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Concrete routing algorithms
// =============================================================================

/// Deterministic greedy routing.
///
/// This is the baseline algorithm and should be used as a correctness
/// reference and fallback for configurations that explicitly request a simple
/// deterministic strategy.
pub mod basic;

/// Deterministic shortest-path routing.
///
/// This is the direct production evolution of the shortest-path behavior that
/// currently exists in `transpiler.rs`.
pub mod shortest_path;

/// Lookahead routing.
///
/// Evaluates candidate movement against both the current front layer and a
/// bounded future interaction window.
pub mod lookahead;

/// SABRE/LightSABRE-style heuristic routing.
///
/// Intended to be the primary general-purpose heuristic implementation for
/// difficult connectivity-constrained circuits.
pub mod sabre;

/// Hardware-aware routing.
///
/// Uses hardware topology properties, gate duration, error/fidelity and other
/// available target metadata through the routing cost abstraction.
pub mod noise_aware;

/// Dynamic/adaptive routing.
///
/// Intended for targets whose availability, calibration, connectivity, or
/// execution conditions can change during a routing workflow.
pub mod dynamic;

// =============================================================================
// Routing algorithm contract
// =============================================================================

use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::result::RoutingResult;
use crate::quantum::routing::types::RoutingInput;

/// Common behavioral contract implemented by every routing algorithm.
///
/// The trait deliberately contains no topology-specific implementation and no
/// compiler IR dependency.
///
/// # Design requirements
///
/// Every implementation must:
///
/// 1. validate the routing input before making routing decisions;
/// 2. preserve logical gate semantics;
/// 3. never create an invalid physical mapping;
/// 4. emit only legal routing operations;
/// 5. honor the supplied routing configuration;
/// 6. honor deterministic execution when requested;
/// 7. honor configured iteration/candidate/resource limits;
/// 8. report failure instead of silently degrading correctness;
/// 9. avoid mutating global process state;
/// 10. return a complete `RoutingResult` or a structured `RoutingError`.
///
/// # Ownership
///
/// The algorithm receives an immutable [`RoutingInput`].
///
/// Speculative mapping/move state is owned by the algorithm's implementation
/// or by the routing primitives it uses.
///
/// The authoritative externally visible commit is performed by the higher
/// routing layer.
///
/// # Why immutable input?
///
/// A routing algorithm should not be able to partially modify the caller's
/// circuit and then fail, leaving the caller with a half-routed program.
///
/// The router can therefore enforce:
///
/// ```text
/// input
///   │
///   ▼
/// algorithm
///   │
///   ├── success ──► RoutingResult ──► commit
///   │
///   └── failure ──► RoutingError ───► rollback/no mutation
/// ```
///
/// # Thread safety
///
/// The trait itself does not require `Send` or `Sync`.
///
/// Concrete implementations that are intended to run in parallel routing
/// trials should additionally implement those auto traits naturally through
/// their contained data.
///
/// No unsafe implementation is permitted.
///
/// # Determinism
///
/// Deterministic behavior is a property of the implementation plus the
/// supplied [`RoutingConfig`].
///
/// Implementations must not call an uncontrolled global RNG when deterministic
/// execution is requested.
pub trait RoutingAlgorithm {
    /// Returns the stable algorithm identifier used for diagnostics and
    /// reproducibility metadata.
    fn name(&self) -> &'static str;

    /// Routes a circuit under the supplied configuration.
    ///
    /// The returned result contains the routed representation, mapping
    /// information and routing metrics defined by `result.rs`.
    fn route(
        &self,
        input: &RoutingInput,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError>;

    /// Returns whether this implementation supports the supplied configuration.
    ///
    /// The default implementation accepts the configuration. Algorithms with
    /// stricter requirements should override this method.
    fn supports(&self, _config: &RoutingConfig) -> bool {
        true
    }

    /// Returns a stable implementation/version identifier.
    ///
    /// This is deliberately separate from `name()` so routing results can
    /// distinguish algorithm families from implementation revisions.
    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

// =============================================================================
// Stable algorithm capability metadata
// =============================================================================

/// Describes capabilities exposed by a concrete routing algorithm.
///
/// This metadata allows `router.rs` to select an algorithm without knowing
/// implementation internals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingAlgorithmCapabilities {
    /// Supports one- and two-qubit routing.
    pub supports_two_qubit: bool,

    /// Can process native multi-qubit interactions when the target permits
    /// them.
    pub supports_multi_qubit: bool,

    /// Supports directed gate constraints.
    pub supports_directed_gates: bool,

    /// Supports weighted topology/cost information.
    pub supports_weighted_costs: bool,

    /// Supports hardware error/fidelity information.
    pub supports_noise_awareness: bool,

    /// Supports duration-aware routing.
    pub supports_duration_awareness: bool,

    /// Supports lookahead/future interaction analysis.
    pub supports_lookahead: bool,

    /// Supports deterministic seeded trials.
    pub supports_deterministic_trials: bool,

    /// Supports dynamic/adaptive target information.
    pub supports_dynamic_targets: bool,

    /// Supports parallel independent trials.
    pub supports_parallel_trials: bool,
}

impl RoutingAlgorithmCapabilities {
    /// Baseline capabilities shared by simple deterministic algorithms.
    pub const BASIC: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: false,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: false,
        supports_duration_awareness: false,
        supports_lookahead: false,
        supports_deterministic_trials: true,
        supports_dynamic_targets: false,
        supports_parallel_trials: false,
    };

    /// Capabilities expected from shortest-path routing.
    pub const SHORTEST_PATH: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: false,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: false,
        supports_duration_awareness: true,
        supports_lookahead: false,
        supports_deterministic_trials: true,
        supports_dynamic_targets: false,
        supports_parallel_trials: false,
    };

    /// Capabilities expected from lookahead routing.
    pub const LOOKAHEAD: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: false,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: true,
        supports_duration_awareness: true,
        supports_lookahead: true,
        supports_deterministic_trials: true,
        supports_dynamic_targets: false,
        supports_parallel_trials: true,
    };

    /// Capabilities expected from SABRE/LightSABRE.
    pub const SABRE: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: false,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: true,
        supports_duration_awareness: true,
        supports_lookahead: true,
        supports_deterministic_trials: true,
        supports_dynamic_targets: false,
        supports_parallel_trials: true,
    };

    /// Capabilities expected from noise-aware routing.
    pub const NOISE_AWARE: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: false,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: true,
        supports_duration_awareness: true,
        supports_lookahead: true,
        supports_deterministic_trials: true,
        supports_dynamic_targets: false,
        supports_parallel_trials: true,
    };

    /// Capabilities expected from dynamic routing.
    pub const DYNAMIC: Self = Self {
        supports_two_qubit: true,
        supports_multi_qubit: true,
        supports_directed_gates: true,
        supports_weighted_costs: true,
        supports_noise_awareness: true,
        supports_duration_awareness: true,
        supports_lookahead: true,
        supports_deterministic_trials: true,
        supports_dynamic_targets: true,
        supports_parallel_trials: true,
    };
}

// =============================================================================
// Stable algorithm identifiers
// =============================================================================

/// Stable identifier for the built-in routing algorithms.
///
/// This is intentionally separate from `config::RoutingAlgorithm`, whose
/// `Custom(String)` variant is needed at configuration boundaries.
///
/// This enum is useful when code needs to reason specifically about built-in
/// implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BuiltinRoutingAlgorithm {
    /// Deterministic greedy routing.
    Basic,

    /// Deterministic shortest-path routing.
    ShortestPath,

    /// Lookahead routing.
    Lookahead,

    /// SABRE/LightSABRE-style routing.
    Sabre,

    /// Hardware-aware routing.
    NoiseAware,

    /// Dynamic routing.
    Dynamic,
}

impl BuiltinRoutingAlgorithm {
    /// Returns the stable machine-readable identifier.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::ShortestPath => "shortest_path",
            Self::Lookahead => "lookahead",
            Self::Sabre => "sabre",
            Self::NoiseAware => "noise_aware",
            Self::Dynamic => "dynamic",
        }
    }

    /// Returns the capabilities expected from this built-in algorithm.
    pub const fn capabilities(self) -> RoutingAlgorithmCapabilities {
        match self {
            Self::Basic => RoutingAlgorithmCapabilities::BASIC,
            Self::ShortestPath => RoutingAlgorithmCapabilities::SHORTEST_PATH,
            Self::Lookahead => RoutingAlgorithmCapabilities::LOOKAHEAD,
            Self::Sabre => RoutingAlgorithmCapabilities::SABRE,
            Self::NoiseAware => RoutingAlgorithmCapabilities::NOISE_AWARE,
            Self::Dynamic => RoutingAlgorithmCapabilities::DYNAMIC,
        }
    }
}

impl std::fmt::Display for BuiltinRoutingAlgorithm {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Public concrete algorithm re-exports
// =============================================================================
//
// These are intentionally kept narrow. Concrete modules may expose additional
// implementation details internally, but the routing algorithms namespace
// should expose only the primary production algorithm types here.
//
// The exact concrete type names are part of the integration contract for the
// subsequent implementation files.

pub use basic::BasicRouter;
pub use dynamic::DynamicRouter;
pub use lookahead::LookaheadRouter;
pub use noise_aware::NoiseAwareRouter;
pub use sabre::SabreRouter;
pub use shortest_path::ShortestPathRouter;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_algorithm_names_are_stable() {
        assert_eq!(BuiltinRoutingAlgorithm::Basic.name(), "basic");
        assert_eq!(
            BuiltinRoutingAlgorithm::ShortestPath.name(),
            "shortest_path"
        );
        assert_eq!(BuiltinRoutingAlgorithm::Lookahead.name(), "lookahead");
        assert_eq!(BuiltinRoutingAlgorithm::Sabre.name(), "sabre");
        assert_eq!(BuiltinRoutingAlgorithm::NoiseAware.name(), "noise_aware");
        assert_eq!(BuiltinRoutingAlgorithm::Dynamic.name(), "dynamic");
    }

    #[test]
    fn builtin_algorithm_capabilities_are_consistent() {
        assert!(
            BuiltinRoutingAlgorithm::Basic
                .capabilities()
                .supports_two_qubit
        );

        assert!(
            BuiltinRoutingAlgorithm::ShortestPath
                .capabilities()
                .supports_two_qubit
        );

        assert!(
            BuiltinRoutingAlgorithm::Lookahead
                .capabilities()
                .supports_lookahead
        );

        assert!(
            BuiltinRoutingAlgorithm::Sabre
                .capabilities()
                .supports_lookahead
        );

        assert!(
            BuiltinRoutingAlgorithm::NoiseAware
                .capabilities()
                .supports_noise_awareness
        );

        assert!(
            BuiltinRoutingAlgorithm::Dynamic
                .capabilities()
                .supports_dynamic_targets
        );
    }

    #[test]
    fn capabilities_are_compile_time_stable_values() {
        let basic = RoutingAlgorithmCapabilities::BASIC;

        assert!(basic.supports_two_qubit);
        assert!(basic.supports_deterministic_trials);
        assert!(!basic.supports_noise_awareness);
        assert!(!basic.supports_dynamic_targets);
    }

    #[test]
    fn algorithm_trait_default_contract_is_safe() {
        struct TestAlgorithm;

        impl RoutingAlgorithm for TestAlgorithm {
            fn name(&self) -> &'static str {
                "test"
            }

            fn route(
                &self,
                _input: &RoutingInput,
                _config: &RoutingConfig,
            ) -> Result<RoutingResult, RoutingError> {
                unreachable!("contract smoke test does not execute routing")
            }
        }

        let algorithm = TestAlgorithm;

        assert_eq!(algorithm.name(), "test");
        assert_eq!(algorithm.version(), "1.0.0");
    }
}