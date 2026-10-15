
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR pass analysis invalidation
//!
//! Path: `src/zuir/pass/invalidation.rs`
//!
//! STATUS
//!
//! Implementation target; freeze after repository integration and tests pass.
//!
//! PURPOSE
//!
//! Determine which currently available analysis results become invalid after
//! a pass executes. This module implements the invalidation policy shared by
//! the pass manager and future pipeline tooling.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/contracts.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//!
//! OWNS
//!
//! - The algorithm that derives invalidated analyses from a pass descriptor,
//!   a pass outcome, and the analyses currently known to be available.
//! - Deterministic invalidation results.
//! - Rejection of contradictory invalidation/preservation declarations.
//!
//! DOES_NOT_OWN
//!
//! - Pass IDs, descriptors, outcomes, and error types.
//! - Analysis computation, analysis result storage, or cache contents.
//! - Pass registration, scheduling, or execution.
//! - IR mutation, verification, or semantic-preservation proofs.
//! - Resource discovery, hardware allocation, or target selection.
//!
//! DEPENDENCIES
//!
//! Uses canonical contracts from `super::contracts` and standard-library
//! collections only. It does not depend on the manager or any IR module.
//!
//! INPUTS
//!
//! - The set of analysis identifiers currently marked available.
//! - The validated descriptor of the executed pass.
//! - The outcome reported by that pass.
//!
//! OUTPUTS
//!
//! A deterministic set of identifiers that the caller must mark unavailable.
//! The input set is never mutated by this function.
//!
//! SEMANTICS
//!
//! - A pass that reports no IR change does not invalidate analyses.
//! - A pass that declares preservation of all analyses invalidates none.
//! - Otherwise, a changed IR invalidates every available analysis not
//!   explicitly preserved by the outcome, as well as every analysis explicitly
//!   invalidated by the descriptor.
//! - Explicit invalidation and explicit preservation of the same analysis is
//!   a contract error, not a precedence rule.
//! - An invalidation declaration for an analysis that is not currently
//!   available has no effect on the returned set.
//!
//! SCALABILITY
//!
//! No fixed analysis-count, hardware-count, or IR-size limit is introduced.
//! Standard collections grow dynamically. Actual memory availability remains
//! finite, and allocation failure is not universally recoverable in Rust.
//!
//! DETERMINISM
//!
//! Results use `BTreeSet`, so iteration order is stable and independent of
//! hash randomization or hardware inventory.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use std::collections::BTreeSet;

use super::contracts::{PassDescriptor, PassError, PassErrorKind, PassOutcome};

/// Compute which currently available analyses become invalid after a pass.
///
/// This function is deliberately pure with respect to caller-owned analysis
/// state: it does not mutate `available`. The manager applies the returned
/// invalidations only after the pass has completed successfully and the
/// configured output-verification policy has succeeded.
///
/// # Errors
///
/// Returns a structured `PassError` if:
///
/// - The descriptor is invalid.
/// - The pass simultaneously declares an analysis preserved and invalidated.
///
/// An error leaves caller-owned analysis state unchanged.
///
/// # Example
///
/// ```ignore
/// let invalidated = compute_invalidated_analyses(
///     analyses.available(),
///     &descriptor,
///     &outcome,
/// )?;
///
/// for analysis in &invalidated {
///     analyses.mark_unavailable(analysis);
/// }
/// ```
pub fn compute_invalidated_analyses(
    available: &BTreeSet<String>,
    descriptor: &PassDescriptor,
    outcome: &PassOutcome,
) -> Result<BTreeSet<String>, PassError> {
    // Reuse the canonical descriptor validator. Do not maintain a second
    // implementation of pass metadata validation in this module.
    descriptor.validate()?;

    // A preservation declaration cannot override an explicit invalidation.
    // Reject the contradiction before calculating or applying any changes.
    if let Some(analysis) = descriptor
        .invalidated_analyses
        .intersection(&outcome.preserved_analyses)
        .next()
    {
        return Err(PassError::for_pass(
            PassErrorKind::InvalidConfiguration,
            descriptor.id.clone(),
            format!(
                "analysis {analysis:?} is both explicitly preserved by the \
                 pass outcome and explicitly invalidated by its descriptor"
            ),
        ));
    }

    // A pass that made no change must not invalidate analyses merely because
    // its descriptor lists analyses that could be invalidated by a mutation.
    if !outcome.changed {
        return Ok(BTreeSet::new());
    }

    // This descriptor declaration is a pass contract. Its correctness cannot
    // be proven by this algorithm; the pass author remains responsible for it.
    if descriptor.preserves_all_analyses {
        return Ok(BTreeSet::new());
    }

    // Build the result before the manager changes its analysis state. An
    // analysis not currently available cannot become invalid in this step.
    //
    // The default policy is conservative: after a change, every currently
    // available analysis is invalid unless explicitly preserved, except that
    // explicit invalidation always remains mandatory.
    let invalidated = available
        .iter()
        .filter(|analysis| {
            descriptor.invalidated_analyses.contains(*analysis)
                || !outcome.preserved_analyses.contains(*analysis)
        })
        .cloned()
        .collect();

    Ok(invalidated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::pass::contracts::{PassId, PassVersion};

    fn descriptor() -> PassDescriptor {
        PassDescriptor::new(
            PassId::new("core.example").expect("valid test pass ID"),
            PassVersion::new(1, 0, 0),
            "Test pass",
        )
    }

    fn available(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn unchanged_pass_does_not_invalidate_analyses() {
        let mut pass = descriptor();
        pass.invalidated_analyses
            .insert("core.cfg".to_owned());

        let state = available(&["core.cfg", "core.dominance"]);
        let outcome = PassOutcome::unchanged();

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert!(result.is_empty());
        assert_eq!(state, available(&["core.cfg", "core.dominance"]));
    }

    #[test]
    fn changed_pass_invalidates_unpreserved_available_analyses() {
        let pass = descriptor();
        let state = available(&[
            "core.cfg",
            "core.dominance",
            "core.liveness",
        ]);
        let outcome = PassOutcome::preserving(
            true,
            ["core.cfg".to_owned()],
        );

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert_eq!(
            result,
            available(&["core.dominance", "core.liveness"])
        );
    }

    #[test]
    fn explicit_invalidation_applies_even_if_other_analyses_are_preserved() {
        let mut pass = descriptor();
        pass.invalidated_analyses
            .insert("core.cfg".to_owned());

        let state = available(&["core.cfg", "core.dominance"]);
        let outcome = PassOutcome::preserving(
            true,
            ["core.dominance".to_owned()],
        );

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert_eq!(result, available(&["core.cfg"]));
    }

    #[test]
    fn preserving_all_analyses_keeps_all_available_analyses() {
        let mut pass = descriptor();
        pass.preserves_all_analyses = true;

        let state = available(&["core.cfg", "core.dominance"]);
        let outcome = PassOutcome::changed();

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn contradictory_descriptor_and_outcome_are_rejected() {
        let mut pass = descriptor();
        pass.invalidated_analyses
            .insert("core.cfg".to_owned());

        let state = available(&["core.cfg", "core.dominance"]);
        let outcome = PassOutcome::preserving(
            true,
            ["core.cfg".to_owned()],
        );

        let error =
            compute_invalidated_analyses(&state, &pass, &outcome)
                .expect_err("contradictory declarations must fail");

        assert_eq!(error.kind, PassErrorKind::InvalidConfiguration);
        assert_eq!(error.pass.as_ref(), Some(&pass.id));

        // Calculation failure must not mutate the caller's state.
        assert_eq!(state, available(&["core.cfg", "core.dominance"]));
    }

    #[test]
    fn invalid_descriptor_is_rejected() {
        let mut pass = descriptor();
        pass.preserves_all_analyses = true;
        pass.invalidated_analyses
            .insert("core.cfg".to_owned());

        let error = compute_invalidated_analyses(
            &available(&["core.cfg"]),
            &pass,
            &PassOutcome::changed(),
        )
        .expect_err("invalid descriptor must fail");

        assert_eq!(error.kind, PassErrorKind::InvalidConfiguration);
    }

    #[test]
    fn unavailable_analyses_are_not_reported_as_invalidated() {
        let mut pass = descriptor();
        pass.invalidated_analyses
            .insert("core.not_computed".to_owned());

        let state = available(&["core.cfg"]);
        let outcome = PassOutcome::preserving(
            true,
            ["core.cfg".to_owned()],
        );

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn empty_analysis_state_is_supported() {
        let pass = descriptor();
        let state = BTreeSet::new();
        let outcome = PassOutcome::changed();

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn invalidation_order_is_deterministic() {
        let pass = descriptor();
        let state = available(&[
            "quantum.liveness",
            "core.cfg",
            "hdl.dependencies",
            "classical.aliases",
        ]);
        let outcome = PassOutcome::changed();

        let result =
            compute_invalidated_analyses(&state, &pass, &outcome).unwrap();

        let ordered: Vec<&str> =
            result.iter().map(String::as_str).collect();

        assert_eq!(
            ordered,
            vec![
                "classical.aliases",
                "core.cfg",
                "hdl.dependencies",
                "quantum.liveness",
            ]
        );
    }
}
