//! Zenith Intermediate Representation Generator
//!
//! Translates the AST into a typed, SSA-like IR used by the optimizer
//! and all code-generation backends.

use crate::ast::*;
use crate::lexer::TokenType;
use std::collections::HashMap;

// ─── IR Types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Void,
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
    Ptr(Box<IrType>),
    Array(Box<IrType>, usize),
    Struct(String, Vec<(String, IrType)>),
    Function(Vec<IrType>, Box<IrType>),
    Quantum,
    Opaque(String),
}

impl IrType {
    pub fn from_ast_type(ty: &Type) -> Self {
        match ty {
            Type::Unit => IrType::Void,
            Type::Bool => IrType::Bool,
            Type::Int(w) => match w {
                IntWidth::I8 => IrType::I8,
                IntWidth::I16 => IrType::I16,
                IntWidth::I32 => IrType::I32,
                IntWidth::I64 | IntWidth::ISize => IrType::I64,
                IntWidth::I128 => IrType::I128,
            },
            Type::UInt(w) => match w {
                IntWidth::I8 => IrType::U8,
                IntWidth::I16 => IrType::U16,
                IntWidth::I32 => IrType::U32,
                IntWidth::I64 | IntWidth::ISize => IrType::U64,
                IntWidth::I128 => IrType::U128,
            },
            Type::Float(w) => match w {
                FloatWidth::F32 => IrType::F32,
                FloatWidth::F64 => IrType::F64,
            },
            Type::String | Type::Str => IrType::Ptr(Box::new(IrType::I8)),
            Type::Array(elem, sz) => {
                let n = sz.unwrap_or(0);
                IrType::Array(Box::new(IrType::from_ast_type(elem)), n)
            }
            Type::Named(n) => IrType::Opaque(n.clone()),
            Type::Quantum => IrType::Quantum,
            _ => IrType::Opaque("unknown".into()),
        }
    }

    pub fn ir_name(&self) -> String {
        match self {
            IrType::Void => "void".into(),
            IrType::Bool => "i1".into(),
            IrType::I8 => "i8".into(),
            IrType::I16 => "i16".into(),
            IrType::I32 => "i32".into(),
            IrType::I64 => "i64".into(),
            IrType::I128 => "i128".into(),
            IrType::U8 => "i8".into(),
            IrType::U16 => "i16".into(),
            IrType::U32 => "i32".into(),
            IrType::U64 => "i64".into(),
            IrType::U128 => "i128".into(),
            IrType::F32 => "float".into(),
            IrType::F64 => "double".into(),
            IrType::Ptr(inner) => format!("{}*", inner.ir_name()),
            IrType::Array(inner, n) => format!("[{} x {}]", n, inner.ir_name()),
            IrType::Struct(name, _) => format!("%struct.{}", name),
            IrType::Function(params, ret) => {
                let ps: Vec<String> = params.iter().map(|p| p.ir_name()).collect();
                format!("{} ({})", ret.ir_name(), ps.join(", "))
            }
            IrType::Quantum => "i64".into(),
            IrType::Opaque(n) => format!("%{}", n),
        }
    }
}

// ─── IR Values ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IrRegister(pub String, pub IrType);

impl IrRegister {
    pub fn new(name: impl Into<String>, ty: IrType) -> Self {
        IrRegister(name.into(), ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Reg(IrRegister),
    ConstInt(i64, IrType),
    ConstFloat(f64, IrType),
    ConstBool(bool),
    ConstStr(String),
    /// Address of a named global symbol (e.g. a string literal constant or
    /// a lifted lambda function). `len` is the byte length of the
    /// underlying `[len x i8]` global array (including any null
    /// terminator) when the global is a string constant; use 0 when the
    /// referenced global isn't an `[N x i8]` array (e.g. a function
    /// symbol), in which case the bare `@name` form is emitted instead of
    /// a `getelementptr`.
    GlobalPtr(String, usize),
    ConstNull,
    Void,
}

impl IrValue {
    pub fn ty(&self) -> IrType {
        match self {
            IrValue::Reg(r) => r.1.clone(),
            IrValue::ConstInt(_, t) => t.clone(),
            IrValue::ConstFloat(_, t) => t.clone(),
            IrValue::ConstBool(_) => IrType::Bool,
            IrValue::ConstStr(_) => IrType::Ptr(Box::new(IrType::I8)),
            IrValue::GlobalPtr(..) => IrType::Ptr(Box::new(IrType::I8)),
            IrValue::ConstNull => IrType::Ptr(Box::new(IrType::Void)),
            IrValue::Void => IrType::Void,
        }
    }

    pub fn to_ir_string(&self) -> String {
        match self {
            IrValue::Reg(r) => format!("%{}", r.0),
            IrValue::ConstInt(n, _) => n.to_string(),
            IrValue::ConstFloat(f, _) => format!("{:.6e}", f),
            IrValue::ConstBool(b) => {
                if *b {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            IrValue::ConstStr(s) => format!("c\"{}\\00\"", s),
            IrValue::GlobalPtr(name, len) => {
                if *len > 0 {
                    format!(
                        "getelementptr inbounds ([{len} x i8], [{len} x i8]* @{name}, i64 0, i64 0)",
                        len = len,
                        name = name
                    )
                } else {
                    format!("@{}", name)
                }
            }
            IrValue::ConstNull => "null".into(),
            IrValue::Void => "void".into(),
        }
    }
}

// ─── IR Instructions ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Stack-based allocation
    Alloca(IrRegister, IrType),
    Load(IrRegister, IrValue),
    Store(IrValue, IrValue),
    // Arithmetic
    Add(IrRegister, IrValue, IrValue),
    Sub(IrRegister, IrValue, IrValue),
    Mul(IrRegister, IrValue, IrValue),
    Div(IrRegister, IrValue, IrValue),
    Rem(IrRegister, IrValue, IrValue),
    Neg(IrRegister, IrValue),
    // Bitwise
    And(IrRegister, IrValue, IrValue),
    Or(IrRegister, IrValue, IrValue),
    Xor(IrRegister, IrValue, IrValue),
    Shl(IrRegister, IrValue, IrValue),
    Shr(IrRegister, IrValue, IrValue),
    Not(IrRegister, IrValue),
    // Comparison
    Cmp(IrRegister, CmpOp, IrValue, IrValue),
    // Control flow
    Label(String),
    Jump(String),
    CondJump(IrValue, String, String),
    Ret(Option<IrValue>),
    Unreachable,
    // Calls
    Call(Option<IrRegister>, String, Vec<IrValue>),
    CallIndirect(Option<IrRegister>, IrValue, Vec<IrValue>),
    // Memory
    GetElementPtr(IrRegister, IrValue, Vec<IrValue>),
    BitCast(IrRegister, IrValue, IrType),
    // Assignments
    Assign(IrRegister, IrValue),
    // Phi (SSA)
    Phi(IrRegister, Vec<(IrValue, String)>),
    // Conversions
    ZExt(IrRegister, IrValue, IrType),
    SExt(IrRegister, IrValue, IrType),
    Trunc(IrRegister, IrValue, IrType),
    FpExt(IrRegister, IrValue, IrType),
    FpTrunc(IrRegister, IrValue, IrType),
    SIToFP(IrRegister, IrValue, IrType),
    FPToSI(IrRegister, IrValue, IrType),
    // Zenith-specific
    QuantumGate(IrRegister, String, Vec<IrValue>),
    NanoOp(IrRegister, String, Vec<IrValue>),
    SankofaRecall(IrRegister, IrValue),
    SankofaRemember(String, IrValue),
    // Comments (preserved for readability)
    Comment(String),
}

impl IrInstruction {
    pub fn to_ir_string(&self) -> String {
        match self {
            IrInstruction::Alloca(r, ty) => format!("  %{} = alloca {}", r.0, ty.ir_name()),
            IrInstruction::Load(r, ptr) => format!(
                "  %{} = load {}, {}* {}",
                r.0,
                r.1.ir_name(),
                r.1.ir_name(),
                ptr.to_ir_string()
            ),
            IrInstruction::Store(val, ptr) => format!(
                "  store {} {}, {}* {}",
                val.ty().ir_name(),
                val.to_ir_string(),
                val.ty().ir_name(),
                ptr.to_ir_string()
            ),
            IrInstruction::Add(r, a, b) => format!(
                "  %{} = add {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Sub(r, a, b) => format!(
                "  %{} = sub {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Mul(r, a, b) => format!(
                "  %{} = mul {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Div(r, a, b) => format!(
                "  %{} = sdiv {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Rem(r, a, b) => format!(
                "  %{} = srem {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Neg(r, a) => format!(
                "  %{} = sub {} 0, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string()
            ),
            IrInstruction::And(r, a, b) => format!(
                "  %{} = and {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Or(r, a, b) => format!(
                "  %{} = or {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Xor(r, a, b) => format!(
                "  %{} = xor {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Shl(r, a, b) => format!(
                "  %{} = shl {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Shr(r, a, b) => format!(
                "  %{} = ashr {} {}, {}",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string(),
                b.to_ir_string()
            ),
            IrInstruction::Not(r, a) => format!(
                "  %{} = xor {} {}, -1",
                r.0,
                a.ty().ir_name(),
                a.to_ir_string()
            ),
            IrInstruction::Cmp(r, op, a, b) => {
                let pred = match op {
                    CmpOp::Eq => "eq",
                    CmpOp::Ne => "ne",
                    CmpOp::Lt => "slt",
                    CmpOp::Le => "sle",
                    CmpOp::Gt => "sgt",
                    CmpOp::Ge => "sge",
                    CmpOp::FLt => "olt",
                    CmpOp::FLe => "ole",
                    CmpOp::FGt => "ogt",
                    CmpOp::FGe => "oge",
                    CmpOp::FEq => "oeq",
                    CmpOp::FNe => "one",
                };
                let instr = if a.ty() == IrType::F64 || a.ty() == IrType::F32 {
                    "fcmp"
                } else {
                    "icmp"
                };
                format!(
                    "  %{} = {} {} {} {}, {}",
                    r.0,
                    instr,
                    pred,
                    a.ty().ir_name(),
                    a.to_ir_string(),
                    b.to_ir_string()
                )
            }
            IrInstruction::Label(l) => format!("{}:", l),
            IrInstruction::Jump(l) => format!("  br label %{}", l),
            IrInstruction::CondJump(cond, t, f) => format!(
                "  br i1 {}, label %{}, label %{}",
                cond.to_ir_string(),
                t,
                f
            ),
            IrInstruction::Ret(None) => "  ret void".into(),
            IrInstruction::Ret(Some(v)) => {
                format!("  ret {} {}", v.ty().ir_name(), v.to_ir_string())
            }
            IrInstruction::Unreachable => "  unreachable".into(),
            IrInstruction::Call(reg, func, args) => {
                let args_str: Vec<String> = args
                    .iter()
                    .map(|a| format!("{} {}", a.ty().ir_name(), a.to_ir_string()))
                    .collect();
                match reg {
                    Some(r) => format!(
                        "  %{} = call {} @{}({})",
                        r.0,
                        r.1.ir_name(),
                        func,
                        args_str.join(", ")
                    ),
                    None => format!("  call void @{}({})", func, args_str.join(", ")),
                }
            }
            IrInstruction::Assign(r, v) => format!("  %{} = {}", r.0, v.to_ir_string()),
            IrInstruction::Phi(r, incoming) => {
                let pairs: Vec<String> = incoming
                    .iter()
                    .map(|(v, l)| format!("[{}, %{}]", v.to_ir_string(), l))
                    .collect();
                format!("  %{} = phi {} {}", r.0, r.1.ir_name(), pairs.join(", "))
            }
            IrInstruction::ZExt(r, v, t) => format!(
                "  %{} = zext {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::SExt(r, v, t) => format!(
                "  %{} = sext {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::Trunc(r, v, t) => format!(
                "  %{} = trunc {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::SIToFP(r, v, t) => format!(
                "  %{} = sitofp {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::FPToSI(r, v, t) => format!(
                "  %{} = fptosi {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::BitCast(r, v, t) => format!(
                "  %{} = bitcast {} {} to {}",
                r.0,
                v.ty().ir_name(),
                v.to_ir_string(),
                t.ir_name()
            ),
            IrInstruction::GetElementPtr(r, base, indices) => {
                let idx_str: Vec<String> = indices
                    .iter()
                    .map(|i| format!("{} {}", i.ty().ir_name(), i.to_ir_string()))
                    .collect();
                format!(
                    "  %{} = getelementptr {}, {}* {}, {}",
                    r.0,
                    base.ty().ir_name(),
                    base.ty().ir_name(),
                    base.to_ir_string(),
                    idx_str.join(", ")
                )
            }
            IrInstruction::QuantumGate(r, gate, args) => {
                format!(
                    "  ; quantum_gate {} {} [{}]",
                    gate,
                    r.0,
                    args.iter()
                        .map(|a| a.to_ir_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrInstruction::NanoOp(r, op, args) => {
                format!(
                    "  ; nano_op {} {} [{}]",
                    op,
                    r.0,
                    args.iter()
                        .map(|a| a.to_ir_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrInstruction::SankofaRecall(r, domain) => {
                format!("  ; sankofa_recall {} from {}", r.0, domain.to_ir_string())
            }
            IrInstruction::SankofaRemember(name, val) => {
                format!("  ; sankofa_remember {} = {}", name, val.to_ir_string())
            }
            IrInstruction::Comment(c) => format!("  ; {}", c),
            _ => format!("  ; <unimplemented ir instruction>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    FEq,
    FNe,
    FLt,
    FLe,
    FGt,
    FGe,
}

// ─── IR Function ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub body: Vec<IrInstruction>,
    pub is_external: bool,
}

impl IrFunction {
    pub fn new(
        name: impl Into<String>,
        params: Vec<(String, IrType)>,
        return_type: IrType,
    ) -> Self {
        IrFunction {
            name: name.into(),
            params,
            return_type,
            body: vec![],
            is_external: false,
        }
    }

    pub fn push(&mut self, ins: IrInstruction) {
        self.body.push(ins);
    }

    pub fn to_ir_string(&self) -> String {
        if self.is_external {
            let params: Vec<String> = self.params.iter().map(|(_, t)| t.ir_name()).collect();
            return format!(
                "declare {} @{}({})",
                self.return_type.ir_name(),
                self.name,
                params.join(", ")
            );
        }
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("{} %{}", t.ir_name(), n))
            .collect();
        let mut out = format!(
            "define {} @{}({}) {{\n",
            self.return_type.ir_name(),
            self.name,
            params.join(", ")
        );
        for inst in &self.body {
            out.push_str(&inst.to_ir_string());
            out.push('\n');
        }
        out.push_str("}\n");
        out
    }
}

// ─── IR Module ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IrGlobal {
    pub name: String,
    pub ty: IrType,
    pub value: IrValue,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct IrModule {
    pub name: String,
    pub functions: Vec<IrFunction>,
    pub globals: Vec<IrGlobal>,
    pub type_defs: Vec<(String, Vec<(String, IrType)>)>,
    pub string_literals: Vec<(String, String)>,
    pub target_triple: String,
    pub data_layout: String,
}

impl IrModule {
    pub fn new(name: impl Into<String>) -> Self {
        IrModule {
            name: name.into(),
            functions: vec![],
            globals: vec![],
            type_defs: vec![],
            string_literals: vec![],
            target_triple: "x86_64-pc-linux-gnu".into(),
            data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
                .into(),
        }
    }

    pub fn add_function(&mut self, f: IrFunction) {
        self.functions.push(f);
    }

    pub fn add_global(&mut self, g: IrGlobal) {
        self.globals.push(g);
    }

    /// Total number of IR instructions across all functions in this module.
    pub fn instruction_count(&self) -> usize {
        self.functions.iter().map(|f| f.body.len()).sum()
    }

    pub fn to_ir_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("; Zenith LLVM IR — module: {}\n", self.name));
        out.push_str(&format!("target triple = \"{}\"\n", self.target_triple));
        out.push_str(&format!("target datalayout = \"{}\"\n\n", self.data_layout));

        // String constants
        for (name, s) in &self.string_literals {
            out.push_str(&format!(
                "@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n",
                name,
                s.len() + 1,
                s
            ));
        }
        if !self.string_literals.is_empty() {
            out.push('\n');
        }

        // Globals
        for g in &self.globals {
            let kw = if g.is_const { "constant" } else { "global" };
            out.push_str(&format!(
                "@{} = {} {} {}\n",
                g.name,
                kw,
                g.ty.ir_name(),
                g.value.to_ir_string()
            ));
        }
        if !self.globals.is_empty() {
            out.push('\n');
        }

        // Functions
        for f in &self.functions {
            out.push_str(&f.to_ir_string());
            out.push('\n');
        }
        out
    }
}

// ─── IR Generator ────────────────────────────────────────────────────────────

pub struct IrGenerator {
    reg_counter: usize,
    label_counter: usize,
    env: HashMap<String, IrValue>,
    string_counter: usize,
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            reg_counter: 0,
            label_counter: 0,
            env: HashMap::new(),
            string_counter: 0,
        }
    }

    fn fresh_reg(&mut self, ty: IrType) -> IrRegister {
        let n = self.reg_counter;
        self.reg_counter += 1;
        IrRegister(format!("r{}", n), ty)
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        let n = self.label_counter;
        self.label_counter += 1;
        format!("{}{}", prefix, n)
    }

    pub fn generate(&mut self, program: &Program) -> IrModule {
        let mut module = IrModule::new("zenith_module");
        // Add runtime externs
        let mut ext_print = IrFunction::new(
            "zenith_println",
            vec![("s".into(), IrType::Ptr(Box::new(IrType::I8)))],
            IrType::Void,
        );
        ext_print.is_external = true;
        module.add_function(ext_print);

        let mut main_fn = IrFunction::new("main", vec![], IrType::I32);
        main_fn.push(IrInstruction::Comment("Zenith entry point".into()));

        for stmt in &program.statements {
            self.emit_statement(stmt, &mut main_fn, &mut module);
        }
        main_fn.push(IrInstruction::Ret(Some(IrValue::ConstInt(0, IrType::I32))));

        // Top-level `fn` declarations (handled in emit_statement's
        // Statement::Function arm) are added directly to `module` as they
        // are encountered, using their declared name. If the source
        // program itself declares a top-level `fn main(...)`, adding this
        // synthetic wrapper under the same name would produce two
        // functions called `main` in one module -- invalid/duplicate
        // symbols for every backend target. Resolve the collision instead
        // of silently emitting broken output:
        //   - if the synthetic wrapper has no real work in it beyond the
        //     entry comment (i.e. every top-level statement was itself an
        //     item declaration like `fn`/`struct`/etc., so nothing was
        //     appended to its body), just drop it and let the
        //     user-defined `main` be the sole entry point.
        //   - if the synthetic wrapper DOES carry real top-level
        //     statements (loose code outside of any function) as well as
        //     a user-defined `main`, keep both by renaming the synthetic
        //     wrapper to `__zenith_start` rather than dropping code.
        let user_defined_main = module.functions.iter().any(|f| f.name == "main");
        let synthetic_is_empty = main_fn.body.len() <= 2; // entry comment + trailing ret
        if user_defined_main {
            if synthetic_is_empty {
                // Nothing of value in the synthetic wrapper; the user's
                // own `main` is the entry point.
            } else {
                main_fn.name = "__zenith_start".to_string();
                module.add_function(main_fn);
            }
        } else {
            module.add_function(main_fn);
        }
        module
    }

    fn emit_statement(&mut self, stmt: &Statement, func: &mut IrFunction, module: &mut IrModule) {
        match stmt {
            Statement::Let(_, name, _, expr) | Statement::Const(_, name, _, expr) => {
                let val = self.emit_expression(expr, func, module);
                self.env.insert(name.clone(), val);
            }

            Statement::Return(_, expr) => {
                let val = self.emit_expression(expr, func, module);
                func.push(IrInstruction::Ret(Some(val)));
            }

            Statement::Expression(expr) => {
                self.emit_expression(expr, func, module);
            }

            Statement::Function(_, name, params, ret_ann, body) => {
                let param_types: Vec<(String, IrType)> = params
                    .iter()
                    .map(|p| (p.name.0.clone(), IrType::I64))
                    .collect();
                let ret_ty = IrType::I64;
                let mut f = IrFunction::new(name, param_types.clone(), ret_ty.clone());
                let old_env = self.env.clone();
                for (pname, pty) in &param_types {
                    self.env.insert(
                        pname.clone(),
                        IrValue::Reg(IrRegister(pname.clone(), pty.clone())),
                    );
                }
                let body_val = self.emit_expression(body, &mut f, module);
                match &f.body.last() {
                    Some(IrInstruction::Ret(_)) => {}
                    _ => {
                        if body_val != IrValue::Void {
                            f.push(IrInstruction::Ret(Some(body_val)));
                        } else {
                            f.push(IrInstruction::Ret(None));
                        }
                    }
                }
                self.env = old_env;
                module.add_function(f);
            }

            Statement::While(_, cond, body) => {
                let lbl_cond = self.fresh_label("while_cond");
                let lbl_body = self.fresh_label("while_body");
                let lbl_exit = self.fresh_label("while_exit");
                func.push(IrInstruction::Jump(lbl_cond.clone()));
                func.push(IrInstruction::Label(lbl_cond.clone()));
                let cv = self.emit_expression(cond, func, module);
                func.push(IrInstruction::CondJump(
                    cv,
                    lbl_body.clone(),
                    lbl_exit.clone(),
                ));
                func.push(IrInstruction::Label(lbl_body.clone()));
                self.emit_expression(body, func, module);
                func.push(IrInstruction::Jump(lbl_cond.clone()));
                func.push(IrInstruction::Label(lbl_exit.clone()));
            }

            Statement::For(_, var, iter, body) => {
                let iter_val = self.emit_expression(iter, func, module);
                let idx_reg = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::Assign(
                    idx_reg.clone(),
                    IrValue::ConstInt(0, IrType::I64),
                ));
                let lbl_cond = self.fresh_label("for_cond");
                let lbl_body = self.fresh_label("for_body");
                let lbl_exit = self.fresh_label("for_exit");
                func.push(IrInstruction::Jump(lbl_cond.clone()));
                func.push(IrInstruction::Label(lbl_cond.clone()));
                // Simplified: emit body (full index-based iteration needs type info)
                func.push(IrInstruction::Label(lbl_body.clone()));
                let old = self.env.get(&var.0).cloned();
                self.env
                    .insert(var.0.clone(), IrValue::Reg(idx_reg.clone()));
                self.emit_expression(body, func, module);
                if let Some(old_val) = old {
                    self.env.insert(var.0.clone(), old_val);
                } else {
                    self.env.remove(&var.0);
                }
                func.push(IrInstruction::Label(lbl_exit.clone()));
                let _ = iter_val;
            }

            Statement::Match(_, expr, cases) => {
                let val = self.emit_expression(expr, func, module);
                let lbl_exit = self.fresh_label("match_exit");
                let mut arm_labels: Vec<String> = cases
                    .iter()
                    .enumerate()
                    .map(|(i, _)| self.fresh_label(&format!("match_arm{}", i)))
                    .collect();
                for (i, case) in cases.iter().enumerate() {
                    func.push(IrInstruction::Label(arm_labels[i].clone()));
                    self.emit_expression(&case.body, func, module);
                    func.push(IrInstruction::Jump(lbl_exit.clone()));
                }
                func.push(IrInstruction::Label(lbl_exit.clone()));
                let _ = val;
            }

            Statement::Struct(_, name, _, fields) => {
                let field_types: Vec<(String, IrType)> = fields
                    .iter()
                    .map(|f| (f.name.0.clone(), IrType::I64))
                    .collect();
                module.type_defs.push((name.0.clone(), field_types));
            }

            Statement::Break(_) => {
                // Break handled at while/for level via label jumping
            }

            Statement::Continue(_) => {}

            Statement::SankofaMemory(_, name, expr) => {
                let val = self.emit_expression(expr, func, module);
                func.push(IrInstruction::SankofaRemember(name.clone(), val));
            }

            Statement::Module(_, _name, stmts) => {
                for s in stmts {
                    self.emit_statement(s, func, module);
                }
            }

            Statement::Import(_, _) | Statement::Use(_, _) => {}

            Statement::QuantumCircuit(_, name, body) => {
                let r = self.fresh_reg(IrType::Quantum);
                let args = vec![];
                func.push(IrInstruction::QuantumGate(r.clone(), name.clone(), args));
                self.env.insert(name.clone(), IrValue::Reg(r));
                self.emit_expression(body, func, module);
            }

            Statement::NanoAgent(_, name, body) => {
                let r = self.fresh_reg(IrType::Opaque("NanoAgent".into()));
                func.push(IrInstruction::NanoOp(r.clone(), name.clone(), vec![]));
                self.env.insert(name.clone(), IrValue::Reg(r));
                self.emit_expression(body, func, module);
            }

            Statement::Wisdom(_, name, expr) => {
                let val = self.emit_expression(expr, func, module);
                module.add_global(IrGlobal {
                    name: name.clone(),
                    ty: val.ty(),
                    value: val,
                    is_const: true,
                });
            }

            _ => {
                func.push(IrInstruction::Comment(format!(
                    "unimplemented stmt: {:?}",
                    std::mem::discriminant(stmt)
                )));
            }
        }
    }

    fn emit_expression(
        &mut self,
        expr: &Expression,
        func: &mut IrFunction,
        module: &mut IrModule,
    ) -> IrValue {
        match expr {
            Expression::Literal(lit) => self.emit_literal(lit, func, module),

            Expression::Identifier(id) => self
                .env
                .get(&id.0)
                .cloned()
                .unwrap_or(IrValue::ConstInt(0, IrType::I64)),

            Expression::Infix(_, left, op, right) => {
                let lv = self.emit_expression(left, func, module);
                let rv = self.emit_expression(right, func, module);
                self.emit_binop(op, lv, rv, func)
            }

            Expression::Prefix(_, op, inner) => {
                let v = self.emit_expression(inner, func, module);
                match op {
                    TokenType::Minus => {
                        let r = self.fresh_reg(v.ty());
                        func.push(IrInstruction::Neg(r.clone(), v));
                        IrValue::Reg(r)
                    }
                    TokenType::Not => {
                        let r = self.fresh_reg(IrType::Bool);
                        func.push(IrInstruction::Not(r.clone(), v));
                        IrValue::Reg(r)
                    }
                    _ => v,
                }
            }

            Expression::If(_, cond, then, else_) => {
                let cond_val = self.emit_expression(cond, func, module);
                let lbl_then = self.fresh_label("if_then");
                let lbl_else = self.fresh_label("if_else");
                let lbl_end = self.fresh_label("if_end");
                let has_else = else_.is_some();

                func.push(IrInstruction::CondJump(
                    cond_val,
                    lbl_then.clone(),
                    if has_else {
                        lbl_else.clone()
                    } else {
                        lbl_end.clone()
                    },
                ));

                func.push(IrInstruction::Label(lbl_then.clone()));
                let then_val = self.emit_expression(then, func, module);
                func.push(IrInstruction::Jump(lbl_end.clone()));

                if let Some(else_expr) = else_ {
                    func.push(IrInstruction::Label(lbl_else.clone()));
                    let _else_val = self.emit_expression(else_expr, func, module);
                    func.push(IrInstruction::Jump(lbl_end.clone()));
                }

                func.push(IrInstruction::Label(lbl_end.clone()));
                then_val
            }

            Expression::Block(_, stmts) => {
                let mut last = IrValue::Void;
                for (i, s) in stmts.iter().enumerate() {
                    if i == stmts.len() - 1 {
                        if let Statement::Expression(e) = s {
                            last = self.emit_expression(e, func, module);
                        } else {
                            self.emit_statement(s, func, module);
                        }
                    } else {
                        self.emit_statement(s, func, module);
                    }
                }
                last
            }

            Expression::Call(_, callee, args) => {
                let arg_vals: Vec<IrValue> = args
                    .iter()
                    .map(|a| self.emit_expression(a, func, module))
                    .collect();
                if let Expression::Identifier(id) = callee.as_ref() {
                    let ret_reg = self.fresh_reg(IrType::I64);
                    if id.0 == "println" || id.0 == "print" {
                        func.push(IrInstruction::Call(None, "zenith_println".into(), arg_vals));
                        return IrValue::Void;
                    }
                    func.push(IrInstruction::Call(
                        Some(ret_reg.clone()),
                        id.0.clone(),
                        arg_vals,
                    ));
                    IrValue::Reg(ret_reg)
                } else {
                    let fn_val = self.emit_expression(callee, func, module);
                    let ret_reg = self.fresh_reg(IrType::I64);
                    func.push(IrInstruction::CallIndirect(
                        Some(ret_reg.clone()),
                        fn_val,
                        arg_vals,
                    ));
                    IrValue::Reg(ret_reg)
                }
            }

            Expression::Lambda(_, params, body) => {
                let fn_name = format!("lambda_{}", self.reg_counter);
                self.reg_counter += 1;
                let param_types: Vec<(String, IrType)> = params
                    .iter()
                    .map(|p| (p.name.0.clone(), IrType::I64))
                    .collect();
                let mut lf = IrFunction::new(&fn_name, param_types.clone(), IrType::I64);
                let old_env = self.env.clone();
                for (n, t) in &param_types {
                    self.env
                        .insert(n.clone(), IrValue::Reg(IrRegister(n.clone(), t.clone())));
                }
                let body_val = self.emit_expression(body, &mut lf, module);
                lf.push(IrInstruction::Ret(Some(body_val)));
                self.env = old_env;
                module.add_function(lf);
                IrValue::GlobalPtr(fn_name, 0)
            }

            Expression::Array(_, elems) => {
                for e in elems {
                    self.emit_expression(e, func, module);
                }
                let r = self.fresh_reg(IrType::Ptr(Box::new(IrType::I64)));
                func.push(IrInstruction::Alloca(
                    r.clone(),
                    IrType::Array(Box::new(IrType::I64), elems.len()),
                ));
                IrValue::Reg(r)
            }

            Expression::Assign(_, target, value) => {
                let val = self.emit_expression(value, func, module);
                if let Expression::Identifier(id) = target.as_ref() {
                    self.env.insert(id.0.clone(), val.clone());
                }
                val
            }

            Expression::CompoundAssign(_, target, op, value) => {
                let cur = self.emit_expression(target, func, module);
                let val = self.emit_expression(value, func, module);
                let result = self.emit_binop(op, cur, val, func);
                if let Expression::Identifier(id) = target.as_ref() {
                    self.env.insert(id.0.clone(), result.clone());
                }
                result
            }

            Expression::MemberAccess(_, obj, _field) => {
                let _obj_val = self.emit_expression(obj, func, module);
                let r = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::Comment(format!(
                    "member access .{}",
                    _field.0
                )));
                IrValue::Reg(r)
            }

            Expression::MethodCall(_, obj, method, args) => {
                let obj_val = self.emit_expression(obj, func, module);
                let mut arg_vals = vec![obj_val];
                for a in args {
                    arg_vals.push(self.emit_expression(a, func, module));
                }
                let r = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::Call(
                    Some(r.clone()),
                    method.0.clone(),
                    arg_vals,
                ));
                IrValue::Reg(r)
            }

            Expression::Cast(_, inner, _target_ty) => {
                let v = self.emit_expression(inner, func, module);
                let r = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::BitCast(r.clone(), v, IrType::I64));
                IrValue::Reg(r)
            }

            Expression::Await(_, inner) => self.emit_expression(inner, func, module),

            Expression::Async(_, body) => self.emit_expression(body, func, module),

            Expression::Spawn(_, body) => self.emit_expression(body, func, module),

            Expression::New(_, name, args) => {
                let arg_vals: Vec<IrValue> = args
                    .iter()
                    .map(|a| self.emit_expression(a, func, module))
                    .collect();
                let r = self.fresh_reg(IrType::Opaque(name.0.clone()));
                func.push(IrInstruction::Call(
                    Some(r.clone()),
                    format!("{}_new", name.0),
                    arg_vals,
                ));
                IrValue::Reg(r)
            }

            Expression::Index(_, base, idx) => {
                let base_val = self.emit_expression(base, func, module);
                let idx_val = self.emit_expression(idx, func, module);
                let r = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::GetElementPtr(
                    r.clone(),
                    base_val,
                    vec![idx_val],
                ));
                IrValue::Reg(r)
            }

            Expression::Range(_, start, end, _) => {
                let sv = self.emit_expression(start, func, module);
                let ev = self.emit_expression(end, func, module);
                let r = self.fresh_reg(IrType::Opaque("Range".into()));
                func.push(IrInstruction::Comment(format!(
                    "range {}..{}",
                    sv.to_ir_string(),
                    ev.to_ir_string()
                )));
                IrValue::Reg(r)
            }

            Expression::Recall(_, domain) => {
                let dv = self.emit_expression(domain, func, module);
                let r = self.fresh_reg(IrType::Opaque("Memory".into()));
                func.push(IrInstruction::SankofaRecall(r.clone(), dv));
                IrValue::Reg(r)
            }

            Expression::Remember(_, name, val) => {
                let v = self.emit_expression(val, func, module);
                func.push(IrInstruction::SankofaRemember(name.clone(), v));
                IrValue::Void
            }

            Expression::Learn(_, data) => {
                let dv = self.emit_expression(data, func, module);
                let r = self.fresh_reg(IrType::Opaque("Knowledge".into()));
                func.push(IrInstruction::NanoOp(r.clone(), "learn".into(), vec![dv]));
                IrValue::Reg(r)
            }

            Expression::Perform(_, effect) => {
                let ev = self.emit_expression(effect, func, module);
                let r = self.fresh_reg(IrType::Opaque("Effect".into()));
                func.push(IrInstruction::NanoOp(r.clone(), "perform".into(), vec![ev]));
                IrValue::Reg(r)
            }

            Expression::Zamani(_, body) => {
                let bv = self.emit_expression(body, func, module);
                let r = self.fresh_reg(IrType::Opaque("Memory".into()));
                func.push(IrInstruction::Comment("zamani (past) block".into()));
                func.push(IrInstruction::SankofaRemember(
                    "__zamani__".into(),
                    bv.clone(),
                ));
                let _ = r;
                bv
            }

            Expression::Sasa(_, body) => {
                let bv = self.emit_expression(body, func, module);
                func.push(IrInstruction::Comment("sasa (present) block".into()));
                bv
            }

            Expression::QuantumOp(_, gate, args) => {
                let arg_vals: Vec<IrValue> = args
                    .iter()
                    .map(|a| self.emit_expression(a, func, module))
                    .collect();
                let r = self.fresh_reg(IrType::Quantum);
                func.push(IrInstruction::QuantumGate(
                    r.clone(),
                    gate.clone(),
                    arg_vals,
                ));
                IrValue::Reg(r)
            }

            Expression::NanoOp(_, op, args) => {
                let arg_vals: Vec<IrValue> = args
                    .iter()
                    .map(|a| self.emit_expression(a, func, module))
                    .collect();
                let r = self.fresh_reg(IrType::Opaque("NanoResult".into()));
                func.push(IrInstruction::NanoOp(r.clone(), op.clone(), arg_vals));
                IrValue::Reg(r)
            }

            Expression::Match(_, expr, cases) => {
                let val = self.emit_expression(expr, func, module);
                let lbl_exit = self.fresh_label("match_exit");
                for case in cases {
                    let lbl = self.fresh_label("match_arm");
                    func.push(IrInstruction::Label(lbl));
                    self.emit_expression(&case.body, func, module);
                    func.push(IrInstruction::Jump(lbl_exit.clone()));
                }
                func.push(IrInstruction::Label(lbl_exit));
                let _ = val;
                IrValue::Void
            }

            Expression::Try(_, inner) => self.emit_expression(inner, func, module),

            Expression::TryCatch(_, body, arms) => {
                let body_val = self.emit_expression(body, func, module);
                for arm in arms {
                    self.emit_expression(&arm.body, func, module);
                }
                body_val
            }

            Expression::Struct(_, name, fields) => {
                for (_, val) in fields {
                    self.emit_expression(val, func, module);
                }
                let r = self.fresh_reg(IrType::Opaque(name.0.clone()));
                func.push(IrInstruction::Comment(format!("struct literal {}", name.0)));
                IrValue::Reg(r)
            }

            Expression::Tuple(_, elems) => {
                for e in elems {
                    self.emit_expression(e, func, module);
                }
                let r = self.fresh_reg(IrType::I64);
                IrValue::Reg(r)
            }

            Expression::Loop(_, body) => {
                let lbl = self.fresh_label("loop");
                func.push(IrInstruction::Label(lbl.clone()));
                self.emit_expression(body, func, module);
                func.push(IrInstruction::Jump(lbl));
                IrValue::Void
            }

            Expression::Macro(_, name, args) => {
                let r = self.fresh_reg(IrType::I64);
                func.push(IrInstruction::Comment(format!(
                    "macro {}!({})",
                    name,
                    args.len()
                )));
                IrValue::Reg(r)
            }

            Expression::TypeAscription(_, inner, _) => self.emit_expression(inner, func, module),
        }
    }

    fn emit_literal(
        &mut self,
        lit: &Literal,
        func: &mut IrFunction,
        module: &mut IrModule,
    ) -> IrValue {
        match lit {
            Literal::Integer(n, _) => IrValue::ConstInt(*n, IrType::I64),
            Literal::Float(f, _) => IrValue::ConstFloat(*f, IrType::F64),
            Literal::Boolean(b, _) => IrValue::ConstBool(*b),
            Literal::String(s, _) => {
                let name = format!("str{}", self.string_counter);
                self.string_counter += 1;
                module.string_literals.push((name.clone(), s.clone()));
                // +1 accounts for the NUL terminator emitted alongside the
                // literal by IrModule::to_ir_string()'s global definition.
                IrValue::GlobalPtr(name, s.len() + 1)
            }
            Literal::Char(c, _) => IrValue::ConstInt(*c as i64, IrType::I8),
            Literal::Null(_) | Literal::Unit(_) => IrValue::Void,
            Literal::Quantum(q, _) => {
                let r = self.fresh_reg(IrType::Quantum);
                func.push(IrInstruction::Comment(format!("quantum literal {}", q)));
                IrValue::Reg(r)
            }
            Literal::Nano(n, _) => {
                let r = self.fresh_reg(IrType::Opaque("Nano".into()));
                func.push(IrInstruction::Comment(format!("nano literal {}", n)));
                IrValue::Reg(r)
            }
            Literal::MTS(m, _) => {
                let r = self.fresh_reg(IrType::Opaque("MTS".into()));
                func.push(IrInstruction::Comment(format!("mts literal {}", m)));
                IrValue::Reg(r)
            }
        }
    }

    fn emit_binop(
        &mut self,
        op: &TokenType,
        lv: IrValue,
        rv: IrValue,
        func: &mut IrFunction,
    ) -> IrValue {
        let ty = lv.ty();
        let r = self.fresh_reg(ty.clone());
        let instr = match op {
            TokenType::Plus => IrInstruction::Add(r.clone(), lv, rv),
            TokenType::Minus => IrInstruction::Sub(r.clone(), lv, rv),
            TokenType::Star => IrInstruction::Mul(r.clone(), lv, rv),
            TokenType::Slash => IrInstruction::Div(r.clone(), lv, rv),
            TokenType::Modulo => IrInstruction::Rem(r.clone(), lv, rv),
            TokenType::Equals => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Eq, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::NotEquals => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Ne, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::LessThan => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Lt, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::LessThanEqual => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Le, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::GreaterThan => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Gt, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::GreaterThanEqual => {
                let cr = self.fresh_reg(IrType::Bool);
                func.push(IrInstruction::Cmp(cr.clone(), CmpOp::Ge, lv, rv));
                return IrValue::Reg(cr);
            }
            TokenType::LogicalAnd | TokenType::KeywordAnd => IrInstruction::And(r.clone(), lv, rv),
            TokenType::LogicalOr | TokenType::KeywordOr => IrInstruction::Or(r.clone(), lv, rv),
            TokenType::BitAnd | TokenType::Ampersand => IrInstruction::And(r.clone(), lv, rv),
            TokenType::Pipe => IrInstruction::Or(r.clone(), lv, rv),
            TokenType::Caret => IrInstruction::Xor(r.clone(), lv, rv),
            TokenType::LeftShift => IrInstruction::Shl(r.clone(), lv, rv),
            TokenType::RightShift => IrInstruction::Shr(r.clone(), lv, rv),
            _ => IrInstruction::Comment(format!("unknown binop {:?}", op)),
        };
        func.push(instr);
        IrValue::Reg(r)
    }
}

impl Default for IrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
