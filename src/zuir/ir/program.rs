
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Program
//!
//! Canonical path: `src/zuir/ir/program.rs`
//! Rust edition: 2021
//! Minimum Rust baseline: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the top-level ZUIR program container independently of the concrete
//! module representation. The container owns program identity, representation
//! versions, module membership, required dialect versions, and descriptive
//! program metadata.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! ## OWNS
//!
//! - Program-level identity and representation versions.
//! - Unique module membership within this program.
//! - Required dialect declarations and their versions.
//! - Descriptive metadata that does not change program semantics.
//! - Deterministic iteration over program-owned modules.
//! - Program-container invariants and local validation.
//!
//! ## DOES NOT OWN
//!
//! - AST construction, source parsing, or name resolution.
//! - Semantic analysis or source-language type inference.
//! - Concrete module, function, block, operation, or value semantics.
//! - Dialect registration or executable extension behavior.
//! - Full structural or semantic verification.
//! - Serialization compatibility decisions or migration algorithms.
//! - Target selection, physical allocation, scheduling, or execution.
//! - Quantum qubit identities or a competing quantum IR.
//!
//! ## INTEGRATION CONTRACT
//!
//! This module depends only on foundational ZUIR IDs and version contracts.
//! `M` is the concrete module payload, supplied by the canonical module
//! implementation in `ir/module.rs` when that module is integrated.
//!
//! This generic boundary is deliberate: it permits the program container
//! to be implemented and tested before the module representation is frozen.
//! It does not define a second module schema.
//!
//! After `ir/module.rs` is frozen, the public API may expose a concrete alias
//! or wrapper for the repository's canonical module type. That integration
//! must happen in the IR facade, not by duplicating module semantics here.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! No universal hardware or module-count ceiling is imposed by this type.
//! The collection grows according to available memory and allocator behavior.
//! `usize` and the allocator's address space remain finite implementation
//! constraints; this type does not promise literal infinity.
//!
//! Resource budgets belong to the configuration and processing layers.
//! Callers must enforce their configured budgets when building, verifying,
//! importing, or serializing a program.
//!
//! ## DETERMINISM
//!
//! `BTreeMap` gives deterministic key ordering for modules, dialect
//! requirements, and descriptive metadata. This is deterministic traversal,
//! not a guarantee that IDs or program content are stable across builds.
//!
//! ## FREEZE CRITERIA
//!
//! - Program and module IDs are distinct typed identities.
//! - Duplicate module IDs are rejected.
//! - Required dialect names are validated and versioned.
//! - Descriptive metadata cannot masquerade as semantic attributes.
//! - Version identities are explicit.
//! - No unsafe code or fixed hardware assumptions are introduced.
//! - Invalid input and duplicate membership have deterministic errors.
//! - Tests cover empty programs, insertion, replacement rejection,
//!   removal, deterministic iteration, and dialect validation.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::zuir::ids::{ModuleId, ProgramId};
use crate::zuir::version::{
    DialectVersion, ZuirFormatVersion, ZuirSchemaVersion,
};

/// A qualified name for a required dialect.
///
/// The value is a name, not proof that the dialect is registered, trusted,
/// installed, or executable. The dialect registry owns those decisions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DialectName(String);

impl DialectName {
    /// Construct a dialect name using a conservative portable grammar.
    ///
    /// Names are lowercase ASCII namespace segments separated by dots.
    /// Each segment must begin with a lowercase ASCII letter or digit and
    /// may subsequently contain lowercase letters, digits, `_`, or `-`.
    ///
    /// Examples:
    /// - `core`
    /// - `classical`
    /// - `quantum`
    /// - `vendor.tensor`
    ///
    /// This validates the spelling only. Registry ownership and namespace
    /// authorization are checked by the dialect registry.
    pub fn new(value: impl Into<String>) -> Result<Self, ProgramError> {
        let value = value.into();

        if !is_valid_dialect_name(&value) {
            return Err(ProgramError::InvalidDialectName(value));
        }

        Ok(Self(value))
    }

    /// Return the canonical dialect name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this wrapper and return its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
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

/// A dialect version required to interpret a program correctly.
///
/// A declaration is a compatibility requirement, not a registration or
/// capability grant. The dialect registry must resolve the name and prove
/// that the installed definition is compatible before interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectRequirement {
    name: DialectName,
    version: DialectVersion,
}

impl DialectRequirement {
    /// Construct a required dialect declaration.
    #[must_use]
    pub fn new(name: DialectName, version: DialectVersion) -> Self {
        Self { name, version }
    }

    /// Return the required dialect name.
    #[must_use]
    pub fn name(&self) -> &DialectName {
        &self.name
    }

    /// Return the required dialect version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.version
    }
}

/// Errors produced by local program-container operations.
///
/// These errors describe container invariants only. They do not replace
/// the canonical `ZuirError` used at public compiler/API boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    /// A dialect name did not satisfy the portable naming contract.
    InvalidDialectName(String),

    /// A module with this ID already belongs to the program.
    DuplicateModule(ModuleId),

    /// A module ID is not present in the program.
    UnknownModule(ModuleId),

    /// A dialect requirement was already declared under the same name.
    DuplicateDialectRequirement(DialectName),

    /// A required dialect name was not declared.
    UnknownDialectRequirement(DialectName),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDialectName(name) => {
                write!(f, "invalid qualified dialect name: {name:?}")
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
            Self::UnknownDialectRequirement(name) => {
                write!(f, "dialect requirement {name} is not declared")
            }
        }
    }
}

impl std::error::Error for ProgramError {}

/// The canonical top-level ZUIR program container.
///
/// `M` is the canonical module representation owned by `ir/module.rs`.
/// This container deliberately does not assume the module's internal shape.
///
/// Modules are keyed by their typed IDs. Insertion never silently replaces an
/// existing module. Ordered maps provide deterministic traversal.
///
/// # Identity
///
/// `ProgramId` identifies this program in the identity scope established by
/// the enclosing compilation or artifact owner. This type does not claim
/// that the numeric ID is globally unique or stable across compilations.
///
/// # Versioning
///
/// The schema version describes the logical representation contract.
/// The format version describes the serialized encoding contract. They are
/// intentionally independent.
///
/// # Metadata
///
/// `metadata` is descriptive metadata only. Semantic operation attributes,
/// effects, resource requirements, and contracts belong to their respective
/// canonical IR entities and must not be stored here as untyped strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "M: Serialize",
    deserialize = "M: Deserialize<'de>"
))]
pub struct ZuirProgram<M> {
    id: ProgramId,
    schema_version: ZuirSchemaVersion,
    format_version: ZuirFormatVersion,

    /// Modules keyed by typed identity for deterministic traversal.
    modules: BTreeMap<ModuleId, M>,

    /// Dialects that must be available to interpret this program.
    dialects: BTreeMap<DialectName, DialectRequirement>,

    /// Descriptive, non-semantic program metadata.
    ///
    /// This map is not a substitute for typed semantic attributes.
    metadata: BTreeMap<String, String>,
}

impl<M> ZuirProgram<M> {
    /// Create an empty program with explicit identity and representation
    /// versions.
    ///
    /// The caller owns ID allocation and must ensure that `id` is unique in
    /// the required scope. The program does not silently allocate IDs from
    /// a separate allocator because independent allocators can collide.
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

    /// Return the program identity.
    #[must_use]
    pub const fn id(&self) -> ProgramId {
        self.id
    }

    /// Return the logical schema version.
    #[must_use]
    pub const fn schema_version(&self) -> ZuirSchemaVersion {
        self.schema_version
    }

    /// Return the serialized-format version.
    #[must_use]
    pub const fn format_version(&self) -> ZuirFormatVersion {
        self.format_version
    }

    /// Return the number of modules currently owned by the program.
    ///
    /// This is an observation, not a language-wide or hardware capacity.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Return whether the program currently contains no modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Return whether the program contains a module with this ID.
    #[must_use]
    pub fn contains_module(&self, id: ModuleId) -> bool {
        self.modules.contains_key(&id)
    }

    /// Insert a module without replacing an existing module.
    ///
    /// If the ID already exists, the new module is returned in the error-free
    /// result so the caller can recover it without losing ownership.
    ///
    /// `Err((ProgramError, M))` avoids discarding a potentially large module
    /// or requiring a clone merely to report a duplicate identity.
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

    /// Borrow a module by identity.
    #[must_use]
    pub fn module(&self, id: ModuleId) -> Option<&M> {
        self.modules.get(&id)
    }

    /// Mutably borrow a module by identity.
    ///
    /// Mutation of a module may invalidate previously established verification
    /// results. The caller must re-run the appropriate verifier before the
    /// program is accepted by a downstream compiler stage.
    pub fn module_mut(&mut self, id: ModuleId) -> Option<&mut M> {
        self.modules.get_mut(&id)
    }

    /// Remove and return a module by identity.
    pub fn remove_module(&mut self, id: ModuleId) -> Option<M> {
        self.modules.remove(&id)
    }

    /// Iterate over modules in deterministic `ModuleId` order.
    pub fn modules(&self) -> impl ExactSizeIterator<Item = (ModuleId, &M)> + '_ {
        self.modules.iter().map(|(id, module)| (*id, module))
    }

    /// Iterate over module IDs in deterministic order.
    pub fn module_ids(&self) -> impl ExactSizeIterator<Item = ModuleId> + '_ {
        self.modules.keys().copied()
    }

    /// Declare a required dialect.
    ///
    /// A requirement with the same name cannot be silently replaced by a
    /// different version. Use `replace_dialect_requirement` only when an
    /// explicit migration or construction step intends that change.
    pub fn require_dialect(
        &mut self,
        requirement: DialectRequirement,
    ) -> Result<(), ProgramError> {
        let name = requirement.name().clone();

        if self.dialects.contains_key(&name) {
            return Err(ProgramError::DuplicateDialectRequirement(name));
        }

        self.dialects.insert(name, requirement);
        Ok(())
    }

    /// Explicitly replace a dialect requirement.
    ///
    /// This method changes the program's interpretation requirements. It
    /// must be used only by a controlled construction, migration, or
    /// transformation step that has established compatibility.
    pub fn replace_dialect_requirement(
        &mut self,
        requirement: DialectRequirement,
    ) -> Option<DialectRequirement> {
        self.dialects
            .insert(requirement.name().clone(), requirement)
    }

    /// Return a required dialect by qualified name.
    #[must_use]
    pub fn dialect_requirement(
        &self,
        name: &DialectName,
    ) -> Option<&DialectRequirement> {
        self.dialects.get(name)
    }

    /// Iterate over required dialects in deterministic name order.
    pub fn dialect_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = (&DialectName, &DialectRequirement)> + '_ {
        self.dialects.iter()
    }

    /// Return whether the program declares any required dialects.
    #[must_use]
    pub fn has_dialect_requirements(&self) -> bool {
        !self.dialects.is_empty()
    }

    /// Add descriptive metadata.
    ///
    /// Empty keys are rejected. Metadata is not permitted to override
    /// semantic attributes, effects, dialect definitions, or versions.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ProgramError> {
        let key = key.into();

        if key.trim().is_empty() || key.trim() != key {
            return Err(ProgramError::InvalidDialectName(format!(
                "invalid metadata key {key:?}"
            )));
        }

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Read descriptive metadata.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Iterate over descriptive metadata in deterministic key order.
    pub fn metadata_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &str)> + '_ {
        self.metadata
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Return a read-only view of the module collection.
    ///
    /// This is useful for verification and serialization adapters that need
    /// to inspect membership without mutating the program.
    #[must_use]
    pub fn modules_by_id(&self) -> &BTreeMap<ModuleId, M> {
        &self.modules
    }

    /// Return a read-only view of the required dialect declarations.
    #[must_use]
    pub fn required_dialects(
        &self,
    ) -> &BTreeMap<DialectName, DialectRequirement> {
        &self.dialects
    }

    /// Return a read-only view of descriptive metadata.
    #[must_use]
    pub fn descriptive_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{IdAllocator, ModuleId, ProgramId};
    use crate::zuir::version::{
        CURRENT_ZUIR_FORMAT_VERSION, CURRENT_ZUIR_SCHEMA_VERSION,
        DialectVersion,
    };

    fn program() -> ZuirProgram<String> {
        let ids = IdAllocator::<ProgramId>::new();
        ZuirProgram::new(
            ids.try_next().expect("program ID"),
            CURRENT_ZUIR_SCHEMA_VERSION,
            CURRENT_ZUIR_FORMAT_VERSION,
        )
    }

    fn module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("module ID must be nonzero")
    }

    fn dialect(name: &str) -> DialectRequirement {
        DialectRequirement::new(
            DialectName::new(name).expect("valid dialect name"),
            DialectVersion::new(1, 0, 0),
        )
    }

    #[test]
    fn new_program_has_explicit_versions_and_no_modules() {
        let program = program();

        assert_eq!(program.schema_version(), CURRENT_ZUIR_SCHEMA_VERSION);
        assert_eq!(program.format_version(), CURRENT_ZUIR_FORMAT_VERSION);
        assert!(program.is_empty());
        assert_eq!(program.module_count(), 0);
    }

    #[test]
    fn module_insert_lookup_and_remove_are_consistent() {
        let mut program = program();

        program
            .insert_module(module_id(1), "module body".to_owned())
            .expect("first insertion succeeds");

        assert!(program.contains_module(module_id(1)));
        assert_eq!(program.module(module_id(1)).map(String::as_str), Some("module body"));
        assert_eq!(program.module_count(), 1);

        assert_eq!(
            program.remove_module(module_id(1)),
            Some("module body".to_owned())
        );
        assert!(program.is_empty());
    }

    #[test]
    fn duplicate_module_ids_do_not_replace_existing_modules() {
        let mut program = program();

        program
            .insert_module(module_id(7), "original".to_owned())
            .expect("first insertion succeeds");

        let error = program
            .insert_module(module_id(7), "replacement".to_owned())
            .expect_err("duplicate ID must be rejected");

        assert_eq!(error.0, ProgramError::DuplicateModule(module_id(7)));
        assert_eq!(error.1, "replacement");
        assert_eq!(program.module(module_id(7)).map(String::as_str), Some("original"));
    }

    #[test]
    fn module_iteration_is_deterministically_ordered_by_id() {
        let mut program = program();

        program.insert_module(module_id(30), "c".into()).expect("insert");
        program.insert_module(module_id(10), "a".into()).expect("insert");
        program.insert_module(module_id(20), "b".into()).expect("insert");

        let ids: Vec<_> = program.module_ids().collect();

        assert_eq!(ids, vec![module_id(10), module_id(20), module_id(30)]);
    }

    #[test]
    fn dialect_requirements_are_versioned_and_unique() {
        let mut program = program();

        program
            .require_dialect(dialect("quantum"))
            .expect("first requirement succeeds");

        assert_eq!(
            program
                .dialect_requirement(&DialectName::new("quantum").expect("name"))
                .expect("requirement exists")
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
    fn dialect_names_reject_empty_segments_and_invalid_characters() {
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
                "expected {invalid:?} to be rejected"
            );
        }

        for valid in ["core", "quantum", "vendor.tensor", "x2.custom_ops"] {
            assert!(
                DialectName::new(valid).is_ok(),
                "expected {valid:?} to be accepted"
            );
        }
    }

    #[test]
    fn metadata_is_descriptive_and_deterministically_ordered() {
        let mut program = program();

        program.set_metadata("z.note", "last").expect("metadata");
        program.set_metadata("a.note", "first").expect("metadata");

        assert_eq!(program.metadata("a.note"), Some("first"));

        let keys: Vec<_> = program
            .metadata_entries()
            .map(|(key, _)| key)
            .collect();

        assert_eq!(keys, vec!["a.note", "z.note"]);
    }

    #[test]
    fn serde_round_trip_preserves_program_container() {
        let mut original = program();

        original
            .insert_module(module_id(11), "module".to_owned())
            .expect("insert");
        original
            .require_dialect(dialect("classical"))
            .expect("dialect");
        original
            .set_metadata("build.origin", "test")
            .expect("metadata");

        let encoded =
            serde_json::to_string(&original).expect("serialization succeeds");
        let decoded: ZuirProgram<String> =
            serde_json::from_str(&encoded).expect("deserialization succeeds");

        assert_eq!(decoded, original);
    }
}
