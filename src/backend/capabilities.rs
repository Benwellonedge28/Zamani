
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Extensible target-capability contracts for the Zamani backend.
//!
//! Path: `src/backend/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Describe capabilities that a backend target declares it can provide,
//! together with the status and evidence supporting that declaration.
//! Capability identifiers are extensible strings rather than a closed enum,
//! so new computational domains can be introduced without changing a central
//! universal list.
//!
//! # Architectural authority
//!
//! This module follows the boundaries established by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/compiler_types.rs`
//! - `src/compiler/diagnostics.rs`
//! - `src/backend/errors.rs`
//! - `src/backend/diagnostics.rs`
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Validated, extensible capability identifiers.
//! - Capability descriptions and declaration status.
//! - Capability evidence and evidence policy.
//! - Deterministic capability collections and profiles.
//! - Requirement matching and structured compatibility results.
//!
//! # Non-goals
//!
//! This module does not own:
//!
//! - Source-language semantics or semantic validation.
//! - Workload requirement declarations in the semantic model.
//! - Target selection or target discovery.
//! - Physical resource allocation, scheduling, routing, or placement.
//! - Quantum gate sets, QEC, calibrated pulses, or hardware topology.
//! - HDL synthesis, timing closure, or bitstream generation.
//! - Toolchain invocation, filesystem writes, logging, or diagnostics output.
//!
//! # Integration contract
//!
//! - `errors.rs` remains the backend error authority.
//! - `diagnostics.rs` remains the backend diagnostic authority.
//! - `target.rs` should embed or reference a `CapabilityProfile`.
//! - `selection.rs` should evaluate requirements against a target profile.
//! - `requirements.rs` should own compilation/workload requirements,
//!   preferences, constraints, and hints. It may adapt them to the
//!   capability-level `CapabilityRequirement` defined here.
//! - Target-specific modules may declare capabilities but must not redefine
//!   the matching policy or capability status model.
//! - `mod.rs` must declare `pub mod capabilities;` when it becomes the
//!   canonical module root.
//!
//! # Scalability and portability
//!
//! No universal maximum number of capabilities, devices, qubits, CPUs,
//! nodes, tensor dimensions, threads, or memory bytes is imposed here.
//! A capability is not a promise of unlimited physical resources. Concrete
//! resource feasibility is decided by the relevant target, runtime, and
//! resource-planning layers.
//!
//! # Determinism
//!
//! Ordered maps and sets are used so that iteration and serialized map order
//! are deterministic for the same data. Evidence strings should not include
//! nondeterministic timestamps, machine-local paths, secrets, or transient
//! runtime state if they are used in fingerprints or reproducible manifests.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::errors::{BackendError, BackendErrorKind, BackendResult};

/// A validated, extensible capability identifier.
///
/// Identifiers are case-sensitive and use ASCII letters and digits plus
/// `.`, `_`, `-`, and `:`. The first character must be an ASCII letter or
/// digit. Names should be qualified by domain where practical, for example:
///
/// - `cpu.integer`
/// - `cpu.vector`
/// - `gpu.compute`
/// - `quantum.measurement`
/// - `quantum.mid_circuit_measurement`
/// - `hdl.synthesis`
/// - `tensor.compute`
/// - `distributed.execution`
///
/// This syntax intentionally does not reserve a fixed list of domains.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

/// Validation failure for a capability identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityIdError {
    /// The identifier is empty.
    Empty,

    /// The first character is not an ASCII letter or digit.
    InvalidFirstCharacter,

    /// The identifier contains a character outside the permitted syntax.
    InvalidCharacter,
}

impl fmt::Display for CapabilityIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "capability identifier must not be empty",
            Self::InvalidFirstCharacter => {
                "capability identifier must begin with an ASCII letter or digit"
            }
            Self::InvalidCharacter => {
                "capability identifier contains an unsupported character"
            }
        })
    }
}

impl std::error::Error for CapabilityIdError {}

impl CapabilityId {
    /// Construct and validate a capability identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityIdError> {
        let value = value.into();
        let mut chars = value.chars();

        let Some(first) = chars.next() else {
            return Err(CapabilityIdError::Empty);
        };

        if !first.is_ascii_alphanumeric() {
            return Err(CapabilityIdError::InvalidFirstCharacter);
        }

        if !chars.all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, '.' | '_' | '-' | ':')
        }) {
            return Err(CapabilityIdError::InvalidCharacter);
        }

        Ok(Self(value))
    }

    /// Return the validated identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its owned string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Construct an identifier controlled by this implementation.
    ///
    /// The literal is still validated. An invalid built-in identifier is an
    /// implementation defect and therefore causes an explicit panic rather
    /// than allowing invalid data to enter the capability registry.
    pub fn from_static(value: &'static str) -> Self {
        match Self::new(value) {
            Ok(identifier) => identifier,
            Err(error) => panic!("invalid built-in capability identifier: {error}"),
        }
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for CapabilityId {
    type Error = CapabilityIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CapabilityId> for String {
    fn from(value: CapabilityId) -> Self {
        value.0
    }
}

impl Serialize for CapabilityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Lifecycle and availability state of a declared capability.
///
/// The state describes the backend's claim about the capability. It does
/// not prove that a particular workload fits within available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CapabilityStatus {
    /// The capability is declared supported and is not experimental.
    Supported,

    /// The capability is available, but its contract or implementation is
    /// not yet considered stable.
    Experimental,

    /// The capability is retained for compatibility but should not be used
    /// for new portable programs unless policy explicitly permits it.
    Deprecated,

    /// The capability is known but unavailable for this profile.
    ///
    /// This status is useful for explaining a known limitation. A capability
    /// omitted from a profile is instead unknown or undeclared.
    Unavailable,
}

impl CapabilityStatus {
    /// Whether this status may satisfy a requirement under the given policy.
    pub const fn is_eligible(self, policy: CapabilityPolicy) -> bool {
        match self {
            Self::Supported => true,
            Self::Experimental => policy.allow_experimental,
            Self::Deprecated => policy.allow_deprecated,
            Self::Unavailable => false,
        }
    }

    /// Stable textual representation suitable for logs and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Unavailable => "unavailable",
        }
    }
}

impl fmt::Display for CapabilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// How the backend obtained evidence for a capability declaration.
///
/// This is provenance metadata, not a guarantee of semantic correctness.
/// The actual verification process and its trust model belong to the
/// backend/toolchain integration that supplies the evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CapabilityEvidenceKind {
    /// A capability is declared by configuration or a target description.
    Declared,

    /// A suitable validation procedure or conformance test confirmed it.
    Verified,

    /// The capability was observed through a runtime or toolchain probe.
    Observed,

    /// The capability is inferred from other declared target metadata.
    Inferred,
}

impl CapabilityEvidenceKind {
    /// Whether this evidence kind represents direct validation.
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::Verified | Self::Observed)
    }
}

/// Evidence supporting a capability declaration.
///
/// Evidence is descriptive and deterministic by design. It deliberately
/// contains no mandatory timestamp, machine-local path, or provider-specific
/// object. Consumers may keep richer private provenance elsewhere and expose
/// a stable summary here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CapabilityEvidence {
    kind: CapabilityEvidenceKind,
    source: String,
    detail: Option<String>,
}

impl CapabilityEvidence {
    /// Construct capability evidence.
    pub fn new(
        kind: CapabilityEvidenceKind,
        source: impl Into<String>,
        detail: Option<String>,
    ) -> Self {
        Self {
            kind,
            source: source.into(),
            detail,
        }
    }

    /// Return the evidence category.
    pub const fn kind(&self) -> CapabilityEvidenceKind {
        self.kind
    }

    /// Return the evidence source, such as a conformance suite identifier.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return the optional explanatory detail.
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

/// Descriptive contract for one capability.
///
/// Metadata is intentionally extensible. `attributes` can record namespaced
/// declarative facts such as a supported format version or a feature profile.
/// These attributes are not automatically interpreted as resource guarantees
/// or as workload requirements; matching semantics must be defined by the
/// responsible domain or target adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    id: CapabilityId,
    status: CapabilityStatus,
    description: String,
    evidence: BTreeSet<CapabilityEvidence>,
    attributes: BTreeMap<String, String>,
}

impl CapabilityDescriptor {
    /// Construct a descriptor with no evidence or additional attributes.
    pub fn new(
        id: CapabilityId,
        status: CapabilityStatus,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            status,
            description: description.into(),
            evidence: BTreeSet::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Construct a descriptor from an identifier string.
    pub fn from_name(
        id: impl Into<String>,
        status: CapabilityStatus,
        description: impl Into<String>,
    ) -> BackendResult<Self> {
        let id = CapabilityId::new(id).map_err(|error| {
            BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("invalid capability identifier: {error}"),
            )
        })?;

        Ok(Self::new(id, status, description))
    }

    /// Return this capability's identifier.
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Return the declared status.
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Return the human-readable description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the evidence set in deterministic order.
    pub fn evidence(&self) -> &BTreeSet<CapabilityEvidence> {
        &self.evidence
    }

    /// Return extension attributes in deterministic key order.
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Replace the declared status.
    pub fn with_status(mut self, status: CapabilityStatus) -> Self {
        self.status = status;
        self
    }

    /// Add evidence supporting this declaration.
    pub fn with_evidence(mut self, evidence: CapabilityEvidence) -> Self {
        self.evidence.insert(evidence);
        self
    }

    /// Add or replace an extension attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Whether the descriptor contains direct verification or observation
    /// evidence.
    pub fn has_direct_evidence(&self) -> bool {
        self.evidence.iter().any(|evidence| evidence.kind().is_direct())
    }

    /// Validate descriptor invariants after construction or deserialization.
    ///
    /// An empty description is permitted because identifiers and status are
    /// sufficient for matching. Attribute keys must be non-empty so that
    /// serialized metadata cannot contain ambiguous empty keys.
    pub fn validate(&self) -> BackendResult<()> {
        for key in self.attributes.keys() {
            if key.trim().is_empty() {
                return Err(BackendError::invalid_input(format!(
                    "capability `{}` has an empty attribute key",
                    self.id
                )));
            }
        }

        Ok(())
    }
}

/// Deterministic collection of capability descriptors.
///
/// The collection is keyed by `CapabilityId`; duplicate IDs are rejected
/// rather than silently overwriting a declaration. This is important because
/// competing declarations could otherwise make target compatibility depend
/// on registration order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySet {
    entries: BTreeMap<CapabilityId, CapabilityDescriptor>,
}

impl CapabilitySet {
    /// Create an empty capability set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of descriptors in the set.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over descriptors in deterministic identifier order.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityDescriptor> {
        self.entries.values()
    }

    /// Look up a descriptor by validated identifier.
    pub fn get(&self, id: &CapabilityId) -> Option<&CapabilityDescriptor> {
        self.entries.get(id)
    }

    /// Look up a descriptor by identifier text.
    ///
    /// Invalid identifier text is treated as no match. Call `CapabilityId::new`
    /// directly when callers need to distinguish malformed input from absence.
    pub fn get_by_name(&self, id: &str) -> Option<&CapabilityDescriptor> {
        let id = CapabilityId::new(id.to_owned()).ok()?;
        self.get(&id)
    }

    /// Whether a capability is declared in this set, regardless of status.
    pub fn contains(&self, id: &CapabilityId) -> bool {
        self.entries.contains_key(id)
    }

    /// Insert a new descriptor.
    ///
    /// Fails on duplicate identifiers. To intentionally update a descriptor,
    /// call `replace` so the mutation is explicit at the call site.
    pub fn insert(&mut self, descriptor: CapabilityDescriptor) -> BackendResult<()> {
        descriptor.validate()?;

        if self.entries.contains_key(descriptor.id()) {
            return Err(BackendError::invalid_input(format!(
                "duplicate capability declaration `{}`",
                descriptor.id()
            )));
        }

        self.entries.insert(descriptor.id().clone(), descriptor);
        Ok(())
    }

    /// Replace an existing descriptor, returning the previous value.
    ///
    /// This is an explicit update API. It also permits adding a descriptor
    /// when the identifier was not previously present.
    pub fn replace(
        &mut self,
        descriptor: CapabilityDescriptor,
    ) -> BackendResult<Option<CapabilityDescriptor>> {
        descriptor.validate()?;
        Ok(self
            .entries
            .insert(descriptor.id().clone(), descriptor))
    }

    /// Remove and return a descriptor.
    pub fn remove(&mut self, id: &CapabilityId) -> Option<CapabilityDescriptor> {
        self.entries.remove(id)
    }

    /// Validate every descriptor in the set.
    ///
    /// Call this after deserializing data from an untrusted source, before
    /// accepting the set as a target's capability declaration.
    pub fn validate(&self) -> BackendResult<()> {
        for (key, descriptor) in &self.entries {
            descriptor.validate()?;

            if key != descriptor.id() {
                return Err(BackendError::invalid_input(format!(
                    "capability set key `{key}` does not match descriptor ID `{}`",
                    descriptor.id()
                )));
            }
        }

        Ok(())
    }

    /// Check a collection of capability requirements under a policy.
    ///
    /// The result preserves input requirement order, including repeated
    /// requirements. It does not allocate hardware or infer that the target
    /// has sufficient capacity for a workload.
    pub fn check(
        &self,
        requirements: &[CapabilityRequirement],
        policy: CapabilityPolicy,
    ) -> CapabilityCheckResult {
        let results = requirements
            .iter()
            .map(|requirement| {
                let descriptor = self.get(&requirement.id);

                let outcome = match descriptor {
                    None => CapabilityOutcome::Missing,
                    Some(descriptor)
                        if descriptor.status() == CapabilityStatus::Unavailable =>
                    {
                        CapabilityOutcome::Unavailable
                    }
                    Some(descriptor)
                        if !descriptor.status().is_eligible(policy) =>
                    {
                        CapabilityOutcome::DisallowedStatus(descriptor.status())
                    }
                    Some(descriptor)
                        if policy.require_direct_evidence
                            && !descriptor.has_direct_evidence() =>
                    {
                        CapabilityOutcome::InsufficientEvidence
                    }
                    Some(_) => CapabilityOutcome::Satisfied,
                };

                CapabilityRequirementResult {
                    id: requirement.id.clone(),
                    outcome,
                }
            })
            .collect();

        CapabilityCheckResult { results }
    }
}

/// Policy controlling which declarations may satisfy a capability requirement.
///
/// Defaults are conservative: experimental and deprecated capabilities are
/// not accepted, and direct evidence is not required unless a caller opts in.
/// A production release or security-sensitive deployment may choose stricter
/// evidence policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityPolicy {
    /// Accept capabilities marked experimental.
    pub allow_experimental: bool,

    /// Accept capabilities marked deprecated.
    pub allow_deprecated: bool,

    /// Require direct verification or observation evidence.
    pub require_direct_evidence: bool,
}

impl Default for CapabilityPolicy {
    fn default() -> Self {
        Self {
            allow_experimental: false,
            allow_deprecated: false,
            require_direct_evidence: false,
        }
    }
}

/// One capability that must be available for a compilation or target choice.
///
/// This is a backend-level matching primitive. The semantic model and
/// `requirements.rs` remain responsible for deciding which requirements a
/// source program actually has and whether each one is mandatory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    id: CapabilityId,
}

impl CapabilityRequirement {
    /// Require a capability by validated identifier.
    pub fn new(id: CapabilityId) -> Self {
        Self { id }
    }

    /// Require a capability by identifier text.
    pub fn from_name(id: impl Into<String>) -> BackendResult<Self> {
        let id = CapabilityId::new(id).map_err(|error| {
            BackendError::new(
                BackendErrorKind::InvalidInput,
                format!("invalid capability requirement: {error}"),
            )
        })?;

        Ok(Self::new(id))
    }

    /// Return the required capability identifier.
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }
}

/// Outcome of matching one requirement against a capability profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CapabilityOutcome {
    /// The target's declaration satisfies the requirement and active policy.
    Satisfied,

    /// The capability is not declared by this profile.
    Missing,

    /// The capability is explicitly declared unavailable.
    Unavailable,

    /// The capability is declared but its lifecycle status is disallowed.
    DisallowedStatus(CapabilityStatus),

    /// Policy requires direct evidence, but the declaration has none.
    InsufficientEvidence,
}

impl CapabilityOutcome {
    /// Whether this outcome satisfies the requirement.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

/// Result for one capability requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirementResult {
    id: CapabilityId,
    outcome: CapabilityOutcome,
}

impl CapabilityRequirementResult {
    /// Return the requirement identifier.
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Return the matching outcome.
    pub const fn outcome(&self) -> CapabilityOutcome {
        self.outcome
    }

    /// Whether this requirement is satisfied.
    pub const fn is_satisfied(&self) -> bool {
        self.outcome.is_satisfied()
    }
}

/// Aggregate result of a capability check.
///
/// A check result is informational and deterministic. It does not itself
/// raise a compiler diagnostic or choose a fallback target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityCheckResult {
    results: Vec<CapabilityRequirementResult>,
}

impl CapabilityCheckResult {
    /// Return per-requirement results in the order requested.
    pub fn results(&self) -> &[CapabilityRequirementResult] {
        &self.results
    }

    /// Whether every requirement was satisfied.
    pub fn is_compatible(&self) -> bool {
        self.results.iter().all(CapabilityRequirementResult::is_satisfied)
    }

    /// Return requirements that were not satisfied.
    pub fn failures(&self) -> impl Iterator<Item = &CapabilityRequirementResult> {
        self.results
            .iter()
            .filter(|result| !result.is_satisfied())
    }

    /// Number of requirements checked.
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Whether no requirements were supplied.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}

/// Named capability profile for a backend target.
///
/// A profile contains only capability declarations and stable metadata. It
/// must not be treated as a complete target description: architecture,
/// operating system, ABI, resource availability, toolchain state, and
/// artifact formats belong to the target and toolchain contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityProfile {
    name: String,
    capabilities: CapabilitySet,
    attributes: BTreeMap<String, String>,
}

impl CapabilityProfile {
    /// Create an empty profile with a descriptive name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capabilities: CapabilitySet::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Return the profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the capability set.
    pub fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }

    /// Mutably access the capability set to add declarations before the
    /// profile is published or registered.
    pub fn capabilities_mut(&mut self) -> &mut CapabilitySet {
        &mut self.capabilities
    }

    /// Return profile-level metadata in deterministic order.
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Add or replace profile metadata.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Validate the profile before registration or use.
    pub fn validate(&self) -> BackendResult<()> {
        if self.name.trim().is_empty() {
            return Err(BackendError::invalid_input(
                "capability profile name must not be empty",
            ));
        }

        if self.attributes.keys().any(|key| key.trim().is_empty()) {
            return Err(BackendError::invalid_input(
                "capability profile contains an empty attribute key",
            ));
        }

        self.capabilities.validate()
    }

    /// Check requirements against this profile.
    pub fn check(
        &self,
        requirements: &[CapabilityRequirement],
        policy: CapabilityPolicy,
    ) -> CapabilityCheckResult {
        self.capabilities.check(requirements, policy)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CapabilityDescriptor, CapabilityEvidence, CapabilityEvidenceKind, CapabilityId,
        CapabilityOutcome, CapabilityPolicy, CapabilityProfile, CapabilityRequirement,
        CapabilitySet, CapabilityStatus,
    };

    #[test]
    fn capability_ids_are_validated_and_extensible() {
        assert!(CapabilityId::new("quantum.mid_circuit_measurement").is_ok());
        assert!(CapabilityId::new("future-domain.feature-v2").is_ok());
        assert!(CapabilityId::new("").is_err());
        assert!(CapabilityId::new("-invalid").is_err());
        assert!(CapabilityId::new("contains spaces").is_err());
    }

    #[test]
    fn capability_ids_reject_invalid_serialized_values() {
        let invalid = serde_json::from_str::<CapabilityId>("\"bad identifier\"");
        assert!(invalid.is_err());

        let valid = serde_json::from_str::<CapabilityId>("\"future.domain-feature\"");
        assert_eq!(
            valid.expect("valid identifier").as_str(),
            "future.domain-feature"
        );
    }

    #[test]
    fn duplicate_capability_declarations_are_rejected() {
        let mut capabilities = CapabilitySet::new();
        let first = CapabilityDescriptor::from_name(
            "cpu.integer",
            CapabilityStatus::Supported,
            "Integer arithmetic",
        )
        .expect("valid descriptor");
        let duplicate = first.clone();

        capabilities.insert(first).expect("first insertion succeeds");
        assert!(capabilities.insert(duplicate).is_err());
        assert_eq!(capabilities.len(), 1);
    }

    #[test]
    fn matching_distinguishes_missing_unavailable_and_experimental() {
        let mut capabilities = CapabilitySet::new();

        capabilities
            .insert(
                CapabilityDescriptor::from_name(
                    "cpu.integer",
                    CapabilityStatus::Supported,
                    "Integer arithmetic",
                )
                .expect("valid descriptor"),
            )
            .expect("insert supported");

        capabilities
            .insert(
                CapabilityDescriptor::from_name(
                    "quantum.measurement",
                    CapabilityStatus::Experimental,
                    "Quantum measurement",
                )
                .expect("valid descriptor"),
            )
            .expect("insert experimental");

        capabilities
            .insert(
                CapabilityDescriptor::from_name(
                    "hdl.synthesis",
                    CapabilityStatus::Unavailable,
                    "HDL synthesis is unavailable for this profile",
                )
                .expect("valid descriptor"),
            )
            .expect("insert unavailable");

        let requirements = [
            CapabilityRequirement::from_name("cpu.integer").expect("valid requirement"),
            CapabilityRequirement::from_name("quantum.measurement")
                .expect("valid requirement"),
            CapabilityRequirement::from_name("hdl.synthesis").expect("valid requirement"),
            CapabilityRequirement::from_name("gpu.compute").expect("valid requirement"),
        ];

        let conservative = capabilities.check(&requirements, CapabilityPolicy::default());
        assert!(!conservative.is_compatible());

        let outcomes: Vec<_> = conservative
            .results()
            .iter()
            .map(|result| result.outcome())
            .collect();

        assert_eq!(outcomes[0], CapabilityOutcome::Satisfied);
        assert_eq!(
            outcomes[1],
            CapabilityOutcome::DisallowedStatus(CapabilityStatus::Experimental)
        );
        assert_eq!(outcomes[2], CapabilityOutcome::Unavailable);
        assert_eq!(outcomes[3], CapabilityOutcome::Missing);

        let permissive = capabilities.check(
            &requirements,
            CapabilityPolicy {
                allow_experimental: true,
                allow_deprecated: false,
                require_direct_evidence: false,
            },
        );
        assert_eq!(
            permissive.results()[1].outcome(),
            CapabilityOutcome::Satisfied
        );
    }

    #[test]
    fn evidence_policy_requires_direct_evidence_when_requested() {
        let mut capabilities = CapabilitySet::new();

        capabilities
            .insert(
                CapabilityDescriptor::from_name(
                    "gpu.compute",
                    CapabilityStatus::Supported,
                    "GPU computation",
                )
                .expect("valid descriptor"),
            )
            .expect("insert descriptor");

        let requirement =
            CapabilityRequirement::from_name("gpu.compute").expect("valid requirement");

        let policy = CapabilityPolicy {
            allow_experimental: false,
            allow_deprecated: false,
            require_direct_evidence: true,
        };

        let unchecked = capabilities.check(&[requirement.clone()], policy);
        assert_eq!(
            unchecked.results()[0].outcome(),
            CapabilityOutcome::InsufficientEvidence
        );

        let descriptor = CapabilityDescriptor::from_name(
            "gpu.compute",
            CapabilityStatus::Supported,
            "GPU computation",
        )
        .expect("valid descriptor")
        .with_evidence(CapabilityEvidence::new(
            CapabilityEvidenceKind::Verified,
            "backend-capability-conformance",
            Some("conformance suite passed".to_owned()),
        ));

        capabilities
            .replace(descriptor)
            .expect("replacement succeeds");

        let checked = capabilities.check(&[requirement], policy);
        assert!(checked.is_compatible());
    }

    #[test]
    fn profile_serialization_is_deterministic() {
        let mut profile = CapabilityProfile::new("portable-target");
        profile
            .capabilities_mut()
            .insert(
                CapabilityDescriptor::from_name(
                    "tensor.compute",
                    CapabilityStatus::Supported,
                    "Tensor computation",
                )
                .expect("valid descriptor"),
            )
            .expect("insert tensor capability");

        profile
            .capabilities_mut()
            .insert(
                CapabilityDescriptor::from_name(
                    "cpu.integer",
                    CapabilityStatus::Supported,
                    "Integer arithmetic",
                )
                .expect("valid descriptor"),
            )
            .expect("insert CPU capability");

        profile.validate().expect("profile validates");

        let first = serde_json::to_string(&profile).expect("serialize profile");
        let second = serde_json::to_string(&profile).expect("serialize profile again");
        assert_eq!(first, second);

        let decoded: CapabilityProfile =
            serde_json::from_str(&first).expect("deserialize profile");
        decoded.validate().expect("decoded profile validates");
        assert_eq!(decoded, profile);
    }

    #[test]
    fn empty_requirement_set_is_compatible() {
        let capabilities = CapabilitySet::new();
        let result = capabilities.check(&[], CapabilityPolicy::default());

        assert!(result.is_compatible());
        assert!(result.is_empty());
    }
}
