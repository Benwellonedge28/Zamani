#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani Classical Computing — Provenance and Compilation Lineage
//!
//! Path: `src/classical/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the classical subsystem with:
//!
//! - the canonical ZUIR source-provenance API;
//! - deterministic compilation-activity records;
//! - explicit artifact input/output relationships;
//! - open-world tool, phase, artifact, and metadata identifiers;
//! - validation of provenance references;
//! - structured errors for invalid or incomplete lineage.
//!
//! Provenance records where information came from and which activities
//! produced or consumed it. Provenance does not execute compilation,
//! determine program semantics, authorize an operation, or prove that an
//! artifact is correct or trustworthy.
//!
//! # Authority
//!
//! Normative contracts:
//! - `grammar/spec/provenance.md`
//! - `grammar/spec/source-spans.md`
//! - `grammar/spec/source-map.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Grammar contributors:
//! - `grammar/compile/provenance.g4`
//! - `grammar/data/provenance.g4`
//! - `grammar/security/provenance.g4`
//! - `grammar/expressions/provenance.g4`
//! - `grammar/ai/provenance.g4`
//!
//! Canonical source-origin implementation:
//! - `crate::zuir::source_map`
//!
//! Canonical identity implementation:
//! - `crate::zuir::ids`
//!
//! # Ownership
//!
//! This module owns the classical-facing compilation-lineage record types
//! and their in-memory validation rules.
//!
//! It does not define replacement source IDs, source spans, IR IDs, AST
//! nodes, semantic types, security policies, cryptographic hashes, or IR.
//!
//! # Provenance domains
//!
//! Source provenance and compilation lineage are related but distinct:
//!
//! - `crate::zuir::source_map` associates canonical IR entities with source
//!   spans and origin kinds.
//! - `CompilationProvenance` records artifacts and compilation activities,
//!   including input/output relationships and tool context.
//! - Data lineage, security evidence, reproducibility, and trust remain
//!   governed by their own semantic contracts and may contribute metadata
//!   to compilation records through explicit adapters.
//!
//! # Determinism
//!
//! Ordered maps and sets provide deterministic iteration and deduplication.
//! This module does not insert wall-clock timestamps, generate random IDs,
//! hash source content, or infer tool identity. Callers supply those values
//! explicitly when they are known and meaningful.
//!
//! Deterministic record ordering is not proof of a deterministic build.
//!
//! # Scalability
//!
//! There are no fixed limits on artifacts, activities, inputs, outputs,
//! tools, phases, or metadata keys. Collections grow subject to available
//! memory and address space. Allocation failure behavior remains subject to
//! Rust's standard allocation model and the host process.
//!
//! Identifiers are opaque strings supplied by the owning subsystem. This
//! module does not claim global uniqueness or persistent stability for them;
//! callers must establish the required identity scope.
//!
//! # Integration
//!
//! Frontend and compiler adapters register artifacts and record activities.
//! Lowering and optimization adapters record their inputs and outputs.
//! Classical, quantum, HDL, and other domains use the same activity model.
//! Canonical source spans and IR-entity origins remain in `zuir::source_map`.
//!
//! No conversion from a legacy `FileId` to a ZUIR `SourceId` is performed
//! implicitly. Such conversion requires an explicit source identity mapping.
//!
//! # Freeze criteria
//!
//! Freeze this file when it is registered in `classical/mod.rs`, its
//! dependencies match the repository, tests pass, and downstream adapters
//! use these contracts without defining competing provenance authorities.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Canonical source-provenance types.
///
/// Re-exporting the existing types prevents the classical subsystem from
/// creating a competing source-map or source-origin model.
pub use crate::zuir::source_map::{
    ColumnUnit,
    OriginKind,
    SourceDocument,
    SourceDocumentKind,
    SourceEntity,
    SourceMap,
    SourceMapError,
    SourceMapResult,
    SourceOrigin,
    SourcePosition,
    SourceSpan,
};

/// Result type for compilation-provenance operations.
pub type ProvenanceResult<T> = Result<T, ProvenanceError>;

/// Errors produced by compilation-lineage construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceError {
    /// A required identifier or descriptive field is empty.
    EmptyField {
        field: &'static str,
    },

    /// An artifact ID is already registered.
    DuplicateArtifact {
        artifact_id: String,
    },

    /// An activity ID is already registered.
    DuplicateActivity {
        activity_id: String,
    },

    /// An activity refers to an artifact that has not been registered.
    UnknownArtifact {
        activity_id: String,
        artifact_id: String,
    },

    /// An activity identifier is not registered.
    UnknownActivity {
        activity_id: String,
    },

    /// A metadata key is empty.
    EmptyMetadataKey,

    /// The record collection failed an integrity check.
    InvalidRecord {
        activity_id: String,
        reason: String,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "provenance field `{field}` must not be empty")
            }
            Self::DuplicateArtifact { artifact_id } => {
                write!(f, "provenance artifact `{artifact_id}` is already registered")
            }
            Self::DuplicateActivity { activity_id } => {
                write!(f, "provenance activity `{activity_id}` is already registered")
            }
            Self::UnknownArtifact {
                activity_id,
                artifact_id,
            } => write!(
                f,
                "activity `{activity_id}` references unregistered artifact \
                 `{artifact_id}`"
            ),
            Self::UnknownActivity { activity_id } => {
                write!(f, "provenance activity `{activity_id}` is not registered")
            }
            Self::EmptyMetadataKey => {
                f.write_str("provenance metadata keys must not be empty")
            }
            Self::InvalidRecord {
                activity_id,
                reason,
            } => write!(
                f,
                "provenance activity `{activity_id}` is invalid: {reason}"
            ),
        }
    }
}

impl std::error::Error for ProvenanceError {}

/// An artifact participating in compilation lineage.
///
/// `id` is an opaque identifier supplied by the owning subsystem. It may
/// identify a source snapshot, AST export, semantic artifact, IR module,
/// generated file, binary, report, or other compiler-visible artifact.
///
/// `kind` is open-world descriptive metadata. It must not be used as a
/// substitute for a verified type, capability, or security classification.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvenanceArtifact {
    id: String,
    kind: String,
    version: Option<String>,
    content_fingerprint: Option<String>,
    attributes: BTreeMap<String, String>,
}

impl ProvenanceArtifact {
    /// Create an artifact descriptor.
    ///
    /// The caller supplies identity and kind. This constructor does not
    /// calculate a fingerprint or assert that an identifier is globally
    /// unique.
    pub fn new(
        id: impl Into<String>,
        kind: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let id = id.into();
        let kind = kind.into();

        if id.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "artifact.id",
            });
        }

        if kind.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "artifact.kind",
            });
        }

        Ok(Self {
            id,
            kind,
            version: None,
            content_fingerprint: None,
            attributes: BTreeMap::new(),
        })
    }

    /// Set the artifact's externally defined version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set an externally computed content fingerprint.
    ///
    /// This method stores an opaque value. It does not calculate or verify
    /// cryptographic hashes, signatures, or artifact integrity.
    #[must_use]
    pub fn with_content_fingerprint(
        mut self,
        fingerprint: impl Into<String>,
    ) -> Self {
        self.content_fingerprint = Some(fingerprint.into());
        self
    }

    /// Add descriptive metadata.
    ///
    /// Empty keys are rejected. Existing keys are replaced explicitly by
    /// this builder operation; use `CompilationProvenance::register_artifact`
    /// to control duplicate artifact identity.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(ProvenanceError::EmptyMetadataKey);
        }

        self.attributes.insert(key, value.into());
        Ok(self)
    }

    /// Return the artifact identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the open-world artifact kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the optional version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Return the optional externally computed fingerprint.
    #[must_use]
    pub fn content_fingerprint(&self) -> Option<&str> {
        self.content_fingerprint.as_deref()
    }

    /// Return descriptive metadata in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

/// A compilation activity and its artifact relationships.
///
/// An activity represents a recorded event, such as parsing, semantic
/// analysis, lowering, optimization, code generation, validation, packaging,
/// or another current or future compilation phase.
///
/// Input and output IDs are sets: duplicate references within the same
/// relationship have no additional lineage meaning. Their iteration order
/// is deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceActivity {
    id: String,
    kind: String,
    tool: Option<String>,
    tool_version: Option<String>,
    inputs: BTreeSet<String>,
    outputs: BTreeSet<String>,
    attributes: BTreeMap<String, String>,
}

impl ProvenanceActivity {
    /// Create an activity descriptor.
    ///
    /// IDs and activity kinds are supplied by the caller. This type does
    /// not generate timestamps or claim that the activity actually ran;
    /// it records what the caller reports.
    pub fn new(
        id: impl Into<String>,
        kind: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let id = id.into();
        let kind = kind.into();

        if id.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "activity.id",
            });
        }

        if kind.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "activity.kind",
            });
        }

        Ok(Self {
            id,
            kind,
            tool: None,
            tool_version: None,
            inputs: BTreeSet::new(),
            outputs: BTreeSet::new(),
            attributes: BTreeMap::new(),
        })
    }

    /// Attach an explicit tool identity.
    #[must_use]
    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    /// Attach an explicit tool version.
    #[must_use]
    pub fn with_tool_version(mut self, version: impl Into<String>) -> Self {
        self.tool_version = Some(version.into());
        self
    }

    /// Declare an input artifact.
    #[must_use]
    pub fn with_input(mut self, artifact_id: impl Into<String>) -> Self {
        self.inputs.insert(artifact_id.into());
        self
    }

    /// Declare an output artifact.
    #[must_use]
    pub fn with_output(mut self, artifact_id: impl Into<String>) -> Self {
        self.outputs.insert(artifact_id.into());
        self
    }

    /// Add descriptive metadata.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(ProvenanceError::EmptyMetadataKey);
        }

        self.attributes.insert(key, value.into());
        Ok(self)
    }

    /// Return the activity identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the activity kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the optional tool identity.
    #[must_use]
    pub fn tool(&self) -> Option<&str> {
        self.tool.as_deref()
    }

    /// Return the optional tool version.
    #[must_use]
    pub fn tool_version(&self) -> Option<&str> {
        self.tool_version.as_deref()
    }

    /// Return input artifact IDs in deterministic order.
    #[must_use]
    pub fn inputs(&self) -> &BTreeSet<String> {
        &self.inputs
    }

    /// Return output artifact IDs in deterministic order.
    #[must_use]
    pub fn outputs(&self) -> &BTreeSet<String> {
        &self.outputs
    }

    /// Return activity metadata in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

/// Deterministic in-memory compilation-provenance registry.
///
/// The registry records artifacts and activities without executing them.
/// Registration is explicit: referenced artifacts must exist before an
/// activity is accepted. Duplicate IDs are errors rather than silent
/// replacements.
///
/// The registry is intentionally not a persistent database, cryptographic
/// verifier, or globally synchronized event collector.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompilationProvenance {
    artifacts: BTreeMap<String, ProvenanceArtifact>,
    activities: BTreeMap<String, ProvenanceActivity>,
}

impl CompilationProvenance {
    /// Create an empty provenance registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an artifact without replacing an existing artifact.
    pub fn register_artifact(
        &mut self,
        artifact: ProvenanceArtifact,
    ) -> ProvenanceResult<()> {
        let id = artifact.id.clone();

        if self.artifacts.contains_key(&id) {
            return Err(ProvenanceError::DuplicateArtifact { artifact_id: id });
        }

        self.artifacts.insert(id, artifact);
        Ok(())
    }

    /// Record an activity after validating all its artifact references.
    ///
    /// The operation is atomic with respect to this registry: validation
    /// completes before the activity is inserted.
    pub fn record_activity(
        &mut self,
        activity: ProvenanceActivity,
    ) -> ProvenanceResult<()> {
        if self.activities.contains_key(&activity.id) {
            return Err(ProvenanceError::DuplicateActivity {
                activity_id: activity.id,
            });
        }

        for artifact_id in activity.inputs.iter().chain(activity.outputs.iter()) {
            if !self.artifacts.contains_key(artifact_id) {
                return Err(ProvenanceError::UnknownArtifact {
                    activity_id: activity.id.clone(),
                    artifact_id: artifact_id.clone(),
                });
            }
        }

        self.activities.insert(activity.id.clone(), activity);
        Ok(())
    }

    /// Look up an artifact by its opaque ID.
    #[must_use]
    pub fn artifact(&self, id: &str) -> Option<&ProvenanceArtifact> {
        self.artifacts.get(id)
    }

    /// Look up an activity by its opaque ID.
    #[must_use]
    pub fn activity(&self, id: &str) -> Option<&ProvenanceActivity> {
        self.activities.get(id)
    }

    /// Iterate over artifacts in deterministic ID order.
    pub fn artifacts(
        &self,
    ) -> impl Iterator<Item = (&str, &ProvenanceArtifact)> {
        self.artifacts
            .iter()
            .map(|(id, artifact)| (id.as_str(), artifact))
    }

    /// Iterate over activities in deterministic ID order.
    pub fn activities(
        &self,
    ) -> impl Iterator<Item = (&str, &ProvenanceActivity)> {
        self.activities
            .iter()
            .map(|(id, activity)| (id.as_str(), activity))
    }

    /// Return the number of registered artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Return the number of recorded activities.
    #[must_use]
    pub fn activity_count(&self) -> usize {
        self.activities.len()
    }

    /// Return recorded activities that produce an artifact.
    pub fn producers_of(
        &self,
        artifact_id: &str,
    ) -> impl Iterator<Item = &ProvenanceActivity> {
        self.activities
            .values()
            .filter(move |activity| activity.outputs.contains(artifact_id))
    }

    /// Return recorded activities that consume an artifact.
    pub fn consumers_of(
        &self,
        artifact_id: &str,
    ) -> impl Iterator<Item = &ProvenanceActivity> {
        self.activities
            .values()
            .filter(move |activity| activity.inputs.contains(artifact_id))
    }

    /// Validate the complete registry.
    ///
    /// This is useful after importing provenance through a deserializer or
    /// another external adapter. Normal insertion already validates these
    /// relationships.
    pub fn validate(&self) -> ProvenanceResult<()> {
        for (id, artifact) in &self.artifacts {
            if id.trim().is_empty() || artifact.kind.trim().is_empty() {
                return Err(ProvenanceError::InvalidRecord {
                    activity_id: String::new(),
                    reason: format!("artifact `{id}` has an empty required field"),
                });
            }
        }

        for (id, activity) in &self.activities {
            if id.trim().is_empty() || activity.kind.trim().is_empty() {
                return Err(ProvenanceError::InvalidRecord {
                    activity_id: id.clone(),
                    reason: "activity ID or kind is empty".to_owned(),
                });
            }

            for artifact_id in activity.inputs.iter().chain(activity.outputs.iter()) {
                if !self.artifacts.contains_key(artifact_id) {
                    return Err(ProvenanceError::UnknownArtifact {
                        activity_id: id.clone(),
                        artifact_id: artifact_id.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Return true when no artifacts or activities have been recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty() && self.activities.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact(id: &str, kind: &str) -> ProvenanceArtifact {
        ProvenanceArtifact::new(id, kind).expect("valid artifact")
    }

    fn activity(id: &str, kind: &str) -> ProvenanceActivity {
        ProvenanceActivity::new(id, kind).expect("valid activity")
    }

    #[test]
    fn registry_starts_empty() {
        let provenance = CompilationProvenance::new();

        assert!(provenance.is_empty());
        assert_eq!(provenance.artifact_count(), 0);
        assert_eq!(provenance.activity_count(), 0);
    }

    #[test]
    fn rejects_empty_artifact_identity_and_kind() {
        assert!(matches!(
            ProvenanceArtifact::new("", "source"),
            Err(ProvenanceError::EmptyField {
                field: "artifact.id"
            })
        ));

        assert!(matches!(
            ProvenanceArtifact::new("source:main", " "),
            Err(ProvenanceError::EmptyField {
                field: "artifact.kind"
            })
        ));
    }

    #[test]
    fn rejects_duplicate_artifact_registration() {
        let mut provenance = CompilationProvenance::new();

        provenance
            .register_artifact(artifact("source:main", "source"))
            .expect("first registration");

        assert_eq!(
            provenance.register_artifact(artifact("source:main", "source")),
            Err(ProvenanceError::DuplicateArtifact {
                artifact_id: "source:main".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_activity_with_unknown_input_artifact() {
        let mut provenance = CompilationProvenance::new();
        let activity = activity("parse:1", "parse")
            .with_input("missing:source");

        assert_eq!(
            provenance.record_activity(activity),
            Err(ProvenanceError::UnknownArtifact {
                activity_id: "parse:1".to_owned(),
                artifact_id: "missing:source".to_owned(),
            })
        );

        assert_eq!(provenance.activity_count(), 0);
    }

    #[test]
    fn records_and_queries_a_compilation_pipeline() {
        let mut provenance = CompilationProvenance::new();

        provenance
            .register_artifact(artifact("source:main", "source"))
            .expect("register source");

        provenance
            .register_artifact(artifact("ast:main", "frontend.ast"))
            .expect("register AST");

        provenance
            .register_artifact(artifact("ir:main", "zuir.module"))
            .expect("register IR");

        provenance
            .record_activity(
                activity("parse:1", "frontend.parse")
                    .with_input("source:main")
                    .with_output("ast:main")
                    .with_tool("zamani-compiler"),
            )
            .expect("record parse");

        provenance
            .record_activity(
                activity("lower:1", "ir.lower")
                    .with_input("ast:main")
                    .with_output("ir:main")
                    .with_tool("zamani-compiler"),
            )
            .expect("record lowering");

        assert_eq!(provenance.producers_of("ast:main").count(), 1);
        assert_eq!(provenance.consumers_of("ast:main").count(), 1);
        assert_eq!(provenance.producers_of("ir:main").count(), 1);
        assert!(provenance.validate().is_ok());
    }

    #[test]
    fn duplicate_activity_ids_are_rejected() {
        let mut provenance = CompilationProvenance::new();

        provenance
            .register_artifact(artifact("source:main", "source"))
            .expect("register source");

        provenance
            .record_activity(
                activity("parse:1", "frontend.parse")
                    .with_input("source:main"),
            )
            .expect("first activity");

        assert_eq!(
            provenance.record_activity(activity("parse:1", "frontend.parse")),
            Err(ProvenanceError::DuplicateActivity {
                activity_id: "parse:1".to_owned(),
            })
        );
    }

    #[test]
    fn duplicate_input_references_are_deduplicated() {
        let activity = activity("parse:1", "frontend.parse")
            .with_input("source:main")
            .with_input("source:main");

        assert_eq!(activity.inputs().len(), 1);
    }

    #[test]
    fn metadata_is_ordered_and_empty_keys_are_rejected() {
        let activity = activity("build:1", "compile")
            .with_attribute("z-policy", "portable")
            .expect("valid metadata")
            .with_attribute("a-profile", "release")
            .expect("valid metadata");

        let keys: Vec<_> = activity.attributes().keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["a-profile", "z-policy"]);

        assert!(matches!(
            activity("build:2", "compile").with_attribute(" ", "value"),
            Err(ProvenanceError::EmptyMetadataKey)
        ));
    }

    #[test]
    fn repeated_registration_does_not_silently_replace_artifact_metadata() {
        let mut provenance = CompilationProvenance::new();

        provenance
            .register_artifact(
                artifact("ir:main", "zuir.module")
                    .with_version("1")
            )
            .expect("register initial artifact");

        let error = provenance
            .register_artifact(
                artifact("ir:main", "different.kind")
                    .with_version("2")
            )
            .expect_err("duplicate identity must be rejected");

        assert!(matches!(
            error,
            ProvenanceError::DuplicateArtifact { .. }
        ));

        assert_eq!(
            provenance.artifact("ir:main").and_then(ProvenanceArtifact::version),
            Some("1")
        );
    }

    #[test]
    fn caller_supplied_ids_do_not_imply_global_uniqueness() {
        let mut first = CompilationProvenance::new();
        let mut second = CompilationProvenance::new();

        first
            .register_artifact(artifact("artifact:1", "source"))
            .expect("first registry");

        second
            .register_artifact(artifact("artifact:1", "source"))
            .expect("second registry");

        assert_eq!(first.artifact_count(), 1);
        assert_eq!(second.artifact_count(), 1);
    }
}