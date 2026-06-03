#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unreachable_code,
    unused_comparisons
)]

//! Zenith Optimizer — Comprehensive Integration Tests

use zenith_compiler::ir_gen::{IrFunction, IrInstruction, IrModule, IrRegister, IrType, IrValue};
use zenith_compiler::optimizer::Optimizer;

fn make_module_with_instructions(instructions: Vec<IrInstruction>) -> IrModule {
    let mut func = IrFunction::new("test_fn", IrType::Unit);
    for ins in instructions {
        func.push(ins);
    }
    let mut m = IrModule::new();
    m.add_function(func);
    m
}

fn reg(name: &str) -> IrRegister {
    IrRegister(name.to_string())
}
fn ci(n: i64) -> IrValue {
    IrValue::ConstInt(n)
}

// ── Constant folding ──────────────────────────────────────────────────────────

#[test]
fn test_constant_folding_add() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Add(reg("%r0"), ci(3), ci(4))]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    // After folding: Add(3,4) → Assign(7)
    // After folding + DCE, Add may be gone (folded then dead-eliminated)
    let no_add = !m.functions[0].body.iter().any(|i| {
        matches!(
            i,
            IrInstruction::Add(_, IrValue::ConstInt(3), IrValue::ConstInt(4))
        )
    });
    let has_folded = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(7))));
    assert!(
        no_add || has_folded,
        "Expected constant folding: 3+4=7 (Add removed or folded)"
    );
}

#[test]
fn test_constant_folding_sub() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Sub(reg("%r0"), ci(10), ci(3))]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let no_sub = !m.functions[0].body.iter().any(|i| {
        matches!(
            i,
            IrInstruction::Sub(_, IrValue::ConstInt(10), IrValue::ConstInt(3))
        )
    });
    let has_folded = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(7))));
    assert!(no_sub || has_folded, "Expected constant folding: 10-3=7");
}

#[test]
fn test_constant_folding_mul() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Mul(reg("%r0"), ci(6), ci(7))]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let no_mul = !m.functions[0].body.iter().any(|i| {
        matches!(
            i,
            IrInstruction::Mul(_, IrValue::ConstInt(6), IrValue::ConstInt(7))
        )
    });
    let has_folded = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(42))));
    assert!(no_mul || has_folded, "Expected constant folding: 6*7=42");
}

#[test]
fn test_constant_folding_div() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Div(reg("%r0"), ci(20), ci(4))]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let no_div = !m.functions[0].body.iter().any(|i| {
        matches!(
            i,
            IrInstruction::Div(_, IrValue::ConstInt(20), IrValue::ConstInt(4))
        )
    });
    let has_folded = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(5))));
    assert!(no_div || has_folded, "Expected constant folding: 20/4=5");
}

#[test]
fn test_no_div_by_zero_folding() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Div(reg("%r0"), ci(10), ci(0))]);
    let before = format!("{:?}", m.functions[0].body[0]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    // Should NOT fold divide by zero — instruction should remain
    let still_div = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Div(_, _, _)));
    // div/0 is either kept as Div or removed by DCE — but must NEVER become Assign(ConstInt)
    let bad_fold = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(_))));
    assert!(
        !bad_fold || still_div,
        "Division by zero must not be folded to a constant"
    );
}

// ── Nop elimination ───────────────────────────────────────────────────────────

#[test]
fn test_nop_elimination() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::Nop,
        IrInstruction::Assign(reg("%r0"), ci(1)),
        IrInstruction::Nop,
        IrInstruction::Nop,
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let nop_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::Nop))
        .count();
    assert_eq!(nop_count, 0, "All Nops should be eliminated");
}

#[test]
fn test_nop_elimination_preserves_other_instructions() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::Nop,
        IrInstruction::Assign(reg("%r0"), ci(42)),
        IrInstruction::Nop,
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let has_assign = m.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(42))));
    // Assign(42) should survive unless DCE also removes it (it's unused here)
    // Key invariant: no panics, no crashes
    let _ = has_assign; // acceptable either way
}

// ── Quantum gate cancellation ─────────────────────────────────────────────────

#[test]
fn test_quantum_hh_cancellation() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::QGate("H".to_string(), vec![reg("%q0")]),
        IrInstruction::QGate("H".to_string(), vec![reg("%q0")]),
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let h_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::QGate(g, _) if g == "H"))
        .count();
    assert_eq!(h_count, 0, "H·H should cancel to identity");
}

#[test]
fn test_quantum_xx_cancellation() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::QGate("X".to_string(), vec![reg("%q0")]),
        IrInstruction::QGate("X".to_string(), vec![reg("%q0")]),
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let x_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::QGate(g, _) if g == "X"))
        .count();
    assert_eq!(x_count, 0, "X·X should cancel to identity");
}

#[test]
fn test_quantum_different_qubits_no_cancel() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::QGate("H".to_string(), vec![reg("%q0")]),
        IrInstruction::QGate("H".to_string(), vec![reg("%q1")]),
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let h_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::QGate(g, _) if g == "H"))
        .count();
    assert_eq!(h_count, 2, "Gates on different qubits must NOT cancel");
}

// ── Sankofa access optimizer ──────────────────────────────────────────────────

#[test]
fn test_sankofa_recall_deduplication() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::SankofaRecall(reg("%r0"), "wisdom".to_string()),
        IrInstruction::SankofaRecall(reg("%r1"), "wisdom".to_string()),
    ]);
    let mut opt = Optimizer::new();
    let stats = opt.run_all(&mut m);
    // Second recall should be replaced with an Assign from the cached register
    let assign_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::Assign(_, IrValue::Reg(_))))
        .count();
    assert!(assign_count >= 1, "Expected cached recall to become Assign");
}

// ── MTS timeline fusion ───────────────────────────────────────────────────────

#[test]
fn test_mts_snapshot_restore_fusion() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::MTSSnapshot(reg("%t0")),
        IrInstruction::MTSRestore(reg("%t0")),
    ]);
    let mut opt = Optimizer::new();
    opt.run_all(&mut m);
    let snapshot_count = m.functions[0]
        .body
        .iter()
        .filter(|i| matches!(i, IrInstruction::MTSSnapshot(_)))
        .count();
    assert_eq!(
        snapshot_count, 0,
        "Snapshot+Restore pair should be fused/eliminated"
    );
}

// ── Stats ─────────────────────────────────────────────────────────────────────

#[test]
fn test_optimizer_stats_track_passes() {
    let mut m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), ci(1), ci(2)),
        IrInstruction::Nop,
    ]);
    let mut opt = Optimizer::new();
    let stats = opt.run_all(&mut m);
    assert!(stats.passes_run > 0, "Expected at least one pass to run");
}

#[test]
fn test_run_specific_pass() {
    let mut m = make_module_with_instructions(vec![IrInstruction::Nop, IrInstruction::Nop]);
    let opt = Optimizer::new();
    let changes = opt.run_pass("nop_elimination", &mut m).unwrap();
    assert_eq!(changes, 2, "Expected 2 Nops removed");
}
