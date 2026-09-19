/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/functions.g4
 *
 * Role:
 *     Canonical parser grammar for source-level Zamani functions.
 *
 * Status:
 *     Production grammar contract
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust implementation code.
 *     Compiler/frontend/runtime Rust MUST remain safe Rust only.
 *     No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - function declarations;
 *   - function definitions;
 *   - function names;
 *   - function modifiers;
 *   - function generic-list attachment;
 *   - function parameter-list attachment;
 *   - function return-type attachment;
 *   - function effect attachment;
 *   - function contract attachment;
 *   - function body/prototype termination;
 *   - function signature syntax;
 *   - the structural boundary between a function declaration and its body.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - generic-parameter internals;
 *   - parameter internals;
 *   - type-expression syntax;
 *   - expression syntax;
 *   - statement syntax;
 *   - block syntax;
 *   - module/name resolution;
 *   - overload resolution;
 *   - type inference;
 *   - generic substitution;
 *   - trait solving;
 *   - ownership;
 *   - borrowing;
 *   - lifetime analysis;
 *   - effect checking;
 *   - capability checking;
 *   - resource allocation;
 *   - hardware discovery;
 *   - target selection;
 *   - ABI selection;
 *   - calling-convention selection;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution;
 *   - quantum IR.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * A function describes portable computation and semantic intent.
 *
 * The grammar therefore imposes NO language-level finite limit on:
 *
 *   - number of functions;
 *   - parameter count;
 *   - generic parameter count;
 *   - generic bound count;
 *   - source program size;
 *   - recursion depth;
 *   - number of calls;
 *   - number of computational domains;
 *   - number of resources;
 *   - number of quantum objects;
 *   - number of distributed resources.
 *
 * Repetition is represented by grammar repetition operators.
 *
 * Implementation resource limits, when necessary, MUST be:
 *
 *   - explicit;
 *   - configurable;
 *   - documented;
 *   - diagnosable;
 *   - external to language semantics.
 *
 * The grammar MUST NOT encode:
 *
 *   MAX_PARAMETERS
 *   MAX_FUNCTIONS
 *   MAX_THREADS
 *   MAX_CORES
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_QUBITS
 *   MAX_MEMORY
 *   MAX_NODES
 *   MAX_DEVICES
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * A function may operate over:
 *
 *   - classical values;
 *   - quantum values;
 *   - logical qubits;
 *   - tensors;
 *   - distributed values;
 *   - hardware abstractions;
 *   - resources;
 *   - accelerator abstractions;
 *   - future computational domains.
 *
 * The function grammar does not special-case those domains.
 *
 * Domain meaning is obtained from the type/semantic system.
 *
 * Quantum source eventually follows:
 *
 *   Zamani source
 *       ->
 *   frontend AST
 *       ->
 *   semantic analysis
 *       ->
 *   quantum::ir
 *       ->
 *   optimization
 *       ->
 *   routing
 *       ->
 *   scheduling
 *       ->
 *   QEC / resilience
 *       ->
 *   ZQN
 *       ->
 *   HAL
 *       ->
 *   target realization
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is supplied by:
 *
 *     grammar/lexer/tokens.g4
 *
 * and the canonical lexer composition.
 *
 * Function syntax therefore consumes canonical tokens such as:
 *
 *     FN
 *     PUBLIC
 *     PUB
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *     STATIC
 *     CONST
 *     ASYNC
 *     EXTERN
 *     ABSTRACT
 *     FINAL
 *     VIRTUAL
 *     OVERRIDE
 *     INLINE
 *     VOLATILE
 *     PURE
 *     WITH
 *     EFFECT
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     THIN_ARROW
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * Imported parser grammars own:
 *
 *     FunctionGenerics
 *         -> functionGenericParameters
 *
 *     Parameters
 *         -> parameterList
 *
 *     Types
 *         -> typeExpression
 *
 *     Expressions
 *         -> expression
 *
 *     Statements
 *         -> block
 *
 * Those rules MUST NOT be duplicated here.
 *
 * ============================================================================
 */

parser grammar Functions;

options {
    tokenVocab = ZamaniLexer;
}

import FunctionGenerics, Parameters, Types, Expressions, Statements;


/* ============================================================================
 * 1. CANONICAL FUNCTION DECLARATION
 * ============================================================================
 *
 * Complete source-level function declaration.
 *
 * Examples:
 *
 *     fn main() {
 *     }
 *
 *     pub fn add(a: Int, b: Int) -> Int {
 *         return a + b;
 *     }
 *
 *     async fn compute(input: Data) -> Result {
 *         ...
 *     }
 *
 *     fn identity<T>(value: T) -> T {
 *         return value;
 *     }
 *
 *     extern fn foreign_call(value: Int) -> Int;
 *
 * A function may be defined with a body or declared without a body.
 */
functionDeclaration
    : functionModifier*
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
      functionImplementation
    ;


/* ============================================================================
 * 2. FUNCTION NAME
 * ============================================================================
 *
 * A function declaration introduces an unqualified local name.
 *
 * Module/package qualification belongs to the surrounding declaration/module
 * system and name-resolution layer.
 */
functionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 3. FUNCTION MODIFIERS
 * ============================================================================
 *
 * These are source-level declaration properties.
 *
 * Their semantic legality is checked downstream.
 *
 * They do NOT select hardware, ABI, registers, devices, QPUs, CPUs, GPUs,
 * memory banks, physical qubits, topology, or deployment placement.
 */
functionModifier
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | STATIC
    | CONST
    | ASYNC
    | EXTERN
    | ABSTRACT
    | FINAL
    | VIRTUAL
    | OVERRIDE
    | INLINE
    | VOLATILE
    | PURE
    ;


/* ============================================================================
 * 4. RETURN TYPE
 * ============================================================================
 *
 * The return type is owned semantically by the type system.
 *
 * This file only establishes:
 *
 *     -> TypeExpression
 *
 * The typeExpression rule comes from Types.
 */
functionReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 5. EFFECTS
 * ============================================================================
 *
 * Function effects are expressed as a list of symbolic effect references.
 *
 * Example:
 *
 *     fn read() with effects {
 *         io
 *     }
 *
 *     fn hybrid() with effects {
 *         quantum,
 *         io,
 *         distributed
 *     }
 *
 * The grammar does not enumerate every possible effect.
 *
 * That is essential for future extensibility and POCO-REAF.
 */
functionEffectClause
    : WITH
      EFFECTS
      LBRACE
      functionEffectList?
      RBRACE
    ;


/*
 * Effect list.
 *
 * No finite number of effects is imposed.
 */
functionEffectList
    : functionEffectReference
      (COMMA functionEffectReference)*
      COMMA?
    ;


/*
 * Effect references are symbolic names.
 *
 * Qualified names permit future/domain-specific effect namespaces without
 * changing this function grammar.
 *
 * Examples:
 *
 *     io
 *     quantum
 *     network
 *     distributed
 *     security
 *     quantum::measurement
 *     hardware::control
 */
functionEffectReference
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 6. FUNCTION CONTRACTS
 * ============================================================================
 *
 * Contracts remain source-level expressions.
 *
 * Their logical interpretation is owned by semantic analysis.
 *
 * Example:
 *
 *     fn sqrt(x: Float) -> Float
 *     contract {
 *         requires(x >= 0);
 *         ensures(result >= 0);
 *     }
 *     {
 *         ...
 *     }
 *
 * Multiple contract clauses are retained in source order.
 */
functionContractClause
    : CONTRACT
      LBRACE
      functionContractItem*
      RBRACE
    ;


/*
 * A contract item is one of the canonical contract predicates.
 */
functionContractItem
    : functionRequiresContract
    | functionEnsuresContract
    | functionInvariantContract
    ;


/*
 * Preconditions.
 */
functionRequiresContract
    : REQUIRES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/*
 * Postconditions.
 */
functionEnsuresContract
    : ENSURES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/*
 * Invariants.
 */
functionInvariantContract
    : INVARIANT
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 7. FUNCTION IMPLEMENTATION
 * ============================================================================
 *
 * A function either:
 *
 *   - contains a block body;
 *   - terminates as a declaration/prototype.
 *
 * Whether a body-less declaration is legal is determined by semantic context.
 *
 * Examples:
 *
 *     fn add(a: Int, b: Int) -> Int {
 *         return a + b;
 *     }
 *
 *     extern fn host_call(value: Int) -> Int;
 *
 *     abstract fn operation(value: Value) -> Result;
 */
functionImplementation
    : block
    | SEMICOLON
    ;


/* ============================================================================
 * 8. SIGNATURE
 * ============================================================================
 *
 * Signature syntax is reusable by semantic constructs such as:
 *
 *   - interfaces;
 *   - traits;
 *   - extern declarations;
 *   - abstract declarations;
 *   - implementation contracts.
 *
 * It deliberately excludes a function body.
 */
functionSignature
    : functionModifier*
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 9. DECLARATION-ONLY FUNCTION
 * ============================================================================
 *
 * This explicit rule is useful to declaration/trait/interface integration.
 *
 * It does not introduce another syntax.
 */
functionPrototype
    : functionSignature
    ;


/* ============================================================================
 * 10. DEFINITION-ONLY FUNCTION
 * ============================================================================
 *
 * A definition requires a body.
 */
functionDefinition
    : functionModifier*
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
      block
    ;


/* ============================================================================
 * 11. FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * `extern fn ...;` uses the same canonical function signature.
 *
 * This grammar deliberately does NOT define:
 *
 *   - ABI names;
 *   - foreign-language names;
 *   - C/C++/Rust/Python calling conventions;
 *   - symbol mangling;
 *   - binary layout;
 *   - platform ABI;
 *   - register conventions.
 *
 * Those belong to:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *     semantic analysis
 *     compiler lowering
 *
 * The generic source-level function boundary remains the same.
 */
foreignFunctionDeclaration
    : EXTERN
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
      SEMICOLON
    ;


/* ============================================================================
 * 12. ASYNC FUNCTION INTEGRATION
 * ============================================================================
 *
 * Async functions use the canonical ASYNC modifier.
 *
 * The grammar does not define:
 *
 *   - executor;
 *   - scheduler;
 *   - runtime;
 *   - thread count;
 *   - task placement;
 *   - hardware execution policy.
 *
 * Those are downstream concerns.
 */
asyncFunctionDeclaration
    : ASYNC
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
      functionImplementation
    ;


/* ============================================================================
 * 13. COMPILE-TIME FUNCTION INTEGRATION
 * ============================================================================
 *
 * Compile-time execution is NOT a separate function grammar.
 *
 * Existing compile-time function facilities must reuse the canonical function
 * declaration/signature structure.
 *
 * The compile-time subsystem determines whether a function is evaluable during
 * compilation.
 *
 * It does not belong to this parser rule.
 *
 * This alias gives compile-time tooling a stable grammar integration point
 * without creating a second function syntax.
 */
compileTimeFunctionDeclaration
    : functionDeclaration
    ;


/* ============================================================================
 * 14. FUNCTION SIGNATURE COMPONENT
 * ============================================================================
 *
 * This rule exposes the complete reusable signature without implementation.
 *
 * It is useful to interface/trait/foreign/callable declaration grammars.
 */
functionSignatureCore
    : functionModifier*
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionEffectClause?
      functionContractClause*
    ;


/* ============================================================================
 * 15. FUNCTION DECLARATION WITH EXPLICIT BODY
 * ============================================================================
 *
 * This is intentionally a distinct named rule for callers that need to require
 * a definition.
 */
functionBodyDeclaration
    : functionSignatureCore
      block
    ;


/* ============================================================================
 * 16. FUNCTION DECLARATION WITH EXPLICIT PROTOTYPE
 * ============================================================================
 *
 * This is intentionally a distinct named rule for callers that require a
 * declaration without implementation.
 */
functionPrototypeDeclaration
    : functionSignatureCore
      SEMICOLON
    ;


/* ============================================================================
 * 17. SOURCE-LEVEL FUNCTION KIND INTEGRATION
 * ============================================================================
 *
 * Function kind is represented by existing modifiers rather than by an
 * expanding keyword catalogue.
 *
 * Do NOT add rules such as:
 *
 *     gpuFunction
 *     qpuFunction
 *     cpuFunction
 *     fpgaFunction
 *     acceleratorFunction
 *     vendorFunction
 *
 * A function's semantic domain is determined by its types, effects,
 * capabilities, resources, declarations, and semantic analysis.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and prevents the language grammar from becoming coupled to current hardware.
 */


/* ============================================================================
 * 18. CANONICAL LOWERING CONTRACT
 * ============================================================================
 *
 * Every function declaration must eventually map through:
 *
 *     grammar
 *       ->
 *     frontend AST
 *       ->
 *     semantic function model
 *       ->
 *     canonical semantic representation / IR
 *
 * Classical function:
 *
 *     function
 *       ->
 *     classical semantic model
 *       ->
 *     classical IR
 *
 * Quantum-capable function:
 *
 *     function
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum operations
 *       ->
 *     quantum::ir
 *
 * HDL/hardware-aware function:
 *
 *     function
 *       ->
 *     AST
 *       ->
 *     hardware/co-design semantic model
 *       ->
 *     canonical hardware/HDL representation
 *
 * The function grammar never lowers directly to:
 *
 *     - LLVM;
 *     - QIR;
 *     - MLIR;
 *     - vendor assembly;
 *     - QPU instructions;
 *     - physical qubit mappings;
 *     - FPGA primitives;
 *     - CUDA kernels;
 *     - scheduling decisions.
 *
 * Those are downstream lowering targets.
 */


/* ============================================================================
 * 19. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Functions may use resource/capability declarations through:
 *
 *     - parameter types;
 *     - return types;
 *     - effects;
 *     - contracts;
 *     - surrounding declarations;
 *     - resource/capability attributes owned elsewhere.
 *
 * This grammar does not invent:
 *
 *     requires_8_cores
 *     requires_32_qubits
 *     requires_gpu_0
 *     requires_qpu_1
 *
 * Portable requirements belong to the resource/capability system.
 *
 * For example, a semantic layer may interpret:
 *
 *     type QuantumResource
 *
 * or:
 *
 *     capability("quantum.measurement")
 *
 * without changing this grammar.
 */


/* ============================================================================
 * 20. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser/frontend infrastructure must preserve source spans for:
 *
 *     - function modifier;
 *     - function name;
 *     - generic parameter list;
 *     - parameter list;
 *     - return type;
 *     - effects;
 *     - contracts;
 *     - body/prototype terminator.
 *
 * This permits diagnostics such as:
 *
 *     duplicate modifier;
 *     invalid modifier combination;
 *     missing function name;
 *     malformed generic parameters;
 *     malformed parameter list;
 *     missing return type;
 *     malformed effect clause;
 *     malformed contract;
 *     missing body;
 *     invalid declaration termination.
 *
 * Semantic diagnostics remain outside this grammar.
 */


/* ============================================================================
 * 21. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Identical source text and identical lexer configuration must produce the
 * same parse structure.
 *
 * Function modifier order is preserved by the parse tree.
 *
 * Generic parameter order is preserved by FunctionGenerics.
 *
 * Parameter order is preserved by Parameters.
 *
 * Contract order is preserved by functionContractClause*.
 *
 * Effect order is preserved by functionEffectList.
 *
 * No unordered semantic representation is created here.
 */


/* ============================================================================
 * 22. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source forms that remain part of the canonical specification must
 * continue to parse without requiring target-specific extensions.
 *
 * In particular:
 *
 *     fn name(...)
 *     pub fn name(...)
 *     public fn name(...)
 *     private fn name(...)
 *     async fn name(...)
 *     extern fn name(...);
 *     fn name<T>(...)
 *     fn name(value: Type)
 *     fn name(value: Type) -> ReturnType
 *
 * remain within the canonical function syntax model.
 *
 * Any change to:
 *
 *     token names;
 *     modifier spellings;
 *     generic syntax;
 *     parameter syntax;
 *     return syntax;
 *     effect syntax;
 *     contract syntax
 *
 * is a language compatibility change and must be accompanied by the repository
 * compatibility policy and conformance tests.
 */


/* ============================================================================
 * 23. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     - hardware IDs;
 *     - physical addresses;
 *     - CPU counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - QPU counts;
 *     - qubit limits;
 *     - memory limits;
 *     - node limits;
 *     - thread limits;
 *     - tensor limits;
 *     - topology limits;
 *     - vendor-specific instruction names.
 *
 * It therefore remains independent of target size and target generation.
 */


/* ============================================================================
 * 24. COMPLETION CRITERIA
 * ============================================================================
 *
 * functions.g4 is complete only when all of the following are true:
 *
 * [x] Function declaration ownership is unique.
 * [x] Function definition ownership is unique.
 * [x] Function names use the canonical identifier grammar.
 * [x] Generic declarations are delegated to FunctionGenerics.
 * [x] Parameter declarations are delegated to Parameters.
 * [x] Types are delegated to Types.
 * [x] Expressions are delegated to Expressions.
 * [x] Blocks are delegated to Statements.
 * [x] Canonical lexer token names are used.
 * [x] No K_* legacy token aliases remain.
 * [x] No lexer rules are declared here.
 * [x] No machine/resource limits are encoded.
 * [x] No quantum gate catalogue is encoded.
 * [x] No duplicate quantum IR is introduced.
 * [x] Foreign ABI details remain outside this file.
 * [x] Compile-time evaluation remains outside this file.
 * [x] Async runtime scheduling remains outside this file.
 * [x] Contract semantics remain outside this file.
 * [x] Effect semantics remain outside this file.
 * [x] Positive syntax tests exist.
 * [x] Negative syntax tests exist.
 * [x] Boundary tests exist.
 * [x] Scalability tests exist.
 * [x] Compatibility tests exist.
 *
 * ============================================================================
 */