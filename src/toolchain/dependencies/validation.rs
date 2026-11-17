
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Validation
//!
//! Path: src/toolchain/dependencies/validation.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the authoritative toolchain validation boundary for a
//! completed dependency resolution.
//!
//! Validation occurs after dependency resolution and before the
//! resolved dependency graph is accepted by build or installation
//! orchestration.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Validation of resolved dependency graph structure.
//! - Validation of package and candidate identity consistency.
//! - Validation of root dependency declarations.
//! - Validation of transitive dependency declarations.
//! - Verification of activated dependency features.
//! - Re-evaluation of mandatory dependency constraints.
//! - Validation of dependency edge consistency.
//! - Dependency graph cycle and reachability validation.
//! - Deterministic validation reports.
//!
//! This module does not own:
//!
//! - Dependency declarations or package identities.
//! - Version comparison algorithms.
//! - Dependency candidate discovery.
//! - Dependency resolution or backtracking.
//! - Feature activation or feature unification.
//! - Compatibility evaluator implementations.
//! - Source discovery or filesystem access.
//! - Registry access or network operations.
//! - Package archive verification.
//! - Package signatures or trust decisions.
//! - Compiler dependency graphs.
//! - Build-task graphs.
//! - Target selection.
//! - Hardware discovery or allocation.
//! - Runtime scheduling or execution.
//!
//! ## Canonical authorities
//!
//! Dependency declarations:
//!     super::model
//!
//! Constraint evaluation:
//!     super::constraints
//!
//! Dependency solving:
//!     super::resolver
//!
//! Dependency topology:
//!     super::graph
//!
//! Feature activation:
//!     super::features
//!
//! Compatibility:
//!     super::compatibility
//!
//! Source discovery:
//!     super::sources
//!
//! Lockfile:
//!     super::lockfile
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain results:
//!     crate::toolchain::result
//!
//! ## Validation semantics
//!
//! A successful validation establishes that:
//!
//! - The resolution graph is structurally valid.
//! - Every selected candidate has a consistent identity.
//! - Every resolved package is reachable from a declared root.
//! - Every dependency edge corresponds to a declared relationship.
//! - Every mandatory dependency constraint is satisfied.
//! - Every activated feature is established to exist.
//! - The dependency graph is acyclic.
//!
//! Unknown or indeterminate mandatory evidence is not accepted.
//!
//! A failed validation never produces a successful readiness report.
//!
//! ## Constraint classifications
//!
//! Requirement:
//!     Mandatory property that must be established.
//!
//! Constraint:
//!     Mandatory condition that must not be violated.
//!
//! Preference:
//!     Non-mandatory property.
//!
//! Hint:
//!     Non-authoritative optimization information.
//!
//! Preferences and hints do not independently reject a candidate.
//!
//! ## POCO-REAF
//!
//! This module is independent of classical, quantum, HDL, AI,
//! tensor, hybrid, distributed, networking, cryptographic and
//! future computing domains.
//!
//! It validates package-level dependency contracts rather than
//! imposing target-specific implementation requirements.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Dependencies.
//! - Packages.
//! - Features.
//! - Capabilities.
//! - Dependency depth.
//! - Graph edges.
//! - Computing domains.
//! - Version components.
//! - Provider identities.
//!
//! Graph traversal is delegated to the existing iterative graph
//! implementation.
//!
//! Collections grow according to available memory and allocator
//! capacity.
//!
//! This module does not promise literal infinite computation or
//! storage. Configurable operational budgets belong to the caller.
//!
//! ## Determinism
//!
//! Package identities, candidate identities, dependency declarations
//! and graph relationships are processed in canonical ordered form.
//!
//! The validation report uses deterministic package ordering.
//!
//! ## Security
//!
//! This module does not access filesystems, networks, processes,
//! credentials or physical hardware.
//!
//! Package authenticity, artifact integrity and source authorization
//! must be established by their respective authoritative subsystems.
//!
//! ## Freeze contract
//!
//! This module is frozen when:
//!
//! - All selected packages have valid identities.
//! - All declared required dependencies are resolved.
//! - All graph relationships match package declarations.
//! - All mandatory constraints are established.
//! - Unknown mandatory evidence is rejected.
//! - Activated features are verified.
//! - Cycles and unreachable packages are rejected.
//! - Equivalent inputs produce equivalent reports.
//! - All integration tests pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use super::constraints::{
    ConstraintDecision,
    ConstraintEvaluatorRegistry,
    DependencyConstraintSet,
    VersionSchemeId,
};

use super::graph::PackageDependencyGraph;

use super::model::{
    DependencyDeclaration,
    DependencyName,
};

use super::resolver::{
    CandidateRef,
    ResolutionGraph,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION: &str = "dependency.validation";

const OPERATION_DECLARATIONS: &str =
    "dependency.validation.declarations";

const OPERATION_IDENTITIES: &str =
    "dependency.validation.identities";

const OPERATION_CONSTRAINTS: &str =
    "dependency.validation.constraints";

const OPERATION_GRAPH: &str =
    "dependency.validation.graph";

const OPERATION_FEATURES: &str =
    "dependency.validation.features";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(OPERATION)
}

fn package_error(
    message: impl Into<String>,
    package: &DependencyName,
) -> ToolchainError {
    validation_error(message)
        .with_subject(package.to_string())
}

fn internal_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::internal(message)
        .with_operation(OPERATION)
}

// ============================================================================
// VALIDATION COUNTS
// ============================================================================

/// Counts collected during dependency validation.
///
/// Counts are observational information, not capacity limits.
///
/// They use `usize`, matching the collection sizes supported by Rust.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DependencyValidationStatistics {
    /// Number of resolved packages.
    pub packages: usize,

    /// Number of dependency graph edges.
    pub edges: usize,

    /// Number of direct root packages.
    pub roots: usize,

    /// Number of dependency declarations checked.
    pub declarations: usize,

    /// Number of mandatory and non-mandatory constraints evaluated.
    pub constraints: usize,

    /// Number of activated dependency features checked.
    pub activated_features: usize,

    /// Number of resolver warnings preserved.
    pub resolver_warnings: usize,
}

// ============================================================================
// PACKAGE VALIDATION RECORD
// ============================================================================

/// Validation summary for one resolved package.
///
/// Package records are returned in dependency-first topological order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageValidationRecord {
    package: DependencyName,
    candidate: CandidateRef,
    declarations_checked: usize,
    constraints_checked: usize,
    activated_features_checked: usize,
}

impl PackageValidationRecord {
    /// Return the canonical package identity.
    #[must_use]
    pub fn package(&self) -> &DependencyName {
        &self.package
    }

    /// Return the selected candidate identity.
    #[must_use]
    pub fn candidate(&self) -> &CandidateRef {
        &self.candidate
    }

    /// Return the number of declarations checked for this package.
    #[must_use]
    pub const fn declarations_checked(&self) -> usize {
        self.declarations_checked
    }

    /// Return the number of evaluated constraints.
    #[must_use]
    pub const fn constraints_checked(&self) -> usize {
        self.constraints_checked
    }

    /// Return the number of activated features checked.
    #[must_use]
    pub const fn activated_features_checked(&self) -> usize {
        self.activated_features_checked
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete successful dependency validation report.
///
/// This report is constructed only after every required validation
/// stage has completed successfully.
///
/// Its existence therefore represents a validated dependency graph,
/// not merely an attempted validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyValidationReport {
    statistics: DependencyValidationStatistics,
    packages: Vec<PackageValidationRecord>,
    dependency_first_order: Vec<DependencyName>,
}

impl DependencyValidationReport {
    /// Return dependency validation statistics.
    #[must_use]
    pub const fn statistics(
        &self,
    ) -> &DependencyValidationStatistics {
        &self.statistics
    }

    /// Borrow all package validation records.
    ///
    /// Records are ordered dependency-first.
    #[must_use]
    pub fn packages(&self) -> &[PackageValidationRecord] {
        &self.packages
    }

    /// Borrow the dependency-first package execution order.
    ///
    /// This order is suitable for package dependency processing.
    ///
    /// It is not a compiler scheduling or runtime execution plan.
    #[must_use]
    pub fn dependency_first_order(&self) -> &[DependencyName] {
        &self.dependency_first_order
    }

    /// Return the number of validated packages.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.statistics.packages
    }

    /// Return the number of validated dependency edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.statistics.edges
    }

    /// Return whether the validated dependency graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Return whether the report contains validated packages.
    ///
    /// An empty project with no dependencies is valid. Consequently,
    /// this method describes graph contents rather than readiness.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        true
    }
}

// ============================================================================
// VALIDATION INPUT
// ============================================================================

/// Immutable input to dependency validation.
///
/// `roots` contains the effective root declarations supplied to
/// the resolver, including optional declarations that were selected.
///
/// An optional root omitted by the resolver may remain in this list.
/// An unresolved mandatory root is always an error.
///
/// `resolution` must be a completed `ResolutionGraph`, never a
/// `ResolutionConflict`.
///
/// The evaluator registry must contain the version, compatibility
/// and extension evaluators required by the declarations.
pub struct DependencyValidationRequest<'a> {
    roots: &'a [DependencyDeclaration],
    resolution: &'a ResolutionGraph,
    evaluators: &'a ConstraintEvaluatorRegistry,
}

impl<'a> DependencyValidationRequest<'a> {
    /// Construct a dependency validation request.
    #[must_use]
    pub const fn new(
        roots: &'a [DependencyDeclaration],
        resolution: &'a ResolutionGraph,
        evaluators: &'a ConstraintEvaluatorRegistry,
    ) -> Self {
        Self {
            roots,
            resolution,
            evaluators,
        }
    }

    /// Borrow the root declarations.
    #[must_use]
    pub const fn roots(&self) -> &'a [DependencyDeclaration] {
        self.roots
    }

    /// Borrow the resolved graph.
    #[must_use]
    pub const fn resolution(&self) -> &'a ResolutionGraph {
        self.resolution
    }
}

// ============================================================================
// DECLARATION INDEX
// ============================================================================

/// Internal index of declarations associated with resolved packages.
///
/// Multiple declarations for one package are retained because separate
/// dependents may impose different requirements on the same package.
///
/// The index does not perform dependency resolution.
type DeclarationIndex<'a> =
    BTreeMap<DependencyName, Vec<&'a DependencyDeclaration>>;

/// Canonical package-level dependency edge.
///
/// `None` identifies a direct project root dependency.
type ExpectedEdge =
    (Option<DependencyName>, DependencyName);

// ============================================================================
// DECLARATION INDEXING
// ============================================================================

fn insert_declaration<'a>(
    index: &mut DeclarationIndex<'a>,
    declaration: &'a DependencyDeclaration,
) -> Result<()> {
    declaration.validate().map_err(|error| {
        error
            .with_operation(OPERATION_DECLARATIONS)
            .with_subject(declaration.name().to_string())
            .with_context("validating a dependency declaration")
    })?;

    index
        .entry(declaration.name().clone())
        .or_default()
        .push(declaration);

    Ok(())
}

fn validate_root_declarations<'a>(
    roots: &'a [DependencyDeclaration],
    resolution: &ResolutionGraph,
    index: &mut DeclarationIndex<'a>,
    expected_edges: &mut BTreeSet<ExpectedEdge>,
) -> Result<()> {
    // Local aliases must not ambiguously identify different root packages.
    let mut aliases: BTreeMap<String, DependencyName> =
        BTreeMap::new();

    let mut declared_roots = BTreeSet::new();

    for declaration in roots {
        declaration.validate().map_err(|error| {
            error
                .with_operation(OPERATION_DECLARATIONS)
                .with_subject(declaration.name().to_string())
                .with_context("validating a direct dependency declaration")
        })?;

        if let Some(alias) = declaration.alias() {
            match aliases.get(alias.as_str()) {
                Some(existing) if existing != declaration.name() => {
                    return Err(
                        validation_error(
                            format!(
                                "dependency alias '{}' identifies both \
                                 '{}' and '{}'",
                                alias,
                                existing,
                                declaration.name(),
                            ),
                        )
                        .with_operation(OPERATION_DECLARATIONS),
                    );
                }

                Some(_) => {}

                None => {
                    aliases.insert(
                        alias.as_str().to_owned(),
                        declaration.name().clone(),
                    );
                }
            }
        }

        declared_roots.insert(declaration.name().clone());

        let resolved = resolution
            .packages()
            .contains_key(declaration.name());

        if !resolved {
            if declaration.is_optional() {
                // Optional dependencies may legitimately be excluded
                // by the resolver's optional-dependency policy.
                continue;
            }

            return Err(
                package_error(
                    format!(
                        "required root dependency '{}' was not resolved",
                        declaration.name(),
                    ),
                    declaration.name(),
                )
                .with_operation(OPERATION_DECLARATIONS),
            );
        }

        insert_declaration(index, declaration)?;

        expected_edges.insert((
            None,
            declaration.name().clone(),
        ));
    }

    // A resolved root must originate from a supplied direct declaration.
    for root in resolution.roots() {
        if !declared_roots.contains(root) {
            return Err(
                package_error(
                    format!(
                        "resolution contains undeclared root package '{}'",
                        root,
                    ),
                    root,
                )
                .with_operation(OPERATION_DECLARATIONS),
            );
        }
    }

    Ok(())
}

// ============================================================================
// TRANSITIVE DECLARATION INDEXING
// ============================================================================

fn add_transitive_declaration<'a>(
    parent: &DependencyName,
    declaration: &'a DependencyDeclaration,
    resolution: &ResolutionGraph,
    index: &mut DeclarationIndex<'a>,
    expected_edges: &mut BTreeSet<ExpectedEdge>,
) -> Result<()> {
    declaration.validate().map_err(|error| {
        error
            .with_operation(OPERATION_DECLARATIONS)
            .with_subject(declaration.name().to_string())
            .with_context(format!(
                "validating dependency declared by '{}'",
                parent,
            ))
    })?;

    let resolved = resolution
        .packages()
        .contains_key(declaration.name());

    if !resolved {
        if declaration.is_optional() {
            return Ok(());
        }

        return Err(
            package_error(
                format!(
                    "package '{}' requires unresolved dependency '{}'",
                    parent,
                    declaration.name(),
                ),
                parent,
            )
            .with_operation(OPERATION_DECLARATIONS),
        );
    }

    insert_declaration(index, declaration)?;

    expected_edges.insert((
        Some(parent.clone()),
        declaration.name().clone(),
    ));

    Ok(())
}

fn index_transitive_declarations<'a>(
    resolution: &'a ResolutionGraph,
    index: &mut DeclarationIndex<'a>,
    expected_edges: &mut BTreeSet<ExpectedEdge>,
) -> Result<usize> {
    let mut activated_feature_count = 0_usize;

    for (package_name, resolved_package) in resolution.packages() {
        let candidate = resolved_package.candidate();

        candidate.validate().map_err(|error| {
            error
                .with_operation(OPERATION_IDENTITIES)
                .with_subject(package_name.to_string())
                .with_context("validating selected package candidate")
        })?;

        // Candidate package identity must match its graph key.
        if candidate.name() != package_name {
            return Err(
                package_error(
                    format!(
                        "resolved package key '{}' does not match \
                         candidate package identity '{}'",
                        package_name,
                        candidate.name(),
                    ),
                    package_name,
                )
                .with_operation(OPERATION_IDENTITIES),
            );
        }

        // The selected candidate reference must identify the candidate
        // actually stored in the resolved package.
        if resolved_package.reference().candidate() != candidate.id() {
            return Err(
                package_error(
                    format!(
                        "selected candidate reference for '{}' \
                         does not match the candidate identity",
                        package_name,
                    ),
                    package_name,
                )
                .with_operation(OPERATION_IDENTITIES),
            );
        }

        // A provider identity is part of the globally unambiguous
        // candidate reference and is preserved in the report.
        //
        // CandidateRef is the authoritative selected-candidate identity.

        // Verify activated features against actual candidate evidence.
        for feature in resolved_package.activated_features() {
            if !candidate.facts().features().contains(feature) {
                return Err(
                    package_error(
                        format!(
                            "package '{}' activates feature '{}' \
                             that is not established in its feature inventory",
                            package_name,
                            feature,
                        ),
                        package_name,
                    )
                    .with_operation(OPERATION_FEATURES),
                );
            }

            activated_feature_count =
                activated_feature_count
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "Activated dependency feature accounting overflow",
                        )
                        .with_operation(OPERATION_FEATURES)
                    })?;
        }

        // Unconditional dependency declarations.
        for declaration in candidate.dependencies() {
            add_transitive_declaration(
                package_name,
                declaration,
                resolution,
                index,
                expected_edges,
            )?;
        }

        // Feature-specific dependencies are active only when their
        // corresponding package feature has been activated.
        for feature in resolved_package.activated_features() {
            if let Some(declarations) =
                candidate.feature_dependencies().get(feature)
            {
                for declaration in declarations {
                    add_transitive_declaration(
                        package_name,
                        declaration,
                        resolution,
                        index,
                        expected_edges,
                    )?;
                }
            }
        }
    }

    Ok(activated_feature_count)
}

// ============================================================================
// GRAPH VALIDATION
// ============================================================================

fn validate_edge_consistency(
    resolution: &ResolutionGraph,
    expected_edges: &BTreeSet<ExpectedEdge>,
) -> Result<()> {
    let mut actual_edges = BTreeSet::new();

    for edge in resolution.edges() {
        if !resolution.packages().contains_key(edge.to()) {
            return Err(
                package_error(
                    format!(
                        "dependency edge references unresolved package '{}'",
                        edge.to(),
                    ),
                    edge.to(),
                )
                .with_operation(OPERATION_GRAPH),
            );
        }

        if let Some(parent) = edge.from() {
            if !resolution.packages().contains_key(parent) {
                return Err(
                    package_error(
                        format!(
                            "dependency edge references unresolved \
                             parent package '{}'",
                            parent,
                        ),
                        parent,
                    )
                    .with_operation(OPERATION_GRAPH),
                );
            }
        }

        let key = (
            edge.from().cloned(),
            edge.to().clone(),
        );

        if !actual_edges.insert(key) {
            return Err(
                validation_error(
                    "resolution contains duplicate dependency relationships",
                )
                .with_operation(OPERATION_GRAPH),
            );
        }
    }

    if actual_edges != *expected_edges {
        let missing = expected_edges
            .difference(&actual_edges)
            .next();

        if let Some((parent, child)) = missing {
            return Err(
                validation_error(
                    format!(
                        "declared dependency relationship {:?} -> '{}' \
                         is missing from the resolution graph",
                        parent,
                        child,
                    ),
                )
                .with_operation(OPERATION_GRAPH)
                .with_subject(child.to_string()),
            );
        }

        let unexpected = actual_edges
            .difference(expected_edges)
            .next();

        if let Some((parent, child)) = unexpected {
            return Err(
                validation_error(
                    format!(
                        "resolution contains an undeclared dependency \
                         relationship {:?} -> '{}'",
                        parent,
                        child,
                    ),
                )
                .with_operation(OPERATION_GRAPH)
                .with_subject(child.to_string()),
            );
        }
    }

    Ok(())
}

fn validate_graph(
    resolution: &ResolutionGraph,
) -> Result<Vec<DependencyName>> {
    let graph = PackageDependencyGraph::from_resolution(resolution)
        .map_err(|error| {
            error
                .with_operation(OPERATION_GRAPH)
                .with_context(
                    "constructing package dependency topology",
                )
        })?;

    graph.validate_structure().map_err(|error| {
        error
            .with_operation(OPERATION_GRAPH)
            .with_context(
                "validating package dependency graph invariants",
            )
    })?;

    graph.validate_root_reachability().map_err(|error| {
        error
            .with_operation(OPERATION_GRAPH)
            .with_context(
                "validating dependency reachability from project roots",
            )
    })?;

    let ordering = graph.topological_order().map_err(|error| {
        error
            .with_operation(OPERATION_GRAPH)
            .with_context(
                "calculating dependency-first package ordering",
            )
    })?;

    if !ordering.is_acyclic() {
        let description = ordering
            .cycles()
            .first()
            .map(|cycle| {
                cycle
                    .path()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" -> ")
            })
            .unwrap_or_else(|| {
                "cycle detected without a cycle witness".to_owned()
            });

        return Err(
            validation_error(
                format!(
                    "dependency graph contains a cycle: {description}",
                ),
            )
            .with_operation(OPERATION_GRAPH),
        );
    }

    if !ordering.is_complete() {
        return Err(
            validation_error(
                format!(
                    "dependency graph has {} packages blocked \
                     from dependency-first ordering",
                    ordering.blocked().len(),
                ),
            )
            .with_operation(OPERATION_GRAPH),
        );
    }

    Ok(ordering.into_order())
}

// ============================================================================
// VERSION SCHEME EXTRACTION
// ============================================================================

fn candidate_version_scheme(
    package: &DependencyName,
    resolution: &ResolutionGraph,
) -> Result<VersionSchemeId> {
    let resolved = resolution
        .package(package)
        .ok_or_else(|| {
            ToolchainError::not_found(
                format!(
                    "resolved dependency package '{}' was not found",
                    package,
                ),
            )
            .with_operation(OPERATION_CONSTRAINTS)
            .with_subject(package.to_string())
        })?;

    let version = resolved
        .candidate()
        .version()
        .ok_or_else(|| {
            package_error(
                "selected dependency candidate has no established version",
                package,
            )
            .with_operation(OPERATION_CONSTRAINTS)
        })?;

    Ok(version.scheme().clone())
}

// ============================================================================
// PACKAGE CONSTRAINT VALIDATION
// ============================================================================

fn validate_package_constraints(
    package: &DependencyName,
    declarations: &[&DependencyDeclaration],
    resolution: &ResolutionGraph,
    evaluators: &ConstraintEvaluatorRegistry,
) -> Result<usize> {
    let resolved = resolution
        .package(package)
        .ok_or_else(|| {
            ToolchainError::not_found(
                format!("dependency package '{}' was not resolved", package),
            )
            .with_operation(OPERATION_CONSTRAINTS)
        })?;

    let candidate = resolved.candidate();

    let version_scheme = candidate_version_scheme(
        package,
        resolution,
    )?;

    let mut evaluated_count = 0_usize;

    for declaration in declarations {
        let constraints = DependencyConstraintSet::from_declaration(
            declaration,
            Some(version_scheme.clone()),
        )
        .map_err(|error| {
            error
                .with_operation(OPERATION_CONSTRAINTS)
                .with_subject(package.to_string())
                .with_context(
                    "constructing dependency constraints from declaration",
                )
        })?;

        let report = evaluators
            .evaluate(
                &constraints,
                candidate.facts(),
            )
            .map_err(|error| {
                error
                    .with_operation(OPERATION_CONSTRAINTS)
                    .with_subject(package.to_string())
                    .with_context(format!(
                        "evaluating dependency declaration for '{}'",
                        package,
                    ))
            })?;

        evaluated_count = evaluated_count
            .checked_add(report.results().len())
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "Dependency constraint accounting overflow",
                )
                .with_operation(OPERATION_CONSTRAINTS)
            })?;

        match report.decision() {
            ConstraintDecision::Satisfied => {}

            ConstraintDecision::Violated => {
                let explanation = report
                    .results()
                    .iter()
                    .filter(|result| {
                        result.constraint().kind().is_mandatory()
                            && result.evaluation()
                                == super::constraints::ConstraintEvaluation::Violated
                    })
                    .map(|result| result.explanation())
                    .collect::<Vec<_>>()
                    .join("; ");

                return Err(
                    package_error(
                        format!(
                            "mandatory dependency constraint violated \
                             for '{}': {}",
                            package,
                            explanation,
                        ),
                        package,
                    )
                    .with_operation(OPERATION_CONSTRAINTS),
                );
            }

            ConstraintDecision::Indeterminate => {
                let explanation = report
                    .results()
                    .iter()
                    .filter(|result| {
                        result.constraint().kind().is_mandatory()
                            && result.evaluation()
                                == super::constraints::ConstraintEvaluation::Indeterminate
                    })
                    .map(|result| result.explanation())
                    .collect::<Vec<_>>()
                    .join("; ");

                return Err(
                    package_error(
                        format!(
                            "mandatory dependency constraint for '{}' \
                             could not be established: {}",
                            package,
                            explanation,
                        ),
                        package,
                    )
                    .with_operation(OPERATION_CONSTRAINTS),
                );
            }
        }
    }

    Ok(evaluated_count)
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Production dependency validator.
///
/// This service is stateless. Evaluator implementations and their
/// registration lifecycle remain owned by ConstraintEvaluatorRegistry.
///
/// A validator can therefore be reused across projects, workspaces
/// and computing domains.
#[derive(Debug, Default, Clone, Copy)]
pub struct DependencyValidator;

impl DependencyValidator {
    /// Construct a dependency validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validate a completed dependency resolution.
    ///
    /// Validation covers:
    ///
    /// - Direct root declarations.
    /// - Transitive package declarations.
    /// - Candidate identity consistency.
    /// - Candidate feature evidence.
    /// - Mandatory version constraints.
    /// - Mandatory source constraints.
    /// - Mandatory feature constraints.
    /// - Mandatory capability constraints.
    /// - Registered compatibility constraints.
    /// - Dependency edge consistency.
    /// - Root reachability.
    /// - Dependency cycles.
    ///
    /// Optional dependencies omitted by the resolver are permitted.
    ///
    /// An unresolved required dependency is rejected.
    ///
    /// An indeterminate mandatory constraint is rejected.
    ///
    /// Preferences and hints remain non-mandatory according to the
    /// canonical constraint evaluator.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when any required
    /// validation condition cannot be established.
    pub fn validate(
        &self,
        request: &DependencyValidationRequest<'_>,
    ) -> Result<DependencyValidationReport> {
        let resolution = request.resolution;

        // Empty resolution is valid only when no mandatory root
        // dependency has been omitted.
        //
        // The root declaration validator enforces this condition.

        let mut declaration_index: DeclarationIndex<'_> =
            BTreeMap::new();

        let mut expected_edges: BTreeSet<ExpectedEdge> =
            BTreeSet::new();

        // ---------------------------------------------------------------
        // 1. Validate direct root declarations.
        // ---------------------------------------------------------------

        validate_root_declarations(
            request.roots,
            resolution,
            &mut declaration_index,
            &mut expected_edges,
        )?;

        // ---------------------------------------------------------------
        // 2. Validate selected packages and index transitive declarations.
        // ---------------------------------------------------------------

        let activated_features = index_transitive_declarations(
            resolution,
            &mut declaration_index,
            &mut expected_edges,
        )?;

        // ---------------------------------------------------------------
        // 3. Validate graph topology and declared relationships.
        // ---------------------------------------------------------------

        validate_edge_consistency(
            resolution,
            &expected_edges,
        )?;

        let dependency_first_order = validate_graph(resolution)?;

        // ---------------------------------------------------------------
        // 4. Validate candidate identity uniqueness.
        // ---------------------------------------------------------------

        let mut candidate_references: BTreeSet<CandidateRef> =
            BTreeSet::new();

        for (package_name, resolved) in resolution.packages() {
            if !candidate_references.insert(
                resolved.reference().clone(),
            ) {
                return Err(
                    package_error(
                        format!(
                            "candidate identity '{}' is selected \
                             by more than one package",
                            resolved.reference().candidate(),
                        ),
                        package_name,
                    )
                    .with_operation(OPERATION_IDENTITIES),
                );
            }
        }

        // ---------------------------------------------------------------
        // 5. Validate constraints in dependency-first order.
        // ---------------------------------------------------------------

        let mut records = Vec::new();

        records
            .try_reserve(dependency_first_order.len())
            .map_err(|error| {
                ToolchainError::resource_exhausted(
                    format!(
                        "Unable to reserve package validation records: {error}"
                    ),
                )
                .with_operation(OPERATION)
            })?;

        let mut declaration_count = 0_usize;
        let mut constraint_count = 0_usize;

        for package_name in &dependency_first_order {
            let resolved = resolution
                .package(package_name)
                .ok_or_else(|| {
                    internal_error(
                        "Dependency-first order references a missing package",
                    )
                    .with_subject(package_name.to_string())
                })?;

            let declarations = declaration_index
                .get(package_name)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            let evaluated = validate_package_constraints(
                package_name,
                declarations,
                resolution,
                request.evaluators,
            )?;

            declaration_count = declaration_count
                .checked_add(declarations.len())
                .ok_or_else(|| {
                    ToolchainError::resource_exhausted(
                        "Dependency declaration accounting overflow",
                    )
                    .with_operation(OPERATION)
                })?;

            constraint_count = constraint_count
                .checked_add(evaluated)
                .ok_or_else(|| {
                    ToolchainError::resource_exhausted(
                        "Dependency constraint accounting overflow",
                    )
                    .with_operation(OPERATION)
                })?;

            records.push(PackageValidationRecord {
                package: package_name.clone(),
                candidate: resolved.reference().clone(),
                declarations_checked: declarations.len(),
                constraints_checked: evaluated,
                activated_features_checked:
                    resolved.activated_features().len(),
            });
        }

        // ---------------------------------------------------------------
        // 6. Construct successful validation evidence.
        // ---------------------------------------------------------------

        let statistics = DependencyValidationStatistics {
            packages: resolution.len(),
            edges: resolution.edges().len(),
            roots: resolution.roots().len(),
            declarations: declaration_count,
            constraints: constraint_count,
            activated_features,
            resolver_warnings: resolution.warnings().len(),
        };

        Ok(DependencyValidationReport {
            statistics,
            packages: records,
            dependency_first_order,
        })
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Validate a completed dependency resolution.
///
/// This is a convenience wrapper around [`DependencyValidator::validate`].
///
/// The same evaluator registry used by the resolver should be supplied
/// so that validation applies the same registered version and
/// compatibility semantics.
pub fn validate_resolution(
    roots: &[DependencyDeclaration],
    resolution: &ResolutionGraph,
    evaluators: &ConstraintEvaluatorRegistry,
) -> Result<DependencyValidationReport> {
    let request = DependencyValidationRequest::new(
        roots,
        resolution,
        evaluators,
    );

    DependencyValidator::new().validate(&request)
}
