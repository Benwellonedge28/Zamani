#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Scalability Tests
//!
//! Path: `src/hdl/hdlir/tests/scalability.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Exercise the scalability-related contracts exposed by the current HDLIR
//! implementation:
//!
//! - Progressively larger logical design registries.
//! - Large, non-sequentially interpreted module identities.
//! - Symbolic and deeply nested width expressions.
//! - Explicit width-evaluation work budgets.
//! - Checked width arithmetic.
//! - Explicit HDLIR operational budgets.
//! - Increasing artifact payload sizes.
//! - Encoding and decoding failures when configured byte budgets are exceeded.
//!
//! ## Architectural authority
//!
//! These tests follow:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/types/bit_width.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//! - `src/hdl/hdlir/serialization/encode.rs`
//! - `src/hdl/hdlir/serialization/decode.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar and frontend own source syntax and source-level semantics.
//! ZUIR owns the universal computational representation. HDLIR owns its
//! hardware-domain representation, symbolic-width operations, and
//! hardware-specific integration.
//!
//! ## Scope and guarantees
//!
//! These are regression tests for the existing APIs, not a claim that every
//! specified HDLIR subsystem is implemented.
//!
//! The current design registry can be tested directly. The tests do not
//! pretend that registering empty modules is equivalent to constructing a
//! complete gate-level graph, validating behavioral semantics, synthesizing
//! a circuit, or proving physical target feasibility.
//!
//! The payload tests exercise the artifact envelope only. The payload bytes
//! are not claimed to be a canonical serialization of an HDLIR graph.
//!
//! The size values used below are bounded test workloads, not maximum legal
//! design sizes, language limits, or recommended production budgets.
//!
//! ## Scalability rules
//!
//! - No universal hardware-count ceiling is introduced.
//! - Symbolic widths remain symbolic until explicitly evaluated.
//! - Evaluation work and serialization sizes can be bounded by caller policy.
//! - Budget exhaustion is an explicit failure, never success.
//! - Arithmetic overflow is reported rather than wrapped.
//! - IDs identify entities; they are not assumed to be collection indexes.
//! - Tests use deterministic inputs and safe Rust only.
//!
//! Finite software remains subject to representable values, available memory,
//! allocator behavior, execution time, and explicitly configured budgets.
//! These tests do not claim literal infinite processing capacity.
//!
//! ## Integration
//!
//! Register this module from `src/hdl/hdlir/tests/mod.rs`, then register the
//! test composition module from a valid Rust HDLIR module root.
//!
//! The current repository's `src/hdl/hdlir/mod.rs` must first be restored as
//! Rust source. Do not use a fabricated `Placeholder` type to work around it.
//!
//! ## Completion criteria
//!
//! - All test workloads construct using existing public HDLIR APIs.
//! - Every successfully built design passes the design-level validation
//!   required by the test.
//! - Symbolic width evaluation respects exact work budgets.
//! - Width overflow remains explicit.
//! - Serialization round trips preserve the exact supplied payload.
//! - Explicit budget exhaustion is rejected by the owning API.
//! - The tests do not introduce unsafe code or universal hardware ceilings.
//! - Formatting, compilation, tests, and Clippy pass in the actual checkout.

use crate::hdl::hdlir::config::{
    BudgetKind,
    HdlirConfig,
    WorkBudget,
};
use crate::hdl::hdlir::ids::{
    HdlirDesignId,
    HdlirModuleId,
};
use crate::hdl::hdlir::ir::design::HdlirDesign;
use crate::hdl::hdlir::ir::module::HdlirModule;
use crate::hdl::hdlir::serialization::decode::{
    decode_artifact,
    DecodeOptions,
};
use crate::hdl::hdlir::serialization::encode::{
    encode_artifact,
    EncodeOptions,
};
use crate::hdl::hdlir::serialization::format::ArtifactFormat;
use crate::hdl::hdlir::types::bit_width::{
    BitWidth,
    WidthEvaluationBudget,
    WidthEvaluationError,
};

/// Construct a design containing the requested number of empty logical
/// modules.
///
/// This helper tests the actual design registry and its structural invariants.
/// It does not represent empty modules as implemented hardware behavior.
///
/// Module IDs and names are deterministic for the supplied count. The count
/// is a test workload, not a maximum supported module count.
fn design_with_module_count(
    module_count: usize,
) -> HdlirDesign<HdlirModule> {
    assert!(
        module_count > 0,
        "this helper builds a complete design with an entry module"
    );

    let design_id = HdlirDesignId::new(1)
        .expect("the test design identity must be non-zero");

    let mut design = HdlirDesign::new(
        design_id,
        format!("scalability.design.{module_count}"),
    )
    .expect("the design name must be valid");

    let entry_module_id = HdlirModuleId::new(1)
        .expect("the entry module identity must be non-zero");

    for index in 0..module_count {
        let raw_id = u128::try_from(index)
            .expect("test module index must fit the HDLIR ID representation")
            .checked_add(1)
            .expect("test module ID arithmetic must not overflow");

        let module_id = HdlirModuleId::new(raw_id)
            .expect("generated module identity must be non-zero");

        let module = HdlirModule::new(
            module_id,
            format!("module.{index}"),
        )
        .expect("generated module name must be valid");

        design
            .insert_module(module)
            .expect("each generated module ID and name must be unique");
    }

    design
        .set_entry_module(entry_module_id)
        .expect("the entry module must have been registered");

    design
        .validate_complete()
        .expect("the generated design must satisfy design-level invariants");

    design
}

#[test]
fn progressively_larger_design_registries_remain_valid() {
    // These values provide several repeatable regression workloads. They are
    // not hardware limits, maximum supported sizes, or semantic restrictions.
    let workloads = [1_usize, 2, 16, 64, 256, 512];

    for module_count in workloads {
        let design = design_with_module_count(module_count);

        assert_eq!(
            design.module_count(),
            module_count,
            "the registry must retain every inserted module"
        );

        assert!(!design.is_empty());

        assert_eq!(
            design.entry_module_id(),
            HdlirModuleId::new(1),
            "the entry module must not depend on incidental iteration order"
        );

        assert!(
            design.module_by_name("module.0").is_some(),
            "the first module must remain discoverable"
        );

        let last_name = format!("module.{}", module_count - 1);

        assert!(
            design.module_by_name(&last_name).is_some(),
            "the final inserted module must remain discoverable"
        );

        assert_eq!(
            design.modules().count(),
            module_count,
            "deterministic iteration must cover the complete registry"
        );

        design
            .validate_complete()
            .expect("the complete design must remain structurally valid");
    }
}

#[test]
fn large_numeric_module_identity_is_not_a_collection_index() {
    // IDs are logical identities, not indexes into a dense array. The largest
    // representable non-zero ID is valid independently of the registry size.
    let module_id = HdlirModuleId::new(u128::MAX)
        .expect("the maximum non-zero u128 identity must be representable");

    let design_id = HdlirDesignId::new(1)
        .expect("the design identity must be non-zero");

    let mut design = HdlirDesign::new(
        design_id,
        "scalability.large_identity",
    )
    .expect("the design name must be valid");

    let module = HdlirModule::new(module_id, "sparse.identity")
        .expect("the module name must be valid");

    design
        .insert_module(module)
        .expect("a large identity must not require a correspondingly large array");

    design
        .set_entry_module(module_id)
        .expect("the registered module can be selected as the entry");

    design
        .validate_complete()
        .expect("a sparse, large identity must satisfy registry invariants");

    assert_eq!(design.module_count(), 1);
    assert!(
        design.module(module_id).is_some(),
        "lookup must use logical identity rather than dense indexing"
    );
}

#[test]
fn symbolic_width_stays_symbolic_until_evaluation() {
    let width = BitWidth::symbol("DATA_WIDTH")
        .expect("the symbolic width name must be valid");

    assert!(
        !width.is_fully_concrete(),
        "creating a symbolic width must not resolve it prematurely"
    );

    assert_eq!(
        width
            .evaluate_with(WidthEvaluationBudget::new(1), |name| {
                (name == "DATA_WIDTH").then_some(4096)
            })
            .expect("the resolver supplies a positive concrete width"),
        4096
    );

    assert!(
        !width.is_fully_concrete(),
        "evaluating through a resolver must not mutate the symbolic expression"
    );
}

#[test]
fn deep_symbolic_width_expression_evaluates_with_an_explicit_budget() {
    // A deep expression exercises iterative expression traversal. This
    // depth is a stress workload, not a language-level expression-depth cap.
    let depth = 512_usize;

    let node_budget = depth
        .checked_mul(2)
        .and_then(|nodes| nodes.checked_add(1))
        .expect("the test's node budget calculation must be checked");

    let mut expression = BitWidth::symbol("BASE_WIDTH")
        .expect("the base symbol must be valid");

    for _ in 0..depth {
        expression = BitWidth::add(
            expression,
            BitWidth::known(1).expect("one is a valid positive bit width"),
        );
    }

    assert!(
        !expression.is_fully_concrete(),
        "a symbolic descendant must keep the expression symbolic"
    );

    let evaluated = expression
        .evaluate_with(
            WidthEvaluationBudget::new(node_budget),
            |name| (name == "BASE_WIDTH").then_some(8),
        )
        .expect("the expression must evaluate within its exact node budget");

    let expected = u128::try_from(depth)
        .expect("the test depth must fit u128")
        .checked_add(8)
        .expect("the expected test width must not overflow");

    assert_eq!(evaluated, expected);
}

#[test]
fn symbolic_width_budget_exhaustion_is_not_reported_as_success() {
    let expression = BitWidth::add(
        BitWidth::symbol("BASE_WIDTH")
            .expect("the symbol must be valid"),
        BitWidth::known(1).expect("one is a valid width"),
    );

    let result = expression.evaluate_with(
        WidthEvaluationBudget::new(1),
        |name| (name == "BASE_WIDTH").then_some(8),
    );

    assert!(
        matches!(
            result,
            Err(WidthEvaluationError::BudgetExceeded { .. })
        ),
        "an expression requiring more work than allowed must fail explicitly"
    );
}

#[test]
fn symbolic_width_arithmetic_overflow_is_explicit() {
    let maximum = BitWidth::known(u128::MAX)
        .expect("u128::MAX is a non-zero representable width");

    let one = BitWidth::known(1)
        .expect("one is a valid positive width");

    let expression = BitWidth::add(maximum, one);

    let result =
        expression.evaluate_concrete(WidthEvaluationBudget::new(3));

    assert!(
        matches!(
            result,
            Err(WidthEvaluationError::ArithmeticOverflow { .. })
        ),
        "overflow must not wrap or return a false width"
    );
}

#[test]
fn default_configuration_does_not_impose_a_hidden_entity_budget() {
    let config = HdlirConfig::default();

    assert!(
        config.budget(&BudgetKind::IrEntities).is_unlimited(),
        "an unspecified entity budget must not introduce an implicit limit"
    );

    assert!(
        config
            .check_budget(&BudgetKind::IrEntities, u64::MAX)
            .is_ok(),
        "the default configuration must not invent a smaller entity ceiling"
    );
}

#[test]
fn explicit_entity_budget_accepts_the_limit_and_rejects_excess_work() {
    let budget = WorkBudget::bounded(128);

    let config = HdlirConfig::default()
        .with_budget(BudgetKind::IrEntities, budget)
        .expect("the built-in entity budget must be valid");

    assert!(
        config
            .check_budget(&BudgetKind::IrEntities, 128)
            .is_ok(),
        "work equal to the configured limit must be accepted"
    );

    assert!(
        config
            .check_budget(&BudgetKind::IrEntities, 129)
            .is_err(),
        "work above the configured limit must produce an explicit failure"
    );
}

#[test]
fn progressively_larger_payloads_round_trip_without_payload_loss() {
    // These are bounded CI workloads, not a maximum artifact size. Production
    // callers must select budgets appropriate to their own resource policies.
    let payload_sizes = [0_usize, 1, 32, 4096, 65_536];
    let format = ArtifactFormat::new();

    for payload_size in payload_sizes {
        let payload: Vec<u8> = (0..payload_size)
            .map(|index| {
                u8::try_from(index % 251)
                    .expect("the generated byte value must fit u8")
            })
            .collect();

        let encoded = encode_artifact(
            &format,
            &payload,
            EncodeOptions::default(),
        )
        .expect("the artifact must encode within the default policy");

        let decoded = decode_artifact(
            &encoded,
            &DecodeOptions::default(),
        )
        .expect("the encoded artifact must decode");

        assert_eq!(
            decoded.payload(),
            payload.as_slice(),
            "decoding must preserve every payload byte"
        );

        assert_eq!(
            decoded.format(),
            &format,
            "the artifact envelope must survive the round trip"
        );
    }
}

#[test]
fn encoder_rejects_payloads_that_exceed_the_callers_budget() {
    let format = ArtifactFormat::new();
    let payload = b"payload larger than the configured budget";

    let exact_limit_result = encode_artifact(
        &format,
        payload,
        EncodeOptions::default()
            .with_max_payload_bytes(Some(payload.len())),
    );

    assert!(
        exact_limit_result.is_ok(),
        "a payload exactly at its configured limit must be accepted"
    );

    let smaller_limit = payload
        .len()
        .checked_sub(1)
        .expect("the test payload must be non-empty");

    let over_limit_result = encode_artifact(
        &format,
        payload,
        EncodeOptions::default()
            .with_max_payload_bytes(Some(smaller_limit)),
    );

    assert!(
        over_limit_result.is_err(),
        "the encoder must reject payloads exceeding an explicit byte budget"
    );
}

#[test]
fn decoder_rejects_payloads_that_exceed_the_callers_budget() {
    let format = ArtifactFormat::new();
    let payload = b"payload larger than the configured budget";

    let encoded = encode_artifact(
        &format,
        payload,
        EncodeOptions::default(),
    )
    .expect("the baseline artifact must encode");

    let exact_limit_result = decode_artifact(
        &encoded,
        &DecodeOptions::default()
            .with_max_payload_bytes(Some(payload.len())),
    );

    assert!(
        exact_limit_result.is_ok(),
        "a payload exactly at its configured limit must be accepted"
    );

    let smaller_limit = payload
        .len()
        .checked_sub(1)
        .expect("the test payload must be non-empty");

    let over_limit_result = decode_artifact(
        &encoded,
        &DecodeOptions::default()
            .with_max_payload_bytes(Some(smaller_limit)),
    );

    assert!(
        over_limit_result.is_err(),
        "the decoder must reject an artifact exceeding the explicit payload budget"
    );
}