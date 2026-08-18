//! Zamani Compiler — LLVM Backend
//!
//! Production LLVM backend boundary.
//!
//! Responsibilities:
//!   1. Convert Zamani IR into textual LLVM IR.
//!   2. Validate LLVM IR with `llvm-as`.
//!   3. Generate target-specific object code with `llc`.
//!   4. Never report successful compilation unless the requested artifact
//!      was actually produced and is non-empty.
//!   5. Keep temporary compilation state isolated between concurrent builds.
//!   6. Preserve useful diagnostics from the LLVM toolchain.
//!
//! The backend intentionally uses LLVM command-line tools rather than LLVM
//! Rust bindings. This keeps Zamani's compiler core independent of a specific
//! LLVM binding/version.
//!
//! Required tools:
//!   - llvm-as
//!   - llc
//!
//! The paths to both tools can be explicitly configured for CI, cross
//! compilation, hermetic builds, and non-standard LLVM installations.

use crate::ir_gen::IrModule;

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

/// LLVM native-code backend.
#[derive(Debug, Clone)]
pub struct LlvmBackend {
    /// LLVM target triple.
    pub target_triple: String,

    /// LLVM textual IR assembler.
    pub llvm_as: String,

    /// LLVM static compiler/code generator.
    pub llc: String,

    /// LLVM optimization level.
    pub optimization_level: LlvmOptimizationLevel,
}

/// LLVM optimization level.
///
/// These correspond to LLVM's conventional optimization levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlvmOptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl LlvmOptimizationLevel {
    fn as_flag(self) -> &'static str {
        match self {
            Self::O0 => "-O0",
            Self::O1 => "-O1",
            Self::O2 => "-O2",
            Self::O3 => "-O3",
        }
    }
}

impl Default for LlvmOptimizationLevel {
    fn default() -> Self {
        Self::O2
    }
}

/// Structured LLVM backend errors.
///
/// Keeping errors typed makes this backend easier to integrate with the
/// compiler's diagnostic subsystem later.
#[derive(Debug)]
pub enum LlvmBackendError {
    InvalidTarget(String),
    InvalidOutput(String),
    Io {
        operation: String,
        path: Option<PathBuf>,
        source: io::Error,
    },
    ToolExecution {
        tool: String,
        source: io::Error,
    },
    ToolFailure {
        tool: String,
        status: String,
        stdout: String,
        stderr: String,
    },
    MissingArtifact(PathBuf),
    EmptyArtifact(PathBuf),
    InvalidIr(String),
}

impl fmt::Display for LlvmBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(message) => {
                write!(formatter, "LLVM backend: invalid target: {}", message)
            }

            Self::InvalidOutput(message) => {
                write!(formatter, "LLVM backend: invalid output: {}", message)
            }

            Self::Io {
                operation,
                path,
                source,
            } => {
                if let Some(path) = path {
                    write!(
                        formatter,
                        "LLVM backend: {} '{}': {}",
                        operation,
                        path.display(),
                        source
                    )
                } else {
                    write!(
                        formatter,
                        "LLVM backend: {}: {}",
                        operation,
                        source
                    )
                }
            }

            Self::ToolExecution { tool, source } => {
                write!(
                    formatter,
                    "LLVM backend: failed to execute '{}': {}",
                    tool,
                    source
                )
            }

            Self::ToolFailure {
                tool,
                status,
                stdout,
                stderr,
            } => {
                write!(
                    formatter,
                    "LLVM backend: '{}' failed with status {}",
                    tool,
                    status
                )?;

                if !stderr.trim().is_empty() {
                    write!(
                        formatter,
                        "\nstderr:\n{}",
                        stderr.trim()
                    )?;
                }

                if !stdout.trim().is_empty() {
                    write!(
                        formatter,
                        "\nstdout:\n{}",
                        stdout.trim()
                    )?;
                }

                Ok(())
            }

            Self::MissingArtifact(path) => {
                write!(
                    formatter,
                    "LLVM backend: expected artifact '{}' was not produced",
                    path.display()
                )
            }

            Self::EmptyArtifact(path) => {
                write!(
                    formatter,
                    "LLVM backend: generated artifact '{}' is empty",
                    path.display()
                )
            }

            Self::InvalidIr(message) => {
                write!(
                    formatter,
                    "LLVM backend: invalid LLVM IR: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for LlvmBackendError {}

impl LlvmBackend {
    /// Construct an LLVM backend using the supplied target triple.
    pub fn new(target_triple: impl Into<String>) -> Self {
        Self {
            target_triple: target_triple.into(),
            llvm_as: "llvm-as".to_string(),
            llc: "llc".to_string(),
            optimization_level: LlvmOptimizationLevel::default(),
        }
    }

    /// Construct a backend with explicit LLVM executables.
    pub fn with_tools(
        target_triple: impl Into<String>,
        llvm_as: impl Into<String>,
        llc: impl Into<String>,
    ) -> Self {
        Self {
            target_triple: target_triple.into(),
            llvm_as: llvm_as.into(),
            llc: llc.into(),
            optimization_level: LlvmOptimizationLevel::default(),
        }
    }

    /// Configure the LLVM optimization level.
    pub fn with_optimization_level(
        mut self,
        optimization_level: LlvmOptimizationLevel,
    ) -> Self {
        self.optimization_level = optimization_level;
        self
    }

    /// Return the configured target triple.
    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }

    /// Validate the backend configuration.
    pub fn validate(&self) -> Result<(), LlvmBackendError> {
        self.validate_target()?;

        if self.llvm_as.trim().is_empty() {
            return Err(LlvmBackendError::InvalidTarget(
                "llvm-as executable cannot be empty".to_string(),
            ));
        }

        if self.llc.trim().is_empty() {
            return Err(LlvmBackendError::InvalidTarget(
                "llc executable cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Emit target-specific object code.
    ///
    /// Pipeline:
    ///
    /// ```text
    /// Zamani IR
    ///     |
    ///     v
    /// LLVM textual IR
    ///     |
    ///     v
    /// llvm-as
    ///     |
    ///     v
    /// validated LLVM bitcode
    ///     |
    ///     v
    /// llc
    ///     |
    ///     v
    /// target object file
    /// ```
    ///
    /// The final output is written atomically where possible: LLVM generates
    /// into a private temporary file first, then Zamani replaces the requested
    /// output only after successful validation.
    pub fn emit_machine_code(
        &self,
        module: &IrModule,
        output_path: impl AsRef<Path>,
    ) -> Result<(), LlvmBackendError> {
        self.validate()?;

        let output_path = validate_output_path(output_path.as_ref())?;

        let ir_text = self.generate_ir(module)?;

        let temporary_directory =
            TemporaryDirectory::create(output_path.parent().unwrap_or_else(|| Path::new(".")))?;

        let module_name = Self::safe_name(&module.name);

        let llvm_ir_path =
            temporary_directory.path().join(format!("{}.ll", module_name));

        let bitcode_path =
            temporary_directory.path().join(format!("{}.bc", module_name));

        let object_path =
            temporary_directory.path().join(format!("{}.o", module_name));

        write_file(&llvm_ir_path, ir_text.as_bytes())?;

        self.assemble_ir(&llvm_ir_path, &bitcode_path)?;

        self.run_llc(&bitcode_path, &object_path)?;

        validate_artifact(&object_path)?;

        atomic_replace(&object_path, &output_path)?;

        validate_artifact(&output_path)?;

        Ok(())
    }

    /// Emit textual LLVM IR.
    ///
    /// The generated file is written through a private temporary file and
    /// installed only after the write has completed successfully.
    pub fn emit_llvm_ir(
        &self,
        module: &IrModule,
        output_path: impl AsRef<Path>,
    ) -> Result<(), LlvmBackendError> {
        self.validate()?;

        let output_path = validate_output_path(output_path)?;

        let ir_text = self.generate_ir(module)?;

        let temporary_directory =
            TemporaryDirectory::create(output_path.parent().unwrap_or_else(|| Path::new(".")))?;

        let temporary_output =
            temporary_directory.path().join("module.ll");

        write_file(&temporary_output, ir_text.as_bytes())?;

        validate_artifact(&temporary_output)?;

        atomic_replace(&temporary_output, &output_path)?;

        validate_artifact(&output_path)?;

        Ok(())
    }

    /// Generate LLVM textual IR from a Zamani module.
    fn generate_ir(
        &self,
        module: &IrModule,
    ) -> Result<String, LlvmBackendError> {
        let ir_text = module.to_ir_string();

        if ir_text.trim().is_empty() {
            return Err(LlvmBackendError::InvalidIr(format!(
                "module '{}' produced empty LLVM IR",
                module.name
            )));
        }

        Ok(ir_text)
    }

    /// Validate the configured target triple.
    fn validate_target(&self) -> Result<(), LlvmBackendError> {
        let target = self.target_triple.trim();

        if target.is_empty() {
            return Err(LlvmBackendError::InvalidTarget(
                "target triple cannot be empty".to_string(),
            ));
        }

        if target.chars().any(char::is_whitespace) {
            return Err(LlvmBackendError::InvalidTarget(format!(
                "target triple contains whitespace: '{}'",
                target
            )));
        }

        Ok(())
    }

    /// Assemble textual LLVM IR into LLVM bitcode.
    fn assemble_ir(
        &self,
        llvm_ir_path: &Path,
        bitcode_path: &Path,
    ) -> Result<(), LlvmBackendError> {
        let output = Command::new(&self.llvm_as)
            .arg(llvm_ir_path)
            .arg("-o")
            .arg(bitcode_path)
            .output()
            .map_err(|source| LlvmBackendError::ToolExecution {
                tool: self.llvm_as.clone(),
                source,
            })?;

        check_command(&self.llvm_as, output)?;

        validate_artifact(bitcode_path)?;

        Ok(())
    }

    /// Generate target-specific object code using LLVM `llc`.
    fn run_llc(
        &self,
        bitcode_path: &Path,
        object_path: &Path,
    ) -> Result<(), LlvmBackendError> {
        let output = Command::new(&self.llc)
            .arg(bitcode_path)
            .arg("-mtriple")
            .arg(&self.target_triple)
            .arg(self.optimization_level.as_flag())
            .arg("-filetype=obj")
            .arg("-o")
            .arg(object_path)
            .output()
            .map_err(|source| LlvmBackendError::ToolExecution {
                tool: self.llc.clone(),
                source,
            })?;

        check_command(&self.llc, output)?;

        validate_artifact(object_path)?;

        Ok(())
    }

    /// Produce a filesystem-safe module name.
    fn safe_name(name: &str) -> String {
        let sanitized: String = name
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric()
                    || character == '_'
                    || character == '-'
                    || character == '.'
                {
                    character
                } else {
                    '_'
                }
            })
            .collect();

        let sanitized = sanitized.trim_matches('.');

        if sanitized.is_empty() {
            "zamani_module".to_string()
        } else {
            sanitized.to_string()
        }
    }
}

impl Default for LlvmBackend {
    /// Default backend configuration.
    ///
    /// `native` is deliberately not used as a target triple because LLVM
    /// requires an actual target triple. Callers should normally construct
    /// the backend with the compiler's resolved target triple.
    fn default() -> Self {
        Self {
            target_triple: String::new(),
            llvm_as: "llvm-as".to_string(),
            llc: "llc".to_string(),
            optimization_level: LlvmOptimizationLevel::default(),
        }
    }
}

/// Validate and normalize an output path.
fn validate_output_path(path: &Path) -> Result<PathBuf, LlvmBackendError> {
    if path.as_os_str().is_empty() {
        return Err(LlvmBackendError::InvalidOutput(
            "output path cannot be empty".to_string(),
        ));
    }

    if path.exists() && path.is_dir() {
        return Err(LlvmBackendError::InvalidOutput(format!(
            "output path '{}' is a directory",
            path.display()
        )));
    }

    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|source| LlvmBackendError::Io {
                operation: "resolve current directory".to_string(),
                path: None,
                source,
            })?
            .join(path)
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| LlvmBackendError::Io {
            operation: "create output directory".to_string(),
            path: Some(parent.to_path_buf()),
            source,
        })?;
    }

    Ok(path)
}

/// Write a file completely.
fn write_file(
    path: &Path,
    contents: &[u8],
) -> Result<(), LlvmBackendError> {
    fs::write(path, contents).map_err(|source| LlvmBackendError::Io {
        operation: "write file".to_string(),
        path: Some(path.to_path_buf()),
        source,
    })
}

/// Verify that an artifact exists and is non-empty.
fn validate_artifact(path: &Path) -> Result<(), LlvmBackendError> {
    if !path.exists() {
        return Err(LlvmBackendError::MissingArtifact(
            path.to_path_buf(),
        ));
    }

    let metadata =
        fs::metadata(path).map_err(|source| LlvmBackendError::Io {
            operation: "inspect artifact".to_string(),
            path: Some(path.to_path_buf()),
            source,
        })?;

    if !metadata.is_file() {
        return Err(LlvmBackendError::InvalidOutput(format!(
            "artifact '{}' is not a regular file",
            path.display()
        )));
    }

    if metadata.len() == 0 {
        return Err(LlvmBackendError::EmptyArtifact(
            path.to_path_buf(),
        ));
    }

    Ok(())
}

/// Replace the destination with a successfully generated artifact.
///
/// `rename` is atomic on the same filesystem. If the destination exists,
/// Windows requires removal first, while Unix systems allow replacement.
fn atomic_replace(
    source: &Path,
    destination: &Path,
) -> Result<(), LlvmBackendError> {
    #[cfg(windows)]
    {
        if destination.exists() {
            fs::remove_file(destination).map_err(|source_error| {
                LlvmBackendError::Io {
                    operation: "remove previous output".to_string(),
                    path: Some(destination.to_path_buf()),
                    source: source_error,
                }
            })?;
        }
    }

    fs::rename(source, destination).map_err(|source_error| {
        LlvmBackendError::Io {
            operation: "install generated artifact".to_string(),
            path: Some(destination.to_path_buf()),
            source: source_error,
        }
    })
}

/// Convert an LLVM process result into a structured compiler error.
fn check_command(
    tool: &str,
    output: Output,
) -> Result<(), LlvmBackendError> {
    if output.status.success() {
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    let stderr = String::from_utf8_lossy(&output.stderr)
        .trim()
        .to_string();

    let status = output
        .status
        .code()
        .map(|code| code.to_string())
        .unwrap_or_else(|| "terminated by signal".to_string());

    Err(LlvmBackendError::ToolFailure {
        tool: tool.to_string(),
        status,
        stdout,
        stderr,
    })
}

/// Private temporary compilation directory.
///
/// Each compilation receives a unique directory, preventing concurrent
/// compiler invocations from deleting or overwriting one another's state.
struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    fn create(parent: &Path) -> Result<Self, LlvmBackendError> {
        fs::create_dir_all(parent).map_err(|source| {
            LlvmBackendError::Io {
                operation: "create temporary directory parent".to_string(),
                path: Some(parent.to_path_buf()),
                source,
            }
        })?;

        for attempt in 0..32_u32 {
            let unique = unique_suffix(attempt);

            let path = parent.join(format!(
                ".zamani-llvm-{}",
                unique
            ));

            match fs::create_dir(&path) {
                Ok(()) => {
                    return Ok(Self { path });
                }

                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    continue;
                }

                Err(source) => {
                    return Err(LlvmBackendError::Io {
                        operation: "create LLVM temporary directory".to_string(),
                        path: Some(path),
                        source,
                    });
                }
            }
        }

        Err(LlvmBackendError::Io {
            operation: "allocate unique LLVM temporary directory".to_string(),
            path: Some(parent.to_path_buf()),
            source: io::Error::new(
                io::ErrorKind::AlreadyExists,
                "unable to allocate a unique temporary directory after 32 attempts",
            ),
        })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        // Cleanup is deliberately best-effort. A cleanup failure must never
        // turn an otherwise successful compilation into a failed compilation.
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Generate a collision-resistant temporary-directory suffix.
fn unique_suffix(attempt: u32) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    format!(
        "{}-{}-{}",
        std::process::id(),
        nanos,
        attempt
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimization_levels_have_expected_flags() {
        assert_eq!(
            LlvmOptimizationLevel::O0.as_flag(),
            "-O0"
        );

        assert_eq!(
            LlvmOptimizationLevel::O1.as_flag(),
            "-O1"
        );

        assert_eq!(
            LlvmOptimizationLevel::O2.as_flag(),
            "-O2"
        );

        assert_eq!(
            LlvmOptimizationLevel::O3.as_flag(),
            "-O3"
        );
    }

    #[test]
    fn backend_preserves_target_triple() {
        let backend =
            LlvmBackend::new("x86_64-unknown-linux-gnu");

        assert_eq!(
            backend.target_triple(),
            "x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn backend_defaults_to_o2() {
        let backend =
            LlvmBackend::new("x86_64-unknown-linux-gnu");

        assert_eq!(
            backend.optimization_level,
            LlvmOptimizationLevel::O2
        );
    }

    #[test]
    fn safe_name_removes_unsafe_characters() {
        assert_eq!(
            LlvmBackend::safe_name("hello/world:test"),
            "hello_world_test"
        );
    }

    #[test]
    fn safe_name_handles_empty_names() {
        assert_eq!(
            LlvmBackend::safe_name(""),
            "zamani_module"
        );
    }

    #[test]
    fn safe_name_removes_leading_and_trailing_dots() {
        assert_eq!(
            LlvmBackend::safe_name("..module.."),
            "module"
        );
    }

    #[test]
    fn empty_target_is_rejected() {
        let backend = LlvmBackend::new("");

        assert!(backend.validate_target().is_err());
    }

    #[test]
    fn whitespace_target_is_rejected() {
        let backend =
            LlvmBackend::new("x86_64 unknown");

        assert!(backend.validate_target().is_err());
    }

    #[test]
    fn empty_llvm_as_is_rejected() {
        let backend = LlvmBackend {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            llvm_as: String::new(),
            llc: "llc".to_string(),
            optimization_level:
                LlvmOptimizationLevel::O2,
        };

        assert!(backend.validate().is_err());
    }

    #[test]
    fn empty_llc_is_rejected() {
        let backend = LlvmBackend {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            llvm_as: "llvm-as".to_string(),
            llc: String::new(),
            optimization_level:
                LlvmOptimizationLevel::O2,
        };

        assert!(backend.validate().is_err());
    }

    #[test]
    fn default_backend_requires_explicit_target() {
        let backend = LlvmBackend::default();

        assert!(backend.validate().is_err());
    }

    #[test]
    fn portable_target_is_not_silently_invented() {
        let backend =
            LlvmBackend::new("x86_64-unknown-linux-gnu");

        assert_eq!(
            backend.target_triple(),
            "x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn unique_suffix_changes_between_attempts() {
        let first = unique_suffix(0);
        let second = unique_suffix(1);

        assert_ne!(first, second);
    }
}