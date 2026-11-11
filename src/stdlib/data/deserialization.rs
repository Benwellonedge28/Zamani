
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Deserialization
//!
//! Path: `src/stdlib/data/deserialization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a dedicated, portable API for converting encoded data streams
//! into caller-selected canonical data representations.
//!
//! This module delegates actual format decoding to the existing
//! `SerializationEngine` and its registered `SerializationProvider`
//! implementations.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - DeserializationRequest.
//! - DeserializationEngine.
//! - DeserializationReport as an alias of SerializationReport.
//! - Cooperative cancellation of stream operations.
//! - Deserialization-facing API documentation and tests.
//!
//! This module does not own:
//!
//! - SerializationFormatId or SchemaId.
//! - Schema definitions or schema evolution.
//! - Codec implementations.
//! - Format-provider registration.
//! - Canonical Zamani values.
//! - AST or semantic types.
//! - Tensor or quantum-state representations.
//! - Storage implementations.
//! - Filesystem or database access.
//! - Hardware selection.
//! - Scheduling or physical resource allocation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Streaming and scalability
//!
//! Decoding operates on `Read` and `Write` streams.
//!
//! This avoids requiring an entire encoded object or decoded result
//! to fit into one contiguous allocation.
//!
//! No universal input-size, output-size, field-count, record-count,
//! tensor-rank, schema-count, format-count or provider-count ceiling
//! is introduced.
//!
//! Actual limits are determined by the provider, platform, available
//! resources and explicitly configured operational budgets.
//!
//! These limits must not become language-level capability restrictions.
//!
//! ## Integration
//!
//! Codec and provider authority:
//!
//! `crate::stdlib::data::serialization`
//!
//! Byte storage:
//!
//! `crate::stdlib::data::buffers`
//!
//! Persistent storage:
//!
//! `crate::stdlib::data::storage`
//!
//! Operation context and cancellation:
//!
//! `crate::stdlib::context`
//!
//! Standard errors:
//!
//! `crate::stdlib::error`
//!
//! Grammar:
//!
//! `grammar/data/serialization.g4`
//! `grammar/data/deserialization.g4`
//!
//! ## Failure behavior
//!
//! A failed decode may leave partial output in the destination.
//!
//! Callers requiring atomic publication should use a transactional
//! destination, such as a storage writer, and commit only after
//! successful decoding.
//!
//! This module never treats partial or fabricated output as success.

use std::io::{self, Read, Write};

use crate::stdlib::context::OperationContext;
use crate::stdlib::error::Result;

use super::serialization::{
    SerializationEngine,
    SerializationFormatId,
    SerializationOptions,
    SerializationRegistry,
    SerializationReport,
};

/// Deserialization report.
///
/// This is an alias of the canonical serialization report. It does
/// not introduce a competing report representation.
pub type DeserializationReport = SerializationReport;

// ============================================================================
// REQUEST
// ============================================================================

/// Describes one explicit deserialization operation.
///
/// The request identifies the format and carries the options understood
/// by the selected serialization provider.
///
/// The target representation is determined by the caller's canonical
/// data adapter. This request does not introduce a second Zamani value
/// or type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeserializationRequest {
    format: SerializationFormatId,
    options: SerializationOptions,
}

impl DeserializationRequest {
    /// Creates a deserialization request for an explicit format.
    ///
    /// Default provider options are used.
    #[must_use]
    pub fn new(format: SerializationFormatId) -> Self {
        Self {
            format,
            options: SerializationOptions::new(),
        }
    }

    /// Returns the requested serialization format.
    #[must_use]
    pub fn format(&self) -> &SerializationFormatId {
        &self.format
    }

    /// Returns the provider options.
    #[must_use]
    pub fn options(&self) -> &SerializationOptions {
        &self.options
    }

    /// Replaces the provider options.
    ///
    /// Schema references and format-specific properties retain the
    /// meaning defined by the canonical serialization module.
    #[must_use]
    pub fn with_options(
        mut self,
        options: SerializationOptions,
    ) -> Self {
        self.options = options;
        self
    }
}

// ============================================================================
// CANCELLATION-AWARE STREAM ADAPTERS
// ============================================================================

/// A reader that checks cooperative cancellation before every read.
///
/// The adapter does not own the underlying reader or cancellation
/// mechanism.
struct ContextReader<'a> {
    inner: &'a mut dyn Read,
    context: &'a OperationContext,
}

impl Read for ContextReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.context.check_cancelled().map_err(|error| {
            io::Error::new(io::ErrorKind::Interrupted, error)
        })?;

        self.inner.read(buffer)
    }
}

/// A writer that checks cooperative cancellation before every write
/// and flush operation.
///
/// The adapter does not own the underlying writer or cancellation
/// mechanism.
struct ContextWriter<'a> {
    inner: &'a mut dyn Write,
    context: &'a OperationContext,
}

impl Write for ContextWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.context.check_cancelled().map_err(|error| {
            io::Error::new(io::ErrorKind::Interrupted, error)
        })?;

        self.inner.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.context.check_cancelled().map_err(|error| {
            io::Error::new(io::ErrorKind::Interrupted, error)
        })?;

        self.inner.flush()
    }
}

// ============================================================================
// ENGINE
// ============================================================================

/// Dedicated deserialization-facing engine.
///
/// This engine delegates all format lookup, provider dispatch, decoding
/// and report validation to the existing `SerializationEngine`.
///
/// It does not create or maintain a second provider registry.
#[derive(Clone)]
pub struct DeserializationEngine {
    serialization: SerializationEngine,
}

impl DeserializationEngine {
    /// Creates an engine using an existing serialization engine.
    ///
    /// Sharing the existing engine ensures serialization and
    /// deserialization use the same registered format providers.
    #[must_use]
    pub fn new(serialization: SerializationEngine) -> Self {
        Self { serialization }
    }

    /// Creates an engine using an existing provider registry.
    ///
    /// The registry is shared with the resulting serialization engine.
    #[must_use]
    pub fn from_registry(registry: SerializationRegistry) -> Self {
        Self::new(SerializationEngine::new(registry))
    }

    /// Returns the registry used by the underlying serialization engine.
    #[must_use]
    pub fn registry(&self) -> &SerializationRegistry {
        self.serialization.registry()
    }

    /// Deserializes an encoded stream into the supplied destination.
    ///
    /// The format must have a registered provider.
    ///
    /// The provider is responsible for decoding and validating its
    /// documented format and schema contract.
    ///
    /// The destination may contain partial output if decoding fails.
    ///
    /// # Errors
    ///
    /// Returns a structured standard-library error if:
    ///
    /// - the requested format has no registered provider;
    /// - the provider rejects the input;
    /// - decoding fails;
    /// - the provider violates the report contract;
    /// - an underlying stream operation fails.
    pub fn deserialize(
        &self,
        request: &DeserializationRequest,
        source: &mut dyn Read,
        destination: &mut dyn Write,
    ) -> Result<DeserializationReport> {
        self.serialization.decode(
            request.format(),
            source,
            destination,
            request.options(),
        )
    }

    /// Deserializes a stream with cooperative cancellation.
    ///
    /// Cancellation is checked before decoding and at every stream
    /// read, write and flush boundary.
    ///
    /// The underlying provider must use the supplied streams for
    /// cancellation to be observed during its operation.
    ///
    /// A provider that performs independent blocking work must use
    /// its own documented cancellation integration.
    ///
    /// This method does not forcibly terminate a provider, thread,
    /// process, remote worker or physical operation.
    ///
    /// # Errors
    ///
    /// Returns the standard cancellation error when cancellation
    /// is observed. Other failures are propagated from the existing
    /// serialization engine.
    pub fn deserialize_with_context(
        &self,
        request: &DeserializationRequest,
        source: &mut dyn Read,
        destination: &mut dyn Write,
        context: &OperationContext,
    ) -> Result<DeserializationReport> {
        context.check_cancelled()?;

        let mut contextual_source = ContextReader {
            inner: source,
            context,
        };

        let mut contextual_destination = ContextWriter {
            inner: destination,
            context,
        };

        let result = self.serialization.decode(
            request.format(),
            &mut contextual_source,
            &mut contextual_destination,
            request.options(),
        );

        // Prefer the canonical cancellation error if cancellation
        // occurred while the provider was reading or writing.
        if context.is_cancelled() {
            context.check_cancelled()?;
        }

        result
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use crate::stdlib::context::CancellationSignal;
    use crate::stdlib::error::{Error, ErrorKind};

    use super::super::serialization::{
        FormatDescriptor,
        SchemaId,
        SchemaReference,
        SerializationProvider,
        SerializationRegistry,
    };

    struct IdentityProvider {
        descriptor: FormatDescriptor,
    }

    impl IdentityProvider {
        fn new() -> Self {
            Self {
                descriptor: FormatDescriptor::new(
                    SerializationFormatId::new("test.identity")
                        .expect("valid format identifier"),
                    "Identity test provider",
                )
                .with_streaming(true)
                .with_schema_support(true),
            }
        }

        fn copy_stream(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> crate::stdlib::error::Result<SerializationReport> {
            let input_bytes = std::io::copy(source, destination).map_err(
                |error| {
                    Error::new(
                        ErrorKind::Io,
                        "test provider stream operation failed",
                    )
                    .in_component("stdlib.data.deserialization.tests")
                    .during_operation("test.identity.decode")
                    .with_source(error)
                },
            )?;

            Ok(SerializationReport::new(
                self.descriptor.id.clone(),
                input_bytes,
                input_bytes,
                options.schema.clone(),
            ))
        }
    }

    impl SerializationProvider for IdentityProvider {
        fn descriptor(&self) -> &FormatDescriptor {
            &self.descriptor
        }

        fn encode(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> crate::stdlib::error::Result<SerializationReport> {
            self.copy_stream(source, destination, options)
        }

        fn decode(
            &self,
            source: &mut dyn Read,
            destination: &mut dyn Write,
            options: &SerializationOptions,
        ) -> crate::stdlib::error::Result<SerializationReport> {
            self.copy_stream(source, destination, options)
        }
    }

    fn test_engine() -> DeserializationEngine {
        let registry = SerializationRegistry::new();

        registry
            .register(IdentityProvider::new())
            .expect("provider registration succeeds");

        DeserializationEngine::from_registry(registry)
    }

    fn test_request() -> DeserializationRequest {
        DeserializationRequest::new(
            SerializationFormatId::new("test.identity")
                .expect("valid format identifier"),
        )
    }

    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl CancellationSignal for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn request_preserves_explicit_format() {
        let request = test_request();

        assert_eq!(request.format().as_str(), "test.identity");
    }

    #[test]
    fn request_accepts_schema_options() {
        let schema = SchemaReference::new(
            SchemaId::new("zamani.data.record")
                .expect("valid schema identifier"),
        )
        .with_version("1");

        let request = test_request().with_options(
            SerializationOptions::new().with_schema(schema.clone()),
        );

        assert_eq!(request.options().schema, Some(schema));
    }

    #[test]
    fn deserializes_a_stream_using_the_existing_provider() {
        let engine = test_engine();
        let request = test_request();

        let input = b"portable Zamani data";
        let mut source = Cursor::new(input.as_slice());
        let mut destination = Vec::new();

        let report = engine
            .deserialize(
                &request,
                &mut source,
                &mut destination,
            )
            .expect("deserialization succeeds");

        assert_eq!(destination, input);
        assert_eq!(report.format.as_str(), "test.identity");
        assert_eq!(report.input_bytes, input.len() as u64);
        assert_eq!(report.output_bytes, input.len() as u64);
    }

    #[test]
    fn deserializes_with_an_operation_context() {
        let engine = test_engine();
        let request = test_request();

        let context = OperationContext::builder()
            .with_component("application")
            .expect("valid component")
            .with_operation("load.record")
            .expect("valid operation")
            .build();

        let input = b"context-aware data";
        let mut source = Cursor::new(input.as_slice());
        let mut destination = Vec::new();

        engine
            .deserialize_with_context(
                &request,
                &mut source,
                &mut destination,
                &context,
            )
            .expect("deserialization succeeds");

        assert_eq!(destination, input);
    }

    #[test]
    fn rejects_cancellation_before_reading_input() {
        let engine = test_engine();
        let request = test_request();

        let cancellation = Arc::new(TestCancellation {
            cancelled: AtomicBool::new(true),
        });

        let signal: Arc<dyn CancellationSignal> = cancellation;

        let context = OperationContext::builder()
            .with_cancellation(signal)
            .build();

        let mut source = Cursor::new(b"must not be read".as_slice());
        let mut destination = Vec::new();

        let result = engine.deserialize_with_context(
            &request,
            &mut source,
            &mut destination,
            &context,
        );

        assert!(matches!(
            result,
            Err(error) if error.kind() == ErrorKind::Cancelled
        ));

        assert!(destination.is_empty());
        assert_eq!(source.position(), 0);
    }

    #[test]
    fn rejects_unregistered_formats() {
        let engine = DeserializationEngine::from_registry(
            SerializationRegistry::new(),
        );

        let request = DeserializationRequest::new(
            SerializationFormatId::new("unknown.format")
                .expect("valid format identifier"),
        );

        let mut source = Cursor::new(b"input".as_slice());
        let mut destination = Vec::new();

        assert!(engine
            .deserialize(
                &request,
                &mut source,
                &mut destination,
            )
            .is_err());
    }

    #[test]
    fn empty_stream_is_valid_for_a_provider_that_accepts_it() {
        let engine = test_engine();
        let request = test_request();

        let mut source = Cursor::new(&b""[..]);
        let mut destination = Vec::new();

        let report = engine
            .deserialize(
                &request,
                &mut source,
                &mut destination,
            )
            .expect("provider accepts empty stream");

        assert!(destination.is_empty());
        assert_eq!(report.input_bytes, 0);
        assert_eq!(report.output_bytes, 0);
    }

    #[test]
    fn engine_shares_the_existing_provider_registry() {
        let registry = SerializationRegistry::new();

        registry
            .register(IdentityProvider::new())
            .expect("provider registration succeeds");

        let engine = DeserializationEngine::from_registry(
            registry.clone(),
        );

        assert_eq!(engine.registry().provider_count(), Ok(1));
    }
}
