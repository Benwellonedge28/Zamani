/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/functions.g4
 *
 * Grammar:
 *     Functions
 *
 * Purpose:
 *     Canonical source-level grammar for Zamani function declarations and
 *     definitions.
 *
 * This file owns FUNCTION DECLARATIONS.
 *
 * It does NOT own:
 *
 *     - function types;
 *     - expressions;
 *     - statements;
 *     - blocks;
 *     - general types;
 *     - generic type semantics;
 *     - effect semantics;
 *     - capability semantics;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - hardware selection;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - ABI selection;
 *     - calling conventions.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes computation rather than a particular machine.
 *
 * Therefore a function declaration MUST remain independent of:
 *
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU count;
 *     - qubit count;
 *     - thread count;
 *     - memory capacity;
 *     - device identifiers;
 *     - topology;
 *     - physical addresses;
 *     - deployment size;
 *     - hardware vendor;
 *     - runtime implementation.
 *
 * There are no grammar-level limits such as:
 *
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_FUNCTION_DEPTH
 *     MAX_FUNCTIONS
 *     MAX_CALLABLES
 *     MAX_THREADS
 *     MAX_QUBITS
 *
 * Repetition and recursion are bounded only by the parser/compiler/runtime
 * resource policies of the implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Function declarations express reusable computational semantics.
 *
 * The same function source can therefore participate in:
 *
 *     classical execution
 *     quantum-classical execution
 *     hardware/software co-design
 *     distributed execution
 *     accelerator execution
 *     embedded execution
 *     cloud execution
 *     future execution models
 *
 * A function declaration does NOT select:
 *
 *     - a processor;
 *     - a QPU;
 *     - a physical qubit;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a network node;
 *     - a scheduling policy.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - function declaration syntax;
 *     - function definition syntax;
 *     - function names;
 *     - function declaration modifiers;
 *     - function generic parameter syntax;
 *     - function parameter declaration syntax;
 *     - optional parameter initializers;
 *     - variadic parameter syntax;
 *     - function return-type attachment;
 *     - function effect attachment boundary;
 *     - function contract attachment boundary;
 *     - function body boundary;
 *     - declaration-only/prototype boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - function type syntax;
 *       -> grammar/types/function-types.g4
 *
 *     - complete type-expression syntax;
 *       -> grammar/types/
 *
 *     - expression precedence;
 *       -> grammar/expressions/
 *
 *     - statement syntax;
 *       -> grammar/statements/
 *
 *     - block syntax;
 *       -> grammar/statements/
 *
 *     - generic type semantics;
 *       -> grammar/types/generic-types.g4
 *
 *     - generic semantic constraints;
 *       -> semantic/type-system layers
 *
 *     - effect semantics;
 *       -> grammar/effects/
 *
 *     - foreign-function/ABI semantics;
 *       -> grammar/functions/foreign-functions.g4
 *          grammar/interoperability/
 *
 *     - async execution;
 *       -> grammar/functions/async.g4
 *          runtime/concurrency layers
 *
 *     - generator semantics;
 *       -> grammar/functions/generators.g4
 *
 *     - closure/lambda expression syntax;
 *       -> grammar/expressions/
 *
 *     - quantum semantics;
 *       -> quantum grammar + semantic analysis
 *
 *     - canonical quantum representation;
 *       -> quantum::ir
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer/tokens.g4
 *            |
 *            v
 *        Functions
 *            |
 *            +--------------------+
 *            |                    |
 *            v                    v
 *       Types grammar       Expressions grammar
 *            |                    |
 *            +---------+----------+
 *                      |
 *                      v
 *                 Statements
 *                      |
 *                      v
 *                     AST
 *                      |
 *                      v
 *               Semantic analysis
 *                      |
 *          +-----------+------------+
 *          |                        |
 *          v                        v
 *     classical IR              quantum::ir
 *          |                        |
 *          +------------+-----------+
 *                       |
 *                       v
 *              optimization / QEC /
 *              ZQN / routing /
 *              scheduling / HAL
 *                       |
 *                       v
 *                    runtime
 *
 * This grammar MUST NOT introduce a reverse dependency into any of the
 * downstream compiler/runtime layers.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This file is a parser-grammar delegate.
 *
 * The authoritative composed parser is responsible for supplying:
 *
 *     typeExpression
 *     expression
 *     block
 *     effectClause
 *     contractClause
 *     qualifiedName
 *
 * where those rules are owned by their canonical grammar modules.
 *
 * This file intentionally does not duplicate them.
 *
 * The composition layer should therefore include this grammar alongside the
 * canonical Types, Expressions, Statements, Effects, and core grammars.
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
 * This file MUST NOT redefine lexer tokens.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Parser/frontend code integrating this grammar:
 *
 *     - targets Rust 1.97 or Rust 1.97.1;
 *     - uses safe Rust only;
 *     - contains no unsafe blocks;
 *     - contains no unsafe functions;
 *     - contains no target-specific parser behavior;
 *     - preserves source spans;
 *     - reports syntax errors separately from semantic errors;
 *     - does not use machine-specific constants as parser limits.
 *
 * The ANTLR-generated parser is an implementation artifact.
 *
 * It MUST NOT become the canonical AST, semantic model, or IR.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this a syntactically valid function declaration?"
 *
 * Semantic analysis answers:
 *
 *     - Does the function name conflict?
 *     - Are parameter names unique?
 *     - Are parameter types valid?
 *     - Are generic parameters valid?
 *     - Are generic constraints satisfiable?
 *     - Is a default value type-compatible?
 *     - Is the return type valid?
 *     - Are effects permitted?
 *     - Is the function body type-correct?
 *     - Is the function recursively valid?
 *     - Is an extern declaration ABI-compatible?
 *     - Are capability requirements satisfied?
 *     - Are resource requirements satisfiable?
 *     - Is a quantum operation semantically valid?
 *
 * Those questions MUST NOT be encoded as hardware-specific grammar rules.
 *
 * ============================================================================
 */

parser grammar Functions;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. CANONICAL FUNCTION DECLARATION
 * ============================================================================
 *
 * General form:
 *
 *     [modifiers] fn name [generics] (parameters) [-> return-type]
 *         [effects] [contract] body
 *
 * Declaration/prototype form:
 *
 *     [modifiers] fn name [generics] (parameters) [-> return-type]
 *         [effects] [contract] ;
 *
 * The distinction between a definition and declaration is syntactic.
 *
 * Semantic validation determines whether a body-less function is legal in
 * the surrounding context.
 *
 * This permits:
 *
 *     - extern declarations;
 *     - interface declarations;
 *     - abstract declarations;
 *     - forward declarations;
 *     - separately implemented functions;
 *     - language/runtime supplied declarations.
 *
 * ============================================================================
 */

functionDeclaration
    : functionModifier*
      K_FN
      functionName
      functionGenericParameters?
      LPAREN
      functionParameterList?
      RPAREN
      functionReturnClause?
      functionEffectAttachment?
      functionContractAttachment?
      functionBody
    ;


/* ============================================================================
 * 2. FUNCTION NAME
 * ============================================================================
 *
 * Names are lexical identifiers.
 *
 * Name resolution is owned by the semantic/name-resolution layer.
 *
 * A function name does not encode:
 *
 *     - a device;
 *     - an address;
 *     - a processor;
 *     - a QPU;
 *     - a hardware topology.
 *
 * ============================================================================
 */

functionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 3. FUNCTION MODIFIERS
 * ============================================================================
 *
 * Only modifiers already established by the canonical Zamani token vocabulary
 * are consumed here.
 *
 * This file does not invent new lexer keywords.
 *
 * Modifiers are syntactic attributes. Their compatibility and meaning are
 * semantic concerns.
 *
 * ============================================================================
 */

functionModifier
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_ASYNC
    | K_EXTERN
    | K_VOLATILE
    | K_INLINE
    | K_FINAL
    | K_SEALED
    | K_PARTIAL
    | K_VIRTUAL
    | K_OVERRIDE
    | K_ABSTRACT
    | K_MUT
    ;


/* ============================================================================
 * 4. GENERIC FUNCTION PARAMETERS
 * ============================================================================
 *
 * Generic parameters are source-level symbolic parameters.
 *
 * They do not represent:
 *
 *     - physical memory;
 *     - hardware resources;
 *     - qubit counts;
 *     - processor counts;
 *     - runtime device IDs.
 *
 * There is no fixed number of generic parameters.
 *
 * Example:
 *
 *     fn identity<T>(value: T) -> T { ... }
 *
 *     fn map<T, U>(value: T, transform: fn(T) -> U) -> U { ... }
 *
 * The type system determines whether a generic parameter is used correctly.
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


/*
 * A bound is intentionally expressed through the canonical type-expression
 * boundary rather than by introducing a second type system here.
 *
 * Example:
 *
 *     fn<T: Numeric>(value: T) -> T
 *
 * The meaning of Numeric is semantic.
 */

functionGenericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 5. FUNCTION PARAMETERS
 * ============================================================================
 *
 * There is no fixed parameter count.
 *
 * The list may contain any number of parameters supported by the compiler's
 * available resources.
 *
 * The grammar deliberately avoids machine-dependent limits.
 *
 * ============================================================================
 */

functionParameterList
    : functionParameter
      (COMMA functionParameter)*
      COMMA?
    ;


/* ============================================================================
 * 6. INDIVIDUAL PARAMETER
 * ============================================================================
 *
 * Supported forms include:
 *
 *     x
 *     x: int
 *     x: int = 42
 *     mut x: Buffer
 *
 * A parameter without an explicit type is syntactically permitted so that
 * contextual/type-inference systems can decide whether it is valid.
 *
 * The semantic layer MUST reject an omitted type where the selected language
 * mode requires an explicit type.
 *
 * ============================================================================
 */

functionParameter
    : functionParameterModifier*
      functionParameterName
      (COLON typeExpression)?
      functionParameterDefault?
    ;


/* ============================================================================
 * 7. PARAMETER MODIFIERS
 * ============================================================================
 *
 * Parameter modifiers are deliberately restricted to modifiers whose lexical
 * vocabulary already exists.
 *
 * `mut` is source-level mutability information.
 *
 * It does NOT prescribe a physical storage mechanism.
 *
 * ============================================================================
 */

functionParameterModifier
    : K_MUT
    ;


/* ============================================================================
 * 8. PARAMETER NAME
 * ============================================================================
 */

functionParameterName
    : IDENTIFIER
    ;


/* ============================================================================
 * 9. DEFAULT PARAMETER VALUES
 * ============================================================================
 *
 * Default values consume the canonical expression grammar.
 *
 * This grammar does not attempt to duplicate expression precedence.
 *
 * Examples:
 *
 *     x: int = 0
 *     threshold: float = 0.5
 *     state: T = defaultValue
 *
 * Semantic analysis determines whether the initializer is legal.
 *
 * ============================================================================
 */

functionParameterDefault
    : EQUALS
      expression
    ;


/* ============================================================================
 * 10. VARIADIC PARAMETERS
 * ============================================================================
 *
 * Variadic syntax is represented separately from ordinary parameters so that
 * the semantic layer can enforce:
 *
 *     - at most one variadic parameter where the language requires it;
 *     - variadic parameter placement;
 *     - valid element type;
 *     - compatibility with calling conventions.
 *
 * No finite argument-count limit is encoded.
 *
 * The source-level marker is `...`, represented by ELLIPSIS.
 *
 * Canonical form:
 *
 *     fn collect(...values: T) -> Result { ... }
 *
 * ============================================================================
 */

variadicFunctionParameter
    : functionParameterModifier*
      ELLIPSIS
      functionParameterName
      (COLON typeExpression)?
    ;


/*
 * Extended parameter-list form.
 *
 * Keeping variadic parameters in the same list gives the semantic layer the
 * complete source order and prevents the grammar from silently moving a
 * variadic argument to a different position.
 */

functionParameterListWithVariadic
    : functionParameter
      (COMMA functionParameter)*
      COMMA?
      COMMA?
      variadicFunctionParameter
      COMMA?
    ;


/*
 * NOTE:
 *
 * The canonical functionParameterList above intentionally remains the
 * ordinary-parameter rule.
 *
 * The composed function grammar should select one of the following list forms
 * according to the language-version policy:
 *
 *     functionParameterList
 *
 *     functionParameterListWithVariadic
 *
 * This file exposes both rules without forcing an implicit variadic calling
 * convention on every function.
 *
 * The aggregate function declaration therefore uses the explicit dispatcher
 * below.
 */

functionParameterListOrVariadic
    : functionParameterList
    | functionParameterListWithVariadic
    ;


/* ============================================================================
 * 11. RETURN TYPE
 * ============================================================================
 *
 * The return type is optional at the source level.
 *
 * If omitted, semantic/type inference determines whether omission is valid.
 *
 * Examples:
 *
 *     fn work() { ... }
 *
 *     fn work() -> int { ... }
 *
 *     fn measure(q: Qubit) -> Measurement { ... }
 *
 *     fn transform<T>(x: T) -> T { ... }
 *
 * The return type is always a canonical type expression.
 *
 * Function *types* remain owned by:
 *
 *     grammar/types/function-types.g4
 *
 * ============================================================================
 */

functionReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 12. EFFECT ATTACHMENT
 * ============================================================================
 *
 * Effects are not redefined here.
 *
 * The canonical effect grammar owns effect syntax.
 *
 * This rule is intentionally a narrow integration boundary.
 *
 * The composed grammar must provide:
 *
 *     effectClause
 *
 * from the effects subsystem.
 *
 * ============================================================================
 */

functionEffectAttachment
    : effectClause
    ;


/* ============================================================================
 * 13. FUNCTION CONTRACTS
 * ============================================================================
 *
 * Contracts describe source-level behavioral conditions.
 *
 * The contract grammar owns the internal contract syntax.
 *
 * This function grammar merely attaches the canonical contract construct to
 * the function declaration.
 *
 * Semantic validation determines whether:
 *
 *     requires
 *     ensures
 *     invariant
 *
 * are valid in the particular function context.
 *
 * ============================================================================
 */

functionContractAttachment
    : contractClause
    ;


/* ============================================================================
 * 14. FUNCTION BODY
 * ============================================================================
 *
 * Function bodies are owned by the canonical statement/block grammar.
 *
 * This file does not redefine:
 *
 *     block
 *     statement
 *     expression
 *     return
 *     loop
 *     match
 *     exception handling
 *     effect handling
 *
 * A function may syntactically have:
 *
 *     a body
 *
 * or:
 *
 *     a declaration terminator.
 *
 * Semantic analysis decides whether a declaration-only function is permitted
 * in its containing context.
 *
 * ============================================================================
 */

functionBody
    : block
    | SEMICOLON
    ;


/* ============================================================================
 * 15. FUNCTION BODY INTEGRATION CONTRACT
 * ============================================================================
 *
 * `block` MUST be supplied by the canonical statement grammar.
 *
 * Conceptually:
 *
 *     Functions
 *         |
 *         +--> Types.typeExpression
 *         |
 *         +--> Expressions.expression
 *         |
 *         +--> Statements.block
 *         |
 *         +--> Effects.effectClause
 *         |
 *         +--> Contracts.contractClause
 *
 * This avoids the historical problem where the function grammar created
 * duplicate versions of those constructs.
 *
 * ============================================================================
 */


/* ============================================================================
 * 16. METHOD COMPATIBILITY
 * ============================================================================
 *
 * Methods may use the same function declaration grammar when they occur inside
 * a class/trait/interface/implementation body.
 *
 * This grammar therefore does not hard-code:
 *
 *     self
 *     this
 *     object
 *     receiver
 *
 * as mandatory parameters.
 *
 * Receiver semantics belong to the surrounding declaration/type system.
 *
 * Examples that may be represented by ordinary parameters:
 *
 *     fn process(self, value: T) -> R
 *
 *     fn process(this, value: T) -> R
 *
 *     fn process(receiver: Receiver, value: T) -> R
 *
 * Whether a parameter is a receiver is semantic/contextual information.
 *
 * ============================================================================
 */


/* ============================================================================
 * 17. ASYNC FUNCTIONS
 * ============================================================================
 *
 * `async` is accepted as a declaration modifier because it is already part of
 * the canonical Zamani lexical vocabulary.
 *
 * This grammar does NOT define:
 *
 *     Future
 *     Promise
 *     executor
 *     scheduler
 *     task runtime
 *     polling
 *     wakeups
 *
 * Those belong to the concurrency/runtime architecture.
 *
 * Example:
 *
 *     async fn load() -> Data { ... }
 *
 * The semantic/runtime layers determine how that function executes.
 *
 * ============================================================================
 */


/* ============================================================================
 * 18. EXTERN FUNCTIONS
 * ============================================================================
 *
 * `extern` is accepted syntactically.
 *
 * This grammar does NOT define an ABI.
 *
 * It does not encode:
 *
 *     C
 *     C++
 *     Rust ABI
 *     System V
 *     Windows ABI
 *     GPU ABI
 *     FPGA interface
 *     QPU interface
 *     vendor-specific calling conventions.
 *
 * Those belong to interoperability/ABI compilation layers.
 *
 * Example:
 *
 *     extern fn external_compute(x: int) -> int;
 *
 * Semantic validation determines whether the declaration contains sufficient
 * interoperability information for compilation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 19. INLINE / OPTIMIZATION INDEPENDENCE
 * ============================================================================
 *
 * `inline` is a source-level request/hint.
 *
 * It does not require inlining.
 *
 * Optimization owns the actual transformation.
 *
 * Therefore:
 *
 *     inline fn f(...) -> ... { ... }
 *
 * must not force a particular compiler backend to inline the function.
 *
 * This preserves POCO-REAF.
 *
 * ============================================================================
 */


/* ============================================================================
 * 20. CONST FUNCTION INTEGRATION
 * ============================================================================
 *
 * `const` is accepted as a declaration modifier because it exists in the
 * lexical vocabulary.
 *
 * The semantic/compile-time system determines:
 *
 *     - whether the body is compile-time evaluable;
 *     - which effects are forbidden;
 *     - whether resource-dependent operations are allowed;
 *     - whether quantum/hardware operations are permitted.
 *
 * The grammar does not attempt to execute or validate constant evaluation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 21. QUANTUM FUNCTION INTEGRATION
 * ============================================================================
 *
 * Quantum types can occur through `typeExpression`.
 *
 * Therefore the function grammar naturally supports signatures such as:
 *
 *     fn prepare(q: Qubit) -> Qubit { ... }
 *
 *     fn measure(q: Qubit) -> Measurement { ... }
 *
 *     fn execute(state: QuantumState) -> ClassicalResult { ... }
 *
 *     fn transform<T>(value: T) -> T { ... }
 *
 * This file MUST NOT introduce:
 *
 *     q[0]
 *     q[1]
 *     MAX_QUBITS
 *     MAX_QUBIT_PARAMETERS
 *     fixed QPU identifiers
 *     topology requirements
 *     gate sets
 *     physical qubit IDs.
 *
 * Such information belongs to the quantum semantic/resource/hardware layers.
 *
 * Function signatures therefore remain portable from tiny systems to systems
 * whose available computational resources are arbitrarily larger.
 *
 * ============================================================================
 */


/* ============================================================================
 * 22. HYBRID COMPUTING
 * ============================================================================
 *
 * No special "hybrid function" grammar is required.
 *
 * Hybrid functions arise naturally from combinations of canonical types and
 * effects.
 *
 * Examples:
 *
 *     fn classical_to_quantum(value: ClassicalValue) -> Qubit
 *
 *     fn quantum_to_classical(q: Qubit) -> Measurement
 *
 *     fn hybrid(q: Qubit, x: Vector) -> Result
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 */


/* ============================================================================
 * 23. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Function syntax does not encode a hardware implementation.
 *
 * A hardware-related function can consume canonical hardware/resource types,
 * while placement, synthesis, scheduling and physical realization occur
 * downstream.
 *
 * No syntax here may impose:
 *
 *     fixed clock rate
 *     fixed register width
 *     fixed number of execution units
 *     fixed FPGA size
 *     fixed ASIC resource count
 *     fixed accelerator count.
 *
 * ============================================================================
 */


/* ============================================================================
 * 24. DISTRIBUTED FUNCTION INTEGRATION
 * ============================================================================
 *
 * Distribution is represented through semantic effects, capabilities,
 * resources, or domain declarations rather than a second function grammar.
 *
 * This prevents function declarations from becoming coupled to:
 *
 *     - node count;
 *     - network topology;
 *     - cluster size;
 *     - cloud provider;
 *     - deployment location.
 *
 * ============================================================================
 */


/* ============================================================================
 * 25. RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * A function declaration does not allocate resources.
 *
 * For example:
 *
 *     fn compute<T>(value: T) -> T { ... }
 *
 * does not imply:
 *
 *     one CPU
 *     one GPU
 *     one QPU
 *     one thread
 *     one node
 *
 * Resource requirements are resolved from the semantic program and execution
 * context by the resource/compiler/runtime layers.
 *
 * ============================================================================
 */


/* ============================================================================
 * 26. RECURSION
 * ============================================================================
 *
 * The grammar permits recursive source structure naturally.
 *
 * There is no grammar-level recursion-depth constant.
 *
 * Actual compiler recursion/stack limits are implementation policies and must
 * not be confused with language-level semantic limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * Given the same token stream and language-version context, this grammar must
 * produce the same parse structure.
 *
 * Semantic registries, hardware discovery, runtime state, and resource
 * availability MUST NOT influence parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. SOURCE-SPAN PRESERVATION
 * ============================================================================
 *
 * Frontend/AST integration must preserve source spans for:
 *
 *     function declaration
 *     modifiers
 *     function name
 *     generic parameters
 *     each parameter
 *     parameter type
 *     parameter default
 *     return type
 *     effects
 *     contracts
 *     body
 *
 * This enables:
 *
 *     diagnostics
 *     formatting
 *     IDE tooling
 *     refactoring
 *     provenance
 *     deterministic compilation diagnostics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 29. SEMANTIC VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * The parser MUST NOT silently accept semantically contradictory declarations
 * as valid program semantics.
 *
 * The semantic layer must validate at least:
 *
 *     - duplicate modifiers;
 *     - incompatible modifiers;
 *     - duplicate parameter names;
 *     - invalid generic parameter names;
 *     - duplicate generic parameters;
 *     - invalid generic bounds;
 *     - variadic placement;
 *     - default-parameter ordering;
 *     - return-type validity;
 *     - body/prototype compatibility;
 *     - extern/body compatibility;
 *     - abstract/body compatibility;
 *     - override validity;
 *     - visibility validity;
 *     - effect compatibility;
 *     - contract compatibility;
 *     - type inference requirements;
 *     - recursive declarations;
 *     - capability requirements.
 *
 * These are intentionally NOT grammar-level machine constraints.
 *
 * ============================================================================
 */


/* ============================================================================
 * 30. ABI / INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * Function declaration syntax must remain ABI-neutral.
 *
 * Interoperability metadata may be attached by the dedicated FFI/interoperability
 * grammar.
 *
 * This file MUST NOT encode platform-specific calling conventions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 31. IR BOUNDARY
 * ============================================================================
 *
 * This grammar produces syntax.
 *
 * It MUST NOT construct:
 *
 *     quantum::ir
 *     classical IR
 *     scheduling DAGs
 *     routing graphs
 *     hardware topology
 *     resource allocations.
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     Functions parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical IR         quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *          optimization / QEC /
 *          ZQN / routing /
 *          scheduling / HAL
 *                    |
 *                    v
 *                 runtime
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. SECURITY BOUNDARY
 * ============================================================================
 *
 * Parsing a function declaration must not:
 *
 *     - access the filesystem;
 *     - access a network;
 *     - inspect hardware;
 *     - execute user code;
 *     - allocate hardware resources;
 *     - invoke a compiler backend;
 *     - invoke a QPU;
 *     - query credentials;
 *     - select a provider.
 *
 * The grammar is a pure source-language description.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. SCALABILITY AUDIT
 * ============================================================================
 *
 * This grammar contains no fixed limits on:
 *
 *     function count
 *     parameter count
 *     generic parameter count
 *     type nesting
 *     expression nesting
 *     function nesting
 *     source size
 *     resource count
 *     device count
 *     node count
 *     thread count
 *     qubit count.
 *
 * Any implementation limit must be represented as a compiler/parser resource
 * policy rather than embedded into this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. COMPATIBILITY
 * ============================================================================
 *
 * Existing Zamani source using:
 *
 *     fn name(...) ...
 *
 * remains representable.
 *
 * Legacy root-grammar function syntax must be migrated into this canonical
 * rule rather than maintaining a second competing function grammar.
 *
 * The root grammar must eventually delegate function declarations to:
 *
 *     Functions.functionDeclaration
 *
 * rather than maintaining its own duplicate `functionDecl`.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. INTEGRATION REQUIREMENTS FOR THE ROOT GRAMMAR
 * ============================================================================
 *
 * The composed Zamani parser must:
 *
 *     1. import/use this Functions grammar;
 *     2. expose `functionDeclaration` as the canonical function-declaration
 *        rule;
 *     3. provide canonical `typeExpression`;
 *     4. provide canonical `expression`;
 *     5. provide canonical `block`;
 *     6. provide canonical `effectClause`;
 *     7. provide canonical `contractClause`;
 *     8. avoid redefining these rules in another function grammar;
 *     9. lower the resulting AST through semantic analysis;
 *    10. never lower directly from grammar into hardware/runtime code.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one canonical function declaration rule.
 *     [x] It uses ZamaniTokens rather than redefining lexer tokens.
 *     [x] It does not redefine function types.
 *     [x] It does not redefine type expressions.
 *     [x] It does not redefine expression precedence.
 *     [x] It does not redefine statement/block syntax.
 *     [x] It has no machine-size constants.
 *     [x] It has no qubit-count limits.
 *     [x] It has no CPU/GPU/FPGA/ASIC limits.
 *     [x] It has no hardware identifiers.
 *     [x] It has no topology assumptions.
 *     [x] It supports generic functions.
 *     [x] It supports arbitrary parameter-list cardinality.
 *     [x] It supports return types.
 *     [x] It supports declaration/prototype form.
 *     [x] It supports default parameters syntactically.
 *     [x] It exposes variadic syntax without imposing a finite limit.
 *     [x] It provides effect integration.
 *     [x] It provides contract integration.
 *     [x] It supports async declaration syntax.
 *     [x] It remains ABI-neutral.
 *     [x] It remains quantum-neutral at the hardware level.
 *     [x] It remains compatible with classical/quantum/hybrid types.
 *     [x] It remains suitable for HDL/hardware-related semantic types.
 *     [x] It preserves the grammar -> AST -> semantic analysis boundary.
 *     [x] It preserves the AST -> IR boundary.
 *     [x] It does not depend on QEC/ZQN/routing/scheduling/runtime.
 *     [x] It contains no Rust code and therefore introduces no unsafe Rust.
 *
 * ============================================================================
 */