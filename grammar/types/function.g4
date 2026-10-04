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
 *     CANONICAL FUNCTION-TYPE DELEGATE GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY source-level FUNCTION TYPE syntax.
 *
 * Canonical examples:
 *
 *     fn() -> Unit
 *     fn(int) -> int
 *     fn(int, float) -> bool
 *     fn(T) -> T
 *     fn(A, B) -> C
 *     fn(fn(int) -> int) -> int
 *     fn(int) -> fn(int) -> int
 *     fn(fn(A) -> B, fn(B) -> C) -> fn(A) -> C
 *
 * Function types are domain-neutral.
 *
 * The same function-type syntax can describe computation involving:
 *
 *     classical values
 *     quantum values
 *     hybrid values
 *     HDL values
 *     hardware abstractions
 *     tensors
 *     distributed values
 *     data values
 *     AI/model values
 *     foreign/interoperability values
 *     future domain-defined values
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *                  |
 *                  v
 *     grammar/antlr/ZamaniParser.g4
 *                  |
 *                  v
 *     Types
 *                  |
 *          +-------+-------+
 *          |               |
 *          v               v
 *      Function         other type delegates
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     domain-neutral TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir          HDL/hardware
 *      semantics              semantics             semantics
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         canonical semantic IR
 *                                 |
 *                         optimization/lowering
 *                                 |
 *                         routing/scheduling
 *                                 |
 *                         resilience/QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     functionType
 *     functionTypeParameterList
 *     functionTypeParameter
 *     functionTypeReturn
 *
 * THIS FILE DOES NOT OWN:
 *
 *     function declarations
 *     function definitions
 *     function names
 *     function parameters with names
 *     generic declarations
 *     generic bounds
 *     where clauses
 *     function contracts
 *     function effects
 *     async declarations
 *     generators
 *     closures
 *     lambdas
 *     foreign functions
 *     calling conventions
 *     ABI declarations
 *     function bodies
 *     expressions
 *     statements
 *     primitive types
 *     named types
 *     generic type applications
 *     tuple types
 *     array types
 *     slice types
 *     references
 *     pointers
 *     quantum operations
 *     hardware selection
 *     resource allocation
 *     capability discovery
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime representation
 *
 * Those responsibilities remain with their canonical owners.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * This is a DELEGATE grammar.
 *
 * The canonical type composition root is:
 *
 *     grammar/types/types.g4
 *
 * `Types` imports this grammar.
 *
 * `Function` intentionally does NOT import `Types`.
 *
 * This prevents a circular grammar dependency:
 *
 *     Types -> Function
 *          X
 *     Function -> Types
 *
 * Function-type parameters and return values refer to the canonical
 * `typeExpression` rule supplied by the delegating `Types` grammar.
 *
 * ANTLR grammar composition permits delegate rules to resolve references
 * through the delegating grammar.
 *
 * Therefore this file MUST NOT define:
 *
 *     typeExpression
 *     typeCore
 *     primitiveType
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *
 * or any other complete type-system composition rule.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * Tokens are owned by:
 *
 *     grammar/lexer/
 *
 * and exposed to the parser through:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical parser root is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * which uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This delegate therefore MUST NOT:
 *
 *     - define lexer rules;
 *     - define token aliases;
 *     - introduce K_FN;
 *     - introduce alternate arrow tokens;
 *     - introduce alternate parenthesis tokens;
 *     - introduce alternate comma tokens.
 *
 * The canonical tokens consumed here are the same vocabulary already used by
 * `grammar/types/types.g4`:
 *
 *     FN
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     THIN_ARROW
 *
 * ============================================================================
 * FUNCTION TYPE MODEL
 * ============================================================================
 *
 * The semantic source-level model is:
 *
 *     TypeExpr::Function(
 *         Vec<TypeExpr>,
 *         Box<TypeExpr>
 *     )
 *
 * Therefore:
 *
 *     fn(A, B) -> C
 *
 * means:
 *
 *     Function(
 *         [A, B],
 *         C
 *     )
 *
 * The grammar preserves:
 *
 *     parameter ordering
 *     parameter type structure
 *     return type structure
 *     nesting
 *     source spans through the parser/AST layer
 *
 * It MUST NOT create a competing:
 *
 *     FunctionType
 *     CallableType
 *     LambdaType
 *     ClosureType
 *
 * AST hierarchy merely to represent this syntax.
 *
 * ============================================================================
 * FUNCTION DECLARATION SEPARATION
 * ============================================================================
 *
 * This file handles:
 *
 *     fn(A, B) -> C
 *
 * as a TYPE.
 *
 * It does NOT handle:
 *
 *     fn name(a: A, b: B) -> C { ... }
 *
 * as a declaration.
 *
 * Named function declarations belong to:
 *
 *     grammar/functions/functions.g4
 *
 * Parameter declarations belong to:
 *
 *     grammar/functions/parameters.g4
 *
 * Generic declarations belong to:
 *
 *     grammar/functions/generics.g4
 *
 * Return clauses for named functions belong to:
 *
 *     grammar/functions/return-types.g4
 *
 * Function bodies belong to the statement/block architecture.
 *
 * This distinction is essential because:
 *
 *     function TYPE
 *
 * and:
 *
 *     function DECLARATION
 *
 * have different semantic ownership.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     fn(parameter-types) -> return-type
 *
 * The return arrow is REQUIRED.
 *
 * Therefore:
 *
 *     fn() -> int
 *
 * is valid.
 *
 *     fn(int) -> int
 *
 * is valid.
 *
 *     fn(int, float) -> bool
 *
 * is valid.
 *
 *     fn()
 *
 * is NOT a complete function type.
 *
 *     fn(int)
 *
 * is NOT a complete function type.
 *
 * This prevents ambiguity between:
 *
 *     function type
 *
 * and:
 *
 *     parenthesized type
 *
 * and ensures every function type has a complete semantic result type.
 *
 * ============================================================================
 * ZERO PARAMETERS
 * ============================================================================
 *
 * Zero-parameter function types are valid:
 *
 *     fn() -> Unit
 *     fn() -> int
 *     fn() -> Qubit
 *     fn() -> Tensor<Value>
 *
 * Empty parameter lists do not represent an absent function parameter.
 *
 * They represent a function with zero parameters.
 *
 * ============================================================================
 * PARAMETER LIST
 * ============================================================================
 *
 * Parameter types are ordered.
 *
 * Example:
 *
 *     fn(A, B, C) -> D
 *
 * has parameter sequence:
 *
 *     A
 *     B
 *     C
 *
 * The grammar MUST NOT reorder parameters.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is permitted:
 *
 *     fn(A,) -> B
 *
 *     fn(
 *         A,
 *         B,
 *     ) -> C
 *
 * The trailing comma is punctuation only.
 *
 * It does not create another parameter.
 *
 * Therefore:
 *
 *     fn(A,) -> B
 *
 * has exactly one parameter.
 *
 * ============================================================================
 * HIGHER-ORDER FUNCTIONS
 * ============================================================================
 *
 * Function types may recursively contain function types because
 * `functionTypeParameter` consumes the canonical `typeExpression`.
 *
 * Valid examples:
 *
 *     fn(fn(int) -> int) -> int
 *
 *     fn(int) -> fn(int) -> int
 *
 *     fn(
 *         fn(A) -> B,
 *         fn(B) -> C,
 *     ) -> fn(A) -> C
 *
 * The grammar contains no semantic nesting ceiling.
 *
 * ============================================================================
 * FUNCTION RETURN TYPES
 * ============================================================================
 *
 * The return type is the canonical `typeExpression`.
 *
 * Therefore return types may be:
 *
 *     primitive types
 *     named types
 *     generic types
 *     tuple types
 *     arrays
 *     slices
 *     references
 *     pointers
 *     Result types
 *     optional types
 *     quantum types
 *     temporal types
 *     dependent types
 *     other future source-level types
 *     nested function types
 *
 * The function grammar does not need to be changed when another independent
 * type category is added to `Types`.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic type parameters are resolved semantically.
 *
 * Examples:
 *
 *     fn(T) -> T
 *
 *     fn(Vec<T>) -> Vec<T>
 *
 *     fn(Result<T, E>) -> T
 *
 * This file does NOT define generic declarations or bounds.
 *
 * Generic declaration ownership remains:
 *
 *     grammar/functions/generics.g4
 *
 * Generic application ownership remains:
 *
 *     grammar/types/generic.g4
 *
 * ============================================================================
 * TYPE QUALIFIER INTEGRATION
 * ============================================================================
 *
 * Function parameter and return types consume the complete canonical
 * `typeExpression`.
 *
 * Consequently, any type qualifiers already accepted by the canonical type
 * system remain available without duplicating their syntax here.
 *
 * Examples may include source-defined forms such as:
 *
 *     fn(linear T) -> T
 *
 * only if the canonical `Types.typeExpression` accepts them.
 *
 * This file does not independently decide which type qualifiers exist.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A function type itself does not invent an effect system.
 *
 * If Zamani's canonical type/effect architecture later makes effects part of
 * function type identity, the effect qualification must be attached through
 * the existing canonical effect grammar.
 *
 * This file MUST NOT introduce:
 *
 *     functionEffect
 *     quantumEffect
 *     aiEffect
 *     gpuEffect
 *
 * as a second effect vocabulary.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * and their established type-system integration.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * `async` function declarations are owned by the function declaration
 * subsystem.
 *
 * Async execution semantics are NOT encoded in this function-type grammar.
 *
 * This file therefore does not define:
 *
 *     asyncFunctionType
 *     asyncReturnType
 *     futureFunctionType
 *
 * as competing type systems.
 *
 * If async function types become a normative part of type identity, that
 * identity must be specified and integrated with the canonical type model
 * rather than creating a parallel function-type grammar.
 *
 * ============================================================================
 * CLOSURE / LAMBDA INTEGRATION
 * ============================================================================
 *
 * A closure or lambda may have a function type.
 *
 * Their expression syntax belongs to:
 *
 *     grammar/functions/closures.g4
 *     grammar/functions/lambdas.g4
 *
 * Their semantic type is represented by the canonical:
 *
 *     TypeExpr::Function
 *
 * where applicable.
 *
 * This file does not define closure capture semantics.
 *
 * ============================================================================
 * GENERATOR INTEGRATION
 * ============================================================================
 *
 * Generator syntax belongs to:
 *
 *     grammar/functions/generators.g4
 *
 * Generator semantic types are resolved downstream.
 *
 * This file does not create:
 *
 *     generatorType
 *
 * as an alternative function-type universe.
 *
 * ============================================================================
 * FOREIGN / FFI INTEGRATION
 * ============================================================================
 *
 * Foreign function declarations belong to:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * Function types remain language-level semantic types.
 *
 * Foreign ABI information is not encoded here.
 *
 * The type grammar therefore does not select:
 *
 *     C ABI
 *     C++ ABI
 *     Rust ABI
 *     WebAssembly ABI
 *     vendor ABI
 *
 * Calling conventions belong to the interoperability/function declaration
 * subsystem.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum function types are ordinary function types whose parameter or
 * return types happen to be quantum types.
 *
 * Examples:
 *
 *     fn(Qubit) -> Qubit
 *
 *     fn(Qubit) -> Measurement
 *
 *     fn(QuantumState) -> ClassicalResult
 *
 *     fn(QRegister<N>) -> Result<State, Error>
 *
 * This grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - allocate qubits;
 *     - inspect coupling maps;
 *     - select QPUs;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - select calibration;
 *     - perform QEC;
 *     - construct ZQN;
 *     - access HAL state.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       |
 *       v
 *     TypeExpr::Function
 *       |
 *       v
 *     semantic function/type model
 *       |
 *       v
 *     quantum semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Function types may contain HDL or hardware semantic types.
 *
 * Examples:
 *
 *     fn(Signal<T>) -> Signal<T>
 *
 *     fn(HardwareValue) -> HardwareValue
 *
 *     fn(Control) -> Operation
 *
 *     fn(Register<T>) -> Register<T>
 *
 * This grammar does not encode:
 *
 *     bus width
 *     register width
 *     register count
 *     port count
 *     pipeline depth
 *     clock count
 *     FPGA capacity
 *     ASIC capacity
 *     accelerator count
 *     physical address width
 *     device identifiers
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Function types can describe:
 *
 *     fn(Tensor<T>) -> Tensor<U>
 *
 *     fn(Model<Input, Output>) -> Prediction
 *
 *     fn(Dataset<Record>) -> Model<Input, Output>
 *
 *     fn(Distribution<T>) -> Probability<T>
 *
 * No machine-learning algorithm becomes part of function-type syntax.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Function types remain independent of the number of:
 *
 *     nodes
 *     processes
 *     workers
 *     channels
 *     devices
 *     accelerators
 *
 * A distributed semantic layer may use function types to describe:
 *
 *     tasks
 *     services
 *     handlers
 *     workers
 *     transformations
 *     collective operations
 *
 * Actual distribution is downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A function type does not itself allocate resources.
 *
 * Resource and capability requirements may be associated with a function
 * declaration, operation, contract, policy, or semantic callable value by
 * their canonical owners.
 *
 * This grammar therefore does NOT introduce:
 *
 *     gpuFunctionType
 *     qpuFunctionType
 *     cpuFunctionType
 *     fpgaFunctionType
 *     distributedFunctionType
 *
 * Resource requirements remain target-independent source intent.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This file deliberately contains no language-level limits on:
 *
 *     parameter count
 *     return type complexity
 *     generic nesting
 *     function nesting
 *     type nesting
 *     quantum resource quantity
 *     CPU quantity
 *     GPU quantity
 *     FPGA quantity
 *     QPU quantity
 *     node quantity
 *     memory capacity
 *     tensor rank
 *     tensor dimensions
 *     register width
 *     network size
 *     device quantity
 *
 * In particular, this file MUST NOT contain:
 *
 *     MAX_FUNCTION_PARAMETERS
 *     MAX_FUNCTION_PARAMETER_COUNT
 *     MAX_RETURN_VALUES
 *     MAX_FUNCTION_DEPTH
 *     MAX_GENERIC_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Practical implementation limits may exist for parser/compiler resource
 * protection, but such limits are implementation policy and MUST NOT alter
 * source-language meaning.
 *
 * A resource-exhaustion diagnostic must remain distinguishable from a syntax
 * or semantic type diagnostic.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is deterministic with respect to source order.
 *
 * It MUST preserve:
 *
 *     parameter ordering
 *     type nesting
 *     return-type structure
 *
 * No grammar action may:
 *
 *     reorder parameters
 *     inspect hardware
 *     inspect filesystem state
 *     inspect network state
 *     inspect time
 *     invoke runtime behavior
 *     perform random selection
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a function type must not:
 *
 *     execute code;
 *     evaluate type-level programs;
 *     access files;
 *     access network resources;
 *     discover hardware;
 *     allocate physical resources;
 *     invoke foreign code;
 *     invoke compiler backends;
 *     perform reflection with side effects.
 *
 * This file contains no embedded target-language actions.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify malformed function-type structure.
 *
 * Required structural errors include:
 *
 *     missing `fn`
 *     missing `(`
 *     missing `)`
 *     missing `->`
 *     missing return type
 *     malformed parameter list
 *     empty parameter slot
 *     repeated comma
 *     malformed nested type
 *
 * Examples of malformed syntax:
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
 * Semantic diagnostics do NOT belong here.
 *
 * For example:
 *
 *     fn(UnknownType) -> UnknownType
 *
 * is structurally valid and becomes a semantic/name-resolution issue.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps to the existing domain-neutral frontend type model:
 *
 *     TypeExpr::Function(
 *         parameters,
 *         return_type
 *     )
 *
 * where:
 *
 *     parameters: Vec<TypeExpr>
 *
 *     return_type: Box<TypeExpr>
 *
 * The grammar must preserve the exact ordered structure needed by that model.
 *
 * No function-type AST extension is required by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     "this source text denotes a function type."
 *
 * Semantic analysis establishes:
 *
 *     - whether every referenced type exists;
 *     - whether generic parameters are valid;
 *     - whether type bounds are satisfied;
 *     - whether parameter/return variance rules apply;
 *     - whether ownership rules are satisfied;
 *     - whether linear/affine semantics are satisfied;
 *     - whether effects are compatible;
 *     - whether capabilities are sufficient;
 *     - whether resources are sufficient;
 *     - whether quantum restrictions are satisfied;
 *     - whether HDL/resource semantics are valid;
 *     - whether foreign interoperability is valid;
 *     - whether overload/call compatibility is valid.
 *
 * None of these semantic checks occur in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Function types do NOT create a separate IR.
 *
 * They remain part of the canonical semantic type representation until
 * downstream compiler stages lower the callable itself.
 *
 * For quantum-containing callable semantics:
 *
 *     TypeExpr::Function
 *          |
 *          v
 *     semantic type/function model
 *          |
 *          v
 *     quantum semantic operations
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum boundary.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Downstream compiler layers may decide:
 *
 *     calling convention
 *     ABI representation
 *     closure representation
 *     register allocation
 *     stack representation
 *     heap representation
 *     device placement
 *     parallelization
 *     distributed execution
 *     quantum lowering
 *     hardware realization
 *
 * None of these decisions belong to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is outside grammar ownership.
 *
 * This grammar does not specify:
 *
 *     function-pointer representation
 *     closure-object layout
 *     stack layout
 *     register layout
 *     calling sequence
 *     physical memory placement
 *     executable address
 *     device invocation mechanism
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical syntax is:
 *
 *     fn(parameter-types) -> return-type
 *
 * The following are intentionally NOT separate function-type syntaxes:
 *
 *     K_FN(...)
 *     function(...)
 *     callable(...)
 *     cpu_fn(...)
 *     gpu_fn(...)
 *     qpu_fn(...)
 *     fpga_fn(...)
 *
 * Historical parser variants must migrate to the canonical `FN` token and
 * `THIN_ARROW` token.
 *
 * Compatibility handling belongs to:
 *
 *     grammar/compatibility/
 *
 * and must not create duplicate canonical grammar rules.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     fn() -> Unit
 *     fn() -> int
 *     fn(int) -> int
 *     fn(int, float) -> bool
 *     fn(int,) -> int
 *     fn(int, float,) -> bool
 *     fn(T) -> T
 *     fn(A, B) -> C
 *     fn(fn(int) -> int) -> int
 *     fn(int) -> fn(int) -> int
 *     fn(fn(A) -> B, fn(B) -> C) -> fn(A) -> C
 *     fn(Qubit) -> Qubit
 *     fn(Qubit) -> Measurement
 *     fn(Tensor<T>) -> Tensor<T>
 *     fn(Signal<T>) -> Signal<T>
 *
 * Negative:
 *
 *     fn
 *     fn(
 *     fn(
 *     fn()
 *     fn(int)
 *     fn(,) -> int
 *     fn(int,, float) -> bool
 *     fn(,int) -> bool
 *     fn(int,) -> bool
 *         // only invalid if trailing-comma policy is removed; currently valid
 *     fn(int -> int
 *     fn(int) int
 *     fn(int, float -> bool
 *     fn(int, ) 
 *
 * NOTE:
 *
 * The final negative case is invalid because the mandatory return clause is
 * absent.
 *
 * Boundary:
 *
 *     zero parameters
 *     one parameter
 *     two parameters
 *     many source-defined parameters
 *     nested function parameters
 *     nested function returns
 *     generic function parameter types
 *     dependent function parameter types
 *     quantum function parameter types
 *     HDL function parameter types
 *     distributed function parameter types
 *
 * Scalability:
 *
 *     parameter repetition uses ANTLR repetition;
 *     no parameter-count constant exists;
 *     nested function types remain recursively representable;
 *     no machine capacity is encoded;
 *     no hardware identity is encoded;
 *     no quantum cardinality is encoded.
 *
 * Determinism:
 *
 *     source order is preserved;
 *     no target-dependent parse behavior exists;
 *     no semantic action exists.
 *
 * Compatibility:
 *
 *     canonical FN token;
 *     canonical THIN_ARROW token;
 *     canonical delimiters;
 *     no duplicate token vocabulary.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/types/types.g4
 *
 * `Types` must import:
 *
 *     Function
 *
 * and must delegate ownership of `functionType` to this file.
 *
 * The old inline `functionType`, `functionTypeParameters`, and
 * `functionTypeReturn` rules in `types.g4` must be removed so this file is the
 * sole owner.
 *
 * `Types.typeCore` must retain:
 *
 *     functionType
 *
 * as the dispatch point.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The root parser already imports:
 *
 *     Types
 *
 * Therefore the root parser obtains `functionType` transitively through
 * `Types`.
 *
 * `ZamaniParser.g4` must NOT independently define `functionType`.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/functions/functions.g4
 *
 * Named function declarations must continue to use the canonical type
 * expression for parameter and return types.
 *
 * Function declarations and function types must remain separate owners.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/functions/parameters.g4
 *
 * Named parameter declarations use their existing declaration syntax.
 *
 * This function-type grammar must not be reused as a substitute for named
 * parameters.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/functions/generics.g4
 *
 * Generic declaration syntax remains owned by that file.
 *
 * Function types merely consume generic type expressions.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/types/generic.g4
 *
 * Generic type applications inside function signatures continue to use the
 * canonical generic type grammar.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/effects/
 *
 * Effect syntax remains owned by the effect subsystem.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/resources/
 *
 * Resource requirements remain outside function-type syntax.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/quantum/
 *
 * Quantum types remain ordinary canonical type expressions.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/hdl/
 *
 * HDL types remain ordinary canonical type expressions.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/hardware/
 *
 * Hardware semantic types remain ordinary canonical type expressions.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     src/ast/mod.rs
 *
 * Function types must lower to the existing:
 *
 *     TypeExpr::Function(Vec<TypeExpr>, Box<TypeExpr>)
 *
 * No new AST node is required.
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     src/parser.rs
 *
 * The existing parser's function-type semantics already construct:
 *
 *     TypeExpr::Function
 *
 * The ANTLR grammar must describe the same source-language construct rather
 * than introduce incompatible syntax.
 *
 * ============================================================================
 * FILE DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     canonical Zamani parser token vocabulary
 *     Types.typeExpression
 *
 * EXPORTS:
 *
 *     functionType
 *     functionTypeParameterList
 *     functionTypeParameter
 *     functionTypeReturn
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     grammar/antlr/ZamaniParser.g4 transitively
 *
 * AST_OWNER:
 *
 *     src/ast/mod.rs
 *
 * SEMANTIC_OWNER:
 *
 *     type/semantic analysis subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     quantum::ir for quantum semantic lowering
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/spec/type-system.md where applicable
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no fixed parameter count
 *     no fixed return count
 *     no fixed nesting depth
 *     no CPU count
 *     no GPU count
 *     no FPGA count
 *     no QPU count
 *     no qubit count
 *     no node count
 *     no thread count
 *     no memory size
 *     no tensor rank
 *     no register width
 *     no network size
 *     no device count
 *     no vendor ABI
 *     no physical topology
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is a delegate grammar.
 *     [x] It owns function-type syntax only.
 *     [x] It does not duplicate typeExpression.
 *     [x] It uses canonical token names.
 *     [x] It has no lexer rules.
 *     [x] It has no semantic actions.
 *     [x] It has no unsafe Rust.
 *     [x] It has no hardware assumptions.
 *     [x] It has no resource limits.
 *     [x] It supports zero parameters.
 *     [x] It supports arbitrary source-defined parameter counts.
 *     [x] It supports trailing commas.
 *     [x] It requires the return arrow.
 *     [x] It supports higher-order functions.
 *     [x] It supports nested function return types.
 *     [x] It consumes canonical typeExpression.
 *     [x] It maps to TypeExpr::Function.
 *     [x] It preserves domain neutrality.
 *     [x] It preserves POCO-REAF.
 *     [x] It preserves quantum::ir as the quantum boundary.
 *
 * Repository integration remains complete only after:
 *
 *     [ ] Types imports Function.
 *     [ ] Types removes its duplicate inline functionType rules.
 *     [ ] Root parser continues importing Types.
 *     [ ] ANTLR generation succeeds.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Compatibility tests pass.
 *     [ ] Rust 1.97 / 1.97.1 build passes.
 *     [ ] safe-Rust audit passes.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A function type describes the semantic signature of callable computation.
 *
 * It does NOT describe the machine on which that computation will execute.
 *
 * Therefore:
 *
 *     FUNCTION TYPE
 *          !=
 *     TARGET IMPLEMENTATION
 *
 * The type:
 *
 *     fn(A) -> B
 *
 * retains the same source-level meaning whether its eventual realization is:
 *
 *     tiny embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future computational substrate
 *
 * subject to the semantic capabilities, resource requirements, contracts,
 * policies, and implementation feasibility of the selected target.
 *
 * ============================================================================
 */

parser grammar Function;


/*
 * ============================================================================
 * FUNCTION TYPE
 * ============================================================================
 *
 * Canonical:
 *
 *     fn(parameter-types) -> return-type
 *
 * The return clause is intentionally mandatory.
 *
 * `typeExpression` is supplied by the canonical `Types` delegating grammar.
 */
functionType
    : FN
      LPAREN
      functionTypeParameterList?
      RPAREN
      functionTypeReturn
    ;


/*
 * ============================================================================
 * FUNCTION TYPE PARAMETERS
 * ============================================================================
 *
 * Zero parameters are represented by omission of the parameter list.
 *
 * One or more parameters use ordered repetition.
 *
 * A trailing comma is accepted.
 *
 * There is no language-level maximum parameter count.
 */
functionTypeParameterList
    : functionTypeParameter
      (COMMA functionTypeParameter)*
      COMMA?
    ;


/*
 * ============================================================================
 * INDIVIDUAL FUNCTION TYPE PARAMETER
 * ============================================================================
 *
 * A parameter is a complete canonical type expression.
 *
 * This permits higher-order and domain-specific types without requiring this
 * grammar to enumerate them.
 */
functionTypeParameter
    : typeExpression
    ;


/*
 * ============================================================================
 * FUNCTION TYPE RETURN
 * ============================================================================
 *
 * Every complete function type has exactly one semantic return type.
 *
 * Tuple returns are represented by the canonical tuple type:
 *
 *     fn(A) -> (B, C)
 *
 * rather than by introducing a second "multiple return" function grammar.
 */
functionTypeReturn
    : THIN_ARROW
      typeExpression
    ;