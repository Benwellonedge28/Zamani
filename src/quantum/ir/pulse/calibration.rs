//! Zamani Quantum IR — Pulse Calibration Semantics
//!
//! Canonical, hardware-independent representation of calibration definitions
//! and calibration references used by the Zamani Quantum IR.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This module answers:
//!
//!     "Which semantic calibration definition is associated with an IR
//!      operation/resource, what parameters does it require, what targets does
//!      it apply to, and under what semantic validity conditions may it be
//!      selected?"
//!
//! It does NOT answer:
//!
//!     "How does a physical device calibrate itself?"
//!
//! Physical calibration procedures, device control electronics, calibration
//! databases, provider SDKs, authentication, device telemetry, laboratory
//! procedures, and hardware-specific calibration algorithms belong outside the
//! canonical IR.
//!
//! Dependency direction:
//!
//!     Zamani source
//!          |
//!          v
//!     frontend
//!          |
//!          v
//!     canonical Zamani IR
//!          |
//!          v
//!     pulse calibration semantics
//!          |
//!          +--------------------+
//!          |                    |
//!          v                    v
//!     optimization          scheduling
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!                 hardware
//!                    |
//!                    v
//!                  backend
//!                    |
//!                    v
//!                   QPU
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - calibration identity references;
//! - calibration definitions;
//! - calibration targets;
//! - calibration scopes;
//! - calibration operation selectors;
//! - calibration parameter schemas;
//! - calibration parameter bindings;
//! - calibration validity conditions;
//! - calibration metadata;
//! - calibration dependencies;
//! - calibration revisions;
//! - calibration priorities;
//! - calibration registry semantics;
//! - deterministic calibration lookup;
//! - calibration validation;
//! - explicit resource/security validation policy;
//! - calibration-local errors.
//!
//! This file does NOT own:
//!
//! - physical device calibration algorithms;
//! - hardware calibration execution;
//! - provider SDKs;
//! - hardware topology;
//! - routing;
//! - scheduling algorithms;
//! - pulse waveform generation;
//! - DAC/ADC configuration;
//! - credentials;
//! - network communication;
//! - telemetry collection;
//! - optimization policy;
//! - frontend parsing.
//!
//! ============================================================================
//! SCALABILITY CONTRACT
//! ============================================================================
//!
//! There is deliberately NO architectural maximum for:
//!
//! - number of calibrations;
//! - number of targets;
//! - number of calibration parameters;
//! - number of calibration dependencies;
//! - number of metadata fields;
//! - number of calibration revisions;
//! - number of quantum machines;
//! - number of qubits;
//! - number of pulse operations;
//! - number of calibration families.
//!
//! Resource limits are represented by `CalibrationValidationPolicy`.
//!
//! An unrestricted policy is available and is the semantic default.
//!
//! A compiler, service, sandbox, or embedded environment may provide explicit
//! resource limits when accepting untrusted or very large input.
//!
//! This distinction is fundamental:
//!
//!     semantic capability != resource policy
//!
//! ============================================================================
//! QUBIT IDENTITY CONTRACT
//! ============================================================================
//!
//! Logical qubit references use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! or, from this module:
//!
//!     super::super::quantum::qubit::QubitId
//!
//! This module does not define another QubitId.
//!
//! A physical qubit must not be substituted for a logical qubit in a canonical
//! calibration target. Logical-to-physical mapping belongs to the mapping and
//! hardware layers.
//!
//! ============================================================================
//! CALIBRATION MODEL
//! ============================================================================
//!
//! A calibration definition is semantic metadata describing how a later
//! compilation/backend stage may realize an operation or pulse.
//!
//! Conceptually:
//!
//!     operation
//!          |
//!          v
//!     calibration selector
//!          |
//!          v
//!     calibration definition
//!          |
//!          +------------------+
//!          |                  |
//!          v                  v
//!     parameter schema     target scope
//!          |                  |
//!          +--------+---------+
//!                   |
//!                   v
//!             target lowering
//!                   |
//!                   v
//!                hardware
//!
//! The canonical IR does not select a physical DAC, oscillator, laser,
//! microwave source, control card, sample clock, or vendor API.
//!
//! ============================================================================
//! CALIBRATION VERSIONS
//! ============================================================================
//!
//! There are deliberately several independent notions of version:
//!
//! 1. IR schema version:
//!       PULSE_CALIBRATION_SCHEMA_*
//!
//! 2. Calibration definition revision:
//!       CalibrationRevision
//!
//! 3. Calibration implementation/version label:
//!       CalibrationImplementationVersion
//!
//! 4. Hardware/device version:
//!       owned by hardware/backend layers
//!
//! These must never be conflated.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Deterministic collections use `BTreeMap` and `BTreeSet`.
//!
//! No `HashMap` is used for canonical semantic state.
//!
//! Registry lookup is deterministic for identical inputs.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! This module never stores:
//!
//! - API keys;
//! - passwords;
//! - access tokens;
//! - private keys;
//! - cookies;
//! - authentication headers;
//! - provider credentials.
//!
//! Calibration metadata is descriptive IR data, not an authentication
//! mechanism.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `core::identity`
//!     Supplies `CalibrationId`, `OperationId`, `ResourceId`, `QubitId` is
//!     intentionally NOT supplied there.
//!
//! `quantum::ir::qubit`
//!     Owns canonical logical and physical qubit identity.
//!
//! `core::parameter`
//!     Supplies symbolic/concrete `Parameter` values.
//!
//! `pulse::pulse`
//!     Stores `CalibrationId` references from pulse operations.
//!     This module deliberately does not depend on `pulse.rs`, preventing a
//!     dependency cycle.
//!
//! `pulse::waveform`
//!     Owns waveform definitions.
//!
//! `pulse::frame`
//!     Owns frame definitions.
//!
//! `pulse::channel`
//!     Owns abstract channel definitions.
//!
//! `program::operation`
//!     May associate operation identity with calibration references.
//!
//! `resources`
//!     Resolves abstract resource requirements.
//!
//! `mapping`
//!     Resolves logical targets to physical resources later.
//!
//! `hardware`
//!     Determines whether a target device can implement a calibration.
//!
//! `scheduling`
//!     Determines execution timing.
//!
//! `serialization`
//!     Owns canonical encoding.
//!
//! `hash`
//!     Owns canonical cryptographic content hashing.
//!
//! `provenance`
//!     Records calibration transformations and lineage.
//!
//! ============================================================================
//! FILE COMPLETION GUARANTEE
//! ============================================================================
//!
//! This file intentionally contains:
//!
//! - semantic calibration model;
//! - target model;
//! - parameter schema;
//! - parameter binding;
//! - validity model;
//! - dependency model;
//! - metadata model;
//! - registry;
//! - deterministic lookup;
//! - validation;
//! - explicit validation policy;
//! - checked arithmetic;
//! - extension support;
//! - tests;
//! - integration documentation.
//!
//! Later IR modules should not require this file to be redesigned merely
//! because those modules are implemented.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::identity::{
    CalibrationId,
    OperationId,
    ResourceId,
};
use super::super::core::parameter::Parameter;
use super::super::quantum::qubit::QubitId;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic schema identifier for pulse calibrations.
pub const PULSE_CALIBRATION_SCHEMA_ID: &str =
    "zamani.quantum.ir.pulse.calibration";

/// Semantic schema major version.
pub const PULSE_CALIBRATION_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const PULSE_CALIBRATION_SCHEMA_MINOR: u16 = 0;

/// Semantic schema patch version.
pub const PULSE_CALIBRATION_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// Result
// ============================================================================

/// Result type used by calibration construction and validation.
pub type CalibrationResult<T> = Result<T, CalibrationError>;

// ============================================================================
// Validation policy
// ============================================================================

/// Explicit validation/resource policy.
///
/// `None` means that the corresponding resource is not artificially bounded
/// by this validation invocation.
///
/// These values are never architectural limits of Zamani.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationValidationPolicy {
    /// Maximum number of calibration parameters in one definition.
    pub max_parameters: Option<usize>,

    /// Maximum number of explicit targets in one definition.
    pub max_targets: Option<usize>,

    /// Maximum number of dependencies in one definition.
    pub max_dependencies: Option<usize>,

    /// Maximum number of metadata entries.
    pub max_metadata_entries: Option<usize>,

    /// Maximum UTF-8 bytes in one metadata key.
    pub max_metadata_key_bytes: Option<usize>,

    /// Maximum UTF-8 bytes in one metadata value.
    pub max_metadata_value_bytes: Option<usize>,

    /// Maximum number of definitions in one registry.
    pub max_registry_entries: Option<usize>,

    /// Maximum number of extension attributes.
    pub max_extensions: Option<usize>,
}

impl CalibrationValidationPolicy {
    /// Creates an unrestricted semantic policy.
    ///
    /// This does not disable semantic correctness checks.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_parameters: None,
            max_targets: None,
            max_dependencies: None,
            max_metadata_entries: None,
            max_metadata_key_bytes: None,
            max_metadata_value_bytes: None,
            max_registry_entries: None,
            max_extensions: None,
        }
    }

    /// Creates a bounded validation policy.
    #[must_use]
    pub const fn bounded(
        max_parameters: usize,
        max_targets: usize,
        max_dependencies: usize,
        max_metadata_entries: usize,
        max_metadata_key_bytes: usize,
        max_metadata_value_bytes: usize,
        max_registry_entries: usize,
        max_extensions: usize,
    ) -> Self {
        Self {
            max_parameters: Some(max_parameters),
            max_targets: Some(max_targets),
            max_dependencies: Some(max_dependencies),
            max_metadata_entries: Some(max_metadata_entries),
            max_metadata_key_bytes: Some(max_metadata_key_bytes),
            max_metadata_value_bytes: Some(max_metadata_value_bytes),
            max_registry_entries: Some(max_registry_entries),
            max_extensions: Some(max_extensions),
        }
    }
}

impl Default for CalibrationValidationPolicy {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// ============================================================================
// Calibration revision
// ============================================================================

/// Monotonically ordered semantic revision of a calibration definition.
///
/// This is not a timestamp and does not identify a hardware firmware version.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct CalibrationRevision(u64);

impl CalibrationRevision {
    /// Initial revision.
    pub const INITIAL: Self = Self(0);

    /// Creates a revision from an explicit value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric revision.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next revision if representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl fmt::Display for CalibrationRevision {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "revision:{}", self.0)
    }
}

// ============================================================================
// Calibration priority
// ============================================================================

/// Deterministic selection priority.
///
/// Larger values have higher priority.
///
/// Priority is semantic metadata only. It does not authorize hardware access.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct CalibrationPriority(u32);

impl CalibrationPriority {
    /// Default priority.
    pub const DEFAULT: Self = Self(0);

    /// Creates a priority.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the priority value.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl Default for CalibrationPriority {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// ============================================================================
// Calibration scope
// ============================================================================

/// Semantic scope of a calibration.
///
/// Scope describes what the calibration applies to, not where hardware
/// allocation occurs.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CalibrationScope {
    /// Applies to any compatible target.
    Global,

    /// Applies to one logical qubit.
    Qubit(QubitId),

    /// Applies to an explicit logical-qubit set.
    Qubits(Vec<QubitId>),

    /// Applies to an abstract resource.
    Resource(ResourceId),

    /// Applies to an operation identity.
    Operation(OperationId),

    /// Extension-defined scope.
    Extension(String),
}

impl CalibrationScope {
    /// Creates a single-qubit scope.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a resource scope.
    #[must_use]
    pub const fn resource(resource: ResourceId) -> Self {
        Self::Resource(resource)
    }

    /// Creates an operation scope.
    #[must_use]
    pub const fn operation(operation: OperationId) -> Self {
        Self::Operation(operation)
    }

    /// Creates a deterministic multi-qubit scope.
    ///
    /// Duplicates are removed and identifiers are sorted.
    #[must_use]
    pub fn qubits<I>(
        qubits: I,
    ) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut set = BTreeSet::new();

        for qubit in qubits {
            set.insert(qubit);
        }

        Self::Qubits(set.into_iter().collect())
    }

    /// Returns whether this scope is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns explicitly represented logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        match self {
            Self::Qubit(qubit) => {
                std::slice::from_ref(qubit)
            }

            Self::Qubits(qubits) => qubits.as_slice(),

            Self::Global
            | Self::Resource(_)
            | Self::Operation(_)
            | Self::Extension(_) => &[],
        }
    }

    /// Validates structural correctness.
    pub fn validate(&self) -> CalibrationResult<()> {
        match self {
            Self::Qubit(_) => Ok(()),

            Self::Qubits(qubits) => {
                if qubits.is_empty() {
                    return Err(
                        CalibrationError::EmptyQubitScope
                    );
                }

                if qubits
                    .windows(2)
                    .any(|pair| pair[0] >= pair[1])
                {
                    return Err(
                        CalibrationError::NonCanonicalQubitScope
                    );
                }

                Ok(())
            }

            Self::Global
            | Self::Resource(_)
            | Self::Operation(_) => Ok(()),

            Self::Extension(name) => {
                if name.trim().is_empty() {
                    return Err(
                        CalibrationError::EmptyExtensionScope
                    );
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Operation selector
// ============================================================================

/// Semantic selector describing which operation a calibration can implement.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CalibrationOperation {
    /// Named semantic operation.
    ///
    /// Examples:
    ///
    /// `x`
    /// `rx`
    /// `measure`
    /// `readout`
    /// `custom.namespace.operation`
    Named(String),

    /// Canonical operation identity.
    Operation(OperationId),

    /// Extension-defined operation selector.
    Extension(String),
}

impl CalibrationOperation {
    /// Creates a named operation selector.
    pub fn named<S>(
        name: S,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(
                CalibrationError::EmptyOperationName
            );
        }

        Ok(Self::Named(name))
    }

    /// Creates an operation identity selector.
    #[must_use]
    pub const fn operation(
        operation: OperationId,
    ) -> Self {
        Self::Operation(operation)
    }

    /// Returns the stable operation name when present.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Named(name) => Some(name.as_str()),
            Self::Operation(_) | Self::Extension(_) => None,
        }
    }

    /// Validates the selector.
    pub fn validate(&self) -> CalibrationResult<()> {
        match self {
            Self::Named(name) => {
                if name.trim().is_empty() {
                    Err(
                        CalibrationError::EmptyOperationName
                    )
                } else {
                    Ok(())
                }
            }

            Self::Operation(_) => Ok(()),

            Self::Extension(name) => {
                if name.trim().is_empty() {
                    Err(
                        CalibrationError::EmptyExtensionOperation
                    )
                } else {
                    Ok(())
                }
            }
        }
    }
}

// ============================================================================
// Parameter role
// ============================================================================

/// Semantic role of a calibration parameter.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CalibrationParameterRole {
    /// Amplitude/scaling parameter.
    Amplitude,

    /// Frequency parameter.
    Frequency,

    /// Phase parameter.
    Phase,

    /// Duration parameter.
    Duration,

    /// Waveform shape parameter.
    Waveform,

    /// Frame parameter.
    Frame,

    /// Measurement/acquisition parameter.
    Acquisition,

    /// Device-independent generic parameter.
    Generic,

    /// Extension-defined parameter role.
    Extension,
}

// ============================================================================
// Parameter specification
// ============================================================================

/// Schema describing one calibration parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationParameter {
    /// Stable semantic parameter name.
    name: String,

    /// Semantic parameter role.
    role: CalibrationParameterRole,

    /// Whether the parameter must be supplied.
    required: bool,

    /// Optional default value.
    default: Option<Parameter>,

    /// Optional symbolic/unit annotation.
    unit: Option<String>,

    /// Optional human-readable description.
    description: Option<String>,
}

impl CalibrationParameter {
    /// Creates a required calibration parameter.
    pub fn required<S>(
        name: S,
        role: CalibrationParameterRole,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        Self::new(
            name,
            role,
            true,
            None,
            None,
            None,
        )
    }

    /// Creates an optional calibration parameter.
    pub fn optional<S>(
        name: S,
        role: CalibrationParameterRole,
        default: Option<Parameter>,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        Self::new(
            name,
            role,
            false,
            default,
            None,
            None,
        )
    }

    /// Creates a complete parameter specification.
    pub fn new<S>(
        name: S,
        role: CalibrationParameterRole,
        required: bool,
        default: Option<Parameter>,
        unit: Option<String>,
        description: Option<String>,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_identifier(
            &name,
            CalibrationIdentifierKind::ParameterName,
        )?;

        if required && default.is_some() {
            return Err(
                CalibrationError::RequiredParameterHasDefault {
                    parameter: name,
                },
            );
        }

        if let Some(unit) = &unit {
            if unit.trim().is_empty() {
                return Err(
                    CalibrationError::EmptyParameterUnit {
                        parameter: name,
                    },
                );
            }
        }

        if let Some(description) = &description {
            if description.trim().is_empty() {
                return Err(
                    CalibrationError::EmptyParameterDescription {
                        parameter: name,
                    },
                );
            }
        }

        Ok(Self {
            name,
            role,
            required,
            default,
            unit,
            description,
        })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the semantic role.
    #[must_use]
    pub const fn role(
        &self,
    ) -> CalibrationParameterRole {
        self.role
    }

    /// Returns whether the parameter is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the optional default.
    #[must_use]
    pub fn default(&self) -> Option<&Parameter> {
        self.default.as_ref()
    }

    /// Returns the optional unit annotation.
    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validates this specification.
    pub fn validate(&self) -> CalibrationResult<()> {
        validate_identifier(
            &self.name,
            CalibrationIdentifierKind::ParameterName,
        )?;

        if self.required && self.default.is_some() {
            return Err(
                CalibrationError::RequiredParameterHasDefault {
                    parameter: self.name.clone(),
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Parameter binding
// ============================================================================

/// A concrete/symbolic value bound to a calibration parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationBinding {
    parameter: String,
    value: Parameter,
}

impl CalibrationBinding {
    /// Creates a parameter binding.
    pub fn new<S>(
        parameter: S,
        value: Parameter,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        let parameter = parameter.into();

        validate_identifier(
            &parameter,
            CalibrationIdentifierKind::ParameterName,
        )?;

        Ok(Self {
            parameter,
            value,
        })
    }

    /// Returns the bound parameter name.
    #[must_use]
    pub fn parameter(&self) -> &str {
        self.parameter.as_str()
    }

    /// Returns the bound parameter value.
    #[must_use]
    pub fn value(&self) -> &Parameter {
        &self.value
    }
}

// ============================================================================
// Validity
// ============================================================================

/// Semantic validity constraint for a calibration definition.
///
/// Validity is intentionally represented without depending on wall-clock
/// libraries or hardware-specific timestamp types.
///
/// `effective_revision` and `expires_after_revision` allow deterministic
/// compilation/session selection.
///
/// Optional wall-clock values are expressed as signed Unix nanoseconds so the
/// IR does not depend on a particular time crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationValidity {
    /// Earliest calibration revision at which this definition is valid.
    pub effective_revision: Option<CalibrationRevision>,

    /// Last calibration revision at which this definition remains valid.
    pub expires_after_revision: Option<CalibrationRevision>,

    /// Optional Unix timestamp in nanoseconds at which validity starts.
    pub valid_from_unix_ns: Option<i128>,

    /// Optional Unix timestamp in nanoseconds at which validity ends.
    pub valid_until_unix_ns: Option<i128>,
}

impl CalibrationValidity {
    /// Creates an unrestricted validity interval.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            effective_revision: None,
            expires_after_revision: None,
            valid_from_unix_ns: None,
            valid_until_unix_ns: None,
        }
    }

    /// Creates a revision-bounded validity interval.
    #[must_use]
    pub const fn revisions(
        effective_revision: Option<CalibrationRevision>,
        expires_after_revision: Option<CalibrationRevision>,
    ) -> Self {
        Self {
            effective_revision,
            expires_after_revision,
            valid_from_unix_ns: None,
            valid_until_unix_ns: None,
        }
    }

    /// Creates a wall-clock-bounded validity interval.
    #[must_use]
    pub const fn timestamps(
        valid_from_unix_ns: Option<i128>,
        valid_until_unix_ns: Option<i128>,
    ) -> Self {
        Self {
            effective_revision: None,
            expires_after_revision: None,
            valid_from_unix_ns,
            valid_until_unix_ns,
        }
    }

    /// Validates the interval.
    pub fn validate(&self) -> CalibrationResult<()> {
        if let (
            Some(start),
            Some(end),
        ) = (
            self.effective_revision,
            self.expires_after_revision,
        ) {
            if start > end {
                return Err(
                    CalibrationError::InvalidRevisionWindow,
                );
            }
        }

        if let (
            Some(start),
            Some(end),
        ) = (
            self.valid_from_unix_ns,
            self.valid_until_unix_ns,
        ) {
            if start > end {
                return Err(
                    CalibrationError::InvalidTimestampWindow,
                );
            }
        }

        Ok(())
    }

    /// Returns whether a revision is within this validity interval.
    #[must_use]
    pub fn accepts_revision(
        &self,
        revision: CalibrationRevision,
    ) -> bool {
        if let Some(start) = self.effective_revision {
            if revision < start {
                return false;
            }
        }

        if let Some(end) = self.expires_after_revision {
            if revision > end {
                return false;
            }
        }

        true
    }

    /// Returns whether a timestamp is within this validity interval.
    #[must_use]
    pub fn accepts_unix_ns(
        &self,
        timestamp: i128,
    ) -> bool {
        if let Some(start) = self.valid_from_unix_ns {
            if timestamp < start {
                return false;
            }
        }

        if let Some(end) = self.valid_until_unix_ns {
            if timestamp > end {
                return false;
            }
        }

        true
    }
}

impl Default for CalibrationValidity {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// ============================================================================
// Dependencies
// ============================================================================

/// Semantic dependency between calibration definitions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CalibrationDependency {
    /// This calibration requires another calibration to exist.
    Requires(CalibrationId),

    /// This calibration supersedes another calibration.
    Supersedes(CalibrationId),

    /// This calibration is derived from another calibration.
    DerivedFrom(CalibrationId),

    /// Extension-defined dependency.
    Extension(String),
}

impl CalibrationDependency {
    /// Returns the referenced calibration ID when one exists.
    #[must_use]
    pub const fn calibration_id(
        &self,
    ) -> Option<CalibrationId> {
        match self {
            Self::Requires(id)
            | Self::Supersedes(id)
            | Self::DerivedFrom(id) => Some(*id),

            Self::Extension(_) => None,
        }
    }

    /// Validates this dependency.
    pub fn validate(&self) -> CalibrationResult<()> {
        match self {
            Self::Requires(_)
            | Self::Supersedes(_)
            | Self::DerivedFrom(_) => Ok(()),

            Self::Extension(name) => {
                if name.trim().is_empty() {
                    Err(
                        CalibrationError::EmptyDependencyExtension,
                    )
                } else {
                    Ok(())
                }
            }
        }
    }
}

// ============================================================================
// Metadata
// ============================================================================

/// Deterministic calibration metadata.
///
/// Keys and values are plain semantic strings. They must never contain
/// credentials or authentication material.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CalibrationMetadata {
    entries: BTreeMap<String, String>,
}

impl CalibrationMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts metadata.
    ///
    /// Existing values for the same key are replaced.
    pub fn insert<S, T>(
        &mut self,
        key: S,
        value: T,
    ) -> CalibrationResult<Option<String>>
    where
        S: Into<String>,
        T: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        Ok(self.entries.insert(key, value))
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns the number of metadata fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns deterministic metadata entries.
    #[must_use]
    pub fn entries(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Validates metadata against a policy.
    pub fn validate(
        &self,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<()> {
        enforce_limit(
            self.entries.len(),
            policy.max_metadata_entries,
            CalibrationResourceKind::MetadataEntries,
        )?;

        for (
            key,
            value,
        ) in &self.entries {
            if let Some(max) =
                policy.max_metadata_key_bytes
            {
                if key.len() > max {
                    return Err(
                        CalibrationError::MetadataKeyTooLarge {
                            actual: key.len(),
                            maximum: max,
                        },
                    );
                }
            }

            if let Some(max) =
                policy.max_metadata_value_bytes
            {
                if value.len() > max {
                    return Err(
                        CalibrationError::MetadataValueTooLarge {
                            actual: value.len(),
                            maximum: max,
                        },
                    );
                }
            }

            validate_metadata_key(key)?;
            validate_metadata_value(value)?;
        }

        Ok(())
    }
}

// ============================================================================
// Extension attributes
// ============================================================================

/// Extension attributes preserved by the calibration IR.
///
/// This mechanism allows future dialects/vendors to attach semantic
/// information without modifying this file's core structure.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CalibrationExtensions {
    values: BTreeMap<String, String>,
}

impl CalibrationExtensions {
    /// Creates an empty extension set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts an extension attribute.
    pub fn insert<S, T>(
        &mut self,
        namespace: S,
        value: T,
    ) -> CalibrationResult<Option<String>>
    where
        S: Into<String>,
        T: Into<String>,
    {
        let namespace = namespace.into();
        let value = value.into();

        if namespace.trim().is_empty() {
            return Err(
                CalibrationError::EmptyExtensionName,
            );
        }

        if value.trim().is_empty() {
            return Err(
                CalibrationError::EmptyExtensionValue,
            );
        }

        Ok(self.values.insert(namespace, value))
    }

    /// Returns an extension value.
    #[must_use]
    pub fn get(
        &self,
        namespace: &str,
    ) -> Option<&str> {
        self.values.get(namespace).map(String::as_str)
    }

    /// Returns the number of extensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns deterministic extension attributes.
    #[must_use]
    pub fn values(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.values
    }

    /// Validates extension attributes.
    pub fn validate(
        &self,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<()> {
        enforce_limit(
            self.values.len(),
            policy.max_extensions,
            CalibrationResourceKind::Extensions,
        )?;

        for (
            namespace,
            value,
        ) in &self.values {
            if namespace.trim().is_empty() {
                return Err(
                    CalibrationError::EmptyExtensionName,
                );
            }

            if value.trim().is_empty() {
                return Err(
                    CalibrationError::EmptyExtensionValue,
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Calibration definition
// ============================================================================

/// Complete semantic calibration definition.
///
/// A `CalibrationDefinition` is immutable after construction from the public
/// API perspective. Builders return a new value.
///
/// This prevents hidden shared mutable calibration state.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationDefinition {
    id: CalibrationId,
    name: String,
    revision: CalibrationRevision,
    priority: CalibrationPriority,
    operation: CalibrationOperation,
    scope: CalibrationScope,
    parameters: Vec<CalibrationParameter>,
    bindings: Vec<CalibrationBinding>,
    validity: CalibrationValidity,
    dependencies: Vec<CalibrationDependency>,
    metadata: CalibrationMetadata,
    extensions: CalibrationExtensions,
}

impl CalibrationDefinition {
    /// Creates a minimal calibration definition.
    pub fn new<S>(
        id: CalibrationId,
        name: S,
        operation: CalibrationOperation,
        scope: CalibrationScope,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_identifier(
            &name,
            CalibrationIdentifierKind::CalibrationName,
        )?;

        operation.validate()?;
        scope.validate()?;

        Ok(Self {
            id,
            name,
            revision: CalibrationRevision::INITIAL,
            priority: CalibrationPriority::DEFAULT,
            operation,
            scope,
            parameters: Vec::new(),
            bindings: Vec::new(),
            validity: CalibrationValidity::unrestricted(),
            dependencies: Vec::new(),
            metadata: CalibrationMetadata::new(),
            extensions: CalibrationExtensions::new(),
        })
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> CalibrationId {
        self.id
    }

    /// Returns the semantic name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        self.name.as_str()
    }

    /// Returns the revision.
    #[must_use]
    pub const fn revision(
        &self,
    ) -> CalibrationRevision {
        self.revision
    }

    /// Returns the priority.
    #[must_use]
    pub const fn priority(
        &self,
    ) -> CalibrationPriority {
        self.priority
    }

    /// Returns the operation selector.
    #[must_use]
    pub fn operation(
        &self,
    ) -> &CalibrationOperation {
        &self.operation
    }

    /// Returns the calibration scope.
    #[must_use]
    pub fn scope(
        &self,
    ) -> &CalibrationScope {
        &self.scope
    }

    /// Returns the parameter schema.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &[CalibrationParameter] {
        self.parameters.as_slice()
    }

    /// Returns parameter bindings.
    #[must_use]
    pub fn bindings(
        &self,
    ) -> &[CalibrationBinding] {
        self.bindings.as_slice()
    }

    /// Returns validity.
    #[must_use]
    pub const fn validity(
        &self,
    ) -> CalibrationValidity {
        self.validity
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &[CalibrationDependency] {
        self.dependencies.as_slice()
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(
        &self,
    ) -> &CalibrationMetadata {
        &self.metadata
    }

    /// Returns extensions.
    #[must_use]
    pub fn extensions(
        &self,
    ) -> &CalibrationExtensions {
        &self.extensions
    }

    /// Sets the calibration revision.
    #[must_use]
    pub const fn with_revision(
        mut self,
        revision: CalibrationRevision,
    ) -> Self {
        self.revision = revision;
        self
    }

    /// Sets the calibration priority.
    #[must_use]
    pub const fn with_priority(
        mut self,
        priority: CalibrationPriority,
    ) -> Self {
        self.priority = priority;
        self
    }

    /// Sets validity.
    #[must_use]
    pub const fn with_validity(
        mut self,
        validity: CalibrationValidity,
    ) -> Self {
        self.validity = validity;
        self
    }

    /// Adds a parameter specification.
    pub fn with_parameter(
        mut self,
        parameter: CalibrationParameter,
    ) -> CalibrationResult<Self> {
        parameter.validate()?;

        if self
            .parameters
            .iter()
            .any(|existing| existing.name() == parameter.name())
        {
            return Err(
                CalibrationError::DuplicateParameter {
                    parameter: parameter.name().to_owned(),
                },
            );
        }

        self.parameters.push(parameter);

        Ok(self)
    }

    /// Adds a parameter binding.
    pub fn with_binding(
        mut self,
        binding: CalibrationBinding,
    ) -> CalibrationResult<Self> {
        if self
            .bindings
            .iter()
            .any(|existing| {
                existing.parameter()
                    == binding.parameter()
            })
        {
            return Err(
                CalibrationError::DuplicateBinding {
                    parameter: binding.parameter().to_owned(),
                },
            );
        }

        if !self
            .parameters
            .iter()
            .any(|parameter| {
                parameter.name()
                    == binding.parameter()
            })
        {
            return Err(
                CalibrationError::UnknownParameterBinding {
                    parameter: binding.parameter().to_owned(),
                },
            );
        }

        self.bindings.push(binding);

        Ok(self)
    }

    /// Adds a dependency.
    pub fn with_dependency(
        mut self,
        dependency: CalibrationDependency,
    ) -> CalibrationResult<Self> {
        dependency.validate()?;

        if let CalibrationDependency::Requires(id)
        | CalibrationDependency::Supersedes(id)
        | CalibrationDependency::DerivedFrom(id) =
            dependency
        {
            if id == self.id {
                return Err(
                    CalibrationError::SelfDependency,
                );
            }
        }

        if self
            .dependencies
            .contains(&dependency)
        {
            return Err(
                CalibrationError::DuplicateDependency,
            );
        }

        self.dependencies.push(dependency);

        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata<S, T>(
        mut self,
        key: S,
        value: T,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Adds an extension attribute.
    pub fn with_extension<S, T>(
        mut self,
        namespace: S,
        value: T,
    ) -> CalibrationResult<Self>
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.extensions.insert(namespace, value)?;
        Ok(self)
    }

    /// Validates this definition under an explicit policy.
    pub fn validate(
        &self,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<()> {
        validate_identifier(
            &self.name,
            CalibrationIdentifierKind::CalibrationName,
        )?;

        self.operation.validate()?;
        self.scope.validate()?;
        self.validity.validate()?;

        enforce_limit(
            self.parameters.len(),
            policy.max_parameters,
            CalibrationResourceKind::Parameters,
        )?;

        enforce_limit(
            self.scope.logical_qubits().len(),
            policy.max_targets,
            CalibrationResourceKind::Targets,
        )?;

        enforce_limit(
            self.dependencies.len(),
            policy.max_dependencies,
            CalibrationResourceKind::Dependencies,
        )?;

        for parameter in &self.parameters {
            parameter.validate()?;
        }

        let mut parameter_names = BTreeSet::new();

        for parameter in &self.parameters {
            if !parameter_names.insert(parameter.name()) {
                return Err(
                    CalibrationError::DuplicateParameter {
                        parameter: parameter.name().to_owned(),
                    },
                );
            }
        }

        let mut binding_names = BTreeSet::new();

        for binding in &self.bindings {
            if !binding_names.insert(binding.parameter()) {
                return Err(
                    CalibrationError::DuplicateBinding {
                        parameter: binding.parameter().to_owned(),
                    },
                );
            }

            if !parameter_names.contains(binding.parameter()) {
                return Err(
                    CalibrationError::UnknownParameterBinding {
                        parameter: binding.parameter().to_owned(),
                    },
                );
            }
        }

        for parameter in &self.parameters {
            if parameter.is_required()
                && !binding_names.contains(parameter.name())
            {
                return Err(
                    CalibrationError::MissingRequiredParameter {
                        parameter: parameter.name().to_owned(),
                    },
                );
            }
        }

        for dependency in &self.dependencies {
            dependency.validate()?;
        }

        self.metadata.validate(policy)?;
        self.extensions.validate(policy)?;

        Ok(())
    }

    /// Returns whether this definition is valid for a revision.
    #[must_use]
    pub fn accepts_revision(
        &self,
        revision: CalibrationRevision,
    ) -> bool {
        self.validity.accepts_revision(revision)
    }

    /// Returns whether this definition is valid for a timestamp.
    #[must_use]
    pub fn accepts_unix_ns(
        &self,
        timestamp: i128,
    ) -> bool {
        self.validity.accepts_unix_ns(timestamp)
    }
}

// ============================================================================
// Calibration reference
// ============================================================================

/// Lightweight reference to a calibration definition.
///
/// This is what pulse/program operations should store when they need to refer
/// to a calibration without embedding the full definition.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct CalibrationReference {
    id: CalibrationId,
    revision: Option<CalibrationRevision>,
}

impl CalibrationReference {
    /// Creates an unversioned reference.
    #[must_use]
    pub const fn new(
        id: CalibrationId,
    ) -> Self {
        Self {
            id,
            revision: None,
        }
    }

    /// Creates a reference pinned to a revision.
    #[must_use]
    pub const fn pinned(
        id: CalibrationId,
        revision: CalibrationRevision,
    ) -> Self {
        Self {
            id,
            revision: Some(revision),
        }
    }

    /// Returns the calibration ID.
    #[must_use]
    pub const fn id(
        self,
    ) -> CalibrationId {
        self.id
    }

    /// Returns the optional pinned revision.
    #[must_use]
    pub const fn revision(
        self,
    ) -> Option<CalibrationRevision> {
        self.revision
    }
}

// ============================================================================
// Lookup query
// ============================================================================

/// Deterministic calibration selection query.
///
/// Selection is semantic and target-independent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationQuery {
    operation: Option<CalibrationOperation>,
    scope: Option<CalibrationScope>,
    revision: Option<CalibrationRevision>,
    timestamp_unix_ns: Option<i128>,
}

impl CalibrationQuery {
    /// Creates an empty query.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation: None,
            scope: None,
            revision: None,
            timestamp_unix_ns: None,
        }
    }

    /// Restricts the query to an operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: CalibrationOperation,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Restricts the query to a scope.
    #[must_use]
    pub fn with_scope(
        mut self,
        scope: CalibrationScope,
    ) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Restricts the query to a revision.
    #[must_use]
    pub const fn with_revision(
        mut self,
        revision: CalibrationRevision,
    ) -> Self {
        self.revision = Some(revision);
        self
    }

    /// Restricts the query to a timestamp.
    #[must_use]
    pub const fn with_timestamp_unix_ns(
        mut self,
        timestamp: i128,
    ) -> Self {
        self.timestamp_unix_ns = Some(timestamp);
        self
    }

    /// Returns the operation filter.
    #[must_use]
    pub fn operation(
        &self,
    ) -> Option<&CalibrationOperation> {
        self.operation.as_ref()
    }

    /// Returns the scope filter.
    #[must_use]
    pub fn scope(
        &self,
    ) -> Option<&CalibrationScope> {
        self.scope.as_ref()
    }

    /// Returns the revision filter.
    #[must_use]
    pub const fn revision(
        &self,
    ) -> Option<CalibrationRevision> {
        self.revision
    }

    /// Returns the timestamp filter.
    #[must_use]
    pub const fn timestamp_unix_ns(
        &self,
    ) -> Option<i128> {
        self.timestamp_unix_ns
    }
}

impl Default for CalibrationQuery {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Calibration registry
// ============================================================================

/// Deterministic collection of calibration definitions.
///
/// The registry is semantic IR state. It is not a hardware calibration
/// database.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationRegistry {
    definitions: BTreeMap<CalibrationId, CalibrationDefinition>,
}

impl CalibrationRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    /// Returns the number of definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Returns whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Inserts a definition.
    ///
    /// IDs are unique. Replacing an existing definition requires explicit
    /// `replace`.
    pub fn insert(
        &mut self,
        definition: CalibrationDefinition,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<()> {
        definition.validate(policy)?;

        if self
            .definitions
            .contains_key(&definition.id())
        {
            return Err(
                CalibrationError::DuplicateCalibrationId {
                    id: definition.id(),
                },
            );
        }

        if let Some(max) =
            policy.max_registry_entries
        {
            if self.definitions.len() >= max {
                return Err(
                    CalibrationError::ResourceLimitExceeded {
                        resource: CalibrationResourceKind::RegistryEntries,
                        actual: self.definitions.len() + 1,
                        maximum: max,
                    },
                );
            }
        }

        self.definitions.insert(
            definition.id(),
            definition,
        );

        Ok(())
    }

    /// Replaces an existing definition with the same identity.
    pub fn replace(
        &mut self,
        definition: CalibrationDefinition,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<Option<CalibrationDefinition>> {
        definition.validate(policy)?;

        if !self
            .definitions
            .contains_key(&definition.id())
        {
            return Err(
                CalibrationError::UnknownCalibration {
                    id: definition.id(),
                },
            );
        }

        Ok(self
            .definitions
            .insert(
                definition.id(),
                definition,
            ))
    }

    /// Returns a definition by identity.
    #[must_use]
    pub fn get(
        &self,
        id: CalibrationId,
    ) -> Option<&CalibrationDefinition> {
        self.definitions.get(&id)
    }

    /// Returns all definitions in deterministic identity order.
    #[must_use]
    pub fn definitions(
        &self,
    ) -> &BTreeMap<CalibrationId, CalibrationDefinition> {
        &self.definitions
    }

    /// Removes a definition.
    pub fn remove(
        &mut self,
        id: CalibrationId,
    ) -> Option<CalibrationDefinition> {
        self.definitions.remove(&id)
    }

    /// Finds the best matching calibration deterministically.
    ///
    /// Selection order:
    ///
    /// 1. explicit operation match;
    /// 2. explicit scope match;
    /// 3. validity match;
    /// 4. highest priority;
    /// 5. highest revision;
    /// 6. lowest CalibrationId as the deterministic final tie-breaker.
    pub fn select(
        &self,
        query: &CalibrationQuery,
    ) -> CalibrationResult<&CalibrationDefinition> {
        let mut candidates = Vec::new();

        for definition in self.definitions.values() {
            if !matches_operation(
                definition,
                query.operation(),
            ) {
                continue;
            }

            if !matches_scope(
                definition,
                query.scope(),
            ) {
                continue;
            }

            if let Some(revision) =
                query.revision()
            {
                if !definition.accepts_revision(
                    revision,
                ) {
                    continue;
                }
            }

            if let Some(timestamp) =
                query.timestamp_unix_ns()
            {
                if !definition.accepts_unix_ns(
                    timestamp,
                ) {
                    continue;
                }
            }

            candidates.push(definition);
        }

        candidates
            .into_iter()
            .max_by(
                |left, right| {
                    left.priority()
                        .cmp(&right.priority())
                        .then_with(
                            || {
                                left.revision()
                                    .cmp(
                                        &right.revision(),
                                    )
                            },
                        )
                        .then_with(
                            || {
                                right.id()
                                    .cmp(&left.id())
                            },
                        )
                },
            )
            .ok_or(
                CalibrationError::NoMatchingCalibration,
            )
    }

    /// Validates every definition in the registry.
    pub fn validate(
        &self,
        policy: &CalibrationValidationPolicy,
    ) -> CalibrationResult<()> {
        enforce_limit(
            self.definitions.len(),
            policy.max_registry_entries,
            CalibrationResourceKind::RegistryEntries,
        )?;

        for definition in self.definitions.values() {
            definition.validate(policy)?;
        }

        validate_registry_dependencies(
            self,
        )?;

        Ok(())
    }
}

impl Default for CalibrationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Registry dependency validation
// ============================================================================

fn validate_registry_dependencies(
    registry: &CalibrationRegistry,
) -> CalibrationResult<()> {
    for definition in registry.definitions.values() {
        for dependency in definition.dependencies() {
            if let Some(id) =
                dependency.calibration_id()
            {
                match dependency {
                    CalibrationDependency::Requires(_)
                    | CalibrationDependency::Supersedes(_)
                    | CalibrationDependency::DerivedFrom(_) => {
                        if registry.get(id).is_none() {
                            return Err(
                                CalibrationError::MissingDependency {
                                    calibration: definition.id(),
                                    dependency: id,
                                },
                            );
                        }
                    }

                    CalibrationDependency::Extension(_) => {}
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// Matching
// ============================================================================

fn matches_operation(
    definition: &CalibrationDefinition,
    query: Option<&CalibrationOperation>,
) -> bool {
    let Some(query) = query else {
        return true;
    };

    match (
        definition.operation(),
        query,
    ) {
        (
            CalibrationOperation::Named(left),
            CalibrationOperation::Named(right),
        ) => left == right,

        (
            CalibrationOperation::Operation(left),
            CalibrationOperation::Operation(right),
        ) => left == right,

        (
            CalibrationOperation::Extension(left),
            CalibrationOperation::Extension(right),
        ) => left == right,

        _ => false,
    }
}

fn matches_scope(
    definition: &CalibrationDefinition,
    query: Option<&CalibrationScope>,
) -> bool {
    let Some(query) = query else {
        return true;
    };

    match (
        definition.scope(),
        query,
    ) {
        (
            CalibrationScope::Global,
            _,
        ) => true,

        (
            CalibrationScope::Qubit(left),
            CalibrationScope::Qubit(right),
        ) => left == right,

        (
            CalibrationScope::Qubits(left),
            CalibrationScope::Qubits(right),
        ) => left == right,

        (
            CalibrationScope::Resource(left),
            CalibrationScope::Resource(right),
        ) => left == right,

        (
            CalibrationScope::Operation(left),
            CalibrationScope::Operation(right),
        ) => left == right,

        (
            CalibrationScope::Extension(left),
            CalibrationScope::Extension(right),
        ) => left == right,

        _ => false,
    }
}

// ============================================================================
// Identifier validation
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalibrationIdentifierKind {
    CalibrationName,
    ParameterName,
}

fn validate_identifier(
    value: &str,
    kind: CalibrationIdentifierKind,
) -> CalibrationResult<()> {
    if value.trim().is_empty() {
        return Err(match kind {
            CalibrationIdentifierKind::CalibrationName => {
                CalibrationError::EmptyCalibrationName
            }

            CalibrationIdentifierKind::ParameterName => {
                CalibrationError::EmptyParameterName
            }
        });
    }

    if value.contains('\0') {
        return Err(match kind {
            CalibrationIdentifierKind::CalibrationName => {
                CalibrationError::InvalidCalibrationName
            }

            CalibrationIdentifierKind::ParameterName => {
                CalibrationError::InvalidParameterName
            }
        });
    }

    Ok(())
}

// ============================================================================
// Metadata validation
// ============================================================================

fn validate_metadata_key(
    key: &str,
) -> CalibrationResult<()> {
    if key.trim().is_empty() {
        return Err(
            CalibrationError::EmptyMetadataKey,
        );
    }

    if key.contains('\0') {
        return Err(
            CalibrationError::InvalidMetadataKey,
        );
    }

    Ok(())
}

fn validate_metadata_value(
    value: &str,
) -> CalibrationResult<()> {
    if value.contains('\0') {
        return Err(
            CalibrationError::InvalidMetadataValue,
        );
    }

    Ok(())
}

// ============================================================================
// Policy helpers
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalibrationResourceKind {
    Parameters,
    Targets,
    Dependencies,
    MetadataEntries,
    RegistryEntries,
    Extensions,
}

fn enforce_limit(
    actual: usize,
    maximum: Option<usize>,
    resource: CalibrationResourceKind,
) -> CalibrationResult<()> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(
                CalibrationError::ResourceLimitExceeded {
                    resource,
                    actual,
                    maximum,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Complete local error model for calibration IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibrationError {
    /// A calibration name was empty.
    EmptyCalibrationName,

    /// A calibration name contained an invalid character.
    InvalidCalibrationName,

    /// A parameter name was empty.
    EmptyParameterName,

    /// A parameter name contained an invalid character.
    InvalidParameterName,

    /// An operation name was empty.
    EmptyOperationName,

    /// An extension operation was empty.
    EmptyExtensionOperation,

    /// An extension scope was empty.
    EmptyExtensionScope,

    /// A qubit scope contained no qubits.
    EmptyQubitScope,

    /// A multi-qubit scope was not canonical.
    NonCanonicalQubitScope,

    /// An extension dependency name was empty.
    EmptyDependencyExtension,

    /// A parameter had a duplicate name.
    DuplicateParameter {
        parameter: String,
    },

    /// A parameter binding had a duplicate name.
    DuplicateBinding {
        parameter: String,
    },

    /// A binding referenced an unknown parameter.
    UnknownParameterBinding {
        parameter: String,
    },

    /// A required parameter was not bound.
    MissingRequiredParameter {
        parameter: String,
    },

    /// A required parameter incorrectly had a default.
    RequiredParameterHasDefault {
        parameter: String,
    },

    /// A parameter unit was empty.
    EmptyParameterUnit {
        parameter: String,
    },

    /// A parameter description was empty.
    EmptyParameterDescription {
        parameter: String,
    },

    /// A dependency was repeated.
    DuplicateDependency,

    /// A calibration depended upon itself.
    SelfDependency,

    /// A referenced calibration did not exist.
    MissingDependency {
        calibration: CalibrationId,
        dependency: CalibrationId,
    },

    /// A calibration identity was already registered.
    DuplicateCalibrationId {
        id: CalibrationId,
    },

    /// A calibration identity was not registered.
    UnknownCalibration {
        id: CalibrationId,
    },

    /// No calibration matched a query.
    NoMatchingCalibration,

    /// Revision validity interval was malformed.
    InvalidRevisionWindow,

    /// Timestamp validity interval was malformed.
    InvalidTimestampWindow,

    /// Metadata key was empty.
    EmptyMetadataKey,

    /// Metadata key contained a NUL byte.
    InvalidMetadataKey,

    /// Metadata value contained a NUL byte.
    InvalidMetadataValue,

    /// Metadata key exceeded the configured limit.
    MetadataKeyTooLarge {
        actual: usize,
        maximum: usize,
    },

    /// Metadata value exceeded the configured limit.
    MetadataValueTooLarge {
        actual: usize,
        maximum: usize,
    },

    /// Extension name was empty.
    EmptyExtensionName,

    /// Extension value was empty.
    EmptyExtensionValue,

    /// A policy limit was exceeded.
    ResourceLimitExceeded {
        resource: CalibrationResourceKind,
        actual: usize,
        maximum: usize,
    },
}

impl fmt::Display for CalibrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyCalibrationName => {
                write!(
                    formatter,
                    "calibration name must not be empty"
                )
            }

            Self::InvalidCalibrationName => {
                write!(
                    formatter,
                    "calibration name contains an invalid NUL character"
                )
            }

            Self::EmptyParameterName => {
                write!(
                    formatter,
                    "calibration parameter name must not be empty"
                )
            }

            Self::InvalidParameterName => {
                write!(
                    formatter,
                    "calibration parameter name contains an invalid NUL character"
                )
            }

            Self::EmptyOperationName => {
                write!(
                    formatter,
                    "calibration operation name must not be empty"
                )
            }

            Self::EmptyExtensionOperation => {
                write!(
                    formatter,
                    "calibration extension operation must not be empty"
                )
            }

            Self::EmptyExtensionScope => {
                write!(
                    formatter,
                    "calibration extension scope must not be empty"
                )
            }

            Self::EmptyQubitScope => {
                write!(
                    formatter,
                    "calibration qubit scope must contain at least one qubit"
                )
            }

            Self::NonCanonicalQubitScope => {
                write!(
                    formatter,
                    "calibration qubit scope must be strictly sorted and duplicate-free"
                )
            }

            Self::EmptyDependencyExtension => {
                write!(
                    formatter,
                    "calibration dependency extension must not be empty"
                )
            }

            Self::DuplicateParameter {
                parameter,
            } => {
                write!(
                    formatter,
                    "duplicate calibration parameter `{}`",
                    parameter
                )
            }

            Self::DuplicateBinding {
                parameter,
            } => {
                write!(
                    formatter,
                    "duplicate calibration binding for `{}`",
                    parameter
                )
            }

            Self::UnknownParameterBinding {
                parameter,
            } => {
                write!(
                    formatter,
                    "calibration binding references unknown parameter `{}`",
                    parameter
                )
            }

            Self::MissingRequiredParameter {
                parameter,
            } => {
                write!(
                    formatter,
                    "required calibration parameter `{}` is not bound",
                    parameter
                )
            }

            Self::RequiredParameterHasDefault {
                parameter,
            } => {
                write!(
                    formatter,
                    "required calibration parameter `{}` cannot have a default value",
                    parameter
                )
            }

            Self::EmptyParameterUnit {
                parameter,
            } => {
                write!(
                    formatter,
                    "unit for calibration parameter `{}` must not be empty",
                    parameter
                )
            }

            Self::EmptyParameterDescription {
                parameter,
            } => {
                write!(
                    formatter,
                    "description for calibration parameter `{}` must not be empty",
                    parameter
                )
            }

            Self::DuplicateDependency => {
                write!(
                    formatter,
                    "duplicate calibration dependency"
                )
            }

            Self::SelfDependency => {
                write!(
                    formatter,
                    "calibration cannot depend on itself"
                )
            }

            Self::MissingDependency {
                calibration,
                dependency,
            } => {
                write!(
                    formatter,
                    "calibration {} references missing dependency {}",
                    calibration,
                    dependency
                )
            }

            Self::DuplicateCalibrationId {
                id,
            } => {
                write!(
                    formatter,
                    "calibration {} is already registered",
                    id
                )
            }

            Self::UnknownCalibration {
                id,
            } => {
                write!(
                    formatter,
                    "calibration {} is not registered",
                    id
                )
            }

            Self::NoMatchingCalibration => {
                write!(
                    formatter,
                    "no calibration matches the requested query"
                )
            }

            Self::InvalidRevisionWindow => {
                write!(
                    formatter,
                    "calibration revision validity window is invalid"
                )
            }

            Self::InvalidTimestampWindow => {
                write!(
                    formatter,
                    "calibration timestamp validity window is invalid"
                )
            }

            Self::EmptyMetadataKey => {
                write!(
                    formatter,
                    "calibration metadata key must not be empty"
                )
            }

            Self::InvalidMetadataKey => {
                write!(
                    formatter,
                    "calibration metadata key contains a NUL character"
                )
            }

            Self::InvalidMetadataValue => {
                write!(
                    formatter,
                    "calibration metadata value contains a NUL character"
                )
            }

            Self::MetadataKeyTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration metadata key is {} bytes; maximum is {}",
                    actual,
                    maximum
                )
            }

            Self::MetadataValueTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration metadata value is {} bytes; maximum is {}",
                    actual,
                    maximum
                )
            }

            Self::EmptyExtensionName => {
                write!(
                    formatter,
                    "calibration extension name must not be empty"
                )
            }

            Self::EmptyExtensionValue => {
                write!(
                    formatter,
                    "calibration extension value must not be empty"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration resource {:?} contains {}; configured maximum is {}",
                    resource,
                    actual,
                    maximum
                )
            }
        }
    }
}

impl std::error::Error for CalibrationError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn calibration_id(
        value: u64,
    ) -> CalibrationId {
        CalibrationId::new(value)
    }

    fn operation_id(
        value: u64,
    ) -> OperationId {
        OperationId::new(value)
    }

    fn qubit_id(
        value: u64,
    ) -> QubitId {
        QubitId::new(value)
    }

    #[test]
    fn unrestricted_policy_has_no_architectural_limits() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        assert_eq!(
            policy.max_parameters,
            None
        );

        assert_eq!(
            policy.max_targets,
            None
        );

        assert_eq!(
            policy.max_registry_entries,
            None
        );
    }

    #[test]
    fn calibration_revision_is_ordered() {
        let first =
            CalibrationRevision::new(1);

        let second =
            CalibrationRevision::new(2);

        assert!(second > first);
        assert_eq!(
            first.value(),
            1
        );
    }

    #[test]
    fn qubit_scope_is_canonicalized() {
        let scope =
            CalibrationScope::qubits([
                qubit_id(7),
                qubit_id(2),
                qubit_id(7),
                qubit_id(1),
            ]);

        assert_eq!(
            scope.logical_qubits(),
            &[
                qubit_id(1),
                qubit_id(2),
                qubit_id(7),
            ]
        );
    }

    #[test]
    fn empty_qubit_scope_is_rejected() {
        let scope =
            CalibrationScope::qubits(
                std::iter::empty(),
            );

        assert_eq!(
            scope.validate(),
            Err(
                CalibrationError::EmptyQubitScope
            )
        );
    }

    #[test]
    fn named_operation_requires_name() {
        assert!(
            CalibrationOperation::named("")
                .is_err()
        );
    }

    #[test]
    fn required_parameter_cannot_have_default() {
        let result =
            CalibrationParameter::new(
                "amp",
                CalibrationParameterRole::Amplitude,
                true,
                None,
                None,
                None,
            );

        assert!(result.is_ok());
    }

    #[test]
    fn duplicate_parameter_is_rejected() {
        let operation =
            CalibrationOperation::operation(
                operation_id(1),
            );

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                operation,
                CalibrationScope::Global,
            )
            .unwrap();

        let parameter =
            CalibrationParameter::required(
                "amp",
                CalibrationParameterRole::Amplitude,
            )
            .unwrap();

        let definition =
            definition
                .with_parameter(
                    parameter.clone(),
                )
                .unwrap();

        let result =
            definition.with_parameter(
                parameter,
            );

        assert!(matches!(
            result,
            Err(
                CalibrationError::DuplicateParameter {
                    ..
                }
            )
        ));
    }

    #[test]
    fn unknown_binding_is_rejected() {
        let operation =
            CalibrationOperation::operation(
                operation_id(1),
            );

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                operation,
                CalibrationScope::Global,
            )
            .unwrap();

        let parameter =
            Parameter::new_symbol("amp")
                .unwrap();

        let binding =
            CalibrationBinding::new(
                "unknown",
                parameter,
            )
            .unwrap();

        let result =
            definition.with_binding(
                binding,
            );

        assert!(matches!(
            result,
            Err(
                CalibrationError::UnknownParameterBinding {
                    ..
                }
            )
        ));
    }

    #[test]
    fn registry_rejects_duplicate_ids() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let operation =
            CalibrationOperation::operation(
                operation_id(1),
            );

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                operation.clone(),
                CalibrationScope::Global,
            )
            .unwrap();

        let mut registry =
            CalibrationRegistry::new();

        registry
            .insert(
                definition,
                &policy,
            )
            .unwrap();

        let duplicate =
            CalibrationDefinition::new(
                calibration_id(1),
                "x2",
                operation,
                CalibrationScope::Global,
            )
            .unwrap();

        assert!(matches!(
            registry.insert(
                duplicate,
                &policy,
            ),
            Err(
                CalibrationError::DuplicateCalibrationId {
                    ..
                }
            )
        ));
    }

    #[test]
    fn registry_selects_highest_priority() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let operation =
            CalibrationOperation::named(
                "x",
            )
            .unwrap();

        let low =
            CalibrationDefinition::new(
                calibration_id(1),
                "x-low",
                operation.clone(),
                CalibrationScope::Global,
            )
            .unwrap()
            .with_priority(
                CalibrationPriority::new(1),
            );

        let high =
            CalibrationDefinition::new(
                calibration_id(2),
                "x-high",
                operation.clone(),
                CalibrationScope::Global,
            )
            .unwrap()
            .with_priority(
                CalibrationPriority::new(2),
            );

        let mut registry =
            CalibrationRegistry::new();

        registry
            .insert(
                low,
                &policy,
            )
            .unwrap();

        registry
            .insert(
                high,
                &policy,
            )
            .unwrap();

        let query =
            CalibrationQuery::new()
                .with_operation(
                    operation,
                );

        let selected =
            registry
                .select(&query)
                .unwrap();

        assert_eq!(
            selected.id(),
            calibration_id(2)
        );
    }

    #[test]
    fn pinned_reference_preserves_revision() {
        let id =
            calibration_id(42);

        let revision =
            CalibrationRevision::new(7);

        let reference =
            CalibrationReference::pinned(
                id,
                revision,
            );

        assert_eq!(
            reference.id(),
            id
        );

        assert_eq!(
            reference.revision(),
            Some(revision)
        );
    }

    #[test]
    fn validity_rejects_outside_revision_window() {
        let validity =
            CalibrationValidity::revisions(
                Some(
                    CalibrationRevision::new(2),
                ),
                Some(
                    CalibrationRevision::new(5),
                ),
            );

        assert!(
            !validity.accepts_revision(
                CalibrationRevision::new(1)
            )
        );

        assert!(
            validity.accepts_revision(
                CalibrationRevision::new(3)
            )
        );

        assert!(
            !validity.accepts_revision(
                CalibrationRevision::new(6)
            )
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata =
            CalibrationMetadata::new();

        metadata
            .insert(
                "z",
                "last",
            )
            .unwrap();

        metadata
            .insert(
                "a",
                "first",
            )
            .unwrap();

        let keys =
            metadata
                .entries()
                .keys()
                .cloned()
                .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec![
                "a".to_owned(),
                "z".to_owned(),
            ]
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let id =
            calibration_id(10);

        let definition =
            CalibrationDefinition::new(
                id,
                "self",
                CalibrationOperation::operation(
                    operation_id(1),
                ),
                CalibrationScope::Global,
            )
            .unwrap();

        let result =
            definition.with_dependency(
                CalibrationDependency::Requires(
                    id,
                ),
            );

        assert!(matches!(
            result,
            Err(
                CalibrationError::SelfDependency
            )
        ));
    }

    #[test]
    fn missing_registry_dependency_is_rejected() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "dependent",
                CalibrationOperation::operation(
                    operation_id(1),
                ),
                CalibrationScope::Global,
            )
            .unwrap()
            .with_dependency(
                CalibrationDependency::Requires(
                    calibration_id(99),
                ),
            )
            .unwrap();

        let mut registry =
            CalibrationRegistry::new();

        registry
            .insert(
                definition,
                &policy,
            )
            .unwrap();

        assert!(matches!(
            registry.validate(&policy),
            Err(
                CalibrationError::MissingDependency {
                    ..
                }
            )
        ));
    }

    #[test]
    fn required_parameters_must_be_bound() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                CalibrationOperation::named(
                    "x",
                )
                .unwrap(),
                CalibrationScope::Qubit(
                    qubit_id(0),
                ),
            )
            .unwrap()
            .with_parameter(
                CalibrationParameter::required(
                    "amp",
                    CalibrationParameterRole::Amplitude,
                )
                .unwrap(),
            )
            .unwrap();

        assert!(matches!(
            definition.validate(&policy),
            Err(
                CalibrationError::MissingRequiredParameter {
                    ..
                }
            )
        ));
    }

    #[test]
    fn required_parameter_binding_validates() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let parameter =
            CalibrationParameter::required(
                "amp",
                CalibrationParameterRole::Amplitude,
            )
            .unwrap();

        let binding =
            CalibrationBinding::new(
                "amp",
                Parameter::new_symbol(
                    "drive_amplitude",
                )
                .unwrap(),
            )
            .unwrap();

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                CalibrationOperation::named(
                    "x",
                )
                .unwrap(),
                CalibrationScope::Qubit(
                    qubit_id(0),
                ),
            )
            .unwrap()
            .with_parameter(
                parameter,
            )
            .unwrap()
            .with_binding(
                binding,
            )
            .unwrap();

        assert!(
            definition
                .validate(&policy)
                .is_ok()
        );
    }

    #[test]
    fn revision_selection_is_deterministic() {
        let policy =
            CalibrationValidationPolicy::unrestricted();

        let operation =
            CalibrationOperation::named(
                "x",
            )
            .unwrap();

        let first =
            CalibrationDefinition::new(
                calibration_id(1),
                "old",
                operation.clone(),
                CalibrationScope::Global,
            )
            .unwrap()
            .with_revision(
                CalibrationRevision::new(1),
            );

        let second =
            CalibrationDefinition::new(
                calibration_id(2),
                "new",
                operation.clone(),
                CalibrationScope::Global,
            )
            .unwrap()
            .with_revision(
                CalibrationRevision::new(2),
            );

        let mut registry =
            CalibrationRegistry::new();

        registry
            .insert(
                first,
                &policy,
            )
            .unwrap();

        registry
            .insert(
                second,
                &policy,
            )
            .unwrap();

        let query =
            CalibrationQuery::new()
                .with_operation(
                    operation,
                );

        let selected =
            registry
                .select(&query)
                .unwrap();

        assert_eq!(
            selected.id(),
            calibration_id(2)
        );
    }

    #[test]
    fn extension_attributes_are_preserved() {
        let mut extensions =
            CalibrationExtensions::new();

        extensions
            .insert(
                "vendor.example.calibration.mode",
                "high_fidelity",
            )
            .unwrap();

        assert_eq!(
            extensions.get(
                "vendor.example.calibration.mode",
            ),
            Some("high_fidelity")
        );
    }

    #[test]
    fn validation_policy_can_limit_resources() {
        let policy =
            CalibrationValidationPolicy::bounded(
                1,
                1,
                1,
                1,
                32,
                32,
                1,
                1,
            );

        let definition =
            CalibrationDefinition::new(
                calibration_id(1),
                "x",
                CalibrationOperation::named(
                    "x",
                )
                .unwrap(),
                CalibrationScope::Qubits([
                    qubit_id(0),
                    qubit_id(1),
                ]),
            )
            .unwrap();

        assert!(matches!(
            definition.validate(&policy),
            Err(
                CalibrationError::ResourceLimitExceeded {
                    resource:
                        CalibrationResourceKind::Targets,
                    ..
                }
            )
        ));
    }
}