
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Target Model
//!
//! Path: `src/hdl/targets/model.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//!
//! ## Status
//!
//! Initial target-model implementation. Integration is complete only after
//! the module is registered and the repository's actual workspace checks pass.
//!
//! ## Purpose
//!
//! Describe an abstract hardware implementation environment without changing
//! the source-level meaning of an HDL design.
//!
//! A target may describe an FPGA, ASIC, simulator, processor-based platform,
//! accelerator, quantum-classical platform, distributed fabric, or a future
//! implementation environment. Those categories are open-ended identifiers,
//! not an exhaustive Rust enumeration.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/hdl/capabilities.rs`
//! - `src/hdl/resources.rs`
//! - `src/hdl/requirements.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//! - `src/frontend/semantic/resources/model.rs`
//!
//! The grammar owns syntax. The frontend semantic model owns source-level
//! meaning. The shared capability subsystem owns capability identity and
//! evidence semantics. The shared resource subsystem owns resource quantities,
//! units, and requirements. This module composes those contracts.
//!
//! ## Owns
//!
//! - Stable, validated target-description identities.
//! - Human-readable target names and open-ended target-kind identifiers.
//! - Optional provider identity and provider-version metadata.
//! - Target-description completeness status.
//! - Open-ended descriptive metadata and labels.
//! - Aggregation of canonical capability evidence.
//! - Optional canonical HDL resource information.
//! - Local validation of target-description records.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or source-level semantic validity.
//! - A second capability identity, evidence, or compatibility model.
//! - A second resource quantity, unit, or requirement model.
//! - Target discovery, authentication, authorization, or trust verification.
//! - A guarantee that advertised capabilities are physically available.
//! - Feasibility evaluation or resource allocation.
//! - Physical device handles, addresses, placement, or routing.
//! - Clock-tree synthesis, timing closure, or vendor-specific primitives.
//! - Synthesis, scheduling, backend code generation, or deployment.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Target-description semantics
//!
//! A target description is information supplied to a later decision process.
//! It is not, by itself, proof that a design is realizable.
//!
//! In particular:
//!
//! - An absent resource profile means that resource information was not
//!   supplied. It does not mean that resources are unlimited.
//! - An incomplete description must not be treated as a complete inventory.
//! - A capability evidence record retains the canonical semantics assigned by
//!   the shared capability subsystem.
//! - Conflicting evidence must remain observable to the canonical evaluator.
//! - Descriptive metadata is not a substitute for typed capability evidence.
//! - A target kind is descriptive and must not be used as proof of support.
//! - Target identity is not a physical device address or runtime handle.
//!
//! This module validates its own representation. It does not decide whether
//! an HDL program is feasible for a target.
//!
//! ## Scalability and POCO-REAF
//!
//! There is no fixed maximum number of targets, labels, metadata entries,
//! capability evidence records, resource descriptions, or hardware units.
//!
//! Collections grow subject to available memory and explicit operational
//! policies owned by the relevant caller. String lengths and collection sizes
//! remain bounded by the actual machine, Rust's representable values, and
//! available resources. Literal infinity is not promised.
//!
//! No universal assumptions are made about CPU count, memory capacity, FPGA
//! logic elements, ASIC cells, device count, register width, clock rate,
//! network size, or accelerator count.
//!
//! Operational exhaustion belongs to the consuming compilation or analysis
//! session. It must not be converted into a successful feasibility result.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical frontend semantic models
//!             |
//!             v
//!   src/hdl/capabilities.rs
//!   src/hdl/resources.rs
//!             |
//!             v
//!   src/hdl/targets/model.rs
//!             |
//!             v
//!   targets/feasibility.rs  (future consumer)
//!             |
//!             v
//!   realization interfaces (future consumer)
//!             |
//!             v
//!   src/hdl/hdlir/ (deferred)
//! ```
//!
//! The diagram describes intended ownership and integration, not a claim that
//! all downstream modules already exist or are operational.
//!
//! ## Freeze criteria
//!
//! - Canonical capability and resource types are reused.
//! - Target identity validation is deterministic.
//! - Unknown and incomplete information remain explicit.
//! - Metadata is not silently interpreted as capability evidence.
//! - No physical allocation or feasibility decision is performed here.
//! - Unit tests and actual workspace checks pass.
//!
//! Changes to these public contracts require coordinated updates to their
//! consumers and corresponding compatibility tests.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::hdl::capabilities::HdlCapabilityEvidence;
use crate::hdl::resources::HdlResourceProfile;

/// Schema version of this in-memory target-description contract.
///
/// This is not the Zamani language version, Rust version, provider version,
/// device revision, or HDLIR schema version.
pub const HDL_TARGET_MODEL_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of this module.
pub const HDL_TARGET_MODEL_MODULE_NAME: &str = "zamani.hdl.targets.model";

/// Validate an open-ended target-description identifier.
///
/// Identifiers are preserved exactly as supplied. This function deliberately
/// does not normalize case, rewrite punctuation, infer a vendor, or interpret
/// a target identifier as a physical device address.
///
/// Empty identifiers and control characters are rejected. Namespaces and
/// identifier syntax beyond these representation-level checks are not
/// restricted to a closed list.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), HdlTargetModelError> {
    if value.trim().is_empty() {
        return Err(HdlTargetModelError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlTargetModelError::ControlCharacter { field });
    }

    Ok(())
}

/// Stable identity for an abstract target description.
///
/// A `TargetId` identifies a description, not necessarily one physical device.
/// Multiple descriptions may refer to different versions, configurations, or
/// capability scopes of the same provider.
///
/// IDs are open-ended and namespace-friendly. The model does not reserve
/// vendor names or prescribe a global registry.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetId(String);

impl TargetId {
    /// Construct a target identity after representation-level validation.
    ///
    /// The original string is preserved exactly.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlTargetModelError> {
        let value = value.into();
        validate_identifier(&value, "target identifier")?;
        Ok(Self(value))
    }

    /// Return the exact identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Completeness of the supplied target description.
///
/// Completeness concerns the description, not whether the target can satisfy
/// any particular program. Even a complete description may establish that a
/// design is infeasible.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TargetDescriptionStatus {
    /// The description is declared complete for the provider's stated scope.
    ///
    /// This is a declaration about the supplied description, not independent
    /// proof that the provider is trustworthy or that hardware is available.
    Complete,

    /// Some target properties are known, but relevant information is absent.
    #[default]
    Partial,

    /// The description does not yet establish a useful inventory.
    Unknown,
}

/// A provider reference for an abstract target description.
///
/// Provider information identifies the source of the description. It does not
/// authenticate the provider or prove that its claims are true.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TargetProvider {
    id: String,
    version: Option<String>,
}

impl TargetProvider {
    /// Construct provider metadata.
    ///
    /// `id` is a stable, open-ended provider identifier. `version` may
    /// identify a provider release, inventory schema, or provider-defined
    /// revision. Its interpretation belongs to the provider contract.
    pub fn new(
        id: impl Into<String>,
        version: Option<String>,
    ) -> Result<Self, HdlTargetModelError> {
        let id = id.into();
        validate_identifier(&id, "provider identifier")?;

        if let Some(version) = version.as_deref() {
            validate_identifier(version, "provider version")?;
        }

        Ok(Self { id, version })
    }

    /// Return the provider identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the optional provider version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

/// An abstract hardware or execution target description.
///
/// The target kind is an open-ended string, such as `fpga`, `asic`,
/// `simulation`, `heterogeneous`, or a future provider-defined category.
/// These are examples, not a closed enumeration.
///
/// `metadata` is descriptive data only. It must not be used as a replacement
/// for typed capability evidence, resource quantities, or feasibility
/// results.
///
/// `capability_evidence` uses the canonical evidence type from the shared
/// semantic capability subsystem. The target model does not reinterpret or
/// authenticate these records.
///
/// `resources` is optional because target inventories may be incomplete or
/// because a provider may not expose resource information. Absence is not
/// interpreted as unlimited capacity.
///
/// All collections are dynamically sized. Their actual capacity is bounded
/// only by representable values and the resources available to the process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTargetDescription {
    id: TargetId,
    name: String,
    kind: String,
    status: TargetDescriptionStatus,
    provider: Option<TargetProvider>,
    labels: BTreeSet<String>,
    metadata: BTreeMap<String, String>,
    capability_evidence: Vec<HdlCapabilityEvidence>,
    resources: Option<HdlResourceProfile>,
}

impl HdlTargetDescription {
    /// Create an abstract target description.
    ///
    /// The new description begins in `Partial` status, has no provider,
    /// labels, metadata, capability evidence, or resource profile.
    ///
    /// Construction does not claim that a target exists, is reachable,
    /// supports any capability, or has any particular resource capacity.
    pub fn new(
        id: TargetId,
        name: impl Into<String>,
        kind: impl Into<String>,
    ) -> Result<Self, HdlTargetModelError> {
        let name = name.into();
        let kind = kind.into();

        validate_identifier(&name, "target name")?;
        validate_identifier(&kind, "target kind")?;

        Ok(Self {
            id,
            name,
            kind,
            status: TargetDescriptionStatus::Partial,
            provider: None,
            labels: BTreeSet::new(),
            metadata: BTreeMap::new(),
            capability_evidence: Vec::new(),
            resources: None,
        })
    }

    /// Return the stable target-description identity.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        &self.id
    }

    /// Return the human-readable target name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the open-ended target-kind identifier.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the declared completeness status.
    #[must_use]
    pub const fn status(&self) -> TargetDescriptionStatus {
        self.status
    }

    /// Return optional provider metadata.
    #[must_use]
    pub fn provider(&self) -> Option<&TargetProvider> {
        self.provider.as_ref()
    }

    /// Return labels in deterministic lexical order.
    #[must_use]
    pub fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    /// Return descriptive metadata in deterministic key order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return capability evidence in insertion order.
    ///
    /// Consumers must use the canonical evidence evaluator. In particular,
    /// repeated or conflicting records must not be silently collapsed into
    /// a Boolean support flag.
    #[must_use]
    pub fn capability_evidence(&self) -> &[HdlCapabilityEvidence] {
        &self.capability_evidence
    }

    /// Return the optional canonical HDL resource profile.
    ///
    /// `None` means that no resource profile was supplied. It does not mean
    /// that the target has unlimited resources.
    #[must_use]
    pub fn resources(&self) -> Option<&HdlResourceProfile> {
        self.resources.as_ref()
    }

    /// Set the declared completeness status.
    ///
    /// This records provider or caller intent. It does not independently
    /// verify that all properties are present or accurate.
    pub fn set_status(&mut self, status: TargetDescriptionStatus) {
        self.status = status;
    }

    /// Set or replace provider metadata.
    pub fn set_provider(
        &mut self,
        provider: Option<TargetProvider>,
    ) {
        self.provider = provider;
    }

    /// Insert a descriptive label.
    ///
    /// Duplicate labels are idempotent. Labels are not capability evidence.
    pub fn insert_label(
        &mut self,
        label: impl Into<String>,
    ) -> Result<bool, HdlTargetModelError> {
        let label = label.into();
        validate_identifier(&label, "target label")?;
        Ok(self.labels.insert(label))
    }

    /// Insert or replace a descriptive metadata entry.
    ///
    /// Keys are validated and preserved exactly. Replacing an existing key
    /// returns its previous value. Metadata does not establish target support.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, HdlTargetModelError> {
        let key = key.into();
        let value = value.into();

        validate_identifier(&key, "metadata key")?;

        if value.chars().any(char::is_control) {
            return Err(HdlTargetModelError::ControlCharacter {
                field: "metadata value",
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Append a canonical capability-evidence record.
    ///
    /// The evidence is preserved as supplied. This method does not establish
    /// authenticity, evaluate compatibility, or infer resource sufficiency.
    /// The canonical evidence subsystem remains responsible for validating
    /// and evaluating its semantics.
    pub fn push_capability_evidence(
        &mut self,
        evidence: HdlCapabilityEvidence,
    ) {
        self.capability_evidence.push(evidence);
    }

    /// Replace the optional resource profile.
    ///
    /// The profile uses canonical shared resource types. No target feasibility
    /// or physical allocation is performed by this operation.
    pub fn set_resources(
        &mut self,
        resources: Option<HdlResourceProfile>,
    ) {
        self.resources = resources;
    }

    /// Return whether the description contains no optional inventory data.
    ///
    /// Identity, name, kind, and status are mandatory fields and are not
    /// counted as optional inventory data.
    #[must_use]
    pub fn has_no_inventory_data(&self) -> bool {
        self.provider.is_none()
            && self.labels.is_empty()
            && self.metadata.is_empty()
            && self.capability_evidence.is_empty()
            && self.resources.is_none()
    }

    /// Validate local representation invariants.
    ///
    /// This validates identifiers and descriptive metadata. It deliberately
    /// does not infer completeness, evaluate evidence, compare requirements,
    /// or establish target feasibility.
    pub fn validate(&self) -> Result<(), HdlTargetModelError> {
        validate_identifier(self.id.as_str(), "target identifier")?;
        validate_identifier(&self.name, "target name")?;
        validate_identifier(&self.kind, "target kind")?;

        if let Some(provider) = &self.provider {
            validate_identifier(&provider.id, "provider identifier")?;

            if let Some(version) = provider.version.as_deref() {
                validate_identifier(version, "provider version")?;
            }
        }

        for label in &self.labels {
            validate_identifier(label, "target label")?;
        }

        for (key, value) in &self.metadata {
            validate_identifier(key, "metadata key")?;

            if value.chars().any(char::is_control) {
                return Err(HdlTargetModelError::ControlCharacter {
                    field: "metadata value",
                });
            }
        }

        Ok(())
    }
}

/// Errors produced by local target-description validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlTargetModelError {
    /// A required identifier or descriptive field was empty or whitespace.
    EmptyField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A field contained a Unicode control character.
    ControlCharacter {
        /// Name of the invalid field.
        field: &'static str,
    },
}

impl fmt::Display for HdlTargetModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(formatter, "{field} must not contain control characters")
            }
        }
    }
}

impl Error for HdlTargetModelError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> HdlTargetDescription {
        HdlTargetDescription::new(
            TargetId::new("example:target").expect("valid target ID"),
            "Example target",
            "future-heterogeneous-platform",
        )
        .expect("valid target description")
    }

    #[test]
    fn target_identity_is_open_ended_and_preserved() {
        let id = TargetId::new("provider:v2/future.fabric")
            .expect("open-ended identifier");

        assert_eq!(id.as_str(), "provider:v2/future.fabric");
        assert_eq!(id.to_string(), "provider:v2/future.fabric");
    }

    #[test]
    fn rejects_empty_and_control_character_identifiers() {
        assert!(matches!(
            TargetId::new(" \t "),
            Err(HdlTargetModelError::EmptyField { .. })
        ));

        assert!(matches!(
            TargetId::new("target\nname"),
            Err(HdlTargetModelError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn target_kind_is_not_a_closed_enumeration() {
        let description = target();

        assert_eq!(description.kind(), "future-heterogeneous-platform");
    }

    #[test]
    fn new_description_is_partial_and_has_no_implied_capacity() {
        let description = target();

        assert_eq!(
            description.status(),
            TargetDescriptionStatus::Partial
        );
        assert!(description.resources().is_none());
        assert!(description.capability_evidence().is_empty());
        assert!(description.has_no_inventory_data());
    }

    #[test]
    fn status_is_explicit_and_independent_of_target_kind() {
        let mut description = target();

        description.set_status(TargetDescriptionStatus::Unknown);
        assert_eq!(
            description.status(),
            TargetDescriptionStatus::Unknown
        );

        description.set_status(TargetDescriptionStatus::Complete);
        assert_eq!(
            description.status(),
            TargetDescriptionStatus::Complete
        );
    }

    #[test]
    fn labels_are_unique_and_deterministically_ordered() {
        let mut description = target();

        assert!(description.insert_label("simulator").unwrap());
        assert!(description.insert_label("portable").unwrap());
        assert!(!description.insert_label("simulator").unwrap());

        assert_eq!(
            description.labels().iter().map(String::as_str).collect::<Vec<_>>(),
            vec!["portable", "simulator"]
        );
    }

    #[test]
    fn metadata_replacement_returns_previous_value() {
        let mut description = target();

        assert_eq!(
            description
                .insert_metadata("provider.revision", "first")
                .unwrap(),
            None
        );

        assert_eq!(
            description
                .insert_metadata("provider.revision", "second")
                .unwrap(),
            Some("first".to_owned())
        );

        assert_eq!(
            description.metadata().get("provider.revision").map(String::as_str),
            Some("second")
        );
    }

    #[test]
    fn rejects_control_characters_in_metadata_values() {
        let mut description = target();

        assert!(matches!(
            description.insert_metadata("description", "bad\nvalue"),
            Err(HdlTargetModelError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn provider_metadata_is_optional_and_validated() {
        let provider = TargetProvider::new(
            "example.provider",
            Some("schema-v1".to_owned()),
        )
        .expect("valid provider");

        assert_eq!(provider.id(), "example.provider");
        assert_eq!(provider.version(), Some("schema-v1"));

        assert!(TargetProvider::new("", None).is_err());
        assert!(TargetProvider::new("provider", Some(" \t ".to_owned())).is_err());
    }

    #[test]
    fn target_validation_checks_representation_invariants() {
        let description = target();

        assert!(description.validate().is_ok());
    }

    #[test]
    fn cloning_preserves_the_description_contract() {
        let mut original = target();

        original.insert_label("portable").unwrap();
        original
            .insert_metadata("inventory.source", "test")
            .unwrap();

        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(HDL_TARGET_MODEL_SCHEMA_VERSION, 1);
        assert_eq!(
            HDL_TARGET_MODEL_MODULE_NAME,
            "zamani.hdl.targets.model"
        );
    }
}
