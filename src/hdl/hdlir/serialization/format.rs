
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Artifact Format Contract
//!
//! Path: `src/hdl/hdlir/serialization/format.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the versioned metadata envelope for persistent HDLIR artifacts.
//! The envelope declares the format version, required features, schema and
//! dialect requirements, canonical ordering policy, and optional payload
//! integrity metadata.
//!
//! This module defines the logical artifact contract, not a byte encoder,
//! decoder, migration engine, or in-memory IR graph.
//!
//! ## Authority
//!
//! This module must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/fingerprint.rs`
//!
//! The grammar owns source syntax. The frontend owns source semantics.
//! ZUIR owns universal computational representation. HDLIR owns the
//! hardware-domain representation. This module owns artifact metadata only.
//!
//! ## Integration
//!
//! - `serialization/mod.rs` exposes the artifact-format API.
//! - `encode.rs` serializes validated metadata and payloads.
//! - `decode.rs` parses untrusted metadata and validates it before use.
//! - `migration.rs` handles explicit, semantics-preserving format migrations.
//! - `version.rs` owns format-version parsing and compatibility policy.
//! - `schema/registry.rs` validates schema and dialect requirements against
//!   definitions actually registered by the reader.
//! - `fingerprint.rs` owns the repository's fingerprint protocol.
//! - `HDLIR-SPEC.md` and `compatibility.md` remain the normative contracts.
//!
//! This module deliberately does not import the future schema registry or
//! graph implementation, avoiding a dependency cycle.
//!
//! ## Scalability
//!
//! No universal hardware-capacity limits are introduced here.
//! Schema and dialect requirements use ordered collections that grow with
//! available resources. Bit widths, memory sizes, resource counts, and
//! parameter values belong to the payload and remain symbolic when required.
//!
//! The format's version components and integrity digest encoding are finite
//! protocol representations, not hardware-size limits.
//!
//! Decoders must enforce caller-configured operational budgets before
//! accepting untrusted input. This module does not claim that arbitrarily
//! large input can be processed with finite resources.
//!
//! ## Security
//!
//! Deserialization does not by itself establish that an artifact is valid,
//! trusted, compatible, or safe to execute. Callers must validate metadata,
//! verify integrity where declared, resolve required schemas and dialects,
//! decode the payload under explicit budgets, and verify the resulting IR.
//!
//! Unknown required features must be rejected. Optional feature declarations
//! may be ignored only when their specified semantics permit it.
//! A digest is evidence of payload integrity only after a trusted implementation
//! recomputes and compares it; a digest is not an authenticity signature.
//!
//! ## Freeze criteria
//!
//! - Metadata has deterministic ordering.
//! - Invalid names, duplicate requirements, and malformed digests are rejected.
//! - Format-version compatibility is delegated to `HdlirVersion`.
//! - Required-feature compatibility is explicit.
//! - Integrity coverage is unambiguous.
//! - Malformed external data never becomes verified IR merely by decoding.
//! - Serialization and integration tests pass.

use core::fmt;
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::version::{
    HdlirVersion, VersionCompatibilityError, HDLIR_FORMAT_VERSION,
};

/// Canonical identifier for the HDLIR artifact family.
///
/// This identifier is independent of the crate version, Zamani language
/// version, grammar version, ZUIR version, and dialect versions.
pub const HDLIR_ARTIFACT_FORMAT_ID: &str = "zamani.hdlir";

/// Canonical textual name of the SHA-256 integrity algorithm.
pub const SHA256_ALGORITHM_ID: &str = "sha256";

/// Metadata describing one required or optional schema or dialect.
///
/// The name is a stable, qualified identifier owned by the relevant
/// schema or dialect registry. The version describes that definition's
/// contract, not the enclosing artifact format.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefinitionRequirement {
    name: String,
    version: HdlirVersion,
    required: bool,
}

impl DefinitionRequirement {
    /// Construct a definition requirement.
    ///
    /// The name must be a non-empty canonical qualified identifier.
    pub fn new(
        name: impl Into<String>,
        version: HdlirVersion,
        required: bool,
    ) -> Result<Self, FormatError> {
        let name = name.into();
        validate_qualified_name(&name, "definition name")?;

        Ok(Self {
            name,
            version,
            required,
        })
    }

    /// Return the canonical definition name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the required definition version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return whether the definition is required to interpret the artifact.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }
}

/// Policy for canonical ordering of artifact metadata and payload records.
///
/// This policy is part of the persistent contract. It must not depend on
/// hash-map iteration order, host endianness, pointers, or discovery order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalOrdering {
    /// Sort named metadata by canonical UTF-8 identifier order and preserve
    /// explicitly semantic sequence order in the payload.
    ///
    /// Ordered collections in this module implement the metadata portion.
    #[default]
    CanonicalIdentifiers,
}

/// Algorithm declared for payload integrity verification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityAlgorithm {
    /// SHA-256 over the exact serialized payload bytes.
    ///
    /// The encoder and decoder must agree on which bytes constitute the
    /// payload. The metadata envelope itself is excluded from this digest.
    Sha256,
}

impl IntegrityAlgorithm {
    /// Return the stable textual algorithm identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => SHA256_ALGORITHM_ID,
        }
    }
}

/// Integrity metadata for the exact serialized payload bytes.
///
/// The digest is encoded as lowercase hexadecimal. This structure declares
/// the algorithm and digest; it does not calculate or verify the digest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PayloadIntegrity {
    algorithm: IntegrityAlgorithm,
    digest_hex: String,
}

impl PayloadIntegrity {
    /// Construct payload integrity metadata from a lowercase hexadecimal
    /// digest.
    ///
    /// The digest length is determined by the selected algorithm rather than
    /// by an arbitrary artifact-size or hardware-capacity limit.
    pub fn new(
        algorithm: IntegrityAlgorithm,
        digest_hex: impl Into<String>,
    ) -> Result<Self, FormatError> {
        let digest_hex = digest_hex.into();
        validate_digest(algorithm, &digest_hex)?;

        Ok(Self {
            algorithm,
            digest_hex,
        })
    }

    /// Return the declared digest algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> IntegrityAlgorithm {
        self.algorithm
    }

    /// Return the lowercase hexadecimal digest.
    #[must_use]
    pub fn digest_hex(&self) -> &str {
        &self.digest_hex
    }
}

/// Versioned HDLIR artifact metadata.
///
/// All fields are private so ordinary Rust callers construct metadata through
/// validated constructors and setters. Deserialization must still be followed
/// by [`Self::validate`], because external data is untrusted.
///
/// The ordered maps and sets give the metadata a stable logical ordering.
/// The encoder remains responsible for producing deterministic bytes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactFormat {
    format_id: String,

    #[serde(with = "version_serde")]
    format_version: HdlirVersion,

    ordering: CanonicalOrdering,

    /// Features required for correct interpretation of this artifact.
    required_features: BTreeSet<String>,

    /// Features declared by the writer but not required for interpretation.
    optional_features: BTreeSet<String>,

    /// Schema requirements keyed by canonical qualified schema name.
    schemas: BTreeMap<String, DefinitionRequirement>,

    /// Dialect requirements keyed by canonical qualified dialect name.
    dialects: BTreeMap<String, DefinitionRequirement>,

    /// Optional integrity metadata for the serialized payload bytes.
    integrity: Option<PayloadIntegrity>,

    /// Optional semantic fingerprint represented as lowercase hexadecimal.
    ///
    /// Its algorithm and protocol version must be defined by the relevant
    /// fingerprint contract. It is not interchangeable with payload integrity.
    semantic_fingerprint_hex: Option<String>,
}

impl ArtifactFormat {
    /// Create a new artifact envelope using the current format version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            format_id: HDLIR_ARTIFACT_FORMAT_ID.to_owned(),
            format_version: HDLIR_FORMAT_VERSION,
            ordering: CanonicalOrdering::CanonicalIdentifiers,
            required_features: BTreeSet::new(),
            optional_features: BTreeSet::new(),
            schemas: BTreeMap::new(),
            dialects: BTreeMap::new(),
            integrity: None,
            semantic_fingerprint_hex: None,
        }
    }

    /// Return the artifact-format identifier.
    #[must_use]
    pub fn format_id(&self) -> &str {
        &self.format_id
    }

    /// Return the artifact-format version.
    #[must_use]
    pub const fn format_version(&self) -> HdlirVersion {
        self.format_version
    }

    /// Return the canonical ordering policy.
    #[must_use]
    pub const fn ordering(&self) -> CanonicalOrdering {
        self.ordering
    }

    /// Return required feature identifiers in canonical order.
    #[must_use]
    pub fn required_features(&self) -> &BTreeSet<String> {
        &self.required_features
    }

    /// Return optional feature identifiers in canonical order.
    #[must_use]
    pub fn optional_features(&self) -> &BTreeSet<String> {
        &self.optional_features
    }

    /// Return schema requirements in canonical name order.
    #[must_use]
    pub fn schemas(&self) -> &BTreeMap<String, DefinitionRequirement> {
        &self.schemas
    }

    /// Return dialect requirements in canonical name order.
    #[must_use]
    pub fn dialects(&self) -> &BTreeMap<String, DefinitionRequirement> {
        &self.dialects
    }

    /// Return optional payload integrity metadata.
    #[must_use]
    pub fn integrity(&self) -> Option<&PayloadIntegrity> {
        self.integrity.as_ref()
    }

    /// Return the optional semantic fingerprint.
    #[must_use]
    pub fn semantic_fingerprint_hex(&self) -> Option<&str> {
        self.semantic_fingerprint_hex.as_deref()
    }

    /// Set the format version.
    ///
    /// Writers should normally retain [`HDLIR_FORMAT_VERSION`]. This setter
    /// exists for explicitly specified migration and compatibility tooling.
    pub fn set_format_version(&mut self, version: HdlirVersion) {
        self.format_version = version;
    }

    /// Add a required feature identifier.
    ///
    /// Feature names are canonical qualified identifiers. Duplicate insertion
    /// is idempotent. A feature cannot be both required and optional.
    pub fn require_feature(
        &mut self,
        feature: impl Into<String>,
    ) -> Result<(), FormatError> {
        let feature = feature.into();
        validate_qualified_name(&feature, "feature identifier")?;

        if self.optional_features.contains(&feature) {
            return Err(FormatError::FeatureClassificationConflict(feature));
        }

        self.required_features.insert(feature);
        Ok(())
    }

    /// Add an optional feature identifier.
    ///
    /// Feature names are canonical qualified identifiers. Duplicate insertion
    /// is idempotent. A feature cannot be both required and optional.
    pub fn declare_optional_feature(
        &mut self,
        feature: impl Into<String>,
    ) -> Result<(), FormatError> {
        let feature = feature.into();
        validate_qualified_name(&feature, "feature identifier")?;

        if self.required_features.contains(&feature) {
            return Err(FormatError::FeatureClassificationConflict(feature));
        }

        self.optional_features.insert(feature);
        Ok(())
    }

    /// Register a schema requirement.
    ///
    /// Duplicate names are rejected rather than silently replacing an earlier
    /// version or required/optional classification.
    pub fn add_schema(
        &mut self,
        requirement: DefinitionRequirement,
    ) -> Result<(), FormatError> {
        insert_requirement(&mut self.schemas, requirement, "schema")
    }

    /// Register a dialect requirement.
    ///
    /// Duplicate names are rejected rather than silently replacing an earlier
    /// version or required/optional classification.
    pub fn add_dialect(
        &mut self,
        requirement: DefinitionRequirement,
    ) -> Result<(), FormatError> {
        insert_requirement(&mut self.dialects, requirement, "dialect")
    }

    /// Declare the serialized payload's integrity digest.
    pub fn set_integrity(
        &mut self,
        integrity: Option<PayloadIntegrity>,
    ) {
        self.integrity = integrity;
    }

    /// Declare an optional semantic fingerprint.
    ///
    /// The supplied value must be non-empty, lowercase hexadecimal. The
    /// algorithm identity is defined by the surrounding fingerprint contract.
    pub fn set_semantic_fingerprint(
        &mut self,
        fingerprint_hex: Option<String>,
    ) -> Result<(), FormatError> {
        if let Some(ref value) = fingerprint_hex {
            validate_hex(value, "semantic fingerprint")?;
        }

        self.semantic_fingerprint_hex = fingerprint_hex;
        Ok(())
    }

    /// Validate the metadata's internal invariants.
    ///
    /// This function does not check whether a particular reader supports
    /// the declared version, features, schemas, or dialects.
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.format_id != HDLIR_ARTIFACT_FORMAT_ID {
            return Err(FormatError::UnknownFormatId(
                self.format_id.clone(),
            ));
        }

        for feature in &self.required_features {
            validate_qualified_name(feature, "required feature")?;

            if self.optional_features.contains(feature) {
                return Err(FormatError::FeatureClassificationConflict(
                    feature.clone(),
                ));
            }
        }

        for feature in &self.optional_features {
            validate_qualified_name(feature, "optional feature")?;
        }

        validate_requirements(&self.schemas, "schema")?;
        validate_requirements(&self.dialects, "dialect")?;

        if let Some(integrity) = &self.integrity {
            validate_digest(integrity.algorithm, &integrity.digest_hex)?;
        }

        if let Some(fingerprint) = &self.semantic_fingerprint_hex {
            validate_hex(fingerprint, "semantic fingerprint")?;
        }

        Ok(())
    }

    /// Check whether this artifact's format version is readable by `reader`.
    ///
    /// This checks only version-level compatibility. It does not establish
    /// support for required features, schema definitions, dialect semantics,
    /// or payload contents.
    pub fn ensure_version_readable_by(
        &self,
        reader: HdlirVersion,
    ) -> Result<(), FormatError> {
        self.format_version
            .ensure_readable_by(reader)
            .map_err(FormatError::VersionIncompatible)
    }
}

impl Default for ArtifactFormat {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors produced while constructing or validating artifact metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FormatError {
    /// A required identifier is empty or not canonically qualified.
    InvalidIdentifier {
        /// The kind of identifier being validated.
        kind: &'static str,
        /// The rejected value.
        value: String,
    },

    /// The artifact declares an unsupported format identifier.
    UnknownFormatId(String),

    /// A feature was classified as both required and optional.
    FeatureClassificationConflict(String),

    /// A schema or dialect name was registered more than once.
    DuplicateRequirement {
        /// The definition category.
        kind: &'static str,
        /// The duplicated qualified name.
        name: String,
    },

    /// A digest is not valid lowercase hexadecimal.
    InvalidHex {
        /// The value being checked.
        kind: &'static str,
        /// The rejected value.
        value: String,
    },

    /// A digest has the wrong length for its declared algorithm.
    InvalidDigestLength {
        /// The digest algorithm.
        algorithm: &'static str,
        /// Actual number of hexadecimal characters.
        actual: usize,
        /// Required number of hexadecimal characters.
        expected: usize,
    },

    /// The artifact version is incompatible with the reader.
    VersionIncompatible(VersionCompatibilityError),
}

impl fmt::Display for FormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(formatter, "invalid {kind} identifier `{value}`")
            }
            Self::UnknownFormatId(value) => {
                write!(formatter, "unsupported HDLIR format identifier `{value}`")
            }
            Self::FeatureClassificationConflict(value) => write!(
                formatter,
                "feature `{value}` cannot be both required and optional"
            ),
            Self::DuplicateRequirement { kind, name } => {
                write!(formatter, "duplicate {kind} requirement `{name}`")
            }
            Self::InvalidHex { kind, value } => {
                write!(formatter, "invalid lowercase hexadecimal {kind}: `{value}`")
            }
            Self::InvalidDigestLength {
                algorithm,
                actual,
                expected,
            } => write!(
                formatter,
                "invalid {algorithm} digest length: got {actual} hexadecimal characters, expected {expected}"
            ),
            Self::VersionIncompatible(error) => {
                write!(formatter, "incompatible HDLIR artifact version: {error}")
            }
        }
    }
}

impl std::error::Error for FormatError {}

fn insert_requirement(
    destination: &mut BTreeMap<String, DefinitionRequirement>,
    requirement: DefinitionRequirement,
    kind: &'static str,
) -> Result<(), FormatError> {
    validate_qualified_name(requirement.name(), kind)?;

    let name = requirement.name.clone();

    if destination.contains_key(&name) {
        return Err(FormatError::DuplicateRequirement { kind, name });
    }

    destination.insert(name, requirement);
    Ok(())
}

fn validate_requirements(
    requirements: &BTreeMap<String, DefinitionRequirement>,
    kind: &'static str,
) -> Result<(), FormatError> {
    for (key, requirement) in requirements {
        validate_qualified_name(key, kind)?;

        if key != requirement.name() {
            return Err(FormatError::InvalidIdentifier {
                kind,
                value: key.clone(),
            });
        }
    }

    Ok(())
}

/// Validate a canonical qualified identifier.
///
/// Each dot-separated segment starts with a lowercase ASCII letter or
/// underscore. Remaining characters may also contain digits and hyphens.
/// Unicode normalization and case folding are deliberately avoided.
fn validate_qualified_name(
    value: &str,
    kind: &'static str,
) -> Result<(), FormatError> {
    let valid = !value.is_empty()
        && value.split('.').all(|segment| {
            let mut bytes = segment.bytes();

            let Some(first) = bytes.next() else {
                return false;
            };

            (first.is_ascii_lowercase() || first == b'_')
                && bytes.all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'_'
                        || byte == b'-'
                })
        });

    if valid {
        Ok(())
    } else {
        Err(FormatError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
        })
    }
}

fn validate_hex(value: &str, kind: &'static str) -> Result<(), FormatError> {
    if !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(FormatError::InvalidHex {
            kind,
            value: value.to_owned(),
        })
    }
}

fn validate_digest(
    algorithm: IntegrityAlgorithm,
    value: &str,
) -> Result<(), FormatError> {
    validate_hex(value, "payload digest")?;

    let expected = match algorithm {
        IntegrityAlgorithm::Sha256 => 64,
    };

    if value.len() != expected {
        return Err(FormatError::InvalidDigestLength {
            algorithm: algorithm.as_str(),
            actual: value.len(),
            expected,
        });
    }

    Ok(())
}

/// Serde adapter preserving `HdlirVersion`'s canonical string representation.
///
/// `HdlirVersion` remains owned by `version.rs`; this module does not
/// introduce a competing version type or change the version API.
mod version_serde {
    use super::*;

    pub fn serialize<S>(
        version: &HdlirVersion,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&version.to_string())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HdlirVersion, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        HdlirVersion::from_str(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_envelope_uses_current_format_version() {
        let format = ArtifactFormat::new();

        assert_eq!(format.format_id(), HDLIR_ARTIFACT_FORMAT_ID);
        assert_eq!(format.format_version(), HDLIR_FORMAT_VERSION);
        assert_eq!(
            format.ordering(),
            CanonicalOrdering::CanonicalIdentifiers
        );
        assert!(format.validate().is_ok());
    }

    #[test]
    fn requirements_have_deterministic_order() {
        let mut format = ArtifactFormat::new();

        format
            .add_schema(
                DefinitionRequirement::new(
                    "hardware.signal",
                    HdlirVersion::new(1, 0, 0),
                    true,
                )
                .unwrap(),
            )
            .unwrap();

        format
            .add_schema(
                DefinitionRequirement::new(
                    "hardware.module",
                    HdlirVersion::new(1, 0, 0),
                    true,
                )
                .unwrap(),
            )
            .unwrap();

        let names: Vec<_> = format.schemas().keys().map(String::as_str).collect();

        assert_eq!(names, vec!["hardware.module", "hardware.signal"]);
    }

    #[test]
    fn duplicate_schema_requirement_is_rejected() {
        let mut format = ArtifactFormat::new();
        let requirement = DefinitionRequirement::new(
            "hardware.signal",
            HdlirVersion::new(1, 0, 0),
            true,
        )
        .unwrap();

        format.add_schema(requirement.clone()).unwrap();

        assert!(matches!(
            format.add_schema(requirement),
            Err(FormatError::DuplicateRequirement {
                kind: "schema",
                ..
            })
        ));
    }

    #[test]
    fn feature_cannot_be_required_and_optional() {
        let mut format = ArtifactFormat::new();

        format.require_feature("hardware.memory").unwrap();

        assert!(matches!(
            format.declare_optional_feature("hardware.memory"),
            Err(FormatError::FeatureClassificationConflict(_))
        ));
    }

    #[test]
    fn identifiers_are_validated() {
        assert!(DefinitionRequirement::new(
            "hardware.sequential",
            HdlirVersion::new(1, 0, 0),
            true,
        )
        .is_ok());

        assert!(DefinitionRequirement::new(
            "Hardware.Sequential",
            HdlirVersion::new(1, 0, 0),
            true,
        )
        .is_err());

        assert!(DefinitionRequirement::new(
            "hardware..sequential",
            HdlirVersion::new(1, 0, 0),
            true,
        )
        .is_err());
    }

    #[test]
    fn sha256_digest_requires_exact_lowercase_hex_length() {
        let valid = "a".repeat(64);

        assert!(PayloadIntegrity::new(
            IntegrityAlgorithm::Sha256,
            valid
        )
        .is_ok());

        assert!(PayloadIntegrity::new(
            IntegrityAlgorithm::Sha256,
            "abc"
        )
        .is_err());

        assert!(PayloadIntegrity::new(
            IntegrityAlgorithm::Sha256,
            "A".repeat(64)
        )
        .is_err());
    }

    #[test]
    fn version_compatibility_is_delegated_to_version_contract() {
        let mut format = ArtifactFormat::new();
        format.set_format_version(HdlirVersion::new(2, 0, 0));

        assert!(matches!(
            format.ensure_version_readable_by(HdlirVersion::new(1, 0, 0)),
            Err(FormatError::VersionIncompatible(
                VersionCompatibilityError::MajorVersionMismatch {
                    artifact_major: 2,
                    reader_major: 1,
                }
            ))
        ));
    }

    #[test]
    fn serde_round_trip_preserves_metadata() {
        let mut format = ArtifactFormat::new();

        format.require_feature("hardware.memory").unwrap();

        format
            .add_dialect(
                DefinitionRequirement::new(
                    "hardware.structural",
                    HdlirVersion::new(1, 0, 0),
                    true,
                )
                .unwrap(),
            )
            .unwrap();

        let encoded = serde_json::to_string(&format).unwrap();
        let decoded: ArtifactFormat = serde_json::from_str(&encoded).unwrap();

        assert_eq!(format, decoded);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn unknown_envelope_fields_are_rejected_by_serde() {
        let input = r#"{
            "format_id":"zamani.hdlir",
            "format_version":"1.0.0",
            "ordering":"canonical_identifiers",
            "required_features":[],
            "optional_features":[],
            "schemas":{},
            "dialects":{},
            "integrity":null,
            "semantic_fingerprint_hex":null,
            "unrecognized_required_semantics":true
        }"#;

        assert!(serde_json::from_str::<ArtifactFormat>(input).is_err());
    }

    #[test]
    fn invalid_feature_metadata_is_rejected() {
        let mut format = ArtifactFormat::new();
        format.required_features.insert("Invalid Feature".to_owned());

        assert!(format.validate().is_err());
    }

    #[test]
    fn semantic_fingerprint_is_not_payload_integrity() {
        let mut format = ArtifactFormat::new();

        format
            .set_semantic_fingerprint(Some("b".repeat(64)))
            .unwrap();

        assert!(format.semantic_fingerprint_hex().is_some());
        assert!(format.integrity().is_none());
    }
}
