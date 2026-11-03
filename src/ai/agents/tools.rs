
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Tool Contracts
//!
//! Path: src/ai/agents/tools.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, extensible tool contracts for AI agents.
//!
//! A tool describes a computational capability that an agent may
//! request through an authorized execution environment.
//!
//! This module defines:
//!
//! - Tool identities and names.
//! - Provider references.
//! - Tool definitions.
//! - Input and output schemas.
//! - Parameter contracts.
//! - Invocation descriptions.
//! - Invocation identifiers.
//! - Authorization requirement references.
//! - Provenance references.
//! - Result contracts.
//! - Provider interfaces.
//! - Structural argument and result validation.
//!
//! ## Ownership
//!
//! This module owns declarative tool contracts and the provider boundary.
//!
//! It does not own:
//!
//! - Source parsing or AST construction.
//! - Semantic name resolution.
//! - Universal type definitions.
//! - Capability discovery.
//! - Authorization enforcement.
//! - Tool implementation registration outside this domain.
//! - Network transport.
//! - Distributed scheduling.
//! - Tool execution engines.
//! - Physical resource allocation.
//! - Hardware selection.
//! - Persistent provenance.
//! - Agent lifecycle.
//! - Agent memory.
//! - Universal intermediate representation.
//!
//! ## Canonical integration
//!
//! grammar/ai/agents.g4
//!     -> Source-level @tool declarations.
//!
//! src/ai/ids.rs
//!     -> AiAgentId.
//!
//! src/ai/agents/definition.rs
//!     -> AgentDefinition.
//!
//! src/ai/agents/capabilities.rs
//!     -> Agent capability declarations.
//!
//! src/ai/agents/policies.rs
//!     -> Agent policy integration.
//!
//! src/ai/agents/communication.rs
//!     -> Communication contracts.
//!
//! src/ai/agents/memory.rs
//!     -> Agent memory provider pattern.
//!
//! src/ai/provenance.rs
//!     -> Provenance integration.
//!
//! src/frontend/semantic/
//!     -> Canonical types, effects, policies and authorization analysis.
//!
//! src/distributed/
//!     -> Remote provider communication.
//!
//! src/runtime/
//!     -> Provider execution.
//!
//! src/zuir/dialects/ai/
//!     -> Canonical AI operation representation.
//!
//! ## Scalability
//!
//! No artificial language-level maximum is imposed on:
//!
//! - Tools.
//! - Providers.
//! - Tool parameters.
//! - Input properties.
//! - Output properties.
//! - Agents.
//! - Tool invocations.
//! - Tool results.
//! - Tool versions.
//!
//! Collections grow subject to available resources and configured
//! operational budgets.
//!
//! Rust representations and physical resources remain finite.
//! This module does not promise physically infinite execution.
//!
//! ## Trust boundary
//!
//! A tool declaration does not grant permission to execute a tool.
//!
//! A provider reference does not prove provider availability.
//!
//! An authorization reference does not prove authorization.
//!
//! A provenance reference does not prove provenance authenticity.
//!
//! Providers must independently validate incoming requests.
//!
//! Structural validation does not replace canonical semantic analysis,
//! security enforcement, or execution-time validation.
//!
//! ## POCO-REAF
//!
//! Tool contracts are independent of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - Accelerator vendor.
//! - FPGA.
//! - ASIC.
//! - QPU.
//! - Neuromorphic architecture.
//! - Operating system.
//! - Network topology.
//! - Distributed node count.
//! - Memory topology.
//!
//! A logical tool may have different compatible provider implementations
//! for different execution environments.
//!
//! The source-level tool contract must not encode a physical device.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical AST
//!      |
//!      v
//! Semantic validation
//!      |
//!      v
//! Agent tool contracts
//!      |
//!      +---- Agent capabilities
//!      +---- Agent policies
//!      +---- Authorization requirements
//!      +---- Provenance
//!      |
//!      v
//! Canonical AI IR
//!      |
//!      v
//! Execution planning
//!      |
//!      v
//! Provider resolution
//!      |
//!      v
//! Authorization enforcement
//!      |
//!      v
//! Runtime / distributed execution
//! ```
//!
//! This module does not implement the final execution stages.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT AND FUTURE TYPES
// ============================================================================

/// Result type for agent-tool operations.
pub type AgentToolResult<T> = Result<T, AgentToolError>;

/// Boxed future returned by an agent-tool provider.
///
/// The future is intentionally not required to be `Send`.
///
/// This supports local, embedded, single-threaded and asynchronous
/// execution environments.
///
/// A runtime requiring `Send` must establish that constraint through
/// its own execution adapter.
pub type AgentToolFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + 'a>>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> AgentToolResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(AgentToolError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_description(
    value: &str,
) -> AgentToolResult<()> {
    if value.chars().any(char::is_control) {
        return Err(AgentToolError::InvalidDescription);
    }

    Ok(())
}

// ============================================================================
// TOOL NAME
// ============================================================================

/// Validated, open-world logical tool name.
///
/// A tool name is not a provider address or implementation identifier.
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
pub struct AgentToolName(String);

impl AgentToolName {
    /// Construct a validated tool name.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier("tool name", &value)?;

        Ok(Self(value))
    }

    /// Return the tool name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentToolName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// TOOL REFERENCE
// ============================================================================

/// Opaque logical reference to a tool.
///
/// This reference identifies a tool contract, not a physical executable.
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
pub struct AgentToolReference(String);

impl AgentToolReference {
    /// Construct a tool reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier("tool reference", &value)?;

        Ok(Self(value))
    }

    /// Return the underlying reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentToolReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PROVIDER REFERENCE
// ============================================================================

/// Opaque reference to a tool provider.
///
/// A provider may be local, remote, embedded, distributed or
/// implemented by a future execution environment.
///
/// The reference does not establish provider availability.
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
pub struct AgentToolProviderReference(String);

impl AgentToolProviderReference {
    /// Construct a provider reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier("provider reference", &value)?;

        Ok(Self(value))
    }

    /// Return the provider reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// AUTHORIZATION REFERENCE
// ============================================================================

/// Opaque reference to an authorization requirement.
///
/// This identifies a requirement owned by the canonical security and
/// semantic policy infrastructure.
///
/// It is not an authorization grant.
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
pub struct AgentToolAuthorizationReference(String);

impl AgentToolAuthorizationReference {
    /// Construct an authorization requirement reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier(
            "authorization reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the authorization reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// PROVENANCE REFERENCE
// ============================================================================

/// Opaque reference to canonical provenance information.
///
/// Provenance storage, verification and integrity remain owned by
/// the canonical provenance infrastructure.
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
pub struct AgentToolProvenanceReference(String);

impl AgentToolProvenanceReference {
    /// Construct a provenance reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier(
            "provenance reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the provenance reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// TOOL VERSION
// ============================================================================

/// Version of a logical tool contract.
///
/// Version semantics are independent of provider implementation versions.
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
pub struct AgentToolVersion {
    /// Major version.
    pub major: u64,

    /// Minor version.
    pub minor: u64,

    /// Patch version.
    pub patch: u64,
}

impl AgentToolVersion {
    /// Construct a tool contract version.
    #[must_use]
    pub const fn new(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for AgentToolVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch,
        )
    }
}

// ============================================================================
// TOOL VALUE TYPE
// ============================================================================

/// Structural JSON value category used by tool contracts.
///
/// This is not a replacement for Zamani's canonical semantic type system.
///
/// It provides a provider-boundary representation for validating
/// serialized tool arguments and results.
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
pub enum AgentToolValueType {
    /// Any JSON value.
    Any,

    /// JSON null.
    Null,

    /// Boolean.
    Boolean,

    /// Integer-valued JSON number.
    Integer,

    /// Numeric JSON value.
    Number,

    /// String.
    String,

    /// Array.
    Array,

    /// Object.
    Object,
}

impl AgentToolValueType {
    /// Check whether a JSON value satisfies this structural type.
    #[must_use]
    pub fn accepts(
        self,
        value: &serde_json::Value,
    ) -> bool {
        use serde_json::Value;

        match self {
            Self::Any => true,
            Self::Null => value.is_null(),
            Self::Boolean => value.is_boolean(),

            Self::Integer => {
                value.as_i64().is_some()
                    || value.as_u64().is_some()
            }

            Self::Number => value.is_number(),
            Self::String => value.is_string(),
            Self::Array => value.is_array(),
            Self::Object => value.is_object(),
        }
    }
}

// ============================================================================
// PARAMETER CONTRACT
// ============================================================================

/// Declarative tool parameter contract.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct AgentToolParameter {
    name: String,

    value_type: AgentToolValueType,

    required: bool,

    description: Option<String>,

    default_value: Option<serde_json::Value>,
}

impl AgentToolParameter {
    /// Construct a parameter contract.
    pub fn new(
        name: impl Into<String>,
        value_type: AgentToolValueType,
        required: bool,
    ) -> AgentToolResult<Self> {
        let name = name.into();

        validate_identifier("parameter name", &name)?;

        Ok(Self {
            name,
            value_type,
            required,
            description: None,
            default_value: None,
        })
    }

    /// Attach a parameter description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let description = description.into();

        validate_description(&description)?;

        self.description = Some(description);

        Ok(self)
    }

    /// Attach a default value.
    ///
    /// The default must satisfy the declared structural type.
    pub fn with_default(
        mut self,
        value: serde_json::Value,
    ) -> AgentToolResult<Self> {
        if !self.value_type.accepts(&value) {
            return Err(
                AgentToolError::InvalidDefaultValue {
                    parameter: self.name.clone(),
                },
            );
        }

        self.default_value = Some(value);

        Ok(self)
    }

    /// Return the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the structural value type.
    #[must_use]
    pub const fn value_type(&self) -> AgentToolValueType {
        self.value_type
    }

    /// Return whether the parameter is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return the default value.
    #[must_use]
    pub fn default_value(&self) -> Option<&serde_json::Value> {
        self.default_value.as_ref()
    }

    /// Validate the complete parameter contract.
    pub fn validate(&self) -> AgentToolResult<()> {
        validate_identifier("parameter name", &self.name)?;

        if let Some(description) = &self.description {
            validate_description(description)?;
        }

        if let Some(default) = &self.default_value {
            if !self.value_type.accepts(default) {
                return Err(
                    AgentToolError::InvalidDefaultValue {
                        parameter: self.name.clone(),
                    },
                );
            }

            if self.required {
                return Err(
                    AgentToolError::RequiredParameterHasDefault {
                        parameter: self.name.clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// TOOL SCHEMA
// ============================================================================

/// Structural schema for tool inputs or outputs.
///
/// Properties are stored in deterministic key order.
///
/// Unknown properties are controlled explicitly.
///
/// The schema does not replace canonical Zamani types.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct AgentToolSchema {
    parameters: BTreeMap<String, AgentToolParameter>,

    additional_properties: bool,
}

impl Default for AgentToolSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentToolSchema {
    /// Construct an empty schema.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parameters: BTreeMap::new(),
            additional_properties: false,
        }
    }

    /// Allow properties not explicitly declared by the schema.
    ///
    /// This must be an intentional contract decision.
    #[must_use]
    pub fn with_additional_properties(
        mut self,
        allowed: bool,
    ) -> Self {
        self.additional_properties = allowed;
        self
    }

    /// Add a parameter without overwriting an existing declaration.
    pub fn add_parameter(
        &mut self,
        parameter: AgentToolParameter,
    ) -> AgentToolResult<()> {
        parameter.validate()?;

        let name = parameter.name().to_owned();

        if self.parameters.contains_key(&name) {
            return Err(
                AgentToolError::DuplicateParameter {
                    name,
                },
            );
        }

        self.parameters.insert(name, parameter);

        Ok(())
    }

    /// Return the declared parameters.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &BTreeMap<String, AgentToolParameter> {
        &self.parameters
    }

    /// Return whether undeclared properties are accepted.
    #[must_use]
    pub const fn allows_additional_properties(&self) -> bool {
        self.additional_properties
    }

    /// Find a declared parameter.
    #[must_use]
    pub fn parameter(
        &self,
        name: &str,
    ) -> Option<&AgentToolParameter> {
        self.parameters.get(name)
    }

    /// Validate a JSON object against this schema.
    ///
    /// Required fields, value types and additional-property policy
    /// are checked.
    ///
    /// This validates structural contracts only. More advanced
    /// semantic constraints must be checked by the canonical
    /// frontend and the provider.
    pub fn validate_value(
        &self,
        value: &serde_json::Value,
    ) -> AgentToolResult<()> {
        let object = value.as_object().ok_or(
            AgentToolError::ExpectedObject,
        )?;

        for (name, parameter) in &self.parameters {
            match object.get(name) {
                Some(value) => {
                    if !parameter.value_type().accepts(value) {
                        return Err(
                            AgentToolError::ParameterTypeMismatch {
                                parameter: name.clone(),
                                expected: parameter.value_type(),
                            },
                        );
                    }
                }

                None if parameter.is_required() => {
                    return Err(
                        AgentToolError::MissingRequiredParameter {
                            parameter: name.clone(),
                        },
                    );
                }

                None => {}
            }
        }

        if !self.additional_properties {
            for name in object.keys() {
                if !self.parameters.contains_key(name) {
                    return Err(
                        AgentToolError::UnknownParameter {
                            parameter: name.clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validate the schema's internal consistency.
    pub fn validate(&self) -> AgentToolResult<()> {
        for (name, parameter) in &self.parameters {
            parameter.validate()?;

            if name != parameter.name() {
                return Err(
                    AgentToolError::SchemaKeyMismatch {
                        key: name.clone(),
                        parameter: parameter.name().to_owned(),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// TOOL DEFINITION
// ============================================================================

/// Complete declarative definition of an agent tool.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct AgentToolDefinition {
    reference: AgentToolReference,

    name: AgentToolName,

    version: AgentToolVersion,

    provider: AgentToolProviderReference,

    description: String,

    input: AgentToolSchema,

    output: AgentToolSchema,

    authorization: BTreeSet<AgentToolAuthorizationReference>,

    provenance: Option<AgentToolProvenanceReference>,
}

impl AgentToolDefinition {
    /// Construct a tool definition.
    pub fn new(
        reference: AgentToolReference,
        name: AgentToolName,
        version: AgentToolVersion,
        provider: AgentToolProviderReference,
        description: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let description = description.into();

        validate_description(&description)?;

        Ok(Self {
            reference,
            name,
            version,
            provider,
            description,
            input: AgentToolSchema::new(),
            output: AgentToolSchema::new(),
            authorization: BTreeSet::new(),
            provenance: None,
        })
    }

    /// Set the input schema.
    #[must_use]
    pub fn with_input_schema(
        mut self,
        schema: AgentToolSchema,
    ) -> Self {
        self.input = schema;
        self
    }

    /// Set the output schema.
    #[must_use]
    pub fn with_output_schema(
        mut self,
        schema: AgentToolSchema,
    ) -> Self {
        self.output = schema;
        self
    }

    /// Add an authorization requirement reference.
    pub fn add_authorization_requirement(
        &mut self,
        reference: AgentToolAuthorizationReference,
    ) {
        self.authorization.insert(reference);
    }

    /// Attach a provenance reference.
    pub fn with_provenance(
        mut self,
        reference: AgentToolProvenanceReference,
    ) -> Self {
        self.provenance = Some(reference);
        self
    }

    /// Return the logical tool reference.
    #[must_use]
    pub fn reference(&self) -> &AgentToolReference {
        &self.reference
    }

    /// Return the tool name.
    #[must_use]
    pub fn name(&self) -> &AgentToolName {
        &self.name
    }

    /// Return the contract version.
    #[must_use]
    pub const fn version(&self) -> AgentToolVersion {
        self.version
    }

    /// Return the provider reference.
    #[must_use]
    pub fn provider(&self) -> &AgentToolProviderReference {
        &self.provider
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the input schema.
    #[must_use]
    pub fn input_schema(&self) -> &AgentToolSchema {
        &self.input
    }

    /// Return the output schema.
    #[must_use]
    pub fn output_schema(&self) -> &AgentToolSchema {
        &self.output
    }

    /// Return authorization requirements.
    #[must_use]
    pub fn authorization_requirements(
        &self,
    ) -> &BTreeSet<AgentToolAuthorizationReference> {
        &self.authorization
    }

    /// Return the optional provenance reference.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&AgentToolProvenanceReference> {
        self.provenance.as_ref()
    }

    /// Validate the complete tool definition.
    pub fn validate(&self) -> AgentToolResult<()> {
        validate_description(&self.description)?;

        self.input.validate()?;
        self.output.validate()?;

        Ok(())
    }
}

// ============================================================================
// INVOCATION IDENTITY
// ============================================================================

/// Opaque identity of a logical tool invocation.
///
/// It is supplied by the caller or execution infrastructure.
///
/// It is not automatically globally unique.
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
pub struct AgentToolInvocationId(String);

impl AgentToolInvocationId {
    /// Construct an invocation identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let value = value.into();

        validate_identifier(
            "invocation identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the invocation identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// INVOCATION
// ============================================================================

/// Declarative request to invoke an agent tool.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct AgentToolInvocation {
    id: AgentToolInvocationId,

    agent: AiAgentId,

    tool: AgentToolReference,

    arguments: serde_json::Value,

    provenance: Option<AgentToolProvenanceReference>,

    idempotency_key: Option<String>,
}

impl AgentToolInvocation {
    /// Construct a tool invocation.
    pub fn new(
        id: AgentToolInvocationId,
        agent: AiAgentId,
        tool: AgentToolReference,
        arguments: serde_json::Value,
    ) -> AgentToolResult<Self> {
        if !arguments.is_object() {
            return Err(AgentToolError::ExpectedObject);
        }

        Ok(Self {
            id,
            agent,
            tool,
            arguments,
            provenance: None,
            idempotency_key: None,
        })
    }

    /// Attach invocation provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AgentToolProvenanceReference,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Attach an idempotency key.
    ///
    /// Providers must define and document their idempotency semantics.
    /// Supplying a key alone does not guarantee exactly-once execution.
    pub fn with_idempotency_key(
        mut self,
        key: impl Into<String>,
    ) -> AgentToolResult<Self> {
        let key = key.into();

        validate_identifier("idempotency key", &key)?;

        self.idempotency_key = Some(key);

        Ok(self)
    }

    /// Return the invocation identity.
    #[must_use]
    pub fn id(&self) -> &AgentToolInvocationId {
        &self.id
    }

    /// Return the requesting agent.
    #[must_use]
    pub const fn agent(&self) -> AiAgentId {
        self.agent
    }

    /// Return the logical tool reference.
    #[must_use]
    pub fn tool(&self) -> &AgentToolReference {
        &self.tool
    }

    /// Return the arguments.
    #[must_use]
    pub fn arguments(&self) -> &serde_json::Value {
        &self.arguments
    }

    /// Return invocation provenance.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&AgentToolProvenanceReference> {
        self.provenance.as_ref()
    }

    /// Return the optional idempotency key.
    #[must_use]
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    /// Validate this invocation against a tool definition.
    pub fn validate_against(
        &self,
        definition: &AgentToolDefinition,
    ) -> AgentToolResult<()> {
        if self.tool != *definition.reference() {
            return Err(AgentToolError::ToolReferenceMismatch {
                expected: definition.reference().as_str().to_owned(),
                actual: self.tool.as_str().to_owned(),
            });
        }

        definition
            .input_schema()
            .validate_value(&self.arguments)
    }
}

// ============================================================================
// TOOL RESPONSE
// ============================================================================

/// Result returned by a tool provider.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct AgentToolResponse {
    invocation: AgentToolInvocationId,

    output: serde_json::Value,

    provenance: Option<AgentToolProvenanceReference>,
}

impl AgentToolResponse {
    /// Construct a provider response.
    pub fn new(
        invocation: AgentToolInvocationId,
        output: serde_json::Value,
    ) -> AgentToolResult<Self> {
        if !output.is_object() {
            return Err(AgentToolError::ExpectedObject);
        }

        Ok(Self {
            invocation,
            output,
            provenance: None,
        })
    }

    /// Attach result provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AgentToolProvenanceReference,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Return the associated invocation identity.
    #[must_use]
    pub fn invocation(&self) -> &AgentToolInvocationId {
        &self.invocation
    }

    /// Return the output value.
    #[must_use]
    pub fn output(&self) -> &serde_json::Value {
        &self.output
    }

    /// Return optional provenance.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&AgentToolProvenanceReference> {
        self.provenance.as_ref()
    }

    /// Validate the response against its originating invocation
    /// and tool definition.
    pub fn validate_against(
        &self,
        invocation: &AgentToolInvocation,
        definition: &AgentToolDefinition,
    ) -> AgentToolResult<()> {
        if self.invocation != invocation.id {
            return Err(
                AgentToolError::InvocationIdentityMismatch,
            );
        }

        definition
            .output_schema()
            .validate_value(&self.output)
    }
}

// ============================================================================
// PROVIDER INTERFACE
// ============================================================================

/// Asynchronous boundary implemented by an agent-tool provider.
///
/// Implementations may delegate to:
///
/// - Local functions.
/// - Embedded services.
/// - External applications.
/// - Remote services.
/// - Distributed systems.
/// - Hardware-facing adapters.
/// - Future computational environments.
///
/// This trait does not prescribe a transport or execution engine.
///
/// Implementations MUST independently:
///
/// - Validate untrusted input.
/// - Enforce authorization.
/// - Apply configured operational budgets.
/// - Preserve invocation identity.
/// - Report failures explicitly.
/// - Avoid reporting success before completion.
///
/// The caller must validate the returned response against the
/// originating invocation and tool definition.
pub trait AgentToolProvider {
    /// Submit one tool invocation.
    fn invoke<'a>(
        &'a self,
        invocation: &'a AgentToolInvocation,
    ) -> AgentToolFuture<'a, AgentToolResult<AgentToolResponse>>;
}

// ============================================================================
// ERROR CONTRACT
// ============================================================================

/// Errors produced by agent-tool contract operations.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum AgentToolError {
    /// An identifier is invalid.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,

        /// Rejected value.
        value: String,
    },

    /// A description contains control characters.
    InvalidDescription,

    /// A default value violates its parameter type.
    InvalidDefaultValue {
        /// Parameter name.
        parameter: String,
    },

    /// A required parameter cannot have a default.
    RequiredParameterHasDefault {
        /// Parameter name.
        parameter: String,
    },

    /// A parameter is duplicated.
    DuplicateParameter {
        /// Parameter name.
        name: String,
    },

    /// A schema map key does not match its parameter name.
    SchemaKeyMismatch {
        /// Map key.
        key: String,

        /// Actual parameter name.
        parameter: String,
    },

    /// A JSON object was required.
    ExpectedObject,

    /// A required parameter is missing.
    MissingRequiredParameter {
        /// Parameter name.
        parameter: String,
    },

    /// A parameter has an incompatible structural type.
    ParameterTypeMismatch {
        /// Parameter name.
        parameter: String,

        /// Expected type.
        expected: AgentToolValueType,
    },

    /// An undeclared parameter was supplied.
    UnknownParameter {
        /// Parameter name.
        parameter: String,
    },

    /// Invocation and definition refer to different tools.
    ToolReferenceMismatch {
        /// Expected reference.
        expected: String,

        /// Actual reference.
        actual: String,
    },

    /// Provider response belongs to another invocation.
    InvocationIdentityMismatch,

    /// Provider execution failed.
    ProviderFailure {
        /// Provider-neutral explanation.
        reason: String,
    },

    /// Provider is unavailable.
    ProviderUnavailable {
        /// Provider reference.
        provider: String,
    },

    /// Provider returned an invalid result.
    InvalidProviderResponse {
        /// Explanation.
        reason: String,
    },

    /// Authorization was denied by the authoritative security system.
    AuthorizationDenied {
        /// Requirement reference.
        requirement: String,
    },

    /// The requested tool does not exist.
    ToolNotFound {
        /// Tool reference.
        reference: String,
    },
}

impl fmt::Display for AgentToolError {
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

            Self::InvalidDescription => {
                formatter.write_str(
                    "tool description contains control characters",
                )
            }

            Self::InvalidDefaultValue { parameter } => {
                write!(
                    formatter,
                    "invalid default value for parameter {parameter:?}"
                )
            }

            Self::RequiredParameterHasDefault { parameter } => {
                write!(
                    formatter,
                    "required parameter {parameter:?} cannot have a default"
                )
            }

            Self::DuplicateParameter { name } => {
                write!(
                    formatter,
                    "duplicate tool parameter: {name:?}"
                )
            }

            Self::SchemaKeyMismatch { key, parameter } => {
                write!(
                    formatter,
                    "schema key {key:?} does not match parameter {parameter:?}"
                )
            }

            Self::ExpectedObject => {
                formatter.write_str(
                    "tool arguments or results must be JSON objects",
                )
            }

            Self::MissingRequiredParameter { parameter } => {
                write!(
                    formatter,
                    "missing required tool parameter: {parameter:?}"
                )
            }

            Self::ParameterTypeMismatch {
                parameter,
                expected,
            } => {
                write!(
                    formatter,
                    "tool parameter {parameter:?} does not satisfy {expected:?}"
                )
            }

            Self::UnknownParameter { parameter } => {
                write!(
                    formatter,
                    "unknown tool parameter: {parameter:?}"
                )
            }

            Self::ToolReferenceMismatch { expected, actual } => {
                write!(
                    formatter,
                    "tool reference mismatch: expected {expected:?}, got {actual:?}"
                )
            }

            Self::InvocationIdentityMismatch => {
                formatter.write_str(
                    "tool response invocation identity does not match request",
                )
            }

            Self::ProviderFailure { reason } => {
                write!(
                    formatter,
                    "agent tool provider failed: {reason}"
                )
            }

            Self::ProviderUnavailable { provider } => {
                write!(
                    formatter,
                    "agent tool provider unavailable: {provider}"
                )
            }

            Self::InvalidProviderResponse { reason } => {
                write!(
                    formatter,
                    "invalid agent tool provider response: {reason}"
                )
            }

            Self::AuthorizationDenied { requirement } => {
                write!(
                    formatter,
                    "tool authorization denied for requirement {requirement:?}"
                )
            }

            Self::ToolNotFound { reference } => {
                write!(
                    formatter,
                    "agent tool not found: {reference:?}"
                )
            }
        }
    }
}

impl Error for AgentToolError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    fn tool_definition() -> AgentToolDefinition {
        let mut input = AgentToolSchema::new();

        input.add_parameter(
            AgentToolParameter::new(
                "query",
                AgentToolValueType::String,
                true,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        input.add_parameter(
            AgentToolParameter::new(
                "limit",
                AgentToolValueType::Integer,
                false,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        let mut output = AgentToolSchema::new();

        output.add_parameter(
            AgentToolParameter::new(
                "result",
                AgentToolValueType::String,
                true,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        AgentToolDefinition::new(
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            AgentToolName::new("search")
                .expect("valid name"),
            AgentToolVersion::new(1, 0, 0),
            AgentToolProviderReference::new("provider.local")
                .expect("valid provider"),
            "Search provider",
        )
        .expect("valid tool")
        .with_input_schema(input)
        .with_output_schema(output)
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(
            AgentToolName::new("").is_err()
        );

        assert!(
            AgentToolReference::new(" ").is_err()
        );

        assert!(
            AgentToolProviderReference::new("").is_err()
        );
    }

    #[test]
    fn identifiers_preserve_open_world_names() {
        let name = AgentToolName::new(
            "future_quantum_assisted_reasoning",
        )
        .expect("valid name");

        assert_eq!(
            name.as_str(),
            "future_quantum_assisted_reasoning"
        );
    }

    #[test]
    fn versions_support_full_u64_range() {
        let version = AgentToolVersion::new(
            u64::MAX,
            u64::MAX,
            u64::MAX,
        );

        assert_eq!(version.major, u64::MAX);
    }

    #[test]
    fn schemas_reject_duplicate_parameters() {
        let mut schema = AgentToolSchema::new();

        let first = AgentToolParameter::new(
            "query",
            AgentToolValueType::String,
            true,
        )
        .expect("valid parameter");

        let second = AgentToolParameter::new(
            "query",
            AgentToolValueType::Integer,
            false,
        )
        .expect("valid parameter");

        schema.add_parameter(first)
            .expect("first insertion");

        assert!(
            schema.add_parameter(second).is_err()
        );
    }

    #[test]
    fn required_parameters_are_validated() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({"query": "Zamani"})
            ).is_ok()
        );

        assert!(
            definition.input_schema().validate_value(
                &json!({})
            ).is_err()
        );
    }

    #[test]
    fn parameter_types_are_validated() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({"query": 42})
            ).is_err()
        );
    }

    #[test]
    fn unknown_parameters_are_rejected_by_default() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({
                    "query": "Zamani",
                    "unknown": true
                })
            ).is_err()
        );
    }

    #[test]
    fn additional_properties_are_explicit() {
        let mut schema =
            AgentToolSchema::new()
                .with_additional_properties(true);

        schema.add_parameter(
            AgentToolParameter::new(
                "query",
                AgentToolValueType::String,
                true,
            )
            .expect("valid parameter"),
        )
        .expect("insertion");

        assert!(
            schema.validate_value(
                &json!({
                    "query": "Zamani",
                    "extension": true
                })
            ).is_ok()
        );
    }

    #[test]
    fn invalid_defaults_are_rejected() {
        let parameter = AgentToolParameter::new(
            "count",
            AgentToolValueType::Integer,
            false,
        )
        .expect("valid parameter");

        assert!(
            parameter.with_default(json!("not an integer")).is_err()
        );
    }

    #[test]
    fn required_parameter_defaults_are_rejected() {
        let parameter = AgentToolParameter::new(
            "query",
            AgentToolValueType::String,
            true,
        )
        .expect("valid parameter");

        assert!(
            parameter.with_default(json!("default")).is_err()
        );
    }

    #[test]
    fn invocation_must_match_tool_definition() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.1")
                .expect("valid ID"),
            AiAgentId::new(1).expect("valid agent ID"),
            AgentToolReference::new("tool.other")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        assert!(
            invocation.validate_against(&definition).is_err()
        );
    }

    #[test]
    fn valid_invocations_pass_schema_validation() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.2")
                .expect("valid ID"),
            AiAgentId::new(2).expect("valid agent ID"),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({
                "query": "Zamani",
                "limit": 10
            }),
        )
        .expect("valid invocation");

        assert!(
            invocation.validate_against(&definition).is_ok()
        );
    }

    #[test]
    fn response_must_match_invocation_identity() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.3")
                .expect("valid ID"),
            AiAgentId::new(3).expect("valid ID"),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        let response = AgentToolResponse::new(
            AgentToolInvocationId::new("invocation.other")
                .expect("valid ID"),
            json!({"result": "answer"}),
        )
        .expect("valid response");

        assert!(
            response.validate_against(
                &invocation,
                &definition,
            ).is_err()
        );
    }

    #[test]
    fn valid_responses_pass_output_validation() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.4")
                .expect("valid ID"),
            AiAgentId::new(4).expect("valid ID"),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        let response = AgentToolResponse::new(
            invocation.id().clone(),
            json!({"result": "answer"}),
        )
        .expect("valid response");

        assert!(
            response.validate_against(
                &invocation,
                &definition,
            ).is_ok()
        );
    }

    #[test]
    fn provenance_references_are_preserved() {
        let definition = tool_definition()
            .with_provenance(
                AgentToolProvenanceReference::new(
                    "provenance.tool.search",
                )
                .expect("valid provenance"),
            );

        assert_eq!(
            definition.provenance()
                .expect("provenance")
                .as_str(),
            "provenance.tool.search"
        );
    }

    #[test]
    fn authorization_requirements_are_deduplicated() {
        let mut definition = tool_definition();

        let reference =
            AgentToolAuthorizationReference::new(
                "security.tool.search",
            )
            .expect("valid reference");

        definition.add_authorization_requirement(
            reference.clone(),
        );

        definition.add_authorization_requirement(
            reference,
        );

        assert_eq!(
            definition.authorization_requirements().len(),
            1
        );
    }

    #[test]
    fn idempotency_key_is_explicit() {
        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.5")
                .expect("valid ID"),
            AiAgentId::new(5).expect("valid ID"),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation")
        .with_idempotency_key("request.5")
        .expect("valid key");

        assert_eq!(
            invocation.idempotency_key(),
            Some("request.5")
        );
    }

    #[test]
    fn tool_definition_serializes_and_deserializes() {
        let original = tool_definition();

        let encoded = serde_json::to_string(&original)
            .expect("serialization");

        let decoded: AgentToolDefinition =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(original, decoded);

        decoded.validate()
            .expect("decoded definition remains valid");
    }

    #[test]
    fn invocation_serializes_and_deserializes() {
        let original = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.6")
                .expect("valid ID"),
            AiAgentId::new(6).expect("valid ID"),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        let encoded = serde_json::to_string(&original)
            .expect("serialization");

        let decoded: AgentToolInvocation =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(original, decoded);
    }
}
