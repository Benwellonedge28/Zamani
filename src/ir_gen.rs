
//! Zenith Universal Meta-Compiler (UMC) Intermediate Representation (IR) Generator
//!
//! This module implements the IR generation phase of the Zenith compiler. It takes
//! the semantically analyzed Abstract Syntax Tree (AST) and translates it into a
//! platform-agnostic Intermediate Representation (IR). This IR is designed to
//! efficiently represent multi-paradigm constructs (classical, quantum, nano, MTS,
//! Sankofa) and is the input for optimization and backend code generation stages.

use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::{
    Program, Statement, Expression, Literal, Identifier, Parameter, MatchCase,
    AccessModifier, ClassMember, InterfaceMember, TypeExpr, MethodModifier, TokenType
};
use crate::compiler_types::{Type, MethodType, IntWidth, FloatWidth};
use crate::semantic::{SymbolTable, SemanticError, Symbol}; // Access to resolved types and symbols
use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrGenError {
    pub message: String,
    pub span: Span,
}

// --- Intermediate Representation (IR) Instructions --- 
#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Control Flow
    Label(String),
    Jump(String),
    CondJump(IrValue, String),
    Ret(Option<IrValue>),

    // Variable Operations
    Alloc(IrRegister, IrType),
    Assign(IrRegister, IrValue),
    Load(IrRegister, IrValue),
    Store(IrValue, IrValue),

    // Arithmetic & Logic
    Add(IrRegister, IrValue, IrValue),
    Sub(IrRegister, IrValue, IrValue),
    Mul(IrRegister, IrValue, IrValue),
    Div(IrRegister, IrValue, IrValue),
    Mod(IrRegister, IrValue, IrValue),
    And(IrRegister, IrValue, IrValue),
    Or(IrRegister, IrValue, IrValue),
    Xor(IrRegister, IrValue, IrValue),
    Not(IrRegister, IrValue),
    Neg(IrRegister, IrValue),

    // Comparisons
    CmpEq(IrRegister, IrValue, IrValue),
    CmpNeq(IrRegister, IrValue, IrValue),
    CmpLt(IrRegister, IrValue, IrValue),
    CmpGt(IrRegister, IrValue, IrValue),
    CmpLe(IrRegister, IrValue, IrValue),
    CmpGe(IrRegister, IrValue, IrValue),

    // Function Calls
    Call(IrRegister, String, Vec<IrValue>), // Result Reg, Function Name, Arguments

    // Quantum-Specific
    QAlloc(IrRegister, IrValue), // Allocate Qubit/QReg, store ID in Reg
    QGate(IrRegister, String, Vec<IrValue>), // Apply Gate to Qubit/QReg in Reg, Gate Name, Arguments (e.g., control qubits)
    QMeasure(IrRegister, IrValue), // Measure Qubit/QReg, store classical result in Reg

    // Nano-Specific
    NanoAssemble(IrRegister, IrValue, Vec<IrValue>), // Assemble NanoAgent, blueprint, components
    NanoCommunicate(IrValue, IrValue, IrValue), // Sender, Receiver, Message
    NanoReplicate(IrRegister, IrValue), // Replicate Agent, store new ID in Reg
    NanoAction(IrValue, IrValue), // Agent, Action

    // MTS-Specific (Multi-Timeline System)
    MTSCreate(IrRegister, IrValue), // Create timeline, initial state, store ID in Reg
    MTSStore(IrValue, IrValue, IrValue), // Timeline ID, Timestamp, State Content
    MTSLoad(IrRegister, IrValue, IrValue), // Load state from Timeline ID at Timestamp, store in Reg
    MTSSynchronize(IrRegister, IrValue, IrValue, IrValue), // Reg for result ID, Timeline1, Timeline2, MergePoint
    MTSCheckCausality(IrRegister, IrValue), // Timeline ID, store bool result in Reg

    // Sankofa-Specific
    SankofaRecordFact(IrValue, IrValue, IrValue, IrValue), // Fact ID, Content, Timestamp, Provenance
    SankofaAccessFact(IrRegister, IrValue), // Reg for content, Fact ID
    SankofaUpdateKnowledge(IrRegister, IrValue, IrValue, IrValue, Vec<IrValue>), // Reg for new version, Knowledge ID, Content, Timestamp, Causal Predecessors
    SankofaAccessKnowledge(IrRegister, IrValue, IrValue), // Reg for content, Knowledge ID, Timestamp
    SankofaTemporalLearn(IrValue, IrValue, IrValue), // Knowledge ID, Start Time, End Time

    // Algebraic Effects
    PerformEffect(Identifier, Option<IrValue>), // Effect name, optional payload
    HandleEffect(Identifier, String, String), // Effect name, handler code block label, original code block label

    // No Operation
    NoOp,

    // --- OOP IR Additions --- 
    AllocObject(IrRegister, Type), // Allocate memory for an object of Type, store ptr in Reg
    LoadField(IrRegister, IrRegister, Identifier), // Load field 'Ident' from object in Reg1, store value in Reg2
    StoreField(IrRegister, Identifier, IrRegister), // Store value in Reg3 into field 'Ident' of object in Reg1
    CallMethod(IrRegister, IrRegister, Identifier, Vec<IrValue>, CallType), // Result Reg, Object in Reg, Method Name, Arguments, CallType (static/dynamic)
    LoadThis(IrRegister), // Load 'this' pointer into register
    LoadSuper(IrRegister), // Load 'super' pointer into register (or parent class context)
    CreateVtable(Type), // Create a vtable for class Type
    CreateItable(Type, Type), // Create an itable for class Type implementing Interface Type
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallType {
    Static,   // Direct call to a known method
    Dynamic,  // Call via vtable lookup
    Super,    // Call to parent class method
}

// --- IR Value Types ---
#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Register(IrRegister),
    Literal(Literal),
    Label(String),
    Type(IrType),
}

// --- IR Register ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrRegister(pub usize);

// --- IR Type (simplified for now) ---
#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    I32, // Integer 32-bit
    Bool,
    String,
    Qubit,
    NanoAgent,
    MtsSlice,
    Ptr,
    Unit,
    Class(Identifier), // Reference to a class type for object allocation
}

pub struct IrGenerator {
    current_ir: Vec<IrInstruction>,
    symbol_table: Arc<SymbolTable>, // Access to the semantically analyzed symbol table
    next_register: usize,
    errors: Vec<IrGenError>,
}

impl IrGenerator {
    pub fn new(symbol_table: Arc<SymbolTable>) -> Self { // Now takes a resolved symbol table
        IrGenerator {
            current_ir: Vec::new(),
            symbol_table,
            next_register: 0,
            errors: Vec::new(),
        }
    }

    pub fn generate_ir(&mut self, program: &Program, initial_symbol_table: &SymbolTable) -> Result<Vec<IrInstruction>, Vec<IrGenError>> {
        // Note: For a real compiler, the symbol_table passed to new() should be the final, resolved table
        // from semantic analysis. We pass it here for conceptual access.

        self.current_ir.clear();
        self.next_register = 0;
        self.errors.clear();

        // Generate IR for OOP metadata first (vtables, itables)
        if let Err(e) = self.generate_oop_metadata_ir() {
            self.errors.push(e);
        }

        for stmt in &program.statements {
            if let Err(e) = self.generate_statement_ir(stmt) {
                self.errors.push(e);
            }
        }

        if self.errors.is_empty() { Ok(self.current_ir.clone()) } else { Err(self.errors.clone()) }
    }

    fn generate_statement_ir(&mut self, stmt: &Statement) -> Result<(), IrGenError> {
        match stmt {
            Statement::Let(span, name, _, expr) => {
                let expr_val = self.generate_expression_ir(expr)?;
                let reg = self.new_register(); // Should look up type in symbol table to get correct IrType
                self.current_ir.push(IrInstruction::Assign(reg, expr_val));
                // Store name -> reg mapping in a local IR symbol table (conceptual)
                Ok(())
            }
            Statement::Return(span, expr) => {
                let expr_val = self.generate_expression_ir(expr)?;
                self.current_ir.push(IrInstruction::Ret(Some(expr_val)));
                Ok(())
            }
            Statement::Expression(expr) => {
                self.generate_expression_ir(expr)?;
                Ok(())
            }
            Statement::Function(span, name, params, ret_type, body) => {
                self.current_ir.push(IrInstruction::Label(format!("fn_{}", name)));
                // Generate IR for function parameters (conceptual: store in registers/stack slots)
                self.generate_expression_ir(body)?;
                self.current_ir.push(IrInstruction::Ret(None)); // Implicit return unit if no explicit return
                Ok(())
            }
            // --- OOP Statements ---
            Statement::Class(_, _, _, _) => Ok(()), // Class definitions primarily influence types and vtables/itables, not direct executable IR for definition itself
            Statement::Interface(_, _, _, _) => Ok(()), // Interfaces are pure type definitions

            _ => Err(IrGenError { message: format!("Unsupported statement for IR generation: {:?}", stmt), span: stmt.span() }),
        }
    }

    fn generate_expression_ir(&mut self, expr: &Expression) -> Result<IrValue, IrGenError> {
        match expr {
            Expression::Identifier(ident) => {
                // Lookup identifier in IR symbol table to get its register or memory location
                // For now, assume it's a placeholder or already assigned to a register
                Ok(IrValue::Register(IrRegister(0))) // Dummy
            }
            Expression::Literal(lit) => Ok(IrValue::Literal(lit.clone())),
            Expression::Infix(span, left, op, right) => {
                let left_val = self.generate_expression_ir(left)?;
                let right_val = self.generate_expression_ir(right)?;
                let result_reg = self.new_register();
                match op {
                    TokenType::Plus => self.current_ir.push(IrInstruction::Add(result_reg, left_val, right_val)),
                    TokenType::Minus => self.current_ir.push(IrInstruction::Sub(result_reg, left_val, right_val)),
                    // ... other ops
                    _ => return Err(IrGenError { message: format!("Unsupported infix operator for IR: {:?}", op), span: span.clone() }),
                }
                Ok(IrValue::Register(result_reg))
            }
            Expression::Call(span, func_expr, args) => {
                // This is for standalone functions, not methods
                if let Expression::Identifier(func_name_ident) = func_expr.as_ref() {
                    let args_ir = args.iter().map(|arg| self.generate_expression_ir(arg)).collect::<Result<Vec<IrValue>, IrGenError>>()?;
                    let result_reg = self.new_register();
                    self.current_ir.push(IrInstruction::Call(result_reg, func_name_ident.0.clone(), args_ir));
                    Ok(IrValue::Register(result_reg))
                } else {
                    Err(IrGenError { message: "Function call on non-identifier expression not yet supported in IR gen.".to_string(), span: func_expr.span() })
                }
            }
            // --- OOP IR Generation ---
            Expression::NewInstance(span, class_name_ident, args) => {
                // Lookup resolved class type from symbol table
                if let Some(Type::Class { name, .. }) = self.symbol_table.lookup_class_type(&class_name_ident.0) {
                    let obj_reg = self.new_register();
                    self.current_ir.push(IrInstruction::AllocObject(obj_reg, Type::Class { name: name.clone(), fields: HashMap::new(), methods: HashMap::new(), parent_class: None, implemented_interfaces: Vec::new(), is_abstract: false })); // Allocate object with resolved class type

                    // Generate IR for constructor arguments
                    let args_ir = args.iter().map(|arg| self.generate_expression_ir(arg)).collect::<Result<Vec<IrValue>, IrGenError>>()?;
                    
                    // Call constructor (conceptual: a special method named 'init' or default constructor)
                    // Add 'this' (object pointer) as first arg to constructor
                    let mut constructor_args = vec![IrValue::Register(obj_reg)];
                    constructor_args.extend(args_ir);
                    
                    // The constructor itself would be a method of the class
                    self.current_ir.push(IrInstruction::CallMethod(obj_reg, obj_reg, Identifier("init".to_string(), span.clone()), constructor_args, CallType::Static)); // Call 'init'
                    Ok(IrValue::Register(obj_reg))
                } else {
                    Err(IrGenError { message: format!("Class '{}' not found for instantiation.", class_name_ident.0), span: span.clone() })
                }
            }
            Expression::MethodCall(span, object_expr, method_name_ident, args) => {
                let obj_value = self.generate_expression_ir(object_expr)?; // Get object pointer/value
                let obj_reg = match obj_value { // Ensure it's in a register
                    IrValue::Register(r) => r,
                    _ => {
                        let temp_reg = self.new_register();
                        self.current_ir.push(IrInstruction::Assign(temp_reg, obj_value));
                        temp_reg
                    }
                };

                let method_args_ir = args.iter().map(|arg| self.generate_expression_ir(arg)).collect::<Result<Vec<IrValue>, IrGenError>>()?;
                
                // Determine CallType (static vs. dynamic) based on object_expr's type and method modifiers
                // This requires looking up the method in the object's resolved type from semantic analysis.
                // For conceptual, let's assume dynamic for now for non-private methods and specific call for super.
                let call_type = if let Expression::Super(_) = object_expr.as_ref() { CallType::Super } else { CallType::Dynamic };

                let return_reg = self.new_register();
                self.current_ir.push(IrInstruction::CallMethod(return_reg, obj_reg, method_name_ident.clone(), method_args_ir, call_type));
                Ok(IrValue::Register(return_reg))
            }
            Expression::FieldAccess(span, object_expr, field_name_ident) => {
                let obj_value = self.generate_expression_ir(object_expr)?; // Object must be resolved to a register
                let obj_reg = match obj_value {
                    IrValue::Register(r) => r,
                    _ => {
                        let temp_reg = self.new_register();
                        self.current_ir.push(IrInstruction::Assign(temp_reg, obj_value));
                        temp_reg
                    }
                };
                let field_reg = self.new_register();
                self.current_ir.push(IrInstruction::LoadField(field_reg, obj_reg, field_name_ident.clone()));
                Ok(IrValue::Register(field_reg))
            }
            Expression::This(span) => {
                let this_reg = self.new_register();
                self.current_ir.push(IrInstruction::LoadThis(this_reg));
                Ok(IrValue::Register(this_reg))
            }
            Expression::Super(span) => {
                let super_reg = self.new_register();
                self.current_ir.push(IrInstruction::LoadSuper(super_reg));
                Ok(IrValue::Register(super_reg))
            }
            _ => Err(IrGenError { message: format!("Unsupported expression for IR generation: {:?}", expr), span: expr.span() }),
        }
    }

    // New helper for allocating registers
    fn new_register(&mut self) -> IrRegister {
        let reg = IrRegister(self.next_register);
        self.next_register += 1;
        reg
    }

    // Function to generate vtables/itables after all classes/interfaces are known
    fn generate_oop_metadata_ir(&mut self) -> Result<(), IrGenError> {
        // Iterate through all resolved classes in the symbol table
        for (name_str, class_type) in self.symbol_table.resolved_classes.iter() {
            if let Type::Class { name, fields, methods, parent_class, implemented_interfaces, .. } = class_type {
                self.current_ir.push(IrInstruction::CreateVtable(class_type.clone()));
                // For each implemented interface, create an Itable
                for iface_type in implemented_interfaces {
                    self.current_ir.push(IrInstruction::CreateItable(class_type.clone(), iface_type.clone()));
                }
            }
        }
        Ok(())
    }
}
