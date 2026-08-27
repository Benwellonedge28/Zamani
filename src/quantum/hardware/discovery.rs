//! Zamani Quantum — Hardware Discovery
//!
//! Production-grade, provider-neutral discovery orchestration for quantum
//! execution targets.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "What quantum execution targets are currently discoverable through the
//! > configured discovery sources, and which of them satisfy a requested
//! > selection policy?"
//!
//! This module owns:
//!
//! - provider-neutral discovery source contracts;
//! - deterministic discovery aggregation;
//! - duplicate backend detection;
//! - backend identity collision detection;
//! - discovery filtering;
//! - capability filtering;
//! - workload compatibility filtering;
//! - status filtering;
//! - provider/region filtering;
//! - resource filtering;
//! - native-instruction filtering;
//! - deterministic result ordering;
//! - bounded discovery;
//! - strict and best-effort discovery policies;
//! - discovery diagnostics;
//! - discovery snapshots;
//! - discovery provenance;
//! - deterministic discovery fingerprints;
//! - source-level failure isolation;
//! - safe source metadata;
//! - discovery service composition.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP/network communication;
//! - provider authentication;
//! - credential storage;
//! - API tokens;
//! - provider SDKs;
//! - job submission;
//! - job execution;
//! - result retrieval;
//! - transpilation;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - benchmark mathematics;
//! - simulation;
//! - emulation;
//! - provider-specific data models.
//!
//! Provider adapters must implement [`DiscoverySource`] and translate their
//! provider-specific device metadata into the canonical
//! [`QuantumBackend`] representation.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Quantum IR
//!                           |
//!                           v
//!                    compatibility
//!                           |
//!                           v
//!                 hardware::discovery
//!                           |
//!             +-------------+-------------+
//!             |             |             |
//!             v             v             v
//!        local source   provider source  config source
//!             |             |             |
//!             +-------------+-------------+
//!                           |
//!                           v
//!                  canonical backends
//!                           |
//!                 +---------+---------+
//!                 |                   |
//!                 v                   v
//!          device registry       provider registry
//!                 |                   |
//!                 +---------+---------+
//!                           |
//!                           v
//!                    compatibility
//!                           |
//!                           v
//!                       execution
//! ```
//!
//! Discovery is therefore an input to registry/selection systems.
//!
//! Discovery MUST NOT depend on benchmarking.
//!
//! Benchmarking may consume discovery results to select and characterize
//! hardware.
//!
//! # Important ownership rule
//!
//! `discovery.rs` does not create a second backend representation.
//!
//! The authoritative backend representation remains:
//!
//! ```text
//! hardware::backend::QuantumBackend
//! ```
//!
//! Discovery produces collections and diagnostics around that canonical type.
//!
//! # Integration contract
//!
//! Current dependencies:
//!
//! - [`super::backend::BackendCapabilities`]
//! - [`super::backend::BackendKind`]
//! - [`super::backend::BackendStatus`]
//! - [`super::backend::QuantumBackend`]
//! - [`super::backend::WorkloadRequirements`]
//!
//! - [`super::topology::HardwareTopology`] indirectly through `QuantumBackend`
//!
//! Future consumers:
//!
//! - `provider_registry.rs`
//! - `device_registry.rs`
//! - `provider.rs`
//! - provider adapters;
//! - `compatibility.rs`;
//! - `validation.rs`;
//! - `execution.rs`;
//! - benchmarking;
//! - Danga.
//!
//! This file intentionally does not require any of those future modules to
//! exist. That guarantees this file can be completed and frozen independently.
//!
//! # Provider integration rule
//!
//! A provider adapter should implement:
//!
//! ```text
//! DiscoverySource
//!       |
//!       v
//! Vec<QuantumBackend>
//!       |
//!       v
//! DiscoveryService
//!       |
//!       v
//! DiscoverySnapshot
//! ```
//!
//! Provider-specific types must never cross the `DiscoverySource` boundary.
//!
//! # Determinism
//!
//! Discovery results are deterministic with respect to a fixed set of source
//! results.
//!
//! Determinism is enforced by:
//!
//! - `BTreeMap` for backend identity indexing;
//! - `BTreeSet` for capability requirements;
//! - stable backend identifiers;
//! - stable source identifiers;
//! - deterministic diagnostic ordering;
//! - deterministic filtering;
//! - deterministic fingerprinting.
//!
//! Discovery never uses random selection.
//!
//! # Security
//!
//! Discovery never stores credentials.
//!
//! Source implementations MUST NOT place secrets into:
//!
//! - backend identifiers;
//! - provider identifiers;
//! - names;
//! - regions;
//! - properties;
//! - discovery diagnostics;
//! - source identifiers.
//!
//! This module also defensively rejects source identifiers and backend
//! identifiers that look like credential-bearing strings when constructing
//! discovery records.
//!
//! # Failure isolation
//!
//! Discovery supports two policies:
//!
//! - [`DiscoveryPolicy::Strict`] — any source failure fails the discovery
//!   operation;
//! - [`DiscoveryPolicy::BestEffort`] — successful sources are retained while
//!   source failures are returned as diagnostics.
//!
//! Best-effort discovery is appropriate for heterogeneous cloud environments.
//!
//! Strict discovery is appropriate when a reproducible deployment requires
//! every configured source to respond successfully.
//!
//! # Snapshot semantics
//!
//! A [`DiscoverySnapshot`] is immutable after construction.
//!
//! It represents the exact canonical set of discovered backends supplied to
//! the caller at that discovery point.
//!
//! Discovery does not claim that a snapshot is permanently current.
//!
//! A later call produces a new snapshot.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! No unsafe code is permitted.
//!
//! No external crate is required by this module.
//!
//! # Thread safety
//!
//! [`DiscoverySource`] requires `Send + Sync` so a discovery service may safely
//! be owned by multithreaded orchestration systems.
//!
//! This module itself contains no global mutable state.
//!
//! # Async boundary
//!
//! This module deliberately uses a synchronous source trait.
//!
//! Network-backed adapters may internally use whatever asynchronous runtime
//! they require, but that runtime must remain below this boundary.
//!
//! This avoids forcing Tokio, async-std, or another runtime onto the canonical
//! hardware abstraction layer.
//!
//! # Versioning
//!
//! Discovery schema:
//!
//! `zamani.quantum.hardware.discovery`
//!
//! Schema version:
//!
//! `1`
//!
//! The schema version describes the semantic discovery contract, not a Rust
//! crate version.
//!
//! # Example
//!
//! ```text
//! let source = MyProviderDiscovery::new(...);
//!
//! let service = DiscoveryService::new()
//!     .with_source(source);
//!
//! let snapshot = service.discover(&DiscoveryQuery::default())?;
//!
//! for backend in snapshot.backends() {
//!     println!("{}", backend.id());
//! }
//! ```
//!
//! The provider implementation is intentionally outside this file.
//!
//! # Relationship with backend.rs
//!
//! `backend.rs` already provides:
//!
//! - canonical backend metadata;
//! - capabilities;
//! - resource limits;
//! - topology;
//! - workload requirements;
//! - backend validation.
//!
//! Discovery consumes those structures rather than redefining them.
//!
//! In particular, discovery never creates a second `BackendCapabilities` or
//! second `BackendStatus` type.
//!
//! # Relationship with topology.rs
//!
//! `QuantumBackend::qubit_count()` and `QuantumBackend::topology()` remain the
//! authoritative topology interface.
//!
//! Discovery only reads topology information.
//!
//! It never mutates topology.
//!
//! # Relationship with future registries
//!
//! `provider_registry.rs` should consume `DiscoverySnapshot::backends()` when
//! refreshing providers.
//!
//! `device_registry.rs` should consume the same snapshot when refreshing
//! device/backend records.
//!
//! Neither registry should implement provider discovery itself.
//!
//! # Relationship with Danga
//!
//! Danga may eventually expose commands such as:
//!
//! ```text
//! danga quantum discover
//! danga quantum devices
//! danga quantum backends
//! danga quantum check
//! ```
//!
//! Those commands should call this module rather than implementing their own
//! discovery protocol.
//!
//! # Relationship with benchmarking
//!
//! Benchmarking may:
//!
//! 1. discover candidates;
//! 2. filter them;
//! 3. validate a benchmark workload;
//! 4. submit through the execution layer;
//! 5. record the discovery snapshot fingerprint as provenance.
//!
//! Hardware discovery itself must remain independent of benchmarking.
//!
//! # Stability rule
//!
//! The following public types form the intended stable contract:
//!
//! - [`DiscoverySource`]
//! - [`DiscoveryService`]
//! - [`DiscoveryQuery`]
//! - [`DiscoveryPolicy`]
//! - [`DiscoverySnapshot`]
//! - [`DiscoveryDiagnostic`]
//! - [`DiscoveryDiagnosticCode`]
//! - [`DiscoveryProvenance`]
//! - [`DiscoveryError`]
//!
//! Provider-specific implementations must not require changes to these types.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use super::backend::{
    BackendCapabilities,
    BackendKind,
    BackendStatus,
    QuantumBackend,
    WorkloadRequirements,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable discovery schema identifier.
pub const DISCOVERY_SCHEMA_ID: &str = "zamani.quantum.hardware.discovery";

/// Stable discovery schema version.
pub const DISCOVERY_SCHEMA_VERSION: u16 = 1;

/// Maximum discovery source identifier length.
pub const MAX_SOURCE_ID_LENGTH: usize = 256;

/// Maximum discovery source name length.
pub const MAX_SOURCE_NAME_LENGTH: usize = 512;

/// Maximum number of discovery sources in one service.
pub const MAX_DISCOVERY_SOURCES: usize = 4096;

/// Maximum number of backends returned by one source.
pub const MAX_BACKENDS_PER_SOURCE: usize = 1_000_000;

/// Maximum number of aggregate backends.
pub const MAX_DISCOVERED_BACKENDS: usize = 2_000_000;

/// Maximum provider filter length.
pub const MAX_PROVIDER_FILTER_LENGTH: usize = 512;

/// Maximum region filter length.
pub const MAX_REGION_FILTER_LENGTH: usize = 256;

/// Maximum backend ID filter length.
pub const MAX_BACKEND_ID_FILTER_LENGTH: usize = 512;

/// Maximum native-instruction filter length.
pub const MAX_INSTRUCTION_FILTER_LENGTH: usize = 256;

/// Maximum diagnostic message length.
pub const MAX_DIAGNOSTIC_MESSAGE_LENGTH: usize = 4096;

// =============================================================================
// Discovery source
// =============================================================================

/// Provider-neutral source of quantum backend descriptors.
///
/// A source is responsible only for obtaining and normalizing backend
/// descriptors.
///
/// Network communication, authentication and provider-specific translation
/// belong inside the implementation.
///
/// # Contract
///
/// Implementations MUST:
///
/// 1. return canonical [`QuantumBackend`] values;
/// 2. never return credentials;
/// 3. never return secret-bearing metadata;
/// 4. return deterministic identifiers;
/// 5. enforce their own provider API limits;
/// 6. return provider failures as [`DiscoveryError`];
/// 7. avoid mutating returned backends after returning them;
/// 8. remain thread-safe.
///
/// Implementations SHOULD:
///
/// - cache provider metadata where appropriate;
/// - respect provider rate limits;
/// - use bounded pagination;
/// - validate provider responses before conversion;
/// - preserve provider error codes inside `DiscoveryError::SourceFailure`.
pub trait DiscoverySource: Send + Sync {
    /// Stable source identifier.
    ///
    /// Examples:
    ///
    /// ```text
    /// local
    /// ibm
    /// ionq
    /// aws-braket
    /// iqm
    /// quantinuum
    /// ```
    fn id(&self) -> &str;

    /// Human-readable source name.
    fn name(&self) -> &str;

    /// Discovers canonical quantum backends.
    fn discover(&self) -> Result<Vec<QuantumBackend>, DiscoveryError>;

    /// Returns whether the source is currently configured.
    ///
    /// This is intentionally separate from whether a discovered backend is
    /// operational.
    fn is_configured(&self) -> bool {
        true
    }
}

// =============================================================================
// Discovery policy
// =============================================================================

/// Policy controlling source failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiscoveryPolicy {
    /// Any source failure causes the entire discovery operation to fail.
    Strict,

    /// Successful sources are retained and source failures are reported.
    BestEffort,
}

impl Default for DiscoveryPolicy {
    fn default() -> Self {
        Self::BestEffort
    }
}

impl DiscoveryPolicy {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::BestEffort => "best_effort",
        }
    }
}

impl fmt::Display for DiscoveryPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Discovery query
// =============================================================================

/// Provider-neutral backend discovery filter.
///
/// Every field is optional.
///
/// An empty query means:
///
/// > discover all configured backends.
///
/// Filters are applied after all source results have been normalized into
/// canonical `QuantumBackend` values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryQuery {
    /// Optional exact provider identifier.
    pub provider: Option<String>,

    /// Optional exact backend identifier.
    pub backend_id: Option<String>,

    /// Optional exact region.
    pub region: Option<String>,

    /// Optional backend kind.
    pub kind: Option<BackendKind>,

    /// Optional status requirement.
    pub status: Option<BackendStatus>,

    /// Minimum number of physical resources.
    pub minimum_qubits: Option<usize>,

    /// Maximum number of physical resources.
    pub maximum_qubits: Option<usize>,

    /// Required stable capability identifiers.
    pub required_capabilities: BTreeSet<String>,

    /// Required native instruction identifiers.
    pub required_instructions: BTreeSet<String>,

    /// Optional workload compatibility requirements.
    pub workload: Option<WorkloadRequirements>,

    /// Require an operational backend.
    pub require_operational: bool,

    /// Require an executable/usable backend status.
    pub require_usable: bool,

    /// Include experimental capabilities in discovery matching.
    ///
    /// This does NOT make experimental capabilities satisfy a stable
    /// capability requirement.
    pub include_experimental: bool,

    /// Maximum number of returned backends.
    ///
    /// `None` means the service default.
    pub limit: Option<usize>,
}

impl Default for DiscoveryQuery {
    fn default() -> Self {
        Self {
            provider: None,
            backend_id: None,
            region: None,
            kind: None,
            status: None,
            minimum_qubits: None,
            maximum_qubits: None,
            required_capabilities: BTreeSet::new(),
            required_instructions: BTreeSet::new(),
            workload: None,
            require_operational: false,
            require_usable: false,
            include_experimental: false,
            limit: None,
        }
    }
}

impl DiscoveryQuery {
    /// Creates an unrestricted discovery query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts discovery to one provider.
    pub fn with_provider(
        mut self,
        provider: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let provider = normalize_filter(
            "provider",
            provider.into(),
            MAX_PROVIDER_FILTER_LENGTH,
        )?;

        self.provider = Some(provider);
        Ok(self)
    }

    /// Restricts discovery to one backend ID.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let backend_id = normalize_filter(
            "backend_id",
            backend_id.into(),
            MAX_BACKEND_ID_FILTER_LENGTH,
        )?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Restricts discovery to one region.
    pub fn with_region(
        mut self,
        region: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let region = normalize_filter(
            "region",
            region.into(),
            MAX_REGION_FILTER_LENGTH,
        )?;

        self.region = Some(region);
        Ok(self)
    }

    /// Restricts discovery to one backend kind.
    pub fn with_kind(mut self, kind: BackendKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Restricts discovery to one status.
    pub fn with_status(mut self, status: BackendStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Requires at least `count` physical resources.
    pub fn with_minimum_qubits(mut self, count: usize) -> Self {
        self.minimum_qubits = Some(count);
        self
    }

    /// Requires at most `count` physical resources.
    pub fn with_maximum_qubits(mut self, count: usize) -> Self {
        self.maximum_qubits = Some(count);
        self
    }

    /// Requires one stable capability.
    pub fn require_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let capability = normalize_filter(
            "capability",
            capability.into(),
            MAX_SOURCE_NAME_LENGTH,
        )?;

        self.required_capabilities
            .insert(normalize_name(&capability));

        Ok(self)
    }

    /// Requires one native instruction.
    pub fn require_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Result<Self, DiscoveryError> {
        let instruction = normalize_filter(
            "instruction",
            instruction.into(),
            MAX_INSTRUCTION_FILTER_LENGTH,
        )?;

        self.required_instructions
            .insert(normalize_name(&instruction));

        Ok(self)
    }

    /// Requires compatibility with a workload.
    pub fn with_workload(
        mut self,
        workload: WorkloadRequirements,
    ) -> Result<Self, DiscoveryError> {
        workload
            .validate()
            .map_err(DiscoveryError::InvalidWorkload)?;

        self.workload = Some(workload);
        Ok(self)
    }

    /// Requires an operational backend.
    pub fn operational(mut self) -> Self {
        self.require_operational = true;
        self
    }

    /// Requires a currently usable backend.
    pub fn usable(mut self) -> Self {
        self.require_usable = true;
        self
    }

    /// Allows experimental capabilities to appear in discovery metadata.
    pub fn include_experimental(mut self) -> Self {
        self.include_experimental = true;
        self
    }

    /// Sets the maximum result count.
    pub fn with_limit(
        mut self,
        limit: usize,
    ) -> Result<Self, DiscoveryError> {
        if limit == 0 {
            return Err(DiscoveryError::InvalidQuery(
                "discovery result limit must be greater than zero".to_string(),
            ));
        }

        if limit > MAX_DISCOVERED_BACKENDS {
            return Err(DiscoveryError::LimitExceeded {
                requested: limit,
                maximum: MAX_DISCOVERED_BACKENDS,
            });
        }

        self.limit = Some(limit);
        Ok(self)
    }

    /// Validates the query.
    pub fn validate(&self) -> Result<(), DiscoveryError> {
        if let (Some(minimum), Some(maximum)) =
            (self.minimum_qubits, self.maximum_qubits)
        {
            if minimum > maximum {
                return Err(DiscoveryError::InvalidQuery(
                    "minimum_qubits cannot exceed maximum_qubits".to_string(),
                ));
            }
        }

        if self.required_capabilities.len() > MAX_DISCOVERED_BACKENDS {
            return Err(DiscoveryError::LimitExceeded {
                requested: self.required_capabilities.len(),
                maximum: MAX_DISCOVERED_BACKENDS,
            });
        }

        if self.required_instructions.len() > MAX_DISCOVERED_BACKENDS {
            return Err(DiscoveryError::LimitExceeded {
                requested: self.required_instructions.len(),
                maximum: MAX_DISCOVERED_BACKENDS,
            });
        }

        if let Some(workload) = &self.workload {
            workload
                .validate()
                .map_err(DiscoveryError::InvalidWorkload)?;
        }

        Ok(())
    }
}

// =============================================================================
// Diagnostics
// =============================================================================

/// Stable discovery diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiscoveryDiagnosticCode {
    /// A source was not configured.
    SourceNotConfigured,

    /// A source failed.
    SourceFailure,

    /// A source returned too many backends.
    SourceLimitExceeded,

    /// A source returned a duplicate backend.
    DuplicateBackend,

    /// Two sources returned the same backend ID with different descriptors.
    BackendIdentityCollision,

    /// A backend was rejected because it did not match a query.
    BackendFiltered,

    /// A backend exceeded the aggregate discovery limit.
    AggregateLimitExceeded,

    /// A query was invalid.
    InvalidQuery,

    /// A source returned an invalid backend descriptor.
    InvalidBackend,

    /// An experimental capability was present.
    ExperimentalCapabilityPresent,

    /// A backend had an unknown status.
    UnknownBackendStatus,
}

impl DiscoveryDiagnosticCode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceNotConfigured => "source.not_configured",
            Self::SourceFailure => "source.failure",
            Self::SourceLimitExceeded => "source.limit_exceeded",
            Self::DuplicateBackend => "backend.duplicate",
            Self::BackendIdentityCollision => "backend.identity_collision",
            Self::BackendFiltered => "backend.filtered",
            Self::AggregateLimitExceeded => "aggregate.limit_exceeded",
            Self::InvalidQuery => "query.invalid",
            Self::InvalidBackend => "backend.invalid",
            Self::ExperimentalCapabilityPresent => {
                "backend.experimental_capability"
            }
            Self::UnknownBackendStatus => "backend.status.unknown",
        }
    }
}

impl fmt::Display for DiscoveryDiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One discovery diagnostic.
///
/// Diagnostics never contain credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryDiagnostic {
    /// Stable diagnostic code.
    pub code: DiscoveryDiagnosticCode,

    /// Source identifier, if applicable.
    pub source_id: Option<String>,

    /// Backend identifier, if applicable.
    pub backend_id: Option<String>,

    /// Human-readable diagnostic.
    pub message: String,

    /// Whether the diagnostic prevented a backend from being included.
    pub blocking: bool,
}

impl DiscoveryDiagnostic {
    fn new(
        code: DiscoveryDiagnosticCode,
        source_id: Option<String>,
        backend_id: Option<String>,
        message: impl Into<String>,
        blocking: bool,
    ) -> Self {
        Self {
            code,
            source_id,
            backend_id,
            message: truncate_message(message.into()),
            blocking,
        }
    }

    /// Returns true when the diagnostic prevented successful discovery.
    pub const fn is_blocking(&self) -> bool {
        self.blocking
    }
}

impl fmt::Display for DiscoveryDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}",
            self.code,
            self.message
        )
    }
}

// =============================================================================
// Discovery provenance
// =============================================================================

/// Immutable provenance information for a discovery operation.
///
/// This does not contain timestamps because this module intentionally does not
/// read the system clock. Callers that need wall-clock timestamps may attach
/// them at a higher layer.
///
/// This makes the discovery core deterministic and testable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryProvenance {
    /// Discovery schema identifier.
    pub schema_id: &'static str,

    /// Discovery schema version.
    pub schema_version: u16,

    /// Discovery policy used.
    pub policy: DiscoveryPolicy,

    /// Source identifiers participating in discovery.
    pub source_ids: Vec<String>,

    /// Number of successfully queried sources.
    pub successful_sources: usize,

    /// Number of failed sources.
    pub failed_sources: usize,

    /// Number of raw backend descriptors returned by sources.
    pub raw_backend_count: usize,

    /// Number of unique backend descriptors retained.
    pub unique_backend_count: usize,

    /// Number of filtered backends.
    pub filtered_backend_count: usize,

    /// Deterministic fingerprint of the resulting snapshot.
    pub fingerprint: String,
}

impl DiscoveryProvenance {
    fn new(
        policy: DiscoveryPolicy,
        source_ids: Vec<String>,
        successful_sources: usize,
        failed_sources: usize,
        raw_backend_count: usize,
        unique_backend_count: usize,
        filtered_backend_count: usize,
        fingerprint: String,
    ) -> Self {
        Self {
            schema_id: DISCOVERY_SCHEMA_ID,
            schema_version: DISCOVERY_SCHEMA_VERSION,
            policy,
            source_ids,
            successful_sources,
            failed_sources,
            raw_backend_count,
            unique_backend_count,
            filtered_backend_count,
            fingerprint,
        }
    }
}

// =============================================================================
// Discovery snapshot
// =============================================================================

/// Immutable result of one discovery operation.
#[derive(Debug, Clone)]
pub struct DiscoverySnapshot {
    backends: Vec<Arc<QuantumBackend>>,
    diagnostics: Vec<DiscoveryDiagnostic>,
    provenance: DiscoveryProvenance,
}

impl DiscoverySnapshot {
    fn new(
        backends: Vec<QuantumBackend>,
        diagnostics: Vec<DiscoveryDiagnostic>,
        provenance: DiscoveryProvenance,
    ) -> Self {
        let backends = backends
            .into_iter()
            .map(Arc::new)
            .collect();

        Self {
            backends,
            diagnostics,
            provenance,
        }
    }

    /// Returns all discovered backends in deterministic order.
    pub fn backends(&self) -> &[Arc<QuantumBackend>] {
        &self.backends
    }

    /// Returns the number of discovered backends.
    pub fn len(&self) -> usize {
        self.backends.len()
    }

    /// Returns whether no backends were discovered.
    pub fn is_empty(&self) -> bool {
        self.backends.is_empty()
    }

    /// Returns discovery diagnostics.
    pub fn diagnostics(&self) -> &[DiscoveryDiagnostic] {
        &self.diagnostics
    }

    /// Returns true when at least one blocking diagnostic exists.
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics.iter().any(
            DiscoveryDiagnostic::is_blocking,
        )
    }

    /// Returns true when at least one warning/informational diagnostic exists.
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Returns immutable discovery provenance.
    pub fn provenance(&self) -> &DiscoveryProvenance {
        &self.provenance
    }

    /// Returns the deterministic discovery fingerprint.
    pub fn fingerprint(&self) -> &str {
        &self.provenance.fingerprint
    }

    /// Finds one backend by exact canonical ID.
    pub fn get(&self, backend_id: &str) -> Option<Arc<QuantumBackend>> {
        self.backends
            .binary_search_by(|backend| backend.id().cmp(backend_id))
            .ok()
            .map(|index| Arc::clone(&self.backends[index]))
    }

    /// Returns all backends belonging to one provider.
    pub fn by_provider(
        &self,
        provider: &str,
    ) -> Vec<Arc<QuantumBackend>> {
        let provider = normalize_name(provider);

        self.backends
            .iter()
            .filter(|backend| normalize_name(backend.provider()) == provider)
            .cloned()
            .collect()
    }

    /// Returns all physical QPU backends.
    pub fn qpus(&self) -> Vec<Arc<QuantumBackend>> {
        self.backends
            .iter()
            .filter(|backend| backend.kind() == BackendKind::Qpu)
            .cloned()
            .collect()
    }

    /// Returns all software backends.
    pub fn software_backends(&self) -> Vec<Arc<QuantumBackend>> {
        self.backends
            .iter()
            .filter(|backend| backend.kind().is_software())
            .cloned()
            .collect()
    }

    /// Returns all currently usable backends.
    pub fn usable(&self) -> Vec<Arc<QuantumBackend>> {
        self.backends
            .iter()
            .filter(|backend| backend.is_available())
            .cloned()
            .collect()
    }

    /// Returns all backends whose provider and backend identifiers are stable.
    pub fn ids(&self) -> Vec<String> {
        self.backends
            .iter()
            .map(|backend| backend.id().to_string())
            .collect()
    }
}

// =============================================================================
// Discovery errors
// =============================================================================

/// Errors produced by discovery orchestration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryError {
    /// No discovery sources were configured.
    NoSources,

    /// The source collection exceeded the supported limit.
    SourceCountExceeded {
        /// Number requested.
        requested: usize,

        /// Maximum supported.
        maximum: usize,
    },

    /// A source identifier is invalid.
    InvalidSourceId {
        /// Invalid source identifier.
        id: String,
    },

    /// A source name is invalid.
    InvalidSourceName {
        /// Invalid source name.
        name: String,
    },

    /// A query is invalid.
    InvalidQuery(String),

    /// A workload requirement is invalid.
    InvalidWorkload(String),

    /// A source failed.
    SourceFailure {
        /// Source identifier.
        source_id: String,

        /// Failure message.
        message: String,
    },

    /// A source returned too many backends.
    SourceLimitExceeded {
        /// Source identifier.
        source_id: String,

        /// Number returned.
        returned: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// Aggregate backend count exceeded the configured safety limit.
    AggregateLimitExceeded {
        /// Number encountered.
        encountered: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// A discovery query or source result exceeded a hard safety limit.
    LimitExceeded {
        /// Requested value.
        requested: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Two sources supplied the same backend ID with incompatible descriptors.
    BackendIdentityCollision {
        /// Backend ID.
        backend_id: String,

        /// First source.
        first_source: String,

        /// Second source.
        second_source: String,
    },

    /// A source returned a backend with an invalid identity.
    InvalidBackend {
        /// Source identifier.
        source_id: String,

        /// Backend identifier if known.
        backend_id: Option<String>,

        /// Validation message.
        message: String,
    },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSources => {
                formatter.write_str(
                    "no quantum hardware discovery sources are configured",
                )
            }

            Self::SourceCountExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "discovery source count {requested} exceeds maximum {maximum}"
            ),

            Self::InvalidSourceId { id } => {
                write!(formatter, "invalid discovery source identifier '{id}'")
            }

            Self::InvalidSourceName { name } => {
                write!(formatter, "invalid discovery source name '{name}'")
            }

            Self::InvalidQuery(message) => {
                write!(formatter, "invalid discovery query: {message}")
            }

            Self::InvalidWorkload(message) => {
                write!(
                    formatter,
                    "invalid discovery workload requirements: {message}"
                )
            }

            Self::SourceFailure {
                source_id,
                message,
            } => write!(
                formatter,
                "discovery source '{source_id}' failed: {message}"
            ),

            Self::SourceLimitExceeded {
                source_id,
                returned,
                maximum,
            } => write!(
                formatter,
                "discovery source '{source_id}' returned {returned} backends; maximum is {maximum}"
            ),

            Self::AggregateLimitExceeded {
                encountered,
                maximum,
            } => write!(
                formatter,
                "discovery encountered {encountered} backends; maximum is {maximum}"
            ),

            Self::LimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "discovery value {requested} exceeds maximum {maximum}"
            ),

            Self::BackendIdentityCollision {
                backend_id,
                first_source,
                second_source,
            } => write!(
                formatter,
                "backend identity collision for '{backend_id}' between sources '{first_source}' and '{second_source}'"
            ),

            Self::InvalidBackend {
                source_id,
                backend_id,
                message,
            } => {
                if let Some(backend_id) = backend_id {
                    write!(
                        formatter,
                        "source '{source_id}' returned invalid backend '{backend_id}': {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "source '{source_id}' returned invalid backend: {message}"
                    )
                }
            }
        }
    }
}

impl Error for DiscoveryError {}

// =============================================================================
// Discovery service
// =============================================================================

/// Provider-neutral discovery orchestrator.
///
/// The service owns discovery sources but does not own provider credentials or
/// provider network clients directly.
///
/// Sources are queried in registration order.
///
/// Results are normalized and then sorted by canonical backend ID before
/// filtering and snapshot creation.
///
/// This gives reproducible output even when providers return devices in
/// different orders.
pub struct DiscoveryService {
    sources: Vec<Arc<dyn DiscoverySource>>,
    policy: DiscoveryPolicy,
    maximum_backends: usize,
}

impl fmt::Debug for DiscoveryService {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiscoveryService")
            .field("source_count", &self.sources.len())
            .field("policy", &self.policy)
            .field("maximum_backends", &self.maximum_backends)
            .finish()
    }
}

impl Default for DiscoveryService {
    fn default() -> Self {
        Self {
            sources: Vec::new(),
            policy: DiscoveryPolicy::BestEffort,
            maximum_backends: MAX_DISCOVERED_BACKENDS,
        }
    }
}

impl DiscoveryService {
    /// Creates an empty discovery service.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the source failure policy.
    pub fn with_policy(mut self, policy: DiscoveryPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Adds a discovery source.
    pub fn with_source<S>(
        mut self,
        source: S,
    ) -> Result<Self, DiscoveryError>
    where
        S: DiscoverySource + 'static,
    {
        self.add_source(source)?;
        Ok(self)
    }

    /// Adds an already shared discovery source.
    pub fn with_shared_source(
        mut self,
        source: Arc<dyn DiscoverySource>,
    ) -> Result<Self, DiscoveryError> {
        self.add_shared_source(source)?;
        Ok(self)
    }

    /// Registers a discovery source.
    pub fn add_source<S>(
        &mut self,
        source: S,
    ) -> Result<(), DiscoveryError>
    where
        S: DiscoverySource + 'static,
    {
        self.add_shared_source(Arc::new(source))
    }

    /// Registers an already shared discovery source.
    pub fn add_shared_source(
        &mut self,
        source: Arc<dyn DiscoverySource>,
    ) -> Result<(), DiscoveryError> {
        validate_source(source.as_ref())?;

        if self.sources.len() >= MAX_DISCOVERY_SOURCES {
            return Err(DiscoveryError::SourceCountExceeded {
                requested: self.sources.len() + 1,
                maximum: MAX_DISCOVERY_SOURCES,
            });
        }

        let source_id = source.id().to_string();

        if self.sources.iter().any(|existing| {
            existing.id() == source_id
        }) {
            return Err(DiscoveryError::InvalidSourceId {
                id: source_id,
            });
        }

        self.sources.push(source);
        Ok(())
    }

    /// Removes a source by exact identifier.
    ///
    /// Returns `true` if a source was removed.
    pub fn remove_source(&mut self, source_id: &str) -> bool {
        let before = self.sources.len();

        self.sources.retain(|source| source.id() != source_id);

        before != self.sources.len()
    }

    /// Returns the configured source count.
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Returns source identifiers in deterministic registration order.
    pub fn source_ids(&self) -> Vec<String> {
        self.sources
            .iter()
            .map(|source| source.id().to_string())
            .collect()
    }

    /// Returns the configured failure policy.
    pub const fn policy(&self) -> DiscoveryPolicy {
        self.policy
    }

    /// Returns the aggregate backend safety limit.
    pub const fn maximum_backends(&self) -> usize {
        self.maximum_backends
    }

    /// Sets the aggregate backend safety limit.
    pub fn set_maximum_backends(
        &mut self,
        maximum: usize,
    ) -> Result<(), DiscoveryError> {
        if maximum == 0 {
            return Err(DiscoveryError::LimitExceeded {
                requested: maximum,
                maximum: MAX_DISCOVERED_BACKENDS,
            });
        }

        if maximum > MAX_DISCOVERED_BACKENDS {
            return Err(DiscoveryError::LimitExceeded {
                requested: maximum,
                maximum: MAX_DISCOVERED_BACKENDS,
            });
        }

        self.maximum_backends = maximum;
        Ok(())
    }

    /// Performs discovery with the default query.
    pub fn discover(
        &self,
        query: &DiscoveryQuery,
    ) -> Result<DiscoverySnapshot, DiscoveryError> {
        query.validate()?;

        if self.sources.is_empty() {
            return Err(DiscoveryError::NoSources);
        }

        let mut raw_backends: BTreeMap<String, QuantumBackend> =
            BTreeMap::new();

        let mut backend_sources: BTreeMap<String, String> = BTreeMap::new();

        let mut diagnostics = Vec::new();

        let mut successful_sources = 0usize;
        let mut failed_sources = 0usize;
        let mut raw_backend_count = 0usize;

        for source in &self.sources {
            if !source.is_configured() {
                diagnostics.push(DiscoveryDiagnostic::new(
                    DiscoveryDiagnosticCode::SourceNotConfigured,
                    Some(source.id().to_string()),
                    None,
                    format!(
                        "discovery source '{}' is not configured",
                        source.id()
                    ),
                    self.policy == DiscoveryPolicy::Strict,
                ));

                if self.policy == DiscoveryPolicy::Strict {
                    return Err(DiscoveryError::SourceFailure {
                        source_id: source.id().to_string(),
                        message: "source is not configured".to_string(),
                    });
                }

                failed_sources = checked_increment(failed_sources)?;
                continue;
            }

            let discovered = match source.discover() {
                Ok(backends) => {
                    successful_sources =
                        checked_increment(successful_sources)?;
                    backends
                }

                Err(error) => {
                    failed_sources =
                        checked_increment(failed_sources)?;

                    diagnostics.push(DiscoveryDiagnostic::new(
                        DiscoveryDiagnosticCode::SourceFailure,
                        Some(source.id().to_string()),
                        None,
                        error.to_string(),
                        self.policy == DiscoveryPolicy::Strict,
                    ));

                    if self.policy == DiscoveryPolicy::Strict {
                        return Err(DiscoveryError::SourceFailure {
                            source_id: source.id().to_string(),
                            message: error.to_string(),
                        });
                    }

                    continue;
                }
            };

            if discovered.len() > MAX_BACKENDS_PER_SOURCE {
                diagnostics.push(DiscoveryDiagnostic::new(
                    DiscoveryDiagnosticCode::SourceLimitExceeded,
                    Some(source.id().to_string()),
                    None,
                    format!(
                        "source returned {} backends; maximum is {}",
                        discovered.len(),
                        MAX_BACKENDS_PER_SOURCE
                    ),
                    true,
                ));

                if self.policy == DiscoveryPolicy::Strict {
                    return Err(DiscoveryError::SourceLimitExceeded {
                        source_id: source.id().to_string(),
                        returned: discovered.len(),
                        maximum: MAX_BACKENDS_PER_SOURCE,
                    });
                }

                continue;
            }

            raw_backend_count = raw_backend_count
                .checked_add(discovered.len())
                .ok_or(DiscoveryError::AggregateLimitExceeded {
                    encountered: usize::MAX,
                    maximum: self.maximum_backends,
                })?;

            if raw_backend_count > self.maximum_backends {
                diagnostics.push(DiscoveryDiagnostic::new(
                    DiscoveryDiagnosticCode::AggregateLimitExceeded,
                    Some(source.id().to_string()),
                    None,
                    format!(
                        "aggregate discovery count exceeded maximum {}",
                        self.maximum_backends
                    ),
                    true,
                ));

                return Err(
                    DiscoveryError::AggregateLimitExceeded {
                        encountered: raw_backend_count,
                        maximum: self.maximum_backends,
                    },
                );
            }

            for backend in discovered {
                validate_discovered_backend(
                    source.id(),
                    &backend,
                )?;

                let backend_id = backend.id().to_string();

                match raw_backends.get(&backend_id) {
                    None => {
                        backend_sources.insert(
                            backend_id.clone(),
                            source.id().to_string(),
                        );

                        raw_backends.insert(
                            backend_id,
                            backend,
                        );
                    }

                    Some(existing) if existing == &backend => {
                        diagnostics.push(
                            DiscoveryDiagnostic::new(
                                DiscoveryDiagnosticCode::DuplicateBackend,
                                Some(source.id().to_string()),
                                Some(backend_id),
                                "identical backend descriptor was returned by multiple discovery sources",
                                false,
                            ),
                        );
                    }

                    Some(_) => {
                        let first_source = backend_sources
                            .get(&backend_id)
                            .cloned()
                            .unwrap_or_else(|| {
                                "unknown".to_string()
                            });

                        let diagnostic =
                            DiscoveryDiagnostic::new(
                                DiscoveryDiagnosticCode::BackendIdentityCollision,
                                Some(source.id().to_string()),
                                Some(backend_id.clone()),
                                format!(
                                    "backend ID '{}' was returned by '{}' and '{}' with different descriptors",
                                    backend_id,
                                    first_source,
                                    source.id()
                                ),
                                true,
                            );

                        diagnostics.push(diagnostic);

                        return Err(
                            DiscoveryError::BackendIdentityCollision {
                                backend_id,
                                first_source,
                                second_source:
                                    source.id().to_string(),
                            },
                        );
                    }
                }
            }
        }

        let unique_backend_count = raw_backends.len();

        let mut filtered_backend_count = 0usize;
        let mut selected = Vec::new();

        for (_backend_id, backend) in raw_backends {
            if backend_matches_query(&backend, query)? {
                selected.push(backend);
            } else {
                filtered_backend_count =
                    checked_increment(filtered_backend_count)?;

                diagnostics.push(
                    DiscoveryDiagnostic::new(
                        DiscoveryDiagnosticCode::BackendFiltered,
                        None,
                        Some(backend.id().to_string()),
                        "backend did not satisfy the discovery query",
                        false,
                    ),
                );
            }
        }

        selected.sort_by(|left, right| {
            canonical_backend_sort_key(left)
                .cmp(&canonical_backend_sort_key(right))
        });

        if let Some(limit) = query.limit {
            if selected.len() > limit {
                let discarded = selected.len() - limit;

                selected.truncate(limit);

                filtered_backend_count =
                    filtered_backend_count
                        .checked_add(discarded)
                        .ok_or(
                            DiscoveryError::AggregateLimitExceeded {
                                encountered: usize::MAX,
                                maximum: self.maximum_backends,
                            },
                        )?;
            }
        }

        let fingerprint = fingerprint_backends(&selected);

        let source_ids = self
            .sources
            .iter()
            .map(|source| source.id().to_string())
            .collect::<Vec<_>>();

        let provenance = DiscoveryProvenance::new(
            self.policy,
            source_ids,
            successful_sources,
            failed_sources,
            raw_backend_count,
            unique_backend_count,
            filtered_backend_count,
            fingerprint,
        );

        sort_diagnostics(&mut diagnostics);

        Ok(DiscoverySnapshot::new(
            selected,
            diagnostics,
            provenance,
        ))
    }

    /// Performs strict discovery.
    pub fn discover_strict(
        &self,
        query: &DiscoveryQuery,
    ) -> Result<DiscoverySnapshot, DiscoveryError> {
        let strict = Self {
            sources: self.sources.clone(),
            policy: DiscoveryPolicy::Strict,
            maximum_backends: self.maximum_backends,
        };

        strict.discover(query)
    }

    /// Performs best-effort discovery.
    pub fn discover_best_effort(
        &self,
        query: &DiscoveryQuery,
    ) -> Result<DiscoverySnapshot, DiscoveryError> {
        let best_effort = Self {
            sources: self.sources.clone(),
            policy: DiscoveryPolicy::BestEffort,
            maximum_backends: self.maximum_backends,
        };

        best_effort.discover(query)
    }
}

// =============================================================================
// Query matching
// =============================================================================

/// Determines whether a canonical backend satisfies a discovery query.
fn backend_matches_query(
    backend: &QuantumBackend,
    query: &DiscoveryQuery,
) -> Result<bool, DiscoveryError> {
    if let Some(provider) = &query.provider {
        if normalize_name(backend.provider()) != normalize_name(provider) {
            return Ok(false);
        }
    }

    if let Some(backend_id) = &query.backend_id {
        if backend.id() != backend_id {
            return Ok(false);
        }
    }

    if let Some(region) = &query.region {
        match backend.metadata.region.as_deref() {
            Some(actual)
                if normalize_name(actual) == normalize_name(region) => {}
            _ => return Ok(false),
        }
    }

    if let Some(kind) = query.kind {
        if backend.kind() != kind {
            return Ok(false);
        }
    }

    if let Some(status) = query.status {
        if backend.status() != status {
            return Ok(false);
        }
    }

    if query.require_operational
        && !backend.status().is_operational()
    {
        return Ok(false);
    }

    if query.require_usable && !backend.is_available() {
        return Ok(false);
    }

    let qubits = backend.qubit_count();

    if let Some(minimum) = query.minimum_qubits {
        if qubits < minimum {
            return Ok(false);
        }
    }

    if let Some(maximum) = query.maximum_qubits {
        if qubits > maximum {
            return Ok(false);
        }
    }

    if !capabilities_match(
        &backend.capabilities,
        &query.required_capabilities,
        query.include_experimental,
    ) {
        return Ok(false);
    }

    if !instructions_match(
        &backend.capabilities,
        &query.required_instructions,
    ) {
        return Ok(false);
    }

    if let Some(workload) = &query.workload {
        if backend.validate(workload).is_err() {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Matches stable capability requirements.
///
/// Experimental capability identifiers never satisfy stable requirements.
fn capabilities_match(
    capabilities: &BackendCapabilities,
    required: &BTreeSet<String>,
    include_experimental: bool,
) -> bool {
    required.iter().all(|required_capability| {
        if capabilities.supports(required_capability) {
            return true;
        }

        include_experimental
            && capabilities.supports_experimental(required_capability)
            && !capabilities.stable_names().iter().any(|stable| {
                stable == required_capability
            })
    })
}

/// Matches native instruction requirements.
fn instructions_match(
    capabilities: &BackendCapabilities,
    required: &BTreeSet<String>,
) -> bool {
    required.iter().all(|instruction| {
        capabilities.native_gates.contains(&normalize_name(instruction))
    })
}

// =============================================================================
// Source/backend validation
// =============================================================================

/// Validates a discovery source before registration.
fn validate_source(
    source: &dyn DiscoverySource,
) -> Result<(), DiscoveryError> {
    let id = source.id();

    validate_safe_identifier(
        id,
        MAX_SOURCE_ID_LENGTH,
    )
    .map_err(|_| DiscoveryError::InvalidSourceId {
        id: id.to_string(),
    })?;

    let name = source.name();

    validate_safe_identifier(
        name,
        MAX_SOURCE_NAME_LENGTH,
    )
    .map_err(|_| DiscoveryError::InvalidSourceName {
        name: name.to_string(),
    })?;

    Ok(())
}

/// Validates a backend returned by a discovery source.
///
/// The backend constructor is expected to have performed structural validation,
/// but discovery performs a second boundary check because discovery is a trust
/// boundary between external providers and the rest of Zamani.
fn validate_discovered_backend(
    source_id: &str,
    backend: &QuantumBackend,
) -> Result<(), DiscoveryError> {
    if validate_safe_identifier(
        backend.id(),
        512,
    )
    .is_err()
    {
        return Err(DiscoveryError::InvalidBackend {
            source_id: source_id.to_string(),
            backend_id: Some(backend.id().to_string()),
            message: "backend identifier is invalid or appears secret-bearing"
                .to_string(),
        });
    }

    if validate_safe_identifier(
        backend.provider(),
        512,
    )
    .is_err()
    {
        return Err(DiscoveryError::InvalidBackend {
            source_id: source_id.to_string(),
            backend_id: Some(backend.id().to_string()),
            message:
                "provider identifier is invalid or appears secret-bearing"
                    .to_string(),
        });
    }

    if backend.metadata.name.trim().is_empty() {
        return Err(DiscoveryError::InvalidBackend {
            source_id: source_id.to_string(),
            backend_id: Some(backend.id().to_string()),
            message: "backend name is empty".to_string(),
        });
    }

    if let Some(region) = &backend.metadata.region {
        if validate_safe_identifier(
            region,
            256,
        )
        .is_err()
        {
            return Err(DiscoveryError::InvalidBackend {
                source_id: source_id.to_string(),
                backend_id: Some(backend.id().to_string()),
                message: "backend region is invalid".to_string(),
            });
        }
    }

    for (key, value) in &backend.metadata.properties {
        if key.trim().is_empty() || value.trim().is_empty() {
            return Err(DiscoveryError::InvalidBackend {
                source_id: source_id.to_string(),
                backend_id: Some(backend.id().to_string()),
                message: "backend metadata contains an empty property"
                    .to_string(),
            });
        }

        if looks_secret_like(key) || looks_secret_like(value) {
            return Err(DiscoveryError::InvalidBackend {
                source_id: source_id.to_string(),
                backend_id: Some(backend.id().to_string()),
                message:
                    "backend metadata appears to contain secret material"
                        .to_string(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Canonical ordering
// =============================================================================

fn canonical_backend_sort_key(
    backend: &QuantumBackend,
) -> (
    &str,
    &str,
    &str,
    &str,
    BackendKind,
) {
    (
        backend.provider(),
        backend.metadata.region.as_deref().unwrap_or(""),
        backend.id(),
        &backend.metadata.version,
        backend.kind(),
    )
}

fn sort_diagnostics(
    diagnostics: &mut Vec<DiscoveryDiagnostic>,
) {
    diagnostics.sort_by(|left, right| {
        (
            left.code,
            left.source_id.as_deref().unwrap_or(""),
            left.backend_id.as_deref().unwrap_or(""),
            left.message.as_str(),
            left.blocking,
        )
            .cmp(&(
                right.code,
                right.source_id.as_deref().unwrap_or(""),
                right.backend_id.as_deref().unwrap_or(""),
                right.message.as_str(),
                right.blocking,
            ))
    });
}

// =============================================================================
// Fingerprinting
// =============================================================================

/// Computes a deterministic non-cryptographic fingerprint for a backend set.
///
/// This fingerprint is suitable for:
///
/// - provenance;
/// - cache keys;
/// - change detection;
/// - benchmark metadata;
/// - discovery snapshot equality hints.
///
/// It is NOT suitable as a cryptographic integrity mechanism.
///
/// A cryptographic hash belongs in a future serialization/provenance layer.
fn fingerprint_backends(
    backends: &[QuantumBackend],
) -> String {
    let mut hash = FNV_OFFSET_BASIS;

    for backend in backends {
        hash_bytes(&mut hash, backend.id().as_bytes());
        hash_bytes(&mut hash, &[0]);

        hash_bytes(
            &mut hash,
            backend.provider().as_bytes(),
        );
        hash_bytes(&mut hash, &[0]);

        hash_bytes(
            &mut hash,
            backend.metadata.name.as_bytes(),
        );
        hash_bytes(&mut hash, &[0]);

        hash_bytes(
            &mut hash,
            backend.metadata.version.as_bytes(),
        );
        hash_bytes(&mut hash, &[0]);

        hash_bytes(
            &mut hash,
            backend.kind().as_str().as_bytes(),
        );
        hash_bytes(&mut hash, &[0]);

        hash_bytes(
            &mut hash,
            backend.status().as_str().as_bytes(),
        );
        hash_bytes(&mut hash, &[0]);

        hash_usize(&mut hash, backend.qubit_count());

        for capability in backend.capability_names() {
            hash_bytes(&mut hash, capability.as_bytes());
            hash_bytes(&mut hash, &[0]);
        }

        for gate in backend.native_gates() {
            hash_bytes(&mut hash, gate.as_bytes());
            hash_bytes(&mut hash, &[0]);
        }

        hash_bytes(&mut hash, &[0xff]);
    }

    format!("{hash:016x}")
}

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x00000100000001b3;

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn hash_usize(hash: &mut u64, value: usize) {
    hash_bytes(hash, &value.to_le_bytes());
}

// =============================================================================
// Safe normalization
// =============================================================================

fn normalize_filter(
    field: &'static str,
    value: String,
    maximum: usize,
) -> Result<String, DiscoveryError> {
    let normalized = value.trim().to_string();

    if normalized.is_empty() {
        return Err(DiscoveryError::InvalidQuery(format!(
            "{field} cannot be empty"
        )));
    }

    if normalized.len() > maximum {
        return Err(DiscoveryError::LimitExceeded {
            requested: normalized.len(),
            maximum,
        });
    }

    if normalized.chars().any(char::is_control) {
        return Err(DiscoveryError::InvalidQuery(format!(
            "{field} contains control characters"
        )));
    }

    if looks_secret_like(&normalized) {
        return Err(DiscoveryError::InvalidQuery(format!(
            "{field} appears to contain secret material"
        )));
    }

    Ok(normalized)
}

fn validate_safe_identifier(
    value: &str,
    maximum: usize,
) -> Result<(), ()> {
    let value = value.trim();

    if value.is_empty()
        || value.len() > maximum
        || value.chars().any(char::is_control)
        || looks_secret_like(value)
    {
        return Err(());
    }

    Ok(())
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn looks_secret_like(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    let markers = [
        "api_key",
        "apikey",
        "access_token",
        "accesstoken",
        "authorization",
        "bearer ",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "secret",
        "session_cookie",
        "sessioncookie",
    ];

    markers.iter().any(|marker| lower.contains(marker))
}

fn truncate_message(mut message: String) -> String {
    if message.len() <= MAX_DIAGNOSTIC_MESSAGE_LENGTH {
        return message;
    }

    message.truncate(MAX_DIAGNOSTIC_MESSAGE_LENGTH);
    message
}

// =============================================================================
// Arithmetic safety
// =============================================================================

fn checked_increment(value: usize) -> Result<usize, DiscoveryError> {
    value
        .checked_add(1)
        .ok_or(DiscoveryError::AggregateLimitExceeded {
            encountered: usize::MAX,
            maximum: MAX_DISCOVERED_BACKENDS,
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    struct StaticSource {
        id: &'static str,
        name: &'static str,
        backends: Vec<QuantumBackend>,
        configured: bool,
    }

    impl DiscoverySource for StaticSource {
        fn id(&self) -> &str {
            self.id
        }

        fn name(&self) -> &str {
            self.name
        }

        fn discover(
            &self,
        ) -> Result<Vec<QuantumBackend>, DiscoveryError> {
            Ok(self.backends.clone())
        }

        fn is_configured(&self) -> bool {
            self.configured
        }
    }

    struct FailingSource;

    impl DiscoverySource for FailingSource {
        fn id(&self) -> &str {
            "failing"
        }

        fn name(&self) -> &str {
            "Failing source"
        }

        fn discover(
            &self,
        ) -> Result<Vec<QuantumBackend>, DiscoveryError> {
            Err(DiscoveryError::SourceFailure {
                source_id: "failing".to_string(),
                message: "synthetic failure".to_string(),
            })
        }
    }

    fn backend(
        id: &str,
        provider: &str,
        kind: BackendKind,
        status: BackendStatus,
        qubits: usize,
    ) -> QuantumBackend {
        use super::super::backend::{
            BackendCapabilities,
            BackendLimits,
            BackendMetadata,
        };
        use super::super::topology::HardwareTopology;

        let metadata = BackendMetadata::new(
            id,
            id,
            provider,
            "1.0",
            kind,
        );

        let capabilities =
            BackendCapabilities::default();

        let limits =
            BackendLimits::unlimited();

        let topology =
            HardwareTopology::new(qubits).expect(
                "test topology must be valid",
            );

        let mut backend = QuantumBackend::new(
            metadata,
            capabilities,
            limits,
            topology,
        )
        .expect("test backend must be valid");

        backend.set_status(status);

        backend
    }

    #[test]
    fn empty_service_has_no_sources() {
        let service = DiscoveryService::new();

        let error = service
            .discover(&DiscoveryQuery::default())
            .expect_err("empty discovery service must fail");

        assert_eq!(error, DiscoveryError::NoSources);
    }

    #[test]
    fn source_registration_is_deterministic() {
        let first = StaticSource {
            id: "first",
            name: "First",
            backends: Vec::new(),
            configured: true,
        };

        let second = StaticSource {
            id: "second",
            name: "Second",
            backends: Vec::new(),
            configured: true,
        };

        let service = DiscoveryService::new()
            .with_source(first)
            .expect("first source")
            .with_source(second)
            .expect("second source");

        assert_eq!(
            service.source_ids(),
            vec!["first", "second"]
        );
    }

    #[test]
    fn duplicate_source_ids_are_rejected() {
        let source_a = StaticSource {
            id: "same",
            name: "A",
            backends: Vec::new(),
            configured: true,
        };

        let source_b = StaticSource {
            id: "same",
            name: "B",
            backends: Vec::new(),
            configured: true,
        };

        let mut service =
            DiscoveryService::new();

        service
            .add_source(source_a)
            .expect("first source must work");

        assert!(matches!(
            service.add_source(source_b),
            Err(DiscoveryError::InvalidSourceId { .. })
        ));
    }

    #[test]
    fn best_effort_isolates_source_failure() {
        let backend = backend(
            "local.simulator",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            4,
        );

        let service = DiscoveryService::new()
            .with_policy(DiscoveryPolicy::BestEffort)
            .with_source(StaticSource {
                id: "local",
                name: "Local",
                backends: vec![backend],
                configured: true,
            })
            .expect("local source")
            .with_source(FailingSource)
            .expect("failing source");

        let snapshot = service
            .discover(&DiscoveryQuery::default())
            .expect("best-effort discovery should succeed");

        assert_eq!(snapshot.len(), 1);
        assert!(snapshot.has_diagnostics());
        assert_eq!(
            snapshot.provenance().failed_sources,
            1
        );
    }

    #[test]
    fn strict_propagates_source_failure() {
        let service = DiscoveryService::new()
            .with_policy(DiscoveryPolicy::Strict)
            .with_source(FailingSource)
            .expect("source registration");

        let error = service
            .discover(&DiscoveryQuery::default())
            .expect_err("strict discovery must fail");

        assert!(matches!(
            error,
            DiscoveryError::SourceFailure { .. }
        ));
    }

    #[test]
    fn results_are_sorted_by_canonical_identity() {
        let first = backend(
            "z",
            "provider",
            BackendKind::Qpu,
            BackendStatus::Available,
            2,
        );

        let second = backend(
            "a",
            "provider",
            BackendKind::Qpu,
            BackendStatus::Available,
            2,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![first, second],
                configured: true,
            })
            .expect("source registration");

        let snapshot = service
            .discover(&DiscoveryQuery::default())
            .expect("discovery");

        assert_eq!(
            snapshot.ids(),
            vec!["a".to_string(), "z".to_string()]
        );
    }

    #[test]
    fn provider_filter_works() {
        let a = backend(
            "a",
            "ibm",
            BackendKind::Qpu,
            BackendStatus::Available,
            5,
        );

        let b = backend(
            "b",
            "ionq",
            BackendKind::Qpu,
            BackendStatus::Available,
            25,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![a, b],
                configured: true,
            })
            .expect("source");

        let query = DiscoveryQuery::new()
            .with_provider("IBM")
            .expect("provider filter");

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.backends()[0].provider(),
            "ibm"
        );
    }

    #[test]
    fn qubit_filter_works() {
        let small = backend(
            "small",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            4,
        );

        let large = backend(
            "large",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            64,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![small, large],
                configured: true,
            })
            .expect("source");

        let query =
            DiscoveryQuery::new().with_minimum_qubits(32);

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.backends()[0].id(),
            "large"
        );
    }

    #[test]
    fn status_filter_works() {
        let available = backend(
            "available",
            "local",
            BackendKind::Qpu,
            BackendStatus::Available,
            4,
        );

        let offline = backend(
            "offline",
            "local",
            BackendKind::Qpu,
            BackendStatus::Offline,
            4,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![available, offline],
                configured: true,
            })
            .expect("source");

        let query = DiscoveryQuery::new()
            .with_status(BackendStatus::Available);

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.backends()[0].id(),
            "available"
        );
    }

    #[test]
    fn usable_filter_excludes_offline_backend() {
        let available = backend(
            "available",
            "local",
            BackendKind::Qpu,
            BackendStatus::Available,
            4,
        );

        let offline = backend(
            "offline",
            "local",
            BackendKind::Qpu,
            BackendStatus::Offline,
            4,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![available, offline],
                configured: true,
            })
            .expect("source");

        let query =
            DiscoveryQuery::new().usable();

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.backends()[0].id(),
            "available"
        );
    }

    #[test]
    fn exact_duplicate_backends_are_deduplicated() {
        let backend = backend(
            "same",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            4,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "first",
                name: "First",
                backends: vec![backend.clone()],
                configured: true,
            })
            .expect("first")
            .with_source(StaticSource {
                id: "second",
                name: "Second",
                backends: vec![backend],
                configured: true,
            })
            .expect("second");

        let snapshot = service
            .discover(&DiscoveryQuery::default())
            .expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert!(snapshot.diagnostics().iter().any(
            |diagnostic| {
                diagnostic.code
                    == DiscoveryDiagnosticCode::DuplicateBackend
            }
        ));
    }

    #[test]
    fn conflicting_backend_identity_is_rejected() {
        let first = backend(
            "same",
            "provider-a",
            BackendKind::Qpu,
            BackendStatus::Available,
            4,
        );

        let second = backend(
            "same",
            "provider-b",
            BackendKind::Qpu,
            BackendStatus::Available,
            8,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "first",
                name: "First",
                backends: vec![first],
                configured: true,
            })
            .expect("first")
            .with_source(StaticSource {
                id: "second",
                name: "Second",
                backends: vec![second],
                configured: true,
            })
            .expect("second");

        let error = service
            .discover(&DiscoveryQuery::default())
            .expect_err("identity collision must fail");

        assert!(matches!(
            error,
            DiscoveryError::BackendIdentityCollision { .. }
        ));
    }

    #[test]
    fn result_limit_is_enforced() {
        let first = backend(
            "a",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            1,
        );

        let second = backend(
            "b",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            2,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![first, second],
                configured: true,
            })
            .expect("source");

        let query = DiscoveryQuery::new()
            .with_limit(1)
            .expect("limit");

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.backends()[0].id(),
            "a"
        );
    }

    #[test]
    fn backend_lookup_uses_sorted_ids() {
        let a = backend(
            "a",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            1,
        );

        let b = backend(
            "b",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            1,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![b, a],
                configured: true,
            })
            .expect("source");

        let snapshot =
            service.discover(&DiscoveryQuery::default())
                .expect("discovery");

        assert!(snapshot.get("a").is_some());
        assert!(snapshot.get("b").is_some());
        assert!(snapshot.get("missing").is_none());
    }

    #[test]
    fn provider_grouping_works() {
        let a = backend(
            "a",
            "provider-a",
            BackendKind::Qpu,
            BackendStatus::Available,
            1,
        );

        let b = backend(
            "b",
            "provider-b",
            BackendKind::Qpu,
            BackendStatus::Available,
            1,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![a, b],
                configured: true,
            })
            .expect("source");

        let snapshot =
            service.discover(&DiscoveryQuery::default())
                .expect("discovery");

        assert_eq!(
            snapshot.by_provider("PROVIDER-A").len(),
            1
        );
    }

    #[test]
    fn qpu_and_software_views_work() {
        let qpu = backend(
            "qpu",
            "provider",
            BackendKind::Qpu,
            BackendStatus::Available,
            4,
        );

        let simulator = backend(
            "simulator",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            4,
        );

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![qpu, simulator],
                configured: true,
            })
            .expect("source");

        let snapshot =
            service.discover(&DiscoveryQuery::default())
                .expect("discovery");

        assert_eq!(snapshot.qpus().len(), 1);
        assert_eq!(
            snapshot.software_backends().len(),
            1
        );
    }

    #[test]
    fn fingerprints_are_deterministic() {
        let a = backend(
            "a",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            4,
        );

        let b = backend(
            "b",
            "local",
            BackendKind::Simulator,
            BackendStatus::Available,
            8,
        );

        let service_a = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![a.clone(), b.clone()],
                configured: true,
            })
            .expect("source");

        let service_b = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![b, a],
                configured: true,
            })
            .expect("source");

        let snapshot_a = service_a
            .discover(&DiscoveryQuery::default())
            .expect("discovery");

        let snapshot_b = service_b
            .discover(&DiscoveryQuery::default())
            .expect("discovery");

        assert_eq!(
            snapshot_a.fingerprint(),
            snapshot_b.fingerprint()
        );
    }

    #[test]
    fn invalid_query_is_rejected() {
        let query = DiscoveryQuery {
            minimum_qubits: Some(10),
            maximum_qubits: Some(5),
            ..DiscoveryQuery::default()
        };

        assert!(query.validate().is_err());
    }

    #[test]
    fn empty_filter_is_rejected() {
        let result =
            DiscoveryQuery::new().with_provider("");

        assert!(result.is_err());
    }

    #[test]
    fn secret_like_filter_is_rejected() {
        let result =
            DiscoveryQuery::new().with_provider("api_key=secret");

        assert!(result.is_err());
    }

    #[test]
    fn unconfigured_source_is_best_effort_diagnostic() {
        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "unconfigured",
                name: "Unconfigured",
                backends: Vec::new(),
                configured: false,
            })
            .expect("source");

        let snapshot =
            service.discover(&DiscoveryQuery::default())
                .expect("best effort");

        assert!(snapshot.is_empty());
        assert_eq!(
            snapshot.provenance().failed_sources,
            1
        );
        assert!(snapshot.diagnostics().iter().any(
            |diagnostic| {
                diagnostic.code
                    == DiscoveryDiagnosticCode::SourceNotConfigured
            }
        ));
    }

    #[test]
    fn strict_unconfigured_source_fails() {
        let service = DiscoveryService::new()
            .with_policy(DiscoveryPolicy::Strict)
            .with_source(StaticSource {
                id: "unconfigured",
                name: "Unconfigured",
                backends: Vec::new(),
                configured: false,
            })
            .expect("source");

        let error =
            service
                .discover(&DiscoveryQuery::default())
                .expect_err("strict mode must fail");

        assert!(matches!(
            error,
            DiscoveryError::SourceFailure { .. }
        ));
    }

    #[test]
    fn required_instruction_filter_is_deterministic() {
        let mut capabilities =
            BackendCapabilities::default();

        capabilities.native_gates =
            BTreeSet::from([
                "cx".to_string(),
                "rz".to_string(),
            ]);

        let metadata =
            super::super::backend::BackendMetadata::new(
                "native",
                "native",
                "local",
                "1.0",
                BackendKind::Simulator,
            );

        let topology =
            super::super::topology::HardwareTopology::new(2)
                .expect("topology");

        let backend =
            QuantumBackend::new(
                metadata,
                capabilities,
                super::super::backend::BackendLimits::unlimited(),
                topology,
            )
            .expect("backend");

        let service = DiscoveryService::new()
            .with_source(StaticSource {
                id: "source",
                name: "Source",
                backends: vec![backend],
                configured: true,
            })
            .expect("source");

        let query = DiscoveryQuery::new()
            .with_required_instruction("CX")
            .expect("instruction");

        let snapshot =
            service.discover(&query).expect("discovery");

        assert_eq!(snapshot.len(), 1);
    }
}