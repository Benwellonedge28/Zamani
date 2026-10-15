
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Builder Integration Tests
//!
//! Path: `src/zuir/tests/builders.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: unsafe Rust is prohibited.
//!
//! PURPOSE
//!
//! Test the public builder contracts used to construct ZUIR programs,
//! regions, and operations, together with the reusable builder-validation
//! framework.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/builder/operation.rs`
//! - `src/zuir/builder/program.rs`
//! - `src/zuir/builder/regions.rs`
//! - `src/zuir/builder/validation.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/operation.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/schema/`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/grammar.md`
//!
//! OWNS
//!
//! - Integration tests for the canonical builder APIs.
//! - Regression checks for ownership, validation, deterministic traversal,
//!   extensibility, and resource-aware construction.
//! - Assertions about the boundary between local construction and
//!   whole-program verification.
//!
//! DOES_NOT_OWN
//!
//! - Production builder implementation.
//! - A second IR representation or independent operation vocabulary.
//! - Source parsing, AST construction, or semantic analysis.
//! - Whole-program verification, optimization, lowering, or execution.
//! - Physical hardware allocation or target feasibility.
//!
//! INTEGRATION CONTRACT
//!
//! Tests use the canonical public builder and IR types. They do not inspect
//! private fields or assume that local builder validation proves global
//! semantic correctness.
//!
//! SCALABILITY AND POCO-REAF
//!
//! No universal hardware capacities or operation-count limits are introduced.
//! Tests exercise dynamically sized collections and large representable IDs.
//! Actual construction remains subject to available resources and explicit
//! policies; literal infinite storage or execution is not promised.
//!
//! FREEZE CRITERIA
//!
//! - This module is declared by the repository's test-module tree.
//! - Every imported public API exists and is accessible.
//! - Tests pass against the actual repository implementation.
//! - Formatting, compilation, and Clippy checks pass.
//! - No unsafe code or target-specific assumptions are introduced.

use crate::zuir::builder::operation::OperationBuilder;
use crate::zuir::builder::program::ProgramBuilder;
use crate::zuir::builder::regions::RegionBuilder;
use crate::zuir::builder::validation::{
    ValidationContext,
    ValidationIssue,
    ValidationMode,
    ValidationPhase,
    ValidationPipeline,
    ValidationRule,
};
use crate::zuir::dialect::definition::{
    DialectExportName,
    DialectName,
};
use crate::zuir::dialect::operation::{
    EffectDeclaration,
    OperationDefinition,
};
use crate::zuir::ids::{
    BlockId,
    FunctionId,
    ModuleId,
    OperationId,
    ProgramId,
    RegionId,
    ValueId,
};
use crate::zuir::ir::block::ZuirBlock;
use crate::zuir::ir::region::{
    CaptureMode,
    RegionIsolation,
    RegionKind,
    RegionOwner,
};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::version::DialectVersion;

// =============================================================================
// Helpers
// =============================================================================

fn program_id(value: u128) -> ProgramId {
    ProgramId::new(value).expect("test program ID must be nonzero")
}

fn module_id(value: u128) -> ModuleId {
    ModuleId::new(value).expect("test module ID must be nonzero")
}

fn function_id(value: u128) -> FunctionId {
    FunctionId::new(value).expect("test function ID must be nonzero")
}

fn region_id(value: u128) -> RegionId {
    RegionId::new(value).expect("test region ID must be nonzero")
}

fn block_id(value: u128) -> BlockId {
    BlockId::new(value).expect("test block ID must be nonzero")
}

fn operation_id(value: u128) -> OperationId {
    OperationId::new(value).expect("test operation ID must be nonzero")
}

fn value_id(value: u128) -> ValueId {
    ValueId::new(value).expect("test value ID must be nonzero")
}

fn test_program_builder() -> ProgramBuilder<String> {
    ProgramBuilder::with_id(
        program_id(1),
        crate::zuir::version::CURRENT_ZUIR_SCHEMA_VERSION,
        crate::zuir::version::CURRENT_ZUIR_FORMAT_VERSION,
    )
}

fn test_region_builder() -> RegionBuilder {
    RegionBuilder::new(
        region_id(1),
        RegionOwner::Function(function_id(2)),
        RegionKind::Structured,
        RegionIsolation::CapturesAllowed,
    )
}

fn test_operation_schema() -> OperationDefinition {
    let mut schema = OperationDefinition::new(
        DialectName::parse("test").expect("valid dialect name"),
        DialectExportName::parse("calculate")
            .expect("valid operation export name"),
        DialectVersion::new(1, 0, 0),
    )
    .expect("operation schema should be constructible");

    // An explicitly known empty effect set is not the same as an unknown
    // effect contract. The builder must not silently assume purity.
    schema
        .set_effects(
            EffectDeclaration::known(
                std::iter::empty::<QualifiedName>(),
            )
            .expect("empty effect set should be valid"),
        )
        .expect("known effect contract should be accepted");

    schema
}

// =============================================================================
// Program construction
// =============================================================================

#[test]
fn program_builder_creates_a_valid_empty_program() {
    let builder = ProgramBuilder::<String>::new()
        .expect("program identity allocation should succeed");

    assert!(builder.is_empty());
    assert_eq!(builder.module_count(), 0);
    assert!(builder.validate().is_ok());

    let program = builder.build().expect("empty program should build");

    assert!(program.is_empty());
    assert!(program.validate().is_ok());
}

#[test]
fn program_builder_preserves_inserted_modules() {
    let mut builder = test_program_builder();
    let first = module_id(10);
    let second = module_id(20);

    builder
        .add_module(first, "classical".to_owned())
        .expect("first module should be inserted");

    builder
        .add_module(second, "quantum".to_owned())
        .expect("second module should be inserted");

    assert_eq!(builder.module_count(), 2);
    assert_eq!(builder.module(first).map(String::as_str), Some("classical"));
    assert_eq!(builder.module(second).map(String::as_str), Some("quantum"));

    let program = builder.build().expect("program should build");

    assert_eq!(program.module_count(), 2);
    assert!(program.contains_module(first));
    assert!(program.contains_module(second));
}

#[test]
fn duplicate_module_insertion_preserves_the_original_and_returns_the_new_value() {
    let mut builder = test_program_builder();
    let id = module_id(10);

    builder
        .add_module(id, "original".to_owned())
        .expect("initial insertion should succeed");

    let (_, rejected) = builder
        .add_module(id, "replacement".to_owned())
        .expect_err("duplicate module IDs must be rejected");

    assert_eq!(rejected, "replacement");
    assert_eq!(builder.module(id).map(String::as_str), Some("original"));
    assert_eq!(builder.module_count(), 1);
}

#[test]
fn program_module_iteration_is_deterministic() {
    let mut builder = test_program_builder();

    for (id, name) in [
        (module_id(30), "third"),
        (module_id(10), "first"),
        (module_id(20), "second"),
    ] {
        builder
            .add_module(id, name.to_owned())
            .expect("unique module should be inserted");
    }

    let ids: Vec<_> = builder.modules().map(|(id, _)| id).collect();

    assert_eq!(
        ids,
        vec![module_id(10), module_id(20), module_id(30)]
    );
}

#[test]
fn program_builder_validates_dialect_requirements() {
    let mut builder = test_program_builder();
    let version = DialectVersion::new(1, 0, 0);

    builder
        .require_dialect_named("quantum", version)
        .expect("valid dialect requirement should be accepted");

    assert!(
        builder
            .require_dialect_named("quantum", version)
            .is_err(),
        "duplicate requirements must not be silently inserted"
    );

    assert!(
        builder
            .require_dialect_named("quantum/physical", version)
            .is_err(),
        "invalid dialect names must be rejected"
    );

    assert_eq!(builder.dialect_requirements().count(), 1);
    assert!(builder.validate().is_ok());
}

#[test]
fn descriptive_metadata_is_validated_and_retrievable() {
    let mut builder = test_program_builder();

    builder
        .set_metadata("build.origin", "builder-integration-test")
        .expect("valid metadata should be accepted");

    assert_eq!(
        builder.metadata("build.origin"),
        Some("builder-integration-test")
    );

    assert!(
        builder.set_metadata(" invalid ", "value").is_err(),
        "malformed metadata keys must be rejected"
    );

    assert_eq!(
        builder.metadata("build.origin"),
        Some("builder-integration-test"),
        "failed metadata updates must not erase valid metadata"
    );
}

#[test]
fn program_builder_accepts_large_nonzero_ids_without_hardware_assumptions() {
    let mut builder = test_program_builder();
    let id = module_id(u128::MAX);

    builder
        .add_module(id, "logical-module".to_owned())
        .expect("the largest nonzero representable ID should be accepted");

    assert!(builder.contains_module(id));
    assert_eq!(builder.module_count(), 1);
    assert!(builder.validate().is_ok());
}

#[test]
fn program_builder_can_rebuild_a_valid_existing_program() {
    let mut builder = test_program_builder();

    builder
        .add_module(module_id(10), "module".to_owned())
        .expect("module insertion should succeed");

    let original = builder.build().expect("initial build should succeed");

    let rebuilt = ProgramBuilder::from_program(original.clone())
        .expect("valid program should be accepted")
        .build()
        .expect("rebuilt program should remain valid");

    assert_eq!(rebuilt, original);
}

// =============================================================================
// Region construction and ownership
// =============================================================================

#[test]
fn region_builder_adds_a_block_and_establishes_parent_membership() {
    let mut builder = test_region_builder();
    let mut block = ZuirBlock::new(block_id(3));

    builder
        .add_block(&mut block)
        .expect("block should be added");

    assert_eq!(block.parent_region(), Some(region_id(1)));
    assert_eq!(builder.region().blocks(), &[block_id(3)]);
    assert!(builder.validate().is_ok());

    let region = builder.finish().expect("valid region should finish");

    assert_eq!(region.id(), region_id(1));
    assert_eq!(region.blocks(), &[block_id(3)]);
}

#[test]
fn region_builder_rejects_a_block_owned_by_another_region_atomically() {
    let mut builder = test_region_builder();
    let mut block = ZuirBlock::in_region(block_id(3), region_id(99));

    assert!(
        builder.add_block(&mut block).is_err(),
        "a block cannot be silently moved from another region"
    );

    assert_eq!(block.parent_region(), Some(region_id(99)));
    assert_eq!(builder.block_count(), 0);
}

#[test]
fn region_builder_rejects_duplicate_block_membership() {
    let mut builder = test_region_builder();
    let mut block = ZuirBlock::new(block_id(3));

    builder
        .add_block(&mut block)
        .expect("first insertion should succeed");

    assert!(
        builder.add_block(&mut block).is_err(),
        "a region must not contain the same block twice"
    );

    assert_eq!(builder.block_count(), 1);
}

#[test]
fn region_builder_rejects_an_empty_region_at_finalization() {
    assert!(
        test_region_builder().finish().is_err(),
        "an incomplete empty region must not be finalized"
    );
}

#[test]
fn isolated_regions_reject_captures() {
    let mut builder = RegionBuilder::new(
        region_id(5),
        RegionOwner::Function(function_id(6)),
        RegionKind::Structured,
        RegionIsolation::Isolated,
    );

    assert!(
        builder.add_capture(value_id(7), CaptureMode::Read).is_err(),
        "isolated regions must not accept enclosing-scope captures"
    );

    assert_eq!(builder.block_count(), 0);
    assert!(builder.validate().is_ok());
}

#[test]
fn region_reservation_does_not_change_logical_contents() {
    let mut builder = test_region_builder();

    builder
        .try_reserve_blocks(16)
        .expect("reservation should succeed under ordinary test resources");

    assert_eq!(builder.block_count(), 0);

    let mut block = ZuirBlock::new(block_id(3));

    builder
        .add_block(&mut block)
        .expect("reservation must not prevent insertion");

    assert_eq!(builder.block_count(), 1);
}

// =============================================================================
// Operation construction from extensible schemas
// =============================================================================

#[test]
fn operation_builder_constructs_an_operation_from_a_registered_schema_contract() {
    let schema = test_operation_schema();

    let operation = OperationBuilder::new(operation_id(1), &schema)
        .expect("valid schema should initialize the builder")
        .build()
        .expect("operation should satisfy its empty-arity schema");

    assert_eq!(operation.id(), operation_id(1));
    assert_eq!(operation.name().canonical_string(), "test::calculate");
    assert!(operation.operands().is_empty());
    assert!(operation.results().is_empty());
    assert!(operation.regions().is_empty());
    assert!(operation.validate().is_ok());
}

#[test]
fn operation_builder_does_not_assume_unknown_effects_are_pure() {
    let schema = OperationDefinition::new(
        DialectName::parse("test").expect("valid dialect"),
        DialectExportName::parse("unknown_effect")
            .expect("valid operation name"),
        DialectVersion::new(1, 0, 0),
    )
    .expect("schema should be constructible");

    let result = OperationBuilder::new(operation_id(2), &schema)
        .expect("schema construction itself should succeed")
        .build();

    assert!(
        result.is_err(),
        "an operation with unknown effects must not be silently treated as pure"
    );
}

// =============================================================================
// Extensible validation pipeline
// =============================================================================

struct MinimumValueRule {
    minimum: u64,
}

impl ValidationRule<u64> for MinimumValueRule {
    fn name(&self) -> &str {
        "tests.builders.minimum_value"
    }

    fn validate(
        &self,
        input: &u64,
        _context: &ValidationContext,
    ) -> Result<Vec<ValidationIssue>, String> {
        if *input >= self.minimum {
            return Ok(Vec::new());
        }

        Ok(vec![
            ValidationIssue::error(
                "ZUIR.TEST.VALUE_BELOW_MINIMUM",
                format!("value {input} is below {}", self.minimum),
            )
            .with_path(["input"]),
        ])
    }
}

struct WarningRule;

impl ValidationRule<u64> for WarningRule {
    fn name(&self) -> &str {
        "tests.builders.warning"
    }

    fn validate(
        &self,
        _input: &u64,
        _context: &ValidationContext,
    ) -> Result<Vec<ValidationIssue>, String> {
        Ok(vec![ValidationIssue::warning(
            "ZUIR.TEST.NON_FATAL_WARNING",
            "synthetic non-fatal warning",
        )])
    }
}

fn validation_context(mode: ValidationMode) -> ValidationContext {
    ValidationContext::new(ValidationPhase::BuilderLocal, mode)
}

#[test]
fn validation_pipeline_accepts_valid_input() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("rule registration should succeed");

    let report = pipeline
        .validate(&10, &validation_context(ValidationMode::CollectAll))
        .expect("validation should complete");

    assert!(!report.has_errors());
    assert_eq!(report.error_count(), 0);
    assert!(report.is_empty());
}

#[test]
fn validation_pipeline_preserves_rule_provenance() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("rule registration should succeed");

    let report = pipeline
        .validate(&3, &validation_context(ValidationMode::CollectAll))
        .expect("validation should produce a report");

    assert!(report.has_errors());
    assert_eq!(report.error_count(), 1);
    assert_eq!(
        report.issues()[0].rule(),
        Some("tests.builders.minimum_value")
    );
    assert_eq!(
        report.issues()[0].path(),
        &["input".to_owned()]
    );
}

#[test]
fn collect_all_mode_keeps_later_diagnostics_after_an_error() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("first rule registration should succeed");

    pipeline
        .try_add_rule(WarningRule)
        .expect("second rule registration should succeed");

    let report = pipeline
        .validate(&3, &validation_context(ValidationMode::CollectAll))
        .expect("pipeline should complete");

    assert_eq!(report.error_count(), 1);
    assert_eq!(report.warning_count(), 1);
    assert_eq!(report.issue_count(), 2);
}

#[test]
fn fail_fast_mode_stops_after_an_error() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("first rule registration should succeed");

    pipeline
        .try_add_rule(WarningRule)
        .expect("second rule registration should succeed");

    let report = pipeline
        .validate(&3, &validation_context(ValidationMode::FailFast))
        .expect("fail-fast validation should return its partial report");

    assert_eq!(report.error_count(), 1);
    assert_eq!(report.warning_count(), 0);
}

#[test]
fn validation_pipeline_rejects_duplicate_rule_names() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("first registration should succeed");

    assert!(
        pipeline
            .try_add_rule(MinimumValueRule { minimum: 20 })
            .is_err(),
        "duplicate rule names would make diagnostic provenance ambiguous"
    );

    assert_eq!(pipeline.rule_count(), 1);
    assert_eq!(
        pipeline.rule_names(),
        vec!["tests.builders.minimum_value"]
    );
}

#[test]
fn validation_pipeline_runs_rules_in_registration_order() {
    let mut pipeline = ValidationPipeline::new();

    pipeline
        .try_add_rule(MinimumValueRule { minimum: 10 })
        .expect("first registration should succeed");

    pipeline
        .try_add_rule(WarningRule)
        .expect("second registration should succeed");

    assert_eq!(
        pipeline.rule_names(),
        vec![
            "tests.builders.minimum_value",
            "tests.builders.warning",
        ]
    );
}

// =============================================================================
// Thread-safety and resource-aware behavior
// =============================================================================

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn canonical_builder_types_satisfy_their_thread_safety_contracts() {
    assert_send_sync::<ProgramBuilder<String>>();
    assert_send_sync::<RegionBuilder>();
    assert_send_sync::<ValidationPipeline<u64>>();
}

#[test]
fn dynamic_program_assembly_has_no_encoded_hardware_capacity() {
    let mut builder = test_program_builder();

    // This is a finite regression test, not a claimed maximum. It checks
    // that assembly uses the dynamic collection contract rather than a
    // machine-specific device, processor, or module capacity.
    let count: u128 = 128;

    for value in 1..=count {
        builder
            .add_module(
                module_id(value),
                format!("logical-module-{value}"),
            )
            .expect("unique logical module should be inserted");
    }

    assert_eq!(builder.module_count(), count as usize);
    assert!(builder.validate().is_ok());
}
