//! Zamani Compiler — Universal Linker & Link-Time Optimization (ZLink)
//!
//! ZLink is the platform-independent linking layer of the Zamani compiler.
//!
//! Design:
//!
//!   Program
//!      ↓
//!   Canonical Zamani IR
//!      ↓
//!   Deterministic Link + LTO
//!      ↓
//!   Portable Zamani Artifact
//!      ↓
//!   Target-specific backend/linker
//!
//! ZLink owns:
//! - IR module composition;
//! - symbol validation;
//! - deterministic string-literal merging;
//! - target validation;
//! - link-time normalization;
//! - native linker process isolation.
//!
//! ZLink does NOT itself:
//! - execute generated programs;
//! - invoke a native linker during ordinary portable linking;
//! - perform network access;
//! - infer target-specific ABI rules;
//! - silently overwrite duplicate symbols.
//!
//! Native linker invocation is deliberately isolated in
//! `link_native_artifact`.

use crate::ir_gen::IrModule;

use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

// ============================================================================
// Targets
// ============================================================================

/// Universal target family supported by ZLink.
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

    /// Zamani portable bytecode.
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
    #[must_use]
    pub fn triple(&self) -> Option<&str> {
        match self {
            Self::Native { triple } => Some(triple),
            Self::Custom { triple, .. } => Some(triple),
            Self::Wasm { .. }
            | Self::ZamaniBytecode
            | Self::Portable => None,
        }
    }

    /// Return a stable artifact-format identifier.
    #[must_use]
    pub fn format(&self) -> &str {
        match self {
            Self::Native { .. } => "native",
            Self::Wasm { .. } => "wasm",
            Self::ZamaniBytecode => "zbc",
            Self::Portable => "zportable",
            Self::Custom { format, .. } => format.as_str(),
        }
    }

    /// Whether this target is intrinsically portable.
    #[must_use]
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

// ============================================================================
// Linking configuration
// ============================================================================

/// Linking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkingMode {
    Executable,
    Shared,
    Static,
    Portable,
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
    /// Require deterministic ordering.
    pub deterministic: bool,

    /// Prevent timestamps from being introduced by ZLink metadata.
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
/// Arguments are passed to `Command` as individual arguments. They are never
/// interpreted through a shell.
#[derive(Debug, Clone)]
pub struct NativeLinkerConfig {
    /// Native linker executable.
    pub executable: String,

    /// Additional library search paths.
    pub library_paths: Vec<PathBuf>,

    /// Libraries to link.
    pub libraries: Vec<String>,

    /// Additional linker arguments.
    pub arguments: Vec<String>,

    /// Optional cross-compilation sysroot.
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
    pub target: LinkTarget,
    pub mode: LinkingMode,
    pub lto: LtoLevel,
    pub reproducibility: ReproducibilityConfig,
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

// ============================================================================
// Results
// ============================================================================

/// Result of platform-independent linking.
#[derive(Debug, Clone)]
pub struct LinkResult {
    pub module: IrModule,
    pub target: LinkTarget,
    pub input_modules: usize,
    pub function_count: usize,
    pub global_count: usize,
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

// ============================================================================
// Errors
// ============================================================================

/// Structured ZLink errors.
#[derive(Debug)]
pub enum LinkerError {
    EmptyInput,
    InvalidTarget(String),
    DuplicateSymbol(String),
    DuplicateStringLiteral(String),
    MissingOutput(PathBuf),
    InvalidOutput(PathBuf),
    MissingInput(PathBuf),
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
                write!(formatter, "ZLink: invalid target: {message}")
            }

            Self::DuplicateSymbol(symbol) => {
                write!(formatter, "ZLink: duplicate symbol '{symbol}'")
            }

            Self::DuplicateStringLiteral(name) => {
                write!(
                    formatter,
                    "ZLink: duplicate string-literal symbol '{name}'"
                )
            }

            Self::MissingOutput(path) => {
                write!(
                    formatter,
                    "ZLink: linker completed but output '{}' does not exist",
                    path.display()
                )
            }

            Self::InvalidOutput(path) => {
                write!(
                    formatter,
                    "ZLink: invalid output path '{}'",
                    path.display()
                )
            }

            Self::MissingInput(path) => {
                write!(
                    formatter,
                    "ZLink: input object '{}' does not exist",
                    path.display()
                )
            }

            Self::Io(message) => {
                write!(formatter, "ZLink I/O error: {message}")
            }

            Self::NativeLinker(message) => {
                write!(formatter, "ZLink native linker error: {message}")
            }
        }
    }
}

impl std::error::Error for LinkerError {}

// ============================================================================
// Linker
// ============================================================================

/// Universal Zamani linker.
#[derive(Debug, Clone)]
pub struct ZamaniLinker {
    pub modules: Vec<IrModule>,
    pub config: LinkerConfig,
}

impl ZamaniLinker {
    /// Construct a linker using the portable default.
    #[must_use]
    pub fn new(modules: Vec<IrModule>) -> Self {
        Self {
            modules,
            config: LinkerConfig::default(),
        }
    }

    /// Construct a linker for an explicit target.
    #[must_use]
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

    /// Construct a linker from a fully validated configuration.
    pub fn with_config(
        modules: Vec<IrModule>,
        config: LinkerConfig,
    ) -> Result<Self, LinkerError> {
        validate_config(&config)?;

        Ok(Self { modules, config })
    }

    /// Replace the current target after validation.
    pub fn set_target(
        &mut self,
        target: LinkTarget,
    ) -> Result<(), LinkerError> {
        let mut config = self.config.clone();
        config.target = target;

        /*
         * Keep the linking mode consistent with the target.
         */
        config.mode = if config.target.is_portable() {
            LinkingMode::Portable
        } else {
            config.mode
        };

        validate_config(&config)?;
        self.config = config;

        Ok(())
    }

    /// Set the LTO level.
    pub fn set_lto(&mut self, level: LtoLevel) {
        self.config.lto = level;
    }

    /// Perform deterministic platform-independent linking.
    ///
    /// No native linker process is started by this operation.
    pub fn link(&self) -> Result<IrModule, LinkerError> {
        Ok(self.link_with_result()?.module)
    }

    /// Link and return complete result metadata.
    pub fn link_with_result(&self) -> Result<LinkResult, LinkerError> {
        if self.modules.is_empty() {
            return Err(LinkerError::EmptyInput);
        }

        validate_config(&self.config)?;

        let mut linked_module = IrModule::new("Zamani_Linked");

        /*
         * Preserve the first module's IR target information when possible.
         *
         * ZLink does not invent a target triple for a portable artifact.
         */
        if let Some(first) = self.modules.first() {
            linked_module.target_triple = first.target_triple.clone();
            linked_module.data_layout = first.data_layout.clone();
        }

        let mut function_symbols = HashSet::new();
        let mut global_symbols = HashSet::new();
        let mut string_symbols = HashSet::new();
        let mut string_values = HashSet::new();

        /*
         * Module order is intentionally preserved.
         *
         * ZLink never iterates through a HashMap/HashSet to decide output
         * order. HashSets are used only for membership validation.
         */
        for module in &self.modules {
            /*
             * Functions.
             */
            for function in &module.functions {
                if !function_symbols.insert(function.name.clone()) {
                    return Err(LinkerError::DuplicateSymbol(
                        function.name.clone(),
                    ));
                }

                if global_symbols.contains(&function.name) {
                    return Err(LinkerError::DuplicateSymbol(
                        function.name.clone(),
                    ));
                }

                linked_module.add_function(function.clone());
            }

            /*
             * Globals.
             */
            for global in &module.globals {
                if !global_symbols.insert(global.name.clone()) {
                    return Err(LinkerError::DuplicateSymbol(
                        global.name.clone(),
                    ));
                }

                if function_symbols.contains(&global.name) {
                    return Err(LinkerError::DuplicateSymbol(
                        global.name.clone(),
                    ));
                }

                linked_module.add_global(global.clone());
            }

            /*
             * String literals are represented by:
             *
             *     (symbol_name, literal_value)
             *
             * in the actual Zamani IR.
             *
             * Identical literal values may be shared only when they use the
             * same symbol. Different symbols are preserved because their
             * references may depend on their exact global identity.
             */
            for (name, value) in &module.string_literals {
                if function_symbols.contains(name)
                    || global_symbols.contains(name)
                {
                    return Err(LinkerError::DuplicateSymbol(
                        name.clone(),
                    ));
                }

                if !string_symbols.insert(name.clone()) {
                    return Err(LinkerError::DuplicateStringLiteral(
                        name.clone(),
                    ));
                }

                if string_values.insert((name.clone(), value.clone())) {
                    linked_module
                        .string_literals
                        .push((name.clone(), value.clone()));
                }
            }

            /*
             * Type definitions are module-level declarations. Preserve their
             * source order while rejecting duplicate type names.
             */
            for (name, fields) in &module.type_defs {
                if linked_module
                    .type_defs
                    .iter()
                    .any(|(existing, _)| existing == name)
                {
                    return Err(LinkerError::DuplicateSymbol(
                        name.clone(),
                    ));
                }

                linked_module
                    .type_defs
                    .push((name.clone(), fields.clone()));
            }
        }

        /*
         * LTO runs only after whole-program composition.
         */
        self.optimize_lto(&mut linked_module);

        Ok(LinkResult::from_module(
            linked_module,
            self.config.target.clone(),
            self.modules.len(),
        ))
    }

    /// Run deterministic link-time normalization.
    ///
    /// Duplicate functions/globals are NOT silently removed here. They have
    /// already been treated as linker errors. Removing one would make symbol
    /// resolution dependent on ordering and could change program semantics.
    pub fn optimize_lto(&self, module: &mut IrModule) {
        if self.config.lto == LtoLevel::None {
            return;
        }

        let initial_count = module.instruction_count();

        /*
         * Deduplicate identical string-literal pairs while preserving their
         * first occurrence.
         *
         * Symbol names are already validated during linking, so this is
         * defensive normalization for callers invoking optimize_lto directly.
         */
        let mut seen_strings: HashSet<(String, String)> = HashSet::new();

        module.string_literals.retain(|literal| {
            seen_strings.insert(literal.clone())
        });

        /*
         * At present, actual instruction-level optimization belongs in the
         * dedicated optimizer pipeline. ZLink deliberately avoids performing
         * unsafe transformations based only on symbol names.
         */

        let final_count = module.instruction_count();

        if initial_count != final_count {
            eprintln!(
                "[ZLink-LTO] instruction count: {initial_count} -> {final_count}"
            );
        }
    }

    /// Link to the portable Zamani representation.
    pub fn link_portable(&self) -> Result<IrModule, LinkerError> {
        let mut config = self.config.clone();
        config.target = LinkTarget::Portable;
        config.mode = LinkingMode::Portable;

        Self {
            modules: self.modules.clone(),
            config,
        }
        .link()
    }

    /// Link for WebAssembly.
    ///
    /// Actual Wasm binary emission remains the responsibility of the Wasm
    /// backend.
    pub fn link_wasm(
        &self,
        architecture: WasmArchitecture,
    ) -> Result<IrModule, LinkerError> {
        let mut config = self.config.clone();
        config.target = LinkTarget::Wasm { architecture };
        config.mode = LinkingMode::Portable;

        Self {
            modules: self.modules.clone(),
            config,
        }
        .link()
    }

    /// Link for a specific native target at the IR/object stage.
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

        Self {
            modules: self.modules.clone(),
            config,
        }
        .link()
    }

    /// Invoke the configured native linker.
    ///
    /// Every argument is supplied directly to `std::process::Command`.
    /// No shell is involved, preventing shell metacharacters in paths or
    /// arguments from becoming executable commands.
    pub fn link_native_artifact(
        &self,
        object_files: &[PathBuf],
        output_path: impl AsRef<Path>,
    ) -> Result<(), LinkerError> {
        if object_files.is_empty() {
            return Err(LinkerError::EmptyInput);
        }

        let output_path = output_path.as_ref();

        let triple = self
            .config
            .target
            .triple()
            .ok_or_else(|| {
                LinkerError::InvalidTarget(
                    "native artifact linking requires a target triple"
                        .to_string(),
                )
            })?;

        if triple.trim().is_empty() {
            return Err(LinkerError::InvalidTarget(
                "native target triple cannot be empty".to_string(),
            ));
        }

        validate_output_path(output_path)?;

        for object in object_files {
            if !object.exists() {
                return Err(LinkerError::MissingInput(object.clone()));
            }

            if !object.is_file() {
                return Err(LinkerError::MissingInput(object.clone()));
            }
        }

        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|error| {
                    LinkerError::Io(format!(
                        "failed to create output directory '{}': {error}",
                        parent.display()
                    ))
                })?;
            }
        }

        let executable = self.config.native.executable.trim();

        if executable.is_empty() {
            return Err(LinkerError::NativeLinker(
                "native linker executable cannot be empty".to_string(),
            ));
        }

        let mut command = Command::new(executable);

        /*
         * Output path is passed as one argument, never shell-expanded.
         */
        command.arg("-o");
        command.arg(output_path);

        if let Some(sysroot) = &self.config.native.sysroot {
            command.arg("--sysroot");
            command.arg(sysroot);
        }

        for path in &self.config.native.library_paths {
            command.arg("-L");
            command.arg(path);
        }

        for library in &self.config.native.libraries {
            if library.trim().is_empty() {
                return Err(LinkerError::NativeLinker(
                    "library name cannot be empty".to_string(),
                ));
            }

            command.arg("-l");
            command.arg(library);
        }

        /*
         * Explicit target-specific arguments are configuration-owned.
         *
         * We intentionally do not automatically add `--target` because not
         * every configured native linker accepts the same target-selection
         * syntax. Toolchain-specific target selection belongs in
         * NativeLinkerConfig::arguments.
         */
        for argument in &self.config.native.arguments {
            command.arg(argument);
        }

        for object in object_files {
            command.arg(object);
        }

        let output = command.output().map_err(|error| {
            LinkerError::NativeLinker(format!(
                "failed to execute '{}': {error}",
                executable
            ))
        })?;

        check_linker_output(executable, output)?;

        if !output_path.exists() {
            return Err(LinkerError::MissingOutput(
                output_path.to_path_buf(),
            ));
        }

        let metadata = std::fs::metadata(output_path).map_err(|error| {
            LinkerError::Io(format!(
                "failed to inspect linker output '{}': {error}",
                output_path.display()
            ))
        })?;

        if !metadata.is_file() {
            return Err(LinkerError::InvalidOutput(
                output_path.to_path_buf(),
            ));
        }

        if metadata.len() == 0 {
            return Err(LinkerError::InvalidOutput(
                output_path.to_path_buf(),
            ));
        }

        Ok(())
    }

    /// Return the number of input modules.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Whether the configured target is portable.
    #[must_use]
    pub fn is_portable_target(&self) -> bool {
        self.config.target.is_portable()
    }
}

// ============================================================================
// Validation
// ============================================================================

fn validate_config(config: &LinkerConfig) -> Result<(), LinkerError> {
    match &config.target {
        LinkTarget::Native { triple } => {
            validate_target_triple(triple, "native")?;
        }

        LinkTarget::Custom { triple, format } => {
            validate_target_triple(triple, "custom")?;

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
            "portable targets cannot use native executable/shared/static modes"
                .to_string(),
        ));
    }

    /*
     * Object linking is meaningful for target-specific compilation.
     */
    if matches!(config.mode, LinkingMode::Object)
        && config.target.is_portable()
    {
        return Err(LinkerError::InvalidTarget(
            "portable targets cannot use native object linking mode"
                .to_string(),
        ));
    }

    if matches!(config.mode, LinkingMode::Portable)
        && !config.target.is_portable()
    {
        return Err(LinkerError::InvalidTarget(
            "portable linking mode requires a portable target"
                .to_string(),
        ));
    }

    Ok(())
}

fn validate_target_triple(
    triple: &str,
    target_kind: &str,
) -> Result<(), LinkerError> {
    if triple.trim().is_empty() {
        return Err(LinkerError::InvalidTarget(format!(
            "{target_kind} target triple cannot be empty"
        )));
    }

    if triple.chars().any(char::is_whitespace) {
        return Err(LinkerError::InvalidTarget(format!(
            "{target_kind} target triple contains whitespace: '{triple}'"
        )));
    }

    Ok(())
}

fn validate_output_path(path: &Path) -> Result<(), LinkerError> {
    if path.as_os_str().is_empty() {
        return Err(LinkerError::InvalidOutput(path.to_path_buf()));
    }

    if path.exists() && path.is_dir() {
        return Err(LinkerError::InvalidOutput(path.to_path_buf()));
    }

    Ok(())
}

// ============================================================================
// Native linker process handling
// ============================================================================

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

    let mut message =
        format!("linker '{linker}' failed with status {status}");

    if !stderr.trim().is_empty() {
        message.push_str("\nstderr:\n");
        message.push_str(stderr.trim());
    }

    if !stdout.trim().is_empty() {
        message.push_str("\nstdout:\n");
        message.push_str(stdout.trim());
    }

    Err(LinkerError::NativeLinker(message))
}

fn default_linker() -> String {
    if cfg!(target_os = "windows") {
        "clang".to_string()
    } else {
        "cc".to_string()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_module(name: &str) -> IrModule {
        IrModule::new(name)
    }

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
        let target = LinkTarget::Native {
            triple: "x86_64-unknown-linux-gnu".to_string(),
        };

        assert!(!target.is_portable());
    }

    #[test]
    fn custom_target_is_not_portable() {
        let target = LinkTarget::Custom {
            triple: "riscv64gc-unknown-linux-gnu".to_string(),
            format: "elf".to_string(),
        };

        assert!(!target.is_portable());
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
    fn portable_formats_are_stable() {
        assert_eq!(LinkTarget::Portable.format(), "zportable");
        assert_eq!(LinkTarget::ZamaniBytecode.format(), "zbc");

        assert_eq!(
            LinkTarget::Wasm {
                architecture: WasmArchitecture::Wasm64
            }
            .format(),
            "wasm"
        );
    }

    #[test]
    fn empty_input_is_rejected() {
        let linker = ZamaniLinker::new(Vec::new());

        assert!(matches!(
            linker.link(),
            Err(LinkerError::EmptyInput)
        ));
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
    fn empty_custom_format_is_rejected() {
        let config = LinkerConfig {
            target: LinkTarget::Custom {
                triple: "x86_64-unknown-linux-gnu".to_string(),
                format: String::new(),
            },
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn portable_target_rejects_executable_mode() {
        let config = LinkerConfig {
            target: LinkTarget::Portable,
            mode: LinkingMode::Executable,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn portable_target_rejects_object_mode() {
        let config = LinkerConfig {
            target: LinkTarget::Portable,
            mode: LinkingMode::Object,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn portable_mode_requires_portable_target() {
        let config = LinkerConfig {
            target: LinkTarget::Native {
                triple: "x86_64-unknown-linux-gnu".to_string(),
            },
            mode: LinkingMode::Portable,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn native_object_mode_is_valid() {
        let config = LinkerConfig {
            target: LinkTarget::Native {
                triple: "aarch64-unknown-linux-gnu".to_string(),
            },
            mode: LinkingMode::Object,
            ..LinkerConfig::default()
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn wasm_portable_mode_is_valid() {
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
    fn default_linker_is_non_empty() {
        assert!(!default_linker().is_empty());
    }

    #[test]
    fn linker_default_is_portable() {
        let linker = ZamaniLinker::new(Vec::new());

        assert!(linker.is_portable_target());
        assert_eq!(linker.config.mode, LinkingMode::Portable);
        assert_eq!(linker.config.lto, LtoLevel::Basic);
    }

    #[test]
    fn with_target_sets_portable_mode_for_portable_target() {
        let linker = ZamaniLinker::with_target(
            vec![empty_module("a")],
            LinkTarget::Portable,
        );

        assert_eq!(linker.config.mode, LinkingMode::Portable);
        assert!(linker.is_portable_target());
    }

    #[test]
    fn with_target_sets_executable_mode_for_native_target() {
        let linker = ZamaniLinker::with_target(
            vec![empty_module("a")],
            LinkTarget::Native {
                triple: "x86_64-unknown-linux-gnu".to_string(),
            },
        );

        assert_eq!(linker.config.mode, LinkingMode::Executable);
        assert!(!linker.is_portable_target());
    }

    #[test]
    fn module_count_is_correct() {
        let linker = ZamaniLinker::new(vec![
            empty_module("a"),
            empty_module("b"),
            empty_module("c"),
        ]);

        assert_eq!(linker.module_count(), 3);
    }

    #[test]
    fn empty_modules_can_be_linked_when_input_exists() {
        let linker = ZamaniLinker::new(vec![empty_module("a")]);

        let result = linker.link_with_result().unwrap();

        assert_eq!(result.input_modules, 1);
        assert_eq!(result.function_count, 0);
        assert_eq!(result.global_count, 0);
        assert_eq!(result.instruction_count, 0);
    }

    #[test]
    fn target_can_be_changed_safely() {
        let mut linker = ZamaniLinker::new(vec![empty_module("a")]);

        linker
            .set_target(LinkTarget::Wasm {
                architecture: WasmArchitecture::Wasm32,
            })
            .unwrap();

        assert!(linker.is_portable_target());
        assert_eq!(linker.config.mode, LinkingMode::Portable);
    }

    #[test]
    fn invalid_target_change_does_not_mutate_configuration() {
        let mut linker = ZamaniLinker::new(vec![empty_module("a")]);

        let original = linker.config.clone();

        let result = linker.set_target(LinkTarget::Custom {
            triple: String::new(),
            format: "elf".to_string(),
        });

        assert!(result.is_err());
        assert_eq!(
            linker.config.target,
            original.target
        );
        assert_eq!(linker.config.mode, original.mode);
    }

    #[test]
    fn link_portable_forces_portable_target() {
        let linker = ZamaniLinker::with_target(
            vec![empty_module("a")],
            LinkTarget::Native {
                triple: "x86_64-unknown-linux-gnu".to_string(),
            },
        );

        let result = linker.link_portable().unwrap();

        assert_eq!(result.name, "Zamani_Linked");
    }

    #[test]
    fn link_wasm_forces_wasm_target() {
        let linker = ZamaniLinker::new(vec![empty_module("a")]);

        let result = linker
            .link_wasm(WasmArchitecture::Wasm64)
            .unwrap();

        assert_eq!(
            result.functions.len(),
            0
        );
    }

    #[test]
    fn link_native_validates_target() {
        let linker = ZamaniLinker::new(vec![empty_module("a")]);

        let result = linker.link_native("");

        assert!(matches!(
            result,
            Err(LinkerError::InvalidTarget(_))
        ));
    }

    #[test]
    fn duplicate_functions_are_rejected() {
        let mut first = IrModule::new("first");
        let mut second = IrModule::new("second");

        first.add_function(crate::ir_gen::IrFunction::new(
            "duplicate",
            Vec::new(),
            crate::ir_gen::IrType::Void,
        ));

        second.add_function(crate::ir_gen::IrFunction::new(
            "duplicate",
            Vec::new(),
            crate::ir_gen::IrType::Void,
        ));

        let linker = ZamaniLinker::new(vec![first, second]);

        assert!(matches!(
            linker.link(),
            Err(LinkerError::DuplicateSymbol(name))
                if name == "duplicate"
        ));
    }

    #[test]
    fn function_global_collision_is_rejected() {
        let mut module = IrModule::new("module");

        module.add_function(crate::ir_gen::IrFunction::new(
            "collision",
            Vec::new(),
            crate::ir_gen::IrType::Void,
        ));

        module.add_global(crate::ir_gen::IrGlobal {
            name: "collision".to_string(),
            ty: crate::ir_gen::IrType::I64,
            value: crate::ir_gen::IrValue::ConstInt(
                1,
                crate::ir_gen::IrType::I64,
            ),
            is_const: true,
        });

        let linker = ZamaniLinker::new(vec![module]);

        assert!(matches!(
            linker.link(),
            Err(LinkerError::DuplicateSymbol(name))
                if name == "collision"
        ));
    }

    #[test]
    fn duplicate_globals_are_rejected() {
        let mut first = IrModule::new("first");
        let mut second = IrModule::new("second");

        let global = || crate::ir_gen::IrGlobal {
            name: "value".to_string(),
            ty: crate::ir_gen::IrType::I64,
            value: crate::ir_gen::IrValue::ConstInt(
                42,
                crate::ir_gen::IrType::I64,
            ),
            is_const: true,
        };

        first.add_global(global());
        second.add_global(global());

        let linker = ZamaniLinker::new(vec![first, second]);

        assert!(matches!(
            linker.link(),
            Err(LinkerError::DuplicateSymbol(name))
                if name == "value"
        ));
    }

    #[test]
    fn string_literals_are_merged_without_changing_order() {
        let mut first = IrModule::new("first");
        let mut second = IrModule::new("second");

        first
            .string_literals
            .push(("str0".into(), "hello".into()));

        second
            .string_literals
            .push(("str1".into(), "world".into()));

        let linker = ZamaniLinker::new(vec![first, second]);

        let result = linker.link().unwrap();

        assert_eq!(
            result.string_literals,
            vec![
                ("str0".into(), "hello".into()),
                ("str1".into(), "world".into()),
            ]
        );
    }

    #[test]
    fn duplicate_string_symbols_are_rejected() {
        let mut first = IrModule::new("first");
        let mut second = IrModule::new("second");

        first
            .string_literals
            .push(("str0".into(), "hello".into()));

        second
            .string_literals
            .push(("str0".into(), "different".into()));

        let linker = ZamaniLinker::new(vec![first, second]);

        assert!(matches!(
            linker.link(),
            Err(LinkerError::DuplicateStringLiteral(name))
                if name == "str0"
        ));
    }

    #[test]
    fn type_definitions_are_preserved() {
        let mut module = IrModule::new("module");

        module.type_defs.push((
            "Point".into(),
            vec![
                ("x".into(), crate::ir_gen::IrType::I64),
                ("y".into(), crate::ir_gen::IrType::I64),
            ],
        ));

        let linker = ZamaniLinker::new(vec![module]);
        let result = linker.link().unwrap();

        assert_eq!(result.type_defs.len(), 1);
        assert_eq!(result.type_defs[0].0, "Point");
    }

    #[test]
    fn duplicate_type_definitions_are_rejected() {
        let mut first = IrModule::new("first");
        let mut second = IrModule::new("second");

        first
            .type_defs
            .push(("Point".into(), Vec::new()));

        second
            .type_defs
            .push(("Point".into(), Vec::new()));

        let linker = ZamaniLinker::new(vec![first, second]);

        assert!(matches!(
            linker.link(),
            Err(LinkerError::DuplicateSymbol(name))
                if name == "Point"
        ));
    }

    #[test]
    fn module_target_information_is_preserved() {
        let mut module = IrModule::new("module");

        module.target_triple =
            "aarch64-unknown-linux-gnu".into();

        module.data_layout =
            "custom-data-layout".into();

        let linker = ZamaniLinker::new(vec![module]);
        let result = linker.link().unwrap();

        assert_eq!(
            result.target_triple,
            "aarch64-unknown-linux-gnu"
        );

        assert_eq!(
            result.data_layout,
            "custom-data-layout"
        );
    }

    #[test]
    fn lto_none_preserves_instruction_count() {
        let linker = ZamaniLinker {
            modules: vec![empty_module("module")],
            config: LinkerConfig {
                lto: LtoLevel::None,
                ..LinkerConfig::default()
            },
        };

        let mut module = IrModule::new("test");

        linker.optimize_lto(&mut module);

        assert_eq!(module.instruction_count(), 0);
    }

    #[test]
    fn optimize_lto_deduplicates_identical_string_pairs() {
        let linker = ZamaniLinker::new(Vec::new());

        let mut module = IrModule::new("test");

        module
            .string_literals
            .push(("a".into(), "hello".into()));

        module
            .string_literals
            .push(("a".into(), "hello".into()));

        linker.optimize_lto(&mut module);

        assert_eq!(module.string_literals.len(), 1);
    }

    #[test]
    fn output_directory_is_created_by_native_link_stage() {
        /*
         * Only validate path semantics here. The test deliberately does not
         * invoke a platform linker, keeping the unit test deterministic and
         * cross-platform.
         */
        let path = PathBuf::from("target/zlink-test-output/program");

        assert!(validate_output_path(&path).is_ok());
    }
}