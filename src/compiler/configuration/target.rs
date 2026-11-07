
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Target Configuration Contracts.
//!
//! Path: src/compiler/configuration/target.rs
//! Edition: 2021
//! Minimum Rust version: 1.97.
//!
//! # Purpose
//!
//! Defines extensible compiler-facing target descriptors.
//!
//! These descriptors represent compilation target intent and metadata.
//! They do not establish physical hardware availability or backend support.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - Target identifiers.
//! - Target roles.
//! - Artifact-format identifiers.
//! - Extensible target metadata.
//! - Target descriptor construction.
//! - Structural descriptor validation.
//! - Legacy CompilationTarget adaptation.
//!
//! Other authorities:
//! - Source target syntax: grammar/compile/target.g4.
//! - Target-selection policy: grammar/compile/target-selection.g4.
//! - Source semantics: grammar/specification/semantic-model.md.
//! - Legacy target identity: crate::compiler_types.
//! - Actual backend target descriptions: crate::backend::target.
//! - Backend capabilities: crate::backend::capabilities.
//! - Compatibility: crate::backend::compatibility.
//! - Resource planning: crate::compiler::planning.
//! - Physical execution: crate::runtime.
//!
//! # POCO-REAF
//!
//! Target identities are open-world.
//!
//! This module imposes no universal maximum on:
//! CPUs, cores, GPUs, FPGAs, QPUs, nodes, devices, memory,
//! register widths, tensor dimensions, or computational domains.
//!
//! Unknown information remains unknown.
//!
//! A valid descriptor does not imply feasibility.
//!
//! # Determinism
//!
//! Metadata maps use BTreeMap.
//! Artifact formats are deduplicated deterministically.
//! Duplicate property definitions are rejected.
//!
//! # Security
//!
//! All externally supplied metadata is validated.
//! No filesystem, network, environment, or hardware access occurs.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler_types::CompilationTarget;

/// Version of the compiler-facing target descriptor API.
///
/// Independent of language, grammar, IR, compiler and backend versions.
pub const TARGET_DESCRIPTOR_API_VERSION: u16 = 2;

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), TargetError> {
    if value.trim().is_empty() {
        return Err(TargetError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(TargetError::ControlCharacter { field });
    }

    Ok(())
}

fn validate_identifier(value: &str) -> Result<(), TargetError> {
    if value.is_empty() {
        return Err(TargetError::InvalidIdentifier {
            value: value.to_owned(),
            reason: "identifier cannot be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(TargetError::InvalidIdentifier {
            value: value.to_owned(),
            reason: "surrounding whitespace is not permitted".to_owned(),
        });
    }

    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b':' | b'/' | b'+'
            )
    }) {
        return Err(TargetError::InvalidIdentifier {
            value: value.to_owned(),
            reason: "identifier contains unsupported characters".to_owned(),
        });
    }

    Ok(())
}

fn validate_optional_text(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), TargetError> {
    if let Some(value) = value {
        validate_text(field, value)?;
    }

    Ok(())
}

/// Validates a decimal numeric representation without imposing
/// a machine-integer width.
fn validate_decimal(value: &str) -> Result<(), TargetError> {
    if value.is_empty() {
        return Err(TargetError::InvalidDecimal {
            value: value.to_owned(),
        });
    }

    let unsigned = value
        .strip_prefix('-')
        .or_else(|| value.strip_prefix('+'))
        .unwrap_or(value);

    let mut parts = unsigned.split('.');

    let integer = parts.next().unwrap_or_default();
    let fraction = parts.next();

    let valid_integer =
        !integer.is_empty()
            && integer.bytes().all(|b| b.is_ascii_digit());

    let valid_fraction = fraction.is_none_or(|fraction| {
        !fraction.is_empty()
            && fraction.bytes().all(|b| b.is_ascii_digit())
    });

    if parts.next().is_some()
        || !valid_integer
        || !valid_fraction
    {
        return Err(TargetError::InvalidDecimal {
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// TARGET IDENTITY
// ============================================================================

/// Stable, extensible target identifier.
///
/// Examples:
///
/// x86_64-linux
/// aarch64
/// wasm32
/// riscv64
/// quantum:provider:processor
/// photonic:provider:device
/// vendor:accelerator:model
///
/// The examples are not a closed target catalogue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId(String);

impl TargetId {
    pub fn new(value: impl Into<String>) -> Result<Self, TargetError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for TargetId {
    type Error = TargetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TargetId {
    type Error = TargetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// TARGET ROLE
// ============================================================================

/// Open-world descriptive target role.
///
/// Examples:
///
/// execution
/// simulation
/// emulation
/// hardware-description
/// intermediate-representation
/// portable-ir
/// deployment-package
///
/// Roles are descriptive metadata, not capabilities.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetRole(String);

impl TargetRole {
    pub fn new(value: impl Into<String>) -> Result<Self, TargetError> {
        let value = value.into();

        validate_text("target role", &value)?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TargetRole {
    fn default() -> Self {
        Self("execution".to_owned())
    }
}

impl fmt::Display for TargetRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// ARTIFACT FORMAT
// ============================================================================

/// Extensible artifact-format identifier.
///
/// Examples include:
///
/// elf
/// wasm
/// llvm-ir
/// qasm
/// verilog
/// systemverilog
/// bitstream
/// portable-ir
/// deployment-bundle
///
/// Declaring a format does not establish backend availability.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactFormat(String);

impl ArtifactFormat {
    pub fn new(value: impl Into<String>) -> Result<Self, TargetError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ArtifactFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// EXTENSIBLE PROPERTY VALUES
// ============================================================================

/// Extensible metadata values.
///
/// Numeric values are represented without imposing a universal
/// fixed-width numeric representation where a textual decimal
/// representation is appropriate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetProperty {
    Text(String),
    Boolean(bool),
    Integer(i128),
    Unsigned(u128),
    Decimal(String),
    Identifier(String),
    List(Vec<TargetProperty>),
    Map(BTreeMap<String, TargetProperty>),
}

impl TargetProperty {
    pub fn validate(&self) -> Result<(), TargetError> {
        match self {
            Self::Text(value) => {
                if value.chars().any(char::is_control) {
                    return Err(TargetError::ControlCharacter {
                        field: "property value",
                    });
                }

                Ok(())
            }

            Self::Decimal(value) => validate_decimal(value),

            Self::Identifier(value) => {
                validate_identifier(value)
            }

            Self::Boolean(_)
            | Self::Integer(_)
            | Self::Unsigned(_) => Ok(()),

            Self::List(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_identifier(key)?;
                    value.validate()?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// TARGET DESCRIPTOR
// ============================================================================

/// Immutable compiler-facing target descriptor.
///
/// This is a description of compilation target intent.
///
/// It is NOT:
///
/// - A physical hardware inventory.
/// - A capability profile.
/// - A backend implementation.
/// - A compatibility result.
/// - A resource allocation.
/// - A runtime execution plan.
///
/// Architecture and platform metadata are descriptive only.
///
/// Actual target evidence belongs to crate::backend::target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetDescriptor {
    id: TargetId,
    display_name: String,
    role: TargetRole,

    architecture: Option<String>,
    vendor: Option<String>,
    operating_system: Option<String>,
    environment: Option<String>,
    abi: Option<String>,

    target_triple: Option<String>,
    data_layout: Option<String>,

    artifact_formats: Vec<ArtifactFormat>,

    properties: BTreeMap<String, TargetProperty>,
    extensions: BTreeMap<String, TargetProperty>,
}

impl TargetDescriptor {
    /// Starts construction of an immutable target descriptor.
    pub fn builder(
        id: TargetId,
        display_name: impl Into<String>,
    ) -> TargetDescriptorBuilder {
        TargetDescriptorBuilder::new(id, display_name)
    }

    #[must_use]
    pub fn id(&self) -> &TargetId {
        &self.id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn role(&self) -> &TargetRole {
        &self.role
    }

    #[must_use]
    pub fn architecture(&self) -> Option<&str> {
        self.architecture.as_deref()
    }

    #[must_use]
    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }

    #[must_use]
    pub fn operating_system(&self) -> Option<&str> {
        self.operating_system.as_deref()
    }

    #[must_use]
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    #[must_use]
    pub fn abi(&self) -> Option<&str> {
        self.abi.as_deref()
    }

    #[must_use]
    pub fn target_triple(&self) -> Option<&str> {
        self.target_triple.as_deref()
    }

    #[must_use]
    pub fn data_layout(&self) -> Option<&str> {
        self.data_layout.as_deref()
    }

    #[must_use]
    pub fn artifact_formats(&self) -> &[ArtifactFormat] {
        &self.artifact_formats
    }

    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, TargetProperty> {
        &self.properties
    }

    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<String, TargetProperty> {
        &self.extensions
    }

    #[must_use]
    pub fn property(&self, key: &str) -> Option<&TargetProperty> {
        self.properties.get(key)
    }

    #[must_use]
    pub fn extension(&self, key: &str) -> Option<&TargetProperty> {
        self.extensions.get(key)
    }

    #[must_use]
    pub fn declares_artifact_format(
        &self,
        format: &ArtifactFormat,
    ) -> bool {
        self.artifact_formats.contains(format)
    }

    /// Validates all descriptor metadata.
    pub fn validate(&self) -> Result<(), TargetError> {
        validate_identifier(self.id.as_str())?;
        validate_text("display name", &self.display_name)?;
        validate_text("target role", self.role.as_str())?;

        for (field, value) in [
            ("architecture", self.architecture.as_deref()),
            ("vendor", self.vendor.as_deref()),
            ("operating system", self.operating_system.as_deref()),
            ("environment", self.environment.as_deref()),
            ("ABI", self.abi.as_deref()),
            ("target triple", self.target_triple.as_deref()),
            ("data layout", self.data_layout.as_deref()),
        ] {
            validate_optional_text(field, value)?;
        }

        if let Some(triple) = &self.target_triple {
            if triple.chars().any(char::is_whitespace) {
                return Err(TargetError::InvalidTargetTriple {
                    value: triple.clone(),
                });
            }
        }

        for format in &self.artifact_formats {
            validate_identifier(format.as_str())?;
        }

        for (key, value) in &self.properties {
            validate_identifier(key)?;
            value.validate()?;
        }

        for (key, value) in &self.extensions {
            validate_identifier(key)?;

            if !key.contains(':') {
                return Err(TargetError::UnqualifiedExtension {
                    key: key.clone(),
                });
            }

            value.validate()?;
        }

        Ok(())
    }

    /// Adapts the existing legacy CompilationTarget.
    ///
    /// This is a compatibility adapter, not an authoritative target
    /// registry or backend capability declaration.
    pub fn from_legacy(
        target: &CompilationTarget,
    ) -> Result<Self, TargetError> {
        let builder = match target {
            CompilationTarget::X86_64Linux => {
                Self::builder(
                    TargetId::new("x86_64-linux")?,
                    "x86-64 Linux",
                )
                .architecture("x86_64")
                .operating_system("linux")
            }

            CompilationTarget::Arm64 => {
                Self::builder(
                    TargetId::new("aarch64")?,
                    "AArch64",
                )
                .architecture("aarch64")
            }

            CompilationTarget::Wasm32 => {
                Self::builder(
                    TargetId::new("wasm32")?,
                    "WebAssembly 32-bit",
                )
                .architecture("wasm32")
                .artifact_format(ArtifactFormat::new("wasm")?)
            }

            CompilationTarget::RiscV => {
                Self::builder(
                    TargetId::new("riscv")?,
                    "RISC-V",
                )
                .architecture("riscv")
            }

            CompilationTarget::QASM => {
                Self::builder(
                    TargetId::new("qasm")?,
                    "OpenQASM",
                )
                .role(TargetRole::new("quantum")?)
                .artifact_format(ArtifactFormat::new("qasm")?)
            }

            CompilationTarget::NanoControl => {
                Self::builder(
                    TargetId::new("nano-control")?,
                    "NanoControl",
                )
            }

            CompilationTarget::MTSBytecode => {
                Self::builder(
                    TargetId::new("mts-bytecode")?,
                    "MTS Bytecode",
                )
                .role(
                    TargetRole::new("intermediate-representation")?,
                )
                .artifact_format(
                    ArtifactFormat::new("mts-bytecode")?,
                )
            }

            CompilationTarget::LLVMIR => {
                Self::builder(
                    TargetId::new("llvm-ir")?,
                    "LLVM Intermediate Representation",
                )
                .role(
                    TargetRole::new("intermediate-representation")?,
                )
                .artifact_format(ArtifactFormat::new("llvm-ir")?)
            }

            CompilationTarget::UniversalIRExport(name) => {
                let normalized = name.trim().to_ascii_lowercase();

                if normalized.is_empty() {
                    return Err(TargetError::InvalidLegacyTarget {
                        reason: "universal IR export name is empty".to_owned(),
                    });
                }

                Self::builder(
                    TargetId::new(format!("ir-export:{normalized}"))?,
                    format!("Universal IR Export: {name}"),
                )
                .role(
                    TargetRole::new("intermediate-representation")?,
                )
                .artifact_format(
                    ArtifactFormat::new(&normalized)?,
                )
            }
        };

        builder.build()
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for validated immutable target descriptors.
#[derive(Debug, Clone)]
pub struct TargetDescriptorBuilder {
    id: TargetId,
    display_name: String,
    role: TargetRole,

    architecture: Option<String>,
    vendor: Option<String>,
    operating_system: Option<String>,
    environment: Option<String>,
    abi: Option<String>,

    target_triple: Option<String>,
    data_layout: Option<String>,

    artifact_formats: Vec<ArtifactFormat>,

    properties: Vec<(String, TargetProperty)>,
    extensions: Vec<(String, TargetProperty)>,

    api_version: u16,
}

impl TargetDescriptorBuilder {
    pub fn new(
        id: TargetId,
        display_name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            role: TargetRole::default(),

            architecture: None,
            vendor: None,
            operating_system: None,
            environment: None,
            abi: None,

            target_triple: None,
            data_layout: None,

            artifact_formats: Vec::new(),

            properties: Vec::new(),
            extensions: Vec::new(),

            api_version: TARGET_DESCRIPTOR_API_VERSION,
        }
    }

    pub fn api_version(mut self, version: u16) -> Self {
        self.api_version = version;
        self
    }

    pub fn role(mut self, role: TargetRole) -> Self {
        self.role = role;
        self
    }

    pub fn architecture(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.architecture = Some(value.into());
        self
    }

    pub fn vendor(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.vendor = Some(value.into());
        self
    }

    pub fn operating_system(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.operating_system = Some(value.into());
        self
    }

    pub fn environment(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn abi(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.abi = Some(value.into());
        self
    }

    pub fn target_triple(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.target_triple = Some(value.into());
        self
    }

    pub fn data_layout(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.data_layout = Some(value.into());
        self
    }

    pub fn artifact_format(
        mut self,
        format: ArtifactFormat,
    ) -> Self {
        self.artifact_formats.push(format);
        self
    }

    pub fn property(
        mut self,
        key: impl Into<String>,
        value: TargetProperty,
    ) -> Self {
        self.properties.push((key.into(), value));
        self
    }

    pub fn extension(
        mut self,
        key: impl Into<String>,
        value: TargetProperty,
    ) -> Self {
        self.extensions.push((key.into(), value));
        self
    }

    pub fn build(self) -> Result<TargetDescriptor, TargetError> {
        if self.api_version != TARGET_DESCRIPTOR_API_VERSION {
            return Err(TargetError::UnsupportedApiVersion {
                supplied: self.api_version,
                supported: TARGET_DESCRIPTOR_API_VERSION,
            });
        }

        let properties = collect_unique_properties(self.properties)?;
        let extensions = collect_unique_properties(self.extensions)?;

        let mut seen_formats = BTreeSet::new();

        let artifact_formats = self
            .artifact_formats
            .into_iter()
            .filter(|format| seen_formats.insert(format.clone()))
            .collect();

        let descriptor = TargetDescriptor {
            id: self.id,
            display_name: self.display_name,
            role: self.role,

            architecture: self.architecture,
            vendor: self.vendor,
            operating_system: self.operating_system,
            environment: self.environment,
            abi: self.abi,

            target_triple: self.target_triple,
            data_layout: self.data_layout,

            artifact_formats,

            properties,
            extensions,
        };

        descriptor.validate()?;

        Ok(descriptor)
    }
}

fn collect_unique_properties(
    entries: Vec<(String, TargetProperty)>,
) -> Result<BTreeMap<String, TargetProperty>, TargetError> {
    let mut result = BTreeMap::new();

    for (key, value) in entries {
        validate_identifier(&key)?;
        value.validate()?;

        if result.insert(key.clone(), value).is_some() {
            return Err(TargetError::DuplicateProperty { key });
        }
    }

    Ok(result)
}

// ============================================================================
// ERRORS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    EmptyField {
        field: &'static str,
    },

    ControlCharacter {
        field: &'static str,
    },

    InvalidIdentifier {
        value: String,
        reason: String,
    },

    InvalidDecimal {
        value: String,
    },

    InvalidTargetTriple {
        value: String,
    },

    DuplicateProperty {
        key: String,
    },

    UnqualifiedExtension {
        key: String,
    },

    UnsupportedApiVersion {
        supplied: u16,
        supported: u16,
    },

    InvalidLegacyTarget {
        reason: String,
    },
}

impl fmt::Display for TargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(f, "{field} cannot contain control characters")
            }

            Self::InvalidIdentifier { value, reason } => {
                write!(f, "invalid identifier `{value}`: {reason}")
            }

            Self::InvalidDecimal { value } => {
                write!(f, "invalid decimal representation `{value}`")
            }

            Self::InvalidTargetTriple { value } => {
                write!(f, "invalid target triple `{value}`")
            }

            Self::DuplicateProperty { key } => {
                write!(f, "duplicate target property `{key}`")
            }

            Self::UnqualifiedExtension { key } => {
                write!(
                    f,
                    "extension property `{key}` must use a qualified namespace"
                )
            }

            Self::UnsupportedApiVersion {
                supplied,
                supported,
            } => {
                write!(
                    f,
                    "unsupported target descriptor API version \
                     {supplied}; supported version is {supported}"
                )
            }

            Self::InvalidLegacyTarget { reason } => {
                write!(f, "invalid legacy target: {reason}")
            }
        }
    }
}

impl Error for TargetError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_open_world_target_identifiers() {
        for value in [
            "x86_64-linux",
            "aarch64",
            "quantum:provider:processor",
            "photonic:provider:device",
            "future:architecture:model",
        ] {
            assert!(TargetId::new(value).is_ok());
        }
    }

    #[test]
    fn rejects_invalid_target_identifiers() {
        assert!(TargetId::new("").is_err());
        assert!(TargetId::new("ARM64").is_err());
        assert!(TargetId::new(" invalid").is_err());
        assert!(TargetId::new("target\nname").is_err());
    }

    #[test]
    fn identifiers_have_no_arbitrary_length_ceiling() {
        let identifier = format!("future-{}", "x".repeat(10_000));

        assert!(TargetId::new(identifier).is_ok());
    }

    #[test]
    fn target_role_is_extensible() {
        let role = TargetRole::new("future-computation-model")
            .expect("valid role");

        assert_eq!(role.as_str(), "future-computation-model");
    }

    #[test]
    fn descriptor_preserves_unknown_information() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("future:target").unwrap(),
            "Future Target",
        )
        .build()
        .unwrap();

        assert_eq!(descriptor.architecture(), None);
        assert_eq!(descriptor.vendor(), None);
        assert_eq!(descriptor.operating_system(), None);
        assert_eq!(descriptor.abi(), None);
        assert_eq!(descriptor.target_triple(), None);
    }

    #[test]
    fn descriptor_separates_architecture_and_operating_system() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("aarch64-linux").unwrap(),
            "AArch64 Linux",
        )
        .architecture("aarch64")
        .operating_system("linux")
        .build()
        .unwrap();

        assert_eq!(descriptor.architecture(), Some("aarch64"));
        assert_eq!(descriptor.operating_system(), Some("linux"));
    }

    #[test]
    fn descriptor_supports_future_artifact_formats() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("future-target").unwrap(),
            "Future Target",
        )
        .artifact_format(
            ArtifactFormat::new("future:artifact").unwrap(),
        )
        .build()
        .unwrap();

        assert_eq!(descriptor.artifact_formats().len(), 1);
    }

    #[test]
    fn duplicate_artifact_formats_are_deduplicated() {
        let format = ArtifactFormat::new("wasm").unwrap();

        let descriptor = TargetDescriptor::builder(
            TargetId::new("wasm-target").unwrap(),
            "WebAssembly",
        )
        .artifact_format(format.clone())
        .artifact_format(format)
        .build()
        .unwrap();

        assert_eq!(descriptor.artifact_formats().len(), 1);
    }

    #[test]
    fn duplicate_properties_are_rejected() {
        let result = TargetDescriptor::builder(
            TargetId::new("test-target").unwrap(),
            "Test",
        )
        .property("architecture", TargetProperty::Text("a".into()))
        .property("architecture", TargetProperty::Text("b".into()))
        .build();

        assert!(matches!(
            result,
            Err(TargetError::DuplicateProperty { .. })
        ));
    }

    #[test]
    fn extension_properties_require_namespaces() {
        let result = TargetDescriptor::builder(
            TargetId::new("test-target").unwrap(),
            "Test",
        )
        .extension("revision", TargetProperty::Text("1".into()))
        .build();

        assert!(matches!(
            result,
            Err(TargetError::UnqualifiedExtension { .. })
        ));
    }

    #[test]
    fn arbitrary_precision_decimal_metadata_is_validated() {
        assert!(TargetProperty::Decimal(
            "999999999999999999999999999999999999999999".into()
        )
        .validate()
        .is_ok());

        assert!(TargetProperty::Decimal("12.75".into()).validate().is_ok());

        assert!(TargetProperty::Decimal("12.invalid".into()).validate().is_err());
    }

    #[test]
    fn nested_properties_are_validated() {
        let mut values = BTreeMap::new();

        values.insert(
            "capacity".to_owned(),
            TargetProperty::Decimal("999999999999999999999".into()),
        );

        let property = TargetProperty::Map(values);

        assert!(property.validate().is_ok());
    }

    #[test]
    fn invalid_api_versions_are_rejected() {
        let result = TargetDescriptor::builder(
            TargetId::new("test-target").unwrap(),
            "Test",
        )
        .api_version(999)
        .build();

        assert!(matches!(
            result,
            Err(TargetError::UnsupportedApiVersion { .. })
        ));
    }

    #[test]
    fn target_triples_reject_whitespace() {
        let result = TargetDescriptor::builder(
            TargetId::new("test-target").unwrap(),
            "Test",
        )
        .target_triple("invalid target triple")
        .build();

        assert!(matches!(
            result,
            Err(TargetError::InvalidTargetTriple { .. })
        ));
    }

    #[test]
    fn descriptor_does_not_claim_physical_feasibility() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("quantum:future:processor").unwrap(),
            "Future Quantum Processor",
        )
        .role(TargetRole::new("quantum").unwrap())
        .build()
        .unwrap();

        // Structural validity is not capability evidence.
        assert_eq!(descriptor.role().as_str(), "quantum");
    }
}
