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
use crate::source_map::Span; // Corrected Span import
use std::collections::{HashMap, VecDeque};

// --- UMC IR Instruction Set (Conceptual) ---
#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Control Flow
    Label(String),
    Jump(String),
    Branch(IrValue, String, String), // condition, true_label, false_label
    Call(IrRegister, String, Vec<IrValue>), // result_reg, function_name, args
    Return(Option<IrValue>),
    Yield(IrValue), // For async/generators
    Await(IrRegister, IrValue), // result_reg = await future_val

    // Memory Operations
    Alloc(IrRegister, IrType), // reg = allocate type
    Load(IrRegister, IrValue), // reg = load from address_val
    Store(IrValue, IrValue), // store value to address_val (val to target_addr)
    GlobalAddr(IrRegister, String, IrType), // reg = address of global_var_name of type
    Deref(IrRegister, IrValue), // reg = dereference pointer_val

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
    CmpLt(IrRegister, IrValue, IrValue), // reg = op1 < op2
    CmpLe(IrRegister, IrValue, IrValue), // reg = op1 <= op2
    CmpGt(IrRegister, IrValue, IrValue), // reg = op1 > op2
    CmpGe(IrRegister, IrValue, IrValue), // reg = op1 >= op2

    // Type Conversion
    Cast(IrRegister, IrValue, IrType), // reg = cast value to type

    // Special Zenith UMC IR - Expanded Quantum
    QInit(IrRegister, IrValue), // reg = init_qubit(state: Literal<"0"|"1"|"+">)
    QAlloc(IrRegister, IrValue), // reg = alloc_qreg(size: Literal<usize>)
    QGate(IrRegister, String, Vec<IrValue>), // result_qreg_or_qubit = gate_name(input_qregs_or_qubits)
    QMeasure(IrRegister, IrValue), // classic_reg = measure qubit_reg (or qreg, gives bit string)
    QEntangle(IrValue, IrValue), // entangle q1, q2
    QTeleport(IrValue, IrValue, IrValue), // teleport q_source, q_dest, classical_key_reg

    // Special Zenith UMC IR - Expanded Nano
    NanoAssemble(IrRegister, IrValue, Vec<IrValue>), // reg = assemble_nano_agent(blueprint_id, components...)
    NanoCommunicate(IrValue, IrValue, IrValue), // nano_agent.communicate(target, message)
    NanoReplicate(IrRegister, IrValue), // new_agent = nano_agent.replicate()

    // Special Zenith UMC IR - Expanded MTS (Multi-Timeline System)
    MTSCreate(IrRegister, IrValue), // reg = create_mts_slice(initial_value)
    MTSLoad(IrRegister, IrValue, IrValue), // reg = mts_slice.load(timestamp)
    MTSStore(IrValue, IrValue, IrValue), // mts_slice.store(value, timestamp)

    // Special Zenith UMC IR - Expanded Effects
    EffectOp(IrRegister, String, Vec<IrValue>), // result = perform_effect_op(effect_name, args)
    HandleEffect(String, String, String), // handle effect_name from effect_value_reg with handler_label

    // Linear/Affine specific operations - Expanded
    Consume(IrValue), // Marks a linear resource as consumed
    Drop(IrValue), // Marks an affine resource as dropped (optional consumption)
    Borrow(IrRegister, IrValue, String), // reg = borrow(original_resource_val, mutability_mode)
    Clone(IrRegister, IrValue), // reg = clone(resource_val) - only for clonable types

    // Sankofa memory operations - Expanded
    ReadHistory(IrRegister, String, IrValue), // reg = read_history(key_id, timestamp_expr)
    WriteHistory(String, IrValue, IrValue), // write_history(key_id, value, timestamp_expr)
    AccessZamani(IrRegister, String), // reg = access_zamani_fact(fact_id) - read immutable past
    AccessSasa(IrRegister, String), // reg = access_sasa_knowledge(knowledge_id) - read evolving present
    TemporalLearn(String, IrValue, IrValue), // temporal_learn(key_id, knowledge_value, timestamp_range)

    NoOp, // Placeholder
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Register(IrRegister),
    Literal(Literal),
    Global(String), // Reference to a global variable/function/memory location
}

// Convert IrRegister to IrValue
impl From<IrRegister> for IrValue {
    fn from(reg: IrRegister) -> Self {
        IrValue::Register(reg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrRegister(usize); // Virtual register ID

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    Void, // For Unit type or functions returning nothing
    I32, F64, Bool, StringPtr, Char,
    Qubit, QubitArray(usize), // Qubit array with explicit size
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
    // Mapping from AST-resolved symbol names to their IR representation
    symbol_table: HashMap<String, IrValue>,
    next_reg: usize,
    next_label: usize,
    errors: Vec<IrGenError>,
    // New: Stack to manage loop labels for break/continue
    loop_labels: Vec<(String, String)>, // (loop_start_label, loop_end_label)
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
            symbol_table: HashMap::new(),
            next_reg: 0,
            next_label: 0,
            errors: Vec::new(),
            loop_labels: Vec::new(),
        }
    }

    fn new_register(&mut self) -> IrRegister {
        let reg = IrRegister(self.next_reg);
        self.next_reg += 1;
        reg
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("L{}{}", prefix, self.next_label);
        self.next_label += 1;
        label
    }

    pub fn generate_ir(&mut self, program: &Program, semantic_symbols: &HashMap<String, Symbol>) -> Result<Vec<IrInstruction>, Vec<IrGenError>> {
        println!("Generating UMC IR from AST...");

        // Populate initial global symbol table from semantic analysis results
        for (name, symbol) in semantic_symbols {
            // For globals, assume they live in a global memory region and map to their address
            self.symbol_table.insert(name.clone(), IrValue::Global(name.clone()));
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
                let expr_val = self.gen_expression(expr);
                let target_reg = self.new_register();
                let ir_type = self.map_type_expr_to_ir_type(type_expr_opt);
                
                // Allocate space for the variable
                self.ir_code.push(IrInstruction::Alloc(target_reg.clone(), ir_type));
                // Store the evaluated expression into the allocated space
                self.ir_code.push(IrInstruction::Store(expr_val, target_reg.into()));
                // Add to current scope's symbol table (simplified as global here)
                self.symbol_table.insert(name.clone(), IrValue::Register(target_reg));
            }
            Statement::Return(span, expr) => {
                let expr_val = self.gen_expression(expr);
                self.ir_code.push(IrInstruction::Return(Some(expr_val)));
            }
            Statement::Expression(expr) => {
                self.gen_expression(expr);
                // Result of expression statements might be discarded if not assigned
            }
            Statement::Function(span, name, params, return_type_expr_opt, body) => {
                let func_label = self.new_label(&format!("func_{}", name));
                self.ir_code.push(IrInstruction::Label(func_label.clone()));

                // Conceptual: Map parameters to function's local registers/stack.
                // For simplicity, skip explicit parameter handling in IR generation for now.

                self.gen_expression(body); // Function body is a block expression

                // Ensure a return instruction. Functions implicitly return Unit if no explicit return. 
                if !matches!(self.ir_code.last(), Some(IrInstruction::Return(_))) {
                    self.ir_code.push(IrInstruction::Return(None));
                }
            }
            Statement::QuantumCircuit(span, name, body) => {
                let qcirc_label = self.new_label(&format!("qcirc_{}", name));
                self.ir_code.push(IrInstruction::Label(qcirc_label.clone()));
                // The body contains quantum operations
                self.gen_expression(body);
                self.ir_code.push(IrInstruction::NoOp); // End of quantum circuit block
            }
            Statement::NanoAgent(span, name, body) => {
                let nano_label = self.new_label(&format!("nano_{}", name));
                self.ir_code.push(IrInstruction::Label(nano_label.clone()));
                // The body defines nano-agent behaviors
                self.gen_expression(body);
                self.ir_code.push(IrInstruction::NoOp); // End of nano-agent block
            }
            Statement::SankofaMemory(span, name, expr) => {
                let expr_val = self.gen_expression(expr);
                // Conceptual: Write to a specific Sankofa historical memory key, assuming "current" timestamp.
                self.ir_code.push(IrInstruction::WriteHistory(name.clone(), expr_val, IrValue::Literal(Literal::String("current".to_string(), span.clone()))));
            }
            Statement::TypeDeclaration(_, _, _) | Statement::EffectDeclaration(_, _) | Statement::LanguageDeclaration(_, _, _) => {
                // These statements are typically handled by semantic analysis and don't generate runtime IR directly,
                // but inform the IR type system or code generation process.
                self.ir_code.push(IrInstruction::NoOp);
            }
            Statement::While(span, cond_expr, body_expr) => {
                let loop_start_label = self.new_label("while_loop_start");
                let loop_end_label = self.new_label("while_loop_end");
                self.loop_labels.push((loop_start_label.clone(), loop_end_label.clone()));

                self.ir_code.push(IrInstruction::Label(loop_start_label.clone()));
                let cond_val = self.gen_expression(cond_expr);
                let cond_reg = self.new_register();
                self.ir_code.push(IrInstruction::Store(cond_val, cond_reg.clone().into())); // Store condition result in a register
                // Branch to end if condition is false
                self.ir_code.push(IrInstruction::Branch(cond_reg.into(), self.new_label("while_cond_true"), loop_end_label.clone())); // The true label is implicitly the next instruction
                
                self.gen_expression(body_expr); // Generate IR for loop body
                self.ir_code.push(IrInstruction::Jump(loop_start_label.clone())); // Jump back to start of loop
                self.ir_code.push(IrInstruction::Label(loop_end_label));

                self.loop_labels.pop(); // Exit loop scope
            }
            Statement::For(span, iterator_var_id, iterable_expr, body_expr) => {
                // Conceptual: A 'for' loop often translates to a 'while' loop internally
                // For simplified IR:
                let iterable_val = self.gen_expression(iterable_expr); // Get the iterable
                let loop_start_label = self.new_label("for_loop_start");
                let loop_end_label = self.new_label("for_loop_end");
                self.loop_labels.push((loop_start_label.clone(), loop_end_label.clone()));

                // Conceptual IR to setup iteration:
                // iter_obj_reg = call __get_iterator(iterable_val)
                let iter_obj_reg = self.new_register();
                let get_iterator_func = self.symbol_table.get("__get_iterator").cloned().unwrap_or_else(|| IrValue::Global("__get_iterator".to_string())).to_string();
                self.ir_code.push(IrInstruction::Call(iter_obj_reg.clone(), get_iterator_func, vec![iterable_val]));

                self.ir_code.push(IrInstruction::Label(loop_start_label.clone()));

                // Conceptual: item_present_reg = call __has_next(iter_obj_reg)
                let has_next_reg = self.new_register();
                let has_next_func = self.symbol_table.get("__has_next").cloned().unwrap_or_else(|| IrValue::Global("__has_next".to_string())).to_string();
                self.ir_code.push(IrInstruction::Call(has_next_reg.clone(), has_next_func, vec![iter_obj_reg.clone().into()]));
                self.ir_code.push(IrInstruction::Branch(has_next_reg.into(), self.new_label("for_loop_body"), loop_end_label.clone()));
                
                // Conceptual: item_val_reg = call __next(iter_obj_reg)
                let item_val_reg = self.new_register();
                let next_func = self.symbol_table.get("__next").cloned().unwrap_or_else(|| IrValue::Global("__next".to_string())).to_string();
                self.ir_code.push(IrInstruction::Call(item_val_reg.clone(), next_func, vec![iter_obj_reg.clone().into()]));

                // Store the current item into the iterator variable's IR representation (conceptual) 
                // This would be more complex with scope management for iterator_var_id
                if let Some(var_ir_val) = self.symbol_table.get(&iterator_var_id.0).cloned() {
                     self.ir_code.push(IrInstruction::Store(item_val_reg.into(), var_ir_val));
                } else {
                    self.errors.push(IrGenError { message: format!("For loop iterator variable '{}' not found in symbol table.", iterator_var_id.0), span: iterator_var_id.1.clone() });
                }

                self.gen_expression(body_expr); // Generate IR for body
                self.ir_code.push(IrInstruction::Jump(loop_start_label.clone()));
                self.ir_code.push(IrInstruction::Label(loop_end_label));

                self.loop_labels.pop(); // Exit loop scope
            }
            Statement::Break(span) => {
                if let Some((_, end_label)) = self.loop_labels.last() {
                    self.ir_code.push(IrInstruction::Jump(end_label.clone()));
                } else {
                    self.errors.push(IrGenError { message: "Break statement outside of loop.".to_string(), span: span.clone() });
                }
            }
            Statement::Continue(span) => {
                if let Some((start_label, _)) = self.loop_labels.last() {
                    self.ir_code.push(IrInstruction::Jump(start_label.clone()));
                } else {
                    self.errors.push(IrGenError { message: "Continue statement outside of loop.".to_string(), span: span.clone() });
                }
            }
            Statement::Match(span, matched_expr, cases) => {
                let matched_val = self.gen_expression(matched_expr);
                let end_label = self.new_label("match_end");

                for (i, case) in cases.iter().enumerate() {
                    let case_label_body = self.new_label(&format!("match_case_{}_body", i));
                    let next_case_label = self.new_label(&format!("match_case_{}_next", i));

                    // Conceptual: Compare matched_val with case.pattern
                    let pattern_val = self.gen_expression(&case.pattern);
                    let cmp_reg = self.new_register();
                    self.ir_code.push(IrInstruction::CmpEq(cmp_reg.clone(), matched_val.clone(), pattern_val));
                    self.ir_code.push(IrInstruction::Branch(cmp_reg.into(), case_label_body.clone(), next_case_label.clone()));

                    self.ir_code.push(IrInstruction::Label(case_label_body));
                    self.gen_expression(&case.body);
                    self.ir_code.push(IrInstruction::Jump(end_label.clone())); // Jump to end after executing case body
                    
                    self.ir_code.push(IrInstruction::Label(next_case_label));
                }
                self.ir_code.push(IrInstruction::Label(end_label));
            }
            Statement::Unsafe(span, proof_opt, inner_block_expr) => {
                // The IR generator simply generates code for the inner block.
                // Semantic analysis ensured proper EVAS proof if required.
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
                        message: format!("Unresolved identifier '{}' during IR generation. (Should have been caught by semantic analysis)", name),
                        span: span.clone(),
                    });
                    IrValue::Register(self.new_register()) // Return dummy
                })
            }
            Expression::Literal(literal) => {
                IrValue::Literal(literal.clone()) // Directly embed AST literal into IR value
            }
            Expression::Prefix(span, op_type, right_expr) => {
                let right_val = self.gen_expression(right_expr);
                let result_reg = self.new_register();
                // Make sure `TokenType` is accessible for `op_type`
                use crate::tokens::TokenType;
                match op_type {
                    TokenType::Bang => self.ir_code.push(IrInstruction::Not(result_reg.clone(), right_val)),
                    TokenType::Minus => self.ir_code.push(IrInstruction::Neg(result_reg.clone(), right_val)),
                    _ => self.errors.push(IrGenError { message: format!("Unhandled prefix operator {:?} during IR generation", op_type), span: span.clone() }),
                }
                IrValue::Register(result_reg)
            }
            Expression::Infix(span, left_expr, op_type, right_expr) => {
                let left_val = self.gen_expression(left_expr);
                let right_val = self.gen_expression(right_expr);
                let result_reg = self.new_register();
                // Make sure `TokenType` is accessible for `op_type`
                use crate::tokens::TokenType;
                match op_type {
                    TokenType::Plus => self.ir_code.push(IrInstruction::Add(result_reg.clone(), left_val, right_val)),
                    TokenType::Minus => self.ir_code.push(IrInstruction::Sub(result_reg.clone(), left_val, right_val)),
                    TokenType::Star => self.ir_code.push(IrInstruction::Mul(result_reg.clone(), left_val, right_val)),
                    TokenType::Slash => self.ir_code.push(IrInstruction::Div(result_reg.clone(), left_val, right_val)),
                    TokenType::Equals => self.ir_code.push(IrInstruction::CmpEq(result_reg.clone(), left_val, right_val)),
                    TokenType::NotEquals => self.ir_code.push(IrInstruction::CmpNe(result_reg.clone(), left_val, right_val)),
                    TokenType::LT => self.ir_code.push(IrInstruction::CmpLt(result_reg.clone(), left_val, right_val)),
                    TokenType::GT => self.ir_code.push(IrInstruction::CmpGt(result_reg.clone(), left_val, right_val)),
                    TokenType::LTE => self.ir_code.push(IrInstruction::CmpLe(result_reg.clone(), left_val, right_val)),
                    TokenType::GTE => self.ir_code.push(IrInstruction::CmpGe(result_reg.clone(), left_val, right_val)),
                    // Further conceptual: Add specific IR for quantum operations that appear as infix.
                    // E.g., a conceptual 'quantum_teleport' operator
                    _ => self.errors.push(IrGenError { message: format!("Unhandled infix operator {:?} during IR generation", op_type), span: span.clone() }),
                }
                IrValue::Register(result_reg)
            }
            Expression::If(span, cond_expr, then_block, else_block_opt) => {
                let cond_val = self.gen_expression(cond_expr);
                let then_label = self.new_label("if_then");
                let else_label = self.new_label("if_else");
                let end_label = self.new_label("if_end");
                
                let result_reg = self.new_register(); // Register to hold result of the if-expression

                self.ir_code.push(IrInstruction::Branch(cond_val, then_label.clone(), else_label.clone()));
                
                self.ir_code.push(IrInstruction::Label(then_label));
                let then_result_val = self.gen_expression(then_block); 
                self.ir_code.push(IrInstruction::Store(then_result_val, result_reg.clone().into())); // Store then-branch result
                self.ir_code.push(IrInstruction::Jump(end_label.clone()));

                self.ir_code.push(IrInstruction::Label(else_label));
                if let Some(else_block) = else_block_opt {
                    let else_result_val = self.gen_expression(else_block);
                    self.ir_code.push(IrInstruction::Store(else_result_val, result_reg.clone().into())); // Store else-branch result
                } else {
                    // If no else branch, and if is an expression, default to Unit/Void
                    // self.ir_code.push(IrInstruction::Store(IrValue::VoidLiteral, result_reg.clone().into()));
                }
                self.ir_code.push(IrInstruction::Jump(end_label.clone())); // Ensure control flow merges
                self.ir_code.push(IrInstruction::Label(end_label));

                IrValue::Register(result_reg)
            }
            Expression::Block(span, statements) => {
                // A block's value is the value of its last expression.
                let mut last_val = IrValue::Literal(Literal::Integer("0".to_string(), span.clone())); // Default to dummy
                for stmt in statements.clone() { // Clone to avoid mutable borrow issues with gen_statement
                    if let Statement::Expression(expr) = stmt {
                        last_val = self.gen_expression(&expr);
                    } else {
                        self.gen_statement(&stmt);
                    }
                }
                last_val
            }
            Expression::Call(span, func_expr, args) => {
                // Assume func_expr evaluates to a function pointer or a global function name
                let func_target = match func_expr.as_ref() {
                    Expression::Identifier(Identifier(name, _)) => name.clone(),
                    _ => {
                        // For more complex function expressions (e.g. closures), we'd need to evaluate the func_expr to a value
                        self.errors.push(IrGenError { message: "Only direct function identifiers supported for calls in conceptual IR.".to_string(), span: span.clone() });
                        "".to_string()
                    }
                };

                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                self.ir_code.push(IrInstruction::Call(result_reg.clone(), func_target, arg_vals));
                IrValue::Register(result_reg)
            }
            Expression::Index(span, array_expr, index_expr) => {
                let array_ptr_val = self.gen_expression(array_expr);
                let index_val = self.gen_expression(index_expr);
                let result_reg = self.new_register();
                
                // Conceptual: Generate address calculation and then load.
                // Assuming array_ptr_val is a base address (e.g., a pointer to the first element).
                // Requires type information from semantic analysis to know element size.
                self.ir_code.push(IrInstruction::Load(result_reg.clone(), array_ptr_val)); // Simplified: needs offset calculation
                // Example of more detailed IR:
                // `elem_size_reg = LoadConstant(size_of(element_type))`
                // `offset_reg = Mul(new_reg, index_val, elem_size_reg)`
                // `addr_reg = Add(new_reg, array_ptr_val, offset_reg)`
                // `Load(result_reg, addr_reg)`
                IrValue::Register(result_reg)
            }
            Expression::MemberAccess(span, object_expr, member_id) => {
                let object_val = self.gen_expression(object_expr); // This would be the object's base address
                let result_reg = self.new_register();
                
                // Conceptual: Access a member by calculating its offset from the object's base address.
                // This requires semantic information about the object's struct definition.
                self.ir_code.push(IrInstruction::Load(result_reg.clone(), object_val)); // Simplified. Needs offset calculation.
                IrValue::Register(result_reg)
            }
            Expression::QuantumGateApplication(span, gate_name, args) => {
                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                self.ir_code.push(IrInstruction::QGate(result_reg.clone(), gate_name.clone(), arg_vals));
                IrValue::Register(result_reg)
            }
            Expression::NanoAction(span, action_name, args) => {
                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                self.ir_code.push(IrInstruction::NanoOp(result_reg.clone(), action_name.clone(), arg_vals));
                IrValue::Register(result_reg)
            }
            Expression::MtsOperation(span, op_name, args) => {
                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                // Conceptual mapping for MTS operations based on op_name
                match op_name.as_str() {
                    "create_slice" => self.ir_code.push(IrInstruction::MTSCreate(result_reg.clone(), arg_vals.get(0).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))))),
                    "load" => self.ir_code.push(IrInstruction::MTSLoad(result_reg.clone(), arg_vals.get(0).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))), arg_vals.get(1).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))))),
                    "store" => self.ir_code.push(IrInstruction::MTSStore(arg_vals.get(0).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))), arg_vals.get(1).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))), arg_vals.get(2).cloned().unwrap_or(IrValue::Literal(Literal::Integer("0".to_string(), span.clone()))))),
                    _ => self.errors.push(IrGenError { message: format!("Unhandled MTS operation '{}' during IR generation", op_name), span: span.clone() }),
                }
                IrValue::Register(result_reg)
            }
            Expression::PerformEffect(span, effect_name, args) => {
                let arg_vals: Vec<IrValue> = args.iter().map(|arg| self.gen_expression(arg)).collect();
                let result_reg = self.new_register();
                self.ir_code.push(IrInstruction::EffectOp(result_reg.clone(), effect_name.clone(), arg_vals));
                IrValue::Register(result_reg)
            }
        }
    }

    // New: Map AST TypeExpr to UMC IR Type
    fn map_type_expr_to_ir_type(&self, ast_type_expr_opt: Option<&TypeExpr>) -> IrType {
        if let Some(ast_type_expr) = ast_type_expr_opt {
            match ast_type_expr {
                TypeExpr::Base(Identifier(name, _)) => match name.as_str() {
                    "int" => IrType::I32,
                    "float" => IrType::F64,
                    "bool" => IrType::Bool,
                    "string" => IrType::StringPtr,
                    "char" => IrType::Char,
                    "Qubit" => IrType::Qubit,
                    "NanoAgent" => IrType::NanoParticle, // Simplified
                    "unit" => IrType::Void, // Map Unit to Void in IR
                    _ => IrType::Unknown,
                },
                TypeExpr::Array(element_type_expr, size_opt) => {
                    let ir_elem_type = self.map_type_expr_to_ir_type(Some(element_type_expr));
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
                    let ir_param_types: Vec<IrType> = param_type_exprs.iter().map(|te| self.map_type_expr_to_ir_type(Some(te))).collect();
                    let ir_return_type = self.map_type_expr_to_ir_type(Some(return_type_expr));
                    IrType::Function(ir_param_types, Box::new(ir_return_type))
                }
                TypeExpr::Linear(inner_type_expr) | TypeExpr::Affine(inner_type_expr) => {
                    // For linear/affine, the underlying IR type is the same,
                    // but the IR generator might emit additional `Consume`/`Drop` instructions.
                    self.map_type_expr_to_ir_type(Some(inner_type_expr))
                }
                TypeExpr::Effectful(base_type_expr, _) => {
                    // Effectful types mostly inform semantic analysis; IR type is the base type.
                    self.map_type_expr_to_ir_type(Some(base_type_expr))
                }
                // Add more mappings for other complex TypeExpr variants
                _ => IrType::Unknown, // Fallback for complex types not yet mapped to IR
            }
        } else {
            IrType::Void // Default for untyped or implicit void contexts
        }
    }


    pub fn get_errors(&self) -> &[IrGenError] {
        &self.errors
    }
}
