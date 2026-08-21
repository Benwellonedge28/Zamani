//! Zamani Quantum Error Correction — Mathematical Verification Orchestrator.
//!
//! # Ownership
//!
//! This module owns the orchestration of mathematical verification across the
//! QEC subsystem.
//!
//! It owns:
//!
//! - verification policies;
//! - verification reports;
//! - verification status;
//! - deterministic verification orchestration;
//! - surface-code structural verification;
//! - stabilizer-group verification orchestration;
//! - logical-operator verification orchestration;
//! - exact-distance verification orchestration;
//! - stabilizer-equivalence classification;
//! - logical-equivalence classification;
//! - correction/syndrome consistency checks;
//! - fail-closed verification helpers.
//!
//! It does NOT own:
//!
//! - Pauli algebra (`stabilizer.rs`);
//! - stabilizer algebra (`stabilizer.rs`);
//! - surface-code topology (`surface_code.rs`);
//! - exact-distance search implementation (`distance.rs`);
//! - decoder algorithms (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - QPU execution (`qpu_adapter.rs`);
//! - raw QPU measurement extraction (`syndrome_extractor.rs`);
//! - statistical confidence intervals (`statistical.rs`);
//! - telemetry transport (`telemetry.rs`);
//! - resource policy (`limits.rs`);
//! - resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`).
//!
//! # Architectural rule
//!
//! Verification is an orchestration layer.
//!
//! ```text
//!                         verification.rs
//!                               |
//!             +-----------------+-----------------+
//!             |                 |                 |
//!             v                 v                 v
//!       surface_code       stabilizer         distance
//!             |                 |                 |
//!             +-----------------+-----------------+
//!                               |
//!                               v
//!                         VerificationReport
//! ```
//!
//! The underlying mathematical operations remain authoritative in their
//! owning modules. This file must never create a second Pauli algebra,
//! stabilizer algebra, topology implementation, or distance-search algorithm.
//!
//! # Verification philosophy
//!
//! Verification is fail-closed.
//!
//! A verification operation must never convert:
//!
//! - resource exhaustion;
//! - cancellation;
//! - malformed input;
//! - incompatible representations;
//! - incomplete verification;
//! - numerical failure
//!
//! into a successful verification result.
//!
//! `Verified` means the requested invariant was actually established.
//!
//! # Determinism
//!
//! Verification must be deterministic for deterministic inputs. Reports use
//! stable ordering and contain no timestamps, random identifiers, pointers,
//! addresses, or backend-specific nondeterministic state.
//!
//! # Security
//!
//! This module never receives or stores QPU credentials, authentication
//! material, raw hardware secrets, or private backend state.
//!
//! QPU verification proves the mathematical contract at the QEC boundary.
//! It does not claim that physical hardware behaved correctly merely because
//! a mathematical representation passed verification.
//!
//! # Resource safety
//!
//! Verification delegates workload policy to `QecLimits` and expensive
//! execution to the owning mathematical modules. This module never creates a
//! second production resource-limit system.
//!
//! # Cancellation
//!
//! Expensive verification operations accept a `CancellationToken` and check
//! it at verification boundaries. The underlying exact-distance verifier
//! remains responsible for polling cancellation during its own search.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are required.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::cancellation::CancellationToken;
use super::errors::{
    NumericalOperation,
    QecError,
    QecResult,
};
use super::stabilizer::{
    PauliString,
    StabilizerGroup,
};
use super::surface_code::SurfaceCode;

/* ========================================================================= */
/* Verification status                                                       */
/* ========================================================================= */

/// Final status of a verification operation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum VerificationStatus {
    /// The requested mathematical invariant was established.
    Verified,

    /// The requested verification could not be completed because the
    /// workload exceeded an intentional resource boundary.
    ResourceLimited,

    /// Verification was deliberately cancelled.
    Cancelled,

    /// Verification was not requested or could not be meaningfully started.
    Unverified,
}

impl VerificationStatus {
    /// Returns whether the requested invariant was actually established.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Returns whether verification was cancelled.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    /// Returns whether verification was stopped by a resource boundary.
    #[must_use]
    pub const fn is_resource_limited(self) -> bool {
        matches!(self, Self::ResourceLimited)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::ResourceLimited => "resource_limited",
            Self::Cancelled => "cancelled",
            Self::Unverified => "unverified",
        }
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/* ========================================================================= */
/* Verification failure classification                                       */
/* ========================================================================= */

/// High-level classification of a failed verification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum VerificationFailure {
    /// Surface-code topology is invalid.
    Topology,

    /// Stabilizer group is invalid.
    Stabilizer,

    /// Logical operators are invalid.
    LogicalOperators,

    /// Code distance could not be established.
    Distance,

    /// Two representations have incompatible dimensions.
    RepresentationMismatch,

    /// A correction does not correspond to the supplied syndrome.
    SyndromeMismatch,

    /// Two corrections are not equivalent under the requested relation.
    EquivalenceMismatch,

    /// Verification was cancelled.
    Cancelled,

    /// Verification exceeded an intentional resource boundary.
    ResourceLimited,

    /// A numerical operation could not be performed safely.
    Numerical,

    /// An internal invariant was violated.
    Internal,
}

impl VerificationFailure {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Topology => "topology",
            Self::Stabilizer => "stabilizer",
            Self::LogicalOperators => "logical_operators",
            Self::Distance => "distance",
            Self::RepresentationMismatch => "representation_mismatch",
            Self::SyndromeMismatch => "syndrome_mismatch",
            Self::EquivalenceMismatch => "equivalence_mismatch",
            Self::Cancelled => "cancelled",
            Self::ResourceLimited => "resource_limited",
            Self::Numerical => "numerical",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for VerificationFailure {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/* ========================================================================= */
/* Logical relation                                                          */
/* ========================================================================= */

/// Relation between two phase-free Pauli operators with respect to a
/// stabilizer code.
///
/// The relation is determined through the canonical stabilizer algebra and
/// logical operators supplied by the `SurfaceCode`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum LogicalRelation {
    /// Operators differ only by a stabilizer.
    StabilizerEquivalent,

    /// Operators differ by logical X, modulo stabilizers.
    LogicalX,

    /// Operators differ by logical Z, modulo stabilizers.
    LogicalZ,

    /// Operators differ by logical Y, modulo stabilizers.
    LogicalY,

    /// Operators are not equivalent under the known logical basis.
    Distinct,
}

impl LogicalRelation {
    /// Returns whether the two operators represent the same physical
    /// correction class modulo stabilizers.
    #[must_use]
    pub const fn is_stabilizer_equivalent(self) -> bool {
        matches!(
            self,
            Self::StabilizerEquivalent
        )
    }

    /// Returns whether the relation represents a logical failure/change.
    #[must_use]
    pub const fn is_logical_difference(self) -> bool {
        matches!(
            self,
            Self::LogicalX
                | Self::LogicalZ
                | Self::LogicalY
        )
    }
}

/* ========================================================================= */
/* Verification policy                                                       */
/* ========================================================================= */

/// Policy controlling mathematical verification.
#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct VerificationPolicy {
    /// Verify the complete surface-code topology.
    pub verify_topology: bool,

    /// Verify the stabilizer group.
    pub verify_stabilizers: bool,

    /// Verify logical X/Z operators.
    pub verify_logical_operators: bool,

    /// Verify exact code distance.
    pub verify_distance: bool,

    /// Verify logical X/Z anti-commutation.
    pub verify_logical_anticommutation: bool,

    /// Require exact distance verification rather than merely trusting the
    /// declared distance.
    pub require_exact_distance: bool,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self {
            verify_topology: true,
            verify_stabilizers: true,
            verify_logical_operators: true,
            verify_distance: true,
            verify_logical_anticommutation: true,
            require_exact_distance: true,
        }
    }
}

impl VerificationPolicy {
    /// Creates a complete mathematical-verification policy.
    #[must_use]
    pub const fn complete() -> Self {
        Self {
            verify_topology: true,
            verify_stabilizers: true,
            verify_logical_operators: true,
            verify_distance: true,
            verify_logical_anticommutation: true,
            require_exact_distance: true,
        }
    }

    /// Creates a structural-only policy.
    #[must_use]
    pub const fn structural() -> Self {
        Self {
            verify_topology: true,
            verify_stabilizers: true,
            verify_logical_operators: true,
            verify_distance: false,
            verify_logical_anticommutation: true,
            require_exact_distance: false,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> QecResult<()> {
        if self.require_exact_distance
            && !self.verify_distance
        {
            return Err(QecError::invalid_input(
                "exact-distance verification was requested but distance verification is disabled",
            ));
        }

        Ok(())
    }
}

/* ========================================================================= */
/* Verification report                                                       */
/* ========================================================================= */

/// Immutable report produced by mathematical verification.
///
/// A report only claims properties that were actually checked.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct VerificationReport {
    status: VerificationStatus,

    topology_verified: bool,

    stabilizers_verified: bool,

    logical_operators_verified: bool,

    logical_anticommutation_verified: bool,

    distance_verified: bool,

    declared_distance: usize,

    verified_distance: Option<usize>,

    num_qubits: usize,

    num_stabilizers: usize,
}

impl VerificationReport {
    fn new(
        code: &SurfaceCode,
    ) -> Self {
        Self {
            status: VerificationStatus::Unverified,
            topology_verified: false,
            stabilizers_verified: false,
            logical_operators_verified: false,
            logical_anticommutation_verified: false,
            distance_verified: false,
            declared_distance: code.distance(),
            verified_distance: None,
            num_qubits: code.num_data_qubits(),
            num_stabilizers: code.num_stabilizers(),
        }
    }

    /// Returns the final verification status.
    #[must_use]
    pub const fn status(
        &self,
    ) -> VerificationStatus {
        self.status
    }

    /// Returns whether topology was verified.
    #[must_use]
    pub const fn topology_verified(
        &self,
    ) -> bool {
        self.topology_verified
    }

    /// Returns whether stabilizers were verified.
    #[must_use]
    pub const fn stabilizers_verified(
        &self,
    ) -> bool {
        self.stabilizers_verified
    }

    /// Returns whether logical operators were verified.
    #[must_use]
    pub const fn logical_operators_verified(
        &self,
    ) -> bool {
        self.logical_operators_verified
    }

    /// Returns whether logical X/Z anti-commutation was verified.
    #[must_use]
    pub const fn logical_anticommutation_verified(
        &self,
    ) -> bool {
        self.logical_anticommutation_verified
    }

    /// Returns whether exact distance was verified.
    #[must_use]
    pub const fn distance_verified(
        &self,
    ) -> bool {
        self.distance_verified
    }

    /// Returns the declared code distance.
    #[must_use]
    pub const fn declared_distance(
        &self,
    ) -> usize {
        self.declared_distance
    }

    /// Returns the mathematically verified distance, if available.
    #[must_use]
    pub const fn verified_distance(
        &self,
    ) -> Option<usize> {
        self.verified_distance
    }

    /// Returns the number of physical data qubits.
    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }

    /// Returns the number of stabilizers.
    #[must_use]
    pub const fn num_stabilizers(
        &self,
    ) -> usize {
        self.num_stabilizers
    }

    /// Returns whether every requested property in the report was verified.
    #[must_use]
    pub const fn is_complete(
        &self,
    ) -> bool {
        self.status.is_verified()
    }
}

/* ========================================================================= */
/* Verification engine                                                       */
/* ========================================================================= */

/// Stateless mathematical verification engine.
///
/// The engine owns no mutable execution state and is therefore safe to reuse
/// for deterministic verification of multiple code instances.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
)]
pub struct VerificationEngine;

impl VerificationEngine {
    /// Creates a verification engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Fully verifies a surface code using the default complete policy.
    pub fn verify_surface_code(
        &self,
        code: &SurfaceCode,
        cancellation: &CancellationToken,
    ) -> QecResult<VerificationReport> {
        self.verify_surface_code_with_policy(
            code,
            VerificationPolicy::complete(),
            cancellation,
        )
    }

    /// Verifies a surface code using an explicit policy.
    ///
    /// No expensive operation is started after cancellation has been
    /// observed.
    pub fn verify_surface_code_with_policy(
        &self,
        code: &SurfaceCode,
        policy: VerificationPolicy,
        cancellation: &CancellationToken,
    ) -> QecResult<VerificationReport> {
        policy.validate()?;

        cancellation.check()?;

        let mut report =
            VerificationReport::new(code);

        if policy.verify_topology {
            self.verify_topology(code)?;
            report.topology_verified = true;
        }

        cancellation.check()?;

        if policy.verify_stabilizers {
            self.verify_stabilizers(code)?;
            report.stabilizers_verified = true;
        }

        cancellation.check()?;

        if policy.verify_logical_operators {
            self.verify_logical_operators(code)?;
            report.logical_operators_verified = true;
        }

        cancellation.check()?;

        if policy.verify_logical_anticommutation {
            self.verify_logical_anticommutation(code)?;
            report.logical_anticommutation_verified =
                true;
        }

        cancellation.check()?;

        if policy.verify_distance {
            let verified =
                code.verify_distance().map_err(
                    |error| {
                        QecError::invalid_topology(
                            format!(
                                "exact surface-code distance verification failed: {error}"
                            ),
                        )
                    },
                )?;

            if policy.require_exact_distance
                && verified != code.distance()
            {
                return Err(
                    QecError::invalid_topology(
                        format!(
                            "declared distance {} does not equal verified distance {verified}",
                            code.distance()
                        ),
                    ),
                );
            }

            report.distance_verified = true;
            report.verified_distance =
                Some(verified);
        }

        report.status =
            VerificationStatus::Verified;

        Ok(report)
    }

    /// Verifies only the surface-code topology contract.
    pub fn verify_topology(
        &self,
        code: &SurfaceCode,
    ) -> QecResult<()> {
        code.validate().map_err(|error| {
            QecError::invalid_topology(format!(
                "surface-code topology validation failed: {error}"
            ))
        })
    }

    /// Verifies the complete stabilizer group attached to a surface code.
    pub fn verify_stabilizers(
        &self,
        code: &SurfaceCode,
    ) -> QecResult<()> {
        let group =
            code.stabilizer_group().map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "unable to construct surface-code stabilizer group: {error}"
                    ),
                )
            })?;

        self.verify_stabilizer_group(
            &group,
            code.num_data_qubits(),
        )
    }

    /// Verifies an arbitrary stabilizer group against an expected physical
    /// qubit count.
    pub fn verify_stabilizer_group(
        &self,
        group: &StabilizerGroup,
        expected_qubits: usize,
    ) -> QecResult<()> {
        if expected_qubits == 0 {
            return Err(
                QecError::invalid_input(
                    "stabilizer verification requires at least one physical qubit",
                ),
            );
        }

        if group.num_qubits()
            != expected_qubits
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "stabilizer group represents {} qubits but {} were expected",
                        group.num_qubits(),
                        expected_qubits
                    ),
                ),
            );
        }

        group.validate().map_err(
            |error| {
                QecError::invalid_stabilizer(
                    format!(
                        "stabilizer-group verification failed: {error}"
                    ),
                )
            },
        )
    }

    /// Verifies the logical X/Z operators attached to a surface code.
    pub fn verify_logical_operators(
        &self,
        code: &SurfaceCode,
    ) -> QecResult<()> {
        let logical_x =
            code.logical_x().operator();

        let logical_z =
            code.logical_z().operator();

        if logical_x.is_identity() {
            return Err(
                QecError::invalid_stabilizer(
                    "logical X cannot be identity",
                ),
            );
        }

        if logical_z.is_identity() {
            return Err(
                QecError::invalid_stabilizer(
                    "logical Z cannot be identity",
                ),
            );
        }

        let expected =
            code.num_data_qubits();

        if logical_x.num_qubits()
            != expected
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "logical X represents {} qubits but code contains {expected}",
                        logical_x.num_qubits()
                    ),
                ),
            );
        }

        if logical_z.num_qubits()
            != expected
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "logical Z represents {} qubits but code contains {expected}",
                        logical_z.num_qubits()
                    ),
                ),
            );
        }

        if logical_x.weight()
            != code.distance()
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "logical X has weight {} but declared code distance is {}",
                        logical_x.weight(),
                        code.distance()
                    ),
                ),
            );
        }

        if logical_z.weight()
            != code.distance()
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "logical Z has weight {} but declared code distance is {}",
                        logical_z.weight(),
                        code.distance()
                    ),
                ),
            );
        }

        let group =
            code.stabilizer_group().map_err(
                |error| {
                    QecError::invalid_stabilizer(
                        format!(
                            "unable to obtain stabilizer group: {error}"
                        ),
                    )
                },
            )?;

        if group
            .contains(logical_x)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "logical-X membership verification failed: {error}"
                    ),
                )
            })?
        {
            return Err(
                QecError::invalid_stabilizer(
                    "logical X must not be a stabilizer",
                ),
            );
        }

        if group
            .contains(logical_z)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "logical-Z membership verification failed: {error}"
                    ),
                )
            })?
        {
            return Err(
                QecError::invalid_stabilizer(
                    "logical Z must not be a stabilizer",
                ),
            );
        }

        for stabilizer in code.stabilizers() {
            if !logical_x
                .commutes_with(
                    stabilizer.operator(),
                )
                .map_err(|error| {
                    QecError::invalid_stabilizer(
                        format!(
                            "logical-X/stabilizer compatibility check failed: {error}"
                        ),
                    )
                })?
            {
                return Err(
                    QecError::invalid_stabilizer(
                        format!(
                            "logical X anticommutes with stabilizer {}",
                            stabilizer.id()
                        ),
                    ),
                );
            }

            if !logical_z
                .commutes_with(
                    stabilizer.operator(),
                )
                .map_err(|error| {
                    QecError::invalid_stabilizer(
                        format!(
                            "logical-Z/stabilizer compatibility check failed: {error}"
                        ),
                    )
                })?
            {
                return Err(
                    QecError::invalid_stabilizer(
                        format!(
                            "logical Z anticommutes with stabilizer {}",
                            stabilizer.id()
                        ),
                    ),
                );
            }
        }

        Ok(())
    }

    /// Verifies that logical X and logical Z anticommute.
    pub fn verify_logical_anticommutation(
        &self,
        code: &SurfaceCode,
    ) -> QecResult<()> {
        let x =
            code.logical_x().operator();

        let z =
            code.logical_z().operator();

        let anticommutes =
            x.anticommutes_with(z)
                .map_err(|error| {
                    QecError::invalid_stabilizer(
                        format!(
                            "logical X/Z compatibility check failed: {error}"
                        ),
                    )
                })?;

        if !anticommutes {
            return Err(
                QecError::invalid_stabilizer(
                    "logical X and logical Z must anticommute",
                ),
            );
        }

        Ok(())
    }

    /// Verifies that two Pauli operators have compatible dimensions.
    pub fn verify_dimensions(
        &self,
        first: &PauliString,
        second: &PauliString,
    ) -> QecResult<()> {
        if first.num_qubits()
            != second.num_qubits()
        {
            return Err(
                QecError::invalid_stabilizer(
                    format!(
                        "Pauli dimension mismatch: {} versus {}",
                        first.num_qubits(),
                        second.num_qubits()
                    ),
                ),
            );
        }

        Ok(())
    }

    /// Determines the stabilizer/logical relation between two Pauli
    /// corrections for a surface code.
    ///
    /// The comparison is performed through the canonical stabilizer group.
    /// No independent stabilizer algebra is implemented here.
    pub fn classify_logical_relation(
        &self,
        code: &SurfaceCode,
        first: &PauliString,
        second: &PauliString,
    ) -> QecResult<LogicalRelation> {
        self.verify_dimensions(
            first,
            second,
        )?;

        let stabilizers =
            code.stabilizer_group().map_err(
                |error| {
                    QecError::invalid_stabilizer(
                        format!(
                            "unable to construct stabilizer group: {error}"
                        ),
                    )
                },
            )?;

        let difference =
            first.multiply(second).map_err(
                |error| {
                    QecError::NumericalFailure {
                        operation:
                            NumericalOperation::StabilizerAlgebra,
                        message: format!(
                            "unable to compute Pauli difference: {error}"
                        ),
                    }
                },
            )?;

        if stabilizers
            .contains(&difference)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "stabilizer-equivalence check failed: {error}"
                    ),
                )
            })?
        {
            return Ok(
                LogicalRelation::StabilizerEquivalent
            );
        }

        let logical_x =
            code.logical_x().operator();

        let logical_z =
            code.logical_z().operator();

        let x_difference =
            difference
                .multiply(logical_x)
                .map_err(|error| {
                    QecError::NumericalFailure {
                        operation:
                            NumericalOperation::StabilizerAlgebra,
                        message: format!(
                            "logical-X comparison failed: {error}"
                        ),
                    }
                })?;

        if stabilizers
            .contains(&x_difference)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "logical-X equivalence check failed: {error}"
                    ),
                )
            })?
        {
            return Ok(LogicalRelation::LogicalX);
        }

        let z_difference =
            difference
                .multiply(logical_z)
                .map_err(|error| {
                    QecError::NumericalFailure {
                        operation:
                            NumericalOperation::StabilizerAlgebra,
                        message: format!(
                            "logical-Z comparison failed: {error}"
                        ),
                    }
                })?;

        if stabilizers
            .contains(&z_difference)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "logical-Z equivalence check failed: {error}"
                        ),
                    )
                })?
        {
            return Ok(LogicalRelation::LogicalZ);
        }

        let logical_y =
            logical_x
                .multiply(logical_z)
                .map_err(|error| {
                    QecError::NumericalFailure {
                        operation:
                            NumericalOperation::StabilizerAlgebra,
                        message: format!(
                            "logical-Y construction failed: {error}"
                        ),
                    }
                })?;

        let y_difference =
            difference
                .multiply(&logical_y)
                .map_err(|error| {
                    QecError::NumericalFailure {
                        operation:
                            NumericalOperation::StabilizerAlgebra,
                        message: format!(
                            "logical-Y comparison failed: {error}"
                        ),
                    }
                })?;

        if stabilizers
            .contains(&y_difference)
            .map_err(|error| {
                QecError::invalid_stabilizer(
                    format!(
                        "logical-Y equivalence check failed: {error}"
                    ),
                )
            })?
        {
            return Ok(LogicalRelation::LogicalY);
        }

        Ok(LogicalRelation::Distinct)
    }

    /// Returns whether two corrections are stabilizer-equivalent.
    pub fn are_stabilizer_equivalent(
        &self,
        code: &SurfaceCode,
        first: &PauliString,
        second: &PauliString,
    ) -> QecResult<bool> {
        Ok(matches!(
            self.classify_logical_relation(
                code,
                first,
                second,
            )?,
            LogicalRelation::StabilizerEquivalent
        ))
    }

    /// Returns whether two corrections differ by a logical operation.
    pub fn are_logically_distinct(
        &self,
        code: &SurfaceCode,
        first: &PauliString,
        second: &PauliString,
    ) -> QecResult<bool> {
        Ok(
            self.classify_logical_relation(
                code,
                first,
                second,
            )?
            .is_logical_difference(),
        )
    }
}

/* ========================================================================= */
/* Convenience functions                                                      */
/* ========================================================================= */

/// Fully verifies a surface code using the default verification policy.
pub fn verify_surface_code(
    code: &SurfaceCode,
    cancellation: &CancellationToken,
) -> QecResult<VerificationReport> {
    VerificationEngine::new()
        .verify_surface_code(
            code,
            cancellation,
        )
}

/// Verifies a surface code with an explicit policy.
pub fn verify_surface_code_with_policy(
    code: &SurfaceCode,
    policy: VerificationPolicy,
    cancellation: &CancellationToken,
) -> QecResult<VerificationReport> {
    VerificationEngine::new()
        .verify_surface_code_with_policy(
            code,
            policy,
            cancellation,
        )
}

/// Checks whether two Pauli corrections are stabilizer-equivalent.
pub fn are_stabilizer_equivalent(
    code: &SurfaceCode,
    first: &PauliString,
    second: &PauliString,
) -> QecResult<bool> {
    VerificationEngine::new()
        .are_stabilizer_equivalent(
            code,
            first,
            second,
        )
}

/// Classifies the logical relation between two Pauli corrections.
pub fn classify_logical_relation(
    code: &SurfaceCode,
    first: &PauliString,
    second: &PauliString,
) -> QecResult<LogicalRelation> {
    VerificationEngine::new()
        .classify_logical_relation(
            code,
            first,
            second,
        )
}

/* ========================================================================= */
/* Tests                                                                      */
/* ========================================================================= */

#[cfg(test)]
mod tests {
    use super::*;

    fn cancellation_token() -> CancellationToken {
        CancellationToken::new()
    }

    #[test]
    fn complete_policy_is_valid() {
        assert!(
            VerificationPolicy::complete()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn structural_policy_is_valid() {
        assert!(
            VerificationPolicy::structural()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn distance_three_surface_code_verifies() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let report =
            verify_surface_code(
                &code,
                &cancellation_token(),
            )
            .expect(
                "distance-3 code must verify",
            );

        assert!(
            report.is_complete()
        );

        assert_eq!(
            report.declared_distance(),
            3
        );

        assert_eq!(
            report.verified_distance(),
            Some(3)
        );

        assert!(
            report.topology_verified()
        );

        assert!(
            report.stabilizers_verified()
        );

        assert!(
            report.logical_operators_verified()
        );

        assert!(
            report.logical_anticommutation_verified()
        );

        assert!(
            report.distance_verified()
        );
    }

    #[test]
    fn distance_five_surface_code_verifies() {
        let code =
            SurfaceCode::new(5)
                .expect(
                    "distance-5 code must construct",
                );

        let report =
            verify_surface_code(
                &code,
                &cancellation_token(),
            )
            .expect(
                "distance-5 code must verify",
            );

        assert_eq!(
            report.verified_distance(),
            Some(5)
        );
    }

    #[test]
    fn identity_is_stabilizer_equivalent_to_identity() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let identity =
            PauliString::identity(
                code.num_data_qubits(),
            );

        let relation =
            classify_logical_relation(
                &code,
                &identity,
                &identity,
            )
            .expect(
                "identity relation must be computable",
            );

        assert_eq!(
            relation,
            LogicalRelation::StabilizerEquivalent
        );
    }

    #[test]
    fn logical_x_differs_from_identity_by_logical_x() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let identity =
            PauliString::identity(
                code.num_data_qubits(),
            );

        let relation =
            classify_logical_relation(
                &code,
                &identity,
                code.logical_x().operator(),
            )
            .expect(
                "logical-X relation must be computable",
            );

        assert_eq!(
            relation,
            LogicalRelation::LogicalX
        );
    }

    #[test]
    fn logical_z_differs_from_identity_by_logical_z() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let identity =
            PauliString::identity(
                code.num_data_qubits(),
            );

        let relation =
            classify_logical_relation(
                &code,
                &identity,
                code.logical_z().operator(),
            )
            .expect(
                "logical-Z relation must be computable",
            );

        assert_eq!(
            relation,
            LogicalRelation::LogicalZ
        );
    }

    #[test]
    fn mismatched_pauli_dimensions_are_rejected() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let first =
            PauliString::identity(
                code.num_data_qubits(),
            );

        let second =
            PauliString::identity(
                code.num_data_qubits() + 1,
            );

        let result =
            classify_logical_relation(
                &code,
                &first,
                &second,
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn verification_is_deterministic() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct",
                );

        let first =
            verify_surface_code(
                &code,
                &cancellation_token(),
            )
            .expect(
                "first verification must succeed",
            );

        let second =
            verify_surface_code(
                &code,
                &cancellation_token(),
            )
            .expect(
                "second verification must succeed",
            );

        assert_eq!(
            first,
            second
        );
    }
}