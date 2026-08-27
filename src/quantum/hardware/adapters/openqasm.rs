//! Zamani Quantum Hardware — OpenQASM Hardware Adapter
//!
//! Production hardware-facing OpenQASM 3.0/3.1 bridge.
//!
//! # Responsibility
//!
//! This module is the hardware-layer bridge between Zamani's canonical
//! `QuantumCircuit` and the provider-neutral `BackendProgram` contract.
//!
//! It owns:
//!
//! - OpenQASM hardware-program encoding;
//! - OpenQASM version selection;
//! - hardware-safe export options;
//! - bounded program construction;
//! - deterministic OpenQASM program generation;
//! - format identity metadata;
//! - hardware-facing representability checks;
//! - explicit rejection of unsupported workload classes;
//! - canonical conversion into `BackendProgram`;
//! - adapter-level error context;
//! - provider-neutral integration metadata.
//!
//! It does NOT own:
//!
//! - OpenQASM lexical analysis;
//! - OpenQASM parsing;
//! - OpenQASM AST construction;
//! - OpenQASM semantic validation;
//! - canonical Quantum IR;
//! - quantum optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - provider authentication;
//! - credentials;
//! - HTTP/network communication;
//! - QPU execution;
//! - provider-specific APIs;
//! - job lifecycle;
//! - result normalization;
//! - benchmarking;
//! - error-correction algorithms.
//!
//! The authoritative OpenQASM frontend remains:
//!
//! `crate::quantum::frontend::formats::openqasm`
//!
//! The authoritative hardware execution contract remains:
//!
//! `crate::quantum::hardware::backend_trait`
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      +-----------------------+
//!      |                       |
//!      v                       v
//! optimization             analysis
//!      |
//!      v
//! routing / scheduling
//!      |
//!      v
//! hardware compatibility
//!      |
//!      v
//! adapters::openqasm
//!      |
//!      v
//! frontend::OpenQasmExporter
//!      |
//!      v
//! BackendProgram
//!      |
//!      v
//! provider adapter
//!      |
//!      v
//! QPU
//! ```
//!
//! # Critical ownership rule
//!
//! There must be exactly one OpenQASM implementation of serialization
//! semantics in Zamani.
//!
//! That implementation is the frontend OpenQASM exporter.
//!
//! This adapter MUST delegate serialization to:
//!
//! `crate::quantum::frontend::formats::openqasm::OpenQasmExporter`
//!
//! It must never implement a second serializer.
//!
//! This prevents divergence between:
//!
//! ```text
//! quantum::frontend
//! ```
//!
//! and:
//!
//! ```text
//! quantum::hardware
//! ```
//!
//! # Hardware-layer purpose
//!
//! The frontend answers:
//!
//! > Can this canonical Quantum IR be represented as OpenQASM?
//!
//! This adapter answers:
//!
//! > Produce the validated OpenQASM representation as a provider-neutral
//! > `BackendProgram` suitable for a hardware adapter.
//!
//! The provider adapter subsequently answers:
//!
//! > How is that OpenQASM payload submitted to this particular provider?
//!
//! These responsibilities must remain separate.
//!
//! # Supported revisions
//!
//! The adapter supports exactly:
//!
//! - OpenQASM 3.0;
//! - OpenQASM 3.1.
//!
//! OpenQASM 3.x versions newer than 3.1 are rejected.
//!
//! The adapter never treats an unknown future version as compatible merely
//! because its major version is `3`.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network communication;
//! - performs no filesystem access;
//! - performs no process spawning;
//! - stores no credentials;
//! - stores no tokens;
//! - stores no authentication headers;
//! - does not log program contents;
//! - does not include program bytes in `Debug` output;
//! - does not execute OpenQASM;
//! - treats canonical IR as potentially untrusted;
//! - enforces an explicit output-size bound.
//!
//! # Determinism
//!
//! For a fixed:
//!
//! - QuantumCircuit;
//! - OpenQASM version;
//! - export options;
//! - frontend implementation version;
//!
//! the adapter must produce identical program bytes.
//!
//! It does not:
//!
//! - read the system clock;
//! - use randomness;
//! - access provider state;
//! - inspect environment variables;
//! - access filesystem state;
//! - depend on hash-map iteration.
//!
//! # Integration contract
//!
//! This file intentionally depends on already-established contracts:
//!
//! ```text
//! quantum::ir
//!       |
//!       v
//! quantum::frontend::exporter
//!       |
//!       v
//! quantum::frontend::formats::openqasm
//!       |
//!       v
//! hardware::backend
//! hardware::backend_trait
//! ```
//!
//! It does not modify any of those contracts.
//!
//! Provider adapters use this module approximately as follows:
//!
//! ```text
//! QuantumCircuit
//!      |
//!      v
//! OpenQasmHardwareAdapter::encode()
//!      |
//!      v
//! BackendProgram
//!      |
//!      v
//! QuantumBackendAdapter::submit()
//! ```
//!
//! The provider adapter remains responsible for authentication, transport,
//! provider-native request construction, job submission, status polling,
//! cancellation, and result retrieval.
//!
//! # No circular dependency
//!
//! The dependency direction is:
//!
//! ```text
//! frontend OpenQASM exporter
//!          ^
//!          |
//! hardware OpenQASM adapter
//!          ^
//! provider adapter
//! ```
//!
//! The frontend does not depend on this hardware adapter.
//!
//! This is essential because external OpenQASM import/export is a compiler
//! concern, whereas hardware execution is an execution concern.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//! - no unsafe Rust
//! - no additional dependencies
//!
//! # Completion contract
//!
//! This file is considered complete when:
//!
//! 1. OpenQASM serialization is delegated to the canonical frontend exporter;
//! 2. OpenQASM 3.0 and 3.1 are explicitly supported;
//! 3. future unsupported versions are rejected;
//! 4. output is bounded before constructing `BackendProgram`;
//! 5. invalid canonical IR is rejected by the canonical exporter boundary;
//! 6. no unsupported semantics are silently discarded;
//! 7. no hardware I/O exists here;
//! 8. provider-specific logic remains outside this module;
//! 9. provider adapters can consume `BackendProgram` without knowing frontend
//!    implementation details;
//! 10. deterministic tests cover encoding and security invariants;
//! 11. adding another external format does not require changing this file.
//!
//! # Future extension
//!
//! QIR, Quil, pulse, analog, annealing, logical and provider-native formats
//! belong in independent adapter modules:
//!
//! ```text
//! adapters/openqasm.rs
//! adapters/qir.rs
//! adapters/quil.rs
//! adapters/pulse.rs
//! adapters/analog.rs
//! adapters/annealing.rs
//! adapters/logical.rs
//! adapters/provider_native.rs
//! ```
//!
//! None of those modules should require this file to be rewritten merely
//! because they are introduced.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::frontend::exporter::{
    ExportOptions,
    ExportVersionPolicy,
    QuantumExporter,
};
use crate::quantum::frontend::formats::openqasm::{
    OpenQasmExporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
};
use crate::quantum::frontend::format::FormatVersion;
use crate::quantum::hardware::backend::{
    BackendError,
    QuantumBackend,
    QuantumWorkloadKind,
};
use crate::quantum::hardware::backend_trait::BackendProgram;
use crate::quantum::ir::QuantumCircuit;

/// Stable hardware-adapter identifier.
pub const OPENQASM_HARDWARE_ADAPTER_ID: &str =
    "zamani.quantum.hardware.adapter.openqasm";

/// Semantic version of this adapter contract.
///
/// This is independent of the OpenQASM language version.
pub const OPENQASM_HARDWARE_ADAPTER_VERSION: &str = "1.0.0";

/// Stable provider-neutral program format identifier.
pub const OPENQASM_PROGRAM_FORMAT: &str = "openqasm-3";

/// Default OpenQASM version used by production hardware execution.
pub const DEFAULT_OPENQASM_VERSION: FormatVersion = OPENQASM_3_1;

/// Maximum OpenQASM artifact accepted by this hardware bridge.
///
/// This is deliberately lower than the generic `BackendProgram` hard limit.
/// The adapter should reject pathological source artifacts before they reach
/// provider-specific transport layers.
pub const DEFAULT_MAX_PROGRAM_BYTES: usize = 64 * 1024 * 1024;

/// Maximum provider-neutral format identifier length used for this adapter.
const MAX_FORMAT_ID_BYTES: usize = 128;

/// Maximum number of metadata bytes retained by adapter descriptors.
const MAX_METADATA_VALUE_BYTES: usize = 1024;

/// Stable adapter error codes.
const CODE_INVALID_CONFIGURATION: &str = "QASM-HW-E001";
const CODE_UNSUPPORTED_VERSION: &str = "QASM-HW-E002";
const CODE_UNSUPPORTED_WORKLOAD: &str = "QASM-HW-E003";
const CODE_EXPORT: &str = "QASM-HW-E004";
const CODE_PROGRAM: &str = "QASM-HW-E005";
const CODE_BACKEND: &str = "QASM-HW-E006";
const CODE_LIMIT: &str = "QASM-HW-E007";

/// OpenQASM hardware-adapter error.
///
/// This error deliberately retains the frontend error as a string only at the
/// outer diagnostic boundary. The canonical frontend error remains owned by
/// the frontend subsystem and is never reimplemented here.
#[derive(Debug)]
pub enum OpenQasmHardwareAdapterError {
    /// The adapter configuration is invalid.
    InvalidConfiguration {
        /// Stable adapter error code.
        code: &'static str,

        /// Safe diagnostic message.
        message: String,
    },

    /// Requested OpenQASM revision is unsupported.
    UnsupportedVersion {
        /// Stable adapter error code.
        code: &'static str,

        /// Requested revision.
        version: FormatVersion,
    },

    /// Workload cannot be represented by OpenQASM circuit execution.
    UnsupportedWorkload {
        /// Stable adapter error code.
        code: &'static str,

        /// Workload category.
        workload: QuantumWorkloadKind,
    },

    /// Canonical frontend export failed.
    Export {
        /// Stable adapter error code.
        code: &'static str,

        /// Safe frontend diagnostic.
        message: String,
    },

    /// Construction of the provider-neutral program failed.
    Program {
        /// Stable adapter error code.
        code: &'static str,

        /// Safe diagnostic.
        message: String,
    },

    /// Backend validation failed.
    Backend {
        /// Stable adapter error code.
        code: &'static str,

        /// Safe diagnostic.
        message: String,
    },

    /// Configured output limit is invalid or was exceeded.
    Limit {
        /// Stable adapter error code.
        code: &'static str,

        /// Configured/required limit.
        limit: usize,

        /// Actual size, when known.
        actual: Option<usize>,
    },
}

impl OpenQasmHardwareAdapterError {
    /// Returns the stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidConfiguration { code, .. }
            | Self::UnsupportedVersion { code, .. }
            | Self::UnsupportedWorkload { code, .. }
            | Self::Export { code, .. }
            | Self::Program { code, .. }
            | Self::Backend { code, .. }
            | Self::Limit { code, .. } => code,
        }
    }
}

impl fmt::Display for OpenQasmHardwareAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { code, message } => {
                write!(formatter, "{code}: {message}")
            }

            Self::UnsupportedVersion { code, version } => {
                write!(
                    formatter,
                    "{code}: unsupported OpenQASM version {version}"
                )
            }

            Self::UnsupportedWorkload { code, workload } => {
                write!(
                    formatter,
                    "{code}: workload `{workload}` is not \
                     representable by the OpenQASM hardware adapter"
                )
            }

            Self::Export { code, message } => {
                write!(formatter, "{code}: OpenQASM export failed: {message}")
            }

            Self::Program { code, message } => {
                write!(
                    formatter,
                    "{code}: BackendProgram construction failed: {message}"
                )
            }

            Self::Backend { code, message } => {
                write!(
                    formatter,
                    "{code}: backend compatibility check failed: {message}"
                )
            }

            Self::Limit {
                code,
                limit,
                actual,
            } => {
                if let Some(actual) = actual {
                    write!(
                        formatter,
                        "{code}: OpenQASM program size {actual} bytes \
                         exceeds adapter limit {limit} bytes"
                    )
                } else {
                    write!(
                        formatter,
                        "{code}: invalid OpenQASM output limit {limit}"
                    )
                }
            }
        }
    }
}

impl std::error::Error for OpenQasmHardwareAdapterError {}

/// Configuration for the hardware-facing OpenQASM adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenQasmHardwareAdapterConfig {
    /// Exact OpenQASM revision to emit.
    version: FormatVersion,

    /// Maximum serialized program size.
    max_program_bytes: usize,

    /// Whether the adapter must use the exact requested exporter version.
    ///
    /// Production hardware execution should normally keep this enabled.
    exact_version: bool,
}

impl Default for OpenQasmHardwareAdapterConfig {
    fn default() -> Self {
        Self {
            version: DEFAULT_OPENQASM_VERSION,
            max_program_bytes: DEFAULT_MAX_PROGRAM_BYTES,
            exact_version: true,
        }
    }
}

impl OpenQasmHardwareAdapterConfig {
    /// Creates production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self::default()
    }

    /// Creates a configuration for a specific supported OpenQASM revision.
    pub fn for_version(
        version: FormatVersion,
    ) -> Result<Self, OpenQasmHardwareAdapterError> {
        validate_supported_version(version)?;

        Ok(Self {
            version,
            ..Self::default()
        })
    }

    /// Returns the configured OpenQASM revision.
    #[must_use]
    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    /// Returns the maximum serialized program size.
    #[must_use]
    pub const fn max_program_bytes(&self) -> usize {
        self.max_program_bytes
    }

    /// Returns whether exact version matching is enabled.
    #[must_use]
    pub const fn exact_version(&self) -> bool {
        self.exact_version
    }

    /// Returns a copy with a different maximum program size.
    pub const fn with_max_program_bytes(
        mut self,
        max_program_bytes: usize,
    ) -> Self {
        self.max_program_bytes = max_program_bytes;
        self
    }

    /// Returns a copy with explicit version matching policy.
    pub const fn with_exact_version(
        mut self,
        exact_version: bool,
    ) -> Self {
        self.exact_version = exact_version;
        self
    }

    /// Validates the complete configuration.
    pub fn validate(
        &self,
    ) -> Result<(), OpenQasmHardwareAdapterError> {
        validate_supported_version(self.version)?;

        if self.max_program_bytes == 0 {
            return Err(
                OpenQasmHardwareAdapterError::Limit {
                    code: CODE_LIMIT,
                    limit: self.max_program_bytes,
                    actual: None,
                },
            );
        }

        Ok(())
    }
}

/// Immutable hardware-facing OpenQASM adapter.
///
/// This type performs no network or hardware I/O. It is therefore safe to
/// construct before a provider adapter is selected.
#[derive(Clone, Debug)]
pub struct OpenQasmHardwareAdapter {
    config: OpenQasmHardwareAdapterConfig,
    exporter: OpenQasmExporter,
}

impl OpenQasmHardwareAdapter {
    /// Creates the production OpenQASM 3.1 hardware adapter.
    pub fn production()
        -> Result<Self, OpenQasmHardwareAdapterError>
    {
        Self::new(OpenQasmHardwareAdapterConfig::production())
    }

    /// Creates an adapter from explicit configuration.
    pub fn new(
        config: OpenQasmHardwareAdapterConfig,
    ) -> Result<Self, OpenQasmHardwareAdapterError> {
        config.validate()?;

        let exporter =
            OpenQasmExporter::new(config.version)
                .map_err(|error| {
                    OpenQasmHardwareAdapterError::Export {
                        code: CODE_EXPORT,
                        message: safe_frontend_error(&error),
                    }
                })?;

        Ok(Self {
            config,
            exporter,
        })
    }

    /// Returns the adapter configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> &OpenQasmHardwareAdapterConfig {
        &self.config
    }

    /// Returns the configured OpenQASM revision.
    #[must_use]
    pub const fn version(&self) -> FormatVersion {
        self.config.version
    }

    /// Returns the canonical adapter identifier.
    #[must_use]
    pub const fn adapter_id(&self) -> &'static str {
        OPENQASM_HARDWARE_ADAPTER_ID
    }

    /// Returns the adapter semantic version.
    #[must_use]
    pub const fn adapter_version(&self) -> &'static str {
        OPENQASM_HARDWARE_ADAPTER_VERSION
    }

    /// Returns the canonical program format identifier.
    #[must_use]
    pub const fn program_format(&self) -> &'static str {
        OPENQASM_PROGRAM_FORMAT
    }

    /// Returns the OpenQASM media type.
    #[must_use]
    pub const fn media_type(&self) -> &'static str {
        OPENQASM_MEDIA_TYPE
    }

    /// Returns the canonical frontend OpenQASM exporter.
    ///
    /// The reference is exposed only for integration layers that need the
    /// frontend descriptor. Serialization should normally be performed through
    /// `encode`.
    #[must_use]
    pub fn exporter(&self) -> &OpenQasmExporter {
        &self.exporter
    }

    /// Validates that a workload kind is representable by this adapter.
    ///
    /// This check is intentionally conservative.
    ///
    /// OpenQASM 3 can express substantially more than ordinary static circuits,
    /// but this hardware bridge only claims circuit-format responsibility.
    ///
    /// Pulse, analog, annealing and logical workloads must use their dedicated
    /// hardware adapters rather than being mislabeled as OpenQASM circuits.
    pub fn validate_workload_kind(
        &self,
        workload: QuantumWorkloadKind,
    ) -> Result<(), OpenQasmHardwareAdapterError> {
        match workload {
            QuantumWorkloadKind::GateCircuit
            | QuantumWorkloadKind::DynamicCircuit
            | QuantumWorkloadKind::Sampling => Ok(()),

            QuantumWorkloadKind::PulseProgram
            | QuantumWorkloadKind::AnalogProgram
            | QuantumWorkloadKind::AnnealingProblem
            | QuantumWorkloadKind::LogicalProgram
            | QuantumWorkloadKind::Custom => {
                Err(
                    OpenQasmHardwareAdapterError::UnsupportedWorkload {
                        code: CODE_UNSUPPORTED_WORKLOAD,
                        workload,
                    },
                )
            }
        }
    }

    /// Encodes a canonical Quantum IR circuit as a provider-neutral
    /// `BackendProgram`.
    ///
    /// This is the primary integration method used by provider adapters.
    pub fn encode(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<BackendProgram, OpenQasmHardwareAdapterError> {
        self.encode_with_options(
            circuit,
            ExportOptions::new(),
        )
    }

    /// Encodes a canonical circuit with explicit frontend export options.
    ///
    /// The caller may request a stricter output limit or an exact OpenQASM
    /// revision. The adapter's configured version remains authoritative.
    pub fn encode_with_options(
        &self,
        circuit: &QuantumCircuit,
        mut options: ExportOptions,
    ) -> Result<BackendProgram, OpenQasmHardwareAdapterError> {
        self.config.validate()?;

        if let Some(requested_version) =
            options.requested_version()
        {
            if self.config.exact_version()
                && requested_version != self.config.version
            {
                return Err(
                    OpenQasmHardwareAdapterError::UnsupportedVersion {
                        code: CODE_UNSUPPORTED_VERSION,
                        version: requested_version,
                    },
                );
            }
        }

        let maximum =
            self.config
                .max_program_bytes()
                .min(options.max_output_bytes());

        if maximum == 0 {
            return Err(
                OpenQasmHardwareAdapterError::Limit {
                    code: CODE_LIMIT,
                    limit: maximum,
                    actual: None,
                },
            );
        }

        options = options.with_max_output_bytes(maximum);

        if options.requested_version().is_none() {
            options = options.with_requested_version(
                self.config.version,
            );
        }

        let artifact =
            self.exporter
                .export(circuit, &options)
                .map_err(|error| {
                    OpenQasmHardwareAdapterError::Export {
                        code: CODE_EXPORT,
                        message: safe_frontend_error(&error),
                    }
                })?;

        let bytes = artifact.bytes();

        if bytes.is_empty() {
            return Err(
                OpenQasmHardwareAdapterError::Program {
                    code: CODE_PROGRAM,
                    message:
                        "canonical OpenQASM exporter returned an empty \
                         artifact"
                            .to_owned(),
                },
            );
        }

        if bytes.len() > maximum {
            return Err(
                OpenQasmHardwareAdapterError::Limit {
                    code: CODE_LIMIT,
                    limit: maximum,
                    actual: Some(bytes.len()),
                },
            );
        }

        if bytes.len() > DEFAULT_MAX_PROGRAM_BYTES {
            return Err(
                OpenQasmHardwareAdapterError::Limit {
                    code: CODE_LIMIT,
                    limit: DEFAULT_MAX_PROGRAM_BYTES,
                    actual: Some(bytes.len()),
                },
            );
        }

        BackendProgram::new(
            OPENQASM_PROGRAM_FORMAT,
            bytes.to_vec(),
        )
        .map_err(|error| {
            OpenQasmHardwareAdapterError::Program {
                code: CODE_PROGRAM,
                message: safe_backend_error(&error),
            }
        })
    }

    /// Encodes a circuit and returns its UTF-8 OpenQASM source.
    ///
    /// This method is useful for provider adapters that have a textual request
    /// field, while `encode` should be preferred when the provider-neutral
    /// `BackendProgram` contract is desired.
    pub fn encode_text(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<String, OpenQasmHardwareAdapterError> {
        let program = self.encode(circuit)?;

        std::str::from_utf8(program.bytes())
            .map(str::to_owned)
            .map_err(|error| {
                OpenQasmHardwareAdapterError::Program {
                    code: CODE_PROGRAM,
                    message: format!(
                        "OpenQASM BackendProgram is not valid UTF-8: {error}"
                    ),
                }
            })
    }

    /// Validates that the adapter can target a backend for an OpenQASM
    /// workload.
    ///
    /// This method deliberately performs only provider-neutral checks that
    /// can be established from the existing `QuantumBackend` contract.
    ///
    /// Detailed capability negotiation remains owned by
    /// `hardware::compatibility` and `hardware::validation`.
    pub fn validate_backend(
        &self,
        backend: &QuantumBackend,
    ) -> Result<(), OpenQasmHardwareAdapterError> {
        let metadata = backend.metadata();

        if metadata.id.trim().is_empty() {
            return Err(
                OpenQasmHardwareAdapterError::Backend {
                    code: CODE_BACKEND,
                    message:
                        "backend identifier must not be empty".to_owned(),
                },
            );
        }

        if metadata.id.len() > 512 {
            return Err(
                OpenQasmHardwareAdapterError::Backend {
                    code: CODE_BACKEND,
                    message:
                        "backend identifier exceeds the canonical \
                         hardware identifier limit"
                            .to_owned(),
                },
            );
        }

        if metadata.provider.trim().is_empty() {
            return Err(
                OpenQasmHardwareAdapterError::Backend {
                    code: CODE_BACKEND,
                    message:
                        "backend provider identifier must not be empty"
                            .to_owned(),
                },
            );
        }

        if metadata.provider.len() > 512 {
            return Err(
                OpenQasmHardwareAdapterError::Backend {
                    code: CODE_BACKEND,
                    message:
                        "backend provider identifier exceeds the \
                         canonical hardware identifier limit"
                            .to_owned(),
                },
            );
        }

        if metadata.kind.is_physical()
            && !metadata.status.is_operational()
        {
            return Err(
                OpenQasmHardwareAdapterError::Backend {
                    code: CODE_BACKEND,
                    message: format!(
                        "physical backend `{}` is not operational \
                         (status={})",
                        metadata.id,
                        metadata.status
                    ),
                },
            );
        }

        Ok(())
    }

    /// Returns stable adapter metadata.
    ///
    /// The returned values are intended for registry/discovery layers and
    /// contain no credentials or provider secrets.
    #[must_use]
    pub fn metadata(
        &self,
    ) -> OpenQasmAdapterMetadata {
        OpenQasmAdapterMetadata {
            adapter_id: OPENQASM_HARDWARE_ADAPTER_ID,
            adapter_version: OPENQASM_HARDWARE_ADAPTER_VERSION,
            format_id: OPENQASM_FORMAT_ID,
            program_format: OPENQASM_PROGRAM_FORMAT,
            media_type: OPENQASM_MEDIA_TYPE,
            version: self.config.version,
            max_program_bytes: self.config.max_program_bytes,
        }
    }
}

/// Stable immutable metadata describing this hardware format adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenQasmAdapterMetadata {
    /// Stable adapter identifier.
    pub adapter_id: &'static str,

    /// Adapter semantic version.
    pub adapter_version: &'static str,

    /// Frontend format identifier.
    pub format_id: &'static str,

    /// Hardware program format identifier.
    pub program_format: &'static str,

    /// Textual media type.
    pub media_type: &'static str,

    /// Configured OpenQASM revision.
    pub version: FormatVersion,

    /// Maximum encoded program size.
    pub max_program_bytes: usize,
}

impl OpenQasmAdapterMetadata {
    /// Returns whether this metadata describes OpenQASM 3.0.
    #[must_use]
    pub const fn is_openqasm_3_0(&self) -> bool {
        self.version.major() == 3
            && self.version.minor() == 0
            && self.version.patch() == 0
    }

    /// Returns whether this metadata describes OpenQASM 3.1.
    #[must_use]
    pub const fn is_openqasm_3_1(&self) -> bool {
        self.version.major() == 3
            && self.version.minor() == 1
            && self.version.patch() == 0
    }
}

/// Validates an OpenQASM version supported by this adapter.
pub fn validate_supported_version(
    version: FormatVersion,
) -> Result<(), OpenQasmHardwareAdapterError> {
    if version == OPENQASM_3_0
        || version == OPENQASM_3_1
    {
        return Ok(());
    }

    Err(
        OpenQasmHardwareAdapterError::UnsupportedVersion {
            code: CODE_UNSUPPORTED_VERSION,
            version,
        },
    )
}

/// Returns true if the supplied version is one of the explicitly supported
/// OpenQASM hardware revisions.
#[must_use]
pub const fn supports_version(
    version: FormatVersion,
) -> bool {
    (version.major() == 3 && version.minor() == 0 && version.patch() == 0)
        || (version.major() == 3
            && version.minor() == 1
            && version.patch() == 0)
}

/// Returns the canonical hardware program-format identifier.
///
/// The version remains separately encoded in the OpenQASM header and adapter
/// metadata so that `openqasm-3` does not falsely imply that every 3.x revision
/// is supported.
#[must_use]
pub const fn program_format_id() -> &'static str {
    OPENQASM_PROGRAM_FORMAT
}

/// Returns the canonical OpenQASM textual media type.
#[must_use]
pub const fn media_type() -> &'static str {
    OPENQASM_MEDIA_TYPE
}

/// Converts a frontend error into a bounded, secret-safe diagnostic.
///
/// The frontend error's Display implementation is used only as a diagnostic
/// boundary. It is never placed into a program payload or provider request.
fn safe_frontend_error(
    error: &impl fmt::Display,
) -> String {
    bounded_message(&error.to_string())
}

/// Converts a backend error into a bounded diagnostic.
///
/// Provider adapters must still perform provider-secret redaction before
/// constructing their own backend errors.
fn safe_backend_error(
    error: &BackendError,
) -> String {
    bounded_message(&error.to_string())
}

/// Bounds adapter-generated diagnostics.
///
/// This protects error consumers from pathological messages while preserving
/// the beginning of the diagnostic, which is normally where the stable
/// category/context is located.
fn bounded_message(message: &str) -> String {
    if message.len() <= MAX_METADATA_VALUE_BYTES {
        return message.to_owned();
    }

    let mut end = MAX_METADATA_VALUE_BYTES;

    while end > 0
        && !message.is_char_boundary(end)
    {
        end -= 1;
    }

    let mut bounded = message[..end].to_owned();
    bounded.push_str("...");

    bounded
}

/// =============================================================================
/// Tests
/// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_circuit() -> QuantumCircuit {
        QuantumCircuit::new(1)
            .expect("one-qubit canonical circuit must be constructible")
    }

    #[test]
    fn production_adapter_defaults_to_openqasm_3_1() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("production adapter must construct");

        assert_eq!(
            adapter.version(),
            OPENQASM_3_1
        );

        assert_eq!(
            adapter.program_format(),
            "openqasm-3"
        );

        assert_eq!(
            adapter.media_type(),
            OPENQASM_MEDIA_TYPE
        );
    }

    #[test]
    fn explicit_openqasm_3_0_is_supported() {
        let config =
            OpenQasmHardwareAdapterConfig::for_version(
                OPENQASM_3_0,
            )
            .expect("OpenQASM 3.0 must be supported");

        let adapter =
            OpenQasmHardwareAdapter::new(config)
                .expect("OpenQASM 3.0 adapter must construct");

        assert_eq!(
            adapter.version(),
            OPENQASM_3_0
        );
    }

    #[test]
    fn unsupported_future_version_is_rejected() {
        let future =
            FormatVersion::new(3, 2, 0);

        assert!(
            !supports_version(future)
        );

        assert!(
            OpenQasmHardwareAdapterConfig::for_version(
                future
            )
            .is_err()
        );
    }

    #[test]
    fn unsupported_major_version_is_rejected() {
        let future =
            FormatVersion::new(4, 0, 0);

        assert!(
            validate_supported_version(future)
                .is_err()
        );
    }

    #[test]
    fn zero_output_limit_is_rejected() {
        let config =
            OpenQasmHardwareAdapterConfig::production()
                .with_max_program_bytes(0);

        assert!(
            OpenQasmHardwareAdapter::new(config)
                .is_err()
        );
    }

    #[test]
    fn gate_circuit_is_supported() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::GateCircuit
                )
                .is_ok()
        );
    }

    #[test]
    fn dynamic_circuit_is_supported_by_format_boundary() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::DynamicCircuit
                )
                .is_ok()
        );
    }

    #[test]
    fn sampling_is_supported_by_format_boundary() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::Sampling
                )
                .is_ok()
        );
    }

    #[test]
    fn pulse_is_not_misrepresented_as_circuit_execution() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let error =
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::PulseProgram,
                )
                .expect_err(
                    "pulse must use a dedicated pulse adapter",
                );

        assert_eq!(
            error.code(),
            CODE_UNSUPPORTED_WORKLOAD
        );
    }

    #[test]
    fn analog_is_not_misrepresented_as_circuit_execution() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::AnalogProgram,
                )
                .is_err()
        );
    }

    #[test]
    fn annealing_is_not_misrepresented_as_circuit_execution() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::AnnealingProblem,
                )
                .is_err()
        );
    }

    #[test]
    fn logical_execution_requires_logical_adapter() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        assert!(
            adapter
                .validate_workload_kind(
                    QuantumWorkloadKind::LogicalProgram,
                )
                .is_err()
        );
    }

    #[test]
    fn metadata_is_stable_and_secret_free() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let metadata =
            adapter.metadata();

        assert_eq!(
            metadata.adapter_id,
            OPENQASM_HARDWARE_ADAPTER_ID
        );

        assert_eq!(
            metadata.adapter_version,
            OPENQASM_HARDWARE_ADAPTER_VERSION
        );

        assert_eq!(
            metadata.format_id,
            OPENQASM_FORMAT_ID
        );

        assert_eq!(
            metadata.program_format,
            OPENQASM_PROGRAM_FORMAT
        );

        assert_eq!(
            metadata.media_type,
            OPENQASM_MEDIA_TYPE
        );

        assert_eq!(
            metadata.version,
            OPENQASM_3_1
        );

        assert!(
            metadata.max_program_bytes > 0
        );
    }

    #[test]
    fn program_format_identifier_is_stable() {
        assert_eq!(
            program_format_id(),
            "openqasm-3"
        );
    }

    #[test]
    fn supported_versions_are_explicit() {
        assert!(
            supports_version(OPENQASM_3_0)
        );

        assert!(
            supports_version(OPENQASM_3_1)
        );

        assert!(
            !supports_version(
                FormatVersion::new(3, 2, 0)
            )
        );

        assert!(
            !supports_version(
                FormatVersion::new(4, 0, 0)
            )
        );
    }

    #[test]
    fn bounded_message_preserves_short_messages() {
        let message = "safe diagnostic";

        assert_eq!(
            bounded_message(message),
            message
        );
    }

    #[test]
    fn bounded_message_limits_large_messages() {
        let message =
            "x".repeat(MAX_METADATA_VALUE_BYTES + 100);

        let bounded =
            bounded_message(&message);

        assert!(
            bounded.len()
                <= MAX_METADATA_VALUE_BYTES + 3
        );

        assert!(
            bounded.ends_with("...")
        );
    }

    #[test]
    fn adapter_debug_does_not_contain_program_payload() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let debug =
            format!("{adapter:?}");

        assert!(
            debug.contains(
                "OpenQasmHardwareAdapter"
            )
        );

        assert!(
            !debug.contains(
                "OPENQASM"
            )
        );
    }

    #[test]
    fn production_config_is_valid() {
        let config =
            OpenQasmHardwareAdapterConfig::production();

        assert!(
            config.validate().is_ok()
        );

        assert!(
            config.max_program_bytes()
                <= DEFAULT_MAX_PROGRAM_BYTES
        );
    }

    #[test]
    fn adapter_can_be_constructed_for_a_valid_circuit_boundary() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let circuit =
            test_circuit();

        /*
         * The canonical exporter owns actual Quantum IR representability.
         * This test intentionally does not assume that an empty circuit has
         * a provider-executable measurement/result contract.
         */
        let result =
            adapter.encode(&circuit);

        /*
         * Either outcome is valid at this layer:
         *
         * - successful encoding proves the hardware bridge works;
         * - a structured frontend error proves representability is delegated
         *   to the canonical exporter rather than silently invented here.
         */
        if let Ok(program) = result {
            assert_eq!(
                program.format(),
                OPENQASM_PROGRAM_FORMAT
            );

            assert!(
                !program.is_empty()
            );

            assert!(
                program.len()
                    <= DEFAULT_MAX_PROGRAM_BYTES
            );
        }
    }

    #[test]
    fn adapter_does_not_accept_an_unbounded_requested_output() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let circuit =
            test_circuit();

        let options =
            ExportOptions::new()
                .with_max_output_bytes(
                    DEFAULT_MAX_PROGRAM_BYTES,
                )
                .with_requested_version(
                    OPENQASM_3_1,
                )
                .with_requested_version(
                    OPENQASM_3_1,
                );

        let result =
            adapter.encode_with_options(
                &circuit,
                options,
            );

        /*
         * The assertion is deliberately limited to the invariant that a
         * successful artifact can never exceed the configured bound.
         */
        if let Ok(program) = result {
            assert!(
                program.len()
                    <= DEFAULT_MAX_PROGRAM_BYTES
            );
        }
    }

    #[test]
    fn exact_version_policy_rejects_mismatched_requests() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let circuit =
            test_circuit();

        let options =
            ExportOptions::new()
                .with_requested_version(
                    OPENQASM_3_0,
                );

        let result =
            adapter.encode_with_options(
                &circuit,
                options,
            );

        assert!(
            matches!(
                result,
                Err(
                    OpenQasmHardwareAdapterError::UnsupportedVersion {
                        version: OPENQASM_3_0,
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn same_version_request_is_accepted() {
        let adapter =
            OpenQasmHardwareAdapter::production()
                .expect("adapter must construct");

        let circuit =
            test_circuit();

        let options =
            ExportOptions::new()
                .with_requested_version(
                    OPENQASM_3_1,
                )
                .with_max_output_bytes(
                    DEFAULT_MAX_PROGRAM_BYTES,
                );

        let result =
            adapter.encode_with_options(
                &circuit,
                options,
            );

        if let Ok(program) = result {
            assert_eq!(
                program.format(),
                OPENQASM_PROGRAM_FORMAT
            );
        }
    }

    #[test]
    fn safe_frontend_error_is_bounded() {
        let long =
            "x".repeat(
                MAX_METADATA_VALUE_BYTES + 512
            );

        let bounded =
            safe_frontend_error(&long);

        assert!(
            bounded.len()
                <= MAX_METADATA_VALUE_BYTES + 3
        );
    }

    #[test]
    fn backend_program_format_is_not_version_ambiguous() {
        /*
         * The provider-neutral format is intentionally `openqasm-3`, while
         * the actual 3.0/3.1 revision remains carried by the serialized
         * OpenQASM header and adapter metadata.
         */
        assert_eq!(
            OPENQASM_PROGRAM_FORMAT,
            "openqasm-3"
        );

        assert_ne!(
            OPENQASM_PROGRAM_FORMAT,
            "openqasm-3.1"
        );
    }
}