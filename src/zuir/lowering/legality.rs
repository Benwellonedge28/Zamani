
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZUIR Lowering Legality
//!
//! Path: `src/zuir/lowering/legality.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Determine whether a proposed lowering is justified by explicit source
//! semantics, destination operation contracts, capability evidence, and
//! declared semantic-preservation guarantees.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/lowering/context.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/operation.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - Lowering-legality input and result contracts.
//! - Deterministic comparison of supplied semantic evidence.
//! - Explicit handling of unsupported and unresolved conversions.
//! - Translation of legality outcomes into lowering-context dispositions.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing, AST construction, or semantic inference.
//! - Dialect registration or operation-schema storage.
//! - Whole-program verification, type inference, or SSA analysis.
//! - Resource allocation, target discovery, scheduling, routing,
//!   calibration, code generation, or runtime execution.
//! - Proof that an externally supplied contract is truthful.
//!
//! ## INTEGRATION
//!
//! Adapters resolve source and destination operation schemas through the
//! canonical dialect registry and verify IR through the canonical verifier.
//! They then construct a `LegalityCandidate` from that evidence.
//!
//! `LoweringContext` records the decision, symbolic resource requirements,
//! and deterministic diagnostic notes. This module does not create a
//! competing registry, verifier, or target model.
//!
//! ## SEMANTIC SAFETY
//!
//! A missing operation definition, incomplete verification result, unknown
//! semantic effect, or unresolved required guarantee never proves legality.
//! Unknown operations must not acquire invented semantics.
//!
//! Legality is not target feasibility. This module does not prove that a
//! device has sufficient physical resources or that execution will succeed.
//!
//! ## SCALABILITY
//!
//! Collections are dynamically sized. No universal maximum operation count,
//! qubit count, register width, tensor rank, node count, or device count is
//! imposed. Work accounting is delegated to `LoweringContext`.
//!
//! ## DETERMINISM
//!
//! Ordered sets and stable decision codes provide deterministic comparisons
//! and reports for equivalent supplied evidence.
//!
//! ## FREEZE CRITERIA
//!
//! - Unknown or incomplete evidence cannot authorize lowering.
//! - Source and destination effects are compared explicitly.
//! - Required capabilities and semantic guarantees are checked.
//! - Expansion is distinguished from direct lowering.
//! - Resource requirements remain symbolic.
//! - Every decision has a stable diagnostic code.
//! - The file compiles against the actual lowering-context API.
//! - Unit and integration tests pass.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::zuir::lowering::context::{
    EntityKey,
    LoweringContext,
    LoweringContextError,
    LoweringDisposition,
    LoweringNote,
    ResourceRequirement,
};

/// Result alias for legality analysis.
pub type LegalityResult<T> = Result<T, LegalityError>;

/// The outcome of legality analysis.
///
/// `Legal` authorizes the proposed semantic conversion under the supplied
/// contracts. It does not prove physical target feasibility or execution
/// success.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LegalityStatus {
    /// The supplied evidence establishes semantic legality.
    Legal,

    /// Additional conditions must be established before lowering.
    Conditional {
        /// Conditions that must be checked by the caller.
        conditions: Vec<String>,
    },

    /// Direct conversion is not justified; expansion is required.
    ExpansionRequired {
        /// Registered or otherwise validated expansion strategy identifier.
        strategy: String,
    },

    /// The proposed conversion cannot preserve the required semantics.
    Unsupported {
        /// Stable explanation for the rejection.
        reason: String,
    },

    /// The available evidence is insufficient to decide legality.
    Deferred {
        /// Missing or unresolved facts.
        reasons: Vec<String>,
    },
}

impl LegalityStatus {
    /// Whether direct lowering is authorized by this decision.
    #[must_use]
    pub fn permits_direct_lowering(&self) -> bool {
        matches!(self, Self::Legal)
    }

    /// Whether the decision is conclusive.
    #[must_use]
    pub fn is_conclusive(&self) -> bool {
        !matches!(self, Self::Deferred { .. })
    }

    /// Return a stable machine-readable decision code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Legal => "ZUIR-LOWER-LEGAL",
            Self::Conditional { .. } => "ZUIR-LOWER-CONDITIONAL",
            Self::ExpansionRequired { .. } => "ZUIR-LOWER-EXPANSION",
            Self::Unsupported { .. } => "ZUIR-LOWER-UNSUPPORTED",
            Self::Deferred { .. } => "ZUIR-LOWER-DEFERRED",
        }
    }
}

/// Semantic facts required to assess a source operation.
///
/// Names are qualified identifiers resolved by the adapter. This module
/// deliberately does not maintain a hard-coded list of operation names,
/// effects, capabilities, or computational domains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationSemantics {
    /// Fully qualified source operation name.
    pub operation_name: String,

    /// Declared semantic effects of the source operation.
    ///
    /// Examples of possible effects include memory writes, I/O,
    /// synchronization, measurement, communication, or extension-defined
    /// effects. The actual vocabulary is owned by the relevant contracts.
    pub effects: BTreeSet<String>,

    /// Capabilities required for the operation's intended semantics.
    pub required_capabilities: BTreeSet<String>,

    /// Semantic guarantees that a destination must preserve.
    ///
    /// The dialect contract defines the meaning of each guarantee.
    pub required_guarantees: BTreeSet<String>,

    /// Whether some semantically relevant source facts remain opaque.
    ///
    /// `true` prevents a successful legality result.
    pub has_unresolved_semantics: bool,
}

impl OperationSemantics {
    /// Construct a source semantic contract.
    pub fn new(
        operation_name: impl Into<String>,
        effects: BTreeSet<String>,
        required_capabilities: BTreeSet<String>,
        required_guarantees: BTreeSet<String>,
        has_unresolved_semantics: bool,
    ) -> LegalityResult<Self> {
        let operation_name = operation_name.into();

        validate_name(&operation_name, "source operation")?;
        validate_names(&effects, "source effect")?;
        validate_names(&required_capabilities, "required capability")?;
        validate_names(&required_guarantees, "required guarantee")?;

        Ok(Self {
            operation_name,
            effects,
            required_capabilities,
            required_guarantees,
            has_unresolved_semantics,
        })
    }
}

/// Evidence supplied by a destination dialect adapter.
///
/// The adapter must derive this information from the actual registered
/// destination operation schema and its semantic contract, not from an
/// operation's spelling or an assumed hardware instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestinationSemantics {
    /// Fully qualified destination operation name.
    pub operation_name: String,

    /// Effects declared by the destination operation contract.
    pub effects: BTreeSet<String>,

    /// Guarantees explicitly preserved by the destination operation.
    pub preserved_guarantees: BTreeSet<String>,

    /// Whether the destination contract has unresolved semantics.
    pub has_unresolved_semantics: bool,

    /// Whether the destination operation schema has been validated.
    pub schema_verified: bool,
}

impl DestinationSemantics {
    /// Construct destination evidence.
    pub fn new(
        operation_name: impl Into<String>,
        effects: BTreeSet<String>,
        preserved_guarantees: BTreeSet<String>,
        has_unresolved_semantics: bool,
        schema_verified: bool,
    ) -> LegalityResult<Self> {
        let operation_name = operation_name.into();

        validate_name(&operation_name, "destination operation")?;
        validate_names(&effects, "destination effect")?;
        validate_names(&preserved_guarantees, "preserved guarantee")?;

        Ok(Self {
            operation_name,
            effects,
            preserved_guarantees,
            has_unresolved_semantics,
            schema_verified,
        })
    }
}

/// All evidence required for one lowering-legality decision.
///
/// Source verification and destination schema validation are separate
/// facts. Neither is inferred from the existence of an operation name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalityCandidate {
    /// Source entity whose conversion is being considered.
    pub source: EntityKey,

    /// Validated source semantic facts, if available.
    pub source_semantics: Option<OperationSemantics>,

    /// Validated destination semantic facts, if available.
    pub destination_semantics: Option<DestinationSemantics>,

    /// Capabilities confirmed available by the caller's target analysis.
    pub available_capabilities: BTreeSet<String>,

    /// Symbolic resource requirements established by semantic analysis.
    ///
    /// These are recorded, not physically allocated or solved here.
    pub resource_requirements: Vec<ResourceRequirement>,

    /// Additional conditions imposed by the adapter or conversion contract.
    pub conditions: Vec<String>,

    /// Optional validated expansion strategy for this conversion.
    pub expansion_strategy: Option<String>,

    /// Whether the source representation passed the required verification.
    pub source_verified: bool,
}

impl LegalityCandidate {
    /// Validate the evidence container and construct a candidate.
    pub fn new(
        source: EntityKey,
        source_semantics: Option<OperationSemantics>,
        destination_semantics: Option<DestinationSemantics>,
        available_capabilities: BTreeSet<String>,
        resource_requirements: Vec<ResourceRequirement>,
        conditions: Vec<String>,
        expansion_strategy: Option<String>,
        source_verified: bool,
    ) -> LegalityResult<Self> {
        validate_names(&available_capabilities, "available capability")?;

        if conditions.iter().any(|value| {
            value.trim().is_empty() || value.contains('\0')
        }) {
            return Err(LegalityError::InvalidInput(
                "conditions must be non-empty and contain no NUL characters"
                    .to_owned(),
            ));
        }

        if let Some(strategy) = expansion_strategy.as_deref() {
            validate_name(strategy, "expansion strategy")?;
        }

        Ok(Self {
            source,
            source_semantics,
            destination_semantics,
            available_capabilities,
            resource_requirements,
            conditions,
            expansion_strategy,
            source_verified,
        })
    }
}

/// A completed legality report for one candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalityReport {
    /// Source entity analyzed.
    pub source: EntityKey,

    /// Final legality status.
    pub status: LegalityStatus,

    /// Stable diagnostic code corresponding to the status.
    pub diagnostic_code: &'static str,

    /// Human-readable explanation, when available.
    pub message: String,
}

impl LegalityReport {
    fn new(source: EntityKey, status: LegalityStatus, message: String) -> Self {
        let diagnostic_code = status.code();

        Self {
            source,
            status,
            diagnostic_code,
            message,
        }
    }
}

/// Failures in the legality-analysis contract.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LegalityError {
    /// An input contract is malformed.
    InvalidInput(String),

    /// The shared lowering context rejected an operation.
    Context(LoweringContextError),
}

impl fmt::Display for LegalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => {
                write!(f, "invalid lowering-legality input: {message}")
            }
            Self::Context(error) => {
                write!(f, "lowering context error: {error}")
            }
        }
    }
}

impl Error for LegalityError {}

impl From<LoweringContextError> for LegalityError {
    fn from(value: LoweringContextError) -> Self {
        Self::Context(value)
    }
}

/// Analyze and record lowering legality using a shared invocation context.
///
/// The function:
///
/// 1. Charges one unit of analysis work.
/// 2. Records the candidate's symbolic resource requirements.
/// 3. Checks source verification and semantic completeness.
/// 4. Checks destination schema verification.
/// 5. Compares effects, required guarantees, and capabilities.
/// 6. Records the resulting disposition and a deterministic diagnostic.
///
/// A returned `Legal` status means only that the supplied contracts justify
/// this conversion. Target resource feasibility must be checked separately.
///
/// Budget exhaustion is returned as an error. It never produces a legal
/// decision, and the context's work counter remains unchanged for a failed
/// charge.
pub fn analyze_legality(
    context: &mut LoweringContext,
    candidate: LegalityCandidate,
) -> LegalityResult<LegalityReport> {
    context.charge_work(1)?;

    for requirement in &candidate.resource_requirements {
        context.require_resource(requirement.clone());
    }

    let status = decide(context, &candidate);
    let message = describe_status(&status);

    let disposition = to_context_disposition(&status);

    context.record_disposition(candidate.source.clone(), disposition)?;

    let note = LoweringNote::new(
        status.code(),
        message.clone(),
        Some(candidate.source.clone()),
    )?;

    context.add_diagnostic(note);

    Ok(LegalityReport::new(
        candidate.source,
        status,
        message,
    ))
}

fn decide(
    context: &LoweringContext,
    candidate: &LegalityCandidate,
) -> LegalityStatus {
    if context.policy().require_verified_source && !candidate.source_verified {
        return deferred(
            "source verification is required but has not been established",
        );
    }

    let Some(source) = candidate.source_semantics.as_ref() else {
        return deferred("source operation semantics are unavailable");
    };

    if source.has_unresolved_semantics {
        return deferred("source operation has unresolved semantic facts");
    }

    let Some(destination) = candidate.destination_semantics.as_ref() else {
        if let Some(strategy) = candidate.expansion_strategy.as_ref() {
            return LegalityStatus::ExpansionRequired {
                strategy: strategy.clone(),
            };
        }

        return LegalityStatus::Unsupported {
            reason: "no destination operation or expansion strategy was supplied"
                .to_owned(),
        };
    };

    if !destination.schema_verified {
        return deferred("destination operation schema is not verified");
    }

    if destination.has_unresolved_semantics {
        return deferred("destination operation semantics are unresolved");
    }

    if !candidate.conditions.is_empty() {
        return LegalityStatus::Conditional {
            conditions: candidate.conditions.clone(),
        };
    }

    // Exact effect-set equality is the conservative default. A conversion
    // that changes effects requires a separate, explicit refinement proof;
    // a subset check alone could silently discard observable behavior.
    if source.effects != destination.effects {
        return LegalityStatus::Unsupported {
            reason: format!(
                "effect mismatch: source declares {:?}, destination declares {:?}",
                source.effects, destination.effects
            ),
        };
    }

    let missing_guarantees: Vec<String> = source
        .required_guarantees
        .difference(&destination.preserved_guarantees)
        .cloned()
        .collect();

    if !missing_guarantees.is_empty() {
        return LegalityStatus::Unsupported {
            reason: format!(
                "destination does not establish required semantic guarantees: \
                 {missing_guarantees:?}"
            ),
        };
    }

    let missing_capabilities: Vec<String> = source
        .required_capabilities
        .difference(&candidate.available_capabilities)
        .cloned()
        .collect();

    if !missing_capabilities.is_empty() {
        return LegalityStatus::Unsupported {
            reason: format!(
                "required target capabilities are unavailable: \
                 {missing_capabilities:?}"
            ),
        };
    }

    LegalityStatus::Legal
}

fn deferred(reason: &str) -> LegalityStatus {
    LegalityStatus::Deferred {
        reasons: vec![reason.to_owned()],
    }
}

fn describe_status(status: &LegalityStatus) -> String {
    match status {
        LegalityStatus::Legal => {
            "lowering is semantically legal under the supplied contracts"
                .to_owned()
        }
        LegalityStatus::Conditional { conditions } => {
            format!("lowering requires additional conditions: {conditions:?}")
        }
        LegalityStatus::ExpansionRequired { strategy } => {
            format!("lowering requires expansion strategy {strategy}")
        }
        LegalityStatus::Unsupported { reason } => {
            format!("lowering is unsupported: {reason}")
        }
        LegalityStatus::Deferred { reasons } => {
            format!("lowering legality is unresolved: {reasons:?}")
        }
    }
}

fn to_context_disposition(status: &LegalityStatus) -> LoweringDisposition {
    match status {
        LegalityStatus::Legal => LoweringDisposition::Legal,

        LegalityStatus::Conditional { conditions } => {
            LoweringDisposition::Conditional {
                conditions: conditions.clone(),
            }
        }

        LegalityStatus::ExpansionRequired { strategy } => {
            LoweringDisposition::ExpansionRequired {
                strategy: strategy.clone(),
            }
        }

        LegalityStatus::Unsupported { reason } => {
            LoweringDisposition::Unsupported {
                reason: reason.clone(),
            }
        }

        // The context contract has no Deferred variant. Record unresolved
        // evidence as conditional rather than incorrectly marking it legal.
        // The full LegalityReport retains the stronger Deferred status.
        LegalityStatus::Deferred { reasons } => {
            LoweringDisposition::Conditional {
                conditions: reasons.clone(),
            }
        }
    }
}

fn validate_name(value: &str, description: &str) -> LegalityResult<()> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(LegalityError::InvalidInput(format!(
            "{description} must be non-empty and contain no NUL characters"
        )));
    }

    Ok(())
}

fn validate_names(
    values: &BTreeSet<String>,
    description: &str,
) -> LegalityResult<()> {
    for value in values {
        validate_name(value, description)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::lowering::context::{
        EntityKind,
        LoweringPolicy,
    };

    fn source_key() -> EntityKey {
        EntityKey::new(
            EntityKind::Operation,
            "zui",
            "source-operation",
        )
        .expect("valid source identity")
    }

    fn semantics() -> OperationSemantics {
        OperationSemantics::new(
            "example.operation",
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            false,
        )
        .expect("valid semantic contract")
    }

    fn destination() -> DestinationSemantics {
        DestinationSemantics::new(
            "example.target_operation",
            BTreeSet::new(),
            BTreeSet::new(),
            false,
            true,
        )
        .expect("valid destination contract")
    }

    fn candidate(
        source_semantics: Option<OperationSemantics>,
        destination_semantics: Option<DestinationSemantics>,
        source_verified: bool,
    ) -> LegalityCandidate {
        LegalityCandidate::new(
            source_key(),
            source_semantics,
            destination_semantics,
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            None,
            source_verified,
        )
        .expect("valid candidate")
    }

    #[test]
    fn matching_verified_contracts_authorize_lowering() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let report = analyze_legality(
            &mut context,
            candidate(Some(semantics()), Some(destination()), true),
        )
        .expect("analysis succeeds");

        assert_eq!(report.status, LegalityStatus::Legal);
        assert!(report.status.permits_direct_lowering());
        assert_eq!(report.diagnostic_code, "ZUIR-LOWER-LEGAL");
    }

    #[test]
    fn unverified_source_never_authorizes_lowering() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let report = analyze_legality(
            &mut context,
            candidate(Some(semantics()), Some(destination()), false),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Deferred { .. }));
        assert!(!report.status.permits_direct_lowering());
    }

    #[test]
    fn missing_source_semantics_are_deferred() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let report = analyze_legality(
            &mut context,
            candidate(None, Some(destination()), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Deferred { .. }));
    }

    #[test]
    fn unverified_destination_schema_is_deferred() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let unverified = DestinationSemantics::new(
            "example.target_operation",
            BTreeSet::new(),
            BTreeSet::new(),
            false,
            false,
        )
        .expect("valid contract");

        let report = analyze_legality(
            &mut context,
            candidate(Some(semantics()), Some(unverified), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Deferred { .. }));
    }

    #[test]
    fn mismatched_effects_are_rejected() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let source = OperationSemantics::new(
            "example.operation",
            BTreeSet::from(["memory.write".to_owned()]),
            BTreeSet::new(),
            BTreeSet::new(),
            false,
        )
        .expect("valid source");

        let target = DestinationSemantics::new(
            "example.target_operation",
            BTreeSet::new(),
            BTreeSet::new(),
            false,
            true,
        )
        .expect("valid target");

        let report = analyze_legality(
            &mut context,
            candidate(Some(source), Some(target), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Unsupported { .. }));
    }

    #[test]
    fn missing_guarantees_are_rejected() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let source = OperationSemantics::new(
            "example.operation",
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::from(["measurement.ordering".to_owned()]),
            false,
        )
        .expect("valid source");

        let target = DestinationSemantics::new(
            "example.target_operation",
            BTreeSet::new(),
            BTreeSet::new(),
            false,
            true,
        )
        .expect("valid target");

        let report = analyze_legality(
            &mut context,
            candidate(Some(source), Some(target), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Unsupported { .. }));
    }

    #[test]
    fn unavailable_capabilities_are_rejected() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let source = OperationSemantics::new(
            "example.operation",
            BTreeSet::new(),
            BTreeSet::from(["quantum.measurement".to_owned()]),
            BTreeSet::new(),
            false,
        )
        .expect("valid source");

        let report = analyze_legality(
            &mut context,
            candidate(Some(source), Some(destination()), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Unsupported { .. }));
    }

    #[test]
    fn unresolved_semantics_are_not_treated_as_legal() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let source = OperationSemantics::new(
            "future.operation",
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            true,
        )
        .expect("valid contract");

        let report = analyze_legality(
            &mut context,
            candidate(Some(source), Some(destination()), true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Deferred { .. }));
    }

    #[test]
    fn expansion_is_distinct_from_direct_lowering() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let input = LegalityCandidate::new(
            source_key(),
            Some(semantics()),
            None,
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            Some("example.expansion".to_owned()),
            true,
        )
        .expect("valid candidate");

        let report =
            analyze_legality(&mut context, input).expect("analysis succeeds");

        assert!(matches!(
            report.status,
            LegalityStatus::ExpansionRequired { .. }
        ));
        assert!(!report.status.permits_direct_lowering());
    }

    #[test]
    fn budget_exhaustion_is_explicit() {
        let mut context = LoweringContext::new(LoweringPolicy {
            work_budget: Some(0),
            ..LoweringPolicy::default()
        });

        let result = analyze_legality(
            &mut context,
            candidate(Some(semantics()), Some(destination()), true),
        );

        assert!(matches!(
            result,
            Err(LegalityError::Context(
                LoweringContextError::WorkBudgetExceeded { .. }
            ))
        ));
        assert_eq!(context.consumed_work(), 0);
    }

    #[test]
    fn deferred_results_are_recorded_without_false_authorization() {
        let mut context = LoweringContext::new(LoweringPolicy::default());

        let report = analyze_legality(
            &mut context,
            candidate(None, None, true),
        )
        .expect("analysis succeeds");

        assert!(matches!(report.status, LegalityStatus::Deferred { .. }));
        assert!(context.has_diagnostics());
        assert!(!context.has_unsupported_entities());
        assert!(matches!(
            context.disposition(&source_key()),
            Some(LoweringDisposition::Conditional { .. })
        ));
    }
}
