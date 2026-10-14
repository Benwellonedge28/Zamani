
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Integration tests for Zamani HDL semantic validation.
//!
//! Path: `src/frontend/semantic/tests/hdl.rs`
//!
//! PURPOSE
//!
//! Exercise the public HDL semantic contract against valid, invalid, tiny,
//! symbolic, large, and cross-reference designs. These tests intentionally
//! do not construct a competing AST or invoke target-specific realization.
//!
//! AUTHORITY
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/processes.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! INTEGRATION
//!
//! This file is a child module of `src/frontend/semantic/tests/mod.rs`.
//! It tests the existing `domains::hdl` API. Grammar checks below verify
//! repository contract artifacts are present; they do not replace ANTLR
//! generation or parser conformance tests.
//!
//! SCALABILITY
//!
//! Test sizes are examples, never language limits. Symbolic and very large
//! widths must remain representable without target allocation or truncation.
//!
//! COMPLETION
//!
//! Run this test module, the complete semantic suite, grammar/parser tests,
//! and workspace checks before declaring HDL production-ready.

use std::collections::BTreeSet;

use crate::frontend::semantic::domains::hdl::{
    validate_hdl, HdlAssignment, HdlClock, HdlDiagnosticCode, HdlDirection, HdlId,
    HdlProcess, HdlProcessKind, HdlProgram, HdlReset, HdlResetBehavior,
    HdlResetPolarity, HdlSeverity, HdlSignal, HdlSignedness, HdlSourceLocation,
    HdlValidationStatus, HdlValueType, HdlWidth,
};

fn id(value: &str) -> HdlId {
    HdlId::new(value).expect("test identifier must be non-empty")
}

fn signal(
    name: &str,
    direction: HdlDirection,
    value_type: HdlValueType,
) -> HdlSignal {
    HdlSignal {
        id: id(name),
        name: name.to_owned(),
        direction,
        value_type,
        location: None,
    }
}

fn bit(name: &str, direction: HdlDirection) -> HdlSignal {
    signal(name, direction, HdlValueType::Bit)
}

fn vector(
    name: &str,
    direction: HdlDirection,
    width: HdlWidth,
) -> HdlSignal {
    signal(
        name,
        direction,
        HdlValueType::BitVector {
            width,
            signedness: HdlSignedness::Unsigned,
        },
    )
}

fn location(source: &str, start: u64, end: u64) -> HdlSourceLocation {
    HdlSourceLocation::new(source, start, end)
        .expect("test source range must be valid")
}

fn assignment(target: &str, reads: &[&str]) -> HdlAssignment {
    HdlAssignment {
        target: id(target),
        reads: reads.iter().map(|name| id(name)).collect(),
        continuous: false,
        location: None,
    }
}

fn combinational_process(
    process_id: &str,
    assignments: Vec<HdlAssignment>,
) -> HdlProcess {
    HdlProcess {
        id: id(process_id),
        kind: HdlProcessKind::Combinational,
        assignments,
        clocks: Vec::new(),
        resets: Vec::new(),
        location: None,
    }
}

fn sequential_process(
    process_id: &str,
    clock_ids: &[&str],
    reset_ids: &[&str],
    assignments: Vec<HdlAssignment>,
) -> HdlProcess {
    HdlProcess {
        id: id(process_id),
        kind: HdlProcessKind::Sequential,
        assignments,
        clocks: clock_ids.iter().map(|name| id(name)).collect(),
        resets: reset_ids.iter().map(|name| id(name)).collect(),
        location: None,
    }
}

fn has_code(
    result: &crate::frontend::semantic::domains::hdl::HdlValidationResult,
    code: HdlDiagnosticCode,
) -> bool {
    result.diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

#[test]
fn empty_program_is_valid_and_complete() {
    let result = validate_hdl(&HdlProgram::default());

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
    assert!(result.diagnostics.is_empty());
}

#[test]
fn smallest_combinational_design_is_valid() {
    let program = HdlProgram {
        signals: vec![
            bit("input_value", HdlDirection::Input),
            bit("output_value", HdlDirection::Output),
        ],
        processes: vec![combinational_process(
            "invert_or_forward",
            vec![assignment("output_value", &["input_value"])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
    assert!(result.diagnostics.is_empty());
}

#[test]
fn positive_concrete_width_is_valid() {
    let program = HdlProgram {
        signals: vec![vector(
            "payload",
            HdlDirection::Internal,
            HdlWidth::known(8).expect("positive width"),
        )],
        ..HdlProgram::default()
    };

    assert_eq!(validate_hdl(&program).status, HdlValidationStatus::Valid);
}

#[test]
fn zero_width_is_rejected_by_semantic_validation() {
    // Direct construction is intentional: semantic validation must defend
    // against malformed normalized inputs even if normal constructors reject
    // invalid values.
    let program = HdlProgram {
        signals: vec![vector(
            "payload",
            HdlDirection::Internal,
            HdlWidth::Known(0),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidWidth));
}

#[test]
fn zero_width_constructor_rejects_invalid_width() {
    assert!(HdlWidth::known(0).is_err());
}

#[test]
fn symbolic_width_is_preserved_without_target_resolution() {
    let symbolic = HdlWidth::Multiply(
        Box::new(HdlWidth::Symbol(id("lane_count"))),
        Box::new(HdlWidth::Known(32)),
    );

    assert_eq!(symbolic.as_known(), None);

    let program = HdlProgram {
        signals: vec![vector(
            "payload",
            HdlDirection::Internal,
            symbolic.clone(),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert_eq!(
        match &program.signals[0].value_type {
            HdlValueType::BitVector { width, .. } => width,
            _ => panic!("expected bit-vector type"),
        },
        &symbolic
    );
}

#[test]
fn very_large_concrete_width_is_not_a_language_capacity_limit() {
    let program = HdlProgram {
        signals: vec![vector(
            "large_symbolic_domain",
            HdlDirection::Internal,
            HdlWidth::Known(u128::MAX),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    // This validates semantic representation only. It does not allocate
    // storage for the represented width or assert target feasibility.
    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
}

#[test]
fn nested_symbolic_width_expression_remains_symbolic() {
    let width = HdlWidth::Add(
        Box::new(HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("banks"))),
            Box::new(HdlWidth::Known(64)),
        )),
        Box::new(HdlWidth::Symbol(id("extra_bits"))),
    );

    assert_eq!(width.as_known(), None);

    let program = HdlProgram {
        signals: vec![vector("memory_bus", HdlDirection::Internal, width)],
        ..HdlProgram::default()
    };

    assert_eq!(validate_hdl(&program).status, HdlValidationStatus::Valid);
}

#[test]
fn unresolved_width_is_reported_as_a_warning_not_silently_resolved() {
    let program = HdlProgram {
        signals: vec![vector(
            "deferred_width",
            HdlDirection::Internal,
            HdlWidth::Unresolved(id("width_parameter")),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
    assert!(result.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == HdlDiagnosticCode::UnresolvedWidth
            && diagnostic.severity == HdlSeverity::Warning
    }));
}

#[test]
fn empty_and_whitespace_only_identifiers_are_rejected() {
    assert!(HdlId::new("").is_err());
    assert!(HdlId::new("   ").is_err());
    assert!(HdlId::new("\t\n").is_err());
}

#[test]
fn source_locations_use_valid_half_open_ranges() {
    let valid = HdlSourceLocation::new("design.zm", 4, 9)
        .expect("valid half-open range");

    assert_eq!(valid.source, "design.zm");
    assert_eq!(valid.start, 4);
    assert_eq!(valid.end, 9);

    let empty = HdlSourceLocation::new("design.zm", 9, 9)
        .expect("empty insertion-point range is valid");

    assert_eq!(empty.start, empty.end);
    assert!(HdlSourceLocation::new("design.zm", 10, 9).is_err());
}

#[test]
fn duplicate_signal_identity_is_diagnosed() {
    let program = HdlProgram {
        signals: vec![
            HdlSignal {
                id: id("shared_id"),
                name: "first".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::Bit,
                location: None,
            },
            HdlSignal {
                id: id("shared_id"),
                name: "second".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::Bit,
                location: None,
            },
        ],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::DuplicateIdentity));
}

#[test]
fn duplicate_signal_names_are_diagnosed() {
    let program = HdlProgram {
        signals: vec![
            HdlSignal {
                id: id("signal_a"),
                name: "data".to_owned(),
                direction: HdlDirection::Input,
                value_type: HdlValueType::Bit,
                location: None,
            },
            HdlSignal {
                id: id("signal_b"),
                name: "data".to_owned(),
                direction: HdlDirection::Output,
                value_type: HdlValueType::Bit,
                location: None,
            },
        ],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::DuplicateName));
}

#[test]
fn process_cannot_drive_an_input_signal() {
    let program = HdlProgram {
        signals: vec![bit("external_input", HdlDirection::Input)],
        processes: vec![combinational_process(
            "illegal_driver",
            vec![assignment("external_input", &[])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidDirection));
}

#[test]
fn unknown_assignment_target_is_diagnosed() {
    let program = HdlProgram {
        processes: vec![combinational_process(
            "unknown_target",
            vec![assignment("missing_signal", &[])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::UnknownSignal));
}

#[test]
fn unknown_read_dependency_is_diagnosed() {
    let program = HdlProgram {
        signals: vec![bit("output_value", HdlDirection::Output)],
        processes: vec![combinational_process(
            "unknown_read",
            vec![assignment("output_value", &["not_declared"])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::UnknownSignal));
}

#[test]
fn combinational_process_cannot_declare_clock_events() {
    let program = HdlProgram {
        processes: vec![HdlProcess {
            id: id("bad_combinational"),
            kind: HdlProcessKind::Combinational,
            assignments: Vec::new(),
            clocks: vec![id("clock_event")],
            resets: Vec::new(),
            location: None,
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidProcess));
    assert!(has_code(
        &result,
        HdlDiagnosticCode::UnknownClockOrReset
    ));
}

#[test]
fn sequential_process_requires_a_clock() {
    let program = HdlProgram {
        processes: vec![sequential_process(
            "missing_clock",
            &[],
            &[],
            Vec::new(),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidProcess));
}

#[test]
fn valid_clock_and_sequential_process_are_accepted() {
    let clock_signal = HdlSignal {
        id: id("clk_signal"),
        name: "clk".to_owned(),
        direction: HdlDirection::Input,
        value_type: HdlValueType::Clock,
        location: None,
    };

    let program = HdlProgram {
        signals: vec![
            clock_signal,
            bit("data_in", HdlDirection::Input),
            bit("data_out", HdlDirection::Output),
        ],
        clocks: vec![HdlClock {
            id: id("main_clock"),
            signal: id("clk_signal"),
            period: Some(id("clock_period")),
            location: None,
        }],
        processes: vec![sequential_process(
            "register_process",
            &["main_clock"],
            &[],
            vec![assignment("data_out", &["data_in"])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
}

#[test]
fn clock_must_reference_a_clock_typed_signal() {
    let program = HdlProgram {
        signals: vec![bit("ordinary_signal", HdlDirection::Input)],
        clocks: vec![HdlClock {
            id: id("bad_clock"),
            signal: id("ordinary_signal"),
            period: None,
            location: None,
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidClock));
}

#[test]
fn clock_with_unknown_signal_is_diagnosed() {
    let program = HdlProgram {
        clocks: vec![HdlClock {
            id: id("missing_signal_clock"),
            signal: id("not_declared"),
            period: None,
            location: None,
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidClock));
}

#[test]
fn reset_must_reference_a_reset_typed_signal() {
    let program = HdlProgram {
        signals: vec![bit("ordinary_signal", HdlDirection::Input)],
        resets: vec![HdlReset {
            id: id("bad_reset"),
            signal: id("ordinary_signal"),
            polarity: HdlResetPolarity::ActiveHigh,
            behavior: HdlResetBehavior::Synchronous,
            location: None,
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidReset));
}

#[test]
fn reset_with_unknown_signal_is_diagnosed() {
    let program = HdlProgram {
        resets: vec![HdlReset {
            id: id("missing_signal_reset"),
            signal: id("not_declared"),
            polarity: HdlResetPolarity::ActiveLow,
            behavior: HdlResetBehavior::Asynchronous,
            location: None,
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::InvalidReset));
}

#[test]
fn process_referencing_unknown_clock_or_reset_is_diagnosed() {
    let program = HdlProgram {
        processes: vec![sequential_process(
            "unresolved_events",
            &["unknown_clock"],
            &["unknown_reset"],
            Vec::new(),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(
        &result,
        HdlDiagnosticCode::UnknownClockOrReset
    ));
}

#[test]
fn multiple_distinct_process_drivers_are_diagnosed() {
    let program = HdlProgram {
        signals: vec![
            bit("left", HdlDirection::Input),
            bit("right", HdlDirection::Input),
            bit("shared_output", HdlDirection::Output),
        ],
        processes: vec![
            combinational_process(
                "driver_one",
                vec![assignment("shared_output", &["left"])],
            ),
            combinational_process(
                "driver_two",
                vec![assignment("shared_output", &["right"])],
            ),
        ],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(&result, HdlDiagnosticCode::MultipleDrivers));
}

#[test]
fn repeated_assignments_inside_one_process_are_not_automatically_multiple_drivers() {
    let program = HdlProgram {
        signals: vec![
            bit("left", HdlDirection::Input),
            bit("right", HdlDirection::Input),
            bit("output_value", HdlDirection::Output),
        ],
        processes: vec![combinational_process(
            "single_owner",
            vec![
                assignment("output_value", &["left"]),
                assignment("output_value", &["right"]),
            ],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert!(!has_code(
        &result,
        HdlDiagnosticCode::MultipleDrivers
    ));
}

#[test]
fn combinational_dependency_cycle_is_diagnosed() {
    let program = HdlProgram {
        signals: vec![
            bit("a", HdlDirection::Internal),
            bit("b", HdlDirection::Internal),
        ],
        processes: vec![
            combinational_process("drive_a", vec![assignment("a", &["b"])]),
            combinational_process("drive_b", vec![assignment("b", &["a"])]),
        ],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(
        &result,
        HdlDiagnosticCode::CombinationalCycle
    ));
}

#[test]
fn combinational_self_dependency_is_diagnosed() {
    let program = HdlProgram {
        signals: vec![bit("loopback", HdlDirection::Internal)],
        processes: vec![combinational_process(
            "self_loop",
            vec![assignment("loopback", &["loopback"])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Invalid);
    assert!(has_code(
        &result,
        HdlDiagnosticCode::CombinationalCycle
    ));
}

#[test]
fn sequential_feedback_is_not_added_to_the_combinational_cycle_graph() {
    let clock_signal = HdlSignal {
        id: id("clock_signal"),
        name: "clock".to_owned(),
        direction: HdlDirection::Input,
        value_type: HdlValueType::Clock,
        location: None,
    };

    let program = HdlProgram {
        signals: vec![
            clock_signal,
            bit("state", HdlDirection::Internal),
        ],
        clocks: vec![HdlClock {
            id: id("clock_id"),
            signal: id("clock_signal"),
            period: None,
            location: None,
        }],
        processes: vec![sequential_process(
            "state_update",
            &["clock_id"],
            &[],
            vec![assignment("state", &["state"])],
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(!has_code(
        &result,
        HdlDiagnosticCode::CombinationalCycle
    ));
}

#[test]
fn diagnostics_have_source_locations_when_input_provides_them() {
    let source_location = location("design.zm", 10, 20);
    let program = HdlProgram {
        signals: vec![HdlSignal {
            id: id("zero_width_signal"),
            name: "bad_bus".to_owned(),
            direction: HdlDirection::Internal,
            value_type: HdlValueType::BitVector {
                width: HdlWidth::Known(0),
                signedness: HdlSignedness::Unsigned,
            },
            location: Some(source_location.clone()),
        }],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|item| item.code == HdlDiagnosticCode::InvalidWidth)
        .expect("zero width must produce a diagnostic");

    assert_eq!(diagnostic.location.as_ref(), Some(&source_location));
    assert_eq!(diagnostic.severity, HdlSeverity::Error);
}

#[test]
fn diagnostic_order_is_deterministic_for_equivalent_input() {
    let program = HdlProgram {
        signals: vec![
            HdlSignal {
                id: id("bad_b"),
                name: "b".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::BitVector {
                    width: HdlWidth::Known(0),
                    signedness: HdlSignedness::Unsigned,
                },
                location: Some(location("design.zm", 20, 21)),
            },
            HdlSignal {
                id: id("bad_a"),
                name: "a".to_owned(),
                direction: HdlDirection::Internal,
                value_type: HdlValueType::BitVector {
                    width: HdlWidth::Known(0),
                    signedness: HdlSignedness::Unsigned,
                },
                location: Some(location("design.zm", 5, 6)),
            },
        ],
        ..HdlProgram::default()
    };

    let first = validate_hdl(&program);
    let second = validate_hdl(&program);

    assert_eq!(first, second);
    assert!(first.diagnostics.windows(2).all(|pair| {
        let left = pair[0].location.as_ref().map(|item| item.start);
        let right = pair[1].location.as_ref().map(|item| item.start);
        left <= right
    }));
}

#[test]
fn equivalent_signal_input_order_produces_equivalent_diagnostics() {
    let a = HdlSignal {
        id: id("a"),
        name: "a".to_owned(),
        direction: HdlDirection::Internal,
        value_type: HdlValueType::BitVector {
            width: HdlWidth::Known(0),
            signedness: HdlSignedness::Unsigned,
        },
        location: Some(location("design.zm", 1, 2)),
    };
    let b = HdlSignal {
        id: id("b"),
        name: "b".to_owned(),
        direction: HdlDirection::Internal,
        value_type: HdlValueType::BitVector {
            width: HdlWidth::Known(0),
            signedness: HdlSignedness::Unsigned,
        },
        location: Some(location("design.zm", 3, 4)),
    };

    let first = validate_hdl(&HdlProgram {
        signals: vec![a.clone(), b.clone()],
        ..HdlProgram::default()
    });
    let second = validate_hdl(&HdlProgram {
        signals: vec![b, a],
        ..HdlProgram::default()
    });

    assert_eq!(first, second);
}

#[test]
fn source_level_validation_does_not_claim_target_feasibility() {
    let program = HdlProgram {
        signals: vec![vector(
            "large_bus",
            HdlDirection::Internal,
            HdlWidth::Symbol(id("target_parameter")),
        )],
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    // The validator has no target description and must not infer physical
    // capacity or feasibility from a source-level symbolic width.
    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
}

#[test]
fn distinct_declaration_ids_can_be_collected_without_fixed_capacity_assumptions() {
    let count = 256usize;
    let signals = (0..count)
        .map(|index| {
            signal(
                &format!("signal_{index}"),
                HdlDirection::Internal,
                HdlValueType::Bit,
            )
        })
        .collect();

    let program = HdlProgram {
        signals,
        ..HdlProgram::default()
    };

    let result = validate_hdl(&program);

    assert_eq!(result.status, HdlValidationStatus::Valid);
    assert!(result.complete);
    assert!(result.diagnostics.is_empty());

    // This is a stress sample only, not a maximum supported declaration count.
    let ids = program
        .signals
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), count);
}

#[test]
fn grammar_composition_root_and_normative_specification_are_present() {
    const HDL_ROOT: &str = include_str!("../../../../grammar/hdl/hdl.g4");
    const HDL_SPEC: &str = include_str!("../../../../grammar/spec/hdl.md");

    assert!(!HDL_ROOT.trim().is_empty());
    assert!(!HDL_SPEC.trim().is_empty());

    // The grammar composition root and the normative specification must
    // continue to describe HDL as a language domain with downstream target
    // realization, rather than embedding physical allocation in this validator.
    assert!(HDL_ROOT.contains("grammar/hdl/hdl.g4"));
    assert!(HDL_ROOT.contains("target realization"));
    assert!(HDL_SPEC.contains("POCO-REAF"));
    assert!(HDL_SPEC.contains("Canonical AST"));
    assert!(HDL_SPEC.contains("semantic"));
}

#[test]
fn canonical_signal_clocking_and_process_grammar_files_are_present() {
    const SIGNALS: &str = include_str!("../../../../grammar/hdl/signals.g4");
    const CLOCKING: &str = include_str!("../../../../grammar/hdl/clocking.g4");
    const PROCESSES: &str = include_str!("../../../../grammar/hdl/processes.g4");

    assert!(!SIGNALS.trim().is_empty());
    assert!(!CLOCKING.trim().is_empty());
    assert!(!PROCESSES.trim().is_empty());

    // These smoke checks guard against accidentally removing or renaming the
    // grammar delegates that own the corresponding syntax domains.
    assert!(SIGNALS.contains("signals.g4"));
    assert!(CLOCKING.contains("clocking.g4"));
    assert!(PROCESSES.contains("processes.g4"));
}
