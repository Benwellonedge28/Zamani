
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Host Platform
//!
//! Path: `src/zur/platforms/host.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the platform-neutral representation of the host environment
//! in which one Zamani Universal Runtime (ZUR) instance operates.
//!
//! The host environment describes:
//!
//! - Host platform identity.
//! - Operating-system identity.
//! - Processor architecture identity.
//! - ABI information.
//! - Target-triple metadata.
//! - Process execution characteristics.
//! - Available host-system facilities.
//! - Available execution interfaces.
//! - Platform-provided descriptive metadata.
//!
//! The representation is open-world and extensible.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - HostPlatformId.
//! - HostPlatformDescriptor.
//! - HostProcessDescriptor.
//! - HostFacilityId.
//! - HostFacilityDescriptor.
//! - ExecutionInterfaceId.
//! - ExecutionInterfaceDescriptor.
//! - HostAvailability.
//! - HostAttributeKey.
//! - HostEnvironment.
//! - HostEnvironmentBuilder.
//!
//! ## This module does NOT own
//!
//! - Source-language semantics.
//! - AST or semantic models.
//! - Requirements.
//! - Capability definitions.
//! - Capability matching.
//! - Feasibility evaluation.
//! - Resource allocation or reservation.
//! - Physical hardware discovery.
//! - Device selection.
//! - CPU/GPU/QPU/FPGA selection.
//! - Backend registration.
//! - Compilation target selection.
//! - Compiler planning.
//! - Scheduling.
//! - Placement.
//! - Routing.
//! - HAL implementation.
//! - Operating-system system calls.
//! - Process creation or termination.
//! - Executable loading.
//! - Runtime execution.
//! - Provider registration.
//! - Authentication or authorization.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ## Important distinctions
//!
//! A host platform descriptor is not a backend target descriptor.
//!
//! A host facility declaration is not a runtime service registration.
//!
//! An available execution interface is not proof that an arbitrary
//! workload can execute through that interface.
//!
//! An observation is not a resource reservation.
//!
//! A host description does not establish feasibility.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Host platforms.
//! - Host facilities.
//! - Execution interfaces.
//! - Host attributes.
//! - Computational domains.
//! - Devices.
//! - Processors.
//! - Memory.
//! - Threads.
//! - Distributed participants.
//! - Identifier lengths.
//!
//! Collections grow according to available memory and actual
//! implementation/platform constraints.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Safety
//!
//! This module uses safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware discovery.
//! No fabricated platform availability.
//! No implicit hardware selection.
//!
//! ## Determinism
//!
//! Host facilities, execution interfaces and attributes use BTreeMap.
//!
//! Their ordering is deterministic.
//!
//! No iteration-order-dependent execution decisions are introduced.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - `zur/error.rs`
//! - `zur/config.rs`
//! - `zur/core/context.rs`
//! - `zur/contracts/provider.rs`
//! - `stdlib/capability.rs`
//! - `backend/target.rs`
//!
//! Downstream:
//!
//! - `zur/platforms/provider.rs`
//! - `zur/platforms/compatibility.rs`
//! - `zur/core/context.rs`
//! - `zur/execution/loader.rs`
//! - `zur/execution/dispatch.rs`
//! - `zur/domains/`
//!
//! This module supplies descriptive host information.
//! Platform adapters supply actual observations and implementations.
//!
//! It does not initialize or execute those adapters.

use core::fmt;

use std::collections::BTreeMap;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// =============================================================================
// COMPONENT IDENTIFICATION
// =============================================================================

const COMPONENT: &str = "zur.platforms.host";

// =============================================================================
// VALIDATION
// =============================================================================

/// Validates an open-world namespaced identifier.
///
/// Accepted characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
/// - Dots separating nonempty segments.
///
/// Identifiers are case-sensitive.
///
/// No arbitrary identifier-length or namespace-depth ceiling exists.
fn validate_identifier(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(host_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(host_error(
            format!(
                "{description} must not contain surrounding whitespace"
            ),
        ));
    }

    let mut segments = value.split('.');

    let first = segments.next().ok_or_else(|| {
        host_error(
            format!("{description} must not be empty"),
        )
    })?;

    let mut count = 0usize;

    for segment in std::iter::once(first).chain(segments) {
        if segment.is_empty() {
            return Err(host_error(
                format!(
                    "{description} contains an empty namespace segment"
                ),
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'_'
                || byte == b'-'
        }) {
            return Err(host_error(
                format!(
                    "{description} contains an invalid character"
                ),
            ));
        }

        if segment.starts_with('_')
            || segment.starts_with('-')
            || segment.ends_with('_')
            || segment.ends_with('-')
        {
            return Err(host_error(
                format!(
                    "{description} contains an invalid segment boundary"
                ),
            ));
        }

        count += 1;
    }

    if count < 2 {
        return Err(host_error(
            format!(
                "{description} must contain at least two namespace segments"
            ),
        ));
    }

    Ok(())
}

/// Validates descriptive platform metadata.
///
/// Empty values and control characters are rejected.
/// Unicode descriptions are permitted.
fn validate_description(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(host_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(host_error(
            format!(
                "{description} must not contain control characters"
            ),
        ));
    }

    Ok(())
}

/// Constructs a structured host-platform error.
fn host_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
}

// =============================================================================
// HOST PLATFORM IDENTITY
// =============================================================================

/// Open-world identity of a host platform.
///
/// This identifies a platform description, not a physical device.
///
/// Examples:
///
/// - `org.example.host`
/// - `org.example.embedded`
/// - `org.example.compute-node`
/// - `vendor.custom-platform`
///
/// These examples do not define a closed platform inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostPlatformId(String);

impl HostPlatformId {
    /// Constructs a validated host platform identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "host platform identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for HostPlatformId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// HOST PLATFORM DESCRIPTION
// =============================================================================

/// Describes the identity of the current execution platform.
///
/// Operating system, architecture, ABI and target triple are independent
/// fields.
///
/// An architecture name does not imply an operating system.
///
/// An operating system does not imply a particular ABI.
///
/// Missing information remains unknown instead of being guessed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostPlatformDescriptor {
    id: HostPlatformId,

    operating_system: Option<String>,

    operating_system_version: Option<String>,

    architecture: Option<String>,

    abi: Option<String>,

    abi_version: Option<String>,

    target_triple: Option<String>,

    properties: BTreeMap<String, String>,
}

impl HostPlatformDescriptor {
    /// Creates a host platform descriptor.
    ///
    /// No operating system, architecture, ABI or target triple
    /// is inferred.
    #[must_use]
    pub fn new(
        id: HostPlatformId,
    ) -> Self {
        Self {
            id,
            operating_system: None,
            operating_system_version: None,
            architecture: None,
            abi: None,
            abi_version: None,
            target_triple: None,
            properties: BTreeMap::new(),
        }
    }

    /// Constructs a descriptor from an identifier string.
    pub fn from_id(
        id: impl Into<String>,
    ) -> ZurResult<Self> {
        Ok(Self::new(
            HostPlatformId::new(id)?,
        ))
    }

    /// Sets the operating-system identity.
    ///
    /// The value is an open-world identifier.
    pub fn with_operating_system(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "operating-system identifier",
        )?;

        self.operating_system = Some(value);

        Ok(self)
    }

    /// Sets the operating-system version.
    ///
    /// The version is descriptive metadata, not a compatibility decision.
    pub fn with_operating_system_version(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_description(
            &value,
            "operating-system version",
        )?;

        self.operating_system_version = Some(value);

        Ok(self)
    }

    /// Sets the processor architecture identity.
    ///
    /// Architecture identifiers are open-world.
    ///
    /// No architecture is assumed by this module.
    pub fn with_architecture(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "host architecture identifier",
        )?;

        self.architecture = Some(value);

        Ok(self)
    }

    /// Sets the ABI identity.
    ///
    /// ABI semantics remain owned by the corresponding ABI authority.
    pub fn with_abi(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "host ABI identifier",
        )?;

        self.abi = Some(value);

        Ok(self)
    }

    /// Sets the ABI version.
    pub fn with_abi_version(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_description(
            &value,
            "host ABI version",
        )?;

        self.abi_version = Some(value);

        Ok(self)
    }

    /// Sets the target triple.
    ///
    /// Target triples are opaque because their interpretation
    /// depends on the associated target ecosystem.
    pub fn with_target_triple(
        mut self,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_description(
            &value,
            "host target triple",
        )?;

        if value.chars().any(char::is_whitespace) {
            return Err(host_error(
                "host target triple must not contain whitespace",
            ));
        }

        self.target_triple = Some(value);

        Ok(self)
    }

    /// Adds descriptive platform metadata.
    ///
    /// Metadata keys are open-world namespaced identifiers.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "host platform property key",
        )?;

        validate_description(
            &value,
            "host platform property value",
        )?;

        self.properties.insert(key, value);

        Ok(self)
    }

    /// Returns the platform identity.
    #[must_use]
    pub fn id(&self) -> &HostPlatformId {
        &self.id
    }

    /// Returns the operating-system identity.
    #[must_use]
    pub fn operating_system(&self) -> Option<&str> {
        self.operating_system.as_deref()
    }

    /// Returns the operating-system version.
    #[must_use]
    pub fn operating_system_version(&self) -> Option<&str> {
        self.operating_system_version.as_deref()
    }

    /// Returns the architecture identity.
    #[must_use]
    pub fn architecture(&self) -> Option<&str> {
        self.architecture.as_deref()
    }

    /// Returns the ABI identity.
    #[must_use]
    pub fn abi(&self) -> Option<&str> {
        self.abi.as_deref()
    }

    /// Returns the ABI version.
    #[must_use]
    pub fn abi_version(&self) -> Option<&str> {
        self.abi_version.as_deref()
    }

    /// Returns the target triple.
    #[must_use]
    pub fn target_triple(&self) -> Option<&str> {
        self.target_triple.as_deref()
    }

    /// Returns all descriptive properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }

    /// Returns a property by key.
    #[must_use]
    pub fn property(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.properties.get(key).map(String::as_str)
    }
}

// =============================================================================
// HOST PROCESS DESCRIPTION
// =============================================================================

/// Describes the execution model of the host process.
///
/// Process models are open-world identifiers.
///
/// Examples:
///
/// - `host.native-process`
/// - `host.freestanding`
/// - `host.sandboxed-process`
/// - `host.managed-process`
///
/// These are descriptive classifications, not execution guarantees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostProcessDescriptor {
    model: String,

    identity: Option<String>,

    properties: BTreeMap<String, String>,
}

impl HostProcessDescriptor {
    /// Constructs a process descriptor.
    pub fn new(
        model: impl Into<String>,
    ) -> ZurResult<Self> {
        let model = model.into();

        validate_identifier(
            &model,
            "host process model",
        )?;

        Ok(Self {
            model,
            identity: None,
            properties: BTreeMap::new(),
        })
    }

    /// Sets an opaque host process identity.
    ///
    /// The value is not interpreted as a PID or machine-specific
    /// integer. Its representation belongs to the supplying platform.
    pub fn with_identity(
        mut self,
        identity: impl Into<String>,
    ) -> ZurResult<Self> {
        let identity = identity.into();

        validate_description(
            &identity,
            "host process identity",
        )?;

        self.identity = Some(identity);

        Ok(self)
    }

    /// Adds descriptive process metadata.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "process property key",
        )?;

        validate_description(
            &value,
            "process property value",
        )?;

        self.properties.insert(key, value);

        Ok(self)
    }

    /// Returns the process model.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the opaque process identity.
    #[must_use]
    pub fn identity(&self) -> Option<&str> {
        self.identity.as_deref()
    }

    /// Returns process properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }
}

// =============================================================================
// HOST FACILITY IDENTITY
// =============================================================================

/// Open-world identity of a host-system facility.
///
/// Examples:
///
/// - `host.process.creation`
/// - `host.process.signals`
/// - `host.memory.virtual`
/// - `host.memory.shared`
/// - `host.filesystem.access`
/// - `host.threading.native`
/// - `host.time.monotonic`
///
/// These examples are not a closed facility inventory.
///
/// Actual facility semantics belong to their platform implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostFacilityId(String);

impl HostFacilityId {
    /// Constructs a validated host facility identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "host facility identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HostFacilityId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// EXECUTION INTERFACE IDENTITY
// =============================================================================

/// Open-world identity of a host execution interface.
///
/// Examples:
///
/// - `host.execution.native`
/// - `host.execution.wasm`
/// - `host.execution.bytecode`
/// - `host.execution.external`
/// - `host.execution.freestanding`
///
/// These are examples, not a supported-interface guarantee.
///
/// The actual implementation and compatibility contract belong
/// to the relevant platform or execution provider.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionInterfaceId(String);

impl ExecutionInterfaceId {
    /// Constructs a validated execution-interface identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "execution interface identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionInterfaceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// OBSERVED AVAILABILITY
// =============================================================================

/// Describes the reported availability of a host facility or interface.
///
/// This is observational metadata.
///
/// `Available` does not mean that a particular workload is feasible.
///
/// Availability can change after the observation.
///
/// Resource reservation and execution remain separate operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HostAvailability {
    /// The supplying platform reports that the facility is available.
    Available,

    /// The supplying platform reports that the facility is unavailable.
    Unavailable,

    /// Availability has not been established.
    Unknown,
}

// =============================================================================
// HOST FACILITY DESCRIPTOR
// =============================================================================

/// Describes one host-system facility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFacilityDescriptor {
    id: HostFacilityId,

    availability: HostAvailability,

    evidence_reference: Option<String>,

    properties: BTreeMap<String, String>,
}

impl HostFacilityDescriptor {
    /// Constructs a host facility descriptor.
    ///
    /// The availability value must come from the supplying platform
    /// implementation or another explicitly trusted source.
    #[must_use]
    pub fn new(
        id: HostFacilityId,
        availability: HostAvailability,
    ) -> Self {
        Self {
            id,
            availability,
            evidence_reference: None,
            properties: BTreeMap::new(),
        }
    }

    /// Adds an opaque reference to external availability evidence.
    ///
    /// This module does not authenticate or verify that evidence.
    pub fn with_evidence_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ZurResult<Self> {
        let reference = reference.into();

        validate_description(
            &reference,
            "host facility evidence reference",
        )?;

        self.evidence_reference = Some(reference);

        Ok(self)
    }

    /// Adds descriptive facility metadata.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "host facility property key",
        )?;

        validate_description(
            &value,
            "host facility property value",
        )?;

        self.properties.insert(key, value);

        Ok(self)
    }

    /// Returns the facility identity.
    #[must_use]
    pub fn id(&self) -> &HostFacilityId {
        &self.id
    }

    /// Returns the reported availability.
    #[must_use]
    pub const fn availability(&self) -> HostAvailability {
        self.availability
    }

    /// Returns the evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Returns descriptive properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }
}

// =============================================================================
// EXECUTION INTERFACE DESCRIPTOR
// =============================================================================

/// Describes one host execution interface.
///
/// The descriptor identifies an interface and reports its availability.
///
/// It does not implement executable loading, code generation,
/// ABI compatibility validation or program execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionInterfaceDescriptor {
    id: ExecutionInterfaceId,

    availability: HostAvailability,

    contract_reference: Option<String>,

    evidence_reference: Option<String>,

    properties: BTreeMap<String, String>,
}

impl ExecutionInterfaceDescriptor {
    /// Constructs an execution-interface descriptor.
    #[must_use]
    pub fn new(
        id: ExecutionInterfaceId,
        availability: HostAvailability,
    ) -> Self {
        Self {
            id,
            availability,
            contract_reference: None,
            evidence_reference: None,
            properties: BTreeMap::new(),
        }
    }

    /// Sets the external execution-interface contract reference.
    pub fn with_contract_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ZurResult<Self> {
        let reference = reference.into();

        validate_identifier(
            &reference,
            "execution interface contract reference",
        )?;

        self.contract_reference = Some(reference);

        Ok(self)
    }

    /// Sets the external availability evidence reference.
    pub fn with_evidence_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ZurResult<Self> {
        let reference = reference.into();

        validate_description(
            &reference,
            "execution interface evidence reference",
        )?;

        self.evidence_reference = Some(reference);

        Ok(self)
    }

    /// Adds descriptive execution-interface metadata.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "execution interface property key",
        )?;

        validate_description(
            &value,
            "execution interface property value",
        )?;

        self.properties.insert(key, value);

        Ok(self)
    }

    /// Returns the interface identity.
    #[must_use]
    pub fn id(&self) -> &ExecutionInterfaceId {
        &self.id
    }

    /// Returns the reported availability.
    #[must_use]
    pub const fn availability(&self) -> HostAvailability {
        self.availability
    }

    /// Returns the interface contract reference.
    #[must_use]
    pub fn contract_reference(&self) -> Option<&str> {
        self.contract_reference.as_deref()
    }

    /// Returns the availability evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Returns descriptive properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }
}

// =============================================================================
// HOST ATTRIBUTE KEY
// =============================================================================

/// Open-world identity of a host environment metadata attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostAttributeKey(String);

impl HostAttributeKey {
    /// Constructs a validated attribute key.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "host attribute key",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical attribute key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HostAttributeKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// HOST ENVIRONMENT
// =============================================================================

/// Immutable description of one host execution environment.
///
/// This structure is constructed from explicitly supplied information.
///
/// It does not discover the current operating system, inspect hardware,
/// enumerate devices, read environment variables or initialize services.
///
/// Platform implementations are responsible for obtaining trustworthy
/// host observations and constructing this description.
///
/// The environment contains:
///
/// - One host platform descriptor.
/// - Optional process information.
/// - Host-system facility descriptions.
/// - Host execution-interface descriptions.
/// - Extensible host metadata.
///
/// It is not a hardware inventory or a resource reservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostEnvironment {
    platform: HostPlatformDescriptor,

    process: Option<HostProcessDescriptor>,

    facilities: BTreeMap<
        HostFacilityId,
        HostFacilityDescriptor,
    >,

    execution_interfaces: BTreeMap<
        ExecutionInterfaceId,
        ExecutionInterfaceDescriptor,
    >,

    attributes: BTreeMap<
        HostAttributeKey,
        String,
    >,
}

impl HostEnvironment {
    /// Creates a builder for a host environment.
    #[must_use]
    pub fn builder(
        platform: HostPlatformDescriptor,
    ) -> HostEnvironmentBuilder {
        HostEnvironmentBuilder::new(platform)
    }

    /// Returns the host platform descriptor.
    #[must_use]
    pub fn platform(&self) -> &HostPlatformDescriptor {
        &self.platform
    }

    /// Returns the optional process descriptor.
    #[must_use]
    pub fn process(&self) -> Option<&HostProcessDescriptor> {
        self.process.as_ref()
    }

    /// Returns all host facility descriptors.
    #[must_use]
    pub fn facilities(
        &self,
    ) -> &BTreeMap<HostFacilityId, HostFacilityDescriptor> {
        &self.facilities
    }

    /// Returns a host facility by identity.
    #[must_use]
    pub fn facility(
        &self,
        id: &HostFacilityId,
    ) -> Option<&HostFacilityDescriptor> {
        self.facilities.get(id)
    }

    /// Returns all execution interfaces.
    #[must_use]
    pub fn execution_interfaces(
        &self,
    ) -> &BTreeMap<
        ExecutionInterfaceId,
        ExecutionInterfaceDescriptor,
    > {
        &self.execution_interfaces
    }

    /// Returns an execution interface by identity.
    #[must_use]
    pub fn execution_interface(
        &self,
        id: &ExecutionInterfaceId,
    ) -> Option<&ExecutionInterfaceDescriptor> {
        self.execution_interfaces.get(id)
    }

    /// Returns all host attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<HostAttributeKey, String> {
        &self.attributes
    }

    /// Returns a host attribute by identity.
    #[must_use]
    pub fn attribute(
        &self,
        key: &HostAttributeKey,
    ) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    /// Returns whether the environment reports a facility as available.
    ///
    /// An absent or unknown facility returns false.
    ///
    /// This does not establish workload feasibility.
    #[must_use]
    pub fn reports_facility_available(
        &self,
        id: &HostFacilityId,
    ) -> bool {
        self.facility(id)
            .is_some_and(|facility| {
                facility.availability()
                    == HostAvailability::Available
            })
    }

    /// Returns whether the environment reports an execution interface
    /// as available.
    ///
    /// An absent or unknown interface returns false.
    ///
    /// This does not establish artifact compatibility or execution success.
    #[must_use]
    pub fn reports_interface_available(
        &self,
        id: &ExecutionInterfaceId,
    ) -> bool {
        self.execution_interface(id)
            .is_some_and(|interface| {
                interface.availability()
                    == HostAvailability::Available
            })
    }

    /// Returns the number of registered host facilities.
    ///
    /// This is an actual collection count, not an architectural ceiling.
    #[must_use]
    pub fn facility_count(&self) -> usize {
        self.facilities.len()
    }

    /// Returns the number of registered execution interfaces.
    #[must_use]
    pub fn execution_interface_count(&self) -> usize {
        self.execution_interfaces.len()
    }

    /// Returns the number of metadata attributes.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }
}

// =============================================================================
// HOST ENVIRONMENT BUILDER
// =============================================================================

/// Builder for an immutable host environment description.
///
/// Construction is explicit and deterministic.
///
/// Duplicate facility identities, execution-interface identities
/// and metadata keys are rejected.
///
/// No hardware discovery or system calls occur during construction.
pub struct HostEnvironmentBuilder {
    platform: HostPlatformDescriptor,

    process: Option<HostProcessDescriptor>,

    facilities: BTreeMap<
        HostFacilityId,
        HostFacilityDescriptor,
    >,

    execution_interfaces: BTreeMap<
        ExecutionInterfaceId,
        ExecutionInterfaceDescriptor,
    >,

    attributes: BTreeMap<
        HostAttributeKey,
        String,
    >,
}

impl HostEnvironmentBuilder {
    /// Creates a builder for the supplied platform.
    #[must_use]
    pub fn new(
        platform: HostPlatformDescriptor,
    ) -> Self {
        Self {
            platform,
            process: None,
            facilities: BTreeMap::new(),
            execution_interfaces: BTreeMap::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the process descriptor.
    ///
    /// Replacing process metadata is explicit.
    #[must_use]
    pub fn with_process(
        mut self,
        process: HostProcessDescriptor,
    ) -> Self {
        self.process = Some(process);
        self
    }

    /// Registers a host facility descriptor.
    ///
    /// Duplicate identities are rejected rather than silently overwritten.
    pub fn with_facility(
        mut self,
        facility: HostFacilityDescriptor,
    ) -> ZurResult<Self> {
        let id = facility.id().clone();

        if self.facilities.contains_key(&id) {
            return Err(host_error(
                format!(
                    "duplicate host facility identity `{id}`"
                ),
            ));
        }

        self.facilities.insert(id, facility);

        Ok(self)
    }

    /// Registers an execution-interface descriptor.
    ///
    /// Duplicate identities are rejected.
    pub fn with_execution_interface(
        mut self,
        interface: ExecutionInterfaceDescriptor,
    ) -> ZurResult<Self> {
        let id = interface.id().clone();

        if self.execution_interfaces.contains_key(&id) {
            return Err(host_error(
                format!(
                    "duplicate execution interface identity `{id}`"
                ),
            ));
        }

        self.execution_interfaces.insert(id, interface);

        Ok(self)
    }

    /// Registers a host metadata attribute.
    ///
    /// Duplicate keys are rejected.
    pub fn with_attribute(
        mut self,
        key: HostAttributeKey,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_description(
            &value,
            "host environment attribute",
        )?;

        if self.attributes.contains_key(&key) {
            return Err(host_error(
                format!(
                    "duplicate host attribute key `{key}`"
                ),
            ));
        }

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Builds the immutable host environment.
    ///
    /// This operation does not establish hardware availability,
    /// resource feasibility, compatibility or authorization.
    pub fn build(self) -> ZurResult<HostEnvironment> {
        Ok(HostEnvironment {
            platform: self.platform,
            process: self.process,
            facilities: self.facilities,
            execution_interfaces: self.execution_interfaces,
            attributes: self.attributes,
        })
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn platform() -> HostPlatformDescriptor {
        HostPlatformDescriptor::from_id(
            "org.example.host",
        )
        .expect("valid platform")
    }

    #[test]
    fn creates_open_world_platform_identity() {
        let id = HostPlatformId::new(
            "vendor.future-platform",
        )
        .expect("valid identity");

        assert_eq!(
            id.as_str(),
            "vendor.future-platform"
        );
    }

    #[test]
    fn rejects_invalid_platform_identity() {
        assert!(
            HostPlatformId::new("").is_err()
        );

        assert!(
            HostPlatformId::new("invalid").is_err()
        );

        assert!(
            HostPlatformId::new("vendor..platform").is_err()
        );
    }

    #[test]
    fn preserves_unknown_platform_information() {
        let descriptor = platform();

        assert_eq!(
            descriptor.operating_system(),
            None
        );

        assert_eq!(
            descriptor.architecture(),
            None
        );

        assert_eq!(
            descriptor.abi(),
            None
        );
    }

    #[test]
    fn records_independent_platform_properties() {
        let descriptor = platform()
            .with_operating_system("linux")
            .expect("OS")
            .with_operating_system_version("example-version")
            .expect("OS version")
            .with_architecture("aarch64")
            .expect("architecture")
            .with_abi("gnu")
            .expect("ABI")
            .with_abi_version("example-abi-version")
            .expect("ABI version")
            .with_target_triple(
                "aarch64-unknown-linux-gnu",
            )
            .expect("target triple");

        assert_eq!(
            descriptor.operating_system(),
            Some("linux")
        );

        assert_eq!(
            descriptor.architecture(),
            Some("aarch64")
        );

        assert_eq!(
            descriptor.abi(),
            Some("gnu")
        );

        assert_eq!(
            descriptor.target_triple(),
            Some("aarch64-unknown-linux-gnu")
        );
    }

    #[test]
    fn creates_process_descriptor_without_os_assumptions() {
        let process = HostProcessDescriptor::new(
            "host.freestanding",
        )
        .expect("process model");

        assert_eq!(
            process.model(),
            "host.freestanding"
        );

        assert_eq!(
            process.identity(),
            None
        );
    }

    #[test]
    fn accepts_extensible_host_facilities() {
        let id = HostFacilityId::new(
            "host.memory.virtual",
        )
        .expect("facility identity");

        let facility = HostFacilityDescriptor::new(
            id,
            HostAvailability::Available,
        );

        let environment = HostEnvironment::builder(platform())
            .with_facility(facility)
            .expect("facility registration")
            .build()
            .expect("environment");

        assert_eq!(
            environment.facility_count(),
            1
        );
    }

    #[test]
    fn accepts_future_execution_interfaces() {
        let id = ExecutionInterfaceId::new(
            "future.photonics.execution",
        )
        .expect("interface identity");

        let interface = ExecutionInterfaceDescriptor::new(
            id,
            HostAvailability::Available,
        );

        let environment = HostEnvironment::builder(platform())
            .with_execution_interface(interface)
            .expect("interface registration")
            .build()
            .expect("environment");

        assert_eq!(
            environment.execution_interface_count(),
            1
        );
    }

    #[test]
    fn unknown_availability_is_not_treated_as_available() {
        let id = HostFacilityId::new(
            "host.process.creation",
        )
        .expect("facility identity");

        let environment = HostEnvironment::builder(platform())
            .with_facility(
                HostFacilityDescriptor::new(
                    id.clone(),
                    HostAvailability::Unknown,
                ),
            )
            .expect("registration")
            .build()
            .expect("environment");

        assert!(
            !environment.reports_facility_available(&id)
        );
    }

    #[test]
    fn rejects_duplicate_facilities() {
        let id = HostFacilityId::new(
            "host.memory.virtual",
        )
        .expect("identity");

        let first = HostFacilityDescriptor::new(
            id.clone(),
            HostAvailability::Available,
        );

        let second = HostFacilityDescriptor::new(
            id,
            HostAvailability::Unavailable,
        );

        let builder = HostEnvironment::builder(platform())
            .with_facility(first)
            .expect("first registration");

        assert!(
            builder.with_facility(second).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_execution_interfaces() {
        let id = ExecutionInterfaceId::new(
            "host.execution.native",
        )
        .expect("identity");

        let first = ExecutionInterfaceDescriptor::new(
            id.clone(),
            HostAvailability::Available,
        );

        let second = ExecutionInterfaceDescriptor::new(
            id,
            HostAvailability::Unavailable,
        );

        let builder = HostEnvironment::builder(platform())
            .with_execution_interface(first)
            .expect("first registration");

        assert!(
            builder.with_execution_interface(second).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_attributes() {
        let key = HostAttributeKey::new(
            "host.environment.region",
        )
        .expect("attribute key");

        let builder = HostEnvironment::builder(platform())
            .with_attribute(
                key.clone(),
                "region-a",
            )
            .expect("first attribute");

        assert!(
            builder.with_attribute(
                key,
                "region-b",
            ).is_err()
        );
    }

    #[test]
    fn preserves_extensible_attributes() {
        let key = HostAttributeKey::new(
            "vendor.platform.configuration",
        )
        .expect("key");

        let environment = HostEnvironment::builder(platform())
            .with_attribute(
                key.clone(),
                "custom-value",
            )
            .expect("attribute")
            .build()
            .expect("environment");

        assert_eq!(
            environment.attribute(&key),
            Some("custom-value")
        );
    }

    #[test]
    fn does_not_infer_execution_interface_availability() {
        let id = ExecutionInterfaceId::new(
            "host.execution.external",
        )
        .expect("identity");

        let environment = HostEnvironment::builder(platform())
            .build()
            .expect("environment");

        assert!(
            !environment.reports_interface_available(&id)
        );
    }

    #[test]
    fn supports_large_extensible_collections() {
        let mut builder = HostEnvironment::builder(platform());

        for index in 0..512 {
            let id = HostFacilityId::new(
                format!("extension.facility_{index}"),
            )
            .expect("facility identity");

            builder = builder
                .with_facility(
                    HostFacilityDescriptor::new(
                        id,
                        HostAvailability::Unknown,
                    ),
                )
                .expect("registration");
        }

        let environment = builder.build()
            .expect("environment");

        assert_eq!(
            environment.facility_count(),
            512
        );
    }

    #[test]
    fn supports_future_computing_domains() {
        let id = ExecutionInterfaceId::new(
            "future.photonic.execution",
        )
        .expect("future interface");

        let interface = ExecutionInterfaceDescriptor::new(
            id,
            HostAvailability::Unknown,
        );

        let environment = HostEnvironment::builder(platform())
            .with_execution_interface(interface)
            .expect("registration")
            .build()
            .expect("environment");

        assert_eq!(
            environment.execution_interface_count(),
            1
        );
    }

    #[test]
    fn host_environment_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<HostEnvironment>();
    }
}
