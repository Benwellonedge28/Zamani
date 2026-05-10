//! Zenith Universal Meta-Compiler (UMC) Intermediate Representation (IR) Generator
//!
//! This module implements the IR generation phase of the Zenith compiler.
//! It translates the semantically-validated Abstract Syntax Tree (AST) into
//! a high-level, machine-agnostic Universal Meta-Compiler Intermediate Representation (UMC IR).
//! The UMC IR is designed to support the diverse computational paradigms of Zenith,
//! including classical, quantum, nano, and multi-timeline systems, while facilitating
//! advanced optimizations and targeting various backends.
//!
//! Key responsibilities include:
//! - Traversing the AST: Visiting each node of the AST to generate corresponding IR instructions.
//! - Symbol to IR mapping: Translating high-level AST symbols (variables, functions, types)
//!   into low-level IR constructs (registers, memory locations, IR types).
//! - Control Flow Graph (CFG) generation: Representing program control flow using basic blocks
//!   and jump/branch instructions.
//! - Type translation: Mapping Zenith's rich type system (classical, quantum, nano, dependent, linear, etc.)
//!   into the UMC IR's unified type system.
//! - Handling special constructs: Generating specific IR for quantum operations (gates, measurements),
//!   nano-agent interactions, Sankofa memory operations, and effect handlers.
//! - Error reporting: Catching any inconsistencies or unhandled scenarios during IR generation.

use crate::ast::{Program, Statement, Expression, Literal, Identifier, TypeExpr, Parameter, MatchCase};
use crate::compiler_types::{Type, Symbol}; // From semantic analysis
use crate::tokens::Span; // For error reporting
use std::collections::{HashMap, VecDeque};

// --- UMC IR Instruction Set (Conceptual) ---
#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Control Flow
    Label(String),
    Jump(String),
    Branch(IrValue, String, String), // condition, true_label, false_label
    Call(IrValue, String, Vec<IrValue>), // result_reg, function_name, args
    Return(Option<IrValue>),
    Yield(IrValue), // For async/generators
    Await(IrValue, IrValue), // result_reg = await future_val

    // Memory Operations
    Alloc(IrRegister, IrType), // reg = allocate type
    Load(IrRegister, IrValue), // reg = load from address_val
    Store(IrValue, IrValue), // store value to address_val
    Global(IrRegister, String, IrType), // reg = global_var_name of type

    // Arithmetic / Logical Operations
    Add(IrRegister, IrValue, IrValue), // reg = op1 + op2
    Sub(IrRegister, IrValue, IrValue),
    Mul(IrRegister, IrValue, IrValue),
    Div(IrRegister, IrValue, IrValue),
    Mod(IrRegister, IrValue, IrValue),
    And(IrRegister, IrValue, IrValue),
    Or(IrRegister, IrValue, IrValue),
    Xor(IrRegister, IrValue, IrValue),
    Not(IrRegister, IrValue),
    Neg(IrRegister, IrValue),

    // Comparison Operations
    CmpEq(IrRegister, IrValue, IrValue), // reg = op1 == op2
    CmpNe(IrRegister, IrValue, IrValue),
    CmpLt(IrRegister, IrValue, IrValue),
    CmpLe(IrRegister, IrValue, IrValue),
    CmpGt(IrRegister, IrValue, IrValue),
    CmpGe(IrRegister, IrValue, IrValue),

    // Type Conversion
    Cast(IrRegister, IrValue, IrType), // reg = cast value to type

    // Special Zenith UMC IR
    QGate(IrRegister, String, Vec<IrValue>), // reg = gate_name(qubit_regs)
    QMeasure(IrRegister, IrValue), // classic_reg = measure qubit_reg
    QEntangle(IrValue, IrValue), // entangle q1, q2 (conceptual)
    QTeleport(IrValue, IrValue, IrValue), // teleport q_source, q_dest, classic_key (conceptual, might require proof)

    NanoOp(IrRegister, String, Vec<IrValue>), // reg = nano_instruction(args)
    MTSOp(IrRegister, String, Vec<IrValue>), // reg = mts_instruction(args)

    EffectOp(IrRegister, String, Vec<IrValue>), // reg = perform_effect(args)
    HandleEffect(String, String), // handle effect_name with handler_label

    NoOp, // Placeholder
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Register(IrRegister),
    Literal(Literal),
    Global(String), // Reference to a global variable/function
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrRegister(usize); // Virtual register ID

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    I32, F64, Bool, StringPtr, Char,
    Qubit, QubitArray(usize),
    NanoParticle, NanoArray(usize),
    Pointer(Box<IrType>),
    Function(Vec<IrType>, Box<IrType>),
    Struct(String, HashMap<String, IrType>),
    Unknown,
    Error,
}

// --- IR Generator Structure ---
pub struct IrGenerator {
    ir_code: Vec<IrInstruction>,
    symbol_table: HashMap<String, IrValue>, // Maps resolved AST symbols to IR values (registers/globals)
    next_reg: usize,
    next_label: usize,
    errors: Vec<IrGenError>,
}

// --- IR Generation Error Structure ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGenError {
    pub message: String,
    pub span: Span, // Reference to the original source location
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            ir_code: Vec::new(),
            symbol_table: HashMap::new(), // Populated by semantic analysis
            next_reg: 0,
            next_label: 0,
            errors: Vec::new(),
        }
    }

    fn new_register(&mut self) -> IrRegister {
        let reg = IrRegister(self.next_reg);
        self.next_reg += 1;
        reg
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}{}", prefix, self.next_label);
        self.next_label += 1;
        label
    }

    pub fn generate_ir(&mut self, program: &Program, semantic_symbols: HashMap<String, Symbol>) -> Result<Vec<IrInstruction>, Vec<IrGenError>> {
        println!("Generating UMC IR from AST...");

        // Populate initial symbol table from semantic analysis results
        for (name, symbol) in semantic_symbols {
            // Conceptual: Allocate a global register or memory location for global symbols
            let ir_val = IrValue::Global(name.clone()); // Simplified
            self.symbol_table.insert(name, ir_val);
        }

        for stmt in &program.statements {
            self.gen_statement(stmt);
        }

        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(self.ir_code.clone())
        }
    }

    fn gen_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(span, name, type_expr_opt, expr) => {
                let expr_reg = self.gen_expression(expr);
                // Conceptual: Allocate local register/stack slot for 'name'
                let target_reg = self.new_register(); // For now, just generate a new reg
                self.ir_code.push(IrInstruction::Alloc(target_reg.clone(), self.map_type_to_ir_type(type_expr_opt.as_ref().map(|te| te.clone()))));
                self.ir_code.push(IrInstruction::Store(expr_reg.into(), target_reg.into()));
                // Update symbol table for local scope (conceptual for now)
                self.symbol_table.insert(name.clone(), IrValue::Register(target_reg));
            }
            Statement::Return(span, expr) => {
                let expr_val = self.gen_expression(expr);
                self.ir_code.push(IrInstruction::Return(Some(expr_val)));
            }
            Statement::Expression(expr) => {
                self.gen_expression(expr);
                // Discard result if it's not assigned
            }
            Statement::Function(span, name, params, return_type_expr_opt, body) => {
                // Conceptual: Function entry point
                self.ir_code.push(IrInstruction::Label(format!("fn_{}", name)));
                // Conceptual: map parameters to incoming registers/stack slots
                // Push body
                self.gen_expression(body); // Function body is a block expression
                // Implicit return Unit if no explicit return
                self.ir_code.push(IrInstruction::Return(None));
            }
            Statement::QuantumCircuit(span, name, body) => {
                self.ir_code.push(IrInstruction::Label(format!("qcirc_{}", name)));
                self.gen_expression(body);
                // Conceptual: A quantum circuit might not 'return' in the classical sense
                self.ir_code.push(IrInstruction::NoOp); // Or a specific quantum-end IR instruction
            }
            Statement::NanoAgent(span, name, body) => {
                self.ir_code.push(IrInstruction::Label(format!("nano_{}", name)));
                self.gen_expression(body);
                self.ir_code.push(IrInstruction::NoOp);
            }
            Statement::SankofaMemory(span, name, expr) => {
                let expr_val = self.gen_expression(expr);
                // Conceptual: Store in special Sankofa memory block
                let target_reg = self.new_register();
                self.ir_code.push(IrInstruction::Store(expr_val, IrValue::Global(format!("sankofa_{}", name))));
            }
            Statement::TypeDeclaration(span, name, type_expr) => {
                // Type declarations often don't generate runtime IR directly,
                // but inform the IR type system or code generation process.
                self.ir_code.push(IrInstruction::NoOp);
            }
            Statement::EffectDeclaration(span, name) => {
                // Effect declarations define effects, actual handling is done via `handle` or special runtime calls.
                self.ir_code.push(IrInstruction::NoOp);
            }
            Statement::LanguageDeclaration(span, name, grammar_expr) => {
                // Meta-compilation directives would be handled at a higher level, possibly
                // by invoking another compiler or runtime code generation during bootstrapping.
                self.ir_code.push(IrInstruction::NoOp);
            }
            Statement::While(span, cond_expr, body_expr) => {
                let loop_label = self.new_label("loop");
                let end_label = self.new_label("loop_end");
                self.ir_code.push(IrInstruction::Label(loop_label.clone()));

                let cond_reg = self.gen_expression(cond_expr);
                self.ir_code.push(IrInstruction::Branch(cond_reg, self.new_label(""), end_label.clone())); // Simplified branch
                
                self.gen_expression(body_expr);
                self.ir_code.push(IrInstruction::Jump(loop_label));
                self.ir_code.push(IrInstruction::Label(end_label));
            }
            Statement::For(span, iterator_var_id, iterable_expr, body_expr) => {
                // Conceptual: This would involve iterating over the iterable.
                // Highly simplified for conceptual purposes.
                let iterable_val = self.gen_expression(iterable_expr);
                let loop_label = self.new_label("for_loop");
                let end_label = self.new_label("for_end");

                // Conceptual: For iter = get_iterator(iterable_val)
                // Conceptual: For loop body:
                // self.ir_code.push(IrInstruction::Call(self.new_register(), "get_next_item".to_string(), vec![iterator_val]));
                // self.ir_code.push(IrInstruction::Branch(cond_reg, loop_label.clone(), end_label.clone()));
                self.ir_code.push(IrInstruction::Label(loop_label.clone()));
                self.gen_expression(body_expr);
                self.ir_code.push(IrInstruction::Jump(loop_label));
                self.ir_code.push(IrInstruction::Label(end_label));
            }
            Statement::Break(span) => {
                // Conceptual: Find nearest enclosing loop and jump to its end_label
                self.ir_code.push(IrInstruction::Jump(self.new_label("break_target"))); // Placeholder
            }
            Statement::Continue(span) => {
                // Conceptual: Find nearest enclosing loop and jump to its loop_label
                self.ir_code.push(IrInstruction::Jump(self.new_label("continue_target"))); // Placeholder
            }
            Statement::Match(span, matched_expr, cases) => {
                let matched_val = self.gen_expression(matched_expr);
                let end_label = self.new_label("match_end");

                for case in cases {
                    let case_label = self.new_label("case");
                    let next_case_label = self.new_label("next_case"); // For fall-through or next pattern

                    // Conceptual: Compare matched_val with case.pattern
                    let pattern_val = self.gen_expression(&case.pattern);
                    let cmp_reg = self.new_register();
                    self.ir_code.push(IrInstruction::CmpEq(cmp_reg.clone(), matched_val.clone(), pattern_val));
                    self.ir_code.push(IrInstruction::Branch(IrValue::Register(cmp_reg), case_label.clone(), next_case_label.clone()));

                    self.ir_code.push(IrInstruction::Label(case_label));
                    self.gen_expression(&case.body);
                    self.ir_code.push(IrInstruction::Jump(end_label.clone()));
                    self.ir_code.push(IrInstruction::Label(next_case_label));
                }
                self.ir_code.push(IrInstruction::Label(end_label));
            }
            Statement::Unsafe(span, proof_opt, inner_block_expr) => {
                // The semantic analyzer has already validated the proof.
                // Here, we just generate IR for the inner block.
                // This block might contain operations that are otherwise forbidden.
                self.gen_expression(inner_block_expr);
            }
        }
    }

    fn gen_expression(&mut self, expr: &Expression) -> IrValue {
        match expr {
            Expression::Identifier(Identifier(name, span)) => {
                // Conceptual: Load value from symbol table entry (which might be a register or global)
                self.symbol_table.get(name).cloned().unwrap_or_else(|| {
                    self.errors.push(IrGenError {
                        message: format!("Unresolved identifier '{}' during IR generation.", name),
                        span: span.clone(),
                    });
                    IrValue::Register(self.new_register()) // Return dummy
                })
            }
            Expression::Literal(literal) => {
                IrValue::Literal(literal.clone()) // Directly use AST literal
            }
            Expression::Prefix(span, op_type, right_expr) => {
                let right_val = self.gen_expression(right_expr);
                let result_reg = self.new_register();
                match op_type {
                    TokenType::Bang => self.ir_code.push(IrInstruction::Not(result_reg.clone(), right_val)),
                    TokenType::Minus => self.ir_code.push(IrInstruction::Neg(result_reg.clone(), right_val)),
                    _ => self.errors.push(IrGenError { message: format!("Unhandled prefix operator {:?}", op_type), span: span.clone() }),
                }
                IrValue::Register(result_reg)
            }
            Expression::Infix(span, left_expr, op_type, right_expr) => {
                let left_val = self.gen_expression(left_expr);
                let right_val = self.gen_expression(right_expr);
                let result_reg = self.new_register();
                match op_type {
                    TokenType::Plus => self.ir_code.push(IrInstruction::Add(result_reg.clone(), left_val, right_val)),
                    TokenType::Minus => self.ir_code.push(IrInstruction::Sub(result_reg.clone(), left_val, right_val)),
                    TokenType::Star => self.ir_code.push(IrInstruction::Mul(result_reg.clone(), left_val, right_val)),
                    TokenType::Slash => self.ir_code.push(IrInstruction::Div(result_reg.clone(), left_val, right_val)),
                    TokenType::Equals => self.ir_code.push(IrInstruction::CmpEq(result_reg.clone(), left_val, right_val)),
                    TokenType::NotEquals => self.ir_code.push(IrInstruction::CmpNe(result_reg.clone(), left_val, right_val)),
                    TokenType::LT => self.ir_code.push(IrInstruction::CmpLt(result_reg.clone(), left_val, right_val)),
                    TokenType::GT => self.ir_code.push(IrInstruction::CmpGt(result_reg.clone(), left_val, right_val)),
                    // ... other infix operators
                    _ => self.errors.push(IrGenError { message: format!("Unhandled infix operator {:?}", op_type), span: span.clone() }),
                }
                IrValue::Register(result_reg)
            }
            Expression::If(span, cond_expr, then_block, else_block_opt) => {
                let cond_val = self.gen_expression(cond_expr);
                let then_label = self.new_label("if_then");
                let else_label = self.new_label("if_else");
                let end_label = self.new_label("if_end");
                
                self.ir_code.push(IrInstruction::Branch(cond_val, then_label.clone(), else_label.clone()));
                
                self.ir_code.push(IrInstruction::Label(then_label));
                let then_result_val = self.gen_expression(then_block); // Get result of then-block
                // Conceptual: If if-expression has a return value, store it
                // self.ir_code.push(IrInstruction::Store(then_result_val, result_reg));
                self.ir_code.push(IrInstruction::Jump(end_label.clone()));

                self.ir_code.push(IrInstruction::Label(else_label));
                if let Some(else_block) = else_block_opt {
                    let else_result_val = self.gen_expression(else_block);
                    // Conceptual: If if-expression has a return value, store it
                    // self.ir_code.push(IrInstruction::Store(else_result_val, result_reg));
                }
                self.ir_code.push(IrInstruction::Jump(end_label.clone())); // Ensure control flow merges
                self.ir_code.push(IrInstruction::Label(end_label));

                IrValue::Register(self.new_register()) // Return placeholder
            }
            Expression::Block(span, statements) => {
                // Blocks create a new scope for local variables implicitly.
                // IR Generation typically flattens this, managing register allocation.
                // The value of a block is the value of its last expression.
                let mut last_val = IrValue::Literal(Literal::Integer("0".to_string(), span.clone())); // Default to dummy
                for stmt in statements {
                    if let Statement::Expression(expr) = stmt {
                        last_val = self.gen_expression(expr);
                    } else {
                        self.gen_statement(stmt);
                    }
                }
                last_val
            }
            Expression::Call(span, func_expr, args) => {
                let func_name_or_ptr = match self.gen_expression(func_expr) {
                    IrValue::Global(name) => name,
                    IrValue::Register(reg) => format!("reg_{}", reg.0), // Conceptual: function pointer in register
                    _ => {
                        self.errors.push(IrGenError { message: "Cannot call non-function IR value.".to_string(), span: span.clone() });
                        "".to_string()
                    }
                };
                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                self.ir_code.push(IrInstruction::Call(IrValue::Register(result_reg.clone()), func_name_or_ptr, arg_vals));
                IrValue::Register(result_reg)
            }
            Expression::Index(span, array_expr, index_expr) => {
                let array_ptr_val = self.gen_expression(array_expr);
                let index_val = self.gen_expression(index_expr);
                let result_reg = self.new_register();
                // Conceptual: Generate load instruction for array element
                self.ir_code.push(IrInstruction::Load(result_reg.clone(), array_ptr_val)); // Simplified: assumes array_ptr_val can be indexed
                // In reality: this would be a series of instructions to calculate element address: 
                // `elem_addr = base_addr + index * elem_size` then `load elem_addr`
                IrValue::Register(result_reg)
            }
            Expression::MemberAccess(span, object_expr, member_id) => {
                let object_val = self.gen_expression(object_expr);
                let result_reg = self.new_register();
                // Conceptual: Generate instruction to access a member of a struct/object
                // This would typically involve an offset from the object's base address.
                self.ir_code.push(IrInstruction::Load(result_reg.clone(), object_val)); // Simplified
                IrValue::Register(result_reg)
            }
        }
    }

    // New: Map AST TypeExpr to UMC IR Type
    fn map_type_to_ir_type(&self, ast_type_expr_opt: Option<TypeExpr>) -> IrType {
        let ast_type = ast_type_expr_opt.unwrap_or_else(|| {
            // Default to a base type if no type expression is provided (e.g., implicit type in 'let')
            TypeExpr::Base(Identifier("Unknown".to_string(), Span::new(0,0,0)))
        });

        match ast_type {
            TypeExpr::Base(Identifier(name, _)) => match name.as_str() {
                "int" => IrType::I32,
                "float" => IrType::F64,
                "bool" => IrType::Bool,
                "string" => IrType::StringPtr,
                "char" => IrType::Char,
                "Qubit" => IrType::Qubit,
                "NanoAgent" => IrType::NanoParticle, // Simplified
                _ => IrType::Unknown,
            },
            TypeExpr::Array(element_type_expr, size_opt) => {
                let ir_elem_type = self.map_type_to_ir_type(Some(*element_type_expr));
                if let Some(size_str) = size_opt {
                    // Conceptual: parse size_str to usize if it's a QReg[N]
                    if ir_elem_type == IrType::Qubit {
                        if let Ok(size) = size_str.parse::<usize>() {
                            return IrType::QubitArray(size);
                        }
                    }
                }
                IrType::Pointer(Box::new(ir_elem_type)) // General array as pointer to first element
            }
            TypeExpr::FunctionType(param_type_exprs, return_type_expr) => {
                let ir_param_types: Vec<IrType> = param_type_exprs.into_iter().map(|te| self.map_type_to_ir_type(Some(te))).collect();
                let ir_return_type = self.map_type_to_ir_type(Some(*return_type_expr));
                IrType::Function(ir_param_types, Box::new(ir_return_type))
            }
            // Add more mappings for Linear, Affine, Effectful, Dependent, etc.
            _ => IrType::Unknown, // Fallback for complex types not yet mapped to IR
        }
    }

    pub fn get_errors(&self) -> &[IrGenError] {
        &self.errors
    }
}
