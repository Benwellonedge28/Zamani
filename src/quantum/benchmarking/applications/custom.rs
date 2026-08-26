//! Zamani Quantum Benchmarking — Custom Application Benchmarks
//!
//! Production implementation of the user-defined application benchmark
//! boundary.
//!
//! # Architectural role
//!
//! This module is the concrete application-layer adapter for custom Zamani
//! quantum applications. It allows a user-defined application to participate
//! in the same benchmarking pipeline as built-in applications such as:
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
//! - Shor.
//!
//! It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - select a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform statistical analysis;
//! - calculate application metrics;
//! - define a second workload representation;
//! - define a second application-generator trait;
//! - parse Zamani source code;
//! - communicate with hardware;
//! - perform filesystem or network I/O.
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani custom benchmark declaration
//!                  │
//!                  ▼
//!        CustomApplicationDefinition
//!                  │
//!                  ▼
//!       CustomApplicationGenerator
//!                  │
//!                  ▼
//!      ApplicationGenerationRequest
//!                  │
//!                  ▼
//!      ApplicationWorkload
//!                  │
//!          ┌───────┴────────┐
//!          ▼                ▼
//!      Quantum IR       application metadata
//!          │                │
//!          └───────┬────────┘
//!                  ▼
//!           core::experiment
//!                  │
//!                  ▼
//!            execution
//!                  │
//!                  ▼
//!            observation
//!                  │
//!                  ▼
//!             analysis
//! ```
//!
//! # Important architectural rule
//!
//! `core::workload::ApplicationWorkload` remains the canonical workload model.
//!
//! This file MUST NOT introduce another workload structure containing:
//!
//! - gates;
//! - qubits;
//! - circuit depth;
//! - backend information;
//! - shots;
//! - execution timing;
//! - routing;
//! - scheduling.
//!
//! Those belong to the canonical workload, IR, execution, hardware and
//! analysis layers respectively.
//!
//! # Custom applications
//!
//! A custom application can be:
//!
//! - circuit-based;
//! - hybrid quantum/classical;
//! - non-circuit;
//! - resource-estimation-only;
//! - simulator-oriented;
//! - hardware-executable;
//! - logically encoded;
//! - classically verifiable;
//! - not classically verifiable.
//!
//! The definition records these semantics without coupling the benchmark to a
//! particular backend.
//!
//! # Determinism
//!
//! Custom generators receive an explicit `ApplicationGenerationRequest`.
//! Reproducible generators MUST derive all generation decisions from:
//!
//! - application ID;
//! - instance ID;
//! - problem size;
//! - parameters;
//! - seed;
//! - sequence index;
//! - generator revision.
//!
//! A custom generator MUST NOT use hidden entropy such as:
//!
//! - system time;
//! - process ID;
//! - pointer addresses;
//! - thread IDs;
//! - environment-dependent iteration order;
//! - global mutable RNG state.
//!
//! If a custom generator is intentionally nondeterministic, it must not
//! advertise the deterministic capability.
//!
//! # Security/resource model
//!
//! Custom benchmark definitions may ultimately originate from the Zamani
//! language or external benchmark configuration. They therefore have to be
//! treated as untrusted data at the definition boundary.
//!
//! This module:
//!
//! - validates all identifiers;
//! - bounds textual metadata;
//! - bounds problem sizes;
//! - bounds parameter counts;
//! - bounds custom tags;
//! - validates capability combinations;
//! - validates generation requests;
//! - prevents zero-sized workloads;
//! - rejects invalid registry identifiers;
//! - avoids unchecked arithmetic;
//! - never executes user source code itself;
//! - never performs I/O.
//!
//! Global benchmark limits remain owned by `core::limits` and `core::config`.
//!
//! # Zamani-language integration
//!
//! The intended future flow is:
//!
//! ```text
//! Zamani:
//!
//! benchmark my_algorithm { ... }
//!             │
//!             ▼
//!        frontend/parser
//!             │
//!             ▼
//!     CustomApplicationDefinition
//!             │
//!             ▼
//!     CustomApplicationGenerator
//!             │
//!             ▼
//!       ApplicationWorkload
//!             │
//!             ▼
//!      BenchmarkExperiment
//! ```
//!
//! The frontend should construct this module's public types. It should not
//! generate Quantum IR itself merely to register a benchmark.
//!
//! # Registry integration
//!
//! `registry::registry` can store an implementation of
//! `ApplicationBenchmarkGenerator` behind its existing benchmark registry
//! abstraction.
//!
//! This module therefore contains no global registry and no global mutable
//! state.
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
//! No external dependencies are required.
//!
//! # Integration contract
//!
//! This file depends on the already-established application generator
//! contract:
//!
//! `benchmarking::generators::application`
//!
//! and the canonical workload:
//!
//! `benchmarking::core::workload`.
//!
//! It intentionally does not require later modules such as:
//!
//! - `core::experiment`;
//! - `core::execution`;
//! - `execution::*`;
//! - `statistics::*`;
//! - `metrics::*`;
//! - `reporting::*`;
//! - `registry::*`.
//!
//! Therefore this file can be completed before those modules are implemented.
//!
//! Later modules consume the generated `ApplicationGeneration` exactly as they
//! consume built-in application generators.
//!
//! # Scientific benchmark boundary
//!
//! This module describes *what* a custom application is and constructs its
//! workload. It does not decide whether the application succeeded.
//!
//! Success criteria belong to the benchmark analysis layer because they may
//! depend on:
//!
//! - observed counts;
//! - expectation values;
//! - approximation error;
//! - solution quality;
//! - runtime;
//! - confidence intervals;
//! - application-specific reference data.
//!
//! This separation permits the same custom application to be benchmarked on
//! different execution targets without changing its generator.

use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    WorkloadId,
};

use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGeneration,
    ApplicationGenerationRequest,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
    MAX_GENERATED_INSTANCES,
    MAX_GENERATION_PARAMETERS,
    MAX_GENERATION_TAGS,
    MAX_INSTANCE_ID_BYTES,
    MAX_PROBLEM_SIZE,
    circuit_application_descriptor,
    hybrid_application_descriptor,
    make_application_workload,
    resource_estimation_descriptor,
};

// =============================================================================
// Stable schema/version constants
// =============================================================================

/// Schema version of the custom application definition.
pub const CUSTOM_APPLICATION_SCHEMA_VERSION: u16 = 1;

/// Contract revision for the custom application generator.
pub const CUSTOM_APPLICATION_CONTRACT_VERSION: u16 = 1;

/// Maximum custom application description length in UTF-8 bytes.
pub const MAX_CUSTOM_APPLICATION_DESCRIPTION_BYTES: usize = 4096;

/// Maximum custom application display-name length in UTF-8 bytes.
pub const MAX_CUSTOM_APPLICATION_NAME_BYTES: usize = 256;

/// Maximum number of custom benchmark tags.
pub const MAX_CUSTOM_APPLICATION_TAGS: usize = 64;

/// Maximum UTF-8 byte length of a custom benchmark tag.
pub const MAX_CUSTOM_APPLICATION_TAG_BYTES: usize = 64;

/// Maximum UTF-8 byte length of a verification identifier.
pub const MAX_VERIFICATION_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a benchmark category.
pub const MAX_CATEGORY_BYTES: usize = 128;

/// Maximum number of required capabilities.
pub const MAX_REQUIRED_CAPABILITIES: usize = 32;

// =============================================================================
// Execution model
// =============================================================================

/// Semantic execution model of a custom application.
///
/// This describes the workload family. It does not select a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CustomApplicationExecutionModel {
    /// The application is represented by a Quantum IR circuit.
    Circuit,

    /// The application combines quantum execution with classical processing.
    Hybrid,

    /// The application does not require a circuit at generation time.
    NonCircuit,
}

impl CustomApplicationExecutionModel {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circuit => "circuit",
            Self::Hybrid => "hybrid",
            Self::NonCircuit => "non_circuit",
        }
    }

    /// Returns whether the model requires an attached circuit immediately.
    #[must_use]
    pub const fn requires_circuit(self) -> bool {
        matches!(self, Self::Circuit | Self::Hybrid)
    }
}

impl fmt::Display for CustomApplicationExecutionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Verification model
// =============================================================================

/// Verification model of a custom application.
///
/// This is descriptive metadata used by later analysis/reporting layers.
/// It does not execute a verifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CustomApplicationVerification {
    /// The result can be checked exactly for supported instances.
    Exact,

    /// The result can be checked against a bounded classical reference.
    ClassicalReference,

    /// The result can be checked statistically.
    Statistical,

    /// No verifier is provided by the application definition.
    None,
}

impl CustomApplicationVerification {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ClassicalReference => "classical_reference",
            Self::Statistical => "statistical",
            Self::None => "none",
        }
    }

    /// Returns whether a verification path exists.
    #[must_use]
    pub const fn is_verifiable(self) -> bool {
        !matches!(self, Self::None)
    }
}

impl fmt::Display for CustomApplicationVerification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark category
// =============================================================================

/// High-level category for a custom application.
///
/// Categories are metadata and do not alter execution semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CustomApplicationCategory {
    /// General quantum algorithm.
    Algorithm,

    /// Optimization workload.
    Optimization,

    /// Quantum simulation/chemistry/materials workload.
    Simulation,

    /// Quantum machine-learning workload.
    MachineLearning,

    /// Cryptographic workload.
    Cryptography,

    /// Scientific/numerical workload.
    Scientific,

    /// Fault-tolerant/logical workload.
    FaultTolerant,

    /// Resource-estimation workload.
    ResourceEstimation,

    /// User-defined category.
    Custom,
}

impl CustomApplicationCategory {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Algorithm => "algorithm",
            Self::Optimization => "optimization",
            Self::Simulation => "simulation",
            Self::MachineLearning => "machine_learning",
            Self::Cryptography => "cryptography",
            Self::Scientific => "scientific",
            Self::FaultTolerant => "fault_tolerant",
            Self::ResourceEstimation => "resource_estimation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for CustomApplicationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Custom application tag
// =============================================================================

/// Bounded machine-readable custom application tag.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomApplicationTag(String);

impl CustomApplicationTag {
    /// Creates a validated tag.
    ///
    /// Accepted grammar:
    ///
    /// `[a-z][a-z0-9_-]*`
    pub fn new<S: Into<String>>(value: S) -> BenchmarkResult<Self> {
        let value = value.into();

        validate_identifier(
            "custom_application_tag",
            &value,
            MAX_CUSTOM_APPLICATION_TAG_BYTES,
        )?;

        Ok(Self(value))
    }

    /// Returns the tag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CustomApplicationTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CustomApplicationTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Custom application definition
// =============================================================================

/// Immutable definition of a custom application benchmark.
///
/// This is metadata/configuration. It is not the generated workload.
///
/// A definition can therefore be:
///
/// - registered;
/// - validated;
/// - versioned;
/// - compared;
/// - exposed by the Zamani frontend;
/// - stored in benchmark provenance;
/// - used to construct a generator.
///
/// No execution state is stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomApplicationDefinition {
    application_id: String,
    name: String,
    version: String,
    description: String,
    category: CustomApplicationCategory,
    execution_model: CustomApplicationExecutionModel,
    verification: CustomApplicationVerification,
    verification_id: Option<String>,
    generator_id: String,
    generator_version: String,
    max_problem_size: usize,
    capabilities: Vec<ApplicationGeneratorCapability>,
    tags: Vec<CustomApplicationTag>,
}

impl CustomApplicationDefinition {
    /// Creates a custom application definition.
    pub fn new<A, N, V, D>(
        application_id: A,
        name: N,
        version: V,
        description: D,
        category: CustomApplicationCategory,
        execution_model: CustomApplicationExecutionModel,
        verification: CustomApplicationVerification,
        generator_id: impl Into<String>,
        generator_version: impl Into<String>,
        max_problem_size: usize,
    ) -> BenchmarkResult<Self>
    where
        A: Into<String>,
        N: Into<String>,
        V: Into<String>,
        D: Into<String>,
    {
        let application_id = application_id.into();
        let name = name.into();
        let version = version.into();
        let description = description.into();
        let generator_id = generator_id.into();
        let generator_version = generator_version.into();

        validate_identifier(
            "application_id",
            &application_id,
            128,
        )?;

        validate_bounded_text(
            "custom_application_name",
            &name,
            MAX_CUSTOM_APPLICATION_NAME_BYTES,
        )?;

        if name.trim().is_empty() {
            return Err(BenchmarkError::InvalidIdentifier {
                field: "custom_application_name".to_owned(),
                value: name,
            });
        }

        validate_identifier(
            "application_version",
            &version,
            64,
        )?;

        validate_bounded_text(
            "custom_application_description",
            &description,
            MAX_CUSTOM_APPLICATION_DESCRIPTION_BYTES,
        )?;

        validate_identifier(
            "generator_id",
            &generator_id,
            128,
        )?;

        validate_identifier(
            "generator_version",
            &generator_version,
            64,
        )?;

        validate_problem_size(max_problem_size)?;

        if verification == CustomApplicationVerification::None
            && verification_id.is_some()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "verification".to_owned(),
                second: "verification_id".to_owned(),
                reason: "verification_id cannot be supplied when verification is none"
                    .to_owned(),
            });
        }

        let mut definition = Self {
            application_id,
            name,
            version,
            description,
            category,
            execution_model,
            verification,
            verification_id: None,
            generator_id,
            generator_version,
            max_problem_size,
            capabilities: Vec::new(),
            tags: Vec::new(),
        };

        if let Some(id) = verification_id {
            definition = definition.with_verification_id(id)?;
        }

        definition.validate()?;

        Ok(definition)
    }

    /// Adds one capability.
    #[must_use]
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
    #[must_use]
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

    /// Adds a custom tag.
    pub fn with_tag(
        mut self,
        tag: CustomApplicationTag,
    ) -> BenchmarkResult<Self> {
        if self.tags.len() >= MAX_CUSTOM_APPLICATION_TAGS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "custom_application_tags".to_owned(),
                requested: (self.tags.len() + 1) as u64,
                maximum: MAX_CUSTOM_APPLICATION_TAGS as u64,
            });
        }

        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.tags.sort_unstable();
        }

        Ok(self)
    }

    /// Adds multiple custom tags.
    pub fn with_tags<I>(
        mut self,
        tags: I,
    ) -> BenchmarkResult<Self>
    where
        I: IntoIterator<Item = CustomApplicationTag>,
    {
        for tag in tags {
            if self.tags.len() >= MAX_CUSTOM_APPLICATION_TAGS {
                return Err(BenchmarkError::ResourceLimitExceeded {
                    resource: "custom_application_tags".to_owned(),
                    requested: (self.tags.len() + 1) as u64,
                    maximum: MAX_CUSTOM_APPLICATION_TAGS as u64,
                });
            }

            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }

        self.tags.sort_unstable();
        Ok(self)
    }

    /// Attaches a verification identifier.
    pub fn with_verification_id<S: Into<String>>(
        mut self,
        verification_id: S,
    ) -> BenchmarkResult<Self> {
        let verification_id = verification_id.into();

        validate_identifier(
            "verification_id",
            &verification_id,
            MAX_VERIFICATION_ID_BYTES,
        )?;

        if self.verification == CustomApplicationVerification::None {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "verification".to_owned(),
                second: "verification_id".to_owned(),
                reason: "a verification identifier requires a verification model"
                    .to_owned(),
            });
        }

        self.verification_id = Some(verification_id);
        Ok(self)
    }

    /// Returns the stable application identifier.
    #[must_use]
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the application version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the benchmark category.
    #[must_use]
    pub const fn category(&self) -> CustomApplicationCategory {
        self.category
    }

    /// Returns the execution model.
    #[must_use]
    pub const fn execution_model(&self) -> CustomApplicationExecutionModel {
        self.execution_model
    }

    /// Returns the verification model.
    #[must_use]
    pub const fn verification(&self) -> CustomApplicationVerification {
        self.verification
    }

    /// Returns the verification identifier, if configured.
    #[must_use]
    pub fn verification_id(&self) -> Option<&str> {
        self.verification_id.as_deref()
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

    /// Returns the maximum problem size accepted by this definition.
    #[must_use]
    pub const fn max_problem_size(&self) -> usize {
        self.max_problem_size
    }

    /// Returns the capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[ApplicationGeneratorCapability] {
        &self.capabilities
    }

    /// Returns the tags.
    #[must_use]
    pub fn tags(&self) -> &[CustomApplicationTag] {
        &self.tags
    }

    /// Returns whether a capability is declared.
    #[must_use]
    pub fn supports(
        &self,
        capability: ApplicationGeneratorCapability,
    ) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Validates the complete definition.
    pub fn validate(&self) -> BenchmarkResult<()> {
        validate_identifier(
            "application_id",
            &self.application_id,
            128,
        )?;

        if self.name.trim().is_empty() {
            return Err(BenchmarkError::InvalidIdentifier {
                field: "custom_application_name".to_owned(),
                value: self.name.clone(),
            });
        }

        validate_bounded_text(
            "custom_application_name",
            &self.name,
            MAX_CUSTOM_APPLICATION_NAME_BYTES,
        )?;

        validate_identifier(
            "application_version",
            &self.version,
            64,
        )?;

        validate_bounded_text(
            "custom_application_description",
            &self.description,
            MAX_CUSTOM_APPLICATION_DESCRIPTION_BYTES,
        )?;

        validate_identifier(
            "generator_id",
            &self.generator_id,
            128,
        )?;

        validate_identifier(
            "generator_version",
            &self.generator_version,
            64,
        )?;

        validate_problem_size(self.max_problem_size)?;

        if self.capabilities.len() > MAX_REQUIRED_CAPABILITIES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "custom_application_capabilities".to_owned(),
                requested: self.capabilities.len() as u64,
                maximum: MAX_REQUIRED_CAPABILITIES as u64,
            });
        }

        if self.tags.len() > MAX_CUSTOM_APPLICATION_TAGS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "custom_application_tags".to_owned(),
                requested: self.tags.len() as u64,
                maximum: MAX_CUSTOM_APPLICATION_TAGS as u64,
            });
        }

        if self.execution_model.requires_circuit()
            && !self.supports(
                ApplicationGeneratorCapability::GeneratesCircuit,
            )
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "execution_model".to_owned(),
                second: "capabilities".to_owned(),
                reason: "circuit and hybrid applications must advertise generates_circuit"
                    .to_owned(),
            });
        }

        if self.execution_model == CustomApplicationExecutionModel::Hybrid
            && !self.supports(ApplicationGeneratorCapability::Hybrid)
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "execution_model".to_owned(),
                second: "capabilities".to_owned(),
                reason: "hybrid applications must advertise hybrid capability"
                    .to_owned(),
            });
        }

        if self.execution_model
            == CustomApplicationExecutionModel::NonCircuit
            && self.supports(
                ApplicationGeneratorCapability::GeneratesCircuit,
            )
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "execution_model".to_owned(),
                second: "capabilities".to_owned(),
                reason: "non-circuit applications cannot advertise generates_circuit"
                    .to_owned(),
            });
        }

        if self.verification == CustomApplicationVerification::None
            && self.verification_id.is_some()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "verification".to_owned(),
                second: "verification_id".to_owned(),
                reason: "verification_id cannot be supplied without verification"
                    .to_owned(),
            });
        }

        if self.verification != CustomApplicationVerification::None
            && self.verification_id.is_none()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "verification".to_owned(),
                second: "verification_id".to_owned(),
                reason: "a verifiable application must identify its verification contract"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Custom application generator
// =============================================================================

/// Production custom application generator.
///
/// `F` is trusted application-construction code supplied by the Rust
/// integration layer. The closure is never invoked by the definition object
/// itself; it is invoked only when a benchmark generation operation explicitly
/// requests an application instance.
///
/// The closure:
///
/// - receives a validated request;
/// - must return the canonical `ApplicationWorkload`;
/// - must not execute the workload;
/// - must not perform benchmark analysis;
/// - must not select a backend.
///
/// The public trait implementation additionally verifies the returned
/// workload against the request and definition.
pub struct CustomApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    definition: CustomApplicationDefinition,
    descriptor: ApplicationGeneratorDescriptor,
    generator: F,
}

impl<F> CustomApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    /// Creates a production custom application generator.
    pub fn new(
        definition: CustomApplicationDefinition,
        generator: F,
    ) -> BenchmarkResult<Self> {
        definition.validate()?;

        let descriptor =
            ApplicationGeneratorDescriptor::new(
                definition.generator_id().to_owned(),
                definition.application_id().to_owned(),
                definition.generator_version().to_owned(),
                definition.description().to_owned(),
            )?
            .with_capabilities(
                definition.capabilities().iter().copied(),
            );

        validate_descriptor_against_definition(
            &descriptor,
            &definition,
        )?;

        Ok(Self {
            definition,
            descriptor,
            generator,
        })
    }

    /// Returns the immutable custom application definition.
    #[must_use]
    pub fn definition(&self) -> &CustomApplicationDefinition {
        &self.definition
    }

    /// Returns the generator descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Generates one application benchmark instance.
    pub fn generate_custom(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationGeneration> {
        self.generate(request)
    }

    /// Generates a bounded batch of application instances.
    pub fn generate_custom_batch(
        &self,
        request: &ApplicationGenerationRequest,
        count: usize,
    ) -> BenchmarkResult<Vec<ApplicationGeneration>> {
        self.generate_batch(request, count)
    }
}

impl<F> ApplicationBenchmarkGenerator
    for CustomApplicationGenerator<F>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;

        if request.application_id()
            != self.definition.application_id()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "definition.application_id".to_owned(),
                reason: "custom application identifiers must match"
                    .to_owned(),
            });
        }

        if request.problem_size()
            > self.definition.max_problem_size()
        {
            return Err(BenchmarkError::InvalidRange {
                field: "problem_size".to_owned(),
                value: request.problem_size().to_string(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    self.definition
                        .max_problem_size()
                        .to_string(),
                ),
            });
        }

        if request.parameters().len()
            > MAX_GENERATION_PARAMETERS
        {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "application_parameters".to_owned(),
                requested: request.parameters().len() as u64,
                maximum: MAX_GENERATION_PARAMETERS as u64,
            });
        }

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let workload = (self.generator)(request)?;

        validate_generated_workload(
            &self.definition,
            request,
            &workload,
        )?;

        Ok(workload)
    }
}

// =============================================================================
// Canonical custom workload constructor
// =============================================================================

/// Creates a custom application workload from a validated generation request.
///
/// This is the preferred implementation for custom applications that do not
/// need to construct the circuit inside this module.
///
/// A circuit-producing application can subsequently attach its canonical
/// `CircuitWorkload` through `ApplicationWorkload::with_circuit()`.
pub fn make_custom_application_workload(
    request: &ApplicationGenerationRequest,
) -> BenchmarkResult<ApplicationWorkload> {
    make_application_workload(request)
}

/// Creates a custom application generator around a workload-construction
/// function.
///
/// This is the simplest integration point for small Rust-defined applications.
pub fn function_generator<F>(
    definition: CustomApplicationDefinition,
    generator: F,
) -> BenchmarkResult<CustomApplicationGenerator<F>>
where
    F: Fn(
            &ApplicationGenerationRequest,
        ) -> BenchmarkResult<ApplicationWorkload>
        + Send
        + Sync,
{
    CustomApplicationGenerator::new(
        definition,
        generator,
    )
}

// =============================================================================
// Built-in definition helpers
// =============================================================================

/// Creates a validated circuit-based custom application definition.
///
/// This helper establishes the required capability contract for circuit
/// applications.
pub fn circuit_definition(
    application_id: impl Into<String>,
    name: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
    category: CustomApplicationCategory,
    generator_id: impl Into<String>,
    generator_version: impl Into<String>,
    max_problem_size: usize,
    verification: CustomApplicationVerification,
    verification_id: Option<String>,
) -> BenchmarkResult<CustomApplicationDefinition> {
    let mut definition = CustomApplicationDefinition::new(
        application_id,
        name,
        version,
        description,
        category,
        CustomApplicationExecutionModel::Circuit,
        verification,
        generator_id,
        generator_version,
        max_problem_size,
    )?
    .with_capabilities([
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::HardwareExecutable,
        ApplicationGeneratorCapability::ScalableProblemSize,
    ]);

    if let Some(id) = verification_id {
        definition =
            definition.with_verification_id(id)?;
    }

    definition.validate()?;
    Ok(definition)
}

/// Creates a validated hybrid custom application definition.
///
/// This is suitable for applications such as user-defined VQE/QAOA-like
/// workloads where the benchmark execution layer will perform repeated
/// quantum/classical iterations.
pub fn hybrid_definition(
    application_id: impl Into<String>,
    name: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
    category: CustomApplicationCategory,
    generator_id: impl Into<String>,
    generator_version: impl Into<String>,
    max_problem_size: usize,
    verification: CustomApplicationVerification,
    verification_id: Option<String>,
) -> BenchmarkResult<CustomApplicationDefinition> {
    let mut definition = CustomApplicationDefinition::new(
        application_id,
        name,
        version,
        description,
        category,
        CustomApplicationExecutionModel::Hybrid,
        verification,
        generator_id,
        generator_version,
        max_problem_size,
    )?
    .with_capabilities([
        ApplicationGeneratorCapability::GeneratesCircuit,
        ApplicationGeneratorCapability::Hybrid,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::HardwareExecutable,
        ApplicationGeneratorCapability::ScalableProblemSize,
    ]);

    if let Some(id) = verification_id {
        definition =
            definition.with_verification_id(id)?;
    }

    definition.validate()?;
    Ok(definition)
}

/// Creates a validated non-circuit custom application definition.
///
/// Appropriate for resource-estimation and non-gate-model workloads.
pub fn non_circuit_definition(
    application_id: impl Into<String>,
    name: impl Into<String>,
    version: impl Into<String>,
    description: impl Into<String>,
    category: CustomApplicationCategory,
    generator_id: impl Into<String>,
    generator_version: impl Into<String>,
    max_problem_size: usize,
    verification: CustomApplicationVerification,
    verification_id: Option<String>,
) -> BenchmarkResult<CustomApplicationDefinition> {
    let mut definition = CustomApplicationDefinition::new(
        application_id,
        name,
        version,
        description,
        category,
        CustomApplicationExecutionModel::NonCircuit,
        verification,
        generator_id,
        generator_version,
        max_problem_size,
    )?
    .with_capabilities([
        ApplicationGeneratorCapability::NonCircuit,
        ApplicationGeneratorCapability::Deterministic,
        ApplicationGeneratorCapability::Parameterized,
        ApplicationGeneratorCapability::ScalableProblemSize,
        ApplicationGeneratorCapability::ResourceEstimation,
    ]);

    if let Some(id) = verification_id {
        definition =
            definition.with_verification_id(id)?;
    }

    definition.validate()?;
    Ok(definition)
}

// =============================================================================
// Request helpers
// =============================================================================

/// Creates a validated custom application generation request.
///
/// The application identifier is taken from the definition, preventing a
/// caller from accidentally generating an instance for a different
/// application.
pub fn request(
    definition: &CustomApplicationDefinition,
    instance_id: WorkloadId,
    problem_size: usize,
    seed: u64,
) -> BenchmarkResult<ApplicationGenerationRequest> {
    definition.validate()?;

    ApplicationGenerationRequest::new(
        definition.application_id().to_owned(),
        instance_id,
        problem_size,
        seed,
    )
}

/// Creates a validated request with parameters.
pub fn request_with_parameters(
    definition: &CustomApplicationDefinition,
    instance_id: WorkloadId,
    problem_size: usize,
    seed: u64,
    parameters: Vec<ApplicationParameter>,
) -> BenchmarkResult<ApplicationGenerationRequest> {
    definition.validate()?;

    if parameters.len() > MAX_GENERATION_PARAMETERS {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "application_parameters".to_owned(),
            requested: parameters.len() as u64,
            maximum: MAX_GENERATION_PARAMETERS as u64,
        });
    }

    ApplicationGenerationRequest::with_metadata(
        definition.application_id().to_owned(),
        instance_id,
        problem_size,
        parameters,
        super::super::generators::application::ApplicationGenerationMetadata::new(
            seed,
            0,
            0,
        ),
    )
}

// =============================================================================
// Validation
// =============================================================================

/// Validates a generated custom workload against its definition and request.
///
/// This is intentionally stricter than the generic generator contract.
///
/// It prevents a buggy custom generator from returning a workload that claims
/// to be a different application or problem size.
fn validate_generated_workload(
    definition: &CustomApplicationDefinition,
    request: &ApplicationGenerationRequest,
    workload: &ApplicationWorkload,
) -> BenchmarkResult<()> {
    if workload.application_id()
        != definition.application_id()
    {
        return Err(BenchmarkError::InvalidWorkload {
            workload: definition.application_id().to_owned(),
            reason:
                "custom generator returned a workload for a different application"
                    .to_owned(),
        });
    }

    if workload.instance_id()
        != request.instance_id()
    {
        return Err(BenchmarkError::InvalidWorkload {
            workload: definition.application_id().to_owned(),
            reason:
                "custom generator returned a workload for a different instance"
                    .to_owned(),
        });
    }

    if workload.problem_size()
        != request.problem_size()
    {
        return Err(BenchmarkError::InvalidWorkload {
            workload: definition.application_id().to_owned(),
            reason:
                "custom generator returned a workload with a different problem size"
                    .to_owned(),
        });
    }

    if workload.parameters()
        != request.parameters()
    {
        return Err(BenchmarkError::InvalidWorkload {
            workload: definition.application_id().to_owned(),
            reason:
                "custom generator returned parameters different from the generation request"
                    .to_owned(),
        });
    }

    match definition.execution_model() {
        CustomApplicationExecutionModel::Circuit
        | CustomApplicationExecutionModel::Hybrid => {
            if !definition.supports(
                ApplicationGeneratorCapability::GeneratesCircuit,
            ) {
                return Err(
                    BenchmarkError::InconsistentConfiguration {
                        first: "execution_model".to_owned(),
                        second: "capabilities".to_owned(),
                        reason:
                            "circuit-capable custom applications must advertise generates_circuit"
                                .to_owned(),
                    },
                );
            }

            // Do not reject a missing circuit here.
            //
            // A generator may legitimately construct the semantic application
            // first and attach the circuit during a later generation/lowering
            // phase. The definition therefore expresses capability rather than
            // prematurely coupling this layer to Quantum IR construction.
        }

        CustomApplicationExecutionModel::NonCircuit => {
            if definition.supports(
                ApplicationGeneratorCapability::GeneratesCircuit,
            ) {
                return Err(
                    BenchmarkError::InconsistentConfiguration {
                        first: "execution_model".to_owned(),
                        second: "capabilities".to_owned(),
                        reason:
                            "non-circuit custom applications cannot advertise generates_circuit"
                                .to_owned(),
                    },
                );
            }
        }
    }

    Ok(())
}

/// Validates an ASCII machine-readable identifier.
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
            maximum: maximum.to_string(),
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

/// Validates descriptor consistency with a custom application definition.
fn validate_descriptor_against_definition(
    descriptor: &ApplicationGeneratorDescriptor,
    definition: &CustomApplicationDefinition,
) -> BenchmarkResult<()> {
    if descriptor.application_id()
        != definition.application_id()
    {
        return Err(BenchmarkError::InconsistentConfiguration {
            first: "descriptor.application_id".to_owned(),
            second: "definition.application_id".to_owned(),
            reason: "custom application descriptor and definition must identify the same application"
                .to_owned(),
        });
    }

    if descriptor.generator_id()
        != definition.generator_id()
    {
        return Err(BenchmarkError::InconsistentConfiguration {
            first: "descriptor.generator_id".to_owned(),
            second: "definition.generator_id".to_owned(),
            reason: "custom generator descriptor and definition must identify the same generator"
                .to_owned(),
        });
    }

    if descriptor.version()
        != definition.generator_version()
    {
        return Err(BenchmarkError::InconsistentConfiguration {
            first: "descriptor.version".to_owned(),
            second: "definition.generator_version".to_owned(),
            reason: "custom generator version must match its definition"
                .to_owned(),
        });
    }

    for capability in definition.capabilities() {
        if !descriptor.supports(*capability) {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "descriptor.capabilities".to_owned(),
                second: "definition.capabilities".to_owned(),
                reason:
                    "descriptor is missing a capability declared by the custom application definition"
                        .to_owned(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn application_id() -> &'static str {
        "custom_application"
    }

    fn definition() -> CustomApplicationDefinition {
        circuit_definition(
            application_id(),
            "Custom Application",
            "1",
            "A production custom quantum application.",
            CustomApplicationCategory::Algorithm,
            "custom_application_generator",
            "1",
            64,
            CustomApplicationVerification::ClassicalReference,
            Some("custom_reference_v1".to_owned()),
        )
        .expect("definition must be valid")
    }

    fn instance_id() -> WorkloadId {
        WorkloadId::new("instance_0")
            .expect("instance identifier must be valid")
    }

    #[test]
    fn circuit_definition_has_consistent_capabilities() {
        let definition = definition();

        assert_eq!(
            definition.execution_model(),
            CustomApplicationExecutionModel::Circuit
        );

        assert!(
            definition.supports(
                ApplicationGeneratorCapability::GeneratesCircuit
            )
        );

        assert!(
            definition.supports(
                ApplicationGeneratorCapability::Deterministic
            )
        );

        assert_eq!(
            definition.verification(),
            CustomApplicationVerification::ClassicalReference
        );

        assert_eq!(
            definition.verification_id(),
            Some("custom_reference_v1")
        );
    }

    #[test]
    fn invalid_application_identifier_is_rejected() {
        let result = CustomApplicationDefinition::new(
            "InvalidApplication",
            "Application",
            "1",
            "description",
            CustomApplicationCategory::Algorithm,
            CustomApplicationExecutionModel::Circuit,
            CustomApplicationVerification::None,
            "custom_generator",
            "1",
            16,
        );

        assert!(result.is_err());
    }

    #[test]
    fn zero_problem_limit_is_rejected() {
        let result = CustomApplicationDefinition::new(
            application_id(),
            "Application",
            "1",
            "description",
            CustomApplicationCategory::Algorithm,
            CustomApplicationExecutionModel::Circuit,
            CustomApplicationVerification::None,
            "custom_generator",
            "1",
            0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn verification_id_requires_verification() {
        let result = CustomApplicationDefinition::new(
            application_id(),
            "Application",
            "1",
            "description",
            CustomApplicationCategory::Algorithm,
            CustomApplicationExecutionModel::Circuit,
            CustomApplicationVerification::None,
            "custom_generator",
            "1",
            16,
        )
        .and_then(|definition| {
            definition.with_verification_id("reference_v1")
        });

        assert!(result.is_err());
    }

    #[test]
    fn verification_requires_verification_id() {
        let result = CustomApplicationDefinition::new(
            application_id(),
            "Application",
            "1",
            "description",
            CustomApplicationCategory::Algorithm,
            CustomApplicationExecutionModel::Circuit,
            CustomApplicationVerification::Exact,
            "custom_generator",
            "1",
            16,
        );

        assert!(result.is_err());
    }

    #[test]
    fn non_circuit_definition_cannot_advertise_circuit_generation() {
        let result = CustomApplicationDefinition::new(
            application_id(),
            "Application",
            "1",
            "description",
            CustomApplicationCategory::ResourceEstimation,
            CustomApplicationExecutionModel::NonCircuit,
            CustomApplicationVerification::None,
            "custom_generator",
            "1",
            16,
        )
        .map(|definition| {
            definition.with_capability(
                ApplicationGeneratorCapability::GeneratesCircuit,
            )
        })
        .and_then(|definition| {
            definition.validate()
        });

        assert!(result.is_err());
    }

    #[test]
    fn custom_tag_is_bounded_and_machine_readable() {
        assert!(
            CustomApplicationTag::new("optimization")
                .is_ok()
        );

        assert!(
            CustomApplicationTag::new("optimization-v1")
                .is_ok()
        );

        assert!(
            CustomApplicationTag::new("Optimization")
                .is_err()
        );

        assert!(
            CustomApplicationTag::new("")
                .is_err()
        );
    }

    #[test]
    fn request_uses_definition_application_id() {
        let definition = definition();

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        assert_eq!(
            request.application_id(),
            application_id()
        );

        assert_eq!(
            request.problem_size(),
            8
        );

        assert_eq!(
            request.metadata().seed(),
            42
        );
    }

    #[test]
    fn custom_generator_produces_canonical_workload() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        let generated = generator
            .generate_custom(&request)
            .expect("generation must succeed");

        assert_eq!(
            generated.workload().application_id(),
            application_id()
        );

        assert_eq!(
            generated.workload().problem_size(),
            8
        );

        assert_eq!(
            generated.metadata().seed(),
            42
        );
    }

    #[test]
    fn generator_rejects_wrong_application() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                let wrong_request =
                    ApplicationGenerationRequest::new(
                        "another_application",
                        request.instance_id().clone(),
                        request.problem_size(),
                        request.metadata().seed(),
                    )?;

                make_custom_application_workload(
                    &wrong_request,
                )
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        assert!(
            generator.generate_custom(&request).is_err()
        );
    }

    #[test]
    fn generator_rejects_problem_size_above_definition_limit() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            65,
            42,
        )
        .expect("request construction itself is valid");

        assert!(
            generator.generate_custom(&request).is_err()
        );
    }

    #[test]
    fn generator_preserves_request_parameters() {
        let definition = definition();

        let parameter =
            ApplicationParameter::new(
                "precision",
                "0.001",
            )
            .expect("parameter must be valid");

        let request =
            request_with_parameters(
                &definition,
                instance_id(),
                8,
                42,
                vec![parameter],
            )
            .expect("request must be valid");

        let generator = function_generator(
            definition,
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let generated = generator
            .generate_custom(&request)
            .expect("generation must succeed");

        assert_eq!(
            generated.workload().parameters(),
            request.parameters()
        );
    }

    #[test]
    fn batch_generation_is_bounded() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        let generated = generator
            .generate_custom_batch(&request, 3)
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
    }

    #[test]
    fn batch_generation_rejects_zero() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        assert!(
            generator
                .generate_custom_batch(&request, 0)
                .is_err()
        );
    }

    #[test]
    fn batch_generation_limit_is_not_silently_ignored() {
        let definition = definition();

        let generator = function_generator(
            definition.clone(),
            |request| {
                make_custom_application_workload(request)
            },
        )
        .expect("generator must be valid");

        let request = request(
            &definition,
            instance_id(),
            8,
            42,
        )
        .expect("request must be valid");

        let result = generator.generate_custom_batch(
            &request,
            MAX_GENERATED_INSTANCES + 1,
        );

        assert!(result.is_err());
    }

    #[test]
    fn resource_estimation_definition_is_non_circuit() {
        let definition = non_circuit_definition(
            "resource_application",
            "Resource Application",
            "1",
            "Resource-estimation-only application.",
            CustomApplicationCategory::ResourceEstimation,
            "resource_generator",
            "1",
            1024,
            CustomApplicationVerification::None,
            None,
        )
        .expect("definition must be valid");

        assert_eq!(
            definition.execution_model(),
            CustomApplicationExecutionModel::NonCircuit
        );

        assert!(
            definition.supports(
                ApplicationGeneratorCapability::NonCircuit
            )
        );

        assert!(
            !definition.supports(
                ApplicationGeneratorCapability::GeneratesCircuit
            )
        );
    }
}