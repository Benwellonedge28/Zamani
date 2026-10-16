
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Logical Address Spaces and Target Bindings.
//!
//! Path: `src/classical/memory/address_space.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Describe logical memory spaces independently of physical hardware and map
//! them to target/backend-specific space identifiers when a target has been
//! selected.
//!
//! # Authority
//!
//! - `grammar/memory/memory.g4` owns memory-space source syntax.
//! - `grammar/memory/allocation.g4` owns allocation-intent syntax.
//! - Semantic analysis owns program validity, ownership, borrowing, aliasing,
//!   and lifetime correctness.
//! - The canonical classical IR and ZUIR own their respective IR identities
//!   and semantic representations.
//! - `src/classical/memory/allocation.rs` owns the classical allocation
//!   request/backend boundary.
//! - The selected runtime/backend owns physical realization.
//! - The normative language specification defines observable semantics.
//!
//! # Owns
//!
//! - Validated, open-world logical address-space identifiers.
//! - Logical address-space descriptors.
//! - Target-specific mappings from logical spaces to backend space names.
//! - Declarative capability requirements for a space and its target binding.
//! - Deterministic registry lookup and structured resolution errors.
//!
//! # Does not own
//!
//! - Grammar, tokens, AST, semantic type systems, or canonical IR.
//! - Physical addresses, pointers, page tables, register addresses, or ABI
//!   layouts.
//! - Allocation, deallocation, memory contents, or allocator accounting.
//! - Device discovery, scheduling, placement policy, or resource provisioning.
//! - Ownership, borrowing, alias analysis, or lifetime verification.
//! - Quantum-specific allocation or quantum-state representations.
//!
//! # POCO-REAF contract
//!
//! The identifiers and descriptors are open-world: new memory spaces,
//! capability names, targets, and backend mappings can be registered without
//! adding enum variants to this module.
//!
//! This module imposes no universal limit on memory-space count, capability
//! count, target count, address width, or memory capacity. The in-memory
//! registry is naturally constrained by available resources and Rust's
//! collection/address-space representations.
//!
//! A logical address space is not proof that a target supports it. Resolution
//! succeeds only when an explicit target binding exists and the caller
//! supplies all required capabilities.
//!
//! # Integration
//!
//! 1. Semantic analysis resolves source-level memory-space references to an
//!    `AddressSpaceId`.
//! 2. Target selection supplies a target identifier and its available
//!    capability identifiers.
//! 3. `AddressSpaceRegistry::resolve` validates the logical space and binding.
//! 4. Lowering/runtime code may pass `ResolvedAddressSpace::backend_space()`
//!    to `AllocationRequest::new` in `allocation.rs`.
//! 5. The allocation backend independently confirms that it supports the
//!    requested backend space.
//!
//! This module does not silently fall back to host memory.
//!
//! # Concurrency and errors
//!
//! Registry operations use a read/write lock. Poisoning is reported through
//! the public `Result` API rather than being silently ignored. Registry
//! writes are atomic at the individual operation level.
//!
//! # Freeze rule
//!
//! Keep this module a declarative address-space registry and resolver. Any
//! future shared classical/quantum memory-space model requires an explicit
//! migration review rather than duplicate competing definitions.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use thiserror::Error;

/// Result type for address-space operations.
pub type AddressSpaceResult<T> = Result<T, AddressSpaceError>;

/// An open-world logical address-space identifier.
///
/// The identifier is semantic data, not a physical address or a backend
/// handle. Qualified and future-defined names are allowed. This type checks
/// only basic identifier hygiene; language-specific name resolution belongs
/// to the frontend and semantic analyzer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressSpaceId(String);

impl AddressSpaceId {
    /// Create a validated identifier.
    ///
    /// Empty names, names with leading/trailing whitespace, and control
    /// characters are rejected. The identifier is otherwise kept unchanged.
    pub fn new(value: impl Into<String>) -> AddressSpaceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "address-space",
                value,
                reason: "identifier must not be empty",
            });
        }

        if value.trim() != value {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "address-space",
                value,
                reason: "leading or trailing whitespace is not permitted",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "address-space",
                value,
                reason: "control characters are not permitted",
            });
        }

        Ok(Self(value))
    }

    /// Return the exact identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AddressSpaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// An open-world target identifier.
///
/// This identifies a target namespace chosen by the compiler/runtime. It does
/// not identify a particular physical device unless the caller's target model
/// explicitly defines it that way.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId(String);

impl TargetId {
    /// Create a validated target identifier.
    pub fn new(value: impl Into<String>) -> AddressSpaceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "target",
                value,
                reason: "identifier must not be empty",
            });
        }

        if value.trim() != value {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "target",
                value,
                reason: "leading or trailing whitespace is not permitted",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "target",
                value,
                reason: "control characters are not permitted",
            });
        }

        Ok(Self(value))
    }

    /// Return the exact target identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TargetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// An open-world capability identifier.
///
/// Capability meanings and availability are supplied by the shared
/// capability/resource system. This module does not define a closed list of
/// hardware features.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Create a validated capability identifier.
    pub fn new(value: impl Into<String>) -> AddressSpaceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "capability",
                value,
                reason: "identifier must not be empty",
            });
        }

        if value.trim() != value {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "capability",
                value,
                reason: "leading or trailing whitespace is not permitted",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(AddressSpaceError::InvalidIdentifier {
                category: "capability",
                value,
                reason: "control characters are not permitted",
            });
        }

        Ok(Self(value))
    }

    /// Return the exact capability identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Declarative description of a logical address space.
///
/// `kind`, `access_modes`, and `coherence` are open-world identifiers rather
/// than exhaustive enums. Their normative meaning belongs to the language
/// specification, shared capability model, or owning dialect.
///
/// `required_capabilities` describes prerequisites; it does not assert that
/// any particular target provides them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressSpaceDescriptor {
    id: AddressSpaceId,
    kind: String,
    coherence: Option<String>,
    access_modes: BTreeSet<String>,
    required_capabilities: BTreeSet<CapabilityId>,
    description: Option<String>,
}

impl AddressSpaceDescriptor {
    /// Construct a logical address-space descriptor.
    ///
    /// `kind` is an extensible semantic category such as a category defined
    /// by the language specification or a registered dialect. It is not a
    /// hardware enum.
    pub fn new(
        id: AddressSpaceId,
        kind: impl Into<String>,
    ) -> AddressSpaceResult<Self> {
        let kind = validate_label("address-space kind", kind.into())?;

        Ok(Self {
            id,
            kind,
            coherence: None,
            access_modes: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            description: None,
        })
    }

    /// Set an optional open-world coherence-model identifier.
    pub fn with_coherence(
        mut self,
        coherence: impl Into<String>,
    ) -> AddressSpaceResult<Self> {
        self.coherence = Some(validate_label(
            "coherence model",
            coherence.into(),
        )?);
        Ok(self)
    }

    /// Add an access-mode identifier.
    ///
    /// This is descriptive metadata, not a substitute for type checking,
    /// authorization, or runtime access enforcement.
    pub fn with_access_mode(
        mut self,
        mode: impl Into<String>,
    ) -> AddressSpaceResult<Self> {
        self.access_modes
            .insert(validate_label("access mode", mode.into())?);
        Ok(self)
    }

    /// Add a capability required to use this logical space.
    #[must_use]
    pub fn requiring(mut self, capability: CapabilityId) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Attach a human-readable description.
    ///
    /// Descriptions are diagnostic metadata and must not be used as semantic
    /// identifiers or as an authorization mechanism.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Logical address-space identifier.
    #[must_use]
    pub fn id(&self) -> &AddressSpaceId {
        &self.id
    }

    /// Open-world semantic category.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Optional coherence-model identifier.
    #[must_use]
    pub fn coherence(&self) -> Option<&str> {
        self.coherence.as_deref()
    }

    /// Declared access-mode identifiers.
    #[must_use]
    pub fn access_modes(&self) -> &BTreeSet<String> {
        &self.access_modes
    }

    /// Required capabilities.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.required_capabilities
    }

    /// Optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Explicit mapping from a logical space to one selected target's backend.
///
/// `backend_space` is an opaque identifier consumed by the selected backend.
/// It is not a pointer, physical address, device handle, or proof that an
/// allocation will succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetAddressSpaceBinding {
    target: TargetId,
    logical_space: AddressSpaceId,
    backend_space: String,
    required_capabilities: BTreeSet<CapabilityId>,
}

impl TargetAddressSpaceBinding {
    /// Construct a target-specific binding.
    pub fn new(
        target: TargetId,
        logical_space: AddressSpaceId,
        backend_space: impl Into<String>,
    ) -> AddressSpaceResult<Self> {
        let backend_space =
            validate_label("backend-space identifier", backend_space.into())?;

        Ok(Self {
            target,
            logical_space,
            backend_space,
            required_capabilities: BTreeSet::new(),
        })
    }

    /// Add a capability required by this particular target mapping.
    #[must_use]
    pub fn requiring(mut self, capability: CapabilityId) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Target to which this binding applies.
    #[must_use]
    pub fn target(&self) -> &TargetId {
        &self.target
    }

    /// Logical address space being mapped.
    #[must_use]
    pub fn logical_space(&self) -> &AddressSpaceId {
        &self.logical_space
    }

    /// Opaque backend-space identifier.
    #[must_use]
    pub fn backend_space(&self) -> &str {
        &self.backend_space
    }

    /// Capabilities required by this binding.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.required_capabilities
    }
}

/// Successful resolution of a logical address space for a target.
///
/// This is an immutable resolution result. The runtime must still verify
/// current resource availability and ask the backend to perform the actual
/// operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAddressSpace {
    target: TargetId,
    descriptor: AddressSpaceDescriptor,
    binding: TargetAddressSpaceBinding,
    required_capabilities: BTreeSet<CapabilityId>,
}

impl ResolvedAddressSpace {
    /// Selected target.
    #[must_use]
    pub fn target(&self) -> &TargetId {
        &self.target
    }

    /// Logical-space descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &AddressSpaceDescriptor {
        &self.descriptor
    }

    /// Target-specific binding.
    #[must_use]
    pub fn binding(&self) -> &TargetAddressSpaceBinding {
        &self.binding
    }

    /// Opaque backend-space identifier suitable for an allocation request.
    #[must_use]
    pub fn backend_space(&self) -> &str {
        self.binding.backend_space()
    }

    /// All capabilities required by both the logical space and binding.
    #[must_use]
    pub fn required_capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.required_capabilities
    }
}

/// Errors raised while declaring or resolving address spaces.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AddressSpaceError {
    /// An identifier or metadata label failed basic validation.
    #[error("invalid {category} identifier {value:?}: {reason}")]
    InvalidIdentifier {
        /// Kind of identifier being validated.
        category: &'static str,
        /// Supplied identifier.
        value: String,
        /// Validation failure.
        reason: &'static str,
    },

    /// A logical address space was already declared.
    #[error("address space {0} is already registered")]
    DuplicateAddressSpace(AddressSpaceId),

    /// A binding for this target and logical space already exists.
    #[error("address space {space} is already bound for target {target}")]
    DuplicateBinding {
        /// Target identifier.
        target: TargetId,
        /// Logical address-space identifier.
        space: AddressSpaceId,
    },

    /// No logical address-space declaration exists.
    #[error("address space {0} is not registered")]
    UnknownAddressSpace(AddressSpaceId),

    /// No binding exists for this logical space on the selected target.
    #[error("address space {space} has no binding for target {target}")]
    UnboundAddressSpace {
        /// Target identifier.
        target: TargetId,
        /// Logical address-space identifier.
        space: AddressSpaceId,
    },

    /// Required capabilities are missing from the target capability set.
    #[error(
        "target {target} cannot resolve address space {space}; \
         missing capabilities: {missing:?}"
    )]
    MissingCapabilities {
        /// Target identifier.
        target: TargetId,
        /// Logical address-space identifier.
        space: AddressSpaceId,
        /// Missing capability identifiers, in deterministic order.
        missing: Vec<CapabilityId>,
    },

    /// The registry lock was poisoned by a panicking writer.
    #[error("address-space registry lock is poisoned")]
    RegistryPoisoned,
}

/// Thread-safe registry of logical spaces and explicit target bindings.
///
/// The registry deliberately does not discover targets or infer mappings.
/// Callers register descriptors and bindings based on authoritative language,
/// target, and capability metadata.
#[derive(Debug, Default)]
pub struct AddressSpaceRegistry {
    spaces: RwLock<BTreeMap<AddressSpaceId, AddressSpaceDescriptor>>,
    bindings: RwLock<BTreeMap<(TargetId, AddressSpaceId), TargetAddressSpaceBinding>>,
}

impl AddressSpaceRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a logical address-space descriptor.
    ///
    /// Duplicate declarations fail rather than silently replacing metadata.
    pub fn register_space(
        &self,
        descriptor: AddressSpaceDescriptor,
    ) -> AddressSpaceResult<()> {
        let mut spaces = self.write_spaces()?;
        let id = descriptor.id().clone();

        if spaces.contains_key(&id) {
            return Err(AddressSpaceError::DuplicateAddressSpace(id));
        }

        spaces.insert(id, descriptor);
        Ok(())
    }

    /// Register an explicit target mapping for a declared logical space.
    ///
    /// The logical space must already exist. A target mapping is not
    /// automatically inherited by other targets.
    pub fn bind_target(
        &self,
        binding: TargetAddressSpaceBinding,
    ) -> AddressSpaceResult<()> {
        let spaces = self.read_spaces()?;

        if !spaces.contains_key(binding.logical_space()) {
            return Err(AddressSpaceError::UnknownAddressSpace(
                binding.logical_space().clone(),
            ));
        }

        let key = (binding.target().clone(), binding.logical_space().clone());
        drop(spaces);

        let mut bindings = self.write_bindings()?;

        if bindings.contains_key(&key) {
            return Err(AddressSpaceError::DuplicateBinding {
                target: key.0,
                space: key.1,
            });
        }

        bindings.insert(key, binding);
        Ok(())
    }

    /// Return a registered logical-space descriptor, if present.
    pub fn get(
        &self,
        id: &AddressSpaceId,
    ) -> AddressSpaceResult<Option<AddressSpaceDescriptor>> {
        Ok(self.read_spaces()?.get(id).cloned())
    }

    /// Resolve a logical space for a target and its currently available
    /// capabilities.
    ///
    /// Resolution requires an explicit binding and the union of the
    /// descriptor's and binding's capability requirements. It does not
    /// reserve resources or guarantee future availability.
    pub fn resolve(
        &self,
        target: &TargetId,
        space: &AddressSpaceId,
        available_capabilities: &BTreeSet<CapabilityId>,
    ) -> AddressSpaceResult<ResolvedAddressSpace> {
        let descriptor = self
            .read_spaces()?
            .get(space)
            .cloned()
            .ok_or_else(|| AddressSpaceError::UnknownAddressSpace(space.clone()))?;

        let key = (target.clone(), space.clone());
        let binding = self
            .read_bindings()?
            .get(&key)
            .cloned()
            .ok_or_else(|| AddressSpaceError::UnboundAddressSpace {
                target: target.clone(),
                space: space.clone(),
            })?;

        let mut required_capabilities =
            descriptor.required_capabilities().clone();
        required_capabilities.extend(binding.required_capabilities().iter().cloned());

        let missing: Vec<_> = required_capabilities
            .difference(available_capabilities)
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(AddressSpaceError::MissingCapabilities {
                target: target.clone(),
                space: space.clone(),
                missing,
            });
        }

        Ok(ResolvedAddressSpace {
            target: target.clone(),
            descriptor,
            binding,
            required_capabilities,
        })
    }

    /// Number of logical address-space descriptors currently registered.
    pub fn space_count(&self) -> AddressSpaceResult<usize> {
        Ok(self.read_spaces()?.len())
    }

    /// Number of explicit target bindings currently registered.
    pub fn binding_count(&self) -> AddressSpaceResult<usize> {
        Ok(self.read_bindings()?.len())
    }

    fn read_spaces(
        &self,
    ) -> AddressSpaceResult<RwLockReadGuard<'_, BTreeMap<AddressSpaceId, AddressSpaceDescriptor>>>
    {
        self.spaces
            .read()
            .map_err(|_| AddressSpaceError::RegistryPoisoned)
    }

    fn write_spaces(
        &self,
    ) -> AddressSpaceResult<RwLockWriteGuard<'_, BTreeMap<AddressSpaceId, AddressSpaceDescriptor>>>
    {
        self.spaces
            .write()
            .map_err(|_| AddressSpaceError::RegistryPoisoned)
    }

    fn read_bindings(
        &self,
    ) -> AddressSpaceResult<
        RwLockReadGuard<'_, BTreeMap<(TargetId, AddressSpaceId), TargetAddressSpaceBinding>>,
    > {
        self.bindings
            .read()
            .map_err(|_| AddressSpaceError::RegistryPoisoned)
    }

    fn write_bindings(
        &self,
    ) -> AddressSpaceResult<
        RwLockWriteGuard<'_, BTreeMap<(TargetId, AddressSpaceId), TargetAddressSpaceBinding>>,
    > {
        self.bindings
            .write()
            .map_err(|_| AddressSpaceError::RegistryPoisoned)
    }
}

fn validate_label(category: &'static str, value: String) -> AddressSpaceResult<String> {
    if value.is_empty() {
        return Err(AddressSpaceError::InvalidIdentifier {
            category,
            value,
            reason: "value must not be empty",
        });
    }

    if value.trim() != value {
        return Err(AddressSpaceError::InvalidIdentifier {
            category,
            value,
            reason: "leading or trailing whitespace is not permitted",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AddressSpaceError::InvalidIdentifier {
            category,
            value,
            reason: "control characters are not permitted",
        });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> AddressSpaceId {
        AddressSpaceId::new(value).expect("valid address-space identifier")
    }

    fn target(value: &str) -> TargetId {
        TargetId::new(value).expect("valid target identifier")
    }

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability identifier")
    }

    fn descriptor(space: &str) -> AddressSpaceDescriptor {
        AddressSpaceDescriptor::new(id(space), "logical")
            .expect("valid descriptor")
    }

    #[test]
    fn accepts_open_world_identifiers() {
        let space = id("vendor.future.memory-domain.v3");
        let target = target("architecture.new-family");
        let capability = capability("memory.remote-coherent");

        assert_eq!(space.as_str(), "vendor.future.memory-domain.v3");
        assert_eq!(target.as_str(), "architecture.new-family");
        assert_eq!(capability.as_str(), "memory.remote-coherent");
    }

    #[test]
    fn rejects_empty_and_unsafe_to_display_identifiers() {
        assert!(AddressSpaceId::new("").is_err());
        assert!(AddressSpaceId::new(" host").is_err());
        assert!(AddressSpaceId::new("host ").is_err());
        assert!(AddressSpaceId::new("host\nspace").is_err());
        assert!(TargetId::new("").is_err());
        assert!(CapabilityId::new(" ").is_err());
    }

    #[test]
    fn registers_and_resolves_an_explicit_mapping() {
        let registry = AddressSpaceRegistry::new();
        let space = id("memory.shared");
        let target = target("target.test");

        registry
            .register_space(descriptor(space.as_str()))
            .expect("register logical space");

        registry
            .bind_target(
                TargetAddressSpaceBinding::new(
                    target.clone(),
                    space.clone(),
                    "backend.shared",
                )
                .expect("valid target binding"),
            )
            .expect("register binding");

        let resolved = registry
            .resolve(&target, &space, &BTreeSet::new())
            .expect("resolve target mapping");

        assert_eq!(resolved.backend_space(), "backend.shared");
        assert_eq!(resolved.descriptor().id(), &space);
        assert_eq!(resolved.target(), &target);
    }

    #[test]
    fn rejects_duplicate_space_declarations() {
        let registry = AddressSpaceRegistry::new();
        registry
            .register_space(descriptor("memory.shared"))
            .expect("first registration");

        assert!(matches!(
            registry.register_space(descriptor("memory.shared")),
            Err(AddressSpaceError::DuplicateAddressSpace(_))
        ));
    }

    #[test]
    fn rejects_binding_to_an_undeclared_space() {
        let registry = AddressSpaceRegistry::new();
        let binding = TargetAddressSpaceBinding::new(
            target("target.test"),
            id("memory.unknown"),
            "backend.unknown",
        )
        .expect("binding construction");

        assert!(matches!(
            registry.bind_target(binding),
            Err(AddressSpaceError::UnknownAddressSpace(_))
        ));
    }

    #[test]
    fn does_not_infer_a_missing_target_binding() {
        let registry = AddressSpaceRegistry::new();
        let space = id("memory.shared");

        registry
            .register_space(descriptor(space.as_str()))
            .expect("register space");

        assert!(matches!(
            registry.resolve(
                &target("target.test"),
                &space,
                &BTreeSet::new()
            ),
            Err(AddressSpaceError::UnboundAddressSpace { .. })
        ));
    }

    #[test]
    fn requires_capabilities_from_both_descriptor_and_binding() {
        let registry = AddressSpaceRegistry::new();
        let space = id("memory.remote");
        let target = target("target.remote");
        let logical_requirement = capability("memory.remote-access");
        let binding_requirement = capability("memory.coherent-access");

        let descriptor = AddressSpaceDescriptor::new(
            space.clone(),
            "remote",
        )
        .expect("descriptor")
        .requiring(logical_requirement.clone());

        registry
            .register_space(descriptor)
            .expect("register space");

        registry
            .bind_target(
                TargetAddressSpaceBinding::new(
                    target.clone(),
                    space.clone(),
                    "provider.remote",
                )
                .expect("binding")
                .requiring(binding_requirement.clone()),
            )
            .expect("register binding");

        let available = BTreeSet::from([logical_requirement.clone()]);

        assert!(matches!(
            registry.resolve(&target, &space, &available),
            Err(AddressSpaceError::MissingCapabilities { .. })
        ));

        let available = BTreeSet::from([
            logical_requirement,
            binding_requirement,
        ]);

        let resolved = registry
            .resolve(&target, &space, &available)
            .expect("all capabilities supplied");

        assert_eq!(resolved.required_capabilities().len(), 2);
    }

    #[test]
    fn does_not_change_a_logical_space_when_binding_another_target() {
        let registry = AddressSpaceRegistry::new();
        let space = id("memory.shared");

        registry
            .register_space(descriptor(space.as_str()))
            .expect("register space");

        for (target_name, backend_name) in [
            ("target.a", "backend.a"),
            ("target.b", "backend.b"),
        ] {
            registry
                .bind_target(
                    TargetAddressSpaceBinding::new(
                        target(target_name),
                        space.clone(),
                        backend_name,
                    )
                    .expect("binding"),
                )
                .expect("register target binding");
        }

        let a = registry
            .resolve(&target("target.a"), &space, &BTreeSet::new())
            .expect("resolve target a");
        let b = registry
            .resolve(&target("target.b"), &space, &BTreeSet::new())
            .expect("resolve target b");

        assert_eq!(a.backend_space(), "backend.a");
        assert_eq!(b.backend_space(), "backend.b");
    }

    #[test]
    fn exposes_deterministic_counts() {
        let registry = AddressSpaceRegistry::new();

        registry
            .register_space(descriptor("memory.host"))
            .expect("register first");
        registry
            .register_space(descriptor("memory.remote"))
            .expect("register second");

        assert_eq!(registry.space_count().expect("space count"), 2);
        assert_eq!(registry.binding_count().expect("binding count"), 0);
    }

    #[test]
    fn descriptors_preserve_extensible_metadata() {
        let capability = capability("memory.atomic");
        let descriptor = AddressSpaceDescriptor::new(
            id("memory.custom"),
            "vendor.custom-kind",
        )
        .expect("descriptor")
        .with_coherence("vendor.coherence-model")
        .expect("coherence")
        .with_access_mode("read-write")
        .expect("access mode")
        .requiring(capability.clone())
        .with_description("Registered by a target extension");

        assert_eq!(descriptor.kind(), "vendor.custom-kind");
        assert_eq!(
            descriptor.coherence(),
            Some("vendor.coherence-model")
        );
        assert!(descriptor.access_modes().contains("read-write"));
        assert!(descriptor.required_capabilities().contains(&capability));
        assert_eq!(
            descriptor.description(),
            Some("Registered by a target extension")
        );
    }
}
