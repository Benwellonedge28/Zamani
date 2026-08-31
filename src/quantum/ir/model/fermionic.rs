//! Zamani Quantum IR — Fermionic Model
//!
//! Canonical, hardware-independent representation of fermionic quantum
//! computation and fermionic operators.
//!
//! # Architectural role
//!
//! This module represents the *meaning* of fermionic computation.
//!
//! It does NOT perform:
//!
//! - Jordan-Wigner transformation;
//! - Bravyi-Kitaev transformation;
//! - parity mapping;
//! - qubit routing;
//! - qubit placement;
//! - circuit synthesis;
//! - pulse generation;
//! - simulation;
//! - numerical diagonalization;
//! - eigensolver execution;
//! - hardware allocation;
//! - backend execution.
//!
//! Those responsibilities belong to downstream compilation, simulation,
//! optimization, routing and backend subsystems.
//!
//! # Semantic model
//!
//! ```text
//! FermionicProgram / Hamiltonian
//!          │
//!          ├── Fermionic modes
//!          │
//!          ├── Fermionic operators
//!          │      ├── creation
//!          │      └── annihilation
//!          │
//!          ├── Fermionic terms
//!          │
//!          └── coefficients
//!
//!                    │
//!                    ▼
//!             encoding / lowering
//!                    │
//!                    ▼
//!              QubitId mapping
//!                    │
//!                    ▼
//!               qubit-level IR
//! ```
//!
//! `QubitId` therefore appears only at the explicit fermion-to-qubit mapping
//! boundary.
//!
//! # Scaling
//!
//! No architectural maximum is imposed on:
//!
//! - number of fermionic modes;
//! - number of terms;
//! - number of factors per term;
//! - number of Hamiltonian terms;
//! - number of symbolic parameters.
//!
//! Concrete compiler/security limits must be supplied by the caller or by
//! `QuantumIrLimits`.
//!
//! The implementation deliberately avoids fixed-size arrays and fixed
//! machine-specific limits.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` enforces the no-unsafe requirement.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::parameter::Parameter;
use super::super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type used by the fermionic IR model.
pub type FermionicResult<T> = Result<T, FermionicError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors local to the fermionic IR model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FermionicError {
    /// A fermionic mode identifier is invalid.
    InvalidModeId,

    /// A mode index overflowed.
    ModeIndexOverflow,

    /// A term contains no operators where at least one is required.
    EmptyTerm,

    /// A fermionic term contains an invalid factor.
    InvalidFactor,

    /// A term contains duplicate mode metadata where uniqueness is required.
    DuplicateMode {
        mode: FermionicModeId,
    },

    /// A qubit encoding maps one fermionic mode more than once.
    DuplicateModeMapping {
        mode: FermionicModeId,
    },

    /// A qubit encoding maps one qubit to more than one fermionic mode.
    DuplicateQubitMapping {
        qubit: QubitId,
    },

    /// A mapping is missing a mode.
    MissingModeMapping {
        mode: FermionicModeId,
    },

    /// A coefficient is structurally invalid.
    InvalidCoefficient,

    /// A symbolic parameter failed validation.
    ParameterError,

    /// A requested operation cannot be performed without violating an
    /// invariant.
    InvalidOperation {
        message: &'static str,
    },

    /// A term or Hamiltonian exceeds an explicitly supplied caller policy.
    ResourceLimit {
        resource: &'static str,
    },
}

impl fmt::Display for FermionicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModeId => {
                f.write_str("invalid fermionic mode identifier")
            }
            Self::ModeIndexOverflow => {
                f.write_str("fermionic mode identifier overflow")
            }
            Self::EmptyTerm => {
                f.write_str("fermionic term cannot be empty")
            }
            Self::InvalidFactor => {
                f.write_str("invalid fermionic operator factor")
            }
            Self::DuplicateMode { mode } => {
                write!(f, "duplicate fermionic mode {mode}")
            }
            Self::DuplicateModeMapping { mode } => {
                write!(f, "fermionic mode {mode} is mapped more than once")
            }
            Self::DuplicateQubitMapping { qubit } => {
                write!(f, "qubit {qubit} is mapped from more than one mode")
            }
            Self::MissingModeMapping { mode } => {
                write!(f, "fermionic mode {mode} has no qubit mapping")
            }
            Self::InvalidCoefficient => {
                f.write_str("invalid fermionic coefficient")
            }
            Self::ParameterError => {
                f.write_str("invalid fermionic parameter")
            }
            Self::InvalidOperation { message } => {
                write!(f, "invalid fermionic operation: {message}")
            }
            Self::ResourceLimit { resource } => {
                write!(f, "fermionic resource limit exceeded: {resource}")
            }
        }
    }
}

impl std::error::Error for FermionicError {}

// =============================================================================
// Fermionic mode identity
// =============================================================================

/// Stable semantic identity of a fermionic mode.
///
/// A fermionic mode is NOT a qubit.
///
/// Examples include:
///
/// - spin-orbitals;
/// - lattice fermion sites;
/// - molecular orbitals;
/// - momentum modes;
/// - abstract fermionic degrees of freedom.
///
/// `FermionicModeId` deliberately has its own type so a fermionic mode cannot
/// accidentally be confused with `QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FermionicModeId(u64);

impl FermionicModeId {
    /// Creates a mode identifier.
    ///
    /// The numeric value is an identity token, not a machine-size limit.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next identity when representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for FermionicModeId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<FermionicModeId> for u64 {
    fn from(value: FermionicModeId) -> Self {
        value.value()
    }
}

impl fmt::Display for FermionicModeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.0)
    }
}

// =============================================================================
// Spin
// =============================================================================

/// Spin projection associated with a fermionic mode.
///
/// This metadata is optional. A fermionic computation does not have to use
/// spin-orbitals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpinProjection {
    /// Spin-up / alpha convention.
    Alpha,

    /// Spin-down / beta convention.
    Beta,

    /// Spin is intentionally unspecified.
    Unspecified,
}

// =============================================================================
// Fermionic mode metadata
// =============================================================================

/// Semantic metadata describing a fermionic mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FermionicMode {
    id: FermionicModeId,
    spin: SpinProjection,
    orbital: Option<u64>,
    site: Option<u64>,
}

impl FermionicMode {
    /// Creates a mode with no optional physical/model metadata.
    #[must_use]
    pub const fn new(id: FermionicModeId) -> Self {
        Self {
            id,
            spin: SpinProjection::Unspecified,
            orbital: None,
            site: None,
        }
    }

    /// Creates a mode with spin metadata.
    #[must_use]
    pub const fn with_spin(
        id: FermionicModeId,
        spin: SpinProjection,
    ) -> Self {
        Self {
            id,
            spin,
            orbital: None,
            site: None,
        }
    }

    /// Returns the mode identity.
    #[must_use]
    pub const fn id(&self) -> FermionicModeId {
        self.id
    }

    /// Returns the spin metadata.
    #[must_use]
    pub const fn spin(&self) -> SpinProjection {
        self.spin
    }

    /// Returns the optional orbital index.
    #[must_use]
    pub const fn orbital(&self) -> Option<u64> {
        self.orbital
    }

    /// Returns the optional lattice/site index.
    #[must_use]
    pub const fn site(&self) -> Option<u64> {
        self.site
    }

    /// Returns a copy with orbital metadata.
    #[must_use]
    pub const fn with_orbital(mut self, orbital: u64) -> Self {
        self.orbital = Some(orbital);
        self
    }

    /// Returns a copy with site metadata.
    #[must_use]
    pub const fn with_site(mut self, site: u64) -> Self {
        self.site = Some(site);
        self
    }
}

// =============================================================================
// Fermionic operator kind
// =============================================================================

/// One canonical fermionic ladder operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FermionicOperatorKind {
    /// Creation operator a†.
    Creation,

    /// Annihilation operator a.
    Annihilation,
}

impl FermionicOperatorKind {
    /// Returns whether this is a creation operator.
    #[must_use]
    pub const fn is_creation(self) -> bool {
        matches!(self, Self::Creation)
    }

    /// Returns whether this is an annihilation operator.
    #[must_use]
    pub const fn is_annihilation(self) -> bool {
        matches!(self, Self::Annihilation)
    }

    /// Returns the fermionic parity contribution.
    #[must_use]
    pub const fn parity(self) -> i8 {
        match self {
            Self::Creation | Self::Annihilation => 1,
        }
    }
}

impl fmt::Display for FermionicOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creation => f.write_str("†"),
            Self::Annihilation => f.write_str(""),
        }
    }
}

// =============================================================================
// Fermionic operator
// =============================================================================

/// A single fermionic ladder operator acting on one mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FermionicOperator {
    mode: FermionicModeId,
    kind: FermionicOperatorKind,
}

impl FermionicOperator {
    /// Creates a creation operator.
    #[must_use]
    pub const fn creation(mode: FermionicModeId) -> Self {
        Self {
            mode,
            kind: FermionicOperatorKind::Creation,
        }
    }

    /// Creates an annihilation operator.
    #[must_use]
    pub const fn annihilation(mode: FermionicModeId) -> Self {
        Self {
            mode,
            kind: FermionicOperatorKind::Annihilation,
        }
    }

    /// Creates an operator from its kind.
    #[must_use]
    pub const fn new(
        mode: FermionicModeId,
        kind: FermionicOperatorKind,
    ) -> Self {
        Self { mode, kind }
    }

    /// Returns the mode.
    #[must_use]
    pub const fn mode(self) -> FermionicModeId {
        self.mode
    }

    /// Returns the operator kind.
    #[must_use]
    pub const fn kind(self) -> FermionicOperatorKind {
        self.kind
    }

    /// Returns whether this is creation.
    #[must_use]
    pub const fn is_creation(self) -> bool {
        self.kind.is_creation()
    }

    /// Returns whether this is annihilation.
    #[must_use]
    pub const fn is_annihilation(self) -> bool {
        self.kind.is_annihilation()
    }
}

impl fmt::Display for FermionicOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            FermionicOperatorKind::Creation => {
                write!(f, "a†_{}", self.mode.value())
            }
            FermionicOperatorKind::Annihilation => {
                write!(f, "a_{}", self.mode.value())
            }
        }
    }
}

// =============================================================================
// Coefficient
// =============================================================================

/// Scalar coefficient of a fermionic term.
///
/// The coefficient may remain symbolic until a downstream compilation or
/// simulation stage binds it.
///
/// Complex coefficients are represented explicitly as real and imaginary
/// parameters rather than by prematurely evaluating them.
#[derive(Debug, Clone, PartialEq)]
pub enum FermionicCoefficient {
    /// Real scalar coefficient.
    Real(Parameter),

    /// Complex scalar coefficient.
    Complex {
        /// Real component.
        real: Parameter,

        /// Imaginary component.
        imaginary: Parameter,
    },
}

impl FermionicCoefficient {
    /// Creates a real coefficient from an existing parameter.
    pub fn real(parameter: Parameter) -> FermionicResult<Self> {
        parameter
            .validate()
            .map_err(|_| FermionicError::ParameterError)?;

        Ok(Self::Real(parameter))
    }

    /// Creates a complex coefficient.
    pub fn complex(
        real: Parameter,
        imaginary: Parameter,
    ) -> FermionicResult<Self> {
        real.validate()
            .map_err(|_| FermionicError::ParameterError)?;

        imaginary
            .validate()
            .map_err(|_| FermionicError::ParameterError)?;

        Ok(Self::Complex { real, imaginary })
    }

    /// Creates the exact real coefficient `1`.
    pub fn one() -> FermionicResult<Self> {
        Self::real(
            Parameter::constant(1.0)
                .map_err(|_| FermionicError::ParameterError)?,
        )
    }

    /// Creates the exact real coefficient `0`.
    pub fn zero() -> FermionicResult<Self> {
        Self::real(
            Parameter::constant(0.0)
                .map_err(|_| FermionicError::ParameterError)?,
        )
    }

    /// Returns whether the coefficient is real.
    #[must_use]
    pub const fn is_real(&self) -> bool {
        matches!(self, Self::Real(_))
    }

    /// Returns whether the coefficient is complex.
    #[must_use]
    pub const fn is_complex(&self) -> bool {
        matches!(self, Self::Complex { .. })
    }

    /// Validates the coefficient.
    pub fn validate(&self) -> FermionicResult<()> {
        match self {
            Self::Real(value) => value
                .validate()
                .map_err(|_| FermionicError::ParameterError),

            Self::Complex { real, imaginary } => {
                real.validate()
                    .map_err(|_| FermionicError::ParameterError)?;

                imaginary
                    .validate()
                    .map_err(|_| FermionicError::ParameterError)
            }
        }
    }
}

impl Default for FermionicCoefficient {
    fn default() -> Self {
        Self::Real(
            Parameter::constant(0.0)
                .expect("finite zero parameter must be valid"),
        )
    }
}

// =============================================================================
// Fermionic term
// =============================================================================

/// One fermionic monomial.
///
/// Mathematically:
///
/// ```text
/// c · a†_p a_q ...
/// ```
///
/// Operator order is semantically significant.
///
/// The constructor preserves caller order. No implicit Jordan-Wigner,
/// Bravyi-Kitaev, parity, or other qubit encoding is performed here.
#[derive(Debug, Clone, PartialEq)]
pub struct FermionicTerm {
    coefficient: FermionicCoefficient,
    operators: Vec<FermionicOperator>,
}

impl FermionicTerm {
    /// Creates a fermionic term.
    pub fn new(
        coefficient: FermionicCoefficient,
        operators: Vec<FermionicOperator>,
    ) -> FermionicResult<Self> {
        coefficient.validate()?;

        if operators.is_empty() {
            return Err(FermionicError::EmptyTerm);
        }

        Ok(Self {
            coefficient,
            operators,
        })
    }

    /// Creates a one-body creation/annihilation term.
    pub fn one_body(
        coefficient: FermionicCoefficient,
        creation: FermionicModeId,
        annihilation: FermionicModeId,
    ) -> FermionicResult<Self> {
        Self::new(
            coefficient,
            vec![
                FermionicOperator::creation(creation),
                FermionicOperator::annihilation(annihilation),
            ],
        )
    }

    /// Creates a number operator term:
    ///
    /// ```text
    /// a†_p a_p
    /// ```
    pub fn number_operator(
        coefficient: FermionicCoefficient,
        mode: FermionicModeId,
    ) -> FermionicResult<Self> {
        Self::new(
            coefficient,
            vec![
                FermionicOperator::creation(mode),
                FermionicOperator::annihilation(mode),
            ],
        )
    }

    /// Creates a two-body interaction term:
    ///
    /// ```text
    /// a†_p a†_q a_r a_s
    /// ```
    pub fn two_body(
        coefficient: FermionicCoefficient,
        p: FermionicModeId,
        q: FermionicModeId,
        r: FermionicModeId,
        s: FermionicModeId,
    ) -> FermionicResult<Self> {
        Self::new(
            coefficient,
            vec![
                FermionicOperator::creation(p),
                FermionicOperator::creation(q),
                FermionicOperator::annihilation(r),
                FermionicOperator::annihilation(s),
            ],
        )
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &FermionicCoefficient {
        &self.coefficient
    }

    /// Returns the ordered operator sequence.
    #[must_use]
    pub fn operators(&self) -> &[FermionicOperator] {
        &self.operators
    }

    /// Returns the number of ladder operators in this term.
    #[must_use]
    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    /// Returns the unique modes referenced by this term in deterministic order.
    #[must_use]
    pub fn modes(&self) -> Vec<FermionicModeId> {
        let mut modes = BTreeSet::new();

        for operator in &self.operators {
            modes.insert(operator.mode());
        }

        modes.into_iter().collect()
    }

    /// Returns whether this term is number-conserving.
    ///
    /// A term is number-conserving when it contains the same number of
    /// creation and annihilation operators.
    #[must_use]
    pub fn is_number_conserving(&self) -> bool {
        let mut creations = 0usize;
        let mut annihilations = 0usize;

        for operator in &self.operators {
            match operator.kind() {
                FermionicOperatorKind::Creation => {
                    creations = creations.saturating_add(1);
                }
                FermionicOperatorKind::Annihilation => {
                    annihilations = annihilations.saturating_add(1);
                }
            }
        }

        creations == annihilations
    }

    /// Returns the fermionic parity of this monomial.
    ///
    /// Fermionic parity is even when the number of ladder operators is even.
    #[must_use]
    pub fn parity(&self) -> u8 {
        (self.operators.len() & 1) as u8
    }

    /// Returns whether the term has even fermionic parity.
    #[must_use]
    pub fn is_even(&self) -> bool {
        self.parity() == 0
    }

    /// Validates local term invariants.
    pub fn validate(&self) -> FermionicResult<()> {
        self.coefficient.validate()?;

        if self.operators.is_empty() {
            return Err(FermionicError::EmptyTerm);
        }

        Ok(())
    }

    /// Returns a deterministic structural key for canonical ordering.
    ///
    /// This does not perform algebraic simplification.
    #[must_use]
    pub fn structural_key(&self) -> Vec<(u64, u8)> {
        self.operators
            .iter()
            .map(|operator| {
                let kind = match operator.kind() {
                    FermionicOperatorKind::Creation => 0,
                    FermionicOperatorKind::Annihilation => 1,
                };

                (operator.mode().value(), kind)
            })
            .collect()
    }
}

// =============================================================================
// Fermionic Hamiltonian
// =============================================================================

/// A fermionic operator expressed as a deterministic collection of terms.
///
/// This representation can express:
///
/// - molecular Hamiltonians;
/// - lattice models;
/// - Hubbard-like models;
/// - interacting fermion systems;
/// - number operators;
/// - arbitrary finite fermionic operator sums.
///
/// It does not require a particular qubit encoding.
#[derive(Debug, Clone, PartialEq)]
pub struct FermionicHamiltonian {
    modes: BTreeMap<FermionicModeId, FermionicMode>,
    terms: Vec<FermionicTerm>,
}

impl Default for FermionicHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

impl FermionicHamiltonian {
    /// Creates an empty fermionic Hamiltonian.
    #[must_use]
    pub fn new() -> Self {
        Self {
            modes: BTreeMap::new(),
            terms: Vec::new(),
        }
    }

    /// Registers a fermionic mode.
    ///
    /// Registration is explicit so validation can detect references to modes
    /// that were not declared by the owning program.
    pub fn add_mode(
        &mut self,
        mode: FermionicMode,
    ) -> FermionicResult<()> {
        if self.modes.contains_key(&mode.id()) {
            return Err(FermionicError::DuplicateMode { mode: mode.id() });
        }

        self.modes.insert(mode.id(), mode);
        Ok(())
    }

    /// Adds an already validated term.
    pub fn add_term(
        &mut self,
        term: FermionicTerm,
    ) -> FermionicResult<()> {
        term.validate()?;

        for mode in term.modes() {
            if !self.modes.contains_key(&mode) {
                return Err(FermionicError::InvalidOperation {
                    message: "term references an undeclared fermionic mode",
                });
            }
        }

        self.terms.push(term);
        Ok(())
    }

    /// Returns all declared modes in deterministic order.
    #[must_use]
    pub fn modes(&self) -> impl Iterator<Item = &FermionicMode> {
        self.modes.values()
    }

    /// Returns a declared mode.
    #[must_use]
    pub fn mode(
        &self,
        id: FermionicModeId,
    ) -> Option<&FermionicMode> {
        self.modes.get(&id)
    }

    /// Returns the terms in semantic insertion order.
    ///
    /// Term order is not interpreted as algebraic operator order.
    #[must_use]
    pub fn terms(&self) -> &[FermionicTerm] {
        &self.terms
    }

    /// Returns the number of declared modes.
    #[must_use]
    pub fn mode_count(&self) -> usize {
        self.modes.len()
    }

    /// Returns the number of terms.
    #[must_use]
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether the Hamiltonian contains no terms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns the unique modes actually referenced by terms.
    #[must_use]
    pub fn referenced_modes(&self) -> Vec<FermionicModeId> {
        let mut modes = BTreeSet::new();

        for term in &self.terms {
            for mode in term.modes() {
                modes.insert(mode);
            }
        }

        modes.into_iter().collect()
    }

    /// Returns whether every term conserves particle number.
    #[must_use]
    pub fn is_number_conserving(&self) -> bool {
        self.terms
            .iter()
            .all(FermionicTerm::is_number_conserving)
    }

    /// Returns whether every term has even fermionic parity.
    #[must_use]
    pub fn is_even(&self) -> bool {
        self.terms.iter().all(FermionicTerm::is_even)
    }

    /// Validates the complete Hamiltonian.
    pub fn validate(&self) -> FermionicResult<()> {
        for mode in self.modes.values() {
            if mode.id().value() == u64::MAX {
                return Err(FermionicError::InvalidModeId);
            }
        }

        for term in &self.terms {
            term.validate()?;

            for mode in term.modes() {
                if !self.modes.contains_key(&mode) {
                    return Err(FermionicError::InvalidOperation {
                        message: "term references an undeclared fermionic mode",
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns a deterministic view of terms sorted by operator structure.
    ///
    /// The original Hamiltonian is not mutated.
    #[must_use]
    pub fn canonical_term_order(&self) -> Vec<&FermionicTerm> {
        let mut terms: Vec<&FermionicTerm> = self.terms.iter().collect();

        terms.sort_by(|left, right| {
            left.structural_key()
                .cmp(&right.structural_key())
        });

        terms
    }
}

// =============================================================================
// Fermion-to-qubit mapping
// =============================================================================

/// Explicit mapping from semantic fermionic modes to canonical IR qubits.
///
/// This mapping does NOT select hardware qubits.
///
/// It only states:
///
/// ```text
/// fermionic mode -> logical/encoded qubit
/// ```
///
/// Physical placement remains outside this module.
///
/// This is the explicit place where `quantum::ir::qubit::QubitId` is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FermionicQubitEncoding {
    mode_to_qubit: BTreeMap<FermionicModeId, QubitId>,
    qubit_to_mode: BTreeMap<QubitId, FermionicModeId>,
}

impl Default for FermionicQubitEncoding {
    fn default() -> Self {
        Self::new()
    }
}

impl FermionicQubitEncoding {
    /// Creates an empty encoding.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mode_to_qubit: BTreeMap::new(),
            qubit_to_mode: BTreeMap::new(),
        }
    }

    /// Adds a one-to-one fermionic-mode-to-qubit mapping.
    pub fn insert(
        &mut self,
        mode: FermionicModeId,
        qubit: QubitId,
    ) -> FermionicResult<()> {
        if self.mode_to_qubit.contains_key(&mode) {
            return Err(FermionicError::DuplicateModeMapping { mode });
        }

        if self.qubit_to_mode.contains_key(&qubit) {
            return Err(FermionicError::DuplicateQubitMapping { qubit });
        }

        self.mode_to_qubit.insert(mode, qubit);
        self.qubit_to_mode.insert(qubit, mode);

        Ok(())
    }

    /// Returns the qubit associated with a fermionic mode.
    #[must_use]
    pub fn qubit_for_mode(
        &self,
        mode: FermionicModeId,
    ) -> Option<QubitId> {
        self.mode_to_qubit.get(&mode).copied()
    }

    /// Returns the fermionic mode associated with a qubit.
    #[must_use]
    pub fn mode_for_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<FermionicModeId> {
        self.qubit_to_mode.get(&qubit).copied()
    }

    /// Returns the number of mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mode_to_qubit.len()
    }

    /// Returns whether no mappings exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mode_to_qubit.is_empty()
    }

    /// Iterates over mappings in deterministic mode order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (FermionicModeId, QubitId)> + '_ {
        self.mode_to_qubit
            .iter()
            .map(|(mode, qubit)| (*mode, *qubit))
    }

    /// Validates that every required mode has a mapping.
    pub fn validate_for_modes<I>(
        &self,
        modes: I,
    ) -> FermionicResult<()>
    where
        I: IntoIterator<Item = FermionicModeId>,
    {
        for mode in modes {
            if !self.mode_to_qubit.contains_key(&mode) {
                return Err(FermionicError::MissingModeMapping { mode });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Fermionic number operator helpers
// =============================================================================

/// Creates the semantic fermionic number operator:
///
/// ```text
/// n_p = a†_p a_p
/// ```
pub fn number_operator(
    coefficient: FermionicCoefficient,
    mode: FermionicModeId,
) -> FermionicResult<FermionicTerm> {
    FermionicTerm::number_operator(coefficient, mode)
}

/// Creates a one-body fermionic hopping term:
///
/// ```text
/// a†_p a_q
/// ```
pub fn hopping_term(
    coefficient: FermionicCoefficient,
    from: FermionicModeId,
    to: FermionicModeId,
) -> FermionicResult<FermionicTerm> {
    FermionicTerm::one_body(coefficient, from, to)
}

/// Creates a two-body fermionic interaction:
///
/// ```text
/// a†_p a†_q a_r a_s
/// ```
pub fn two_body_term(
    coefficient: FermionicCoefficient,
    p: FermionicModeId,
    q: FermionicModeId,
    r: FermionicModeId,
    s: FermionicModeId,
) -> FermionicResult<FermionicTerm> {
    FermionicTerm::two_body(coefficient, p, q, r, s)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn coefficient_one() -> FermionicCoefficient {
        FermionicCoefficient::one().expect("one must be valid")
    }

    #[test]
    fn mode_identity_is_distinct_from_qubit_identity() {
        let mode = FermionicModeId::new(7);
        let qubit = QubitId::new(7);

        assert_eq!(mode.value(), 7);
        assert_eq!(qubit.index(), 7);
    }

    #[test]
    fn creation_and_annihilation_are_distinct() {
        let mode = FermionicModeId::new(0);

        let creation = FermionicOperator::creation(mode);
        let annihilation = FermionicOperator::annihilation(mode);

        assert_ne!(creation, annihilation);
        assert!(creation.is_creation());
        assert!(annihilation.is_annihilation());
    }

    #[test]
    fn number_operator_is_number_conserving() {
        let term =
            number_operator(coefficient_one(), FermionicModeId::new(0))
                .expect("number operator must be valid");

        assert!(term.is_number_conserving());
        assert!(term.is_even());
        assert_eq!(term.operator_count(), 2);
    }

    #[test]
    fn two_body_term_is_even_and_number_conserving() {
        let term = two_body_term(
            coefficient_one(),
            FermionicModeId::new(0),
            FermionicModeId::new(1),
            FermionicModeId::new(2),
            FermionicModeId::new(3),
        )
        .expect("two-body term must be valid");

        assert!(term.is_even());
        assert!(term.is_number_conserving());
        assert_eq!(term.operator_count(), 4);
    }

    #[test]
    fn Hamiltonian_requires_declared_modes() {
        let mut hamiltonian = FermionicHamiltonian::new();

        hamiltonian
            .add_mode(FermionicMode::new(FermionicModeId::new(0)))
            .expect("mode registration must succeed");

        let term =
            number_operator(coefficient_one(), FermionicModeId::new(1))
                .expect("term construction must succeed");

        assert!(hamiltonian.add_term(term).is_err());
    }

    #[test]
    fn Hamiltonian_accepts_declared_modes() {
        let mut hamiltonian = FermionicHamiltonian::new();

        hamiltonian
            .add_mode(FermionicMode::new(FermionicModeId::new(0)))
            .expect("mode registration must succeed");

        hamiltonian
            .add_term(
                number_operator(
                    coefficient_one(),
                    FermionicModeId::new(0),
                )
                .expect("term construction must succeed"),
            )
            .expect("term insertion must succeed");

        assert_eq!(hamiltonian.mode_count(), 1);
        assert_eq!(hamiltonian.term_count(), 1);
        assert!(hamiltonian.validate().is_ok());
    }

    #[test]
    fn encoding_is_bijective() {
        let mode = FermionicModeId::new(4);
        let qubit = QubitId::new(9);

        let mut encoding = FermionicQubitEncoding::new();

        encoding
            .insert(mode, qubit)
            .expect("mapping must succeed");

        assert_eq!(encoding.qubit_for_mode(mode), Some(qubit));
        assert_eq!(encoding.mode_for_qubit(qubit), Some(mode));
        assert_eq!(encoding.len(), 1);
    }

    #[test]
    fn encoding_rejects_duplicate_mode() {
        let mode = FermionicModeId::new(0);

        let mut encoding = FermionicQubitEncoding::new();

        encoding
            .insert(mode, QubitId::new(0))
            .expect("first mapping must succeed");

        assert_eq!(
            encoding.insert(mode, QubitId::new(1)),
            Err(FermionicError::DuplicateModeMapping { mode })
        );
    }

    #[test]
    fn encoding_rejects_duplicate_qubit() {
        let qubit = QubitId::new(0);

        let mut encoding = FermionicQubitEncoding::new();

        encoding
            .insert(FermionicModeId::new(0), qubit)
            .expect("first mapping must succeed");

        assert_eq!(
            encoding.insert(FermionicModeId::new(1), qubit),
            Err(FermionicError::DuplicateQubitMapping { qubit })
        );
    }

    #[test]
    fn deterministic_mode_iteration() {
        let mut hamiltonian = FermionicHamiltonian::new();

        hamiltonian
            .add_mode(FermionicMode::new(FermionicModeId::new(20)))
            .expect("mode must be accepted");

        hamiltonian
            .add_mode(FermionicMode::new(FermionicModeId::new(2)))
            .expect("mode must be accepted");

        let ids: Vec<_> =
            hamiltonian.modes().map(FermionicMode::id).collect();

        assert_eq!(
            ids,
            vec![
                FermionicModeId::new(2),
                FermionicModeId::new(20)
            ]
        );
    }

    #[test]
    fn symbolic_coefficient_is_supported() {
        let parameter =
            Parameter::symbol("t").expect("symbol must be valid");

        let coefficient =
            FermionicCoefficient::real(parameter)
                .expect("coefficient must be valid");

        let term = FermionicTerm::number_operator(
            coefficient,
            FermionicModeId::new(0),
        )
        .expect("symbolic term must be valid");

        assert!(term.validate().is_ok());
    }

    #[test]
    fn complex_coefficient_is_supported() {
        let real =
            Parameter::constant(1.0).expect("finite value is valid");
        let imaginary =
            Parameter::constant(-0.5).expect("finite value is valid");

        let coefficient =
            FermionicCoefficient::complex(real, imaginary)
                .expect("complex coefficient must be valid");

        assert!(coefficient.is_complex());
        assert!(coefficient.validate().is_ok());
    }

    #[test]
    fn mode_next_is_checked() {
        let mode = FermionicModeId::new(u64::MAX);

        assert_eq!(mode.checked_next(), None);
    }

    #[test]
    fn term_structure_preserves_operator_order() {
        let p = FermionicModeId::new(0);
        let q = FermionicModeId::new(1);

        let term = FermionicTerm::new(
            coefficient_one(),
            vec![
                FermionicOperator::annihilation(q),
                FermionicOperator::creation(p),
            ],
        )
        .expect("term must be valid");

        assert_eq!(
            term.operators()[0],
            FermionicOperator::annihilation(q)
        );
        assert_eq!(
            term.operators()[1],
            FermionicOperator::creation(p)
        );
    }
}