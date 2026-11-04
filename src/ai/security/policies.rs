#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Security — Policy Attachments
//!
//! Path: `src/ai/security/policies.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the AI security policy attachment boundary.
//!
//! This module associates existing canonical policy references with
//! AI principals and AI computational resources.
//!
//! It supports:
//!
//! - canonical policy references;
//! - AI principal scopes;
//! - AI resource scopes;
//! - AI action scopes;
//! - combined action and resource scopes;
//! - explicitly broad AI policy scopes;
//! - policy conditions;
//! - policy argument references;
//! - descriptive metadata;
//! - deterministic policy collections;
//! - structural validation;
//! - checked deserialization.
//!
//! ## Architectural ownership
//!
//! This module is an AI security integration layer.
//!
//! Canonical authorities remain:
//!
//! - `crate::ai::security::identity`
//!   Owns AI identity and principal references.
//!
//! - `crate::ai::security::authorization`
//!   Owns AI authorization integration.
//!
//! - `crate::ai::agents::policies`
//!   Owns reusable agent policy references, expression references,
//!   and policy argument representations.
//!
//! - `crate::distributed::security::policy`
//!   Owns runtime policy evaluation and enforcement.
//!
//! - `crate::frontend::semantic`
//!   Owns source-level policy resolution and semantic validation.
//!
//! - `grammar/ai/policies.g4`
//!   Owns the AI policy grammar boundary.
//!
//! - `grammar/policies/policy.g4`
//!   Owns universal policy syntax.
//!
//! - `grammar/security/policies.g4`
//!   Owns security policy grammar composition.
//!
//! ## Owns
//!
//! - AI security policy scopes.
//! - AI security policy binding identities.
//! - AI security policy attachments.
//! - Structural validation of those attachments.
//! - Deterministic collections of attachments.
//!
//! ## Does not own
//!
//! - Policy declarations.
//! - Policy source parsing.
//! - AST construction.
//! - Policy expression parsing.
//! - Policy expression evaluation.
//! - Policy inheritance.
//! - Policy precedence.
//! - Policy conflict resolution.
//! - Authentication.
//! - Trust establishment.
//! - Authorization decisions.
//! - Permission grants.
//! - Capability issuance.
//! - Delegation enforcement.
//! - Policy storage.
//! - Policy distribution.
//! - Runtime policy enforcement.
//! - Execution scheduling.
//! - Hardware selection.
//!
//! ## Security invariants
//!
//! 1. A policy reference is not authorization.
//! 2. A policy attachment does not grant permission.
//! 3. A principal reference does not prove authentication.
//! 4. A resource reference does not prove ownership.
//! 5. A policy condition is not evaluated here.
//! 6. An unresolved policy is not permission.
//! 7. An invalid policy is not permission.
//! 8. An unavailable policy is not permission.
//! 9. An indeterminate policy is not permission.
//! 10. Broad applicability must be explicit.
//! 11. Intelligence does not increase authority.
//! 12. Capability declarations do not grant authority.
//!
//! The canonical semantic and distributed security systems must
//! independently resolve and enforce every applicable policy.
//!
//! ## Scalability
//!
//! This module imposes no artificial semantic maximum on:
//!
//! - policy bindings;
//! - principals;
//! - resources;
//! - policy references;
//! - metadata entries;
//! - policy arguments;
//! - action identifiers;
//! - AI computational domains.
//!
//! Collection growth is limited by available memory, representation
//! limits, and explicitly configured operational budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## POCO-REAF
//!
//! Policy attachments describe logical security intent.
//!
//! They do not encode CPU, GPU, FPGA, ASIC, QPU, neuromorphic,
//! operating-system, vendor, memory-topology, or network-topology
//! assumptions.
//!
//! The same logical policy attachment can be interpreted by the
//! appropriate security infrastructure across compatible execution
//! environments.
//!
//! ## Integration
//!
//! `identity.rs`
//!     -> AI principal and resource identities.
//!
//! `authorization.rs`
//!     -> AI authorization action references.
//!
//! `agents/policies.rs`
//!     -> canonical policy references, expressions and arguments.
//!
//! `frontend/semantic`
//!     -> source policy resolution and semantic validation.
//!
//! `distributed/security/policy.rs`
//!     -> policy evaluation and enforcement.
//!
//! `distributed/security/authorization.rs`
//!     -> protected-operation authorization.
//!
//! `runtime`
//!     -> enforcement at actual protected execution boundaries.

use serde::de;
use serde::{Deserialize, Deserializer, Serialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::agents::policies::{
    AgentPolicyArgument,
    AgentPolicyError,
    AgentPolicyExpressionReference,
    AgentPolicyReference,
};

use crate::ai::security::authorization::AiAuthorizationAction;

use crate::ai::security::identity::{
    AiIdentityRef,
    AiPrincipalRef,
};

// ============================================================================
// PUBLIC POLICY REFERENCE
// ============================================================================

/// Canonical policy reference used by the AI security subsystem.
///
/// The underlying reference type is owned by the existing agent-policy
/// integration module.
///
/// This re-export avoids introducing a competing policy-reference type.
pub use crate::ai::agents::policies::AgentPolicyReference
    as AiSecurityPolicyReference;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by AI security policy operations.
pub type AiSecurityPolicyResult<T> =
    Result<T, AiSecurityPolicyError>;

// ============================================================================
// VALIDATION
// ============================================================================

/// Validate an identifier used by this module.
///
/// Identifiers must be non-empty, have no surrounding whitespace,
/// and contain no whitespace or control characters.
///
/// No arbitrary maximum length is imposed.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> AiSecurityPolicyResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
        || value.chars().any(char::is_control)
    {
        return Err(AiSecurityPolicyError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validate an opaque expression reference.
///
/// Expression syntax and meaning are owned by the canonical frontend.
fn validate_expression_reference(
    value: &AgentPolicyExpressionReference,
) -> AiSecurityPolicyResult<()> {
    if value.as_str().is_empty()
        || value.as_str().trim() != value.as_str()
        || value.as_str().chars().any(char::is_control)
    {
        return Err(
            AiSecurityPolicyError::InvalidExpressionReference,
        );
    }

    Ok(())
}

/// Validate descriptive metadata.
///
/// Metadata is descriptive and must not silently become executable
/// policy semantics.
fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> AiSecurityPolicyResult<()> {
    for (key, value) in metadata {
        validate_identifier("metadata key", key)?;

        if value.chars().any(char::is_control) {
            return Err(
                AiSecurityPolicyError::InvalidMetadataValue {
                    key: key.clone(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// POLICY BINDING IDENTITY
// ============================================================================

/// Identity of one AI security policy attachment.
///
/// The identity is local to its containing policy set.
///
/// It is not a globally unique identifier and does not replace
/// canonical ZUIR identifiers.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct AiSecurityPolicyBindingId(String);

impl AiSecurityPolicyBindingId {
    /// Construct a validated policy binding identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiSecurityPolicyResult<Self> {
        let value = value.into();

        validate_identifier(
            "AI security policy binding identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the binding identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identity and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for AiSecurityPolicyBindingId {
    type Error = AiSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AiSecurityPolicyBindingId> for String {
    fn from(value: AiSecurityPolicyBindingId) -> Self {
        value.0
    }
}

impl fmt::Display for AiSecurityPolicyBindingId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// POLICY SUBJECT
// ============================================================================

/// Principal scope of an AI security policy.
///
/// `AllPrincipals` is an explicit broad scope.
///
/// It is never inferred from a missing or empty principal collection.
///
/// These variants describe applicability only. They do not establish
/// authentication or authorization.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiSecurityPolicySubject {
    /// Apply to one explicitly identified AI security principal.
    Principal(AiPrincipalRef),

    /// Explicitly express applicability to all AI principals.
    ///
    /// The canonical semantic and security systems must determine
    /// whether this broad scope is valid and permitted.
    AllPrincipals,
}

// ============================================================================
// POLICY TARGET
// ============================================================================

/// Target of an AI security policy attachment.
///
/// Targets identify logical AI operations and resources.
///
/// They do not select physical hardware or execution placement.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiSecurityPolicyTarget {
    /// Apply to a particular AI authorization action.
    Action {
        /// Open-ended AI authorization action identifier.
        action: String,
    },

    /// Apply to a particular AI entity.
    Resource {
        /// Target AI resource identity.
        resource: AiIdentityRef,
    },

    /// Apply to an action performed against a particular AI entity.
    ActionOnResource {
        /// Open-ended AI authorization action identifier.
        action: String,

        /// Target AI resource identity.
        resource: AiIdentityRef,
    },

    /// Explicitly apply to all AI operations.
    ///
    /// This is a broad scope and must be explicitly declared.
    AllAiOperations,
}

impl AiSecurityPolicyTarget {
    /// Construct an action target.
    pub fn action(
        action: impl Into<String>,
    ) -> AiSecurityPolicyResult<Self> {
        let action = action.into();

        validate_ai_action(&action)?;

        Ok(Self::Action { action })
    }

    /// Construct a resource target.
    #[must_use]
    pub fn resource(resource: AiIdentityRef) -> Self {
        Self::Resource { resource }
    }

    /// Construct an action-on-resource target.
    pub fn action_on_resource(
        action: impl Into<String>,
        resource: AiIdentityRef,
    ) -> AiSecurityPolicyResult<Self> {
        let action = action.into();

        validate_ai_action(&action)?;

        Ok(Self::ActionOnResource {
            action,
            resource,
        })
    }

    /// Construct an explicit broad AI-operation target.
    #[must_use]
    pub const fn all_ai_operations() -> Self {
        Self::AllAiOperations
    }

    /// Validate the target structure.
    ///
    /// This does not establish that the action or resource exists.
    pub fn validate(&self) -> AiSecurityPolicyResult<()> {
        match self {
            Self::Action { action } => {
                validate_ai_action(action)
            }

            Self::Resource { .. } => Ok(()),

            Self::ActionOnResource { action, .. } => {
                validate_ai_action(action)
            }

            Self::AllAiOperations => Ok(()),
        }
    }
}

/// Validate an AI action using the canonical AI authorization action.
///
/// This deliberately reuses the authorization boundary's action
/// validation instead of introducing another action namespace.
fn validate_ai_action(
    action: &str,
) -> AiSecurityPolicyResult<()> {
    AiAuthorizationAction::new(action.to_owned())
        .map(|_| ())
        .map_err(|_| {
            AiSecurityPolicyError::InvalidAction {
                value: action.to_owned(),
            }
        })
}

// ============================================================================
// POLICY SCOPE
// ============================================================================

/// Complete subject-and-target scope of an AI security policy.
///
/// Both dimensions are explicit.
///
/// A broad subject does not imply a broad target, and a broad target
/// does not imply a broad subject.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AiSecurityPolicyScope {
    subject: AiSecurityPolicySubject,
    target: AiSecurityPolicyTarget,
}

impl AiSecurityPolicyScope {
    /// Construct a policy scope.
    pub fn new(
        subject: AiSecurityPolicySubject,
        target: AiSecurityPolicyTarget,
    ) -> AiSecurityPolicyResult<Self> {
        let scope = Self {
            subject,
            target,
        };

        scope.validate()?;

        Ok(scope)
    }

    /// Return the policy subject.
    #[must_use]
    pub fn subject(&self) -> &AiSecurityPolicySubject {
        &self.subject
    }

    /// Return the policy target.
    #[must_use]
    pub fn target(&self) -> &AiSecurityPolicyTarget {
        &self.target
    }

    /// Validate the scope structure.
    pub fn validate(&self) -> AiSecurityPolicyResult<()> {
        self.target.validate()
    }
}

// ============================================================================
// POLICY BINDING
// ============================================================================

/// A declarative attachment of a canonical policy to an AI security scope.
///
/// The binding references an existing policy. It does not define the
/// policy's rules, expressions, precedence, inheritance, or meaning.
///
/// It does not grant permission.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiSecurityPolicyBinding {
    id: AiSecurityPolicyBindingId,

    policy: AgentPolicyReference,

    scope: AiSecurityPolicyScope,

    condition: Option<AgentPolicyExpressionReference>,

    arguments: BTreeMap<String, AgentPolicyArgument>,

    metadata: BTreeMap<String, String>,
}

/// Deserialization representation for an AI security policy binding.
///
/// The public binding is constructed only after structural validation.
#[derive(Deserialize)]
struct AiSecurityPolicyBindingData {
    id: AiSecurityPolicyBindingId,

    policy: AgentPolicyReference,

    scope: AiSecurityPolicyScope,

    condition: Option<AgentPolicyExpressionReference>,

    #[serde(default)]
    arguments: BTreeMap<String, AgentPolicyArgument>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl<'de> Deserialize<'de> for AiSecurityPolicyBinding {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = AiSecurityPolicyBindingData::deserialize(
            deserializer,
        )?;

        let binding = Self {
            id: data.id,
            policy: data.policy,
            scope: data.scope,
            condition: data.condition,
            arguments: data.arguments,
            metadata: data.metadata,
        };

        binding.validate().map_err(de::Error::custom)?;

        Ok(binding)
    }
}

impl AiSecurityPolicyBinding {
    /// Construct an AI security policy binding.
    pub fn new(
        id: AiSecurityPolicyBindingId,
        policy: AgentPolicyReference,
        scope: AiSecurityPolicyScope,
    ) -> AiSecurityPolicyResult<Self> {
        let binding = Self {
            id,
            policy,
            scope,
            condition: None,
            arguments: BTreeMap::new(),
            metadata: BTreeMap::new(),
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Attach a canonical policy-expression reference.
    ///
    /// The expression is resolved and evaluated by the canonical
    /// frontend and policy infrastructure.
    pub fn with_condition(
        mut self,
        condition: AgentPolicyExpressionReference,
    ) -> AiSecurityPolicyResult<Self> {
        validate_expression_reference(&condition)?;

        self.condition = Some(condition);

        Ok(self)
    }

    /// Add a named policy argument.
    ///
    /// Argument expressions remain owned by the canonical frontend.
    ///
    /// Duplicate names are rejected.
    pub fn add_argument(
        &mut self,
        argument: AgentPolicyArgument,
    ) -> AiSecurityPolicyResult<()> {
        validate_identifier(
            "policy argument name",
            argument.name(),
        )?;

        validate_expression_reference(argument.value())?;

        if self.arguments.contains_key(argument.name()) {
            return Err(
                AiSecurityPolicyError::DuplicateArgument {
                    name: argument.name().to_owned(),
                },
            );
        }

        self.arguments.insert(
            argument.name().to_owned(),
            argument,
        );

        Ok(())
    }

    /// Add descriptive metadata.
    ///
    /// Metadata is not evaluated as policy logic.
    ///
    /// Duplicate keys are rejected.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiSecurityPolicyResult<()> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata key", &key)?;

        if value.chars().any(char::is_control) {
            return Err(
                AiSecurityPolicyError::InvalidMetadataValue {
                    key,
                },
            );
        }

        if self.metadata.contains_key(&key) {
            return Err(
                AiSecurityPolicyError::DuplicateMetadata {
                    key,
                },
            );
        }

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Return the binding identity.
    #[must_use]
    pub fn id(&self) -> &AiSecurityPolicyBindingId {
        &self.id
    }

    /// Return the canonical policy reference.
    #[must_use]
    pub fn policy(&self) -> &AgentPolicyReference {
        &self.policy
    }

    /// Return the policy scope.
    #[must_use]
    pub fn scope(&self) -> &AiSecurityPolicyScope {
        &self.scope
    }

    /// Return the optional condition reference.
    #[must_use]
    pub fn condition(
        &self,
    ) -> Option<&AgentPolicyExpressionReference> {
        self.condition.as_ref()
    }

    /// Return all named arguments in deterministic key order.
    #[must_use]
    pub fn arguments(
        &self,
    ) -> &BTreeMap<String, AgentPolicyArgument> {
        &self.arguments
    }

    /// Return descriptive metadata in deterministic key order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate the complete binding structure.
    ///
    /// This does not resolve the policy or prove that it is applicable.
    pub fn validate(&self) -> AiSecurityPolicyResult<()> {
        self.policy
            .validate()
            .map_err(AiSecurityPolicyError::AgentPolicy)?;

        self.scope.validate()?;

        validate_identifier(
            "AI security policy binding identity",
            self.id.as_str(),
        )?;

        if let Some(condition) = &self.condition {
            validate_expression_reference(condition)?;
        }

        for (key, argument) in &self.arguments {
            validate_identifier(
                "policy argument key",
                key,
            )?;

            validate_identifier(
                "policy argument name",
                argument.name(),
            )?;

            if key != argument.name() {
                return Err(
                    AiSecurityPolicyError::ArgumentKeyMismatch {
                        key: key.clone(),
                        argument: argument.name().to_owned(),
                    },
                );
            }

            validate_expression_reference(argument.value())?;
        }

        validate_metadata(&self.metadata)?;

        Ok(())
    }
}

// ============================================================================
// POLICY SET
// ============================================================================

/// Deterministic collection of AI security policy attachments.
///
/// Bindings are indexed by their local binding identity.
///
/// The collection detects duplicate identities and key mismatches.
///
/// It does not resolve policy conflicts, precedence, inheritance,
/// or authorization decisions.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiSecurityPolicySet {
    bindings: BTreeMap<
        AiSecurityPolicyBindingId,
        AiSecurityPolicyBinding,
    >,
}

/// Deserialization representation for an AI security policy set.
#[derive(Deserialize)]
struct AiSecurityPolicySetData {
    #[serde(default)]
    bindings: BTreeMap<
        AiSecurityPolicyBindingId,
        AiSecurityPolicyBinding,
    >,
}

impl<'de> Deserialize<'de> for AiSecurityPolicySet {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = AiSecurityPolicySetData::deserialize(
            deserializer,
        )?;

        let set = Self {
            bindings: data.bindings,
        };

        set.validate().map_err(de::Error::custom)?;

        Ok(set)
    }
}

impl AiSecurityPolicySet {
    /// Construct an empty policy set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a policy set from existing bindings.
    pub fn from_bindings(
        bindings: impl IntoIterator<Item = AiSecurityPolicyBinding>,
    ) -> AiSecurityPolicyResult<Self> {
        let mut set = Self::new();

        for binding in bindings {
            set.attach(binding)?;
        }

        Ok(set)
    }

    /// Attach one policy binding.
    ///
    /// An existing binding identity cannot be silently replaced.
    pub fn attach(
        &mut self,
        binding: AiSecurityPolicyBinding,
    ) -> AiSecurityPolicyResult<()> {
        binding.validate()?;

        let id = binding.id().clone();

        if self.bindings.contains_key(&id) {
            return Err(
                AiSecurityPolicyError::DuplicateBinding {
                    id: id.to_string(),
                },
            );
        }

        self.bindings.insert(id, binding);

        Ok(())
    }

    /// Detach a policy binding by identity.
    ///
    /// Returns the removed binding if it existed.
    pub fn detach(
        &mut self,
        id: &AiSecurityPolicyBindingId,
    ) -> Option<AiSecurityPolicyBinding> {
        self.bindings.remove(id)
    }

    /// Retrieve a binding by identity.
    #[must_use]
    pub fn get(
        &self,
        id: &AiSecurityPolicyBindingId,
    ) -> Option<&AiSecurityPolicyBinding> {
        self.bindings.get(id)
    }

    /// Iterate over bindings in deterministic identity order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &AiSecurityPolicyBinding> {
        self.bindings.values()
    }

    /// Return the number of attached bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Return whether the set contains no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Validate every binding and collection invariant.
    pub fn validate(&self) -> AiSecurityPolicyResult<()> {
        for (key, binding) in &self.bindings {
            binding.validate()?;

            if key != binding.id() {
                return Err(
                    AiSecurityPolicyError::BindingKeyMismatch {
                        key: key.to_string(),
                        binding: binding.id().to_string(),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by AI security policy attachment operations.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum AiSecurityPolicyError {
    /// An identifier is empty or malformed.
    InvalidIdentifier {
        /// Name of the invalid field.
        field: &'static str,

        /// Supplied invalid value.
        value: String,
    },

    /// An AI authorization action is invalid.
    InvalidAction {
        /// Supplied action.
        value: String,
    },

    /// A policy expression reference is invalid.
    InvalidExpressionReference,

    /// A policy argument name already exists.
    DuplicateArgument {
        /// Duplicate argument name.
        name: String,
    },

    /// A policy binding identity already exists.
    DuplicateBinding {
        /// Duplicate binding identity.
        id: String,
    },

    /// A metadata key already exists.
    DuplicateMetadata {
        /// Duplicate metadata key.
        key: String,
    },

    /// A metadata value contains invalid control characters.
    InvalidMetadataValue {
        /// Metadata key.
        key: String,
    },

    /// The argument map key differs from its internal argument name.
    ArgumentKeyMismatch {
        /// Argument map key.
        key: String,

        /// Argument's own name.
        argument: String,
    },

    /// The policy set key differs from the binding's identity.
    BindingKeyMismatch {
        /// Collection key.
        key: String,

        /// Binding identity.
        binding: String,
    },

    /// An underlying canonical agent-policy reference failed validation.
    AgentPolicy(AgentPolicyError),
}

impl fmt::Display for AiSecurityPolicyError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "invalid {field}: {value:?}"
                )
            }

            Self::InvalidAction { value } => {
                write!(
                    formatter,
                    "invalid AI security action: {value:?}"
                )
            }

            Self::InvalidExpressionReference => {
                formatter.write_str(
                    "invalid AI security policy expression reference",
                )
            }

            Self::DuplicateArgument { name } => {
                write!(
                    formatter,
                    "duplicate AI security policy argument: {name}"
                )
            }

            Self::DuplicateBinding { id } => {
                write!(
                    formatter,
                    "duplicate AI security policy binding: {id}"
                )
            }

            Self::DuplicateMetadata { key } => {
                write!(
                    formatter,
                    "duplicate AI security policy metadata key: {key}"
                )
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "invalid AI security policy metadata value: {key}"
                )
            }

            Self::ArgumentKeyMismatch { key, argument } => {
                write!(
                    formatter,
                    "policy argument key {key:?} does not match its name {argument:?}"
                )
            }

            Self::BindingKeyMismatch { key, binding } => {
                write!(
                    formatter,
                    "policy set key {key:?} does not match binding identity {binding:?}"
                )
            }

            Self::AgentPolicy(error) => {
                write!(
                    formatter,
                    "canonical agent policy reference error: {error}"
                )
            }
        }
    }
}

impl Error for AiSecurityPolicyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AgentPolicy(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::AiAgentId;

    use crate::ai::security::identity::{
        IdentityAuthority,
    };

    fn policy() -> AgentPolicyReference {
        AgentPolicyReference::new(
            "zamani.ai.security",
            "execution",
        )
        .expect("valid policy reference")
    }

    fn principal() -> AiPrincipalRef {
        AiPrincipalRef::agent(
            IdentityAuthority::new("zamani::agents")
                .expect("valid authority"),
            AiAgentId::new(17)
                .expect("valid agent identity"),
        )
    }

    fn scope() -> AiSecurityPolicyScope {
        AiSecurityPolicyScope::new(
            AiSecurityPolicySubject::Principal(principal()),
            AiSecurityPolicyTarget::action(
                "ai.agent.execute",
            )
            .expect("valid AI action"),
        )
        .expect("valid policy scope")
    }

    fn binding(
        id: &str,
    ) -> AiSecurityPolicyBinding {
        AiSecurityPolicyBinding::new(
            AiSecurityPolicyBindingId::new(id)
                .expect("valid binding ID"),
            policy(),
            scope(),
        )
        .expect("valid binding")
    }

    #[test]
    fn binding_id_rejects_empty_values() {
        assert!(
            AiSecurityPolicyBindingId::new("").is_err()
        );
    }

    #[test]
    fn binding_id_rejects_whitespace() {
        assert!(
            AiSecurityPolicyBindingId::new("invalid id").is_err()
        );

        assert!(
            AiSecurityPolicyBindingId::new(" id").is_err()
        );
    }

    #[test]
    fn binding_id_has_no_arbitrary_length_limit() {
        let value = "a".repeat(4096);

        assert!(
            AiSecurityPolicyBindingId::new(value).is_ok()
        );
    }

    #[test]
    fn action_target_requires_ai_namespace() {
        assert!(
            AiSecurityPolicyTarget::action(
                "ai.model.infer",
            )
            .is_ok()
        );

        assert!(
            AiSecurityPolicyTarget::action(
                "distributed.execute",
            )
            .is_err()
        );
    }

    #[test]
    fn action_target_rejects_empty_namespace_segments() {
        assert!(
            AiSecurityPolicyTarget::action(
                "ai..model.infer",
            )
            .is_err()
        );

        assert!(
            AiSecurityPolicyTarget::action(
                "ai.model.",
            )
            .is_err()
        );
    }

    #[test]
    fn broad_subject_must_be_explicit() {
        let broad = AiSecurityPolicyScope::new(
            AiSecurityPolicySubject::AllPrincipals,
            AiSecurityPolicyTarget::action(
                "ai.model.infer",
            )
            .expect("valid action"),
        );

        assert!(broad.is_ok());
    }

    #[test]
    fn broad_target_must_be_explicit() {
        let broad = AiSecurityPolicyScope::new(
            AiSecurityPolicySubject::Principal(principal()),
            AiSecurityPolicyTarget::all_ai_operations(),
        );

        assert!(broad.is_ok());
    }

    #[test]
    fn binding_validates_canonical_policy_reference() {
        let result = AiSecurityPolicyBinding::new(
            AiSecurityPolicyBindingId::new("binding.one")
                .unwrap(),
            policy(),
            scope(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn binding_rejects_invalid_policy_action() {
        let invalid_scope = AiSecurityPolicyScope {
            subject: AiSecurityPolicySubject::Principal(
                principal(),
            ),
            target: AiSecurityPolicyTarget::Action {
                action: "distributed.execute".to_owned(),
            },
        };

        let result = AiSecurityPolicyBinding::new(
            AiSecurityPolicyBindingId::new("binding.invalid")
                .unwrap(),
            policy(),
            invalid_scope,
        );

        assert!(result.is_err());
    }

    #[test]
    fn policy_set_rejects_duplicate_binding_ids() {
        let mut set = AiSecurityPolicySet::new();

        set.attach(binding("binding.one"))
            .expect("first binding");

        assert!(
            matches!(
                set.attach(binding("binding.one")),
                Err(AiSecurityPolicyError::DuplicateBinding { .. })
            )
        );
    }

    #[test]
    fn policy_set_uses_deterministic_identity_order() {
        let mut set = AiSecurityPolicySet::new();

        set.attach(binding("binding.z"))
            .expect("binding z");

        set.attach(binding("binding.a"))
            .expect("binding a");

        let ids: Vec<&str> = set
            .iter()
            .map(|binding| binding.id().as_str())
            .collect();

        assert_eq!(
            ids,
            vec!["binding.a", "binding.z"]
        );
    }

    #[test]
    fn policy_set_can_detach_bindings() {
        let mut set = AiSecurityPolicySet::new();

        let id = AiSecurityPolicyBindingId::new(
            "binding.one",
        )
        .unwrap();

        set.attach(binding("binding.one"))
            .expect("binding");

        assert!(set.detach(&id).is_some());
        assert!(set.is_empty());
    }

    #[test]
    fn policy_binding_serialization_round_trip() {
        let original = binding("binding.serialized");

        let encoded = serde_json::to_string(&original)
            .expect("serialize binding");

        let decoded: AiSecurityPolicyBinding =
            serde_json::from_str(&encoded)
                .expect("deserialize binding");

        assert_eq!(original, decoded);
    }

    #[test]
    fn policy_set_serialization_round_trip() {
        let mut original = AiSecurityPolicySet::new();

        original
            .attach(binding("binding.serialized"))
            .expect("attach binding");

        let encoded = serde_json::to_string(&original)
            .expect("serialize policy set");

        let decoded: AiSecurityPolicySet =
            serde_json::from_str(&encoded)
                .expect("deserialize policy set");

        assert_eq!(original, decoded);
    }

    #[test]
    fn malformed_binding_id_is_rejected_during_deserialization() {
        let result = serde_json::from_str::<
            AiSecurityPolicyBindingId,
        >("\"invalid binding id\"");

        assert!(result.is_err());
    }
}