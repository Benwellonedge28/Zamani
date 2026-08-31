//! Zamani Quantum IR — Canonical QUBO Model
//!
//! Path:
//!     src/quantum/ir/model/qubo.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent representation of a
//! Quadratic Unconstrained Binary Optimization (QUBO) problem.
//!
//! The semantic objective is:
//!
//! ```text
//! E(x) = c
//!      + Σ_i Q[i,i] x_i
//!      + Σ_{i<j} Q[i,j] x_i x_j
//! ```
//!
//! where:
//!
//! ```text
//! x_i ∈ {0, 1}
//! ```
//!
//! The representation is sparse and deterministic.
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - QUBO variable identity;
//! - QUBO variable count;
//! - constant objective offset;
//! - linear coefficients;
//! - quadratic coefficients;
//! - deterministic canonicalization;
//! - objective evaluation;
//! - QUBO → Ising mathematical conversion;
//! - structural validation;
//! - finite-number validation;
//! - semantic equality;
//! - provider-neutral metadata;
//! - serde persistence.
//!
//! This module does NOT own:
//!
//! - physical qubits;
//! - logical qubits;
//! - hardware topology;
//! - annealing hardware;
//! - gate synthesis;
//! - routing;
//! - embedding;
//! - scheduling;
//! - device calibration;
//! - provider APIs;
//! - job submission;
//! - simulation;
//! - optimization algorithms that transform the mathematical problem.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Important architectural rule
//!
//! A QUBO variable is NOT a `QubitId`.
//!
//! A QUBO variable is a mathematical binary decision variable. It may later
//! be embedded onto logical or physical quantum resources, but that mapping is
//! not part of the canonical mathematical QUBO.
//!
//! Therefore this file intentionally does not import:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! A downstream embedding/mapping layer may use `QubitId` when translating this
//! model to a quantum execution representation.
//!
//! # Scalability
//!
//! There are deliberately no constants such as:
//!
//! ```text
//! MAX_QUBO_VARIABLES
//! MAX_QUBO_TERMS
//! MAX_QUBO_EDGES
//! MAX_PROBLEM_SIZE
//! ```
//!
//! The semantic model is not artificially bounded by the IR.
//!
//! The practical limit is determined by:
//!
//! - the selected integer representation;
//! - available memory;
//! - compilation policy;
//! - serialization policy;
//! - execution target;
//! - backend capacity.
//!
//! `u64` is used for semantic variable identity and cardinality. It provides a
//! stable, platform-independent representation and avoids using `usize` as a
//! semantic limit.
//!
//! Collection implementations may use `usize` internally because Rust
//! containers require host-addressable storage. That implementation detail is
//! not part of the QUBO semantic model.
//!
//! # Canonical representation
//!
//! Linear coefficients are stored as:
//!
//! ```text
//! BTreeMap<QuboVariableId, f64>
//! ```
//!
//! Quadratic coefficients are stored as:
//!
//! ```text
//! BTreeMap<(QuboVariableId, QuboVariableId), f64>
//! ```
//!
//! with the invariant:
//!
//! ```text
//! first < second
//! ```
//!
//! Thus `(i,j)` and `(j,i)` are the same mathematical interaction and have only
//! one canonical representation.
//!
//! Zero coefficients are removed from the canonical representation.
//!
//! # Determinism
//!
//! `BTreeMap` is intentionally used rather than `HashMap`.
//!
//! Therefore:
//!
//! - iteration order is deterministic;
//! - serialization order is deterministic when serde's map serializer preserves
//!   map ordering;
//! - canonical term traversal is deterministic;
//! - conversion to Ising is deterministic;
//! - objective evaluation does not depend on hash-map ordering.
//!
//! # Numerical safety
//!
//! QUBO coefficients and offsets use `f64`, but NaN and infinities are never
//! accepted.
//!
//! This is important because:
//!
//! ```text
//! NaN
//! +∞
//! -∞
//! ```
//!
//! would make validation, comparison, optimization, serialization and backend
//! behavior ambiguous.
//!
//! # Rust contract
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! Upstream:
//!
//! ```text
//! Zamani frontend
//!      │
//!      ▼
//! quantum::ir::model::qubo
//! ```
//!
//! Downstream:
//!
//! ```text
//! QUBO
//!  │
//!  ├── Ising conversion
//!  │
//!  ├── resource estimation
//!  │
//!  ├── annealing model
//!  │
//!  ├── optimization
//!  │
//!  └── hardware embedding
//!          │
//!          ▼
//! quantum::ir::qubit::QubitId
//! ```
//!
//! The reverse dependency is forbidden.
//!
//! # Serialization
//!
//! Serde is used because it is already part of the Zamani dependency surface.
//!
//! The serialized representation is intended for:
//!
//! - reproducibility;
//! - compiler artifacts;
//! - benchmarking;
//! - optimization checkpoints;
//! - Danga integration;
//! - provenance;
//! - backend adapters.
//!
//! Credentials and provider secrets do not belong here.
//!
//! # Security
//!
//! This module performs no network access, filesystem access, process spawning,
//! randomness or unsafe operations.
//!
//! Resource exhaustion limits belong to the compilation/service policy rather
//! than being encoded as semantic QUBO limits.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable schema identifier for the canonical Zamani QUBO model.
pub const QUBO_SCHEMA_ID: &str = "zamani.quantum.ir.model.qubo";

/// Current semantic schema version.
pub const QUBO_SCHEMA_VERSION: u16 = 1;

/// Mathematical tolerance used by callers when comparing floating-point
/// energies produced by equivalent transformations.
///
/// This is a comparison policy, not part of the QUBO semantics.
pub const DEFAULT_ENERGY_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Variable identity
// =============================================================================

/// Stable semantic identity of a QUBO binary variable.
///
/// This is deliberately distinct from:
///
/// - `quantum::ir::qubit::QubitId`;
/// - `PhysicalQubitId`;
/// - Rust `usize`.
///
/// A QUBO variable represents a mathematical decision variable, not a quantum
/// resource.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct QuboVariableId(pub u64);

impl QuboVariableId {
    /// Creates a variable identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable semantic value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for QuboVariableId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for QuboVariableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "qv{}", self.0)
    }
}

// =============================================================================
// Sparse interaction identity
// =============================================================================

/// Canonical identity of a quadratic interaction.
///
/// The invariant is:
///
/// ```text
/// first < second
/// ```
///
/// This guarantees that a QUBO interaction has exactly one representation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct QuboInteraction {
    /// First variable.
    pub first: QuboVariableId,

    /// Second variable.
    pub second: QuboVariableId,
}

impl QuboInteraction {
    /// Creates a canonical interaction from two distinct variables.
    ///
    /// The arguments may be supplied in either order.
    pub fn new(
        first: QuboVariableId,
        second: QuboVariableId,
    ) -> Result<Self, QuboError> {
        if first == second {
            return Err(QuboError::SelfInteraction {
                variable: first,
            });
        }

        let (first, second) = if first < second {
            (first, second)
        } else {
            (second, first)
        };

        Ok(Self { first, second })
    }

    /// Returns the two variables in canonical order.
    #[must_use]
    pub const fn variables(self) -> (QuboVariableId, QuboVariableId) {
        (self.first, self.second)
    }
}

// =============================================================================
// QUBO terms
// =============================================================================

/// One canonical QUBO term.
///
/// A linear term contains one variable.
/// A quadratic term contains two distinct variables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuboTerm {
    /// Linear term:
    ///
    /// ```text
    /// coefficient * x_i
    /// ```
    Linear {
        /// Variable.
        variable: QuboVariableId,

        /// Coefficient.
        coefficient: f64,
    },

    /// Quadratic interaction:
    ///
    /// ```text
    /// coefficient * x_i * x_j
    /// ```
    Quadratic {
        /// Canonical interaction.
        interaction: QuboInteraction,

        /// Coefficient.
        coefficient: f64,
    },
}

impl QuboTerm {
    /// Creates a linear term.
    pub fn linear(
        variable: QuboVariableId,
        coefficient: f64,
    ) -> Result<Self, QuboError> {
        validate_finite(coefficient, "linear coefficient")?;

        Ok(Self::Linear {
            variable,
            coefficient,
        })
    }

    /// Creates a quadratic term.
    ///
    /// The variable order is canonicalized automatically.
    pub fn quadratic(
        first: QuboVariableId,
        second: QuboVariableId,
        coefficient: f64,
    ) -> Result<Self, QuboError> {
        validate_finite(coefficient, "quadratic coefficient")?;

        Ok(Self::Quadratic {
            interaction: QuboInteraction::new(first, second)?,
            coefficient,
        })
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> f64 {
        match self {
            Self::Linear { coefficient, .. }
            | Self::Quadratic { coefficient, .. } => *coefficient,
        }
    }

    /// Returns true for a linear term.
    #[must_use]
    pub const fn is_linear(&self) -> bool {
        matches!(self, Self::Linear { .. })
    }

    /// Returns true for a quadratic term.
    #[must_use]
    pub const fn is_quadratic(&self) -> bool {
        matches!(self, Self::Quadratic { .. })
    }
}

// =============================================================================
// QUBO problem
// =============================================================================

/// Canonical sparse Quadratic Unconstrained Binary Optimization problem.
///
/// The objective is:
///
/// ```text
/// E(x) = offset
///      + Σ_i Q[i] x_i
///      + Σ_{i<j} Q[i,j] x_i x_j
/// ```
///
/// where:
///
/// ```text
/// x_i ∈ {0,1}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuboProblem {
    /// Number of semantic binary variables.
    ///
    /// This is `u64` rather than `usize` so the semantic model does not depend
    /// on host pointer width.
    variable_count: u64,

    /// Constant energy offset.
    offset: f64,

    /// Sparse linear coefficients.
    ///
    /// Zero coefficients are not stored.
    linear: BTreeMap<QuboVariableId, f64>,

    /// Sparse quadratic coefficients.
    ///
    /// Keys are always canonicalized such that:
    ///
    /// ```text
    /// first < second
    /// ```
    ///
    /// Zero coefficients are not stored.
    quadratic: BTreeMap<QuboInteraction, f64>,

    /// Optional stable problem identifier.
    problem_id: Option<String>,

    /// Deterministic, non-secret metadata.
    metadata: BTreeMap<String, String>,
}

impl QuboProblem {
    /// Creates an empty QUBO containing `variable_count` variables.
    pub fn new(variable_count: u64) -> Result<Self, QuboError> {
        Ok(Self {
            variable_count,
            offset: 0.0,
            linear: BTreeMap::new(),
            quadratic: BTreeMap::new(),
            problem_id: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Returns the number of semantic variables.
    #[must_use]
    pub const fn variable_count(&self) -> u64 {
        self.variable_count
    }

    /// Returns the constant objective offset.
    #[must_use]
    pub const fn offset(&self) -> f64 {
        self.offset
    }

    /// Returns the sparse linear coefficients.
    #[must_use]
    pub fn linear_terms(&self) -> &BTreeMap<QuboVariableId, f64> {
        &self.linear
    }

    /// Returns the sparse quadratic coefficients.
    #[must_use]
    pub fn quadratic_terms(&self) -> &BTreeMap<QuboInteraction, f64> {
        &self.quadratic
    }

    /// Returns the optional problem identifier.
    #[must_use]
    pub fn problem_id(&self) -> Option<&str> {
        self.problem_id.as_deref()
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Sets the constant offset.
    pub fn set_offset(&mut self, offset: f64) -> Result<(), QuboError> {
        validate_finite(offset, "QUBO offset")?;
        self.offset = offset;
        Ok(())
    }

    /// Builder-style offset setter.
    pub fn with_offset(mut self, offset: f64) -> Result<Self, QuboError> {
        self.set_offset(offset)?;
        Ok(self)
    }

    /// Sets a problem identifier.
    pub fn set_problem_id(
        &mut self,
        problem_id: impl Into<String>,
    ) {
        self.problem_id = Some(problem_id.into());
    }

    /// Builder-style problem identifier setter.
    pub fn with_problem_id(
        mut self,
        problem_id: impl Into<String>,
    ) -> Self {
        self.set_problem_id(problem_id);
        self
    }

    /// Adds or accumulates a linear coefficient.
    ///
    /// If the resulting coefficient is zero, the canonical term is removed.
    pub fn add_linear(
        &mut self,
        variable: QuboVariableId,
        coefficient: f64,
    ) -> Result<(), QuboError> {
        self.validate_variable(variable)?;
        validate_finite(coefficient, "linear coefficient")?;

        if coefficient == 0.0 {
            return Ok(());
        }

        let entry = self.linear.entry(variable).or_insert(0.0);
        *entry += coefficient;

        validate_finite(*entry, "accumulated linear coefficient")?;

        if *entry == 0.0 {
            self.linear.remove(&variable);
        }

        Ok(())
    }

    /// Adds or accumulates a quadratic coefficient.
    ///
    /// `(i,j)` and `(j,i)` are automatically canonicalized to the same
    /// interaction.
    pub fn add_quadratic(
        &mut self,
        first: QuboVariableId,
        second: QuboVariableId,
        coefficient: f64,
    ) -> Result<(), QuboError> {
        self.validate_variable(first)?;
        self.validate_variable(second)?;
        validate_finite(coefficient, "quadratic coefficient")?;

        let interaction = QuboInteraction::new(first, second)?;

        if coefficient == 0.0 {
            return Ok(());
        }

        let entry = self.quadratic.entry(interaction).or_insert(0.0);
        *entry += coefficient;

        validate_finite(*entry, "accumulated quadratic coefficient")?;

        if *entry == 0.0 {
            self.quadratic.remove(&interaction);
        }

        Ok(())
    }

    /// Adds a generic canonical term.
    pub fn add_term(&mut self, term: QuboTerm) -> Result<(), QuboError> {
        match term {
            QuboTerm::Linear {
                variable,
                coefficient,
            } => self.add_linear(variable, coefficient),

            QuboTerm::Quadratic {
                interaction,
                coefficient,
            } => self.add_quadratic(
                interaction.first,
                interaction.second,
                coefficient,
            ),
        }
    }

    /// Returns the coefficient of a linear variable.
    #[must_use]
    pub fn linear_coefficient(&self, variable: QuboVariableId) -> f64 {
        self.linear.get(&variable).copied().unwrap_or(0.0)
    }

    /// Returns the coefficient of a quadratic interaction.
    #[must_use]
    pub fn quadratic_coefficient(
        &self,
        first: QuboVariableId,
        second: QuboVariableId,
    ) -> Result<f64, QuboError> {
        let interaction = QuboInteraction::new(first, second)?;
        Ok(self.quadratic.get(&interaction).copied().unwrap_or(0.0))
    }

    /// Evaluates the objective using a sparse binary assignment.
    ///
    /// Variables absent from `assignment` are interpreted as zero.
    ///
    /// This API is useful for very large sparse QUBOs because it does not
    /// require allocating a dense vector of the entire variable space.
    pub fn evaluate_sparse<I>(
        &self,
        assignment: I,
    ) -> Result<f64, QuboEvaluationError>
    where
        I: IntoIterator<Item = (QuboVariableId, bool)>,
    {
        let mut values = BTreeMap::new();

        for (variable, value) in assignment {
            self.validate_variable(variable)
                .map_err(QuboEvaluationError::InvalidProblem)?;

            if values.insert(variable, value).is_some() {
                return Err(QuboEvaluationError::DuplicateAssignment { variable });
            }
        }

        let mut energy = self.offset;

        for (variable, coefficient) in &self.linear {
            if values.get(variable).copied().unwrap_or(false) {
                energy += *coefficient;
            }
        }

        for (interaction, coefficient) in &self.quadratic {
            let first = values
                .get(&interaction.first)
                .copied()
                .unwrap_or(false);

            let second = values
                .get(&interaction.second)
                .copied()
                .unwrap_or(false);

            if first && second {
                energy += *coefficient;
            }
        }

        if !energy.is_finite() {
            return Err(QuboEvaluationError::NonFiniteResult);
        }

        Ok(energy)
    }

    /// Evaluates the objective using a dense iterator.
    ///
    /// The iterator must contain exactly `variable_count` values in variable
    /// ID order:
    ///
    /// ```text
    /// x_0, x_1, ..., x_(N-1)
    /// ```
    ///
    /// No `usize`-based semantic dimension is required.
    pub fn evaluate_dense<I>(
        &self,
        assignment: I,
    ) -> Result<f64, QuboEvaluationError>
    where
        I: IntoIterator<Item = bool>,
    {
        let mut values = BTreeMap::new();
        let mut index = 0_u64;

        for value in assignment {
            if index >= self.variable_count {
                return Err(QuboEvaluationError::AssignmentTooLarge {
                    expected: self.variable_count,
                });
            }

            values.insert(QuboVariableId::new(index), value);
            index = index
                .checked_add(1)
                .ok_or(QuboEvaluationError::AssignmentIndexOverflow)?;
        }

        if index != self.variable_count {
            return Err(QuboEvaluationError::AssignmentSizeMismatch {
                expected: self.variable_count,
                actual: index,
            });
        }

        self.evaluate_sparse(values)
    }

    /// Returns the number of non-zero linear terms.
    #[must_use]
    pub fn linear_term_count(&self) -> u64 {
        self.linear.len() as u64
    }

    /// Returns the number of non-zero quadratic terms.
    #[must_use]
    pub fn quadratic_term_count(&self) -> u64 {
        self.quadratic.len() as u64
    }

    /// Returns the total number of non-zero objective terms.
    #[must_use]
    pub fn term_count(&self) -> u64 {
        self.linear_term_count() + self.quadratic_term_count()
    }

    /// Returns the variables which occur in the objective.
    ///
    /// The result is deterministic.
    pub fn used_variables(&self) -> impl Iterator<Item = QuboVariableId> + '_ {
        let mut variables = BTreeMap::<QuboVariableId, ()>::new();

        for variable in self.linear.keys() {
            variables.insert(*variable, ());
        }

        for interaction in self.quadratic.keys() {
            variables.insert(interaction.first, ());
            variables.insert(interaction.second, ());
        }

        variables.into_keys()
    }

    /// Returns whether the QUBO contains no non-zero terms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.linear.is_empty() && self.quadratic.is_empty()
    }

    /// Returns the largest variable ID referenced by the objective.
    #[must_use]
    pub fn max_used_variable(&self) -> Option<QuboVariableId> {
        self.used_variables().next_back()
    }

    /// Validates the complete canonical representation.
    pub fn validate(&self) -> Result<(), QuboValidationErrors> {
        let mut errors = QuboValidationErrors::new();

        if !self.offset.is_finite() {
            errors.push(QuboError::NonFiniteCoefficient {
                field: "QUBO offset",
            });
        }

        for (variable, coefficient) in &self.linear {
            if variable.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: *variable,
                    variable_count: self.variable_count,
                });
            }

            if !coefficient.is_finite() {
                errors.push(QuboError::NonFiniteCoefficient {
                    field: "linear coefficient",
                });
            }

            if *coefficient == 0.0 {
                errors.push(QuboError::ExplicitZeroCoefficient {
                    variable: *variable,
                });
            }
        }

        for (interaction, coefficient) in &self.quadratic {
            if interaction.first >= interaction.second {
                errors.push(QuboError::NonCanonicalInteraction {
                    first: interaction.first,
                    second: interaction.second,
                });
            }

            if interaction.first.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: interaction.first,
                    variable_count: self.variable_count,
                });
            }

            if interaction.second.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: interaction.second,
                    variable_count: self.variable_count,
                });
            }

            if !coefficient.is_finite() {
                errors.push(QuboError::NonFiniteCoefficient {
                    field: "quadratic coefficient",
                });
            }

            if *coefficient == 0.0 {
                errors.push(QuboError::ExplicitZeroInteraction {
                    first: interaction.first,
                    second: interaction.second,
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Converts this QUBO to the canonical Ising representation.
    ///
    /// The transformation uses:
    ///
    /// ```text
    /// x_i = (s_i + 1) / 2
    /// ```
    ///
    /// and therefore:
    ///
    /// ```text
    /// J_ij = Q_ij / 4
    ///
    /// h_i = Q_ii / 2
    ///       + Σ_j Q_ij / 4
    ///
    /// offset_ising =
    ///       offset_qubo
    ///       + Σ_i Q_ii / 2
    ///       + Σ_{i<j} Q_ij / 4
    /// ```
    ///
    /// The resulting Ising model satisfies:
    ///
    /// ```text
    /// E_qubo(x) = E_ising(s)
    /// ```
    ///
    /// under `x_i = (s_i + 1) / 2`.
    pub fn to_ising(&self) -> Result<IsingModel, QuboConversionError> {
        self.validate()
            .map_err(QuboConversionError::InvalidQubo)?;

        let mut linear = BTreeMap::<QuboVariableId, f64>::new();
        let mut quadratic = BTreeMap::<QuboInteraction, f64>::new();

        let mut offset = self.offset;

        for (variable, coefficient) in &self.linear {
            let half = *coefficient / 2.0;

            add_finite(
                &mut linear,
                *variable,
                half,
                "Ising linear coefficient",
            )?;

            offset += half;
        }

        for (interaction, coefficient) in &self.quadratic {
            let quarter = *coefficient / 4.0;

            add_finite(
                &mut quadratic,
                *interaction,
                quarter,
                "Ising quadratic coefficient",
            )?;

            add_finite(
                &mut linear,
                interaction.first,
                quarter,
                "Ising linear coefficient",
            )?;

            add_finite(
                &mut linear,
                interaction.second,
                quarter,
                "Ising linear coefficient",
            )?;

            offset += quarter;
        }

        validate_finite(offset, "Ising offset")
            .map_err(QuboConversionError::InvalidCoefficient)?;

        Ok(IsingModel {
            variable_count: self.variable_count,
            offset,
            linear,
            quadratic,
            problem_id: self.problem_id.clone(),
            metadata: self.metadata.clone(),
        })
    }

    /// Creates a canonical QUBO from a collection of terms.
    ///
    /// Duplicate linear or quadratic terms are accumulated.
    pub fn from_terms<I>(
        variable_count: u64,
        terms: I,
    ) -> Result<Self, QuboValidationErrors>
    where
        I: IntoIterator<Item = QuboTerm>,
    {
        let mut problem = match Self::new(variable_count) {
            Ok(problem) => problem,
            Err(error) => return Err(QuboValidationErrors::single(error)),
        };

        let mut errors = QuboValidationErrors::new();

        for term in terms {
            if let Err(error) = problem.add_term(term) {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(problem)
        } else {
            Err(errors)
        }
    }

    fn validate_variable(
        &self,
        variable: QuboVariableId,
    ) -> Result<(), QuboError> {
        if variable.value() >= self.variable_count {
            return Err(QuboError::VariableOutOfRange {
                variable,
                variable_count: self.variable_count,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Ising representation
// =============================================================================

/// Canonical sparse Ising model produced by QUBO conversion.
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
    /// Number of semantic spin variables.
    variable_count: u64,

    /// Constant energy offset.
    offset: f64,

    /// Sparse linear fields.
    linear: BTreeMap<QuboVariableId, f64>,

    /// Sparse pairwise couplings.
    quadratic: BTreeMap<QuboInteraction, f64>,

    /// Optional originating problem identifier.
    problem_id: Option<String>,

    /// Propagated deterministic metadata.
    metadata: BTreeMap<String, String>,
}

impl IsingModel {
    /// Returns the number of spin variables.
    #[must_use]
    pub const fn variable_count(&self) -> u64 {
        self.variable_count
    }

    /// Returns the constant offset.
    #[must_use]
    pub const fn offset(&self) -> f64 {
        self.offset
    }

    /// Returns the sparse linear fields.
    #[must_use]
    pub fn linear_terms(&self) -> &BTreeMap<QuboVariableId, f64> {
        &self.linear
    }

    /// Returns the sparse quadratic couplings.
    #[must_use]
    pub fn quadratic_terms(&self) -> &BTreeMap<QuboInteraction, f64> {
        &self.quadratic
    }

    /// Evaluates a sparse spin assignment.
    ///
    /// Variables absent from the assignment are interpreted as `+1`.
    pub fn evaluate_sparse<I>(
        &self,
        assignment: I,
    ) -> Result<f64, IsingEvaluationError>
    where
        I: IntoIterator<Item = (QuboVariableId, i8)>,
    {
        let mut values = BTreeMap::new();

        for (variable, value) in assignment {
            if variable.value() >= self.variable_count {
                return Err(IsingEvaluationError::VariableOutOfRange {
                    variable,
                    variable_count: self.variable_count,
                });
            }

            if value != -1 && value != 1 {
                return Err(IsingEvaluationError::InvalidSpin {
                    variable,
                    value,
                });
            }

            if values.insert(variable, value).is_some() {
                return Err(IsingEvaluationError::DuplicateAssignment { variable });
            }
        }

        let mut energy = self.offset;

        for (variable, coefficient) in &self.linear {
            let value = values.get(variable).copied().unwrap_or(1);
            energy += *coefficient * f64::from(value);
        }

        for (interaction, coefficient) in &self.quadratic {
            let first = values
                .get(&interaction.first)
                .copied()
                .unwrap_or(1);

            let second = values
                .get(&interaction.second)
                .copied()
                .unwrap_or(1);

            energy += *coefficient * f64::from(first * second);
        }

        if !energy.is_finite() {
            return Err(IsingEvaluationError::NonFiniteResult);
        }

        Ok(energy)
    }

    /// Validates the complete Ising representation.
    pub fn validate(&self) -> Result<(), QuboValidationErrors> {
        let mut errors = QuboValidationErrors::new();

        if !self.offset.is_finite() {
            errors.push(QuboError::NonFiniteCoefficient {
                field: "Ising offset",
            });
        }

        for (variable, coefficient) in &self.linear {
            if variable.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: *variable,
                    variable_count: self.variable_count,
                });
            }

            if !coefficient.is_finite() {
                errors.push(QuboError::NonFiniteCoefficient {
                    field: "Ising linear coefficient",
                });
            }
        }

        for (interaction, coefficient) in &self.quadratic {
            if interaction.first >= interaction.second {
                errors.push(QuboError::NonCanonicalInteraction {
                    first: interaction.first,
                    second: interaction.second,
                });
            }

            if interaction.first.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: interaction.first,
                    variable_count: self.variable_count,
                });
            }

            if interaction.second.value() >= self.variable_count {
                errors.push(QuboError::VariableOutOfRange {
                    variable: interaction.second,
                    variable_count: self.variable_count,
                });
            }

            if !coefficient.is_finite() {
                errors.push(QuboError::NonFiniteCoefficient {
                    field: "Ising quadratic coefficient",
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
// Metadata
// =============================================================================

impl QuboProblem {
    /// Inserts deterministic metadata.
    ///
    /// Metadata is semantic-neutral and must never contain secrets.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Removes metadata and returns its previous value.
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.metadata.remove(key)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error produced while constructing or validating a QUBO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuboError {
    /// A coefficient or offset was not finite.
    NonFiniteCoefficient {
        /// Semantic field containing the invalid number.
        field: &'static str,
    },

    /// A variable ID is outside the declared variable domain.
    VariableOutOfRange {
        /// Invalid variable.
        variable: QuboVariableId,

        /// Declared variable count.
        variable_count: u64,
    },

    /// A variable was paired with itself.
    SelfInteraction {
        /// Invalid variable.
        variable: QuboVariableId,
    },

    /// An interaction is not in canonical order.
    NonCanonicalInteraction {
        /// First variable.
        first: QuboVariableId,

        /// Second variable.
        second: QuboVariableId,
    },

    /// A canonical representation contains an explicit zero term.
    ExplicitZeroCoefficient {
        /// Variable containing the zero coefficient.
        variable: QuboVariableId,
    },

    /// A canonical representation contains an explicit zero interaction.
    ExplicitZeroInteraction {
        /// First variable.
        first: QuboVariableId,

        /// Second variable.
        second: QuboVariableId,
    },

    /// Arithmetic overflow caused a non-finite accumulated result.
    ArithmeticOverflow {
        /// Field being accumulated.
        field: &'static str,
    },
}

impl fmt::Display for QuboError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteCoefficient { field } => {
                write!(formatter, "{field} must be finite")
            }

            Self::VariableOutOfRange {
                variable,
                variable_count,
            } => write!(
                formatter,
                "QUBO variable {variable} is outside variable count {variable_count}"
            ),

            Self::SelfInteraction { variable } => {
                write!(formatter, "QUBO variable {variable} cannot interact with itself")
            }

            Self::NonCanonicalInteraction { first, second } => write!(
                formatter,
                "QUBO interaction must satisfy first < second, got {first} and {second}"
            ),

            Self::ExplicitZeroCoefficient { variable } => write!(
                formatter,
                "canonical QUBO contains explicit zero coefficient for {variable}"
            ),

            Self::ExplicitZeroInteraction { first, second } => write!(
                formatter,
                "canonical QUBO contains explicit zero interaction {first}, {second}"
            ),

            Self::ArithmeticOverflow { field } => {
                write!(formatter, "non-finite arithmetic result while accumulating {field}")
            }
        }
    }
}

impl Error for QuboError {}

/// Collection of QUBO validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuboValidationErrors {
    errors: Vec<QuboError>,
}

impl QuboValidationErrors {
    /// Creates an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Creates a collection containing one error.
    #[must_use]
    pub fn single(error: QuboError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    /// Adds an error.
    pub fn push(&mut self, error: QuboError) {
        self.errors.push(error);
    }

    /// Returns whether there are no errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Returns all errors.
    #[must_use]
    pub fn errors(&self) -> &[QuboError] {
        &self.errors
    }
}

impl Default for QuboValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QuboValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "QUBO validation failed with {} error(s):",
            self.errors.len()
        )?;

        for error in &self.errors {
            writeln!(formatter, "  - {error}")?;
        }

        Ok(())
    }
}

impl Error for QuboValidationErrors {}

/// Error produced while evaluating a QUBO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuboEvaluationError {
    /// The QUBO itself is invalid.
    InvalidProblem(QuboError),

    /// The same variable was assigned more than once.
    DuplicateAssignment {
        /// Duplicate variable.
        variable: QuboVariableId,
    },

    /// Dense assignment contains more entries than the QUBO.
    AssignmentTooLarge {
        /// Required variable count.
        expected: u64,
    },

    /// Dense assignment contains too few entries.
    AssignmentSizeMismatch {
        /// Expected count.
        expected: u64,

        /// Actual count.
        actual: u64,
    },

    /// The dense iterator exceeded the semantic ID range.
    AssignmentIndexOverflow,

    /// Objective evaluation produced a non-finite value.
    NonFiniteResult,
}

impl fmt::Display for QuboEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProblem(error) => write!(formatter, "invalid QUBO: {error}"),

            Self::DuplicateAssignment { variable } => {
                write!(formatter, "duplicate assignment for {variable}")
            }

            Self::AssignmentTooLarge { expected } => {
                write!(formatter, "assignment contains more than {expected} variables")
            }

            Self::AssignmentSizeMismatch { expected, actual } => write!(
                formatter,
                "assignment contains {actual} values but {expected} were required"
            ),

            Self::AssignmentIndexOverflow => {
                formatter.write_str("dense assignment variable index overflow")
            }

            Self::NonFiniteResult => {
                formatter.write_str("QUBO objective evaluation produced a non-finite result")
            }
        }
    }
}

impl Error for QuboEvaluationError {}

/// Error produced while converting QUBO to Ising.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuboConversionError {
    /// Input QUBO is invalid.
    InvalidQubo(QuboValidationErrors),

    /// Conversion produced an invalid coefficient.
    InvalidCoefficient(QuboError),
}

impl fmt::Display for QuboConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubo(errors) => write!(formatter, "{errors}"),
            Self::InvalidCoefficient(error) => {
                write!(formatter, "invalid Ising conversion coefficient: {error}")
            }
        }
    }
}

impl Error for QuboConversionError {}

/// Error produced while evaluating an Ising model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsingEvaluationError {
    /// Variable is outside the declared domain.
    VariableOutOfRange {
        /// Invalid variable.
        variable: QuboVariableId,

        /// Variable count.
        variable_count: u64,
    },

    /// Spin value is neither -1 nor +1.
    InvalidSpin {
        /// Variable.
        variable: QuboVariableId,

        /// Invalid spin.
        value: i8,
    },

    /// Variable assigned more than once.
    DuplicateAssignment {
        /// Duplicate variable.
        variable: QuboVariableId,
    },

    /// Evaluation produced a non-finite value.
    NonFiniteResult,
}

impl fmt::Display for IsingEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableOutOfRange {
                variable,
                variable_count,
            } => write!(
                formatter,
                "Ising variable {variable} is outside variable count {variable_count}"
            ),

            Self::InvalidSpin { variable, value } => {
                write!(formatter, "invalid spin {value} for variable {variable}")
            }

            Self::DuplicateAssignment { variable } => {
                write!(formatter, "duplicate assignment for {variable}")
            }

            Self::NonFiniteResult => {
                formatter.write_str("Ising evaluation produced a non-finite result")
            }
        }
    }
}

impl Error for IsingEvaluationError {}

// =============================================================================
// Numeric helpers
// =============================================================================

fn validate_finite(
    value: f64,
    field: &'static str,
) -> Result<(), QuboError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(QuboError::NonFiniteCoefficient { field })
    }
}

fn add_finite(
    map: &mut BTreeMap<QuboVariableId, f64>,
    key: QuboVariableId,
    value: f64,
    field: &'static str,
) -> Result<(), QuboConversionError> {
    let entry = map.entry(key).or_insert(0.0);
    *entry += value;

    if !entry.is_finite() {
        return Err(QuboConversionError::InvalidCoefficient(
            QuboError::ArithmeticOverflow { field },
        ));
    }

    if *entry == 0.0 {
        map.remove(&key);
    }

    Ok(())
}

fn add_finite_interaction(
    map: &mut BTreeMap<QuboInteraction, f64>,
    key: QuboInteraction,
    value: f64,
    field: &'static str,
) -> Result<(), QuboConversionError> {
    let entry = map.entry(key).or_insert(0.0);
    *entry += value;

    if !entry.is_finite() {
        return Err(QuboConversionError::InvalidCoefficient(
            QuboError::ArithmeticOverflow { field },
        ));
    }

    if *entry == 0.0 {
        map.remove(&key);
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
    fn variable_ids_are_stable_and_ordered() {
        let a = QuboVariableId::new(1);
        let b = QuboVariableId::new(2);

        assert!(a < b);
        assert_eq!(a.value(), 1);
        assert_eq!(b.value(), 2);
    }

    #[test]
    fn interactions_are_canonicalized() {
        let forward =
            QuboInteraction::new(QuboVariableId::new(2), QuboVariableId::new(5))
                .expect("valid interaction");

        let reverse =
            QuboInteraction::new(QuboVariableId::new(5), QuboVariableId::new(2))
                .expect("valid interaction");

        assert_eq!(forward, reverse);
        assert_eq!(
            forward.variables(),
            (QuboVariableId::new(2), QuboVariableId::new(5))
        );
    }

    #[test]
    fn self_interaction_is_rejected() {
        let result =
            QuboInteraction::new(QuboVariableId::new(4), QuboVariableId::new(4));

        assert!(matches!(
            result,
            Err(QuboError::SelfInteraction { .. })
        ));
    }

    #[test]
    fn coefficients_are_accumulated() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.add_linear(QuboVariableId::new(0), 2.0)
            .expect("valid coefficient");

        qubo.add_linear(QuboVariableId::new(0), 3.0)
            .expect("valid coefficient");

        assert_eq!(
            qubo.linear_coefficient(QuboVariableId::new(0)),
            5.0
        );
    }

    #[test]
    fn cancelling_coefficients_are_removed() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.add_linear(QuboVariableId::new(0), 2.0)
            .expect("valid coefficient");

        qubo.add_linear(QuboVariableId::new(0), -2.0)
            .expect("valid coefficient");

        assert!(qubo.linear_terms().is_empty());
    }

    #[test]
    fn sparse_evaluation_is_correct() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.set_offset(1.0).expect("valid offset");

        qubo.add_linear(QuboVariableId::new(0), 2.0)
            .expect("valid coefficient");

        qubo.add_linear(QuboVariableId::new(1), 3.0)
            .expect("valid coefficient");

        qubo.add_quadratic(
            QuboVariableId::new(0),
            QuboVariableId::new(1),
            4.0,
        )
        .expect("valid interaction");

        let energy = qubo
            .evaluate_sparse([
                (QuboVariableId::new(0), true),
                (QuboVariableId::new(1), true),
            ])
            .expect("valid assignment");

        assert_eq!(energy, 10.0);
    }

    #[test]
    fn dense_evaluation_is_correct() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.set_offset(1.0).expect("valid offset");

        qubo.add_linear(QuboVariableId::new(0), 2.0)
            .expect("valid coefficient");

        qubo.add_quadratic(
            QuboVariableId::new(0),
            QuboVariableId::new(2),
            5.0,
        )
        .expect("valid interaction");

        let energy = qubo
            .evaluate_dense([true, false, true])
            .expect("valid assignment");

        assert_eq!(energy, 8.0);
    }

    #[test]
    fn non_finite_coefficients_are_rejected() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        assert!(qubo
            .add_linear(QuboVariableId::new(0), f64::NAN)
            .is_err());

        assert!(qubo
            .add_linear(QuboVariableId::new(0), f64::INFINITY)
            .is_err());

        assert!(qubo
            .add_linear(QuboVariableId::new(0), f64::NEG_INFINITY)
            .is_err());
    }

    #[test]
    fn out_of_range_variables_are_rejected() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        assert!(matches!(
            qubo.add_linear(QuboVariableId::new(2), 1.0),
            Err(QuboError::VariableOutOfRange { .. })
        ));
    }

    #[test]
    fn qubo_to_ising_preserves_energy() {
        let mut qubo = QuboProblem::new(3).expect("valid QUBO");

        qubo.set_offset(1.5).expect("valid offset");

        qubo.add_linear(QuboVariableId::new(0), 2.0)
            .expect("valid coefficient");

        qubo.add_linear(QuboVariableId::new(1), -3.0)
            .expect("valid coefficient");

        qubo.add_quadratic(
            QuboVariableId::new(0),
            QuboVariableId::new(1),
            4.0,
        )
        .expect("valid interaction");

        let ising = qubo.to_ising().expect("valid conversion");

        let qubo_energy = qubo
            .evaluate_sparse([
                (QuboVariableId::new(0), true),
                (QuboVariableId::new(1), false),
            ])
            .expect("valid QUBO assignment");

        let ising_energy = ising
            .evaluate_sparse([
                (QuboVariableId::new(0), 1),
                (QuboVariableId::new(1), -1),
            ])
            .expect("valid Ising assignment");

        assert!(
            (qubo_energy - ising_energy).abs() < DEFAULT_ENERGY_TOLERANCE
        );
    }

    #[test]
    fn reverse_quadratic_input_is_accumulated() {
        let mut qubo = QuboProblem::new(4).expect("valid QUBO");

        qubo.add_quadratic(
            QuboVariableId::new(3),
            QuboVariableId::new(1),
            2.0,
        )
        .expect("valid interaction");

        qubo.add_quadratic(
            QuboVariableId::new(1),
            QuboVariableId::new(3),
            3.0,
        )
        .expect("valid interaction");

        assert_eq!(
            qubo.quadratic_coefficient(
                QuboVariableId::new(1),
                QuboVariableId::new(3)
            )
            .expect("valid interaction"),
            5.0
        );

        assert_eq!(qubo.quadratic_terms().len(), 1);
    }

    #[test]
    fn zero_terms_are_not_stored() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        qubo.add_linear(QuboVariableId::new(0), 0.0)
            .expect("zero is valid");

        qubo.add_quadratic(
            QuboVariableId::new(0),
            QuboVariableId::new(1),
            0.0,
        )
        .expect("zero is valid");

        assert!(qubo.is_empty());
    }

    #[test]
    fn validation_accepts_canonical_problem() {
        let mut qubo = QuboProblem::new(2).expect("valid QUBO");

        qubo.add_linear(QuboVariableId::new(0), 1.0)
            .expect("valid coefficient");

        qubo.add_quadratic(
            QuboVariableId::new(0),
            QuboVariableId::new(1),
            2.0,
        )
        .expect("valid interaction");

        assert!(qubo.validate().is_ok());
    }

    #[test]
    fn sparse_assignment_defaults_missing_variables_to_zero() {
        let mut qubo = QuboProblem::new(100).expect("valid QUBO");

        qubo.add_linear(QuboVariableId::new(99), 7.0)
            .expect("valid coefficient");

        let energy = qubo
            .evaluate_sparse([(QuboVariableId::new(1), true)])
            .expect("valid sparse assignment");

        assert_eq!(energy, 0.0);
    }

    #[test]
    fn ising_invalid_spin_is_rejected() {
        let qubo = QuboProblem::new(2).expect("valid QUBO");
        let ising = qubo.to_ising().expect("valid conversion");

        assert!(matches!(
            ising.evaluate_sparse([(QuboVariableId::new(0), 0)]),
            Err(IsingEvaluationError::InvalidSpin { .. })
        ));
    }

    #[test]
    fn large_semantic_variable_ids_are_not_host_size_limited() {
        let variable_count = u64::MAX;

        let qubo = QuboProblem::new(variable_count)
            .expect("u64 semantic cardinality should be representable");

        assert_eq!(qubo.variable_count(), u64::MAX);
    }
}