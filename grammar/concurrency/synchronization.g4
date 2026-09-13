parser grammar Synchronization;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * Zamani — Concurrency Synchronization Grammar
 * ============================================================================
 *
 * File:
 *   grammar/concurrency/synchronization.g4
 *
 * Purpose:
 *   Defines source-level syntax for synchronization intent.
 *
 * Architectural boundary:
 *
 *   Source
 *      ↓
 *   ANTLR lexer/parser
 *      ↓
 *   Zamani AST
 *      ↓
 *   Semantic analysis
 *      ↓
 *   Canonical IR
 *      ↓
 *   Scheduling / execution / runtime
 *
 * This grammar MUST NOT:
 *
 *   - implement locks
 *   - implement atomics
 *   - implement scheduling
 *   - discover hardware
 *   - select CPUs/GPUs/QPUs
 *   - select thread counts
 *   - select memory sizes
 *   - define a machine topology
 *   - define fixed resource limits
 *   - define runtime algorithms
 *   - contain Rust actions
 *   - contain unsafe code
 *   - duplicate the canonical type system
 *   - duplicate the canonical expression grammar
 *   - duplicate channel semantics
 *   - duplicate task/future/actor semantics
 *
 * Synchronization syntax expresses programmer intent.
 *
 * The compiler/runtime decides how that intent is implemented on the
 * available execution resources.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *   - synchronization declarations
 *   - synchronization scopes
 *   - synchronization guards
 *   - acquire/release operations
 *   - wait/notify operations
 *   - barrier declarations/operations
 *   - atomic operation syntax
 *   - memory-order intent
 *   - synchronization policies
 *   - synchronization composition syntax
 *
 * DOES NOT OWN:
 *   - channel declaration/communication
 *   - task creation
 *   - futures/promises
 *   - actor messaging
 *   - cancellation-token definitions
 *   - memory ownership/borrowing
 *   - hardware topology
 *   - scheduling
 *   - resource discovery
 *   - runtime implementation
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO constants such as:
 *
 *   MAX_THREADS
 *   MAX_LOCKS
 *   MAX_BARRIERS
 *   MAX_PARTICIPANTS
 *   MAX_ATOMICS
 *   MAX_WAITERS
 *   MAX_RESOURCES
 *
 * Counts, participants, capacities, and resource properties are represented
 * through normal language expressions/types/requirements where appropriate.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical rules supplied by the imported grammar
 * layers:
 *
 *   identifier
 *   qualifiedName
 *   expression
 *   typeExpression
 *   block
 *   statement
 *   argumentList
 *   genericArguments
 *   attribute
 *
 * The exact delegate grammar names MUST remain the repository's canonical
 * grammar authorities. Synchronization.g4 must not recreate those rules.
 *
 * The root Zamani grammar should import/expose this grammar through the
 * concurrency grammar rather than duplicating these productions.
 *
 * ============================================================================
 */


/* --------------------------------------------------------------------------
 * Synchronization declarations
 * -------------------------------------------------------------------------- */

/*
 * Generic synchronization declaration.
 *
 * Examples of semantic forms include:
 *
 *   synchronization mutex;
 *   synchronization gate;
 *   synchronization barrier;
 *
 * The concrete synchronization kind remains extensible.
 */
synchronizationDeclaration
    : SYNCHRONIZATION synchronizationName synchronizationType?
      synchronizationOptions? SEMICOLON
    ;


/*
 * Explicitly typed synchronization object.
 *
 * The actual synchronization type is resolved by the canonical type system.
 */
synchronizationType
    : COLON typeExpression
    ;


/*
 * Synchronization options are declarative properties, not implementation
 * directives.
 */
synchronizationOptions
    : synchronizationOption+
    ;


synchronizationOption
    : synchronizationPolicy
    | synchronizationScope
    | synchronizationOrdering
    | synchronizationMemoryOrder
    | synchronizationRequirement
    | synchronizationHint
    ;


/* --------------------------------------------------------------------------
 * Synchronization names/references
 * -------------------------------------------------------------------------- */

synchronizationName
    : identifier
    ;


synchronizationReference
    : qualifiedName
    ;


/* --------------------------------------------------------------------------
 * Synchronization scopes
 * -------------------------------------------------------------------------- */

/*
 * A synchronization scope associates a block with synchronization intent.
 *
 * Runtime implementation is deliberately unspecified.
 */
synchronizationScope
    : SYNCHRONIZE synchronizationTarget? synchronizationGuard? block
    ;


synchronizationTarget
    : synchronizationReference
    ;


synchronizationGuard
    : WITH synchronizationReference
    ;


/*
 * Explicit scoped acquisition.
 *
 * This form allows semantic analysis to establish structured lifetime
 * relationships between acquisition and release.
 */
synchronizedBlock
    : SYNCHRONIZED synchronizationReference block
    ;


/* --------------------------------------------------------------------------
 * Acquire / release
 * -------------------------------------------------------------------------- */

acquireStatement
    : ACQUIRE synchronizationReference acquireOptions? SEMICOLON
    ;


acquireOptions
    : acquireOption+
    ;


acquireOption
    : blockingOption
    | timeoutOption
    | cancellationOption
    | memoryOrderOption
    ;


releaseStatement
    : RELEASE synchronizationReference SEMICOLON
    ;


/*
 * Structured acquisition.
 *
 * The compiler may lower this into an acquire/release pair or another
 * equivalent mechanism while preserving the semantic contract.
 */
withSynchronizationStatement
    : WITH synchronizationReference DO block
    ;


/* --------------------------------------------------------------------------
 * Blocking / non-blocking intent
 * -------------------------------------------------------------------------- */

blockingOption
    : BLOCKING
    | NONBLOCKING
    ;


timeoutOption
    : TIMEOUT expression
    ;


cancellationOption
    : CANCELLABLE
    | IGNORE_CANCELLATION
    ;


/* --------------------------------------------------------------------------
 * Wait / notify
 * -------------------------------------------------------------------------- */

waitStatement
    : WAIT synchronizationReference waitCondition? waitOptions? SEMICOLON
    ;


waitCondition
    : UNTIL expression
    ;


waitOptions
    : waitOption+
    ;


waitOption
    : timeoutOption
    | cancellationOption
    | blockingOption
    ;


notifyStatement
    : NOTIFY synchronizationReference notifyMode? SEMICOLON
    ;


notifyMode
    : ONE
    | ALL
    ;


/*
 * WAIT/NOTIFY express synchronization intent only.
 *
 * They do not imply:
 *
 *   - OS condition variables
 *   - futexes
 *   - spinning
 *   - kernel scheduling
 *   - a particular CPU
 *   - a particular operating system
 */


/* --------------------------------------------------------------------------
 * Barriers
 * -------------------------------------------------------------------------- */

/*
 * A barrier represents a synchronization point.
 *
 * Participant membership may be dynamic and must not be represented by a
 * grammar-level fixed integer limit.
 */
barrierDeclaration
    : BARRIER barrierName barrierOptions? SEMICOLON
    ;


barrierName
    : identifier
    ;


barrierOptions
    : barrierOption+
    ;


barrierOption
    : PARTICIPANTS expression
    | synchronizationPolicy
    | timeoutOption
    | cancellationOption
    | reusableOption
    ;


reusableOption
    : REUSABLE
    | ONE_SHOT
    ;


barrierWaitStatement
    : ARRIVE barrierReference barrierWaitOptions? SEMICOLON
    ;


barrierReference
    : qualifiedName
    ;


barrierWaitOptions
    : barrierWaitOption+
    ;


barrierWaitOption
    : WAIT
    | NO_WAIT
    | timeoutOption
    | cancellationOption
    ;


/*
 * PARTICIPANTS is an expression rather than a fixed literal restriction.
 *
 * This permits:
 *
 *   participants runtime_value
 *   participants worker_count
 *   participants configured_participants
 *
 * without embedding machine-size assumptions in the grammar.
 */


/* --------------------------------------------------------------------------
 * Atomics
 * -------------------------------------------------------------------------- */

/*
 * Atomic operations describe atomicity requirements.
 *
 * They do not require a particular CPU instruction, lock-free implementation,
 * cache architecture, word width, or hardware primitive.
 */
atomicStatement
    : ATOMIC atomicOperation
    ;


atomicOperation
    : atomicLoad
    | atomicStore
    | atomicExchange
    | atomicCompareExchange
    | atomicFetchOperation
    ;


atomicLoad
    : LOAD atomicTarget atomicMemoryOrder? SEMICOLON
    ;


atomicStore
    : STORE atomicTarget FROM expression atomicMemoryOrder? SEMICOLON
    ;


atomicExchange
    : EXCHANGE atomicTarget WITH expression atomicMemoryOrder? SEMICOLON
    ;


atomicCompareExchange
    : COMPARE_EXCHANGE atomicTarget
      EXPECT expression
      REPLACE expression
      atomicMemoryOrder?
      SEMICOLON
    ;


atomicFetchOperation
    : FETCH atomicFetchOperator atomicTarget
      WITH expression
      atomicMemoryOrder?
      SEMICOLON
    ;


atomicFetchOperator
    : ADD
    | SUBTRACT
    | AND
    | OR
    | XOR
    | MIN
    | MAX
    ;


/*
 * Atomic target deliberately uses a canonical expression/reference rather
 * than imposing a fixed scalar representation.
 */
atomicTarget
    : expression
    ;


/* --------------------------------------------------------------------------
 * Memory ordering
 * -------------------------------------------------------------------------- */

atomicMemoryOrder
    : MEMORY_ORDER memoryOrder
    ;


memoryOrderOption
    : atomicMemoryOrder
    ;


memoryOrder
    : RELAXED
    | ACQUIRE_ORDER
    | RELEASE_ORDER
    | ACQ_REL
    | SEQUENTIAL
    | CONSUME_ORDER
    ;


/*
 * Memory-order names describe the language memory model.
 *
 * They are NOT processor-specific instructions.
 */


/* --------------------------------------------------------------------------
 * Synchronization policies
 * -------------------------------------------------------------------------- */

synchronizationPolicy
    : POLICY qualifiedName policyArguments?
    ;


policyArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * Policy names are qualified names rather than a closed list.
 *
 * This gives Zamani extensibility without changing this grammar whenever
 * a future synchronization policy is introduced.
 */


/* --------------------------------------------------------------------------
 * Ordering
 * -------------------------------------------------------------------------- */

synchronizationOrdering
    : ORDER qualifiedName
    ;


/*
 * The grammar does not assume FIFO, fairness, priority ordering, or another
 * implementation property unless the semantic policy explicitly defines it.
 */


/* --------------------------------------------------------------------------
 * Requirements and hints
 * -------------------------------------------------------------------------- */

synchronizationRequirement
    : REQUIRE qualifiedName requirementArguments?
    ;


requirementArguments
    : LPAREN argumentList? RPAREN
    ;


synchronizationHint
    : HINT qualifiedName hintArguments?
    ;


hintArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * Requirements are semantic constraints.
 *
 * Hints are non-mandatory implementation guidance.
 *
 * Neither may be interpreted as an implicit hardware selection.
 */


/* --------------------------------------------------------------------------
 * Synchronization expressions
 * -------------------------------------------------------------------------- */

/*
 * Some synchronization operations naturally produce values.
 *
 * This grammar provides a dedicated syntactic category so the semantic layer
 * can distinguish synchronization operations from ordinary calls without
 * forcing a runtime representation.
 */
synchronizationExpression
    : atomicExpression
    | synchronizationStateExpression
    ;


atomicExpression
    : ATOMIC LPAREN atomicExpressionOperation RPAREN
    ;


atomicExpressionOperation
    : LOAD atomicTarget atomicMemoryOrder?
    | EXCHANGE atomicTarget WITH expression atomicMemoryOrder?
    | COMPARE_EXCHANGE atomicTarget
      EXPECT expression
      REPLACE expression
      atomicMemoryOrder?
    ;


synchronizationStateExpression
    : SYNCHRONIZATION_STATE synchronizationReference
    ;


/* --------------------------------------------------------------------------
 * Synchronization assertions
 * -------------------------------------------------------------------------- */

/*
 * These are declarative assertions about synchronization state.
 *
 * They are not runtime implementation checks unless semantic/runtime layers
 * explicitly lower them as such.
 */
synchronizationAssertion
    : ASSERT SYNCHRONIZATION_STATE synchronizationReference
      synchronizationPredicate?
      SEMICOLON
    ;


synchronizationPredicate
    : IS synchronizationState
    ;


synchronizationState
    : AVAILABLE
    | HELD
    | WAITING
    | RELEASED
    | CLOSED
    | ACTIVE
    | INACTIVE
    ;


/* --------------------------------------------------------------------------
 * Synchronization composition
 * -------------------------------------------------------------------------- */

/*
 * Multiple synchronization resources may be composed without embedding a
 * fixed number of resources.
 */
synchronizationSet
    : LBRACKET synchronizationReferenceList? RBRACKET
    ;


synchronizationReferenceList
    : synchronizationReference
      (COMMA synchronizationReference)*
    ;


acquireSetStatement
    : ACQUIRE synchronizationSet acquireOptions? SEMICOLON
    ;


releaseSetStatement
    : RELEASE synchronizationSet SEMICOLON
    ;


/*
 * Ordering/deadlock policy belongs to semantic analysis/runtime policy.
 * The grammar merely represents the set of synchronization resources.
 */


/* --------------------------------------------------------------------------
 * Synchronization operations
 * -------------------------------------------------------------------------- */

synchronizationStatement
    : acquireStatement
    | releaseStatement
    | waitStatement
    | notifyStatement
    | barrierWaitStatement
    | atomicStatement
    | acquireSetStatement
    | releaseSetStatement
    | synchronizationAssertion
    ;


/* --------------------------------------------------------------------------
 * Shared synchronization construct
 * -------------------------------------------------------------------------- */

synchronizationConstruct
    : synchronizationDeclaration
    | barrierDeclaration
    | synchronizationScope
    | synchronizedBlock
    | withSynchronizationStatement
    | synchronizationStatement
    ;


/*
 * ============================================================================
 * END OF synchronization.g4
 * ============================================================================
 */