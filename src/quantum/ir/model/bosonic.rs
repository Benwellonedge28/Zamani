//! Zamani Quantum IR — Bosonic Quantum Model
//!
//! Canonical, hardware-independent semantic representation of bosonic
//! quantum computation.
//!
//! # Architectural role
//!
//! This module represents quantum computations whose fundamental degrees of
//! freedom are bosonic modes rather than qubits.
//!
//! It is intentionally independent of:
//!
//! - a particular photonic architecture;
//! - a particular optical platform;
//! - a particular continuous-variable hardware implementation;
//! - a simulator state representation;
//! - a Fock-space truncation policy;
//! - a compiler optimization strategy;
//! - a routing implementation;
//! - a pulse compiler;
//! - a backend;
//! - a detector implementation;
//! - a vendor SDK.
//!
//! The semantic dependency direction is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! BosonicModel
//!      │
//!      ├── target-independent optimization
//!      ├── resource analysis
//!      ├── mapping
//!      ├── scheduling
//!      └── lowering
//!              │
//!              ▼
//!       target/hardware/backend
//! ```
//!
//! # Why bosonic computation is a separate model
//!
//! A bosonic mode is not intrinsically a qubit.
//!
//! A bosonic system may use:
//!
//! - Fock states;
//! - coherent states;
//! - squeezed states;
//! - thermal states;
//! - arbitrary wave packets;
//! - continuous variables;
//! - photon-number states;
//! - Gaussian operations;
//! - non-Gaussian operations;
//! - interferometers;
//! - adaptive measurements;
//! - multimode transformations.
//!
//! Therefore this module must not encode the assumption:
//!
//! ```text
//! one quantum resource == one qubit
//! ```
//!
//! Nor may it assume:
//!
//! ```text
//! mode occupation <= fixed hardware number
//! ```
//!
//! # Scalability principle
//!
//! The number of modes is data, not an architectural constant.
//!
//! The model supports:
//!
//! ```text
//! 1 mode
//! 2 modes
//! N modes
//! arbitrarily large finite mode sets
//! ```
//!
//! subject only to explicit caller/compiler/resource constraints and the
//! representational limits of the host environment.
//!
//! This file deliberately does NOT contain:
//!
//! ```text
//! MAX_MODES
//! MAX_PHOTONS
//! MAX_FOCK_LEVEL
//! MAX_INTERFEROMETER_SIZE
//! ```
//!
//! Any such restriction belongs to an explicit resource/security policy,
//! simulator configuration, target capability, or backend.
//!
//! # No implicit Fock-space truncation
//!
//! A particularly important invariant is:
//!
//! > The canonical bosonic IR never silently truncates an occupation space.
//!
//! A backend may require a finite Fock cutoff. That cutoff must be represented
//! as target/simulation policy, not silently inserted into the semantic model.
//!
//! # Parameters
//!
//! Numerical coefficients use the canonical Zamani IR [`Parameter`] type.
//!
//! This permits:
//!
//! - constants;
//! - symbolic parameters;
//! - expressions;
//! - late binding;
//! - target-independent optimization;
//! - runtime parameter binding.
//!
//! # Operators
//!
//! The model provides canonical bosonic operator primitives:
//!
//! - creation;
//! - annihilation;
//! - number;
//! - position/quadrature;
//! - momentum/quadrature;
//! - parity;
//! - identity;
//! - custom extensible operators.
//!
//! Higher-level operations include:
//!
//! - prepare Fock state;
//! - prepare coherent state;
//! - prepare squeezed state;
//! - prepare thermal state;
//! - displacement;
//! - phase rotation;
//! - squeezing;
//! - beam splitter;
//! - phase shifter;
//! - multimode interferometer;
//! - conditional operation;
//! - measurement;
//! - custom operations.
//!
//! The list is a standard semantic vocabulary, not a closed universe.
//!
//! # Hardware independence
//!
//! A beam splitter in this IR does not specify:
//!
//! - a physical optical element;
//! - a waveguide;
//! - a fiber;
//! - a cavity;
//! - a frequency band;
//! - a DAC;
//! - a detector;
//! - a vendor;
//! - a physical port.
//!
//! Those decisions belong downstream.
//!
//! # Hybrid quantum systems
//!
//! This module does not pretend that a bosonic mode is a qubit.
//!
//! If a future hybrid model needs to connect a bosonic mode with a logical
//! qubit, it should use an explicit hybrid/encoding operation at the model
//! integration boundary and use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! there.
//!
//! That preserves the distinction between:
//!
//! ```text
//! QubitId        = qubit identity
//! BosonicModeId  = bosonic mode identity
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! # Integration contract
//!
//! `model/mod.rs` should expose this module with:
//!
//! ```text
//! pub mod bosonic;
//! ```
//!
//! `quantum::ir::parameter::Parameter` is consumed for symbolic numerical
//! quantities.
//!
//! `quantum::ir::qubit::QubitId` is intentionally NOT used for bosonic mode
//! identity. It belongs only at an explicit hybrid boundary.
//!
//! Validation, serialization, hashing and backend lowering should consume
//! this model rather than embedding bosonic semantics elsewhere.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - bosonic mode identity;
//! - mode collections;
//! - bosonic basis/state preparation descriptions;
//! - bosonic operator semantics;
//! - bosonic terms and expressions;
//! - bosonic operations;
//! - bosonic measurements;
//! - multimode transformations;
//! - semantic model validation;
//! - deterministic structural inspection.
//!
//! This file does NOT own:
//!
//! - physical topology;
//! - detector calibration;
//! - optical hardware;
//! - Fock-space simulation;
//! - numerical state vectors;
//! - density matrices;
//! - photon sampling execution;
//! - pulse synthesis;
//! - scheduling;
//! - routing;
//! - hardware mapping.
//!
//! # Determinism
//!
//! Ordered collections are used where semantic ordering matters.
//! User-provided operation order is preserved.
//!
//! Metadata and custom names are explicit values rather than hidden global
//! registries.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::parameter::Parameter;

// =============================================================================
// Bosonic mode identity
// =============================================================================

/// Stable identity of a bosonic quantum mode.
///
/// A `BosonicModeId` is a semantic identifier. It does not imply:
///
/// - a physical optical path;
/// - a hardware channel;
/// - a detector;
/// - a frequency;
/// - a spatial location;
/// - a physical qubit.
///
/// Mode placement is a downstream concern.
///
/// The underlying value is `u64` rather than `usize` so the semantic identity
/// does not change merely because the compiler runs on a different host
/// pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BosonicModeId(u64);

impl BosonicModeId {
    /// Creates a bosonic mode identifier.
    #[must_use]
    pub const fn new(index: u64) -> Self {
        Self(index)
    }

    /// Returns the stable numeric identifier.
    #[must_use]
    pub const fn index(self) -> u64 {
        self.0
    }

    /// Returns the next identifier when representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for BosonicModeId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<BosonicModeId> for u64 {
    fn from(value: BosonicModeId) -> Self {
        value.index()
    }
}

impl fmt::Display for BosonicModeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "m{}", self.0)
    }
}

// =============================================================================
// Mode collection
// =============================================================================

/// Deterministic collection of bosonic modes.
///
/// This collection does not imply that the modes are physically adjacent,
/// connected, or simultaneously available.
///
/// It is a semantic namespace/container only.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BosonicModes {
    modes: Vec<BosonicModeId>,
}

impl BosonicModes {
    /// Creates an empty mode collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { modes: Vec::new() }
    }

    /// Creates a collection from explicit identifiers.
    ///
    /// Identifiers must be unique.
    pub fn from_ids<I>(
        ids: I,
    ) -> Result<Self, BosonicModelError>
    where
        I: IntoIterator<Item = BosonicModeId>,
    {
        let mut modes = Vec::new();
        let mut seen = BTreeSet::new();

        for mode in ids {
            if !seen.insert(mode) {
                return Err(BosonicModelError::DuplicateMode(mode));
            }

            modes.push(mode);
        }

        Ok(Self { modes })
    }

    /// Creates `count` consecutive semantic mode identifiers starting at
    /// `start`.
    ///
    /// This is a convenience constructor, not a hardware allocation API.
    pub fn consecutive(
        start: u64,
        count: u64,
    ) -> Result<Self, BosonicModelError> {
        let end = start.checked_add(count).ok_or(
            BosonicModelError::IdentifierOverflow,
        )?;

        let mut modes = Vec::new();
        let mut current = start;

        while current < end {
            modes.push(BosonicModeId::new(current));

            current = current.checked_add(1).ok_or(
                BosonicModelError::IdentifierOverflow,
            )?;
        }

        Ok(Self { modes })
    }

    /// Returns the number of explicitly represented modes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.modes.len()
    }

    /// Returns whether the collection contains no modes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modes.is_empty()
    }

    /// Returns whether a mode is present.
    #[must_use]
    pub fn contains(&self, mode: BosonicModeId) -> bool {
        self.modes.contains(&mode)
    }

    /// Returns the mode at a container position.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BosonicModeId> {
        self.modes.get(index).copied()
    }

    /// Returns the modes as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[BosonicModeId] {
        &self.modes
    }

    /// Appends a mode if it does not already exist.
    pub fn push(
        &mut self,
        mode: BosonicModeId,
    ) -> Result<(), BosonicModelError> {
        if self.contains(mode) {
            return Err(BosonicModelError::DuplicateMode(mode));
        }

        self.modes.push(mode);
        Ok(())
    }

    /// Returns an iterator over modes in declaration order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = BosonicModeId> + '_ {
        self.modes.iter().copied()
    }
}

impl IntoIterator for BosonicModes {
    type Item = BosonicModeId;
    type IntoIter = std::vec::IntoIter<BosonicModeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.modes.into_iter()
    }
}

// =============================================================================
// Fock occupation
// =============================================================================

/// Exact finite Fock occupation number.
///
/// This is a semantic photon/mode occupation value.
///
/// It is deliberately not a simulator amplitude and not a probability.
///
/// The value is `u64` so the canonical IR does not artificially restrict
/// occupation to common small values such as 0, 1, 2, or 3.
///
/// A backend or simulator may impose a smaller practical range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FockOccupation(u64);

impl FockOccupation {
    /// Vacuum occupation.
    pub const ZERO: Self = Self(0);

    /// Creates an occupation number.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the occupation value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable occupation.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for FockOccupation {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<FockOccupation> for u64 {
    fn from(value: FockOccupation) -> Self {
        value.value()
    }
}

impl fmt::Display for FockOccupation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Fock state
// =============================================================================

/// Explicit finite Fock-basis assignment.
///
/// Only explicitly assigned modes are represented. Unmentioned modes are
/// interpreted by the surrounding operation according to its semantics.
///
/// For a complete state preparation, use [`BosonicStatePreparation::Fock`]
/// with the required mode assignments.
///
/// The structure does not materialize a Hilbert space and therefore does not
/// introduce a dimension limit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FockState {
    occupations: Vec<(BosonicModeId, FockOccupation)>,
}

impl FockState {
    /// Creates an empty assignment.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            occupations: Vec::new(),
        }
    }

    /// Creates a Fock state from explicit assignments.
    pub fn from_assignments<I>(
        assignments: I,
    ) -> Result<Self, BosonicModelError>
    where
        I: IntoIterator<Item = (BosonicModeId, FockOccupation)>,
    {
        let mut state = Self::new();

        for (mode, occupation) in assignments {
            state.set(mode, occupation)?;
        }

        Ok(state)
    }

    /// Assigns an occupation to a mode.
    ///
    /// Reassigning an existing mode replaces its previous semantic value.
    pub fn set(
        &mut self,
        mode: BosonicModeId,
        occupation: FockOccupation,
    ) -> Result<(), BosonicModelError> {
        if let Some(existing) = self
            .occupations
            .iter_mut()
            .find(|(existing, _)| *existing == mode)
        {
            existing.1 = occupation;
            return Ok(());
        }

        self.occupations.push((mode, occupation));
        Ok(())
    }

    /// Returns an occupation assignment.
    #[must_use]
    pub fn get(
        &self,
        mode: BosonicModeId,
    ) -> Option<FockOccupation> {
        self.occupations
            .iter()
            .find(|(candidate, _)| *candidate == mode)
            .map(|(_, occupation)| *occupation)
    }

    /// Returns all assignments.
    #[must_use]
    pub fn assignments(
        &self,
    ) -> &[(BosonicModeId, FockOccupation)] {
        &self.occupations
    }

    /// Returns whether no explicit occupations exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.occupations.is_empty()
    }

    /// Returns the number of explicit mode assignments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.occupations.len()
    }

    /// Validates that every mode is unique.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        let mut seen = BTreeSet::new();

        for (mode, _) in &self.occupations {
            if !seen.insert(*mode) {
                return Err(BosonicModelError::DuplicateMode(*mode));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Bosonic operator kind
// =============================================================================

/// Primitive bosonic operator kind.
///
/// Standard primitives are represented directly. [`Custom`] provides an
/// explicit extension point for future bosonic architectures without forcing
/// every new operator into this file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BosonicOperatorKind {
    /// Identity operator.
    Identity,

    /// Creation operator a†.
    Creation,

    /// Annihilation operator a.
    Annihilation,

    /// Number operator n = a†a.
    Number,

    /// Position quadrature.
    Position,

    /// Momentum quadrature.
    Momentum,

    /// Parity operator.
    Parity,

    /// Extensible namespaced/custom operator.
    Custom(String),
}

impl BosonicOperatorKind {
    /// Creates a validated custom operator name.
    pub fn custom<S: Into<String>>(
        name: S,
    ) -> Result<Self, BosonicModelError> {
        let name = name.into();

        validate_name(&name, "bosonic operator")?;

        Ok(Self::Custom(name))
    }

    /// Returns the stable semantic name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Identity => "identity",
            Self::Creation => "creation",
            Self::Annihilation => "annihilation",
            Self::Number => "number",
            Self::Position => "position",
            Self::Momentum => "momentum",
            Self::Parity => "parity",
            Self::Custom(name) => name,
        }
    }

    /// Returns whether the operator is custom.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

// =============================================================================
// Bosonic operator
// =============================================================================

/// A bosonic operator acting on one mode.
///
/// Multi-mode operators are represented as products/sums of individual
/// operator factors through [`BosonicOperatorProduct`] and
/// [`BosonicOperatorExpression`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BosonicOperator {
    mode: BosonicModeId,
    kind: BosonicOperatorKind,
}

impl BosonicOperator {
    /// Creates a primitive bosonic operator.
    #[must_use]
    pub const fn new(
        mode: BosonicModeId,
        kind: BosonicOperatorKind,
    ) -> Self {
        Self { mode, kind }
    }

    /// Returns the target mode.
    #[must_use]
    pub const fn mode(&self) -> BosonicModeId {
        self.mode
    }

    /// Returns the operator kind.
    #[must_use]
    pub fn kind(&self) -> &BosonicOperatorKind {
        &self.kind
    }

    /// Validates the operator.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        if let BosonicOperatorKind::Custom(name) = &self.kind {
            validate_name(name, "bosonic operator")?;
        }

        Ok(())
    }
}

// =============================================================================
// Operator product
// =============================================================================

/// Ordered product of bosonic operators.
///
/// Operator ordering is semantically significant and is therefore preserved.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BosonicOperatorProduct {
    factors: Vec<BosonicOperator>,
}

impl BosonicOperatorProduct {
    /// Creates an empty product, representing the multiplicative identity.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            factors: Vec::new(),
        }
    }

    /// Creates a product from explicit factors.
    pub fn from_factors<I>(
        factors: I,
    ) -> Result<Self, BosonicModelError>
    where
        I: IntoIterator<Item = BosonicOperator>,
    {
        let mut product = Self::identity();

        for factor in factors {
            product.push(factor)?;
        }

        Ok(product)
    }

    /// Appends an operator factor.
    pub fn push(
        &mut self,
        factor: BosonicOperator,
    ) -> Result<(), BosonicModelError> {
        factor.validate()?;
        self.factors.push(factor);
        Ok(())
    }

    /// Returns all factors.
    #[must_use]
    pub fn factors(&self) -> &[BosonicOperator] {
        &self.factors
    }

    /// Returns whether this is the multiplicative identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the number of factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns whether the product has no factors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    /// Validates all factors.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        for factor in &self.factors {
            factor.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Operator expression
// =============================================================================

/// Sum of bosonic operator products.
///
/// Each term consists of a symbolic/numeric coefficient and an ordered
/// operator product.
///
/// This representation is suitable for:
///
/// - Hamiltonians;
/// - observables;
/// - generators;
/// - non-Gaussian operators;
/// - custom multimode transformations.
///
/// It does not impose a matrix dimension or Fock-space truncation.
#[derive(Debug, Clone, PartialEq)]
pub struct BosonicOperatorExpression {
    terms: Vec<BosonicOperatorTerm>,
}

impl Default for BosonicOperatorExpression {
    fn default() -> Self {
        Self::new()
    }
}

impl BosonicOperatorExpression {
    /// Creates the zero operator expression.
    #[must_use]
    pub const fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Creates an expression from explicit terms.
    pub fn from_terms<I>(
        terms: I,
    ) -> Result<Self, BosonicModelError>
    where
        I: IntoIterator<Item = BosonicOperatorTerm>,
    {
        let mut expression = Self::new();

        for term in terms {
            expression.push(term)?;
        }

        Ok(expression)
    }

    /// Appends an operator term.
    pub fn push(
        &mut self,
        term: BosonicOperatorTerm,
    ) -> Result<(), BosonicModelError> {
        term.validate()?;
        self.terms.push(term);
        Ok(())
    }

    /// Returns all terms.
    #[must_use]
    pub fn terms(&self) -> &[BosonicOperatorTerm] {
        &self.terms
    }

    /// Returns the number of terms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether this expression is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Validates all terms.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        for term in &self.terms {
            term.validate()?;
        }

        Ok(())
    }

    /// Collects all modes referenced by this expression in deterministic
    /// numeric order.
    #[must_use]
    pub fn referenced_modes(&self) -> Vec<BosonicModeId> {
        let mut modes = BTreeSet::new();

        for term in &self.terms {
            for factor in term.product.factors() {
                modes.insert(factor.mode());
            }
        }

        modes.into_iter().collect()
    }
}

// =============================================================================
// Operator term
// =============================================================================

/// One coefficient multiplied by one ordered bosonic operator product.
#[derive(Debug, Clone, PartialEq)]
pub struct BosonicOperatorTerm {
    coefficient: Parameter,
    product: BosonicOperatorProduct,
}

impl BosonicOperatorTerm {
    /// Creates an operator term.
    pub fn new(
        coefficient: Parameter,
        product: BosonicOperatorProduct,
    ) -> Result<Self, BosonicModelError> {
        coefficient
            .validate()
            .map_err(|error| {
                BosonicModelError::InvalidParameter(error.to_string())
            })?;

        product.validate()?;

        Ok(Self {
            coefficient,
            product,
        })
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Returns the operator product.
    #[must_use]
    pub fn product(&self) -> &BosonicOperatorProduct {
        &self.product
    }

    /// Validates this term.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        self.coefficient
            .validate()
            .map_err(|error| {
                BosonicModelError::InvalidParameter(error.to_string())
            })?;

        self.product.validate()
    }
}

// =============================================================================
// State preparation
// =============================================================================

/// Canonical bosonic state-preparation semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum BosonicStatePreparation {
    /// Explicit Fock-basis preparation.
    Fock(FockState),

    /// Coherent state |alpha>.
    Coherent {
        /// Target mode.
        mode: BosonicModeId,

        /// Complex coherent amplitude represented by two real parameters.
        amplitude_real: Parameter,

        /// Imaginary component.
        amplitude_imaginary: Parameter,
    },

    /// Single-mode squeezed state.
    Squeezed {
        /// Target mode.
        mode: BosonicModeId,

        /// Squeezing magnitude.
        magnitude: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// Thermal state.
    Thermal {
        /// Target mode.
        mode: BosonicModeId,

        /// Mean occupation.
        mean_occupation: Parameter,
    },

    /// Vacuum preparation.
    Vacuum {
        /// Target mode.
        mode: BosonicModeId,
    },

    /// Extensible custom state preparation.
    Custom {
        /// Stable semantic operation name.
        name: String,

        /// Modes affected by the preparation.
        modes: Vec<BosonicModeId>,

        /// Optional parameters.
        parameters: Vec<Parameter>,
    },
}

impl BosonicStatePreparation {
    /// Returns the modes affected by this preparation.
    #[must_use]
    pub fn modes(&self) -> Vec<BosonicModeId> {
        match self {
            Self::Fock(state) => state
                .assignments()
                .iter()
                .map(|(mode, _)| *mode)
                .collect(),

            Self::Coherent { mode, .. }
            | Self::Squeezed { mode, .. }
            | Self::Thermal { mode, .. }
            | Self::Vacuum { mode } => vec![*mode],

            Self::Custom { modes, .. } => modes.clone(),
        }
    }

    /// Validates the state-preparation semantics.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        match self {
            Self::Fock(state) => state.validate(),

            Self::Coherent {
                amplitude_real,
                amplitude_imaginary,
                ..
            } => {
                validate_parameter(amplitude_real)?;
                validate_parameter(amplitude_imaginary)?;
                Ok(())
            }

            Self::Squeezed {
                magnitude,
                phase,
                ..
            } => {
                validate_parameter(magnitude)?;
                validate_parameter(phase)?;
                Ok(())
            }

            Self::Thermal {
                mean_occupation,
                ..
            } => validate_parameter(mean_occupation),

            Self::Vacuum { .. } => Ok(()),

            Self::Custom {
                name,
                modes,
                parameters,
            } => {
                validate_name(name, "bosonic state preparation")?;
                validate_unique_modes(modes)?;

                for parameter in parameters {
                    validate_parameter(parameter)?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Bosonic operation
// =============================================================================

/// Hardware-independent bosonic operation.
///
/// This is the central semantic operation vocabulary for this model.
///
/// Standard operations are represented explicitly while [`Custom`] provides
/// controlled extensibility for new architectures.
#[derive(Debug, Clone, PartialEq)]
pub enum BosonicOperation {
    /// Prepare one or more modes.
    Prepare(BosonicStatePreparation),

    /// Displacement D(alpha).
    Displacement {
        /// Target mode.
        mode: BosonicModeId,

        /// Real displacement component.
        real: Parameter,

        /// Imaginary displacement component.
        imaginary: Parameter,
    },

    /// Phase-space rotation R(phi).
    PhaseRotation {
        /// Target mode.
        mode: BosonicModeId,

        /// Rotation angle.
        angle: Parameter,
    },

    /// Single-mode squeezing S(z).
    Squeezing {
        /// Target mode.
        mode: BosonicModeId,

        /// Squeezing magnitude.
        magnitude: Parameter,

        /// Squeezing phase.
        phase: Parameter,
    },

    /// Two-mode beam splitter.
    BeamSplitter {
        /// First mode.
        first: BosonicModeId,

        /// Second mode.
        second: BosonicModeId,

        /// Mixing angle.
        angle: Parameter,

        /// Relative phase.
        phase: Parameter,
    },

    /// Phase shifter.
    PhaseShifter {
        /// Target mode.
        mode: BosonicModeId,

        /// Phase.
        phase: Parameter,
    },

    /// Arbitrary multimode linear optical transformation.
    ///
    /// The matrix is represented symbolically as a rectangular parameter
    /// array. No fixed matrix size is assumed.
    Interferometer {
        /// Ordered input/output mode list.
        modes: Vec<BosonicModeId>,

        /// Row-major transformation parameters.
        ///
        /// The expected semantic dimension is `modes.len() × modes.len()`.
        /// Validation checks the number of supplied elements without imposing
        /// a fixed maximum.
        matrix: Vec<Parameter>,
    },

    /// Apply an arbitrary operator generated by an operator expression.
    OperatorEvolution {
        /// Operator generator.
        generator: BosonicOperatorExpression,

        /// Evolution parameter.
        parameter: Parameter,
    },

    /// Measurement operation.
    Measure(BosonicMeasurement),

    /// Conditional operation.
    ///
    /// The condition itself remains an opaque semantic expression at this
    /// model boundary; classical control semantics are owned by the canonical
    /// IR control-flow subsystem.
    Conditional {
        /// Opaque condition identifier/name.
        condition: String,

        /// Nested operation.
        operation: Box<BosonicOperation>,
    },

    /// Extensible operation.
    Custom {
        /// Stable namespaced semantic operation name.
        name: String,

        /// Affected modes.
        modes: Vec<BosonicModeId>,

        /// Symbolic/numeric operation parameters.
        parameters: Vec<Parameter>,
    },
}

impl BosonicOperation {
    /// Returns all modes referenced by the operation.
    #[must_use]
    pub fn modes(&self) -> Vec<BosonicModeId> {
        match self {
            Self::Prepare(preparation) => preparation.modes(),

            Self::Displacement { mode, .. }
            | Self::PhaseRotation { mode, .. }
            | Self::Squeezing { mode, .. }
            | Self::PhaseShifter { mode, .. } => vec![*mode],

            Self::BeamSplitter {
                first, second, ..
            } => vec![*first, *second],

            Self::Interferometer { modes, .. } => modes.clone(),

            Self::OperatorEvolution { generator, .. } => {
                generator.referenced_modes()
            }

            Self::Measure(measurement) => measurement.modes(),

            Self::Conditional { operation, .. } => operation.modes(),

            Self::Custom { modes, .. } => modes.clone(),
        }
    }

    /// Returns whether this operation contains a symbolic parameter.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Prepare(preparation) => {
                preparation_is_symbolic(preparation)
            }

            Self::Displacement {
                real,
                imaginary,
                ..
            } => real.is_symbolic() || imaginary.is_symbolic(),

            Self::PhaseRotation { angle, .. }
            | Self::PhaseShifter { phase: angle, .. } => {
                angle.is_symbolic()
            }

            Self::Squeezing {
                magnitude, phase, ..
            } => magnitude.is_symbolic() || phase.is_symbolic(),

            Self::BeamSplitter { angle, phase, .. } => {
                angle.is_symbolic() || phase.is_symbolic()
            }

            Self::Interferometer { matrix, .. } => {
                matrix.iter().any(Parameter::is_symbolic)
            }

            Self::OperatorEvolution {
                generator,
                parameter,
            } => {
                parameter.is_symbolic()
                    || generator
                        .terms()
                        .iter()
                        .any(|term| term.coefficient().is_symbolic())
            }

            Self::Measure(measurement) => {
                measurement_is_symbolic(measurement)
            }

            Self::Conditional { operation, .. } => {
                operation.is_symbolic()
            }

            Self::Custom { parameters, .. } => {
                parameters.iter().any(Parameter::is_symbolic)
            }
        }
    }

    /// Validates the operation.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        match self {
            Self::Prepare(preparation) => preparation.validate(),

            Self::Displacement {
                real,
                imaginary,
                ..
            } => {
                validate_parameter(real)?;
                validate_parameter(imaginary)?;
                Ok(())
            }

            Self::PhaseRotation { angle, .. }
            | Self::PhaseShifter { phase: angle, .. } => {
                validate_parameter(angle)
            }

            Self::Squeezing {
                magnitude, phase, ..
            } => {
                validate_parameter(magnitude)?;
                validate_parameter(phase)?;
                Ok(())
            }

            Self::BeamSplitter {
                first,
                second,
                angle,
                phase,
            } => {
                validate_distinct_modes(*first, *second)?;
                validate_parameter(angle)?;
                validate_parameter(phase)?;
                Ok(())
            }

            Self::Interferometer { modes, matrix } => {
                validate_unique_modes(modes)?;

                let mode_count = modes.len();

                let expected = mode_count
                    .checked_mul(mode_count)
                    .ok_or(BosonicModelError::MatrixDimensionOverflow)?;

                if matrix.len() != expected {
                    return Err(
                        BosonicModelError::InvalidMatrixDimension {
                            modes: mode_count,
                            elements: matrix.len(),
                        },
                    );
                }

                for parameter in matrix {
                    validate_parameter(parameter)?;
                }

                Ok(())
            }

            Self::OperatorEvolution {
                generator,
                parameter,
            } => {
                generator.validate()?;
                validate_parameter(parameter)
            }

            Self::Measure(measurement) => measurement.validate(),

            Self::Conditional {
                condition,
                operation,
            } => {
                validate_name(condition, "conditional expression")?;
                operation.validate()
            }

            Self::Custom {
                name,
                modes,
                parameters,
            } => {
                validate_name(name, "bosonic operation")?;
                validate_unique_modes(modes)?;

                for parameter in parameters {
                    validate_parameter(parameter)?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Measurement
// =============================================================================

/// Bosonic measurement semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum BosonicMeasurement {
    /// Photon-number/Fock measurement.
    PhotonNumber {
        /// Modes measured.
        modes: Vec<BosonicModeId>,
    },

    /// Homodyne measurement.
    Homodyne {
        /// Target mode.
        mode: BosonicModeId,

        /// Local-oscillator phase.
        phase: Parameter,
    },

    /// Heterodyne measurement.
    Heterodyne {
        /// Target mode.
        mode: BosonicModeId,
    },

    /// Quadrature measurement.
    Quadrature {
        /// Target mode.
        mode: BosonicModeId,

        /// Quadrature angle.
        angle: Parameter,
    },

    /// Parity measurement.
    Parity {
        /// Target mode.
        mode: BosonicModeId,
    },

    /// General observable measurement.
    Observable {
        /// Observable expression.
        observable: BosonicOperatorExpression,

        /// Modes involved in the observable.
        modes: Vec<BosonicModeId>,
    },

    /// Extensible measurement semantics.
    Custom {
        /// Stable namespaced measurement name.
        name: String,

        /// Modes measured.
        modes: Vec<BosonicModeId>,

        /// Symbolic/numeric parameters.
        parameters: Vec<Parameter>,
    },
}

impl BosonicMeasurement {
    /// Returns the modes measured/referenced by this measurement.
    #[must_use]
    pub fn modes(&self) -> Vec<BosonicModeId> {
        match self {
            Self::PhotonNumber { modes }
            | Self::Observable { modes, .. }
            | Self::Custom { modes, .. } => modes.clone(),

            Self::Homodyne { mode, .. }
            | Self::Heterodyne { mode }
            | Self::Quadrature { mode, .. }
            | Self::Parity { mode } => vec![*mode],
        }
    }

    /// Validates the measurement.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        match self {
            Self::PhotonNumber { modes } => {
                validate_unique_modes(modes)?;

                if modes.is_empty() {
                    return Err(
                        BosonicModelError::EmptyModeSet {
                            context: "photon-number measurement",
                        },
                    );
                }

                Ok(())
            }

            Self::Homodyne { phase, .. }
            | Self::Quadrature { angle: phase, .. } => {
                validate_parameter(phase)
            }

            Self::Heterodyne { .. }
            | Self::Parity { .. } => Ok(()),

            Self::Observable {
                observable,
                modes,
            } => {
                validate_unique_modes(modes)?;
                observable.validate()?;

                let referenced =
                    observable.referenced_modes();

                for mode in referenced {
                    if !modes.contains(&mode) {
                        return Err(
                            BosonicModelError::ObservableModeNotDeclared {
                                mode,
                            },
                        );
                    }
                }

                Ok(())
            }

            Self::Custom {
                name,
                modes,
                parameters,
            } => {
                validate_name(name, "bosonic measurement")?;
                validate_unique_modes(modes)?;

                for parameter in parameters {
                    validate_parameter(parameter)?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Bosonic program
// =============================================================================

/// Complete bosonic semantic program.
///
/// This is the primary model-level container.
///
/// It intentionally does not contain:
///
/// - hardware;
/// - physical topology;
/// - scheduler state;
/// - backend handles;
/// - simulator state.
#[derive(Debug, Clone, PartialEq)]
pub struct BosonicProgram {
    modes: BosonicModes,
    operations: Vec<BosonicOperation>,
}

impl BosonicProgram {
    /// Creates an empty bosonic program.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            modes: BosonicModes::new(),
            operations: Vec::new(),
        }
    }

    /// Creates a program from explicit modes and operations.
    pub fn from_parts(
        modes: BosonicModes,
        operations: Vec<BosonicOperation>,
    ) -> Result<Self, BosonicModelError> {
        let program = Self {
            modes,
            operations,
        };

        program.validate()?;

        Ok(program)
    }

    /// Adds a semantic mode declaration.
    pub fn add_mode(
        &mut self,
        mode: BosonicModeId,
    ) -> Result<(), BosonicModelError> {
        self.modes.push(mode)
    }

    /// Adds a semantic operation.
    ///
    /// The operation is validated before being inserted.
    pub fn push(
        &mut self,
        operation: BosonicOperation,
    ) -> Result<(), BosonicModelError> {
        operation.validate()?;

        for mode in operation.modes() {
            if !self.modes.contains(mode) {
                return Err(
                    BosonicModelError::UndeclaredMode(mode),
                );
            }
        }

        self.operations.push(operation);
        Ok(())
    }

    /// Returns the declared modes.
    #[must_use]
    pub fn modes(&self) -> &BosonicModes {
        &self.modes
    }

    /// Returns operations in semantic program order.
    #[must_use]
    pub fn operations(&self) -> &[BosonicOperation] {
        &self.operations
    }

    /// Returns the number of declared modes.
    #[must_use]
    pub fn mode_count(&self) -> usize {
        self.modes.len()
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether any operation contains symbolic parameters.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.operations
            .iter()
            .any(BosonicOperation::is_symbolic)
    }

    /// Returns all modes actually referenced by operations.
    #[must_use]
    pub fn referenced_modes(&self) -> Vec<BosonicModeId> {
        let mut modes = BTreeSet::new();

        for operation in &self.operations {
            for mode in operation.modes() {
                modes.insert(mode);
            }
        }

        modes.into_iter().collect()
    }

    /// Validates the complete semantic program.
    pub fn validate(&self) -> Result<(), BosonicModelError> {
        for operation in &self.operations {
            operation.validate()?;

            for mode in operation.modes() {
                if !self.modes.contains(mode) {
                    return Err(
                        BosonicModelError::UndeclaredMode(mode),
                    );
                }
            }
        }

        Ok(())
    }
}

impl Default for BosonicProgram {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Canonical errors produced by this bosonic semantic model.
///
/// These errors describe malformed IR, not hardware execution failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BosonicModelError {
    /// A mode appears more than once in one semantic collection.
    DuplicateMode(BosonicModeId),

    /// A referenced mode was not declared in the owning program.
    UndeclaredMode(BosonicModeId),

    /// Two operands that must be distinct refer to the same mode.
    NonDistinctModes(BosonicModeId),

    /// A required mode collection is empty.
    EmptyModeSet {
        /// Semantic context.
        context: &'static str,
    },

    /// A custom name is invalid.
    InvalidName {
        /// Semantic namespace.
        kind: &'static str,

        /// Invalid name.
        name: String,
    },

    /// A canonical parameter could not be validated.
    InvalidParameter(String),

    /// A matrix dimension calculation overflowed the host integer type.
    MatrixDimensionOverflow,

    /// A supplied matrix has an invalid number of elements.
    InvalidMatrixDimension {
        /// Number of modes.
        modes: usize,

        /// Supplied element count.
        elements: usize,
    },

    /// A mode referenced by an observable was omitted from its mode list.
    ObservableModeNotDeclared {
        /// Missing mode.
        mode: BosonicModeId,
    },

    /// Identifier arithmetic overflowed.
    IdentifierOverflow,
}

impl fmt::Display for BosonicModelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateMode(mode) => {
                write!(formatter, "duplicate bosonic mode {mode}")
            }

            Self::UndeclaredMode(mode) => {
                write!(formatter, "undeclared bosonic mode {mode}")
            }

            Self::NonDistinctModes(mode) => {
                write!(
                    formatter,
                    "operation requires distinct modes; received {mode} twice"
                )
            }

            Self::EmptyModeSet { context } => {
                write!(
                    formatter,
                    "{context} requires at least one mode"
                )
            }

            Self::InvalidName { kind, name } => {
                write!(
                    formatter,
                    "invalid {kind} name `{name}`"
                )
            }

            Self::InvalidParameter(message) => {
                write!(
                    formatter,
                    "invalid bosonic parameter: {message}"
                )
            }

            Self::MatrixDimensionOverflow => {
                formatter.write_str(
                    "bosonic matrix dimension calculation overflowed",
                )
            }

            Self::InvalidMatrixDimension {
                modes,
                elements,
            } => {
                write!(
                    formatter,
                    "invalid bosonic transformation matrix: \
                     {modes} modes require {modes} × {modes} elements, \
                     but {elements} elements were supplied"
                )
            }

            Self::ObservableModeNotDeclared { mode } => {
                write!(
                    formatter,
                    "observable references mode {mode} that is not \
                     declared in the observable mode set"
                )
            }

            Self::IdentifierOverflow => {
                formatter.write_str(
                    "bosonic identifier arithmetic overflowed",
                )
            }
        }
    }
}

impl std::error::Error for BosonicModelError {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_name(
    name: &str,
    kind: &'static str,
) -> Result<(), BosonicModelError> {
    if name.is_empty()
        || !name
            .chars()
            .all(|character| {
                character.is_alphanumeric()
                    || character == '_'
                    || character == '.'
                    || character == ':'
                    || character == '-'
            })
    {
        return Err(BosonicModelError::InvalidName {
            kind,
            name: name.to_owned(),
        });
    }

    Ok(())
}

fn validate_parameter(
    parameter: &Parameter,
) -> Result<(), BosonicModelError> {
    parameter
        .validate()
        .map_err(|error| {
            BosonicModelError::InvalidParameter(
                error.to_string(),
            )
        })
}

fn validate_unique_modes(
    modes: &[BosonicModeId],
) -> Result<(), BosonicModelError> {
    let mut seen = BTreeSet::new();

    for mode in modes {
        if !seen.insert(*mode) {
            return Err(BosonicModelError::DuplicateMode(*mode));
        }
    }

    Ok(())
}

fn validate_distinct_modes(
    first: BosonicModeId,
    second: BosonicModeId,
) -> Result<(), BosonicModelError> {
    if first == second {
        return Err(BosonicModelError::NonDistinctModes(first));
    }

    Ok(())
}

fn preparation_is_symbolic(
    preparation: &BosonicStatePreparation,
) -> bool {
    match preparation {
        BosonicStatePreparation::Fock(_) => false,

        BosonicStatePreparation::Coherent {
            amplitude_real,
            amplitude_imaginary,
            ..
        } => {
            amplitude_real.is_symbolic()
                || amplitude_imaginary.is_symbolic()
        }

        BosonicStatePreparation::Squeezed {
            magnitude,
            phase,
            ..
        } => magnitude.is_symbolic() || phase.is_symbolic(),

        BosonicStatePreparation::Thermal {
            mean_occupation,
            ..
        } => mean_occupation.is_symbolic(),

        BosonicStatePreparation::Vacuum { .. } => false,

        BosonicStatePreparation::Custom {
            parameters, ..
        } => parameters.iter().any(Parameter::is_symbolic),
    }
}

fn measurement_is_symbolic(
    measurement: &BosonicMeasurement,
) -> bool {
    match measurement {
        BosonicMeasurement::PhotonNumber { .. }
        | BosonicMeasurement::Heterodyne { .. }
        | BosonicMeasurement::Parity { .. } => false,

        BosonicMeasurement::Homodyne { phase, .. }
        | BosonicMeasurement::Quadrature { angle: phase, .. } => {
            phase.is_symbolic()
        }

        BosonicMeasurement::Observable { observable, .. } => {
            observable
                .terms()
                .iter()
                .any(|term| term.coefficient().is_symbolic())
        }

        BosonicMeasurement::Custom {
            parameters, ..
        } => parameters.iter().any(Parameter::is_symbolic),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("test parameter must be finite")
    }

    #[test]
    fn mode_identity_is_stable() {
        let first = BosonicModeId::new(42);
        let second = BosonicModeId::new(42);

        assert_eq!(first, second);
        assert_eq!(first.index(), 42);
        assert_eq!(first.to_string(), "m42");
    }

    #[test]
    fn mode_collection_preserves_declaration_order() {
        let modes = BosonicModes::from_ids([
            BosonicModeId::new(7),
            BosonicModeId::new(2),
            BosonicModeId::new(9),
        ])
        .expect("unique modes");

        assert_eq!(
            modes.as_slice(),
            &[
                BosonicModeId::new(7),
                BosonicModeId::new(2),
                BosonicModeId::new(9)
            ]
        );
    }

    #[test]
    fn duplicate_modes_are_rejected() {
        let result = BosonicModes::from_ids([
            BosonicModeId::new(1),
            BosonicModeId::new(1),
        ]);

        assert!(matches!(
            result,
            Err(BosonicModelError::DuplicateMode(
                BosonicModeId(1)
            ))
        ));
    }

    #[test]
    fn fock_state_supports_arbitrary_finite_occupations() {
        let state = FockState::from_assignments([
            (
                BosonicModeId::new(0),
                FockOccupation::new(0),
            ),
            (
                BosonicModeId::new(1),
                FockOccupation::new(1_000_000),
            ),
            (
                BosonicModeId::new(2),
                FockOccupation::new(u64::MAX),
            ),
        ])
        .expect("valid assignments");

        assert_eq!(
            state.get(BosonicModeId::new(1)),
            Some(FockOccupation::new(1_000_000))
        );

        assert_eq!(
            state.get(BosonicModeId::new(2)),
            Some(FockOccupation::new(u64::MAX))
        );
    }

    #[test]
    fn operator_expression_collects_modes() {
        let first = BosonicOperator::new(
            BosonicModeId::new(0),
            BosonicOperatorKind::Creation,
        );

        let second = BosonicOperator::new(
            BosonicModeId::new(7),
            BosonicOperatorKind::Annihilation,
        );

        let product =
            BosonicOperatorProduct::from_factors([
                first, second,
            ])
            .expect("valid product");

        let term =
            BosonicOperatorTerm::new(constant(1.0), product)
                .expect("valid term");

        let expression =
            BosonicOperatorExpression::from_terms([term])
                .expect("valid expression");

        assert_eq!(
            expression.referenced_modes(),
            vec![
                BosonicModeId::new(0),
                BosonicModeId::new(7)
            ]
        );
    }

    #[test]
    fn beam_splitter_rejects_same_mode() {
        let operation = BosonicOperation::BeamSplitter {
            first: BosonicModeId::new(0),
            second: BosonicModeId::new(0),
            angle: constant(0.5),
            phase: constant(0.0),
        };

        assert!(matches!(
            operation.validate(),
            Err(BosonicModelError::NonDistinctModes(
                BosonicModeId(0)
            ))
        ));
    }

    #[test]
    fn interferometer_is_not_hard_coded_to_a_size() {
        let modes = vec![
            BosonicModeId::new(0),
            BosonicModeId::new(1),
            BosonicModeId::new(2),
        ];

        let matrix = (0..9)
            .map(|_| constant(0.0))
            .collect::<Vec<_>>();

        let operation =
            BosonicOperation::Interferometer { modes, matrix };

        assert!(operation.validate().is_ok());
    }

    #[test]
    fn invalid_interferometer_dimension_is_rejected() {
        let modes = vec![
            BosonicModeId::new(0),
            BosonicModeId::new(1),
        ];

        let matrix = vec![
            constant(1.0),
            constant(0.0),
            constant(0.0),
        ];

        let operation =
            BosonicOperation::Interferometer { modes, matrix };

        assert!(matches!(
            operation.validate(),
            Err(
                BosonicModelError::InvalidMatrixDimension {
                    modes: 2,
                    elements: 3
                }
            )
        ));
    }

    #[test]
    fn program_rejects_undeclared_modes() {
        let mut program = BosonicProgram::new();

        program
            .add_mode(BosonicModeId::new(0))
            .expect("mode insertion");

        let operation = BosonicOperation::Displacement {
            mode: BosonicModeId::new(1),
            real: constant(0.0),
            imaginary: constant(0.0),
        };

        assert!(matches!(
            program.push(operation),
            Err(BosonicModelError::UndeclaredMode(
                BosonicModeId(1)
            ))
        ));
    }

    #[test]
    fn program_accepts_symbolic_operations() {
        let mut program = BosonicProgram::new();

        program
            .add_mode(BosonicModeId::new(0))
            .expect("mode insertion");

        let theta =
            Parameter::symbol("theta")
                .expect("valid symbol");

        program
            .push(BosonicOperation::PhaseRotation {
                mode: BosonicModeId::new(0),
                angle: theta,
            })
            .expect("valid operation");

        assert!(program.is_symbolic());
        assert_eq!(program.operation_count(), 1);
    }

    #[test]
    fn observable_requires_declared_modes() {
        let operator = BosonicOperator::new(
            BosonicModeId::new(3),
            BosonicOperatorKind::Number,
        );

        let product =
            BosonicOperatorProduct::from_factors([operator])
                .expect("valid product");

        let term =
            BosonicOperatorTerm::new(constant(1.0), product)
                .expect("valid term");

        let observable =
            BosonicOperatorExpression::from_terms([term])
                .expect("valid observable");

        let measurement = BosonicMeasurement::Observable {
            observable,
            modes: vec![BosonicModeId::new(0)],
        };

        assert!(matches!(
            measurement.validate(),
            Err(
                BosonicModelError::ObservableModeNotDeclared {
                    mode: BosonicModeId(3)
                }
            )
        ));
    }

    #[test]
    fn custom_operations_are_extensible() {
        let operation = BosonicOperation::Custom {
            name: "zamani.future.bosonic_operation".to_owned(),
            modes: vec![
                BosonicModeId::new(0),
                BosonicModeId::new(1),
            ],
            parameters: vec![constant(1.0)],
        };

        assert!(operation.validate().is_ok());
    }

    #[test]
    fn custom_operator_names_are_extensible() {
        let operator =
            BosonicOperatorKind::custom(
                "vendor.example.non_gaussian",
            )
            .expect("valid custom operator");

        assert!(operator.is_custom());
        assert_eq!(
            operator.name(),
            "vendor.example.non_gaussian"
        );
    }

    #[test]
    fn vacuum_preparation_has_no_symbolic_parameters() {
        let preparation =
            BosonicStatePreparation::Vacuum {
                mode: BosonicModeId::new(0),
            };

        assert!(preparation.validate().is_ok());
        assert!(!preparation_is_symbolic(&preparation));
    }

    #[test]
    fn large_mode_identifier_does_not_use_usize() {
        let mode = BosonicModeId::new(u64::MAX);

        assert_eq!(mode.index(), u64::MAX);
        assert_eq!(mode.checked_next(), None);
    }

    #[test]
    fn zero_parameter_operator_is_valid() {
        let product =
            BosonicOperatorProduct::identity();

        let term =
            BosonicOperatorTerm::new(
                constant(1.0),
                product,
            )
            .expect("identity term");

        let expression =
            BosonicOperatorExpression::from_terms([term])
                .expect("valid expression");

        assert!(expression.validate().is_ok());
        assert!(expression.referenced_modes().is_empty());
    }
}