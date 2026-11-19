
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Regression Tests
//!
//! Path: `src/toolchain/testing/regression.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Preserve previously established toolchain behavior across changes to
//! the project, build, dependency, package, registry, target, provider,
//! security, and deployment subsystems.
//!
//! This module is a regression-test suite. It is not a production service,
//! compiler, package manager, security implementation, or test executor.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Regression assertions for public toolchain contracts.
//! - Regression assertions for canonical error and result behavior.
//! - Regression assertions for typed toolchain identities.
//! - Regression assertions for independently versioned contracts.
//! - Regression assertions for conformance evidence handling.
//! - Regression assertions for fail-closed security validation.
//! - Small synthetic providers used only by these tests.
//!
//! It does not own:
//!
//! - Production toolchain APIs.
//! - Project or manifest parsing.
//! - Dependency resolution.
//! - Build planning or execution.
//! - Compiler or backend behavior.
//! - Package serialization or extraction.
//! - Cryptographic algorithms.
//! - Trust-store implementation.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Production provider registration.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Normative references
//!
//! Architecture:
//!     grammar/DESIGN.md
//!
//! POCO-REAF:
//!     grammar/specification/poco-reaf.md
//!
//! Scalability:
//!     grammar/specification/scalability-model.md
//!
//! Security semantics:
//!     grammar/spec/security.md
//!
//! Security grammar:
//!     grammar/security/
//!
//! Universal policy grammar:
//!     grammar/policies/policy.g4
//!
//! Canonical parser composition:
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniLexer.g4
//!     grammar/antlr/ZamaniParser.g4
//!
//! Grammar inventory:
//!     grammar/MANIFEST.yaml
//!
//! ## Canonical implementation authorities
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain results:
//!     crate::toolchain::result
//!
//! Typed identities:
//!     crate::toolchain::ids
//!
//! Contract versions:
//!     crate::toolchain::version
//!
//! Conformance:
//!     crate::toolchain::testing::conformance
//!
//! Security validation:
//!     crate::toolchain::security::validation
//!
//! ## Regression rules
//!
//! - A failed operation must remain failed.
//! - Error classification and causal sources must be preserved.
//! - Cancellation and resource exhaustion must remain distinguishable.
//! - Typed identities must reject noncanonical representations.
//! - Contract versions must reject malformed representations.
//! - Missing conformance evidence must never pass.
//! - Evidence from an unrelated authority must never satisfy a requirement.
//! - Duplicate requirements and evidence must be rejected.
//! - An empty security request must never authorize an operation.
//! - Required unresolved security checks must block acceptance.
//! - An explicit security rejection must block acceptance.
//! - An extension error must never become verified evidence.
//! - Advisory uncertainty must not silently invalidate otherwise verified
//!   mandatory checks.
//!
//! ## Scalability
//!
//! This module imposes no universal limits on projects, packages,
//! dependencies, targets, providers, security checks, computing domains,
//! hardware resources, or distributed participants.
//!
//! Tests use small deterministic inputs because they are unit tests.
//! They do not establish a maximum supported workload.
//!
//! Dedicated stress and scalability tests belong in the toolchain
//! scalability test suite.
//!
//! ## Isolation
//!
//! Synthetic providers are local to this module.
//! They have no filesystem, network, process, credential, or hardware
//! side effects.
//!
//! No global mutable state, worker pool, scheduler, or background thread
//! is created by this module.

// ============================================================================
// CANONICAL TOOLCHAIN CONTRACT REGRESSIONS
// ============================================================================

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io;

    use crate::toolchain::errors::{
        ToolchainError,
        ToolchainErrorKind,
    };

    use crate::toolchain::result::{
        Result as ToolchainResultAlias,
        ToolchainErrorExt,
    };

    use crate::toolchain::ids::{
        IdAllocationError,
        IdAllocator,
        IdNamespace,
        PackageId,
        TaskId,
    };

    use crate::toolchain::version::ToolchainApiVersion;

    use crate::toolchain::testing::conformance::{
        ConformanceAuthority,
        ConformanceCheckId,
        ConformanceError,
        ConformanceEvidence,
        ConformanceRequirement,
        ConformanceStatus,
        ConformanceSuite,
        EvidenceOutcome,
        RequirementLevel,
    };

    use crate::toolchain::security::validation::{
        CheckRequirement,
        ExtensionCheck,
        ExtensionEvidence,
        SecurityCheckId,
        SecurityValidationExtension,
        SecurityValidationReport,
        SecurityValidationRequest,
        SecurityValidator,
        ValidationStatus,
    };

    // ========================================================================
    // ERROR CONTRACT REGRESSIONS
    // ========================================================================

    #[test]
    fn stable_error_codes_remain_distinct() {
        let invalid_input = ToolchainError::invalid_input(
            "invalid regression input",
        );

        let resource_exhausted = ToolchainError::resource_exhausted(
            "configured operational budget exhausted",
        );

        let cancelled = ToolchainError::cancelled(
            "operation cancelled",
        );

        assert_eq!(invalid_input.code(), "ZT0001");
        assert_eq!(resource_exhausted.code(), "ZT0016");
        assert_eq!(cancelled.code(), "ZT0017");

        assert_ne!(invalid_input.code(), resource_exhausted.code());
        assert_ne!(invalid_input.code(), cancelled.code());
        assert_ne!(resource_exhausted.code(), cancelled.code());

        assert_eq!(
            invalid_input.kind(),
            ToolchainErrorKind::InvalidInput,
        );

        assert_eq!(
            resource_exhausted.kind(),
            ToolchainErrorKind::ResourceExhausted,
        );

        assert_eq!(
            cancelled.kind(),
            ToolchainErrorKind::Cancelled,
        );
    }

    #[test]
    fn contextual_enrichment_never_converts_failure_to_success() {
        let original: ToolchainResultAlias<()> = Err(
            ToolchainError::build_failed(
                "synthetic compiler failure",
            ),
        );

        let error = original
            .with_toolchain_operation("regression.build")
            .with_toolchain_subject("project:regression")
            .with_toolchain_context("build execution")
            .expect_err("a failed build must remain an error");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::BuildFailed,
        );

        assert_eq!(
            error.operation(),
            Some("regression.build"),
        );

        assert_eq!(
            error.subject(),
            Some("project:regression"),
        );

        assert_eq!(
            error.context(),
            &["build execution".to_owned()],
        );
    }

    #[test]
    fn successful_results_remain_unchanged_by_error_enrichment() {
        let original: ToolchainResultAlias<u64> = Ok(73);

        let result = original
            .with_toolchain_operation("regression.success")
            .with_toolchain_subject("task:regression")
            .with_toolchain_context("successful operation");

        assert_eq!(
            result.expect("successful result must remain successful"),
            73,
        );
    }

    #[test]
    fn causal_io_errors_survive_toolchain_context_enrichment() {
        let source = io::Error::new(
            io::ErrorKind::NotFound,
            "regression source file is missing",
        );

        let original = ToolchainError::from(source);

        assert_eq!(original.kind(), ToolchainErrorKind::Io);
        assert!(original.has_source());

        let result: ToolchainResultAlias<()> = Err(original);

        let error = result
            .with_toolchain_operation("regression.manifest.load")
            .with_toolchain_context("workspace discovery")
            .expect_err("the original I/O failure must be retained");

        assert_eq!(error.kind(), ToolchainErrorKind::Io);
        assert!(error.has_source());

        let cause = Error::source(&error)
            .expect("the original causal error must remain available");

        assert!(
            cause
                .to_string()
                .contains("regression source file is missing"),
        );
    }

    // ========================================================================
    // TYPED IDENTITY REGRESSIONS
    // ========================================================================

    #[test]
    fn typed_ids_round_trip_without_losing_identity() {
        let task = TaskId::parse("regression:17")
            .expect("canonical task identity must parse");

        assert_eq!(task.to_string(), "regression:17");
        assert_eq!(task.namespace().as_str(), "regression");
        assert_eq!(task.value(), 17);

        let package = PackageId::parse("regression:29")
            .expect("canonical package identity must parse");

        assert_eq!(package.to_string(), "regression:29");
        assert_eq!(package.value(), 29);

        // Different typed categories must remain different Rust types.
        // Their textual representations do not erase category ownership.
        assert_eq!(task.to_string(), "regression:17");
        assert_eq!(package.to_string(), "regression:29");
    }

    #[test]
    fn typed_ids_reject_noncanonical_values() {
        assert!(TaskId::parse("regression:0").is_err());
        assert!(TaskId::parse("regression:017").is_err());
        assert!(TaskId::parse("Regression:17").is_err());
        assert!(TaskId::parse("regression:-17").is_err());
        assert!(TaskId::parse("regression:17:extra").is_err());
    }

    #[test]
    fn typed_id_allocator_does_not_wrap_or_reuse_values() {
        let namespace = IdNamespace::new("regression")
            .expect("test namespace must be valid");

        let allocator = IdAllocator::<TaskId>::with_next(
            namespace,
            u128::MAX,
        )
        .expect("maximum nonzero identity is a valid checkpoint");

        let final_identity = allocator
            .try_next()
            .expect("maximum nonzero identity must be issuable");

        assert_eq!(final_identity.value(), u128::MAX);

        assert_eq!(
            allocator.try_next().expect_err(
                "allocator must not wrap or reuse an identity",
            ),
            IdAllocationError::Exhausted,
        );

        assert!(
            allocator
                .is_exhausted()
                .expect("allocator state must be readable"),
        );
    }

    // ========================================================================
    // VERSION CONTRACT REGRESSIONS
    // ========================================================================

    #[test]
    fn toolchain_contract_versions_round_trip_canonically() {
        let version: ToolchainApiVersion = "1.2.3"
            .parse()
            .expect("canonical version must parse");

        assert_eq!(version.to_string(), "1.2.3");
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn toolchain_contract_versions_reject_noncanonical_text() {
        assert!("".parse::<ToolchainApiVersion>().is_err());
        assert!("1".parse::<ToolchainApiVersion>().is_err());
        assert!("1.2".parse::<ToolchainApiVersion>().is_err());
        assert!("01.2.3".parse::<ToolchainApiVersion>().is_err());
        assert!("1.2.3.4".parse::<ToolchainApiVersion>().is_err());
        assert!("1.2.3-alpha".parse::<ToolchainApiVersion>().is_err());
        assert!(" 1.2.3".parse::<ToolchainApiVersion>().is_err());
    }

    // ========================================================================
    // CONFORMANCE FRAMEWORK REGRESSIONS
    // ========================================================================

    fn requirement(
        id: &str,
    ) -> (
        ConformanceCheckId,
        ConformanceAuthority,
        ConformanceRequirement,
    ) {
        let check_id = ConformanceCheckId::new(id)
            .expect("test check identity must be valid");

        let authority = ConformanceAuthority::new(
            "grammar/specification/poco-reaf.md",
        )
        .expect("test authority must be valid");

        let requirement = ConformanceRequirement::new(
            check_id.clone(),
            authority.clone(),
            "POCO-REAF semantic preservation",
            "Compatible target realizations preserve the specified program meaning.",
            RequirementLevel::Mandatory,
        )
        .expect("test requirement must be valid");

        (check_id, authority, requirement)
    }

    fn evidence(
        check_id: ConformanceCheckId,
        authority: ConformanceAuthority,
        outcome: EvidenceOutcome,
    ) -> ConformanceEvidence {
        ConformanceEvidence::new(
            check_id,
            authority,
            outcome,
            "regression://toolchain/poco-reaf",
            "deterministic regression test",
            "regression-contract:1",
            "The regression test produced this explicit evidence outcome.",
        )
        .expect("test evidence must be valid")
    }

    #[test]
    fn missing_mandatory_conformance_evidence_is_inconclusive() {
        let (_, _, requirement) = requirement(
            "portability.semantic-preservation",
        );

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("unique requirement must register");

        let report = suite
            .evaluate(std::iter::empty())
            .expect("missing evidence must produce a report");

        assert_eq!(report.total(), 1);
        assert_eq!(report.status(), ConformanceStatus::Inconclusive);
        assert!(!report.is_conformant());
        assert!(report.is_inconclusive());
        assert_eq!(report.mandatory_blocked(), 1);

        report
            .validate()
            .expect("generated report must satisfy its invariants");
    }

    #[test]
    fn affirmative_authoritative_evidence_can_satisfy_conformance() {
        let (check_id, authority, requirement) = requirement(
            "portability.semantic-preservation",
        );

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("unique requirement must register");

        let report = suite
            .evaluate([evidence(
                check_id,
                authority,
                EvidenceOutcome::Satisfied,
            )])
            .expect("valid evidence must be evaluated");

        assert_eq!(report.status(), ConformanceStatus::Conformant);
        assert!(report.is_conformant());
        assert_eq!(report.mandatory_passed(), 1);
        assert_eq!(report.mandatory_failed(), 0);
        assert_eq!(report.mandatory_blocked(), 0);

        report
            .validate()
            .expect("generated report must satisfy its invariants");
    }

    #[test]
    fn violating_conformance_evidence_is_never_a_pass() {
        let (check_id, authority, requirement) = requirement(
            "security.fail-closed",
        );

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("unique requirement must register");

        let report = suite
            .evaluate([evidence(
                check_id,
                authority,
                EvidenceOutcome::Violated,
            )])
            .expect("violating evidence must be evaluated");

        assert_eq!(
            report.status(),
            ConformanceStatus::NonConformant,
        );

        assert!(!report.is_conformant());
        assert_eq!(report.mandatory_failed(), 1);

        report
            .validate()
            .expect("generated report must satisfy its invariants");
    }

    #[test]
    fn conformance_evidence_from_another_authority_is_rejected() {
        let (check_id, authority, requirement) = requirement(
            "portability.semantic-preservation",
        );

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("unique requirement must register");

        let unrelated_authority = ConformanceAuthority::new(
            "unrelated.authority",
        )
        .expect("test authority must be valid");

        let result = suite.evaluate([evidence(
            check_id,
            unrelated_authority,
            EvidenceOutcome::Satisfied,
        )]);

        assert!(matches!(
            result,
            Err(ConformanceError::AuthorityMismatch { .. }),
        ));

        // The correct authority is intentionally not substituted.
        // A mismatched evidence record must not become a passing result.
        let _ = authority;
    }

    #[test]
    fn duplicate_conformance_requirements_are_rejected() {
        let (_, _, first) = requirement("security.integrity");
        let (_, _, duplicate) = requirement("security.integrity");

        let mut suite = ConformanceSuite::new();

        suite
            .register(first)
            .expect("first requirement must register");

        assert!(suite.register(duplicate).is_err());
        assert_eq!(suite.len(), 1);
    }

    #[test]
    fn duplicate_conformance_evidence_is_rejected() {
        let (check_id, authority, requirement) = requirement(
            "security.integrity",
        );

        let mut suite = ConformanceSuite::new();

        suite
            .register(requirement)
            .expect("requirement must register");

        let first = evidence(
            check_id.clone(),
            authority.clone(),
            EvidenceOutcome::Satisfied,
        );

        let second = evidence(
            check_id,
            authority,
            EvidenceOutcome::Satisfied,
        );

        assert!(
            suite.evaluate([first, second]).is_err(),
            "duplicate evidence must not be silently selected",
        );
    }

    // ========================================================================
    // SECURITY VALIDATION REGRESSIONS
    // ========================================================================

    /// A deterministic synthetic security authority for regression tests.
    ///
    /// It does not implement or simulate cryptographic verification.
    /// It returns only the explicit outcome configured by the test.
    struct FixedSecurityExtension {
        id: &'static str,
        status: ValidationStatus,
        fail: bool,
    }

    impl SecurityValidationExtension for FixedSecurityExtension {
        fn id(&self) -> &str {
            self.id
        }

        fn validate(
            &self,
            _operation: &str,
        ) -> crate::toolchain::errors::ToolchainResult<ExtensionEvidence> {
            if self.fail {
                return Err(ToolchainError::provider_failed(
                    "synthetic regression provider failure",
                ));
            }

            ExtensionEvidence::new(
                self.id,
                format!("regression://security/{}", self.id),
                self.status,
                "synthetic regression evidence",
            )
        }
    }

    fn validate_one_extension(
        extension: &FixedSecurityExtension,
        requirement: CheckRequirement,
    ) -> crate::toolchain::errors::ToolchainResult<SecurityValidationReport> {
        let check_id = SecurityCheckId::new(
            format!("regression.security.{}", extension.id),
        )?;

        let request = SecurityValidationRequest::new(
            "regression.security.operation",
        )?
        .with_extension(ExtensionCheck::new(
            check_id,
            requirement,
            extension,
        ));

        SecurityValidator::new().validate(request)
    }

    #[test]
    fn empty_security_validation_never_authorizes() {
        let request = SecurityValidationRequest::new(
            "regression.package.publish",
        )
        .expect("operation identity must be valid");

        assert!(request.is_empty());

        let report = SecurityValidator::new()
            .validate(request)
            .expect("empty validation should produce a report");

        assert_eq!(report.status(), ValidationStatus::NotEvaluated);
        assert!(!report.can_proceed());
        assert_eq!(report.check_count(), 0);
    }

    #[test]
    fn required_verified_security_evidence_allows_validation_to_proceed() {
        let extension = FixedSecurityExtension {
            id: "regression.integrity.authority",
            status: ValidationStatus::Verified,
            fail: false,
        };

        let report = validate_one_extension(
            &extension,
            CheckRequirement::Required,
        )
        .expect("synthetic verified evidence must be evaluated");

        assert_eq!(report.status(), ValidationStatus::Verified);
        assert!(report.required_checks_verified());
        assert!(report.can_proceed());
        assert_eq!(report.verified_count(), 1);
    }

    #[test]
    fn required_indeterminate_security_evidence_blocks_acceptance() {
        let extension = FixedSecurityExtension {
            id: "regression.trust.authority",
            status: ValidationStatus::Indeterminate,
            fail: false,
        };

        let report = validate_one_extension(
            &extension,
            CheckRequirement::Required,
        )
        .expect("indeterminate evidence must produce a report");

        assert_eq!(
            report.status(),
            ValidationStatus::Indeterminate,
        );

        assert!(!report.required_checks_verified());
        assert!(!report.can_proceed());
        assert!(report.has_unresolved_checks());
    }

    #[test]
    fn required_not_evaluated_security_evidence_blocks_acceptance() {
        let extension = FixedSecurityExtension {
            id: "regression.authorization.authority",
            status: ValidationStatus::NotEvaluated,
            fail: false,
        };

        let report = validate_one_extension(
            &extension,
            CheckRequirement::Required,
        )
        .expect("not-evaluated evidence must produce a report");

        assert_eq!(
            report.status(),
            ValidationStatus::NotEvaluated,
        );

        assert!(!report.can_proceed());
        assert!(report.has_unresolved_checks());
    }

    #[test]
    fn explicit_rejection_blocks_even_an_advisory_check() {
        let extension = FixedSecurityExtension {
            id: "regression.rejected.authority",
            status: ValidationStatus::Rejected,
            fail: false,
        };

        let report = validate_one_extension(
            &extension,
            CheckRequirement::Advisory,
        )
        .expect("rejected evidence must produce a report");

        assert_eq!(report.status(), ValidationStatus::Rejected);
        assert_eq!(report.rejected_count(), 1);
        assert!(!report.can_proceed());
    }

    #[test]
    fn advisory_uncertainty_does_not_hide_required_success() {
        let required = FixedSecurityExtension {
            id: "regression.required.authority",
            status: ValidationStatus::Verified,
            fail: false,
        };

        let advisory = FixedSecurityExtension {
            id: "regression.advisory.authority",
            status: ValidationStatus::Indeterminate,
            fail: false,
        };

        let request = SecurityValidationRequest::new(
            "regression.security.operation",
        )
        .expect("operation identity must be valid")
        .with_extension(ExtensionCheck::new(
            SecurityCheckId::new("regression.required.check")
                .expect("check identity must be valid"),
            CheckRequirement::Required,
            &required,
        ))
        .with_extension(ExtensionCheck::new(
            SecurityCheckId::new("regression.advisory.check")
                .expect("check identity must be valid"),
            CheckRequirement::Advisory,
            &advisory,
        ));

        let report = SecurityValidator::new()
            .validate(request)
            .expect("both checks must be evaluated");

        assert_eq!(report.status(), ValidationStatus::Verified);
        assert!(report.required_checks_verified());
        assert!(report.can_proceed());

        // The unresolved advisory result remains visible. It is not
        // discarded merely because mandatory checks passed.
        assert!(report.has_unresolved_checks());
        assert_eq!(report.unresolved_count(), 1);
    }

    #[test]
    fn security_provider_failure_remains_a_failure() {
        let extension = FixedSecurityExtension {
            id: "regression.failing.authority",
            status: ValidationStatus::Verified,
            fail: true,
        };

        let error = validate_one_extension(
            &extension,
            CheckRequirement::Required,
        )
        .expect_err("provider failure must not become verified evidence");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ValidationFailed,
        );

        assert!(error.has_source());
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn security_extension_identity_must_be_valid() {
        let extension = FixedSecurityExtension {
            id: "invalid\nsecurity.authority",
            status: ValidationStatus::Verified,
            fail: false,
        };

        let result = validate_one_extension(
            &extension,
            CheckRequirement::Required,
        );

        assert!(
            result.is_err(),
            "control characters must not be accepted in authority identities",
        );
    }
}
