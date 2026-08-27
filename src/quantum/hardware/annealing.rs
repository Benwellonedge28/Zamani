//! Zamani Quantum Hardware — Quantum Annealing
//!
//! Production-grade, provider-neutral quantum annealing model.
//!
//! # Responsibility
//!
//! This module defines the canonical hardware-independent representation of
//! quantum annealing workloads and their results.
//!
//! It owns:
//!
//! - QUBO problems;
//! - Ising models;
//! - binary and spin variables;
//! - QUBO/Ising conversion;
//! - annealing schedules;
//! - annealing controls;
//! - annealing workload requirements;
//! - annealing execution metadata;
//! - annealing samples/results;
//! - objective-energy evaluation;
//! - deterministic validation;
//! - stable identifiers;
//! - serialization contracts;
//! - provider-neutral validation diagnostics.
//!
//! It deliberately does NOT own:
//!
//! - provider APIs;
//! - HTTP/network communication;
//! - credentials;
//! - authentication;
//! - job submission;
//! - job polling;
//! - queues;
//! - provider-specific device types;
//! - provider-specific topology;
//! - routing algorithms;
//! - scheduling algorithms;
//! - benchmarking;
//! - statistical analysis;
//! - quantum IR;
//! - source-language parsing;
//! - simulation;
//! - optimization algorithms that transform a user's mathematical problem.
//!
//! Those responsibilities belong to other Zamani subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum frontend / algorithms
//!      |
//!      v
//! Zamani Quantum IR / workload construction
//!      |
//!      v
//! Annealing problem
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! QUBO                  Ising model
//!      |                    |
//!      +---------+----------+
//!                |
//!                v
//!       annealing.rs
//!                |
//!       +--------+--------+
//!       |        |        |
//!       v        v        v
//!    backend  compatibility  resource estimation
//!       |
//!       v
//! provider adapter
//!       |
//!       v
//! physical annealer
//!       |
//!       v
//! AnnealingResult
//!       |
//!       v
//! benchmarking / analysis
//! ```
//!
//! # Critical architectural distinction
//!
//! Quantum annealing is NOT represented as an ordinary gate-model circuit.
//!
//! A gate-model workload is approximately:
//!
//! ```text
//! gates -> circuit -> transpilation -> execution
//! ```
//!
//! An annealing workload is:
//!
//! ```text
//! QUBO/Ising model
//!       |
//!       v
//! annealing controls
//!       |
//!       v
//! hardware-specific embedding/scheduling
//!       |
//!       v
//! annealing execution
//! ```
//!
//! Therefore this module must not introduce fake `Gate` or `Circuit`
//! representations for annealing.
//!
//! # Canonical mathematical conventions
//!
//! ## QUBO
//!
//! Zamani uses the canonical upper-triangular QUBO representation:
//!
//! ```text
//! E(x) = offset + Σ_i Q[i,i] x_i + Σ_{i<j} Q[i,j] x_i x_j
//! ```
//!
//! where:
//!
//! ```text
//! x_i ∈ {0,1}
//! ```
//!
//! The matrix is therefore represented as a deterministic collection of
//! diagonal and `(i,j)` interaction terms with `i < j`.
//!
//! ## Ising
//!
//! Zamani uses:
//!
//! ```text
//! E(s) = offset + Σ_i h_i s_i + Σ_{i<j} J[i,j] s_i s_j
//! ```
//!
//! where:
//!
//! ```text
//! s_i ∈ {-1,+1}
//! ```
//!
//! ## QUBO → Ising
//!
//! Using:
//!
//! ```text
//! x_i = (s_i + 1) / 2
//! ```
//!
//! gives:
//!
//! ```text
//! J[i,j] = Q[i,j] / 4
//!
//! h_i = Q[i,i] / 2 + Σ_j Q[min(i,j),max(i,j)] / 4
//!
//! offset_ising = offset_qubo
//!               + Σ_i Q[i,i] / 2
//!               + Σ_{i<j} Q[i,j] / 4
//! ```
//!
//! This convention is fixed and must not be silently changed by adapters.
//!
//! # Provider independence
//!
//! This file contains no IBM, D-Wave, Fujitsu, Rigetti, AWS, or other
//! provider-specific types.
//!
//! Provider adapters are responsible for translating these canonical
//! structures into provider-specific formats.
//!
//! # Integration contract
//!
//! Consumers may depend on this module:
//!
//! - `backend.rs`
//! - `backend_trait.rs`
//! - `compatibility.rs`
//! - `validation.rs`
//! - `execution.rs`
//! - `job.rs`
//! - `resource_estimator.rs`
//! - provider adapters;
//! - `quantum::benchmarking`;
//! - Danga;
//! - future annealing algorithms.
//!
//! This module must NOT depend on those consumers.
//!
//! # Stability rule
//!
//! The following concepts are part of the stable public contract:
//!
//! - `QuboProblem`;
//! - `QuboTerm`;
//! - `IsingModel`;
//! - `IsingTerm`;
//! - `AnnealingSchedule`;
//! - `AnnealingSchedulePoint`;
//! - `AnnealingControls`;
//! - `AnnealingWorkload`;
//! - `AnnealingSample`;
//! - `AnnealingResult`;
//! - `AnnealingRequirements`;
//! - `AnnealingValidationError`;
//! - `AnnealingValidationErrors`.
//!
//! New provider-specific functionality must be implemented outside this file.
//!
//! # Determinism
//!
//! Deterministic ordering is mandatory.
//!
//! BTreeMap/BTreeSet are used where ordered collections are required.
//!
//! The module never:
//!
//! - reads the system clock;
//! - accesses the network;
//! - generates randomness;
//! - depends on provider state.
//!
//! # Numerical safety
//!
//! Quantum-annealing coefficients are floating-point values. Production code
//! must never accept NaN or infinite coefficients.
//!
//! All public constructors validate:
//!
//! - finite coefficients;
//! - finite offsets;
//! - valid variable indices;
//! - valid schedule values;
//! - monotonic schedule times;
//! - non-zero sample counts;
//! - binary/spin assignments;
//! - compatible assignment dimensions.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Serialization
//!
//! Serde is used because it is already part of the Zamani dependency surface.
//!
//! The serialized representation is intended for:
//!
//! - execution provenance;
//! - benchmark records;
//! - reproducibility;
//! - job persistence;
//! - provider adapter boundaries;
//! - Danga manifests;
//! - diagnostics.
//!
//! Secrets are never stored in this module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for Zamani annealing workloads.
pub const ANNEALING_SCHEMA_ID: &str = "zamani.quantum.hardware.annealing";

/// Current schema version.
///
/// Increment only when the serialized semantic contract changes.
pub const ANNEALING_SCHEMA_VERSION: u16 = 1;

/// Maximum number of variables in one canonical problem.
///
/// This protects the in-memory representation from accidental pathological
/// allocations. Provider-specific hardware limits remain separate.
pub const MAX_VARIABLES: usize = 10_000_000;

/// Maximum number of QUBO/Ising interaction terms.
pub const MAX_INTERACTION_TERMS: usize = 50_000_000;

/// Maximum number of schedule points.
pub const MAX_SCHEDULE_POINTS: usize = 1_000_000;

/// Maximum number of samples represented by one result.
pub const MAX_RESULT_SAMPLES: usize = 10_000_000;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Numerical tolerance used when comparing energies generated through
/// mathematically equivalent transformations.
pub const DEFAULT_ENERGY_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Variable domains
// =============================================================================

/// Domain of variables used by an annealing model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableDomain {
    /// Binary QUBO variable: 0 or 1.
    Binary,

    /// Spin Ising variable: -1 or +1.
    Spin,
}

impl VariableDomain {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Spin => "spin",
        }
    }
}

impl fmt::Display for VariableDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// QUBO term
// =============================================================================

/// One canonical QUBO term.
///
/// A linear term is represented by `j == None`.
///
/// A quadratic term is represented by `Some(j)`, with the invariant:
///
/// ```text
/// i < j
/// ```
///
/// This removes duplicate representations such as `(i,j)` and `(j,i)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuboTerm {
    /// First variable index.
    pub i: usize,

    /// Optional second variable index.
    ///
    /// `None` means a diagonal/linear term.
    pub j: Option<usize>,

    /// Coefficient.
    pub coefficient: f64,
}

impl QuboTerm {
    /// Creates a linear QUBO term.
    pub fn linear(i: usize, coefficient: f64) -> Result<Self, AnnealingValidationError> {
        validate_finite(coefficient, "qubo linear coefficient")?;

        Ok(Self {
            i,
            j: None,
            coefficient,
        })
    }

    /// Creates a quadratic QUBO term.
    ///
    /// The indices are canonicalized so callers may supply either order.
    pub fn quadratic(
        i: usize,
        j: usize,
        coefficient: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_finite(coefficient, "qubo quadratic coefficient")?;

        if i == j {
            return Err(AnnealingValidationError::DiagonalInteraction {
                index: i,
            });
        }

        let (first, second) = if i < j { (i, j) } else { (j, i) };

        Ok(Self {
            i: first,
            j: Some(second),
            coefficient,
        })
    }

    /// Returns true if this is a linear term.
    pub const fn is_linear(&self) -> bool {
        self.j.is_none()
    }

    /// Returns true if this is a quadratic term.
    pub const fn is_quadratic(&self) -> bool {
        self.j.is_some()
    }

    /// Validates the term against a variable count.
    pub fn validate(&self, variable_count: usize) -> Result<(), AnnealingValidationError> {
        validate_variable_count(variable_count)?;

        if self.i >= variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: self.i,
                variable_count,
            });
        }

        validate_finite(self.coefficient, "qubo term coefficient")?;

        if let Some(j) = self.j {
            if j >= variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index: j,
                    variable_count,
                });
            }

            if self.i >= j {
                return Err(AnnealingValidationError::NonCanonicalInteraction {
                    i: self.i,
                    j,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// QUBO problem
// =============================================================================

/// Canonical Quadratic Unconstrained Binary Optimization problem.
///
/// The objective is:
///
/// ```text
/// E(x) = offset
///      + Σ_i Q[i,i] x_i
///      + Σ_{i<j} Q[i,j] x_i x_j
/// ```
///
/// where each `x_i` is either 0 or 1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuboProblem {
    /// Number of binary variables.
    pub variable_count: usize,

    /// Constant energy offset.
    pub offset: f64,

    /// Deterministically ordered linear terms.
    ///
    /// The key is the variable index.
    pub linear: BTreeMap<usize, f64>,

    /// Deterministically ordered quadratic terms.
    ///
    /// The key is `(i, j)` with `i < j`.
    pub quadratic: BTreeMap<(usize, usize), f64>,

    /// Optional human-readable problem identifier.
    pub problem_id: Option<String>,

    /// Non-secret deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QuboProblem {
    /// Creates an empty QUBO with the requested number of variables.
    pub fn new(variable_count: usize) -> Result<Self, AnnealingValidationError> {
        validate_variable_count(variable_count)?;

        Ok(Self {
            variable_count,
            offset: 0.0,
            linear: BTreeMap::new(),
            quadratic: BTreeMap::new(),
            problem_id: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Creates a QUBO from a constant offset and terms.
    pub fn from_terms(
        variable_count: usize,
        offset: f64,
        terms: impl IntoIterator<Item = QuboTerm>,
    ) -> Result<Self, AnnealingValidationErrors> {
        let mut problem = match Self::new(variable_count) {
            Ok(problem) => problem,
            Err(error) => return Err(AnnealingValidationErrors::single(error)),
        };

        if let Err(error) = validate_finite(offset, "qubo offset") {
            return Err(AnnealingValidationErrors::single(error));
        }

        problem.offset = offset;

        let mut errors = AnnealingValidationErrors::new();

        for term in terms {
            if let Err(error) = term.validate(variable_count) {
                errors.push(error);
                continue;
            }

            if let Some(j) = term.j {
                *problem.quadratic.entry((term.i, j)).or_insert(0.0) +=
                    term.coefficient;
            } else {
                *problem.linear.entry(term.i).or_insert(0.0) += term.coefficient;
            }
        }

        if errors.is_empty() {
            Ok(problem)
        } else {
            Err(errors)
        }
    }

    /// Sets the constant offset.
    pub fn with_offset(mut self, offset: f64) -> Result<Self, AnnealingValidationError> {
        validate_finite(offset, "qubo offset")?;
        self.offset = offset;
        Ok(self)
    }

    /// Sets a stable problem identifier.
    pub fn with_problem_id(
        mut self,
        problem_id: impl Into<String>,
    ) -> Result<Self, AnnealingValidationError> {
        let problem_id = problem_id.into();

        validate_text_identifier(&problem_id, "problem_id", 512)?;

        self.problem_id = Some(problem_id);
        Ok(self)
    }

    /// Adds or accumulates a linear coefficient.
    pub fn add_linear(
        &mut self,
        index: usize,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        if index >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index,
                variable_count: self.variable_count,
            });
        }

        validate_finite(coefficient, "qubo linear coefficient")?;

        *self.linear.entry(index).or_insert(0.0) += coefficient;

        validate_finite(
            *self.linear.get(&index).unwrap_or(&0.0),
            "accumulated qubo linear coefficient",
        )?;

        Ok(())
    }

    /// Adds or accumulates a quadratic coefficient.
    pub fn add_quadratic(
        &mut self,
        i: usize,
        j: usize,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        if i >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: i,
                variable_count: self.variable_count,
            });
        }

        if j >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: j,
                variable_count: self.variable_count,
            });
        }

        if i == j {
            return self.add_linear(i, coefficient);
        }

        validate_finite(coefficient, "qubo quadratic coefficient")?;

        let key = if i < j { (i, j) } else { (j, i) };

        *self.quadratic.entry(key).or_insert(0.0) += coefficient;

        validate_finite(
            *self.quadratic.get(&key).unwrap_or(&0.0),
            "accumulated qubo quadratic coefficient",
        )?;

        Ok(())
    }

    /// Adds arbitrary metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        add_metadata(&mut self.metadata, key.into(), value.into())
    }

    /// Evaluates the QUBO objective for a binary assignment.
    pub fn evaluate(
        &self,
        assignment: &[u8],
    ) -> Result<f64, AnnealingValidationError> {
        self.validate()?;

        if assignment.len() != self.variable_count {
            return Err(AnnealingValidationError::AssignmentLengthMismatch {
                expected: self.variable_count,
                actual: assignment.len(),
            });
        }

        for (index, value) in assignment.iter().copied().enumerate() {
            if value > 1 {
                return Err(AnnealingValidationError::InvalidBinaryValue {
                    index,
                    value,
                });
            }
        }

        let mut energy = self.offset;

        for (&index, &coefficient) in &self.linear {
            energy += coefficient * f64::from(assignment[index]);
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            energy += coefficient
                * f64::from(assignment[i])
                * f64::from(assignment[j]);
        }

        validate_finite(energy, "qubo evaluated energy")?;

        Ok(energy)
    }

    /// Converts this QUBO to the canonical Ising representation.
    pub fn to_ising(&self) -> Result<IsingModel, AnnealingValidationError> {
        self.validate()?;

        let mut h = BTreeMap::new();
        let mut interactions = BTreeMap::new();

        let mut offset = self.offset;

        for (&index, &coefficient) in &self.linear {
            *h.entry(index).or_insert(0.0) += coefficient / 2.0;
            offset += coefficient / 2.0;
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            let coupling = coefficient / 4.0;

            *h.entry(i).or_insert(0.0) += coupling;
            *h.entry(j).or_insert(0.0) += coupling;

            interactions.insert((i, j), coupling);

            offset += coupling;
        }

        IsingModel::from_parts(
            self.variable_count,
            offset,
            h,
            interactions,
            self.problem_id.clone(),
            self.metadata.clone(),
        )
    }

    /// Returns the number of non-zero linear terms.
    pub fn linear_term_count(&self) -> usize {
        self.linear.len()
    }

    /// Returns the number of non-zero quadratic terms.
    pub fn quadratic_term_count(&self) -> usize {
        self.quadratic.len()
    }

    /// Returns total number of non-zero terms.
    pub fn term_count(&self) -> usize {
        self.linear.len() + self.quadratic.len()
    }

    /// Validates the complete QUBO.
    pub fn validate(&self) -> Result<(), AnnealingValidationError> {
        validate_variable_count(self.variable_count)?;
        validate_finite(self.offset, "qubo offset")?;

        if self.term_count() > MAX_INTERACTION_TERMS {
            return Err(AnnealingValidationError::TooManyTerms {
                count: self.term_count(),
                maximum: MAX_INTERACTION_TERMS,
            });
        }

        for (&index, &coefficient) in &self.linear {
            if index >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index,
                    variable_count: self.variable_count,
                });
            }

            validate_finite(coefficient, "qubo linear coefficient")?;
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            if i >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index: i,
                    variable_count: self.variable_count,
                });
            }

            if j >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index: j,
                    variable_count: self.variable_count,
                });
            }

            if i >= j {
                return Err(AnnealingValidationError::NonCanonicalInteraction {
                    i,
                    j,
                });
            }

            validate_finite(coefficient, "qubo quadratic coefficient")?;
        }

        validate_metadata(&self.metadata)?;

        if let Some(problem_id) = &self.problem_id {
            validate_text_identifier(problem_id, "problem_id", 512)?;
        }

        Ok(())
    }

    /// Returns all terms in deterministic order.
    pub fn terms(&self) -> Vec<QuboTerm> {
        let mut terms = Vec::with_capacity(self.term_count());

        for (&i, &coefficient) in &self.linear {
            terms.push(QuboTerm {
                i,
                j: None,
                coefficient,
            });
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            terms.push(QuboTerm {
                i,
                j: Some(j),
                coefficient,
            });
        }

        terms
    }
}

// =============================================================================
// Ising term
// =============================================================================

/// One canonical Ising interaction term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsingTerm {
    /// First spin index.
    pub i: usize,

    /// Second spin index.
    pub j: usize,

    /// Coupling coefficient.
    pub coefficient: f64,
}

impl IsingTerm {
    /// Creates an Ising coupling and canonicalizes its indices.
    pub fn new(
        i: usize,
        j: usize,
        coefficient: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_finite(coefficient, "ising coupling coefficient")?;

        if i == j {
            return Err(AnnealingValidationError::DiagonalInteraction {
                index: i,
            });
        }

        let (first, second) = if i < j { (i, j) } else { (j, i) };

        Ok(Self {
            i: first,
            j: second,
            coefficient,
        })
    }

    /// Validates this term.
    pub fn validate(&self, variable_count: usize) -> Result<(), AnnealingValidationError> {
        if self.i >= variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: self.i,
                variable_count,
            });
        }

        if self.j >= variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: self.j,
                variable_count,
            });
        }

        if self.i >= self.j {
            return Err(AnnealingValidationError::NonCanonicalInteraction {
                i: self.i,
                j: self.j,
            });
        }

        validate_finite(self.coefficient, "ising coupling coefficient")
    }
}

// =============================================================================
// Ising model
// =============================================================================

/// Canonical Ising model.
///
/// The objective is:
///
/// ```text
/// E(s) = offset
///      + Σ_i h_i s_i
///      + Σ_{i<j} J[i,j] s_i s_j
/// ```
///
/// where:
///
/// ```text
/// s_i ∈ {-1,+1}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsingModel {
    /// Number of spin variables.
    pub variable_count: usize,

    /// Constant energy offset.
    pub offset: f64,

    /// Local fields `h_i`.
    pub linear: BTreeMap<usize, f64>,

    /// Couplings `J[i,j]`.
    pub quadratic: BTreeMap<(usize, usize), f64>,

    /// Optional human-readable problem identifier.
    pub problem_id: Option<String>,

    /// Non-secret deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl IsingModel {
    /// Creates an empty Ising model.
    pub fn new(variable_count: usize) -> Result<Self, AnnealingValidationError> {
        validate_variable_count(variable_count)?;

        Ok(Self {
            variable_count,
            offset: 0.0,
            linear: BTreeMap::new(),
            quadratic: BTreeMap::new(),
            problem_id: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Creates a complete Ising model from its canonical components.
    pub fn from_parts(
        variable_count: usize,
        offset: f64,
        linear: BTreeMap<usize, f64>,
        quadratic: BTreeMap<(usize, usize), f64>,
        problem_id: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, AnnealingValidationError> {
        let model = Self {
            variable_count,
            offset,
            linear,
            quadratic,
            problem_id,
            metadata,
        };

        model.validate()?;

        Ok(model)
    }

    /// Adds or accumulates a local field.
    pub fn add_linear(
        &mut self,
        index: usize,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        if index >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index,
                variable_count: self.variable_count,
            });
        }

        validate_finite(coefficient, "ising linear coefficient")?;

        *self.linear.entry(index).or_insert(0.0) += coefficient;

        validate_finite(
            *self.linear.get(&index).unwrap_or(&0.0),
            "accumulated ising linear coefficient",
        )?;

        Ok(())
    }

    /// Adds or accumulates an Ising coupling.
    pub fn add_quadratic(
        &mut self,
        i: usize,
        j: usize,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        if i >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: i,
                variable_count: self.variable_count,
            });
        }

        if j >= self.variable_count {
            return Err(AnnealingValidationError::VariableOutOfRange {
                index: j,
                variable_count: self.variable_count,
            });
        }

        if i == j {
            return Err(AnnealingValidationError::DiagonalInteraction {
                index: i,
            });
        }

        validate_finite(coefficient, "ising quadratic coefficient")?;

        let key = if i < j { (i, j) } else { (j, i) };

        *self.quadratic.entry(key).or_insert(0.0) += coefficient;

        validate_finite(
            *self.quadratic.get(&key).unwrap_or(&0.0),
            "accumulated ising quadratic coefficient",
        )?;

        Ok(())
    }

    /// Evaluates the Ising energy for a spin assignment.
    pub fn evaluate(
        &self,
        assignment: &[i8],
    ) -> Result<f64, AnnealingValidationError> {
        self.validate()?;

        if assignment.len() != self.variable_count {
            return Err(AnnealingValidationError::AssignmentLengthMismatch {
                expected: self.variable_count,
                actual: assignment.len(),
            });
        }

        for (index, value) in assignment.iter().copied().enumerate() {
            if value != -1 && value != 1 {
                return Err(AnnealingValidationError::InvalidSpinValue {
                    index,
                    value,
                });
            }
        }

        let mut energy = self.offset;

        for (&index, &coefficient) in &self.linear {
            energy += coefficient * f64::from(assignment[index]);
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            energy += coefficient
                * f64::from(assignment[i])
                * f64::from(assignment[j]);
        }

        validate_finite(energy, "ising evaluated energy")?;

        Ok(energy)
    }

    /// Converts the Ising model to the canonical QUBO representation.
    ///
    /// Uses:
    ///
    /// ```text
    /// s_i = 2*x_i - 1
    /// ```
    pub fn to_qubo(&self) -> Result<QuboProblem, AnnealingValidationError> {
        self.validate()?;

        let mut qubo = QuboProblem::new(self.variable_count)?;

        qubo.offset = self.offset;

        for (&index, &coefficient) in &self.linear {
            qubo.offset -= coefficient;
            qubo.add_linear(index, 2.0 * coefficient)?;
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            qubo.offset += coefficient;
            qubo.add_linear(i, -2.0 * coefficient)?;
            qubo.add_linear(j, -2.0 * coefficient)?;
            qubo.add_quadratic(i, j, 4.0 * coefficient)?;
        }

        qubo.problem_id = self.problem_id.clone();
        qubo.metadata = self.metadata.clone();

        qubo.validate()?;

        Ok(qubo)
    }

    /// Validates the complete model.
    pub fn validate(&self) -> Result<(), AnnealingValidationError> {
        validate_variable_count(self.variable_count)?;
        validate_finite(self.offset, "ising offset")?;

        let term_count = self.linear.len() + self.quadratic.len();

        if term_count > MAX_INTERACTION_TERMS {
            return Err(AnnealingValidationError::TooManyTerms {
                count: term_count,
                maximum: MAX_INTERACTION_TERMS,
            });
        }

        for (&index, &coefficient) in &self.linear {
            if index >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index,
                    variable_count: self.variable_count,
                });
            }

            validate_finite(coefficient, "ising linear coefficient")?;
        }

        for (&(i, j), &coefficient) in &self.quadratic {
            if i >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index: i,
                    variable_count: self.variable_count,
                });
            }

            if j >= self.variable_count {
                return Err(AnnealingValidationError::VariableOutOfRange {
                    index: j,
                    variable_count: self.variable_count,
                });
            }

            if i >= j {
                return Err(AnnealingValidationError::NonCanonicalInteraction {
                    i,
                    j,
                });
            }

            validate_finite(coefficient, "ising quadratic coefficient")?;
        }

        validate_metadata(&self.metadata)?;

        if let Some(problem_id) = &self.problem_id {
            validate_text_identifier(problem_id, "problem_id", 512)?;
        }

        Ok(())
    }

    /// Returns the number of local fields.
    pub fn linear_term_count(&self) -> usize {
        self.linear.len()
    }

    /// Returns the number of couplings.
    pub fn quadratic_term_count(&self) -> usize {
        self.quadratic.len()
    }

    /// Returns total number of non-zero terms.
    pub fn term_count(&self) -> usize {
        self.linear.len() + self.quadratic.len()
    }
}

// =============================================================================
// Annealing schedule
// =============================================================================

/// One point in an annealing schedule.
///
/// `time` is measured in the abstract schedule time unit exposed by the
/// canonical Zamani model. Provider adapters map it to device-specific units.
///
/// `value` normally represents an anneal fraction/control parameter, but its
/// precise physical interpretation is intentionally provider-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSchedulePoint {
    /// Schedule time.
    pub time: f64,

    /// Control/anneal value.
    pub value: f64,
}

impl AnnealingSchedulePoint {
    /// Creates a schedule point.
    pub fn new(
        time: f64,
        value: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_finite(time, "schedule time")?;
        validate_finite(value, "schedule value")?;

        if time < 0.0 {
            return Err(AnnealingValidationError::NegativeValue {
                field: "schedule time",
                value: time,
            });
        }

        Ok(Self { time, value })
    }
}

/// Canonical annealing schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSchedule {
    /// Ordered schedule points.
    pub points: Vec<AnnealingSchedulePoint>,

    /// Whether interpolation between points is linear.
    pub linear_interpolation: bool,
}

impl AnnealingSchedule {
    /// Creates a schedule from validated points.
    pub fn new(
        points: Vec<AnnealingSchedulePoint>,
    ) -> Result<Self, AnnealingValidationErrors> {
        let schedule = Self {
            points,
            linear_interpolation: true,
        };

        schedule.validate()?;

        Ok(schedule)
    }

    /// Creates the canonical two-point linear schedule.
    ///
    /// The schedule starts at `(0, 0)` and finishes at `(duration, 1)`.
    pub fn linear(duration: f64) -> Result<Self, AnnealingValidationError> {
        validate_finite(duration, "annealing duration")?;

        if duration <= 0.0 {
            return Err(AnnealingValidationError::NonPositiveValue {
                field: "annealing duration",
                value: duration,
            });
        }

        Ok(Self {
            points: vec![
                AnnealingSchedulePoint {
                    time: 0.0,
                    value: 0.0,
                },
                AnnealingSchedulePoint {
                    time: duration,
                    value: 1.0,
                },
            ],
            linear_interpolation: true,
        })
    }

    /// Sets interpolation mode.
    pub fn with_linear_interpolation(mut self, enabled: bool) -> Self {
        self.linear_interpolation = enabled;
        self
    }

    /// Returns the final schedule time.
    pub fn duration(&self) -> Option<f64> {
        self.points.last().map(|point| point.time)
    }

    /// Validates the schedule.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if self.points.len() < 2 {
            errors.push(AnnealingValidationError::InsufficientSchedulePoints {
                count: self.points.len(),
            });
        }

        if self.points.len() > MAX_SCHEDULE_POINTS {
            errors.push(AnnealingValidationError::TooManySchedulePoints {
                count: self.points.len(),
                maximum: MAX_SCHEDULE_POINTS,
            });
        }

        let mut previous_time = None;

        for (index, point) in self.points.iter().enumerate() {
            if let Err(error) = validate_finite(point.time, "schedule time") {
                errors.push(error);
            }

            if let Err(error) = validate_finite(point.value, "schedule value") {
                errors.push(error);
            }

            if point.time < 0.0 {
                errors.push(AnnealingValidationError::NegativeValue {
                    field: "schedule time",
                    value: point.time,
                });
            }

            if let Some(previous) = previous_time {
                if point.time <= previous {
                    errors.push(
                        AnnealingValidationError::NonMonotonicSchedule {
                            previous_index: index.saturating_sub(1),
                            current_index: index,
                            previous_time: previous,
                            current_time: point.time,
                        },
                    );
                }
            }

            previous_time = Some(point.time);
        }

        if let Some(first) = self.points.first() {
            if first.time != 0.0 {
                errors.push(AnnealingValidationError::InvalidScheduleStart {
                    time: first.time,
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// =============================================================================
// Annealing controls
// =============================================================================

/// Provider-neutral controls for annealing execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingControls {
    /// Number of requested samples/shots.
    pub reads: u64,

    /// Annealing schedule.
    pub schedule: AnnealingSchedule,

    /// Optional thermalization/preparation duration.
    pub thermalization_time: Option<f64>,

    /// Optional post-anneal/readout delay.
    pub readout_delay: Option<f64>,

    /// Optional spin-reversal transform count.
    ///
    /// The actual transform generation remains provider/execution specific.
    pub spin_reversal_transforms: u32,

    /// Optional chain strength for embedded problems.
    ///
    /// This is a request-level hint. Embedding remains outside this module.
    pub chain_strength: Option<f64>,

    /// Optional provider-neutral seed.
    ///
    /// Whether the target actually supports deterministic seeding is a
    /// capability question owned by the backend layer.
    pub seed: Option<u64>,
}

impl Default for AnnealingControls {
    fn default() -> Self {
        Self {
            reads: 1,
            schedule: AnnealingSchedule::linear(1.0)
                .expect("canonical linear schedule is valid"),
            thermalization_time: None,
            readout_delay: None,
            spin_reversal_transforms: 0,
            chain_strength: None,
            seed: None,
        }
    }
}

impl AnnealingControls {
    /// Validates all controls.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if self.reads == 0 {
            errors.push(AnnealingValidationError::ZeroReads);
        }

        if let Err(schedule_errors) = self.schedule.validate() {
            errors.extend(schedule_errors);
        }

        if let Some(value) = self.thermalization_time {
            if let Err(error) =
                validate_non_negative_finite(value, "thermalization_time")
            {
                errors.push(error);
            }
        }

        if let Some(value) = self.readout_delay {
            if let Err(error) =
                validate_non_negative_finite(value, "readout_delay")
            {
                errors.push(error);
            }
        }

        if let Some(value) = self.chain_strength {
            if let Err(error) = validate_finite(value, "chain_strength") {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// =============================================================================
// Annealing workload
// =============================================================================

/// Canonical annealing workload.
///
/// Exactly one mathematical representation is authoritative at a time.
///
/// `Qubo` and `Ising` can be converted deterministically without changing the
/// represented objective, subject to floating-point arithmetic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "problem")]
pub enum AnnealingProblem {
    /// Binary QUBO problem.
    #[serde(rename = "qubo")]
    Qubo(QuboProblem),

    /// Spin Ising problem.
    #[serde(rename = "ising")]
    Ising(IsingModel),
}

impl AnnealingProblem {
    /// Returns the number of variables.
    pub fn variable_count(&self) -> usize {
        match self {
            Self::Qubo(problem) => problem.variable_count,
            Self::Ising(problem) => problem.variable_count,
        }
    }

    /// Returns the variable domain.
    pub const fn domain(&self) -> VariableDomain {
        match self {
            Self::Qubo(_) => VariableDomain::Binary,
            Self::Ising(_) => VariableDomain::Spin,
        }
    }

    /// Validates the problem.
    pub fn validate(&self) -> Result<(), AnnealingValidationError> {
        match self {
            Self::Qubo(problem) => problem.validate(),
            Self::Ising(problem) => problem.validate(),
        }
    }

    /// Converts the workload into an Ising model.
    pub fn to_ising(&self) -> Result<IsingModel, AnnealingValidationError> {
        match self {
            Self::Qubo(problem) => problem.to_ising(),
            Self::Ising(problem) => {
                problem.validate()?;
                Ok(problem.clone())
            }
        }
    }

    /// Converts the workload into a QUBO.
    pub fn to_qubo(&self) -> Result<QuboProblem, AnnealingValidationError> {
        match self {
            Self::Qubo(problem) => {
                problem.validate()?;
                Ok(problem.clone())
            }
            Self::Ising(problem) => problem.to_qubo(),
        }
    }
}

/// Provider-neutral annealing workload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingWorkload {
    /// Mathematical optimization problem.
    pub problem: AnnealingProblem,

    /// Annealing controls.
    pub controls: AnnealingControls,

    /// Optional stable workload identifier.
    pub workload_id: Option<String>,

    /// Non-secret deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl AnnealingWorkload {
    /// Creates a workload.
    pub fn new(
        problem: AnnealingProblem,
        controls: AnnealingControls,
    ) -> Result<Self, AnnealingValidationErrors> {
        let workload = Self {
            problem,
            controls,
            workload_id: None,
            metadata: BTreeMap::new(),
        };

        workload.validate()?;

        Ok(workload)
    }

    /// Sets a workload identifier.
    pub fn with_workload_id(
        mut self,
        workload_id: impl Into<String>,
    ) -> Result<Self, AnnealingValidationError> {
        let workload_id = workload_id.into();

        validate_text_identifier(&workload_id, "workload_id", 512)?;

        self.workload_id = Some(workload_id);
        Ok(self)
    }

    /// Adds deterministic metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        add_metadata(&mut self.metadata, key.into(), value.into())
    }

    /// Validates the complete workload.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if let Err(error) = self.problem.validate() {
            errors.push(error);
        }

        if let Err(control_errors) = self.controls.validate() {
            errors.extend(control_errors);
        }

        if let Some(id) = &self.workload_id {
            if let Err(error) = validate_text_identifier(id, "workload_id", 512) {
                errors.push(error);
            }
        }

        if let Err(error) = validate_metadata(&self.metadata) {
            errors.push(error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns the problem domain.
    pub const fn domain(&self) -> VariableDomain {
        self.problem.domain()
    }

    /// Returns the number of variables.
    pub fn variable_count(&self) -> usize {
        self.problem.variable_count()
    }
}

// =============================================================================
// Workload requirements
// =============================================================================

/// Requirements that the hardware/backend must satisfy before submission.
///
/// This is deliberately provider-neutral and can later be mapped into the
/// authoritative hardware capability model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingRequirements {
    /// Required number of variables.
    pub minimum_variables: usize,

    /// Whether annealing execution itself is mandatory.
    pub require_annealing: bool,

    /// Whether custom schedules are required.
    pub require_custom_schedule: bool,

    /// Whether deterministic seeding is required.
    pub require_deterministic_seed: bool,

    /// Whether spin-reversal transforms are required.
    pub require_spin_reversal_transforms: bool,

    /// Whether provider-side chain embedding is required.
    pub require_embedding: bool,

    /// Whether arbitrary QUBO input is required.
    pub require_qubo: bool,

    /// Whether native Ising input is required.
    pub require_ising: bool,

    /// Whether raw sample output is required.
    pub require_samples: bool,

    /// Whether exact energy evaluation by the provider is required.
    pub require_energies: bool,
}

impl AnnealingRequirements {
    /// Derives requirements from a workload.
    pub fn from_workload(workload: &AnnealingWorkload) -> Self {
        Self {
            minimum_variables: workload.variable_count(),
            require_annealing: true,
            require_custom_schedule: workload.controls.schedule.points.len() > 2,
            require_deterministic_seed: workload.controls.seed.is_some(),
            require_spin_reversal_transforms:
                workload.controls.spin_reversal_transforms > 0,
            require_embedding: workload.controls.chain_strength.is_some(),
            require_qubo: matches!(workload.problem, AnnealingProblem::Qubo(_)),
            require_ising: matches!(workload.problem, AnnealingProblem::Ising(_)),
            require_samples: true,
            require_energies: true,
        }
    }
}

// =============================================================================
// Sample/result model
// =============================================================================

/// One annealing sample.
///
/// The assignment uses the domain of the submitted workload:
///
/// - QUBO: `0` or `1`;
/// - Ising: `-1` or `+1`.
///
/// To keep the representation compact and deterministic, binary samples use
/// `i8` as well; `0/1` are valid binary values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSample {
    /// Variable assignment.
    pub assignment: Vec<i8>,

    /// Objective energy.
    pub energy: f64,

    /// Number of times this exact sample occurred.
    pub occurrences: u64,

    /// Optional provider-neutral sample timing.
    pub timing: Option<f64>,
}

impl AnnealingSample {
    /// Creates a sample.
    pub fn new(
        assignment: Vec<i8>,
        energy: f64,
        occurrences: u64,
    ) -> Result<Self, AnnealingValidationError> {
        if occurrences == 0 {
            return Err(AnnealingValidationError::ZeroOccurrences);
        }

        validate_finite(energy, "sample energy")?;

        Ok(Self {
            assignment,
            energy,
            occurrences,
            timing: None,
        })
    }

    /// Adds optional timing information.
    pub fn with_timing(
        mut self,
        timing: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_non_negative_finite(timing, "sample timing")?;
        self.timing = Some(timing);
        Ok(self)
    }

    /// Validates the sample against a workload domain and variable count.
    pub fn validate(
        &self,
        variable_count: usize,
        domain: VariableDomain,
    ) -> Result<(), AnnealingValidationError> {
        if self.assignment.len() != variable_count {
            return Err(AnnealingValidationError::AssignmentLengthMismatch {
                expected: variable_count,
                actual: self.assignment.len(),
            });
        }

        validate_finite(self.energy, "sample energy")?;

        if self.occurrences == 0 {
            return Err(AnnealingValidationError::ZeroOccurrences);
        }

        for (index, value) in self.assignment.iter().copied().enumerate() {
            match domain {
                VariableDomain::Binary => {
                    if value != 0 && value != 1 {
                        return Err(AnnealingValidationError::InvalidBinaryValue {
                            index,
                            value: value as u8,
                        });
                    }
                }
                VariableDomain::Spin => {
                    if value != -1 && value != 1 {
                        return Err(AnnealingValidationError::InvalidSpinValue {
                            index,
                            value,
                        });
                    }
                }
            }
        }

        if let Some(timing) = self.timing {
            validate_non_negative_finite(timing, "sample timing")?;
        }

        Ok(())
    }
}

/// Normalized result of an annealing execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingResult {
    /// Schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Optional job/execution identifier.
    pub job_id: Option<String>,

    /// Optional backend identifier.
    ///
    /// This is only an opaque provider-neutral reference. No provider-specific
    /// backend structure is stored here.
    pub backend_id: Option<String>,

    /// Number of variables.
    pub variable_count: usize,

    /// Variable domain.
    pub domain: VariableDomain,

    /// Samples returned by the backend.
    pub samples: Vec<AnnealingSample>,

    /// Optional total execution time.
    pub execution_time: Option<f64>,

    /// Optional provider queue time.
    pub queue_time: Option<f64>,

    /// Optional seed used by the execution target.
    pub seed: Option<u64>,

    /// Non-secret deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl AnnealingResult {
    /// Creates an empty result for a workload.
    pub fn new(workload: &AnnealingWorkload) -> Result<Self, AnnealingValidationError> {
        workload
            .validate()
            .map_err(|errors| AnnealingValidationError::InvalidWorkload {
                reason: errors.to_string(),
            })?;

        Ok(Self {
            schema_id: ANNEALING_SCHEMA_ID.to_owned(),
            schema_version: ANNEALING_SCHEMA_VERSION,
            job_id: None,
            backend_id: None,
            variable_count: workload.variable_count(),
            domain: workload.domain(),
            samples: Vec::new(),
            execution_time: None,
            queue_time: None,
            seed: workload.controls.seed,
            metadata: BTreeMap::new(),
        })
    }

    /// Adds a sample.
    pub fn push_sample(
        &mut self,
        sample: AnnealingSample,
    ) -> Result<(), AnnealingValidationError> {
        if self.samples.len() >= MAX_RESULT_SAMPLES {
            return Err(AnnealingValidationError::TooManySamples {
                count: self.samples.len() + 1,
                maximum: MAX_RESULT_SAMPLES,
            });
        }

        sample.validate(self.variable_count, self.domain)?;

        self.samples.push(sample);

        Ok(())
    }

    /// Sets an opaque job identifier.
    pub fn with_job_id(
        mut self,
        job_id: impl Into<String>,
    ) -> Result<Self, AnnealingValidationError> {
        let job_id = job_id.into();

        validate_text_identifier(&job_id, "job_id", 512)?;

        self.job_id = Some(job_id);
        Ok(self)
    }

    /// Sets an opaque backend identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, AnnealingValidationError> {
        let backend_id = backend_id.into();

        validate_text_identifier(&backend_id, "backend_id", 512)?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Sets execution time.
    pub fn with_execution_time(
        mut self,
        execution_time: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_non_negative_finite(execution_time, "execution_time")?;
        self.execution_time = Some(execution_time);
        Ok(self)
    }

    /// Sets queue time.
    pub fn with_queue_time(
        mut self,
        queue_time: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_non_negative_finite(queue_time, "queue_time")?;
        self.queue_time = Some(queue_time);
        Ok(self)
    }

    /// Adds deterministic metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        add_metadata(&mut self.metadata, key.into(), value.into())
    }

    /// Returns the sample with minimum energy.
    pub fn best_sample(&self) -> Option<&AnnealingSample> {
        self.samples
            .iter()
            .min_by(|left, right| {
                left.energy
                    .partial_cmp(&right.energy)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Returns the minimum observed energy.
    pub fn best_energy(&self) -> Option<f64> {
        self.best_sample().map(|sample| sample.energy)
    }

    /// Returns total reported occurrences.
    pub fn total_occurrences(&self) -> u64 {
        self.samples
            .iter()
            .map(|sample| sample.occurrences)
            .sum()
    }

    /// Validates the complete result.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if self.schema_id != ANNEALING_SCHEMA_ID {
            errors.push(AnnealingValidationError::InvalidSchemaId {
                expected: ANNEALING_SCHEMA_ID.to_owned(),
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != ANNEALING_SCHEMA_VERSION {
            errors.push(AnnealingValidationError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if let Err(error) = validate_variable_count(self.variable_count) {
            errors.push(error);
        }

        if self.samples.len() > MAX_RESULT_SAMPLES {
            errors.push(AnnealingValidationError::TooManySamples {
                count: self.samples.len(),
                maximum: MAX_RESULT_SAMPLES,
            });
        }

        for sample in &self.samples {
            if let Err(error) =
                sample.validate(self.variable_count, self.domain)
            {
                errors.push(error);
            }
        }

        if let Some(value) = self.execution_time {
            if let Err(error) =
                validate_non_negative_finite(value, "execution_time")
            {
                errors.push(error);
            }
        }

        if let Some(value) = self.queue_time {
            if let Err(error) =
                validate_non_negative_finite(value, "queue_time")
            {
                errors.push(error);
            }
        }

        if let Some(job_id) = &self.job_id {
            if let Err(error) = validate_text_identifier(job_id, "job_id", 512) {
                errors.push(error);
            }
        }

        if let Some(backend_id) = &self.backend_id {
            if let Err(error) =
                validate_text_identifier(backend_id, "backend_id", 512)
            {
                errors.push(error);
            }
        }

        if let Err(error) = validate_metadata(&self.metadata) {
            errors.push(error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Evaluates the canonical objective represented by the result's domain.
    ///
    /// This is useful for independently verifying provider-reported energies.
    pub fn verify_sample_energy(
        &self,
        sample: &AnnealingSample,
        problem: &AnnealingProblem,
        tolerance: f64,
    ) -> Result<bool, AnnealingValidationError> {
        validate_finite(tolerance, "energy tolerance")?;

        if tolerance < 0.0 {
            return Err(AnnealingValidationError::NegativeValue {
                field: "energy tolerance",
                value: tolerance,
            });
        }

        if sample.assignment.len() != problem.variable_count() {
            return Err(AnnealingValidationError::AssignmentLengthMismatch {
                expected: problem.variable_count(),
                actual: sample.assignment.len(),
            });
        }

        let calculated = match problem {
            AnnealingProblem::Qubo(qubo) => {
                let assignment: Vec<u8> =
                    sample.assignment.iter().copied().map(|v| v as u8).collect();

                qubo.evaluate(&assignment)?
            }
            AnnealingProblem::Ising(ising) => {
                ising.evaluate(&sample.assignment)?
            }
        };

        Ok((calculated - sample.energy).abs() <= tolerance)
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// Structured validation failure for annealing models.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnnealingValidationError {
    /// Variable count is invalid.
    InvalidVariableCount {
        /// Supplied variable count.
        count: usize,

        /// Maximum supported by the canonical representation.
        maximum: usize,
    },

    /// Too many interaction terms.
    TooManyTerms {
        /// Supplied number of terms.
        count: usize,

        /// Maximum permitted.
        maximum: usize,
    },

    /// Variable index is outside the declared domain.
    VariableOutOfRange {
        /// Invalid variable index.
        index: usize,

        /// Declared variable count.
        variable_count: usize,
    },

    /// Interaction is diagonal.
    DiagonalInteraction {
        /// Variable index.
        index: usize,
    },

    /// Interaction is not in canonical `i < j` form.
    NonCanonicalInteraction {
        /// First index.
        i: usize,

        /// Second index.
        j: usize,
    },

    /// A floating-point value is not finite.
    NonFiniteValue {
        /// Field being validated.
        field: String,

        /// Actual value.
        value: f64,
    },

    /// A numeric value is negative where only non-negative values are allowed.
    NegativeValue {
        /// Field being validated.
        field: String,

        /// Actual value.
        value: f64,
    },

    /// A numeric value must be greater than zero.
    NonPositiveValue {
        /// Field being validated.
        field: String,

        /// Actual value.
        value: f64,
    },

    /// Invalid metadata key/value.
    InvalidMetadata {
        /// Reason.
        reason: String,
    },

    /// Invalid text identifier.
    InvalidIdentifier {
        /// Field name.
        field: String,

        /// Reason.
        reason: String,
    },

    /// Schedule contains too few points.
    InsufficientSchedulePoints {
        /// Actual point count.
        count: usize,
    },

    /// Schedule contains too many points.
    TooManySchedulePoints {
        /// Actual point count.
        count: usize,

        /// Maximum permitted.
        maximum: usize,
    },

    /// Schedule times are not strictly increasing.
    NonMonotonicSchedule {
        /// Previous point index.
        previous_index: usize,

        /// Current point index.
        current_index: usize,

        /// Previous time.
        previous_time: f64,

        /// Current time.
        current_time: f64,
    },

    /// Schedule does not begin at time zero.
    InvalidScheduleStart {
        /// Actual first time.
        time: f64,
    },

    /// Zero reads/shots were requested.
    ZeroReads,

    /// A sample has zero occurrences.
    ZeroOccurrences,

    /// Assignment has incorrect dimensionality.
    AssignmentLengthMismatch {
        /// Required length.
        expected: usize,

        /// Actual length.
        actual: usize,
    },

    /// Invalid binary value.
    InvalidBinaryValue {
        /// Variable index.
        index: usize,

        /// Invalid value.
        value: u8,
    },

    /// Invalid spin value.
    InvalidSpinValue {
        /// Variable index.
        index: usize,

        /// Invalid value.
        value: i8,
    },

    /// Too many result samples.
    TooManySamples {
        /// Actual sample count.
        count: usize,

        /// Maximum permitted.
        maximum: usize,
    },

    /// Invalid schema identifier.
    InvalidSchemaId {
        /// Expected schema identifier.
        expected: String,

        /// Actual identifier.
        actual: String,
    },

    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version encountered.
        version: u16,
    },

    /// Workload validation failed.
    InvalidWorkload {
        /// Human-readable reason.
        reason: String,
    },
}

impl fmt::Display for AnnealingValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVariableCount { count, maximum } => {
                write!(
                    formatter,
                    "invalid annealing variable count {count}; maximum is {maximum}"
                )
            }

            Self::TooManyTerms { count, maximum } => {
                write!(
                    formatter,
                    "too many annealing interaction terms: {count}; maximum is {maximum}"
                )
            }

            Self::VariableOutOfRange {
                index,
                variable_count,
            } => {
                write!(
                    formatter,
                    "variable index {index} is outside variable count {variable_count}"
                )
            }

            Self::DiagonalInteraction { index } => {
                write!(
                    formatter,
                    "diagonal interaction for variable {index} is not a quadratic interaction"
                )
            }

            Self::NonCanonicalInteraction { i, j } => {
                write!(
                    formatter,
                    "interaction ({i}, {j}) is not in canonical i < j form"
                )
            }

            Self::NonFiniteValue { field, value } => {
                write!(formatter, "{field} must be finite; received {value}")
            }

            Self::NegativeValue { field, value } => {
                write!(formatter, "{field} must be non-negative; received {value}")
            }

            Self::NonPositiveValue { field, value } => {
                write!(formatter, "{field} must be greater than zero; received {value}")
            }

            Self::InvalidMetadata { reason } => {
                write!(formatter, "invalid annealing metadata: {reason}")
            }

            Self::InvalidIdentifier { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }

            Self::InsufficientSchedulePoints { count } => {
                write!(
                    formatter,
                    "annealing schedule requires at least two points; received {count}"
                )
            }

            Self::TooManySchedulePoints { count, maximum } => {
                write!(
                    formatter,
                    "too many annealing schedule points: {count}; maximum is {maximum}"
                )
            }

            Self::NonMonotonicSchedule {
                previous_index,
                current_index,
                previous_time,
                current_time,
            } => {
                write!(
                    formatter,
                    "annealing schedule times must increase strictly: point \
                     {previous_index} has {previous_time}, point {current_index} \
                     has {current_time}"
                )
            }

            Self::InvalidScheduleStart { time } => {
                write!(
                    formatter,
                    "annealing schedule must start at time 0; received {time}"
                )
            }

            Self::ZeroReads => {
                formatter.write_str("annealing reads/shots must be greater than zero")
            }

            Self::ZeroOccurrences => {
                formatter.write_str("annealing sample occurrences must be greater than zero")
            }

            Self::AssignmentLengthMismatch { expected, actual } => {
                write!(
                    formatter,
                    "annealing assignment length mismatch: expected {expected}, received {actual}"
                )
            }

            Self::InvalidBinaryValue { index, value } => {
                write!(
                    formatter,
                    "invalid binary value {value} at variable {index}; expected 0 or 1"
                )
            }

            Self::InvalidSpinValue { index, value } => {
                write!(
                    formatter,
                    "invalid spin value {value} at variable {index}; expected -1 or +1"
                )
            }

            Self::TooManySamples { count, maximum } => {
                write!(
                    formatter,
                    "too many annealing result samples: {count}; maximum is {maximum}"
                )
            }

            Self::InvalidSchemaId { expected, actual } => {
                write!(
                    formatter,
                    "invalid annealing schema id: expected {expected}, received {actual}"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported annealing schema version {version}"
                )
            }

            Self::InvalidWorkload { reason } => {
                write!(formatter, "invalid annealing workload: {reason}")
            }
        }
    }
}

impl Error for AnnealingValidationError {}

/// Collection of structured annealing validation failures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingValidationErrors {
    /// Validation failures in deterministic insertion order.
    pub errors: Vec<AnnealingValidationError>,
}

impl AnnealingValidationErrors {
    /// Creates an empty collection.
    pub const fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// Creates a collection containing one error.
    pub fn single(error: AnnealingValidationError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    /// Adds an error.
    pub fn push(&mut self, error: AnnealingValidationError) {
        self.errors.push(error);
    }

    /// Adds all errors from another collection.
    pub fn extend(&mut self, errors: Self) {
        self.errors.extend(errors.errors);
    }

    /// Returns whether there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }
}

impl Default for AnnealingValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AnnealingValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.errors.iter().enumerate() {
            if index > 0 {
                formatter.write_str("; ")?;
            }

            write!(formatter, "{error}")?;
        }

        Ok(())
    }
}

impl Error for AnnealingValidationErrors {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_variable_count(
    count: usize,
) -> Result<(), AnnealingValidationError> {
    if count == 0 {
        return Err(AnnealingValidationError::InvalidVariableCount {
            count,
            maximum: MAX_VARIABLES,
        });
    }

    if count > MAX_VARIABLES {
        return Err(AnnealingValidationError::InvalidVariableCount {
            count,
            maximum: MAX_VARIABLES,
        });
    }

    Ok(())
}

fn validate_finite(
    value: f64,
    field: &str,
) -> Result<(), AnnealingValidationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AnnealingValidationError::NonFiniteValue {
            field: field.to_owned(),
            value,
        })
    }
}

fn validate_non_negative_finite(
    value: f64,
    field: &str,
) -> Result<(), AnnealingValidationError> {
    validate_finite(value, field)?;

    if value < 0.0 {
        return Err(AnnealingValidationError::NegativeValue {
            field: field.to_owned(),
            value,
        });
    }

    Ok(())
}

fn validate_text_identifier(
    value: &str,
    field: &str,
    maximum_length: usize,
) -> Result<(), AnnealingValidationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(AnnealingValidationError::InvalidIdentifier {
            field: field.to_owned(),
            reason: "identifier must not be empty".to_owned(),
        });
    }

    if trimmed.len() > maximum_length {
        return Err(AnnealingValidationError::InvalidIdentifier {
            field: field.to_owned(),
            reason: format!(
                "identifier length {} exceeds maximum {maximum_length}",
                trimmed.len()
            ),
        });
    }

    if trimmed.chars().any(char::is_control) {
        return Err(AnnealingValidationError::InvalidIdentifier {
            field: field.to_owned(),
            reason: "identifier must not contain control characters".to_owned(),
        });
    }

    Ok(())
}

fn add_metadata(
    metadata: &mut BTreeMap<String, String>,
    key: String,
    value: String,
) -> Result<(), AnnealingValidationError> {
    if metadata.len() >= MAX_METADATA_ENTRIES && !metadata.contains_key(&key) {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: format!(
                "metadata entry limit of {MAX_METADATA_ENTRIES} exceeded"
            ),
        });
    }

    if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: format!(
                "metadata key must contain 1..={MAX_METADATA_KEY_LENGTH} bytes"
            ),
        });
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: format!(
                "metadata value exceeds {MAX_METADATA_VALUE_LENGTH} bytes"
            ),
        });
    }

    if key.chars().any(char::is_control) {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: "metadata key contains a control character".to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: "metadata value contains a control character".to_owned(),
        });
    }

    metadata.insert(key, value);

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<(), AnnealingValidationError> {
    if metadata.len() > MAX_METADATA_ENTRIES {
        return Err(AnnealingValidationError::InvalidMetadata {
            reason: format!(
                "metadata contains {} entries; maximum is {MAX_METADATA_ENTRIES}",
                metadata.len()
            ),
        });
    }

    for (key, value) in metadata {
        if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
            return Err(AnnealingValidationError::InvalidMetadata {
                reason: "metadata key length is invalid".to_owned(),
            });
        }

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(AnnealingValidationError::InvalidMetadata {
                reason: "metadata value length is invalid".to_owned(),
            });
        }

        if key.chars().any(char::is_control) {
            return Err(AnnealingValidationError::InvalidMetadata {
                reason: "metadata key contains a control character".to_owned(),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(AnnealingValidationError::InvalidMetadata {
                reason: "metadata value contains a control character".to_owned(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_qubo_is_rejected() {
        assert!(QuboProblem::new(0).is_err());
    }

    #[test]
    fn qubo_linear_energy_is_correct() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        qubo.add_linear(0, 2.0).expect("valid coefficient");
        qubo.add_linear(1, -1.0).expect("valid coefficient");

        let energy = qubo.evaluate(&[1, 0]).expect("valid assignment");

        assert!((energy - 2.0).abs() < DEFAULT_ENERGY_TOLERANCE);
    }

    #[test]
    fn qubo_quadratic_energy_is_correct() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        qubo.add_linear(0, 1.0).expect("valid coefficient");
        qubo.add_linear(1, 2.0).expect("valid coefficient");
        qubo.add_quadratic(0, 1, 3.0)
            .expect("valid coefficient");

        let energy = qubo.evaluate(&[1, 1]).expect("valid assignment");

        assert!((energy - 6.0).abs() < DEFAULT_ENERGY_TOLERANCE);
    }

    #[test]
    fn quadratic_indices_are_canonicalized() {
        let term =
            QuboTerm::quadratic(5, 2, 3.0).expect("valid quadratic term");

        assert_eq!(term.i, 2);
        assert_eq!(term.j, Some(5));
    }

    #[test]
    fn qubo_to_ising_preserves_energy() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.offset = 1.25;
        qubo.add_linear(0, 2.0).expect("valid coefficient");
        qubo.add_linear(1, -3.0).expect("valid coefficient");
        qubo.add_quadratic(0, 1, 4.0)
            .expect("valid coefficient");
        qubo.add_quadratic(1, 2, -2.0)
            .expect("valid coefficient");

        let ising = qubo.to_ising().expect("conversion succeeds");

        let binary = [1_u8, 0_u8, 1_u8];

        let spin = [
            if binary[0] == 0 { -1 } else { 1 },
            if binary[1] == 0 { -1 } else { 1 },
            if binary[2] == 0 { -1 } else { 1 },
        ];

        let qubo_energy = qubo.evaluate(&binary).expect("valid QUBO energy");
        let ising_energy = ising.evaluate(&spin).expect("valid Ising energy");

        assert!(
            (qubo_energy - ising_energy).abs()
                < DEFAULT_ENERGY_TOLERANCE
        );
    }

    #[test]
    fn ising_to_qubo_preserves_energy() {
        let mut ising = IsingModel::new(3).expect("valid Ising");

        ising.offset = 0.75;
        ising.add_linear(0, 1.5).expect("valid coefficient");
        ising.add_linear(1, -2.0).expect("valid coefficient");
        ising.add_quadratic(0, 1, 0.5)
            .expect("valid coefficient");
        ising.add_quadratic(1, 2, -1.25)
            .expect("valid coefficient");

        let qubo = ising.to_qubo().expect("conversion succeeds");

        let spin = [1_i8, -1_i8, 1_i8];

        let binary = [
            if spin[0] == -1 { 0 } else { 1 },
            if spin[1] == -1 { 0 } else { 1 },
            if spin[2] == -1 { 0 } else { 1 },
        ];

        let ising_energy = ising.evaluate(&spin).expect("valid Ising energy");
        let qubo_energy = qubo.evaluate(&binary).expect("valid QUBO energy");

        assert!(
            (ising_energy - qubo_energy).abs()
                < DEFAULT_ENERGY_TOLERANCE
        );
    }

    #[test]
    fn linear_schedule_is_valid() {
        let schedule =
            AnnealingSchedule::linear(10.0).expect("valid schedule");

        schedule.validate().expect("schedule validates");

        assert_eq!(schedule.points.len(), 2);
        assert_eq!(schedule.points[0].time, 0.0);
        assert_eq!(schedule.points[1].time, 10.0);
        assert_eq!(schedule.points[1].value, 1.0);
    }

    #[test]
    fn non_monotonic_schedule_is_rejected() {
        let points = vec![
            AnnealingSchedulePoint {
                time: 0.0,
                value: 0.0,
            },
            AnnealingSchedulePoint {
                time: 2.0,
                value: 0.5,
            },
            AnnealingSchedulePoint {
                time: 1.0,
                value: 1.0,
            },
        ];

        assert!(AnnealingSchedule::new(points).is_err());
    }

    #[test]
    fn schedule_must_start_at_zero() {
        let points = vec![
            AnnealingSchedulePoint {
                time: 1.0,
                value: 0.0,
            },
            AnnealingSchedulePoint {
                time: 2.0,
                value: 1.0,
            },
        ];

        assert!(AnnealingSchedule::new(points).is_err());
    }

    #[test]
    fn zero_reads_are_rejected() {
        let mut controls = AnnealingControls::default();
        controls.reads = 0;

        assert!(controls.validate().is_err());
    }

    #[test]
    fn invalid_binary_sample_is_rejected() {
        let sample =
            AnnealingSample::new(vec![0, 2], 1.0, 1)
                .expect("construction itself is valid");

        assert!(sample
            .validate(2, VariableDomain::Binary)
            .is_err());
    }

    #[test]
    fn invalid_spin_sample_is_rejected() {
        let sample =
            AnnealingSample::new(vec![1, 0], 1.0, 1)
                .expect("construction itself is valid");

        assert!(sample
            .validate(2, VariableDomain::Spin)
            .is_err());
    }

    #[test]
    fn best_sample_is_deterministic() {
        let problem =
            QuboProblem::new(2).expect("valid problem");

        let workload = AnnealingWorkload::new(
            AnnealingProblem::Qubo(problem),
            AnnealingControls::default(),
        )
        .expect("valid workload");

        let mut result =
            AnnealingResult::new(&workload).expect("valid result");

        result
            .push_sample(
                AnnealingSample::new(vec![0, 0], 2.0, 1)
                    .expect("valid sample"),
            )
            .expect("sample accepted");

        result
            .push_sample(
                AnnealingSample::new(vec![1, 1], -3.0, 2)
                    .expect("valid sample"),
            )
            .expect("sample accepted");

        assert_eq!(
            result.best_sample().expect("best sample").energy,
            -3.0
        );
    }

    #[test]
    fn result_energy_verification_works() {
        let mut qubo =
            QuboProblem::new(2).expect("valid problem");

        qubo.add_linear(0, 1.0).expect("valid coefficient");
        qubo.add_linear(1, 2.0).expect("valid coefficient");
        qubo.add_quadratic(0, 1, -4.0)
            .expect("valid coefficient");

        let problem = AnnealingProblem::Qubo(qubo.clone());

        let controls = AnnealingControls::default();

        let workload =
            AnnealingWorkload::new(problem.clone(), controls)
                .expect("valid workload");

        let result =
            AnnealingResult::new(&workload).expect("valid result");

        let sample =
            AnnealingSample::new(vec![1, 1], -1.0, 1)
                .expect("valid sample");

        let verified = result
            .verify_sample_energy(
                &sample,
                &problem,
                DEFAULT_ENERGY_TOLERANCE,
            )
            .expect("verification succeeds");

        assert!(verified);
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut problem =
            QuboProblem::new(2).expect("valid problem");

        problem
            .add_metadata("z", "last")
            .expect("metadata accepted");

        problem
            .add_metadata("a", "first")
            .expect("metadata accepted");

        let keys: Vec<&String> = problem.metadata.keys().collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn workload_requirements_follow_workload() {
        let problem =
            QuboProblem::new(5).expect("valid problem");

        let mut controls = AnnealingControls::default();
        controls.seed = Some(42);

        let workload = AnnealingWorkload::new(
            AnnealingProblem::Qubo(problem),
            controls,
        )
        .expect("valid workload");

        let requirements =
            AnnealingRequirements::from_workload(&workload);

        assert_eq!(requirements.minimum_variables, 5);
        assert!(requirements.require_annealing);
        assert!(requirements.require_qubo);
        assert!(!requirements.require_ising);
        assert!(requirements.require_deterministic_seed);
    }

    #[test]
    fn schema_constants_are_stable() {
        assert_eq!(
            ANNEALING_SCHEMA_ID,
            "zamani.quantum.hardware.annealing"
        );

        assert_eq!(ANNEALING_SCHEMA_VERSION, 1);
    }

    #[test]
    fn non_finite_coefficients_are_rejected() {
        let result = QuboTerm::linear(0, f64::NAN);

        assert!(result.is_err());

        let result = QuboTerm::linear(0, f64::INFINITY);

        assert!(result.is_err());
    }

    #[test]
    fn zero_duration_schedule_is_rejected() {
        assert!(AnnealingSchedule::linear(0.0).is_err());
        assert!(AnnealingSchedule::linear(-1.0).is_err());
    }
}