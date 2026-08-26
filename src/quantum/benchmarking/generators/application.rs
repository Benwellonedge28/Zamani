//! Zamani Quantum Benchmarking — Application Workload Generator
//!
//! Defines the production application-benchmark generation contract.
//!
//! # Architectural role
//!
//! This module is responsible for describing and constructing application-level
//! benchmark workloads. It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - select a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform statistical analysis;
//! - calculate benchmark metrics;
//! - define application algorithms themselves;
//! - duplicate Quantum IR;
//! - parse Zamani source code;
//! - communicate with hardware;
//! - perform filesystem or network I/O.
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani application benchmark declaration
//!                 │
//!                 ▼
//!     ApplicationGenerationRequest
//!                 │
//!                 ▼
//!     ApplicationBenchmarkGenerator
//!                 │
//!                 ▼
//!       ApplicationWorkload
//!                 │
//!        ┌────────┴─────────┐
//!        ▼                  ▼
//! Quantum IR            application metadata
//!        │                  │
//!        └────────┬─────────┘
//!                 ▼
//!          core::experiment
//!                 │
//!                 ▼
//!          execution layer
//!                 │
//!                 ▼
//!          observations
//!                 │
//!                 ▼
//!          application analysis
//! ```
//!
//! # Important boundary
//!
//! `core::workload::ApplicationWorkload` is the canonical application workload
//! representation. This module must not introduce another competing workload
//! structure.
//!
//! Application-specific implementations such as:
//!
//! - Deutsch-Jozsa;
//! - Bernstein-Vazirani;
//! - Hidden Shift;
//! - QFT;
//! - Grover;
//! - Phase Estimation;
//! - Amplitude Estimation;
//! - VQE;
//! - QAOA;
//! - MaxCut;
//! - Hamiltonian simulation;
//! - HHL;
//! - Monte Carlo;
//! - Shor;
//! - user-defined Zamani applications;
//!
//! should implement [`ApplicationBenchmarkGenerator`] and return an
//! `ApplicationGeneration` containing the canonical
//! `core::workload::ApplicationWorkload`.
//!
//! # Reproducibility
//!
//! Application generation is deterministic with respect to the generation
//! inputs selected by the caller:
//!
//! - application identifier;
//! - instance identifier;
//! - problem size;
//! - application parameters;
//! - seed;
//! - sequence index;
//! - generator revision.
//!
//! This module does not implement the random-number algorithm. The common
//! random-number implementation belongs to `generators::random`.
//!
//! A generator must never use a hidden global RNG, system time, process ID,
//! pointer address, thread ID, or other implicit entropy source to change the
//! generated workload.
//!
//! # Security/resource model
//!
//! Application benchmark requests can originate from the Zamani language,
//! configuration files, CI, external benchmark definitions, or machine APIs.
//! They are therefore treated as untrusted input.
//!
//! This module:
//!
//! - validates identifiers;
//! - bounds parameter counts;
//! - bounds parameter name/value sizes through the canonical workload model;
//! - rejects zero problem sizes;
//! - rejects empty generator/application identifiers;
//! - rejects inconsistent generation requests;
//! - avoids unbounded recursive generation;
//! - does not allocate based on unchecked multiplication;
//! - does not execute user-provided code;
//! - does not perform I/O.
//!
//! Global resource limits remain the responsibility of
//! `core::limits` / `core::config`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//! No external dependencies are required.
//!
//! # Integration contract
//!
//! This file intentionally integrates with the following existing modules:
//!
//! ```text
//! generators/application.rs
//!          │
//!          ├── core::errors
//!          ├── core::workload
//!          └── future generators::random
//!
//!          │
//!          ▼
//! core::experiment
//!          │
//!          ▼
//! core::execution
//!          │
//!          ▼
//! core::observation / result
//! ```
//!
//! It does not require changes to those modules merely to define this
//! generator contract.
//!
//! Individual application generators can later attach a
//! `CircuitWorkload` using `ApplicationWorkload::with_circuit()`.
//!
//! The generated application workload remains independent of backend-specific
//! compilation, routing, scheduling, and execution.

use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    WorkloadError,
    WorkloadId,
};

// =============================================================================
// Stable schema/version constants
// =============================================================================

/// Schema version for application-generator descriptors and generation
/// metadata.
///
/// This is independent from the Quantum IR version and from individual
/// application algorithm versions.
pub const APPLICATION_GENERATOR_SCHEMA_VERSION: u16 = 1;

/// Current generation-contract revision.
///
/// Increase this only when the semantics of the generator contract change.
pub const APPLICATION_GENERATION_CONTRACT_VERSION: u16 = 1;

/// Maximum UTF-8 byte length of a generator identifier.
pub const MAX_GENERATOR_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a generator version identifier.
pub const MAX_GENERATOR_VERSION_BYTES: usize = 64;

/// Maximum UTF-8 byte length of a generator description.
pub const MAX_GENERATOR_DESCRIPTION_BYTES: usize = 4096;

/// Maximum number of parameters accepted by one generation request.
///
/// The canonical workload model currently permits the same order of
/// magnitude. Keeping a local bound prevents the generator layer from
/// accidentally becoming more permissive than the workload representation.
pub const MAX_GENERATION_PARAMETERS: usize = 256;

/// Maximum number of generated application instances requested by one
/// generator operation.
pub const MAX_GENERATED_INSTANCES: usize = 1_000_000;

/// Maximum problem size representable directly by this generation contract.
///
/// This is deliberately a conservative structural guard. Larger workloads
/// can be supported by future configuration/limits layers without changing
/// the semantic generator API.
pub const MAX_PROBLEM_SIZE: usize = usize::MAX / 2;

/// Maximum byte length of a benchmark case/instance identifier.
pub const MAX_INSTANCE_ID_BYTES: usize = 128;

/// Maximum number of custom generation tags.
pub const MAX_GENERATION_TAGS: usize = 64;

/// Maximum UTF-8 byte length of one generation tag.
pub const MAX_GENERATION_TAG_BYTES: usize = 64;

// =============================================================================
// Generator capability
// =============================================================================

/// Execution/workload capability exposed by an application generator.
///
/// This is descriptive metadata. Capability negotiation with a concrete
/// backend belongs to the hardware/execution layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApplicationGeneratorCapability {
    /// Generator normally produces a Quantum IR circuit.
    GeneratesCircuit,

    /// Generator may produce an application without a circuit.
    NonCircuit,

    /// Generator represents a hybrid quantum/classical workload.
    Hybrid,

    /// Generator is deterministic for identical generation inputs.
    Deterministic,

    /// Generator can produce multiple independent instances.
    BatchGeneration,

    /// Generator can represent arbitrary problem sizes.
    ScalableProblemSize,

    /// Generator supports user-supplied application parameters.
    Parameterized,

    /// Generator provides an exact small-instance reference.
    ExactSmallInstanceReference,

    /// Generator provides a classical verification path.
    ClassicallyVerifiable,

    /// Generator may be used for resource-estimation workloads.
    ResourceEstimation,

    /// Generator represents a fault-tolerant/logical application.
    LogicalQubit,

    /// Generator may produce workloads suitable for noisy hardware.
    HardwareExecutable,
}

impl ApplicationGeneratorCapability {
    /// Stable machine-readable capability identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratesCircuit => "generates_circuit",
            Self::NonCircuit => "non_circuit",
            Self::Hybrid => "hybrid",
            Self::Deterministic => "deterministic",
            Self::BatchGeneration => "batch_generation",
            Self::ScalableProblemSize => "scalable_problem_size",
            Self::Parameterized => "parameterized",
            Self::ExactSmallInstanceReference => "exact_small_instance_reference",
            Self::ClassicallyVerifiable => "classically_verifiable",
            Self::ResourceEstimation => "resource_estimation",
            Self::LogicalQubit => "logical_qubit",
            Self::HardwareExecutable => "hardware_executable",
        }
    }
}

impl fmt::Display for ApplicationGeneratorCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Generator identity
// =============================================================================

/// Stable identity and capabilities of an application generator.
///
/// This metadata is deliberately separate from an application workload.
/// Multiple generator implementations can represent the same application
/// while differing in circuit construction, approximation strategy, or
/// optimization policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationGeneratorDescriptor {
    generator_id: String,
    application_id: String,
    version: String,
    description: String,
    capabilities: Vec<ApplicationGeneratorCapability>,
}

impl ApplicationGeneratorDescriptor {
    /// Creates a validated generator descriptor.
    pub fn new<G, A, V, D>(
        generator_id: G,
        application_id: A,
        version: V,
        description: D,
    ) -> BenchmarkResult<Self>
    where
        G: Into<String>,
        A: Into<String>,
        V: Into<String>,
        D: Into<String>,
    {
        let generator_id = generator_id.into();
        let application_id = application_id.into();
        let version = version.into();
        let description = description.into();

        validate_identifier(
            "generator_id",
            &generator_id,
            MAX_GENERATOR_ID_BYTES,
        )?;

        validate_identifier(
            "application_id",
            &application_id,
            MAX_GENERATOR_ID_BYTES,
        )?;

        validate_identifier(
            "generator_version",
            &version,
            MAX_GENERATOR_VERSION_BYTES,
        )?;

        validate_bounded_text(
            "generator_description",
            &description,
            MAX_GENERATOR_DESCRIPTION_BYTES,
        )?;

        Ok(Self {
            generator_id,
            application_id,
            version,
            description,
            capabilities: Vec::new(),
        })
    }

    /// Adds one capability.
    ///
    /// Duplicate capabilities are ignored so that descriptor equality remains
    /// stable regardless of repeated registration attempts.
    pub fn with_capability(
        mut self,
        capability: ApplicationGeneratorCapability,
    ) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
            self.capabilities.sort_unstable();
        }

        self
    }

    /// Adds multiple capabilities.
    pub fn with_capabilities<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = ApplicationGeneratorCapability>,
    {
        for capability in capabilities {
            if !self.capabilities.contains(&capability) {
                self.capabilities.push(capability);
            }
        }

        self.capabilities.sort_unstable();
        self
    }

    /// Returns the stable generator identifier.
    #[must_use]
    pub fn generator_id(&self) -> &str {
        &self.generator_id
    }

    /// Returns the application identifier represented by this generator.
    #[must_use]
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    /// Returns the generator version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the generator description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the immutable capability list.
    #[must_use]
    pub fn capabilities(&self) -> &[ApplicationGeneratorCapability] {
        &self.capabilities
    }

    /// Returns whether the generator exposes a capability.
    #[must_use]
    pub fn supports(
        &self,
        capability: ApplicationGeneratorCapability,
    ) -> bool {
        self.capabilities.contains(&capability)
    }
}

// =============================================================================
// Generation metadata
// =============================================================================

/// Deterministic metadata identifying one generation operation.
///
/// This object is intentionally independent from the resulting
/// `ApplicationWorkload`. It can therefore be persisted by provenance/result
/// layers without modifying the workload semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationGenerationMetadata {
    /// User/experiment supplied seed.
    seed: u64,

    /// Position of this generated instance in a deterministic generation
    /// sequence.
    sequence_index: u64,

    /// Generator contract revision.
    generator_revision: u32,

    /// Generator schema revision.
    schema_version: u16,
}

impl ApplicationGenerationMetadata {
    /// Creates generation metadata.
    pub const fn new(
        seed: u64,
        sequence_index: u64,
        generator_revision: u32,
    ) -> Self {
        Self {
            seed,
            sequence_index,
            generator_revision,
            schema_version: APPLICATION_GENERATOR_SCHEMA_VERSION,
        }
    }

    /// Returns the deterministic seed.
    #[must_use]
    pub const fn seed(self) -> u64 {
        self.seed
    }

    /// Returns the generation sequence index.
    #[must_use]
    pub const fn sequence_index(self) -> u64 {
        self.sequence_index
    }

    /// Returns the generator revision.
    #[must_use]
    pub const fn generator_revision(self) -> u32 {
        self.generator_revision
    }

    /// Returns the generator schema version.
    #[must_use]
    pub const fn schema_version(self) -> u16 {
        self.schema_version
    }
}

// =============================================================================
// Generation tags
// =============================================================================

/// Bounded, deterministic metadata attached to a generation request.
///
/// Tags are descriptive only. They must not affect application semantics
/// unless the concrete generator explicitly documents them as generation
/// inputs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerationTag {
    value: String,
}

impl GenerationTag {
    /// Creates a validated generation tag.
    pub fn new<S: Into<String>>(value: S) -> BenchmarkResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(BenchmarkError::InvalidIdentifier {
                field: "generation_tag".to_owned(),
                value,
            });
        }

        if value.len() > MAX_GENERATION_TAG_BYTES {
            return Err(BenchmarkError::InvalidRange {
                field: "generation_tag".to_owned(),
                value: value.len().to_string(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_GENERATION_TAG_BYTES.to_string()),
            });
        }

        if !value
            .bytes()
            .all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'
                    || byte == b'-'
            })
        {
            return Err(BenchmarkError::InvalidIdentifier {
                field: "generation_tag".to_owned(),
                value,
            });
        }

        Ok(Self { value })
    }

    /// Returns the tag value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for GenerationTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GenerationTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Generation request
// =============================================================================

/// Request for one application benchmark instance.
///
/// The request contains only generation inputs. It deliberately contains no:
///
/// - backend;
/// - shots;
/// - timeout;
/// - queue configuration;
/// - compiler configuration;
/// - routing configuration;
/// - scheduling configuration;
/// - statistical configuration.
///
/// Those belong to experiment/execution configuration.
#[derive(Debug, Clone)]
pub struct ApplicationGenerationRequest {
    application_id: String,
    instance_id: WorkloadId,
    problem_size: usize,
    parameters: Vec<ApplicationParameter>,
    metadata: ApplicationGenerationMetadata,
    tags: Vec<GenerationTag>,
}

impl ApplicationGenerationRequest {
    /// Creates a validated generation request.
    pub fn new<A>(
        application_id: A,
        instance_id: WorkloadId,
        problem_size: usize,
        seed: u64,
    ) -> BenchmarkResult<Self>
    where
        A: Into<String>,
    {
        Self::with_metadata(
            application_id,
            instance_id,
            problem_size,
            Vec::new(),
            ApplicationGenerationMetadata::new(seed, 0, 0),
        )
    }

    /// Creates a request with explicit generation metadata.
    pub fn with_metadata<A>(
        application_id: A,
        instance_id: WorkloadId,
        problem_size: usize,
        parameters: Vec<ApplicationParameter>,
        metadata: ApplicationGenerationMetadata,
    ) -> BenchmarkResult<Self>
    where
        A: Into<String>,
    {
        let application_id = application_id.into();

        validate_identifier(
            "application_id",
            &application_id,
            MAX_GENERATOR_ID_BYTES,
        )?;

        validate_instance_id(&instance_id)?;

        validate_problem_size(problem_size)?;

        validate_parameter_count(parameters.len())?;

        Ok(Self {
            application_id,
            instance_id,
            problem_size,
            parameters,
            metadata,
            tags: Vec::new(),
        })
    }

    /// Adds one application parameter.
    pub fn with_parameter(
        mut self,
        parameter: ApplicationParameter,
    ) -> BenchmarkResult<Self> {
        validate_parameter_count(
            self.parameters.len().saturating_add(1),
        )?;

        self.parameters.push(parameter);
        Ok(self)
    }

    /// Adds multiple application parameters.
    pub fn with_parameters<I>(
        mut self,
        parameters: I,
    ) -> BenchmarkResult<Self>
    where
        I: IntoIterator<Item = ApplicationParameter>,
    {
        for parameter in parameters {
            validate_parameter_count(
                self.parameters.len().saturating_add(1),
            )?;

            self.parameters.push(parameter);
        }

        Ok(self)
    }

    /// Adds one generation tag.
    pub fn with_tag(
        mut self,
        tag: GenerationTag,
    ) -> BenchmarkResult<Self> {
        if self.tags.len() >= MAX_GENERATION_TAGS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "application_generation_tags".to_owned(),
                requested: self.tags.len() as u64 + 1,
                maximum: MAX_GENERATION_TAGS as u64,
            });
        }

        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }

        Ok(self)
    }

    /// Adds multiple generation tags.
    pub fn with_tags<I>(
        mut self,
        tags: I,
    ) -> BenchmarkResult<Self>
    where
        I: IntoIterator<Item = GenerationTag>,
    {
        for tag in tags {
            if self.tags.len() >= MAX_GENERATION_TAGS {
                return Err(BenchmarkError::ResourceLimitExceeded {
                    resource: "application_generation_tags".to_owned(),
                    requested: self.tags.len() as u64 + 1,
                    maximum: MAX_GENERATION_TAGS as u64,
                });
            }

            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }

        Ok(self)
    }

    /// Returns the application identifier.
    #[must_use]
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    /// Returns the workload instance identifier.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        &self.instance_id
    }

    /// Returns the requested problem size.
    #[must_use]
    pub const fn problem_size(&self) -> usize {
        self.problem_size
    }

    /// Returns the application parameters.
    #[must_use]
    pub fn parameters(&self) -> &[ApplicationParameter] {
        &self.parameters
    }

    /// Returns generation metadata.
    #[must_use]
    pub const fn metadata(&self) -> ApplicationGenerationMetadata {
        self.metadata
    }

    /// Returns generation tags.
    #[must_use]
    pub fn tags(&self) -> &[GenerationTag] {
        &self.tags
    }

    /// Returns a copy of this request with a different sequence index.
    ///
    /// This is useful for deterministic batch generation.
    #[must_use]
    pub const fn with_sequence_index(
        mut self,
        sequence_index: u64,
    ) -> Self {
        self.metadata = ApplicationGenerationMetadata::new(
            self.metadata.seed(),
            sequence_index,
            self.metadata.generator_revision(),
        );
        self
    }

    /// Returns a copy of this request with a different generator revision.
    #[must_use]
    pub const fn with_generator_revision(
        mut self,
        generator_revision: u32,
    ) -> Self {
        self.metadata = ApplicationGenerationMetadata::new(
            self.metadata.seed(),
            self.metadata.sequence_index(),
            generator_revision,
        );
        self
    }

    /// Validates the complete request.
    pub fn validate(&self) -> BenchmarkResult<()> {
        validate_identifier(
            "application_id",
            &self.application_id,
            MAX_GENERATOR_ID_BYTES,
        )?;

        validate_instance_id(&self.instance_id)?;
        validate_problem_size(self.problem_size)?;
        validate_parameter_count(self.parameters.len())?;

        if self.metadata.schema_version()
            != APPLICATION_GENERATOR_SCHEMA_VERSION
        {
            return Err(BenchmarkError::ReproducibilityFailure {
                component: "application_generator_schema".to_owned(),
                expected: APPLICATION_GENERATOR_SCHEMA_VERSION.to_string(),
                actual: self.metadata.schema_version().to_string(),
            });
        }

        if self.tags.len() > MAX_GENERATION_TAGS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "application_generation_tags".to_owned(),
                requested: self.tags.len() as u64,
                maximum: MAX_GENERATION_TAGS as u64,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Generated application
// =============================================================================

/// Result of application workload generation.
///
/// The workload is the canonical semantic object. Generation metadata is
/// carried alongside it so provenance layers can persist exactly how the
/// workload was produced.
#[derive(Debug, Clone)]
pub struct ApplicationGeneration {
    workload: ApplicationWorkload,
    metadata: ApplicationGenerationMetadata,
    generator_id: String,
    generator_version: String,
}

impl ApplicationGeneration {
    /// Creates a generation result after validating the generated workload
    /// against the request.
    pub fn new(
        workload: ApplicationWorkload,
        request: &ApplicationGenerationRequest,
        descriptor: &ApplicationGeneratorDescriptor,
    ) -> BenchmarkResult<Self> {
        request.validate()?;

        if workload.application_id() != request.application_id() {
            return Err(BenchmarkError::InvalidWorkload {
                workload: request.application_id().to_owned(),
                reason: "generated workload application_id does not match the generation request"
                    .to_owned(),
            });
        }

        if workload.instance_id() != request.instance_id() {
            return Err(BenchmarkError::InvalidWorkload {
                workload: request.application_id().to_owned(),
                reason: "generated workload instance_id does not match the generation request"
                    .to_owned(),
            });
        }

        if workload.problem_size() != request.problem_size() {
            return Err(BenchmarkError::InvalidWorkload {
                workload: request.application_id().to_owned(),
                reason: "generated workload problem_size does not match the generation request"
                    .to_owned(),
            });
        }

        if descriptor.application_id() != request.application_id() {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "generator.application_id".to_owned(),
                second: "request.application_id".to_owned(),
                reason: "application identifiers must match".to_owned(),
            });
        }

        Ok(Self {
            workload,
            metadata: request.metadata(),
            generator_id: descriptor.generator_id().to_owned(),
            generator_version: descriptor.version().to_owned(),
        })
    }

    /// Returns the canonical application workload.
    #[must_use]
    pub fn workload(&self) -> &ApplicationWorkload {
        &self.workload
    }

    /// Consumes the result and returns the canonical workload.
    #[must_use]
    pub fn into_workload(self) -> ApplicationWorkload {
        self.workload
    }

    /// Returns deterministic generation metadata.
    #[must_use]
    pub const fn metadata(&self) -> ApplicationGenerationMetadata {
        self.metadata
    }

    /// Returns the generator identifier.
    #[must_use]
    pub fn generator_id(&self) -> &str {
        &self.generator_id
    }

    /// Returns the generator version.
    #[must_use]
    pub fn generator_version(&self) -> &str {
        &self.generator_version
    }
}

// =============================================================================
// Application generator trait
// =============================================================================

/// Production contract implemented by application benchmark generators.
///
/// Implementations should be small, deterministic construction layers.
///
/// The implementation for an application such as QAOA, Grover, VQE, or QFT
/// belongs under `src/quantum/benchmarking/applications/`, while reusable
/// construction logic can live under `generators/`.
///
/// A generator is expected to:
///
/// 1. validate the request;
/// 2. verify that the application ID matches;
/// 3. construct the application workload;
/// 4. optionally construct a Quantum IR circuit;
/// 5. return the canonical `ApplicationWorkload`.
///
/// It must not execute the workload.
///
/// # Determinism
///
/// For deterministic generators, identical:
///
/// ```text
/// descriptor
/// + request
/// + seed
/// + generator revision
/// ```
///
/// must produce semantically identical workloads.
///
/// If a generator intentionally produces nondeterministic workloads, it must
/// not advertise `Deterministic` capability and must provide sufficient
/// provenance through the surrounding benchmark system.
pub trait ApplicationBenchmarkGenerator: Send + Sync {
    /// Returns immutable generator metadata.
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor;

    /// Performs generator-specific validation.
    ///
    /// The default implementation checks the common contract.
    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;

        if self.descriptor().application_id()
            != request.application_id()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "generator.application_id".to_owned(),
                second: "request.application_id".to_owned(),
                reason: "generator and request application identifiers must match"
                    .to_owned(),
            });
        }

        Ok(())
    }

    /// Generates one application workload.
    ///
    /// Implementations must not execute the generated workload.
    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload>;

    /// Generates and validates one application benchmark instance.
    fn generate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationGeneration> {
        self.validate(request)?;

        let workload = self.generate_workload(request)?;

        ApplicationGeneration::new(
            workload,
            request,
            self.descriptor(),
        )
    }

    /// Generates a deterministic sequence of application workloads.
    ///
    /// Each generated request receives a monotonically increasing sequence
    /// index beginning at the request's existing sequence index.
    ///
    /// The seed itself is not changed. Generators that need independent
    /// streams must derive them through the common random-generation layer
    /// rather than modifying the semantic seed here.
    fn generate_batch(
        &self,
        request: &ApplicationGenerationRequest,
        count: usize,
    ) -> BenchmarkResult<Vec<ApplicationGeneration>> {
        self.validate(request)?;

        if count == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "application_generation_count".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_GENERATED_INSTANCES.to_string()),
            });
        }

        if count > MAX_GENERATED_INSTANCES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "application_generation_count".to_owned(),
                requested: count as u64,
                maximum: MAX_GENERATED_INSTANCES as u64,
            });
        }

        let end = request
            .metadata()
            .sequence_index()
            .checked_add(count as u64)
            .ok_or_else(|| BenchmarkError::NumericalOverflow {
                operation: "application generation sequence index".to_owned(),
                value: Some(count.to_string()),
            })?;

        let _ = end;

        let mut generated = Vec::with_capacity(count);

        for offset in 0..count {
            let sequence_index = request
                .metadata()
                .sequence_index()
                .checked_add(offset as u64)
                .ok_or_else(|| BenchmarkError::NumericalOverflow {
                    operation: "application generation sequence index".to_owned(),
                    value: Some(offset.to_string()),
                })?;

            let request_for_instance =
                request.clone().with_sequence_index(sequence_index);

            generated.push(self.generate(&request_for_instance)?);
        }

        Ok(generated)
    }
}

// =============================================================================
// Static generator adapter
// =============================================================================

/// Convenience adapter for generators that are represented by a function.
///
/// This avoids requiring every small application benchmark to define a
/// dedicated zero-sized Rust type.
///
/// The function must obey the same contract as
/// `ApplicationBenchmarkGenerator::generate_workload`.
pub struct FunctionApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    descriptor: ApplicationGeneratorDescriptor,
    generator: F,
}

impl<F> FunctionApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    /// Creates a function-backed application generator.
    pub fn new(
        descriptor: ApplicationGeneratorDescriptor,
        generator: F,
    ) -> Self {
        Self {
            descriptor,
            generator,
        }
    }
}

impl<F> ApplicationBenchmarkGenerator
    for FunctionApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        (self.generator)(request)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a machine-readable ASCII identifier.
///
/// Accepted grammar:
///
/// ```text
/// [a-z][a-z0-9_-]*
/// ```
///
/// Hyphens are allowed because application and generator identifiers are
/// external registry identifiers. Internal Rust module names remain governed
/// by Rust's identifier rules.
fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> BenchmarkResult<()> {
    if value.is_empty() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    if value.len() > maximum {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.len().to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(maximum.to_string()),
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    if !bytes.iter().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || *byte == b'_'
            || *byte == b'-'
    }) {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validates bounded descriptive text.
fn validate_bounded_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> BenchmarkResult<()> {
    if value.len() > maximum {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.len().to_string(),
            minimum: Some("0".to_owned()),
            maximum: Some(maximum.to_string()),
        });
    }

    Ok(())
}

/// Validates an application instance identifier.
fn validate_instance_id(
    instance_id: &WorkloadId,
) -> BenchmarkResult<()> {
    if instance_id.as_str().len() > MAX_INSTANCE_ID_BYTES {
        return Err(BenchmarkError::InvalidRange {
            field: "instance_id".to_owned(),
            value: instance_id.as_str().len().to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_INSTANCE_ID_BYTES.to_string()),
        });
    }

    Ok(())
}

/// Validates a problem size.
fn validate_problem_size(
    problem_size: usize,
) -> BenchmarkResult<()> {
    if problem_size == 0 {
        return Err(BenchmarkError::InvalidRange {
            field: "problem_size".to_owned(),
            value: "0".to_owned(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_PROBLEM_SIZE.to_string()),
        });
    }

    if problem_size > MAX_PROBLEM_SIZE {
        return Err(BenchmarkError::InvalidRange {
            field: "problem_size".to_owned(),
            value: problem_size.to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_PROBLEM_SIZE.to_string()),
        });
    }

    Ok(())
}

/// Validates the number of application parameters.
fn validate_parameter_count(
    count: usize,
) -> BenchmarkResult<()> {
    if count > MAX_GENERATION_PARAMETERS {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "application_parameters".to_owned(),
            requested: count as u64,
            maximum: MAX_GENERATION_PARAMETERS as u64,
        });
    }

    Ok(())
}

// =============================================================================
// Workload error translation
// =============================================================================

/// Converts the legacy/local workload validation error into the canonical
/// benchmarking error hierarchy.
///
/// `core::workload` currently exposes `WorkloadError` for intrinsic workload
/// validation. The public generator API nevertheless exposes only
/// `BenchmarkError`, preventing application generators from creating a second
/// error hierarchy at their boundary.
///
/// This adapter can be removed later if `core::workload` itself is migrated
/// completely to `BenchmarkError`.
fn workload_error(
    application_id: &str,
    error: WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload: application_id.to_owned(),
        reason: error.to_string(),
    }
}

// =============================================================================
// Built-in descriptor helpers
// =============================================================================

/// Creates the canonical descriptor for a circuit-producing application
/// generator.
///
/// This helper is intended for application modules such as QFT, Grover,
/// QAOA, VQE, and MaxCut.
pub fn circuit_application_descriptor(
    generator_id: impl Into<String>,
    application_id: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
) -> BenchmarkResult<ApplicationGeneratorDescriptor> {
    ApplicationGeneratorDescriptor::new(
        generator_id,
        application_id,
        version,
        description,
    )
    .map(|descriptor| {
        descriptor.with_capabilities([
            ApplicationGeneratorCapability::GeneratesCircuit,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::HardwareExecutable,
            ApplicationGeneratorCapability::ScalableProblemSize,
        ])
    })
}

/// Creates the canonical descriptor for a hybrid application generator.
///
/// Intended for workloads such as VQE and QAOA where quantum execution is
/// embedded in a classical optimization loop.
pub fn hybrid_application_descriptor(
    generator_id: impl Into<String>,
    application_id: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
) -> BenchmarkResult<ApplicationGeneratorDescriptor> {
    ApplicationGeneratorDescriptor::new(
        generator_id,
        application_id,
        version,
        description,
    )
    .map(|descriptor| {
        descriptor.with_capabilities([
            ApplicationGeneratorCapability::GeneratesCircuit,
            ApplicationGeneratorCapability::Hybrid,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::HardwareExecutable,
            ApplicationGeneratorCapability::ScalableProblemSize,
        ])
    })
}

/// Creates the canonical descriptor for a resource-estimation generator.
///
/// Resource-estimation generators are allowed to produce an
/// `ApplicationWorkload` without an attached circuit.
pub fn resource_estimation_descriptor(
    generator_id: impl Into<String>,
    application_id: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
) -> BenchmarkResult<ApplicationGeneratorDescriptor> {
    ApplicationGeneratorDescriptor::new(
        generator_id,
        application_id,
        version,
        description,
    )
    .map(|descriptor| {
        descriptor.with_capabilities([
            ApplicationGeneratorCapability::NonCircuit,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::ResourceEstimation,
            ApplicationGeneratorCapability::ScalableProblemSize,
        ])
    })
}

// =============================================================================
// Canonical workload constructor
// =============================================================================

/// Constructs the canonical `ApplicationWorkload` from a validated generation
/// request.
///
/// Keeping this constructor here ensures every application generator uses the
/// same request-to-workload conversion and therefore receives identical
/// validation semantics.
///
/// A generator that needs to attach a circuit can call:
///
/// ```text
/// make_application_workload(request)?
///     .with_circuit(circuit_workload)
/// ```
///
/// The circuit itself must be produced by the application/circuit generator
/// and must already be represented by the canonical Quantum IR.
pub fn make_application_workload(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<ApplicationWorkload> {
    request.validate()?;

    let mut workload = ApplicationWorkload::new(
        request.application_id().to_owned(),
        request.instance_id().clone(),
        request.problem_size(),
    )
    .map_err(|error| {
        workload_error(request.application_id(), error)
    })?;

    for parameter in request.parameters() {
        workload
            .add_parameter(parameter.clone())
            .map_err(|error| {
                workload_error(request.application_id(), error)
            })?;
    }

    Ok(workload)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_workload_id(
        value: &str,
    ) -> WorkloadId {
        WorkloadId::new(value).expect("test workload ID must be valid")
    }

    fn test_descriptor()
        -> ApplicationGeneratorDescriptor
    {
        circuit_application_descriptor(
            "test_generator",
            "test_application",
            "1",
            "test application generator",
        )
        .expect("test descriptor must be valid")
    }

    struct TestGenerator {
        descriptor: ApplicationGeneratorDescriptor,
    }

    impl TestGenerator {
        fn new() -> Self {
            Self {
                descriptor: test_descriptor(),
            }
        }
    }

    impl ApplicationBenchmarkGenerator for TestGenerator {
        fn descriptor(
            &self,
        ) -> &ApplicationGeneratorDescriptor {
            &self.descriptor
        }

        fn generate_workload(
            &self,
            request: &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload> {
            make_application_workload(request)
        }
    }

    #[test]
    fn descriptor_validates_and_exposes_identity() {
        let descriptor = test_descriptor();

        assert_eq!(
            descriptor.generator_id(),
            "test_generator"
        );
        assert_eq!(
            descriptor.application_id(),
            "test_application"
        );
        assert_eq!(descriptor.version(), "1");
        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::GeneratesCircuit
            )
        );
        assert!(
            descriptor.supports(
                ApplicationGeneratorCapability::Deterministic
            )
        );
    }

    #[test]
    fn invalid_generator_identifier_is_rejected() {
        let result = ApplicationGeneratorDescriptor::new(
            "InvalidGenerator",
            "test_application",
            "1",
            "description",
        );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_application_identifier_is_rejected() {
        let result = ApplicationGeneratorDescriptor::new(
            "test_generator",
            "InvalidApplication",
            "1",
            "description",
        );

        assert!(result.is_err());
    }

    #[test]
    fn zero_problem_size_is_rejected() {
        let result = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("test_instance"),
            0,
            42,
        );

        assert!(result.is_err());
    }

    #[test]
    fn request_is_deterministically_described() {
        let request_a = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request must be valid");

        let request_b = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request must be valid");

        assert_eq!(
            request_a.metadata(),
            request_b.metadata()
        );
        assert_eq!(
            request_a.application_id(),
            request_b.application_id()
        );
        assert_eq!(
            request_a.problem_size(),
            request_b.problem_size()
        );
    }

    #[test]
    fn sequence_index_can_be_changed_without_changing_seed() {
        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request must be valid");

        let next = request.clone().with_sequence_index(7);

        assert_eq!(next.metadata().seed(), 42);
        assert_eq!(
            next.metadata().sequence_index(),
            7
        );
        assert_eq!(
            request.metadata().sequence_index(),
            0
        );
    }

    #[test]
    fn generator_constructs_canonical_application_workload() {
        let generator = TestGenerator::new();

        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request must be valid");

        let generated = generator
            .generate(&request)
            .expect("generation must succeed");

        assert_eq!(
            generated.workload().application_id(),
            "test_application"
        );
        assert_eq!(
            generated.workload().problem_size(),
            16
        );
        assert_eq!(
            generated.metadata().seed(),
            42
        );
    }

    #[test]
    fn generator_rejects_application_mismatch() {
        let generator = TestGenerator::new();

        let request = ApplicationGenerationRequest::new(
            "other_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request itself is valid");

        let result = generator.generate(&request);

        assert!(result.is_err());
    }

    #[test]
    fn batch_generation_has_stable_sequence_indices() {
        let generator = TestGenerator::new();

        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            8,
            123,
        )
        .expect("request must be valid");

        let generated = generator
            .generate_batch(&request, 3)
            .expect("batch generation must succeed");

        assert_eq!(generated.len(), 3);
        assert_eq!(
            generated[0].metadata().sequence_index(),
            0
        );
        assert_eq!(
            generated[1].metadata().sequence_index(),
            1
        );
        assert_eq!(
            generated[2].metadata().sequence_index(),
            2
        );

        assert_eq!(
            generated[0].metadata().seed(),
            generated[1].metadata().seed()
        );
        assert_eq!(
            generated[1].metadata().seed(),
            generated[2].metadata().seed()
        );
    }

    #[test]
    fn parameterized_application_workload_preserves_parameters() {
        let parameter = ApplicationParameter::new(
            "vertices",
            "16",
        )
        .expect("parameter must be valid");

        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            16,
            42,
        )
        .expect("request must be valid")
        .with_parameter(parameter)
        .expect("parameter must be accepted");

        let generator = TestGenerator::new();

        let generated = generator
            .generate(&request)
            .expect("generation must succeed");

        assert_eq!(
            generated.workload().parameters().len(),
            1
        );
        assert_eq!(
            generated.workload().parameters()[0].name(),
            "vertices"
        );
    }

    #[test]
    fn duplicate_capabilities_are_not_inserted() {
        let descriptor = ApplicationGeneratorDescriptor::new(
            "test_generator",
            "test_application",
            "1",
            "description",
        )
        .expect("descriptor must be valid")
        .with_capability(
            ApplicationGeneratorCapability::Deterministic,
        )
        .with_capability(
            ApplicationGeneratorCapability::Deterministic,
        );

        assert_eq!(
            descriptor
                .capabilities()
                .iter()
                .filter(|capability| {
                    **capability
                        == ApplicationGeneratorCapability::Deterministic
                })
                .count(),
            1
        );
    }

    #[test]
    fn empty_batch_is_rejected() {
        let generator = TestGenerator::new();

        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            8,
            42,
        )
        .expect("request must be valid");

        assert!(
            generator.generate_batch(&request, 0).is_err()
        );
    }

    #[test]
    fn generation_tags_are_bounded_and_validated() {
        let tag = GenerationTag::new("small_instance")
            .expect("tag must be valid");

        assert_eq!(tag.as_str(), "small_instance");
    }

    #[test]
    fn invalid_generation_tag_is_rejected() {
        assert!(
            GenerationTag::new("Invalid Tag").is_err()
        );
    }

    #[test]
    fn built_in_descriptor_helpers_expose_expected_capabilities() {
        let hybrid = hybrid_application_descriptor(
            "vqe_generator",
            "vqe",
            "1",
            "VQE generator",
        )
        .expect("descriptor must be valid");

        assert!(
            hybrid.supports(
                ApplicationGeneratorCapability::Hybrid
            )
        );

        let resource = resource_estimation_descriptor(
            "shor_resource_estimator",
            "shor",
            "1",
            "Shor resource estimator",
        )
        .expect("descriptor must be valid");

        assert!(
            resource.supports(
                ApplicationGeneratorCapability::ResourceEstimation
            )
        );
        assert!(
            resource.supports(
                ApplicationGeneratorCapability::NonCircuit
            )
        );
    }

    #[test]
    fn function_generator_adapter_works() {
        let descriptor = test_descriptor();

        let generator = FunctionApplicationGenerator::new(
            descriptor,
            |request| make_application_workload(request),
        );

        let request = ApplicationGenerationRequest::new(
            "test_application",
            test_workload_id("instance_a"),
            4,
            99,
        )
        .expect("request must be valid");

        let result = generator
            .generate(&request)
            .expect("function generator must work");

        assert_eq!(
            result.workload().application_id(),
            "test_application"
        );
    }
}