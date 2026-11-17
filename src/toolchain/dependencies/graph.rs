#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Dependency Graph
//!
//! Path: src/toolchain/dependencies/graph.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. Purpose
//!
//! Owns package-level dependency topology and graph analysis.
//!
//! This module provides:
//!
//! - Package graph construction.
//! - Package identity membership.
//! - Direct dependency relationships.
//! - Reverse dependent relationships.
//! - Root package tracking.
//! - Dependency-first topological ordering.
//! - Iterative strongly connected component analysis.
//! - Deterministic cycle witnesses.
//! - ResolutionGraph integration.
//! - Structural graph validation.
//! - Reachability validation.
//! - Safe graph mutation.
//! - Graph statistics.
//!
//! ## 2. Ownership
//!
//! This module owns:
//!
//! - Package dependency graph topology.
//! - Package graph adjacency indexes.
//! - Graph relationship accounting.
//! - Package graph structural invariants.
//! - Package graph traversal.
//! - Cycle analysis.
//! - Dependency-first ordering.
//!
//! This module does NOT own:
//!
//! - Package declarations.
//! - Package identity definitions.
//! - Manifest parsing.
//! - Source-language parsing.
//! - Version interpretation.
//! - Dependency candidate discovery.
//! - Dependency resolution.
//! - Candidate selection.
//! - Backtracking.
//! - Feature unification.
//! - Constraint evaluation.
//! - Lockfile serialization.
//! - Registry access.
//! - Package downloading.
//! - Package extraction.
//! - Package installation.
//! - Package signing.
//! - Compilation.
//! - Build-task graphs.
//! - Compiler dependency graphs.
//! - Runtime scheduling.
//! - Hardware discovery.
//! - Resource allocation.
//!
//! ## 3. Canonical authorities
//!
//! Package identities:
//!     super::model::DependencyName
//!
//! Package declarations:
//!     super::model::DependencyDeclaration
//!
//! Version and compatibility constraints:
//!     super::constraints
//!
//! Dependency resolution:
//!     super::resolver::DependencyResolver
//!
//! Resolved package data:
//!     super::resolver::ResolutionGraph
//!
//! Build-task dependencies:
//!     crate::toolchain::build::graph
//!
//! Compilation dependencies:
//!     crate::compiler::graph
//!
//! Source-level semantic dependencies:
//!     crate::frontend::semantic::traversal
//!
//! ## 4. Graph direction
//!
//! The canonical relationship is:
//!
//!     dependent -> dependency
//!
//! Example:
//!
//!     application -> library
//!     library     -> foundation
//!
//! Dependency-first processing order:
//!
//!     foundation
//!     library
//!     application
//!
//! A package dependency edge does not automatically imply a compiler
//! scheduling edge, runtime scheduling edge or hardware relationship.
//!
//! ## 5. Package identity
//!
//! The current resolver selects one release per canonical DependencyName
//! within a resolution graph.
//!
//! This graph preserves that existing identity contract.
//!
//! It does not use PackageCoordinate as a resolved package-instance ID:
//! PackageCoordinate preserves a requested version requirement, whereas
//! the resolver's CandidateRef identifies the selected candidate.
//!
//! Supporting simultaneous versions of the same canonical package name
//! requires a separately designed package-instance identity and a
//! coordinated resolver/lockfile migration.
//!
//! ## 6. Determinism
//!
//! Internal hash collections provide efficient membership and mutation.
//!
//! Every externally observable ordered result is explicitly sorted.
//!
//! Ready packages in topological ordering are selected using canonical
//! DependencyName ordering.
//!
//! Cycle components and cycle witnesses are deterministic.
//!
//! Hash-map iteration order is never exposed as an ordering guarantee.
//!
//! ## 7. Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Packages.
//! - Dependencies.
//! - Dependents.
//! - Graph depth.
//! - Graph nodes.
//! - Graph edges.
//! - Dependency relationships.
//! - Package-name qualification.
//! - Computing domains.
//! - Hardware resources.
//!
//! Graph traversal is iterative and does not consume one Rust call-stack
//! frame per dependency.
//!
//! Storage is O(V + E), subject to collection and identifier storage.
//!
//! Hash-based graph operations have expected constant-time membership
//! operations, with deterministic sorting performed at ordered output
//! boundaries.
//!
//! Topological ordering is O(V + E + V log V), excluding the cost of
//! ordering adjacency lists for deterministic processing.
//!
//! Strongly connected component analysis is iterative Kosaraju analysis.
//!
//! Explicit operational budgets belong to higher-level services.
//!
//! Literal infinite memory, storage or computation is not promised.
//!
//! ## 8. Allocation and failure handling
//!
//! Collections supporting fallible reservation use try_reserve.
//!
//! Detectable reservation failures are returned as ResourceExhausted.
//!
//! Semantic graph mutation is not committed until the required collection
//! reservations have succeeded.
//!
//! Rust does not provide fallible reservation for every standard-library
//! collection. Consequently, this module does not claim to recover from
//! every possible allocator failure, including allocations performed by
//! owned String cloning.
//!
//! ## 9. Cycle semantics
//!
//! Adding an edge does not silently discard a cycle.
//!
//! Cycles are retained so that analysis can provide a concrete diagnostic.
//!
//! Every reported cycle contains:
//!
//! - Its strongly connected component.
//! - One deterministic simple cycle witness.
//!
//! The witness starts and ends at the same package.
//!
//! Only one witness is returned per cyclic component. Enumerating every
//! possible simple cycle is deliberately avoided because their number
//! can grow exponentially.
//!
//! ## 10. Resolution integration
//!
//! from_resolution() constructs a validated topology snapshot from the
//! existing resolver-owned ResolutionGraph.
//!
//! It preserves the resolver's package names, direct roots and edges.
//!
//! It verifies that:
//!
//! - All edge endpoints exist.
//! - Root edges agree with the root set.
//! - Every package is reachable from a root.
//! - Forward and reverse adjacency relationships agree.
//! - Graph accounting is consistent.
//!
//! Cycles are returned by analysis rather than hidden during construction.
//!
//! ## 11. Concurrency
//!
//! The graph contains no global mutable state, locks, worker pools,
//! background threads or implicit scheduling.
//!
//! Immutable references can be shared according to normal Rust borrowing
//! and Send/Sync properties.
//!
//! Mutation requires exclusive access.
//!
//! ## 12. Serialization
//!
//! This is an in-memory graph contract.
//!
//! Lockfile serialization remains owned by dependencies::lockfile.
//!
//! Internal HashMap and HashSet representations must not become an
//! accidental persistent storage format.

use std::cmp::Reverse;
use std::collections::{
    BinaryHeap,
    HashMap,
    HashSet,
    VecDeque,
};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;

use super::model::DependencyName;
use super::resolver::ResolutionGraph;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the package dependency graph.
pub const PACKAGE_DEPENDENCY_GRAPH_API_VERSION: u16 = 1;

/// Logical schema version of the in-memory graph contract.
pub const PACKAGE_DEPENDENCY_GRAPH_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_GRAPH: &str = "dependency.graph";
const OPERATION_VALIDATION: &str = "dependency.graph.validate";
const OPERATION_TRAVERSAL: &str = "dependency.graph.traversal";
const OPERATION_CYCLE: &str = "dependency.graph.cycles";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn allocation_error(
    subject: &'static str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(
        format!(
            "Unable to reserve memory for {subject}: {error}"
        ),
    )
    .with_operation(OPERATION_GRAPH)
    .with_subject(subject)
}

fn missing_package(
    package: &DependencyName,
) -> ToolchainError {
    ToolchainError::not_found(
        format!(
            "Package '{package}' does not exist in the dependency graph"
        ),
    )
    .with_operation(OPERATION_GRAPH)
    .with_subject(package.to_string())
}

fn invalid_graph(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(OPERATION_VALIDATION)
}

fn internal_graph_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::internal(message)
        .with_operation(OPERATION_GRAPH)
}

fn count_overflow(
    subject: &'static str,
) -> ToolchainError {
    ToolchainError::resource_exhausted(
        format!(
            "The representational capacity of usize was exceeded \
             while accounting for {subject}"
        ),
    )
    .with_operation(OPERATION_GRAPH)
    .with_subject(subject)
}

// ============================================================================
// INTERNAL PACKAGE NODE
// ============================================================================

/// Internal package topology.
///
/// `dependencies` contains the packages required by this package.
///
/// `dependents` contains the packages that require this package.
///
/// Both relationships are maintained together.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PackageNode {
    dependencies: HashSet<DependencyName>,
    dependents: HashSet<DependencyName>,
    is_root: bool,
}

// ============================================================================
// PACKAGE DEPENDENCY GRAPH
// ============================================================================

/// Deterministic package dependency graph.
///
/// The graph stores each package once and maintains both directions
/// of every direct dependency relationship.
///
/// The canonical edge direction is:
///
/// `dependent -> dependency`.
///
/// Cycles are permitted during graph construction and are subsequently
/// reported by [`Self::analyze_cycles`].
///
/// This graph does not resolve package versions or select candidates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageDependencyGraph {
    nodes: HashMap<DependencyName, PackageNode>,
    edge_count: usize,
    root_count: usize,
}

impl PackageDependencyGraph {
    // ========================================================================
    // CONSTRUCTION
    // ========================================================================

    /// Construct an empty package dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a package graph from an authoritative resolution result.
    ///
    /// The resulting graph is an in-memory topology snapshot.
    ///
    /// The resolver remains the owner of selected candidates, package
    /// versions, activated features, candidate references and warnings.
    ///
    /// This method does not modify the supplied resolution graph.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - A root package is missing from the resolved package collection.
    /// - An edge references an unknown package.
    /// - A root edge disagrees with the root package collection.
    /// - A root package has no corresponding root edge.
    /// - A package cannot be reached from any root.
    /// - An internal graph invariant is violated.
    /// - A required collection reservation fails.
    pub fn from_resolution(
        resolution: &ResolutionGraph,
    ) -> ToolchainResult<Self> {
        let mut graph = Self::new();

        // First establish every canonical package identity.
        for package in resolution.packages().keys() {
            graph.add_package(package.clone())?;
        }

        // Preserve the resolver's root package identities.
        for root in resolution.roots() {
            if !resolution.packages().contains_key(root) {
                return Err(
                    invalid_graph(
                        format!(
                            "Resolution root '{root}' is absent \
                             from the resolved package collection"
                        ),
                    )
                    .with_subject(root.to_string()),
                );
            }

            graph.add_root(root.clone())?;
        }

        // Import direct package relationships.
        //
        // Root edges have no parent and therefore do not introduce
        // an artificial package node into the graph.
        for edge in resolution.edges() {
            match edge.from() {
                Some(dependent) => {
                    graph.add_dependency(
                        dependent,
                        edge.to(),
                    )?;
                }

                None => {
                    if !resolution.roots().contains(edge.to()) {
                        return Err(
                            invalid_graph(
                                format!(
                                    "Resolution contains root edge \
                                     to '{0}', but that package is not \
                                     declared as a resolution root",
                                    edge.to()
                                ),
                            )
                            .with_subject(edge.to().to_string()),
                        );
                    }
                }
            }
        }

        // Every declared root must have a corresponding root edge.
        for root in resolution.roots() {
            let has_root_edge = resolution
                .edges()
                .iter()
                .any(|edge| {
                    edge.from().is_none()
                        && edge.to() == root
                });

            if !has_root_edge {
                return Err(
                    invalid_graph(
                        format!(
                            "Resolution root '{root}' has no \
                             corresponding root dependency edge"
                        ),
                    )
                    .with_subject(root.to_string()),
                );
            }
        }

        graph.validate_structure()?;
        graph.ensure_all_packages_reachable()?;

        Ok(graph)
    }

    // ========================================================================
    // PACKAGE MEMBERSHIP
    // ========================================================================

    /// Add a package to the graph.
    ///
    /// The package is not automatically a root.
    ///
    /// Adding an existing package is idempotent and returns `false`.
    ///
    /// Returns `true` when a new package is inserted.
    ///
    /// # Errors
    ///
    /// Returns ResourceExhausted if the graph cannot reserve the
    /// required collection capacity.
    pub fn add_package(
        &mut self,
        package: DependencyName,
    ) -> ToolchainResult<bool> {
        if self.nodes.contains_key(&package) {
            return Ok(false);
        }

        self.nodes
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("package graph nodes", error)
            })?;

        self.nodes.insert(
            package,
            PackageNode::default(),
        );

        Ok(true)
    }

    /// Add a root package.
    ///
    /// A root package is a direct dependency of the project or
    /// workspace being resolved.
    ///
    /// If the package does not yet exist, it is inserted.
    ///
    /// Repeated root insertion is idempotent and returns `false`.
    ///
    /// Returns `true` when the package becomes a root.
    ///
    /// This operation does not create a synthetic root package.
    pub fn add_root(
        &mut self,
        package: DependencyName,
    ) -> ToolchainResult<bool> {
        let existing_root = self
            .nodes
            .get(&package)
            .is_some_and(|node| node.is_root);

        if existing_root {
            return Ok(false);
        }

        let next_root_count = self
            .root_count
            .checked_add(1)
            .ok_or_else(|| count_overflow("root packages"))?;

        if !self.nodes.contains_key(&package) {
            self.nodes
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error("package graph nodes", error)
                })?;

            self.nodes.insert(
                package.clone(),
                PackageNode::default(),
            );
        }

        let node = self
            .nodes
            .get_mut(&package)
            .ok_or_else(|| {
                internal_graph_error(
                    "Root package insertion did not create its node",
                )
            })?;

        node.is_root = true;
        self.root_count = next_root_count;

        Ok(true)
    }

    /// Remove root status without removing the package.
    ///
    /// Returns `false` when the package is not a root.
    pub fn remove_root(
        &mut self,
        package: &DependencyName,
    ) -> ToolchainResult<bool> {
        let Some(node) = self.nodes.get_mut(package) else {
            return Ok(false);
        };

        if !node.is_root {
            return Ok(false);
        }

        let next_root_count = self
            .root_count
            .checked_sub(1)
            .ok_or_else(|| {
                internal_graph_error(
                    "Root package accounting underflow",
                )
            })?;

        node.is_root = false;
        self.root_count = next_root_count;

        Ok(true)
    }

    /// Return whether a package exists in the graph.
    #[must_use]
    pub fn contains_package(
        &self,
        package: &DependencyName,
    ) -> bool {
        self.nodes.contains_key(package)
    }

    /// Return the number of packages in the graph.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of direct package dependency relationships.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.edge_count
    }

    /// Return the number of direct root packages.
    #[must_use]
    pub fn root_count(&self) -> usize {
        self.root_count
    }

    /// Return whether the graph has no packages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    // ========================================================================
    // EDGE INSERTION
    // ========================================================================

    /// Add a direct package dependency relationship.
    ///
    /// The relationship means:
    ///
    /// `dependent -> dependency`.
    ///
    /// Both packages must already exist.
    ///
    /// Duplicate relationships are idempotent and return `false`.
    ///
    /// Self-dependencies and multi-package cycles are retained so that
    /// cycle analysis can produce a meaningful diagnostic.
    ///
    /// The forward and reverse indexes are updated together.
    ///
    /// Required collection capacity is reserved before either index
    /// is semantically modified.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - NotFound if either package does not exist.
    /// - ResourceExhausted if collection reservation fails.
    /// - Internal if an existing graph invariant is inconsistent.
    pub fn add_dependency(
        &mut self,
        dependent: &DependencyName,
        dependency: &DependencyName,
    ) -> ToolchainResult<bool> {
        let dependent_node = self
            .nodes
            .get(dependent)
            .ok_or_else(|| missing_package(dependent))?;

        let dependency_node = self
            .nodes
            .get(dependency)
            .ok_or_else(|| missing_package(dependency))?;

        let forward_exists = dependent_node
            .dependencies
            .contains(dependency);

        let reverse_exists = dependency_node
            .dependents
            .contains(dependent);

        if forward_exists != reverse_exists {
            return Err(
                internal_graph_error(
                    "Forward and reverse dependency indexes disagree",
                )
                .with_subject(dependent.to_string()),
            );
        }

        if forward_exists {
            return Ok(false);
        }

        let next_edge_count = self
            .edge_count
            .checked_add(1)
            .ok_or_else(|| count_overflow("dependency edges"))?;

        // Reserve both sides before changing either relationship.
        self.nodes
            .get_mut(dependent)
            .ok_or_else(|| missing_package(dependent))?
            .dependencies
            .try_reserve(1)
            .map_err(|error| {
                allocation_error(
                    "package dependency relationships",
                    error,
                )
            })?;

        self.nodes
            .get_mut(dependency)
            .ok_or_else(|| missing_package(dependency))?
            .dependents
            .try_reserve(1)
            .map_err(|error| {
                allocation_error(
                    "package reverse dependency relationships",
                    error,
                )
            })?;

        let inserted_forward = self
            .nodes
            .get_mut(dependent)
            .ok_or_else(|| missing_package(dependent))?
            .dependencies
            .insert(dependency.clone());

        if !inserted_forward {
            return Err(
                internal_graph_error(
                    "Dependency insertion unexpectedly found \
                     an existing forward relationship",
                ),
            );
        }

        let inserted_reverse = self
            .nodes
            .get_mut(dependency)
            .ok_or_else(|| missing_package(dependency))?
            .dependents
            .insert(dependent.clone());

        if !inserted_reverse {
            // Restore the forward index if the reverse index violates
            // the prevalidated insertion invariant.
            if let Some(node) = self.nodes.get_mut(dependent) {
                node.dependencies.remove(dependency);
            }

            return Err(
                internal_graph_error(
                    "Reverse dependency insertion violated \
                     the graph insertion invariant",
                ),
            );
        }

        self.edge_count = next_edge_count;

        Ok(true)
    }

    // ========================================================================
    // EDGE REMOVAL
    // ========================================================================

    /// Remove a direct package dependency relationship.
    ///
    /// Both package identities must exist.
    ///
    /// Returns `false` when the relationship does not exist.
    ///
    /// Both indexes must agree before mutation.
    pub fn remove_dependency(
        &mut self,
        dependent: &DependencyName,
        dependency: &DependencyName,
    ) -> ToolchainResult<bool> {
        let dependent_node = self
            .nodes
            .get(dependent)
            .ok_or_else(|| missing_package(dependent))?;

        let dependency_node = self
            .nodes
            .get(dependency)
            .ok_or_else(|| missing_package(dependency))?;

        let forward_exists = dependent_node
            .dependencies
            .contains(dependency);

        let reverse_exists = dependency_node
            .dependents
            .contains(dependent);

        if forward_exists != reverse_exists {
            return Err(
                internal_graph_error(
                    "Cannot remove an edge because the graph indexes disagree",
                ),
            );
        }

        if !forward_exists {
            return Ok(false);
        }

        let next_edge_count = self
            .edge_count
            .checked_sub(1)
            .ok_or_else(|| {
                internal_graph_error(
                    "Dependency edge accounting underflow",
                )
            })?;

        let removed_forward = self
            .nodes
            .get_mut(dependent)
            .ok_or_else(|| missing_package(dependent))?
            .dependencies
            .remove(dependency);

        let removed_reverse = self
            .nodes
            .get_mut(dependency)
            .ok_or_else(|| missing_package(dependency))?
            .dependents
            .remove(dependent);

        if !removed_forward || !removed_reverse {
            return Err(
                internal_graph_error(
                    "Dependency removal did not update both indexes",
                ),
            );
        }

        self.edge_count = next_edge_count;

        Ok(true)
    }

    // ========================================================================
    // PACKAGE REMOVAL
    // ========================================================================

    /// Remove a package and all relationships referencing it.
    ///
    /// This removes:
    ///
    /// - Its direct dependency edges.
    /// - Its incoming dependent edges.
    /// - Its root status.
    ///
    /// All affected neighboring indexes are updated.
    ///
    /// Returns `false` when the package does not exist.
    ///
    /// The operation validates the affected relationships and accounting
    /// before changing graph contents.
    pub fn remove_package(
        &mut self,
        package: &DependencyName,
    ) -> ToolchainResult<bool> {
        let Some(removed) = self.nodes.get(package) else {
            return Ok(false);
        };

        // Prevalidate outgoing reverse relationships.
        for dependency in &removed.dependencies {
            if dependency == package {
                continue;
            }

            let Some(dependency_node) = self.nodes.get(dependency) else {
                return Err(
                    internal_graph_error(
                        "An outgoing dependency references a missing package",
                    ),
                );
            };

            if !dependency_node.dependents.contains(package) {
                return Err(
                    internal_graph_error(
                        "An outgoing dependency has no matching reverse edge",
                    ),
                );
            }
        }

        // Prevalidate incoming forward relationships.
        for dependent in &removed.dependents {
            if dependent == package {
                continue;
            }

            let Some(dependent_node) = self.nodes.get(dependent) else {
                return Err(
                    internal_graph_error(
                        "An incoming dependent references a missing package",
                    ),
                );
            };

            if !dependent_node.dependencies.contains(package) {
                return Err(
                    internal_graph_error(
                        "An incoming dependent has no matching forward edge",
                    ),
                );
            }
        }

        // A self-edge appears in both adjacency sets but represents
        // exactly one relationship.
        let has_self_edge = removed.dependencies.contains(package);

        let total_removed_edges = removed
            .dependencies
            .len()
            .checked_add(removed.dependents.len())
            .and_then(|count| {
                count.checked_sub(usize::from(has_self_edge))
            })
            .ok_or_else(|| {
                internal_graph_error(
                    "Unable to calculate removed dependency relationships",
                )
            })?;

        let next_edge_count = self
            .edge_count
            .checked_sub(total_removed_edges)
            .ok_or_else(|| {
                internal_graph_error(
                    "Package removal would underflow edge accounting",
                )
            })?;

        let next_root_count = if removed.is_root {
            Some(
                self.root_count.checked_sub(1).ok_or_else(|| {
                    internal_graph_error(
                        "Package removal would underflow root accounting",
                    )
                })?,
            )
        } else {
            None
        };

        // All validation is complete. No further allocation is needed.
        let removed = self
            .nodes
            .remove(package)
            .ok_or_else(|| {
                internal_graph_error(
                    "Package disappeared during exclusive graph mutation",
                )
            })?;

        for dependency in removed.dependencies {
            if &dependency == package {
                continue;
            }

            if let Some(node) = self.nodes.get_mut(&dependency) {
                node.dependents.remove(package);
            }
        }

        for dependent in removed.dependents {
            if &dependent == package {
                continue;
            }

            if let Some(node) = self.nodes.get_mut(&dependent) {
                node.dependencies.remove(package);
            }
        }

        self.edge_count = next_edge_count;

        if let Some(root_count) = next_root_count {
            self.root_count = root_count;
        }

        Ok(true)
    }

    // ========================================================================
    // PACKAGE QUERIES
    // ========================================================================

    /// Return all package names in canonical sorted order.
    ///
    /// Returned names are owned snapshots.
    pub fn packages(
        &self,
    ) -> ToolchainResult<Vec<DependencyName>> {
        let ordered = self.sorted_package_refs()?;

        let mut packages = Vec::new();

        packages
            .try_reserve(ordered.len())
            .map_err(|error| {
                allocation_error("ordered package results", error)
            })?;

        packages.extend(
            ordered.into_iter().cloned(),
        );

        Ok(packages)
    }

    /// Return all root package names in canonical sorted order.
    pub fn roots(
        &self,
    ) -> ToolchainResult<Vec<DependencyName>> {
        let ordered = self.sorted_package_refs()?;

        let root_capacity = self.root_count;

        let mut roots = Vec::new();

        roots
            .try_reserve(root_capacity)
            .map_err(|error| {
                allocation_error("root package results", error)
            })?;

        for package in ordered {
            let node = self.nodes.get(package).ok_or_else(|| {
                internal_graph_error(
                    "Ordered package disappeared during root query",
                )
            })?;

            if node.is_root {
                roots.push(package.clone());
            }
        }

        Ok(roots)
    }

    /// Return direct dependencies of a package in canonical order.
    ///
    /// Returns NotFound when the package is not part of the graph.
    pub fn dependencies_of(
        &self,
        package: &DependencyName,
    ) -> ToolchainResult<Vec<DependencyName>> {
        let node = self
            .nodes
            .get(package)
            .ok_or_else(|| missing_package(package))?;

        let ordered = sorted_neighbors(&node.dependencies)?;

        let mut result = Vec::new();

        result
            .try_reserve(ordered.len())
            .map_err(|error| {
                allocation_error("direct dependency results", error)
            })?;

        result.extend(
            ordered.into_iter().cloned(),
        );

        Ok(result)
    }

    /// Return packages that directly depend on the supplied package.
    ///
    /// Results are in canonical package-name order.
    ///
    /// Returns NotFound when the package is not part of the graph.
    pub fn dependents_of(
        &self,
        package: &DependencyName,
    ) -> ToolchainResult<Vec<DependencyName>> {
        let node = self
            .nodes
            .get(package)
            .ok_or_else(|| missing_package(package))?;

        let ordered = sorted_neighbors(&node.dependents)?;

        let mut result = Vec::new();

        result
            .try_reserve(ordered.len())
            .map_err(|error| {
                allocation_error("dependent package results", error)
            })?;

        result.extend(
            ordered.into_iter().cloned(),
        );

        Ok(result)
    }

    /// Return all direct dependency edges in canonical order.
    pub fn edges(
        &self,
    ) -> ToolchainResult<Vec<PackageDependencyEdge>> {
        let mut result = Vec::new();

        result
            .try_reserve(self.edge_count)
            .map_err(|error| {
                allocation_error("package dependency edge results", error)
            })?;

        for dependent in self.sorted_package_refs()? {
            let node = self.nodes.get(dependent).ok_or_else(|| {
                internal_graph_error(
                    "Ordered package disappeared during edge query",
                )
            })?;

            for dependency in sorted_neighbors(&node.dependencies)? {
                result.push(PackageDependencyEdge {
                    dependent: dependent.clone(),
                    dependency: dependency.clone(),
                });
            }
        }

        Ok(result)
    }

    // ========================================================================
    // STRUCTURAL VALIDATION
    // ========================================================================

    /// Validate graph structure, edge symmetry and accounting.
    ///
    /// This operation does not reject cycles.
    ///
    /// Cycles are a separate graph-analysis result.
    pub fn validate_structure(
        &self,
    ) -> ToolchainResult<()> {
        let mut calculated_edges = 0_usize;
        let mut calculated_roots = 0_usize;

        for (package, node) in &self.nodes {
            if node.is_root {
                calculated_roots = calculated_roots
                    .checked_add(1)
                    .ok_or_else(|| {
                        count_overflow("validated root packages")
                    })?;
            }

            // Validate dependent -> dependency relationships.
            for dependency in &node.dependencies {
                let Some(dependency_node) = self.nodes.get(dependency) else {
                    return Err(
                        invalid_graph(
                            format!(
                                "Package '{package}' references missing \
                                 dependency '{dependency}'"
                            ),
                        )
                        .with_subject(package.to_string()),
                    );
                };

                if !dependency_node.dependents.contains(package) {
                    return Err(
                        invalid_graph(
                            format!(
                                "Dependency edge '{package}' -> \
                                 '{dependency}' has no reverse relationship"
                            ),
                        )
                        .with_subject(package.to_string()),
                    );
                }

                calculated_edges = calculated_edges
                    .checked_add(1)
                    .ok_or_else(|| {
                        count_overflow("validated dependency edges")
                    })?;
            }

            // Validate dependency <- dependent relationships.
            for dependent in &node.dependents {
                let Some(dependent_node) = self.nodes.get(dependent) else {
                    return Err(
                        invalid_graph(
                            format!(
                                "Package '{package}' has missing dependent \
                                 '{dependent}'"
                            ),
                        )
                        .with_subject(package.to_string()),
                    );
                };

                if !dependent_node.dependencies.contains(package) {
                    return Err(
                        invalid_graph(
                            format!(
                                "Reverse relationship '{dependent}' -> \
                                 '{package}' has no matching forward edge"
                            ),
                        )
                        .with_subject(package.to_string()),
                    );
                }
            }
        }

        if calculated_edges != self.edge_count {
            return Err(
                invalid_graph(
                    format!(
                        "Dependency edge accounting mismatch: \
                         stored={}, calculated={calculated_edges}",
                        self.edge_count,
                    ),
                ),
            );
        }

        if calculated_roots != self.root_count {
            return Err(
                invalid_graph(
                    format!(
                        "Root package accounting mismatch: \
                         stored={}, calculated={calculated_roots}",
                        self.root_count,
                    ),
                ),
            );
        }

        Ok(())
    }

    /// Validate that every package is reachable from at least one root.
    ///
    /// This is primarily used when importing a completed resolution graph.
    ///
    /// A manually constructed partial graph may intentionally contain
    /// disconnected packages, so this method is not automatically
    /// imposed by add_package().
    pub fn validate_root_reachability(
        &self,
    ) -> ToolchainResult<()> {
        self.ensure_all_packages_reachable()
    }

    fn ensure_all_packages_reachable(
        &self,
    ) -> ToolchainResult<()> {
        if self.nodes.is_empty() {
            return Ok(());
        }

        let mut visited: HashSet<&DependencyName> = HashSet::new();

        visited
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("package reachability visited set", error)
            })?;

        let mut queue: VecDeque<&DependencyName> = VecDeque::new();

        queue
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("package reachability queue", error)
            })?;

        for (package, node) in &self.nodes {
            if node.is_root && visited.insert(package) {
                queue.push_back(package);
            }
        }

        while let Some(package) = queue.pop_front() {
            let node = self.nodes.get(package).ok_or_else(|| {
                internal_graph_error(
                    "Reachability traversal encountered a missing package",
                )
            })?;

            for dependency in &node.dependencies {
                if visited.insert(dependency) {
                    queue.push_back(dependency);
                }
            }
        }

        if visited.len() != self.nodes.len() {
            let ordered = self.sorted_package_refs()?;

            for package in ordered {
                if !visited.contains(package) {
                    return Err(
                        invalid_graph(
                            format!(
                                "Resolved package '{package}' is not \
                                 reachable from any root dependency"
                            ),
                        )
                        .with_subject(package.to_string()),
                    );
                }
            }
        }

        Ok(())
    }

    // ========================================================================
    // GRAPH STATISTICS
    // ========================================================================

    /// Return a snapshot of graph statistics.
    #[must_use]
    pub fn statistics(&self) -> PackageDependencyGraphStatistics {
        PackageDependencyGraphStatistics {
            packages: self.nodes.len(),
            dependencies: self.edge_count,
            roots: self.root_count,
            packages_with_dependencies: self
                .nodes
                .values()
                .filter(|node| !node.dependencies.is_empty())
                .count(),
            packages_with_dependents: self
                .nodes
                .values()
                .filter(|node| !node.dependents.is_empty())
                .count(),
        }
    }

    // ========================================================================
    // TOPOLOGICAL ORDERING
    // ========================================================================

    /// Produce deterministic dependency-first topological ordering.
    ///
    /// Packages with no dependencies become eligible first.
    ///
    /// If multiple packages are simultaneously eligible, the smallest
    /// canonical DependencyName is selected.
    ///
    /// When cycles exist:
    ///
    /// - `order` contains packages that can be ordered.
    /// - `blocked` contains packages that could not be emitted.
    /// - `cycles` contains the cyclic strongly connected components.
    ///
    /// Packages downstream of a cycle are also blocked.
    ///
    /// A cycle is represented in the returned result, not as an
    /// automatically successful topological ordering.
    ///
    /// # Errors
    ///
    /// Returns an error for structural inconsistency or resource
    /// reservation failure.
    pub fn topological_order(
        &self,
    ) -> ToolchainResult<TopologicalOrder> {
        self.validate_structure()?;

        let mut remaining: HashMap<&DependencyName, usize> =
            HashMap::new();

        remaining
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological dependency counts", error)
            })?;

        let mut ready: BinaryHeap<Reverse<&DependencyName>> =
            BinaryHeap::new();

        ready
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological ready queue", error)
            })?;

        for (package, node) in &self.nodes {
            remaining.insert(
                package,
                node.dependencies.len(),
            );

            if node.dependencies.is_empty() {
                ready.push(Reverse(package));
            }
        }

        let mut order = Vec::new();

        order
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological order", error)
            })?;

        // Kahn's algorithm.
        //
        // The number of unresolved prerequisites is tracked for each
        // dependent package.
        //
        // Processing a dependency decreases the count of its dependents.
        while let Some(Reverse(dependency)) = ready.pop() {
            if remaining.remove(dependency).is_none() {
                return Err(
                    internal_graph_error(
                        "Topological traversal emitted a package twice \
                         or encountered an unknown package",
                    )
                    .with_subject(dependency.to_string()),
                );
            }

            order.push(dependency.clone());

            let dependency_node = self.nodes.get(dependency).ok_or_else(|| {
                internal_graph_error(
                    "Topological traversal encountered a missing node",
                )
            })?;

            let dependents = sorted_neighbors(
                &dependency_node.dependents,
            )?;

            for dependent in dependents {
                let unresolved = remaining.get_mut(dependent).ok_or_else(|| {
                    internal_graph_error(
                        "A dependent was emitted before its prerequisite",
                    )
                    .with_subject(dependent.to_string())
                })?;

                if *unresolved == 0 {
                    return Err(
                        internal_graph_error(
                            "Topological prerequisite accounting underflow",
                        )
                        .with_subject(dependent.to_string()),
                    );
                }

                *unresolved -= 1;

                if *unresolved == 0 {
                    ready.push(Reverse(dependent));
                }
            }
        }

        // Packages still in `remaining` are cyclic or depend on a cycle.
        let mut blocked = Vec::new();

        blocked
            .try_reserve(remaining.len())
            .map_err(|error| {
                allocation_error("blocked package results", error)
            })?;

        for package in self.sorted_package_refs()? {
            if remaining.contains_key(package) {
                blocked.push(package.clone());
            }
        }

        let cycles = if blocked.is_empty() {
            Vec::new()
        } else {
            self.analyze_cycles()?.into_cycles()
        };

        Ok(TopologicalOrder {
            order,
            blocked,
            cycles,
        })
    }

    // ========================================================================
    // STRONGLY CONNECTED COMPONENTS AND CYCLES
    // ========================================================================

    /// Analyze all cyclic strongly connected components.
    ///
    /// Uses iterative Kosaraju analysis.
    ///
    /// The first pass follows dependency relationships.
    ///
    /// The second pass follows reverse dependent relationships.
    ///
    /// No graph-depth-dependent recursion is used.
    ///
    /// Every cyclic component receives one deterministic cycle witness.
    pub fn analyze_cycles(
        &self,
    ) -> ToolchainResult<CycleAnalysis> {
        self.validate_structure()?;

        let packages = self.sorted_package_refs()?;

        let mut visited: HashSet<&DependencyName> = HashSet::new();

        visited
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("cycle analysis visited set", error)
            })?;

        let mut finish_order: Vec<&DependencyName> = Vec::new();

        finish_order
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("cycle analysis finish order", error)
            })?;

        // --------------------------------------------------------------------
        // FIRST PASS
        // --------------------------------------------------------------------
        //
        // Iterative DFS following:
        //
        //     dependent -> dependency
        //
        // Finish order is produced without recursive function calls.

        for root in packages.iter().copied() {
            if !visited.insert(root) {
                continue;
            }

            let neighbors = self.sorted_dependencies(root)?;

            let mut stack = Vec::new();

            stack
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error("cycle analysis DFS stack", error)
                })?;

            stack.push(DfsFrame {
                node: root,
                neighbors,
                next: 0,
            });

            while !stack.is_empty() {
                let next_neighbor = {
                    let frame = stack.last_mut().ok_or_else(|| {
                        internal_graph_error(
                            "Cycle analysis DFS stack unexpectedly became empty",
                        )
                    })?;

                    if frame.next < frame.neighbors.len() {
                        let neighbor = frame.neighbors[frame.next];

                        frame.next += 1;

                        Some(neighbor)
                    } else {
                        None
                    }
                };

                if let Some(neighbor) = next_neighbor {
                    if visited.insert(neighbor) {
                        let neighbors = self.sorted_dependencies(neighbor)?;

                        stack
                            .try_reserve(1)
                            .map_err(|error| {
                                allocation_error(
                                    "cycle analysis DFS stack",
                                    error,
                                )
                            })?;

                        stack.push(DfsFrame {
                            node: neighbor,
                            neighbors,
                            next: 0,
                        });
                    }
                } else {
                    let finished = stack.pop().ok_or_else(|| {
                        internal_graph_error(
                            "Unable to complete cycle analysis DFS frame",
                        )
                    })?;

                    finish_order.push(finished.node);
                }
            }
        }

        // --------------------------------------------------------------------
        // SECOND PASS
        // --------------------------------------------------------------------
        //
        // Reverse the first-pass finish order and traverse:
        //
        //     dependency <- dependent
        //
        // Every resulting component is a strongly connected component.

        visited.clear();

        let mut cycles = Vec::new();

        for root in finish_order.iter().rev().copied() {
            if !visited.insert(root) {
                continue;
            }

            let mut component: Vec<&DependencyName> = Vec::new();

            let mut stack: Vec<&DependencyName> = Vec::new();

            stack
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error("strongly connected component stack", error)
                })?;

            stack.push(root);

            while let Some(package) = stack.pop() {
                component
                    .try_reserve(1)
                    .map_err(|error| {
                        allocation_error(
                            "strongly connected component members",
                            error,
                        )
                    })?;

                component.push(package);

                let node = self.nodes.get(package).ok_or_else(|| {
                    internal_graph_error(
                        "SCC traversal encountered a missing package",
                    )
                })?;

                for dependent in &node.dependents {
                    if visited.insert(dependent) {
                        stack
                            .try_reserve(1)
                            .map_err(|error| {
                                allocation_error(
                                    "strongly connected component stack",
                                    error,
                                )
                            })?;

                        stack.push(dependent);
                    }
                }
            }

            component.sort_unstable();

            let is_cyclic = if component.len() > 1 {
                true
            } else {
                let package = component.first().ok_or_else(|| {
                    internal_graph_error(
                        "An SCC unexpectedly contains no packages",
                    )
                })?;

                self.nodes
                    .get(*package)
                    .is_some_and(|node| {
                        node.dependencies.contains(*package)
                    })
            };

            if !is_cyclic {
                continue;
            }

            let path = self.find_cycle_path(&component)?;

            let mut members = Vec::new();

            members
                .try_reserve(component.len())
                .map_err(|error| {
                    allocation_error("cycle component results", error)
                })?;

            members.extend(
                component.iter().map(|package| (*package).clone()),
            );

            cycles
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error("cycle analysis results", error)
                })?;

            cycles.push(DependencyCycle {
                members,
                path,
            });
        }

        // Hash-based internal storage must never determine report order.
        cycles.sort_by(|left, right| {
            left.members.cmp(&right.members)
        });

        Ok(CycleAnalysis { cycles })
    }

    // ========================================================================
    // INTERNAL TRAVERSAL HELPERS
    // ========================================================================

    fn sorted_package_refs(
        &self,
    ) -> ToolchainResult<Vec<&DependencyName>> {
        let mut packages = Vec::new();

        packages
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("ordered package references", error)
            })?;

        packages.extend(self.nodes.keys());

        packages.sort_unstable();

        Ok(packages)
    }

    fn sorted_dependencies<'a>(
        &'a self,
        package: &'a DependencyName,
    ) -> ToolchainResult<Vec<&'a DependencyName>> {
        let node = self.nodes.get(package).ok_or_else(|| {
            missing_package(package)
        })?;

        sorted_neighbors(&node.dependencies)
    }

    fn find_cycle_path(
        &self,
        component: &[&DependencyName],
    ) -> ToolchainResult<Vec<DependencyName>> {
        let start = *component.first().ok_or_else(|| {
            internal_graph_error(
                "Cannot construct a cycle path from an empty component",
            )
        })?;

        let start_node = self.nodes.get(start).ok_or_else(|| {
            missing_package(start)
        })?;

        // A self-dependency is already a complete simple cycle.
        if start_node.dependencies.contains(start) {
            let mut path = Vec::new();

            path.try_reserve(2)
                .map_err(|error| {
                    allocation_error("cycle witness", error)
                })?;

            path.push(start.clone());
            path.push(start.clone());

            return Ok(path);
        }

        let mut members: HashSet<&DependencyName> = HashSet::new();

        members
            .try_reserve(component.len())
            .map_err(|error| {
                allocation_error("cycle component membership", error)
            })?;

        members.extend(component.iter().copied());

        let mut visited: HashSet<&DependencyName> = HashSet::new();

        visited
            .try_reserve(component.len())
            .map_err(|error| {
                allocation_error("cycle witness visited set", error)
            })?;

        visited.insert(start);

        let path_capacity = component
            .len()
            .checked_add(1)
            .ok_or_else(|| count_overflow("cycle witness length"))?;

        let mut path: Vec<&DependencyName> = Vec::new();

        path.try_reserve(path_capacity)
            .map_err(|error| {
                allocation_error("cycle witness path", error)
            })?;

        path.push(start);

        let first_neighbors = self.sorted_dependencies(start)?;

        let mut stack = Vec::new();

        stack
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("cycle witness DFS stack", error)
            })?;

        stack.push(DfsFrame {
            node: start,
            neighbors: first_neighbors,
            next: 0,
        });

        // The search is restricted to the strongly connected component.
        //
        // Because every member of a cyclic SCC can reach the starting
        // package, an edge returning to start must eventually be found.
        //
        // Globally visited vertices avoid repeatedly exploring completed
        // subgraphs.
        while !stack.is_empty() {
            let next_neighbor = {
                let frame = stack.last_mut().ok_or_else(|| {
                    internal_graph_error(
                        "Cycle witness DFS stack unexpectedly became empty",
                    )
                })?;

                if frame.next < frame.neighbors.len() {
                    let neighbor = frame.neighbors[frame.next];

                    frame.next += 1;

                    Some(neighbor)
                } else {
                    None
                }
            };

            if let Some(neighbor) = next_neighbor {
                if neighbor == start {
                    let mut result = Vec::new();

                    result
                        .try_reserve(path.len() + 1)
                        .map_err(|error| {
                            allocation_error("cycle witness result", error)
                        })?;

                    result.extend(
                        path.iter().map(|package| (*package).clone()),
                    );

                    result.push(start.clone());

                    return Ok(result);
                }

                if members.contains(neighbor)
                    && visited.insert(neighbor)
                {
                    path
                        .try_reserve(1)
                        .map_err(|error| {
                            allocation_error("cycle witness path", error)
                        })?;

                    path.push(neighbor);

                    let neighbors =
                        self.sorted_dependencies(neighbor)?;

                    stack
                        .try_reserve(1)
                        .map_err(|error| {
                            allocation_error("cycle witness DFS stack", error)
                        })?;

                    stack.push(DfsFrame {
                        node: neighbor,
                        neighbors,
                        next: 0,
                    });
                }
            } else {
                stack.pop();

                let _ = path.pop();
            }
        }

        Err(
            internal_graph_error(
                "A cyclic strongly connected component did not \
                 produce a cycle witness",
            )
            .with_subject(start.to_string()),
        )
    }
}

// ============================================================================
// ORDERED ADJACENCY HELPER
// ============================================================================

fn sorted_neighbors(
    values: &HashSet<DependencyName>,
) -> ToolchainResult<Vec<&DependencyName>> {
    let mut result = Vec::new();

    result
        .try_reserve(values.len())
        .map_err(|error| {
            allocation_error("ordered adjacency results", error)
        })?;

    result.extend(values.iter());

    result.sort_unstable();

    Ok(result)
}

// ============================================================================
// DFS FRAME
// ============================================================================

/// One frame in an iterative depth-first traversal.
struct DfsFrame<'a> {
    node: &'a DependencyName,
    neighbors: Vec<&'a DependencyName>,
    next: usize,
}

// ============================================================================
// PACKAGE DEPENDENCY EDGE
// ============================================================================

/// One direct package dependency relationship.
///
/// The direction is:
///
/// `dependent -> dependency`.
///
/// This is a topology relationship, not a compiler scheduling instruction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageDependencyEdge {
    dependent: DependencyName,
    dependency: DependencyName,
}

impl PackageDependencyEdge {
    /// Borrow the dependent package.
    #[must_use]
    pub fn dependent(&self) -> &DependencyName {
        &self.dependent
    }

    /// Borrow the required dependency package.
    #[must_use]
    pub fn dependency(&self) -> &DependencyName {
        &self.dependency
    }
}

// ============================================================================
// CYCLE WITNESS
// ============================================================================

/// One cyclic strongly connected package component.
///
/// `members` contains all package names belonging to the component.
///
/// `path` contains one deterministic simple cycle witness.
///
/// The first and final elements of `path` are identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCycle {
    members: Vec<DependencyName>,
    path: Vec<DependencyName>,
}

impl DependencyCycle {
    /// Borrow the sorted members of this cyclic component.
    #[must_use]
    pub fn members(&self) -> &[DependencyName] {
        &self.members
    }

    /// Borrow the deterministic cycle witness.
    ///
    /// The first package is repeated as the final package.
    #[must_use]
    pub fn path(&self) -> &[DependencyName] {
        &self.path
    }
}

// ============================================================================
// CYCLE ANALYSIS
// ============================================================================

/// Complete result of package dependency cycle analysis.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CycleAnalysis {
    cycles: Vec<DependencyCycle>,
}

impl CycleAnalysis {
    /// Return whether the graph is acyclic.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.cycles.is_empty()
    }

    /// Borrow all cyclic strongly connected components.
    ///
    /// Components are returned in deterministic order.
    #[must_use]
    pub fn cycles(&self) -> &[DependencyCycle] {
        &self.cycles
    }

    /// Return the number of cyclic strongly connected components.
    #[must_use]
    pub fn cycle_count(&self) -> usize {
        self.cycles.len()
    }

    /// Consume the analysis and return its cycle components.
    #[must_use]
    pub fn into_cycles(self) -> Vec<DependencyCycle> {
        self.cycles
    }
}

// ============================================================================
// TOPOLOGICAL ORDER
// ============================================================================

/// Dependency-first topological ordering result.
///
/// `order` contains packages that can be processed in dependency-first order.
///
/// `blocked` contains packages that could not be emitted because of
/// cycles or dependencies on cyclic packages.
///
/// `cycles` contains the actual cyclic components.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TopologicalOrder {
    order: Vec<DependencyName>,
    blocked: Vec<DependencyName>,
    cycles: Vec<DependencyCycle>,
}

impl TopologicalOrder {
    /// Borrow the dependency-first package order.
    #[must_use]
    pub fn order(&self) -> &[DependencyName] {
        &self.order
    }

    /// Borrow packages blocked from topological emission.
    #[must_use]
    pub fn blocked(&self) -> &[DependencyName] {
        &self.blocked
    }

    /// Borrow cyclic components.
    #[must_use]
    pub fn cycles(&self) -> &[DependencyCycle] {
        &self.cycles
    }

    /// Return whether every package was topologically ordered.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.blocked.is_empty()
    }

    /// Return whether the ordering contains no cycles.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.cycles.is_empty()
    }

    /// Consume the result and return the ordered packages.
    ///
    /// This method does not discard the fact that packages may have
    /// been blocked. Call `is_complete()` before using this method
    /// as a complete dependency order.
    #[must_use]
    pub fn into_order(self) -> Vec<DependencyName> {
        self.order
    }
}

// ============================================================================
// GRAPH STATISTICS
// ============================================================================

/// Snapshot of package dependency graph statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PackageDependencyGraphStatistics {
    /// Number of packages.
    pub packages: usize,

    /// Number of unique direct dependency relationships.
    pub dependencies: usize,

    /// Number of root packages.
    pub roots: usize,

    /// Number of packages with at least one dependency.
    pub packages_with_dependencies: usize,

    /// Number of packages with at least one dependent.
    pub packages_with_dependents: usize,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn package(value: &str) -> DependencyName {
        DependencyName::new(value)
            .expect("test package name must be valid")
    }

    fn add_package(
        graph: &mut PackageDependencyGraph,
        value: &str,
    ) {
        graph
            .add_package(package(value))
            .expect("package insertion must succeed");
    }

    fn add_edge(
        graph: &mut PackageDependencyGraph,
        dependent: &str,
        dependency: &str,
    ) {
        graph
            .add_dependency(
                &package(dependent),
                &package(dependency),
            )
            .expect("dependency insertion must succeed");
    }

    fn graph_with_packages(
        names: &[&str],
    ) -> PackageDependencyGraph {
        let mut graph = PackageDependencyGraph::new();

        for name in names {
            add_package(&mut graph, name);
        }

        graph
    }

    #[test]
    fn api_versions_are_nonzero() {
        assert_ne!(
            PACKAGE_DEPENDENCY_GRAPH_API_VERSION,
            0
        );

        assert_ne!(
            PACKAGE_DEPENDENCY_GRAPH_SCHEMA_VERSION,
            0
        );
    }

    #[test]
    fn empty_graph_is_valid_and_acyclic() {
        let graph = PackageDependencyGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.package_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert_eq!(graph.root_count(), 0);

        graph
            .validate_structure()
            .expect("empty graph must be structurally valid");

        let order = graph
            .topological_order()
            .expect("empty graph must be orderable");

        assert!(order.is_complete());
        assert!(order.is_acyclic());
        assert!(order.order().is_empty());
        assert!(order.blocked().is_empty());

        let cycles = graph
            .analyze_cycles()
            .expect("empty graph cycle analysis must succeed");

        assert!(cycles.is_acyclic());
    }

    #[test]
    fn package_insertion_is_idempotent() {
        let mut graph = PackageDependencyGraph::new();

        assert!(graph.add_package(package("foundation")).unwrap());
        assert!(!graph.add_package(package("foundation")).unwrap());

        assert_eq!(graph.package_count(), 1);
    }

    #[test]
    fn roots_are_distinct_from_ordinary_packages() {
        let mut graph = PackageDependencyGraph::new();

        graph.add_package(package("library")).unwrap();

        assert_eq!(graph.root_count(), 0);

        assert!(graph.add_root(package("library")).unwrap());
        assert!(!graph.add_root(package("library")).unwrap());

        assert_eq!(graph.root_count(), 1);

        assert_eq!(
            graph.roots().unwrap(),
            vec![package("library")]
        );

        assert!(graph.remove_root(&package("library")).unwrap());

        assert_eq!(graph.root_count(), 0);

        assert!(graph.contains_package(&package("library")));
    }

    #[test]
    fn dependency_first_order_is_correct() {
        let mut graph = graph_with_packages(&[
            "application",
            "library",
            "foundation",
        ]);

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "library", "foundation");

        let result = graph.topological_order().unwrap();

        assert!(result.is_complete());

        assert_eq!(
            result.order(),
            &[
                package("foundation"),
                package("library"),
                package("application"),
            ]
        );
    }

    #[test]
    fn independent_packages_have_deterministic_order() {
        let mut first = graph_with_packages(&[
            "zeta",
            "alpha",
            "middle",
        ]);

        let second = graph_with_packages(&[
            "middle",
            "zeta",
            "alpha",
        ]);

        assert_eq!(
            first.topological_order().unwrap().order(),
            second.topological_order().unwrap().order()
        );

        assert_eq!(
            first.topological_order().unwrap().order(),
            &[
                package("alpha"),
                package("middle"),
                package("zeta"),
            ]
        );
    }

    #[test]
    fn duplicate_dependency_edges_are_idempotent() {
        let mut graph = graph_with_packages(&[
            "application",
            "library",
        ]);

        assert!(
            graph
                .add_dependency(
                    &package("application"),
                    &package("library"),
                )
                .unwrap()
        );

        assert!(
            !graph
                .add_dependency(
                    &package("application"),
                    &package("library"),
                )
                .unwrap()
        );

        assert_eq!(graph.dependency_count(), 1);

        graph
            .validate_structure()
            .expect("both indexes must agree");
    }

    #[test]
    fn reverse_relationships_are_maintained() {
        let mut graph = graph_with_packages(&[
            "application",
            "library",
            "plugin",
        ]);

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "plugin", "library");

        assert_eq!(
            graph.dependents_of(&package("library")).unwrap(),
            vec![
                package("application"),
                package("plugin"),
            ]
        );

        assert_eq!(
            graph.dependencies_of(&package("application")).unwrap(),
            vec![package("library")]
        );
    }

    #[test]
    fn unknown_packages_are_rejected_when_adding_edges() {
        let mut graph = PackageDependencyGraph::new();

        add_package(&mut graph, "application");

        assert!(
            graph
                .add_dependency(
                    &package("application"),
                    &package("missing"),
                )
                .is_err()
        );

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn self_dependency_is_reported_as_a_cycle() {
        let mut graph = graph_with_packages(&["alias"]);

        add_edge(&mut graph, "alias", "alias");

        let analysis = graph.analyze_cycles().unwrap();

        assert_eq!(analysis.cycle_count(), 1);

        assert_eq!(
            analysis.cycles()[0].path(),
            &[package("alias"), package("alias")]
        );

        let order = graph.topological_order().unwrap();

        assert!(!order.is_complete());
        assert_eq!(order.blocked(), &[package("alias")]);
    }

    #[test]
    fn multi_package_cycle_has_a_concrete_witness() {
        let mut graph = graph_with_packages(&[
            "alpha",
            "beta",
            "gamma",
        ]);

        add_edge(&mut graph, "alpha", "beta");
        add_edge(&mut graph, "beta", "gamma");
        add_edge(&mut graph, "gamma", "alpha");

        let analysis = graph.analyze_cycles().unwrap();

        assert_eq!(analysis.cycle_count(), 1);

        let cycle = &analysis.cycles()[0];

        assert_eq!(
            cycle.members(),
            &[
                package("alpha"),
                package("beta"),
                package("gamma"),
            ]
        );

        let path = cycle.path();

        assert_eq!(path.first(), path.last());
        assert_eq!(path.len(), 4);

        for pair in path.windows(2) {
            assert!(
                graph
                    .dependencies_of(&pair[0])
                    .unwrap()
                    .contains(&pair[1])
            );
        }
    }

    #[test]
    fn separate_cycles_are_reported_deterministically() {
        let mut graph = graph_with_packages(&[
            "a",
            "b",
            "x",
            "y",
        ]);

        add_edge(&mut graph, "a", "b");
        add_edge(&mut graph, "b", "a");

        add_edge(&mut graph, "x", "y");
        add_edge(&mut graph, "y", "x");

        let analysis = graph.analyze_cycles().unwrap();

        assert_eq!(analysis.cycle_count(), 2);

        assert_eq!(
            analysis.cycles()[0].members(),
            &[package("a"), package("b")]
        );

        assert_eq!(
            analysis.cycles()[1].members(),
            &[package("x"), package("y")]
        );
    }

    #[test]
    fn packages_downstream_of_cycles_are_blocked() {
        let mut graph = graph_with_packages(&[
            "isolated",
            "a",
            "b",
            "consumer",
        ]);

        add_edge(&mut graph, "a", "b");
        add_edge(&mut graph, "b", "a");
        add_edge(&mut graph, "consumer", "a");

        let order = graph.topological_order().unwrap();

        assert_eq!(
            order.order(),
            &[package("isolated")]
        );

        assert_eq!(
            order.blocked(),
            &[
                package("a"),
                package("b"),
                package("consumer"),
            ]
        );

        assert_eq!(order.cycles().len(), 1);
    }

    #[test]
    fn removing_an_edge_preserves_other_relationships() {
        let mut graph = graph_with_packages(&[
            "application",
            "library",
            "foundation",
        ]);

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "application", "foundation");

        assert!(
            graph
                .remove_dependency(
                    &package("application"),
                    &package("library"),
                )
                .unwrap()
        );

        assert_eq!(graph.dependency_count(), 1);

        assert_eq!(
            graph.dependencies_of(&package("application")).unwrap(),
            vec![package("foundation")]
        );

        graph
            .validate_structure()
            .expect("remaining relationships must be consistent");
    }

    #[test]
    fn removing_a_package_removes_incoming_and_outgoing_edges() {
        let mut graph = graph_with_packages(&[
            "application",
            "library",
            "foundation",
        ]);

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "library", "foundation");

        assert!(graph.remove_package(&package("library")).unwrap());

        assert_eq!(graph.package_count(), 2);
        assert_eq!(graph.dependency_count(), 0);

        assert!(
            graph
                .dependencies_of(&package("application"))
                .unwrap()
                .is_empty()
        );

        assert!(
            graph
                .dependents_of(&package("foundation"))
                .unwrap()
                .is_empty()
        );

        graph
            .validate_structure()
            .expect("package removal must preserve graph invariants");
    }

    #[test]
    fn removing_a_self_dependent_package_counts_one_edge() {
        let mut graph = graph_with_packages(&["self"]);

        add_edge(&mut graph, "self", "self");

        assert_eq!(graph.dependency_count(), 1);

        assert!(graph.remove_package(&package("self")).unwrap());

        assert_eq!(graph.package_count(), 0);
        assert_eq!(graph.dependency_count(), 0);

        graph
            .validate_structure()
            .expect("self-edge removal must preserve accounting");
    }

    #[test]
    fn removing_a_root_package_updates_root_accounting() {
        let mut graph = PackageDependencyGraph::new();

        graph.add_root(package("application")).unwrap();

        assert_eq!(graph.root_count(), 1);

        graph.remove_package(&package("application")).unwrap();

        assert_eq!(graph.root_count(), 0);
        assert_eq!(graph.package_count(), 0);
    }

    #[test]
    fn disconnected_packages_fail_root_reachability_validation() {
        let mut graph = PackageDependencyGraph::new();

        graph.add_root(package("application")).unwrap();
        add_package(&mut graph, "unreachable");

        assert!(
            graph.validate_root_reachability().is_err()
        );
    }

    #[test]
    fn all_transitive_packages_are_reachable_from_roots() {
        let mut graph = PackageDependencyGraph::new();

        graph.add_root(package("application")).unwrap();

        add_package(&mut graph, "library");
        add_package(&mut graph, "foundation");

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "library", "foundation");

        graph
            .validate_root_reachability()
            .expect("all packages must be reachable");
    }

    #[test]
    fn graph_statistics_are_consistent() {
        let mut graph = PackageDependencyGraph::new();

        graph.add_root(package("application")).unwrap();
        add_package(&mut graph, "library");
        add_package(&mut graph, "foundation");

        add_edge(&mut graph, "application", "library");
        add_edge(&mut graph, "library", "foundation");

        assert_eq!(
            graph.statistics(),
            PackageDependencyGraphStatistics {
                packages: 3,
                dependencies: 2,
                roots: 1,
                packages_with_dependencies: 2,
                packages_with_dependents: 2,
            }
        );
    }

    #[test]
    fn graph_edge_results_are_canonically_sorted() {
        let mut graph = graph_with_packages(&[
            "consumer",
            "zeta",
            "alpha",
        ]);

        add_edge(&mut graph, "consumer", "zeta");
        add_edge(&mut graph, "consumer", "alpha");

        let edges = graph.edges().unwrap();

        assert_eq!(
            edges,
            vec![
                PackageDependencyEdge {
                    dependent: package("consumer"),
                    dependency: package("alpha"),
                },
                PackageDependencyEdge {
                    dependent: package("consumer"),
                    dependency: package("zeta"),
                },
            ]
        );
    }

    #[test]
    fn long_dependency_chain_uses_iterative_algorithms() {
        let count = 20_000_usize;

        let mut graph = PackageDependencyGraph::new();

        let mut names = Vec::new();

        names
            .try_reserve(count)
            .expect("test package-name reservation");

        for index in 0..count {
            let name = package(
                &format!("package-{index:05}"),
            );

            graph
                .add_package(name.clone())
                .expect("package insertion");

            names.push(name);
        }

        for index in 1..count {
            graph
                .add_dependency(
                    &names[index],
                    &names[index - 1],
                )
                .expect("dependency insertion");
        }

        let order = graph
            .topological_order()
            .expect("long chain ordering must succeed");

        assert!(order.is_complete());

        assert_eq!(order.order().len(), count);

        assert_eq!(
            order.order().first(),
            names.first()
        );

        assert_eq!(
            order.order().last(),
            names.last()
        );

        assert!(
            graph
                .analyze_cycles()
                .unwrap()
                .is_acyclic()
        );
    }
}