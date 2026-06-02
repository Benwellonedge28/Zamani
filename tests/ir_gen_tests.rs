//! Zenith IR Generator — Comprehensive Integration Tests

use std::sync::Arc;
use zenith_compiler::ir_gen::{IrGenerator, IrInstruction, IrModule, IrRegister, IrValue};
use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::source_map::{FileId, SourceFile};

fn gen_ir(source: &str) -> IrModule {
    let sf = Arc::new(SourceFile::new("<test>".to_string(), source.to_string()));
    let lex = Lexer::new(FileId::new(1), sf);
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    assert!(
        parser.get_errors().is_empty(),
        "Parse errors: {:?}",
        parser.get_errors()
    );
    let mut gen = IrGenerator::new();
    gen.generate(&prog)
}

fn has_instruction<F: Fn(&IrInstruction) -> bool>(module: &IrModule, pred: F) -> bool {
    module
        .functions
        .iter()
        .any(|f| f.body.iter().any(|i| pred(i)))
}

// ── Module structure ──────────────────────────────────────────────────────────

#[test]
fn test_empty_program_generates_module() {
    let module = gen_ir("");
    assert!(
        !module.functions.is_empty(),
        "Should have at least __top__ function"
    );
}

#[test]
fn test_module_has_top_level_function() {
    let module = gen_ir("let x = 1;");
    assert!(module.functions.iter().any(|f| f.name == "__top__"));
}

#[test]
fn test_function_declaration_adds_function() {
    let module = gen_ir("fn hello() { let x = 1; }");
    assert!(
        module.functions.iter().any(|f| f.name == "hello"),
        "Expected 'hello' function in IR module"
    );
}

#[test]
fn test_multiple_functions() {
    let module = gen_ir("fn a() { let x = 1; } fn b() { let y = 2; }");
    assert!(module.functions.iter().any(|f| f.name == "a"));
    assert!(module.functions.iter().any(|f| f.name == "b"));
}

// ── Integer literals ──────────────────────────────────────────────────────────

#[test]
fn test_integer_literal_generates_assign() {
    let module = gen_ir("let x = 42;");
    let found = has_instruction(&module, |i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(42)))
    });
    assert!(found, "Expected Assign with ConstInt(42)");
}

#[test]
fn test_zero_literal() {
    let module = gen_ir("let z = 0;");
    let found = has_instruction(&module, |i| {
        matches!(i, IrInstruction::Assign(_, IrValue::ConstInt(0)))
    });
    assert!(found, "Expected ConstInt(0)");
}

#[test]
fn test_negative_literal_via_prefix() {
    let module = gen_ir("let neg = 5 - 10;");
    let found = has_instruction(&module, |i| matches!(i, IrInstruction::Sub(_, _, _)));
    assert!(found, "Expected Sub instruction");
}

// ── Arithmetic IR ─────────────────────────────────────────────────────────────

#[test]
fn test_addition_generates_add() {
    let module = gen_ir("let r = 1 + 2;");
    assert!(
        has_instruction(&module, |i| matches!(i, IrInstruction::Add(_, _, _))),
        "Expected Add instruction"
    );
}

#[test]
fn test_subtraction_generates_sub() {
    let module = gen_ir("let r = 10 - 3;");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::Sub(_, _, _)
    )));
}

#[test]
fn test_multiplication_generates_mul() {
    let module = gen_ir("let r = 4 * 5;");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::Mul(_, _, _)
    )));
}

#[test]
fn test_division_generates_div() {
    let module = gen_ir("let r = 8 / 2;");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::Div(_, _, _)
    )));
}

// ── Comparison IR ─────────────────────────────────────────────────────────────

#[test]
fn test_equality_generates_cmpeq() {
    let module = gen_ir("let b = 1 == 1;");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::CmpEq(_, _, _)
    )));
}

#[test]
fn test_less_than_generates_cmplt() {
    let module = gen_ir("let b = 1 < 2;");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::CmpLt(_, _, _)
    )));
}

// ── Control flow IR ───────────────────────────────────────────────────────────

#[test]
fn test_if_generates_condjump() {
    let module = gen_ir("if true { let x = 1; }");
    assert!(
        has_instruction(&module, |i| matches!(i, IrInstruction::CondJump(_, _, _))),
        "Expected CondJump for if"
    );
}

#[test]
fn test_while_generates_jump_and_condjump() {
    let module = gen_ir("while true { let x = 1; }");
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::Jump(_)
    )));
    assert!(has_instruction(&module, |i| matches!(
        i,
        IrInstruction::CondJump(_, _, _)
    )));
}

#[test]
fn test_function_call_generates_call() {
    let module = gen_ir("let r = foo(1);");
    assert!(has_instruction(
        &module,
        |i| matches!(i, IrInstruction::Call(_, name, _) if name == "foo")
    ));
}

// ── Zenith-specific IR ────────────────────────────────────────────────────────

#[test]
fn test_quantum_circuit_generates_qalloc() {
    let module = gen_ir("quantum circuit Bell { let q = 1; }");
    assert!(
        has_instruction(&module, |i| matches!(i, IrInstruction::QAlloc(_, _))),
        "Expected QAlloc for quantum circuit"
    );
}

#[test]
fn test_nano_agent_generates_nanospawn() {
    let module = gen_ir("agent Scout { let x = 1; }");
    assert!(
        has_instruction(
            &module,
            |i| matches!(i, IrInstruction::NanoSpawn(_, name) if name == "Scout")
        ),
        "Expected NanoSpawn for agent"
    );
}

#[test]
fn test_sankofa_generates_store() {
    let module = gen_ir("remember mem_val = 42;");
    assert!(
        has_instruction(
            &module,
            |i| matches!(i, IrInstruction::SankofaStore(key, _) if key == "mem_val")
        ),
        "Expected SankofaStore for remember"
    );
}

// ── Ret instruction ───────────────────────────────────────────────────────────

#[test]
fn test_top_level_ends_with_ret() {
    let module = gen_ir("let x = 1;");
    let top = module
        .functions
        .iter()
        .find(|f| f.name == "__top__")
        .unwrap();
    assert!(
        matches!(top.body.last(), Some(IrInstruction::Ret(None))),
        "Top-level should end with Ret(None)"
    );
}

#[test]
fn test_instruction_count_nonzero() {
    let module = gen_ir("let a = 1; let b = 2; let c = a + b;");
    assert!(
        module.instruction_count() > 0,
        "Expected non-zero instructions"
    );
}
