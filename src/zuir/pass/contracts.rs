
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Pass Contracts
//!
//! Path: `src/zuir/pass/contracts.rs`
//!
//! PURPOSE
//!
//! Define the canonical, implementation-independent contracts shared by
//! pass registration, pass execution, pipeline reporting, and pass tooling.
//!
//! AUTHORITY
//!
//! This module conforms to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/grammar-authority.md`
//!
//! OWNS
//!
//! - Pass identity and semantic version.
//! - Pass descriptor and analysis declarations.
//! - Pass error categories and structured pass errors.
//! - The result of a completed pass.
//! - Validation of pass IDs, analysis IDs, and descriptors.
//!
//! DOES NOT OWN
//!
//! - Pass execution, cancellation, verification orchestration, or budgets.
//! - Analysis result storage or analysis invalidation algorithms.
//! - IR entities, source syntax, dialect registration, or target discovery.
//! - Physical resource allocation, scheduling, routing, or runtime behavior.
//! - Whether a pass is semantically correct; declarations are contracts,
//!   not proofs.
//!
//! DEPENDENCY RULE
//!
//! This module depends only on the Rust standard library. The manager,
//! registry, and future pass tooling depend on these canonical definitions.
//! This module must not import those higher-level subsystems.
//!
//! SCALABILITY
//!
//! No hardware population or pass-count limit is defined here. Collections
//! are dynamically sized. Actual allocation and finite identifier space
//! limitations remain explicit implementation realities.
//!
//! COMPATIBILITY
//!
//! Pass IDs and version triples are suitable for pipeline manifests, but
//! this module does not itself define a persistent serialization format.
//! Published IDs and the meanings of existing error kinds must not be
//! silently changed.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use core::fmt;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------
// Pass identity
// -----------------------------------------------------------------------------

/// Stable, namespaced identity for a compiler pass.
///
/// Examples:
/// - `core.canonicalize`
/// - `classical.simplify`
/// - `quantum.lower_logical_ops`
/// - `hdl.elaborate`
/// - `tensor.fuse_operations`
///
/// An ID identifies a pass contract, not a concrete device or registration
/// position. IDs are case-sensitive and use ASCII namespace segments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(String);

impl PassId {
    /// Construct and validate a pass ID.
    ///
    /// Each dot-separated segment must be non-empty, start with an ASCII
    /// letter or digit, and contain only ASCII letters, digits, `_`, or `-`.
    /// At least one dot is required to enforce namespacing.
    pub fn new(name: impl Into<String>) -> Result<Self, PassError> {
        let name = name.into();

        if !is_valid_qualified_id(&name) {
            return Err(PassError::new(
                PassErrorKind::InvalidPassId,
                format!(
                    "invalid pass ID {name:?}; expected a qualified ASCII ID \
                     such as `core.canonicalize`"
                ),
            ));
        }

        Ok(Self(name))
    }

    /// Borrow the canonical textual ID.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// -----------------------------------------------------------------------------
// Pass version
// -----------------------------------------------------------------------------

/// Semantic version of a pass implementation and its contract.
///
/// Persisted pipelines should identify passes by both `PassId` and
/// `PassVersion`; registry indices are not persistent identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl PassVersion {
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

// -----------------------------------------------------------------------------
// Pass descriptor
// -----------------------------------------------------------------------------

/// Declarative contract required to register and schedule a pass.
///
/// Analysis names are extensible strings, not a closed enum. Namespaced
/// identifiers are recommended, for example `core.cfg` or
/// `quantum.logical_liveness`.
///
/// These fields express the pass author's declarations. They do not prove
/// semantic preservation or replace IR verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassDescriptor {
    /// Stable, qualified identity.
    pub id: PassId,

    /// Version of this pass contract and implementation.
    pub version: PassVersion,

    /// Human-readable explanation; must not be blank.
    pub description: String,

    /// Analyses that must be available before execution.
    pub required_analyses: BTreeSet<String>,

    /// Analyses that the pass declares invalid after execution.
    pub invalidated_analyses: BTreeSet<String>,

    /// Whether the pass guarantees that every analysis remains valid.
    ///
    /// When true, `invalidated_analyses` must be empty.
    pub preserves_all_analyses: bool,

    /// Whether the manager must verify IR before this pass executes.
    pub requires_verified_input: bool,

    /// Whether the manager should verify the resulting IR when verification
    /// policy requests output verification.
    pub verifies_output: bool,

    /// Whether the pass claims reproducible behavior for equivalent inputs
    /// under the same relevant configuration and environment.
    pub deterministic: bool,
}

impl PassDescriptor {
    /// Construct a descriptor with conservative verification defaults.
    #[must_use]
    pub fn new(
        id: PassId,
        version: PassVersion,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            version,
            description: description.into(),
            required_analyses: BTreeSet::new(),
            invalidated_analyses: BTreeSet::new(),
            preserves_all_analyses: false,
            requires_verified_input: true,
            verifies_output: true,
            deterministic: true,
        }
    }

    /// Validate metadata independently of registration.
    ///
    /// The registry must call this before mutating its state. Domain-specific
    /// preconditions and semantic correctness remain the responsibility of
    /// the pass, verifier, and manager.
    pub fn validate(&self) -> Result<(), PassError> {
        if self.description.trim().is_empty() {
            return Err(PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                self.id.clone(),
                "pass description must not be empty",
            ));
        }

        if self.preserves_all_analyses && !self.invalidated_analyses.is_empty() {
            return Err(PassError::for_pass(
                PassErrorKind::InvalidConfiguration,
                self.id.clone(),
                "a pass cannot preserve all analyses and also declare \
                 analyses invalidated",
            ));
        }

        for name in self
            .required_analyses
            .iter()
            .chain(self.invalidated_analyses.iter())
        {
            if !is_valid_analysis_id(name) {
                return Err(PassError::for_pass(
                    PassErrorKind::InvalidConfiguration,
                    self.id.clone(),
                    format!("invalid analysis identifier: {name:?}"),
                ));
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Error contract
// -----------------------------------------------------------------------------

/// Stable machine-readable category for a pass subsystem failure.
///
/// Existing variants must retain their meanings. New variants may be added
/// when callers can handle unknown or future cases appropriately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassErrorKind {
    InvalidPassId,
    DuplicatePass,
    UnknownPass,
    InvalidConfiguration,
    Cancelled,
    ExecutionLimitExceeded,
    TimeBudgetExceeded,
    MissingAnalysis,
    PassFailed,
    VerificationFailed,
    NondeterministicPassRejected,
}

/// Structured pass subsystem error.
///
/// `kind` is the machine-readable category. `message` is explanatory text
/// and must not be treated as a stable API. `pass` is present when the error
/// can be attributed to a particular pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassError {
    pub kind: PassErrorKind,
    pub pass: Option<PassId>,
    pub message: String,
}

impl PassError {
    /// Create an error not attributed to a particular pass.
    #[must_use]
    pub fn new(kind: PassErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            pass: None,
            message: message.into(),
        }
    }

    /// Create an error attributed to a specific pass.
    #[must_use]
    pub fn for_pass(
        kind: PassErrorKind,
        pass: PassId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            message: message.into(),
        }
    }

    /// Attach a pass ID only when the error does not already have one.
    #[must_use]
    pub fn with_pass_if_missing(mut self, pass: &PassId) -> Self {
        if self.pass.is_none() {
            self.pass = Some(pass.clone());
        }
        self
    }
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.pass {
            Some(pass) => write!(f, "{:?} in pass {pass}: {}", self.kind, self.message),
            None => write!(f, "{:?}: {}", self.kind, self.message),
        }
    }
}

impl std::error::Error for PassError {}

// -----------------------------------------------------------------------------
// Pass outcome
// -----------------------------------------------------------------------------

/// Result returned by a successfully completed pass.
///
/// `preserved_analyses` is the pass's explicit preservation declaration for
/// this particular execution. It is not a proof that the analyses remain
/// correct. The manager must apply the agreed invalidation policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOutcome {
    /// Whether the pass changed the IR.
    pub changed: bool,

    /// Analyses guaranteed by the pass to remain valid.
    pub preserved_analyses: BTreeSet<String>,
}

impl PassOutcome {
    /// Report that the IR was not changed.
    #[must_use]
    pub fn unchanged() -> Self {
        Self {
            changed: false,
            preserved_analyses: BTreeSet::new(),
        }
    }

    /// Report that the IR was changed, with no explicit analysis-preservation
    /// declarations.
    #[must_use]
    pub fn changed() -> Self {
        Self {
            changed: true,
            preserved_analyses: BTreeSet::new(),
        }
    }

    /// Report whether the IR changed and which analyses remain valid.
    #[must_use]
    pub fn preserving(
        changed: bool,
        analyses: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            changed,
            preserved_analyses: analyses.into_iter().collect(),
        }
    }
}

// -----------------------------------------------------------------------------
// Identifier validation
// -----------------------------------------------------------------------------

fn is_valid_qualified_id(value: &str) -> bool {
    if value.is_empty() || value.trim() != value || !value.contains('.') {
        return false;
    }

    value.split('.').all(is_valid_id_segment)
}

fn is_valid_id_segment(segment: &str) -> bool {
    let mut chars = segment.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    first.is_ascii_alphanumeric()
        && chars.all(|ch| {
            ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
        })
}

fn is_valid_analysis_id(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    // Analysis identifiers may be namespaced, but they are not forced into a
    // closed vocabulary. Permit common extension separators while rejecting
    // whitespace and control characters.
    value
        .chars()
        .all(|ch| !ch.is_whitespace() && !ch.is_control())
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> PassId {
        PassId::new(value).expect("test ID must be valid")
    }

    #[test]
    fn accepts_namespaced_domain_ids() {
        for value in [
            "core.canonicalize",
            "classical.simplify",
            "quantum.lower_logical_ops",
            "hdl.elaborate",
            "tensor.fuse-operations",
            "future_domain.pass_v2",
        ] {
            assert_eq!(PassId::new(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn rejects_malformed_pass_ids() {
        for value in [
            "",
            "canonicalize",
            ".core",
            "core.",
            "core..canonicalize",
            " core.canonicalize",
            "core.canonicalize ",
            "core.bad name",
            "core/operation",
            "core.é",
        ] {
            assert!(
                PassId::new(value).is_err(),
                "unexpectedly accepted {value:?}"
            );
        }
    }

    #[test]
    fn pass_id_display_and_order_are_stable() {
        let a = id("core.alpha");
        let b = id("core.beta");

        assert_eq!(a.to_string(), "core.alpha");
        assert!(a < b);
        assert_eq!(a, id("core.alpha"));
    }

    #[test]
    fn descriptor_defaults_are_explicit_and_conservative() {
        let descriptor = PassDescriptor::new(
            id("core.verify"),
            PassVersion::new(1, 0, 0),
            "Verify IR invariants",
        );

        assert!(descriptor.requires_verified_input);
        assert!(descriptor.verifies_output);
        assert!(descriptor.deterministic);
        assert!(descriptor.required_analyses.is_empty());
        assert!(descriptor.invalidated_analyses.is_empty());
        assert!(!descriptor.preserves_all_analyses);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn descriptor_rejects_blank_description() {
        let descriptor = PassDescriptor::new(
            id("core.example"),
            PassVersion::new(1, 0, 0),
            " \t ",
        );

        assert_eq!(
            descriptor.validate().unwrap_err().kind,
            PassErrorKind::InvalidConfiguration
        );
    }

    #[test]
    fn descriptor_rejects_contradictory_analysis_metadata() {
        let mut descriptor = PassDescriptor::new(
            id("core.example"),
            PassVersion::new(1, 0, 0),
            "Example pass",
        );

        descriptor.preserves_all_analyses = true;
        descriptor
            .invalidated_analyses
            .insert("core.cfg".to_owned());

        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn descriptor_accepts_extensible_analysis_names() {
        let mut descriptor = PassDescriptor::new(
            id("quantum.optimize"),
            PassVersion::new(2, 1, 0),
            "Optimize quantum-domain operations",
        );

        descriptor
            .required_analyses
            .insert("quantum.logical_liveness".to_owned());
        descriptor
            .invalidated_analyses
            .insert("custom-domain.analysis-v3".to_owned());

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn descriptor_rejects_empty_or_whitespace_analysis_names() {
        for name in ["", " ", "core cfg", "\tanalysis"] {
            let mut descriptor = PassDescriptor::new(
                id("core.example"),
                PassVersion::new(1, 0, 0),
                "Example pass",
            );

            descriptor.required_analyses.insert(name.to_owned());
            assert!(descriptor.validate().is_err(), "accepted {name:?}");
        }
    }

    #[test]
    fn pass_errors_keep_structured_context() {
        let pass_id = id("core.canonicalize");
        let error = PassError::for_pass(
            PassErrorKind::PassFailed,
            pass_id.clone(),
            "failed to canonicalize",
        );

        assert_eq!(error.kind, PassErrorKind::PassFailed);
        assert_eq!(error.pass.as_ref(), Some(&pass_id));
        assert!(error.to_string().contains("core.canonicalize"));

        let attached = PassError::new(PassErrorKind::PassFailed, "failure")
            .with_pass_if_missing(&pass_id);
        assert_eq!(attached.pass, Some(pass_id));
    }

    #[test]
    fn pass_outcomes_represent_change_and_preservation() {
        assert!(!PassOutcome::unchanged().changed);
        assert!(PassOutcome::changed().changed);

        let outcome = PassOutcome::preserving(
            true,
            ["core.cfg".to_owned(), "core.dominance".to_owned()],
        );

        assert!(outcome.changed);
        assert_eq!(outcome.preserved_analyses.len(), 2);
        assert!(outcome.preserved_analyses.contains("core.cfg"));
    }

    #[test]
    fn versions_have_ordered_semantic_components() {
        let v1 = PassVersion::new(1, 0, 0);
        let v2 = PassVersion::new(1, 1, 0);
        let v3 = PassVersion::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
    }
}
