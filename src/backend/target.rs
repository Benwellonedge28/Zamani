
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Target descriptions and target metadata for the Zamani backend.
//!
//! Path: `src/backend/target.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Describe compilation targets independently of workload requirements,
//! target selection, code generation, toolchain execution, and physical
//! resource allocation.
//!
//! # Architectural authority
//!
//! - `crate::compiler_types::CompilationTarget` owns the existing legacy
//!   compiler target selection API.
//! - `super::capabilities` owns capability identifiers, declarations,
//!   evidence, profiles, and capability-matching policy.
//! - `super::requirements` owns backend-facing workload requirements.
//! - `super::errors` owns structured backend errors.
//! - `grammar/Zamani.g4` and the canonical imported grammar components own
//!   source syntax.
//! - `grammar/specification/semantic-model.md` owns source-language meaning.
//! - Domain IRs own their respective computational representations.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Extensible target identifiers.
//! - Target descriptors and descriptive metadata.
//! - Optional target triples, architecture names, operating systems, ABIs,
//!   and data-layout descriptions.
//! - Artifact-format identifiers associated with a target.
//! - The target's capability profile.
//! - An explicit indication of whether an existing legacy emitter is known
//!   to be present, the descriptor is metadata-only, or support is unavailable.
//! - An adapter from the existing `CompilationTarget` enum.
//!
//! # Non-goals
//!
//! This module does not own:
//!
//! - Source parsing, semantic analysis, or source-language validity.
//! - Workload requirements, preferences, constraints, or hints.
//! - Target discovery, registry management, or target selection.
//! - Capability matching policy.
//! - Code generation or IR lowering.
//! - Toolchain discovery or invocation.
//! - Device discovery, physical resource allocation, placement, or scheduling.
//! - Quantum routing, calibration, error correction, or pulse generation.
//! - HDL synthesis, timing closure, or bitstream generation.
//! - Runtime execution or deployment.
//!
//! # Scalability and POCO-REAF
//!
//! Target identifiers, artifact-format identifiers, and extension properties
//! are open-world strings rather than closed universal enumerations.
//! Architecture, operating system, ABI, and data-layout metadata are
//! independent because they are not interchangeable concepts.
//!
//! This module imposes no universal limits on CPUs, qubits, devices, nodes,
//! memory, tensor dimensions, registers, or other resource capacities.
//! A descriptor does not promise unlimited resources or prove that a workload
//! is feasible. Feasibility must be established by the responsible downstream
//! components using actual target and resource evidence.
//!
//! # Integration contract
//!
//! 1. `mod.rs` declares `pub mod target;`.
//! 2. `capabilities.rs` remains the sole owner of `CapabilityProfile`.
//! 3. `compiler_types.rs` remains the owner of `CompilationTarget`.
//! 4. `selection.rs` consumes descriptors and evaluates workload requirements.
//! 5. `registry.rs` owns registration, duplicate detection, and discovery.
//! 6. `validation.rs` checks the requested target, input representation,
//!    capabilities, artifact contract, and toolchain preconditions.
//! 7. Target emitters must verify their own actual support. A descriptor alone
//!    does not make a target executable or production-ready.
//! 8. This module must not import AST, semantic analyzer, classical IR,
//!    quantum IR, HDL IR, or runtime implementation types.
//!
//! # Migration note
//!
//! The repository has both `src/backend.rs` and `src/backend/mod.rs`.
//! Resolve that module-root conflict as a separate migration step before
//! treating this file as integrated into the compiled crate.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::compiler_types::CompilationTarget;

use super::capabilities::CapabilityProfile;
use super::errors::{BackendError, BackendErrorKind, BackendResult};

/// A validated, open-world identifier for a compilation target.
///
/// Examples include `x86-64-linux`, `aarch64`, `wasm32`, `quantum-qasm`,
/// and vendor- or extension-defined identifiers.
///
/// IDs are case-sensitive and are not silently normalized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId(String);

impl TargetId {
    /// Construct a target identifier.
    ///
    /// Accepted characters are ASCII letters, digits, `.`, `_`, `-`, `:`,
    /// and `/`. The first character must be an ASCII letter or digit.
    ///
    /// The syntax is deliberately open-world: it does not enumerate
    /// architectures, vendors, computational domains, or execution models.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "target identifier must not be empty",
            ));
        }

        let mut characters = value.chars();

        let first = characters.next().ok_or_else(|| {
            BackendError::new(
                BackendErrorKind::InvalidInput,
                "target identifier must not be empty",
            )
        })?;

        if !first.is_ascii_alphanumeric() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!(
                    "target identifier must begin with an ASCII letter or digit: `{value}`"
                ),
            ));
        }

        if !characters.all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, '.' | '_' | '-' | ':' | '/')
        }) {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("invalid character in target identifier `{value}`"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier without allocating.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An extensible identifier for an artifact format.
///
/// Examples include `llvm-ir`, `assembly`, `wat`, `wasm`, `qasm`, `verilog`,
/// `vhdl`, `mts-bytecode`, and extension-defined formats.
///
/// The format identifier does not, by itself, establish that an artifact is
/// valid, executable, verified, or supported by an installed toolchain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactFormat(String);

impl ArtifactFormat {
    /// Construct an artifact-format identifier.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "artifact-format identifier must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!(
                    "artifact-format identifier must not contain leading or trailing \
                     whitespace: `{value}`"
                ),
            ));
        }

        if !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b'-' | b':' | b'/')
        }) {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("invalid character in artifact-format identifier `{value}`"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier without allocating.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ArtifactFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An opaque target triple.
///
/// A triple's grammar and interpretation depend on the target ecosystem and
/// toolchain. This wrapper validates basic input hygiene but deliberately
/// does not claim to parse every vendor's triple syntax.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetTriple(String);

impl TargetTriple {
    /// Construct a non-empty target triple.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "target triple must be non-empty and have no surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_whitespace) {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("target triple contains whitespace: `{value}`"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the triple without allocating.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TargetTriple {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Descriptive ABI metadata.
///
/// All fields are optional because an architecture name alone does not
/// establish an operating system, calling convention, object format, or ABI
/// version. Unknown metadata must remain unknown rather than be guessed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AbiDescriptor {
    name: Option<String>,
    version: Option<String>,
    calling_convention: Option<String>,
    object_format: Option<String>,
}

impl AbiDescriptor {
    /// Construct an ABI descriptor with a required descriptive name.
    pub fn new(name: impl Into<String>) -> BackendResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "ABI name must not be empty",
            ));
        }

        Ok(Self {
            name: Some(name),
            version: None,
            calling_convention: None,
            object_format: None,
        })
    }

    /// Set the ABI version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the calling convention.
    pub fn with_calling_convention(
        mut self,
        convention: impl Into<String>,
    ) -> Self {
        self.calling_convention = Some(convention.into());
        self
    }

    /// Set the object-file format.
    pub fn with_object_format(mut self, format: impl Into<String>) -> Self {
        self.object_format = Some(format.into());
        self
    }

    /// Return the ABI name, if known.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the ABI version, if known.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Return the calling convention, if known.
    pub fn calling_convention(&self) -> Option<&str> {
        self.calling_convention.as_deref()
    }

    /// Return the object-file format, if known.
    pub fn object_format(&self) -> Option<&str> {
        self.object_format.as_deref()
    }

    fn validate(&self) -> BackendResult<()> {
        for (field, value) in [
            ("ABI name", self.name.as_deref()),
            ("ABI version", self.version.as_deref()),
            ("calling convention", self.calling_convention.as_deref()),
            ("object format", self.object_format.as_deref()),
        ] {
            if value.is_some_and(|value| value.trim().is_empty()) {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidInput,
                    format!("{field} must not be empty when supplied"),
                ));
            }
        }

        Ok(())
    }
}

/// Descriptive data-layout metadata.
///
/// The layout string is intentionally opaque. LLVM data-layout syntax,
/// address-space rules, pointer representations, alignment, endianness, and
/// other layout properties are target-specific. This type does not assume
/// that all address spaces share a pointer width or representation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DataLayoutDescriptor {
    format: Option<String>,
    description: Option<String>,
}

impl DataLayoutDescriptor {
    /// Create a descriptor with an explicitly identified layout format.
    pub fn new(format: impl Into<String>) -> BackendResult<Self> {
        let format = format.into();

        if format.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "data-layout format must not be empty",
            ));
        }

        Ok(Self {
            format: Some(format),
            description: None,
        })
    }

    /// Attach a descriptive representation of the layout.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the format name, if known.
    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Return the layout description, if supplied.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn validate(&self) -> BackendResult<()> {
        for (field, value) in [
            ("data-layout format", self.format.as_deref()),
            ("data-layout description", self.description.as_deref()),
        ] {
            if value.is_some_and(|value| value.trim().is_empty()) {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidInput,
                    format!("{field} must not be empty when supplied"),
                ));
            }
        }

        Ok(())
    }
}

/// Broad execution-model classification.
///
/// `Other` keeps the model extensible. This classification is descriptive;
/// it is not a replacement for target capabilities or artifact-format
/// contracts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionModel {
    /// Conventional native execution.
    Native,

    /// WebAssembly execution.
    WebAssembly,

    /// Quantum-program or quantum-circuit output.
    Quantum,

    /// Hardware-description or hardware-synthesis flow.
    HardwareDescription,

    /// Bytecode interpreted or executed by a corresponding runtime.
    Bytecode,

    /// Intermediate representation rather than a directly executable target.
    IntermediateRepresentation,

    /// Named extension-defined execution model.
    Other(String),
}

impl ExecutionModel {
    /// Return a stable descriptive name.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Native => "native",
            Self::WebAssembly => "webassembly",
            Self::Quantum => "quantum",
            Self::HardwareDescription => "hardware-description",
            Self::Bytecode => "bytecode",
            Self::IntermediateRepresentation => "intermediate-representation",
            Self::Other(name) => name,
        }
    }
}

/// What is known about implementation availability for a target.
///
/// `LegacyEmitterPresent` means the inspected legacy dispatcher has an
/// emitter path for the target. It is not a production-readiness certification.
///
/// `DescriptorOnly` means that metadata can be described, but this adapter
/// does not assert that a complete emitter is available.
///
/// `Unavailable` means this adapter must not advertise the legacy target path
/// as usable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplementationAvailability {
    /// The descriptor exists, but no emitter availability is asserted.
    DescriptorOnly,

    /// An emitter path exists in the legacy implementation.
    LegacyEmitterPresent,

    /// The legacy implementation cannot correctly provide this target.
    Unavailable,
}

/// Target descriptor.
///
/// This value describes a target; it does not select it, execute it, prove
/// its compatibility with a workload, or allocate its resources.
///
/// Private fields prevent construction of obviously malformed descriptors
/// without passing through the public constructor. Deserialized or externally
/// supplied metadata must still be validated by its ingestion boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetDescriptor {
    id: TargetId,
    display_name: String,
    execution_model: ExecutionModel,
    triple: Option<TargetTriple>,
    architecture: Option<String>,
    operating_system: Option<String>,
    abi: Option<AbiDescriptor>,
    data_layout: Option<DataLayoutDescriptor>,
    artifact_formats: BTreeSet<ArtifactFormat>,
    capability_profile: CapabilityProfile,
    availability: ImplementationAvailability,
    properties: BTreeMap<String, String>,
}

impl TargetDescriptor {
    /// Create a new target descriptor with no inferred architecture,
    /// operating system, ABI, data layout, artifact formats, or capabilities.
    ///
    /// The default availability is `DescriptorOnly`. A caller must explicitly
    /// declare what is known about implementation availability.
    pub fn new(
        id: TargetId,
        display_name: impl Into<String>,
        execution_model: ExecutionModel,
    ) -> BackendResult<Self> {
        let display_name = display_name.into();

        if display_name.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "target display name must not be empty",
            ));
        }

        if let ExecutionModel::Other(name) = &execution_model {
            if name.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidInput,
                    "extension execution-model name must not be empty",
                ));
            }
        }

        Ok(Self {
            id,
            display_name,
            execution_model,
            triple: None,
            architecture: None,
            operating_system: None,
            abi: None,
            data_layout: None,
            artifact_formats: BTreeSet::new(),
            capability_profile: CapabilityProfile::new("target"),
            availability: ImplementationAvailability::DescriptorOnly,
            properties: BTreeMap::new(),
        })
    }

    /// Construct a descriptor from an open-world identifier.
    pub fn from_name(
        id: impl Into<String>,
        display_name: impl Into<String>,
        execution_model: ExecutionModel,
    ) -> BackendResult<Self> {
        Self::new(TargetId::new(id)?, display_name, execution_model)
    }

    /// Adapt the repository's existing legacy target enum.
    ///
    /// This adapter preserves the legacy public API while keeping target
    /// metadata in one reusable descriptor. It deliberately avoids guessing
    /// unspecified triples, operating systems, ABIs, data layouts, or resource
    /// capacities.
    ///
    /// In particular, `CompilationTarget::Arm64` is marked `Unavailable`
    /// because the inspected legacy dispatcher incorrectly routes it to the
    /// x86-64 backend. This adapter does not repeat that dispatch defect.
    ///
    /// Availability records the legacy implementation inventory, not a
    /// verified claim of production readiness.
    pub fn from_compilation_target(
        target: &CompilationTarget,
    ) -> BackendResult<Self> {
        let descriptor = match target {
            CompilationTarget::X86_64Linux => {
                let mut descriptor = Self::from_name(
                    "x86-64-linux",
                    "x86-64 Linux",
                    ExecutionModel::Native,
                )?
                .with_architecture("x86_64")?
                .with_operating_system("linux")?
                .with_abi(AbiDescriptor::new("system-v")?)?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("assembly")?;
                descriptor
            }

            CompilationTarget::Arm64 => Self::from_name(
                "aarch64",
                "AArch64",
                ExecutionModel::Native,
            )?
            .with_architecture("aarch64")?
            .with_availability(ImplementationAvailability::Unavailable),

            CompilationTarget::Wasm32 => {
                let mut descriptor = Self::from_name(
                    "wasm32",
                    "WebAssembly 32-bit",
                    ExecutionModel::WebAssembly,
                )?
                .with_architecture("wasm32")?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("wat")?;
                Ok(descriptor)?
            }

            CompilationTarget::QASM => {
                let mut descriptor = Self::from_name(
                    "quantum-qasm",
                    "Quantum Assembly",
                    ExecutionModel::Quantum,
                )?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("qasm")?;
                descriptor
            }

            CompilationTarget::NanoControl => {
                let mut descriptor = Self::from_name(
                    "nanocontrol",
                    "NanoControl",
                    ExecutionModel::Bytecode,
                )?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("nano-control")?;
                descriptor
            }

            CompilationTarget::MTSBytecode => {
                let mut descriptor = Self::from_name(
                    "mts-bytecode",
                    "MTS Bytecode",
                    ExecutionModel::Bytecode,
                )?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("mts-bytecode")?;
                descriptor
            }

            CompilationTarget::LLVMIR => {
                let mut descriptor = Self::from_name(
                    "llvm-ir",
                    "LLVM Intermediate Representation",
                    ExecutionModel::IntermediateRepresentation,
                )?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("llvm-ir")?;
                descriptor
            }

            CompilationTarget::RiscV => {
                let mut descriptor = Self::from_name(
                    "riscv",
                    "RISC-V",
                    ExecutionModel::Native,
                )?
                .with_architecture("riscv64")?
                .with_availability(
                    ImplementationAvailability::LegacyEmitterPresent,
                );

                descriptor.insert_artifact_format("assembly")?;
                descriptor
            }

            CompilationTarget::UniversalIRExport(name) => {
                let id = format!("universal-ir:{name}");
                let mut descriptor = Self::from_name(
                    id,
                    format!("Universal IR export ({name})"),
                    ExecutionModel::IntermediateRepresentation,
                )?
                .with_availability(ImplementationAvailability::DescriptorOnly);

                descriptor.insert_artifact_format("universal-ir")?;
                descriptor
            }
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Return the target identifier.
    pub fn id(&self) -> &TargetId {
        &self.id
    }

    /// Return the display name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return the execution-model classification.
    pub fn execution_model(&self) -> &ExecutionModel {
        &self.execution_model
    }

    /// Return the target triple, if explicitly known.
    pub fn triple(&self) -> Option<&TargetTriple> {
        self.triple.as_ref()
    }

    /// Return the architecture name, if explicitly known.
    pub fn architecture(&self) -> Option<&str> {
        self.architecture.as_deref()
    }

    /// Return the operating system, if explicitly known.
    pub fn operating_system(&self) -> Option<&str> {
        self.operating_system.as_deref()
    }

    /// Return the ABI metadata, if supplied.
    pub fn abi(&self) -> Option<&AbiDescriptor> {
        self.abi.as_ref()
    }

    /// Return the data-layout metadata, if supplied.
    pub fn data_layout(&self) -> Option<&DataLayoutDescriptor> {
        self.data_layout.as_ref()
    }

    /// Return the set of artifact formats declared by this descriptor.
    pub fn artifact_formats(&self) -> &BTreeSet<ArtifactFormat> {
        &self.artifact_formats
    }

    /// Return the target's canonical backend capability profile.
    pub fn capability_profile(&self) -> &CapabilityProfile {
        &self.capability_profile
    }

    /// Mutably access the capability profile.
    ///
    /// Call [`TargetDescriptor::validate`] after changing it and before
    /// registration or target selection.
    pub fn capability_profile_mut(&mut self) -> &mut CapabilityProfile {
        &mut self.capability_profile
    }

    /// Return the implementation-availability declaration.
    pub const fn availability(&self) -> ImplementationAvailability {
        self.availability
    }

    /// Return extension properties in deterministic key order.
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }

    /// Set the target triple.
    pub fn with_triple(mut self, triple: TargetTriple) -> Self {
        self.triple = Some(triple);
        self
    }

    /// Set the target triple from text.
    pub fn with_triple_name(
        self,
        triple: impl Into<String>,
    ) -> BackendResult<Self> {
        Ok(self.with_triple(TargetTriple::new(triple)?))
    }

    /// Set the architecture name.
    pub fn with_architecture(
        mut self,
        architecture: impl Into<String>,
    ) -> BackendResult<Self> {
        self.architecture = Some(non_empty_metadata(
            "architecture",
            architecture.into(),
        )?);
        Ok(self)
    }

    /// Set the operating-system name.
    pub fn with_operating_system(
        mut self,
        operating_system: impl Into<String>,
    ) -> BackendResult<Self> {
        self.operating_system = Some(non_empty_metadata(
            "operating system",
            operating_system.into(),
        )?);
        Ok(self)
    }

    /// Set ABI metadata.
    pub fn with_abi(mut self, abi: AbiDescriptor) -> BackendResult<Self> {
        abi.validate()?;
        self.abi = Some(abi);
        Ok(self)
    }

    /// Set data-layout metadata.
    pub fn with_data_layout(
        mut self,
        data_layout: DataLayoutDescriptor,
    ) -> BackendResult<Self> {
        data_layout.validate()?;
        self.data_layout = Some(data_layout);
        Ok(self)
    }

    /// Add a supported artifact format.
    ///
    /// This only declares the format associated with the descriptor. Actual
    /// emission and validation must be implemented by the relevant backend.
    pub fn with_artifact_format(
        mut self,
        format: ArtifactFormat,
    ) -> Self {
        self.artifact_formats.insert(format);
        self
    }

    /// Add an artifact format from text.
    pub fn with_artifact_format_name(
        self,
        format: impl Into<String>,
    ) -> BackendResult<Self> {
        Ok(self.with_artifact_format(ArtifactFormat::new(format)?))
    }

    /// Replace the capability profile.
    pub fn with_capability_profile(
        mut self,
        profile: CapabilityProfile,
    ) -> BackendResult<Self> {
        profile.validate()?;
        self.capability_profile = profile;
        Ok(self)
    }

    /// Set implementation availability explicitly.
    pub fn with_availability(
        mut self,
        availability: ImplementationAvailability,
    ) -> Self {
        self.availability = availability;
        self
    }

    /// Add or replace a namespaced extension property.
    ///
    /// Properties are descriptive metadata. They must not silently override
    /// capability matching rules, workload requirements, or target semantics.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> BackendResult<Self> {
        let key = non_empty_metadata("target property key", key.into())?;
        let value = non_empty_metadata("target property value", value.into())?;
        self.properties.insert(key, value);
        Ok(self)
    }

    /// Validate the complete descriptor before registration or use.
    ///
    /// Call this after building or mutating a descriptor, and after accepting
    /// target metadata from an external source.
    pub fn validate(&self) -> BackendResult<()> {
        if self.display_name.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                "target display name must not be empty",
            ));
        }

        if let ExecutionModel::Other(name) = &self.execution_model {
            if name.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidInput,
                    "extension execution-model name must not be empty",
                ));
            }
        }

        if let Some(architecture) = &self.architecture {
            validate_metadata("architecture", architecture)?;
        }

        if let Some(operating_system) = &self.operating_system {
            validate_metadata("operating system", operating_system)?;
        }

        if let Some(abi) = &self.abi {
            abi.validate()?;
        }

        if let Some(data_layout) = &self.data_layout {
            data_layout.validate()?;
        }

        for (key, value) in &self.properties {
            validate_metadata("target property key", key)?;
            validate_metadata("target property value", value)?;
        }

        self.capability_profile.validate()
    }

    /// Whether this descriptor declares an artifact format.
    pub fn supports_artifact_format(&self, format: &ArtifactFormat) -> bool {
        self.artifact_formats.contains(format)
    }

    /// Whether this descriptor declares an artifact format by name.
    ///
    /// Invalid format names return `false`; use `ArtifactFormat::new` when
    /// callers need to distinguish invalid input from a missing declaration.
    pub fn supports_artifact_format_name(&self, format: &str) -> bool {
        ArtifactFormat::new(format.to_owned())
            .is_ok_and(|format| self.supports_artifact_format(&format))
    }

    fn insert_artifact_format(
        &mut self,
        format: &str,
    ) -> BackendResult<()> {
        self.artifact_formats.insert(ArtifactFormat::new(format)?);
        Ok(())
    }
}

fn non_empty_metadata(field: &str, value: String) -> BackendResult<String> {
    validate_metadata(field, &value)?;
    Ok(value)
}

fn validate_metadata(field: &str, value: &str) -> BackendResult<()> {
    if value.trim().is_empty() {
        return Err(BackendError::new(
            BackendErrorKind::InvalidInput,
            format!("{field} must not be empty when supplied"),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        AbiDescriptor, ArtifactFormat, ExecutionModel, ImplementationAvailability,
        TargetDescriptor, TargetId, TargetTriple,
    };
    use crate::compiler_types::CompilationTarget;

    #[test]
    fn target_ids_are_open_world_but_validated() {
        assert!(TargetId::new("x86-64-linux").is_ok());
        assert!(TargetId::new("future.vendor:accelerator-v2").is_ok());
        assert!(TargetId::new("domain/target-v1").is_ok());

        assert!(TargetId::new("").is_err());
        assert!(TargetId::new("-invalid").is_err());
        assert!(TargetId::new("target with spaces").is_err());
    }

    #[test]
    fn artifact_formats_are_open_world() {
        assert!(ArtifactFormat::new("llvm-ir").is_ok());
        assert!(ArtifactFormat::new("future-format-v2").is_ok());
        assert!(ArtifactFormat::new("vendor/custom:format").is_ok());

        assert!(ArtifactFormat::new("").is_err());
        assert!(ArtifactFormat::new("format with spaces").is_err());
    }

    #[test]
    fn target_triples_reject_empty_or_whitespace_values() {
        assert!(TargetTriple::new("x86_64-unknown-linux-gnu").is_ok());
        assert!(TargetTriple::new("").is_err());
        assert!(TargetTriple::new(" target ").is_err());
        assert!(TargetTriple::new("target with spaces").is_err());
    }

    #[test]
    fn descriptor_does_not_infer_unspecified_metadata() {
        let descriptor = TargetDescriptor::from_name(
            "future-accelerator",
            "Future Accelerator",
            ExecutionModel::Other("heterogeneous".to_owned()),
        )
        .expect("valid descriptor");

        assert_eq!(descriptor.architecture(), None);
        assert_eq!(descriptor.operating_system(), None);
        assert_eq!(descriptor.triple(), None);
        assert_eq!(descriptor.abi(), None);
        assert_eq!(descriptor.data_layout(), None);
        assert_eq!(
            descriptor.availability(),
            ImplementationAvailability::DescriptorOnly
        );
    }

    #[test]
    fn legacy_arm64_mapping_does_not_claim_x86_support() {
        let descriptor = TargetDescriptor::from_compilation_target(
            &CompilationTarget::Arm64,
        )
        .expect("descriptor can describe AArch64");

        assert_eq!(descriptor.architecture(), Some("aarch64"));
        assert_eq!(
            descriptor.availability(),
            ImplementationAvailability::Unavailable
        );
        assert!(!descriptor.supports_artifact_format_name("assembly"));
    }

    #[test]
    fn legacy_x86_target_preserves_known_metadata() {
        let descriptor = TargetDescriptor::from_compilation_target(
            &CompilationTarget::X86_64Linux,
        )
        .expect("descriptor");

        assert_eq!(descriptor.architecture(), Some("x86_64"));
        assert_eq!(descriptor.operating_system(), Some("linux"));
        assert_eq!(
            descriptor.abi().and_then(AbiDescriptor::name),
            Some("system-v")
        );
        assert!(descriptor.supports_artifact_format_name("assembly"));
    }

    #[test]
    fn llvm_ir_is_not_misclassified_as_a_native_architecture() {
        let descriptor = TargetDescriptor::from_compilation_target(
            &CompilationTarget::LLVMIR,
        )
        .expect("descriptor");

        assert_eq!(
            descriptor.execution_model(),
            &ExecutionModel::IntermediateRepresentation
        );
        assert_eq!(descriptor.architecture(), None);
        assert!(descriptor.supports_artifact_format_name("llvm-ir"));
    }

    #[test]
    fn descriptor_validation_checks_capability_profile() {
        let mut descriptor = TargetDescriptor::from_name(
            "test-target",
            "Test Target",
            ExecutionModel::Native,
        )
        .expect("descriptor");

        descriptor
            .capability_profile_mut()
            .with_attribute("vendor", "example");

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn duplicate_artifact_format_insertion_is_idempotent() {
        let format = ArtifactFormat::new("llvm-ir").expect("format");
        let descriptor = TargetDescriptor::from_name(
            "test-target",
            "Test Target",
            ExecutionModel::IntermediateRepresentation,
        )
        .expect("descriptor")
        .with_artifact_format(format.clone())
        .with_artifact_format(format);

        assert_eq!(descriptor.artifact_formats().len(), 1);
    }

    #[test]
    fn invalid_abi_metadata_is_rejected() {
        assert!(AbiDescriptor::new("system-v")
            .expect("valid ABI")
            .with_calling_convention(" ")
            .validate()
            .is_err());
    }
}
