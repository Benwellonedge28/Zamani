
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL realization provider interface
//!
//! Path: `src/hdl/realization/interface.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//!
//! ## Purpose
//!
//! Define the portable contract between a declarative HDL realization plan
//! and an implementation provider.
//!
//! This module negotiates provider capabilities, validates preconditions,
//! coordinates requested operations, supports cooperative cancellation, and
//! checks the resulting report's bookkeeping.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/hdl/realization/plan.rs`
//! - `src/hdl/realization/reports.rs`
//! - `src/hdl/targets/model.rs`
//! - `src/hdl/targets/feasibility.rs`
//!
//! ## Ownership
//!
//! This module owns provider negotiation and orchestration contracts.
//!
//! It does not own source parsing, semantic validation, target discovery,
//! capability evidence evaluation, physical allocation, synthesis, placement,
//! routing, deployment, or any intermediate representation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Safety and scalability
//!
//! This module uses safe Rust only. Collections have no arbitrary
//! language-level item ceilings. Work is bounded by available resources and
//! provider-defined operational policies.
//!
//! Cancellation is cooperative: providers must check the supplied context
//! during long-running work. Cancellation cannot forcibly interrupt a
//! provider blocked in an operation that ignores the context.
//!
//! ## Integration
//!
//! The composition root must register this module alongside `plan` and
//! `reports`. This module uses their existing public contracts rather than
//! defining duplicate plans, target identities, or report records.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::hdl::realization::plan::{
    HdlRealizationOperation,
    HdlRealizationPlan,
};
use crate::hdl::realization::reports::{
    HdlOperationOutcome,
    HdlOperationResult,
    HdlRealizationOutcome,
    HdlRealizationReport,
    HdlReportError,
    HdlReportIssue,
    HdlReportIssueKind,
    HdlReportSeverity,
};
use crate::hdl::targets::feasibility::FeasibilityStatus;

/// Stable architectural name of this module.
pub const HDL_REALIZATION_INTERFACE_MODULE_NAME: &str =
    "zamani.hdl.realization.interface";

/// Version of the provider-interface contract.
///
/// This is not the language, target, provider, or HDLIR version.
pub const HDL_REALIZATION_INTERFACE_VERSION: u16 = 1;

/// An implementation provider's identity and declared capabilities.
///
/// Provider identifiers and operation identifiers are open-ended. The
/// interface does not maintain a closed list of vendors, hardware types,
/// implementation techniques, or future operations.
///
/// Advertised capabilities are declarations by the provider. They are not
/// independent proof that a physical device exists or that realization will
/// succeed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlProviderDescriptor {
    id: String,
    version: String,
    interface_version: u16,
    operations: BTreeSet<String>,
}

impl HdlProviderDescriptor {
    /// Create a provider descriptor.
    ///
    /// `id` and `version` must be nonempty and contain no control characters.
    /// Operation identifiers are validated in the same way. Identical
    /// operation identifiers are naturally deduplicated by the set.
    ///
    /// # Errors
    ///
    /// Returns an error when an identifier or version is invalid, or when
    /// the interface version is zero.
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
        interface_version: u16,
        operations: impl IntoIterator<Item = String>,
    ) -> Result<Self, HdlRealizationInterfaceError> {
        let id = id.into();
        let version = version.into();

        validate_text(&id, "provider identifier")?;
        validate_text(&version, "provider version")?;

        if interface_version == 0 {
            return Err(HdlRealizationInterfaceError::InvalidInput {
                field: "interface version",
                reason: "must be nonzero".to_owned(),
            });
        }

        let mut validated_operations = BTreeSet::new();

        for operation in operations {
            validate_text(&operation, "operation identifier")?;
            validated_operations.insert(operation);
        }

        Ok(Self {
            id,
            version,
            interface_version,
            operations: validated_operations,
        })
    }

    /// Return the provider identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the provider version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the provider-interface contract version.
    #[must_use]
    pub const fn interface_version(&self) -> u16 {
        self.interface_version
    }

    /// Return advertised operation identifiers in deterministic order.
    pub fn operations(&self) -> impl Iterator<Item = &str> {
        self.operations.iter().map(String::as_str)
    }

    /// Return whether the provider advertises the given operation.
    #[must_use]
    pub fn supports_operation(&self, id: &str) -> bool {
        self.operations.contains(id)
    }
}

/// Explicit policy for provider negotiation and execution.
///
/// These choices are policies, not changes to HDL language semantics.
/// Neither option makes a target feasible or makes an unsupported operation
/// supported.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdlRealizationPolicy {
    /// Permit a plan whose feasibility assessment is indeterminate or absent.
    ///
    /// When false, realization requires an attached assessment with status
    /// `Feasible`. An assessment marked `Infeasible` is rejected regardless
    /// of this option.
    pub allow_unproven_feasibility: bool,

    /// Permit unsupported optional operations.
    ///
    /// Required operations are never permitted to be unsupported.
    pub allow_unsupported_optional_operations: bool,
}

/// Cooperative cancellation context supplied to a provider.
///
/// Clones share the same cancellation state. A coordinator or caller can
/// request cancellation through [`Self::cancel`]. Providers should check
/// [`Self::is_cancelled`] between bounded units of work.
///
/// Cancellation is not a forced interruption mechanism.
#[derive(Clone, Debug, Default)]
pub struct HdlRealizationContext {
    cancelled: Arc<AtomicBool>,
}

impl HdlRealizationContext {
    /// Create a fresh, uncancelled context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation for this context and all its clones.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Return whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Return an error if cancellation has been requested.
    ///
    /// Providers should call this at safe checkpoints before starting
    /// additional work and after returning from long-running operations.
    pub fn check_cancelled(
        &self,
    ) -> Result<(), HdlRealizationInterfaceError> {
        if self.is_cancelled() {
            Err(HdlRealizationInterfaceError::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// Errors produced by the realization interface.
#[derive(Debug)]
#[non_exhaustive]
pub enum HdlRealizationInterfaceError {
    /// An interface argument is invalid.
    InvalidInput {
        /// Name of the invalid field.
        field: &'static str,
        /// Explanation of the validation failure.
        reason: String,
    },

    /// The provider uses an incompatible interface version.
    IncompatibleInterfaceVersion {
        /// Version required by the coordinator.
        expected: u16,
        /// Version advertised by the provider.
        actual: u16,
    },

    /// The plan has no feasibility evidence, or its feasibility is
    /// indeterminate when policy does not permit proceeding.
    FeasibilityNotEstablished,

    /// The target has been assessed as infeasible.
    InfeasibleTarget,

    /// A blocking implementation decision remains unresolved.
    BlockingDecisions,

    /// A required operation is not advertised by the provider.
    UnsupportedRequiredOperation {
        /// Unsupported operation identifier.
        operation_id: String,
    },

    /// An optional unsupported operation is disallowed by policy.
    UnsupportedOptionalOperation {
        /// Unsupported operation identifier.
        operation_id: String,
    },

    /// The provider returned a result for the wrong operation.
    OperationMismatch {
        /// Requested operation identifier.
        expected: String,
        /// Returned operation identifier.
        actual: String,
    },

    /// The provider returned a pending result for an operation it claimed
    /// to execute.
    PendingOperationResult {
        /// Operation identifier.
        operation_id: String,
    },

    /// The provider's returned report does not match the requested target.
    TargetMismatch {
        /// Target identifier expected by the plan.
        expected: String,
        /// Target identifier reported by the provider.
        actual: String,
    },

    /// Cancellation was requested.
    Cancelled,

    /// The provider failed while performing an operation.
    ProviderFailure {
        /// Provider identifier.
        provider_id: String,
        /// Operation identifier, if applicable.
        operation_id: Option<String>,
        /// Human-readable explanation.
        message: String,
    },

    /// A report violated its canonical bookkeeping contract.
    InvalidReport(HdlReportError),
}

impl fmt::Display for HdlRealizationInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { field, reason } => {
                write!(f, "invalid {field}: {reason}")
            }
            Self::IncompatibleInterfaceVersion { expected, actual } => {
                write!(
                    f,
                    "incompatible realization interface version: \
                     expected {expected}, provider advertises {actual}"
                )
            }
            Self::FeasibilityNotEstablished => {
                f.write_str("target feasibility has not been established")
            }
            Self::InfeasibleTarget => {
                f.write_str("target is infeasible for this realization plan")
            }
            Self::BlockingDecisions => {
                f.write_str("blocking realization decisions remain unresolved")
            }
            Self::UnsupportedRequiredOperation { operation_id } => {
                write!(
                    f,
                    "provider does not support required operation `{operation_id}`"
                )
            }
            Self::UnsupportedOptionalOperation { operation_id } => {
                write!(
                    f,
                    "provider does not support optional operation \
                     `{operation_id}`, and policy disallows omission"
                )
            }
            Self::OperationMismatch { expected, actual } => {
                write!(
                    f,
                    "provider returned operation `{actual}` for request `{expected}`"
                )
            }
            Self::PendingOperationResult { operation_id } => {
                write!(
                    f,
                    "provider returned a pending result for executed operation \
                     `{operation_id}`"
                )
            }
            Self::TargetMismatch { expected, actual } => {
                write!(
                    f,
                    "provider report target `{actual}` does not match plan target \
                     `{expected}`"
                )
            }
            Self::Cancelled => f.write_str("HDL realization was cancelled"),
            Self::ProviderFailure {
                provider_id,
                operation_id,
                message,
            } => {
                write!(f, "provider `{provider_id}` failed")?;
                if let Some(operation) = operation_id {
                    write!(f, " during `{operation}`")?;
                }
                write!(f, ": {message}")
            }
            Self::InvalidReport(error) => {
                write!(f, "invalid realization report: {error}")
            }
        }
    }
}

impl Error for HdlRealizationInterfaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidReport(error) => Some(error),
            _ => None,
        }
    }
}

impl From<HdlReportError> for HdlRealizationInterfaceError {
    fn from(error: HdlReportError) -> Self {
        Self::InvalidReport(error)
    }
}

/// Provider contract for realizing an HDL plan.
///
/// Implementations must not reinterpret the source program's meaning.
/// They must not claim that estimates are measured resource usage, that
/// advertised capabilities prove physical availability, or that successful
/// execution independently verifies the resulting hardware.
///
/// Providers must check cancellation at suitable checkpoints and return
/// operation results with the exact identifier requested by the coordinator.
///
/// The interface does not require a provider to use a particular hardware
/// vendor, simulator, implementation algorithm, or internal representation.
pub trait HdlRealizationProvider: Send + Sync {
    /// Return the provider's identity and advertised operations.
    fn descriptor(&self) -> &HdlProviderDescriptor;

    /// Execute one requested operation.
    ///
    /// The provider must return a terminal operation outcome rather than
    /// `Pending`. A failed operation may be represented by an operation
    /// result or by returning a provider error.
    ///
    /// The coordinator validates the returned operation identifier.
    fn execute_operation(
        &self,
        plan: &HdlRealizationPlan,
        operation: &HdlRealizationOperation,
        context: &HdlRealizationContext,
    ) -> Result<HdlOperationResult, HdlRealizationInterfaceError>;
}

/// Validate the provider contract and execute a realization plan.
///
/// Operations are processed in the deterministic order supplied by the plan.
/// The coordinator does not silently skip unsupported required operations,
/// unresolved blocking decisions, or infeasible targets.
///
/// An indeterminate or missing feasibility assessment is accepted only when
/// `policy.allow_unproven_feasibility` is true. This option permits an attempt;
/// it does not change the feasibility result.
///
/// # Errors
///
/// Returns an error for failed preconditions, incompatible provider versions,
/// unsupported operations prohibited by policy, cancellation, provider
/// failures, target mismatches, or invalid report bookkeeping.
///
/// # Important
///
/// A returned report records the provider's reported outcomes. It is not
/// independent verification of generated artifacts or physical hardware.
pub fn realize(
    provider: &dyn HdlRealizationProvider,
    plan: &HdlRealizationPlan,
    policy: HdlRealizationPolicy,
    context: &HdlRealizationContext,
) -> Result<HdlRealizationReport, HdlRealizationInterfaceError> {
    context.check_cancelled()?;

    let descriptor = provider.descriptor();

    if descriptor.interface_version() != HDL_REALIZATION_INTERFACE_VERSION {
        return Err(
            HdlRealizationInterfaceError::IncompatibleInterfaceVersion {
                expected: HDL_REALIZATION_INTERFACE_VERSION,
                actual: descriptor.interface_version(),
            },
        );
    }

    if plan.has_blocking_decisions() {
        return Err(HdlRealizationInterfaceError::BlockingDecisions);
    }

    match plan.feasibility_status() {
        Some(FeasibilityStatus::Infeasible) => {
            return Err(HdlRealizationInterfaceError::InfeasibleTarget);
        }
        Some(FeasibilityStatus::Feasible) => {}
        Some(FeasibilityStatus::Indeterminate) | None => {
            if !policy.allow_unproven_feasibility {
                return Err(
                    HdlRealizationInterfaceError::FeasibilityNotEstablished,
                );
            }
        }
    }

    // Preflight all operations before invoking the provider. This prevents
    // avoidable partial execution when a required operation is unsupported.
    for operation in plan.operations() {
        if descriptor.supports_operation(operation.id()) {
            continue;
        }

        if operation.is_required() {
            return Err(
                HdlRealizationInterfaceError::UnsupportedRequiredOperation {
                    operation_id: operation.id().to_owned(),
                },
            );
        }

        if !policy.allow_unsupported_optional_operations {
            return Err(
                HdlRealizationInterfaceError::UnsupportedOptionalOperation {
                    operation_id: operation.id().to_owned(),
                },
            );
        }
    }

    context.check_cancelled()?;

    let mut report = HdlRealizationReport::from_plan(plan)?;

    for operation in plan.operations() {
        context.check_cancelled()?;

        if !descriptor.supports_operation(operation.id()) {
            let result = HdlOperationResult::new(
                operation.id(),
                HdlOperationOutcome::Unsupported,
                Some("Optional operation is not supported by this provider.".to_owned()),
            )?;

            report.record_operation(result)?;
            continue;
        }

        let result = provider.execute_operation(plan, operation, context)?;

        if result.operation_id() != operation.id() {
            return Err(
                HdlRealizationInterfaceError::OperationMismatch {
                    expected: operation.id().to_owned(),
                    actual: result.operation_id().to_owned(),
                },
            );
        }

        if result.outcome() == HdlOperationOutcome::Pending {
            return Err(
                HdlRealizationInterfaceError::PendingOperationResult {
                    operation_id: operation.id().to_owned(),
                },
            );
        }

        report.record_operation(result)?;
    }

    context.check_cancelled()?;

    let has_failed_operation = report.operations().any(|result| {
        matches!(
            result.outcome(),
            HdlOperationOutcome::Failed
                | HdlOperationOutcome::Cancelled
                | HdlOperationOutcome::Unsupported
        )
    });

    let has_partial_operation = report.operations().any(|result| {
        result.outcome() == HdlOperationOutcome::Partial
    });

    let has_unproven_feasibility = !matches!(
        plan.feasibility_status(),
        Some(FeasibilityStatus::Feasible)
    );

    let has_unresolved_optional_operation = report.operations().any(|result| {
        result.outcome() == HdlOperationOutcome::Unsupported
    });

    let outcome = if has_failed_operation {
        HdlRealizationOutcome::Failed
    } else if has_partial_operation
        || has_unproven_feasibility
        || has_unresolved_optional_operation
    {
        HdlRealizationOutcome::Partial
    } else {
        HdlRealizationOutcome::Completed
    };

    report.set_outcome(outcome);
    report.validate_consistency()?;

    Ok(report)
}

fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), HdlRealizationInterfaceError> {
    if value.trim().is_empty() {
        return Err(HdlRealizationInterfaceError::InvalidInput {
            field,
            reason: "must not be empty".to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlRealizationInterfaceError::InvalidInput {
            field,
            reason: "must not contain control characters".to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_descriptor_preserves_open_ended_operations() {
        let descriptor = HdlProviderDescriptor::new(
            "example.provider",
            "1.0",
            HDL_REALIZATION_INTERFACE_VERSION,
            vec![
                "future.operation".to_owned(),
                "synthesis.prepare".to_owned(),
                "synthesis.prepare".to_owned(),
            ],
        )
        .expect("valid descriptor");

        assert!(descriptor.supports_operation("future.operation"));
        assert!(descriptor.supports_operation("synthesis.prepare"));
        assert!(!descriptor.supports_operation("unadvertised.operation"));

        let operations: Vec<_> = descriptor.operations().collect();
        assert_eq!(
            operations,
            vec!["future.operation", "synthesis.prepare"]
        );
    }

    #[test]
    fn provider_descriptor_rejects_invalid_identifiers() {
        assert!(HdlProviderDescriptor::new(
            "",
            "1.0",
            HDL_REALIZATION_INTERFACE_VERSION,
            Vec::<String>::new(),
        )
        .is_err());

        assert!(HdlProviderDescriptor::new(
            "example.provider",
            "1.0",
            0,
            Vec::<String>::new(),
        )
        .is_err());

        assert!(HdlProviderDescriptor::new(
            "example.provider",
            "1.0",
            HDL_REALIZATION_INTERFACE_VERSION,
            vec!["bad\noperation".to_owned()],
        )
        .is_err());
    }

    #[test]
    fn cancellation_is_shared_between_context_clones() {
        let context = HdlRealizationContext::new();
        let clone = context.clone();

        assert!(!context.is_cancelled());

        clone.cancel();

        assert!(context.is_cancelled());
        assert!(matches!(
            context.check_cancelled(),
            Err(HdlRealizationInterfaceError::Cancelled)
        ));
    }

    #[test]
    fn interface_error_retains_report_error_source() {
        fn assert_error_source<E: Error + 'static>(error: &E) {
            let _ = error.source();
        }

        let error = HdlRealizationInterfaceError::Cancelled;
        assert_error_source(&error);
        assert!(error.source().is_none());
    }
}
