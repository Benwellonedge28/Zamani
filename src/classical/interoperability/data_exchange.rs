
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Data Exchange Contracts
//!
//! Path: src/classical/interoperability/data_exchange.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! STATUS
//!   Proposed implementation; repository integration and compiler
//!   verification are still required.
//!
//! PURPOSE
//!   Define validated, declarative contracts for exchanging data across
//!   Zamani and external data boundaries.
//!
//! OWNS
//!   - Open-world identifiers for exchange formats, schemas, encodings,
//!     versions, integrity algorithms, and extension metadata.
//!   - Source and destination endpoint descriptions.
//!   - Transfer, ordering, integrity, and data-loss intent.
//!   - Local validation of exchange contracts.
//!
//! DOES NOT OWN
//!   - Serialization or deserialization algorithms.
//!   - Canonical serialized bytes, framing, hashing, or wire protocols.
//!   - ABI definitions or foreign-function declarations.
//!   - Canonical classical values, source types, ASTs, or IR identities.
//!   - Runtime transport, file access, network access, allocation,
//!     authentication, encryption, scheduling, or hardware selection.
//!
//! AUTHORITY AND INTEGRATION
//!   - grammar/spec/data.md defines the wider data semantics.
//!   - grammar/data/serialization.g4 owns serialization syntax.
//!   - grammar/interoperability/serialization.g4 is an interoperability
//!     facade, not a competing serialization grammar.
//!   - classical/interoperability/abi.rs owns ABI contracts.
//!   - quantum::ir::classical::extern_call owns canonical external-call IR.
//!   - classical/error.rs owns the shared classical error vocabulary.
//!
//! POCO-REAF
//!   No universal maximum is imposed on identifiers, metadata entries,
//!   schemas, endpoints, or exchanged data sizes. This module does not
//!   allocate payloads or claim that physical resources are infinite.
//!   Actual allocation, payload-size, time, and transport budgets belong
//!   to the relevant codec or runtime and must be explicit there.
//!
//! FREEZE RULE
//!   Changes to these public contract semantics require compatibility
//!   review and tests. This module describes intent; it does not execute it.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::classical::error::ClassicalError;

/// Result type for data-exchange contract operations.
pub type DataExchangeResult<T> = Result<T, DataExchangeError>;

/// Stable error categories for local data-exchange validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataExchangeErrorKind {
    /// An identifier is empty or contains a forbidden NUL character.
    InvalidIdentifier,

    /// A contract or endpoint is internally inconsistent.
    InvalidContract,

    /// Extension metadata contains an invalid key or value.
    InvalidMetadata,

    /// A version or compatibility requirement cannot be satisfied.
    IncompatibleVersion,
}

impl DataExchangeErrorKind {
    /// Stable machine-readable error code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "data_exchange.invalid_identifier",
            Self::InvalidContract => "data_exchange.invalid_contract",
            Self::InvalidMetadata => "data_exchange.invalid_metadata",
            Self::IncompatibleVersion => "data_exchange.incompatible_version",
        }
    }
}

/// A structured validation error.
///
/// Consumers should branch on `kind()` rather than parse display text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataExchangeError {
    kind: DataExchangeErrorKind,
    subject: String,
    message: String,
}

impl DataExchangeError {
    fn new(
        kind: DataExchangeErrorKind,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn kind(&self) -> DataExchangeErrorKind {
        self.kind
    }

    /// Returns the field or contract associated with the error.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns explanatory text. This is not a stable machine interface.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts this local error into the shared classical error vocabulary.
    #[must_use]
    pub fn into_classical_error(self) -> ClassicalError {
        ClassicalError::validation_failed(format!(
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        ))
    }
}

impl fmt::Display for DataExchangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        )
    }
}

impl Error for DataExchangeError {}

/// An open-world identifier used by an exchange contract.
///
/// Examples include a schema name, media type, encoding name, format
/// identifier, extension key, or externally defined version identifier.
/// This type intentionally does not enumerate current vendors or formats.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataExchangeIdentifier(String);

impl DataExchangeIdentifier {
    /// Creates an identifier after basic representation validation.
    ///
    /// The accepted character set remains open so future formats and
    /// providers do not require changes to a closed Rust enum.
    pub fn new(value: impl Into<String>) -> DataExchangeResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.contains('\0') {
            return Err(DataExchangeError::new(
                DataExchangeErrorKind::InvalidIdentifier,
                "identifier",
                "an identifier must be non-empty and must not contain NUL",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DataExchangeIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Describes one side of a data-exchange boundary.
///
/// These are symbolic references, not resolved schemas, codecs, native
/// layouts, physical addresses, or runtime handles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataExchangeEndpoint {
    schema: DataExchangeIdentifier,
    format: DataExchangeIdentifier,
    encoding: DataExchangeIdentifier,
    version: Option<DataExchangeIdentifier>,
}

impl DataExchangeEndpoint {
    /// Creates an endpoint from symbolic schema, format, and encoding
    /// identifiers, with an optional schema/format version.
    pub fn new(
        schema: impl Into<String>,
        format: impl Into<String>,
        encoding: impl Into<String>,
        version: Option<String>,
    ) -> DataExchangeResult<Self> {
        Ok(Self {
            schema: DataExchangeIdentifier::new(schema)?,
            format: DataExchangeIdentifier::new(format)?,
            encoding: DataExchangeIdentifier::new(encoding)?,
            version: version
                .map(DataExchangeIdentifier::new)
                .transpose()?,
        })
    }

    /// Returns the schema reference.
    #[must_use]
    pub fn schema(&self) -> &DataExchangeIdentifier {
        &self.schema
    }

    /// Returns the format identifier.
    #[must_use]
    pub fn format(&self) -> &DataExchangeIdentifier {
        &self.format
    }

    /// Returns the encoding identifier.
    #[must_use]
    pub fn encoding(&self) -> &DataExchangeIdentifier {
        &self.encoding
    }

    /// Returns the optional declared version.
    #[must_use]
    pub fn version(&self) -> Option<&DataExchangeIdentifier> {
        self.version.as_ref()
    }

    /// Revalidates this endpoint at a trust boundary.
    pub fn validate(&self) -> DataExchangeResult<()> {
        validate_identifier("endpoint.schema", &self.schema)?;
        validate_identifier("endpoint.format", &self.format)?;
        validate_identifier("endpoint.encoding", &self.encoding)?;

        if let Some(version) = &self.version {
            validate_identifier("endpoint.version", version)?;
        }

        Ok(())
    }
}

/// Declares how data is logically transferred.
///
/// This is descriptive intent. A streaming codec or transport must implement
/// the declared behavior and enforce its own explicit resource budgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferMode {
    /// Exchange a complete logical value or document.
    Complete,

    /// Exchange an ordered or unordered stream according to the contract.
    Streaming,

    /// Exchange changes relative to a separately identified state or version.
    Incremental,

    /// Exchange a reference to data managed by another subsystem.
    Reference,
}

/// Describes the ordering guarantee required at the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderingGuarantee {
    /// The contract makes no ordering guarantee.
    Unspecified,

    /// Preserve the source order when the source has a defined order.
    PreserveSourceOrder,

    /// Require a total order defined by an independently specified rule.
    RequireTotalOrder,
}

/// Declares how conversion-related information loss must be handled.
///
/// The codec or semantic adapter must determine whether a conversion is
/// lossy. This enum does not perform conversion or prove losslessness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataLossPolicy {
    /// Reject a conversion if it cannot preserve the required information.
    RejectLoss,

    /// Permit conversion only when its loss is explicitly declared.
    RequireExplicitDeclaration,

    /// Permit conversion loss under the enclosing program's policy.
    Permit,
}

/// Declarative integrity intent for exchanged data.
///
/// Algorithm names are open identifiers. This type does not calculate
/// digests, authenticate messages, or provide cryptographic guarantees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityRequirement {
    /// The contract makes no additional integrity requirement.
    Unspecified,

    /// A downstream codec or transport must apply the named digest scheme.
    Digest(DataExchangeIdentifier),

    /// A downstream trusted implementation must apply the named
    /// authentication/integrity scheme.
    Authenticated(DataExchangeIdentifier),
}

/// A validated, target-independent data-exchange contract.
///
/// The contract records what the boundary requires. It does not contain the
/// payload, codec implementation, credentials, transport connection, or
/// runtime state.
///
/// `BTreeMap` gives deterministic iteration order for extension metadata.
/// No semantic limit is imposed on the number of extension entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataExchangeContract {
    name: DataExchangeIdentifier,
    source: DataExchangeEndpoint,
    destination: DataExchangeEndpoint,
    transfer_mode: TransferMode,
    ordering: OrderingGuarantee,
    data_loss_policy: DataLossPolicy,
    integrity: IntegrityRequirement,
    extensions: BTreeMap<DataExchangeIdentifier, String>,
}

impl DataExchangeContract {
    /// Creates a contract with conservative defaults.
    ///
    /// Data loss is rejected unless the caller explicitly chooses a different
    /// policy. Ordering is unspecified unless requested.
    pub fn new(
        name: impl Into<String>,
        source: DataExchangeEndpoint,
        destination: DataExchangeEndpoint,
    ) -> DataExchangeResult<Self> {
        let contract = Self {
            name: DataExchangeIdentifier::new(name)?,
            source,
            destination,
            transfer_mode: TransferMode::Complete,
            ordering: OrderingGuarantee::Unspecified,
            data_loss_policy: DataLossPolicy::RejectLoss,
            integrity: IntegrityRequirement::Unspecified,
            extensions: BTreeMap::new(),
        };

        contract.validate()?;
        Ok(contract)
    }

    /// Sets the transfer intent.
    #[must_use]
    pub fn with_transfer_mode(mut self, mode: TransferMode) -> Self {
        self.transfer_mode = mode;
        self
    }

    /// Sets the ordering requirement.
    #[must_use]
    pub fn with_ordering(mut self, ordering: OrderingGuarantee) -> Self {
        self.ordering = ordering;
        self
    }

    /// Sets the conversion/data-loss policy.
    #[must_use]
    pub fn with_data_loss_policy(mut self, policy: DataLossPolicy) -> Self {
        self.data_loss_policy = policy;
        self
    }

    /// Sets the integrity requirement.
    #[must_use]
    pub fn with_integrity(
        mut self,
        integrity: IntegrityRequirement,
    ) -> Self {
        self.integrity = integrity;
        self
    }

    /// Adds or replaces an extension property.
    ///
    /// Extension keys are open-world identifiers. Values are opaque
    /// metadata, not executable instructions. Duplicate keys intentionally
    /// replace the previous value so the resulting contract has one
    /// unambiguous value for each key.
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> DataExchangeResult<Self> {
        let key = DataExchangeIdentifier::new(key)?;
        let value = value.into();

        if value.contains('\0') {
            return Err(DataExchangeError::new(
                DataExchangeErrorKind::InvalidMetadata,
                key.as_str(),
                "extension metadata values must not contain NUL",
            ));
        }

        self.extensions.insert(key, value);
        Ok(self)
    }

    /// Revalidates the contract, including its endpoints and metadata.
    ///
    /// Call this after decoding untrusted metadata or at another trust
    /// boundary. This method validates structure only; compatibility with
    /// actual schemas, formats, versions, and target capabilities must be
    /// checked by the owning semantic/compiler/runtime subsystems.
    pub fn validate(&self) -> DataExchangeResult<()> {
        validate_identifier("contract.name", &self.name)?;
        self.source.validate()?;
        self.destination.validate()?;

        for (key, value) in &self.extensions {
            validate_identifier("extension.key", key)?;

            if value.contains('\0') {
                return Err(DataExchangeError::new(
                    DataExchangeErrorKind::InvalidMetadata,
                    key.as_str(),
                    "extension metadata values must not contain NUL",
                ));
            }
        }

        match &self.integrity {
            IntegrityRequirement::Unspecified => {}
            IntegrityRequirement::Digest(algorithm)
            | IntegrityRequirement::Authenticated(algorithm) => {
                validate_identifier("integrity.algorithm", algorithm)?;
            }
        }

        Ok(())
    }

    /// Returns the contract name.
    #[must_use]
    pub fn name(&self) -> &DataExchangeIdentifier {
        &self.name
    }

    /// Returns the source endpoint.
    #[must_use]
    pub fn source(&self) -> &DataExchangeEndpoint {
        &self.source
    }

    /// Returns the destination endpoint.
    #[must_use]
    pub fn destination(&self) -> &DataExchangeEndpoint {
        &self.destination
    }

    /// Returns the transfer intent.
    #[must_use]
    pub const fn transfer_mode(&self) -> TransferMode {
        self.transfer_mode
    }

    /// Returns the ordering requirement.
    #[must_use]
    pub const fn ordering(&self) -> OrderingGuarantee {
        self.ordering
    }

    /// Returns the data-loss policy.
    #[must_use]
    pub const fn data_loss_policy(&self) -> DataLossPolicy {
        self.data_loss_policy
    }

    /// Returns the integrity requirement.
    #[must_use]
    pub fn integrity(&self) -> &IntegrityRequirement {
        &self.integrity
    }

    /// Returns extension metadata in deterministic key order.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<DataExchangeIdentifier, String> {
        &self.extensions
    }

    /// Looks up an extension property by its textual identifier.
    #[must_use]
    pub fn extension(&self, key: &str) -> Option<&str> {
        self.extensions
            .iter()
            .find(|(candidate, _)| candidate.as_str() == key)
            .map(|(_, value)| value.as_str())
    }

    /// Converts local validation failure into the repository-wide error type.
    pub fn validate_classical(&self) -> Result<(), ClassicalError> {
        self.validate()
            .map_err(DataExchangeError::into_classical_error)
    }
}

fn validate_identifier(
    subject: &str,
    identifier: &DataExchangeIdentifier,
) -> DataExchangeResult<()> {
    if identifier.as_str().trim().is_empty()
        || identifier.as_str().contains('\0')
    {
        return Err(DataExchangeError::new(
            DataExchangeErrorKind::InvalidIdentifier,
            subject,
            "identifier must be non-empty and must not contain NUL",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn endpoint(
        schema: &str,
        format: &str,
        encoding: &str,
    ) -> DataExchangeEndpoint {
        DataExchangeEndpoint::new(
            schema,
            format,
            encoding,
            None,
        )
        .expect("test endpoint should be valid")
    }

    fn contract() -> DataExchangeContract {
        DataExchangeContract::new(
            "example.exchange",
            endpoint("example.schema", "example.format", "example.encoding"),
            endpoint("destination.schema", "future.format", "future.encoding"),
        )
        .expect("test contract should be valid")
    }

    #[test]
    fn rejects_empty_identifiers() {
        let error = DataExchangeIdentifier::new("  ")
            .expect_err("blank identifier must be rejected");

        assert_eq!(
            error.kind(),
            DataExchangeErrorKind::InvalidIdentifier
        );
    }

    #[test]
    fn rejects_nul_in_identifiers() {
        assert!(DataExchangeIdentifier::new("format\0name").is_err());
    }

    #[test]
    fn supports_open_world_format_and_encoding_names() {
        let value = contract();

        assert_eq!(value.destination().format().as_str(), "future.format");
        assert_eq!(
            value.destination().encoding().as_str(),
            "future.encoding"
        );
    }

    #[test]
    fn defaults_to_rejecting_data_loss() {
        assert_eq!(
            contract().data_loss_policy(),
            DataLossPolicy::RejectLoss
        );
    }

    #[test]
    fn preserves_transfer_and_ordering_intent() {
        let value = contract()
            .with_transfer_mode(TransferMode::Streaming)
            .with_ordering(OrderingGuarantee::PreserveSourceOrder);

        assert_eq!(value.transfer_mode(), TransferMode::Streaming);
        assert_eq!(
            value.ordering(),
            OrderingGuarantee::PreserveSourceOrder
        );
    }

    #[test]
    fn extension_metadata_is_deterministic_and_replaceable() {
        let value = contract()
            .with_extension("vendor.property", "first")
            .expect("first extension should be valid")
            .with_extension("vendor.property", "second")
            .expect("replacement should be valid");

        assert_eq!(value.extension("vendor.property"), Some("second"));
        assert_eq!(value.extensions().len(), 1);
    }

    #[test]
    fn rejects_nul_in_extension_values() {
        let error = contract()
            .with_extension("vendor.property", "invalid\0value")
            .expect_err("NUL in extension metadata must be rejected");

        assert_eq!(error.kind(), DataExchangeErrorKind::InvalidMetadata);
    }

    #[test]
    fn supports_integrity_requirements_without_executing_them() {
        let algorithm = DataExchangeIdentifier::new("future.digest")
            .expect("algorithm identifier should be valid");

        let value = contract()
            .with_integrity(IntegrityRequirement::Digest(algorithm));

        assert!(value.validate().is_ok());
    }

    #[test]
    fn validates_endpoints_and_contract() {
        assert!(contract().validate().is_ok());
    }
}
