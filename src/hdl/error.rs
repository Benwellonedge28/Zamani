
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL subsystem errors
//!
//! File: `src/hdl/error.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the shared, typed error contract for `src/hdl/`.
//!
//! This module distinguishes invalid API inputs, invalid configuration,
//! unresolved prerequisites, unsupported functionality, operational budget
//! exhaustion, cancellation, target infeasibility, provider failures, and
//! internal contract violations.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/parser/error.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/compiler/diagnostics.rs`
//!
//! ## Owns
//!
//! - Stable HDL error classifications and machine-readable codes.
//! - Error descriptions and structured contextual information.
//! - Error chaining for underlying infrastructure or provider failures.
//! - The HDL subsystem's `Result` alias.
//!
//! ## Does not own
//!
//! - Canonical source spans, source maps, or AST identities.
//! - Parser-specific errors or parser recovery.
//! - User-facing compiler diagnostic rendering.
//! - HDL semantic diagnostic collection or ordering.
//! - Language type checking or semantic validation.
//! - Target discovery, physical resource allocation, or hardware execution.
//! - HDLIR, lowering, optimization, scheduling, placement, or routing.
//!
//! ## Integration contract
//!
//! Public HDL APIs should return `HdlResult<T>` when a failure represents an
//! inability to complete the requested HDL operation. Callers must preserve
//! the typed error and its source chain instead of reducing it to a `String`.
//!
//! Source-language errors that can be reported while analysis continues
//! should ordinarily remain structured diagnostics owned by the appropriate
//! frontend or validation subsystem. This error type does not replace those
//! diagnostics.
//!
//! Parser errors should be retained as their original error type or wrapped
//! with `HdlError::with_source` at a genuine subsystem boundary. This module
//! does not depend on parser implementation details.
//!
//! The diagnostics adapter may map `HdlErrorKind::stable_code()` and
//! contextual information into `crate::compiler::diagnostics::Diagnostic`.
//! It must use the repository's canonical `Span` when one is available;
//! this module deliberately defines no replacement span type.
//!
//! ## Scalability and resource safety
//!
//! No language-level hardware capacity limits are defined here.
//! `u128` quantities in budget errors are descriptive counters, not universal
//! limits on hardware dimensions or program sizes. Callers whose counters
//! exceed `u128` must report overflow or use a different accounting strategy
//! rather than silently saturating and claiming exact measurements.
//!
//! Operational budgets are policy decisions made by the consuming subsystem.
//! Exhausting a budget is not evidence that a source program is invalid or
//! that a target is incapable of realizing it.
//!
//! Error context is dynamically sized. Applications processing untrusted
//! input should apply their own explicit memory and diagnostic-retention
//! budgets. This module does not impose a hidden context-count limit.
//!
//! ## Determinism
//!
//! Error codes are stable identifiers. Display messages are intended for
//! humans and are not a serialization format. Context is rendered in the
//! order in which it was attached.
//!
//! ## Freeze criteria
//!
//! - Every error category has a documented meaning.
//! - Causes remain available through `std::error::Error::source()`.
//! - Stable codes do not depend on human-readable messages.
//! - Target infeasibility remains distinct from invalid source semantics.
//! - Operational budget exhaustion remains distinct from target capacity.
//! - The public API does not depend on HDLIR implementation details.

use core::fmt;
use std::error::Error;

/// Result type for fallible operations in the HDL subsystem.
///
/// This alias is deliberately scoped to HDL APIs. It does not replace the
/// repository's parser, semantic-analysis, or compiler result types.
pub type HdlResult<T> = Result<T, HdlError>;

/// Stable classification of an HDL subsystem failure.
///
/// These categories describe why an operation could not complete. They are
/// not a replacement for source-level semantic diagnostics.
///
/// The enum is non-exhaustive so new failure categories can be introduced
/// without requiring every downstream consumer to match every future variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HdlErrorKind {
    /// An API received malformed or contradictory input.
    InvalidInput,

    /// A configuration is malformed or internally inconsistent.
    InvalidConfiguration,

    /// A required declaration, value, capability description, or prerequisite
    /// has not been resolved.
    UnresolvedPrerequisite,

    /// A requested operation or feature is not supported by this
    /// implementation.
    UnsupportedOperation,

    /// A configured operational budget prevented an operation from
    /// completing.
    ResourceBudgetExhausted,

    /// The operation was explicitly cancelled before completion.
    Cancelled,

    /// The requested design requirements cannot be satisfied by the
    /// described target.
    TargetInfeasible,

    /// A target or implementation provider failed while processing a
    /// request.
    ProviderFailure,

    /// An internal API contract or invariant was violated.
    InternalContractViolation,
}

impl HdlErrorKind {
    /// Return the stable machine-readable identifier for this category.
    ///
    /// These identifiers are suitable for diagnostic adapters and
    /// machine-readable reporting. They are not human-readable messages.
    #[must_use]
    pub const fn stable_code(self) -> &'static str {
        match self {
            Self::InvalidInput => "HDL-E-INVALID-INPUT",
            Self::InvalidConfiguration => "HDL-E-INVALID-CONFIGURATION",
            Self::UnresolvedPrerequisite => "HDL-E-UNRESOLVED-PREREQUISITE",
            Self::UnsupportedOperation => "HDL-E-UNSUPPORTED-OPERATION",
            Self::ResourceBudgetExhausted => "HDL-E-BUDGET-EXHAUSTED",
            Self::Cancelled => "HDL-E-CANCELLED",
            Self::TargetInfeasible => "HDL-E-TARGET-INFEASIBLE",
            Self::ProviderFailure => "HDL-E-PROVIDER-FAILURE",
            Self::InternalContractViolation => "HDL-E-INTERNAL-CONTRACT",
        }
    }

    /// Return whether the failure describes an implementation or execution
    /// problem rather than a source-language semantic violation.
    ///
    /// This classification is advisory. A diagnostic adapter should still
    /// use the specific error kind and the surrounding pipeline stage.
    #[must_use]
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::ResourceBudgetExhausted
                | Self::Cancelled
                | Self::ProviderFailure
                | Self::InternalContractViolation
        )
    }

    /// Return whether the failure directly describes target feasibility.
    #[must_use]
    pub const fn is_target_feasibility_failure(self) -> bool {
        matches!(self, Self::TargetInfeasible)
    }
}

impl fmt::Display for HdlErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_code())
    }
}

/// Structured information about an operational budget exhaustion.
///
/// The `used` and `limit` fields describe the budget's accounting unit, which
/// is named by `unit`. The unit might be bytes, operations, work units, or
/// another explicitly documented quantity.
///
/// These fields do not represent hardware capacities or language-level
/// limits. They are descriptive counters supplied by the caller.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HdlBudgetExhaustion {
    /// Name of the operation that could not complete.
    pub operation: String,

    /// Name of the configured budget.
    pub budget: String,

    /// Unit used by the budget's accounting policy.
    pub unit: String,

    /// Amount accounted for when the operation stopped.
    pub used: u128,

    /// Configured budget value.
    pub limit: u128,
}

impl HdlBudgetExhaustion {
    /// Construct a budget-exhaustion description.
    ///
    /// This constructor records the supplied values without imposing a
    /// universal budget or assuming a particular accounting policy.
    #[must_use]
    pub fn new(
        operation: impl Into<String>,
        budget: impl Into<String>,
        unit: impl Into<String>,
        used: u128,
        limit: u128,
    ) -> Self {
        Self {
            operation: operation.into(),
            budget: budget.into(),
            unit: unit.into(),
            used,
            limit,
        }
    }
}

/// Structured details for a target-feasibility failure.
///
/// This records why a requested realization could not be established.
/// It does not own the target model, hardware inventory, or feasibility
/// analysis algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HdlTargetFailure {
    /// Identity or descriptive name of the target, when known.
    pub target: Option<String>,

    /// Requirement or capability that could not be satisfied.
    pub requirement: String,

    /// Explanation of the feasibility failure.
    pub reason: String,
}

impl HdlTargetFailure {
    /// Construct a target-feasibility failure description.
    #[must_use]
    pub fn new(
        target: Option<String>,
        requirement: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            target,
            requirement: requirement.into(),
            reason: reason.into(),
        }
    }
}

/// Structured details associated with an HDL subsystem error.
///
/// The variants preserve machine-readable details without making the error
/// depend on a particular parser, semantic model, target provider, or IR.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HdlErrorDetail {
    /// An invalid or missing input was identified.
    Input {
        /// Name of the input, declaration, or parameter.
        name: String,

        /// Explanation of the problem.
        reason: String,
    },

    /// A configuration entry was rejected.
    Configuration {
        /// Name of the configuration setting.
        setting: String,

        /// Explanation of the problem.
        reason: String,
    },

    /// A prerequisite could not be resolved.
    Prerequisite {
        /// Identity or description of the prerequisite.
        name: String,

        /// Explanation of why it remains unresolved.
        reason: String,
    },

    /// A requested operation is unsupported.
    Unsupported {
        /// Name of the requested operation or feature.
        operation: String,

        /// Explanation of the support limitation.
        reason: String,
    },

    /// An operational budget was exhausted.
    Budget(HdlBudgetExhaustion),

    /// The requested realization is infeasible for a target.
    Target(HdlTargetFailure),

    /// An implementation provider failed.
    Provider {
        /// Provider name or identity, if known.
        provider: Option<String>,

        /// Operation that failed.
        operation: String,
    },

    /// An internal contract or invariant was violated.
    Contract {
        /// Name of the affected contract or invariant.
        contract: String,

        /// Explanation of the violation.
        reason: String,
    },
}

/// Additional context attached to an HDL error.
///
/// Context is informational and does not change the stable classification
/// of the underlying failure. Each entry should identify a meaningful
/// operation, subsystem boundary, or processing stage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HdlErrorContext {
    /// Name of the subsystem or operation.
    pub operation: String,

    /// Optional description of the relevant object or processing stage.
    pub description: Option<String>,
}

impl HdlErrorContext {
    /// Construct a context entry.
    #[must_use]
    pub fn new(
        operation: impl Into<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            operation: operation.into(),
            description,
        }
    }
}

/// A typed failure produced by the HDL subsystem.
///
/// `HdlError` preserves its stable classification, optional structured
/// details, contextual information, and optional underlying cause.
///
/// This type intentionally does not implement `Clone` or `Eq`: an underlying
/// error may own non-cloneable resources and may not support equality.
/// Consumers needing stable comparison should compare `kind()`,
/// `stable_code()`, and explicitly selected structured details instead.
#[derive(Debug)]
pub struct HdlError {
    kind: HdlErrorKind,
    message: String,
    detail: Option<HdlErrorDetail>,
    context: Vec<HdlErrorContext>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl HdlError {
    /// Construct an HDL error with a classification and human-readable
    /// explanation.
    ///
    /// Prefer the specialized constructors below when structured details
    /// are available.
    #[must_use]
    pub fn new(kind: HdlErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            detail: None,
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an invalid-input error.
    #[must_use]
    pub fn invalid_input(
        name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let reason = reason.into();

        Self {
            kind: HdlErrorKind::InvalidInput,
            message: format!("invalid HDL input `{name}`: {reason}"),
            detail: Some(HdlErrorDetail::Input { name, reason }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct a configuration error.
    #[must_use]
    pub fn invalid_configuration(
        setting: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let setting = setting.into();
        let reason = reason.into();

        Self {
            kind: HdlErrorKind::InvalidConfiguration,
            message: format!("invalid HDL configuration `{setting}`: {reason}"),
            detail: Some(HdlErrorDetail::Configuration { setting, reason }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an unresolved-prerequisite error.
    #[must_use]
    pub fn unresolved_prerequisite(
        name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let reason = reason.into();

        Self {
            kind: HdlErrorKind::UnresolvedPrerequisite,
            message: format!("unresolved HDL prerequisite `{name}`: {reason}"),
            detail: Some(HdlErrorDetail::Prerequisite { name, reason }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an unsupported-operation error.
    #[must_use]
    pub fn unsupported_operation(
        operation: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let operation = operation.into();
        let reason = reason.into();

        Self {
            kind: HdlErrorKind::UnsupportedOperation,
            message: format!("unsupported HDL operation `{operation}`: {reason}"),
            detail: Some(HdlErrorDetail::Unsupported { operation, reason }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an operational-budget-exhaustion error.
    #[must_use]
    pub fn budget_exhausted(details: HdlBudgetExhaustion) -> Self {
        let message = format!(
            "HDL operation `{}` exhausted budget `{}`: {} {} used, limit {} {}",
            details.operation,
            details.budget,
            details.used,
            details.unit,
            details.limit,
            details.unit,
        );

        Self {
            kind: HdlErrorKind::ResourceBudgetExhausted,
            message,
            detail: Some(HdlErrorDetail::Budget(details)),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an explicit cancellation error.
    #[must_use]
    pub fn cancelled(operation: impl Into<String>) -> Self {
        let operation = operation.into();

        Self::new(
            HdlErrorKind::Cancelled,
            format!("HDL operation `{operation}` was cancelled"),
        )
    }

    /// Construct a target-feasibility error.
    ///
    /// This means the requested realization could not satisfy the supplied
    /// requirements. It does not imply that the HDL source is semantically
    /// invalid.
    #[must_use]
    pub fn target_infeasible(details: HdlTargetFailure) -> Self {
        let message = match details.target.as_deref() {
            Some(target) => format!(
                "HDL target `{target}` cannot satisfy requirement `{}`: {}",
                details.requirement, details.reason
            ),
            None => format!(
                "HDL target feasibility failed for requirement `{}`: {}",
                details.requirement, details.reason
            ),
        };

        Self {
            kind: HdlErrorKind::TargetInfeasible,
            message,
            detail: Some(HdlErrorDetail::Target(details)),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct a provider-failure error.
    #[must_use]
    pub fn provider_failure(
        provider: Option<String>,
        operation: impl Into<String>,
    ) -> Self {
        let operation = operation.into();

        let message = match provider.as_deref() {
            Some(provider) => format!(
                "HDL provider `{provider}` failed during `{operation}`"
            ),
            None => format!("HDL provider failed during `{operation}`"),
        };

        Self {
            kind: HdlErrorKind::ProviderFailure,
            message,
            detail: Some(HdlErrorDetail::Provider {
                provider,
                operation,
            }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Construct an internal-contract-violation error.
    ///
    /// Use this when a subsystem contract is violated, not for ordinary
    /// malformed user source or a target that lacks a capability.
    #[must_use]
    pub fn contract_violation(
        contract: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let contract = contract.into();
        let reason = reason.into();

        Self {
            kind: HdlErrorKind::InternalContractViolation,
            message: format!(
                "HDL contract `{contract}` violated: {reason}"
            ),
            detail: Some(HdlErrorDetail::Contract { contract, reason }),
            context: Vec::new(),
            source: None,
        }
    }

    /// Return the error classification.
    #[must_use]
    pub const fn kind(&self) -> HdlErrorKind {
        self.kind
    }

    /// Return the stable machine-readable error code.
    #[must_use]
    pub const fn stable_code(&self) -> &'static str {
        self.kind.stable_code()
    }

    /// Return the primary human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return optional structured error details.
    #[must_use]
    pub fn detail(&self) -> Option<&HdlErrorDetail> {
        self.detail.as_ref()
    }

    /// Return contextual information in attachment order.
    #[must_use]
    pub fn context(&self) -> &[HdlErrorContext] {
        &self.context
    }

    /// Attach context describing the operation or subsystem boundary.
    ///
    /// Context is appended in the order in which calls are made. It does not
    /// replace the original error classification or underlying cause.
    #[must_use]
    pub fn with_context(
        mut self,
        operation: impl Into<String>,
        description: Option<String>,
    ) -> Self {
        self.context.push(HdlErrorContext::new(operation, description));
        self
    }

    /// Attach an underlying error while preserving its concrete type in the
    /// standard error source chain.
    ///
    /// This method does not stringify or discard the supplied error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Return the immediate underlying error, if present.
    #[must_use]
    pub fn source_error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl fmt::Display for HdlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.stable_code(), self.message)?;

        for context in &self.context {
            write!(formatter, "\n  while {}", context.operation)?;

            if let Some(description) = &context.description {
                write!(formatter, ": {description}")?;
            }
        }

        Ok(())
    }
}

impl Error for HdlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn stable_codes_are_non_empty_and_distinct() {
        let kinds = [
            HdlErrorKind::InvalidInput,
            HdlErrorKind::InvalidConfiguration,
            HdlErrorKind::UnresolvedPrerequisite,
            HdlErrorKind::UnsupportedOperation,
            HdlErrorKind::ResourceBudgetExhausted,
            HdlErrorKind::Cancelled,
            HdlErrorKind::TargetInfeasible,
            HdlErrorKind::ProviderFailure,
            HdlErrorKind::InternalContractViolation,
        ];

        for (index, kind) in kinds.iter().enumerate() {
            assert!(!kind.stable_code().is_empty());

            for other in kinds.iter().skip(index + 1) {
                assert_ne!(kind.stable_code(), other.stable_code());
            }
        }
    }

    #[test]
    fn invalid_input_preserves_structured_details() {
        let error = HdlError::invalid_input(
            "signal_width",
            "the expression is malformed",
        );

        assert_eq!(error.kind(), HdlErrorKind::InvalidInput);

        match error.detail() {
            Some(HdlErrorDetail::Input { name, reason }) => {
                assert_eq!(name, "signal_width");
                assert_eq!(reason, "the expression is malformed");
            }
            other => panic!("unexpected error detail: {other:?}"),
        }
    }

    #[test]
    fn configuration_failure_is_distinct_from_invalid_input() {
        let error = HdlError::invalid_configuration(
            "analysis_budget",
            "the configured value is inconsistent",
        );

        assert_eq!(
            error.kind(),
            HdlErrorKind::InvalidConfiguration
        );
        assert_ne!(
            error.stable_code(),
            HdlErrorKind::InvalidInput.stable_code()
        );
    }

    #[test]
    fn budget_exhaustion_retains_accounting_information() {
        let details = HdlBudgetExhaustion::new(
            "dependency_analysis",
            "work_units",
            "operations",
            120,
            100,
        );

        let error = HdlError::budget_exhausted(details);

        assert_eq!(
            error.kind(),
            HdlErrorKind::ResourceBudgetExhausted
        );

        match error.detail() {
            Some(HdlErrorDetail::Budget(details)) => {
                assert_eq!(details.operation, "dependency_analysis");
                assert_eq!(details.used, 120);
                assert_eq!(details.limit, 100);
                assert_eq!(details.unit, "operations");
            }
            other => panic!("unexpected error detail: {other:?}"),
        }
    }

    #[test]
    fn target_infeasibility_is_not_source_invalidity() {
        let error = HdlError::target_infeasible(HdlTargetFailure::new(
            Some("example-target".to_owned()),
            "required_capability",
            "the target does not provide this capability",
        ));

        assert_eq!(error.kind(), HdlErrorKind::TargetInfeasible);
        assert!(error.kind().is_target_feasibility_failure());
        assert!(!error.kind().is_operational());
    }

    #[test]
    fn context_is_preserved_and_rendered_deterministically() {
        let error = HdlError::unresolved_prerequisite(
            "clock_domain",
            "the clock declaration has not been resolved",
        )
        .with_context(
            "validate_process",
            Some("process `controller`".to_owned()),
        )
        .with_context("validate_design", None);

        assert_eq!(error.context().len(), 2);
        assert!(error.context()[0].operation == "validate_process");
        assert!(error.context()[1].operation == "validate_design");

        let first = error.to_string();
        let second = error.to_string();

        assert_eq!(first, second);
        assert!(first.contains("HDL-E-UNRESOLVED-PREREQUISITE"));
        assert!(first.contains("process `controller`"));
    }

    #[test]
    fn underlying_error_is_available_through_standard_error_chain() {
        let error = HdlError::provider_failure(
            Some("test-provider".to_owned()),
            "initialize",
        )
        .with_source(std::io::Error::other("provider initialization failed"));

        assert_eq!(error.kind(), HdlErrorKind::ProviderFailure);
        assert!(error.source().is_some());
        assert!(error.source_error().is_some());
        assert!(error.to_string().contains("test-provider"));
    }

    #[test]
    fn cancellation_is_distinct_from_budget_exhaustion() {
        let error = HdlError::cancelled("elaboration");

        assert_eq!(error.kind(), HdlErrorKind::Cancelled);
        assert!(error.kind().is_operational());
        assert_ne!(
            error.stable_code(),
            HdlErrorKind::ResourceBudgetExhausted.stable_code()
        );
    }

    #[test]
    fn result_alias_supports_normal_error_propagation() {
        fn operation() -> HdlResult<()> {
            Err(HdlError::invalid_input("module", "missing declaration"))
        }

        let result = operation();

        assert!(matches!(
            result,
            Err(ref error) if error.kind() == HdlErrorKind::InvalidInput
        ));
    }

    #[test]
    fn contract_violations_are_distinct_from_user_input_errors() {
        let error = HdlError::contract_violation(
            "provider_response",
            "the provider returned an inconsistent result",
        );

        assert_eq!(
            error.kind(),
            HdlErrorKind::InternalContractViolation
        );

        match error.detail() {
            Some(HdlErrorDetail::Contract { contract, reason }) => {
                assert_eq!(contract, "provider_response");
                assert_eq!(
                    reason,
                    "the provider returned an inconsistent result"
                );
            }
            other => panic!("unexpected error detail: {other:?}"),
        }
    }
}
