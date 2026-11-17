
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Resolver
//!
//! Path: `src/toolchain/dependencies/resolver.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Resolves normalized package dependency declarations into a deterministic
//! package-resolution graph.
//!
//! The resolver coordinates:
//!
//! - Canonical dependency declarations from `super::model`.
//! - Constraint evaluation from `super::constraints`.
//! - Candidate discovery through registered provider implementations.
//! - Transitive dependency expansion.
//! - Feature unification.
//! - Candidate selection and backtracking.
//! - Optional dependency policy.
//! - Lock selection.
//! - Dependency-cycle detection.
//! - Explicit operational budgets.
//!
//! ## Ownership
//!
//! This module owns package dependency solving.
//!
//! It does not own:
//!
//! - Zamani source parsing.
//! - TOML parsing.
//! - Manifest schema.
//! - Version comparison algorithms.
//! - Registry protocols or network access.
//! - Git operations.
//! - Filesystem access.
//! - Package downloading or extraction.
//! - Package installation.
//! - Package signing or trust.
//! - Compiler dependency graphs.
//! - Build-task graphs.
//! - Target selection.
//! - Hardware discovery or allocation.
//! - Runtime scheduling or execution.
//!
//! Candidate providers own source discovery and source-specific evidence.
//! Constraint evaluators own version and compatibility interpretation.
//! The future lockfile subsystem owns serialization and lockfile migration.
//!
//! ## Version semantics
//!
//! Version requirements are interpreted only by registered evaluators.
//! The candidate's explicit version scheme is supplied to the existing
//! constraint subsystem.
//!
//! No default SemVer interpretation is assumed.
//!
//! Unknown version schemes or insufficient version evidence remain
//! indeterminate.
//!
//! ## Candidate completeness
//!
//! A complete inventory establishes that all candidates in its declared
//! source scope have been supplied.
//!
//! A partial inventory does not establish that undiscovered candidates
//! do not exist.
//!
//! An unavailable inventory supplies no usable candidate-discovery evidence.
//!
//! No-candidate results from incomplete inventories are indeterminate.
//!
//! ## Resolution identity
//!
//! This implementation selects one release per canonical `DependencyName`
//! within one resolution graph.
//!
//! Aliases remain local source-level names and do not create additional
//! package identities.
//!
//! Supporting simultaneous versions of the same canonical package name
//! requires a separately designed package-instance identity and graph key.
//!
//! ## Feature semantics
//!
//! Requested features are unified across all incoming declarations for the
//! same package.
//!
//! A candidate must establish that every requested feature exists.
//! Feature-specific dependencies are expanded only after the candidate
//! satisfies the accumulated mandatory constraints.
//!
//! ## Determinism
//!
//! Package names, provider identifiers, candidate identifiers, graph nodes,
//! graph edges and lock entries use ordered collections.
//!
//! Providers have explicit priorities. Within one provider, candidate order
//! is preserved and must be deterministic.
//!
//! Candidate preferences and hints rank candidates but never reject them.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on dependencies, candidates, features,
//! packages, graph nodes, providers, or dependency depth.
//!
//! Search is iterative and does not use recursive dependency traversal.
//!
//! Search snapshots and candidate inventories consume memory proportional
//! to the explored resolution state. Configurable budgets allow callers to
//! bound operational work without creating language-wide capacity ceilings.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! credential, registry or hardware access.
//!
//! Provider data is validated before it enters the resolution graph.
//! Provider errors are preserved as toolchain errors with additional context.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::Result;

use super::constraints::{
    CandidateVersion,
    Completeness,
    ConstraintDecision,
    ConstraintEvaluation,
    ConstraintEvaluatorRegistry,
    ConstraintKind,
    DependencyCandidateFacts,
    DependencyConstraint,
    DependencyConstraintSet,
    DependencyFeature,
    Evidence,
};

use super::model::{
    DependencyDeclaration,
    DependencyName,
    DependencySource,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_RESOLVE: &str = "dependency.resolve";
const OPERATION_PROVIDER: &str = "dependency.provider";
const OPERATION_CANDIDATE: &str = "dependency.candidate";
const OPERATION_LOCK: &str = "dependency.lock";

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_text(value: &str, field: &'static str) -> Result<()> {
    if value.is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_RESOLVE),
        );
    }

    if value.trim() != value {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain leading or trailing whitespace"
                ),
            )
            .with_operation(OPERATION_RESOLVE),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not contain control characters"),
            )
            .with_operation(OPERATION_RESOLVE),
        );
    }

    Ok(())
}

// ============================================================================
// PROVIDER IDENTITY
// ============================================================================

/// Stable identity of one registered dependency candidate provider.
///
/// A provider identity is an ecosystem or implementation identifier.
/// It is not a package identity or physical hardware identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CandidateProviderId(String);

impl CandidateProviderId {
    /// Construct a validated provider identity.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_text(&value, "candidate provider identity")?;
        Ok(Self(value))
    }

    /// Borrow the provider identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CandidateProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CANDIDATE IDENTITY
// ============================================================================

/// Stable provider-defined identity of one package candidate.
///
/// The identifier must remain stable for the same candidate across
/// discovery operations and lockfile creation.
///
/// Candidate identifiers are scoped by [`CandidateProviderId`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CandidateId(String);

impl CandidateId {
    /// Construct a validated candidate identity.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_text(&value, "dependency candidate identity")?;
        Ok(Self(value))
    }

    /// Borrow the candidate identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CANDIDATE REFERENCE
// ============================================================================

/// Globally unambiguous reference to a provider-owned candidate.
///
/// The pair is used in resolved graphs and lock snapshots.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CandidateRef {
    provider: CandidateProviderId,
    candidate: CandidateId,
}

impl CandidateRef {
    /// Construct a candidate reference.
    #[must_use]
    pub fn new(
        provider: CandidateProviderId,
        candidate: CandidateId,
    ) -> Self {
        Self {
            provider,
            candidate,
        }
    }

    /// Borrow the provider identity.
    #[must_use]
    pub fn provider(&self) -> &CandidateProviderId {
        &self.provider
    }

    /// Borrow the candidate identity.
    #[must_use]
    pub fn candidate(&self) -> &CandidateId {
        &self.candidate
    }
}

// ============================================================================
// DEPENDENCY CANDIDATE
// ============================================================================

/// One discovered package candidate.
///
/// Candidate version and source evidence are supplied through
/// [`DependencyCandidateFacts`]. The candidate must have an established
/// version because the resolver needs a stable release identity.
///
/// Dependencies and feature-specific dependencies are normalized
/// declarations owned by `super::model`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCandidate {
    id: CandidateId,
    name: DependencyName,
    facts: DependencyCandidateFacts,
    dependencies: Vec<DependencyDeclaration>,
    feature_dependencies:
        BTreeMap<DependencyFeature, Vec<DependencyDeclaration>>,
}

impl DependencyCandidate {
    /// Construct and validate a package candidate.
    ///
    /// Every feature used as a key in `feature_dependencies` must be
    /// present in the candidate's observed feature inventory.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed candidate identity, missing version
    /// evidence, invalid dependency declarations or inconsistent feature
    /// metadata.
    pub fn new(
        id: CandidateId,
        name: DependencyName,
        facts: DependencyCandidateFacts,
        dependencies: Vec<DependencyDeclaration>,
        feature_dependencies: BTreeMap<
            DependencyFeature,
            Vec<DependencyDeclaration>,
        >,
    ) -> Result<Self> {
        let candidate = Self {
            id,
            name,
            facts,
            dependencies,
            feature_dependencies,
        };

        candidate.validate()?;
        Ok(candidate)
    }

    /// Borrow the provider-defined candidate identity.
    #[must_use]
    pub fn id(&self) -> &CandidateId {
        &self.id
    }

    /// Borrow the canonical package name.
    #[must_use]
    pub fn name(&self) -> &DependencyName {
        &self.name
    }

    /// Borrow candidate facts.
    #[must_use]
    pub fn facts(&self) -> &DependencyCandidateFacts {
        &self.facts
    }

    /// Borrow unconditional package dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[DependencyDeclaration] {
        &self.dependencies
    }

    /// Borrow feature-specific dependency declarations.
    #[must_use]
    pub fn feature_dependencies(
        &self,
    ) -> &BTreeMap<DependencyFeature, Vec<DependencyDeclaration>> {
        &self.feature_dependencies
    }

    /// Return the established candidate version.
    #[must_use]
    pub fn version(&self) -> Option<&CandidateVersion> {
        match self.facts.version() {
            Evidence::Known(Some(version)) => Some(version),
            Evidence::Known(None) | Evidence::Unknown => None,
        }
    }

    /// Validate all candidate-owned declarations and evidence.
    pub fn validate(&self) -> Result<()> {
        match self.facts.version() {
            Evidence::Known(Some(_)) => {}
            Evidence::Known(None) => {
                return Err(
                    ToolchainError::validation_failed(
                        "dependency candidate has no package version",
                    )
                    .with_operation(OPERATION_CANDIDATE)
                    .with_subject(self.name.to_string()),
                );
            }
            Evidence::Unknown => {
                return Err(
                    ToolchainError::validation_failed(
                        "dependency candidate version evidence is unknown",
                    )
                    .with_operation(OPERATION_CANDIDATE)
                    .with_subject(self.name.to_string()),
                );
            }
        }

        for dependency in &self.dependencies {
            dependency.validate()?;
        }

        for (feature, dependencies) in &self.feature_dependencies {
            if !self.facts.features().contains(feature) {
                return Err(
                    ToolchainError::validation_failed(
                        format!(
                            "feature dependency metadata references feature \
                             '{feature}' that is absent from the observed \
                             feature inventory"
                        ),
                    )
                    .with_operation(OPERATION_CANDIDATE)
                    .with_subject(self.name.to_string()),
                );
            }

            for dependency in dependencies {
                dependency.validate()?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// CANDIDATE INVENTORY
// ============================================================================

/// Candidate inventory returned by one provider.
///
/// `Complete` means that the provider has supplied every candidate in
/// the source scope it claims to cover.
///
/// `Partial` means that additional candidates may exist.
///
/// `Unavailable` means that the provider has no usable inventory evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateInventory {
    candidates: Vec<DependencyCandidate>,
    completeness: Completeness,
}

impl CandidateInventory {
    /// Construct a validated candidate inventory.
    ///
    /// Candidate identifiers must be unique within one provider inventory.
    ///
    /// An unavailable inventory cannot contain candidates.
    pub fn new(
        candidates: Vec<DependencyCandidate>,
        completeness: Completeness,
    ) -> Result<Self> {
        if completeness == Completeness::Unavailable
            && !candidates.is_empty()
        {
            return Err(
                ToolchainError::validation_failed(
                    "an unavailable candidate inventory cannot contain candidates",
                )
                .with_operation(OPERATION_PROVIDER),
            );
        }

        let mut identities = BTreeSet::new();

        for candidate in &candidates {
            candidate.validate()?;

            if !identities.insert(candidate.id().clone()) {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        format!(
                            "duplicate candidate identity '{}'",
                            candidate.id(),
                        ),
                    )
                    .with_operation(OPERATION_PROVIDER)
                    .with_subject(candidate.name().to_string()),
                );
            }
        }

        Ok(Self {
            candidates,
            completeness,
        })
    }

    /// Borrow discovered candidates in provider-defined order.
    #[must_use]
    pub fn candidates(&self) -> &[DependencyCandidate] {
        &self.candidates
    }

    /// Return the inventory completeness.
    #[must_use]
    pub const fn completeness(&self) -> Completeness {
        self.completeness
    }

    /// Return whether the provider established a complete inventory.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completeness, Completeness::Complete)
    }
}

// ============================================================================
// CANDIDATE QUERY
// ============================================================================

/// Immutable query supplied to candidate providers.
///
/// Providers may use the declarations to identify relevant source scopes.
/// They must not silently remove candidates merely because a candidate
/// does not satisfy a version, feature or capability requirement.
///
/// Candidate constraint evaluation belongs to the resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateQuery {
    package: DependencyName,
    declarations: Vec<DependencyDeclaration>,
    requested_features: BTreeSet<DependencyFeature>,
    declared_sources: BTreeSet<DependencySource>,
}

impl CandidateQuery {
    fn from_pending(
        package: DependencyName,
        pending: &PendingDependency,
    ) -> Result<Self> {
        let mut requested_features = BTreeSet::new();
        let mut declared_sources = BTreeSet::new();

        for declaration in &pending.declarations {
            for feature in declaration.requested_features() {
                requested_features.insert(
                    DependencyFeature::new(feature.clone())?,
                );
            }

            if let Some(source) = declaration.source() {
                declared_sources.insert(source.clone());
            }
        }

        Ok(Self {
            package,
            declarations: pending.declarations.clone(),
            requested_features,
            declared_sources,
        })
    }

    /// Borrow the requested canonical package name.
    #[must_use]
    pub fn package(&self) -> &DependencyName {
        &self.package
    }

    /// Borrow all normalized declarations accumulated for this package.
    #[must_use]
    pub fn declarations(&self) -> &[DependencyDeclaration] {
        &self.declarations
    }

    /// Borrow the unified requested feature set.
    #[must_use]
    pub fn requested_features(&self) -> &BTreeSet<DependencyFeature> {
        &self.requested_features
    }

    /// Borrow explicitly declared dependency sources.
    ///
    /// An empty set means that no source was explicitly selected.
    #[must_use]
    pub fn declared_sources(&self) -> &BTreeSet<DependencySource> {
        &self.declared_sources
    }
}

// ============================================================================
// CANDIDATE PROVIDER CONTRACT
// ============================================================================

/// Provider contract for discovering dependency candidates.
///
/// A provider may represent a local source, registry, Git source,
/// URL source, private package service, federated source or future
/// package ecosystem.
///
/// Return values:
///
/// - `Ok(Some(inventory))`: this provider applies to the query.
/// - `Ok(None)`: this provider does not apply to the query.
/// - `Err(error)`: this provider was applicable but failed.
///
/// An applicable provider must report inventory completeness accurately.
/// It must not report `Complete` when pagination, access, filtering,
/// authentication or source discovery left candidates unexamined.
pub trait DependencyCandidateProvider: Send + Sync {
    /// Discover candidates for one package query.
    fn discover(
        &self,
        query: &CandidateQuery,
    ) -> Result<Option<CandidateInventory>>;
}

// ============================================================================
// PROVIDER REGISTRATION
// ============================================================================

struct ProviderRegistration {
    priority: i64,
    provider: Arc<dyn DependencyCandidateProvider>,
}

/// Registry of extensible dependency candidate providers.
///
/// Providers are ordered by ascending priority and then provider identity.
/// A provider's candidate order is preserved within that provider.
#[derive(Default)]
pub struct CandidateProviderRegistry {
    providers: BTreeMap<CandidateProviderId, ProviderRegistration>,
}

impl CandidateProviderRegistry {
    /// Construct an empty provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a candidate provider.
    ///
    /// Lower numeric priorities are queried first. Equal priorities are
    /// ordered by provider identity.
    ///
    /// Duplicate provider identities are rejected.
    pub fn register<P>(
        &mut self,
        id: CandidateProviderId,
        priority: i64,
        provider: P,
    ) -> Result<()>
    where
        P: DependencyCandidateProvider + 'static,
    {
        if self.providers.contains_key(&id) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!("candidate provider '{id}' is already registered"),
                )
                .with_operation(OPERATION_PROVIDER),
            );
        }

        self.providers.insert(
            id,
            ProviderRegistration {
                priority,
                provider: Arc::new(provider),
            },
        );

        Ok(())
    }

    /// Return the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Return whether no providers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    fn discover(
        &self,
        query: &CandidateQuery,
        budget: &mut BudgetMeter,
    ) -> Result<AggregatedInventory> {
        let mut registrations: Vec<_> = self.providers.iter().collect();

        registrations.sort_by(|(left_id, left), (right_id, right)| {
            left.priority
                .cmp(&right.priority)
                .then_with(|| left_id.cmp(right_id))
        });

        let mut candidates = Vec::new();
        let mut applicable_providers = 0_usize;
        let mut all_complete = true;
        let mut any_usable_inventory = false;

        for (provider_id, registration) in registrations {
            budget.consume_provider_call()?;

            let inventory = registration
                .provider
                .discover(query)
                .map_err(|error| {
                    error
                        .with_operation(OPERATION_PROVIDER)
                        .with_subject(provider_id.to_string())
                        .with_context(format!(
                            "discovering candidates for '{}'",
                            query.package(),
                        ))
                })?;

            let Some(inventory) = inventory else {
                continue;
            };

            applicable_providers = applicable_providers.saturating_add(1);

            if inventory.completeness() != Completeness::Complete {
                all_complete = false;
            }

            if inventory.completeness() != Completeness::Unavailable {
                any_usable_inventory = true;
            }

            for candidate in inventory.candidates() {
                if candidate.name() != query.package() {
                    return Err(
                        ToolchainError::provider_failed(
                            format!(
                                "provider returned package '{}' for query '{}'",
                                candidate.name(),
                                query.package(),
                            ),
                        )
                        .with_operation(OPERATION_PROVIDER)
                        .with_subject(provider_id.to_string()),
                    );
                }

                candidates.push(CandidateChoice {
                    reference: CandidateRef::new(
                        provider_id.clone(),
                        candidate.id().clone(),
                    ),
                    candidate: candidate.clone(),
                    preference_score: (0, 0),
                });
            }
        }

        let completeness = if applicable_providers == 0
            || !any_usable_inventory
        {
            Completeness::Unavailable
        } else if all_complete {
            Completeness::Complete
        } else {
            Completeness::Partial
        };

        Ok(AggregatedInventory {
            candidates,
            completeness,
        })
    }
}

#[derive(Debug, Clone)]
struct AggregatedInventory {
    candidates: Vec<CandidateChoice>,
    completeness: Completeness,
}

#[derive(Debug, Clone)]
struct CandidateChoice {
    reference: CandidateRef,
    candidate: DependencyCandidate,
    preference_score: (usize, usize),
}

// ============================================================================
// LOCK CONTRACT
// ============================================================================

/// Policy for applying an existing dependency lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockMode {
    /// Ignore an existing lock.
    Ignore,

    /// Prefer locked candidates but permit a new valid solution.
    Prefer,

    /// Require the exact locked candidate for every resolved package.
    Frozen,
}

/// One locked package candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedCandidate {
    reference: CandidateRef,
}

impl LockedCandidate {
    /// Construct a locked candidate reference.
    #[must_use]
    pub fn new(reference: CandidateRef) -> Self {
        Self { reference }
    }

    /// Borrow the locked candidate reference.
    #[must_use]
    pub fn reference(&self) -> &CandidateRef {
        &self.reference
    }
}

/// In-memory dependency lock snapshot.
///
/// Serialization, format versioning, file I/O and migration belong to
/// `dependencies::lockfile`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolutionLock {
    packages: BTreeMap<DependencyName, LockedCandidate>,
}

impl ResolutionLock {
    /// Construct an empty lock snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert one locked package.
    ///
    /// Duplicate package names are rejected.
    pub fn insert(
        &mut self,
        package: DependencyName,
        candidate: LockedCandidate,
    ) -> Result<()> {
        if self.packages.contains_key(&package) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!("lock already contains package '{package}'"),
                )
                .with_operation(OPERATION_LOCK),
            );
        }

        self.packages.insert(package, candidate);
        Ok(())
    }

    /// Find a locked candidate by canonical package name.
    #[must_use]
    pub fn get(
        &self,
        package: &DependencyName,
    ) -> Option<&LockedCandidate> {
        self.packages.get(package)
    }

    /// Borrow all locked package entries.
    #[must_use]
    pub fn packages(
        &self,
    ) -> &BTreeMap<DependencyName, LockedCandidate> {
        &self.packages
    }

    /// Return the number of locked packages.
    #[must_use]
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    /// Return whether the lock contains no packages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }
}

// ============================================================================
// RESOLUTION BUDGET
// ============================================================================

/// Configurable operational budgets for dependency resolution.
///
/// `None` means that this resolver-level budget is not configured.
/// It does not mean that the process has infinite memory or execution time.
///
/// These limits protect a particular operation. They are not package,
/// language, hardware or dependency-count ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolutionBudget {
    max_provider_calls: Option<u128>,
    max_candidate_evaluations: Option<u128>,
    max_backtracks: Option<u128>,
}

impl Default for ResolutionBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl ResolutionBudget {
    /// Construct a budget with no resolver-specific numeric limits.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_provider_calls: None,
            max_candidate_evaluations: None,
            max_backtracks: None,
        }
    }

    /// Set the provider-call budget.
    #[must_use]
    pub const fn with_max_provider_calls(
        mut self,
        maximum: Option<u128>,
    ) -> Self {
        self.max_provider_calls = maximum;
        self
    }

    /// Set the candidate-evaluation budget.
    #[must_use]
    pub const fn with_max_candidate_evaluations(
        mut self,
        maximum: Option<u128>,
    ) -> Self {
        self.max_candidate_evaluations = maximum;
        self
    }

    /// Set the backtracking budget.
    #[must_use]
    pub const fn with_max_backtracks(
        mut self,
        maximum: Option<u128>,
    ) -> Self {
        self.max_backtracks = maximum;
        self
    }
}

#[derive(Debug, Default)]
struct BudgetMeter {
    provider_calls: u128,
    candidate_evaluations: u128,
    backtracks: u128,
}

impl BudgetMeter {
    fn consume_provider_call(&mut self, budget: &ResolutionBudget) -> Result<()> {
        self.provider_calls = self.provider_calls.saturating_add(1);

        check_budget(
            self.provider_calls,
            budget.max_provider_calls,
            "dependency provider-call budget exhausted",
        )
    }

    fn consume_candidate_evaluation(
        &mut self,
        budget: &ResolutionBudget,
    ) -> Result<()> {
        self.candidate_evaluations =
            self.candidate_evaluations.saturating_add(1);

        check_budget(
            self.candidate_evaluations,
            budget.max_candidate_evaluations,
            "dependency candidate-evaluation budget exhausted",
        )
    }

    fn consume_backtrack(&mut self, budget: &ResolutionBudget) -> Result<()> {
        self.backtracks = self.backtracks.saturating_add(1);

        check_budget(
            self.backtracks,
            budget.max_backtracks,
            "dependency backtracking budget exhausted",
        )
    }
}

fn check_budget(
    actual: u128,
    maximum: Option<u128>,
    message: &'static str,
) -> Result<()> {
    if maximum.is_some_and(|limit| actual > limit) {
        return Err(
            ToolchainError::resource_exhausted(message)
                .with_operation(OPERATION_RESOLVE),
        );
    }

    Ok(())
}

// ============================================================================
// RESOLUTION OPTIONS
// ============================================================================

/// Configuration for one dependency-resolution operation.
#[derive(Debug, Clone)]
pub struct ResolutionOptions {
    include_optional_dependencies: bool,
    lock_mode: LockMode,
    budget: ResolutionBudget,
    additional_constraints:
        BTreeMap<DependencyName, DependencyConstraintSet>,
}

impl Default for ResolutionOptions {
    fn default() -> Self {
        Self {
            include_optional_dependencies: false,
            lock_mode: LockMode::Ignore,
            budget: ResolutionBudget::default(),
            additional_constraints: BTreeMap::new(),
        }
    }
}

impl ResolutionOptions {
    /// Construct default resolution options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether optional dependency declarations are included.
    #[must_use]
    pub fn with_optional_dependencies(
        mut self,
        include: bool,
    ) -> Self {
        self.include_optional_dependencies = include;
        self
    }

    /// Set the lock policy.
    #[must_use]
    pub fn with_lock_mode(mut self, mode: LockMode) -> Self {
        self.lock_mode = mode;
        self
    }

    /// Set operational budgets.
    #[must_use]
    pub fn with_budget(mut self, budget: ResolutionBudget) -> Self {
        self.budget = budget;
        self
    }

    /// Add an extra constraint for a package.
    ///
    /// This is the extension point for workspace, organization or
    /// caller-supplied requirements, constraints, preferences and hints.
    ///
    /// Preferences and hints are used for ranking only. They cannot reject
    /// a candidate.
    pub fn add_constraint(
        &mut self,
        package: DependencyName,
        constraint: DependencyConstraint,
    ) -> Result<()> {
        self.additional_constraints
            .entry(package)
            .or_default()
            .insert(constraint)
    }

    /// Borrow additional constraints for all packages.
    #[must_use]
    pub fn additional_constraints(
        &self,
    ) -> &BTreeMap<DependencyName, DependencyConstraintSet> {
        &self.additional_constraints
    }
}

// ============================================================================
// RESOLUTION DIAGNOSTICS
// ============================================================================

/// Stable classification of a dependency-resolution conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolutionDiagnosticCode {
    /// No provider established an applicable candidate inventory.
    CandidateEvidenceUnavailable,

    /// A complete inventory contains no candidates.
    NoCandidates,

    /// Candidate evidence is incomplete or insufficient.
    CandidateEvidenceIncomplete,

    /// Every sufficiently known candidate violates mandatory constraints.
    ConstraintsUnsatisfied,

    /// A dependency cycle was found.
    DependencyCycle,

    /// Frozen resolution has no lock entry for a required package.
    LockEntryMissing,

    /// The locked candidate was not found in the available inventory.
    LockedCandidateUnavailable,

    /// The locked candidate exists but violates mandatory constraints.
    LockedCandidateRejected,

    /// A preferred locked candidate was not available for selection.
    LockPreferenceUnavailable,
}

/// Structured dependency-resolution diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionDiagnostic {
    code: ResolutionDiagnosticCode,
    package: Option<DependencyName>,
    message: String,
    cycle: Vec<DependencyName>,
}

impl ResolutionDiagnostic {
    /// Construct a diagnostic without a cycle path.
    #[must_use]
    pub fn new(
        code: ResolutionDiagnosticCode,
        package: Option<DependencyName>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            package,
            message: message.into(),
            cycle: Vec::new(),
        }
    }

    /// Construct a dependency-cycle diagnostic.
    #[must_use]
    pub fn cycle(path: Vec<DependencyName>) -> Self {
        let package = path.first().cloned();

        Self {
            code: ResolutionDiagnosticCode::DependencyCycle,
            package,
            message: "dependency graph contains a cycle".to_owned(),
            cycle: path,
        }
    }

    /// Return the diagnostic code.
    #[must_use]
    pub const fn code(&self) -> ResolutionDiagnosticCode {
        self.code
    }

    /// Borrow the affected package, if known.
    #[must_use]
    pub fn package(&self) -> Option<&DependencyName> {
        self.package.as_ref()
    }

    /// Borrow the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Borrow the cycle path, if this is a cycle diagnostic.
    #[must_use]
    pub fn cycle_path(&self) -> &[DependencyName] {
        &self.cycle
    }
}

/// A dependency-resolution conflict.
///
/// `indeterminate == true` means that the resolver cannot prove that no
/// solution exists because one or more relevant inventories or candidate
/// facts were incomplete.
///
/// A non-indeterminate conflict means that the available complete evidence
/// establishes that the attempted resolution has no valid solution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionConflict {
    diagnostic: ResolutionDiagnostic,
    indeterminate: bool,
}

impl ResolutionConflict {
    /// Construct a resolution conflict.
    #[must_use]
    pub fn new(
        diagnostic: ResolutionDiagnostic,
        indeterminate: bool,
    ) -> Self {
        Self {
            diagnostic,
            indeterminate,
        }
    }

    /// Borrow the conflict diagnostic.
    #[must_use]
    pub fn diagnostic(&self) -> &ResolutionDiagnostic {
        &self.diagnostic
    }

    /// Return whether the conflict is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }
}

// ============================================================================
// RESOLVED GRAPH
// ============================================================================

/// One successfully selected package in a resolution graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPackage {
    reference: CandidateRef,
    candidate: DependencyCandidate,
    activated_features: BTreeSet<DependencyFeature>,
}

impl ResolvedPackage {
    /// Borrow the selected candidate reference.
    #[must_use]
    pub fn reference(&self) -> &CandidateRef {
        &self.reference
    }

    /// Borrow the selected candidate.
    #[must_use]
    pub fn candidate(&self) -> &DependencyCandidate {
        &self.candidate
    }

    /// Borrow the unified activated feature set.
    #[must_use]
    pub fn activated_features(&self) -> &BTreeSet<DependencyFeature> {
        &self.activated_features
    }
}

/// One directed package dependency edge.
///
/// `from == None` represents a direct root dependency.
/// Otherwise, `from` identifies the parent package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyEdge {
    from: Option<DependencyName>,
    to: DependencyName,
}

impl DependencyEdge {
    /// Borrow the parent package, or return `None` for a root edge.
    #[must_use]
    pub fn from(&self) -> Option<&DependencyName> {
        self.from.as_ref()
    }

    /// Borrow the child package.
    #[must_use]
    pub fn to(&self) -> &DependencyName {
        &self.to
    }
}

/// A fully resolved package dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionGraph {
    packages: BTreeMap<DependencyName, ResolvedPackage>,
    edges: BTreeSet<DependencyEdge>,
    roots: BTreeSet<DependencyName>,
    warnings: Vec<ResolutionDiagnostic>,
}

impl ResolutionGraph {
    fn from_state(state: &SearchState) -> Self {
        let packages = state
            .selected
            .iter()
            .map(|(name, selected)| {
                (
                    name.clone(),
                    ResolvedPackage {
                        reference: selected.reference.clone(),
                        candidate: selected.candidate.clone(),
                        activated_features: selected.expanded_features.clone(),
                    },
                )
            })
            .collect();

        Self {
            packages,
            edges: state.edges.clone(),
            roots: state.roots.clone(),
            warnings: state.warnings.clone(),
        }
    }

    /// Borrow all resolved packages in canonical package-name order.
    #[must_use]
    pub fn packages(&self) -> &BTreeMap<DependencyName, ResolvedPackage> {
        &self.packages
    }

    /// Find a resolved package by canonical package name.
    #[must_use]
    pub fn package(
        &self,
        name: &DependencyName,
    ) -> Option<&ResolvedPackage> {
        self.packages.get(name)
    }

    /// Borrow all package dependency edges.
    #[must_use]
    pub fn edges(&self) -> &BTreeSet<DependencyEdge> {
        &self.edges
    }

    /// Borrow direct root package names.
    #[must_use]
    pub fn roots(&self) -> &BTreeSet<DependencyName> {
        &self.roots
    }

    /// Borrow non-fatal resolution warnings.
    #[must_use]
    pub fn warnings(&self) -> &[ResolutionDiagnostic] {
        &self.warnings
    }

    /// Return the number of resolved packages.
    #[must_use]
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    /// Return whether the graph contains no resolved packages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Produce an in-memory lock snapshot from the resolved graph.
    ///
    /// Lockfile serialization belongs to `dependencies::lockfile`.
    #[must_use]
    pub fn to_lock(&self) -> ResolutionLock {
        let packages = self
            .packages
            .iter()
            .map(|(name, package)| {
                (
                    name.clone(),
                    LockedCandidate::new(package.reference.clone()),
                )
            })
            .collect();

        ResolutionLock { packages }
    }

    fn find_cycle(&self) -> Option<Vec<DependencyName>> {
        let mut adjacency: BTreeMap<
            DependencyName,
            BTreeSet<DependencyName>,
        > = self
            .packages
            .keys()
            .cloned()
            .map(|name| (name, BTreeSet::new()))
            .collect();

        for edge in &self.edges {
            if let Some(parent) = &edge.from {
                if let Some(children) = adjacency.get_mut(parent) {
                    children.insert(edge.to.clone());
                }
            }
        }

        let mut colors: BTreeMap<DependencyName, u8> = self
            .packages
            .keys()
            .cloned()
            .map(|name| (name, 0))
            .collect();

        for start in self.packages.keys() {
            if colors.get(start).copied().unwrap_or(0) != 0 {
                continue;
            }

            let mut path = vec![start.clone()];
            let first_neighbors: Vec<_> = adjacency
                .get(start)
                .into_iter()
                .flat_map(|neighbors| neighbors.iter().cloned())
                .collect();

            let mut frames = vec![(start.clone(), first_neighbors, 0_usize)];

            if let Some(color) = colors.get_mut(start) {
                *color = 1;
            }

            while let Some((node, neighbors, next_index)) = frames.last_mut() {
                if *next_index >= neighbors.len() {
                    let completed = node.clone();
                    frames.pop();

                    if let Some(color) = colors.get_mut(&completed) {
                        *color = 2;
                    }

                    let _ = path.pop();
                    continue;
                }

                let neighbor = neighbors[*next_index].clone();
                *next_index += 1;

                match colors.get(&neighbor).copied().unwrap_or(0) {
                    0 => {
                        if let Some(color) = colors.get_mut(&neighbor) {
                            *color = 1;
                        }

                        path.push(neighbor.clone());

                        let next_neighbors: Vec<_> = adjacency
                            .get(&neighbor)
                            .into_iter()
                            .flat_map(|values| values.iter().cloned())
                            .collect();

                        frames.push((neighbor, next_neighbors, 0));
                    }

                    1 => {
                        if let Some(position) =
                            path.iter().position(|item| item == &neighbor)
                        {
                            let mut cycle = path[position..].to_vec();
                            cycle.push(neighbor);
                            return Some(cycle);
                        }
                    }

                    _ => {}
                }
            }
        }

        None
    }
}

// ============================================================================
// RESOLUTION OUTCOME
// ============================================================================

/// Result of attempting dependency resolution.
///
/// A conflict is a structured outcome, not a successful resolution.
/// Fatal provider, evaluator and operational-budget failures are returned
/// through the canonical toolchain `Result`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionOutcome {
    /// Every required package was resolved and the graph is acyclic.
    Resolved(ResolutionGraph),

    /// No solution was established.
    Conflict(ResolutionConflict),
}

impl ResolutionOutcome {
    /// Borrow the resolved graph, if resolution succeeded.
    #[must_use]
    pub fn graph(&self) -> Option<&ResolutionGraph> {
        match self {
            Self::Resolved(graph) => Some(graph),
            Self::Conflict(_) => None,
        }
    }

    /// Borrow the conflict, if resolution did not succeed.
    #[must_use]
    pub fn conflict(&self) -> Option<&ResolutionConflict> {
        match self {
            Self::Resolved(_) => None,
            Self::Conflict(conflict) => Some(conflict),
        }
    }

    /// Return whether a complete resolution was produced.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }
}

// ============================================================================
// RESOLVER STATE
// ============================================================================

#[derive(Debug, Clone, Default)]
struct PendingDependency {
    declarations: Vec<DependencyDeclaration>,
    requested_features: BTreeSet<DependencyFeature>,
}

#[derive(Debug, Clone)]
struct SelectedCandidate {
    reference: CandidateRef,
    candidate: DependencyCandidate,
    expanded_features: BTreeSet<DependencyFeature>,
    validated_declaration_count: usize,
    inventory_completeness: Completeness,
}

#[derive(Debug, Clone, Default)]
struct SearchState {
    pending: BTreeMap<DependencyName, PendingDependency>,
    selected: BTreeMap<DependencyName, SelectedCandidate>,
    edges: BTreeSet<DependencyEdge>,
    roots: BTreeSet<DependencyName>,
    warnings: Vec<ResolutionDiagnostic>,
}

#[derive(Debug, Clone)]
struct DecisionFrame {
    before: SearchState,
    choices: Vec<CandidateChoice>,
    next_choice: usize,
    inventory_completeness: Completeness,
    saw_indeterminate: bool,
}

enum Step {
    Solved(ResolutionGraph),
    Conflict(ResolutionConflict),
    Branch {
        choices: Vec<CandidateChoice>,
        inventory_completeness: Completeness,
        saw_indeterminate: bool,
    },
}

// ============================================================================
// DEPENDENCY RESOLVER
// ============================================================================

/// Dependency resolver using registered candidate and constraint providers.
///
/// The resolver borrows provider and evaluator registries. It does not
/// own their lifetimes or perform provider registration implicitly.
pub struct DependencyResolver<'a> {
    providers: &'a CandidateProviderRegistry,
    evaluators: &'a ConstraintEvaluatorRegistry,
}

impl<'a> DependencyResolver<'a> {
    /// Construct a resolver from canonical provider registries.
    #[must_use]
    pub fn new(
        providers: &'a CandidateProviderRegistry,
        evaluators: &'a ConstraintEvaluatorRegistry,
    ) -> Self {
        Self {
            providers,
            evaluators,
        }
    }

    /// Resolve a collection of root dependency declarations.
    ///
    /// The operation is deterministic for equivalent declarations,
    /// provider inventories, provider priorities, evaluator behavior,
    /// options and lock data.
    ///
    /// # Errors
    ///
    /// Returns a toolchain error for invalid declarations, invalid provider
    /// data, provider/evaluator failures or exhausted operational budgets.
    ///
    /// A valid but unsatisfied or indeterminate dependency problem is
    /// returned as [`ResolutionOutcome::Conflict`].
    pub fn resolve<I>(
        &self,
        roots: I,
        options: &ResolutionOptions,
        lock: Option<&ResolutionLock>,
    ) -> Result<ResolutionOutcome>
    where
        I: IntoIterator<Item = DependencyDeclaration>,
    {
        if options.lock_mode != LockMode::Ignore && lock.is_none() {
            return Err(
                ToolchainError::invalid_input(
                    "a lock snapshot is required for the selected lock mode",
                )
                .with_operation(OPERATION_RESOLVE),
            );
        }

        let mut state = SearchState::default();

        for declaration in roots {
            declaration.validate()?;

            add_declaration(
                &mut state,
                None,
                declaration,
                options.include_optional_dependencies,
            )?;
        }

        let mut frames: Vec<DecisionFrame> = Vec::new();
        let mut meter = BudgetMeter::default();

        loop {
            match self.advance(
                &mut state,
                &mut frames,
                options,
                lock,
                &mut meter,
            )? {
                Step::Solved(graph) => {
                    return Ok(ResolutionOutcome::Resolved(graph));
                }

                Step::Conflict(conflict) => {
                    match self.backtrack(
                        &mut state,
                        &mut frames,
                        conflict,
                        options,
                        &mut meter,
                    )? {
                        Some(final_conflict) => {
                            return Ok(ResolutionOutcome::Conflict(
                                final_conflict,
                            ));
                        }

                        None => {}
                    }
                }

                Step::Branch {
                    choices,
                    inventory_completeness,
                    saw_indeterminate,
                } => {
                    let Some(first) = choices.first().cloned() else {
                        return Err(
                            ToolchainError::internal(
                                "resolver produced an empty candidate branch",
                            )
                            .with_operation(OPERATION_RESOLVE),
                        );
                    };

                    let frame = DecisionFrame {
                        before: state.clone(),
                        choices,
                        next_choice: 1,
                        inventory_completeness,
                        saw_indeterminate,
                    };

                    frames.push(frame);

                    let package = first.candidate.name().clone();

                    apply_candidate(
                        &mut state,
                        first,
                        inventory_completeness,
                        options.include_optional_dependencies,
                    )?;

                    if !state.selected.contains_key(&package) {
                        return Err(
                            ToolchainError::internal(
                                "candidate selection did not update resolver state",
                            )
                            .with_operation(OPERATION_RESOLVE)
                            .with_subject(package.to_string()),
                        );
                    }
                }
            }
        }
    }

    fn advance(
        &self,
        state: &mut SearchState,
        _frames: &mut Vec<DecisionFrame>,
        options: &ResolutionOptions,
        lock: Option<&ResolutionLock>,
        meter: &mut BudgetMeter,
    ) -> Result<Step> {
        loop {
            let selected_names: Vec<_> =
                state.selected.keys().cloned().collect();

            let mut expanded_dependencies = false;

            for name in selected_names {
                let Some(pending) = state.pending.get(&name).cloned() else {
                    return Err(
                        ToolchainError::internal(
                            "selected package has no pending dependency record",
                        )
                        .with_operation(OPERATION_RESOLVE)
                        .with_subject(name.to_string()),
                    );
                };

                let Some(selected_snapshot) =
                    state.selected.get(&name).cloned()
                else {
                    continue;
                };

                if selected_snapshot.validated_declaration_count
                    != pending.declarations.len()
                {
                    let report = self.evaluate_candidate(
                        &name,
                        &pending,
                        &selected_snapshot.candidate,
                        options,
                        meter,
                    )?;

                    match report.decision() {
                        ConstraintDecision::Satisfied => {
                            if let Some(selected) =
                                state.selected.get_mut(&name)
                            {
                                selected.validated_declaration_count =
                                    pending.declarations.len();
                            }
                        }

                        ConstraintDecision::Violated => {
                            return Ok(Step::Conflict(
                                ResolutionConflict::new(
                                    ResolutionDiagnostic::new(
                                        ResolutionDiagnosticCode::ConstraintsUnsatisfied,
                                        Some(name.clone()),
                                        "the selected candidate violates a \
                                         newly accumulated mandatory dependency \
                                         constraint",
                                    ),
                                    selected_snapshot.inventory_completeness
                                        != Completeness::Complete,
                                ),
                            ));
                        }

                        ConstraintDecision::Indeterminate => {
                            return Ok(Step::Conflict(
                                ResolutionConflict::new(
                                    ResolutionDiagnostic::new(
                                        ResolutionDiagnosticCode::CandidateEvidenceIncomplete,
                                        Some(name.clone()),
                                        "the selected candidate cannot be \
                                         established as compatible with all \
                                         accumulated dependency requirements",
                                    ),
                                    true,
                                ),
                            ));
                        }
                    }
                }

                let new_features: Vec<_> = pending
                    .requested_features
                    .difference(&selected_snapshot.expanded_features)
                    .cloned()
                    .collect();

                if !new_features.is_empty() {
                    let mut feature_dependencies = Vec::new();

                    for feature in &new_features {
                        if let Some(dependencies) = selected_snapshot
                            .candidate
                            .feature_dependencies()
                            .get(feature)
                        {
                            feature_dependencies.extend(
                                dependencies.iter().cloned(),
                            );
                        }
                    }

                    if let Some(selected) = state.selected.get_mut(&name) {
                        selected
                            .expanded_features
                            .extend(new_features);
                    }

                    for dependency in feature_dependencies {
                        add_declaration(
                            state,
                            Some(name.clone()),
                            dependency,
                            options.include_optional_dependencies,
                        )?;
                    }

                    expanded_dependencies = true;
                }
            }

            if expanded_dependencies {
                continue;
            }

            let unresolved = state
                .pending
                .iter()
                .find(|(name, _)| !state.selected.contains_key(*name))
                .map(|(name, pending)| (name.clone(), pending.clone()));

            let Some((package, pending)) = unresolved else {
                let graph = ResolutionGraph::from_state(state);

                if let Some(cycle) = graph.find_cycle() {
                    return Ok(Step::Conflict(
                        ResolutionConflict::new(
                            ResolutionDiagnostic::cycle(cycle),
                            false,
                        ),
                    ));
                }

                return Ok(Step::Solved(graph));
            };

            let query = CandidateQuery::from_pending(
                package.clone(),
                &pending,
            )?;

            let inventory = self.providers.discover(&query, meter)?;

            let mut choices = Vec::new();
            let mut saw_indeterminate = false;
            let mut locked_candidate_seen = false;
            let mut locked_candidate_satisfied = false;

            let lock_entry = if options.lock_mode == LockMode::Ignore {
                None
            } else {
                lock.and_then(|value| value.get(&package))
            };

            if options.lock_mode == LockMode::Frozen
                && lock_entry.is_none()
            {
                return Ok(Step::Conflict(
                    ResolutionConflict::new(
                        ResolutionDiagnostic::new(
                            ResolutionDiagnosticCode::LockEntryMissing,
                            Some(package.clone()),
                            "frozen resolution requires a lock entry for \
                             every selected package",
                        ),
                        false,
                    ),
                    false,
                ));
            }

            for mut candidate_choice in inventory.candidates.iter().cloned() {
                meter.consume_candidate_evaluation(&options.budget)?;

                let candidate = &candidate_choice.candidate;

                if let Some(entry) = lock_entry {
                    if candidate_choice.reference == *entry.reference() {
                        locked_candidate_seen = true;
                    }
                }

                let report = self.evaluate_candidate(
                    &package,
                    &pending,
                    candidate,
                    options,
                    meter,
                )?;

                match report.decision() {
                    ConstraintDecision::Satisfied => {
                        candidate_choice.preference_score =
                            preference_score(&report);

                        if let Some(entry) = lock_entry {
                            if candidate_choice.reference
                                == *entry.reference()
                            {
                                locked_candidate_satisfied = true;
                            }
                        }

                        choices.push(candidate_choice);
                    }

                    ConstraintDecision::Violated => {}

                    ConstraintDecision::Indeterminate => {
                        saw_indeterminate = true;
                    }
                }
            }

            choices.sort_by(|left, right| {
                right
                    .preference_score
                    .cmp(&left.preference_score)
            });

            match options.lock_mode {
                LockMode::Ignore => {}

                LockMode::Prefer => {
                    if let Some(entry) = lock_entry {
                        if let Some(index) = choices.iter().position(
                            |choice| {
                                choice.reference == *entry.reference()
                            },
                        ) {
                            let locked = choices.remove(index);
                            choices.insert(0, locked);
                        } else {
                            state.warnings.push(
                                ResolutionDiagnostic::new(
                                    ResolutionDiagnosticCode::LockPreferenceUnavailable,
                                    Some(package.clone()),
                                    "the preferred locked candidate was not \
                                     available as a satisfying candidate; \
                                     normal resolution will be attempted",
                                ),
                            );
                        }
                    } else {
                        state.warnings.push(
                            ResolutionDiagnostic::new(
                                ResolutionDiagnosticCode::LockPreferenceUnavailable,
                                Some(package.clone()),
                                "no lock entry exists for this package; \
                                 normal resolution will be attempted",
                            ),
                        );
                    }
                }

                LockMode::Frozen => {
                    let entry = lock_entry.ok_or_else(|| {
                        ToolchainError::internal(
                            "frozen lock entry disappeared during resolution",
                        )
                        .with_operation(OPERATION_LOCK)
                    })?;

                    if locked_candidate_satisfied {
                        choices.retain(|choice| {
                            choice.reference == *entry.reference()
                        });
                    } else {
                        let indeterminate =
                            inventory.completeness != Completeness::Complete
                                || saw_indeterminate;

                        let (code, message) = if locked_candidate_seen {
                            (
                                ResolutionDiagnosticCode::LockedCandidateRejected,
                                "the locked candidate violates or cannot \
                                 establish the mandatory dependency constraints",
                            )
                        } else {
                            (
                                ResolutionDiagnosticCode::LockedCandidateUnavailable,
                                "the locked candidate was not found in the \
                                 available candidate inventory",
                            )
                        };

                        return Ok(Step::Conflict(
                            ResolutionConflict::new(
                                ResolutionDiagnostic::new(
                                    code,
                                    Some(package.clone()),
                                    message,
                                ),
                                indeterminate,
                            ),
                        ));
                    }
                }
            }

            if choices.is_empty() {
                let (code, message, indeterminate) =
                    if inventory.candidates.is_empty() {
                        if inventory.completeness == Completeness::Complete {
                            (
                                ResolutionDiagnosticCode::NoCandidates,
                                "no candidate exists in the complete source inventory",
                                false,
                            )
                        } else {
                            (
                                ResolutionDiagnosticCode::CandidateEvidenceIncomplete,
                                "no candidate was discovered, but the source \
                                 inventory is incomplete or unavailable",
                                true,
                            )
                        }
                    } else if saw_indeterminate
                        || inventory.completeness != Completeness::Complete
                    {
                        (
                            ResolutionDiagnosticCode::CandidateEvidenceIncomplete,
                            "available candidate evidence cannot establish \
                             a satisfying dependency solution",
                            true,
                        )
                    } else {
                        (
                            ResolutionDiagnosticCode::ConstraintsUnsatisfied,
                            "all discovered candidates violate mandatory \
                             dependency constraints",
                            false,
                        )
                    };

                return Ok(Step::Conflict(
                    ResolutionConflict::new(
                        ResolutionDiagnostic::new(
                            code,
                            Some(package),
                            message,
                        ),
                        indeterminate,
                    ),
                ));
            }

            return Ok(Step::Branch {
                choices,
                inventory_completeness: inventory.completeness,
                saw_indeterminate,
            });
        }
    }

    fn evaluate_candidate(
        &self,
        package: &DependencyName,
        pending: &PendingDependency,
        candidate: &DependencyCandidate,
        options: &ResolutionOptions,
        _meter: &mut BudgetMeter,
    ) -> Result<super::constraints::ConstraintReport> {
        if candidate.name() != package {
            return Err(
                ToolchainError::provider_failed(
                    "candidate package identity does not match the requested package",
                )
                .with_operation(OPERATION_CANDIDATE)
                .with_subject(package.to_string()),
            );
        }

        let scheme = candidate.version().map(|version| {
            version.scheme().clone()
        });

        let mut combined = DependencyConstraintSet::new();
        let mut unique = BTreeSet::new();

        for declaration in &pending.declarations {
            let declaration_constraints =
                DependencyConstraintSet::from_declaration(
                    declaration,
                    scheme.clone(),
                )?;

            for constraint in declaration_constraints.iter() {
                if unique.insert(constraint.clone()) {
                    combined.insert(constraint.clone())?;
                }
            }
        }

        if let Some(additional) =
            options.additional_constraints.get(package)
        {
            for constraint in additional.iter() {
                if unique.insert(constraint.clone()) {
                    combined.insert(constraint.clone())?;
                }
            }
        }

        self.evaluators.evaluate(&combined, candidate.facts())
    }

    fn backtrack(
        &self,
        state: &mut SearchState,
        frames: &mut Vec<DecisionFrame>,
        mut conflict: ResolutionConflict,
        options: &ResolutionOptions,
        meter: &mut BudgetMeter,
    ) -> Result<Option<ResolutionConflict>> {
        while let Some(mut frame) = frames.pop() {
            *state = frame.before.clone();

            if frame.next_choice < frame.choices.len() {
                meter.consume_backtrack(&options.budget)?;

                let choice = frame.choices[frame.next_choice].clone();
                frame.next_choice += 1;

                let completeness = frame.inventory_completeness;

                frames.push(frame);

                apply_candidate(
                    state,
                    choice,
                    completeness,
                    options.include_optional_dependencies,
                )?;

                return Ok(None);
            }

            if frame.inventory_completeness != Completeness::Complete
                || frame.saw_indeterminate
            {
                conflict.indeterminate = true;
            }
        }

        Ok(Some(conflict))
    }
}

// ============================================================================
// STATE MUTATION
// ============================================================================

fn add_declaration(
    state: &mut SearchState,
    parent: Option<DependencyName>,
    declaration: DependencyDeclaration,
    include_optional: bool,
) -> Result<()> {
    declaration.validate()?;

    if declaration.is_optional() && !include_optional {
        return Ok(());
    }

    let name = declaration.name().clone();

    let pending = state.pending.entry(name.clone()).or_default();

    if !pending.declarations.contains(&declaration) {
        for feature in declaration.requested_features() {
            pending.requested_features.insert(
                DependencyFeature::new(feature.clone())?,
            );
        }

        pending.declarations.push(declaration);
    }

    if parent.is_none() {
        state.roots.insert(name.clone());
    }

    state.edges.insert(DependencyEdge {
        from: parent,
        to: name,
    });

    Ok(())
}

fn apply_candidate(
    state: &mut SearchState,
    choice: CandidateChoice,
    inventory_completeness: Completeness,
    include_optional: bool,
) -> Result<()> {
    let name = choice.candidate.name().clone();

    if state.selected.contains_key(&name) {
        return Err(
            ToolchainError::internal(
                "attempted to select a package that is already selected",
            )
            .with_operation(OPERATION_RESOLVE)
            .with_subject(name.to_string()),
        );
    }

    let pending = state.pending.get(&name).ok_or_else(|| {
        ToolchainError::internal(
            "candidate selection has no pending dependency declaration",
        )
        .with_operation(OPERATION_RESOLVE)
        .with_subject(name.to_string())
    })?;

    let selected = SelectedCandidate {
        reference: choice.reference,
        candidate: choice.candidate.clone(),
        expanded_features: BTreeSet::new(),
        validated_declaration_count: pending.declarations.len(),
        inventory_completeness,
    };

    state.selected.insert(name.clone(), selected);

    for dependency in choice.candidate.dependencies() {
        add_declaration(
            state,
            Some(name.clone()),
            dependency.clone(),
            include_optional,
        )?;
    }

    Ok(())
}

fn preference_score(
    report: &super::constraints::ConstraintReport,
) -> (usize, usize) {
    let mut preferences = 0_usize;
    let mut hints = 0_usize;

    for result in report.results() {
        if result.evaluation() != ConstraintEvaluation::Satisfied {
            continue;
        }

        match result.constraint().kind() {
            ConstraintKind::Preference => {
                preferences = preferences.saturating_add(1);
            }

            ConstraintKind::Hint => {
                hints = hints.saturating_add(1);
            }

            ConstraintKind::Requirement | ConstraintKind::Constraint => {}
        }
    }

    (preferences, hints)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::constraints::{
        CandidateVersion,
        DependencyCandidateFacts,
        DependencyConstraint,
        DependencyFeature,
        EvidenceSet,
        VersionConstraintEvaluator,
        VersionSchemeId,
        ConstraintPredicate,
        VersionRequirement,
    };

    use std::collections::BTreeMap;

    fn package(value: &str) -> DependencyName {
        DependencyName::new(value).expect("valid package name")
    }

    fn provider_id(value: &str) -> CandidateProviderId {
        CandidateProviderId::new(value).expect("valid provider identity")
    }

    fn candidate_id(value: &str) -> CandidateId {
        CandidateId::new(value).expect("valid candidate identity")
    }

    fn scheme() -> VersionSchemeId {
        VersionSchemeId::new("test.exact")
            .expect("valid version scheme")
    }

    fn candidate(
        id: &str,
        name: &str,
        version: &str,
        dependencies: Vec<DependencyDeclaration>,
    ) -> DependencyCandidate {
        let facts = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(scheme(), version)
                    .expect("valid candidate version"),
            )
            .without_source()
            .with_feature_inventory(
                EvidenceSet::complete(
                    Vec::<DependencyFeature>::new(),
                ),
            );

        DependencyCandidate::new(
            candidate_id(id),
            package(name),
            facts,
            dependencies,
            BTreeMap::new(),
        )
        .expect("valid dependency candidate")
    }

    #[derive(Clone)]
    struct StaticProvider {
        inventories: BTreeMap<String, CandidateInventory>,
    }

    impl DependencyCandidateProvider for StaticProvider {
        fn discover(
            &self,
            query: &CandidateQuery,
        ) -> Result<Option<CandidateInventory>> {
            Ok(self
                .inventories
                .get(query.package().as_str())
                .cloned())
        }
    }

    fn registry(
        inventories: impl IntoIterator<Item = (String, CandidateInventory)>,
    ) -> CandidateProviderRegistry {
        let provider = StaticProvider {
            inventories: inventories.into_iter().collect(),
        };

        let mut registry = CandidateProviderRegistry::new();

        registry
            .register(
                provider_id("test.provider"),
                0,
                provider,
            )
            .expect("provider registration");

        registry
    }

    struct ExactVersionEvaluator;

    impl VersionConstraintEvaluator for ExactVersionEvaluator {
        fn evaluate(
            &self,
            requirement: &VersionRequirement,
            candidate: &CandidateVersion,
        ) -> Result<ConstraintEvaluation> {
            Ok(
                if requirement.as_str() == candidate.value() {
                    ConstraintEvaluation::Satisfied
                } else {
                    ConstraintEvaluation::Violated
                },
            )
        }
    }

    fn evaluators() -> ConstraintEvaluatorRegistry {
        let mut evaluators = ConstraintEvaluatorRegistry::new();

        evaluators
            .register_version_evaluator(
                scheme(),
                ExactVersionEvaluator,
            )
            .expect("version evaluator registration");

        evaluators
    }

    #[test]
    fn complete_empty_inventory_proves_no_candidates() {
        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                Vec::new(),
                Completeness::Complete,
            )
            .expect("complete inventory"),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let conflict = outcome.conflict().expect("expected conflict");

        assert_eq!(
            conflict.diagnostic().code(),
            ResolutionDiagnosticCode::NoCandidates,
        );

        assert!(!conflict.is_indeterminate());
    }

    #[test]
    fn partial_empty_inventory_is_indeterminate() {
        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                Vec::new(),
                Completeness::Partial,
            )
            .expect("partial inventory"),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let conflict = outcome.conflict().expect("expected conflict");

        assert_eq!(
            conflict.diagnostic().code(),
            ResolutionDiagnosticCode::CandidateEvidenceIncomplete,
        );

        assert!(conflict.is_indeterminate());
    }

    #[test]
    fn unavailable_provider_evidence_never_proves_absence() {
        let provider = CandidateProviderRegistry::new();
        let evaluators = ConstraintEvaluatorRegistry::new();

        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        assert!(
            outcome
                .conflict()
                .expect("expected conflict")
                .is_indeterminate()
        );
    }

    #[test]
    fn transitive_dependency_is_resolved() {
        let root_candidate = candidate(
            "root-release",
            "root",
            "1",
            vec![
                DependencyDeclaration::named("child").unwrap(),
            ],
        );

        let child_candidate = candidate(
            "child-release",
            "child",
            "1",
            Vec::new(),
        );

        let provider = registry([
            (
                "root".to_owned(),
                CandidateInventory::new(
                    vec![root_candidate],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
            (
                "child".to_owned(),
                CandidateInventory::new(
                    vec![child_candidate],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
        ]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let graph = outcome.graph().expect("successful resolution");

        assert_eq!(graph.len(), 2);
        assert!(graph.package(&package("child")).is_some());
        assert_eq!(graph.edges().len(), 2);
    }

    #[test]
    fn backtracking_finds_a_compatible_transitive_solution() {
        let first_root = candidate(
            "root-first",
            "root",
            "1",
            vec![
                DependencyDeclaration::named("child")
                    .unwrap()
                    .with_version(
                        VersionRequirement::new("not-selected")
                            .unwrap(),
                    ),
            ],
        );

        let second_root = candidate(
            "root-second",
            "root",
            "2",
            vec![
                DependencyDeclaration::named("child")
                    .unwrap()
                    .with_version(
                        VersionRequirement::new("selected")
                            .unwrap(),
                    ),
            ],
        );

        let child = candidate(
            "child-selected",
            "child",
            "selected",
            Vec::new(),
        );

        let provider = registry([
            (
                "root".to_owned(),
                CandidateInventory::new(
                    vec![first_root, second_root],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
            (
                "child".to_owned(),
                CandidateInventory::new(
                    vec![child],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
        ]);

        let evaluators = evaluators();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let graph = outcome.graph().expect("backtracking should find solution");

        assert_eq!(
            graph
                .package(&package("root"))
                .unwrap()
                .candidate()
                .id()
                .as_str(),
            "root-second",
        );
    }

    #[test]
    fn dependency_cycles_are_reported() {
        let root = candidate(
            "root-release",
            "root",
            "1",
            vec![
                DependencyDeclaration::named("child").unwrap(),
            ],
        );

        let child = candidate(
            "child-release",
            "child",
            "1",
            vec![
                DependencyDeclaration::named("root").unwrap(),
            ],
        );

        let provider = registry([
            (
                "root".to_owned(),
                CandidateInventory::new(
                    vec![root],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
            (
                "child".to_owned(),
                CandidateInventory::new(
                    vec![child],
                    Completeness::Complete,
                )
                .unwrap(),
            ),
        ]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let conflict = outcome.conflict().expect("cycle must be rejected");

        assert_eq!(
            conflict.diagnostic().code(),
            ResolutionDiagnosticCode::DependencyCycle,
        );

        assert!(!conflict.diagnostic().cycle_path().is_empty());
    }

    #[test]
    fn optional_dependencies_are_excluded_by_default() {
        let provider = CandidateProviderRegistry::new();
        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let optional = DependencyDeclaration::named("optional")
            .unwrap()
            .optional(true);

        let outcome = resolver
            .resolve(
                [optional],
                &ResolutionOptions::new(),
                None,
            )
            .expect("optional dependency should be skipped");

        assert!(outcome.graph().unwrap().is_empty());
    }

    #[test]
    fn frozen_lock_requires_a_lock_entry() {
        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                vec![candidate("root-release", "root", "1", Vec::new())],
                Completeness::Complete,
            )
            .unwrap(),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new().with_lock_mode(LockMode::Frozen),
                Some(&ResolutionLock::new()),
            )
            .expect("resolution operation");

        assert_eq!(
            outcome
                .conflict()
                .unwrap()
                .diagnostic()
                .code(),
            ResolutionDiagnosticCode::LockEntryMissing,
        );
    }

    #[test]
    fn resolved_graph_produces_lock_snapshot() {
        let root = candidate("root-release", "root", "1", Vec::new());

        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                vec![root],
                Completeness::Complete,
            )
            .unwrap(),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &ResolutionOptions::new(),
                None,
            )
            .expect("resolution operation");

        let graph = outcome.graph().unwrap();
        let lock = graph.to_lock();

        assert_eq!(lock.len(), 1);
        assert!(lock.get(&package("root")).is_some());
    }

    #[test]
    fn preferences_rank_candidates_without_rejecting_them() {
        let preferred_feature =
            DependencyFeature::new("preferred").unwrap();

        let plain_facts = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(scheme(), "1").unwrap(),
            )
            .without_source()
            .with_feature_inventory(
                EvidenceSet::complete(
                    Vec::<DependencyFeature>::new(),
                ),
            );

        let preferred_facts = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(scheme(), "2").unwrap(),
            )
            .without_source()
            .with_feature_inventory(
                EvidenceSet::complete([
                    preferred_feature.clone(),
                ]),
            );

        let plain = DependencyCandidate::new(
            candidate_id("plain"),
            package("root"),
            plain_facts,
            Vec::new(),
            BTreeMap::new(),
        )
        .unwrap();

        let preferred = DependencyCandidate::new(
            candidate_id("preferred"),
            package("root"),
            preferred_facts,
            Vec::new(),
            BTreeMap::new(),
        )
        .unwrap();

        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                vec![plain, preferred],
                Completeness::Complete,
            )
            .unwrap(),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();

        let mut options = ResolutionOptions::new();

        options
            .add_constraint(
                package("root"),
                DependencyConstraint::preference(
                    ConstraintPredicate::feature(preferred_feature),
                )
                .unwrap(),
            )
            .unwrap();

        let resolver = DependencyResolver::new(&provider, &evaluators);

        let outcome = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &options,
                None,
            )
            .unwrap();

        assert_eq!(
            outcome
                .graph()
                .unwrap()
                .package(&package("root"))
                .unwrap()
                .candidate()
                .id()
                .as_str(),
            "preferred",
        );
    }

    #[test]
    fn provider_registry_rejects_duplicate_provider_identity() {
        let provider = StaticProvider {
            inventories: BTreeMap::new(),
        };

        let mut registry = CandidateProviderRegistry::new();

        registry
            .register(
                provider_id("same"),
                0,
                provider.clone(),
            )
            .unwrap();

        assert!(
            registry
                .register(
                    provider_id("same"),
                    1,
                    provider,
                )
                .is_err()
        );
    }

    #[test]
    fn candidate_evaluation_budget_is_enforced() {
        let provider = registry([(
            "root".to_owned(),
            CandidateInventory::new(
                vec![candidate("root-release", "root", "1", Vec::new())],
                Completeness::Complete,
            )
            .unwrap(),
        )]);

        let evaluators = ConstraintEvaluatorRegistry::new();
        let resolver = DependencyResolver::new(&provider, &evaluators);

        let options = ResolutionOptions::new().with_budget(
            ResolutionBudget::unbounded()
                .with_max_candidate_evaluations(Some(0)),
        );

        let error = resolver
            .resolve(
                [DependencyDeclaration::named("root").unwrap()],
                &options,
                None,
            )
            .expect_err("candidate budget must be enforced");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ResourceExhausted,
        );
    }
}
