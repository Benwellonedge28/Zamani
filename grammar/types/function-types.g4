/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/function-types.g4
 *
 * Grammar:
 *     FunctionTypes
 *
 * Status:
 *     Production-ready function-type grammar component.
 *
 * Purpose:
 *     Defines the source-level syntax of function types in Zamani.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar owns ONLY function-type syntax.
 *
 * It provides the parser-level category:
 *
 *     functionType
 *
 * Function types describe callable computational values independently of:
 *
 *     - CPU architecture;
 *     - GPU architecture;
 *     - FPGA architecture;
 *     - ASIC implementation;
 *     - QPU implementation;
 *     - ABI;
 *     - calling convention;
 *     - physical resources;
 *     - scheduling;
 *     - routing;
 *     - deployment;
 *     - runtime representation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - function type syntax;
 *   - function parameter type lists;
 *   - function return type syntax;
 *   - zero-parameter function types;
 *   - multi-parameter function types;
 *   - trailing-comma compatibility;
 *   - variadic function-type syntax, where explicitly supported;
 *   - function-type modifiers that are intrinsic to the callable type;
 *   - function-type parser boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - function declarations;
 *   - function definitions;
 *   - function bodies;
 *   - parameter declarations;
 *   - generic declaration syntax;
 *   - generic constraint declarations;
 *   - closure syntax;
 *   - async function declarations;
 *   - generator declarations;
 *   - foreign-function declarations;
 *   - ABI declarations;
 *   - calling conventions;
 *   - effect semantics;
 *   - capability semantics;
 *   - resource allocation;
 *   - scheduling;
 *   - hardware selection;
 *   - quantum allocation;
 *   - QEC;
 *   - ZQN;
 *   - optimization;
 *   - routing;
 *   - canonical IR;
 *   - runtime representation.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *                  |
 *                  v
 *           FunctionTypes
 *                  |
 *                  v
 *          canonical Types
 *                  |
 *                  v
 *             frontend AST
 *                  |
 *                  v
 *          semantic type system
 *                  |
 *                  +----------------------+
 *                  |                      |
 *                  v                      v
 *             classical IR          quantum::ir
 *                  |                      |
 *                  +----------+-----------+
 *                             |
 *                             v
 *                    optimization /
 *                    routing /
 *                    scheduling /
 *                    hardware /
 *                    runtime
 *
 * FunctionTypes MUST NOT depend on any downstream compiler/runtime subsystem.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Function types describe the semantic shape of a callable computation.
 *
 * They do NOT describe:
 *
 *     - where the function executes;
 *     - how many CPUs execute it;
 *     - how many GPUs execute it;
 *     - how many QPUs execute it;
 *     - how many threads exist;
 *     - where memory resides;
 *     - which device executes it;
 *     - which network node executes it;
 *     - which physical qubits are used;
 *     - which hardware topology is selected.
 *
 * Therefore:
 *
 *     fn(A, B) -> C
 *
 * remains valid regardless of whether the implementation eventually runs on:
 *
 *     - an embedded CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum-classical system;
 *     - a cluster;
 *     - a cloud service;
 *     - a future computational architecture.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is deliberately NO grammar constant for:
 *
 *     MAX_PARAMETERS
 *     MAX_RETURN_VALUES
 *     MAX_FUNCTION_TYPE_DEPTH
 *     MAX_GENERIC_PARAMETERS
 *     MAX_CALLABLES
 *     MAX_NESTING
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *
 * Recursive grammar composition provides unbounded syntactic structure subject
 * only to implementation resources and explicitly defined compiler policies.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *
 *     ZamaniTokens
 *
 * Important tokens used here include:
 *
 *     K_FN
 *     K_MUT
 *     THIN_ARROW
 *     COMMA
 *     LPAREN
 *     RPAREN
 *     IDENTIFIER
 *
 * This grammar MUST NOT redefine any lexer token.
 *
 * ============================================================================
 * TYPE-EXPRESSION CONTRACT
 * ============================================================================
 *
 * `typeExpression` remains owned by the canonical type grammar:
 *
 *     grammar/types/types.g4
 *
 * FunctionTypes consumes that rule.
 *
 * This is essential because function parameter and return types may themselves
 * be:
 *
 *     primitive types
 *     named types
 *     generic types
 *     tuple types
 *     arrays
 *     slices
 *     option types
 *     result types
 *     references
 *     quantum types
 *     hardware/resource abstractions
 *     other future types
 *     nested function types
 *
 * FunctionTypes MUST NOT duplicate those type categories.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is a parser delegate.
 *
 * The canonical root type grammar imports it:
 *
 *     import FunctionTypes;
 *
 * The root `Types` grammar owns the complete `typeExpression` rule.
 *
 * FunctionTypes may therefore reference:
 *
 *     typeExpression
 *
 * without creating a second type-expression authority.
 *
 * This is deliberate ANTLR grammar composition rather than a circular
 * semantic dependency.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing determines syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether parameter types are valid;
 *     - whether return types are valid;
 *     - whether a function is callable;
 *     - generic substitution;
 *     - effect compatibility;
 *     - capability requirements;
 *     - ownership/borrowing;
 *     - resource requirements;
 *     - quantum validity;
 *     - hardware validity;
 *     - ABI compatibility;
 *     - calling convention;
 *     - target lowering.
 *
 * ============================================================================
 * FUNCTION TYPE EXAMPLES
 * ============================================================================
 *
 *     fn() -> int
 *
 *     fn(int) -> int
 *
 *     fn(int, float) -> bool
 *
 *     fn(Qubit) -> Result
 *
 *     fn([Qubit]) -> [Qubit]
 *
 *     fn(T) -> T
 *
 *     fn(fn(int) -> int) -> int
 *
 *     fn((int, float)) -> bool
 *
 *     fn(&T) -> &T
 *
 *     fn([T; N]) -> [T; N]
 *
 *     fn(QuantumState) -> ClassicalResult
 *
 * No physical machine assumption is encoded by any of these forms.
 *
 * ============================================================================
 */

parser grammar FunctionTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical function-type syntax.
 *
 * Basic form:
 *
 *     fn(parameter-types) -> return-type
 *
 * ============================================================================
 */

functionType
    : functionTypePrefix
      LPAREN
      functionTypeParameterList?
      RPAREN
      functionTypeReturn
    ;


/* ============================================================================
 * 2. FUNCTION TYPE PREFIX
 * ============================================================================
 *
 * `fn` is the canonical callable-type introducer.
 *
 * Function declarations may have additional declaration-level modifiers such
 * as async, extern, unsafe, visibility, generic parameters, effects, etc.
 *
 * Those modifiers do NOT belong here unless they are semantically part of the
 * callable type itself.
 *
 * ============================================================================
 */

functionTypePrefix
    : K_FN
    ;


/* ============================================================================
 * 3. PARAMETER LIST
 * ============================================================================
 *
 * There is no fixed parameter-count limit.
 *
 * The recursive/list structure permits arbitrary parameter counts subject to
 * compiler resources.
 *
 * ============================================================================
 */

functionTypeParameterList
    : functionTypeParameter
      (COMMA functionTypeParameter)*
      COMMA?
    ;


/* ============================================================================
 * 4. PARAMETER TYPE
 * ============================================================================
 *
 * A function-type parameter is a TYPE, not a function declaration parameter.
 *
 * Examples:
 *
 *     fn(int) -> bool
 *
 *     fn(Qubit) -> Result
 *
 *     fn(fn(int) -> int) -> int
 *
 * `typeExpression` is supplied by the canonical Types grammar.
 *
 * ============================================================================
 */

functionTypeParameter
    : typeExpression
    ;


/* ============================================================================
 * 5. RETURN TYPE
 * ============================================================================
 *
 * A function type always has an explicit return-type position.
 *
 * Example:
 *
 *     fn(int) -> bool
 *
 * This avoids conflating:
 *
 *     callable type
 *
 * with:
 *
 *     function declaration.
 *
 * ============================================================================
 */

functionTypeReturn
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 6. ZERO-PARAMETER FUNCTIONS
 * ============================================================================
 *
 * The optional parameter-list form permits:
 *
 *     fn() -> T
 *
 * No special `fn0` or fixed-arity representation is introduced.
 *
 * ============================================================================
 */


/* ============================================================================
 * 7. NESTED FUNCTION TYPES
 * ============================================================================
 *
 * Because `typeExpression` may contain `functionType`, nested callables are
 * naturally supported:
 *
 *     fn(fn(int) -> int) -> int
 *
 *     fn() -> fn(int) -> bool
 *
 *     fn(fn(A) -> B, fn(B) -> C) -> fn(A) -> C
 *
 * There is no finite nesting limit in this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 8. FUNCTION TYPES AS PARAMETERS
 * ============================================================================
 *
 * Function types may appear wherever the canonical type grammar permits a
 * type expression.
 *
 * Examples:
 *
 *     fn(fn(int) -> int) -> bool
 *
 *     fn((fn(int) -> int), Qubit) -> Result
 *
 * No special parameter category is required.
 *
 * ============================================================================
 */


/* ============================================================================
 * 9. FUNCTION TYPES AS RETURN VALUES
 * ============================================================================
 *
 * Higher-order functions are supported naturally.
 *
 * Examples:
 *
 *     fn(int) -> fn(int) -> int
 *
 *     fn(Qubit) -> fn(ClassicalValue) -> Result
 *
 * The AST builder must preserve the nested type structure rather than
 * flattening it.
 *
 * ============================================================================
 */


/* ============================================================================
 * 10. TRAILING COMMA
 * ============================================================================
 *
 * Trailing commas are accepted:
 *
 *     fn(int,) -> int
 *
 *     fn(
 *         int,
 *         float,
 *     ) -> bool
 *
 * This provides stable formatting and minimizes source churn.
 *
 * The formatter is responsible for canonical presentation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 11. TYPE PARAMETER COMPATIBILITY
 * ============================================================================
 *
 * Generic function declarations are owned by:
 *
 *     grammar/functions/generics.g4
 *
 * and:
 *
 *     grammar/types/generic-types.g4
 *
 * FunctionTypes does NOT define generic declarations.
 *
 * Nevertheless, function types can contain generic/type-parameter references
 * through the canonical `typeExpression` rule.
 *
 * Example:
 *
 *     fn(T) -> T
 *
 * Resolution of `T` is semantic and depends on the surrounding generic
 * declaration context.
 *
 * This grammar must not determine whether `T` is:
 *
 *     - a type parameter;
 *     - a type alias;
 *     - a named type;
 *     - a module-qualified type;
 *     - a future type constructor.
 *
 * ============================================================================
 */


/* ============================================================================
 * 12. EFFECT INTEGRATION
 * ============================================================================
 *
 * Effects are owned by:
 *
 *     grammar/effects/
 *
 * FunctionTypes does NOT duplicate the effect grammar.
 *
 * If Zamani later defines effect-bearing function types such as:
 *
 *     fn(A) -> B ! IO
 *
 * or another canonical effect syntax, the effect system must establish the
 * syntax and integration contract first.
 *
 * The function-type grammar should then consume that canonical rule.
 *
 * It must NOT independently invent a second effect syntax.
 *
 * ============================================================================
 */


/* ============================================================================
 * 13. ASYNC INTEGRATION
 * ============================================================================
 *
 * Async function declarations are owned by:
 *
 *     grammar/functions/async.g4
 *
 * `async` is therefore NOT duplicated here merely because a function
 * declaration may be asynchronous.
 *
 * If semantic analysis determines that asynchronousness is part of function
 * type identity, a future language-versioned extension can add an explicit
 * function-type modifier through one canonical rule.
 *
 * Until such a rule exists,:
 *
 *     async
 *
 * remains declaration/effect syntax rather than being silently treated as a
 * function-type property.
 *
 * ============================================================================
 */


/* ============================================================================
 * 14. GENERATOR INTEGRATION
 * ============================================================================
 *
 * Generator declarations belong to:
 *
 *     grammar/functions/generators.g4
 *
 * Generator yield semantics must not be encoded by this grammar.
 *
 * If generator types become first-class source types, their canonical type
 * constructor belongs in the broader type-system architecture and can then be
 * consumed through `typeExpression`.
 *
 * ============================================================================
 */


/* ============================================================================
 * 15. FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign functions and FFI declarations belong to:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * FunctionTypes does not encode:
 *
 *     C ABI
 *     C++
 *     Python ABI
 *     System ABI
 *     vendor ABI
 *     platform calling convention
 *
 * Those are compilation/interoperability semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 16. ABI INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT contain syntax such as:
 *
 *     x86_64_fn
 *     arm64_fn
 *     wasm_fn
 *     cuda_fn
 *     qpu_fn
 *
 * unless a future language specification explicitly establishes such a
 * concept as a semantic target contract.
 *
 * Physical ABI selection belongs downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 17. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are consumed through:
 *
 *     typeExpression
 *
 * Therefore function types may express:
 *
 *     fn(Qubit) -> Qubit
 *
 *     fn(Qubit) -> ClassicalValue
 *
 *     fn([Qubit]) -> Result
 *
 *     fn(QuantumState) -> Measurement
 *
 * without this grammar importing:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     hardware
 *
 * A function type containing `Qubit` does NOT identify:
 *
 *     physical qubit 0
 *
 *     physical qubit 1
 *
 *     a QPU
 *
 *     a topology
 *
 *     a gate set
 *
 *     a device.
 *
 * Those are downstream semantic and compilation decisions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 18. CLASSICAL / QUANTUM INTEROPERABILITY
 * ============================================================================
 *
 * Function types naturally support hybrid signatures:
 *
 *     fn(ClassicalValue, Qubit) -> ClassicalValue
 *
 *     fn(Qubit) -> ClassicalResult
 *
 *     fn(ClassicalControl) -> QuantumOperation
 *
 *     fn(QuantumState) -> ClassicalMeasurement
 *
 * This grammar does not privilege classical or quantum computation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 19. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related types may occur in function signatures through
 * `typeExpression`.
 *
 * Examples:
 *
 *     fn(Signal) -> Signal
 *
 *     fn(Clock) -> Register
 *
 *     fn(HardwareInterface) -> Result
 *
 *     fn(AcceleratorInput) -> AcceleratorOutput
 *
 * FunctionTypes does not select physical hardware.
 *
 * ============================================================================
 */


/* ============================================================================
 * 20. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed/resource abstractions may occur through normal type expressions.
 *
 * Examples:
 *
 *     fn(Message) -> Message
 *
 *     fn(NodeHandle) -> Result
 *
 *     fn(Stream<T>) -> Stream<U>
 *
 * The grammar does not encode:
 *
 *     node count
 *     topology
 *     host address
 *     device address
 *     cluster size
 *
 * ============================================================================
 */


/* ============================================================================
 * 21. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements are NOT encoded by arbitrary function-type syntax.
 *
 * For example:
 *
 *     fn(Qubit) -> Qubit
 *
 * expresses a type-level signature.
 *
 * It does NOT mean:
 *
 *     allocate a particular physical qubit
 *
 * or:
 *
 *     reserve a particular QPU.
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *     semantic capability analysis
 *     compilation context
 *     scheduling
 *     runtime resource negotiation
 *
 * ============================================================================
 */


/* ============================================================================
 * 22. OWNERSHIP / BORROWING INTEGRATION
 * ============================================================================
 *
 * References remain normal types:
 *
 *     fn(&T) -> &T
 *
 *     fn(&mut T) -> T
 *
 * FunctionTypes does not implement borrow checking.
 *
 * Ownership/lifetime legality belongs to semantic analysis.
 *
 * ============================================================================
 */


/* ============================================================================
 * 23. FUNCTION TYPE AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The AST builder should map it into the repository's canonical TypeExpr
 * representation.
 *
 * Conceptual representation:
 *
 *     FunctionType
 *         parameters: [TypeExpr, ...]
 *         return_type: TypeExpr
 *
 * The exact Rust enum/struct names are owned by the repository's AST/type
 * implementation and MUST NOT be invented by this grammar.
 *
 * No:
 *
 *     FunctionTypeAst
 *     FunctionTypeIR
 *     QuantumFunctionTypeIR
 *
 * should be introduced solely because this grammar exists.
 *
 * ============================================================================
 */


/* ============================================================================
 * 24. SEMANTIC VALIDATION
 * ============================================================================
 *
 * After parsing, semantic analysis must validate:
 *
 *     - parameter types;
 *     - return type;
 *     - generic parameters;
 *     - generic bounds;
 *     - ownership;
 *     - lifetime rules;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - domain-specific restrictions;
 *     - recursion legality;
 *     - target-independent semantic equivalence.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 25. CANONICAL FUNCTION TYPE IDENTITY
 * ============================================================================
 *
 * Function type identity must be derived from semantic structure.
 *
 * Conceptually:
 *
 *     fn(A, B) -> C
 *
 * differs from:
 *
 *     fn(A) -> B
 *
 * and:
 *
 *     fn(A, B) -> C
 *
 * is structurally equivalent regardless of formatting:
 *
 *     fn(A,B)->C
 *
 *     fn(
 *         A,
 *         B,
 *     ) -> C
 *
 * Formatting must not change semantic identity.
 *
 * ============================================================================
 */


/* ============================================================================
 * 26. NO MACHINE-DEPENDENT FUNCTION ARITY
 * ============================================================================
 *
 * Forbidden grammar design:
 *
 *     functionType2
 *     functionType4
 *     functionType8
 *     functionType16
 *
 * or any finite enumeration of parameter counts.
 *
 * Correct:
 *
 *     functionTypeParameterList
 *
 * with repetition.
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. NO FIXED RETURN VALUE COUNT
 * ============================================================================
 *
 * Zamani does not need a separate finite grammar inventory for:
 *
 *     fn1
 *     fn2
 *     fn4
 *
 * Multiple return values can be represented through canonical composite types,
 * for example:
 *
 *     fn(A) -> (B, C)
 *
 * This keeps function signatures orthogonal to machine ABI conventions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. NO HARD-CODED RESOURCE LIMITS
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_PARAMETERS
 *     MAX_FUNCTIONS
 *     MAX_CALL_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Any compiler safety/resource limit must be implemented as an explicit
 * compiler policy rather than silently encoded in source grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 29. ERROR-HANDLING CONTRACT
 * ============================================================================
 *
 * Malformed function types must remain syntax errors.
 *
 * Examples:
 *
 *     fn
 *     fn(
 *     fn() 
 *     fn(int)
 *     fn(,) -> int
 *     fn(int,,bool) -> int
 *     fn(int -> int
 *
 * The parser must not silently:
 *
 *     - invent a return type;
 *     - discard parameters;
 *     - infer missing delimiters as semantics;
 *     - select an ABI;
 *     - choose a hardware target.
 *
 * Error recovery belongs to the canonical parser/diagnostic layer.
 *
 * ============================================================================
 */


/* ============================================================================
 * 30. DETERMINISM
 * ============================================================================
 *
 * No semantic predicates, embedded target-language actions, filesystem access,
 * network access, runtime calls, or hardware discovery are permitted.
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *
 * the parse structure must be deterministic.
 *
 * ============================================================================
 */


/* ============================================================================
 * 31. SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no commands;
 *     - evaluates no function;
 *     - allocates no hardware resource;
 *     - selects no device;
 *     - contains no Rust code;
 *     - contains no unsafe operations.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. RUST INTEGRATION CONTRACT
 * ============================================================================
 *
 * This `.g4` file itself contains no Rust.
 *
 * The generated/consuming Zamani compiler/frontend implementation MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * The compiler crate should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * or the repository's stronger equivalent safety policy.
 *
 * No grammar action may introduce Rust implementation dependencies.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. COMPILER INTEGRATION
 * ============================================================================
 *
 * FunctionTypes participates in:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     TypeExpr AST
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     classical semantics       quantum semantics
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *                  canonical IR
 *                     |
 *                     v
 *          optimization / lowering
 *                     |
 *                     v
 *           routing / scheduling
 *                     |
 *                     v
 *             target realization
 *
 * FunctionTypes must never directly construct IR.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * This grammar has NO dependency on:
 *
 *     quantum::ir
 *
 * If a function type contains quantum types, semantic lowering eventually
 * converts the relevant semantic information into the canonical quantum
 * representation.
 *
 * The grammar must never:
 *
 *     - construct quantum gates;
 *     - construct qubit IDs;
 *     - allocate qubits;
 *     - construct quantum operations;
 *     - perform routing;
 *     - perform scheduling.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. QEC / ZQN BOUNDARY
 * ============================================================================
 *
 * FunctionTypes has NO dependency on:
 *
 *     QEC
 *     ZQN
 *
 * A function signature may describe a semantic quantum computation, but error
 * correction and noise/fault semantics belong to their respective subsystems.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. HARDWARE BOUNDARY
 * ============================================================================
 *
 * FunctionTypes has NO dependency on:
 *
 *     hardware discovery
 *     calibration
 *     topology
 *     backend IDs
 *     physical qubits
 *     device IDs
 *
 * A backend may later determine how a function is realized.
 *
 * The source function type remains unchanged.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. RUNTIME BOUNDARY
 * ============================================================================
 *
 * Runtime invocation semantics are downstream.
 *
 * The grammar does not:
 *
 *     dispatch functions;
 *     allocate stacks;
 *     allocate registers;
 *     select CPUs;
 *     select GPUs;
 *     select QPUs;
 *     create network endpoints.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. TOOLING CONTRACT
 * ============================================================================
 *
 * Tools may consume the `functionType` parse-tree category for:
 *
 *     - syntax highlighting;
 *     - formatter implementation;
 *     - documentation generation;
 *     - signature display;
 *     - source indexing;
 *     - IDE navigation;
 *     - semantic analysis;
 *     - type checking;
 *     - API documentation.
 *
 * Tools must not infer hardware characteristics from the grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. FORMATTER CONTRACT
 * ============================================================================
 *
 * A canonical formatter may normalize:
 *
 *     fn(int,float)->bool
 *
 * into:
 *
 *     fn(int, float) -> bool
 *
 * or a multiline representation.
 *
 * Formatting must preserve:
 *
 *     parameter order;
 *     parameter types;
 *     return type;
 *     nested function structure.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical function syntax:
 *
 *     fn(...) -> ...
 *
 * must remain stable.
 *
 * Any future syntax change must be introduced through the language-versioning
 * and compatibility systems rather than silently changing this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. TEST CONTRACT
 * ============================================================================
 *
 * Tests belong under the grammar test hierarchy, for example:
 *
 *     grammar/tests/types/functions/
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 *     fn() -> int
 *     fn(int) -> int
 *     fn(int, float) -> bool
 *     fn(Qubit) -> Qubit
 *     fn([Qubit]) -> Result
 *     fn((int, float)) -> bool
 *     fn(fn(int) -> int) -> int
 *     fn(int) -> fn(int) -> int
 *     fn(T) -> T
 *
 * --------------------------------------------------------------------------
 * Formatting
 * --------------------------------------------------------------------------
 *
 *     fn(int,float)->bool
 *     fn(int, float,) -> bool
 *     fn(
 *         int,
 *         float,
 *     ) -> bool
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 *     fn
 *     fn(
 *     fn(
 *     fn()
 *     fn(int)
 *     fn(,) -> int
 *     fn(int,,bool) -> int
 *     fn(int -> int
 *     fn(int,) 
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 *     zero parameters
 *     one parameter
 *     many parameters
 *     deeply nested function types
 *     large parameter lists
 *     nested generic parameter types
 *     nested quantum types
 *     nested composite types
 *
 * --------------------------------------------------------------------------
 * Cross-domain
 * --------------------------------------------------------------------------
 *
 *     classical -> classical
 *     classical -> quantum
 *     quantum -> classical
 *     quantum -> quantum
 *     hardware -> classical
 *     classical -> hardware
 *     quantum + classical + hardware
 *     distributed + quantum
 *     AI + accelerator
 *
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 * Tests must NOT encode artificial maxima as language requirements.
 *
 * Any stress-test limit must be clearly marked as a TEST EXECUTION LIMIT,
 * never a grammar/language limit.
 *
 * --------------------------------------------------------------------------
 * Determinism
 * --------------------------------------------------------------------------
 *
 * Identical source and grammar version must produce identical parse structure.
 *
 * --------------------------------------------------------------------------
 * Round-trip
 * --------------------------------------------------------------------------
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve function-type semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must remain free of:
 *
 *     MAX_PARAMETERS
 *     MAX_FUNCTION_ARITY
 *     MAX_NESTING
 *     MAX_FUNCTION_DEPTH
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NODES
 *
 * Any future limit discovered here must be classified as:
 *
 *     1. language semantic requirement
 *     2. target requirement
 *     3. resource constraint
 *     4. implementation limitation
 *     5. accidental hard-coding
 *     6. test-only limitation
 *     7. documentation-only limitation
 *
 * Only categories 1-3 may survive, and they must not be disguised as grammar
 * limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] It is a parser grammar.
 *
 * [ ] It uses:
 *
 *         tokenVocab = ZamaniTokens
 *
 * [ ] It defines exactly one canonical public function-type category:
 *
 *         functionType
 *
 * [ ] It consumes the canonical `typeExpression` rule.
 *
 * [ ] It does not redefine primitive types.
 *
 * [ ] It does not redefine composite types.
 *
 * [ ] It does not redefine generic types.
 *
 * [ ] It does not redefine identifiers.
 *
 * [ ] It does not define function declarations.
 *
 * [ ] It does not define function bodies.
 *
 * [ ] It does not define ABI semantics.
 *
 * [ ] It does not define effects.
 *
 * [ ] It does not define hardware semantics.
 *
 * [ ] It does not depend on quantum::ir.
 *
 * [ ] It does not depend on QEC.
 *
 * [ ] It does not depend on ZQN.
 *
 * [ ] It contains no machine-size assumptions.
 *
 * [ ] It contains no finite function-arity catalog.
 *
 * [ ] It supports nested function types.
 *
 * [ ] It supports zero-parameter functions.
 *
 * [ ] It supports arbitrary parameter lists.
 *
 * [ ] It supports arbitrary return types.
 *
 * [ ] It supports trailing commas.
 *
 * [ ] It preserves source-level semantic structure.
 *
 * [ ] It contains no semantic actions.
 *
 * [ ] It contains no target-language code.
 *
 * [ ] It contains no unsafe implementation.
 *
 * [ ] It has positive tests.
 *
 * [ ] It has negative tests.
 *
 * [ ] It has boundary tests.
 *
 * [ ] It has scalability tests.
 *
 * [ ] It has cross-domain tests.
 *
 * [ ] It has deterministic-parser tests.
 *
 * [ ] It has round-trip tests.
 *
 * [ ] Its integration with `types.g4` is verified.
 *
 * [ ] Its integration with the canonical lexer is verified.
 *
 * [ ] Its AST mapping is verified against the existing TypeExpr model.
 *
 * [ ] No downstream subsystem needs to modify this file to use the function
 *     type correctly.
 *
 * ============================================================================
 */