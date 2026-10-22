
```rust
//! # Zamani Hybrid Computing — Canonical Errors
//!
//! Path: `src/hybrid/errors.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only. Unsafe code is forbidden.
//!
//! ## Purpose
//!
//! Define the canonical, structured error vocabulary for the hybrid
//! composition subsystem.
//!
//! Hybrid errors describe failures in representing, validating, composing,
//! exchanging data between, or preparing computations across domains.
//! They do not redefine the semantics of the source language, classical
//! computation, quantum computation, or any other participating domain.
//!
//! ## Ownership
//!
//! This module owns:
//! - Stable, machine-readable hybrid error categories.
//! - Structured error details and contextual annotations.
//! - Human-readable error formatting.
//! - Constructors for common hybrid failures.
//! - Error classification helpers.
//!
//! This module does not own:
//! - Source-language diagnostics or source maps.
//! - AST, semantic-model, classical IR, quantum IR, or ZUIR definitions.
//! - Domain-specific operation semantics.
//! - Hardware discovery, physical allocation, routing, scheduling, or QEC.
//! - Runtime execution or vendor-specific error types.
//! - Serialization compatibility or schema migration.
//!
//! ## Dependency contract
//!
//! This file is intentionally independent of other Zamani modules.
//! Other modules may depend on this file; this file must not depend on them.
//!
//! When IDs, source spans, resource policies, and canonical diagnostic
//! types are integrated, adapters may add typed references without making
//! this foundational module depend on higher-level subsystems.
//!
//! ## Scalability contract
//!
//! This module imposes no arbitrary limits on the number of domains,
//! graph nodes, edges, values, resources, or operations.
//!
//! Error context uses growable collections. Allocation can still fail under
//! operating-system or allocator constraints; Rust's standard allocation
//! behavior is not an unlimited-memory guarantee.
//!
//! Operational budgets belong to explicit configuration and validation
//! policies, not to hidden limits in this error vocabulary.
//!
//! ## Compatibility contract
//!
//! `HybridErrorCode::as_str()` provides stable machine-readable identifiers.
//! Human-readable messages are explanatory and are not a serialization
//! protocol. Callers must not parse `Display` output to identify an error.
//!
//! Adding a new enum variant can affect exhaustive downstream matches.
//! Consumers that need forward-compatible matching should match error codes
//! or use a wildcard arm.
//!
//! ## Completion criteria
//!
//! - The module compiles independently.
//! - Every error has a stable code and structured category.
//! - Context can be added without replacing the original failure.
//! - Display output is deterministic for the same error value.
//! - No unsafe code or hardware-size assumptions are introduced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::error::Error;

/// Stable machine-readable categories for hybrid subsystem failures.
///
/// These identifiers are intended for diagnostics, logging, tests, API
/// consumers, and future integrations. They are not source-language tokens.
///
/// Keep existing string identifiers stable after public release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum HybridErrorCode {
    /// The supplied hybrid representation is structurally invalid.
    InvalidRepresentation,

    /// A required domain or domain instance is unknown.
    UnknownDomain,

    /// Two declarations conflict or define the same identity improperly.
    DuplicateIdentity,

    /// A reference points to an entity that does not exist.
    InvalidReference,

    /// A graph edge or dependency is structurally invalid.
    InvalidDependency,

    /// A dependency relationship violates declared execution semantics.
    DependencyViolation,

    /// A cycle violates a specific construct's declared semantics.
    InvalidCycle,

    /// A domain interface or contract is malformed.
    InvalidContract,

    /// A domain interface does not satisfy a required contract.
    ContractMismatch,

    /// The requested interface or operation is not supported.
    UnsupportedOperation,

    /// A required computation domain or feature is unavailable.
    UnsupportedDomain,

    /// A value cannot cross a domain boundary under the declared contract.
    DataExchangeFailure,

    /// A value has an incompatible type or representation.
    ValueIncompatibility,

    /// An ownership, borrowing, or lifetime contract is violated.
    OwnershipViolation,

    /// A required dependency or result is not available.
    DependencyNotReady,

    /// A synchronization contract cannot be satisfied.
    SynchronizationFailure,

    /// An operation did not reach the required completion state.
    CompletionFailure,

    /// A cancellation request or cancellation contract failed.
    CancellationFailure,

    /// A required capability is not provided by the described target.
    MissingCapability,

    /// A resource requirement cannot be satisfied.
    ResourceInfeasible,

    /// Available information is insufficient to establish feasibility.
    FeasibilityUnknown,

    /// A mandatory constraint cannot be satisfied.
    ConstraintViolation,

    /// A supplied policy or configuration is invalid.
    InvalidConfiguration,

    /// An explicit operational budget has been exhausted.
    BudgetExhausted,

    /// A requested version is not supported.
    UnsupportedVersion,

    /// A versioned representation requires an unavailable migration.
    MigrationRequired,

    /// The representation is malformed or inconsistent.
    MalformedInput,

    /// Encoding a representation failed.
    EncodingFailure,

    /// Decoding a representation failed.
    DecodingFailure,

    /// A deterministic operation received inconsistent inputs or policy.
    DeterminismViolation,

    /// A proposed realization does not preserve required semantics.
    SemanticPreservationFailure,

    /// An integration adapter could not map a canonical representation.
    IntegrationFailure,

    /// A compiler-stage operation failed.
    CompilationFailure,

    /// A runtime-stage operation failed.
    RuntimeFailure,

    /// An external domain implementation reported a failure.
    DomainFailure,

    /// An internal invariant was violated.
    InternalInvariantViolation,
}

impl HybridErrorCode {
    /// Returns the stable machine-readable identifier for this category.
    ///
    /// These strings are part of the public diagnostic contract.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRepresentation => "HYB001",
            Self::UnknownDomain => "HYB002",
            Self::DuplicateIdentity => "HYB003",
            Self::InvalidReference => "HYB004",
            Self::InvalidDependency => "HYB005",
            Self::DependencyViolation => "HYB006",
            Self::InvalidCycle => "HYB007",
            Self::InvalidContract => "HYB008",
            Self::ContractMismatch => "HYB009",
            Self::UnsupportedOperation => "HYB010",
            Self::UnsupportedDomain => "HYB011",
            Self::DataExchangeFailure => "HYB012",
            Self::ValueIncompatibility => "HYB013",
            Self::OwnershipViolation => "HYB014",
            Self::DependencyNotReady => "HYB015",
            Self::SynchronizationFailure => "HYB016",
            Self::CompletionFailure => "HYB017",
            Self::CancellationFailure => "HYB018",
            Self::MissingCapability => "HYB019",
            Self::ResourceInfeasible => "HYB020",
            Self::FeasibilityUnknown => "HYB021",
            Self::ConstraintViolation => "HYB022",
            Self::InvalidConfiguration => "HYB023",
            Self::BudgetExhausted => "HYB024",
            Self::UnsupportedVersion => "HYB025",
            Self::MigrationRequired => "HYB026",
            Self::MalformedInput => "HYB027",
            Self::EncodingFailure => "HYB028",
            Self::DecodingFailure => "HYB029",
            Self::DeterminismViolation => "HYB030",
            Self::SemanticPreservationFailure => "HYB031",
            Self::IntegrationFailure => "HYB032",
            Self::CompilationFailure => "HYB033",
            Self::RuntimeFailure => "HYB034",
            Self::DomainFailure => "HYB035",
            Self::InternalInvariantViolation => "HYB036",
        }
    }

    /// Returns whether the failure indicates that an operation cannot
    /// safely continue under the same assumptions.
    ///
    /// This is a classification aid, not a recovery decision.
    #[must_use]
    pub const fn requires_correction_or_policy_change(self) -> bool {
        matches!(
            self,
            Self::InvalidRepresentation
                | Self::InvalidDependency
                | Self::DependencyViolation
                | Self::InvalidCycle
                | Self::InvalidContract
                | Self::ContractMismatch
                | Self::OwnershipViolation
                | Self::MissingCapability
                | Self::ResourceInfeasible
                | Self::ConstraintViolation
                | Self::InvalidConfiguration
                | Self::BudgetExhausted
                | Self::UnsupportedVersion
                | Self::MigrationRequired
                | Self::MalformedInput
                | Self::DeterminismViolation
                | Self::SemanticPreservationFailure
                | Self::InternalInvariantViolation
        )
    }
}

impl fmt::Display for HybridErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Context attached to a hybrid error.
///
/// Context explains where a failure occurred without replacing its original
/// category or message. The fields deliberately use strings at this
/// foundational boundary so that the error module does not depend on
/// unfinished domain IDs, source maps, or graph types.
///
/// Higher-level APIs should populate these values from canonical typed
/// entities rather than treating arbitrary strings as authoritative IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HybridErrorContext {
    /// Name of the subsystem or operation that added this context.
    component: String,

    /// Optional entity identifier or descriptive reference.
    entity: Option<String>,

    /// Additional explanation specific to this context.
    detail: Option<String>,
}

impl HybridErrorContext {
    /// Creates a context with a component name.
    #[must_use]
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            entity: None,
            detail: None,
        }
    }

    /// Associates this context with an entity or reference.
    #[must_use]
    pub fn with_entity(mut self, entity: impl Into<String>) -> Self {
        self.entity = Some(entity.into());
        self
    }

    /// Adds explanatory detail.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Returns the component that added this context.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Returns the optional entity reference.
    #[must_use]
    pub fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    /// Returns the optional detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

impl fmt::Display for HybridErrorContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.component)?;

        if let Some(entity) = &self.entity {
            write!(formatter, " [entity: {entity}]")?;
        }

        if let Some(detail) = &self.detail {
            write!(formatter, ": {detail}")?;
        }

        Ok(())
    }
}

/// Canonical structured error for the hybrid subsystem.
///
/// `HybridError` preserves the original category and message while allowing
/// callers to add context as an error travels through graph validation,
/// domain adapters, compilation, serialization, or runtime integration.
///
/// The error is intentionally not `Clone`: callers should not assume that
/// all future wrapped causes or external errors can be duplicated cheaply.
#[derive(Debug, PartialEq, Eq)]
pub struct HybridError {
    code: HybridErrorCode,
    message: String,
    context: Vec<HybridErrorContext>,
}

impl HybridError {
    /// Creates a hybrid error from a stable category and an explanatory
    /// message.
    ///
    /// Prefer the specialized constructors below when a category has a
    /// well-defined meaning.
    #[must_use]
    pub fn new(code: HybridErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: Vec::new(),
        }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn code(&self) -> HybridErrorCode {
        self.code
    }

    /// Returns the stable machine-readable code string.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the primary explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns contextual annotations in the order they were added.
    #[must_use]
    pub fn context(&self) -> &[HybridErrorContext] {
        &self.context
    }

    /// Adds context without replacing the original error.
    ///
    /// Context order is deterministic: the first item is the earliest
    /// annotation, and the final item is the most recently added annotation.
    #[must_use]
    pub fn with_context(mut self, context: HybridErrorContext) -> Self {
        self.context.push(context);
        self
    }

    /// Adds a simple component annotation.
    #[must_use]
    pub fn in_component(self, component: impl Into<String>) -> Self {
        self.with_context(HybridErrorContext::new(component))
    }

    /// Adds an entity reference and explanatory detail.
    #[must_use]
    pub fn at_entity(
        self,
        component: impl Into<String>,
        entity: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        self.with_context(
            HybridErrorContext::new(component)
                .with_entity(entity)
                .with_detail(detail),
        )
    }

    /// Creates an invalid-representation error.
    #[must_use]
    pub fn invalid_representation(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::InvalidRepresentation, message)
    }

    /// Creates an unknown-domain error.
    #[must_use]
    pub fn unknown_domain(domain: impl Into<String>) -> Self {
        let domain = domain.into();
        Self::new(
            HybridErrorCode::UnknownDomain,
            format!("unknown computation domain `{domain}`"),
        )
    }

    /// Creates an invalid-reference error.
    #[must_use]
    pub fn invalid_reference(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::InvalidReference, message)
    }

    /// Creates an invalid-dependency error.
    #[must_use]
    pub fn invalid_dependency(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::InvalidDependency, message)
    }

    /// Creates a contract-mismatch error.
    #[must_use]
    pub fn contract_mismatch(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::ContractMismatch, message)
    }

    /// Creates a missing-capability error.
    #[must_use]
    pub fn missing_capability(capability: impl Into<String>) -> Self {
        let capability = capability.into();
        Self::new(
            HybridErrorCode::MissingCapability,
            format!("required capability `{capability}` is unavailable"),
        )
    }

    /// Creates a resource-infeasibility error.
    #[must_use]
    pub fn resource_infeasible(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::ResourceInfeasible, message)
    }

    /// Creates a budget-exhaustion error.
    ///
    /// `budget_name` identifies the explicitly configured operational
    /// budget. This constructor does not establish a default or universal
    /// limit.
    #[must_use]
    pub fn budget_exhausted(
        budget_name: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let budget_name = budget_name.into();
        let detail = detail.into();

        Self::new(
            HybridErrorCode::BudgetExhausted,
            format!("operational budget `{budget_name}` exhausted: {detail}"),
        )
    }

    /// Creates an unsupported-version error.
    #[must_use]
    pub fn unsupported_version(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::UnsupportedVersion, message)
    }

    /// Creates a malformed-input error.
    #[must_use]
    pub fn malformed_input(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::MalformedInput, message)
    }

    /// Creates a serialization-encoding error.
    #[must_use]
    pub fn encoding_failure(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::EncodingFailure, message)
    }

    /// Creates a serialization-decoding error.
    #[must_use]
    pub fn decoding_failure(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::DecodingFailure, message)
    }

    /// Creates a semantic-preservation error.
    #[must_use]
    pub fn semantic_preservation_failure(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::SemanticPreservationFailure, message)
    }

    /// Creates an integration error.
    #[must_use]
    pub fn integration_failure(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::IntegrationFailure, message)
    }

    /// Creates an internal-invariant error.
    ///
    /// Use only when an internal invariant is actually violated. Do not use
    /// this for ordinary invalid user input or unsupported target capability.
    #[must_use]
    pub fn internal_invariant_violation(message: impl Into<String>) -> Self {
        Self::new(HybridErrorCode::InternalInvariantViolation, message)
    }
}

impl fmt::Display for HybridError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)?;

        for context in &self.context {
            write!(formatter, "\n  at {context}")?;
        }

        Ok(())
    }
}

impl Error for HybridError {}

/// Convenient result alias for hybrid subsystem operations.
///
/// Use `Result<T, HybridError>` explicitly at public API boundaries if
/// an alias would obscure the return type for downstream documentation.
pub type HybridResult<T> = Result<T, HybridError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_and_unique() {
        let codes = [
            HybridErrorCode::InvalidRepresentation,
            HybridErrorCode::UnknownDomain,
            HybridErrorCode::DuplicateIdentity,
            HybridErrorCode::InvalidReference,
            HybridErrorCode::InvalidDependency,
            HybridErrorCode::DependencyViolation,
            HybridErrorCode::InvalidCycle,
            HybridErrorCode::InvalidContract,
            HybridErrorCode::ContractMismatch,
            HybridErrorCode::UnsupportedOperation,
            HybridErrorCode::UnsupportedDomain,
            HybridErrorCode::DataExchangeFailure,
            HybridErrorCode::ValueIncompatibility,
            HybridErrorCode::OwnershipViolation,
            HybridErrorCode::DependencyNotReady,
            HybridErrorCode::SynchronizationFailure,
            HybridErrorCode::CompletionFailure,
            HybridErrorCode::CancellationFailure,
            HybridErrorCode::MissingCapability,
            HybridErrorCode::ResourceInfeasible,
            HybridErrorCode::FeasibilityUnknown,
            HybridErrorCode::ConstraintViolation,
            HybridErrorCode::InvalidConfiguration,
            HybridErrorCode::BudgetExhausted,
            HybridErrorCode::UnsupportedVersion,
            HybridErrorCode::MigrationRequired,
            HybridErrorCode::MalformedInput,
            HybridErrorCode::EncodingFailure,
            HybridErrorCode::DecodingFailure,
            HybridErrorCode::DeterminismViolation,
            HybridErrorCode::SemanticPreservationFailure,
            HybridErrorCode::IntegrationFailure,
            HybridErrorCode::CompilationFailure,
            HybridErrorCode::RuntimeFailure,
            HybridErrorCode::DomainFailure,
            HybridErrorCode::InternalInvariantViolation,
        ];

        for (index, code) in codes.iter().enumerate() {
            assert!(!code.as_str().is_empty());

            for other in codes.iter().skip(index + 1) {
                assert_ne!(
                    code.as_str(),
                    other.as_str(),
                    "duplicate error code identifier"
                );
            }
        }

        assert_eq!(
            HybridErrorCode::InvalidRepresentation.as_str(),
            "HYB001"
        );
        assert_eq!(
            HybridErrorCode::InternalInvariantViolation.as_str(),
            "HYB036"
        );
    }

    #[test]
    fn error_preserves_its_original_category_when_context_is_added() {
        let error = HybridError::invalid_dependency("dependency endpoint is missing")
            .in_component("graph verification")
            .at_entity(
                "hybrid graph",
                "edge-17",
                "the destination node does not exist",
            );

        assert_eq!(error.code(), HybridErrorCode::InvalidDependency);
        assert_eq!(error.code_str(), "HYB005");
        assert_eq!(error.context().len(), 2);
        assert_eq!(error.context()[0].component(), "graph verification");
        assert_eq!(error.context()[1].entity(), Some("edge-17"));
    }

    #[test]
    fn display_output_is_deterministic() {
        let error = HybridError::missing_capability("quantum.measurement")
            .in_component("feasibility analysis");

        let first = error.to_string();
        let second = error.to_string();

        assert_eq!(first, second);
        assert!(first.contains("HYB019"));
        assert!(first.contains("quantum.measurement"));
        assert!(first.contains("feasibility analysis"));
    }

    #[test]
    fn budget_errors_require_an_explicit_budget_name() {
        let error = HybridError::budget_exhausted(
            "configured graph-verification work budget",
            "the requested verification work could not be completed",
        );

        assert_eq!(error.code(), HybridErrorCode::BudgetExhausted);
        assert!(error.message().contains("configured graph-verification"));
        assert!(error.message().contains("could not be completed"));
    }

    #[test]
    fn error_context_does_not_change_the_error_code() {
        let original = HybridError::resource_infeasible(
            "the described target cannot satisfy the resource requirement",
        );

        let enriched = original
            .in_component("compiler planning")
            .in_component("target feasibility");

        assert_eq!(enriched.code(), HybridErrorCode::ResourceInfeasible);
        assert_eq!(enriched.context().len(), 2);
    }

    #[test]
    fn source_language_and_hardware_limits_are_not_encoded_here() {
        // A budget-exhaustion error is tied to a named operational policy.
        // This module defines no universal domain, node, device, or graph cap.
        let error = HybridError::budget_exhausted(
            "caller-supplied decode budget",
            "decode work stopped according to policy",
        );

        assert_eq!(error.code(), HybridErrorCode::BudgetExhausted);
    }

    #[test]
    fn specialized_constructors_select_the_expected_categories() {
        assert_eq!(
            HybridError::unknown_domain("future.domain").code(),
            HybridErrorCode::UnknownDomain
        );
        assert_eq!(
            HybridError::invalid_reference("unknown endpoint").code(),
            HybridErrorCode::InvalidReference
        );
        assert_eq!(
            HybridError::contract_mismatch("incompatible interface").code(),
            HybridErrorCode::ContractMismatch
        );
        assert_eq!(
            HybridError::unsupported_version("unknown schema").code(),
            HybridErrorCode::UnsupportedVersion
        );
        assert_eq!(
            HybridError::decoding_failure("invalid encoding").code(),
            HybridErrorCode::DecodingFailure
        );
    }

    #[test]
    fn error_implements_standard_error_and_result_alias_is_usable() {
        fn returns_error() -> HybridResult<()> {
            Err(HybridError::invalid_representation("test failure"))
        }

        let error = returns_error().expect_err("the test must return its error");
        let standard_error: &dyn Error = &error;

        assert!(standard_error.to_string().contains("test failure"));
    }
}
```
