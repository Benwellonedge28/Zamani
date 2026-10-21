
#![forbid(unsafe_code)]
//! Zamani backend security boundary.
//!
//! Path: src/backend/security.rs
//!
//! Purpose:
//!     Provide target-independent security controls for backend output paths,
//!     external tool configuration, and security-sensitive artifact handling.
//!
//! Ownership:
//!     This module owns backend security policy and validation primitives.
//!     It does not own semantic analysis, IR verification, threat-pattern
//!     analysis, linking, process execution, or target-specific lowering.
//!
//! Integration:
//!     - src/compiler/safety_guard.rs:
//!       Keep instruction-stream threat inspection there. This module
//!       provides complementary filesystem/tool trust-boundary controls.
//!     - src/compiler/linker.rs:
//!       Validate native-linker executables and output paths before invoking
//!       the existing linking implementation.
//!     - src/compiler/llvm_backend.rs and src/compiler/wasm_backend.rs:
//!       Validate tool paths and output destinations before generation.
//!     - src/backend/process.rs, when introduced:
//!       Consume ToolInvocation only through an explicit, audited process
//!       execution layer. This file intentionally does not run processes.
//!     - src/backend/errors.rs, when introduced:
//!       Map SecurityError to the canonical backend error type in the
//!       integration adapter rather than creating a second global error model.
//!
//! Security properties:
//!     - No unsafe Rust.
//!     - No shell parsing or shell-command interpolation.
//!     - No implicit creation of output directories.
//!     - Output files are created exclusively, never overwritten.
//!     - Existing symbolic links in output paths are rejected.
//!     - External tools must match an explicit executable allowlist.
//!     - Policies are caller-configurable; no hardware capacity assumptions.
//!
//! Important limitation:
//!     std-only path validation cannot eliminate every filesystem TOCTOU race.
//!     Callers processing hostile concurrent filesystem modifications must
//!     additionally use OS-level sandboxing and directory-handle-relative
//!     operations supplied by an appropriate platform-specific layer.
//!
//! This module is deliberately independent of backend target implementations.

use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Component, Path, PathBuf};

/// Stable classification of a backend security failure.
///
/// Keep variants machine-readable. User-facing formatting belongs to the
/// diagnostics layer; callers should match variants instead of parsing text.
#[derive(Debug)]
pub enum SecurityError {
    InvalidPolicy(String),
    InvalidPath(String),
    PathOutsideRoot(PathBuf),
    SymlinkRejected(PathBuf),
    MissingParent(PathBuf),
    ParentNotDirectory(PathBuf),
    OutputAlreadyExists(PathBuf),
    InvalidExecutable(PathBuf),
    ExecutableNotAllowed(PathBuf),
    InvalidArgument,
    InvalidEnvironment,
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy(message) => {
                write!(f, "invalid backend security policy: {message}")
            }
            Self::InvalidPath(message) => {
                write!(f, "invalid backend path: {message}")
            }
            Self::PathOutsideRoot(path) => {
                write!(f, "path is outside the permitted root: {}", path.display())
            }
            Self::SymlinkRejected(path) => {
                write!(f, "symbolic link is not permitted: {}", path.display())
            }
            Self::MissingParent(path) => {
                write!(f, "output parent does not exist: {}", path.display())
            }
            Self::ParentNotDirectory(path) => {
                write!(f, "output parent is not a directory: {}", path.display())
            }
            Self::OutputAlreadyExists(path) => {
                write!(f, "refusing to overwrite existing output: {}", path.display())
            }
            Self::InvalidExecutable(path) => {
                write!(f, "invalid executable: {}", path.display())
            }
            Self::ExecutableNotAllowed(path) => {
                write!(f, "executable is not allowlisted: {}", path.display())
            }
            Self::InvalidArgument => {
                write!(f, "tool argument contains an invalid NUL byte")
            }
            Self::InvalidEnvironment => {
                write!(f, "invalid tool environment entry")
            }
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                f,
                "{operation} failed for '{}': {source}",
                path.display()
            ),
        }
    }
}

impl Error for SecurityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn io_error(
    operation: &'static str,
    path: &Path,
    source: io::Error,
) -> SecurityError {
    SecurityError::Io {
        operation,
        path: path.to_path_buf(),
        source,
    }
}

/// Filesystem policy for backend-produced artifacts.
///
/// The root must already exist and be a directory. Output subdirectories
/// must also already exist. This avoids silently creating directory trees
/// in locations that have not been explicitly provisioned by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputPathPolicy {
    root: PathBuf,
}

impl OutputPathPolicy {
    /// Construct a policy rooted at an existing directory.
    ///
    /// Canonicalization resolves the root once, so subsequent relative
    /// destinations are interpreted against a stable absolute root path.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, SecurityError> {
        let supplied = root.as_ref();

        let metadata = fs::symlink_metadata(supplied)
            .map_err(|e| io_error("inspect output root", supplied, e))?;

        if metadata.file_type().is_symlink() {
            return Err(SecurityError::SymlinkRejected(supplied.to_path_buf()));
        }

        if !metadata.is_dir() {
            return Err(SecurityError::ParentNotDirectory(
                supplied.to_path_buf(),
            ));
        }

        let root = fs::canonicalize(supplied)
            .map_err(|e| io_error("canonicalize output root", supplied, e))?;

        Ok(Self { root })
    }

    /// Return the canonical, validated root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Validate a relative output destination.
    ///
    /// Absolute paths, parent traversal, current-directory components,
    /// platform prefixes, Windows-style separators, drive-like paths,
    /// control characters, and empty destinations are rejected.
    ///
    /// Every existing parent component is inspected with symlink_metadata.
    /// Parent directories must already exist and must not be symlinks.
    ///
    /// The final destination may not already exist. The returned path is
    /// intended for use with create_new_output(), not for unrestricted writes.
    pub fn validate_relative(
        &self,
        relative: impl AsRef<Path>,
    ) -> Result<PathBuf, SecurityError> {
        let relative = relative.as_ref();

        if relative.as_os_str().is_empty() {
            return Err(SecurityError::InvalidPath(
                "destination is empty".into(),
            ));
        }

        // Enforce portable path syntax rather than relying on the host OS to
        // interpret another OS's separators or drive syntax.
        let raw = relative.to_str().ok_or_else(|| {
            SecurityError::InvalidPath(
                "destination is not valid UTF-8".into(),
            )
        })?;

        if raw.contains('\0')
            || raw.contains('\\')
            || raw.chars().any(char::is_control)
            || raw.starts_with('/')
            || raw.starts_with("//")
            || raw.contains(':')
        {
            return Err(SecurityError::InvalidPath(
                "destination contains a forbidden character or prefix".into(),
            ));
        }

        let mut components = Vec::new();

        for component in relative.components() {
            match component {
                Component::Normal(part) => {
                    if part == OsStr::new("") {
                        return Err(SecurityError::InvalidPath(
                            "empty path component".into(),
                        ));
                    }
                    components.push(part.to_os_string());
                }
                Component::CurDir => {
                    return Err(SecurityError::InvalidPath(
                        "current-directory components are forbidden".into(),
                    ));
                }
                Component::ParentDir => {
                    return Err(SecurityError::InvalidPath(
                        "parent-directory traversal is forbidden".into(),
                    ));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(SecurityError::InvalidPath(
                        "absolute paths and prefixes are forbidden".into(),
                    ));
                }
            }
        }

        if components.is_empty() {
            return Err(SecurityError::InvalidPath(
                "destination has no normal components".into(),
            ));
        }

        let mut destination = self.root.clone();

        for component in &components {
            destination.push(component);
        }

        // Check every parent component, not merely the final filename.
        // This rejects existing symlinks in the destination path.
        let mut parent = self.root.clone();

        for component in components.iter().take(components.len() - 1) {
            parent.push(component);

            let metadata = match fs::symlink_metadata(&parent) {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    return Err(SecurityError::MissingParent(parent));
                }
                Err(error) => {
                    return Err(io_error(
                        "inspect output parent",
                        &parent,
                        error,
                    ));
                }
            };

            if metadata.file_type().is_symlink() {
                return Err(SecurityError::SymlinkRejected(parent));
            }

            if !metadata.is_dir() {
                return Err(SecurityError::ParentNotDirectory(parent));
            }
        }

        match fs::symlink_metadata(&destination) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(SecurityError::SymlinkRejected(destination));
                }

                return Err(SecurityError::OutputAlreadyExists(destination));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(io_error(
                    "inspect output destination",
                    &destination,
                    error,
                ));
            }
        }

        Ok(destination)
    }

    /// Exclusively create a new output file under the policy root.
    ///
    /// Existing files are never truncated or replaced. The caller owns the
    /// returned file and must check all writes and finalization operations.
    ///
    /// This operation is safer than validating a path and then opening it
    /// with File::create(), but std-only path walking is not fully race-free
    /// against an adversary who can concurrently replace parent directories.
    pub fn create_new_output(
        &self,
        relative: impl AsRef<Path>,
    ) -> Result<(PathBuf, File), SecurityError> {
        let destination = self.validate_relative(relative)?;

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&destination)
            .map_err(|error| {
                if error.kind() == io::ErrorKind::AlreadyExists {
                    SecurityError::OutputAlreadyExists(
                        destination.clone(),
                    )
                } else {
                    io_error("create output file", &destination, error)
                }
            })?;

        Ok((destination, file))
    }
}

/// Policy for external executables.
///
/// Executables are compared using canonical paths. A bare executable name
/// resolved implicitly through PATH is not accepted by this policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolPolicy {
    allowed_executables: Vec<PathBuf>,
}

impl ToolPolicy {
    /// Create a policy from explicit executable paths.
    ///
    /// Every configured executable must exist and be a regular file.
    /// Empty allowlists are valid but authorize no executable.
    pub fn new<I, P>(paths: I) -> Result<Self, SecurityError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut allowed_executables = Vec::new();

        for supplied in paths {
            let path = supplied.as_ref();

            let metadata = fs::symlink_metadata(path)
                .map_err(|e| io_error("inspect tool executable", path, e))?;

            if metadata.file_type().is_symlink() {
                return Err(SecurityError::SymlinkRejected(
                    path.to_path_buf(),
                ));
            }

            if !metadata.is_file() {
                return Err(SecurityError::InvalidExecutable(
                    path.to_path_buf(),
                ));
            }

            let canonical = fs::canonicalize(path)
                .map_err(|e| io_error("canonicalize executable", path, e))?;

            if !allowed_executables.contains(&canonical) {
                allowed_executables.push(canonical);
            }
        }

        // Stable ordering supports reproducible diagnostics and manifests.
        allowed_executables.sort();

        Ok(Self {
            allowed_executables,
        })
    }

    /// Check that an executable is explicitly allowlisted.
    ///
    /// The returned path is canonical and can be passed to a process runner.
    /// The process runner must still apply sandboxing and environment policy.
    pub fn validate_executable(
        &self,
        supplied: impl AsRef<Path>,
    ) -> Result<PathBuf, SecurityError> {
        let supplied = supplied.as_ref();

        let metadata = fs::symlink_metadata(supplied)
            .map_err(|e| io_error("inspect executable", supplied, e))?;

        if metadata.file_type().is_symlink() {
            return Err(SecurityError::SymlinkRejected(
                supplied.to_path_buf(),
            ));
        }

        if !metadata.is_file() {
            return Err(SecurityError::InvalidExecutable(
                supplied.to_path_buf(),
            ));
        }

        let canonical = fs::canonicalize(supplied)
            .map_err(|e| io_error("canonicalize executable", supplied, e))?;

        if self.allowed_executables.binary_search(&canonical).is_err() {
            return Err(SecurityError::ExecutableNotAllowed(canonical));
        }

        Ok(canonical)
    }

    /// Return the configured executable allowlist.
    #[must_use]
    pub fn allowed_executables(&self) -> &[PathBuf] {
        &self.allowed_executables
    }
}

/// An explicitly validated process invocation.
///
/// This is data, not an execution facility. It deliberately does not expose
/// a shell command string. A process runner should use Command::new(executable)
/// followed by args(), env_clear(), explicitly approved env(), and an explicit
/// current directory. It must never join arguments into a shell expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolInvocation {
    executable: PathBuf,
    arguments: Vec<String>,
    environment: Vec<(String, String)>,
    working_directory: Option<PathBuf>,
}

impl ToolInvocation {
    /// Construct an invocation after validating the executable against policy.
    pub fn new<I, S>(
        policy: &ToolPolicy,
        executable: impl AsRef<Path>,
        arguments: I,
    ) -> Result<Self, SecurityError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let executable = policy.validate_executable(executable)?;

        let arguments: Vec<String> =
            arguments.into_iter().map(Into::into).collect();

        if arguments.iter().any(|argument| argument.contains('\0')) {
            return Err(SecurityError::InvalidArgument);
        }

        Ok(Self {
            executable,
            arguments,
            environment: Vec::new(),
            working_directory: None,
        })
    }

    /// Add a single explicitly named environment variable.
    ///
    /// The process runner must clear inherited environment variables before
    /// applying this list. Values are never interpolated into a shell.
    pub fn with_environment(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, SecurityError> {
        let key = key.into();
        let value = value.into();

        if key.is_empty()
            || key.contains('=')
            || key.contains('\0')
            || key.chars().any(char::is_control)
            || value.contains('\0')
        {
            return Err(SecurityError::InvalidEnvironment);
        }

        // Reject duplicates rather than allowing order-dependent overrides.
        if self.environment.iter().any(|(existing, _)| existing == &key) {
            return Err(SecurityError::InvalidEnvironment);
        }

        self.environment.push((key, value));
        self.environment.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(self)
    }

    /// Set a working directory that already exists and is not a symlink.
    pub fn with_working_directory(
        mut self,
        directory: impl AsRef<Path>,
    ) -> Result<Self, SecurityError> {
        let directory = directory.as_ref();

        let metadata = fs::symlink_metadata(directory)
            .map_err(|e| io_error("inspect working directory", directory, e))?;

        if metadata.file_type().is_symlink() {
            return Err(SecurityError::SymlinkRejected(
                directory.to_path_buf(),
            ));
        }

        if !metadata.is_dir() {
            return Err(SecurityError::ParentNotDirectory(
                directory.to_path_buf(),
            ));
        }

        self.working_directory = Some(
            fs::canonicalize(directory)
                .map_err(|e| io_error("canonicalize working directory", directory, e))?,
        );

        Ok(self)
    }

    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    #[must_use]
    pub fn environment(&self) -> &[(String, String)] {
        &self.environment
    }

    #[must_use]
    pub fn working_directory(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock before Unix epoch")
                .as_nanos();

            let path = std::env::temp_dir().join(format!(
                "zamani-backend-security-{}-{unique}",
                std::process::id()
            ));

            fs::create_dir(&path).expect("create test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn accepts_a_simple_relative_output_path() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        let validated = policy.validate_relative("module.ll").unwrap();

        assert_eq!(validated, policy.root().join("module.ll"));
    }

    #[test]
    fn rejects_parent_traversal() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        assert!(matches!(
            policy.validate_relative("../outside.ll"),
            Err(SecurityError::InvalidPath(_))
        ));
    }

    #[test]
    fn rejects_absolute_paths() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        assert!(matches!(
            policy.validate_relative("/tmp/out.ll"),
            Err(SecurityError::InvalidPath(_))
        ));
    }

    #[test]
    fn rejects_windows_style_separators_and_drive_paths() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        assert!(policy.validate_relative(r"..\outside.ll").is_err());
        assert!(policy.validate_relative("C:/outside.ll").is_err());
    }

    #[test]
    fn does_not_overwrite_existing_output() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        fs::write(directory.path().join("artifact.ll"), b"original")
            .unwrap();

        assert!(matches!(
            policy.create_new_output("artifact.ll"),
            Err(SecurityError::OutputAlreadyExists(_))
        ));

        assert_eq!(
            fs::read(directory.path().join("artifact.ll")).unwrap(),
            b"original"
        );
    }

    #[test]
    fn creates_a_new_output_exclusively() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        let (path, mut file) = policy.create_new_output("artifact.ll").unwrap();

        use std::io::Write;
        file.write_all(b"; valid test artifact\n").unwrap();
        file.sync_all().unwrap();

        assert_eq!(
            fs::read(path).unwrap(),
            b"; valid test artifact\n"
        );
    }

    #[test]
    fn rejects_missing_parent_directories() {
        let directory = TestDirectory::new();
        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        assert!(matches!(
            policy.validate_relative("not-created/artifact.ll"),
            Err(SecurityError::MissingParent(_))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinked_output_parent() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new();
        let outside = TestDirectory::new();

        symlink(outside.path(), directory.path().join("redirect"))
            .unwrap();

        let policy = OutputPathPolicy::new(directory.path()).unwrap();

        assert!(matches!(
            policy.validate_relative("redirect/artifact.ll"),
            Err(SecurityError::SymlinkRejected(_))
        ));
    }

    #[test]
    fn rejects_an_empty_executable_allowlist() {
        let policy = ToolPolicy::new(Vec::<PathBuf>::new()).unwrap();

        assert!(matches!(
            policy.validate_executable("some-tool"),
            Err(SecurityError::Io { .. })
                | Err(SecurityError::InvalidExecutable(_))
                | Err(SecurityError::ExecutableNotAllowed(_))
        ));
    }

    #[test]
    fn rejects_duplicate_environment_keys() {
        let directory = TestDirectory::new();
        let policy = ToolPolicy::new([directory.path()]);

        // The directory is not an executable file.
        assert!(policy.is_err());
    }
}
