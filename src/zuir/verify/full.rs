
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Full-Program Verification
//!
//! Path: src/zuir/verify/full.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Orchestrate the canonical ZUIR verification stages over one complete,
//! immutable verification view. Aggregate deterministic, machine-readable
//! findings and distinguish invalid IR from incomplete verification.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ir/
//! - src/zuir/verify/structural.rs
//! - src/zuir/verify/control_flow.rs
//! - src/zuir/verify/dominance.rs
//! - src/zuir/verify/types.rs
//! - src/zuir/verify/effects.rs
//! - src/zuir/verify/ownership.rs
//! - src/zuir/verify/resources.rs
//! - src/zuir/verify/dialects.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//! - grammar/specification/semantic-model.md
//!
//! OWNS
//!
//! - The public full-verification input contract.
//! - Verification-stage orchestration and stage ordering.
//! - Aggregation of findings from the existing verification modules.
//! - Overall verification status and deterministic report ordering.
//!
//! DOES NOT OWN
//!
//! - IR storage, mutation, or identifier allocation.
//! - Individual structural, type, CFG, dominance, effect, ownership,
//!   resource, or dialect verification algorithms.
//! - Source parsing, grammar conformance, AST construction, or name
//!   resolution.
//! - Target discovery, resource allocation, routing, scheduling,
//!   code generation, or runtime execution.
//! - A proof of arbitrary extension implementation correctness.
//!
//! SCALABILITY
//!
//! No fixed hardware, operation-count, qubit-count, tensor-rank, device,
//! processor, node, or topology ceiling is imposed. All input collections
//! are dynamically sized. Verification is bounded by actual memory,
//! address space, execution time, and any budgets enforced by the caller.
//!
//! SAFETY
//!
//! This module contains no unsafe code and does not mutate the supplied IR.
//!
//! IMPORTANT COMPLETENESS BOUNDARY
//!
//! Running dominance analysis is not equivalent to verifying SSA use-def
//! legality. Likewise, a clean report from this orchestrator proves only
//! the properties covered by the stages actually invoked. A dedicated SSA
//! verifier, when integrated, must be invoked here before this module may
//! claim complete SSA validation.
//!
//! FREEZE CRITERIA
//!
//! - All inputs are explicit; no global mutable registry is consulted.
//! - Each existing verifier remains the sole owner of its algorithm.
//! - Missing verification context cannot silently authorize execution.
//! - Errors, deferred findings, and warnings remain distinguishable.
//! - Findings have stable codes and deterministic ordering.
//! - No hardware-specific limits or unsafe code are introduced.

use std::collections::BTreeMap;

use crate::zuir::dialect::operation::OperationDefinition;
use crate::zuir::dialect::registry::DialectRegistry;
use crate::zuir::ids::TypeId;
use crate::zuir::ir::{
    EffectDefinition, OwnershipIndex, ZuirBlock, ZuirFunction, ZuirModule,
    ZuirOperation, ZuirProgram, ZuirRegion,
};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::TypeDef;

use super::control_flow::{
    verify_control_flow, ControlFlowInput,
};
use super::dialects::{
    verify_dialects, DialectIssueSeverity,
};
use super::dominance::{
    verify_dominance, DominanceInput,
};
use super::effects::{
    verify_effects, EffectVerificationInput,
};
use super::ownership::{
    verify_ownership, OwnershipVerificationInput,
};
use super::resources::{
    verify_resources, ResourceIntentEntry, ResourceIssueSeverity,
};
use super::structural::{
    verify_structural, StructuralInput,
};
use super::types::{
    verify_types, TypeVerificationInput,
};

/// The severity of an aggregated verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerificationSeverity {
    /// A known invariant or semantic contract is violated.
    Error,

    /// The finding is informative but does not itself invalidate the IR.
    Warning,

    /// Verification could not establish the required property.
    Deferred,
}

/// The stage that produced a verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerificationStage {
    Structural,
    ControlFlow,
    Dominance,
    Types,
    Effects,
    Ownership,
    Resources,
    Dialects,
}

/// Overall result of a full-verification run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    /// All invoked verification stages passed, and no stage reported
    /// incomplete verification.
    Valid,

    /// At least one invoked stage proved an invariant violation.
    Invalid,

    /// No known violation was found, but one or more required properties
    /// could not be established.
    Incomplete,
}

/// One stable, machine-readable verification finding.
///
/// `code` is the programmatic interface. Consumers must not parse `message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationIssue {
    stage: VerificationStage,
    severity: VerificationSeverity,
    code: String,
    message: String,
}

impl VerificationIssue {
    fn new(
        stage: VerificationStage,
        severity: VerificationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            stage,
            severity,
            code: code.into(),
            message: message.into(),
        }
    }

    /// Return the stage that produced this finding.
    #[must_use]
    pub const fn stage(&self) -> VerificationStage {
        self.stage
    }

    /// Return its severity.
    #[must_use]
    pub const fn severity(&self) -> VerificationSeverity {
        self.severity
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return its explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Complete context required by the currently implemented verification
/// stages.
///
/// Every slice must describe the same IR snapshot. The caller is responsible
/// for supplying all regions, blocks, operations, types, effects, ownership
/// contracts, resource intents, dialect manifests, and operation schemas
/// relevant to the program.
///
/// This type borrows its inputs and never mutates them.
pub struct FullVerificationInput<'a> {
    /// Canonical top-level program and function hierarchy.
    pub program: &'a ZuirProgram<ZuirModule<ZuirFunction>>,

    /// Every region belonging to the verification snapshot.
    pub regions: &'a [ZuirRegion],

    /// Every block belonging to the verification snapshot.
    pub blocks: &'a [ZuirBlock],

    /// Every operation belonging to the verification snapshot.
    pub operations: &'a [ZuirOperation],

    /// Canonical type definitions indexed by TypeId.
    pub type_definitions: &'a BTreeMap<TypeId, TypeDef>,

    /// Registered effect definitions indexed by qualified name.
    pub effect_definitions: &'a BTreeMap<QualifiedName, EffectDefinition>,

    /// Canonical ownership contracts and recorded ownership events.
    pub ownership: &'a OwnershipIndex,

    /// Resource intents and their caller-defined logical scopes.
    pub resource_intents: &'a [ResourceIntentEntry<'a>],

    /// Registered dialect manifests.
    pub dialect_registry: &'a DialectRegistry,

    /// Operation schemas available for dialect verification.
    pub operation_schemas: &'a [OperationDefinition],
}

/// Aggregate report for a complete orchestration run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FullVerificationReport {
    issues: Vec<VerificationIssue>,
    stages_completed: Vec<VerificationStage>,
}

impl FullVerificationReport {
    /// Return the overall verification status.
    ///
    /// Invalid takes precedence over incomplete: a known violation remains
    /// a violation even when another stage is deferred.
    #[must_use]
    pub fn status(&self) -> VerificationStatus {
        if self
            .issues
            .iter()
            .any(|issue| issue.severity == VerificationSeverity::Error)
        {
            VerificationStatus::Invalid
        } else if self
            .issues
            .iter()
            .any(|issue| issue.severity == VerificationSeverity::Deferred)
        {
            VerificationStatus::Incomplete
        } else {
            VerificationStatus::Valid
        }
    }

    /// Return true only when every invoked stage passed without deferred
    /// findings or errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status() == VerificationStatus::Valid
    }

    /// Return true if a known violation was found.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == VerificationSeverity::Error)
    }

    /// Return true if at least one property could not be established.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == VerificationSeverity::Deferred)
    }

    /// Borrow all aggregated findings in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[VerificationIssue] {
        &self.issues
    }

    /// Return the stages that completed execution.
    #[must_use]
    pub fn stages_completed(&self) -> &[VerificationStage] {
        &self.stages_completed
    }

    /// Count findings with the requested severity.
    #[must_use]
    pub fn count(&self, severity: VerificationSeverity) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == severity)
            .count()
    }

    fn push(
        &mut self,
        stage: VerificationStage,
        severity: VerificationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.issues.push(VerificationIssue::new(
            stage, severity, code, message,
        ));
    }

    fn complete_stage(&mut self, stage: VerificationStage) {
        self.stages_completed.push(stage);
    }

    fn sort_findings(&mut self) {
        self.issues.sort_by(|left, right| {
            (
                left.stage,
                left.severity,
                &left.code,
                &left.message,
            )
                .cmp(&(
                    right.stage,
                    right.severity,
                    &right.code,
                    &right.message,
                ))
        });
    }
}

/// Verify the supplied program snapshot using all currently integrated
/// verification stages.
///
/// Stages are executed in a fixed order. A failure in one stage does not
/// suppress independent diagnostics from later stages; each stage is
/// responsible for validating the references it consumes.
///
/// # Stage order
///
/// 1. Structural integrity.
/// 2. Explicit control-flow integrity.
/// 3. Dominance and reachability analysis.
/// 4. Semantic type verification.
/// 5. Effect-contract verification.
/// 6. Ownership-contract verification.
/// 7. Resource-intent verification.
/// 8. Dialect and operation-schema verification.
///
/// # Important
///
/// This function does not establish target feasibility, physical resource
/// availability, complete interprocedural effect correctness, arbitrary
/// extension correctness, or SSA legality unless those properties are
/// independently implemented and invoked by an integrated verifier.
pub fn verify_full(
    input: &FullVerificationInput<'_>,
) -> FullVerificationReport {
    let mut report = FullVerificationReport::default();

    // 1. Structural integrity.
    let structural = verify_structural(&StructuralInput {
        program: input.program,
        regions: input.regions,
        blocks: input.blocks,
        operations: input.operations,
    });

    for issue in structural.issues() {
        report.push(
            VerificationStage::Structural,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Structural);

    // 2. Control-flow integrity.
    let control_flow = verify_control_flow(&ControlFlowInput {
        regions: input.regions,
        blocks: input.blocks,
    });

    for issue in control_flow.issues() {
        report.push(
            VerificationStage::ControlFlow,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::ControlFlow);

    // 3. Dominance and reachability analysis.
    //
    // Dominance analysis does not, by itself, establish SSA use-def legality.
    let dominance = verify_dominance(&DominanceInput {
        regions: input.regions,
        blocks: input.blocks,
    });

    for issue in dominance.issues() {
        report.push(
            VerificationStage::Dominance,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Dominance);

    // 4. Type definitions and references.
    let types = verify_types(&TypeVerificationInput {
        program: input.program,
        regions: input.regions,
        blocks: input.blocks,
        operations: input.operations,
        type_definitions: input.type_definitions,
    });

    for issue in types.issues() {
        report.push(
            VerificationStage::Types,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Types);

    // 5. Effects.
    let effects = verify_effects(&EffectVerificationInput {
        program: input.program,
        regions: input.regions,
        blocks: input.blocks,
        operations: input.operations,
        effect_definitions: input.effect_definitions,
    });

    for issue in effects.issues() {
        report.push(
            VerificationStage::Effects,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Effects);

    // 6. Ownership.
    let ownership = verify_ownership(&OwnershipVerificationInput {
        ownership: input.ownership,
        regions: input.regions,
        blocks: input.blocks,
        operations: input.operations,
    });

    for issue in ownership.issues() {
        report.push(
            VerificationStage::Ownership,
            VerificationSeverity::Error,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Ownership);

    // 7. Resource intents.
    let resources = verify_resources(input.resource_intents);

    for issue in resources.issues() {
        let severity = match issue.severity() {
            ResourceIssueSeverity::Error => VerificationSeverity::Error,
            ResourceIssueSeverity::Warning => VerificationSeverity::Warning,
            ResourceIssueSeverity::Deferred => VerificationSeverity::Deferred,
        };

        report.push(
            VerificationStage::Resources,
            severity,
            issue.code(),
            format!("{}: {}", issue.scope(), issue.message()),
        );
    }
    report.complete_stage(VerificationStage::Resources);

    // 8. Dialect manifests and operation schemas.
    let dialects = verify_dialects(
        input.operations,
        input.dialect_registry,
        input.operation_schemas,
    );

    for issue in dialects.issues() {
        let severity = match issue.severity() {
            DialectIssueSeverity::Error => VerificationSeverity::Error,
            DialectIssueSeverity::Deferred => VerificationSeverity::Deferred,
        };

        report.push(
            VerificationStage::Dialects,
            severity,
            issue.code(),
            issue.message(),
        );
    }
    report.complete_stage(VerificationStage::Dialects);

    // Canonical ordering makes the aggregate report independent of the
    // order in which individual stages happened to append their findings.
    report.sort_findings();

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_report_is_valid_but_has_no_completed_stages() {
        let report = FullVerificationReport::default();

        assert_eq!(report.status(), VerificationStatus::Valid);
        assert!(report.is_valid());
        assert!(!report.has_errors());
        assert!(!report.is_incomplete());
        assert!(report.issues().is_empty());
        assert!(report.stages_completed().is_empty());
    }

    #[test]
    fn errors_take_precedence_over_deferred_findings() {
        let mut report = FullVerificationReport::default();

        report.push(
            VerificationStage::Structural,
            VerificationSeverity::Deferred,
            "TEST-DEFERRED",
            "verification context is incomplete",
        );

        report.push(
            VerificationStage::Types,
            VerificationSeverity::Error,
            "TEST-INVALID",
            "a required type is unresolved",
        );

        assert_eq!(report.status(), VerificationStatus::Invalid);
        assert!(report.has_errors());
        assert!(report.is_incomplete());
    }

    #[test]
    fn deferred_findings_never_count_as_valid() {
        let mut report = FullVerificationReport::default();

        report.push(
            VerificationStage::Dialects,
            VerificationSeverity::Deferred,
            "TEST-DEFERRED",
            "extension semantics are unavailable",
        );

        assert_eq!(report.status(), VerificationStatus::Incomplete);
        assert!(!report.is_valid());
        assert!(!report.has_errors());
        assert!(report.is_incomplete());
    }

    #[test]
    fn warning_only_report_is_not_invalid() {
        let mut report = FullVerificationReport::default();

        report.push(
            VerificationStage::Resources,
            VerificationSeverity::Warning,
            "TEST-WARNING",
            "a non-mandatory resource preference could not be applied",
        );

        assert_eq!(report.status(), VerificationStatus::Valid);
        assert!(report.is_valid());
        assert_eq!(report.count(VerificationSeverity::Warning), 1);
    }

    #[test]
    fn finding_order_is_deterministic() {
        let mut report = FullVerificationReport::default();

        report.push(
            VerificationStage::Types,
            VerificationSeverity::Error,
            "Z",
            "second",
        );
        report.push(
            VerificationStage::Structural,
            VerificationSeverity::Error,
            "A",
            "first",
        );
        report.push(
            VerificationStage::Types,
            VerificationSeverity::Error,
            "A",
            "third",
        );

        report.sort_findings();

        assert_eq!(report.issues()[0].stage(), VerificationStage::Structural);
        assert_eq!(report.issues()[1].code(), "A");
        assert_eq!(report.issues()[2].code(), "Z");
    }
}
