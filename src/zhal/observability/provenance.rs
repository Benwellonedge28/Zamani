#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware and Execution Provenance
//!
//! Path: `src/zhal/observability/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides structured, portable provenance records for hardware-backed
//! observations and operations crossing the ZHAL boundary.
//!
//! Provenance describes the origin and supporting references associated
//! with hardware observations, provider interactions, operations and
//! results.
//!
//! It supports:
//!
//! - Classical hardware.
//! - Quantum hardware.
//! - HDL realization.
//! - FPGA and ASIC hardware.
//! - Computational accelerators.
//! - Embedded hardware.
//! - Hybrid execution.
//! - Distributed and remote hardware.
//! - Future computational technologies.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Provenance reference identifiers.
//! - Hardware artifact revision references.
//! - Provider/device provenance composition.
//! - Capability-evidence references.
//! - Resource-observation references.
//! - Operation provenance references.
//! - Result provenance references.
//! - Parent and source provenance references.
//! - Provenance record validation.
//! - Portable provenance serialization.
//!
//! This module does NOT own:
//!
//! - Provider identity definitions.
//! - Hardware device identity definitions.
//! - Capability identity definitions.
//! - Capability evaluation.
//! - Resource allocation.
//! - Resource measurement.
//! - Hardware discovery.
//! - Device sessions.
//! - Operation execution.
//! - Operation outcome classification.
//! - Error classification.
//! - Diagnostic classification.
//! - Telemetry collection.
//! - Event storage.
//! - Event ordering.
//! - Distributed tracing.
//! - Logging.
//! - Metrics.
//! - Authentication.
//! - Authorization.
//! - Cryptographic signing.
//! - Artifact storage.
//! - Retention or archival policy.
//! - Quantum execution semantics.
//! - HDL realization semantics.
//! - Distributed scheduling.
//! - Compiler semantics.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Canonical repository authorities
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Hardware operation identity:
//!     crate::zhal::identity::HardwareOperationId
//!
//! Other hardware identities:
//!     crate::zhal::identity
//!
//! Hardware telemetry observations:
//!     crate::zhal::observability::telemetry
//!
//! Structured diagnostics:
//!     crate::zhal::observability::diagnostics
//!
//! Operation context:
//!     crate::zhal::context
//!
//! Standard-library capability authority:
//!     crate::stdlib::capability
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Runtime observability:
//!     crate::zur::observability
//!
//! ## Provenance semantics
//!
//! Provenance is descriptive evidence about origin and relationships.
//!
//! A provenance reference does not establish:
//!
//! - That the referenced object still exists.
//! - That the referenced observation is current.
//! - That a provider is trusted.
//! - That a capability is currently available.
//! - That a resource has been reserved.
//! - That an operation succeeded.
//! - That a result is correct.
//! - That a hardware artifact is authentic.
//! - That an artifact has been cryptographically verified.
//!
//! Those properties require evidence and validation from their
//! respective authorities.
//!
//! ## Evidence references
//!
//! Capability evidence and resource observations are referenced,
//! not reimplemented.
//!
//! This avoids copying or redefining the standard-library capability
//! model, telemetry observations, resource models or runtime accounting.
//!
//! A reference identifies an externally managed evidence object.
//!
//! Its owner determines retrieval, retention, access control and
//! interpretation.
//!
//! ## Operation and result provenance
//!
//! Operation provenance identifies an individual hardware operation
//! and may refer to its operation contract and input artifact.
//!
//! Result provenance identifies an externally managed result or artifact.
//!
//! This module does not infer operation success from the presence of
//! a result reference.
//!
//! Operation outcome remains owned by the canonical operation/error
//! contracts.
//!
//! ## Distributed provenance
//!
//! Provenance references may identify objects managed by remote
//! providers or distributed execution systems.
//!
//! This module does not assume that references are globally resolvable,
//! that clocks are synchronized, or that remote observations are
//! immediately consistent.
//!
//! Distributed systems remain responsible for reference resolution,
//! ordering, clock interpretation and consistency.
//!
//! ## Security
//!
//! Provenance records contain identifiers and references, not credentials.
//!
//! Never put passwords, tokens, private keys, authorization headers,
//! session secrets or raw confidential payloads into provenance fields.
//!
//! References are not authorization grants.
//!
//! This module does not sign, authenticate or verify provenance.
//!
//! Access control belongs to the owner of the referenced evidence.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Identifier length.
//! - Namespace depth.
//! - Revision-string length.
//! - Capability evidence references.
//! - Resource observation references.
//! - Parent provenance references.
//! - Provider count.
//! - Device count.
//! - Operation count.
//! - Result count.
//! - Hardware categories.
//! - Computational domains.
//!
//! Collections grow according to available memory and platform
//! representation limits.
//!
//! This module does not maintain an unbounded global provenance journal.
//!
//! Records are constructed on demand and passed to externally owned
//! observability, storage or reporting infrastructure.
//!
//! Literal infinite storage or computation is not promised.
//!
//! Operational budgets and retention policies belong to their owners.
//!
//! ## Timestamp policy
//!
//! This module does not generate timestamps.
//!
//! Hardware telemetry already owns observation timestamps.
//!
//! Distributed execution owns remote clock interpretation.
//!
//! A caller may associate this record with an externally timestamped
//! telemetry observation or event.
//!
//! This avoids creating a competing clock or timestamp authority.
//!
//! ## Grammar boundary
//!
//! Relevant grammar and specifications:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//! - grammar/specification/semantic-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! Hardware grammar describes computational effects and intent.
//!
//! Execution-context grammar describes execution intent.
//!
//! Distributed-execution grammar describes logical distributed intent.
//!
//! The semantic model distinguishes requirements, capabilities,
//! constraints, preferences and hints.
//!
//! None of these grammars defines physical provenance storage.
//!
//! Provenance is a runtime/provider observation and must not alter
//! source-language semantics or select a physical execution target.
//!
//! ## POCO-REAF
//!
//! Provenance must remain independent of a particular physical target.
//!
//! A portable program may acquire different provider, device, firmware,
//! capability and resource provenance in different environments.
//!
//! The program's semantic meaning must not change merely because
//! its physical realization changes.
//!
//! ## Dependencies
//!
//! - Rust standard library.
//! - serde.
//! - crate::stdlib::provider.
//! - crate::zhal::identity.
//!
//! No hardware SDK, operating-system API, network client, clock,
//! storage engine or runtime executor is required.

use core::fmt;

use serde::Serialize;

use crate::stdlib::provider::ProviderId;

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareOperationId,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable provenance component identity.
pub const COMPONENT: &str =
    "zhal.observability.provenance";

/// Stable provenance schema identifier.
///
/// Incompatible serialized-contract changes require a new schema
/// identifier or an explicitly versioned schema migration.
pub const PROVENANCE_SCHEMA: &str =
    "zamani.zhal.provenance.v1";

/// Provenance schema version.
///
/// This is independent of the Zamani language version, Cargo package
/// version, ZHAL API version, provider API version and firmware version.
pub const PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Component safety policy.
pub const SAFETY_POLICY: &str =
    "unsafe-rust-forbidden";

/// Component scalability policy.
pub const SCALABILITY_POLICY: &str =
    "open-world-resource-bounded-provenance";

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Classification of provenance validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProvenanceErrorKind {
    /// A required identifier is empty.
    EmptyIdentifier,

    /// An identifier contains whitespace.
    WhitespaceInIdentifier,

    /// An identifier contains a control character.
    ControlCharacterInIdentifier,

    /// An identifier contains an invalid namespace segment.
    InvalidNamespace,

    /// A revision description is empty.
    EmptyRevision,

    /// A revision description contains invalid characters.
    InvalidRevision,

    /// The device belongs to a different provider.
    ProviderDeviceMismatch,

    /// An operation reference is invalid.
    InvalidOperationReference,

    /// A result reference is invalid.
    InvalidResultReference,
}

impl fmt::Display for ProvenanceErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::EmptyIdentifier => {
                "provenance identifier must not be empty"
            }
            Self::WhitespaceInIdentifier => {
                "provenance identifier must not contain whitespace"
            }
            Self::ControlCharacterInIdentifier => {
                "provenance identifier must not contain control characters"
            }
            Self::InvalidNamespace => {
                "provenance namespace is invalid"
            }
            Self::EmptyRevision => {
                "artifact revision must not be empty"
            }
            Self::InvalidRevision => {
                "artifact revision contains invalid characters"
            }
            Self::ProviderDeviceMismatch => {
                "device provider identity does not match provenance provider"
            }
            Self::InvalidOperationReference => {
                "operation provenance reference is invalid"
            }
            Self::InvalidResultReference => {
                "result provenance reference is invalid"
            }
        };

        formatter.write_str(message)
    }
}

/// Structured provenance validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceError {
    kind: ProvenanceErrorKind,
    component: &'static str,
}

impl ProvenanceError {
    fn new(
        kind: ProvenanceErrorKind,
        component: &'static str,
    ) -> Self {
        Self {
            kind,
            component,
        }
    }

    /// Returns the validation error classification.
    #[must_use]
    pub const fn kind(&self) -> ProvenanceErrorKind {
        self.kind
    }

    /// Returns the component that failed validation.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }
}

impl fmt::Display for ProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.component,
            self.kind,
        )
    }
}

impl std::error::Error for ProvenanceError {}

/// Result type for provenance construction and validation.
pub type ProvenanceResult<T> =
    Result<T, ProvenanceError>;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_opaque_identifier(
    value: &str,
    component: &'static str,
) -> ProvenanceResult<()> {
    if value.is_empty() {
        return Err(ProvenanceError::new(
            ProvenanceErrorKind::EmptyIdentifier,
            component,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ProvenanceError::new(
            ProvenanceErrorKind::ControlCharacterInIdentifier,
            component,
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(ProvenanceError::new(
            ProvenanceErrorKind::WhitespaceInIdentifier,
            component,
        ));
    }

    Ok(())
}

fn validate_namespace(
    namespace: &str,
) -> ProvenanceResult<()> {
    validate_opaque_identifier(
        namespace,
        "provenance.namespace",
    )?;

    // Namespace segments are open-world qualified names.
    // They are not tied to a closed hardware-domain enumeration.
    //
    // Dots delimit segments. Empty segments are rejected.
    // Segment spelling remains case-sensitive.
    //
    // No arbitrary namespace depth or length is imposed.
    for segment in namespace.split('.') {
        if segment.is_empty() {
            return Err(ProvenanceError::new(
                ProvenanceErrorKind::InvalidNamespace,
                "provenance.namespace",
            ));
        }

        if !segment.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || character == '_'
                || character == '-'
        }) {
            return Err(ProvenanceError::new(
                ProvenanceErrorKind::InvalidNamespace,
                "provenance.namespace",
            ));
        }
    }

    Ok(())
}

fn validate_revision(
    revision: &str,
) -> ProvenanceResult<()> {
    if revision.is_empty() {
        return Err(ProvenanceError::new(
            ProvenanceErrorKind::EmptyRevision,
            "provenance.revision",
        ));
    }

    if revision.chars().any(char::is_control) {
        return Err(ProvenanceError::new(
            ProvenanceErrorKind::InvalidRevision,
            "provenance.revision",
        ));
    }

    Ok(())
}

// ============================================================================
// PROVENANCE REFERENCES
// ============================================================================

/// Open-world reference to an externally managed provenance object.
///
/// A reference consists of:
///
/// - A namespace identifying the reference authority or object class.
/// - An opaque identifier interpreted by that authority.
///
/// Examples:
///
/// - `stdlib.capability.evidence`
/// - `zhal.resource.observation`
/// - `quantum.hardware.calibration`
/// - `hdl.realization.artifact`
/// - `distributed.execution.operation`
/// - `future.photonic.device.evidence`
///
/// The namespace is intentionally extensible.
///
/// A reference does not imply that the referenced object is available,
/// trusted, current, authenticated or authorized.
///
/// The referenced object's owner controls its resolution and lifecycle.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct ProvenanceReference {
    namespace: String,
    identity: String,
}

impl ProvenanceReference {
    /// Constructs a validated external provenance reference.
    ///
    /// No arbitrary identifier length or namespace-depth limit is imposed.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace or identity is invalid.
    pub fn new(
        namespace: impl Into<String>,
        identity: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let namespace = namespace.into();
        let identity = identity.into();

        validate_namespace(&namespace)?;

        validate_opaque_identifier(
            &identity,
            "provenance.reference.identity",
        )?;

        Ok(Self {
            namespace,
            identity,
        })
    }

    /// Returns the reference namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the opaque reference identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

impl fmt::Display for ProvenanceReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.namespace,
            self.identity,
        )
    }
}

// ============================================================================
// HARDWARE ARTIFACT REVISION
// ============================================================================

/// Identifies a hardware-related artifact and its observed revision.
///
/// This can describe:
///
/// - Hardware revision.
/// - Firmware revision.
/// - Microcode revision.
/// - Device configuration revision.
/// - Calibration artifact revision.
/// - HDL realization artifact revision.
/// - Provider implementation revision.
/// - Other provider-defined hardware artifacts.
///
/// The artifact reference identifies the external artifact authority.
///
/// The revision is the exact producer-supplied revision string.
///
/// This type does not interpret vendor versioning schemes or establish
/// authenticity, compatibility or successful verification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareArtifactRevision {
    artifact: ProvenanceReference,
    revision: String,
}

impl HardwareArtifactRevision {
    /// Constructs an artifact revision reference.
    ///
    /// The revision string is preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns an error if the revision is empty or contains
    /// control characters.
    pub fn new(
        artifact: ProvenanceReference,
        revision: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let revision = revision.into();

        validate_revision(&revision)?;

        Ok(Self {
            artifact,
            revision,
        })
    }

    /// Returns the artifact reference.
    #[must_use]
    pub fn artifact(&self) -> &ProvenanceReference {
        &self.artifact
    }

    /// Returns the exact revision supplied by the artifact authority.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

// ============================================================================
// OPERATION PROVENANCE
// ============================================================================

/// Provenance associated with an individual hardware operation.
///
/// The canonical operation identity is reused from ZHAL identity.
///
/// Optional contract and input references identify externally owned
/// descriptions or artifacts.
///
/// This type does not describe the operation's execution outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationProvenance {
    operation_id: HardwareOperationId,
    contract: Option<ProvenanceReference>,
    input: Option<ProvenanceReference>,
}

impl OperationProvenance {
    /// Constructs operation provenance.
    #[must_use]
    pub fn new(
        operation_id: HardwareOperationId,
    ) -> Self {
        Self {
            operation_id,
            contract: None,
            input: None,
        }
    }

    /// Attaches an external operation-contract reference.
    #[must_use]
    pub fn with_contract(
        mut self,
        contract: ProvenanceReference,
    ) -> Self {
        self.contract = Some(contract);
        self
    }

    /// Attaches an external input-artifact reference.
    ///
    /// The reference identifies an artifact; it does not copy the
    /// input payload into the provenance record.
    #[must_use]
    pub fn with_input(
        mut self,
        input: ProvenanceReference,
    ) -> Self {
        self.input = Some(input);
        self
    }

    /// Returns the individual hardware operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &HardwareOperationId {
        &self.operation_id
    }

    /// Returns the optional operation-contract reference.
    #[must_use]
    pub fn contract(&self) -> Option<&ProvenanceReference> {
        self.contract.as_ref()
    }

    /// Returns the optional input-artifact reference.
    #[must_use]
    pub fn input(&self) -> Option<&ProvenanceReference> {
        self.input.as_ref()
    }
}

// ============================================================================
// RESULT PROVENANCE
// ============================================================================

/// Provenance associated with an externally managed operation result.
///
/// A result reference may identify:
///
/// - A result object.
/// - A measurement collection.
/// - A generated artifact.
/// - A realization report.
/// - A remote result record.
/// - A provider-specific output object.
///
/// The reference does not prove that the operation succeeded.
///
/// Operation outcome remains owned by the canonical operation and
/// error contracts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResultProvenance {
    result: ProvenanceReference,
    artifact: Option<ProvenanceReference>,
}

impl ResultProvenance {
    /// Constructs result provenance from an external result reference.
    #[must_use]
    pub fn new(
        result: ProvenanceReference,
    ) -> Self {
        Self {
            result,
            artifact: None,
        }
    }

    /// Attaches an optional generated-artifact reference.
    #[must_use]
    pub fn with_artifact(
        mut self,
        artifact: ProvenanceReference,
    ) -> Self {
        self.artifact = Some(artifact);
        self
    }

    /// Returns the external result reference.
    #[must_use]
    pub fn result(&self) -> &ProvenanceReference {
        &self.result
    }

    /// Returns the optional generated-artifact reference.
    #[must_use]
    pub fn artifact(&self) -> Option<&ProvenanceReference> {
        self.artifact.as_ref()
    }
}

// ============================================================================
// PROVENANCE RECORD
// ============================================================================

/// Immutable-by-interface provenance record for a hardware interaction.
///
/// The record connects canonical provider/device identity to externally
/// owned capability evidence, resource observations, operation references
/// and result references.
///
/// It is a structured snapshot, not an event journal.
///
/// It does not collect telemetry, classify errors, retain execution
/// history or resolve external references.
///
/// All collections grow according to available resources. There are
/// no arbitrary hardware-count or evidence-count ceilings.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareProvenance {
    schema: &'static str,
    schema_version: u16,

    provider_id: ProviderId,

    device: Option<HardwareDeviceIdentity>,

    hardware_revision: Option<HardwareArtifactRevision>,

    firmware_revision: Option<HardwareArtifactRevision>,

    capability_evidence: Vec<ProvenanceReference>,

    resource_observations: Vec<ProvenanceReference>,

    operation: Option<OperationProvenance>,

    result: Option<ResultProvenance>,

    parents: Vec<ProvenanceReference>,

    source: Option<ProvenanceReference>,
}

impl HardwareProvenance {
    /// Creates a provenance record for a canonical provider.
    ///
    /// The provider identity is reused from the standard library.
    ///
    /// No timestamp is generated. The caller may associate this record
    /// with an externally timestamped telemetry observation or event.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
    ) -> Self {
        Self {
            schema: PROVENANCE_SCHEMA,
            schema_version: PROVENANCE_SCHEMA_VERSION,

            provider_id,

            device: None,

            hardware_revision: None,

            firmware_revision: None,

            capability_evidence: Vec::new(),

            resource_observations: Vec::new(),

            operation: None,

            result: None,

            parents: Vec::new(),

            source: None,
        }
    }

    /// Associates a provider-scoped physical device.
    ///
    /// The device's provider identity must match this record's
    /// provider identity.
    ///
    /// # Errors
    ///
    /// Returns ProviderDeviceMismatch if the provider identities differ.
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> ProvenanceResult<Self> {
        if device.provider_id() != &self.provider_id {
            return Err(ProvenanceError::new(
                ProvenanceErrorKind::ProviderDeviceMismatch,
                "provenance.device",
            ));
        }

        self.device = Some(device);

        Ok(self)
    }

    /// Associates the observed hardware revision.
    #[must_use]
    pub fn with_hardware_revision(
        mut self,
        revision: HardwareArtifactRevision,
    ) -> Self {
        self.hardware_revision = Some(revision);
        self
    }

    /// Associates the observed firmware revision.
    #[must_use]
    pub fn with_firmware_revision(
        mut self,
        revision: HardwareArtifactRevision,
    ) -> Self {
        self.firmware_revision = Some(revision);
        self
    }

    /// Adds an externally managed capability-evidence reference.
    ///
    /// The referenced capability evidence remains owned by the
    /// standard-library or domain-specific capability authority.
    #[must_use]
    pub fn with_capability_evidence(
        mut self,
        evidence: ProvenanceReference,
    ) -> Self {
        self.capability_evidence.push(evidence);
        self
    }

    /// Adds an externally managed resource-observation reference.
    ///
    /// Resource measurements and allocation state remain owned by
    /// telemetry, resource and runtime authorities.
    #[must_use]
    pub fn with_resource_observation(
        mut self,
        observation: ProvenanceReference,
    ) -> Self {
        self.resource_observations.push(observation);
        self
    }

    /// Associates an individual hardware operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: OperationProvenance,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Associates an externally managed result.
    #[must_use]
    pub fn with_result(
        mut self,
        result: ResultProvenance,
    ) -> Self {
        self.result = Some(result);
        self
    }

    /// Adds a parent provenance reference.
    ///
    /// Parent references allow callers to connect records into a
    /// provenance graph without this module owning graph storage,
    /// traversal, cycle detection or distributed consistency.
    #[must_use]
    pub fn with_parent(
        mut self,
        parent: ProvenanceReference,
    ) -> Self {
        self.parents.push(parent);
        self
    }

    /// Associates an external source reference.
    ///
    /// This may identify a source observation, compilation artifact,
    /// provider record or other externally managed origin.
    #[must_use]
    pub fn with_source(
        mut self,
        source: ProvenanceReference,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Validates the record's internal invariants.
    ///
    /// This method does not resolve external references or verify
    /// the truth of provider-supplied evidence.
    ///
    /// # Errors
    ///
    /// Returns ProviderDeviceMismatch if the device provider differs
    /// from the record provider.
    pub fn validate(&self) -> ProvenanceResult<()> {
        if let Some(device) = &self.device {
            if device.provider_id() != &self.provider_id {
                return Err(ProvenanceError::new(
                    ProvenanceErrorKind::ProviderDeviceMismatch,
                    "provenance.device",
                ));
            }
        }

        Ok(())
    }

    /// Returns the schema identifier.
    #[must_use]
    pub const fn schema(&self) -> &'static str {
        self.schema
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the optional provider-scoped hardware device identity.
    #[must_use]
    pub fn device(&self) -> Option<&HardwareDeviceIdentity> {
        self.device.as_ref()
    }

    /// Returns the optional hardware revision.
    #[must_use]
    pub fn hardware_revision(
        &self,
    ) -> Option<&HardwareArtifactRevision> {
        self.hardware_revision.as_ref()
    }

    /// Returns the optional firmware revision.
    #[must_use]
    pub fn firmware_revision(
        &self,
    ) -> Option<&HardwareArtifactRevision> {
        self.firmware_revision.as_ref()
    }

    /// Returns all capability-evidence references.
    #[must_use]
    pub fn capability_evidence(
        &self,
    ) -> &[ProvenanceReference] {
        &self.capability_evidence
    }

    /// Returns all resource-observation references.
    #[must_use]
    pub fn resource_observations(
        &self,
    ) -> &[ProvenanceReference] {
        &self.resource_observations
    }

    /// Returns the optional operation provenance.
    #[must_use]
    pub fn operation(
        &self,
    ) -> Option<&OperationProvenance> {
        self.operation.as_ref()
    }

    /// Returns the optional result provenance.
    #[must_use]
    pub fn result(
        &self,
    ) -> Option<&ResultProvenance> {
        self.result.as_ref()
    }

    /// Returns the parent provenance references.
    #[must_use]
    pub fn parents(
        &self,
    ) -> &[ProvenanceReference] {
        &self.parents
    }

    /// Returns the optional source reference.
    #[must_use]
    pub fn source(
        &self,
    ) -> Option<&ProvenanceReference> {
        self.source.as_ref()
    }

    /// Returns whether a physical device is associated with this record.
    #[must_use]
    pub fn has_device(&self) -> bool {
        self.device.is_some()
    }

    /// Returns whether operation provenance is present.
    #[must_use]
    pub fn has_operation(&self) -> bool {
        self.operation.is_some()
    }

    /// Returns whether result provenance is present.
    #[must_use]
    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    /// Returns whether the record references capability evidence.
    #[must_use]
    pub fn has_capability_evidence(&self) -> bool {
        !self.capability_evidence.is_empty()
    }

    /// Returns whether the record references resource observations.
    #[must_use]
    pub fn has_resource_observations(&self) -> bool {
        !self.resource_observations.is_empty()
    }
}

// ============================================================================
// COMPONENT API
// ============================================================================

/// Returns the stable provenance component identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the provenance schema identifier.
#[must_use]
pub const fn schema_id() -> &'static str {
    PROVENANCE_SCHEMA
}

/// Returns the provenance schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    PROVENANCE_SCHEMA_VERSION
}

/// Returns the component safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    SAFETY_POLICY
}

/// Returns the component scalability policy.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    SCALABILITY_POLICY
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareOperationId,
    };

    fn provider() -> ProviderId {
        ProviderId::new("zamani.test.hardware")
            .expect("valid provider identity")
    }

    fn reference(
        namespace: &str,
        identity: &str,
    ) -> ProvenanceReference {
        ProvenanceReference::new(
            namespace,
            identity,
        )
        .expect("valid provenance reference")
    }

    fn device(
        provider_id: ProviderId,
        device_id: &str,
    ) -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            provider_id,
            DeviceId::new(device_id)
                .expect("valid device identity"),
        )
    }

    // ------------------------------------------------------------------------
    // Reference validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_open_world_namespaces() {
        for namespace in [
            "stdlib.capability.evidence",
            "zhal.resource.observation",
            "quantum.hardware.calibration",
            "hdl.realization.artifact",
            "distributed.execution.operation",
            "future.photonic.device.evidence",
            "future.neuromorphic.provider",
        ] {
            assert!(
                ProvenanceReference::new(
                    namespace,
                    "reference-001",
                )
                .is_ok(),
                "rejected namespace: {namespace}",
            );
        }
    }

    #[test]
    fn rejects_empty_reference_namespace() {
        assert_eq!(
            ProvenanceReference::new(
                "",
                "reference-001",
            )
            .expect_err("empty namespace must fail")
            .kind(),
            ProvenanceErrorKind::EmptyIdentifier,
        );
    }

    #[test]
    fn rejects_empty_reference_identity() {
        assert_eq!(
            ProvenanceReference::new(
                "zhal.hardware",
                "",
            )
            .expect_err("empty identity must fail")
            .kind(),
            ProvenanceErrorKind::EmptyIdentifier,
        );
    }

    #[test]
    fn rejects_invalid_namespace_segments() {
        for namespace in [
            ".hardware",
            "hardware.",
            "hardware..device",
            "hardware/device",
            "hardware device",
        ] {
            assert!(
                ProvenanceReference::new(
                    namespace,
                    "reference-001",
                )
                .is_err(),
                "unexpectedly accepted {namespace}",
            );
        }
    }

    #[test]
    fn rejects_control_characters() {
        assert_eq!(
            ProvenanceReference::new(
                "zhal.hardware",
                "reference\n001",
            )
            .expect_err("control character must fail")
            .kind(),
            ProvenanceErrorKind::ControlCharacterInIdentifier,
        );
    }

    #[test]
    fn accepts_long_identifiers_without_arbitrary_ceiling() {
        let identity = format!(
            "future.{}",
            "hardware-reference.".repeat(1_000),
        );

        let reference = ProvenanceReference::new(
            "future.hardware",
            identity.clone(),
        )
        .expect("long identifier");

        assert_eq!(
            reference.identity(),
            identity,
        );
    }

    // ------------------------------------------------------------------------
    // Artifact revisions
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_hardware_revision() {
        let revision = HardwareArtifactRevision::new(
            reference(
                "hardware.revision",
                "device-model",
            ),
            "revision-2026.11",
        )
        .expect("valid revision");

        assert_eq!(
            revision.revision(),
            "revision-2026.11",
        );

        assert_eq!(
            revision.artifact().namespace(),
            "hardware.revision",
        );
    }

    #[test]
    fn preserves_provider_defined_firmware_revision() {
        let revision = HardwareArtifactRevision::new(
            reference(
                "hardware.firmware",
                "firmware-image",
            ),
            "vendor-build-7.4.custom",
        )
        .expect("valid firmware revision");

        assert_eq!(
            revision.revision(),
            "vendor-build-7.4.custom",
        );
    }

    #[test]
    fn rejects_empty_revision() {
        assert_eq!(
            HardwareArtifactRevision::new(
                reference(
                    "hardware.firmware",
                    "firmware-image",
                ),
                "",
            )
            .expect_err("empty revision must fail")
            .kind(),
            ProvenanceErrorKind::EmptyRevision,
        );
    }

    // ------------------------------------------------------------------------
    // Provider and device integration
    // ------------------------------------------------------------------------

    #[test]
    fn creates_provider_only_provenance() {
        let provenance = HardwareProvenance::new(
            provider(),
        );

        assert_eq!(
            provenance.provider_id().as_str(),
            "zamani.test.hardware",
        );

        assert!(!provenance.has_device());

        assert!(
            provenance.validate().is_ok(),
        );
    }

    #[test]
    fn associates_device_with_matching_provider() {
        let provider = provider();

        let provenance = HardwareProvenance::new(
            provider.clone(),
        )
        .with_device(
            device(
                provider,
                "device.example-01",
            ),
        )
        .expect("matching provider");

        assert!(provenance.has_device());

        assert!(
            provenance.validate().is_ok(),
        );
    }

    #[test]
    fn rejects_device_from_another_provider() {
        let provenance = HardwareProvenance::new(
            ProviderId::new("provider.one")
                .expect("provider one"),
        );

        let result = provenance.with_device(
            device(
                ProviderId::new("provider.two")
                    .expect("provider two"),
                "device.example",
            ),
        );

        assert_eq!(
            result.expect_err("provider mismatch").kind(),
            ProvenanceErrorKind::ProviderDeviceMismatch,
        );
    }

    // ------------------------------------------------------------------------
    // Capability and resource evidence
    // ------------------------------------------------------------------------

    #[test]
    fn records_external_capability_evidence() {
        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_capability_evidence(
            reference(
                "stdlib.capability.evidence",
                "evidence.quantum.execution",
            ),
        )
        .with_capability_evidence(
            reference(
                "quantum.hardware.calibration",
                "calibration.snapshot",
            ),
        );

        assert_eq!(
            provenance.capability_evidence().len(),
            2,
        );
    }

    #[test]
    fn records_external_resource_observations() {
        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_resource_observation(
            reference(
                "zhal.resource.observation",
                "memory.available.snapshot",
            ),
        )
        .with_resource_observation(
            reference(
                "zhal.resource.observation",
                "compute.capacity.snapshot",
            ),
        );

        assert_eq!(
            provenance.resource_observations().len(),
            2,
        );
    }

    // ------------------------------------------------------------------------
    // Operation and result provenance
    // ------------------------------------------------------------------------

    #[test]
    fn records_hardware_operation_provenance() {
        let operation_id = HardwareOperationId::new(
            "operation.example-001",
        )
        .expect("operation identity");

        let operation = OperationProvenance::new(
            operation_id,
        )
        .with_contract(
            reference(
                "provider.operation.contract",
                "hardware.compute.v1",
            ),
        )
        .with_input(
            reference(
                "artifact.input",
                "input.dataset.001",
            ),
        );

        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_operation(operation);

        assert!(provenance.has_operation());

        let recorded = provenance
            .operation()
            .expect("operation provenance");

        assert_eq!(
            recorded.contract()
                .expect("operation contract")
                .identity(),
            "hardware.compute.v1",
        );

        assert_eq!(
            recorded.input()
                .expect("input artifact")
                .identity(),
            "input.dataset.001",
        );
    }

    #[test]
    fn records_result_provenance_without_claiming_success() {
        let result = ResultProvenance::new(
            reference(
                "distributed.execution.result",
                "result.remote.001",
            ),
        )
        .with_artifact(
            reference(
                "hdl.realization.artifact",
                "realization.output.001",
            ),
        );

        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_result(result);

        assert!(provenance.has_result());

        assert_eq!(
            provenance.result()
                .expect("result provenance")
                .result()
                .identity(),
            "result.remote.001",
        );
    }

    // ------------------------------------------------------------------------
    // Provenance graph references
    // ------------------------------------------------------------------------

    #[test]
    fn records_parent_provenance_references() {
        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_parent(
            reference(
                "zhal.provenance",
                "parent.local.001",
            ),
        )
        .with_parent(
            reference(
                "distributed.provenance",
                "parent.remote.002",
            ),
        );

        assert_eq!(
            provenance.parents().len(),
            2,
        );
    }

    #[test]
    fn records_external_source_reference() {
        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_source(
            reference(
                "compiler.artifact",
                "portable-program.001",
            ),
        );

        assert_eq!(
            provenance.source()
                .expect("source reference")
                .identity(),
            "portable-program.001",
        );
    }

    // ------------------------------------------------------------------------
    // Schema and serialization
    // ------------------------------------------------------------------------

    #[test]
    fn exposes_stable_schema_identity() {
        assert_eq!(
            schema_id(),
            "zamani.zhal.provenance.v1",
        );

        assert_eq!(
            schema_version(),
            1,
        );

        assert_eq!(
            component_id(),
            "zhal.observability.provenance",
        );
    }

    #[test]
    fn serializes_portable_provenance() {
        let provenance = HardwareProvenance::new(
            provider(),
        )
        .with_capability_evidence(
            reference(
                "stdlib.capability.evidence",
                "evidence.classical.compute",
            ),
        )
        .with_resource_observation(
            reference(
                "zhal.resource.observation",
                "resource.memory.snapshot",
            ),
        );

        let serialized = serde_json::to_string(
            &provenance,
        )
        .expect("serialize provenance");

        assert!(
            serialized.contains(
                "zamani.zhal.provenance.v1",
            ),
        );

        assert!(
            serialized.contains(
                "evidence.classical.compute",
            ),
        );

        assert!(
            serialized.contains(
                "resource.memory.snapshot",
            ),
        );
    }

    // ------------------------------------------------------------------------
    // Domain portability and policy
    // ------------------------------------------------------------------------

    #[test]
    fn supports_multiple_computational_domains() {
        for namespace in [
            "classical.hardware",
            "quantum.hardware",
            "hdl.realization",
            "accelerator.hardware",
            "distributed.execution",
            "embedded.hardware",
            "hybrid.execution",
            "future.photonic.hardware",
            "future.neuromorphic.hardware",
        ] {
            assert!(
                ProvenanceReference::new(
                    namespace,
                    "evidence.example",
                )
                .is_ok(),
                "rejected domain {namespace}",
            );
        }
    }

    #[test]
    fn component_policies_are_available() {
        assert_eq!(
            safety_policy(),
            "unsafe-rust-forbidden",
        );

        assert!(
            scalability_policy().contains("open-world"),
        );
    }

    #[test]
    fn provenance_does_not_generate_timestamps() {
        let provenance = HardwareProvenance::new(
            provider(),
        );

        let serialized = serde_json::to_string(
            &provenance,
        )
        .expect("serialize provenance");

        assert!(
            !serialized.contains("timestamp"),
        );
    }
}

// ============================================================================
// END
// ============================================================================