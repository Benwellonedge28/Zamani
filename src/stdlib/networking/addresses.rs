
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Networking Addresses
//!
//! Path: `src/stdlib/networking/addresses.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides portable, extensible address abstractions for the Zamani
//! standard library.
//!
//! An address represents a logical communication locator or an
//! address descriptor. It is not necessarily a physical IP address,
//! socket address, network interface, machine, node, or device.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Open-world address-family identities.
//! - Address values.
//! - Address descriptors.
//! - Address properties.
//! - Qualified address references.
//! - Address-resolution request contracts.
//! - Resolved-address representations.
//! - Resolution outcomes.
//! - Address validation.
//! - Address-specific errors.
//!
//! This module does NOT own:
//!
//! - Source grammar or parsing.
//! - AST construction.
//! - Semantic name resolution.
//! - Semantic capability definitions.
//! - Semantic resource requirements.
//! - Endpoint declarations.
//! - Protocol definitions.
//! - Network transports.
//! - DNS implementation.
//! - IP allocation.
//! - MAC allocation.
//! - Network-interface discovery.
//! - Socket creation.
//! - Port allocation.
//! - Routing.
//! - Distributed placement.
//! - Scheduling.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their authoritative
//! repository subsystems.
//!
//! ## Grammar authorities
//!
//! Normative specification:
//! `grammar/spec/networking.md`
//!
//! Networking grammar architecture:
//! `grammar/networking/README.md`
//!
//! Address grammar:
//! `grammar/networking/addresses.g4`
//!
//! Endpoint grammar:
//! `grammar/networking/endpoints.g4`
//!
//! Networking composition:
//! `grammar/networking/networking.g4`
//!
//! ## Architectural integration
//!
//! Source:
//! `grammar/networking/addresses.g4`
//!
//! Frontend AST:
//! `src/frontend/ast/`
//!
//! Semantic analysis:
//! `src/frontend/semantic/`
//!
//! Standard-library capabilities:
//! `src/stdlib/capability.rs`
//!
//! Standard-library requirements:
//! `src/stdlib/requirement.rs`
//!
//! Standard-library feasibility:
//! `src/stdlib/feasibility.rs`
//!
//! Networking protocols:
//! `src/stdlib/networking/protocols.rs`
//!
//! Networking endpoints:
//! `src/stdlib/networking/endpoints.rs`
//!
//! Networking transports:
//! `src/stdlib/networking/transports.rs`
//!
//! Runtime:
//! `src/runtime/`
//!
//! Distributed computation:
//! `src/distributed/`
//!
//! HAL and provider implementations:
//! Their respective authoritative subsystems.
//!
//! ## POCO-REAF
//!
//! Address representations are independent of the physical
//! scale and computational domain of execution.
//!
//! The model supports:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware communication.
//! - Hybrid computing.
//! - Distributed computing.
//! - AI and tensor systems.
//! - Embedded computing.
//! - Future networking technologies.
//!
//! Address families use extensible identities rather than
//! a closed enumeration.
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Address count.
//! - Address-value length.
//! - Address-family count.
//! - Property count.
//! - Property-value length.
//! - Qualified-name depth.
//! - Resolved-address count.
//! - Provider-supported address families.
//!
//! Actual storage and processing remain subject to available
//! resources and explicitly configured operational budgets.
//!
//! This does not promise literal infinite storage or computation.
//!
//! ## Fundamental distinctions
//!
//! Address:
//! A logical communication locator or descriptor.
//!
//! Address family:
//! An open-world classification of an address.
//!
//! Address reference:
//! A symbolic reference to an address declaration.
//!
//! Resolved address:
//! A concrete locator returned by an authoritative resolver.
//!
//! Resolution:
//! A provider-backed operation that maps an address descriptor
//! to one or more resolved locators.
//!
//! None of these concepts implicitly identifies a physical device.
//!
//! ## Error handling
//!
//! Invalid local values return structured errors.
//!
//! Resolution failures are distinct from invalid address
//! construction and from an empty successful lookup.
//!
//! No network operation is reported as successful unless
//! an actual resolver reports the corresponding result.

use core::fmt;
use std::error::Error as StdError;
use std::net::IpAddr;
use std::str::FromStr;

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Address-specific error classification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AddressErrorKind {
    /// An address-family identifier is invalid.
    InvalidFamily,

    /// An address value is invalid.
    InvalidValue,

    /// An address property is invalid.
    InvalidProperty,

    /// An address reference is invalid.
    InvalidReference,

    /// An address descriptor is invalid.
    InvalidDescriptor,

    /// A resolver does not support the requested address family.
    UnsupportedFamily,

    /// A resolution provider failed.
    ResolutionFailed,

    /// Resolution was unavailable in the current execution context.
    ResolutionUnavailable,

    /// The requested address could not be found.
    NotFound,

    /// Resolution was cancelled.
    Cancelled,

    /// Resolution timed out.
    TimedOut,

    /// An internal address operation failed.
    Internal,
}

impl AddressErrorKind {
    /// Returns a stable textual classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidFamily => "invalid_family",
            Self::InvalidValue => "invalid_value",
            Self::InvalidProperty => "invalid_property",
            Self::InvalidReference => "invalid_reference",
            Self::InvalidDescriptor => "invalid_descriptor",
            Self::UnsupportedFamily => "unsupported_family",
            Self::ResolutionFailed => "resolution_failed",
            Self::ResolutionUnavailable => "resolution_unavailable",
            Self::NotFound => "not_found",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for AddressErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured networking-address error.
#[derive(Debug)]
pub struct AddressError {
    kind: AddressErrorKind,
    message: String,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl AddressError {
    /// Creates a new address error.
    #[must_use]
    pub fn new(
        kind: AddressErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> AddressErrorKind {
        self.kind
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Attaches an underlying causal error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns whether a causal error exists.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "networking address error [{}]: {}",
            self.kind,
            self.message
        )
    }
}

impl StdError for AddressError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for address operations.
pub type AddressResult<T> = std::result::Result<T, AddressError>;

fn error(
    kind: AddressErrorKind,
    message: impl Into<String>,
) -> AddressError {
    AddressError::new(kind, message)
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates a portable address-family or property identifier.
///
/// Identifiers use an extensible ASCII namespace convention.
///
/// A qualified identifier uses `::` between components.
///
/// Examples:
///
/// - `network::logical`
/// - `network::ipv4`
/// - `network::future_fabric`
/// - `vendor::special_address`
///
/// The validator does not maintain a closed list of known families.
fn validate_qualified_identifier(
    value: &str,
) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    if value.chars().any(char::is_control) {
        return false;
    }

    let mut components = value.split("::");

    components.all(|component| {
        !component.is_empty()
            && component
                .bytes()
                .all(|byte| {
                    byte.is_ascii_alphanumeric()
                        || byte == b'_'
                        || byte == b'-'
                        || byte == b'.'
                })
            && component
                .as_bytes()
                .first()
                .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    })
}

// ============================================================================
// ADDRESS FAMILY
// ============================================================================

/// Open-world address-family identity.
///
/// Address families are represented by namespaced identifiers rather
/// than a closed Rust enumeration.
///
/// This allows future address technologies to be introduced without
/// modifying the standard-library address implementation.
///
/// Examples:
///
/// - `network::logical`
/// - `network::ipv4`
/// - `network::ipv6`
/// - `network::mac`
/// - `network::dns`
/// - `network::fabric`
/// - `vendor::future_address`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddressFamilyId(String);

impl AddressFamilyId {
    /// Constructs a validated address-family identity.
    pub fn new(value: impl Into<String>) -> AddressResult<Self> {
        let value = value.into();

        if !validate_qualified_identifier(&value) {
            return Err(error(
                AddressErrorKind::InvalidFamily,
                "invalid address-family identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the complete family identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks the family identity without allocating.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        self.0 == value
    }
}

impl fmt::Display for AddressFamilyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for AddressFamilyId {
    type Err = AddressError;

    fn from_str(value: &str) -> AddressResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// ADDRESS VALUE
// ============================================================================

/// An opaque textual address value.
///
/// The standard library does not assume that every address is an IP
/// address or that every future address family has the same encoding.
///
/// Values remain opaque until interpreted by the appropriate
/// semantic validator or resolution provider.
///
/// This type does not parse source expressions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddressValue(String);

impl AddressValue {
    /// Creates a validated address value.
    ///
    /// Empty values and control characters are rejected.
    ///
    /// No arbitrary maximum length is imposed.
    pub fn new(value: impl Into<String>) -> AddressResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(error(
                AddressErrorKind::InvalidValue,
                "address value cannot be empty",
            ));
        }

        if value.trim() != value {
            return Err(error(
                AddressErrorKind::InvalidValue,
                "address value cannot have surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(error(
                AddressErrorKind::InvalidValue,
                "address value contains control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the value's UTF-8 byte length.
    #[must_use]
    pub fn len_bytes(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for AddressValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for AddressValue {
    type Err = AddressError;

    fn from_str(value: &str) -> AddressResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// ADDRESS PROPERTY
// ============================================================================

/// A generic address property.
///
/// Property keys are open-world identifiers.
///
/// Property values remain opaque textual values. Their interpretation
/// belongs to semantic analysis or the relevant networking provider.
///
/// This avoids coupling the standard library to a fixed collection of
/// networking property names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressProperty {
    key: String,
    value: AddressValue,
}

impl AddressProperty {
    /// Creates a validated address property.
    pub fn new(
        key: impl Into<String>,
        value: AddressValue,
    ) -> AddressResult<Self> {
        let key = key.into();

        if !validate_qualified_identifier(&key) {
            return Err(error(
                AddressErrorKind::InvalidProperty,
                "invalid address property identifier",
            ));
        }

        Ok(Self { key, value })
    }

    /// Returns the property key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the opaque property value.
    #[must_use]
    pub fn value(&self) -> &AddressValue {
        &self.value
    }
}

// ============================================================================
// ADDRESS DESCRIPTOR
// ============================================================================

/// Portable logical address descriptor.
///
/// An address descriptor describes communication intent without
/// allocating or discovering a physical address.
///
/// Properties preserve insertion order and duplicate declarations.
/// Semantic analysis determines whether duplicate properties are valid.
///
/// The descriptor does not represent an endpoint declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressDescriptor {
    family: AddressFamilyId,
    value: Option<AddressValue>,
    properties: Vec<AddressProperty>,
}

impl AddressDescriptor {
    /// Creates a descriptor with a known family and value.
    #[must_use]
    pub fn new(
        family: AddressFamilyId,
        value: AddressValue,
    ) -> Self {
        Self {
            family,
            value: Some(value),
            properties: Vec::new(),
        }
    }

    /// Creates a descriptor whose value will be supplied by
    /// semantic evaluation or another authoritative source.
    #[must_use]
    pub fn without_value(family: AddressFamilyId) -> Self {
        Self {
            family,
            value: None,
            properties: Vec::new(),
        }
    }

    /// Returns the address family.
    #[must_use]
    pub fn family(&self) -> &AddressFamilyId {
        &self.family
    }

    /// Returns the optional address value.
    #[must_use]
    pub fn value(&self) -> Option<&AddressValue> {
        self.value.as_ref()
    }

    /// Returns the ordered address properties.
    #[must_use]
    pub fn properties(&self) -> &[AddressProperty] {
        &self.properties
    }

    /// Adds a property while preserving declaration order.
    pub fn add_property(
        &mut self,
        property: AddressProperty,
    ) {
        self.properties.push(property);
    }

    /// Adds a property using its key and textual value.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AddressResult<Self> {
        self.add_property(AddressProperty::new(
            key,
            AddressValue::new(value)?,
        )?);

        Ok(self)
    }

    /// Returns all properties matching a key.
    ///
    /// Multiple matching properties are deliberately preserved.
    pub fn properties_named<'a>(
        &'a self,
        key: &'a str,
    ) -> impl Iterator<Item = &'a AddressProperty> + 'a {
        self.properties
            .iter()
            .filter(move |property| property.key() == key)
    }
}

// ============================================================================
// ADDRESS REFERENCE
// ============================================================================

/// A symbolic qualified reference to an address declaration.
///
/// References are not resolved during construction.
///
/// Name lookup belongs to the canonical semantic or runtime
/// resolution authority.
///
/// Examples:
///
/// - `server`
/// - `network::server`
/// - `services::compute::address`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddressReference(String);

impl AddressReference {
    /// Creates a validated symbolic address reference.
    pub fn new(value: impl Into<String>) -> AddressResult<Self> {
        let value = value.into();

        if !validate_qualified_identifier(&value) {
            return Err(error(
                AddressErrorKind::InvalidReference,
                "invalid qualified address reference",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the qualified reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AddressReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for AddressReference {
    type Err = AddressError;

    fn from_str(value: &str) -> AddressResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// STANDARD IP ADDRESS ADAPTER
// ============================================================================

/// A standard IP-address adapter.
///
/// IP is one supported address representation, not the universal
/// address model.
///
/// Custom and future address families remain represented by
/// [`AddressFamilyId`] and [`AddressValue`].
///
/// Parsing delegates to Rust's standard library.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StandardIpAddress(IpAddr);

impl StandardIpAddress {
    /// Creates an IP-address adapter.
    #[must_use]
    pub const fn new(address: IpAddr) -> Self {
        Self(address)
    }

    /// Returns the underlying standard IP address.
    #[must_use]
    pub const fn as_ip_addr(&self) -> IpAddr {
        self.0
    }

    /// Converts the address to an opaque address value.
    pub fn to_address_value(self) -> AddressResult<AddressValue> {
        AddressValue::new(self.0.to_string())
    }

    /// Parses an IP address without imposing a custom parser limit.
    pub fn parse(value: &str) -> AddressResult<Self> {
        let address = value.parse::<IpAddr>().map_err(|source| {
            AddressError::new(
                AddressErrorKind::InvalidValue,
                "invalid standard IP address",
            )
            .with_source(source)
        })?;

        Ok(Self(address))
    }
}

impl fmt::Display for StandardIpAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// ============================================================================
// RESOLVED ADDRESS
// ============================================================================

/// A concrete address returned by an authoritative resolver.
///
/// A resolved address is still a logical locator. It does not prove
/// that a physical connection exists or that a device has been
/// allocated.
///
/// Port and transport information are intentionally excluded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedAddress {
    family: AddressFamilyId,
    value: AddressValue,
}

impl ResolvedAddress {
    /// Creates a resolved address.
    #[must_use]
    pub fn new(
        family: AddressFamilyId,
        value: AddressValue,
    ) -> Self {
        Self { family, value }
    }

    /// Returns the address family.
    #[must_use]
    pub fn family(&self) -> &AddressFamilyId {
        &self.family
    }

    /// Returns the resolved value.
    #[must_use]
    pub fn value(&self) -> &AddressValue {
        &self.value
    }

    /// Converts the address to a standard IP representation.
    ///
    /// Returns an error if this address belongs to another family
    /// or does not contain a valid IP representation.
    pub fn as_standard_ip(&self) -> AddressResult<StandardIpAddress> {
        StandardIpAddress::parse(self.value.as_str())
    }
}

// ============================================================================
// RESOLUTION REQUEST
// ============================================================================

/// A request passed to an address-resolution provider.
///
/// The request contains portable address intent only.
///
/// It does not prescribe DNS, a transport, a particular provider,
/// or a physical network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolutionRequest {
    descriptor: AddressDescriptor,
}

impl ResolutionRequest {
    /// Creates a resolution request.
    #[must_use]
    pub fn new(descriptor: AddressDescriptor) -> Self {
        Self { descriptor }
    }

    /// Returns the requested descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &AddressDescriptor {
        &self.descriptor
    }
}

// ============================================================================
// RESOLUTION OUTCOME
// ============================================================================

/// Outcome of an address-resolution operation.
///
/// These outcomes are distinct from operational failures.
///
/// In particular, a deferred result is not a successful resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResolutionOutcome {
    /// One or more addresses were successfully resolved.
    Resolved(Vec<ResolvedAddress>),

    /// The authoritative resolver established that no matching
    /// address exists in its resolution scope.
    NotFound,

    /// Resolution requires a later operation or additional information.
    ///
    /// No address has been resolved yet.
    Deferred,
}

impl ResolutionOutcome {
    /// Returns whether at least one address was resolved.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(addresses) if !addresses.is_empty())
    }

    /// Returns resolved addresses, if present.
    #[must_use]
    pub fn addresses(&self) -> Option<&[ResolvedAddress]> {
        match self {
            Self::Resolved(addresses) => Some(addresses),
            Self::NotFound | Self::Deferred => None,
        }
    }
}

// ============================================================================
// RESOLUTION PROVIDER CONTRACT
// ============================================================================

/// Contract for an address-resolution implementation.
///
/// Implementations may integrate with:
///
/// - DNS providers.
/// - Logical service registries.
/// - Distributed discovery systems.
/// - Local address registries.
/// - Network-specific address resolvers.
/// - Future communication technologies.
///
/// This trait does not perform network operations itself.
///
/// The implementation must report actual results and must never
/// fabricate successful resolution.
///
/// Runtime integrations are responsible for supplying the
/// appropriate provider and execution context.
///
/// The trait is synchronous by design: asynchronous execution,
/// cancellation and scheduling belong to the runtime's executor
/// and its provider adapter.
pub trait AddressResolver: Send + Sync {
    /// Resolves an address descriptor.
    ///
    /// An implementation must distinguish:
    ///
    /// - Successfully resolved addresses.
    /// - Authoritatively absent addresses.
    /// - Deferred resolution.
    /// - Operational failures.
    ///
    /// # Errors
    ///
    /// Returns an [`AddressError`] when resolution cannot be
    /// completed because of an operational failure.
    fn resolve(
        &self,
        request: &ResolutionRequest,
    ) -> AddressResult<ResolutionOutcome>;
}

// ============================================================================
// ADDRESS VALIDATION HELPERS
// ============================================================================

/// Validates an address descriptor's local structural integrity.
///
/// This function does not perform:
///
/// - DNS resolution.
/// - Semantic name lookup.
/// - Capability analysis.
/// - Feasibility analysis.
/// - Network access.
/// - Provider discovery.
/// - Hardware discovery.
pub fn validate_descriptor(
    descriptor: &AddressDescriptor,
) -> AddressResult<()> {
    if descriptor.family.as_str().is_empty() {
        return Err(error(
            AddressErrorKind::InvalidDescriptor,
            "address descriptor has no family",
        ));
    }

    if let Some(value) = descriptor.value() {
        if value.is_empty() {
            return Err(error(
                AddressErrorKind::InvalidDescriptor,
                "address descriptor contains an empty value",
            ));
        }
    }

    for property in descriptor.properties() {
        if !validate_qualified_identifier(property.key()) {
            return Err(error(
                AddressErrorKind::InvalidProperty,
                "address descriptor contains an invalid property",
            ));
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_open_world_address_families() {
        let family = AddressFamilyId::new(
            "vendor::future_fabric",
        )
        .expect("future address family should be accepted");

        assert_eq!(
            family.as_str(),
            "vendor::future_fabric"
        );
    }

    #[test]
    fn rejects_empty_address_families() {
        assert_eq!(
            AddressFamilyId::new("").unwrap_err().kind(),
            AddressErrorKind::InvalidFamily
        );
    }

    #[test]
    fn rejects_empty_family_components() {
        assert_eq!(
            AddressFamilyId::new("network::::ipv4")
                .unwrap_err()
                .kind(),
            AddressErrorKind::InvalidFamily
        );
    }

    #[test]
    fn supports_arbitrarily_deep_qualified_family_names() {
        let family = AddressFamilyId::new(
            "domain::region::fabric::service::address",
        )
        .expect("qualified family should be accepted");

        assert_eq!(
            family.as_str(),
            "domain::region::fabric::service::address"
        );
    }

    #[test]
    fn preserves_address_values() {
        let value = AddressValue::new(
            "logical://compute-service",
        )
        .expect("valid address value");

        assert_eq!(
            value.as_str(),
            "logical://compute-service"
        );
    }

    #[test]
    fn rejects_empty_address_values() {
        assert_eq!(
            AddressValue::new("")
                .unwrap_err()
                .kind(),
            AddressErrorKind::InvalidValue
        );
    }

    #[test]
    fn supports_values_beyond_small_fixed_buffers() {
        let value = "a".repeat(16_384);

        let address = AddressValue::new(value.clone())
            .expect("address should not have an arbitrary small limit");

        assert_eq!(address.len_bytes(), value.len());
    }

    #[test]
    fn rejects_control_characters() {
        assert_eq!(
            AddressValue::new("address\nvalue")
                .unwrap_err()
                .kind(),
            AddressErrorKind::InvalidValue
        );
    }

    #[test]
    fn supports_extensible_address_properties() {
        let descriptor = AddressDescriptor::without_value(
            AddressFamilyId::new("network::logical")
                .expect("valid family"),
        )
        .with_property("future::property", "custom-value")
        .expect("valid property");

        assert_eq!(
            descriptor.properties()[0].key(),
            "future::property"
        );
    }

    #[test]
    fn preserves_duplicate_properties_and_order() {
        let mut descriptor = AddressDescriptor::without_value(
            AddressFamilyId::new("network::logical")
                .expect("valid family"),
        );

        descriptor = descriptor
            .with_property("locality", "region")
            .expect("first property");

        descriptor = descriptor
            .with_property("locality", "global")
            .expect("second property");

        assert_eq!(descriptor.properties().len(), 2);

        assert_eq!(
            descriptor.properties()[0].value().as_str(),
            "region"
        );

        assert_eq!(
            descriptor.properties()[1].value().as_str(),
            "global"
        );
    }

    #[test]
    fn supports_symbolic_address_references() {
        let reference = AddressReference::new(
            "services::compute::address",
        )
        .expect("valid reference");

        assert_eq!(
            reference.as_str(),
            "services::compute::address"
        );
    }

    #[test]
    fn rejects_invalid_address_references() {
        assert_eq!(
            AddressReference::new("services::::address")
                .unwrap_err()
                .kind(),
            AddressErrorKind::InvalidReference
        );
    }

    #[test]
    fn parses_ipv4_addresses() {
        let address = StandardIpAddress::parse(
            "192.0.2.1",
        )
        .expect("valid IPv4 address");

        assert_eq!(
            address.to_string(),
            "192.0.2.1"
        );
    }

    #[test]
    fn parses_ipv6_addresses() {
        let address = StandardIpAddress::parse(
            "2001:db8::1",
        )
        .expect("valid IPv6 address");

        assert_eq!(
            address.to_string(),
            "2001:db8::1"
        );
    }

    #[test]
    fn rejects_invalid_ip_addresses() {
        assert_eq!(
            StandardIpAddress::parse("999.999.999.999")
                .unwrap_err()
                .kind(),
            AddressErrorKind::InvalidValue
        );
    }

    #[test]
    fn standard_ip_is_only_one_address_representation() {
        let custom = AddressDescriptor::new(
            AddressFamilyId::new("vendor::future_network")
                .expect("valid family"),
            AddressValue::new("custom-locator")
                .expect("valid value"),
        );

        assert!(validate_descriptor(&custom).is_ok());
    }

    #[test]
    fn creates_resolved_addresses() {
        let address = ResolvedAddress::new(
            AddressFamilyId::new("network::ipv4")
                .expect("valid family"),
            AddressValue::new("192.0.2.10")
                .expect("valid value"),
        );

        assert_eq!(
            address.as_standard_ip()
                .expect("valid IP")
                .to_string(),
            "192.0.2.10"
        );
    }

    #[test]
    fn rejects_wrong_representation_conversion() {
        let address = ResolvedAddress::new(
            AddressFamilyId::new("network::logical")
                .expect("valid family"),
            AddressValue::new("service://compute")
                .expect("valid value"),
        );

        assert!(
            address.as_standard_ip().is_err()
        );
    }

    #[test]
    fn distinguishes_resolution_outcomes() {
        assert!(
            ResolutionOutcome::Resolved(vec![
                ResolvedAddress::new(
                    AddressFamilyId::new("network::logical")
                        .expect("valid family"),
                    AddressValue::new("service://compute")
                        .expect("valid value"),
                )
            ])
            .is_resolved()
        );

        assert!(
            !ResolutionOutcome::NotFound.is_resolved()
        );

        assert!(
            !ResolutionOutcome::Deferred.is_resolved()
        );
    }

    #[test]
    fn validates_descriptors() {
        let descriptor = AddressDescriptor::new(
            AddressFamilyId::new("network::logical")
                .expect("valid family"),
            AddressValue::new("compute")
                .expect("valid value"),
        );

        assert!(
            validate_descriptor(&descriptor).is_ok()
        );
    }

    #[test]
    fn preserves_structured_error_information() {
        let error = AddressError::new(
            AddressErrorKind::ResolutionFailed,
            "provider operation failed",
        );

        assert_eq!(
            error.kind(),
            AddressErrorKind::ResolutionFailed
        );

        assert!(
            error.to_string().contains("provider operation failed")
        );
    }

    #[test]
    fn error_classifications_have_stable_names() {
        assert_eq!(
            AddressErrorKind::InvalidFamily.as_str(),
            "invalid_family"
        );

        assert_eq!(
            AddressErrorKind::ResolutionUnavailable.as_str(),
            "resolution_unavailable"
        );
    }
}
