
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Classical Interoperability Integration Tests
//!
//! File: `src/classical/tests/interoperability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the integration contracts for:
//! - ABI declarations and validation;
//! - foreign-function declaration resolution;
//! - data-exchange contracts;
//! - explicit compatibility evidence and policy;
//! - open-world extension identifiers;
//! - deterministic metadata and parameter ordering;
//! - large generated contract collections;
//! - failure handling and non-execution guarantees.
//!
//! ## Architectural authority
//!
//! - `grammar/Zamani.g4`: canonical grammar composition.
//! - `grammar/interoperability/abi.g4`: ABI source syntax.
//! - `grammar/spec/data.md`: data semantics.
//! - `grammar/specification/semantic-model.md`: language semantics.
//! - `grammar/specification/scalability-model.md`: resource-parametric scaling.
//! - `grammar/validation/semantic-boundaries.md`: semantic boundaries.
//! - `src/classical/interoperability/abi.rs`: ABI contracts.
//! - `src/classical/interoperability/foreign_calls.rs`: foreign-call adapters.
//! - `src/classical/interoperability/data_exchange.rs`: exchange contracts.
//! - `src/classical/interoperability/compatibility.rs`: compatibility decisions.
//! - `src/classical/error.rs`: shared classical errors.
//! - `src/quantum/ir/classical/extern_call.rs`: canonical external-call IR.
//!
//! ## Ownership
//!
//! This file owns tests only. Production contracts, ABI realizations,
//! data representations, codecs, and canonical IR belong to their owning
//! modules.
//!
//! ## Scalability
//!
//! Generated test workloads are configurable test inputs, not language
//! limits. No test asserts a universal maximum number of functions,
//! parameters, metadata entries, formats, or exchanged bytes.
//!
//! Physical resource exhaustion is not equivalent to source invalidity.
//! Real allocation, time, and execution budgets must be explicit at the
//! layer that performs the relevant work.
//!
//! ## Non-goals
//!
//! These tests do not execute native functions, load shared libraries,
//! perform network transfers, allocate hardware, or claim that a foreign
//! ABI is supported on every target.
//!
//! ## Integration
//!
//! Register this module from `src/classical/tests/mod.rs`. The production
//! modules under `src/classical/interoperability/` remain the owners of
//! their respective APIs.

use crate::classical::interoperability::abi::{
    AbiBoundaryKind,
    AbiCompatibility,
    AbiContract,
    AbiErrorKind,
    AbiFunction,
    AbiIdentifier,
    AbiLinkage,
    AbiParameter,
    AbiPassingMode,
    AbiProfile,
    AbiTypeRef,
    AbiVersion,
};
use crate::classical::interoperability::compatibility::{
    CompatibilityChecker,
    CompatibilityDimension,
    CompatibilityEvidence,
    CompatibilityFact,
    CompatibilityPolicy,
    CompatibilityStatus,
};
use crate::classical::interoperability::data_exchange::{
    DataExchangeContract,
    DataExchangeEndpoint,
    DataExchangeErrorKind,
    DataExchangeIdentifier,
    DataLossPolicy,
    IntegrityRequirement,
    OrderingGuarantee,
    TransferMode,
};
use crate::classical::interoperability::foreign_calls::{
    ForeignCallError,
    ForeignFunctionDeclaration,
};

// ---------------------------------------------------------------------------
// Shared test fixtures
// ---------------------------------------------------------------------------

fn endpoint(
    schema: &str,
    format: &str,
    encoding: &str,
    version: Option<&str>,
) -> DataExchangeEndpoint {
    DataExchangeEndpoint::new(
        schema,
        format,
        encoding,
        version.map(str::to_owned),
    )
    .expect("test endpoint must be valid")
}

fn exchange_contract() -> DataExchangeContract {
    DataExchangeContract::new(
        "zamani.test.exchange",
        endpoint(
            "zamani.test.customer",
            "zamani.test.format",
            "zamani.test.encoding",
            Some("v1"),
        ),
        endpoint(
            "zamani.test.customer",
            "zamani.test.format",
            "zamani.test.encoding",
            Some("v1"),
        ),
    )
    .expect("test exchange contract must be valid")
}

fn abi_function(
    name: &str,
    symbol: &str,
    parameter_names: &[&str],
) -> AbiFunction {
    let input_type = AbiTypeRef::new("zamani.test.integer")
        .expect("test input type must be valid")
        .with_passing(AbiPassingMode::ByValue);

    let parameters = parameter_names
        .iter()
        .map(|parameter_name| {
            AbiParameter::new(
                *parameter_name,
                input_type.clone(),
            )
            .expect("test parameter must be valid")
        })
        .collect();

    let result = AbiTypeRef::new("zamani.test.integer")
        .expect("test result type must be valid");

    AbiFunction::new(
        name,
        symbol,
        parameters,
        Some(result),
    )
    .expect("test ABI function must be valid")
}

fn abi_contract() -> AbiContract {
    let profile = AbiProfile::new(
        "zamani.test.portable",
        "zamani.test.calling-convention.v1",
        AbiLinkage::Host,
    )
    .expect("test ABI profile must be valid");

    let mut contract = AbiContract::new(
        "zamani.test.math",
        AbiBoundaryKind::Callable,
        profile,
    )
    .expect("test ABI contract must be valid");

    contract
        .add_function(abi_function(
            "add",
            "zamani_test_add",
            &["left", "right"],
        ))
        .expect("test function must be added");

    contract
}

// ---------------------------------------------------------------------------
// ABI identity and open-world extensibility
// ---------------------------------------------------------------------------

#[test]
fn abi_identifiers_reject_empty_and_nul_values() {
    assert!(AbiIdentifier::new("").is_err());
    assert!(AbiIdentifier::new("   ").is_err());
    assert!(AbiIdentifier::new("foreign\0symbol").is_err());
}

#[test]
fn abi_identifiers_preserve_open_world_names() {
    let identifier = AbiIdentifier::new(
        "future.vendor.architecture.calling-convention.v9",
    )
    .expect("future ABI identifiers must be representable");

    assert_eq!(
        identifier.as_str(),
        "future.vendor.architecture.calling-convention.v9"
    );
}

#[test]
fn calling_conventions_are_not_limited_to_a_closed_enum() {
    let profile = AbiProfile::new(
        "future-profile",
        "future.vendor.vector-call.v3",
        AbiLinkage::Custom(
            AbiIdentifier::new("future.vendor.linker")
                .expect("custom linkage identity must be valid"),
        ),
    )
    .expect("future calling conventions must be representable");

    assert_eq!(
        profile.calling_convention(),
        "future.vendor.vector-call.v3"
    );
}

#[test]
fn abi_contract_accepts_valid_function_declarations() {
    let contract = abi_contract();

    assert!(contract.validate().is_ok());
    assert!(contract.validate_classical().is_ok());
    assert_eq!(contract.functions().len(), 1);
}

#[test]
fn abi_function_parameter_order_is_preserved() {
    let contract = abi_contract();
    let function = &contract.functions()[0];

    assert_eq!(function.parameters().len(), 2);
    assert_eq!(function.parameters()[0].name(), "left");
    assert_eq!(function.parameters()[1].name(), "right");
}

#[test]
fn duplicate_parameter_names_are_rejected() {
    let value_type = AbiTypeRef::new("zamani.test.integer")
        .expect("test type must be valid");

    let first = AbiParameter::new(
        "value",
        value_type.clone(),
    )
    .expect("first parameter must be constructible");

    let second = AbiParameter::new(
        "value",
        value_type,
    )
    .expect("second parameter must be constructible");

    let result = AbiFunction::new(
        "duplicate_parameter",
        "zamani_test_duplicate_parameter",
        vec![first, second],
        None,
    );

    assert!(matches!(
        result,
        Err(error)
            if error.kind() == AbiErrorKind::DuplicateDeclaration
    ));
}

#[test]
fn duplicate_external_symbols_are_rejected() {
    let mut contract = abi_contract();

    let duplicate = AbiFunction::new(
        "another_function",
        "zamani_test_add",
        Vec::new(),
        None,
    )
    .expect("the individual declaration is structurally valid");

    let result = contract.add_function(duplicate);

    assert!(matches!(
        result,
        Err(error)
            if error.kind() == AbiErrorKind::DuplicateDeclaration
    ));

    assert_eq!(contract.functions().len(), 1);
    assert!(contract.validate().is_ok());
}

// ---------------------------------------------------------------------------
// ABI compatibility declarations
// ---------------------------------------------------------------------------

#[test]
fn abi_version_ranges_are_validated_explicitly() {
    let compatibility = AbiCompatibility::new(
        AbiVersion::new("2").expect("version must be valid"),
    )
    .with_minimum(
        AbiVersion::new("3").expect("version must be valid"),
    )
    .with_maximum(
        AbiVersion::new("1").expect("version must be valid"),
    );

    assert!(matches!(
        compatibility.validate_lexicographic(),
        Err(error) if error.kind() == AbiErrorKind::InvalidVersion
    ));
}

#[test]
fn abi_property_duplicates_do_not_silently_overwrite() {
    let mut contract = abi_contract();

    contract
        .insert_property("provider", "first")
        .expect("first property must be accepted");

    assert!(contract
        .insert_property("provider", "second")
        .is_err());

    let key = AbiIdentifier::new("provider")
        .expect("property key must be valid");

    assert_eq!(
        contract.properties().get(&key).map(String::as_str),
        Some("first")
    );
}

// ---------------------------------------------------------------------------
// Foreign-function declaration resolution
// ---------------------------------------------------------------------------

#[test]
fn foreign_call_resolution_finds_a_declared_function() {
    let contract = abi_contract();

    let declaration = ForeignFunctionDeclaration::resolve(
        &contract,
        "add",
    )
    .expect("declared function must resolve");

    assert_eq!(declaration.function().name(), "add");
    assert_eq!(
        declaration.function().symbol(),
        "zamani_test_add"
    );
    assert_eq!(
        declaration.contract().name(),
        "zamani.test.math"
    );
}

#[test]
fn foreign_call_resolution_rejects_unknown_functions() {
    let contract = abi_contract();

    let result = ForeignFunctionDeclaration::resolve(
        &contract,
        "function_that_does_not_exist",
    );

    assert!(matches!(
        result,
        Err(ForeignCallError::FunctionNotFound { .. })
    ));
}

#[test]
fn foreign_call_resolution_does_not_execute_the_function() {
    // Resolving a declaration validates metadata and produces a binding.
    // It must not invoke the external symbol.
    let contract = abi_contract();

    let declaration = ForeignFunctionDeclaration::resolve(
        &contract,
        "add",
    )
    .expect("declaration must resolve");

    assert_eq!(declaration.function().name(), "add");
    assert!(contract.validate().is_ok());
}

// ---------------------------------------------------------------------------
// Data-exchange contract validation
// ---------------------------------------------------------------------------

#[test]
fn data_exchange_identifiers_reject_empty_and_nul_values() {
    let empty = DataExchangeIdentifier::new("  ")
        .expect_err("blank identifiers must be rejected");

    assert_eq!(
        empty.kind(),
        DataExchangeErrorKind::InvalidIdentifier
    );

    let nul = DataExchangeIdentifier::new("format\0name")
        .expect_err("NUL identifiers must be rejected");

    assert_eq!(
        nul.kind(),
        DataExchangeErrorKind::InvalidIdentifier
    );
}

#[test]
fn data_exchange_contract_defaults_to_rejecting_data_loss() {
    assert_eq!(
        exchange_contract().data_loss_policy(),
        DataLossPolicy::RejectLoss
    );
}

#[test]
fn data_exchange_preserves_transfer_and_ordering_intent() {
    let contract = exchange_contract()
        .with_transfer_mode(TransferMode::Streaming)
        .with_ordering(OrderingGuarantee::PreserveSourceOrder);

    assert_eq!(
        contract.transfer_mode(),
        TransferMode::Streaming
    );
    assert_eq!(
        contract.ordering(),
        OrderingGuarantee::PreserveSourceOrder
    );
    assert!(contract.validate().is_ok());
}

#[test]
fn data_exchange_extension_metadata_is_replaceable_and_deterministic() {
    let contract = exchange_contract()
        .with_extension("vendor.property", "first")
        .expect("first extension must be accepted")
        .with_extension("vendor.property", "second")
        .expect("replacement must be accepted");

    assert_eq!(
        contract.extension("vendor.property"),
        Some("second")
    );
    assert_eq!(contract.extensions().len(), 1);
}

#[test]
fn data_exchange_rejects_nul_in_extension_values() {
    let result = exchange_contract()
        .with_extension("vendor.property", "invalid\0value");

    assert!(matches!(
        result,
        Err(error)
            if error.kind() == DataExchangeErrorKind::InvalidMetadata
    ));
}

#[test]
fn data_exchange_integrity_is_declarative() {
    let algorithm = DataExchangeIdentifier::new(
        "future.vendor.integrity-algorithm.v2",
    )
    .expect("integrity algorithm identity must be valid");

    let contract = exchange_contract()
        .with_integrity(IntegrityRequirement::Digest(algorithm));

    assert!(contract.validate().is_ok());

    // This validates the declared integrity contract. It does not compute
    // a digest, authenticate a peer, or perform a transfer.
}

// ---------------------------------------------------------------------------
// Explicit compatibility evidence and policy
// ---------------------------------------------------------------------------

fn evidence(
    fact: CompatibilityFact,
) -> CompatibilityEvidence {
    CompatibilityEvidence::new(fact, fact, fact, fact)
}

#[test]
fn equivalent_evidence_is_compatible_under_exact_policy() {
    let report = CompatibilityChecker::assess(
        evidence(CompatibilityFact::Equivalent),
        CompatibilityPolicy::Exact,
    );

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.is_exactly_equivalent());
    assert!(report.ensure_compatible().is_ok());
}

#[test]
fn declared_compatibility_is_not_exact_equivalence() {
    let report = CompatibilityChecker::assess(
        evidence(CompatibilityFact::Compatible),
        CompatibilityPolicy::AllowDeclaredCompatible,
    );

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(!report.is_exactly_equivalent());
}

#[test]
fn exact_policy_rejects_merely_declared_compatibility() {
    let report = CompatibilityChecker::assess(
        evidence(CompatibilityFact::Compatible),
        CompatibilityPolicy::Exact,
    );

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.ensure_compatible().is_err());
}

#[test]
fn migration_requires_explicit_policy_permission() {
    let facts = CompatibilityEvidence::new(
        CompatibilityFact::Equivalent,
        CompatibilityFact::Equivalent,
        CompatibilityFact::Equivalent,
        CompatibilityFact::RequiresMigration,
    );

    let rejected = CompatibilityChecker::assess(
        facts.clone(),
        CompatibilityPolicy::AllowDeclaredCompatible,
    );

    let permitted = CompatibilityChecker::assess(
        facts,
        CompatibilityPolicy::AllowMigration,
    );

    assert_eq!(
        rejected.status(),
        CompatibilityStatus::Incompatible
    );
    assert_eq!(
        permitted.status(),
        CompatibilityStatus::RequiresMigration
    );
    assert_eq!(
        permitted.migration_dimensions(),
        vec![CompatibilityDimension::Version]
    );
}

#[test]
fn unknown_compatibility_evidence_is_not_silently_accepted() {
    let report = CompatibilityChecker::assess(
        evidence(CompatibilityFact::Unknown),
        CompatibilityPolicy::AllowMigration,
    );

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate
    );
    assert!(report.ensure_compatible().is_err());
}

#[test]
fn explicit_incompatibility_takes_precedence_over_unknown_evidence() {
    let facts = CompatibilityEvidence::new(
        CompatibilityFact::Unknown,
        CompatibilityFact::Incompatible,
        CompatibilityFact::Equivalent,
        CompatibilityFact::Equivalent,
    );

    let report = CompatibilityChecker::assess(
        facts,
        CompatibilityPolicy::AllowMigration,
    );

    assert_eq!(
        report.status(),
        CompatibilityStatus::Incompatible
    );
}

#[test]
fn absent_versions_do_not_imply_version_equivalence() {
    let source = endpoint(
        "schema.customer",
        "format.example",
        "encoding.example",
        None,
    );

    let destination = endpoint(
        "schema.customer",
        "format.example",
        "encoding.example",
        None,
    );

    let report = CompatibilityChecker::assess_endpoints(
        &source,
        &destination,
        CompatibilityPolicy::Exact,
    );

    assert_eq!(
        report.schema().fact(),
        CompatibilityFact::Equivalent
    );
    assert_eq!(
        report.format().fact(),
        CompatibilityFact::Equivalent
    );
    assert_eq!(
        report.encoding().fact(),
        CompatibilityFact::Equivalent
    );
    assert_eq!(
        report.version().fact(),
        CompatibilityFact::Unknown
    );
    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate
    );
}

#[test]
fn version_strings_are_not_compared_as_numeric_versions() {
    let source = endpoint(
        "schema.customer",
        "format.example",
        "encoding.example",
        Some("2"),
    );

    let destination = endpoint(
        "schema.customer",
        "format.example",
        "encoding.example",
        Some("10"),
    );

    let evidence =
        CompatibilityEvidence::from_endpoint_identity(
            &source,
            &destination,
        );

    assert_eq!(
        evidence.version(),
        CompatibilityFact::Unknown
    );
}

#[test]
fn compatibility_decision_order_is_stable() {
    let report = CompatibilityChecker::assess(
        evidence(CompatibilityFact::Equivalent),
        CompatibilityPolicy::Exact,
    );

    let dimensions = report
        .decisions()
        .into_iter()
        .map(|decision| decision.dimension())
        .collect::<Vec<_>>();

    assert_eq!(
        dimensions,
        vec![
            CompatibilityDimension::Schema,
            CompatibilityDimension::Format,
            CompatibilityDimension::Encoding,
            CompatibilityDimension::Version,
        ]
    );
}

// ---------------------------------------------------------------------------
// Configurable scaling and deterministic contract construction
// ---------------------------------------------------------------------------

#[test]
fn abi_contracts_support_generated_function_workloads() {
    let mut contract = abi_contract();

    // This is a regression-test workload, not a semantic maximum.
    let workload_size = 256_usize;

    for index in 0..workload_size {
        let name = format!("generated_function_{index}");
        let symbol = format!("generated_symbol_{index}");

        contract
            .add_function(AbiFunction::new(
                name,
                symbol,
                Vec::new(),
                None,
            )
            .expect("generated function must be valid"))
            .expect("generated function name and symbol must be unique");
    }

    assert_eq!(
        contract.functions().len(),
        workload_size + 1
    );
    assert!(contract.validate().is_ok());
}

#[test]
fn data_exchange_supports_generated_extension_metadata() {
    let mut contract = exchange_contract();
    let workload_size = 256_usize;

    for index in 0..workload_size {
        let key = format!("vendor.property.{index}");
        let value = format!("value.{index}");

        contract = contract
            .with_extension(key, value)
            .expect("generated extension must be valid");
    }

    assert_eq!(contract.extensions().len(), workload_size);
    assert!(contract.validate().is_ok());
}

#[test]
fn generated_exchange_contracts_preserve_open_world_identifiers() {
    let workload_size = 128_usize;

    for index in 0..workload_size {
        let schema = format!("future.schema.{index}");
        let format = format!("future.format.{index}");
        let encoding = format!("future.encoding.{index}");

        let contract = DataExchangeContract::new(
            format!("future.exchange.{index}"),
            endpoint(&schema, &format, &encoding, Some("v1")),
            endpoint(&schema, &format, &encoding, Some("v1")),
        )
        .expect("future identifiers must not require enum changes");

        assert!(contract.validate().is_ok());
        assert_eq!(
            contract.destination().format().as_str(),
            format
        );
    }
}

#[test]
fn repeated_compatibility_assessment_is_deterministic() {
    let facts = evidence(CompatibilityFact::Equivalent);

    let first = CompatibilityChecker::assess(
        facts.clone(),
        CompatibilityPolicy::Exact,
    );

    let second = CompatibilityChecker::assess(
        facts,
        CompatibilityPolicy::Exact,
    );

    assert_eq!(first.status(), second.status());
    assert_eq!(
        first.is_exactly_equivalent(),
        second.is_exactly_equivalent()
    );

    let first_dimensions = first
        .decisions()
        .into_iter()
        .map(|decision| decision.dimension())
        .collect::<Vec<_>>();

    let second_dimensions = second
        .decisions()
        .into_iter()
        .map(|decision| decision.dimension())
        .collect::<Vec<_>>();

    assert_eq!(first_dimensions, second_dimensions);
}
