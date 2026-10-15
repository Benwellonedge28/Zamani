#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Public API Integration Tests
//!
//! Path: `src/zuir/tests/api.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety policy: unsafe Rust is prohibited.
//!
//! STATUS
//!
//! Proposed integration-test module. Freeze only after the module is wired
//! into the repository's test tree and the applicable Cargo checks pass.
//!
//! PURPOSE
//!
//! Exercise the supported public ZUIR API as a consumer would use it.
//! These tests validate configuration ownership, request validation,
//! deterministic defaults, error preservation, phase handling, and the
//! facade's delegation contract.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/api.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/builder/`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! OWNS
//!
//! - Regression tests for the public API facade.
//! - Assertions about documented API-boundary behavior.
//! - Consumer-level checks for public configuration and error contracts.
//!
//! DOES_NOT_OWN
//!
//! - IR construction, operation semantics, or verification algorithms.
//! - Source parsing, AST construction, or semantic analysis.
//! - Dialect registration, serialization, optimization, or lowering.
//! - Hardware discovery, resource allocation, or runtime execution.
//! - Private implementation details of sibling modules.
//!
//! INTEGRATION CONTRACT
//!
//! The tests use the canonical public API in `crate::zuir::api`.
//! They do not depend on private fields or require a particular internal
//! representation of configuration, errors, or future IR data structures.
//!
//! The test module must be declared by the actual parent test module.
//! Do not add a second definition of ZuirApi or duplicate production logic.
//!
//! SCALABILITY AND POCO-REAF
//!
//! These tests introduce no hardware capacities, fixed operation counts,
//! fixed qubit counts, or assumptions about target architecture.
//!
//! They test the API boundary, not literal infinite execution. Actual work
//! remains bounded by available resources and the configured policies of
//! the subsystem performing that work.
//!
//! FREEZE CRITERIA
//!
//! - The test module is connected to the actual repository module tree.
//! - Every imported symbol exists and is publicly accessible.
//! - Tests compile and pass against the real implementation.
//! - No test relies on a particular backend or physical target.
//! - Formatting, compilation, relevant tests, and Clippy pass.

// ---------------------------------------------------------------------------
// Public API imports
// ---------------------------------------------------------------------------

use crate::zuir::api::{
    validate_config,
    validate_operation_label,
    CompatibilityPolicy,
    ErrorPhase,
    ExtensionPolicy,
    VerificationPolicy,
    ZuirApi,
    ZuirConfig,
    ZuirError,
    ZuirErrorKind,
    ZuirResult,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Assert that the public facade can be shared between threads.
///
/// This is a compile-time contract check. It does not spawn threads or assume
/// that the facade owns shared mutable compiler state.
fn assert_send_sync<T: Send + Sync>() {}

/// Construct an API with the ordinary, valid default configuration.
fn default_api() -> ZuirApi {
    ZuirApi::with_config(ZuirConfig::default())
        .expect("the default ZUIR configuration must be valid")
}

// ---------------------------------------------------------------------------
// Construction and configuration
// ---------------------------------------------------------------------------

#[test]
fn default_constructor_produces_a_valid_api() {
    let api = ZuirApi::default();

    assert!(
        api.validate_config().is_ok(),
        "default configuration must validate"
    );

    assert!(
        api.config().deterministic(),
        "determinism must be a correctness guarantee"
    );

    assert!(
        api.validate_request("tests.api.default").is_ok(),
        "a non-empty operation label must be accepted"
    );
}

#[test]
fn new_default_matches_default_constructor() {
    let ordinary = ZuirApi::default();
    let explicit = ZuirApi::new_default();

    assert_eq!(ordinary, explicit);
    assert_eq!(ordinary.config(), explicit.config());
}

#[test]
fn caller_configuration_is_retained_without_mutation() {
    let config = ZuirConfig::default()
        .with_verification_policy(VerificationPolicy::Thorough)
        .with_compatibility_policy(CompatibilityPolicy::Compatible)
        .with_extension_policy(ExtensionPolicy::AllowExperimental);

    let expected = config.clone();

    let api = ZuirApi::with_config(config)
        .expect("a valid default-derived configuration must be accepted");

    assert_eq!(api.config(), &expected);
    assert!(api.validate_config().is_ok());
}

#[test]
fn standalone_configuration_validation_uses_the_canonical_contract() {
    let config = ZuirConfig::default();

    assert!(validate_config(&config).is_ok());
}

#[test]
fn configuration_conversion_preserves_the_configuration() {
    let config = ZuirConfig::default()
        .with_verification_policy(VerificationPolicy::Thorough);

    let expected = config.clone();
    let api = ZuirApi::from(config);

    assert_eq!(api.config(), &expected);
    assert!(api.validate_config().is_ok());
}

#[test]
fn cloned_api_preserves_configuration_and_equality() {
    let api = default_api();
    let cloned = api.clone();

    assert_eq!(api, cloned);
    assert_eq!(api.config(), cloned.config());

    // Cloning an API must not alter the original configuration.
    assert!(api.validate_config().is_ok());
    assert!(cloned.validate_config().is_ok());
}

#[test]
fn public_facade_is_send_and_sync() {
    assert_send_sync::<ZuirApi>();
}

// ---------------------------------------------------------------------------
// Request-boundary validation
// ---------------------------------------------------------------------------

#[test]
fn operation_label_validation_accepts_nonempty_labels() {
    let labels = [
        "verify",
        "serialize.program",
        "lower.quantum",
        "hdl.synthesis",
        "tensor.compute",
        "distributed.execute",
        "extension.example.operation",
        "operation with spaces",
        "操作",
    ];

    for label in labels {
        assert!(
            validate_operation_label(label).is_ok(),
            "expected valid operation label: {label:?}"
        );
    }
}

#[test]
fn operation_label_validation_rejects_empty_and_whitespace_labels() {
    let labels = [
        "",
        " ",
        "\t",
        "\n",
        "\r\n",
        " \t\n ",
    ];

    for label in labels {
        let result = validate_operation_label(label);

        assert!(
            result.is_err(),
            "expected invalid operation label: {label:?}"
        );

        let error = result.expect_err("invalid labels must produce an error");

        assert_eq!(error.kind(), ZuirErrorKind::InvalidInput);
        assert_eq!(
            error.context().phase(),
            Some(ErrorPhase::Configuration)
        );
    }
}

#[test]
fn request_validation_rejects_invalid_label_before_work_starts() {
    let api = default_api();
    let mut invoked = false;

    let result = api.run(" \t ", |_config| {
        invoked = true;
        Ok(())
    });

    assert!(result.is_err());
    assert!(
        !invoked,
        "invalid requests must be rejected before delegated work"
    );

    let error = result.expect_err("invalid label must fail");

    assert_eq!(error.kind(), ZuirErrorKind::InvalidInput);
    assert_eq!(
        error.context().phase(),
        Some(ErrorPhase::Configuration)
    );
}

#[test]
fn direct_request_validation_accepts_a_valid_label() {
    let api = default_api();

    assert!(
        api.validate_request("tests.api.request").is_ok()
    );
}

// ---------------------------------------------------------------------------
// Delegation behavior
// ---------------------------------------------------------------------------

#[test]
fn run_passes_the_configured_configuration_to_the_closure() {
    let config = ZuirConfig::default()
        .with_verification_policy(VerificationPolicy::Thorough);

    let api = ZuirApi::with_config(config.clone())
        .expect("configuration must be valid");

    let result = api.run("tests.api.inspect_config", |received| {
        assert_eq!(received, &config);
        assert_eq!(
            received.verification_policy(),
            VerificationPolicy::Thorough
        );

        Ok(73_u64)
    });

    assert_eq!(
        result.expect("delegated operation should succeed"),
        73
    );
}

#[test]
fn run_invokes_a_valid_operation_exactly_once() {
    let api = default_api();
    let mut invocation_count = 0_usize;

    let result = api.run("tests.api.single_invocation", |_config| {
        invocation_count += 1;
        Ok(())
    });

    assert!(result.is_ok());
    assert_eq!(invocation_count, 1);
}

#[test]
fn run_returns_the_success_value_unchanged() {
    let api = default_api();

    let expected = String::from("ZUIR public API result");

    let actual = api
        .run("tests.api.return_value", |_config| {
            Ok(expected.clone())
        })
        .expect("delegated operation should succeed");

    assert_eq!(actual, expected);
}

#[test]
fn run_propagates_the_original_error_kind() {
    let api = default_api();

    let result: ZuirResult<()> = api.run(
        "tests.api.expected_failure",
        |_config| {
            Err(ZuirError::new(
                ZuirErrorKind::UnsupportedOperation,
                "no supported implementation is registered",
            ))
        },
    );

    let error = result.expect_err("the delegated failure must propagate");

    assert_eq!(error.kind(), ZuirErrorKind::UnsupportedOperation);

    assert!(
        error
            .to_string()
            .contains("no supported implementation is registered"),
        "the original explanatory message must remain available"
    );
}

#[test]
fn run_adds_operation_context_to_a_delegated_failure() {
    let api = default_api();
    let label = "tests.api.context_preservation";

    let result: ZuirResult<()> = api.run(label, |_config| {
        Err(ZuirError::new(
            ZuirErrorKind::UnsupportedOperation,
            "unsupported test operation",
        ))
    });

    let error = result.expect_err("the delegated failure must propagate");

    let has_operation_context = error
        .context()
        .notes()
        .iter()
        .any(|note| note.contains(label));

    assert!(
        has_operation_context,
        "the API operation label must be preserved as contextual information"
    );
}

#[test]
fn run_in_phase_adds_a_fallback_phase_when_none_exists() {
    let api = default_api();

    let result: ZuirResult<()> = api.run_in_phase(
        ErrorPhase::Verification,
        "tests.api.phase_fallback",
        |_config| {
            Err(ZuirError::new(
                ZuirErrorKind::VerificationFailed,
                "verification failed",
            ))
        },
    );

    let error = result.expect_err("the verification failure must propagate");

    assert_eq!(error.kind(), ZuirErrorKind::VerificationFailed);
    assert_eq!(
        error.context().phase(),
        Some(ErrorPhase::Verification)
    );
}

#[test]
fn run_in_phase_preserves_an_existing_more_specific_phase() {
    let api = default_api();

    let result: ZuirResult<()> = api.run_in_phase(
        ErrorPhase::Verification,
        "tests.api.phase_preservation",
        |_config| {
            Err(
                ZuirError::new(
                    ZuirErrorKind::InvalidInput,
                    "invalid configuration input",
                )
                .with_phase(ErrorPhase::Configuration),
            )
        },
    );

    let error = result.expect_err("the delegated failure must propagate");

    assert_eq!(error.kind(), ZuirErrorKind::InvalidInput);
    assert_eq!(
        error.context().phase(),
        Some(ErrorPhase::Configuration),
        "a delegated phase must not be overwritten by the fallback"
    );
}

#[test]
fn run_in_phase_rejects_invalid_requests_before_invocation() {
    let api = default_api();
    let mut invoked = false;

    let result = api.run_in_phase(
        ErrorPhase::Verification,
        "",
        |_config| {
            invoked = true;
            Ok(())
        },
    );

    assert!(result.is_err());
    assert!(
        !invoked,
        "invalid requests must not reach the delegated closure"
    );
}

// ---------------------------------------------------------------------------
// Error contract and deterministic behavior
// ---------------------------------------------------------------------------

#[test]
fn invalid_request_error_has_a_stable_machine_readable_category() {
    let api = default_api();

    let error = api
        .run("", |_config| Ok(()))
        .expect_err("an empty label must be rejected");

    assert_eq!(error.kind(), ZuirErrorKind::InvalidInput);
    assert_eq!(
        error.context().phase(),
        Some(ErrorPhase::Configuration)
    );
}

#[test]
fn repeated_default_construction_produces_equivalent_configuration() {
    let first = ZuirApi::default();
    let second = ZuirApi::default();

    assert_eq!(first.config(), second.config());
    assert!(first.config().deterministic());
    assert!(second.config().deterministic());
}

#[test]
fn operation_labels_are_context_not_backend_selection() {
    let api = default_api();

    // A label is diagnostic context. The facade must pass it only through
    // the API boundary and must not treat it as an executable instruction.
    let label = "quantum.lower";

    let result = api.run(label, |_config| Ok("delegated"));

    assert_eq!(
        result.expect("the supplied closure determines this result"),
        "delegated"
    );
}

// ---------------------------------------------------------------------------
// Resource and portability contract
// ---------------------------------------------------------------------------

#[test]
fn facade_does_not_require_a_specific_computational_domain() {
    let api = default_api();

    let labels = [
        "classical.verify",
        "quantum.verify",
        "hdl.verify",
        "tensor.verify",
        "ai.verify",
        "distributed.verify",
        "networking.verify",
        "crypto.verify",
        "future_domain.verify",
    ];

    for label in labels {
        assert!(
            api.run(label, |_config| Ok(())).is_ok(),
            "the generic facade must not reject a domain merely by name: {label}"
        );
    }
}

#[test]
fn facade_does_not_claim_that_delegation_performs_ir_verification() {
    let api = default_api();

    // This API is a generic invocation boundary. A successful closure
    // indicates only that the delegated closure succeeded. Actual IR
    // verification must be performed by the canonical verifier subsystem.
    let result = api.run("tests.api.delegation_contract", |_config| {
        Ok("delegation succeeded")
    });

    assert_eq!(
        result.expect("the test closure succeeds"),
        "delegation succeeded"
    );
}