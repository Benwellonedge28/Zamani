//! Zamani Quantum Noise (ZQN) — Canonical Quantum IR Integration.
//!
//! This module is the narrow integration boundary between:
//!
//!     crate::quantum::ir
//!
//! and:
//!
//!     crate::quantum::zqn
//!
//! # Mission
//!
//! This file answers:
//!
//! > "Which ZQN semantic objects are associated with which canonical Quantum
//! > IR operations and resources?"
//!
//! It does NOT redefine quantum IR semantics.
//!
//! It does NOT define another `Operation`.
//!
//! It does NOT define another `QubitId`.
//!
//! It does NOT define another `PhysicalQubitId`.
//!
//! It does NOT execute channels.
//!
//! It does NOT generate faults.
//!
//! It does NOT perform routing.
//!
//! It does NOT perform scheduling.
//!
//! It does NOT perform QEC.
//!
//! It does NOT contact hardware.
//!
//! It does NOT simulate quantum states.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani source
//!                         │
//!                         ▼
//!                  quantum frontend
//!                         │
//!                         ▼
//!                 ┌───────────────┐
//!                 │ quantum::ir   │
//!                 │ canonical     │
//!                 │ semantic IR   │
//!                 └───────┬───────┘
//!                         │
//!                         │ OperationId / QubitId / PhysicalQubitId
//!                         │
//!                         ▼
//!                 ┌───────────────┐
//!                 │ ZQN IR        │
//!                 │ integration   │
//!                 │               │
//!                 │ this module   │
//!                 └───────┬───────┘
//!                         │
//!              ┌──────────┼────────────┐
//!              │          │            │
//!              ▼          ▼            ▼
//!            noise      faults       channels
//!              │          │            │
//!              └──────────┼────────────┘
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!       routing       scheduling         QEC
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                       target
//!                         │
//!                         ▼
//!                      runtime
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - ZQN-to-IR association values;
//! - operation-level ZQN bindings;
//! - resource-level ZQN bindings;
//! - deterministic binding ordering;
//! - binding validation;
//! - duplicate-binding policy;
//! - immutable inspection;
//! - replacement/removal of explicitly identified bindings;
//! - conversion-free use of canonical IR identities;
//! - an IR-independent association index;
//! - validation of references to a supplied set of canonical operation IDs;
//! - validation of canonical logical/physical resource identity relationships;
//! - deterministic fingerprints of the association structure;
//! - resource-policy-independent semantic representation.
//!
//! # Non-ownership
//!
//! This module does NOT own:
//!
//! - `Operation`;
//! - `OperationBody`;
//! - `OperationId`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - gate semantics;
//! - measurement semantics;
//! - circuit semantics;
//! - noise-model semantics;
//! - channel mathematics;
//! - fault semantics;
//! - probability mathematics;
//! - calibration;
//! - characterization;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - execution;
//! - serialization wire formats;
//! - canonical IR hashing;
//! - global identity allocation.
//!
//! The canonical Quantum IR remains the source of truth for quantum program
//! semantics. The ZQN identity module remains the source of truth for ZQN
//! object identities.
//!
//! # Critical single-source-of-truth rule
//!
//! Quantum resource identity MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Specifically:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file MUST NOT define:
//!
//! ```text
//! ZqnQubitId
//! ZqnPhysicalQubitId
//! NoiseQubitId
//! PhysicalResourceId
//! ```
//!
//! merely as replacements for the canonical IR resource identities.
//!
//! ZQN-specific object identities such as `NoiseModelId` remain owned by:
//!
//! ```text
//! crate::quantum::zqn::core::ids
//! ```
//!
//! This separation is already established by the repository's ZQN identity
//! contract.
//!
//! # Why this is an association layer rather than a "noisy IR"
//!
//! The canonical IR answers:
//!
//!     WHAT does the quantum program mean?
//!
//! ZQN answers:
//!
//!     WHAT physical uncertainty/noise/fault semantics affect it?
//!
//! Therefore the integration must not mutate or replace canonical IR
//! operations merely to attach noise.
//!
//! The relationship is:
//!
//! ```text
//! canonical Operation
//!        │
//!        │ OperationId
//!        ▼
//! ZQN association
//!        │
//!        ├── NoiseModelId
//!        ├── resource scope
//!        ├── optional physical scope
//!        └── semantic metadata
//! ```
//!
//! The operation itself remains owned by `quantum::ir`.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic machine-size ceiling in this module.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_BINDINGS
//! MAX_RESOURCES
//! ```
//!
//! Association storage grows according to the actual number of associations
//! requested by the caller and the resources available to the host.
//!
//! A selector can be used instead of enumerating every resource when a later
//! ZQN layer supports selector resolution.
//!
//! "Infinity" means:
//!
//! > ZQN does not encode an artificial finite quantum-machine size limit.
//!
//! It does not mean that a finite machine can allocate infinite memory.
//!
//! Resource/security limits belong to the explicit ZQN execution/resource
//! policy, not to this semantic integration representation.
//!
//! # No dependence on concrete Operation internals
//!
//! This module intentionally associates bindings by `OperationId` rather than
//! depending on the private structure of `Operation`.
//!
//! This is important because the canonical IR can evolve its operation
//! representation without requiring the ZQN integration layer to be rewritten.
//!
//! The canonical IR already establishes `OperationId` as the stable identity of
//! an operation.
//!
//! Consequently:
//!
//! ```text
//! Operation implementation changes
//!             │
//!             ▼
//! OperationId contract remains stable
//!             │
//!             ▼
//! ZQN integration remains stable
//! ```
//!
//! # Dependency direction
//!
//! Allowed:
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ▼
//! quantum::zqn::integration::ir
//!          │
//!          ├── routing
//!          ├── scheduling
//!          ├── qec
//!          ├── hardware
//!          ├── simulation
//!          └── benchmarking
//! ```
//!
//! This file must NOT import those downstream consumers.
//!
//! In particular, it must not depend on:
//!
//! ```text
//! zqn::simulation
//! zqn::target
//! zqn::calibration
//! quantum::routing
//! quantum::scheduling
//! quantum::error_correction
//! quantum::hardware
//! ```
//!
//! Doing so would turn a foundational integration boundary into a dependency
//! hub and make future integration changes unnecessarily circular.
//!
//! # Integration with `noise::model`
//!
//! `NoiseModelId` is referenced by value.
//!
//! This file does not resolve the ID to a `NoiseModel`.
//!
//! Resolution remains the responsibility of the ZQN model/registry layer.
//!
//! Therefore:
//!
//! ```text
//! IrNoiseBinding
//!       │
//!       └── NoiseModelId
//!                 │
//!                 ▼
//!           noise::model
//!                 │
//!                 ▼
//!             NoiseModel
//! ```
//!
//! This keeps this module independent from the concrete noise-model
//! implementation.
//!
//! # Integration with `noise::application`
//!
//! `noise::application` owns actual noise-application semantics.
//!
//! This module intentionally does not duplicate `NoiseApplication`.
//!
//! Instead, an application layer can use:
//!
//! ```text
//! IrNoiseBinding
//! ```
//!
//! to identify the canonical IR operation/resource to which an application is
//! attached.
//!
//! A future adapter may therefore perform:
//!
//! ```text
//! IrNoiseBinding
//!        │
//!        ▼
//! NoiseApplicationRequest
//!        │
//!        ▼
//! NoiseModel
//! ```
//!
//! without requiring this module to know the application implementation.
//!
//! # Integration with channels
//!
//! This module does not contain `ChannelId` bindings directly as a semantic
//! requirement. Channel selection belongs to the ZQN noise/application layer.
//!
//! A future consumer can associate channel IDs through a `NoiseModelId` or
//! application identity without modifying this file.
//!
//! This is deliberate: adding a new channel representation must not change
//! the IR integration contract.
//!
//! # Integration with faults
//!
//! Faults are not embedded into canonical IR operations here.
//!
//! A fault generator may use:
//!
//! ```text
//! OperationId
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! from this module's binding information to create a ZQN `Fault`.
//!
//! Fault semantics remain owned by `zqn::fault`.
//!
//! # Integration with routing
//!
//! Routing can consume operation-level and resource-level ZQN bindings to
//! determine noise-aware costs.
//!
//! Routing remains responsible for deciding physical placement.
//!
//! This module therefore never creates a logical-to-physical mapping.
//!
//! A binding may reference an already-known physical resource, but the binding
//! does not assert that the resource is actually available on a target.
//!
//! Hardware capability validation remains downstream.
//!
//! # Integration with scheduling
//!
//! Scheduling may use this association layer to determine which ZQN model
//! applies to an operation and therefore estimate time-dependent noise.
//!
//! Scheduling owns time.
//!
//! This module does not assign timestamps, durations, priorities, or execution
//! order.
//!
//! # Integration with QEC
//!
//! QEC may consume bindings and resolve them into physical fault semantics.
//!
//! The dependency direction remains:
//!
//! ```text
//! ZQN integration
//!       │
//!       ▼
//! QEC adapter
//!       │
//!       ▼
//! QEC physical fault model
//! ```
//!
//! This module does not know stabilizer codes, surface codes, decoders,
//! syndromes, logical corrections, or logical error models.
//!
//! # Integration with hardware
//!
//! A physical-resource binding does not establish hardware existence.
//!
//! For example:
//!
//! ```text
//! PhysicalQubitId::new(10_000)
//! ```
//!
//! only identifies a physical resource in the canonical identity domain.
//!
//! Whether that resource exists on a particular target is decided by target
//! capability/resource validation.
//!
//! # Integration with simulation
//!
//! Simulation can consume this layer to determine which ZQN semantics apply to
//! a canonical operation/resource.
//!
//! This file does not allocate quantum state, tensors, trajectories or
//! probability distributions.
//!
//! # Integration with benchmarking
//!
//! Benchmarking can group results by:
//!
//! - operation identity;
//! - ZQN model identity;
//! - resource scope;
//! - semantic binding identity.
//!
//! Benchmarking remains responsible for experimental methodology and metrics.
//!
//! # Determinism
//!
//! This module is fully deterministic.
//!
//! It contains:
//!
//! - no RNG;
//! - no clock access;
//! - no thread-local state;
//! - no process ID;
//! - no memory addresses;
//! - no global mutable state;
//! - no random identity generation;
//! - no unordered semantic operation storage.
//!
//! `BTreeMap` and `BTreeSet` are deliberately used for canonical deterministic
//! ordering.
//!
//! # Parallelism
//!
//! The primary integration structure is an owned value.
//!
//! Once constructed, immutable references can be shared by concurrent readers
//! according to normal Rust ownership rules.
//!
//! No `Send`, `Sync`, or other marker trait is implemented manually.
//!
//! # Resource safety
//!
//! This module never automatically expands a binding to all resources of a
//! target.
//!
//! A selector remains a selector until an explicit downstream resolver expands
//! it.
//!
//! This is critical for very large machines.
//!
//! For example:
//!
//! ```text
//! Selector("all physical resources")
//! ```
//!
//! remains one semantic selector rather than becoming millions/billions of
//! concrete entries merely because the binding was constructed.
//!
//! # Validation
//!
//! Validation is divided into two levels.
//!
//! Local validation:
//!
//! - operation IDs are valid values;
//! - logical/physical scopes are structurally valid;
//! - collections contain no duplicates where uniqueness is required;
//! - bindings have a valid model ID;
//! - metadata is valid;
//!
//! External/reference validation:
//!
//! - whether an operation ID exists in a particular circuit;
//! - whether a logical qubit exists in a particular program;
//! - whether a physical resource exists on a target;
//! - whether a model ID is registered;
//! - whether a target supports the referenced semantics.
//!
//! The latter cannot be performed here because this module deliberately does
//! not own those registries.
//!
//! # Serialization
//!
//! This file intentionally does not depend on `serde` or define a wire format.
//!
//! The future ZQN `io` subsystem owns serialization.
//!
//! Serialization must preserve:
//!
//! - binding identity;
//! - operation identity;
//! - model identity;
//! - resource scope;
//! - physical scope;
//! - ordering semantics;
//! - metadata;
//! - schema/version information supplied by the IO layer.
//!
//! Rust memory layout is not a wire-format contract.
//!
//! # Hashing/fingerprints
//!
//! This module provides a deterministic structural fingerprint using the
//! standard library's deterministic byte hashing primitive available through
//! `DefaultHasher`.
//!
//! The fingerprint is an in-process deterministic association fingerprint.
//!
//! It MUST NOT be treated as:
//!
//! - a cryptographic hash;
//! - a persistent object ID;
//! - a serialization format;
//! - a security credential.
//!
//! Canonical persistent content hashing remains owned by the repository's
//! canonical hashing subsystem.
//!
//! # Versioning
//!
//! The semantic version of ZQN is owned by:
//!
//! ```text
//! crate::quantum::zqn::core::version
//! ```
//!
//! This module does not create a second schema version.
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! Safety is compiler-enforced with:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! 1. canonical IR operation identity is used directly;
//! 2. canonical logical/physical qubit identity is used directly;
//! 3. no second operation or qubit model is introduced;
//! 4. ZQN model identity remains ZQN-owned;
//! 5. operation/resource associations are deterministic;
//! 6. associations do not require materializing an entire machine;
//! 7. invalid duplicate semantic bindings are rejected;
//! 8. external existence checks remain external;
//! 9. no vendor/backend dependency exists;
//! 10. no routing/scheduling/QEC semantics are duplicated;
//! 11. no unsafe code exists;
//! 12. the file can be stabilized independently of later ZQN modules;
//! 13. later ZQN modules can consume it without modifying its foundations;
//! 14. the same association model works for tiny and extremely large systems.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::ids::NoiseModelId;

// =============================================================================
// Result
// =============================================================================

/// Result type for canonical IR/ZQN integration operations.
pub type IrIntegrationResult<T> = Result<T, IrIntegrationError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the ZQN ↔ Quantum IR association layer.
///
/// The error type deliberately distinguishes:
///
/// - local structural errors;
/// - duplicate bindings;
/// - reference errors;
/// - capacity/policy errors.
///
/// It does not attempt to report backend-specific failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrIntegrationError {
    /// An operation identity was already bound according to the selected
    /// duplicate policy.
    DuplicateOperationBinding {
        /// The operation that already has a binding.
        operation: OperationId,
    },

    /// An operation identity does not exist in a supplied canonical operation
    /// namespace.
    UnknownOperation {
        /// Referenced operation.
        operation: OperationId,
    },

    /// A required model identity was not supplied.
    InvalidNoiseModelId,

    /// A resource selector was structurally invalid.
    InvalidSelector {
        /// Stable validation description.
        message: &'static str,
    },

    /// A resource collection contained a duplicate identity.
    DuplicateResource {
        /// The duplicated logical or physical resource.
        resource: ResourceIdentity,
    },

    /// A composite scope was empty.
    EmptyCompositeScope,

    /// An explicitly supplied resource binding is structurally inconsistent.
    InvalidResourceBinding {
        /// Stable validation description.
        message: &'static str,
    },

    /// An operation-binding collection exceeded an explicitly supplied caller
    /// policy.
    BindingLimitExceeded {
        /// Configured maximum.
        maximum: usize,

        /// Number requested.
        requested: usize,
    },

    /// A resource-binding collection exceeded an explicitly supplied caller
    /// policy.
    ResourceLimitExceeded {
        /// Configured maximum.
        maximum: usize,

        /// Number requested.
        requested: usize,
    },

    /// A supplied operation namespace contained duplicate identities.
    DuplicateOperationIdentity {
        /// Duplicated identity.
        operation: OperationId,
    },

    /// A caller attempted to associate a binding with an empty operation
    /// identity namespace.
    EmptyOperationNamespace,

    /// An externally supplied reference set was internally inconsistent.
    InvalidReferenceSet {
        /// Stable validation description.
        message: &'static str,
    },
}

impl fmt::Display for IrIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperationBinding { operation } => write!(
                formatter,
                "ZQN operation binding already exists for operation {operation}"
            ),

            Self::UnknownOperation { operation } => write!(
                formatter,
                "ZQN binding references unknown canonical IR operation {operation}"
            ),

            Self::InvalidNoiseModelId => {
                formatter.write_str("ZQN binding requires a valid noise-model identity")
            }

            Self::InvalidSelector { message } => {
                write!(formatter, "invalid ZQN IR resource selector: {message}")
            }

            Self::DuplicateResource { resource } => {
                write!(formatter, "duplicate resource in ZQN IR scope: {resource}")
            }

            Self::EmptyCompositeScope => {
                formatter.write_str("ZQN composite resource scope cannot be empty")
            }

            Self::InvalidResourceBinding { message } => {
                write!(formatter, "invalid ZQN IR resource binding: {message}")
            }

            Self::BindingLimitExceeded {
                maximum,
                requested,
            } => write!(
                formatter,
                "ZQN IR binding limit exceeded: requested {requested}, maximum {maximum}"
            ),

            Self::ResourceLimitExceeded {
                maximum,
                requested,
            } => write!(
                formatter,
                "ZQN IR resource limit exceeded: requested {requested}, maximum {maximum}"
            ),

            Self::DuplicateOperationIdentity { operation } => write!(
                formatter,
                "canonical IR operation namespace contains duplicate operation {operation}"
            ),

            Self::EmptyOperationNamespace => {
                formatter.write_str("canonical IR operation namespace is empty")
            }

            Self::InvalidReferenceSet { message } => {
                write!(formatter, "invalid canonical IR reference set: {message}")
            }
        }
    }
}

impl std::error::Error for IrIntegrationError {}

// =============================================================================
// Resource identity
// =============================================================================

/// Canonical quantum resource identity used by ZQN IR integration.
///
/// This enum does not introduce new resource identity types. Its variants
/// contain the canonical IR identities directly.
///
/// The distinction between logical and physical resources is intentional:
///
/// ```text
/// QubitId
///     logical semantic resource
///
/// PhysicalQubitId
///     physical resource identity
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceIdentity {
    /// Canonical logical qubit identity.
    LogicalQubit(QubitId),

    /// Canonical physical qubit identity.
    PhysicalQubit(PhysicalQubitId),
}

impl ResourceIdentity {
    /// Creates a logical-resource identity.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-resource identity.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Returns true when this is a logical resource.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true when this is a physical resource.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

impl fmt::Display for ResourceIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(qubit) => write!(formatter, "logical:{qubit}"),
            Self::PhysicalQubit(qubit) => write!(formatter, "physical:{qubit}"),
        }
    }
}

// =============================================================================
// Resource selector
// =============================================================================

/// Declarative selector for resources.
///
/// Selectors are intentionally not resolved in this module.
///
/// This permits a binding to remain compact even when the target contains a
/// very large number of resources.
///
/// The selector is semantic intent; target/resource resolution is downstream.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IrResourceSelector {
    /// Select every logical resource in the applicable execution scope.
    AllLogicalQubits,

    /// Select every physical resource in the applicable execution scope.
    AllPhysicalQubits,

    /// Select resources by a stable externally defined label.
    Label(String),

    /// Select resources by a stable namespace.
    Namespace(String),

    /// Select resources matching a stable user-defined expression.
    ///
    /// The expression is data, not executable code.
    Predicate(String),
}

impl IrResourceSelector {
    /// Creates a label selector.
    pub fn label<S>(label: S) -> IrIntegrationResult<Self>
    where
        S: Into<String>,
    {
        let value = label.into();

        if value.trim().is_empty() {
            return Err(IrIntegrationError::InvalidSelector {
                message: "resource label cannot be empty",
            });
        }

        Ok(Self::Label(value))
    }

    /// Creates a namespace selector.
    pub fn namespace<S>(namespace: S) -> IrIntegrationResult<Self>
    where
        S: Into<String>,
    {
        let value = namespace.into();

        if value.trim().is_empty() {
            return Err(IrIntegrationError::InvalidSelector {
                message: "resource namespace cannot be empty",
            });
        }

        Ok(Self::Namespace(value))
    }

    /// Creates an opaque declarative predicate.
    pub fn predicate<S>(predicate: S) -> IrIntegrationResult<Self>
    where
        S: Into<String>,
    {
        let value = predicate.into();

        if value.trim().is_empty() {
            return Err(IrIntegrationError::InvalidSelector {
                message: "resource predicate cannot be empty",
            });
        }

        Ok(Self::Predicate(value))
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Resource scope to which a ZQN binding applies.
///
/// This type deliberately supports both explicitly enumerated resources and
/// deferred selectors.
///
/// Explicit enumeration is useful for small or known subsets.
///
/// Selectors prevent an integration layer from being forced to materialize
/// enormous machines merely to express a semantic rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IrResourceScope {
    /// Applies to the entire applicable IR/execution scope.
    Global,

    /// One explicitly identified resource.
    Resource(ResourceIdentity),

    /// Multiple explicitly identified resources.
    Resources(BTreeSet<ResourceIdentity>),

    /// A deferred resource selector.
    Selector(IrResourceSelector),

    /// Multiple scopes that must all be considered by the downstream
    /// resolver.
    Composite(Vec<IrResourceScope>),
}

impl IrResourceScope {
    /// Creates a global scope.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates a single logical-qubit scope.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::Resource(ResourceIdentity::LogicalQubit(qubit))
    }

    /// Creates a single physical-qubit scope.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::Resource(ResourceIdentity::PhysicalQubit(qubit))
    }

    /// Creates an explicit resource scope.
    ///
    /// The input is canonicalized into deterministic set ordering.
    ///
    /// Duplicate resources are rejected rather than silently discarded.
    pub fn resources<I>(resources: I) -> IrIntegrationResult<Self>
    where
        I: IntoIterator<Item = ResourceIdentity>,
    {
        let mut set = BTreeSet::new();

        for resource in resources {
            if !set.insert(resource) {
                return Err(IrIntegrationError::DuplicateResource { resource });
            }
        }

        if set.is_empty() {
            return Err(IrIntegrationError::InvalidResourceBinding {
                message: "explicit resource scope cannot be empty",
            });
        }

        if set.len() == 1 {
            let resource = *set
                .first()
                .expect("a one-element BTreeSet contains a first element");

            return Ok(Self::Resource(resource));
        }

        Ok(Self::Resources(set))
    }

    /// Creates a composite scope.
    ///
    /// Empty composites are rejected because they have no semantic target.
    pub fn composite<I>(scopes: I) -> IrIntegrationResult<Self>
    where
        I: IntoIterator<Item = IrResourceScope>,
    {
        let values: Vec<_> = scopes.into_iter().collect();

        if values.is_empty() {
            return Err(IrIntegrationError::EmptyCompositeScope);
        }

        if values.iter().any(IrResourceScope::is_semantically_empty) {
            return Err(IrIntegrationError::InvalidResourceBinding {
                message: "composite scope contains an empty semantic scope",
            });
        }

        Ok(Self::Composite(values))
    }

    /// Creates a selector scope.
    #[must_use]
    pub const fn selector(selector: IrResourceSelector) -> Self {
        Self::Selector(selector)
    }

    /// Returns true if the scope has no semantic target.
    #[must_use]
    pub fn is_semantically_empty(&self) -> bool {
        match self {
            Self::Global => false,
            Self::Resource(_) => false,
            Self::Resources(resources) => resources.is_empty(),
            Self::Selector(_) => false,
            Self::Composite(scopes) => scopes.is_empty(),
        }
    }

    /// Returns true when this scope contains a logical resource.
    #[must_use]
    pub fn contains_logical_resources(&self) -> bool {
        match self {
            Self::Global => false,

            Self::Resource(resource) => resource.is_logical(),

            Self::Resources(resources) => {
                resources.iter().any(|resource| resource.is_logical())
            }

            Self::Selector(IrResourceSelector::AllLogicalQubits) => true,

            Self::Selector(_) => false,

            Self::Composite(scopes) => scopes
                .iter()
                .any(IrResourceScope::contains_logical_resources),
        }
    }

    /// Returns true when this scope contains a physical resource.
    #[must_use]
    pub fn contains_physical_resources(&self) -> bool {
        match self {
            Self::Global => false,

            Self::Resource(resource) => resource.is_physical(),

            Self::Resources(resources) => {
                resources.iter().any(|resource| resource.is_physical())
            }

            Self::Selector(IrResourceSelector::AllPhysicalQubits) => true,

            Self::Selector(_) => false,

            Self::Composite(scopes) => scopes
                .iter()
                .any(IrResourceScope::contains_physical_resources),
        }
    }

    /// Returns the number of explicitly represented resources.
    ///
    /// Deferred selectors and global scopes return zero because they have not
    /// been resolved.
    #[must_use]
    pub fn explicit_resource_count(&self) -> usize {
        match self {
            Self::Global | Self::Selector(_) => 0,

            Self::Resource(_) => 1,

            Self::Resources(resources) => resources.len(),

            Self::Composite(scopes) => scopes
                .iter()
                .map(IrResourceScope::explicit_resource_count)
                .sum(),
        }
    }

    /// Returns true if this scope contains the specified resource.
    ///
    /// Selector semantics are deliberately not resolved here and therefore
    /// return false unless the selector itself is an exact resource collection
    /// representation.
    #[must_use]
    pub fn contains_explicit(&self, resource: ResourceIdentity) -> bool {
        match self {
            Self::Global => false,

            Self::Resource(value) => *value == resource,

            Self::Resources(values) => values.contains(&resource),

            Self::Selector(_) => false,

            Self::Composite(scopes) => scopes
                .iter()
                .any(|scope| scope.contains_explicit(resource)),
        }
    }
}

// =============================================================================
// Operation scope
// =============================================================================

/// The canonical operation scope of a ZQN binding.
///
/// An operation binding always uses the canonical `OperationId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IrOperationRef {
    operation: OperationId,
}

impl IrOperationRef {
    /// Creates an operation reference from the canonical IR identity.
    #[must_use]
    pub const fn new(operation: OperationId) -> Self {
        Self { operation }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation
    }
}

impl From<OperationId> for IrOperationRef {
    fn from(operation: OperationId) -> Self {
        Self::new(operation)
    }
}

// =============================================================================
// Binding kind
// =============================================================================

/// Describes whether a ZQN binding is attached to an operation, resource, or
/// both.
///
/// The enum is intentionally small. It describes the association structure,
/// not the noise semantics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IrBindingScope {
    /// Applies to one canonical operation.
    Operation(IrOperationRef),

    /// Applies to an IR resource without requiring an operation.
    Resource(IrResourceScope),

    /// Applies when both an operation and resource scope match.
    OperationAndResource {
        /// Canonical operation.
        operation: IrOperationRef,

        /// Resource scope.
        resource: IrResourceScope,
    },
}

impl IrBindingScope {
    /// Returns the operation ID if this binding has one.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        match self {
            Self::Operation(operation) => Some(operation.operation_id()),

            Self::Resource(_) => None,

            Self::OperationAndResource { operation, .. } => {
                Some(operation.operation_id())
            }
        }
    }

    /// Returns the resource scope if this binding has one.
    #[must_use]
    pub fn resource_scope(&self) -> Option<&IrResourceScope> {
        match self {
            Self::Operation(_) => None,

            Self::Resource(resource) => Some(resource),

            Self::OperationAndResource { resource, .. } => Some(resource),
        }
    }

    /// Returns true if this binding targets an operation.
    #[must_use]
    pub const fn targets_operation(&self) -> bool {
        matches!(
            self,
            Self::Operation(_) | Self::OperationAndResource { .. }
        )
    }

    /// Returns true if this binding targets a resource.
    #[must_use]
    pub const fn targets_resource(&self) -> bool {
        matches!(
            self,
            Self::Resource(_) | Self::OperationAndResource { .. }
        )
    }
}

// =============================================================================
// Binding
// =============================================================================

/// Immutable association between canonical Quantum IR semantics and a ZQN
/// noise model.
///
/// This is deliberately NOT a `NoiseApplication`.
///
/// It identifies:
///
/// ```text
/// canonical IR location
///        │
///        ▼
/// ZQN NoiseModelId
/// ```
///
/// The model is resolved/executed elsewhere.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IrNoiseBinding {
    /// Canonical IR/ZQN association scope.
    scope: IrBindingScope,

    /// ZQN-owned noise-model identity.
    noise_model: NoiseModelId,

    /// Optional stable human/application-level label.
///
/// This is descriptive metadata, not semantic execution code.
    label: Option<String>,
}

impl IrNoiseBinding {
    /// Creates an operation-level noise binding.
    pub fn for_operation(
        operation: OperationId,
        noise_model: NoiseModelId,
    ) -> IrIntegrationResult<Self> {
        Self::new(
            IrBindingScope::Operation(IrOperationRef::new(operation)),
            noise_model,
        )
    }

    /// Creates a resource-level noise binding.
    pub fn for_resource(
        resource: IrResourceScope,
        noise_model: NoiseModelId,
    ) -> IrIntegrationResult<Self> {
        Self::new(IrBindingScope::Resource(resource), noise_model)
    }

    /// Creates an operation-and-resource binding.
    pub fn for_operation_and_resource(
        operation: OperationId,
        resource: IrResourceScope,
        noise_model: NoiseModelId,
    ) -> IrIntegrationResult<Self> {
        Self::new(
            IrBindingScope::OperationAndResource {
                operation: IrOperationRef::new(operation),
                resource,
            },
            noise_model,
        )
    }

    /// Creates a binding from an explicit scope.
    pub fn new(
        scope: IrBindingScope,
        noise_model: NoiseModelId,
    ) -> IrIntegrationResult<Self> {
        if scope
            .resource_scope()
            .is_some_and(IrResourceScope::is_semantically_empty)
        {
            return Err(IrIntegrationError::InvalidResourceBinding {
                message: "binding resource scope cannot be empty",
            });
        }

        Ok(Self {
            scope,
            noise_model,
            label: None,
        })
    }

    /// Sets an optional descriptive label.
    ///
    /// Empty labels are rejected.
    pub fn with_label<S>(mut self, label: S) -> IrIntegrationResult<Self>
    where
        S: Into<String>,
    {
        let label = label.into();

        if label.trim().is_empty() {
            return Err(IrIntegrationError::InvalidResourceBinding {
                message: "binding label cannot be empty",
            });
        }

        self.label = Some(label);
        Ok(self)
    }

    /// Returns the binding scope.
    #[must_use]
    pub const fn scope(&self) -> &IrBindingScope {
        &self.scope
    }

    /// Returns the canonical operation identity, if any.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        self.scope.operation_id()
    }

    /// Returns the associated ZQN noise-model identity.
    #[must_use]
    pub const fn noise_model_id(&self) -> NoiseModelId {
        self.noise_model
    }

    /// Returns the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns true when this binding targets an operation.
    #[must_use]
    pub const fn targets_operation(&self) -> bool {
        self.scope.targets_operation()
    }

    /// Returns true when this binding targets a resource.
    #[must_use]
    pub const fn targets_resource(&self) -> bool {
        self.scope.targets_resource()
    }

    /// Validates local semantic invariants.
    pub fn validate(&self) -> IrIntegrationResult<()> {
        if let Some(resource) = self.scope.resource_scope() {
            if resource.is_semantically_empty() {
                return Err(IrIntegrationError::InvalidResourceBinding {
                    message: "binding resource scope cannot be empty",
                });
            }
        }

        if self
            .label
            .as_deref()
            .is_some_and(|label| label.trim().is_empty())
        {
            return Err(IrIntegrationError::InvalidResourceBinding {
                message: "binding label cannot be empty",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Duplicate policy
// =============================================================================

/// Defines what the association container should do when an operation already
/// has a binding.
///
/// Resource-only bindings are not affected by this policy because they do not
/// belong to the operation map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationBindingPolicy {
    /// Reject a second operation binding.
    RejectDuplicate,

    /// Replace the previous operation binding atomically.
    Replace,

    /// Allow multiple operation bindings.
    ///
    /// This mode is represented by the secondary deterministic binding set.
    AllowMultiple,
}

impl Default for OperationBindingPolicy {
    fn default() -> Self {
        Self::RejectDuplicate
    }
}

// =============================================================================
// Integration set
// =============================================================================

/// Deterministic association set between canonical Quantum IR and ZQN.
///
/// `IrNoiseBindings` is the main public container of this integration module.
///
/// It does NOT contain canonical operations themselves.
///
/// Instead it contains stable identities and ZQN references.
///
/// This is deliberate because duplicating an entire canonical IR operation
/// here would violate ownership boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrNoiseBindings {
    /// One-to-one fast lookup for the common operation-binding case.
    operations: BTreeMap<OperationId, IrNoiseBinding>,

    /// Resource-only bindings.
    resources: BTreeSet<IrNoiseBinding>,

    /// Additional operation bindings used only when the caller explicitly
    /// chooses `AllowMultiple`.
    ///
    /// The primary map remains the canonical lookup for single bindings.
    additional_operation_bindings: BTreeSet<IrNoiseBinding>,

    /// Duplicate policy for operation bindings.
    operation_policy: OperationBindingPolicy,
}

impl Default for IrNoiseBindings {
    fn default() -> Self {
        Self::new()
    }
}

impl IrNoiseBindings {
    /// Creates an empty deterministic binding set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: BTreeMap::new(),
            resources: BTreeSet::new(),
            additional_operation_bindings: BTreeSet::new(),
            operation_policy: OperationBindingPolicy::RejectDuplicate,
        }
    }

    /// Creates an empty binding set with an explicit operation policy.
    #[must_use]
    pub fn with_operation_policy(policy: OperationBindingPolicy) -> Self {
        Self {
            operation_policy: policy,
            ..Self::new()
        }
    }

    /// Returns the operation-binding policy.
    #[must_use]
    pub const fn operation_policy(&self) -> OperationBindingPolicy {
        self.operation_policy
    }

    /// Returns the number of primary operation bindings.
    #[must_use]
    pub fn operation_binding_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of resource-only bindings.
    #[must_use]
    pub fn resource_binding_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of additional operation bindings.
    #[must_use]
    pub fn additional_operation_binding_count(&self) -> usize {
        self.additional_operation_bindings.len()
    }

    /// Returns the total number of stored bindings.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.operations
            .len()
            .saturating_add(self.resources.len())
            .saturating_add(self.additional_operation_bindings.len())
    }

    /// Returns true when no bindings exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
            && self.resources.is_empty()
            && self.additional_operation_bindings.is_empty()
    }

    /// Adds a binding atomically.
    ///
    /// If validation fails, the container is not modified.
    pub fn insert(&mut self, binding: IrNoiseBinding) -> IrIntegrationResult<()> {
        binding.validate()?;

        if let Some(operation) = binding.operation_id() {
            match self.operation_policy {
                OperationBindingPolicy::RejectDuplicate => {
                    if self.operations.contains_key(&operation) {
                        return Err(
                            IrIntegrationError::DuplicateOperationBinding {
                                operation,
                            },
                        );
                    }

                    self.operations.insert(operation, binding);
                    Ok(())
                }

                OperationBindingPolicy::Replace => {
                    self.operations.insert(operation, binding);
                    Ok(())
                }

                OperationBindingPolicy::AllowMultiple => {
                    if let Some(existing) = self.operations.get(&operation) {
                        if existing == &binding {
                            return Ok(());
                        }
                    }

                    if self.operations.contains_key(&operation) {
                        self.additional_operation_bindings.insert(binding);
                    } else {
                        self.operations.insert(operation, binding);
                    }

                    Ok(())
                }
            }
        } else {
            self.resources.insert(binding);
            Ok(())
        }
    }

    /// Inserts a binding and applies an explicit maximum binding policy.
    ///
    /// The limit is supplied by the caller rather than being encoded in ZQN.
    pub fn insert_with_limit(
        &mut self,
        binding: IrNoiseBinding,
        maximum_bindings: usize,
    ) -> IrIntegrationResult<()> {
        let current = self.binding_count();

        if current >= maximum_bindings {
            return Err(IrIntegrationError::BindingLimitExceeded {
                maximum: maximum_bindings,
                requested: current.saturating_add(1),
            });
        }

        self.insert(binding)
    }

    /// Inserts a collection atomically.
    ///
    /// If any binding fails validation or duplicate checking, no binding from
    /// the collection is committed.
    pub fn extend<I>(&mut self, bindings: I) -> IrIntegrationResult<()>
    where
        I: IntoIterator<Item = IrNoiseBinding>,
    {
        let mut candidate = self.clone();

        for binding in bindings {
            candidate.insert(binding)?;
        }

        *self = candidate;
        Ok(())
    }

    /// Returns the primary binding for an operation.
    #[must_use]
    pub fn get_operation(
        &self,
        operation: OperationId,
    ) -> Option<&IrNoiseBinding> {
        self.operations.get(&operation)
    }

    /// Returns all resource-only bindings in deterministic order.
    #[must_use]
    pub fn resource_bindings(
        &self,
    ) -> impl Iterator<Item = &IrNoiseBinding> {
        self.resources.iter()
    }

    /// Returns all primary operation bindings in deterministic operation-ID
    /// order.
    #[must_use]
    pub fn operation_bindings(
        &self,
    ) -> impl Iterator<Item = &IrNoiseBinding> {
        self.operations.values()
    }

    /// Returns all additional operation bindings in deterministic order.
    #[must_use]
    pub fn additional_operation_bindings(
        &self,
    ) -> impl Iterator<Item = &IrNoiseBinding> {
        self.additional_operation_bindings.iter()
    }

    /// Returns every binding in one deterministic ordering.
    ///
    /// The returned vector is newly allocated by the caller-facing inspection
    /// operation. Internal storage remains ordered and does not require this
    /// allocation.
    #[must_use]
    pub fn bindings(&self) -> Vec<&IrNoiseBinding> {
        let mut result = Vec::with_capacity(self.binding_count());

        result.extend(self.operations.values());
        result.extend(self.additional_operation_bindings.iter());
        result.extend(self.resources.iter());

        result.sort();

        result
    }

    /// Removes the primary operation binding.
    ///
    /// Returns the removed binding if one existed.
    pub fn remove_operation(
        &mut self,
        operation: OperationId,
    ) -> Option<IrNoiseBinding> {
        self.operations.remove(&operation)
    }

    /// Removes a resource-only binding.
    ///
    /// Returns true if the binding existed.
    pub fn remove_resource(&mut self, binding: &IrNoiseBinding) -> bool {
        self.resources.remove(binding)
    }

    /// Removes all bindings associated with an operation.
    ///
    /// This includes the primary operation binding and any additional bindings
    /// created under `AllowMultiple`.
    pub fn remove_all_for_operation(
        &mut self,
        operation: OperationId,
    ) -> Vec<IrNoiseBinding> {
        let mut removed = Vec::new();

        if let Some(binding) = self.operations.remove(&operation) {
            removed.push(binding);
        }

        let additional: Vec<_> = self
            .additional_operation_bindings
            .iter()
            .filter(|binding| binding.operation_id() == Some(operation))
            .cloned()
            .collect();

        for binding in additional {
            if self.additional_operation_bindings.remove(&binding) {
                removed.push(binding);
            }
        }

        removed.sort();

        removed
    }

    /// Validates all bindings contained in this structure.
    pub fn validate(&self) -> IrIntegrationResult<()> {
        for binding in self.operations.values() {
            binding.validate()?;
        }

        for binding in &self.resources {
            binding.validate()?;
        }

        for binding in &self.additional_operation_bindings {
            binding.validate()?;
        }

        if self.operation_policy != OperationBindingPolicy::AllowMultiple
            && !self.additional_operation_bindings.is_empty()
        {
            return Err(IrIntegrationError::InvalidReferenceSet {
                message:
                    "additional operation bindings require AllowMultiple policy",
            });
        }

        Ok(())
    }

    /// Validates operation references against a canonical operation namespace.
    ///
    /// This method accepts an iterator instead of a concrete `QuantumCircuit`
    /// so the integration layer remains independent of the circuit model.
    ///
    /// The supplied namespace is treated as authoritative for this validation
    /// call only.
    pub fn validate_operations<I>(
        &self,
        operations: I,
    ) -> IrIntegrationResult<()>
    where
        I: IntoIterator<Item = OperationId>,
    {
        let known: BTreeSet<OperationId> = operations.into_iter().collect();

        for binding in &self.operations {
            if !known.contains(binding.0) {
                return Err(IrIntegrationError::UnknownOperation {
                    operation: *binding.0,
                });
            }
        }

        for binding in &self.additional_operation_bindings {
            if let Some(operation) = binding.operation_id() {
                if !known.contains(&operation) {
                    return Err(IrIntegrationError::UnknownOperation {
                        operation,
                    });
                }
            }
        }

        Ok(())
    }

    /// Validates operation references against a caller-supplied namespace with
    /// duplicate detection.
    ///
    /// This variant is useful for streaming canonical IR operation IDs from a
    /// large circuit.
    pub fn validate_operations_exact<I>(
        &self,
        operations: I,
    ) -> IrIntegrationResult<()>
    where
        I: IntoIterator<Item = OperationId>,
    {
        let mut known = BTreeSet::new();

        for operation in operations {
            if !known.insert(operation) {
                return Err(
                    IrIntegrationError::DuplicateOperationIdentity {
                        operation,
                    },
                );
            }
        }

        self.validate_operations(known)
    }

    /// Returns all operation IDs that have a primary binding.
    #[must_use]
    pub fn operation_ids(&self) -> impl Iterator<Item = OperationId> + '_ {
        self.operations.keys().copied()
    }

    /// Returns true if at least one binding references the supplied model.
    #[must_use]
    pub fn contains_noise_model(&self, model: NoiseModelId) -> bool {
        self.operations
            .values()
            .any(|binding| binding.noise_model_id() == model)
            || self
                .resources
                .iter()
                .any(|binding| binding.noise_model_id() == model)
            || self
                .additional_operation_bindings
                .iter()
                .any(|binding| binding.noise_model_id() == model)
    }

    /// Returns the number of bindings referencing the supplied model.
    #[must_use]
    pub fn count_noise_model(&self, model: NoiseModelId) -> usize {
        self.operations
            .values()
            .filter(|binding| binding.noise_model_id() == model)
            .count()
            .saturating_add(
                self.resources
                    .iter()
                    .filter(|binding| binding.noise_model_id() == model)
                    .count(),
            )
            .saturating_add(
                self.additional_operation_bindings
                    .iter()
                    .filter(|binding| binding.noise_model_id() == model)
                    .count(),
            )
    }

    /// Returns all bindings associated with a specific noise model.
    ///
    /// The returned vector is deterministic and newly allocated.
    #[must_use]
    pub fn bindings_for_noise_model(
        &self,
        model: NoiseModelId,
    ) -> Vec<&IrNoiseBinding> {
        let mut result = self
            .bindings()
            .into_iter()
            .filter(|binding| binding.noise_model_id() == model)
            .collect::<Vec<_>>();

        result.sort();

        result
    }

    /// Returns all bindings associated with a canonical operation.
    ///
    /// This includes the primary binding and any additional bindings.
    #[must_use]
    pub fn bindings_for_operation(
        &self,
        operation: OperationId,
    ) -> Vec<&IrNoiseBinding> {
        let mut result = Vec::new();

        if let Some(binding) = self.operations.get(&operation) {
            result.push(binding);
        }

        result.extend(
            self.additional_operation_bindings
                .iter()
                .filter(|binding| binding.operation_id() == Some(operation)),
        );

        result.sort();

        result
    }

    /// Finds all explicitly represented bindings for a canonical resource.
    ///
    /// Deferred selectors are not resolved.
    #[must_use]
    pub fn bindings_for_resource(
        &self,
        resource: ResourceIdentity,
    ) -> Vec<&IrNoiseBinding> {
        let mut result = Vec::new();

        for binding in self.bindings() {
            if binding
                .scope()
                .resource_scope()
                .is_some_and(|scope| scope.contains_explicit(resource))
            {
                result.push(binding);
            }
        }

        result.sort();

        result
    }

    /// Returns the number of explicitly enumerated resources across all
    /// bindings.
    ///
    /// Selector scopes do not contribute because they have not been resolved.
    #[must_use]
    pub fn explicit_resource_count(&self) -> usize {
        self.bindings()
            .into_iter()
            .map(|binding| {
                binding
                    .scope()
                    .resource_scope()
                    .map_or(0, IrResourceScope::explicit_resource_count)
            })
            .sum()
    }

    /// Returns a deterministic structural fingerprint.
    ///
    /// This is suitable for:
    ///
    /// - in-process change detection;
    /// - deterministic test assertions;
    /// - cache-key components where cryptographic security is not required.
    ///
    /// It is NOT a cryptographic hash and MUST NOT replace the repository's
    /// canonical hashing subsystem.
    #[must_use]
    pub fn structural_fingerprint(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.operation_policy.hash(&mut hasher);

        for binding in self.bindings() {
            binding.hash(&mut hasher);
        }

        hasher.finish()
    }

    /// Clears every binding.
    pub fn clear(&mut self) {
        self.operations.clear();
        self.resources.clear();
        self.additional_operation_bindings.clear();
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for constructing a deterministic ZQN ↔ IR binding set.
///
/// The builder performs validation before producing the final immutable-value
/// style container.
#[derive(Debug, Clone)]
pub struct IrNoiseBindingsBuilder {
    policy: OperationBindingPolicy,
    bindings: Vec<IrNoiseBinding>,
    maximum_bindings: Option<usize>,
}

impl Default for IrNoiseBindingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IrNoiseBindingsBuilder {
    /// Creates a builder with duplicate rejection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: OperationBindingPolicy::RejectDuplicate,
            bindings: Vec::new(),
            maximum_bindings: None,
        }
    }

    /// Sets the operation duplicate policy.
    #[must_use]
    pub const fn operation_policy(
        mut self,
        policy: OperationBindingPolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Applies an explicit caller-owned maximum binding count.
    ///
    /// This is a resource policy, not a ZQN semantic limit.
    #[must_use]
    pub const fn maximum_bindings(mut self, maximum: usize) -> Self {
        self.maximum_bindings = Some(maximum);
        self
    }

    /// Adds one binding to the candidate set.
    pub fn push(
        &mut self,
        binding: IrNoiseBinding,
    ) -> IrIntegrationResult<()> {
        binding.validate()?;

        if let Some(maximum) = self.maximum_bindings {
            if self.bindings.len() >= maximum {
                return Err(IrIntegrationError::BindingLimitExceeded {
                    maximum,
                    requested: self.bindings.len().saturating_add(1),
                });
            }
        }

        self.bindings.push(binding);
        Ok(())
    }

    /// Builds the final deterministic association set.
    ///
    /// Construction is transactional: if any binding is invalid or
    /// contradictory, no partially constructed result is returned.
    pub fn build(self) -> IrIntegrationResult<IrNoiseBindings> {
        let mut result =
            IrNoiseBindings::with_operation_policy(self.policy);

        result.extend(self.bindings)?;

        Ok(result)
    }
}

// =============================================================================
// Operation binding helper
// =============================================================================

/// Creates a validated operation-level binding.
///
/// This helper is intentionally small so downstream code can write:
///
/// ```text
/// bind_operation(operation_id, noise_model_id)
/// ```
///
/// without constructing an intermediate representation manually.
pub fn bind_operation(
    operation: OperationId,
    noise_model: NoiseModelId,
) -> IrIntegrationResult<IrNoiseBinding> {
    IrNoiseBinding::for_operation(operation, noise_model)
}

/// Creates a validated resource-level binding.
pub fn bind_resource(
    resource: IrResourceScope,
    noise_model: NoiseModelId,
) -> IrIntegrationResult<IrNoiseBinding> {
    IrNoiseBinding::for_resource(resource, noise_model)
}

/// Creates a validated operation-and-resource binding.
pub fn bind_operation_and_resource(
    operation: OperationId,
    resource: IrResourceScope,
    noise_model: NoiseModelId,
) -> IrIntegrationResult<IrNoiseBinding> {
    IrNoiseBinding::for_operation_and_resource(
        operation,
        resource,
        noise_model,
    )
}

// =============================================================================
// Canonical operation namespace helper
// =============================================================================

/// Validates that a sequence of canonical operation IDs contains no duplicates
/// and can therefore be used as an IR reference namespace.
///
/// This function does not inspect concrete operations.
pub fn validate_operation_namespace<I>(
    operations: I,
) -> IrIntegrationResult<BTreeSet<OperationId>>
where
    I: IntoIterator<Item = OperationId>,
{
    let mut result = BTreeSet::new();

    for operation in operations {
        if !result.insert(operation) {
            return Err(IrIntegrationError::DuplicateOperationIdentity {
                operation,
            });
        }
    }

    Ok(result)
}

/// Validates a non-empty canonical operation namespace.
///
/// This is useful for consumers that require a concrete operation set rather
/// than merely permitting an empty program.
pub fn validate_non_empty_operation_namespace<I>(
    operations: I,
) -> IrIntegrationResult<BTreeSet<OperationId>>
where
    I: IntoIterator<Item = OperationId>,
{
    let result = validate_operation_namespace(operations)?;

    if result.is_empty() {
        return Err(IrIntegrationError::EmptyOperationNamespace);
    }

    Ok(result)
}

// =============================================================================
// Resource helpers
// =============================================================================

/// Creates a logical resource identity using the canonical IR `QubitId`.
#[must_use]
pub const fn logical_resource(qubit: QubitId) -> ResourceIdentity {
    ResourceIdentity::LogicalQubit(qubit)
}

/// Creates a physical resource identity using the canonical IR
/// `PhysicalQubitId`.
#[must_use]
pub const fn physical_resource(
    qubit: PhysicalQubitId,
) -> ResourceIdentity {
    ResourceIdentity::PhysicalQubit(qubit)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn model(value: u64) -> NoiseModelId {
        NoiseModelId::new(value)
    }

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn logical(value: u64) -> QubitId {
        QubitId::new(value)
    }

    fn physical(value: u64) -> PhysicalQubitId {
        PhysicalQubitId::new(value)
    }

    #[test]
    fn uses_canonical_logical_qubit_identity() {
        let qubit = logical(7);

        let resource = ResourceIdentity::logical(qubit);

        assert_eq!(
            resource,
            ResourceIdentity::LogicalQubit(qubit)
        );
    }

    #[test]
    fn uses_canonical_physical_qubit_identity() {
        let qubit = physical(11);

        let resource = ResourceIdentity::physical(qubit);

        assert_eq!(
            resource,
            ResourceIdentity::PhysicalQubit(qubit)
        );
    }

    #[test]
    fn logical_and_physical_identity_domains_remain_distinct() {
        let logical_resource = ResourceIdentity::logical(logical(7));
        let physical_resource = ResourceIdentity::physical(physical(7));

        assert_ne!(logical_resource, physical_resource);
    }

    #[test]
    fn operation_binding_uses_canonical_operation_id() {
        let operation_id = operation(42);
        let binding =
            IrNoiseBinding::for_operation(operation_id, model(1))
                .expect("operation binding should be valid");

        assert_eq!(
            binding.operation_id(),
            Some(operation_id)
        );

        assert_eq!(binding.noise_model_id(), model(1));
    }

    #[test]
    fn resource_binding_does_not_require_operation_id() {
        let binding = IrNoiseBinding::for_resource(
            IrResourceScope::logical_qubit(logical(2)),
            model(1),
        )
        .expect("resource binding should be valid");

        assert_eq!(binding.operation_id(), None);
        assert!(binding.targets_resource());
        assert!(!binding.targets_operation());
    }

    #[test]
    fn operation_and_resource_binding_contains_both_domains() {
        let operation_id = operation(3);

        let binding = IrNoiseBinding::for_operation_and_resource(
            operation_id,
            IrResourceScope::physical_qubit(physical(8)),
            model(9),
        )
        .expect("combined binding should be valid");

        assert_eq!(
            binding.operation_id(),
            Some(operation_id)
        );

        assert!(binding.targets_operation());
        assert!(binding.targets_resource());
    }

    #[test]
    fn empty_explicit_resource_scope_is_rejected() {
        let result = IrResourceScope::resources(
            std::iter::empty::<ResourceIdentity>(),
        );

        assert!(matches!(
            result,
            Err(IrIntegrationError::InvalidResourceBinding { .. })
        ));
    }

    #[test]
    fn duplicate_resources_are_rejected() {
        let resource = ResourceIdentity::logical(logical(1));

        let result = IrResourceScope::resources([
            resource,
            resource,
        ]);

        assert!(matches!(
            result,
            Err(IrIntegrationError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn selector_does_not_materialize_resources() {
        let scope = IrResourceScope::selector(
            IrResourceSelector::AllPhysicalQubits,
        );

        assert_eq!(scope.explicit_resource_count(), 0);
        assert!(scope.contains_physical_resources());
    }

    #[test]
    fn selector_label_rejects_empty_value() {
        let result = IrResourceSelector::label("   ");

        assert!(matches!(
            result,
            Err(IrIntegrationError::InvalidSelector { .. })
        ));
    }

    #[test]
    fn duplicate_operation_is_rejected_by_default() {
        let operation_id = operation(1);

        let first =
            IrNoiseBinding::for_operation(operation_id, model(1))
                .expect("binding should be valid");

        let second =
            IrNoiseBinding::for_operation(operation_id, model(2))
                .expect("binding should be valid");

        let mut bindings = IrNoiseBindings::new();

        bindings
            .insert(first)
            .expect("first binding should insert");

        let result = bindings.insert(second);

        assert!(matches!(
            result,
            Err(IrIntegrationError::DuplicateOperationBinding {
                operation
            }) if operation == operation_id
        ));
    }

    #[test]
    fn replacement_policy_is_atomic() {
        let operation_id = operation(1);

        let first =
            IrNoiseBinding::for_operation(operation_id, model(1))
                .expect("binding should be valid");

        let second =
            IrNoiseBinding::for_operation(operation_id, model(2))
                .expect("binding should be valid");

        let mut bindings =
            IrNoiseBindings::with_operation_policy(
                OperationBindingPolicy::Replace,
            );

        bindings
            .insert(first)
            .expect("first binding should insert");

        bindings
            .insert(second)
            .expect("replacement should succeed");

        assert_eq!(
            bindings
                .get_operation(operation_id)
                .expect("operation should exist")
                .noise_model_id(),
            model(2)
        );
    }

    #[test]
    fn multiple_policy_preserves_additional_bindings() {
        let operation_id = operation(1);

        let first =
            IrNoiseBinding::for_operation(operation_id, model(1))
                .expect("binding should be valid");

        let second =
            IrNoiseBinding::for_operation(operation_id, model(2))
                .expect("binding should be valid");

        let mut bindings =
            IrNoiseBindings::with_operation_policy(
                OperationBindingPolicy::AllowMultiple,
            );

        bindings
            .insert(first)
            .expect("first binding should insert");

        bindings
            .insert(second)
            .expect("second binding should insert");

        assert_eq!(bindings.operation_binding_count(), 1);
        assert_eq!(
            bindings.additional_operation_binding_count(),
            1
        );
        assert_eq!(
            bindings.bindings_for_operation(operation_id).len(),
            2
        );
    }

    #[test]
    fn insertion_is_deterministic() {
        let mut first = IrNoiseBindings::new();
        let mut second = IrNoiseBindings::new();

        let a =
            IrNoiseBinding::for_operation(operation(2), model(20))
                .expect("binding should be valid");

        let b =
            IrNoiseBinding::for_operation(operation(1), model(10))
                .expect("binding should be valid");

        first
            .insert(a.clone())
            .expect("binding should insert");

        first
            .insert(b.clone())
            .expect("binding should insert");

        second
            .insert(b)
            .expect("binding should insert");

        second
            .insert(a)
            .expect("binding should insert");

        assert_eq!(
            first.structural_fingerprint(),
            second.structural_fingerprint()
        );
    }

    #[test]
    fn operation_namespace_rejects_duplicates() {
        let result = validate_operation_namespace([
            operation(1),
            operation(2),
            operation(1),
        ]);

        assert!(matches!(
            result,
            Err(IrIntegrationError::DuplicateOperationIdentity {
                operation
            }) if operation == operation(1)
        ));
    }

    #[test]
    fn operation_namespace_accepts_unique_ids() {
        let result = validate_operation_namespace([
            operation(1),
            operation(2),
            operation(3),
        ])
        .expect("namespace should be valid");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn empty_non_empty_namespace_is_rejected() {
        let result =
            validate_non_empty_operation_namespace(
                std::iter::empty::<OperationId>(),
            );

        assert!(matches!(
            result,
            Err(IrIntegrationError::EmptyOperationNamespace)
        ));
    }

    #[test]
    fn binding_validation_does_not_require_model_registry() {
        let binding =
            IrNoiseBinding::for_operation(operation(1), model(999))
                .expect("structural binding should be valid");

        assert!(binding.validate().is_ok());
    }

    #[test]
    fn unknown_operation_is_detected_externally() {
        let mut bindings = IrNoiseBindings::new();

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation(99),
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("binding should insert");

        let result =
            bindings.validate_operations([operation(1), operation(2)]);

        assert!(matches!(
            result,
            Err(IrIntegrationError::UnknownOperation {
                operation
            }) if operation == operation(99)
        ));
    }

    #[test]
    fn resource_lookup_is_deterministic() {
        let resource = ResourceIdentity::logical(logical(4));

        let binding =
            IrNoiseBinding::for_resource(
                IrResourceScope::Resource(resource),
                model(1),
            )
            .expect("binding should be valid");

        let mut bindings = IrNoiseBindings::new();

        bindings
            .insert(binding)
            .expect("binding should insert");

        let found = bindings.bindings_for_resource(resource);

        assert_eq!(found.len(), 1);
        assert!(found[0].scope().resource_scope().is_some());
    }

    #[test]
    fn builder_builds_atomically() {
        let operation_id = operation(1);

        let mut builder = IrNoiseBindingsBuilder::new();

        builder
            .push(
                IrNoiseBinding::for_operation(
                    operation_id,
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("push should succeed");

        let bindings =
            builder.build().expect("build should succeed");

        assert_eq!(bindings.operation_binding_count(), 1);
    }

    #[test]
    fn builder_detects_duplicate_operations() {
        let operation_id = operation(1);

        let mut builder = IrNoiseBindingsBuilder::new();

        builder
            .push(
                IrNoiseBinding::for_operation(
                    operation_id,
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("first push should succeed");

        builder
            .push(
                IrNoiseBinding::for_operation(
                    operation_id,
                    model(2),
                )
                .expect("binding should be valid"),
            )
            .expect("second push should succeed");

        let result = builder.build();

        assert!(matches!(
            result,
            Err(IrIntegrationError::DuplicateOperationBinding {
                operation
            }) if operation == operation_id
        ));
    }

    #[test]
    fn explicit_binding_limit_is_policy_not_semantic_limit() {
        let mut builder =
            IrNoiseBindingsBuilder::new().maximum_bindings(1);

        builder
            .push(
                IrNoiseBinding::for_operation(
                    operation(1),
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("first binding should fit");

        let result = builder.push(
            IrNoiseBinding::for_operation(
                operation(2),
                model(2),
            )
            .expect("binding should be valid"),
        );

        assert!(matches!(
            result,
            Err(IrIntegrationError::BindingLimitExceeded {
                maximum: 1,
                requested: 2
            })
        ));
    }

    #[test]
    fn remove_all_for_operation_is_complete() {
        let operation_id = operation(1);

        let mut bindings =
            IrNoiseBindings::with_operation_policy(
                OperationBindingPolicy::AllowMultiple,
            );

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation_id,
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("first binding should insert");

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation_id,
                    model(2),
                )
                .expect("binding should be valid"),
            )
            .expect("second binding should insert");

        let removed =
            bindings.remove_all_for_operation(operation_id);

        assert_eq!(removed.len(), 2);
        assert!(bindings.bindings_for_operation(operation_id).is_empty());
    }

    #[test]
    fn composite_scope_rejects_empty_scope() {
        let result = IrResourceScope::composite(
            std::iter::empty::<IrResourceScope>(),
        );

        assert!(matches!(
            result,
            Err(IrIntegrationError::EmptyCompositeScope)
        ));
    }

    #[test]
    fn composite_scope_preserves_nested_semantics() {
        let scope = IrResourceScope::composite([
            IrResourceScope::logical_qubit(logical(1)),
            IrResourceScope::physical_qubit(physical(2)),
        ])
        .expect("composite should be valid");

        assert!(scope.contains_logical_resources());
        assert!(scope.contains_physical_resources());
        assert_eq!(scope.explicit_resource_count(), 2);
    }

    #[test]
    fn labels_are_metadata_not_execution_code() {
        let binding =
            IrNoiseBinding::for_operation(
                operation(1),
                model(1),
            )
            .expect("binding should be valid")
            .with_label("readout-noise")
            .expect("label should be valid");

        assert_eq!(binding.label(), Some("readout-noise"));
    }

    #[test]
    fn global_scope_is_not_an_explicit_resource() {
        let scope = IrResourceScope::global();

        assert_eq!(scope.explicit_resource_count(), 0);
        assert!(!scope.contains_logical_resources());
        assert!(!scope.contains_physical_resources());
    }

    #[test]
    fn helpers_use_canonical_types() {
        let logical = logical_resource(logical(5));
        let physical = physical_resource(physical(6));

        assert!(logical.is_logical());
        assert!(physical.is_physical());
    }

    #[test]
    fn bindings_can_be_filtered_by_noise_model() {
        let mut bindings = IrNoiseBindings::new();

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation(1),
                    model(10),
                )
                .expect("binding should be valid"),
            )
            .expect("binding should insert");

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation(2),
                    model(20),
                )
                .expect("binding should be valid"),
            )
            .expect("binding should insert");

        assert!(bindings.contains_noise_model(model(10)));
        assert_eq!(bindings.count_noise_model(model(10)), 1);
        assert_eq!(
            bindings.bindings_for_noise_model(model(20)).len(),
            1
        );
    }

    #[test]
    fn clear_removes_every_binding() {
        let mut bindings = IrNoiseBindings::new();

        bindings
            .insert(
                IrNoiseBinding::for_operation(
                    operation(1),
                    model(1),
                )
                .expect("binding should be valid"),
            )
            .expect("binding should insert");

        bindings
            .insert(
                IrNoiseBinding::for_resource(
                    IrResourceScope::logical_qubit(logical(1)),
                    model(2),
                )
                .expect("binding should be valid"),
            )
            .expect("binding should insert");

        bindings.clear();

        assert!(bindings.is_empty());
        assert_eq!(bindings.binding_count(), 0);
    }
}