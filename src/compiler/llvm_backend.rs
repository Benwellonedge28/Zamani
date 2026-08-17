//! Zamani Compiler — LLVM Backend
//!
//! Production-oriented LLVM backend boundary.
//!
//! Responsibilities:
//!   1. Convert the Zamani IR module into LLVM IR.
//!   2. Validate the generated LLVM IR.
//!   3. Invoke the installed LLVM toolchain to produce native output.
//!   4. Propagate compiler/toolchain failures instead of reporting false success.
//!   5. Keep target selection deterministic and explicit.
//!
//! The backend deliberately does not embed LLVM bindings. This keeps the core
//! compiler independent of a particular LLVM Rust binding version while allowing
//! LLVM to be used as the native-code generation backend.
//!
//! Required external tools:
//!   - `llvm-as` for LLVM IR validation/assembly.
//!   - `llc` for native/object-code generation.
//!
//! The exact LLVM binaries can be overridden through the constructor.

use crate::ir_gen::IrModule;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// LLVM native-code backend.
///
/// The backend is intentionally stateless apart from the target triple and
/// LLVM executable locations, making it safe to construct per compilation.
#[derive(Debug, Clone)]
pub struct LlvmBackend {
    /// LLVM target triple, for example:
    /// `x86_64-unknown-linux-gnu`
    pub target_triple: String,

    /// Path/name of the LLVM IR assembler.
    pub llvm_as: String,

    /// Path/name of the LLVM static compiler.
    pub llc: String,

    /// LLVM optimization level.
    pub optimization_level: LlvmOptimizationLevel,
}

/// Supported LLVM optimization levels.
///
/// These map directly to LLVM's conventional `-O0` through `-O3` levels.
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

impl LlvmBackend {
    /// Construct an LLVM backend using the system LLVM installation.
    pub fn new(target_triple: impl Into<String>) -> Self {
        Self {
            target_triple: target_triple.into(),
            llvm_as: "llvm-as".to_string(),
            llc: "llc".to_string(),
            optimization_level: LlvmOptimizationLevel::default(),
        }
    }

    /// Construct a backend with explicitly selected LLVM executables.
    ///
    /// This is useful for CI, hermetic toolchains, cross compilation, and
    /// systems where LLVM is installed outside the normal PATH.
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

    /// Set the LLVM optimization level.
    pub fn with_optimization_level(
        mut self,
        optimization_level: LlvmOptimizationLevel,
    ) -> Self {
        self.optimization_level = optimization_level;
        self
    }

    /// Return the target triple used by this backend.
    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }

    /// Generate native machine/object code from a Zamani IR module.
    ///
    /// The pipeline is:
    ///
    /// ```text
    /// Zamani IR
    ///    ↓
    /// LLVM textual IR
    ///    ↓
    /// llvm-as
    ///    ↓
    /// validated LLVM bitcode
    ///    ↓
    /// llc
    ///    ↓
    /// native object/machine code
    /// ```
    ///
    /// The method never returns `Ok(())` unless the requested output has
    /// actually been produced successfully.
    pub fn emit_machine_code(
        &self,
        module: &IrModule,
        output_path: &str,
    ) -> Result<(), String> {
        self.validate_target()?;

        let output = Path::new(output_path);

        if output.as_os_str().is_empty() {
            return Err("LLVM backend: output path cannot be empty".to_string());
        }

        if let Some(parent) = output.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|error| {
                    format!(
                        "LLVM backend: failed to create output directory '{}': {}",
                        parent.display(),
                        error
                    )
                })?;
            }
        }

        let ir_text = module.to_ir_string();

        if ir_text.trim().is_empty() {
            return Err(format!(
                "LLVM backend: IR module '{}' produced empty LLVM IR",
                module.name
            ));
        }

        let temporary_dir = Self::temporary_directory(output)?;

        let llvm_ir_path = temporary_dir.join(format!("{}.ll", Self::safe_name(&module.name)));
        let bitcode_path =
            temporary_dir.join(format!("{}.bc", Self::safe_name(&module.name)));

        fs::write(&llvm_ir_path, ir_text.as_bytes()).map_err(|error| {
            format!(
                "LLVM backend: failed to write temporary LLVM IR '{}': {}",
                llvm_ir_path.display(),
                error
            )
        })?;

        // Stage 1: assemble and validate LLVM IR.
        self.assemble_ir(&llvm_ir_path, &bitcode_path)?;

        // Stage 2: generate native machine/object code.
        self.run_llc(&bitcode_path, output)?;

        if !output.exists() {
            return Err(format!(
                "LLVM backend: LLVM reported success but output '{}' does not exist",
                output.display()
            ));
        }

        let metadata = fs::metadata(output).map_err(|error| {
            format!(
                "LLVM backend: failed to inspect generated output '{}': {}",
                output.display(),
                error
            )
        })?;

        if metadata.len() == 0 {
            return Err(format!(
                "LLVM backend: generated output '{}' is empty",
                output.display()
            ));
        }

        // Best-effort cleanup. Compilation success must not depend on cleanup.
        let _ = fs::remove_file(&llvm_ir_path);
        let _ = fs::remove_file(&bitcode_path);
        let _ = fs::remove_dir(&temporary_dir);

        Ok(())
    }

    /// Emit LLVM textual IR without invoking the native backend.
    ///
    /// This is useful for debugging, compiler tests, inspection, and later
    /// LLVM optimization stages.
    pub fn emit_llvm_ir(
        &self,
        module: &IrModule,
        output_path: &str,
    ) -> Result<(), String> {
        self.validate_target()?;

        let output = Path::new(output_path);

        if output.as_os_str().is_empty() {
            return Err("LLVM backend: output path cannot be empty".to_string());
        }

        if let Some(parent) = output.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|error| {
                    format!(
                        "LLVM backend: failed to create output directory '{}': {}",
                        parent.display(),
                        error
                    )
                })?;
            }
        }

        let ir_text = module.to_ir_string();

        if ir_text.trim().is_empty() {
            return Err(format!(
                "LLVM backend: module '{}' generated empty LLVM IR",
                module.name
            ));
        }

        fs::write(output, ir_text.as_bytes()).map_err(|error| {
            format!(
                "LLVM backend: failed to write LLVM IR '{}': {}",
                output.display(),
                error
            )
        })?;

        Ok(())
    }

    /// Validate that the configured LLVM target is usable.
    fn validate_target(&self) -> Result<(), String> {
        if self.target_triple.trim().is_empty() {
            return Err("LLVM backend: target triple cannot be empty".to_string());
        }

        if self.target_triple.chars().any(char::is_whitespace) {
            return Err(format!(
                "LLVM backend: target triple contains whitespace: '{}'",
                self.target_triple
            ));
        }

        Ok(())
    }

    /// Assemble LLVM textual IR into validated LLVM bitcode.
    fn assemble_ir(
        &self,
        llvm_ir_path: &Path,
        bitcode_path: &Path,
    ) -> Result<(), String> {
        let output = Command::new(&self.llvm_as)
            .arg(llvm_ir_path)
            .arg("-o")
            .arg(bitcode_path)
            .output()
            .map_err(|error| {
                format!(
                    "LLVM backend: failed to execute '{}': {}. \
                     Ensure LLVM is installed and llvm-as is available.",
                    self.llvm_as, error
                )
            })?;

        Self::check_command(
            &format!("LLVM assembler '{}'", self.llvm_as),
            output,
        )?;

        if !bitcode_path.exists() {
            return Err(format!(
                "LLVM backend: llvm-as completed successfully but '{}' was not created",
                bitcode_path.display()
            ));
        }

        Ok(())
    }

    /// Generate native object/machine code using LLVM's `llc`.
    fn run_llc(
        &self,
        bitcode_path: &Path,
        output_path: &Path,
    ) -> Result<(), String> {
        let output = Command::new(&self.llc)
            .arg(bitcode_path)
            .arg("-mtriple")
            .arg(&self.target_triple)
            .arg(self.optimization_level.as_flag())
            .arg("-filetype=obj")
            .arg("-o")
            .arg(output_path)
            .output()
            .map_err(|error| {
                format!(
                    "LLVM backend: failed to execute '{}': {}. \
                     Ensure LLVM is installed and llc is available.",
                    self.llc, error
                )
            })?;

        Self::check_command(
            &format!("LLVM code generator '{}'", self.llc),
            output,
        )
    }

    /// Convert process failure into a useful compiler diagnostic.
    fn check_command(
        command_name: &str,
        output: Output,
    ) -> Result<(), String> {
        if output.status.success() {
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let mut message = format!(
            "{} failed with exit status {}.",
            command_name,
            output
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "terminated by signal".to_string())
        );

        if !stderr.trim().is_empty() {
            message.push_str(&format!("\nLLVM stderr:\n{}", stderr.trim()));
        }

        if !stdout.trim().is_empty() {
            message.push_str(&format!("\nLLVM stdout:\n{}", stdout.trim()));
        }

        Err(format!("LLVM backend: {}", message))
    }

    /// Create a private temporary directory adjacent to the requested output.
    ///
    /// Keeping temporary files near the output avoids crossing filesystem
    /// boundaries during compilation and makes cleanup deterministic.
    fn temporary_directory(output: &Path) -> Result<PathBuf, String> {
        let parent = output.parent().unwrap_or_else(|| Path::new("."));

        let file_name = output
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("zamani-output");

        let directory = parent.join(format!(".{}.llvm-tmp", file_name));

        if directory.exists() {
            fs::remove_dir_all(&directory).map_err(|error| {
                format!(
                    "LLVM backend: failed to remove stale temporary directory '{}': {}",
                    directory.display(),
                    error
                )
            })?;
        }

        fs::create_dir_all(&directory).map_err(|error| {
            format!(
                "LLVM backend: failed to create temporary directory '{}': {}",
                directory.display(),
                error
            )
        })?;

        Ok(directory)
    }

    /// Produce a filesystem-safe module name for temporary artifacts.
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

        if sanitized.is_empty() {
            "zamani_module".to_string()
        } else {
            sanitized
        }
    }
}

impl Default for LlvmBackend {
    fn default() -> Self {
        Self::new("native")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimization_levels_have_expected_flags() {
        assert_eq!(LlvmOptimizationLevel::O0.as_flag(), "-O0");
        assert_eq!(LlvmOptimizationLevel::O1.as_flag(), "-O1");
        assert_eq!(LlvmOptimizationLevel::O2.as_flag(), "-O2");
        assert_eq!(LlvmOptimizationLevel::O3.as_flag(), "-O3");
    }

    #[test]
    fn backend_preserves_target_triple() {
        let backend = LlvmBackend::new("x86_64-unknown-linux-gnu");

        assert_eq!(
            backend.target_triple(),
            "x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn backend_defaults_to_o2() {
        let backend = LlvmBackend::new("x86_64-unknown-linux-gnu");

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
    fn empty_target_is_rejected() {
        let backend = LlvmBackend::new("");

        assert!(backend.validate_target().is_err());
    }

    #[test]
    fn whitespace_in_target_is_rejected() {
        let backend = LlvmBackend::new("x86_64 unknown");

        assert!(backend.validate_target().is_err());
    }
}