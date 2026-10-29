
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Logical state descriptors for Zamani distributed computation.
//!
//! # Purpose
//!
//! This module describes distributed state independently of its physical
//! storage, memory address, host, device, network location, serialization
//! format, and execution provider.
//!
//! It records logical identity, descriptive metadata, ownership intent,
//! consistency requirements, version information, and checkpoint policy.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - `DistributedStateDescriptor`;
//! - open-world ownership classifications;
//! - open-world consistency model names;
//! - logical state version metadata;
//! - checkpoint intent;
//! - validation of these descriptor-level invariants.
//!
//! It reuses:
//!
//! - `crate::distributed::ids::DistributedStateId`;
//! - `crate::distributed::ids::ParticipantId`;
//! - `crate::distributed::ids::CheckpointId` where checkpoint integration
//!   requires a checkpoint identity.
//!
//! It does not own:
//!
//! - source syntax or semantic analysis;
//! - the canonical type, ownership, borrowing, or lifetime system;
//! - state payload values or a competing intermediate representation;
//! - physical memory allocation or storage locations;
//! - replica creation, placement, or synchronization algorithms;
//! - consistency or transaction implementation;
//! - checkpoint persistence or restoration;
//! - state migration or transport;
//! - runtime execution or resource allocation.
//!
//! These remain responsibilities of the existing semantic, memory,
//! planning, persistence, transport, and runtime subsystems.
//!
//! # Distributed semantic contract
//!
//! A logical state descriptor is metadata, not the state payload itself.
//! Creating a descriptor does not allocate storage, create a replica,
//! authenticate an owner, prove a consistency guarantee, or checkpoint data.
//!
//! Shared and replicated state must identify their consistency requirements.
//! A runtime must not silently weaken those requirements when the selected
//! realization cannot satisfy them.
//!
//! A checkpoint policy expresses intent. It does not prove that a particular
//! state representation or live execution can actually be checkpointed.
//!
//! # Scalability
//!
//! This module imposes no application-defined universal maximum on the
//! number of states, descriptors, participants, or consistency model names.
//! Actual memory, address-space, integer-representation, and operational
//! limits remain finite and must be reported by the layer that encounters
//! them.
//!
//! # Serialization boundary
//!
//! These types intentionally do not derive `Deserialize`. Deserializing a
//! descriptor must not bypass its validation invariants. Persistence and
//! transport integrations should use versioned DTOs and validate the data
//! before constructing these domain types.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use core::fmt;
use std::error::Error;

use crate::distributed::ids::{DistributedStateId, ParticipantId};

/// A validated descriptive label for a logical distributed state.
///
/// A label is not an identity, a storage key, a variable binding, a memory
/// address, or an authorization credential.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateLabel(String);

impl StateLabel {
    /// Construct a validated state label.
    ///
    /// Empty strings, whitespace-only strings, and strings containing
    /// Unicode control characters are rejected. Leading and trailing
    /// whitespace is otherwise preserved.
    ///
    /// # Errors
    ///
    /// Returns a [`StateValidationError`] if the label is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, StateValidationError> {
        let value = value.into();
        validate_text(&value, TextField::Label)?;
        Ok(Self(value))
    }

    /// Return the label as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the label and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for StateLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An open-world name for a distributed state ownership classification.
///
/// Standard classifications are provided by [`StateOwnership`]. A custom
/// name allows future language or application-defined classifications
/// without requiring the distributed model to enumerate every possibility.
///
/// The name itself does not implement the ownership, borrowing, or lifetime
/// semantics owned by Zamani's canonical semantic model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateOwnershipName(String);

impl StateOwnershipName {
    /// Construct a validated ownership classification name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty, whitespace-only, or contains
    /// a Unicode control character.
    pub fn new(value: impl Into<String>) -> Result<Self, StateValidationError> {
        let value = value.into();
        validate_text(&value, TextField::OwnershipName)?;
        Ok(Self(value))
    }

    /// Return the classification name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StateOwnershipName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Logical ownership intent for distributed state.
///
/// These variants describe intended state behavior. They do not implement
/// synchronization, ownership transfer, replication, or storage.
///
/// `Custom` preserves an open-world extension point. The semantic layer must
/// define the meaning and validity of a custom classification before it can
/// be relied upon for execution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum StateOwnership {
    /// One logical owner is responsible for the state.
    SingleOwner,

    /// Multiple participants may observe or modify the state.
    Shared,

    /// The state is intended to have multiple equivalent or coordinated
    /// realizations.
    Replicated,

    /// The logical state is divided into independently managed portions.
    Partitioned,

    /// The state is derived from other authoritative state or computation.
    Derived,

    /// The state is immutable under its semantic contract.
    Immutable,

    /// The state is append-only under its semantic contract.
    AppendOnly,

    /// Updates are governed by a transactional semantic contract.
    Transactional,

    /// The state is represented by an event history.
    EventSourced,

    /// An extension-defined ownership classification.
    Custom(StateOwnershipName),
}

impl StateOwnership {
    /// Return whether this classification requires an explicit consistency
    /// model in the distributed state descriptor.
    ///
    /// Shared and replicated state require a declared consistency model.
    /// Other ownership classifications may also need consistency rules
    /// according to their semantic definitions.
    #[must_use]
    pub const fn requires_consistency(&self) -> bool {
        matches!(self, Self::Shared | Self::Replicated)
    }

    /// Return whether this classification is one of the standard variants.
    #[must_use]
    pub const fn is_standard(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }
}

/// An open-world name for a state consistency model.
///
/// This type does not implement a consistency algorithm. It records the
/// requested model so that semantic validation and planning can determine
/// whether a runtime and storage provider can satisfy it.
///
/// Examples of possible names include `strong`, `causal`, `eventual`,
/// `transactional`, or a qualified application-defined model. These examples
/// are not a claim that Zamani currently implements those guarantees.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsistencyModel(String);

impl ConsistencyModel {
    /// Construct a validated consistency model name.
    ///
    /// The name is open-world. This method validates its representation only;
    /// it does not prove that the model is defined, supported, or feasible.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty, whitespace-only, or contains
    /// a Unicode control character.
    pub fn new(value: impl Into<String>) -> Result<Self, StateValidationError> {
        let value = value.into();
        validate_text(&value, TextField::ConsistencyModel)?;
        Ok(Self(value))
    }

    /// Return the consistency model name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConsistencyModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Logical checkpoint intent for a distributed state.
///
/// This policy records what the program or caller requests. Whether a
/// checkpoint is possible depends on the actual representation, runtime,
/// provider, and current execution state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CheckpointPolicy {
    /// No checkpoint requirement has been declared.
    Unspecified,

    /// Checkpointing is permitted, but is not required for correctness.
    Permitted,

    /// The realization must provide the required checkpoint guarantee.
    Required,

    /// Checkpointing must not be used for this state under the applicable
    /// execution policy.
    Prohibited,
}

/// A monotonically increasing logical state version.
///
/// Versions are metadata for distinguishing revisions. This type does not
/// provide distributed consensus, conflict resolution, causal ordering, or
/// atomic update semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateVersion(u128);

impl StateVersion {
    /// The initial logical version.
    pub const INITIAL: Self = Self(0);

    /// Construct a version from its numeric representation.
    ///
    /// This does not establish that the version is current or valid for a
    /// particular state. Those facts require authoritative runtime metadata.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Return the numeric version.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }

    /// Return the next version, or report representational exhaustion.
    ///
    /// This method never wraps the version to zero.
    ///
    /// # Errors
    ///
    /// Returns [`StateValidationError::VersionExhausted`] if the current
    /// version is the maximum representable `u128`.
    pub fn checked_next(self) -> Result<Self, StateValidationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(StateValidationError::VersionExhausted)
    }
}

impl fmt::Display for StateVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated logical descriptor for distributed state.
///
/// The descriptor is portable metadata. It deliberately contains no payload,
/// physical storage address, endpoint, provider handle, replica list, or
/// runtime allocation.
///
/// The `owner` field identifies a logical participant when one is declared.
/// It does not authenticate that participant or prove that it currently owns
/// the state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedStateDescriptor {
    id: DistributedStateId,
    label: Option<StateLabel>,
    owner: Option<ParticipantId>,
    ownership: StateOwnership,
    consistency: Option<ConsistencyModel>,
    version: StateVersion,
    checkpoint_policy: CheckpointPolicy,
}

impl DistributedStateDescriptor {
    /// Construct a validated distributed state descriptor.
    ///
    /// `id` is allocated by the caller through the canonical distributed ID
    /// allocator. This constructor does not create or allocate an ID.
    ///
    /// Shared and replicated state require an explicit consistency model.
    /// This is a descriptor-level invariant, not proof that a particular
    /// runtime can implement that model.
    ///
    /// # Errors
    ///
    /// Returns [`StateValidationError::ConsistencyRequired`] when shared or
    /// replicated state has no declared consistency model.
    pub fn new(
        id: DistributedStateId,
        owner: Option<ParticipantId>,
        ownership: StateOwnership,
        consistency: Option<ConsistencyModel>,
        checkpoint_policy: CheckpointPolicy,
    ) -> Result<Self, StateValidationError> {
        let descriptor = Self {
            id,
            label: None,
            owner,
            ownership,
            consistency,
            version: StateVersion::INITIAL,
            checkpoint_policy,
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Return the logical state identity.
    #[must_use]
    pub const fn id(&self) -> DistributedStateId {
        self.id
    }

    /// Return the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&StateLabel> {
        self.label.as_ref()
    }

    /// Return the declared logical owner, if any.
    #[must_use]
    pub const fn owner(&self) -> Option<ParticipantId> {
        self.owner
    }

    /// Return the ownership classification.
    #[must_use]
    pub const fn ownership(&self) -> &StateOwnership {
        &self.ownership
    }

    /// Return the declared consistency model, if any.
    #[must_use]
    pub const fn consistency(&self) -> Option<&ConsistencyModel> {
        self.consistency.as_ref()
    }

    /// Return the logical version.
    #[must_use]
    pub const fn version(&self) -> StateVersion {
        self.version
    }

    /// Return the checkpoint policy.
    #[must_use]
    pub const fn checkpoint_policy(&self) -> CheckpointPolicy {
        self.checkpoint_policy
    }

    /// Set or replace the descriptive label.
    ///
    /// This changes descriptor metadata only; it does not change state
    /// identity or payload.
    pub fn set_label(&mut self, label: Option<StateLabel>) {
        self.label = label;
    }

    /// Replace the declared logical owner.
    ///
    /// This updates metadata only. It does not perform ownership transfer,
    /// authorization, migration, or synchronization.
    pub fn set_owner(&mut self, owner: Option<ParticipantId>) {
        self.owner = owner;
    }

    /// Replace the checkpoint policy.
    ///
    /// This updates intent only. It does not create a checkpoint or verify
    /// that checkpointing is supported.
    pub fn set_checkpoint_policy(&mut self, policy: CheckpointPolicy) {
        self.checkpoint_policy = policy;
    }

    /// Validate the descriptor's own invariants.
    ///
    /// This method does not consult a registry, semantic analyzer, runtime,
    /// storage provider, or consistency implementation.
    ///
    /// # Errors
    ///
    /// Returns [`StateValidationError::ConsistencyRequired`] when shared or
    /// replicated state has no declared consistency model.
    pub fn validate(&self) -> Result<(), StateValidationError> {
        if self.ownership.requires_consistency() && self.consistency.is_none() {
            return Err(StateValidationError::ConsistencyRequired {
                ownership: self.ownership.clone(),
            });
        }

        Ok(())
    }

    /// Return a descriptor with a caller-supplied next logical version.
    ///
    /// The caller is responsible for obtaining authoritative update
    /// permission and committing the version change through the runtime or
    /// storage system. This method alone does not commit an update.
    ///
    /// # Errors
    ///
    /// Returns [`StateValidationError::VersionRegression`] if the supplied
    /// version is older than the descriptor's current version.
    pub fn with_version(
        &self,
        version: StateVersion,
    ) -> Result<Self, StateValidationError> {
        if version < self.version {
            return Err(StateValidationError::VersionRegression {
                current: self.version,
                requested: version,
            });
        }

        let mut next = self.clone();
        next.version = version;
        next.validate()?;
        Ok(next)
    }
}

/// Errors produced by distributed state descriptor validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StateValidationError {
    /// A required textual field is empty or whitespace-only.
    EmptyText {
        /// The field that failed validation.
        field: &'static str,
    },

    /// A textual field contains a Unicode control character.
    ControlCharacter {
        /// The field that failed validation.
        field: &'static str,
    },

    /// Shared or replicated state has no declared consistency model.
    ConsistencyRequired {
        /// The ownership classification requiring consistency.
        ownership: StateOwnership,
    },

    /// The logical version cannot be incremented in its representation.
    VersionExhausted,

    /// A requested version is older than the current descriptor version.
    VersionRegression {
        /// Current descriptor version.
        current: StateVersion,

        /// Requested version.
        requested: StateVersion,
    },
}

impl fmt::Display for StateValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(f, "{field} must not be empty or whitespace-only")
            }
            Self::ControlCharacter { field } => {
                write!(f, "{field} must not contain Unicode control characters")
            }
            Self::ConsistencyRequired { ownership } => {
                write!(
                    f,
                    "ownership classification '{ownership}' requires a consistency model"
                )
            }
            Self::VersionExhausted => {
                f.write_str("distributed state version exhausted its numeric representation")
            }
            Self::VersionRegression { current, requested } => {
                write!(
                    f,
                    "state version regression: current version is {current}, requested {requested}"
                )
            }
        }
    }
}

impl Error for StateValidationError {}

impl fmt::Display for StateOwnership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleOwner => f.write_str("single-owner"),
            Self::Shared => f.write_str("shared"),
            Self::Replicated => f.write_str("replicated"),
            Self::Partitioned => f.write_str("partitioned"),
            Self::Derived => f.write_str("derived"),
            Self::Immutable => f.write_str("immutable"),
            Self::AppendOnly => f.write_str("append-only"),
            Self::Transactional => f.write_str("transactional"),
            Self::EventSourced => f.write_str("event-sourced"),
            Self::Custom(name) => name.fmt(f),
        }
    }
}

#[derive(Clone, Copy)]
enum TextField {
    Label,
    OwnershipName,
    ConsistencyModel,
}

impl TextField {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Label => "state label",
            Self::OwnershipName => "state ownership name",
            Self::ConsistencyModel => "consistency model",
        }
    }
}

fn validate_text(value: &str, field: TextField) -> Result<(), StateValidationError> {
    if value.trim().is_empty() {
        return Err(StateValidationError::EmptyText {
            field: field.as_str(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(StateValidationError::ControlCharacter {
            field: field.as_str(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::ids::DistributedStateId;

    fn state_id() -> DistributedStateId {
        DistributedStateId::new(1, 1).expect("non-zero test ID components")
    }

    fn participant_id() -> ParticipantId {
        ParticipantId::new(1, 2).expect("non-zero test ID components")
    }

    fn consistency() -> ConsistencyModel {
        ConsistencyModel::new("causal").expect("valid consistency name")
    }

    #[test]
    fn constructs_single_owner_state_without_consistency_requirement() {
        let descriptor = DistributedStateDescriptor::new(
            state_id(),
            Some(participant_id()),
            StateOwnership::SingleOwner,
            None,
            CheckpointPolicy::Unspecified,
        )
        .expect("single-owner state need not declare shared consistency");

        assert_eq!(descriptor.id(), state_id());
        assert_eq!(descriptor.owner(), Some(participant_id()));
        assert_eq!(descriptor.version(), StateVersion::INITIAL);
        assert!(descriptor.consistency().is_none());
    }

    #[test]
    fn shared_state_requires_consistency() {
        let result = DistributedStateDescriptor::new(
            state_id(),
            None,
            StateOwnership::Shared,
            None,
            CheckpointPolicy::Permitted,
        );

        assert!(matches!(
            result,
            Err(StateValidationError::ConsistencyRequired { .. })
        ));
    }

    #[test]
    fn replicated_state_requires_consistency() {
        let descriptor = DistributedStateDescriptor::new(
            state_id(),
            None,
            StateOwnership::Replicated,
            Some(consistency()),
            CheckpointPolicy::Required,
        );

        assert!(descriptor.is_ok());
    }

    #[test]
    fn supports_open_world_consistency_names() {
        let model = ConsistencyModel::new("application.consistency.v2")
            .expect("qualified names are open-world");

        assert_eq!(model.as_str(), "application.consistency.v2");
    }

    #[test]
    fn rejects_empty_and_control_character_names() {
        assert!(matches!(
            StateLabel::new(" \t "),
            Err(StateValidationError::EmptyText { .. })
        ));

        assert!(matches!(
            ConsistencyModel::new("causal\nconsistency"),
            Err(StateValidationError::ControlCharacter { .. })
        ));

        assert!(matches!(
            StateOwnershipName::new(""),
            Err(StateValidationError::EmptyText { .. })
        ));
    }

    #[test]
    fn versions_increment_without_wrapping() {
        assert_eq!(
            StateVersion::INITIAL
                .checked_next()
                .expect("initial version can increment"),
            StateVersion::new(1)
        );

        assert_eq!(
            StateVersion::new(u128::MAX).checked_next(),
            Err(StateValidationError::VersionExhausted)
        );
    }

    #[test]
    fn rejects_version_regression() {
        let descriptor = DistributedStateDescriptor::new(
            state_id(),
            None,
            StateOwnership::Immutable,
            None,
            CheckpointPolicy::Prohibited,
        )
        .expect("valid immutable descriptor");

        assert_eq!(
            descriptor.with_version(StateVersion::new(1))
                .expect("version can advance")
                .version(),
            StateVersion::new(1)
        );

        assert!(matches!(
            descriptor.with_version(StateVersion::new(0)),
            Err(StateValidationError::VersionRegression { .. })
        ));
    }

    #[test]
    fn label_is_metadata_not_identity() {
        let mut descriptor = DistributedStateDescriptor::new(
            state_id(),
            None,
            StateOwnership::Derived,
            None,
            CheckpointPolicy::Unspecified,
        )
        .expect("valid derived descriptor");

        descriptor.set_label(Some(
            StateLabel::new("cached computation").expect("valid label"),
        ));

        assert_eq!(
            descriptor.label().map(StateLabel::as_str),
            Some("cached computation")
        );
        assert_eq!(descriptor.id(), state_id());
    }

    #[test]
    fn custom_ownership_classifications_remain_open_world() {
        let custom = StateOwnership::Custom(
            StateOwnershipName::new("application.partitioned-append-only")
                .expect("valid extension name"),
        );

        assert!(!custom.is_standard());
        assert_eq!(custom.to_string(), "application.partitioned-append-only");
    }
}
