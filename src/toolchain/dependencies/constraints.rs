
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Constraints
//!
//! Path: src/toolchain/dependencies/constraints.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the extensible constraint model and evaluation boundary used
//! by the Zamani package dependency subsystem.
//!
//! This module evaluates dependency requirements against explicitly
//! supplied candidate facts. It does not discover candidates, resolve
//! dependencies, download packages, access registries, or select targets.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Dependency constraint classifications.
//! - Dependency constraint predicates.
//! - Dependency constraint collections.
//! - Candidate fact and evidence representations.
//! - Version evaluator registration.
//! - Compatibility evaluator registration.
//! - Extension evaluator registration.
//! - Constraint evaluation reports.
//! - Mandatory constraint aggregation.
//!
//! It does not own:
//!
//! - Source-language constraint syntax.
//! - General semantic constraint evaluation.
//! - Canonical dependency declarations.
//! - Version comparison algorithms.
//! - Registry access.
//! - Dependency candidate discovery.
//! - Dependency graph construction.
//! - Feature activation or unification.
//! - Package downloading or installation.
//! - Hardware capability discovery.
//! - Target selection.
//! - Physical resource allocation.
//! - Compiler IR.
//! - Runtime scheduling.
//!
//! ## Canonical authorities
//!
//! Dependency declarations:
//! `super::model`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Toolchain results:
//! `crate::toolchain::result`
//!
//! Source-language constraint syntax:
//! `grammar/core/constraints.g4`
//!
//! Dependency declaration syntax:
//! `grammar/modules/dependencies.g4`
//!
//! Resource and capability semantics:
//! The canonical semantic/resource/capability authorities.
//!
//! Compatibility:
//! `grammar/compatibility/` and its corresponding semantic contracts.
//!
//! ## Evaluation semantics
//!
//! The evaluator uses three outcomes:
//!
//! - Satisfied: sufficient evidence establishes the predicate.
//! - Violated: sufficient evidence establishes that the predicate fails.
//! - Indeterminate: available evidence cannot establish either result.
//!
//! Unknown evidence MUST NOT be interpreted as satisfaction.
//!
//! A complete inventory can establish absence. A partial or unavailable
//! inventory cannot establish absence.
//!
//! A known present fact may satisfy a predicate even when its containing
//! inventory is incomplete.
//!
//! ## Constraint classification
//!
//! Requirement:
//! A mandatory property that must be established.
//!
//! Constraint:
//! A mandatory condition that must not be violated.
//!
//! Preference:
//! A non-mandatory property that may influence candidate ranking.
//!
//! Hint:
//! Non-authoritative information that may guide optimization.
//!
//! Preferences and hints MUST NOT reject an otherwise admissible candidate.
//!
//! Policies and authorization remain separate authorities.
//!
//! ## Versioning
//!
//! Version expressions remain opaque until evaluated by an explicitly
//! registered version-scheme implementation.
//!
//! This module does not assume SemVer, numeric ordering, fixed version
//! components, or a universal comparison algorithm.
//!
//! An absent version-scheme evaluator produces an indeterminate result.
//!
//! ## Portability and POCO-REAF
//!
//! Dependency constraints express portable package requirements.
//!
//! They MUST NOT identify physical CPUs, GPUs, FPGAs, ASICs, QPUs,
//! specific machines, or fixed hardware inventories.
//!
//! New computing domains and dependency-source providers must integrate
//! through existing extensible contracts.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Dependencies.
//! - Constraints.
//! - Features.
//! - Capabilities.
//! - Evaluators.
//! - Compatibility namespaces.
//! - Extension namespaces.
//! - Version components.
//! - Computing domains.
//!
//! Collections grow according to available memory and allocator capacity.
//! Actual operations remain subject to available resources and explicitly
//! configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Ordered collections are used wherever ordering is observable.
//!
//! Constraint evaluation follows deterministic constraint-set ordering.
//!
//! Provider implementations are responsible for deterministic evaluation
//! of equivalent inputs when their contracts require reproducibility.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! hardware, or credential access.
//!
//! External facts and evaluator implementations must be validated by
//! their respective authoritative subsystems before registration or use.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::Result;

use super::model::{
    DependencyCapability,
    DependencyDeclaration,
    DependencySource,
    VersionRequirement,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_CONSTRAINT: &str = "dependency.constraint";

const OPERATION_EVALUATION: &str = "dependency.constraint.evaluate";

const OPERATION_REGISTRATION: &str = "dependency.constraint.register";

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_CONSTRAINT),
        );
    }

    if value.trim() != value {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain leading or trailing whitespace"
                ),
            )
            .with_operation(OPERATION_CONSTRAINT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not contain control characters"),
            )
            .with_operation(OPERATION_CONSTRAINT),
        );
    }

    Ok(())
}

fn validate_expression(
    value: &str,
    field: &'static str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_CONSTRAINT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not contain control characters"),
            )
            .with_operation(OPERATION_CONSTRAINT),
        );
    }

    Ok(())
}

// ============================================================================
// VERSION SCHEME IDENTITY
// ============================================================================

/// Extensible identifier for a package version-comparison scheme.
///
/// Examples include registered ecosystem-specific schemes. The identifier
/// does not imply any particular version syntax or comparison algorithm.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionSchemeId(String);

impl VersionSchemeId {
    /// Construct a validated version-scheme identifier.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "version scheme")?;

        Ok(Self(value))
    }

    /// Borrow the version-scheme identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VersionSchemeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CONSTRAINT NAMESPACE
// ============================================================================

/// Extensible namespace for compatibility and extension evaluators.
///
/// Namespaces identify semantic contracts, not physical devices or vendors.
/// Their interpretation belongs to registered evaluator implementations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstraintNamespace(String);

impl ConstraintNamespace {
    /// Construct a validated constraint namespace.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "constraint namespace")?;

        Ok(Self(value))
    }

    /// Borrow the namespace identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConstraintNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DEPENDENCY FEATURE
// ============================================================================

/// Validated dependency feature identifier.
///
/// The canonical source-language qualified-name grammar remains
/// authoritative for source syntax.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyFeature(String);

impl DependencyFeature {
    /// Construct a validated dependency feature.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_identifier(&value, "dependency feature")?;

        Ok(Self(value))
    }

    /// Borrow the feature identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencyFeature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DEPENDENCY SOURCE CONSTRAINT
// ============================================================================

/// A dependency source constraint using exact declared-source equality.
///
/// Source resolution, path canonicalization, registry lookup, trust and
/// authorization remain outside this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencySourceConstraint {
    expected: DependencySource,
}

impl DependencySourceConstraint {
    /// Construct an exact source constraint.
    #[must_use]
    pub fn exact(expected: DependencySource) -> Self {
        Self { expected }
    }

    /// Borrow the expected dependency source.
    #[must_use]
    pub fn expected(&self) -> &DependencySource {
        &self.expected
    }
}

// ============================================================================
// CONSTRAINT CLASSIFICATION
// ============================================================================

/// Semantic classification of a dependency constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConstraintKind {
    /// A mandatory property that must be established.
    Requirement,

    /// A mandatory condition that must not be violated.
    Constraint,

    /// A desirable property that may influence candidate ranking.
    Preference,

    /// Non-authoritative optimization information.
    Hint,
}

impl ConstraintKind {
    /// Return whether this classification is mandatory.
    #[must_use]
    pub const fn is_mandatory(self) -> bool {
        matches!(
            self,
            Self::Requirement | Self::Constraint
        )
    }
}

// ============================================================================
// CONSTRAINT PREDICATE
// ============================================================================

/// Extensible predicate evaluated against dependency candidate facts.
///
/// Compatibility and extension expressions are opaque at this boundary.
/// Their interpretation belongs to registered evaluators.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ConstraintPredicate {
    /// A package version requirement.
    ///
    /// `scheme: None` means that the governing version scheme has not
    /// been established. Evaluation must therefore remain indeterminate.
    Version {
        /// Original version requirement.
        requirement: VersionRequirement,

        /// Explicit comparison scheme, when established.
        scheme: Option<VersionSchemeId>,
    },

    /// An exact dependency-source requirement.
    Source(DependencySourceConstraint),

    /// A required package feature.
    Feature(DependencyFeature),

    /// A required package capability.
    Capability(DependencyCapability),

    /// A compatibility expression evaluated by a registered compatibility
    /// evaluator.
    Compatibility {
        /// Compatibility contract namespace.
        namespace: ConstraintNamespace,

        /// Opaque compatibility expression.
        expression: String,
    },

    /// An extension expression evaluated by a registered extension
    /// evaluator.
    Extension {
        /// Extension namespace.
        namespace: ConstraintNamespace,

        /// Opaque extension expression.
        expression: String,
    },
}

impl ConstraintPredicate {
    /// Construct a version predicate without assuming a default scheme.
    #[must_use]
    pub fn version(
        requirement: VersionRequirement,
        scheme: Option<VersionSchemeId>,
    ) -> Self {
        Self::Version {
            requirement,
            scheme,
        }
    }

    /// Construct an exact source predicate.
    #[must_use]
    pub fn source(source: DependencySource) -> Self {
        Self::Source(
            DependencySourceConstraint::exact(source),
        )
    }

    /// Construct a feature predicate.
    #[must_use]
    pub fn feature(feature: DependencyFeature) -> Self {
        Self::Feature(feature)
    }

    /// Construct a capability predicate.
    #[must_use]
    pub fn capability(capability: DependencyCapability) -> Self {
        Self::Capability(capability)
    }

    /// Construct a compatibility predicate.
    pub fn compatibility(
        namespace: ConstraintNamespace,
        expression: impl Into<String>,
    ) -> Result<Self> {
        let expression = expression.into();

        validate_expression(
            &expression,
            "compatibility expression",
        )?;

        Ok(Self::Compatibility {
            namespace,
            expression,
        })
    }

    /// Construct an extensible predicate.
    pub fn extension(
        namespace: ConstraintNamespace,
        expression: impl Into<String>,
    ) -> Result<Self> {
        let expression = expression.into();

        validate_expression(
            &expression,
            "extension expression",
        )?;

        Ok(Self::Extension {
            namespace,
            expression,
        })
    }
}

fn validate_predicate(
    predicate: &ConstraintPredicate,
) -> Result<()> {
    match predicate {
        ConstraintPredicate::Version {
            requirement,
            scheme,
        } => {
            validate_identifier(
                requirement.as_str(),
                "version requirement",
            )?;

            if let Some(scheme) = scheme {
                validate_identifier(
                    scheme.as_str(),
                    "version scheme",
                )?;
            }
        }

        ConstraintPredicate::Source(_) => {}

        ConstraintPredicate::Feature(feature) => {
            validate_identifier(
                feature.as_str(),
                "dependency feature",
            )?;
        }

        ConstraintPredicate::Capability(capability) => {
            validate_identifier(
                capability.as_str(),
                "dependency capability",
            )?;
        }

        ConstraintPredicate::Compatibility {
            namespace,
            expression,
        }
        | ConstraintPredicate::Extension {
            namespace,
            expression,
        } => {
            validate_identifier(
                namespace.as_str(),
                "constraint namespace",
            )?;

            validate_expression(
                expression,
                "constraint expression",
            )?;
        }
    }

    Ok(())
}

// ============================================================================
// DEPENDENCY CONSTRAINT
// ============================================================================

/// One validated dependency constraint.
///
/// The predicate and classification are immutable after construction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyConstraint {
    kind: ConstraintKind,
    predicate: ConstraintPredicate,
}

impl DependencyConstraint {
    /// Construct a validated dependency constraint.
    pub fn new(
        kind: ConstraintKind,
        predicate: ConstraintPredicate,
    ) -> Result<Self> {
        validate_predicate(&predicate)?;

        Ok(Self {
            kind,
            predicate,
        })
    }

    /// Construct a mandatory requirement.
    pub fn requirement(
        predicate: ConstraintPredicate,
    ) -> Result<Self> {
        Self::new(ConstraintKind::Requirement, predicate)
    }

    /// Construct a mandatory constraint.
    pub fn constraint(
        predicate: ConstraintPredicate,
    ) -> Result<Self> {
        Self::new(ConstraintKind::Constraint, predicate)
    }

    /// Construct a non-mandatory preference.
    pub fn preference(
        predicate: ConstraintPredicate,
    ) -> Result<Self> {
        Self::new(ConstraintKind::Preference, predicate)
    }

    /// Construct a non-authoritative hint.
    pub fn hint(
        predicate: ConstraintPredicate,
    ) -> Result<Self> {
        Self::new(ConstraintKind::Hint, predicate)
    }

    /// Return the constraint classification.
    #[must_use]
    pub const fn kind(&self) -> ConstraintKind {
        self.kind
    }

    /// Borrow the predicate.
    #[must_use]
    pub fn predicate(&self) -> &ConstraintPredicate {
        &self.predicate
    }
}

// ============================================================================
// CONSTRAINT COLLECTION
// ============================================================================

/// Deterministically ordered dependency constraints.
///
/// Duplicate constraints are rejected instead of being silently discarded.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DependencyConstraintSet {
    constraints: BTreeSet<DependencyConstraint>,
}

impl DependencyConstraintSet {
    /// Construct an empty constraint set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert one constraint.
    ///
    /// Duplicate insertion returns an error.
    pub fn insert(
        &mut self,
        constraint: DependencyConstraint,
    ) -> Result<()> {
        if !self.constraints.insert(constraint.clone()) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "duplicate dependency constraint",
                )
                .with_operation(OPERATION_CONSTRAINT),
            );
        }

        Ok(())
    }

    /// Return the number of stored constraints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Return whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Iterate over constraints in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &DependencyConstraint> {
        self.constraints.iter()
    }

    /// Construct mandatory constraints from a canonical dependency
    /// declaration.
    ///
    /// A version scheme is optional because the manifest model preserves
    /// version expressions without assuming SemVer or another scheme.
    ///
    /// When a version exists but its scheme is unknown, the resulting
    /// predicate remains indeterminate until an explicit scheme is supplied.
    pub fn from_declaration(
        declaration: &DependencyDeclaration,
        version_scheme: Option<VersionSchemeId>,
    ) -> Result<Self> {
        declaration.validate()?;

        let mut constraints = Self::new();

        if let Some(version) = declaration.version() {
            constraints.insert(
                DependencyConstraint::constraint(
                    ConstraintPredicate::version(
                        version.clone(),
                        version_scheme,
                    ),
                )?,
            )?;
        }

        if let Some(source) = declaration.source() {
            constraints.insert(
                DependencyConstraint::constraint(
                    ConstraintPredicate::source(source.clone()),
                )?,
            )?;
        }

        for feature in declaration.requested_features() {
            constraints.insert(
                DependencyConstraint::requirement(
                    ConstraintPredicate::feature(
                        DependencyFeature::new(feature.clone())?,
                    ),
                )?,
            )?;
        }

        for capability in declaration.required_capabilities() {
            constraints.insert(
                DependencyConstraint::requirement(
                    ConstraintPredicate::capability(
                        capability.clone(),
                    ),
                )?,
            )?;
        }

        Ok(constraints)
    }
}

// ============================================================================
// EVIDENCE COMPLETENESS
// ============================================================================

/// Completeness classification of an observed candidate inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Completeness {
    /// The inventory is complete for its declared scope.
    Complete,

    /// The inventory contains some facts but may omit others.
    Partial,

    /// No usable inventory is available.
    Unavailable,
}

// ============================================================================
// INDIVIDUAL FACT EVIDENCE
// ============================================================================

/// Evidence for an individual fact.
///
/// `Known(None)` and `Unknown` are deliberately different:
///
/// - `Known(None)` establishes that the value is absent.
/// - `Unknown` means its presence or absence has not been established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evidence<T> {
    /// The fact is known, including an explicitly known absence.
    Known(T),

    /// The fact is not established.
    Unknown,
}

// ============================================================================
// SET EVIDENCE
// ============================================================================

/// An inventory of facts together with its completeness.
///
/// A missing value in a complete inventory is absent.
/// A missing value in a partial or unavailable inventory is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSet<T> {
    values: BTreeSet<T>,
    completeness: Completeness,
}

impl<T: Ord> EvidenceSet<T> {
    /// Construct a complete inventory.
    pub fn complete(
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        Self {
            values: values.into_iter().collect(),
            completeness: Completeness::Complete,
        }
    }

    /// Construct a partial inventory.
    pub fn partial(
        values: impl IntoIterator<Item = T>,
    ) -> Self {
        Self {
            values: values.into_iter().collect(),
            completeness: Completeness::Partial,
        }
    }

    /// Construct an unavailable inventory.
    #[must_use]
    pub fn unavailable() -> Self {
        Self {
            values: BTreeSet::new(),
            completeness: Completeness::Unavailable,
        }
    }

    /// Test whether a fact is present in the observed inventory.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    /// Borrow the observed values.
    #[must_use]
    pub fn values(&self) -> &BTreeSet<T> {
        &self.values
    }

    /// Return the inventory completeness.
    #[must_use]
    pub const fn completeness(&self) -> Completeness {
        self.completeness
    }

    /// Return whether the inventory can establish absence.
    #[must_use]
    pub const fn establishes_absence(&self) -> bool {
        matches!(
            self.completeness,
            Completeness::Complete
        )
    }
}

// ============================================================================
// METADATA EVIDENCE
// ============================================================================

/// Candidate metadata accompanied by completeness information.
///
/// Extension evaluators may inspect this data without making metadata
/// interpretation part of the generic dependency model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceMap {
    values: BTreeMap<String, String>,
    completeness: Completeness,
}

impl EvidenceMap {
    /// Construct a complete metadata inventory.
    #[must_use]
    pub fn complete(values: BTreeMap<String, String>) -> Self {
        Self {
            values,
            completeness: Completeness::Complete,
        }
    }

    /// Construct a partial metadata inventory.
    #[must_use]
    pub fn partial(values: BTreeMap<String, String>) -> Self {
        Self {
            values,
            completeness: Completeness::Partial,
        }
    }

    /// Construct an unavailable metadata inventory.
    #[must_use]
    pub fn unavailable() -> Self {
        Self {
            values: BTreeMap::new(),
            completeness: Completeness::Unavailable,
        }
    }

    /// Look up an observed metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// Borrow all observed metadata.
    #[must_use]
    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    /// Return metadata completeness.
    #[must_use]
    pub const fn completeness(&self) -> Completeness {
        self.completeness
    }
}

// ============================================================================
// CANDIDATE VERSION
// ============================================================================

/// A candidate package version with its explicit comparison scheme.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CandidateVersion {
    scheme: VersionSchemeId,
    value: String,
}

impl CandidateVersion {
    /// Construct validated candidate version evidence.
    pub fn new(
        scheme: VersionSchemeId,
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "candidate package version",
        )?;

        Ok(Self {
            scheme,
            value,
        })
    }

    /// Borrow the version scheme.
    #[must_use]
    pub fn scheme(&self) -> &VersionSchemeId {
        &self.scheme
    }

    /// Borrow the original candidate version.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// DEPENDENCY CANDIDATE FACTS
// ============================================================================

/// Explicit facts established about one dependency candidate.
///
/// Candidate facts are supplied by the resolver and authoritative
/// dependency-source or registry providers.
///
/// A default instance represents unavailable evidence, not an empty
/// candidate with proven absence of every property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCandidateFacts {
    version: Evidence<Option<CandidateVersion>>,
    source: Evidence<Option<DependencySource>>,
    features: EvidenceSet<DependencyFeature>,
    capabilities: EvidenceSet<DependencyCapability>,
    metadata: EvidenceMap,
}

impl Default for DependencyCandidateFacts {
    fn default() -> Self {
        Self {
            version: Evidence::Unknown,
            source: Evidence::Unknown,
            features: EvidenceSet::unavailable(),
            capabilities: EvidenceSet::unavailable(),
            metadata: EvidenceMap::unavailable(),
        }
    }
}

impl DependencyCandidateFacts {
    /// Construct candidate facts with all evidence initially unknown.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a known candidate version.
    #[must_use]
    pub fn with_version(
        mut self,
        version: CandidateVersion,
    ) -> Self {
        self.version = Evidence::Known(Some(version));
        self
    }

    /// Record that the candidate has no version information.
    #[must_use]
    pub fn without_version(mut self) -> Self {
        self.version = Evidence::Known(None);
        self
    }

    /// Mark candidate version evidence as unknown.
    #[must_use]
    pub fn with_unknown_version(mut self) -> Self {
        self.version = Evidence::Unknown;
        self
    }

    /// Record a known candidate source.
    #[must_use]
    pub fn with_source(
        mut self,
        source: DependencySource,
    ) -> Self {
        self.source = Evidence::Known(Some(source));
        self
    }

    /// Record that the candidate has no declared source.
    #[must_use]
    pub fn without_source(mut self) -> Self {
        self.source = Evidence::Known(None);
        self
    }

    /// Mark candidate source evidence as unknown.
    #[must_use]
    pub fn with_unknown_source(mut self) -> Self {
        self.source = Evidence::Unknown;
        self
    }

    /// Supply a feature inventory with explicit completeness.
    #[must_use]
    pub fn with_feature_inventory(
        mut self,
        features: EvidenceSet<DependencyFeature>,
    ) -> Self {
        self.features = features;
        self
    }

    /// Supply a capability inventory with explicit completeness.
    #[must_use]
    pub fn with_capability_inventory(
        mut self,
        capabilities: EvidenceSet<DependencyCapability>,
    ) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Supply metadata with explicit completeness.
    #[must_use]
    pub fn with_metadata_inventory(
        mut self,
        metadata: EvidenceMap,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Borrow version evidence.
    #[must_use]
    pub fn version(
        &self,
    ) -> &Evidence<Option<CandidateVersion>> {
        &self.version
    }

    /// Borrow source evidence.
    #[must_use]
    pub fn source(
        &self,
    ) -> &Evidence<Option<DependencySource>> {
        &self.source
    }

    /// Borrow feature evidence.
    #[must_use]
    pub fn features(
        &self,
    ) -> &EvidenceSet<DependencyFeature> {
        &self.features
    }

    /// Borrow capability evidence.
    #[must_use]
    pub fn capabilities(
        &self,
    ) -> &EvidenceSet<DependencyCapability> {
        &self.capabilities
    }

    /// Borrow metadata evidence.
    #[must_use]
    pub fn metadata(&self) -> &EvidenceMap {
        &self.metadata
    }
}

// ============================================================================
// EVALUATION OUTCOMES
// ============================================================================

/// Result of evaluating one dependency predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintEvaluation {
    /// Sufficient evidence establishes satisfaction.
    Satisfied,

    /// Sufficient evidence establishes a violation.
    Violated,

    /// Available evidence cannot establish satisfaction or violation.
    Indeterminate,
}

fn evaluation_description(
    evaluation: ConstraintEvaluation,
) -> &'static str {
    match evaluation {
        ConstraintEvaluation::Satisfied => {
            "constraint satisfied"
        }

        ConstraintEvaluation::Violated => {
            "constraint violated"
        }

        ConstraintEvaluation::Indeterminate => {
            "constraint could not be established"
        }
    }
}

/// Aggregate decision for a dependency candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintDecision {
    /// Every mandatory constraint was established as satisfied.
    Satisfied,

    /// At least one mandatory constraint was established as violated.
    Violated,

    /// No mandatory violation was established, but at least one
    /// mandatory constraint remains indeterminate.
    Indeterminate,
}

// ============================================================================
// VERSION EVALUATOR
// ============================================================================

/// Provider contract for interpreting a registered version scheme.
///
/// Implementations own their version syntax and comparison semantics.
/// This module does not supply a universal SemVer implementation.
pub trait VersionConstraintEvaluator: Send + Sync {
    /// Evaluate one requirement against one candidate version.
    fn evaluate(
        &self,
        requirement: &VersionRequirement,
        candidate: &CandidateVersion,
    ) -> Result<ConstraintEvaluation>;
}

// ============================================================================
// COMPATIBILITY AND EXTENSION EVALUATOR
// ============================================================================

/// Provider contract for a namespaced compatibility or extension
/// expression.
///
/// The provider receives the complete candidate evidence so that it can
/// distinguish missing facts from proven absence.
pub trait NamedConstraintEvaluator: Send + Sync {
    /// Evaluate an opaque expression against candidate evidence.
    fn evaluate(
        &self,
        expression: &str,
        candidate: &DependencyCandidateFacts,
    ) -> Result<ConstraintEvaluation>;
}

// ============================================================================
// EVALUATOR REGISTRY
// ============================================================================

/// Thread-safe provider registry for version, compatibility and
/// extension constraint evaluators.
///
/// Registration is explicit. Duplicate registrations are rejected.
/// Unknown namespaces and schemes remain indeterminate.
#[derive(Default)]
pub struct ConstraintEvaluatorRegistry {
    version_evaluators:
        BTreeMap<VersionSchemeId, Arc<dyn VersionConstraintEvaluator>>,

    compatibility_evaluators:
        BTreeMap<ConstraintNamespace, Arc<dyn NamedConstraintEvaluator>>,

    extension_evaluators:
        BTreeMap<ConstraintNamespace, Arc<dyn NamedConstraintEvaluator>>,
}

impl ConstraintEvaluatorRegistry {
    /// Construct an empty evaluator registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a version evaluator.
    pub fn register_version_evaluator<E>(
        &mut self,
        scheme: VersionSchemeId,
        evaluator: E,
    ) -> Result<()>
    where
        E: VersionConstraintEvaluator + 'static,
    {
        if self.version_evaluators.contains_key(&scheme) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!(
                        "version evaluator already registered for '{scheme}'"
                    ),
                )
                .with_operation(OPERATION_REGISTRATION),
            );
        }

        self.version_evaluators.insert(
            scheme,
            Arc::new(evaluator),
        );

        Ok(())
    }

    /// Register a compatibility evaluator.
    pub fn register_compatibility_evaluator<E>(
        &mut self,
        namespace: ConstraintNamespace,
        evaluator: E,
    ) -> Result<()>
    where
        E: NamedConstraintEvaluator + 'static,
    {
        if self.compatibility_evaluators.contains_key(&namespace) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!(
                        "compatibility evaluator already registered \
                         for '{namespace}'"
                    ),
                )
                .with_operation(OPERATION_REGISTRATION),
            );
        }

        self.compatibility_evaluators.insert(
            namespace,
            Arc::new(evaluator),
        );

        Ok(())
    }

    /// Register an extensible constraint evaluator.
    pub fn register_extension_evaluator<E>(
        &mut self,
        namespace: ConstraintNamespace,
        evaluator: E,
    ) -> Result<()>
    where
        E: NamedConstraintEvaluator + 'static,
    {
        if self.extension_evaluators.contains_key(&namespace) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    format!(
                        "extension evaluator already registered \
                         for '{namespace}'"
                    ),
                )
                .with_operation(OPERATION_REGISTRATION),
            );
        }

        self.extension_evaluators.insert(
            namespace,
            Arc::new(evaluator),
        );

        Ok(())
    }

    /// Evaluate a complete constraint set against candidate facts.
    ///
    /// Mandatory evaluator failures propagate as errors.
    ///
    /// Preference and hint evaluator failures are preserved in their
    /// individual results as indeterminate evaluations. They cannot
    /// independently reject a candidate.
    pub fn evaluate(
        &self,
        constraints: &DependencyConstraintSet,
        candidate: &DependencyCandidateFacts,
    ) -> Result<ConstraintReport> {
        let mut results = Vec::with_capacity(
            constraints.len(),
        );

        for constraint in constraints.iter() {
            match self.evaluate_predicate(
                constraint.predicate(),
                candidate,
            ) {
                Ok((evaluation, explanation)) => {
                    results.push(ConstraintResult {
                        constraint: constraint.clone(),
                        evaluation,
                        explanation,
                        evaluator_error: None,
                    });
                }

                Err(error) if constraint.kind().is_mandatory() => {
                    return Err(
                        error.with_context(
                            format!(
                                "mandatory dependency constraint \
                                 evaluation failed: {:?}",
                                constraint.kind(),
                            ),
                        )
                        .with_context(OPERATION_EVALUATION),
                    );
                }

                Err(error) => {
                    let error = error.with_context(
                        format!(
                            "non-mandatory dependency constraint \
                             evaluation failed: {:?}",
                            constraint.kind(),
                        ),
                    );

                    results.push(ConstraintResult {
                        constraint: constraint.clone(),
                        evaluation:
                            ConstraintEvaluation::Indeterminate,
                        explanation:
                            "optional evaluator failed; candidate \
                             admission is not affected".to_owned(),
                        evaluator_error: Some(error),
                    });
                }
            }
        }

        Ok(ConstraintReport::new(results))
    }

    fn evaluate_predicate(
        &self,
        predicate: &ConstraintPredicate,
        candidate: &DependencyCandidateFacts,
    ) -> Result<(ConstraintEvaluation, String)> {
        match predicate {
            ConstraintPredicate::Version {
                requirement,
                scheme,
            } => {
                let Some(scheme) = scheme else {
                    return Ok((
                        ConstraintEvaluation::Indeterminate,
                        "version comparison scheme is unspecified".to_owned(),
                    ));
                };

                let version = match candidate.version() {
                    Evidence::Known(Some(version)) => version,

                    Evidence::Known(None) => {
                        return Ok((
                            ConstraintEvaluation::Violated,
                            "candidate has no declared version".to_owned(),
                        ));
                    }

                    Evidence::Unknown => {
                        return Ok((
                            ConstraintEvaluation::Indeterminate,
                            "candidate version evidence is unavailable"
                                .to_owned(),
                        ));
                    }
                };

                if version.scheme() != scheme {
                    return Ok((
                        ConstraintEvaluation::Indeterminate,
                        format!(
                            "candidate version scheme '{}' does not match \
                             requested scheme '{}'",
                            version.scheme(),
                            scheme,
                        ),
                    ));
                }

                let Some(evaluator) =
                    self.version_evaluators.get(scheme)
                else {
                    return Ok((
                        ConstraintEvaluation::Indeterminate,
                        format!(
                            "no evaluator is registered for version scheme \
                             '{scheme}'"
                        ),
                    ));
                };

                let evaluation = evaluator.evaluate(
                    requirement,
                    version,
                )?;

                Ok((
                    evaluation,
                    evaluation_description(evaluation).to_owned(),
                ))
            }

            ConstraintPredicate::Source(source_constraint) => {
                match candidate.source() {
                    Evidence::Known(Some(source)) => {
                        let evaluation =
                            if source == source_constraint.expected() {
                                ConstraintEvaluation::Satisfied
                            } else {
                                ConstraintEvaluation::Violated
                            };

                        Ok((
                            evaluation,
                            evaluation_description(evaluation).to_owned(),
                        ))
                    }

                    Evidence::Known(None) => Ok((
                        ConstraintEvaluation::Violated,
                        "candidate has no declared source".to_owned(),
                    )),

                    Evidence::Unknown => Ok((
                        ConstraintEvaluation::Indeterminate,
                        "candidate source evidence is unavailable".to_owned(),
                    )),
                }
            }

            ConstraintPredicate::Feature(feature) => {
                if candidate.features().contains(feature) {
                    return Ok((
                        ConstraintEvaluation::Satisfied,
                        format!("feature '{feature}' is present"),
                    ));
                }

                if candidate.features().establishes_absence() {
                    return Ok((
                        ConstraintEvaluation::Violated,
                        format!(
                            "feature '{feature}' is absent from a complete \
                             feature inventory"
                        ),
                    ));
                }

                Ok((
                    ConstraintEvaluation::Indeterminate,
                    format!(
                        "feature '{feature}' is absent from the observed \
                         inventory, but inventory completeness is not \
                         established"
                    ),
                ))
            }

            ConstraintPredicate::Capability(capability) => {
                if candidate.capabilities().contains(capability) {
                    return Ok((
                        ConstraintEvaluation::Satisfied,
                        format!("capability '{capability}' is present"),
                    ));
                }

                if candidate.capabilities().establishes_absence() {
                    return Ok((
                        ConstraintEvaluation::Violated,
                        format!(
                            "capability '{capability}' is absent from a \
                             complete capability inventory"
                        ),
                    ));
                }

                Ok((
                    ConstraintEvaluation::Indeterminate,
                    format!(
                        "capability '{capability}' is not established"
                    ),
                ))
            }

            ConstraintPredicate::Compatibility {
                namespace,
                expression,
            } => {
                let Some(evaluator) =
                    self.compatibility_evaluators.get(namespace)
                else {
                    return Ok((
                        ConstraintEvaluation::Indeterminate,
                        format!(
                            "no compatibility evaluator is registered \
                             for '{namespace}'"
                        ),
                    ));
                };

                let evaluation = evaluator.evaluate(
                    expression,
                    candidate,
                )?;

                Ok((
                    evaluation,
                    evaluation_description(evaluation).to_owned(),
                ))
            }

            ConstraintPredicate::Extension {
                namespace,
                expression,
            } => {
                let Some(evaluator) =
                    self.extension_evaluators.get(namespace)
                else {
                    return Ok((
                        ConstraintEvaluation::Indeterminate,
                        format!(
                            "no extension evaluator is registered \
                             for '{namespace}'"
                        ),
                    ));
                };

                let evaluation = evaluator.evaluate(
                    expression,
                    candidate,
                )?;

                Ok((
                    evaluation,
                    evaluation_description(evaluation).to_owned(),
                ))
            }
        }
    }
}

// ============================================================================
// INDIVIDUAL EVALUATION RESULT
// ============================================================================

/// Result of evaluating one dependency constraint.
///
/// An optional evaluator error is retained for failed preference or hint
/// evaluation. It is never silently discarded.
#[derive(Debug, Clone)]
pub struct ConstraintResult {
    constraint: DependencyConstraint,
    evaluation: ConstraintEvaluation,
    explanation: String,
    evaluator_error: Option<ToolchainError>,
}

impl ConstraintResult {
    /// Borrow the evaluated constraint.
    #[must_use]
    pub fn constraint(&self) -> &DependencyConstraint {
        &self.constraint
    }

    /// Return the evaluation outcome.
    #[must_use]
    pub const fn evaluation(&self) -> ConstraintEvaluation {
        self.evaluation
    }

    /// Borrow the human-readable evaluation explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Borrow an evaluator failure, if one occurred.
    #[must_use]
    pub fn evaluator_error(&self) -> Option<&ToolchainError> {
        self.evaluator_error.as_ref()
    }
}

// ============================================================================
// AGGREGATE CONSTRAINT REPORT
// ============================================================================

/// Deterministic report of dependency candidate constraint evaluation.
///
/// Only requirements and constraints determine the aggregate decision.
/// Preferences and hints remain visible for subsequent candidate ranking
/// and optimization.
#[derive(Debug, Clone)]
pub struct ConstraintReport {
    decision: ConstraintDecision,
    results: Vec<ConstraintResult>,
}

impl ConstraintReport {
    fn new(results: Vec<ConstraintResult>) -> Self {
        // A mandatory violation dominates indeterminate mandatory evidence.
        //
        // For example:
        // - One required capability is definitively absent.
        // - Another capability is unknown.
        //
        // The candidate is still rejected because a mandatory violation
        // has already been established.

        let mandatory_violated = results.iter().any(|result| {
            result.constraint.kind().is_mandatory()
                && result.evaluation == ConstraintEvaluation::Violated
        });

        let mandatory_indeterminate = results.iter().any(|result| {
            result.constraint.kind().is_mandatory()
                && result.evaluation
                    == ConstraintEvaluation::Indeterminate
        });

        let decision = if mandatory_violated {
            ConstraintDecision::Violated
        } else if mandatory_indeterminate {
            ConstraintDecision::Indeterminate
        } else {
            ConstraintDecision::Satisfied
        };

        Self {
            decision,
            results,
        }
    }

    /// Return the aggregate mandatory-constraint decision.
    #[must_use]
    pub const fn decision(&self) -> ConstraintDecision {
        self.decision
    }

    /// Borrow every individual constraint result.
    #[must_use]
    pub fn results(&self) -> &[ConstraintResult] {
        &self.results
    }

    /// Iterate over results for a particular constraint classification.
    pub fn results_of(
        &self,
        kind: ConstraintKind,
    ) -> impl Iterator<Item = &ConstraintResult> {
        self.results
            .iter()
            .filter(move |result| result.constraint.kind() == kind)
    }

    /// Return whether every mandatory constraint is satisfied.
    ///
    /// An indeterminate decision is not considered satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        matches!(
            self.decision,
            ConstraintDecision::Satisfied
        )
    }

    /// Return whether at least one mandatory constraint is violated.
    #[must_use]
    pub const fn is_violated(&self) -> bool {
        matches!(
            self.decision,
            ConstraintDecision::Violated
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::{
        BTreeMap,
        HashSet,
    };

    fn version_scheme() -> VersionSchemeId {
        VersionSchemeId::new("test.exact")
            .expect("valid test scheme")
    }

    fn feature(value: &str) -> DependencyFeature {
        DependencyFeature::new(value)
            .expect("valid feature")
    }

    fn capability(value: &str) -> DependencyCapability {
        DependencyCapability::new(value)
            .expect("valid capability")
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

    struct AlwaysSatisfiedEvaluator;

    impl NamedConstraintEvaluator for AlwaysSatisfiedEvaluator {
        fn evaluate(
            &self,
            _expression: &str,
            _candidate: &DependencyCandidateFacts,
        ) -> Result<ConstraintEvaluation> {
            Ok(ConstraintEvaluation::Satisfied)
        }
    }

    struct FailingEvaluator;

    impl NamedConstraintEvaluator for FailingEvaluator {
        fn evaluate(
            &self,
            _expression: &str,
            _candidate: &DependencyCandidateFacts,
        ) -> Result<ConstraintEvaluation> {
            Err(
                ToolchainError::provider_failed(
                    "test evaluator failure",
                ),
            )
        }
    }

    #[test]
    fn default_candidate_facts_are_unknown() {
        let candidate = DependencyCandidateFacts::new();

        assert_eq!(
            candidate.version(),
            &Evidence::Unknown,
        );

        assert_eq!(
            candidate.source(),
            &Evidence::Unknown,
        );

        assert_eq!(
            candidate.features().completeness(),
            Completeness::Unavailable,
        );

        assert_eq!(
            candidate.capabilities().completeness(),
            Completeness::Unavailable,
        );
    }

    #[test]
    fn unknown_version_scheme_is_indeterminate() {
        let requirement = VersionRequirement::new("1.0")
            .expect("valid requirement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::version(
                    requirement,
                    None,
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(
                    version_scheme(),
                    "1.0",
                )
                .expect("valid candidate version"),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn registered_version_evaluator_can_establish_satisfaction() {
        let scheme = version_scheme();

        let requirement = VersionRequirement::new("1.0")
            .expect("valid requirement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::version(
                    requirement,
                    Some(scheme.clone()),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(
                    scheme.clone(),
                    "1.0",
                )
                .expect("valid candidate version"),
            );

        let mut registry = ConstraintEvaluatorRegistry::new();

        registry
            .register_version_evaluator(
                scheme,
                ExactVersionEvaluator,
            )
            .expect("register evaluator");

        let report = registry
            .evaluate(&constraints, &candidate)
            .expect("evaluate");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }

    #[test]
    fn unregistered_version_scheme_never_passes() {
        let scheme = version_scheme();

        let requirement = VersionRequirement::new("1.0")
            .expect("valid requirement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::version(
                    requirement,
                    Some(scheme.clone()),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_version(
                CandidateVersion::new(scheme, "1.0")
                    .expect("valid candidate version"),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn complete_feature_inventory_establishes_absence() {
        let requested = feature("quantum.measurement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::feature(
                    requested,
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_feature_inventory(
                EvidenceSet::complete(
                    [feature("classical.compute")],
                ),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Violated,
        );
    }

    #[test]
    fn partial_feature_inventory_does_not_establish_absence() {
        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::feature(
                    feature("quantum.measurement"),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_feature_inventory(
                EvidenceSet::partial(
                    [feature("classical.compute")],
                ),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn present_feature_satisfies_even_partial_inventory() {
        let requested = feature("quantum.measurement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::feature(
                    requested.clone(),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_feature_inventory(
                EvidenceSet::partial([requested]),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }

    #[test]
    fn complete_capability_inventory_establishes_absence() {
        let requested = capability("quantum.measurement");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::capability(
                    requested,
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_capability_inventory(
                EvidenceSet::complete([
                    capability("classical.compute"),
                ]),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Violated,
        );
    }

    #[test]
    fn incomplete_capability_inventory_is_indeterminate() {
        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::capability(
                    capability("quantum.measurement"),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_capability_inventory(
                EvidenceSet::unavailable(),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn exact_source_constraint_matches() {
        let source = DependencySource::registry(
            Some("main".to_owned()),
        )
        .expect("valid registry source");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::source(
                    source.clone(),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_source(source);

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }

    #[test]
    fn unknown_source_does_not_pass() {
        let source = DependencySource::registry(
            Some("main".to_owned()),
        )
        .expect("valid registry source");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::source(source),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(
                &constraints,
                &DependencyCandidateFacts::new(),
            )
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn dependency_source_constraint_implements_hash() {
        let source = DependencySource::registry(
            Some("main".to_owned()),
        )
        .expect("valid registry source");

        let constraint =
            DependencySourceConstraint::exact(source);

        let mut values = HashSet::new();

        values.insert(constraint.clone());

        assert!(values.contains(&constraint));
    }

    #[test]
    fn unknown_compatibility_evaluator_is_indeterminate() {
        let namespace = ConstraintNamespace::new(
            "zamani.compatibility",
        )
        .expect("valid namespace");

        let predicate = ConstraintPredicate::compatibility(
            namespace,
            "abi.compatible",
        )
        .expect("valid predicate");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(predicate)
                .expect("valid constraint"),
        )
        .expect("insert constraint");

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(
                &constraints,
                &DependencyCandidateFacts::new(),
            )
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn registered_extension_evaluator_is_used() {
        let namespace = ConstraintNamespace::new(
            "zamani.test.extension",
        )
        .expect("valid namespace");

        let predicate = ConstraintPredicate::extension(
            namespace.clone(),
            "test.expression",
        )
        .expect("valid predicate");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::constraint(predicate)
                .expect("valid constraint"),
        )
        .expect("insert constraint");

        let mut registry = ConstraintEvaluatorRegistry::new();

        registry
            .register_extension_evaluator(
                namespace,
                AlwaysSatisfiedEvaluator,
            )
            .expect("register evaluator");

        let report = registry
            .evaluate(
                &constraints,
                &DependencyCandidateFacts::new(),
            )
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }

    #[test]
    fn preferences_and_hints_never_reject_candidates() {
        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::preference(
                ConstraintPredicate::capability(
                    capability("gpu.compute"),
                ),
            )
            .expect("valid preference"),
        )
        .expect("insert preference");

        constraints.insert(
            DependencyConstraint::hint(
                ConstraintPredicate::feature(
                    feature("experimental.optimization"),
                ),
            )
            .expect("valid hint"),
        )
        .expect("insert hint");

        let candidate = DependencyCandidateFacts::new()
            .with_capability_inventory(
                EvidenceSet::complete([]),
            )
            .with_feature_inventory(
                EvidenceSet::complete([]),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );

        assert_eq!(
            report.results_of(ConstraintKind::Preference)
                .count(),
            1,
        );

        assert_eq!(
            report.results_of(ConstraintKind::Hint)
                .count(),
            1,
        );
    }

    #[test]
    fn mandatory_violation_dominates_indeterminate_result() {
        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::capability(
                    capability("quantum.measurement"),
                ),
            )
            .expect("valid requirement"),
        )
        .expect("insert requirement");

        constraints.insert(
            DependencyConstraint::constraint(
                ConstraintPredicate::feature(
                    feature("required.feature"),
                ),
            )
            .expect("valid constraint"),
        )
        .expect("insert constraint");

        let candidate = DependencyCandidateFacts::new()
            .with_capability_inventory(
                EvidenceSet::complete([]),
            )
            .with_feature_inventory(
                EvidenceSet::unavailable(),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Violated,
        );
    }

    #[test]
    fn indeterminate_mandatory_result_prevents_satisfaction() {
        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(
                ConstraintPredicate::capability(
                    capability("quantum.measurement"),
                ),
            )
            .expect("valid requirement"),
        )
        .expect("insert requirement");

        let candidate = DependencyCandidateFacts::new()
            .with_capability_inventory(
                EvidenceSet::partial([]),
            );

        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(&constraints, &candidate)
            .expect("evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Indeterminate,
        );
    }

    #[test]
    fn mandatory_evaluator_failure_propagates() {
        let namespace = ConstraintNamespace::new(
            "zamani.test.failure",
        )
        .expect("valid namespace");

        let predicate = ConstraintPredicate::extension(
            namespace.clone(),
            "test.failure",
        )
        .expect("valid predicate");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::requirement(predicate)
                .expect("valid requirement"),
        )
        .expect("insert requirement");

        let mut registry = ConstraintEvaluatorRegistry::new();

        registry
            .register_extension_evaluator(
                namespace,
                FailingEvaluator,
            )
            .expect("register evaluator");

        let result = registry.evaluate(
            &constraints,
            &DependencyCandidateFacts::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn optional_evaluator_failure_is_preserved_without_rejection() {
        let namespace = ConstraintNamespace::new(
            "zamani.test.optional.failure",
        )
        .expect("valid namespace");

        let predicate = ConstraintPredicate::extension(
            namespace.clone(),
            "test.failure",
        )
        .expect("valid predicate");

        let mut constraints = DependencyConstraintSet::new();

        constraints.insert(
            DependencyConstraint::preference(predicate)
                .expect("valid preference"),
        )
        .expect("insert preference");

        let mut registry = ConstraintEvaluatorRegistry::new();

        registry
            .register_extension_evaluator(
                namespace,
                FailingEvaluator,
            )
            .expect("register evaluator");

        let report = registry
            .evaluate(
                &constraints,
                &DependencyCandidateFacts::new(),
            )
            .expect("optional evaluator failure is reportable");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );

        assert!(
            report.results()[0].evaluator_error().is_some()
        );

        assert_eq!(
            report.results()[0].evaluation(),
            ConstraintEvaluation::Indeterminate,
        );
    }

    #[test]
    fn duplicate_constraints_are_rejected() {
        let constraint = DependencyConstraint::requirement(
            ConstraintPredicate::capability(
                capability("classical.compute"),
            ),
        )
        .expect("valid constraint");

        let mut constraints = DependencyConstraintSet::new();

        constraints
            .insert(constraint.clone())
            .expect("first insertion");

        assert!(
            constraints.insert(constraint).is_err()
        );
    }

    #[test]
    fn duplicate_evaluator_registration_is_rejected() {
        let scheme = version_scheme();

        let mut registry = ConstraintEvaluatorRegistry::new();

        registry
            .register_version_evaluator(
                scheme.clone(),
                ExactVersionEvaluator,
            )
            .expect("first registration");

        assert!(
            registry
                .register_version_evaluator(
                    scheme,
                    ExactVersionEvaluator,
                )
                .is_err()
        );
    }

    #[test]
    fn dependency_declaration_generates_constraints() {
        let declaration = DependencyDeclaration::named(
            "zamani.quantum.runtime",
        )
        .expect("valid dependency")
        .with_version(
            VersionRequirement::new("1.0")
                .expect("valid version requirement"),
        )
        .with_source(
            DependencySource::registry(
                Some("main".to_owned()),
            )
            .expect("valid source"),
        )
        .with_features(["measurement", "simulation"])
        .expect("valid features")
        .with_capability(
            capability("quantum.measurement"),
        )
        .expect("valid capability");

        let constraints =
            DependencyConstraintSet::from_declaration(
                &declaration,
                Some(version_scheme()),
            )
            .expect("dependency constraints");

        assert_eq!(constraints.len(), 5);
    }

    #[test]
    fn metadata_inventory_preserves_extension_facts() {
        let mut values = BTreeMap::new();

        values.insert(
            "abi".to_owned(),
            "zamani.abi.v1".to_owned(),
        );

        let candidate = DependencyCandidateFacts::new()
            .with_metadata_inventory(
                EvidenceMap::complete(values),
            );

        assert_eq!(
            candidate.metadata().get("abi"),
            Some("zamani.abi.v1"),
        );

        assert_eq!(
            candidate.metadata().completeness(),
            Completeness::Complete,
        );
    }

    #[test]
    fn empty_constraint_set_is_satisfied() {
        let report = ConstraintEvaluatorRegistry::new()
            .evaluate(
                &DependencyConstraintSet::new(),
                &DependencyCandidateFacts::new(),
            )
            .expect("empty evaluation");

        assert_eq!(
            report.decision(),
            ConstraintDecision::Satisfied,
        );
    }
}
