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
 * Role:
 *     Canonical parser grammar for source-level Zamani functions.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar owns the SYNTAX of:
 *
 *     - function declarations;
 *     - function definitions;
 *     - function names;
 *     - function modifiers;
 *     - generic function parameters;
 *     - generic bounds;
 *     - ordinary parameters;
 *     - variadic parameters;
 *     - parameter defaults;
 *     - return-type attachment;
 *     - function effects attachment;
 *     - function contracts;
 *     - function bodies;
 *     - declaration/prototype termination.
 *
 * This grammar DOES NOT own:
 *
 *     - lexical token definitions;
 *     - type semantics;
 *     - expression semantics;
 *     - statement semantics;
 *     - name resolution;
 *     - type inference;
 *     - generic substitution;
 *     - ownership/borrowing;
 *     - effect checking;
 *     - capability checking;
 *     - resource allocation;
 *     - hardware selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum IR;
 *     - runtime execution;
 *     - ABI selection;
 *     - calling-convention selection.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A function expresses portable computation and intent.
 *
 * Function syntax MUST NOT encode:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - physical addresses;
 *     - device identifiers;
 *     - topology;
 *     - vendor-specific hardware;
 *     - deployment size.
 *
 * There are intentionally no grammar constants such as:
 *
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_FUNCTIONS
 *     MAX_QUBITS
 *     MAX_THREADS
 *
 * Repetition is represented recursively/repetitively in the grammar and is
 * bounded only by implementation resource policies.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing determines whether function syntax is structurally valid.
 *
 * Semantic analysis subsequently determines:
 *
 *     - whether names are valid;
 *     - whether names are unique;
 *     - whether types exist;
 *     - whether generic bounds are satisfiable;
 *     - whether defaults are type-compatible;
 *     - whether effects are permitted;
 *     - whether contracts are valid;
 *     - whether a body returns correctly;
 *     - whether async/generator/compile-time modifiers are legal;
 *     - whether foreign declarations satisfy ABI requirements;
 *     - whether capability/resource requirements can be satisfied.
 *
 * None of those decisions are encoded as target-specific grammar rules.
 *
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * A function may operate over:
 *
 *     classical values
 *     quantum values
 *     logical qubits
 *     hardware abstractions
 *     tensors
 *     distributed values
 *     accelerator resources
 *     future domain types
 *
 * The grammar does not distinguish those cases by machine-specific syntax.
 *
 * For example:
 *
 *     fn transform<T>(value: T) -> T { ... }
 *
 * is portable regardless of whether T eventually represents:
 *
 *     - a scalar;
 *     - a tensor;
 *     - a quantum abstraction;
 *     - a distributed object;
 *     - a hardware resource;
 *     - another future semantic type.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This grammar is a parser delegate.
 *
 * It imports canonical parser grammars for:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * Therefore this file MUST NOT duplicate:
 *
 *     typeExpression
 *     expression
 *     block
 *
 * The aggregate Zamani parser is responsible for composing the complete
 * language grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical modular lexer vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *
 *     ZamaniTokens
 *
 * This file MUST NOT declare lexer rules.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Grammar-generated/compiler integration MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Rust implementation code MUST use safe Rust only.
 *
 * The compiler crate MUST enforce:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * This grammar itself contains no Rust implementation code.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite grammar-level ceiling is imposed on:
 *
 *     - function count;
 *     - parameter count;
 *     - generic parameter count;
 *     - generic nesting;
 *     - recursion depth;
 *     - source program size;
 *     - quantum resource count;
 *     - distributed resource count.
 *
 * Any implementation limit MUST live in explicit parser/compiler resource
 * policy rather than in the language grammar.
 *
 * ============================================================================
 */

parser grammar Functions;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * Canonical parser composition.
 *
 * These delegates provide the canonical:
 *
 *     typeExpression
 *     expression
 *     block
 *
 * rules used below.
 *
 * The repository's aggregate parser must ensure that these delegates use the
 * same canonical ZamaniTokens vocabulary.
 */
import Types, Expressions, Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete function declaration.
 *
 * Examples:
 *
 *     fn main() {
 *     }
 *
 *     pub fn add(a: int, b: int) -> int {
 *         return a + b;
 *     }
 *
 *     fn identity<T>(value: T) -> T {
 *         return value;
 *     }
 *
 *     extern fn foreign_call(value: int) -> int;
 *
 *     fn declaration_only(value: int) -> int;
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
      functionContractAttachment*
      functionImplementation
    ;


/* ============================================================================
 * 2. FUNCTION NAME
 * ========================================================================== */

/**
 * Function names are ordinary identifiers.
 *
 * Qualified names are deliberately NOT accepted here.
 *
 * A declaration belongs to its containing module/namespace; qualification is
 * handled by module/name-resolution infrastructure rather than embedded into
 * the function's local declaration syntax.
 */
functionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 3. FUNCTION MODIFIERS
 * ========================================================================== */

/**
 * Function modifiers describe source-level declaration properties.
 *
 * Their compatibility and semantics are checked later.
 *
 * Hardware-specific modifiers do not belong here.
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
    ;


/* ============================================================================
 * 4. GENERIC FUNCTION PARAMETERS
 * ========================================================================== */

/**
 * Generic parameter list.
 *
 * No finite generic arity is encoded.
 *
 * Examples:
 *
 *     <T>
 *     <T, U>
 *     <T: Numeric>
 *     <T: Numeric + Comparable>
 */
functionGenericParameters
    : LESS_THAN
      functionGenericParameter
      (COMMA functionGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


/**
 * Individual generic parameter.
 *
 * A generic parameter may have zero or more semantic bounds.
 */
functionGenericParameter
    : IDENTIFIER
      functionGenericBounds?
    ;


/**
 * Generic bounds.
 *
 * Multiple bounds are represented structurally.
 *
 * The semantic/type system decides whether the bounds are meaningful and
 * satisfiable.
 */
functionGenericBounds
    : COLON
      functionGenericBound
      (PLUS functionGenericBound)*
    ;


/**
 * A bound is represented by the canonical type-expression grammar.
 *
 * This intentionally permits:
 *
 *     T: Numeric
 *     T: quantum::State
 *     T: hardware::Resource
 *     T: SomeFutureCapability
 *
 * without hard-coding those semantic domains into function syntax.
 */
functionGenericBound
    : typeExpression
    ;


/* ============================================================================
 * 5. FUNCTION PARAMETERS
 * ========================================================================== */

/**
 * Function parameter list.
 *
 * Variadic parameters are integrated directly into the canonical list rather
 * than maintaining a second competing list production.
 *
 * This prevents the old architecture's ambiguity where:
 *
 *     functionParameterList
 *     functionParameterListWithVariadic
 *
 * could describe overlapping syntax.
 */
functionParameterList
    : functionParameter
      (COMMA functionParameter)*
      COMMA?
    ;


/**
 * A function parameter is either ordinary or variadic.
 *
 * The grammar does not impose an argument-count limit.
 */
functionParameter
    : ordinaryFunctionParameter
    | variadicFunctionParameter
    ;


/* ============================================================================
 * 6. ORDINARY PARAMETERS
 * ========================================================================== */

/**
 * Ordinary parameter.
 *
 * Examples:
 *
 *     value
 *     value: int
 *     value: int = 0
 *     mut value: Buffer
 */
ordinaryFunctionParameter
    : functionParameterModifier*
      functionParameterPattern
      functionParameterType?
      functionParameterDefault?
    ;


/**
 * Parameter pattern.
 *
 * The base function grammar accepts an identifier or a mutable identifier.
 *
 * More sophisticated destructuring patterns remain owned by the canonical
 * pattern grammar and can be integrated through the aggregate parser.
 */
functionParameterPattern
    : IDENTIFIER
    | K_MUT IDENTIFIER
    ;


/**
 * Parameter type.
 */
functionParameterType
    : COLON
      typeExpression
    ;


/**
 * Parameter default.
 *
 * The value is a canonical Zamani expression.
 *
 * No target-specific constant folding occurs here.
 */
functionParameterDefault
    : EQUALS
      expression
    ;


/* ============================================================================
 * 7. PARAMETER MODIFIERS
 * ========================================================================== */

/**
 * Parameter-level modifiers.
 *
 * `mut` describes source-level mutability only.
 *
 * It does not select:
 *
 *     - registers;
 *     - memory;
 *     - address spaces;
 *     - physical storage;
 *     - hardware.
 */
functionParameterModifier
    : K_MUT
    ;


/* ============================================================================
 * 8. VARIADIC PARAMETERS
 * ========================================================================== */

/**
 * Variadic parameter.
 *
 * Canonical form:
 *
 *     fn collect(...values: T) -> Result {
 *     }
 *
 * The grammar imposes no finite number of arguments.
 *
 * Placement, uniqueness, ABI legality and invocation rules are semantic
 * validation responsibilities.
 */
variadicFunctionParameter
    : functionParameterModifier*
      ELLIPSIS
      IDENTIFIER
      functionParameterType?
    ;


/* ============================================================================
 * 9. RETURN TYPE
 * ========================================================================== */

/**
 * Optional return type.
 *
 * Examples:
 *
 *     fn work()
 *     fn work() -> int
 *     fn measure() -> Measurement
 *     fn transform<T>(value: T) -> T
 *
 * Type semantics belong to the canonical type system.
 */
functionReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 10. EFFECT ATTACHMENT
 * ========================================================================== */

/**
 * Function effects.
 *
 * Effects are expressed through the canonical effect namespace rather than
 * enumerating all possible effects here.
 *
 * Canonical source form:
 *
 *     fn read() with effects { io }
 *
 *     fn compute() with effects {
 *         quantum
 *         io
 *     }
 *
 * The effect names remain open through qualifiedName.
 *
 * This allows future effect domains without changing this grammar.
 */
functionEffectAttachment
    : K_WITH
      K_EFFECT
      LBRACE
      functionEffectReferenceList?
      RBRACE
    ;


/**
 * Effect list.
 *
 * No fixed effect count is imposed.
 */
functionEffectReferenceList
    : functionEffectReference
      (COMMA functionEffectReference)*
      COMMA?
    ;


/**
 * Effect reference.
 *
 * Qualified names permit:
 *
 *     io
 *     quantum
 *     network
 *     security
 *     distributed
 *     future::effect
 */
functionEffectReference
    : qualifiedFunctionEffectName
    ;


/**
 * Effect names are syntactically qualified identifiers.
 *
 * Semantic effect resolution belongs to the effect subsystem.
 */
qualifiedFunctionEffectName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 11. CONTRACTS
 * ========================================================================== */

/**
 * A function may have zero or more contracts.
 *
 * Multiple contract clauses are retained rather than collapsed by the parser,
 * allowing semantic analysis to preserve source order and provenance.
 *
 * Example:
 *
 *     fn sqrt(x: Float) -> Float
 *         contract {
 *             requires(x >= 0);
 *             ensures(result >= 0);
 *         }
 *     {
 *     }
 */
functionContractAttachment
    : K_CONTRACT
      LBRACE
      functionContractItem*
      RBRACE
    ;


/**
 * Contract items.
 *
 * The actual logical meaning remains the responsibility of semantic analysis.
 */
functionContractItem
    : functionRequiresContract
    | functionEnsuresContract
    | functionInvariantContract
    ;


/**
 * Preconditions.
 */
functionRequiresContract
    : K_REQUIRES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/**
 * Postconditions.
 */
functionEnsuresContract
    : K_ENSURES
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/**
 * Invariants.
 */
functionInvariantContract
    : K_INVARIANT
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * 12. FUNCTION IMPLEMENTATION
 * ========================================================================== */

/**
 * A function is either:
 *
 *     - defined with a body;
 *     - declared without a body.
 *
 * Body-less declarations are useful for:
 *
 *     - extern functions;
 *     - interfaces;
 *     - abstract APIs;
 *     - separately implemented functions;
 *     - compiler/runtime supplied functions;
 *     - foreign interfaces.
 *
 * Whether a body-less declaration is legal is a semantic/contextual question.
 */
functionImplementation
    : block
    | SEMICOLON
    ;


/* ============================================================================
 * 13. FUNCTION SIGNATURE
 * ========================================================================== */

/**
 * Signature-only form for consumers such as:
 *
 *     interfaces
 *     traits
 *     extern declarations
 *     declarations
 *     implementations
 *
 * This rule deliberately excludes a body.
 */
functionSignature
    : functionModifier*
      K_FN
      functionName
      functionGenericParameters?
      LPAREN
      functionParameterList?
      RPAREN
      functionReturnClause?
      functionEffectAttachment?
      functionContractAttachment*
      SEMICOLON?
    ;


/* ============================================================================
 * 14. FOREIGN/EXTERNAL FUNCTION INTEGRATION
 * ========================================================================== */

/**
 * Foreign-function declarations are NOT given a separate function syntax
 * here.
 *
 * `extern fn ...;`
 *
 * is parsed as an ordinary function declaration using K_EXTERN.
 *
 * The dedicated foreign-functions.g4 grammar owns additional foreign-language
 * and ABI syntax.
 *
 * This prevents functions.g4 from becoming an ABI grammar.
 */
foreignFunctionSignature
    : K_EXTERN
      K_FN
      functionName
      functionGenericParameters?
      LPAREN
      functionParameterList?
      RPAREN
      functionReturnClause?
      functionEffectAttachment?
      functionContractAttachment*
      SEMICOLON
    ;


/* ============================================================================
 * 15. COMPILE-TIME FUNCTION INTEGRATION
 * ========================================================================== */

/**
 * Compile-time functions use the normal function declaration boundary.
 *
 * The compile-time subsystem may interpret an established modifier/attribute
 * or provide a specialized declaration rule in the aggregate grammar.
 *
 * No compile-time evaluator is implemented here.
 */
compileTimeFunctionSignature
    : functionSignature
    ;


/* ============================================================================
 * 16. ASYNC FUNCTION INTEGRATION
 * ========================================================================== */

/**
 * Async functions use the normal function declaration with K_ASYNC.
 *
 * Async scheduling/execution semantics belong to the concurrency/runtime
 * subsystems.
 *
 * This rule is provided as an integration boundary and does not duplicate
 * function syntax.
 */
asyncFunctionDeclaration
    : K_ASYNC
      K_FN
      functionName
      functionGenericParameters?
      LPAREN
      functionParameterList?
      RPAREN
      functionReturnClause?
      functionEffectAttachment?
      functionContractAttachment*
      functionImplementation
    ;


/* ============================================================================
 * 17. GENERATOR FUNCTION INTEGRATION
 * ========================================================================== */

/**
 * Generator semantics are represented by the ordinary function body and the
 * canonical `yield` statement.
 *
 * The generators grammar owns generator-specific semantic validation.
 */
generatorFunctionDeclaration
    : functionDeclaration
    ;


/* ============================================================================
 * 18. FUNCTION CALL CONTRACT
 * ========================================================================== */

/**
 * This rule is intentionally NOT a function-call expression.
 *
 * Function invocation belongs to expressions/calls.g4.
 *
 * Keeping invocation outside this file prevents:
 *
 *     function declaration syntax
 *
 * from becoming coupled to:
 *
 *     calling syntax.
 */


/* ============================================================================
 * 19. FUNCTION DECLARATION SEMANTIC NOTES
 * ========================================================================== */

/*
 * The parser MUST NOT reject a function because:
 *
 *     - its parameter type is quantum;
 *     - its return type is quantum;
 *     - it references a hardware abstraction;
 *     - it requires distributed execution;
 *     - it uses an accelerator type;
 *     - it contains a large generic structure;
 *     - it has many parameters;
 *     - it has nested generic parameters;
 *     - it has a future domain type.
 *
 * Such questions belong to semantic analysis and downstream compilation.
 */


/* ============================================================================
 * 20. HARD-CODING PROHIBITIONS
 * ========================================================================== */

/*
 * Forbidden in this grammar:
 *
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_FUNCTION_DEPTH
 *     MAX_FUNCTIONS
 *     MAX_CALL_DEPTH
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Forbidden machine-specific syntax:
 *
 *     fn @device0 ...
 *     fn @qpu7 ...
 *     fn @core3 ...
 *     fn @gpu2 ...
 *
 * unless such notation is explicitly introduced by a separate, target-specific
 * deployment/placement language.
 */


/* ============================================================================
 * 21. SOURCE-LEVEL PORTABILITY
 * ========================================================================== */

/*
 * Valid examples include:
 *
 *     fn add<T>(a: T, b: T) -> T {
 *         ...
 *     }
 *
 *     fn process(q: Qubit) -> Result {
 *         ...
 *     }
 *
 *     fn transform<T: Numeric>(value: T) -> T {
 *         ...
 *     }
 *
 *     async fn execute<T>(value: T) -> T {
 *         ...
 *     }
 *
 *     extern fn foreign_operation(value: int) -> int;
 *
 * None of these declarations specifies:
 *
 *     - physical hardware;
 *     - topology;
 *     - resource count;
 *     - device ID;
 *     - scheduling policy.
 *
 * Therefore the same source-level function may participate in the POCO-REAF
 * compilation model.
 */


/* ============================================================================
 * 22. AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST corresponding to this grammar MUST preserve at minimum:
 *
 *     FunctionDeclaration
 *         modifiers
 *         name
 *         generic_parameters
 *         parameters
 *         return_type
 *         effects
 *         contracts
 *         body_or_declaration
 *         source_span
 *
 * Parameter nodes MUST preserve:
 *
 *     name/pattern
 *     mutability
 *     type
 *     default_value
 *     variadic
 *     source_span
 *
 * Generic parameter nodes MUST preserve:
 *
 *     name
 *     bounds
 *     source_span
 *
 * The AST MUST NOT lower:
 *
 *     Qubit
 *
 * into:
 *
 *     physical qubit index
 *
 * nor:
 *
 *     hardware::Resource
 *
 * into:
 *
 *     concrete device allocation.
 */


/* ============================================================================
 * 23. IR CONTRACT
 * ========================================================================== */

/*
 * Function syntax lowers through semantic analysis.
 *
 * The grammar MUST NOT construct IR directly.
 *
 * Canonical direction:
 *
 *     Functions
 *         |
 *         v
 *     Frontend AST
 *         |
 *         v
 *     Name/type/effect/capability analysis
 *         |
 *         +----------------------+
 *         |                      |
 *         v                      v
 *     classical/control IR   quantum semantic IR
 *                                |
 *                                v
 *                           quantum::ir
 *
 * Function grammar MUST NOT:
 *
 *     - create QuantumGate values;
 *     - create QubitId values;
 *     - assign physical qubits;
 *     - choose routing;
 *     - schedule operations;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - choose hardware.
 */


/* ============================================================================
 * 24. ERROR CONTRACT
 * ========================================================================== */

/*
 * Syntax errors belong to the parser diagnostics layer.
 *
 * Semantic errors belong to semantic analysis.
 *
 * The parser MUST preserve:
 *
 *     - source location;
 *     - offending token;
 *     - expected grammar category;
 *     - parser context.
 *
 * The grammar MUST NOT silently reinterpret malformed function declarations.
 */


/* ============================================================================
 * 25. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * The grammar must produce deterministic parse structure for valid source.
 *
 * In particular:
 *
 *     functionParameter
 *
 * has explicit alternatives for:
 *
 *     ordinaryFunctionParameter
 *     variadicFunctionParameter
 *
 * and the variadic marker ELLIPSIS is lexically distinct from DOT/DOT_DOT.
 *
 * The grammar MUST NOT depend on:
 *
 *     - hardware state;
 *     - runtime state;
 *     - random values;
 *     - network state;
 *     - backend discovery.
 */


/* ============================================================================
 * 26. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete only when:
 *
 * [ ] It is the sole function-declaration syntax authority.
 * [ ] ZamaniTokens is the canonical token vocabulary.
 * [ ] Types is the canonical type-expression authority.
 * [ ] Expressions is the canonical expression authority.
 * [ ] Statements is the canonical block/body authority.
 * [ ] No function grammar duplicates type syntax.
 * [ ] No function grammar duplicates expression precedence.
 * [ ] No function grammar duplicates statement syntax.
 * [ ] Generic arity is unbounded by grammar design.
 * [ ] Parameter arity is unbounded by grammar design.
 * [ ] Variadic syntax has one canonical representation.
 * [ ] Return types use typeExpression.
 * [ ] Defaults use expression.
 * [ ] Function bodies use block.
 * [ ] Effects remain extensible.
 * [ ] Contracts preserve source expressions.
 * [ ] Extern functions remain syntactically portable.
 * [ ] ABI decisions remain outside this file.
 * [ ] Quantum functions remain hardware independent.
 * [ ] No physical resource limits occur in this grammar.
 * [ ] No unsafe Rust dependency is introduced.
 * [ ] Positive parser tests exist.
 * [ ] Negative parser tests exist.
 * [ ] Boundary/scalability tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] Round-trip tests exist where the AST printer supports them.
 * [ ] Core.g4 no longer contains a competing functionDeclaration rule.
 */