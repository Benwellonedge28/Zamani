
#![forbid(unsafe_code)]

//! LLVM IR verification for Zamani.
//!
//! Path: src/backend/llvm/verification.rs
//!
//! # Responsibilities
//!
//! - Validate non-empty LLVM textual IR using the configured `llvm-as` tool.
//! - Optionally enforce the expected target triple and data layout.
//! - Keep generated input and bitcode in a private temporary workspace.
//! - Return structured verification results and actionable errors.
//! - Never report success when the verifier fails or produces no bitcode.
//! - Avoid retaining the complete bitcode output in memory.
//! - Clean up temporary files on success and ordinary error returns.
//!
//! # Ownership boundaries
//!
//! - `crate::ir_verify` verifies Zamani IR, not LLVM IR.
//! - `crate::compiler::llvm_backend::LlvmBackend` owns the existing
//!   LLVM compilation pipeline and target object generation.
//! - This module verifies LLVM textual IR using an external LLVM tool.
//! - Target selection and authoritative target metadata belong to the
//!   compilation configuration and target description.
//! - Filesystem output of final compilation artifacts belongs to the
//!   compiler's output stage.
//!
//! # Integration
//!
//! ```text
//! verified Zamani IR
//!         |
//!         v
//! LLVM emitter
//!         |
//!         v
//! LlvmIrVerifier::verify
//!         |
//!         v
//! validated LLVM IR
//!         |
//!         v
//! LLVM optimization / object generation
//! ```
//!
//! This verifier does not prove equivalence between Zamani semantics and
//! LLVM semantics. That guarantee requires correct lowering and appropriate
//! end-to-end semantic tests.

use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique suffix source for private temporary workspaces.
///
/// This counter is not a resource limit. Workspace creation also uses
/// atomic filesystem creation, so concurrent verifier instances do not
/// share temporary files.
static NEXT_WORKSPACE_ID: AtomicU64 = AtomicU64::new(0);

/// Options for LLVM textual IR verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlvmVerificationConfig {
    /// Path or executable name for the LLVM IR assembler.
    ///
    /// The default uses the executable found through the process environment.
    /// Hermetic builds should provide an explicit tool path.
    pub llvm_as: PathBuf,

    /// Expected LLVM target triple, if the caller has selected a target.
    ///
    /// When set, the module must declare the exact triple.
    pub expected_target_triple: Option<String>,

    /// Expected LLVM data layout, if known from the selected target/toolchain.
    ///
    /// When set, the module must declare the exact layout.
    pub expected_data_layout: Option<String>,
}

impl Default for LlvmVerificationConfig {
    fn default() -> Self {
        Self {
            llvm_as: PathBuf::from("llvm-as"),
            expected_target_triple: None,
            expected_data_layout: None,
        }
    }
}

impl LlvmVerificationConfig {
    /// Create configuration using an explicit LLVM assembler executable.
    pub fn new(llvm_as: impl Into<PathBuf>) -> Self {
        Self {
            llvm_as: llvm_as.into(),
            ..Self::default()
        }
    }

    /// Require the emitted IR to declare this target triple.
    pub fn with_target_triple(mut self, triple: impl Into<String>) -> Self {
        self.expected_target_triple = Some(triple.into());
        self
    }

    /// Require the emitted IR to declare this data layout.
    pub fn with_data_layout(mut self, layout: impl Into<String>) -> Self {
        self.expected_data_layout = Some(layout.into());
        self
    }

    fn validate(&self) -> Result<(), LlvmVerificationError> {
        if self.llvm_as.as_os_str().is_empty() {
            return Err(LlvmVerificationError::InvalidConfiguration(
                "llvm-as executable path must not be empty".into(),
            ));
        }

        if let Some(triple) = &self.expected_target_triple {
            validate_metadata_value("target triple", triple)?;
        }

        if let Some(layout) = &self.expected_data_layout {
            validate_metadata_value("data layout", layout)?;
        }

        Ok(())
    }
}

/// Structured verification failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlvmVerificationError {
    InvalidConfiguration(String),
    EmptyInput,
    MissingTargetTriple(String),
    TargetTripleMismatch {
        expected: String,
        actual: Option<String>,
    },
    MissingDataLayout(String),
    DataLayoutMismatch {
        expected: String,
        actual: Option<String>,
    },
    Io {
        operation: String,
        path: Option<PathBuf>,
        message: String,
    },
    ToolExecution {
        tool: PathBuf,
        message: String,
    },
    ToolRejectedIr {
        tool: PathBuf,
        status: Option<i32>,
        stdout: String,
        stderr: String,
    },
    MissingBitcode(PathBuf),
    EmptyBitcode(PathBuf),
}

impl fmt::Display for LlvmVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid LLVM verification configuration: {message}")
            }
            Self::EmptyInput => {
                write!(f, "cannot verify empty LLVM IR")
            }
            Self::MissingTargetTriple(expected) => {
                write!(
                    f,
                    "LLVM IR does not declare the required target triple {expected:?}"
                )
            }
            Self::TargetTripleMismatch { expected, actual } => {
                write!(
                    f,
                    "LLVM target triple mismatch: expected {expected:?}, found {actual:?}"
                )
            }
            Self::MissingDataLayout(expected) => {
                write!(
                    f,
                    "LLVM IR does not declare the required data layout {expected:?}"
                )
            }
            Self::DataLayoutMismatch { expected, actual } => {
                write!(
                    f,
                    "LLVM data layout mismatch: expected {expected:?}, found {actual:?}"
                )
            }
            Self::Io {
                operation,
                path,
                message,
            } => {
                if let Some(path) = path {
                    write!(f, "{operation} '{}': {message}", path.display())
                } else {
                    write!(f, "{operation}: {message}")
                }
            }
            Self::ToolExecution { tool, message } => {
                write!(
                    f,
                    "failed to execute LLVM verifier '{}': {message}",
                    tool.display()
                )
            }
            Self::ToolRejectedIr {
                tool,
                status,
                stdout,
                stderr,
            } => {
                write!(
                    f,
                    "LLVM verifier '{}' rejected the IR (exit status {status:?})",
                    tool.display()
                )?;

                if !stderr.trim().is_empty() {
                    write!(f, "\nstderr:\n{}", stderr.trim())?;
                }

                if !stdout.trim().is_empty() {
                    write!(f, "\nstdout:\n{}", stdout.trim())?;
                }

                Ok(())
            }
            Self::MissingBitcode(path) => {
                write!(
                    f,
                    "LLVM assembler did not produce bitcode at '{}'",
                    path.display()
                )
            }
            Self::EmptyBitcode(path) => {
                write!(
                    f,
                    "LLVM assembler produced an empty bitcode file at '{}'",
                    path.display()
                )
            }
        }
    }
}

impl Error for LlvmVerificationError {}

/// Successful verification evidence.
///
/// `llvm_as` accepted the textual IR and generated non-empty bitcode.
/// This result does not establish runtime correctness, target availability,
/// or semantic equivalence with the original Zamani program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlvmVerificationReport {
    /// Executable configured to perform the verification.
    pub verifier: PathBuf,

    /// Number of bytes in the textual LLVM IR.
    pub input_bytes: u64,

    /// Number of bytes in the generated bitcode.
    pub bitcode_bytes: u64,

    /// Expected target triple checked, if configured.
    pub target_triple: Option<String>,

    /// Expected data layout checked, if configured.
    pub data_layout: Option<String>,
}

/// Verifies LLVM textual IR using an external LLVM assembler.
#[derive(Debug, Clone)]
pub struct LlvmIrVerifier {
    config: LlvmVerificationConfig,
}

impl Default for LlvmIrVerifier {
    fn default() -> Self {
        Self::new(LlvmVerificationConfig::default())
    }
}

impl LlvmIrVerifier {
    /// Create a verifier from explicit configuration.
    pub fn new(config: LlvmVerificationConfig) -> Self {
        Self { config }
    }

    /// Return the verifier configuration.
    pub fn config(&self) -> &LlvmVerificationConfig {
        &self.config
    }

    /// Validate LLVM textual IR.
    ///
    /// Verification proceeds in this order:
    ///
    /// 1. Validate configuration and input.
    /// 2. Check required target metadata.
    /// 3. Write the input to a unique private temporary workspace.
    /// 4. Run `llvm-as` with argument-based invocation.
    /// 5. Require a successful exit status and non-empty bitcode.
    /// 6. Return structured evidence and clean up temporary files.
    ///
    /// The verifier never writes the final application artifact. Temporary
    /// workspace creation is an explicit side effect of this verification API.
    pub fn verify(
        &self,
        llvm_ir: &str,
    ) -> Result<LlvmVerificationReport, LlvmVerificationError> {
        self.config.validate()?;

        if llvm_ir.trim().is_empty() {
            return Err(LlvmVerificationError::EmptyInput);
        }

        validate_target_metadata(&self.config, llvm_ir)?;

        let workspace = TemporaryWorkspace::create()?;
        let input_path = workspace.path().join("module.ll");
        let bitcode_path = workspace.path().join("module.bc");

        write_input(&input_path, llvm_ir.as_bytes())?;

        let output = Command::new(&self.config.llvm_as)
            .arg(&input_path)
            .arg("-o")
            .arg(&bitcode_path)
            .output()
            .map_err(|error| LlvmVerificationError::ToolExecution {
                tool: self.config.llvm_as.clone(),
                message: error.to_string(),
            })?;

        if !output.status.success() {
            return Err(LlvmVerificationError::ToolRejectedIr {
                tool: self.config.llvm_as.clone(),
                status: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        let metadata = fs::metadata(&bitcode_path).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                LlvmVerificationError::MissingBitcode(bitcode_path.clone())
            } else {
                LlvmVerificationError::Io {
                    operation: "inspect generated LLVM bitcode".into(),
                    path: Some(bitcode_path.clone()),
                    message: error.to_string(),
                }
            }
        })?;

        if !metadata.is_file() {
            return Err(LlvmVerificationError::MissingBitcode(bitcode_path));
        }

        if metadata.len() == 0 {
            return Err(LlvmVerificationError::EmptyBitcode(bitcode_path));
        }

        Ok(LlvmVerificationReport {
            verifier: self.config.llvm_as.clone(),
            input_bytes: u64::try_from(llvm_ir.len()).unwrap_or(u64::MAX),
            bitcode_bytes: metadata.len(),
            target_triple: self.config.expected_target_triple.clone(),
            data_layout: self.config.expected_data_layout.clone(),
        })
    }
}

/// Validate that the configured metadata values can safely be represented
/// as LLVM string attributes in the generated module.
fn validate_metadata_value(
    name: &str,
    value: &str,
) -> Result<(), LlvmVerificationError> {
    if value.trim().is_empty() {
        return Err(LlvmVerificationError::InvalidConfiguration(format!(
            "{name} must not be empty"
        )));
    }

    if value.chars().any(|ch| ch.is_control() || ch == '"' || ch == '\\') {
        return Err(LlvmVerificationError::InvalidConfiguration(format!(
            "{name} contains a control character or unsupported quoting character"
        )));
    }

    Ok(())
}

/// Check target metadata before invoking LLVM.
///
/// This is an exact declaration check. It does not independently determine
/// whether a target triple is supported by the installed LLVM build.
fn validate_target_metadata(
    config: &LlvmVerificationConfig,
    llvm_ir: &str,
) -> Result<(), LlvmVerificationError> {
    if let Some(expected) = &config.expected_target_triple {
        let actual = find_quoted_assignment(llvm_ir, "target triple");

        match actual {
            None => {
                return Err(LlvmVerificationError::MissingTargetTriple(
                    expected.clone(),
                ));
            }
            Some(actual) if actual != expected => {
                return Err(LlvmVerificationError::TargetTripleMismatch {
                    expected: expected.clone(),
                    actual: Some(actual),
                });
            }
            Some(_) => {}
        }
    }

    if let Some(expected) = &config.expected_data_layout {
        let actual = find_quoted_assignment(llvm_ir, "target datalayout");

        match actual {
            None => {
                return Err(LlvmVerificationError::MissingDataLayout(
                    expected.clone(),
                ));
            }
            Some(actual) if actual != expected => {
                return Err(LlvmVerificationError::DataLayoutMismatch {
                    expected: expected.clone(),
                    actual: Some(actual),
                });
            }
            Some(_) => {}
        }
    }

    Ok(())
}

/// Read a simple LLVM header assignment such as:
///
/// `target triple = "aarch64-unknown-linux-gnu"`
///
/// LLVM string escape decoding is intentionally not attempted here. Target
/// metadata produced by the Zamani emitter must use the canonical literal
/// representation expected by the selected target descriptor.
fn find_quoted_assignment(ir: &str, key: &str) -> Option<String> {
    for line in ir.lines() {
        let line = line.trim();

        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };

        let Some(rest) = rest.trim_start().strip_prefix('=') else {
            continue;
        };

        let value = rest.trim();

        if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
            return None;
        }

        return Some(value[1..value.len() - 1].to_owned());
    }

    None
}

/// Write all bytes and report the exact operation and path on failure.
fn write_input(path: &Path, bytes: &[u8]) -> Result<(), LlvmVerificationError> {
    let mut file = File::create(path).map_err(|error| LlvmVerificationError::Io {
        operation: "create temporary LLVM input".into(),
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;

    file.write_all(bytes)
        .map_err(|error| LlvmVerificationError::Io {
            operation: "write temporary LLVM input".into(),
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        })?;

    file.flush()
        .map_err(|error| LlvmVerificationError::Io {
            operation: "flush temporary LLVM input".into(),
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        })?;

    Ok(())
}

/// A uniquely created temporary directory that is removed on drop.
///
/// Creation uses `create_dir`, which atomically claims the name and avoids
/// following a pre-existing directory or sharing another build's workspace.
#[derive(Debug)]
struct TemporaryWorkspace {
    path: PathBuf,
}

impl TemporaryWorkspace {
    fn create() -> Result<Self, LlvmVerificationError> {
        let base = std::env::temp_dir();

        loop {
            let id = NEXT_WORKSPACE_ID.fetch_add(1, Ordering::Relaxed);
            let path = base.join(format!(
                "zamani-llvm-verify-{}-{id}",
                std::process::id()
            ));

            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(LlvmVerificationError::Io {
                        operation: "create private LLVM verification workspace".into(),
                        path: Some(path),
                        message: error.to_string(),
                    });
                }
            }
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryWorkspace {
    fn drop(&mut self) {
        // Cleanup is best effort. Drop cannot return an error; callers that
        // require audited cleanup should use the compiler's managed
        // temporary-artifact subsystem and surface cleanup failures there.
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        let verifier = LlvmIrVerifier::default();
        assert_eq!(
            verifier.verify(" \n\t"),
            Err(LlvmVerificationError::EmptyInput)
        );
    }

    #[test]
    fn accepts_matching_target_triple() {
        let config = LlvmVerificationConfig::default()
            .with_target_triple("aarch64-unknown-linux-gnu");

        assert!(validate_target_metadata(
            &config,
            "target triple = \"aarch64-unknown-linux-gnu\"\n"
        )
        .is_ok());
    }

    #[test]
    fn rejects_mismatched_target_triple() {
        let config = LlvmVerificationConfig::default()
            .with_target_triple("aarch64-unknown-linux-gnu");

        assert!(matches!(
            validate_target_metadata(
                &config,
                "target triple = \"x86_64-unknown-linux-gnu\"\n"
            ),
            Err(LlvmVerificationError::TargetTripleMismatch { .. })
        ));
    }

    #[test]
    fn rejects_missing_data_layout() {
        let config = LlvmVerificationConfig::default()
            .with_data_layout("e-m:e-p:64:64");

        assert!(matches!(
            validate_target_metadata(&config, "define void @f() {\n ret void\n}\n"),
            Err(LlvmVerificationError::MissingDataLayout(_))
        ));
    }

    #[test]
    fn rejects_metadata_with_control_characters() {
        assert!(matches!(
            validate_metadata_value("target triple", "x86_64\ninvalid"),
            Err(LlvmVerificationError::InvalidConfiguration(_))
        ));
    }

    #[test]
    fn parses_quoted_assignments() {
        assert_eq!(
            find_quoted_assignment(
                "source_filename = \"module.zm\"\ntarget triple = \"wasm32-unknown-unknown\"\n",
                "target triple"
            ),
            Some("wasm32-unknown-unknown".into())
        );
    }
}
