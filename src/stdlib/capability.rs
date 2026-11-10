
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Capability Contracts
//!
//! Path: `src/stdlib/capability.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, extensible descriptions of computational capabilities
//! and representations of capability evidence and availability.
//!
//! This module provides standard-library-facing capability contracts.
//! It does not discover hardware, allocate resources, select execution
//! providers, validate source-language semantics, or decide whether a
//! program can execute on a particular target.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Extensible capability identifiers.
//! - Capability kind identifiers.
//! - Capability descriptions.
//! - Optional capability version identity.
//! - Capability metadata.
//! - Capability availability states.
//! - Capability evidence records.
//! - Immutable capability snapshots.
//! - Deterministic capability lookup and iteration.
//!
//! This module does not own:
//!
//! - Source-language capability semantics.
//! - Semantic capability obligations.
//! - Semantic authorization.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Provider selection.
//! - Execution scheduling.
//! - Target feasibility.
//! - Hardware topology.
//! - Device identity.
//! - Compiler or IR representations.
//! - Runtime execution.
//!
//! ## Architectural authorities
//!
//! The following repository subsystems remain authoritative:
//!
//! - `src/frontend/semantic/`:
//!   source-level capability meaning, semantic obligations and evidence.
//! - `src/compiler/`:
//!   compilation and target-planning integration.
//! - `src/runtime/`:
//!   runtime capability discovery and execution decisions.
//! - HAL and provider implementations:
//!   actual platform and hardware capability evidence.
//! - `src/quantum/`:
//!   quantum computation and quantum IR.
//! - `src/hdl/`:
//!   hardware-description semantics.
//! - `src/ai/`:
//!   AI computation semantics.
//! - `src/distributed/`:
//!   distributed computation semantics.
//!
//! This module must not become a competing authority for any of them.
//!
//! ## POCO-REAF
//!
//! Capability identifiers are extensible strings rather than a closed enum.
//!
//! New computational domains, execution technologies, providers and future
//! machines can introduce new capability identifiers without modifying
//! this module.
//!
//! Capabilities describe what an execution environment can provide.
//! They do not identify a particular device or guarantee that resources
//! are currently available.
//!
//! No universal hardware ceilings are defined here.
//!
//! ## Availability semantics
//!
//! The following states are deliberately distinct:
//!
//! - Available: evidence establishes that the capability is available.
//! - Unavailable: evidence establishes that it is unavailable.
//! - Conditional: availability depends on stated conditions.
//! - Unknown: there is insufficient evidence to establish availability.
//!
//! An absent capability entry is not proof that the capability is
//! unsupported. It is an absence of information.
//!
//! ## Dependencies
//!
//! This module depends only on:
//!
//! - Rust's standard library.
//! - `crate::stdlib::version::Version`.
//!
//! It does not depend on semantic analysis, hardware, runtime, providers,
//! compiler internals, or external crates.
//!
//! ## Integration contract
//!
//! `src/stdlib/mod.rs` must expose this module with:
//!
//!     pub mod capability;
//!
//! Semantic and runtime integrations must use explicit adapters.
//! They must not make the semantic subsystem depend on this module merely
//! to obtain its own canonical capability types.
//!
//! ## Scalability
//!
//! Collections grow according to available memory and configured operational
//! budgets. No arbitrary number of capabilities, providers, domains,
//! devices, or metadata entries is imposed.
//!
//! This does not promise literal infinite storage or computation.
//! It avoids introducing arbitrary language-level ceilings.

use std::collections::BTreeMap;
use std::fmt;

// ============================================================================
// CAPABILITY IDENTIFIERS
// ============================================================================

/// A validated, extensible capability identity.
///
/// Identifiers are case-sensitive and use a portable ASCII namespace
/// convention.
///
/// Examples:
///
/// - `classical.integer`
— `quantum.computation`
/// - `quantum.error_correction`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `ai.tensor_acceleration`
/// - `vendor.extension.special_operation`
///
/// The identifier is not a device name, provider name, or physical resource
/// identifier.
///
/// No fixed list of computational domains is maintained here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Creates a validated capability identifier.
    ///
    /// Accepted characters:
    ///
    /// - ASCII lowercase letters.
    /// - ASCII digits.
    /// - Underscores.
    /// - Hyphens.
    /// - Dots separating namespace segments.
    ///
    /// Each segment must be non-empty and begin with an ASCII lowercase
    /// letter or digit.
    ///
    /// Examples:
    ///
    /// - `quantum.computation`
    /// - `hardware-description.synthesis`
    /// - `vendor_42.special-operation`
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityError`] when the identifier is empty or invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityError> {
        let value = value.into();

        validate_qualified_identifier(&value, "capability identifier")?;

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the namespace portion, if the identifier is qualified.
    ///
    /// For `quantum.computation`, this returns `Some("quantum")`.
    /// For `computation`, this returns `None`.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.0.rsplit_once('.').map(|(namespace, _)| namespace)
    }

    /// Returns the final identifier component.
    #[must_use]
    pub fn name(&self) -> &str {
        self.0
            .rsplit('.')
            .next()
            .unwrap_or(self.0.as_str())
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<&str> for CapabilityId {
    type Error = CapabilityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for CapabilityId {
    type Error = CapabilityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// CAPABILITY KIND
// ============================================================================

/// An extensible classification identifier for a capability.
///
/// This is intentionally a validated string rather than a closed enum.
/// New kinds can be introduced without changing this module.
///
/// Examples:
///
/// - `computation`
/// - `communication`
/// - `storage`
/// - `security`
/// - `acceleration`
/// - `simulation`
/// - `custom`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityKind(String);

impl CapabilityKind {
    /// Creates a capability kind.
    ///
    /// Uses the same identifier rules as [`CapabilityId`].
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityError> {
        let value = value.into();

        validate_qualified_identifier(&value, "capability kind")?;

        Ok(Self(value))
    }

    /// Returns the kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// CAPABILITY DESCRIPTION
// ============================================================================

/// A portable description of a computational capability.
///
/// A capability describes an operation or service that an execution
/// environment may provide.
///
/// It does not assert that the capability is currently available.
///
/// A capability description is independent of:
///
/// - Hardware vendor.
/// - Device model.
/// - Operating system.
/// - Physical resource identity.
/// - Provider implementation.
/// - Runtime allocation.
/// - Execution placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    id: CapabilityId,
    kind: CapabilityKind,
    description: Option<String>,
    version: Option<super::version::Version>,
    attributes: BTreeMap<String, String>,
}

impl Capability {
    /// Creates a capability description.
    ///
    /// # Arguments
    ///
    /// - `id`: stable, extensible capability identity.
    /// - `kind`: extensible capability classification.
    ///
    /// # Examples
    ///
    /// ```
    /// use zamani_compiler::stdlib::capability::{
    ///     Capability, CapabilityId, CapabilityKind,
    /// };
    ///
    /// let capability = Capability::new(
    ///     CapabilityId::new("quantum.computation").unwrap(),
    ///     CapabilityKind::new("computation").unwrap(),
    /// );
    ///
    /// assert_eq!(capability.id().as_str(), "quantum.computation");
    /// ```
    #[must_use]
    pub fn new(id: CapabilityId, kind: CapabilityKind) -> Self {
        Self {
            id,
            kind,
            description: None,
            version: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Sets a human-readable description.
    ///
    /// An empty description is normalized to `None`.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        let description = description.into();

        self.description = if description.trim().is_empty() {
            None
        } else {
            Some(description)
        };

        self
    }

    /// Sets the version of the capability contract.
    ///
    /// This version describes the capability contract, not the compiler,
    /// language, grammar, package, provider, or physical device version.
    #[must_use]
    pub fn with_version(mut self, version: super::version::Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Adds or replaces a namespaced metadata attribute.
    ///
    /// Metadata is descriptive. It must not silently change the meaning
    /// of the capability or act as a resource allocation instruction.
    ///
    /// Attribute keys use the same identifier rules as capability IDs.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute key is invalid.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        let key = key.into();
        let value = value.into();

        validate_qualified_identifier(&key, "capability attribute key")?;

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the capability kind.
    #[must_use]
    pub fn kind(&self) -> &CapabilityKind {
        &self.kind
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the optional capability contract version.
    #[must_use]
    pub fn version(&self) -> Option<&super::version::Version> {
        self.version.as_ref()
    }

    /// Returns all descriptive attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Returns one descriptive attribute.
    #[must_use]
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }
}

// ============================================================================
// CAPABILITY AVAILABILITY
// ============================================================================

/// The evidence-based availability state of a capability.
///
/// These states must not be conflated.
///
/// In particular, `Unknown` does not mean `Unavailable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityStatus {
    /// Evidence establishes that the capability is available.
    Available,

    /// Evidence establishes that the capability is unavailable.
    Unavailable,

    /// The capability is available only under stated conditions.
    Conditional,

    /// Available evidence is insufficient to determine its state.
    Unknown,
}

impl CapabilityStatus {
    /// Returns whether the state establishes unconditional availability.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the state establishes unavailability.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether availability is conditional.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional)
    }

    /// Returns whether availability is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns an optional Boolean availability interpretation.
    ///
    /// - `Some(true)`: established available.
    /// - `Some(false)`: established unavailable.
    /// - `None`: conditional or unknown.
    ///
    /// Conditional capability availability must be evaluated by the
    /// responsible requirement or feasibility subsystem.
    #[must_use]
    pub const fn availability(self) -> Option<bool> {
        match self {
            Self::Available => Some(true),
            Self::Unavailable => Some(false),
            Self::Conditional | Self::Unknown => None,
        }
    }
}

impl fmt::Display for CapabilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Conditional => "conditional",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// CAPABILITY EVIDENCE
// ============================================================================

/// A descriptive record identifying the origin of capability information.
///
/// Evidence records do not independently prove that a capability is
/// available. Their authority and trust must be evaluated by the subsystem
/// responsible for the evidence source.
///
/// Examples of evidence origins include:
///
/// - A runtime capability probe.
/// - A HAL provider report.
/// - A platform manifest.
/// - A compiler target description.
/// - A configured execution environment.
///
/// This module does not perform those probes or authenticate their results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityEvidence {
    source: String,
    description: String,
}

impl CapabilityEvidence {
    /// Creates a capability evidence record.
    ///
    /// `source` identifies the origin of the evidence.
    /// `description` explains what the evidence reports.
    ///
    /// # Errors
    ///
    /// Returns an error if either field is empty after trimming.
    pub fn new(
        source: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        let source = source.into();
        let description = description.into();

        if source.trim().is_empty() {
            return Err(CapabilityError::EmptyEvidenceSource);
        }

        if description.trim().is_empty() {
            return Err(CapabilityError::EmptyEvidenceDescription);
        }

        Ok(Self {
            source,
            description,
        })
    }

    /// Returns the evidence source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the evidence description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// ============================================================================
// CAPABILITY STATE
// ============================================================================

/// A capability description accompanied by its current reported state
/// and available descriptive evidence.
///
/// This type is a data representation. It does not perform discovery,
/// refresh, authorization, or feasibility evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityState {
    capability: Capability,
    status: CapabilityStatus,
    evidence: Vec<CapabilityEvidence>,
}

impl CapabilityState {
    /// Creates a capability state.
    ///
    /// An unknown state is valid and should be used when the source cannot
    /// establish availability.
    #[must_use]
    pub fn new(
        capability: Capability,
        status: CapabilityStatus,
    ) -> Self {
        Self {
            capability,
            status,
            evidence: Vec::new(),
        }
    }

    /// Adds descriptive evidence to this state.
    #[must_use]
    pub fn with_evidence(mut self, evidence: CapabilityEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Returns the capability description.
    #[must_use]
    pub fn capability(&self) -> &Capability {
        &self.capability
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        self.capability.id()
    }

    /// Returns the reported availability state.
    #[must_use]
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Returns the descriptive evidence records.
    #[must_use]
    pub fn evidence(&self) -> &[CapabilityEvidence] {
        &self.evidence
    }

    /// Returns whether unconditional availability has been established.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.status.is_available()
    }
}

// ============================================================================
// CAPABILITY SNAPSHOT
// ============================================================================

/// An immutable-by-interface snapshot of capability descriptions and
/// their reported availability states.
///
/// A snapshot represents information supplied by an external authority.
/// It is not a live hardware inventory and does not reserve resources.
///
/// Entries are kept in a `BTreeMap` to provide deterministic iteration.
///
/// The snapshot does not imply that:
///
/// - A device is exclusively reserved.
/// - A resource is still available at execution time.
/// - A provider is authorized.
/// - A requirement is satisfied.
/// - A computation is feasible.
/// - A capability is compatible with every operation.
///
/// Those decisions belong to the appropriate semantic, planning,
/// provider, or runtime authority.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilitySnapshot {
    entries: BTreeMap<CapabilityId, CapabilityState>,
}

impl CapabilitySnapshot {
    /// Creates an empty capability snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a capability state.
    ///
    /// Duplicate identities are rejected rather than silently replacing
    /// an existing entry.
    ///
    /// Callers that need to refresh a state should construct a new snapshot
    /// or explicitly remove the previous entry before inserting the update.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityError::DuplicateCapability`] when the identity
    /// already exists.
    pub fn insert(
        &mut self,
        state: CapabilityState,
    ) -> Result<(), CapabilityError> {
        let id = state.id().clone();

        if self.entries.contains_key(&id) {
            return Err(CapabilityError::DuplicateCapability(id));
        }

        self.entries.insert(id, state);

        Ok(())
    }

    /// Returns a capability state by identity.
    #[must_use]
    pub fn get(&self, id: &CapabilityId) -> Option<&CapabilityState> {
        self.entries.get(id)
    }

    /// Looks up a capability using its string identity.
    ///
    /// Invalid identifiers and absent entries both return `None`.
    /// Use [`CapabilityId::new`] first when the caller needs to distinguish
    /// malformed identifiers from absent capabilities.
    #[must_use]
    pub fn get_by_name(&self, id: &str) -> Option<&CapabilityState> {
        let id = CapabilityId::new(id.to_owned()).ok()?;

        self.get(&id)
    }

    /// Returns the reported state of a capability.
    ///
    /// An absent entry returns `Unknown`. Absence is not proof of
    /// unavailability.
    #[must_use]
    pub fn status(&self, id: &CapabilityId) -> CapabilityStatus {
        self.get(id)
            .map(CapabilityState::status)
            .unwrap_or(CapabilityStatus::Unknown)
    }

    /// Returns whether the snapshot establishes unconditional availability.
    ///
    /// An absent entry, a conditional entry, or an unknown entry returns
    /// `false`. This method does not assert that those states are
    /// unavailable; it only answers whether availability is established.
    #[must_use]
    pub fn establishes_availability(&self, id: &CapabilityId) -> bool {
        self.status(id).is_available()
    }

    /// Removes an entry and returns its previous state.
    ///
    /// This mutates the snapshot being constructed. It does not remove
    /// a capability from a real provider or execution environment.
    pub fn remove(&mut self, id: &CapabilityId) -> Option<CapabilityState> {
        self.entries.remove(id)
    }

    /// Returns the number of capability entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the snapshot contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns deterministic iteration over capability states.
    ///
    /// Iteration is ordered by capability identity.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&CapabilityId, &CapabilityState)> {
        self.entries.iter()
    }

    /// Returns deterministic iteration over capability identities.
    pub fn ids(&self) -> impl Iterator<Item = &CapabilityId> {
        self.entries.keys()
    }

    /// Returns deterministic iteration over capability states.
    pub fn states(&self) -> impl Iterator<Item = &CapabilityState> {
        self.entries.values()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by capability contract construction and snapshot
/// management.
///
/// These errors describe invalid capability data or duplicate entries.
/// They do not represent hardware, provider, semantic, or runtime failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// The field that contained the invalid identifier.
        field: &'static str,
    },

    /// An identifier contains an invalid character or segment.
    InvalidIdentifier {
        /// The field that contained the invalid identifier.
        field: &'static str,

        /// The invalid value.
        value: String,
    },

    /// Evidence has no source.
    EmptyEvidenceSource,

    /// Evidence has no description.
    EmptyEvidenceDescription,

    /// A capability identity already exists in the snapshot.
    DuplicateCapability(CapabilityId),
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(formatter, "invalid {field}: `{value}`")
            }

            Self::EmptyEvidenceSource => {
                formatter.write_str("capability evidence source must not be empty")
            }

            Self::EmptyEvidenceDescription => {
                formatter.write_str(
                    "capability evidence description must not be empty",
                )
            }

            Self::DuplicateCapability(id) => {
                write!(formatter, "duplicate capability identity `{id}`")
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

/// Validates an extensible, dot-separated identifier.
///
/// The grammar is intentionally independent of any particular computational
/// domain or vendor.
///
/// No arbitrary identifier length limit is imposed here.
fn validate_qualified_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), CapabilityError> {
    if value.is_empty() {
        return Err(CapabilityError::EmptyIdentifier { field });
    }

    let valid = value.split('.').all(|segment| {
        let mut bytes = segment.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
            return false;
        }

        bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-'
        })
    });

    if !valid {
        return Err(CapabilityError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn capability(id: &str, kind: &str) -> Capability {
        Capability::new(
            CapabilityId::new(id).expect("valid capability ID"),
            CapabilityKind::new(kind).expect("valid capability kind"),
        )
    }

    #[test]
    fn accepts_extensible_capability_namespaces() {
        let examples = [
            "classical.integer",
            "quantum.computation",
            "quantum.error_correction",
            "hdl.synthesis",
            "hybrid.execution",
            "distributed.execution",
            "ai.tensor_acceleration",
            "vendor_42.special-operation",
            "future.domain.capability",
        ];

        for example in examples {
            assert_eq!(
                CapabilityId::new(example).unwrap().as_str(),
                example
            );
        }
    }

    #[test]
    fn rejects_empty_capability_identity() {
        assert!(matches!(
            CapabilityId::new(""),
            Err(CapabilityError::EmptyIdentifier { .. })
        ));
    }

    #[test]
    fn rejects_invalid_capability_identity() {
        let invalid = [
            ".quantum",
            "quantum.",
            "quantum..computation",
            "Quantum.computation",
            "quantum computation",
            "quantum/computation",
            "quantum:computation",
            "-quantum",
            "_quantum",
        ];

        for value in invalid {
            assert!(
                CapabilityId::new(value).is_err(),
                "unexpectedly accepted {value}"
            );
        }
    }

    #[test]
    fn capability_identity_exposes_namespace_and_name() {
        let id = CapabilityId::new("quantum.computation").unwrap();

        assert_eq!(id.namespace(), Some("quantum"));
        assert_eq!(id.name(), "computation");
        assert_eq!(id.to_string(), "quantum.computation");
    }

    #[test]
    fn capability_kind_is_extensible() {
        let kind = CapabilityKind::new("future.computational.kind").unwrap();

        assert_eq!(kind.as_str(), "future.computational.kind");
    }

    #[test]
    fn capability_description_preserves_contract_metadata() {
        let version = super::super::version::Version::from_str("1.2.3")
            .expect("valid version");

        let capability = capability("quantum.computation", "computation")
            .with_description("Portable quantum computation")
            .with_version(version)
            .with_attribute("execution.model", "gate-based")
            .unwrap();

        assert_eq!(
            capability.id().as_str(),
            "quantum.computation"
        );

        assert_eq!(
            capability.kind().as_str(),
            "computation"
        );

        assert_eq!(
            capability.description(),
            Some("Portable quantum computation")
        );

        assert_eq!(
            capability.version().unwrap().to_string(),
            "1.2.3"
        );

        assert_eq!(
            capability.attribute("execution.model"),
            Some("gate-based")
        );
    }

    #[test]
    fn rejects_invalid_attribute_names() {
        let result = capability("quantum.computation", "computation")
            .with_attribute("invalid key", "value");

        assert!(matches!(
            result,
            Err(CapabilityError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn empty_description_is_normalized() {
        let capability = capability("classical.integer", "computation")
            .with_description("   ");

        assert_eq!(capability.description(), None);
    }

    #[test]
    fn availability_states_remain_distinct() {
        assert_eq!(
            CapabilityStatus::Available.availability(),
            Some(true)
        );

        assert_eq!(
            CapabilityStatus::Unavailable.availability(),
            Some(false)
        );

        assert_eq!(
            CapabilityStatus::Conditional.availability(),
            None
        );

        assert_eq!(
            CapabilityStatus::Unknown.availability(),
            None
        );
    }

    #[test]
    fn evidence_requires_source_and_description() {
        assert!(CapabilityEvidence::new(
            "",
            "Capability probe succeeded"
        ).is_err());

        assert!(CapabilityEvidence::new(
            "runtime.probe",
            ""
        ).is_err());

        assert!(CapabilityEvidence::new(
            "runtime.probe",
            "Capability probe succeeded"
        ).is_ok());
    }

    #[test]
    fn capability_state_preserves_evidence() {
        let evidence = CapabilityEvidence::new(
            "runtime.probe",
            "The execution environment reports support",
        )
        .unwrap();

        let state = CapabilityState::new(
            capability("quantum.computation", "computation"),
            CapabilityStatus::Available,
        )
        .with_evidence(evidence);

        assert!(state.is_available());
        assert_eq!(state.evidence().len(), 1);
        assert_eq!(
            state.evidence()[0].source(),
            "runtime.probe"
        );
    }

    #[test]
    fn snapshot_inserts_and_retrieves_capabilities() {
        let mut snapshot = CapabilitySnapshot::new();

        let state = CapabilityState::new(
            capability("classical.integer", "computation"),
            CapabilityStatus::Available,
        );

        snapshot.insert(state).unwrap();

        let id = CapabilityId::new("classical.integer").unwrap();

        assert_eq!(snapshot.len(), 1);
        assert!(!snapshot.is_empty());
        assert!(snapshot.get(&id).is_some());
        assert!(snapshot.establishes_availability(&id));
    }

    #[test]
    fn duplicate_capability_identity_is_rejected() {
        let mut snapshot = CapabilitySnapshot::new();

        let first = CapabilityState::new(
            capability("quantum.computation", "computation"),
            CapabilityStatus::Available,
        );

        let second = CapabilityState::new(
            capability("quantum.computation", "computation"),
            CapabilityStatus::Unavailable,
        );

        snapshot.insert(first).unwrap();

        assert!(matches!(
            snapshot.insert(second),
            Err(CapabilityError::DuplicateCapability(_))
        ));

        assert_eq!(snapshot.len(), 1);
        assert_eq!(
            snapshot.status(
                &CapabilityId::new("quantum.computation").unwrap()
            ),
            CapabilityStatus::Available
        );
    }

    #[test]
    fn absent_capability_is_unknown_not_unavailable() {
        let snapshot = CapabilitySnapshot::new();
        let id = CapabilityId::new("future.computation").unwrap();

        assert_eq!(
            snapshot.status(&id),
            CapabilityStatus::Unknown
        );

        assert!(!snapshot.establishes_availability(&id));
        assert!(snapshot.get(&id).is_none());
    }

    #[test]
    fn unknown_is_not_reported_as_unavailable() {
        assert!(!CapabilityStatus::Unknown.is_unavailable());
        assert!(CapabilityStatus::Unknown.is_unknown());
    }

    #[test]
    fn conditional_capability_is_not_unconditionally_available() {
        let mut snapshot = CapabilitySnapshot::new();

        let state = CapabilityState::new(
            capability("distributed.execution", "computation"),
            CapabilityStatus::Conditional,
        );

        snapshot.insert(state).unwrap();

        let id = CapabilityId::new("distributed.execution").unwrap();

        assert_eq!(
            snapshot.status(&id),
            CapabilityStatus::Conditional
        );

        assert!(!snapshot.establishes_availability(&id));
    }

    #[test]
    fn snapshot_iteration_is_deterministic() {
        let mut snapshot = CapabilitySnapshot::new();

        for id in [
            "quantum.computation",
            "classical.integer",
            "distributed.execution",
        ] {
            snapshot
                .insert(CapabilityState::new(
                    capability(id, "computation"),
                    CapabilityStatus::Available,
                ))
                .unwrap();
        }

        let ids: Vec<&str> = snapshot
            .ids()
            .map(CapabilityId::as_str)
            .collect();

        assert_eq!(
            ids,
            vec![
                "classical.integer",
                "distributed.execution",
                "quantum.computation",
            ]
        );
    }

    #[test]
    fn snapshot_removes_only_the_requested_entry() {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(CapabilityState::new(
                capability("classical.integer", "computation"),
                CapabilityStatus::Available,
            ))
            .unwrap();

        let id = CapabilityId::new("classical.integer").unwrap();

        assert!(snapshot.remove(&id).is_some());
        assert!(snapshot.is_empty());
        assert!(snapshot.remove(&id).is_none());
    }

    #[test]
    fn lookup_by_name_handles_invalid_and_missing_names() {
        let snapshot = CapabilitySnapshot::new();

        assert!(snapshot.get_by_name("invalid name").is_none());
        assert!(snapshot.get_by_name("quantum.computation").is_none());
    }

    #[test]
    fn capability_ids_have_value_semantics() {
        let first = CapabilityId::new("quantum.computation").unwrap();
        let second = CapabilityId::new("quantum.computation").unwrap();

        assert_eq!(first, second);
        assert_eq!(
            std::collections::BTreeSet::from([first, second]).len(),
            1
        );
    }

    #[test]
    fn capability_metadata_is_deterministically_ordered() {
        let capability = capability("ai.tensor_acceleration", "acceleration")
            .with_attribute("zeta.property", "second")
            .unwrap()
            .with_attribute("alpha.property", "first")
            .unwrap();

        let keys: Vec<&str> = capability
            .attributes()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["alpha.property", "zeta.property"]);
    }
}
