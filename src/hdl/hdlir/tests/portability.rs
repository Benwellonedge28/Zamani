
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! HDL portability and scalability regression tests.
//!
//! Path: `src/hdl/hdlir/tests/portability.rs`
//! Rust: 1.97+
//! Edition: 2021
//!
//! STATUS
//! ------
//! Initial portability regression suite. These tests exercise the existing
//! canonical HDL semantic contract as a prerequisite for HDLIR integration.
//! They do not claim that the HDLIR graph, target-feasibility engine,
//! serialization, or cross-target lowering has already been implemented.
//!
//! AUTHORITY
//! ---------
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`, where registered and available
//! - The HDLIR specification and compatibility contract, once implemented.
//!
//! OWNERSHIP
//! ---------
//! This file owns tests only. It does not define production HDLIR types,
//! target descriptors, semantic rules, hardware limits, or alternate IRs.
//!
//! INTEGRATION
//! -----------
//! Include this file from `src/hdl/hdlir/tests/mod.rs` using
//! `mod portability;` and register the test module from the HDLIR
//! composition root once that root is valid Rust and the test hierarchy
//! has been established.
//!
//! The current HDLIR directory does not yet expose the APIs required for
//! genuine HDLIR round-trip and cross-target realization tests. Those tests
//! must be added against the actual implementation when those contracts
//! exist; they must not be simulated with test-only production substitutes.
//!
//! SCALABILITY
//! -----------
//! Test sizes below are workloads, not language limits. No test establishes
//! a maximum number of signals, modules, bits, or hardware resources.
//! Symbolic values must remain symbolic until an owning stage can resolve
//! them. Concrete values must be checked without narrowing them to `usize`.
//!
//! SAFETY
//! ------
//! Safe Rust only. No unsafe code, external hardware access, or target
//! discovery is required by this suite.

use crate::frontend::semantic::domains::hdl::{
    validate_hdl, HdlDirection, HdlId, HdlProgram, HdlSeverity, HdlSignal,
    HdlSignedness, HdlValueType, HdlWidth, HdlValidationStatus,
};

/// Construct a signal using the canonical HDL semantic input contract.
///
/// The helper intentionally delegates identity validation to `HdlId` and
/// does not introduce a test-only signal or width representation.
fn signal(
    id: impl Into<String>,
    name: impl Into<String>,
    width: HdlWidth,
) -> HdlSignal {
    HdlSignal {
        id: HdlId::new(id).expect("test signal ID must be non-empty"),
        name: name.into(),
        direction: HdlDirection::Internal,
        value_type: HdlValueType::BitVector {
            width,
            signedness: HdlSignedness::Unsigned,
        },
        location: None,
    }
}

/// Return the first canonical HDL signal width, if present.
fn first_signal_width(program: &HdlProgram) -> Option<&HdlWidth> {
    program.signals.first().and_then(|signal| {
        match &signal.value_type {
            HdlValueType::BitVector { width, .. } => Some(width),
            _ => None,
        }
    })
}

#[test]
fn symbolic_widths_are_preserved_without_host_sized_narrowing() {
    let parameter = HdlId::new("DATA_WIDTH").expect("valid parameter ID");

    let symbolic_width = HdlWidth::Add(
        Box::new(HdlWidth::Symbol(parameter.clone())),
        Box::new(HdlWidth::Multiply(
            Box::new(HdlWidth::Known(2)),
            Box::new(HdlWidth::Symbol(parameter)),
        )),
    );

    let program = HdlProgram {
        signals: vec![signal(
            "signal-1",
            "data",
            symbolic_width.clone(),
        )],
        ..HdlProgram::default()
    };

    assert_eq!(first_signal_width(&program), Some(&symbolic_width));

    // Validation may diagnose unresolved semantic relationships, but it
    // must not mutate the caller's program or rewrite symbolic expressions
    // into guessed concrete hardware sizes.
    let before = program.clone();
    let _result = validate_hdl(&program);

    assert_eq!(program, before);
    assert_eq!(first_signal_width(&program), Some(&symbolic_width));
}

#[test]
fn unresolved_widths_remain_explicit() {
    let unresolved_id =
        HdlId::new("UNRESOLVED_PARAMETER").expect("valid parameter ID");
    let width = HdlWidth::Unresolved(unresolved_id);

    assert!(width.is_unresolved());

    let program = HdlProgram {
        signals: vec![signal("signal-1", "payload", width.clone())],
        ..HdlProgram::default()
    };

    assert_eq!(first_signal_width(&program), Some(&width));

    let before = program.clone();
    let result = validate_hdl(&program);

    assert_eq!(program, before);
    assert_eq!(first_signal_width(&program), Some(&width));

    // An unresolved relationship must not be mistaken for proof that a
    // target can realize the design. The validator's exact diagnostic
    // policy remains authoritative.
    if result.status == HdlValidationStatus::Valid {
        assert!(
            result
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.severity != HdlSeverity::Error),
            "a valid result must not contain error-severity diagnostics"
        );
    }
}

#[test]
fn positive_concrete_widths_are_not_narrowed_to_usize() {
    // Some host architectures can represent a u128 value that does not fit
    // in usize. Exercise that case whenever the current host provides one.
    let beyond_usize = (usize::MAX as u128).checked_add(1);

    let Some(width_value) = beyond_usize else {
        // On a hypothetical host whose usize already spans all u128 values,
        // there is no u128 value above usize::MAX to test.
        return;
    };

    let width = HdlWidth::Known(width_value);

    assert_eq!(width.as_known(), Some(width_value));

    let program = HdlProgram {
        signals: vec![signal("wide-1", "wide_data", width.clone())],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(first_signal_width(&program), Some(&width));

    // The width is positive and representable by the canonical u128
    // contract. It must not be rejected solely because it cannot fit in
    // the host's usize.
    assert!(
        !result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == crate::frontend::semantic::domains::hdl::HdlDiagnosticCode::InvalidWidth
        }),
        "a positive u128 width must not be rejected by host-size narrowing"
    );
}

#[test]
fn equivalent_inputs_produce_deterministic_validation_results() {
    let width_id = HdlId::new("BUS_WIDTH").expect("valid width ID");

    let program = HdlProgram {
        signals: vec![
            signal(
                "signal-a",
                "input_bus",
                HdlWidth::Symbol(width_id.clone()),
            ),
            signal(
                "signal-b",
                "output_bus",
                HdlWidth::Add(
                    Box::new(HdlWidth::Symbol(width_id.clone())),
                    Box::new(HdlWidth::Known(1)),
                ),
            ),
            signal(
                "signal-c",
                "wide_bus",
                HdlWidth::Multiply(
                    Box::new(HdlWidth::Symbol(width_id)),
                    Box::new(HdlWidth::Known(2)),
                ),
            ),
        ],
        ..HdlProgram::default()
    };

    let first = validate_hdl(&program);
    let second = validate_hdl(&program);

    assert_eq!(
        first, second,
        "equivalent input and configuration must produce equal results"
    );
}

#[test]
fn validation_does_not_mutate_the_semantic_input() {
    let program = HdlProgram {
        signals: vec![
            signal("signal-a", "a", HdlWidth::Known(1)),
            signal("signal-b", "b", HdlWidth::Known(8)),
            signal(
                "signal-c",
                "c",
                HdlWidth::Add(
                    Box::new(HdlWidth::Known(8)),
                    Box::new(HdlWidth::Known(8)),
                ),
            ),
        ],
        ..HdlProgram::default()
    };

    let original = program.clone();

    let _ = validate_hdl(&program);

    assert_eq!(
        program, original,
        "validation must not mutate caller-owned semantic input"
    );
}

#[test]
fn increasing_design_sizes_do_not_depend_on_a_fixed_hardware_ceiling() {
    // These are deliberately modest CI workloads, not claims about the
    // maximum supported design size. Increase them in stress-test jobs
    // according to the test environment and explicit operational budgets.
    //
    // This test exercises the actual semantic validator. It does not prove
    // HDLIR graph scalability until an HDLIR builder and verifier exist.
    for count in [0_usize, 1, 16, 256, 2_048] {
        let mut program = HdlProgram::default();

        program.signals.reserve(count);

        for index in 0..count {
            let id = format!("signal-{index}");
            let name = format!("data-{index}");

            program.signals.push(signal(
                id,
                name,
                HdlWidth::Known(1),
            ));
        }

        assert_eq!(program.signals.len(), count);

        let before = program.clone();
        let first = validate_hdl(&program);
        let second = validate_hdl(&program);

        assert_eq!(
            first, second,
            "validation must be deterministic for {count} signals"
        );

        assert_eq!(
            program, before,
            "validation must preserve the input at size {count}"
        );

        assert!(
            first.complete,
            "the existing validator must report its actual completion state"
        );
    }
}

#[test]
fn duplicate_logical_identities_are_not_made_portable_by_rewriting_them() {
    let duplicate_id = "duplicate-signal";

    let program = HdlProgram {
        signals: vec![
            signal(duplicate_id, "left", HdlWidth::Known(1)),
            signal(duplicate_id, "right", HdlWidth::Known(1)),
        ],
        ..HdlProgram::default()
    };

    let original = program.clone();
    let result = validate_hdl(&program);

    assert_eq!(program, original);

    assert_eq!(
        result.status,
        HdlValidationStatus::Invalid,
        "duplicate identities must be diagnosed, not silently rewritten"
    );

    assert!(
        result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == crate::frontend::semantic::domains::hdl::HdlDiagnosticCode::DuplicateIdentity
        }),
        "the canonical validator must identify the duplicate identity"
    );
}

#[test]
fn invalid_width_is_not_silently_converted_to_a_valid_width() {
    // The public enum can represent malformed input even though the
    // HdlWidth::known constructor rejects zero. The validator must diagnose
    // malformed externally constructed semantic input.
    let program = HdlProgram {
        signals: vec![signal(
            "zero-width-signal",
            "invalid_data",
            HdlWidth::Known(0),
        )],
        ..HdlProgram::default()
    };

    let original = program.clone();
    let result = validate_hdl(&program);

    assert_eq!(program, original);

    assert_eq!(
        result.status,
        HdlValidationStatus::Invalid,
        "a zero-width signal must not be accepted as valid"
    );

    assert!(
        result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == crate::frontend::semantic::domains::hdl::HdlDiagnosticCode::InvalidWidth
        }),
        "the invalid width must have a structured diagnostic"
    );
}
