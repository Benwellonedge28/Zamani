/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/async.g4
 *
 * Responsibility:
 *     Canonical parser grammar for asynchronous function syntax.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     functions/async.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     target-independent IR
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *       classical          quantum/hybrid
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             scheduling/runtime
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no executable Rust and introduces no unsafe
 *     requirement. The Zamani compiler is required to use safe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - async function declarations;
 *     - async function modifiers;
 *     - async function signatures;
 *     - async function bodies;
 *     - async-specific function syntax;
 *     - await expressions;
 *     - async-specific function grammar composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions generally;
 *     - blocks;
 *     - parameters;
 *     - parameter types;
 *     - function types;
 *     - return types;
 *     - generic parameter syntax;
 *     - where clauses;
 *     - statements generally;
 *     - futures as runtime objects;
 *     - task executors;
 *     - schedulers;
 *     - worker counts;
 *     - thread counts;
 *     - CPU topology;
 *     - GPU topology;
 *     - QPU topology;
 *     - hardware resources;
 *     - quantum IR;
 *     - runtime state.
 *
 * Those constructs belong to their canonical grammar/semantic owners.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * `async` expresses source-level asynchronous execution intent.
 *
 * It MUST NOT prescribe:
 *
 *     - a particular operating-system thread;
 *     - a thread pool;
 *     - a reactor;
 *     - an executor;
 *     - a number of workers;
 *     - a CPU;
 *     - a GPU;
 *     - a QPU;
 *     - a network transport;
 *     - a queue implementation;
 *     - a scheduling policy.
 *
 * The same source program may therefore be lowered to:
 *
 *     - synchronous execution;
 *     - a state machine;
 *     - cooperative execution;
 *     - event-driven execution;
 *     - distributed execution;
 *     - accelerator execution;
 *     - heterogeneous execution;
 *     - quantum/classical orchestration;
 *     - a future execution mechanism.
 *
 * The choice belongs to semantic analysis, compilation, scheduling and
 * runtime/target realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants for:
 *
 *     MAX_TASKS
 *     MAX_FUTURES
 *     MAX_AWAIT_DEPTH
 *     MAX_ASYNC_FUNCTIONS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * The grammar therefore imposes no artificial finite machine-size limit.
 *
 * Resource limitations are represented and enforced downstream through:
 *
 *     - resource requirements;
 *     - capabilities;
 *     - constraints;
 *     - scheduling;
 *     - runtime policy;
 *     - target capabilities.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar REUSES canonical rules supplied by the surrounding grammar:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     parameterList
 *     returnType
 *     genericParameters
 *     whereClause
 *     typeExpression
 *
 * It MUST NOT redefine those rules.
 *
 * The parent `functions.g4` grammar is responsible for composing this module
 * with the other function grammar modules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser lowers:
 *
 *     async fn name(...) { ... }
 *
 * into the repository's canonical async-function AST representation.
 *
 * The existing repository already has a dedicated async AST representation
 * whose purpose is source-level structure rather than runtime execution.
 *
 * This grammar therefore does NOT introduce:
 *
 *     Future<T>
 *     TaskHandle
 *     Executor
 *     RuntimeTaskId
 *     PollState
 *     Waker
 *     Worker
 *     Thread
 *
 * as grammar-owned runtime concepts.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax accepts an asynchronous function.
 *
 * Semantic analysis determines:
 *
 *     - whether the signature is type-correct;
 *     - the resulting async function type;
 *     - whether captures are valid;
 *     - ownership/lifetime requirements;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - cancellation behavior;
 *     - suspension semantics;
 *     - domain-specific execution requirements.
 *
 * The grammar does none of these checks.
 *
 * ============================================================================
 * QUANTUM / HYBRID COMPATIBILITY
 * ============================================================================
 *
 * An async function may contain:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - measurement;
 *     - classical feedback;
 *     - distributed operations;
 *     - accelerator operations;
 *     - HDL interaction;
 *     - future computational domains.
 *
 * None of those domains are encoded into this grammar.
 *
 * In particular, this file contains no:
 *
 *     - qubit count;
 *     - physical-qubit identifier;
 *     - QPU identifier;
 *     - gate-set requirement;
 *     - topology;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - QEC implementation;
 *     - ZQN model.
 *
 * Quantum semantic lowering remains downstream and `quantum::ir` remains the
 * canonical quantum semantic boundary.
 *
 * ============================================================================
 * INTEGRATION GUARANTEE
 * ============================================================================
 *
 * Completing this file must not require later modification merely because:
 *
 *     - the runtime changes;
 *     - a new scheduler is introduced;
 *     - a new QPU is supported;
 *     - a new CPU architecture appears;
 *     - GPU execution is added;
 *     - distributed execution changes;
 *     - resource discovery changes;
 *     - hardware topology changes.
 *
 * Those are downstream concerns.
 *
 * This file only needs modification when the Zamani SOURCE LANGUAGE itself
 * changes its asynchronous syntax.
 *
 * ============================================================================
 */

parser grammar AsyncFunctions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * 1. ASYNC FUNCTION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     async fn compute() {
 *         ...
 *     }
 *
 * With return type:
 *
 *     async fn compute() -> Result {
 *         ...
 *     }
 *
 * With generics:
 *
 *     async fn compute<T>(value: T) -> T {
 *         ...
 *     }
 *
 * With constraints:
 *
 *     async fn compute<T>(value: T) -> T
 *     where T: Trait {
 *         ...
 *     }
 *
 * The ordinary function grammar owns the underlying signature components.
 * This rule only establishes the async function form.
 */

asyncFunctionDeclaration
    : ASYNC
      functionDeclarationCore
    ;


/*
 * ============================================================================
 * 2. ASYNC FUNCTION CORE
 * ============================================================================
 *
 * `functionDeclarationCore` is intentionally composed from canonical function
 * components rather than redefining:
 *
 *     identifier
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *     blockExpression
 *
 * The exact canonical function production is owned by `functions.g4`.
 *
 * This rule is the integration seam between async syntax and ordinary
 * function syntax.
 */

functionDeclarationCore
    : FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      blockExpression
    ;


/*
 * ============================================================================
 * 3. ASYNC FUNCTION SIGNATURE
 * ============================================================================
 *
 * Reusable signature form for contexts that need to declare an async
 * callable without immediately defining its body.
 *
 * Example:
 *
 *     async fn compute(value: T) -> U;
 *
 * The semicolon form is deliberately separate from the definition form so
 * declarations and definitions can be validated independently.
 */

asyncFunctionSignature
    : ASYNC
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
    ;


/*
 * ============================================================================
 * 4. ASYNC FUNCTION DECLARATION / DEFINITION
 * ============================================================================
 *
 * A reusable production for callers such as:
 *
 *     declarations
 *     interfaces
 *     traits
 *     modules
 *     foreign declarations
 *     function dispatchers
 *
 * A definition owns its block.
 */

asyncFunctionDefinition
    : asyncFunctionSignature
      blockExpression
    ;


/*
 * ============================================================================
 * 5. ASYNC FUNCTION DECLARATION WITHOUT BODY
 * ============================================================================
 *
 * This is intentionally syntactic only.
 *
 * Whether a body-less async function is legal in a particular context is
 * determined by the enclosing declaration grammar and semantic analysis.
 */

asyncFunctionPrototype
    : asyncFunctionSignature
      SEMI
    ;


/*
 * ============================================================================
 * 6. ASYNC MODIFIER
 * ============================================================================
 *
 * This rule exists so function composition can use a stable named production
 * rather than duplicating the token directly.
 *
 * It does NOT define the lexical token. The lexer owns ASYNC.
 */

asyncModifier
    : ASYNC
    ;


/*
 * ============================================================================
 * 7. AWAIT EXPRESSION
 * ============================================================================
 *
 * Canonical form:
 *
 *     await computation
 *
 *     await computation()
 *
 *     await future.value
 *
 * The operand is the canonical Zamani expression.
 *
 * This rule deliberately does NOT introduce a special future grammar.
 *
 * Whether an expression is awaitable is a semantic/type-system question.
 */

awaitExpression
    : AWAIT expression
    ;


/*
 * ============================================================================
 * 8. AWAIT STATEMENT
 * ============================================================================
 *
 * The statement form exists for statement dispatchers that distinguish
 * expression statements from specialized statements.
 *
 * Example:
 *
 *     await operation();
 *
 * A caller that already has a canonical expression-statement rule may instead
 * represent `awaitExpression` as an ordinary expression.
 */

awaitStatement
    : awaitExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 9. ASYNC FUNCTION MEMBER
 * ============================================================================
 *
 * Reusable production for declaration containers that need to distinguish
 * asynchronous functions from ordinary functions.
 *
 * Attributes/visibility remain owned by the surrounding declaration grammar.
 */

asyncFunctionMember
    : asyncFunctionDefinition
    | asyncFunctionPrototype
    ;


/*
 * ============================================================================
 * 10. ASYNC CALLABLE
 * ============================================================================
 *
 * A reusable syntactic category for callers that need to accept either:
 *
 *     async function definition
 *     async function prototype
 *
 * No runtime callable type is introduced here.
 */

asyncCallableDeclaration
    : asyncFunctionMember
    ;


/*
 * ============================================================================
 * 11. ASYNC FUNCTION WITH TRAILING ATTRIBUTES
 * ============================================================================
 *
 * Attributes are owned by the canonical attribute grammar.
 *
 * This rule exists only as an integration seam for function dispatchers.
 *
 * If the canonical function dispatcher already owns attributes, it SHOULD
 * consume `attribute*` before selecting this rule instead of using this rule.
 *
 * Consequently this grammar does not redefine `attribute`.
 */

asyncAttributedFunction
    : attribute*
      asyncFunctionDefinition
    ;


/*
 * ============================================================================
 * 12. ASYNC DECLARATION DISPATCH
 * ============================================================================
 *
 * This is the single reusable entry point intended for `functions.g4`.
 *
 * It prevents other function grammar files from having to know the internal
 * structure of async declarations.
 */

asyncFunction
    : asyncFunctionDefinition
    | asyncFunctionPrototype
    ;


/*
 * ============================================================================
 * 13. ASYNC EXPRESSION DISPATCH
 * ============================================================================
 *
 * This is the single reusable entry point intended for expression dispatch.
 *
 * It currently contains `awaitExpression`.
 *
 * Future async source constructs may be added here only when they become
 * actual Zamani language syntax. Runtime concepts must never be added merely
 * because a runtime implements them.
 */

asyncExpression
    : awaitExpression
    ;