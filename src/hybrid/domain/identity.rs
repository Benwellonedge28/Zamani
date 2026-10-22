
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Domain Identity
//!
//! Path: `src/hybrid/domain/identity.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only; unsafe code is forbidden.
//!
//! ## Purpose
//!
//! Define stable, domain-neutral identities for computational domains
//! participating in Zamani hybrid computation.
//!
//! A domain identity describes a semantic computational domain, such as
//! `classical`, `quantum`, `hdl`, `accelerator`, `distributed`, or a
//! registered domain introduced by a third party.
//!
//! A domain is not a physical device, execution target, runtime session,
//! compiler backend, or hardware allocation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - `DomainId`, the canonical identifier of a semantic domain.
//! - `DomainIdentity`, descriptive identity metadata for that domain.
//! - `DomainIdError`, structured validation failures.
//! - Built-in domain identifier constants.
//! - Deterministic formatting and parsing of domain identifiers.
//!
//! This module does not own:
//!
//! - Domain implementation registration or registry lifecycle.
//! - Domain interface definitions or their version negotiation.
//! - AST, semantic-model, classical IR, quantum IR, or ZUIR identities.
//! - Hardware inventory, device selection, allocation, or scheduling.
//! - Domain operation semantics.
//! - Execution, networking, or vendor-specific SDKs.
//!
//! ## Identifier grammar
//!
//! An identifier is a dot-separated sequence of non-empty ASCII segments.
//! Each segment begins with an ASCII lowercase letter or digit and continues
//! with ASCII lowercase letters, digits, hyphens, or underscores.
//!
//! Examples:
//!
//! - `classical`
//! - `quantum`
//! - `accelerator.gpu`
//! - `org.example.quantum`
//! - `research.custom_domain_v2`
//!
//! Identifiers are case-sensitive. Uppercase letters, whitespace, empty
//! segments, leading/trailing dots, and other punctuation are rejected.
//! The constructor does not silently normalize an invalid identifier.
//!
//! There is deliberately no fixed maximum identifier length, segment count,
//! or number of domain identities. Input-processing budgets, if required,
//! belong to the caller's explicit operational policy.
//!
//! ## Portability
//!
//! Built-in identifiers are convenience constants, not a closed inventory.
//! Adding a domain does not require modifying this file.
//!
//! An identifier does not prove that a domain is installed, registered,
//! compatible, available, or executable. Those questions belong to the
//! registry, contract-validation, feasibility, compiler, and runtime layers.
//!
//! ## Serialization
//!
//! Serde represents `DomainId` as a string. Deserialization validates the
//! identifier, so malformed identifiers cannot enter through ordinary
//! Serde deserialization.
//!
//! `DomainIdentity` metadata is not a substitute for an authoritative domain
//! contract. A display name or description must never determine semantics.
//!
//! ## Integration
//!
//! - `domain/mod.rs` should declare this module and selectively re-export
//!   `DomainId`, `DomainIdentity`, and `DomainIdError`.
//! - `domain/interface.rs` should use `DomainId` for the semantic domain
//!   portion of an interface identity.
//! - `domain/registry.rs` should key registrations by `DomainId` and perform
//!   availability, duplicate-registration, and version checks.
//! - `domain/composition.rs` should use `DomainId` to describe participating
//!   domains; instance identity remains a separate concern.
//! - `contract.rs` already owns `DomainInterfaceRef`; its domain string
//!   should be validated or converted at the integration boundary rather
//!   than duplicated as a competing domain-identity implementation.
//! - `version.rs` remains the owner of `DomainContractVersion`.
//! - `ids.rs` remains the owner of hybrid graph and domain-instance IDs.
//! - `grammar/spec/hybrid.md` and `grammar/hybrid/` remain authoritative for
//!   language and hybrid-composition semantics.
//! - `src/compiler/hybrid_pipeline.rs` retains its existing compilation
//!   profile and target responsibilities; target-family types are not
//!   aliases for semantic domain identities.
//! - `quantum::ir` remains the canonical quantum semantic IR.
//!
//! ## Completion criteria
//!
//! - Invalid identifiers are rejected consistently.
//! - Built-in and third-party domain names use the same validation rules.
//! - Identity equality and ordering are deterministic.
//! - Serialization cannot bypass identifier validation.
//! - No hardware limits, closed domain inventory, or unsafe Rust are added.

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Canonical identity of a semantic computational domain.
///
/// The value is a validated, dot-separated identifier. It is deliberately
/// represented as an extensible string rather than a closed enum, allowing
/// future and third-party domains without changing this module.
///
/// Equality, ordering, and hashing use the complete canonical identifier.
/// No case folding, alias resolution, or registry lookup is performed.
///
/// # Examples
///
/// ```
/// use zamani_compiler::hybrid::domain::identity::DomainId;
///
/// let quantum = DomainId::new("quantum").unwrap();
/// let extension = DomainId::new("org.example.quantum").unwrap();
///
/// assert_ne!(quantum, extension);
/// assert_eq!(quantum.as_str(), "quantum");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(String);

/// Validation errors for semantic domain identifiers.
///
/// These errors describe malformed identifiers, not unavailable domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DomainIdError {
    /// The identifier is empty.
    Empty,

    /// The identifier contains an empty dot-separated segment.
    EmptySegment,

    /// A segment begins with a character other than an ASCII lowercase
    /// letter or digit.
    InvalidSegmentStart,

    /// A segment contains a character outside the allowed ASCII alphabet.
    InvalidCharacter,
}

impl fmt::Display for DomainIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("domain identifier cannot be empty"),
            Self::EmptySegment => {
                f.write_str("domain identifier contains an empty segment")
            }
            Self::InvalidSegmentStart => {
                f.write_str(
                    "each domain identifier segment must start with an \
                     ASCII lowercase letter or digit",
                )
            }
            Self::InvalidCharacter => {
                f.write_str(
                    "domain identifier contains a character outside the \
                     permitted ASCII alphabet",
                )
            }
        }
    }
}

impl std::error::Error for DomainIdError {}

impl DomainId {
    /// Canonical identifier for classical computation.
    pub const CLASSICAL: &'static str = "classical";

    /// Canonical identifier for quantum computation.
    pub const QUANTUM: &'static str = "quantum";

    /// Canonical identifier for HDL computation.
    pub const HDL: &'static str = "hdl";

    /// Canonical identifier for accelerator computation.
    pub const ACCELERATOR: &'static str = "accelerator";

    /// Canonical identifier for distributed computation.
    pub const DISTRIBUTED: &'static str = "distributed";

    /// Canonical identifier for AI computation.
    pub const AI: &'static str = "ai";

    /// Canonical identifier for data computation.
    pub const DATA: &'static str = "data";

    /// Canonical identifier for networking computation.
    pub const NETWORK: &'static str = "network";

    /// Canonical identifier for security computation.
    pub const SECURITY: &'static str = "security";

    /// Construct a domain identifier after validating its complete value.
    ///
    /// This method intentionally has no built-in length or segment-count
    /// ceiling. Callers processing untrusted input may enforce an explicit
    /// operational budget before allocating or parsing large values.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainIdError> {
        let value = value.into();
        validate_domain_id(&value)?;
        Ok(Self(value))
    }

    /// Return the canonical identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return whether this identity equals the supplied canonical identifier.
    ///
    /// The argument is compared literally; it is not normalized or parsed.
    #[must_use]
    pub fn is(&self, canonical_identifier: &str) -> bool {
        self.0 == canonical_identifier
    }

    /// Return the number of dot-separated segments.
    ///
    /// The result is derived from the stored identifier and does not impose
    /// a limit on the number of segments.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.0.bytes().filter(|byte| *byte == b'.').count() + 1
    }

    /// Return the final segment of this identifier.
    ///
    /// For example, `org.example.quantum` returns `quantum`.
    #[must_use]
    pub fn leaf(&self) -> &str {
        // Construction guarantees that the string is non-empty and that
        // every segment is non-empty.
        self.0
            .rsplit('.')
            .next()
            .expect("a validated domain identifier has a final segment")
    }

    /// Return the namespace prefix, if this identifier is namespaced.
    ///
    /// For `org.example.quantum`, this returns `Some("org.example")`.
    /// For `quantum`, this returns `None`.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.0.rsplit_once('.').map(|(namespace, _)| namespace)
    }

    /// Construct an identity for a built-in domain.
    ///
    /// The supplied value must be one of the constants documented on this
    /// type. This method does not register the domain or establish that an
    /// implementation exists.
    #[must_use]
    pub fn classical() -> Self {
        Self(Self::CLASSICAL.to_owned())
    }

    /// Construct the canonical quantum domain identity.
    #[must_use]
    pub fn quantum() -> Self {
        Self(Self::QUANTUM.to_owned())
    }

    /// Construct the canonical HDL domain identity.
    #[must_use]
    pub fn hdl() -> Self {
        Self(Self::HDL.to_owned())
    }

    /// Construct the canonical accelerator domain identity.
    #[must_use]
    pub fn accelerator() -> Self {
        Self(Self::ACCELERATOR.to_owned())
    }

    /// Construct the canonical distributed-computation domain identity.
    #[must_use]
    pub fn distributed() -> Self {
        Self(Self::DISTRIBUTED.to_owned())
    }

    /// Construct the canonical AI domain identity.
    #[must_use]
    pub fn ai() -> Self {
        Self(Self::AI.to_owned())
    }

    /// Construct the canonical data-computation domain identity.
    #[must_use]
    pub fn data() -> Self {
        Self(Self::DATA.to_owned())
    }

    /// Construct the canonical networking domain identity.
    #[must_use]
    pub fn network() -> Self {
        Self(Self::NETWORK.to_owned())
    }

    /// Construct the canonical security domain identity.
    #[must_use]
    pub fn security() -> Self {
        Self(Self::SECURITY.to_owned())
    }
}

impl AsRef<str> for DomainId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DomainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for DomainId {
    type Err = DomainIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for DomainId {
    type Error = DomainIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for DomainId {
    type Error = DomainIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Serialize for DomainId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DomainId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Descriptive identity metadata for a semantic computational domain.
///
/// `DomainIdentity` is intentionally distinct from `DomainId`: an ID is the
/// key used for equality and lookup, while a display name and description
/// are non-authoritative metadata.
///
/// The metadata is not proof that a domain has been registered or that an
/// implementation fulfils its advertised capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainIdentity {
    id: DomainId,

    /// Human-readable display name. This does not affect identity.
    display_name: String,

    /// Optional human-readable explanation. This does not affect identity.
    description: Option<String>,
}

impl DomainIdentity {
    /// Construct domain metadata with a validated canonical identifier.
    ///
    /// Display names and descriptions are descriptive text. Empty display
    /// names are rejected; descriptions may be absent or empty.
    pub fn new(
        id: DomainId,
        display_name: impl Into<String>,
        description: Option<String>,
    ) -> Result<Self, DomainIdentityError> {
        let display_name = display_name.into();

        if display_name.trim().is_empty() {
            return Err(DomainIdentityError::EmptyDisplayName);
        }

        Ok(Self {
            id,
            display_name,
            description,
        })
    }

    /// Return the canonical domain identifier.
    #[must_use]
    pub fn id(&self) -> &DomainId {
        &self.id
    }

    /// Return the descriptive display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return the optional descriptive text.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Replace descriptive metadata without changing domain identity.
    pub fn update_metadata(
        &mut self,
        display_name: impl Into<String>,
        description: Option<String>,
    ) -> Result<(), DomainIdentityError> {
        let display_name = display_name.into();

        if display_name.trim().is_empty() {
            return Err(DomainIdentityError::EmptyDisplayName);
        }

        self.display_name = display_name;
        self.description = description;
        Ok(())
    }
}

/// Errors for descriptive domain identity metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DomainIdentityError {
    /// A descriptive display name cannot be empty or whitespace-only.
    EmptyDisplayName,
}

impl fmt::Display for DomainIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDisplayName => {
                f.write_str("domain display name cannot be empty")
            }
        }
    }
}

impl std::error::Error for DomainIdentityError {}

/// Validate the complete canonical domain identifier.
///
/// This helper deliberately works over bytes because the identifier grammar
/// is ASCII-only. It rejects non-ASCII bytes without Unicode normalization.
fn validate_domain_id(value: &str) -> Result<(), DomainIdError> {
    if value.is_empty() {
        return Err(DomainIdError::Empty);
    }

    let mut segment_start = true;

    for byte in value.bytes() {
        if byte == b'.' {
            if segment_start {
                return Err(DomainIdError::EmptySegment);
            }

            segment_start = true;
            continue;
        }

        if segment_start {
            if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() {
                return Err(DomainIdError::InvalidSegmentStart);
            }

            segment_start = false;
            continue;
        }

        if !byte.is_ascii_lowercase()
            && !byte.is_ascii_digit()
            && byte != b'-'
            && byte != b'_'
        {
            return Err(DomainIdError::InvalidCharacter);
        }
    }

    if segment_start {
        return Err(DomainIdError::EmptySegment);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_builtin_domain_identities() {
        for value in [
            DomainId::CLASSICAL,
            DomainId::QUANTUM,
            DomainId::HDL,
            DomainId::ACCELERATOR,
            DomainId::DISTRIBUTED,
            DomainId::AI,
            DomainId::DATA,
            DomainId::NETWORK,
            DomainId::SECURITY,
        ] {
            assert_eq!(DomainId::new(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn accepts_extensible_namespaced_domain_ids() {
        let id = DomainId::new("org.example.quantum_v2").unwrap();

        assert_eq!(id.as_str(), "org.example.quantum_v2");
        assert_eq!(id.segment_count(), 3);
        assert_eq!(id.namespace(), Some("org.example"));
        assert_eq!(id.leaf(), "quantum_v2");
    }

    #[test]
    fn rejects_empty_and_malformed_identifiers() {
        assert_eq!(DomainId::new(""), Err(DomainIdError::Empty));
        assert_eq!(
            DomainId::new(".quantum"),
            Err(DomainIdError::EmptySegment)
        );
        assert_eq!(
            DomainId::new("quantum."),
            Err(DomainIdError::EmptySegment)
        );
        assert_eq!(
            DomainId::new("quantum..accelerator"),
            Err(DomainIdError::EmptySegment)
        );
        assert_eq!(
            DomainId::new("Quantum"),
            Err(DomainIdError::InvalidSegmentStart)
        );
        assert_eq!(
            DomainId::new("org.Example.quantum"),
            Err(DomainIdError::InvalidCharacter)
        );
        assert_eq!(
            DomainId::new("org example.quantum"),
            Err(DomainIdError::InvalidCharacter)
        );
        assert_eq!(
            DomainId::new("org.example/quantum"),
            Err(DomainIdError::InvalidCharacter)
        );
    }

    #[test]
    fn identity_is_case_sensitive_and_does_not_alias_domains() {
        let quantum = DomainId::new("quantum").unwrap();
        let custom = DomainId::new("org.example.quantum").unwrap();

        assert_ne!(quantum, custom);
        assert!(quantum.is("quantum"));
        assert!(!quantum.is("Quantum"));
    }

    #[test]
    fn parses_and_displays_canonical_identifiers() {
        let id: DomainId = "accelerator.gpu".parse().unwrap();

        assert_eq!(id.to_string(), "accelerator.gpu");
        assert_eq!(id.as_ref(), "accelerator.gpu");
    }

    #[test]
    fn serializes_domain_ids_as_strings_and_validates_on_decode() {
        let id = DomainId::new("org.example.quantum").unwrap();

        let encoded = serde_json::to_string(&id).unwrap();
        assert_eq!(encoded, "\"org.example.quantum\"");

        let decoded: DomainId = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, id);

        assert!(serde_json::from_str::<DomainId>("\"Quantum\"").is_err());
        assert!(serde_json::from_str::<DomainId>("\"quantum..gpu\"").is_err());
    }

    #[test]
    fn ordering_and_hashing_are_consistent_with_equality() {
        use std::collections::HashSet;

        let first = DomainId::new("classical").unwrap();
        let second = DomainId::new("quantum").unwrap();
        let repeated = DomainId::new("classical").unwrap();

        assert_eq!(first, repeated);
        assert!(first < second);

        let values = HashSet::from([first, second, repeated]);
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn metadata_does_not_change_canonical_identity() {
        let id = DomainId::quantum();

        let mut identity = DomainIdentity::new(
            id.clone(),
            "Quantum Computing",
            Some("Quantum semantic domain".to_owned()),
        )
        .unwrap();

        assert_eq!(identity.id(), &id);
        assert_eq!(identity.display_name(), "Quantum Computing");

        identity
            .update_metadata("Quantum", Some("Updated description".to_owned()))
            .unwrap();

        assert_eq!(identity.id(), &id);
        assert_eq!(identity.display_name(), "Quantum");
        assert_eq!(identity.description(), Some("Updated description"));
    }

    #[test]
    fn rejects_empty_display_names() {
        let id = DomainId::classical();

        assert_eq!(
            DomainIdentity::new(id.clone(), "  ", None),
            Err(DomainIdentityError::EmptyDisplayName)
        );

        let mut identity =
            DomainIdentity::new(id.clone(), "Classical", None).unwrap();

        assert_eq!(
            identity.update_metadata("", None),
            Err(DomainIdentityError::EmptyDisplayName)
        );

        // A rejected update must not partially mutate the existing value.
        assert_eq!(identity.id(), &id);
        assert_eq!(identity.display_name(), "Classical");
    }

    #[test]
    fn identifier_length_and_segment_count_are_not_artificially_capped() {
        let long_segment = "a".repeat(8_192);
        let value = format!("org.example.{long_segment}");
        let id = DomainId::new(value.clone()).unwrap();

        assert_eq!(id.as_str(), value);
        assert_eq!(id.segment_count(), 3);

        let many_segments = std::iter::repeat_n("x", 512)
            .collect::<Vec<_>>()
            .join(".");
        let id = DomainId::new(many_segments).unwrap();

        assert_eq!(id.segment_count(), 512);
    }
}
