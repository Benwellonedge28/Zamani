#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Resource Requirements
//!
//! Path: `src/frontend/semantic/resources/requirements.rs`
//!
//! ## PURPOSE
//!
//! Translate canonical AST resource declarations into the existing,
//! target-independent semantic resource model.
//!
//! ## STATUS
//!
//! Implementation proposed for integration with the native frontend.
//! Repository compilation and conformance tests are required before this
//! module can be declared frozen.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/ast/node/resources/resource_kind.rs`
//! - `src/frontend/semantic/resources/model.rs`
//!
//! The canonical grammar defines syntax. Normative specifications define
//! semantics. The native AST is the source-structure authority.
//!
//! ## OWNS
//!
//! - Resolving the semantic inputs needed to extract a resource requirement.
//! - Converting resource declaration nodes into canonical requirements.
//! - Preserving source order and source-node identity.
//! - Reporting extraction failures without silently accepting invalid nodes.
//! - Separating incomplete extraction from successful extraction.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing or redefining resource syntax.
//! - The canonical AST or AST node identity.
//! - The semantic resource model or resource-kind identity.
//! - Name resolution, type inference, or expression evaluation.
//! - Capability discovery or physical resource inventory.
//! - Resource allocation, target selection, routing, or scheduling.
//! - Target feasibility or backend-specific resource realization.
//!
//! ## INTEGRATION CONTRACT
//!
//! `model.rs` owns `ResourceRequirement`, `ResourceQuantity`,
//! `ResourceUnit`, and `ResourceIntent`.
//!
//! This module constructs requirements through `ResourceRequirement::from_ast`.
//! It does not define competing resource representations.
//!
//! The caller supplies a resolver that uses the canonical symbol table,
//! resource-kind registry, annotation semantics, and unit registry.
//! Resource cardinality expressions remain symbolic until the appropriate
//! expression-analysis service establishes their meaning.
//!
//! `resources/mod.rs` should expose this module after the repository-wide
//! module integration audit confirms its sibling module declarations.
//!
//! ## SCALABILITY
//!
//! This module defines no machine-capacity maximum and does not convert
//! resource cardinalities to `usize`, `u64`, or `u128`.
//!
//! Collection sizes are naturally bounded by available memory and the
//! configured analysis budget. Such operational constraints are not
//! language-level resource ceilings.
//!
//! ## DETERMINISM
//!
//! Requirements are returned in input order. No hash-map iteration order,
//! global state, random values, timestamps, or hardware probes affect results.
//!
//! ## ERROR HANDLING
//!
//! An invalid declaration, unresolved resource kind, resolver failure, or
//! invalid semantic model construction is reported explicitly.
//! Batch extraction retains independent successful results and records each
//! failed item. A partial batch must never be treated as complete analysis.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical resource model and AST APIs compile against this module.
//! - `resources/mod.rs` exports the intended API.
//! - Name resolution and annotation semantics are supplied by the real
//!   semantic context.
//! - Unit, integration, determinism, error-recovery, and scalability tests pass.
//! - Full workspace checks pass on the supported Rust toolchain.
//!
//! No `unsafe` code is permitted.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::resources::resource::Resource as AstResource;
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;

use super::model::{
    ResourceIntent,
    ResourceRequirement,
    ResourceUnit,
};

/// Resolves the semantic information required to interpret an AST resource.
///
/// Implementations should delegate to the canonical semantic context rather
/// than maintaining an independent symbol table or unit registry.
///
/// All methods must be deterministic for the same AST, configuration, and
/// semantic environment.
///
/// This trait intentionally does not expose physical resource availability:
/// that is a downstream target-feasibility concern.
pub trait ResourceRequirementResolver {
    /// Resolves the namespaced semantic kind of a resource declaration.
    ///
    /// An unresolved name must produce an error; it must not be replaced with
    /// a guessed kind or an implicitly valid default.
    fn resolve_kind(
        &self,
        resource: &AstResource,
    ) -> Result<ResourceKind, String>;

    /// Resolves an explicitly declared unit, if present.
    ///
    /// Returns `Ok(None)` when the declaration has no unit or the normative
    /// language semantics establish that no unit is required.
    ///
    /// Implementations must distinguish an absent unit from a present but
    /// invalid or unresolved unit and return an error for the latter.
    fn resolve_unit(
        &self,
        resource: &AstResource,
    ) -> Result<Option<ResourceUnit>, String>;

    /// Resolves the declaration's semantic intent.
    ///
    /// The implementation must inspect the actual AST representation and
    /// relevant validated annotations. It must not infer intent from the
    /// resource's name or from target availability.
    fn resolve_intent(
        &self,
        resource: &AstResource,
    ) -> Result<ResourceIntent, String>;
}

/// The stage at which resource requirement extraction failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequirementExtractionStage {
    /// The AST resource node failed local validation.
    AstValidation,

    /// The node is not a resource declaration.
    DeclarationKind,

    /// Resolving the semantic resource kind failed.
    ResourceKindResolution,

    /// Resolving the resource unit failed.
    UnitResolution,

    /// Resolving the declaration's intent failed.
    IntentResolution,

    /// Constructing the canonical semantic requirement failed.
    ModelConstruction,
}

impl fmt::Display for RequirementExtractionStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AstValidation => formatter.write_str("AST validation"),
            Self::DeclarationKind => formatter.write_str("declaration validation"),
            Self::ResourceKindResolution => formatter.write_str("resource-kind resolution"),
            Self::UnitResolution => formatter.write_str("resource-unit resolution"),
            Self::IntentResolution => formatter.write_str("resource-intent resolution"),
            Self::ModelConstruction => formatter.write_str("semantic model construction"),
        }
    }
}

/// A structured failure while extracting a resource requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementExtractionError {
    source_node: Option<NodeId>,
    stage: RequirementExtractionStage,
    message: String,
}

impl RequirementExtractionError {
    /// Creates an extraction error.
    #[must_use]
    pub fn new(
        source_node: Option<NodeId>,
        stage: RequirementExtractionStage,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_node,
            stage,
            message: message.into(),
        }
    }

    /// Returns the AST node associated with the error, when available.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }

    /// Returns the extraction stage that failed.
    #[must_use]
    pub const fn stage(&self) -> RequirementExtractionStage {
        self.stage
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RequirementExtractionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.source_node {
            Some(node) => write!(
                formatter,
                "resource requirement extraction failed during {} for AST node {:?}: {}",
                self.stage, node, self.message
            ),
            None => write!(
                formatter,
                "resource requirement extraction failed during {}: {}",
                self.stage, self.message
            ),
        }
    }
}

impl std::error::Error for RequirementExtractionError {}

/// The result of extracting requirements from a collection of AST resources.
///
/// A report can contain successful requirements and failures simultaneously.
/// Consumers must inspect [`Self::is_complete`] before treating extraction as
/// complete semantic analysis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequirementExtractionReport {
    requirements: Vec<ResourceRequirement>,
    errors: Vec<RequirementExtractionError>,
}

impl RequirementExtractionReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Returns extracted requirements in source/input order.
    #[must_use]
    pub fn requirements(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns extraction failures in source/input order.
    #[must_use]
    pub fn errors(&self) -> &[RequirementExtractionError] {
        &self.errors
    }

    /// Returns whether every supplied resource was extracted successfully.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns whether no resource requirements or errors were produced.
    ///
    /// An empty report is not evidence that a program has no requirements
    /// unless the caller also knows that the input collection was complete.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty() && self.errors.is_empty()
    }

    /// Returns the number of successfully extracted requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Returns the number of failed resource extractions.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Consumes the report and returns its requirements only if extraction
    /// was complete.
    ///
    /// Partial success is not silently promoted to complete success.
    pub fn into_complete(
        self,
    ) -> Result<Vec<ResourceRequirement>, Vec<RequirementExtractionError>> {
        if self.errors.is_empty() {
            Ok(self.requirements)
        } else {
            Err(self.errors)
        }
    }

    fn push_requirement(&mut self, requirement: ResourceRequirement) {
        self.requirements.push(requirement);
    }

    fn push_error(&mut self, error: RequirementExtractionError) {
        self.errors.push(error);
    }
}

/// Extracts one resource requirement from a validated canonical AST resource.
///
/// This function performs no hardware discovery and does not evaluate
/// cardinality expressions. The expression remains represented by its
/// canonical AST node identity in the semantic resource model.
///
/// # Errors
///
/// Returns a structured error if the AST node is invalid, is not a
/// declaration, cannot be resolved, or cannot be represented by the canonical
/// semantic resource model.
pub fn extract_requirement<R>(
    resource: &AstResource,
    resolver: &R,
) -> Result<ResourceRequirement, RequirementExtractionError>
where
    R: ResourceRequirementResolver + ?Sized,
{
    let source_node = Some(resource.node().id());

    resource.validate_local().map_err(|error| {
        RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::AstValidation,
            error.to_string(),
        )
    })?;

    if !resource.is_declaration() {
        return Err(RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::DeclarationKind,
            "expected a resource declaration; resource references must be resolved first",
        ));
    }

    let kind = resolver.resolve_kind(resource).map_err(|error| {
        RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::ResourceKindResolution,
            error,
        )
    })?;

    let unit = resolver.resolve_unit(resource).map_err(|error| {
        RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::UnitResolution,
            error,
        )
    })?;

    let intent = resolver.resolve_intent(resource).map_err(|error| {
        RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::IntentResolution,
            error,
        )
    })?;

    ResourceRequirement::from_ast(resource, kind, unit, intent).map_err(|error| {
        RequirementExtractionError::new(
            source_node,
            RequirementExtractionStage::ModelConstruction,
            error.to_string(),
        )
    })
}

/// Extracts requirements from an ordered collection of AST resource nodes.
///
/// Each resource is handled independently. Failures are accumulated rather
/// than suppressing later diagnostics. Both successful results and errors
/// preserve input order.
///
/// The caller must ensure that the supplied collection contains the complete
/// set of resource declarations intended for this pass. This function does
/// not traverse the AST or discover declarations by itself.
pub fn extract_requirements<'a, R, I>(
    resources: I,
    resolver: &R,
) -> RequirementExtractionReport
where
    R: ResourceRequirementResolver + ?Sized,
    I: IntoIterator<Item = &'a AstResource>,
{
    let mut report = RequirementExtractionReport::new();

    for resource in resources {
        match extract_requirement(resource, resolver) {
            Ok(requirement) => report.push_requirement(requirement),
            Err(error) => report.push_error(error),
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_stage_display_is_stable() {
        assert_eq!(
            RequirementExtractionStage::ResourceKindResolution.to_string(),
            "resource-kind resolution"
        );
        assert_eq!(
            RequirementExtractionStage::ModelConstruction.to_string(),
            "semantic model construction"
        );
    }

    #[test]
    fn empty_report_is_complete_but_not_evidence_of_empty_program() {
        let report = RequirementExtractionReport::new();

        assert!(report.is_complete());
        assert!(report.is_empty());
        assert_eq!(report.requirement_count(), 0);
        assert_eq!(report.error_count(), 0);
        assert!(report.into_complete().unwrap().is_empty());
    }

    #[test]
    fn incomplete_report_cannot_be_promoted_to_complete_success() {
        let mut report = RequirementExtractionReport::new();
        report.push_error(RequirementExtractionError::new(
            None,
            RequirementExtractionStage::ResourceKindResolution,
            "unresolved resource kind",
        ));

        assert!(!report.is_complete());
        assert_eq!(report.error_count(), 1);
        assert!(report.into_complete().is_err());
    }

    #[test]
    fn extraction_error_retains_stage_and_message() {
        let error = RequirementExtractionError::new(
            None,
            RequirementExtractionStage::UnitResolution,
            "unit is unresolved",
        );

        assert_eq!(error.stage(), RequirementExtractionStage::UnitResolution);
        assert_eq!(error.message(), "unit is unresolved");
        assert!(error.source_node().is_none());
    }
}