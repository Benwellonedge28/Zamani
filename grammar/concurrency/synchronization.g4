/*
 * ============================================================================
 * Zamani Programming Language
 * Production Synchronization Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/synchronization.g4
 *
 * Grammar:
 *     Synchronization
 *
 * Status:
 *     PRODUCTION-READY MODULAR CONCURRENCY COMPONENT
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for synchronization intent.
 *
 * Synchronization is a semantic property of computation, not a particular
 * implementation mechanism.
 *
 * The same source construct may ultimately be implemented using:
 *
 *     local synchronization
 *     shared-memory synchronization
 *     distributed synchronization
 *     accelerator synchronization
 *     heterogeneous synchronization
 *     quantum/classical coordination
 *     hardware synchronization
 *     future synchronization mechanisms
 *
 * without changing the source grammar merely because the target changes.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Synchronization parser rules
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> synchronization semantics
 *          +--> ownership / borrowing
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> memory model
 *          |
 *          v
 *     canonical semantic representation / IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> scheduling
 *          +--> placement
 *          +--> distributed realization
 *          +--> runtime realization
 *          +--> hardware realization
 *          |
 *          v
 *     target
 *
 * This grammar creates NO synchronization IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     synchronization declarations
 *     synchronization scopes
 *     synchronized blocks
 *     acquire operations
 *     release operations
 *     wait operations
 *     notify operations
 *     barrier declarations
 *     barrier arrival/wait operations
 *     atomic operations
 *     atomic memory-order syntax
 *     synchronization policies
 *     synchronization requirements
 *     synchronization hints
 *     synchronization assertions
 *     synchronization-set syntax
 *     synchronization expressions
 *     synchronization composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     ordinary expressions
 *     expression precedence
 *     ordinary types
 *     blocks
 *     general statements
 *     tasks
 *     futures
 *     channels
 *     actors
 *     memory ownership
 *     borrowing
 *     resource discovery
 *     resource allocation
 *     scheduling
 *     placement
 *     routing
 *     distributed deployment
 *     hardware topology
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     QPU selection
 *     quantum::ir
 *     QEC
 *     ZQN
 *     calibration
 *     HAL
 *     runtime algorithms
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Synchronization syntax imposes NO language-level finite limits on:
 *
 *     synchronization objects
 *     locks
 *     barriers
 *     atomic objects
 *     participants
 *     waiters
 *     synchronization sets
 *     nested synchronization scopes
 *     synchronization operations
 *     parallel regions
 *     processes
 *     tasks
 *     threads
 *     nodes
 *
 * There is deliberately no:
 *
 *     MAX_LOCKS
 *     MAX_BARRIERS
 *     MAX_ATOMICS
 *     MAX_PARTICIPANTS
 *     MAX_WAITERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Repetition is structural.
 *
 * Practical parser/compiler/runtime limits are implementation-resource
 * constraints and MUST NOT become language-level synchronization limits.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / IMPLEMENTATION SEPARATION
 * ============================================================================
 *
 * Synchronization syntax may express:
 *
 *     requirements
 *     constraints
 *     policies
 *     preferences
 *     hints
 *
 * It MUST NOT silently encode implementation decisions.
 *
 * For example:
 *
 *     requires synchronization(...)
 *
 * is portable source intent.
 *
 * It is NOT equivalent to:
 *
 *     use thread 7
 *     use core 3
 *     use CPU 0
 *     use node 2
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Structural punctuation comes from the canonical punctuation vocabulary:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *
 * Existing keyword tokens are reused whenever they already exist.
 *
 * New synchronization vocabulary belongs in:
 *
 *     grammar/lexer/keywords.g4
 *
 * It MUST NOT be defined inside this parser grammar.
 *
 * ============================================================================
 * PARSER DEPENDENCIES
 * ============================================================================
 *
 * This grammar consumes canonical syntax supplied by:
 *
 *     Expressions
 *     Types
 *     Calls
 *     ZamaniExpressionBlocks
 *
 * These components remain authoritative for:
 *
 *     expression
 *     typeExpression
 *     argumentList
 *     blockExpression
 *
 * Synchronization.g4 therefore MUST NOT redefine those rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each synchronization construct maps to the existing domain-neutral frontend
 * AST model.
 *
 * The AST should preserve at minimum:
 *
 *     construct kind
 *     operands/references
 *     options
 *     expressions
 *     memory-order intent
 *     policy names
 *     source spans
 *     child ordering
 *
 * This grammar does NOT define a Rust AST.
 *
 * The Rust implementation remains owned by:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     synchronization-object resolution
 *     type checking
 *     lvalue validation for atomic targets
 *     atomicity validation
 *     memory-order legality
 *     ownership
 *     borrowing
 *     lifetime
 *     effect checking
 *     capability checking
 *     resource requirements
 *     deadlock analysis where supported
 *     ordering guarantees
 *     fairness semantics
 *     cancellation semantics
 *     timeout semantics
 *     distributed consistency
 *     implementation feasibility
 *
 * The parser only establishes syntactic structure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO synchronization-specific IR.
 *
 * Synchronization constructs lower through the existing semantic/IR
 * architecture.
 *
 * A construct may eventually lower to:
 *
 *     classical synchronization representation
 *     distributed synchronization representation
 *     accelerator coordination
 *     hardware synchronization intent
 *     runtime synchronization
 *     future synchronization representation
 *
 * The grammar does not select among them.
 *
 * ============================================================================
 * QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Synchronization may coordinate classical and quantum computation.
 *
 * Example semantic flow:
 *
 *     classical computation
 *          |
 *          v
 *     synchronization
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical computation
 *
 * Synchronization.g4 does NOT create quantum IR.
 *
 * Quantum constructs continue through:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Synchronization may be realized locally or across a distributed system.
 *
 * The grammar does not assume:
 *
 *     one process
 *     one machine
 *     one node
 *     one network
 *     one memory domain
 *
 * It does not encode:
 *
 *     node identifiers
 *     physical addresses
 *     network addresses
 *     fixed topology
 *
 * Distributed realization belongs downstream.
 *
 * ============================================================================
 * MEMORY-MODEL INTEGRATION
 * ============================================================================
 *
 * Memory-order syntax expresses language-level ordering intent.
 *
 * It does NOT identify:
 *
 *     cache architecture
 *     CPU instruction
 *     cache line width
 *     register width
 *     coherence protocol
 *     processor model
 *
 * Those are target semantics.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no executable code
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime execution
 *     no unsafe Rust
 *
 * The consuming compiler implementation must remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SYNCHRONIZATION DECLARATIONS
 * ============================================================================
 */

/*
 * Generic synchronization object declaration.
 *
 * Examples:
 *
 *     synchronization lock;
 *     synchronization gate;
 *     synchronization coordinator;
 *
 * The semantic layer determines the synchronization object's actual kind.
 */
synchronizationDeclaration
    : SYNCHRONIZATION synchronizationName synchronizationType?
      synchronizationOptions? SEMICOLON
    ;


/*
 * Explicit synchronization type annotation.
 *
 * The type expression is owned by the canonical type grammar.
 */
synchronizationType
    : COLON typeExpression
    ;


/*
 * Synchronization options are an ordered source-level collection.
 *
 * No finite option count is imposed.
 */
synchronizationOptions
    : synchronizationOption+
    ;


synchronizationOption
    : synchronizationPolicy
    | synchronizationScopeOption
    | synchronizationOrdering
    | synchronizationMemoryOrder
    | synchronizationRequirement
    | synchronizationHint
    ;


/*
 * ============================================================================
 * NAMES / REFERENCES
 * ============================================================================
 */

synchronizationName
    : identifier
    ;


synchronizationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION SCOPES
 * ============================================================================
 */

/*
 * Associates a block with synchronization intent.
 *
 * Examples:
 *
 *     synchronize lock {
 *         work();
 *     }
 *
 *     synchronize resource with guard {
 *         work();
 *     }
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


synchronizationScopeOption
    : SCOPE qualifiedName
    ;


/*
 * Structured synchronized block.
 *
 * The semantic layer determines acquisition/release behavior.
 */
synchronizedBlock
    : SYNCHRONIZED synchronizationReference block
    ;


/*
 * Explicit structured acquisition.
 */
withSynchronizationStatement
    : WITH synchronizationReference DO block
    ;


/*
 * ============================================================================
 * ACQUIRE / RELEASE
 * ============================================================================
 */

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


acquireSetStatement
    : ACQUIRE synchronizationSet acquireOptions? SEMICOLON
    ;


releaseSetStatement
    : RELEASE synchronizationSet SEMICOLON
    ;


/*
 * ============================================================================
 * BLOCKING / NON-BLOCKING INTENT
 * ============================================================================
 */

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


/*
 * ============================================================================
 * WAIT / NOTIFY
 * ============================================================================
 */

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
 * WAIT and NOTIFY describe synchronization intent.
 *
 * They do not prescribe:
 *
 *     condition variables
 *     futexes
 *     kernel waits
 *     spinning
 *     polling
 *     OS primitives
 *     processor instructions
 */


/*
 * ============================================================================
 * BARRIERS
 * ============================================================================
 */

/*
 * A barrier is a logical synchronization point.
 *
 * Participant membership may be determined dynamically.
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
 * ============================================================================
 * ATOMIC OPERATIONS
 * ============================================================================
 */

/*
 * Atomicity is a semantic requirement.
 *
 * The grammar does not require:
 *
 *     lock-free execution
 *     a native processor instruction
 *     a specific register width
 *     a specific cache architecture
 *     a particular memory model implementation
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
    : COMPARE_EXCHANGE
      atomicTarget
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


atomicTarget
    : expression
    ;


/*
 * Semantic analysis must establish that an atomic target is writable and
 * otherwise satisfies the language's atomic-access rules.
 */


/*
 * ============================================================================
 * MEMORY ORDER
 * ============================================================================
 */

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
 * Memory-order names describe language-level synchronization guarantees.
 *
 * They do not name processor instructions.
 */


/*
 * ============================================================================
 * POLICIES
 * ============================================================================
 */

/*
 * Policy names remain extensible.
 *
 * This avoids turning every future synchronization strategy into a new
 * language keyword.
 */
synchronizationPolicy
    : POLICY qualifiedName policyArguments?
    ;


policyArguments
    : LPAREN argumentList? RPAREN
    ;


synchronizationOrdering
    : ORDER qualifiedName
    ;


/*
 * ============================================================================
 * REQUIREMENTS / HINTS
 * ============================================================================
 */

/*
 * Reuse the existing `REQUIRES` keyword rather than introducing a competing
 * `REQUIRE` token.
 */
synchronizationRequirement
    : REQUIRES qualifiedName requirementArguments?
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
 * Neither performs hardware selection.
 */


/*
 * ============================================================================
 * SYNCHRONIZATION EXPRESSIONS
 * ============================================================================
 */

/*
 * Synchronization expressions are expressions capable of producing a
 * synchronization-related semantic value.
 *
 * This rule is intentionally reachable from synchronizationConstruct.
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
    | COMPARE_EXCHANGE
      atomicTarget
      EXPECT expression
      REPLACE expression
      atomicMemoryOrder?
    ;


synchronizationStateExpression
    : SYNCHRONIZATION_STATE synchronizationReference
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION ASSERTIONS
 * ============================================================================
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


/*
 * ============================================================================
 * SYNCHRONIZATION SETS
 * ============================================================================
 */

/*
 * An unbounded syntactic set of synchronization references.
 *
 * Example:
 *
 *     [lock_a, lock_b, barrier_c]
 *
 * The number of entries is not a machine limit.
 */
synchronizationSet
    : LBRACKET synchronizationReferenceList? RBRACKET
    ;


synchronizationReferenceList
    : synchronizationReference
      (COMMA synchronizationReference)*
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION STATEMENTS
 * ============================================================================
 */

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
    | withSynchronizationStatement
    ;


/*
 * ============================================================================
 * PUBLIC SYNCHRONIZATION CONSTRUCT
 * ============================================================================
 *
 * This is the integration boundary consumed by concurrency.g4.
 *
 * All synchronization-specific syntax enters through this rule.
 *
 * ============================================================================
 */

synchronizationConstruct
    : synchronizationDeclaration
    | barrierDeclaration
    | synchronizationScope
    | synchronizedBlock
    | synchronizationStatement
    | synchronizationExpression
    ;