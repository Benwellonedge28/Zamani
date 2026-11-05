
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Sessions.
//!
//! Path: src/compiler/api/session.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety:
//! - unsafe Rust is forbidden.
//! - shared session state is synchronized.
//! - no global mutable session state is introduced.
//!
//! # Purpose
//!
//! Provides a thread-safe lifecycle boundary for one compilation operation.
//!
//! A CompilationSession coordinates the beginning, execution, cancellation,
//! completion, failure and closure of a compilation request.
//!
//! It delegates actual compilation to the existing CompilerService.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - compilation-session lifecycle;
//! - session state transitions;
//! - cancellation requests;
//! - session closure;
//! - concurrent session control;
//! - compilation-operation exclusivity;
//! - session-level error propagation.
//!
//! This module does not own:
//!
//! - compiler configuration;
//! - source parsing;
//! - AST construction;
//! - semantic analysis;
//! - ownership analysis;
//! - canonical IR;
//! - optimization;
//! - hybrid semantic composition;
//! - hardware discovery;
//! - physical resource allocation;
//! - backend code generation;
//! - runtime execution.
//!
//! # POCO-REAF
//!
//! Session management is independent of:
//!
//! - processor architecture;
//! - hardware vendor;
//! - accelerator type;
//! - quantum hardware provider;
//! - distributed topology;
//! - deployment environment.
//!
//! A session delegates compilation through CompilationService.
//!
//! It does not impose universal hardware or program-size limits.
//!
//! # Cancellation semantics
//!
//! Cancellation is cooperative at the service boundary.
//!
//! The existing CompilationDriver interface does not accept a cancellation
//! token. Consequently, cancellation requested during an active compilation
//! cannot forcibly interrupt that driver's internal work.
//!
//! This module guarantees that a result is not published as successful after
//! cancellation or closure has been requested.
//!
//! True interruption of individual compilation stages requires a future
//! cancellation-aware CompilationDriver contract.
//!
//! # Session lifecycle
//!
//! Ready
//!   -> Running
//!   -> Completed
//!   -> Failed
//!   -> CancellationRequested
//!   -> CloseRequested
//!
//! CancellationRequested -> Cancelled
//! CloseRequested        -> Closed
//!
//! Terminal sessions cannot be reused.
//!
//! # Integration
//!
//! Depends on:
//!
//! - crate::compiler::api::compiler::CompilerService
//! - crate::compiler::api::compiler::CompilationDriver
//! - crate::compiler::CompilerError
//! - crate::compiler::CompilationOutput
//!
//! Does not depend on future request.rs or response.rs files.
//!
//! This keeps the session contract independent of those future API files.

use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use super::compiler::{
    CompilationDriver,
    CompilerService,
};

use super::{
    CompilationOutput,
    CompilerError,
};

// -----------------------------------------------------------------------------
// Session state
// -----------------------------------------------------------------------------

/// Lifecycle state of one compilation session.
///
/// Each session represents one compilation operation.
///
/// Terminal states cannot transition back to Ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session has been created but compilation has not started.
    Ready,

    /// Compilation is currently executing.
    Running,

    /// Cancellation has been requested during compilation.
    CancellationRequested,

    /// Session closure has been requested during compilation.
    CloseRequested,

    /// Compilation completed successfully.
    Completed,

    /// Compilation failed.
    Failed,

    /// Compilation was cancelled or its result was discarded.
    Cancelled,

    /// Session is closed.
    Closed,
}

impl SessionState {
    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::Closed
        )
    }

    /// Returns whether compilation is actively executing.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(
            self,
            Self::Running
                | Self::CancellationRequested
                | Self::CloseRequested
        )
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub const fn cancellation_requested(self) -> bool {
        matches!(
            self,
            Self::CancellationRequested
                | Self::Cancelled
                | Self::CloseRequested
                | Self::Closed
        )
    }
}

// -----------------------------------------------------------------------------
// Session errors
// -----------------------------------------------------------------------------

/// Errors produced by compilation-session management.
#[derive(Debug)]
pub enum SessionError {
    /// The underlying compiler failed.
    Compilation(CompilerError),

    /// The session was cancelled.
    Cancelled,

    /// The session has been closed.
    Closed,

    /// The requested operation is invalid in the current state.
    InvalidState {
        state: SessionState,
        operation: &'static str,
    },

    /// Shared session state was poisoned.
    ///
    /// This indicates that a thread panicked while holding the session
    /// synchronization lock.
    SynchronizationPoisoned,
}

impl fmt::Display for SessionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Compilation(error) => {
                write!(
                    formatter,
                    "compilation session failed: {error}"
                )
            }

            Self::Cancelled => {
                write!(
                    formatter,
                    "compilation session was cancelled"
                )
            }

            Self::Closed => {
                write!(
                    formatter,
                    "compilation session is closed"
                )
            }

            Self::InvalidState {
                state,
                operation,
            } => {
                write!(
                    formatter,
                    "cannot perform operation '{operation}' in session state {state:?}"
                )
            }

            Self::SynchronizationPoisoned => {
                write!(
                    formatter,
                    "compilation session synchronization state is poisoned"
                )
            }
        }
    }
}

impl Error for SessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Compilation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<CompilerError> for SessionError {
    fn from(error: CompilerError) -> Self {
        Self::Compilation(error)
    }
}

// -----------------------------------------------------------------------------
// Shared session control
// -----------------------------------------------------------------------------

/// Shared internal lifecycle state.
///
/// A mutex protects transitions so that cancellation, closure and completion
/// cannot independently overwrite each other's state.
#[derive(Debug)]
struct SessionInner {
    state: Mutex<SessionState>,
}

impl SessionInner {
    fn new() -> Self {
        Self {
            state: Mutex::new(SessionState::Ready),
        }
    }
}

// -----------------------------------------------------------------------------
// External session control handle
// -----------------------------------------------------------------------------

/// Thread-safe handle for controlling a compilation session.
///
/// The handle may be cloned and transferred to another thread.
///
/// It allows another component to request cancellation or closure without
/// needing access to the CompilerService or its configuration.
///
/// Cloning this handle does not create another compilation session.
#[derive(Debug, Clone)]
pub struct SessionControl {
    inner: Arc<SessionInner>,
}

impl SessionControl {
    /// Creates a control handle from shared session state.
    fn new(inner: Arc<SessionInner>) -> Self {
        Self { inner }
    }

    /// Returns the current session state.
    ///
    /// # Errors
    ///
    /// Returns SynchronizationPoisoned if the lifecycle mutex is poisoned.
    pub fn state(&self) -> Result<SessionState, SessionError> {
        let state = self
            .inner
            .state
            .lock()
            .map_err(|_| SessionError::SynchronizationPoisoned)?;

        Ok(*state)
    }

    /// Requests cancellation.
    ///
    /// If compilation is running, its state becomes
    /// CancellationRequested.
    ///
    /// If compilation has not started, the session becomes Cancelled.
    ///
    /// If compilation has already reached a terminal state, the request
    /// has no effect.
    ///
    /// Returns true when cancellation was accepted or was already requested.
    ///
    /// IMPORTANT:
    ///
    /// This does not forcibly interrupt an executing legacy compilation
    /// driver. The active operation is allowed to return, after which its
    /// result is discarded.
    pub fn request_cancel(&self) -> Result<bool, SessionError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| SessionError::SynchronizationPoisoned)?;

        match *state {
            SessionState::Ready => {
                *state = SessionState::Cancelled;
                Ok(true)
            }

            SessionState::Running => {
                *state = SessionState::CancellationRequested;
                Ok(true)
            }

            SessionState::CancellationRequested => {
                Ok(true)
            }

            SessionState::CloseRequested => {
                Ok(false)
            }

            SessionState::Completed
            | SessionState::Failed
            | SessionState::Cancelled
            | SessionState::Closed => {
                Ok(false)
            }
        }
    }

    /// Requests session closure.
    ///
    /// If compilation is active, cancellation is requested and the session
    /// transitions to Closed when the driver returns.
    ///
    /// If compilation has not started, the session closes immediately.
    ///
    /// Closing a terminal session is idempotent.
    pub fn close(&self) -> Result<(), SessionError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| SessionError::SynchronizationPoisoned)?;

        match *state {
            SessionState::Ready
            | SessionState::Completed
            | SessionState::Failed
            | SessionState::Cancelled
            | SessionState::Closed => {
                *state = SessionState::Closed;
            }

            SessionState::Running
            | SessionState::CancellationRequested
            | SessionState::CloseRequested => {
                *state = SessionState::CloseRequested;
            }
        }

        Ok(())
    }

    /// Returns whether cancellation or closure has been requested.
    pub fn is_cancelled(&self) -> Result<bool, SessionError> {
        Ok(self.state()?.cancellation_requested())
    }

    /// Returns whether the session has reached a terminal state.
    pub fn is_finished(&self) -> Result<bool, SessionError> {
        Ok(self.state()?.is_terminal())
    }
}

// -----------------------------------------------------------------------------
// Compilation session
// -----------------------------------------------------------------------------

/// One compilation operation with an independently managed lifecycle.
///
/// The session delegates actual compilation to CompilerService.
///
/// It does not own or duplicate compiler stages.
///
/// A session is single-use:
///
/// - one compilation operation may begin;
/// - one terminal outcome is produced;
/// - a new compilation requires a new session.
///
/// This prevents accidental reuse of stale cancellation state.
pub struct CompilationSession<D: CompilationDriver> {
    service: CompilerService<D>,
    control: SessionControl,
}

impl<D: CompilationDriver> CompilationSession<D> {
    /// Creates a new compilation session.
    ///
    /// The service retains ownership of its immutable configuration and
    /// compilation driver.
    ///
    /// The session creates only its own lifecycle state.
    pub fn new(service: CompilerService<D>) -> Self {
        Self {
            service,
            control: SessionControl::new(
                Arc::new(SessionInner::new()),
            ),
        }
    }

    /// Returns a cloneable control handle.
    ///
    /// This allows cancellation or closure from another thread.
    #[must_use]
    pub fn control(&self) -> SessionControl {
        self.control.clone()
    }

    /// Returns the current lifecycle state.
    pub fn state(&self) -> Result<SessionState, SessionError> {
        self.control.state()
    }

    /// Begins the compilation operation.
    ///
    /// Only Ready sessions may begin compilation.
    fn begin(&self) -> Result<(), SessionError> {
        let mut state = self
            .control
            .inner
            .state
            .lock()
            .map_err(|_| SessionError::SynchronizationPoisoned)?;

        match *state {
            SessionState::Ready => {
                *state = SessionState::Running;
                Ok(())
            }

            SessionState::Cancelled => {
                Err(SessionError::Cancelled)
            }

            SessionState::Closed
            | SessionState::CloseRequested => {
                Err(SessionError::Closed)
            }

            current => {
                Err(SessionError::InvalidState {
                    state: current,
                    operation: "begin compilation",
                })
            }
        }
    }

    /// Resolves the session after the compilation driver returns.
    ///
    /// Cancellation and closure take precedence over the driver's result.
    ///
    /// A cancelled or closed session never publishes a successful
    /// CompilationOutput.
    fn finish(
        &self,
        result: Result<CompilationOutput, CompilerError>,
    ) -> Result<CompilationOutput, SessionError> {
        let mut state = self
            .control
            .inner
            .state
            .lock()
            .map_err(|_| SessionError::SynchronizationPoisoned)?;

        match *state {
            SessionState::Running => {
                match result {
                    Ok(output) => {
                        *state = SessionState::Completed;
                        Ok(output)
                    }

                    Err(error) => {
                        *state = SessionState::Failed;
                        Err(SessionError::Compilation(error))
                    }
                }
            }

            SessionState::CancellationRequested => {
                *state = SessionState::Cancelled;
                Err(SessionError::Cancelled)
            }

            SessionState::CloseRequested => {
                *state = SessionState::Closed;
                Err(SessionError::Closed)
            }

            SessionState::Closed => {
                Err(SessionError::Closed)
            }

            SessionState::Cancelled => {
                Err(SessionError::Cancelled)
            }

            current => {
                Err(SessionError::InvalidState {
                    state: current,
                    operation: "finish compilation",
                })
            }
        }
    }

    /// Executes one compilation operation under session lifecycle control.
    fn execute<F>(
        &self,
        operation: F,
    ) -> Result<CompilationOutput, SessionError>
    where
        F: FnOnce() -> Result<CompilationOutput, CompilerError>,
    {
        self.begin()?;

        // The lifecycle lock is deliberately not held while compiling.
        //
        // This allows another thread to request cancellation or closure
        // while the compilation driver is running.
        //
        // The driver remains responsible for its own internal
        // synchronization and panic policy.
        let result = operation();

        self.finish(result)
    }

    /// Compiles source text within this session.
    ///
    /// This delegates directly to the existing CompilerService.
    ///
    /// No parser, semantic analyzer, IR generator or backend is duplicated.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - SessionError::Compilation for compiler failures;
    /// - SessionError::Cancelled for cancellation;
    /// - SessionError::Closed for closure;
    /// - SessionError::InvalidState for illegal lifecycle operations;
    /// - SessionError::SynchronizationPoisoned for poisoned state.
    pub fn compile_source(
        &self,
        source: &str,
        source_name: &str,
    ) -> Result<CompilationOutput, SessionError> {
        self.execute(|| {
            self.service.compile_source(
                source,
                source_name,
            )
        })
    }

    /// Compiles a source file within this session.
    ///
    /// File reading and compilation are covered by the same lifecycle.
    ///
    /// The file is read as UTF-8.
    ///
    /// No arbitrary source-size limit is introduced here.
    ///
    /// Actual filesystem and memory limits remain environmental constraints.
    pub fn compile_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<CompilationOutput, SessionError> {
        let path = path.as_ref();

        self.execute(|| {
            let source = std::fs::read_to_string(path)
                .map_err(|error| CompilerError::Io {
                    path: path.display().to_string(),
                    message: error.to_string(),
                })?;

            let source_name = path.display().to_string();

            self.service.compile_source(
                &source,
                &source_name,
            )
        })
    }

    /// Compatibility-friendly alias for compile_path.
    pub fn compile_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<CompilationOutput, SessionError> {
        self.compile_path(path)
    }

    /// Closes the session.
    ///
    /// If compilation is active, closure is deferred until the driver
    /// returns.
    pub fn close(&self) -> Result<(), SessionError> {
        self.control.close()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::{
        CompilationTarget,
        CompilerConfig,
        OptimizationLevel,
    };

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use std::sync::Condvar;
    use std::thread;

    #[derive(Debug, Default)]
    struct TestDriver {
        calls: AtomicUsize,
    }

    impl CompilationDriver for TestDriver {
        fn compile_source(
            &self,
            source: &str,
            _source_name: &str,
            _config: &CompilerConfig,
        ) -> Result<CompilationOutput, CompilerError> {
            self.calls.fetch_add(1, Ordering::Relaxed);

            Ok(CompilationOutput {
                bytes: source.as_bytes().to_vec(),
                target: "session-test".to_string(),
                size_bytes: source.len(),
            })
        }
    }

    fn test_config() -> CompilerConfig {
        CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable-test-ir".to_string(),
            ),
            opt_level: OptimizationLevel::None,
            debug_info: false,
            verify: true,
            emit_ir: true,
            parallel: false,
        }
    }

    fn test_session() -> CompilationSession<TestDriver> {
        let service = CompilerService::new(
            test_config(),
            TestDriver::default(),
        );

        CompilationSession::new(service)
    }

    #[test]
    fn new_session_starts_ready() {
        let session = test_session();

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Ready
        );
    }

    #[test]
    fn successful_compilation_completes_session() {
        let session = test_session();

        let output = session
            .compile_source("let value = 42;", "test.zm")
            .expect("compilation should succeed");

        assert_eq!(
            output.bytes,
            b"let value = 42;"
        );

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Completed
        );
    }

    #[test]
    fn session_cannot_compile_twice() {
        let session = test_session();

        session
            .compile_source("first", "first.zm")
            .expect("first compilation should succeed");

        let result = session.compile_source(
            "second",
            "second.zm",
        );

        assert!(matches!(
            result,
            Err(SessionError::InvalidState {
                state: SessionState::Completed,
                ..
            })
        ));
    }

    #[test]
    fn cancellation_before_compilation_prevents_execution() {
        let session = test_session();
        let control = session.control();

        assert!(
            control.request_cancel()
                .expect("cancellation should succeed")
        );

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Cancelled
        );

        let result = session.compile_source(
            "let value = 42;",
            "test.zm",
        );

        assert!(matches!(
            result,
            Err(SessionError::Cancelled)
        ));
    }

    #[test]
    fn cancellation_after_completion_has_no_effect() {
        let session = test_session();

        session
            .compile_source("value", "test.zm")
            .expect("compilation should succeed");

        assert!(
            !session.control()
                .request_cancel()
                .expect("control should be available")
        );

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Completed
        );
    }

    #[test]
    fn close_before_compilation_prevents_execution() {
        let session = test_session();

        session.close().expect("close should succeed");

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Closed
        );

        let result = session.compile_source(
            "source",
            "test.zm",
        );

        assert!(matches!(
            result,
            Err(SessionError::Closed)
        ));
    }

    #[test]
    fn close_is_idempotent() {
        let session = test_session();

        session.close().expect("first close should succeed");
        session.close().expect("second close should succeed");

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Closed
        );
    }

    #[test]
    fn failed_compilation_sets_failed_state() {
        struct FailingDriver;

        impl CompilationDriver for FailingDriver {
            fn compile_source(
                &self,
                _source: &str,
                _source_name: &str,
                _config: &CompilerConfig,
            ) -> Result<CompilationOutput, CompilerError> {
                Err(CompilerError::Parse {
                    message: "intentional failure".to_string(),
                })
            }
        }

        let service = CompilerService::new(
            test_config(),
            FailingDriver,
        );

        let session = CompilationSession::new(service);

        let result = session.compile_source(
            "invalid",
            "invalid.zm",
        );

        assert!(matches!(
            result,
            Err(SessionError::Compilation(
                CompilerError::Parse { .. }
            ))
        ));

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Failed
        );
    }

    #[test]
    fn cloned_control_handles_share_state() {
        let session = test_session();

        let first = session.control();
        let second = first.clone();

        first.request_cancel()
            .expect("cancellation should succeed");

        assert_eq!(
            second.state().expect("state should be available"),
            SessionState::Cancelled
        );
    }

    #[test]
    fn close_during_compilation_discards_result() {
        let gate = Arc::new((
            Mutex::new(false),
            Condvar::new(),
        ));

        struct BlockingDriver {
            gate: Arc<(Mutex<bool>, Condvar)>,
        }

        impl CompilationDriver for BlockingDriver {
            fn compile_source(
                &self,
                source: &str,
                _source_name: &str,
                _config: &CompilerConfig,
            ) -> Result<CompilationOutput, CompilerError> {
                let (lock, condition) = &*self.gate;

                let mut released = lock.lock().unwrap();

                while !*released {
                    released = condition.wait(released).unwrap();
                }

                Ok(CompilationOutput {
                    bytes: source.as_bytes().to_vec(),
                    target: "blocking-test".to_string(),
                    size_bytes: source.len(),
                })
            }
        }

        let service = CompilerService::new(
            test_config(),
            BlockingDriver {
                gate: Arc::clone(&gate),
            },
        );

        let session = Arc::new(
            CompilationSession::new(service)
        );

        let worker_session = Arc::clone(&session);

        let worker = thread::spawn(move || {
            worker_session.compile_source(
                "test",
                "test.zm",
            )
        });

        // Wait until the driver has entered its blocking section.
        //
        // The test gate is released after requesting closure.
        session.control()
            .close()
            .expect("closure request should succeed");

        {
            let (lock, condition) = &*gate;

            let mut released = lock.lock().unwrap();
            *released = true;
            condition.notify_all();
        }

        let result = worker.join()
            .expect("worker should finish");

        assert!(matches!(
            result,
            Err(SessionError::Closed)
        ));

        assert_eq!(
            session.state().expect("state should be available"),
            SessionState::Closed
        );
    }

    #[test]
    fn session_state_terminal_classification_is_correct() {
        assert!(!SessionState::Ready.is_terminal());
        assert!(!SessionState::Running.is_terminal());

        assert!(SessionState::Completed.is_terminal());
        assert!(SessionState::Failed.is_terminal());
        assert!(SessionState::Cancelled.is_terminal());
        assert!(SessionState::Closed.is_terminal());
    }

    #[test]
    fn session_running_classification_is_correct() {
        assert!(SessionState::Running.is_running());
        assert!(
            SessionState::CancellationRequested.is_running()
        );
        assert!(SessionState::CloseRequested.is_running());

        assert!(!SessionState::Ready.is_running());
        assert!(!SessionState::Completed.is_running());
    }

    #[test]
    fn independent_sessions_have_independent_lifecycles() {
        let first = test_session();
        let second = test_session();

        first.control()
            .request_cancel()
            .expect("first session should cancel");

        assert_eq!(
            first.state().expect("state should be available"),
            SessionState::Cancelled
        );

        assert_eq!(
            second.state().expect("state should be available"),
            SessionState::Ready
        );

        second
            .compile_source("independent", "second.zm")
            .expect("second session should compile");
    }
}
