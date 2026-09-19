/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/async.g4
 *
 * Grammar:
 *     AsyncFunctions
 *
 * Status:
 *     CANONICAL production async-syntax delegate
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This file contains parser grammar only.
 *
 *     It contains:
 *       - no embedded Rust;
 *       - no semantic predicates;
 *       - no runtime execution;
 *       - no filesystem access;
 *       - no networking;
 *       - no hardware discovery;
 *       - no target selection;
 *       - no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE OWNER of syntax that is intrinsically asynchronous
 * but is not itself the complete function-declaration grammar.
 *
 * It provides the reusable async syntax consumed by:
 *
 *     grammar/functions/functions.g4
 *     grammar/expressions/expressions.g4
 *
 * The two production constructs owned here are:
 *
 *     asyncModifier
 *     awaitExpression
 *
 * Function declaration structure remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * Therefore:
 *
 *     async fn f(...) -> T { ... }
 *
 * is assembled by:
 *
 *     Functions
 *         |
 *         +--> asyncModifier
 *         +--> functionName
 *         +--> functionGenericParameters
 *         +--> parameterList
 *         +--> functionReturnClause
 *         +--> functionEffectClause
 *         +--> functionContractClause
 *         +--> functionImplementation
 *
 * This file MUST NOT recreate that complete declaration.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     function grammar          expression grammar
 *          |                          |
 *          +----------+---------------+
 *                     |
 *                     v
 *               async syntax
 *                     |
 *                     v
 *             domain-neutral AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical   quantum::ir  HDL/hardware
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *               optimization
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *       routing   scheduling  resilience
 *                                |
 *                                v
 *                               QEC
 *                                |
 *                                v
 *                               ZQN
 *                                |
 *                                v
 *                               HAL
 *                                |
 *                                v
 *                         target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - asyncModifier;
 *     - awaitExpression;
 *     - the source-level association of `async` with an async-capable
 *       expression/function grammar position;
 *     - the lexical/parser boundary for `await`;
 *     - the reusable integration contract for async syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - functionDeclaration;
 *     - functionDefinition;
 *     - functionSignature;
 *     - functionName;
 *     - parameters;
 *     - generic parameters;
 *     - return clauses;
 *     - effects;
 *     - contracts;
 *     - function bodies;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - blocks;
 *     - statements;
 *     - types;
 *     - futures;
 *     - promises;
 *     - tasks;
 *     - executors;
 *     - schedulers;
 *     - workers;
 *     - threads;
 *     - queues;
 *     - polling;
 *     - wake mechanisms;
 *     - cancellation implementation;
 *     - runtime state;
 *     - resource allocation;
 *     - hardware selection;
 *     - device placement;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - IR construction.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one owner for each of these rules:
 *
 *     asyncModifier
 *     awaitExpression
 *
 * There MUST NOT be another authoritative definition of either rule in:
 *
 *     grammar/functions/functions.g4
 *     grammar/concurrency/
 *     grammar/expressions/
 *     grammar/antlr/
 *     grammar/Zamani.g4
 *
 * Other grammars may CONSUME these rules through parser composition.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` remains the canonical owner of the complete
 * function declaration.
 *
 * Its modifier dispatch SHOULD consume:
 *
 *     asyncModifier
 *
 * rather than directly owning another async modifier production.
 *
 * Conceptually:
 *
 *     functionModifier
 *         : PUBLIC
 *         | ...
 *         | asyncModifier
 *         | ...
 *         ;
 *
 * Therefore:
 *
 *     async fn compute(...) { ... }
 *
 * is parsed by the ordinary function declaration structure.
 *
 * The complete function remains one AST function declaration with an async
 * source modifier.
 *
 * This is deliberate.
 *
 * It prevents:
 *
 *     functionDeclaration
 *     asyncFunctionDeclaration
 *
 * from becoming two competing declarations with subtly different support for:
 *
 *     generics
 *     parameters
 *     returns
 *     effects
 *     contracts
 *     prototypes
 *     bodies
 *     attributes
 *     visibility
 *     future function features
 *
 * ============================================================================
 * RETURN INTEGRATION
 * ============================================================================
 *
 * Return syntax remains owned by:
 *
 *     grammar/functions/returns.g4
 *
 * Therefore this file MUST NOT define:
 *
 *     asyncReturnClause
 *     asyncReturnType
 *     futureReturnClause
 *
 * An async function may use the canonical:
 *
 *     functionReturnClause
 *
 * because async-ness is a function semantic property, not a different source
 * type-syntax authority.
 *
 * Examples:
 *
 *     async fn compute() -> Result { ... }
 *
 *     async fn measure(q: Qubit) -> Measurement { ... }
 *
 *     async fn transform<T>(value: T) -> Result<T, Error> { ... }
 *
 * The semantic/type system determines the legality of those return types.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic parameter syntax remains owned by:
 *
 *     grammar/functions/generics.g4
 *
 * This file does not redefine:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterBound
 *
 * Async functions therefore automatically inherit the same scalable generic
 * syntax as ordinary functions.
 *
 * ============================================================================
 * PARAMETER INTEGRATION
 * ============================================================================
 *
 * Parameter syntax remains owned by:
 *
 *     grammar/functions/parameters.g4
 *
 * This file does not redefine:
 *
 *     parameterList
 *     parameter
 *
 * Async functions therefore automatically inherit the canonical parameter
 * system.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * This file does not define:
 *
 *     Future<T>
 *     Task<T>
 *     Promise<T>
 *     Stream<T>
 *     Async<T>
 *
 * as grammar-level special cases.
 *
 * If such constructs are valid Zamani types, they are ordinary canonical
 * type expressions and belong to:
 *
 *     grammar/types/
 *
 * Their semantic interpretation belongs to the type/semantic system.
 *
 * This permits future asynchronous abstractions without repeatedly changing
 * the async grammar.
 *
 * ============================================================================
 * AWAIT OWNERSHIP
 * ============================================================================
 *
 * This file owns the source-level `await` operator.
 *
 * Canonical form:
 *
 *     await expression
 *
 * The operand is a canonical postfix expression.
 *
 * This intentionally gives `await` unary-expression-like behavior without
 * making this file the owner of the complete expression hierarchy.
 *
 * Example:
 *
 *     await task
 *
 *     await compute()
 *
 *     await service.request(value)
 *
 *     await stream.next()
 *
 *     await accelerator.run(kernel)
 *
 *     await quantum_operation()
 *
 * The semantic layer determines whether the operand is actually awaitable.
 *
 * ============================================================================
 * AWAIT PRECEDENCE
 * ============================================================================
 *
 * `await` binds to one canonical postfix expression.
 *
 * Therefore:
 *
 *     await compute()
 *
 * is parsed as:
 *
 *     await (compute())
 *
 * while:
 *
 *     await compute() + value
 *
 * is parsed at the surrounding expression hierarchy as:
 *
 *     (await (compute())) + value
 *
 * This keeps `await` from accidentally consuming an entire binary expression.
 *
 * If the language specification later changes await precedence, that change
 * belongs to the canonical expression-precedence contract rather than being
 * hidden inside this file.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `awaitExpression` is intended to be admitted by the canonical expression
 * hierarchy.
 *
 * The expression composition layer should provide an explicit async-aware
 * prefix/operand boundary rather than duplicating the await syntax.
 *
 * Recommended composition:
 *
 *     prefixExpression
 *         : ...
 *         | awaitExpression
 *         | ...
 *         ;
 *
 * The expression grammar remains the sole owner of:
 *
 *     expression
 *
 * and of overall precedence.
 *
 * This file only supplies:
 *
 *     awaitExpression
 *
 * ============================================================================
 * POSTFIX DEPENDENCY
 * ============================================================================
 *
 * `awaitExpression` consumes:
 *
 *     postfixExpression
 *
 * from:
 *
 *     grammar/expressions/postfix.g4
 *
 * This is intentional because postfix expressions already represent:
 *
 *     identifiers
 *     calls
 *     indexing
 *     member access
 *     chained access
 *     other canonical postfix operations
 *
 * This avoids importing the complete `Expressions` grammar here and therefore
 * avoids creating a parser-composition cycle:
 *
 *     Expressions
 *         -> AsyncFunctions
 *         -> Expressions
 *
 * Instead the dependency is:
 *
 *     Expressions
 *         -> AsyncFunctions
 *         -> Postfix
 *
 * while the canonical expression hierarchy remains owned by Expressions.
 *
 * ============================================================================
 * ASYNC EXPRESSION SCOPE
 * ============================================================================
 *
 * This file intentionally does NOT introduce:
 *
 *     asyncBlock
 *     asyncClosure
 *     asyncLambda
 *     spawnExpression
 *     taskExpression
 *     futureExpression
 *
 * merely because those concepts may exist in a runtime.
 *
 * They require independent language contracts before becoming syntax.
 *
 * Existing runtime concepts MUST NOT automatically become source-language
 * syntax.
 *
 * If an async closure or async block is formally added to the Zamani language,
 * it should receive its own complete syntax/AST/semantic/IR contract and then
 * be composed into the appropriate expression grammar.
 *
 * ============================================================================
 * ASYNC FUNCTION SEMANTICS
 * ============================================================================
 *
 * The grammar only records:
 *
 *     async
 *
 * on the function declaration.
 *
 * Semantic analysis determines:
 *
 *     - whether the function is suspendable;
 *     - what its callable type is;
 *     - what its completion semantics are;
 *     - whether suspension points are legal;
 *     - whether borrowed values may cross suspension points;
 *     - whether ownership survives suspension;
 *     - what effects it has;
 *     - what capabilities it requires;
 *     - what resources it requires;
 *     - whether cancellation is supported;
 *     - whether cancellation is cooperative or otherwise;
 *     - whether execution may be distributed;
 *     - whether execution may be accelerated;
 *     - whether execution interacts with quantum computation;
 *     - whether execution interacts with HDL/hardware abstractions.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * AWAIT SEMANTICS
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the operand is awaitable;
 *     - what value the await produces;
 *     - what effects the await introduces;
 *     - whether suspension is permitted in the surrounding context;
 *     - ownership/lifetime implications;
 *     - cancellation implications;
 *     - resource implications;
 *     - error propagation;
 *     - control-flow implications.
 *
 * The grammar must never attempt to determine these properties.
 *
 * ============================================================================
 * CANCELLATION INTEGRATION
 * ============================================================================
 *
 * Cancellation is intentionally NOT represented as a grammar-level runtime
 * implementation.
 *
 * The semantic/runtime architecture may provide cancellation through:
 *
 *     execution policy
 *     effect system
 *     capability system
 *     runtime task model
 *     resilience system
 *     cancellation token model
 *
 * This file must not introduce:
 *
 *     thread_cancel
 *     executor_cancel
 *     worker_cancel
 *     queue_cancel
 *
 * or any target-specific cancellation syntax.
 *
 * If the language eventually defines a source-level cancellation construct,
 * it must be specified independently and then composed here only as a
 * syntactic feature.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * This file does not own the complete concurrency model.
 *
 * Concurrency syntax belongs to:
 *
 *     grammar/concurrency/
 *
 * In particular, the concurrency layer may define:
 *
 *     tasks
 *     channels
 *     synchronization
 *     parallelism
 *     actors
 *     distributed execution
 *
 * Async functions may use those constructs through the ordinary statement,
 * expression, type, effect, and capability systems.
 *
 * `async.g4` must not redefine those constructs.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Async syntax is target independent.
 *
 * It MUST NOT contain:
 *
 *     requires_1_thread
 *     requires_8_threads
 *     requires_16_cores
 *     requires_gpu_0
 *     requires_qpu_1
 *     requires_32_qubits
 *
 * or equivalent fixed target assumptions.
 *
 * Portable requirements belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * A semantic/resource system may determine at compile or execution time how
 * the async computation is realized using available resources.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Async syntax describes computation and suspension intent.
 *
 * It does NOT prescribe:
 *
 *     - thread count;
 *     - worker count;
 *     - core count;
 *     - CPU identity;
 *     - GPU identity;
 *     - FPGA identity;
 *     - QPU identity;
 *     - device identity;
 *     - node count;
 *     - queue implementation;
 *     - scheduler implementation;
 *     - executor implementation;
 *     - event-loop implementation;
 *     - network transport;
 *     - physical qubit mapping;
 *     - hardware topology.
 *
 * Therefore an async program may be lowered to different execution strategies
 * without changing its source semantics.
 *
 * Examples include:
 *
 *     cooperative execution
 *     event-driven execution
 *     state-machine execution
 *     thread-based execution
 *     process-based execution
 *     distributed execution
 *     accelerator execution
 *     heterogeneous execution
 *     quantum/classical orchestration
 *     future execution models
 *
 * The selection is downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Async syntax remains domain neutral.
 *
 * An async function may contain operations whose semantic meaning belongs to
 * the quantum domain.
 *
 * Example:
 *
 *     async fn execute(q: Qubit) -> Measurement {
 *         let result = await measure(q);
 *         result
 *     }
 *
 * The grammar does not know whether `measure` is:
 *
 *     classical
 *     quantum
 *     hybrid
 *     remote
 *     simulated
 *     hardware-backed
 *
 * Semantic analysis resolves that meaning.
 *
 * If the computation becomes quantum semantic IR, the lowering path remains:
 *
 *     frontend AST
 *         ->
 *     semantic quantum operation
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC / resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *
 * This file MUST NOT create:
 *
 *     AsyncQuantumIR
 *     AsyncQubitIR
 *     AsyncGateIR
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Async functions may invoke or await operations associated with:
 *
 *     HDL simulation
 *     hardware control
 *     accelerator execution
 *     device communication
 *     data movement
 *     distributed services
 *
 * None of those targets are encoded in this grammar.
 *
 * Source-level async semantics remain unchanged.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An awaitable computation may semantically represent:
 *
 *     local computation
 *     remote computation
 *     distributed service
 *     actor interaction
 *     network request
 *     accelerator completion
 *     quantum-device completion
 *
 * The grammar does not distinguish these cases.
 *
 * The distinction belongs to types, effects, capabilities, resources, semantic
 * analysis, and runtime/target realization.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must preserve source locations for:
 *
 *     asyncModifier
 *     awaitExpression
 *     await operand
 *
 * Examples of syntactic errors include:
 *
 *     async
 *
 *     await
 *
 *     await ;
 *
 *     async fn
 *
 * The parser should report the missing syntactic construct at the nearest
 * meaningful source position.
 *
 * Semantic errors such as:
 *
 *     await nonAwaitableValue
 *
 * are NOT parser errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing these constructs MUST depend only on:
 *
 *     - token sequence;
 *     - active Zamani grammar/language version.
 *
 * It MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware discovery;
 *     - runtime scheduler state;
 *     - device availability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing async syntax is non-executing.
 *
 * The parser MUST NOT:
 *
 *     - create tasks;
 *     - start threads;
 *     - contact executors;
 *     - access networks;
 *     - inspect devices;
 *     - invoke QPUs;
 *     - invoke HDL simulators;
 *     - allocate runtime resources;
 *     - execute awaited expressions.
 *
 * `await` is syntax until semantic lowering/runtime execution occurs.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file introduces no artificial finite limit for:
 *
 *     async functions
 *     await expressions
 *     nesting
 *     call chains
 *     expression chains
 *     functions
 *     parameters
 *     generic parameters
 *     tasks
 *     resources
 *     devices
 *     nodes
 *     qubits
 *     threads
 *     cores
 *     accelerators
 *
 * Grammar repetition and canonical expression composition remain open-ended.
 *
 * Practical implementation limits are external resource-policy concerns and
 * must never become source-language semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Existing frontend AST support already includes an await expression:
 *
 *     Expression::Await(Span, Box<Expression>)
 *
 * Therefore:
 *
 *     awaitExpression
 *
 * MUST lower to that existing domain-neutral AST representation.
 *
 * No new async-specific AST hierarchy is introduced by this grammar.
 *
 * For the function modifier, the frontend AST must preserve that the function
 * declaration is asynchronous.
 *
 * The exact Rust representation is owned by:
 *
 *     src/frontend/ast/
 *
 * and/or the canonical function AST layer.
 *
 * This grammar must not define a second function AST.
 *
 * ============================================================================
 * SEMANTIC / IR CONTRACT
 * ============================================================================
 *
 * The lowering direction is:
 *
 *     async syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic async/function model
 *         ->
 *     canonical semantic representation
 *         ->
 *     appropriate domain IR
 *
 * Potential downstream representations include:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed representation
 *     accelerator representation
 *
 * Async syntax itself is not an IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consuming this feature may include:
 *
 *     structural validation
 *     name resolution
 *     type checking
 *     effect checking
 *     capability checking
 *     ownership/lifetime analysis
 *     suspension analysis
 *     async lowering
 *     control-flow lowering
 *     resource analysis
 *     optimization
 *     scheduling
 *     target lowering
 *
 * The grammar must not depend on the existence of any particular compiler
 * implementation.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime implementation may choose among:
 *
 *     state machines
 *     cooperative scheduling
 *     event loops
 *     worker execution
 *     distributed execution
 *     heterogeneous execution
 *     accelerator execution
 *     other future mechanisms
 *
 * No such mechanism is part of this grammar.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * The Rust implementation consuming it must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository's safe-Rust policy.
 *
 * The compiler implementation should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * at appropriate crate boundaries.
 *
 * This grammar itself contains no Rust implementation code and therefore cannot
 * introduce unsafe operations.
 *
 * ============================================================================
 * FEATURE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] async-specific syntax has one owner;
 * [x] async function declarations reuse functions.g4;
 * [x] return syntax reuses returns.g4;
 * [x] generic syntax reuses generics.g4;
 * [x] parameter syntax reuses parameters.g4;
 * [x] type syntax remains in types/;
 * [x] expression precedence remains in expressions/;
 * [x] await syntax has one owner;
 * [x] await lowers to the existing Await AST representation;
 * [x] no runtime executor is encoded;
 * [x] no scheduler is encoded;
 * [x] no thread count is encoded;
 * [x] no hardware identity is encoded;
 * [x] no machine-size limit is encoded;
 * [x] quantum lowering remains downstream;
 * [x] quantum::ir remains canonical;
 * [x] diagnostics preserve source locations;
 * [x] deterministic parsing is preserved;
 * [x] safe Rust remains the implementation requirement;
 * [x] positive tests exist;
 * [x] negative tests exist;
 * [x] boundary tests exist;
 * [x] scalability tests exist;
 * [x] compatibility tests exist.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar AsyncFunctions;

options {
    tokenVocab = ZamaniLexer;
}

import Postfix;


/* ============================================================================
 * 1. ASYNC FUNCTION MODIFIER
 * ========================================================================== */

/*
 * Canonical source-level modifier:
 *
 *     async
 *
 * This rule owns the parser-level occurrence of ASYNC.
 *
 * The lexical spelling remains owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * The token is assembled by the canonical lexer.
 */
asyncModifier
    : ASYNC
    ;


/* ============================================================================
 * 2. AWAIT EXPRESSION
 * ========================================================================== */

/*
 * Canonical await form:
 *
 *     await postfixExpression
 *
 * Examples:
 *
 *     await value
 *     await task()
 *     await service.request(value)
 *     await stream.next()
 *
 * The operand is intentionally a postfix expression rather than the complete
 * expression rule.
 *
 * This gives await unary-expression-like binding while avoiding an expression
 * grammar import cycle.
 */
awaitExpression
    : AWAIT
      postfixExpression
    ;


/* ============================================================================
 * 3. ASYNC EXPRESSION
 * ========================================================================== */

/*
 * Stable named integration boundary for expression-composition grammars.
 *
 * The expression hierarchy may consume:
 *
 *     asyncExpression
 *
 * rather than depending directly on the internal await rule.
 *
 * This creates a future extension point without introducing a second
 * expression hierarchy.
 */
asyncExpression
    : awaitExpression
    ;