//! Zamani Universal Meta-Compiler (UMC): Advanced Object-Oriented Programming
//!
//! Production-oriented OOP metadata and orchestration layer.
//!
//! Responsibilities:
//! - represent advanced Zamani object/class metadata;
//! - validate inheritance, interfaces, fields, and methods;
//! - provide explicit access-control metadata;
//! - provide safe meta-object operations;
//! - provide lifecycle/self-healing policy metadata;
//! - expose deterministic reflection information;
//! - provide security/ethical policy boundaries.
//!
//! This module deliberately does NOT:
//! - execute generated code;
//! - invent cryptographic primitives;
//! - perform arbitrary runtime code mutation;
//! - silently bypass access control;
//! - implement another compiler pipeline.
//!
//! Runtime execution belongs to the runtime subsystem.
//! Code generation belongs to the compiler/backend subsystem.
//! Cryptography belongs to the crypto subsystem.
//! Formal verification belongs to the verification subsystem.

use crate::ast::{Identifier, Type};
use crate::stdlib::collections::{List, Map};
use crate::stdlib::meta_ops::MetaValue;
use crate::toolchain::meta_programming::ZamaniCodeSnippet;

// -----------------------------------------------------------------------------
// Lifecycle
// -----------------------------------------------------------------------------

/// Initializes the advanced OOP metadata subsystem.
///
/// Initialization is intentionally side-effect free apart from a diagnostic
/// message retained for compatibility with `compiler::initialize_compiler`.
pub fn init_oop_advanced() {
    println!("  - Initializing Zamani Advanced OOP metadata subsystem...");
}

/// Shuts down the advanced OOP metadata subsystem.
pub fn shutdown_oop_advanced() {
    println!("  - Shutting down Zamani Advanced OOP metadata subsystem...");
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the advanced OOP subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OopError {
    EmptyIdentifier {
        kind: &'static str,
    },

    DuplicateField {
        name: String,
    },

    DuplicateMethod {
        name: String,
    },

    DuplicateParent {
        name: String,
    },

    DuplicateInterface {
        name: String,
    },

    InvalidInheritance {
        class_name: String,
        parent_name: String,
    },

    InvalidMethod {
        method_name: String,
        reason: String,
    },

    InvalidAccessPolicy {
        reason: String,
    },

    InvalidSecurityPolicy {
        reason: String,
    },

    InvalidMetaOperation {
        reason: String,
    },

    EmptyObjectState,

    UnsupportedOperation {
        operation: String,
    },
}

impl std::fmt::Display for OopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(f, "OOP {} identifier cannot be empty", kind)
            }

            Self::DuplicateField { name } => {
                write!(f, "duplicate field '{}'", name)
            }

            Self::DuplicateMethod { name } => {
                write!(f, "duplicate method '{}'", name)
            }

            Self::DuplicateParent { name } => {
                write!(f, "duplicate parent class '{}'", name)
            }

            Self::DuplicateInterface { name } => {
                write!(f, "duplicate interface '{}'", name)
            }

            Self::InvalidInheritance {
                class_name,
                parent_name,
            } => {
                write!(
                    f,
                    "invalid inheritance: '{}' cannot inherit from '{}'",
                    class_name, parent_name
                )
            }

            Self::InvalidMethod {
                method_name,
                reason,
            } => {
                write!(
                    f,
                    "invalid method '{}': {}",
                    method_name, reason
                )
            }

            Self::InvalidAccessPolicy { reason } => {
                write!(f, "invalid access policy: {}", reason)
            }

            Self::InvalidSecurityPolicy { reason } => {
                write!(f, "invalid security policy: {}", reason)
            }

            Self::InvalidMetaOperation { reason } => {
                write!(f, "invalid meta-object operation: {}", reason)
            }

            Self::EmptyObjectState => {
                write!(f, "object state cannot be empty")
            }

            Self::UnsupportedOperation { operation } => {
                write!(
                    f,
                    "unsupported OOP operation '{}'",
                    operation
                )
            }
        }
    }
}

impl std::error::Error for OopError {}

// -----------------------------------------------------------------------------
// Security
// -----------------------------------------------------------------------------

/// Security classification for an object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityLevel {
    Unclassified,
    Confidential,
    Secret,
    TopSecret,

    /// Security depends on a configured external cryptographic policy.
    QuantumSecured {
        security_bits: u16,
    },

    /// State must remain encrypted outside an approved cryptographic boundary.
    HomomorphicallyEncrypted,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Unclassified
    }
}

impl SecurityLevel {
    pub fn validate(&self) -> Result<(), OopError> {
        if let Self::QuantumSecured { security_bits } = self {
            if *security_bits == 0 {
                return Err(OopError::InvalidSecurityPolicy {
                    reason: "quantum security strength cannot be zero".to_string(),
                });
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Access control
// -----------------------------------------------------------------------------

/// Access level for fields and methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessLevel {
    Public,
    Private,
    Protected,

    /// Access is restricted to explicitly named identities/contexts.
    Restricted(Vec<String>),
}

impl Default for AccessLevel {
    fn default() -> Self {
        Self::Private
    }
}

/// Context-sensitive access rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessRule {
    pub context: String,
    pub level: AccessLevel,
}

impl AccessRule {
    pub fn new(
        context: impl Into<String>,
        level: AccessLevel,
    ) -> Result<Self, OopError> {
        let context = context.into();

        if context.trim().is_empty() {
            return Err(OopError::InvalidAccessPolicy {
                reason: "access-rule context cannot be empty".to_string(),
            });
        }

        Ok(Self { context, level })
    }
}

/// Fine-grained object access policy.
#[derive(Debug, Clone, PartialEq)]
pub struct AccessPolicy {
    pub default_access: AccessLevel,
    pub field_overrides: Map<Identifier, AccessLevel>,
    pub method_overrides: Map<Identifier, AccessLevel>,
    pub context_based_rules: List<AccessRule>,
}

impl Default for AccessPolicy {
    fn default() -> Self {
        Self {
            default_access: AccessLevel::Private,
            field_overrides: Map::new(),
            method_overrides: Map::new(),
            context_based_rules: List::new(),
        }
    }
}

impl AccessPolicy {
    pub fn validate(&self) -> Result<(), OopError> {
        for rule in self.context_based_rules.iter() {
            if rule.context.trim().is_empty() {
                return Err(OopError::InvalidAccessPolicy {
                    reason: "context-based rule has an empty context".to_string(),
                });
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Self-healing policy
// -----------------------------------------------------------------------------

/// Policy describing what may happen when an object becomes invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfHealingPolicy {
    None,

    /// Restore state from an externally verified snapshot.
    RollbackToSnapshot,

    /// Quarantine the object and report the failure.
    QuarantineAndReport,

    /// Maintain a fixed number of redundant state replicas.
    AdaptiveRedundancy {
        replicas: usize,
    },

    /// Generate a proposal which must be separately verified and approved.
    ProposeEvolution {
        require_verification: bool,
        require_approval: bool,
    },
}

impl Default for SelfHealingPolicy {
    fn default() -> Self {
        Self::None
    }
}

impl SelfHealingPolicy {
    pub fn validate(&self) -> Result<(), OopError> {
        if let Self::AdaptiveRedundancy { replicas } = self {
            if *replicas == 0 {
                return Err(OopError::InvalidSecurityPolicy {
                    reason: "adaptive redundancy requires at least one replica"
                        .to_string(),
                });
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Multi-paradigm references
// -----------------------------------------------------------------------------

/// Opaque reference to quantum-specific object metadata.
///
/// Actual quantum state is owned by the quantum runtime/backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumStateRef {
    pub id: String,
}

impl QuantumStateRef {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "quantum state",
            });
        }

        Ok(Self { id })
    }
}

/// Opaque reference to a nano-runtime object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NanoAgentRef {
    pub id: String,
}

impl NanoAgentRef {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "nano agent",
            });
        }

        Ok(Self { id })
    }
}

/// Opaque reference to temporal/MTS state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MTSStateSnapshot {
    pub id: String,
}

impl MTSStateSnapshot {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "MTS snapshot",
            });
        }

        Ok(Self { id })
    }
}

// -----------------------------------------------------------------------------
// Object representation
// -----------------------------------------------------------------------------

/// Advanced Zamani object representation.
///
/// The enum contains metadata/references only. Runtime state remains owned by
/// the runtime subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum ZamaniObject {
    ClassicalObject(
        Identifier,
        Map<Identifier, MetaValue>,
    ),

    QuantumObject(
        Identifier,
        List<QuantumStateRef>,
    ),

    NanoObject(
        Identifier,
        NanoAgentRef,
    ),

    MTSObject(
        Identifier,
        List<MTSStateSnapshot>,
    ),

    /// Encrypted state is represented by an opaque ciphertext identifier.
    ///
    /// This module deliberately does not implement cryptography itself.
    HomomorphicObject(
        Identifier,
        String,
    ),
}

impl ZamaniObject {
    /// Returns the object's stable compiler-level identifier.
    pub fn get_id(&self) -> Identifier {
        match self {
            Self::ClassicalObject(id, _)
            | Self::QuantumObject(id, _)
            | Self::NanoObject(id, _)
            | Self::MTSObject(id, _)
            | Self::HomomorphicObject(id, _) => id.clone(),
        }
    }

    /// Returns the object kind.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ClassicalObject(_, _) => "classical",
            Self::QuantumObject(_, _) => "quantum",
            Self::NanoObject(_, _) => "nano",
            Self::MTSObject(_, _) => "mts",
            Self::HomomorphicObject(_, _) => "homomorphic",
        }
    }

    /// Returns a deterministic object description suitable for diagnostics.
    pub fn describe(&self) -> String {
        format!(
            "{}:{}",
            self.kind(),
            self.get_id().0
        )
    }

    /// Returns a source-level representation suitable for diagnostics.
    ///
    /// This is intentionally descriptive rather than executable.
    pub fn get_zamani_code_representation(&self) -> ZamaniCodeSnippet {
        format!(
            "// Zamani object representation\n// kind: {}\n// id: {}\n",
            self.kind(),
            self.get_id().0
        )
    }
}

// -----------------------------------------------------------------------------
// Method definitions
// -----------------------------------------------------------------------------

/// Placeholder for a quantum implementation.
///
/// Actual circuit representation belongs to the quantum compiler/backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumCircuitDefinition {
    pub identifier: String,
}

impl QuantumCircuitDefinition {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let identifier = id.into();

        if identifier.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "quantum circuit",
            });
        }

        Ok(Self { identifier })
    }
}

/// Placeholder for nano behavior.
///
/// Actual nano scheduling belongs to the nano runtime/backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NanoBehaviorBlueprint {
    pub identifier: String,
}

impl NanoBehaviorBlueprint {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let identifier = id.into();

        if identifier.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "nano behavior",
            });
        }

        Ok(Self { identifier })
    }
}

/// Placeholder for MTS workflow metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MTSWorkflowDefinition {
    pub identifier: String,
}

impl MTSWorkflowDefinition {
    pub fn new(id: impl Into<String>) -> Result<Self, OopError> {
        let identifier = id.into();

        if identifier.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "MTS workflow",
            });
        }

        Ok(Self { identifier })
    }
}

/// Advanced method definition.
///
/// Each implementation is optional because Zamani supports multiple execution
/// backends without forcing every method to have every implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinitionAdvanced {
    pub name: Identifier,
    pub parameters: Map<Identifier, Type>,
    pub return_type: Type,

    pub classical_impl: Option<ZamaniCodeSnippet>,
    pub quantum_impl: Option<QuantumCircuitDefinition>,
    pub nano_impl: Option<NanoBehaviorBlueprint>,
    pub temporal_impl: Option<MTSWorkflowDefinition>,

    pub security_constraints: List<String>,
    pub evas_approval_required: bool,
}

impl MethodDefinitionAdvanced {
    pub fn validate(&self) -> Result<(), OopError> {
        if self.name.0.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "method",
            });
        }

        if let Some(code) = &self.classical_impl {
            if code.trim().is_empty() {
                return Err(OopError::InvalidMethod {
                    method_name: self.name.0.clone(),
                    reason: "classical implementation is empty".to_string(),
                });
            }
        }

        for constraint in self.security_constraints.iter() {
            if constraint.trim().is_empty() {
                return Err(OopError::InvalidMethod {
                    method_name: self.name.0.clone(),
                    reason: "security constraint cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Class definitions
// -----------------------------------------------------------------------------

/// Advanced class definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDefinitionAdvanced {
    pub name: Identifier,
    pub parent_classes: List<Identifier>,
    pub interfaces: List<Identifier>,
    pub fields: Map<Identifier, Type>,
    pub methods: Map<Identifier, MethodDefinitionAdvanced>,

    pub access_policies: AccessPolicy,
    pub security_level: SecurityLevel,
    pub self_healing_policy: SelfHealingPolicy,

    pub meta_object_protocol: Option<Identifier>,
}

impl ClassDefinitionAdvanced {
    /// Validates the complete class declaration.
    pub fn validate(&self) -> Result<(), OopError> {
        if self.name.0.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "class",
            });
        }

        self.security_level.validate()?;
        self.self_healing_policy.validate()?;
        self.access_policies.validate()?;

        let mut parents = std::collections::HashSet::new();

        for parent in self.parent_classes.iter() {
            if parent.0.trim().is_empty() {
                return Err(OopError::InvalidInheritance {
                    class_name: self.name.0.clone(),
                    parent_name: "<empty>".to_string(),
                });
            }

            if parent == &self.name {
                return Err(OopError::InvalidInheritance {
                    class_name: self.name.0.clone(),
                    parent_name: parent.0.clone(),
                });
            }

            if !parents.insert(parent.0.clone()) {
                return Err(OopError::DuplicateParent {
                    name: parent.0.clone(),
                });
            }
        }

        let mut interfaces = std::collections::HashSet::new();

        for interface in self.interfaces.iter() {
            if interface.0.trim().is_empty() {
                return Err(OopError::EmptyIdentifier {
                    kind: "interface",
                });
            }

            if !interfaces.insert(interface.0.clone()) {
                return Err(OopError::DuplicateInterface {
                    name: interface.0.clone(),
                });
            }
        }

        for method in self.methods.values() {
            method.validate()?;
        }

        Ok(())
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn method_count(&self) -> usize {
        self.methods.len()
    }

    pub fn parent_count(&self) -> usize {
        self.parent_classes.len()
    }

    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }
}

// -----------------------------------------------------------------------------
// Meta-object protocol
// -----------------------------------------------------------------------------

/// Explicit, auditable meta-object operations.
///
/// Operations are descriptions; execution is performed by the appropriate
/// compiler/runtime subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaObjectOperation {
    OverrideMethod {
        object_id: Identifier,
        method_name: Identifier,
        implementation_ref: String,
    },

    InstallBehavior {
        class_name: Identifier,
        behavior_ref: String,
    },

    ReflectObject {
        object_id: Identifier,
    },
}

/// Safe meta-object protocol.
///
/// It validates operations but does not silently mutate executable code.
#[derive(Debug, Default, Clone)]
pub struct MetaObjectProtocol;

impl MetaObjectProtocol {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_operation(
        &self,
        operation: &MetaObjectOperation,
    ) -> Result<(), OopError> {
        match operation {
            MetaObjectOperation::OverrideMethod {
                object_id,
                method_name,
                implementation_ref,
            } => {
                if object_id.0.trim().is_empty() {
                    return Err(OopError::InvalidMetaOperation {
                        reason: "object identifier cannot be empty".to_string(),
                    });
                }

                if method_name.0.trim().is_empty() {
                    return Err(OopError::InvalidMetaOperation {
                        reason: "method identifier cannot be empty".to_string(),
                    });
                }

                if implementation_ref.trim().is_empty() {
                    return Err(OopError::InvalidMetaOperation {
                        reason: "implementation reference cannot be empty".to_string(),
                    });
                }
            }

            MetaObjectOperation::InstallBehavior {
                class_name,
                behavior_ref,
            } => {
                if class_name.0.trim().is_empty()
                    || behavior_ref.trim().is_empty()
                {
                    return Err(OopError::InvalidMetaOperation {
                        reason:
                            "class name and behavior reference are required"
                                .to_string(),
                    });
                }
            }

            MetaObjectOperation::ReflectObject { object_id } => {
                if object_id.0.trim().is_empty() {
                    return Err(OopError::InvalidMetaOperation {
                        reason: "object identifier cannot be empty".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validates a method override request.
    ///
    /// The actual mutation must be performed by an authorized compiler/runtime
    /// component after this validation succeeds.
    pub fn validate_method_override(
        &self,
        object_id: Identifier,
        method_name: Identifier,
        implementation_ref: impl Into<String>,
    ) -> Result<MetaObjectOperation, OopError> {
        let operation = MetaObjectOperation::OverrideMethod {
            object_id,
            method_name,
            implementation_ref: implementation_ref.into(),
        };

        self.validate_operation(&operation)?;

        Ok(operation)
    }

    /// Validates installation of a class-level behavior.
    pub fn validate_behavior_installation(
        &self,
        class_name: Identifier,
        behavior_ref: impl Into<String>,
    ) -> Result<MetaObjectOperation, OopError> {
        let operation = MetaObjectOperation::InstallBehavior {
            class_name,
            behavior_ref: behavior_ref.into(),
        };

        self.validate_operation(&operation)?;

        Ok(operation)
    }

    /// Creates an auditable reflection request.
    pub fn reflect_object(
        &self,
        object_id: Identifier,
    ) -> Result<MetaObjectOperation, OopError> {
        let operation = MetaObjectOperation::ReflectObject { object_id };

        self.validate_operation(&operation)?;

        Ok(operation)
    }
}

// -----------------------------------------------------------------------------
// Reflection
// -----------------------------------------------------------------------------

/// Stable reflection information.
///
/// This avoids exposing internal implementation details directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectReflection {
    pub object_id: String,
    pub object_kind: String,
}

impl ObjectReflection {
    pub fn from_object(object: &ZamaniObject) -> Self {
        Self {
            object_id: object.get_id().0.clone(),
            object_kind: object.kind().to_string(),
        }
    }

    pub fn as_map(&self) -> Map<String, String> {
        let mut result = Map::new();

        result.insert(
            "id".to_string(),
            self.object_id.clone(),
        );

        result.insert(
            "kind".to_string(),
            self.object_kind.clone(),
        );

        result
    }
}

// -----------------------------------------------------------------------------
// Autonomous object
// -----------------------------------------------------------------------------

/// Capability boundary for autonomous object behavior.
///
/// Implementations belong to the runtime/agent subsystem.
pub trait AutonomousObjectController {
    fn cognitive_cycle(&mut self) -> Result<(), String>;

    fn request_optimization(
        &mut self,
        goal: &str,
    ) -> Result<(), String>;
}

/// Object wrapper carrying an externally supplied controller.
///
/// The compiler module does not construct or execute an AGI agent itself.
pub struct AutonomousObject<C>
where
    C: AutonomousObjectController,
{
    pub base_object: ZamaniObject,
    pub controller: C,
}

impl<C> AutonomousObject<C>
where
    C: AutonomousObjectController,
{
    pub fn new(
        base_object: ZamaniObject,
        controller: C,
    ) -> Self {
        Self {
            base_object,
            controller,
        }
    }

    pub fn autonomous_cognitive_cycle(
        &mut self,
    ) -> Result<(), String> {
        self.controller.cognitive_cycle()
    }

    /// Requests optimization through the external controller.
    ///
    /// This function does not directly replace executable code.
    pub fn request_self_optimization(
        &mut self,
        goal: &str,
    ) -> Result<(), String> {
        if goal.trim().is_empty() {
            return Err(
                "self-optimization goal cannot be empty".to_string()
            );
        }

        self.controller.request_optimization(goal)
    }
}

// -----------------------------------------------------------------------------
// Secure object boundary
// -----------------------------------------------------------------------------

/// Metadata describing an externally managed encrypted object state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedObjectState {
    pub object_id: Identifier,
    pub ciphertext_ref: String,
}

impl EncryptedObjectState {
    pub fn new(
        object_id: Identifier,
        ciphertext_ref: impl Into<String>,
    ) -> Result<Self, OopError> {
        let ciphertext_ref = ciphertext_ref.into();

        if object_id.0.trim().is_empty() {
            return Err(OopError::EmptyIdentifier {
                kind: "object",
            });
        }

        if ciphertext_ref.trim().is_empty() {
            return Err(OopError::InvalidSecurityPolicy {
                reason: "ciphertext reference cannot be empty".to_string(),
            });
        }

        Ok(Self {
            object_id,
            ciphertext_ref,
        })
    }
}

/// Secure object operation request.
///
/// Actual cryptographic execution belongs to the crypto subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureObjectOperation {
    EncryptState {
        object_id: Identifier,
    },

    Compute {
        ciphertext_ref: String,
        operation: String,
    },

    VerifySignature {
        object_id: Identifier,
        signature_ref: String,
    },
}

/// Security boundary for object operations.
#[derive(Debug, Default, Clone)]
pub struct SecureObjectOperations;

impl SecureObjectOperations {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(
        &self,
        operation: &SecureObjectOperation,
    ) -> Result<(), OopError> {
        match operation {
            SecureObjectOperation::EncryptState { object_id } => {
                if object_id.0.trim().is_empty() {
                    return Err(OopError::EmptyIdentifier {
                        kind: "object",
                    });
                }
            }

            SecureObjectOperation::Compute {
                ciphertext_ref,
                operation,
            } => {
                if ciphertext_ref.trim().is_empty() {
                    return Err(OopError::InvalidSecurityPolicy {
                        reason:
                            "ciphertext reference cannot be empty"
                                .to_string(),
                    });
                }

                if operation.trim().is_empty() {
                    return Err(OopError::InvalidSecurityPolicy {
                        reason:
                            "cryptographic operation cannot be empty"
                                .to_string(),
                    });
                }
            }

            SecureObjectOperation::VerifySignature {
                object_id,
                signature_ref,
            } => {
                if object_id.0.trim().is_empty()
                    || signature_ref.trim().is_empty()
                {
                    return Err(OopError::InvalidSecurityPolicy {
                        reason:
                            "object and signature references are required"
                                .to_string(),
                    });
                }
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Compatibility helpers
// -----------------------------------------------------------------------------

impl ZamaniObject {
    /// Returns a diagnostic representation of the object's implementation.
    ///
    /// The old implementation attempted to mutate the object directly.
    /// Production code instead returns a representation and leaves mutation
    /// to the authorized compiler/runtime owner.
    pub fn update_implementation(
        &mut self,
        code: ZamaniCodeSnippet,
    ) -> Result<(), OopError> {
        if code.trim().is_empty() {
            return Err(OopError::InvalidMetaOperation {
                reason:
                    "implementation update cannot contain empty code"
                        .to_string(),
            });
        }

        Err(OopError::UnsupportedOperation {
            operation:
                "direct object implementation mutation".to_string(),
        })
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier(
            name.to_string(),
            crate::source_map::Span::dummy(),
        )
    }

    #[test]
    fn object_id_is_preserved() {
        let object = ZamaniObject::ClassicalObject(
            identifier("User"),
            Map::new(),
        );

        assert_eq!(object.get_id().0, "User");
    }

    #[test]
    fn object_kind_is_deterministic() {
        let object = ZamaniObject::ClassicalObject(
            identifier("User"),
            Map::new(),
        );

        assert_eq!(object.kind(), "classical");
    }

    #[test]
    fn empty_quantum_reference_is_rejected() {
        assert!(QuantumStateRef::new("").is_err());
    }

    #[test]
    fn empty_nano_reference_is_rejected() {
        assert!(NanoAgentRef::new(" ").is_err());
    }

    #[test]
    fn empty_mts_reference_is_rejected() {
        assert!(MTSStateSnapshot::new("").is_err());
    }

    #[test]
    fn quantum_security_requires_nonzero_strength() {
        let policy = SecurityLevel::QuantumSecured {
            security_bits: 0,
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn redundancy_requires_replica() {
        let policy = SelfHealingPolicy::AdaptiveRedundancy {
            replicas: 0,
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn meta_object_override_is_validated() {
        let protocol = MetaObjectProtocol::new();

        let result = protocol.validate_method_override(
            identifier("object"),
            identifier("method"),
            "impl:1",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn empty_meta_object_override_is_rejected() {
        let protocol = MetaObjectProtocol::new();

        let result = protocol.validate_method_override(
            identifier(""),
            identifier("method"),
            "impl:1",
        );

        assert!(result.is_err());
    }

    #[test]
    fn reflection_is_deterministic() {
        let object = ZamaniObject::ClassicalObject(
            identifier("User"),
            Map::new(),
        );

        let reflection = ObjectReflection::from_object(&object);

        assert_eq!(reflection.object_id, "User");
        assert_eq!(reflection.object_kind, "classical");
    }

    #[test]
    fn encrypted_state_requires_ciphertext_reference() {
        let result = EncryptedObjectState::new(
            identifier("secure"),
            "",
        );

        assert!(result.is_err());
    }

    #[test]
    fn secure_compute_requires_operation() {
        let operations = SecureObjectOperations::new();

        let operation = SecureObjectOperation::Compute {
            ciphertext_ref: "cipher:1".to_string(),
            operation: "".to_string(),
        };

        assert!(operations.validate(&operation).is_err());
    }

    #[test]
    fn implementation_update_never_silently_mutates_code() {
        let mut object = ZamaniObject::ClassicalObject(
            identifier("User"),
            Map::new(),
        );

        let result = object.update_implementation(
            "// replacement",
        );

        assert!(result.is_err());
    }
}