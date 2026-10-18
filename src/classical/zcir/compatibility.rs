
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZCIR — Compatibility and Migration Contracts
//!
//! Path: `src/classical/zcir/compatibility.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Apply explicit compatibility policies to ZCIR artifact version manifests
//! and maintain a deterministic catalog of declared forward migration paths.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/classical/zcir/version.rs`
//!
//! ## OWNS
//!
//! - Four-part ZCIR artifact compatibility assessment.
//! - Explicit, forward-only migration declarations.
//! - Deterministic migration-path discovery.
//! - Errors for invalid or conflicting migration declarations.
//!
//! ## DOES_NOT_OWN
//!
//! - Version identities or compatibility decision semantics.
//! - Artifact decoding, validation, serialization, or interpretation.
//! - The implementation or execution of a migration.
//! - Semantic equivalence proofs or IR verification.
//! - Dialect registration or language feature validation.
//! - Target discovery, hardware allocation, or resource planning.
//!
//! ## INTEGRATION CONTRACT
//!
//! `version.rs` owns ZcirSchemaVersion, ZcirFormatVersion,
//! ZcirVersionManifest, and the ZUIR compatibility-policy adapters.
//! `zuir::version` owns CompatibilityPolicy, CompatibilityDecision,
//! Compatibility, CompatibilityReason, DialectVersion, LanguageVersion,
//! and VersionTriple.
//!
//! This module consumes those types and does not redefine them.
//! Schema and format policies are independent from dialect and language
//! policies. The caller supplies all four policies explicitly.
//!
//! MigrationCatalog records declarations of migration edges. Finding a path
//! means only that the catalog declares a sequence of migration identifiers.
//! It does not establish that implementations exist, that they succeed, or
//! that they preserve semantics. The migration executor must resolve every
//! identifier to an implementation and validate its result.
//!
//! Unknown, incompatible, future, or migration-required versions must not
//! be interpreted merely because a catalog path exists.
//!
//! ## SCALABILITY
//!
//! No fixed artifact-version count, migration count, path length, operation
//! count, or hardware-capacity ceiling is imposed. Collections are dynamic.
//! Path discovery is iterative, so a long migration chain does not consume
//! the Rust call stack. Actual memory, address-space, and work constraints
//! remain finite.
//!
//! ## DETERMINISM
//!
//! Ordered maps and sets make path selection deterministic. When multiple
//! paths exist, breadth-first search selects a shortest path and visits
//! equally short alternatives in version order.
//!
//! ## FREEZE CRITERIA
//!
//! - The parent module declares this module once.
//! - All version and decision types reuse canonical definitions.
//! - All four manifest dimensions are checked independently.
//! - Migration paths are explicit, deterministic, and forward-only.
//! - Tests cover incompatible versions, conflicts, missing paths, and chains.
//! - Repository formatting, compilation, tests, and linting pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error as StdError;

use crate::zuir::version::{
    Compatibility,
    CompatibilityDecision,
    CompatibilityPolicy,
    VersionTriple,
};

use super::version::{
    ZcirFormatVersion,
    ZcirSchemaVersion,
    ZcirVersionManifest,
};

/// Explicit policies for every version category in a ZCIR artifact.
///
/// These policies must describe the reader or consumer that is about to
/// interpret the artifact. They must not be inferred from untrusted artifact
/// metadata alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZcirCompatibilityPolicies {
    schema: CompatibilityPolicy,
    format: CompatibilityPolicy,
    dialect: CompatibilityPolicy,
    language: CompatibilityPolicy,
}

impl ZcirCompatibilityPolicies {
    /// Construct policies for the four independent version dimensions.
    #[must_use]
    pub const fn new(
        schema: CompatibilityPolicy,
        format: CompatibilityPolicy,
        dialect: CompatibilityPolicy,
        language: CompatibilityPolicy,
    ) -> Self {
        Self {
            schema,
            format,
            dialect,
            language,
        }
    }

    /// Return the schema policy.
    #[must_use]
    pub const fn schema(self) -> CompatibilityPolicy {
        self.schema
    }

    /// Return the serialization-format policy.
    #[must_use]
    pub const fn format(self) -> CompatibilityPolicy {
        self.format
    }

    /// Return the dialect policy.
    #[must_use]
    pub const fn dialect(self) -> CompatibilityPolicy {
        self.dialect
    }

    /// Return the language policy.
    #[must_use]
    pub const fn language(self) -> CompatibilityPolicy {
        self.language
    }
}

/// Compatibility assessment for all version categories in a ZCIR artifact.
///
/// A compatible version manifest is necessary but not sufficient for
/// accepting an artifact. Decoding, structural verification, feature
/// validation, and policy checks remain separate obligations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZcirCompatibilityReport {
    schema: CompatibilityDecision,
    format: CompatibilityDecision,
    dialect: CompatibilityDecision,
    language: CompatibilityDecision,
}

impl ZcirCompatibilityReport {
    /// Construct an assessment from four independent decisions.
    #[must_use]
    pub const fn new(
        schema: CompatibilityDecision,
        format: CompatibilityDecision,
        dialect: CompatibilityDecision,
        language: CompatibilityDecision,
    ) -> Self {
        Self {
            schema,
            format,
            dialect,
            language,
        }
    }

    /// Return the schema decision.
    #[must_use]
    pub const fn schema(self) -> CompatibilityDecision {
        self.schema
    }

    /// Return the format decision.
    #[must_use]
    pub const fn format(self) -> CompatibilityDecision {
        self.format
    }

    /// Return the dialect decision.
    #[must_use]
    pub const fn dialect(self) -> CompatibilityDecision {
        self.dialect
    }

    /// Return the language decision.
    #[must_use]
    pub const fn language(self) -> CompatibilityDecision {
        self.language
    }

    /// Return true only if every version is compatible without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(self) -> bool {
        matches!(self.schema.compatibility(), Compatibility::Compatible)
            && matches!(self.format.compatibility(), Compatibility::Compatible)
            && matches!(self.dialect.compatibility(), Compatibility::Compatible)
            && matches!(self.language.compatibility(), Compatibility::Compatible)
    }

    /// Return true if any version is incompatible or requires migration.
    ///
    /// This does not distinguish a migratable version from an unsupported
    /// version. Inspect the individual decisions to determine the reason.
    #[must_use]
    pub const fn requires_migration_or_rejection(self) -> bool {
        !self.is_compatible_without_migration()
    }
}

/// Assess an artifact against caller-selected compatibility policies.
///
/// The schema, format, dialect, and language versions are checked
/// independently. The function does not decode the artifact, execute
/// migrations, or certify semantic correctness.
///
/// The manifest is metadata and must not be used to choose weaker policies
/// than the receiving implementation actually supports.
#[must_use]
pub const fn assess_compatibility(
    manifest: ZcirVersionManifest,
    policies: ZcirCompatibilityPolicies,
) -> ZcirCompatibilityReport {
    ZcirCompatibilityReport::new(
        policies.schema().check(manifest.schema().components()),
        policies.format().check(manifest.format().components()),
        policies.dialect().check(manifest.dialect().components()),
        policies.language().check(manifest.language().components()),
    )
}

/// Version category for a declared migration.
///
/// Each category has an independent migration graph. A schema migration
/// cannot be used as a substitute for a serialization-format migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MigrationDomain {
    /// Migration of the logical ZCIR schema.
    Schema,
    /// Migration of the serialized artifact format.
    Format,
    /// Migration of a dialect contract.
    Dialect,
    /// Migration of a language-version contract.
    Language,
}

/// One declared migration edge.
///
/// The identifier is an opaque, caller-defined key for an implementation
/// registered elsewhere. This record is not itself executable migration code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationStep {
    domain: MigrationDomain,
    from: VersionTriple,
    to: VersionTriple,
    migration_id: String,
}

impl MigrationStep {
    /// Construct a migration step.
    #[must_use]
    pub fn new(
        domain: MigrationDomain,
        from: VersionTriple,
        to: VersionTriple,
        migration_id: impl Into<String>,
    ) -> Result<Self, MigrationCatalogError> {
        let migration_id = migration_id.into();

        if migration_id.trim().is_empty() || migration_id.contains('\0') {
            return Err(MigrationCatalogError::InvalidMigrationId);
        }

        if to <= from {
            return Err(MigrationCatalogError::MigrationMustAdvance {
                from,
                to,
            });
        }

        Ok(Self {
            domain,
            from,
            to,
            migration_id,
        })
    }

    /// Return the version category this step migrates.
    #[must_use]
    pub const fn domain(&self) -> MigrationDomain {
        self.domain
    }

    /// Return the source version.
    #[must_use]
    pub const fn from(&self) -> VersionTriple {
        self.from
    }

    /// Return the destination version.
    #[must_use]
    pub const fn to(&self) -> VersionTriple {
        self.to
    }

    /// Return the opaque identifier of the migration implementation.
    #[must_use]
    pub fn migration_id(&self) -> &str {
        &self.migration_id
    }
}

/// Failure while registering or resolving declared migrations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MigrationCatalogError {
    /// A migration identifier was empty or contained a NUL character.
    InvalidMigrationId,

    /// Migration declarations must advance to a strictly greater version.
    MigrationMustAdvance {
        /// Source version.
        from: VersionTriple,
        /// Destination version.
        to: VersionTriple,
    },

    /// A source/destination edge already has a different migration identity.
    ConflictingMigration {
        /// Migration category.
        domain: MigrationDomain,
        /// Source version.
        from: VersionTriple,
        /// Destination version.
        to: VersionTriple,
    },
}

impl fmt::Display for MigrationCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMigrationId => {
                f.write_str("migration identifier must be non-empty and contain no NUL")
            }
            Self::MigrationMustAdvance { from, to } => {
                write!(f, "migration must advance version: {from} -> {to}")
            }
            Self::ConflictingMigration { domain, from, to } => {
                write!(
                    f,
                    "conflicting {domain:?} migration declarations for {from} -> {to}"
                )
            }
        }
    }
}

impl StdError for MigrationCatalogError {}

/// Deterministic catalog of declared version migration edges.
///
/// Edges are separated by version category. The catalog allows multiple
/// successive migrations to be composed into a path. It does not store
/// executable closures or assert that the named implementations are present.
///
/// The catalog is intended to be built from trusted implementation
/// registration, not directly from an untrusted artifact.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MigrationCatalog {
    edges: BTreeMap<
        (MigrationDomain, VersionTriple),
        BTreeMap<VersionTriple, String>,
    >,
}

impl MigrationCatalog {
    /// Create an empty migration catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a declared forward migration.
    ///
    /// Re-registering the exact same edge and identifier is idempotent.
    /// Registering the same edge with a different identifier is an error;
    /// the catalog will not silently replace a migration implementation.
    pub fn register(
        &mut self,
        step: MigrationStep,
    ) -> Result<bool, MigrationCatalogError> {
        let key = (step.domain(), step.from());

        if let Some(existing) = self
            .edges
            .get(&key)
            .and_then(|destinations| destinations.get(&step.to()))
        {
            if existing == step.migration_id() {
                return Ok(false);
            }

            return Err(MigrationCatalogError::ConflictingMigration {
                domain: step.domain(),
                from: step.from(),
                to: step.to(),
            });
        }

        self.edges
            .entry(key)
            .or_default()
            .insert(step.to(), step.migration_id().to_owned());

        Ok(true)
    }

    /// Return the number of declared migration edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges
            .values()
            .map(BTreeMap::len)
            .sum()
    }

    /// Return whether the catalog has no declared migration edges.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.values().all(BTreeMap::is_empty)
    }

    /// Find a deterministic shortest migration path.
    ///
    /// Breadth-first search selects a shortest path. When multiple shortest
    /// paths exist, destination versions are visited in ascending order.
    /// An empty vector means the source and destination versions are equal.
    /// `None` means no declared path exists.
    ///
    /// This method discovers declarations only. Before applying the path,
    /// the caller must resolve every migration ID to a trusted implementation,
    /// validate each intermediate artifact, and enforce applicable policies.
    #[must_use]
    pub fn find_path(
        &self,
        domain: MigrationDomain,
        from: VersionTriple,
        to: VersionTriple,
    ) -> Option<Vec<MigrationStep>> {
        if from == to {
            return Some(Vec::new());
        }

        if to < from {
            return None;
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut predecessor: BTreeMap<
            VersionTriple,
            (VersionTriple, String),
        > = BTreeMap::new();

        queue.push_back(from);
        visited.insert(from);

        while let Some(current) = queue.pop_front() {
            let Some(destinations) = self.edges.get(&(domain, current)) else {
                continue;
            };

            for (next, migration_id) in destinations {
                if *next > to || !visited.insert(*next) {
                    continue;
                }

                predecessor.insert(
                    *next,
                    (current, migration_id.clone()),
                );

                if *next == to {
                    return Some(reconstruct_path(
                        domain,
                        from,
                        to,
                        &predecessor,
                    ));
                }

                queue.push_back(*next);
            }
        }

        None
    }
}

/// Reconstruct a path after breadth-first search reaches its destination.
fn reconstruct_path(
    domain: MigrationDomain,
    from: VersionTriple,
    to: VersionTriple,
    predecessor: &BTreeMap<VersionTriple, (VersionTriple, String)>,
) -> Vec<MigrationStep> {
    let mut reversed = Vec::new();
    let mut current = to;

    while current != from {
        let (previous, migration_id) = predecessor
            .get(&current)
            .expect("visited destination must have a predecessor");

        reversed.push(MigrationStep {
            domain,
            from: *previous,
            to: current,
            migration_id: migration_id.clone(),
        });

        current = *previous;
    }

    reversed.reverse();
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::version::{
        CompatibilityReason,
        DialectVersion,
        LanguageVersion,
    };
    use crate::classical::zcir::version::{
        ZcirFormatVersion,
        ZcirSchemaVersion,
        ZcirVersionManifest,
    };

    fn v(major: u32, minor: u32, patch: u32) -> VersionTriple {
        VersionTriple::new(major, minor, patch)
    }

    fn exact(version: VersionTriple) -> CompatibilityPolicy {
        CompatibilityPolicy::exact(version)
    }

    fn policies(version: VersionTriple) -> ZcirCompatibilityPolicies {
        ZcirCompatibilityPolicies::new(
            exact(version),
            exact(version),
            exact(version),
            exact(version),
        )
    }

    fn manifest(
        schema: VersionTriple,
        format: VersionTriple,
        dialect: VersionTriple,
        language: VersionTriple,
    ) -> ZcirVersionManifest {
        ZcirVersionManifest::new(
            ZcirSchemaVersion::from_components(schema),
            ZcirFormatVersion::from_components(format),
            DialectVersion::from_components(dialect),
            LanguageVersion::from_components(language),
        )
    }

    fn step(
        domain: MigrationDomain,
        from: VersionTriple,
        to: VersionTriple,
        id: &str,
    ) -> MigrationStep {
        MigrationStep::new(domain, from, to, id)
            .expect("test migration declaration should be valid")
    }

    #[test]
    fn accepts_exact_match_for_all_version_dimensions() {
        let current = v(1, 0, 0);
        let artifact = manifest(current, current, current, current);

        let report = assess_compatibility(artifact, policies(current));

        assert!(report.is_compatible_without_migration());
        assert!(!report.requires_migration_or_rejection());
        assert_eq!(report.schema().reason(), CompatibilityReason::ExactMatch);
        assert_eq!(report.format().reason(), CompatibilityReason::ExactMatch);
        assert_eq!(report.dialect().reason(), CompatibilityReason::ExactMatch);
        assert_eq!(report.language().reason(), CompatibilityReason::ExactMatch);
    }

    #[test]
    fn rejects_an_incompatible_schema_even_if_other_versions_match() {
        let current = v(1, 0, 0);
        let artifact = manifest(v(2, 0, 0), current, current, current);

        let report = assess_compatibility(artifact, policies(current));

        assert!(!report.is_compatible_without_migration());
        assert_eq!(
            report.schema().compatibility(),
            Compatibility::Incompatible
        );
        assert_eq!(
            report.format().compatibility(),
            Compatibility::Compatible
        );
        assert_eq!(
            report.dialect().compatibility(),
            Compatibility::Compatible
        );
        assert_eq!(
            report.language().compatibility(),
            Compatibility::Compatible
        );
    }

    #[test]
    fn language_and_dialect_policies_are_independent() {
        let current = v(1, 0, 0);
        let other = v(1, 1, 0);
        let artifact = manifest(current, current, other, current);

        let selected_policies = ZcirCompatibilityPolicies::new(
            exact(current),
            exact(current),
            exact(current),
            exact(current),
        );

        let report = assess_compatibility(artifact, selected_policies);

        assert_eq!(
            report.dialect().compatibility(),
            Compatibility::RequiresMigration
        );
        assert_eq!(
            report.language().compatibility(),
            Compatibility::Compatible
        );
    }

    #[test]
    fn migration_step_requires_a_nonempty_identifier_and_forward_version() {
        assert!(matches!(
            MigrationStep::new(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(2, 0, 0),
                " "
            ),
            Err(MigrationCatalogError::InvalidMigrationId)
        ));

        assert!(matches!(
            MigrationStep::new(
                MigrationDomain::Schema,
                v(2, 0, 0),
                v(1, 0, 0),
                "downgrade"
            ),
            Err(MigrationCatalogError::MigrationMustAdvance { .. })
        ));

        assert!(matches!(
            MigrationStep::new(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(1, 0, 0),
                "identity"
            ),
            Err(MigrationCatalogError::MigrationMustAdvance { .. })
        ));
    }

    #[test]
    fn identical_migration_registration_is_idempotent() {
        let mut catalog = MigrationCatalog::new();
        let migration = step(
            MigrationDomain::Schema,
            v(1, 0, 0),
            v(1, 1, 0),
            "schema-1-to-1-1",
        );

        assert!(catalog.register(migration.clone()).unwrap());
        assert!(!catalog.register(migration).unwrap());
        assert_eq!(catalog.len(), 1);
    }

    #[test]
    fn conflicting_migration_registration_is_rejected() {
        let mut catalog = MigrationCatalog::new();

        catalog
            .register(step(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(1, 1, 0),
                "migration-a",
            ))
            .unwrap();

        let result = catalog.register(step(
            MigrationDomain::Schema,
            v(1, 0, 0),
            v(1, 1, 0),
            "migration-b",
        ));

        assert!(matches!(
            result,
            Err(MigrationCatalogError::ConflictingMigration {
                domain: MigrationDomain::Schema,
                ..
            })
        ));
        assert_eq!(catalog.len(), 1);
    }

    #[test]
    fn finds_a_multi_step_migration_path() {
        let mut catalog = MigrationCatalog::new();

        catalog
            .register(step(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(1, 1, 0),
                "schema-a",
            ))
            .unwrap();

        catalog
            .register(step(
                MigrationDomain::Schema,
                v(1, 1, 0),
                v(2, 0, 0),
                "schema-b",
            ))
            .unwrap();

        let path = catalog
            .find_path(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(2, 0, 0),
            )
            .expect("a declared path should exist");

        assert_eq!(path.len(), 2);
        assert_eq!(path[0].migration_id(), "schema-a");
        assert_eq!(path[1].migration_id(), "schema-b");
        assert_eq!(path[0].from(), v(1, 0, 0));
        assert_eq!(path[1].to(), v(2, 0, 0));
    }

    #[test]
    fn chooses_a_shortest_path_deterministically() {
        let mut catalog = MigrationCatalog::new();

        catalog
            .register(step(
                MigrationDomain::Format,
                v(1, 0, 0),
                v(1, 1, 0),
                "two-step-first",
            ))
            .unwrap();

        catalog
            .register(step(
                MigrationDomain::Format,
                v(1, 1, 0),
                v(2, 0, 0),
                "two-step-second",
            ))
            .unwrap();

        catalog
            .register(step(
                MigrationDomain::Format,
                v(1, 0, 0),
                v(2, 0, 0),
                "direct",
            ))
            .unwrap();

        let path = catalog
            .find_path(
                MigrationDomain::Format,
                v(1, 0, 0),
                v(2, 0, 0),
            )
            .expect("direct path should exist");

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].migration_id(), "direct");
    }

    #[test]
    fn migration_paths_cannot_cross_version_domains() {
        let mut catalog = MigrationCatalog::new();

        catalog
            .register(step(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(2, 0, 0),
                "schema-migration",
            ))
            .unwrap();

        assert!(catalog
            .find_path(
                MigrationDomain::Format,
                v(1, 0, 0),
                v(2, 0, 0),
            )
            .is_none());
    }

    #[test]
    fn missing_and_reverse_paths_are_not_inferred() {
        let catalog = MigrationCatalog::new();

        assert!(catalog
            .find_path(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(2, 0, 0),
            )
            .is_none());

        assert!(catalog
            .find_path(
                MigrationDomain::Schema,
                v(2, 0, 0),
                v(1, 0, 0),
            )
            .is_none());

        assert_eq!(
            catalog.find_path(
                MigrationDomain::Schema,
                v(1, 0, 0),
                v(1, 0, 0),
            ),
            Some(Vec::new())
        );
    }
}
