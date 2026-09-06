//! Zamani Quantum Scheduling — Scheduler Plugin Registry
//!
//! Path:
//!     src/quantum/scheduling/plugins/registry.rs
//!
//! # Responsibility
//!
//! This module owns the caller-owned registry of scheduler plugins.
//!
//! It is responsible for:
//!
//! - registering scheduler-planner factories;
//! - validating plugin metadata;
//! - preventing duplicate planner identifiers;
//! - deterministic lookup;
//! - explicit planner creation;
//! - compatibility inspection;
//! - deterministic metadata snapshots;
//! - lifecycle isolation between scheduling invocations.
//!
//! It does NOT implement scheduling algorithms.
//!
//! Actual scheduling behavior belongs to:
//!
//! ```text
//! quantum::scheduling::planners
//! quantum::scheduling::algorithms
//! ```
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ▼
//! SchedulerPluginRegistry
//!      │
//!      ├── explicit plugin selection
//!      │
//!      ├── capability inspection
//!      │
//!      └── fresh planner creation
//!      │
//!      ▼
//! SchedulingPlanner
//!      │
//!      ▼
//! SchedulingResult
//! ```
//!
//! # Critical architectural rule
//!
//! The registry answers:
//!
//! > Which scheduler implementation should be instantiated?
//!
//! It does NOT answer:
//!
//! > How should the quantum program be scheduled?
//!
//! Therefore this file must never contain:
//!
//! - ASAP implementation;
//! - ALAP implementation;
//! - list scheduling;
//! - critical-path scheduling;
//! - RCPSP;
//! - event scheduling;
//! - routing;
//! - hardware execution;
//! - QEC decoding;
//! - noise modelling;
//! - quantum gate semantics.
//!
//! # Write once, scale everywhere
//!
//! This registry contains no machine-size assumptions.
//!
//! It has no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum topology size;
//! - maximum schedule depth;
//! - fixed gate count;
//! - fixed gate arity;
//! - fixed channel count;
//! - fixed QEC distance;
//! - hardware-specific constants.
//!
//! The registry therefore works for the same Zamani program targeting:
//!
//! ```text
//! one qubit
//! small QPU
//! large QPU
//! modular QPU
//! distributed QPU
//! quantum network
//! future quantum architecture
//! ```
//!
//! "Infinity" means that the registry introduces no artificial finite quantum
//! machine limit. Real execution is necessarily bounded by available memory,
//! address space, compute resources, explicit caller limits, and target
//! resources.
//!
//! # Canonical quantum identity
//!
//! This registry does not need to inspect qubit identities directly.
//!
//! Whenever a plugin implementation needs quantum identities, the authoritative
//! types remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No registry-specific qubit identity is introduced here.
//!
//! # No global mutable state
//!
//! `SchedulerPluginRegistry` is explicitly caller-owned.
//!
//! There is deliberately no:
//!
//! - global registry;
//! - global mutex;
//! - singleton scheduler;
//! - process-wide mutable plugin table.
//!
//! This prevents hidden coupling between independent compiler invocations,
//! tests, users, and target contexts.
//!
//! If an application needs shared registration, it may place this registry
//! behind its own synchronization primitive.
//!
//! The registry itself remains an ordinary Rust value.
//!
//! # Factory isolation
//!
//! The registry stores factories rather than scheduler instances.
//!
//! ```text
//! registry
//!    │
//!    ├── metadata
//!    └── factory
//!          │
//!          ▼
//!      fresh planner
//!          │
//!          ▼
//!       one run
//! ```
//!
//! A factory must construct a fresh planner instance for every invocation.
//!
//! This prevents state from one compilation leaking into another compilation.
//!
//! # Explicit selection
//!
//! The registry never silently replaces an explicitly requested scheduler.
//!
//! ```text
//! requested planner
//!       │
//!       ├── exists ──► create
//!       │
//!       └── absent ─► error
//! ```
//!
//! There is no implicit fallback.
//!
//! Automatic planner selection belongs to a higher-level scheduling policy or
//! planner-selection component, not this registry.
//!
//! # Determinism
//!
//! Registry ordering is deterministic because `BTreeMap` is used.
//!
//! Registration order therefore cannot affect:
//!
//! - metadata enumeration;
//! - snapshots;
//! - plugin-name enumeration.
//!
//! Plugin scheduling determinism itself remains the responsibility of the
//! `SchedulingPlanner` implementation and its `SchedulingContext`.
//!
//! # Compatibility
//!
//! Compatibility has two levels:
//!
//! 1. registry/plugin metadata compatibility;
//! 2. planner/context compatibility.
//!
//! The registry performs both without duplicating planner capability semantics.
//!
//! # Safe extensibility
//!
//! This is an in-process safe-Rust plugin registry.
//!
//! It deliberately does not load native dynamic libraries.
//!
//! It does not use:
//!
//! - `unsafe`;
//! - `dlopen`;
//! - `LoadLibrary`;
//! - raw function pointers;
//! - foreign ABI discovery;
//! - `libloading`.
//!
//! Independently deployed schedulers should eventually use a separately defined
//! process, WASM, or other safe serialization boundary.
//!
//! # Thread safety
//!
//! Factories are required to be `Send + Sync`.
//!
//! The registry itself does not internally synchronize mutation.
//!
//! Registration requires `&mut self`.
//!
//! After construction, immutable registry references may safely be shared.
//!
//! # Versioning
//!
//! This registry API has its own version.
//!
//! It is independent from:
//!
//! - Zamani package version;
//! - `PLANNER_CONTRACT_VERSION`;
//! - concrete planner implementation versions.
//!
//! # Rust
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::quantum::scheduling::context::SchedulingContext;
use crate::quantum::scheduling::errors::{
    PluginOperation,
    SchedulingError,
    SchedulingResult,
};
use crate::quantum::scheduling::planners::planner::{
    PlannerCapabilities,
    PlannerCompatibility,
    PlannerId,
    PlannerMetadata,
    PlannerPreconditionError,
    PlannerVersion,
    SchedulingPlanner,
};
use crate::quantum::scheduling::result::SchedulingResult as ScheduleArtifact;

// =============================================================================
// Registry API version
// =============================================================================

/// Semantic version of the scheduler-plugin registry contract.
///
/// This version describes the registry API itself. It is intentionally
/// independent of concrete planner versions.
pub const SCHEDULER_PLUGIN_REGISTRY_API_VERSION: u32 = 1;

// =============================================================================
// Scheduler plugin identifier
// =============================================================================

/// Stable identifier for a registered scheduler plugin.
///
/// This type is deliberately distinct from the planner's implementation type,
/// while internally using the canonical `PlannerId` representation.
///
/// No machine-size information is encoded in the identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchedulerPluginId(PlannerId);

impl SchedulerPluginId {
    /// Creates a scheduler-plugin identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        let value = value.into();

        let id = PlannerId::new(value.clone()).map_err(|error| {
            SchedulerPluginRegistryError::InvalidIdentifier {
                identifier: value,
                reason: error.to_string(),
            }
        })?;

        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the underlying canonical planner identifier.
    #[must_use]
    pub fn as_planner_id(&self) -> &PlannerId {
        &self.0
    }

    /// Consumes the wrapper and returns the canonical planner identifier.
    #[must_use]
    pub fn into_planner_id(self) -> PlannerId {
        self.0
    }
}

impl fmt::Display for SchedulerPluginId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<String> for SchedulerPluginId {
    type Error = SchedulerPluginRegistryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SchedulerPluginId {
    type Error = SchedulerPluginRegistryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Scheduler plugin descriptor
// =============================================================================

/// Immutable metadata describing one scheduler plugin.
///
/// The descriptor contains no executable state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerPluginDescriptor {
    /// Stable plugin identifier.
    id: SchedulerPluginId,

    /// Concrete planner implementation version.
    version: PlannerVersion,

    /// Registry API version expected by the plugin.
    registry_api_version: u32,

    /// Human-readable name.
    display_name: String,

    /// Optional author/vendor information.
    author: Option<String>,

    /// Optional description.
    description: Option<String>,

    /// Declared planner capabilities.
    capabilities: PlannerCapabilities,
}

impl SchedulerPluginDescriptor {
    /// Creates a descriptor using the current registry API version.
    pub fn new(
        id: impl Into<String>,
        version: PlannerVersion,
        display_name: impl Into<String>,
        capabilities: PlannerCapabilities,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        Self::with_registry_api_version(
            id,
            version,
            SCHEDULER_PLUGIN_REGISTRY_API_VERSION,
            display_name,
            capabilities,
        )
    }

    /// Creates a descriptor with an explicit registry API version.
    pub fn with_registry_api_version(
        id: impl Into<String>,
        version: PlannerVersion,
        registry_api_version: u32,
        display_name: impl Into<String>,
        capabilities: PlannerCapabilities,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        let id = SchedulerPluginId::new(id)?;

        let display_name = validate_text(
            display_name.into(),
            "display_name",
            false,
        )?;

        Ok(Self {
            id,
            version,
            registry_api_version,
            display_name,
            author: None,
            description: None,
            capabilities,
        })
    }

    /// Adds optional author/vendor metadata.
    pub fn with_author(
        mut self,
        author: impl Into<String>,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        self.author = Some(validate_text(
            author.into(),
            "author",
            true,
        )?);

        Ok(self)
    }

    /// Adds optional description metadata.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        self.description = Some(validate_text(
            description.into(),
            "description",
            true,
        )?);

        Ok(self)
    }

    /// Returns the stable plugin identifier.
    #[must_use]
    pub fn id(&self) -> &SchedulerPluginId {
        &self.id
    }

    /// Returns the concrete planner version.
    #[must_use]
    pub const fn version(&self) -> PlannerVersion {
        self.version
    }

    /// Returns the registry API version expected by the plugin.
    #[must_use]
    pub const fn registry_api_version(&self) -> u32 {
        self.registry_api_version
    }

    /// Returns the human-readable display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns optional author information.
    #[must_use]
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    /// Returns optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns declared capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> PlannerCapabilities {
        self.capabilities
    }

    /// Validates registry API compatibility.
    pub fn validate_api_version(
        &self,
    ) -> Result<(), SchedulerPluginRegistryError> {
        if self.registry_api_version
            != SCHEDULER_PLUGIN_REGISTRY_API_VERSION
        {
            return Err(
                SchedulerPluginRegistryError::UnsupportedRegistryApi {
                    plugin: self.id.clone(),
                    requested: self.registry_api_version,
                    supported: SCHEDULER_PLUGIN_REGISTRY_API_VERSION,
                },
            );
        }

        Ok(())
    }

    /// Converts the descriptor into canonical planner metadata.
    #[must_use]
    pub fn planner_metadata(&self) -> PlannerMetadata {
        PlannerMetadata::new(
            self.id.as_planner_id().clone(),
            self.version,
            self.display_name.clone(),
            self.description.clone().unwrap_or_default(),
            self.capabilities,
        )
    }
}

// =============================================================================
// Factory
// =============================================================================

/// Safe in-process constructor for a fresh scheduling planner.
///
/// Every call must return a new planner instance.
///
/// The factory is required to be `Send + Sync` so immutable registries can be
/// shared between scheduling clients when desired.
pub type SchedulerPluginFactory =
    Arc<dyn Fn() -> SchedulingResult<Box<dyn SchedulingPlanner>>
        + Send
        + Sync>;

/// Helper abstraction for callers that prefer factory providers.
pub trait SchedulerPluginFactoryProvider {
    /// Creates a fresh planner.
    fn create(
        &self,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>>;
}

impl<F> SchedulerPluginFactoryProvider for F
where
    F: Fn() -> SchedulingResult<Box<dyn SchedulingPlanner>>,
{
    fn create(
        &self,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        (self)()
    }
}

// =============================================================================
// Registered plugin
// =============================================================================

/// One validated plugin registration.
///
/// Metadata and factory are immutable after registration.
#[derive(Clone)]
pub struct RegisteredSchedulerPlugin {
    descriptor: SchedulerPluginDescriptor,
    factory: SchedulerPluginFactory,
}

impl fmt::Debug for RegisteredSchedulerPlugin {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RegisteredSchedulerPlugin")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl RegisteredSchedulerPlugin {
    /// Creates and validates a registered plugin.
    ///
    /// The factory is exercised once during registration to verify that:
    ///
    /// - it can create a planner;
    /// - the planner identifier matches;
    /// - the planner version matches;
    /// - declared capabilities match.
    ///
    /// Registration remains transactional because the registry only inserts
    /// the plugin after this function succeeds.
    pub fn new(
        descriptor: SchedulerPluginDescriptor,
        factory: SchedulerPluginFactory,
    ) -> Result<Self, SchedulerPluginRegistryError> {
        descriptor.validate_api_version()?;

        let registered = Self {
            descriptor,
            factory,
        };

        registered.validate_factory_contract()?;

        Ok(registered)
    }

    /// Returns immutable plugin metadata.
    #[must_use]
    pub fn descriptor(&self) -> &SchedulerPluginDescriptor {
        &self.descriptor
    }

    /// Creates a fresh planner.
    pub fn create(
        &self,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let planner = (self.factory)().map_err(|error| {
            SchedulingError::PluginError {
                plugin: self.descriptor.id.as_str().to_owned(),
                operation: PluginOperation::Create,
                reason: error.to_string(),
            }
        })?;

        self.validate_planner_metadata(planner.as_ref())
            .map_err(SchedulingError::from)?;

        Ok(planner)
    }

    /// Creates a fresh planner and validates it against a context.
    pub fn create_for_context(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let planner = self.create()?;

        planner
            .validate_context(context)
            .map_err(|error| SchedulingError::PluginError {
                plugin: self.descriptor.id.as_str().to_owned(),
                operation: PluginOperation::Schedule,
                reason: error.to_string(),
            })?;

        Ok(planner)
    }

    /// Inspects compatibility without scheduling.
    ///
    /// This method constructs a temporary planner because planner-specific
    /// compatibility rules may be unavailable from metadata alone.
    #[must_use]
    pub fn compatibility(
        &self,
        context: &SchedulingContext,
    ) -> PlannerCompatibility {
        let metadata = self.descriptor.planner_metadata();

        let planner = match (self.factory)() {
            Ok(planner) => planner,
            Err(error) => {
                return PlannerCompatibility::incompatible(
                    metadata.id.clone(),
                    vec![
                        PlannerPreconditionError::FactoryUnavailable {
                            planner: metadata.id.clone(),
                            reason: error.to_string(),
                        },
                    ],
                );
            }
        };

        if let Err(error) =
            self.validate_planner_metadata(planner.as_ref())
        {
            return PlannerCompatibility::incompatible(
                metadata.id.clone(),
                vec![
                    PlannerPreconditionError::FactoryUnavailable {
                        planner: metadata.id.clone(),
                        reason: error.to_string(),
                    },
                ],
            );
        }

        match planner.validate_context(context) {
            Ok(()) => {
                PlannerCompatibility::compatible(
                    metadata.id.clone(),
                )
            }

            Err(reason) => {
                PlannerCompatibility::incompatible(
                    metadata.id.clone(),
                    vec![reason],
                )
            }
        }
    }

    fn validate_factory_contract(
        &self,
    ) -> Result<(), SchedulerPluginRegistryError> {
        let planner = (self.factory)().map_err(|error| {
            SchedulerPluginRegistryError::FactoryValidation {
                plugin: self.descriptor.id.clone(),
                reason: error.to_string(),
            }
        })?;

        self.validate_planner_metadata(planner.as_ref())
    }

    fn validate_planner_metadata(
        &self,
        planner: &dyn SchedulingPlanner,
    ) -> Result<(), SchedulerPluginRegistryError> {
        let metadata = planner.metadata();

        if metadata.id != *self.descriptor.id.as_planner_id() {
            return Err(
                SchedulerPluginRegistryError::MetadataMismatch {
                    plugin: self.descriptor.id.clone(),
                    field: "planner identifier",
                    expected: self.descriptor.id.as_str().to_owned(),
                    actual: metadata.id.to_string(),
                },
            );
        }

        if metadata.version != self.descriptor.version {
            return Err(
                SchedulerPluginRegistryError::MetadataMismatch {
                    plugin: self.descriptor.id.clone(),
                    field: "planner version",
                    expected: self.descriptor.version.to_string(),
                    actual: metadata.version.to_string(),
                },
            );
        }

        if metadata.capabilities
            != self.descriptor.capabilities
        {
            return Err(
                SchedulerPluginRegistryError::CapabilityMismatch {
                    plugin: self.descriptor.id.clone(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Registry
// =============================================================================

/// Caller-owned deterministic scheduler-plugin registry.
///
/// The registry is intentionally an ordinary value.
///
/// There is no global mutable registry.
#[derive(Clone, Default)]
pub struct SchedulerPluginRegistry {
    plugins:
        BTreeMap<SchedulerPluginId, RegisteredSchedulerPlugin>,
}

impl fmt::Debug for SchedulerPluginRegistry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("SchedulerPluginRegistry")
            .field(
                "plugins",
                &self.plugins.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl SchedulerPluginRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
        }
    }

    /// Returns the number of registered plugins.
    ///
    /// This is registry cardinality only and is not a quantum-machine limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether no plugins are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Registers a fully validated plugin.
    ///
    /// Duplicate identifiers are rejected.
    ///
    /// The registry remains unchanged if registration fails.
    pub fn register(
        &mut self,
        plugin: RegisteredSchedulerPlugin,
    ) -> Result<(), SchedulerPluginRegistryError> {
        let id = plugin.descriptor.id.clone();

        if self.plugins.contains_key(&id) {
            return Err(
                SchedulerPluginRegistryError::Duplicate {
                    plugin: id,
                },
            );
        }

        self.plugins.insert(id, plugin);

        Ok(())
    }

    /// Registers a plugin from metadata and a factory.
    pub fn register_factory(
        &mut self,
        descriptor: SchedulerPluginDescriptor,
        factory: SchedulerPluginFactory,
    ) -> Result<(), SchedulerPluginRegistryError> {
        let plugin =
            RegisteredSchedulerPlugin::new(
                descriptor,
                factory,
            )?;

        self.register(plugin)
    }

    /// Removes a plugin explicitly.
    pub fn remove(
        &mut self,
        id: &SchedulerPluginId,
    ) -> Option<RegisteredSchedulerPlugin> {
        self.plugins.remove(id)
    }

    /// Returns a registered plugin.
    #[must_use]
    pub fn get(
        &self,
        id: &SchedulerPluginId,
    ) -> Option<&RegisteredSchedulerPlugin> {
        self.plugins.get(id)
    }

    /// Returns a registered plugin by string identifier.
    pub fn get_str(
        &self,
        id: &str,
    ) -> Result<
        Option<&RegisteredSchedulerPlugin>,
        SchedulerPluginRegistryError,
    > {
        let id = SchedulerPluginId::new(id)?;
        Ok(self.get(&id))
    }

    /// Returns whether a plugin exists.
    #[must_use]
    pub fn contains(
        &self,
        id: &SchedulerPluginId,
    ) -> bool {
        self.plugins.contains_key(id)
    }

    /// Returns whether a string identifier exists.
    pub fn contains_str(
        &self,
        id: &str,
    ) -> Result<bool, SchedulerPluginRegistryError> {
        Ok(self.get_str(id)?.is_some())
    }

    /// Returns all plugin identifiers in deterministic order.
    #[must_use]
    pub fn ids(&self) -> Vec<&SchedulerPluginId> {
        self.plugins.keys().collect()
    }

    /// Returns all plugin descriptors in deterministic order.
    #[must_use]
    pub fn descriptors(
        &self,
    ) -> Vec<&SchedulerPluginDescriptor> {
        self.plugins
            .values()
            .map(RegisteredSchedulerPlugin::descriptor)
            .collect()
    }

    /// Returns all plugins compatible with the supplied context.
    ///
    /// The returned collection is ordered by stable plugin identifier.
    #[must_use]
    pub fn compatible_plugins(
        &self,
        context: &SchedulingContext,
    ) -> Vec<&RegisteredSchedulerPlugin> {
        self.plugins
            .values()
            .filter(|plugin| {
                plugin
                    .compatibility(context)
                    .is_compatible()
            })
            .collect()
    }

    /// Returns compatibility information for every registered plugin.
    ///
    /// The report is deterministic.
    #[must_use]
    pub fn compatibility_report(
        &self,
        context: &SchedulingContext,
    ) -> Vec<PlannerCompatibility> {
        self.plugins
            .values()
            .map(|plugin| {
                plugin.compatibility(context)
            })
            .collect()
    }

    /// Creates a planner by explicit identifier.
    ///
    /// There is deliberately no fallback.
    pub fn create(
        &self,
        id: &SchedulerPluginId,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let plugin = self.plugins.get(id).ok_or_else(|| {
            SchedulingError::PluginError {
                plugin: id.as_str().to_owned(),
                operation: PluginOperation::Lookup,
                reason:
                    "scheduler plugin is not registered"
                        .to_owned(),
            }
        })?;

        plugin.create()
    }

    /// Creates a planner by string identifier.
    pub fn create_str(
        &self,
        id: &str,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let validated =
            SchedulerPluginId::new(id).map_err(|error| {
                SchedulingError::PluginError {
                    plugin: id.to_owned(),
                    operation: PluginOperation::Lookup,
                    reason: error.to_string(),
                }
            })?;

        self.create(&validated)
    }

    /// Creates a planner and validates compatibility with a context.
    pub fn create_for_context(
        &self,
        id: &SchedulerPluginId,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let plugin = self.plugins.get(id).ok_or_else(|| {
            SchedulingError::PluginError {
                plugin: id.as_str().to_owned(),
                operation: PluginOperation::Lookup,
                reason:
                    "scheduler plugin is not registered"
                        .to_owned(),
            }
        })?;

        plugin.create_for_context(context)
    }

    /// Creates a planner by string identifier and validates context
    /// compatibility.
    pub fn create_str_for_context(
        &self,
        id: &str,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let validated =
            SchedulerPluginId::new(id).map_err(|error| {
                SchedulingError::PluginError {
                    plugin: id.to_owned(),
                    operation: PluginOperation::Lookup,
                    reason: error.to_string(),
                }
            })?;

        self.create_for_context(
            &validated,
            context,
        )
    }

    /// Executes one explicitly selected scheduler plugin.
    ///
    /// A fresh planner instance is created for every invocation.
    pub fn schedule(
        &self,
        id: &SchedulerPluginId,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        let planner =
            self.create_for_context(id, context)?;

        planner
            .plan_checked(context)
            .map_err(|error| {
                SchedulingError::PluginError {
                    plugin: id.as_str().to_owned(),
                    operation: PluginOperation::Schedule,
                    reason: error.to_string(),
                }
            })
    }

    /// Executes one explicitly selected scheduler plugin by string name.
    pub fn schedule_str(
        &self,
        id: &str,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        let validated =
            SchedulerPluginId::new(id).map_err(|error| {
                SchedulingError::PluginError {
                    plugin: id.to_owned(),
                    operation: PluginOperation::Lookup,
                    reason: error.to_string(),
                }
            })?;

        self.schedule(&validated, context)
    }

    /// Produces an immutable metadata-only registry snapshot.
    ///
    /// Factories and executable planner state are intentionally excluded.
    #[must_use]
    pub fn snapshot(
        &self,
    ) -> SchedulerPluginRegistrySnapshot {
        SchedulerPluginRegistrySnapshot {
            api_version:
                SCHEDULER_PLUGIN_REGISTRY_API_VERSION,
            plugins: self
                .plugins
                .values()
                .map(|plugin| {
                    plugin.descriptor.clone()
                })
                .collect(),
        }
    }
}

// =============================================================================
// Registry snapshot
// =============================================================================

/// Immutable metadata snapshot of a plugin registry.
///
/// This contains no executable factory state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerPluginRegistrySnapshot {
    /// Registry API version.
    pub api_version: u32,

    /// Registered plugin descriptors in deterministic order.
    pub plugins: Vec<SchedulerPluginDescriptor>,
}

impl SchedulerPluginRegistrySnapshot {
    /// Returns the number of descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether the snapshot contains no plugins.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Finds a descriptor by identifier.
    #[must_use]
    pub fn find(
        &self,
        id: &SchedulerPluginId,
    ) -> Option<&SchedulerPluginDescriptor> {
        self.plugins
            .iter()
            .find(|plugin| plugin.id() == id)
    }
}

// =============================================================================
// Registry errors
// =============================================================================

/// Errors specific to scheduler-plugin registration and discovery.
///
/// Scheduling execution errors remain represented by the canonical
/// `SchedulingError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerPluginRegistryError {
    /// Plugin identifier is invalid.
    InvalidIdentifier {
        /// Supplied identifier.
        identifier: String,

        /// Validation reason.
        reason: String,
    },

    /// Plugin metadata contains an invalid value.
    InvalidMetadata {
        /// Metadata field.
        field: &'static str,

        /// Validation reason.
        reason: String,
    },

    /// Registry API version is unsupported.
    UnsupportedRegistryApi {
        /// Plugin identifier.
        plugin: SchedulerPluginId,

        /// Plugin-declared version.
        requested: u32,

        /// Registry-supported version.
        supported: u32,
    },

    /// Plugin identifier is already registered.
    Duplicate {
        /// Duplicate identifier.
        plugin: SchedulerPluginId,
    },

    /// Factory failed while being validated.
    FactoryValidation {
        /// Plugin identifier.
        plugin: SchedulerPluginId,

        /// Factory error.
        reason: String,
    },

    /// Factory-created planner metadata disagrees with registration metadata.
    MetadataMismatch {
        /// Plugin identifier.
        plugin: SchedulerPluginId,

        /// Metadata field.
        field: &'static str,

        /// Expected value.
        expected: String,

        /// Actual value.
        actual: String,
    },

    /// Factory-created planner capabilities disagree with registration.
    CapabilityMismatch {
        /// Plugin identifier.
        plugin: SchedulerPluginId,
    },
}

impl fmt::Display for SchedulerPluginRegistryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                identifier,
                reason,
            } => write!(
                formatter,
                "invalid scheduler plugin identifier `{identifier}`: {reason}"
            ),

            Self::InvalidMetadata {
                field,
                reason,
            } => write!(
                formatter,
                "invalid scheduler plugin metadata `{field}`: {reason}"
            ),

            Self::UnsupportedRegistryApi {
                plugin,
                requested,
                supported,
            } => write!(
                formatter,
                "scheduler plugin `{plugin}` requires registry API \
                 version {requested}, but version {supported} is supported"
            ),

            Self::Duplicate { plugin } => write!(
                formatter,
                "scheduler plugin `{plugin}` is already registered"
            ),

            Self::FactoryValidation {
                plugin,
                reason,
            } => write!(
                formatter,
                "scheduler plugin `{plugin}` factory validation failed: \
                 {reason}"
            ),

            Self::MetadataMismatch {
                plugin,
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "scheduler plugin `{plugin}` metadata mismatch for \
                 `{field}`: expected `{expected}`, got `{actual}`"
            ),

            Self::CapabilityMismatch { plugin } => write!(
                formatter,
                "scheduler plugin `{plugin}` capability declaration \
                 does not match its planner"
            ),
        }
    }
}

impl Error for SchedulerPluginRegistryError {}

// =============================================================================
// Conversion into canonical scheduling errors
// =============================================================================

impl From<SchedulerPluginRegistryError>
    for SchedulingError
{
    fn from(
        error: SchedulerPluginRegistryError,
    ) -> Self {
        let plugin = match &error {
            SchedulerPluginRegistryError::InvalidIdentifier {
                identifier,
                ..
            } => identifier.clone(),

            SchedulerPluginRegistryError::InvalidMetadata {
                field,
                ..
            } => (*field).to_owned(),

            SchedulerPluginRegistryError::UnsupportedRegistryApi {
                plugin,
                ..
            }
            | SchedulerPluginRegistryError::Duplicate {
                plugin,
            }
            | SchedulerPluginRegistryError::FactoryValidation {
                plugin,
                ..
            }
            | SchedulerPluginRegistryError::MetadataMismatch {
                plugin,
                ..
            }
            | SchedulerPluginRegistryError::CapabilityMismatch {
                plugin,
            } => plugin.as_str().to_owned(),
        };

        SchedulingError::PluginError {
            plugin,
            operation: PluginOperation::Register,
            reason: error.to_string(),
        }
    }
}

// =============================================================================
// Metadata validation
// =============================================================================

fn validate_text(
    value: String,
    field: &'static str,
    optional: bool,
) -> Result<String, SchedulerPluginRegistryError> {
    if value.is_empty() && !optional {
        return Err(
            SchedulerPluginRegistryError::InvalidMetadata {
                field,
                reason: "value must not be empty".to_owned(),
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            SchedulerPluginRegistryError::InvalidMetadata {
                field,
                reason:
                    "value must not contain control characters"
                        .to_owned(),
            },
        );
    }

    Ok(value)
}

// =============================================================================
// Registration helpers
// =============================================================================

/// Registers a scheduler planner using a constructor closure.
///
/// The constructor must return a fresh planner instance each time.
pub fn register_planner<F>(
    registry: &mut SchedulerPluginRegistry,
    descriptor: SchedulerPluginDescriptor,
    constructor: F,
) -> Result<(), SchedulerPluginRegistryError>
where
    F: Fn() -> SchedulingResult<Box<dyn SchedulingPlanner>>
        + Send
        + Sync
        + 'static,
{
    registry.register_factory(
        descriptor,
        Arc::new(constructor),
    )
}

/// Registers a `Default` planner.
///
/// The planner must be stateless with respect to individual scheduling
/// invocations or otherwise construct all invocation state inside `plan`.
pub fn register_default_planner<P>(
    registry: &mut SchedulerPluginRegistry,
    descriptor: SchedulerPluginDescriptor,
) -> Result<(), SchedulerPluginRegistryError>
where
    P: SchedulingPlanner + Default + 'static,
{
    register_planner(
        registry,
        descriptor,
        || Ok(Box::new(P::default())),
    )
}

// =============================================================================
// Inspection helpers
// =============================================================================

/// Inspects a concrete scheduler planner against a context.
///
/// This delegates to the canonical planner inspection API rather than
/// reproducing compatibility logic.
#[must_use]
pub fn inspect_scheduler_plugin(
    planner: &dyn SchedulingPlanner,
    context: &SchedulingContext,
) -> PlannerCompatibility {
    crate::quantum::scheduling::planners::planner::inspect_planner(
        planner,
        context,
    )
}

/// Returns canonical planner metadata.
#[must_use]
pub fn scheduler_plugin_metadata(
    planner: &dyn SchedulingPlanner,
) -> &PlannerMetadata {
    crate::quantum::scheduling::planners::planner::planner_metadata(
        planner,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlanner {
        metadata: PlannerMetadata,
    }

    impl Default for TestPlanner {
        fn default() -> Self {
            let id = PlannerId::new("test.scheduler")
                .expect("test planner ID must be valid");

            Self {
                metadata: PlannerMetadata::new(
                    id,
                    PlannerVersion::new(1, 0, 0),
                    "Test Scheduler",
                    "Registry contract test planner.",
                    PlannerCapabilities::static_default(),
                ),
            }
        }
    }

    impl SchedulingPlanner for TestPlanner {
        fn metadata(&self) -> &PlannerMetadata {
            &self.metadata
        }

        fn plan(
            &self,
            _context: &SchedulingContext,
        ) -> SchedulingResult<ScheduleArtifact> {
            Err(SchedulingError::PluginError {
                plugin: "test.scheduler".to_owned(),
                operation: PluginOperation::Schedule,
                reason:
                    "test planner does not execute schedules"
                        .to_owned(),
            })
        }
    }

    fn descriptor() -> SchedulerPluginDescriptor {
        SchedulerPluginDescriptor::new(
            "test.scheduler",
            PlannerVersion::new(1, 0, 0),
            "Test Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("test descriptor must be valid")
    }

    #[test]
    fn valid_plugin_identifier_is_accepted() {
        let id = SchedulerPluginId::new(
            "scheduling.list_v1",
        )
        .expect("identifier must be valid");

        assert_eq!(
            id.as_str(),
            "scheduling.list_v1"
        );
    }

    #[test]
    fn invalid_plugin_identifier_is_rejected() {
        assert!(
            SchedulerPluginId::new("")
                .is_err()
        );

        assert!(
            SchedulerPluginId::new("scheduler/name")
                .is_err()
        );
    }

    #[test]
    fn registry_starts_empty() {
        let registry =
            SchedulerPluginRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registration_is_successful() {
        let mut registry =
            SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            descriptor(),
        )
        .expect("registration must succeed");

        assert_eq!(registry.len(), 1);
        assert!(
            registry
                .contains_str("test.scheduler")
                .expect("identifier is valid")
        );
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry =
            SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            descriptor(),
        )
        .expect("first registration");

        let result =
            register_default_planner::<TestPlanner>(
                &mut registry,
                descriptor(),
            );

        assert!(
            matches!(
                result,
                Err(
                    SchedulerPluginRegistryError::Duplicate {
                        ..
                    }
                )
            )
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn identifiers_are_enumerated_deterministically() {
        let mut registry =
            SchedulerPluginRegistry::new();

        let z = SchedulerPluginDescriptor::new(
            "z.scheduler",
            PlannerVersion::new(1, 0, 0),
            "Z Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        let a = SchedulerPluginDescriptor::new(
            "a.scheduler",
            PlannerVersion::new(1, 0, 0),
            "A Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        register_planner(
            &mut registry,
            z,
            || Ok(Box::new(TestPlanner {
                metadata: PlannerMetadata::new(
                    PlannerId::new("z.scheduler")
                        .expect("valid ID"),
                    PlannerVersion::new(1, 0, 0),
                    "Z Scheduler",
                    "test",
                    PlannerCapabilities::static_default(),
                ),
            })),
        )
        .expect("z registration");

        register_planner(
            &mut registry,
            a,
            || Ok(Box::new(TestPlanner::default())),
        )
        .expect("a registration");

        let names = registry
            .ids()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "a.scheduler",
                "z.scheduler"
            ]
        );
    }

    #[test]
    fn snapshot_is_deterministically_sorted() {
        let mut registry =
            SchedulerPluginRegistry::new();

        let z = SchedulerPluginDescriptor::new(
            "z.scheduler",
            PlannerVersion::new(1, 0, 0),
            "Z Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        let a = SchedulerPluginDescriptor::new(
            "a.scheduler",
            PlannerVersion::new(1, 0, 0),
            "A Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        register_planner(
            &mut registry,
            z,
            || {
                Ok(Box::new(TestPlanner {
                    metadata: PlannerMetadata::new(
                        PlannerId::new("z.scheduler")
                            .expect("valid ID"),
                        PlannerVersion::new(1, 0, 0),
                        "Z Scheduler",
                        "test",
                        PlannerCapabilities::static_default(),
                    ),
                }))
            },
        )
        .expect("z registration");

        register_planner(
            &mut registry,
            a,
            || Ok(Box::new(TestPlanner::default())),
        )
        .expect("a registration");

        let snapshot =
            registry.snapshot();

        assert_eq!(snapshot.len(), 2);

        assert_eq!(
            snapshot.plugins[0].id().as_str(),
            "a.scheduler"
        );

        assert_eq!(
            snapshot.plugins[1].id().as_str(),
            "z.scheduler"
        );
    }

    #[test]
    fn factory_creates_fresh_planners() {
        let mut registry =
            SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            descriptor(),
        )
        .expect("registration");

        let id =
            SchedulerPluginId::new("test.scheduler")
                .expect("valid ID");

        let first =
            registry.create(&id)
                .expect("first planner");

        let second =
            registry.create(&id)
                .expect("second planner");

        assert_eq!(
            first.metadata().id,
            second.metadata().id
        );

        assert_eq!(
            first.metadata().version,
            second.metadata().version
        );

        assert!(
            !std::ptr::eq(
                first.as_ref(),
                second.as_ref()
            )
        );
    }

    #[test]
    fn explicit_unknown_plugin_does_not_fallback() {
        let registry =
            SchedulerPluginRegistry::new();

        let result =
            registry.create_str(
                "does.not.exist"
            );

        assert!(
            matches!(
                result,
                Err(
                    SchedulingError::PluginError {
                        operation:
                            PluginOperation::Lookup,
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn registry_remove_is_explicit() {
        let mut registry =
            SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            descriptor(),
        )
        .expect("registration");

        let id =
            SchedulerPluginId::new("test.scheduler")
                .expect("valid ID");

        assert!(
            registry.remove(&id).is_some()
        );

        assert!(registry.is_empty());
    }

    #[test]
    fn cloned_registry_has_independent_membership() {
        let mut original =
            SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut original,
            descriptor(),
        )
        .expect("registration");

        let mut cloned =
            original.clone();

        let id =
            SchedulerPluginId::new("test.scheduler")
                .expect("valid ID");

        cloned.remove(&id);

        assert_eq!(original.len(), 1);
        assert_eq!(cloned.len(), 0);
    }
}