
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Self-Synthesizing Backend Engine (SSBE).
//!
//! Path: src/compiler/ssbe.rs
//! Edition: 2021
//! Minimum Rust version: 1.97.
//!
//! # Purpose
//!
//! SSBE generates deterministic Rust source containing validated
//! backend-provider metadata.
//!
//! SSBE does not implement target-specific machine-code generation.
//! It does not create executable artifacts or register unverified
//! backend implementations.
//!
//! Generated provider metadata must subsequently pass the normal
//! Rust compilation, security, backend integration, registration,
//! artifact validation and compiler qualification processes.
//!
//! # Architectural ownership
//!
//! Canonical source semantics:
//!     crate::frontend::semantic
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Canonical backend interface:
//!     crate::backend
//!
//! Canonical backend registry:
//!     crate::backend::registry
//!
//! Compiler-facing backend integration:
//!     crate::compiler::backends
//!
//! Artifact validation:
//!     crate::compiler::artifacts
//!
//! Security verification:
//!     crate::compiler::security
//!
//! Compilation orchestration:
//!     crate::compiler::pipeline
//!
//! SSBE integrates with these authorities.
//! It does not replace them.
//!
//! # Scalability
//!
//! This module imposes no arbitrary universal limits on:
//!
//! - Instruction count.
//! - Instruction length.
//! - Substrate name length.
//! - Paradigm name length.
//! - Translation-rule length.
//! - Backend provider count.
//! - Computational domains.
//! - Hardware architectures.
//! - Accelerator count.
//! - Quantum processor count.
//! - Distributed participants.
//! - Future computational technologies.
//!
//! An explicitly configured output budget is an operational
//! restriction, not a language or hardware capability limit.
//!
//! Actual capacity depends on available memory, storage,
//! operating-system limits and configured operational policies.
//!
//! # Security
//!
//! - No unsafe Rust.
//! - No execution of generated source.
//! - No execution of translation rules.
//! - No arbitrary shell commands.
//! - No path derived directly from untrusted substrate names.
//! - No silent replacement when overwrite is disabled.
//! - Exclusive temporary-file creation.
//! - Deterministic metadata generation.
//! - Structured error propagation.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! A generated provider descriptor is independent of physical
//! resource allocation and runtime execution.
//!
//! A registered provider must separately demonstrate compatibility,
//! capability support, feasibility and correct backend realization.

use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable SSBE module identity.
pub const SSBE_MODULE_NAME: &str =
    "zamani.compiler.ssbe";

/// SSBE API version.
///
/// Independent of the Zamani language, compiler, backend and IR versions.
pub const SSBE_API_VERSION: u16 = 2;

/// Generated metadata schema version.
pub const SSBE_METADATA_SCHEMA_VERSION: u32 = 1;

/// Generated metadata source format.
pub const SSBE_SOURCE_FORMAT: &str =
    "zamani.backend-provider-metadata.rust";

const TEMPORARY_FILE_PREFIX: &str = ".zamani-ssbe";

static TEMPORARY_FILE_SEQUENCE: AtomicU64 =
    AtomicU64::new(0);

// ============================================================================
// SUBSTRATE DEFINITION
// ============================================================================

/// Definition of an execution substrate.
///
/// This is provider-generation input.
///
/// It is not a physical hardware allocation, backend implementation,
/// target compatibility decision or executable instruction stream.
///
/// The instruction set represents descriptive provider metadata.
/// It is not interpreted or executed by SSBE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstrateDefinition {
    /// Human-readable and machine-identifying substrate name.
    pub name: String,

    /// Programming or architectural paradigm.
    pub paradigm: String,

    /// Descriptive instruction set.
    pub instruction_set: Vec<String>,

    /// Human-readable translation-rule description.
    ///
    /// This field is metadata only. SSBE does not interpret it
    /// as executable code.
    pub translation_rule: String,
}

impl SubstrateDefinition {
    /// Construct and validate a substrate definition.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is empty or contains
    /// prohibited control characters.
    pub fn new(
        name: impl Into<String>,
        paradigm: impl Into<String>,
        instruction_set: Vec<String>,
        translation_rule: impl Into<String>,
    ) -> Result<Self, SsbeError> {
        let definition = Self {
            name: name.into(),
            paradigm: paradigm.into(),
            instruction_set,
            translation_rule: translation_rule.into(),
        };

        definition.validate()?;

        Ok(definition)
    }

    /// Validate the complete substrate definition.
    ///
    /// No arbitrary metadata-length or instruction-count ceiling
    /// is imposed.
    pub fn validate(&self) -> Result<(), SsbeError> {
        validate_text(
            &self.name,
            "substrate name",
        )?;

        validate_text(
            &self.paradigm,
            "paradigm",
        )?;

        validate_text(
            &self.translation_rule,
            "translation rule",
        )?;

        if self.instruction_set.is_empty() {
            return Err(SsbeError::InvalidDefinition(
                "instruction set cannot be empty".into(),
            ));
        }

        for (index, instruction) in
            self.instruction_set.iter().enumerate()
        {
            if instruction.trim().is_empty() {
                return Err(SsbeError::InvalidDefinition(
                    format!(
                        "instruction at index {index} cannot be empty"
                    ),
                ));
            }

            if instruction.chars().any(char::is_control) {
                return Err(SsbeError::InvalidDefinition(
                    format!(
                        "instruction at index {index} contains control characters"
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Return the deterministic content identity of this definition.
    ///
    /// The identity includes all metadata and preserves instruction order.
    ///
    /// It is suitable for identifying generated metadata, not for
    /// establishing provider authenticity or semantic correctness.
    pub fn fingerprint(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(b"zamani-ssbe-substrate-v1");

        hash_field(&mut hasher, self.name.as_bytes());
        hash_field(&mut hasher, self.paradigm.as_bytes());
        hash_field(
            &mut hasher,
            self.translation_rule.as_bytes(),
        );

        hasher.update(
            (self.instruction_set.len() as u128).to_be_bytes(),
        );

        for instruction in &self.instruction_set {
            hash_field(&mut hasher, instruction.as_bytes());
        }

        hex::encode(hasher.finalize())
    }

    /// Return a deterministic generated source filename.
    ///
    /// The filename is derived from the complete definition fingerprint,
    /// not directly from untrusted user-controlled names.
    pub fn backend_filename(&self) -> String {
        format!(
            "ssbe-{}.rs",
            self.fingerprint()
        )
    }

    /// Return the stable generated metadata type name.
    ///
    /// The type name is deliberately independent of the substrate name.
    pub fn backend_type_name(&self) -> &'static str {
        "SubstrateMetadata"
    }
}

/// Hash a length-delimited field.
///
/// Length delimiting prevents ambiguous concatenation of metadata fields.
fn hash_field(
    hasher: &mut Sha256,
    value: &[u8],
) {
    hasher.update(
        (value.len() as u128).to_be_bytes(),
    );

    hasher.update(value);
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// SSBE operational configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsbeConfig {
    /// Maximum generated source size in bytes.
    ///
    /// Defaults to `usize::MAX`, meaning no additional SSBE-specific
    /// source-size ceiling.
    ///
    /// The process remains subject to actual memory and operating-system
    /// resource availability.
    pub max_generated_source_bytes: usize,

    /// Whether an existing generated file may be replaced.
    ///
    /// Defaults to false.
    pub allow_overwrite: bool,
}

impl Default for SsbeConfig {
    fn default() -> Self {
        Self {
            max_generated_source_bytes: usize::MAX,
            allow_overwrite: false,
        }
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured SSBE errors.
#[derive(Debug)]
pub enum SsbeError {
    /// Invalid substrate metadata.
    InvalidDefinition(String),

    /// Invalid output directory.
    InvalidOutputDirectory(PathBuf),

    /// Existing output cannot be replaced.
    OutputExists(PathBuf),

    /// Generated source exceeds the explicitly configured budget.
    GeneratedSourceTooLarge {
        /// Actual generated size.
        size: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Allocation failed while constructing generated source.
    Allocation {
        /// Operation being performed.
        operation: &'static str,
    },

    /// Filesystem operation failed.
    Io {
        /// Operation being performed.
        operation: &'static str,

        /// Affected path.
        path: PathBuf,

        /// Original operating-system error.
        source: io::Error,
    },

    /// Generated output could not be installed safely.
    Installation {
        /// Destination path.
        path: PathBuf,

        /// Original operating-system error.
        source: io::Error,
    },
}

impl fmt::Display for SsbeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDefinition(message) => {
                write!(
                    formatter,
                    "SSBE invalid substrate definition: {message}"
                )
            }

            Self::InvalidOutputDirectory(path) => {
                write!(
                    formatter,
                    "SSBE invalid output directory: '{}'",
                    path.display()
                )
            }

            Self::OutputExists(path) => {
                write!(
                    formatter,
                    "SSBE output already exists: '{}'",
                    path.display()
                )
            }

            Self::GeneratedSourceTooLarge {
                size,
                maximum,
            } => {
                write!(
                    formatter,
                    "Generated source size {size} exceeds configured budget {maximum}"
                )
            }

            Self::Allocation { operation } => {
                write!(
                    formatter,
                    "SSBE allocation failed while {operation}"
                )
            }

            Self::Io {
                operation,
                path,
                source,
            } => {
                write!(
                    formatter,
                    "SSBE I/O failure while {operation} '{}': {source}",
                    path.display()
                )
            }

            Self::Installation {
                path,
                source,
            } => {
                write!(
                    formatter,
                    "SSBE could not safely install '{}': {source}",
                    path.display()
                )
            }
        }
    }
}

impl Error for SsbeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Installation { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ============================================================================
// RESULT
// ============================================================================

/// Result of successful metadata synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisResult {
    /// Absolute path of the generated Rust source.
    pub path: PathBuf,

    /// Generated source size in bytes.
    pub bytes_written: usize,

    /// Number of metadata instructions represented.
    pub instruction_count: usize,

    /// Deterministic substrate identity.
    pub substrate_fingerprint: String,

    /// SHA-256 digest of the generated source.
    pub source_digest: String,
}

// ============================================================================
// FALLIBLE SOURCE BUILDER
// ============================================================================

/// Fallible source builder with configurable resource accounting.
struct SourceBuilder {
    source: String,
    maximum: usize,
}

impl SourceBuilder {
    fn new(maximum: usize) -> Self {
        Self {
            source: String::new(),
            maximum,
        }
    }

    fn push(
        &mut self,
        value: &str,
    ) -> Result<(), SsbeError> {
        let new_size = self.source.len()
            .checked_add(value.len())
            .ok_or(SsbeError::GeneratedSourceTooLarge {
                size: usize::MAX,
                maximum: self.maximum,
            })?;

        if new_size > self.maximum {
            return Err(SsbeError::GeneratedSourceTooLarge {
                size: new_size,
                maximum: self.maximum,
            });
        }

        self.source
            .try_reserve(value.len())
            .map_err(|_| SsbeError::Allocation {
                operation: "reserving generated source memory",
            })?;

        self.source.push_str(value);

        Ok(())
    }

    fn push_rust_string(
        &mut self,
        value: &str,
    ) -> Result<(), SsbeError> {
        self.push("\"")?;

        for character in value.chars() {
            match character {
                '\\' => self.push("\\\\")?,
                '"' => self.push("\\\"")?,
                '\n' => self.push("\\n")?,
                '\r' => self.push("\\r")?,
                '\t' => self.push("\\t")?,
                '\0' => self.push("\\0")?,

                character if character.is_control() => {
                    self.push("\\u{")?;

                    let escaped = format!(
                        "{:x}",
                        character as u32
                    );

                    self.push(&escaped)?;
                    self.push("}")?;
                }

                character => {
                    let mut encoded = [0_u8; 4];

                    self.push(
                        character.encode_utf8(&mut encoded),
                    )?;
                }
            }
        }

        self.push("\"")
    }

    fn finish(self) -> String {
        self.source
    }
}

// ============================================================================
// ENGINE
// ============================================================================

/// Self-Synthesizing Backend Engine.
///
/// The engine is stateless apart from its immutable configuration.
///
/// Cloned engines share no mutable synthesis state.
#[derive(Debug, Clone)]
pub struct SelfSynthesizingBackendEngine {
    /// Configured output directory.
    ///
    /// Retained as a public string for compatibility with the
    /// previous SSBE interface.
    pub output_dir: String,

    config: SsbeConfig,
}

impl SelfSynthesizingBackendEngine {
    /// Construct SSBE with default configuration.
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: SsbeConfig::default(),
        }
    }

    /// Construct SSBE with explicit configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when the configured source-size budget is zero.
    pub fn with_config(
        output_dir: impl Into<String>,
        config: SsbeConfig,
    ) -> Result<Self, SsbeError> {
        if config.max_generated_source_bytes == 0 {
            return Err(SsbeError::InvalidDefinition(
                "generated source budget must be greater than zero".into(),
            ));
        }

        Ok(Self {
            output_dir: output_dir.into(),
            config,
        })
    }

    /// Return the active immutable configuration.
    pub fn config(&self) -> &SsbeConfig {
        &self.config
    }

    /// Validate and resolve the output directory.
    ///
    /// The directory is created if it does not exist.
    ///
    /// The returned path is canonicalized, preventing generated
    /// filenames from being resolved through an uncanonicalized
    /// relative directory path.
    pub fn validate_output_directory(
        &self,
    ) -> Result<PathBuf, SsbeError> {
        let configured = PathBuf::from(&self.output_dir);

        if configured.as_os_str().is_empty() {
            return Err(
                SsbeError::InvalidOutputDirectory(configured)
            );
        }

        if configured.exists() && !configured.is_dir() {
            return Err(
                SsbeError::InvalidOutputDirectory(configured)
            );
        }

        if !configured.exists() {
            fs::create_dir_all(&configured).map_err(|source| {
                SsbeError::Io {
                    operation: "creating output directory",
                    path: configured.clone(),
                    source,
                }
            })?;
        }

        fs::canonicalize(&configured).map_err(|source| {
            SsbeError::Io {
                operation: "canonicalizing output directory",
                path: configured,
                source,
            }
        })
    }

    /// Generate deterministic backend-provider metadata source.
    ///
    /// This method performs no filesystem operations.
    ///
    /// The resulting Rust source contains metadata only.
    /// It does not implement the canonical Backend trait.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid metadata or exhausted configured
    /// generation budgets.
    pub fn generate_backend_source(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<String, SsbeError> {
        substrate.validate()?;

        let mut builder = SourceBuilder::new(
            self.config.max_generated_source_bytes,
        );

        builder.push(
            "// Generated by Zamani SSBE.\n\
             // This file contains provider metadata only.\n\
             // It is not an executable backend implementation.\n\
             // Generated source requires normal compilation and verification.\n\n",
        )?;

        builder.push("#![forbid(unsafe_code)]\n\n")?;

        builder.push(
            "/// Generated substrate-provider metadata.\n\
             #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
             pub struct SubstrateMetadata;\n\n",
        )?;

        builder.push("impl SubstrateMetadata {\n")?;

        builder.push(
            "    /// Metadata schema version.\n\
             pub const SCHEMA_VERSION: u32 = ",
        )?;

        builder.push(
            &SSBE_METADATA_SCHEMA_VERSION.to_string(),
        )?;

        builder.push(";\n\n")?;

        builder.push(
            "    /// Stable substrate fingerprint.\n\
             pub const FINGERPRINT: &str = ",
        )?;

        builder.push_rust_string(
            &substrate.fingerprint(),
        )?;

        builder.push(";\n\n")?;

        builder.push(
            "    /// Original substrate name.\n\
             pub const SUBSTRATE: &str = ",
        )?;

        builder.push_rust_string(
            &substrate.name,
        )?;

        builder.push(";\n\n")?;

        builder.push(
            "    /// Substrate programming paradigm.\n\
             pub const PARADIGM: &str = ",
        )?;

        builder.push_rust_string(
            &substrate.paradigm,
        )?;

        builder.push(";\n\n")?;

        builder.push(
            "    /// Translation-rule description.\n\
             /// This value is descriptive metadata only.\n\
             pub const TRANSLATION_RULE: &str = ",
        )?;

        builder.push_rust_string(
            &substrate.translation_rule,
        )?;

        builder.push(";\n\n")?;

        builder.push(
            "    /// Descriptive instruction metadata.\n\
             pub const INSTRUCTION_SET: &[&str] = &[\n",
        )?;

        for instruction in &substrate.instruction_set {
            builder.push("        ")?;

            builder.push_rust_string(instruction)?;

            builder.push(",\n")?;
        }

        builder.push("    ];\n")?;

        builder.push("}\n")?;

        Ok(builder.finish())
    }

    /// Synthesize metadata source and install it in the output directory.
    ///
    /// The legacy string-path return type is retained for compatibility.
    ///
    /// Prefer `synthesize_backend_checked` for structured error handling.
    pub fn synthesize_backend(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<String, String> {
        self.synthesize_backend_checked(substrate)
            .map(|result| {
                result.path.to_string_lossy().into_owned()
            })
            .map_err(|error| error.to_string())
    }

    /// Synthesize and safely install metadata source.
    ///
    /// The generated file is written to a unique temporary file.
    ///
    /// The temporary file is fully written and synchronized before
    /// installation.
    ///
    /// With overwrite disabled, installation uses an exclusive
    /// filesystem hard-link operation, preventing a concurrent
    /// process from silently replacing an existing destination.
    ///
    /// With overwrite enabled, the temporary file is atomically
    /// renamed where the operating system supports that operation.
    ///
    /// # Errors
    ///
    /// Returns structured validation, allocation, budget or I/O errors.
    pub fn synthesize_backend_checked(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<SynthesisResult, SsbeError> {
        substrate.validate()?;

        let source = self.generate_backend_source(substrate)?;

        let output_dir = self.validate_output_directory()?;

        let output_path = output_dir.join(
            substrate.backend_filename(),
        );

        if output_path.exists() && !self.config.allow_overwrite {
            return Err(SsbeError::OutputExists(output_path));
        }

        let source_digest = hex::encode(
            Sha256::digest(source.as_bytes()),
        );

        let temporary_path = create_temporary_file(
            &output_dir,
            &substrate.fingerprint(),
        )?;

        let write_result = write_and_sync(
            &temporary_path,
            source.as_bytes(),
        );

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary_path);

            return Err(error);
        }

        let installation = if self.config.allow_overwrite {
            fs::rename(
                &temporary_path,
                &output_path,
            )
        } else {
            fs::hard_link(
                &temporary_path,
                &output_path,
            )
        };

        if let Err(source_error) = installation {
            let _ = fs::remove_file(&temporary_path);

            if source_error.kind() == io::ErrorKind::AlreadyExists {
                return Err(
                    SsbeError::OutputExists(output_path)
                );
            }

            return Err(SsbeError::Installation {
                path: output_path,
                source: source_error,
            });
        }

        if !self.config.allow_overwrite {
            fs::remove_file(&temporary_path).map_err(|source| {
                SsbeError::Io {
                    operation: "removing installed temporary link",
                    path: temporary_path,
                    source,
                }
            })?;
        }

        Ok(SynthesisResult {
            path: output_path,
            bytes_written: source.len(),
            instruction_count: substrate.instruction_set.len(),
            substrate_fingerprint: substrate.fingerprint(),
            source_digest,
        })
    }
}

// ============================================================================
// FILESYSTEM OPERATIONS
// ============================================================================

/// Create a unique temporary file in the destination directory.
///
/// `create_new(true)` ensures that an existing file is never opened
/// or overwritten.
///
/// Temporary files reside on the same filesystem as the destination,
/// allowing atomic installation operations.
fn create_temporary_file(
    directory: &Path,
    fingerprint: &str,
) -> Result<PathBuf, SsbeError> {
    loop {
        let sequence = TEMPORARY_FILE_SEQUENCE.fetch_add(
            1,
            Ordering::Relaxed,
        );

        let mut filename = String::new();

        filename
            .try_reserve(
                TEMPORARY_FILE_PREFIX.len()
                    + fingerprint.len()
                    + 64,
            )
            .map_err(|_| SsbeError::Allocation {
                operation: "constructing temporary filename",
            })?;

        filename.push_str(TEMPORARY_FILE_PREFIX);
        filename.push('-');
        filename.push_str(fingerprint);
        filename.push('-');

        filename.push_str(
            &std::process::id().to_string(),
        );

        filename.push('-');

        filename.push_str(
            &sequence.to_string(),
        );

        filename.push_str(".tmp");

        let path = directory.join(filename);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(_) => return Ok(path),

            Err(error)
                if error.kind() == io::ErrorKind::AlreadyExists =>
            {
                continue;
            }

            Err(source) => {
                return Err(SsbeError::Io {
                    operation: "creating exclusive temporary file",
                    path,
                    source,
                });
            }
        }
    }
}

/// Write and synchronize generated source.
fn write_and_sync(
    path: &Path,
    contents: &[u8],
) -> Result<(), SsbeError> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|source| SsbeError::Io {
            operation: "opening temporary file",
            path: path.to_path_buf(),
            source,
        })?;

    file.write_all(contents).map_err(|source| {
        SsbeError::Io {
            operation: "writing generated source",
            path: path.to_path_buf(),
            source,
        }
    })?;

    file.sync_all().map_err(|source| {
        SsbeError::Io {
            operation: "synchronizing generated source",
            path: path.to_path_buf(),
            source,
        }
    })?;

    Ok(())
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validate required textual metadata.
///
/// No arbitrary maximum byte or character count is imposed.
fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), SsbeError> {
    if value.trim().is_empty() {
        return Err(SsbeError::InvalidDefinition(
            format!("{field} cannot be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(SsbeError::InvalidDefinition(
            format!(
                "{field} contains prohibited control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// PUBLIC HELPERS
// ============================================================================

/// Return the SSBE module identity.
pub const fn module_name() -> &'static str {
    SSBE_MODULE_NAME
}

/// Return the SSBE API version.
pub const fn api_version() -> u16 {
    SSBE_API_VERSION
}

/// Return the generated metadata schema version.
pub const fn metadata_schema_version() -> u32 {
    SSBE_METADATA_SCHEMA_VERSION
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIRECTORY_SEQUENCE: AtomicU64 =
        AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let path = std::env::temp_dir().join(
                format!(
                    "zamani-ssbe-test-{}-{sequence}",
                    std::process::id()
                ),
            );

            fs::create_dir_all(&path)
                .expect("test directory must be created");

            Self { path }
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn sample_definition() -> SubstrateDefinition {
        SubstrateDefinition::new(
            "RISC-V CPU",
            "Classical",
            vec![
                "LOAD".into(),
                "STORE".into(),
                "ADD".into(),
            ],
            "Translate validated classical IR",
        )
        .expect("sample definition must be valid")
    }

    #[test]
    fn definition_validates() {
        let definition = sample_definition();

        assert!(definition.validate().is_ok());
        assert_eq!(definition.instruction_set.len(), 3);
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = SubstrateDefinition::new(
            "",
            "Classical",
            vec!["ADD".into()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_paradigm_is_rejected() {
        let result = SubstrateDefinition::new(
            "CPU",
            "",
            vec!["ADD".into()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_translation_rule_is_rejected() {
        let result = SubstrateDefinition::new(
            "CPU",
            "Classical",
            vec!["ADD".into()],
            "",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_instruction_set_is_rejected() {
        let result = SubstrateDefinition::new(
            "CPU",
            "Classical",
            Vec::new(),
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_instruction_is_rejected() {
        let result = SubstrateDefinition::new(
            "CPU",
            "Classical",
            vec!["".into()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn control_characters_are_rejected() {
        let result = SubstrateDefinition::new(
            "CPU\n",
            "Classical",
            vec!["ADD".into()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn long_metadata_is_supported() {
        let long_value = "x".repeat(16_384);

        let definition = SubstrateDefinition::new(
            long_value.clone(),
            "Classical",
            vec![long_value.clone()],
            long_value,
        );

        assert!(definition.is_ok());
    }

    #[test]
    fn instruction_count_is_not_artificially_limited() {
        let instructions = (0..70_000)
            .map(|index| format!("OP_{index}"))
            .collect();

        let definition = SubstrateDefinition::new(
            "Large substrate",
            "Classical",
            instructions,
            "rule",
        )
        .expect("large instruction set must be accepted");

        assert_eq!(
            definition.instruction_set.len(),
            70_000
        );
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let first = sample_definition();
        let second = sample_definition();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn fingerprint_changes_when_metadata_changes() {
        let first = sample_definition();

        let second = SubstrateDefinition::new(
            "Different CPU",
            "Classical",
            vec!["LOAD".into()],
            "rule",
        )
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn filename_is_deterministic_and_safe() {
        let definition = sample_definition();

        let filename = definition.backend_filename();

        assert!(filename.starts_with("ssbe-"));
        assert!(filename.ends_with(".rs"));
        assert!(!filename.contains('/'));
        assert!(!filename.contains('\\'));

        assert_eq!(
            filename,
            definition.backend_filename()
        );
    }

    #[test]
    fn generated_source_contains_metadata() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new(
            "generated",
        );

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(source.contains("SUBSTRATE"));
        assert!(source.contains("PARADIGM"));
        assert!(source.contains("TRANSLATION_RULE"));
        assert!(source.contains("INSTRUCTION_SET"));
        assert!(source.contains("FINGERPRINT"));
    }

    #[test]
    fn generated_source_does_not_claim_to_implement_backend() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new(
            "generated",
        );

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(
            !source.contains("impl Backend for")
        );

        assert!(
            !source.contains("pub fn emit_code")
        );
    }

    #[test]
    fn generated_source_escapes_metadata() {
        let definition = SubstrateDefinition::new(
            "Quantum \"Core\"",
            "Quantum",
            vec!["H".into()],
            "Apply \"translation\"",
        )
        .unwrap();

        let engine = SelfSynthesizingBackendEngine::new(
            "generated",
        );

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(
            source.contains("Quantum \\\"Core\\\"")
        );

        assert!(
            source.contains("Apply \\\"translation\\\"")
        );
    }

    #[test]
    fn generated_source_is_deterministic() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new(
            "generated",
        );

        let first = engine
            .generate_backend_source(&definition)
            .unwrap();

        let second = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn zero_source_budget_is_rejected() {
        let result = SelfSynthesizingBackendEngine::with_config(
            "generated",
            SsbeConfig {
                max_generated_source_bytes: 0,
                allow_overwrite: false,
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn configured_source_budget_is_enforced() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::with_config(
            "generated",
            SsbeConfig {
                max_generated_source_bytes: 10,
                allow_overwrite: false,
            },
        )
        .unwrap();

        assert!(matches!(
            engine.generate_backend_source(&definition),
            Err(SsbeError::GeneratedSourceTooLarge { .. })
        ));
    }

    #[test]
    fn default_configuration_has_no_arbitrary_source_ceiling() {
        let config = SsbeConfig::default();

        assert_eq!(
            config.max_generated_source_bytes,
            usize::MAX
        );

        assert!(!config.allow_overwrite);
    }

    #[test]
    fn synthesis_creates_metadata_file() {
        let directory = TestDirectory::new();

        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new(
            directory.path.to_str().unwrap(),
        );

        let result = engine
            .synthesize_backend_checked(&definition)
            .unwrap();

        assert!(result.path.is_file());

        assert_eq!(
            result.instruction_count,
            definition.instruction_set.len()
        );

        assert!(result.bytes_written > 0);

        assert!(!result.source_digest.is_empty());
        assert!(!result.substrate_fingerprint.is_empty());
    }

    #[test]
    fn synthesis_refuses_overwrite_by_default() {
        let directory = TestDirectory::new();

        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new(
            directory.path.to_str().unwrap(),
        );

        engine
            .synthesize_backend_checked(&definition)
            .unwrap();

        let result = engine
            .synthesize_backend_checked(&definition);

        assert!(matches!(
            result,
            Err(SsbeError::OutputExists(_))
        ));
    }

    #[test]
    fn explicitly_configured_overwrite_is_supported() {
        let directory = TestDirectory::new();

        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::with_config(
            directory.path.to_string_lossy().into_owned(),
            SsbeConfig {
                max_generated_source_bytes: usize::MAX,
                allow_overwrite: true,
            },
        )
        .unwrap();

        let first = engine
            .synthesize_backend_checked(&definition)
            .unwrap();

        let second = engine
            .synthesize_backend_checked(&definition)
            .unwrap();

        assert_eq!(first.path, second.path);
        assert_eq!(first.source_digest, second.source_digest);
    }

    #[test]
    fn output_directory_is_canonicalized() {
        let directory = TestDirectory::new();

        let engine = SelfSynthesizingBackendEngine::new(
            directory.path.to_str().unwrap(),
        );

        let canonical = engine
            .validate_output_directory()
            .unwrap();

        assert!(canonical.is_absolute());
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.ssbe"
        );

        assert_eq!(api_version(), 2);
        assert_eq!(metadata_schema_version(), 1);
    }
}
