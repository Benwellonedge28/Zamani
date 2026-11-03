
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Pipeline Validation
//!
//! Path: src/ai/pipeline/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Coordinates validation of target-independent AI pipelines.
//!
//! This module validates:
//!
//! - pipeline and graph identity consistency;
//! - graph node and declared stage consistency;
//! - stage operation compatibility through an external semantic context;
//! - data-flow compatibility through an external semantic context;
//! - pipeline-level semantic obligations;
//! - cycle legality through an external semantic context;
//! - validation completeness.
//!
//! ## Ownership
//!
//! This module owns validation coordination and validation reporting.
//!
//! It does not own:
//!
//! - pipeline definitions;
//! - graph construction;
//! - dependency analysis algorithms;
//! - AST construction;
//! - parsing;
//! - type inference;
//! - tensor semantics;
//! - AI operation registration;
//! - universal IR verification;
//! - hardware discovery;
//! - resource allocation;
//! - device placement;
//! - scheduling;
//! - distributed execution;
//! - runtime execution.
//!
//! ## Canonical authorities
//!
//! definition.rs:
//!     Owns pipeline and stage definitions.
//!
//! graph.rs:
//!     Owns pipeline graph structure.
//!
//! dependencies.rs:
//!     Owns dependency and cycle analysis.
//!
//! frontend/semantic/domains/ai.rs:
//!     Owns AI semantic validation.
//!
//! zuir:
//!     Owns universal IR verification.
//!
//! distributed:
//!     Owns distributed placement and scheduling.
//!
//! runtime:
//!     Owns execution.
//!
//! ## Important semantic rule
//!
//! Structural validity is not equivalent to semantic validity.
//!
//! A validation context that cannot establish a required property must
//! return Incomplete, not Accepted.
//!
//! The final report is valid only when:
//!
//! - all required checks completed;
//! - no error-level diagnostic exists.
//!
//! ## Scalability
//!
//! No arbitrary language-level maximum is imposed on:
//!
//! - pipeline stages;
//! - graph nodes;
//! - graph edges;
//! - dependencies;
//! - inputs;
//! - outputs;
//! - graph depth;
//! - graph fan-in;
//! - graph fan-out;
//! - cycle component size.
//!
//! Dynamic collection capacity, process address space, allocator
//! availability, operational budgets and execution resources remain
//! finite implementation constraints.
//!
//! This module never converts resource exhaustion or incomplete analysis
//! into successful validation.
//!
//! ## POCO-REAF
//!
//! Validation operates on logical computation and semantic contracts.
//!
//! It does not depend on:
//!
//! - CPU identities;
//! - GPU identities;
//! - QPU identities;
//! - FPGA identities;
//! - hardware vendors;
//! - device counts;
//! - worker counts;
//! - physical memory capacities;
//! - physical network topology.
//!
//! ## Integration
//!
//! PipelineDefinition
//!     |
//!     v
//! PipelineGraph
//!     |
//!     v
//! DependencyAnalysis
//!     |
//!     v
//! THIS MODULE
//!     |
//!     +--> structural validation
//!     +--> stage validation
//!     +--> data-flow validation
//!     +--> cycle policy validation
//!     +--> pipeline policy validation
//!     |
//!     v
//! canonical semantic integration
//!     |
//!     v
//! ZUIR verification
//!     |
//!     v
//! optimization and lowering
//!     |
//!     v
//! distributed/runtime infrastructure

use crate::ai::pipeline::definition::{
    PipelineDefinition,
    PipelinePort,
    PipelinePortDirection,
    PipelineStageDefinition,
    PipelineStageId,
};

use crate::ai::pipeline::dependencies::{
    DependencyAnalysis,
};

use crate::ai::pipeline::graph::{
    PipelineEdge,
    PipelineEdgeKind,
    PipelineGraph,
};

use crate::ai::ids::AiPipelineId;

use std::collections::BTreeSet;
use std::fmt;

// ============================================================================
// DIAGNOSTIC SEVERITY
// ============================================================================

/// Severity of a pipeline validation diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipelineDiagnosticSeverity {
    /// Informational diagnostic.
    Note,

    /// Non-fatal condition requiring attention.
    Warning,

    /// A required validation operation could not be completed.
    Incomplete,

    /// A pipeline invariant or semantic requirement was violated.
    Error,
}

// ============================================================================
// DIAGNOSTICS
// ============================================================================

/// A deterministic diagnostic produced during pipeline validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineValidationIssue {
    code: String,
    severity: PipelineDiagnosticSeverity,
    stage: Option<PipelineStageId>,
    message: String,
}

impl PipelineValidationIssue {
    /// Return the stable diagnostic code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the diagnostic severity.
    pub const fn severity(&self) -> PipelineDiagnosticSeverity {
        self.severity
    }

    /// Return the associated stage, if any.
    pub fn stage(&self) -> Option<&PipelineStageId> {
        self.stage.as_ref()
    }

    /// Return the diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// SEMANTIC VALIDATION DECISIONS
// ============================================================================

/// Result of an individual validation obligation.
///
/// A semantic adapter must distinguish successful verification from
/// unavailable information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationDecision {
    /// The requested property was successfully verified.
    Accepted,

    /// The requested property was verified to be invalid.
    Rejected {
        /// Explanation of the violation.
        reason: String,
    },

    /// The requested property could not be established.
    ///
    /// This is not equivalent to acceptance.
    Incomplete {
        /// Explanation of why verification could not finish.
        reason: String,
    },
}

impl ValidationDecision {
    /// Construct a rejection.
    pub fn rejected(reason: impl Into<String>) -> Self {
        Self::Rejected {
            reason: reason.into(),
        }
    }

    /// Construct an incomplete decision.
    pub fn incomplete(reason: impl Into<String>) -> Self {
        Self::Incomplete {
            reason: reason.into(),
        }
    }
}

// ============================================================================
// VALIDATION CONTEXT
// ============================================================================

/// External semantic services required to validate a pipeline.
///
/// Implementations should delegate to the repository's canonical
/// frontend semantic authorities.
///
/// This trait is an integration boundary, not another semantic engine.
///
/// Default implementations return Incomplete deliberately. An unimplemented
/// semantic check must never silently become successful.
pub trait PipelineValidationContext {

    /// Validate the pipeline-level semantic contract.
    ///
    /// This can verify language-specific pipeline rules, policies,
    /// effects, requirements and obligations.
    fn validate_pipeline(
        &self,
        _definition: &PipelineDefinition,
    ) -> ValidationDecision {
        ValidationDecision::incomplete(
            "pipeline semantic validation was not supplied",
        )
    }

    /// Validate a declared stage and its referenced operation.
    ///
    /// The implementation should resolve the operation through the
    /// canonical AI operation registry or ZUIR dialect infrastructure.
    fn validate_stage(
        &self,
        _stage: &PipelineStageDefinition,
    ) -> ValidationDecision {
        ValidationDecision::incomplete(
            "stage operation validation was not supplied",
        )
    }

    /// Validate a graph edge and its associated data contracts.
    ///
    /// This method is called for every edge.
    ///
    /// For data-flow edges, the source and target stage definitions
    /// are supplied.
    ///
    /// The canonical semantic implementation is responsible for
    /// type, shape, effect and contract compatibility.
    ///
    /// This module deliberately does not assume that two opaque
    /// contract identifiers being different means they are incompatible.
    fn validate_edge(
        &self,
        _edge: &PipelineEdge,
        _source: &PipelineStageDefinition,
        _target: &PipelineStageDefinition,
    ) -> ValidationDecision {
        ValidationDecision::incomplete(
            "edge semantic validation was not supplied",
        )
    }

    /// Validate the semantic legality of a cyclic dependency component.
    ///
    /// Cycles are not automatically invalid.
    ///
    /// Feedback, iteration, recurrence and streaming semantics may
    /// legitimately require cycles.
    fn validate_cycle(
        &self,
        _component: &[PipelineStageId],
    ) -> ValidationDecision {
        ValidationDecision::incomplete(
            "cycle legality was not established by semantic analysis",
        )
    }
}

// ============================================================================
// STRUCTURAL-ONLY CONTEXT
// ============================================================================

/// Context that explicitly indicates unavailable semantic validation.
///
/// This is useful when structural checks are required before the
/// frontend semantic integration is available.
///
/// Its result can never establish complete pipeline validity.
#[derive(Clone, Copy, Debug, Default)]
pub struct StructuralOnlyContext;

impl PipelineValidationContext for StructuralOnlyContext {}

// ============================================================================
// VALIDATION OPTIONS
// ============================================================================

/// Configuration controlling pipeline validation.
///
/// These settings affect validation policy, not pipeline meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineValidationOptions {
    /// Whether an empty pipeline produces a warning.
    ///
    /// Empty pipelines are not universally invalid at this structural
    /// layer. The canonical language semantics decide their legality.
    pub warn_on_empty_pipeline: bool,
}

impl Default for PipelineValidationOptions {
    fn default() -> Self {
        Self {
            warn_on_empty_pipeline: true,
        }
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete or incomplete pipeline validation result.
///
/// The dependency analysis is retained so downstream consumers can reuse
/// the canonical dependency analysis instead of computing a duplicate.
///
/// A report is valid only when analysis is complete and no errors exist.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineValidationReport {
    pipeline_id: AiPipelineId,
    stage_count: usize,
    edge_count: usize,
    complete: bool,
    issues: Vec<PipelineValidationIssue>,
    dependency_analysis: Option<DependencyAnalysis>,
}

impl PipelineValidationReport {

    /// Return the pipeline identity.
    pub const fn pipeline_id(&self) -> AiPipelineId {
        self.pipeline_id
    }

    /// Return the number of declared stages.
    pub const fn stage_count(&self) -> usize {
        self.stage_count
    }

    /// Return the number of graph edges.
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Return whether all required validation checks completed.
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Return whether the pipeline is fully valid.
    ///
    /// Incomplete analysis is never considered valid.
    pub fn is_valid(&self) -> bool {
        self.complete && !self.has_errors()
    }

    /// Return whether an error-level diagnostic exists.
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity == PipelineDiagnosticSeverity::Error
        })
    }

    /// Return whether an incomplete diagnostic exists.
    pub fn has_incomplete_checks(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity == PipelineDiagnosticSeverity::Incomplete
        })
    }

    /// Return whether warnings exist.
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity == PipelineDiagnosticSeverity::Warning
        })
    }

    /// Return all diagnostics in deterministic order.
    pub fn issues(&self) -> &[PipelineValidationIssue] {
        &self.issues
    }

    /// Return the dependency analysis, if graph consistency allowed it.
    pub fn dependency_analysis(&self) -> Option<&DependencyAnalysis> {
        self.dependency_analysis.as_ref()
    }

    /// Return the dependency-respecting topological order, if one exists.
    ///
    /// This is a logical precedence order, not a runtime schedule.
    pub fn topological_order(&self) -> Option<&[PipelineStageId]> {
        self.dependency_analysis
            .as_ref()
            .and_then(DependencyAnalysis::topological_order)
    }

    /// Return the number of error-level diagnostics.
    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|issue| {
            issue.severity == PipelineDiagnosticSeverity::Error
        }).count()
    }

    /// Return the number of incomplete checks.
    pub fn incomplete_count(&self) -> usize {
        self.issues.iter().filter(|issue| {
            issue.severity == PipelineDiagnosticSeverity::Incomplete
        }).count()
    }
}

// ============================================================================
// VALIDATION ENGINE
// ============================================================================

/// Coordinates validation of canonical pipeline definitions and graphs.
///
/// The engine is stateless. It does not own registries, global configuration,
/// hardware resources, execution state or mutable compiler sessions.
pub struct PipelineValidator;

impl PipelineValidator {

    /// Validate a pipeline using structural checks only.
    ///
    /// Semantic checks are explicitly reported as incomplete.
    ///
    /// Production compilation should normally use
    /// `validate_with_context` with the canonical semantic adapter.
    #[must_use]
    pub fn validate_structure(
        definition: &PipelineDefinition,
        graph: &PipelineGraph,
    ) -> PipelineValidationReport {
        Self::validate_with_context(
            definition,
            graph,
            &StructuralOnlyContext,
            PipelineValidationOptions::default(),
        )
    }

    /// Validate a pipeline using supplied semantic services.
    ///
    /// The function performs structural checks first.
    ///
    /// If the definition and graph do not describe the same pipeline,
    /// semantic callbacks are not invoked on the inconsistent pair.
    ///
    /// All semantic decisions are explicit and retained in the report.
    #[must_use]
    pub fn validate_with_context<C>(
        definition: &PipelineDefinition,
        graph: &PipelineGraph,
        context: &C,
        options: PipelineValidationOptions,
    ) -> PipelineValidationReport
    where
        C: PipelineValidationContext,
    {
        let mut report = PipelineValidationReport {
            pipeline_id: definition.id(),
            stage_count: definition.stage_count(),
            edge_count: graph.edge_count(),
            complete: true,
            issues: Vec::new(),
            dependency_analysis: None,
        };

        // ------------------------------------------------------------
        // 1. Pipeline identity
        // ------------------------------------------------------------

        if definition.id() != graph.pipeline_id() {
            report.add_error(
                "AI-PIPELINE-IDENTITY-MISMATCH",
                None,
                format!(
                    "pipeline identity mismatch: definition={}, graph={}",
                    definition.id(),
                    graph.pipeline_id(),
                ),
            );

            report.complete = false;

            report.finish();

            return report;
        }

        // ------------------------------------------------------------
        // 2. Definition and graph node consistency
        // ------------------------------------------------------------

        let declared: BTreeSet<PipelineStageId> = definition
            .stages()
            .iter()
            .map(|stage| stage.id().clone())
            .collect();

        let graph_nodes: BTreeSet<PipelineStageId> =
            graph.nodes().cloned().collect();

        if declared != graph_nodes {
            report.add_error(
                "AI-PIPELINE-STAGE-SET-MISMATCH",
                None,
                "pipeline graph nodes do not exactly match declared stages",
            );

            report.complete = false;

            report.finish();

            return report;
        }

        // ------------------------------------------------------------
        // 3. Empty pipeline policy
        // ------------------------------------------------------------

        if definition.is_empty() && options.warn_on_empty_pipeline {
            report.add_warning(
                "AI-PIPELINE-EMPTY",
                None,
                "pipeline contains no declared stages",
            );
        }

        // ------------------------------------------------------------
        // 4. Canonical dependency analysis
        // ------------------------------------------------------------

        let dependencies = DependencyAnalysis::analyze(graph);

        report.dependency_analysis = Some(dependencies);

        // ------------------------------------------------------------
        // 5. Pipeline-level semantic validation
        // ------------------------------------------------------------

        report.record_decision(
            "AI-PIPELINE-SEMANTICS",
            None,
            context.validate_pipeline(definition),
        );

        // ------------------------------------------------------------
        // 6. Stage operation validation
        // ------------------------------------------------------------

        for stage in definition.stages() {
            report.record_decision(
                "AI-PIPELINE-STAGE-OPERATION",
                Some(stage.id().clone()),
                context.validate_stage(stage),
            );
        }

        // ------------------------------------------------------------
        // 7. Edge and data-flow validation
        // ------------------------------------------------------------

        for edge in graph.edges() {
            let source_id = edge.source().stage_id();
            let target_id = edge.target().stage_id();

            let source = definition.stage(source_id);
            let target = definition.stage(target_id);

            let (Some(source), Some(target)) = (source, target) else {
                // The graph implementation normally prevents this.
                // Retain defensive validation for inconsistent inputs.
                report.add_error(
                    "AI-PIPELINE-UNDECLARED-EDGE-ENDPOINT",
                    None,
                    format!(
                        "edge references an undeclared endpoint: {} -> {}",
                        source_id,
                        target_id,
                    ),
                );

                report.complete = false;

                continue;
            };

            // Validate edge structure and semantic compatibility.
            report.validate_edge_structure(edge);

            // All edge kinds are passed through the semantic adapter.
            //
            // This permits the frontend to check data-flow contracts,
            // effects, control dependencies and domain-specific rules.
            report.record_decision(
                "AI-PIPELINE-EDGE-SEMANTICS",
                Some(target.id().clone()),
                context.validate_edge(edge, source, target),
            );
        }

        // ------------------------------------------------------------
        // 8. Cycle legality
        // ------------------------------------------------------------

        if let Some(analysis) = report.dependency_analysis.as_ref() {
            for component in analysis.cyclic_components() {
                report.record_decision(
                    "AI-PIPELINE-CYCLE-SEMANTICS",
                    component.first().cloned(),
                    context.validate_cycle(component),
                );
            }
        }

        // ------------------------------------------------------------
        // 9. Finalization
        // ------------------------------------------------------------

        report.finish();

        report
    }
}

// ============================================================================
// REPORT INTERNAL OPERATIONS
// ============================================================================

impl PipelineValidationReport {

    fn add_issue(
        &mut self,
        code: impl Into<String>,
        severity: PipelineDiagnosticSeverity,
        stage: Option<PipelineStageId>,
        message: impl Into<String>,
    ) {
        self.issues.push(PipelineValidationIssue {
            code: code.into(),
            severity,
            stage,
            message: message.into(),
        });
    }

    fn add_error(
        &mut self,
        code: impl Into<String>,
        stage: Option<PipelineStageId>,
        message: impl Into<String>,
    ) {
        self.add_issue(
            code,
            PipelineDiagnosticSeverity::Error,
            stage,
            message,
        );
    }

    fn add_warning(
        &mut self,
        code: impl Into<String>,
        stage: Option<PipelineStageId>,
        message: impl Into<String>,
    ) {
        self.add_issue(
            code,
            PipelineDiagnosticSeverity::Warning,
            stage,
            message,
        );
    }

    fn record_decision(
        &mut self,
        code: &'static str,
        stage: Option<PipelineStageId>,
        decision: ValidationDecision,
    ) {
        match decision {
            ValidationDecision::Accepted => {}

            ValidationDecision::Rejected { reason } => {
                self.add_error(
                    code,
                    stage,
                    reason,
                );
            }

            ValidationDecision::Incomplete { reason } => {
                self.complete = false;

                self.add_issue(
                    code,
                    PipelineDiagnosticSeverity::Incomplete,
                    stage,
                    reason,
                );
            }
        }
    }

    fn validate_edge_structure(
        &mut self,
        edge: &PipelineEdge,
    ) {
        match edge.kind() {
            PipelineEdgeKind::DataFlow => {
                let source_port = edge.source().port_name();
                let target_port = edge.target().port_name();

                if source_port.is_none() || target_port.is_none() {
                    self.add_error(
                        "AI-PIPELINE-DATAFLOW-MISSING-PORT",
                        Some(edge.target().stage_id().clone()),
                        "data-flow edge requires source and target ports",
                    );
                }
            }

            PipelineEdgeKind::Dependency
            | PipelineEdgeKind::Control => {
                if edge.source().port_name().is_some()
                    || edge.target().port_name().is_some()
                {
                    self.add_error(
                        "AI-PIPELINE-STAGE-EDGE-HAS-PORT",
                        Some(edge.target().stage_id().clone()),
                        "dependency and control edges must use stage-only endpoints",
                    );
                }
            }
        }
    }

    fn finish(&mut self) {
        // Stable ordering makes diagnostics reproducible.
        self.issues.sort_by(|left, right| {
            (
                left.severity,
                &left.code,
                &left.stage,
                &left.message,
            )
                .cmp(&(
                    right.severity,
                    &right.code,
                    &right.stage,
                    &right.message,
                ))
        });

        // An error is a completed discovery of invalidity.
        // An incomplete check is a failure to establish validity.
        //
        // Neither may produce a successful report.
    }
}

// ============================================================================
// PUBLIC CONVENIENCE API
// ============================================================================

/// Validate a pipeline structurally.
///
/// Semantic checks remain explicitly incomplete because no canonical
/// semantic context was supplied.
#[must_use]
pub fn validate_pipeline_structure(
    definition: &PipelineDefinition,
    graph: &PipelineGraph,
) -> PipelineValidationReport {
    PipelineValidator::validate_structure(
        definition,
        graph,
    )
}

/// Validate a pipeline using canonical semantic services.
#[must_use]
pub fn validate_pipeline<C>(
    definition: &PipelineDefinition,
    graph: &PipelineGraph,
    context: &C,
) -> PipelineValidationReport
where
    C: PipelineValidationContext,
{
    PipelineValidator::validate_with_context(
        definition,
        graph,
        context,
        PipelineValidationOptions::default(),
    )
}

/// Validate a pipeline using explicit validation options.
#[must_use]
pub fn validate_pipeline_with_options<C>(
    definition: &PipelineDefinition,
    graph: &PipelineGraph,
    context: &C,
    options: PipelineValidationOptions,
) -> PipelineValidationReport
where
    C: PipelineValidationContext,
{
    PipelineValidator::validate_with_context(
        definition,
        graph,
        context,
        options,
    )
}

// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for PipelineDiagnosticSeverity {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Note => f.write_str("note"),
            Self::Warning => f.write_str("warning"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Error => f.write_str("error"),
        }
    }
}

impl fmt::Display for PipelineValidationIssue {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{} [{}]: {}",
            self.severity,
            self.code,
            self.message,
        )?;

        if let Some(stage) = &self.stage {
            write!(f, " (stage: {stage})")?;
        }

        Ok(())
    }
}

impl fmt::Display for PipelineValidationReport {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(
            f,
            "AI pipeline validation: {}",
            if self.is_valid() {
                "valid"
            } else if self.has_errors() {
                "invalid"
            } else {
                "incomplete"
            },
        )?;

        writeln!(f, "Pipeline: {}", self.pipeline_id)?;
        writeln!(f, "Stages: {}", self.stage_count)?;
        writeln!(f, "Edges: {}", self.edge_count)?;
        writeln!(f, "Complete: {}", self.complete)?;

        for issue in &self.issues {
            writeln!(f, "{issue}")?;
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::ai::pipeline::definition::{
        PipelineDefinitionBuilder,
        PipelineOperationRef,
        PipelinePort,
        PipelineStageDefinition,
    };

    use crate::ai::pipeline::graph::PipelineGraph;

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value)
            .expect("valid stage ID")
    }

    fn stage(value: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(value),
            PipelineOperationRef::new(
                "zamani.ai",
                "operation",
            )
            .expect("valid operation reference"),
        )
    }

    fn definition(
        id: u128,
        stages: Vec<PipelineStageDefinition>,
    ) -> PipelineDefinition {
        let mut builder = PipelineDefinitionBuilder::new(
            AiPipelineId::new(id).expect("valid pipeline ID"),
            "test_pipeline",
        )
        .expect("valid pipeline");

        for stage in stages {
            builder.add_stage(stage).expect("unique stage");
        }

        builder.build()
    }

    fn graph(
        definition: &PipelineDefinition,
    ) -> PipelineGraph {
        PipelineGraph::builder(definition).build()
    }

    #[derive(Clone, Copy)]
    struct AcceptAllContext;

    impl PipelineValidationContext for AcceptAllContext {

        fn validate_pipeline(
            &self,
            _definition: &PipelineDefinition,
        ) -> ValidationDecision {
            ValidationDecision::Accepted
        }

        fn validate_stage(
            &self,
            _stage: &PipelineStageDefinition,
        ) -> ValidationDecision {
            ValidationDecision::Accepted
        }

        fn validate_edge(
            &self,
            _edge: &PipelineEdge,
            _source: &PipelineStageDefinition,
            _target: &PipelineStageDefinition,
        ) -> ValidationDecision {
            ValidationDecision::Accepted
        }

        fn validate_cycle(
            &self,
            _component: &[PipelineStageId],
        ) -> ValidationDecision {
            ValidationDecision::Accepted
        }
    }

    #[test]
    fn complete_context_produces_valid_report() {
        let definition = definition(
            1,
            vec![
                stage("source"),
                stage("target"),
            ],
        );

        let graph = graph(&definition);

        let report = validate_pipeline(
            &definition,
            &graph,
            &AcceptAllContext,
        );

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert!(!report.has_errors());
    }

    #[test]
    fn structural_only_validation_never_claims_complete_validity() {
        let definition = definition(
            1,
            vec![stage("source")],
        );

        let graph = graph(&definition);

        let report = validate_pipeline_structure(
            &definition,
            &graph,
        );

        assert!(!report.is_complete());
        assert!(!report.is_valid());
        assert!(report.has_incomplete_checks());
    }

    #[test]
    fn pipeline_identity_mismatch_is_rejected() {
        let first = definition(
            1,
            vec![stage("source")],
        );

        let second = definition(
            2,
            vec![stage("source")],
        );

        let graph = graph(&first);

        let report = validate_pipeline(
            &second,
            &graph,
            &AcceptAllContext,
        );

        assert!(report.has_errors());
        assert!(!report.is_complete());
        assert!(!report.is_valid());

        assert!(
            report.issues().iter().any(|issue| {
                issue.code() == "AI-PIPELINE-IDENTITY-MISMATCH"
            })
        );
    }

    #[test]
    fn empty_pipeline_is_not_universally_rejected() {
        let definition = definition(
            1,
            Vec::new(),
        );

        let graph = graph(&definition);

        let report = validate_pipeline(
            &definition,
            &graph,
            &AcceptAllContext,
        );

        assert!(report.is_valid());
        assert!(report.has_warnings());
    }

    #[test]
    fn semantic_rejection_produces_error() {
        struct RejectContext;

        impl PipelineValidationContext for RejectContext {
            fn validate_pipeline(
                &self,
                _definition: &PipelineDefinition,
            ) -> ValidationDecision {
                ValidationDecision::rejected(
                    "pipeline violates its declared policy",
                )
            }
        }

        let definition = definition(
            1,
            vec![stage("source")],
        );

        let graph = graph(&definition);

        let report = validate_pipeline(
            &definition,
            &graph,
            &RejectContext,
        );

        assert!(report.has_errors());
        assert!(!report.is_valid());
    }

    #[test]
    fn missing_semantic_check_is_incomplete() {
        let definition = definition(
            1,
            vec![stage("source")],
        );

        let graph = graph(&definition);

        let report = validate_pipeline(
            &definition,
            &graph,
            &StructuralOnlyContext,
        );

        assert!(!report.is_complete());
        assert!(report.has_incomplete_checks());
    }

    #[test]
    fn dependency_analysis_is_reused() {
        let definition = definition(
            1,
            vec![
                stage("source"),
                stage("target"),
            ],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder.add_dependency(
            &stage_id("source"),
            &stage_id("target"),
        ).expect("valid dependency");

        let graph = builder.build();

        let report = validate_pipeline(
            &definition,
            &graph,
            &AcceptAllContext,
        );

        let analysis = report
            .dependency_analysis()
            .expect("dependency analysis exists");

        assert_eq!(analysis.stage_count(), 2);

        assert_eq!(
            report.topological_order().map(|order| {
                order.iter()
                    .map(PipelineStageId::as_str)
                    .collect::<Vec<_>>()
            }),
            Some(vec!["source", "target"]),
        );
    }

    #[test]
    fn valid_cycles_are_decided_by_semantic_context() {
        let definition = definition(
            1,
            vec![
                stage("first"),
                stage("second"),
            ],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder.add_dependency(
            &stage_id("first"),
            &stage_id("second"),
        ).expect("first dependency");

        builder.add_dependency(
            &stage_id("second"),
            &stage_id("first"),
        ).expect("second dependency");

        let graph = builder.build();

        let report = validate_pipeline(
            &definition,
            &graph,
            &AcceptAllContext,
        );

        assert!(report.is_valid());

        assert!(
            report.dependency_analysis()
                .expect("analysis exists")
                .has_cycles()
        );

        assert!(report.topological_order().is_none());
    }

    #[test]
    fn unknown_cycle_legality_is_not_accepted() {
        let definition = definition(
            1,
            vec![
                stage("first"),
                stage("second"),
            ],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder.add_dependency(
            &stage_id("first"),
            &stage_id("second"),
        ).expect("first dependency");

        builder.add_dependency(
            &stage_id("second"),
            &stage_id("first"),
        ).expect("second dependency");

        let graph = builder.build();

        let report = validate_pipeline(
            &definition,
            &graph,
            &StructuralOnlyContext,
        );

        assert!(!report.is_valid());
        assert!(report.has_incomplete_checks());
    }

    #[test]
    fn stage_validation_is_performed_for_every_stage() {
        use std::cell::Cell;

        struct CountingContext {
            count: Cell<usize>,
        }

        impl PipelineValidationContext for CountingContext {
            fn validate_pipeline(
                &self,
                _definition: &PipelineDefinition,
            ) -> ValidationDecision {
                ValidationDecision::Accepted
            }

            fn validate_stage(
                &self,
                _stage: &PipelineStageDefinition,
            ) -> ValidationDecision {
                self.count.set(self.count.get() + 1);
                ValidationDecision::Accepted
            }
        }

        let definition = definition(
            1,
            vec![
                stage("a"),
                stage("b"),
                stage("c"),
            ],
        );

        let graph = graph(&definition);

        let context = CountingContext {
            count: Cell::new(0),
        };

        let report = validate_pipeline(
            &definition,
            &graph,
            &context,
        );

        assert_eq!(context.count.get(), 3);
        assert!(report.is_valid());
    }

    #[test]
    fn incomplete_is_not_equivalent_to_warning() {
        let decision = ValidationDecision::incomplete(
            "semantic information unavailable",
        );

        assert!(
            matches!(
                decision,
                ValidationDecision::Incomplete { .. }
            )
        );
    }
}
