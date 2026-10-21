
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Reusable test infrastructure for Zamani backend implementations.
//!
//! Path: `src/backend/testing.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: Rust 1.97.
//!
//! # Purpose
//!
//! Provide deterministic, hardware-independent support for testing backend
//! contracts, fixture inputs, artifact storage, and external-tool behavior.
//!
//! # Ownership
//!
//! This module owns:
//! - Explicitly registered source and byte fixtures.
//! - Isolated temporary test workspaces.
//! - An in-memory artifact store for tests.
//! - A deterministic mock-toolchain interface.
//! - Generic assertions for determinism and contract outcomes.
//!
//! It does not own:
//! - Compilation, source semantics, IR verification, or target selection.
//! - Production artifact persistence or the production artifact cache.
//! - Real external process execution.
//! - Quantum simulation, HDL simulation, hardware access, or cluster execution.
//! - Production diagnostics, security policy, or telemetry.
//!
//! # Integration
//!
//! Declare `pub mod testing;` in the canonical `src/backend/mod.rs` after
//! resolving the existing `src/backend.rs` / `src/backend/mod.rs` ownership
//! conflict.
//!
//! Backend and target tests may use these utilities directly. They should
//! adapt their real compilation APIs to the generic assertion helpers instead
//! of adding a second compilation interface here.
//!
//! The mock toolchain is deliberately independent of the future production
//! `process.rs` and `toolchain.rs` APIs. When those APIs are frozen, adapters
//! can bridge them to this module without changing the fixture or workspace
//! contracts.
//!
//! # Resource and security contract
//!
//! No universal hardware limits are defined. Fixture size, stored artifact
//! count, and mock output sizes are bounded only when the test explicitly
//! configures a budget. Temporary files are created beneath an isolated
//! workspace. Relative paths containing traversal or root components are
//! rejected.
//!
//! Temporary workspaces are for tests, not a security sandbox. Do not use
//! this module to execute untrusted programs or to defend against a hostile
//! process with concurrent access to the same filesystem.
//!
//! No function in this module executes a real process or accesses a device.

use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

static WORKSPACE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Errors raised by test infrastructure.
///
/// These errors describe the test harness, not source-language or backend
/// compilation failures.
#[derive(Debug)]
pub enum TestingError {
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    InvalidRelativePath(PathBuf),
    FixtureAlreadyExists(String),
    FixtureNotFound(String),
    ArtifactAlreadyExists(String),
    ArtifactNotFound(String),
    InvalidKey(String),
    MockResponsesExhausted,
    InvalidConfiguration(String),
    LockPoisoned(&'static str),
}

impl fmt::Display for TestingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => write!(
                f,
                "test infrastructure could not {operation} `{}`: {source}",
                path.display()
            ),
            Self::InvalidRelativePath(path) => {
                write!(f, "test path must be a safe relative path: {}", path.display())
            }
            Self::FixtureAlreadyExists(name) => {
                write!(f, "test fixture `{name}` is already registered")
            }
            Self::FixtureNotFound(name) => {
                write!(f, "test fixture `{name}` was not registered")
            }
            Self::ArtifactAlreadyExists(key) => {
                write!(f, "test artifact `{key}` already exists")
            }
            Self::ArtifactNotFound(key) => {
                write!(f, "test artifact `{key}` was not found")
            }
            Self::InvalidKey(key) => write!(f, "invalid test key `{key}`"),
            Self::MockResponsesExhausted => {
                write!(f, "mock toolchain has no response queued for this invocation")
            }
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid test infrastructure configuration: {message}")
            }
            Self::LockPoisoned(name) => {
                write!(f, "test infrastructure lock `{name}` is poisoned")
            }
        }
    }
}

impl Error for TestingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn io_error(
    operation: &'static str,
    path: impl Into<PathBuf>,
    source: io::Error,
) -> TestingError {
    TestingError::Io {
        operation,
        path: path.into(),
        source,
    }
}

fn lock<'a, T>(
    mutex: &'a Mutex<T>,
    name: &'static str,
) -> Result<MutexGuard<'a, T>, TestingError> {
    mutex.lock().map_err(|_| TestingError::LockPoisoned(name))
}

fn validate_relative_path(path: &Path) -> Result<(), TestingError> {
    if path.as_os_str().is_empty() {
        return Err(TestingError::InvalidRelativePath(path.to_path_buf()));
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(TestingError::InvalidRelativePath(path.to_path_buf()));
            }
        }
    }

    Ok(())
}

fn validate_key(key: &str) -> Result<(), TestingError> {
    if key.is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
    {
        return Err(TestingError::InvalidKey(key.to_owned()));
    }

    Ok(())
}

/// One named fixture registered by a test suite.
///
/// The fixture content is kept as bytes so the harness can also support
/// binary inputs, serialized IR, generated artifacts, and malformed-input
/// regression cases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestFixture {
    name: String,
    bytes: Vec<u8>,
}

impl TestFixture {
    /// Create a fixture from arbitrary bytes.
    pub fn new(
        name: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self, TestingError> {
        let name = name.into();
        validate_key(&name)?;
        Ok(Self {
            name,
            bytes: bytes.into(),
        })
    }

    /// Create a text fixture.
    pub fn text(
        name: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<Self, TestingError> {
        Self::new(name, text.into().into_bytes())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Interpret the fixture as UTF-8 text.
    pub fn as_text(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.bytes)
    }
}

/// Explicit fixture catalog.
///
/// No assumptions are made about Zamani source syntax or about which fixtures
/// exist in the repository. A test suite registers the exact inputs it needs.
#[derive(Clone, Debug, Default)]
pub struct FixtureCatalog {
    fixtures: BTreeMap<String, TestFixture>,
}

impl FixtureCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a fixture, rejecting accidental duplicate names.
    pub fn register(&mut self, fixture: TestFixture) -> Result<(), TestingError> {
        let name = fixture.name.clone();

        if self.fixtures.contains_key(&name) {
            return Err(TestingError::FixtureAlreadyExists(name));
        }

        self.fixtures.insert(name, fixture);
        Ok(())
    }

    pub fn register_text(
        &mut self,
        name: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<(), TestingError> {
        self.register(TestFixture::text(name, text)?)
    }

    pub fn get(&self, name: &str) -> Result<&TestFixture, TestingError> {
        self.fixtures
            .get(name)
            .ok_or_else(|| TestingError::FixtureNotFound(name.to_owned()))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.fixtures.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }

    /// Names are returned in deterministic lexicographic order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.fixtures.keys().map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &TestFixture)> {
        self.fixtures
            .iter()
            .map(|(name, fixture)| (name.as_str(), fixture))
    }
}

/// An isolated temporary directory for a test.
///
/// The directory is created atomically beneath the operating system's
/// temporary directory. Its name is unique within the process under normal
/// operation; the filesystem's exclusive `create_dir` operation resolves
/// collisions safely.
///
/// The directory is removed on drop on a best-effort basis. Tests that need
/// to inspect failed outputs should copy them to an explicitly managed
/// location before the workspace is dropped.
#[derive(Debug)]
pub struct TestWorkspace {
    root: PathBuf,
}

impl TestWorkspace {
    /// Create a fresh workspace.
    pub fn new() -> Result<Self, TestingError> {
        let base = std::env::temp_dir();
        let process_id = std::process::id();

        loop {
            let sequence = WORKSPACE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let candidate = base.join(format!(
                "zamani-test-{process_id}-{sequence}"
            ));

            match fs::create_dir(&candidate) {
                Ok(()) => {
                    let root = fs::canonicalize(&candidate)
                        .map_err(|error| io_error("canonicalize workspace", &candidate, error))?;
                    return Ok(Self { root });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(io_error("create workspace", candidate, error)),
            }
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Resolve a safe relative path beneath the workspace.
    ///
    /// This validates path components but is not an OS-level sandbox against
    /// hostile concurrent filesystem modifications.
    pub fn path(&self, relative: impl AsRef<Path>) -> Result<PathBuf, TestingError> {
        let relative = relative.as_ref();
        validate_relative_path(relative)?;
        Ok(self.root.join(relative))
    }

    /// Write a file beneath the workspace, creating parent directories.
    ///
    /// This is intended for trusted test code. It refuses to replace an
    /// existing file, preventing tests from accidentally hiding prior output.
    pub fn write_new(
        &self,
        relative: impl AsRef<Path>,
        bytes: &[u8],
    ) -> Result<PathBuf, TestingError> {
        let path = self.path(relative)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| io_error("create test directory", parent, error))?;
        }

        use std::io::Write;

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| io_error("create test file", &path, error))?;

        file.write_all(bytes)
            .map_err(|error| io_error("write test file", &path, error))?;

        Ok(path)
    }

    pub fn write_fixture(
        &self,
        fixture: &TestFixture,
        relative: impl AsRef<Path>,
    ) -> Result<PathBuf, TestingError> {
        self.write_new(relative, fixture.bytes())
    }

    pub fn read(&self, relative: impl AsRef<Path>) -> Result<Vec<u8>, TestingError> {
        let path = self.path(relative)?;
        fs::read(&path).map_err(|error| io_error("read test file", path, error))
    }
}

impl Drop for TestWorkspace {
    fn drop(&mut self) {
        // Cleanup must not panic during test unwinding. Callers needing a
        // cleanup failure reported explicitly should remove the workspace
        // themselves before dropping it.
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Immutable artifact data retained by the in-memory test store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestArtifact {
    key: String,
    media_type: String,
    bytes: Vec<u8>,
}

impl TestArtifact {
    pub fn new(
        key: impl Into<String>,
        media_type: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self, TestingError> {
        let key = key.into();
        let media_type = media_type.into();
        validate_key(&key)?;
        validate_key(&media_type)?;

        Ok(Self {
            key,
            media_type,
            bytes: bytes.into(),
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
    }
}

/// Thread-safe in-memory artifact store for tests.
///
/// The store is deliberately separate from production `cache.rs` and
/// `artifact.rs`. It does not imply that an artifact has been verified,
/// linked, signed, or proven portable.
#[derive(Debug, Default)]
pub struct TestArtifactStore {
    artifacts: Mutex<BTreeMap<String, TestArtifact>>,
}

impl TestArtifactStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert without overwriting an artifact with the same key.
    pub fn insert_new(&self, artifact: TestArtifact) -> Result<(), TestingError> {
        let key = artifact.key.clone();
        let mut artifacts = lock(&self.artifacts, "artifact store")?;

        if artifacts.contains_key(&key) {
            return Err(TestingError::ArtifactAlreadyExists(key));
        }

        artifacts.insert(key, artifact);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<TestArtifact, TestingError> {
        lock(&self.artifacts, "artifact store")?
            .get(key)
            .cloned()
            .ok_or_else(|| TestingError::ArtifactNotFound(key.to_owned()))
    }

    pub fn contains(&self, key: &str) -> Result<bool, TestingError> {
        Ok(lock(&self.artifacts, "artifact store")?.contains_key(key))
    }

    pub fn len(&self) -> Result<usize, TestingError> {
        Ok(lock(&self.artifacts, "artifact store")?.len())
    }

    pub fn is_empty(&self) -> Result<bool, TestingError> {
        Ok(lock(&self.artifacts, "artifact store")?.is_empty())
    }

    /// Return a deterministic snapshot ordered by artifact key.
    pub fn snapshot(&self) -> Result<Vec<TestArtifact>, TestingError> {
        Ok(lock(&self.artifacts, "artifact store")?
            .values()
            .cloned()
            .collect())
    }

    pub fn clear(&self) -> Result<(), TestingError> {
        lock(&self.artifacts, "artifact store")?.clear();
        Ok(())
    }
}

/// A mock external-tool invocation.
///
/// This records an executable identifier and individual arguments. It does
/// not parse a shell command and never launches the named executable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockInvocation {
    executable: String,
    arguments: Vec<String>,
    working_directory: Option<PathBuf>,
}

impl MockInvocation {
    pub fn new(
        executable: impl Into<String>,
        arguments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, TestingError> {
        let executable = executable.into();
        validate_key(&executable)?;

        let arguments: Vec<String> = arguments.into_iter().map(Into::into).collect();
        if arguments
            .iter()
            .any(|argument| argument.contains('\0'))
        {
            return Err(TestingError::InvalidConfiguration(
                "mock argument contains a NUL byte".to_owned(),
            ));
        }

        Ok(Self {
            executable,
            arguments,
            working_directory: None,
        })
    }

    pub fn with_working_directory(
        mut self,
        directory: impl Into<PathBuf>,
    ) -> Self {
        self.working_directory = Some(directory.into());
        self
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn working_directory(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }
}

/// Deterministic result supplied by a mock toolchain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockToolResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl MockToolResult {
    pub fn success() -> Self {
        Self {
            exit_code: 0,
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    pub fn success_with_stdout(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            exit_code: 0,
            stdout: bytes.into(),
            stderr: Vec::new(),
        }
    }

    pub fn failure(
        exit_code: i32,
        stderr: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            exit_code,
            stdout: Vec::new(),
            stderr: stderr.into(),
        }
    }

    pub fn succeeded(&self) -> bool {
        self.exit_code == 0
    }
}

/// Deterministic mock toolchain with queued responses and recorded invocations.
///
/// Responses are consumed in invocation order. Tests must enqueue one response
/// per expected invocation; exhaustion is an explicit error rather than
/// fabricated success. A single instance can be shared between threads.
#[derive(Debug, Default)]
pub struct MockToolchain {
    responses: Mutex<VecDeque<MockToolResult>>,
    invocations: Mutex<Vec<MockInvocation>>,
}

impl MockToolchain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue_response(&self, response: MockToolResult) -> Result<(), TestingError> {
        lock(&self.responses, "mock responses")?.push_back(response);
        Ok(())
    }

    /// Record an invocation and return its next queued result.
    ///
    /// No real process is started.
    pub fn invoke(
        &self,
        invocation: MockInvocation,
    ) -> Result<MockToolResult, TestingError> {
        let mut responses = lock(&self.responses, "mock responses")?;
        let response = responses
            .pop_front()
            .ok_or(TestingError::MockResponsesExhausted)?;

        lock(&self.invocations, "mock invocations")?.push(invocation);
        Ok(response)
    }

    pub fn invocations(&self) -> Result<Vec<MockInvocation>, TestingError> {
        Ok(lock(&self.invocations, "mock invocations")?.clone())
    }

    pub fn remaining_responses(&self) -> Result<usize, TestingError> {
        Ok(lock(&self.responses, "mock responses")?.len())
    }

    pub fn assert_all_responses_consumed(&self) -> Result<(), TestingError> {
        let remaining = self.remaining_responses()?;
        if remaining == 0 {
            Ok(())
        } else {
            Err(TestingError::InvalidConfiguration(format!(
                "{remaining} mock toolchain response(s) were not consumed"
            )))
        }
    }
}

/// Assert that two executions produce the same result.
///
/// This is a generic helper: callers adapt their real backend API and decide
/// which inputs, configurations, target identities, and toolchain versions
/// must be held constant for the determinism claim to be meaningful.
pub fn assert_deterministic<T, E, F>(
    mut operation: F,
) -> Result<T, DeterminismFailure<E>>
where
    T: Clone + PartialEq + fmt::Debug,
    E: fmt::Debug,
    F: FnMut() -> Result<T, E>,
{
    let first = operation().map_err(DeterminismFailure::Execution)?;
    let second = operation().map_err(DeterminismFailure::Execution)?;

    if first != second {
        return Err(DeterminismFailure::DifferentResults {
            first,
            second,
        });
    }

    Ok(first)
}

/// Failure returned by [`assert_deterministic`].
#[derive(Debug, PartialEq, Eq)]
pub enum DeterminismFailure<E> {
    Execution(E),
    DifferentResults { first: String, second: String },
}

impl<E: fmt::Debug> fmt::Display for DeterminismFailure<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Execution(error) => {
                write!(f, "backend test operation failed: {error:?}")
            }
            Self::DifferentResults { first, second } => write!(
                f,
                "determinism contract failed: first result {first}, second result {second}"
            ),
        }
    }
}

/// Assert that two backend executions have identical results, retaining
/// debug representations in the failure for types that do not implement
/// `PartialEq` themselves.
pub fn assert_deterministic_debug<T, E, F>(
    mut operation: F,
) -> Result<T, DeterminismFailure<E>>
where
    T: Clone + fmt::Debug,
    E: fmt::Debug,
    F: FnMut() -> Result<T, E>,
{
    let first = operation().map_err(DeterminismFailure::Execution)?;
    let second = operation().map_err(DeterminismFailure::Execution)?;

    let first_debug = format!("{first:?}");
    let second_debug = format!("{second:?}");

    if first_debug != second_debug {
        return Err(DeterminismFailure::DifferentResults {
            first: first_debug,
            second: second_debug,
        });
    }

    Ok(first)
}

/// Assert that a backend operation fails for the supplied input.
///
/// The caller supplies the operation and its input type; this helper does not
/// assume that all failures share a particular error representation.
pub fn assert_rejected<I, O, E, F>(
    input: &I,
    operation: F,
) -> Result<E, ContractAssertionError>
where
    F: FnOnce(&I) -> Result<O, E>,
{
    match operation(input) {
        Ok(_) => Err(ContractAssertionError::ExpectedRejection),
        Err(error) => Ok(error),
    }
}

/// Generic failure for a backend contract assertion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractAssertionError {
    ExpectedRejection,
}

impl fmt::Display for ContractAssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedRejection => {
                write!(f, "backend contract expected rejection but operation succeeded")
            }
        }
    }
}

impl Error for ContractAssertionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_catalog_is_deterministic_and_rejects_duplicates() {
        let mut catalog = FixtureCatalog::new();
        catalog
            .register_text("quantum", "fixture bytes")
            .expect("register fixture");
        catalog
            .register_text("classical", "another fixture")
            .expect("register fixture");

        assert_eq!(
            catalog.names().collect::<Vec<_>>(),
            vec!["classical", "quantum"]
        );
        assert!(matches!(
            catalog.register_text("quantum", "duplicate"),
            Err(TestingError::FixtureAlreadyExists(_))
        ));
    }

    #[test]
    fn fixture_text_rejects_invalid_utf8() {
        let fixture = TestFixture::new("binary", vec![0xff, 0xfe])
            .expect("binary fixture is valid");
        assert!(fixture.as_text().is_err());
    }

    #[test]
    fn workspace_writes_and_reads_nested_files() {
        let workspace = TestWorkspace::new().expect("create workspace");
        let path = workspace
            .write_new("fixtures/classical/input.zm", b"test input")
            .expect("write fixture");

        assert!(path.is_file());
        assert_eq!(
            workspace
                .read("fixtures/classical/input.zm")
                .expect("read fixture"),
            b"test input"
        );
    }

    #[test]
    fn workspace_rejects_path_traversal_and_absolute_paths() {
        let workspace = TestWorkspace::new().expect("create workspace");

        assert!(matches!(
            workspace.path("../outside"),
            Err(TestingError::InvalidRelativePath(_))
        ));
        assert!(matches!(
            workspace.path(workspace.root().join("absolute")),
            Err(TestingError::InvalidRelativePath(_))
        ));
    }

    #[test]
    fn workspace_does_not_overwrite_existing_files() {
        let workspace = TestWorkspace::new().expect("create workspace");
        workspace
            .write_new("artifact.bin", b"first")
            .expect("write first artifact");

        assert!(workspace.write_new("artifact.bin", b"second").is_err());
        assert_eq!(
            workspace.read("artifact.bin").expect("read artifact"),
            b"first"
        );
    }

    #[test]
    fn artifact_store_rejects_duplicate_keys_and_returns_snapshots() {
        let store = TestArtifactStore::new();
        store
            .insert_new(
                TestArtifact::new("module-a", "application/octet-stream", b"abc".to_vec())
                    .expect("create artifact"),
            )
            .expect("insert artifact");

        assert_eq!(store.get("module-a").expect("get artifact").bytes(), b"abc");
        assert!(matches!(
            store.insert_new(
                TestArtifact::new("module-a", "application/octet-stream", b"xyz".to_vec())
                    .expect("create duplicate"),
            ),
            Err(TestingError::ArtifactAlreadyExists(_))
        ));
        assert_eq!(store.snapshot().expect("snapshot").len(), 1);
    }

    #[test]
    fn mock_toolchain_records_calls_and_never_fabricates_success() {
        let mock = MockToolchain::new();
        mock.enqueue_response(MockToolResult::success_with_stdout(b"validated".to_vec()))
            .expect("queue response");

        let invocation = MockInvocation::new("validator", ["--check", "module.wasm"])
            .expect("create invocation");
        let result = mock.invoke(invocation.clone()).expect("mock invocation");

        assert!(result.succeeded());
        assert_eq!(result.stdout, b"validated");
        assert_eq!(mock.invocations().expect("invocations"), vec![invocation]);
        mock.assert_all_responses_consumed()
            .expect("all responses consumed");

        assert!(matches!(
            mock.invoke(
                MockInvocation::new("validator", ["--check"])
                    .expect("create invocation")
            ),
            Err(TestingError::MockResponsesExhausted)
        ));
    }

    #[test]
    fn mock_toolchain_can_model_tool_failure() {
        let mock = MockToolchain::new();
        mock.enqueue_response(MockToolResult::failure(7, b"invalid artifact".to_vec()))
            .expect("queue response");

        let result = mock
            .invoke(MockInvocation::new("validator", ["bad.wasm"]).expect("invocation"))
            .expect("mock returns a result");

        assert_eq!(result.exit_code, 7);
        assert!(!result.succeeded());
        assert_eq!(result.stderr, b"invalid artifact");
    }

    #[test]
    fn deterministic_assertion_accepts_identical_results() {
        let mut call_count = 0;
        let result = assert_deterministic(|| {
            call_count += 1;
            Ok::<_, &'static str>("stable artifact")
        })
        .expect("same result");

        assert_eq!(result, "stable artifact");
        assert_eq!(call_count, 2);
    }

    #[test]
    fn deterministic_assertion_reports_different_results() {
        let mut call_count = 0;
        let result = assert_deterministic(|| {
            call_count += 1;
            Ok::<_, &'static str>(call_count)
        });

        assert!(matches!(
            result,
            Err(DeterminismFailure::DifferentResults { .. })
        ));
    }

    #[test]
    fn rejection_assertion_accepts_an_explicit_error() {
        let error = assert_rejected(&false, |accepted| {
            if *accepted {
                Ok("accepted")
            } else {
                Err("unsupported capability")
            }
        })
        .expect("operation should reject");

        assert_eq!(error, "unsupported capability");
    }

    #[test]
    fn rejection_assertion_detects_unexpected_success() {
        let result = assert_rejected(&true, |accepted| {
            if *accepted {
                Ok("accepted")
            } else {
                Err("rejected")
            }
        });

        assert_eq!(result, Err(ContractAssertionError::ExpectedRejection));
    }

    #[test]
    fn keys_reject_control_characters_and_whitespace_padding() {
        assert!(TestArtifact::new("bad\nkey", "text/plain", b"x".to_vec()).is_err());
        assert!(TestFixture::new(" padded ", b"x".to_vec()).is_err());
    }
}
