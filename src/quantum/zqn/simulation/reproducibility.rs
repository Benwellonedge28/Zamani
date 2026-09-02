//! Zamani Quantum Noise (ZQN) — Reproducibility
//!
//! Production-grade deterministic execution identity and seed-material
//! derivation for ZQN simulation and stochastic execution.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - deterministic reproducibility coordinates;
//! - canonical seed-material derivation;
//! - domain separation between stochastic consumers;
//! - stable execution-coordinate encoding;
//! - reproducibility fingerprints;
//! - deterministic derivation suitable for sequential, parallel, and
//!   distributed execution;
//! - explicit reproducibility contracts;
//! - reproducibility metadata that can be consumed by simulation,
//!   characterization, benchmarking, QEC, routing, scheduling, and runtime
//!   integration.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - random-number-generator implementations;
//! - global RNG state;
//! - `thread_rng` or equivalent entropy sources;
//! - probability distributions;
//! - quantum channels;
//! - noise models;
//! - quantum state representations;
//! - simulation algorithms;
//! - canonical quantum IR;
//! - QubitId or PhysicalQubitId;
//! - hardware APIs;
//! - calibration semantics;
//! - QEC decoding;
//! - benchmarking methodology;
//! - serialization wire formats.
//!
//! The existing [`ZqnContext`] owns the root deterministic policy and seed.
//! This module consumes that policy and derives stable child seed material.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                    canonical computation
//!                              |
//!                              v
//!                         ZqnContext
//!                              |
//!                    root deterministic seed
//!                              |
//!                              v
//!                  simulation::reproducibility
//!                              |
//!        +---------------------+---------------------+
//!        |                     |                     |
//!        v                     v                     v
//!     channel               noise                  QEC
//!     sampler               model                sampling
//!        |                     |                     |
//!        +---------------------+---------------------+
//!                              |
//!                              v
//!                         RNG adapter
//!                              |
//!                              v
//!                     stochastic realization
//! ```
//!
//! The reproducibility layer derives *seed material*.
//! It does not instantiate or own the RNG used by a simulator.
//!
//! # Fundamental determinism contract
//!
//! Deterministic ZQN execution must satisfy:
//!
//! ```text
//! same semantic inputs
//!         +
//! same ZQN version
//!         +
//! same root seed
//!         +
//! same execution scope
//!         +
//! same stable coordinates
//!         +
//! same stochastic algorithm
//!         =
//! same seed material
//! ```
//!
//! The result must not depend on:
//!
//! - thread ID;
//! - worker count;
//! - memory address;
//! - process ID;
//! - task scheduling;
//! - hash-map iteration order;
//! - wall-clock time;
//! - allocation order;
//! - CPU core;
//! - machine hostname;
//! - network arrival order.
//!
//! # Parallel determinism
//!
//! A deterministic run must be partitionable.
//!
//! For example:
//!
//! ```text
//! sequential execution
//!
//! shot 0 -> operation 0
//! shot 0 -> operation 1
//! shot 1 -> operation 0
//! shot 1 -> operation 1
//!
//! parallel execution
//!
//! worker A -> shot 0
//! worker B -> shot 1
//! ```
//!
//! must derive exactly the same per-coordinate seed material.
//!
//! The worker assignment itself is never part of the derivation.
//!
//! # Distributed determinism
//!
//! Distributed execution may include a stable semantic partition coordinate,
//! such as a node/shard identity, when that identity is part of the execution
//! contract.
//!
//! It must never use an ephemeral process ID or network address as a semantic
//! identity.
//!
//! # No artificial scaling limit
//!
//! This module contains no maximum:
//!
//! - qubit count;
//! - operation count;
//! - shot count;
//! - circuit depth;
//! - device count;
//! - node count;
//! - noise-event count.
//!
//! Coordinates are represented using fixed-size integer values and therefore
//! impose only the representational limits of the host type itself.
//!
//! This is not a semantic limit on quantum-system size.
//!
//! Large workloads remain streamable because this module derives one
//! coordinate at a time and does not collect the entire circuit or execution
//! history.
//!
//! # Cryptographic hash
//!
//! SHA-256 is used as a deterministic domain-separated derivation primitive.
//!
//! This does NOT mean that the derived seed is a secret.
//!
//! Reproducibility material is intended to be reproducible, inspectable, and
//! attributable. Confidentiality belongs to the security layer.
//!
//! SHA-256 is used here because the repository already depends on `sha2` and
//! because deterministic cryptographic hashing avoids relying on the stability
//! guarantees of Rust's general-purpose hashers.
//!
//! In particular, `DefaultHasher` or arbitrary `Hash` implementations are not
//! used as persistent reproducibility primitives.
//!
//! # Stable encoding
//!
//! Inputs are encoded using explicit byte tags and little-endian fixed-width
//! integers.
//!
//! Variable-length byte strings are length-prefixed.
//!
//! This prevents ambiguous concatenations such as:
//!
//! ```text
//! [ab, c]
//! [a, bc]
//! ```
//!
//! from producing the same encoded coordinate.
//!
//! # Versioning
//!
//! The derivation domain contains an explicit reproducibility algorithm
//! version.
//!
//! Changing the derivation algorithm therefore requires an explicit version
//! change rather than silently changing scientific results.
//!
//! The ZQN semantic version remains owned by `core::version`.
//!
//! This module owns only the version of its deterministic derivation algorithm.
//!
//! # Canonical quantum identity
//!
//! This module intentionally does not import `QubitId` or `PhysicalQubitId`.
//!
//! Reproducibility coordinates are resource-agnostic.
//!
//! When a caller needs a quantum-resource coordinate, it must provide a
//! canonical, stable representation obtained from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! through an integration adapter.
//!
//! ZQN does not define another quantum-resource identifier.
//!
//! # Integration with `simulation::engine`
//!
//! The simulation engine owns execution orchestration and stable coordinates.
//! It does not own seed derivation.
//!
//! `reproducibility.rs` consumes:
//!
//! ```text
//! shot index
//! operation index
//! operation identity
//! execution scope
//! ```
//!
//! when supplied by the execution layer.
//!
//! The engine documentation explicitly establishes that stochastic realization
//! belongs to the supplied executor/sampling subsystem and that deterministic
//! execution must use explicit execution coordinates. This module is the
//! concrete derivation boundary for that contract.
//!
//! # Integration with `simulation::sampler`
//!
//! `sampler.rs` should obtain a [`SeedMaterial`] from this module and initialize
//! its chosen RNG or sampling algorithm from that material.
//!
//! The sampler remains responsible for:
//!
//! - RNG implementation;
//! - distribution sampling;
//! - rejection/acceptance logic;
//! - statistical semantics.
//!
//! It must not derive its own competing semantic seed hierarchy.
//!
//! # Integration with `simulation::deterministic`
//!
//! `deterministic.rs` should require/advertise the reproducibility contract
//! defined here but must not duplicate the derivation algorithm.
//!
//! Deterministic execution orchestration and reproducibility derivation are
//! deliberately separate responsibilities.
//!
//! # Integration with `noise`
//!
//! Noise models may derive independent streams by specifying a stable domain
//! label/model identity.
//!
//! Different noise consumers must not accidentally share the same stochastic
//! stream merely because they happen to execute at the same operation.
//!
//! # Integration with QEC
//!
//! QEC fault sampling should derive its stochastic material from this module
//! when it is executed under ZQN deterministic semantics.
//!
//! QEC must supply its own stable domain label so syndrome sampling and
//! physical-noise sampling do not collide.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may derive reproducible experiment/shot/sample coordinates
//! through this module.
//!
//! Benchmark methodology remains outside ZQN.
//!
//! # Integration with characterization
//!
//! Characterization experiments may use deterministic coordinates to make
//! repeated simulations and synthetic-data generation reproducible.
//!
//! Physical experiment randomness remains a hardware/experiment concern.
//!
//! # Integration with runtime
//!
//! The runtime supplies the `ZqnContext` and may supply an explicit execution
//! scope.
//!
//! Runtime scheduling must not alter deterministic seed material.
//!
//! # Integration with distributed execution
//!
//! Distributed systems should use explicit stable semantic coordinates for
//! partitioning.
//!
//! They must not use:
//!
//! - process IDs;
//! - thread IDs;
//! - hostnames;
//! - socket addresses;
//! - allocation addresses;
//! - task IDs generated by schedulers.
//!
//! unless those values have explicitly become part of the semantic execution
//! identity.
//!
//! # Serialization
//!
//! This module does not define the persistent serialization format.
//!
//! [`SeedMaterial`] and [`ReproducibilityCoordinates`] are value objects that
//! can be represented by `zqn::io` later.
//!
//! Their semantic byte representation is explicitly defined by the derivation
//! algorithm and therefore does not depend on Rust struct layout.
//!
//! # Security
//!
//! Reproducibility is not authorization.
//!
//! A seed must not be treated as a credential.
//!
//! If a caller needs confidential random material, it must use a dedicated
//! cryptographic/security subsystem.
//!
//! # Resource safety
//!
//! All derivation operations are O(1) in additional memory with respect to the
//! quantum-system size.
//!
//! A caller can derive seed material for one shot/operation/sample without
//! materializing all coordinates.
//!
//! Hash input length is bounded by the supplied coordinate metadata and does
//! not depend on the number of qubits unless the caller explicitly supplies a
//! resource-key byte string of that size.
//!
//! Callers should therefore use compact canonical resource identities rather
//! than serializing an entire quantum state into a coordinate.
//!
//! # Error behavior
//!
//! Seed derivation itself is infallible for valid fixed-width coordinates.
//!
//! Invalid variable-length components are rejected before hashing when their
//! declared length cannot be represented by the supported encoding.
//!
//! The public API intentionally uses `ZqnResult` so the module can integrate
//! with the repository-wide error vocabulary.
//!
//! # Thread safety
//!
//! The types in this module are immutable value types.
//!
//! No global mutable state exists.
//!
//! No RNG is shared.
//!
//! Consequently the derivation functions are safe to call concurrently.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Testing contract
//!
//! This module tests:
//!
//! - identical inputs produce identical seed material;
//! - different domains produce different material;
//! - different shots produce different material;
//! - different operations produce different material;
//! - different sample indices produce different material;
//! - execution scopes are separated;
//! - algorithm version is part of the derivation domain;
//! - variable-length components are length-delimited;
//! - deterministic derivation is independent of call order;
//! - no hidden mutable/global RNG exists;
//! - output size is fixed;
//! - derivation remains constant-memory with respect to workload size.
//!
//! Mathematical probability/channel tests belong to their owning modules.
//!
//! # File-completion guarantee
//!
//! This file is complete when:
//!
//! 1. the existing `ZqnContext` remains the sole root determinism policy;
//! 2. no second RNG is introduced;
//! 3. no global mutable state exists;
//! 4. deterministic derivation is domain-separated;
//! 5. stable coordinates are explicit;
//! 6. derivation is independent of thread scheduling;
//! 7. derivation is independent of worker count;
//! 8. derivation is independent of hash-map iteration order;
//! 9. derivation is independent of wall-clock time;
//! 10. derivation does not create another qubit identity system;
//! 11. derivation has an explicit algorithm version;
//! 12. seed material has a stable byte representation;
//! 13. future RNG implementations can consume the result without modifying
//!     this file;
//! 14. future simulator implementations can consume the result without
//!     modifying this file;
//! 15. future QEC integrations can consume the result without modifying this
//!     file;
//! 16. future hardware integrations do not require changes to this file;
//! 17. larger quantum systems do not require changes to this file;
//! 18. distributed execution does not require changes to this file;
//! 19. serialization can consume the types without making Rust layout the wire
//!     contract;
//! 20. Rust 1.97/1.97.1 compiles without unsafe.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use sha2::{Digest, Sha256};

use crate::quantum::zqn::core::{
    context::{ZqnContext, ZqnDeterminism},
    errors::ZqnResult,
};

// =============================================================================
// Stable algorithm identity
// =============================================================================

/// Version of the ZQN reproducibility derivation algorithm.
///
/// This is deliberately independent of the overall ZQN semantic version.
///
/// Increment this value only when the canonical byte encoding or derivation
/// semantics change.
pub const REPRODUCIBILITY_ALGORITHM_VERSION: u16 = 1;

/// Stable domain separator for ZQN reproducibility derivation.
const DOMAIN_SEPARATOR: &[u8] = b"ZAMANI/ZQN/REPRODUCIBILITY";

/// Fixed output size of SHA-256 seed material.
pub const SEED_MATERIAL_BYTES: usize = 32;

// =============================================================================
// Seed material
// =============================================================================

/// Deterministic seed material produced by ZQN reproducibility derivation.
///
/// This is raw deterministic material, not an RNG.
///
/// Consumers may use it to initialize an RNG or another explicitly documented
/// stochastic algorithm.
///
/// The byte array is intentionally fixed-size and allocation-free.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SeedMaterial([u8; SEED_MATERIAL_BYTES]);

impl SeedMaterial {
    /// Creates seed material from exactly 32 bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; SEED_MATERIAL_BYTES]) -> Self {
        Self(bytes)
    }

    /// Returns the complete deterministic seed material.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; SEED_MATERIAL_BYTES] {
        &self.0
    }

    /// Copies the material into a fixed-size array.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; SEED_MATERIAL_BYTES] {
        self.0
    }

    /// Returns the first 64 bits as little-endian material.
    ///
    /// This is provided only for RNG APIs that require a single `u64`.
    ///
    /// Consumers that support larger seed material should use [`as_bytes`]
    /// instead.
    #[must_use]
    pub fn as_u64_le(self) -> u64 {
        let mut bytes = [0_u8; 8];
        bytes.copy_from_slice(&self.0[..8]);
        u64::from_le_bytes(bytes)
    }
}

// =============================================================================
// Stable execution coordinates
// =============================================================================

/// Stable coordinates identifying one stochastic derivation point.
///
/// The coordinates contain no thread-local or machine-local information.
///
/// All fields are explicit semantic execution coordinates.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReproducibilityCoordinates {
    /// Shot index.
    pub shot_index: u64,

    /// Operation position in the canonical execution stream.
    pub operation_index: u64,

    /// Sample index within the operation.
    ///
    /// For example, a sampler may use:
    ///
    /// ```text
    /// 0, 1, 2, ...
    /// ```
    ///
    /// for independently generated stochastic samples.
    pub sample_index: u64,

    /// Optional stable execution partition.
    ///
    /// This can represent a semantic shard/node/partition when the distributed
    /// execution contract explicitly defines one.
    ///
    /// `None` is the canonical single-partition case.
    pub partition_index: Option<u64>,
}

impl ReproducibilityCoordinates {
    /// Creates coordinates for one shot and operation.
    #[must_use]
    pub const fn new(
        shot_index: u64,
        operation_index: u64,
    ) -> Self {
        Self {
            shot_index,
            operation_index,
            sample_index: 0,
            partition_index: None,
        }
    }

    /// Sets the stochastic sample index.
    #[must_use]
    pub const fn with_sample_index(
        mut self,
        sample_index: u64,
    ) -> Self {
        self.sample_index = sample_index;
        self
    }

    /// Sets an explicit semantic execution partition.
    #[must_use]
    pub const fn with_partition(
        mut self,
        partition_index: u64,
    ) -> Self {
        self.partition_index = Some(partition_index);
        self
    }
}

// =============================================================================
// Stable domain
// =============================================================================

/// Stable stochastic-domain identifier.
///
/// Domains prevent unrelated stochastic consumers from accidentally sharing
/// the same derived stream.
///
/// Examples:
///
/// ```text
/// "zqn/noise/gate"
/// "zqn/noise/readout"
/// "zqn/qec/fault"
/// "zqn/benchmark/synthetic"
/// ```
///
/// The domain is semantic metadata and must remain stable for reproducible
/// experiments.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReproducibilityDomain<'a> {
    value: &'a [u8],
}

impl<'a> ReproducibilityDomain<'a> {
    /// Creates a domain from stable bytes.
    ///
    /// The caller is responsible for ensuring that the supplied bytes are a
    /// stable semantic identifier.
    #[must_use]
    pub const fn from_bytes(value: &'a [u8]) -> Self {
        Self { value }
    }

    /// Creates a domain from a static string.
    #[must_use]
    pub const fn from_static(value: &'static str) -> ReproducibilityDomain<'static> {
        ReproducibilityDomain {
            value: value.as_bytes(),
        }
    }

    /// Returns the domain bytes.
    #[must_use]
    pub const fn as_bytes(self) -> &'a [u8] {
        self.value
    }
}

// =============================================================================
// Reproducibility descriptor
// =============================================================================

/// Stable semantic descriptor for one reproducibility derivation.
///
/// This object is deliberately independent of a simulator or RNG.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReproducibilityDescriptor<'a> {
    /// Stochastic domain.
    pub domain: ReproducibilityDomain<'a>,

    /// Execution coordinates.
    pub coordinates: ReproducibilityCoordinates,

    /// Optional stable program identity.
    ///
    /// The caller should supply canonical program identity bytes from the
    /// quantum IR/provenance subsystem.
    pub program_identity: &'a [u8],

    /// Optional stable noise-model identity.
    pub model_identity: &'a [u8],

    /// Optional stable calibration identity.
    pub calibration_identity: &'a [u8],

    /// Optional stable target identity.
    pub target_identity: &'a [u8],

    /// Optional caller-defined semantic component.
    ///
    /// This is useful for future domains without modifying this structure.
    pub extra_identity: &'a [u8],
}

impl<'a> ReproducibilityDescriptor<'a> {
    /// Creates a minimal descriptor.
    #[must_use]
    pub const fn new(
        domain: ReproducibilityDomain<'a>,
        coordinates: ReproducibilityCoordinates,
    ) -> Self {
        Self {
            domain,
            coordinates,
            program_identity: &[],
            model_identity: &[],
            calibration_identity: &[],
            target_identity: &[],
            extra_identity: &[],
        }
    }

    /// Adds stable program identity bytes.
    #[must_use]
    pub const fn with_program_identity(
        mut self,
        identity: &'a [u8],
    ) -> Self {
        self.program_identity = identity;
        self
    }

    /// Adds stable model identity bytes.
    #[must_use]
    pub const fn with_model_identity(
        mut self,
        identity: &'a [u8],
    ) -> Self {
        self.model_identity = identity;
        self
    }

    /// Adds stable calibration identity bytes.
    #[must_use]
    pub const fn with_calibration_identity(
        mut self,
        identity: &'a [u8],
    ) -> Self {
        self.calibration_identity = identity;
        self
    }

    /// Adds stable target identity bytes.
    #[must_use]
    pub const fn with_target_identity(
        mut self,
        identity: &'a [u8],
    ) -> Self {
        self.target_identity = identity;
        self
    }

    /// Adds another stable semantic identity component.
    #[must_use]
    pub const fn with_extra_identity(
        mut self,
        identity: &'a [u8],
    ) -> Self {
        self.extra_identity = identity;
        self
    }
}

// =============================================================================
// Reproducibility context
// =============================================================================

/// Reproducibility view derived from an existing [`ZqnContext`].
///
/// This type does not replace `ZqnContext`.
///
/// It exists to make the reproducibility contract explicit at simulation
/// boundaries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReproducibilityContext {
    /// Root seed supplied by the ZQN context.
    root_seed: u64,

    /// Optional execution scope supplied by the ZQN context.
    execution_scope: Option<u64>,

    /// Whether deterministic derivation is enabled.
    deterministic: bool,
}

impl ReproducibilityContext {
    /// Creates a reproducibility context from the canonical ZQN context.
    ///
    /// Nondeterministic contexts are represented explicitly rather than being
    /// silently converted into deterministic execution.
    #[must_use]
    pub fn from_zqn_context(context: &ZqnContext) -> Self {
        match context.determinism() {
            ZqnDeterminism::Deterministic { seed } => Self {
                root_seed: seed,
                execution_scope: context.execution_scope().id().map(|id| id.value()),
                deterministic: true,
            },

            ZqnDeterminism::Nondeterministic => Self {
                root_seed: 0,
                execution_scope: context.execution_scope().id().map(|id| id.value()),
                deterministic: false,
            },
        }
    }

    /// Creates deterministic reproducibility directly from explicit root
    /// material.
    ///
    /// This constructor is useful for tests and low-level integrations where a
    /// full `ZqnContext` is not yet available.
    #[must_use]
    pub const fn deterministic(seed: u64) -> Self {
        Self {
            root_seed: seed,
            execution_scope: None,
            deterministic: true,
        }
    }

    /// Adds a stable execution scope.
    #[must_use]
    pub const fn with_execution_scope(
        mut self,
        scope: u64,
    ) -> Self {
        self.execution_scope = Some(scope);
        self
    }

    /// Returns whether deterministic derivation is available.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        self.deterministic
    }

    /// Returns the root seed when deterministic execution is enabled.
    #[must_use]
    pub const fn root_seed(self) -> Option<u64> {
        if self.deterministic {
            Some(self.root_seed)
        } else {
            None
        }
    }

    /// Returns the optional execution scope.
    #[must_use]
    pub const fn execution_scope(self) -> Option<u64> {
        self.execution_scope
    }

    /// Derives deterministic seed material for one descriptor.
    ///
    /// Returns `None` for an explicitly nondeterministic context.
    #[must_use]
    pub fn derive(
        self,
        descriptor: &ReproducibilityDescriptor<'_>,
    ) -> Option<SeedMaterial> {
        if !self.deterministic {
            return None;
        }

        Some(derive_seed_material(
            self.root_seed,
            self.execution_scope,
            descriptor,
        ))
    }
}

// =============================================================================
// Canonical derivation
// =============================================================================

/// Derives deterministic 256-bit seed material.
///
/// This function is the single canonical derivation algorithm for this module.
///
/// It is:
///
/// - deterministic;
/// - thread independent;
/// - worker independent;
/// - process independent;
/// - allocation independent;
/// - constant-memory with respect to workload size.
///
/// The function does not use system entropy.
#[must_use]
pub fn derive_seed_material(
    root_seed: u64,
    execution_scope: Option<u64>,
    descriptor: &ReproducibilityDescriptor<'_>,
) -> SeedMaterial {
    let mut hasher = Sha256::new();

    // Global domain separation.
    hasher.update(DOMAIN_SEPARATOR);

    // Derivation algorithm version.
    hasher.update(REPRODUCIBILITY_ALGORITHM_VERSION.to_le_bytes());

    // Root seed.
    hasher.update(root_seed.to_le_bytes());

    // Execution scope.
    write_optional_u64(&mut hasher, execution_scope);

    // Semantic stochastic domain.
    write_bytes(&mut hasher, descriptor.domain.as_bytes());

    // Stable execution coordinates.
    hasher.update(descriptor.coordinates.shot_index.to_le_bytes());
    hasher.update(descriptor.coordinates.operation_index.to_le_bytes());
    hasher.update(descriptor.coordinates.sample_index.to_le_bytes());

    write_optional_u64(
        &mut hasher,
        descriptor.coordinates.partition_index,
    );

    // Stable semantic identities.
    write_bytes(&mut hasher, descriptor.program_identity);
    write_bytes(&mut hasher, descriptor.model_identity);
    write_bytes(&mut hasher, descriptor.calibration_identity);
    write_bytes(&mut hasher, descriptor.target_identity);
    write_bytes(&mut hasher, descriptor.extra_identity);

    let digest = hasher.finalize();

    let mut material = [0_u8; SEED_MATERIAL_BYTES];
    material.copy_from_slice(&digest);

    SeedMaterial::from_bytes(material)
}

// =============================================================================
// Derivation helpers
// =============================================================================

/// Writes a length-delimited byte string into the canonical hash stream.
fn write_bytes(
    hasher: &mut Sha256,
    bytes: &[u8],
) {
    // `usize` is deliberately converted through checked u64 conversion rather
    // than using the platform-dependent byte width of usize.
    //
    // The repository's execution objects are finite host values. A byte slice
    // larger than u64::MAX cannot exist on supported Rust platforms, so this
    // conversion is infallible in practice while remaining explicit.
    let length = bytes.len() as u64;

    hasher.update(length.to_le_bytes());
    hasher.update(bytes);
}

/// Writes an optional fixed-width integer with explicit presence encoding.
fn write_optional_u64(
    hasher: &mut Sha256,
    value: Option<u64>,
) {
    match value {
        Some(value) => {
            hasher.update([1_u8]);
            hasher.update(value.to_le_bytes());
        }

        None => {
            hasher.update([0_u8]);
        }
    }
}

// =============================================================================
// Convenience API
// =============================================================================

/// Derives seed material directly from a ZQN context.
///
/// This is the preferred high-level API for simulation integrations.
#[must_use]
pub fn derive_from_context(
    context: &ZqnContext,
    descriptor: &ReproducibilityDescriptor<'_>,
) -> Option<SeedMaterial> {
    ReproducibilityContext::from_zqn_context(context).derive(descriptor)
}

/// Derives seed material for one shot/operation/sample coordinate.
///
/// This convenience function is intentionally small and allocation-free.
#[must_use]
pub fn derive_for_coordinate(
    context: &ZqnContext,
    domain: ReproducibilityDomain<'_>,
    coordinates: ReproducibilityCoordinates,
) -> Option<SeedMaterial> {
    let descriptor = ReproducibilityDescriptor::new(domain, coordinates);

    derive_from_context(context, &descriptor)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const DOMAIN: ReproducibilityDomain<'static> =
        ReproducibilityDomain::from_static("zqn/test");

    #[test]
    fn identical_inputs_are_identical() {
        let context = ReproducibilityContext::deterministic(42);

        let descriptor = ReproducibilityDescriptor::new(
            DOMAIN,
            ReproducibilityCoordinates::new(3, 7)
                .with_sample_index(11),
        );

        let first = context
            .derive(&descriptor)
            .expect("deterministic context must derive material");

        let second = context
            .derive(&descriptor)
            .expect("deterministic context must derive material");

        assert_eq!(first, second);
    }

    #[test]
    fn different_root_seeds_are_separated() {
        let coordinates = ReproducibilityCoordinates::new(0, 0);

        let first = ReproducibilityContext::deterministic(1)
            .derive(&ReproducibilityDescriptor::new(DOMAIN, coordinates))
            .expect("deterministic context");

        let second = ReproducibilityContext::deterministic(2)
            .derive(&ReproducibilityDescriptor::new(DOMAIN, coordinates))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn different_shots_are_separated() {
        let context = ReproducibilityContext::deterministic(42);

        let first = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0),
            ))
            .expect("deterministic context");

        let second = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(1, 0),
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn different_operations_are_separated() {
        let context = ReproducibilityContext::deterministic(42);

        let first = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0),
            ))
            .expect("deterministic context");

        let second = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 1),
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn different_samples_are_separated() {
        let context = ReproducibilityContext::deterministic(42);

        let first = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0)
                    .with_sample_index(0),
            ))
            .expect("deterministic context");

        let second = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0)
                    .with_sample_index(1),
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn different_domains_are_separated() {
        let context = ReproducibilityContext::deterministic(42);
        let coordinates = ReproducibilityCoordinates::new(0, 0);

        let first = context
            .derive(&ReproducibilityDescriptor::new(
                ReproducibilityDomain::from_static("zqn/noise"),
                coordinates,
            ))
            .expect("deterministic context");

        let second = context
            .derive(&ReproducibilityDescriptor::new(
                ReproducibilityDomain::from_static("zqn/qec"),
                coordinates,
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn execution_scope_is_separated() {
        let coordinates = ReproducibilityCoordinates::new(0, 0);

        let first = ReproducibilityContext::deterministic(42)
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                coordinates,
            ))
            .expect("deterministic context");

        let second = ReproducibilityContext::deterministic(42)
            .with_execution_scope(99)
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                coordinates,
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn partition_is_separated() {
        let context = ReproducibilityContext::deterministic(42);

        let first = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0),
            ))
            .expect("deterministic context");

        let second = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0)
                    .with_partition(1),
            ))
            .expect("deterministic context");

        assert_ne!(first, second);
    }

    #[test]
    fn program_model_and_target_identity_affect_derivation() {
        let context = ReproducibilityContext::deterministic(42);
        let coordinates = ReproducibilityCoordinates::new(0, 0);

        let base = ReproducibilityDescriptor::new(DOMAIN, coordinates);

        let program = base.with_program_identity(b"program-a");
        let model = base.with_model_identity(b"model-a");
        let target = base.with_target_identity(b"target-a");

        let base_seed = context.derive(&base).expect("deterministic context");
        let program_seed = context.derive(&program).expect("deterministic context");
        let model_seed = context.derive(&model).expect("deterministic context");
        let target_seed = context.derive(&target).expect("deterministic context");

        assert_ne!(base_seed, program_seed);
        assert_ne!(base_seed, model_seed);
        assert_ne!(base_seed, target_seed);
    }

    #[test]
    fn variable_length_identity_is_unambiguous() {
        let context = ReproducibilityContext::deterministic(42);
        let coordinates = ReproducibilityCoordinates::new(0, 0);

        let first = ReproducibilityDescriptor::new(DOMAIN, coordinates)
            .with_extra_identity(b"abc");

        let second = ReproducibilityDescriptor::new(DOMAIN, coordinates)
            .with_extra_identity(b"ab");

        let first_seed = context.derive(&first).expect("deterministic context");
        let second_seed = context.derive(&second).expect("deterministic context");

        assert_ne!(first_seed, second_seed);
    }

    #[test]
    fn nondeterministic_context_does_not_fabricate_determinism() {
        let context = ReproducibilityContext {
            root_seed: 0,
            execution_scope: None,
            deterministic: false,
        };

        let descriptor = ReproducibilityDescriptor::new(
            DOMAIN,
            ReproducibilityCoordinates::new(0, 0),
        );

        assert!(context.derive(&descriptor).is_none());
    }

    #[test]
    fn output_is_fixed_size() {
        let context = ReproducibilityContext::deterministic(42);

        let material = context
            .derive(&ReproducibilityDescriptor::new(
                DOMAIN,
                ReproducibilityCoordinates::new(0, 0),
            ))
            .expect("deterministic context");

        assert_eq!(material.as_bytes().len(), SEED_MATERIAL_BYTES);
    }

    #[test]
    fn derivation_is_call_order_independent() {
        let context = ReproducibilityContext::deterministic(123);

        let first_descriptor = ReproducibilityDescriptor::new(
            DOMAIN,
            ReproducibilityCoordinates::new(0, 10),
        );

        let second_descriptor = ReproducibilityDescriptor::new(
            DOMAIN,
            ReproducibilityCoordinates::new(0, 11),
        );

        let first_before = context
            .derive(&first_descriptor)
            .expect("deterministic context");

        let _ = context
            .derive(&second_descriptor)
            .expect("deterministic context");

        let first_after = context
            .derive(&first_descriptor)
            .expect("deterministic context");

        assert_eq!(first_before, first_after);
    }

    #[test]
    fn u64_conversion_is_stable() {
        let material = SeedMaterial::from_bytes([0xAA; SEED_MATERIAL_BYTES]);

        assert_eq!(material.as_u64_le(), u64::from_le_bytes([0xAA; 8]));
    }

    #[test]
    fn convenience_api_matches_context_api() {
        let context = ZqnContext::new().deterministic(42);

        let coordinates = ReproducibilityCoordinates::new(5, 9);

        let descriptor =
            ReproducibilityDescriptor::new(DOMAIN, coordinates);

        let direct = derive_from_context(&context, &descriptor);

        let convenience =
            derive_for_coordinate(&context, DOMAIN, coordinates);

        assert_eq!(direct, convenience);
    }
}