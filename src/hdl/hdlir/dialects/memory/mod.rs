
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Memory Dialect
//!
//! Path: `src/hdl/hdlir/dialects/memory/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Memory-dialect foundation; production release requires the
//! complete HDLIR module tree and repository-wide qualification.
//!
//! ## Purpose
//!
//! Define the memory dialect's registration identity and verification
//! boundary. This module composes the existing logical memory type and
//! declaration APIs without introducing a second memory representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/memories.g4`
//! - `grammar/hdl/arrays.g4`
//! - `grammar/hdl/hdl.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/memories.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/types/memory.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar and frontend own source syntax and source semantics.
//! `types/memory.rs` owns logical memory types and their contracts.
//! `ir/memories.rs` owns memory declarations and logical port bindings.
//! The schema registry owns dialect registration and definition identity.
//! This module owns the memory-dialect verification entry points.
//!
//! ## Owns
//!
//! - The versioned identity of the built-in memory dialect.
//! - Registration of the dialect in the shared HDLIR schema registry.
//! - The policy for requiring declared memory-port binding records.
//! - Composition of local declaration and graph-reference validation.
//!
//! ## Does not own
//!
//! - A second memory type, memory declaration table, or operation graph.
//! - Source parsing, AST construction, name resolution, or source semantics.
//! - The meaning of arbitrary schema-defined operation payloads.
//! - Runtime memory contents or simulation execution.
//! - Physical addresses, memory banks, caches, controllers, DMA, or allocation.
//! - Synthesis, technology mapping, placement, routing, or timing closure.
//! - The universal operation vocabulary for all present and future targets.
//!
//! ## Completeness and verification
//!
//! A memory declaration may be incomplete while a design is being built.
//! This module therefore makes declared-port binding completeness an explicit
//! policy. Structural verification always runs. Complete-design verification
//! may additionally require a binding record for every port declared by the
//! memory type.
//!
//! Reference verification delegates to the canonical `HdlirMemoryTable`
//! APIs. The supplied callbacks must check existence in the correct graph
//! and identity namespace. These checks do not replace type compatibility,
//! signal direction, clock-domain, or operation-specific verification.
//!
//! ## Scalability
//!
//! This module introduces no universal limit on memory count, dimensions,
//! element width, port count, or hardware capacity. Symbolic dimensions
//! remain in `MemoryType` and are not evaluated here. Collection and work
//! limits belong to explicit operational budgets in the stages that need
//! them. Every invocation is finite and subject to available resources.
//!
//! ## Extensibility
//!
//! This module registers the dialect identity, not a closed list of memory
//! operations. New operations and type extensions must use the shared schema
//! registry and their own explicit semantic contracts. Registration alone
//! never claims that an operation is implemented, verified, or realizable.
//!
//! ## Integration
//!
//! - `dialects/mod.rs` must declare `pub mod memory;`.
//! - The HDLIR composition root must register `dialects` and its dependencies.
//! - `ir/mod.rs` must expose the actual `ir::memories` module.
//! - `types/mod.rs` must expose the actual `types::memory` module.
//! - The design verifier must call this dialect's declaration/reference
//!   verification entry points as part of whole-design verification.
//! - Type, connectivity, clock, and resource verification remain separate
//!   mandatory checks where the design requires them.
//! - Semantic lowering must preserve source-defined dimensions, port access,
//!   clocking, initialization, collision behavior, and clause categories.
//!
//! ## Freeze criteria
//!
//! - The complete HDLIR composition root compiles.
//! - The dialect is registered exactly once in the intended registry.
//! - Structural and completeness policies are covered by tests.
//! - Reference callbacks are integrated with the canonical graph.
//! - Type, connectivity, clock and resource verification are integrated.
//! - The full repository's formatting, build, tests and Clippy checks pass.

use crate::hdl::hdlir::config::ExtensionPolicy;
use crate::hdl::hdlir::errors::HdlirResult;
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId, HdlirModuleId, HdlirSignalId, HdlirValueId,
};
use crate::hdl::hdlir::ir::memories::HdlirMemoryTable;
use crate::hdl::hdlir::schema::registry::{
    DefinitionStability, DialectDefinition, QualifiedName, RegistryBatch, SchemaRegistry,
};
use crate::hdl::hdlir::version::HdlirVersion;

/// Canonical qualified name of the built-in memory dialect.
///
/// This is a schema identity, not a target or physical-memory identifier.
pub const MEMORY_DIALECT_NAME: &str = "zamani.memory";

/// Version of the memory-dialect contract.
///
/// This version is independent of the Rust crate version, HDL language
/// version, and serialized HDLIR format version.
pub const MEMORY_DIALECT_VERSION: HdlirVersion = HdlirVersion::new(1, 0, 0);

/// Policy for validating memory-port binding completeness.
///
/// Structural validation always checks every declaration and its local
/// invariants. This policy controls whether every port declared by a
/// `MemoryType` must also have a corresponding binding record.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MemoryPortBindingPolicy {
    /// Permit missing binding records during incremental construction.
    ///
    /// This policy does not make an incomplete design ready for realization.
    /// The enclosing design must apply its required completeness checks
    /// before accepting the design as complete.
    #[default]
    AllowIncomplete,

    /// Require a binding record for every declared logical memory port.
    ///
    /// This checks record completeness, not whether each record contains all
    /// schema-required endpoint bindings. Those requirements depend on the
    /// registered memory-port contract and must be checked separately.
    RequireDeclaredPorts,
}

/// Stateless entry point for the built-in HDLIR memory dialect.
///
/// Memory declarations and types remain owned by their canonical modules.
/// This type coordinates registration and the checks appropriate to this
/// dialect without copying or taking ownership of the graph.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MemoryDialect;

impl MemoryDialect {
    /// Return the qualified identity of this dialect.
    ///
    /// # Errors
    ///
    /// Returns an error if the built-in name violates the shared qualified
    /// name contract. Such an error indicates an implementation defect.
    pub fn name() -> HdlirResult<QualifiedName> {
        QualifiedName::parse(MEMORY_DIALECT_NAME)
    }

    /// Construct the versioned dialect definition for registry insertion.
    ///
    /// The definition identifies the dialect namespace. It does not claim
    /// that every memory operation or backend is implemented.
    ///
    /// # Errors
    ///
    /// Returns an error if the name or definition violates the shared schema
    /// registry's construction contract.
    pub fn definition() -> HdlirResult<DialectDefinition> {
        DialectDefinition::new(
            Self::name()?,
            MEMORY_DIALECT_VERSION,
            DefinitionStability::Core,
            "Target-independent logical memory declarations, access intent, \
             and memory-specific verification contracts.",
        )
    }

    /// Register the memory dialect in the shared schema registry.
    ///
    /// Registration is atomic through `SchemaRegistry::register_batch`.
    /// Existing definitions are not replaced. A duplicate registration
    /// returns the registry's structured error and leaves its state intact.
    ///
    /// This function registers the dialect identity only. Memory operations
    /// must be registered separately when their operand, result, attribute,
    /// effect, and semantic-verification contracts are defined.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid definitions, duplicate registration,
    /// registry policy violations, or registry generation exhaustion.
    pub fn register(
        registry: &mut SchemaRegistry,
        extension_policy: ExtensionPolicy,
    ) -> HdlirResult<u64> {
        let batch = RegistryBatch::new().with_dialect(Self::definition()?);
        registry.register_batch(batch, extension_policy)
    }

    /// Verify local structure and the selected port-binding completeness
    /// policy for every memory declaration in a table.
    ///
    /// This method does not resolve module, signal, value, or clock-domain
    /// references. Use [`Self::verify_references`] for those checks.
    ///
    /// # Errors
    ///
    /// Returns the first structural, index-consistency, or completeness
    /// error. A failure means the requested verification did not succeed.
    pub fn verify_declarations(
        memories: &HdlirMemoryTable,
        binding_policy: MemoryPortBindingPolicy,
    ) -> HdlirResult<()> {
        memories.validate_structure()?;

        for memory in memories.iter() {
            memory.validate_structure()?;

            if binding_policy == MemoryPortBindingPolicy::RequireDeclaredPorts {
                memory.validate_port_binding_completeness()?;
            }
        }

        Ok(())
    }

    /// Verify declaration structure, selected completeness rules, and graph
    /// references for all memories in the supplied table.
    ///
    /// The reference callbacks must resolve IDs against the containing
    /// design and enforce any required namespace and visibility rules.
    /// This method deliberately does not infer types or binding meanings.
    ///
    /// # Errors
    ///
    /// Returns the first structural, completeness, or unresolved-reference
    /// error. It never converts an incomplete check into successful
    /// verification.
    pub fn verify_references<FM, FS, FV, FC>(
        memories: &HdlirMemoryTable,
        binding_policy: MemoryPortBindingPolicy,
        module_exists: FM,
        signal_exists: FS,
        value_exists: FV,
        clock_domain_exists: FC,
    ) -> HdlirResult<()>
    where
        FM: FnMut(HdlirModuleId) -> bool,
        FS: FnMut(HdlirSignalId) -> bool,
        FV: FnMut(HdlirValueId) -> bool,
        FC: FnMut(HdlirClockDomainId) -> bool,
    {
        Self::verify_declarations(memories, binding_policy)?;

        memories.validate_module_references(module_exists)?;

        memories.validate_endpoint_references(
            signal_exists,
            value_exists,
            clock_domain_exists,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialect_name_and_version_are_stable() {
        let name = MemoryDialect::name().expect("valid built-in dialect name");

        assert_eq!(name.as_str(), MEMORY_DIALECT_NAME);
        assert_eq!(MEMORY_DIALECT_VERSION.major(), 1);
        assert_eq!(MEMORY_DIALECT_VERSION.minor(), 0);
        assert_eq!(MEMORY_DIALECT_VERSION.patch(), 0);
    }

    #[test]
    fn registration_adds_the_dialect_to_the_shared_registry() {
        let mut registry = SchemaRegistry::new();

        let generation = MemoryDialect::register(
            &mut registry,
            ExtensionPolicy::StableOnly,
        )
        .expect("built-in memory dialect should register");

        assert_eq!(generation, 1);

        let name = MemoryDialect::name().expect("valid dialect name");
        assert!(registry.dialect(&name).is_some());
    }

    #[test]
    fn duplicate_registration_is_rejected_without_changing_generation() {
        let mut registry = SchemaRegistry::new();

        MemoryDialect::register(&mut registry, ExtensionPolicy::StableOnly)
            .expect("first registration should succeed");

        let generation_before = registry.generation();

        assert!(
            MemoryDialect::register(&mut registry, ExtensionPolicy::StableOnly).is_err()
        );
        assert_eq!(registry.generation(), generation_before);
    }

    #[test]
    fn empty_memory_table_is_structurally_valid() {
        let memories = HdlirMemoryTable::new();

        assert!(
            MemoryDialect::verify_declarations(
                &memories,
                MemoryPortBindingPolicy::RequireDeclaredPorts,
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_table_passes_reference_validation() {
        let memories = HdlirMemoryTable::new();

        assert!(
            MemoryDialect::verify_references(
                &memories,
                MemoryPortBindingPolicy::RequireDeclaredPorts,
                |_| false,
                |_| false,
                |_| false,
                |_| false,
            )
            .is_ok()
        );
    }
}
