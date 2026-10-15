
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Value Backend Contract
//!
//! File: src/zuir/values/backend.rs
//! Edition: Rust 2021
//! Minimum compiler: Rust 1.97+
//!
//! PURPOSE
//!
//! Define a target-neutral contract for determining whether a backend can
//! represent or lower a ZUIR value. Concrete target implementations belong
//! outside this module.
//!
//! OWNS
//!
//! - Backend identity and declared capabilities.
//! - Portable value descriptors used for support queries.
//! - Explicit support and rejection outcomes.
//! - Backend-independent lowering requests and results.
//! - A thread-safe backend interface.
//!
//! DOES_NOT_OWN
//!
//! - ZUIR's canonical value representation.
//! - Source parsing, grammar, semantic analysis, or type inference.
//! - Target code generation, execution, memory allocation, or scheduling.
//! - Physical device discovery or resource allocation.
//! - Backend registration or plugin loading.
//!
//! DEPENDENCIES
//!
//! Uses the shared errors in values/errors.rs and standard Rust types.
//! It intentionally does not depend on a specific device SDK.
//!
//! POCO-REAF
//!
//! A backend advertises what it supports. The caller selects a suitable
//! backend, validates its requirements, and invokes lowering explicitly.
//! Unsupported semantics are never silently approximated.
//!
//! SCALABILITY
//!
//! Collections are dynamically sized. No hardware counts, tensor ranks,
//! widths, or resource capacities are fixed here. Real resource limits
//! belong to backend capability reports and feasibility checks.
//!
//! INTEGRATION
//!
//! - values/errors.rs supplies the common value error contract.
//! - values/constants.rs, literals.rs, arrays.rs, tables.rs, and
//!   expressions.rs provide descriptors for the values they own.
//! - values/capability.rs supplies semantic capability information where
//!   appropriate; this module does not duplicate that model.
//! - lowering/ and dialect-specific modules implement actual conversions.
//! - The values module owner must declare `pub mod backend;`.
//!
//! FREEZE CRITERIA
//!
//! - The value descriptor maps losslessly to existing canonical value types.
//! - Backend implementations declare deterministic behavior and supported
//!   semantic features.
//! - Unsupported and resource-limited outcomes remain distinguishable.
//! - Conformance tests prove that backend support claims are truthful.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::errors::{ValueError, ValueErrorKind, ValueResult};

/// A stable, qualified backend identifier.
///
/// Use namespaced identifiers such as `org.example.cpu` or
/// `org.example.quantum`. Vendor-specific identifiers must not become
/// universal ZUIR operation names.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BackendId(String);

impl BackendId {
    /// Creates an identifier after validating its basic syntax.
    ///
    /// This validates the identifier format only. Registration and trust
    /// decisions belong to the backend registry.
    pub fn new(value: impl Into<String>) -> ValueResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_whitespace)
            || !value.contains('.')
            || value.starts_with('.')
            || value.ends_with('.')
            || value.split('.').any(str::is_empty)
        {
            return Err(ValueError::new(
                ValueErrorKind::InvalidExtensionIdentity,
                "backend identifier must be a non-empty qualified name",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BackendId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Backend implementation version.
///
/// Version interpretation and compatibility policy are backend-specific;
/// this structure does not infer compatibility from a version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BackendVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl BackendVersion {
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }
}

/// Semantic execution or representation domain.
///
/// This is a classification for negotiation, not a closed list of all
/// computational domains. New domains are identified by qualified names
/// in extension descriptors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendDomain {
    Classical,
    Quantum,
    Hdl,
    Tensor,
    Distributed,
    Networking,
    Cryptography,
    Other(String),
}

impl BackendDomain {
    /// Returns the stable domain name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Classical => "classical",
            Self::Quantum => "quantum",
            Self::Hdl => "hdl",
            Self::Tensor => "tensor",
            Self::Distributed => "distributed",
            Self::Networking => "networking",
            Self::Cryptography => "cryptography",
            Self::Other(name) => name,
        }
    }
}

/// A named capability or semantic feature.
///
/// Capabilities are namespaced and extensible. Their meaning must be
/// documented by the defining backend or extension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> ValueResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_whitespace)
            || !value.contains('.')
            || value.starts_with('.')
            || value.ends_with('.')
            || value.split('.').any(str::is_empty)
        {
            return Err(ValueError::new(
                ValueErrorKind::InvalidExtensionIdentity,
                "capability identifier must be a qualified name",
            ));
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Describes a value kind without binding the universal IR to a device.
///
/// The descriptor is a query abstraction, not a replacement for the
/// canonical type/value model. Callers must derive it from validated ZUIR
/// values, never from untrusted source text alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueDescriptor {
    /// Qualified semantic kind, for example `core.integer` or
    /// `tensor.array`. This is not a backend instruction name.
    pub kind: String,

    /// Qualified semantic type, if known.
    pub type_name: Option<String>,

    /// Required semantic capabilities.
    pub required_capabilities: Vec<CapabilityId>,

    /// Optional semantic attributes in canonical key order.
    ///
    /// Values are strings for negotiation only. Actual value data remains
    /// in its owning ZUIR representation.
    pub attributes: BTreeMap<String, String>,
}

impl ValueDescriptor {
    pub fn new(kind: impl Into<String>) -> ValueResult<Self> {
        let kind = kind.into();

        if kind.is_empty() || kind.trim() != kind {
            return Err(ValueError::new(
                ValueErrorKind::InvalidValueReference,
                "value kind must not be empty or contain surrounding whitespace",
            ));
        }

        Ok(Self {
            kind,
            type_name: None,
            required_capabilities: Vec::new(),
            attributes: BTreeMap::new(),
        })
    }

    #[must_use]
    pub fn with_type(mut self, type_name: impl Into<String>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    #[must_use]
    pub fn requiring(mut self, capability: CapabilityId) -> Self {
        self.required_capabilities.push(capability);
        self
    }

    #[must_use]
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Backend support level.
///
/// `Supported` means the backend claims it can preserve the requested
/// semantics under the reported constraints. It does not mean execution
/// resources have already been allocated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportLevel {
    /// Supported without a semantic conversion.
    Native,

    /// Supported through a documented semantics-preserving lowering.
    Lowerable,

    /// Support depends on additional requirements.
    Conditional,

    /// Not supported by this backend.
    Unsupported,
}

/// Why a backend cannot currently accept a value.
///
/// These outcomes are not interchangeable: a missing capability may be
/// resolved by selecting another backend, whereas invalid input must be
/// corrected before lowering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedReason {
    UnknownValueKind,
    MissingCapability(CapabilityId),
    UnsupportedType,
    UnsupportedSemantics,
    UnsupportedConversion,
    ResourceRequirementNotMet,
    BackendUnavailable,
    BackendVersionIncompatible,
    ExtensionNotTrusted,
    Other(String),
}

/// Result of asking whether a backend supports a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportReport {
    pub backend: BackendId,
    pub level: SupportLevel,
    pub reasons: Vec<UnsupportedReason>,
    pub requirements: Vec<ResourceRequirement>,
    pub explanation: Option<String>,
}

impl SupportReport {
    #[must_use]
    pub fn is_supported(&self) -> bool {
        matches!(self.level, SupportLevel::Native | SupportLevel::Lowerable)
            && self.reasons.is_empty()
    }

    #[must_use]
    pub fn requires_lowering(&self) -> bool {
        self.level == SupportLevel::Lowerable
    }
}

/// A symbolic resource requirement.
///
/// The backend interprets the dimension and units according to its
/// capability contract. Values are represented as strings so requirements
/// are not restricted to a machine integer or floating-point range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub dimension: String,
    pub relation: RequirementRelation,
    pub quantity: String,
    pub unit: Option<String>,
}

impl ResourceRequirement {
    pub fn new(
        dimension: impl Into<String>,
        relation: RequirementRelation,
        quantity: impl Into<String>,
    ) -> ValueResult<Self> {
        let dimension = dimension.into();
        let quantity = quantity.into();

        if dimension.is_empty()
            || dimension.trim() != dimension
            || quantity.is_empty()
            || quantity.trim() != quantity
        {
            return Err(ValueError::new(
                ValueErrorKind::InvalidAggregate,
                "resource dimension and quantity must be non-empty",
            ));
        }

        Ok(Self {
            dimension,
            relation,
            quantity,
            unit: None,
        })
    }

    #[must_use]
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

/// Relationship used by a symbolic resource requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementRelation {
    AtLeast,
    AtMost,
    Exactly,
    Available,
    Preferred,
}

/// A declared backend capability profile.
///
/// This is descriptive data. A backend must not claim a capability unless
/// its implementation and conformance tests justify that claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCapabilities {
    pub domains: Vec<BackendDomain>,
    pub capabilities: Vec<CapabilityId>,
    pub supported_value_kinds: Vec<String>,
    pub deterministic_lowering: bool,
    pub preserves_source_provenance: bool,
    pub supports_symbolic_requirements: bool,
}

impl BackendCapabilities {
    #[must_use]
    pub fn supports_capability(&self, capability: &CapabilityId) -> bool {
        self.capabilities.contains(capability)
    }

    #[must_use]
    pub fn supports_domain(&self, domain: &BackendDomain) -> bool {
        self.domains.contains(domain)
    }

    #[must_use]
    pub fn supports_value_kind(&self, kind: &str) -> bool {
        self.supported_value_kinds.iter().any(|item| item == kind)
    }
}

/// Context supplied to a backend for a lowering request.
///
/// Resource budgets and target selection policies are supplied by the
/// compiler or caller. They are not hard-coded by this module.
#[derive(Debug, Clone, Default)]
pub struct BackendContext {
    pub required_capabilities: Vec<CapabilityId>,
    pub resource_requirements: Vec<ResourceRequirement>,
    pub deterministic_required: bool,
    pub provenance_required: bool,
}

/// A backend-specific, semantics-preserving lowering request.
///
/// The payload remains opaque to the universal interface. The adapter
/// producing it must retain a lossless association with the original ZUIR
/// value and type information.
#[derive(Debug, Clone)]
pub struct LoweringRequest {
    pub value: ValueDescriptor,
    pub context: BackendContext,
}

/// Backend-specific lowered representation.
///
/// The universal interface treats the representation as opaque. A
/// concrete backend should use a typed payload internally and expose
/// serialization only through an explicitly versioned format.
#[derive(Debug, Clone)]
pub struct LoweredValue {
    pub backend: BackendId,
    pub representation_kind: String,
    pub representation: Arc<[u8]>,
    pub semantic_fingerprint: Option<String>,
}

/// Errors returned by a backend implementation.
///
/// These failures are kept separate from value representation errors:
/// a well-formed value can still be unsupported by a particular backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendError {
    pub backend: BackendId,
    pub kind: BackendErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendErrorKind {
    Unsupported,
    InvalidRequest,
    IncompatibleVersion,
    MissingCapability,
    ResourceUnavailable,
    LoweringFailed,
    SemanticPreservationUnproven,
    InternalFailure,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "backend {} failed ({:?}): {}",
            self.backend, self.kind, self.message
        )
    }
}

impl Error for BackendError {}

/// Thread-safe interface implemented by a concrete value backend.
///
/// Implementations must be deterministic when they declare deterministic
/// lowering, must not silently discard semantic information, and must
/// return an explicit error when they cannot preserve the required meaning.
pub trait ValueBackend: Send + Sync {
    /// Stable backend identity.
    fn id(&self) -> &BackendId;

    /// Implementation version.
    fn version(&self) -> BackendVersion;

    /// Declared capabilities.
    fn capabilities(&self) -> &BackendCapabilities;

    /// Queries support without executing the value.
    fn query_support(
        &self,
        request: &LoweringRequest,
    ) -> Result<SupportReport, BackendError>;

    /// Lowers a value after support and policy checks.
    ///
    /// Implementations must validate the request again where necessary;
    /// a previous support query is not an authorization or allocation
    /// guarantee.
    fn lower(
        &self,
        request: &LoweringRequest,
    ) -> Result<LoweredValue, BackendError>;
}

/// Shared ownership of a backend implementation.
pub type SharedValueBackend = Arc<dyn ValueBackend>;

/// Validates common negotiation requirements before invoking a backend.
///
/// This is a policy pre-check, not a substitute for the backend's own
/// support query or semantic verification.
pub fn validate_request(
    capabilities: &BackendCapabilities,
    request: &LoweringRequest,
) -> ValueResult<()> {
    for required in &request.context.required_capabilities {
        if !capabilities.supports_capability(required) {
            return Err(ValueError::new(
                ValueErrorKind::UnsupportedValueKind,
                format!("required backend capability is unavailable: {}", required.as_str()),
            ));
        }
    }

    for required in &request.value.required_capabilities {
        if !capabilities.supports_capability(required) {
            return Err(ValueError::new(
                ValueErrorKind::UnsupportedValueKind,
                format!("value requires unavailable capability: {}", required.as_str()),
            ));
        }
    }

    if request.context.deterministic_required
        && !capabilities.deterministic_lowering
    {
        return Err(ValueError::new(
            ValueErrorKind::UnsupportedValueKind,
            "request requires deterministic lowering",
        ));
    }

    if request.context.provenance_required
        && !capabilities.preserves_source_provenance
    {
        return Err(ValueError::new(
            ValueErrorKind::UnsupportedValueKind,
            "request requires source provenance preservation",
        ));
    }

    if !capabilities.supports_value_kind(&request.value.kind) {
        return Err(ValueError::new(
            ValueErrorKind::UnsupportedValueKind,
            format!("backend does not declare support for {}", request.value.kind),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_capabilities() -> BackendCapabilities {
        BackendCapabilities {
            domains: vec![BackendDomain::Classical],
            capabilities: vec![
                CapabilityId::new("core.integer").expect("valid capability"),
            ],
            supported_value_kinds: vec!["core.integer".to_owned()],
            deterministic_lowering: true,
            preserves_source_provenance: true,
            supports_symbolic_requirements: true,
        }
    }

    fn sample_request() -> LoweringRequest {
        LoweringRequest {
            value: ValueDescriptor::new("core.integer")
                .expect("valid descriptor")
                .requiring(
                    CapabilityId::new("core.integer")
                        .expect("valid capability"),
                ),
            context: BackendContext {
                deterministic_required: true,
                provenance_required: true,
                ..BackendContext::default()
            },
        }
    }

    #[test]
    fn backend_ids_must_be_qualified() {
        assert!(BackendId::new("example.cpu").is_ok());
        assert!(BackendId::new("cpu").is_err());
        assert!(BackendId::new("example..cpu").is_err());
    }

    #[test]
    fn capability_ids_must_be_qualified() {
        assert!(CapabilityId::new("quantum.measurement").is_ok());
        assert!(CapabilityId::new("measurement").is_err());
    }

    #[test]
    fn valid_request_passes_capability_checks() {
        assert!(validate_request(&sample_capabilities(), &sample_request()).is_ok());
    }

    #[test]
    fn unsupported_value_kind_is_rejected_explicitly() {
        let capabilities = sample_capabilities();
        let mut request = sample_request();
        request.value.kind = "future.special_value".to_owned();

        assert!(validate_request(&capabilities, &request).is_err());
    }

    #[test]
    fn missing_capability_is_not_silently_accepted() {
        let capabilities = sample_capabilities();
        let request = LoweringRequest {
            value: ValueDescriptor::new("core.integer")
                .expect("valid descriptor")
                .requiring(
                    CapabilityId::new("future.required_feature")
                        .expect("valid capability"),
                ),
            context: BackendContext::default(),
        };

        let error = validate_request(&capabilities, &request)
            .expect_err("missing capability must fail");

        assert_eq!(error.kind(), ValueErrorKind::UnsupportedValueKind);
    }

    #[test]
    fn resource_requirements_are_symbolic() {
        let requirement = ResourceRequirement::new(
            "quantum.qubits",
            RequirementRelation::AtLeast,
            "required_logical_qubits",
        )
        .expect("valid requirement");

        assert_eq!(requirement.dimension, "quantum.qubits");
        assert_eq!(requirement.quantity, "required_logical_qubits");
    }

    #[test]
    fn custom_domains_are_supported() {
        let domain = BackendDomain::Other("future.domain".to_owned());
        assert_eq!(domain.name(), "future.domain");
    }

    #[test]
    fn versions_do_not_imply_compatibility() {
        let first = BackendVersion::new(1, 0, 0);
        let second = BackendVersion::new(1, 1, 0);

        assert!(second > first);
        // Compatibility must be negotiated by a backend-specific contract.
    }
}
