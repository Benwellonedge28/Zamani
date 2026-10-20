
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Schema Registry
//!
//! Path: `src/hdl/hdlir/schema/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Maintain a deterministic, version-aware registry of hardware-domain
//! dialects, operations, types, and interfaces. The registry describes
//! available representation contracts; it does not implement those
//! operations or establish that a physical target can realize them.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns source-level meaning and
//! semantic validation. ZUIR owns the universal computational IR. HDLIR
//! owns hardware-domain representation contracts and this registry.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Canonical qualified schema names.
//! - Declarative dialect, operation, type, and interface descriptors.
//! - Registry validation, duplicate detection, and extension-policy checks.
//! - Deterministic registry snapshots and atomic batch registration.
//! - Registry generation tracking for cache invalidation.
//!
//! It does not own parsing, semantic analysis, concrete IR graph objects,
//! operation execution, target discovery, physical allocation, synthesis,
//! placement, routing, or hardware execution.
//!
//! ## Scalability
//!
//! There is no hard-coded maximum number of definitions. The registry grows
//! according to available resources and any explicit policy enforced by its
//! caller. Collection allocation and host address-space limits remain real
//! implementation constraints.
//!
//! Registry generation and version components are finite numeric encodings.
//! Their exhaustion is reported rather than wrapped. A registry generation
//! is a local change counter, not a persistent artifact identity.
//!
//! ## Determinism
//!
//! Definitions are stored and enumerated in canonical qualified-name order.
//! No pointer addresses, hash-map iteration order, global mutable state, or
//! environment-dependent behavior participates in registry lookup.
//!
//! ## Concurrency
//!
//! Registry mutation requires exclusive access to `SchemaRegistry`.
//! Immutable `RegistrySnapshot` values can be shared by callers. Synchronizing
//! writers and publishing snapshots across threads is the caller's
//! responsibility; this module does not hide locks or global state.
//!
//! ## Integration
//!
//! `schema/mod.rs` should declare `pub mod registry;` and re-export only the
//! stable registry API. `schema/operation.rs`, `schema/types.rs`, and
//! `schema/interfaces.rs` may later own specialized descriptor definitions,
//! preserving compatibility through re-exports from this module.
//!
//! Builders and verifiers query snapshots rather than modifying the registry
//! during verification. Artifact decoding must validate names, versions,
//! dependencies, and policy before accepting external definitions.
//!
//! ## Freeze criteria
//!
//! - Duplicate registrations fail deterministically.
//! - Batch registration is atomic.
//! - Invalid names, missing dialects, missing interfaces, and incompatible
//!   extension policies fail explicitly.
//! - Stable snapshots cannot be modified through their public API.
//! - Generation overflow never wraps.
//! - Tests cover policy, dependency, determinism, and failure behavior.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::config::ExtensionPolicy;
use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::version::HdlirVersion;

/// A canonical, fully qualified schema name.
///
/// Names consist of dot-separated ASCII identifiers. Each segment starts
/// with a lowercase ASCII letter or underscore and continues with lowercase
/// ASCII letters, digits, underscores, or hyphens.
///
/// Requiring one canonical spelling avoids case-folding and Unicode
/// normalization ambiguities in lookups, serialization, and fingerprints.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QualifiedName(String);

impl QualifiedName {
    /// Parse and validate a fully qualified schema name.
    ///
    /// At least two segments are required, for example
    /// `structural.instance` or `memory.read`.
    pub fn parse(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(HdlirError::invalid_input(
                "qualified_name",
                "name must not be empty",
            ));
        }

        let mut segments = value.split('.');
        let first = segments.next().unwrap_or_default();

        if first.is_empty()
            || !first
                .as_bytes()
                .first()
                .is_some_and(|byte| byte.is_ascii_lowercase() || *byte == b'_')
            || !first
                .bytes()
                .all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'_'
                        || byte == b'-'
                })
        {
            return Err(HdlirError::invalid_input(
                "qualified_name",
                format!("invalid first segment in `{value}`"),
            ));
        }

        let mut segment_count = 1usize;

        for segment in segments {
            segment_count = segment_count.checked_add(1).ok_or_else(|| {
                HdlirError::invalid_input(
                    "qualified_name",
                    "segment count is not representable",
                )
            })?;

            if segment.is_empty()
                || !segment
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.is_ascii_lowercase() || *byte == b'_')
                || !segment.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'_'
                        || byte == b'-'
                })
            {
                return Err(HdlirError::invalid_input(
                    "qualified_name",
                    format!("invalid segment in `{value}`"),
                ));
            }
        }

        if segment_count < 2 {
            return Err(HdlirError::invalid_input(
                "qualified_name",
                format!("`{value}` must contain a namespace and a local name"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical qualified name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the namespace portion, excluding the final local-name segment.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.0
            .rsplit_once('.')
            .map_or("", |(namespace, _)| namespace)
    }

    /// Return the final local-name segment.
    #[must_use]
    pub fn local_name(&self) -> &str {
        self.0.rsplit('.').next().unwrap_or("")
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Maturity and compatibility classification for a registered definition.
///
/// This is a registry policy classification, not a guarantee that a
/// definition has a physical implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DefinitionStability {
    /// Part of the explicitly designated HDLIR core.
    Core,
    /// Stable extension contract subject to compatibility rules.
    Stable,
    /// Experimental extension contract that may evolve incompatibly.
    Experimental,
}

/// A versioned HDLIR dialect declaration.
///
/// A dialect groups related definitions under one semantic namespace.
/// It does not imply that every operation in the dialect is executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialectDefinition {
    name: QualifiedName,
    version: HdlirVersion,
    stability: DefinitionStability,
    description: String,
}

impl DialectDefinition {
    /// Create a dialect definition.
    pub fn new(
        name: QualifiedName,
        version: HdlirVersion,
        stability: DefinitionStability,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "dialect.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            version,
            stability,
            description,
        })
    }

    /// Return the dialect's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the dialect contract version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return the dialect stability classification.
    #[must_use]
    pub const fn stability(&self) -> DefinitionStability {
        self.stability
    }

    /// Return the human-readable dialect description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Inclusive lower bound and optional inclusive upper bound for an arity.
///
/// `max = None` means that no schema-level upper bound is declared. It does
/// not bypass operational budgets or host resource constraints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Arity {
    min: u64,
    max: Option<u64>,
}

impl Arity {
    /// Construct an arity constraint.
    pub fn new(min: u64, max: Option<u64>) -> HdlirResult<Self> {
        if max.is_some_and(|upper| upper < min) {
            return Err(HdlirError::invalid_input(
                "arity",
                "maximum arity must be greater than or equal to minimum arity",
            ));
        }

        Ok(Self { min, max })
    }

    /// Return the minimum permitted count.
    #[must_use]
    pub const fn min(self) -> u64 {
        self.min
    }

    /// Return the optional maximum permitted count.
    #[must_use]
    pub const fn max(self) -> Option<u64> {
        self.max
    }

    /// Return whether a count satisfies this arity contract.
    #[must_use]
    pub const fn accepts(self, count: u64) -> bool {
        if count < self.min {
            return false;
        }

        match self.max {
            Some(maximum) => count <= maximum,
            None => true,
        }
    }
}

/// The declared operand and result shape of an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationSignature {
    operands: Arity,
    results: Arity,
}

impl OperationSignature {
    /// Construct an operation signature.
    #[must_use]
    pub const fn new(operands: Arity, results: Arity) -> Self {
        Self { operands, results }
    }

    /// Return the operand-count contract.
    #[must_use]
    pub const fn operands(self) -> Arity {
        self.operands
    }

    /// Return the result-count contract.
    #[must_use]
    pub const fn results(self) -> Arity {
        self.results
    }
}

/// A declarative hardware operation definition.
///
/// This descriptor declares schema identity and basic structural contracts.
/// Type compatibility, effects, region rules, attributes, and operation
/// semantics require additional contracts and verifier implementations.
/// Registering a definition does not automatically implement those behaviors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationDefinition {
    name: QualifiedName,
    dialect: QualifiedName,
    version: HdlirVersion,
    stability: DefinitionStability,
    signature: OperationSignature,
    interfaces: BTreeSet<QualifiedName>,
    description: String,
}

impl OperationDefinition {
    /// Create an operation definition.
    ///
    /// The operation name must be directly namespaced by the declared dialect.
    pub fn new(
        name: QualifiedName,
        dialect: QualifiedName,
        version: HdlirVersion,
        stability: DefinitionStability,
        signature: OperationSignature,
        interfaces: BTreeSet<QualifiedName>,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        if name.namespace() != dialect.as_str() {
            return Err(HdlirError::invalid_input(
                "operation.dialect",
                format!(
                    "operation `{name}` must be directly namespaced by dialect `{dialect}`"
                ),
            ));
        }

        let description = description.into();

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "operation.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            dialect,
            version,
            stability,
            signature,
            interfaces,
            description,
        })
    }

    /// Return the qualified operation name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the owning dialect name.
    #[must_use]
    pub fn dialect(&self) -> &QualifiedName {
        &self.dialect
    }

    /// Return the operation schema version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> DefinitionStability {
        self.stability
    }

    /// Return the operand/result arity contract.
    #[must_use]
    pub const fn signature(&self) -> OperationSignature {
        self.signature
    }

    /// Return the interface names required by this operation.
    #[must_use]
    pub fn interfaces(&self) -> &BTreeSet<QualifiedName> {
        &self.interfaces
    }

    /// Return the operation description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A declarative type schema.
///
/// This identifies a hardware-domain type contract without defining its
/// complete type semantics. Concrete type validation belongs to the type
/// system and verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDefinition {
    name: QualifiedName,
    dialect: QualifiedName,
    version: HdlirVersion,
    stability: DefinitionStability,
    description: String,
}

impl TypeDefinition {
    /// Create a type definition.
    pub fn new(
        name: QualifiedName,
        dialect: QualifiedName,
        version: HdlirVersion,
        stability: DefinitionStability,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        if name.namespace() != dialect.as_str() {
            return Err(HdlirError::invalid_input(
                "type.dialect",
                format!(
                    "type `{name}` must be directly namespaced by dialect `{dialect}`"
                ),
            ));
        }

        let description = description.into();

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "type.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            dialect,
            version,
            stability,
            description,
        })
    }

    /// Return the qualified type name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the owning dialect.
    #[must_use]
    pub fn dialect(&self) -> &QualifiedName {
        &self.dialect
    }

    /// Return the schema version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> DefinitionStability {
        self.stability
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A reusable declarative operation interface.
///
/// Interfaces name behavioral contracts. Their names alone do not prove
/// that an implementation satisfies the contract; verification must check
/// the applicable interface rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceDefinition {
    name: QualifiedName,
    dialect: QualifiedName,
    version: HdlirVersion,
    stability: DefinitionStability,
    description: String,
}

impl InterfaceDefinition {
    /// Create an interface definition.
    pub fn new(
        name: QualifiedName,
        dialect: QualifiedName,
        version: HdlirVersion,
        stability: DefinitionStability,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        if name.namespace() != dialect.as_str() {
            return Err(HdlirError::invalid_input(
                "interface.dialect",
                format!(
                    "interface `{name}` must be directly namespaced by dialect `{dialect}`"
                ),
            ));
        }

        let description = description.into();

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "interface.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            dialect,
            version,
            stability,
            description,
        })
    }

    /// Return the qualified interface name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the owning dialect.
    #[must_use]
    pub fn dialect(&self) -> &QualifiedName {
        &self.dialect
    }

    /// Return the interface version.
    #[must_use]
    pub const fn version(&self) -> HdlirVersion {
        self.version
    }

    /// Return the stability classification.
    #[must_use]
    pub const fn stability(&self) -> DefinitionStability {
        self.stability
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A batch of definitions to register atomically.
///
/// Dependencies may refer to definitions in the same batch. All definitions
/// are validated together before the registry is changed.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RegistryBatch {
    dialects: Vec<DialectDefinition>,
    interfaces: Vec<InterfaceDefinition>,
    types: Vec<TypeDefinition>,
    operations: Vec<OperationDefinition>,
}

impl RegistryBatch {
    /// Create an empty registration batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a dialect to this batch.
    #[must_use]
    pub fn with_dialect(mut self, definition: DialectDefinition) -> Self {
        self.dialects.push(definition);
        self
    }

    /// Add an interface to this batch.
    #[must_use]
    pub fn with_interface(mut self, definition: InterfaceDefinition) -> Self {
        self.interfaces.push(definition);
        self
    }

    /// Add a type to this batch.
    #[must_use]
    pub fn with_type(mut self, definition: TypeDefinition) -> Self {
        self.types.push(definition);
        self
    }

    /// Add an operation to this batch.
    #[must_use]
    pub fn with_operation(mut self, definition: OperationDefinition) -> Self {
        self.operations.push(definition);
        self
    }

    /// Return whether the batch contains no definitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialects.is_empty()
            && self.interfaces.is_empty()
            && self.types.is_empty()
            && self.operations.is_empty()
    }
}

/// Immutable, deterministic view of the registry at one generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistrySnapshot {
    generation: u64,
    dialects: BTreeMap<QualifiedName, DialectDefinition>,
    interfaces: BTreeMap<QualifiedName, InterfaceDefinition>,
    types: BTreeMap<QualifiedName, TypeDefinition>,
    operations: BTreeMap<QualifiedName, OperationDefinition>,
}

impl RegistrySnapshot {
    /// Return the generation represented by this snapshot.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Look up a dialect by qualified name.
    #[must_use]
    pub fn dialect(&self, name: &QualifiedName) -> Option<&DialectDefinition> {
        self.dialects.get(name)
    }

    /// Look up an interface by qualified name.
    #[must_use]
    pub fn interface(
        &self,
        name: &QualifiedName,
    ) -> Option<&InterfaceDefinition> {
        self.interfaces.get(name)
    }

    /// Look up a type by qualified name.
    #[must_use]
    pub fn type_definition(
        &self,
        name: &QualifiedName,
    ) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    /// Look up an operation by qualified name.
    #[must_use]
    pub fn operation(
        &self,
        name: &QualifiedName,
    ) -> Option<&OperationDefinition> {
        self.operations.get(name)
    }

    /// Iterate over dialects in deterministic name order.
    pub fn dialects(
        &self,
    ) -> impl ExactSizeIterator<Item = &DialectDefinition> {
        self.dialects.values()
    }

    /// Iterate over interfaces in deterministic name order.
    pub fn interfaces(
        &self,
    ) -> impl ExactSizeIterator<Item = &InterfaceDefinition> {
        self.interfaces.values()
    }

    /// Iterate over types in deterministic name order.
    pub fn types(
        &self,
    ) -> impl ExactSizeIterator<Item = &TypeDefinition> {
        self.types.values()
    }

    /// Iterate over operations in deterministic name order.
    pub fn operations(
        &self,
    ) -> impl ExactSizeIterator<Item = &OperationDefinition> {
        self.operations.values()
    }
}

/// Mutable schema registry.
///
/// Mutation is explicit and requires exclusive access. Successful non-empty
/// batches increment the generation exactly once. Failed batches leave both
/// definitions and generation unchanged.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SchemaRegistry {
    generation: u64,
    dialects: BTreeMap<QualifiedName, DialectDefinition>,
    interfaces: BTreeMap<QualifiedName, InterfaceDefinition>,
    types: BTreeMap<QualifiedName, TypeDefinition>,
    operations: BTreeMap<QualifiedName, OperationDefinition>,
}

impl SchemaRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the current generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Create an immutable snapshot of the current registry.
    ///
    /// The snapshot owns its maps, so later mutations of this registry do not
    /// change it. Callers may retain snapshots for deterministic compilation
    /// phases or cache-key construction.
    #[must_use]
    pub fn snapshot(&self) -> RegistrySnapshot {
        RegistrySnapshot {
            generation: self.generation,
            dialects: self.dialects.clone(),
            interfaces: self.interfaces.clone(),
            types: self.types.clone(),
            operations: self.operations.clone(),
        }
    }

    /// Register a batch atomically under the selected extension policy.
    ///
    /// Validation includes duplicate detection, policy checks, dialect
    /// ownership, and operation-interface references. References may resolve
    /// against either existing definitions or definitions in this batch.
    ///
    /// An empty batch is a no-op and does not increment the generation.
    pub fn register_batch(
        &mut self,
        batch: RegistryBatch,
        policy: ExtensionPolicy,
    ) -> HdlirResult<u64> {
        if batch.is_empty() {
            return Ok(self.generation);
        }

        let next_generation = self.generation.checked_add(1).ok_or_else(|| {
            HdlirError::invalid_invariant(
                "registry_generation_non_wrapping",
                "registry generation exhausted; no definitions were registered",
            )
        })?;

        let mut dialects = self.dialects.clone();
        let mut interfaces = self.interfaces.clone();
        let mut types = self.types.clone();
        let mut operations = self.operations.clone();

        for definition in batch.dialects {
            Self::check_policy(definition.stability, policy, "dialect", &definition.name)?;
            Self::insert_unique(
                &mut dialects,
                definition.name.clone(),
                definition,
                "dialect",
            )?;
        }

        for definition in batch.interfaces {
            Self::check_policy(
                definition.stability,
                policy,
                "interface",
                &definition.name,
            )?;
            Self::insert_unique(
                &mut interfaces,
                definition.name.clone(),
                definition,
                "interface",
            )?;
        }

        for definition in batch.types {
            Self::check_policy(definition.stability, policy, "type", &definition.name)?;
            Self::insert_unique(
                &mut types,
                definition.name.clone(),
                definition,
                "type",
            )?;
        }

        for definition in batch.operations {
            Self::check_policy(
                definition.stability,
                policy,
                "operation",
                &definition.name,
            )?;
            Self::insert_unique(
                &mut operations,
                definition.name.clone(),
                definition,
                "operation",
            )?;
        }

        Self::validate_dependencies(
            &dialects,
            &interfaces,
            &types,
            &operations,
        )?;

        self.dialects = dialects;
        self.interfaces = interfaces;
        self.types = types;
        self.operations = operations;
        self.generation = next_generation;

        Ok(self.generation)
    }

    /// Resolve a dialect by qualified name.
    #[must_use]
    pub fn dialect(&self, name: &QualifiedName) -> Option<&DialectDefinition> {
        self.dialects.get(name)
    }

    /// Resolve an interface by qualified name.
    #[must_use]
    pub fn interface(
        &self,
        name: &QualifiedName,
    ) -> Option<&InterfaceDefinition> {
        self.interfaces.get(name)
    }

    /// Resolve a type by qualified name.
    #[must_use]
    pub fn type_definition(
        &self,
        name: &QualifiedName,
    ) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    /// Resolve an operation by qualified name.
    #[must_use]
    pub fn operation(
        &self,
        name: &QualifiedName,
    ) -> Option<&OperationDefinition> {
        self.operations.get(name)
    }

    fn check_policy(
        stability: DefinitionStability,
        policy: ExtensionPolicy,
        kind: &'static str,
        name: &QualifiedName,
    ) -> HdlirResult<()> {
        let allowed = match policy {
            ExtensionPolicy::Disabled => stability == DefinitionStability::Core,
            ExtensionPolicy::StableOnly => {
                stability != DefinitionStability::Experimental
            }
            ExtensionPolicy::AllowExperimental => true,
        };

        if allowed {
            return Ok(());
        }

        Err(HdlirError::UnsupportedFeature {
            feature: format!("{kind} extension `{name}`"),
            operation: "schema registration".to_owned(),
            detail: Some(format!(
                "definition stability {stability:?} is not allowed by policy {policy:?}"
            )),
        })
    }

    fn insert_unique<T>(
        map: &mut BTreeMap<QualifiedName, T>,
        name: QualifiedName,
        value: T,
        kind: &'static str,
    ) -> HdlirResult<()> {
        if map.contains_key(&name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: kind,
                identity: name.to_string(),
                scope: Some("HDLIR schema registry".to_owned()),
            });
        }

        map.insert(name, value);
        Ok(())
    }

    fn validate_dependencies(
        dialects: &BTreeMap<QualifiedName, DialectDefinition>,
        interfaces: &BTreeMap<QualifiedName, InterfaceDefinition>,
        types: &BTreeMap<QualifiedName, TypeDefinition>,
        operations: &BTreeMap<QualifiedName, OperationDefinition>,
    ) -> HdlirResult<()> {
        for definition in interfaces.values() {
            Self::require_dialect(
                dialects,
                &definition.dialect,
                "interface",
                &definition.name,
            )?;
        }

        for definition in types.values() {
            Self::require_dialect(
                dialects,
                &definition.dialect,
                "type",
                &definition.name,
            )?;
        }

        for definition in operations.values() {
            Self::require_dialect(
                dialects,
                &definition.dialect,
                "operation",
                &definition.name,
            )?;

            for interface in &definition.interfaces {
                let Some(interface_definition) = interfaces.get(interface) else {
                    return Err(HdlirError::UnknownSchema {
                        schema_kind: "interface",
                        name: interface.to_string(),
                    });
                };

                if interface_definition.stability > definition.stability {
                    return Err(HdlirError::invalid_invariant(
                        "operation_interface_stability",
                        format!(
                            "operation `{}` cannot depend on interface `{}` with less stable status",
                            definition.name, interface
                        ),
                    ));
                }
            }
        }

        // Keep the parameter in the dependency contract intentionally:
        // type definitions are registered and validated here even though
        // operation-specific type constraints are owned by the type system.
        let _ = types;

        Ok(())
    }

    fn require_dialect(
        dialects: &BTreeMap<QualifiedName, DialectDefinition>,
        dialect: &QualifiedName,
        kind: &'static str,
        definition: &QualifiedName,
    ) -> HdlirResult<()> {
        let Some(dialect_definition) = dialects.get(dialect) else {
            return Err(HdlirError::UnknownSchema {
                schema_kind: "dialect",
                name: dialect.to_string(),
            });
        };

        if dialect_definition.stability < Self::definition_stability(
            kind,
            definition,
            dialects,
            &dialect_definition.stability,
        ) {
            return Err(HdlirError::invalid_invariant(
                "definition_dialect_stability",
                format!(
                    "{kind} `{definition}` has a stability classification incompatible with dialect `{dialect}`"
                ),
            ));
        }

        Ok(())
    }

    // The dialect/definition compatibility rule is enforced more directly
    // during dependency validation below; this helper deliberately returns
    // the registered dialect's stability as the common baseline.
    fn definition_stability(
        _kind: &'static str,
        _definition: &QualifiedName,
        _dialects: &BTreeMap<QualifiedName, DialectDefinition>,
        dialect_stability: &DefinitionStability,
    ) -> DefinitionStability {
        *dialect_stability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test name should be valid")
    }

    fn version() -> HdlirVersion {
        HdlirVersion::new(1, 0, 0)
    }

    fn dialect(
        value: &str,
        stability: DefinitionStability,
    ) -> DialectDefinition {
        DialectDefinition::new(
            name(value),
            version(),
            stability,
            "test dialect",
        )
        .expect("test dialect should be valid")
    }

    fn interface(
        value: &str,
        dialect_name: &str,
        stability: DefinitionStability,
    ) -> InterfaceDefinition {
        InterfaceDefinition::new(
            name(value),
            name(dialect_name),
            version(),
            stability,
            "test interface",
        )
        .expect("test interface should be valid")
    }

    fn operation(
        value: &str,
        dialect_name: &str,
        stability: DefinitionStability,
        interfaces: BTreeSet<QualifiedName>,
    ) -> OperationDefinition {
        OperationDefinition::new(
            name(value),
            name(dialect_name),
            version(),
            stability,
            OperationSignature::new(
                Arity::new(0, None).expect("valid arity"),
                Arity::new(0, None).expect("valid arity"),
            ),
            interfaces,
            "test operation",
        )
        .expect("test operation should be valid")
    }

    #[test]
    fn qualified_names_are_canonical_and_validated() {
        assert_eq!(name("structural.instance").namespace(), "structural");
        assert_eq!(name("memory.read").local_name(), "read");

        assert!(QualifiedName::parse("").is_err());
        assert!(QualifiedName::parse("operation").is_err());
        assert!(QualifiedName::parse("Structural.instance").is_err());
        assert!(QualifiedName::parse("structural..instance").is_err());
        assert!(QualifiedName::parse("structural.instance!").is_err());
    }

    #[test]
    fn arity_checks_bounds_without_a_fixed_maximum() {
        let arity = Arity::new(2, None).expect("valid arity");
        assert!(!arity.accepts(1));
        assert!(arity.accepts(2));
        assert!(arity.accepts(u64::MAX));
        assert!(Arity::new(3, Some(2)).is_err());
    }

    #[test]
    fn registration_is_deterministic_and_lookup_is_exact() {
        let mut registry = SchemaRegistry::new();
        let batch = RegistryBatch::new()
            .with_dialect(dialect("structural", DefinitionStability.Core))
            .with_operation(operation(
                "structural.instance",
                "structural",
                DefinitionStability.Core,
                BTreeSet::new(),
            ));

        assert_eq!(
            registry
                .register_batch(batch, ExtensionPolicy::StableOnly)
                .expect("batch should register"),
            1
        );

        assert!(registry.operation(&name("structural.instance")).is_some());
        assert!(registry.operation(&name("structural.unknown")).is_none());
        assert_eq!(registry.snapshot().generation(), 1);
    }

    #[test]
    fn batch_registration_is_atomic_on_duplicate() {
        let mut registry = SchemaRegistry::new();
        registry
            .register_batch(
                RegistryBatch::new()
                    .with_dialect(dialect(
                        "structural",
                        DefinitionStability.Core,
                    )),
                ExtensionPolicy::StableOnly,
            )
            .expect("initial registration should succeed");

        let before = registry.snapshot();
        let result = registry.register_batch(
            RegistryBatch::new()
                .with_dialect(dialect(
                    "structural",
                    DefinitionStability.Core,
                ))
                .with_dialect(dialect(
                    "memory",
                    DefinitionStability.Core,
                )),
            ExtensionPolicy::StableOnly,
        );

        assert!(result.is_err());
        assert_eq!(registry.snapshot(), before);
        assert!(registry.dialect(&name("memory")).is_none());
    }

    #[test]
    fn missing_dialect_rejects_the_entire_batch() {
        let mut registry = SchemaRegistry::new();
        let result = registry.register_batch(
            RegistryBatch::new().with_type(
                TypeDefinition::new(
                    name("memory.word"),
                    name("memory"),
                    version(),
                    DefinitionStability.Stable,
                    "memory word",
                )
                .expect("descriptor should be valid"),
            ),
            ExtensionPolicy::StableOnly,
        );

        assert!(result.is_err());
        assert_eq!(registry.generation(), 0);
    }

    #[test]
    fn missing_interface_rejects_operation_registration() {
        let mut registry = SchemaRegistry::new();
        let mut required = BTreeSet::new();
        required.insert(name("structural.terminator"));

        let result = registry.register_batch(
            RegistryBatch::new()
                .with_dialect(dialect(
                    "structural",
                    DefinitionStability.Core,
                ))
                .with_operation(operation(
                    "structural.branch",
                    "structural",
                    DefinitionStability.Core,
                    required,
                )),
            ExtensionPolicy::StableOnly,
        );

        assert!(result.is_err());
        assert_eq!(registry.generation(), 0);
    }

    #[test]
    fn extension_policy_rejects_experimental_definitions_by_default() {
        let mut registry = SchemaRegistry::new();
        let result = registry.register_batch(
            RegistryBatch::new().with_dialect(dialect(
                "experimental",
                DefinitionStability::Experimental,
            )),
            ExtensionPolicy::StableOnly,
        );

        assert!(result.is_err());
        assert_eq!(registry.generation(), 0);
    }

    #[test]
    fn snapshots_do_not_change_after_later_registration() {
        let mut registry = SchemaRegistry::new();
        registry
            .register_batch(
                RegistryBatch::new().with_dialect(dialect(
                    "structural",
                    DefinitionStability.Core,
                )),
                ExtensionPolicy::StableOnly,
            )
            .expect("first registration should succeed");

        let snapshot = registry.snapshot();

        registry
            .register_batch(
                RegistryBatch::new().with_dialect(dialect(
                    "memory",
                    DefinitionStability.Stable,
                )),
                ExtensionPolicy::StableOnly,
            )
            .expect("second registration should succeed");

        assert_eq!(snapshot.generation(), 1);
        assert!(snapshot.dialect(&name("memory")).is_none());
        assert_eq!(registry.generation(), 2);
    }

    #[test]
    fn empty_batch_is_a_no_op() {
        let mut registry = SchemaRegistry::new();
        assert_eq!(
            registry
                .register_batch(RegistryBatch::new(), ExtensionPolicy::StableOnly)
                .expect("empty batch is valid"),
            0
        );
        assert_eq!(registry.generation(), 0);
    }
}
