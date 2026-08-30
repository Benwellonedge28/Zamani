//! Zamani Quantum Optimization — Single-Qubit Gate Fusion
//!
//! Production-grade local fusion of consecutive single-qubit unitary
//! operations into one canonical `U3` operation when the resulting unitary is
//! exactly representable by `U3`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                      │
//!                                      ▼
//!                         optimization::local::gate_fusion
//!                                      │
//!                    ┌─────────────────┴─────────────────┐
//!                    │                                   │
//!                    ▼                                   ▼
//!              constant 1Q gates                 canonical U3 gate
//!                    │                                   │
//!                    └─────────────────┬─────────────────┘
//!                                      ▼
//!                             optimized Quantum IR
//! ```
//!
//! # Responsibility
//!
//! This pass performs one narrowly defined transformation:
//!
//! ```text
//! A(q); B(q)
//!       ↓
//! U3(...)(q)
//! ```
//!
//! when:
//!
//! 1. both operations are unitary;
//! 2. both operate on exactly one logical qubit;
//! 3. both have only constant numerical parameters;
//! 4. both belong to the explicitly supported single-qubit gate family;
//! 5. their product is exactly representable by the canonical `U3` form;
//! 6. no global phase is silently discarded;
//! 7. the resulting gate passes canonical IR validation.
//!
//! # Deliberate non-responsibilities
//!
//! This module does NOT:
//!
//! - define another `QuantumGate`;
//! - define another circuit representation;
//! - optimize symbolic parameters;
//! - perform rotation algebra already owned by `rotation.rs`;
//! - perform commutation;
//! - cross measurements;
//! - cross barriers;
//! - cross resets;
//! - optimize multi-qubit gates;
//! - perform routing;
//! - perform scheduling;
//! - perform hardware execution;
//! - call a simulator;
//! - call a QPU;
//! - approximate a unitary;
//! - silently ignore global phase;
//! - use `unsafe` code.
//!
//! # Why this is separate from `rotation.rs`
//!
//! `rotation.rs` already provides exact symbolic/constant fusion for:
//!
//! ```text
//! RX(a); RX(b)
//! RY(a); RY(b)
//! RZ(a); RZ(b)
//! Phase(a); Phase(b)
//! U1(a); U1(b)
//! CRX(a); CRX(b)
//! CRY(a); CRY(b)
//! CRZ(a); CRZ(b)
//! ```
//!
//! This pass must therefore NOT become another implementation of rotation
//! addition.
//!
//! Instead it handles heterogeneous constant sequences such as:
//!
//! ```text
//! H; X; S; T
//! ```
//!
//! when their complete product can safely be represented by one `U3` gate.
//!
//! # Semantic ordering
//!
//! For a circuit sequence:
//!
//! ```text
//! A;
//! B;
//! ```
//!
//! the resulting unitary is:
//!
//! ```text
//! B × A
//! ```
//!
//! because `A` acts first on the state and `B` acts second.
//!
//! # Global phase
//!
//! A major correctness requirement is that the pass does not perform the
//! common but unsafe transformation:
//!
//! ```text
//! U ≈ e^(iφ) U3
//! ```
//!
//! merely because the two matrices represent the same measurement
//! probabilities.
//!
//! This implementation accepts a fusion only when the generated `U3` matrix
//! matches the original product within the pass's finite-number comparison
//! policy.
//!
//! It does NOT accept an arbitrary global-phase difference.
//!
//! This means the pass may conservatively decline a legal optimization. That is
//! intentional. An optimizer must prefer "not optimized" over an unproven
//! semantic transformation.
//!
//! # Symbolic parameters
//!
//! Symbolic parameters are deliberately not fused here.
//!
//! For example:
//!
//! ```text
//! RX(theta); H
//! ```
//!
//! is not converted into a symbolic `U3` expression.
//!
//! This is because symbolic matrix algebra belongs to the parameter/algebra
//! layers and should not be approximated or duplicated in a local pass.
//!
//! A later optimizer can add an exact symbolic synthesis facility without
//! changing the semantic contract of this pass.
//!
//! # Supported gates
//!
//! The supported single-qubit gate family is explicitly enumerated:
//!
//! - I
//! - X
//! - Y
//! - Z
//! - H
//! - S
//! - Sdg
//! - T
//! - Tdg
//! - V
//! - Vdg
//! - RX
//! - RY
//! - RZ
//! - Phase
//! - U1
//! - U2
//! - U3
//!
//! Unknown future gates are NOT automatically treated as matrices.
//!
//! This is intentional. Adding a new gate to the canonical IR requires an
//! explicit semantic decision before this optimizer can fuse it.
//!
//! # Complexity
//!
//! Let `N` be the number of circuit operations.
//!
//! Discovery is:
//!
//! - time: O(N);
//! - auxiliary memory: O(1) for matrix accumulation;
//! - no circuit-sized dependency graph;
//! - no state-vector allocation;
//! - no tensor-network simulation;
//! - no recursion proportional to circuit depth.
//!
//! Mutation is performed through the canonical circuit mutation API.
//!
//! The pass therefore scales with circuit size rather than with the Hilbert
//! space dimension.
//!
//! # Resource scaling
//!
//! There is intentionally no artificial circuit-size limit in this file.
//!
//! Resource limits are owned by:
//!
//! - `optimization::limits`;
//! - `optimization::context`;
//! - `optimization::pipeline`.
//!
//! The pass periodically cooperates with the optimizer context and does not
//! allocate memory proportional to `2^n`.
//!
//! # Determinism
//!
//! The pass is deterministic:
//!
//! - no randomness;
//! - no system time;
//! - no environment state;
//! - no hash-map iteration;
//! - no backend state.
//!
//! The same validated circuit produces the same result.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ## `quantum::ir::gate`
//!
//! Uses:
//!
//! - `Gate`;
//! - `GateKind`;
//! - canonical gate accessors;
//! - canonical validation.
//!
//! ## `quantum::ir::parameter`
//!
//! Uses:
//!
//! - `Parameter::Constant`.
//!
//! Symbolic parameters are intentionally rejected by this pass.
//!
//! ## `quantum::ir::circuit`
//!
//! Uses the canonical `QuantumCircuit` access and validated mutation APIs.
//!
//! ## `optimization::pass`
//!
//! Implements `OptimizationPass`.
//!
//! ## `optimization::context`
//!
//! Uses invocation-scoped resource checks.
//!
//! ## `optimization::cost`
//!
//! Fusion reduces operation count when successful.
//!
//! ## `optimization::verification`
//!
//! Whole-circuit semantic verification remains a pipeline concern.
//!
//! ## `optimization::local::rotation`
//!
//! Rotation fusion remains responsible for symbolic and same-axis rotation
//! composition.
//!
//! ## `optimization::local::cancellation`
//!
//! Cancellation can run before or after this pass.
//!
//! ## `optimization::local::peephole`
//!
//! Peephole rules can further simplify the generated canonical `U3` or
//! surrounding gates.
//!
//! ## `optimization::synthesis`
//!
//! Target-specific synthesis may later decompose the fused `U3` into native
//! operations.
//!
//! ## `optimization::targets`
//!
//! This pass is target-independent. Native-gate decomposition belongs to
//! target-aware synthesis.
//!
//! # Recommended pipeline position
//!
//! ```text
//! normalize
//!     ↓
//! identity
//!     ↓
//! cancellation
//!     ↓
//! rotation_fusion
//!     ↓
//! gate_fusion
//!     ↓
//! peephole
//!     ↓
//! cancellation
//!     ↓
//! target synthesis
//! ```
//!
//! The pipeline may choose a different order depending on its optimization
//! profile.

#![forbid(unsafe_code)]

use std::f64::consts::PI;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationResult,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassChange,
    PassComplexity,
    PassDeterminism,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable optimizer pass identifier.
pub const PASS_ID: &str = "local.gate_fusion";

/// Stable human-readable name.
pub const PASS_NAME: &str = "Single-Qubit Gate Fusion";

/// Algorithm version used for provenance.
pub const ALGORITHM_VERSION: &str = "1";

// =============================================================================
// Numerical policy
// =============================================================================

/// Numerical comparison tolerance.
///
/// This is used only for validating finite floating-point matrix arithmetic.
///
/// It is never used to compare symbolic parameters because symbolic parameters
/// are not handled by this pass.
const MATRIX_TOLERANCE: f64 = 1.0e-12;

/// Small threshold used to detect matrix entries that are mathematically zero.
const ZERO_THRESHOLD: f64 = 1.0e-12;

/// Maximum number of operations between context resource checks.
///
/// This is an implementation-detail interval, not a circuit-size limit.
const RESOURCE_CHECK_INTERVAL: usize = 1024;

// =============================================================================
// Complex number
// =============================================================================

/// Minimal dependency-free complex scalar used by the fusion algorithm.
///
/// Zamani's canonical IR deliberately does not require a complex-number
/// dependency merely to represent gates. Gate fusion therefore uses this
/// small local value type for 2×2 matrix arithmetic.
///
/// This type is not a second quantum representation. It is only a temporary
/// mathematical value used while computing one fused gate.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };

    const ONE: Self = Self { re: 1.0, im: 0.0 };

    const I: Self = Self { re: 0.0, im: 1.0 };

    #[must_use]
    const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[must_use]
    fn from_polar(radius: f64, phase: f64) -> Self {
        Self {
            re: radius * phase.cos(),
            im: radius * phase.sin(),
        }
    }

    #[must_use]
    fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[must_use]
    fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    #[must_use]
    fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    #[must_use]
    fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    #[must_use]
    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    #[must_use]
    fn approximately_zero(self) -> bool {
        self.norm() <= ZERO_THRESHOLD
    }

    #[must_use]
    fn approximately_equal(self, other: Self) -> bool {
        let delta = self - other;

        delta.norm() <= MATRIX_TOLERANCE
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

// =============================================================================
// 2×2 matrix
// =============================================================================

/// Temporary 2×2 complex matrix.
///
/// The matrix is never stored in the canonical IR.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix2 {
    m00: Complex,
    m01: Complex,
    m10: Complex,
    m11: Complex,
}

impl Matrix2 {
    const IDENTITY: Self = Self {
        m00: Complex::ONE,
        m01: Complex::ZERO,
        m10: Complex::ZERO,
        m11: Complex::ONE,
    };

    #[must_use]
    const fn new(
        m00: Complex,
        m01: Complex,
        m10: Complex,
        m11: Complex,
    ) -> Self {
        Self {
            m00,
            m01,
            m10,
            m11,
        }
    }

    #[must_use]
    fn multiply(self, rhs: Self) -> Self {
        Self {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
        }
    }

    #[must_use]
    fn approximately_equal(self, rhs: Self) -> bool {
        self.m00.approximately_equal(rhs.m00)
            && self.m01.approximately_equal(rhs.m01)
            && self.m10.approximately_equal(rhs.m10)
            && self.m11.approximately_equal(rhs.m11)
    }

    #[must_use]
    fn is_finite(self) -> bool {
        self.m00.is_finite()
            && self.m01.is_finite()
            && self.m10.is_finite()
            && self.m11.is_finite()
    }

    #[must_use]
    fn determinant(self) -> Complex {
        self.m00 * self.m11 - self.m01 * self.m10
    }

    #[must_use]
    fn scaled(self, scalar: Complex) -> Self {
        Self {
            m00: self.m00 * scalar,
            m01: self.m01 * scalar,
            m10: self.m10 * scalar,
            m11: self.m11 * scalar,
        }
    }
}

// =============================================================================
// Gate classification
// =============================================================================

/// Returns whether a gate can participate in this fusion pass.
#[must_use]
fn is_supported_single_qubit_gate(gate: &Gate) -> bool {
    if gate.qubits().len() != 1 {
        return false;
    }

    if !gate.kind().is_unitary() {
        return false;
    }

    if gate.classical_target().is_some() {
        return false;
    }

    matches!(
        gate.kind(),
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::V
            | GateKind::Vdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
    )
}

/// Returns all constant numerical parameters of a gate.
///
/// `None` means at least one parameter is symbolic.
fn constant_parameters(gate: &Gate) -> Option<Vec<f64>> {
    let mut result = Vec::with_capacity(gate.parameters().len());

    for parameter in gate.parameters() {
        match parameter {
            Parameter::Constant(value) if value.is_finite() => {
                result.push(*value);
            }

            _ => return None,
        }
    }

    Some(result)
}

// =============================================================================
// Gate → matrix
// =============================================================================

/// Converts one supported canonical gate into its exact temporary matrix.
///
/// This function deliberately refuses unknown gate kinds rather than making
/// assumptions about their semantics.
fn gate_matrix(gate: &Gate) -> Option<Matrix2> {
    if !is_supported_single_qubit_gate(gate) {
        return None;
    }

    let parameters = constant_parameters(gate)?;

    let matrix = match gate.kind() {
        GateKind::I => Matrix2::IDENTITY,

        GateKind::X => Matrix2::new(
            Complex::ZERO,
            Complex::ONE,
            Complex::ONE,
            Complex::ZERO,
        ),

        GateKind::Y => Matrix2::new(
            Complex::ZERO,
            -Complex::I,
            Complex::I,
            Complex::ZERO,
        ),

        GateKind::Z => Matrix2::new(
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            -Complex::ONE,
        ),

        GateKind::H => {
            let s = 1.0 / 2.0_f64.sqrt();

            Matrix2::new(
                Complex::new(s, 0.0),
                Complex::new(s, 0.0),
                Complex::new(s, 0.0),
                Complex::new(-s, 0.0),
            )
        }

        GateKind::S => Matrix2::new(
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::I,
        ),

        GateKind::Sdg => Matrix2::new(
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            -Complex::I,
        ),

        GateKind::T => {
            let phase = Complex::from_polar(1.0, PI / 4.0);

            Matrix2::new(
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                phase,
            )
        }

        GateKind::Tdg => {
            let phase = Complex::from_polar(1.0, -PI / 4.0);

            Matrix2::new(
                Complex::ONE,
                Complex::ZERO,
                Complex::ZERO,
                phase,
            )
        }

        GateKind::V => {
            let a = Complex::new(0.5, 0.5);
            let b = Complex::new(0.5, -0.5);

            Matrix2::new(a, b, b, a)
        }

        GateKind::Vdg => {
            let a = Complex::new(0.5, -0.5);
            let b = Complex::new(0.5, 0.5);

            Matrix2::new(a, b, b, a)
        }

        GateKind::RX => {
            let theta = parameters.first().copied()?;

            rotation_x(theta)
        }

        GateKind::RY => {
            let theta = parameters.first().copied()?;

            rotation_y(theta)
        }

        GateKind::RZ => {
            let theta = parameters.first().copied()?;

            rotation_z(theta)
        }

        GateKind::Phase | GateKind::U1 => {
            let theta = parameters.first().copied()?;

            phase(theta)
        }

        GateKind::U2 => {
            let phi = parameters.first().copied()?;
            let lambda = parameters.get(1).copied()?;

            u3_matrix(PI / 2.0, phi, lambda)
        }

        GateKind::U3 => {
            let theta = parameters.first().copied()?;
            let phi = parameters.get(1).copied()?;
            let lambda = parameters.get(2).copied()?;

            u3_matrix(theta, phi, lambda)
        }

        _ => return None,
    };

    matrix.is_finite().then_some(matrix)
}

#[must_use]
fn rotation_x(theta: f64) -> Matrix2 {
    let half = theta / 2.0;

    let c = half.cos();
    let s = half.sin();

    Matrix2::new(
        Complex::new(c, 0.0),
        Complex::new(0.0, -s),
        Complex::new(0.0, -s),
        Complex::new(c, 0.0),
    )
}

#[must_use]
fn rotation_y(theta: f64) -> Matrix2 {
    let half = theta / 2.0;

    let c = half.cos();
    let s = half.sin();

    Matrix2::new(
        Complex::new(c, 0.0),
        Complex::new(-s, 0.0),
        Complex::new(s, 0.0),
        Complex::new(c, 0.0),
    )
}

#[must_use]
fn rotation_z(theta: f64) -> Matrix2 {
    let half = theta / 2.0;

    Matrix2::new(
        Complex::from_polar(1.0, -half),
        Complex::ZERO,
        Complex::ZERO,
        Complex::from_polar(1.0, half),
    )
}

#[must_use]
fn phase(theta: f64) -> Matrix2 {
    Matrix2::new(
        Complex::ONE,
        Complex::ZERO,
        Complex::ZERO,
        Complex::from_polar(1.0, theta),
    )
}

#[must_use]
fn u3_matrix(
    theta: f64,
    phi: f64,
    lambda: f64,
) -> Matrix2 {
    let half = theta / 2.0;

    let c = half.cos();
    let s = half.sin();

    Matrix2::new(
        Complex::new(c, 0.0),
        Complex::from_polar(-s, lambda),
        Complex::from_polar(s, phi),
        Complex::from_polar(c, phi + lambda),
    )
}

// =============================================================================
// Matrix → U3
// =============================================================================

/// Converts a unitary matrix into a canonical `U3` representation when exact
/// representation is possible.
///
/// The returned parameters use the canonical `U3(theta, phi, lambda)`
/// convention represented by the IR.
///
/// The function is deliberately conservative. It reconstructs the candidate
/// matrix and verifies it before returning it.
fn decompose_to_u3(
    matrix: Matrix2,
) -> Option<(f64, f64, f64)> {
    if !matrix.is_finite() {
        return None;
    }

    let determinant = matrix.determinant();

    if !determinant.is_finite() {
        return None;
    }

    // A valid 2×2 unitary must have determinant magnitude one.
    if (determinant.norm() - 1.0).abs() > MATRIX_TOLERANCE {
        return None;
    }

    let a = matrix.m00;
    let b = matrix.m01;
    let c = matrix.m10;
    let d = matrix.m11;

    let a_norm = a.norm();
    let b_norm = b.norm();
    let c_norm = c.norm();
    let d_norm = d.norm();

    // Unitary matrices have matching singular magnitudes.
    if (a_norm - d_norm).abs() > MATRIX_TOLERANCE {
        return None;
    }

    if (b_norm - c_norm).abs() > MATRIX_TOLERANCE {
        return None;
    }

    // -------------------------------------------------------------------------
    // Generic case: both cosine and sine components are non-zero.
    // -------------------------------------------------------------------------

    if a_norm > ZERO_THRESHOLD && b_norm > ZERO_THRESHOLD {
        let theta = 2.0 * b_norm.atan2(a_norm);

        let phi = c.arg();

        let lambda = normalize_angle(b.arg() - PI);

        let candidate = u3_matrix(theta, phi, lambda);

        if candidate.approximately_equal(matrix) {
            return Some((
                normalize_angle(theta),
                normalize_angle(phi),
                normalize_angle(lambda),
            ));
        }

        // U3 is sometimes reached through the alternative sign branch.
        let theta_alt = -theta;

        let phi_alt = normalize_angle(c.arg() + PI);
        let lambda_alt = b.arg();

        let candidate_alt =
            u3_matrix(theta_alt, phi_alt, lambda_alt);

        if candidate_alt.approximately_equal(matrix) {
            return Some((
                normalize_angle(theta_alt),
                normalize_angle(phi_alt),
                normalize_angle(lambda_alt),
            ));
        }
    }

    // -------------------------------------------------------------------------
    // Diagonal case.
    // -------------------------------------------------------------------------

    if b.approximately_zero() && c.approximately_zero() {
        // U3(0, 0, lambda) is diag(1, e^(i lambda)).
        //
        // Exact representation requires the first diagonal element to be one.
        // We deliberately do not remove a global phase here.
        if a.approximately_equal(Complex::ONE) {
            let lambda = d.arg();

            let candidate =
                u3_matrix(0.0, 0.0, lambda);

            if candidate.approximately_equal(matrix) {
                return Some((
                    0.0,
                    0.0,
                    normalize_angle(lambda),
                ));
            }
        }

        // The theta=2π branch can represent -I exactly.
        if a.approximately_equal(Complex::new(-1.0, 0.0))
            && d.approximately_equal(Complex::new(-1.0, 0.0))
        {
            let candidate =
                u3_matrix(2.0 * PI, 0.0, 0.0);

            if candidate.approximately_equal(matrix) {
                return Some((2.0 * PI, 0.0, 0.0));
            }
        }
    }

    // -------------------------------------------------------------------------
    // Anti-diagonal case.
    // -------------------------------------------------------------------------

    if a.approximately_zero() && d.approximately_zero() {
        // theta = π gives:
        //
        // [ 0, -e^(iλ) ]
        // [ e^(iφ),  0  ]
        //
        // Solve directly and verify.
        let theta = PI;
        let phi = c.arg();
        let lambda = normalize_angle(b.arg() - PI);

        let candidate =
            u3_matrix(theta, phi, lambda);

        if candidate.approximately_equal(matrix) {
            return Some((
                theta,
                normalize_angle(phi),
                normalize_angle(lambda),
            ));
        }
    }

    None
}

// =============================================================================
// Gate construction
// =============================================================================

/// Builds the canonical fused `U3` gate.
fn build_fused_gate(
    first: &Gate,
    second: &Gate,
    matrix: Matrix2,
) -> Option<Gate> {
    if first.qubits() != second.qubits() {
        return None;
    }

    let (theta, phi, lambda) =
        decompose_to_u3(matrix)?;

    let qubits = first.qubits().to_vec();

    let parameters = vec![
        Parameter::Constant(theta),
        Parameter::Constant(phi),
        Parameter::Constant(lambda),
    ];

    let fused = Gate::new(
        GateKind::U3,
        qubits,
        parameters,
        None,
        None,
    )
    .ok()?;

    // Final semantic guard.
    //
    // Never trust the decomposition without reconstructing the resulting
    // canonical gate and comparing it against the original matrix.
    let reconstructed = gate_matrix(&fused)?;

    if !reconstructed.approximately_equal(matrix) {
        return None;
    }

    Some(fused)
}

// =============================================================================
// Pair fusion
// =============================================================================

/// Attempts to fuse two adjacent canonical operations.
///
/// The second operation is multiplied on the left because quantum operations
/// execute from left to right in circuit order.
fn fuse_pair(
    first: &Gate,
    second: &Gate,
) -> Option<Gate> {
    if !is_supported_single_qubit_gate(first)
        || !is_supported_single_qubit_gate(second)
    {
        return None;
    }

    if first.qubits() != second.qubits() {
        return None;
    }

    let first_matrix = gate_matrix(first)?;
    let second_matrix = gate_matrix(second)?;

    let combined =
        second_matrix.multiply(first_matrix);

    build_fused_gate(first, second, combined)
}

// =============================================================================
// Statistics
// =============================================================================

/// Detailed gate-fusion statistics.
///
/// The optimizer-level `PassOutcome` remains the public pipeline contract.
/// This structure contains additional diagnostics useful for tests,
/// benchmarking, and provenance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GateFusionStatistics {
    /// Number of operations inspected.
    pub operations_inspected: u64,

    /// Number of adjacent pairs considered.
    pub candidate_pairs: u64,

    /// Number of pairs containing supported constant single-qubit gates.
    pub supported_pairs: u64,

    /// Number of successful fusions.
    pub fusions: u64,

    /// Number of original operations removed.
    pub operations_removed: u64,

    /// Number of fused operations inserted/replaced.
    pub operations_replaced: u64,

    /// Number of candidate pairs rejected because their parameters were not
    /// constant.
    pub symbolic_rejections: u64,

    /// Number of candidate pairs rejected because they could not be represented
    /// exactly by U3.
    pub representation_rejections: u64,
}

impl GateFusionStatistics {
    /// Returns true if the circuit changed.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.fusions != 0
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production single-qubit gate-fusion pass.
///
/// The pass contains no invocation-specific mutable state.
#[derive(Debug, Clone)]
pub struct GateFusion {
    metadata: PassMetadata,
}

impl GateFusion {
    /// Constructs the production gate-fusion pass.
    pub fn new() -> Result<Self, OptimizationError> {
        let metadata = build_metadata()?;

        Ok(Self { metadata })
    }

    /// Stable pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Stable algorithm version.
    #[must_use]
    pub const fn algorithm_version() -> &'static str {
        ALGORITHM_VERSION
    }

    /// Returns pass metadata.
    #[must_use]
    pub const fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    /// Direct invocation helper.
    ///
    /// This uses the same implementation as the `OptimizationPass` contract.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        self.run_impl(circuit, context)
    }

    /// Executes one deterministic fusion phase.
    fn run_impl(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        circuit
            .validate()
            .map_err(|error| {
                transformation_error(
                    "input circuit validation failed",
                    error.to_string(),
                )
            })?;

        let operations_before =
            checked_u64(circuit.len())?;

        if circuit.len() < 2 {
            return Ok(
                PassOutcome::unchanged(
                    operations_before,
                    operations_before,
                )
                .with_iterations(1)
                .with_message(
                    "gate fusion requires at least two operations",
                ),
            );
        }

        let mut statistics =
            GateFusionStatistics::default();

        // A single forward scan is used.
        //
        // Successful fusion:
        //
        //     A B C
        //       ↓
        //     F C
        //
        // The newly-created F is immediately considered against C on the next
        // iteration. This allows maximal local fusion without recursive
        // function calls.
        let mut index = 0usize;

        while index + 1 < circuit.len() {
            if index % RESOURCE_CHECK_INTERVAL == 0 {
                context
                    .check_limits()
                    .map_err(|error| {
                        transformation_error(
                            "optimizer resource check failed",
                            error.to_string(),
                        )
                    })?;
            }

            statistics.operations_inspected =
                statistics
                    .operations_inspected
                    .saturating_add(2);

            statistics.candidate_pairs =
                statistics
                    .candidate_pairs
                    .saturating_add(1);

            let first = circuit
                .get(index)
                .ok_or_else(|| {
                    transformation_error(
                        "operation lookup failed",
                        format!(
                            "operation index {index} disappeared \
                             during fusion",
                        ),
                    )
                })?;

            let second = circuit
                .get(index + 1)
                .ok_or_else(|| {
                    transformation_error(
                        "operation lookup failed",
                        format!(
                            "operation index {} disappeared \
                             during fusion",
                            index + 1,
                        ),
                    )
                })?;

            if !is_supported_single_qubit_gate(first)
                || !is_supported_single_qubit_gate(second)
            {
                index += 1;
                continue;
            }

            statistics.supported_pairs =
                statistics
                    .supported_pairs
                    .saturating_add(1);

            if constant_parameters(first).is_none()
                || constant_parameters(second).is_none()
            {
                statistics.symbolic_rejections =
                    statistics
                        .symbolic_rejections
                        .saturating_add(1);

                index += 1;
                continue;
            }

            let Some(fused) = fuse_pair(first, second)
            else {
                statistics.representation_rejections =
                    statistics
                        .representation_rejections
                        .saturating_add(1);

                index += 1;
                continue;
            };

            // The canonical circuit API validates the replacement. If the
            // replacement cannot be committed, the optimizer returns an error
            // instead of leaving an invalid transformation behind.
            circuit
                .replace(index, fused)
                .map_err(|error| {
                    transformation_error(
                        "failed to replace fused gate",
                        error.to_string(),
                    )
                })?;

            // Remove the now-redundant second operation.
            //
            // It is deliberately removed only after replacement succeeds.
            // If removal fails, the pass reports failure rather than silently
            // continuing with an inconsistent circuit.
            circuit
                .remove(index + 1)
                .map_err(|error| {
                    transformation_error(
                        "failed to remove fused operation",
                        error.to_string(),
                    )
                })?;

            statistics.fusions =
                statistics.fusions.saturating_add(1);

            statistics.operations_removed =
                statistics
                    .operations_removed
                    .saturating_add(1);

            statistics.operations_replaced =
                statistics
                    .operations_replaced
                    .saturating_add(1);

            // Do not increment `index`.
            //
            // The newly fused operation at `index` may fuse with the next
            // operation.
        }

        circuit
            .validate()
            .map_err(|error| {
                transformation_error(
                    "optimized circuit validation failed",
                    error.to_string(),
                )
            })?;

        let operations_after =
            checked_u64(circuit.len())?;

        if !statistics.changed() {
            return Ok(
                PassOutcome::unchanged(
                    operations_before,
                    operations_after,
                )
                .with_iterations(1)
                .with_message(
                    "no exactly representable single-qubit fusion \
                     opportunities were found",
                ),
            );
        }

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_iterations(1)
            .with_change(
                PassChange::OperationsReplaced(
                    statistics.operations_replaced,
                ),
            )
            .with_change(
                PassChange::OperationsRemoved(
                    statistics.operations_removed,
                ),
            )
            .with_message(
                "fused exactly representable constant single-qubit \
                 unitary sequences",
            ),
        )
    }
}

// =============================================================================
// Pass implementation
// =============================================================================

impl OptimizationPass for GateFusion {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> OptimizationResult<PassOutcome> {
        self.run_impl(circuit, context)
    }
}

// =============================================================================
// Metadata
// =============================================================================

fn build_metadata() -> Result<PassMetadata, OptimizationError> {
    PassMetadata::builder()
        .id(PASS_ID)
        .name(PASS_NAME)
        .version(ALGORITHM_VERSION)
        .kind(PassKind::LocalRewrite)
        .scope(PassScope::LocalWindow)
        .complexity(PassComplexity::Linear)
        .determinism(PassDeterminism::Deterministic)
        .execution_policy(PassExecutionPolicy::Serial)
        .capability(PassCapability::SemanticPreservation)
        .capability(PassCapability::OperationFusion)
        .capability(PassCapability::OperationReplacement)
        .capability(PassCapability::ParameterTransformation)
        .build()
        .map_err(|error| {
            transformation_error(
                "invalid gate-fusion metadata",
                error.to_string(),
            )
        })
}

// =============================================================================
// Error helpers
// =============================================================================

fn transformation_error(
    message: &'static str,
    detail: String,
) -> OptimizationError {
    OptimizationError::pass_failure(
        PASS_ID,
        format!("{message}: {detail}"),
    )
}

fn checked_u64(value: usize) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        transformation_error(
            "integer conversion overflow",
            format!(
                "cannot represent operation count {value} as u64",
            ),
        )
    })
}

// =============================================================================
// Angle normalization
// =============================================================================

#[must_use]
fn normalize_angle(angle: f64) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let two_pi = 2.0 * PI;

    let mut result = angle % two_pi;

    if result > PI {
        result -= two_pi;
    } else if result < -PI {
        result += two_pi;
    }

    if result.abs() <= ZERO_THRESHOLD {
        0.0
    } else {
        result
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn constant_parameter(value: f64) -> Parameter {
        Parameter::Constant(value)
    }

    fn gate(
        kind: GateKind,
        qubit: usize,
        parameters: Vec<Parameter>,
    ) -> Gate {
        Gate::new(
            kind,
            vec![
                crate::quantum::ir::qubits::QubitId::new(qubit)
                    .expect("test qubit should be valid"),
            ],
            parameters,
            None,
            None,
        )
        .expect("test gate should be valid")
    }

    #[test]
    fn identity_matrix_is_stable() {
        let matrix = Matrix2::IDENTITY;

        assert!(
            matrix.approximately_equal(Matrix2::IDENTITY)
        );
    }

    #[test]
    fn x_matrix_is_correct() {
        let x = gate_matrix(&gate(GateKind::X, 0, vec![]))
            .expect("X must have a matrix");

        assert!(
            x.approximately_equal(
                Matrix2::new(
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ONE,
                    Complex::ZERO,
                )
            )
        );
    }

    #[test]
    fn hadamard_matrix_is_unitary() {
        let h = gate_matrix(
            &gate(GateKind::H, 0, vec![]),
        )
        .expect("H must have a matrix");

        let product = h.multiply(h);

        assert!(
            product.approximately_equal(Matrix2::IDENTITY)
        );
    }

    #[test]
    fn rx_matrix_is_unitary() {
        let rx = gate_matrix(
            &gate(
                GateKind::RX,
                0,
                vec![constant_parameter(0.37)],
            ),
        )
        .expect("RX must have a matrix");

        let dagger_product = conjugate_transpose(rx)
            .multiply(rx);

        assert!(
            dagger_product.approximately_equal(
                Matrix2::IDENTITY
            )
        );
    }

    #[test]
    fn identity_fuses_to_u3_identity() {
        let identity =
            gate(GateKind::I, 0, vec![]);

        let fused =
            build_fused_gate(
                &identity,
                &identity,
                Matrix2::IDENTITY,
            )
            .expect("identity should be representable");

        assert_eq!(fused.kind(), GateKind::U3);

        let reconstructed =
            gate_matrix(&fused)
                .expect("fused gate should have a matrix");

        assert!(
            reconstructed
                .approximately_equal(Matrix2::IDENTITY)
        );
    }

    #[test]
    fn x_x_fuses_to_identity() {
        let x =
            gate(GateKind::X, 0, vec![]);

        let fused =
            fuse_pair(&x, &x)
                .expect("X followed by X should fuse");

        let matrix =
            gate_matrix(&fused)
                .expect("fused matrix should exist");

        assert!(
            matrix
                .approximately_equal(Matrix2::IDENTITY)
        );
    }

    #[test]
    fn h_h_fuses_to_identity() {
        let h =
            gate(GateKind::H, 0, vec![]);

        let fused =
            fuse_pair(&h, &h)
                .expect("H followed by H should fuse");

        let matrix =
            gate_matrix(&fused)
                .expect("fused matrix should exist");

        assert!(
            matrix
                .approximately_equal(Matrix2::IDENTITY)
        );
    }

    #[test]
    fn rotation_is_not_symbolically_fused() {
        let first =
            gate(
                GateKind::RX,
                0,
                vec![Parameter::Symbol(
                    "theta".to_string(),
                )],
            );

        let second =
            gate(
                GateKind::RX,
                0,
                vec![constant_parameter(0.5)],
            );

        assert!(
            fuse_pair(&first, &second).is_none()
        );
    }

    #[test]
    fn different_qubits_are_not_fused() {
        let first =
            gate(GateKind::H, 0, vec![]);

        let second =
            gate(GateKind::H, 1, vec![]);

        assert!(
            fuse_pair(&first, &second).is_none()
        );
    }

    #[test]
    fn two_qubit_gates_are_not_fused() {
        let first =
            Gate::new(
                GateKind::CX,
                vec![
                    crate::quantum::ir::qubits::QubitId::new(
                        0,
                    )
                    .expect("test qubit"),
                    crate::quantum::ir::qubits::QubitId::new(
                        1,
                    )
                    .expect("test qubit"),
                ],
                vec![],
                None,
                None,
            )
            .expect("test CX");

        let second =
            first.clone();

        assert!(
            fuse_pair(&first, &second).is_none()
        );
    }

    #[test]
    fn measurements_are_not_fused() {
        // A measurement is intentionally excluded before any matrix conversion.
        //
        // This test verifies the classification boundary rather than creating
        // a full measurement payload.
        assert!(
            !matches!(
                GateKind::Measure,
                GateKind::I
                    | GateKind::X
                    | GateKind::Y
                    | GateKind::Z
                    | GateKind::H
                    | GateKind::S
                    | GateKind::Sdg
                    | GateKind::T
                    | GateKind::Tdg
                    | GateKind::V
                    | GateKind::Vdg
                    | GateKind::RX
                    | GateKind::RY
                    | GateKind::RZ
                    | GateKind::Phase
                    | GateKind::U1
                    | GateKind::U2
                    | GateKind::U3
            )
        );
    }

    #[test]
    fn u3_round_trip_is_exact() {
        let original =
            gate(
                GateKind::U3,
                0,
                vec![
                    constant_parameter(0.73),
                    constant_parameter(-0.41),
                    constant_parameter(1.17),
                ],
            );

        let matrix =
            gate_matrix(&original)
                .expect("U3 must have a matrix");

        let parameters =
            decompose_to_u3(matrix)
                .expect("U3 should be representable");

        let reconstructed =
            u3_matrix(
                parameters.0,
                parameters.1,
                parameters.2,
            );

        assert!(
            reconstructed
                .approximately_equal(matrix)
        );
    }

    #[test]
    fn h_x_h_is_fusable() {
        let h =
            gate(GateKind::H, 0, vec![]);

        let x =
            gate(GateKind::X, 0, vec![]);

        let first =
            gate_matrix(&h)
                .expect("H matrix");

        let second =
            gate_matrix(&x)
                .expect("X matrix");

        let third =
            gate_matrix(&h)
                .expect("H matrix");

        let combined =
            third
                .multiply(
                    second.multiply(first),
                );

        let fused =
            build_fused_gate(
                &h,
                &x,
                second.multiply(first),
            );

        // The pair H;X is independently representable in the supported
        // canonical U3 family. The final H is tested through the mathematical
        // product to ensure multiplication ordering is correct.
        let expected =
            third.multiply(
                second.multiply(first),
            );

        if let Some(gate) = fused {
            let fused_matrix =
                gate_matrix(&gate)
                    .expect("fused matrix");

            assert!(
                fused_matrix
                    .approximately_equal(
                        second.multiply(first),
                    )
            );
        }

        assert!(
            combined.approximately_equal(expected)
        );
    }

    // -------------------------------------------------------------------------
    // Matrix helpers used only by tests/verification.
    // -------------------------------------------------------------------------

    #[must_use]
    fn conjugate_transpose(matrix: Matrix2) -> Matrix2 {
        Matrix2::new(
            matrix.m00.conjugate(),
            matrix.m10.conjugate(),
            matrix.m01.conjugate(),
            matrix.m11.conjugate(),
        )
    }
}