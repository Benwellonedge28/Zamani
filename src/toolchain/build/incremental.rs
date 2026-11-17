
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Incremental Build Coordination
//!
//! Path: src/toolchain/build/incremental.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Coordinate incremental build decisions using the canonical build-input
//! snapshot and the existing compiler incremental infrastructure.
//!
//! This module determines whether previously recorded build inputs have
//! changed and coordinates the invalidation of affected build artifacts.
//!
//! It does not compile programs or implement a cache.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! - toolchain/build/inputs.rs:
//!   Immutable logical and target-realization input identities.
//!
//! - compiler/incremental/mod.rs:
//!   Canonical incremental artifact state and invalidation coordination.
//!
//! - zuir/incremental/dependencies.rs:
//!   Canonical dependency identities and dependency graph.
//!
//! - zuir/incremental/invalidation.rs:
//!   Canonical transitive invalidation planning.
//!
//! - compiler/incremental/fingerprints.rs:
//!   Compiler fingerprint construction.
//!
//! - backend/cache.rs:
//!   Artifact-cache storage and integrity.
//!
//! - compiler/reproducibility/:
//!   Reproducible compilation identities.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Toolchain-facing incremental build coordination.
//! - Association of build identities with immutable input snapshots.
//! - Logical build and target-realization artifact identities.
//! - Detection of changed logical and realization fingerprints.
//! - Coordination of compiler-owned invalidation.
//! - Reporting incremental build decisions.
//! - Integration of toolchain artifact dependencies.
//!
//! This module does not own:
//!
//! - Source discovery or source parsing.
//! - Grammar or language semantics.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Build-input fingerprint construction.
//! - Compiler cache-key construction.
//! - Compiler dependency graph algorithms.
//! - Cache storage, admission or eviction.
//! - Target discovery or feasibility.
//! - Physical resource planning.
//! - Backend selection or code generation.
//! - Artifact construction or verification.
//! - Runtime execution.
//! - Deployment.
//!
//! POCO-REAF
//!
//! One logical build can have multiple target-specific realizations.
//!
//! The logical identity must not be contaminated by incidental physical
//! target information. Target-specific fingerprints remain isolated.
//!
//! A target realization depends on its logical build inputs. Consequently,
//! a change in logical inputs invalidates every dependent realization.
//!
//! A realization-specific change does not automatically invalidate
//! unrelated realizations.
//!
//! PORTABILITY
//!
//! Incremental identity is based on authoritative content fingerprints,
//! not machine names, process IDs, timestamps, memory addresses,
//! filesystem enumeration order or host-specific incidental state.
//!
//! SCALABILITY
//!
//! No universal maximum is imposed on:
//!
//! - Logical builds.
//! - Target realizations.
//! - Build artifacts.
//! - Dependencies.
//! - Computing domains.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Tensor dimensions.
//!
//! Collections grow according to available resources.
//!
//! Actual operation remains subject to address space, memory, execution
//! time, storage and explicitly configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! CACHE CORRECTNESS
//!
//! This module does not treat an input fingerprint as proof that a cached
//! artifact is valid.
//!
//! Cache lookup, complete cache-key derivation, payload integrity,
//! artifact compatibility and artifact verification remain with their
//! respective compiler and backend authorities.
//!
//! A cache miss never changes program semantics.
//!
//! A failed compilation must never be recorded as a successful build.
//!
//! CONCURRENCY
//!
//! Snapshot updates are serialized by this coordinator's mutex.
//!
//! The compiler incremental coordinator independently protects its
//! dependency graph and artifact state.
//!
//! No caller-provided callback is executed while either coordinator's
//! state is locked.
//!
//! INTEGRATION
//!
//! Inputs:
//!     build::inputs::BuildInputs
//!
//! Invalidation:
//!     compiler::incremental::IncrementalCompiler
//!
//! Dependencies:
//!     zuir::incremental::DependencyKind
//!
//! Consumers:
//!     build::plan
//!     build::executor
//!     build::reporting
//!
//! FREEZE CONDITIONS
//!
//! - Logical and realization identities remain separate.
//! - Identical snapshots produce no unnecessary invalidation.
//! - Changed logical inputs invalidate dependent realizations.
//! - Changed realization inputs do not invalidate unrelated realizations.
//! - Removed realizations cannot remain reusable.
//! - Dependency invalidation is delegated to the compiler authority.
//! - No artifact becomes clean before a successful external commit.
//! - No duplicate fingerprint or cache implementation is introduced.
//! - All tests and downstream integration checks pass.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{Mutex, MutexGuard};

use crate::compiler::incremental::{
    ArtifactState,
    IncrementalCompiler,
    IncrementalError,
};

use crate::toolchain::build::inputs::BuildInputs;
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::ToolchainResult;

use crate::zuir::fingerprint::stable_hash::Fingerprint;

use crate::zuir::incremental::{
    DependencyKey,
    DependencyKind,
};

// ============================================================================
// VERSIONING
// ============================================================================

/// Public API version of incremental build coordination.
///
/// Independent of the compiler, grammar, manifest, package, artifact,
//! backend, runtime and language versions.
pub const INCREMENTAL_BUILD_API_VERSION: u16 = 1;

/// Version of the toolchain incremental-build decision contract.
///
/// Increment when externally observable decision semantics change
/// incompatibly.
pub const INCREMENTAL_BUILD_SCHEMA_VERSION: u32 = 1;

/// Stable operation identifier used in structured toolchain errors.
const OPERATION: &str = "build.incremental";

// ============================================================================
// ERROR HANDLING
// ============================================================================

fn incremental_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION)
}

fn compiler_error(
    error: IncrementalError,
) -> ToolchainError {
    incremental_error(
        ToolchainErrorKind::Internal,
        format!("compiler incremental operation failed: {error}"),
    )
}

fn dependency_error(
    error: impl fmt::Display,
) -> ToolchainError {
    incremental_error(
        ToolchainErrorKind::InvalidInput,
        format!("invalid incremental dependency: {error}"),
    )
}

fn validate_identity(
    identity: &str,
    description: &'static str,
) -> ToolchainResult<()> {
    if identity.trim().is_empty() {
        return Err(incremental_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not be empty"),
        ));
    }

    if identity.contains('\0') {
        return Err(incremental_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not contain NUL"),
        ));
    }

    Ok(())
}

// ============================================================================
// STABLE ARTIFACT IDENTITY
// ============================================================================

/// Namespace of an artifact managed by toolchain incremental coordination.
///
/// This type identifies a build-level artifact. It is not a compiler IR
/// identity, physical device identity or artifact content fingerprint.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncrementalArtifactId {
    build_identity: String,
    artifact_identity: String,
}

impl IncrementalArtifactId {
    /// Construct a stable artifact identity within a logical build.
    ///
    /// The supplied identities must be stable across repeated builds.
    ///
    /// They must not be:
    ///
    /// - Process-local identifiers.
    /// - Memory addresses.
    /// - Random values.
    /// - Filesystem enumeration indices.
    /// - Display-only labels.
    ///
    /// # Errors
    ///
    /// Returns an error when either identity is empty or contains NUL.
    pub fn new(
        build_identity: impl Into<String>,
        artifact_identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let build_identity = build_identity.into();
        let artifact_identity = artifact_identity.into();

        validate_identity(
            &build_identity,
            "logical build identity",
        )?;

        validate_identity(
            &artifact_identity,
            "build artifact identity",
        )?;

        Ok(Self {
            build_identity,
            artifact_identity,
        })
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_identity(&self) -> &str {
        &self.build_identity
    }

    /// Return the artifact identity within the build.
    #[must_use]
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }

    /// Return the canonical dependency-graph key.
    ///
    /// Length-prefixed components prevent delimiter ambiguity.
    ///
    /// The identity is independent of content fingerprints.
    pub fn dependency_key(&self) -> ToolchainResult<DependencyKey> {
        let encoded = format!(
            "toolchain-artifact:{}:{}:{}:{}",
            self.build_identity.len(),
            self.build_identity,
            self.artifact_identity.len(),
            self.artifact_identity,
        );

        DependencyKey::new(encoded)
            .map_err(dependency_error)
    }
}

// ============================================================================
// BUILD ARTIFACT CATEGORIES
// ============================================================================

/// The identity of one toolchain-managed build artifact.
///
/// Logical artifacts are independent of target realization.
///
/// Realization artifacts are associated with one stable target-realization
/// identity supplied by the canonical build-planning subsystem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildArtifactScope {
    /// Artifact belonging to the target-independent logical build.
    Logical,

    /// Artifact belonging to a particular target realization.
    TargetRealization(String),
}

impl BuildArtifactScope {
    /// Construct a target-realization scope.
    ///
    /// # Errors
    ///
    /// Returns an error when the realization identity is invalid.
    pub fn target_realization(
        identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let identity = identity.into();

        validate_identity(
            &identity,
            "target realization identity",
        )?;

        Ok(Self::TargetRealization(identity))
    }

    /// Return the realization identity when this is a target-specific
    /// artifact.
    #[must_use]
    pub fn realization_identity(&self) -> Option<&str> {
        match self {
            Self::Logical => None,
            Self::TargetRealization(identity) => Some(identity),
        }
    }
}

/// Construct the stable logical-input artifact identity.
fn logical_artifact_id(
    build_identity: &str,
) -> ToolchainResult<IncrementalArtifactId> {
    IncrementalArtifactId::new(
        build_identity,
        "logical-build-inputs",
    )
}

/// Construct a stable realization-input artifact identity.
fn realization_artifact_id(
    build_identity: &str,
    realization_identity: &str,
) -> ToolchainResult<IncrementalArtifactId> {
    IncrementalArtifactId::new(
        build_identity,
        format!(
            "target-realization:{}:{}",
            realization_identity.len(),
            realization_identity,
        ),
    )
}

// ============================================================================
// INCREMENTAL DECISIONS
// ============================================================================

/// Classification of a change observed in one build-input snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncrementalChangeKind {
    /// No relevant input identity changed.
    Unchanged,

    /// The logical build identity changed.
    LogicalInputsChanged,

    /// One or more target-realization identities changed.
    RealizationInputsChanged,

    /// Both logical and realization identities changed.
    LogicalAndRealizationInputsChanged,

    /// This is the first snapshot recorded for the build.
    InitialSnapshot,
}

/// One target-realization change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealizationChange {
    identity: String,
    previous: Option<Fingerprint>,
    current: Option<Fingerprint>,
}

impl RealizationChange {
    /// Return the stable realization identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the previous fingerprint, if one existed.
    #[must_use]
    pub const fn previous(&self) -> Option<Fingerprint> {
        self.previous
    }

    /// Return the current fingerprint, if one exists.
    #[must_use]
    pub const fn current(&self) -> Option<Fingerprint> {
        self.current
    }

    /// Return whether the realization was removed.
    #[must_use]
    pub const fn was_removed(&self) -> bool {
        self.previous.is_some() && self.current.is_none()
    }

    /// Return whether the realization was newly introduced.
    #[must_use]
    pub const fn was_added(&self) -> bool {
        self.previous.is_none() && self.current.is_some()
    }
}

/// Report describing one incremental build-input update.
///
/// This report describes input identity and invalidation, not successful
/// compilation or successful artifact generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncrementalBuildReport {
    build_identity: String,
    change_kind: IncrementalChangeKind,
    previous_logical_fingerprint: Option<Fingerprint>,
    current_logical_fingerprint: Fingerprint,
    realization_changes: Vec<RealizationChange>,
    invalidated_artifacts: Vec<String>,
}

impl IncrementalBuildReport {
    /// Return the logical build identity.
    #[must_use]
    pub fn build_identity(&self) -> &str {
        &self.build_identity
    }

    /// Return the overall change classification.
    #[must_use]
    pub const fn change_kind(&self) -> IncrementalChangeKind {
        self.change_kind
    }

    /// Return the previous logical fingerprint, if available.
    #[must_use]
    pub const fn previous_logical_fingerprint(
        &self,
    ) -> Option<Fingerprint> {
        self.previous_logical_fingerprint
    }

    /// Return the current logical fingerprint.
    #[must_use]
    pub const fn current_logical_fingerprint(
        &self,
    ) -> Fingerprint {
        self.current_logical_fingerprint
    }

    /// Return all target-realization changes.
    #[must_use]
    pub fn realization_changes(&self) -> &[RealizationChange] {
        &self.realization_changes
    }

    /// Return invalidated artifact identities in deterministic order.
    ///
    /// These artifacts require the appropriate downstream rebuild or
    /// validation before they can be reused.
    #[must_use]
    pub fn invalidated_artifacts(&self) -> &[String] {
        &self.invalidated_artifacts
    }

    /// Return whether the snapshot changed.
    #[must_use]
    pub const fn changed(&self) -> bool {
        !matches!(
            self.change_kind,
            IncrementalChangeKind::Unchanged
        )
    }

    /// Return whether a full logical-build invalidation occurred.
    #[must_use]
    pub const fn logical_inputs_changed(&self) -> bool {
        matches!(
            self.change_kind,
            IncrementalChangeKind::InitialSnapshot
                | IncrementalChangeKind::LogicalInputsChanged
                | IncrementalChangeKind::LogicalAndRealizationInputsChanged
        )
    }
}

// ============================================================================
// INTERNAL SNAPSHOT STATE
// ============================================================================

#[derive(Clone, Debug)]
struct RecordedSnapshot {
    logical_fingerprint: Fingerprint,
    realizations: BTreeMap<String, Fingerprint>,
}

impl RecordedSnapshot {
    fn from_inputs(inputs: &BuildInputs) -> Self {
        let realizations = inputs
            .realizations()
            .iter()
            .map(|realization| {
                (
                    realization.identity().to_owned(),
                    realization.fingerprint(),
                )
            })
            .collect();

        Self {
            logical_fingerprint: inputs.logical_fingerprint(),
            realizations,
        }
    }
}

#[derive(Debug, Default)]
struct CoordinatorState {
    snapshots: BTreeMap<String, RecordedSnapshot>,
}

/// Thread-safe toolchain incremental build coordinator.
///
/// This is an orchestration facade over the existing compiler incremental
/// authority. It does not own another dependency graph or cache.
pub struct IncrementalBuildCoordinator {
    state: Mutex<CoordinatorState>,
    compiler: IncrementalCompiler,
}

impl fmt::Debug for IncrementalBuildCoordinator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let state = self.lock_state();

        formatter
            .debug_struct("IncrementalBuildCoordinator")
            .field("tracked_builds", &state.snapshots.len())
            .field(
                "compiler_artifacts",
                &self.compiler.artifact_count(),
            )
            .field(
                "dirty_artifacts",
                &self.compiler.dirty_count(),
            )
            .finish()
    }
}

// ============================================================================
// COORDINATOR IMPLEMENTATION
// ============================================================================

impl IncrementalBuildCoordinator {
    /// Construct an incremental build coordinator.
    ///
    /// Dependency tracking and invalidation remain active.
    ///
    /// Cache storage is not duplicated here. The compiler's canonical
    /// incremental subsystem owns cache integration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Mutex::new(CoordinatorState::default()),
            compiler: IncrementalCompiler::without_cache(),
        }
    }

    /// Construct a coordinator around an existing compiler incremental
    /// coordinator.
    ///
    /// This permits the application to share its existing compiler
    /// incremental state and cache infrastructure.
    #[must_use]
    pub fn with_compiler(
        compiler: IncrementalCompiler,
    ) -> Self {
        Self {
            state: Mutex::new(CoordinatorState::default()),
            compiler,
        }
    }

    /// Return the canonical compiler incremental coordinator.
    ///
    /// This is the authority for dependency invalidation and artifact
    /// lifecycle state.
    #[must_use]
    pub fn compiler(&self) -> &IncrementalCompiler {
        &self.compiler
    }

    /// Record and compare an immutable build-input snapshot.
    ///
    /// This operation:
    ///
    /// 1. Validates the logical build identity.
    /// 2. Compares logical fingerprints.
    /// 3. Compares target-realization fingerprints.
    /// 4. Registers logical and realization artifact identities.
    /// 5. Establishes realization-to-logical dependency relationships.
    /// 6. Delegates changed-input invalidation to the compiler.
    /// 7. Invalidates removed realizations.
    /// 8. Stores the new snapshot after successful coordination.
    ///
    /// A new snapshot is not evidence of successful compilation.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identities or failed compiler
    /// dependency/invalidation operations.
    pub fn update_inputs(
        &self,
        build_identity: impl Into<String>,
        inputs: &BuildInputs,
    ) -> ToolchainResult<IncrementalBuildReport> {
        let build_identity = build_identity.into();

        validate_identity(
            &build_identity,
            "logical build identity",
        )?;

        // Construct the complete next snapshot before mutating state.
        let next = RecordedSnapshot::from_inputs(inputs);

        let mut state = self.lock_state();

        let previous = state.snapshots.get(&build_identity).cloned();

        let previous_logical = previous
            .as_ref()
            .map(|snapshot| snapshot.logical_fingerprint);

        let logical_changed = previous_logical
            .map(|fingerprint| {
                fingerprint != next.logical_fingerprint
            })
            .unwrap_or(true);

        let previous_realizations = previous
            .as_ref()
            .map(|snapshot| snapshot.realizations.clone())
            .unwrap_or_default();

        let mut realization_changes = Vec::new();

        for identity in previous_realizations
            .keys()
            .chain(next.realizations.keys())
            .collect::<BTreeSet<_>>()
        {
            let old = previous_realizations.get(identity.as_str()).copied();
            let new = next.realizations.get(identity.as_str()).copied();

            if old != new {
                realization_changes.push(RealizationChange {
                    identity: identity.clone(),
                    previous: old,
                    current: new,
                });
            }
        }

        let realization_changed = !realization_changes.is_empty();

        let change_kind = match (
            previous.is_none(),
            logical_changed,
            realization_changed,
        ) {
            (true, _, _) => IncrementalChangeKind::InitialSnapshot,

            (false, false, false) => IncrementalChangeKind::Unchanged,

            (false, true, false) => {
                IncrementalChangeKind::LogicalInputsChanged
            }

            (false, false, true) => {
                IncrementalChangeKind::RealizationInputsChanged
            }

            (false, true, true) => {
                IncrementalChangeKind::LogicalAndRealizationInputsChanged
            }
        };

        let mut invalidated = BTreeSet::<String>::new();

        // Register the logical build artifact.
        let logical_id = logical_artifact_id(&build_identity)?;
        let logical_key = logical_id.dependency_key()?;

        self.compiler
            .register_artifact(logical_key.as_str())
            .map_err(compiler_error)?;

        // Register every current realization and establish its dependency
        // on the logical build input artifact.
        //
        // The dependency direction is:
        //
        // realization artifact -> logical build artifact
        //
        // A logical-input change therefore invalidates all realizations.
        for identity in next.realizations.keys() {
            let realization_id = realization_artifact_id(
                &build_identity,
                identity,
            )?;

            let realization_key = realization_id.dependency_key()?;

            self.compiler
                .register_artifact(realization_key.as_str())
                .map_err(compiler_error)?;

            self.compiler
                .add_dependency(
                    &realization_key,
                    &logical_key,
                    DependencyKind::Semantic,
                )
                .map_err(compiler_error)?;
        }

        // Update the logical identity first. The compiler coordinator
        // performs transitive invalidation using its canonical graph.
        if logical_changed {
            if let Some(result) = self.compiler
                .update_fingerprint(
                    &logical_key,
                    next.logical_fingerprint,
                )
                .map_err(compiler_error)?
            {
                for artifact in result.invalidated_artifacts() {
                    invalidated.insert(
                        artifact.as_str().to_owned(),
                    );
                }
            }
        }

        // Update only the realizations whose own fingerprints changed.
        //
        // Unrelated realizations remain reusable when their complete
        // authoritative fingerprints remain identical.
        for change in &realization_changes {
            let Some(current) = change.current() else {
                continue;
            };

            let realization_id = realization_artifact_id(
                &build_identity,
                change.identity(),
            )?;

            let realization_key = realization_id.dependency_key()?;

            if let Some(result) = self.compiler
                .update_fingerprint(
                    &realization_key,
                    current,
                )
                .map_err(compiler_error)?
            {
                for artifact in result.invalidated_artifacts() {
                    invalidated.insert(
                        artifact.as_str().to_owned(),
                    );
                }
            }
        }

        // A realization removed from the requested build matrix must
        // not remain registered as a reusable build realization.
        for change in &realization_changes {
            if !change.was_removed() {
                continue;
            }

            let realization_id = realization_artifact_id(
                &build_identity,
                change.identity(),
            )?;

            let realization_key = realization_id.dependency_key()?;

            if self.compiler
                .remove_artifact(&realization_key)
                .map_err(compiler_error)?
            {
                invalidated.insert(
                    realization_key.as_str().to_owned(),
                );
            }
        }

        // Commit the snapshot only after successful coordination.
        //
        // If coordination fails, the old snapshot remains available and
        // a retry can safely repeat idempotent registrations/updates.
        state.snapshots.insert(
            build_identity.clone(),
            next.clone(),
        );

        Ok(IncrementalBuildReport {
            build_identity,
            change_kind,
            previous_logical_fingerprint: previous_logical,
            current_logical_fingerprint: next.logical_fingerprint,
            realization_changes,
            invalidated_artifacts: invalidated.into_iter().collect(),
        })
    }

    /// Register a dependency between two artifacts in one logical build.
    ///
    /// `dependent` requires `prerequisite`.
    ///
    /// If the prerequisite changes, the dependent is invalidated through
    /// the compiler's canonical dependency graph.
    ///
    /// Artifact identities must be stable logical identities, not content
    /// fingerprints.
    pub fn register_dependency(
        &self,
        build_identity: &str,
        dependent: &str,
        prerequisite: &str,
        kind: DependencyKind,
    ) -> ToolchainResult<bool> {
        validate_identity(
            build_identity,
            "logical build identity",
        )?;

        let dependent = IncrementalArtifactId::new(
            build_identity,
            dependent,
        )?;

        let prerequisite = IncrementalArtifactId::new(
            build_identity,
            prerequisite,
        )?;

        let dependent_key = dependent.dependency_key()?;
        let prerequisite_key = prerequisite.dependency_key()?;

        self.compiler
            .register_artifact(dependent_key.as_str())
            .map_err(compiler_error)?;

        self.compiler
            .register_artifact(prerequisite_key.as_str())
            .map_err(compiler_error)?;

        self.compiler
            .add_dependency(
                &dependent_key,
                &prerequisite_key,
                kind,
            )
            .map_err(compiler_error)
    }

    /// Register an artifact whose implementation is managed by an
    /// external compiler, backend or artifact provider.
    ///
    /// Registration does not mark the artifact clean.
    pub fn register_artifact(
        &self,
        identity: &IncrementalArtifactId,
    ) -> ToolchainResult<()> {
        let key = identity.dependency_key()?;

        self.compiler
            .register_artifact(key.as_str())
            .map_err(compiler_error)?;

        Ok(())
    }

    /// Record a changed fingerprint for a registered artifact.
    ///
    /// Transitive invalidation is delegated to the compiler incremental
    /// coordinator.
    pub fn update_artifact_fingerprint(
        &self,
        identity: &IncrementalArtifactId,
        fingerprint: Fingerprint,
    ) -> ToolchainResult<Vec<String>> {
        let key = identity.dependency_key()?;

        let result = self.compiler
            .update_fingerprint(&key, fingerprint)
            .map_err(compiler_error)?;

        let mut invalidated = BTreeSet::new();

        if let Some(result) = result {
            for artifact in result.invalidated_artifacts() {
                invalidated.insert(
                    artifact.as_str().to_owned(),
                );
            }
        }

        Ok(invalidated.into_iter().collect())
    }

    /// Record that an artifact was successfully produced, verified and
    /// committed by its authoritative external artifact destination.
    ///
    /// Call this only after the external operation has succeeded.
    ///
    /// This method does not perform compilation, artifact verification,
    /// signing or storage.
    ///
    /// A failed compilation must never call this method.
    pub fn record_successful_commit(
        &self,
        identity: &IncrementalArtifactId,
        fingerprint: Fingerprint,
    ) -> ToolchainResult<()> {
        let key = identity.dependency_key()?;

        self.compiler
            .register_artifact(key.as_str())
            .map_err(compiler_error)?;

        self.compiler
            .mark_clean_after_commit(
                &key,
                fingerprint,
            )
            .map_err(compiler_error)
    }

    /// Retrieve the current compiler-owned state of a registered artifact.
    pub fn artifact_state(
        &self,
        identity: &IncrementalArtifactId,
    ) -> ToolchainResult<ArtifactState> {
        let key = identity.dependency_key()?;

        self.compiler
            .artifact(&key)
            .map_err(compiler_error)
    }

    /// Determine whether an artifact is dirty.
    ///
    /// Unknown artifacts produce an error rather than being reported as
    /// clean.
    pub fn is_dirty(
        &self,
        identity: &IncrementalArtifactId,
    ) -> ToolchainResult<bool> {
        let key = identity.dependency_key()?;

        self.compiler
            .artifact(&key)
            .map_err(compiler_error)?;

        Ok(self.compiler.is_dirty(&key))
    }

    /// Return all dirty compiler artifact identities in deterministic order.
    #[must_use]
    pub fn dirty_artifacts(&self) -> Vec<String> {
        self.compiler
            .dirty_artifacts()
            .into_iter()
            .map(|identity| identity.as_str().to_owned())
            .collect()
    }

    /// Return the number of tracked logical build snapshots.
    #[must_use]
    pub fn tracked_build_count(&self) -> usize {
        self.lock_state().snapshots.len()
    }

    /// Remove a logical build snapshot and its incremental artifact state.
    ///
    /// The compiler coordinator invalidates dependent artifacts before
    /// removing each registered build artifact.
    ///
    /// This operation does not delete external artifact files or package
    /// contents.
    pub fn forget_build(
        &self,
        build_identity: &str,
    ) -> ToolchainResult<bool> {
        validate_identity(
            build_identity,
            "logical build identity",
        )?;

        let mut state = self.lock_state();

        let Some(snapshot) = state.snapshots.remove(build_identity) else {
            return Ok(false);
        };

        // Remove realization artifacts before the logical root.
        for identity in snapshot.realizations.keys() {
            let realization_id = realization_artifact_id(
                build_identity,
                identity,
            )?;

            let realization_key = realization_id.dependency_key()?;

            self.compiler
                .remove_artifact(&realization_key)
                .map_err(compiler_error)?;
        }

        let logical_id = logical_artifact_id(build_identity)?;
        let logical_key = logical_id.dependency_key()?;

        self.compiler
            .remove_artifact(&logical_key)
            .map_err(compiler_error)?;

        Ok(true)
    }

    // ------------------------------------------------------------------------
    // Synchronization
    // ------------------------------------------------------------------------

    fn lock_state(&self) -> MutexGuard<'_, CoordinatorState> {
        match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

impl Default for IncrementalBuildCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_identity_is_stable() {
        let first = IncrementalArtifactId::new(
            "project-a",
            "classical-module",
        )
        .expect("valid identity");

        let second = IncrementalArtifactId::new(
            "project-a",
            "classical-module",
        )
        .expect("valid identity");

        assert_eq!(
            first.dependency_key().expect("key"),
            second.dependency_key().expect("key"),
        );
    }

    #[test]
    fn artifact_identity_separates_builds() {
        let first = IncrementalArtifactId::new(
            "project-a",
            "module",
        )
        .expect("valid identity");

        let second = IncrementalArtifactId::new(
            "project-b",
            "module",
        )
        .expect("valid identity");

        assert_ne!(
            first.dependency_key().expect("key"),
            second.dependency_key().expect("key"),
        );
    }

    #[test]
    fn artifact_identity_separates_ambiguous_delimiters() {
        let first = IncrementalArtifactId::new(
            "project:a",
            "module",
        )
        .expect("valid identity");

        let second = IncrementalArtifactId::new(
            "project",
            "a:module",
        )
        .expect("valid identity");

        assert_ne!(
            first.dependency_key().expect("key"),
            second.dependency_key().expect("key"),
        );
    }

    #[test]
    fn empty_build_identity_is_rejected() {
        assert!(
            IncrementalArtifactId::new(
                "",
                "module",
            )
            .is_err()
        );
    }

    #[test]
    fn empty_artifact_identity_is_rejected() {
        assert!(
            IncrementalArtifactId::new(
                "project",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn logical_scope_has_no_realization_identity() {
        assert_eq!(
            BuildArtifactScope::Logical
                .realization_identity(),
            None,
        );
    }

    #[test]
    fn realization_scope_preserves_identity() {
        let scope = BuildArtifactScope::target_realization(
            "target-provider:realization-1",
        )
        .expect("valid realization");

        assert_eq!(
            scope.realization_identity(),
            Some("target-provider:realization-1"),
        );
    }

    #[test]
    fn coordinator_starts_without_tracked_builds() {
        let coordinator = IncrementalBuildCoordinator::new();

        assert_eq!(
            coordinator.tracked_build_count(),
            0,
        );

        assert!(coordinator.dirty_artifacts().is_empty());
    }

    #[test]
    fn compiler_artifacts_are_initially_dirty() {
        let coordinator = IncrementalBuildCoordinator::new();

        let artifact = IncrementalArtifactId::new(
            "project-a",
            "module-a",
        )
        .expect("identity");

        coordinator
            .register_artifact(&artifact)
            .expect("registration");

        assert!(
            coordinator
                .is_dirty(&artifact)
                .expect("artifact state")
        );
    }

    #[test]
    fn successful_commit_marks_artifact_clean() {
        let coordinator = IncrementalBuildCoordinator::new();

        let artifact = IncrementalArtifactId::new(
            "project-a",
            "module-a",
        )
        .expect("identity");

        let fingerprint = Fingerprint::from_bytes([7_u8; 32]);

        coordinator
            .record_successful_commit(
                &artifact,
                fingerprint,
            )
            .expect("successful commit");

        assert!(
            !coordinator
                .is_dirty(&artifact)
                .expect("artifact state")
        );
    }

    #[test]
    fn changed_artifact_invalidates_dependent() {
        let coordinator = IncrementalBuildCoordinator::new();

        let source = IncrementalArtifactId::new(
            "project-a",
            "source",
        )
        .expect("source");

        let compiled = IncrementalArtifactId::new(
            "project-a",
            "compiled",
        )
        .expect("compiled");

        coordinator
            .register_artifact(&source)
            .expect("source registration");

        coordinator
            .register_artifact(&compiled)
            .expect("compiled registration");

        coordinator
            .register_dependency(
                "project-a",
                "compiled",
                "source",
                DependencyKind::Source,
            )
            .expect("dependency");

        coordinator
            .record_successful_commit(
                &source,
                Fingerprint::from_bytes([1_u8; 32]),
            )
            .expect("source commit");

        coordinator
            .record_successful_commit(
                &compiled,
                Fingerprint::from_bytes([2_u8; 32]),
            )
            .expect("compiled commit");

        coordinator
            .update_artifact_fingerprint(
                &source,
                Fingerprint::from_bytes([3_u8; 32]),
            )
            .expect("fingerprint update");

        assert!(
            coordinator
                .is_dirty(&source)
                .expect("source state")
        );

        assert!(
            coordinator
                .is_dirty(&compiled)
                .expect("compiled state")
        );
    }

    #[test]
    fn unchanged_artifact_fingerprint_does_not_invalidate() {
        let coordinator = IncrementalBuildCoordinator::new();

        let artifact = IncrementalArtifactId::new(
            "project-a",
            "source",
        )
        .expect("identity");

        let fingerprint = Fingerprint::from_bytes([9_u8; 32]);

        coordinator
            .register_artifact(&artifact)
            .expect("registration");

        coordinator
            .record_successful_commit(
                &artifact,
                fingerprint,
            )
            .expect("commit");

        let invalidated = coordinator
            .update_artifact_fingerprint(
                &artifact,
                fingerprint,
            )
            .expect("update");

        assert!(invalidated.is_empty());

        assert!(
            !coordinator
                .is_dirty(&artifact)
                .expect("state")
        );
    }

    #[test]
    fn unknown_artifact_is_not_reported_as_clean() {
        let coordinator = IncrementalBuildCoordinator::new();

        let artifact = IncrementalArtifactId::new(
            "project-a",
            "unknown",
        )
        .expect("identity");

        assert!(
            coordinator.is_dirty(&artifact).is_err()
        );
    }
}
