
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Logical participant model for Zamani distributed computation.
//!
//! Path: `src/distributed/model/participant.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Describe a logical participant in a distributed computation independently
//! of the machine, process, device, network endpoint, or execution provider
//! that may eventually realize it.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - The logical participant descriptor.
//! - Open-world participant roles.
//! - Optional human-readable participant labels.
//! - Extensible, non-authoritative descriptive metadata.
//! - Validation of the descriptor's own representation.
//!
//! This module does not own:
//!
//! - Participant identifiers: `crate::distributed::ids::ParticipantId`
//!   is authoritative.
//! - Semantic capability identities, requirements, or evidence.
//! - Resource quantities, allocation, or reservation.
//! - Physical nodes, network addresses, processes, or device identities.
//! - Membership, discovery, health, liveness, or authentication.
//! - Placement, partitioning, scheduling, routing, or execution.
//! - ZUIR, ZCIR, `quantum::ir`, HDLIR, or backend implementations.
//!
//! # POCO-REAF and scalability
//!
//! A logical participant is not tied to a fixed physical topology.
//! Its descriptor can be used whether a realization employs one execution
//! context or many, subject to the actual capabilities and resources available.
//!
//! Collections have no application-defined universal capacity ceiling.
//! Real memory, address-space, serialization, and identifier-representation
//! limits remain finite and must be handled explicitly by their owning layers.
//!
//! # Identity and equality
//!
//! `ParticipantId` is the participant's identity. Labels, roles, and metadata
//! describe the participant but do not replace its identity.
//!
//! Two descriptors with the same ID refer to the same logical identity,
//! even if their descriptive fields differ. Registries must detect and
//! explicitly reconcile conflicting descriptors instead of silently
//! treating the conflict as a new participant.
//!
//! # Security
//!
//! A descriptor is data, not proof of identity or authorization.
//! Constructing or deserializing one does not authenticate a participant.
//! Metadata supplied by an untrusted peer must not be used as trusted
//! capability evidence or authorization policy without verification.

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::distributed::ids::ParticipantId;

/// An open-world logical role for a distributed participant.
///
/// Roles are descriptive classifications, not an exhaustive list of
/// supported hardware or execution providers.
///
/// Examples include `worker`, `coordinator`, `service`, `agent`,
/// `controller`, or application-defined role names.
///
/// Whether a role is supported, trusted, or eligible for an operation is
/// decided by the appropriate semantic, policy, membership, or planning layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParticipantRole(String);

impl ParticipantRole {
    /// Construct a participant role from a non-empty name.
    ///
    /// Role names are open-world. This constructor validates only the
    /// representation; it does not resolve a role against a registry or
    /// assert that an implementation supports it.
    ///
    /// # Errors
    ///
    /// Returns [`ParticipantValidationError::EmptyRole`] if the value is
    /// empty or contains only whitespace, or
    /// [`ParticipantValidationError::ControlCharacter`] if it contains
    /// a Unicode control character.
    pub fn new(value: impl Into<String>) -> Result<Self, ParticipantValidationError> {
        let value = value.into();
        validate_text(&value, TextField::Role)?;

        Ok(Self(value))
    }

    /// Return the role name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the role and return its name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ParticipantRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An optional, human-readable label for a logical participant.
///
/// A label is not an identity, network address, machine name, or
/// authorization credential.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParticipantLabel(String);

impl ParticipantLabel {
    /// Construct a validated participant label.
    ///
    /// # Errors
    ///
    /// Returns an error if the label is empty, contains only whitespace,
    /// or contains a Unicode control character.
    pub fn new(value: impl Into<String>) -> Result<Self, ParticipantValidationError> {
        let value = value.into();
        validate_text(&value, TextField::Label)?;

        Ok(Self(value))
    }

    /// Return the label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the label and return its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ParticipantLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The category of text rejected by participant descriptor validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextField {
    Role,
    Label,
    MetadataKey,
    MetadataValue,
}

fn validate_text(
    value: &str,
    field: TextField,
) -> Result<(), ParticipantValidationError> {
    if value.trim().is_empty() {
        return Err(match field {
            TextField::Role => ParticipantValidationError::EmptyRole,
            TextField::Label => ParticipantValidationError::EmptyLabel,
            TextField::MetadataKey => ParticipantValidationError::EmptyMetadataKey,
            TextField::MetadataValue => ParticipantValidationError::EmptyMetadataValue,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ParticipantValidationError::ControlCharacter { field });
    }

    Ok(())
}

/// Validation failures for a logical participant descriptor.
///
/// These errors concern the descriptor's representation only. They do not
/// indicate resource infeasibility, unavailable capabilities, or a failed
/// physical participant.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParticipantValidationError {
    /// The role is empty or contains only whitespace.
    EmptyRole,

    /// The optional label was supplied but is empty or whitespace-only.
    EmptyLabel,

    /// A metadata key is empty or contains only whitespace.
    EmptyMetadataKey,

    /// A metadata value is empty or contains only whitespace.
    EmptyMetadataValue,

    /// A text field contains a Unicode control character.
    ControlCharacter {
        /// The field in which the invalid character was found.
        field: TextField,
    },
}

impl fmt::Display for ParticipantValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRole => f.write_str(
                "participant role must not be empty or whitespace-only",
            ),
            Self::EmptyLabel => f.write_str(
                "participant label must not be empty or whitespace-only",
            ),
            Self::EmptyMetadataKey => f.write_str(
                "participant metadata key must not be empty or whitespace-only",
            ),
            Self::EmptyMetadataValue => f.write_str(
                "participant metadata value must not be empty or whitespace-only",
            ),
            Self::ControlCharacter { field } => {
                write!(f, "participant {field:?} contains a control character")
            }
        }
    }
}

impl std::error::Error for ParticipantValidationError {}

/// A logical participant in a distributed Zamani computation.
///
/// This descriptor deliberately excludes physical placement, endpoint
/// addresses, liveness, resource quantities, and authentication state.
///
/// Metadata is descriptive and must not be interpreted as trusted security
/// evidence or as a substitute for the canonical semantic capability model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Participant {
    /// Stable logical identity, allocated by the distributed ID subsystem.
    id: ParticipantId,

    /// Open-world logical role.
    role: ParticipantRole,

    /// Optional human-readable label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<ParticipantLabel>,

    /// Extensible descriptive metadata.
    ///
    /// Keys and values are strings to avoid introducing a second type system
    /// or duplicating the canonical semantic attribute representation.
    ///
    /// This map is not authoritative for capabilities, requirements,
    /// authorization, physical addresses, or resource allocation.
    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl Participant {
    /// Construct a participant descriptor with a stable ID and role.
    ///
    /// A successful construction does not register, discover, authenticate,
    /// schedule, or start the participant.
    #[must_use]
    pub fn new(id: ParticipantId, role: ParticipantRole) -> Self {
        Self {
            id,
            role,
            label: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Construct a participant descriptor from a role name.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the role name is invalid.
    pub fn with_role(
        id: ParticipantId,
        role: impl Into<String>,
    ) -> Result<Self, ParticipantValidationError> {
        Ok(Self::new(id, ParticipantRole::new(role)?))
    }

    /// Return the participant's stable logical identity.
    #[must_use]
    pub const fn id(&self) -> ParticipantId {
        self.id
    }

    /// Return the participant's logical role.
    #[must_use]
    pub fn role(&self) -> &ParticipantRole {
        &self.role
    }

    /// Replace the participant's logical role.
    ///
    /// This changes descriptive data only. It does not grant permissions
    /// or establish that the participant can perform that role.
    pub fn set_role(&mut self, role: ParticipantRole) {
        self.role = role;
    }

    /// Return the optional human-readable label.
    #[must_use]
    pub fn label(&self) -> Option<&ParticipantLabel> {
        self.label.as_ref()
    }

    /// Set a validated human-readable label.
    pub fn set_label(&mut self, label: ParticipantLabel) {
        self.label = Some(label);
    }

    /// Clear the optional human-readable label.
    pub fn clear_label(&mut self) {
        self.label = None;
    }

    /// Return descriptive metadata.
    ///
    /// The returned map is ordered for deterministic iteration and
    /// reproducible presentation.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Look up a descriptive metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Insert or replace a descriptive metadata entry.
    ///
    /// Metadata values are intentionally opaque to this module. The caller
    /// must not use them as a substitute for canonical semantic attributes,
    /// capability evidence, trusted identity, or policy evaluation.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the key or value is empty,
    /// whitespace-only, or contains a Unicode control character.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ParticipantValidationError> {
        let key = key.into();
        let value = value.into();

        validate_text(&key, TextField::MetadataKey)?;
        validate_text(&value, TextField::MetadataValue)?;

        Ok(self.metadata.insert(key, value))
    }

    /// Remove a metadata entry by key.
    ///
    /// Returns the previous value when the key existed.
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.metadata.remove(key)
    }

    /// Return whether the descriptor contains a metadata key.
    #[must_use]
    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }

    /// Return a copy of the descriptor with a replacement role.
    #[must_use]
    pub fn with_replacement_role(mut self, role: ParticipantRole) -> Self {
        self.role = role;
        self
    }

    /// Return a copy of the descriptor with a label.
    #[must_use]
    pub fn with_label(mut self, label: ParticipantLabel) -> Self {
        self.label = Some(label);
        self
    }

    /// Return a copy of the descriptor with one metadata entry.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the key or value is invalid.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ParticipantValidationError> {
        self.insert_metadata(key, value)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::ids::ParticipantId;

    fn test_id() -> ParticipantId {
        ParticipantId::new(1, 1).expect("valid test participant ID")
    }

    #[test]
    fn constructs_logical_participant() {
        let role = ParticipantRole::new("worker").expect("valid role");
        let participant = Participant::new(test_id(), role);

        assert_eq!(participant.id(), test_id());
        assert_eq!(participant.role().as_str(), "worker");
        assert!(participant.label().is_none());
        assert!(participant.metadata().is_empty());
    }

    #[test]
    fn role_names_are_open_world() {
        let role = ParticipantRole::new("future.photonic.execution")
            .expect("an extension-defined role is valid");

        assert_eq!(role.as_str(), "future.photonic.execution");
    }

    #[test]
    fn rejects_empty_role() {
        assert_eq!(
            ParticipantRole::new(" \t "),
            Err(ParticipantValidationError::EmptyRole)
        );
    }

    #[test]
    fn rejects_control_characters_in_labels() {
        assert!(matches!(
            ParticipantLabel::new("worker\nnode"),
            Err(ParticipantValidationError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn metadata_is_validated_and_replaceable() {
        let mut participant =
            Participant::with_role(test_id(), "worker").expect("valid participant");

        assert_eq!(
            participant
                .insert_metadata("region", "edge")
                .expect("valid metadata"),
            None
        );

        assert_eq!(
            participant
                .insert_metadata("region", "cloud")
                .expect("valid replacement"),
            Some("edge".to_owned())
        );

        assert_eq!(participant.metadata_value("region"), Some("cloud"));
    }

    #[test]
    fn rejects_empty_metadata() {
        let mut participant =
            Participant::with_role(test_id(), "worker").expect("valid participant");

        assert_eq!(
            participant.insert_metadata("", "value"),
            Err(ParticipantValidationError::EmptyMetadataKey)
        );

        assert_eq!(
            participant.insert_metadata("key", " "),
            Err(ParticipantValidationError::EmptyMetadataValue)
        );
    }

    #[test]
    fn metadata_removal_reports_previous_value() {
        let participant = Participant::with_role(test_id(), "worker")
            .expect("valid participant")
            .with_metadata("region", "edge")
            .expect("valid metadata");

        let mut participant = participant;

        assert_eq!(
            participant.remove_metadata("region"),
            Some("edge".to_owned())
        );
        assert_eq!(participant.remove_metadata("region"), None);
    }

    #[test]
    fn participant_clone_preserves_logical_identity() {
        let participant = Participant::with_role(test_id(), "worker")
            .expect("valid participant")
            .with_label(
                ParticipantLabel::new("portable-worker").expect("valid label"),
            );

        let clone = participant.clone();

        assert_eq!(participant.id(), clone.id());
        assert_eq!(participant, clone);
    }

    #[test]
    fn descriptor_serialization_round_trip() {
        let participant = Participant::with_role(test_id(), "worker")
            .expect("valid participant")
            .with_label(
                ParticipantLabel::new("edge-worker").expect("valid label"),
            )
            .with_metadata("deployment", "test")
            .expect("valid metadata");

        let encoded =
            serde_json::to_string(&participant).expect("serialize participant");
        let decoded: Participant =
            serde_json::from_str(&encoded).expect("deserialize participant");

        assert_eq!(decoded, participant);
    }
}
