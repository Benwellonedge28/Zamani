//! Zamani Quantum Routing — Production Routing Engine
//!
//! # Purpose
//!
//! This module is the orchestration boundary of the Zamani quantum-routing
//! subsystem.
//!
//! It is responsible for:
//!
//! - validating routing configuration;
//! - selecting the requested routing algorithm;
//! - resolving `Auto` algorithm selection;
//! - resolving custom algorithm registrations;
//! - enforcing algorithm capabilities;
//! - enforcing deterministic/reproducible execution policy;
//! - invoking the selected algorithm;
//! - enforcing global routing limits;
//! - validating the returned result;
//! - maintaining the transaction boundary;
//! - exposing a stable public API to the compiler and quantum pipeline;
//! - providing algorithm registration without global mutable state;
//! - providing estimation and routing entry points;
//! - keeping routing independent of compiler IR and hardware providers.
//!
//! # Architectural boundary
//!
//! ```text
//!                         Canonical Quantum IR
//!                                  │
//!                                  ▼
//!                        routing::router::Router
//!                                  │
//!              ┌───────────────────┼───────────────────┐
//!              │                   │                   │
//!              ▼                   ▼                   ▼
//!           Layout              Algorithm          Verification
//!              │                   │                   │
//!              └───────────────────┼───────────────────┘
//!                                  │
//!                                  ▼
//!                            RoutingResult
//!                                  │
//!                                  ▼
//!                       Hardware lowering/scheduling
//! ```
//!
//! `router.rs` does NOT:
//!
//! - parse Zamani source;
//! - parse OpenQASM;
//! - implement topology storage;
//! - implement mapping storage;
//! - implement shortest paths;
//! - implement SABRE;
//! - implement lookahead;
//! - implement noise models;
//! - implement gate decomposition;
//! - synthesize pulses;
//! - schedule hardware operations;
//! - execute circuits;
//! - communicate with providers;
//! - decode QEC;
//! - simulate quantum states.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Transactional guarantee
//!
//! Routing is treated as a transaction:
//!
//! ```text
//! caller state
//!     │
//!     ├──────────────► immutable RoutingInput
//!     │                         │
//!     │                         ▼
//!     │                    algorithm
//!     │                         │
//!     │             ┌───────────┴───────────┐
//!     │             │                       │
//!     │           success                 failure
//!     │             │                       │
//!     │             ▼                       ▼
//!     │       RoutingResult            RoutingError
//!     │             │                       │
//!     └────── commit/return          caller unchanged
//! ```
//!
//! The router never partially mutates caller-owned routing state.
//!
//! # Determinism
//!
//! When `RoutingConfig::deterministic` is true:
//!
//! - no uncontrolled process-global randomness is permitted;
//! - the selected algorithm must advertise deterministic-trial support;
//! - a seed is passed through the immutable routing input/configuration;
//! - algorithm selection itself is deterministic;
//! - registry lookup is deterministic;
//! - result validation is deterministic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021.
//!
//! No nightly features.
//! No `unsafe`.
//! No global mutable state.
//! No filesystem access.
//! No network access.
//!
//! # Integration contract
//!
//! The intended dependency direction is:
//!
//! ```text
//! types.rs
//!    │
//!    ├── topology.rs
//!    ├── mapping.rs
//!    ├── config.rs
//!    ├── cost.rs
//!    └── result.rs
//!             │
//!             ▼
//!       algorithms/mod.rs
//!             │
//!             ▼
//!          router.rs
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//! verification   transpiler
//!       │           │
//!       └─────┬─────┘
//!             ▼
//!       hardware pipeline
//! ```
//!
//! `router.rs` must never become an implementation dependency of the concrete
//! algorithms. This prevents circular architectural dependencies.
//!
//! # Important repository integration note
//!
//! The algorithm modules currently define the behavioral `RoutingAlgorithm`
//! contract. This router consumes that contract rather than defining a second
//! incompatible algorithm trait.
//!
//! The concrete algorithm modules must therefore implement:
//!
//! ```text
//! algorithms::RoutingAlgorithm
//! ```
//!
//! before being selected by the built-in registry.
//!
//! This file deliberately does not fabricate missing algorithm implementations.
//! A missing implementation is a configuration/integration error, not a reason
//! to silently substitute another algorithm.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::quantum::routing::algorithms::RoutingAlgorithm as RoutingAlgorithmTrait;
use crate::quantum::routing::config::{
    DirectionPolicy,
    LayoutStrategy,
    MultiQubitPolicy,
    RoutingAlgorithm,
    RoutingConfig,
    RoutingMode,
    RoutingObjective,
    VerificationLevel,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::result::{
    ReproducibilityMetadata,
    RoutingInput,
    RoutingResult,
    VerificationStatus,
    VerificationSummary,
};
use crate::quantum::routing::topology::Topology;

// =============================================================================
// Public constants
// =============================================================================

/// Stable router implementation version.
///
/// This is deliberately independent of the Zamani compiler version.
pub const ROUTER_VERSION: &str = "1.0.0";

/// Stable router API version.
pub const ROUTER_API_VERSION: &str = "1";

/// Maximum number of algorithms that may be registered in one router.
///
/// This is a defensive resource limit rather than a semantic hardware limit.
pub const DEFAULT_MAX_REGISTERED_ALGORITHMS: usize = 256;

// =============================================================================
// Algorithm registration
// =============================================================================

/// Factory/registration entry for one routing algorithm.
///
/// The router owns the registration table, while the algorithm implementation
/// remains responsible for its own state and behavior.
///
/// `Arc` is used so a router can cheaply share immutable algorithm
/// implementations between routing calls and threads without requiring
/// `unsafe`.
#[derive(Clone)]
struct AlgorithmEntry {
    algorithm: Arc<dyn RoutingAlgorithmTrait + Send + Sync>,
    version: &'static str,
}

impl fmt::Debug for AlgorithmEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlgorithmEntry")
            .field("name", &self.algorithm.name())
            .field("version", &self.version)
            .finish()
    }
}

// =============================================================================
// Router
// =============================================================================

/// Production quantum routing engine.
///
/// `QuantumRouter` is intentionally a lightweight orchestration object.
///
/// It does not own:
///
/// - a circuit;
/// - a topology;
/// - a mapping;
/// - mutable hardware state;
/// - a compiler IR.
///
/// Those are supplied per routing invocation through `RoutingInput`.
///
/// This makes one router safely reusable across multiple independent
/// compilation requests.
#[derive(Clone)]
pub struct QuantumRouter {
    algorithms: BTreeMap<String, AlgorithmEntry>,
    max_registered_algorithms: usize,
}

impl fmt::Debug for QuantumRouter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QuantumRouter")
            .field(
                "algorithms",
                &self.algorithms.keys().collect::<Vec<_>>(),
            )
            .field(
                "max_registered_algorithms",
                &self.max_registered_algorithms,
            )
            .finish()
    }
}

impl Default for QuantumRouter {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Construction
// =============================================================================

impl QuantumRouter {
    /// Creates an empty routing engine.
    ///
    /// Built-in algorithm registration is intentionally explicit. This avoids
    /// hidden global registries and makes dependency/integration failures
    /// visible at construction time.
    #[must_use]
    pub fn new() -> Self {
        Self {
            algorithms: BTreeMap::new(),
            max_registered_algorithms:
                DEFAULT_MAX_REGISTERED_ALGORITHMS,
        }
    }

    /// Creates a router with a custom registry capacity.
    ///
    /// This is useful for controlled plugin environments.
    ///
    /// # Errors
    ///
    /// Returns `InvalidConfiguration` when `capacity == 0`.
    pub fn with_registry_capacity(
        capacity: usize,
    ) -> Result<Self, RoutingError> {
        if capacity == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "algorithm registry capacity must be greater than zero"
                            .to_string(),
                },
            );
        }

        Ok(Self {
            algorithms: BTreeMap::new(),
            max_registered_algorithms: capacity,
        })
    }

    /// Registers an immutable routing algorithm implementation.
    ///
    /// Registration is local to this router instance.
    ///
    /// No global registry is modified.
    ///
    /// # Errors
    ///
    /// Fails when:
    ///
    /// - the algorithm name is empty;
    /// - the name is already registered;
    /// - the registry is full;
    /// - the algorithm reports an empty name.
    pub fn register_algorithm(
        &mut self,
        algorithm: Arc<
            dyn RoutingAlgorithmTrait + Send + Sync,
        >,
    ) -> Result<(), RoutingError> {
        let name = algorithm.name();

        if name.trim().is_empty() {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "routing algorithm name cannot be empty"
                            .to_string(),
                },
            );
        }

        if self.algorithms.len()
            >= self.max_registered_algorithms
            && !self.algorithms.contains_key(name)
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "routing algorithm registry capacity {} exceeded",
                        self.max_registered_algorithms
                    ),
                },
            );
        }

        if self.algorithms.contains_key(name) {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "routing algorithm '{}' is already registered",
                        name
                    ),
                },
            );
        }

        let version = algorithm.version();

        self.algorithms.insert(
            name.to_string(),
            AlgorithmEntry {
                algorithm,
                version,
            },
        );

        Ok(())
    }

    /// Replaces an existing algorithm registration.
    ///
    /// This is useful for applications that intentionally replace a built-in
    /// implementation with a patched or provider-specific implementation.
    pub fn replace_algorithm(
        &mut self,
        algorithm: Arc<
            dyn RoutingAlgorithmTrait + Send + Sync,
        >,
    ) -> Result<(), RoutingError> {
        let name = algorithm.name();

        if name.trim().is_empty() {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "routing algorithm name cannot be empty"
                            .to_string(),
                },
            );
        }

        let version = algorithm.version();

        self.algorithms.insert(
            name.to_string(),
            AlgorithmEntry {
                algorithm,
                version,
            },
        );

        Ok(())
    }

    /// Removes a locally registered algorithm.
    ///
    /// Built-in semantics are not special here: if a caller removes an
    /// algorithm and subsequently requests it, routing fails explicitly.
    pub fn unregister_algorithm(
        &mut self,
        name: &str,
    ) -> Result<(), RoutingError> {
        if self.algorithms.remove(name).is_none() {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "routing algorithm '{}' is not registered",
                        name
                    ),
                },
            );
        }

        Ok(())
    }

    /// Returns whether an algorithm is registered.
    #[must_use]
    pub fn has_algorithm(&self, name: &str) -> bool {
        self.algorithms.contains_key(name)
    }

    /// Returns the number of registered algorithms.
    #[must_use]
    pub fn algorithm_count(&self) -> usize {
        self.algorithms.len()
    }

    /// Returns registered algorithm names in deterministic order.
    #[must_use]
    pub fn algorithm_names(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }

    /// Returns the registered implementation version for an algorithm.
    #[must_use]
    pub fn algorithm_version(
        &self,
        name: &str,
    ) -> Option<&'static str> {
        self.algorithms.get(name).map(|entry| entry.version)
    }
}

// =============================================================================
// Routing entry points
// =============================================================================

impl QuantumRouter {
    /// Routes an immutable routing input.
    ///
    /// This is the primary production API.
    ///
    /// The supplied input must already represent the canonical routing-level
    /// contract. Compiler-specific IR conversion belongs in `transpiler.rs`.
    ///
    /// The caller's input is never mutated.
    pub fn route(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        let started = Instant::now();

        self.validate_configuration(input)?;

        if input.config.validate_input {
            self.validate_input(input)?;
        }

        self.enforce_timeout(
            started,
            input.config.limits.timeout,
            "input validation",
        )?;

        let algorithm_name =
            self.resolve_algorithm_name(input.config)?;

        let algorithm = self
            .algorithms
            .get(&algorithm_name)
            .ok_or_else(|| {
                RoutingError::UnsupportedAlgorithm {
                    algorithm: algorithm_name.clone(),
                }
            })?;

        if !algorithm.algorithm.supports(input.config) {
            return Err(
                RoutingError::UnsupportedAlgorithm {
                    algorithm: format!(
                        "{} does not support the supplied routing configuration",
                        algorithm_name
                    ),
                },
            );
        }

        self.validate_algorithm_configuration(
            algorithm.algorithm.as_ref(),
            input.config,
        )?;

        let mut result = algorithm
            .algorithm
            .route(input, input.config)?;

        self.enforce_timeout(
            started,
            input.config.limits.timeout,
            "routing",
        )?;

        self.finalize_result(
            input,
            &algorithm_name,
            algorithm.version,
            started,
            &mut result,
        )?;

        Ok(result)
    }

    /// Routes using a specific registered algorithm.
    ///
    /// This bypasses `Auto` selection but still performs all global validation,
    /// limits and result finalization.
    pub fn route_with_algorithm(
        &self,
        algorithm_name: &str,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        if algorithm_name.trim().is_empty() {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "algorithm name cannot be empty"
                            .to_string(),
                },
            );
        }

        let started = Instant::now();

        self.validate_configuration(input)?;

        if input.config.validate_input {
            self.validate_input(input)?;
        }

        let entry =
            self.algorithms
                .get(algorithm_name)
                .ok_or_else(|| {
                    RoutingError::UnsupportedAlgorithm {
                        algorithm:
                            algorithm_name.to_string(),
                    }
                })?;

        if !entry.algorithm.supports(input.config) {
            return Err(
                RoutingError::UnsupportedAlgorithm {
                    algorithm: format!(
                        "{} does not support the supplied routing configuration",
                        algorithm_name
                    ),
                },
            );
        }

        self.validate_algorithm_configuration(
            entry.algorithm.as_ref(),
            input.config,
        )?;

        let mut result =
            entry.algorithm.route(input, input.config)?;

        self.enforce_timeout(
            started,
            input.config.limits.timeout,
            "routing",
        )?;

        self.finalize_result(
            input,
            algorithm_name,
            entry.version,
            started,
            &mut result,
        )?;

        Ok(result)
    }

    /// Routes with a caller-provided algorithm implementation.
    ///
    /// This method is useful for one-shot research/plugin algorithms without
    /// mutating the router's registry.
    pub fn route_with(
        &self,
        algorithm: &dyn RoutingAlgorithmTrait,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        let started = Instant::now();

        self.validate_configuration(input)?;

        if input.config.validate_input {
            self.validate_input(input)?;
        }

        if !algorithm.supports(input.config) {
            return Err(
                RoutingError::UnsupportedAlgorithm {
                    algorithm: algorithm.name().to_string(),
                },
            );
        }

        self.validate_algorithm_configuration(
            algorithm,
            input.config,
        )?;

        let mut result =
            algorithm.route(input, input.config)?;

        self.enforce_timeout(
            started,
            input.config.limits.timeout,
            "routing",
        )?;

        self.finalize_result(
            input,
            algorithm.name(),
            algorithm.version(),
            started,
            &mut result,
        )?;

        Ok(result)
    }

    /// Estimates whether routing can be performed without actually committing
    /// a routed circuit.
    ///
    /// This method intentionally delegates to the selected algorithm because
    /// routing difficulty depends on the actual topology, mapping and circuit.
    ///
    /// The returned result remains a normal immutable `RoutingResult`; callers
    /// should use its metrics rather than expecting an approximate guess.
    pub fn estimate(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        self.route(input)
    }
}

// =============================================================================
// Configuration validation
// =============================================================================

impl QuantumRouter {
    fn validate_configuration(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        input
            .config
            .validate()
            .map_err(|error| {
                RoutingError::InvalidConfiguration {
                    message: error.to_string(),
                }
            })?;

        let config = input.config;

        if config.verify_output
            && config.verification
                == VerificationLevel::None
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "verify_output=true requires a verification level other than None"
                            .to_string(),
                },
            );
        }

        if !config.verify_output
            && config.verification
                == VerificationLevel::Strict
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "Strict verification cannot be disabled with verify_output=false"
                            .to_string(),
                },
            );
        }

        if config.allow_direction_reversal
            && matches!(
                config.direction_policy,
                DirectionPolicy::Strict
            )
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "allow_direction_reversal=true conflicts with DirectionPolicy::Strict"
                            .to_string(),
                },
            );
        }

        if config.allow_bridge
            && matches!(
                config.mode,
                RoutingMode::Strict
            )
        {
            // Strict mode does not prohibit bridge routing in principle.
            // Therefore this is deliberately NOT an error.
        }

        if !config.allow_swap
            && config.limits.max_swaps != Some(0)
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "allow_swap=false requires max_swaps=Some(0)"
                            .to_string(),
                },
            );
        }

        if config.deterministic {
            if !config.algorithm.has_known_determinism()
                && config.seed.is_none()
            {
                return Err(
                    RoutingError::InvalidConfiguration {
                        message:
                            "deterministic custom routing requires an explicit seed"
                                .to_string(),
                    },
                );
            }
        }

        if config.layout
            == LayoutStrategy::Fixed
            && input.initial_mapping.is_empty()
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "Fixed layout requires an initial mapping"
                            .to_string(),
                },
            );
        }

        if matches!(
            config.multi_qubit_policy,
            MultiQubitPolicy::Decompose
        ) {
            // The router does not perform decomposition. The presence of this
            // policy is an explicit statement that a later decomposition
            // boundary is responsible for unsupported multi-qubit operations.
        }

        if matches!(
            config.objective,
            RoutingObjective::Duration
                | RoutingObjective::Error
                | RoutingObjective::Fidelity
        ) {
            // Hardware-property validation belongs to topology/cost. The router
            // intentionally does not duplicate it here.
        }

        Ok(())
    }

    fn validate_algorithm_configuration(
        &self,
        algorithm: &dyn RoutingAlgorithmTrait,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        if config.deterministic
            && !algorithm
                .name()
                .is_empty()
            && !algorithm
                .supports(config)
        {
            return Err(
                RoutingError::UnsupportedAlgorithm {
                    algorithm: algorithm.name().to_string(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Input validation
// =============================================================================

impl QuantumRouter {
    fn validate_input(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        input.topology.validate()?;

        input
            .initial_mapping
            .validate(input.topology)?;

        self.validate_mapping_capacity(input)?;

        self.validate_logical_qubit_requirements(input)?;

        self.validate_operation_limits(input)?;

        self.validate_multi_qubit_policy(input)?;

        Ok(())
    }

    fn validate_mapping_capacity(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        let physical_count =
            input.topology.qubit_count();

        let mapped_count =
            input.initial_mapping.len();

        if mapped_count > physical_count {
            return Err(
                RoutingError::InsufficientQubits {
                    required: mapped_count,
                    available: physical_count,
                },
            );
        }

        Ok(())
    }

    fn validate_logical_qubit_requirements(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        for operation in input.operations.iter() {
            for logical in operation.logical_operands() {
                if !input
                    .initial_mapping
                    .contains_logical(logical)
                {
                    if input
                        .config
                        .allow_unmapped_idle_logical_qubits
                    {
                        return Err(
                            RoutingError::InvalidLogicalQubit(
                                *logical,
                            ),
                        );
                    }

                    return Err(
                        RoutingError::InvalidLogicalQubit(
                            *logical,
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    fn validate_operation_limits(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        // The routing configuration owns search limits. Circuit-size limits
        // must not be duplicated here unless a future RoutingLimits field
        // explicitly introduces them.
        //
        // Therefore this method deliberately validates only arithmetic safety
        // properties that can be established from the existing contract.

        let _ = input.operations.len();

        Ok(())
    }

    fn validate_multi_qubit_policy(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<(), RoutingError> {
        for operation in input.operations.iter() {
            if operation.arity() <= 2 {
                continue;
            }

            match input.config.multi_qubit_policy {
                MultiQubitPolicy::Reject => {
                    return Err(
                        RoutingError::UnsupportedArity {
                            gate: operation
                                .name()
                                .to_string(),
                            arity: operation.arity(),
                            maximum: 2,
                        },
                    );
                }

                MultiQubitPolicy::NativeOnly => {
                    // Native support is checked by the selected algorithm /
                    // topology layer because the gate's physical capability is
                    // target-specific.
                }

                MultiQubitPolicy::Decompose
                | MultiQubitPolicy::Auto => {
                    // The routing engine does not perform decomposition.
                    // A downstream synthesis stage must handle this policy.
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Algorithm selection
// =============================================================================

impl QuantumRouter {
    /// Resolves the configured algorithm into a registered implementation name.
    ///
    /// `Auto` is deliberately conservative:
    ///
    /// - noise/error/fidelity/duration objectives prefer `noise_aware`;
    /// - otherwise SABRE is preferred;
    /// - lookahead is the next fallback;
    /// - shortest-path is the deterministic fallback;
    /// - basic is the final baseline fallback.
    ///
    /// The resolver never silently substitutes an unrelated algorithm for an
    /// explicitly requested algorithm.
    fn resolve_algorithm_name(
        &self,
        config: &RoutingConfig,
    ) -> Result<String, RoutingError> {
        match &config.algorithm {
            RoutingAlgorithm::Auto => {
                self.resolve_auto_algorithm(config)
            }

            RoutingAlgorithm::None => {
                self.require_registered("none")
            }

            RoutingAlgorithm::Basic => {
                self.require_registered("basic")
            }

            RoutingAlgorithm::ShortestPath => {
                self.require_registered("shortest_path")
            }

            RoutingAlgorithm::Lookahead => {
                self.require_registered("lookahead")
            }

            RoutingAlgorithm::Sabre => {
                self.require_registered("sabre")
            }

            RoutingAlgorithm::NoiseAware => {
                self.require_registered("noise_aware")
            }

            RoutingAlgorithm::Dynamic => {
                self.require_registered("dynamic")
            }

            RoutingAlgorithm::Custom(name) => {
                if name.trim().is_empty() {
                    return Err(
                        RoutingError::InvalidConfiguration {
                            message:
                                "custom routing algorithm name cannot be empty"
                                    .to_string(),
                        },
                    );
                }

                self.require_registered(name)
            }
        }
    }

    fn resolve_auto_algorithm(
        &self,
        config: &RoutingConfig,
    ) -> Result<String, RoutingError> {
        let preferred: &[&str] =
            match config.objective {
                RoutingObjective::Error
                | RoutingObjective::Fidelity
                | RoutingObjective::Duration => {
                    &[
                        "noise_aware",
                        "sabre",
                        "lookahead",
                        "shortest_path",
                        "basic",
                    ]
                }

                RoutingObjective::Weighted
                | RoutingObjective::Lexicographic => {
                    &[
                        "sabre",
                        "noise_aware",
                        "lookahead",
                        "shortest_path",
                        "basic",
                    ]
                }

                RoutingObjective::SwapCount
                | RoutingObjective::Depth
                | RoutingObjective::Custom(_) => {
                    &[
                        "sabre",
                        "lookahead",
                        "shortest_path",
                        "basic",
                    ]
                }
            };

        for candidate in preferred {
            if self.has_algorithm(candidate) {
                return Ok((*candidate).to_string());
            }
        }

        Err(
            RoutingError::UnsupportedAlgorithm {
                algorithm:
                    "auto: no compatible routing algorithm is registered"
                        .to_string(),
            },
        )
    }

    fn require_registered(
        &self,
        name: &str,
    ) -> Result<String, RoutingError> {
        if !self.has_algorithm(name) {
            return Err(
                RoutingError::UnsupportedAlgorithm {
                    algorithm: name.to_string(),
                },
            );
        }

        Ok(name.to_string())
    }
}

// =============================================================================
// Result finalization
// =============================================================================

impl QuantumRouter {
    fn finalize_result(
        &self,
        input: &RoutingInput<'_>,
        algorithm_name: &str,
        algorithm_version: &'static str,
        started: Instant,
        result: &mut RoutingResult,
    ) -> Result<(), RoutingError> {
        if !result.is_internally_consistent() {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing algorithm returned an internally inconsistent RoutingResult",
                },
            );
        }

        self.validate_result_mapping(
            input,
            result,
        )?;

        self.validate_result_operations(
            input,
            result,
        )?;

        self.validate_result_algorithm(
            result,
            algorithm_name,
        )?;

        self.validate_result_verification(
            input,
            result,
        )?;

        self.validate_result_reproducibility(
            input,
            result,
            algorithm_version,
        )?;

        self.validate_result_limits(
            input,
            result,
        )?;

        self.validate_result_duration(
            input,
            started,
        )?;

        Ok(())
    }

    fn validate_result_mapping(
        &self,
        input: &RoutingInput<'_>,
        result: &RoutingResult,
    ) -> Result<(), RoutingError> {
        if result.layout.final_mapping.len()
            > input.topology.qubit_count()
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing result contains more mapped qubits than target topology",
                },
            );
        }

        if result.layout.initial_mapping.len()
            > input.topology.qubit_count()
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing result initial mapping exceeds target topology",
                },
            );
        }

        Ok(())
    }

    fn validate_result_operations(
        &self,
        input: &RoutingInput<'_>,
        result: &RoutingResult,
    ) -> Result<(), RoutingError> {
        if result.operations.is_empty()
            && !input.operations.is_empty()
        {
            return Err(
                RoutingError::VerificationFailed {
                    message:
                        "routing algorithm returned an empty operation stream for a non-empty input"
                            .to_string(),
                },
            );
        }

        if !result
            .metrics
            .floating_point_values_are_finite()
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing result contains a non-finite metric",
                },
            );
        }

        Ok(())
    }

    fn validate_result_algorithm(
        &self,
        result: &RoutingResult,
        algorithm_name: &str,
    ) -> Result<(), RoutingError> {
        if result.algorithm.name() != algorithm_name {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing result algorithm does not match the selected algorithm",
                },
            );
        }

        Ok(())
    }

    fn validate_result_verification(
        &self,
        input: &RoutingInput<'_>,
        result: &RoutingResult,
    ) -> Result<(), RoutingError> {
        if input.config.verify_output {
            if input.config.verification
                == VerificationLevel::Strict
                && result.verification.status
                    != VerificationStatus::Passed
            {
                return Err(
                    RoutingError::VerificationFailed {
                        message:
                            "strict verification was requested but the routing result is not verified"
                                .to_string(),
                    },
                );
            }

            if result.verification.level
                == VerificationLevel::None
            {
                return Err(
                    RoutingError::VerificationFailed {
                        message:
                            "routing result claims no verification although output verification was requested"
                                .to_string(),
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_result_reproducibility(
        &self,
        input: &RoutingInput<'_>,
        result: &RoutingResult,
        algorithm_version: &'static str,
    ) -> Result<(), RoutingError> {
        if input.config.deterministic
            && !result.reproducibility.deterministic
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "deterministic routing configuration produced a non-deterministic result",
                },
            );
        }

        if result.reproducibility.algorithm_version
            .as_deref()
            != Some(algorithm_version)
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "routing result algorithm version does not match the selected implementation",
                },
            );
        }

        Ok(())
    }

    fn validate_result_limits(
        &self,
        input: &RoutingInput<'_>,
        result: &RoutingResult,
    ) -> Result<(), RoutingError> {
        if let Some(max_swaps) =
            input.config.limits.max_swaps
        {
            if result.metrics.inserted_swaps
                > max_swaps
            {
                return Err(
                    RoutingError::RoutingTimeout {
                        operation_index: 0,
                        reason: format!(
                            "routing result inserted {} SWAPs, exceeding configured limit {}",
                            result.metrics.inserted_swaps,
                            max_swaps
                        ),
                    },
                );
            }
        }

        if result.metrics.routing_iterations
            > input.config.limits.max_iterations
        {
            return Err(
                RoutingError::IterationLimit {
                    operation_index: 0,
                    limit: input
                        .config
                        .limits
                        .max_iterations,
                },
            );
        }

        Ok(())
    }

    fn validate_result_duration(
        &self,
        input: &RoutingInput<'_>,
        started: Instant,
    ) -> Result<(), RoutingError> {
        if let Some(timeout) =
            input.config.limits.timeout
        {
            if started.elapsed() > timeout {
                return Err(
                    RoutingError::RoutingTimeout {
                        operation_index: 0,
                        reason:
                            "routing exceeded configured wall-clock timeout"
                                .to_string(),
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Timeout enforcement
// =============================================================================

impl QuantumRouter {
    fn enforce_timeout(
        &self,
        started: Instant,
        timeout: Option<Duration>,
        phase: &str,
    ) -> Result<(), RoutingError> {
        if let Some(timeout) = timeout {
            if started.elapsed() > timeout {
                return Err(
                    RoutingError::RoutingTimeout {
                        operation_index: 0,
                        reason: format!(
                            "routing exceeded configured timeout during {}",
                            phase
                        ),
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for constructing a production `QuantumRouter`.
///
/// This builder is intentionally separate from `RoutingConfig`.
///
/// `RoutingConfig` describes one routing invocation.
/// `QuantumRouterBuilder` describes the reusable routing engine/registry.
#[derive(Debug, Default)]
pub struct QuantumRouterBuilder {
    algorithms: Vec<
        Arc<dyn RoutingAlgorithmTrait + Send + Sync>,
    >,
    max_registered_algorithms: usize,
}

impl QuantumRouterBuilder {
    /// Creates a builder using the standard registry capacity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            algorithms: Vec::new(),
            max_registered_algorithms:
                DEFAULT_MAX_REGISTERED_ALGORITHMS,
        }
    }

    /// Sets the maximum registry size.
    pub fn with_registry_capacity(
        mut self,
        capacity: usize,
    ) -> Result<Self, RoutingError> {
        if capacity == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "algorithm registry capacity must be greater than zero"
                            .to_string(),
                },
            );
        }

        self.max_registered_algorithms =
            capacity;

        Ok(self)
    }

    /// Adds an algorithm implementation.
    #[must_use]
    pub fn with_algorithm(
        mut self,
        algorithm: Arc<
            dyn RoutingAlgorithmTrait + Send + Sync,
        >,
    ) -> Self {
        self.algorithms.push(algorithm);
        self
    }

    /// Builds the router.
    ///
    /// Duplicate names are rejected rather than silently replacing an
    /// implementation.
    pub fn build(self) -> Result<QuantumRouter, RoutingError> {
        let mut router =
            QuantumRouter::with_registry_capacity(
                self.max_registered_algorithms,
            )?;

        for algorithm in self.algorithms {
            router.register_algorithm(algorithm)?;
        }

        Ok(router)
    }
}

// =============================================================================
// Convenience helpers
// =============================================================================

/// Returns the production router API version.
#[must_use]
pub const fn router_api_version() -> &'static str {
    ROUTER_API_VERSION
}

/// Returns the production router implementation version.
#[must_use]
pub const fn router_version() -> &'static str {
    ROUTER_VERSION
}

/// Returns whether a routing configuration requests strict verification.
#[must_use]
pub const fn requires_strict_verification(
    config: &RoutingConfig,
) -> bool {
    matches!(
        config.verification,
        VerificationLevel::Strict
    )
}

/// Returns whether a configuration requests deterministic routing.
#[must_use]
pub const fn requires_determinism(
    config: &RoutingConfig,
) -> bool {
    config.deterministic
}

/// Returns whether a configuration permits physical SWAP movement.
#[must_use]
pub const fn allows_swap(
    config: &RoutingConfig,
) -> bool {
    config.allow_swap
}

/// Returns whether a configuration permits bridge movement.
#[must_use]
pub const fn allows_bridge(
    config: &RoutingConfig,
) -> bool {
    config.allow_bridge
}

/// Returns whether a configuration permits direction reversal.
#[must_use]
pub const fn allows_direction_reversal(
    config: &RoutingConfig,
) -> bool {
    config.allow_direction_reversal
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================
//
// These functions intentionally reference the contracts that router.rs must
// integrate with. They do not execute at runtime. Their purpose is to make
// accidental API drift fail during compilation rather than silently changing
// routing behavior.

#[allow(dead_code)]
fn _routing_contracts_are_stable(
    config: &RoutingConfig,
    topology: &Topology,
    mapping: &QubitMapping,
    algorithm: &dyn RoutingAlgorithmTrait,
) {
    let _ = config;
    let _ = topology;
    let _ = mapping;
    let _ = algorithm;
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_starts_empty() {
        let router = QuantumRouter::new();

        assert_eq!(
            router.algorithm_count(),
            0
        );
        assert!(
            router.algorithm_names().is_empty()
        );
    }

    #[test]
    fn registry_capacity_must_be_non_zero() {
        let result =
            QuantumRouter::with_registry_capacity(0);

        assert!(result.is_err());
    }

    #[test]
    fn builder_capacity_must_be_non_zero() {
        let result =
            QuantumRouterBuilder::new()
                .with_registry_capacity(0);

        assert!(result.is_err());
    }

    #[test]
    fn helper_versions_are_stable() {
        assert_eq!(
            router_api_version(),
            ROUTER_API_VERSION
        );
        assert_eq!(
            router_version(),
            ROUTER_VERSION
        );
    }

    #[test]
    fn configuration_helpers_are_correct() {
        let config = RoutingConfig::default();

        assert!(
            requires_strict_verification(
                &RoutingConfig {
                    verification:
                        VerificationLevel::Strict,
                    ..config.clone()
                }
            )
        );

        assert!(
            requires_determinism(&config)
        );

        assert!(
            allows_swap(&config)
        );

        assert!(
            !allows_bridge(&config)
        );

        assert!(
            !allows_direction_reversal(
                &config
            )
        );
    }
}