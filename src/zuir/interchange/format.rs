//! ZUIR interchange format contracts.
//!
//! PURPOSE
//!
//! Defines stable, format-independent identities, versions, capabilities,
//! descriptors, and negotiation rules for importing and exporting
//! computational representations.
//!
//! STATUS
//!
//! Proposed implementation. The module must be wired into the actual
//! `src/zuir/interchange/mod.rs` before it can be considered integrated.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/compatibility.md`, where applicable
//!
//! OWNS
//!
//! - Interchange format identity and version values.
//! - Namespaced capability identifiers and deterministic capability sets.
//! - Format descriptors and descriptor validation.
//! - Format negotiation decisions.
//! - The format-provider contract.
//!
//! DOES_NOT_OWN
//!
//! - Source grammar or parsing.
//! - ZUIR operation, type, or value definitions.
//! - Import/export algorithms or semantic conversion.
//! - Serialization encoding, decoding, or schema migration.
//! - Target discovery, hardware allocation, or execution.
//! - Source-map or diagnostic storage.
//!
//! DEPENDENCIES
//!
//! Standard library only. No dependency on ZUIR nodes, frontend AST types,
//! a particular interchange format, or a specific target.
//!
//! SAFETY
//!
//! Safe Rust only. No `unsafe` code.
//!
//! SCALABILITY
//!
//! Collections grow dynamically. This module defines no universal maximum
//! format count, capability count, media type count, or file size.
//! Concrete allocation and input limits belong to the caller's explicit
//! resource policy.
//!
//! DETERMINISM
//!
//! Ordered sets and maps are used wherever iteration order is observable.
//! Negotiation does not select an arbitrary first match.
//!
//! COMPATIBILITY
//!
//! Version values are explicit. This module does not assume that two format
//! versions are interchangeable or that a newer version can always be read.
//! Compatibility must be declared by the format implementation.
//!
//! INTEGRATION
//!
//! `interchange/import.rs` and `interchange/export.rs` should consume these
//! contracts. `interchange/migration.rs` owns version migration. Serialization
//! owns the encoded representation. The actual parent module must declare
//! `pub mod format;` and re-export only the APIs intended to be public.
//!
//! FREEZE CRITERIA
//!
//! - Unit tests pass.
//! - Parent module exports are wired to these exact types.
//! - Import and export adapters declare their supported versions and
//!   capabilities.
//! - Negotiation rejects ambiguous and incompatible requests.
//! - No caller silently treats an unknown capability as supported.
//!
//! NOTE
//!
//! This file is deliberately self-contained so its foundational contracts
//! can be reviewed before dependent interchange modules are implemented.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

/// A validated, stable interchange format identifier.
///
/// Identifiers are namespaced strings such as `zamani.zuir`,
/// `openqasm`, or `org.example.custom-ir`.
///
/// The identifier names a format family, not a particular version, device,
/// compiler, or implementation.
///
/// The spelling is preserved exactly. Case folding is not performed.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatId(String);

impl FormatId {
    /// Construct an identifier after validating its syntax.
    ///
    /// The grammar is intentionally independent of any particular format:
    ///
    /// - one or more ASCII alphanumeric characters, `.`, `_`, `-`, or `/`;
    /// - the first and last characters must be alphanumeric;
    /// - consecutive separators are permitted to avoid imposing a
    ///   namespace convention on future formats.
    ///
    /// Empty strings, whitespace, control characters, and non-ASCII
    /// characters are rejected. Human-readable names belong in metadata.
    pub fn new(value: impl Into<String>) -> Result<Self, FormatError> {
        let value = value.into();

        let mut chars = value.chars();

        let first = chars.next().ok_or_else(|| {
            FormatError::InvalidIdentifier {
                kind: "format",
                value: value.clone(),
            }
        })?;

        let last = value.chars().last().ok_or_else(|| {
            FormatError::InvalidIdentifier {
                kind: "format",
                value: value.clone(),
            }
        })?;

        let is_alphanumeric = |ch: char| ch.is_ascii_alphanumeric();
        let is_allowed = |ch: char| {
            is_alphanumeric(ch) || matches!(ch, '.' | '_' | '-' | '/')
        };

        if !is_alphanumeric(first)
            || !is_alphanumeric(last)
            || !value.chars().all(is_allowed)
        {
            return Err(FormatError::InvalidIdentifier {
                kind: "format",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FormatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A numeric version for a format or interchange schema.
///
/// Numeric components avoid lexicographic version comparisons and do not
/// assume a fixed machine architecture.
///
/// Compatibility is not inferred from the numbers alone. In particular,
/// a greater version does not imply that a reader supports it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl FormatVersion {
    /// Construct a numeric version.
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn major(self) -> u64 {
        self.major
    }

    pub const fn minor(self) -> u64 {
        self.minor
    }

    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for FormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A validated, namespaced capability identifier.
///
/// Capabilities describe a format's declared abilities, not the capabilities
/// of a physical device or the permissions granted to execute a program.
///
/// Examples include `syntax.quantum`, `semantics.control-flow`,
/// `metadata.provenance`, and `encoding.streaming`.
///
/// Unknown capability identifiers can be retained by callers, but must
/// never be interpreted as supported without an explicit contract.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Construct a capability identifier.
    ///
    /// The accepted spelling is the same portable ASCII identifier grammar
    /// used by `FormatId`.
    pub fn new(value: impl Into<String>) -> Result<Self, FormatError> {
        let value = value.into();

        let first = value.chars().next();
        let last = value.chars().last();

        let is_alphanumeric = |ch: char| ch.is_ascii_alphanumeric();
        let is_allowed = |ch: char| {
            is_alphanumeric(ch) || matches!(ch, '.' | '_' | '-' | '/')
        };

        if first.is_none_or(|ch| !is_alphanumeric(ch))
            || last.is_none_or(|ch| !is_alphanumeric(ch))
            || !value.chars().all(is_allowed)
        {
            return Err(FormatError::InvalidIdentifier {
                kind: "capability",
                value,
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Deterministically ordered set of declared format capabilities.
///
/// The set is extensible: adding a capability does not require adding a
/// variant to a closed Rust enum or modifying the universal IR.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormatCapabilities {
    entries: BTreeSet<CapabilityId>,
}

impl FormatCapabilities {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a capability. Returns `true` if it was newly inserted.
    pub fn insert(&mut self, capability: CapabilityId) -> bool {
        self.entries.insert(capability)
    }

    /// Return whether this exact capability is declared.
    pub fn contains(&self, capability: &CapabilityId) -> bool {
        self.entries.contains(capability)
    }

    /// Return the number of distinct capabilities.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate in deterministic lexical order.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityId> {
        self.entries.iter()
    }

    /// True only if every required capability is declared.
    pub fn contains_all<'a>(
        &self,
        required: impl IntoIterator<Item = &'a CapabilityId>,
    ) -> bool {
        required.into_iter().all(|item| self.contains(item))
    }

    /// Return the capabilities declared by both sets.
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            entries: self.entries.intersection(&other.entries).cloned().collect(),
        }
    }
}

/// Description of one supported interchange format version.
///
/// This descriptor is declarative. It does not itself prove that a format
/// implementation preserves ZUIR semantics; adapters and conformance tests
/// must establish that separately.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatDescriptor {
    id: FormatId,
    version: FormatVersion,
    capabilities: FormatCapabilities,
    media_types: BTreeSet<String>,
    extensions: BTreeSet<String>,
}

impl FormatDescriptor {
    /// Create a descriptor with no optional media types or file extensions.
    pub fn new(id: FormatId, version: FormatVersion) -> Self {
        Self {
            id,
            version,
            capabilities: FormatCapabilities::new(),
            media_types: BTreeSet::new(),
            extensions: BTreeSet::new(),
        }
    }

    pub fn id(&self) -> &FormatId {
        &self.id
    }

    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    pub fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    pub fn media_types(&self) -> impl Iterator<Item = &str> {
        self.media_types.iter().map(String::as_str)
    }

    pub fn extensions(&self) -> impl Iterator<Item = &str> {
        self.extensions.iter().map(String::as_str)
    }

    /// Declare an additional capability.
    pub fn with_capability(mut self, capability: CapabilityId) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Declare a media type, such as `application/json`.
    ///
    /// This validates basic portable spelling, not the complete IANA media
    /// type registry. Parameters should be normalized by the adapter before
    /// they are used as lookup keys.
    pub fn with_media_type(
        mut self,
        media_type: impl Into<String>,
    ) -> Result<Self, FormatError> {
        let media_type = media_type.into();

        if !is_valid_media_type(&media_type) {
            return Err(FormatError::InvalidMediaType(media_type));
        }

        self.media_types.insert(media_type);
        Ok(self)
    }

    /// Declare a file extension without a leading dot.
    ///
    /// Extensions are hints for discovery, not proof of content or format.
    pub fn with_extension(
        mut self,
        extension: impl Into<String>,
    ) -> Result<Self, FormatError> {
        let extension = extension.into();

        if !is_valid_extension(&extension) {
            return Err(FormatError::InvalidExtension(extension));
        }

        self.extensions.insert(extension);
        Ok(self)
    }

    /// Validate the descriptor's own invariants.
    pub fn validate(&self) -> Result<(), FormatError> {
        for media_type in &self.media_types {
            if !is_valid_media_type(media_type) {
                return Err(FormatError::InvalidMediaType(media_type.clone()));
            }
        }

        for extension in &self.extensions {
            if !is_valid_extension(extension) {
                return Err(FormatError::InvalidExtension(extension.clone()));
            }
        }

        Ok(())
    }
}

/// Validate a portable media-type spelling.
///
/// The check intentionally does not claim to implement the full media type
/// standards registry. A concrete adapter may apply stricter rules.
fn is_valid_media_type(value: &str) -> bool {
    let Some((type_part, subtype_part)) = value.split_once('/') else {
        return false;
    };

    if type_part.is_empty() || subtype_part.is_empty() {
        return false;
    }

    if subtype_part.contains('/') {
        return false;
    }

    let valid_token = |part: &str| {
        part.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'!' | b'#' | b'$' | b'&' | b'-' | b'^' | b'_'
                        | b'.' | b'+'
                )
        })
    };

    valid_token(type_part) && valid_token(subtype_part)
}

fn is_valid_extension(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('.')
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')
        })
}

/// Error raised when an interchange contract is malformed or incompatible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatError {
    InvalidIdentifier {
        kind: &'static str,
        value: String,
    },
    InvalidMediaType(String),
    InvalidExtension(String),
    InvalidDescriptor(String),
    UnsupportedFormat {
        id: FormatId,
        version: FormatVersion,
    },
    MissingCapabilities(Vec<CapabilityId>),
    AmbiguousFormat {
        candidates: Vec<FormatId>,
    },
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind} identifier: {value:?}")
            }
            Self::InvalidMediaType(value) => {
                write!(f, "invalid media type: {value:?}")
            }
            Self::InvalidExtension(value) => {
                write!(f, "invalid file extension: {value:?}")
            }
            Self::InvalidDescriptor(message) => {
                write!(f, "invalid format descriptor: {message}")
            }
            Self::UnsupportedFormat { id, version } => {
                write!(f, "unsupported format {id} version {version}")
            }
            Self::MissingCapabilities(capabilities) => {
                write!(f, "required format capabilities are missing: ")?;
                for (index, capability) in capabilities.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{capability}")?;
                }
                Ok(())
            }
            Self::AmbiguousFormat { candidates } => {
                f.write_str("format negotiation is ambiguous: ")?;
                for (index, id) in candidates.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{id}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for FormatError {}

/// A request for a specific interchange format version and capability set.
///
/// `required_capabilities` represents semantic requirements, not preferences.
/// A candidate missing any required capability is not a valid match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatRequirement {
    id: FormatId,
    version: FormatVersion,
    required_capabilities: FormatCapabilities,
}

impl FormatRequirement {
    pub fn new(id: FormatId, version: FormatVersion) -> Self {
        Self {
            id,
            version,
            required_capabilities: FormatCapabilities::new(),
        }
    }

    pub fn requiring(mut self, capability: CapabilityId) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    pub fn id(&self) -> &FormatId {
        &self.id
    }

    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    pub fn required_capabilities(&self) -> &FormatCapabilities {
        &self.required_capabilities
    }

    /// Match only the exact declared format and version.
    ///
    /// This intentionally does not infer compatibility across versions.
    /// A migration or adapter must explicitly establish that compatibility.
    pub fn matches_exact(
        &self,
        descriptor: &FormatDescriptor,
    ) -> Result<(), FormatError> {
        if self.id != descriptor.id || self.version != descriptor.version {
            return Err(FormatError::UnsupportedFormat {
                id: self.id.clone(),
                version: self.version,
            });
        }

        let missing: Vec<CapabilityId> = self
            .required_capabilities
            .iter()
            .filter(|capability| !descriptor.capabilities.contains(capability))
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(FormatError::MissingCapabilities(missing));
        }

        Ok(())
    }
}

/// Contract implemented by an interchange format adapter.
///
/// Implementations should return a descriptor that accurately reflects the
/// behavior actually supported by that implementation. Advertising a
/// capability without implementing it violates this contract.
///
/// Import and export behavior should be defined by separate adapter traits
/// in their respective modules; this trait deliberately does not conflate
/// format identification with the ability to read or write.
pub trait InterchangeFormat: Send + Sync {
    /// Return the stable descriptor for this format implementation.
    fn descriptor(&self) -> &FormatDescriptor;
}

/// Deterministically select an exact descriptor match.
///
/// This function intentionally returns an error if more than one candidate
/// matches. A registry must not silently choose whichever adapter happened
/// to be inserted first.
///
/// Version migration and format conversion are separate operations.
pub fn negotiate_exact<'a>(
    requirement: &FormatRequirement,
    candidates: &'a [FormatDescriptor],
) -> Result<&'a FormatDescriptor, FormatError> {
    let mut matched: Vec<&FormatDescriptor> = Vec::new();

    for candidate in candidates {
        if requirement.matches_exact(candidate).is_ok() {
            matched.push(candidate);
        }
    }

    match matched.as_slice() {
        [only] => Ok(*only),
        [] => Err(FormatError::UnsupportedFormat {
            id: requirement.id.clone(),
            version: requirement.version,
        }),
        _ => {
            let mut ids: Vec<FormatId> =
                matched.iter().map(|item| item.id.clone()).collect();
            ids.sort();
            ids.dedup();

            Err(FormatError::AmbiguousFormat { candidates: ids })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> FormatId {
        FormatId::new(value).expect("valid format id")
    }

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability id")
    }

    fn descriptor() -> FormatDescriptor {
        FormatDescriptor::new(id("zamani.zuir"), FormatVersion::new(1, 0, 0))
            .with_capability(capability("semantics.control-flow"))
            .with_capability(capability("semantics.quantum"))
    }

    #[test]
    fn accepts_portable_namespaced_identifiers() {
        assert_eq!(
            id("org.example.custom-ir").as_str(),
            "org.example.custom-ir"
        );
        assert_eq!(capability("semantics.quantum").as_str(), "semantics.quantum");
    }

    #[test]
    fn rejects_empty_or_malformed_identifiers() {
        assert!(FormatId::new("").is_err());
        assert!(FormatId::new(".bad").is_err());
        assert!(FormatId::new("bad/").is_err());
        assert!(FormatId::new("has whitespace").is_err());
        assert!(CapabilityId::new("bad capability").is_err());
    }

    #[test]
    fn version_order_is_numeric() {
        assert!(FormatVersion::new(2, 0, 0) > FormatVersion::new(1, 99, 99));
        assert!(FormatVersion::new(1, 10, 0) > FormatVersion::new(1, 9, 0));
    }

    #[test]
    fn capability_iteration_is_deterministic() {
        let mut capabilities = FormatCapabilities::new();
        capabilities.insert(capability("semantics.quantum"));
        capabilities.insert(capability("semantics.classical"));
        capabilities.insert(capability("semantics.quantum"));

        let values: Vec<&str> = capabilities.iter().map(CapabilityId::as_str).collect();

        assert_eq!(
            values,
            vec!["semantics.classical", "semantics.quantum"]
        );
    }

    #[test]
    fn descriptor_validates_media_types_and_extensions() {
        let value = FormatDescriptor::new(
            id("zamani.zuir"),
            FormatVersion::new(1, 0, 0),
        )
        .with_media_type("application/json")
        .expect("valid media type")
        .with_extension("zui")
        .expect("valid extension");

        assert!(value.validate().is_ok());
        assert!(
            FormatDescriptor::new(id("example.format"), FormatVersion::new(1, 0, 0))
                .with_media_type("not-a-media-type")
                .is_err()
        );
        assert!(
            FormatDescriptor::new(id("example.format"), FormatVersion::new(1, 0, 0))
                .with_extension(".json")
                .is_err()
        );
    }

    #[test]
    fn exact_negotiation_requires_every_capability() {
        let requirement = FormatRequirement::new(
            id("zamani.zuir"),
            FormatVersion::new(1, 0, 0),
        )
        .requiring(capability("semantics.quantum"))
        .requiring(capability("semantics.control-flow"));

        assert!(
            negotiate_exact(&requirement, &[descriptor()]).is_ok()
        );

        let missing = FormatRequirement::new(
            id("zamani.zuir"),
            FormatVersion::new(1, 0, 0),
        )
        .requiring(capability("encoding.lossless-hdl"));

        assert!(
            matches!(
                negotiate_exact(&missing, &[descriptor()]),
                Err(FormatError::UnsupportedFormat { .. })
            )
        );
    }

    #[test]
    fn negotiation_rejects_unknown_versions() {
        let requirement = FormatRequirement::new(
            id("zamani.zuir"),
            FormatVersion::new(2, 0, 0),
        );

        assert!(
            matches!(
                negotiate_exact(&requirement, &[descriptor()]),
                Err(FormatError::UnsupportedFormat { .. })
            )
        );
    }

    #[test]
    fn negotiation_rejects_ambiguous_matches() {
        let candidate = descriptor();
        let requirement = FormatRequirement::new(
            id("zamani.zuir"),
            FormatVersion::new(1, 0, 0),
        );

        assert!(
            matches!(
                negotiate_exact(
                    &requirement,
                    &[candidate.clone(), candidate]
                ),
                Err(FormatError::AmbiguousFormat { .. })
            )
        );
    }

    #[test]
    fn unsupported_capabilities_are_not_implicitly_added() {
        let value = descriptor();
        assert!(
            !value
                .capabilities()
                .contains(&capability("encoding.lossless-hdl"))
        );
    }
}