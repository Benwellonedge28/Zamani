//! Zamani Quantum IR — Quantum Annealing Model
//!
//! Canonical, provider-independent semantic representation of quantum
//! annealing / adiabatic optimization workloads.
//!
//! # Architectural role
//!
//! This module represents the mathematical and semantic meaning of an
//! annealing computation:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend / algorithm construction
//!      |
//!      v
//! quantum::ir::model::annealing
//!      |
//!      +----------------------+
//!      |                      |
//!      v                      v
//!     QUBO                  Ising
//!      |                      |
//!      +----------+-----------+
//!                 |
//!                 v
//!       target-independent IR
//!                 |
//!                 v
//!       capability resolution
//!                 |
//!                 v
//!       embedding / mapping
//!                 |
//!                 v
//!       target-specific scheduling
//!                 |
//!                 v
//!       hardware/backend
//! ```
//!
//! The IR answers:
//!
//! > What annealing computation does the programmer mean?
//!
//! It does NOT answer:
//!
//! - which annealer executes it;
//! - which physical qubits are selected;
//! - which hardware graph is used;
//! - how minor embedding is performed;
//! - which chain strength is selected by a backend;
//! - which physical coupler implements an interaction;
//! - which DAC is used;
//! - which provider API is called;
//! - how a job is submitted;
//! - how a job is polled;
//! - how samples are statistically analysed;
//! - how a backend-specific schedule is generated.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal-program principle
//!
//! An annealing program should be expressible once and lowered to any target
//! whose capabilities and resources satisfy its requirements.
//!
//! Consequently this module contains:
//!
//! - no provider names;
//! - no hardware topology;
//! - no provider-specific variable limits;
//! - no provider-specific coefficient limits;
//! - no fixed maximum variable count;
//! - no fixed maximum interaction count;
//! - no fixed schedule-point count;
//! - no fixed sample count;
//! - no hardware qubit-count ceiling.
//!
//! A concrete compiler or backend may impose explicit operational limits, but
//! those limits must remain outside this semantic model.
//!
//! # Mathematical conventions
//!
//! ## QUBO
//!
//! The canonical QUBO objective is:
//!
//! ```text
//! E(x) = offset
//!      + Σ_i a_i x_i
//!      + Σ_{i<j} b_ij x_i x_j
//! ```
//!
//! with:
//!
//! ```text
//! x_i ∈ {0, 1}
//! ```
//!
//! ## Ising
//!
//! The canonical Ising objective is:
//!
//! ```text
//! E(s) = offset
//!      + Σ_i h_i s_i
//!      + Σ_{i<j} J_ij s_i s_j
//! ```
//!
//! with:
//!
//! ```text
//! s_i ∈ {-1, +1}
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
//! J_ij = Q_ij / 4
//!
//! h_i = Q_ii / 2 + Σ_j Q_ij / 4
//!
//! offset_ising = offset_qubo
//!              + Σ_i Q_ii / 2
//!              + Σ_{i<j} Q_ij / 4
//! ```
//!
//! ## Ising → QUBO
//!
//! Using:
//!
//! ```text
//! s_i = 2x_i - 1
//! ```
//!
//! gives:
//!
//! ```text
//! Q_ij = 4 J_ij
//!
//! Q_ii = 2 h_i - 4 Σ_j J_ij
//!
//! offset_qubo = offset_ising
//!              - Σ_i h_i
//!              + Σ_{i<j} J_ij
//! ```
//!
//! The transformations are exact at the mathematical level, subject only to
//! finite floating-point representation when `f64` is used.
//!
//! # Sparse representation
//!
//! Annealing problems are represented sparsely with ordered maps.
//!
//! This is intentional.
//!
//! A problem with a small number of interactions among a very large logical
//! variable universe must not require a dense matrix.
//!
//! Therefore:
//!
//! ```text
//! variables = semantic identifiers
//! linear     = sparse map
//! quadratic  = sparse map
//! ```
//!
//! rather than:
//!
//! ```text
//! Vec<Vec<f64>>
//! ```
//!
//! This permits the representation to scale with the actual semantic data.
//!
//! # Logical qubits
//!
//! Annealing variables are mathematical variables first.
//!
//! They are not automatically qubits.
//!
//! When an annealing workload is mapped onto a quantum device, a variable may
//! be associated with a logical qubit using:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! The mapping is represented explicitly by `AnnealingVariableBinding`.
//!
//! This avoids incorrectly defining every annealing variable as a physical
//! qubit.
//!
//! # Timing and schedules
//!
//! A semantic annealing schedule is represented using normalized progress
//! coordinates and symbolic schedule segments.
//!
//! The IR does not assume a particular physical unit such as microseconds.
//! Target-specific duration conversion belongs downstream.
//!
//! A target may later interpret normalized anneal progress as:
//!
//! ```text
//! s ∈ [0, 1]
//! ```
//!
//! while choosing its own physical duration and hardware-dependent control
//! functions.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Dependencies
//!
//! This module intentionally depends only on:
//!
//! - Rust standard library;
//! - `serde`, already used by the Zamani quantum subsystem;
//! - canonical `quantum::ir::qubit` identifiers.
//!
//! It must not depend on:
//!
//! - `quantum::hardware`;
//! - provider implementations;
//! - routing;
//! - scheduling;
//! - optimization algorithms;
//! - simulation;
//! - benchmarking;
//! - execution;
//! - frontend parsers.
//!
//! # Integration contract
//!
//! Downstream consumers may use this module from:
//!
//! - annealing algorithms;
//! - IR construction;
//! - target capability analysis;
//! - resource estimation;
//! - embedding/routing;
//! - hardware adapters;
//! - simulators;
//! - benchmarking;
//! - Danga;
//! - serialization;
//! - provenance.
//!
//! None of those modules are dependencies of this file.
//!
//! # Security
//!
//! No public constructor accepts unchecked NaN or infinite numerical values.
//!
//! Arithmetic that can overflow is checked.
//!
//! Collections use deterministic ordered representations.
//!
//! No implicit allocation based on untrusted numeric counts is performed.
//!
//! There are no architectural maximum constants in this file.
//!
//! # Serialization
//!
//! Serde derives are provided for deterministic persistence at the IR boundary.
//! Canonical byte encoding remains the responsibility of
//! `quantum::ir::serialization`.
//!
//! This module does not define a second serialization protocol.
//!
//! # Hashing
//!
//! This module does not implement canonical hashing.
//!
//! `quantum::ir::hash` remains the authoritative hashing boundary.
//!
//! # Important ownership rule
//!
//! This file owns the semantic annealing model.
//!
//! It does NOT own execution results, statistical analysis, backend jobs,
//! provider calibration, hardware topology, or embedding.
//!
//! -----------------------------------------------------------------------------
//! No unsafe code.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::super::qubit::QubitId;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier.
pub const ANNEALING_SCHEMA_ID: &str = "zamani.quantum.ir.model.annealing";

/// Current semantic schema version.
///
/// This version is for this module's semantic contract. The canonical global
/// IR version remains owned by `quantum::ir::identity`.
pub const ANNEALING_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Variable identity
// =============================================================================

/// Stable semantic annealing-variable identifier.
///
/// This is deliberately independent of `QubitId`.
///
/// A mathematical optimization variable is not inherently a qubit. A later
/// mapping stage may bind it to a logical qubit.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct VariableId(u64);

impl VariableId {
    /// Creates a semantic variable identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identifier.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for VariableId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<VariableId> for u64 {
    fn from(value: VariableId) -> Self {
        value.value()
    }
}

impl fmt::Display for VariableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "v{}", self.0)
    }
}

// =============================================================================
// Variable domain
// =============================================================================

/// Domain of an annealing variable.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum VariableDomain {
    /// QUBO variable: `0` or `1`.
    Binary,

    /// Ising variable: `-1` or `+1`.
    Spin,
}

impl VariableDomain {
    /// Returns a stable machine-readable representation.
    #[must_use]
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
// Quadratic pair
// =============================================================================

/// Canonical unordered pair of distinct variables.
///
/// The invariant is:
///
/// ```text
/// first < second
/// ```
///
/// This prevents duplicate representations of the same interaction.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct VariablePair {
    first: VariableId,
    second: VariableId,
}

impl VariablePair {
    /// Creates a canonical pair.
    ///
    /// Returns an error when both identifiers are equal.
    pub const fn new(
        first: VariableId,
        second: VariableId,
    ) -> Result<Self, AnnealingValidationError> {
        if first == second {
            return Err(AnnealingValidationError::SelfInteraction {
                variable: first,
            });
        }

        if first < second {
            Ok(Self { first, second })
        } else {
            Ok(Self {
                first: second,
                second: first,
            })
        }
    }

    /// Returns the first identifier.
    #[must_use]
    pub const fn first(self) -> VariableId {
        self.first
    }

    /// Returns the second identifier.
    #[must_use]
    pub const fn second(self) -> VariableId {
        self.second
    }
}

impl fmt::Display for VariablePair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.first, self.second)
    }
}

// =============================================================================
// Sparse QUBO
// =============================================================================

/// Canonical sparse QUBO problem.
///
/// The mathematical objective is:
///
/// ```text
/// E(x) = offset
///      + Σ_i linear[i] x_i
///      + Σ_{i<j} quadratic[(i,j)] x_i x_j
/// ```
///
/// Variables are identified explicitly rather than inferred from dense
/// storage.
///
/// This means a large sparse problem does not require allocation proportional
/// to the square of the number of variables.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuboProblem {
    /// Variables explicitly declared by the model.
    ///
    /// A variable may be declared even when its coefficient is currently zero.
    variables: BTreeSet<VariableId>,

    /// Constant energy offset.
    offset: f64,

    /// Sparse linear coefficients.
    linear: BTreeMap<VariableId, f64>,

    /// Sparse quadratic coefficients.
    quadratic: BTreeMap<VariablePair, f64>,

    /// Optional semantic problem name.
    name: Option<String>,

    /// Non-secret deterministic metadata.
    metadata: BTreeMap<String, String>,
}

impl QuboProblem {
    /// Creates an empty QUBO.
    #[must_use]
    pub fn new() -> Self {
        Self {
            variables: BTreeSet::new(),
            offset: 0.0,
            linear: BTreeMap::new(),
            quadratic: BTreeMap::new(),
            name: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a QUBO with an explicit variable set.
    pub fn with_variables<I>(variables: I) -> Result<Self, AnnealingValidationErrors>
    where
        I: IntoIterator<Item = VariableId>,
    {
        let mut result = Self::new();

        let mut errors = AnnealingValidationErrors::new();

        for variable in variables {
            if result.variables.insert(variable) {
                continue;
            }

            errors.push(AnnealingValidationError::DuplicateVariable { variable });
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }

    /// Returns the number of explicitly declared variables.
    #[must_use]
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Returns whether the model has no variables.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Returns all variables in deterministic order.
    #[must_use]
    pub fn variables(&self) -> &BTreeSet<VariableId> {
        &self.variables
    }

    /// Declares a variable.
    pub fn add_variable(
        &mut self,
        variable: VariableId,
    ) -> Result<(), AnnealingValidationError> {
        if !self.variables.insert(variable) {
            return Err(AnnealingValidationError::DuplicateVariable { variable });
        }

        Ok(())
    }

    /// Declares multiple variables.
    pub fn add_variables<I>(
        &mut self,
        variables: I,
    ) -> Result<(), AnnealingValidationErrors>
    where
        I: IntoIterator<Item = VariableId>,
    {
        let mut errors = AnnealingValidationErrors::new();

        for variable in variables {
            if let Err(error) = self.add_variable(variable) {
                errors.push(error);
            }
        }

        errors.into_result(())
    }

    /// Returns the constant offset.
    #[must_use]
    pub const fn offset(&self) -> f64 {
        self.offset
    }

    /// Sets the constant offset.
    pub fn set_offset(&mut self, offset: f64) -> Result<(), AnnealingValidationError> {
        validate_finite(offset, "QUBO offset")?;
        self.offset = offset;
        Ok(())
    }

    /// Adds to a linear coefficient.
    ///
    /// Zero coefficients are retained only if the variable itself is
    /// explicitly declared.
    pub fn add_linear(
        &mut self,
        variable: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "QUBO linear coefficient")?;

        self.variables.insert(variable);

        let entry = self.linear.entry(variable).or_insert(0.0);

        *entry = checked_add_f64(*entry, coefficient, "QUBO linear coefficient")?;

        Ok(())
    }

    /// Sets a linear coefficient.
    pub fn set_linear(
        &mut self,
        variable: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "QUBO linear coefficient")?;

        self.variables.insert(variable);
        self.linear.insert(variable, coefficient);

        Ok(())
    }

    /// Returns a linear coefficient.
    #[must_use]
    pub fn linear_coefficient(&self, variable: VariableId) -> f64 {
        self.linear.get(&variable).copied().unwrap_or(0.0)
    }

    /// Returns all sparse linear coefficients.
    #[must_use]
    pub fn linear_terms(&self) -> &BTreeMap<VariableId, f64> {
        &self.linear
    }

    /// Adds to a quadratic coefficient.
    ///
    /// The pair is canonicalized automatically.
    pub fn add_quadratic(
        &mut self,
        first: VariableId,
        second: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "QUBO quadratic coefficient")?;

        let pair = VariablePair::new(first, second)?;

        self.variables.insert(pair.first());
        self.variables.insert(pair.second());

        let entry = self.quadratic.entry(pair).or_insert(0.0);

        *entry = checked_add_f64(*entry, coefficient, "QUBO quadratic coefficient")?;

        Ok(())
    }

    /// Sets a quadratic coefficient.
    pub fn set_quadratic(
        &mut self,
        first: VariableId,
        second: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "QUBO quadratic coefficient")?;

        let pair = VariablePair::new(first, second)?;

        self.variables.insert(pair.first());
        self.variables.insert(pair.second());

        self.quadratic.insert(pair, coefficient);

        Ok(())
    }

    /// Returns a quadratic coefficient.
    #[must_use]
    pub fn quadratic_coefficient(
        &self,
        first: VariableId,
        second: VariableId,
    ) -> Option<f64> {
        let pair = VariablePair::new(first, second).ok()?;
        self.quadratic.get(&pair).copied()
    }

    /// Returns all sparse quadratic coefficients.
    #[must_use]
    pub fn quadratic_terms(&self) -> &BTreeMap<VariablePair, f64> {
        &self.quadratic
    }

    /// Sets a semantic name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        let name = name.into();
        validate_text(&name, "QUBO name")?;
        self.name = Some(name);
        Ok(())
    }

    /// Returns the optional semantic name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Adds deterministic metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Evaluates the QUBO objective for a complete binary assignment.
    pub fn energy(
        &self,
        assignment: &BTreeMap<VariableId, bool>,
    ) -> Result<f64, AnnealingValidationError> {
        self.validate()?;

        for variable in &self.variables {
            if !assignment.contains_key(variable) {
                return Err(AnnealingValidationError::MissingAssignment {
                    variable: *variable,
                });
            }
        }

        for variable in assignment.keys() {
            if !self.variables.contains(variable) {
                return Err(AnnealingValidationError::UnknownAssignmentVariable {
                    variable: *variable,
                });
            }
        }

        let mut energy = self.offset;

        for (variable, coefficient) in &self.linear {
            if assignment[variable] {
                energy = checked_add_f64(energy, *coefficient, "QUBO energy")?;
            }
        }

        for (pair, coefficient) in &self.quadratic {
            if assignment[&pair.first()] && assignment[&pair.second()] {
                energy = checked_add_f64(energy, *coefficient, "QUBO energy")?;
            }
        }

        Ok(energy)
    }

    /// Converts this QUBO to the canonical Ising representation.
    pub fn to_ising(&self) -> Result<IsingModel, AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if let Err(error) = self.validate() {
            errors.extend(error);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut ising = IsingModel::new();

        ising.set_offset(
            checked_add_f64(
                self.offset,
                0.0,
                "QUBO-to-Ising offset",
            )
            .map_err(AnnealingValidationErrors::single)?,
        )
        .map_err(AnnealingValidationErrors::single)?;

        for variable in &self.variables {
            ising.add_variable(*variable);

            let qii = self.linear_coefficient(*variable);

            let mut h = qii / 2.0;
            validate_finite(h, "QUBO-to-Ising linear coefficient")
                .map_err(AnnealingValidationErrors::single)?;

            for (pair, coefficient) in &self.quadratic {
                if pair.first() == *variable || pair.second() == *variable {
                    h = checked_add_f64(
                        h,
                        *coefficient / 4.0,
                        "QUBO-to-Ising linear coefficient",
                    )
                    .map_err(AnnealingValidationErrors::single)?;
                }
            }

            if h != 0.0 {
                ising
                    .set_linear(*variable, h)
                    .map_err(AnnealingValidationErrors::single)?;
            }
        }

        let mut offset = self.offset;

        for (variable, coefficient) in &self.linear {
            let contribution = *coefficient / 2.0;

            offset = checked_add_f64(
                offset,
                contribution,
                "QUBO-to-Ising offset",
            )
            .map_err(AnnealingValidationErrors::single)?;

            let _ = variable;
        }

        for (pair, coefficient) in &self.quadratic {
            let contribution = *coefficient / 4.0;

            offset = checked_add_f64(
                offset,
                contribution,
                "QUBO-to-Ising offset",
            )
            .map_err(AnnealingValidationErrors::single)?;

            ising
                .set_quadratic(
                    pair.first(),
                    pair.second(),
                    contribution,
                )
                .map_err(AnnealingValidationErrors::single)?;
        }

        ising
            .set_offset(offset)
            .map_err(AnnealingValidationErrors::single)?;

        Ok(ising)
    }

    /// Validates the complete QUBO.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if !self.offset.is_finite() {
            errors.push(AnnealingValidationError::NonFiniteValue {
                field: "QUBO offset",
                value: self.offset,
            });
        }

        for variable in self.linear.keys() {
            if !self.variables.contains(variable) {
                errors.push(AnnealingValidationError::MissingDeclaredVariable {
                    variable: *variable,
                });
            }
        }

        for (pair, coefficient) in &self.quadratic {
            if !self.variables.contains(&pair.first()) {
                errors.push(
                    AnnealingValidationError::MissingDeclaredVariable {
                        variable: pair.first(),
                    },
                );
            }

            if !self.variables.contains(&pair.second()) {
                errors.push(
                    AnnealingValidationError::MissingDeclaredVariable {
                        variable: pair.second(),
                    },
                );
            }

            if pair.first() >= pair.second() {
                errors.push(AnnealingValidationError::NonCanonicalPair {
                    pair: *pair,
                });
            }

            if !coefficient.is_finite() {
                errors.push(AnnealingValidationError::NonFiniteValue {
                    field: "QUBO quadratic coefficient",
                    value: *coefficient,
                });
            }
        }

        for coefficient in self.linear.values() {
            if !coefficient.is_finite() {
                errors.push(AnnealingValidationError::NonFiniteValue {
                    field: "QUBO linear coefficient",
                    value: *coefficient,
                });
            }
        }

        validate_metadata_collection(&self.metadata, &mut errors);

        errors.into_result(())
    }
}

impl Default for QuboProblem {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Ising model
// =============================================================================

/// Canonical sparse Ising model.
///
/// ```text
/// E(s) = offset
///      + Σ_i h_i s_i
///      + Σ_{i<j} J_ij s_i s_j
/// ```
///
/// where `s_i ∈ {-1,+1}`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsingModel {
    variables: BTreeSet<VariableId>,
    offset: f64,
    linear: BTreeMap<VariableId, f64>,
    quadratic: BTreeMap<VariablePair, f64>,
    name: Option<String>,
    metadata: BTreeMap<String, String>,
}

impl IsingModel {
    /// Creates an empty Ising model.
    #[must_use]
    pub fn new() -> Self {
        Self {
            variables: BTreeSet::new(),
            offset: 0.0,
            linear: BTreeMap::new(),
            quadratic: BTreeMap::new(),
            name: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the number of variables.
    #[must_use]
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Returns the variables.
    #[must_use]
    pub fn variables(&self) -> &BTreeSet<VariableId> {
        &self.variables
    }

    /// Declares a variable.
    pub fn add_variable(&mut self, variable: VariableId) {
        self.variables.insert(variable);
    }

    /// Returns the constant offset.
    #[must_use]
    pub const fn offset(&self) -> f64 {
        self.offset
    }

    /// Sets the constant offset.
    pub fn set_offset(&mut self, offset: f64) -> Result<(), AnnealingValidationError> {
        validate_finite(offset, "Ising offset")?;
        self.offset = offset;
        Ok(())
    }

    /// Adds to a linear bias.
    pub fn add_linear(
        &mut self,
        variable: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "Ising linear coefficient")?;

        self.variables.insert(variable);

        let entry = self.linear.entry(variable).or_insert(0.0);

        *entry = checked_add_f64(*entry, coefficient, "Ising linear coefficient")?;

        Ok(())
    }

    /// Sets a linear bias.
    pub fn set_linear(
        &mut self,
        variable: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "Ising linear coefficient")?;

        self.variables.insert(variable);
        self.linear.insert(variable, coefficient);

        Ok(())
    }

    /// Returns a linear bias.
    #[must_use]
    pub fn linear_coefficient(&self, variable: VariableId) -> f64 {
        self.linear.get(&variable).copied().unwrap_or(0.0)
    }

    /// Returns all linear coefficients.
    #[must_use]
    pub fn linear_terms(&self) -> &BTreeMap<VariableId, f64> {
        &self.linear
    }

    /// Adds to a quadratic coupling.
    pub fn add_quadratic(
        &mut self,
        first: VariableId,
        second: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "Ising quadratic coefficient")?;

        let pair = VariablePair::new(first, second)?;

        self.variables.insert(pair.first());
        self.variables.insert(pair.second());

        let entry = self.quadratic.entry(pair).or_insert(0.0);

        *entry = checked_add_f64(*entry, coefficient, "Ising quadratic coefficient")?;

        Ok(())
    }

    /// Sets a quadratic coupling.
    pub fn set_quadratic(
        &mut self,
        first: VariableId,
        second: VariableId,
        coefficient: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(coefficient, "Ising quadratic coefficient")?;

        let pair = VariablePair::new(first, second)?;

        self.variables.insert(pair.first());
        self.variables.insert(pair.second());

        self.quadratic.insert(pair, coefficient);

        Ok(())
    }

    /// Returns a quadratic coupling.
    #[must_use]
    pub fn quadratic_coefficient(
        &self,
        first: VariableId,
        second: VariableId,
    ) -> Option<f64> {
        let pair = VariablePair::new(first, second).ok()?;
        self.quadratic.get(&pair).copied()
    }

    /// Returns all quadratic couplings.
    #[must_use]
    pub fn quadratic_terms(&self) -> &BTreeMap<VariablePair, f64> {
        &self.quadratic
    }

    /// Evaluates the Ising objective.
    pub fn energy(
        &self,
        assignment: &BTreeMap<VariableId, i8>,
    ) -> Result<f64, AnnealingValidationError> {
        self.validate()?;

        for variable in &self.variables {
            if !assignment.contains_key(variable) {
                return Err(AnnealingValidationError::MissingAssignment {
                    variable: *variable,
                });
            }
        }

        for (variable, value) in assignment {
            if !self.variables.contains(variable) {
                return Err(AnnealingValidationError::UnknownAssignmentVariable {
                    variable: *variable,
                });
            }

            if *value != -1 && *value != 1 {
                return Err(AnnealingValidationError::InvalidSpinValue {
                    variable: *variable,
                    value: *value,
                });
            }
        }

        let mut energy = self.offset;

        for (variable, coefficient) in &self.linear {
            let spin = f64::from(assignment[variable]);

            energy = checked_add_f64(
                energy,
                coefficient * spin,
                "Ising energy",
            )?;
        }

        for (pair, coefficient) in &self.quadratic {
            let first = f64::from(assignment[&pair.first()]);
            let second = f64::from(assignment[&pair.second()]);

            energy = checked_add_f64(
                energy,
                coefficient * first * second,
                "Ising energy",
            )?;
        }

        Ok(energy)
    }

    /// Converts this Ising model to QUBO.
    pub fn to_qubo(&self) -> Result<QuboProblem, AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if let Err(error) = self.validate() {
            errors.extend(error);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut qubo = QuboProblem::new();

        let mut offset = self.offset;

        for variable in &self.variables {
            qubo.add_variable(*variable);

            let h = self.linear_coefficient(*variable);

            let mut qii = checked_mul_f64(
                2.0,
                h,
                "Ising-to-QUBO diagonal coefficient",
            )
            .map_err(AnnealingValidationErrors::single)?;

            for pair in self.quadratic.keys() {
                if pair.first() == *variable || pair.second() == *variable {
                    let j = self.quadratic[pair];

                    qii = checked_sub_f64(
                        qii,
                        4.0 * j,
                        "Ising-to-QUBO diagonal coefficient",
                    )
                    .map_err(AnnealingValidationErrors::single)?;
                }
            }

            if qii != 0.0 {
                qubo
                    .set_linear(*variable, qii)
                    .map_err(AnnealingValidationErrors::single)?;
            }

            offset = checked_sub_f64(
                offset,
                h,
                "Ising-to-QUBO offset",
            )
            .map_err(AnnealingValidationErrors::single)?;
        }

        for (pair, coupling) in &self.quadratic {
            let qij = checked_mul_f64(
                4.0,
                *coupling,
                "Ising-to-QUBO quadratic coefficient",
            )
            .map_err(AnnealingValidationErrors::single)?;

            qubo
                .set_quadratic(
                    pair.first(),
                    pair.second(),
                    qij,
                )
                .map_err(AnnealingValidationErrors::single)?;

            offset = checked_add_f64(
                offset,
                *coupling,
                "Ising-to-QUBO offset",
            )
            .map_err(AnnealingValidationErrors::single)?;
        }

        qubo
            .set_offset(offset)
            .map_err(AnnealingValidationErrors::single)?;

        Ok(qubo)
    }

    /// Validates the model.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if !self.offset.is_finite() {
            errors.push(AnnealingValidationError::NonFiniteValue {
                field: "Ising offset",
                value: self.offset,
            });
        }

        for variable in self.linear.keys() {
            if !self.variables.contains(variable) {
                errors.push(
                    AnnealingValidationError::MissingDeclaredVariable {
                        variable: *variable,
                    },
                );
            }
        }

        for (pair, coefficient) in &self.quadratic {
            if !self.variables.contains(&pair.first()) {
                errors.push(
                    AnnealingValidationError::MissingDeclaredVariable {
                        variable: pair.first(),
                    },
                );
            }

            if !self.variables.contains(&pair.second()) {
                errors.push(
                    AnnealingValidationError::MissingDeclaredVariable {
                        variable: pair.second(),
                    },
                );
            }

            if pair.first() >= pair.second() {
                errors.push(AnnealingValidationError::NonCanonicalPair {
                    pair: *pair,
                });
            }

            if !coefficient.is_finite() {
                errors.push(AnnealingValidationError::NonFiniteValue {
                    field: "Ising quadratic coefficient",
                    value: *coefficient,
                });
            }
        }

        for coefficient in self.linear.values() {
            if !coefficient.is_finite() {
                errors.push(AnnealingValidationError::NonFiniteValue {
                    field: "Ising linear coefficient",
                    value: *coefficient,
                });
            }
        }

        validate_metadata_collection(&self.metadata, &mut errors);

        errors.into_result(())
    }
}

impl Default for IsingModel {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Variable bindings
// =============================================================================

/// Explicit association between an annealing variable and a canonical Zamani
/// logical qubit.
///
/// This does not perform placement or routing.
///
/// It only records semantic intent/identity.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct AnnealingVariableBinding {
    variable: VariableId,
    qubit: QubitId,
}

impl AnnealingVariableBinding {
    /// Creates a variable-to-logical-qubit binding.
    #[must_use]
    pub const fn new(variable: VariableId, qubit: QubitId) -> Self {
        Self { variable, qubit }
    }

    /// Returns the variable.
    #[must_use]
    pub const fn variable(self) -> VariableId {
        self.variable
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }
}

// =============================================================================
// Annealing schedule
// =============================================================================

/// Normalized annealing schedule point.
///
/// `s` is a normalized progress coordinate.
///
/// ```text
/// 0 <= s <= 1
/// ```
///
/// `time` is a normalized/non-negative semantic coordinate. It is intentionally
/// not tied to a hardware unit.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct AnnealingSchedulePoint {
    /// Normalized progress.
    s: f64,

    /// Semantic schedule coordinate.
    time: f64,
}

impl AnnealingSchedulePoint {
    /// Creates a schedule point.
    pub fn new(
        s: f64,
        time: f64,
    ) -> Result<Self, AnnealingValidationError> {
        validate_finite(s, "annealing schedule s")?;
        validate_finite(time, "annealing schedule time")?;

        if !(0.0..=1.0).contains(&s) {
            return Err(AnnealingValidationError::ScheduleProgressOutOfRange {
                value: s,
            });
        }

        if time < 0.0 {
            return Err(AnnealingValidationError::NegativeScheduleTime {
                value: time,
            });
        }

        Ok(Self { s, time })
    }

    /// Returns normalized progress.
    #[must_use]
    pub const fn s(self) -> f64 {
        self.s
    }

    /// Returns semantic schedule time.
    #[must_use]
    pub const fn time(self) -> f64 {
        self.time
    }
}

/// A target-independent annealing schedule.
///
/// Points are strictly ordered by semantic time and non-decreasing in `s`.
///
/// No provider-specific schedule shape is assumed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSchedule {
    points: Vec<AnnealingSchedulePoint>,
}

impl AnnealingSchedule {
    /// Creates an empty schedule.
    #[must_use]
    pub const fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Creates a schedule from points.
    pub fn from_points(
        points: Vec<AnnealingSchedulePoint>,
    ) -> Result<Self, AnnealingValidationErrors> {
        let schedule = Self { points };

        schedule.validate()?;

        Ok(schedule)
    }

    /// Adds a point.
    pub fn push(
        &mut self,
        point: AnnealingSchedulePoint,
    ) -> Result<(), AnnealingValidationError> {
        if let Some(previous) = self.points.last() {
            if point.time() <= previous.time() {
                return Err(
                    AnnealingValidationError::NonMonotonicScheduleTime {
                        previous: previous.time(),
                        current: point.time(),
                    },
                );
            }

            if point.s() < previous.s() {
                return Err(
                    AnnealingValidationError::NonMonotonicScheduleProgress {
                        previous: previous.s(),
                        current: point.s(),
                    },
                );
            }
        }

        self.points.push(point);
        Ok(())
    }

    /// Returns schedule points.
    #[must_use]
    pub fn points(&self) -> &[AnnealingSchedulePoint] {
        &self.points
    }

    /// Returns the number of points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the schedule is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Validates the complete schedule.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        for window in self.points.windows(2) {
            let previous = window[0];
            let current = window[1];

            if current.time() <= previous.time() {
                errors.push(
                    AnnealingValidationError::NonMonotonicScheduleTime {
                        previous: previous.time(),
                        current: current.time(),
                    },
                );
            }

            if current.s() < previous.s() {
                errors.push(
                    AnnealingValidationError::NonMonotonicScheduleProgress {
                        previous: previous.s(),
                        current: current.s(),
                    },
                );
            }
        }

        if let Some(first) = self.points.first() {
            if first.s() != 0.0 {
                errors.push(
                    AnnealingValidationError::ScheduleDoesNotStartAtZero {
                        value: first.s(),
                    },
                );
            }
        }

        if let Some(last) = self.points.last() {
            if last.s() != 1.0 {
                errors.push(
                    AnnealingValidationError::ScheduleDoesNotEndAtOne {
                        value: last.s(),
                    },
                );
            }
        }

        errors.into_result(())
    }
}

impl Default for AnnealingSchedule {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Schedule policy
// =============================================================================

/// Semantic annealing protocol.
///
/// These are semantic protocols, not provider-specific execution commands.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AnnealingProtocol {
    /// Standard monotonic anneal.
    Standard,

    /// Reverse annealing protocol.
    Reverse,

    /// Pause-and-continue protocol.
    Pause,

    /// Quench/rapid termination protocol.
    Quench,

    /// User-supplied semantic schedule.
    Custom,
}

impl Default for AnnealingProtocol {
    fn default() -> Self {
        Self::Standard
    }
}

/// Semantic controls for an annealing workload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnealingControls {
    protocol: AnnealingProtocol,

    /// Optional custom schedule.
    schedule: Option<AnnealingSchedule>,

    /// Optional normalized initial progress.
    initial_s: Option<f64>,

    /// Optional normalized final progress.
    final_s: Option<f64>,

    /// Optional semantic pause point.
    pause_s: Option<f64>,

    /// Optional semantic duration.
    ///
    /// This value has no physical unit at the IR level.
    duration: Option<f64>,

    /// Optional number of requested repetitions.
    ///
    /// This is a workload request, not a backend allocation guarantee.
    repetitions: Option<u64>,

    /// Optional per-variable normalized anneal offsets.
    anneal_offsets: BTreeMap<VariableId, f64>,
}

impl AnnealingControls {
    /// Creates default standard-anneal controls.
    #[must_use]
    pub fn new() -> Self {
        Self {
            protocol: AnnealingProtocol::Standard,
            schedule: None,
            initial_s: None,
            final_s: None,
            pause_s: None,
            duration: None,
            repetitions: None,
            anneal_offsets: BTreeMap::new(),
        }
    }

    /// Sets the protocol.
    #[must_use]
    pub fn with_protocol(mut self, protocol: AnnealingProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Sets a custom schedule.
    pub fn with_schedule(
        mut self,
        schedule: AnnealingSchedule,
    ) -> Result<Self, AnnealingValidationErrors> {
        schedule.validate()?;
        self.schedule = Some(schedule);
        Ok(self)
    }

    /// Sets normalized initial progress.
    pub fn set_initial_s(
        &mut self,
        value: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_progress(value, "initial anneal progress")?;
        self.initial_s = Some(value);
        Ok(())
    }

    /// Sets normalized final progress.
    pub fn set_final_s(
        &mut self,
        value: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_progress(value, "final anneal progress")?;
        self.final_s = Some(value);
        Ok(())
    }

    /// Sets a normalized pause point.
    pub fn set_pause_s(
        &mut self,
        value: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_progress(value, "pause progress")?;
        self.pause_s = Some(value);
        Ok(())
    }

    /// Sets semantic duration.
    pub fn set_duration(
        &mut self,
        value: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(value, "annealing duration")?;

        if value < 0.0 {
            return Err(AnnealingValidationError::NegativeDuration { value });
        }

        self.duration = Some(value);
        Ok(())
    }

    /// Sets requested repetitions.
    pub fn set_repetitions(&mut self, repetitions: u64) {
        self.repetitions = Some(repetitions);
    }

    /// Sets a per-variable anneal offset.
    pub fn set_anneal_offset(
        &mut self,
        variable: VariableId,
        offset: f64,
    ) -> Result<(), AnnealingValidationError> {
        validate_finite(offset, "anneal offset")?;
        self.anneal_offsets.insert(variable, offset);
        Ok(())
    }

    /// Returns the protocol.
    #[must_use]
    pub const fn protocol(&self) -> AnnealingProtocol {
        self.protocol
    }

    /// Returns the optional schedule.
    #[must_use]
    pub fn schedule(&self) -> Option<&AnnealingSchedule> {
        self.schedule.as_ref()
    }

    /// Returns initial progress.
    #[must_use]
    pub const fn initial_s(&self) -> Option<f64> {
        self.initial_s
    }

    /// Returns final progress.
    #[must_use]
    pub const fn final_s(&self) -> Option<f64> {
        self.final_s
    }

    /// Returns pause progress.
    #[must_use]
    pub const fn pause_s(&self) -> Option<f64> {
        self.pause_s
    }

    /// Returns semantic duration.
    #[must_use]
    pub const fn duration(&self) -> Option<f64> {
        self.duration
    }

    /// Returns requested repetitions.
    #[must_use]
    pub const fn repetitions(&self) -> Option<u64> {
        self.repetitions
    }

    /// Returns anneal offsets.
    #[must_use]
    pub fn anneal_offsets(&self) -> &BTreeMap<VariableId, f64> {
        &self.anneal_offsets
    }

    /// Validates controls.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if let Some(schedule) = &self.schedule {
            if let Err(schedule_errors) = schedule.validate() {
                errors.extend(schedule_errors);
            }
        }

        if let (Some(initial), Some(final_s)) = (self.initial_s, self.final_s) {
            if final_s < initial {
                errors.push(
                    AnnealingValidationError::InvalidProgressInterval {
                        start: initial,
                        end: final_s,
                    },
                );
            }
        }

        if self.protocol == AnnealingProtocol::Reverse
            && self.initial_s.is_none()
            && self.schedule.is_none()
        {
            errors.push(AnnealingValidationError::ReverseProtocolNeedsStart);
        }

        for offset in self.anneal_offsets.values() {
            if !offset.is_finite() {
                errors.push(AnnealingValidationError::NonFiniteValue {
                    field: "anneal offset",
                    value: *offset,
                });
            }
        }

        errors.into_result(())
    }
}

impl Default for AnnealingControls {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Resource requirements
// =============================================================================

/// Abstract annealing resource requirement.
///
/// This remains deliberately provider-neutral.
///
/// The concrete hardware layer decides whether and how these requirements can
/// be satisfied.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AnnealingResourceKind {
    /// Number of logical mathematical variables.
    LogicalVariables,

    /// Number of interactions.
    Interactions,

    /// Number of logical qubits required after mapping.
    LogicalQubits,

    /// Number of repetitions requested.
    Repetitions,

    /// Maximum interaction locality.
    InteractionLocality,
}

/// A single abstract annealing resource requirement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnealingResourceRequirement {
    kind: AnnealingResourceKind,
    minimum: u64,
}

impl AnnealingResourceRequirement {
    /// Creates a minimum resource requirement.
    #[must_use]
    pub const fn at_least(
        kind: AnnealingResourceKind,
        minimum: u64,
    ) -> Self {
        Self { kind, minimum }
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> AnnealingResourceKind {
        self.kind
    }

    /// Returns the minimum.
    #[must_use]
    pub const fn minimum(&self) -> u64 {
        self.minimum
    }
}

// =============================================================================
// Workload
// =============================================================================

/// Canonical quantum-annealing workload.
///
/// A workload contains the mathematical problem plus target-independent
/// execution intent.
///
/// It does not represent a submitted job.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnealingWorkload {
    /// Stable semantic workload identifier.
    workload_id: Option<String>,

    /// The problem representation.
    problem: AnnealingProblem,

    /// Optional logical-qubit bindings.
    bindings: BTreeMap<VariableId, QubitId>,

    /// Target-independent annealing controls.
    controls: AnnealingControls,

    /// Explicit semantic resource requirements.
    requirements: BTreeMap<AnnealingResourceKind, AnnealingResourceRequirement>,

    /// Non-secret deterministic metadata.
    metadata: BTreeMap<String, String>,
}

impl AnnealingWorkload {
    /// Creates a workload from a QUBO problem.
    #[must_use]
    pub fn from_qubo(problem: QuboProblem) -> Self {
        Self {
            workload_id: None,
            problem: AnnealingProblem::Qubo(problem),
            bindings: BTreeMap::new(),
            controls: AnnealingControls::new(),
            requirements: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a workload from an Ising model.
    #[must_use]
    pub fn from_ising(problem: IsingModel) -> Self {
        Self {
            workload_id: None,
            problem: AnnealingProblem::Ising(problem),
            bindings: BTreeMap::new(),
            controls: AnnealingControls::new(),
            requirements: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the problem.
    #[must_use]
    pub fn problem(&self) -> &AnnealingProblem {
        &self.problem
    }

    /// Returns mutable access to the problem.
    #[must_use]
    pub fn problem_mut(&mut self) -> &mut AnnealingProblem {
        &mut self.problem
    }

    /// Sets a semantic workload identifier.
    pub fn set_workload_id(
        &mut self,
        id: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        let id = id.into();
        validate_text(&id, "workload identifier")?;
        self.workload_id = Some(id);
        Ok(())
    }

    /// Returns the optional workload identifier.
    #[must_use]
    pub fn workload_id(&self) -> Option<&str> {
        self.workload_id.as_deref()
    }

    /// Binds a variable to a logical qubit.
    pub fn bind_variable(
        &mut self,
        variable: VariableId,
        qubit: QubitId,
    ) -> Result<(), AnnealingValidationError> {
        if !self.problem.variables().contains(&variable) {
            return Err(AnnealingValidationError::UnknownVariable {
                variable,
            });
        }

        if let Some(existing_variable) = self
            .bindings
            .iter()
            .find_map(|(bound_variable, bound_qubit)| {
                if *bound_qubit == qubit && *bound_variable != variable {
                    Some(*bound_variable)
                } else {
                    None
                }
            })
        {
            return Err(
                AnnealingValidationError::LogicalQubitAlreadyBound {
                    qubit,
                    existing_variable,
                },
            );
        }

        self.bindings.insert(variable, qubit);
        Ok(())
    }

    /// Returns variable-to-logical-qubit bindings.
    #[must_use]
    pub fn bindings(&self) -> &BTreeMap<VariableId, QubitId> {
        &self.bindings
    }

    /// Returns mutable controls.
    #[must_use]
    pub fn controls_mut(&mut self) -> &mut AnnealingControls {
        &mut self.controls
    }

    /// Returns controls.
    #[must_use]
    pub fn controls(&self) -> &AnnealingControls {
        &self.controls
    }

    /// Adds a resource requirement.
    pub fn require_at_least(
        &mut self,
        kind: AnnealingResourceKind,
        minimum: u64,
    ) {
        self.requirements.insert(
            kind,
            AnnealingResourceRequirement::at_least(kind, minimum),
        );
    }

    /// Returns resource requirements.
    #[must_use]
    pub fn requirements(
        &self,
    ) -> &BTreeMap<AnnealingResourceKind, AnnealingResourceRequirement> {
        &self.requirements
    }

    /// Inserts deterministic metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), AnnealingValidationError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validates the workload.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        let mut errors = AnnealingValidationErrors::new();

        if let Some(id) = &self.workload_id {
            if let Err(error) = validate_text(id, "workload identifier") {
                errors.push(error);
            }
        }

        if let Err(problem_errors) = self.problem.validate() {
            errors.extend(problem_errors);
        }

        if let Err(control_errors) = self.controls.validate() {
            errors.extend(control_errors);
        }

        for variable in self.bindings.keys() {
            if !self.problem.variables().contains(variable) {
                errors.push(AnnealingValidationError::UnknownVariable {
                    variable: *variable,
                });
            }
        }

        let mut used_qubits = BTreeMap::<QubitId, VariableId>::new();

        for (variable, qubit) in &self.bindings {
            if let Some(existing) = used_qubits.insert(*qubit, *variable) {
                if existing != *variable {
                    errors.push(
                        AnnealingValidationError::LogicalQubitAlreadyBound {
                            qubit: *qubit,
                            existing_variable: existing,
                        },
                    );
                }
            }
        }

        validate_metadata_collection(&self.metadata, &mut errors);

        errors.into_result(())
    }
}

// =============================================================================
// Annealing problem
// =============================================================================

/// The mathematical problem representation carried by an annealing workload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AnnealingProblem {
    /// Binary QUBO representation.
    Qubo(QuboProblem),

    /// Spin Ising representation.
    Ising(IsingModel),
}

impl AnnealingProblem {
    /// Returns all semantic variables.
    #[must_use]
    pub fn variables(&self) -> &BTreeSet<VariableId> {
        match self {
            Self::Qubo(problem) => problem.variables(),
            Self::Ising(problem) => problem.variables(),
        }
    }

    /// Returns the variable count.
    #[must_use]
    pub fn variable_count(&self) -> usize {
        self.variables().len()
    }

    /// Validates the mathematical problem.
    pub fn validate(&self) -> Result<(), AnnealingValidationErrors> {
        match self {
            Self::Qubo(problem) => problem.validate(),
            Self::Ising(problem) => problem.validate(),
        }
    }

    /// Converts the problem to canonical Ising form.
    pub fn to_ising(&self) -> Result<IsingModel, AnnealingValidationErrors> {
        match self {
            Self::Qubo(problem) => problem.to_ising(),
            Self::Ising(problem) => Ok(problem.clone()),
        }
    }

    /// Converts the problem to canonical QUBO form.
    pub fn to_qubo(&self) -> Result<QuboProblem, AnnealingValidationErrors> {
        match self {
            Self::Qubo(problem) => Ok(problem.clone()),
            Self::Ising(problem) => problem.to_qubo(),
        }
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// One annealing validation error.
#[derive(Clone, Debug, PartialEq)]
pub enum AnnealingValidationError {
    /// A numerical field contained NaN or infinity.
    NonFiniteValue {
        /// Field description.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// Duplicate variable declaration.
    DuplicateVariable {
        /// Variable.
        variable: VariableId,
    },

    /// A variable referenced by a coefficient was not explicitly declared.
    MissingDeclaredVariable {
        /// Variable.
        variable: VariableId,
    },

    /// A variable is not present in the model.
    UnknownVariable {
        /// Variable.
        variable: VariableId,
    },

    /// A pair contains the same variable twice.
    SelfInteraction {
        /// Variable.
        variable: VariableId,
    },

    /// Pair is not in canonical order.
    NonCanonicalPair {
        /// Pair.
        pair: VariablePair,
    },

    /// Required assignment is missing.
    MissingAssignment {
        /// Variable.
        variable: VariableId,
    },

    /// Assignment contains a variable not present in the model.
    UnknownAssignmentVariable {
        /// Variable.
        variable: VariableId,
    },

    /// A spin assignment is not -1 or +1.
    InvalidSpinValue {
        /// Variable.
        variable: VariableId,

        /// Invalid value.
        value: i8,
    },

    /// Schedule progress is outside [0,1].
    ScheduleProgressOutOfRange {
        /// Invalid value.
        value: f64,
    },

    /// Schedule time is negative.
    NegativeScheduleTime {
        /// Invalid value.
        value: f64,
    },

    /// Schedule time is not strictly increasing.
    NonMonotonicScheduleTime {
        /// Previous time.
        previous: f64,

        /// Current time.
        current: f64,
    },

    /// Schedule progress decreases.
    NonMonotonicScheduleProgress {
        /// Previous progress.
        previous: f64,

        /// Current progress.
        current: f64,
    },

    /// Schedule does not start at normalized zero.
    ScheduleDoesNotStartAtZero {
        /// First value.
        value: f64,
    },

    /// Schedule does not finish at normalized one.
    ScheduleDoesNotEndAtOne {
        /// Final value.
        value: f64,
    },

    /// Duration is negative.
    NegativeDuration {
        /// Invalid value.
        value: f64,
    },

    /// Progress interval is reversed.
    InvalidProgressInterval {
        /// Start.
        start: f64,

        /// End.
        end: f64,
    },

    /// Reverse protocol lacks enough start information.
    ReverseProtocolNeedsStart,

    /// Logical qubit already belongs to another variable.
    LogicalQubitAlreadyBound {
        /// Qubit.
        qubit: QubitId,

        /// Existing variable.
        existing_variable: VariableId,
    },

    /// Text metadata is empty.
    EmptyText {
        /// Field.
        field: &'static str,
    },

    /// Text contains an invalid control character.
    InvalidText {
        /// Field.
        field: &'static str,
    },

    /// Metadata key is too large for the semantic metadata contract.
    MetadataKeyTooLong {
        /// Length.
        length: usize,
    },

    /// Metadata value is too large for the semantic metadata contract.
    MetadataValueTooLong {
        /// Length.
        length: usize,
    },

    /// Checked floating-point arithmetic produced a non-finite result.
    ArithmeticOverflow {
        /// Operation.
        operation: &'static str,
    },
}

impl fmt::Display for AnnealingValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field, value } => {
                write!(formatter, "{field} is not finite: {value}")
            }
            Self::DuplicateVariable { variable } => {
                write!(formatter, "duplicate variable {variable}")
            }
            Self::MissingDeclaredVariable { variable } => {
                write!(
                    formatter,
                    "coefficient references undeclared variable {variable}"
                )
            }
            Self::UnknownVariable { variable } => {
                write!(formatter, "unknown variable {variable}")
            }
            Self::SelfInteraction { variable } => {
                write!(formatter, "self interaction on {variable}")
            }
            Self::NonCanonicalPair { pair } => {
                write!(formatter, "non-canonical interaction pair {pair}")
            }
            Self::MissingAssignment { variable } => {
                write!(formatter, "missing assignment for {variable}")
            }
            Self::UnknownAssignmentVariable { variable } => {
                write!(
                    formatter,
                    "assignment contains unknown variable {variable}"
                )
            }
            Self::InvalidSpinValue { variable, value } => {
                write!(
                    formatter,
                    "invalid spin value {value} for {variable}; expected -1 or +1"
                )
            }
            Self::ScheduleProgressOutOfRange { value } => {
                write!(
                    formatter,
                    "annealing progress {value} is outside [0, 1]"
                )
            }
            Self::NegativeScheduleTime { value } => {
                write!(
                    formatter,
                    "annealing schedule time cannot be negative: {value}"
                )
            }
            Self::NonMonotonicScheduleTime { previous, current } => {
                write!(
                    formatter,
                    "annealing schedule time is not strictly increasing: {previous} -> {current}"
                )
            }
            Self::NonMonotonicScheduleProgress { previous, current } => {
                write!(
                    formatter,
                    "annealing progress decreases: {previous} -> {current}"
                )
            }
            Self::ScheduleDoesNotStartAtZero { value } => {
                write!(
                    formatter,
                    "annealing schedule must start at s=0, found {value}"
                )
            }
            Self::ScheduleDoesNotEndAtOne { value } => {
                write!(
                    formatter,
                    "annealing schedule must end at s=1, found {value}"
                )
            }
            Self::NegativeDuration { value } => {
                write!(formatter, "annealing duration cannot be negative: {value}")
            }
            Self::InvalidProgressInterval { start, end } => {
                write!(
                    formatter,
                    "invalid annealing progress interval: {start} -> {end}"
                )
            }
            Self::ReverseProtocolNeedsStart => {
                formatter.write_str(
                    "reverse annealing requires explicit start progress or a schedule",
                )
            }
            Self::LogicalQubitAlreadyBound {
                qubit,
                existing_variable,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is already bound to {existing_variable}"
                )
            }
            Self::EmptyText { field } => {
                write!(formatter, "{field} cannot be empty")
            }
            Self::InvalidText { field } => {
                write!(formatter, "{field} contains an invalid control character")
            }
            Self::MetadataKeyTooLong { length } => {
                write!(
                    formatter,
                    "metadata key is too long: {length} bytes"
                )
            }
            Self::MetadataValueTooLong { length } => {
                write!(
                    formatter,
                    "metadata value is too long: {length} bytes"
                )
            }
            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "non-finite result during {operation}"
                )
            }
        }
    }
}

impl Error for AnnealingValidationError {}

/// Collection of validation errors.
///
/// Validation is deliberately capable of returning multiple errors so callers
/// can repair a whole workload instead of repeatedly discovering one error at
/// a time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnnealingValidationErrors {
    errors: Vec<AnnealingValidationError>,
}

impl AnnealingValidationErrors {
    /// Creates an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Creates a collection containing one error.
    #[must_use]
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
    pub fn extend(&mut self, other: Self) {
        self.errors.extend(other.errors);
    }

    /// Returns whether no errors exist.
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
    pub fn as_slice(&self) -> &[AnnealingValidationError] {
        &self.errors
    }

    /// Converts the collection to a result.
    pub fn into_result<T>(
        self,
        value: T,
    ) -> Result<T, Self> {
        if self.is_empty() {
            Ok(value)
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for AnnealingValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.errors.iter().enumerate() {
            if index != 0 {
                formatter.write_str("; ")?;
            }

            write!(formatter, "{error}")?;
        }

        Ok(())
    }
}

impl Error for AnnealingValidationErrors {}

// =============================================================================
// Numerical helpers
// =============================================================================

fn validate_finite(
    value: f64,
    field: &'static str,
) -> Result<(), AnnealingValidationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AnnealingValidationError::NonFiniteValue {
            field,
            value,
        })
    }
}

fn checked_add_f64(
    lhs: f64,
    rhs: f64,
    operation: &'static str,
) -> Result<f64, AnnealingValidationError> {
    let result = lhs + rhs;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(AnnealingValidationError::ArithmeticOverflow {
            operation,
        })
    }
}

fn checked_sub_f64(
    lhs: f64,
    rhs: f64,
    operation: &'static str,
) -> Result<f64, AnnealingValidationError> {
    let result = lhs - rhs;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(AnnealingValidationError::ArithmeticOverflow {
            operation,
        })
    }
}

fn checked_mul_f64(
    lhs: f64,
    rhs: f64,
    operation: &'static str,
) -> Result<f64, AnnealingValidationError> {
    let result = lhs * rhs;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(AnnealingValidationError::ArithmeticOverflow {
            operation,
        })
    }
}

fn validate_progress(
    value: f64,
    field: &'static str,
) -> Result<(), AnnealingValidationError> {
    validate_finite(value, field)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(AnnealingValidationError::ScheduleProgressOutOfRange {
            value,
        });
    }

    Ok(())
}

fn validate_text(
    text: &str,
    field: &'static str,
) -> Result<(), AnnealingValidationError> {
    if text.is_empty() {
        return Err(AnnealingValidationError::EmptyText { field });
    }

    if text.chars().any(char::is_control) {
        return Err(AnnealingValidationError::InvalidText { field });
    }

    Ok(())
}

fn validate_metadata(
    key: &str,
    value: &str,
) -> Result<(), AnnealingValidationError> {
    validate_text(key, "metadata key")?;

    if key.len() > 256 {
        return Err(AnnealingValidationError::MetadataKeyTooLong {
            length: key.len(),
        });
    }

    if value.len() > 4096 {
        return Err(AnnealingValidationError::MetadataValueTooLong {
            length: value.len(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AnnealingValidationError::InvalidText {
            field: "metadata value",
        });
    }

    Ok(())
}

fn validate_metadata_collection(
    metadata: &BTreeMap<String, String>,
    errors: &mut AnnealingValidationErrors,
) {
    for (key, value) in metadata {
        if let Err(error) = validate_metadata(key, value) {
            errors.push(error);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn v(value: u64) -> VariableId {
        VariableId::new(value)
    }

    #[test]
    fn variable_pair_is_canonicalized() {
        let pair = VariablePair::new(v(8), v(2))
            .expect("distinct variables must form a pair");

        assert_eq!(pair.first(), v(2));
        assert_eq!(pair.second(), v(8));
    }

    #[test]
    fn self_interaction_is_rejected() {
        let error = VariablePair::new(v(1), v(1))
            .expect_err("self interactions must be rejected");

        assert_eq!(
            error,
            AnnealingValidationError::SelfInteraction {
                variable: v(1)
            }
        );
    }

    #[test]
    fn sparse_qubo_can_be_built_without_dense_storage() {
        let mut qubo = QuboProblem::new();

        qubo.add_linear(v(1_000_000), 2.0)
            .expect("finite coefficient");

        qubo.add_quadratic(v(1_000_000), v(2_000_000), -1.5)
            .expect("finite coefficient");

        assert_eq!(qubo.variable_count(), 2);
        assert_eq!(
            qubo.quadratic_coefficient(v(2_000_000), v(1_000_000)),
            Some(-1.5)
        );
    }

    #[test]
    fn qubo_energy_is_correct() {
        let mut qubo = QuboProblem::new();

        qubo.add_linear(v(0), 2.0)
            .expect("finite coefficient");

        qubo.add_linear(v(1), 3.0)
            .expect("finite coefficient");

        qubo.add_quadratic(v(0), v(1), -4.0)
            .expect("finite coefficient");

        let mut assignment = BTreeMap::new();
        assignment.insert(v(0), true);
        assignment.insert(v(1), true);

        let energy = qubo
            .energy(&assignment)
            .expect("complete assignment");

        assert!((energy - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn qubo_ising_conversion_preserves_energy() {
        let mut qubo = QuboProblem::new();

        qubo.set_offset(1.25)
            .expect("finite offset");

        qubo.add_linear(v(0), 2.0)
            .expect("finite coefficient");

        qubo.add_linear(v(1), -3.0)
            .expect("finite coefficient");

        qubo.add_quadratic(v(0), v(1), 4.0)
            .expect("finite coefficient");

        let ising = qubo
            .to_ising()
            .expect("valid QUBO must convert");

        let mut binary = BTreeMap::new();
        binary.insert(v(0), true);
        binary.insert(v(1), false);

        let mut spin = BTreeMap::new();
        spin.insert(v(0), 1);
        spin.insert(v(1), -1);

        let qubo_energy = qubo
            .energy(&binary)
            .expect("valid binary assignment");

        let ising_energy = ising
            .energy(&spin)
            .expect("valid spin assignment");

        assert!(
            (qubo_energy - ising_energy).abs() < 1.0e-10,
            "{qubo_energy} != {ising_energy}"
        );
    }

    #[test]
    fn ising_qubo_conversion_preserves_energy() {
        let mut ising = IsingModel::new();

        ising.set_offset(0.75)
            .expect("finite offset");

        ising.add_linear(v(0), 1.5)
            .expect("finite coefficient");

        ising.add_linear(v(1), -0.5)
            .expect("finite coefficient");

        ising.add_quadratic(v(0), v(1), 2.0)
            .expect("finite coefficient");

        let qubo = ising
            .to_qubo()
            .expect("valid Ising model must convert");

        let mut spin = BTreeMap::new();
        spin.insert(v(0), 1);
        spin.insert(v(1), -1);

        let mut binary = BTreeMap::new();
        binary.insert(v(0), true);
        binary.insert(v(1), false);

        let ising_energy = ising
            .energy(&spin)
            .expect("valid spin assignment");

        let qubo_energy = qubo
            .energy(&binary)
            .expect("valid binary assignment");

        assert!(
            (ising_energy - qubo_energy).abs() < 1.0e-10,
            "{ising_energy} != {qubo_energy}"
        );
    }

    #[test]
    fn schedule_requires_zero_to_one_progress() {
        let first = AnnealingSchedulePoint::new(0.0, 0.0)
            .expect("valid point");

        let last = AnnealingSchedulePoint::new(1.0, 10.0)
            .expect("valid point");

        let schedule = AnnealingSchedule::from_points(vec![first, last])
            .expect("valid schedule");

        assert_eq!(schedule.len(), 2);
    }

    #[test]
    fn schedule_rejects_decreasing_progress() {
        let first = AnnealingSchedulePoint::new(0.0, 0.0)
            .expect("valid point");

        let second = AnnealingSchedulePoint::new(0.5, 2.0)
            .expect("valid point");

        let third = AnnealingSchedulePoint::new(0.4, 3.0)
            .expect("valid point");

        let result = AnnealingSchedule::from_points(vec![first, second, third]);

        assert!(result.is_err());
    }

    #[test]
    fn logical_qubit_binding_uses_canonical_qubit_id() {
        let mut qubo = QuboProblem::new();

        qubo.add_variable(v(10))
            .expect("variable should be added");

        let mut workload = AnnealingWorkload::from_qubo(qubo);

        workload
            .bind_variable(v(10), QubitId::new(7))
            .expect("binding should succeed");

        assert_eq!(
            workload.bindings().get(&v(10)),
            Some(&QubitId::new(7))
        );
    }

    #[test]
    fn duplicate_logical_qubit_binding_is_rejected() {
        let mut qubo = QuboProblem::new();

        qubo.add_variable(v(1))
            .expect("variable should be added");

        qubo.add_variable(v(2))
            .expect("variable should be added");

        let mut workload = AnnealingWorkload::from_qubo(qubo);

        workload
            .bind_variable(v(1), QubitId::new(4))
            .expect("first binding should succeed");

        let error = workload
            .bind_variable(v(2), QubitId::new(4))
            .expect_err("same logical qubit cannot be bound twice");

        assert_eq!(
            error,
            AnnealingValidationError::LogicalQubitAlreadyBound {
                qubit: QubitId::new(4),
                existing_variable: v(1),
            }
        );
    }

    #[test]
    fn non_finite_coefficients_are_rejected() {
        let mut qubo = QuboProblem::new();

        let error = qubo
            .add_linear(v(0), f64::NAN)
            .expect_err("NaN must be rejected");

        assert!(matches!(
            error,
            AnnealingValidationError::NonFiniteValue { .. }
        ));
    }

    #[test]
    fn workload_validation_detects_unknown_binding() {
        let mut qubo = QuboProblem::new();

        qubo.add_variable(v(1))
            .expect("variable should be added");

        let mut workload = AnnealingWorkload::from_qubo(qubo);

        workload.bindings.insert(v(99), QubitId::new(3));

        let errors = workload
            .validate()
            .expect_err("unknown binding must be rejected");

        assert!(errors.as_slice().iter().any(|error| {
            matches!(
                error,
                AnnealingValidationError::UnknownVariable {
                    variable
                } if *variable == v(99)
            )
        }));
    }

    #[test]
    fn sparse_variable_ids_do_not_imply_dense_allocation() {
        let mut qubo = QuboProblem::new();

        qubo.add_variable(VariableId::new(u64::MAX - 1))
            .expect("large semantic id is valid");

        assert_eq!(qubo.variable_count(), 1);
    }

    #[test]
    fn default_controls_are_valid() {
        AnnealingControls::new()
            .validate()
            .expect("default controls must be valid");
    }

    #[test]
    fn default_models_are_valid() {
        QuboProblem::new()
            .validate()
            .expect("empty QUBO is valid");

        IsingModel::new()
            .validate()
            .expect("empty Ising model is valid");
    }
}