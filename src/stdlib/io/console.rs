
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Console I/O
//!
//! Path: `src/stdlib/io/console.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides portable, provider-independent console interfaces for
//! standard input, standard output and standard error.
//!
//! The implementation supports:
//!
//! - Real host standard input.
//! - Real host standard output.
//! - Real host standard error.
//! - UTF-8 line-oriented input.
//! - Explicit end-of-input reporting.
//! - Text output.
//! - Line-oriented output.
//! - Explicit flushing.
//! - Cooperative cancellation.
//! - Structured error propagation.
//! - Injectable console providers.
//! - Concurrent sharing of console handles.
//! - In-memory testing providers.
//!
//! ## Architectural ownership
//!
//! This module owns console API contracts and the host standard-I/O
//! adapter.
//!
//! It does not own:
//!
//! - Filesystem operations.
//! - Network transports.
//! - Device access.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Process management.
//! - Distributed execution.
//! - Compiler semantics.
//! - I/O effect interpretation.
//! - Byte-stream infrastructure.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ## Integration
//!
//! Structured errors:
//! `crate::stdlib::error`
//!
//! Operation context:
//! `crate::stdlib::context`
//!
//! Byte streams:
//! `crate::stdlib::io::streams`
//!
//! I/O semantic grammar:
//! `grammar/effects/io.g4`
//!
//! Runtime and host integration:
//! `crate::runtime`
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Input line length.
//! - Output text length.
//! - Number of console operations.
//! - Number of console instances.
//! - Number of providers.
//!
//! Input is read incrementally through the host's buffered input API.
//!
//! Output is transferred through the selected provider.
//!
//! Practical limits depend on available memory, host facilities,
//! provider capabilities and explicitly configured operational policies.
//!
//! ## Cancellation
//!
//! Cancellation is cooperative.
//!
//! Every operation checks cancellation before starting.
//!
//! Cancellation cannot forcibly interrupt an already-blocking operating
//! system standard-input operation.
//!
//! Providers requiring interruptible input must implement that behavior
//! through their own execution and cancellation facilities.
//!
//! ## Error guarantees
//!
//! EOF is represented by `Ok(None)`.
//!
//! An empty input line is represented by `Ok(Some(String::new()))`
//! only when the provider removes the line terminator.
//!
//! The default host provider preserves the line terminator.
//!
//! A failed output operation is never reported as successful.
//!
//! Successful output means the provider accepted the complete text.
//!
//! Flushing is separate from writing.
//!
//! Flushing does not claim durable persistence.
//!
//! ## Safety
//!
//! This module uses safe Rust only.

use std::fmt;
use std::io::{self, BufRead, Write};
use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// IDENTIFICATION
// ============================================================================

/// Stable standard-library component identifier.
pub const COMPONENT: &str = "stdlib.io.console";

/// Standard input operation identifier.
pub const READ_LINE_OPERATION: &str = "io.console.read_line";

/// Standard output operation identifier.
pub const WRITE_STDOUT_OPERATION: &str = "io.console.write_stdout";

/// Standard error operation identifier.
pub const WRITE_STDERR_OPERATION: &str = "io.console.write_stderr";

/// Standard output flush operation identifier.
pub const FLUSH_STDOUT_OPERATION: &str = "io.console.flush_stdout";

/// Standard error flush operation identifier.
pub const FLUSH_STDERR_OPERATION: &str = "io.console.flush_stderr";

// ============================================================================
// ERROR CONVERSION
// ============================================================================

/// Converts an operating-system I/O error into a Zamani error.
///
/// The original operating-system error is preserved as the causal source.
fn convert_io_error(
    error: io::Error,
    operation: &'static str,
) -> Error {
    let kind = match error.kind() {
        io::ErrorKind::PermissionDenied => {
            ErrorKind::PermissionDenied
        }

        io::ErrorKind::NotFound => {
            ErrorKind::NotFound
        }

        io::ErrorKind::AlreadyExists => {
            ErrorKind::AlreadyExists
        }

        io::ErrorKind::InvalidInput => {
            ErrorKind::InvalidInput
        }

        io::ErrorKind::InvalidData => {
            ErrorKind::InvalidData
        }

        io::ErrorKind::TimedOut => {
            ErrorKind::TimedOut
        }

        io::ErrorKind::WouldBlock => {
            ErrorKind::ResourceUnavailable
        }

        io::ErrorKind::OutOfMemory => {
            ErrorKind::ResourceUnavailable
        }

        io::ErrorKind::Interrupted => {
            ErrorKind::Io
        }

        _ => {
            ErrorKind::Io
        }
    };

    Error::new(
        kind,
        error.to_string(),
    )
    .in_component(COMPONENT)
    .during_operation(operation)
    .with_source(error)
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Provider contract for console input and output.
///
/// Implementations may connect to:
///
/// - Host standard streams.
/// - Embedded console interfaces.
/// - Virtual terminals.
/// - Remote interactive sessions.
/// - Container-provided consoles.
/// - Simulated consoles.
/// - Future execution environments.
///
/// A provider must not report successful output until the complete
/// supplied text has been accepted according to its documented
/// output contract.
///
/// Implementations must check the supplied operation context
/// before beginning an operation.
///
/// A provider may implement stronger cancellation guarantees
/// than the default host implementation.
///
/// This trait deliberately does not select hardware, operating
/// systems, terminals or physical devices.
pub trait ConsoleBackend: Send + Sync {
    /// Reads one UTF-8 line from the provider's standard input.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(line))` when a line is received.
    /// - `Ok(None)` when the input source reaches EOF.
    ///
    /// Implementations should preserve line terminators consistently.
    ///
    /// The line length is not subject to an arbitrary library limit.
    ///
    /// # Errors
    ///
    /// Returns a structured error when input fails or cancellation
    /// is requested.
    fn read_line(
        &self,
        context: &OperationContext,
    ) -> Result<Option<String>>;

    /// Writes complete UTF-8 text to standard output.
    ///
    /// A successful result means the provider accepted the complete
    /// supplied text.
    ///
    /// This does not independently guarantee that the text has been
    /// flushed or durably persisted.
    ///
    /// # Errors
    ///
    /// Returns a structured error when output fails or cancellation
    /// is requested.
    fn write_stdout(
        &self,
        text: &str,
        context: &OperationContext,
    ) -> Result<()>;

    /// Writes complete UTF-8 text to standard error.
    ///
    /// A successful result means the provider accepted the complete
    /// supplied text.
    ///
    /// # Errors
    ///
    /// Returns a structured error when output fails or cancellation
    /// is requested.
    fn write_stderr(
        &self,
        text: &str,
        context: &OperationContext,
    ) -> Result<()>;

    /// Flushes standard output.
    ///
    /// Flushing is permitted even when cancellation has been requested.
    ///
    /// This permits previously accepted output to be flushed during
    /// orderly cancellation and shutdown.
    ///
    /// # Errors
    ///
    /// Returns a structured error if flushing fails.
    fn flush_stdout(&self) -> Result<()>;

    /// Flushes standard error.
    ///
    /// Flushing is permitted even when cancellation has been requested.
    ///
    /// # Errors
    ///
    /// Returns a structured error if flushing fails.
    fn flush_stderr(&self) -> Result<()>;
}

// ============================================================================
// HOST STANDARD-IO PROVIDER
// ============================================================================

/// Host operating-system standard-I/O provider.
///
/// This provider uses Rust's real standard input, output and error
/// facilities.
///
/// It does not fabricate input or output.
///
/// It does not claim to implement durable storage.
///
/// It does not assume a particular operating system.
#[derive(Debug, Default, Clone, Copy)]
pub struct HostConsoleBackend;

impl HostConsoleBackend {
    /// Creates a host standard-I/O provider.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ConsoleBackend for HostConsoleBackend {
    fn read_line(
        &self,
        context: &OperationContext,
    ) -> Result<Option<String>> {
        context.check_cancelled()?;

        let stdin = io::stdin();

        let mut input = String::new();

        let amount = {
            let mut locked_input = stdin.lock();

            locked_input
                .read_line(&mut input)
                .map_err(|error| {
                    convert_io_error(
                        error,
                        READ_LINE_OPERATION,
                    )
                })?
        };

        if amount == 0 {
            return Ok(None);
        }

        Ok(Some(input))
    }

    fn write_stdout(
        &self,
        text: &str,
        context: &OperationContext,
    ) -> Result<()> {
        context.check_cancelled()?;

        let stdout = io::stdout();

        let mut locked_output = stdout.lock();

        locked_output
            .write_all(text.as_bytes())
            .map_err(|error| {
                convert_io_error(
                    error,
                    WRITE_STDOUT_OPERATION,
                )
            })
    }

    fn write_stderr(
        &self,
        text: &str,
        context: &OperationContext,
    ) -> Result<()> {
        context.check_cancelled()?;

        let stderr = io::stderr();

        let mut locked_error = stderr.lock();

        locked_error
            .write_all(text.as_bytes())
            .map_err(|error| {
                convert_io_error(
                    error,
                    WRITE_STDERR_OPERATION,
                )
            })
    }

    fn flush_stdout(&self) -> Result<()> {
        let stdout = io::stdout();

        let mut locked_output = stdout.lock();

        locked_output
            .flush()
            .map_err(|error| {
                convert_io_error(
                    error,
                    FLUSH_STDOUT_OPERATION,
                )
            })
    }

    fn flush_stderr(&self) -> Result<()> {
        let stderr = io::stderr();

        let mut locked_error = stderr.lock();

        locked_error
            .flush()
            .map_err(|error| {
                convert_io_error(
                    error,
                    FLUSH_STDERR_OPERATION,
                )
            })
    }
}

// ============================================================================
// CONSOLE
// ============================================================================

/// Portable Zamani console interface.
///
/// The console combines:
///
/// - A provider.
/// - An immutable operation context.
///
/// Cloning a console shares its provider and context.
///
/// The console does not own process-wide initialization or shutdown.
///
/// The console does not own an independent thread or executor.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::io::console::Console;
///
/// let console = Console::new();
///
/// console.write_stdout("Zamani").unwrap();
/// console.write_line(" is portable.").unwrap();
/// console.flush_stdout().unwrap();
/// ```
#[derive(Clone)]
pub struct Console {
    backend: Arc<dyn ConsoleBackend>,
    context: OperationContext,
}

impl Console {
    /// Creates a console connected to real host standard streams.
    ///
    /// The default context does not impose an artificial operation
    /// budget or cancellation deadline.
    #[must_use]
    pub fn new() -> Self {
        Self::with_backend_and_context(
            HostConsoleBackend::new(),
            OperationContext::builder().build(),
        )
    }

    /// Creates a console using the supplied provider.
    ///
    /// A default operation context is used.
    #[must_use]
    pub fn with_backend<B>(backend: B) -> Self
    where
        B: ConsoleBackend + 'static,
    {
        Self::with_backend_and_context(
            backend,
            OperationContext::builder().build(),
        )
    }

    /// Creates a console with an explicit provider and context.
    #[must_use]
    pub fn with_backend_and_context<B>(
        backend: B,
        context: OperationContext,
    ) -> Self
    where
        B: ConsoleBackend + 'static,
    {
        Self {
            backend: Arc::new(backend),
            context,
        }
    }

    /// Creates a console using a shared provider.
    ///
    /// This constructor permits several consoles to share the same
    /// provider instance.
    #[must_use]
    pub fn with_shared_backend(
        backend: Arc<dyn ConsoleBackend>,
        context: OperationContext,
    ) -> Self {
        Self {
            backend,
            context,
        }
    }

    /// Returns the operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns whether the operation context is cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.context.is_cancelled()
    }

    /// Reads one line from standard input.
    ///
    /// The default host provider preserves the line terminator.
    ///
    /// For example, a line containing `Zamani` followed by a newline
    /// is returned as `Zamani\n`.
    ///
    /// EOF is represented by `Ok(None)`.
    ///
    /// An empty line is distinct from EOF.
    ///
    /// No arbitrary line-length limit is imposed.
    ///
    /// The underlying input operation may block. Cooperative
    /// cancellation cannot forcibly interrupt a blocking host read.
    ///
    /// # Errors
    ///
    /// Returns a structured error if input fails or cancellation
    /// is requested before reading.
    pub fn read_line(&self) -> Result<Option<String>> {
        self.context.check_cancelled()?;

        self.backend.read_line(&self.context)
    }

    /// Reads one line and removes its trailing line terminator.
    ///
    /// Both LF and CRLF terminators are recognized.
    ///
    /// Other characters, including internal whitespace, are preserved.
    ///
    /// EOF is represented by `Ok(None)`.
    pub fn read_line_without_terminator(
        &self,
    ) -> Result<Option<String>> {
        let Some(mut line) = self.read_line()? else {
            return Ok(None);
        };

        if line.ends_with('\n') {
            line.pop();

            if line.ends_with('\r') {
                line.pop();
            }
        } else if line.ends_with('\r') {
            line.pop();
        }

        Ok(Some(line))
    }

    /// Writes text to standard output.
    ///
    /// The provider must accept the complete supplied text or
    /// return an error.
    ///
    /// Writing does not implicitly flush the output.
    pub fn write_stdout(
        &self,
        text: &str,
    ) -> Result<()> {
        self.context.check_cancelled()?;

        self.backend.write_stdout(
            text,
            &self.context,
        )
    }

    /// Writes text followed by a newline to standard output.
    ///
    /// The operation performs two ordered provider writes.
    ///
    /// If the first write succeeds but the second fails, the
    /// first write is not rolled back.
    ///
    /// The output is not implicitly flushed.
    pub fn write_line(
        &self,
        text: &str,
    ) -> Result<()> {
        self.write_stdout(text)?;

        self.write_stdout("\n")
    }

    /// Writes text to standard error.
    ///
    /// The output is not implicitly flushed.
    pub fn write_stderr(
        &self,
        text: &str,
    ) -> Result<()> {
        self.context.check_cancelled()?;

        self.backend.write_stderr(
            text,
            &self.context,
        )
    }

    /// Writes text followed by a newline to standard error.
    ///
    /// The output is not implicitly flushed.
    pub fn write_error_line(
        &self,
        text: &str,
    ) -> Result<()> {
        self.write_stderr(text)?;

        self.write_stderr("\n")
    }

    /// Flushes standard output.
    ///
    /// Flushing is deliberately permitted after cancellation.
    pub fn flush_stdout(&self) -> Result<()> {
        self.backend.flush_stdout()
    }

    /// Flushes standard error.
    ///
    /// Flushing is deliberately permitted after cancellation.
    pub fn flush_stderr(&self) -> Result<()> {
        self.backend.flush_stderr()
    }

    /// Writes text and then flushes standard output.
    pub fn write_stdout_and_flush(
        &self,
        text: &str,
    ) -> Result<()> {
        self.write_stdout(text)?;

        self.flush_stdout()
    }

    /// Writes text and then flushes standard error.
    pub fn write_stderr_and_flush(
        &self,
        text: &str,
    ) -> Result<()> {
        self.write_stderr(text)?;

        self.flush_stderr()
    }

    /// Writes formatted text to standard output.
    ///
    /// Formatting is performed using Rust's formatting machinery.
    ///
    /// The formatted output is materialized in a String, so its
    /// memory requirement depends on the resulting output size.
    ///
    /// Call `write_stdout` with existing text when avoiding an
    /// additional formatting allocation is preferable.
    pub fn write_stdout_fmt(
        &self,
        arguments: fmt::Arguments<'_>,
    ) -> Result<()> {
        let mut output = String::new();

        fmt::write(&mut output, arguments)
            .map_err(|error| {
                Error::new(
                    ErrorKind::Internal,
                    error.to_string(),
                )
                .in_component(COMPONENT)
                .during_operation(WRITE_STDOUT_OPERATION)
            })?;

        self.write_stdout(&output)
    }

    /// Writes formatted text to standard error.
    pub fn write_stderr_fmt(
        &self,
        arguments: fmt::Arguments<'_>,
    ) -> Result<()> {
        let mut output = String::new();

        fmt::write(&mut output, arguments)
            .map_err(|error| {
                Error::new(
                    ErrorKind::Internal,
                    error.to_string(),
                )
                .in_component(COMPONENT)
                .during_operation(WRITE_STDERR_OPERATION)
            })?;

        self.write_stderr(&output)
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Console {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("Console")
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// FORMATTING MACROS
// ============================================================================

/// Writes formatted text to a console's standard output.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::io::console::{Console, console_print};
///
/// let console = Console::new();
///
/// console_print!(console, "Value: {}", 42).unwrap();
/// ```
#[macro_export]
macro_rules! console_print {
    ($console:expr, $($argument:tt)*) => {
        $console.write_stdout_fmt(
            format_args!($($argument)*)
        )
    };
}

/// Writes formatted text followed by a newline to standard output.
#[macro_export]
macro_rules! console_println {
    ($console:expr) => {
        $console.write_line("")
    };

    ($console:expr, $($argument:tt)*) => {
        $console.write_stdout_fmt(
            format_args!($($argument)*)
        ).and_then(|()| $console.write_stdout("\n"))
    };
}

/// Writes formatted text to standard error.
#[macro_export]
macro_rules! console_eprint {
    ($console:expr, $($argument:tt)*) => {
        $console.write_stderr_fmt(
            format_args!($($argument)*)
        )
    };
}

/// Writes formatted text followed by a newline to standard error.
#[macro_export]
macro_rules! console_eprintln {
    ($console:expr) => {
        $console.write_error_line("")
    };

    ($console:expr, $($argument:tt)*) => {
        $console.write_stderr_fmt(
            format_args!($($argument)*)
        ).and_then(|()| $console.write_stderr("\n"))
    };
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;
    use std::sync::Mutex;

    /// In-memory console provider for isolated testing.
    #[derive(Default)]
    struct MemoryConsoleBackend {
        input: Mutex<VecDeque<String>>,
        stdout: Mutex<String>,
        stderr: Mutex<String>,
    }

    impl MemoryConsoleBackend {
        fn with_input(
            lines: impl IntoIterator<Item = String>,
        ) -> Self {
            Self {
                input: Mutex::new(
                    lines.into_iter().collect(),
                ),
                stdout: Mutex::new(String::new()),
                stderr: Mutex::new(String::new()),
            }
        }

        fn stdout_contents(&self) -> String {
            self.stdout
                .lock()
                .expect("stdout lock")
                .clone()
        }

        fn stderr_contents(&self) -> String {
            self.stderr
                .lock()
                .expect("stderr lock")
                .clone()
        }
    }

    impl ConsoleBackend for MemoryConsoleBackend {
        fn read_line(
            &self,
            context: &OperationContext,
        ) -> Result<Option<String>> {
            context.check_cancelled()?;

            Ok(
                self.input
                    .lock()
                    .map_err(|_| {
                        Error::new(
                            ErrorKind::Concurrency,
                            "memory console input lock poisoned",
                        )
                    })?
                    .pop_front(),
            )
        }

        fn write_stdout(
            &self,
            text: &str,
            context: &OperationContext,
        ) -> Result<()> {
            context.check_cancelled()?;

            self.stdout
                .lock()
                .map_err(|_| {
                    Error::new(
                        ErrorKind::Concurrency,
                        "memory console stdout lock poisoned",
                    )
                })?
                .push_str(text);

            Ok(())
        }

        fn write_stderr(
            &self,
            text: &str,
            context: &OperationContext,
        ) -> Result<()> {
            context.check_cancelled()?;

            self.stderr
                .lock()
                .map_err(|_| {
                    Error::new(
                        ErrorKind::Concurrency,
                        "memory console stderr lock poisoned",
                    )
                })?
                .push_str(text);

            Ok(())
        }

        fn flush_stdout(&self) -> Result<()> {
            Ok(())
        }

        fn flush_stderr(&self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn host_backend_is_constructible() {
        let backend = HostConsoleBackend::new();

        let _ = backend;
    }

    #[test]
    fn console_supports_injected_backend() {
        let backend = Arc::new(
            MemoryConsoleBackend::default(),
        );

        let console = Console::with_shared_backend(
            backend.clone(),
            OperationContext::default(),
        );

        console.write_stdout("Zamani").unwrap();

        assert_eq!(
            backend.stdout_contents(),
            "Zamani",
        );
    }

    #[test]
    fn console_reads_lines() {
        let backend = MemoryConsoleBackend::with_input([
            "first\n".to_owned(),
            "second\n".to_owned(),
        ]);

        let console = Console::with_backend(backend);

        assert_eq!(
            console.read_line().unwrap(),
            Some("first\n".to_owned()),
        );

        assert_eq!(
            console.read_line().unwrap(),
            Some("second\n".to_owned()),
        );

        assert_eq!(
            console.read_line().unwrap(),
            None,
        );
    }

    #[test]
    fn console_distinguishes_empty_line_from_eof() {
        let backend = MemoryConsoleBackend::with_input([
            "\n".to_owned(),
        ]);

        let console = Console::with_backend(backend);

        assert_eq!(
            console.read_line().unwrap(),
            Some("\n".to_owned()),
        );

        assert_eq!(
            console.read_line().unwrap(),
            None,
        );
    }

    #[test]
    fn console_removes_line_terminator() {
        let backend = MemoryConsoleBackend::with_input([
            "Zamani\r\n".to_owned(),
            "Portable\n".to_owned(),
            "Last".to_owned(),
        ]);

        let console = Console::with_backend(backend);

        assert_eq!(
            console
                .read_line_without_terminator()
                .unwrap(),
            Some("Zamani".to_owned()),
        );

        assert_eq!(
            console
                .read_line_without_terminator()
                .unwrap(),
            Some("Portable".to_owned()),
        );

        assert_eq!(
            console
                .read_line_without_terminator()
                .unwrap(),
            Some("Last".to_owned()),
        );
    }

    #[test]
    fn console_writes_output() {
        let backend = Arc::new(
            MemoryConsoleBackend::default(),
        );

        let console = Console::with_shared_backend(
            backend.clone(),
            OperationContext::default(),
        );

        console.write_stdout("Hello").unwrap();
        console.write_line(" Zamani").unwrap();

        assert_eq!(
            backend.stdout_contents(),
            "Hello Zamani\n",
        );
    }

    #[test]
    fn console_writes_error_output() {
        let backend = Arc::new(
            MemoryConsoleBackend::default(),
        );

        let console = Console::with_shared_backend(
            backend.clone(),
            OperationContext::default(),
        );

        console.write_error_line("Failure").unwrap();

        assert_eq!(
            backend.stderr_contents(),
            "Failure\n",
        );
    }

    #[test]
    fn console_supports_formatted_output() {
        let backend = Arc::new(
            MemoryConsoleBackend::default(),
        );

        let console = Console::with_shared_backend(
            backend.clone(),
            OperationContext::default(),
        );

        console
            .write_stdout_fmt(
                format_args!("Value: {}", 42),
            )
            .unwrap();

        assert_eq!(
            backend.stdout_contents(),
            "Value: 42",
        );
    }

    #[test]
    fn console_supports_flush() {
        let console = Console::with_backend(
            MemoryConsoleBackend::default(),
        );

        assert!(console.flush_stdout().is_ok());
        assert!(console.flush_stderr().is_ok());
    }

    #[test]
    fn console_is_cloneable() {
        let console = Console::new();

        let cloned = console.clone();

        assert_eq!(
            console.is_cancelled(),
            cloned.is_cancelled(),
        );
    }

    #[test]
    fn console_default_uses_host_backend() {
        let console = Console::default();

        assert!(!console.is_cancelled());
    }
}
