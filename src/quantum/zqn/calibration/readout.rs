//! Zamani Quantum Noise (ZQN) — Readout Calibration.
//!
//! Path:
//!
//!     src/quantum/zqn/calibration/readout.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines the canonical, backend-independent semantic model for
//! quantum readout calibration.
//!
//! A readout calibration describes the relationship between an intended/true
//! measurement outcome and an observed readout outcome for one or more
//! canonical quantum resources.
//!
//! The central semantic object is:
//!
//!     ReadoutCalibration
//!
//! It can represent:
//!
//! - single-resource readout calibration;
//! - multi-resource readout calibration;
//! - arbitrary outcome alphabets;
//! - asymmetric assignment errors;
//! - correlated readout errors;
//! - joint assignment matrices;
//! - calibration validity intervals;
//! - calibration provenance references;
//! - uncertainty/parameter references;
//! - revisions;
//! - explicit semantic status;
//! - explicit resource validation.
//!
//! This module deliberately does NOT assume:
//!
//! - binary measurement;
//! - qubits only;
//! - a fixed number of outcomes;
//! - a fixed number of resources;
//! - independent readout errors;
//! - symmetric readout errors;
//! - a particular hardware technology;
//! - a particular vendor;
//! - a particular measurement basis;
//! - a particular correction algorithm.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module OWNS:
//!
//! - readout-calibration semantics;
//! - readout resource scope;
//! - readout outcome spaces;
//! - assignment matrices;
//! - readout calibration validity;
//! - readout calibration status;
//! - references to calibration/noise parameters;
//! - structural validation of readout calibration data;
//! - deterministic access to assignment probabilities.
//!
//! This module DOES NOT OWN:
//!
//! - canonical quantum operation semantics;
//! - canonical qubit identity;
//! - measurement IR;
//! - quantum state evolution;
//! - quantum-channel mathematics;
//! - calibration snapshots;
//! - calibration registries;
//! - hardware discovery;
//! - hardware credentials;
//! - vendor APIs;
//! - statistical fitting;
//! - characterization experiments;
//! - drift algorithms;
//! - interpolation algorithms;
//! - readout-error correction algorithms;
//! - benchmarking methodology;
//! - simulation;
//! - QEC;
//! - routing;
//! - scheduling;
//! - serialization wire formats;
//! - global mutable state.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                 canonical resource identity
//!                              │
//!                              ▼
//!                  calibration::readout
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!        calibration       measurement       noise model
//!          snapshot          integration         │
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                         simulation
//!                              │
//!                              ▼
//!                         observations
//!                              │
//!                              ▼
//!                         benchmarking
//! ```
//!
//! The direction is intentionally one-way:
//!
//! readout calibration provides semantic data to consumers.
//!
//! It does not call hardware, benchmarking, simulation, QEC, routing or
//! scheduling implementations.
//!
//! ============================================================================
//! CANONICAL RESOURCE IDENTITY
//! ============================================================================
//!
//! ZQN MUST NOT define another QubitId or PhysicalQubitId.
//!
//! Canonical resource identity remains owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! This module therefore uses:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! when readout calibration is associated with logical or physical resources.
//!
//! The IDs are treated as opaque identity values. This module does not depend
//! on their internal representation.
//!
//! ============================================================================
//! WRITE-ONCE / SCALE-EVERYWHERE CONTRACT
//! ============================================================================
//!
//! This file contains NO semantic machine-size limit.
//!
//! In particular, it does not define:
//!
//!     MAX_QUBITS
//!     MAX_READOUTS
//!     MAX_OUTCOMES
//!     MAX_MATRIX_SIZE
//!     MAX_CALIBRATIONS
//!
//! The number of resources and outcomes is determined by the data.
//!
//! Resource exhaustion is handled only through explicit caller-provided
//! validation limits.
//!
//! "Infinity" therefore means:
//!
//!     no artificial finite ZQN machine-size ceiling
//!
//! rather than:
//!
//!     infinite physical memory or execution resources.
//!
//! ============================================================================
//! MATHEMATICAL SEMANTICS
//! ============================================================================
//!
//! For a prepared/true outcome `t` and observed outcome `o`, the assignment
//! probability is:
//
//!     A[o,t] = P(observed = o | true = t)
//!
//! Depending on the representation, the matrix is stored row-major as:
//
//!     row = true/prepared outcome
//!     column = observed outcome
//!
//! Therefore every true/prepared-outcome row must satisfy:
//
//!     P(o | t) >= 0
//!
//! and, when a complete assignment model is required:
//
//!     Σ_o P(o | t) = 1
//!
//! The matrix is allowed to be asymmetric.
//!
//! This module does not assume that the number of prepared outcomes equals the
//! number of observed outcomes. This permits readout technologies where the
//! observed alphabet differs from the prepared-state alphabet.
//!
//! ============================================================================
//! CORRELATED READOUT
//! ============================================================================
//!
//! A calibration may cover multiple resources.
//!
//! The module therefore does not model readout calibration as:
//
//!     Vec<[2 x 2]>
//!
//! or:
//!
//!     one fixed matrix per qubit.
//!
//! Instead, an assignment model may contain an arbitrary finite outcome space.
//!
//! For joint readout, a caller may construct outcomes representing joint
//! states, for example:
//
//!     "00"
//!     "01"
//!     "10"
//!     "11"
//!
//! or any other explicitly defined outcome labels.
//!
//! No binary assumption is embedded in the implementation.
//!
//! The semantic meaning of the labels belongs to the measurement/IR layer.
//!
//! ============================================================================
//! UNCERTAINTY
//! ============================================================================
//!
//! This file does not duplicate CalibrationParameter values.
//!
//! Uncertainty-producing calibration parameters are owned by:
//!
//!     calibration::parameter
//!
//! ReadoutCalibration may reference those parameters using NoiseParameterId.
//!
//! This keeps one canonical parameter definition while allowing a readout
//! calibration to identify which calibrated quantities it depends upon.
//!
//! ============================================================================
//! VALIDITY
//! ============================================================================
//!
//! A readout calibration may have:
//!
//!     not_before_ns
//!     not_after_ns
//!
//! The interval is half-open:
//
//!     [not_before_ns, not_after_ns)
//!
//! `None` means unbounded in that direction.
//!
//! Thus:
//!
//!     None, None
//!
//! means no declared temporal bound.
//!
//! This is not permission to ignore stale calibration. Consumers must apply
//! their own execution policy.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! This module is deterministic.
//!
//! It does not:
//!
//! - read the wall clock;
//! - generate random values;
//! - use a global RNG;
//! - use process identity;
//! - use thread identity;
//! - use memory addresses;
//! - perform implicit I/O;
//! - depend on hash-map iteration order.
//!
//! Sampling and stochastic execution belong to the simulation/noise layers.
//!
//! ============================================================================
//! NUMERICAL SAFETY
//! ============================================================================
//!
//! All floating-point assignment probabilities must be:
//!
//!     finite
//!     >= 0
//!     <= 1
//!
//! NaN and ±infinity are rejected.
//!
//! Normalization is validated with an explicitly supplied tolerance.
//!
//! Tolerances themselves must be finite and non-negative.
//!
//! ============================================================================
//! RESOURCE SAFETY
//! ============================================================================
//!
//! Matrix dimensions are checked using checked arithmetic.
//!
//! No multiplication such as:
//!
//!     rows * columns
//!
//! is performed without overflow checking.
//!
//! Large calibration data must be validated against caller-provided limits
//! before expensive operations are performed.
//!
//! The default validation policy imposes no semantic finite limit.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Readout calibration is data, not authority.
//!
//! This module contains no:
//!
//! - API keys;
//! - credentials;
//! - authentication tokens;
//! - executable code;
//! - network handles;
//! - filesystem handles;
//! - vendor SDK objects.
//!
//! Outcome labels and metadata are treated as inert UTF-8 data.
//!
//! ============================================================================
//! SERIALIZATION
//! ============================================================================
//!
//! This module does NOT define a wire format.
//!
//! Serialization belongs to:
//!
//!     crate::quantum::zqn::io
//!
//! The serialization layer must preserve the semantic fields exposed by this
//! module, including:
//!
//! - schema version;
//! - calibration identity;
//! - resource scope;
//! - outcome spaces;
//! - assignment probabilities;
//! - validity;
//! - revision;
//! - status;
//! - parameter references;
//! - provenance reference.
//!
//! Rust memory layout is not a serialization contract.
//!
//! ============================================================================
//! RUST COMPATIBILITY
//! ============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::{
    CalibrationId,
    NoiseParameterId,
};

/// Current semantic schema version for [`ReadoutCalibration`].
///
/// This is a representation version, not a machine-size limit.
pub const READOUT_CALIBRATION_SCHEMA_VERSION: u16 = 1;

/// Default normalization tolerance used by [`ReadoutCalibration::validate`].
///
/// This is a numerical policy default, not a semantic machine-size limit.
pub const DEFAULT_NORMALIZATION_TOLERANCE: f64 = 1.0e-12;

// ============================================================================
// RESOURCE SCOPE
// ============================================================================

/// Quantum resources covered by a readout calibration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadoutResourceScope {
    /// Calibration associated with one or more logical qubits.
    LogicalQubits(Vec<QubitId>),

    /// Calibration associated with one or more physical qubits.
    PhysicalQubits(Vec<PhysicalQubitId>),

    /// Calibration associated with a named abstract resource.
    ///
    /// This is useful for non-qubit measurement resources such as:
    ///
    /// - photonic modes;
    /// - resonators;
    /// - detector channels;
    /// - bosonic modes;
    /// - continuous-variable resources;
    /// - future quantum modalities.
    Named {
        /// Namespace identifying the resource domain.
        namespace: String,

        /// Stable resource key inside the namespace.
        key: String,
    },

    /// Calibration covering a device-wide measurement resource.
    DeviceWide,

    /// Calibration covering an explicitly grouped collection of scopes.
    Composite(Vec<Self>),
}

impl ReadoutResourceScope {
    /// Returns a logical-qubit scope.
    pub fn logical_qubits(qubits: Vec<QubitId>) -> ZqnResult<Self> {
        let scope = Self::LogicalQubits(qubits);
        scope.validate()?;
        Ok(scope)
    }

    /// Returns a physical-qubit scope.
    pub fn physical_qubits(qubits: Vec<PhysicalQubitId>) -> ZqnResult<Self> {
        let scope = Self::PhysicalQubits(qubits);
        scope.validate()?;
        Ok(scope)
    }

    /// Returns a named resource scope.
    pub fn named(
        namespace: impl Into<String>,
        key: impl Into<String>,
    ) -> ZqnResult<Self> {
        let scope = Self::Named {
            namespace: namespace.into(),
            key: key.into(),
        };

        scope.validate()?;
        Ok(scope)
    }

    /// Returns a device-wide scope.
    #[must_use]
    pub const fn device_wide() -> Self {
        Self::DeviceWide
    }

    /// Returns a composite scope.
    pub fn composite(scopes: Vec<Self>) -> ZqnResult<Self> {
        let scope = Self::Composite(scopes);
        scope.validate()?;
        Ok(scope)
    }

    /// Validates structural correctness.
    pub fn validate(&self) -> ZqnResult<()> {
        match self {
            Self::LogicalQubits(qubits) => {
                if qubits.is_empty() {
                    return Err(structure_error(
                        ZqnErrorCode::InvalidIdentifier,
                        "logical-qubit readout scope must not be empty",
                    ));
                }

                if contains_duplicate_values(qubits) {
                    return Err(structure_error(
                        ZqnErrorCode::DuplicateIdentifier,
                        "logical-qubit readout scope contains duplicate resources",
                    ));
                }
            }

            Self::PhysicalQubits(qubits) => {
                if qubits.is_empty() {
                    return Err(structure_error(
                        ZqnErrorCode::InvalidIdentifier,
                        "physical-qubit readout scope must not be empty",
                    ));
                }

                if contains_duplicate_values(qubits) {
                    return Err(structure_error(
                        ZqnErrorCode::DuplicateIdentifier,
                        "physical-qubit readout scope contains duplicate resources",
                    ));
                }
            }

            Self::Named { namespace, key } => {
                if namespace.trim().is_empty() {
                    return Err(structure_error(
                        ZqnErrorCode::InvalidIdentifier,
                        "readout resource namespace must not be empty",
                    ));
                }

                if key.trim().is_empty() {
                    return Err(structure_error(
                        ZqnErrorCode::InvalidIdentifier,
                        "readout resource key must not be empty",
                    ));
                }
            }

            Self::DeviceWide => {}

            Self::Composite(scopes) => {
                if scopes.is_empty() {
                    return Err(structure_error(
                        ZqnErrorCode::InvalidIdentifier,
                        "composite readout scope must not be empty",
                    ));
                }

                for scope in scopes {
                    scope.validate()?;
                }
            }
        }

        Ok(())
    }

    /// Returns the number of explicitly represented resources.
    ///
    /// `DeviceWide` and named resources represent one semantic resource.
    /// Composite scopes sum their component resource counts.
    ///
    /// This is descriptive and never acts as an architectural limit.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        match self {
            Self::LogicalQubits(resources) => resources.len(),
            Self::PhysicalQubits(resources) => resources.len(),
            Self::Named { .. } | Self::DeviceWide => 1,
            Self::Composite(scopes) => scopes
                .iter()
                .map(Self::resource_count)
                .fold(0usize, usize::saturating_add),
        }
    }
}

// ============================================================================
// OUTCOME SPACE
// ============================================================================

/// Explicit readout outcome alphabet.
///
/// Outcome labels are semantic identifiers supplied by the measurement layer.
/// This type deliberately does not assume binary outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadoutOutcomeSpace {
    labels: Vec<String>,
}

impl ReadoutOutcomeSpace {
    /// Creates an outcome space from explicit labels.
    pub fn new(labels: Vec<String>) -> ZqnResult<Self> {
        if labels.is_empty() {
            return Err(structure_error(
                ZqnErrorCode::EmptyDistribution,
                "readout outcome space must not be empty",
            ));
        }

        if labels.iter().any(|label| label.is_empty()) {
            return Err(structure_error(
                ZqnErrorCode::InvalidIdentifier,
                "readout outcome labels must not be empty",
            ));
        }

        if contains_duplicate_values(&labels) {
            return Err(structure_error(
                ZqnErrorCode::DuplicateIdentifier,
                "readout outcome labels must be unique",
            ));
        }

        Ok(Self { labels })
    }

    /// Creates an outcome space from any string-like iterator.
    pub fn from_labels<I, S>(labels: I) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(labels.into_iter().map(Into::into).collect())
    }

    /// Returns all outcome labels in deterministic insertion order.
    #[must_use]
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Returns the number of outcomes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns whether the outcome space is empty.
    ///
    /// This is always false for a successfully constructed value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Finds an outcome index.
    #[must_use]
    pub fn index_of(&self, label: &str) -> Option<usize> {
        self.labels.iter().position(|candidate| candidate == label)
    }

    /// Returns an outcome label by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&str> {
        self.labels.get(index).map(String::as_str)
    }

    /// Validates the outcome space.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.labels.is_empty() {
            return Err(structure_error(
                ZqnErrorCode::EmptyDistribution,
                "readout outcome space must not be empty",
            ));
        }

        if self.labels.iter().any(|label| label.is_empty()) {
            return Err(structure_error(
                ZqnErrorCode::InvalidIdentifier,
                "readout outcome labels must not be empty",
            ));
        }

        if contains_duplicate_values(&self.labels) {
            return Err(structure_error(
                ZqnErrorCode::DuplicateIdentifier,
                "readout outcome labels must be unique",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// ASSIGNMENT MATRIX
// ============================================================================

/// Classical readout assignment matrix.
///
/// The semantic orientation is:
///
///     probabilities[true_outcome][observed_outcome]
///
/// and therefore:
///
///     P(observed | true)
///
/// Each row corresponds to one true/prepared outcome.
///
/// The number of rows is the number of true outcomes and the number of
/// columns is the number of observed outcomes.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadoutAssignmentMatrix {
    true_outcomes: ReadoutOutcomeSpace,
    observed_outcomes: ReadoutOutcomeSpace,
    probabilities: Vec<f64>,
}

impl ReadoutAssignmentMatrix {
    /// Creates an assignment matrix.
    ///
    /// `probabilities` must contain exactly:
    ///
    ///     true_outcomes.len() * observed_outcomes.len()
    ///
    /// values in row-major order.
    pub fn new(
        true_outcomes: ReadoutOutcomeSpace,
        observed_outcomes: ReadoutOutcomeSpace,
        probabilities: Vec<f64>,
    ) -> ZqnResult<Self> {
        true_outcomes.validate()?;
        observed_outcomes.validate()?;

        let expected = checked_matrix_size(
            true_outcomes.len(),
            observed_outcomes.len(),
        )?;

        if probabilities.len() != expected {
            return Err(structure_error(
                ZqnErrorCode::ChannelDimensionMismatch,
                format!(
                    "readout assignment matrix contains {} values but {} are required",
                    probabilities.len(),
                    expected
                ),
            ));
        }

        validate_probabilities(&probabilities)?;

        Ok(Self {
            true_outcomes,
            observed_outcomes,
            probabilities,
        })
    }

    /// Creates an identity/perfect assignment matrix when the true and
    /// observed alphabets are identical.
    pub fn identity(outcomes: ReadoutOutcomeSpace) -> ZqnResult<Self> {
        outcomes.validate()?;

        let dimension = outcomes.len();
        let size = checked_matrix_size(dimension, dimension)?;
        let mut probabilities = vec![0.0; size];

        for index in 0..dimension {
            let position = index
                .checked_mul(dimension)
                .and_then(|offset| offset.checked_add(index))
                .ok_or_else(|| structure_error(
                    ZqnErrorCode::SizeOverflow,
                    "readout identity matrix index overflow",
                ))?;

            probabilities[position] = 1.0;
        }

        Self::new(
            outcomes.clone(),
            outcomes,
            probabilities,
        )
    }

    /// Returns the true/prepared outcome space.
    #[must_use]
    pub fn true_outcomes(&self) -> &ReadoutOutcomeSpace {
        &self.true_outcomes
    }

    /// Returns the observed outcome space.
    #[must_use]
    pub fn observed_outcomes(&self) -> &ReadoutOutcomeSpace {
        &self.observed_outcomes
    }

    /// Returns the row-major probability storage.
    ///
    /// The returned slice is read-only.
    #[must_use]
    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }

    /// Returns the number of true outcomes.
    #[must_use]
    pub fn true_outcome_count(&self) -> usize {
        self.true_outcomes.len()
    }

    /// Returns the number of observed outcomes.
    #[must_use]
    pub fn observed_outcome_count(&self) -> usize {
        self.observed_outcomes.len()
    }

    /// Returns the assignment probability by indices.
    ///
    /// Semantics:
    ///
    ///     P(observed | true)
    #[must_use]
    pub fn probability(
        &self,
        true_index: usize,
        observed_index: usize,
    ) -> Option<f64> {
        if true_index >= self.true_outcome_count()
            || observed_index >= self.observed_outcome_count()
        {
            return None;
        }

        let index = true_index
            .checked_mul(self.observed_outcome_count())?
            .checked_add(observed_index)?;

        self.probabilities.get(index).copied()
    }

    /// Returns an assignment probability by outcome labels.
    #[must_use]
    pub fn probability_for_labels(
        &self,
        true_label: &str,
        observed_label: &str,
    ) -> Option<f64> {
        let true_index = self.true_outcomes.index_of(true_label)?;
        let observed_index = self.observed_outcomes.index_of(observed_label)?;

        self.probability(true_index, observed_index)
    }

    /// Returns a complete row for a true outcome.
    #[must_use]
    pub fn row(&self, true_index: usize) -> Option<&[f64]> {
        if true_index >= self.true_outcome_count() {
            return None;
        }

        let columns = self.observed_outcome_count();

        let start = true_index.checked_mul(columns)?;
        let end = start.checked_add(columns)?;

        self.probabilities.get(start..end)
    }

    /// Validates numerical and dimensional invariants without requiring
    /// normalization.
    pub fn validate_structure(&self) -> ZqnResult<()> {
        self.true_outcomes.validate()?;
        self.observed_outcomes.validate()?;

        let expected = checked_matrix_size(
            self.true_outcome_count(),
            self.observed_outcome_count(),
        )?;

        if self.probabilities.len() != expected {
            return Err(structure_error(
                ZqnErrorCode::ChannelDimensionMismatch,
                "readout assignment matrix dimensions do not match storage",
            ));
        }

        validate_probabilities(&self.probabilities)?;

        Ok(())
    }

    /// Validates the complete assignment model.
    ///
    /// `tolerance` is applied independently to every true-outcome row.
    pub fn validate_normalized(&self, tolerance: f64) -> ZqnResult<()> {
        self.validate_structure()?;
        validate_tolerance(tolerance)?;

        for true_index in 0..self.true_outcome_count() {
            let row = self
                .row(true_index)
                .ok_or_else(|| structure_error(
                    ZqnErrorCode::ChannelDimensionMismatch,
                    "unable to access readout assignment matrix row",
                ))?;

            let sum = compensated_sum(row);

            if !sum.is_finite() {
                return Err(numerical_error(
                    "readout assignment row normalization produced a non-finite value",
                ));
            }

            if (sum - 1.0).abs() > tolerance {
                return Err(structure_error(
                    ZqnErrorCode::DistributionNotNormalized,
                    format!(
                        "readout assignment row {} sums to {:.17e}, outside tolerance {:.17e}",
                        true_index,
                        sum,
                        tolerance
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Returns the maximum absolute deviation from row normalization.
    pub fn normalization_error(&self) -> ZqnResult<f64> {
        self.validate_structure()?;

        let mut maximum = 0.0_f64;

        for true_index in 0..self.true_outcome_count() {
            let row = self
                .row(true_index)
                .ok_or_else(|| structure_error(
                    ZqnErrorCode::ChannelDimensionMismatch,
                    "unable to access readout assignment matrix row",
                ))?;

            let sum = compensated_sum(row);

            if !sum.is_finite() {
                return Err(numerical_error(
                    "readout assignment normalization error is non-finite",
                ));
            }

            maximum = maximum.max((sum - 1.0).abs());
        }

        Ok(maximum)
    }

    /// Applies the assignment matrix to a true-outcome probability vector.
    ///
    /// Input:
    ///
    ///     P(true)
    ///
    /// Output:
    ///
    ///     P(observed)
    ///
    /// The operation does not silently renormalize.
    pub fn apply(
        &self,
        true_distribution: &[f64],
    ) -> ZqnResult<Vec<f64>> {
        self.validate_structure()?;

        if true_distribution.len() != self.true_outcome_count() {
            return Err(structure_error(
                ZqnErrorCode::ChannelDimensionMismatch,
                format!(
                    "true distribution contains {} entries but {} are required",
                    true_distribution.len(),
                    self.true_outcome_count()
                ),
            ));
        }

        validate_probabilities(true_distribution)?;

        let observed_count = self.observed_outcome_count();
        let mut result = vec![0.0; observed_count];

        for true_index in 0..self.true_outcome_count() {
            let true_probability = true_distribution[true_index];

            let row = self
                .row(true_index)
                .ok_or_else(|| structure_error(
                    ZqnErrorCode::ChannelDimensionMismatch,
                    "unable to access readout assignment row",
                ))?;

            for observed_index in 0..observed_count {
                let contribution = true_probability * row[observed_index];

                if !contribution.is_finite() {
                    return Err(numerical_error(
                        "readout assignment application produced a non-finite value",
                    ));
                }

                result[observed_index] += contribution;

                if !result[observed_index].is_finite() {
                    return Err(numerical_error(
                        "readout assignment accumulation produced a non-finite value",
                    ));
                }
            }
        }

        Ok(result)
    }
}

// ============================================================================
// CALIBRATION VALIDITY
// ============================================================================

/// Temporal validity interval for a readout calibration.
///
/// The interval is half-open:
///
///     [not_before_ns, not_after_ns)
///
/// `None` means unbounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadoutCalibrationValidity {
    not_before_ns: Option<i128>,
    not_after_ns: Option<i128>,
}

impl ReadoutCalibrationValidity {
    /// Creates a validity interval.
    pub fn new(
        not_before_ns: Option<i128>,
        not_after_ns: Option<i128>,
    ) -> ZqnResult<Self> {
        if let (Some(start), Some(end)) = (not_before_ns, not_after_ns) {
            if start >= end {
                return Err(structure_error(
                    ZqnErrorCode::InvalidCalibration,
                    "readout calibration validity interval must have start < end",
                ));
            }
        }

        Ok(Self {
            not_before_ns,
            not_after_ns,
        })
    }

    /// Creates an unbounded validity interval.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            not_before_ns: None,
            not_after_ns: None,
        }
    }

    /// Returns the lower bound.
    #[must_use]
    pub const fn not_before_ns(&self) -> Option<i128> {
        self.not_before_ns
    }

    /// Returns the upper bound.
    #[must_use]
    pub const fn not_after_ns(&self) -> Option<i128> {
        self.not_after_ns
    }

    /// Returns whether a timestamp belongs to this interval.
    #[must_use]
    pub fn contains(&self, timestamp_ns: i128) -> bool {
        let after_start = self
            .not_before_ns
            .map_or(true, |start| timestamp_ns >= start);

        let before_end = self
            .not_after_ns
            .map_or(true, |end| timestamp_ns < end);

        after_start && before_end
    }

    /// Validates the interval.
    pub fn validate(&self) -> ZqnResult<()> {
        if let (Some(start), Some(end)) =
            (self.not_before_ns, self.not_after_ns)
        {
            if start >= end {
                return Err(structure_error(
                    ZqnErrorCode::InvalidCalibration,
                    "readout calibration validity interval must have start < end",
                ));
            }
        }

        Ok(())
    }
}

impl Default for ReadoutCalibrationValidity {
    fn default() -> Self {
        Self::unbounded()
    }
}

// ============================================================================
// STATUS
// ============================================================================

/// Semantic lifecycle status of a readout calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReadoutCalibrationStatus {
    /// Calibration has passed structural and numerical validation.
    Valid,

    /// Calibration has been received but not yet validated.
    Unvalidated,

    /// Calibration is known to be stale.
    Stale,

    /// Calibration is known to be invalid.
    Invalid,

    /// Calibration is intentionally disabled.
    Disabled,

    /// Calibration has been superseded by another revision.
    Superseded,
}

impl Default for ReadoutCalibrationStatus {
    fn default() -> Self {
        Self::Unvalidated
    }
}

impl fmt::Display for ReadoutCalibrationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => formatter.write_str("valid"),
            Self::Unvalidated => formatter.write_str("unvalidated"),
            Self::Stale => formatter.write_str("stale"),
            Self::Invalid => formatter.write_str("invalid"),
            Self::Disabled => formatter.write_str("disabled"),
            Self::Superseded => formatter.write_str("superseded"),
        }
    }
}

// ============================================================================
// REVISION
// ============================================================================

/// Semantic revision of a readout calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReadoutCalibrationRevision {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ReadoutCalibrationRevision {
    /// Creates a revision.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major revision.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor revision.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch revision.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for ReadoutCalibrationRevision {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for ReadoutCalibrationRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// RESOURCE LIMITS
// ============================================================================

/// Explicit resource policy for readout-calibration validation.
///
/// `None` means that this module does not impose a finite limit for that
/// quantity.
///
/// These values are execution/resource-safety policy, not semantic limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadoutCalibrationLimits {
    /// Maximum number of resources allowed by the caller.
    pub max_resources: Option<usize>,

    /// Maximum number of true outcomes.
    pub max_true_outcomes: Option<usize>,

    /// Maximum number of observed outcomes.
    pub max_observed_outcomes: Option<usize>,

    /// Maximum number of assignment-matrix elements.
    pub max_matrix_elements: Option<usize>,

    /// Maximum number of referenced calibration/noise parameters.
    pub max_parameter_references: Option<usize>,
}

impl Default for ReadoutCalibrationLimits {
    fn default() -> Self {
        Self {
            max_resources: None,
            max_true_outcomes: None,
            max_observed_outcomes: None,
            max_matrix_elements: None,
            max_parameter_references: None,
        }
    }
}

impl ReadoutCalibrationLimits {
    /// Creates unlimited semantic validation limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_resources: None,
            max_true_outcomes: None,
            max_observed_outcomes: None,
            max_matrix_elements: None,
            max_parameter_references: None,
        }
    }

    /// Validates the limit configuration itself.
    pub fn validate(&self) -> ZqnResult<()> {
        // usize options cannot be negative, so the structural validation is
        // intentionally empty today. Keeping this method allows future
        // policy fields to be added without changing consumer contracts.
        Ok(())
    }
}

// ============================================================================
// READOUT CALIBRATION
// ============================================================================

/// Canonical backend-independent readout calibration.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadoutCalibration {
    calibration_id: CalibrationId,
    scope: ReadoutResourceScope,
    assignment: ReadoutAssignmentMatrix,
    validity: ReadoutCalibrationValidity,
    revision: ReadoutCalibrationRevision,
    status: ReadoutCalibrationStatus,
    parameter_references: Vec<NoiseParameterId>,
    provenance_reference: Option<CalibrationId>,
    label: Option<String>,
}

impl ReadoutCalibration {
    /// Creates a readout calibration.
    ///
    /// The constructor performs structural validation but does not require
    /// normalized rows. Call [`Self::validate`] when a complete probability
    /// model is required.
    #[must_use]
    pub fn new(
        calibration_id: CalibrationId,
        scope: ReadoutResourceScope,
        assignment: ReadoutAssignmentMatrix,
    ) -> ZqnResult<Self> {
        scope.validate()?;
        assignment.validate_structure()?;

        Ok(Self {
            calibration_id,
            scope,
            assignment,
            validity: ReadoutCalibrationValidity::default(),
            revision: ReadoutCalibrationRevision::default(),
            status: ReadoutCalibrationStatus::Unvalidated,
            parameter_references: Vec::new(),
            provenance_reference: None,
            label: None,
        })
    }

    /// Returns the stable calibration identity.
    #[must_use]
    pub const fn calibration_id(&self) -> CalibrationId {
        self.calibration_id
    }

    /// Returns the resource scope.
    #[must_use]
    pub fn scope(&self) -> &ReadoutResourceScope {
        &self.scope
    }

    /// Returns the assignment matrix.
    #[must_use]
    pub fn assignment(&self) -> &ReadoutAssignmentMatrix {
        &self.assignment
    }

    /// Returns the validity interval.
    #[must_use]
    pub const fn validity(&self) -> &ReadoutCalibrationValidity {
        &self.validity
    }

    /// Returns the semantic revision.
    #[must_use]
    pub const fn revision(&self) -> ReadoutCalibrationRevision {
        self.revision
    }

    /// Returns the semantic status.
    #[must_use]
    pub const fn status(&self) -> ReadoutCalibrationStatus {
        self.status
    }

    /// Returns all referenced calibration/noise parameter IDs.
    #[must_use]
    pub fn parameter_references(&self) -> &[NoiseParameterId] {
        &self.parameter_references
    }

    /// Returns the optional provenance/calibration lineage reference.
    #[must_use]
    pub const fn provenance_reference(&self) -> Option<CalibrationId> {
        self.provenance_reference
    }

    /// Returns the optional human-readable label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns a copy with a new validity interval.
    pub fn with_validity(
        mut self,
        validity: ReadoutCalibrationValidity,
    ) -> ZqnResult<Self> {
        validity.validate()?;
        self.validity = validity;
        Ok(self)
    }

    /// Returns a copy with a new revision.
    #[must_use]
    pub fn with_revision(
        mut self,
        revision: ReadoutCalibrationRevision,
    ) -> Self {
        self.revision = revision;
        self
    }

    /// Returns a copy with a new semantic status.
    #[must_use]
    pub fn with_status(
        mut self,
        status: ReadoutCalibrationStatus,
    ) -> Self {
        self.status = status;
        self
    }

    /// Returns a copy with an optional provenance reference.
    #[must_use]
    pub fn with_provenance_reference(
        mut self,
        provenance_reference: Option<CalibrationId>,
    ) -> Self {
        self.provenance_reference = provenance_reference;
        self
    }

    /// Returns a copy with a human-readable label.
    pub fn with_label(
        mut self,
        label: impl Into<String>,
    ) -> ZqnResult<Self> {
        let label = label.into();

        if label.trim().is_empty() {
            return Err(structure_error(
                ZqnErrorCode::InvalidIdentifier,
                "readout calibration label must not be empty",
            ));
        }

        self.label = Some(label);
        Ok(self)
    }

    /// Returns a copy with an additional calibration/noise parameter reference.
    pub fn with_parameter_reference(
        mut self,
        parameter_id: NoiseParameterId,
    ) -> ZqnResult<Self> {
        if self.parameter_references.contains(&parameter_id) {
            return Err(structure_error(
                ZqnErrorCode::DuplicateIdentifier,
                "readout calibration parameter reference is duplicated",
            ));
        }

        self.parameter_references.push(parameter_id);
        Ok(self)
    }

    /// Returns a copy with multiple parameter references.
    pub fn with_parameter_references(
        mut self,
        parameter_ids: Vec<NoiseParameterId>,
    ) -> ZqnResult<Self> {
        for parameter_id in parameter_ids {
            if self.parameter_references.contains(&parameter_id) {
                return Err(structure_error(
                    ZqnErrorCode::DuplicateIdentifier,
                    "readout calibration parameter reference is duplicated",
                ));
            }

            self.parameter_references.push(parameter_id);
        }

        Ok(self)
    }

    /// Validates the complete readout calibration using the default
    /// normalization tolerance and unlimited semantic resource limits.
    pub fn validate(&self) -> ZqnResult<()> {
        self.validate_with_limits(
            &ReadoutCalibrationLimits::unlimited(),
            DEFAULT_NORMALIZATION_TOLERANCE,
        )
    }

    /// Validates the calibration using explicit resource limits and numerical
    /// tolerance.
    pub fn validate_with_limits(
        &self,
        limits: &ReadoutCalibrationLimits,
        tolerance: f64,
    ) -> ZqnResult<()> {
        limits.validate()?;
        validate_tolerance(tolerance)?;

        self.scope.validate()?;
        self.validity.validate()?;

        let resources = self.scope.resource_count();

        enforce_optional_limit(
            resources,
            limits.max_resources,
            "readout calibration resource count",
        )?;

        let true_outcomes = self.assignment.true_outcome_count();
        let observed_outcomes = self.assignment.observed_outcome_count();

        enforce_optional_limit(
            true_outcomes,
            limits.max_true_outcomes,
            "readout calibration true-outcome count",
        )?;

        enforce_optional_limit(
            observed_outcomes,
            limits.max_observed_outcomes,
            "readout calibration observed-outcome count",
        )?;

        let matrix_elements = checked_matrix_size(
            true_outcomes,
            observed_outcomes,
        )?;

        enforce_optional_limit(
            matrix_elements,
            limits.max_matrix_elements,
            "readout calibration matrix element count",
        )?;

        enforce_optional_limit(
            self.parameter_references.len(),
            limits.max_parameter_references,
            "readout calibration parameter-reference count",
        )?;

        if self.parameter_references.len()
            != unique_count(&self.parameter_references)
        {
            return Err(structure_error(
                ZqnErrorCode::DuplicateIdentifier,
                "readout calibration contains duplicate parameter references",
            ));
        }

        self.assignment.validate_normalized(tolerance)?;

        match self.status {
            ReadoutCalibrationStatus::Valid => {}
            ReadoutCalibrationStatus::Unvalidated
            | ReadoutCalibrationStatus::Stale
            | ReadoutCalibrationStatus::Invalid
            | ReadoutCalibrationStatus::Disabled
            | ReadoutCalibrationStatus::Superseded => {
                // Status does not invalidate mathematical structure. A
                // consumer decides whether a non-Valid status may be used.
            }
        }

        Ok(())
    }

    /// Returns whether the calibration is temporally applicable.
    #[must_use]
    pub fn is_valid_at(&self, timestamp_ns: i128) -> bool {
        self.validity.contains(timestamp_ns)
    }

    /// Returns whether this calibration is semantically usable according to
    /// status and validity at the supplied timestamp.
    #[must_use]
    pub fn is_usable_at(&self, timestamp_ns: i128) -> bool {
        self.status == ReadoutCalibrationStatus::Valid
            && self.validity.contains(timestamp_ns)
    }

    /// Returns the assignment probability:
    ///
    ///     P(observed | true)
    #[must_use]
    pub fn assignment_probability(
        &self,
        true_index: usize,
        observed_index: usize,
    ) -> Option<f64> {
        self.assignment
            .probability(true_index, observed_index)
    }

    /// Returns the assignment probability using outcome labels.
    #[must_use]
    pub fn assignment_probability_for_labels(
        &self,
        true_label: &str,
        observed_label: &str,
    ) -> Option<f64> {
        self.assignment
            .probability_for_labels(true_label, observed_label)
    }

    /// Applies the readout assignment model to a true probability
    /// distribution.
    pub fn apply(
        &self,
        true_distribution: &[f64],
    ) -> ZqnResult<Vec<f64>> {
        self.assignment.apply(true_distribution)
    }

    /// Returns the number of explicitly represented quantum resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.scope.resource_count()
    }

    /// Returns the number of true outcomes.
    #[must_use]
    pub fn true_outcome_count(&self) -> usize {
        self.assignment.true_outcome_count()
    }

    /// Returns the number of observed outcomes.
    #[must_use]
    pub fn observed_outcome_count(&self) -> usize {
        self.assignment.observed_outcome_count()
    }

    /// Returns the maximum absolute normalization error.
    pub fn normalization_error(&self) -> ZqnResult<f64> {
        self.assignment.normalization_error()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn structure_error(
    code: ZqnErrorCode,
    message: impl Into<String>,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Calibration,
        code,
        message.into(),
    )
}

fn numerical_error(message: impl Into<String>) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Calibration,
        ZqnErrorCode::InvalidEstimate,
        message.into(),
    )
}

fn checked_matrix_size(
    rows: usize,
    columns: usize,
) -> ZqnResult<usize> {
    rows.checked_mul(columns).ok_or_else(|| {
        structure_error(
            ZqnErrorCode::SizeOverflow,
            "readout assignment matrix dimension multiplication overflowed",
        )
    })
}

fn validate_probability(value: f64) -> ZqnResult<()> {
    if !value.is_finite() {
        return Err(ZqnError::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::NonFiniteProbability,
            "readout probability must be finite",
        ));
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(ZqnError::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::InvalidProbability,
            "readout probability must be within [0, 1]",
        ));
    }

    Ok(())
}

fn validate_probabilities(values: &[f64]) -> ZqnResult<()> {
    for &value in values {
        validate_probability(value)?;
    }

    Ok(())
}

fn validate_tolerance(tolerance: f64) -> ZqnResult<()> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ZqnError::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::InvalidProbability,
            "readout normalization tolerance must be finite and non-negative",
        ));
    }

    Ok(())
}

fn enforce_optional_limit(
    actual: usize,
    limit: Option<usize>,
    description: &str,
) -> ZqnResult<()> {
    if let Some(limit) = limit {
        if actual > limit {
            return Err(ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::LimitExceeded,
                format!(
                    "{} {} exceeds configured limit {}",
                    description,
                    actual,
                    limit
                ),
            ));
        }
    }

    Ok(())
}

fn contains_duplicate_values<T>(values: &[T]) -> bool
where
    T: PartialEq,
{
    for index in 0..values.len() {
        if values[index + 1..].contains(&values[index]) {
            return true;
        }
    }

    false
}

fn unique_count<T>(values: &[T]) -> usize
where
    T: PartialEq,
{
    let mut unique = 0usize;

    for index in 0..values.len() {
        if !values[..index].contains(&values[index]) {
            unique = unique.saturating_add(1);
        }
    }

    unique
}

/// Performs deterministic compensated summation.
///
/// This reduces avoidable floating-point summation error without introducing
/// a particular external numerical dependency.
fn compensated_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;

    for &value in values {
        let corrected = value - compensation;
        let next = sum + corrected;
        compensation = (next - sum) - corrected;
        sum = next;
    }

    sum
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn outcomes(labels: &[&str]) -> ReadoutOutcomeSpace {
        ReadoutOutcomeSpace::from_labels(
            labels.iter().copied(),
        )
        .expect("test outcome space should be valid")
    }

    fn matrix_2x2(
        a00: f64,
        a01: f64,
        a10: f64,
        a11: f64,
    ) -> ReadoutAssignmentMatrix {
        ReadoutAssignmentMatrix::new(
            outcomes(&["0", "1"]),
            outcomes(&["0", "1"]),
            vec![a00, a01, a10, a11],
        )
        .expect("test matrix should be structurally valid")
    }

    #[test]
    fn identity_assignment_is_normalized() {
        let matrix =
            ReadoutAssignmentMatrix::identity(outcomes(&["0", "1"]))
                .expect("identity matrix should construct");

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("identity matrix should be normalized");
    }

    #[test]
    fn asymmetric_readout_is_supported() {
        let matrix = matrix_2x2(
            0.99,
            0.01,
            0.08,
            0.92,
        );

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("asymmetric matrix should be valid");

        assert_eq!(
            matrix.probability_for_labels("0", "1"),
            Some(0.01)
        );

        assert_eq!(
            matrix.probability_for_labels("1", "0"),
            Some(0.08)
        );
    }

    #[test]
    fn arbitrary_outcome_alphabet_is_supported() {
        let true_space = outcomes(&["ground", "excited", "leaked"]);
        let observed_space = outcomes(&["g", "e", "l"]);

        let matrix = ReadoutAssignmentMatrix::new(
            true_space,
            observed_space,
            vec![
                0.98, 0.01, 0.01,
                0.02, 0.96, 0.02,
                0.03, 0.04, 0.93,
            ],
        )
        .expect("arbitrary outcome matrix should construct");

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("arbitrary outcome matrix should normalize");
    }

    #[test]
    fn different_true_and_observed_dimensions_are_supported() {
        let matrix = ReadoutAssignmentMatrix::new(
            outcomes(&["0", "1", "2"]),
            outcomes(&["0", "1"]),
            vec![
                0.9, 0.1,
                0.2, 0.8,
                0.6, 0.4,
            ],
        )
        .expect("rectangular assignment matrix should construct");

        assert_eq!(matrix.true_outcome_count(), 3);
        assert_eq!(matrix.observed_outcome_count(), 2);

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("rectangular matrix should normalize");
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let result = ReadoutAssignmentMatrix::new(
            outcomes(&["0"]),
            outcomes(&["0"]),
            vec![f64::NAN],
        );

        assert!(result.is_err());
    }

    #[test]
    fn negative_probability_is_rejected() {
        let result = ReadoutAssignmentMatrix::new(
            outcomes(&["0"]),
            outcomes(&["0"]),
            vec![-0.1],
        );

        assert!(result.is_err());
    }

    #[test]
    fn probability_above_one_is_rejected() {
        let result = ReadoutAssignmentMatrix::new(
            outcomes(&["0"]),
            outcomes(&["0"]),
            vec![1.1],
        );

        assert!(result.is_err());
    }

    #[test]
    fn non_normalized_matrix_is_rejected_by_full_validation() {
        let matrix = matrix_2x2(
            0.5,
            0.1,
            0.4,
            0.4,
        );

        assert!(
            matrix
                .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn distribution_application_is_deterministic() {
        let matrix = matrix_2x2(
            0.9,
            0.1,
            0.2,
            0.8,
        );

        let result = matrix
            .apply(&[0.25, 0.75])
            .expect("distribution application should succeed");

        assert!((result[0] - 0.375).abs() < 1.0e-15);
        assert!((result[1] - 0.625).abs() < 1.0e-15);
    }

    #[test]
    fn outcome_labels_are_unique() {
        assert!(
            ReadoutOutcomeSpace::from_labels(["0", "0"]).is_err()
        );
    }

    #[test]
    fn empty_outcome_space_is_rejected() {
        assert!(
            ReadoutOutcomeSpace::from_labels(
                core::iter::empty::<&str>()
            )
            .is_err()
        );
    }

    #[test]
    fn empty_logical_scope_is_rejected() {
        let result =
            ReadoutResourceScope::logical_qubits(Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn empty_physical_scope_is_rejected() {
        let result =
            ReadoutResourceScope::physical_qubits(Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn device_wide_scope_has_no_machine_size_assumption() {
        let scope = ReadoutResourceScope::device_wide();

        assert_eq!(scope.resource_count(), 1);
        scope
            .validate()
            .expect("device-wide scope should be valid");
    }

    #[test]
    fn validity_interval_is_half_open() {
        let validity =
            ReadoutCalibrationValidity::new(
                Some(100),
                Some(200),
            )
            .expect("validity interval should construct");

        assert!(!validity.contains(99));
        assert!(validity.contains(100));
        assert!(validity.contains(199));
        assert!(!validity.contains(200));
    }

    #[test]
    fn invalid_validity_interval_is_rejected() {
        assert!(
            ReadoutCalibrationValidity::new(
                Some(200),
                Some(100),
            )
            .is_err()
        );
    }

    #[test]
    fn resource_limits_are_explicit_policy() {
        let matrix = matrix_2x2(
            0.9,
            0.1,
            0.1,
            0.9,
        );

        let limits = ReadoutCalibrationLimits {
            max_resources: Some(1),
            max_true_outcomes: Some(2),
            max_observed_outcomes: Some(2),
            max_matrix_elements: Some(4),
            max_parameter_references: None,
        };

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("matrix itself should be valid");

        // The limits object itself remains valid even though the resource
        // policy may later reject a calibration using more resources.
        limits
            .validate()
            .expect("limit configuration should be valid");
    }

    #[test]
    fn normalization_error_is_zero_for_identity() {
        let matrix =
            ReadoutAssignmentMatrix::identity(outcomes(&["0", "1", "2"]))
                .expect("identity should construct");

        let error = matrix
            .normalization_error()
            .expect("normalization error should calculate");

        assert_eq!(error, 0.0);
    }

    #[test]
    fn matrix_shape_is_checked() {
        let result = ReadoutAssignmentMatrix::new(
            outcomes(&["0", "1"]),
            outcomes(&["0", "1"]),
            vec![1.0, 0.0],
        );

        assert!(result.is_err());
    }

    #[test]
    fn arbitrary_matrix_size_is_data_driven() {
        let dimension = 16usize;
        let outcomes = (0..dimension)
            .map(|index| format!("state-{index}"))
            .collect::<Vec<_>>();

        let space = ReadoutOutcomeSpace::new(
            outcomes.clone(),
        )
        .expect("generated outcome space should be valid");

        let mut probabilities =
            vec![0.0; dimension * dimension];

        for index in 0..dimension {
            probabilities[index * dimension + index] = 1.0;
        }

        let matrix = ReadoutAssignmentMatrix::new(
            space.clone(),
            space,
            probabilities,
        )
        .expect("generated matrix should construct");

        matrix
            .validate_normalized(DEFAULT_NORMALIZATION_TOLERANCE)
            .expect("generated identity matrix should normalize");
    }

    #[test]
    fn resource_scope_accepts_named_future_modalities() {
        let scope =
            ReadoutResourceScope::named(
                "photonic.mode",
                "mode-0",
            )
            .expect("named scope should construct");

        assert_eq!(scope.resource_count(), 1);
    }

    #[test]
    fn calibration_starts_unvalidated() {
        // CalibrationId construction is intentionally delegated to the
        // canonical ZQN ID implementation. This test checks the semantic
        // default without constructing an implementation-specific ID.
        assert_eq!(
            ReadoutCalibrationStatus::default(),
            ReadoutCalibrationStatus::Unvalidated
        );
    }

    #[test]
    fn tolerance_must_be_finite() {
        assert!(
            validate_tolerance(f64::NAN).is_err()
        );

        assert!(
            validate_tolerance(f64::INFINITY).is_err()
        );
    }

    #[test]
    fn tolerance_must_not_be_negative() {
        assert!(
            validate_tolerance(-1.0).is_err()
        );
    }
}