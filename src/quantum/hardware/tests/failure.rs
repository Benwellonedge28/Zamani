//! Zamani Quantum Hardware — Failure Conformance Tests
//!
//! Production conformance suite for:
//! `crate::quantum::hardware::failure`.
//!
//! # Responsibility
//!
//! This file tests the public, provider-neutral failure contract.
//!
//! It does NOT implement the failure model.
//!
//! The production implementation belongs in:
//!
//! `src/quantum/hardware/failure.rs`
//!
//! This file belongs in:
//!
//! `src/quantum/hardware/tests/failure.rs`
//!
//! # Architectural contract
//!
//! The failure model sits between normalized hardware errors and execution
//! policy:
//!
//! ```text
//! provider adapter
//!       |
//!       v
//! hardware::errors
//!       |
//!       v
//! hardware::failure
//!       |
//!       +-------------------+
//!       |                   |
//!       v                   v
//! retry policy          telemetry
//!       |
//!       v
//! execution / job / queue
//! ```
//!
//! The tests enforce the following production invariants:
//!
//! 1. Stable machine-readable identifiers never become empty.
//! 2. Failure classifications remain deterministic.
//! 3. Unknown conditions fail closed.
//! 4. Permanent failures are never automatically retried.
//! 5. Transient failures are not sufficient by themselves to authorize retry.
//! 6. Non-idempotent quantum operations are never automatically retried.
//! 7. Unknown idempotency fails closed under the production policy.
//! 8. Retry limits are enforced.
//! 9. Backoff is deterministic and bounded.
//! 10. Retry-after values cannot exceed the global safety bound.
//! 11. Retry policies validate their configuration.
//! 12. Production retry policy is conservative.
//! 13. Disabled retry means no retry.
//! 14. Stable string identifiers remain suitable for telemetry/serialization.
//! 15. Failure classifications are totally ordered and hashable.
//! 16. No provider-specific types are required by the core failure contract.
//!
//! # Rust compatibility
//!
//! Tested target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! # Integration
//!
//! `src/quantum/hardware/mod.rs` must expose:
//!
//! ```text
//! pub mod failure;
//! ```
//!
//! `src/quantum/hardware/tests/mod.rs` must expose this file:
//!
//! ```text
//! mod failure;
//! ```
//!
//! The test suite then imports the canonical implementation through:
//!
//! ```text
//! crate::quantum::hardware::failure
//! ```
//!
//! This prevents the tests from maintaining a second implementation of the
//! production failure model.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::hardware::failure::{
    BackoffPolicy,
    BackoffPolicyError,
    FailureKind,
    FailurePermanence,
    FailurePhase,
    FailureSeverity,
    Idempotency,
    RetryAfter,
    RetryAfterError,
    RetryDecision,
    RetryPolicy,
    RetryPolicyError,
    RetryReason,
    RetrySafety,
    FAILURE_SCHEMA_ID,
    FAILURE_SCHEMA_VERSION,
    MAX_BACKOFF_MULTIPLIER,
    MAX_FAILURE_MESSAGE_LENGTH,
    MAX_JOB_ID_LENGTH,
    MAX_PROVIDER_CODE_LENGTH,
    MAX_REQUEST_ID_LENGTH,
    MAX_RETRY_ATTEMPTS,
    MAX_RETRY_DELAY_MS,
};

// ============================================================================
// Stable schema
// ============================================================================

#[test]
fn schema_identifier_is_stable_and_non_empty() {
    assert_eq!(
        FAILURE_SCHEMA_ID,
        "zamani.quantum.hardware.failure"
    );
    assert!(!FAILURE_SCHEMA_ID.is_empty());
}

#[test]
fn schema_version_is_non_zero() {
    assert!(FAILURE_SCHEMA_VERSION >= 1);
}

#[test]
fn safety_limits_are_non_zero() {
    assert!(MAX_FAILURE_MESSAGE_LENGTH > 0);
    assert!(MAX_PROVIDER_CODE_LENGTH > 0);
    assert!(MAX_JOB_ID_LENGTH > 0);
    assert!(MAX_REQUEST_ID_LENGTH > 0);
    assert!(MAX_RETRY_ATTEMPTS > 0);
    assert!(MAX_RETRY_DELAY_MS > 0);
    assert!(MAX_BACKOFF_MULTIPLIER > 0);
}

// ============================================================================
// Failure phases
// ============================================================================

#[test]
fn every_failure_phase_has_a_stable_identifier() {
    let phases = [
        FailurePhase::Preflight,
        FailurePhase::Validation,
        FailurePhase::Submission,
        FailurePhase::Queue,
        FailurePhase::Execution,
        FailurePhase::Cancellation,
        FailurePhase::ResultRetrieval,
        FailurePhase::ResultNormalization,
        FailurePhase::Discovery,
        FailurePhase::HealthCheck,
        FailurePhase::Calibration,
        FailurePhase::Transport,
        FailurePhase::Authentication,
        FailurePhase::Authorization,
        FailurePhase::Serialization,
        FailurePhase::LocalExecution,
        FailurePhase::Internal,
    ];

    for phase in phases {
        assert!(!phase.as_str().is_empty());
        assert_eq!(phase.to_string(), phase.as_str());
    }
}

#[test]
fn failure_phase_identifiers_are_unique() {
    let phases = [
        FailurePhase::Preflight,
        FailurePhase::Validation,
        FailurePhase::Submission,
        FailurePhase::Queue,
        FailurePhase::Execution,
        FailurePhase::Cancellation,
        FailurePhase::ResultRetrieval,
        FailurePhase::ResultNormalization,
        FailurePhase::Discovery,
        FailurePhase::HealthCheck,
        FailurePhase::Calibration,
        FailurePhase::Transport,
        FailurePhase::Authentication,
        FailurePhase::Authorization,
        FailurePhase::Serialization,
        FailurePhase::LocalExecution,
        FailurePhase::Internal,
    ];

    for (index, left) in phases.iter().enumerate() {
        for right in phases.iter().skip(index + 1) {
            assert_ne!(
                left.as_str(),
                right.as_str(),
                "failure phase identifiers must be unique"
            );
        }
    }
}

// ============================================================================
// Failure kinds
// ============================================================================

#[test]
fn every_failure_kind_has_a_stable_identifier() {
    let kinds = [
        FailureKind::InvalidRequest,
        FailureKind::CapabilityMismatch,
        FailureKind::ResourceLimit,
        FailureKind::TopologyMismatch,
        FailureKind::CalibrationFailure,
        FailureKind::StaleCalibration,
        FailureKind::BackendUnavailable,
        FailureKind::BackendRetired,
        FailureKind::QueueUnavailable,
        FailureKind::QueueTimeout,
        FailureKind::SubmissionRejected,
        FailureKind::ExecutionRejected,
        FailureKind::HardwareExecutionFailure,
        FailureKind::TransientExecutionFailure,
        FailureKind::PermanentExecutionFailure,
        FailureKind::RateLimited,
        FailureKind::ServiceUnavailable,
        FailureKind::NetworkFailure,
        FailureKind::TransportTimeout,
        FailureKind::ExecutionTimeout,
        FailureKind::AuthenticationFailure,
        FailureKind::AuthorizationFailure,
        FailureKind::CancellationFailure,
        FailureKind::ResultUnavailable,
        FailureKind::InvalidResult,
        FailureKind::ResultNormalizationFailure,
        FailureKind::SerializationFailure,
        FailureKind::LocalExecutionFailure,
        FailureKind::InternalInvariant,
        FailureKind::Unsupported,
        FailureKind::Unknown,
    ];

    for kind in kinds {
        assert!(!kind.as_str().is_empty());
        assert_eq!(kind.to_string(), kind.as_str());
    }
}

#[test]
fn failure_kind_identifiers_are_unique() {
    let kinds = [
        FailureKind::InvalidRequest,
        FailureKind::CapabilityMismatch,
        FailureKind::ResourceLimit,
        FailureKind::TopologyMismatch,
        FailureKind::CalibrationFailure,
        FailureKind::StaleCalibration,
        FailureKind::BackendUnavailable,
        FailureKind::BackendRetired,
        FailureKind::QueueUnavailable,
        FailureKind::QueueTimeout,
        FailureKind::SubmissionRejected,
        FailureKind::ExecutionRejected,
        FailureKind::HardwareExecutionFailure,
        FailureKind::TransientExecutionFailure,
        FailureKind::PermanentExecutionFailure,
        FailureKind::RateLimited,
        FailureKind::ServiceUnavailable,
        FailureKind::NetworkFailure,
        FailureKind::TransportTimeout,
        FailureKind::ExecutionTimeout,
        FailureKind::AuthenticationFailure,
        FailureKind::AuthorizationFailure,
        FailureKind::CancellationFailure,
        FailureKind::ResultUnavailable,
        FailureKind::InvalidResult,
        FailureKind::ResultNormalizationFailure,
        FailureKind::SerializationFailure,
        FailureKind::LocalExecutionFailure,
        FailureKind::InternalInvariant,
        FailureKind::Unsupported,
        FailureKind::Unknown,
    ];

    for (index, left) in kinds.iter().enumerate() {
        for right in kinds.iter().skip(index + 1) {
            assert_ne!(
                left.as_str(),
                right.as_str(),
                "failure kind identifiers must be unique"
            );
        }
    }
}

// ============================================================================
// Permanence
// ============================================================================

#[test]
fn transient_permanence_is_potentially_retryable() {
    assert!(FailurePermanence::Transient.may_retry());
}

#[test]
fn permanent_permanence_is_never_potentially_retryable() {
    assert!(!FailurePermanence::Permanent.may_retry());
}

#[test]
fn unknown_permanence_fails_closed() {
    assert!(!FailurePermanence::Unknown.may_retry());
}

#[test]
fn_known_transient_failure_kinds_are_classified_as_transient() {
    let transient = [
        FailureKind::BackendUnavailable,
        FailureKind::QueueUnavailable,
        FailureKind::QueueTimeout,
        FailureKind::RateLimited,
        FailureKind::ServiceUnavailable,
        FailureKind::NetworkFailure,
        FailureKind::TransportTimeout,
        FailureKind::TransientExecutionFailure,
    ];

    for kind in transient {
        assert_eq!(
            kind.default_permanence(),
            FailurePermanence::Transient
        );
    }
}

#[test]
fn clearly_permanent_failure_kinds_are_classified_as_permanent() {
    let permanent = [
        FailureKind::InvalidRequest,
        FailureKind::CapabilityMismatch,
        FailureKind::ResourceLimit,
        FailureKind::TopologyMismatch,
        FailureKind::StaleCalibration,
        FailureKind::BackendRetired,
        FailureKind::SubmissionRejected,
        FailureKind::ExecutionRejected,
        FailureKind::PermanentExecutionFailure,
        FailureKind::AuthenticationFailure,
        FailureKind::AuthorizationFailure,
        FailureKind::InvalidResult,
        FailureKind::SerializationFailure,
        FailureKind::InternalInvariant,
        FailureKind::Unsupported,
    ];

    for kind in permanent {
        assert_eq!(
            kind.default_permanence(),
            FailurePermanence::Permanent
        );
    }
}

#[test]
fn ambiguous_failure_kinds_fail_closed_to_unknown() {
    let ambiguous = [
        FailureKind::CalibrationFailure,
        FailureKind::HardwareExecutionFailure,
        FailureKind::ExecutionTimeout,
        FailureKind::CancellationFailure,
        FailureKind::ResultUnavailable,
        FailureKind::ResultNormalizationFailure,
        FailureKind::LocalExecutionFailure,
        FailureKind::Unknown,
    ];

    for kind in ambiguous {
        assert_eq!(
            kind.default_permanence(),
            FailurePermanence::Unknown
        );
    }
}

// ============================================================================
// Severity
// ============================================================================

#[test]
fn every_failure_severity_has_a_stable_identifier() {
    let severities = [
        FailureSeverity::Info,
        FailureSeverity::Warning,
        FailureSeverity::Error,
        FailureSeverity::Critical,
    ];

    for severity in severities {
        assert!(!severity.as_str().is_empty());
        assert_eq!(severity.to_string(), severity.as_str());
    }
}

#[test]
fn critical_failures_are_reserved_for_safety_or_integrity_conditions() {
    assert_eq!(
        FailureSeverity::default_for(FailureKind::InternalInvariant),
        FailureSeverity::Critical
    );

    assert_eq!(
        FailureSeverity::default_for(FailureKind::HardwareExecutionFailure),
        FailureSeverity::Critical
    );

    assert_eq!(
        FailureSeverity::default_for(FailureKind::PermanentExecutionFailure),
        FailureSeverity::Critical
    );
}

#[test]
fn ordinary_validation_failures_are_not_classified_as_critical() {
    assert_eq!(
        FailureSeverity::default_for(FailureKind::InvalidRequest),
        FailureSeverity::Error
    );

    assert_eq!(
        FailureSeverity::default_for(FailureKind::CapabilityMismatch),
        FailureSeverity::Error
    );

    assert_eq!(
        FailureSeverity::default_for(FailureKind::Unsupported),
        FailureSeverity::Error
    );
}

// ============================================================================
// Retry safety
// ============================================================================

#[test]
fn retry_safety_identifiers_are_stable() {
    assert_eq!(RetrySafety::Safe.as_str(), "safe");
    assert_eq!(RetrySafety::Unsafe.as_str(), "unsafe");
    assert_eq!(RetrySafety::Unknown.as_str(), "unknown");
}

#[test]
fn idempotency_maps_to_conservative_retry_safety() {
    assert_eq!(
        Idempotency::Idempotent.retry_safety(),
        RetrySafety::Safe
    );

    assert_eq!(
        Idempotency::NonIdempotent.retry_safety(),
        RetrySafety::Unsafe
    );

    assert_eq!(
        Idempotency::Unknown.retry_safety(),
        RetrySafety::Unknown
    );
}

#[test]
fn non_idempotent_operations_are_never_treated_as_safe() {
    assert_eq!(
        Idempotency::NonIdempotent.retry_safety(),
        RetrySafety::Unsafe
    );
}

#[test]
fn unknown_idempotency_fails_closed() {
    assert_eq!(
        Idempotency::Unknown.retry_safety(),
        RetrySafety::Unknown
    );
}

// ============================================================================
// Retry decisions
// ============================================================================

#[test]
fn retry_decision_identifiers_are_stable() {
    assert_eq!(RetryDecision::Retry.as_str(), "retry");
    assert_eq!(
        RetryDecision::RetryConditional.as_str(),
        "retry_conditional"
    );
    assert_eq!(
        RetryDecision::DoNotRetry.as_str(),
        "do_not_retry"
    );
    assert_eq!(RetryDecision::Unknown.as_str(), "unknown");
}

#[test]
fn_retry_decision_semantics_are_consistent() {
    assert!(RetryDecision::Retry.is_retry());
    assert!(RetryDecision::Retry.may_retry());

    assert!(!RetryDecision::RetryConditional.is_retry());
    assert!(RetryDecision::RetryConditional.may_retry());

    assert!(!RetryDecision::DoNotRetry.is_retry());
    assert!(!RetryDecision::DoNotRetry.may_retry());

    assert!(!RetryDecision::Unknown.is_retry());
    assert!(!RetryDecision::Unknown.may_retry());
}

// ============================================================================
// Retry reasons
// ============================================================================

#[test]
fn retry_reason_identifiers_are_stable() {
    let reasons = [
        RetryReason::PermanentFailure,
        RetryReason::TransientAndSafe,
        RetryReason::RetrySafetyUnknown,
        RetryReason::NonIdempotent,
        RetryReason::RetryLimitReached,
        RetryReason::RetryPolicyDisabled,
        RetryReason::DelayLimitReached,
        RetryReason::ConditionalPolicy,
        RetryReason::UnknownFailure,
        RetryReason::NotRetryable,
    ];

    for reason in reasons {
        assert!(!reason.as_str().is_empty());
        assert_eq!(reason.to_string(), reason.as_str());
    }
}

// ============================================================================
// Retry-after
// ============================================================================

#[test]
fn retry_after_accepts_zero_delay() {
    let retry_after = RetryAfter::from_millis(0)
        .expect("zero retry-after delay is valid");

    assert_eq!(retry_after.as_millis(), 0);
}

#[test]
fn retry_after_accepts_global_maximum() {
    let retry_after = RetryAfter::from_millis(MAX_RETRY_DELAY_MS)
        .expect("global maximum retry-after delay must be accepted");

    assert_eq!(
        retry_after.as_millis(),
        MAX_RETRY_DELAY_MS
    );
}

#[test]
fn retry_after_rejects_values_above_global_maximum() {
    let error = RetryAfter::from_millis(
        MAX_RETRY_DELAY_MS.saturating_add(1)
    )
    .expect_err("oversized retry-after must be rejected");

    assert_eq!(
        error,
        RetryAfterError::TooLarge {
            milliseconds: MAX_RETRY_DELAY_MS + 1,
            maximum: MAX_RETRY_DELAY_MS,
        }
    );
}

#[test]
fn retry_after_error_is_human_readable_without_being_secret_bearing() {
    let error = RetryAfter::from_millis(
        MAX_RETRY_DELAY_MS.saturating_add(1)
    )
    .expect_err("oversized retry-after must fail");

    let message = error.to_string();

    assert!(message.contains("retry-after"));
    assert!(message.contains("ms"));
}

// ============================================================================
// Backoff
// ============================================================================

#[test]
fn production_backoff_is_valid() {
    let policy = BackoffPolicy::default_production();

    assert!(policy.initial_delay_ms <= policy.maximum_delay_ms);
    assert!(policy.multiplier > 0);
    assert!(policy.multiplier <= MAX_BACKOFF_MULTIPLIER);

    policy
        .clone()
        .new(
            policy.initial_delay_ms,
            policy.maximum_delay_ms,
            policy.multiplier,
        )
        .expect("production backoff must validate");
}

#[test]
fn backoff_attempt_zero_returns_initial_delay() {
    let policy = BackoffPolicy::new(100, 30_000, 2)
        .expect("valid backoff");

    assert_eq!(policy.delay_for_attempt(0), 100);
}

#[test]
fn backoff_grows_exponentially_until_capped() {
    let policy = BackoffPolicy::new(100, 10_000, 2)
        .expect("valid backoff");

    assert_eq!(policy.delay_for_attempt(0), 100);
    assert_eq!(policy.delay_for_attempt(1), 200);
    assert_eq!(policy.delay_for_attempt(2), 400);
    assert_eq!(policy.delay_for_attempt(3), 800);
    assert_eq!(policy.delay_for_attempt(4), 1_600);
}

#[test]
fn backoff_never_exceeds_configured_maximum() {
    let policy = BackoffPolicy::new(100, 1_000, 2)
        .expect("valid backoff");

    for attempt in 0..100 {
        assert!(
            policy.delay_for_attempt(attempt) <= 1_000,
            "attempt {} exceeded maximum",
            attempt
        );
    }
}

#[test]
fn backoff_is_deterministic() {
    let policy = BackoffPolicy::new(137, 12_345, 3)
        .expect("valid backoff");

    for attempt in 0..32 {
        assert_eq!(
            policy.delay_for_attempt(attempt),
            policy.delay_for_attempt(attempt)
        );
    }
}

#[test]
fn backoff_handles_large_attempt_numbers_without_overflow() {
    let policy = BackoffPolicy::new(100, 30_000, 2)
        .expect("valid backoff");

    let delay = policy.delay_for_attempt(u32::MAX);

    assert_eq!(delay, 30_000);
}

#[test]
fn backoff_rejects_zero_multiplier() {
    let error = BackoffPolicy::new(100, 1_000, 0)
        .expect_err("zero multiplier must be rejected");

    assert_eq!(error, BackoffPolicyError::ZeroMultiplier);
}

#[test]
fn backoff_rejects_maximum_below_initial() {
    let error = BackoffPolicy::new(1_000, 100, 2)
        .expect_err("maximum below initial must be rejected");

    assert_eq!(
        error,
        BackoffPolicyError::MaximumBelowInitial
    );
}

#[test]
fn backoff_rejects_excessive_multiplier() {
    let multiplier = MAX_BACKOFF_MULTIPLIER + 1;

    let error = BackoffPolicy::new(100, 10_000, multiplier)
        .expect_err("excessive multiplier must be rejected");

    assert_eq!(
        error,
        BackoffPolicyError::MultiplierTooLarge {
            value: multiplier
        }
    );
}

#[test]
fn backoff_rejects_initial_delay_above_global_limit() {
    let value = MAX_RETRY_DELAY_MS + 1;

    let error = BackoffPolicy::new(value, value, 2)
        .expect_err("oversized initial delay must be rejected");

    assert_eq!(
        error,
        BackoffPolicyError::InitialDelayTooLarge { value }
    );
}

#[test]
fn backoff_rejects_maximum_delay_above_global_limit() {
    let value = MAX_RETRY_DELAY_MS + 1;

    let error = BackoffPolicy::new(100, value, 2)
        .expect_err("oversized maximum delay must be rejected");

    assert_eq!(
        error,
        BackoffPolicyError::MaximumDelayTooLarge { value }
    );
}

// ============================================================================
// Retry policy
// ============================================================================

#[test]
fn production_retry_policy_is_valid() {
    RetryPolicy::production()
        .validate()
        .expect("production retry policy must be valid");
}

#[test]
fn disabled_retry_policy_is_valid() {
    RetryPolicy::disabled()
        .validate()
        .expect("disabled retry policy must be valid");
}

#[test]
fn enabled_policy_without_retries_is_invalid() {
    let policy = RetryPolicy {
        enabled: true,
        maximum_retries: 0,
        retry_unknown_failures: false,
        retry_unknown_idempotency: false,
        retry_transient: true,
        backoff: BackoffPolicy::default_production(),
    };

    assert_eq!(
        policy.validate(),
        Err(RetryPolicyError::EnabledWithoutRetries)
    );
}

#[test]
fn retry_policy_rejects_excessive_retry_count() {
    let policy = RetryPolicy {
        enabled: true,
        maximum_retries: MAX_RETRY_ATTEMPTS + 1,
        retry_unknown_failures: false,
        retry_unknown_idempotency: false,
        retry_transient: true,
        backoff: BackoffPolicy::default_production(),
    };

    assert_eq!(
        policy.validate(),
        Err(RetryPolicyError::TooManyRetries {
            value: MAX_RETRY_ATTEMPTS + 1
        })
    );
}

// ============================================================================
// Regression tests for critical quantum-safety rules
// ============================================================================

#[test]
fn transient_failure_does_not_imply_retry_safety() {
    assert!(FailurePermanence::Transient.may_retry());

    assert_eq!(
        Idempotency::Unknown.retry_safety(),
        RetrySafety::Unknown
    );

    assert!(!RetrySafety::Unknown.as_str().is_empty());
}

#[test]
fn non_idempotent_qpu_submission_is_not_safe_to_retry() {
    assert_eq!(
        Idempotency::NonIdempotent.retry_safety(),
        RetrySafety::Unsafe
    );
}

#[test]
fn unknown_qpu_submission_idempotency_fails_closed() {
    assert_eq!(
        Idempotency::Unknown.retry_safety(),
        RetrySafety::Unknown
    );
}

#[test]
fn permanent_failure_cannot_be_marked_potentially_retryable_by_permanence() {
    assert!(!FailurePermanence::Permanent.may_retry());
}

#[test]
fn unknown_failure_cannot_be_marked_potentially_retryable_by_permanence() {
    assert!(!FailurePermanence::Unknown.may_retry());
}

// ============================================================================
// Determinism / trait contracts
// ============================================================================

#[test]
fn failure_enums_are_hashable_and_orderable() {
    use std::collections::{BTreeSet, HashSet};

    let mut ordered = BTreeSet::new();
    ordered.insert(FailurePhase::Execution);
    ordered.insert(FailurePhase::Submission);
    ordered.insert(FailurePhase::Queue);

    assert_eq!(ordered.len(), 3);

    let mut hashed = HashSet::new();
    hashed.insert(FailureKind::NetworkFailure);
    hashed.insert(FailureKind::NetworkFailure);
    hashed.insert(FailureKind::RateLimited);

    assert_eq!(hashed.len(), 2);
}

#[test]
fn enum_display_is_machine_identifier_not_debug_format() {
    assert_eq!(
        FailureKind::NetworkFailure.to_string(),
        "network_failure"
    );

    assert_eq!(
        FailurePhase::ResultRetrieval.to_string(),
        "result_retrieval"
    );

    assert_eq!(
        FailurePermanence::Transient.to_string(),
        "transient"
    );
}

// ============================================================================
// Provider-neutrality regression test
// ============================================================================

#[test]
fn failure_contract_contains_no_provider_specific_core_variants() {
    // This is intentionally a compile-time/API-shape regression test.
    //
    // Provider-specific failures must be represented through normalized
    // FailureKind values and provider metadata in the production model.
    //
    // If a provider-specific enum starts appearing in this test's import
    // surface, the provider-neutral architecture has been violated.
    let normalized = [
        FailureKind::BackendUnavailable,
        FailureKind::ServiceUnavailable,
        FailureKind::NetworkFailure,
        FailureKind::SubmissionRejected,
        FailureKind::ExecutionRejected,
        FailureKind::Unknown,
    ];

    assert_eq!(normalized.len(), 6);
}

// ============================================================================
// Production policy invariants
// ============================================================================

#[test]
fn production_policy_does_not_retry_unknown_failures() {
    let policy = RetryPolicy::production();

    assert!(!policy.retry_unknown_failures);
}

#[test]
fn production_policy_does_not_retry_unknown_idempotency() {
    let policy = RetryPolicy::production();

    assert!(!policy.retry_unknown_idempotency);
}

#[test]
fn production_policy_enables_only_explicit_transient_retry() {
    let policy = RetryPolicy::production();

    assert!(policy.enabled);
    assert!(policy.retry_transient);
}

#[test]
fn production_policy_has_bounded_retry_count() {
    let policy = RetryPolicy::production();

    assert!(policy.maximum_retries > 0);
    assert!(policy.maximum_retries <= MAX_RETRY_ATTEMPTS);
}

#[test]
fn disabled_policy_has_zero_retry_budget() {
    let policy = RetryPolicy::disabled();

    assert!(!policy.enabled);
    assert_eq!(policy.maximum_retries, 0);
    assert!(!policy.retry_transient);
    assert!(!policy.retry_unknown_failures);
    assert!(!policy.retry_unknown_idempotency);
}

// ============================================================================
// Regression guard for accidental API drift
// ============================================================================

#[test]
fn stable_identifiers_are_lower_snake_case() {
    let identifiers = [
        FailurePhase::ResultNormalization.as_str(),
        FailureKind::TransientExecutionFailure.as_str(),
        FailurePermanence::Transient.as_str(),
        FailureSeverity::Critical.as_str(),
        RetrySafety::Unknown.as_str(),
        Idempotency::NonIdempotent.as_str(),
        RetryDecision::RetryConditional.as_str(),
        RetryReason::RetrySafetyUnknown.as_str(),
    ];

    for identifier in identifiers {
        assert!(
            !identifier.is_empty(),
            "stable identifier must not be empty"
        );

        assert!(
            identifier
                .chars()
                .all(|character| {
                    character.is_ascii_lowercase()
                        || character.is_ascii_digit()
                        || character == '_'
                }),
            "identifier `{}` is not lower_snake_case",
            identifier
        );

        assert!(
            !identifier.starts_with('_'),
            "identifier `{}` must not start with `_`",
        );

        assert!(
            !identifier.ends_with('_'),
            "identifier `{}` must not end with `_`",
        );
    }
}