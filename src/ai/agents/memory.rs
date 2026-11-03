
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Memory Contracts
//!
//! Path: `src/ai/agents/memory.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, extensible memory contracts for AI agents.
//!
//! This module describes how an agent declares memory requirements and
//! expresses memory operations. It does not implement persistent storage,
//! search engines, databases, distributed communication, or execution.
//!
//! ## Canonical ownership
//!
//! - `grammar/ai/agents.g4`: agent source syntax.
//! - `grammar/ai/ai.g4`: AI grammar composition.
//! - `src/ai/ids.rs`: canonical AI identities.
//! - `src/ai/agents/definition.rs`: agent definitions.
//! - `src/ai/agents/capabilities.rs`: agent capabilities.
//! - `src/ai/agents/coordination.rs`: agent coordination.
//! - `src/ai/agents/communication.rs`: agent communication.
//! - `src/ai/agents/planning.rs`: agent planning.
//! - `src/ai/provenance.rs`: AI provenance integration.
//! - `src/frontend/semantic/`: canonical semantic authority.
//! - `src/runtime/sankofa/`: existing memory and knowledge infrastructure.
//! - `src/distributed/`: distributed storage and communication integration.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Agent-specific memory declarations.
//! - Open-world memory categories.
//! - Memory access permissions as declarative contracts.
//! - Memory references.
//! - Memory request and response contracts.
//! - Memory record metadata.
//! - The asynchronous memory-provider interface.
//! - Structural validation of memory requests and profiles.
//!
//! This module does not own:
//!
//! - Persistent memory storage.
//! - Database implementation.
//! - Knowledge graph storage.
//! - Content-addressing algorithms.
//! - Cryptographic integrity verification.
//! - Authentication or authorization enforcement.
//! - Agent coordination.
//! - Message transport.
//! - Distributed placement or scheduling.
//! - Runtime memory allocation.
//! - Source parsing or AST construction.
//! - Semantic name resolution.
//! - Universal IR.
//! - Model execution.
//!
//! ## POCO-REAF
//!
//! Memory contracts are independent of physical execution targets.
//!
//! An agent may use local, remote, replicated, embedded, distributed,
//! persistent, transient, or future memory providers without changing
//! its source-level memory intent.
//!
//! No fixed maximum is imposed on agents, memory declarations, records,
//! metadata entries, memory categories, or provider implementations.
//!
//! Operational limits such as query budgets, retention limits, storage
//! quotas, and deadlines belong to provider configuration and execution
//! policy. They are not language-level semantic ceilings.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Dependency direction
//!
//!     Canonical AST
//!          |
//!          v
//!     Semantic validation
//!          |
//!          v
//!     Agent memory contracts
//!          |
//!          v
//!     Runtime memory-provider adapter
//!          |
//!          +--> Sankofa
//!          +--> external memory providers
//!          +--> local or remote storage
//!          +--> future memory systems
//!
//! This module does not depend on runtime implementations.
//!
//! ## Trust boundary
//!
//! A memory reference is an opaque identifier, not proof of existence,
//! ownership, authenticity, authorization, or integrity.
//!
//! A provider must enforce its own access controls and validate the
//! authority of the requesting execution context.
//!
//! Metadata and provenance references are descriptive claims until
//! checked by their authoritative subsystems.

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::ai::ids::AiAgentId;

/// Result type for agent-memory operations.
pub type AgentMemoryResult<T> = Result<T, AgentMemoryError>;

/// Boxed future returned by an agent-memory provider.
///
/// The future is not required to be `Send`. This permits providers to
/// support single-threaded, embedded, local, and asynchronous execution
/// environments. An execution adapter may impose stronger requirements
/// when its runtime needs them.
pub type AgentMemoryFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + 'a>>;

/// Validates an opaque memory-domain identifier.
///
/// Values are preserved exactly. The function does not normalize names,
/// interpret namespaces, or resolve references.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> AgentMemoryResult<()> {
    if value.is_empty()
        || value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(AgentMemoryError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Defines a validated, open-world string reference.
///
/// Deserialization passes through the same validation as ordinary
/// construction. No fixed length or category limit is imposed.
macro_rules! define_memory_reference {
    (
        $(#[$meta:meta])*
        $name:ident,
        $field:literal
    ) => {
        $(#[$meta])*
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
        pub struct $name(String);

        impl $name {
            /// Creates a validated reference.
            pub fn new(
                value: impl Into<String>,
            ) -> AgentMemoryResult<Self> {
                let value = value.into();
                validate_identifier($field, &value)?;
                Ok(Self(value))
            }

            /// Returns the reference without interpreting it.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the reference and returns its underlying value.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = AgentMemoryError;

            fn try_from(value: String) -> AgentMemoryResult<Self> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

define_memory_reference!(
    /// Open-world name of an agent memory declaration.
    AgentMemoryName,
    "agent memory name"
);

define_memory_reference!(
    /// Open-world memory category.
    ///
    /// Examples include `working`, `episodic`, `semantic`,
    /// `procedural`, or extension-defined categories.
    AgentMemoryKind,
    "agent memory kind"
);

define_memory_reference!(
    /// Opaque reference to a memory resource or provider-owned namespace.
    ///
    /// The reference does not imply a physical location or storage format.
    AgentMemoryReference,
    "agent memory reference"
);

define_memory_reference!(
    /// Opaque reference to a memory value.
    ///
    /// Actual content and its storage representation are owned by
    /// the selected memory provider.
    AgentMemoryValueReference,
    "agent memory value reference"
);

define_memory_reference!(
    /// Opaque reference to a canonical expression or query.
    ///
    /// The frontend owns expression parsing and semantic validation.
    AgentMemoryExpressionReference,
    "agent memory expression reference"
);

define_memory_reference!(
    /// Opaque reference to canonical provenance information.
    AgentMemoryProvenanceReference,
    "agent memory provenance reference"
);

define_memory_reference!(
    /// Opaque provider-defined continuation token.
    ///
    /// The token is not interpreted by this module.
    AgentMemoryContinuation,
    "agent memory continuation"
);

/// Permission declared for an agent memory resource.
///
/// These values describe requested access. They do not authorize access.
/// Authorization is enforced by the security and runtime subsystems.
#[derive(
    Clone,
    Copy,
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
pub enum AgentMemoryAccess {
    /// Permit reading records.
    Read,

    /// Permit creating or updating records.
    Write,

    /// Permit searching records.
    Search,

    /// Permit requesting deletion or retraction.
    Forget,
}

/// Declarative memory resource contract for one agent.
///
/// A declaration describes the logical memory resource and its allowed
/// access operations. It does not allocate or open that resource.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMemoryDeclaration {
    name: AgentMemoryName,
    kind: AgentMemoryKind,
    reference: AgentMemoryReference,
    access: BTreeSet<AgentMemoryAccess>,
    description: Option<String>,
    metadata: BTreeMap<String, String>,
}

impl AgentMemoryDeclaration {
    /// Creates a memory declaration.
    pub fn new(
        name: AgentMemoryName,
        kind: AgentMemoryKind,
        reference: AgentMemoryReference,
    ) -> Self {
        Self {
            name,
            kind,
            reference,
            access: BTreeSet::new(),
            description: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a declared access permission.
    #[must_use]
    pub fn with_access(
        mut self,
        access: AgentMemoryAccess,
    ) -> Self {
        self.access.insert(access);
        self
    }

    /// Replaces the declared access permissions.
    #[must_use]
    pub fn with_access_set(
        mut self,
        access: BTreeSet<AgentMemoryAccess>,
    ) -> Self {
        self.access = access;
        self
    }

    /// Adds a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AgentMemoryResult<Self> {
        let description = description.into();

        if description.chars().any(char::is_control) {
            return Err(AgentMemoryError::InvalidDescription);
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Adds provider-neutral metadata.
    ///
    /// Existing keys are replaced. The map provides deterministic ordering.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentMemoryResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_identifier("memory metadata key", &key)?;

        if value.chars().any(char::is_control) {
            return Err(AgentMemoryError::InvalidMetadataValue {
                key,
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Returns the declaration name.
    #[must_use]
    pub fn name(&self) -> &AgentMemoryName {
        &self.name
    }

    /// Returns the open-world memory category.
    #[must_use]
    pub fn kind(&self) -> &AgentMemoryKind {
        &self.kind
    }

    /// Returns the opaque resource reference.
    #[must_use]
    pub fn reference(&self) -> &AgentMemoryReference {
        &self.reference
    }

    /// Returns the declared access permissions.
    #[must_use]
    pub fn access(&self) -> &BTreeSet<AgentMemoryAccess> {
        &self.access
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validates the declaration's structural invariants.
    pub fn validate(&self) -> AgentMemoryResult<()> {
        validate_identifier("agent memory name", self.name.as_str())?;
        validate_identifier("agent memory kind", self.kind.as_str())?;
        validate_identifier(
            "agent memory reference",
            self.reference.as_str(),
        )?;

        if let Some(description) = &self.description {
            if description.chars().any(char::is_control) {
                return Err(AgentMemoryError::InvalidDescription);
            }
        }

        for (key, value) in &self.metadata {
            validate_identifier("memory metadata key", key)?;

            if value.chars().any(char::is_control) {
                return Err(AgentMemoryError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Declarative memory profile belonging to one agent.
///
/// The profile is a collection of logical declarations. It does not own
/// the agent definition, agent lifecycle, or memory provider.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMemoryProfile {
    agent_id: AiAgentId,
    declarations: BTreeMap<AgentMemoryName, AgentMemoryDeclaration>,
}

impl AgentMemoryProfile {
    /// Creates an empty profile for an existing agent identity.
    #[must_use]
    pub fn new(agent_id: AiAgentId) -> Self {
        Self {
            agent_id,
            declarations: BTreeMap::new(),
        }
    }

    /// Returns the owning agent identity.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Returns all declarations in deterministic name order.
    #[must_use]
    pub fn declarations(
        &self,
    ) -> &BTreeMap<AgentMemoryName, AgentMemoryDeclaration> {
        &self.declarations
    }

    /// Returns the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.declarations.len()
    }

    /// Returns whether the profile has no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// Adds a declaration.
    ///
    /// Duplicate declaration names are rejected. The profile is unchanged
    /// when insertion fails.
    pub fn add_declaration(
        &mut self,
        declaration: AgentMemoryDeclaration,
    ) -> AgentMemoryResult<()> {
        declaration.validate()?;

        let name = declaration.name().clone();

        if self.declarations.contains_key(&name) {
            return Err(AgentMemoryError::DuplicateDeclaration {
                name: name.to_string(),
            });
        }

        self.declarations.insert(name, declaration);
        Ok(())
    }

    /// Finds a declaration by its exact name.
    #[must_use]
    pub fn find_declaration(
        &self,
        name: &AgentMemoryName,
    ) -> Option<&AgentMemoryDeclaration> {
        self.declarations.get(name)
    }

    /// Removes a declaration by its exact name.
    pub fn remove_declaration(
        &mut self,
        name: &AgentMemoryName,
    ) -> Option<AgentMemoryDeclaration> {
        self.declarations.remove(name)
    }

    /// Validates every declaration and checks key consistency.
    pub fn validate(&self) -> AgentMemoryResult<()> {
        for (key, declaration) in &self.declarations {
            declaration.validate()?;

            if key != declaration.name() {
                return Err(AgentMemoryError::DeclarationKeyMismatch {
                    key: key.to_string(),
                    declaration: declaration.name().to_string(),
                });
            }
        }

        Ok(())
    }
}

/// Requested memory operation.
///
/// This describes intent only. The provider determines how the operation
/// is implemented and whether the request is feasible and authorized.
#[derive(
    Clone,
    Copy,
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
pub enum AgentMemoryOperation {
    /// Retrieve a record by reference.
    Read,

    /// Store or update a value.
    Write,

    /// Search using a canonical expression reference.
    Search,

    /// Request deletion or logical retraction.
    Forget,
}

/// Declarative request to a memory provider.
///
/// Fields are optional at construction time so adapters can build requests
/// incrementally. `validate` enforces the fields required by each operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMemoryRequest {
    agent_id: AiAgentId,
    operation: AgentMemoryOperation,
    memory: AgentMemoryReference,
    target: Option<AgentMemoryValueReference>,
    expression: Option<AgentMemoryExpressionReference>,
    value: Option<AgentMemoryValueReference>,
    provenance: Option<AgentMemoryProvenanceReference>,
    continuation: Option<AgentMemoryContinuation>,
    result_limit: Option<NonZeroUsize>,
    metadata: BTreeMap<String, String>,
}

impl AgentMemoryRequest {
    /// Creates a request for an agent and a logical memory resource.
    #[must_use]
    pub fn new(
        agent_id: AiAgentId,
        operation: AgentMemoryOperation,
        memory: AgentMemoryReference,
    ) -> Self {
        Self {
            agent_id,
            operation,
            memory,
            target: None,
            expression: None,
            value: None,
            provenance: None,
            continuation: None,
            result_limit: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Sets the target record reference.
    #[must_use]
    pub fn with_target(
        mut self,
        target: AgentMemoryValueReference,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the canonical expression reference used for search.
    #[must_use]
    pub fn with_expression(
        mut self,
        expression: AgentMemoryExpressionReference,
    ) -> Self {
        self.expression = Some(expression);
        self
    }

    /// Sets the value reference used by a write operation.
    #[must_use]
    pub fn with_value(
        mut self,
        value: AgentMemoryValueReference,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Attaches a provenance reference.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AgentMemoryProvenanceReference,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Sets a provider-defined continuation token.
    #[must_use]
    pub fn with_continuation(
        mut self,
        continuation: AgentMemoryContinuation,
    ) -> Self {
        self.continuation = Some(continuation);
        self
    }

    /// Sets an optional operational result limit.
    ///
    /// This is a request-level budget, not a semantic maximum. The provider
    /// may reject or further constrain it according to its own policy.
    #[must_use]
    pub fn with_result_limit(
        mut self,
        result_limit: NonZeroUsize,
    ) -> Self {
        self.result_limit = Some(result_limit);
        self
    }

    /// Adds provider-neutral request metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentMemoryResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_identifier("request metadata key", &key)?;

        if value.chars().any(char::is_control) {
            return Err(AgentMemoryError::InvalidMetadataValue {
                key,
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Returns the requesting agent identity.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Returns the requested operation.
    #[must_use]
    pub const fn operation(&self) -> AgentMemoryOperation {
        self.operation
    }

    /// Returns the logical memory reference.
    #[must_use]
    pub fn memory(&self) -> &AgentMemoryReference {
        &self.memory
    }

    /// Returns the optional target record.
    #[must_use]
    pub fn target(&self) -> Option<&AgentMemoryValueReference> {
        self.target.as_ref()
    }

    /// Returns the optional expression reference.
    #[must_use]
    pub fn expression(
        &self,
    ) -> Option<&AgentMemoryExpressionReference> {
        self.expression.as_ref()
    }

    /// Returns the optional value reference.
    #[must_use]
    pub fn value(&self) -> Option<&AgentMemoryValueReference> {
        self.value.as_ref()
    }

    /// Returns the optional provenance reference.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&AgentMemoryProvenanceReference> {
        self.provenance.as_ref()
    }

    /// Returns the optional continuation token.
    #[must_use]
    pub fn continuation(&self) -> Option<&AgentMemoryContinuation> {
        self.continuation.as_ref()
    }

    /// Returns the optional result limit.
    #[must_use]
    pub const fn result_limit(&self) -> Option<NonZeroUsize> {
        self.result_limit
    }

    /// Returns deterministic request metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validates operation-specific request requirements.
    pub fn validate(&self) -> AgentMemoryResult<()> {
        validate_identifier(
            "agent memory reference",
            self.memory.as_str(),
        )?;

        for (key, value) in &self.metadata {
            validate_identifier("request metadata key", key)?;

            if value.chars().any(char::is_control) {
                return Err(AgentMemoryError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        match self.operation {
            AgentMemoryOperation::Read
            | AgentMemoryOperation::Forget => {
                if self.target.is_none() {
                    return Err(
                        AgentMemoryError::MissingTargetReference {
                            operation: self.operation,
                        },
                    );
                }
            }

            AgentMemoryOperation::Write => {
                if self.value.is_none() {
                    return Err(
                        AgentMemoryError::MissingValueReference,
                    );
                }
            }

            AgentMemoryOperation::Search => {
                if self.expression.is_none() {
                    return Err(
                        AgentMemoryError::MissingExpressionReference,
                    );
                }
            }
        }

        Ok(())
    }
}

/// A provider-returned logical memory record.
///
/// The record contains references, not the underlying payload. The provider
/// remains responsible for payload storage, integrity, access control,
/// versioning, and persistence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMemoryRecord {
    agent_id: AiAgentId,
    memory: AgentMemoryReference,
    reference: AgentMemoryValueReference,
    kind: AgentMemoryKind,
    provenance: Option<AgentMemoryProvenanceReference>,
    metadata: BTreeMap<String, String>,
}

impl AgentMemoryRecord {
    /// Creates a record description returned by a provider.
    #[must_use]
    pub fn new(
        agent_id: AiAgentId,
        memory: AgentMemoryReference,
        reference: AgentMemoryValueReference,
        kind: AgentMemoryKind,
    ) -> Self {
        Self {
            agent_id,
            memory,
            reference,
            kind,
            provenance: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Attaches a provenance reference.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AgentMemoryProvenanceReference,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Returns the owning agent identity.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Returns the logical memory resource.
    #[must_use]
    pub fn memory(&self) -> &AgentMemoryReference {
        &self.memory
    }

    /// Returns the opaque record reference.
    #[must_use]
    pub fn reference(&self) -> &AgentMemoryValueReference {
        &self.reference
    }

    /// Returns the memory category.
    #[must_use]
    pub fn kind(&self) -> &AgentMemoryKind {
        &self.kind
    }

    /// Returns the optional provenance reference.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&AgentMemoryProvenanceReference> {
        self.provenance.as_ref()
    }

    /// Returns record metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Adds validated record metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentMemoryResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_identifier("record metadata key", &key)?;

        if value.chars().any(char::is_control) {
            return Err(AgentMemoryError::InvalidMetadataValue {
                key,
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Validates the record's structural references and metadata.
    pub fn validate(&self) -> AgentMemoryResult<()> {
        validate_identifier("agent memory reference", self.memory.as_str())?;
        validate_identifier(
            "agent memory value reference",
            self.reference.as_str(),
        )?;
        validate_identifier("agent memory kind", self.kind.as_str())?;

        for (key, value) in &self.metadata {
            validate_identifier("record metadata key", key)?;

            if value.chars().any(char::is_control) {
                return Err(AgentMemoryError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Result returned by a memory provider.
///
/// `records` may be empty. A continuation token indicates that the provider
/// has additional results available under its own consistency and pagination
/// rules.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMemoryResponse {
    records: Vec<AgentMemoryRecord>,
    continuation: Option<AgentMemoryContinuation>,
}

impl AgentMemoryResponse {
    /// Creates a response from provider-returned records.
    #[must_use]
    pub fn new(records: Vec<AgentMemoryRecord>) -> Self {
        Self {
            records,
            continuation: None,
        }
    }

    /// Attaches a continuation token.
    #[must_use]
    pub fn with_continuation(
        mut self,
        continuation: AgentMemoryContinuation,
    ) -> Self {
        self.continuation = Some(continuation);
        self
    }

    /// Returns the records in provider-defined result order.
    #[must_use]
    pub fn records(&self) -> &[AgentMemoryRecord] {
        &self.records
    }

    /// Returns the optional continuation token.
    #[must_use]
    pub fn continuation(&self) -> Option<&AgentMemoryContinuation> {
        self.continuation.as_ref()
    }

    /// Returns whether the response contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns the number of returned records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Validates all returned records.
    pub fn validate(&self) -> AgentMemoryResult<()> {
        for record in &self.records {
            record.validate()?;
        }

        Ok(())
    }
}

/// Provider-neutral interface for executing agent memory requests.
///
/// Implementations belong to runtime or integration modules. Examples
/// include adapters for Sankofa, embedded storage, remote memory services,
/// replicated stores, or future provider implementations.
///
/// The trait does not prescribe a database, transport, persistence model,
/// consistency model, or physical location.
///
/// Implementations must not treat a structurally valid request as proof
/// that the caller is authorized.
pub trait AgentMemoryProvider {
    /// Submits one validated memory request.
    ///
    /// The caller should validate the request before submission. Providers
    /// must still validate all inputs at their trust boundary.
    fn submit<'a>(
        &'a self,
        request: &'a AgentMemoryRequest,
    ) -> AgentMemoryFuture<'a, AgentMemoryResult<AgentMemoryResponse>>;
}

/// Structured errors for agent memory contracts and provider boundaries.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AgentMemoryError {
    /// An identifier was empty, whitespace-padded, or contained controls.
    InvalidIdentifier {
        /// Identifier field name.
        field: &'static str,
        /// Rejected value.
        value: String,
    },

    /// A memory declaration name already exists in the profile.
    DuplicateDeclaration {
        /// Duplicate declaration name.
        name: String,
    },

    /// A declaration map key differs from its contained declaration name.
    DeclarationKeyMismatch {
        /// Map key.
        key: String,
        /// Declaration name.
        declaration: String,
    },

    /// A description contained forbidden control characters.
    InvalidDescription,

    /// A metadata value contained forbidden control characters.
    InvalidMetadataValue {
        /// Metadata key.
        key: String,
    },

    /// A read or forget request omitted its target reference.
    MissingTargetReference {
        /// Operation requiring a target.
        operation: AgentMemoryOperation,
    },

    /// A write request omitted its value reference.
    MissingValueReference,

    /// A search request omitted its expression reference.
    MissingExpressionReference,

    /// A provider rejected a request.
    ProviderRejected {
        /// Provider-neutral explanation.
        reason: String,
    },

    /// A provider reported an unavailable resource.
    ResourceUnavailable {
        /// Opaque resource reference.
        memory: String,
    },

    /// A provider reported an integrity or consistency failure.
    ProviderIntegrityFailure {
        /// Provider-neutral explanation.
        reason: String,
    },
}

impl fmt::Display for AgentMemoryError {
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

            Self::DuplicateDeclaration { name } => {
                write!(
                    formatter,
                    "duplicate agent memory declaration: {name}"
                )
            }

            Self::DeclarationKeyMismatch {
                key,
                declaration,
            } => {
                write!(
                    formatter,
                    "agent memory declaration key {key:?} \
                     does not match contained name {declaration:?}"
                )
            }

            Self::InvalidDescription => {
                formatter.write_str(
                    "agent memory description contains control characters",
                )
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "agent memory metadata value for {key:?} \
                     contains control characters"
                )
            }

            Self::MissingTargetReference { operation } => {
                write!(
                    formatter,
                    "memory operation {operation:?} requires a target reference"
                )
            }

            Self::MissingValueReference => {
                formatter.write_str(
                    "memory write requires a value reference",
                )
            }

            Self::MissingExpressionReference => {
                formatter.write_str(
                    "memory search requires an expression reference",
                )
            }

            Self::ProviderRejected { reason } => {
                write!(formatter, "memory provider rejected request: {reason}")
            }

            Self::ResourceUnavailable { memory } => {
                write!(
                    formatter,
                    "agent memory resource is unavailable: {memory}"
                )
            }

            Self::ProviderIntegrityFailure { reason } => {
                write!(
                    formatter,
                    "agent memory provider integrity failure: {reason}"
                )
            }
        }
    }
}

impl Error for AgentMemoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_id(value: u128) -> AiAgentId {
        AiAgentId::new(value).expect("nonzero agent identity")
    }

    fn memory_name(value: &str) -> AgentMemoryName {
        AgentMemoryName::new(value).expect("valid memory name")
    }

    fn memory_kind(value: &str) -> AgentMemoryKind {
        AgentMemoryKind::new(value).expect("valid memory kind")
    }

    fn memory_reference(value: &str) -> AgentMemoryReference {
        AgentMemoryReference::new(value).expect("valid memory reference")
    }

    fn value_reference(value: &str) -> AgentMemoryValueReference {
        AgentMemoryValueReference::new(value)
            .expect("valid value reference")
    }

    #[test]
    fn creates_an_agent_memory_profile() {
        let profile = AgentMemoryProfile::new(agent_id(1));

        assert_eq!(profile.agent_id(), agent_id(1));
        assert!(profile.is_empty());
        assert_eq!(profile.len(), 0);
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn creates_an_extensible_memory_declaration() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("research_state"),
            memory_kind("custom.vendor.episodic"),
            memory_reference("provider://research/state"),
        )
        .with_access(AgentMemoryAccess::Read)
        .with_access(AgentMemoryAccess::Write);

        assert_eq!(declaration.kind().as_str(), "custom.vendor.episodic");
        assert!(declaration.access().contains(&AgentMemoryAccess::Read));
        assert!(declaration.access().contains(&AgentMemoryAccess::Write));
        assert!(declaration.validate().is_ok());
    }

    #[test]
    fn rejects_empty_memory_identifiers() {
        assert!(AgentMemoryName::new("").is_err());
        assert!(AgentMemoryKind::new("   ").is_err());
        assert!(AgentMemoryReference::new("").is_err());
        assert!(AgentMemoryValueReference::new("\n").is_err());
    }

    #[test]
    fn preserves_identifier_values_without_normalization() {
        assert!(AgentMemoryName::new(" research ").is_err());

        let name = memory_name("research");

        assert_eq!(name.as_str(), "research");
    }

    #[test]
    fn supports_open_world_memory_categories() {
        let first = memory_kind("episodic");
        let second = memory_kind("vendor.extension.temporal_graph");

        assert_ne!(first, second);
        assert_eq!(
            second.as_str(),
            "vendor.extension.temporal_graph"
        );
    }

    #[test]
    fn profile_rejects_duplicate_declaration_names() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("state"),
            memory_kind("working"),
            memory_reference("memory://state"),
        );

        let mut profile = AgentMemoryProfile::new(agent_id(2));

        profile
            .add_declaration(declaration.clone())
            .expect("first declaration");

        assert!(matches!(
            profile.add_declaration(declaration),
            Err(AgentMemoryError::DuplicateDeclaration { .. })
        ));

        assert_eq!(profile.len(), 1);
    }

    #[test]
    fn profile_lookup_and_removal_use_exact_names() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("history"),
            memory_kind("episodic"),
            memory_reference("memory://history"),
        );

        let mut profile = AgentMemoryProfile::new(agent_id(3));

        profile
            .add_declaration(declaration)
            .expect("valid declaration");

        let name = memory_name("history");

        assert!(profile.find_declaration(&name).is_some());
        assert!(profile.remove_declaration(&name).is_some());
        assert!(profile.find_declaration(&name).is_none());
    }

    #[test]
    fn request_validation_requires_read_target() {
        let request = AgentMemoryRequest::new(
            agent_id(4),
            AgentMemoryOperation::Read,
            memory_reference("memory://state"),
        );

        assert!(matches!(
            request.validate(),
            Err(AgentMemoryError::MissingTargetReference { .. })
        ));
    }

    #[test]
    fn request_validation_accepts_read_target() {
        let request = AgentMemoryRequest::new(
            agent_id(5),
            AgentMemoryOperation::Read,
            memory_reference("memory://state"),
        )
        .with_target(value_reference("record:42"));

        assert!(request.validate().is_ok());
    }

    #[test]
    fn request_validation_requires_write_value() {
        let request = AgentMemoryRequest::new(
            agent_id(6),
            AgentMemoryOperation::Write,
            memory_reference("memory://state"),
        );

        assert_eq!(
            request.validate(),
            Err(AgentMemoryError::MissingValueReference)
        );
    }

    #[test]
    fn request_validation_accepts_write_value() {
        let request = AgentMemoryRequest::new(
            agent_id(7),
            AgentMemoryOperation::Write,
            memory_reference("memory://state"),
        )
        .with_value(value_reference("value:42"));

        assert!(request.validate().is_ok());
    }

    #[test]
    fn request_validation_requires_search_expression() {
        let request = AgentMemoryRequest::new(
            agent_id(8),
            AgentMemoryOperation::Search,
            memory_reference("memory://knowledge"),
        );

        assert_eq!(
            request.validate(),
            Err(AgentMemoryError::MissingExpressionReference)
        );
    }

    #[test]
    fn request_validation_accepts_search_expression() {
        let expression = AgentMemoryExpressionReference::new(
            "semantic-expression:42",
        )
        .expect("valid expression reference");

        let request = AgentMemoryRequest::new(
            agent_id(9),
            AgentMemoryOperation::Search,
            memory_reference("memory://knowledge"),
        )
        .with_expression(expression);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn request_validation_requires_forget_target() {
        let request = AgentMemoryRequest::new(
            agent_id(10),
            AgentMemoryOperation::Forget,
            memory_reference("memory://state"),
        );

        assert!(matches!(
            request.validate(),
            Err(AgentMemoryError::MissingTargetReference { .. })
        ));
    }

    #[test]
    fn supports_optional_operational_result_budgets() {
        let request = AgentMemoryRequest::new(
            agent_id(11),
            AgentMemoryOperation::Search,
            memory_reference("memory://knowledge"),
        )
        .with_expression(
            AgentMemoryExpressionReference::new("expression:1")
                .expect("valid expression"),
        )
        .with_result_limit(
            NonZeroUsize::new(100).expect("nonzero result limit"),
        );

        assert_eq!(
            request.result_limit().map(NonZeroUsize::get),
            Some(100)
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn memory_metadata_is_deterministically_ordered() {
        let mut declaration = AgentMemoryDeclaration::new(
            memory_name("state"),
            memory_kind("working"),
            memory_reference("memory://state"),
        );

        declaration
            .insert_metadata("z", "last")
            .expect("valid metadata");

        declaration
            .insert_metadata("a", "first")
            .expect("valid metadata");

        let keys: Vec<&String> =
            declaration.metadata().keys().collect();

        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[1].as_str(), "z");
    }

    #[test]
    fn record_validates_opaque_references() {
        let record = AgentMemoryRecord::new(
            agent_id(12),
            memory_reference("memory://knowledge"),
            value_reference("record:1"),
            memory_kind("semantic"),
        );

        assert!(record.validate().is_ok());
    }

    #[test]
    fn response_supports_empty_results() {
        let response = AgentMemoryResponse::default();

        assert!(response.is_empty());
        assert_eq!(response.len(), 0);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn reference_serialization_round_trip() {
        let original = memory_reference("provider://agent/state");

        let encoded =
            serde_json::to_string(&original).expect("serialize reference");

        let decoded: AgentMemoryReference =
            serde_json::from_str(&encoded).expect("deserialize reference");

        assert_eq!(original, decoded);
    }

    #[test]
    fn reference_deserialization_rejects_invalid_values() {
        let decoded = serde_json::from_str::<AgentMemoryReference>("\"  \"");

        assert!(decoded.is_err());
    }

    #[test]
    fn profile_serialization_round_trip() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("history"),
            memory_kind("episodic"),
            memory_reference("provider://history"),
        )
        .with_access(AgentMemoryAccess::Read);

        let mut profile = AgentMemoryProfile::new(agent_id(13));

        profile
            .add_declaration(declaration)
            .expect("valid declaration");

        let encoded =
            serde_json::to_string(&profile).expect("serialize profile");

        let decoded: AgentMemoryProfile =
            serde_json::from_str(&encoded).expect("deserialize profile");

        assert_eq!(profile, decoded);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn profiles_support_independent_agent_identities() {
        let first = AgentMemoryProfile::new(agent_id(100));
        let second = AgentMemoryProfile::new(agent_id(101));

        assert_ne!(first.agent_id(), second.agent_id());
    }
}
