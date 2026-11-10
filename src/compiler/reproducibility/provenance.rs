
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Compiler — Reproducibility Provenance Fingerprinting.
//!
//! Path:
//!     src/compiler/reproducibility/provenance.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe code is forbidden.
//!
//! # Purpose
//!
//! Provide a deterministic, versioned content identity for the authoritative
//! compilation provenance ledger.
//!
//! This module fingerprints immutable provenance snapshots produced by:
//!
//!     crate::compiler::context::provenance_context
//!
//! It does not maintain another provenance ledger.
//!
//! # Canonical authorities
//!
//! Provenance storage:
//!     crate::compiler::context::provenance_context
//!
//! Provenance analysis:
//!     crate::compiler::analysis::provenance
//!
//! Fingerprint algorithm:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Source identity:
//!     crate::compiler::context::compilation_context
//!
//! Compilation inputs:
//!     crate::compiler::reproducibility::inputs
//!
//! Compilation environment:
//!     crate::compiler::reproducibility::environment
//!
//! Incremental fingerprints:
//!     crate::compiler::incremental::fingerprints
//!
//! Artifact metadata:
//!     crate::compiler::artifacts
//!
//! Security and trust:
//!     crate::compiler::security
//!
//! Normative semantic contract:
//!     grammar/spec/provenance.md
//!
//! Source syntax:
//!     grammar/compile/provenance.g4
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Provenance snapshot fingerprinting.
//! - Provenance fingerprint schema versioning.
//! - Canonical provenance event encoding.
//! - Canonical provenance relationship encoding.
//! - Provenance fingerprint construction errors.
//! - Immutable provenance fingerprint results.
//!
//! This module does not own:
//!
//! - Provenance event creation.
//! - Provenance ledger storage.
//! - Provenance identifiers.
//! - Provenance event validation.
//! - Source maps.
//! - Content hashing algorithms.
//! - Incremental cache keys.
//! - Dependency graph construction.
//! - Semantic interpretation.
//! - Canonical IR.
//! - Artifact serialization.
//! - Cryptographic signatures.
//! - Security authorization.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! # Determinism
//!
//! Fingerprinting must be independent of:
//!
//! - Event insertion order.
//! - Hash-map randomization.
//! - Thread completion order.
//! - Process identifiers.
//! - Memory addresses.
//! - Wall-clock time.
//! - Hostname.
//! - Filesystem enumeration order.
//! - Locale.
//!
//! ProvenanceSnapshot::topological_events provides deterministic causal
//! ordering with stable event identifiers as tie-breakers.
//!
//! The ledger revision is intentionally excluded because it describes
//! mutation history, not the canonical content of the provenance graph.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Provenance events.
//! - Causal relationships.
//! - Source references.
//! - Input artifacts.
//! - Output artifacts.
//! - Transformations.
//! - Computing domains.
//! - Providers.
//! - Targets.
//! - Provenance properties.
//!
//! Fingerprinting is incremental.
//!
//! No complete encoded provenance document is constructed in memory.
//!
//! Practical limits remain those of available memory, execution time,
//! address space and explicitly configured operational budgets.
//!
//! Literal infinite processing or storage capacity is not promised.
//!
//! # Security
//!
//! A fingerprint establishes content identity, not authenticity.
//!
//! A fingerprint does not prove:
//!
//! - That an event actually occurred.
//! - That a compiler is trustworthy.
//! - That an artifact is correct.
//! - That a transformation preserved semantics.
//! - That a producer was authorized.
//! - That a declared toolchain was genuinely used.
//!
//! Those guarantees require independent verification.
//!
//! Callers must not put credentials, passwords, private keys or other
//! secret material into provenance properties.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Provenance fingerprinting must work across classical, quantum, HDL,
//! AI, tensor, hybrid, distributed and future computing domains.
//!
//! It must not make provenance identity depend on an arbitrary universal
//! hardware enumeration or a fixed hardware capacity.

use std::error::Error;
use std::fmt;

use crate::compiler::context::provenance_context::{
    ContentDigest,
    ProvenanceArtifactReference,
    ProvenanceContext,
    ProvenanceDeterminism,
    ProvenanceError,
    ProvenanceEvent,
    ProvenanceOutcome,
    ProvenanceSnapshot,
    ProvenanceSourceReference,
};

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

// ============================================================================
// VERSIONING
// ============================================================================

/// Public API version of compiler provenance fingerprinting.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Provenance context API version.
/// - ZUIR version.
/// - Domain IR versions.
/// - Artifact format version.
/// - Backend ABI versions.
pub const PROVENANCE_FINGERPRINT_API_VERSION: u16 = 1;

/// Version of the canonical provenance fingerprint schema.
///
/// Increment this version when the selected fields, canonical ordering,
/// field interpretation or encoding changes incompatibly.
pub const PROVENANCE_FINGERPRINT_SCHEMA_VERSION: u32 = 1;

/// Canonical fingerprint domain for compilation provenance.
///
/// This domain is independent of the ZUIR fingerprint algorithm version.
pub const PROVENANCE_FINGERPRINT_DOMAIN: &str =
    "zamani.compiler.provenance-ledger.v1";

// ============================================================================
// RESULT
// ============================================================================

/// Immutable fingerprint of an authoritative provenance snapshot.
///
/// This type contains no provenance events and does not duplicate the
/// authoritative provenance ledger.
///
/// The fingerprint identifies the canonical encoded provenance graph.
///
/// It does not prove authenticity or semantic correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvenanceFingerprint {
    schema_version: u32,
    fingerprint: Fingerprint,
    event_count: u64,
}

impl ProvenanceFingerprint {
    /// Returns the provenance fingerprint schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the canonical content fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Returns the number of provenance events represented.
    #[must_use]
    pub const fn event_count(&self) -> u64 {
        self.event_count
    }

    /// Returns the hexadecimal fingerprint representation.
    #[must_use]
    pub fn to_hex(self) -> String {
        self.fingerprint.to_hex()
    }
}

impl fmt::Display for ProvenanceFingerprint {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}", self.fingerprint)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by provenance fingerprinting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceFingerprintError {
    /// The authoritative provenance context failed.
    Provenance(ProvenanceError),

    /// The canonical fingerprint implementation failed.
    Fingerprinting(FingerprintError),

    /// A collection count could not be represented by the schema.
    CountOverflow {
        /// Collection whose count could not be represented.
        collection: &'static str,
    },

    /// A future provenance classification cannot be encoded by this
    /// fingerprint schema without an explicit schema update.
    UnsupportedClassification {
        /// Classification category.
        category: &'static str,
    },
}

impl fmt::Display for ProvenanceFingerprintError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Provenance(error) => {
                write!(
                    formatter,
                    "provenance snapshot access failed: {error}"
                )
            }

            Self::Fingerprinting(error) => {
                write!(
                    formatter,
                    "provenance fingerprint construction failed: {error}"
                )
            }

            Self::CountOverflow { collection } => {
                write!(
                    formatter,
                    "provenance {collection} count overflowed"
                )
            }

            Self::UnsupportedClassification { category } => {
                write!(
                    formatter,
                    "unsupported provenance classification: {category}"
                )
            }
        }
    }
}

impl Error for ProvenanceFingerprintError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Provenance(error) => Some(error),
            Self::Fingerprinting(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ProvenanceError> for ProvenanceFingerprintError {
    fn from(error: ProvenanceError) -> Self {
        Self::Provenance(error)
    }
}

impl From<FingerprintError> for ProvenanceFingerprintError {
    fn from(error: FingerprintError) -> Self {
        Self::Fingerprinting(error)
    }
}

// ============================================================================
// COUNT VALIDATION
// ============================================================================

fn encode_count(
    builder: &mut StableHashBuilder,
    field: &'static str,
    count: usize,
) -> Result<(), ProvenanceFingerprintError> {
    let count = u64::try_from(count).map_err(|_| {
        ProvenanceFingerprintError::CountOverflow {
            collection: field,
        }
    })?;

    builder.field_u64(field, count)?;

    Ok(())
}

// ============================================================================
// OPTIONAL STRING ENCODING
// ============================================================================

fn encode_optional_string(
    builder: &mut StableHashBuilder,
    field: &'static str,
    value: Option<&str>,
) -> Result<(), ProvenanceFingerprintError> {
    match value {
        Some(value) => {
            builder.field_bool(
                &format!("{field}.present"),
                true,
            )?;

            builder.field_str(field, value)?;
        }

        None => {
            builder.field_bool(
                &format!("{field}.present"),
                false,
            )?;
        }
    }

    Ok(())
}

// ============================================================================
// DIGEST ENCODING
// ============================================================================

fn encode_optional_digest(
    builder: &mut StableHashBuilder,
    field: &'static str,
    digest: Option<&ContentDigest>,
) -> Result<(), ProvenanceFingerprintError> {
    match digest {
        Some(digest) => {
            builder.field_bool(
                &format!("{field}.present"),
                true,
            )?;

            builder.field_str(
                &format!("{field}.algorithm"),
                digest.algorithm(),
            )?;

            builder.field_str(
                &format!("{field}.value"),
                digest.value(),
            )?;
        }

        None => {
            builder.field_bool(
                &format!("{field}.present"),
                false,
            )?;
        }
    }

    Ok(())
}

// ============================================================================
// SOURCE ENCODING
// ============================================================================

fn encode_source(
    builder: &mut StableHashBuilder,
    source: &ProvenanceSourceReference,
) -> Result<(), ProvenanceFingerprintError> {
    builder.field_str(
        "source.logical-name",
        source.logical_name(),
    )?;

    match source.source_map() {
        Some(source_map) => {
            builder.field_bool(
                "source.source-map.present",
                true,
            )?;

            builder.field_str(
                "source.source-map.locator",
                source_map.locator(),
            )?;
        }

        None => {
            builder.field_bool(
                "source.source-map.present",
                false,
            )?;
        }
    }

    Ok(())
}

// ============================================================================
// ARTIFACT ENCODING
// ============================================================================

fn encode_artifact(
    builder: &mut StableHashBuilder,
    artifact: &ProvenanceArtifactReference,
) -> Result<(), ProvenanceFingerprintError> {
    builder.field_str(
        "artifact.identifier",
        artifact.identifier(),
    )?;

    builder.field_str(
        "artifact.kind",
        artifact.kind().as_str(),
    )?;

    encode_optional_string(
        builder,
        "artifact.format",
        artifact.format(),
    )?;

    encode_optional_digest(
        builder,
        "artifact.digest",
        artifact.digest(),
    )?;

    Ok(())
}

// ============================================================================
// DETERMINISM ENCODING
// ============================================================================

fn encode_determinism(
    builder: &mut StableHashBuilder,
    determinism: &ProvenanceDeterminism,
) -> Result<(), ProvenanceFingerprintError> {
    match determinism {
        ProvenanceDeterminism::Deterministic => {
            builder.field_str(
                "event.determinism",
                "deterministic",
            )?;
        }

        ProvenanceDeterminism::DeterministicGivenInputs => {
            builder.field_str(
                "event.determinism",
                "deterministic-given-inputs",
            )?;
        }

        ProvenanceDeterminism::Nondeterministic => {
            builder.field_str(
                "event.determinism",
                "nondeterministic",
            )?;
        }

        ProvenanceDeterminism::Unspecified => {
            builder.field_str(
                "event.determinism",
                "unspecified",
            )?;
        }

        ProvenanceDeterminism::Custom(value) => {
            builder.field_str(
                "event.determinism",
                "custom",
            )?;

            builder.field_str(
                "event.determinism.custom",
                value,
            )?;
        }

        _ => {
            return Err(
                ProvenanceFingerprintError::UnsupportedClassification {
                    category: "determinism",
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// OUTCOME ENCODING
// ============================================================================

fn encode_outcome(
    builder: &mut StableHashBuilder,
    outcome: &ProvenanceOutcome,
) -> Result<(), ProvenanceFingerprintError> {
    match outcome {
        ProvenanceOutcome::Succeeded => {
            builder.field_str(
                "event.outcome",
                "succeeded",
            )?;
        }

        ProvenanceOutcome::Rejected => {
            builder.field_str(
                "event.outcome",
                "rejected",
            )?;
        }

        ProvenanceOutcome::Failed => {
            builder.field_str(
                "event.outcome",
                "failed",
            )?;
        }

        ProvenanceOutcome::Skipped => {
            builder.field_str(
                "event.outcome",
                "skipped",
            )?;
        }

        ProvenanceOutcome::Custom(value) => {
            builder.field_str(
                "event.outcome",
                "custom",
            )?;

            builder.field_str(
                "event.outcome.custom",
                value,
            )?;
        }

        _ => {
            return Err(
                ProvenanceFingerprintError::UnsupportedClassification {
                    category: "outcome",
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// EVENT ENCODING
// ============================================================================

fn encode_event(
    builder: &mut StableHashBuilder,
    event: &ProvenanceEvent,
) -> Result<(), ProvenanceFingerprintError> {
    builder.field_str(
        "event.identifier",
        event.identifier().as_str(),
    )?;

    builder.field_str(
        "event.kind",
        event.kind().as_str(),
    )?;

    builder.field_str(
        "event.actor.component",
        event.actor().component(),
    )?;

    encode_optional_string(
        builder,
        "event.actor.version",
        event.actor().version(),
    )?;

    builder.field_str(
        "event.operation",
        event.operation(),
    )?;

    // Causal parents are supplied by the authoritative ordered set.

    let parents: Vec<_> = event.parents().collect();

    encode_count(
        builder,
        "event.parents.count",
        parents.len(),
    )?;

    for parent in parents {
        builder.field_str(
            "event.parent",
            parent.as_str(),
        )?;
    }

    // Source references.

    let sources: Vec<_> = event.sources().collect();

    encode_count(
        builder,
        "event.sources.count",
        sources.len(),
    )?;

    for source in sources {
        encode_source(builder, source)?;
    }

    // Input artifacts.

    let inputs: Vec<_> = event.inputs().collect();

    encode_count(
        builder,
        "event.inputs.count",
        inputs.len(),
    )?;

    for artifact in inputs {
        encode_artifact(builder, artifact)?;
    }

    // Output artifacts.

    let outputs: Vec<_> = event.outputs().collect();

    encode_count(
        builder,
        "event.outputs.count",
        outputs.len(),
    )?;

    for artifact in outputs {
        encode_artifact(builder, artifact)?;
    }

    // Semantic contract.

    encode_optional_string(
        builder,
        "event.semantic-contract",
        event.semantic_contract()
            .map(|contract| contract.identifier()),
    )?;

    // Target identity.

    encode_optional_string(
        builder,
        "event.target",
        event.target()
            .map(|target| target.identifier()),
    )?;

    // Determinism and outcome.

    encode_determinism(
        builder,
        event.determinism(),
    )?;

    encode_outcome(
        builder,
        event.outcome(),
    )?;

    // Extensible properties.

    let properties: Vec<_> = event.properties().collect();

    encode_count(
        builder,
        "event.properties.count",
        properties.len(),
    )?;

    for (key, value) in properties {
        builder.field_str(
            "event.property.key",
            key,
        )?;

        builder.field_str(
            "event.property.value",
            value,
        )?;
    }

    // Optional description.

    encode_optional_string(
        builder,
        "event.description",
        event.description(),
    )?;

    Ok(())
}

// ============================================================================
// PROVENANCE FINGERPRINTING
// ============================================================================

/// Calculate the canonical fingerprint of an immutable provenance snapshot.
///
/// The snapshot is obtained from the authoritative provenance context.
///
/// The function:
///
/// 1. Obtains deterministic causal event ordering.
/// 2. Includes the compilation identity.
/// 3. Encodes each event using its canonical public representation.
/// 4. Includes all supported lineage relationships.
/// 5. Uses the canonical ZUIR stable hashing implementation.
/// 6. Returns an immutable fingerprint result.
///
/// The provenance ledger revision is deliberately excluded.
///
/// This ensures that equivalent provenance graphs have equivalent
/// fingerprints regardless of the order in which independent events
/// were inserted.
///
/// # Errors
///
/// Returns an explicit error when:
///
/// - The authoritative snapshot cannot be topologically ordered.
/// - A fingerprint operation fails.
/// - A collection count cannot be represented.
/// - A classification requires a fingerprint schema update.
pub fn fingerprint_snapshot(
    snapshot: &ProvenanceSnapshot,
) -> Result<ProvenanceFingerprint, ProvenanceFingerprintError> {
    let events = snapshot.topological_events()?;

    let event_count = u64::try_from(events.len()).map_err(|_| {
        ProvenanceFingerprintError::CountOverflow {
            collection: "events",
        }
    })?;

    let mut builder = StableHashBuilder::new(
        PROVENANCE_FINGERPRINT_DOMAIN,
    )?;

    builder.field_u32(
        "schema.version",
        PROVENANCE_FINGERPRINT_SCHEMA_VERSION,
    )?;

    builder.field_str(
        "compilation.identifier",
        snapshot.compilation().as_str(),
    )?;

    builder.field_u64(
        "provenance.events.count",
        event_count,
    )?;

    for event in events {
        encode_event(
            &mut builder,
            event,
        )?;
    }

    let fingerprint = builder.finish();

    Ok(ProvenanceFingerprint {
        schema_version: PROVENANCE_FINGERPRINT_SCHEMA_VERSION,
        fingerprint,
        event_count,
    })
}

/// Fingerprint the current immutable snapshot of an authoritative
/// provenance context.
///
/// This method does not expose or mutate the underlying ledger.
///
/// A new snapshot should be obtained when the provenance ledger changes.
pub fn fingerprint_context(
    context: &ProvenanceContext,
) -> Result<ProvenanceFingerprint, ProvenanceFingerprintError> {
    let snapshot = context.snapshot()?;

    fingerprint_snapshot(&snapshot)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::provenance_context::{
        CompilationProvenanceId,
        ProvenanceActor,
        ProvenanceEventId,
        ProvenanceEventKind,
    };

    fn empty_context() -> ProvenanceContext {
        let compilation = CompilationProvenanceId::new(
            "test-compilation",
        )
        .expect("valid compilation identity");

        ProvenanceContext::new(compilation)
    }

    fn create_event(
        identifier: &str,
    ) -> ProvenanceEvent {
        let event_id = ProvenanceEventId::new(
            identifier,
        )
        .expect("valid event identity");

        let kind = ProvenanceEventKind::new(
            "compiler.transformation",
        )
        .expect("valid event kind");

        let actor = ProvenanceActor::new(
            "zamani.compiler",
        )
        .expect("valid actor");

        ProvenanceEvent::new(
            event_id,
            kind,
            actor,
            "test-operation",
        )
        .expect("valid provenance event")
    }

    #[test]
    fn empty_provenance_has_stable_fingerprint() {
        let context = empty_context();

        let first = fingerprint_context(&context)
            .expect("fingerprinting succeeds");

        let second = fingerprint_context(&context)
            .expect("fingerprinting succeeds");

        assert_eq!(
            first,
            second,
        );

        assert_eq!(
            first.event_count(),
            0,
        );
    }

    #[test]
    fn provenance_event_changes_fingerprint() {
        let context = empty_context();

        let before = fingerprint_context(&context)
            .expect("initial fingerprint");

        context.record(
            create_event("event-1"),
        )
        .expect("event recording succeeds");

        let after = fingerprint_context(&context)
            .expect("updated fingerprint");

        assert_ne!(
            before.fingerprint(),
            after.fingerprint(),
        );

        assert_eq!(
            after.event_count(),
            1,
        );
    }

    #[test]
    fn identical_snapshots_have_identical_fingerprints() {
        let context = empty_context();

        context.record(
            create_event("event-1"),
        )
        .expect("event recording succeeds");

        let first_snapshot = context.snapshot()
            .expect("snapshot creation succeeds");

        let second_snapshot = context.snapshot()
            .expect("snapshot creation succeeds");

        assert_eq!(
            fingerprint_snapshot(&first_snapshot)
                .expect("first fingerprint"),
            fingerprint_snapshot(&second_snapshot)
                .expect("second fingerprint"),
        );
    }

    #[test]
    fn fingerprint_schema_is_explicitly_versioned() {
        let context = empty_context();

        let result = fingerprint_context(&context)
            .expect("fingerprinting succeeds");

        assert_eq!(
            result.schema_version(),
            PROVENANCE_FINGERPRINT_SCHEMA_VERSION,
        );
    }

    #[test]
    fn ledger_revision_does_not_define_content_identity() {
        let context = empty_context();

        let first = fingerprint_context(&context)
            .expect("first fingerprint");

        let _snapshot = context.snapshot()
            .expect("snapshot");

        let second = fingerprint_context(&context)
            .expect("second fingerprint");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn hexadecimal_representation_is_stable() {
        let context = empty_context();

        let result = fingerprint_context(&context)
            .expect("fingerprinting succeeds");

        assert_eq!(
            result.to_hex().len(),
            64,
        );

        assert_eq!(
            result.to_string(),
            result.to_hex(),
        );
    }
}
