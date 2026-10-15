#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Interchange Migration
//!
//! PATH
//!   src/zuir/interchange/migration.rs
//!
//! PURPOSE
//!   Provide an explicit, versioned migration boundary for artifacts that
//!   share a format identity but use different format/schema versions.
//!
//! AUTHORITY
//!   - src/zuir/ZUIR-SPEC.md, serialization and migration requirements
//!   - src/zuir/interchange/format.rs, format identity and version contracts
//!   - grammar/specification/compatibility.md
//!   - grammar/specification/semantic-model.md
//!
//! OWNS
//!   - Migration descriptors and migration context.
//!   - The MigrationAdapter contract.
//!   - Migration registration and duplicate detection.
//!   - Deterministic migration-path planning.
//!   - Migration execution and structured failure reporting.
//!
//! DOES_NOT_OWN
//!   - Source grammar, lexing, parsing, AST construction, or name resolution.
//!   - Canonical ZUIR data structures or their verification rules.
//!   - Format encoding, decoding, or import/export adapters.
//!   - The global language, schema, or dialect version policies.
//!   - Hardware discovery, target selection, allocation, or execution.
//!
//! DEPENDENCIES
//!   - super::format::{FormatId, FormatVersion}
//!   - Rust standard library
//!
//! SEMANTIC CONTRACT
//!   A migration must either produce a valid destination-version artifact
//!   or return an error. It must not silently discard unsupported semantics,
//!   invent operation meanings, or weaken type, effect, ownership, resource,
//!   numerical, quantum, or HDL guarantees.
//!
//!   The caller retains the original artifact by borrowing it for migration.
//!   Migration adapters must not mutate the borrowed source artifact.
//!
//! VERSION CONTRACT
//!   Format versions are not assumed to be mutually compatible.
//!   A migration edge is an explicit directed transformation from one exact
//!   version to another. Reverse migration is not inferred.
//!
//! DETERMINISM
//!   Migration paths are selected using shortest-path planning over a
//!   deterministically ordered graph. If more than one eligible shortest
//!   path exists, migration fails with AmbiguousPath rather than choosing
//!   based on registration order.
//!
//! SCALABILITY
//!   The registry uses dynamically sized collections and imposes no
//!   universal maximum number of formats, versions, or migration steps.
//!   Callers may configure an operational step budget. Process memory,
//!   execution time, and address space remain finite.
//!
//! INTEGRATION
//!   - interchange/format.rs owns FormatId and FormatVersion.
//!   - interchange/import.rs may invoke migration explicitly when requested.
//!   - interchange/export.rs remains responsible for exporting the resulting
//!     artifact into the explicitly requested destination format.
//!   - serialization owns actual encoding, decoding, and schema validation.
//!   - the parent interchange/mod.rs must declare `pub mod migration;`.
//!
//! FREEZE CRITERIA
//!   - The parent module is wired into the crate.
//!   - Migration adapters validate their source and destination contracts.
//!   - Duplicate edges and ambiguous shortest paths are rejected.
//!   - Failed migrations leave the caller's original artifact available.
//!   - Tests pass against the repository's actual compiler and module graph.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::sync::Arc;

use super::format::{FormatId, FormatVersion};

/// Error type returned by migration operations.
pub type MigrationResult<T> = Result<T, MigrationError>;

/// A validated declaration of one directed migration step.
///
/// The format identity is kept separate from the version pair. This module
/// does not infer that two different format identities are interchangeable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationDescriptor {
    format_id: FormatId,
    from_version: FormatVersion,
    to_version: FormatVersion,
    name: String,
    deterministic: bool,
}

impl MigrationDescriptor {
    /// Construct a migration declaration.
    ///
    /// Identical source and destination versions are rejected because an
    /// identity transformation does not need a registered migration step.
    pub fn new(
        format_id: FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
        name: impl Into<String>,
        deterministic: bool,
    ) -> MigrationResult<Self> {
        let name = name.into();

        if from_version == to_version {
            return Err(MigrationError::InvalidDescriptor {
                reason: "source and destination versions must differ".to_owned(),
            });
        }

        if name.trim().is_empty() {
            return Err(MigrationError::InvalidDescriptor {
                reason: "migration name must not be empty".to_owned(),
            });
        }

        Ok(Self {
            format_id,
            from_version,
            to_version,
            name,
            deterministic,
        })
    }

    /// Format family to which this migration applies.
    #[must_use]
    pub fn format_id(&self) -> &FormatId {
        &self.format_id
    }

    /// Exact source version accepted by this migration.
    #[must_use]
    pub const fn from_version(&self) -> FormatVersion {
        self.from_version
    }

    /// Exact destination version produced by this migration.
    #[must_use]
    pub const fn to_version(&self) -> FormatVersion {
        self.to_version
    }

    /// Human-readable migration name for diagnostics and tooling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the adapter promises deterministic output for equivalent
    /// input artifacts and equivalent migration configuration.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        self.deterministic
    }
}

/// Per-request policy for planning and running migrations.
///
/// `max_steps = None` means this context imposes no migration-step budget.
/// It does not guarantee unbounded execution time or available memory.
///
/// Deterministic migrations are required by default. A caller may explicitly
/// permit nondeterministic migration steps, but doing so changes the
/// reproducibility contract for the resulting artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationContext {
    max_steps: Option<usize>,
    require_deterministic: bool,
}

impl Default for MigrationContext {
    fn default() -> Self {
        Self {
            max_steps: None,
            require_deterministic: true,
        }
    }
}

impl MigrationContext {
    /// Create the default migration policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of migration steps allowed in a chain.
    ///
    /// A zero-step budget permits only an identity request, where source and
    /// destination versions are equal.
    #[must_use]
    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = Some(max_steps);
        self
    }

    /// Require all selected migration steps to declare deterministic output.
    #[must_use]
    pub fn requiring_determinism(mut self, required: bool) -> Self {
        self.require_deterministic = required;
        self
    }

    /// Return the configured maximum chain length, if any.
    #[must_use]
    pub const fn max_steps(&self) -> Option<usize> {
        self.max_steps
    }

    /// Return whether nondeterministic migration steps are forbidden.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.require_deterministic
    }
}

/// Result of a migration request.
///
/// `Unchanged` avoids requiring `T: Clone` for an identity migration. In
/// that case the caller's original artifact remains the authoritative value.
/// `Migrated` contains a new artifact produced by the selected migration
/// chain.
#[derive(Debug, PartialEq, Eq)]
pub enum MigrationOutcome<T> {
    /// Source and destination versions were identical; no step was executed.
    Unchanged,

    /// The migration chain completed and produced the destination artifact.
    Migrated(T),
}

/// Contract implemented by a concrete migration step.
///
/// Implementations must:
/// - accept only the declared source version and format family;
/// - produce an artifact satisfying the declared destination version;
/// - preserve semantics guaranteed by the relevant compatibility contract;
/// - avoid mutating the borrowed input;
/// - report failure instead of silently dropping unsupported meaning;
/// - avoid executing user programs while migrating their representation.
///
/// The registry cannot prove that an adapter's semantic claims are correct.
/// That guarantee requires adapter-specific verification and conformance
/// tests.
pub trait MigrationAdapter<T>: Send + Sync {
    /// Return the stable declaration for this migration step.
    fn descriptor(&self) -> &MigrationDescriptor;

    /// Convert an artifact from the declared source version to the declared
    /// destination version.
    ///
    /// The returned error is preserved as the source of MigrationError.
    fn migrate(
        &self,
        artifact: &T,
        context: &MigrationContext,
    ) -> Result<T, Box<dyn Error + Send + Sync + 'static>>;
}

/// Internal ordered key for one directed edge in the migration graph.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MigrationKey {
    format_id: FormatId,
    from_version: FormatVersion,
    to_version: FormatVersion,
}

impl MigrationKey {
    fn from_descriptor(descriptor: &MigrationDescriptor) -> Self {
        Self {
            format_id: descriptor.format_id.clone(),
            from_version: descriptor.from_version,
            to_version: descriptor.to_version,
        }
    }
}

/// Errors raised while registering, planning, or executing migrations.
#[derive(Debug)]
#[non_exhaustive]
pub enum MigrationError {
    /// A descriptor violates the migration contract.
    InvalidDescriptor {
        reason: String,
    },

    /// A migration already exists for this exact format/version edge.
    DuplicateMigration {
        format_id: FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
    },

    /// No eligible migration path connects the requested versions.
    NoPath {
        format_id: FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
        deterministic_only: bool,
    },

    /// More than one eligible shortest path connects the versions.
    ///
    /// The registry refuses to choose one based on insertion order.
    AmbiguousPath {
        format_id: FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
    },

    /// The shortest eligible path exceeds the caller's step budget.
    StepLimitExceeded {
        required_steps: usize,
        maximum_steps: usize,
    },

    /// A registered migration adapter failed.
    AdapterFailed {
        descriptor: MigrationDescriptor,
        source: Box<dyn Error + Send + Sync + 'static>,
    },

    /// The registry's internal graph and adapter table became inconsistent.
    ///
    /// This indicates an implementation or integration defect rather than
    /// malformed user input.
    RegistryInvariantViolation {
        reason: String,
    },
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDescriptor { reason } => {
                write!(f, "invalid migration descriptor: {reason}")
            }
            Self::DuplicateMigration {
                format_id,
                from_version,
                to_version,
            } => write!(
                f,
                "duplicate migration for {format_id}: \
                 {from_version} -> {to_version}"
            ),
            Self::NoPath {
                format_id,
                from_version,
                to_version,
                deterministic_only,
            } => write!(
                f,
                "no eligible migration path for {format_id}: \
                 {from_version} -> {to_version} \
                 (deterministic-only: {deterministic_only})"
            ),
            Self::AmbiguousPath {
                format_id,
                from_version,
                to_version,
            } => write!(
                f,
                "multiple shortest migration paths for {format_id}: \
                 {from_version} -> {to_version}"
            ),
            Self::StepLimitExceeded {
                required_steps,
                maximum_steps,
            } => write!(
                f,
                "migration requires {required_steps} steps, \
                 exceeding the configured maximum of {maximum_steps}"
            ),
            Self::AdapterFailed { descriptor, source } => write!(
                f,
                "migration '{}' for {} failed ({} -> {}): {}",
                descriptor.name,
                descriptor.format_id,
                descriptor.from_version,
                descriptor.to_version,
                source
            ),
            Self::RegistryInvariantViolation { reason } => {
                write!(f, "migration registry invariant violation: {reason}")
            }
        }
    }
}

impl Error for MigrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AdapterFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

/// Registry of versioned migrations for a single artifact type.
///
/// A registry may contain steps for multiple format families. Each request
/// specifies the format family explicitly, preventing accidental migration
/// across unrelated representations.
///
/// This registry supports schema evolution where the in-memory artifact
/// type `T` is stable across versions. When a format migration changes the
/// in-memory Rust type, implement a separate typed conversion boundary or
/// migrate serialized artifacts through explicitly versioned import/export
/// adapters instead of erasing type safety.
pub struct MigrationRegistry<T> {
    adapters: BTreeMap<MigrationKey, Arc<dyn MigrationAdapter<T>>>,
    outgoing: BTreeMap<(FormatId, FormatVersion), BTreeSet<MigrationKey>>,
}

impl<T> Default for MigrationRegistry<T> {
    fn default() -> Self {
        Self {
            adapters: BTreeMap::new(),
            outgoing: BTreeMap::new(),
        }
    }
}

impl<T> fmt::Debug for MigrationRegistry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MigrationRegistry")
            .field("registered_migration_count", &self.adapters.len())
            .finish()
    }
}

impl<T> MigrationRegistry<T> {
    /// Create an empty migration registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered migration edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Return whether the registry has no migration edges.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Register a migration adapter.
    ///
    /// Duplicate format/source/destination triples are rejected. Registration
    /// order never replaces an existing adapter or influences path choice.
    pub fn register<A>(&mut self, adapter: A) -> MigrationResult<()>
    where
        A: MigrationAdapter<T> + 'static,
    {
        self.register_shared(Arc::new(adapter))
    }

    /// Register an already shared migration adapter.
    pub fn register_shared(
        &mut self,
        adapter: Arc<dyn MigrationAdapter<T>>,
    ) -> MigrationResult<()> {
        let descriptor = adapter.descriptor().clone();

        if descriptor.name.trim().is_empty() {
            return Err(MigrationError::InvalidDescriptor {
                reason: "migration name must not be empty".to_owned(),
            });
        }

        if descriptor.from_version == descriptor.to_version {
            return Err(MigrationError::InvalidDescriptor {
                reason: "source and destination versions must differ".to_owned(),
            });
        }

        let key = MigrationKey::from_descriptor(&descriptor);

        if self.adapters.contains_key(&key) {
            return Err(MigrationError::DuplicateMigration {
                format_id: key.format_id,
                from_version: key.from_version,
                to_version: key.to_version,
            });
        }

        let outgoing_key = (key.format_id.clone(), key.from_version);
        self.outgoing
            .entry(outgoing_key)
            .or_default()
            .insert(key.clone());

        self.adapters.insert(key, adapter);
        Ok(())
    }

    /// Return all migration declarations in deterministic key order.
    #[must_use]
    pub fn descriptors(&self) -> Vec<MigrationDescriptor> {
        self.adapters
            .values()
            .map(|adapter| adapter.descriptor().clone())
            .collect()
    }

    /// Plan the shortest eligible path without executing any migration.
    ///
    /// If multiple shortest paths exist, returns `AmbiguousPath`.
    /// If `from_version == to_version`, returns an empty path.
    pub fn plan(
        &self,
        format_id: &FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
        context: &MigrationContext,
    ) -> MigrationResult<Vec<MigrationDescriptor>> {
        if from_version == to_version {
            return Ok(Vec::new());
        }

        let keys = self.resolve_path(
            format_id,
            from_version,
            to_version,
            context.requires_determinism(),
        )?;

        if let Some(maximum_steps) = context.max_steps() {
            if keys.len() > maximum_steps {
                return Err(MigrationError::StepLimitExceeded {
                    required_steps: keys.len(),
                    maximum_steps,
                });
            }
        }

        keys.into_iter()
            .map(|key| {
                self.adapters
                    .get(&key)
                    .map(|adapter| adapter.descriptor().clone())
                    .ok_or_else(|| MigrationError::RegistryInvariantViolation {
                        reason: "planned migration edge has no registered adapter".to_owned(),
                    })
            })
            .collect()
    }

    /// Migrate an artifact using an explicitly requested format and version
    /// pair.
    ///
    /// The original artifact is borrowed, not consumed. If any migration
    /// fails, the caller still owns the original input. Intermediate results
    /// are private to this call and are not returned after failure.
    ///
    /// No migration is attempted when source and destination versions match.
    /// No cross-format conversion is inferred.
    pub fn migrate(
        &self,
        format_id: &FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
        artifact: &T,
        context: &MigrationContext,
    ) -> MigrationResult<MigrationOutcome<T>> {
        if from_version == to_version {
            return Ok(MigrationOutcome::Unchanged);
        }

        let path = self.plan(
            format_id,
            from_version,
            to_version,
            context,
        )?;

        let mut current: Option<T> = None;

        for descriptor in path {
            let key = MigrationKey::from_descriptor(&descriptor);
            let adapter = self.adapters.get(&key).ok_or_else(|| {
                MigrationError::RegistryInvariantViolation {
                    reason: "planned migration adapter disappeared".to_owned(),
                }
            })?;

            let input = match current.as_ref() {
                Some(intermediate) => intermediate,
                None => artifact,
            };

            let next = adapter
                .migrate(input, context)
                .map_err(|source| MigrationError::AdapterFailed {
                    descriptor: descriptor.clone(),
                    source,
                })?;

            current = Some(next);
        }

        current
            .map(MigrationOutcome::Migrated)
            .ok_or_else(|| MigrationError::RegistryInvariantViolation {
                reason: "non-identity migration produced an empty path".to_owned(),
            })
    }

    fn resolve_path(
        &self,
        format_id: &FormatId,
        from_version: FormatVersion,
        to_version: FormatVersion,
        deterministic_only: bool,
    ) -> MigrationResult<Vec<MigrationKey>> {
        let mut distances: BTreeMap<FormatVersion, usize> = BTreeMap::new();
        let mut path_counts: BTreeMap<FormatVersion, u8> = BTreeMap::new();
        let mut predecessor: BTreeMap<FormatVersion, MigrationKey> =
            BTreeMap::new();
        let mut queue = VecDeque::new();

        distances.insert(from_version, 0);
        path_counts.insert(from_version, 1);
        queue.push_back(from_version);

        while let Some(current_version) = queue.pop_front() {
            let current_distance = *distances
                .get(&current_version)
                .ok_or_else(|| MigrationError::RegistryInvariantViolation {
                    reason: "queued version has no recorded distance".to_owned(),
                })?;

            if current_version == to_version {
                // BFS guarantees that a shortest path has been found.
                // Continue processing nodes at the same preceding depth so
                // that alternate shortest paths can still be detected.
                continue;
            }

            let outgoing_key = (format_id.clone(), current_version);
            let Some(edges) = self.outgoing.get(&outgoing_key) else {
                continue;
            };

            for edge in edges {
                let Some(adapter) = self.adapters.get(edge) else {
                    return Err(MigrationError::RegistryInvariantViolation {
                        reason: "outgoing edge has no registered adapter".to_owned(),
                    });
                };

                if deterministic_only
                    && !adapter.descriptor().is_deterministic()
                {
                    continue;
                }

                let next_version = edge.to_version;
                let next_distance = current_distance.checked_add(1).ok_or_else(
                    || MigrationError::RegistryInvariantViolation {
                        reason: "migration path length overflow".to_owned(),
                    },
                )?;

                match distances.get(&next_version).copied() {
                    None => {
                        distances.insert(next_version, next_distance);
                        path_counts.insert(
                            next_version,
                            *path_counts.get(&current_version).unwrap_or(&0),
                        );
                        predecessor.insert(next_version, edge.clone());
                        queue.push_back(next_version);
                    }
                    Some(existing_distance) if existing_distance == next_distance => {
                        let current_paths =
                            *path_counts.get(&current_version).unwrap_or(&0);
                        let existing_paths =
                            *path_counts.get(&next_version).unwrap_or(&0);

                        // Only uniqueness matters. Saturating at two avoids
                        // counting an unbounded number of equivalent routes.
                        path_counts.insert(
                            next_version,
                            existing_paths.saturating_add(current_paths).min(2),
                        );
                    }
                    Some(_) => {
                        // A longer route cannot be part of a shortest path.
                    }
                }
            }
        }

        let Some(_) = distances.get(&to_version) else {
            return Err(MigrationError::NoPath {
                format_id: format_id.clone(),
                from_version,
                to_version,
                deterministic_only,
            });
        };

        if path_counts.get(&to_version).copied().unwrap_or(0) > 1 {
            return Err(MigrationError::AmbiguousPath {
                format_id: format_id.clone(),
                from_version,
                to_version,
            });
        }

        let mut reversed = Vec::new();
        let mut cursor = to_version;

        while cursor != from_version {
            let edge = predecessor.get(&cursor).cloned().ok_or_else(|| {
                MigrationError::RegistryInvariantViolation {
                    reason: "unique path is missing a predecessor edge".to_owned(),
                }
            })?;

            cursor = edge.from_version;
            reversed.push(edge);
        }

        reversed.reverse();
        Ok(reversed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestFailure(&'static str);

    impl fmt::Display for TestFailure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    impl Error for TestFailure {}

    struct AppendMigration {
        descriptor: MigrationDescriptor,
        suffix: &'static str,
        fail: bool,
    }

    impl AppendMigration {
        fn new(
            from: u64,
            to: u64,
            suffix: &'static str,
        ) -> Self {
            Self {
                descriptor: MigrationDescriptor::new(
                    FormatId::new("zamani.zuir").expect("valid format ID"),
                    FormatVersion::new(from, 0, 0),
                    FormatVersion::new(to, 0, 0),
                    format!("v{from}-to-v{to}"),
                    true,
                )
                .expect("valid migration descriptor"),
                suffix,
                fail: false,
            }
        }

        fn failing(from: u64, to: u64) -> Self {
            let mut migration = Self::new(from, to, "-unused");
            migration.fail = true;
            migration
        }
    }

    impl MigrationAdapter<String> for AppendMigration {
        fn descriptor(&self) -> &MigrationDescriptor {
            &self.descriptor
        }

        fn migrate(
            &self,
            artifact: &String,
            _context: &MigrationContext,
        ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
            if self.fail {
                return Err(Box::new(TestFailure("deliberate migration failure")));
            }

            Ok(format!("{artifact}{}", self.suffix))
        }
    }

    fn version(value: u64) -> FormatVersion {
        FormatVersion::new(value, 0, 0)
    }

    fn format_id() -> FormatId {
        FormatId::new("zamani.zuir").expect("valid format ID")
    }

    #[test]
    fn identity_migration_does_not_require_clone() {
        let registry = MigrationRegistry::<String>::new();
        let input = String::from("original");

        let result = registry
            .migrate(
                &format_id(),
                version(1),
                version(1),
                &input,
                &MigrationContext::new(),
            )
            .expect("identity migration succeeds");

        assert_eq!(result, MigrationOutcome::Unchanged);
        assert_eq!(input, "original");
    }

    #[test]
    fn migrates_through_a_unique_shortest_chain() {
        let mut registry = MigrationRegistry::new();
        registry
            .register(AppendMigration::new(1, 2, "-two"))
            .expect("register first step");
        registry
            .register(AppendMigration::new(2, 3, "-three"))
            .expect("register second step");

        let input = String::from("artifact");
        let result = registry
            .migrate(
                &format_id(),
                version(1),
                version(3),
                &input,
                &MigrationContext::new(),
            )
            .expect("migration succeeds");

        assert_eq!(
            result,
            MigrationOutcome::Migrated(
                "artifact-two-three".to_owned()
            )
        );
        assert_eq!(input, "artifact");
    }

    #[test]
    fn rejects_duplicate_edges() {
        let mut registry = MigrationRegistry::new();
        registry
            .register(AppendMigration::new(1, 2, "-a"))
            .expect("first registration");

        let error = registry
            .register(AppendMigration::new(1, 2, "-b"))
            .expect_err("duplicate edge must fail");

        assert!(matches!(
            error,
            MigrationError::DuplicateMigration { .. }
        ));
    }

    #[test]
    fn rejects_ambiguous_shortest_paths() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(AppendMigration::new(1, 2, "-a"))
            .expect("register edge");
        registry
            .register(AppendMigration::new(1, 3, "-b"))
            .expect("register edge");
        registry
            .register(AppendMigration::new(2, 4, "-c"))
            .expect("register edge");
        registry
            .register(AppendMigration::new(3, 4, "-d"))
            .expect("register edge");

        let error = registry
            .plan(
                &format_id(),
                version(1),
                version(4),
                &MigrationContext::new(),
            )
            .expect_err("two shortest paths must be rejected");

        assert!(matches!(error, MigrationError::AmbiguousPath { .. }));
    }

    #[test]
    fn enforces_the_configured_step_budget() {
        let mut registry = MigrationRegistry::new();
        registry
            .register(AppendMigration::new(1, 2, "-two"))
            .expect("register edge");
        registry
            .register(AppendMigration::new(2, 3, "-three"))
            .expect("register edge");

        let error = registry
            .plan(
                &format_id(),
                version(1),
                version(3),
                &MigrationContext::new().with_max_steps(1),
            )
            .expect_err("step budget must be enforced");

        assert!(matches!(
            error,
            MigrationError::StepLimitExceeded {
                required_steps: 2,
                maximum_steps: 1
            }
        ));
    }

    #[test]
    fn deterministic_policy_excludes_nondeterministic_steps() {
        let mut registry = MigrationRegistry::new();
        let descriptor = MigrationDescriptor::new(
            format_id(),
            version(1),
            version(2),
            "nondeterministic-step",
            false,
        )
        .expect("valid descriptor");

        struct NondeterministicMigration {
            descriptor: MigrationDescriptor,
        }

        impl MigrationAdapter<String> for NondeterministicMigration {
            fn descriptor(&self) -> &MigrationDescriptor {
                &self.descriptor
            }

            fn migrate(
                &self,
                artifact: &String,
                _context: &MigrationContext,
            ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
                Ok(artifact.clone())
            }
        }

        registry
            .register(NondeterministicMigration { descriptor })
            .expect("register adapter");

        let error = registry
            .plan(
                &format_id(),
                version(1),
                version(2),
                &MigrationContext::new(),
            )
            .expect_err("default policy requires deterministic migrations");

        assert!(matches!(error, MigrationError::NoPath { .. }));

        assert_eq!(
            registry
                .plan(
                    &format_id(),
                    version(1),
                    version(2),
                    &MigrationContext::new().requiring_determinism(false),
                )
                .expect("explicitly permit nondeterministic migration")
                .len(),
            1
        );
    }

    #[test]
    fn failed_migration_preserves_the_original_artifact() {
        let mut registry = MigrationRegistry::new();
        registry
            .register(AppendMigration::failing(1, 2))
            .expect("register failing adapter");

        let input = String::from("original");
        let result = registry.migrate(
            &format_id(),
            version(1),
            version(2),
            &input,
            &MigrationContext::new(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::AdapterFailed { .. })
        ));
        assert_eq!(input, "original");
    }

    #[test]
    fn unrelated_format_families_are_not_mixed() {
        let mut registry = MigrationRegistry::new();
        registry
            .register(AppendMigration::new(1, 2, "-two"))
            .expect("register edge");

        let other_format =
            FormatId::new("org.example.other-ir").expect("valid format ID");

        let error = registry
            .plan(
                &other_format,
                version(1),
                version(2),
                &MigrationContext::new(),
            )
            .expect_err("unrelated formats must not share migration paths");

        assert!(matches!(error, MigrationError::NoPath { .. }));
    }
}