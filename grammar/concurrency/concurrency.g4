/*
 * ============================================================================
 * Zamani Programming Language
 * Production Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/concurrency.g4
 *
 * Grammar role:
 *     Reusable ANTLR4 parser-domain grammar for concurrency syntax.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     The Zamani compiler is implemented in safe Rust.
 *     This grammar contains no target-language actions and requires no unsafe.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * This file owns:
 *
 *     - concurrency-domain syntax;
 *     - async function syntax;
 *     - await syntax;
 *     - spawn syntax;
 *     - explicit parallel-intent syntax;
 *     - structured concurrency syntax;
 *     - concurrency scopes;
 *     - concurrency extension syntax;
 *     - concurrency declarations where the language specification explicitly
 *       permits them.
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - patterns;
 *     - functions generally;
 *     - blocks generally;
 *     - statements generally;
 *     - effects;
 *     - memory;
 *     - resources;
 *     - scheduling;
 *     - routing;
 *     - hardware;
 *     - quantum IR;
 *     - classical IR;
 *     - runtime implementation;
 *     - executor implementation;
 *     - operating-system threads;
 *     - worker pools;
 *     - CPU topology;
 *     - GPU topology;
 *     - QPU topology;
 *     - machine sizes;
 *     - device identifiers;
 *     - backend selection.
 *
 * Those concepts belong to their respective canonical domains.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Concurrency syntax expresses COMPUTATIONAL INTENT.
 *
 * It must not encode a particular realization.
 *
 * Therefore this grammar never imposes:
 *
 *     MAX_THREADS
 *     MAX_TASKS
 *     MAX_ACTORS
 *     MAX_CHANNELS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_NODES
 *     MAX_PARALLELISM
 *     MAX_DEVICES
 *
 * Nor does it encode:
 *
 *     thread == core
 *     task == thread
 *     actor == process
 *     channel == queue
 *     node == machine
 *
 * Such mappings are target/runtime decisions.
 *
 * A concurrency construct may consequently be lowered to:
 *
 *     - one execution context;
 *     - many CPU cores;
 *     - GPU execution;
 *     - accelerator execution;
 *     - distributed execution;
 *     - heterogeneous execution;
 *     - quantum/classical orchestration;
 *     - a future computational substrate.
 *
 * ============================================================================
 * COMPILATION PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *        Core                Concurrency
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *                Frontend AST
 *                     |
 *                     v
 *       name/type/effect/resource analysis
 *                     |
 *                     v
 *             canonical semantic IR
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical    quantum    distributed
 *                     |
 *                     v
 *       optimization / scheduling /
 *       resilience / target lowering
 *                     |
 *                     v
 *                  runtime
 *
 * Concurrency.g4 therefore never lowers directly to threads, tasks, hardware,
 * quantum operations, or runtime APIs.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * The composed parser supplies the following canonical rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     pattern
 *     parameterList
 *     parameter
 *     genericParameters
 *     block
 *     blockExpression
 *     functionSignature
 *     returnType
 *     whereClause
 *     attribute
 *     argumentList
 *     literal
 *
 * They MUST remain owned by their canonical grammar domains.
 *
 * This file intentionally does not redefine them.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonically established concurrency tokens include:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * The lexer remains the sole owner of lexical spelling.
 *
 * This grammar does not declare lexer rules.
 *
 * New concurrency vocabulary MUST NOT be added here as lexer rules.
 *
 * ============================================================================
 * IMPORTANT LANGUAGE-POLICY RULE
 * ============================================================================
 *
 * The canonical syntax specification currently establishes:
 *
 *     async
 *     await
 *     spawn
 *
 * as concurrency constructs.
 *
 * Other implementation-specific concurrency vocabulary must not silently
 * become part of the language merely because it appears in an implementation
 * lexer.
 *
 * Future constructs therefore use explicit extension productions in this
 * grammar and must acquire:
 *
 *     1. specification;
 *     2. lexer contract if a reserved keyword is required;
 *     3. parser contract;
 *     4. AST representation;
 *     5. semantic rules;
 *     6. diagnostics;
 *     7. IR lowering;
 *     8. runtime/lowering integration;
 *     9. tests;
 *
 * before becoming canonical language features.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Syntax answers:
 *
 *     What did the programmer write?
 *
 * AST answers:
 *
 *     What structural concurrency construct was written?
 *
 * Semantic analysis answers:
 *
 *     Is the construct legal?
 *
 * Effect/resource analysis answers:
 *
 *     What capabilities, effects, synchronization, resource requirements,
 *     ordering requirements, cancellation properties, and determinism
 *     implications does it introduce?
 *
 * Canonical IR answers:
 *
 *     What target-independent computation does it represent?
 *
 * Scheduling/runtime answers:
 *
 *     How can the computation be realized with currently available resources?
 *
 * ============================================================================
 */

parser grammar Concurrency;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC DOMAIN ENTRY POINT
 * ============================================================================
 *
 * The composed Zamani parser should call `concurrencyConstruct` when entering
 * the concurrency domain.
 *
 * The root parser remains responsible for deciding where this domain may
 * appear in a complete compilation unit.
 * ============================================================================
 */

concurrencyConstruct
    : asyncFunctionDeclaration
    | awaitExpression
    | spawnExpression
    | parallelExpression
    | structuredConcurrencyScope
    | concurrencyExtension
    ;


/* ============================================================================
 * 2. ASYNCHRONOUS FUNCTION DECLARATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     async fn compute(x: T) -> R {
 *         ...
 *     }
 *
 * This rule does not determine:
 *
 *     - executor;
 *     - thread;
 *     - worker;
 *     - scheduler;
 *     - stack;
 *     - device;
 *     - placement.
 *
 * Those are semantic/runtime concerns.
 * ============================================================================
 */

asyncFunctionDeclaration
    : ASYNC
      FN
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
      block
    ;


/* ============================================================================
 * 3. ASYNC FUNCTION SIGNATURE
 * ============================================================================
 *
 * Used where a declaration needs only a signature.
 *
 * The canonical function grammar remains authoritative for ordinary functions.
 * ============================================================================
 */

asyncFunctionSignature
    : ASYNC
      FN
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/* ============================================================================
 * 4. AWAIT EXPRESSION
 * ============================================================================
 *
 * `await` consumes an ordinary Zamani expression.
 *
 * Examples:
 *
 *     await task
 *     await compute()
 *     await value
 *
 * The type/effect system determines whether the expression is awaitable.
 *
 * The grammar must not introduce a special finite task-handle type.
 * ============================================================================
 */

awaitExpression
    : AWAIT expression
    ;


/* ============================================================================
 * 5. SPAWN EXPRESSION
 * ============================================================================
 *
 * Spawn creates concurrent execution intent.
 *
 * Examples:
 *
 *     spawn compute()
 *
 *     spawn {
 *         work()
 *     }
 *
 * The result type and lifetime semantics belong to semantic analysis.
 *
 * The runtime may implement this using:
 *
 *     - a thread;
 *     - a task;
 *     - an event loop;
 *     - a coroutine;
 *     - an accelerator;
 *     - a distributed execution context;
 *     - another execution mechanism.
 *
 * The source grammar does not choose among them.
 * ============================================================================
 */

spawnExpression
    : SPAWN
      (
          blockExpression
        | expression
      )
    ;


/* ============================================================================
 * 6. PARALLEL INTENT
 * ============================================================================
 *
 * `parallel` expresses that the enclosed computation may be executed with
 * parallelism.
 *
 * It does NOT request a particular number of workers.
 *
 * Examples:
 *
 *     parallel {
 *         compute_a()
 *         compute_b()
 *     }
 *
 *     parallel expression
 *
 * Actual parallelism is determined after semantic analysis.
 * ============================================================================
 */

parallelExpression
    : PARALLEL
      (
          blockExpression
        | expression
      )
    ;


/* ============================================================================
 * 7. STRUCTURED CONCURRENCY
 * ============================================================================
 *
 * Structured concurrency is intentionally expressed through an extensible
 * semantic form rather than introducing an unapproved permanent keyword set.
 *
 * The canonical language can later reserve a dedicated keyword without
 * requiring a redesign of the underlying AST/semantic model.
 *
 * Examples of possible future forms:
 *
 *     concurrent { ... }
 *     task_scope { ... }
 *
 * Such spellings must only become canonical after their lexer/specification
 * contracts are established.
 *
 * ============================================================================
 */

structuredConcurrencyScope
    : concurrencyScopeKeyword
      blockExpression
    ;


/* ============================================================================
 * 8. CONCURRENCY SCOPE KEYWORD
 * ============================================================================
 *
 * The parser accepts an identifier here so that domain extensions can be
 * represented without making this grammar the owner of the global keyword
 * namespace.
 *
 * Semantic analysis MUST validate the identifier against the active Zamani
 * language version and concurrency capability set.
 *
 * This is intentionally not an unrestricted general-purpose statement rule.
 * ============================================================================
 */

concurrencyScopeKeyword
    : identifier
    ;


/* ============================================================================
 * 9. CONCURRENCY BLOCK
 * ============================================================================
 *
 * A concurrency block is a lexical boundary.
 *
 * It does not itself imply:
 *
 *     parallelism;
 *     synchronization;
 *     isolation;
 *     atomicity;
 *     cancellation;
 *     distribution.
 *
 * Those properties must be represented by the surrounding semantic construct.
 * ============================================================================
 */

concurrencyBlock
    : LBRACE
      concurrencyElement*
      RBRACE
    ;


concurrencyElement
    : attribute
    | concurrencyConstruct
    | statement
    ;


/* ============================================================================
 * 10. ASYNC/AWAIT COMPOSITION
 * ============================================================================
 *
 * These rules expose concurrency expressions as composable domain productions
 * without redefining the canonical expression grammar.
 * ============================================================================
 */

concurrencyExpression
    : awaitExpression
    | spawnExpression
    | parallelExpression
    ;


/* ============================================================================
 * 11. CONCURRENCY STATEMENT ADAPTER
 * ============================================================================
 *
 * The root parser may use this adapter when a concurrency operation occurs in
 * statement position.
 *
 * No separate statement AST is implied by this rule.
 * ============================================================================
 */

concurrencyStatement
    : concurrencyExpression SEMI?
    ;


/* ============================================================================
 * 12. STRUCTURED TASK EXTENSION
 * ============================================================================
 *
 * Generic future-proof task-scope syntax.
 *
 * The first identifier is a semantic concurrency construct name. Its meaning
 * is resolved by the active language/concurrency dialect.
 *
 * This prevents the grammar from hard-coding:
 *
 *     task;
 *     thread;
 *     worker;
 *     executor;
 *     scheduler.
 *
 * ============================================================================
 */

taskScopeExtension
    : concurrencyExtensionName
      blockExpression
    ;


/* ============================================================================
 * 13. CONCURRENCY EXTENSION
 * ============================================================================
 *
 * Future concurrency models may be introduced through a qualified semantic
 * operation.
 *
 * Examples:
 *
 *     concurrency::dataflow(...)
 *     concurrency::actor(...)
 *     concurrency::distributed(...)
 *     concurrency::transaction(...)
 *     concurrency::speculative(...)
 *     concurrency::heterogeneous(...)
 *
 * The grammar deliberately does not enumerate those operations.
 *
 * The semantic registry determines whether an extension is known, enabled,
 * version-compatible, and legal in the current context.
 * ============================================================================
 */

concurrencyExtension
    : concurrencyExtensionCall
    | taskScopeExtension
    ;


concurrencyExtensionCall
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


concurrencyExtensionName
    : qualifiedName
    ;


/* ============================================================================
 * 14. CONCURRENCY ATTRIBUTE
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 *
 * This adapter exists solely so semantic tooling can classify an attribute as
 * concurrency-related after parsing.
 * ============================================================================
 */

concurrencyAttribute
    : attribute
    ;


/* ============================================================================
 * 15. CONCURRENCY FUNCTION PARAMETER ADAPTER
 * ============================================================================
 *
 * Parameter syntax remains owned by the canonical function/type grammar.
 * This rule exists only as an explicit integration point for semantic tooling.
 * ============================================================================
 */

concurrencyParameterList
    : parameterList
    ;


/* ============================================================================
 * 16. CONCURRENCY RESOURCE REFERENCE
 * ============================================================================
 *
 * Resource names are semantic names.
 *
 * This rule does not identify:
 *
 *     CPU
 *     core
 *     thread
 *     GPU
 *     QPU
 *     node
 *     device
 *
 * as mandatory physical resources.
 *
 * Resource semantics are supplied by the resource/capability system.
 * ============================================================================
 */

concurrencyResourceReference
    : qualifiedName
    ;


/* ============================================================================
 * 17. CONCURRENCY POLICY REFERENCE
 * ============================================================================
 *
 * Policy identity is represented by a qualified name.
 *
 * Policy interpretation belongs to semantic/resource/runtime layers.
 * ============================================================================
 */

concurrencyPolicyReference
    : qualifiedName
    ;


/* ============================================================================
 * 18. CONCURRENCY CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability identity remains open-world.
 *
 * This permits future substrates without modifying the grammar for every new
 * hardware or runtime capability.
 * ============================================================================
 */

concurrencyCapabilityReference
    : qualifiedName
    ;


/* ============================================================================
 * 19. CONCURRENCY REQUIREMENT
 * ============================================================================
 *
 * Requirements describe semantic necessities.
 *
 * A requirement is NOT a concrete machine selection.
 *
 * For example, a semantic layer may interpret:
 *
 *     concurrency::asynchronous
 *
 * without requiring:
 *
 *     thread_count = N
 *     worker_count = N
 *     core_count = N
 *
 * ============================================================================
 */

concurrencyRequirement
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 20. CONCURRENCY REQUIREMENT LIST
 * ============================================================================
 */

concurrencyRequirementList
    : concurrencyRequirement
      (
          COMMA
          concurrencyRequirement
      )*
      COMMA?
    ;


/* ============================================================================
 * 21. CONCURRENCY POLICY LIST
 * ============================================================================
 */

concurrencyPolicyList
    : concurrencyPolicyReference
      (
          COMMA
          concurrencyPolicyReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 22. CONCURRENCY CAPABILITY LIST
 * ============================================================================
 */

concurrencyCapabilityList
    : concurrencyCapabilityReference
      (
          COMMA
          concurrencyCapabilityReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 23. SEMANTIC CONCURRENCY SPECIFICATION
 * ============================================================================
 *
 * This is an extensibility boundary.
 *
 * The syntax remains intentionally small.
 *
 * Semantic interpretation is delegated to:
 *
 *     capability analysis
 *     effect analysis
 *     resource analysis
 *     concurrency validation
 *     canonical IR lowering
 *
 * ============================================================================
 */

concurrencySpecification
    : concurrencyRequirementClause?
      concurrencyCapabilityClause?
      concurrencyPolicyClause?
    ;


concurrencyRequirementClause
    : REQUIRES
      concurrencyRequirementList
    ;


concurrencyCapabilityClause
    : concurrencyCapabilityKeyword
      concurrencyCapabilityList
    ;


concurrencyCapabilityKeyword
    : identifier
    ;


concurrencyPolicyClause
    : concurrencyPolicyKeyword
      concurrencyPolicyList
    ;


concurrencyPolicyKeyword
    : identifier
    ;


/* ============================================================================
 * 24. CONCURRENCY OPERATION
 * ============================================================================
 *
 * Generic concurrency operation form.
 *
 * This allows future concurrency facilities to be added without creating a
 * new grammar rule for every runtime technology.
 * ============================================================================
 */

concurrencyOperation
    : concurrencyExtensionCall
    ;


/* ============================================================================
 * 25. CONCURRENCY COMPOSITION
 * ============================================================================
 *
 * Composition remains semantic rather than hardware-defined.
 *
 * A composed concurrency operation can be lowered to a dependency graph,
 * structured task graph, distributed graph, accelerator graph, or another
 * canonical execution representation.
 * ============================================================================
 */

concurrencyComposition
    : concurrencyExpression
    ;


/* ============================================================================
 * 26. DETERMINISM MARKER
 * ============================================================================
 *
 * Determinism itself is a semantic property.
 *
 * This rule is an extension point for future deterministic-concurrency
 * annotations without requiring a new parser architecture.
 * ============================================================================
 */

concurrencyDeterminismAnnotation
    : attribute
    ;


/* ============================================================================
 * 27. CANCELLATION EXTENSION
 * ============================================================================
 *
 * Cancellation is intentionally represented as an extension operation until
 * it receives canonical lexer/specification/AST status.
 *
 * Example semantic operation:
 *
 *     concurrency::cancel(task)
 *
 * No forced cancellation model is encoded.
 * ============================================================================
 */

cancellationOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 28. JOIN EXTENSION
 * ============================================================================
 *
 * Joining is an execution semantic and must not assume that a task maps to a
 * particular runtime object.
 *
 * Example:
 *
 *     concurrency::join(task)
 *
 * ============================================================================
 */

joinOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 29. SYNCHRONIZATION EXTENSION
 * ============================================================================
 *
 * Synchronization remains semantic.
 *
 * It does not imply a particular lock, mutex, barrier, atomic instruction,
 * hardware primitive, or operating-system primitive.
 * ============================================================================
 */

synchronizationOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 30. COMMUNICATION EXTENSION
 * ============================================================================
 *
 * Communication is represented without hard-coding:
 *
 *     channel count;
 *     channel capacity;
 *     transport;
 *     node count;
 *     network topology;
 *     mailbox implementation.
 *
 * ============================================================================
 */

communicationOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 31. DISTRIBUTED CONCURRENCY EXTENSION
 * ============================================================================
 *
 * Distributed execution remains a semantic capability.
 *
 * It may later lower to:
 *
 *     local execution;
 *     remote execution;
 *     cluster execution;
 *     cloud execution;
 *     heterogeneous execution;
 *     quantum-classical distributed execution.
 *
 * ============================================================================
 */

distributedConcurrencyOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 32. HETEROGENEOUS CONCURRENCY EXTENSION
 * ============================================================================
 *
 * This provides an integration point for classical/quantum/accelerator
 * orchestration without embedding any particular machine architecture.
 * ============================================================================
 */

heterogeneousConcurrencyOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 33. QUANTUM-CLASSICAL CONCURRENCY EXTENSION
 * ============================================================================
 *
 * Concurrency may orchestrate quantum and classical work, but this grammar
 * must not create a second quantum representation.
 *
 * Quantum semantics remain owned by the quantum grammar and ultimately by
 * quantum::ir.
 * ============================================================================
 */

quantumClassicalConcurrencyOperation
    : concurrencyExtensionName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 34. CONCURRENCY DECLARATION
 * ============================================================================
 *
 * A generic declaration is provided as an extensibility point.
 *
 * Its semantic meaning is resolved by the active concurrency dialect.
 *
 * It does not create a new machine abstraction.
 * ============================================================================
 */

concurrencyDeclaration
    : concurrencyDeclarationName
      identifier
      concurrencyDeclarationParameters?
      block
    ;


concurrencyDeclarationName
    : qualifiedName
    ;


concurrencyDeclarationParameters
    : LPAREN
      parameterList?
      RPAREN
    ;


/* ============================================================================
 * 35. CONCURRENCY DIALECT EXTENSION
 * ============================================================================
 *
 * Dialects allow future concurrency models to be introduced without modifying
 * the fundamental concurrency architecture.
 *
 * Dialect registration itself belongs to the dialect/semantic layer.
 * ============================================================================
 */

concurrencyDialectExtension
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 36. SOURCE-LEVEL INTEGRATION CONTRACT
 * ============================================================================
 *
 * The composed Zamani parser MUST integrate this grammar through one canonical
 * ownership point.
 *
 * Recommended root-parser integration:
 *
 *     statement
 *         |
 *         +--> concurrencyStatement
 *
 * and:
 *
 *     expression
 *         |
 *         +--> concurrencyExpression
 *
 * Function declarations remain owned by the canonical function grammar.
 * Therefore `asyncFunctionDeclaration` should be selected by the declaration
 * dispatcher before ordinary `functionDeclaration` when ASYNC is present.
 *
 * There MUST NOT be two competing rules that independently parse:
 *
 *     async fn ...
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * Every accepted concurrency production MUST have a corresponding AST
 * representation.
 *
 * Minimum semantic AST categories:
 *
 *     AsyncFunction
 *     Await
 *     Spawn
 *     Parallel
 *     StructuredConcurrency
 *     ConcurrencyExtension
 *
 * The AST MUST preserve:
 *
 *     source span;
 *     construct kind;
 *     child expressions;
 *     block/body;
 *     generic arguments/parameters;
 *     attributes;
 *     source ordering;
 *     relevant semantic names.
 *
 * The parser MUST NOT perform:
 *
 *     type inference;
 *     resource allocation;
 *     scheduling;
 *     lowering;
 *     optimization;
 *     hardware selection.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. EFFECT INTEGRATION
 * ============================================================================
 *
 * Concurrency may introduce effects such as:
 *
 *     asynchronous execution;
 *     nondeterminism;
 *     communication;
 *     synchronization;
 *     external interaction;
 *     cancellation;
 *     distributed execution.
 *
 * Effect classification belongs to the effect/semantic layer.
 *
 * This grammar does not create an independent effect vocabulary.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis determines whether a concurrency intent can be realized.
 *
 * Examples of downstream facts:
 *
 *     available execution capacity;
 *     memory availability;
 *     communication capability;
 *     latency constraints;
 *     energy constraints;
 *     accelerator availability;
 *     distributed placement;
 *     quantum/classical execution resources.
 *
 * None of these are grammar-level limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * The scheduler consumes semantic/IR dependency information.
 *
 * This grammar does NOT determine:
 *
 *     ASAP/ALAP;
 *     list scheduling;
 *     critical path;
 *     RCPSP;
 *     resource allocation;
 *     timing;
 *     hardware routing.
 *
 * The scheduling subsystem remains authoritative for those decisions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime lowering may map:
 *
 *     spawn
 *     await
 *     parallel
 *
 * to any supported execution mechanism.
 *
 * No runtime-specific API names may be embedded in this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Concurrency may orchestrate quantum and classical computation.
 *
 * However:
 *
 *     Concurrency.g4
 *
 * MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum gates
 *     quantum circuit IR
 *     QEC algorithms
 *     noise models
 *     ZQN semantics
 *     routing
 *     pulse schedules
 *
 * Quantum source constructs belong to the quantum grammar and lower through
 * the canonical quantum semantic boundary (`quantum::ir`).
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. MEMORY INTEGRATION
 * ============================================================================
 *
 * Concurrency interacts with memory ownership, borrowing, sharing and
 * lifetimes, but does not redefine them.
 *
 * The semantic/type system determines whether a value may safely cross an
 * asynchronous or concurrent boundary.
 *
 * This is particularly important for:
 *
 *     ownership;
 *     borrowing;
 *     lifetimes;
 *     mutable access;
 *     shared access;
 *     synchronization.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed execution is not synonymous with concurrency.
 *
 * A local concurrent program may execute on one machine.
 *
 * A distributed program may execute across many machines.
 *
 * The same concurrency intent may be lowered differently according to target
 * capabilities.
 *
 * No node count is represented by this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 45. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Runtime nondeterminism is a semantic property and must never be introduced
 * accidentally by parser behavior.
 *
 * Source order and source spans must remain deterministic.
 *
 * ============================================================================
 */


/* ============================================================================
 * 46. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no finite language-level limits on:
 *
 *     tasks;
 *     concurrent scopes;
 *     async functions;
 *     spawn expressions;
 *     nesting depth;
 *     logical resources;
 *     devices;
 *     nodes;
 *     workers;
 *     cores;
 *     threads.
 *
 * Any implementation resource limit belongs outside the grammar and must be
 * represented as an explicit compiler/resource diagnostic.
 *
 * ============================================================================
 */


/* ============================================================================
 * 47. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_THREADS
 *     MAX_TASKS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_NODES
 *     MAX_CHANNELS
 *     MAX_ACTORS
 *     MAX_DEVICES
 *     DEFAULT_THREAD_COUNT
 *     DEFAULT_WORKER_COUNT
 *
 * Also forbidden:
 *
 *     CPU-specific syntax;
 *     GPU-specific syntax;
 *     QPU-specific topology;
 *     fixed device identifiers;
 *     fixed hardware addresses;
 *     fixed queue sizes;
 *     fixed stack sizes;
 *     fixed memory sizes.
 *
 * ============================================================================
 */


/* ============================================================================
 * 48. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must report:
 *
 *     source span;
 *     unexpected token;
 *     expected syntax;
 *     active grammar context.
 *
 * Semantic diagnostics must separately report:
 *
 *     unsupported concurrency capability;
 *     invalid await operand;
 *     invalid spawn context;
 *     illegal lifetime crossing;
 *     invalid effect;
 *     resource insufficiency;
 *     unsupported target realization.
 *
 * The parser must never encode target availability as syntax validity.
 *
 * ============================================================================
 */


/* ============================================================================
 * 49. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new concurrency implementation must not silently change the
 * semantics of existing:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * constructs.
 *
 * New syntax requires:
 *
 *     specification update;
 *     lexer contract if necessary;
 *     parser contract;
 *     AST contract;
 *     semantic contract;
 *     diagnostics;
 *     tests;
 *     compatibility entry.
 *
 * ============================================================================
 */


/* ============================================================================
 * 50. REQUIRED TEST MATRIX
 * ============================================================================
 *
 * Positive:
 *
 *     async fn f() {}
 *     async fn f(x: T) -> R {}
 *     await value;
 *     spawn work();
 *     spawn { work(); }
 *     parallel compute();
 *     parallel { compute(); }
 *
 * Negative:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * without required operands/bodies must fail according to parser context.
 *
 * Boundary:
 *
 *     deeply nested concurrency scopes;
 *     large parameter lists;
 *     large expressions;
 *     large numbers of spawn constructs;
 *     large numbers of parallel constructs.
 *
 * Scalability:
 *
 *     no grammar failure caused by a fixed task/thread/core/device limit.
 *
 * Cross-domain:
 *
 *     async + classical;
 *     async + quantum;
 *     spawn + quantum;
 *     parallel + classical;
 *     parallel + quantum;
 *     concurrency + distributed;
 *     concurrency + HDL;
 *     concurrency + accelerator;
 *     concurrency + effects;
 *     concurrency + memory ownership.
 *
 * Round-trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> canonical printer
 *       -> parser
 *
 * must preserve the intended concurrency semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] Every rule has one clearly defined owner.
 *     [ ] No canonical expression rule is duplicated.
 *     [ ] No canonical type rule is duplicated.
 *     [ ] No canonical identifier rule is duplicated.
 *     [ ] No canonical function rule is duplicated.
 *     [ ] No lexer rule exists in this file.
 *     [ ] async/await/spawn integrate with the canonical lexer.
 *     [ ] parallel integrates with the canonical lexer where enabled.
 *     [ ] Every accepted production has an AST destination.
 *     [ ] Every AST node has semantic analysis.
 *     [ ] Every semantic node has an IR destination.
 *     [ ] No hardware topology is encoded.
 *     [ ] No machine-size limit is encoded.
 *     [ ] No Rust unsafe code is required.
 *     [ ] No runtime implementation is embedded.
 *     [ ] No scheduler implementation is embedded.
 *     [ ] No quantum IR is duplicated.
 *     [ ] No QEC/ZQN ownership is duplicated.
 *     [ ] Memory ownership remains with the memory/type systems.
 *     [ ] Resource feasibility remains downstream.
 *     [ ] Deterministic parsing is verified.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Compatibility tests exist.
 *     [ ] POCO-REAF scalability tests exist.
 *
 * ============================================================================
 */