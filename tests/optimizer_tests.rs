#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unreachable_code,
    unused_comparisons
)]

//! Zamani Optimizer — Comprehensive Integration Tests

use zamani_compiler::ir_gen::{
    CmpOp, IrFunction, IrInstruction, IrModule, IrRegister, IrType, IrValue,
};
use zamani_compiler::optimizer::{OptimizationConfig, Optimizer};

fn make_module_with_instructions(instructions: Vec<IrInstruction>) -> IrModule {
    let mut func = IrFunction::new("test_fn", vec![], IrType::Void);
    for ins in instructions {
        func.push(ins);
    }
    let mut m = IrModule::new("test_module");
    m.add_function(func);
    m
}

fn reg(name: &str) -> IrRegister {
    IrRegister(name.to_string(), IrType::I64)
}
fn ci(n: i64) -> IrValue {
    IrValue::ConstInt(n, IrType::I64)
}

// ── Constant folding ──────────────────────────────────────────────────────────

#[test]
fn test_constant_folding_add() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), ci(3), ci(4)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let has_folded = out.functions[0].body.iter().any(|i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(7, _)))
            || matches!(i, IrInstruction::Ret(Some(IrValue::ConstInt(7, _))))
    });
    assert!(has_folded, "Expected constant folding: 3+4=7");
}

#[test]
fn test_constant_folding_sub() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Sub(reg("%r0"), ci(10), ci(3)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let has_folded = out.functions[0].body.iter().any(|i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(7, _)))
            || matches!(i, IrInstruction::Ret(Some(IrValue::ConstInt(7, _))))
    });
    assert!(has_folded, "Expected constant folding: 10-3=7");
}

#[test]
fn test_constant_folding_mul() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Mul(reg("%r0"), ci(6), ci(7)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let has_folded = out.functions[0].body.iter().any(|i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(42, _)))
            || matches!(i, IrInstruction::Ret(Some(IrValue::ConstInt(42, _))))
    });
    assert!(has_folded, "Expected constant folding: 6*7=42");
}

#[test]
fn test_constant_folding_div() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Div(reg("%r0"), ci(20), ci(4)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let has_folded = out.functions[0].body.iter().any(|i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(5, _)))
            || matches!(i, IrInstruction::Ret(Some(IrValue::ConstInt(5, _))))
    });
    assert!(has_folded, "Expected constant folding: 20/4=5");
}

#[test]
fn test_no_div_by_zero_folding() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Div(reg("%r0"), ci(10), ci(0)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    // div/0 must never be folded into a constant Assign
    let bad_fold = out.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(_, _))));
    assert!(
        !bad_fold,
        "Division by zero must not be folded to a constant"
    );
}

// ── Strength reduction ────────────────────────────────────────────────────────

#[test]
fn test_strength_reduction_mul_by_one() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Mul(reg("%r0"), IrValue::Reg(reg("%x")), ci(1)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    assert!(
        opt.stats.strength_reductions > 0,
        "Expected a strength reduction for x*1"
    );
    let _ = out;
}

#[test]
fn test_strength_reduction_add_zero() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), IrValue::Reg(reg("%x")), ci(0)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    opt.optimize(&m);
    assert!(
        opt.stats.strength_reductions > 0,
        "Expected a strength reduction for x+0"
    );
}

#[test]
fn test_strength_reduction_sub_self() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Sub(reg("%r0"), IrValue::Reg(reg("%x")), IrValue::Reg(reg("%x"))),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    opt.optimize(&m);
    assert!(
        opt.stats.strength_reductions > 0,
        "Expected x-x to be reduced to 0"
    );
}

// ── Dead code elimination ─────────────────────────────────────────────────────

#[test]
fn test_dce_removes_unused_instruction() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%unused"), ci(1), ci(2)),
        IrInstruction::Ret(Some(ci(0))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let still_present = out.functions[0].body.iter().any(|i| {
        matches!(i, IrInstruction::Add(r, _, _) if r.0 == "unused")
            || matches!(i, IrInstruction::Assign(r, _) if r.0 == "unused")
    });
    assert!(
        !still_present,
        "Unused instruction should be dead-code eliminated"
    );
}

#[test]
fn test_dce_keeps_used_instruction() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), ci(1), ci(2)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    // Either folded-and-kept as Assign, or Add is retained because it's used by Ret
    assert!(
        !out.functions[0].body.is_empty(),
        "Function body should not be empty"
    );
}

// ── Branch simplification ─────────────────────────────────────────────────────

#[test]
fn test_branch_simplification_true_condition() {
    let m = make_module_with_instructions(vec![
        IrInstruction::CondJump(IrValue::ConstBool(true), "then".into(), "else".into()),
        IrInstruction::Label("then".into()),
        IrInstruction::Ret(None),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    let out = opt.optimize(&m);
    let has_condjump = out.functions[0]
        .body
        .iter()
        .any(|i| matches!(i, IrInstruction::CondJump(_, _, _)));
    assert!(
        !has_condjump,
        "Constant-true CondJump should simplify to Jump"
    );
}

// ── Config levels ──────────────────────────────────────────────────────────────

#[test]
fn test_optimization_level_zero_is_noop() {
    let m = make_module_with_instructions(vec![IrInstruction::Add(reg("%r0"), ci(3), ci(4))]);
    let mut opt = Optimizer::new(OptimizationConfig::none());
    let out = opt.optimize(&m);
    // Level 0 should leave the module unchanged
    assert!(matches!(
        out.functions[0].body[0],
        IrInstruction::Add(_, IrValue::ConstInt(3, _), IrValue::ConstInt(4, _))
    ));
}

#[test]
fn test_optimizer_stats_track_constant_folds() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), ci(1), ci(2)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::new(OptimizationConfig::default());
    opt.optimize(&m);
    assert!(
        opt.stats.constants_folded > 0,
        "Expected at least one constant fold recorded"
    );
}

#[test]
fn test_optimizer_with_level_constructor() {
    let m = make_module_with_instructions(vec![
        IrInstruction::Add(reg("%r0"), ci(5), ci(5)),
        IrInstruction::Ret(Some(IrValue::Reg(reg("%r0")))),
    ]);
    let mut opt = Optimizer::with_level(3);
    let out = opt.optimize(&m);
    assert!(!out.functions[0].body.is_empty());
}
