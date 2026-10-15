//! ZUIR dialect-system conformance tests.
//!
//! Path: src/zuir/tests/dialects.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Test the existing dialect manifest, operation-schema, registry, and
//! extensibility contracts without introducing a competing dialect model.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/dialect/definition.rs
//! - src/zuir/dialect/operation.rs
//! - src/zuir/dialect/interfaces.rs
//! - src/zuir/dialect/registry.rs
//! - src/zuir/dialect/versioning.rs
//! - src/zuir/schema/types.rs
//! - src/zuir/version.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compatibility.md
//! - grammar/specification/extensibility.md, when present
//!
//! OWNS
//!
//! Regression tests for dialect declarations, extensible operation names,
//! schema validation, dependency handling, registry atomicity, export
//! resolution, deterministic enumeration, and dynamic growth.
//!
//! DOES_NOT_OWN
//!
//! These tests do not implement dialect registration, operation semantics,
//! source parsing, semantic analysis, executable plugin loading, whole-program
//! verification, or target lowering.
//!
//! INTEGRATION CONTRACT
//!
//! - definition.rs owns declarative dialect manifests.
//! - operation.rs owns operation schemas.
//! - interfaces.rs owns interface contracts.
//! - registry.rs owns registry state and cross-dialect validation.
//! - versioning.rs owns compatibility decisions and migration plans.
//! - schema/types.rs owns QualifiedName and SchemaType.
//! - version.rs owns version identities.
//! - The source grammar and semantic model remain authoritative for language
//!   syntax and source-level meaning.
//!
//! Register this file from the existing ZUIR test-module hierarchy only after
//! confirming that hierarchy's actual module declarations.
//!
//! SCALABILITY
//!
//! Collections are dynamically sized. Test workload sizes are regression
//! parameters, not universal limits on dialects, operations, hardware, or
//! computational resources.
//!
//! SAFETY AND TRUST
//!
//! A valid manifest or registered operation schema does not prove that its
//! implementation is trusted, executable, semantically correct, or supported
//! by a target. Those properties require separate contracts.
//!
//! FREEZE CRITERIA
//!
//! - Tests exercise canonical production types.
//! - Invalid declarations are rejected.
//! - Failed registration leaves the registry unchanged.
//! - Export resolution checks both identity and category.
//! - Unknown effects are not treated as known-pure effects.
//! - Ordering is deterministic.
//! - No fixed list of domain operation names is required.
//! - All tests compile and pass against the integrated repository.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;

use crate::zuir::dialect::definition::{
    DialectCompatibility,
    DialectDefinition,
    DialectDependency,
    DialectExport,
    DialectExportName,
    DialectImplementation,
    DialectName,
    DialectTrust,
    ExportKind,
    VersionRequirement,
};
use crate::zuir::dialect::operation::{
    Cardinality,
    EffectDeclaration,
    OperandGroup,
    OperationDefinition,
    ResultGroup,
};
use crate::zuir::dialect::registry::{
    DialectRegistry,
    ExportReference,
    RegistryError,
};
use crate::zuir::schema::types::{
    QualifiedName,
    SchemaType,
};
use crate::zuir::version::{
    DialectVersion,
    INITIAL_DIALECT_VERSION,
};

// -----------------------------------------------------------------------------
// Shared test fixtures
// -----------------------------------------------------------------------------

fn qualified_name(value: &str) -> QualifiedName {
    QualifiedName::parse(value)
        .expect("test qualified name must be valid")
}

fn dialect_name(value: &str) -> DialectName {
    DialectName::parse(value)
        .expect("test dialect name must be valid")
}

fn export_name(value: &str) -> DialectExportName {
    DialectExportName::parse(value)
        .expect("test export name must be valid")
}

fn compatibility() -> DialectCompatibility {
    DialectCompatibility::new(
        VersionRequirement::registry_policy(),
        VersionRequirement::registry_policy(),
        false,
    )
    .expect("test compatibility declaration must be valid")
}

fn dialect_definition(name: &str) -> DialectDefinition {
    DialectDefinition::new(
        dialect_name(name),
        INITIAL_DIALECT_VERSION,
        compatibility(),
        DialectImplementation::new(DialectTrust::DeclarativeOnly),
    )
    .expect("test dialect definition must be valid")
}

fn dialect_export(name: &str, kind: ExportKind) -> DialectExport {
    DialectExport::new(export_name(name), kind)
        .expect("test export declaration must be valid")
}

fn operation_definition(
    dialect: &str,
    operation: &str,
) -> OperationDefinition {
    OperationDefinition::new(
        dialect_name(dialect),
        export_name(operation),
        INITIAL_DIALECT_VERSION,
    )
    .expect("test operation definition must be valid")
}

fn register(
    registry: &mut DialectRegistry,
    definition: DialectDefinition,
) {
    registry
        .register(definition)
        .expect("valid test dialect must register");
}

// -----------------------------------------------------------------------------
// Dialect definition contracts
// -----------------------------------------------------------------------------

#[test]
fn creates_and_validates_an_empty_dialect() {
    let definition = dialect_definition("example");

    assert!(definition.validate().is_ok());
    assert_eq!(definition.export_count(), 0);
    assert_eq!(definition.dependency_count(), 0);
}

#[test]
fn supports_namespaced_dialect_names() {
    let definition = dialect_definition("vendor::scientific::compute");

    assert_eq!(
        definition.name().to_string(),
        "vendor::scientific::compute"
    );
    assert!(definition.validate().is_ok());
}

#[test]
fn supports_open_ended_operation_export_names() {
    let mut definition = dialect_definition("quantum");

    definition
        .add_export(dialect_export(
            "future::adaptive_operation",
            ExportKind::Operation,
        ))
        .expect("new operation name must be accepted");

    assert_eq!(definition.export_count(), 1);
    assert!(definition.validate().is_ok());
}

#[test]
fn rejects_duplicate_exports() {
    let mut definition = dialect_definition("example");
    let export = dialect_export("example::calculate", ExportKind::Operation);

    definition
        .add_export(export.clone())
        .expect("first export must succeed");

    assert!(definition.add_export(export).is_err());
    assert_eq!(definition.export_count(), 1);
}

#[test]
fn keeps_exports_in_deterministic_name_order() {
    let mut definition = dialect_definition("example");

    for name in [
        "example::zeta",
        "example::alpha",
        "example::middle",
    ] {
        definition
            .add_export(dialect_export(name, ExportKind::Operation))
            .expect("unique export must succeed");
    }

    let names: Vec<String> = definition
        .exports()
        .map(|export| export.name().to_string())
        .collect();

    assert_eq!(
        names,
        vec![
            "example::alpha",
            "example::middle",
            "example::zeta",
        ]
    );
}

#[test]
fn distinguishes_export_categories() {
    let mut definition = dialect_definition("example");

    definition
        .add_export(dialect_export(
            "example::payload",
            ExportKind::Type,
        ))
        .expect("type export must succeed");

    let name = export_name("example::payload");

    assert!(definition.has_export(&name, ExportKind::Type));
    assert!(!definition.has_export(&name, ExportKind::Operation));
}

#[test]
fn accepts_extension_interfaces_without_core_enum_changes() {
    let mut definition = dialect_definition("future_domain");

    definition
        .add_interface(qualified_name("future_domain::streaming"))
        .expect("extension interface must be accepted");

    let interfaces: Vec<String> = definition
        .interfaces()
        .map(QualifiedName::canonical_string)
        .collect();

    assert_eq!(interfaces, vec!["future_domain::streaming"]);
    assert!(definition.validate().is_ok());
}

#[test]
fn rejects_a_dialect_dependency_on_itself() {
    let mut definition = dialect_definition("example");

    let dependency = DialectDependency::new(
        dialect_name("example"),
        VersionRequirement::registry_policy(),
        false,
    )
    .expect("dependency declaration itself must be valid");

    assert!(definition.add_dependency(dependency).is_err());
    assert_eq!(definition.dependency_count(), 0);
}

#[test]
fn rejects_duplicate_dependencies() {
    let mut definition = dialect_definition("consumer");

    let first = DialectDependency::new(
        dialect_name("provider"),
        VersionRequirement::registry_policy(),
        false,
    )
    .expect("first dependency must be valid");

    let second = DialectDependency::new(
        dialect_name("provider"),
        VersionRequirement::registry_policy(),
        true,
    )
    .expect("second dependency must be valid");

    definition
        .add_dependency(first)
        .expect("first dependency must be accepted");

    assert!(definition.add_dependency(second).is_err());
    assert_eq!(definition.dependency_count(), 1);
}

#[test]
fn accepts_inert_descriptive_metadata() {
    let mut definition = dialect_definition("example");

    definition
        .insert_metadata("documentation", "Example dialect")
        .expect("valid metadata must be accepted");

    assert_eq!(
        definition.metadata().collect::<Vec<_>>(),
        vec![("documentation", "Example dialect")]
    );
    assert!(definition.validate().is_ok());
}

#[test]
fn rejects_invalid_metadata_keys() {
    let mut definition = dialect_definition("example");

    assert!(definition.insert_metadata("", "value").is_err());
    assert!(definition.insert_metadata(" invalid ", "value").is_err());
}

// -----------------------------------------------------------------------------
// Operation schema contracts
// -----------------------------------------------------------------------------

#[test]
fn creates_a_valid_operation_schema() {
    let definition =
        operation_definition("classical", "classical::calculate");

    assert_eq!(definition.dialect(), &dialect_name("classical"));
    assert_eq!(
        definition.name(),
        &export_name("classical::calculate")
    );
    assert!(definition.validate().is_ok());
}

#[test]
fn operation_schemas_default_to_unknown_effects() {
    let definition =
        operation_definition("example", "example::external_operation");

    assert!(!definition.effects().is_known());
    assert!(definition.effects().effects().is_none());
    assert!(definition.validate().is_ok());
}

#[test]
fn allows_an_explicitly_known_empty_effect_set() {
    let mut definition =
        operation_definition("example", "example::pure_operation");

    definition
        .set_effects(
            EffectDeclaration::known(std::iter::empty())
                .expect("empty effect set must be valid"),
        )
        .expect("effect declaration must be accepted");

    assert!(definition.effects().is_known());
    assert_eq!(
        definition.effects().effects(),
        Some(&BTreeSet::new())
    );
    assert!(definition.validate().is_ok());
}

#[test]
fn supports_extension_defined_effect_names() {
    let mut definition =
        operation_definition("hybrid", "hybrid::custom_operation");

    definition
        .set_effects(
            EffectDeclaration::known([
                qualified_name("quantum::measurement"),
                qualified_name("distributed::communication"),
            ])
            .expect("qualified extension effects must be valid"),
        )
        .expect("effect declaration must be accepted");

    let effects = definition
        .effects()
        .effects()
        .expect("effect set must be known");

    assert_eq!(effects.len(), 2);
    assert!(effects.contains(&qualified_name("quantum::measurement")));
    assert!(effects.contains(&qualified_name("distributed::communication")));
}

#[test]
fn supports_variadic_operand_and_result_groups() {
    let mut definition =
        operation_definition("tensor", "tensor::transform");

    let operand = OperandGroup::new(
        qualified_name("input"),
        SchemaType::integer(),
        Cardinality::variadic(),
    )
    .expect("variadic operand group must be valid");

    let result = ResultGroup::new(
        qualified_name("output"),
        SchemaType::integer(),
        Cardinality::variadic(),
    )
    .expect("variadic result group must be valid");

    definition
        .add_operand_group(operand)
        .expect("operand group must be accepted");

    definition
        .add_result_group(result)
        .expect("result group must be accepted");

    assert_eq!(definition.operands().len(), 1);
    assert_eq!(definition.results().len(), 1);
    assert!(definition.validate().is_ok());
}

#[test]
fn operation_schema_supports_domain_specific_versions() {
    let version = DialectVersion::new(7, 3, 2);

    let definition = OperationDefinition::new(
        dialect_name("vendor::domain"),
        export_name("vendor::domain::specialized"),
        version,
    )
    .expect("valid operation identity must be accepted");

    assert_eq!(definition.version(), version);
    assert!(definition.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Registry behavior and export resolution
// -----------------------------------------------------------------------------

#[test]
fn starts_with_an_empty_registry() {
    let registry = DialectRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
    assert!(registry.iter().next().is_none());
}

#[test]
fn registers_and_retrieves_a_dialect() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("example"));

    assert_eq!(registry.len(), 1);
    assert!(registry.get_by_name("example").is_some());
    assert!(registry.get_by_name("not::registered").is_none());
}

#[test]
fn invalid_lookup_does_not_mutate_registry() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("example"));

    assert!(registry.get_by_name("invalid name").is_none());
    assert_eq!(registry.len(), 1);
}

#[test]
fn rejects_duplicate_registration_atomically() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("example"));

    let error = registry
        .register(dialect_definition("example"))
        .expect_err("duplicate dialect must be rejected");

    assert!(matches!(error, RegistryError::DuplicateDialect(_)));
    assert_eq!(registry.len(), 1);
    assert!(registry.get_by_name("example").is_some());
}

#[test]
fn rejects_a_missing_required_dependency_atomically() {
    let mut registry = DialectRegistry::new();
    let mut consumer = dialect_definition("consumer");

    consumer
        .add_dependency(
            DialectDependency::new(
                dialect_name("missing_provider"),
                VersionRequirement::registry_policy(),
                false,
            )
            .expect("dependency declaration must be valid"),
        )
        .expect("dependency must be declared");

    let result = registry.register(consumer);

    assert!(matches!(
        result,
        Err(RegistryError::MissingDependency { .. })
    ));
    assert!(registry.is_empty());
}

#[test]
fn permits_a_missing_optional_dependency() {
    let mut registry = DialectRegistry::new();
    let mut consumer = dialect_definition("consumer");

    consumer
        .add_dependency(
            DialectDependency::new(
                dialect_name("optional_provider"),
                VersionRequirement::registry_policy(),
                true,
            )
            .expect("optional dependency must be valid"),
        )
        .expect("optional dependency must be declared");

    registry
        .register(consumer)
        .expect("missing optional dependency must be permitted");

    assert_eq!(registry.len(), 1);
}

#[test]
fn registers_a_valid_dependency_batch_atomically() {
    let mut registry = DialectRegistry::new();

    let mut consumer = dialect_definition("consumer");

    consumer
        .add_dependency(
            DialectDependency::new(
                dialect_name("provider"),
                VersionRequirement::registry_policy(),
                false,
            )
            .expect("dependency must be valid"),
        )
        .expect("dependency must be accepted");

    registry
        .register_batch([
            consumer,
            dialect_definition("provider"),
        ])
        .expect("batch should resolve dependencies");

    assert_eq!(registry.len(), 2);
    assert!(registry.get_by_name("consumer").is_some());
    assert!(registry.get_by_name("provider").is_some());
}

#[test]
fn failed_batch_registration_preserves_existing_state() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("existing"));

    let result = registry.register_batch([
        dialect_definition("new_dialect"),
        dialect_definition("existing"),
    ]);

    assert!(matches!(
        result,
        Err(RegistryError::DuplicateDialect(_))
    ));

    assert_eq!(registry.len(), 1);
    assert!(registry.get_by_name("existing").is_some());
    assert!(registry.get_by_name("new_dialect").is_none());
}

#[test]
fn resolves_a_declared_export() {
    let mut registry = DialectRegistry::new();
    let mut definition = dialect_definition("quantum");

    definition
        .add_export(dialect_export(
            "quantum::measure",
            ExportKind::Operation,
        ))
        .expect("operation export must be valid");

    register(&mut registry, definition);

    let reference = ExportReference::new(
        dialect_name("quantum"),
        export_name("quantum::measure"),
        ExportKind::Operation,
    );

    let resolved = registry
        .resolve_export(&reference)
        .expect("declared export must resolve");

    assert_eq!(resolved.kind(), ExportKind::Operation);
}

#[test]
fn rejects_export_resolution_when_category_does_not_match() {
    let mut registry = DialectRegistry::new();
    let mut definition = dialect_definition("example");

    definition
        .add_export(dialect_export(
            "example::payload",
            ExportKind::Type,
        ))
        .expect("type export must be valid");

    register(&mut registry, definition);

    let reference = ExportReference::new(
        dialect_name("example"),
        export_name("example::payload"),
        ExportKind::Operation,
    );

    assert!(matches!(
        registry.resolve_export(&reference),
        Err(RegistryError::ExportKindMismatch { .. })
    ));
}

#[test]
fn rejects_resolution_of_an_undeclared_export() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("example"));

    let reference = ExportReference::new(
        dialect_name("example"),
        export_name("example::missing"),
        ExportKind::Operation,
    );

    assert!(matches!(
        registry.resolve_export(&reference),
        Err(RegistryError::UnknownExport { .. })
    ));
}

#[test]
fn rejects_resolution_through_an_unknown_dialect() {
    let registry = DialectRegistry::new();

    let reference = ExportReference::new(
        dialect_name("missing"),
        export_name("missing::operation"),
        ExportKind::Operation,
    );

    assert!(matches!(
        registry.resolve_export(&reference),
        Err(RegistryError::UnknownDialect(_))
    ));
}

#[test]
fn refuses_to_remove_a_required_dependency() {
    let mut registry = DialectRegistry::new();

    let provider = dialect_definition("provider");
    let mut consumer = dialect_definition("consumer");

    consumer
        .add_dependency(
            DialectDependency::new(
                dialect_name("provider"),
                VersionRequirement::registry_policy(),
                false,
            )
            .expect("dependency must be valid"),
        )
        .expect("dependency must be added");

    registry
        .register_batch([provider, consumer])
        .expect("both dialects must register");

    assert!(registry.unregister(&dialect_name("provider")).is_err());
    assert_eq!(registry.len(), 2);
    assert!(registry.get_by_name("provider").is_some());
}

#[test]
fn allows_removal_of_an_unused_dialect() {
    let mut registry = DialectRegistry::new();

    register(&mut registry, dialect_definition("unused"));

    let removed = registry
        .unregister(&dialect_name("unused"))
        .expect("unused dialect must be removable");

    assert_eq!(removed.name(), &dialect_name("unused"));
    assert!(registry.is_empty());
}

// -----------------------------------------------------------------------------
// Determinism and dynamic growth
// -----------------------------------------------------------------------------

#[test]
fn registry_enumeration_is_deterministic() {
    let mut registry = DialectRegistry::new();

    for name in ["zeta", "alpha", "middle", "beta"] {
        register(&mut registry, dialect_definition(name));
    }

    let names: Vec<String> = registry
        .names()
        .map(ToString::to_string)
        .collect();

    assert_eq!(names, vec!["alpha", "beta", "middle", "zeta"]);
}

#[test]
fn accepts_a_dynamically_sized_set_of_dialects() {
    // A regression workload only, not a maximum supported dialect count.
    let count: usize = 128;
    let mut registry = DialectRegistry::new();

    let definitions = (0..count).map(|index| {
        dialect_definition(&format!("test_domain::dialect_{index}"))
    });

    registry
        .register_batch(definitions)
        .expect("independent dialect batch must register");

    assert_eq!(registry.len(), count);

    for index in 0..count {
        let name = format!("test_domain::dialect_{index}");
        assert!(registry.get_by_name(&name).is_some());
    }
}

#[test]
fn unknown_operations_remain_declarative_until_resolved() {
    let mut registry = DialectRegistry::new();
    let mut definition = dialect_definition("future_domain");

    definition
        .add_export(dialect_export(
            "future_domain::not_yet_executable",
            ExportKind::Operation,
        ))
        .expect("future operation declaration must be valid");

    register(&mut registry, definition);

    let reference = ExportReference::new(
        dialect_name("future_domain"),
        export_name("future_domain::not_yet_executable"),
        ExportKind::Operation,
    );

    // Successful resolution proves that the manifest declares this name.
    // It does not prove that an implementation or target exists.
    assert!(registry.resolve_export(&reference).is_ok());
}