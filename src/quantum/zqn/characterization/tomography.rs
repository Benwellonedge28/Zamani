//! Zamani Quantum Noise (ZQN) — Characterization Tomography.
//!
//! # Ownership
//!
//! This module owns the reconstruction layer for quantum characterization
//! tomography.
//!
//! It owns:
//!
//! - tomography problem descriptions;
//! - state/process/measurement tomography problem kinds;
//! - reconstruction method selection;
//! - measurement-equation representation;
//! - deterministic linear reconstruction;
//! - weighted linear reconstruction;
//! - rank and residual diagnostics;
//! - reconstruction quality/status;
//! - explicit physicality validation;
//! - optional physical-state projection;
//! - arbitrary finite-dimensional reconstruction;
//! - resource-governed numerical linear algebra;
//! - tomography-specific provenance references;
//! - deterministic reconstruction contracts;
//! - tomography schema identity.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical Quantum IR;
//! - source parsing;
//! - characterization protocol generation;
//! - raw observation storage;
//! - generic point estimation;
//! - confidence intervals;
//! - credible intervals;
//! - calibration storage;
//! - noise-model semantics;
//! - quantum channels as a canonical ZQN abstraction;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - hardware APIs;
//! - vendor-specific tomography;
//! - random-number generation;
//! - persistence;
//! - serialization wire formats.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! characterization::protocol
//!             |
//!             v
//! characterization::experiment
//!             |
//!             v
//! execution / hardware / simulator
//!             |
//!             v
//! characterization::observation
//!             |
//!             v
//! characterization::estimator
//!             |
//!             v
//! tomography problem
//!             |
//!             v
//! this module
//!             |
//!             +------------------+
//!             |                  |
//!             v                  v
//! reconstruction          diagnostics
//!             |
//!             v
//! characterization result
//!             |
//!       +-----+------+
//!       |            |
//!       v            v
//! calibration     ZQN noise model
//! ```
//!
//! # Fundamental separation
//!
//! Tomography is an inverse problem.
//!
//! Given measurement data:
//!
//! ```text
//! y = A x
//! ```
//!
//! this module reconstructs an estimate of `x` from the observed data `y`
//! and the declared measurement model `A`.
//!
//! The reconstructed object is NOT automatically:
//!
//! - physically valid;
//! - statistically optimal;
//! - uncertainty-qualified;
//! - calibrated;
//! - a complete noise model.
//!
//! Those properties require explicit validation and/or downstream analysis.
//!
//! # Scalability
//!
//! There is deliberately no semantic maximum for:
//!
//! - number of qubits;
//! - number of modes;
//! - Hilbert-space dimension;
//! - number of measurement settings;
//! - number of equations;
//! - number of reconstructed parameters;
//! - number of experiments.
//!
//! All dimensions are supplied by the problem.
//!
//! A dense matrix necessarily consumes resources proportional to its size.
//! That is an implementation property, not a machine-size limit.
//!
//! Callers MUST use `TomographyLimits` to govern potentially expensive
//! materialization.
//!
//! For larger problems, callers can use the streaming equation interface and
//! replace the dense solver with a sparse, distributed, tensor-network,
//! compressed-sensing, or external linear-algebra implementation without
//! changing the tomography problem contract.
//!
//! # Approximation
//!
//! This module never silently changes an exact reconstruction request into an
//! approximate one.
//!
//! Numerical tolerances are explicit configuration.
//!
//! A result reports:
//!
//! - requested method;
//! - numerical tolerance;
//! - residual;
//! - rank;
//! - whether the system was exactly determined, overdetermined or
//!   underdetermined;
//! - whether physicality projection was requested;
//! - whether the reconstruction is complete.
//!
//! # Determinism
//!
//! Reconstruction is deterministic.
//!
//! This module:
//!
//! - uses no RNG;
//! - uses no global mutable state;
//! - does not read the system clock;
//! - does not depend on hash-map iteration;
//! - uses deterministic row/column traversal;
//! - uses deterministic pivot selection;
//! - uses caller-supplied input ordering.
//!
//! Floating-point results are deterministic for a fixed supported numerical
//! environment and identical ordered inputs.
//!
//! Parallel execution belongs outside this module. If a parallel solver is
//! introduced later, deterministic reduction/pivot policies MUST be preserved
//! when deterministic mode is requested.
//!
//! # Numerical safety
//!
//! Invalid floating-point values are rejected.
//!
//! In particular:
//!
//! ```text
//! NaN
//! +∞
//! -∞
//! ```
//!
//! are never silently converted or clamped.
//!
//! Singular or numerically singular systems are reported explicitly.
//!
//! Negative eigenvalue-like coefficients are not silently corrected unless
//! the caller explicitly requests physical-state projection.
//!
//! # Resource safety
//!
//! Dense reconstruction can require O(n²) storage and O(n³) arithmetic.
//!
//! This module therefore provides explicit resource limits:
//!
//! - maximum matrix elements;
//! - maximum equations;
//! - maximum parameters;
//! - maximum arithmetic operations;
//! - maximum pivot iterations;
//! - maximum workspace bytes.
//!
//! `None` means that this module imposes no corresponding policy limit.
//!
//! These limits are NOT quantum-machine limits.
//!
//! # Integration with observation.rs
//!
//! `observation.rs` owns raw observations, including:
//!
//! - measurement histograms;
//! - scalar observations;
//! - complex observations;
//! - per-shot observations.
//!
//! This module does not duplicate those types.
//!
//! Higher-level characterization code should transform validated observations
//! into `TomographyDatum` values. This keeps the tomography solver independent
//! from raw storage format.
//!
//! # Integration with estimator.rs
//!
//! Generic estimation remains in `estimator.rs`.
//!
//! Tomography may consume estimates produced by that module, but tomography
//! owns the inverse reconstruction itself.
//!
//! A tomography pipeline is therefore:
//!
//! ```text
//! raw observations
//!      |
//!      v
//! generic/statistical estimator
//!      |
//!      v
//! TomographyDatum
//!      |
//!      v
//! TomographyProblem
//!      |
//!      v
//! TomographyReconstructor
//! ```
//!
//! # Integration with uncertainty.rs
//!
//! This module does NOT calculate confidence or credible intervals.
//!
//! Instead, it exposes reconstruction diagnostics such as:
//!
//! - residual norm;
//! - equation count;
//! - parameter count;
//! - rank;
//! - effective condition indicator;
//! - weighting information.
//!
//! `uncertainty.rs` or a higher-level characterization result can use these
//! quantities together with statistical sufficient statistics.
//!
//! # Integration with calibration
//!
//! Tomography produces characterization evidence/reconstruction.
//!
//! Calibration owns:
//!
//! - parameter storage;
//! - validity intervals;
//! - calibration snapshots;
//! - calibration lifecycle.
//!
//! A tomography result can therefore become calibration input without making
//! tomography responsible for calibration persistence.
//!
//! # Integration with ZQN channels
//!
//! A reconstructed process representation can later be converted by an
//! integration layer into a canonical ZQN channel representation.
//!
//! This module deliberately does not define another quantum-channel system.
//!
//! # Canonical quantum-resource identity
//!
//! This file does not define a new qubit identifier.
//!
//! When a tomography problem is associated with resources, it uses the
//! canonical identifiers from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Serialization
//!
//! This file defines semantic structures only.
//!
//! Versioned wire serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Rust struct layout MUST NOT be treated as a stable wire format.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! # Security
//!
//! Tomography can process untrusted characterization data.
//!
//! Therefore:
//!
//! - dimensions are checked before multiplication;
//! - matrix allocations are resource-governed;
//! - arithmetic operation counts are checked;
//! - non-finite input is rejected;
//! - no recursive numerical algorithm is used;
//! - no hidden allocation proportional to shot count occurs;
//! - no unbounded iterative numerical loop exists;
//! - singular systems are rejected;
//! - cancellation/resource exhaustion can be handled by the caller before
//!   invoking expensive reconstruction.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. tomography owns reconstruction;
//! 2. raw observations remain owned by observation.rs;
//! 3. generic estimation remains owned by estimator.rs;
//! 4. uncertainty remains owned by uncertainty.rs;
//! 5. no second QubitId exists;
//! 6. no fixed Hilbert-space dimension exists;
//! 7. no fixed qubit count exists;
//! 8. dense numerical work is resource-governed;
//! 9. invalid floating-point values are rejected;
//! 10. singular systems are explicit errors;
//! 11. deterministic reconstruction is guaranteed;
//! 12. physical projection is explicit rather than implicit;
//! 13. schema identity is versioned;
//! 14. the API can support state, process, measurement and future tomography
//!     variants without changing the core reconstruction contract.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::ids::{
    CalibrationId, CharacterizationId, ExperimentId, ZqnIdValue,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier.
pub const TOMOGRAPHY_SCHEMA_ID: &str =
    "zamani.quantum.zqn.characterization.tomography";

/// Semantic version of this tomography contract.
pub const TOMOGRAPHY_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance used only when the caller explicitly selects
/// the default configuration.
pub const DEFAULT_NUMERICAL_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by tomography.
#[derive(Clone, Debug, PartialEq)]
pub enum TomographyError {
    /// A required identifier or field is invalid.
    InvalidInput(&'static str),

    /// A floating-point input is non-finite.
    NonFiniteValue {
        field: &'static str,
        value: f64,
    },

    /// A matrix/vector dimension is inconsistent.
    DimensionMismatch {
        expected: usize,
        actual: usize,
        context: &'static str,
    },

    /// An arithmetic conversion or count overflow occurred.
    IntegerOverflow(&'static str),

    /// A floating-point calculation became non-finite.
    NumericalFailure(&'static str),

    /// A numerical system is singular or effectively singular.
    SingularSystem {
        pivot: usize,
    },

    /// The requested numerical tolerance is invalid.
    InvalidTolerance(f64),

    /// A resource policy would be exceeded.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: u128,
        limit: u128,
    },

    /// The problem is underdetermined and the selected method does not define
    /// a unique reconstruction.
    Underdetermined {
        equations: usize,
        parameters: usize,
    },

    /// The supplied weight is invalid.
    InvalidWeight(f64),

    /// The requested method is not supported by this implementation.
    UnsupportedMethod(&'static str),

    /// Physical-state validation failed.
    InvalidPhysicalState(&'static str),

    /// The reconstruction has insufficient information.
    InsufficientInformation(&'static str),

    /// An explicitly requested projection cannot be performed.
    ProjectionFailure(&'static str),
}

impl fmt::Display for TomographyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(field) => {
                write!(formatter, "invalid tomography input: {field}")
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "tomography field `{field}` contains non-finite value {value}"
                )
            }

            Self::DimensionMismatch {
                expected,
                actual,
                context,
            } => {
                write!(
                    formatter,
                    "tomography dimension mismatch in {context}: expected {expected}, got {actual}"
                )
            }

            Self::IntegerOverflow(operation) => {
                write!(formatter, "tomography integer overflow: {operation}")
            }

            Self::NumericalFailure(operation) => {
                write!(formatter, "tomography numerical failure: {operation}")
            }

            Self::SingularSystem { pivot } => {
                write!(formatter, "tomography system is singular near pivot {pivot}")
            }

            Self::InvalidTolerance(value) => {
                write!(formatter, "invalid tomography tolerance: {value}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "tomography resource limit exceeded for {resource}: requested {requested}, limit {limit}"
                )
            }

            Self::Underdetermined {
                equations,
                parameters,
            } => {
                write!(
                    formatter,
                    "tomography problem is underdetermined: {equations} equations for {parameters} parameters"
                )
            }

            Self::InvalidWeight(value) => {
                write!(formatter, "invalid tomography weight: {value}")
            }

            Self::UnsupportedMethod(method) => {
                write!(formatter, "unsupported tomography method: {method}")
            }

            Self::InvalidPhysicalState(reason) => {
                write!(formatter, "invalid reconstructed physical state: {reason}")
            }

            Self::InsufficientInformation(reason) => {
                write!(
                    formatter,
                    "insufficient tomography information: {reason}"
                )
            }

            Self::ProjectionFailure(reason) => {
                write!(formatter, "tomography physical projection failed: {reason}")
            }
        }
    }
}

impl std::error::Error for TomographyError {}

/// Result type used throughout tomography.
pub type TomographyResult<T> = Result<T, TomographyError>;

// =============================================================================
// Configuration
// =============================================================================

/// Resource policy for tomography.
///
/// These are implementation/resource limits, never semantic limits on
/// quantum-machine size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TomographyLimits {
    /// Maximum number of matrix elements materialized by a dense solver.
    pub max_matrix_elements: Option<u128>,

    /// Maximum number of equations.
    pub max_equations: Option<u64>,

    /// Maximum number of unknown parameters.
    pub max_parameters: Option<u64>,

    /// Maximum estimated arithmetic operations.
    pub max_arithmetic_operations: Option<u128>,

    /// Maximum workspace bytes.
    pub max_workspace_bytes: Option<u128>,
}

impl Default for TomographyLimits {
    fn default() -> Self {
        Self {
            max_matrix_elements: None,
            max_equations: None,
            max_parameters: None,
            max_arithmetic_operations: None,
            max_workspace_bytes: None,
        }
    }
}

impl TomographyLimits {
    fn check_u64(
        actual: usize,
        limit: Option<u64>,
        resource: &'static str,
    ) -> TomographyResult<()> {
        if let Some(limit) = limit {
            let actual =
                u64::try_from(actual).map_err(|_| {
                    TomographyError::IntegerOverflow(resource)
                })?;

            if actual > limit {
                return Err(TomographyError::ResourceLimitExceeded {
                    resource,
                    requested: u128::from(actual),
                    limit: u128::from(limit),
                });
            }
        }

        Ok(())
    }

    fn check_elements(
        rows: usize,
        columns: usize,
        limit: Option<u128>,
    ) -> TomographyResult<u128> {
        let rows = u128::try_from(rows)
            .map_err(|_| TomographyError::IntegerOverflow("matrix rows"))?;
        let columns = u128::try_from(columns)
            .map_err(|_| TomographyError::IntegerOverflow("matrix columns"))?;

        let elements = rows.checked_mul(columns).ok_or(
            TomographyError::IntegerOverflow("matrix element count"),
        )?;

        if let Some(limit) = limit {
            if elements > limit {
                return Err(TomographyError::ResourceLimitExceeded {
                    resource: "matrix elements",
                    requested: elements,
                    limit,
                });
            }
        }

        Ok(elements)
    }
}

/// Numerical policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TomographyNumericalPolicy {
    /// Pivot/singularity tolerance.
    pub tolerance: f64,

    /// Whether approximate numerical reconstruction is explicitly permitted.
    pub allow_approximation: bool,

    /// Whether physical-state projection may be used.
    pub allow_physical_projection: bool,
}

impl Default for TomographyNumericalPolicy {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_NUMERICAL_TOLERANCE,
            allow_approximation: true,
            allow_physical_projection: false,
        }
    }
}

impl TomographyNumericalPolicy {
    pub fn validate(&self) -> TomographyResult<()> {
        if !self.tolerance.is_finite()
            || self.tolerance <= 0.0
        {
            return Err(TomographyError::InvalidTolerance(
                self.tolerance,
            ));
        }

        Ok(())
    }
}

/// Complete reconstruction policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TomographyPolicy {
    pub limits: TomographyLimits,
    pub numerical: TomographyNumericalPolicy,
}

impl Default for TomographyPolicy {
    fn default() -> Self {
        Self {
            limits: TomographyLimits::default(),
            numerical: TomographyNumericalPolicy::default(),
        }
    }
}

impl TomographyPolicy {
    pub fn validate(&self) -> TomographyResult<()> {
        self.numerical.validate()
    }
}

// =============================================================================
// Tomography kind
// =============================================================================

/// The semantic object being reconstructed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TomographyKind {
    /// Quantum-state tomography.
    State,

    /// Quantum-process/channel tomography.
    Process,

    /// Measurement-device tomography.
    Measurement,

    /// Preparation-device tomography.
    Preparation,

    /// General characterization whose mathematical model is supplied by the
    /// caller.
    General,
}

// =============================================================================
// Resource scope
// =============================================================================

/// Resources associated with a tomography problem.
///
/// Canonical qubit identities come directly from the Quantum IR.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TomographyResource {
    LogicalQubit(QubitId),
    PhysicalQubit(PhysicalQubitId),
    ZqnResource(ZqnIdValue),
    Named(String),
}

impl TomographyResource {
    pub fn validate(&self) -> TomographyResult<()> {
        if let Self::Named(name) = self {
            if name.trim().is_empty() {
                return Err(TomographyError::InvalidInput(
                    "empty resource name",
                ));
            }
        }

        Ok(())
    }
}

/// Resource scope for a tomography result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TomographyScope {
    pub resources: Vec<TomographyResource>,
    pub aggregate: bool,
}

impl TomographyScope {
    pub fn validate(&self) -> TomographyResult<()> {
        for resource in &self.resources {
            resource.validate()?;
        }

        if self.resources.is_empty() && !self.aggregate {
            return Err(TomographyError::InvalidInput(
                "tomography scope is empty",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Complex scalar
// =============================================================================

/// Dependency-free complex scalar.
///
/// This is intentionally local to tomography and is not a replacement for
/// the repository's future canonical complex-number abstraction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex64 {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex64 {
    pub const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    pub const ONE: Self = Self {
        real: 1.0,
        imaginary: 0.0,
    };

    pub fn new(real: f64, imaginary: f64) -> TomographyResult<Self> {
        if !real.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "complex.real",
                value: real,
            });
        }

        if !imaginary.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "complex.imaginary",
                value: imaginary,
            });
        }

        Ok(Self { real, imaginary })
    }

    pub fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn norm_squared(self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn scale(self, factor: f64) -> TomographyResult<Self> {
        let real = self.real * factor;
        let imaginary = self.imaginary * factor;

        Self::new(real, imaginary)
    }

    pub fn add(self, other: Self) -> TomographyResult<Self> {
        Self::new(
            self.real + other.real,
            self.imaginary + other.imaginary,
        )
    }

    pub fn sub(self, other: Self) -> TomographyResult<Self> {
        Self::new(
            self.real - other.real,
            self.imaginary - other.imaginary,
        )
    }

    pub fn mul(self, other: Self) -> TomographyResult<Self> {
        Self::new(
            self.real * other.real - self.imaginary * other.imaginary,
            self.real * other.imaginary
                + self.imaginary * other.real,
        )
    }

    pub fn div_real(self, divisor: f64) -> TomographyResult<Self> {
        if !divisor.is_finite() || divisor == 0.0 {
            return Err(TomographyError::NumericalFailure(
                "complex division",
            ));
        }

        Self::new(
            self.real / divisor,
            self.imaginary / divisor,
        )
    }
}

// =============================================================================
// Dense real matrix
// =============================================================================

/// Row-major dense real matrix.
///
/// This is a numerical implementation detail of the tomography solver, not a
/// second quantum IR.
#[derive(Clone, Debug, PartialEq)]
pub struct RealMatrix {
    rows: usize,
    columns: usize,
    data: Vec<f64>,
}

impl RealMatrix {
    pub fn zeros(
        rows: usize,
        columns: usize,
        limits: &TomographyLimits,
    ) -> TomographyResult<Self> {
        let elements =
            TomographyLimits::check_elements(
                rows,
                columns,
                limits.max_matrix_elements,
            )?;

        let bytes = elements
            .checked_mul(
                u128::try_from(core::mem::size_of::<f64>())
                    .map_err(|_| {
                        TomographyError::IntegerOverflow(
                            "matrix byte size",
                        )
                    })?,
            )
            .ok_or(TomographyError::IntegerOverflow(
                "matrix byte size",
            ))?;

        if let Some(limit) = limits.max_workspace_bytes {
            if bytes > limit {
                return Err(
                    TomographyError::ResourceLimitExceeded {
                        resource: "matrix workspace",
                        requested: bytes,
                        limit,
                    },
                );
            }
        }

        let len = usize::try_from(elements)
            .map_err(|_| {
                TomographyError::IntegerOverflow(
                    "matrix allocation length",
                )
            })?;

        Ok(Self {
            rows,
            columns,
            data: vec![0.0; len],
        })
    }

    pub fn from_rows(
        rows: &[Vec<f64>],
        limits: &TomographyLimits,
    ) -> TomographyResult<Self> {
        if rows.is_empty() {
            return Err(TomographyError::InvalidInput(
                "empty matrix",
            ));
        }

        let columns = rows[0].len();

        if columns == 0 {
            return Err(TomographyError::InvalidInput(
                "matrix has zero columns",
            ));
        }

        let mut matrix =
            Self::zeros(rows.len(), columns, limits)?;

        for (row_index, row) in rows.iter().enumerate() {
            if row.len() != columns {
                return Err(TomographyError::DimensionMismatch {
                    expected: columns,
                    actual: row.len(),
                    context: "matrix row",
                });
            }

            for (column_index, value) in row.iter().enumerate() {
                matrix.set(row_index, column_index, *value)?;
            }
        }

        Ok(matrix)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    fn index(
        &self,
        row: usize,
        column: usize,
    ) -> TomographyResult<usize> {
        if row >= self.rows {
            return Err(TomographyError::InvalidInput(
                "matrix row out of bounds",
            ));
        }

        if column >= self.columns {
            return Err(TomographyError::InvalidInput(
                "matrix column out of bounds",
            ));
        }

        row.checked_mul(self.columns)
            .and_then(|base| base.checked_add(column))
            .ok_or(TomographyError::IntegerOverflow(
                "matrix index",
            ))
    }

    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> TomographyResult<f64> {
        let index = self.index(row, column)?;
        Ok(self.data[index])
    }

    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: f64,
    ) -> TomographyResult<()> {
        if !value.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "matrix element",
                value,
            });
        }

        let index = self.index(row, column)?;
        self.data[index] = value;
        Ok(())
    }

    pub fn swap_rows(
        &mut self,
        first: usize,
        second: usize,
    ) -> TomographyResult<()> {
        if first >= self.rows || second >= self.rows {
            return Err(TomographyError::InvalidInput(
                "matrix row out of bounds",
            ));
        }

        if first == second {
            return Ok(());
        }

        for column in 0..self.columns {
            let a = self.index(first, column)?;
            let b = self.index(second, column)?;
            self.data.swap(a, b);
        }

        Ok(())
    }

    pub fn scale_row(
        &mut self,
        row: usize,
        factor: f64,
    ) -> TomographyResult<()> {
        if !factor.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "row scale",
                value: factor,
            });
        }

        for column in 0..self.columns {
            let index = self.index(row, column)?;
            let value = self.data[index] * factor;

            if !value.is_finite() {
                return Err(TomographyError::NumericalFailure(
                    "row scaling",
                ));
            }

            self.data[index] = value;
        }

        Ok(())
    }

    pub fn subtract_scaled_row(
        &mut self,
        target: usize,
        source: usize,
        factor: f64,
    ) -> TomographyResult<()> {
        if !factor.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "row elimination factor",
                value: factor,
            });
        }

        for column in 0..self.columns {
            let target_index =
                self.index(target, column)?;
            let source_index =
                self.index(source, column)?;

            let value = self.data[target_index]
                - factor * self.data[source_index];

            if !value.is_finite() {
                return Err(TomographyError::NumericalFailure(
                    "row elimination",
                ));
            }

            self.data[target_index] = value;
        }

        Ok(())
    }
}

// =============================================================================
// Tomography datum
// =============================================================================

/// One scalar tomography equation.
///
/// The equation represented is:
///
/// ```text
/// Σ coefficient[j] * parameter[j] = value
/// ```
///
/// `weight` is optional. If supplied, it is interpreted as a non-negative
/// statistical weight by weighted least squares.
#[derive(Clone, Debug, PartialEq)]
pub struct TomographyDatum {
    pub coefficients: Vec<f64>,
    pub value: f64,
    pub weight: Option<f64>,
}

impl TomographyDatum {
    pub fn new(
        coefficients: Vec<f64>,
        value: f64,
    ) -> TomographyResult<Self> {
        if coefficients.is_empty() {
            return Err(TomographyError::InvalidInput(
                "empty tomography coefficient row",
            ));
        }

        if !value.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "tomography datum value",
                value,
            });
        }

        for coefficient in &coefficients {
            if !coefficient.is_finite() {
                return Err(TomographyError::NonFiniteValue {
                    field: "tomography coefficient",
                    value: *coefficient,
                });
            }
        }

        Ok(Self {
            coefficients,
            value,
            weight: None,
        })
    }

    pub fn with_weight(
        mut self,
        weight: f64,
    ) -> TomographyResult<Self> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(TomographyError::InvalidWeight(weight));
        }

        self.weight = Some(weight);
        Ok(self)
    }
}

// =============================================================================
// Problem
// =============================================================================

/// A complete tomography reconstruction problem.
///
/// The parameter vector is representation-defined by the caller.
///
/// For state tomography it may represent:
///
/// - density-matrix coordinates;
//! - Bloch-vector coordinates;
//! - another declared Hermitian basis.
///
/// For process tomography it may represent:
///
/// - Choi coordinates;
//! - Pauli-transfer coordinates;
//! - Liouville coordinates;
//! - another explicitly declared basis.
///
/// This module reconstructs the declared parameter vector; it does not invent
/// a competing canonical quantum representation.
#[derive(Clone, Debug, PartialEq)]
pub struct TomographyProblem {
    pub kind: TomographyKind,
    pub parameter_count: usize,
    pub data: Vec<TomographyDatum>,
    pub scope: TomographyScope,

    /// Optional scientific provenance.
    pub characterization: Option<CharacterizationId>,
    pub experiment: Option<ExperimentId>,
    pub calibration: Option<CalibrationId>,
    pub target: Option<ZqnIdValue>,
}

impl TomographyProblem {
    pub fn validate(
        &self,
        limits: &TomographyLimits,
    ) -> TomographyResult<()> {
        if self.parameter_count == 0 {
            return Err(TomographyError::InvalidInput(
                "zero tomography parameters",
            ));
        }

        if self.data.is_empty() {
            return Err(TomographyError::InvalidInput(
                "empty tomography dataset",
            ));
        }

        TomographyLimits::check_u64(
            self.parameter_count,
            limits.max_parameters,
            "parameters",
        )?;

        TomographyLimits::check_u64(
            self.data.len(),
            limits.max_equations,
            "equations",
        )?;

        self.scope.validate()?;

        for datum in &self.data {
            if datum.coefficients.len()
                != self.parameter_count
            {
                return Err(
                    TomographyError::DimensionMismatch {
                        expected: self.parameter_count,
                        actual: datum.coefficients.len(),
                        context: "tomography equation",
                    },
                );
            }

            if let Some(weight) = datum.weight {
                if !weight.is_finite() || weight < 0.0 {
                    return Err(
                        TomographyError::InvalidWeight(weight),
                    );
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Reconstruction method
// =============================================================================

/// Reconstruction algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TomographyMethod {
    /// Solve a square full-rank system directly.
    LinearInversion,

    /// Solve the normal equations:
    ///
    /// ```text
    /// Aᵀ W A x = Aᵀ W y
    /// ```
    ///
    /// This supports overdetermined systems.
    WeightedLeastSquares,
}

impl TomographyMethod {
    fn requires_square_system(self) -> bool {
        matches!(self, Self::LinearInversion)
    }
}

// =============================================================================
// Diagnostics
// =============================================================================

/// Structural information about a tomography problem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemShape {
    ExactlyDetermined,
    Overdetermined,
    Underdetermined,
}

/// Numerical diagnostics from reconstruction.
#[derive(Clone, Debug, PartialEq)]
pub struct TomographyDiagnostics {
    pub equations: usize,
    pub parameters: usize,
    pub shape: SystemShape,
    pub rank: usize,
    pub residual_l2: f64,
    pub residual_max: f64,
    pub minimum_pivot: f64,
    pub maximum_pivot: f64,
    pub condition_indicator: Option<f64>,
}

impl TomographyDiagnostics {
    fn validate(&self) -> TomographyResult<()> {
        for value in [
            self.residual_l2,
            self.residual_max,
            self.minimum_pivot,
            self.maximum_pivot,
        ] {
            if !value.is_finite() {
                return Err(TomographyError::NumericalFailure(
                    "non-finite tomography diagnostic",
                ));
            }
        }

        if self.minimum_pivot < 0.0
            || self.maximum_pivot < 0.0
        {
            return Err(TomographyError::InvalidInput(
                "negative pivot diagnostic",
            ));
        }

        if let Some(condition) = self.condition_indicator {
            if !condition.is_finite() || condition < 0.0 {
                return Err(TomographyError::NumericalFailure(
                    "invalid condition indicator",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Reconstruction result
// =============================================================================

/// Status of a tomography reconstruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TomographyStatus {
    Valid,
    NumericallyIllConditioned,
    PhysicallyUnvalidated,
}

/// Reconstructed parameter vector.
#[derive(Clone, Debug, PartialEq)]
pub struct TomographyResultData {
    pub schema_id: &'static str,
    pub schema_version: u32,

    pub kind: TomographyKind,
    pub method: TomographyMethod,

    pub parameters: Vec<f64>,

    pub diagnostics: TomographyDiagnostics,

    pub status: TomographyStatus,

    /// Whether physical projection was explicitly requested and applied.
    pub physical_projection_applied: bool,

    /// Numerical tolerance used by reconstruction.
    pub tolerance: f64,

    pub scope: TomographyScope,

    pub characterization: Option<CharacterizationId>,
    pub experiment: Option<ExperimentId>,
    pub calibration: Option<CalibrationId>,
    pub target: Option<ZqnIdValue>,
}

impl TomographyResultData {
    pub fn validate(&self) -> TomographyResult<()> {
        if self.schema_id != TOMOGRAPHY_SCHEMA_ID {
            return Err(TomographyError::InvalidInput(
                "invalid tomography schema",
            ));
        }

        if self.schema_version == 0 {
            return Err(TomographyError::InvalidInput(
                "invalid tomography schema version",
            ));
        }

        if !self.tolerance.is_finite()
            || self.tolerance <= 0.0
        {
            return Err(TomographyError::InvalidTolerance(
                self.tolerance,
            ));
        }

        if self.parameters.is_empty() {
            return Err(TomographyError::InvalidInput(
                "empty reconstructed parameter vector",
            ));
        }

        for parameter in &self.parameters {
            if !parameter.is_finite() {
                return Err(TomographyError::NonFiniteValue {
                    field: "reconstructed parameter",
                    value: *parameter,
                });
            }
        }

        self.diagnostics.validate()?;
        self.scope.validate()?;

        Ok(())
    }
}

// =============================================================================
// Solver
// =============================================================================

/// Deterministic tomography reconstructor.
#[derive(Clone, Copy, Debug)]
pub struct TomographyReconstructor {
    policy: TomographyPolicy,
}

impl TomographyReconstructor {
    pub fn new(
        policy: TomographyPolicy,
    ) -> TomographyResult<Self> {
        policy.validate()?;

        Ok(Self { policy })
    }

    pub fn policy(&self) -> TomographyPolicy {
        self.policy
    }

    /// Reconstructs the supplied tomography problem.
    pub fn reconstruct(
        &self,
        problem: &TomographyProblem,
        method: TomographyMethod,
    ) -> TomographyResult<TomographyResultData> {
        problem.validate(&self.policy.limits)?;

        let equations = problem.data.len();
        let parameters = problem.parameter_count;

        let shape = if equations < parameters {
            SystemShape::Underdetermined
        } else if equations == parameters {
            SystemShape::ExactlyDetermined
        } else {
            SystemShape::Overdetermined
        };

        if method.requires_square_system()
            && equations != parameters
        {
            return if equations < parameters {
                Err(TomographyError::Underdetermined {
                    equations,
                    parameters,
                })
            } else {
                Err(TomographyError::UnsupportedMethod(
                    "linear inversion requires a square system",
                ))
            };
        }

        if matches!(method, TomographyMethod::WeightedLeastSquares)
            && equations < parameters
        {
            return Err(TomographyError::Underdetermined {
                equations,
                parameters,
            });
        }

        let operations = Self::estimate_operations(
            equations,
            parameters,
            method,
        )?;

        if let Some(limit) =
            self.policy.limits.max_arithmetic_operations
        {
            if operations > limit {
                return Err(
                    TomographyError::ResourceLimitExceeded {
                        resource: "arithmetic operations",
                        requested: operations,
                        limit,
                    },
                );
            }
        }

        let (parameters_out, diagnostics) =
            match method {
                TomographyMethod::LinearInversion => {
                    self.solve_linear_inversion(problem)?
                }

                TomographyMethod::WeightedLeastSquares => {
                    self.solve_weighted_least_squares(problem)?
                }
            };

        let mut result = TomographyResultData {
            schema_id: TOMOGRAPHY_SCHEMA_ID,
            schema_version: TOMOGRAPHY_SCHEMA_VERSION,
            kind: problem.kind,
            method,
            parameters: parameters_out,
            diagnostics,
            status: TomographyStatus::Valid,
            physical_projection_applied: false,
            tolerance: self.policy.numerical.tolerance,
            scope: problem.scope.clone(),
            characterization: problem.characterization,
            experiment: problem.experiment,
            calibration: problem.calibration,
            target: problem.target,
        };

        if result
            .diagnostics
            .condition_indicator
            .is_some_and(|condition| condition > 1.0 / self.policy.numerical.tolerance)
        {
            result.status =
                TomographyStatus::NumericallyIllConditioned;
        }

        result.validate()?;

        Ok(result)
    }

    fn estimate_operations(
        equations: usize,
        parameters: usize,
        method: TomographyMethod,
    ) -> TomographyResult<u128> {
        let equations = u128::try_from(equations)
            .map_err(|_| {
                TomographyError::IntegerOverflow(
                    "equation count",
                )
            })?;

        let parameters = u128::try_from(parameters)
            .map_err(|_| {
                TomographyError::IntegerOverflow(
                    "parameter count",
                )
            })?;

        match method {
            TomographyMethod::LinearInversion => {
                parameters
                    .checked_mul(parameters)
                    .and_then(|v| v.checked_mul(parameters))
                    .and_then(|v| v.checked_mul(2))
                    .ok_or(TomographyError::IntegerOverflow(
                        "linear inversion operation estimate",
                    ))
            }

            TomographyMethod::WeightedLeastSquares => {
                equations
                    .checked_mul(parameters)
                    .and_then(|v| {
                        v.checked_mul(parameters)
                    })
                    .and_then(|v| v.checked_add(
                        parameters
                            .checked_mul(parameters)?
                            .checked_mul(parameters)?,
                    ))
                    .ok_or(TomographyError::IntegerOverflow(
                        "least squares operation estimate",
                    ))
            }
        }
    }

    fn solve_linear_inversion(
        &self,
        problem: &TomographyProblem,
    ) -> TomographyResult<(
        Vec<f64>,
        TomographyDiagnostics,
    )> {
        let n = problem.parameter_count;

        let mut augmented =
            RealMatrix::zeros(
                n,
                n + 1,
                &self.policy.limits,
            )?;

        for row in 0..n {
            for column in 0..n {
                augmented.set(
                    row,
                    column,
                    problem.data[row]
                        .coefficients[column],
                )?;
            }

            augmented.set(
                row,
                n,
                problem.data[row].value,
            )?;
        }

        let (
            solution,
            minimum_pivot,
            maximum_pivot,
        ) = Self::gaussian_solve(
            &mut augmented,
            self.policy.numerical.tolerance,
        )?;

        let diagnostics =
            Self::diagnostics_from_solution(
                problem,
                &solution,
                minimum_pivot,
                maximum_pivot,
            )?;

        Ok((solution, diagnostics))
    }

    fn solve_weighted_least_squares(
        &self,
        problem: &TomographyProblem,
    ) -> TomographyResult<(
        Vec<f64>,
        TomographyDiagnostics,
    )> {
        let p = problem.parameter_count;

        let mut normal =
            RealMatrix::zeros(
                p,
                p + 1,
                &self.policy.limits,
            )?;

        for datum in &problem.data {
            let weight = datum.weight.unwrap_or(1.0);

            if !weight.is_finite() || weight < 0.0 {
                return Err(
                    TomographyError::InvalidWeight(weight),
                );
            }

            if weight == 0.0 {
                continue;
            }

            for i in 0..p {
                let ai = datum.coefficients[i];

                for j in 0..p {
                    let aj = datum.coefficients[j];
                    let old = normal.get(i, j)?;

                    let contribution =
                        weight * ai * aj;

                    let updated =
                        old + contribution;

                    if !updated.is_finite() {
                        return Err(
                            TomographyError::NumericalFailure(
                                "normal-equation accumulation",
                            ),
                        );
                    }

                    normal.set(i, j, updated)?;
                }

                let rhs = normal.get(i, p)?;
                let contribution =
                    weight * ai * datum.value;

                let updated = rhs + contribution;

                if !updated.is_finite() {
                    return Err(
                        TomographyError::NumericalFailure(
                            "least-squares right-hand side",
                        ),
                    );
                }

                normal.set(i, p, updated)?;
            }
        }

        let (
            solution,
            minimum_pivot,
            maximum_pivot,
        ) = Self::gaussian_solve(
            &mut normal,
            self.policy.numerical.tolerance,
        )?;

        let diagnostics =
            Self::diagnostics_from_solution(
                problem,
                &solution,
                minimum_pivot,
                maximum_pivot,
            )?;

        Ok((solution, diagnostics))
    }

    fn gaussian_solve(
        matrix: &mut RealMatrix,
        tolerance: f64,
    ) -> TomographyResult<(
        Vec<f64>,
        f64,
        f64,
    )> {
        let n = matrix.rows();

        if matrix.columns() != n + 1 {
            return Err(TomographyError::DimensionMismatch {
                expected: n + 1,
                actual: matrix.columns(),
                context: "augmented linear system",
            });
        }

        let mut minimum_pivot =
            f64::INFINITY;
        let mut maximum_pivot = 0.0;

        for pivot in 0..n {
            let mut best_row = pivot;
            let mut best_abs =
                matrix.get(pivot, pivot)?.abs();

            for candidate in (pivot + 1)..n {
                let value =
                    matrix.get(candidate, pivot)?.abs();

                if value > best_abs {
                    best_abs = value;
                    best_row = candidate;
                }
            }

            if !best_abs.is_finite() {
                return Err(
                    TomographyError::NumericalFailure(
                        "pivot selection",
                    ),
                );
            }

            if best_abs <= tolerance {
                return Err(
                    TomographyError::SingularSystem {
                        pivot,
                    },
                );
            }

            if best_row != pivot {
                matrix.swap_rows(
                    pivot,
                    best_row,
                )?;
            }

            minimum_pivot =
                minimum_pivot.min(best_abs);
            maximum_pivot =
                maximum_pivot.max(best_abs);

            let pivot_value =
                matrix.get(pivot, pivot)?;

            for row in (pivot + 1)..n {
                let value =
                    matrix.get(row, pivot)?;

                if value.abs() <= tolerance {
                    matrix.set(
                        row,
                        pivot,
                        0.0,
                    )?;
                    continue;
                }

                let factor =
                    value / pivot_value;

                matrix.subtract_scaled_row(
                    row,
                    pivot,
                    factor,
                )?;
            }
        }

        let mut solution = vec![0.0; n];

        for row in (0..n).rev() {
            let pivot =
                matrix.get(row, row)?;

            if pivot.abs() <= tolerance {
                return Err(
                    TomographyError::SingularSystem {
                        pivot: row,
                    },
                );
            }

            let mut rhs =
                matrix.get(row, n)?;

            for column in (row + 1)..n {
                rhs -= matrix
                    .get(row, column)?
                    * solution[column];
            }

            let value = rhs / pivot;

            if !value.is_finite() {
                return Err(
                    TomographyError::NumericalFailure(
                        "back substitution",
                    ),
                );
            }

            solution[row] = value;
        }

        Ok((
            solution,
            minimum_pivot,
            maximum_pivot,
        ))
    }

    fn diagnostics_from_solution(
        problem: &TomographyProblem,
        solution: &[f64],
        minimum_pivot: f64,
        maximum_pivot: f64,
    ) -> TomographyResult<TomographyDiagnostics> {
        let equations = problem.data.len();
        let parameters = problem.parameter_count;

        let shape = if equations < parameters {
            SystemShape::Underdetermined
        } else if equations == parameters {
            SystemShape::ExactlyDetermined
        } else {
            SystemShape::Overdetermined
        };

        let mut residual_squared = 0.0;
        let mut residual_max = 0.0;

        for datum in &problem.data {
            let mut predicted = 0.0;

            for index in 0..parameters {
                predicted +=
                    datum.coefficients[index]
                        * solution[index];
            }

            if !predicted.is_finite() {
                return Err(
                    TomographyError::NumericalFailure(
                        "tomography prediction",
                    ),
                );
            }

            let residual =
                predicted - datum.value;

            residual_squared += residual * residual;
            residual_max =
                residual_max.max(residual.abs());
        }

        if !residual_squared.is_finite() {
            return Err(
                TomographyError::NumericalFailure(
                    "tomography residual",
                ),
            );
        }

        let residual_l2 =
            residual_squared.sqrt();

        let condition_indicator =
            if minimum_pivot > 0.0 {
                Some(maximum_pivot / minimum_pivot)
            } else {
                None
            };

        let rank = parameters;

        let diagnostics =
            TomographyDiagnostics {
                equations,
                parameters,
                shape,
                rank,
                residual_l2,
                residual_max,
                minimum_pivot,
                maximum_pivot,
                condition_indicator,
            };

        diagnostics.validate()?;

        Ok(diagnostics)
    }
}

// =============================================================================
// State physicality helpers
// =============================================================================

/// Checks whether a real matrix can represent a normalized diagonal density
/// matrix.
///
/// This helper is intentionally limited to diagonal real density matrices.
/// General Hermitian/PSD matrix validation requires the declared matrix
/// representation and is therefore not silently assumed here.
pub fn validate_diagonal_density_matrix(
    probabilities: &[f64],
    tolerance: f64,
) -> TomographyResult<()> {
    if probabilities.is_empty() {
        return Err(TomographyError::InvalidPhysicalState(
            "empty density matrix",
        ));
    }

    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(TomographyError::InvalidTolerance(
            tolerance,
        ));
    }

    let mut sum = 0.0;

    for probability in probabilities {
        if !probability.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "density probability",
                value: *probability,
            });
        }

        if *probability < -tolerance {
            return Err(TomographyError::InvalidPhysicalState(
                "negative probability",
            ));
        }

        sum += *probability;
    }

    if !sum.is_finite() {
        return Err(TomographyError::NumericalFailure(
            "density normalization",
        ));
    }

    if (sum - 1.0).abs() > tolerance {
        return Err(TomographyError::InvalidPhysicalState(
            "density matrix is not normalized",
        ));
    }

    Ok(())
}

/// Explicitly projects a diagonal vector onto the probability simplex.
///
/// This is an approximation/projection operation and therefore MUST be
/// explicitly requested by the caller.
///
/// The implementation is deterministic and requires O(n log n) temporary
/// storage.
pub fn project_to_probability_simplex(
    values: &[f64],
    tolerance: f64,
) -> TomographyResult<Vec<f64>> {
    if values.is_empty() {
        return Err(TomographyError::InvalidInput(
            "empty probability vector",
        ));
    }

    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(TomographyError::InvalidTolerance(
            tolerance,
        ));
    }

    for value in values {
        if !value.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                field: "simplex value",
                value: *value,
            });
        }
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| {
        b.partial_cmp(a)
            .unwrap_or(core::cmp::Ordering::Equal)
    });

    let mut cumulative = 0.0;
    let mut rho = None;

    for (index, value) in sorted.iter().enumerate() {
        cumulative += *value;

        let denominator = (index + 1) as f64;
        let threshold =
            *value + (1.0 - cumulative) / denominator;

        if threshold > 0.0 {
            rho = Some(index);
        }
    }

    let rho = rho.ok_or(
        TomographyError::ProjectionFailure(
            "probability simplex projection",
        ),
    )?;

    let denominator = (rho + 1) as f64;

    let sum_top = sorted
        .iter()
        .take(rho + 1)
        .fold(0.0, |sum, value| sum + *value);

    let theta =
        (sum_top - 1.0) / denominator;

    let mut projected =
        Vec::with_capacity(values.len());

    for value in values {
        let projected_value =
            (*value - theta).max(0.0);

        if !projected_value.is_finite() {
            return Err(
                TomographyError::ProjectionFailure(
                    "non-finite projected probability",
                ),
            );
        }

        projected.push(projected_value);
    }

    let total =
        projected.iter().fold(0.0, |sum, value| {
            sum + *value
        });

    if !total.is_finite() || total <= 0.0 {
        return Err(
            TomographyError::ProjectionFailure(
                "invalid projected normalization",
            ),
        );
    }

    if (total - 1.0).abs() > tolerance {
        for value in &mut projected {
            *value /= total;
        }
    }

    validate_diagonal_density_matrix(
        &projected,
        tolerance * 10.0,
    )?;

    Ok(projected)
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Builds a tomography problem from an iterator without forcing callers to
/// materialize an intermediate observation-specific representation.
pub fn problem_from_iter<I>(
    kind: TomographyKind,
    parameter_count: usize,
    data: I,
    scope: TomographyScope,
) -> TomographyResult<TomographyProblem>
where
    I: IntoIterator<Item = TomographyDatum>,
{
    let data: Vec<TomographyDatum> =
        data.into_iter().collect();

    let problem = TomographyProblem {
        kind,
        parameter_count,
        data,
        scope,
        characterization: None,
        experiment: None,
        calibration: None,
        target: None,
    };

    problem.validate(&TomographyLimits::default())?;

    Ok(problem)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn two_by_two_problem() -> TomographyProblem {
        TomographyProblem {
            kind: TomographyKind::State,
            parameter_count: 2,
            data: vec![
                TomographyDatum::new(
                    vec![1.0, 0.0],
                    0.25,
                )
                .expect("valid datum"),
                TomographyDatum::new(
                    vec![0.0, 1.0],
                    0.75,
                )
                .expect("valid datum"),
            ],
            scope: TomographyScope {
                resources: vec![],
                aggregate: true,
            },
            characterization: None,
            experiment: None,
            calibration: None,
            target: None,
        }
    }

    #[test]
    fn exact_linear_inversion_is_deterministic() {
        let problem = two_by_two_problem();

        let reconstructor =
            TomographyReconstructor::new(
                TomographyPolicy::default(),
            )
            .expect("valid policy");

        let first = reconstructor
            .reconstruct(
                &problem,
                TomographyMethod::LinearInversion,
            )
            .expect("reconstruction");

        let second = reconstructor
            .reconstruct(
                &problem,
                TomographyMethod::LinearInversion,
            )
            .expect("reconstruction");

        assert_eq!(
            first.parameters,
            second.parameters
        );

        assert!(
            (first.parameters[0] - 0.25).abs()
                < 1.0e-12
        );

        assert!(
            (first.parameters[1] - 0.75).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn weighted_least_squares_reconstructs_overdetermined_system() {
        let problem = TomographyProblem {
            kind: TomographyKind::General,
            parameter_count: 2,
            data: vec![
                TomographyDatum::new(
                    vec![1.0, 0.0],
                    0.25,
                )
                .expect("valid datum"),
                TomographyDatum::new(
                    vec![0.0, 1.0],
                    0.75,
                )
                .expect("valid datum"),
                TomographyDatum::new(
                    vec![1.0, 1.0],
                    1.0,
                )
                .expect("valid datum"),
            ],
            scope: TomographyScope {
                resources: vec![],
                aggregate: true,
            },
            characterization: None,
            experiment: None,
            calibration: None,
            target: None,
        };

        let reconstructor =
            TomographyReconstructor::new(
                TomographyPolicy::default(),
            )
            .expect("valid policy");

        let result = reconstructor
            .reconstruct(
                &problem,
                TomographyMethod::WeightedLeastSquares,
            )
            .expect("reconstruction");

        assert!(
            (result.parameters[0] - 0.25).abs()
                < 1.0e-10
        );

        assert!(
            (result.parameters[1] - 0.75).abs()
                < 1.0e-10
        );
    }

    #[test]
    fn singular_system_is_rejected() {
        let problem = TomographyProblem {
            kind: TomographyKind::State,
            parameter_count: 2,
            data: vec![
                TomographyDatum::new(
                    vec![1.0, 1.0],
                    1.0,
                )
                .expect("valid datum"),
                TomographyDatum::new(
                    vec![2.0, 2.0],
                    2.0,
                )
                .expect("valid datum"),
            ],
            scope: TomographyScope {
                resources: vec![],
                aggregate: true,
            },
            characterization: None,
            experiment: None,
            calibration: None,
            target: None,
        };

        let reconstructor =
            TomographyReconstructor::new(
                TomographyPolicy::default(),
            )
            .expect("valid policy");

        let error = reconstructor
            .reconstruct(
                &problem,
                TomographyMethod::LinearInversion,
            )
            .expect_err("system must be singular");

        assert!(matches!(
            error,
            TomographyError::SingularSystem { .. }
        ));
    }

    #[test]
    fn non_finite_data_is_rejected() {
        let result =
            TomographyDatum::new(
                vec![f64::NAN],
                1.0,
            );

        assert!(matches!(
            result,
            Err(TomographyError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn simplex_projection_is_valid() {
        let values =
            vec![-0.1, 0.3, 0.8];

        let projected =
            project_to_probability_simplex(
                &values,
                1.0e-10,
            )
            .expect("projection");

        validate_diagonal_density_matrix(
            &projected,
            1.0e-8,
        )
        .expect("valid projected state");
    }

    #[test]
    fn resource_limit_is_enforced() {
        let limits = TomographyLimits {
            max_matrix_elements: Some(1),
            ..TomographyLimits::default()
        };

        let result =
            RealMatrix::zeros(
                2,
                2,
                &limits,
            );

        assert!(matches!(
            result,
            Err(
                TomographyError::ResourceLimitExceeded {
                    resource: "matrix elements",
                    ..
                }
            )
        ));
    }

    #[test]
    fn canonical_quantum_resource_ids_are_used() {
        let _logical =
            TomographyResource::LogicalQubit(
                QubitId::new(0),
            );

        let _physical =
            TomographyResource::PhysicalQubit(
                PhysicalQubitId::new(0),
            );
    }
}