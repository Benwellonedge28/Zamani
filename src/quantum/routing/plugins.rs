//! Zamani Quantum Routing — Safe Routing Plugin Registry
//!
//! `src/quantum/routing/plugins.rs`
//!
//! # Purpose
//!
//! This module provides the production-safe extension boundary for routing
//! algorithms.
//!
//! A routing plugin is an in-process implementation of the canonical
//! `algorithms::RoutingAlgorithm` trait. Plugins can therefore be supplied by:
//!
//! - Zamani's built-in routing algorithms;
//! - downstream compiler components;
//! - research implementations;
//! - provider-independent hardware strategies;
//! - application-specific routing strategies;
//! - external Rust crates compiled into the Zamani process.
//!
//! # Critical safety rule
//!
//! This module deliberately does NOT perform native dynamic-library loading.
//!
//! In particular, it does not:
//!
//! - load `.so` files;
//! - load `.dll` files;
//! - load `.dylib` files;
//! - resolve arbitrary native symbols;
//! - cast raw function pointers;
//! - execute foreign ABI code;
//! - use `libloading`;
//! - use `dlopen`/`LoadLibrary`;
//! - use `extern` plugin entry points;
//! - use `unsafe`.
//!
//! The registry is therefore completely safe Rust.
//!
//! If Zamani later requires independently deployed routing plugins, the safe
//! production architecture should use a versioned process/IPC or WASM boundary
//! rather than introducing unsafe native loading into this module.
//!
//! # Architectural position
//!
//! ```text
//!                         RoutingConfig
//!                              │
//!                              ▼
//!                    ┌──────────────────┐
//!                    │ PluginRegistry   │
//!                    └────────┬─────────┘
//!                             │
//!              ┌──────────────┼──────────────┐
//!              │              │              │
//!              ▼              ▼              ▼
//!          built-in       custom Rust     application
//!          algorithm       algorithm       algorithm
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                    RoutingAlgorithm
//!                             │
//!                             ▼
//!                         router.rs
//!                             │
//!                             ▼
//!                       RoutingResult
//! ```
//!
//! # Responsibility
//!
//! This file owns:
//!
//! - plugin metadata;
//! - plugin registration;
//! - plugin lookup;
//! - duplicate-name prevention;
//! - plugin-name validation;
//! - plugin capability declaration;
//! - plugin factory registration;
//! - plugin instantiation;
//! - immutable registry snapshots;
//! - deterministic plugin discovery;
//! - compatibility checks;
//! - registry-level diagnostics;
//! - plugin lifecycle boundaries;
//! - safe in-process extensibility.
//!
//! It does NOT own:
//!
//! - routing algorithms themselves;
//! - topology;
//! - mapping;
//! - path finding;
//! - layout;
//! - routing costs;
//! - compiler IR;
//! - OpenQASM;
//! - hardware-provider APIs;
//! - scheduling;
//! - pulse generation;
//! - simulation;
//! - QEC decoding;
//! - benchmark execution.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Integration contract
//!
//! The plugin registry is deliberately built around the already-established
//! `algorithms::RoutingAlgorithm` behavioral contract.
//!
//! ```text
//! plugins.rs
//!      │
//!      ├── RoutingPluginMetadata
//!      ├── RoutingPluginDescriptor
//!      ├── RoutingPluginFactory
//!      └── RoutingPluginRegistry
//!             │
//!             ▼
//! algorithms::RoutingAlgorithm
//!             │
//!             ▼
//! router.rs
//! ```
//!
//! The registry therefore does not introduce a second routing-algorithm trait.
//!
//! # Configuration integration
//!
//! `config.rs` already provides:
//!
//! ```text
//! RoutingAlgorithm::Custom(String)
//! ```
//!
//! The router can resolve that string through this registry:
//!
//! ```text
//! RoutingConfig
//!      │
//!      ▼
//! RoutingAlgorithm::Custom("my_router")
//!      │
//!      ▼
//! PluginRegistry::create("my_router")
//!      │
//!      ▼
//! Box/Arc<dyn RoutingAlgorithm>
//! ```
//!
//! Built-in algorithms may also be registered explicitly under their stable
//! names.
//!
//! # No global registry
//!
//! This module intentionally does NOT expose a global mutable registry.
//!
//! A global registry would introduce:
//!
//! - hidden process state;
//! - test-order dependence;
//! - plugin registration races;
//! - reproducibility problems;
//! - difficult dependency ownership;
//! - accidental cross-compilation contamination.
//!
//! Instead, callers explicitly own a `RoutingPluginRegistry`.
//!
//! This makes routing:
//!
//! - deterministic;
//! - testable;
//! - thread-safe when the registered implementations are thread-safe;
//! - suitable for multiple independent compiler contexts;
//! - suitable for multiple hardware targets in one process.
//!
//! # Plugin identity
//!
//! Plugin names are stable machine-readable identifiers.
//!
//! They must:
//!
//! - be non-empty;
//! - be ASCII;
//! - begin with an ASCII letter;
//! - contain only ASCII letters, digits, `_`, `-`, or `.`;
//! - not exceed `MAX_PLUGIN_NAME_LENGTH`;
//! - be unique within a registry.
//!
//! The registry never silently replaces an existing plugin.
//!
//! # Versioning
//!
//! Plugins expose:
//!
//! - name;
//! - version;
//! - API version;
//! - optional author/vendor;
//! - optional description;
//! - capabilities.
//!
//! The API version belongs to this registry contract and is independent from
//! the plugin implementation version.
//!
//! ```text
//! plugin API version
//!        !=
//! plugin implementation version
//! ```
//!
//! This permits a plugin implementation to evolve without pretending that its
//! registry ABI changed.
//!
//! # Determinism
//!
//! Registration order is never used for lookup.
//!
//! Plugin discovery is sorted by stable plugin name.
//!
//! Plugin metadata must not contain process-dependent values.
//!
//! A plugin itself remains responsible for honoring the deterministic settings
//! supplied through `RoutingConfig`.
//!
//! # Thread safety
//!
//! The registry stores `Arc<dyn RoutingAlgorithm>` values.
//!
//! The registry itself does not use locks because mutation requires exclusive
//! access to the registry.
//!
//! Cloned registries share immutable plugin implementations through `Arc` but
//! own their own name-to-descriptor map.
//!
//! The registry can therefore be placed behind an `RwLock` by a higher-level
//! application if dynamic registration is required.
//!
//! This module does not impose a global synchronization policy on Zamani.
//!
//! # Lifecycle
//!
//! Registration:
//!
//! ```text
//! metadata + factory
//!        │
//!        ▼
//! validation
//!        │
//!        ▼
//! duplicate check
//!        │
//!        ▼
//! registry
//! ```
//!
//! Execution:
//!
//! ```text
//! lookup
//!   │
//!   ▼
//! compatibility check
//!   │
//!   ▼
//! factory
//!   │
//!   ▼
//! RoutingAlgorithm
//!   │
//!   ▼
//! route()
//! ```
//!
//! # Factory model
//!
//! A factory is used instead of storing only one algorithm instance.
//!
//! This allows an algorithm to create a fresh execution object for each
//! routing request, avoiding accidental state sharing between compilations.
//!
//! ```text
//! registry
//!    │
//!    ├── metadata
//!    └── factory
//!             │
//!             ▼
//!       fresh algorithm
//!             │
//!             ▼
//!        one route call
//! ```
//!
//! The factory must not depend on mutable global state.
//!
//! # Resource ownership
//!
//! A plugin does not own:
//!
//! - the caller's circuit;
//! - the caller's topology;
//! - compiler-global state;
//! - hardware-provider credentials;
//! - a global random generator.
//!
//! The canonical `RoutingInput` is passed to the algorithm by the routing
//! contract.
//!
//! # Failure behavior
//!
//! Plugin registration fails explicitly when:
//!
//! - the name is invalid;
//! - the name is already registered;
//! - metadata is inconsistent;
//! - capabilities are incompatible with the declared implementation.
//!
//! Plugin creation fails explicitly when:
//!
//! - the name is absent;
//! - the requested configuration is unsupported;
//! - the factory cannot produce an implementation.
//!
//! No failure silently falls back to another custom plugin.
//!
//! # Built-in algorithms
//!
//! This registry can contain built-in algorithms, but registration is explicit.
//!
//! The router remains responsible for selecting built-in algorithms for
//! `RoutingAlgorithm::Auto`.
//!
//! The registry is primarily responsible for custom/extension selection.
//!
//! # Native dynamic libraries
//!
//! Native ABI loading is intentionally excluded.
//!
//! A future independently deployed plugin protocol should look conceptually
//! like:
//!
//! ```text
//! Zamani router
//!      │
//!      │ versioned IPC / WASM boundary
//!      ▼
//! external routing process/module
//!      │
//!      ▼
//! serialized routing contract
//! ```
//!
//! Such a system must be specified separately because process isolation,
//! serialization, resource limits, capability negotiation, and protocol
//! compatibility are different concerns from an in-process Rust registry.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! The module explicitly denies unsafe Rust.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! - plugin identity is validated;
//! - duplicate registration is rejected;
//! - plugin metadata is immutable after registration;
//! - plugin factories are explicit;
//! - lookup is deterministic;
//! - creation is isolated per request;
//! - configuration compatibility can be checked;
//! - plugin capabilities are queryable;
//! - no global mutable registry exists;
//! - no unsafe code exists;
//! - the public API requires no later structural changes when new routing
//!   algorithms are added.
//!
//! New algorithms should be able to integrate by registering a descriptor and
//! factory without modifying this file.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::routing::algorithms::RoutingAlgorithm;
use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::errors::{
    AlgorithmError,
    RoutingError,
    RoutingResult,
};
use crate::quantum::routing::types::RoutingAlgorithmCapabilities;

// =============================================================================
// Public constants
// =============================================================================

/// Current routing-plugin API version.
///
/// This is the version of the in-process Rust plugin contract, not the version
/// of an individual algorithm implementation.
pub const ROUTING_PLUGIN_API_VERSION: u32 = 1;

/// Maximum permitted plugin identifier length.
pub const MAX_PLUGIN_NAME_LENGTH: usize = 128;

/// Maximum permitted plugin version string length.
pub const MAX_PLUGIN_VERSION_LENGTH: usize = 64;

/// Maximum permitted author/vendor string length.
pub const MAX_PLUGIN_AUTHOR_LENGTH: usize = 256;

/// Maximum permitted description length.
pub const MAX_PLUGIN_DESCRIPTION_LENGTH: usize = 1024;

// =============================================================================
// Plugin name
// =============================================================================

/// Validated stable identifier for a routing plugin.
///
/// This type prevents arbitrary strings from being used as registry keys.
///
/// # Allowed grammar
///
/// ```text
/// [A-Za-z][A-Za-z0-9_.-]{0,127}
/// ```
///
/// Examples:
///
/// ```text
/// my_router
/// sabre_research
/// vendor.router
/// router-v2
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingPluginName(String);

impl RoutingPluginName {
    /// Creates and validates a plugin name.
    pub fn new(name: impl Into<String>) -> Result<Self, PluginError> {
        let name = name.into();

        validate_identifier(
            &name,
            MAX_PLUGIN_NAME_LENGTH,
            "plugin name",
        )?;

        Ok(Self(name))
    }

    /// Returns the plugin identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the validated identifier into an owned `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RoutingPluginName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RoutingPluginName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Plugin version
// =============================================================================

/// Validated plugin implementation version.
///
/// The registry deliberately treats the version as an opaque stable string.
/// Semantic-version parsing is not required at the registry boundary because
/// plugins may use versions such as:
///
/// ```text
/// 1.2.3
/// 2026.08
/// research-7
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingPluginVersion(String);

impl RoutingPluginVersion {
    /// Creates and validates a plugin version string.
    pub fn new(version: impl Into<String>) -> Result<Self, PluginError> {
        let version = version.into();

        if version.is_empty() {
            return Err(PluginError::InvalidVersion {
                version,
                reason: "version must not be empty".to_string(),
            });
        }

        if version.len() > MAX_PLUGIN_VERSION_LENGTH {
            return Err(PluginError::InvalidVersion {
                version,
                reason: format!(
                    "version exceeds maximum length of {} bytes",
                    MAX_PLUGIN_VERSION_LENGTH
                ),
            });
        }

        if !version.is_ascii() {
            return Err(PluginError::InvalidVersion {
                version,
                reason: "version must contain only ASCII characters".to_string(),
            });
        }

        if version.chars().any(|character| character.is_control()) {
            return Err(PluginError::InvalidVersion {
                version,
                reason: "version must not contain control characters".to_string(),
            });
        }

        Ok(Self(version))
    }

    /// Returns the version.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RoutingPluginVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RoutingPluginVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Plugin metadata
// =============================================================================

/// Immutable metadata describing one routing plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPluginMetadata {
    /// Stable plugin identifier.
    name: RoutingPluginName,

    /// Plugin implementation version.
    version: RoutingPluginVersion,

    /// Version of this routing-plugin API.
    api_version: u32,

    /// Optional author/vendor identity.
    author: Option<String>,

    /// Optional human-readable description.
    description: Option<String>,

    /// Declared routing capabilities.
    capabilities: RoutingAlgorithmCapabilities,
}

impl RoutingPluginMetadata {
    /// Creates metadata for the current plugin API version.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        capabilities: RoutingAlgorithmCapabilities,
    ) -> Result<Self, PluginError> {
        Self::with_api_version(
            name,
            version,
            ROUTING_PLUGIN_API_VERSION,
            capabilities,
        )
    }

    /// Creates metadata with an explicitly declared plugin API version.
    pub fn with_api_version(
        name: impl Into<String>,
        version: impl Into<String>,
        api_version: u32,
        capabilities: RoutingAlgorithmCapabilities,
    ) -> Result<Self, PluginError> {
        let name = RoutingPluginName::new(name)?;
        let version = RoutingPluginVersion::new(version)?;

        if api_version == 0 {
            return Err(PluginError::UnsupportedApiVersion {
                requested: api_version,
                supported: ROUTING_PLUGIN_API_VERSION,
            });
        }

        Ok(Self {
            name,
            version,
            api_version,
            author: None,
            description: None,
            capabilities,
        })
    }

    /// Adds author/vendor metadata.
    ///
    /// This method returns a new metadata value and does not mutate a
    /// previously stored descriptor.
    pub fn with_author(
        mut self,
        author: impl Into<String>,
    ) -> Result<Self, PluginError> {
        let author = author.into();

        validate_metadata_string(
            &author,
            MAX_PLUGIN_AUTHOR_LENGTH,
            "author",
        )?;

        self.author = Some(author);
        Ok(self)
    }

    /// Adds a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, PluginError> {
        let description = description.into();

        validate_metadata_string(
            &description,
            MAX_PLUGIN_DESCRIPTION_LENGTH,
            "description",
        )?;

        self.description = Some(description);
        Ok(self)
    }

    /// Returns the stable plugin name.
    #[must_use]
    pub fn name(&self) -> &RoutingPluginName {
        &self.name
    }

    /// Returns the implementation version.
    #[must_use]
    pub fn version(&self) -> &RoutingPluginVersion {
        &self.version
    }

    /// Returns the plugin API version.
    #[must_use]
    pub const fn api_version(&self) -> u32 {
        self.api_version
    }

    /// Returns the optional author/vendor.
    #[must_use]
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns declared algorithm capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> RoutingAlgorithmCapabilities {
        self.capabilities
    }

    /// Validates this metadata against the current registry API.
    pub fn validate(&self) -> Result<(), PluginError> {
        if self.api_version != ROUTING_PLUGIN_API_VERSION {
            return Err(PluginError::UnsupportedApiVersion {
                requested: self.api_version,
                supported: ROUTING_PLUGIN_API_VERSION,
            });
        }

        validate_identifier(
            self.name.as_str(),
            MAX_PLUGIN_NAME_LENGTH,
            "plugin name",
        )?;

        validate_metadata_string(
            self.version.as_str(),
            MAX_PLUGIN_VERSION_LENGTH,
            "version",
        )?;

        if let Some(author) = &self.author {
            validate_metadata_string(
                author,
                MAX_PLUGIN_AUTHOR_LENGTH,
                "author",
            )?;
        }

        if let Some(description) = &self.description {
            validate_metadata_string(
                description,
                MAX_PLUGIN_DESCRIPTION_LENGTH,
                "description",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Plugin factory
// =============================================================================

/// Factory used to create a fresh routing algorithm instance.
///
/// A factory is intentionally represented as an `Arc`-owned closure so the
/// registry can cheaply clone descriptors without requiring a concrete
/// algorithm type.
///
/// The factory must be safe Rust and should create an independent execution
/// instance for each request.
pub type RoutingPluginFactory = Arc<
    dyn Fn() -> RoutingResult<Box<dyn RoutingAlgorithm>>
        + Send
        + Sync
        + 'static,
>;

// =============================================================================
// Plugin descriptor
// =============================================================================

/// Complete immutable registration record for a routing plugin.
///
/// A descriptor combines:
///
/// - metadata;
/// - factory;
/// - optional compatibility predicate.
///
/// The descriptor itself contains no mutable plugin state.
pub struct RoutingPluginDescriptor {
    metadata: RoutingPluginMetadata,
    factory: RoutingPluginFactory,
}

impl RoutingPluginDescriptor {
    /// Creates a plugin descriptor.
    pub fn new(
        metadata: RoutingPluginMetadata,
        factory: RoutingPluginFactory,
    ) -> Result<Self, PluginError> {
        metadata.validate()?;

        let descriptor = Self {
            metadata,
            factory,
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Convenience constructor for a factory function/closure.
    pub fn from_factory<F>(
        metadata: RoutingPluginMetadata,
        factory: F,
    ) -> Result<Self, PluginError>
    where
        F: Fn() -> RoutingResult<Box<dyn RoutingAlgorithm>>
            + Send
            + Sync
            + 'static,
    {
        Self::new(metadata, Arc::new(factory))
    }

    /// Creates a descriptor from a concrete algorithm constructor.
    ///
    /// The constructor must return a fresh implementation for every call.
    pub fn from_constructor<A, F>(
        metadata: RoutingPluginMetadata,
        constructor: F,
    ) -> Result<Self, PluginError>
    where
        A: RoutingAlgorithm + 'static,
        F: Fn() -> A + Send + Sync + 'static,
    {
        Self::from_factory(metadata, move || {
            Ok(Box::new(constructor()) as Box<dyn RoutingAlgorithm>)
        })
    }

    /// Returns immutable plugin metadata.
    #[must_use]
    pub fn metadata(&self) -> &RoutingPluginMetadata {
        &self.metadata
    }

    /// Returns the plugin factory.
    #[must_use]
    pub fn factory(&self) -> &RoutingPluginFactory {
        &self.factory
    }

    /// Validates the descriptor.
    ///
    /// Validation intentionally does not execute the factory. Registration must
    /// remain side-effect free with respect to algorithm construction.
    pub fn validate(&self) -> Result<(), PluginError> {
        self.metadata.validate()?;

        if self.metadata.capabilities.supports_two_qubit == false {
            return Err(PluginError::InvalidCapabilities {
                plugin: self.metadata.name().to_string(),
                reason:
                    "routing plugins must declare two-qubit routing support"
                        .to_string(),
            });
        }

        Ok(())
    }

    /// Instantiates a fresh routing algorithm.
    pub fn create(
        &self,
    ) -> Result<Box<dyn RoutingAlgorithm>, PluginError> {
        let algorithm = (self.factory)().map_err(|error| {
            PluginError::FactoryFailed {
                plugin: self.metadata.name().to_string(),
                error: error.to_string(),
            }
        })?;

        if algorithm.name().is_empty() {
            return Err(PluginError::InvalidImplementation {
                plugin: self.metadata.name().to_string(),
                reason: "algorithm returned an empty name".to_string(),
            });
        }

        Ok(algorithm)
    }
}

impl Clone for RoutingPluginDescriptor {
    fn clone(&self) -> Self {
        Self {
            metadata: self.metadata.clone(),
            factory: Arc::clone(&self.factory),
        }
    }
}

impl fmt::Debug for RoutingPluginDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RoutingPluginDescriptor")
            .field("metadata", &self.metadata)
            .field("factory", &"<opaque>")
            .finish()
    }
}

// =============================================================================
// Registry snapshot
// =============================================================================

/// Immutable deterministic view of registered plugins.
///
/// A snapshot is useful when routing is running and another part of an
/// application wants to inspect the available plugin set without obtaining
/// mutable access to the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPluginInfo {
    /// Stable plugin name.
    pub name: String,

    /// Plugin implementation version.
    pub version: String,

    /// Plugin API version.
    pub api_version: u32,

    /// Optional author/vendor.
    pub author: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Declared capabilities.
    pub capabilities: RoutingAlgorithmCapabilities,
}

impl From<&RoutingPluginDescriptor> for RoutingPluginInfo {
    fn from(descriptor: &RoutingPluginDescriptor) -> Self {
        let metadata = descriptor.metadata();

        Self {
            name: metadata.name().to_string(),
            version: metadata.version().to_string(),
            api_version: metadata.api_version(),
            author: metadata.author().map(str::to_owned),
            description: metadata.description().map(str::to_owned),
            capabilities: metadata.capabilities(),
        }
    }
}

/// Immutable snapshot of a plugin registry.
///
/// This snapshot contains metadata only and therefore cannot execute plugins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPluginRegistrySnapshot {
    plugins: Vec<RoutingPluginInfo>,
}

impl RoutingPluginRegistrySnapshot {
    /// Creates a snapshot from metadata records.
    fn new(mut plugins: Vec<RoutingPluginInfo>) -> Self {
        plugins.sort_by(|left, right| left.name.cmp(&right.name));

        Self { plugins }
    }

    /// Returns registered plugins in deterministic name order.
    #[must_use]
    pub fn plugins(&self) -> &[RoutingPluginInfo] {
        &self.plugins
    }

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

    /// Finds metadata for one plugin.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&RoutingPluginInfo> {
        self.plugins
            .binary_search_by(|plugin| plugin.name.as_str().cmp(name))
            .ok()
            .map(|index| &self.plugins[index])
    }
}

// =============================================================================
// Plugin registry
// =============================================================================

/// Explicit owner of routing plugins.
///
/// The registry deliberately has no global singleton.
///
/// A compiler, application, test, or routing context should create the registry
/// it needs and pass it explicitly to the routing layer.
#[derive(Debug, Clone, Default)]
pub struct RoutingPluginRegistry {
    plugins: BTreeMap<RoutingPluginName, RoutingPluginDescriptor>,
}

impl RoutingPluginRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
        }
    }

    /// Returns the number of registered plugins.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns whether the registry contains no plugins.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Registers a plugin descriptor.
    ///
    /// Duplicate names are rejected. The existing plugin is never replaced
    /// implicitly.
    pub fn register(
        &mut self,
        descriptor: RoutingPluginDescriptor,
    ) -> Result<(), PluginError> {
        descriptor.validate()?;

        let name = descriptor.metadata().name().clone();

        if self.plugins.contains_key(&name) {
            return Err(PluginError::AlreadyRegistered {
                name: name.to_string(),
            });
        }

        self.plugins.insert(name, descriptor);
        Ok(())
    }

    /// Registers a plugin from metadata and a factory.
    pub fn register_factory<F>(
        &mut self,
        metadata: RoutingPluginMetadata,
        factory: F,
    ) -> Result<(), PluginError>
    where
        F: Fn() -> RoutingResult<Box<dyn RoutingAlgorithm>>
            + Send
            + Sync
            + 'static,
    {
        let descriptor =
            RoutingPluginDescriptor::from_factory(metadata, factory)?;

        self.register(descriptor)
    }

    /// Registers a concrete algorithm constructor.
    ///
    /// The constructor is called whenever the plugin is instantiated.
    pub fn register_constructor<A, F>(
        &mut self,
        metadata: RoutingPluginMetadata,
        constructor: F,
    ) -> Result<(), PluginError>
    where
        A: RoutingAlgorithm + 'static,
        F: Fn() -> A + Send + Sync + 'static,
    {
        let descriptor =
            RoutingPluginDescriptor::from_constructor(
                metadata,
                constructor,
            )?;

        self.register(descriptor)
    }

    /// Removes a registered plugin.
    ///
    /// Removing a plugin affects only this registry. Existing algorithm
    /// instances remain owned by their callers.
    pub fn unregister(
        &mut self,
        name: &str,
    ) -> Result<RoutingPluginDescriptor, PluginError> {
        let name = RoutingPluginName::new(name)?;

        self.plugins.remove(&name).ok_or_else(|| {
            PluginError::NotRegistered {
                name: name.to_string(),
            }
        })
    }

    /// Returns whether a plugin is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.plugins
            .keys()
            .any(|plugin_name| plugin_name.as_str() == name)
    }

    /// Returns immutable metadata for a registered plugin.
    #[must_use]
    pub fn metadata(
        &self,
        name: &str,
    ) -> Option<&RoutingPluginMetadata> {
        self.plugins
            .iter()
            .find(|(plugin_name, _)| plugin_name.as_str() == name)
            .map(|(_, descriptor)| descriptor.metadata())
    }

    /// Returns a registered descriptor.
    #[must_use]
    pub fn descriptor(
        &self,
        name: &str,
    ) -> Option<&RoutingPluginDescriptor> {
        self.plugins
            .iter()
            .find(|(plugin_name, _)| plugin_name.as_str() == name)
            .map(|(_, descriptor)| descriptor)
    }

    /// Creates a fresh algorithm instance from a plugin.
    ///
    /// Configuration compatibility is checked before factory execution.
    pub fn create(
        &self,
        name: &str,
        config: &RoutingConfig,
    ) -> Result<Box<dyn RoutingAlgorithm>, PluginError> {
        let descriptor = self
            .descriptor(name)
            .ok_or_else(|| PluginError::NotRegistered {
                name: name.to_string(),
            })?;

        ensure_api_compatible(descriptor.metadata())?;
        ensure_configuration_compatible(
            descriptor.metadata(),
            config,
        )?;

        let algorithm = descriptor.create()?;

        if !algorithm.supports(config) {
            return Err(PluginError::ConfigurationUnsupported {
                plugin: descriptor.metadata().name().to_string(),
                algorithm: algorithm.name().to_string(),
                configuration: config.algorithm().to_string(),
            });
        }

        Ok(algorithm)
    }

    /// Creates a fresh algorithm without configuration compatibility checking.
    ///
    /// This is intended for discovery, testing, and callers that perform their
    /// own compatibility validation.
    pub fn create_unchecked_config(
        &self,
        name: &str,
    ) -> Result<Box<dyn RoutingAlgorithm>, PluginError> {
        let descriptor = self
            .descriptor(name)
            .ok_or_else(|| PluginError::NotRegistered {
                name: name.to_string(),
            })?;

        ensure_api_compatible(descriptor.metadata())?;

        descriptor.create()
    }

    /// Returns all plugin metadata in deterministic order.
    #[must_use]
    pub fn snapshot(&self) -> RoutingPluginRegistrySnapshot {
        let plugins = self
            .plugins
            .values()
            .map(RoutingPluginInfo::from)
            .collect();

        RoutingPluginRegistrySnapshot::new(plugins)
    }

    /// Returns all registered plugin names in deterministic order.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.plugins
            .keys()
            .map(|name| name.as_str().to_owned())
            .collect()
    }

    /// Validates every descriptor in the registry.
    ///
    /// This is useful immediately before a production compilation context is
    /// activated.
    pub fn validate(&self) -> Result<(), PluginError> {
        for descriptor in self.plugins.values() {
            descriptor.validate()?;
        }

        Ok(())
    }

    /// Registers all descriptors from another registry.
    ///
    /// The operation is transactional with respect to this registry:
    ///
    /// - either every plugin is registered;
    /// - or no plugin from `other` is registered.
    ///
    /// Existing plugins are never overwritten.
    pub fn extend(
        &mut self,
        other: &RoutingPluginRegistry,
    ) -> Result<(), PluginError> {
        for descriptor in other.plugins.values() {
            descriptor.validate()?;

            let name = descriptor.metadata().name();

            if self.plugins.contains_key(name) {
                return Err(PluginError::AlreadyRegistered {
                    name: name.to_string(),
                });
            }
        }

        for descriptor in other.plugins.values() {
            let name = descriptor.metadata().name().clone();
            self.plugins.insert(name, descriptor.clone());
        }

        Ok(())
    }
}

// =============================================================================
// Configuration compatibility
// =============================================================================

/// Validates plugin API compatibility.
fn ensure_api_compatible(
    metadata: &RoutingPluginMetadata,
) -> Result<(), PluginError> {
    if metadata.api_version() != ROUTING_PLUGIN_API_VERSION {
        return Err(PluginError::UnsupportedApiVersion {
            requested: metadata.api_version(),
            supported: ROUTING_PLUGIN_API_VERSION,
        });
    }

    Ok(())
}

/// Validates broad configuration compatibility from declared capabilities.
///
/// Concrete algorithm support is still checked after instantiation through
/// `RoutingAlgorithm::supports`.
fn ensure_configuration_compatible(
    metadata: &RoutingPluginMetadata,
    config: &RoutingConfig,
) -> Result<(), PluginError> {
    let capabilities = metadata.capabilities();

    let algorithm = config.algorithm();

    if config.deterministic() && !capabilities.supports_deterministic_trials {
        return Err(PluginError::ConfigurationUnsupported {
            plugin: metadata.name().to_string(),
            algorithm: algorithm.to_string(),
            configuration:
                "deterministic routing was requested but the plugin does not declare deterministic support"
                    .to_string(),
        });
    }

    if config.lookahead_depth() > 0 && !capabilities.supports_lookahead {
        return Err(PluginError::ConfigurationUnsupported {
            plugin: metadata.name().to_string(),
            algorithm: algorithm.to_string(),
            configuration:
                "lookahead configuration was requested but the plugin does not declare lookahead support"
                    .to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum_length: usize,
    kind: &str,
) -> Result<(), PluginError> {
    if value.is_empty() {
        return Err(PluginError::InvalidName {
            name: value.to_string(),
            reason: format!("{kind} must not be empty"),
        });
    }

    if value.len() > maximum_length {
        return Err(PluginError::InvalidName {
            name: value.to_string(),
            reason: format!(
                "{kind} exceeds maximum length of {maximum_length} bytes"
            ),
        });
    }

    if !value.is_ascii() {
        return Err(PluginError::InvalidName {
            name: value.to_string(),
            reason: format!("{kind} must contain only ASCII characters"),
        });
    }

    let mut characters = value.chars();

    let first = characters.next().ok_or_else(|| {
        PluginError::InvalidName {
            name: value.to_string(),
            reason: format!("{kind} must not be empty"),
        }
    })?;

    if !first.is_ascii_alphabetic() {
        return Err(PluginError::InvalidName {
            name: value.to_string(),
            reason: format!(
                "{kind} must begin with an ASCII letter"
            ),
        });
    }

    if characters.any(|character| {
        !(character.is_ascii_alphanumeric()
            || matches!(character, '_' | '-' | '.'))
    }) {
        return Err(PluginError::InvalidName {
            name: value.to_string(),
            reason: format!(
                "{kind} contains an unsupported character"
            ),
        });
    }

    Ok(())
}

fn validate_metadata_string(
    value: &str,
    maximum_length: usize,
    kind: &str,
) -> Result<(), PluginError> {
    if value.len() > maximum_length {
        return Err(PluginError::InvalidMetadata {
            field: kind.to_string(),
            reason: format!(
                "value exceeds maximum length of {maximum_length} bytes"
            ),
        });
    }

    if !value.is_ascii() {
        return Err(PluginError::InvalidMetadata {
            field: kind.to_string(),
            reason: "value must contain only ASCII characters".to_string(),
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(PluginError::InvalidMetadata {
            field: kind.to_string(),
            reason: "value must not contain control characters".to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Plugin errors
// =============================================================================

/// Errors specific to plugin registration and lifecycle management.
///
/// These errors remain local to `plugins.rs` because plugin-management failures
/// are distinct from failures encountered while actually executing a routing
/// algorithm.
///
/// When a plugin is selected by the main routing engine, callers may translate
/// these errors into the canonical `RoutingError::Algorithm(...)` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginError {
    /// Plugin identifier is invalid.
    InvalidName {
        /// Supplied identifier.
        name: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Plugin implementation version is invalid.
    InvalidVersion {
        /// Supplied version.
        version: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Metadata contains an invalid field.
    InvalidMetadata {
        /// Field name.
        field: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Capability declaration is invalid.
    InvalidCapabilities {
        /// Plugin name.
        plugin: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Plugin API version is not supported.
    UnsupportedApiVersion {
        /// Version requested by plugin.
        requested: u32,

        /// API version supported by this registry.
        supported: u32,
    },

    /// A plugin with the same name already exists.
    AlreadyRegistered {
        /// Duplicate plugin name.
        name: String,
    },

    /// Requested plugin is absent.
    NotRegistered {
        /// Requested plugin name.
        name: String,
    },

    /// Plugin factory failed to create an algorithm.
    FactoryFailed {
        /// Plugin name.
        plugin: String,

        /// Factory error detail.
        error: String,
    },

    /// Factory returned an invalid implementation.
    InvalidImplementation {
        /// Plugin name.
        plugin: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Plugin cannot satisfy the requested configuration.
    ConfigurationUnsupported {
        /// Plugin name.
        plugin: String,

        /// Concrete algorithm name.
        algorithm: String,

        /// Configuration incompatibility.
        configuration: String,
    },

    /// Plugin registry invariant was violated.
    RegistryInvariantViolation {
        /// Diagnostic detail.
        detail: String,
    },
}

impl fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { name, reason } => write!(
                formatter,
                "invalid routing plugin name `{name}`: {reason}"
            ),

            Self::InvalidVersion { version, reason } => write!(
                formatter,
                "invalid routing plugin version `{version}`: {reason}"
            ),

            Self::InvalidMetadata { field, reason } => write!(
                formatter,
                "invalid routing plugin metadata `{field}`: {reason}"
            ),

            Self::InvalidCapabilities { plugin, reason } => write!(
                formatter,
                "invalid capabilities for routing plugin `{plugin}`: {reason}"
            ),

            Self::UnsupportedApiVersion {
                requested,
                supported,
            } => write!(
                formatter,
                "routing plugin API version {requested} is unsupported; supported version is {supported}"
            ),

            Self::AlreadyRegistered { name } => write!(
                formatter,
                "routing plugin `{name}` is already registered"
            ),

            Self::NotRegistered { name } => write!(
                formatter,
                "routing plugin `{name}` is not registered"
            ),

            Self::FactoryFailed { plugin, error } => write!(
                formatter,
                "routing plugin `{plugin}` factory failed: {error}"
            ),

            Self::InvalidImplementation { plugin, reason } => write!(
                formatter,
                "routing plugin `{plugin}` produced an invalid implementation: {reason}"
            ),

            Self::ConfigurationUnsupported {
                plugin,
                algorithm,
                configuration,
            } => write!(
                formatter,
                "routing plugin `{plugin}` cannot execute algorithm `{algorithm}` with the requested configuration: {configuration}"
            ),

            Self::RegistryInvariantViolation { detail } => write!(
                formatter,
                "routing plugin registry invariant violation: {detail}"
            ),
        }
    }
}

impl std::error::Error for PluginError {}

// =============================================================================
// Conversion to canonical routing error
// =============================================================================

/// Converts plugin-management failures into the canonical routing algorithm
/// error category.
///
/// This conversion intentionally does not require adding plugin-specific
/// variants to `errors.rs`.
///
/// That keeps `errors.rs` independent from this extension mechanism.
impl From<PluginError> for RoutingError {
    fn from(error: PluginError) -> Self {
        let detail = error.to_string();

        match error {
            PluginError::ConfigurationUnsupported {
                plugin,
                configuration,
                ..
            } => RoutingError::Algorithm(
                AlgorithmError::Incompatible {
                    algorithm: plugin,
                    reason: configuration,
                },
            ),

            PluginError::NotRegistered { name } => {
                RoutingError::Algorithm(
                    AlgorithmError::Unsupported {
                        algorithm: name,
                    },
                )
            }

            PluginError::UnsupportedApiVersion {
                requested,
                supported,
            } => RoutingError::Algorithm(
                AlgorithmError::Incompatible {
                    algorithm: "plugin_api".to_string(),
                    reason: format!(
                        "plugin API version {requested} is unsupported; supported version is {supported}"
                    ),
                },
            ),

            _ => RoutingError::Algorithm(
                AlgorithmError::SearchFailed {
                    algorithm: "plugin_registry".to_string(),
                    detail,
                },
            ),
        }
    }
}

// =============================================================================
// Built-in registration helpers
// =============================================================================

/// Registers a built-in routing algorithm under a stable custom-plugin name.
///
/// This helper is useful when the router wants to expose a built-in algorithm
/// through the same registry mechanism as custom algorithms.
///
/// The registry does not instantiate the algorithm until `create()` is called.
pub fn register_algorithm<A, F>(
    registry: &mut RoutingPluginRegistry,
    name: impl Into<String>,
    version: impl Into<String>,
    capabilities: RoutingAlgorithmCapabilities,
    constructor: F,
) -> Result<(), PluginError>
where
    A: RoutingAlgorithm + 'static,
    F: Fn() -> A + Send + Sync + 'static,
{
    let metadata =
        RoutingPluginMetadata::new(name, version, capabilities)?;

    registry.register_constructor::<A, F>(
        metadata,
        constructor,
    )
}

/// Registers an algorithm with richer metadata.
pub fn register_algorithm_with_metadata<A, F>(
    registry: &mut RoutingPluginRegistry,
    metadata: RoutingPluginMetadata,
    constructor: F,
) -> Result<(), PluginError>
where
    A: RoutingAlgorithm + 'static,
    F: Fn() -> A + Send + Sync + 'static,
{
    registry.register_constructor::<A, F>(
        metadata,
        constructor,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::routing::config::RoutingConfig;
    use crate::quantum::routing::types::RoutingAlgorithmCapabilities;

    /// Minimal test algorithm used to exercise the plugin registry without
    /// coupling the tests to a concrete production routing implementation.
    #[derive(Debug, Clone, Copy)]
    struct TestRoutingAlgorithm;

    impl RoutingAlgorithm for TestRoutingAlgorithm {
        fn name(&self) -> &'static str {
            "test"
        }

        fn route(
            &self,
            _input: &crate::quantum::routing::types::RoutingInput,
            _config: &RoutingConfig,
        ) -> Result<
            crate::quantum::routing::result::RoutingResult,
            RoutingError,
        > {
            Err(RoutingError::Algorithm(
                AlgorithmError::NoValidResult {
                    algorithm: "test".to_string(),
                },
            ))
        }

        fn supports(&self, _config: &RoutingConfig) -> bool {
            true
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }
    }

    fn test_capabilities() -> RoutingAlgorithmCapabilities {
        RoutingAlgorithmCapabilities::BASIC
    }

    fn test_metadata(
        name: &str,
    ) -> RoutingPluginMetadata {
        RoutingPluginMetadata::new(
            name,
            "1.0.0",
            test_capabilities(),
        )
        .expect("test metadata should be valid")
    }

    #[test]
    fn valid_plugin_name_is_accepted() {
        let name = RoutingPluginName::new("research_router_v1")
            .expect("valid name should be accepted");

        assert_eq!(
            name.as_str(),
            "research_router_v1"
        );
    }

    #[test]
    fn plugin_name_must_start_with_letter() {
        let result = RoutingPluginName::new("1router");

        assert!(matches!(
            result,
            Err(PluginError::InvalidName { .. })
        ));
    }

    #[test]
    fn plugin_name_rejects_spaces() {
        let result = RoutingPluginName::new("my router");

        assert!(matches!(
            result,
            Err(PluginError::InvalidName { .. })
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("test_router"),
                || TestRoutingAlgorithm,
            )
            .expect("first registration should succeed");

        let result =
            registry.register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("test_router"),
                || TestRoutingAlgorithm,
            );

        assert!(matches!(
            result,
            Err(PluginError::AlreadyRegistered { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registration_is_deterministically_sorted() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("z_router"),
                || TestRoutingAlgorithm,
            )
            .expect("z registration should succeed");

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("a_router"),
                || TestRoutingAlgorithm,
            )
            .expect("a registration should succeed");

        let names = registry.names();

        assert_eq!(
            names,
            vec![
                "a_router".to_string(),
                "z_router".to_string(),
            ]
        );
    }

    #[test]
    fn snapshot_is_sorted() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("z_router"),
                || TestRoutingAlgorithm,
            )
            .expect("z registration should succeed");

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("a_router"),
                || TestRoutingAlgorithm,
            )
            .expect("a registration should succeed");

        let snapshot = registry.snapshot();

        assert_eq!(snapshot.len(), 2);
        assert_eq!(
            snapshot.plugins()[0].name,
            "a_router"
        );
        assert_eq!(
            snapshot.plugins()[1].name,
            "z_router"
        );
    }

    #[test]
    fn plugin_can_be_created_as_fresh_instance() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("test_router"),
                || TestRoutingAlgorithm,
            )
            .expect("registration should succeed");

        let config = RoutingConfig::default();

        let first = registry
            .create("test_router", &config)
            .expect("plugin should instantiate");

        let second = registry
            .create("test_router", &config)
            .expect("plugin should instantiate");

        assert_eq!(first.name(), "test");
        assert_eq!(second.name(), "test");
    }

    #[test]
    fn unknown_plugin_is_rejected() {
        let registry = RoutingPluginRegistry::new();
        let config = RoutingConfig::default();

        let result = registry.create(
            "does_not_exist",
            &config,
        );

        assert!(matches!(
            result,
            Err(PluginError::NotRegistered { .. })
        ));
    }

    #[test]
    fn unregister_removes_plugin() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("temporary"),
                || TestRoutingAlgorithm,
            )
            .expect("registration should succeed");

        assert!(registry.contains("temporary"));

        registry
            .unregister("temporary")
            .expect("unregister should succeed");

        assert!(!registry.contains("temporary"));
        assert!(registry.is_empty());
    }

    #[test]
    fn unregister_unknown_plugin_fails() {
        let mut registry = RoutingPluginRegistry::new();

        let result = registry.unregister("missing");

        assert!(matches!(
            result,
            Err(PluginError::NotRegistered { .. })
        ));
    }

    #[test]
    fn metadata_validation_rejects_wrong_api_version() {
        let result = RoutingPluginMetadata::with_api_version(
            "router",
            "1.0.0",
            ROUTING_PLUGIN_API_VERSION + 1,
            test_capabilities(),
        );

        assert!(matches!(
            result,
            Err(PluginError::UnsupportedApiVersion { .. })
        ));
    }

    #[test]
    fn metadata_is_immutable_after_registration() {
        let mut registry = RoutingPluginRegistry::new();

        registry
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("immutable_router"),
                || TestRoutingAlgorithm,
            )
            .expect("registration should succeed");

        let metadata = registry
            .metadata("immutable_router")
            .expect("metadata should exist");

        assert_eq!(
            metadata.name().as_str(),
            "immutable_router"
        );
        assert_eq!(
            metadata.version().as_str(),
            "1.0.0"
        );
    }

    #[test]
    fn registry_extend_is_transactional_on_duplicate() {
        let mut first = RoutingPluginRegistry::new();
        let mut second = RoutingPluginRegistry::new();

        first
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("existing"),
                || TestRoutingAlgorithm,
            )
            .expect("registration should succeed");

        second
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("new"),
                || TestRoutingAlgorithm,
            )
            .expect("registration should succeed");

        second
            .register_constructor::<TestRoutingAlgorithm, _>(
                test_metadata("existing"),
                || TestRoutingAlgorithm,
            )
            .expect("second registry should allow its own name");

        let result = first.extend(&second);

        assert!(matches!(
            result,
            Err(PluginError::AlreadyRegistered { .. })
        ));

        assert_eq!(first.len(), 1);
        assert!(first.contains("existing"));
        assert!(!first.contains("new"));
    }

    #[test]
    fn plugin_error_converts_to_routing_error() {
        let error =
            PluginError::NotRegistered {
                name: "missing".to_string(),
            };

        let routing_error: RoutingError = error.into();

        assert!(matches!(
            routing_error,
            RoutingError::Algorithm(
                AlgorithmError::Unsupported { .. }
            )
        ));
    }
}