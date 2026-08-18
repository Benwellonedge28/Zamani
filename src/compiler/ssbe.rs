//! Zamani Compiler — Self-Synthesizing Backend Engine (SSBE)
//!
//! SSBE converts validated substrate specifications into deterministic backend
//! source modules.
//!
//! Important security boundary:
//! SSBE generates source text, but it does not compile or execute that source.
//! Generated backends must subsequently pass the normal Zamani compiler,
//! formatter, static analysis, security checks, and CI pipeline.
//!
//! Design goals:
//! - deterministic backend generation
//! - strict substrate validation
//! - safe output-path handling
//! - protection against path traversal
//! - Rust-source escaping for generated metadata
//! - atomic file replacement
//! - bounded input sizes
//! - structured errors
//! - testable generation without filesystem access
//! - compatibility with the previous public API

use std::fmt;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

/// Maximum number of instructions accepted for one synthesized backend.
pub const MAX_INSTRUCTION_COUNT: usize = 65_536;

/// Maximum length of a single instruction.
pub const MAX_INSTRUCTION_LENGTH: usize = 512;

/// Maximum substrate name length.
pub const MAX_SUBSTRATE_NAME_LENGTH: usize = 128;

/// Maximum paradigm length.
pub const MAX_PARADIGM_LENGTH: usize = 128;

/// Maximum translation-rule length.
pub const MAX_TRANSLATION_RULE_LENGTH: usize = 16_384;

/// Definition of a target execution substrate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstrateDefinition {
    /// Human-readable and machine-identifying substrate name.
    pub name: String,

    /// Programming/architectural paradigm represented by the substrate.
    pub paradigm: String,

    /// Canonical instruction set accepted by the substrate backend.
    pub instruction_set: Vec<String>,

    /// Translation rule used as backend metadata.
    pub translation_rule: String,
}

impl SubstrateDefinition {
    /// Construct a substrate definition and validate it immediately.
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
    pub fn validate(&self) -> Result<(), SsbeError> {
        validate_text_field(
            &self.name,
            "substrate name",
            MAX_SUBSTRATE_NAME_LENGTH,
        )?;

        validate_text_field(
            &self.paradigm,
            "paradigm",
            MAX_PARADIGM_LENGTH,
        )?;

        if self.instruction_set.is_empty() {
            return Err(SsbeError::InvalidDefinition(
                "instruction set cannot be empty".to_string(),
            ));
        }

        if self.instruction_set.len() > MAX_INSTRUCTION_COUNT {
            return Err(SsbeError::InvalidDefinition(format!(
                "instruction set contains {} entries; maximum is {}",
                self.instruction_set.len(),
                MAX_INSTRUCTION_COUNT
            )));
        }

        for (index, instruction) in self.instruction_set.iter().enumerate() {
            validate_instruction(instruction, index)?;
        }

        validate_text_field(
            &self.translation_rule,
            "translation rule",
            MAX_TRANSLATION_RULE_LENGTH,
        )?;

        Ok(())
    }

    /// Return a deterministic filesystem-safe backend filename.
    pub fn backend_filename(&self) -> String {
        format!("{}.rs", sanitize_identifier(&self.name))
    }

    /// Return the deterministic Rust type name for this backend.
    pub fn backend_type_name(&self) -> String {
        format!("{}Backend", rust_type_identifier(&self.name))
    }
}

/// SSBE configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsbeConfig {
    /// Maximum generated source size in bytes.
    pub max_generated_source_bytes: usize,

    /// Whether existing generated files may be replaced.
    pub allow_overwrite: bool,
}

impl Default for SsbeConfig {
    fn default() -> Self {
        Self {
            max_generated_source_bytes: 4 * 1024 * 1024,
            allow_overwrite: true,
        }
    }
}

/// Structured SSBE errors.
#[derive(Debug)]
pub enum SsbeError {
    InvalidDefinition(String),
    InvalidIdentifier(String),
    InvalidOutputDirectory(PathBuf),
    OutputExists(PathBuf),
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    GeneratedSourceTooLarge {
        size: usize,
        maximum: usize,
    },
}

impl fmt::Display for SsbeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDefinition(message) => {
                write!(formatter, "SSBE invalid substrate definition: {message}")
            }

            Self::InvalidIdentifier(message) => {
                write!(formatter, "SSBE invalid identifier: {message}")
            }

            Self::InvalidOutputDirectory(path) => {
                write!(
                    formatter,
                    "SSBE invalid output directory '{}'",
                    path.display()
                )
            }

            Self::OutputExists(path) => {
                write!(
                    formatter,
                    "SSBE output already exists and overwriting is disabled: '{}'",
                    path.display()
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

            Self::GeneratedSourceTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "SSBE generated source is {size} bytes; maximum is {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for SsbeError {}

/// Result of backend synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisResult {
    /// Path of the generated backend source.
    pub path: PathBuf,

    /// Generated source size.
    pub bytes_written: usize,

    /// Number of instructions represented.
    pub instruction_count: usize,
}

/// Self-Synthesizing Backend Engine.
#[derive(Debug, Clone)]
pub struct SelfSynthesizingBackendEngine {
    /// Directory in which generated backend modules are written.
    pub output_dir: String,

    config: SsbeConfig,
}

impl SelfSynthesizingBackendEngine {
    /// Construct an SSBE using default configuration.
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            config: SsbeConfig::default(),
        }
    }

    /// Construct SSBE with explicit configuration.
    pub fn with_config(
        output_dir: impl Into<String>,
        config: SsbeConfig,
    ) -> Result<Self, SsbeError> {
        if config.max_generated_source_bytes == 0 {
            return Err(SsbeError::InvalidDefinition(
                "maximum generated source size must be greater than zero"
                    .to_string(),
            ));
        }

        Ok(Self {
            output_dir: output_dir.into(),
            config,
        })
    }

    /// Return the active configuration.
    pub fn config(&self) -> &SsbeConfig {
        &self.config
    }

    /// Validate the configured output directory.
    pub fn validate_output_directory(&self) -> Result<PathBuf, SsbeError> {
        let path = PathBuf::from(&self.output_dir);

        if path.as_os_str().is_empty() {
            return Err(SsbeError::InvalidOutputDirectory(path));
        }

        if path.exists() && !path.is_dir() {
            return Err(SsbeError::InvalidOutputDirectory(path));
        }

        Ok(path)
    }

    /// Generate backend Rust source without writing it to disk.
    ///
    /// This method is the safest/testable core of SSBE.
    pub fn generate_backend_source(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<String, SsbeError> {
        substrate.validate()?;

        let backend_type = substrate.backend_type_name();

        let mut source = String::with_capacity(
            1024 + substrate.instruction_set.len() * 64,
        );

        source.push_str("//! Zamani Autogenerated Backend.\n");
        source.push_str("//!\n");
        source.push_str("//! This file was generated by SSBE.\n");
        source.push_str("//! Do not edit manually.\n");
        source.push_str("//!\n");
        source.push_str("//! Generation is deterministic for an identical\n");
        source.push_str("//! SubstrateDefinition.\n\n");

        source.push_str("#![allow(dead_code)]\n\n");

        source.push_str("/// Target substrate metadata.\n");
        source.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        source.push_str("pub struct BackendMetadata;\n\n");

        source.push_str("impl BackendMetadata {\n");

        source.push_str("    pub const SUBSTRATE: &str = ");
        append_rust_string_literal(&mut source, &substrate.name);
        source.push_str(";\n");

        source.push_str("    pub const PARADIGM: &str = ");
        append_rust_string_literal(&mut source, &substrate.paradigm);
        source.push_str(";\n");

        source.push_str("    pub const TRANSLATION_RULE: &str = ");
        append_rust_string_literal(&mut source, &substrate.translation_rule);
        source.push_str(";\n");

        source.push_str("    pub const INSTRUCTION_SET: &[&str] = &[\n");

        for instruction in &substrate.instruction_set {
            source.push_str("        ");
            append_rust_string_literal(&mut source, instruction);
            source.push_str(",\n");
        }

        source.push_str("    ];\n");
        source.push_str("}\n\n");

        source.push_str("/// Synthesized backend marker.\n");
        source.push_str("#[derive(Debug, Clone, Copy, Default)]\n");
        source.push_str("pub struct ");
        source.push_str(&backend_type);
        source.push_str(";\n\n");

        source.push_str("impl ");
        source.push_str(&backend_type);
        source.push_str(" {\n");

        source.push_str(
            "    /// Returns the canonical target substrate name.\n",
        );
        source.push_str("    pub const fn substrate_name() -> &'static str {\n");
        source.push_str("        BackendMetadata::SUBSTRATE\n");
        source.push_str("    }\n\n");

        source.push_str(
            "    /// Returns the target paradigm.\n",
        );
        source.push_str("    pub const fn paradigm() -> &'static str {\n");
        source.push_str("        BackendMetadata::PARADIGM\n");
        source.push_str("    }\n\n");

        source.push_str(
            "    /// Returns the canonical instruction set.\n",
        );
        source.push_str(
            "    pub const fn instruction_set() -> &'static [&'static str] {\n",
        );
        source.push_str("        BackendMetadata::INSTRUCTION_SET\n");
        source.push_str("    }\n\n");

        source.push_str(
            "    /// Produces backend metadata for downstream compilation.\n",
        );
        source.push_str(
            "    pub fn emit_code(module_name: &str) -> String {\n",
        );
        source.push_str(
            "        format!(\"{}:{}\", Self::substrate_name(), module_name)\n",
        );
        source.push_str("    }\n");

        source.push_str("}\n");

        if source.len() > self.config.max_generated_source_bytes {
            return Err(SsbeError::GeneratedSourceTooLarge {
                size: source.len(),
                maximum: self.config.max_generated_source_bytes,
            });
        }

        Ok(source)
    }

    /// Synthesize a backend and atomically write it to the configured output
    /// directory.
    ///
    /// The generated file is first written to a temporary file and then
    /// renamed into place. This prevents consumers from observing a partially
    /// generated Rust source file.
    pub fn synthesize_backend(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<String, String> {
        self.synthesize_backend_checked(substrate)
            .map(|result| result.path.to_string_lossy().into_owned())
            .map_err(|error| error.to_string())
    }

    /// Structured version of `synthesize_backend`.
    pub fn synthesize_backend_checked(
        &self,
        substrate: &SubstrateDefinition,
    ) -> Result<SynthesisResult, SsbeError> {
        let output_dir = self.validate_output_directory()?;

        substrate.validate()?;

        let source = self.generate_backend_source(substrate)?;

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).map_err(|source| SsbeError::Io {
                operation: "creating output directory",
                path: output_dir.clone(),
                source,
            })?;
        }

        let file_name = substrate.backend_filename();
        let output_path = output_dir.join(&file_name);

        ensure_path_is_direct_child(&output_dir, &output_path)?;

        if output_path.exists() && !self.config.allow_overwrite {
            return Err(SsbeError::OutputExists(output_path));
        }

        let temporary_path = temporary_path_for(&output_path);

        fs::write(&temporary_path, source.as_bytes()).map_err(|source| {
            SsbeError::Io {
                operation: "writing temporary backend",
                path: temporary_path.clone(),
                source,
            }
        })?;

        if let Err(error) = fs::rename(&temporary_path, &output_path) {
            let _ = fs::remove_file(&temporary_path);

            return Err(SsbeError::Io {
                operation: "atomically installing backend",
                path: output_path,
                source: error,
            });
        }

        Ok(SynthesisResult {
            path: output_path,
            bytes_written: source.len(),
            instruction_count: substrate.instruction_set.len(),
        })
    }
}

/// Ensure a generated file cannot escape the configured output directory.
fn ensure_path_is_direct_child(
    directory: &Path,
    candidate: &Path,
) -> Result<(), SsbeError> {
    let candidate_name = candidate.file_name().ok_or_else(|| {
        SsbeError::InvalidOutputDirectory(candidate.to_path_buf())
    })?;

    if candidate_name.is_empty() {
        return Err(SsbeError::InvalidOutputDirectory(
            candidate.to_path_buf(),
        ));
    }

    for component in Path::new(candidate_name).components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(SsbeError::InvalidIdentifier(
                "generated filename contains an unsafe path component"
                    .to_string(),
            ));
        }
    }

    let expected_parent = directory;

    if candidate.parent() != Some(expected_parent) {
        return Err(SsbeError::InvalidIdentifier(
            "generated backend path escapes the configured output directory"
                .to_string(),
        ));
    }

    Ok(())
}

/// Produce a unique temporary path adjacent to the final output.
fn temporary_path_for(output: &Path) -> PathBuf {
    let mut temporary = output.to_path_buf();

    let extension = match output.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => format!("{extension}.tmp"),
        None => "tmp".to_string(),
    };

    temporary.set_extension(extension);

    temporary
}

/// Convert an arbitrary substrate name into a deterministic safe identifier.
///
/// This identifier is only used for a filename/type name. The original name
/// is preserved separately as escaped metadata.
fn sanitize_identifier(value: &str) -> String {
    let mut result = String::with_capacity(value.len());

    for character in value.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            result.push(character.to_ascii_lowercase());
        } else {
            result.push('_');
        }
    }

    while result.contains("__") {
        result = result.replace("__", "_");
    }

    let result = result.trim_matches('_').to_string();

    if result.is_empty() {
        "generated_backend".to_string()
    } else if result
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        format!("backend_{result}")
    } else {
        result
    }
}

/// Convert a substrate name into a valid Rust type identifier.
fn rust_type_identifier(value: &str) -> String {
    let sanitized = sanitize_identifier(value);

    let mut result = String::with_capacity(sanitized.len());

    let mut uppercase_next = true;

    for character in sanitized.chars() {
        if character == '_' {
            uppercase_next = true;
            continue;
        }

        if uppercase_next {
            result.push(character.to_ascii_uppercase());
            uppercase_next = false;
        } else {
            result.push(character);
        }
    }

    if result.is_empty() {
        "GeneratedBackend".to_string()
    } else if result
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        format!("Backend{result}")
    } else {
        result
    }
}

/// Escape arbitrary text into a valid Rust string literal.
fn append_rust_string_literal(output: &mut String, value: &str) {
    output.push('"');

    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\0' => output.push_str("\\0"),
            character if character.is_control() => {
                output.push_str(&format!("\\u{{{:x}}}", character as u32));
            }
            character => output.push(character),
        }
    }

    output.push('"');
}

/// Validate general textual metadata.
fn validate_text_field(
    value: &str,
    field: &'static str,
    maximum_length: usize,
) -> Result<(), SsbeError> {
    if value.trim().is_empty() {
        return Err(SsbeError::InvalidDefinition(format!(
            "{field} cannot be empty"
        )));
    }

    if value.len() > maximum_length {
        return Err(SsbeError::InvalidDefinition(format!(
            "{field} exceeds maximum length of {maximum_length} bytes"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(SsbeError::InvalidDefinition(format!(
            "{field} contains control characters"
        )));
    }

    Ok(())
}

/// Validate one instruction.
fn validate_instruction(
    instruction: &str,
    index: usize,
) -> Result<(), SsbeError> {
    if instruction.trim().is_empty() {
        return Err(SsbeError::InvalidDefinition(format!(
            "instruction at index {index} is empty"
        )));
    }

    if instruction.len() > MAX_INSTRUCTION_LENGTH {
        return Err(SsbeError::InvalidDefinition(format!(
            "instruction at index {index} exceeds maximum length of {} bytes",
            MAX_INSTRUCTION_LENGTH
        )));
    }

    if instruction.chars().any(char::is_control) {
        return Err(SsbeError::InvalidDefinition(format!(
            "instruction at index {index} contains control characters"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_definition() -> SubstrateDefinition {
        SubstrateDefinition::new(
            "RISC-V CPU",
            "Classical",
            vec![
                "LOAD".to_string(),
                "STORE".to_string(),
                "ADD".to_string(),
            ],
            "lower Zamani IR to RISC-V instructions",
        )
        .unwrap()
    }

    #[test]
    fn substrate_definition_validates() {
        let definition = sample_definition();

        assert!(definition.validate().is_ok());
        assert_eq!(definition.instruction_set.len(), 3);
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = SubstrateDefinition::new(
            "",
            "Classical",
            vec!["ADD".to_string()],
            "rule",
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
            vec!["".to_string()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn filename_is_deterministic() {
        let definition = sample_definition();

        assert_eq!(
            definition.backend_filename(),
            "risc-v_cpu.rs"
        );
    }

    #[test]
    fn type_name_is_generated() {
        let definition = sample_definition();

        assert_eq!(
            definition.backend_type_name(),
            "RiscVCPUBackend"
        );
    }

    #[test]
    fn source_contains_escaped_metadata() {
        let definition = SubstrateDefinition::new(
            "Quantum \"Core\"",
            "Quantum",
            vec!["H".to_string()],
            "apply \"translation\"",
        )
        .unwrap();

        let engine = SelfSynthesizingBackendEngine::new("generated");

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(source.contains("Quantum \\\"Core\\\""));
        assert!(source.contains("apply \\\"translation\\\""));
    }

    #[test]
    fn generated_source_contains_instruction_set() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new("generated");

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(source.contains("\"LOAD\""));
        assert!(source.contains("\"STORE\""));
        assert!(source.contains("\"ADD\""));
    }

    #[test]
    fn path_traversal_is_not_used_as_filename() {
        let definition = SubstrateDefinition::new(
            "../../escape",
            "Classical",
            vec!["ADD".to_string()],
            "rule",
        )
        .unwrap();

        assert_eq!(
            definition.backend_filename(),
            "escape.rs"
        );
    }

    #[test]
    fn invalid_cache_like_control_data_is_rejected() {
        let result = SubstrateDefinition::new(
            "CPU\n",
            "Classical",
            vec!["ADD".to_string()],
            "rule",
        );

        assert!(result.is_err());
    }

    #[test]
    fn configuration_rejects_zero_source_limit() {
        let result = SelfSynthesizingBackendEngine::with_config(
            "generated",
            SsbeConfig {
                max_generated_source_bytes: 0,
                allow_overwrite: true,
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn generated_source_is_deterministic() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new("generated");

        let first = engine
            .generate_backend_source(&definition)
            .unwrap();

        let second = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn synthesis_does_not_execute_generated_code() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new("generated");

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(!source.contains("std::process::Command"));
        assert!(!source.contains("Command::new"));
    }

    #[test]
    fn source_size_limit_is_enforced() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::with_config(
            "generated",
            SsbeConfig {
                max_generated_source_bytes: 10,
                allow_overwrite: true,
            },
        )
        .unwrap();

        let result = engine.generate_backend_source(&definition);

        assert!(matches!(
            result,
            Err(SsbeError::GeneratedSourceTooLarge { .. })
        ));
    }

    #[test]
    fn metadata_is_exposed_by_generated_backend() {
        let definition = sample_definition();

        let engine = SelfSynthesizingBackendEngine::new("generated");

        let source = engine
            .generate_backend_source(&definition)
            .unwrap();

        assert!(source.contains("pub const SUBSTRATE"));
        assert!(source.contains("pub const PARADIGM"));
        assert!(source.contains("pub const TRANSLATION_RULE"));
        assert!(source.contains("pub const INSTRUCTION_SET"));
    }
}