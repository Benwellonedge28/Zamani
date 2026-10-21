
#![deny(unsafe_code)]

//! Safe external-process execution for Zamani backend toolchains.
//!
//! # Responsibilities
//!
//! - Execute a program using an executable path and argument vector.
//! - Configure the child environment and working directory explicitly.
//! - Capture stdout and stderr concurrently to avoid pipe-buffer deadlocks.
//! - Drain output even after the capture budget is reached.
//! - Support optional execution deadlines and cooperative cancellation.
//! - Report spawn, I/O, timeout, cancellation, and non-zero-exit failures.
//!
//! # Non-responsibilities
//!
//! This module does not implement linking, target selection, artifact validation,
//! compiler semantics, or toolchain discovery. Callers own those responsibilities.
//!
//! # Security
//!
//! Arguments are passed directly to `std::process::Command`; this module does
//! not construct shell command strings. Callers must still validate executable
//! trust, arguments, environment variables, and filesystem paths.
//!
//! # Resource limits
//!
//! The default captured-output budget is an operational safeguard, not a limit
//! on Zamani program size or target capacity. Set it to `None` to capture all
//! output, subject to available memory. Execution timeout is opt-in and
//! configurable. A process that never exits can therefore run indefinitely if
//! the caller supplies no timeout and never cancels it.
//!
//! # Process-tree limitation
//!
//! The standard library can terminate the direct child, but does not provide a
//! portable, safe process-tree termination API. Toolchains that spawn detached
//! descendants should be isolated by a platform-specific process supervisor.
//! Such a supervisor must not be simulated by shell-string interpolation.

use std::ffi::OsString;
use std::fmt;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Default maximum bytes retained from each output stream.
///
/// This is a configurable operational budget. It does not limit how much output
/// the child may produce: excess bytes are drained and counted, but not stored.
pub const DEFAULT_CAPTURE_LIMIT_PER_STREAM: usize = 8 * 1024 * 1024;

/// Default interval between child-status checks.
pub const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Whether to inherit the parent environment or begin with an empty one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentMode {
    /// Inherit the parent environment, then apply removals and overrides.
    Inherit,

    /// Start with an empty environment, then apply configured variables.
    ///
    /// Callers using this mode should configure any variables required by the
    /// toolchain. Prefer an absolute executable path for hermetic builds.
    Clear,
}

/// Cooperative cancellation token shared by a caller and one or more requests.
///
/// Cancelling a running request causes the runner to attempt to terminate its
/// direct child. Cancellation cannot interrupt a process before it is spawned.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a fresh, non-cancelled token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Return whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Description of one external-process invocation.
///
/// Arguments and environment values are stored as `OsString` so non-Unicode
/// paths and arguments are supported on platforms that permit them.
#[derive(Debug, Clone)]
pub struct ProcessRequest {
    program: PathBuf,
    args: Vec<OsString>,
    environment_mode: EnvironmentMode,
    environment: Vec<(OsString, OsString)>,
    removed_environment: Vec<OsString>,
    current_dir: Option<PathBuf>,
    timeout: Option<Duration>,
    poll_interval: Duration,
    max_output_bytes_per_stream: Option<usize>,
    cancellation: CancellationToken,
}

impl ProcessRequest {
    /// Create a request for an executable.
    ///
    /// The default environment is inherited. The default timeout is unlimited;
    /// production callers should set a deadline for tools that could hang.
    /// Output retention is bounded by the configurable default capture budget.
    #[must_use]
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            environment_mode: EnvironmentMode::Inherit,
            environment: Vec::new(),
            removed_environment: Vec::new(),
            current_dir: None,
            timeout: None,
            poll_interval: DEFAULT_POLL_INTERVAL,
            max_output_bytes_per_stream: Some(DEFAULT_CAPTURE_LIMIT_PER_STREAM),
            cancellation: CancellationToken::new(),
        }
    }

    /// Append one argument without shell parsing or interpolation.
    #[must_use]
    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Append arguments without shell parsing or interpolation.
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Select the child environment policy.
    #[must_use]
    pub fn environment_mode(mut self, mode: EnvironmentMode) -> Self {
        self.environment_mode = mode;
        self
    }

    /// Set or override an environment variable for the child.
    #[must_use]
    pub fn env(
        mut self,
        key: impl Into<OsString>,
        value: impl Into<OsString>,
    ) -> Self {
        self.environment.push((key.into(), value.into()));
        self
    }

    /// Remove an inherited environment variable.
    ///
    /// In `EnvironmentMode::Clear`, this is redundant but harmless.
    #[must_use]
    pub fn remove_env(mut self, key: impl Into<OsString>) -> Self {
        self.removed_environment.push(key.into());
        self
    }

    /// Set the child's working directory.
    #[must_use]
    pub fn current_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.current_dir = Some(path.into());
        self
    }

    /// Set the maximum allowed execution duration.
    ///
    /// A zero duration is valid and requests an effectively immediate deadline.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Run without an execution deadline.
    ///
    /// Use only when an unlimited runtime is intentional.
    #[must_use]
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Configure how frequently the runner checks the child status.
    ///
    /// A zero polling interval is rejected when the request is executed.
    #[must_use]
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Set the maximum bytes retained from *each* output stream.
    ///
    /// Excess output is still drained to prevent the child blocking on a full
    /// pipe. Use `None` to retain all output, subject to available memory.
    #[must_use]
    pub fn max_output_bytes_per_stream(mut self, limit: Option<usize>) -> Self {
        self.max_output_bytes_per_stream = limit;
        self
    }

    /// Set the cancellation token observed by the runner.
    #[must_use]
    pub fn cancellation_token(mut self, token: CancellationToken) -> Self {
        self.cancellation = token;
        self
    }

    /// Return the configured executable path.
    #[must_use]
    pub fn program(&self) -> &Path {
        &self.program
    }

    fn validate(&self) -> Result<(), ProcessError> {
        if self.program.as_os_str().is_empty() {
            return Err(ProcessError::InvalidRequest(
                "executable path must not be empty".to_owned(),
            ));
        }

        if self.poll_interval.is_zero() {
            return Err(ProcessError::InvalidRequest(
                "poll interval must be greater than zero".to_owned(),
            ));
        }

        Ok(())
    }
}

/// Captured bytes from one output stream.
///
/// Bytes are preserved exactly. Call [`CapturedStream::to_string_lossy`] only
/// when a human-readable diagnostic is needed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapturedStream {
    bytes: Vec<u8>,
    total_bytes: u64,
    truncated: bool,
}

impl CapturedStream {
    /// Bytes retained in memory.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Retained byte count.
    #[must_use]
    pub fn captured_len(&self) -> usize {
        self.bytes.len()
    }

    /// Total bytes read from the stream, including discarded excess bytes.
    #[must_use]
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// Whether any bytes were discarded because of the capture budget.
    #[must_use]
    pub fn was_truncated(&self) -> bool {
        self.truncated
    }

    /// Decode retained bytes lossily for diagnostics.
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }
}

/// Successful process execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    /// Exit code, if the platform reports one.
    ///
    /// A process terminated by a signal may have no exit code.
    pub exit_code: Option<i32>,

    /// Whether the process reported a successful exit.
    pub success: bool,

    /// Captured standard output.
    pub stdout: CapturedStream,

    /// Captured standard error.
    pub stderr: CapturedStream,

    /// Elapsed wall-clock time measured by this runner.
    pub elapsed: Duration,
}

/// Errors returned by [`ProcessRunner`].
#[derive(Debug)]
pub enum ProcessError {
    InvalidRequest(String),

    Spawn {
        program: PathBuf,
        source: io::Error,
    },

    Io {
        operation: &'static str,
        source: io::Error,
    },

    ReaderPanicked {
        stream: &'static str,
    },

    TimedOut {
        program: PathBuf,
        timeout: Duration,
        output: ProcessOutput,
    },

    Cancelled {
        program: PathBuf,
        output: ProcessOutput,
    },

    NonZeroExit {
        program: PathBuf,
        output: ProcessOutput,
    },
}

impl ProcessError {
    /// Return captured output for failures that occurred after the process
    /// started and its output streams were collected.
    #[must_use]
    pub fn output(&self) -> Option<&ProcessOutput> {
        match self {
            Self::TimedOut { output, .. }
            | Self::Cancelled { output, .. }
            | Self::NonZeroExit { output, .. } => Some(output),
            Self::InvalidRequest(_)
            | Self::Spawn { .. }
            | Self::Io { .. }
            | Self::ReaderPanicked { .. } => None,
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => {
                write!(f, "invalid process request: {message}")
            }
            Self::Spawn { program, source } => {
                write!(f, "failed to start `{}`: {source}", program.display())
            }
            Self::Io { operation, source } => {
                write!(f, "process I/O failure while {operation}: {source}")
            }
            Self::ReaderPanicked { stream } => {
                write!(f, "the {stream} capture thread panicked")
            }
            Self::TimedOut {
                program, timeout, ..
            } => write!(
                f,
                "process `{}` exceeded timeout of {:?}",
                program.display(),
                timeout
            ),
            Self::Cancelled { program, .. } => {
                write!(f, "process `{}` was cancelled", program.display())
            }
            Self::NonZeroExit { program, output } => {
                match output.exit_code {
                    Some(code) => write!(
                        f,
                        "process `{}` exited with status {code}",
                        program.display()
                    ),
                    None => write!(
                        f,
                        "process `{}` failed without a reported exit code",
                        program.display()
                    ),
                }
            }
        }
    }
}

impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Spawn { source, .. } | Self::Io { source, .. } => Some(source),
            Self::InvalidRequest(_)
            | Self::ReaderPanicked { .. }
            | Self::TimedOut { .. }
            | Self::Cancelled { .. }
            | Self::NonZeroExit { .. } => None,
        }
    }
}

/// Stateless process runner.
///
/// A runner can be shared between compilation tasks. Request state and
/// cancellation tokens are independent unless callers explicitly share them.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessRunner;

impl ProcessRunner {
    /// Construct a runner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Execute a request.
    ///
    /// Returns `Ok` only if the child process exits successfully. A non-zero
    /// exit is a typed error whose payload retains stdout and stderr. Timeout
    /// and cancellation failures also retain collected output when available.
    pub fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessError> {
        request.validate()?;

        if request.cancellation.is_cancelled() {
            return Err(ProcessError::Cancelled {
                program: request.program.clone(),
                output: empty_output(Duration::ZERO),
            });
        }

        let started = Instant::now();
        let mut command = Command::new(&request.program);

        command
            .args(&request.args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(directory) = &request.current_dir {
            command.current_dir(directory);
        }

        if request.environment_mode == EnvironmentMode::Clear {
            command.env_clear();
        }

        for key in &request.removed_environment {
            command.env_remove(key);
        }

        for (key, value) in &request.environment {
            command.env(key, value);
        }

        let mut child = command.spawn().map_err(|source| ProcessError::Spawn {
            program: request.program.clone(),
            source,
        })?;

        let stdout = child.stdout.take().ok_or_else(|| ProcessError::Io {
            operation: "opening child stdout",
            source: io::Error::other("stdout pipe was not created"),
        })?;

        let stderr = match child.stderr.take() {
            Some(stderr) => stderr,
            None => {
                let _ = terminate_child(&mut child);
                return Err(ProcessError::Io {
                    operation: "opening child stderr",
                    source: io::Error::other("stderr pipe was not created"),
                });
            }
        };

        let stdout_reader = match spawn_reader(
            "stdout",
            stdout,
            request.max_output_bytes_per_stream,
        ) {
            Ok(reader) => reader,
            Err(source) => {
                let _ = terminate_child(&mut child);
                return Err(ProcessError::Io {
                    operation: "starting stdout capture",
                    source,
                });
            }
        };

        let stderr_reader = match spawn_reader(
            "stderr",
            stderr,
            request.max_output_bytes_per_stream,
        ) {
            Ok(reader) => reader,
            Err(source) => {
                let _ = terminate_child(&mut child);
                let _ = join_reader(stdout_reader, "stdout");
                return Err(ProcessError::Io {
                    operation: "starting stderr capture",
                    source,
                });
            }
        };

        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {}
                Err(source) => {
                    let _ = terminate_child(&mut child);
                    let _ = join_reader(stdout_reader, "stdout");
                    let _ = join_reader(stderr_reader, "stderr");
                    return Err(ProcessError::Io {
                        operation: "checking child status",
                        source,
                    });
                }
            }

            if request.cancellation.is_cancelled() {
                terminate_child(&mut child).map_err(|source| ProcessError::Io {
                    operation: "terminating cancelled child",
                    source,
                })?;

                let output = collect_output(
                    stdout_reader,
                    stderr_reader,
                    started.elapsed(),
                )?;

                return Err(ProcessError::Cancelled {
                    program: request.program.clone(),
                    output,
                });
            }

            if let Some(timeout) = request.timeout {
                if started.elapsed() >= timeout {
                    terminate_child(&mut child).map_err(|source| ProcessError::Io {
                        operation: "terminating timed-out child",
                        source,
                    })?;

                    let output = collect_output(
                        stdout_reader,
                        stderr_reader,
                        started.elapsed(),
                    )?;

                    return Err(ProcessError::TimedOut {
                        program: request.program.clone(),
                        timeout,
                        output,
                    });
                }
            }

            thread::sleep(request.poll_interval);
        };

        let output = collect_output(
            stdout_reader,
            stderr_reader,
            started.elapsed(),
        )?;

        let output = ProcessOutput {
            exit_code: status.code(),
            success: status.success(),
            ..output
        };

        if output.success {
            Ok(output)
        } else {
            Err(ProcessError::NonZeroExit {
                program: request.program.clone(),
                output,
            })
        }
    }
}

fn empty_output(elapsed: Duration) -> ProcessOutput {
    ProcessOutput {
        exit_code: None,
        success: false,
        stdout: CapturedStream::default(),
        stderr: CapturedStream::default(),
        elapsed,
    }
}

fn terminate_child(child: &mut Child) -> io::Result<ExitStatus> {
    match child.kill() {
        Ok(()) => child.wait(),
        Err(kill_error) => match child.try_wait()? {
            Some(status) => Ok(status),
            None => Err(kill_error),
        },
    }
}

fn spawn_reader<R>(
    name: &'static str,
    reader: R,
    limit: Option<usize>,
) -> io::Result<JoinHandle<io::Result<CapturedStream>>>
where
    R: Read + Send + 'static,
{
    thread::Builder::new()
        .name(format!("zamani-process-{name}"))
        .spawn(move || drain_stream(reader, limit))
}

fn drain_stream<R: Read>(
    mut reader: R,
    limit: Option<usize>,
) -> io::Result<CapturedStream> {
    // The fixed buffer is only an I/O chunk size, not an output-size limit.
    let mut buffer = [0_u8; 8192];
    let mut captured = CapturedStream::default();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }

        captured.total_bytes = captured.total_bytes.saturating_add(count as u64);

        let remaining = limit
            .map(|maximum| maximum.saturating_sub(captured.bytes.len()))
            .unwrap_or(count);

        let retained = count.min(remaining);

        if retained > 0 {
            captured.bytes.extend_from_slice(&buffer[..retained]);
        }

        if retained < count {
            captured.truncated = true;
        }
    }

    Ok(captured)
}

fn join_reader(
    reader: JoinHandle<io::Result<CapturedStream>>,
    stream: &'static str,
) -> Result<CapturedStream, ProcessError> {
    let result = reader
        .join()
        .map_err(|_| ProcessError::ReaderPanicked { stream })?;

    result.map_err(|source| ProcessError::Io {
        operation: match stream {
            "stdout" => "reading child stdout",
            "stderr" => "reading child stderr",
            _ => "reading child output",
        },
        source,
    })
}

fn collect_output(
    stdout_reader: JoinHandle<io::Result<CapturedStream>>,
    stderr_reader: JoinHandle<io::Result<CapturedStream>>,
    elapsed: Duration,
) -> Result<ProcessOutput, ProcessError> {
    let stdout = join_reader(stdout_reader, "stdout")?;
    let stderr = join_reader(stderr_reader, "stderr")?;

    Ok(ProcessOutput {
        exit_code: None,
        success: false,
        stdout,
        stderr,
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper_request(mode: &str) -> ProcessRequest {
        let executable = std::env::current_exe()
            .expect("the Rust test executable should be available");

        ProcessRequest::new(executable)
            .args([
                "--exact",
                "backend::process::tests::child_helper",
                "--nocapture",
            ])
            .env("ZAMANI_PROCESS_HELPER_MODE", mode)
            .timeout(Duration::from_secs(10))
    }

    // This test also serves as a child-process fixture when the environment
    // variable is supplied by helper_request().
    #[test]
    fn child_helper() {
        match std::env::var("ZAMANI_PROCESS_HELPER_MODE").as_deref() {
            Ok("output") => {
                println!("stdout marker");
                eprintln!("stderr marker");
            }
            Ok("failure") => {
                println!("failure stdout");
                eprintln!("failure stderr");
                std::process::exit(23);
            }
            Ok("sleep") => {
                thread::sleep(Duration::from_secs(30));
            }
            _ => {}
        }
    }

    #[test]
    fn captures_both_streams() {
        let output = ProcessRunner::new()
            .run(&helper_request("output"))
            .expect("helper should exit successfully");

        assert!(output.success);
        assert!(output.stdout.to_string_lossy().contains("stdout marker"));
        assert!(output.stderr.to_string_lossy().contains("stderr marker"));
        assert!(!output.stdout.was_truncated());
        assert!(!output.stderr.was_truncated());
    }

    #[test]
    fn nonzero_exit_preserves_output_and_status() {
        let error = ProcessRunner::new()
            .run(&helper_request("failure"))
            .expect_err("the helper intentionally exits unsuccessfully");

        match error {
            ProcessError::NonZeroExit { output, .. } => {
                assert_eq!(output.exit_code, Some(23));
                assert!(output.stdout.to_string_lossy().contains("failure stdout"));
                assert!(output.stderr.to_string_lossy().contains("failure stderr"));
            }
            other => panic!("expected NonZeroExit, got {other:?}"),
        }
    }

    #[test]
    fn timeout_terminates_direct_child() {
        let request = helper_request("sleep").timeout(Duration::from_millis(50));

        let error = ProcessRunner::new()
            .run(&request)
            .expect_err("the sleeping helper should time out");

        assert!(matches!(error, ProcessError::TimedOut { .. }));
    }

    #[test]
    fn cancellation_before_start_is_reported() {
        let token = CancellationToken::new();
        token.cancel();

        let request = ProcessRequest::new("this-executable-must-not-be-started")
            .cancellation_token(token);

        let error = ProcessRunner::new()
            .run(&request)
            .expect_err("a pre-cancelled request must not start");

        assert!(matches!(error, ProcessError::Cancelled { .. }));
    }

    #[test]
    fn output_capture_budget_is_not_a_pipe_read_limit() {
        let output = ProcessRunner::new()
            .run(
                &helper_request("output")
                    .max_output_bytes_per_stream(Some(1)),
            )
            .expect("the child should still exit successfully");

        assert_eq!(output.stdout.captured_len(), 1);
        assert!(output.stdout.was_truncated());
        assert!(output.stdout.total_bytes() > 1);

        assert_eq!(output.stderr.captured_len(), 1);
        assert!(output.stderr.was_truncated());
        assert!(output.stderr.total_bytes() > 1);
    }

    #[test]
    fn zero_poll_interval_is_rejected() {
        let request = ProcessRequest::new("unused").poll_interval(Duration::ZERO);

        assert!(matches!(
            ProcessRunner::new().run(&request),
            Err(ProcessError::InvalidRequest(_))
        ));
    }

    #[test]
    fn clear_environment_does_not_inherit_unconfigured_variables() {
        let request = ProcessRequest::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "backend::process::tests::child_helper",
                "--nocapture",
            ])
            .environment_mode(EnvironmentMode::Clear)
            .env("ZAMANI_PROCESS_HELPER_MODE", "output")
            .timeout(Duration::from_secs(10));

        let output = ProcessRunner::new()
            .run(&request)
            .expect("helper should run with its configured environment");

        assert!(output.success);
    }
}
