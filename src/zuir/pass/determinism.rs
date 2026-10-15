
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR pass determinism
//!
//! Path: `src/zuir/pass/determinism.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! PURPOSE
//!
//! Define reusable checks and reports for the determinism declarations of
//! compiler passes and ordered pass pipelines.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/contracts.rs`
//! - `src/zuir/pass/manager.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/extensibility.md`
//!
//! OWNS
//!
//! - Validation of pass determinism declarations against execution policy.
//! - Ordered assessment of a pipeline's declared determinism.
//! - Structured rejection of nondeterministic passes when required.
//!
//! DOES_NOT_OWN
//!
//! - Pass identities, descriptors, versions, or error definitions.
//! - Pass execution, scheduling, or parallel execution policy.
//! - Canonical IR serialization or stable IR fingerprints.
//! - Proof that a pass implementation is actually deterministic.
//! - Hardware discovery, target selection, resource allocation, or runtime
//!   execution.
//!
//! DETERMINISM CONTRACT
//!
//! A deterministic declaration is a contract made by a pass implementation,
//! not a proof. This module checks the declaration against policy; it cannot
//! establish that arbitrary pass code behaves deterministically.
//!
//! Pipeline order is semantically significant and is never sorted here.
//! Equivalent reproducibility claims additionally depend on relevant input
//! IR, pass versions, configuration, dialect definitions, external inputs,
//! and any other inputs that can affect pass behavior.
//!
//! SCALABILITY
//!
//! No fixed pass-count, hardware-count, or IR-size limit is introduced.
//! Pipeline assessment uses dynamically sized collections. Concrete execution
//! remains limited by available memory, time, and explicit operational budgets.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use super::contracts::{
    PassDescriptor, PassError, PassErrorKind, PassId,
};

/// Policy controlling whether a pass may declare nondeterministic behavior.
///
/// This policy does not change a pass's semantics. It controls whether a
/// pipeline is permitted to execute a pass whose descriptor does not promise
/// reproducible behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeterminismPolicy {
    /// Permit deterministic and nondeterministic passes.
    ///
    /// Reports still distinguish passes that declare nondeterminism.
    #[default]
    AllowNondeterministic,

    /// Require every pass to declare deterministic behavior.
    ///
    /// A pass that does not make this declaration is rejected before it runs.
    RequireDeclaredDeterminism,
}

impl DeterminismPolicy {
    /// Whether this policy requires every pass to declare deterministic
    /// behavior.
    #[must_use]
    pub const fn requires_determinism(self) -> bool {
        matches!(self, Self::RequireDeclaredDeterminism)
    }
}

/// Assessment of a pipeline's pass determinism declarations.
///
/// `nondeterministic_passes` preserves pipeline execution order. It is not
/// sorted by ID, because pipeline order may affect program meaning.
///
/// This assessment reports declarations only. An empty list does not prove
/// bit-for-bit reproducibility of the entire compilation or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminismAssessment {
    /// Number of pass descriptors assessed.
    pub pass_count: usize,

    /// IDs of passes that do not declare deterministic behavior, in pipeline
    /// order.
    pub nondeterministic_passes: Vec<PassId>,
}

impl DeterminismAssessment {
    /// Whether every assessed pass declares deterministic behavior.
    ///
    /// The result is `true` for an empty pipeline, because no pass violates
    /// the declaration requirement. This does not assert anything about
    /// external inputs, configuration, or downstream execution.
    #[must_use]
    pub fn all_passes_declare_determinism(&self) -> bool {
        self.nondeterministic_passes.is_empty()
    }

    /// Number of passes that do not declare deterministic behavior.
    #[must_use]
    pub fn nondeterministic_count(&self) -> usize {
        self.nondeterministic_passes.len()
    }
}

/// Validate one pass descriptor against the supplied determinism policy.
///
/// The descriptor's canonical validation is always performed first. This
/// function deliberately reuses `contracts.rs` rather than duplicating its
/// validation rules.
///
/// # Errors
///
/// Returns the canonical descriptor-validation error if the descriptor is
/// invalid. Returns `NondeterministicPassRejected` attributed to the pass if
/// the policy requires deterministic behavior and the pass does not declare
/// it.
///
/// # Guarantees
///
/// This function does not execute the pass, mutate IR, change configuration,
/// or modify caller-owned state.
pub fn validate_pass_determinism(
    descriptor: &PassDescriptor,
    policy: DeterminismPolicy,
) -> Result<(), PassError> {
    descriptor.validate()?;

    if policy.requires_determinism() && !descriptor.deterministic {
        return Err(PassError::for_pass(
            PassErrorKind::NondeterministicPassRejected,
            descriptor.id.clone(),
            "pass does not declare deterministic behavior and was rejected \
             by the active determinism policy",
        ));
    }

    Ok(())
}

/// Assess the determinism declarations of an ordered pipeline.
///
/// The iterator order is treated as execution order and preserved in the
/// assessment. This function never sorts or reorders descriptors.
///
/// Every descriptor is validated. Under `RequireDeclaredDeterminism`, the
/// first nondeterministic pass in pipeline order causes an error. Otherwise,
/// the returned assessment records all nondeterministic pass IDs in order.
///
/// # Errors
///
/// Returns a descriptor-validation error for invalid metadata, or
/// `NondeterministicPassRejected` when the policy requires determinism but a
/// pass does not declare it.
///
/// # Resource behavior
///
/// The result contains one ID for each nondeterministic pass. Memory use
/// therefore grows with the number of such passes; no arbitrary capacity
/// limit is imposed.
pub fn assess_pipeline_determinism<'a>(
    descriptors: impl IntoIterator<Item = &'a PassDescriptor>,
    policy: DeterminismPolicy,
) -> Result<DeterminismAssessment, PassError> {
    let mut pass_count = 0usize;
    let mut nondeterministic_passes = Vec::new();

    for descriptor in descriptors {
        validate_pass_determinism(descriptor, DeterminismPolicy::AllowNondeterministic)?;

        pass_count = pass_count.checked_add(1).ok_or_else(|| {
            PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                descriptor.id.clone(),
                "pipeline pass count overflowed the implementation's index type",
            )
        })?;

        if !descriptor.deterministic {
            if policy.requires_determinism() {
                return Err(PassError::for_pass(
                    PassErrorKind::NondeterministicPassRejected,
                    descriptor.id.clone(),
                    "pipeline requires deterministic behavior, but this pass \
                     does not declare it",
                ));
            }

            nondeterministic_passes.push(descriptor.id.clone());
        }
    }

    Ok(DeterminismAssessment {
        pass_count,
        nondeterministic_passes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::pass::contracts::{PassId, PassVersion};

    fn descriptor(name: &str, deterministic: bool) -> PassDescriptor {
        let mut descriptor = PassDescriptor::new(
            PassId::new(name).expect("test pass ID must be valid"),
            PassVersion::new(1, 0, 0),
            "Test pass",
        );

        descriptor.deterministic = deterministic;
        descriptor
    }

    #[test]
    fn default_policy_allows_nondeterministic_passes() {
        assert_eq!(
            DeterminismPolicy::default(),
            DeterminismPolicy::AllowNondeterministic
        );

        let pass = descriptor("core.example", false);

        assert!(
            validate_pass_determinism(&pass, DeterminismPolicy::default()).is_ok()
        );
    }

    #[test]
    fn required_determinism_accepts_a_declared_deterministic_pass() {
        let pass = descriptor("core.canonicalize", true);

        assert!(
            validate_pass_determinism(
                &pass,
                DeterminismPolicy::RequireDeclaredDeterminism
            )
            .is_ok()
        );
    }

    #[test]
    fn required_determinism_rejects_a_nondeterministic_pass() {
        let pass = descriptor("custom.parallel_transform", false);

        let error = validate_pass_determinism(
            &pass,
            DeterminismPolicy::RequireDeclaredDeterminism,
        )
        .expect_err("nondeterministic pass must be rejected");

        assert_eq!(
            error.kind,
            PassErrorKind::NondeterministicPassRejected
        );
        assert_eq!(error.pass.as_ref(), Some(&pass.id));
    }

    #[test]
    fn pipeline_assessment_counts_all_passes() {
        let first = descriptor("core.canonicalize", true);
        let second = descriptor("classical.simplify", true);
        let third = descriptor("quantum.optimize", true);

        let assessment = assess_pipeline_determinism(
            [&first, &second, &third],
            DeterminismPolicy::RequireDeclaredDeterminism,
        )
        .expect("all passes declare determinism");

        assert_eq!(assessment.pass_count, 3);
        assert_eq!(assessment.nondeterministic_count(), 0);
        assert!(assessment.all_passes_declare_determinism());
    }

    #[test]
    fn pipeline_assessment_preserves_execution_order() {
        let first = descriptor("quantum.transform", false);
        let second = descriptor("core.canonicalize", true);
        let third = descriptor("hdl.elaborate", false);

        let assessment = assess_pipeline_determinism(
            [&first, &second, &third],
            DeterminismPolicy::AllowNondeterministic,
        )
        .expect("nondeterministic passes are allowed by policy");

        assert_eq!(assessment.pass_count, 3);
        assert_eq!(
            assessment.nondeterministic_passes,
            vec![first.id.clone(), third.id.clone()]
        );
    }

    #[test]
    fn strict_pipeline_policy_rejects_first_nondeterministic_pass() {
        let first = descriptor("core.canonicalize", true);
        let second = descriptor("custom.parallel_transform", false);
        let third = descriptor("quantum.optimize", false);

        let error = assess_pipeline_determinism(
            [&first, &second, &third],
            DeterminismPolicy::RequireDeclaredDeterminism,
        )
        .expect_err("pipeline contains nondeterministic passes");

        assert_eq!(
            error.kind,
            PassErrorKind::NondeterministicPassRejected
        );
        assert_eq!(error.pass.as_ref(), Some(&second.id));
    }

    #[test]
    fn invalid_descriptors_are_rejected_even_when_nondeterminism_is_allowed() {
        let mut pass = descriptor("core.example", false);
        pass.description = "  ".to_owned();

        let error = validate_pass_determinism(
            &pass,
            DeterminismPolicy::AllowNondeterministic,
        )
        .expect_err("invalid descriptor must be rejected");

        assert_eq!(error.kind, PassErrorKind::InvalidConfiguration);
        assert_eq!(error.pass.as_ref(), Some(&pass.id));
    }

    #[test]
    fn empty_pipeline_is_vacuously_deterministic_by_declaration() {
        let assessment = assess_pipeline_determinism(
            std::iter::empty::<&PassDescriptor>(),
            DeterminismPolicy::RequireDeclaredDeterminism,
        )
        .expect("empty pipeline has no nondeterministic passes");

        assert_eq!(assessment.pass_count, 0);
        assert!(assessment.all_passes_declare_determinism());
        assert!(assessment.nondeterministic_passes.is_empty());
    }

    #[test]
    fn deterministic_policy_is_explicit() {
        assert!(!DeterminismPolicy::AllowNondeterministic.requires_determinism());
        assert!(
            DeterminismPolicy::RequireDeclaredDeterminism.requires_determinism()
        );
    }
}
