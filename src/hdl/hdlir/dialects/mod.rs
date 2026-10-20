
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Dialects
//!
//! Path: `src/hdl/hdlir/dialects/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Compose the implemented HDLIR dialects and provide one explicit entry
//! point for registering their identities in the canonical HDLIR schema
//! registry.
//!
//! ## Architectural authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - Applicable feature grammars under `grammar/hdl/`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/validation/semantic-boundaries.md`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/compatibility.md`.
//! - `src/hdl/hdlir/config.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/schema/registry.rs`.
//! - `src/hdl/hdlir/version.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! The grammar owns source syntax. The frontend owns source-level semantic
//! truth. ZUIR owns universal computational representation. HDLIR owns
//! hardware-domain representation and dialect contracts. Target realization
//! owns physical allocation, placement, routing, and technology-specific
//! implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The Rust composition boundary for implemented HDLIR dialects.
//! - Canonical built-in dialect identity registration.
//! - The initial versions of dialect identities owned by this composition
//!   boundary.
//! - The atomic registration entry point for the built-in dialect set.
//!
//! It does not own:
//!
//! - A second schema registry or an alternative operation vocabulary.
//! - Source parsing, AST construction, or semantic analysis.
//! - Concrete operation implementations or operation execution.
//! - A second design, module, signal, memory, or connectivity representation.
//! - Hardware discovery, physical allocation, synthesis, placement, routing,
//!   clock-tree synthesis, or timing closure.
//! - A guarantee that every registered dialect operation is implemented
//!   or supported by every target.
//!
//! ## Dialect identity versus implementation availability
//!
//! Registering a dialect identity establishes its namespace, version,
//! stability classification, and description in the canonical registry.
//! It does not register operation definitions automatically, prove operation
//! semantics, or certify target support.
//!
//! Consumers MUST inspect the registered operation/type/interface contracts
//! and the applicable implementation capabilities before using an operation.
//! Missing required definitions or semantics MUST result in an explicit
//! failure. Unknown optional definitions may be preserved opaquely only when
//! the serialization and compatibility contracts permit it.
//!
//! ## Extensibility
//!
//! The built-in set is not a closed list of every hardware dialect Zamani
//! may ever support. Third-party, vendor, analog/mixed-signal, high-level
//! synthesis, reconfigurable-fabric, and future dialects can be added through
//! the shared schema registry and its extension policy.
//!
//! Adding a dialect MUST NOT require changing the core IR merely to introduce
//! a new operation family. Extensions MUST NOT redefine existing universal
//! constructs or bypass required verification.
//!
//! The built-in registration function deliberately registers only the five
//! dialects whose implementation modules already exist in this directory.
//! It does not scan the filesystem, discover plugins implicitly, execute
//! extension code, or depend on registration order outside its own batch.
//!
//! ## Scalability
//!
//! This module defines no universal hardware limits and no fixed maximum
//! number of dialects or schema definitions. Registry storage and work scale
//! with the definitions registered and the resources available.
//!
//! All registration failures are returned through the canonical HDLIR error
//! contract. The registry is responsible for duplicate detection, policy
//! enforcement, generation overflow checking, and atomicity.
//!
//! No software can process literally infinite input. Resource exhaustion,
//! representational limits, or operational-budget exhaustion must be handled
//! by the owning component and must not be presented as successful,
//! complete processing.
//!
//! ## Determinism
//!
//! Dialect identities use canonical qualified names. The schema registry
//! owns deterministic storage, lookup, snapshots, and duplicate handling.
//! This module introduces no global mutable state, hidden initialization,
//! filesystem traversal, or environment-dependent registration.
//!
//! ## Registration behavior
//!
//! `register_builtin_dialects` submits all five identities as one batch.
//! If registration fails, the canonical registry must retain its previous
//! definitions and generation unchanged. Repeated registration is not
//! silently treated as success: the registry reports duplicate definitions.
//!
//! The caller owns the registry and chooses the extension policy. This module
//! does not override that policy or permit experimental extensions implicitly.
//!
//! ## Integration
//!
//! `src/hdl/hdlir/mod.rs` must register this module after its foundational
//! modules and `schema` module are declared.
//!
//! The canonical registry is `crate::hdl::hdlir::schema::registry`.
//! The canonical policy is `crate::hdl::hdlir::config::ExtensionPolicy`.
//! The canonical version and error contracts are reused directly.
//!
//! Structural, behavioral, and sequential identity descriptors are composed
//! here because their current modules do not all expose a common registration
//! API. Memory and physical-intent definitions are obtained from their
//! existing module APIs; their identities are not duplicated.
//!
//! Future changes to a dialect's public name or initial version require a
//! reviewed compatibility change and corresponding tests.
//!
//! ## Freeze criteria
//!
//! - Every declared child module exists and compiles.
//! - All built-in identities are valid canonical qualified names.
//! - Registration is atomic and duplicate registration fails explicitly.
//! - No second registry or parallel dialect identity system is introduced.
//! - No target-specific realization behavior is introduced.
//! - Repository formatting, compilation, tests, and Clippy checks pass.
//!
//! Passing these criteria for this file alone does not establish production
//! readiness of the complete HDLIR subsystem.

use crate::hdl::hdlir::config::ExtensionPolicy;
use crate::hdl::hdlir::errors::HdlirResult;
use crate::hdl::hdlir::schema::registry::{
    DefinitionStability,
    DialectDefinition,
    QualifiedName,
    RegistryBatch,
    SchemaRegistry,
};
use crate::hdl::hdlir::version::HdlirVersion;

/// Initial version of built-in dialect identities declared by this module.
///
/// This is a dialect-contract version, not the Zamani language version,
/// crate version, HDLIR artifact-format version, or target version.
///
/// A released dialect must keep this value consistent with its normative
/// specification and compatibility policy.
pub const BUILTIN_DIALECT_VERSION: HdlirVersion = HdlirVersion::new(1, 0, 0);

/// Canonical qualified name of the structural composition dialect.
///
/// This identifies logical hierarchy, instances, parameter bindings, and
/// structural port relationships. It does not identify physical cells,
/// pins, or placed hardware resources.
pub const STRUCTURAL_DIALECT_NAME: &str = "zamani.structural";

/// Canonical qualified name of the sequential dialect.
///
/// This identity covers logical state-transition intent. It does not identify
/// physical registers, clock resources, or a technology-specific primitive.
pub const SEQUENTIAL_DIALECT_NAME: &str = "zamani.sequential";

/// Structural hardware composition.
///
/// Owns structural instance and binding contracts. The canonical design,
/// module, port, and connectivity representations remain owned by `ir`.
pub mod structural;

/// Behavioral hardware processes.
///
/// Owns behavioral-dialect verification contracts. Canonical process records
/// and operation definitions remain owned by the IR and schema subsystems.
pub mod behavioral;

/// Sequential and state-transition intent.
///
/// Owns sequential-specific contracts without replacing canonical process,
/// signal, value, clock, or reset identities.
pub mod sequential;

/// Abstract logical memory declarations and access contracts.
///
/// Physical memory allocation and technology mapping remain downstream.
pub mod memory;

/// Explicit target-aware implementation intent.
///
/// This dialect distinguishes target-dependent commitments from portable
/// semantics; it does not implement physical realization.
pub mod physical_intent;

/// Construct a built-in dialect identity using the shared schema contract.
///
/// This helper creates metadata only. It does not create operation
/// definitions or assert that operations have executable implementations.
fn built_in_definition(
    name: &str,
    description: &'static str,
) -> HdlirResult<DialectDefinition> {
    DialectDefinition::new(
        QualifiedName::parse(name)?,
        BUILTIN_DIALECT_VERSION,
        DefinitionStability::Core,
        description,
    )
}

/// Register all currently implemented built-in dialect identities atomically.
///
/// The five identities are:
///
/// - [`STRUCTURAL_DIALECT_NAME`]
/// - [`behavioral::DIALECT_NAME`]
/// - [`SEQUENTIAL_DIALECT_NAME`]
/// - [`memory::MEMORY_DIALECT_NAME`]
/// - [`physical_intent::PHYSICAL_INTENT_DIALECT_NAME`]
///
/// Structural, behavioral, and sequential identities use the common
/// composition contract in this module. Memory and physical-intent identities
/// are obtained through their existing definition APIs to avoid duplicate
/// identity definitions.
///
/// The caller controls extension policy. All built-in definitions are marked
/// `Core`; policy enforcement remains the responsibility of the canonical
/// registry.
///
/// # Atomicity
///
/// All identities are submitted in one `RegistryBatch`. The registry either
/// registers the entire batch or returns an error without partially
/// modifying its prior state.
///
/// # Errors
///
/// Returns the canonical HDLIR error if a name or definition is invalid,
/// a dialect is already registered, the extension policy rejects a
/// definition, or the registry cannot advance its generation safely.
///
/// # Example
///
/// ```ignore
/// use crate::hdl::hdlir::config::ExtensionPolicy;
/// use crate::hdl::hdlir::dialects::register_builtin_dialects;
/// use crate::hdl::hdlir::schema::registry::SchemaRegistry;
///
/// let mut registry = SchemaRegistry::new();
/// register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly)?;
/// # Ok::<(), crate::hdl::hdlir::errors::HdlirError>(())
/// ```
///
/// This registers dialect identities only. Callers must separately register
/// any required operation, type, and interface definitions before constructing
/// or accepting IR that depends on them.
pub fn register_builtin_dialects(
    registry: &mut SchemaRegistry,
    extension_policy: ExtensionPolicy,
) -> HdlirResult<u64> {
    let structural = built_in_definition(
        STRUCTURAL_DIALECT_NAME,
        "Target-independent logical hardware hierarchy, instances, \
         parameter bindings, and structural port relationships.",
    )?;

    let behavioral = built_in_definition(
        behavioral::DIALECT_NAME,
        "Logical hardware processes and their behavioral verification \
         contracts.",
    )?;

    let sequential = built_in_definition(
        SEQUENTIAL_DIALECT_NAME,
        "Logical sequential state-transition, clocked-update, and reset \
         intent without physical register allocation.",
    )?;

    // Reuse the dialects' own canonical definitions rather than duplicating
    // their names, versions, or descriptions here.
    let memory = memory::MemoryDialect::definition()?;
    let physical_intent = physical_intent::PhysicalIntentDialect::definition()?;

    let batch = RegistryBatch::new()
        .with_dialect(structural)
        .with_dialect(behavioral)
        .with_dialect(sequential)
        .with_dialect(memory)
        .with_dialect(physical_intent);

    registry.register_batch(batch, extension_policy)
}

#[cfg(test)]
mod tests {
    use super::{
        register_builtin_dialects,
        BUILTIN_DIALECT_VERSION,
        SEQUENTIAL_DIALECT_NAME,
        STRUCTURAL_DIALECT_NAME,
    };
    use crate::hdl::hdlir::config::ExtensionPolicy;
    use crate::hdl::hdlir::schema::registry::{
        QualifiedName,
        SchemaRegistry,
    };
    use crate::hdl::hdlir::version::HdlirVersion;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("built-in dialect name must be valid")
    }

    #[test]
    fn built_in_version_is_independent_of_artifact_versioning() {
        assert_eq!(BUILTIN_DIALECT_VERSION, HdlirVersion::new(1, 0, 0));
    }

    #[test]
    fn registration_adds_all_five_built_in_identities() {
        let mut registry = SchemaRegistry::new();

        let generation =
            register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly)
                .expect("built-in registration should succeed");

        assert_eq!(generation, registry.generation());

        let snapshot = registry.snapshot();
        assert_eq!(snapshot.dialects().len(), 5);

        for dialect_name in [
            STRUCTURAL_DIALECT_NAME,
            super::behavioral::DIALECT_NAME,
            SEQUENTIAL_DIALECT_NAME,
            super::memory::MEMORY_DIALECT_NAME,
            super::physical_intent::PHYSICAL_INTENT_DIALECT_NAME,
        ] {
            assert!(
                snapshot.dialect(&name(dialect_name)).is_some(),
                "missing built-in dialect: {dialect_name}",
            );
        }
    }

    #[test]
    fn built_in_identities_use_expected_contract_versions() {
        let mut registry = SchemaRegistry::new();

        register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly)
            .expect("built-in registration should succeed");

        let snapshot = registry.snapshot();

        for dialect_name in [
            STRUCTURAL_DIALECT_NAME,
            super::behavioral::DIALECT_NAME,
            SEQUENTIAL_DIALECT_NAME,
            super::memory::MEMORY_DIALECT_NAME,
            super::physical_intent::PHYSICAL_INTENT_DIALECT_NAME,
        ] {
            let definition = snapshot
                .dialect(&name(dialect_name))
                .expect("built-in identity should exist");

            assert_eq!(
                definition.version(),
                BUILTIN_DIALECT_VERSION,
                "unexpected dialect version: {dialect_name}",
            );
        }
    }

    #[test]
    fn duplicate_registration_fails_without_partial_mutation() {
        let mut registry = SchemaRegistry::new();

        register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly)
            .expect("first registration should succeed");

        let generation_before = registry.generation();
        let snapshot_before = registry.snapshot();

        let result =
            register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly);

        assert!(
            result.is_err(),
            "duplicate dialect registration must not silently succeed",
        );
        assert_eq!(registry.generation(), generation_before);
        assert_eq!(registry.snapshot(), snapshot_before);
    }

    #[test]
    fn registration_does_not_depend_on_experimental_extension_permission() {
        let mut registry = SchemaRegistry::new();

        register_builtin_dialects(&mut registry, ExtensionPolicy::StableOnly)
            .expect("core dialect identities should register under stable-only policy");

        assert_eq!(registry.snapshot().dialects().len(), 5);
    }
}
