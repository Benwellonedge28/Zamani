//! Zamani Quantum Noise (ZQN) — Gate Calibration.
//!
//! Path:
//!
//!     src/quantum/zqn/calibration/gate.rs
//!
//! # Purpose
//!
//! This module defines the backend-independent semantic representation of the
//! calibration associated with a quantum operation.
//!
//! A gate calibration answers:
//!
//! > Which calibrated parameters and quality information apply when a
//! > particular canonical quantum operation is realized on particular quantum
//! > resources during a particular validity interval?
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - gate-calibration identity;
//! - gate-calibration revision;
//! - canonical operation/resource association;
//! - arbitrary-arity gate resource roles;
//! - references to generic calibration parameters;
//! - calibration quality metrics;
//! - validity windows;
//! - calibration status;
//! - deterministic parameter ordering;
//! - gate-calibration validation.
//!
//! This module does NOT own:
//!
//! - the canonical quantum operation definition;
//! - gate semantics;
//! - gate parsing;
//! - frontend ASTs;
//! - quantum IR construction;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - QPU APIs;
//! - credentials;
//! - pulse execution;
//! - channel mathematics;
//! - noise models;
//! - calibration experiments;
//! - statistical fitting;
//! - drift algorithms;
//! - interpolation algorithms;
//! - readout calibration;
//! - simulator state;
//! - QEC;
//! - benchmarking methodology;
//! - serialization wire formats.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Critical architectural rule
//!
//! `GateCalibration` is NOT a replacement for the canonical quantum IR.
//!
//! The canonical operation identity remains owned by:
//!
//!     crate::quantum::ir
//!
//! In particular, this module does not define a new `Gate`, `Operation`,
//! `QubitId`, or `PhysicalQubitId`.
//!
//! # Canonical resource identity
//!
//! When a calibration is associated with quantum resources, this module uses:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! directly.
//!
//! ZQN must never create another qubit identity type merely for calibration.
//!
//! # Parameter ownership
//!
//! Generic calibration parameters are owned by:
//!
//!     calibration::parameter
//!
//! This module references them through `NoiseParameterId`.
//!
//! This prevents gate calibration from becoming a second parameter registry.
//!
//! # Write once / scale everywhere
//!
//! There is no fixed:
//!
//! - number of qubits;
//! - gate arity;
//! - number of parameters;
//! - number of controls;
//! - number of targets;
//! - number of calibration resources;
//! - number of simultaneous resources.
//!
//! A one-resource operation and an operation involving arbitrarily many
//! resources use the same representation.
//!
//! Any practical resource restriction belongs to an explicit resource-policy
//! or execution context, not to this semantic type.
//!
//! # Technology neutrality
//!
//! Although the file is named `gate.rs`, the representation deliberately does
//! not assume:
//!
//! - qubit-only hardware;
//! - binary quantum systems;
//! - a fixed gate set;
//! - Pauli gates;
//! - superconducting hardware;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - a particular pulse representation.
//!
//! The canonical operation may represent a gate-like operation for one target,
//! many targets, or a future operation family.
//!
//! # Determinism
//!
//! This module:
//!
//! - does not use an RNG;
//! - does not read wall-clock time;
//! - does not generate IDs;
//! - does not use global state;
//! - does not perform I/O;
//! - does not depend on hash-map iteration order.
//!
//! Parameter bindings are explicitly ordered and validation is deterministic.
//!
//! # Immutability
//!
//! `GateCalibration` is an immutable value object from the perspective of its
//! public API. Updating a calibration produces a new revision/value rather than
//! silently mutating a calibration already used by an execution.
//!
//! # Numerical safety
//!
//! All floating-point metrics are required to be finite.
//!
//! NaN and positive/negative infinity are rejected.
//!
//! No numerical approximation is silently introduced by this module.
//!
//! # Serialization
//!
//! This module intentionally does not define a wire format.
//!
//! The canonical serialization layer is:
//!
//!     crate::quantum::zqn::io
//!
//! The serialized representation must preserve all semantic fields, including
//! parameter binding order and resource-role order.
//!
//! Rust memory layout is not a serialization contract.
//!
//! # Security
//!
//! A gate calibration is data.
//!
//! It must never contain:
//!
//! - credentials;
//! - authentication tokens;
//! - executable code;
//! - provider SDK objects;
//! - network handles;
//! - implicit file paths;
//! - mutable hardware handles.
//!
//! An identifier grants no authority over the referenced resource.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration graph
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                  OperationId / QubitId
//!                         │
//!                         ▼
//!              calibration::gate
//!                         │
//!          ┌──────────────┼──────────────┐
//!          │              │              │
//!          ▼              ▼              ▼
//!     parameter       snapshot        validation
//!          │              │
//!          └───────┬──────┘
//!                  ▼
//!              noise model
//!                  │
//!       ┌──────────┼──────────┐
//!       ▼          ▼          ▼
//!    hardware    routing    scheduling
//!       │
//!       ▼
//!    runtime
//! ```
//!
//! # Integration contract
//!
//! `GateCalibration` is intended to be consumed by:
//!
//! - `calibration::snapshot` — stores immutable calibration state;
//! - `calibration::device` — associates calibration with device resources;
//! - `calibration::validation` — validates calibration consistency;
//! - `calibration::drift` — produces newer calibration revisions;
//! - `calibration::interpolation` — derives time-valid calibration values;
//! - `noise::model` — obtains calibrated noise parameters;
//! - `integration::ir` — associates calibration with canonical IR operations;
//! - `integration::routing` — obtains calibration-aware operation costs;
//! - `integration::scheduling` — obtains calibrated durations and quality;
//! - `integration::hardware` — imports/exports abstract calibration information;
//! - `integration::benchmarking` — associates observations with calibration;
//! - `simulation` — consumes resolved parameter values rather than owning them.
//!
//! No consumer should modify a `GateCalibration` in place.
//!
//! # Error contract
//!
//! All operational validation failures are reported through the canonical:
//!
//!     crate::quantum::zqn::core::errors::ZqnError
//!
//! via:
//!
//!     crate::quantum::zqn::core::errors::ZqnResult
//!
//! No competing top-level ZQN error hierarchy is introduced here.
//!
//! # Testing contract
//!
//! This file owns tests for:
//!
//! - valid construction;
//! - empty-name rejection;
//! - arbitrary resource arity;
//! - logical/physical resource identity;
//! - duplicate role rejection;
//! - duplicate parameter binding rejection;
//! - non-finite metric rejection;
//! - invalid validity windows;
//! - deterministic ordering;
//! - status transitions;
//! - validation;
//! - equality and cloning;
//! - absence of machine-size assumptions.
//!
//! Higher-level tests belong in `calibration/`, `integration/`, and
//! `tests/scaling/`.
//!
//! # No unsafe
//!
//! `#![forbid(unsafe_code)]` makes unsafe Rust impossible in this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::calibration::parameter::{
    CalibrationParameter,
    CalibrationScope,
    CalibrationParameterStatus,
};
use crate::quantum::zqn::core::errors::ZqnResult;
use crate::quantum::zqn::core::ids::{CalibrationId, NoiseParameterId};

// ============================================================================
// VERSION
// ============================================================================

/// Semantic revision of this gate-calibration representation.
///
/// This is a schema/semantic revision, not a hardware-size limit.
pub const GATE_CALIBRATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// REVISION
// ============================================================================

/// Semantic revision of a gate calibration.
///
/// A revision identifies a different semantic calibration state. It does not
/// represent execution ordering or hardware topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateCalibrationRevision {
    major: u32,
    minor: u32,
    patch: u32,
}

impl GateCalibrationRevision {
    /// Creates an explicit gate-calibration revision.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for GateCalibrationRevision {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for GateCalibrationRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ============================================================================
// STATUS
// ============================================================================

/// Lifecycle state of a gate calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateCalibrationStatus {
    /// Calibration is validated and usable.
    Valid,

    /// Calibration has been recorded but not fully validated.
    Unvalidated,

    /// Calibration is no longer temporally current.
    Stale,

    /// Calibration has failed validation.
    Invalid,

    /// Calibration has intentionally been disabled.
    Disabled,

    /// Calibration has been replaced by a newer revision.
    Superseded,
}

impl Default for GateCalibrationStatus {
    fn default() -> Self {
        Self::Unvalidated
    }
}

impl fmt::Display for GateCalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => f.write_str("valid"),
            Self::Unvalidated => f.write_str("unvalidated"),
            Self::Stale => f.write_str("stale"),
            Self::Invalid => f.write_str("invalid"),
            Self::Disabled => f.write_str("disabled"),
            Self::Superseded => f.write_str("superseded"),
        }
    }
}

// ============================================================================
// OPERATION SELECTOR
// ============================================================================

/// Identifies the canonical operation to which the calibration applies.
///
/// `OperationId` is optional because some calibration data describes a family
/// of equivalent canonical operations before a concrete IR operation exists.
///
/// `name` is a semantic operation-family name, not a vendor API name.
///
/// Examples could be semantic names such as `rotation`, `controlled_rotation`,
/// or a Zamani-defined operation-family identifier. This module deliberately
/// does not hard-code those names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateOperationSelector {
    operation: Option<OperationId>,
    name: String,
}

impl GateOperationSelector {
    /// Creates a selector for a semantic operation family.
    pub fn named(name: impl Into<String>) -> ZqnResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate operation calibration name must not be empty",
            ));
        }

        Ok(Self {
            operation: None,
            name,
        })
    }

    /// Creates a selector for one canonical IR operation.
    pub fn operation(operation: OperationId, name: impl Into<String>) -> ZqnResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate operation calibration name must not be empty",
            ));
        }

        Ok(Self {
            operation: Some(operation),
            name,
        })
    }

    /// Returns the optional canonical IR operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the semantic operation-family name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validates the selector.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.name.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate operation calibration name must not be empty",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// RESOURCE ROLE
// ============================================================================

/// Semantic role assigned to a calibrated quantum resource.
///
/// Roles are data-driven and are not limited to `control` and `target`.
///
/// This allows future operations to use roles such as:
///
/// - control;
/// - target;
/// - spectator;
/// - ancilla;
/// - mediator;
/// - mode;
/// - drive;
/// - auxiliary.
///
/// The operation semantics remain owned by the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateResourceRole {
    role: String,
}

impl GateResourceRole {
    /// Creates a semantic resource role.
    pub fn new(role: impl Into<String>) -> ZqnResult<Self> {
        let role = role.into();

        if role.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate resource role must not be empty",
            ));
        }

        Ok(Self { role })
    }

    /// Returns the role name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.role
    }
}

impl fmt::Display for GateResourceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.role)
    }
}

// ============================================================================
// RESOURCE REFERENCE
// ============================================================================

/// Canonical quantum resource associated with a gate calibration.
///
/// Logical and physical qubit identities are imported directly from
/// `quantum::ir::qubit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateResource {
    /// A canonical logical qubit.
    LogicalQubit(QubitId),

    /// A canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl GateResource {
    /// Creates a logical-qubit resource.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit resource.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }
}

// ============================================================================
// RESOURCE BINDING
// ============================================================================

/// Associates a semantic resource role with one canonical resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateResourceBinding {
    role: GateResourceRole,
    resource: GateResource,
}

impl GateResourceBinding {
    /// Creates a role/resource binding.
    pub fn new(role: GateResourceRole, resource: GateResource) -> Self {
        Self { role, resource }
    }

    /// Returns the role.
    #[must_use]
    pub fn role(&self) -> &GateResourceRole {
        &self.role
    }

    /// Returns the bound resource.
    #[must_use]
    pub const fn resource(&self) -> GateResource {
        self.resource
    }
}

// ============================================================================
// PARAMETER BINDING
// ============================================================================

/// Semantic role of a calibration parameter within a gate calibration.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateParameterRole {
    name: String,
}

impl GateParameterRole {
    /// Creates a semantic parameter role.
    pub fn new(name: impl Into<String>) -> ZqnResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate calibration parameter role must not be empty",
            ));
        }

        Ok(Self { name })
    }

    /// Returns the role name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for GateParameterRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

/// References a generic calibration parameter without taking ownership of the
/// parameter registry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateParameterBinding {
    role: GateParameterRole,
    parameter: NoiseParameterId,
    required: bool,
}

impl GateParameterBinding {
    /// Creates a required parameter binding.
    #[must_use]
    pub fn required(
        role: GateParameterRole,
        parameter: NoiseParameterId,
    ) -> Self {
        Self {
            role,
            parameter,
            required: true,
        }
    }

    /// Creates an optional parameter binding.
    #[must_use]
    pub fn optional(
        role: GateParameterRole,
        parameter: NoiseParameterId,
    ) -> Self {
        Self {
            role,
            parameter,
            required: false,
        }
    }

    /// Returns the semantic parameter role.
    #[must_use]
    pub fn role(&self) -> &GateParameterRole {
        &self.role
    }

    /// Returns the referenced parameter identity.
    #[must_use]
    pub const fn parameter_id(&self) -> NoiseParameterId {
        self.parameter
    }

    /// Returns whether the parameter is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }
}

// ============================================================================
// QUALITY METRIC
// ============================================================================

/// A calibrated quality metric.
///
/// The metric name is intentionally semantic rather than vendor-specific.
///
/// Examples:
///
/// - fidelity;
/// - error_probability;
/// - duration;
/// - leakage_probability;
/// - calibration_confidence.
///
/// The interpretation of the metric belongs to the consumer that owns that
/// metric's semantics. This type only guarantees a finite value.
#[derive(Debug, Clone, PartialEq)]
pub struct GateCalibrationMetric {
    name: String,
    value: f64,
}

impl GateCalibrationMetric {
    /// Creates a finite calibration metric.
    pub fn new(name: impl Into<String>, value: f64) -> ZqnResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate calibration metric name must not be empty",
            ));
        }

        if !value.is_finite() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate calibration metric must be finite",
            ));
        }

        Ok(Self { name, value })
    }

    /// Returns the metric name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metric value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

impl Eq for GateCalibrationMetric {}

impl std::hash::Hash for GateCalibrationMetric {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.value.to_bits().hash(state);
    }
}

// ============================================================================
// VALIDITY WINDOW
// ============================================================================

/// Explicit validity interval for a gate calibration.
///
/// The interval is half-open:
///
///     [start, end)
///
/// `None` means unbounded on that side.
///
/// The time unit is intentionally not encoded here. The owning calibration
/// snapshot/time subsystem defines the timestamp scale and epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GateCalibrationValidity {
    start: Option<i128>,
    end: Option<i128>,
}

impl GateCalibrationValidity {
    /// Creates a validity interval.
    pub fn new(start: Option<i128>, end: Option<i128>) -> ZqnResult<Self> {
        if let (Some(start), Some(end)) = (start, end) {
            if start >= end {
                return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                    "gate calibration validity interval must satisfy start < end",
                ));
            }
        }

        Ok(Self { start, end })
    }

    /// Creates an unbounded validity interval.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Returns the optional start.
    #[must_use]
    pub const fn start(&self) -> Option<i128> {
        self.start
    }

    /// Returns the optional end.
    #[must_use]
    pub const fn end(&self) -> Option<i128> {
        self.end
    }

    /// Returns whether a timestamp belongs to this interval.
    #[must_use]
    pub const fn contains(&self, timestamp: i128) -> bool {
        let after_start = match self.start {
            Some(start) => timestamp >= start,
            None => true,
        };

        let before_end = match self.end {
            Some(end) => timestamp < end,
            None => true,
        };

        after_start && before_end
    }
}

impl Default for GateCalibrationValidity {
    fn default() -> Self {
        Self::unbounded()
    }
}

// ============================================================================
// GATE CALIBRATION
// ============================================================================

/// Immutable backend-independent gate calibration.
///
/// The type intentionally stores references to generic calibration parameters
/// rather than copying parameter values. Parameter ownership remains in
/// `calibration::parameter` / the enclosing calibration snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateCalibration {
    id: CalibrationId,
    revision: GateCalibrationRevision,
    operation: GateOperationSelector,
    resources: Vec<GateResourceBinding>,
    parameters: Vec<GateParameterBinding>,
    metrics: Vec<GateCalibrationMetric>,
    validity: GateCalibrationValidity,
    status: GateCalibrationStatus,
    metadata: BTreeMap<String, String>,
}

impl GateCalibration {
    /// Creates a gate calibration.
    ///
    /// Validation is performed immediately so an invalid calibration cannot be
    /// created through this constructor.
    pub fn new(
        id: CalibrationId,
        revision: GateCalibrationRevision,
        operation: GateOperationSelector,
        resources: Vec<GateResourceBinding>,
        parameters: Vec<GateParameterBinding>,
        metrics: Vec<GateCalibrationMetric>,
        validity: GateCalibrationValidity,
        status: GateCalibrationStatus,
        metadata: BTreeMap<String, String>,
    ) -> ZqnResult<Self> {
        let calibration = Self {
            id,
            revision,
            operation,
            resources,
            parameters,
            metrics,
            validity,
            status,
            metadata,
        };

        calibration.validate()?;

        Ok(calibration)
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn id(&self) -> CalibrationId {
        self.id
    }

    /// Returns the semantic revision.
    #[must_use]
    pub const fn revision(&self) -> GateCalibrationRevision {
        self.revision
    }

    /// Returns the operation selector.
    #[must_use]
    pub fn operation(&self) -> &GateOperationSelector {
        &self.operation
    }

    /// Returns the ordered resource bindings.
    #[must_use]
    pub fn resources(&self) -> &[GateResourceBinding] {
        &self.resources
    }

    /// Returns the ordered parameter bindings.
    #[must_use]
    pub fn parameters(&self) -> &[GateParameterBinding] {
        &self.parameters
    }

    /// Returns the ordered quality metrics.
    #[must_use]
    pub fn metrics(&self) -> &[GateCalibrationMetric] {
        &self.metrics
    }

    /// Returns the validity interval.
    #[must_use]
    pub const fn validity(&self) -> GateCalibrationValidity {
        self.validity
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> GateCalibrationStatus {
        self.status
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns a new calibration with a different lifecycle status.
    ///
    /// No existing calibration is mutated.
    pub fn with_status(
        &self,
        status: GateCalibrationStatus,
    ) -> Self {
        let mut next = self.clone();
        next.status = status;
        next
    }

    /// Returns a new calibration with one metadata entry.
    ///
    /// Metadata is non-semantic annotation. An empty key is rejected.
    pub fn with_metadata(
        &self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZqnResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                "gate calibration metadata key must not be empty",
            ));
        }

        let mut next = self.clone();
        next.metadata.insert(key, value.into());
        next.validate()?;
        Ok(next)
    }

    /// Returns whether this calibration is temporally applicable.
    #[must_use]
    pub const fn is_valid_at(&self, timestamp: i128) -> bool {
        self.validity.contains(timestamp)
            && matches!(self.status, GateCalibrationStatus::Valid)
    }

    /// Returns all parameter IDs in deterministic binding order.
    #[must_use]
    pub fn parameter_ids(&self) -> Vec<NoiseParameterId> {
        self.parameters
            .iter()
            .map(GateParameterBinding::parameter_id)
            .collect()
    }

    /// Validates the complete gate-calibration object.
    ///
    /// This validates structural invariants owned by this module.
    ///
    /// It intentionally does not validate whether a referenced parameter
    /// exists in a registry or whether a resource physically exists on a
    /// device. Those checks belong to snapshot/device/target validation.
    pub fn validate(&self) -> ZqnResult<()> {
        self.operation.validate()?;

        Self::validate_resources(&self.resources)?;
        Self::validate_parameters(&self.parameters)?;
        Self::validate_metrics(&self.metrics)?;

        // Validate the validity window by reconstructing it through the public
        // constructor. This keeps the invariant centralized.
        GateCalibrationValidity::new(
            self.validity.start(),
            self.validity.end(),
        )?;

        for key in self.metadata.keys() {
            if key.trim().is_empty() {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "gate calibration metadata key must not be empty",
                    ),
                );
            }
        }

        Ok(())
    }

    fn validate_resources(
        resources: &[GateResourceBinding],
    ) -> ZqnResult<()> {
        let mut seen = Vec::<GateResourceBinding>::with_capacity(resources.len());

        for resource in resources {
            if seen.iter().any(|existing| existing == resource) {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "duplicate gate calibration resource binding",
                    ),
                );
            }

            if seen.iter().any(|existing| {
                existing.role() == resource.role()
            }) {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "duplicate gate calibration resource role",
                    ),
                );
            }

            seen.push(resource.clone());
        }

        Ok(())
    }

    fn validate_parameters(
        parameters: &[GateParameterBinding],
    ) -> ZqnResult<()> {
        let mut seen_ids = Vec::<NoiseParameterId>::with_capacity(parameters.len());
        let mut seen_roles = Vec::<GateParameterRole>::with_capacity(parameters.len());

        for parameter in parameters {
            if seen_ids.iter().any(|id| *id == parameter.parameter_id()) {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "duplicate gate calibration parameter reference",
                    ),
                );
            }

            if seen_roles.iter().any(|role| role == parameter.role()) {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "duplicate gate calibration parameter role",
                    ),
                );
            }

            seen_ids.push(parameter.parameter_id());
            seen_roles.push(parameter.role().clone());
        }

        Ok(())
    }

    fn validate_metrics(
        metrics: &[GateCalibrationMetric],
    ) -> ZqnResult<()> {
        let mut seen_names = Vec::<String>::with_capacity(metrics.len());

        for metric in metrics {
            if metric.name().trim().is_empty() {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "gate calibration metric name must not be empty",
                    ),
                );
            }

            if !metric.value().is_finite() {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "gate calibration metric must be finite",
                    ),
                );
            }

            if seen_names.iter().any(|name| name == metric.name()) {
                return Err(
                    crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                        "duplicate gate calibration metric name",
                    ),
                );
            }

            seen_names.push(metric.name().to_owned());
        }

        Ok(())
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for ergonomic construction of a validated gate calibration.
///
/// The builder contains no global state and does not generate IDs.
#[derive(Debug, Clone)]
pub struct GateCalibrationBuilder {
    id: CalibrationId,
    revision: GateCalibrationRevision,
    operation: GateOperationSelector,
    resources: Vec<GateResourceBinding>,
    parameters: Vec<GateParameterBinding>,
    metrics: Vec<GateCalibrationMetric>,
    validity: GateCalibrationValidity,
    status: GateCalibrationStatus,
    metadata: BTreeMap<String, String>,
}

impl GateCalibrationBuilder {
    /// Creates a builder.
    #[must_use]
    pub fn new(
        id: CalibrationId,
        operation: GateOperationSelector,
    ) -> Self {
        Self {
            id,
            revision: GateCalibrationRevision::default(),
            operation,
            resources: Vec::new(),
            parameters: Vec::new(),
            metrics: Vec::new(),
            validity: GateCalibrationValidity::default(),
            status: GateCalibrationStatus::Unvalidated,
            metadata: BTreeMap::new(),
        }
    }

    /// Sets the semantic revision.
    #[must_use]
    pub const fn revision(
        mut self,
        revision: GateCalibrationRevision,
    ) -> Self {
        self.revision = revision;
        self
    }

    /// Adds one resource binding.
    #[must_use]
    pub fn resource(
        mut self,
        role: GateResourceRole,
        resource: GateResource,
    ) -> Self {
        self.resources.push(GateResourceBinding::new(role, resource));
        self
    }

    /// Adds one required parameter.
    #[must_use]
    pub fn required_parameter(
        mut self,
        role: GateParameterRole,
        parameter: NoiseParameterId,
    ) -> Self {
        self.parameters
            .push(GateParameterBinding::required(role, parameter));
        self
    }

    /// Adds one optional parameter.
    #[must_use]
    pub fn optional_parameter(
        mut self,
        role: GateParameterRole,
        parameter: NoiseParameterId,
    ) -> Self {
        self.parameters
            .push(GateParameterBinding::optional(role, parameter));
        self
    }

    /// Adds one quality metric.
    pub fn metric(
        mut self,
        metric: GateCalibrationMetric,
    ) -> Self {
        self.metrics.push(metric);
        self
    }

    /// Sets the validity interval.
    #[must_use]
    pub fn validity(
        mut self,
        validity: GateCalibrationValidity,
    ) -> Self {
        self.validity = validity;
        self
    }

    /// Sets the lifecycle status.
    #[must_use]
    pub fn status(
        mut self,
        status: GateCalibrationStatus,
    ) -> Self {
        self.status = status;
        self
    }

    /// Adds metadata.
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZqnResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(
                crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                    "gate calibration metadata key must not be empty",
                ),
            );
        }

        self.metadata.insert(key, value.into());
        Ok(self)
    }

    /// Builds and validates the calibration.
    pub fn build(self) -> ZqnResult<GateCalibration> {
        GateCalibration::new(
            self.id,
            self.revision,
            self.operation,
            self.resources,
            self.parameters,
            self.metrics,
            self.validity,
            self.status,
            self.metadata,
        )
    }
}

// ============================================================================
// PARAMETER RESOLUTION CONTRACT
// ============================================================================

/// Resolves a referenced calibration parameter.
///
/// The gate-calibration layer deliberately does not own a registry. An
/// enclosing calibration snapshot/registry supplies this resolver.
pub trait CalibrationParameterResolver {
    /// Resolves a parameter identity.
    fn resolve(
        &self,
        id: NoiseParameterId,
    ) -> ZqnResult<&CalibrationParameter>;
}

/// Validates that all required gate-calibration parameter references resolve
/// to usable parameters.
///
/// This is intentionally separate from `GateCalibration::validate()` because
/// reference existence is an enclosing snapshot/registry responsibility.
pub fn validate_parameter_references<R>(
    calibration: &GateCalibration,
    resolver: &R,
) -> ZqnResult<()>
where
    R: CalibrationParameterResolver,
{
    for binding in calibration.parameters() {
        let parameter = resolver.resolve(binding.parameter_id())?;

        parameter
            .validate()
            .map_err(|error| {
                crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                    error.to_string(),
                )
            })?;

        if binding.is_required()
            && !matches!(
                parameter.status(),
                CalibrationParameterStatus::Valid
            )
        {
            return Err(
                crate::quantum::zqn::core::errors::ZqnError::invalid_calibration(
                    "required gate calibration parameter is not valid",
                ),
            );
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_is_deterministic() {
        let revision = GateCalibrationRevision::new(1, 2, 3);

        assert_eq!(revision.major(), 1);
        assert_eq!(revision.minor(), 2);
        assert_eq!(revision.patch(), 3);
        assert_eq!(revision.to_string(), "1.2.3");
    }

    #[test]
    fn validity_is_half_open() {
        let validity =
            GateCalibrationValidity::new(Some(10), Some(20)).expect("valid");

        assert!(!validity.contains(9));
        assert!(validity.contains(10));
        assert!(validity.contains(19));
        assert!(!validity.contains(20));
    }

    #[test]
    fn validity_rejects_reversed_interval() {
        let result = GateCalibrationValidity::new(Some(20), Some(10));

        assert!(result.is_err());
    }

    #[test]
    fn operation_selector_rejects_empty_name() {
        assert!(GateOperationSelector::named("   ").is_err());
    }

    #[test]
    fn resource_role_rejects_empty_name() {
        assert!(GateResourceRole::new("").is_err());
    }

    #[test]
    fn parameter_role_rejects_empty_name() {
        assert!(GateParameterRole::new("").is_err());
    }

    #[test]
    fn metric_rejects_non_finite_values() {
        assert!(GateCalibrationMetric::new("fidelity", f64::NAN).is_err());
        assert!(
            GateCalibrationMetric::new("fidelity", f64::INFINITY).is_err()
        );
        assert!(
            GateCalibrationMetric::new("fidelity", f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn resource_bindings_support_arbitrary_arity() {
        let operation =
            GateOperationSelector::named("arbitrary_operation").expect("valid");

        let resources = vec![
            GateResourceBinding::new(
                GateResourceRole::new("control").expect("valid"),
                GateResource::logical(QubitId::new(0)),
            ),
            GateResourceBinding::new(
                GateResourceRole::new("target").expect("valid"),
                GateResource::logical(QubitId::new(1)),
            ),
            GateResourceBinding::new(
                GateResourceRole::new("spectator").expect("valid"),
                GateResource::logical(QubitId::new(2)),
            ),
        ];

        let calibration = GateCalibration::new(
            CalibrationId::new(1),
            GateCalibrationRevision::default(),
            operation,
            resources,
            Vec::new(),
            Vec::new(),
            GateCalibrationValidity::default(),
            GateCalibrationStatus::Unvalidated,
            BTreeMap::new(),
        )
        .expect("valid calibration");

        assert_eq!(calibration.resources().len(), 3);
    }

    #[test]
    fn duplicate_resource_role_is_rejected() {
        let operation =
            GateOperationSelector::named("operation").expect("valid");

        let role = GateResourceRole::new("target").expect("valid");

        let resources = vec![
            GateResourceBinding::new(
                role.clone(),
                GateResource::logical(QubitId::new(0)),
            ),
            GateResourceBinding::new(
                role,
                GateResource::logical(QubitId::new(1)),
            ),
        ];

        let result = GateCalibration::new(
            CalibrationId::new(1),
            GateCalibrationRevision::default(),
            operation,
            resources,
            Vec::new(),
            Vec::new(),
            GateCalibrationValidity::default(),
            GateCalibrationStatus::Unvalidated,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_parameter_reference_is_rejected() {
        let operation =
            GateOperationSelector::named("operation").expect("valid");

        let parameter = NoiseParameterId::new(10);

        let parameters = vec![
            GateParameterBinding::required(
                GateParameterRole::new("duration").expect("valid"),
                parameter,
            ),
            GateParameterBinding::optional(
                GateParameterRole::new("error").expect("valid"),
                parameter,
            ),
        ];

        let result = GateCalibration::new(
            CalibrationId::new(1),
            GateCalibrationRevision::default(),
            operation,
            Vec::new(),
            parameters,
            Vec::new(),
            GateCalibrationValidity::default(),
            GateCalibrationStatus::Unvalidated,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_metric_name_is_rejected() {
        let operation =
            GateOperationSelector::named("operation").expect("valid");

        let metrics = vec![
            GateCalibrationMetric::new("fidelity", 0.99).expect("valid"),
            GateCalibrationMetric::new("fidelity", 0.98).expect("valid"),
        ];

        let result = GateCalibration::new(
            CalibrationId::new(1),
            GateCalibrationRevision::default(),
            operation,
            Vec::new(),
            Vec::new(),
            metrics,
            GateCalibrationValidity::default(),
            GateCalibrationStatus::Unvalidated,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn physical_qubit_uses_canonical_ir_identity() {
        let resource =
            GateResource::physical(PhysicalQubitId::new(42));

        assert_eq!(
            resource,
            GateResource::PhysicalQubit(PhysicalQubitId::new(42))
        );
    }

    #[test]
    fn status_change_does_not_mutate_original() {
        let operation =
            GateOperationSelector::named("operation").expect("valid");

        let calibration = GateCalibration::new(
            CalibrationId::new(1),
            GateCalibrationRevision::default(),
            operation,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            GateCalibrationValidity::default(),
            GateCalibrationStatus::Unvalidated,
            BTreeMap::new(),
        )
        .expect("valid calibration");

        let updated =
            calibration.with_status(GateCalibrationStatus::Valid);

        assert_eq!(
            calibration.status(),
            GateCalibrationStatus::Unvalidated
        );

        assert_eq!(updated.status(), GateCalibrationStatus::Valid);
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let operation =
            GateOperationSelector::named("operation").expect("valid");

        let calibration = GateCalibrationBuilder::new(
            CalibrationId::new(1),
            operation,
        )
        .metadata("z", "last")
        .expect("valid")
        .metadata("a", "first")
        .expect("valid")
        .build()
        .expect("valid calibration");

        let keys: Vec<&str> = calibration
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["a", "z"]);
    }
}