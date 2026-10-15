
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Program Container
//!
//! Path: src/zuir/ir/program.rs
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//!
//! PURPOSE
//!
//! Own the canonical top-level ZUIR program container: program identity,
//! schema and format versions, module membership, dialect requirements,
//! and descriptive metadata.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/version.rs
//! - src/zuir/ir/module.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//!
//! OWNS
//!
//! - Program identity and representation-version fields.
//! - Unique module membership keyed by ModuleId.
//! - Validated dialect names and unique dialect requirements.
//! - Descriptive metadata and its validation.
//! - Deterministic iteration and local container validation.
//!
//! DOES NOT OWN
//!
//! - AST construction, parsing, or name resolution.
//! - Function, block, operation, type, or SSA semantics.
//! - Dialect registration or executable extension behavior.
//! - Whole-program semantic verification.
//! - Resource negotiation, physical allocation, scheduling, or execution.
//! - Quantum-specific IR or physical qubit identities.
//!
//! DEPENDENCIES
//!
//! Uses the canonical ProgramId, ModuleId, and version types.
//! M remains generic so this file does not duplicate module semantics.
//!
//! INTEGRATION
//!
//! The canonical module owner is ir/module.rs. Insert modules using their
//! canonical ModuleId. The full verifier checks relationships between
//! program, module, symbol, type, and operation identities.
//!
//! The serializer must reject malformed container data and run validate()
//! before exposing decoded data to compilation or execution stages.
//!
//! SCALABILITY
//!
//! Collections are dynamically sized. This file imposes no universal
//! module-count, hardware-count, qubit-count, memory-size, or node-count
//! ceiling. Rust address space, allocator capacity, and available memory
//! remain finite; allocation failure is not a valid-program semantic change.
//!
//! SAFETY
//!
//! No unsafe code.
//!
//! FREEZE CRITERIA
//!
//! - Duplicate module IDs never overwrite existing modules.
//! - Dialect names and metadata keys are validated at construction and decode.
//! - Dialect map keys agree with their stored requirements.
//! - Invalid state can be detected by validate().
//! - Iteration order is deterministic.
//! - Unit, serialization, and integration tests pass.

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::zuir::ids::{ModuleId, ProgramId};
use crate::zuir::version::{
    DialectVersion, ZuirFormatVersion, ZuirSchemaVersion,
};

// =============================================================================
// Qualified dialect names
// =============================================================================

/// A validated, portable, qualified dialect name.
///
/// This validates spelling only. It does not prove that a dialect is
/// installed, trusted, registered, authorized, or executable.
///
/// Grammar:
///
///     name    := segment ("." segment)*
///     segment := [a-z0-9] [a-z0-9_-]*
///
/// Examples: core, classical, quantum, vendor.tensor.
///
/// Names are not normalized. Uppercase characters, whitespace, empty
/// segments, and path separators are rejected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct DialectName(String);

impl DialectName {
    /// Construct a dialect name after validating its spelling.
    pub fn new(value: impl Into<String>) -> Result<Self, ProgramError> {
        let value = value.into();

        if !is_valid_dialect_name(&value) {
            return Err(ProgramError::InvalidDialectName(value));
        }

        Ok(Self(value))
    }

    /// Return the validated qualified name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this wrapper and return its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Recheck the invariant at a trust boundary.
    pub fn validate(&self) -> Result<(), ProgramError> {
        if is_valid_dialect_name(&self.0) {
            Ok(())
        } else {
            Err(ProgramError::InvalidDialectName(self.0.clone()))
        }
    }
}

impl<'de> Deserialize<'de> for DialectName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for DialectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn is_valid_dialect_name(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    value.split('.').all(|segment| {
        let mut chars = segment.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
            return false;
        }

        chars.all(|ch| {
            ch.is_ascii_lowercase()
                || ch.is_ascii_digit()
                || ch == '_'
                || ch == '-'
        })
    })
}

// =============================================================================
// Dialect requirements
// =============================================================================

/// A versioned requirement to interpret a dialect.
///
/// A requirement is not a registry entry or authorization grant.
/// Compatibility must be established by the dialect registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectRequirement {
    name: DialectName,
    version: DialectVersion,
}

impl DialectRequirement {
    /// Construct a dialect requirement.
    #[must_use]
    pub fn new(name: DialectName, version: DialectVersion) -> Self {
        Self { name, version }
    }

    /// Return the dialect name.
    #[must_use]
    pub fn name(&self) -> &DialectName {
        &self.name
    }

    /// Return the required dialect version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.version
    }

    /// Validate this requirement's local invariants.
    pub fn validate(&self) -> Result<(), ProgramError> {
        self.name.validate()
    }
}

// =============================================================================
// Descriptive metadata
// =============================================================================

/// Validate a metadata key.
///
/// Metadata keys are names, not paths or semantic attribute encodings.
/// Their grammar is deliberately open-ended but rejects empty values,
/// leading/trailing whitespace, control characters, and NUL.
fn validate_metadata_key(key: &str) -> Result<(), ProgramError> {
    if key.is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
    {
        return Err(ProgramError::InvalidMetadataKey(key.to_owned()));
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors from local program-container operations.
///
/// These errors describe container integrity only. They do not replace
/// the repository's canonical ZuirError at public compiler boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    /// A dialect name has invalid syntax.
    InvalidDialectName(String),

    /// A metadata key is empty or invalid.
    InvalidMetadataKey(String),

    /// A module ID already belongs to this program.
    DuplicateModule(ModuleId),

    /// A module ID is not present in this program.
    UnknownModule(ModuleId),

    /// A dialect name already has a requirement.
    DuplicateDialectRequirement(DialectName),

    /// A dialect map key differs from the stored requirement's name.
    DialectKeyMismatch {
        key: DialectName,
        requirement_name: DialectName,
    },

    /// A dialect requirement is malformed.
    InvalidDialectRequirement(DialectName),

    /// A stored metadata key is invalid.
    InvalidStoredMetadataKey(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDialectName(name) => {
                write!(f, "invalid qualified dialect name: {name:?}")
            }
            Self::InvalidMetadataKey(key) => {
                write!(f, "invalid descriptive metadata key: {key:?}")
            }
            Self::DuplicateModule(id) => {
                write!(f, "module {id} already belongs to this program")
            }
            Self::UnknownModule(id) => {
                write!(f, "module {id} does not belong to this program")
            }
            Self::DuplicateDialectRequirement(name) => {
                write!(f, "dialect requirement {name} is already declared")
            }
            Self::DialectKeyMismatch {
                key,
                requirement_name,
            } => write!(
                f,
                "dialect map key {key} does not match requirement name \
                 {requirement_name}"
            ),
            Self::InvalidDialectRequirement(name) => {
                write!(f, "invalid requirement for dialect {name}")
            }
            Self::InvalidStoredMetadataKey(key) => {
                write!(f, "invalid stored metadata key: {key:?}")
            }
        }
    }
}

impl std::error::Error for ProgramError {}

// =============================================================================
// Program container
// =============================================================================

/// Canonical top-level ZUIR program container.
///
/// `M` is the module payload owned by `ir/module.rs`. The generic parameter
/// avoids duplicating module structure and permits independent testing.
///
/// Modules are stored by typed identity. The container does not claim that
/// an ID is globally unique or stable across independent compilations.
///
/// The schema version describes logical IR semantics. The format version
/// describes the persistence encoding. They are independent contracts.
///
/// Program metadata is descriptive only; operation attributes, resource
/// requirements, effects, contracts, and provenance belong to their
/// respective semantic owners.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ZuirProgram<M> {
    id: ProgramId,
    schema_version: ZuirSchemaVersion,
    format_version: ZuirFormatVersion,
    modules: BTreeMap<ModuleId, M>,
    dialects: BTreeMap<DialectName, DialectRequirement>,
    metadata: BTreeMap<String, String>,
}

impl<'de, M> Deserialize<'de> for ZuirProgram<M>
where
    M: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireProgram<M> {
            id: ProgramId,
            schema_version: ZuirSchemaVersion,
            format_version: ZuirFormatVersion,
            modules: BTreeMap<ModuleId, M>,
            dialects: BTreeMap<DialectName, DialectRequirement>,
            metadata: BTreeMap<String, String>,
        }

        let wire = WireProgram::<M>::deserialize(deserializer)?;

        let program = Self {
            id: wire.id,
            schema_version: wire.schema_version,
            format_version: wire.format_version,
            modules: wire.modules,
            dialects: wire.dialects,
            metadata: wire.metadata,
        };

        program.validate().map_err(serde::de::Error::custom)?;
        Ok(program)
    }
}

impl<M> ZuirProgram<M> {
    /// Create an empty program with explicit identity and versions.
    ///
    /// ID allocation is owned by the caller so that independent allocators
    /// cannot silently establish conflicting identity scopes.
    #[must_use]
    pub fn new(
        id: ProgramId,
        schema_version: ZuirSchemaVersion,
        format_version: ZuirFormatVersion,
    ) -> Self {
        Self {
            id,
            schema_version,
            format_version,
            modules: BTreeMap::new(),
            dialects: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Return the program ID.
    #[must_use]
    pub const fn id(&self) -> ProgramId {
        self.id
    }

    /// Return the logical schema version.
    #[must_use]
    pub const fn schema_version(&self) -> ZuirSchemaVersion {
        self.schema_version
    }

    /// Return the serialization format version.
    #[must_use]
    pub const fn format_version(&self) -> ZuirFormatVersion {
        self.format_version
    }

    /// Return the number of modules in this program.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Return whether this program contains no modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Check whether a module ID is present.
    #[must_use]
    pub fn contains_module(&self, id: ModuleId) -> bool {
        self.modules.contains_key(&id)
    }

    /// Insert a module without replacing an existing module.
    ///
    /// On failure, the caller retains ownership of the supplied module.
    pub fn insert_module(
        &mut self,
        id: ModuleId,
        module: M,
    ) -> Result<(), (ProgramError, M)> {
        if self.modules.contains_key(&id) {
            return Err((ProgramError::DuplicateModule(id), module));
        }

        self.modules.insert(id, module);
        Ok(())
    }

    /// Insert a module using its canonical ID.
    ///
    /// The module type must expose `id()` in its own API. This generic
    /// container deliberately does not require that trait because module.rs
    /// may be independently evolved or wrapped by repository adapters.
    ///
    /// Call `insert_module(module.id(), module)` at the integration boundary.
    ///
    /// This method is intentionally not implemented generically.
    ///
    /// The explicit-ID form prevents this container from inventing module
    /// identity semantics for arbitrary payload types.
    #[must_use]
    pub fn module(&self, id: ModuleId) -> Option<&M> {
        self.modules.get(&id)
    }

    /// Mutably borrow a module by ID.
    ///
    /// Any mutation may invalidate earlier verification results. The caller
    /// must reverify before handing the program to a trusted downstream stage.
    pub fn module_mut(&mut self, id: ModuleId) -> Option<&mut M> {
        self.modules.get_mut(&id)
    }

    /// Remove and return a module.
    pub fn remove_module(&mut self, id: ModuleId) -> Option<M> {
        self.modules.remove(&id)
    }

    /// Iterate through modules in deterministic ModuleId order.
    pub fn modules(
        &self,
    ) -> impl ExactSizeIterator<Item = (ModuleId, &M)> + '_ {
        self.modules.iter().map(|(id, module)| (*id, module))
    }

    /// Iterate through module IDs in deterministic order.
    pub fn module_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = ModuleId> + '_ {
        self.modules.keys().copied()
    }

    /// Return a read-only view of the module map.
    #[must_use]
    pub fn modules_by_id(&self) -> &BTreeMap<ModuleId, M> {
        &self.modules
    }

    /// Declare a required dialect without silently replacing a requirement.
    pub fn require_dialect(
        &mut self,
        requirement: DialectRequirement,
    ) -> Result<(), ProgramError> {
        requirement.validate()?;

        let name = requirement.name().clone();

        if self.dialects.contains_key(&name) {
            return Err(ProgramError::DuplicateDialectRequirement(name));
        }

        self.dialects.insert(name, requirement);
        Ok(())
    }

    /// Replace a dialect requirement explicitly.
    ///
    /// Use only in a controlled migration or transformation that has already
    /// established compatibility. Invalid requirements are rejected before
    /// the existing requirement is changed.
    pub fn replace_dialect_requirement(
        &mut self,
        requirement: DialectRequirement,
    ) -> Result<Option<DialectRequirement>, ProgramError> {
        requirement.validate()?;

        Ok(self
            .dialects
            .insert(requirement.name().clone(), requirement))
    }

    /// Remove a dialect requirement by name.
    pub fn remove_dialect_requirement(
        &mut self,
        name: &DialectName,
    ) -> Option<DialectRequirement> {
        self.dialects.remove(name)
    }

    /// Look up a dialect requirement.
    #[must_use]
    pub fn dialect_requirement(
        &self,
        name: &DialectName,
    ) -> Option<&DialectRequirement> {
        self.dialects.get(name)
    }

    /// Iterate through dialect requirements in deterministic name order.
    pub fn dialect_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = (&DialectName, &DialectRequirement)> + '_
    {
        self.dialects.iter()
    }

    /// Return whether dialect requirements are present.
    #[must_use]
    pub fn has_dialect_requirements(&self) -> bool {
        !self.dialects.is_empty()
    }

    /// Return a read-only view of the dialect map.
    #[must_use]
    pub fn required_dialects(
        &self,
    ) -> &BTreeMap<DialectName, DialectRequirement> {
        &self.dialects
    }

    /// Set descriptive metadata.
    ///
    /// The returned previous value, if any, allows the caller to observe
    /// replacement. This is descriptive metadata, not a semantic attribute.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ProgramError> {
        let key = key.into();
        validate_metadata_key(&key)?;

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Remove descriptive metadata.
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.metadata.remove(key)
    }

    /// Read descriptive metadata.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Iterate through descriptive metadata in deterministic key order.
    pub fn metadata_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &str)> + '_ {
        self.metadata
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Return a read-only view of descriptive metadata.
    #[must_use]
    pub fn descriptive_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate invariants owned by the program container.
    ///
    /// This does not verify module bodies, symbols, types, operation legality,
    /// dialect registration, or schema compatibility. Those are owned by the
    /// full verifier, dialect registry, and compatibility subsystem.
    pub fn validate(&self) -> Result<(), ProgramError> {
        for (key, requirement) in &self.dialects {
            requirement
                .validate()
                .map_err(|_| {
                    ProgramError::InvalidDialectRequirement(key.clone())
                })?;

            if key != requirement.name() {
                return Err(ProgramError::DialectKeyMismatch {
                    key: key.clone(),
                    requirement_name: requirement.name().clone(),
                });
            }
        }

        for key in self.metadata.keys() {
            validate_metadata_key(key)
                .map_err(|_| ProgramError::InvalidStoredMetadataKey(key.clone()))?;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{IdAllocator, ModuleId, ProgramId};
    use crate::zuir::version::{
        CURRENT_ZUIR_FORMAT_VERSION,
        CURRENT_ZUIR_SCHEMA_VERSION,
        DialectVersion,
    };

    fn program() -> ZuirProgram<String> {
        let allocator = IdAllocator::<ProgramId>::new();

        ZuirProgram::new(
            allocator.try_next().expect("program ID"),
            CURRENT_ZUIR_SCHEMA_VERSION,
            CURRENT_ZUIR_FORMAT_VERSION,
        )
    }

    fn module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("nonzero module ID")
    }

    fn dialect(name: &str) -> DialectRequirement {
        DialectRequirement::new(
            DialectName::new(name).expect("valid dialect name"),
            DialectVersion::new(1, 0, 0),
        )
    }

    #[test]
    fn new_program_has_explicit_versions_and_is_empty() {
        let program = program();

        assert_eq!(program.schema_version(), CURRENT_ZUIR_SCHEMA_VERSION);
        assert_eq!(program.format_version(), CURRENT_ZUIR_FORMAT_VERSION);
        assert!(program.is_empty());
        assert_eq!(program.module_count(), 0);
        assert!(program.validate().is_ok());
    }

    #[test]
    fn module_insert_lookup_remove_and_count_are_consistent() {
        let mut program = program();

        program
            .insert_module(module_id(1), "body".to_owned())
            .expect("insert");

        assert!(program.contains_module(module_id(1)));
        assert_eq!(program.module(module_id(1)).map(String::as_str), Some("body"));
        assert_eq!(program.module_count(), 1);

        assert_eq!(program.remove_module(module_id(1)).as_deref(), Some("body"));
        assert!(program.is_empty());
    }

    #[test]
    fn duplicate_module_does_not_replace_existing_payload() {
        let mut program = program();

        program
            .insert_module(module_id(7), "original".to_owned())
            .expect("first insert");

        let (error, returned) = program
            .insert_module(module_id(7), "replacement".to_owned())
            .expect_err("duplicate must fail");

        assert_eq!(error, ProgramError::DuplicateModule(module_id(7)));
        assert_eq!(returned, "replacement");
        assert_eq!(program.module(module_id(7)).map(String::as_str), Some("original"));
    }

    #[test]
    fn module_iteration_is_deterministic() {
        let mut program = program();

        program.insert_module(module_id(30), "c".into()).expect("insert");
        program.insert_module(module_id(10), "a".into()).expect("insert");
        program.insert_module(module_id(20), "b".into()).expect("insert");

        assert_eq!(
            program.module_ids().collect::<Vec<_>>(),
            vec![module_id(10), module_id(20), module_id(30)]
        );
    }

    #[test]
    fn dialect_requirements_are_unique_and_versioned() {
        let mut program = program();

        program.require_dialect(dialect("quantum")).expect("require");

        assert_eq!(
            program
                .dialect_requirement(&DialectName::new("quantum").expect("name"))
                .expect("present")
                .version(),
            DialectVersion::new(1, 0, 0)
        );

        assert_eq!(
            program.require_dialect(dialect("quantum")),
            Err(ProgramError::DuplicateDialectRequirement(
                DialectName::new("quantum").expect("name")
            ))
        );
    }

    #[test]
    fn invalid_dialect_names_are_rejected() {
        for invalid in [
            "",
            ".core",
            "core.",
            "core..quantum",
            "Core",
            "core quantum",
            "core/quantum",
            " core",
        ] {
            assert!(
                DialectName::new(invalid).is_err(),
                "accepted invalid dialect name: {invalid:?}"
            );
        }

        for valid in [
            "core",
            "quantum",
            "vendor.tensor",
            "x2.custom_ops",
        ] {
            assert!(
                DialectName::new(valid).is_ok(),
                "rejected valid dialect name: {valid:?}"
            );
        }
    }

    #[test]
    fn metadata_keys_are_validated_and_ordered() {
        let mut program = program();

        program.set_metadata("z.note", "last").expect("metadata");
        program.set_metadata("a.note", "first").expect("metadata");

        assert_eq!(program.metadata("a.note"), Some("first"));
        assert_eq!(
            program
                .metadata_entries()
                .map(|(key, _)| key)
                .collect::<Vec<_>>(),
            vec!["a.note", "z.note"]
        );

        assert!(matches!(
            program.set_metadata(" bad", "value"),
            Err(ProgramError::InvalidMetadataKey(_))
        ));

        assert!(matches!(
            program.set_metadata("bad\nkey", "value"),
            Err(ProgramError::InvalidMetadataKey(_))
        ));
    }

    #[test]
    fn dialect_replacement_is_explicit() {
        let mut program = program();

        program
            .require_dialect(dialect("quantum"))
            .expect("initial requirement");

        let old = program
            .replace_dialect_requirement(DialectRequirement::new(
                DialectName::new("quantum").expect("name"),
                DialectVersion::new(2, 0, 0),
            ))
            .expect("replacement")
            .expect("old requirement");

        assert_eq!(old.version(), DialectVersion::new(1, 0, 0));

        assert_eq!(
            program
                .dialect_requirement(&DialectName::new("quantum").expect("name"))
                .expect("new requirement")
                .version(),
            DialectVersion::new(2, 0, 0)
        );
    }

    #[test]
    fn serde_round_trip_preserves_container() {
        let mut original = program();

        original
            .insert_module(module_id(11), "module".to_owned())
            .expect("module");
        original
            .require_dialect(dialect("classical"))
            .expect("dialect");
        original
            .set_metadata("build.origin", "test")
            .expect("metadata");

        let encoded = serde_json::to_string(&original).expect("encode");
        let decoded: ZuirProgram<String> =
            serde_json::from_str(&encoded).expect("decode");

        assert_eq!(decoded, original);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn empty_program_can_be_validated_without_domain_assumptions() {
        let program = program();
        assert_eq!(program.validate(), Ok(()));
    }
}
