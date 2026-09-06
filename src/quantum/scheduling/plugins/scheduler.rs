//! Zamani Quantum Scheduling — Safe Scheduler Plugin Registry
//!
//! Path:
//!     src/quantum/scheduling/plugins/scheduler.rs
//!
//! # Purpose
//!
//! This module provides the production-safe extension boundary for quantum
//! scheduling planners.
//!
//! A scheduling plugin is an in-process implementation of the canonical:
//!
//!     crate::quantum::scheduling::planners::planner::SchedulingPlanner
//!
//! Plugins may provide:
//!
//! - built-in scheduling planners;
//! - research scheduling algorithms;
//! - application-specific schedulers;
//! - provider-independent scheduling strategies;
//! - adaptive schedulers;
//! - experimental schedulers;
//! - downstream Rust implementations;
//! - specialized QEC schedulers;
//! - distributed scheduling strategies.
//!
//! # Critical architectural rule
//!
//! This module owns PLUGIN MANAGEMENT.
//!
//! It does NOT own scheduling algorithms.
//!
//! In particular, this module must never become another implementation of:
//!
//! - ASAP;
//! - ALAP;
//! - list scheduling;
//! - critical-path scheduling;
//! - RCPSP;
//! - event scheduling;
//! - resource scheduling;
//! - routing;
//! - QEC;
//! - hardware execution.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                         SchedulingConfig
//!                              │
//!                              ▼
//!                    SchedulerPluginRegistry
//!                              │
//!                    ┌─────────┴─────────┐
//!                    │                   │
//!                    ▼                   ▼
//!              built-in planner     custom planner
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                    SchedulingPlanner
//!                              │
//!                              ▼
//!                    SchedulingContext
//!                              │
//!                              ▼
//!                       SchedulingResult
//! ```
//!
//! # Stable behavioral contract
//!
//! The canonical behavioral interface is:
//!
//!     crate::quantum::scheduling::planners::planner::SchedulingPlanner
//!
//! This module deliberately does NOT introduce a competing scheduler trait.
//!
//! A new scheduler implementation therefore needs only:
//!
//! 1. an implementation of `SchedulingPlanner`;
//! 2. immutable metadata;
//! 3. a factory;
//! 4. registration in a caller-owned registry.
//!
//! Adding another scheduling algorithm must not require changing this file.
//!
//! # Write once, scale everywhere
//!
//! Nothing in this module represents:
//!
//! - qubit count;
//! - physical qubit count;
//! - operation count;
//! - resource count;
//! - topology dimensions;
//! - schedule depth;
//! - timing resolution;
//! - channel count;
//! - QEC distance;
//! - hardware technology;
//! - vendor.
//!
//! All target-specific information remains in `SchedulingContext`.
//!
//! Therefore a registered planner can be selected for:
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
//! "Infinity" means that this registry introduces no artificial machine-size
//! ceiling. Actual execution remains bounded by the available address space,
//! memory, CPU time, explicit caller limits, and target resources.
//!
//! # Safe extensibility
//!
//! This module intentionally does NOT perform native dynamic-library loading.
//!
//! It does not use:
//!
//! - `unsafe`;
//! - `libloading`;
//! - `dlopen`;
//! - `LoadLibrary`;
//! - raw symbols;
//! - foreign function pointers;
//! - native ABI discovery.
//!
//! In-process plugins are ordinary safe Rust values.
//!
//! Future independently deployed scheduler plugins should use a separately
//! specified process/IPC or WASM boundary rather than making this registry
//! unsafe.
//!
//! # Ownership
//!
//! The registry is caller-owned.
//!
//! There is deliberately no global mutable scheduler registry.
//!
//! This prevents:
//!
//! - hidden global state;
//! - test-order dependence;
//! - cross-compilation contamination;
//! - plugin registration races;
//! - accidental scheduler sharing;
//! - nondeterministic discovery.
//!
//! A higher-level application may place its registry behind an appropriate
//! synchronization primitive if it explicitly requires shared mutation.
//!
//! This module itself does not impose such a policy.
//!
//! # Factory model
//!
//! A registry stores factories rather than one mutable scheduler instance.
//!
//! ```text
//! registry
//!     │
//!     ├── metadata
//!     └── factory
//!           │
//!           ▼
//!     fresh planner
//!           │
//!           ▼
//!       one request
//! ```
//!
//! This prevents accidental state sharing between independent compilations.
//!
//! Factories receive no hidden global state.
//!
//! # Canonical qubit identity
//!
//! This file does not need to inspect qubit identities directly.
//!
//! When scheduler plugins do require qubit identity, the authoritative types
//! are:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No scheduler plugin may define a replacement qubit identity.
//!
//! # Separation of responsibilities
//!
//! ```text
//! quantum::ir
//!       │
//!       ▼
//! routing
//!       │
//!       ▼
//! SchedulingContext
//!       │
//!       ▼
//! scheduler plugin
//!       │
//!       ▼
//! SchedulingPlanner
//!       │
//!       ▼
//! SchedulingResult
//! ```
//!
//! Routing answers:
//!
//!     WHERE?
//!
//! Scheduling answers:
//!
//!     WHEN?
//!
//! Hardware answers:
//!
//!     CAN THIS TARGET EXECUTE IT?
//!
//! The plugin registry must not collapse these boundaries.
//!
//! # Capability negotiation
//!
//! A plugin advertises `PlannerCapabilities`.
//!
//! Compatibility is checked against the supplied `SchedulingContext` by the
//! canonical planner contract.
//!
//! The registry itself does not invent hardware requirements.
//!
//! A plugin that cannot satisfy the requested scheduling context must fail
//! explicitly.
//!
//! There is no silent fallback from an explicitly requested plugin.
//!
//! # Determinism
//!
//! Registry lookup is deterministic.
//!
//! Metadata discovery is sorted by stable plugin identifier.
//!
//! Registration order never determines lookup semantics.
//!
//! The registry itself introduces no randomness.
//!
//! Planner determinism remains the responsibility of the planner implementation
//! and the canonical `SchedulingPlanner` contract.
//!
//! # Versioning
//!
//! Three versions are intentionally distinct:
//!
//! 1. `SCHEDULER_PLUGIN_API_VERSION`
//!    The plugin registry contract.
//!
//! 2. `PlannerVersion`
//!    The implementation version of a concrete planner.
//!
//! 3. `PLANNER_CONTRACT_VERSION`
//!    The canonical scheduling planner behavioral contract.
//!
//! These must not be conflated.
//!
//! # Failure semantics
//!
//! Registration fails explicitly for:
//!
//! - invalid names;
//! - invalid metadata;
//! - incompatible API versions;
//! - duplicate names;
//! - invalid factories.
//!
//! Creation fails explicitly for:
//!
//! - unknown plugin;
//! - factory failure;
//! - incompatible context;
//! - unsupported scheduling requirements.
//!
//! No explicit custom scheduler request may silently fall back to a different
//! scheduler.
//!
//! # Thread safety
//!
//! The registry stores `Arc`-owned factory objects.
//!
//! The registry itself requires `&mut self` for registration and therefore does
//! not need an internal lock.
//!
//! After registration is complete, callers may share immutable registry
//! references according to the normal Rust ownership rules.
//!
//! # Rust compatibility
//!
//! Supported:
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

// =============================================================================
// Imports
// =============================================================================

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
// Public API version
// =============================================================================

/// Version of the safe in-process scheduler-plugin contract.
///
/// This is independent from the version of a concrete planner and independent
/// from the scheduler behavioral contract.
pub const SCHEDULER_PLUGIN_API_VERSION: u32 = 1;

// =============================================================================
// Metadata validation limits
// =============================================================================
//
// These are metadata validation boundaries only.
// They are NOT quantum-machine limits.
// They do not restrict:
// - qubit count;
// - operation count;
// - resource count;
// - topology size;
// - schedule depth.
// =============================================================================

/// Maximum permitted scheduler-plugin name length in bytes.
pub const MAX_SCHEDULER_PLUGIN_NAME_LENGTH: usize = 128;

/// Maximum permitted author/vendor metadata length in bytes.
pub const MAX_SCHEDULER_PLUGIN_AUTHOR_LENGTH: usize = 256;

/// Maximum permitted description length in bytes.
pub const MAX_SCHEDULER_PLUGIN_DESCRIPTION_LENGTH: usize = 1024;

// =============================================================================
// Scheduler plugin name
// =============================================================================

/// Validated stable identifier for a scheduler plugin.
///
/// Allowed grammar:
///
/// ```text
/// [A-Za-z][A-Za-z0-9_.:-]*
/// ```
///
/// The name is an identifier only. It does not encode machine size or
/// hardware topology.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchedulerPluginName(String);

impl SchedulerPluginName {
    /// Creates a validated scheduler-plugin name.
    pub fn new(name: impl Into<String>) -> Result<Self, SchedulerPluginError> {
        let name = name.into();

        validate_identifier(
            &name,
            MAX_SCHEDULER_PLUGIN_NAME_LENGTH,
            "scheduler plugin name",
        )?;

        Ok(Self(name))
    }

    /// Returns the validated name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns the underlying string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SchedulerPluginName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SchedulerPluginName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<String> for SchedulerPluginName {
    type Error = SchedulerPluginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SchedulerPluginName {
    type Error = SchedulerPluginError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Plugin descriptor
// =============================================================================

/// Immutable descriptor for one scheduler plugin.
///
/// The descriptor contains no scheduler execution state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerPluginDescriptor {
    /// Stable plugin identifier.
    name: SchedulerPluginName,

    /// Plugin implementation version.
    version: PlannerVersion,

    /// Registry API version supported by this plugin.
    api_version: u32,

    /// Human-readable plugin name.
    display_name: String,

    /// Optional author/vendor information.
    author: Option<String>,

    /// Optional human-readable description.
    description: Option<String>,

    /// Planner capability declaration.
    capabilities: PlannerCapabilities,
}

impl SchedulerPluginDescriptor {
    /// Creates a descriptor for the current plugin API.
    pub fn new(
        name: impl Into<String>,
        version: PlannerVersion,
        display_name: impl Into<String>,
        capabilities: PlannerCapabilities,
    ) -> Result<Self, SchedulerPluginError> {
        Self::with_api_version(
            name,
            version,
            SCHEDULER_PLUGIN_API_VERSION,
            display_name,
            capabilities,
        )
    }

    /// Creates a descriptor with an explicitly declared plugin API version.
    pub fn with_api_version(
        name: impl Into<String>,
        version: PlannerVersion,
        api_version: u32,
        display_name: impl Into<String>,
        capabilities: PlannerCapabilities,
    ) -> Result<Self, SchedulerPluginError> {
        let name = SchedulerPluginName::new(name)?;
        let display_name = validate_metadata_text(
            display_name.into(),
            "display name",
            MAX_SCHEDULER_PLUGIN_NAME_LENGTH,
            false,
        )?;

        Ok(Self {
            name,
            version,
            api_version,
            display_name,
            author: None,
            description: None,
            capabilities,
        })
    }

    /// Adds optional author/vendor metadata.
    #[must_use]
    pub fn with_author(
        mut self,
        author: impl Into<String>,
    ) -> Result<Self, SchedulerPluginError> {
        self.author = Some(validate_metadata_text(
            author.into(),
            "author",
            MAX_SCHEDULER_PLUGIN_AUTHOR_LENGTH,
            true,
        )?);

        Ok(self)
    }

    /// Adds optional description metadata.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, SchedulerPluginError> {
        self.description = Some(validate_metadata_text(
            description.into(),
            "description",
            MAX_SCHEDULER_PLUGIN_DESCRIPTION_LENGTH,
            true,
        )?);

        Ok(self)
    }

    /// Returns the stable plugin name.
    #[must_use]
    pub fn name(&self) -> &SchedulerPluginName {
        &self.name
    }

    /// Returns the plugin implementation version.
    #[must_use]
    pub const fn version(&self) -> PlannerVersion {
        self.version
    }

    /// Returns the scheduler-plugin API version.
    #[must_use]
    pub const fn api_version(&self) -> u32 {
        self.api_version
    }

    /// Returns the human-readable display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns optional author/vendor information.
    #[must_use]
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    /// Returns optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns declared planner capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> PlannerCapabilities {
        self.capabilities
    }

    /// Validates that the descriptor targets the supported plugin API.
    pub fn validate_api_version(&self) -> Result<(), SchedulerPluginError> {
        if self.api_version != SCHEDULER_PLUGIN_API_VERSION {
            return Err(SchedulerPluginError::UnsupportedApiVersion {
                plugin: self.name.clone(),
                requested: self.api_version,
                supported: SCHEDULER_PLUGIN_API_VERSION,
            });
        }

        Ok(())
    }

    /// Converts this plugin descriptor into canonical planner metadata.
    #[must_use]
    pub fn planner_metadata(&self) -> PlannerMetadata {
        PlannerMetadata::new(
            PlannerId::new(self.name.as_str())
                .expect("validated scheduler plugin name must be a valid PlannerId"),
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

/// Factory used to create a fresh scheduler planner.
///
/// Factories are stored behind `Arc` so registry descriptors can be cheaply
/// cloned without requiring unsafe code.
///
/// A factory must not rely on mutable global state.
pub type SchedulerPluginFactory =
    Arc<dyn Fn() -> SchedulingResult<Box<dyn SchedulingPlanner>> + Send + Sync>;

/// Helper trait for factory construction.
///
/// This is useful when callers prefer a concrete function instead of an
/// explicit closure.
pub trait SchedulerPluginFactoryProvider {
    /// Creates one fresh planner instance.
    fn create(&self) -> SchedulingResult<Box<dyn SchedulingPlanner>>;
}

impl<F> SchedulerPluginFactoryProvider for F
where
    F: Fn() -> SchedulingResult<Box<dyn SchedulingPlanner>>,
{
    fn create(&self) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        (self)()
    }
}

// =============================================================================
// Registered plugin
// =============================================================================

/// Immutable registered scheduler plugin.
///
/// The descriptor and factory are kept together so the registry cannot
/// accidentally associate metadata from one planner with another factory.
#[derive(Clone)]
pub struct RegisteredSchedulerPlugin {
    descriptor: SchedulerPluginDescriptor,
    factory: SchedulerPluginFactory,
}

impl fmt::Debug for RegisteredSchedulerPlugin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredSchedulerPlugin")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl RegisteredSchedulerPlugin {
    /// Creates a registered plugin record.
    pub fn new(
        descriptor: SchedulerPluginDescriptor,
        factory: SchedulerPluginFactory,
    ) -> Result<Self, SchedulerPluginError> {
        descriptor.validate_api_version()?;

        let plugin = Self {
            descriptor,
            factory,
        };

        plugin.validate_factory_metadata()?;

        Ok(plugin)
    }

    /// Returns immutable plugin metadata.
    #[must_use]
    pub fn descriptor(&self) -> &SchedulerPluginDescriptor {
        &self.descriptor
    }

    /// Creates a fresh planner instance.
    pub fn create(
        &self,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let planner = (self.factory)().map_err(|error| {
            SchedulingError::PluginError {
                plugin: self.descriptor.name.as_str().to_owned(),
                operation: PluginOperation::Create,
                reason: error.to_string(),
            }
        })?;

        self.validate_planner_identity(planner.as_ref())?;

        Ok(planner)
    }

    /// Creates a fresh planner and verifies it against a scheduling context.
    pub fn create_for_context(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let planner = self.create()?;

        planner
            .validate_context(context)
            .map_err(|error| SchedulingError::PluginError {
                plugin: self.descriptor.name.as_str().to_owned(),
                operation: PluginOperation::Schedule,
                reason: error.to_string(),
            })?;

        Ok(planner)
    }

    /// Checks plugin/context compatibility without constructing a planner.
    ///
    /// This is conservative: capability validation is performed from the
    /// descriptor. Concrete planner-specific validation is performed when a
    /// fresh planner is created.
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
                    vec![PlannerPreconditionError::FactoryUnavailable {
                        planner: metadata.id.clone(),
                        reason: error.to_string(),
                    }],
                );
            }
        };

        match planner.validate_context(context) {
            Ok(()) => PlannerCompatibility::compatible(metadata.id.clone()),
            Err(reason) => PlannerCompatibility::incompatible(
                metadata.id.clone(),
                vec![reason],
            ),
        }
    }

    fn validate_factory_metadata(
        &self,
    ) -> Result<(), SchedulerPluginError> {
        let planner = (self.factory)().map_err(|error| {
            SchedulerPluginError::FactoryValidation {
                plugin: self.descriptor.name.clone(),
                reason: error.to_string(),
            }
        })?;

        self.validate_planner_identity(planner.as_ref())
    }

    fn validate_planner_identity(
        &self,
        planner: &dyn SchedulingPlanner,
    ) -> Result<(), SchedulerPluginError> {
        let metadata = planner.metadata();

        let planner_name = metadata.id.as_str();

        if planner_name != self.descriptor.name.as_str() {
            return Err(SchedulerPluginError::MetadataMismatch {
                plugin: self.descriptor.name.clone(),
                field: "planner identifier",
                expected: self.descriptor.name.as_str().to_owned(),
                actual: planner_name.to_owned(),
            });
        }

        if metadata.version != self.descriptor.version {
            return Err(SchedulerPluginError::MetadataMismatch {
                plugin: self.descriptor.name.clone(),
                field: "planner version",
                expected: self.descriptor.version.to_string(),
                actual: metadata.version.to_string(),
            });
        }

        if metadata.capabilities != self.descriptor.capabilities {
            return Err(SchedulerPluginError::CapabilityMismatch {
                plugin: self.descriptor.name.clone(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Registry
// =============================================================================

/// Caller-owned registry of scheduler plugins.
///
/// Internally uses `BTreeMap` to guarantee deterministic name ordering.
///
/// The registry has no global mutable state.
///
/// ```text
/// SchedulerPluginRegistry
///         │
///         ├── planner A
///         ├── planner B
///         └── planner C
/// ```
///
/// Each registered plugin has its own immutable descriptor and factory.
#[derive(Clone, Default)]
pub struct SchedulerPluginRegistry {
    plugins: BTreeMap<SchedulerPluginName, RegisteredSchedulerPlugin>,
}

impl fmt::Debug for SchedulerPluginRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SchedulerPluginRegistry")
            .field("plugins", &self.plugins.keys().collect::<Vec<_>>())
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

    /// Returns the number of registered scheduler plugins.
    ///
    /// This is a registry metadata value, not a scheduling-machine limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether the registry contains no plugins.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Registers a scheduler plugin.
    ///
    /// Registration is transactional:
    ///
    /// - descriptor validation occurs first;
    /// - factory validation occurs before insertion;
    /// - duplicate names are rejected;
    /// - the registry is unchanged on failure.
    pub fn register(
        &mut self,
        plugin: RegisteredSchedulerPlugin,
    ) -> Result<(), SchedulerPluginError> {
        let name = plugin.descriptor.name.clone();

        if self.plugins.contains_key(&name) {
            return Err(SchedulerPluginError::Duplicate {
                plugin: name,
            });
        }

        self.plugins.insert(name, plugin);

        Ok(())
    }

    /// Registers a plugin from a descriptor and factory.
    pub fn register_factory(
        &mut self,
        descriptor: SchedulerPluginDescriptor,
        factory: SchedulerPluginFactory,
    ) -> Result<(), SchedulerPluginError> {
        let plugin = RegisteredSchedulerPlugin::new(
            descriptor,
            factory,
        )?;

        self.register(plugin)
    }

    /// Removes a plugin from the registry.
    ///
    /// Removal is explicit and deterministic.
    pub fn remove(
        &mut self,
        name: &SchedulerPluginName,
    ) -> Option<RegisteredSchedulerPlugin> {
        self.plugins.remove(name)
    }

    /// Returns a registered plugin by name.
    #[must_use]
    pub fn get(
        &self,
        name: &SchedulerPluginName,
    ) -> Option<&RegisteredSchedulerPlugin> {
        self.plugins.get(name)
    }

    /// Returns a plugin by string name.
    ///
    /// The input is validated before lookup.
    pub fn get_str(
        &self,
        name: &str,
    ) -> Result<Option<&RegisteredSchedulerPlugin>, SchedulerPluginError> {
        let name = SchedulerPluginName::new(name)?;
        Ok(self.get(&name))
    }

    /// Returns whether a plugin is registered.
    pub fn contains(
        &self,
        name: &SchedulerPluginName,
    ) -> bool {
        self.plugins.contains_key(name)
    }

    /// Returns whether a string name is registered.
    pub fn contains_str(
        &self,
        name: &str,
    ) -> Result<bool, SchedulerPluginError> {
        Ok(self.get_str(name)?.is_some())
    }

    /// Returns registered plugin names in deterministic lexical order.
    #[must_use]
    pub fn names(&self) -> Vec<&SchedulerPluginName> {
        self.plugins.keys().collect()
    }

    /// Returns immutable registered plugin descriptors in deterministic order.
    #[must_use]
    pub fn descriptors(&self) -> Vec<&SchedulerPluginDescriptor> {
        self.plugins
            .values()
            .map(RegisteredSchedulerPlugin::descriptor)
            .collect()
    }

    /// Returns all plugins compatible with the supplied context.
    ///
    /// Results are deterministic and ordered by plugin identifier.
    #[must_use]
    pub fn compatible_plugins(
        &self,
        context: &SchedulingContext,
    ) -> Vec<&RegisteredSchedulerPlugin> {
        self.plugins
            .values()
            .filter(|plugin| {
                plugin.compatibility(context).is_compatible()
            })
            .collect()
    }

    /// Returns compatibility information for every registered plugin.
    ///
    /// The returned list is deterministic.
    #[must_use]
    pub fn compatibility_report(
        &self,
        context: &SchedulingContext,
    ) -> Vec<PlannerCompatibility> {
        self.plugins
            .values()
            .map(|plugin| plugin.compatibility(context))
            .collect()
    }

    /// Creates a planner selected by explicit plugin name.
    ///
    /// Explicit selection never silently falls back to another plugin.
    pub fn create(
        &self,
        name: &SchedulerPluginName,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let plugin = self.plugins.get(name).ok_or_else(|| {
            SchedulingError::PluginError {
                plugin: name.as_str().to_owned(),
                operation: PluginOperation::Lookup,
                reason: "scheduler plugin is not registered".to_owned(),
            }
        })?;

        plugin.create()
    }

    /// Creates a planner by string name.
    pub fn create_str(
        &self,
        name: &str,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let name = SchedulerPluginName::new(name).map_err(|error| {
            SchedulingError::PluginError {
                plugin: name.to_owned(),
                operation: PluginOperation::Lookup,
                reason: error.to_string(),
            }
        })?;

        self.create(&name)
    }

    /// Creates a planner and validates it against a context.
    pub fn create_for_context(
        &self,
        name: &SchedulerPluginName,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let plugin = self.plugins.get(name).ok_or_else(|| {
            SchedulingError::PluginError {
                plugin: name.as_str().to_owned(),
                operation: PluginOperation::Lookup,
                reason: "scheduler plugin is not registered".to_owned(),
            }
        })?;

        plugin.create_for_context(context)
    }

    /// Creates a planner from a string name and validates the context.
    pub fn create_str_for_context(
        &self,
        name: &str,
        context: &SchedulingContext,
    ) -> SchedulingResult<Box<dyn SchedulingPlanner>> {
        let validated_name =
            SchedulerPluginName::new(name).map_err(|error| {
                SchedulingError::PluginError {
                    plugin: name.to_owned(),
                    operation: PluginOperation::Lookup,
                    reason: error.to_string(),
                }
            })?;

        self.create_for_context(
            &validated_name,
            context,
        )
    }

    /// Executes an explicitly selected plugin.
    ///
    /// The registry creates a fresh planner for this invocation.
    pub fn schedule(
        &self,
        name: &SchedulerPluginName,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        let planner = self.create_for_context(name, context)?;

        planner
            .plan_checked(context)
            .map_err(|error| SchedulingError::PluginError {
                plugin: name.as_str().to_owned(),
                operation: PluginOperation::Schedule,
                reason: error.to_string(),
            })
    }

    /// Executes an explicitly selected plugin by string name.
    pub fn schedule_str(
        &self,
        name: &str,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        let validated_name =
            SchedulerPluginName::new(name).map_err(|error| {
                SchedulingError::PluginError {
                    plugin: name.to_owned(),
                    operation: PluginOperation::Lookup,
                    reason: error.to_string(),
                }
            })?;

        self.schedule(&validated_name, context)
    }

    /// Returns a deterministic snapshot of registry metadata.
    ///
    /// The snapshot contains descriptors only. Factories and executable state
    /// are intentionally excluded.
    #[must_use]
    pub fn snapshot(&self) -> SchedulerPluginRegistrySnapshot {
        let plugins = self
            .plugins
            .values()
            .map(|plugin| plugin.descriptor.clone())
            .collect();

        SchedulerPluginRegistrySnapshot {
            api_version: SCHEDULER_PLUGIN_API_VERSION,
            plugins,
        }
    }
}

// =============================================================================
// Immutable registry snapshot
// =============================================================================

/// Serializable-style immutable metadata snapshot of a plugin registry.
///
/// This type deliberately contains no executable factories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerPluginRegistrySnapshot {
    /// Plugin registry API version.
    pub api_version: u32,

    /// Registered plugin descriptors in deterministic order.
    pub plugins: Vec<SchedulerPluginDescriptor>,
}

impl SchedulerPluginRegistrySnapshot {
    /// Returns the number of registered plugins.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether no plugins are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Finds a descriptor by plugin name.
    #[must_use]
    pub fn find(
        &self,
        name: &SchedulerPluginName,
    ) -> Option<&SchedulerPluginDescriptor> {
        self.plugins
            .iter()
            .find(|descriptor| descriptor.name() == name)
    }
}

// =============================================================================
// Scheduler plugin errors
// =============================================================================

/// Errors specific to scheduler-plugin registration and discovery.
///
/// Execution failures are represented by the canonical `SchedulingError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerPluginError {
    /// Plugin name is invalid.
    InvalidName {
        /// Supplied name.
        name: String,

        /// Reason.
        reason: String,
    },

    /// Plugin metadata is invalid.
    InvalidMetadata {
        /// Metadata field.
        field: &'static str,

        /// Reason.
        reason: String,
    },

    /// Plugin API version is not supported.
    UnsupportedApiVersion {
        /// Plugin identifier.
        plugin: SchedulerPluginName,

        /// Version supplied by plugin.
        requested: u32,

        /// Version supported by registry.
        supported: u32,
    },

    /// Plugin name already exists.
    Duplicate {
        /// Duplicate plugin identifier.
        plugin: SchedulerPluginName,
    },

    /// Factory cannot create a planner during registration validation.
    FactoryValidation {
        /// Plugin identifier.
        plugin: SchedulerPluginName,

        /// Factory error.
        reason: String,
    },

    /// Factory metadata disagrees with registered descriptor.
    MetadataMismatch {
        /// Plugin identifier.
        plugin: SchedulerPluginName,

        /// Metadata field that disagreed.
        field: &'static str,

        /// Expected value.
        expected: String,

        /// Actual value.
        actual: String,
    },

    /// Factory capability declaration disagrees with descriptor.
    CapabilityMismatch {
        /// Plugin identifier.
        plugin: SchedulerPluginName,
    },

    /// A factory returned an invalid planner.
    InvalidPlanner {
        /// Plugin identifier.
        plugin: SchedulerPluginName,

        /// Reason.
        reason: String,
    },
}

impl fmt::Display for SchedulerPluginError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidName { name, reason } => {
                write!(
                    formatter,
                    "invalid scheduler plugin name `{name}`: {reason}"
                )
            }

            Self::InvalidMetadata { field, reason } => {
                write!(
                    formatter,
                    "invalid scheduler plugin metadata `{field}`: {reason}"
                )
            }

            Self::UnsupportedApiVersion {
                plugin,
                requested,
                supported,
            } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` uses API version \
                     {requested}, but version {supported} is supported"
                )
            }

            Self::Duplicate { plugin } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` is already registered"
                )
            }

            Self::FactoryValidation {
                plugin,
                reason,
            } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` factory validation failed: \
                     {reason}"
                )
            }

            Self::MetadataMismatch {
                plugin,
                field,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` metadata mismatch for \
                     `{field}`: expected `{expected}`, got `{actual}`"
                )
            }

            Self::CapabilityMismatch { plugin } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` capability declaration \
                     does not match its planner"
                )
            }

            Self::InvalidPlanner {
                plugin,
                reason,
            } => {
                write!(
                    formatter,
                    "scheduler plugin `{plugin}` produced an invalid planner: \
                     {reason}"
                )
            }
        }
    }
}

impl Error for SchedulerPluginError {}

// =============================================================================
// Canonical scheduling-error conversion
// =============================================================================

impl From<SchedulerPluginError> for SchedulingError {
    fn from(error: SchedulerPluginError) -> Self {
        let plugin = match &error {
            SchedulerPluginError::InvalidName { name, .. } => name.clone(),

            SchedulerPluginError::InvalidMetadata { field, .. } => {
                (*field).to_owned()
            }

            SchedulerPluginError::UnsupportedApiVersion {
                plugin, ..
            }
            | SchedulerPluginError::Duplicate { plugin }
            | SchedulerPluginError::FactoryValidation {
                plugin, ..
            }
            | SchedulerPluginError::MetadataMismatch {
                plugin, ..
            }
            | SchedulerPluginError::CapabilityMismatch { plugin }
            | SchedulerPluginError::InvalidPlanner {
                plugin, ..
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
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum: usize,
    kind: &'static str,
) -> Result<(), SchedulerPluginError> {
    if value.is_empty() {
        return Err(SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!("{kind} must not be empty"),
        });
    }

    if value.len() > maximum {
        return Err(SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!(
                "{kind} exceeds the maximum metadata length of {maximum} bytes"
            ),
        });
    }

    if !value.is_ascii() {
        return Err(SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!("{kind} must contain only ASCII characters"),
        });
    }

    let mut characters = value.bytes();

    let first = characters.next().ok_or_else(|| {
        SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!("{kind} must not be empty"),
        }
    })?;

    if !first.is_ascii_alphabetic() {
        return Err(SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!("{kind} must begin with an ASCII letter"),
        });
    }

    if !characters.all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'.' | b'_' | b'-' | b':'
            )
    }) {
        return Err(SchedulerPluginError::InvalidName {
            name: value.to_owned(),
            reason: format!(
                "{kind} contains an invalid character; only ASCII \
                 letters, digits, '.', '_', '-', and ':' are permitted"
            ),
        });
    }

    Ok(())
}

fn validate_metadata_text(
    value: String,
    field: &'static str,
    maximum: usize,
    optional: bool,
) -> Result<String, SchedulerPluginError> {
    if value.is_empty() {
        if optional {
            return Ok(value);
        }

        return Err(SchedulerPluginError::InvalidMetadata {
            field,
            reason: "value must not be empty".to_owned(),
        });
    }

    if value.len() > maximum {
        return Err(SchedulerPluginError::InvalidMetadata {
            field,
            reason: format!(
                "value exceeds maximum length of {maximum} bytes"
            ),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(SchedulerPluginError::InvalidMetadata {
            field,
            reason: "value must not contain control characters".to_owned(),
        });
    }

    Ok(value)
}

// =============================================================================
// Built-in registration helpers
// =============================================================================

/// Registers a planner using a descriptor and a concrete constructor.
///
/// This helper keeps registration concise while preserving the same validation
/// and factory semantics as `register_factory`.
pub fn register_planner<F>(
    registry: &mut SchedulerPluginRegistry,
    descriptor: SchedulerPluginDescriptor,
    constructor: F,
) -> Result<(), SchedulerPluginError>
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

/// Registers an existing immutable planner constructor.
///
/// The constructor must create a fresh planner instance for every invocation.
pub fn register_default_planner<P>(
    registry: &mut SchedulerPluginRegistry,
    descriptor: SchedulerPluginDescriptor,
) -> Result<(), SchedulerPluginError>
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
// Compatibility helpers
// =============================================================================

/// Checks whether a concrete planner is compatible with a context.
///
/// This function is useful before inserting a planner into a larger pipeline.
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

/// Returns planner metadata without exposing implementation state.
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
                .expect("test planner identifier must be valid");

            Self {
                metadata: PlannerMetadata::new(
                    id,
                    PlannerVersion::new(1, 0, 0),
                    "Test Scheduler",
                    "Scheduler plugin registry test planner.",
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
                reason: "test planner does not execute schedules".to_owned(),
            })
        }
    }

    fn test_descriptor() -> SchedulerPluginDescriptor {
        SchedulerPluginDescriptor::new(
            "test.scheduler",
            PlannerVersion::new(1, 0, 0),
            "Test Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("test descriptor must be valid")
    }

    #[test]
    fn plugin_name_accepts_valid_identifier() {
        let name = SchedulerPluginName::new("scheduler.list_v1")
            .expect("valid scheduler name");

        assert_eq!(name.as_str(), "scheduler.list_v1");
    }

    #[test]
    fn plugin_name_rejects_empty_identifier() {
        assert!(SchedulerPluginName::new("").is_err());
    }

    #[test]
    fn plugin_name_rejects_non_ascii_identifier() {
        assert!(SchedulerPluginName::new("scheduler.é").is_err());
    }

    #[test]
    fn plugin_name_rejects_leading_digit() {
        assert!(SchedulerPluginName::new("1.scheduler").is_err());
    }

    #[test]
    fn descriptor_validates_current_api() {
        let descriptor = test_descriptor();

        assert_eq!(
            descriptor.api_version(),
            SCHEDULER_PLUGIN_API_VERSION
        );

        assert!(descriptor.validate_api_version().is_ok());
    }

    #[test]
    fn registry_starts_empty() {
        let registry = SchedulerPluginRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_registers_plugin() {
        let mut registry = SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            test_descriptor(),
        )
        .expect("registration should succeed");

        assert_eq!(registry.len(), 1);
        assert!(
            registry
                .contains_str("test.scheduler")
                .expect("name is valid")
        );
    }

    #[test]
    fn registry_rejects_duplicate_plugin() {
        let mut registry = SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            test_descriptor(),
        )
        .expect("first registration should succeed");

        let result = register_default_planner::<TestPlanner>(
            &mut registry,
            test_descriptor(),
        );

        assert!(matches!(
            result,
            Err(SchedulerPluginError::Duplicate { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_lookup_is_deterministic() {
        let mut registry = SchedulerPluginRegistry::new();

        let second = SchedulerPluginDescriptor::new(
            "z.scheduler",
            PlannerVersion::new(1, 0, 0),
            "Z Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        let first = SchedulerPluginDescriptor::new(
            "a.scheduler",
            PlannerVersion::new(1, 0, 0),
            "A Scheduler",
            PlannerCapabilities::static_default(),
        )
        .expect("descriptor");

        register_planner::<fn() -> SchedulingResult<Box<dyn SchedulingPlanner>>>(
            &mut registry,
            first,
            || Ok(Box::new(TestPlanner::default())),
        )
        .expect("first registration");

        register_planner(
            &mut registry,
            second,
            || Ok(Box::new(TestPlanner {
                metadata: PlannerMetadata::new(
                    PlannerId::new("z.scheduler")
                        .expect("valid id"),
                    PlannerVersion::new(1, 0, 0),
                    "Z Scheduler",
                    "test",
                    PlannerCapabilities::static_default(),
                ),
            })),
        )
        .expect("second registration");

        let names = registry
            .names()
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec!["a.scheduler", "z.scheduler"]
        );
    }

    #[test]
    fn registry_snapshot_is_sorted() {
        let mut registry = SchedulerPluginRegistry::new();

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
                        .expect("valid id"),
                    PlannerVersion::new(1, 0, 0),
                    "Z Scheduler",
                    "test",
                    PlannerCapabilities::static_default(),
                ),
            })),
        )
        .expect("registration");

        register_planner(
            &mut registry,
            a,
            || Ok(Box::new(TestPlanner::default())),
        )
        .expect("registration");

        let snapshot = registry.snapshot();

        assert_eq!(snapshot.len(), 2);
        assert_eq!(
            snapshot.plugins[0].name().as_str(),
            "a.scheduler"
        );
        assert_eq!(
            snapshot.plugins[1].name().as_str(),
            "z.scheduler"
        );
    }

    #[test]
    fn factory_produces_fresh_instances() {
        let mut registry = SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            test_descriptor(),
        )
        .expect("registration");

        let name = SchedulerPluginName::new("test.scheduler")
            .expect("valid name");

        let first = registry
            .create(&name)
            .expect("first planner");

        let second = registry
            .create(&name)
            .expect("second planner");

        assert_eq!(
            first.metadata().id,
            second.metadata().id
        );

        assert_eq!(
            first.metadata().version,
            second.metadata().version
        );
    }

    #[test]
    fn explicit_unknown_plugin_fails_without_fallback() {
        let registry = SchedulerPluginRegistry::new();

        let result = registry.create_str("does.not.exist");

        assert!(matches!(
            result,
            Err(SchedulingError::PluginError {
                operation: PluginOperation::Lookup,
                ..
            })
        ));
    }

    #[test]
    fn invalid_plugin_name_is_rejected() {
        let result =
            SchedulerPluginName::new("scheduler/name");

        assert!(matches!(
            result,
            Err(SchedulerPluginError::InvalidName { .. })
        ));
    }

    #[test]
    fn registry_remove_is_explicit() {
        let mut registry = SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut registry,
            test_descriptor(),
        )
        .expect("registration");

        let name = SchedulerPluginName::new("test.scheduler")
            .expect("valid name");

        assert!(registry.remove(&name).is_some());
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_clone_is_independent_in_metadata() {
        let mut original = SchedulerPluginRegistry::new();

        register_default_planner::<TestPlanner>(
            &mut original,
            test_descriptor(),
        )
        .expect("registration");

        let mut clone = original.clone();

        let name = SchedulerPluginName::new("test.scheduler")
            .expect("valid name");

        clone.remove(&name);

        assert_eq!(original.len(), 1);
        assert_eq!(clone.len(), 0);
    }
}