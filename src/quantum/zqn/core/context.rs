//! Zamani Quantum Noise (ZQN) — Execution Context
//!
//! # Ownership
//!
//! This module owns the immutable, runtime-independent execution context used
//! by ZQN operations.
//!
//! `ZqnContext` answers:
//!
//! > "Under which explicit semantic, resource-policy, determinism, and
//! > execution-scope conditions is this ZQN operation being evaluated?"
//!
//! The context provides the stable foundation consumed by:
//!
//! - probability;
//! - channel;
//! - fault;
//! - noise;
//! - operations;
//! - calibration;
//! - characterization;
//! - simulation;
//! - propagation;
//! - target integration;
//! - IR integration;
//! - routing integration;
//! - scheduling integration;
//! - QEC integration;
//! - hardware integration;
//! - benchmarking;
//! - runtime integration;
//! - I/O validation.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - quantum IR semantics;
//! - logical-qubit semantics;
//! - physical-qubit semantics;
//! - hardware capability definitions;
//! - hardware discovery;
//! - QPU credentials;
//! - routing;
//! - scheduling;
//! - simulation algorithms;
//! - quantum-channel mathematics;
//! - probability mathematics;
//! - fault semantics;
//! - calibration data;
//! - characterization protocols;
//! - benchmarking methodology;
//! - serialization transport;
//! - global runtime state;
//! - random-number generation.
//!
//! Those responsibilities remain with their respective subsystems.
//!
//! # Architectural position
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! ZQN core
//!      │
//!      ├── context.rs
//!      ├── limits.rs
//!      ├── ids.rs
//!      ├── errors.rs
//!      └── version.rs
//!              │
//!              ▼
//!       ZQN domain modules
//!              │
//!       ┌──────┼────────┐
//!       ▼      ▼        ▼
//!    channel  noise    fault
//!       │      │        │
//!       └──────┼────────┘
//!              ▼
//!        integrations
//! ```
//!
//! `context.rs` is deliberately below the domain-specific ZQN modules.
//!
//! Domain modules consume the context; the context must not depend on them.
//!
//! # Why context exists
//!
//! A production quantum-noise system cannot safely pass unrelated execution
//! parameters through ad-hoc argument lists.
//!
//! Without a common context, different subsystems eventually invent competing
//! versions of:
//!
//! - resource limits;
//! - random seeds;
//! - reproducibility rules;
//! - execution identity;
//! - cancellation;
//! - numerical policy;
//! - validation mode;
//! - temporal scope.
//!
//! That would make identical Zamani programs behave differently depending on
//! which subsystem happened to execute them.
//!
//! `ZqnContext` therefore provides one stable execution boundary.
//!
//! # Write once, scale everywhere
//!
//! The context contains NO semantic maximum for:
//!
//! - qubits;
//! - qudits;
//! - modes;
//! - operations;
//! - circuit depth;
//! - faults;
//! - channels;
//! - shots;
//! - experiments;
//! - calibration entries;
//! - tensor dimensions;
//! - matrix dimensions;
//! - devices;
//! - execution nodes;
//! - execution links.
//!
//! Resource ceilings are delegated to [`ZqnLimits`].
//!
//! `ZqnLimits` explicitly distinguishes:
//!
//! ```text
//! None       = no ZQN-imposed ceiling
//! Some(n>0)  = explicit policy ceiling
//! ```
//!
//! The absence of a ZQN ceiling never claims that hardware, RAM, storage,
//! operating-system resources, network resources, or other infrastructure are
//! infinite.
//!
//! It means only that this context does not impose an additional semantic
//! ceiling.
//!
//! This is the foundation for Zamani's:
//!
//! > one program → many machine sizes
//!
//! model.
//!
//! # Canonical quantum-resource identity
//!
//! This module intentionally does not define `QubitId` or `PhysicalQubitId`.
//!
//! The authoritative identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A context may describe an execution scope, but it must not create a second
//! quantum-resource identity system.
//!
//! Higher-level consumers that need resource-specific scope should carry the
//! canonical IR types directly.
//!
//! For example:
//!
//! ```text
//! fn evaluate(
//!     context: &ZqnContext,
//!     qubit: crate::quantum::ir::qubit::QubitId,
//! ) -> ZqnResult<...>
//! ```
//!
//! rather than introducing a ZQN-specific qubit identifier.
//!
//! # Context versus configuration
//!
//! A configuration describes what a caller wants.
//!
//! A context describes the conditions under which an operation is evaluated.
//!
//! Therefore the context contains only information that is appropriate to
//! share across one coherent ZQN execution scope.
//!
//! Noise-model configuration, channel parameters, calibration values and
//! target-specific settings remain owned by their respective modules.
//!
//! # Context immutability
//!
//! `ZqnContext` is immutable after construction.
//!
//! This is intentional.
//!
//! An execution context is a semantic boundary, not a mutable global registry.
//!
//! If an execution needs a different:
//!
//! - limit policy;
//! - seed;
//! - validation mode;
//! - execution scope;
//! - time scope;
//!
//! it must receive a new context or an explicitly derived child context.
//!
//! Existing contexts must never be silently mutated while another operation is
//! using them.
//!
//! # Child contexts
//!
//! ZQN operations often need nested execution scopes:
//!
//! ```text
//! program
//!   ├── operation
//!   │     ├── channel evaluation
//!   │     └── fault sampling
//!   └── measurement
//!         └── readout model
//! ```
//!
//! Child contexts therefore inherit the parent's immutable semantic settings
//! while allowing an explicit child execution scope.
//!
//! Child creation must never silently weaken a parent's resource policy.
//!
//! The effective limit of a child context is the intersection of the parent
//! policy and the requested child policy.
//!
//! Conceptually:
//!
//! ```text
//! child_limit = minimum(parent_limit, requested_limit)
//! ```
//!
//! This prevents a nested subsystem from escaping an outer execution policy.
//!
//! # Resource policy
//!
//! `ZqnContext` owns one [`ZqnLimits`] value.
//!
//! It does not perform resource accounting.
//!
//! The distinction is mandatory:
//!
//! ```text
//! ZqnContext
//!     │
//!     └── declares/adopts execution policy
//!
//! Runtime resource manager
//!     │
//!     └── measures actual consumption
//!
//! Allocator / OS
//!     │
//!     └── provides actual host resources
//!
//! Hardware target
//!     │
//!     └── provides actual quantum resources
//! ```
//!
//! A context therefore cannot claim that a requested workload will actually
//! fit merely because it passes `ZqnLimits`.
//!
//! Admission policy and physical availability are separate concerns.
//!
//! # Determinism
//!
//! Determinism is a first-class part of the context.
//!
//! ZQN must never rely on a hidden global RNG.
//!
//! The context therefore carries an explicit deterministic execution policy.
//!
//! The policy can be:
//!
//! ```text
//! deterministic
//! nondeterministic
//! ```
//!
//! In deterministic mode, stochastic consumers must derive their random
//! streams from explicit caller-supplied seed material and stable execution
//! coordinates.
//!
//! This module itself does not generate random numbers.
//!
//! # Reproducibility
//!
//! Reproducibility requires more than a seed.
//!
//! A reproducible ZQN execution is conceptually identified by:
//!
//! ```text
//! seed
//! + execution scope
//! + operation scope
//! + sample index
//! ```
//!
//! The actual stochastic subsystem owns the derivation algorithm.
//!
//! `ZqnContext` merely carries the stable root information.
//!
//! This avoids coupling the core context to a particular RNG crate or RNG
//! implementation.
//!
//! # Parallel determinism
//!
//! The context contract explicitly supports:
//!
//! ```text
//! one thread
//! many threads
//! distributed execution
//! ```
//!
//! without making thread scheduling part of stochastic semantics.
//!
//! A deterministic ZQN consumer must derive stochastic work from stable
//! semantic coordinates rather than thread IDs, memory addresses, or execution
//! order.
//!
//! # Cancellation
//!
//! Long-running ZQN operations must be cancellable without introducing global
//! mutable state.
//!
//! The context therefore supports an optional shared cancellation signal.
//!
//! Cancellation is an execution concern, not a noise-model semantic.
//!
//! A cancelled operation must return an explicit failure rather than silently
//! producing partial scientific results as though execution completed.
//!
//! # Wall-clock time
//!
//! Wall-clock time is intentionally NOT captured automatically when a context
//! is created.
//!
//! Doing so would make otherwise identical contexts differ merely because they
//! were constructed at different times.
//!
//! If physical time matters to a noise model, calibration model, drift model,
//! or experiment, the owning subsystem must provide an explicit time value or
//! calibration snapshot.
//!
//! `ZqnContext` therefore remains deterministic unless the caller explicitly
//! chooses otherwise.
//!
//! # Validation mode
//!
//! Validation policy is explicit.
//!
//! ZQN distinguishes:
//!
//! ```text
//! Strict
//! Permissive
//! ```
//!
//! Strict mode must reject invalid, ambiguous, unsupported, or approximate
//! behavior where the relevant subsystem can determine that it violates the
//! requested contract.
//!
//! Permissive mode may allow subsystem-defined warnings or explicitly declared
//! approximations, but it must never silently convert an unsupported semantic
//! request into an unrelated computation.
//!
//! The actual approximation/error contract belongs to the subsystem performing
//! the approximation.
//!
//! # Numerical policy
//!
//! The context provides a small numerical-policy vocabulary without choosing a
//! particular matrix, tensor, simulator, or floating-point backend.
//!
//! It can express:
//!
//! - an explicit absolute tolerance;
//! - an explicit relative tolerance;
//! - whether finite numerical values are required.
//!
//! It does NOT define:
//!
//! - matrix libraries;
//! - tensor libraries;
//! - arbitrary-precision implementations;
//! - simulator algorithms;
//! - channel representations.
//!
//! Those remain separate.
//!
//! # Precision and scalability
//!
//! Tolerances are semantic policies, not machine-size limits.
//!
//! They do not impose a maximum system size.
//!
//! Large systems may require different numerical strategies, but the context
//! itself remains independent of the chosen representation.
//!
//! # Execution scope
//!
//! A context may carry an optional opaque ZQN object identity representing the
//! current execution scope.
//!
//! This is deliberately an object identity rather than a quantum-resource
//! identity.
//!
//! It may be used by:
//!
//! - provenance;
//! - deterministic sampling;
//! - telemetry correlation;
//! - distributed execution;
//! - result association.
//!
//! It does not grant authorization and does not prove resource existence.
//!
//! # Integration with `core::ids`
//!
//! The context uses [`ZqnObjectId`] for optional generic execution scope
//! identity.
//!
//! It does not introduce a new `ExecutionId` type here because identity
//! allocation belongs to the identity subsystem.
//!
//! If a future dedicated execution identity is needed, it should be introduced
//! in `core::ids` and integrated as a deliberate API/schema change rather than
//! hidden inside this module.
//!
//! # Integration with `core::limits`
//!
//! The context owns [`ZqnLimits`] and exposes:
//!
//! - the configured policy;
//! - limit intersection for child contexts;
//! - request checking;
//! - checked resource arithmetic through the underlying limits subsystem.
//!
//! It does not duplicate the resource-limit implementation.
//!
//! # Integration with `core::version`
//!
//! Every context records the ZQN version metadata against which it was created.
//!
//! This makes execution context self-describing and prevents downstream
//! diagnostics from having to infer the ZQN semantic contract.
//!
//! The authoritative version remains `core::version`.
//!
//! This file does not define another version constant.
//!
//! # Integration with `core::errors`
//!
//! Context operations that need to report ZQN-wide failures use the canonical
//! `ZqnError`/`ZqnResult` vocabulary.
//!
//! The context does not define a competing top-level ZQN error hierarchy.
//!
//! Limit-specific errors remain owned by `core::limits` where the API directly
//! exposes limit-policy validation.
//!
//! # Integration with future `core::capabilities`
//!
//! `ZqnContext` deliberately does not import a not-yet-defined
//! `ZqnCapabilities` type.
//!
//! This prevents the context foundation from becoming coupled to target
//! capability implementation details.
//!
//! Future capability validation consumes:
//!
//! ```text
//! &ZqnContext
//! ```
//!
//! rather than requiring `ZqnContext` to own a particular capability
//! representation.
//!
//! This makes `context.rs` complete independently.
//!
//! # Integration with future provenance
//!
//! Provenance may consume the context's:
//!
//! - version metadata;
//! - execution scope;
//! - deterministic policy;
//! - validation policy;
//! - numerical policy.
//!
//! Context does not depend on provenance.
//!
//! This preserves dependency direction.
//!
//! # Integration with future calibration
//!
//! Calibration consumers may derive a specialized child context for an
//! explicit calibration/execution scope, but calibration data itself does not
//! belong in this file.
//!
//! A calibration snapshot should remain an immutable domain object owned by
//! `calibration/snapshot.rs`.
//!
//! # Integration with future simulation
//!
//! Simulators consume:
//!
//! ```text
//! &ZqnContext
//! ```
//!
//! to obtain:
//!
//! - limits;
//! - determinism policy;
//! - cancellation;
//! - numerical tolerances;
//! - validation mode;
//! - execution scope.
//!
//! The simulator chooses its own state representation and numerical algorithm.
//!
//! # Integration with future hardware
//!
//! Hardware adapters may consume the context for policy and reproducibility.
//!
//! They must not treat the context as:
//!
//! - QPU credentials;
//! - a hardware handle;
//! - a device object;
//! - a capability registry.
//!
//! Those belong to the hardware/target layers.
//!
//! # Integration with future QEC
//!
//! QEC can derive a context for:
//!
//! - syndrome sampling;
//! - fault generation;
//! - logical-error analysis;
//! - decoder experiments.
//!
//! QEC-specific semantics remain outside this module.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling can use the context's limits and deterministic
//! execution policy while independently consuming ZQN noise information.
//!
//! The context does not know routing or scheduling algorithms.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may use the execution scope and deterministic policy to make
//! benchmark runs reproducible and attributable.
//!
//! Benchmark definitions remain outside this module.
//!
//! # Integration with I/O
//!
//! `ZqnContext` is intentionally a runtime object rather than a wire-format
//! object.
//!
//! Its semantic fields are inspectable and can be copied into an explicit
//! serialization schema by `zqn::io`.
//!
//! This avoids making the Rust struct layout an accidental persistence
//! contract.
//!
//! # Serialization boundary
//!
//! The context itself is not derived as `Serialize`/`Deserialize` because it
//! contains runtime-only cancellation state.
//!
//! Instead:
//!
//! ```text
//! ZqnContext
//!      │
//!      ├── semantic fields
//!      │
//!      ▼
//! ZQN I/O schema
//! ```
//!
//! The I/O subsystem owns the stable wire representation.
//!
//! This is intentional and prevents runtime handles from accidentally becoming
//! persistent data.
//!
//! # Thread safety
//!
//! Context semantic fields are immutable.
//!
//! Cancellation is represented by an atomic shared signal so multiple workers
//! can observe cancellation without mutable data races.
//!
//! The context contains no unsafe code and no mutable global state.
//!
//! # Security
//!
//! Context values are not authorization credentials.
//!
//! In particular:
//!
//! - `ZqnObjectId` is not an access token;
//! - the seed is not a secret by definition;
//! - limits are not security authorization;
//! - validation mode is not a privilege;
//! - execution scope is not authentication.
//!
//! Authentication and authorization belong to the surrounding runtime/security
//! architecture.
//!
//! # Resource safety
//!
//! Context construction is constant-size with respect to the quantum-system
//! size.
//!
//! The context does not allocate collections proportional to:
//!
//! - qubit count;
//! - operation count;
//! - shot count;
//! - channel dimension;
//! - correlation-domain size.
//!
//! This is essential for scalability.
//!
//! Large work products remain owned by the modules that actually need them.
//!
//! # No unsafe
//!
//! This file contains no unsafe Rust.
//!
//! `#![forbid(unsafe_code)]` makes accidental introduction of unsafe code a
//! compile-time error.
//!
//! # Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # File-completion guarantee
//!
//! This file is complete when:
//!
//! 1. `ZqnContext` is immutable after construction;
//! 2. `ZqnLimits` is the sole resource-policy owner;
//! 3. no machine-size ceiling is introduced;
//! 4. no ZQN-specific qubit identity is introduced;
//! 5. canonical IR qubit IDs remain available to higher-level consumers;
//! 6. version metadata comes from `core::version`;
//! 7. no hidden RNG exists;
//! 8. deterministic execution is explicitly represented;
//! 9. cancellation is explicit and thread-safe;
//! 10. wall-clock time is not implicitly captured;
//! 11. child contexts cannot weaken parent limits;
//! 12. validation policy is explicit;
//! 13. numerical tolerance policy is explicit;
//! 14. runtime-only state is not accidentally serialized;
//! 15. no global mutable state exists;
//! 16. no unsafe code exists;
//! 17. downstream modules can consume the context without modifying this file;
//! 18. larger quantum systems do not require changes to this file;
//! 19. new hardware technologies do not require changes to this file;
//! 20. new noise models do not require changes to this file;
//! 21. new channel representations do not require changes to this file;
//! 22. distributed execution does not require a different context type;
//! 23. deterministic parallel execution can use the same context semantics.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::quantum::zqn::core::{
    ids::ZqnObjectId,
    limits::{Limit, LimitError, LimitKind, ResourceCount, ZqnLimits},
    version::{ZqnVersionMetadata, ZQN_VERSION_METADATA},
};

// ============================================================================
// Canonical result/error integration
// ============================================================================

use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};

// ============================================================================
// Validation policy
// ============================================================================

/// Validation strictness used by a ZQN execution context.
///
/// Validation policy is intentionally independent from any specific noise
/// model or target.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ZqnValidationMode {
    /// Reject invalid, ambiguous, unsupported, or contract-breaking input
    /// whenever the relevant subsystem can establish that it is invalid.
    #[default]
    Strict,

    /// Permit explicitly supported warnings and approximations where the
    /// consuming subsystem provides a declared approximation/error contract.
    ///
    /// Permissive mode never means "ignore errors".
    Permissive,
}

impl ZqnValidationMode {
    /// Returns the stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Permissive => "permissive",
        }
    }

    /// Returns true when strict validation is required.
    #[must_use]
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::Strict)
    }
}

impl fmt::Display for ZqnValidationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Determinism policy
// ============================================================================

/// Determinism policy for stochastic ZQN execution.
///
/// This type does not generate random numbers.
///
/// It only states whether consumers are required to make stochastic behavior
/// reproducible.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ZqnDeterminism {
    /// Stochastic operations must derive their behavior deterministically from
    /// explicit execution inputs.
    Deterministic {
        /// Root seed material supplied by the caller.
        ///
        /// The value is deliberately opaque to this module. ZQN does not
        /// prescribe an RNG algorithm.
        seed: u64,
    },

    /// Stochastic operations may use nondeterministic entropy supplied by their
    /// execution environment.
    ///
    /// This mode must be selected explicitly.
    Nondeterministic,
}

impl Default for ZqnDeterminism {
    fn default() -> Self {
        Self::Deterministic { seed: 0 }
    }
}

impl ZqnDeterminism {
    /// Creates deterministic execution from explicit seed material.
    ///
    /// The seed is not itself an RNG and this function performs no randomness.
    #[must_use]
    pub const fn deterministic(seed: u64) -> Self {
        Self::Deterministic { seed }
    }

    /// Creates explicitly nondeterministic execution policy.
    #[must_use]
    pub const fn nondeterministic() -> Self {
        Self::Nondeterministic
    }

    /// Returns true when deterministic execution is required.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        matches!(self, Self::Deterministic { .. })
    }

    /// Returns the deterministic root seed when one is configured.
    #[must_use]
    pub const fn seed(self) -> Option<u64> {
        match self {
            Self::Deterministic { seed } => Some(seed),
            Self::Nondeterministic => None,
        }
    }

    /// Returns a stable machine-readable policy name.
    #[must_use]
    pub const fn mode_name(self) -> &'static str {
        match self {
            Self::Deterministic { .. } => "deterministic",
            Self::Nondeterministic => "nondeterministic",
        }
    }
}

impl fmt::Display for ZqnDeterminism {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deterministic { seed } => {
                write!(formatter, "deterministic(seed={seed})")
            }
            Self::Nondeterministic => formatter.write_str("nondeterministic"),
        }
    }
}

// ============================================================================
// Numerical policy
// ============================================================================

/// Numerical policy carried by a ZQN execution context.
///
/// Tolerances are explicit policies, not machine-size limits.
///
/// `None` means that this context does not impose an additional tolerance at
/// this layer. The consuming numerical subsystem must then use its own
/// explicitly documented default or reject an operation that requires a
/// tolerance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZqnNumericalPolicy {
    /// Optional absolute tolerance.
    pub absolute_tolerance: Option<f64>,

    /// Optional relative tolerance.
    pub relative_tolerance: Option<f64>,

    /// Whether finite numerical values are required.
    ///
    /// When true, NaN and infinite values must be rejected by consumers before
    /// they enter mathematical operations.
    pub require_finite: bool,
}

impl Default for ZqnNumericalPolicy {
    fn default() -> Self {
        Self {
            absolute_tolerance: None,
            relative_tolerance: None,
            require_finite: true,
        }
    }
}

impl ZqnNumericalPolicy {
    /// Creates the default strict numerical policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            absolute_tolerance: None,
            relative_tolerance: None,
            require_finite: true,
        }
    }

    /// Creates a policy with an explicit absolute tolerance.
    ///
    /// The value must be finite and non-negative.
    pub fn with_absolute_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, ZqnError> {
        validate_tolerance(tolerance, "absolute")?;
        self.absolute_tolerance = Some(tolerance);
        Ok(self)
    }

    /// Creates a policy with an explicit relative tolerance.
    ///
    /// The value must be finite and non-negative.
    pub fn with_relative_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, ZqnError> {
        validate_tolerance(tolerance, "relative")?;
        self.relative_tolerance = Some(tolerance);
        Ok(self)
    }

    /// Configures whether consumers must reject non-finite values.
    #[must_use]
    pub const fn require_finite(mut self, required: bool) -> Self {
        self.require_finite = required;
        self
    }

    /// Validates the complete numerical policy.
    pub fn validate(self) -> ZqnResult<()> {
        if let Some(value) = self.absolute_tolerance {
            validate_tolerance(value, "absolute")?;
        }

        if let Some(value) = self.relative_tolerance {
            validate_tolerance(value, "relative")?;
        }

        Ok(())
    }
}

fn validate_tolerance(
    tolerance: f64,
    name: &'static str,
) -> ZqnResult<()> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::InvalidStructure,
            format!(
                "ZQN {name} tolerance must be finite and non-negative"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// Cancellation
// ============================================================================

/// Thread-safe cancellation signal used by a [`ZqnContext`].
///
/// The signal contains no global state and performs no I/O.
///
/// Cancellation is cooperative: a computation must explicitly check the
/// signal at suitable interruption points.
#[derive(Clone, Debug)]
pub struct ZqnCancellation {
    cancelled: Arc<AtomicBool>,
}

impl Default for ZqnCancellation {
    fn default() -> Self {
        Self::new()
    }
}

impl ZqnCancellation {
    /// Creates a new non-cancelled signal.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Requests cancellation.
    ///
    /// Cancellation is idempotent.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Returns an error if cancellation has been requested.
    pub fn check(&self) -> ZqnResult<()> {
        if self.is_cancelled() {
            return Err(ZqnError::new(
                ZqnErrorKind::Determinism,
                ZqnErrorCode::Cancelled,
                "ZQN execution was cancelled".to_string(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Execution scope
// ============================================================================

/// Optional execution scope associated with a ZQN context.
///
/// This is deliberately an opaque ZQN object identity.
///
/// It is not:
//!
//! - a qubit ID;
//! - a physical-resource ID;
//! - a memory address;
//! - a hardware handle;
//! - an authorization credential.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct ZqnExecutionScope(Option<ZqnObjectId>);

impl ZqnExecutionScope {
    /// Creates a context without an explicit execution scope.
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    /// Creates an execution scope from a ZQN object identity.
    #[must_use]
    pub const fn from_id(id: ZqnObjectId) -> Self {
        Self(Some(id))
    }

    /// Returns the underlying optional ZQN object identity.
    #[must_use]
    pub const fn id(self) -> Option<ZqnObjectId> {
        self.0
    }

    /// Returns whether an execution scope is present.
    #[must_use]
    pub const fn is_present(self) -> bool {
        self.0.is_some()
    }
}

impl Default for ZqnExecutionScope {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Display for ZqnExecutionScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(id) => id.fmt(formatter),
            None => formatter.write_str("none"),
        }
    }
}

// ============================================================================
// Context
// ============================================================================

/// Immutable ZQN execution context.
///
/// This is the central policy/context boundary shared by ZQN operations.
///
/// The context is intentionally constant-size with respect to the quantum
/// system represented by the operation.
///
/// It does not contain a circuit, channel matrix, fault collection, qubit
/// collection, calibration table, tensor, or simulator state.
#[derive(Clone, Debug)]
pub struct ZqnContext {
    version: ZqnVersionMetadata,
    limits: ZqnLimits,
    determinism: ZqnDeterminism,
    validation: ZqnValidationMode,
    numerical: ZqnNumericalPolicy,
    execution_scope: ZqnExecutionScope,
    cancellation: ZqnCancellation,
}

impl Default for ZqnContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ZqnContext {
    /// Creates a production-safe default ZQN context.
    ///
    /// The default context:
    ///
    /// - uses the current ZQN version metadata;
    /// - imposes no additional ZQN resource ceilings;
    /// - uses deterministic execution with seed `0`;
    /// - uses strict validation;
    /// - requires finite numerical values;
    /// - has no explicit execution scope;
    /// - is not cancelled.
    ///
    /// Seed `0` is not a claim that every stochastic operation will return the
    /// same result independent of its semantic coordinates. It is the root
    /// seed from which consumers must derive stable substreams.
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: ZQN_VERSION_METADATA,
            limits: ZqnLimits::default(),
            determinism: ZqnDeterminism::default(),
            validation: ZqnValidationMode::Strict,
            numerical: ZqnNumericalPolicy::strict(),
            execution_scope: ZqnExecutionScope::none(),
            cancellation: ZqnCancellation::new(),
        }
    }

    /// Creates a context from an explicit resource policy.
    ///
    /// The supplied policy is copied into the immutable context.
    ///
    /// This function validates the policy before construction.
    pub fn with_limits(limits: ZqnLimits) -> Result<Self, LimitError> {
        limits.validate()?;

        let mut context = Self::new();
        context.limits = limits;
        Ok(context)
    }

    /// Creates a context with the supplied version metadata.
    ///
    /// Version metadata is copied; the context does not become the owner of
    /// version semantics.
    #[must_use]
    pub const fn with_version(
        mut self,
        version: ZqnVersionMetadata,
    ) -> Self {
        self.version = version;
        self
    }

    /// Replaces the resource policy after validating it.
    ///
    /// This returns a new context. Existing contexts are never mutated.
    pub fn with_limits_policy(
        mut self,
        limits: ZqnLimits,
    ) -> Result<Self, LimitError> {
        limits.validate()?;
        self.limits = limits;
        Ok(self)
    }

    /// Sets deterministic execution with explicit seed material.
    ///
    /// This performs no random generation.
    #[must_use]
    pub const fn deterministic(self, seed: u64) -> Self {
        self.with_determinism(ZqnDeterminism::Deterministic { seed })
    }

    /// Sets explicitly nondeterministic execution.
    ///
    /// Consumers remain responsible for obtaining entropy safely.
    #[must_use]
    pub const fn nondeterministic(self) -> Self {
        self.with_determinism(ZqnDeterminism::Nondeterministic)
    }

    /// Sets the deterministic execution policy.
    #[must_use]
    pub const fn with_determinism(
        mut self,
        determinism: ZqnDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Sets the validation mode.
    #[must_use]
    pub const fn with_validation_mode(
        mut self,
        mode: ZqnValidationMode,
    ) -> Self {
        self.validation = mode;
        self
    }

    /// Sets the numerical policy after validating it.
    pub fn with_numerical_policy(
        mut self,
        policy: ZqnNumericalPolicy,
    ) -> ZqnResult<Self> {
        policy.validate()?;
        self.numerical = policy;
        Ok(self)
    }

    /// Associates an execution scope with the context.
    #[must_use]
    pub const fn with_execution_scope(
        mut self,
        scope: ZqnExecutionScope,
    ) -> Self {
        self.execution_scope = scope;
        self
    }

    /// Replaces the cancellation signal.
    ///
    /// The returned context shares the supplied signal.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: ZqnCancellation,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    // ------------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------------

    /// Returns the ZQN version metadata associated with this context.
    #[must_use]
    pub const fn version(&self) -> ZqnVersionMetadata {
        self.version
    }

    /// Returns the active resource policy.
    #[must_use]
    pub const fn limits(&self) -> &ZqnLimits {
        &self.limits
    }

    /// Returns the deterministic execution policy.
    #[must_use]
    pub const fn determinism(&self) -> ZqnDeterminism {
        self.determinism
    }

    /// Returns the validation mode.
    #[must_use]
    pub const fn validation_mode(&self) -> ZqnValidationMode {
        self.validation
    }

    /// Returns the numerical policy.
    #[must_use]
    pub const fn numerical_policy(&self) -> ZqnNumericalPolicy {
        self.numerical
    }

    /// Returns the execution scope.
    #[must_use]
    pub const fn execution_scope(&self) -> ZqnExecutionScope {
        self.execution_scope
    }

    /// Returns a clone of the shared cancellation signal.
    #[must_use]
    pub fn cancellation(&self) -> ZqnCancellation {
        self.cancellation.clone()
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates the complete context.
    ///
    /// This performs context-level validation only.
    ///
    /// It does not attempt to validate:
    ///
    /// - quantum channels;
    /// - noise models;
    /// - calibration;
    /// - hardware;
    /// - target capabilities;
    /// - circuits.
    ///
    /// Those belong to their owning modules.
    pub fn validate(&self) -> ZqnResult<()> {
        self.limits
            .validate()
            .map_err(limit_error_to_zqn_error)?;

        self.numerical.validate()?;

        Ok(())
    }

    /// Checks whether cancellation has been requested.
    pub fn check_cancellation(&self) -> ZqnResult<()> {
        self.cancellation.check()
    }

    /// Checks cancellation and then validates the context.
    pub fn preflight(&self) -> ZqnResult<()> {
        self.check_cancellation()?;
        self.validate()
    }

    // ------------------------------------------------------------------------
    // Resource-policy helpers
    // ------------------------------------------------------------------------

    /// Checks a requested resource quantity against the context policy.
    pub fn check_limit(
        &self,
        resource: LimitKind,
        requested: ResourceCount,
    ) -> Result<(), LimitError> {
        self.limit_for(resource).check(resource, requested)
    }

    /// Returns the configured policy dimension corresponding to `resource`.
    ///
    /// This method centralizes resource-to-policy mapping so consumers do not
    /// need to duplicate knowledge of `ZqnLimits`.
    #[must_use]
    pub const fn limit_for(
        &self,
        resource: LimitKind,
    ) -> Limit {
        match resource {
            LimitKind::LogicalResources => self.limits.logical_resources,
            LimitKind::PhysicalResources => self.limits.physical_resources,
            LimitKind::ClassicalResources => self.limits.classical_resources,
            LimitKind::Operations => self.limits.operations,
            LimitKind::Depth => self.limits.depth,
            LimitKind::Faults => self.limits.faults,
            LimitKind::CorrelatedResources => {
                self.limits.correlated_resources
            }
            LimitKind::DistributionEntries => {
                self.limits.distribution_entries
            }
            LimitKind::Samples => self.limits.samples,
            LimitKind::TensorElements => self.limits.tensor_elements,
            LimitKind::MatrixElements => self.limits.matrix_elements,
            LimitKind::ChannelOperators => self.limits.channel_operators,
            LimitKind::Parameters => self.limits.parameters,
            LimitKind::CalibrationEntries => {
                self.limits.calibration_entries
            }
            LimitKind::Observations => self.limits.observations,
            LimitKind::Experiments => self.limits.experiments,
            LimitKind::NoiseApplications => {
                self.limits.noise_applications
            }
            LimitKind::BufferedEvents => self.limits.buffered_events,
            LimitKind::MemoryBytes => self.limits.memory_bytes,
            LimitKind::SerializedBytes => self.limits.serialized_bytes,
            LimitKind::ParallelTasks => self.limits.parallel_tasks,
            LimitKind::ExecutionNodes => self.limits.execution_nodes,
            LimitKind::ExecutionLinks => self.limits.execution_links,
            LimitKind::TimeSteps => self.limits.time_steps,
            LimitKind::Pulses => self.limits.pulses,
            LimitKind::Measurements => self.limits.measurements,
            LimitKind::Resets => self.limits.resets,
            LimitKind::TransportOperations => {
                self.limits.transport_operations
            }
            LimitKind::CompositeResources => {
                self.limits.composite_resources
            }
            LimitKind::VerificationOperations => {
                self.limits.verification_operations
            }
            LimitKind::Unknown => Limit::unlimited(),
        }
    }

    /// Checks an addition against a resource limit using the underlying
    /// checked arithmetic provided by `ZqnLimits`.
    pub fn checked_add(
        &self,
        resource: LimitKind,
        left: ResourceCount,
        right: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        self.limits.checked_add(resource, left, right)
    }

    /// Checks a multiplication against a resource limit using the underlying
    /// checked arithmetic provided by `ZqnLimits`.
    pub fn checked_mul(
        &self,
        resource: LimitKind,
        left: ResourceCount,
        right: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        self.limits.checked_mul(resource, left, right)
    }

    /// Checks an accumulated resource total.
    ///
    /// The operation is performed by the canonical `ZqnLimits` arithmetic
    /// layer.
    pub fn checked_accumulate(
        &self,
        resource: LimitKind,
        current: ResourceCount,
        additional: ResourceCount,
    ) -> Result<ResourceCount, LimitError> {
        self.limits
            .checked_accumulate(resource, current, additional)
    }

    // ------------------------------------------------------------------------
    // Child-context derivation
    // ------------------------------------------------------------------------

    /// Creates a child context that cannot weaken the parent's resource policy.
    ///
    /// For every resource dimension:
    ///
    /// ```text
    /// child = min(parent, requested)
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// parent = bounded(100)
    /// requested = unlimited
    /// child = bounded(100)
    /// ```
    ///
    /// and:
    ///
    /// ```text
    /// parent = bounded(100)
    /// requested = bounded(10)
    /// child = bounded(10)
    /// ```
    ///
    /// This property is critical for nested simulation, characterization,
    /// QEC, benchmarking, and distributed execution.
    pub fn child_with_limits(
        &self,
        requested: ZqnLimits,
    ) -> Result<Self, LimitError> {
        requested.validate()?;

        let mut child = self.clone();

        child.limits = intersect_limits(&self.limits, &requested)?;

        Ok(child)
    }

    /// Creates a child context with a different execution scope while
    /// preserving every other parent policy.
    #[must_use]
    pub const fn child_with_scope(
        &self,
        scope: ZqnExecutionScope,
    ) -> Self {
        Self {
            version: self.version,
            limits: self.limits,
            determinism: self.determinism,
            validation: self.validation,
            numerical: self.numerical,
            execution_scope: scope,
            cancellation: self.cancellation.clone(),
        }
    }

    /// Creates a child context with an explicit deterministic seed.
    ///
    /// The caller is responsible for deriving a semantically stable child
    /// seed. This method does not hash, mix, or generate seed material.
    #[must_use]
    pub const fn child_with_determinism(
        &self,
        determinism: ZqnDeterminism,
    ) -> Self {
        Self {
            version: self.version,
            limits: self.limits,
            determinism,
            validation: self.validation,
            numerical: self.numerical,
            execution_scope: self.execution_scope,
            cancellation: self.cancellation.clone(),
        }
    }

    // ------------------------------------------------------------------------
    // Stable semantic identity helpers
    // ------------------------------------------------------------------------

    /// Returns the root deterministic seed when deterministic execution is
    /// enabled.
    #[must_use]
    pub const fn seed(&self) -> Option<u64> {
        self.determinism.seed()
    }

    /// Returns whether this context requires deterministic stochastic
    /// execution.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        self.determinism.is_deterministic()
    }

    /// Returns whether strict validation is active.
    #[must_use]
    pub const fn is_strict(&self) -> bool {
        self.validation.is_strict()
    }
}

// ============================================================================
// Limit intersection
// ============================================================================

/// Intersects two complete resource policies.
///
/// The result is the most restrictive policy in every resource dimension.
///
/// This function deliberately enumerates the fields of `ZqnLimits` instead of
/// using reflection, maps, strings, or dynamic field lookup.
///
/// That keeps the operation:
///
/// - allocation-free;
/// - deterministic;
/// - type-safe;
/// - constant-size.
///
/// A new resource dimension added to `ZqnLimits` requires an explicit update to
/// this function. That is intentional because changing the policy schema is a
/// deliberate API/schema change rather than something that can silently escape
/// the child-context restriction.
fn intersect_limits(
    parent: &ZqnLimits,
    requested: &ZqnLimits,
) -> Result<ZqnLimits, LimitError> {
    Ok(ZqnLimits {
        schema_version: parent.schema_version,

        logical_resources: parent
            .logical_resources
            .minimum(requested.logical_resources),

        physical_resources: parent
            .physical_resources
            .minimum(requested.physical_resources),

        classical_resources: parent
            .classical_resources
            .minimum(requested.classical_resources),

        operations: parent
            .operations
            .minimum(requested.operations),

        depth: parent.depth.minimum(requested.depth),

        faults: parent.faults.minimum(requested.faults),

        correlated_resources: parent
            .correlated_resources
            .minimum(requested.correlated_resources),

        distribution_entries: parent
            .distribution_entries
            .minimum(requested.distribution_entries),

        samples: parent.samples.minimum(requested.samples),

        tensor_elements: parent
            .tensor_elements
            .minimum(requested.tensor_elements),

        matrix_elements: parent
            .matrix_elements
            .minimum(requested.matrix_elements),

        channel_operators: parent
            .channel_operators
            .minimum(requested.channel_operators),

        parameters: parent.parameters.minimum(requested.parameters),

        calibration_entries: parent
            .calibration_entries
            .minimum(requested.calibration_entries),

        observations: parent
            .observations
            .minimum(requested.observations),

        experiments: parent
            .experiments
            .minimum(requested.experiments),

        noise_applications: parent
            .noise_applications
            .minimum(requested.noise_applications),

        buffered_events: parent
            .buffered_events
            .minimum(requested.buffered_events),

        memory_bytes: parent
            .memory_bytes
            .minimum(requested.memory_bytes),

        serialized_bytes: parent
            .serialized_bytes
            .minimum(requested.serialized_bytes),

        parallel_tasks: parent
            .parallel_tasks
            .minimum(requested.parallel_tasks),

        execution_nodes: parent
            .execution_nodes
            .minimum(requested.execution_nodes),

        execution_links: parent
            .execution_links
            .minimum(requested.execution_links),

        time_steps: parent
            .time_steps
            .minimum(requested.time_steps),

        pulses: parent.pulses.minimum(requested.pulses),

        measurements: parent
            .measurements
            .minimum(requested.measurements),

        resets: parent.resets.minimum(requested.resets),

        transport_operations: parent
            .transport_operations
            .minimum(requested.transport_operations),

        composite_resources: parent
            .composite_resources
            .minimum(requested.composite_resources),

        verification_operations: parent
            .verification_operations
            .minimum(requested.verification_operations),
    })
}

// ============================================================================
// Limit error conversion
// ============================================================================

fn limit_error_to_zqn_error(error: LimitError) -> ZqnError {
    let message = error.to_string();

    match error {
        LimitError::ZeroLimit { .. }
        | LimitError::Exceeded { .. }
        | LimitError::InvalidValue { .. } => ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::LimitExceeded,
            message,
        ),

        LimitError::ArithmeticOverflow { .. }
        | LimitError::HostSizeOverflow { .. } => ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            message,
        ),

        LimitError::UnsupportedSchema { .. } => ZqnError::new(
            ZqnErrorKind::Version,
            ZqnErrorCode::UnsupportedSchema,
            message,
        ),

        LimitError::Inconsistent { .. } => ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::InvalidStructure,
            message,
        ),
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for ZqnContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ZqnContext{{version={}, validation={}, determinism={}, \
             execution_scope={}, limits=..., cancellation={}}}",
            self.version.semantic,
            self.validation,
            self.determinism,
            self.execution_scope,
            self.cancellation.is_cancelled(),
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_is_valid() {
        let context = ZqnContext::new();

        assert!(context.validate().is_ok());
        assert!(context.is_strict());
        assert!(context.is_deterministic());
        assert_eq!(context.seed(), Some(0));
        assert!(!context.cancellation().is_cancelled());
    }

    #[test]
    fn default_context_has_no_zqn_resource_ceiling() {
        let context = ZqnContext::new();

        assert!(context
            .limit_for(LimitKind::LogicalResources)
            .is_unlimited());

        assert!(context
            .limit_for(LimitKind::Operations)
            .is_unlimited());

        assert!(context
            .limit_for(LimitKind::Samples)
            .is_unlimited());
    }

    #[test]
    fn deterministic_policy_is_explicit() {
        let context = ZqnContext::new().deterministic(1234);

        assert!(context.is_deterministic());
        assert_eq!(context.seed(), Some(1234));
    }

    #[test]
    fn nondeterministic_policy_is_explicit() {
        let context = ZqnContext::new().nondeterministic();

        assert!(!context.is_deterministic());
        assert_eq!(context.seed(), None);
    }

    #[test]
    fn cancellation_is_shared() {
        let context = ZqnContext::new();
        let cancellation = context.cancellation();

        assert!(!context.cancellation().is_cancelled());

        cancellation.cancel();

        assert!(context.cancellation().is_cancelled());
        assert!(context.check_cancellation().is_err());
    }

    #[test]
    fn cancellation_is_idempotent() {
        let cancellation = ZqnCancellation::new();

        cancellation.cancel();
        cancellation.cancel();

        assert!(cancellation.is_cancelled());
    }

    #[test]
    fn child_context_cannot_weaken_parent_limit() {
        let parent_limits = ZqnLimits::default()
            .with_operations(
                Limit::bounded_for(LimitKind::Operations, 100)
                    .expect("positive limit"),
            );

        let parent =
            ZqnContext::with_limits(parent_limits).expect("valid limits");

        let requested = ZqnLimits::default();

        let child = parent
            .child_with_limits(requested)
            .expect("valid child policy");

        assert_eq!(
            child
                .limit_for(LimitKind::Operations)
                .maximum(),
            Some(100)
        );
    }

    #[test]
    fn child_context_can_tighten_parent_limit() {
        let parent_limits = ZqnLimits::default()
            .with_operations(
                Limit::bounded_for(LimitKind::Operations, 100)
                    .expect("positive limit"),
            );

        let parent =
            ZqnContext::with_limits(parent_limits).expect("valid limits");

        let requested = ZqnLimits::default()
            .with_operations(
                Limit::bounded_for(LimitKind::Operations, 25)
                    .expect("positive limit"),
            );

        let child = parent
            .child_with_limits(requested)
            .expect("valid child policy");

        assert_eq!(
            child
                .limit_for(LimitKind::Operations)
                .maximum(),
            Some(25)
        );
    }

    #[test]
    fn unlimited_parent_and_bounded_child_remain_bounded() {
        let parent = ZqnContext::new();

        let requested = ZqnLimits::default()
            .with_operations(
                Limit::bounded_for(LimitKind::Operations, 25)
                    .expect("positive limit"),
            );

        let child = parent
            .child_with_limits(requested)
            .expect("valid child policy");

        assert_eq!(
            child
                .limit_for(LimitKind::Operations)
                .maximum(),
            Some(25)
        );
    }

    #[test]
    fn numerical_policy_rejects_nan() {
        let result =
            ZqnNumericalPolicy::strict().with_absolute_tolerance(f64::NAN);

        assert!(result.is_err());
    }

    #[test]
    fn numerical_policy_rejects_infinity() {
        let result =
            ZqnNumericalPolicy::strict()
                .with_relative_tolerance(f64::INFINITY);

        assert!(result.is_err());
    }

    #[test]
    fn numerical_policy_rejects_negative_tolerance() {
        let result =
            ZqnNumericalPolicy::strict().with_absolute_tolerance(-1.0);

        assert!(result.is_err());
    }

    #[test]
    fn numerical_policy_accepts_zero_tolerance() {
        let result =
            ZqnNumericalPolicy::strict().with_absolute_tolerance(0.0);

        assert!(result.is_ok());
    }

    #[test]
    fn execution_scope_is_not_a_qubit_identity() {
        let scope = ZqnExecutionScope::from_id(ZqnObjectId::new(7));

        assert!(scope.is_present());
        assert_eq!(scope.id(), Some(ZqnObjectId::new(7)));
    }

    #[test]
    fn child_scope_preserves_parent_policy() {
        let parent = ZqnContext::new().deterministic(42);

        let child =
            parent.child_with_scope(ZqnExecutionScope::from_id(
                ZqnObjectId::new(100),
            ));

        assert_eq!(child.seed(), Some(42));
        assert_eq!(
            child.execution_scope().id(),
            Some(ZqnObjectId::new(100))
        );
    }

    #[test]
    fn child_determinism_is_explicit() {
        let parent = ZqnContext::new().deterministic(42);

        let child = parent.child_with_determinism(
            ZqnDeterminism::Deterministic { seed: 99 },
        );

        assert_eq!(parent.seed(), Some(42));
        assert_eq!(child.seed(), Some(99));
    }

    #[test]
    fn context_display_is_deterministic() {
        let context = ZqnContext::new().deterministic(42);

        let first = context.to_string();
        let second = context.to_string();

        assert_eq!(first, second);
    }

    #[test]
    fn validation_mode_is_explicit() {
        let strict = ZqnContext::new();

        let permissive = strict
            .clone()
            .with_validation_mode(ZqnValidationMode::Permissive);

        assert!(strict.is_strict());
        assert!(!permissive.is_strict());
    }

    #[test]
    fn version_is_current_by_default() {
        let context = ZqnContext::new();

        assert_eq!(context.version(), ZQN_VERSION_METADATA);
    }

    #[test]
    fn unknown_resource_has_no_zqn_limit() {
        let context = ZqnContext::new();

        assert!(context
            .limit_for(LimitKind::Unknown)
            .is_unlimited());
    }

    #[test]
    fn context_does_not_capture_wall_clock_time() {
        let first = ZqnContext::new();
        let second = ZqnContext::new();

        assert_eq!(first.version(), second.version());
        assert_eq!(first.determinism(), second.determinism());
        assert_eq!(
            first.validation_mode(),
            second.validation_mode()
        );
        assert_eq!(
            first.execution_scope(),
            second.execution_scope()
        );
    }

    #[test]
    fn cancellation_does_not_change_context_semantics() {
        let context = ZqnContext::new();
        let cancellation = context.cancellation();

        cancellation.cancel();

        assert_eq!(context.seed(), Some(0));
        assert_eq!(
            context.validation_mode(),
            ZqnValidationMode::Strict
        );
    }
}