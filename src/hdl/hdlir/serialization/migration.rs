
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Versioned HDLIR artifact migrations
//!
//! Path: `src/hdl/hdlir/serialization/migration.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide an explicit, deterministic registry of version-to-version
//! transformations for serialized HDLIR payloads and their artifact metadata.
//!
//! A migration is a registered transformation from one exact
//! `HdlirVersion` to another. A multi-step migration is possible only when
//! every edge in the path is registered. The registry never guesses how an
//! older payload should be interpreted.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//! - `src/hdl/hdlir/serialization/encode.rs`
//! - `src/hdl/hdlir/serialization/decode.rs`
//!
//! The grammar owns source syntax. The frontend owns source semantics.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain representation. This module owns migration orchestration,
//! not the semantics of each individual historical payload.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Explicit migration-step registration and lookup.
//! - Deterministic migration-path discovery.
//! - Migration execution with configurable step and output-size budgets.
//! - Updating the artifact-format version after each successful step.
//! - Invalidating payload integrity and semantic fingerprints when payload
//!   bytes change.
//! - Structured errors for missing paths, failed transformations, and
//!   exhausted budgets.
//!
//! This module does not own:
//!
//! - Artifact framing, JSON parsing, or untrusted byte-stream decoding.
//! - HDLIR payload encoding or graph construction.
//! - Schema/dialect registration or source-level semantic analysis.
//! - The implementation of a particular historical migration.
//! - Proof of semantic equivalence.
//! - Synthesis, scheduling, physical allocation, placement, or routing.
//!
//! ## Migration contract
//!
//! Each registered transformation MUST:
//!
//! - Declare its exact source and destination format versions.
//! - Reject or explicitly handle payload structures it does not understand.
//! - Preserve all semantics covered by its migration contract, or fail.
//! - Avoid silently discarding unknown required features or extensions.
//! - Use checked arithmetic and explicit resource handling.
//! - Be deterministic when its contract promises deterministic output.
//! - Avoid panics and avoid modifying shared global state.
//!
//! The registry cannot prove these properties for user-supplied callbacks.
//! Registration is an assertion by the migration implementation author.
//! Production qualification must test each migration against historical
//! fixtures and the relevant semantic-preservation checks.
//!
//! ## Scalability
//!
//! This module introduces no hardware-capacity limits. The optional budgets
//! below limit migration work, not design semantics. An absent budget means
//! the caller has chosen not to impose that particular limit; it does not
//! promise infinite memory or processing capacity.
//!
//! ## Integration
//!
//! - `format.rs` supplies the validated artifact metadata contract.
//! - `version.rs` supplies `HdlirVersion` and version identity.
//! - `decode.rs` remains responsible for framing, metadata parsing, and
//!   payload-integrity verification.
//! - `encode.rs` re-encodes the migrated metadata and payload into a new
//!   artifact, recalculating payload integrity as configured.
//! - A payload codec and HDLIR verifier must validate the migrated payload
//!   before the result is treated as usable HDLIR.
//!
//! **Decoder integration requirement:** the current `decode_artifact` path
//! checks compatibility against `HDLIR_FORMAT_VERSION`. To migrate an
//! artifact that this check rejects, the serialization facade must expose a
//! migration-specific decoding path that validates framing, metadata
//! invariants, and declared integrity while deferring the final format-version
//! compatibility decision to this registry. Do not weaken ordinary decoding
//! or duplicate the framing parser in this module.

use core::fmt;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use crate::hdl::hdlir::serialization::format::{ArtifactFormat, FormatError};
use crate::hdl::hdlir::version::HdlirVersion;

/// Callback that transforms a payload from one exact format version to
/// another.
///
/// The callback receives validated artifact metadata and immutable source
/// payload bytes. It returns newly owned destination payload bytes.
///
/// Implementations must not mutate shared state, panic, or silently discard
/// semantics. They must return an error when the source payload or any
/// required extension cannot be migrated correctly.
///
/// This callback does not itself establish semantic equivalence.
pub type MigrationTransform = dyn Fn(
        &ArtifactFormat,
        &[u8],
    ) -> Result<Vec<u8>, MigrationTransformError>
    + Send
    + Sync
    + 'static;

/// Failure reported by an individual registered transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationTransformError {
    message: String,
}

impl MigrationTransformError {
    /// Create a transformation failure with an explanatory message.
    ///
    /// The message should describe the unsupported or invalid condition.
    /// It must not contain secrets from the artifact payload.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Return the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for MigrationTransformError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for MigrationTransformError {}

/// One explicitly registered migration edge.
#[derive(Clone)]
pub struct MigrationStep {
    from: HdlirVersion,
    to: HdlirVersion,
    transform: Arc<MigrationTransform>,
}

impl fmt::Debug for MigrationStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MigrationStep")
            .field("from", &self.from)
            .field("to", &self.to)
            .finish_non_exhaustive()
    }
}

impl MigrationStep {
    /// Construct a migration step between two distinct format versions.
    ///
    /// Version edges are exact: a callback registered for one source version
    /// is not assumed to accept another version, even if both share a major
    /// version.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationError::IdentityStep`] if both versions are equal.
    pub fn new<F>(
        from: HdlirVersion,
        to: HdlirVersion,
        transform: F,
    ) -> Result<Self, MigrationError>
    where
        F: Fn(
                &ArtifactFormat,
                &[u8],
            ) -> Result<Vec<u8>, MigrationTransformError>
            + Send
            + Sync
            + 'static,
    {
        if from == to {
            return Err(MigrationError::IdentityStep { version: from });
        }

        Ok(Self {
            from,
            to,
            transform: Arc::new(transform),
        })
    }

    /// Return the exact source format version.
    #[must_use]
    pub const fn from_version(&self) -> HdlirVersion {
        self.from
    }

    /// Return the exact destination format version.
    #[must_use]
    pub const fn to_version(&self) -> HdlirVersion {
        self.to
    }
}

/// Registry of explicitly implemented migration steps.
///
/// The registry is keyed by the exact `(source, destination)` pair. Duplicate
/// edges are rejected, so registering a new callback cannot silently replace
/// an existing migration implementation.
///
/// When multiple paths exist, path selection is deterministic: breadth-first
/// search chooses a path with the fewest registered steps, breaking ties by
/// the ordered version-pair keys.
#[derive(Clone, Default)]
pub struct MigrationRegistry {
    steps: BTreeMap<(HdlirVersion, HdlirVersion), MigrationStep>,
}

impl MigrationRegistry {
    /// Create an empty migration registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an explicit migration step.
    ///
    /// The registry does not infer missing intermediate versions or synthesize
    /// migration callbacks.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationError::DuplicateStep`] if this exact edge already
    /// exists.
    pub fn register(&mut self, step: MigrationStep) -> Result<(), MigrationError> {
        let key = (step.from, step.to);

        if self.steps.contains_key(&key) {
            return Err(MigrationError::DuplicateStep {
                from: step.from,
                to: step.to,
            });
        }

        self.steps.insert(key, step);
        Ok(())
    }

    /// Return whether an exact migration edge is registered.
    #[must_use]
    pub fn contains_step(
        &self,
        from: HdlirVersion,
        to: HdlirVersion,
    ) -> bool {
        self.steps.contains_key(&(from, to))
    }

    /// Return the number of registered migration edges.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Find a deterministic shortest path of registered migration steps.
    ///
    /// The returned list is empty when `from == to`. No path is inferred from
    /// version-number proximity, shared major versions, or patch numbers.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationError::NoPath`] when no registered path reaches the
    /// destination.
    pub fn migration_path(
        &self,
        from: HdlirVersion,
        to: HdlirVersion,
    ) -> Result<Vec<MigrationStep>, MigrationError> {
        if from == to {
            return Ok(Vec::new());
        }

        let mut queue = VecDeque::new();
        let mut predecessor: BTreeMap<
            HdlirVersion,
            (HdlirVersion, (HdlirVersion, HdlirVersion)),
        > = BTreeMap::new();

        predecessor.insert(from, (from, (from, from)));
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            // BTreeMap iteration guarantees stable ordering of outgoing edges.
            for (&(edge_from, edge_to), _) in self.steps.range(
                (current, HdlirVersion::new(0, 0, 0))
                    ..=(current, HdlirVersion::new(u32::MAX, u32::MAX, u32::MAX)),
            ) {
                if edge_from != current || predecessor.contains_key(&edge_to) {
                    continue;
                }

                predecessor.insert(edge_to, (current, (edge_from, edge_to)));

                if edge_to == to {
                    let mut path_keys = Vec::new();
                    let mut cursor = to;

                    while cursor != from {
                        let (previous, edge_key) = predecessor
                            .get(&cursor)
                            .copied()
                            .ok_or(MigrationError::InternalPathInconsistency)?;

                        path_keys.push(edge_key);
                        cursor = previous;
                    }

                    path_keys.reverse();

                    let mut path = Vec::new();
                    path.try_reserve(path_keys.len()).map_err(|_| {
                        MigrationError::AllocationFailed {
                            resource: "migration path",
                            requested: path_keys.len(),
                        }
                    })?;

                    for key in path_keys {
                        let step = self
                            .steps
                            .get(&key)
                            .ok_or(MigrationError::InternalPathInconsistency)?;
                        path.push(step.clone());
                    }

                    return Ok(path);
                }

                queue.push_back(edge_to);
            }
        }

        Err(MigrationError::NoPath { from, to })
    }

    /// Migrate an artifact's validated metadata and opaque payload to a
    /// requested destination format version.
    ///
    /// This function operates on already-extracted metadata and payload bytes.
    /// It does not parse artifact framing and does not establish that the
    /// payload is a valid HDLIR graph.
    ///
    /// The source metadata must pass `ArtifactFormat::validate()`, and its
    /// declared version must match the first registered migration edge.
    /// After every successful transformation, the metadata version is updated
    /// and stale integrity/fingerprint metadata is cleared. The canonical
    /// encoder can subsequently calculate a fresh payload digest.
    ///
    /// # Errors
    ///
    /// Returns a structured error for invalid metadata, a missing path,
    /// version mismatch, transformation failure, output-budget exhaustion,
    /// or allocation failure.
    pub fn migrate(
        &self,
        format: &ArtifactFormat,
        payload: &[u8],
        target: HdlirVersion,
        options: &MigrationOptions,
    ) -> Result<MigratedArtifact, MigrationError> {
        format
            .validate()
            .map_err(MigrationError::InvalidFormat)?;

        let source = format.format_version();
        let path = self.migration_path(source, target)?;

        if let Some(max_steps) = options.max_steps {
            if path.len() > max_steps {
                return Err(MigrationError::StepBudgetExceeded {
                    limit: max_steps,
                    required: path.len(),
                });
            }
        }

        let mut current_format = format.clone();
        let mut current_payload = Vec::new();

        current_payload
            .try_reserve_exact(payload.len())
            .map_err(|_| MigrationError::AllocationFailed {
                resource: "source payload copy",
                requested: payload.len(),
            })?;
        current_payload.extend_from_slice(payload);

        enforce_payload_budget(&current_payload, options)?;

        for (index, step) in path.iter().enumerate() {
            if current_format.format_version() != step.from {
                return Err(MigrationError::UnexpectedStepSource {
                    expected: step.from,
                    actual: current_format.format_version(),
                });
            }

            let next_payload = (step.transform)(&current_format, &current_payload)
                .map_err(|source| MigrationError::TransformFailed {
                    from: step.from,
                    to: step.to,
                    step_index: index,
                    source,
                })?;

            enforce_payload_budget(&next_payload, options)?;

            current_format.set_format_version(step.to);

            // The previous digest describes the old bytes and is now stale.
            // A semantic fingerprint also cannot be retained unless its
            // protocol explicitly proves that the fingerprint remains valid.
            current_format.set_integrity(None);
            current_format
                .set_semantic_fingerprint(None)
                .map_err(MigrationError::InvalidFormat)?;

            current_format
                .validate()
                .map_err(MigrationError::InvalidFormat)?;

            current_payload = next_payload;
        }

        if current_format.format_version() != target {
            return Err(MigrationError::DestinationMismatch {
                expected: target,
                actual: current_format.format_version(),
            });
        }

        Ok(MigratedArtifact {
            format: current_format,
            payload: current_payload,
            source_version: source,
            target_version: target,
            steps_applied: path.len(),
        })
    }
}

/// Operational budgets for migration.
///
/// No defaults impose arbitrary hardware or artifact-size ceilings. Callers
/// can supply limits appropriate to their trust boundary and available
/// resources.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MigrationOptions {
    max_steps: Option<usize>,
    max_output_payload_bytes: Option<usize>,
}

impl MigrationOptions {
    /// Create migration options without explicit operational budgets.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_steps: None,
            max_output_payload_bytes: None,
        }
    }

    /// Limit the number of migration steps in a selected path.
    #[must_use]
    pub const fn with_max_steps(mut self, limit: Option<usize>) -> Self {
        self.max_steps = limit;
        self
    }

    /// Limit the payload size accepted after each migration step.
    ///
    /// The limit is checked after a callback returns. Individual callbacks
    /// should independently enforce their own temporary-memory and work
    /// budgets because this outer check cannot prevent a callback from
    /// allocating an oversized intermediate result.
    #[must_use]
    pub const fn with_max_output_payload_bytes(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_output_payload_bytes = limit;
        self
    }

    /// Return the configured migration-step budget.
    #[must_use]
    pub const fn max_steps(self) -> Option<usize> {
        self.max_steps
    }

    /// Return the configured output-payload budget.
    #[must_use]
    pub const fn max_output_payload_bytes(self) -> Option<usize> {
        self.max_output_payload_bytes
    }
}

/// Successfully migrated artifact metadata and payload bytes.
///
/// This is not a verified HDLIR graph. Callers must encode the artifact using
/// `serialization/encode.rs`, decode the payload with the canonical payload
/// codec, and run the applicable HDLIR verification before treating it as
/// usable IR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigratedArtifact {
    format: ArtifactFormat,
    payload: Vec<u8>,
    source_version: HdlirVersion,
    target_version: HdlirVersion,
    steps_applied: usize,
}

impl MigratedArtifact {
    /// Return the migrated artifact metadata.
    #[must_use]
    pub fn format(&self) -> &ArtifactFormat {
        &self.format
    }

    /// Return the migrated payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Consume this result and return its metadata and payload.
    #[must_use]
    pub fn into_parts(self) -> (ArtifactFormat, Vec<u8>) {
        (self.format, self.payload)
    }

    /// Return the original format version.
    #[must_use]
    pub const fn source_version(&self) -> HdlirVersion {
        self.source_version
    }

    /// Return the destination format version.
    #[must_use]
    pub const fn target_version(&self) -> HdlirVersion {
        self.target_version
    }

    /// Return the number of migration steps applied.
    #[must_use]
    pub const fn steps_applied(&self) -> usize {
        self.steps_applied
    }
}

/// Failures encountered while discovering or executing a migration.
#[derive(Debug)]
#[non_exhaustive]
pub enum MigrationError {
    /// Artifact metadata did not pass its own validation.
    InvalidFormat(FormatError),

    /// A no-op migration step was registered.
    IdentityStep {
        /// The version used as both source and destination.
        version: HdlirVersion,
    },

    /// A migration edge was registered more than once.
    DuplicateStep {
        /// Registered source version.
        from: HdlirVersion,
        /// Registered destination version.
        to: HdlirVersion,
    },

    /// No sequence of registered steps reaches the requested destination.
    NoPath {
        /// Starting version.
        from: HdlirVersion,
        /// Requested destination version.
        to: HdlirVersion,
    },

    /// The metadata version does not match the registered step source.
    UnexpectedStepSource {
        /// Source version required by the step.
        expected: HdlirVersion,
        /// Version actually present in metadata.
        actual: HdlirVersion,
    },

    /// The transformation callback rejected the artifact.
    TransformFailed {
        /// Source version of the failed step.
        from: HdlirVersion,
        /// Destination version of the failed step.
        to: HdlirVersion,
        /// Zero-based position of the step in the selected path.
        step_index: usize,
        /// Callback-provided reason.
        source: MigrationTransformError,
    },

    /// The selected migration path exceeds its configured step budget.
    StepBudgetExceeded {
        /// Configured maximum number of steps.
        limit: usize,
        /// Number of steps required by the selected path.
        required: usize,
    },

    /// A migration output exceeds its configured payload budget.
    PayloadBudgetExceeded {
        /// Configured maximum output size.
        limit: usize,
        /// Actual or required output size.
        actual: usize,
    },

    /// A required allocation failed.
    AllocationFailed {
        /// Resource being allocated.
        resource: &'static str,
        /// Requested capacity.
        requested: usize,
    },

    /// The final metadata version did not match the requested destination.
    DestinationMismatch {
        /// Requested destination version.
        expected: HdlirVersion,
        /// Version returned by migration execution.
        actual: HdlirVersion,
    },

    /// The internally reconstructed path was inconsistent with the registry.
    InternalPathInconsistency,
}

impl fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(error) => {
                write!(formatter, "invalid HDLIR artifact metadata: {error}")
            }
            Self::IdentityStep { version } => {
                write!(formatter, "migration step cannot map {version} to itself")
            }
            Self::DuplicateStep { from, to } => {
                write!(formatter, "duplicate HDLIR migration step {from} -> {to}")
            }
            Self::NoPath { from, to } => {
                write!(formatter, "no registered HDLIR migration path from {from} to {to}")
            }
            Self::UnexpectedStepSource { expected, actual } => write!(
                formatter,
                "migration step expected source version {expected}, got {actual}"
            ),
            Self::TransformFailed {
                from,
                to,
                step_index,
                source,
            } => write!(
                formatter,
                "HDLIR migration step {step_index} ({from} -> {to}) failed: {source}"
            ),
            Self::StepBudgetExceeded { limit, required } => write!(
                formatter,
                "HDLIR migration requires {required} steps, exceeding the configured limit of {limit}"
            ),
            Self::PayloadBudgetExceeded { limit, actual } => write!(
                formatter,
                "HDLIR migration payload is {actual} bytes, exceeding the configured limit of {limit}"
            ),
            Self::AllocationFailed { resource, requested } => write!(
                formatter,
                "could not allocate {requested} bytes for HDLIR {resource}"
            ),
            Self::DestinationMismatch { expected, actual } => write!(
                formatter,
                "HDLIR migration requested destination {expected} but produced {actual}"
            ),
            Self::InternalPathInconsistency => {
                formatter.write_str("HDLIR migration registry produced an inconsistent path")
            }
        }
    }
}

impl std::error::Error for MigrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFormat(error) => Some(error),
            Self::TransformFailed { source, .. } => Some(source),
            Self::IdentityStep { .. }
            | Self::DuplicateStep { .. }
            | Self::NoPath { .. }
            | Self::UnexpectedStepSource { .. }
            | Self::StepBudgetExceeded { .. }
            | Self::PayloadBudgetExceeded { .. }
            | Self::AllocationFailed { .. }
            | Self::DestinationMismatch { .. }
            | Self::InternalPathInconsistency => None,
        }
    }
}

fn enforce_payload_budget(
    payload: &[u8],
    options: &MigrationOptions,
) -> Result<(), MigrationError> {
    if let Some(limit) = options.max_output_payload_bytes {
        if payload.len() > limit {
            return Err(MigrationError::PayloadBudgetExceeded {
                limit,
                actual: payload.len(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::serialization::format::ArtifactFormat;

    fn version(major: u32, minor: u32, patch: u32) -> HdlirVersion {
        HdlirVersion::new(major, minor, patch)
    }

    #[test]
    fn rejects_identity_migration_step() {
        let current = version(1, 0, 0);

        let result = MigrationStep::new(current, current, |_, payload| {
            Ok(payload.to_vec())
        });

        assert!(matches!(
            result,
            Err(MigrationError::IdentityStep { version }) if version == current
        ));
    }

    #[test]
    fn rejects_duplicate_edges() {
        let mut registry = MigrationRegistry::new();
        let from = version(1, 0, 0);
        let to = version(1, 1, 0);

        registry
            .register(
                MigrationStep::new(from, to, |_, payload| Ok(payload.to_vec()))
                    .expect("valid edge"),
            )
            .expect("first registration");

        let duplicate =
            MigrationStep::new(from, to, |_, payload| Ok(payload.to_vec()))
                .expect("valid edge");

        assert!(matches!(
            registry.register(duplicate),
            Err(MigrationError::DuplicateStep { .. })
        ));
    }

    #[test]
    fn migrates_through_explicit_steps_and_clears_stale_metadata() {
        let v1 = version(1, 0, 0);
        let v2 = version(1, 1, 0);
        let v3 = version(1, 2, 0);

        let mut registry = MigrationRegistry::new();
        registry
            .register(
                MigrationStep::new(v1, v2, |_, payload| {
                    let mut output = b"v2:".to_vec();
                    output.extend_from_slice(payload);
                    Ok(output)
                })
                .expect("valid v1 -> v2 step"),
            )
            .expect("register v1 -> v2");

        registry
            .register(
                MigrationStep::new(v2, v3, |_, payload| {
                    let mut output = b"v3:".to_vec();
                    output.extend_from_slice(payload);
                    Ok(output)
                })
                .expect("valid v2 -> v3 step"),
            )
            .expect("register v2 -> v3");

        let mut format = ArtifactFormat::new();
        format.set_format_version(v1);
        format
            .set_semantic_fingerprint(Some("aabb".to_owned()))
            .expect("valid fingerprint");

        let result = registry
            .migrate(
                &format,
                b"design",
                v3,
                &MigrationOptions::new(),
            )
            .expect("migration should succeed");

        assert_eq!(result.source_version(), v1);
        assert_eq!(result.target_version(), v3);
        assert_eq!(result.steps_applied(), 2);
        assert_eq!(result.payload(), b"v3:v2:design");
        assert_eq!(result.format().format_version(), v3);
        assert_eq!(result.format().semantic_fingerprint_hex(), None);
        assert_eq!(result.format().integrity(), None);
    }

    #[test]
    fn refuses_to_guess_a_missing_path() {
        let registry = MigrationRegistry::new();

        assert!(matches!(
            registry.migration_path(version(1, 0, 0), version(2, 0, 0)),
            Err(MigrationError::NoPath { .. })
        ));
    }

    #[test]
    fn enforces_step_budget_before_running_callbacks() {
        let from = version(1, 0, 0);
        let middle = version(1, 1, 0);
        let to = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();
        registry
            .register(
                MigrationStep::new(from, middle, |_, payload| {
                    Ok(payload.to_vec())
                })
                .expect("valid edge"),
            )
            .expect("register first edge");
        registry
            .register(
                MigrationStep::new(middle, to, |_, payload| {
                    Ok(payload.to_vec())
                })
                .expect("valid edge"),
            )
            .expect("register second edge");

        let result = registry.migrate(
            &{
                let mut format = ArtifactFormat::new();
                format.set_format_version(from);
                format
            },
            b"payload",
            to,
            &MigrationOptions::new().with_max_steps(Some(1)),
        );

        assert!(matches!(
            result,
            Err(MigrationError::StepBudgetExceeded {
                limit: 1,
                required: 2
            })
        ));
    }

    #[test]
    fn rejects_oversized_migration_output() {
        let from = version(1, 0, 0);
        let to = version(1, 1, 0);
        let mut registry = MigrationRegistry::new();

        registry
            .register(
                MigrationStep::new(from, to, |_, _| Ok(vec![1, 2, 3]))
                    .expect("valid edge"),
            )
            .expect("register edge");

        let mut format = ArtifactFormat::new();
        format.set_format_version(from);

        let result = registry.migrate(
            &format,
            b"x",
            to,
            &MigrationOptions::new().with_max_output_payload_bytes(Some(2)),
        );

        assert!(matches!(
            result,
            Err(MigrationError::PayloadBudgetExceeded {
                limit: 2,
                actual: 3
            })
        ));
    }

    #[test]
    fn rejects_callback_failure_without_returning_partial_result() {
        let from = version(1, 0, 0);
        let to = version(1, 1, 0);
        let mut registry = MigrationRegistry::new();

        registry
            .register(
                MigrationStep::new(from, to, |_, _| {
                    Err(MigrationTransformError::new(
                        "legacy operation has no semantics-preserving mapping",
                    ))
                })
                .expect("valid edge"),
            )
            .expect("register edge");

        let mut format = ArtifactFormat::new();
        format.set_format_version(from);

        assert!(matches!(
            registry.migrate(
                &format,
                b"payload",
                to,
                &MigrationOptions::new()
            ),
            Err(MigrationError::TransformFailed { .. })
        ));
    }
}
