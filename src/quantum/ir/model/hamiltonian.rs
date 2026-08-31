//! Zamani Quantum IR — Hamiltonian Model
//!
//! Path:
//!     src/quantum/ir/model/hamiltonian.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent representation of a
//! quantum Hamiltonian.
//!
//! A Hamiltonian represents semantic quantum evolution such as:
//!
//!     H
//!     H(t)
//!     H(theta)
//!     H = Σ_i c_i P_i
//!
//! without deciding:
//!
//! - which hardware executes it;
//! - which gates implement it;
//! - which pulse sequence implements it;
//! - which simulator represents it;
//! - which physical qubits are used;
//! - how the Hamiltonian is optimized;
//! - how time evolution is discretized;
//! - which numerical method is used;
//! - how the backend schedules execution.
//!
//! Those responsibilities belong to downstream compiler layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! canonical Zamani IR
//!      │
//!      ├── model::hamiltonian     ← THIS FILE
//!      │
//!      ├── optimization
//!      │
//!      ├── decomposition
//!      │
//!      ├── mapping/routing
//!      │
//!      ├── scheduling
//!      │
//!      ├── pulse lowering
//!      │
//!      ├── simulator
//!      │
//!      └── backend
//!
//! # Universal-program principle
//!
//! The Hamiltonian representation must work for:
//!
//! - one qubit;
//! - small systems;
//! - large systems;
//! - sparse systems;
//! - distributed systems;
//! - logical qubits;
//! - future quantum architectures;
//! - arbitrary finite numbers of Hamiltonian terms;
//! - symbolic coefficients;
//! - time-dependent coefficients;
//! - standard Pauli Hamiltonians;
//! - extensible non-Pauli operators.
//!
//! There is deliberately NO:
//!
//! - MAX_QUBITS;
//! - MAX_TERMS;
//! - MAX_LOCALITY;
//! - MAX_OPERATOR_ARITY;
//! - MAX_HAMILTONIAN_SIZE;
//! - fixed hardware topology;
//! - fixed gate decomposition;
//! - fixed numerical precision;
//! - fixed simulator representation.
//!
//! Any resource/security limits belong to an explicit compiler/execution
//! policy, not to this semantic model.
//!
//! # Important design rule
//!
//! A Hamiltonian is not represented as a dense matrix.
//!
//! Dense matrices scale exponentially with the number of quantum resources
//! and would make the IR itself impose an unnecessary representation ceiling.
//!
//! Instead, this module uses a symbolic sum of operator terms:
//!
//!     H = Σ_k c_k O_k
//!
//! where each `O_k` is represented structurally.
//!
//! Pauli products are represented natively because they are compact and form
//! an important canonical basis for qubit Hamiltonians.
//!
//! Arbitrary operator terms are also supported through `OperatorDescriptor`.
//!
//! # Coefficients
//!
//! Coefficients use the canonical Zamani `Parameter` representation.
//!
//! This permits:
//!
//!     0.5
//!     theta
//!     theta / 2
//!     2 * theta + pi
//!     t
//!     J(t)
//!
//! without forcing numerical binding during IR construction.
//!
//! The Hamiltonian model therefore remains compatible with:
//!
//! - symbolic compilation;
//! - parameter sweeps;
//! - VQE;
//! - QAOA;
//! - quantum simulation;
//! - optimal control;
//! - time-dependent evolution;
//! - calibration-aware lowering.
//!
//! # Hermiticity
//!
//! A physical Hamiltonian must be Hermitian.
//!
//! This module distinguishes:
//!
//! - `Hermitian` — the term is known to be Hermitian;
//! - `NonHermitian` — explicitly known not to be Hermitian;
//! - `Unknown` — the IR does not have enough information to prove it.
//!
//! Pauli products are intrinsically Hermitian. Their coefficients are real
//! because `Parameter` is a real scalar expression.
//!
//! Arbitrary operators require an explicit hermiticity classification.
//!
//! The model does NOT perform matrix-level symbolic proof of arbitrary
//! operators. That belongs to a specialized mathematical analysis pass.
//!
//! # Canonical qubit identity
//!
//! All logical qubits use:
//!
//!     quantum::ir::qubit::QubitId
//!
//! This file deliberately does not define another qubit identifier.
//!
//! Physical qubit identities do not belong in the semantic Hamiltonian.
//! Mapping from logical to physical resources belongs to the mapping/routing
//! layer.
//!
//! # Dependency contract
//!
//! This file depends only on foundational IR contracts:
//!
//!     quantum::ir::core::parameter
//!     quantum::ir::qubit
//!
//! It does NOT depend on:
//!
//!     operation
//!     circuit
//!     hardware
//!     simulator
//!     optimization
//!     routing
//!     scheduling
//!     pulse
//!     backend
//!
//! This allows this file to be completed and frozen independently.
//!
//! # Integration contract
//!
//! Upstream:
//!
//!     core::parameter::Parameter
//!     qubit::QubitId
//!
//! Downstream consumers:
//!
//!     model::circuit
//!     model::analog
//!     model::annealing
//!     algorithms::vqe
//!     algorithms::qaoa
//!     optimization
//!     simulation
//!     hardware
//!     scheduling
//!     validation
//!     serialization
//!     hashing
//!     analysis
//!
//! None of those downstream modules may redefine the Hamiltonian types.
//!
//! # Serialization contract
//!
//! The semantic structure must be serializable without converting it to a
//! matrix or relying on `Display` strings.
//!
//! Stable serialization should encode:
//!
//!     Hamiltonian
//!       ├── terms
//!       │     ├── coefficient
//!       │     ├── operator
//!       │     ├── targets
//!       │     └── hermiticity
//!       └── metadata
//!
//! Canonical serialization should sort mathematically commutative collections
//! deterministically while preserving the semantic order of non-commutative
//! operator factors.
//!
//! # Hashing contract
//!
//! The structures intentionally derive deterministic equality/order traits
//! where possible.
//!
//! Cryptographic hashing is NOT implemented here. The repository's hashing
//! layer owns canonical hashing.
//!
//! # Validation contract
//!
//! Local invariants are checked during construction:
//!
//! - Pauli products cannot contain duplicate qubit identifiers;
//! - arbitrary operator target lists cannot contain duplicate identifiers;
//! - identity terms contain no targets;
//! - non-identity Pauli products contain at least one target;
//! - operator descriptors cannot have an empty namespace/name;
//! - coefficients must be valid `Parameter`s;
//! - explicit `NonHermitian` Hamiltonians can be rejected by strict validation;
//! - empty Hamiltonians are valid and represent the zero Hamiltonian.
//!
//! Whole-program validation remains outside this module.
//!
//! # Scalability
//!
//! `Vec` is used for materialized term collections because the compiler must
//! eventually iterate over terms. The semantic model has no fixed term limit.
//!
//! Large systems can use:
//!
//! - sparse Hamiltonians;
//! - streaming construction;
//! - partitioned Hamiltonians;
//! - distributed lowering;
//! - lazy generation;
//! - compressed representations;
//! - external storage;
//! - compiler resource policies.
//!
//! None of those require changing this semantic model.
//!
//! # Rust contract
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Ownership
//!
//! This file OWNS:
//!
//! - `Hamiltonian`;
//! - `HamiltonianTerm`;
//! - `HamiltonianOperator`;
//! - `PauliProduct`;
//! - `PauliFactor`;
//! - `OperatorDescriptor`;
//! - `Hermiticity`;
//! - local Hamiltonian errors;
//! - local Hamiltonian validation policy.
//!
//! This file DOES NOT OWN:
//!
//! - physical hardware;
//! - gate decomposition;
//! - pulse synthesis;
//! - numerical simulation;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - backend execution.
//!
//! # Safety
//!
//! No unsafe code is used or required.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::core::parameter::{
    Parameter,
    ParameterError,
    ParameterValidationPolicy,
};
use super::super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type for Hamiltonian construction and local validation.
pub type HamiltonianResult<T> = Result<T, HamiltonianError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by Hamiltonian construction or local validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HamiltonianError {
    /// A namespace is empty.
    EmptyNamespace,

    /// An operator name is empty.
    EmptyOperatorName,

    /// A Pauli product contains the same logical qubit more than once.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A generic operator target list contains the same logical qubit more
    /// than once.
    DuplicateOperatorTarget {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A Pauli factor has an invalid semantic identity.
    InvalidPauliFactor,

    /// A Pauli identity factor was supplied explicitly.
    ///
    /// Identity has no target and therefore should be represented by the
    /// absence of a factor or by `PauliProduct::identity()`.
    ExplicitIdentityFactor,

    /// An operator descriptor has an invalid target count.
    TargetArityMismatch {
        /// Declared arity.
        declared: u64,

        /// Actual target count.
        actual: u64,
    },

    /// A non-identity operator was created without targets.
    MissingTargets,

    /// An explicitly non-Hermitian term was used where a Hermitian Hamiltonian
    /// is required.
    NonHermitianTerm,

    /// The Hamiltonian contains an explicitly non-Hermitian term.
    NonHermitianHamiltonian,

    /// A parameter operation failed.
    Parameter(ParameterError),
}

impl fmt::Display for HamiltonianError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("Hamiltonian operator namespace cannot be empty")
            }

            Self::EmptyOperatorName => {
                formatter.write_str("Hamiltonian operator name cannot be empty")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "Hamiltonian Pauli product contains duplicate qubit {qubit}"
                )
            }

            Self::DuplicateOperatorTarget { qubit } => {
                write!(
                    formatter,
                    "Hamiltonian operator contains duplicate target qubit {qubit}"
                )
            }

            Self::InvalidPauliFactor => {
                formatter.write_str("invalid Pauli factor")
            }

            Self::ExplicitIdentityFactor => {
                formatter.write_str(
                    "Pauli identity must not be represented as an explicit \
                     targeted factor",
                )
            }

            Self::TargetArityMismatch { declared, actual } => {
                write!(
                    formatter,
                    "operator declares arity {declared} but has {actual} targets"
                )
            }

            Self::MissingTargets => {
                formatter.write_str(
                    "non-identity Hamiltonian operator requires at least one \
                     target",
                )
            }

            Self::NonHermitianTerm => {
                formatter.write_str(
                    "non-Hermitian Hamiltonian term is not allowed in strict \
                     Hermitian construction",
                )
            }

            Self::NonHermitianHamiltonian => {
                formatter.write_str(
                    "Hamiltonian contains an explicitly non-Hermitian term",
                )
            }

            Self::Parameter(error) => {
                write!(formatter, "invalid Hamiltonian coefficient: {error}")
            }
        }
    }
}

impl std::error::Error for HamiltonianError {}

impl From<ParameterError> for HamiltonianError {
    fn from(error: ParameterError) -> Self {
        Self::Parameter(error)
    }
}

// =============================================================================
// Hermiticity
// =============================================================================

/// Hermiticity knowledge for a Hamiltonian operator or term.
///
/// `Unknown` is intentionally different from `NonHermitian`.
///
/// An IR consumer may require proof of Hermiticity and reject `Unknown`, while
/// a less restrictive consumer may permit it and defer proof to a later pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hermiticity {
    /// The operator is known to be Hermitian.
    Hermitian,

    /// The operator is explicitly known to be non-Hermitian.
    NonHermitian,

    /// The IR does not have enough information to prove either property.
    Unknown,
}

impl Hermiticity {
    /// Returns whether the operator is known to be Hermitian.
    #[must_use]
    pub const fn is_hermitian(self) -> bool {
        matches!(self, Self::Hermitian)
    }

    /// Returns whether the operator is known to be non-Hermitian.
    #[must_use]
    pub const fn is_non_hermitian(self) -> bool {
        matches!(self, Self::NonHermitian)
    }

    /// Returns whether Hermiticity is unresolved.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Pauli factor
// =============================================================================

/// Single-qubit Pauli operator.
///
/// The identity is deliberately excluded.
///
/// An identity factor carries no qubit-local information and is represented by
/// simply omitting the factor from a `PauliProduct`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PauliFactor {
    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

impl PauliFactor {
    /// Returns the canonical single-character name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        }
    }

    /// Returns whether this Pauli is intrinsically Hermitian.
    #[must_use]
    pub const fn hermiticity(self) -> Hermiticity {
        Hermiticity::Hermitian
    }
}

impl fmt::Display for PauliFactor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Pauli product
// =============================================================================

/// A sparse tensor product of single-qubit Pauli operators.
///
/// Examples:
///
/// ```text
/// X(q0)
///
/// Z(q1)
///
/// X(q0) Z(q1)
///
/// Y(q0) Y(q1) Z(q7)
/// ```
///
/// The factors are maintained in deterministic `QubitId` order.
///
/// This makes equivalent target orderings canonical:
///
/// ```text
/// X(q0) Z(q1)
/// ```
///
/// and
///
/// ```text
/// Z(q1) X(q0)
/// ```
///
/// produce the same structural Pauli product when their factors refer to
/// distinct qubits.
///
/// The product itself is Hermitian.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PauliProduct {
    factors: Vec<(QubitId, PauliFactor)>,
}

impl PauliProduct {
    /// Creates the identity operator.
    ///
    /// The identity is represented by an empty factor list.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            factors: Vec::new(),
        }
    }

    /// Creates a single-qubit Pauli operator.
    #[must_use]
    pub fn single(
        qubit: QubitId,
        factor: PauliFactor,
    ) -> Self {
        Self {
            factors: vec![(qubit, factor)],
        }
    }

    /// Creates a Pauli product from factors.
    ///
    /// Factors are sorted by `QubitId` so construction is deterministic.
    ///
    /// Duplicate qubits are rejected because a canonical Pauli product must
    /// contain at most one single-qubit Pauli factor per target.
    pub fn new<I>(
        factors: I,
    ) -> HamiltonianResult<Self>
    where
        I: IntoIterator<Item = (QubitId, PauliFactor)>,
    {
        let mut factors: Vec<(QubitId, PauliFactor)> =
            factors.into_iter().collect();

        factors.sort_by_key(|(qubit, _)| *qubit);

        let mut seen = BTreeSet::new();

        for (qubit, _) in &factors {
            if !seen.insert(*qubit) {
                return Err(HamiltonianError::DuplicateQubit {
                    qubit: *qubit,
                });
            }
        }

        Ok(Self { factors })
    }

    /// Returns the number of non-identity Pauli factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns whether this product is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the deterministic factor slice.
    #[must_use]
    pub fn factors(&self) -> &[(QubitId, PauliFactor)] {
        &self.factors
    }

    /// Returns the qubits touched by this product.
    ///
    /// The returned vector is deterministic and follows canonical qubit order.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        self.factors
            .iter()
            .map(|(qubit, _)| *qubit)
            .collect()
    }

    /// Returns the factor associated with a qubit.
    #[must_use]
    pub fn factor_at(
        &self,
        qubit: QubitId,
    ) -> Option<PauliFactor> {
        self.factors
            .binary_search_by_key(&qubit, |(id, _)| *id)
            .ok()
            .map(|index| self.factors[index].1)
    }

    /// Returns the Pauli product's Hermiticity.
    #[must_use]
    pub const fn hermiticity(&self) -> Hermiticity {
        Hermiticity::Hermitian
    }

    /// Returns the product of two Pauli products when they act on disjoint
    /// qubits.
    ///
    /// This method intentionally rejects overlapping qubits rather than
    /// silently performing Pauli multiplication and introducing phase factors.
    /// Algebraic Pauli multiplication belongs to a dedicated algebra/optimizer
    /// layer.
    pub fn tensor_product(
        &self,
        other: &Self,
    ) -> HamiltonianResult<Self> {
        let mut combined = Vec::with_capacity(
            self.factors.len().saturating_add(other.factors.len()),
        );

        combined.extend_from_slice(&self.factors);
        combined.extend_from_slice(&other.factors);

        Self::new(combined)
    }

    /// Returns whether this product acts only on the supplied qubit.
    #[must_use]
    pub fn acts_on_single_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.factors.len() == 1
            && self.factors[0].0 == qubit
    }
}

impl Default for PauliProduct {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for PauliProduct {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.is_identity() {
            return formatter.write_str("I");
        }

        for (index, (qubit, factor)) in self.factors.iter().enumerate() {
            if index != 0 {
                formatter.write_str(" ")?;
            }

            write!(formatter, "{factor}({qubit})")?;
        }

        Ok(())
    }
}

// =============================================================================
// Generic operator descriptor
// =============================================================================

/// Stable semantic descriptor for a non-Pauli Hamiltonian operator.
///
/// This is intentionally descriptive rather than a dense matrix.
///
/// Examples include future operators such as:
///
/// ```text
/// fermionic.creation
/// fermionic.annihilation
/// bosonic.creation
/// bosonic.annihilation
/// cv.position
/// cv.momentum
/// custom.operator
/// ```
///
/// A downstream dialect can attach richer semantics without changing this
/// foundational Hamiltonian model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperatorDescriptor {
    namespace: String,
    name: String,
    arity: u64,
    hermiticity: Hermiticity,
}

impl OperatorDescriptor {
    /// Creates an operator descriptor.
    ///
    /// `arity` is the semantic number of target resources.
    pub fn new<N, S>(
        namespace: N,
        name: S,
        arity: u64,
        hermiticity: Hermiticity,
    ) -> HamiltonianResult<Self>
    where
        N: Into<String>,
        S: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(HamiltonianError::EmptyNamespace);
        }

        if name.trim().is_empty() {
            return Err(HamiltonianError::EmptyOperatorName);
        }

        Ok(Self {
            namespace,
            name,
            arity,
            hermiticity,
        })
    }

    /// Returns the operator namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the operator name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the semantic operator arity.
    #[must_use]
    pub const fn arity(&self) -> u64 {
        self.arity
    }

    /// Returns Hermiticity information.
    #[must_use]
    pub const fn hermiticity(&self) -> Hermiticity {
        self.hermiticity
    }

    /// Returns the fully-qualified operator name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for OperatorDescriptor {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Hamiltonian operator
// =============================================================================

/// Operator appearing in a Hamiltonian term.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HamiltonianOperator {
    /// Sparse tensor product of Pauli operators.
    Pauli(PauliProduct),

    /// Extensible operator described by a semantic namespace/name and target
    /// resources.
    Generic {
        /// Semantic operator descriptor.
        descriptor: OperatorDescriptor,

        /// Logical quantum resources on which the operator acts.
        targets: Vec<QubitId>,
    },
}

impl HamiltonianOperator {
    /// Creates a Pauli operator.
    #[must_use]
    pub const fn pauli(product: PauliProduct) -> Self {
        Self::Pauli(product)
    }

    /// Creates a generic operator.
    ///
    /// The target list is canonicalized by sorting and is checked for
    /// duplicate logical qubits.
    pub fn generic(
        descriptor: OperatorDescriptor,
        targets: Vec<QubitId>,
    ) -> HamiltonianResult<Self> {
        let actual = targets.len() as u64;

        if descriptor.arity() != actual {
            return Err(HamiltonianError::TargetArityMismatch {
                declared: descriptor.arity(),
                actual,
            });
        }

        if descriptor.arity() != 0 && targets.is_empty() {
            return Err(HamiltonianError::MissingTargets);
        }

        let mut targets = targets;

        targets.sort();

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(
                    HamiltonianError::DuplicateOperatorTarget {
                        qubit: *target,
                    },
                );
            }
        }

        Ok(Self::Generic {
            descriptor,
            targets,
        })
    }

    /// Returns the operator's Hermiticity knowledge.
    #[must_use]
    pub fn hermiticity(&self) -> Hermiticity {
        match self {
            Self::Pauli(product) => product.hermiticity(),

            Self::Generic {
                descriptor,
                ..
            } => descriptor.hermiticity(),
        }
    }

    /// Returns the number of quantum targets.
    #[must_use]
    pub fn arity(&self) -> usize {
        match self {
            Self::Pauli(product) => product.len(),

            Self::Generic {
                targets,
                ..
            } => targets.len(),
        }
    }

    /// Returns all logical qubits referenced by this operator.
    ///
    /// The returned vector is deterministic.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        match self {
            Self::Pauli(product) => product.qubits(),

            Self::Generic {
                targets,
                ..
            } => targets.clone(),
        }
    }

    /// Returns whether this operator is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        match self {
            Self::Pauli(product) => product.is_identity(),

            Self::Generic {
                descriptor,
                targets,
            } => descriptor.arity() == 0 && targets.is_empty(),
        }
    }

    /// Returns the Pauli product if this is a Pauli operator.
    #[must_use]
    pub fn as_pauli(&self) -> Option<&PauliProduct> {
        match self {
            Self::Pauli(product) => Some(product),
            Self::Generic { .. } => None,
        }
    }

    /// Returns the generic descriptor and targets if this is a generic
    /// operator.
    #[must_use]
    pub fn as_generic(
        &self,
    ) -> Option<(&OperatorDescriptor, &[QubitId])> {
        match self {
            Self::Pauli(_) => None,

            Self::Generic {
                descriptor,
                targets,
            } => Some((descriptor, targets)),
        }
    }
}

impl fmt::Display for HamiltonianOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Pauli(product) => write!(formatter, "{product}"),

            Self::Generic {
                descriptor,
                targets,
            } => {
                write!(formatter, "{descriptor}")?;

                if !targets.is_empty() {
                    formatter.write_str("(")?;

                    for (index, target) in targets.iter().enumerate() {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }

                        write!(formatter, "{target}")?;
                    }

                    formatter.write_str(")")?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Hamiltonian term
// =============================================================================

/// One term of a Hamiltonian.
///
/// Semantically:
///
///     coefficient × operator
///
/// Examples:
///
///     0.5 × Z(q0)
///
///     J × X(q0) X(q1)
///
///     theta × Z(q0) Z(q1)
///
///     t × custom.operator(q0, q1)
#[derive(Debug, Clone, PartialEq)]
pub struct HamiltonianTerm {
    coefficient: Parameter,
    operator: HamiltonianOperator,
}

impl HamiltonianTerm {
    /// Creates a Hamiltonian term.
    pub fn new(
        coefficient: Parameter,
        operator: HamiltonianOperator,
    ) -> Self {
        Self {
            coefficient,
            operator,
        }
    }

    /// Creates a Pauli Hamiltonian term.
    pub fn pauli(
        coefficient: Parameter,
        product: PauliProduct,
    ) -> Self {
        Self::new(
            coefficient,
            HamiltonianOperator::Pauli(product),
        )
    }

    /// Creates a generic Hamiltonian term.
    pub fn generic(
        coefficient: Parameter,
        descriptor: OperatorDescriptor,
        targets: Vec<QubitId>,
    ) -> HamiltonianResult<Self> {
        let operator =
            HamiltonianOperator::generic(descriptor, targets)?;

        Ok(Self::new(coefficient, operator))
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Returns the operator.
    #[must_use]
    pub fn operator(&self) -> &HamiltonianOperator {
        &self.operator
    }

    /// Returns the operator's Hermiticity.
    #[must_use]
    pub fn hermiticity(&self) -> Hermiticity {
        self.operator.hermiticity()
    }

    /// Returns all logical qubits referenced by the term.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        self.operator.qubits()
    }

    /// Returns whether this term is explicitly non-Hermitian.
    #[must_use]
    pub fn is_non_hermitian(&self) -> bool {
        self.hermiticity().is_non_hermitian()
    }

    /// Returns whether this term is known to be Hermitian.
    #[must_use]
    pub fn is_hermitian(&self) -> bool {
        self.hermiticity().is_hermitian()
    }
}

// =============================================================================
// Hamiltonian validation policy
// =============================================================================

/// Explicit validation policy for a Hamiltonian.
///
/// These are compiler/security policies, NOT semantic limits.
///
/// `None` means that the policy does not impose that particular bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HamiltonianValidationPolicy {
    /// Maximum number of terms inspected.
    pub max_terms: Option<usize>,

    /// Maximum number of logical qubit references inspected.
    pub max_qubit_references: Option<usize>,

    /// Parameter validation policy used for coefficients.
    pub parameter_policy: ParameterValidationPolicy,

    /// Whether an explicitly non-Hermitian term is rejected.
    pub require_hermitian: bool,

    /// Whether unknown Hermiticity is rejected.
    ///
    /// This is useful for execution paths that require proof rather than
    /// merely absence of known non-Hermitian structure.
    pub require_known_hermitian: bool,
}

impl HamiltonianValidationPolicy {
    /// Returns an unrestricted semantic policy.
    ///
    /// Explicitly non-Hermitian Hamiltonians are permitted by this policy,
    /// allowing generic mathematical operators to be represented before a
    /// downstream pass decides whether Hermitian evolution is required.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_terms: None,
            max_qubit_references: None,
            parameter_policy:
                ParameterValidationPolicy::unrestricted(),
            require_hermitian: false,
            require_known_hermitian: false,
        }
    }

    /// Returns a strict physical-Hamiltonian policy.
    ///
    /// The policy requires every term to be known Hermitian.
    #[must_use]
    pub const fn strict_hermitian() -> Self {
        Self {
            max_terms: None,
            max_qubit_references: None,
            parameter_policy:
                ParameterValidationPolicy::unrestricted(),
            require_hermitian: true,
            require_known_hermitian: true,
        }
    }

    /// Returns a resource-bounded strict policy.
    #[must_use]
    pub const fn bounded_strict_hermitian(
        max_terms: Option<usize>,
        max_qubit_references: Option<usize>,
        parameter_policy: ParameterValidationPolicy,
    ) -> Self {
        Self {
            max_terms,
            max_qubit_references,
            parameter_policy,
            require_hermitian: true,
            require_known_hermitian: true,
        }
    }
}

impl Default for HamiltonianValidationPolicy {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// =============================================================================
// Hamiltonian
// =============================================================================

/// Canonical sparse symbolic Hamiltonian.
///
/// The semantic form is:
///
///     H = Σ_k c_k O_k
///
/// where `c_k` is a symbolic/numeric `Parameter` and `O_k` is a structured
/// quantum operator.
///
/// An empty Hamiltonian is valid and represents the zero operator.
#[derive(Debug, Clone, PartialEq)]
pub struct Hamiltonian {
    terms: Vec<HamiltonianTerm>,
}

impl Hamiltonian {
    /// Creates an empty, zero Hamiltonian.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            terms: Vec::new(),
        }
    }

    /// Creates a Hamiltonian from terms.
    ///
    /// No architectural size limit is imposed.
    ///
    /// Use `validate_with_policy` when a compiler/security boundary needs an
    /// explicit resource budget.
    #[must_use]
    pub fn new(terms: Vec<HamiltonianTerm>) -> Self {
        Self { terms }
    }

    /// Creates a Hamiltonian containing one term.
    #[must_use]
    pub fn single(term: HamiltonianTerm) -> Self {
        Self {
            terms: vec![term],
        }
    }

    /// Returns the number of terms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether the Hamiltonian is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns the term slice.
    #[must_use]
    pub fn terms(&self) -> &[HamiltonianTerm] {
        &self.terms
    }

    /// Returns an iterator over terms.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &HamiltonianTerm> {
        self.terms.iter()
    }

    /// Adds a term without imposing a semantic size limit.
    ///
    /// Resource policies should be enforced by the caller through
    /// `validate_with_policy`.
    pub fn push(
        &mut self,
        term: HamiltonianTerm,
    ) {
        self.terms.push(term);
    }

    /// Returns a new Hamiltonian with one additional term.
    #[must_use]
    pub fn with_term(
        mut self,
        term: HamiltonianTerm,
    ) -> Self {
        self.push(term);
        self
    }

    /// Returns all logical qubits referenced by the Hamiltonian.
    ///
    /// Qubits are returned in deterministic ascending order with duplicates
    /// removed.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        let mut qubits = BTreeSet::new();

        for term in &self.terms {
            for qubit in term.qubits() {
                qubits.insert(qubit);
            }
        }

        qubits.into_iter().collect()
    }

    /// Returns the number of distinct logical qubits referenced by the
    /// Hamiltonian.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        let mut qubits = BTreeSet::new();

        for term in &self.terms {
            for qubit in term.qubits() {
                qubits.insert(qubit);
            }
        }

        qubits.len()
    }

    /// Returns whether every term is known Hermitian.
    ///
    /// An empty Hamiltonian is considered Hermitian.
    #[must_use]
    pub fn is_known_hermitian(&self) -> bool {
        self.terms
            .iter()
            .all(HamiltonianTerm::is_hermitian)
    }

    /// Returns whether any term is explicitly known to be non-Hermitian.
    #[must_use]
    pub fn contains_non_hermitian_term(&self) -> bool {
        self.terms
            .iter()
            .any(HamiltonianTerm::is_non_hermitian)
    }

    /// Returns whether the Hamiltonian contains an operator whose Hermiticity
    /// is unresolved.
    #[must_use]
    pub fn contains_unknown_hermiticity(&self) -> bool {
        self.terms.iter().any(|term| {
            term.hermiticity().is_unknown()
        })
    }

    /// Validates using the unrestricted semantic policy.
    pub fn validate(&self) -> HamiltonianResult<()> {
        self.validate_with_policy(
            HamiltonianValidationPolicy::unrestricted(),
        )
    }

    /// Validates using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: HamiltonianValidationPolicy,
    ) -> HamiltonianResult<()> {
        if let Some(limit) = policy.max_terms {
            if self.terms.len() > limit {
                return Err(HamiltonianError::Parameter(
                    ParameterError::InvalidStructure {
                        reason:
                            "Hamiltonian term count exceeds the supplied \
                             validation policy",
                    },
                ));
            }
        }

        let mut qubit_references = 0usize;

        for term in &self.terms {
            term.coefficient.validate_with_policy(
                policy.parameter_policy,
            )?;

            let term_qubits = term.qubits();

            qubit_references = qubit_references
                .checked_add(term_qubits.len())
                .ok_or(HamiltonianError::Parameter(
                    ParameterError::InvalidStructure {
                        reason:
                            "Hamiltonian qubit-reference count overflowed",
                    },
                ))?;

            if let Some(limit) = policy.max_qubit_references {
                if qubit_references > limit {
                    return Err(HamiltonianError::Parameter(
                        ParameterError::InvalidStructure {
                            reason:
                                "Hamiltonian qubit-reference count exceeds \
                                 the supplied validation policy",
                        },
                    ));
                }
            }

            if policy.require_hermitian
                && term.is_non_hermitian()
            {
                return Err(
                    HamiltonianError::NonHermitianTerm
                );
            }

            if policy.require_known_hermitian
                && !term.is_hermitian()
            {
                return Err(
                    HamiltonianError::NonHermitianTerm
                );
            }
        }

        Ok(())
    }

    /// Validates that the Hamiltonian is suitable for physical Hermitian
    /// evolution.
    ///
    /// This rejects both explicitly non-Hermitian and unknown operators.
    pub fn validate_hermitian(&self) -> HamiltonianResult<()> {
        self.validate_with_policy(
            HamiltonianValidationPolicy::strict_hermitian(),
        )
    }

    /// Returns the number of terms acting on exactly one logical qubit.
    #[must_use]
    pub fn single_qubit_term_count(&self) -> usize {
        self.terms
            .iter()
            .filter(|term| term.operator().arity() == 1)
            .count()
    }

    /// Returns the number of terms acting on more than one logical qubit.
    #[must_use]
    pub fn multi_qubit_term_count(&self) -> usize {
        self.terms
            .iter()
            .filter(|term| term.operator().arity() > 1)
            .count()
    }

    /// Returns the maximum locality/arity present in the Hamiltonian.
    ///
    /// Returns zero for the zero Hamiltonian.
    #[must_use]
    pub fn max_locality(&self) -> usize {
        self.terms
            .iter()
            .map(|term| term.operator().arity())
            .max()
            .unwrap_or(0)
    }

    /// Returns whether every operator in this Hamiltonian is a Pauli product.
    #[must_use]
    pub fn is_pauli_only(&self) -> bool {
        self.terms
            .iter()
            .all(|term| term.operator().as_pauli().is_some())
    }

    /// Returns whether every term is a zero-target identity term.
    ///
    /// The zero Hamiltonian returns `true`.
    #[must_use]
    pub fn is_identity_only(&self) -> bool {
        self.terms
            .iter()
            .all(|term| term.operator().is_identity())
    }

    /// Returns a deterministic list of all Pauli factors used by this
    /// Hamiltonian.
    ///
    /// Generic operators are ignored.
    #[must_use]
    pub fn pauli_terms(
        &self,
    ) -> impl Iterator<Item = &PauliProduct> {
        self.terms.iter().filter_map(|term| {
            term.operator().as_pauli()
        })
    }
}

impl Default for Hamiltonian {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Construction helpers
// =============================================================================

/// Creates a Pauli Hamiltonian term from one qubit.
///
/// Equivalent to:
///
///     coefficient × P(qubit)
pub fn pauli_term(
    coefficient: Parameter,
    qubit: QubitId,
    factor: PauliFactor,
) -> HamiltonianTerm {
    HamiltonianTerm::pauli(
        coefficient,
        PauliProduct::single(qubit, factor),
    )
}

/// Creates a Pauli-product Hamiltonian term.
///
/// This is useful for:
///
///     X(q0) X(q1)
///     Z(q0) Z(q1)
///     X(q0) Y(q3) Z(q7)
pub fn pauli_product_term(
    coefficient: Parameter,
    factors: Vec<(QubitId, PauliFactor)>,
) -> HamiltonianResult<HamiltonianTerm> {
    let product = PauliProduct::new(factors)?;

    Ok(HamiltonianTerm::pauli(
        coefficient,
        product,
    ))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn identity_product_is_empty_and_hermitian() {
        let identity = PauliProduct::identity();

        assert!(identity.is_identity());
        assert_eq!(identity.len(), 0);
        assert!(identity.hermiticity().is_hermitian());
    }

    #[test]
    fn pauli_factors_are_canonicalized_by_qubit() {
        let product = PauliProduct::new(vec![
            (q(7), PauliFactor::Z),
            (q(1), PauliFactor::X),
            (q(4), PauliFactor::Y),
        ])
        .expect("valid Pauli product");

        assert_eq!(
            product.factors(),
            &[
                (q(1), PauliFactor::X),
                (q(4), PauliFactor::Y),
                (q(7), PauliFactor::Z),
            ]
        );
    }

    #[test]
    fn duplicate_pauli_qubit_is_rejected() {
        let result = PauliProduct::new(vec![
            (q(0), PauliFactor::X),
            (q(0), PauliFactor::Z),
        ]);

        assert!(matches!(
            result,
            Err(HamiltonianError::DuplicateQubit {
                qubit
            }) if qubit == q(0)
        ));
    }

    #[test]
    fn tensor_product_rejects_overlapping_targets() {
        let left =
            PauliProduct::single(q(0), PauliFactor::X);

        let right =
            PauliProduct::single(q(0), PauliFactor::Z);

        assert!(matches!(
            left.tensor_product(&right),
            Err(HamiltonianError::DuplicateQubit {
                qubit
            }) if qubit == q(0)
        ));
    }

    #[test]
    fn tensor_product_accepts_disjoint_targets() {
        let left =
            PauliProduct::single(q(0), PauliFactor::X);

        let right =
            PauliProduct::single(q(1), PauliFactor::Z);

        let combined = left
            .tensor_product(&right)
            .expect("disjoint products");

        assert_eq!(combined.len(), 2);
        assert_eq!(
            combined.factor_at(q(0)),
            Some(PauliFactor::X)
        );
        assert_eq!(
            combined.factor_at(q(1)),
            Some(PauliFactor::Z)
        );
    }

    #[test]
    fn generic_operator_arity_is_checked() {
        let descriptor = OperatorDescriptor::new(
            "test",
            "operator",
            2,
            Hermiticity::Hermitian,
        )
        .expect("valid descriptor");

        let result =
            HamiltonianOperator::generic(
                descriptor,
                vec![q(0)],
            );

        assert!(matches!(
            result,
            Err(HamiltonianError::TargetArityMismatch {
                declared: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn generic_operator_targets_are_canonicalized() {
        let descriptor = OperatorDescriptor::new(
            "test",
            "operator",
            3,
            Hermiticity::Hermitian,
        )
        .expect("valid descriptor");

        let operator =
            HamiltonianOperator::generic(
                descriptor,
                vec![q(9), q(1), q(5)],
            )
            .expect("valid operator");

        assert_eq!(
            operator.qubits(),
            vec![q(1), q(5), q(9)]
        );
    }

    #[test]
    fn generic_duplicate_target_is_rejected() {
        let descriptor = OperatorDescriptor::new(
            "test",
            "operator",
            2,
            Hermiticity::Hermitian,
        )
        .expect("valid descriptor");

        let result =
            HamiltonianOperator::generic(
                descriptor,
                vec![q(1), q(1)],
            );

        assert!(matches!(
            result,
            Err(
                HamiltonianError::DuplicateOperatorTarget {
                    qubit
                }
            ) if qubit == q(1)
        ));
    }

    #[test]
    fn single_qubit_term_helper_uses_canonical_qubit_id() {
        let coefficient =
            Parameter::constant(0.5)
                .expect("finite parameter");

        let term = pauli_term(
            coefficient,
            q(3),
            PauliFactor::Z,
        );

        assert_eq!(
            term.qubits(),
            vec![q(3)]
        );
        assert!(term.is_hermitian());
    }

    #[test]
    fn pauli_hamiltonian_is_known_hermitian() {
        let coefficient =
            Parameter::constant(0.5)
                .expect("finite parameter");

        let term = pauli_product_term(
            coefficient,
            vec![
                (q(0), PauliFactor::Z),
                (q(1), PauliFactor::Z),
            ],
        )
        .expect("valid term");

        let hamiltonian =
            Hamiltonian::single(term);

        assert!(hamiltonian.is_known_hermitian());
        assert!(!hamiltonian.contains_non_hermitian_term());
        assert_eq!(hamiltonian.qubit_count(), 2);
        assert_eq!(hamiltonian.max_locality(), 2);
        assert!(hamiltonian.is_pauli_only());

        hamiltonian
            .validate_hermitian()
            .expect("Pauli Hamiltonian is Hermitian");
    }

    #[test]
    fn symbolic_coefficient_is_supported() {
        let coefficient =
            Parameter::symbol("J")
                .expect("valid symbol");

        let term = pauli_term(
            coefficient,
            q(0),
            PauliFactor::Z,
        );

        let hamiltonian =
            Hamiltonian::single(term);

        hamiltonian
            .validate()
            .expect("symbolic coefficient is valid");
    }

    #[test]
    fn symbolic_time_dependent_coefficient_is_supported() {
        let coefficient =
            Parameter::symbol("t")
                .expect("valid symbol");

        let term = pauli_product_term(
            coefficient,
            vec![
                (q(0), PauliFactor::X),
                (q(1), PauliFactor::X),
            ],
        )
        .expect("valid term");

        let hamiltonian =
            Hamiltonian::single(term);

        assert!(hamiltonian.is_known_hermitian());
        assert_eq!(hamiltonian.max_locality(), 2);
    }

    #[test]
    fn zero_hamiltonian_is_valid() {
        let hamiltonian = Hamiltonian::zero();

        assert!(hamiltonian.is_empty());
        assert!(hamiltonian.is_known_hermitian());
        assert_eq!(hamiltonian.qubit_count(), 0);
        assert_eq!(hamiltonian.max_locality(), 0);

        hamiltonian
            .validate_hermitian()
            .expect("zero Hamiltonian is Hermitian");
    }

    #[test]
    fn qubit_collection_is_unique_and_deterministic() {
        let coefficient =
            Parameter::constant(1.0)
                .expect("finite parameter");

        let first = pauli_product_term(
            coefficient.clone(),
            vec![
                (q(8), PauliFactor::X),
                (q(2), PauliFactor::Z),
            ],
        )
        .expect("valid term");

        let second = pauli_product_term(
            coefficient,
            vec![
                (q(5), PauliFactor::Y),
                (q(2), PauliFactor::X),
            ],
        )
        .expect("valid term");

        let hamiltonian =
            Hamiltonian::new(vec![first, second]);

        assert_eq!(
            hamiltonian.qubits(),
            vec![q(2), q(5), q(8)]
        );

        assert_eq!(hamiltonian.qubit_count(), 3);
    }

    #[test]
    fn unknown_hermiticity_is_distinguished_from_non_hermitian() {
        let descriptor = OperatorDescriptor::new(
            "custom",
            "operator",
            1,
            Hermiticity::Unknown,
        )
        .expect("valid descriptor");

        let coefficient =
            Parameter::constant(1.0)
                .expect("finite parameter");

        let term =
            HamiltonianTerm::generic(
                coefficient,
                descriptor,
                vec![q(0)],
            )
            .expect("valid term");

        let hamiltonian =
            Hamiltonian::single(term);

        assert!(!hamiltonian.is_known_hermitian());
        assert!(!hamiltonian.contains_non_hermitian_term());
        assert!(hamiltonian.contains_unknown_hermiticity());

        assert!(matches!(
            hamiltonian.validate_hermitian(),
            Err(HamiltonianError::NonHermitianTerm)
        ));
    }

    #[test]
    fn explicitly_non_hermitian_operator_is_rejected_by_strict_policy() {
        let descriptor = OperatorDescriptor::new(
            "custom",
            "operator",
            1,
            Hermiticity::NonHermitian,
        )
        .expect("valid descriptor");

        let coefficient =
            Parameter::constant(1.0)
                .expect("finite parameter");

        let term =
            HamiltonianTerm::generic(
                coefficient,
                descriptor,
                vec![q(0)],
            )
            .expect("valid term");

        let hamiltonian =
            Hamiltonian::single(term);

        assert!(hamiltonian.contains_non_hermitian_term());

        assert!(matches!(
            hamiltonian.validate_hermitian(),
            Err(HamiltonianError::NonHermitianTerm)
        ));
    }

    #[test]
    fn unrestricted_policy_can_represent_unknown_generic_operator() {
        let descriptor = OperatorDescriptor::new(
            "future",
            "operator",
            2,
            Hermiticity::Unknown,
        )
        .expect("valid descriptor");

        let coefficient =
            Parameter::symbol("g")
                .expect("valid parameter");

        let term =
            HamiltonianTerm::generic(
                coefficient,
                descriptor,
                vec![q(0), q(7)],
            )
            .expect("valid term");

        let hamiltonian =
            Hamiltonian::single(term);

        hamiltonian
            .validate()
            .expect("unrestricted semantic validation");
    }

    #[test]
    fn no_fixed_qubit_limit_is_encoded() {
        let high_id =
            QubitId::new(usize::MAX);

        let product =
            PauliProduct::single(
                high_id,
                PauliFactor::Z,
            );

        assert_eq!(
            product.qubits(),
            vec![high_id]
        );
    }

    #[test]
    fn helpers_support_many_terms_without_semantic_limit() {
        let mut hamiltonian =
            Hamiltonian::zero();

        for index in 0..1024usize {
            let coefficient =
                Parameter::constant(1.0)
                    .expect("finite coefficient");

            hamiltonian.push(
                pauli_term(
                    coefficient,
                    q(index),
                    PauliFactor::Z,
                ),
            );
        }

        assert_eq!(hamiltonian.len(), 1024);
        assert_eq!(hamiltonian.qubit_count(), 1024);
        assert_eq!(hamiltonian.max_locality(), 1);
    }

    #[test]
    fn display_is_deterministic_for_pauli_product() {
        let product = PauliProduct::new(vec![
            (q(5), PauliFactor::Z),
            (q(1), PauliFactor::X),
        ])
        .expect("valid product");

        assert_eq!(
            product.to_string(),
            "X(q1) Z(q5)"
        );
    }
}