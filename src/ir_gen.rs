//! Zenith UMC Intermediate Representation Generator
//!
//! Translates the typed AST into a platform-agnostic IR module ready for
//! optimization and backend code generation.

use crate::ast::{Expression, Literal, Program, Statement};
use std::collections::HashMap;

// ─── IR types ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Unit,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    F128,
    Ptr(Box<IrType>),
    Array(Box<IrType>, usize),
    Struct(String),
    Quantum,
    Nano,
    MTS,
    Sankofa,
    Function(Vec<IrType>, Box<IrType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Reg(IrRegister),
    ConstInt(i64),
    ConstFloat(f64),
    ConstBool(bool),
    ConstStr(String),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrRegister(pub String);

impl IrRegister {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Control
    Label(String),
    Jump(String),
    CondJump(IrValue, String, String),
    Ret(Option<IrValue>),

    // Variables
    Alloc(IrRegister, IrType),
    Assign(IrRegister, IrValue),
    Load(IrRegister, IrValue),
    Store(IrValue, IrValue),

    // Arithmetic
    Add(IrRegister, IrValue, IrValue),
    Sub(IrRegister, IrValue, IrValue),
    Mul(IrRegister, IrValue, IrValue),
    Div(IrRegister, IrValue, IrValue),
    Mod(IrRegister, IrValue, IrValue),

    // Comparison
    CmpEq(IrRegister, IrValue, IrValue),
    CmpNeq(IrRegister, IrValue, IrValue),
    CmpLt(IrRegister, IrValue, IrValue),
    CmpGt(IrRegister, IrValue, IrValue),
    CmpLe(IrRegister, IrValue, IrValue),
    CmpGe(IrRegister, IrValue, IrValue),

    // Logic
    And(IrRegister, IrValue, IrValue),
    Or(IrRegister, IrValue, IrValue),
    Not(IrRegister, IrValue),

    // Functions
    Call(IrRegister, String, Vec<IrValue>),
    CallVoid(String, Vec<IrValue>),

    // Quantum
    QAlloc(IrRegister, u32),
    QGate(String, Vec<IrRegister>),
    QMeasure(IrRegister, IrRegister),

    // Nano
    NanoSpawn(IrRegister, String),
    NanoSend(IrRegister, IrValue),

    // MTS
    MTSSnapshot(IrRegister),
    MTSRestore(IrRegister),

    // Sankofa
    SankofaStore(String, IrValue),
    SankofaRecall(IrRegister, String),

    // Memory
    Phi(IrRegister, Vec<(IrValue, String)>),
    Nop,
}

// ─── IR Function / Module ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub body: Vec<IrInstruction>,
}

impl IrFunction {
    pub fn new(name: impl Into<String>, ret: IrType) -> Self {
        IrFunction {
            name: name.into(),
            params: vec![],
            return_type: ret,
            body: vec![],
        }
    }

    pub fn push(&mut self, ins: IrInstruction) {
        self.body.push(ins);
    }
}

#[derive(Debug, Clone, Default)]
pub struct IrModule {
    pub functions: Vec<IrFunction>,
    pub globals: HashMap<String, IrValue>,
    pub types: HashMap<String, IrType>,
}

impl IrModule {
    pub fn new() -> Self {
        IrModule::default()
    }
    pub fn add_function(&mut self, f: IrFunction) {
        self.functions.push(f);
    }
    pub fn instruction_count(&self) -> usize {
        self.functions.iter().map(|f| f.body.len()).sum()
    }
}

// ─── IR generator ────────────────────────────────────────────────────────────

pub struct IrGenerator {
    pub module: IrModule,
    reg_counter: usize,
    label_counter: usize,
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            module: IrModule::new(),
            reg_counter: 0,
            label_counter: 0,
        }
    }

    fn fresh_reg(&mut self) -> IrRegister {
        let r = IrRegister(format!("%r{}", self.reg_counter));
        self.reg_counter += 1;
        r
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        let l = format!("{}{}", prefix, self.label_counter);
        self.label_counter += 1;
        l
    }

    pub fn generate(&mut self, program: &Program) -> IrModule {
        let mut top = IrFunction::new("__top__", IrType::Unit);
        for stmt in &program.statements {
            self.emit_statement(stmt, &mut top);
        }
        top.push(IrInstruction::Ret(None));
        self.module.add_function(top);
        self.module.clone()
    }

    fn emit_statement(&mut self, stmt: &Statement, func: &mut IrFunction) {
        match stmt {
            Statement::Let(_, name, _, expr) => {
                let val = self.emit_expression(expr, func);
                let reg = IrRegister(format!("%{}", name));
                func.push(IrInstruction::Assign(reg, val));
            }
            Statement::Return(_, expr) => {
                let val = self.emit_expression(expr, func);
                func.push(IrInstruction::Ret(Some(val)));
            }
            Statement::Expression(expr) => {
                self.emit_expression(expr, func);
            }
            Statement::Function(_, name, params, ret_ann, body) => {
                let mut f = IrFunction::new(name.clone(), IrType::Unit);
                for param in params {
                    f.params.push((param.name.0.clone(), IrType::I64));
                }
                self.emit_expression(body, &mut f);
                f.push(IrInstruction::Ret(None));
                self.module.add_function(f);
            }
            Statement::While(_, cond, body) => {
                let lbl_cond = self.fresh_label("while_cond");
                let lbl_body = self.fresh_label("while_body");
                let lbl_end = self.fresh_label("while_end");
                func.push(IrInstruction::Jump(lbl_cond.clone()));
                func.push(IrInstruction::Label(lbl_cond.clone()));
                let cv = self.emit_expression(cond, func);
                func.push(IrInstruction::CondJump(
                    cv,
                    lbl_body.clone(),
                    lbl_end.clone(),
                ));
                func.push(IrInstruction::Label(lbl_body));
                self.emit_expression(body, func);
                func.push(IrInstruction::Jump(lbl_cond));
                func.push(IrInstruction::Label(lbl_end));
            }
            Statement::QuantumCircuit(_, name, body) => {
                let reg = self.fresh_reg();
                func.push(IrInstruction::QAlloc(reg.clone(), 8));
                self.emit_expression(body, func);
            }
            Statement::NanoAgent(_, name, body) => {
                let reg = self.fresh_reg();
                func.push(IrInstruction::NanoSpawn(reg, name.clone()));
                self.emit_expression(body, func);
            }
            Statement::SankofaMemory(_, name, expr) => {
                let val = self.emit_expression(expr, func);
                func.push(IrInstruction::SankofaStore(name.clone(), val));
            }
            Statement::Match(_, subject, cases) => {
                let sv = self.emit_expression(subject, func);
                let end = self.fresh_label("match_end");
                for case in cases {
                    let lbl = self.fresh_label("case");
                    func.push(IrInstruction::Label(lbl));
                    self.emit_expression(&case.body, func);
                }
                func.push(IrInstruction::Label(end));
            }
            _ => {
                func.push(IrInstruction::Nop);
            }
        }
    }

    fn emit_expression(&mut self, expr: &Expression, func: &mut IrFunction) -> IrValue {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Integer(n, _) => IrValue::ConstInt(*n),
                Literal::Float(f, _) => IrValue::ConstFloat(*f),
                Literal::Boolean(b, _) => IrValue::ConstBool(*b),
                Literal::String(s, _) => IrValue::ConstStr(s.clone()),
                _ => IrValue::Null,
            },
            Expression::Identifier(id) => {
                let reg = self.fresh_reg();
                func.push(IrInstruction::Load(
                    reg.clone(),
                    IrValue::Reg(IrRegister(format!("%{}", id.0))),
                ));
                IrValue::Reg(reg)
            }
            Expression::Infix(_, left, op, right) => {
                let lv = self.emit_expression(left, func);
                let rv = self.emit_expression(right, func);
                let reg = self.fresh_reg();
                use crate::lexer::TokenType::*;
                let ins = match op {
                    Plus => IrInstruction::Add(reg.clone(), lv, rv),
                    Minus => IrInstruction::Sub(reg.clone(), lv, rv),
                    Star => IrInstruction::Mul(reg.clone(), lv, rv),
                    Slash => IrInstruction::Div(reg.clone(), lv, rv),
                    Modulo => IrInstruction::Mod(reg.clone(), lv, rv),
                    Equals => IrInstruction::CmpEq(reg.clone(), lv, rv),
                    NotEquals => IrInstruction::CmpNeq(reg.clone(), lv, rv),
                    LessThan => IrInstruction::CmpLt(reg.clone(), lv, rv),
                    GreaterThan => IrInstruction::CmpGt(reg.clone(), lv, rv),
                    LessThanEqual => IrInstruction::CmpLe(reg.clone(), lv, rv),
                    GreaterThanEqual => IrInstruction::CmpGe(reg.clone(), lv, rv),
                    LogicalAnd => IrInstruction::And(reg.clone(), lv, rv),
                    LogicalOr => IrInstruction::Or(reg.clone(), lv, rv),
                    _ => IrInstruction::Nop,
                };
                func.push(ins);
                IrValue::Reg(reg)
            }
            Expression::Prefix(_, op, operand) => {
                let ov = self.emit_expression(operand, func);
                let reg = self.fresh_reg();
                use crate::lexer::TokenType;
                let ins = match op {
                    TokenType::Not => IrInstruction::Not(reg.clone(), ov),
                    TokenType::Minus => IrInstruction::Sub(reg.clone(), IrValue::ConstInt(0), ov),
                    _ => IrInstruction::Nop,
                };
                func.push(ins);
                IrValue::Reg(reg)
            }
            Expression::Call(_, func_expr, args) => {
                let fn_name = match func_expr.as_ref() {
                    Expression::Identifier(id) => id.0.clone(),
                    _ => "__dynamic__".to_string(),
                };
                let arg_vals: Vec<IrValue> =
                    args.iter().map(|a| self.emit_expression(a, func)).collect();
                let reg = self.fresh_reg();
                func.push(IrInstruction::Call(reg.clone(), fn_name, arg_vals));
                IrValue::Reg(reg)
            }
            Expression::If(_, cond, then_branch, else_branch) => {
                let cv = self.emit_expression(cond, func);
                let then_lbl = self.fresh_label("then");
                let else_lbl = self.fresh_label("else");
                let end_lbl = self.fresh_label("endif");
                func.push(IrInstruction::CondJump(
                    cv,
                    then_lbl.clone(),
                    else_lbl.clone(),
                ));
                func.push(IrInstruction::Label(then_lbl));
                let then_val = self.emit_expression(then_branch, func);
                func.push(IrInstruction::Jump(end_lbl.clone()));
                func.push(IrInstruction::Label(else_lbl));
                if let Some(eb) = else_branch {
                    self.emit_expression(eb, func);
                }
                func.push(IrInstruction::Label(end_lbl));
                then_val
            }
            Expression::Block(_, stmts) => {
                let mut last = IrValue::Null;
                for stmt in stmts {
                    if let Statement::Expression(e) = stmt {
                        last = self.emit_expression(e, func);
                    } else {
                        self.emit_statement(stmt, func);
                    }
                }
                last
            }
            Expression::Recall(_, key) => {
                let key_str = match key.as_ref() {
                    Expression::Identifier(id) => id.0.clone(),
                    _ => "__key__".to_string(),
                };
                let reg = self.fresh_reg();
                func.push(IrInstruction::SankofaRecall(reg.clone(), key_str));
                IrValue::Reg(reg)
            }
            _ => IrValue::Null,
        }
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
