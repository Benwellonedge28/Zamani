/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/compile-time-functions.g4
 *
 * Grammar:
 *     CompileTimeFunctions
 *
 * Status:
 *     Production parser-grammar integration contract
 *
 * Purpose:
 *     Define the syntactic boundary for functions whose evaluation or
 *     specialization is permitted/required to participate in Zamani's
 *     compile-time evaluation model.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Compile-time functions are still Zamani functions.
 *
 * This file MUST NOT create a second function language.
 *
 * Ordinary function syntax remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * Types remain owned by:
 *
 *     grammar/types/
 *
 * Expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * Compile-time expression forms remain owned by:
 *
 *     grammar/expressions/compile-time.g4
 *
 * Function effects remain owned by:
 *
 *     grammar/effects/
 *
 * Function contracts remain owned by the canonical contract grammar.
 *
 * Foreign-function syntax remains owned by:
 *
 *     grammar/functions/foreign-functions.g4
 *
 * The purpose of this file is therefore to establish the compile-time
 * function integration boundary without duplicating those grammars.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compile-time execution MUST NOT make source semantics dependent upon:
 *
 *     - the compiler host CPU;
 *     - the compiler host operating system;
 *     - compiler host memory size;
 *     - compiler host thread count;
 *     - a particular accelerator;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular QPU;
 *     - a particular device;
 *     - a particular topology;
 *     - a particular deployment.
 *
 * A compile-time function describes computation that may be evaluated during
 * compilation.
 *
 * It does NOT mean that the function is permitted to inspect arbitrary
 * properties of the machine performing compilation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the compile-time-function integration boundary;
 *     - the syntactic classification of compile-time function declarations;
 *     - compile-time function declaration attachment;
 *     - compile-time evaluation/specialization attachment boundaries;
 *     - compile-time-function-specific syntactic restrictions that cannot
 *       be represented by ordinary function syntax;
 *     - integration points between ordinary functions and compile-time
 *       evaluation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary function declarations;
 *     - function names;
 *     - parameter syntax;
 *     - generic parameter syntax;
 *     - type syntax;
 *     - expression precedence;
 *     - statement syntax;
 *     - blocks;
 *     - effects;
 *     - contracts;
 *     - macros;
 *     - macro expansion;
 *     - reflection semantics;
 *     - constant evaluation;
 *     - partial evaluation;
 *     - generic specialization;
 *     - optimization;
 *     - target selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - quantum IR;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - ABI selection;
 *     - foreign-function implementation.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer/tokens.g4
 *            |
 *            v
 *     functions/functions.g4
 *            |
 *            +-----------------------+
 *            |                       |
 *            v                       v
 *     compile-time-functions   expressions/compile-time.g4
 *            |                       |
 *            +-----------+-----------+
 *                        |
 *                        v
 *                      AST
 *                        |
 *                        v
 *                 semantic analysis
 *                        |
 *             +----------+----------+
 *             |                     |
 *             v                     v
 *       classical IR          quantum::ir
 *             |                     |
 *             +----------+----------+
 *                        |
 *                        v
 *        optimization / lowering /
 *        routing / scheduling / HAL
 *                        |
 *                        v
 *                     runtime
 *
 * This grammar MUST NOT create reverse dependencies.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani token vocabulary.
 *
 * It MUST NOT define lexer tokens.
 *
 * In particular, this file MUST NOT invent separate tokens such as:
 *
 *     COMPTIME
 *     EVAL
 *     SPECIALIZE
 *     TYPEOF
 *     HAS
 *
 * Compile-time expression vocabulary is already integrated through the
 * canonical compile-time-expression grammar and semantic system.
 *
 * Existing language vocabulary such as:
 *
 *     const
 *     fn
 *     let
 *     assert
 *     sizeof
 *
 * remains owned by the canonical lexer.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Rust integration targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Generated parser/frontend integration MUST use safe Rust only.
 *
 * This grammar itself contains no Rust code and no unsafe operations.
 *
 * The generated parser MUST NOT become:
 *
 *     - the canonical AST;
 *     - the semantic model;
 *     - classical IR;
 *     - quantum::ir;
 *     - a hardware model.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a compile-time function declaration MUST NOT:
 *
 *     - execute the function;
 *     - access the filesystem;
 *     - access a network;
 *     - inspect hardware;
 *     - access credentials;
 *     - select a provider;
 *     - allocate target resources;
 *     - invoke a QPU;
 *     - invoke a GPU;
 *     - invoke an FPGA;
 *     - invoke an ASIC;
 *     - perform linking;
 *     - perform deployment.
 *
 * Compile-time execution is a later semantic/compiler operation.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     "This declaration is syntactically eligible to participate in the
 *      compile-time function model."
 *
 * Semantic analysis establishes:
 *
 *     - whether the function is actually compile-time evaluable;
 *     - whether its dependencies are compile-time evaluable;
 *     - whether its effects are permitted;
 *     - whether its arguments are compile-time available;
 *     - whether evaluation is deterministic;
 *     - whether evaluation is reproducible;
 *     - whether evaluation is permitted by the compilation context;
 *     - whether specialization is legal;
 *     - whether recursion is legal;
 *     - whether termination can be established where required;
 *     - whether resource requirements are acceptable;
 *     - whether the resulting value is representable;
 *     - whether generated semantics preserve program meaning.
 *
 * These are NOT grammar-level machine constraints.
 *
 * ============================================================================
 */

parser grammar CompileTimeFunctions;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. CANONICAL COMPILE-TIME FUNCTION ENTRY POINT
 * ============================================================================
 *
 * A compile-time function is an ordinary Zamani function participating in
 * compile-time evaluation.
 *
 * The canonical source-level marker is the existing `const` function
 * modifier.
 *
 * Therefore:
 *
 *     const fn factorial(n: int) -> int {
 *         ...
 *     }
 *
 * is classified as a compile-time-capable function declaration.
 *
 * This rule deliberately does NOT reproduce:
 *
 *     function name
 *     generic parameters
 *     parameters
 *     return types
 *     effects
 *     contracts
 *     body
 *
 * Those remain owned by Functions.
 *
 * ============================================================================
 */

compileTimeFunctionDeclaration
    : K_CONST compileTimeFunctionCore
    ;


/* ============================================================================
 * 2. FUNCTION CORE INTEGRATION
 * ============================================================================
 *
 * `compileTimeFunctionCore` is an integration boundary.
 *
 * The composed parser MUST bind this boundary to the canonical function
 * declaration machinery.
 *
 * The canonical function grammar owns:
 *
 *     functionDeclaration
 *
 * and the semantic layer determines whether the declaration's complete
 * modifier set is compatible with compile-time execution.
 *
 * The composition layer SHOULD expose a dedicated compile-time declaration
 * entry point rather than copying the ordinary function grammar.
 *
 * ============================================================================
 */

compileTimeFunctionCore
    : K_FN
      functionName
      functionGenericParameters?
      LPAREN
      functionParameterListOrVariadic?
      RPAREN
      functionReturnClause?
      functionEffectAttachment?
      functionContractAttachment?
      compileTimeFunctionBody
    ;


/* ============================================================================
 * 3. FUNCTION NAME
 * ============================================================================
 *
 * The function name remains a canonical lexical identifier.
 *
 * It MUST NOT encode:
 *
 *     compiler;
 *     host;
 *     device;
 *     CPU;
 *     GPU;
 *     QPU;
 *     FPGA;
 *     ASIC;
 *     topology;
 *     address;
 *     resource count.
 *
 * Name resolution remains downstream.
 *
 * ============================================================================
 */

functionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * Compile-time functions may be generic.
 *
 * Generic parameters are symbolic language parameters.
 *
 * They do NOT represent:
 *
 *     hardware resources;
 *     physical memory;
 *     qubit counts;
 *     processor counts;
 *     device counts.
 *
 * ============================================================================
 */

functionGenericParameters
    : LESS_THAN
      functionGenericParameter
      (COMMA functionGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


functionGenericParameter
    : IDENTIFIER
      functionGenericParameterBound?
    ;


functionGenericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 5. PARAMETERS
 * ============================================================================
 *
 * No finite parameter-count limit is encoded.
 *
 * Resource limitations belong to parser/compiler resource policy.
 *
 * ============================================================================
 */

functionParameterListOrVariadic
    : functionParameterList
    | functionParameterListWithVariadic
    ;


functionParameterList
    : functionParameter
      (COMMA functionParameter)*
      COMMA?
    ;


functionParameter
    : functionParameterModifier*
      functionParameterName
      (COLON typeExpression)?
      functionParameterDefault?
    ;


functionParameterModifier
    : K_MUT
    ;


functionParameterName
    : IDENTIFIER
    ;


functionParameterDefault
    : EQUALS
      expression
    ;


functionParameterListWithVariadic
    : functionParameter
      (COMMA functionParameter)*
      COMMA
      variadicFunctionParameter
      COMMA?
    ;


variadicFunctionParameter
    : functionParameterModifier*
      ELLIPSIS
      functionParameterName
      (COLON typeExpression)?
    ;


/* ============================================================================
 * 6. RETURN TYPE
 * ============================================================================
 *
 * The return type is canonical Zamani type syntax.
 *
 * Compile-time evaluation does not introduce a second type system.
 *
 * ============================================================================
 */

functionReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 7. EFFECT ATTACHMENT
 * ============================================================================
 *
 * Effects remain owned by the canonical effects grammar.
 *
 * Compile-time semantic analysis determines which effects are legal.
 *
 * Examples of questions that belong downstream:
 *
 *     Can this function perform I/O?
 *     Can it access external state?
 *     Can it inspect resources?
 *     Can it access network state?
 *     Can it invoke target-specific functionality?
 *
 * ============================================================================
 */

functionEffectAttachment
    : effectClause
    ;


/* ============================================================================
 * 8. FUNCTION CONTRACT ATTACHMENT
 * ============================================================================
 *
 * Contracts remain owned by the canonical contract grammar.
 *
 * Compile-time functions receive the same contract machinery as ordinary
 * functions.
 *
 * Semantic analysis determines whether a contract itself is compile-time
 * evaluable.
 *
 * ============================================================================
 */

functionContractAttachment
    : contractClause
    ;


/* ============================================================================
 * 9. COMPILE-TIME FUNCTION BODY
 * ============================================================================
 *
 * A compile-time function uses the canonical Zamani block syntax.
 *
 * This grammar does NOT introduce a second compile-time statement language.
 *
 * The semantic layer determines which operations inside the body are legal
 * during compile-time evaluation.
 *
 * ============================================================================
 */

compileTimeFunctionBody
    : block
    | SEMICOLON
    ;


/* ============================================================================
 * 10. COMPILE-TIME FUNCTION INVOCATION BOUNDARY
 * ============================================================================
 *
 * Invocation remains ordinary expression syntax.
 *
 * This grammar therefore does not define a second call operator.
 *
 * Example:
 *
 *     const fn square(x: int) -> int {
 *         return x * x;
 *     }
 *
 *     const value = square(4);
 *
 * The expression grammar owns:
 *
 *     square(4)
 *
 * The compile-time semantic evaluator determines whether the call is
 * evaluated during compilation.
 *
 * ============================================================================
 */

compileTimeFunctionInvocation
    : expression
    ;


/* ============================================================================
 * 11. COMPILE-TIME VALUE BOUNDARY
 * ============================================================================
 *
 * A compile-time function may produce:
 *
 *     scalar values;
 *     structured values;
 *     type-level information;
 *     compile-time metadata;
 *     specialization information;
 *     generated semantic information.
 *
 * The grammar deliberately does not prescribe a finite value universe.
 *
 * Type validity remains owned by the type system.
 *
 * ============================================================================
 */

compileTimeFunctionResult
    : expression
    ;


/* ============================================================================
 * 12. COMPILE-TIME EVALUATION INTEGRATION
 * ============================================================================
 *
 * Explicit compile-time evaluation syntax is owned by:
 *
 *     grammar/expressions/compile-time.g4
 *
 * This file therefore exposes only the function-side integration point.
 *
 * A compile-time function may be referenced by a compile-time expression
 * wherever the expression grammar permits it.
 *
 * This prevents compile-time functions from inventing a parallel evaluation
 * syntax.
 *
 * ============================================================================
 */

compileTimeEvaluationTarget
    : compileTimeFunctionReference
    ;


compileTimeFunctionReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 13. SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Generic specialization belongs to the semantic/compiler layers.
 *
 * The grammar does not introduce:
 *
 *     specialize<T>
 *
 * or equivalent target-specific syntax.
 *
 * Generic function syntax remains canonical.
 *
 * Specialization may occur because the compiler determines that a generic
 * compile-time function can be evaluated or specialized.
 *
 * ============================================================================
 */

compileTimeSpecializationTarget
    : compileTimeFunctionReference
    ;


/* ============================================================================
 * 14. RECURSION
 * ============================================================================
 *
 * Recursive compile-time functions are syntactically permitted.
 *
 * This is intentional.
 *
 * The grammar must not impose arbitrary recursion-depth limits such as:
 *
 *     MAX_RECURSION = 64
 *     MAX_RECURSION = 1024
 *
 * Termination, evaluation budgets, recursion safety, and compiler resource
 * limits belong to semantic/compiler policy.
 *
 * ============================================================================
 */

compileTimeRecursiveFunction
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 15. DETERMINISM BOUNDARY
 * ============================================================================
 *
 * Determinism is NOT established by grammar.
 *
 * The semantic/compiler layer must determine whether a compile-time function
 * depends upon observable external state.
 *
 * Compile-time evaluation SHOULD be deterministic unless the language
 * specification explicitly permits a different evaluation class.
 *
 * Examples requiring semantic analysis:
 *
 *     clock;
 *     random source;
 *     environment variables;
 *     filesystem;
 *     network;
 *     hardware discovery;
 *     provider state;
 *     external service state.
 *
 * None of these become grammar-level restrictions here.
 *
 * ============================================================================
 */

compileTimeDeterminismBoundary
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 16. RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * Compile-time functions may perform computations whose size is determined
 * by program semantics.
 *
 * The grammar imposes no fixed limits on:
 *
 *     number of compile-time functions;
 *     number of parameters;
 *     number of generic parameters;
 *     recursion depth;
 *     expression size;
 *     generated semantic objects;
 *     type complexity;
 *     data dimensions;
 *     collection cardinality.
 *
 * Compiler implementation limits may exist, but they MUST remain compiler
 * resource policy rather than language grammar constants.
 *
 * ============================================================================
 */

compileTimeResourceIndependentFunction
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 17. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Compile-time functions may manipulate quantum-domain values only where the
 * semantic/type/effect system permits it.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_QUBITS;
 *     MAX_QUBIT_PARAMETERS;
 *     fixed QPU identifiers;
 *     fixed gate sets;
 *     physical topology;
 *     physical addresses;
 *     fixed quantum-register sizes.
 *
 * Quantum syntax belongs to:
 *
 *     grammar/quantum/
 *
 * Canonical quantum meaning belongs to:
 *
 *     quantum::ir
 *
 * A compile-time function therefore cannot become an alternate quantum IR.
 *
 * ============================================================================
 */

compileTimeQuantumIntegration
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 18. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Compile-time functions may operate on classical values and types.
 *
 * Numerical, symbolic, vector, matrix, tensor, and accelerator semantics
 * remain owned by their corresponding domain/type/semantic layers.
 *
 * This grammar does not impose numerical representation limits.
 *
 * ============================================================================
 */

compileTimeClassicalIntegration
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 19. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Compile-time functions may participate in hardware/software co-design,
 * including parameter generation and structural specialization.
 *
 * They MUST NOT:
 *
 *     - select a physical FPGA;
 *     - select an ASIC;
 *     - select a CPU;
 *     - select a GPU;
 *     - select a QPU;
 *     - encode fixed clock availability;
 *     - encode fixed hardware resource counts;
 *     - encode physical addresses.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 */

compileTimeHardwareIntegration
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 20. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Compile-time functions may compute:
 *
 *     configuration;
 *     topology-independent plans;
 *     schemas;
 *     generated declarations;
 *     static routing metadata;
 *     protocol metadata.
 *
 * They must not turn compile-time evaluation into implicit network execution.
 *
 * ============================================================================
 */

compileTimeDistributedIntegration
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 21. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Compile-time functions may participate in compile-time generation or
 * specialization involving AI/data abstractions where permitted by the
 * semantic layer.
 *
 * The grammar does not define:
 *
 *     model size limits;
 *     tensor dimension limits;
 *     dataset limits;
 *     accelerator counts;
 *     memory limits.
 *
 * ============================================================================
 */

compileTimeAIIntegration
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 22. FOREIGN FUNCTION RESTRICTION
 * ============================================================================
 *
 * A foreign function is not automatically a compile-time function.
 *
 * Foreign-function syntax belongs to:
 *
 *     grammar/functions/foreign-functions.g4
 *
 * Whether a foreign function is safe and deterministic for compile-time
 * evaluation is a semantic/compiler decision.
 *
 * A compile-time function MUST NOT silently turn arbitrary foreign calls
 * into compile-time execution.
 *
 * ============================================================================
 */

compileTimeForeignBoundary
    : compileTimeFunctionReference
    ;


/* ============================================================================
 * 23. MACRO BOUNDARY
 * ============================================================================
 *
 * Compile-time functions and macros are different language mechanisms.
 *
 * This file MUST NOT define macro declarations or expansion.
 *
 * Macros belong to:
 *
 *     grammar/macros/
 *
 * A compile-time function computes a semantic value or compile-time result.
 *
 * Macro expansion transforms source structure.
 *
 * The compiler may integrate both mechanisms downstream without collapsing
 * their ownership boundaries.
 *
 * ============================================================================
 */

compileTimeMacroBoundary
    : compileTimeFunctionReference
    ;


/* ============================================================================
 * 24. METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Metaprogramming owns reflection, generation, quotation, and specialization
 * mechanisms.
 *
 * Compile-time functions may serve as one execution mechanism for those
 * facilities, but this grammar does not define their semantic implementation.
 *
 * ============================================================================
 */

compileTimeMetaprogrammingBoundary
    : compileTimeFunctionReference
    ;


/* ============================================================================
 * 25. CANONICAL INTEGRATION RULE
 * ============================================================================
 *
 * The composition layer should expose compile-time function declarations
 * through this rule:
 *
 *     compileTimeFunctionDeclaration
 *
 * and ordinary functions through:
 *
 *     functionDeclaration
 *
 * The canonical AST MUST normalize both into the same function declaration
 * representation with compile-time classification represented as semantic
 * metadata rather than a second function AST hierarchy.
 *
 * ============================================================================
 */

compileTimeFunction
    : compileTimeFunctionDeclaration
    ;


/* ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - source span;
 *     - function name;
 *     - modifiers;
 *     - generic parameters;
 *     - parameters;
 *     - parameter defaults;
 *     - return type;
 *     - effects;
 *     - contracts;
 *     - body;
 *     - compile-time classification.
 *
 * It MUST NOT embed:
 *
 *     - hardware topology;
 *     - physical device IDs;
 *     - runtime scheduling;
 *     - routing;
 *     - QEC state;
 *     - ZQN state;
 *     - backend implementation details.
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST validate at least:
 *
 *     - compile-time modifier validity;
 *     - duplicate/conflicting modifiers;
 *     - parameter validity;
 *     - generic validity;
 *     - return type validity;
 *     - effect restrictions;
 *     - contract validity;
 *     - recursive evaluation policy;
 *     - deterministic evaluation policy;
 *     - dependency availability;
 *     - compile-time value validity;
 *     - resource/evaluation policy;
 *     - specialization legality;
 *     - generated-result validity.
 *
 * Semantic analysis MUST NOT interpret parser acceptance as proof that a
 * function is safely executable at compile time.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. EFFECT CONTRACT
 * ============================================================================
 *
 * Compile-time evaluation must be effect-aware.
 *
 * The following are examples of effects that may require semantic rejection
 * or explicit permission:
 *
 *     filesystem access;
 *     network access;
 *     nondeterministic external state;
 *     hardware discovery;
 *     device access;
 *     runtime-only resources;
 *     provider services;
 *     uncontrolled foreign calls.
 *
 * The grammar does not enumerate a permanent list of forbidden effects.
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 * This permits future effect systems without repeatedly rewriting this file.
 *
 * ============================================================================
 */


/* ============================================================================
 * 29. IR CONTRACT
 * ============================================================================
 *
 * Compile-time functions do not create a separate IR.
 *
 * Their results are lowered through the ordinary semantic pipeline.
 *
 * Depending on the result, downstream lowering may produce:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     metadata;
 *     compile-time constants;
 *     specialized AST/IR;
 *     other canonical representations.
 *
 * The compile-time grammar MUST NOT define any of those representations.
 *
 * ============================================================================
 */


/* ============================================================================
 * 30. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a compile-time function produces or transforms quantum semantics,
 * semantic lowering MUST ultimately use:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT introduce a compile-time quantum IR.
 *
 * Downstream systems remain responsible for:
 *
 *     optimization;
 *     QEC;
 *     ZQN;
 *     routing;
 *     scheduling;
 *     hardware realization;
 *     runtime execution.
 *
 * ============================================================================
 */


/* ============================================================================
 * 31. SECURITY CONTRACT
 * ============================================================================
 *
 * Compile-time evaluation must be sandboxable and policy-controlled.
 *
 * The grammar itself is incapable of granting:
 *
 *     filesystem permission;
 *     network permission;
 *     hardware permission;
 *     credential access;
 *     provider access.
 *
 * Such permissions must come from explicit compiler/security policy.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parser output MUST be deterministic for identical source and identical
 * grammar/token vocabulary.
 *
 * Compile-time evaluation determinism is a semantic/compiler property.
 *
 * Reproducible builds SHOULD be achievable by ensuring that compile-time
 * evaluation receives an explicit compilation context rather than implicitly
 * observing the host machine.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Compile-time functions MUST preserve the distinction between:
 *
 *     source semantics
 *
 * and:
 *
 *     compiler implementation.
 *
 * A compile-time function MUST NOT cause the same source program to acquire
 * different language meaning merely because compilation occurs on:
 *
 *     CPU A;
 *     CPU B;
 *     GPU-enabled host;
 *     quantum-enabled host;
 *     FPGA development system;
 *     cloud worker;
 *     embedded development environment.
 *
 * Target-specific information may influence compilation only through explicit
 * compilation-context semantics and target/resource contracts.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level fixed limits on:
 *
 *     compile-time function count;
 *     parameter count;
 *     generic parameter count;
 *     expression size;
 *     source size;
 *     recursion depth;
 *     specialization count;
 *     generated declarations;
 *     generated values;
 *     quantum resources;
 *     classical resources;
 *     hardware resources;
 *     distributed resources.
 *
 * Any actual limit must be:
 *
 *     explicit;
 *     implementation-owned;
 *     diagnosable;
 *     configurable where appropriate;
 *     independent of source-language semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_COMPTIME_FUNCTIONS
 *     MAX_COMPTIME_DEPTH
 *     MAX_SPECIALIZATIONS
 *     MAX_GENERATED_TYPES
 *     MAX_GENERATED_VALUES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or equivalent hidden restrictions.
 *
 * A numeric literal is permitted when it is genuinely part of source
 * semantics.
 *
 * A numeric literal MUST NOT be used as an implementation ceiling merely
 * because the current compiler prefers that value.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. COMPATIBILITY
 * ============================================================================
 *
 * Existing:
 *
 *     const fn ...
 *
 * declarations remain compatible with the canonical function grammar.
 *
 * The repository MUST NOT maintain two independent definitions of function
 * syntax.
 *
 * The root/composed grammar should normalize compile-time functions and
 * ordinary functions through the same function AST representation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The authoritative composed parser should integrate this grammar alongside:
 *
 *     functions/functions.g4
 *     expressions/expressions.g4
 *     expressions/compile-time.g4
 *     types/*
 *     statements/*
 *     effects/*
 *     core/*
 *
 * The root grammar MUST NOT duplicate:
 *
 *     compile-time function syntax;
 *     ordinary function syntax;
 *     parameter syntax;
 *     type syntax;
 *     expression syntax.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST be covered by grammar tests.
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 *     const fn identity(x: int) -> int {
 *         return x;
 *     }
 *
 *     const fn square<T>(x: T) -> T {
 *         ...
 *     }
 *
 *     const fn make_value(x: int = 1) -> int {
 *         ...
 *     }
 *
 *     const fn fold(...values: int) -> int {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 * Generic
 * --------------------------------------------------------------------------
 *
 *     const fn identity<T>(value: T) -> T {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 * Quantum
 * --------------------------------------------------------------------------
 *
 *     const fn build_circuit<T>(value: T) -> Circuit<T> {
 *         ...
 *     }
 *
 * No fixed qubit count may be required by the grammar.
 *
 * --------------------------------------------------------------------------
 * Hardware
 * --------------------------------------------------------------------------
 *
 *     const fn configure<T>(value: T) -> T {
 *         ...
 *     }
 *
 * Hardware-specific realization remains downstream.
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 *     const const fn invalid(...) ...
 *
 * must be rejected by the composed grammar/semantic validation.
 *
 * A compile-time function with incompatible effects must be rejected by
 * semantic analysis rather than silently accepted as executable compile-time
 * computation.
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 * Test:
 *
 *     zero-parameter functions;
 *     one-parameter functions;
 *     many parameters;
 *     deeply nested types;
 *     deeply nested expressions;
 *     generic functions;
 *     recursive functions;
 *     large compile-time computations.
 *
 * Tests MUST NOT encode artificial finite maxima as language requirements.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The compile-time function grammar must be tested in combination with:
 *
 *     classical + compile-time
 *     quantum + compile-time
 *     hybrid + compile-time
 *     HDL + compile-time
 *     hardware + compile-time
 *     distributed + compile-time
 *     AI + compile-time
 *     data + compile-time
 *     interoperability + compile-time
 *
 * The grammar must remain domain-neutral.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the canonical AST printer/serializer supports these declarations:
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
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve compile-time function classification and semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] Compile-time functions use the canonical function model.
 *
 * [ ] No duplicate type system exists here.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] No duplicate statement grammar exists here.
 *
 * [ ] No duplicate effect grammar exists here.
 *
 * [ ] No duplicate contract grammar exists here.
 *
 * [ ] No duplicate foreign-function grammar exists here.
 *
 * [ ] No new lexer keywords are invented here.
 *
 * [ ] `const fn` integrates with the canonical function vocabulary.
 *
 * [ ] Compile-time expression syntax remains owned by expressions/compile-time.g4.
 *
 * [ ] Compile-time semantics remain downstream.
 *
 * [ ] Compile-time functions cannot implicitly select hardware.
 *
 * [ ] Compile-time functions cannot implicitly access devices.
 *
 * [ ] Compile-time functions cannot implicitly access networks/filesystems.
 *
 * [ ] No machine-size limits are encoded.
 *
 * [ ] No qubit limits are encoded.
 *
 * [ ] No CPU/GPU/FPGA/ASIC/QPU limits are encoded.
 *
 * [ ] No recursion limit is encoded.
 *
 * [ ] No specialization limit is encoded.
 *
 * [ ] AST integration is defined.
 *
 * [ ] Semantic integration is defined.
 *
 * [ ] Classical IR integration is defined.
 *
 * [ ] quantum::ir integration is defined.
 *
 * [ ] Optimization integration is defined.
 *
 * [ ] Scheduling integration is defined.
 *
 * [ ] Hardware integration is defined.
 *
 * [ ] Runtime integration is defined.
 *
 * [ ] Security boundaries are defined.
 *
 * [ ] Determinism requirements are defined.
 *
 * [ ] POCO-REAF requirements are defined.
 *
 * [ ] Rust 1.97 / 1.97.1 integration remains safe-Rust-only.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * ============================================================================
 */