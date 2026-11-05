
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Target Descriptors
//!
//! Path:
//!     src/compiler/configuration/target.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ================================================================
//! PURPOSE
//! ================================================================
//!
//! Defines the target-independent description of compilation targets.
//!
//! A target descriptor describes an execution or artifact environment
//! without making assumptions about the resources physically available.
//!
//! It provides:
//!
//! - Stable target identity.
//! - Architecture identification.
//! - Vendor identification.
//! - Operating-system identification.
//! - Execution-environment identification.
//! - ABI metadata.
//! - Data-layout metadata.
//! - Target triple metadata.
//! - Artifact-format declarations.
//! - Extensible target properties.
//! - Validation.
//! - Legacy CompilationTarget adaptation.
//!
//! ================================================================
//! OWNERSHIP
//! ================================================================
//!
//! This module owns target descriptors and their structural validation.
//!
//! It does not own:
//!
//! - Hardware discovery.
//! - Hardware capability measurement.
//! - Physical resource allocation.
//! - Backend selection.
//! - Backend implementation.
//! - Runtime scheduling.
//! - Device placement.
//! - Quantum routing.
//! - Quantum error correction.
//! - HDL synthesis.
//! - Semantic analysis.
//! - Program requirements.
//! - Program preferences.
//! - IR definitions.
//!
//! ================================================================
//! POCO-REAF
//! ================================================================
//!
//! Target descriptions are open-ended.
//!
//! No universal maximum is imposed on:
//!
//! - CPUs.
//! - Cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Registers.
//! - Register width.
//! - Tensor rank.
//! - Vector width.
//! - Quantum resources.
//! - Hardware resources.
//!
//! Actual feasibility belongs to capability negotiation and resource
//! planning.
//!
//! An absent property means unknown or unspecified, not unlimited.
//!
//! ================================================================
//! INTEGRATION
//! ================================================================
//!
//! Existing:
//!     crate::compiler_types::CompilationTarget
//!
//! Future:
//!     configuration/registry.rs
//!     configuration/capabilities.rs
//!     configuration/requirements.rs
//!     backends/registry.rs
//!     backends/selection.rs
//!     planning/target_discovery.rs
//!     planning/capability_matching.rs
//!     artifacts/artifact.rs
//!
//! The target descriptor is deliberately independent of these future
//! modules so its contract can be frozen first.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler_types::CompilationTarget;

// ============================================================================
// VERSION
// ============================================================================

/// Public target-descriptor API version.
///
/// This is independent of the Zamani language, compiler, IR and backend
/// versions.
pub const TARGET_DESCRIPTOR_API_VERSION: u16 = 1;

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates free-form descriptor text.
///
/// Empty values and control characters are rejected.
///
/// No arbitrary string-length limit is imposed.
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

/// Validates a stable target identifier.
///
/// Identifiers use a portable, case-sensitive ASCII representation.
///
/// Allowed characters:
///
/// - Lowercase ASCII letters.
/// - Digits.
/// - Hyphens.
/// - Underscores.
/// - Periods.
/// - Colons.
/// - Forward slashes.
/// - Plus signs.
///
/// This supports vendor-qualified and extension-qualified identifiers.
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
            reason: "leading or trailing whitespace is forbidden".to_owned(),
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
            reason:
                "identifier contains unsupported characters; \
                 use lowercase ASCII letters, digits, \
                 hyphens, underscores, periods, colons, \
                 slashes or plus signs"
                    .to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// TARGET ID
// ============================================================================

/// Stable, open-ended target identifier.
///
/// Examples:
///
/// - `x86_64-unknown-linux-gnu`
/// - `aarch64-unknown-linux-gnu`
/// - `wasm32-unknown-unknown`
/// - `riscv64-unknown-linux-gnu`
/// - `vendor:accelerator:model`
/// - `quantum:provider:processor`
/// - `photonic:provider:device`
///
/// Identifiers are not interpreted as hardware capabilities.
///
/// Two descriptors with the same identifier refer to the same logical
/// target identity and must not silently overwrite each other in a registry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId(String);

impl TargetId {
    /// Creates a validated target identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, TargetError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
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

/// Describes the role of a target.
///
/// The string representation is intentionally extensible.
///
/// Examples:
///
/// - `execution`
/// - `portable-ir`
/// - `intermediate-representation`
/// - `hardware-description`
/// - `simulation`
/// - `emulation`
/// - `deployment-package`
///
/// This is descriptive metadata, not proof that an execution backend exists.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetRole(String);

impl TargetRole {
    /// Creates a target role.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, TargetError> {
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

// ============================================================================
// ARTIFACT FORMAT
// ============================================================================

/// Identifies an output artifact format.
///
/// Examples:
///
/// - `elf`
/// - `macho`
/// - `coff`
/// - `wasm`
/// - `llvm-ir`
/// - `qasm`
/// - `verilog`
/// - `systemverilog`
/// - `bitstream`
/// - `portable-ir`
/// - `deployment-bundle`
///
/// Artifact formats are open-ended.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactFormat(String);

impl ArtifactFormat {
    /// Creates a validated artifact format identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, TargetError> {
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// TARGET PROPERTY
// ============================================================================

/// An extensible target metadata value.
///
/// Properties describe target metadata.
///
/// They do not independently establish hardware capability or guarantee
/// physical availability.
///
/// Structured capability contracts belong to the capability subsystem.
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
    /// Validates the property recursively.
    pub fn validate(&self) -> Result<(), TargetError> {
        match self {
            Self::Text(value) | Self::Decimal(value) => {
                validate_text("property value", value)
            }

            Self::Identifier(value) => {
                validate_identifier(value)
            }

            Self::Boolean(_) | Self::Integer(_) | Self::Unsigned(_) => {
                Ok(())
            }

            Self::List(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_text("property key", key)?;
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

/// Immutable description of a compilation target.
///
/// The descriptor separates:
///
/// - Logical target identity.
/// - CPU or accelerator architecture.
/// - Vendor.
/// - Operating system.
/// - ABI.
/// - Execution environment.
/// - Target triple.
/// - Data layout.
/// - Artifact formats.
/// - Extensible metadata.
///
/// Optional values represent information that is unknown or unspecified.
///
/// An unknown field must never be interpreted as a wildcard or as proof
/// of compatibility.
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
    /// Creates a descriptor builder.
    pub fn builder(
        id: TargetId,
        display_name: impl Into<String>,
    ) -> TargetDescriptorBuilder {
        TargetDescriptorBuilder::new(id, display_name)
    }

    /// Returns the stable target identifier.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        &self.id
    }

    /// Returns the display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the target role.
    #[must_use]
    pub fn role(&self) -> &TargetRole {
        &self.role
    }

    /// Returns the architecture identifier, if known.
    #[must_use]
    pub fn architecture(&self) -> Option<&str> {
        self.architecture.as_deref()
    }

    /// Returns the vendor identifier, if known.
    #[must_use]
    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }

    /// Returns the operating-system identifier, if known.
    #[must_use]
    pub fn operating_system(&self) -> Option<&str> {
        self.operating_system.as_deref()
    }

    /// Returns the execution environment, if known.
    #[must_use]
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    /// Returns the ABI identifier, if known.
    #[must_use]
    pub fn abi(&self) -> Option<&str> {
        self.abi.as_deref()
    }

    /// Returns the target triple, if supplied.
    ///
    /// The triple is opaque metadata.
    ///
    /// This module deliberately does not parse triples using a closed
    /// architecture or operating-system inventory.
    #[must_use]
    pub fn target_triple(&self) -> Option<&str> {
        self.target_triple.as_deref()
    }

    /// Returns the target data-layout description, if supplied.
    ///
    /// Data-layout syntax is interpreted by the appropriate IR/backend
    /// integration layer.
    #[must_use]
    pub fn data_layout(&self) -> Option<&str> {
        self.data_layout.as_deref()
    }

    /// Returns supported declared artifact formats.
    #[must_use]
    pub fn artifact_formats(&self) -> &[ArtifactFormat] {
        &self.artifact_formats
    }

    /// Returns target properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, TargetProperty> {
        &self.properties
    }

    /// Returns extension properties.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<String, TargetProperty> {
        &self.extensions
    }

    /// Retrieves a property by name.
    #[must_use]
    pub fn property(&self, key: &str) -> Option<&TargetProperty> {
        self.properties.get(key)
    }

    /// Retrieves an extension property.
    #[must_use]
    pub fn extension(&self, key: &str) -> Option<&TargetProperty> {
        self.extensions.get(key)
    }

    /// Checks whether an artifact format is declared.
    ///
    /// This checks descriptor metadata only. It does not prove that a
    /// registered backend can generate that artifact.
    #[must_use]
    pub fn declares_artifact_format(
        &self,
        format: &ArtifactFormat,
    ) -> bool {
        self.artifact_formats.contains(format)
    }

    /// Validates the complete descriptor.
    pub fn validate(&self) -> Result<(), TargetError> {
        validate_identifier(self.id.as_str())?;

        validate_text("display name", &self.display_name)?;

        validate_text("target role", self.role.as_str())?;

        validate_optional_text(
            "architecture",
            self.architecture.as_deref(),
        )?;

        validate_optional_text(
            "vendor",
            self.vendor.as_deref(),
        )?;

        validate_optional_text(
            "operating system",
            self.operating_system.as_deref(),
        )?;

        validate_optional_text(
            "execution environment",
            self.environment.as_deref(),
        )?;

        validate_optional_text(
            "ABI",
            self.abi.as_deref(),
        )?;

        validate_optional_text(
            "target triple",
            self.target_triple.as_deref(),
        )?;

        validate_optional_text(
            "data layout",
            self.data_layout.as_deref(),
        )?;

        for format in &self.artifact_formats {
            validate_identifier(format.as_str())?;
        }

        for (key, value) in &self.properties {
            validate_text("property key", key)?;
            value.validate()?;
        }

        for (key, value) in &self.extensions {
            validate_identifier(key)?;
            value.validate()?;
        }

        Ok(())
    }

    /// Converts an existing legacy compilation target into a descriptor.
    ///
    /// This adapter preserves the distinction between hardware targets
    /// and non-hardware artifact targets.
    ///
    /// Unknown ABI, vendor, OS, environment and data-layout values remain
    /// unknown rather than being fabricated.
    pub fn from_legacy(
        target: &CompilationTarget,
    ) -> Result<Self, TargetError> {
        let mut builder = match target {
            CompilationTarget::X86_64Linux => {
                TargetDescriptor::builder(
                    TargetId::new("x86_64-linux")?,
                    "x86-64 Linux",
                )
                .architecture("x86_64")
                .operating_system("linux")
                .role(TargetRole::new("execution")?)
            }

            CompilationTarget::Arm64 => {
                TargetDescriptor::builder(
                    TargetId::new("aarch64")?,
                    "AArch64",
                )
                .architecture("aarch64")
                .role(TargetRole::new("execution")?)
            }

            CompilationTarget::Wasm32 => {
                TargetDescriptor::builder(
                    TargetId::new("wasm32")?,
                    "WebAssembly 32-bit",
                )
                .architecture("wasm32")
                .role(TargetRole::new("execution")?)
                .artifact_format(ArtifactFormat::new("wasm")?)
            }

            CompilationTarget::RiscV => {
                TargetDescriptor::builder(
                    TargetId::new("riscv")?,
                    "RISC-V",
                )
                .architecture("riscv")
                .role(TargetRole::new("execution")?)
            }

            CompilationTarget::QASM => {
                TargetDescriptor::builder(
                    TargetId::new("qasm")?,
                    "OpenQASM",
                )
                .role(TargetRole::new("hardware-description")?)
                .artifact_format(ArtifactFormat::new("qasm")?)
            }

            CompilationTarget::NanoControl => {
                TargetDescriptor::builder(
                    TargetId::new("nano-control")?,
                    "Nano Control",
                )
                .role(TargetRole::new("execution")?)
            }

            CompilationTarget::MTSBytecode => {
                TargetDescriptor::builder(
                    TargetId::new("mts-bytecode")?,
                    "MTS Bytecode",
                )
                .role(TargetRole::new("intermediate-representation")?)
                .artifact_format(ArtifactFormat::new("mts-bytecode")?)
            }

            CompilationTarget::LLVMIR => {
                TargetDescriptor::builder(
                    TargetId::new("llvm-ir")?,
                    "LLVM Intermediate Representation",
                )
                .role(TargetRole::new("intermediate-representation")?)
                .artifact_format(ArtifactFormat::new("llvm-ir")?)
            }

            CompilationTarget::UniversalIRExport(name) => {
                let normalized = name.trim().to_ascii_lowercase();

                if normalized.is_empty() {
                    return Err(TargetError::InvalidLegacyTarget {
                        reason: "universal IR export name cannot be empty"
                            .to_owned(),
                    });
                }

                TargetDescriptor::builder(
                    TargetId::new(format!("ir-export:{normalized}"))?,
                    format!("Universal IR Export: {name}"),
                )
                .role(TargetRole::new("intermediate-representation")?)
                .artifact_format(ArtifactFormat::new(&normalized)?)
            }
        };

        // Legacy targets do not provide reliable information about
        // every ABI, environment, data-layout or capability.
        //
        // Do not invent these values.

        builder.api_version(TARGET_DESCRIPTOR_API_VERSION);

        builder.build()
    }
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

// ============================================================================
// TARGET DESCRIPTOR BUILDER
// ============================================================================

/// Builder for immutable target descriptors.
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

    properties: BTreeMap<String, TargetProperty>,

    extensions: BTreeMap<String, TargetProperty>,

    api_version: u16,
}

impl TargetDescriptorBuilder {
    /// Creates a new descriptor builder.
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
            properties: BTreeMap::new(),
            extensions: BTreeMap::new(),
            api_version: TARGET_DESCRIPTOR_API_VERSION,
        }
    }

    /// Sets the descriptor API version.
    pub fn api_version(mut self, version: u16) -> Self {
        self.api_version = version;
        self
    }

    /// Sets the target role.
    pub fn role(mut self, role: TargetRole) -> Self {
        self.role = role;
        self
    }

    /// Sets the architecture.
    pub fn architecture(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.architecture = Some(value.into());
        self
    }

    /// Sets the vendor.
    pub fn vendor(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.vendor = Some(value.into());
        self
    }

    /// Sets the operating system.
    pub fn operating_system(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.operating_system = Some(value.into());
        self
    }

    /// Sets the execution environment.
    pub fn environment(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Sets the ABI.
    pub fn abi(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.abi = Some(value.into());
        self
    }

    /// Sets the target triple.
    pub fn target_triple(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.target_triple = Some(value.into());
        self
    }

    /// Sets the target data layout.
    pub fn data_layout(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.data_layout = Some(value.into());
        self
    }

    /// Adds a declared artifact format.
    pub fn artifact_format(
        mut self,
        format: ArtifactFormat,
    ) -> Self {
        self.artifact_formats.push(format);
        self
    }

    /// Adds a target property.
    pub fn property(
        mut self,
        key: impl Into<String>,
        value: TargetProperty,
    ) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Adds an extension property.
    ///
    /// Extension identifiers are namespaced to prevent accidental
    /// collisions with future standard properties.
    pub fn extension(
        mut self,
        key: impl Into<String>,
        value: TargetProperty,
    ) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    /// Builds and validates an immutable descriptor.
    pub fn build(self) -> Result<TargetDescriptor, TargetError> {
        if self.api_version != TARGET_DESCRIPTOR_API_VERSION {
            return Err(TargetError::UnsupportedApiVersion {
                supplied: self.api_version,
                supported: TARGET_DESCRIPTOR_API_VERSION,
            });
        }

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
            artifact_formats: deduplicate_formats(
                self.artifact_formats,
            ),
            properties: self.properties,
            extensions: self.extensions,
        };

        descriptor.validate()?;

        Ok(descriptor)
    }
}

/// Removes duplicate artifact declarations while preserving deterministic
/// order.
fn deduplicate_formats(
    formats: Vec<ArtifactFormat>,
) -> Vec<ArtifactFormat> {
    let mut seen = std::collections::BTreeSet::new();
    let mut result = Vec::new();

    for format in formats {
        if seen.insert(format.clone()) {
            result.push(format);
        }
    }

    result
}

// ============================================================================
// ERRORS
// ============================================================================

/// Target descriptor validation and construction errors.
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

    UnsupportedApiVersion {
        supplied: u16,
        supported: u16,
    },

    InvalidLegacyTarget {
        reason: String,
    },
}

impl fmt::Display for TargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} cannot contain control characters"
                )
            }

            Self::InvalidIdentifier { value, reason } => {
                write!(
                    formatter,
                    "invalid identifier `{value}`: {reason}"
                )
            }

            Self::UnsupportedApiVersion {
                supplied,
                supported,
            } => {
                write!(
                    formatter,
                    "unsupported target descriptor API version \
                     {supplied}; supported version is {supported}"
                )
            }

            Self::InvalidLegacyTarget { reason } => {
                write!(
                    formatter,
                    "invalid legacy compilation target: {reason}"
                )
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
    fn target_id_accepts_extension_identifiers() {
        let id = TargetId::new("vendor:photonic:processor").unwrap();

        assert_eq!(
            id.as_str(),
            "vendor:photonic:processor"
        );
    }

    #[test]
    fn target_id_rejects_empty_identifiers() {
        assert!(TargetId::new("").is_err());
    }

    #[test]
    fn target_id_rejects_uppercase_identifiers() {
        assert!(TargetId::new("ARM64").is_err());
    }

    #[test]
    fn target_id_rejects_control_characters() {
        assert!(TargetId::new("target\nname").is_err());
    }

    #[test]
    fn target_id_does_not_impose_arbitrary_length_limits() {
        let value = format!("target-{}", "x".repeat(10_000));

        assert!(TargetId::new(value).is_ok());
    }

    #[test]
    fn target_descriptor_preserves_architecture() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("aarch64").unwrap(),
            "AArch64",
        )
        .architecture("aarch64")
        .build()
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("aarch64")
        );
    }

    #[test]
    fn target_descriptor_separates_os_and_architecture() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("example-target").unwrap(),
            "Example",
        )
        .architecture("example-arch")
        .operating_system("example-os")
        .build()
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("example-arch")
        );

        assert_eq!(
            descriptor.operating_system(),
            Some("example-os")
        );
    }

    #[test]
    fn unknown_target_properties_remain_unknown() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("future-target").unwrap(),
            "Future Target",
        )
        .build()
        .unwrap();

        assert_eq!(descriptor.architecture(), None);
        assert_eq!(descriptor.vendor(), None);
        assert_eq!(descriptor.abi(), None);
        assert_eq!(descriptor.data_layout(), None);
    }

    #[test]
    fn target_descriptor_supports_future_architectures() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("future:optical:processor").unwrap(),
            "Optical Processor",
        )
        .architecture("optical-computing")
        .vendor("future-vendor")
        .role(TargetRole::new("execution").unwrap())
        .build()
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("optical-computing")
        );
    }

    #[test]
    fn artifact_formats_are_extensible() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("custom-target").unwrap(),
            "Custom Target",
        )
        .artifact_format(
            ArtifactFormat::new("custom-bitstream").unwrap(),
        )
        .build()
        .unwrap();

        assert_eq!(
            descriptor.artifact_formats()[0].as_str(),
            "custom-bitstream"
        );
    }

    #[test]
    fn duplicate_artifact_formats_are_removed() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("custom-target").unwrap(),
            "Custom Target",
        )
        .artifact_format(
            ArtifactFormat::new("wasm").unwrap(),
        )
        .artifact_format(
            ArtifactFormat::new("wasm").unwrap(),
        )
        .build()
        .unwrap();

        assert_eq!(descriptor.artifact_formats().len(), 1);
    }

    #[test]
    fn target_properties_support_extensions() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("custom-target").unwrap(),
            "Custom Target",
        )
        .extension(
            "vendor:revision",
            TargetProperty::Text("revision-1".to_owned()),
        )
        .build()
        .unwrap();

        assert_eq!(
            descriptor.extension("vendor:revision"),
            Some(&TargetProperty::Text(
                "revision-1".to_owned()
            ))
        );
    }

    #[test]
    fn invalid_extension_names_are_rejected() {
        let result = TargetDescriptor::builder(
            TargetId::new("custom-target").unwrap(),
            "Custom Target",
        )
        .extension(
            "INVALID NAME",
            TargetProperty::Boolean(true),
        )
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn legacy_x86_target_preserves_x86_architecture() {
        let descriptor = TargetDescriptor::from_legacy(
            &CompilationTarget::X86_64Linux,
        )
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("x86_64")
        );

        assert_eq!(
            descriptor.operating_system(),
            Some("linux")
        );
    }

    #[test]
    fn legacy_arm_target_never_becomes_x86() {
        let descriptor = TargetDescriptor::from_legacy(
            &CompilationTarget::Arm64,
        )
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("aarch64")
        );

        assert_ne!(
            descriptor.architecture(),
            Some("x86_64")
        );
    }

    #[test]
    fn legacy_wasm_target_declares_wasm_artifact() {
        let descriptor = TargetDescriptor::from_legacy(
            &CompilationTarget::Wasm32,
        )
        .unwrap();

        assert!(
            descriptor.artifact_formats().iter().any(
                |format| format.as_str() == "wasm"
            )
        );
    }

    #[test]
    fn legacy_qasm_is_not_misrepresented_as_cpu_architecture() {
        let descriptor = TargetDescriptor::from_legacy(
            &CompilationTarget::QASM,
        )
        .unwrap();

        assert_eq!(descriptor.architecture(), None);

        assert_eq!(
            descriptor.role().as_str(),
            "hardware-description"
        );
    }

    #[test]
    fn target_descriptor_is_deterministically_comparable() {
        let first = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "Example",
        )
        .build()
        .unwrap();

        let second = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "Example",
        )
        .build()
        .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn unsupported_api_versions_are_rejected() {
        let result = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "Example",
        )
        .api_version(999)
        .build();

        assert!(matches!(
            result,
            Err(TargetError::UnsupportedApiVersion { .. })
        ));
    }

    #[test]
    fn invalid_display_names_are_rejected() {
        let result = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "",
        )
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn data_layout_is_not_interpreted_as_architecture() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "Example",
        )
        .architecture("example-architecture")
        .data_layout("e-p:64:64")
        .build()
        .unwrap();

        assert_eq!(
            descriptor.architecture(),
            Some("example-architecture")
        );

        assert_eq!(
            descriptor.data_layout(),
            Some("e-p:64:64")
        );
    }

    #[test]
    fn target_descriptor_supports_nested_properties() {
        let mut nested = BTreeMap::new();

        nested.insert(
            "revision".to_owned(),
            TargetProperty::Unsigned(1),
        );

        let descriptor = TargetDescriptor::builder(
            TargetId::new("example").unwrap(),
            "Example",
        )
        .property(
            "metadata",
            TargetProperty::Map(nested),
        )
        .build()
        .unwrap();

        assert!(descriptor.property("metadata").is_some());
    }
}
