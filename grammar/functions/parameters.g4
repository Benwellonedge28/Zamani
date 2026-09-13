/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/parameters.g4
 *
 * Role:
 *     Canonical reusable parser grammar for function/callable parameters.
 *
 * Grammar layer:
 *     Parser-only delegate grammar.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     No Rust `unsafe` code is required or permitted.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         UTF-8 source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                    canonical parser grammar
 *                              |
 *             +----------------+----------------+
 *             |                                 |
 *             v                                 v
 *        Parameters.g4                    other syntax
 *             |                                 |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         Frontend AST
 *                              |
 *                              v
 *                    semantic/type analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          classical       quantum::ir      resource/effect
 *          lowering                         analysis
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                 optimization / scheduling /
 *                 routing / resilience / ZQN
 *                              |
 *                              v
 *                       target realization
 *
 * This grammar owns ONLY the concrete syntax of callable parameters.
 *
 * It does NOT own:
 *
 *   - function semantics;
 *   - function overloading;
 *   - name resolution;
 *   - type inference;
 *   - type compatibility;
 *   - generic constraint solving;
 *   - ownership;
 *   - borrowing;
 *   - lifetime analysis;
 *   - effect checking;
 *   - capability checking;
 *   - resource checking;
 *   - ABI lowering;
 *   - calling conventions;
 *   - register allocation;
 *   - stack layout;
 *   - hardware placement;
 *   - quantum allocation;
 *   - physical qubit assignment;
 *   - scheduling;
 *   - optimization;
 *   - runtime dispatch.
 *
 * ============================================================================
 * CANONICAL SPECIFICATION
 * ============================================================================
 *
 * The canonical source-language specification defines:
 *
 * ParameterList ::= Parameter { "," Parameter } ;
 *
 * Parameter ::=
 *     ["mut"]
 *     IDENT
 *     [":" TypeExpression]
 *     ["=" Expression] ;
 *
 * Examples:
 *
 *     fn add(a: Int, b: Int) -> Int { ... }
 *
 *     fn compute(input: Data) -> Result { ... }
 *
 *     fn transform(mut value: Value) -> Value { ... }
 *
 *     fn configure(value: Config = default_config) -> Result { ... }
 *
 *     fn infer(value) { ... }
 *
 * Missing type annotations are syntactically valid.
 *
 * Whether a missing type is accepted semantically is decided by the
 * type-system policy. This grammar MUST NOT silently convert it into a
 * dynamic type.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Parameters describe the logical interface of a callable computation.
 *
 * They MUST NOT encode:
 *
 *   - maximum parameter count;
 *   - maximum argument count;
 *   - maximum parameter name length beyond the lexer policy;
 *   - CPU count;
 *   - GPU count;
 *   - QPU count;
 *   - qubit count;
 *   - memory capacity;
 *   - register count;
 *   - machine word size;
 *   - device identity;
 *   - topology;
 *   - network size;
 *   - hardware address;
 *   - accelerator identity;
 *   - vendor ABI;
 *   - target-specific calling convention.
 *
 * Any finite limitation encountered during compilation or execution belongs
 * to the appropriate resource, target, runtime, or operating-environment
 * layer.
 *
 * Therefore the grammar accepts arbitrarily many syntactically valid
 * parameters subject only to the available parser/runtime resources.
 *
 * There is intentionally NO grammar rule such as:
 *
 *     parameter parameter? parameter?
 *
 * or any other artificial finite expansion.
 *
 * Repetition operators are used instead.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar establishes:
 *
 *     parameter syntax
 *
 * It does NOT establish:
 *
 *     parameter meaning
 *
 * The frontend AST should preserve at least:
 *
 *     - source span;
 *     - mutability marker;
 *     - parameter binding/pattern;
 *     - optional type expression;
 *     - optional default expression.
 *
 * Semantic analysis subsequently determines:
 *
 *     - whether the parameter name is valid;
 *     - whether the pattern is supported;
 *     - whether the type is well-formed;
 *     - whether the type can be inferred;
 *     - whether the default expression is type-compatible;
 *     - whether the default is legal in the function context;
 *     - ownership/borrowing semantics;
 *     - effect/capability requirements;
 *     - resource requirements;
 *     - ABI representation;
 *     - lowering strategy.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * `parameters.g4` owns:
 *
 *     parameterList
 *     parameter
 *
 * It does NOT own:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * Those rules belong to their respective grammar layers and are deliberately
 * referenced here rather than duplicated.
 *
 * This prevents:
 *
 *     parameters.g4 -> duplicate type grammar
 *     parameters.g4 -> duplicate expression grammar
 *     parameters.g4 -> duplicate identifier grammar
 *
 * Such duplication would create semantic drift.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This file is intended to be imported by the canonical parser grammar.
 *
 * Example:
 *
 *     parser grammar CoreParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Parameters;
 *
 * The canonical/root grammar owns the surrounding function declaration:
 *
 *     functionDeclaration
 *         : ...
 *           genericParameters?
 *           LPAREN parameterList? RPAREN
 *           ...
 *         ;
 *
 * Because `parameterList` is supplied by this delegate grammar, the
 * surrounding function grammar remains independent of the implementation
 * details of individual parameters.
 *
 * ANTLR grammar imports compose parser grammars into the root parser grammar.
 *
 * ============================================================================
 * NON-CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Dependency direction:
 *
 *     lexer
 *       |
 *       +--> identifier grammar
 *       +--> type grammar
 *       +--> expression grammar
 *       |
 *       v
 *     parameters.g4
 *       |
 *       v
 *     functions.g4 / core parser
 *
 * There must be NO dependency:
 *
 *     parameters.g4 -> AST
 *     parameters.g4 -> semantic analysis
 *     parameters.g4 -> IR
 *     parameters.g4 -> runtime
 *     parameters.g4 -> hardware
 *
 * In particular:
 *
 *     parameters.g4 -> quantum::ir
 *
 * is forbidden.
 *
 * A quantum parameter remains syntactically a parameter. Its semantic type
 * may later be lowered into the appropriate quantum representation.
 *
 * ============================================================================
 * NO TARGET COUPLING
 * ============================================================================
 *
 * These are intentionally NOT grammar constructs:
 *
 *     cpu_parameter
 *     gpu_parameter
 *     qpu_parameter
 *     physical_qubit_parameter
 *     register_parameter
 *     device_parameter
 *     cuda_parameter
 *     vendor_parameter
 *
 * Target requirements belong to capabilities, resources, compilation
 * context, hardware abstraction, or target lowering.
 *
 * ============================================================================
 * DEFAULT VALUE CONTRACT
 * ============================================================================
 *
 * A default value is an expression:
 *
 *     parameter := expression
 *
 * The grammar does not evaluate the expression.
 *
 * It does not determine:
 *
 *     - whether evaluation is compile-time;
 *     - whether evaluation is runtime;
 *     - whether evaluation has side effects;
 *     - whether the expression is deterministic;
 *     - whether the expression is resource-consuming.
 *
 * Those decisions belong to semantic analysis and the execution model.
 *
 * ============================================================================
 * MUTABILITY CONTRACT
 * ============================================================================
 *
 * `mut` is syntactic metadata attached to the parameter binding.
 *
 * It does not mean:
 *
 *     - mutable hardware;
 *     - mutable quantum state;
 *     - mutable physical qubit;
 *     - mutable memory allocation;
 *     - writable device state.
 *
 * The semantic/type/ownership layers define what mutation means for the
 * parameter's actual type.
 *
 * ============================================================================
 * GENERIC PARAMETER CONTRACT
 * ============================================================================
 *
 * Generic parameters are NOT defined here.
 *
 * A function may syntactically contain:
 *
 *     genericParameters?
 *
 * before its parameter list.
 *
 * Generic parameter declarations belong to the generic/type grammar.
 *
 * Example:
 *
 *     fn map<T>(value: T) -> T { ... }
 *
 * The parameter grammar only owns:
 *
 *     value: T
 *
 * It does not own:
 *
 *     <T>
 *
 * ============================================================================
 * PATTERN CONTRACT
 * ============================================================================
 *
 * The canonical syntax specification currently defines a parameter as:
 *
 *     ["mut"] IDENT
 *
 * rather than an arbitrary pattern.
 *
 * Therefore this grammar intentionally does NOT broaden parameters to:
 *
 *     tuple patterns
 *     array patterns
 *     destructuring patterns
 *     or-patterns
 *     range patterns
 *     arbitrary match patterns
 *
 * If parameter destructuring is introduced in a future language version,
 * it must be added through an explicit language-specification change,
 * AST contract, semantic contract, diagnostics, compatibility policy,
 * and tests.
 *
 * This prevents `pattern` from accidentally making parameter syntax broader
 * than the canonical language specification.
 *
 * ============================================================================
 * TRAILING COMMA CONTRACT
 * ============================================================================
 *
 * The canonical ParameterList production is:
 *
 *     Parameter { "," Parameter }
 *
 * Therefore a trailing comma is NOT accepted here.
 *
 * This is deliberate.
 *
 * A trailing-comma policy must be language-wide rather than independently
 * invented by individual grammar files.
 *
 * If Zamani later adopts trailing commas universally, that should be changed
 * in the canonical syntax specification and all affected grammar contracts
 * together.
 *
 * ============================================================================
 * ERROR BEHAVIOUR
 * ============================================================================
 *
 * This grammar must allow the parser to report deterministic syntax errors
 * for malformed parameter declarations.
 *
 * Examples of invalid syntax:
 *
 *     fn f(, x) { ... }
 *     fn f(x:) { ... }
 *     fn f(: Int) { ... }
 *     fn f(x = ) { ... }
 *     fn f(x: Int,) { ... }
 *
 * The grammar itself MUST NOT:
 *
 *     - recover by silently deleting a parameter;
 *     - invent a parameter name;
 *     - invent a type;
 *     - invent a default value;
 *     - reinterpret malformed syntax as a different construct.
 *
 * Error recovery is owned by the parser/frontend diagnostic infrastructure.
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * Nothing in this grammar requires Rust unsafe code.
 *
 * Generated parser/runtime integration MUST remain compatible with the
 * repository's safe-Rust requirement.
 *
 * Rust 1.97 and Rust 1.97.1 are supported implementation baselines.
 *
 * ============================================================================
 */

parser grammar Parameters;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PARAMETER LIST
 * ========================================================================== */

/**
 * ParameterList ::= Parameter { "," Parameter } ;
 *
 * A parameter list contains one or more parameters.
 *
 * The surrounding function/callable declaration is responsible for making
 * the entire list optional:
 *
 *     LPAREN parameterList? RPAREN
 *
 * This rule intentionally has no finite parameter-count bound.
 */
parameterList
    : parameter (COMMA parameter)*
    ;


/* ============================================================================
 * PARAMETER
 * ========================================================================== */

/**
 * Parameter ::=
 *     ["mut"]
 *     IDENT
 *     [":" TypeExpression]
 *     ["=" Expression] ;
 *
 * The order is intentional:
 *
 *     mut? identifier type? default?
 *
 * Examples:
 *
 *     value
 *     value: Int
 *     mut value
 *     mut value: Int
 *     value = default_value
 *     value: Int = default_value
 *     mut value: Int = default_value
 */
parameter
    : MUT? identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;


/* ============================================================================
 * INTEGRATION ASSERTION
 * ============================================================================
 *
 * The following rules are intentionally referenced rather than redefined:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * Their ownership remains in the corresponding grammar modules.
 *
 * This file therefore has exactly two owned parser rules:
 *
 *     parameterList
 *     parameter
 *
 * Any future attempt to add duplicate identifier/type/expression rules here
 * should be rejected during grammar review.
 * ============================================================================
 */