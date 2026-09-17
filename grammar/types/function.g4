/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/function.g4
 *
 * Grammar:
 *     Function
 *
 * Status:
 *     CANONICAL function-type grammar component.
 *
 * Purpose:
 *     Defines source-level function/callable TYPE syntax.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns ONLY the syntax of function types.
 *
 * It does not own:
 *
 *   - function declarations;
 *   - function definitions;
 *   - function bodies;
 *   - parameter declarations;
 *   - generic declarations;
 *   - generic bounds;
 *   - closures;
 *   - lambdas;
 *   - async declarations;
 *   - generators;
 *   - FFI declarations;
 *   - ABI selection;
 *   - calling-convention selection;
 *   - effects implementation;
 *   - capability resolution;
 *   - resource allocation;
 *   - hardware selection;
 *   - scheduling;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - backend selection;
 *   - runtime representation.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is the canonical owner of:
 *
 *     functionType
 *     functionTypeParameterList
 *     functionTypeParameter
 *     functionTypeReturn
 *
 * Existing:
 *
 *     grammar/types/function-types.g4
 *
 * MUST NOT remain a second implementation of these rules.
 *
 * It becomes a compatibility/deprecation surface after integration.
 *
 * ============================================================================
 * TYPE SYSTEM BOUNDARY
 * ============================================================================
 *
 * Function parameter and return positions consume the canonical type
 * expression supplied by the type-system composition layer.
 *
 * This file MUST NOT duplicate:
 *
 *     primitiveType
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     referenceType
 *     pointerType
 *     optionalType
 *     resultType
 *     quantumType
 *     resourceType
 *     capabilityType
 *     dependentType
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended architecture is:
 *
 *     canonical lexer
 *          |
 *          v
 *     shared type-expression contract
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     Function              other types
 *          |
 *          v
 *     Types composition
 *
 * There MUST NOT be:
 *
 *     Types -> Function -> Types
 *
 * circular grammar imports.
 *
 * The repository's type-composition layer must therefore expose the canonical
 * type-expression dependency to this delegate through a one-way ANTLR grammar
 * composition boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Function types describe WHAT callable computation accepts and returns.
 *
 * They do not describe WHERE it executes.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     MAX_PARAMETERS
 *     MAX_RETURN_VALUES
 *     MAX_FUNCTION_DEPTH
 *     MAX_GENERIC_PARAMETERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * There is no language-level parameter-count limit.
 *
 * There is no language-level nesting limit.
 *
 * There is no language-level callable-resource limit.
 *
 * Actual compiler/parser resource budgets are implementation policy and must
 * not become source-language semantics.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * A function type such as:
 *
 *     fn(int) -> int
 *
 * must remain meaningful on:
 *
 *     - tiny embedded systems;
 *     - CPUs;
 *     - multicore systems;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - heterogeneous systems;
 *     - distributed systems;
 *     - cloud systems;
 *     - future architectures.
 *
 * No physical execution target is selected by this grammar.
 *
 * ============================================================================
 * FUNCTION TYPE MODEL
 * ============================================================================
 *
 * Canonical form:
 *
 *     fn(parameter-types) -> return-type
 *
 * Examples:
 *
 *     fn() -> Unit
 *
 *     fn(int) -> int
 *
 *     fn(int, float) -> bool
 *
 *     fn(T) -> T
 *
 *     fn(Qubit) -> Measurement
 *
 *     fn(QuantumState) -> ClassicalResult
 *
 *     fn(fn(int) -> int) -> int
 *
 *     fn(int) -> fn(int) -> int
 *
 * ============================================================================
 * ZERO PARAMETERS
 * ============================================================================
 *
 * Empty parameter lists are valid:
 *
 *     fn() -> T
 *
 * ============================================================================
 * MULTIPLE PARAMETERS
 * ============================================================================
 *
 * Parameter lists are ordered and unbounded by language semantics:
 *
 *     fn(A, B, C) -> D
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is accepted:
 *
 *     fn(A,) -> B
 *
 *     fn(
 *         A,
 *         B,
 *     ) -> C
 *
 * ============================================================================
 * HIGHER-ORDER FUNCTIONS
 * ============================================================================
 *
 * Function types can occur anywhere the canonical type-expression grammar
 * permits a type:
 *
 *     fn(fn(A) -> B) -> C
 *
 *     fn(A) -> fn(B) -> C
 *
 *     fn(fn(A) -> B, fn(B) -> C) -> fn(A) -> C
 *
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * Generic declaration syntax belongs to:
 *
 *     grammar/functions/generics.g4
 *
 * Function types may refer to generic type parameters through the canonical
 * type-expression rule:
 *
 *     fn(T) -> T
 *
 * Whether T is actually a type parameter is semantic information.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effects belong to grammar/effects/.
 *
 * This file must not invent a second effect syntax.
 *
 * If effectful function types become part of the normative type identity,
 * they must be integrated through one canonical effect rule and one
 * specification-level contract.
 *
 * ============================================================================
 * ASYNC
 * ============================================================================
 *
 * Async declarations belong to:
 *
 *     grammar/functions/async.g4
 *
 * This file does not independently assign `async` to function-type identity.
 *
 * ============================================================================
 * GENERATORS
 * ============================================================================
 *
 * Generator declarations belong to:
 *
 *     grammar/functions/generators.g4
 *
 * Generator type semantics belong to the type system rather than being
 * duplicated here.
 *
 * ============================================================================
 * FOREIGN FUNCTIONS
 * ============================================================================
 *
 * FFI syntax belongs to:
 *
 *     grammar/interoperability/
 *
 * This grammar does not encode:
 *
 *     C ABI
 *     C++
 *     Python ABI
 *     Rust ABI
 *     WebAssembly ABI
 *     vendor-specific ABI
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are consumed through the canonical type-expression system.
 *
 * Valid examples include:
 *
 *     fn(Qubit) -> Qubit
 *
 *     fn(Qubit) -> Measurement
 *
 *     fn(QuantumState) -> ClassicalResult
 *
 *     fn(QRegister<N>) -> Result
 *
 * This grammar MUST NOT:
 *
 *     - enumerate physical qubits;
 *     - select QPUs;
 *     - define coupling maps;
 *     - perform routing;
 *     - schedule gates;
 *     - implement QEC;
 *     - define ZQN semantics;
 *     - access HAL state.
 *
 * Those responsibilities remain downstream.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM / HDL
 * ============================================================================
 *
 * Function types are domain-neutral.
 *
 * Examples:
 *
 *     fn(ClassicalValue) -> ClassicalValue
 *
 *     fn(Qubit) -> ClassicalValue
 *
 *     fn(ClassicalControl) -> QuantumOperation
 *
 *     fn(HardwareSignal) -> HardwareSignal
 *
 * The function type system does not need separate function grammars for each
 * domain.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Function types MUST NOT contain universal target-selection constructs such
 * as:
 *
 *     cpu_fn
 *     gpu_fn
 *     fpga_fn
 *     qpu_fn
 *     physical_qubit_fn
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST lower to the repository's existing domain-neutral
 * frontend TypeExpr representation.
 *
 * Expected semantic mapping:
 *
 *     functionType
 *         ->
 *     TypeExpr::Function
 *
 * The AST must preserve:
 *
 *     - parameter order;
 *     - every parameter type;
 *     - return type;
 *     - source spans;
 *     - nesting;
 *     - generic references;
 *     - domain-neutral type structure.
 *
 * The grammar MUST NOT introduce:
 *
 *     QuantumFunctionType
 *     GPUFunctionType
 *     HDLFunctionType
 *     VendorFunctionType
 *
 * merely because the parameter/return types belong to those domains.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     "this source text denotes a function type."
 *
 * Semantic analysis establishes:
 *
 *     - type validity;
 *     - generic binding;
 *     - type substitution;
 *     - effect compatibility;
 *     - capability requirements;
 *     - ownership rules;
 *     - borrowing rules;
 *     - resource requirements;
 *     - quantum validity;
 *     - hardware compatibility;
 *     - calling compatibility;
 *     - overload compatibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Function types are source/type-system constructs.
 *
 * They must not create a second domain-specific IR.
 *
 * After semantic analysis they lower through the repository's canonical
 * semantic/IR pipeline.
 *
 * For quantum-containing signatures:
 *
 *     function type
 *          |
 *          v
 *     semantic type model
 *          |
 *          v
 *     quantum semantic operations where applicable
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may subsequently determine:
 *
 *     - calling convention;
 *     - ABI;
 *     - register allocation;
 *     - stack representation;
 *     - closure representation;
 *     - device placement;
 *     - accelerator mapping;
 *     - distributed execution;
 *     - quantum lowering;
 *     - hardware realization.
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is outside grammar ownership.
 *
 * A function type does not prescribe:
 *
 *     - stack layout;
 *     - register layout;
 *     - heap representation;
 *     - closure object layout;
 *     - function pointer representation;
 *     - device invocation mechanism.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors must identify:
 *
 *     - unexpected token;
 *     - expected function-type delimiter;
 *     - malformed parameter list;
 *     - missing return arrow;
 *     - missing return type;
 *     - malformed trailing comma.
 *
 * Semantic diagnostics must remain outside this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parameter order is source order.
 *
 * No parser action may reorder parameters.
 *
 * No parser action may infer target-specific information.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar must not:
 *
 *     - execute functions;
 *     - evaluate type-level code;
 *     - access filesystem/network resources;
 *     - resolve external packages;
 *     - access hardware;
 *     - invoke compiler backends.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     fn() -> Unit
 *     fn(int) -> int
 *     fn(int, float) -> bool
 *     fn(T) -> T
 *     fn(Qubit) -> Measurement
 *     fn(fn(int) -> int) -> int
 *     fn(int) -> fn(int) -> int
 *     fn(int,) -> int
 *
 * Negative:
 *
 *     fn
 *     fn(
 *     fn()
 *     fn(int)
 *     fn(,) -> int
 *     fn(int,, float) -> bool
 *     fn(int -> int
 *     fn(int) int
 *
 * Boundary:
 *
 *     zero parameters;
 *     one parameter;
 *     many parameters;
 *     deeply nested function types;
 *     deeply nested generic types;
 *     function returning function;
 *     function accepting function.
 *
 * Scalability:
 *
 *     parameter count must not be grammar-limited;
 *     nesting must not be grammar-limited;
 *     generic references must not be grammar-limited;
 *     domain types must not be grammar-limited.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] functionType is the sole canonical function-type rule;
 *     [ ] no duplicate function-type authority exists;
 *     [ ] canonical lexer vocabulary is used;
 *     [ ] parameter types use canonical type-expression syntax;
 *     [ ] return types use canonical type-expression syntax;
 *     [ ] zero-parameter functions work;
 *     [ ] arbitrary parameter lists work;
 *     [ ] trailing commas work;
 *     [ ] nested function types work;
 *     [ ] generic type parameters work;
 *     [ ] quantum types work;
 *     [ ] classical types work;
 *     [ ] HDL/resource types work;
 *     [ ] no hardware limits are encoded;
 *     [ ] no physical resource IDs are encoded;
 *     [ ] no second AST is introduced;
 *     [ ] TypeExpr::Function mapping is documented;
 *     [ ] semantic ownership is documented;
 *     [ ] IR ownership is documented;
 *     [ ] compiler ownership is documented;
 *     [ ] runtime ownership is documented;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] determinism tests exist;
 *     [ ] hard-coding audit passes.
 *
 * ============================================================================
 */

parser grammar Function;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC FUNCTION-TYPE RULE
 * ============================================================================
 *
 * Canonical form:
 *
 *     fn(parameter-types) -> return-type
 *
 * The actual `typeExpression` rule is supplied by the shared canonical
 * type-system composition layer. It MUST NOT be redefined here.
 */

functionType
    : K_FN
      LPAREN
      functionTypeParameterList?
      RPAREN
      THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * PARAMETER LIST
 * ============================================================================
 *
 * No fixed parameter-count limit exists.
 */

functionTypeParameterList
    : functionTypeParameter
      (COMMA functionTypeParameter)*
      COMMA?
    ;


/*
 * ============================================================================
 * PARAMETER TYPE
 * ============================================================================
 */

functionTypeParameter
    : typeExpression
    ;