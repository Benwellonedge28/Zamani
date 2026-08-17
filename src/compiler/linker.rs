#![allow(dead_code, unused_variables, unused_imports)]

//! Zamani Compiler — Universal Linker & Link-Time Optimization (ZLink)
//!
//! ZLink is the platform-independent linking layer of the Zamani compiler.
//!
//! Design goals:
//!
//!   Program Once
//!       ↓
//!   Canonical Zamani IR
//!       ↓
//!   Deterministic Link + LTO
//!       ↓
//!   ┌──────────────────────────────────────────────┐
//!   │ Portable Zamani Artifact                    │
//!   │ WASM / Zamani Bytecode / Portable IR         │
//!   └──────────────────────────────────────────────┘
//!       ↓
//!   ┌──────────────────────────────────────────────┐
//!   │ Target-specific native artifact             │
//!   │ x86_64 / ARM64 / RISC-V / GPU / QPU / etc. │
//!   └──────────────────────────────────────────────┘
//!
//! ZLink deliberately keeps platform-independent linking separate from native
//! linker invocation. This allows the same Zamani program to be compiled for
//! multiple targets without changing its source code.
//!
//! The "Program Once → Compile Once → Run Everywhere" objective is implemented
//! through portable Zamani artifacts. Native executables remain target-specific.
//!
//! The linker therefore never claims that one native executable is universally
//! executable on every operating system and CPU architecture.

use crate::ir_gen::IrModule;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Universal target family supported by ZLink.
///
/// Native targets remain explicitly identified while portable targets provide
/// the foundation for "compile once, run anywhere" execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkTarget {
    /// Host/native target selected by the compiler.
    Native {
        triple: String,
    },

    /// WebAssembly target.
    Wasm {
        architecture: WasmArchitecture,
    },

    /// Zamani's portable intermediate bytecode.
    ZamaniBytecode,

    /// Zamani portable execution format.
    Portable,

    /// Explicit custom target.
    Custom {
        triple: String,
        format: String,
    },
}

impl LinkTarget {
    /// Return the target triple where one exists.
    pub fn triple(&self) -> Option<&str> {
        match self {
            Self::Native { triple } => Some(triple),
            Self::Custom { triple, .. } => Some(triple),
            Self::Wasm { .. }
            | Self::ZamaniBytecode
            | Self::Portable => None,
        }
    }

    /// Return a stable artifact format identifier.
    pub fn format(&self) -> &str {
        match self {
            Self::Native { .. } => "native",
            Self::Wasm { .. } => "wasm",
            Self::ZamaniBytecode => "zbc",
            Self::Portable => "zportable",
            Self::Custom { format, .. } => format.as_str(),
        }
    }

    /// Whether the target is intrinsically portable across host platforms.
    pub fn is_portable(&self) -> bool {
        matches!(
            self,
            Self::Wasm { .. }
                | Self::ZamaniBytecode
                | Self::Portable
        )
    }
}

/// WebAssembly architecture/profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmArchitecture {
    Wasm32,
    Wasm64,
}

impl Default for WasmArchitecture {
    fn default() -> Self {
        Self::Wasm32
    }
}

/// Linking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkingMode {
    /// Produce a normal executable artifact.
    Executable,

    /// Produce a shared library/module.
    Shared,

    /// Produce a static library/archive.
    Static,

    /// Produce a portable Zamani artifact.
    Portable,

    /// Produce an intermediate object/module.
    Object,
}

impl Default for LinkingMode {
    fn default() -> Self {
        Self::Executable
    }
}

/// Link-time optimization level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LtoLevel {
    None,
    Basic,
    Aggressive,
    Full,
}

impl Default for LtoLevel {
    fn default() -> Self {
        Self::Basic
    }
}

/// Reproducibility settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReproducibilityConfig {
    /// Make ordering deterministic.
    pub deterministic: bool,

    /// Avoid timestamps in generated metadata.
    pub strip_timestamps: bool,
}

impl Default for ReproducibilityConfig {
    fn default() -> Self {
        Self {
            deterministic: true,
            strip_timestamps: true,
        }
    }
}

/// Native linker configuration.
///
/// This is only used when ZLink has to produce a target-specific native
/// artifact. Portable linking does not require an external native linker.
#[derive(Debug, Clone)]
pub struct NativeLinkerConfig {
    /// Linker executable.
    pub executable: String,

    /// Additional library search directories.
    pub library_paths: Vec<PathBuf>,

    /// Libraries to link.
    pub libraries: Vec<String>,

    /// Raw linker arguments.
    pub arguments: Vec<String>,

    /// Optional sysroot used for cross compilation.
    pub sysroot: Option<PathBuf>,
}

impl Default for NativeLinkerConfig {
    fn default() -> Self {
        Self {
            executable: default_linker(),
            library_paths: Vec::new(),
            libraries: Vec::new(),
            arguments: Vec::new(),
            sysroot: None,
        }
    }
}

/// Universal ZLink configuration.
#[derive(Debug, Clone)]
pub struct LinkerConfig {
    /// Destination compilation target.
    pub target: LinkTarget,

    /// Link mode.
    pub mode: LinkingMode,

    /// LTO configuration.
    pub lto: LtoLevel,

    /// Reproducibility configuration.
    pub reproducibility: ReproducibilityConfig,

    /// Optional native linker configuration.
    pub native: NativeLinkerConfig,
}

impl Default for LinkerConfig {
    fn default() -> Self {
        Self {
            target: LinkTarget::Portable,
            mode: LinkingMode::Portable,
            lto: LtoLevel::Basic,
            reproducibility: ReproducibilityConfig::default(),
            native: NativeLinkerConfig::default(),
        }
    }
}

/// Result of the platform-independent linking stage.
#[derive(Debug, Clone)]
pub struct LinkResult {
    /// Fully linked Zamani IR.
    pub module: IrModule,

    /// Target selected for this link.
    pub target: LinkTarget,

    /// Number of input modules.
    pub input_modules: usize,

    /// Number of functions in the resulting module.
    pub function_count: usize,

    /// Number of globals in the resulting module.
    pub global_count: usize,

    /// Number of instructions after LTO.
    pub instruction_count: usize,
}

impl LinkResult {
    fn from_module(
        module: IrModule,
        target: LinkTarget,
        input_modules: usize,
    ) -> Self {
        Self {
            function_count: module.functions.len(),
            global_count: module.globals.len(),
            instruction_count: module.instruction_count(),
            module,
            target,
            input_modules,
        }
    }
}

/// ZLink errors.
///
/// Keeping errors structured internally makes the linker easier to integrate
/// with compiler diagnostics later.
#[derive(Debug)]
pub enum LinkerError {
    EmptyInput,
    InvalidTarget(String),
    DuplicateSymbol(String),
    MissingOutput(PathBuf),
    Io(String),
    NativeLinker(String),
}

impl fmt::Display for LinkerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => {
                write!(formatter, "ZLink: no input modules were provided")
            }

            Self::InvalidTarget(message) => {
                write!(formatter, "ZLink: invalid target: {}", message)
            }

            Self::DuplicateSymbol(symbol) => {
                write!(formatter, "ZLink: duplicate symbol '{}'", symbol)
            }

            Self::MissingOutput(path) => {
                write!(
                    formatter,
                    "ZLink: linker completed but output '{}' does not exist",
                    path.display()
                )
            }

            Self::Io(message) => {
                write!(formatter, "ZLink I/O error: {}", message)
            }

            Self::NativeLinker(message) => {
                write!(formatter, "ZLink native linker error: {}", message)
            }
        }
    }
}

impl std::error::Error for LinkerError {}

/// Universal Zamani linker.
#[derive(Debug, Clone)]
pub struct ZamaniLinker {
    pub modules: Vec<IrModule>,
    pub config: LinkerConfig,
}

impl ZamaniLinker {
    /// Construct a linker using the portable default.
    ///
    /// Portable linking is preferred because it avoids coupling the canonical
    /// Zamani artifact to the host operating system.
    pub fn new(modules: Vec<IrModule>) -> Self {
        Self {
            modules,
            config: LinkerConfig::default(),
        }
    }

    /// Construct a linker with an explicit target.
    pub fn with_target(
        modules: Vec<IrModule>,
        target: LinkTarget,
    ) -> Self {
        let mode = if target.is_portable() {
            LinkingMode::Portable
        } else {
            LinkingMode::Executable
        };

        Self {
            modules,
            config: LinkerConfig {
                target,
                mode,
                ..LinkerConfig::default()
            },
        }
    }

    /// Construct a fully configured linker.
    pub fn with_config(
        modules: Vec<IrModule>,
        config: LinkerConfig,
    ) -> Result<Self, LinkerError> {
        validate_config(&config)?;

        Ok(Self { modules, config })
    }

    /// Set the target.
    pub fn set_target(&mut self, target: LinkTarget) -> Result<(), LinkerError> {
        let mut config = self.config.clone();
        config.target = target;

        validate_config(&config)?;

        self.config = config;

        Ok(())
    }

    /// Set LTO level.
    pub fn set_lto(&mut self, level: LtoLevel) {
        self.config.lto = level;
    }

    /// Perform deterministic platform-independent linking.
    ///
    /// This is the fundamental ZLink operation. No native linker is invoked.
    pub fn link(&self) -> Result<IrModule, LinkerError> {
        Ok(self.link_with_result()?.module)
    }

    /// Link and return complete metadata about the resulting artifact.
    pub fn link_with_result(&self) -> Result<LinkResult, LinkerError> {
        if self.modules.is_empty() {
            return Err(LinkerError::EmptyInput);
        }

        validate_config(&self.config)?;

        println!(
            "[ZLink] Linking {} Zamani IR module(s) for target '{}'",
            self.modules.len(),
            self.config.target.format()
        );

        let mut linked_module = IrModule::new("Zamani_Linked");

        let mut function_symbols = HashSet::new();
        let mut global_symbols = HashSet::new();
        let mut strings = HashSet::new();

        /*
         * Deterministic module traversal.
         *
         * The caller-provided module order is preserved. Symbols are checked
         * before insertion so accidental collisions cannot silently overwrite
         * program components.
         */
        for module in &self.modules {
            for function in &module.functions {
                if !function_symbols.insert(function.name.clone()) {
                    return Err(LinkerError::DuplicateSymbol(
                        function.name.clone(),
                    ));
                }

                linked_module.add_function(function.clone());
            }

            for global in &module.globals {
                let name = global.name.clone();

                if !global_symbols.insert(name.clone()) {
                    return Err(LinkerError::DuplicateSymbol(name));
                }

                linked_module.add_global(global.clone());
            }

            for literal in &module.string_literals {
                if strings.insert(literal.clone()) {
                    linked_module.string_literals.push(literal.clone());
                }
            }
        }

        /*
         * LTO happens after all modules have been merged.
         *
         * This is important because optimizations such as dead-code removal
         * and duplicate elimination require whole-program visibility.
         */
        self.optimize_lto(&mut linked_module);

        println!(
            "  -> Linked functions: {}",
            linked_module.functions.len()
        );

        println!(
            "  -> Linked globals: {}",
            linked_module.globals.len()
        );

        println!(
            "  -> Final instructions: {}",
            linked_module.instruction_count()
        );

        Ok(LinkResult::from_module(
            linked_module,
            self.config.target.clone(),
            self.modules.len(),
        ))
    }

    /// Run deterministic link-time optimization.
    ///
    /// This method only performs transformations that are safe at the current
    /// IR abstraction level. More aggressive optimizations should eventually
    /// be implemented by dedicated IR optimization passes.
    pub fn optimize_lto(&self, module: &mut IrModule) {
        if self.config.lto == LtoLevel::None {
            return;
        }

        let initial_count = module.instruction_count();

        println!(
            "[ZLink-LTO] Running {:?} link-time optimization...",
            self.config.lto
        );

        /*
         * Remove duplicate functions by symbol name.
         *
         * The primary link stage already rejects duplicate symbols. This
         * operation therefore acts as a defensive normalization step for
         * modules constructed or modified after linking.
         */
        let mut seen_functions = HashSet::new();

        module.functions.retain(|function| {
            seen_functions.insert(function.name.clone())
        });

        /*
         * Remove duplicate globals using their symbol names.
         */
        let mut seen_globals = HashSet::new();

        module.globals.retain(|global| {
            seen_globals.insert(global.name.clone())
        });

        /*
         * Deduplicate string literals while preserving deterministic order.
         */
        let mut seen_strings = HashSet::new();

        module.string_literals.retain(|literal| {
            seen_strings.insert(literal.clone())
        });

        let final_count = module.instruction_count();

        println!(
            "  -> LTO complete: instructions {} -> {}",
            initial_count,
            final_count
        );
    }

    /// Link to a portable Zamani artifact representation.
    ///
    /// The canonical IR remains independent from CPU architecture and host
    /// operating system. A future serializer can persist this representation
    /// as `.zportable` or another stable Zamani artifact format.
    pub fn link_portable(&self) -> Result<IrModule, LinkerError> {
        let mut config = self.config.clone();

        config.target = LinkTarget::Portable;
        config.mode = LinkingMode::Portable;

        let linker = Self {
            modules: self.modules.clone(),
            config,
        };

        linker.link()
    }

    /// Link for WebAssembly.
    ///
    /// This produces a target-selected IR module. Actual Wasm binary emission
    /// belongs to `wasm_backend`, keeping target-specific code outside ZLink.
    pub fn link_wasm(
        &self,
        architecture: WasmArchitecture,
    ) -> Result<IrModule, LinkerError> {
        let mut config = self.config.clone();

        config.target = LinkTarget::Wasm { architecture };
        config.mode = LinkingMode::Portable;

        let linker = Self {
            modules: self.modules.clone(),
            config,
        };

        linker.link()
    }

    /// Link for a specific native target.
    ///
    /// This is cross-compilation aware: the target triple describes the
    /// destination rather than the machine running Zamani.
    pub fn link_native(
        &self,
        target_triple: impl Into<String>,
    ) -> Result<IrModule, LinkerError> {
        let target = LinkTarget::Native {
            triple: target_triple.into(),
        };

        let mut config = self.config.clone();

        config.target = target;
        config.mode = LinkingMode::Object;

        let linker = Self {
            modules: self.modules.clone(),
            config,
        };

        linker.link()
    }

    /// Invoke an external native linker.
    ///
    /// ZLink does not assume that the host and destination platform are the
    /// same. The configured target and sysroot are passed explicitly.
    pub fn link_native_artifact(
        &self,
        object_files: &[PathBuf],
        output_path: impl AsRef<Path>,
    ) -> Result<(), LinkerError> {
        let output_path = output_path.as_ref();

        if object_files.is_empty() {
            return Err(LinkerError::EmptyInput);
        }

        let triple = self
            .config
            .target
            .triple()
            .ok_or_else(|| {
                LinkerError::InvalidTarget(
                    "native artifact linking requires a native target triple"
                        .to_string(),
                )
            })?;

        validate_output_path(output_path)?;

        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|error| {
                    LinkerError::Io(format!(
                        "failed to create '{}': {}",
                        parent.display(),
                        error
                    ))
                })?;
            }
        }

        let mut command = Command::new(&self.config.native.executable);

        /*
         * Target selection.
         *
         * Different native linkers use different target flags. The
         * environment-specific linker configuration can provide those flags
         * through `arguments`. We deliberately do not invent platform-specific
         * flags here.
         */
        command.arg("-o").arg(output_path);

        if let Some(sysroot) = &self.config.native.sysroot {
            command.arg("--sysroot").arg(sysroot);
        }

        for path in &self.config.native.library_paths {
            command.arg("-L").arg(path);
        }

        for library in &self.config.native.libraries {
            command.arg("-l").arg(library);
        }

        for argument in &self.config.native.arguments {
            command.arg(argument);
        }

        for object in object_files {
            if !object.exists() {
                return Err(LinkerError::Io(format!(
                    "object file '{}' does not exist",
                    object.display()
                )));
            }

            command.arg(object);
        }

        println!(
            "[ZLink] Native linking target '{}' using '{}'",
            triple,
            self.config.native.executable
        );

        let output = command.output().map_err(|error| {
            LinkerError::NativeLinker(format!(
                "failed to execute '{}': {}",
                self.config.native.executable,
                error
            ))
        })?;

        check_linker_output(&self.config.native.executable, output)?;

        if !output_path.exists() {
            return Err(LinkerError::MissingOutput(
                output_path.to_path_buf(),
            ));
        }

        let metadata = std::fs::metadata(output_path).map_err(|error| {
            LinkerError::Io(format!(
                "failed to inspect '{}': {}",
                output_path.display(),
                error
            ))
        })?;

        if metadata.len() == 0 {
            return Err(LinkerError::Io(format!(
                "linker produced empty output '{}'",
                output_path.display()
            )));
        }

        Ok(())
    }

    /// Number of input modules.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Whether the current target is portable.
    pub fn is_portable_target(&self) -> bool {
        self.config.target.is_portable()
    }
}

/// Validate the complete linker configuration.
fn validate_config(config: &LinkerConfig) -> Result<(), LinkerError> {
    match &config.target {
        LinkTarget::Native { triple } => {
            if triple.trim().is_empty() {
                return Err(LinkerError::InvalidTarget(
                    "native target triple cannot be empty".to_string(),
                ));
            }

            if triple.chars().any(char::is_whitespace) {
                return Err(LinkerError::InvalidTarget(format!(
                    "native target triple contains whitespace: '{}'",
                    triple
                )));
            }
        }

        LinkTarget::Custom { triple, format } => {
            if triple.trim().is_empty() {
                return Err(LinkerError::InvalidTarget(
                    "custom target triple cannot be empty".to_string(),
                ));
            }

            if format.trim().is_empty() {
                return Err(LinkerError::InvalidTarget(
                    "custom target format cannot be empty".to_string(),
                ));
            }
        }

        LinkTarget::Wasm { .. }
        | LinkTarget::ZamaniBytecode
        | LinkTarget::Portable => {}
    }

    if config.target.is_portable()
        && matches!(
            config.mode,
            LinkingMode::Executable
                | LinkingMode::Shared
                | LinkingMode::Static
        )
    {
        return Err(LinkerError::InvalidTarget(
            "portable targets cannot use native executable/shared/static linking modes"
                .to_string(),
        ));
    }

    Ok(())
}

/// Validate the final output path.
fn validate_output_path(path: &Path) -> Result<(), LinkerError> {
    if path.as_os_str().is_empty() {
        return Err(LinkerError::Io(
            "output path cannot be empty".to_string(),
        ));
    }

    if path.exists() && path.is_dir() {
        return Err(LinkerError::Io(format!(
            "output path '{}' is a directory",
            path.display()
        )));
    }

    Ok(())
}

/// Validate a native linker process result.
fn check_linker_output(
    linker: &str,
    output: Output,
) -> Result<(), LinkerError> {
    if output.status.success() {
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let status = output
        .status
        .code()
        .map(|code| code.to_string())
        .unwrap_or_else(|| "terminated by signal".to_string());

    let mut message = format!(
        "linker '{}' failed with status {}",
        linker, status
    );

    if !stderr.trim().is_empty() {
        message.push_str(&format!(
            "\nstderr:\n{}",
            stderr.trim()
        ));
    }

    if !stdout.trim().is_empty() {
        message.push_str(&format!(
            "\nstdout:\n{}",
            stdout.trim()
        ));
    }

    Err(LinkerError::NativeLinker(message))
}

/// Select a conventional system linker.
///
/// The target-specific toolchain can override this through
/// `NativeLinkerConfig`.
fn default_linker() -> String {
    if cfg!(target_os = "windows") {
        "clang".to_string()
    } else {
        "cc".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_target_is_portable() {
        assert!(LinkTarget::Portable.is_portable());
        assert!(LinkTarget::ZamaniBytecode.is_portable());
        assert!(
            LinkTarget::Wasm {
                architecture: WasmArchitecture::Wasm32
            }
            .is_portable()
        );
    }

    #[test]
    fn native_target_is_not_portable() {
        assert!(
            !LinkTarget::Native {
                triple: "x86_64-unknown-linux-gnu".to_string()
            }
            .is_portable()
        );
    }

    #[test]
    fn target_triple_is_preserved() {
        let target = LinkTarget::Native {
            triple: "aarch64-unknown-linux-gnu".to_string(),
        };

        assert_eq!(
            target.triple(),
            Some("aarch64-unknown-linux-gnu")
        );
    }

    #[test]
    fn portable_format_is_stable() {
        assert_eq!(LinkTarget::Portable.format(), "zportable");
        assert_eq!(LinkTarget::ZamaniBytecode.format(), "zbc");
    }

    #[test]
    fn empty_native_target_is_rejected() {
        let config = LinkerConfig {
            target: LinkTarget::Native {
                triple: String::new(),
            },
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn whitespace_native_target_is_rejected() {
        let config = LinkerConfig {
            target: LinkTarget::Native {
                triple: "x86_64 unknown".to_string(),
            },
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn portable_target_rejects_native_executable_mode() {
        let config = LinkerConfig {
            target: LinkTarget::Portable,
            mode: LinkingMode::Executable,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn wasm_target_is_valid() {
        let config = LinkerConfig {
            target: LinkTarget::Wasm {
                architecture: WasmArchitecture::Wasm32,
            },
            mode: LinkingMode::Portable,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn native_target_allows_cross_compilation() {
        let config = LinkerConfig {
            target: LinkTarget::Native {
                triple: "aarch64-unknown-linux-gnu".to_string(),
            },
            mode: LinkingMode::Object,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_ok());
    }
}