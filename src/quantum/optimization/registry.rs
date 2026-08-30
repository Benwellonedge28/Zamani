//! Zamani Quantum Optimization — Pass Registry
//!
//! Production registry for quantum optimization passes.
//!
//! # Architectural role
//!
//! `registry.rs` owns discovery and construction of optimization passes.
//! It does NOT:
//!
//! - execute passes;
//! - own the optimization pipeline;
//! - own scheduling;
//! - own analysis computation;
//! - own the canonical Quantum IR;
//! - perform circuit transformations;
//! - perform backend I/O;
//! - perform hardware routing;
//! - perform quantum execution;
//! - own global mutable compiler state.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization::pass
//!      │
//!      ▼
//! optimization::registry
//!      │
//!      ├──────────────► optimization::pipeline
//!      │
//!      ├──────────────► optimization::planner
//!      │
//!      └──────────────► optimization::scheduler
//! ```
//!
//! The registry is deliberately independent of the pipeline. A pipeline may
//! use a registry, but a registry must never execute a pipeline.
//!
//! # Design goals
//!
//! This implementation is designed for:
//!
//! - deterministic pass discovery;
//! - deterministic pass construction;
//! - duplicate detection;
//! - aliases;
//! - pass families/categories;
//! - pass capabilities;
//! - target compatibility metadata;
//! - thread-safe immutable snapshots;
//! - scoped registries;
//! - compiler/test isolation;
//! - future dynamically supplied passes;
//! - zero unsafe code;
//! - no process-global mutable state;
//! - no dependency on a specific backend;
//! - no dependency on a specific quantum hardware provider;
//! - bounded registry growth;
//! - explicit replacement/removal semantics;
//! - stable machine-readable pass identifiers;
//! - human-readable diagnostics;
//! - Rust 1.97 / Rust 1.97.1 compatibility.
//!
//! # Important architectural rule
//!
//! The canonical quantum representation remains `quantum::ir`.
//!
//! This registry MUST NOT introduce a second `QuantumGate`, `QuantumCircuit`,
//! or other competing representation.
//!
//! # Pass registration model
//!
//! A pass is registered using:
//!
//! - a stable identifier;
//! - a human-readable name;
//! - a version;
//! - a description;
//! - a factory;
//! - metadata describing capabilities and requirements;
//! - optional aliases.
//!
//! Example:
//!
//! ```text
//! registry.register(
//!     PassRegistration::new(
//!         PassDescriptor::new(
//!             PassId::new("local.cancellation"),
//!             "Gate Cancellation",
//!             "1.0.0",
//!             "Removes adjacent inverse operations.",
//!             PassFactory::from_fn(|| {
//!                 Box::new(CancellationPass::default())
//!             }),
//!         )
//!         .with_category(PassCategory::Local)
//!         .with_capability(PassCapability::PreservesUnitarySemantics)
//!         .with_alias("cancellation"),
//!     )
//! )?;
//! ```
//!
//! The exact concrete pass implementation belongs in `pass.rs` and the
//! individual pass modules.
//!
//! # Determinism
//!
//! `BTreeMap` is intentionally used instead of `HashMap` for the authoritative
//! registry. Registry iteration therefore has stable ordering independent of
//! randomized hashing.
//!
//! # Concurrency
//!
//! The registry itself does not require a mutex because it is normally built
//! during compiler setup and then shared immutably.
//!
//! `PassFactory` is required to be `Send + Sync`, and constructed passes are
//! required to be `Send`.
//!
//! This permits:
//!
//! ```text
//! registry
//!     │
//!     ├── compiler thread
//!     ├── worker thread
//!     ├── worker thread
//!     └── worker thread
//! ```
//!
//! without process-global mutable registration state.
//!
//! # No unsafe
//!
//! This file intentionally contains no `unsafe` code and requires no unsafe
//! support from downstream optimization passes.
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
//! # Integration contract
//!
//! `pass.rs` must provide the object-safe `OptimizationPass` trait used by this
//! registry.
//!
//! The trait is expected to be constructible as a trait object:
//!
//! ```text
//! Box<dyn OptimizationPass>
//! ```
//!
//! and should be `Send`.
//!
//! The registry intentionally does not depend on concrete optimization pass
//! implementations. Individual passes register themselves through explicit
//! registration functions owned by `optimization::mod.rs`, `planner.rs`, or
//! compiler initialization code.
//!
//! This keeps the registry reusable for:
//!
//! - local optimization;
//! - algebraic optimization;
//! - synthesis;
//! - fault-tolerant optimization;
//! - parameter optimization;
//! - structural optimization;
//! - target-aware optimization;
//! - future quantum computing models.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use super::pass::OptimizationPass;

// =============================================================================
// Public constants
// =============================================================================

/// Maximum identifier length accepted by the registry.
///
/// This protects diagnostics, configuration files, logs, and registry memory
/// from accidentally enormous identifiers.
pub const MAX_PASS_ID_LENGTH: usize = 256;

/// Maximum human-readable pass name length.
pub const MAX_PASS_NAME_LENGTH: usize = 512;

/// Maximum pass description length.
pub const MAX_PASS_DESCRIPTION_LENGTH: usize = 4096;

/// Maximum alias length.
pub const MAX_PASS_ALIAS_LENGTH: usize = 256;

/// Maximum number of aliases allowed for one pass.
pub const MAX_ALIASES_PER_PASS: usize = 64;

/// Maximum number of tags allowed for one pass.
pub const MAX_TAGS_PER_PASS: usize = 64;

/// Maximum number of registered passes in one registry.
///
/// This is a safety boundary, not an architectural limit on the number of
/// passes Zamani can eventually support. It can be increased through a future
/// resource-policy layer without changing the registry's public model.
pub const DEFAULT_MAX_REGISTERED_PASSES: usize = 16_384;

/// Maximum number of registered aliases in one registry.
pub const DEFAULT_MAX_REGISTERED_ALIASES: usize = 65_536;

// =============================================================================
// Pass ID
// =============================================================================

/// Stable machine-readable optimization pass identifier.
///
/// Pass IDs are semantic API identifiers and should remain stable across
/// compiler releases once published.
///
/// Recommended naming:
///
/// ```text
/// local.cancellation
/// local.peephole
/// local.rotation
/// algebra.clifford
/// algebra.phase_polynomial
/// synthesis.single_qubit
/// synthesis.two_qubit
/// fault_tolerant.t_count
/// fault_tolerant.t_depth
/// structural.control_flow
/// parameter.constant_fold
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(String);

impl PassId {
    /// Creates a validated pass identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, RegistryError> {
        let value = value.into();

        validate_identifier(
            &value,
            MAX_PASS_ID_LENGTH,
            "pass identifier",
        )?;

        Ok(Self(value))
    }

    /// Creates a pass identifier from a static string.
    ///
    /// This function is intended for compile-time-known identifiers.
    ///
    /// # Panics
    ///
    /// Panics only if the programmer supplies an invalid static identifier.
    pub fn from_static(value: &'static str) -> Self {
        Self::new(value)
            .expect("static optimization pass identifier must be valid")
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the identifier is namespaced.
    pub fn is_namespaced(&self) -> bool {
        self.0.contains('.')
    }

    /// Returns the namespace component before the first dot.
    pub fn namespace(&self) -> Option<&str> {
        self.0.split_once('.').map(|(namespace, _)| namespace)
    }

    /// Returns the local name after the final dot.
    pub fn local_name(&self) -> &str {
        self.0
            .rsplit_once('.')
            .map(|(_, name)| name)
            .unwrap_or(self.as_str())
    }
}

impl fmt::Display for PassId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for PassId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Pass version
// =============================================================================

/// Registry-visible pass version.
///
/// This is deliberately independent of the crate/package version.
///
/// A pass can evolve independently from the overall Zamani compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassVersion {
    /// Major compatibility version.
    pub major: u16,

    /// Minor feature version.
    pub minor: u16,

    /// Patch/bug-fix version.
    pub patch: u16,
}

impl PassVersion {
    /// Creates a pass version.
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Stable version string.
    pub fn as_string(&self) -> String {
        format!(
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

impl Default for PassVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for PassVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Pass category
// =============================================================================

/// Broad optimization-pass classification.
///
/// Categories are metadata, not execution ordering.
///
/// The pipeline/planner decides execution order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PassCategory {
    /// Canonicalization and normalization.
    Normalization,

    /// Small local rewrites.
    Local,

    /// Algebraic transformations.
    Algebraic,

    /// Circuit synthesis.
    Synthesis,

    /// Fault-tolerant optimization.
    FaultTolerant,

    /// Parameter/symbolic optimization.
    Parameter,

    /// Structural/control-flow optimization.
    Structural,

    /// Target-aware optimization.
    TargetAware,

    /// Approximate or stochastic optimization.
    Stochastic,

    /// Verification-oriented transformation.
    Verification,

    /// General composite pass.
    Composite,

    /// Extension point for future quantum models.
    Other,
}

impl PassCategory {
    /// Stable machine-readable category name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normalization => "normalization",
            Self::Local => "local",
            Self::Algebraic => "algebraic",
            Self::Synthesis => "synthesis",
            Self::FaultTolerant => "fault_tolerant",
            Self::Parameter => "parameter",
            Self::Structural => "structural",
            Self::TargetAware => "target_aware",
            Self::Stochastic => "stochastic",
            Self::Verification => "verification",
            Self::Composite => "composite",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for PassCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Pass capability
// =============================================================================

/// Declares a property or capability of an optimization pass.
///
/// Capabilities are deliberately explicit because the planner must not infer
/// semantic safety from a pass's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PassCapability {
    /// Pass can operate on an empty circuit.
    HandlesEmptyCircuit,

    /// Pass preserves exact unitary semantics.
    PreservesUnitarySemantics,

    /// Pass preserves semantics up to global phase.
    PreservesGlobalPhaseEquivalentSemantics,

    /// Pass preserves computational-basis measurement distributions.
    PreservesMeasurementSemantics,

    /// Pass may change global phase.
    MayChangeGlobalPhase,

    /// Pass can process symbolic parameters.
    SupportsSymbolicParameters,

    /// Pass requires parameters to be numerically bound.
    RequiresBoundParameters,

    /// Pass supports measurements.
    SupportsMeasurements,

    /// Pass supports reset operations.
    SupportsReset,

    /// Pass supports classical control.
    SupportsClassicalControl,

    /// Pass supports structured control flow.
    SupportsControlFlow,

    /// Pass can operate on multi-qubit gates.
    SupportsMultiQubitOperations,

    /// Pass can synthesize operations.
    PerformsSynthesis,

    /// Pass can introduce operations.
    MayIntroduceOperations,

    /// Pass can remove operations.
    MayRemoveOperations,

    /// Pass can change circuit depth.
    MayChangeDepth,

    /// Pass can change qubit width.
    MayChangeWidth,

    /// Pass can change two-qubit gate count.
    MayChangeTwoQubitCount,

    /// Pass can change T count.
    MayChangeTCount,

    /// Pass can change T depth.
    MayChangeTDepth,

    /// Pass is target-independent.
    TargetIndependent,

    /// Pass requires a target gate set.
    RequiresTargetGateSet,

    /// Pass may be computationally expensive.
    Expensive,

    /// Pass is suitable for aggressive optimization.
    Aggressive,

    /// Pass is deterministic when its inputs/configuration are deterministic.
    Deterministic,

    /// Pass uses a random source.
    UsesRandomness,

    /// Pass supports deterministic seeded execution.
    SupportsDeterministicSeed,

    /// Pass can be safely repeated until a fixed point.
    FixedPointSafe,

    /// Pass is intended to run at most once in a pipeline stage.
    SingleShotRecommended,

    /// Pass supports parallel execution when the pipeline allows it.
    ParallelSafe,

    /// Pass requires exclusive access to the circuit.
    RequiresExclusiveCircuitAccess,
}

impl PassCapability {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandlesEmptyCircuit => "handles_empty_circuit",
            Self::PreservesUnitarySemantics => {
                "preserves_unitary_semantics"
            }
            Self::PreservesGlobalPhaseEquivalentSemantics => {
                "preserves_global_phase_equivalent_semantics"
            }
            Self::PreservesMeasurementSemantics => {
                "preserves_measurement_semantics"
            }
            Self::MayChangeGlobalPhase => "may_change_global_phase",
            Self::SupportsSymbolicParameters => {
                "supports_symbolic_parameters"
            }
            Self::RequiresBoundParameters => {
                "requires_bound_parameters"
            }
            Self::SupportsMeasurements => "supports_measurements",
            Self::SupportsReset => "supports_reset",
            Self::SupportsClassicalControl => {
                "supports_classical_control"
            }
            Self::SupportsControlFlow => "supports_control_flow",
            Self::SupportsMultiQubitOperations => {
                "supports_multi_qubit_operations"
            }
            Self::PerformsSynthesis => "performs_synthesis",
            Self::MayIntroduceOperations => "may_introduce_operations",
            Self::MayRemoveOperations => "may_remove_operations",
            Self::MayChangeDepth => "may_change_depth",
            Self::MayChangeWidth => "may_change_width",
            Self::MayChangeTwoQubitCount => {
                "may_change_two_qubit_count"
            }
            Self::MayChangeTCount => "may_change_t_count",
            Self::MayChangeTDepth => "may_change_t_depth",
            Self::TargetIndependent => "target_independent",
            Self::RequiresTargetGateSet => {
                "requires_target_gate_set"
            }
            Self::Expensive => "expensive",
            Self::Aggressive => "aggressive",
            Self::Deterministic => "deterministic",
            Self::UsesRandomness => "uses_randomness",
            Self::SupportsDeterministicSeed => {
                "supports_deterministic_seed"
            }
            Self::FixedPointSafe => "fixed_point_safe",
            Self::SingleShotRecommended => "single_shot_recommended",
            Self::ParallelSafe => "parallel_safe",
            Self::RequiresExclusiveCircuitAccess => {
                "requires_exclusive_circuit_access"
            }
        }
    }
}

// =============================================================================
// Pass target family
// =============================================================================

/// Declares the broad target families for which a pass is useful.
///
/// These are intentionally not hardware vendor identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PassTarget {
    /// Generic hardware-independent logical circuits.
    Generic,

    /// Clifford circuits.
    Clifford,

    /// Clifford+T circuits.
    CliffordT,

    /// Fault-tolerant logical circuits.
    FaultTolerant,

    /// Superconducting-style native gate targets.
    Superconducting,

    /// Trapped-ion-style native gate targets.
    TrappedIon,

    /// Neutral-atom-style targets.
    NeutralAtom,

    /// Photonic targets.
    Photonic,

    /// Continuous-variable targets.
    ContinuousVariable,

    /// Simulation-oriented optimization.
    Simulation,

    /// Logical-only optimization.
    Logical,

    /// Target-independent algorithmic optimization.
    Algorithmic,

    /// Future/unknown target family.
    Other,
}

impl PassTarget {
    /// Stable machine-readable target name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Clifford => "clifford",
            Self::CliffordT => "clifford_t",
            Self::FaultTolerant => "fault_tolerant",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::Simulation => "simulation",
            Self::Logical => "logical",
            Self::Algorithmic => "algorithmic",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for PassTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Pass descriptor
// =============================================================================

/// Immutable metadata describing a registered optimization pass.
///
/// This type contains no executable pass state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassDescriptor {
    /// Stable machine-readable identifier.
    id: PassId,

    /// Human-readable name.
    name: String,

    /// Pass version.
    version: PassVersion,

    /// Human-readable description.
    description: String,

    /// Classification.
    category: PassCategory,

    /// Declared capabilities.
    capabilities: BTreeSet<PassCapability>,

    /// Supported target families.
    targets: BTreeSet<PassTarget>,

    /// User-facing aliases.
    aliases: BTreeSet<String>,

    /// Additional machine-readable tags.
    tags: BTreeSet<String>,

    /// Whether this pass is part of the stable public optimization API.
    stable: bool,

    /// Whether the pass is enabled for automatic discovery.
    enabled_by_default: bool,

    /// Optional replacement pass identifier.
    replacement: Option<PassId>,
}

impl PassDescriptor {
    /// Creates a descriptor.
    pub fn new(
        id: PassId,
        name: impl Into<String>,
        version: PassVersion,
        description: impl Into<String>,
    ) -> Result<Self, RegistryError> {
        let name = name.into();
        let description = description.into();

        validate_text(
            &name,
            MAX_PASS_NAME_LENGTH,
            "pass name",
        )?;

        validate_text(
            &description,
            MAX_PASS_DESCRIPTION_LENGTH,
            "pass description",
        )?;

        Ok(Self {
            id,
            name,
            version,
            description,
            category: PassCategory::Other,
            capabilities: BTreeSet::new(),
            targets: BTreeSet::from([PassTarget::Generic]),
            aliases: BTreeSet::new(),
            tags: BTreeSet::new(),
            stable: false,
            enabled_by_default: false,
            replacement: None,
        })
    }

    /// Returns the pass ID.
    pub fn id(&self) -> &PassId {
        &self.id
    }

    /// Returns the human-readable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the pass version.
    pub const fn version(&self) -> PassVersion {
        self.version
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the category.
    pub const fn category(&self) -> PassCategory {
        self.category
    }

    /// Returns all capabilities.
    pub fn capabilities(
        &self,
    ) -> impl Iterator<Item = PassCapability> + '_ {
        self.capabilities.iter().copied()
    }

    /// Returns all supported target families.
    pub fn targets(&self) -> impl Iterator<Item = PassTarget> + '_ {
        self.targets.iter().copied()
    }

    /// Returns all aliases.
    pub fn aliases(&self) -> impl Iterator<Item = &str> + '_ {
        self.aliases.iter().map(String::as_str)
    }

    /// Returns all tags.
    pub fn tags(&self) -> impl Iterator<Item = &str> + '_ {
        self.tags.iter().map(String::as_str)
    }

    /// Returns whether the pass is stable.
    pub const fn is_stable(&self) -> bool {
        self.stable
    }

    /// Returns whether automatic profiles may select this pass.
    pub const fn enabled_by_default(&self) -> bool {
        self.enabled_by_default
    }

    /// Returns the replacement pass, if deprecated.
    pub fn replacement(&self) -> Option<&PassId> {
        self.replacement.as_ref()
    }

    /// Sets the category.
    pub fn with_category(
        mut self,
        category: PassCategory,
    ) -> Self {
        self.category = category;
        self
    }

    /// Adds a capability.
    pub fn with_capability(
        mut self,
        capability: PassCapability,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds a target family.
    pub fn with_target(
        mut self,
        target: PassTarget,
    ) -> Self {
        self.targets.insert(target);
        self
    }

    /// Replaces the complete target set.
    pub fn with_targets<I>(
        mut self,
        targets: I,
    ) -> Self
    where
        I: IntoIterator<Item = PassTarget>,
    {
        self.targets.clear();

        for target in targets {
            self.targets.insert(target);
        }

        self
    }

    /// Adds an alias.
    pub fn with_alias(
        mut self,
        alias: impl Into<String>,
    ) -> Result<Self, RegistryError> {
        let alias = normalize_alias(&alias.into())?;

        if self.aliases.len() >= MAX_ALIASES_PER_PASS {
            return Err(
                RegistryError::AliasLimitExceeded {
                    pass: self.id.clone(),
                    maximum: MAX_ALIASES_PER_PASS,
                },
            );
        }

        if alias == self.id.as_str() {
            return Err(
                RegistryError::AliasEqualsCanonicalId {
                    pass: self.id.clone(),
                    alias,
                },
            );
        }

        self.aliases.insert(alias);

        Ok(self)
    }

    /// Adds a machine-readable tag.
    pub fn with_tag(
        mut self,
        tag: impl Into<String>,
    ) -> Result<Self, RegistryError> {
        let tag = normalize_tag(&tag.into())?;

        if self.tags.len() >= MAX_TAGS_PER_PASS {
            return Err(
                RegistryError::TagLimitExceeded {
                    pass: self.id.clone(),
                    maximum: MAX_TAGS_PER_PASS,
                },
            );
        }

        self.tags.insert(tag);

        Ok(self)
    }

    /// Marks the pass as stable.
    pub fn stable(mut self) -> Self {
        self.stable = true;
        self
    }

    /// Enables the pass for automatic profile discovery.
    pub fn enabled_by_default(mut self) -> Self {
        self.enabled_by_default = true;
        self
    }

    /// Marks this pass as replaced by another pass.
    pub fn replaced_by(
        mut self,
        replacement: PassId,
    ) -> Self {
        self.replacement = Some(replacement);
        self.enabled_by_default = false;
        self
    }

    /// Returns whether a capability is present.
    pub fn has_capability(
        &self,
        capability: PassCapability,
    ) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns whether the pass advertises support for a target.
    pub fn supports_target(
        &self,
        target: PassTarget,
    ) -> bool {
        self.targets.contains(&target)
            || self.targets.contains(&PassTarget::Generic)
    }
}

// =============================================================================
// Pass factory
// =============================================================================

/// Factory for constructing independent optimization-pass instances.
///
/// A factory must create a fresh pass instance for every call.
///
/// The registry never stores an already-running pass instance.
///
/// This distinction is important for parallel compilation and for preventing
/// accidental sharing of mutable pass state between compilations.
pub type PassFactory =
    Arc<dyn Fn() -> Box<dyn OptimizationPass> + Send + Sync + 'static>;

impl fmt::Debug for PassFactory {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PassFactory")
            .finish_non_exhaustive()
    }
}

impl PassFactory {
    /// Creates a factory from a closure/function.
    pub fn from_fn<F>(factory: F) -> Self
    where
        F: Fn() -> Box<dyn OptimizationPass>
            + Send
            + Sync
            + 'static,
    {
        Arc::new(factory)
    }

    /// Constructs a fresh pass instance.
    pub fn construct(&self) -> Box<dyn OptimizationPass> {
        (self)()
    }
}

// =============================================================================
// Pass registration
// =============================================================================

/// Complete registration record for one optimization pass.
///
/// This is the unit inserted into `PassRegistry`.
#[derive(Clone)]
pub struct PassRegistration {
    /// Immutable pass metadata.
    descriptor: PassDescriptor,

    /// Fresh-instance constructor.
    factory: PassFactory,
}

impl fmt::Debug for PassRegistration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PassRegistration")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl PassRegistration {
    /// Creates a pass registration.
    pub fn new(
        descriptor: PassDescriptor,
        factory: PassFactory,
    ) -> Self {
        Self {
            descriptor,
            factory,
        }
    }

    /// Returns pass metadata.
    pub fn descriptor(&self) -> &PassDescriptor {
        &self.descriptor
    }

    /// Returns the pass factory.
    pub fn factory(&self) -> &PassFactory {
        &self.factory
    }

    /// Constructs a fresh pass instance.
    pub fn construct(
        &self,
    ) -> Box<dyn OptimizationPass> {
        self.factory.construct()
    }
}

// =============================================================================
// Registry errors
// =============================================================================

/// Errors produced by the optimization pass registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Pass ID is invalid.
    InvalidIdentifier {
        field: &'static str,
        value: String,
        maximum: usize,
    },

    /// Text field is invalid.
    InvalidText {
        field: &'static str,
        maximum: usize,
    },

    /// Alias is invalid.
    InvalidAlias {
        value: String,
    },

    /// Tag is invalid.
    InvalidTag {
        value: String,
    },

    /// A pass is already registered.
    DuplicatePass {
        id: PassId,
    },

    /// Alias is already registered.
    DuplicateAlias {
        alias: String,
        existing: PassId,
    },

    /// Alias equals the canonical pass ID.
    AliasEqualsCanonicalId {
        pass: PassId,
        alias: String,
    },

    /// Alias belongs to another pass.
    AliasConflict {
        alias: String,
        requested: PassId,
        existing: PassId,
    },

    /// Pass count exceeds registry capacity.
    RegistryCapacityExceeded {
        maximum: usize,
    },

    /// Alias count exceeds registry capacity.
    AliasCapacityExceeded {
        maximum: usize,
    },

    /// One pass has too many aliases.
    AliasLimitExceeded {
        pass: PassId,
        maximum: usize,
    },

    /// One pass has too many tags.
    TagLimitExceeded {
        pass: PassId,
        maximum: usize,
    },

    /// Requested pass does not exist.
    PassNotFound {
        name: String,
    },

    /// Alias resolves to a missing pass.
    BrokenAlias {
        alias: String,
        target: PassId,
    },

    /// Replacement target does not exist.
    MissingReplacement {
        pass: PassId,
        replacement: PassId,
    },

    /// Registry contains an inconsistent alias mapping.
    RegistryInvariantViolation {
        message: &'static str,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                field,
                value,
                maximum,
            } => write!(
                formatter,
                "invalid {field} `{value}`; maximum length is {maximum}"
            ),

            Self::InvalidText {
                field,
                maximum,
            } => write!(
                formatter,
                "invalid {field}; maximum length is {maximum}"
            ),

            Self::InvalidAlias { value } => {
                write!(formatter, "invalid pass alias `{value}`")
            }

            Self::InvalidTag { value } => {
                write!(formatter, "invalid pass tag `{value}`")
            }

            Self::DuplicatePass { id } => {
                write!(formatter, "optimization pass `{id}` is already registered")
            }

            Self::DuplicateAlias {
                alias,
                existing,
            } => write!(
                formatter,
                "alias `{alias}` is already registered for pass `{existing}`"
            ),

            Self::AliasEqualsCanonicalId { pass, alias } => write!(
                formatter,
                "alias `{alias}` cannot equal canonical pass ID `{pass}`"
            ),

            Self::AliasConflict {
                alias,
                requested,
                existing,
            } => write!(
                formatter,
                "alias `{alias}` for pass `{requested}` conflicts with pass `{existing}`"
            ),

            Self::RegistryCapacityExceeded { maximum } => write!(
                formatter,
                "optimization pass registry capacity exceeded: maximum {maximum}"
            ),

            Self::AliasCapacityExceeded { maximum } => write!(
                formatter,
                "optimization alias registry capacity exceeded: maximum {maximum}"
            ),

            Self::AliasLimitExceeded { pass, maximum } => write!(
                formatter,
                "pass `{pass}` exceeds maximum aliases per pass: {maximum}"
            ),

            Self::TagLimitExceeded { pass, maximum } => write!(
                formatter,
                "pass `{pass}` exceeds maximum tags: {maximum}"
            ),

            Self::PassNotFound { name } => {
                write!(formatter, "optimization pass `{name}` was not found")
            }

            Self::BrokenAlias { alias, target } => write!(
                formatter,
                "alias `{alias}` points to missing pass `{target}`"
            ),

            Self::MissingReplacement {
                pass,
                replacement,
            } => write!(
                formatter,
                "pass `{pass}` declares missing replacement pass `{replacement}`"
            ),

            Self::RegistryInvariantViolation { message } => {
                write!(formatter, "optimization registry invariant violated: {message}")
            }
        }
    }
}

impl std::error::Error for RegistryError {}

// =============================================================================
// Registry limits
// =============================================================================

/// Resource policy for one pass registry.
///
/// Registry limits prevent accidental unbounded growth while still allowing
/// extremely large compiler installations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegistryLimits {
    /// Maximum number of registered passes.
    pub max_passes: usize,

    /// Maximum number of aliases.
    pub max_aliases: usize,
}

impl Default for RegistryLimits {
    fn default() -> Self {
        Self {
            max_passes: DEFAULT_MAX_REGISTERED_PASSES,
            max_aliases: DEFAULT_MAX_REGISTERED_ALIASES,
        }
    }
}

impl RegistryLimits {
    /// Creates registry limits.
    pub const fn new(
        max_passes: usize,
        max_aliases: usize,
    ) -> Self {
        Self {
            max_passes,
            max_aliases,
        }
    }

    /// Validates the limits.
    pub fn validate(self) -> Result<Self, RegistryError> {
        if self.max_passes == 0 {
            return Err(
                RegistryError::RegistryCapacityExceeded {
                    maximum: 0,
                },
            );
        }

        if self.max_aliases == 0 {
            return Err(
                RegistryError::AliasCapacityExceeded {
                    maximum: 0,
                },
            );
        }

        Ok(self)
    }
}

// =============================================================================
// Registry snapshot
// =============================================================================

/// Immutable registry snapshot.
///
/// Pipelines and planners should prefer snapshots when they need a stable view
/// while another part of the compiler constructs a separate registry.
///
/// The snapshot contains cloned registrations but no mutable global state.
#[derive(Debug, Clone)]
pub struct PassRegistrySnapshot {
    registrations: BTreeMap<PassId, PassRegistration>,
    aliases: BTreeMap<String, PassId>,
}

impl PassRegistrySnapshot {
    /// Returns the number of registered passes.
    pub fn len(&self) -> usize {
        self.registrations.len()
    }

    /// Returns whether the snapshot contains no passes.
    pub fn is_empty(&self) -> bool {
        self.registrations.is_empty()
    }

    /// Returns whether a pass or alias exists.
    pub fn contains(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }

    /// Resolves a canonical ID or alias.
    pub fn resolve(&self, name: &str) -> Option<&PassRegistration> {
        let canonical = resolve_name(
            name,
            &self.registrations,
            &self.aliases,
        )?;

        self.registrations.get(canonical)
    }

    /// Returns metadata for a pass.
    pub fn descriptor(
        &self,
        name: &str,
    ) -> Option<&PassDescriptor> {
        self.resolve(name).map(PassRegistration::descriptor)
    }

    /// Constructs a pass by ID or alias.
    pub fn construct(
        &self,
        name: &str,
    ) -> Result<Box<dyn OptimizationPass>, RegistryError> {
        self.resolve(name)
            .map(PassRegistration::construct)
            .ok_or_else(|| RegistryError::PassNotFound {
                name: name.to_string(),
            })
    }

    /// Iterates over registrations in deterministic canonical-ID order.
    pub fn registrations(
        &self,
    ) -> impl Iterator<Item = &PassRegistration> {
        self.registrations.values()
    }

    /// Iterates over pass descriptors.
    pub fn descriptors(
        &self,
    ) -> impl Iterator<Item = &PassDescriptor> {
        self.registrations
            .values()
            .map(PassRegistration::descriptor)
    }

    /// Returns the canonical pass IDs.
    pub fn pass_ids(
        &self,
    ) -> impl Iterator<Item = &PassId> {
        self.registrations.keys()
    }

    /// Returns aliases in deterministic order.
    pub fn aliases(
        &self,
    ) -> impl Iterator<Item = (&str, &PassId)> {
        self.aliases
            .iter()
            .map(|(alias, id)| (alias.as_str(), id))
    }

    /// Finds all passes belonging to a category.
    pub fn by_category(
        &self,
        category: PassCategory,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration.descriptor().category() == category
            })
            .collect()
    }

    /// Finds all passes supporting a target.
    pub fn by_target(
        &self,
        target: PassTarget,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .supports_target(target)
            })
            .collect()
    }

    /// Finds all passes containing a capability.
    pub fn by_capability(
        &self,
        capability: PassCapability,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .has_capability(capability)
            })
            .collect()
    }

    /// Finds all passes containing a tag.
    pub fn by_tag(
        &self,
        tag: &str,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .tags()
                    .any(|candidate| candidate == tag)
            })
            .collect()
    }
}

// =============================================================================
// Pass registry
// =============================================================================

/// Explicit optimization-pass registry.
///
/// The registry owns registration metadata and factories but never owns an
/// executing pass instance.
///
/// It is intentionally not a singleton.
#[derive(Debug, Clone)]
pub struct PassRegistry {
    registrations: BTreeMap<PassId, PassRegistration>,
    aliases: BTreeMap<String, PassId>,
    limits: RegistryLimits,
}

impl Default for PassRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PassRegistry {
    /// Creates an empty registry using production limits.
    pub fn new() -> Self {
        Self {
            registrations: BTreeMap::new(),
            aliases: BTreeMap::new(),
            limits: RegistryLimits::default(),
        }
    }

    /// Creates an empty registry with explicit resource limits.
    pub fn with_limits(
        limits: RegistryLimits,
    ) -> Result<Self, RegistryError> {
        Ok(Self {
            registrations: BTreeMap::new(),
            aliases: BTreeMap::new(),
            limits: limits.validate()?,
        })
    }

    /// Returns registry limits.
    pub const fn limits(&self) -> RegistryLimits {
        self.limits
    }

    /// Returns the number of registered passes.
    pub fn len(&self) -> usize {
        self.registrations.len()
    }

    /// Returns whether the registry contains no passes.
    pub fn is_empty(&self) -> bool {
        self.registrations.is_empty()
    }

    /// Returns the number of registered aliases.
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }

    /// Registers one pass.
    ///
    /// Registration is atomic: if validation fails, the registry remains
    /// unchanged.
    pub fn register(
        &mut self,
        registration: PassRegistration,
    ) -> Result<(), RegistryError> {
        let descriptor = registration.descriptor();

        let id = descriptor.id().clone();

        if self.registrations.contains_key(&id) {
            return Err(
                RegistryError::DuplicatePass { id },
            );
        }

        if self.registrations.len() >= self.limits.max_passes {
            return Err(
                RegistryError::RegistryCapacityExceeded {
                    maximum: self.limits.max_passes,
                },
            );
        }

        self.validate_aliases_for_registration(
            descriptor,
        )?;

        if let Some(replacement) = descriptor.replacement() {
            if replacement == descriptor.id() {
                return Err(
                    RegistryError::MissingReplacement {
                        pass: id,
                        replacement: replacement.clone(),
                    },
                );
            }
        }

        // Insert the canonical registration only after all validation has
        // succeeded. This preserves atomicity.
        self.registrations.insert(
            id.clone(),
            registration,
        );

        // Alias insertion cannot fail after the previous validation unless
        // this method's invariants are violated. Keep the operation explicit
        // nevertheless so future changes remain easy to audit.
        let aliases = self
            .registrations
            .get(&id)
            .expect("registration inserted above")
            .descriptor()
            .aliases()
            .map(str::to_owned)
            .collect::<Vec<_>>();

        for alias in aliases {
            if self.aliases.insert(
                alias.clone(),
                id.clone(),
            ).is_some() {
                // Roll back rather than leaving a partially registered pass.
                for inserted_alias in self
                    .registrations
                    .get(&id)
                    .expect("registration exists")
                    .descriptor()
                    .aliases()
                {
                    self.aliases.remove(inserted_alias);
                }

                self.registrations.remove(&id);

                return Err(
                    RegistryError::RegistryInvariantViolation {
                        message:
                            "alias insertion unexpectedly replaced an existing alias",
                    },
                );
            }
        }

        Ok(())
    }

    /// Registers several passes atomically.
    ///
    /// If any registration fails, none of the registrations are committed.
    pub fn register_all<I>(
        &mut self,
        registrations: I,
    ) -> Result<(), RegistryError>
    where
        I: IntoIterator<Item = PassRegistration>,
    {
        let mut staged = self.clone();

        for registration in registrations {
            staged.register(registration)?;
        }

        *self = staged;

        Ok(())
    }

    /// Removes a pass and all of its aliases.
    ///
    /// Removing a pass is explicit and therefore cannot accidentally happen
    /// during normal lookup.
    pub fn unregister(
        &mut self,
        name: &str,
    ) -> Result<PassRegistration, RegistryError> {
        let id = self
            .resolve_id(name)
            .cloned()
            .ok_or_else(|| RegistryError::PassNotFound {
                name: name.to_string(),
            })?;

        let registration = self
            .registrations
            .remove(&id)
            .ok_or_else(|| {
                RegistryError::RegistryInvariantViolation {
                    message:
                        "resolved pass ID disappeared before unregister",
                }
            })?;

        let aliases = registration
            .descriptor()
            .aliases()
            .map(str::to_owned)
            .collect::<Vec<_>>();

        for alias in aliases {
            self.aliases.remove(&alias);
        }

        Ok(registration)
    }

    /// Replaces an existing pass registration atomically.
    ///
    /// Replacement is allowed only when the canonical ID is identical.
    /// Changing a canonical ID must be represented as unregister/register or
    /// by an explicit alias migration.
    pub fn replace(
        &mut self,
        registration: PassRegistration,
    ) -> Result<(), RegistryError> {
        let id = registration.descriptor().id().clone();

        if !self.registrations.contains_key(&id) {
            return Err(
                RegistryError::PassNotFound {
                    name: id.to_string(),
                },
            );
        }

        let mut staged = self.clone();

        staged.unregister(id.as_str())?;
        staged.register(registration)?;

        *self = staged;

        Ok(())
    }

    /// Returns a registration by canonical ID or alias.
    pub fn get(
        &self,
        name: &str,
    ) -> Option<&PassRegistration> {
        let id = self.resolve_id(name)?;
        self.registrations.get(id)
    }

    /// Returns a descriptor by canonical ID or alias.
    pub fn descriptor(
        &self,
        name: &str,
    ) -> Option<&PassDescriptor> {
        self.get(name)
            .map(PassRegistration::descriptor)
    }

    /// Constructs a fresh pass instance by canonical ID or alias.
    pub fn construct(
        &self,
        name: &str,
    ) -> Result<Box<dyn OptimizationPass>, RegistryError> {
        self.get(name)
            .map(PassRegistration::construct)
            .ok_or_else(|| RegistryError::PassNotFound {
                name: name.to_string(),
            })
    }

    /// Returns whether a canonical ID or alias exists.
    pub fn contains(
        &self,
        name: &str,
    ) -> bool {
        self.get(name).is_some()
    }

    /// Resolves a canonical ID or alias to a canonical ID.
    pub fn resolve_id(
        &self,
        name: &str,
    ) -> Option<&PassId> {
        let normalized = normalize_lookup_name(name).ok()?;

        if let Some(id) = self.aliases.get(&normalized) {
            return Some(id);
        }

        self.registrations
            .keys()
            .find(|id| id.as_str() == normalized)
    }

    /// Returns all registered pass IDs in deterministic order.
    pub fn pass_ids(
        &self,
    ) -> impl Iterator<Item = &PassId> {
        self.registrations.keys()
    }

    /// Returns all registrations in deterministic order.
    pub fn registrations(
        &self,
    ) -> impl Iterator<Item = &PassRegistration> {
        self.registrations.values()
    }

    /// Returns all descriptors.
    pub fn descriptors(
        &self,
    ) -> impl Iterator<Item = &PassDescriptor> {
        self.registrations
            .values()
            .map(PassRegistration::descriptor)
    }

    /// Returns all aliases.
    pub fn aliases(
        &self,
    ) -> impl Iterator<Item = (&str, &PassId)> {
        self.aliases
            .iter()
            .map(|(alias, id)| (alias.as_str(), id))
    }

    /// Finds passes by category.
    pub fn by_category(
        &self,
        category: PassCategory,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration.descriptor().category() == category
            })
            .collect()
    }

    /// Finds passes by target family.
    pub fn by_target(
        &self,
        target: PassTarget,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .supports_target(target)
            })
            .collect()
    }

    /// Finds passes by capability.
    pub fn by_capability(
        &self,
        capability: PassCapability,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .has_capability(capability)
            })
            .collect()
    }

    /// Finds passes by tag.
    pub fn by_tag(
        &self,
        tag: &str,
    ) -> Vec<&PassRegistration> {
        let tag = match normalize_tag(tag) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };

        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .tags()
                    .any(|candidate| candidate == tag)
            })
            .collect()
    }

    /// Returns stable passes intended for automatic selection.
    pub fn automatic_passes(
        &self,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                let descriptor = registration.descriptor();

                descriptor.enabled_by_default()
                    && descriptor.is_stable()
                    && descriptor.replacement().is_none()
            })
            .collect()
    }

    /// Returns deprecated passes.
    pub fn deprecated_passes(
        &self,
    ) -> Vec<&PassRegistration> {
        self.registrations
            .values()
            .filter(|registration| {
                registration
                    .descriptor()
                    .replacement()
                    .is_some()
            })
            .collect()
    }

    /// Validates all registry invariants.
    pub fn validate(&self) -> Result<(), RegistryError> {
        if self.registrations.len() > self.limits.max_passes {
            return Err(
                RegistryError::RegistryInvariantViolation {
                    message:
                        "pass count exceeds configured registry limit",
                },
            );
        }

        if self.aliases.len() > self.limits.max_aliases {
            return Err(
                RegistryError::RegistryInvariantViolation {
                    message:
                        "alias count exceeds configured registry limit",
                },
            );
        }

        for (alias, target) in &self.aliases {
            if !self.registrations.contains_key(target) {
                return Err(
                    RegistryError::BrokenAlias {
                        alias: alias.clone(),
                        target: target.clone(),
                    },
                );
            }

            if self.registrations.contains_key(
                &PassId(alias.clone()),
            ) {
                return Err(
                    RegistryError::RegistryInvariantViolation {
                        message:
                            "alias collides with a canonical pass ID",
                    },
                );
            }
        }

        for registration in self.registrations.values() {
            let descriptor = registration.descriptor();

            for alias in descriptor.aliases() {
                match self.aliases.get(alias) {
                    Some(target)
                        if target == descriptor.id() => {}

                    Some(_) => {
                        return Err(
                            RegistryError::AliasConflict {
                                alias: alias.to_string(),
                                requested: descriptor.id().clone(),
                                existing: self.aliases
                                    .get(alias)
                                    .expect(
                                        "alias was checked above",
                                    )
                                    .clone(),
                            },
                        );
                    }

                    None => {
                        return Err(
                            RegistryError::BrokenAlias {
                                alias: alias.to_string(),
                                target: descriptor.id().clone(),
                            },
                        );
                    }
                }
            }

            if let Some(replacement) =
                descriptor.replacement()
            {
                if !self.registrations.contains_key(replacement) {
                    return Err(
                        RegistryError::MissingReplacement {
                            pass: descriptor.id().clone(),
                            replacement: replacement.clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Creates an immutable snapshot.
    pub fn snapshot(&self) -> PassRegistrySnapshot {
        PassRegistrySnapshot {
            registrations: self.registrations.clone(),
            aliases: self.aliases.clone(),
        }
    }

    /// Clears all registrations.
    ///
    /// Intended primarily for isolated compiler instances and tests.
    pub fn clear(&mut self) {
        self.registrations.clear();
        self.aliases.clear();
    }

    /// Creates a registry containing the supplied registrations.
    pub fn from_registrations<I>(
        registrations: I,
    ) -> Result<Self, RegistryError>
    where
        I: IntoIterator<Item = PassRegistration>,
    {
        let mut registry = Self::new();
        registry.register_all(registrations)?;
        Ok(registry)
    }

    fn validate_aliases_for_registration(
        &self,
        descriptor: &PassDescriptor,
    ) -> Result<(), RegistryError> {
        let aliases = descriptor
            .aliases()
            .collect::<Vec<_>>();

        let projected_alias_count = self
            .aliases
            .len()
            .checked_add(aliases.len())
            .ok_or(
                RegistryError::AliasCapacityExceeded {
                    maximum: self.limits.max_aliases,
                },
            )?;

        if projected_alias_count
            > self.limits.max_aliases
        {
            return Err(
                RegistryError::AliasCapacityExceeded {
                    maximum: self.limits.max_aliases,
                },
            );
        }

        for alias in aliases {
            if alias == descriptor.id().as_str() {
                return Err(
                    RegistryError::AliasEqualsCanonicalId {
                        pass: descriptor.id().clone(),
                        alias: alias.to_string(),
                    },
                );
            }

            if self.registrations.keys().any(|id| {
                id.as_str() == alias
            }) {
                let existing = self
                    .registrations
                    .keys()
                    .find(|id| id.as_str() == alias)
                    .expect("canonical ID was found above")
                    .clone();

                return Err(
                    RegistryError::AliasConflict {
                        alias: alias.to_string(),
                        requested: descriptor.id().clone(),
                        existing,
                    },
                );
            }

            if let Some(existing) = self.aliases.get(alias) {
                return Err(
                    RegistryError::AliasConflict {
                        alias: alias.to_string(),
                        requested: descriptor.id().clone(),
                        existing: existing.clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Built-in registration helpers
// =============================================================================

/// Registers a single pass with a mutable registry.
///
/// This helper exists to make subsystem registration concise while keeping
/// registration explicit and testable.
pub fn register_pass(
    registry: &mut PassRegistry,
    descriptor: PassDescriptor,
    factory: PassFactory,
) -> Result<(), RegistryError> {
    registry.register(
        PassRegistration::new(
            descriptor,
            factory,
        ),
    )
}

/// Registers multiple passes atomically.
pub fn register_passes<I>(
    registry: &mut PassRegistry,
    registrations: I,
) -> Result<(), RegistryError>
where
    I: IntoIterator<Item = PassRegistration>,
{
    registry.register_all(registrations)
}

// =============================================================================
// Registry query
// =============================================================================

/// Declarative registry query.
///
/// This is intentionally simple enough to remain stable while the optimizer
/// grows.
#[derive(Debug, Clone, Default)]
pub struct PassQuery {
    category: Option<PassCategory>,
    capability: Option<PassCapability>,
    target: Option<PassTarget>,
    tag: Option<String>,
    stable_only: bool,
    automatic_only: bool,
    include_deprecated: bool,
}

impl PassQuery {
    /// Creates an empty query.
    pub const fn new() -> Self {
        Self {
            category: None,
            capability: None,
            target: None,
            tag: None,
            stable_only: false,
            automatic_only: false,
            include_deprecated: false,
        }
    }

    /// Restricts the query to one category.
    pub const fn category(
        mut self,
        category: PassCategory,
    ) -> Self {
        self.category = Some(category);
        self
    }

    /// Restricts the query to one capability.
    pub const fn capability(
        mut self,
        capability: PassCapability,
    ) -> Self {
        self.capability = Some(capability);
        self
    }

    /// Restricts the query to one target.
    pub const fn target(
        mut self,
        target: PassTarget,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Restricts the query to a tag.
    pub fn tag(
        mut self,
        tag: impl Into<String>,
    ) -> Result<Self, RegistryError> {
        self.tag = Some(normalize_tag(&tag.into())?);
        Ok(self)
    }

    /// Requires stable passes.
    pub const fn stable_only(
        mut self,
    ) -> Self {
        self.stable_only = true;
        self
    }

    /// Requires automatic/default passes.
    pub const fn automatic_only(
        mut self,
    ) -> Self {
        self.automatic_only = true;
        self
    }

    /// Allows deprecated/replaced passes.
    pub const fn include_deprecated(
        mut self,
    ) -> Self {
        self.include_deprecated = true;
        self
    }

    /// Executes the query.
    pub fn execute<'a>(
        &self,
        registry: &'a PassRegistry,
    ) -> Vec<&'a PassRegistration> {
        registry
            .registrations()
            .filter(|registration| {
                let descriptor = registration.descriptor();

                if let Some(category) = self.category {
                    if descriptor.category() != category {
                        return false;
                    }
                }

                if let Some(capability) = self.capability {
                    if !descriptor.has_capability(capability) {
                        return false;
                    }
                }

                if let Some(target) = self.target {
                    if !descriptor.supports_target(target) {
                        return false;
                    }
                }

                if let Some(tag) = &self.tag {
                    if !descriptor
                        .tags()
                        .any(|candidate| candidate == tag)
                    {
                        return false;
                    }
                }

                if self.stable_only
                    && !descriptor.is_stable()
                {
                    return false;
                }

                if self.automatic_only
                    && !descriptor.enabled_by_default()
                {
                    return false;
                }

                if !self.include_deprecated
                    && descriptor.replacement().is_some()
                {
                    return false;
                }

                true
            })
            .collect()
    }
}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> Result<(), RegistryError> {
    if value.is_empty()
        || value.len() > maximum
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
    {
        return Err(
            RegistryError::InvalidIdentifier {
                field,
                value: value.to_string(),
                maximum,
            },
        );
    }

    if value.starts_with('.')
        || value.ends_with('.')
        || value.contains("..")
    {
        return Err(
            RegistryError::InvalidIdentifier {
                field,
                value: value.to_string(),
                maximum,
            },
        );
    }

    if !value
        .chars()
        .all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '_' | '-' | '.'
                )
        })
    {
        return Err(
            RegistryError::InvalidIdentifier {
                field,
                value: value.to_string(),
                maximum,
            },
        );
    }

    Ok(())
}

fn validate_text(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> Result<(), RegistryError> {
    if value.is_empty()
        || value.len() > maximum
        || value.trim() != value
    {
        return Err(
            RegistryError::InvalidText {
                field,
                maximum,
            },
        );
    }

    Ok(())
}

fn normalize_alias(
    value: &str,
) -> Result<String, RegistryError> {
    let normalized = value
        .trim()
        .to_ascii_lowercase();

    validate_identifier(
        &normalized,
        MAX_PASS_ALIAS_LENGTH,
        "pass alias",
    )
    .map_err(|_| RegistryError::InvalidAlias {
        value: value.to_string(),
    })?;

    Ok(normalized)
}

fn normalize_tag(
    value: &str,
) -> Result<String, RegistryError> {
    let normalized = value
        .trim()
        .to_ascii_lowercase();

    if normalized.is_empty()
        || normalized.len() > MAX_PASS_ALIAS_LENGTH
        || normalized.contains(char::is_whitespace)
        || !normalized.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '_' | '-' | '.'
                )
        })
    {
        return Err(
            RegistryError::InvalidTag {
                value: value.to_string(),
            },
        );
    }

    Ok(normalized)
}

fn normalize_lookup_name(
    value: &str,
) -> Result<String, RegistryError> {
    let normalized = value
        .trim()
        .to_ascii_lowercase();

    validate_identifier(
        &normalized,
        MAX_PASS_ID_LENGTH,
        "pass lookup name",
    )?;

    Ok(normalized)
}

fn resolve_name<'a>(
    name: &str,
    registrations: &'a BTreeMap<
        PassId,
        PassRegistration,
    >,
    aliases: &'a BTreeMap<String, PassId>,
) -> Option<&'a PassId> {
    let normalized = normalize_lookup_name(name).ok()?;

    if let Some(id) = aliases.get(&normalized) {
        return Some(id);
    }

    registrations
        .keys()
        .find(|id| id.as_str() == normalized)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test pass
    // -------------------------------------------------------------------------

    #[derive(Debug, Default)]
    struct TestPass;

    impl OptimizationPass for TestPass {
        // The exact execution contract belongs to pass.rs.
        //
        // The registry deliberately tests only object construction and
        // registration. This keeps registry tests independent from the
        // implementation of circuit transformation.
    }

    fn test_factory() -> PassFactory {
        PassFactory::from_fn(|| {
            Box::new(TestPass)
        })
    }

    fn descriptor(
        id: &'static str,
    ) -> PassDescriptor {
        PassDescriptor::new(
            PassId::from_static(id),
            id,
            PassVersion::new(1, 0, 0),
            "Test optimization pass.",
        )
        .expect("test descriptor should be valid")
    }

    fn registration(
        id: &'static str,
    ) -> PassRegistration {
        PassRegistration::new(
            descriptor(id),
            test_factory(),
        )
    }

    // -------------------------------------------------------------------------
    // Pass IDs
    // -------------------------------------------------------------------------

    #[test]
    fn pass_id_accepts_namespaced_identifier() {
        let id = PassId::new(
            "local.cancellation",
        )
        .expect("ID should be valid");

        assert_eq!(
            id.as_str(),
            "local.cancellation"
        );

        assert_eq!(
            id.namespace(),
            Some("local")
        );

        assert_eq!(
            id.local_name(),
            "cancellation"
        );
    }

    #[test]
    fn pass_id_rejects_empty_identifier() {
        assert!(
            PassId::new("").is_err()
        );
    }

    #[test]
    fn pass_id_rejects_whitespace() {
        assert!(
            PassId::new("local cancellation").is_err()
        );
    }

    #[test]
    fn pass_id_rejects_double_dot() {
        assert!(
            PassId::new("local..cancellation")
                .is_err()
        );
    }

    // -------------------------------------------------------------------------
    // Descriptor
    // -------------------------------------------------------------------------

    #[test]
    fn descriptor_defaults_to_generic_target() {
        let descriptor =
            descriptor("local.test");

        assert!(
            descriptor.supports_target(
                PassTarget::Generic
            )
        );

        assert_eq!(
            descriptor.category(),
            PassCategory::Other
        );
    }

    #[test]
    fn descriptor_supports_builder_methods() {
        let descriptor =
            descriptor("local.test")
                .with_category(
                    PassCategory::Local,
                )
                .with_capability(
                    PassCapability::MayRemoveOperations,
                )
                .with_target(
                    PassTarget::CliffordT,
                )
                .with_alias(
                    "test",
                )
                .expect("alias should be valid")
                .with_tag(
                    "unit-test",
                )
                .expect("tag should be valid")
                .stable()
                .enabled_by_default();

        assert_eq!(
            descriptor.category(),
            PassCategory::Local
        );

        assert!(
            descriptor.has_capability(
                PassCapability::MayRemoveOperations
            )
        );

        assert!(
            descriptor.supports_target(
                PassTarget::CliffordT
            )
        );

        assert!(
            descriptor.aliases().any(
                |alias| alias == "test"
            )
        );

        assert!(
            descriptor.tags().any(
                |tag| tag == "unit-test"
            )
        );

        assert!(
            descriptor.is_stable()
        );

        assert!(
            descriptor.enabled_by_default()
        );
    }

    // -------------------------------------------------------------------------
    // Registration
    // -------------------------------------------------------------------------

    #[test]
    fn registers_pass() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.test",
                ),
            )
            .expect("registration should succeed");

        assert_eq!(
            registry.len(),
            1
        );

        assert!(
            registry.contains(
                "local.test"
            )
        );
    }

    #[test]
    fn rejects_duplicate_pass() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.test",
                ),
            )
            .expect("first registration should succeed");

        let error = registry
            .register(
                registration(
                    "local.test",
                ),
            )
            .expect_err(
                "duplicate should fail",
            );

        assert_eq!(
            error,
            RegistryError::DuplicatePass {
                id: PassId::from_static(
                    "local.test"
                ),
            }
        );
    }

    #[test]
    fn aliases_resolve_to_canonical_id() {
        let descriptor =
            descriptor(
                "local.cancellation",
            )
            .with_alias(
                "cancellation",
            )
            .expect("alias should be valid");

        let mut registry =
            PassRegistry::new();

        registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect("registration should succeed");

        assert!(
            registry.contains(
                "cancellation"
            )
        );

        assert_eq!(
            registry
                .resolve_id(
                    "cancellation"
                )
                .expect("alias should resolve")
                .as_str(),
            "local.cancellation"
        );
    }

    #[test]
    fn rejects_alias_collision() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.first",
                ),
            )
            .expect("registration should succeed");

        let descriptor =
            descriptor(
                "local.second",
            )
            .with_alias(
                "local.first",
            )
            .expect("descriptor construction itself is valid");

        let error = registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect_err(
                "alias collision should fail",
            );

        assert!(matches!(
            error,
            RegistryError::AliasConflict { .. }
        ));

        assert!(
            registry.contains(
                "local.first"
            )
        );

        assert!(
            !registry.contains(
                "local.second"
            )
        );
    }

    #[test]
    fn failed_registration_is_atomic() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.first",
                ),
            )
            .expect("registration should succeed");

        let descriptor =
            descriptor(
                "local.second",
            )
            .with_alias(
                "local.first",
            )
            .expect("descriptor should be constructible");

        assert!(
            registry
                .register(
                    PassRegistration::new(
                        descriptor,
                        test_factory(),
                    ),
                )
                .is_err()
        );

        assert_eq!(
            registry.len(),
            1
        );

        assert!(
            !registry.contains(
                "local.second"
            )
        );
    }

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    #[test]
    fn constructs_fresh_pass_instances() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.test",
                ),
            )
            .expect("registration should succeed");

        let first = registry
            .construct(
                "local.test",
            )
            .expect("pass should construct");

        let second = registry
            .construct(
                "local.test",
            )
            .expect("pass should construct");

        assert_eq!(
            std::any::type_name_of_val(
                &*first
            ),
            std::any::type_name_of_val(
                &*second
            )
        );
    }

    // -------------------------------------------------------------------------
    // Queries
    // -------------------------------------------------------------------------

    #[test]
    fn query_filters_by_category() {
        let mut registry =
            PassRegistry::new();

        let descriptor =
            descriptor(
                "local.test",
            )
            .with_category(
                PassCategory::Local,
            );

        registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect("registration should succeed");

        let results =
            PassQuery::new()
                .category(
                    PassCategory::Local,
                )
                .execute(
                    &registry,
                );

        assert_eq!(
            results.len(),
            1
        );
    }

    #[test]
    fn query_filters_by_capability() {
        let mut registry =
            PassRegistry::new();

        let descriptor =
            descriptor(
                "local.test",
            )
            .with_capability(
                PassCapability::MayRemoveOperations,
            );

        registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect("registration should succeed");

        let results =
            PassQuery::new()
                .capability(
                    PassCapability::MayRemoveOperations,
                )
                .execute(
                    &registry,
                );

        assert_eq!(
            results.len(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Snapshot
    // -------------------------------------------------------------------------

    #[test]
    fn snapshot_is_stable() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "local.test",
                ),
            )
            .expect("registration should succeed");

        let snapshot =
            registry.snapshot();

        registry.clear();

        assert_eq!(
            snapshot.len(),
            1
        );

        assert!(
            snapshot.contains(
                "local.test"
            )
        );
    }

    // -------------------------------------------------------------------------
    // Unregister
    // -------------------------------------------------------------------------

    #[test]
    fn unregister_removes_aliases() {
        let descriptor =
            descriptor(
                "local.test",
            )
            .with_alias(
                "test",
            )
            .expect("alias should be valid");

        let mut registry =
            PassRegistry::new();

        registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect("registration should succeed");

        assert!(
            registry.contains(
                "test"
            )
        );

        registry
            .unregister(
                "local.test",
            )
            .expect("unregister should succeed");

        assert!(
            !registry.contains(
                "local.test"
            )
        );

        assert!(
            !registry.contains(
                "test"
            )
        );
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    #[test]
    fn valid_registry_passes_validation() {
        let descriptor =
            descriptor(
                "local.test",
            )
            .with_alias(
                "test",
            )
            .expect("alias should be valid");

        let mut registry =
            PassRegistry::new();

        registry
            .register(
                PassRegistration::new(
                    descriptor,
                    test_factory(),
                ),
            )
            .expect("registration should succeed");

        registry
            .validate()
            .expect("registry should be valid");
    }

    // -------------------------------------------------------------------------
    // Deterministic ordering
    // -------------------------------------------------------------------------

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry =
            PassRegistry::new();

        registry
            .register(
                registration(
                    "z.last",
                ),
            )
            .expect("registration should succeed");

        registry
            .register(
                registration(
                    "a.first",
                ),
            )
            .expect("registration should succeed");

        registry
            .register(
                registration(
                    "m.middle",
                ),
            )
            .expect("registration should succeed");

        let ids = registry
            .pass_ids()
            .map(PassId::as_str)
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                "a.first",
                "m.middle",
                "z.last",
            ]
        );
    }

    // -------------------------------------------------------------------------
    // Atomic batch registration
    // -------------------------------------------------------------------------

    #[test]
    fn batch_registration_is_atomic() {
        let mut registry =
            PassRegistry::new();

        let registrations = vec![
            registration(
                "local.first",
            ),
            registration(
                "local.second",
            ),
            registration(
                "local.first",
            ),
        ];

        assert!(
            registry
                .register_all(
                    registrations,
                )
                .is_err()
        );

        assert!(
            registry.is_empty()
        );
    }

    // -------------------------------------------------------------------------
    // Capacity
    // -------------------------------------------------------------------------

    #[test]
    fn registry_respects_capacity() {
        let limits =
            RegistryLimits::new(
                1,
                10,
            );

        let mut registry =
            PassRegistry::with_limits(
                limits,
            )
            .expect("limits should be valid");

        registry
            .register(
                registration(
                    "local.first",
                ),
            )
            .expect("first registration should succeed");

        let error = registry
            .register(
                registration(
                    "local.second",
                ),
            )
            .expect_err(
                "second registration should exceed capacity",
            );

        assert_eq!(
            error,
            RegistryError::RegistryCapacityExceeded {
                maximum: 1,
            }
        );
    }

    // -------------------------------------------------------------------------
    // Deprecation
    // -------------------------------------------------------------------------

    #[test]
    fn deprecated_pass_is_not_automatic() {
        let replacement =
            PassId::from_static(
                "local.new",
            );

        let descriptor =
            descriptor(
                "local.old",
            )
            .replaced_by(
                replacement,
            );

        assert!(
            !descriptor.enabled_by_default()
        );

        assert!(
            descriptor.replacement().is_some()
        );
    }

    // -------------------------------------------------------------------------
    // Version
    // -------------------------------------------------------------------------

    #[test]
    fn pass_version_formats_stably() {
        let version =
            PassVersion::new(
                2,
                4,
                7,
            );

        assert_eq!(
            version.as_string(),
            "2.4.7"
        );

        assert_eq!(
            version.to_string(),
            "2.4.7"
        );
    }
}