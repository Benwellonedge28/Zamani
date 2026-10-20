
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR External-Format Import Boundary
//!
//! Path: `src/hdl/hdlir/interchange/import.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Extensible import boundary; individual format adapters must
//! be implemented and qualified before they can be advertised as supported.
//!
//! ## Purpose
//!
//! Define the common boundary through which external HDL formats are
//! imported into Zamani's existing HDL semantic-input representation.
//!
//! The pipeline is:
//!
//! ```text
//! untrusted external bytes
//!          |
//!          v
//! input-size and encoding checks
//!          |
//!          v
//! registered format-specific importer
//!          |
//!          v
//! canonical HdlProgram semantic input
//!          |
//!          v
//! canonical HDL semantic validation
//!          |
//!          v
//! src/hdl/hdlir/lowering/from_semantic.rs
//!          |
//!          v
//! HDLIR, for the subset currently supported losslessly
//! ```
//!
//! This module does not claim that parsing alone establishes semantic
//! validity, that semantic validity guarantees HDLIR representability,
//! or that HDLIR representability guarantees target feasibility.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/lowering/from_semantic.rs`
//! - `src/hdl/hdlir/serialization/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - External-format importer registration.
//! - Format identifier validation and lookup.
//! - Import input-size policy.
//! - Import-specific structured errors.
//! - The boundary between external parsing and canonical semantic input.
//!
//! It does not own:
//!
//! - Source grammar, lexical rules, or the canonical Zamani parser.
//! - The canonical AST, source-span system, or semantic model.
//! - HDL semantic validation rules.
//! - HDLIR graph construction, verification, or transformations.
//! - Serialization of HDLIR artifacts.
//! - Vendor tool invocation, target discovery, physical allocation,
//!   synthesis, placement, routing, or timing closure.
//!
//! ## Security and scalability
//!
//! External input is untrusted. An adapter must validate its syntax,
//! reject malformed input, preserve supported semantics, and return
//! explicit errors for unsupported constructs.
//!
//! The byte budget limits input size only. It is not a hardware capacity
//! limit and does not bound every parser's CPU, memory, nesting, or
//! intermediate-representation costs. Adapters must enforce their own
//! explicit work and nesting budgets.
//!
//! No universal hardware maxima are introduced. Finite software remains
//! constrained by available memory, execution time, representable values,
//! supported semantics, and caller-selected operational budgets.
//!
//! The registry is explicitly owned by its caller. This module uses no
//! global mutable state, implicit format discovery, filesystem access,
//! network access, or environment-dependent behavior.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::frontend::semantic::domains::hdl::HdlProgram;

/// A stable, normalized identifier for an external HDL format.
///
/// Include the format and dialect/version in the identifier when their
/// syntax or semantics differ, for example `verilog-2005` or
/// `systemverilog-2017`. These examples are identifiers only; they do not
/// imply that an importer for either format is implemented.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlFormatId(String);

impl HdlFormatId {
    /// Construct a normalized external-format identifier.
    ///
    /// Identifiers are ASCII, non-empty, lowercase, and limited to
    /// alphanumeric characters plus `.`, `_`, and `-`.
    ///
    /// Normalization is deliberately not performed implicitly: callers
    /// must provide canonical identifiers so registration and lookup
    /// remain deterministic.
    pub fn new(value: impl Into<String>) -> Result<Self, ImportError> {
        let value = value.into();

        if value.is_empty()
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
        {
            return Err(ImportError::InvalidFormatId { value });
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HdlFormatId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Per-invocation resource policy for external-format import.
///
/// An absent limit means this policy imposes no limit for that particular
/// quantity. It does not promise unlimited processing.
///
/// The importer adapter remains responsible for enforcing parser-specific
/// work, nesting, diagnostic, and allocation budgets.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImportOptions {
    /// Maximum permitted source size in bytes, or `None` for no limit.
    pub max_source_bytes: Option<u64>,
}

impl ImportOptions {
    /// Construct import options with an explicit source-byte budget.
    #[must_use]
    pub const fn new(max_source_bytes: Option<u64>) -> Self {
        Self { max_source_bytes }
    }

    /// Construct options without an explicit source-byte budget.
    ///
    /// Other operational budgets may still be required by the selected
    /// format adapter.
    #[must_use]
    pub const fn unbounded_source() -> Self {
        Self {
            max_source_bytes: None,
        }
    }
}

/// Borrowed input supplied to a registered format importer.
///
/// Source bytes are not assumed to be UTF-8 until the selected adapter
/// establishes that its format requires UTF-8.
#[derive(Clone, Copy, Debug)]
pub struct ImportRequest<'a> {
    /// Untrusted source bytes.
    pub source: &'a [u8],

    /// Optional caller-provided source name for diagnostics and provenance.
    ///
    /// This is metadata, not permission to read a file or access a network.
    pub source_name: Option<&'a str>,

    /// Immutable policy for this import invocation.
    pub options: ImportOptions,
}

/// The result of parsing an external format into the canonical HDL input.
///
/// `HdlProgram` is the existing repository-owned semantic input type.
/// This wrapper is not a new AST or a competing semantic model.
///
/// The returned program must still pass canonical semantic validation
/// before it can be trusted or lowered.
#[derive(Debug)]
pub struct ImportedHdl {
    /// Canonical HDL semantic input produced by the adapter.
    pub program: HdlProgram,

    /// Format identifier of the adapter that produced this input.
    pub format: HdlFormatId,

    /// Optional caller-provided source name.
    pub source_name: Option<String>,

    /// Number of source bytes consumed by this import request.
    pub source_bytes: u64,
}

/// Structured errors at the external-format import boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImportError {
    /// The format identifier is empty or violates the identifier contract.
    InvalidFormatId {
        /// Identifier supplied by the caller.
        value: String,
    },

    /// No importer has been registered for the requested format.
    UnsupportedFormat {
        /// Requested format identifier.
        format: HdlFormatId,
    },

    /// The source exceeds the configured source-byte budget.
    SourceBudgetExceeded {
        /// Actual source length in bytes.
        actual: u64,

        /// Configured maximum source length.
        limit: u64,
    },

    /// Source length could not be represented by the import API's counter.
    SourceLengthUnrepresentable,

    /// The adapter rejected malformed input.
    MalformedInput {
        /// Format identifier of the responsible adapter.
        format: HdlFormatId,

        /// Actionable description of the parsing failure.
        message: String,
    },

    /// The input uses a construct the adapter does not support.
    UnsupportedConstruct {
        /// Format identifier of the responsible adapter.
        format: HdlFormatId,

        /// Description of the unsupported construct.
        message: String,
    },

    /// The adapter could not complete within its configured operational
    /// budget.
    BudgetExceeded {
        /// Format identifier of the responsible adapter.
        format: HdlFormatId,

        /// Budget or resource that was exhausted.
        resource: String,
    },

    /// Registration would replace an existing format adapter.
    DuplicateFormat {
        /// Already registered format identifier.
        format: HdlFormatId,
    },

    /// A format adapter failed internally.
    ///
    /// This is for expected, recoverable adapter failures. Adapters should
    /// not catch and disguise panics as ordinary import errors.
    AdapterFailure {
        /// Format identifier of the responsible adapter.
        format: HdlFormatId,

        /// Actionable description of the failure.
        message: String,
    },
}

impl fmt::Display for ImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormatId { value } => {
                write!(formatter, "invalid HDL format identifier: {value:?}")
            }
            Self::UnsupportedFormat { format } => {
                write!(formatter, "no importer registered for HDL format {format}")
            }
            Self::SourceBudgetExceeded { actual, limit } => write!(
                formatter,
                "HDL import source size {actual} bytes exceeds budget {limit} bytes"
            ),
            Self::SourceLengthUnrepresentable => {
                formatter.write_str("HDL import source length is not representable")
            }
            Self::MalformedInput { format, message } => {
                write!(formatter, "malformed {format} input: {message}")
            }
            Self::UnsupportedConstruct { format, message } => {
                write!(formatter, "unsupported construct in {format}: {message}")
            }
            Self::BudgetExceeded { format, resource } => {
                write!(formatter, "{format} importer exhausted budget: {resource}")
            }
            Self::DuplicateFormat { format } => {
                write!(formatter, "HDL format {format} is already registered")
            }
            Self::AdapterFailure { format, message } => {
                write!(formatter, "{format} importer failed: {message}")
            }
        }
    }
}

impl Error for ImportError {}

/// Contract implemented by each supported external HDL format.
///
/// Implementations must:
///
/// - Parse only the format and dialect they explicitly claim to support.
/// - Treat the source as untrusted input.
/// - Enforce parser-specific work and nesting budgets.
/// - Return the existing `HdlProgram` representation without creating
///   another AST or semantic model.
/// - Preserve every construct and semantic distinction represented by
///   the canonical input contract.
/// - Reject unsupported or lossy constructs explicitly.
/// - Avoid reporting a successful import after truncation or incomplete
///   parsing.
/// - Avoid filesystem, network, environment, or target discovery unless
///   a separately authorized higher-level service explicitly provides it.
///
/// Importing syntax is not semantic validation. The caller must run the
/// canonical validator and then use the established lowering adapter.
pub trait ExternalHdlImporter: Send + Sync {
    /// Return the canonical format identifier handled by this adapter.
    fn format_id(&self) -> HdlFormatId;

    /// Parse the requested source into the existing HDL semantic input.
    fn import(&self, request: ImportRequest<'_>) -> Result<HdlProgram, ImportError>;
}

/// Explicit registry of available external-format adapters.
///
/// A registry has no built-in formats. A format becomes supported only
/// after a concrete adapter is registered and its conformance tests pass.
///
/// `BTreeMap` provides deterministic iteration and listing order.
#[derive(Default)]
pub struct ImportRegistry {
    adapters: BTreeMap<HdlFormatId, Arc<dyn ExternalHdlImporter>>,
}

impl fmt::Debug for ImportRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImportRegistry")
            .field("registered_formats", &self.adapters.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ImportRegistry {
    /// Create an empty registry.
    ///
    /// No external format is presumed to be supported.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a format importer without replacing an existing adapter.
    ///
    /// The adapter's declared identifier is checked before registration.
    /// Duplicate registration is rejected to avoid behavior changing
    /// according to initialization order.
    pub fn register(
        &mut self,
        adapter: Arc<dyn ExternalHdlImporter>,
    ) -> Result<(), ImportError> {
        let format = adapter.format_id();

        // Revalidate the adapter-provided identifier at the trust boundary.
        HdlFormatId::new(format.as_str().to_owned())?;

        if self.adapters.contains_key(&format) {
            return Err(ImportError::DuplicateFormat { format });
        }

        self.adapters.insert(format, adapter);
        Ok(())
    }

    /// Return whether an adapter is registered for the specified format.
    #[must_use]
    pub fn contains(&self, format: &HdlFormatId) -> bool {
        self.adapters.contains_key(format)
    }

    /// List registered formats in deterministic lexical order.
    #[must_use]
    pub fn formats(&self) -> Vec<HdlFormatId> {
        self.adapters.keys().cloned().collect()
    }

    /// Import source bytes through the selected registered adapter.
    ///
    /// The source-size budget is checked before the adapter is invoked.
    /// This method does not validate the returned program or lower it to
    /// HDLIR; those operations belong to their canonical owners.
    pub fn import(
        &self,
        format: &HdlFormatId,
        request: ImportRequest<'_>,
    ) -> Result<ImportedHdl, ImportError> {
        let source_bytes = u64::try_from(request.source.len())
            .map_err(|_| ImportError::SourceLengthUnrepresentable)?;

        if let Some(limit) = request.options.max_source_bytes {
            if source_bytes > limit {
                return Err(ImportError::SourceBudgetExceeded {
                    actual: source_bytes,
                    limit,
                });
            }
        }

        let adapter = self
            .adapters
            .get(format)
            .ok_or_else(|| ImportError::UnsupportedFormat {
                format: format.clone(),
            })?;

        // Do not catch panics here: an adapter panic is an implementation
        // defect, not a valid malformed-input or unsupported-format result.
        let program = adapter.import(request)?;

        Ok(ImportedHdl {
            program,
            format: format.clone(),
            source_name: request.source_name.map(str::to_owned),
            source_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestImporter {
        format: HdlFormatId,
    }

    impl ExternalHdlImporter for TestImporter {
        fn format_id(&self) -> HdlFormatId {
            self.format.clone()
        }

        fn import(&self, _request: ImportRequest<'_>) -> Result<HdlProgram, ImportError> {
            Ok(HdlProgram::default())
        }
    }

    fn test_format() -> HdlFormatId {
        HdlFormatId::new("test-hdl-v1").expect("valid test format")
    }

    fn test_importer() -> Arc<dyn ExternalHdlImporter> {
        Arc::new(TestImporter {
            format: test_format(),
        })
    }

    #[test]
    fn format_ids_require_canonical_ascii_identifiers() {
        assert!(HdlFormatId::new("verilog-2005").is_ok());
        assert!(HdlFormatId::new("systemverilog_2017").is_ok());
        assert!(HdlFormatId::new("").is_err());
        assert!(HdlFormatId::new("Verilog").is_err());
        assert!(HdlFormatId::new("../verilog").is_err());
        assert!(HdlFormatId::new("vhdl 2008").is_err());
    }

    #[test]
    fn registry_starts_empty_and_lists_formats_deterministically() {
        let registry = ImportRegistry::new();

        assert!(registry.formats().is_empty());
        assert!(!registry.contains(&test_format()));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = ImportRegistry::new();

        registry.register(test_importer()).expect("first registration");
        let error = registry
            .register(test_importer())
            .expect_err("duplicate registration must fail");

        assert!(matches!(error, ImportError::DuplicateFormat { .. }));
    }

    #[test]
    fn unknown_format_is_not_guessed_or_silently_fallbacked() {
        let registry = ImportRegistry::new();
        let request = ImportRequest {
            source: b"module top;",
            source_name: None,
            options: ImportOptions::default(),
        };

        let error = registry
            .import(&test_format(), request)
            .expect_err("unregistered format must fail");

        assert!(matches!(error, ImportError::UnsupportedFormat { .. }));
    }

    #[test]
    fn source_budget_is_checked_before_adapter_invocation() {
        let mut registry = ImportRegistry::new();
        registry.register(test_importer()).expect("registration");

        let request = ImportRequest {
            source: b"module top;",
            source_name: Some("fixture.hdl"),
            options: ImportOptions::new(Some(1)),
        };

        let error = registry
            .import(&test_format(), request)
            .expect_err("source exceeds budget");

        assert!(matches!(
            error,
            ImportError::SourceBudgetExceeded {
                actual: 11,
                limit: 1
            }
        ));
    }

    #[test]
    fn exact_source_budget_is_accepted() {
        let mut registry = ImportRegistry::new();
        registry.register(test_importer()).expect("registration");

        let source = b"";
        let result = registry.import(
            &test_format(),
            ImportRequest {
                source,
                source_name: Some("empty.fixture"),
                options: ImportOptions::new(Some(0)),
            },
        );

        assert!(result.is_ok());
        let imported = result.expect("empty source fits zero-byte budget");
        assert_eq!(imported.source_bytes, 0);
        assert_eq!(imported.source_name.as_deref(), Some("empty.fixture"));
        assert_eq!(imported.format, test_format());
    }

    #[test]
    fn registry_format_listing_is_stable() {
        let mut registry = ImportRegistry::new();

        let second = HdlFormatId::new("test-format-b").expect("format");
        let first = HdlFormatId::new("test-format-a").expect("format");

        registry
            .register(Arc::new(TestImporter {
                format: second.clone(),
            }))
            .expect("register second");
        registry
            .register(Arc::new(TestImporter {
                format: first.clone(),
            }))
            .expect("register first");

        assert_eq!(registry.formats(), vec![first, second]);
    }
}
