#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Backend security integration and regression tests.
//!
//! Path: src/backend/tests/security.rs
//!
//! Purpose:
//!     Verify the security boundary exposed by `crate::backend::security`
//!     and its integration with the existing compiler safety guard.
//!
//! Ownership:
//!     These tests do not implement security policy. They test the existing
//!     production APIs and ensure that security failures remain explicit.
//!
//! Integration:
//!     - src/backend/security.rs:
//!       OutputPathPolicy, SecurityError, ToolPolicy, ToolInvocation.
//!     - src/backend/process.rs:
//!       Process execution must continue to use argument vectors, explicit
//!       environment policy, cancellation, timeouts, and typed errors.
//!     - src/compiler/safety_guard.rs:
//!       Instruction-stream threat inspection remains owned by the compiler
//!       safety guard; this test suite verifies that the backend does not
//!       replace or bypass it.
//!     - src/compiler/linker.rs:
//!       Native linking remains a separate operation. Security policy must
//!       be applied before untrusted tool configuration and output paths are
//!       passed to a process or filesystem operation.
//!     - grammar/specification/semantic-model.md:
//!       Security controls must not redefine program semantics.
//!     - grammar/validation/semantic-boundaries.md and
//!       grammar/validation/scalability-rules.md:
//!       Operational safeguards must not become universal hardware limits.
//!
//! Safety:
//!     No unsafe Rust is permitted.
//!
//! Resource policy:
//!     The test harness uses temporary files and directories only. It does
//!     not impose universal limits on source size, qubits, CPUs, devices,
//!     memory, tensor dimensions, or target capacity.
//!
//! Note:
//!     The standard-library filesystem APIs cannot eliminate every TOCTOU
//!     race. Tests of path validation are not a substitute for OS-level
//!     sandboxing or directory-handle-relative operations when hostile
//!     concurrent filesystem modification is in scope.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::backend::security::{
    OutputPathPolicy, SecurityError, ToolInvocation, ToolPolicy,
};
use crate::compiler::safety_guard::{
    CoordinatedThreatRule, GlobalSecurityContext, SafetyGuard, SafetyGuardError,
    SafetyPolicy, SecurityDecision,
};

/// Generates distinct names for independent test directories.
///
/// This is test isolation, not a limit on program or hardware capacity.
static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Owns one temporary test directory and removes it on drop.
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be at or after the Unix epoch")
            .as_nanos();

        let sequence = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);

        let path = std::env::temp_dir().join(format!(
            "zamani-backend-security-{}-{timestamp}-{sequence}",
            std::process::id(),
        ));

        fs::create_dir(&path).expect("create isolated security-test directory");

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn child(&self, relative: impl AsRef<Path>) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        // Cleanup is best effort so a cleanup failure does not mask the
        // assertion or security error that caused the test to finish.
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn output_policy(directory: &TestDirectory) -> OutputPathPolicy {
    OutputPathPolicy::new(directory.path())
        .expect("an existing ordinary directory should be a valid output root")
}

fn assert_invalid_relative_path(policy: &OutputPathPolicy, path: &str) {
    assert!(
        matches!(
            policy.validate_relative(path),
            Err(SecurityError::InvalidPath(_))
        ),
        "expected InvalidPath for destination {path:?}"
    );
}

// ---------------------------------------------------------------------------
// Output path validation
// ---------------------------------------------------------------------------

#[test]
fn accepts_a_normal_relative_destination_without_writing_a_file() {
    let directory = TestDirectory::new();
    let policy = output_policy(&directory);

    let destination = policy
        .validate_relative("artifacts/module.ll")
        .expect_err("the unprovisioned parent directory must be rejected");

    assert!(
        matches!(destination, SecurityError::MissingParent(_)),
        "missing output parents must be reported explicitly"
    );

    fs::create_dir(directory.child("artifacts"))
        .expect("provision the intended output directory");

    let destination = policy
        .validate_relative("artifacts/module.ll")
        .expect("an ordinary relative path with existing parents is valid");

    assert_eq!(destination, directory.child("artifacts/module.ll"));
    assert!(!destination.exists());
}

#[test]
fn rejects_empty_and_non_portable_destinations() {
    let directory = TestDirectory::new();
    let policy = output_policy(&directory);

    for path in [
        "",
        ".",
        "./artifact.ll",
        "..",
        "../outside.ll",
        "nested/../../outside.ll",
        "/absolute/path.ll",
        "C:/outside.ll",
        "C:\\outside.ll",
        "nested\\artifact.ll",
        "dir/name:stream",
        "line\nbreak.ll",
        "tab\tcharacter.ll",
    ] {
        assert_invalid_relative_path(&policy, path);
    }
}

#[test]
fn rejects_a_destination_with_an_unprovisioned_parent() {
    let directory = TestDirectory::new();
    let policy = output_policy(&directory);

    let result = policy.validate_relative("not-created/output.bin");

    assert!(
        matches!(result, Err(SecurityError::MissingParent(_))),
        "the output policy must not create parent directories implicitly"
    );

    assert!(!directory.child("not-created").exists());
}

#[test]
fn rejects_a_regular_file_used_as_an_output_parent() {
    let directory = TestDirectory::new();
    fs::write(directory.child("parent"), b"not a directory")
        .expect("create a regular file");

    let policy = output_policy(&directory);
    let result = policy.validate_relative("parent/output.bin");

    assert!(
        matches!(result, Err(SecurityError::ParentNotDirectory(_))),
        "a file must not be accepted as an output directory"
    );
}

#[test]
fn rejects_an_existing_destination_instead_of_overwriting_it() {
    let directory = TestDirectory::new();
    let destination = directory.child("artifact.ll");
    fs::write(&destination, b"original artifact")
        .expect("create an existing artifact");

    let policy = output_policy(&directory);
    let result = policy.validate_relative("artifact.ll");

    assert!(
        matches!(result, Err(SecurityError::OutputAlreadyExists(_))),
        "existing output must be rejected"
    );

    assert_eq!(
        fs::read(&destination).expect("read the existing artifact"),
        b"original artifact",
        "validation must not modify an existing artifact"
    );
}

#[test]
fn creates_a_new_artifact_exclusively() {
    let directory = TestDirectory::new();
    let policy = output_policy(&directory);

    let (path, mut file) = policy
        .create_new_output("module.ll")
        .expect("create a previously nonexistent artifact");

    file.write_all(b"; generated artifact\n")
        .expect("write artifact contents");
    file.flush().expect("flush artifact contents");
    drop(file);

    assert_eq!(path, directory.child("module.ll"));
    assert_eq!(
        fs::read(&path).expect("read generated artifact"),
        b"; generated artifact\n"
    );

    let second_attempt = policy.create_new_output("module.ll");

    assert!(
        matches!(
            second_attempt,
            Err(SecurityError::OutputAlreadyExists(_))
        ),
        "a second creation attempt must not truncate or replace the artifact"
    );

    assert_eq!(
        fs::read(&path).expect("read artifact after rejected overwrite"),
        b"; generated artifact\n"
    );
}

#[test]
fn validates_paths_without_creating_or_mutating_artifacts() {
    let directory = TestDirectory::new();
    fs::create_dir(directory.child("outputs"))
        .expect("create output directory");

    let policy = output_policy(&directory);

    let _ = policy
        .validate_relative("outputs/program.wasm")
        .expect("validate destination");

    assert!(
        !directory.child("outputs/program.wasm").exists(),
        "validation must not itself publish an artifact"
    );
}

// ---------------------------------------------------------------------------
// Symbolic-link handling
// ---------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn rejects_a_symbolic_link_as_the_output_root() {
    use std::os::unix::fs::symlink;

    let directory = TestDirectory::new();
    let real_root = directory.child("real-root");
    fs::create_dir(&real_root).expect("create real root");

    let linked_root = directory.child("linked-root");
    symlink(&real_root, &linked_root).expect("create root symlink");

    let result = OutputPathPolicy::new(&linked_root);

    assert!(
        matches!(result, Err(SecurityError::SymlinkRejected(_))),
        "a symbolic link must not be accepted as the configured root"
    );
}

#[cfg(unix)]
#[test]
fn rejects_a_symbolic_link_in_a_destination_parent() {
    use std::os::unix::fs::symlink;

    let directory = TestDirectory::new();
    let external = directory.child("external");
    fs::create_dir(&external).expect("create external directory");

    let linked_parent = directory.child("linked-parent");
    symlink(&external, &linked_parent).expect("create parent symlink");

    let policy = output_policy(&directory);
    let result = policy.validate_relative("linked-parent/output.bin");

    assert!(
        matches!(result, Err(SecurityError::SymlinkRejected(_))),
        "destination parents must not escape the policy root through symlinks"
    );
}

#[cfg(unix)]
#[test]
fn rejects_a_symbolic_link_at_the_final_destination() {
    use std::os::unix::fs::symlink;

    let directory = TestDirectory::new();
    let external = directory.child("external.bin");
    fs::write(&external, b"do not replace").expect("create external file");

    let linked_destination = directory.child("artifact.bin");
    symlink(&external, &linked_destination).expect("create destination symlink");

    let policy = output_policy(&directory);
    let result = policy.validate_relative("artifact.bin");

    assert!(
        matches!(result, Err(SecurityError::SymlinkRejected(_))),
        "the final output destination must not be a symbolic link"
    );

    assert_eq!(
        fs::read(external).expect("read external file"),
        b"do not replace"
    );
}

// ---------------------------------------------------------------------------
// Executable allowlisting and invocation construction
// ---------------------------------------------------------------------------

#[test]
fn allows_only_an_explicitly_configured_executable() {
    let directory = TestDirectory::new();
    let allowed = std::env::current_exe()
        .expect("the test executable path must be available");
    let denied = directory.child("unlisted-tool");
    fs::write(&denied, b"not an executable tool")
        .expect("create a regular file that is not allowlisted");

    let policy = ToolPolicy::new([allowed.clone()])
        .expect("the current test executable is a regular file");

    let canonical_allowed =
        fs::canonicalize(&allowed).expect("canonicalize allowed executable");

    assert_eq!(
        policy
            .validate_executable(&allowed)
            .expect("the explicitly configured executable should be accepted"),
        canonical_allowed
    );

    assert!(
        matches!(
            policy.validate_executable(&denied),
            Err(SecurityError::ExecutableNotAllowed(_))
        ),
        "an unlisted executable must be rejected"
    );
}

#[test]
fn an_empty_allowlist_authorizes_no_executable() {
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy = ToolPolicy::new(std::iter::empty::<PathBuf>())
        .expect("an empty allowlist is a valid deny-all policy");

    assert!(
        matches!(
            policy.validate_executable(executable),
            Err(SecurityError::ExecutableNotAllowed(_))
        ),
        "an empty allowlist must not implicitly trust PATH or the host"
    );
}

#[test]
fn rejects_a_bare_or_missing_executable_path() {
    let policy = ToolPolicy::new(std::iter::empty::<PathBuf>())
        .expect("construct deny-all policy");

    let result = policy.validate_executable("tool-that-does-not-exist");

    assert!(
        result.is_err(),
        "missing or unresolved executable paths must not be trusted"
    );
}

#[cfg(unix)]
#[test]
fn rejects_a_symbolic_link_as_an_allowlisted_executable() {
    use std::os::unix::fs::symlink;

    let directory = TestDirectory::new();
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let linked_executable = directory.child("linked-tool");

    symlink(executable, &linked_executable)
        .expect("create executable symlink");

    assert!(
        matches!(
            ToolPolicy::new([linked_executable]),
            Err(SecurityError::SymlinkRejected(_))
        ),
        "a symlink must not be silently accepted into the executable allowlist"
    );
}

#[test]
fn keeps_shell_metacharacters_as_literal_arguments() {
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy =
        ToolPolicy::new([executable.clone()]).expect("allow test executable");

    let arguments = [
        "argument with spaces".to_owned(),
        "$(do-not-execute)".to_owned(),
        "; touch should-not-exist".to_owned(),
        "| arbitrary-command".to_owned(),
        "--output=path with spaces".to_owned(),
    ];

    let invocation = ToolInvocation::new(
        &policy,
        executable,
        arguments.iter().cloned(),
    )
    .expect("ordinary argument strings must remain valid");

    assert_eq!(
        invocation.arguments(),
        arguments.as_slice(),
        "arguments must not be joined, split, or interpreted as shell syntax"
    );
}

#[test]
fn rejects_nul_bytes_in_arguments() {
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy =
        ToolPolicy::new([executable.clone()]).expect("allow test executable");

    let result = ToolInvocation::new(
        &policy,
        executable,
        [String::from("valid"), String::from("invalid\0argument")],
    );

    assert!(
        matches!(result, Err(SecurityError::InvalidArgument)),
        "NUL bytes must be rejected before process creation"
    );
}

#[test]
fn rejects_invalid_environment_entries() {
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy =
        ToolPolicy::new([executable.clone()]).expect("allow test executable");

    let invocation = ToolInvocation::new(&policy, executable, ["--version"])
        .expect("construct an invocation");

    assert!(
        matches!(
            invocation.clone().with_environment("", "value"),
            Err(SecurityError::InvalidEnvironment)
        ),
        "environment keys must not be empty"
    );

    assert!(
        matches!(
            invocation.clone().with_environment("INVALID=KEY", "value"),
            Err(SecurityError::InvalidEnvironment)
        ),
        "environment keys must not contain '='"
    );

    assert!(
        matches!(
            invocation.clone().with_environment("INVALID\nKEY", "value"),
            Err(SecurityError::InvalidEnvironment)
        ),
        "environment keys must not contain control characters"
    );

    assert!(
        matches!(
            invocation.clone().with_environment("VALID_KEY", "value\0"),
            Err(SecurityError::InvalidEnvironment)
        ),
        "environment values must not contain NUL"
    );
}

#[test]
fn rejects_duplicate_environment_keys() {
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy =
        ToolPolicy::new([executable.clone()]).expect("allow test executable");

    let invocation = ToolInvocation::new(&policy, executable, ["--version"])
        .expect("construct invocation")
        .with_environment("LANG", "C")
        .expect("add first environment value");

    assert!(
        matches!(
            invocation.with_environment("LANG", "another-value"),
            Err(SecurityError::InvalidEnvironment)
        ),
        "duplicate keys must not create order-dependent environment policy"
    );
}

#[test]
fn accepts_and_canonicalizes_an_existing_working_directory() {
    let directory = TestDirectory::new();
    let executable =
        std::env::current_exe().expect("test executable path must be available");
    let policy =
        ToolPolicy::new([executable.clone()]).expect("allow test executable");

    let invocation = ToolInvocation::new(&policy, executable, ["--version"])
        .expect("construct invocation")
        .with_working_directory(directory.path())
        .expect("configure an existing working directory");

    assert_eq!(
        invocation.working_directory(),
        Some(
            fs::canonicalize(directory.path())
                .expect("canonicalize working directory")
                .as_path()
        )
    );
}

// ---------------------------------------------------------------------------
// Existing compiler security guard integration
// ---------------------------------------------------------------------------

#[test]
fn compiler_safety_guard_blocks_a_known_cross_substrate_threat() {
    let mut context = GlobalSecurityContext::new();

    let precursor = SafetyGuard::try_new("NEUROMORPHIC")
        .expect("valid substrate identifier");
    let quantum = SafetyGuard::try_new("QUANTUM")
        .expect("valid substrate identifier");

    precursor
        .inspect_with_policy(
            &[String::from("PREPARE_SHARED_BUFFER")],
            &mut context,
        )
        .expect("the precursor alone should be recorded");

    let result = quantum.inspect_with_policy(
        &[String::from("EXPLOIT_SHARED_BUS")],
        &mut context,
    );

    assert!(
        matches!(result, Err(SafetyGuardError::ThreatDetected { .. })),
        "a matching coordinated threat must fail closed"
    );

    assert!(
        context.audit_events().iter().any(|event| {
            event.decision == SecurityDecision::Block
        }),
        "blocking a threat must leave a structured audit event"
    );
}

#[test]
fn compiler_safety_guard_rejects_malformed_identifiers() {
    assert!(
        matches!(
            SafetyGuard::try_new("bad\nsubstrate"),
            Err(SafetyGuardError::InvalidSubstrate(_))
        ),
        "control characters must not be accepted in substrate identifiers"
    );

    let mut context = GlobalSecurityContext::new();

    assert!(
        matches!(
            context.try_record_signal("bad\0signal"),
            Err(SafetyGuardError::InvalidSignal(_))
        ),
        "malformed security signals must be rejected"
    );

    assert!(
        matches!(
            context.try_check_coordinated_threat("QUANTUM", "bad\ninstruction"),
            Err(SafetyGuardError::InvalidInstruction(_))
        ),
        "malformed instruction identifiers must be rejected"
    );
}

#[test]
fn compiler_safety_context_enforces_configured_operational_budgets() {
    let policy = SafetyPolicy {
        max_signals: 1,
        max_audit_events: 1,
        ..SafetyPolicy::default()
    };

    let mut context = GlobalSecurityContext::with_policy(policy)
        .expect("positive operational budgets should be valid");

    assert_eq!(
        context
            .try_record_signal("FIRST_SIGNAL")
            .expect("first distinct signal should fit the configured budget"),
        true
    );

    assert!(
        matches!(
            context.try_record_signal("SECOND_SIGNAL"),
            Err(SafetyGuardError::ContextLimitExceeded)
        ),
        "resource-budget exhaustion must be explicit, not silent"
    );

    assert_eq!(context.signal_count(), 1);
}

#[test]
fn compiler_safety_policy_rejects_zero_capacity() {
    let policy = SafetyPolicy {
        max_signals: 0,
        ..SafetyPolicy::default()
    };

    assert!(
        matches!(
            GlobalSecurityContext::with_policy(policy),
            Err(SafetyGuardError::ContextLimitExceeded)
        ),
        "a zero operational capacity must not be interpreted as unlimited"
    );
}

#[test]
fn custom_threat_rules_are_supported_without_changing_backend_targets() {
    let rule = CoordinatedThreatRule::new(
        "custom.cross-domain-rule",
        "CUSTOM_PRECURSOR",
        "CUSTOM_ACCELERATOR",
        "CUSTOM_UNSAFE_OPERATION",
    )
    .expect("construct a valid extensible threat rule");

    let policy = SafetyPolicy {
        threat_rules: vec![rule],
        ..SafetyPolicy::default()
    };

    let mut context = GlobalSecurityContext::with_policy(policy)
        .expect("custom rules should be accepted");

    context
        .try_record_signal("CUSTOM_PRECURSOR")
        .expect("record custom precursor");

    let guard = SafetyGuard::try_new("CUSTOM_ACCELERATOR")
        .expect("custom substrate identifiers should be supported");

    let result = guard.inspect_with_policy(
        &[String::from("CUSTOM_UNSAFE_OPERATION")],
        &mut context,
    );

    assert!(
        matches!(result, Err(SafetyGuardError::ThreatDetected { .. })),
        "custom security rules must be evaluated without a closed target enum"
    );
}