
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Policy Integration
//!
//! Path: `src/ai/agents/policies.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides portable, declarative policy attachments for AI agents.
//!
//! This module connects agent definitions and agent operations to
//! Zamani's canonical universal policy architecture.
//!
//! It supports:
//!
//! - policy references;
//! - policy binding identities;
//! - agent and operation scopes;
//! - explicit broad policy scopes;
//! - canonical expression references;
//! - parameter bindings;
//! - policy metadata;
//! - multiple policy attachments;
//! - deterministic structural validation.
//!
//! ## Canonical ownership
//!
//! Source grammar:
//!   grammar/ai/agents.g4
//!   grammar/ai/policies.g4
//!
//! Universal policy grammar:
//!   grammar/policies/policy.g4
//!   grammar/expressions/policy.g4
//!
//! Canonical agent identity:
//!   src/ai/ids.rs
//!
//! Agent definition:
//!   src/ai/agents/definition.rs
//!
//! Agent capabilities:
//!   src/ai/agents/capabilities.rs
//!
//! Agent tools:
//!   src/ai/agents/tools.rs
//!
//! Agent planning:
//!   src/ai/agents/planning.rs
//!
//! Agent coordination:
//!   src/ai/agents/coordination.rs
//!
//! AI semantic analysis:
//!   src/frontend/semantic/domains/ai.rs
//!
//! Security enforcement:
//!   src/distributed/security/
//!
//! ## Ownership
//!
//! This module owns declarative agent-policy attachments and their
//! structural integrity.
//!
//! It does not own:
//!
//! - policy source parsing;
//! - AST construction;
//! - universal policy expressions;
//! - universal policy declarations;
//! - policy evaluation;
//! - policy precedence;
//! - policy inheritance;
//! - policy conflict resolution;
//! - authorization;
//! - authentication;
//! - permission enforcement;
//! - capability discovery;
//! - resource feasibility;
//! - execution;
//! - scheduling;
//! - hardware selection;
//! - policy IR.
//!
//! ## Security boundary
//!
//! A policy reference is not authorization.
//!
//! A policy attachment is not permission to execute an operation.
//!
//! Structural validation is not semantic validation.
//!
//! The canonical semantic policy subsystem resolves policy meaning.
//!
//! The authoritative security subsystem enforces authorization.
//!
//! An unresolved, invalid, unavailable or indeterminate policy
//! must never be interpreted as permission.
//!
//! ## Scalability
//!
//! This module introduces no artificial semantic maximum on:
//!
//! - policy attachments;
//! - agents;
//! - policy references;
//! - policy arguments;
//! - policy metadata;
//! - policy targets;
//! - operation scopes.
//!
//! Collection growth is subject to actual memory availability,
//! representation limits and explicitly configured operational budgets.
//!
//! This does not promise physically infinite computation.
//!
//! ## POCO-REAF
//!
//! Agent policy attachments are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - FPGA;
//! - ASIC;
//! - QPU;
//! - neuromorphic hardware;
//! - operating system;
//! - memory topology;
//! - network topology;
//! - distributed node count.
//!
//! Policies express governing intent.
//!
//! Downstream semantic, compiler, security and execution systems
//! determine whether and how that intent can be realized.
//!
//! ## Integration
//!
//! Agent definitions:
//!   src/ai/agents/definition.rs
//!
//! Agent tools:
//!   src/ai/agents/tools.rs
//!
//! Agent planning:
//!   src/ai/agents/planning.rs
//!
//! Agent coordination:
//!   src/ai/agents/coordination.rs
//!
//! Agent capability declarations:
//!   src/ai/agents/capabilities.rs
//!
//! Canonical AI semantics:
//!   src/frontend/semantic/domains/ai.rs
//!
//! Canonical policy grammar:
//!   grammar/policies/policy.g4
//!
//! Canonical policy expression grammar:
//!   grammar/expressions/policy.g4
//!
//! Distributed security enforcement:
//!   src/distributed/security/policy.rs
//!
//! ## Important integration invariant
//!
//! This module stores policy references and bindings.
//!
//! It never creates an independent policy evaluator or authorization
//! mechanism.
//!
//! Semantic and security authorities must independently resolve,
//! validate and enforce applicable policies.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent policy operations.
pub type AgentPolicyResult<T> = Result<T, AgentPolicyError>;

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> AgentPolicyResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
    {
        return Err(AgentPolicyError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_reference(
    field: &'static str,
    value: &str,
) -> AgentPolicyResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(AgentPolicyError::InvalidReference {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> AgentPolicyResult<()> {
    for (key, value) in metadata {
        validate_identifier("metadata key", key)?;

        if value.chars().any(char::is_control) {
            return Err(AgentPolicyError::InvalidMetadataValue {
                key: key.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// POLICY REFERENCE
// ============================================================================

/// An opaque reference to a canonical Zamani policy.
///
/// The universal policy subsystem owns the referenced policy's
/// declaration, expression, directives and semantics.
///
/// This type does not implement policy evaluation.
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
pub struct AgentPolicyReference {
    namespace: String,
    name: String,
}

impl AgentPolicyReference {
    /// Construct a policy reference.
    ///
    /// Namespace and name are logical identifiers.
    ///
    /// They are not physical resource or hardware identifiers.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AgentPolicyResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier("policy namespace", &namespace)?;
        validate_identifier("policy name", &name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the policy namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the policy name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified policy reference.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Validate the reference.
    pub fn validate(&self) -> AgentPolicyResult<()> {
        validate_identifier(
            "policy namespace",
            &self.namespace,
        )?;

        validate_identifier(
            "policy name",
            &self.name,
        )
    }
}

impl fmt::Display for AgentPolicyReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// POLICY BINDING IDENTITY
// ============================================================================

/// Identity of one policy attachment.
///
/// This identity is local to the policy collection in which
/// the binding is stored.
///
/// It does not replace universal ZUIR identifiers.
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
#[serde(transparent)]
pub struct AgentPolicyBindingId(String);

impl AgentPolicyBindingId {
    /// Construct a policy binding identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentPolicyResult<Self> {
        let value = value.into();

        validate_identifier(
            "policy binding identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the binding identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentPolicyBindingId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// POLICY TARGET REFERENCE
// ============================================================================

/// An extensible reference to a logical agent-related target.
///
/// The namespace identifies the semantic target category.
///
/// Examples include:
///
/// - `agent::operation`
/// - `agent::tool`
/// - `agent::plan`
/// - `agent::communication`
/// - `agent::memory`
/// - `agent::coordination`
/// - future registered target categories.
///
/// These examples are not a closed enumeration.
///
/// The canonical semantic subsystem resolves the reference.
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
pub struct AgentPolicyTargetReference {
    namespace: String,
    name: String,
}

impl AgentPolicyTargetReference {
    /// Construct a logical target reference.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AgentPolicyResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier(
            "policy target namespace",
            &namespace,
        )?;

        validate_identifier(
            "policy target name",
            &name,
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the target namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the target name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validate the target reference.
    pub fn validate(&self) -> AgentPolicyResult<()> {
        validate_identifier(
            "policy target namespace",
            &self.namespace,
        )?;

        validate_identifier(
            "policy target name",
            &self.name,
        )
    }
}

// ============================================================================
// POLICY SUBJECT
// ============================================================================

/// Logical subject to which a policy attachment applies.
///
/// Broad applicability must be explicit.
///
/// This type expresses scope only. It does not determine whether
/// the subject is authorized to perform an operation.
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
pub enum AgentPolicySubject {
    /// Apply to one specific agent.
    Agent(AiAgentId),

    /// Apply to a registered logical agent-related target.
    Target(AgentPolicyTargetReference),

    /// Explicitly express applicability to all agents.
    ///
    /// The canonical policy subsystem determines whether such
    /// broad applicability is valid and permitted.
    AllAgents,
}

impl AgentPolicySubject {
    /// Validate the subject's structural representation.
    pub fn validate(&self) -> AgentPolicyResult<()> {
        match self {
            Self::Agent(_) | Self::AllAgents => Ok(()),

            Self::Target(target) => target.validate(),
        }
    }
}

// ============================================================================
// POLICY EXPRESSION REFERENCE
// ============================================================================

/// Opaque reference to a canonical Zamani expression.
///
/// Expressions may originate from the canonical frontend AST.
///
/// This module does not parse, evaluate or reinterpret expressions.
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
#[serde(transparent)]
pub struct AgentPolicyExpressionReference(String);

impl AgentPolicyExpressionReference {
    /// Construct an opaque expression reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentPolicyResult<Self> {
        let value = value.into();

        validate_reference(
            "policy expression reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the unresolved expression reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// POLICY ARGUMENT
// ============================================================================

/// A named argument binding for a canonical policy.
///
/// The value is an opaque reference to a canonical expression.
///
/// Expression evaluation and type checking belong to the frontend
/// semantic and universal policy subsystems.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentPolicyArgument {
    name: String,
    value: AgentPolicyExpressionReference,
}

impl AgentPolicyArgument {
    /// Construct a named policy argument.
    pub fn new(
        name: impl Into<String>,
        value: AgentPolicyExpressionReference,
    ) -> AgentPolicyResult<Self> {
        let name = name.into();

        validate_identifier(
            "policy argument name",
            &name,
        )?;

        Ok(Self {
            name,
            value,
        })
    }

    /// Return the argument name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the expression reference.
    #[must_use]
    pub fn value(
        &self,
    ) -> &AgentPolicyExpressionReference {
        &self.value
    }
}

// ============================================================================
// POLICY BINDING
// ============================================================================

/// A declarative attachment of a canonical policy to an agent subject.
///
/// A binding describes which policy is associated with which
/// logical subject and which expression references are supplied.
///
/// It does not define policy precedence, evaluation, inheritance,
/// authorization or execution.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentPolicyBinding {
    id: AgentPolicyBindingId,

    policy: AgentPolicyReference,

    subject: AgentPolicySubject,

    condition: Option<AgentPolicyExpressionReference>,

    arguments: BTreeMap<String, AgentPolicyArgument>,

    metadata: BTreeMap<String, String>,
}

impl AgentPolicyBinding {
    /// Construct a policy binding.
    pub fn new(
        id: AgentPolicyBindingId,
        policy: AgentPolicyReference,
        subject: AgentPolicySubject,
    ) -> AgentPolicyResult<Self> {
        policy.validate()?;
        subject.validate()?;

        Ok(Self {
            id,
            policy,
            subject,
            condition: None,
            arguments: BTreeMap::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Attach an opaque canonical condition reference.
    ///
    /// The frontend semantic subsystem resolves its meaning.
    pub fn with_condition(
        mut self,
        condition: AgentPolicyExpressionReference,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Add a policy argument.
    ///
    /// Duplicate argument names are rejected.
    pub fn add_argument(
        &mut self,
        argument: AgentPolicyArgument,
    ) -> AgentPolicyResult<()> {
        validate_identifier(
            "policy argument name",
            argument.name(),
        )?;

        if self.arguments.contains_key(argument.name()) {
            return Err(
                AgentPolicyError::DuplicateArgument {
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
    /// Metadata does not change policy semantics.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentPolicyResult<()> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata key", &key)?;

        if value.chars().any(char::is_control) {
            return Err(
                AgentPolicyError::InvalidMetadataValue {
                    key,
                },
            );
        }

        if self.metadata.contains_key(&key) {
            return Err(
                AgentPolicyError::DuplicateMetadata {
                    key,
                },
            );
        }

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Return the binding identity.
    #[must_use]
    pub fn id(&self) -> &AgentPolicyBindingId {
        &self.id
    }

    /// Return the canonical policy reference.
    #[must_use]
    pub fn policy(&self) -> &AgentPolicyReference {
        &self.policy
    }

    /// Return the policy subject.
    #[must_use]
    pub fn subject(&self) -> &AgentPolicySubject {
        &self.subject
    }

    /// Return the optional condition.
    #[must_use]
    pub fn condition(
        &self,
    ) -> Option<&AgentPolicyExpressionReference> {
        self.condition.as_ref()
    }

    /// Return all argument bindings.
    #[must_use]
    pub fn arguments(
        &self,
    ) -> &BTreeMap<String, AgentPolicyArgument> {
        &self.arguments
    }

    /// Return descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate the complete binding structure.
    ///
    /// This does not establish that the referenced policy or
    /// expressions exist or are semantically valid.
    pub fn validate(&self) -> AgentPolicyResult<()> {
        self.policy.validate()?;
        self.subject.validate()?;

        validate_identifier(
            "policy binding identity",
            self.id.as_str(),
        )?;

        for (key, argument) in &self.arguments {
            validate_identifier(
                "policy argument name",
                key,
            )?;

            if key != argument.name() {
                return Err(
                    AgentPolicyError::ArgumentKeyMismatch {
                        key: key.clone(),
                        argument: argument.name().to_owned(),
                    },
                );
            }

            validate_reference(
                "policy argument expression",
                argument.value().as_str(),
            )?;
        }

        validate_metadata(&self.metadata)?;

        if let Some(condition) = &self.condition {
            validate_reference(
                "policy condition",
                condition.as_str(),
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// AGENT POLICY SET
// ============================================================================

/// Collection of declarative policy attachments.
///
/// Bindings are stored by identity in a BTreeMap.
///
/// This provides deterministic enumeration and duplicate detection.
///
/// It does not resolve conflicts or determine which policy takes
/// precedence. Those operations belong to the canonical policy
/// subsystem.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentPolicySet {
    bindings: BTreeMap<
        AgentPolicyBindingId,
        AgentPolicyBinding,
    >,
}

impl AgentPolicySet {
    /// Construct an empty policy collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a policy collection from existing bindings.
    pub fn from_bindings(
        bindings: impl IntoIterator<Item = AgentPolicyBinding>,
    ) -> AgentPolicyResult<Self> {
        let mut result = Self::new();

        for binding in bindings {
            result.attach(binding)?;
        }

        Ok(result)
    }

    /// Attach a policy binding.
    ///
    /// An existing binding identity cannot be silently replaced.
    pub fn attach(
        &mut self,
        binding: AgentPolicyBinding,
    ) -> AgentPolicyResult<()> {
        binding.validate()?;

        let id = binding.id().clone();

        if self.bindings.contains_key(&id) {
            return Err(
                AgentPolicyError::DuplicateBinding {
                    id: id.to_string(),
                },
            );
        }

        self.bindings.insert(id, binding);

        Ok(())
    }

    /// Remove a binding by identity.
    ///
    /// Returns the removed binding when present.
    pub fn detach(
        &mut self,
        id: &AgentPolicyBindingId,
    ) -> Option<AgentPolicyBinding> {
        self.bindings.remove(id)
    }

    /// Find a binding by identity.
    #[must_use]
    pub fn get(
        &self,
        id: &AgentPolicyBindingId,
    ) -> Option<&AgentPolicyBinding> {
        self.bindings.get(id)
    }

    /// Return all bindings in deterministic identity order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &AgentPolicyBinding> {
        self.bindings.values()
    }

    /// Return the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Return whether the collection contains no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Validate every binding and its collection invariants.
    pub fn validate(&self) -> AgentPolicyResult<()> {
        for (key, binding) in &self.bindings {
            binding.validate()?;

            if key != binding.id() {
                return Err(
                    AgentPolicyError::BindingKeyMismatch {
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

/// Errors produced by agent policy declarations and validation.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum AgentPolicyError {
    /// An identifier is invalid.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,

        /// Invalid value.
        value: String,
    },

    /// An opaque reference is invalid.
    InvalidReference {
        /// Reference field.
        field: &'static str,

        /// Invalid value.
        value: String,
    },

    /// A policy binding already exists.
    DuplicateBinding {
        /// Duplicate binding identity.
        id: String,
    },

    /// A policy argument already exists.
    DuplicateArgument {
        /// Duplicate argument name.
        name: String,
    },

    /// Metadata already exists.
    DuplicateMetadata {
        /// Duplicate metadata key.
        key: String,
    },

    /// Metadata contains an invalid value.
    InvalidMetadataValue {
        /// Metadata key.
        key: String,
    },

    /// A binding argument key differs from its internal name.
    ArgumentKeyMismatch {
        /// Collection key.
        key: String,

        /// Argument name.
        argument: String,
    },

    /// A binding map key differs from the binding identity.
    BindingKeyMismatch {
        /// Collection key.
        key: String,

        /// Binding identity.
        binding: String,
    },
}

impl fmt::Display for AgentPolicyError {
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

            Self::InvalidReference { field, value } => {
                write!(
                    formatter,
                    "invalid {field} reference: {value:?}"
                )
            }

            Self::DuplicateBinding { id } => {
                write!(
                    formatter,
                    "duplicate agent policy binding: {id}"
                )
            }

            Self::DuplicateArgument { name } => {
                write!(
                    formatter,
                    "duplicate agent policy argument: {name}"
                )
            }

            Self::DuplicateMetadata { key } => {
                write!(
                    formatter,
                    "duplicate agent policy metadata key: {key}"
                )
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "invalid value for agent policy metadata: {key}"
                )
            }

            Self::ArgumentKeyMismatch { key, argument } => {
                write!(
                    formatter,
                    "policy argument key {key:?} does not match internal name {argument:?}"
                )
            }

            Self::BindingKeyMismatch { key, binding } => {
                write!(
                    formatter,
                    "policy binding key {key:?} does not match binding identity {binding:?}"
                )
            }
        }
    }
}

impl Error for AgentPolicyError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> AgentPolicyReference {
        AgentPolicyReference::new(
            "zamani.agent",
            "execution",
        )
        .expect("valid policy reference")
    }

    fn binding(
        id: &str,
    ) -> AgentPolicyBinding {
        AgentPolicyBinding::new(
            AgentPolicyBindingId::new(id)
                .expect("valid binding identity"),
            policy(),
            AgentPolicySubject::AllAgents,
        )
        .expect("valid binding")
    }

    #[test]
    fn policy_reference_is_validated() {
        assert!(
            AgentPolicyReference::new(
                "zamani.agent",
                "execution",
            )
            .is_ok()
        );

        assert!(
            AgentPolicyReference::new(
                "",
                "execution",
            )
            .is_err()
        );

        assert!(
            AgentPolicyReference::new(
                "zamani.agent",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn policy_reference_is_qualified() {
        let reference = policy();

        assert_eq!(
            reference.qualified_name(),
            "zamani.agent::execution"
        );
    }

    #[test]
    fn binding_identity_is_validated() {
        assert!(
            AgentPolicyBindingId::new("execution_binding")
                .is_ok()
        );

        assert!(
            AgentPolicyBindingId::new("")
                .is_err()
        );
    }

    #[test]
    fn target_reference_is_open_world() {
        let target = AgentPolicyTargetReference::new(
            "future.agent.domain",
            "custom_operation",
        )
        .expect("valid target");

        assert_eq!(
            target.namespace(),
            "future.agent.domain"
        );

        assert_eq!(
            target.name(),
            "custom_operation"
        );
    }

    #[test]
    fn broad_scope_must_be_explicit() {
        let subject = AgentPolicySubject::AllAgents;

        assert!(subject.validate().is_ok());
    }

    #[test]
    fn target_scope_is_validated() {
        let subject = AgentPolicySubject::Target(
            AgentPolicyTargetReference::new(
                "agent",
                "tool",
            )
            .expect("valid target"),
        );

        assert!(subject.validate().is_ok());
    }

    #[test]
    fn policy_binding_is_constructed() {
        let value = binding("binding_one");

        assert_eq!(
            value.id().as_str(),
            "binding_one"
        );

        assert_eq!(
            value.policy().name(),
            "execution"
        );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn policy_condition_is_preserved() {
        let condition =
            AgentPolicyExpressionReference::new(
                "semantic.expression.condition",
            )
            .expect("valid expression");

        let value = binding("binding_condition")
            .with_condition(condition.clone());

        assert_eq!(
            value.condition(),
            Some(&condition)
        );
    }

    #[test]
    fn duplicate_arguments_are_rejected() {
        let mut value = binding("binding_arguments");

        let first = AgentPolicyArgument::new(
            "context",
            AgentPolicyExpressionReference::new(
                "expression.context",
            )
            .expect("valid expression"),
        )
        .expect("valid argument");

        let second = AgentPolicyArgument::new(
            "context",
            AgentPolicyExpressionReference::new(
                "expression.other",
            )
            .expect("valid expression"),
        )
        .expect("valid argument");

        assert!(value.add_argument(first).is_ok());

        assert!(matches!(
            value.add_argument(second),
            Err(AgentPolicyError::DuplicateArgument { .. })
        ));
    }

    #[test]
    fn duplicate_metadata_is_rejected() {
        let mut value = binding("binding_metadata");

        assert!(
            value.add_metadata("owner", "agent").is_ok()
        );

        assert!(matches!(
            value.add_metadata("owner", "another"),
            Err(AgentPolicyError::DuplicateMetadata { .. })
        ));
    }

    #[test]
    fn duplicate_bindings_are_rejected() {
        let mut policies = AgentPolicySet::new();

        assert!(
            policies.attach(binding("first")).is_ok()
        );

        assert!(matches!(
            policies.attach(binding("first")),
            Err(AgentPolicyError::DuplicateBinding { .. })
        ));
    }

    #[test]
    fn distinct_bindings_can_reference_the_same_policy() {
        let mut policies = AgentPolicySet::new();

        assert!(
            policies.attach(binding("first")).is_ok()
        );

        assert!(
            policies.attach(binding("second")).is_ok()
        );

        assert_eq!(policies.len(), 2);
    }

    #[test]
    fn policy_collection_has_deterministic_order() {
        let mut policies = AgentPolicySet::new();

        policies.attach(binding("zeta")).unwrap();
        policies.attach(binding("alpha")).unwrap();
        policies.attach(binding("middle")).unwrap();

        let identities: Vec<_> = policies
            .iter()
            .map(|value| value.id().as_str())
            .collect();

        assert_eq!(
            identities,
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn policy_can_be_detached() {
        let mut policies = AgentPolicySet::new();

        let value = binding("removable");
        let id = value.id().clone();

        policies.attach(value).unwrap();

        assert!(policies.get(&id).is_some());

        assert!(policies.detach(&id).is_some());

        assert!(policies.get(&id).is_none());
    }

    #[test]
    fn empty_policy_set_is_valid() {
        let policies = AgentPolicySet::new();

        assert!(policies.is_empty());
        assert_eq!(policies.len(), 0);
        assert!(policies.validate().is_ok());
    }

    #[test]
    fn policy_collection_validates_all_bindings() {
        let policies = AgentPolicySet::from_bindings([
            binding("one"),
            binding("two"),
            binding("three"),
        ])
        .expect("valid policy collection");

        assert_eq!(policies.len(), 3);
        assert!(policies.validate().is_ok());
    }

    #[test]
    fn expression_references_reject_empty_values() {
        assert!(
            AgentPolicyExpressionReference::new("")
                .is_err()
        );
    }

    #[test]
    fn metadata_rejects_control_characters() {
        let mut value = binding("metadata_validation");

        assert!(
            value.add_metadata(
                "description",
                "invalid\nvalue",
            )
            .is_err()
        );
    }
}
