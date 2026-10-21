#![deny(unsafe_code)]

//! External toolchain discovery, identification, validation, and invocation.
//!
//! # Responsibilities
//!
//! - Resolve explicitly configured executable paths or discover executables
//!   through PATH.
//! - Probe and parse tool versions.
//! - Enforce caller-specified version requirements.
//! - Invoke validated tools through `backend::process::ProcessRunner`.
//! - Preserve process output, cancellation, timeout, and failure information.
//! - Produce a descriptor that records which executable and version were
//!   validated.
//!
//! # Non-responsibilities
//!
//! This module does not:
//! - choose a compilation target;
//! - decide whether a target supports a Zamani semantic capability;
//! - generate LLVM, WebAssembly, QASM, HDL, or native code;
//! - link artifacts or resolve symbols;
//! - implement a sandbox;
//! - infer a tool's version or successful execution when probing fails.
//!
//! # Integration
//!
//! `process.rs` owns external process execution. `toolchain.rs` owns tool
//! discovery and version validation. Target backends own the meaning of
//! tool arguments and the validation of generated artifacts.
//!
//! Executable authorization and filesystem policy must be enforced by the
//! caller's security boundary. Discovering a tool does not make it trusted.
//!
//! # Scalability
//!
//! No universal hardware capacities, target counts, or tool counts are
//! imposed. Version and output limits are configurable operational policies.
//! A toolchain's actual limitations remain properties of that toolchain and
//! target, not universal Zamani language restrictions.

use crate::backend::process::{
    CancellationToken, EnvironmentMode, ProcessError, ProcessOutput, ProcessRequest,
    ProcessRunner,
};

use std::cmp::Ordering;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Default deadline for a tool's version probe.
///
/// This is an operational default, not a limit on program size or hardware
/// capacity. Callers can choose a different deadline or disable it.
pub const DEFAULT_VERSION_PROBE_TIMEOUT: Duration = Duration::from_secs(10);

/// Default deadline for ordinary tool invocations.
///
/// Long-running tools should receive an explicitly appropriate timeout.
/// `None` means no deadline; cancellation remains available.
pub const DEFAULT_INVOCATION_TIMEOUT: Duration = Duration::from_secs(300);

/// Version of a tool parsed from its version output.
///
/// This type compares the numeric major, minor, and patch components.
/// It deliberately does not claim full SemVer compatibility: vendor suffixes,
/// build metadata, prerelease ordering, and vendor-specific version schemes
/// are not normalized into a universal semantic-version interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl ToolVersion {
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse a numeric version from a version string.
    ///
    /// The parser searches for a numeric token containing at least one dot.
    /// It accepts `1.2`, `1.2.3`, and tokens such as `v1.2.3`.
    /// Missing patch components are treated as zero.
    ///
    /// This is intentionally a basic numeric parser, not a universal parser
    /// for every vendor's version scheme. A tool with a nonstandard scheme
    /// should use a dedicated adapter rather than relying on guessed meaning.
    pub fn parse(output: &str) -> Option<Self> {
        for token in output.split_whitespace() {
            let token = token.trim_matches(|character: char| {
                !character.is_ascii_alphanumeric() && character != '.'
            });

            let token = token.strip_prefix('v').unwrap_or(token);
            let numeric_end = token
                .find(|character: char| {
                    !character.is_ascii_digit() && character != '.'
                })
                .unwrap_or(token.len());

            let numeric = &token[..numeric_end];

            if !numeric.contains('.') {
                continue;
            }

            let mut parts = numeric.split('.');
            let major = parts.next()?.parse::<u64>().ok()?;
            let minor = parts.next()?.parse::<u64>().ok()?;
            let patch = match parts.next() {
                Some(value) if !value.is_empty() => value.parse::<u64>().ok()?,
                Some(_) => continue,
                None => 0,
            };

            // Do not accept malformed values such as "1..3" or "1.2.3.4".
            if parts.next().is_some() {
                continue;
            }

            return Some(Self::new(major, minor, patch));
        }

        None
    }
}

impl Ord for ToolVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(
            other.major,
            other.minor,
            other.patch,
        ))
    }
}

impl PartialOrd for ToolVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ToolVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Version policy applied after a successful version probe.
///
/// All configured conditions must hold. An empty policy accepts any version
/// that the configured probe can identify.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VersionRequirement {
    minimum_inclusive: Option<ToolVersion>,
    maximum_inclusive: Option<ToolVersion>,
    exact: Option<ToolVersion>,
}

impl VersionRequirement {
    #[must_use]
    pub const fn any() -> Self {
        Self {
            minimum_inclusive: None,
            maximum_inclusive: None,
            exact: None,
        }
    }

    #[must_use]
    pub fn at_least(mut self, version: ToolVersion) -> Self {
        self.minimum_inclusive = Some(version);
        self
    }

    #[must_use]
    pub fn at_most(mut self, version: ToolVersion) -> Self {
        self.maximum_inclusive = Some(version);
        self
    }

    #[must_use]
    pub fn exactly(mut self, version: ToolVersion) -> Self {
        self.exact = Some(version);
        self
    }

    pub fn validate(&self, actual: ToolVersion) -> Result<(), ToolchainError> {
        if let Some(exact) = self.exact {
            if actual != exact {
                return Err(ToolchainError::VersionMismatch {
                    actual,
                    requirement: format!("exactly {exact}"),
                });
            }
        }

        if let Some(minimum) = self.minimum_inclusive {
            if actual < minimum {
                return Err(ToolchainError::VersionMismatch {
                    actual,
                    requirement: format!("at least {minimum}"),
                });
            }
        }

        if let Some(maximum) = self.maximum_inclusive {
            if actual > maximum {
                return Err(ToolchainError::VersionMismatch {
                    actual,
                    requirement: format!("at most {maximum}"),
                });
            }
        }

        if let (Some(minimum), Some(maximum)) =
            (self.minimum_inclusive, self.maximum_inclusive)
        {
            if minimum > maximum {
                return Err(ToolchainError::InvalidSpecification(
                    "minimum version exceeds maximum version".to_owned(),
                ));
            }
        }

        Ok(())
    }
}

/// Configuration for one external tool.
///
/// `id` is a stable, caller-defined identifier such as `llvm.clang`,
/// `llvm.llvm-as`, `wasm.wat-validator`, or `hdl.verilator`.
///
/// The identifier is metadata; it is not interpreted as an executable name.
/// `executable` is the actual path or executable name used for discovery.
#[derive(Debug, Clone)]
pub struct ToolchainSpec {
    id: String,
    executable: PathBuf,
    version_args: Vec<OsString>,
    version_requirement: VersionRequirement,
    environment_mode: EnvironmentMode,
    environment: Vec<(OsString, OsString)>,
    removed_environment: Vec<OsString>,
    current_dir: Option<PathBuf>,
    version_probe_timeout: Option<Duration>,
    invocation_timeout: Option<Duration>,
    max_output_bytes_per_stream: Option<usize>,
}

impl ToolchainSpec {
    /// Create a tool specification.
    ///
    /// Defaults use `--version`, an inherited environment, a bounded captured
    /// output budget, and finite timeouts. Toolchains with different version
    /// commands must configure `version_args` explicitly.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        executable: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: id.into(),
            executable: executable.into(),
            version_args: vec![OsString::from("--version")],
            version_requirement: VersionRequirement::any(),
            environment_mode: EnvironmentMode::Inherit,
            environment: Vec::new(),
            removed_environment: Vec::new(),
            current_dir: None,
            version_probe_timeout: Some(DEFAULT_VERSION_PROBE_TIMEOUT),
            invocation_timeout: Some(DEFAULT_INVOCATION_TIMEOUT),
            max_output_bytes_per_stream: Some(
                crate::backend::process::DEFAULT_CAPTURE_LIMIT_PER_STREAM,
            ),
        }
    }

    #[must_use]
    pub fn version_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.version_args = args.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn version_requirement(
        mut self,
        requirement: VersionRequirement,
    ) -> Self {
        self.version_requirement = requirement;
        self
    }

    #[must_use]
    pub fn environment_mode(mut self, mode: EnvironmentMode) -> Self {
        self.environment_mode = mode;
        self
    }

    #[must_use]
    pub fn env(
        mut self,
        key: impl Into<OsString>,
        value: impl Into<OsString>,
    ) -> Self {
        self.environment.push((key.into(), value.into()));
        self
    }

    #[must_use]
    pub fn remove_env(mut self, key: impl Into<OsString>) -> Self {
        self.removed_environment.push(key.into());
        self
    }

    #[must_use]
    pub fn current_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.current_dir = Some(path.into());
        self
    }

    /// Configure the version-probe deadline.
    ///
    /// `None` disables the deadline.
    #[must_use]
    pub fn version_probe_timeout(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.version_probe_timeout = timeout;
        self
    }

    /// Configure the default invocation deadline.
    ///
    /// `None` permits an invocation to run without a deadline. Callers should
    /// still retain a cancellation path for long-running external tools.
    #[must_use]
    pub fn invocation_timeout(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.invocation_timeout = timeout;
        self
    }

    /// Configure captured output retained from each stream.
    ///
    /// `None` retains all output in memory. This should only be used when the
    /// caller intentionally accepts memory consumption proportional to output.
    #[must_use]
    pub fn max_output_bytes_per_stream(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_output_bytes_per_stream = limit;
        self
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    fn validate(&self) -> Result<(), ToolchainError> {
        if self.id.trim().is_empty() {
            return Err(ToolchainError::InvalidSpecification(
                "toolchain identifier must not be empty".to_owned(),
            ));
        }

        if self.executable.as_os_str().is_empty() {
            return Err(ToolchainError::InvalidSpecification(
                "toolchain executable must not be empty".to_owned(),
            ));
        }

        if let (Some(minimum), Some(maximum)) = (
            self.version_requirement.minimum_inclusive,
            self.version_requirement.maximum_inclusive,
        ) {
            if minimum > maximum {
                return Err(ToolchainError::InvalidSpecification(
                    "minimum version exceeds maximum version".to_owned(),
                ));
            }
        }

        if let Some(exact) = self.version_requirement.exact {
            if self
                .version_requirement
                .minimum_inclusive
                .is_some_and(|minimum| exact < minimum)
                || self
                    .version_requirement
                    .maximum_inclusive
                    .is_some_and(|maximum| exact > maximum)
            {
                return Err(ToolchainError::InvalidSpecification(
                    "exact version conflicts with configured version range"
                        .to_owned(),
                ));
            }
        }

        Ok(())
    }
}

/// Result of resolving and validating an external toolchain.
///
/// A descriptor represents the exact executable path resolved during
/// discovery and the numeric version parsed during the probe. It is not a
/// cryptographic identity or a guarantee that the executable cannot change
/// after validation. Hermetic build systems should additionally pin tools by
/// trusted package/image digest or executable hash.
#[derive(Debug, Clone)]
pub struct ToolchainDescriptor {
    id: String,
    executable: PathBuf,
    version: ToolVersion,
    version_output: String,
    spec: ToolchainSpec,
}

impl ToolchainDescriptor {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    #[must_use]
    pub const fn version(&self) -> ToolVersion {
        self.version
    }

    #[must_use]
    pub fn version_output(&self) -> &str {
        &self.version_output
    }

    /// Run the validated tool with arguments passed individually.
    ///
    /// The arguments are never joined into a shell command. The caller owns
    /// argument semantics and must validate any paths or user-controlled
    /// options before invocation.
    pub fn invoke<I, S>(
        &self,
        runner: &ProcessRunner,
        args: I,
        cancellation: CancellationToken,
    ) -> Result<ProcessOutput, ToolchainError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let mut request = ProcessRequest::new(&self.executable)
            .args(args)
            .environment_mode(self.spec.environment_mode)
            .max_output_bytes_per_stream(
                self.spec.max_output_bytes_per_stream,
            )
            .cancellation_token(cancellation);

        if let Some(directory) = &self.spec.current_dir {
            request = request.current_dir(directory);
        }

        if let Some(timeout) = self.spec.invocation_timeout {
            request = request.timeout(timeout);
        }

        for (key, value) in &self.spec.environment {
            request = request.env(key.clone(), value.clone());
        }

        for key in &self.spec.removed_environment {
            request = request.remove_env(key.clone());
        }

        runner.run(&request).map_err(|source| {
            ToolchainError::Invocation {
                toolchain_id: self.id.clone(),
                executable: self.executable.clone(),
                source,
            }
        })
    }

    /// Run without a caller-shared cancellation token.
    ///
    /// For cancellable work, prefer `invoke` and retain the token.
    pub fn invoke_uncancellable<I, S>(
        &self,
        runner: &ProcessRunner,
        args: I,
    ) -> Result<ProcessOutput, ToolchainError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.invoke(runner, args, CancellationToken::new())
    }
}

/// Stateless toolchain manager.
///
/// Discovery is deliberately explicit. No global registry is mutated and no
/// compiler target is selected implicitly.
#[derive(Debug, Default, Clone, Copy)]
pub struct ToolchainManager {
    runner: ProcessRunner,
}

impl ToolchainManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            runner: ProcessRunner::new(),
        }
    }

    /// Resolve an executable, run its version probe, and enforce its policy.
    ///
    /// A failed probe, unparseable version, missing executable, or version
    /// mismatch is an error. None of these conditions is treated as success.
    pub fn discover(
        &self,
        spec: &ToolchainSpec,
    ) -> Result<ToolchainDescriptor, ToolchainError> {
        spec.validate()?;

        let executable = resolve_executable(&spec.executable).map_err(
            |source| ToolchainError::ExecutableNotFound {
                toolchain_id: spec.id.clone(),
                requested: spec.executable.clone(),
                source,
            },
        )?;

        let mut request = ProcessRequest::new(&executable)
            .args(spec.version_args.iter().cloned())
            .environment_mode(spec.environment_mode)
            .max_output_bytes_per_stream(spec.max_output_bytes_per_stream);

        if let Some(directory) = &spec.current_dir {
            request = request.current_dir(directory);
        }

        if let Some(timeout) = spec.version_probe_timeout {
            request = request.timeout(timeout);
        }

        for (key, value) in &spec.environment {
            request = request.env(key.clone(), value.clone());
        }

        for key in &spec.removed_environment {
            request = request.remove_env(key.clone());
        }

        let output = self.runner.run(&request).map_err(|source| {
            ToolchainError::VersionProbe {
                toolchain_id: spec.id.clone(),
                executable: executable.clone(),
                source,
            }
        })?;

        let version_output = combined_output(&output);
        let version = ToolVersion::parse(&version_output).ok_or_else(|| {
            ToolchainError::UnparseableVersion {
                toolchain_id: spec.id.clone(),
                executable: executable.clone(),
                output: version_output.clone(),
            }
        })?;

        spec.version_requirement.validate(version)?;

        Ok(ToolchainDescriptor {
            id: spec.id.clone(),
            executable,
            version,
            version_output,
            spec: spec.clone(),
        })
    }

    /// Discover a toolchain and verify that its version satisfies the spec.
    ///
    /// This named entry point makes the validation step explicit for callers
    /// that use toolchains as a build prerequisite.
    pub fn validate(
        &self,
        spec: &ToolchainSpec,
    ) -> Result<ToolchainDescriptor, ToolchainError> {
        self.discover(spec)
    }
}

/// Failures during toolchain discovery, validation, or invocation.
#[derive(Debug)]
pub enum ToolchainError {
    InvalidSpecification(String),

    ExecutableNotFound {
        toolchain_id: String,
        requested: PathBuf,
        source: std::io::Error,
    },

    VersionProbe {
        toolchain_id: String,
        executable: PathBuf,
        source: ProcessError,
    },

    UnparseableVersion {
        toolchain_id: String,
        executable: PathBuf,
        output: String,
    },

    VersionMismatch {
        actual: ToolVersion,
        requirement: String,
    },

    Invocation {
        toolchain_id: String,
        executable: PathBuf,
        source: ProcessError,
    },
}

impl fmt::Display for ToolchainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpecification(message) => {
                write!(formatter, "invalid toolchain specification: {message}")
            }
            Self::ExecutableNotFound {
                toolchain_id,
                requested,
                source,
            } => write!(
                formatter,
                "toolchain `{toolchain_id}` executable `{}` could not be resolved: {source}",
                requested.display()
            ),
            Self::VersionProbe {
                toolchain_id,
                executable,
                source,
            } => write!(
                formatter,
                "version probe failed for toolchain `{toolchain_id}` at `{}`: {source}",
                executable.display()
            ),
            Self::UnparseableVersion {
                toolchain_id,
                executable,
                output,
            } => write!(
                formatter,
                "could not parse a numeric version for toolchain `{toolchain_id}` at `{}`; output: {}",
                executable.display(),
                output.trim()
            ),
            Self::VersionMismatch {
                actual,
                requirement,
            } => write!(
                formatter,
                "tool version {actual} does not satisfy requirement {requirement}"
            ),
            Self::Invocation {
                toolchain_id,
                executable,
                source,
            } => write!(
                formatter,
                "invocation failed for toolchain `{toolchain_id}` at `{}`: {source}",
                executable.display()
            ),
        }
    }
}

impl Error for ToolchainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ExecutableNotFound { source, .. } => Some(source),
            Self::VersionProbe { source, .. }
            | Self::Invocation { source, .. } => Some(source),
            Self::InvalidSpecification(_)
            | Self::UnparseableVersion { .. }
            | Self::VersionMismatch { .. } => None,
        }
    }
}

/// Resolve a configured executable path or search PATH.
///
/// Empty PATH entries are ignored so an empty component does not implicitly
/// add the current working directory as a tool search location. Explicit
/// relative paths containing path separators remain caller-controlled input
/// and should be authorized by the security layer.
fn resolve_executable(requested: &Path) -> Result<PathBuf, std::io::Error> {
    if requested.components().count() > 1 || requested.is_absolute() {
        return canonical_executable(requested);
    }

    let path = std::env::var_os("PATH").ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "PATH is not set; configure an explicit executable path",
        )
    })?;

    for directory in std::env::split_paths(&path) {
        if directory.as_os_str().is_empty() {
            continue;
        }

        let candidate = directory.join(requested);

        if let Ok(resolved) = canonical_executable(&candidate) {
            return Ok(resolved);
        }

        #[cfg(windows)]
        {
            for extension in windows_path_extensions() {
                let candidate = directory.join(append_extension(
                    requested,
                    &extension,
                ));

                if let Ok(resolved) = canonical_executable(&candidate) {
                    return Ok(resolved);
                }
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "no executable file named `{}` was found in PATH; configure an explicit path or install the tool",
            requested.to_string_lossy()
        ),
    ))
}

fn canonical_executable(path: &Path) -> Result<PathBuf, std::io::Error> {
    let metadata = std::fs::metadata(path)?;

    if !metadata.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("`{}` is not a regular file", path.display()),
        ));
    }

    // Canonicalization stabilizes the path used for this invocation, but it
    // does not prove executable trust or prevent replacement after validation.
    std::fs::canonicalize(path)
}

fn combined_output(output: &ProcessOutput) -> String {
    let stdout = output.stdout.to_string_lossy();
    let stderr = output.stderr.to_string_lossy();

    match (stdout.is_empty(), stderr.is_empty()) {
        (false, false) => format!("{stdout}\n{stderr}"),
        (false, true) => stdout,
        (true, false) => stderr,
        (true, true) => String::new(),
    }
}

#[cfg(windows)]
fn windows_path_extensions() -> Vec<OsString> {
    let configured = std::env::var_os("PATHEXT")
        .unwrap_or_else(|| OsString::from(".COM;.EXE;.BAT;.CMD"));

    configured
        .to_string_lossy()
        .split(';')
        .filter(|extension| !extension.is_empty())
        .map(OsString::from)
        .collect()
}

#[cfg(windows)]
fn append_extension(path: &Path, extension: &OsStr) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(extension);
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_numeric_version_formats() {
        assert_eq!(
            ToolVersion::parse("clang version 18.1.8"),
            Some(ToolVersion::new(18, 1, 8))
        );

        assert_eq!(
            ToolVersion::parse("GNU ld version 2.40"),
            Some(ToolVersion::new(2, 40, 0))
        );

        assert_eq!(
            ToolVersion::parse("tool v1.2.3"),
            Some(ToolVersion::new(1, 2, 3))
        );
    }

    #[test]
    fn rejects_output_without_numeric_version() {
        assert_eq!(ToolVersion::parse("tool version unknown"), None);
    }

    #[test]
    fn versions_compare_by_numeric_components() {
        assert!(ToolVersion::new(10, 0, 0) > ToolVersion::new(9, 99, 99));
        assert!(ToolVersion::new(1, 10, 0) > ToolVersion::new(1, 9, 99));
    }

    #[test]
    fn version_requirement_enforces_inclusive_bounds() {
        let requirement = VersionRequirement::any()
            .at_least(ToolVersion::new(1, 2, 0))
            .at_most(ToolVersion::new(2, 0, 0));

        assert!(requirement
            .validate(ToolVersion::new(1, 2, 0))
            .is_ok());

        assert!(requirement
            .validate(ToolVersion::new(2, 0, 0))
            .is_ok());

        assert!(requirement
            .validate(ToolVersion::new(1, 1, 9))
            .is_err());

        assert!(requirement
            .validate(ToolVersion::new(2, 0, 1))
            .is_err());
    }

    #[test]
    fn exact_version_is_enforced() {
        let requirement = VersionRequirement::any()
            .exactly(ToolVersion::new(17, 0, 1));

        assert!(requirement
            .validate(ToolVersion::new(17, 0, 1))
            .is_ok());

        assert!(requirement
            .validate(ToolVersion::new(17, 0, 2))
            .is_err());
    }

    #[test]
    fn rejects_conflicting_version_range() {
        let spec = ToolchainSpec::new("test.tool", "tool")
            .version_requirement(
                VersionRequirement::any()
                    .at_least(ToolVersion::new(3, 0, 0))
                    .at_most(ToolVersion::new(2, 0, 0)),
            );

        assert!(matches!(
            spec.validate(),
            Err(ToolchainError::InvalidSpecification(_))
        ));
    }

    #[test]
    fn rejects_empty_toolchain_identifier() {
        let spec = ToolchainSpec::new("  ", "tool");
        assert!(matches!(
            spec.validate(),
            Err(ToolchainError::InvalidSpecification(_))
        ));
    }
}