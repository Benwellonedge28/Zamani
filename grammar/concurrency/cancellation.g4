/*
 * ============================================================================
 * Zamani Programming Language
 * Production Cancellation Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/cancellation.g4
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust actions, predicates, embedded code,
 *     runtime implementation, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL SYNTAX for cancellation intent.
 *
 * Cancellation is a cross-cutting concurrency capability. It may apply to:
 *
 *     - tasks
 *     - futures
 *     - asynchronous computations
 *     - actors
 *     - channel operations
 *     - synchronization waits
 *     - parallel computations
 *     - distributed computations
 *     - accelerator work
 *     - quantum/classical execution
 *     - long-running computations
 *
 * This grammar records programmer intent.
 *
 * It does NOT implement cancellation.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Parser composition
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic analysis
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *     Effect analysis       Capability analysis
 *       |                         |
 *       +-------------+-----------+
 *                     |
 *                     v
 *               Canonical IR
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      scheduling   runtime   resilience
 *
 * Cancellation syntax MUST NOT directly depend on runtime implementation.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - cancellation declarations
 *     - cancellation source/handle syntax
 *     - cancellation requests
 *     - cancellation scopes
 *     - cancellation boundaries
 *     - cancellation propagation intent
 *     - cancellation policy references
 *     - cancellation observation
 *     - cancellation-aware operation intent
 *     - cancellation composition
 *     - cancellation-safe cleanup intent
 *     - cancellation-related requirements and hints
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - task creation
 *     - future implementation
 *     - actor implementation
 *     - channel implementation
 *     - synchronization implementation
 *     - scheduling
 *     - resource discovery
 *     - hardware discovery
 *     - hardware topology
 *     - CPU/core/thread counts
 *     - GPU/QPU selection
 *     - quantum cancellation implementation
 *     - QEC implementation
 *     - ZQN implementation
 *     - resilience algorithms
 *     - checkpoint serialization
 *     - runtime cancellation state
 *     - cancellation tokens as runtime objects
 *     - cancellation propagation algorithms
 *     - retry algorithms
 *     - rollback algorithms
 *     - process termination
 *     - signal handling
 *     - operating-system APIs
 *     - network transport
 *     - device APIs
 *
 * Those belong to their respective semantic/runtime subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Cancellation must not encode assumptions about machine scale.
 *
 * There are deliberately NO grammar-level limits for:
 *
 *     - number of cancellation scopes
 *     - number of cancellation sources
 *     - number of cancellable operations
 *     - number of tasks
 *     - number of futures
 *     - number of actors
 *     - number of channels
 *     - number of nodes
 *     - number of devices
 *     - number of cancellation listeners
 *     - cancellation propagation depth
 *
 * The following are therefore forbidden as grammar semantics:
 *
 *     MAX_TASKS
 *     MAX_CANCEL_TOKENS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * Runtime and target limitations are downstream concerns.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * Canonical parser rules consumed here:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     statement
 *     argumentList
 *     typeExpression
 *
 * Cancellation MUST NOT redefine any of these.
 *
 * Existing concurrency grammars remain authoritative for their domains:
 *
 *     tasks.g4
 *     futures.g4
 *     actors.g4
 *     channels.g4
 *     synchronization.g4
 *     parallel.g4
 *     data-parallel.g4
 *     task-parallel.g4
 *
 * ============================================================================
 * SEMANTIC OWNERSHIP
 * ============================================================================
 *
 * Grammar:
 *     syntax only.
 *
 * AST:
 *     structural cancellation intent.
 *
 * Semantic analysis:
 *     determines whether a cancellation construct is valid.
 *
 * Effect analysis:
 *     determines cancellation-related effects.
 *
 * Capability analysis:
 *     determines whether a target/execution context can honor the requested
 *     cancellation semantics.
 *
 * Resource analysis:
 *     determines resource implications.
 *
 * Canonical IR:
 *     represents target-independent cancellation semantics.
 *
 * Scheduling:
 *     determines when cancellation-aware work may execute.
 *
 * Runtime:
 *     implements cancellation.
 *
 * Resilience:
 *     may coordinate cancellation with retry/recovery/escalation.
 *
 * Hardware:
 *     provides target capabilities where applicable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. CANCELLATION ROOT
 * ============================================================================
 *
 * Stable public parser entry point.
 */
cancellationConstruct
    : cancellationDeclaration
    | cancellationScope
    | cancellationRequest
    | cancellationObservation
    | cancellationBoundary
    | cancellationCleanup
    | cancellationStatement
    | cancellationExpression
    ;


/*
 * ============================================================================
 * 2. CANCELLATION DECLARATION
 * ============================================================================
 *
 * Declares cancellation intent without defining the runtime representation.
 *
 * Examples of semantic forms:
 *
 *     cancellation source;
 *     cancellation source_name;
 *
 * The actual runtime object is owned downstream.
 */
cancellationDeclaration
    : CANCELLATION cancellationName cancellationDeclarationOptions? SEMICOLON
    ;


cancellationName
    : identifier
    ;


cancellationDeclarationOptions
    : cancellationDeclarationOption+
    ;


cancellationDeclarationOption
    : cancellationPolicy
    | cancellationPropagation
    | cancellationScopePolicy
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 3. CANCELLATION SOURCE REFERENCE
 * ============================================================================
 *
 * A reference identifies semantic cancellation intent.
 *
 * It does not imply a particular runtime token type.
 */
cancellationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. CANCELLATION REQUEST
 * ============================================================================
 *
 * Requests cancellation of a computation or cancellation scope.
 *
 * Runtime behavior is determined downstream.
 */
cancellationRequest
    : CANCEL cancellationTarget cancellationRequestOptions? SEMICOLON
    ;


cancellationTarget
    : cancellationReference
    | expression
    ;


cancellationRequestOptions
    : cancellationRequestOption+
    ;


cancellationRequestOption
    : cancellationReason
    | cancellationPropagation
    | cancellationPolicy
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 5. CANCELLATION REASON
 * ============================================================================
 *
 * The reason is an expression rather than a closed enumeration.
 *
 * This allows future domains to introduce structured reasons without changing
 * this grammar every time a new cancellation cause is introduced.
 */
cancellationReason
    : REASON expression
    ;


/*
 * ============================================================================
 * 6. CANCELLATION SCOPE
 * ============================================================================
 *
 * Structured cancellation scope.
 *
 * The lifetime of the scope is semantic. Runtime ownership belongs elsewhere.
 */
cancellationScope
    : CANCEL_SCOPE cancellationScopeTarget? cancellationScopeOptions? blockExpression
    ;


cancellationScopeTarget
    : cancellationReference
    ;


cancellationScopeOptions
    : cancellationScopeOption+
    ;


cancellationScopeOption
    : cancellationPolicy
    | cancellationPropagation
    | cancellationScopePolicy
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 7. CANCELLATION SCOPE POLICY
 * ============================================================================
 *
 * Controls semantic behavior of a cancellation scope.
 *
 * The policy name is extensible rather than hard-coded to a finite runtime
 * implementation.
 */
cancellationScopePolicy
    : SCOPE_POLICY qualifiedName policyArguments?
    ;


/*
 * ============================================================================
 * 8. CANCELLATION PROPAGATION
 * ============================================================================
 *
 * Describes how cancellation intent should relate to nested/associated work.
 *
 * The actual propagation algorithm belongs to semantic/runtime layers.
 */
cancellationPropagation
    : PROPAGATE cancellationPropagationMode
    ;


cancellationPropagationMode
    : cancellationPropagationDirection
    | qualifiedName
    ;


cancellationPropagationDirection
    : DOWNSTREAM
    | UPSTREAM
    | CHILDREN
    | PARENTS
    | ASSOCIATED
    | NONE
    ;


/*
 * ============================================================================
 * 9. CANCELLATION POLICY
 * ============================================================================
 *
 * Policy identifiers are extensible.
 *
 * No fixed runtime policy set is encoded into the grammar.
 */
cancellationPolicy
    : POLICY qualifiedName policyArguments?
    ;


policyArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 10. CANCELLATION OBSERVATION
 * ============================================================================
 *
 * Allows source code to observe cancellation state without defining the
 * underlying state representation.
 */
cancellationObservation
    : IS_CANCELLED cancellationReference
    ;


cancellationObservationStatement
    : cancellationObservation SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CANCELLATION CHECK
 * ============================================================================
 *
 * Explicit cooperative cancellation checkpoint.
 *
 * The runtime may implement this through a token, flag, event, message,
 * control-plane signal, or another mechanism.
 *
 * The grammar intentionally does not choose one.
 */
cancellationCheck
    : CHECK_CANCELLATION cancellationReference? SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CANCELLATION BOUNDARY
 * ============================================================================
 *
 * Establishes a semantic boundary at which cancellation behavior is
 * explicitly considered.
 *
 * This is particularly important for:
 *
 *     - long-running computations
 *     - asynchronous work
 *     - distributed work
 *     - accelerator operations
 *     - quantum execution
 *     - blocking operations
 *
 * It does not imply a specific implementation mechanism.
 */
cancellationBoundary
    : CANCELLATION_BOUNDARY cancellationBoundaryOptions? blockExpression
    ;


cancellationBoundaryOptions
    : cancellationBoundaryOption+
    ;


cancellationBoundaryOption
    : cancellationPolicy
    | cancellationPropagation
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 13. CANCELLATION-AWARE BLOCK
 * ============================================================================
 *
 * A named integration boundary for other concurrency grammars.
 */
cancellableBlock
    : CANCELLABLE blockExpression
    ;


/*
 * ============================================================================
 * 14. NON-CANCELLABLE BLOCK
 * ============================================================================
 *
 * Explicitly declares that cancellation should not be acted upon inside the
 * specified semantic region.
 *
 * This is an intent declaration, not permission to violate external runtime
 * termination requirements.
 */
nonCancellableBlock
    : IGNORE_CANCELLATION blockExpression
    ;


/*
 * ============================================================================
 * 15. CANCELLATION CLEANUP
 * ============================================================================
 *
 * Provides structured syntax for work that must be considered when
 * cancellation occurs.
 *
 * It does NOT define how cleanup executes.
 */
cancellationCleanup
    : ON_CANCEL cancellationCleanupBody
    ;


cancellationCleanupBody
    : blockExpression
    ;


/*
 * ============================================================================
 * 16. CANCELLATION HANDLER
 * ============================================================================
 *
 * A structured cancellation handler.
 *
 * Semantic analysis determines whether the handler is reachable, whether its
 * effects are legal, and whether its completion guarantees are sufficient.
 */
cancellationHandler
    : HANDLE_CANCEL blockExpression
    ;


/*
 * ============================================================================
 * 17. CANCELLATION-AWARE COMPOSITION
 * ============================================================================
 *
 * Allows a cancellation scope to explicitly associate work with a source.
 */
cancellationWithSource
    : WITH cancellationReference DO blockExpression
    ;


/*
 * ============================================================================
 * 18. CANCELLATION REQUIREMENT
 * ============================================================================
 *
 * A requirement expresses semantic necessity.
 *
 * It is NOT a machine selection.
 *
 * Examples of possible semantic meanings:
 *
 *     cancellation must be cooperative
 *     cancellation must be observable
 *     cancellation must propagate
 *
 * The actual vocabulary is extensible.
 */
cancellationRequirement
    : REQUIRE qualifiedName requirementArguments?
    ;


requirementArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 19. CANCELLATION HINT
 * ============================================================================
 *
 * Hints are non-binding implementation guidance.
 */
cancellationHint
    : HINT qualifiedName hintArguments?
    ;


hintArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 20. CANCELLATION EXPRESSION
 * ============================================================================
 *
 * Expression-valued cancellation observation.
 */
cancellationExpression
    : cancellationStateExpression
    | cancellationRequestExpression
    ;


cancellationStateExpression
    : IS_CANCELLED LPAREN cancellationReference RPAREN
    ;


cancellationRequestExpression
    : CANCEL LPAREN cancellationTarget cancellationRequestOptions? RPAREN
    ;


/*
 * ============================================================================
 * 21. CANCELLATION STATE
 * ============================================================================
 *
 * State names describe semantic state, not runtime representation.
 */
cancellationStateExpression
    : CANCELLATION_STATE cancellationReference
    ;


cancellationState
    : ACTIVE
    | REQUESTED
    | PROPAGATING
    | OBSERVED
    | COMPLETED
    | IGNORED
    | UNKNOWN
    ;


/*
 * ============================================================================
 * 22. CANCELLATION STATE ASSERTION
 * ============================================================================
 *
 * The semantic layer determines whether such an assertion is meaningful.
 */
cancellationAssertion
    : ASSERT CANCELLATION_STATE cancellationReference
      cancellationStatePredicate?
      SEMICOLON
    ;


cancellationStatePredicate
    : IS cancellationState
    ;


/*
 * ============================================================================
 * 23. COOPERATIVE CANCELLATION
 * ============================================================================
 *
 * Cooperative cancellation means the computation reaches cancellation-aware
 * boundaries/checkpoints and participates in cancellation.
 *
 * This grammar records the intent only.
 */
cooperativeCancellation
    : COOPERATIVE cancellationCooperationTarget?
    ;


cancellationCooperationTarget
    : cancellationReference
    | expression
    ;


/*
 * ============================================================================
 * 24. IMMEDIATE CANCELLATION INTENT
 * ============================================================================
 *
 * This describes requested semantic urgency.
 *
 * It does NOT guarantee that the underlying computation can be physically
 * interrupted at an arbitrary instruction boundary.
 *
 * Correctness of such behavior is determined downstream.
 */
immediateCancellation
    : IMMEDIATE CANCEL cancellationTarget cancellationRequestOptions? SEMICOLON
    ;


/*
 * ============================================================================
 * 25. DEFERRED CANCELLATION INTENT
 * ============================================================================
 *
 * Allows cancellation to be observed at the next legal semantic boundary.
 */
deferredCancellation
    : DEFERRED CANCEL cancellationTarget cancellationRequestOptions? SEMICOLON
    ;


/*
 * ============================================================================
 * 26. CANCELLATION-AWARE WAIT
 * ============================================================================
 *
 * This grammar does not redefine synchronization WAIT.
 *
 * It merely provides cancellation composition around an existing expression.
 */
cancellationAwareWait
    : WAIT_UNTIL cancellationReference expression
    ;


cancellationAwareWaitStatement
    : cancellationAwareWait SEMICOLON
    ;


/*
 * ============================================================================
 * 27. CANCELLATION-AWARE OPERATION
 * ============================================================================
 *
 * Allows an arbitrary canonical expression to be annotated with cancellation
 * intent without introducing a second expression language.
 */
cancellationAwareExpression
    : CANCELLABLE expression
    ;


cancellationAwareStatement
    : cancellationAwareExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 28. CANCELLATION PROTECTION
 * ============================================================================
 *
 * A protected region expresses that cancellation should be deferred/observed
 * according to the semantic policy attached to the region.
 */
cancellationProtection
    : PROTECT_FROM_CANCELLATION
      cancellationProtectionOptions?
      blockExpression
    ;


cancellationProtectionOptions
    : cancellationProtectionOption+
    ;


cancellationProtectionOption
    : cancellationPolicy
    | cancellationRequirement
    | cancellationHint
    ;


cancellationProtectionOption
    : cancellationPolicy
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 29. CANCELLATION FINALIZATION
 * ============================================================================
 *
 * Structured finalization associated with cancellation.
 */
cancellationFinally
    : CANCEL_FINALLY blockExpression
    ;


/*
 * ============================================================================
 * 30. CANCELLATION LIFECYCLE
 * ============================================================================
 *
 * Explicit lifecycle declaration.
 *
 * The lifecycle is semantic and target-independent.
 */
cancellationLifecycle
    : CANCELLATION_LIFECYCLE
      cancellationLifecycleStage+
    ;


cancellationLifecycleStage
    : cancellationLifecycleStageName blockExpression?
    ;


cancellationLifecycleStageName
    : REQUESTED
    | PROPAGATING
    | OBSERVED
    | COMPLETED
    ;


/*
 * ============================================================================
 * 31. CANCELLATION COMPOSITION
 * ============================================================================
 *
 * Provides a single rule for parser integration with the concurrency root.
 */
cancellationStatement
    : cancellationRequest
    | cancellationObservationStatement
    | cancellationCheck
    | cancellationCleanup
    | cancellationHandler
    | cancellationAssertion
    | cancellationAwareWaitStatement
    | cancellationAwareStatement
    | immediateCancellation
    | deferredCancellation
    | cancellationFinally
    ;


/*
 * ============================================================================
 * 32. CANCELLATION EXPRESSION ROOT
 * ============================================================================
 */
cancellationExpressionRoot
    : cancellationExpression
    | cancellationAwareExpression
    | cancellationStateExpression
    | cancellationRequestExpression
    ;


/*
 * ============================================================================
 * 33. TASK INTEGRATION
 * ============================================================================
 *
 * This rule intentionally does NOT redefine task syntax.
 *
 * `tasks.g4` owns:
 *
 *     spawn
 *     await
 *     parallel task intent
 *
 * Cancellation composes around task expressions through ordinary expressions.
 */
cancellableTask
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 34. FUTURE INTEGRATION
 * ============================================================================
 *
 * `futures.g4` owns future/async syntax.
 *
 * Cancellation only contributes cancellation intent.
 */
cancellableFuture
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 35. ACTOR INTEGRATION
 * ============================================================================
 *
 * Actor lifecycle and messaging remain owned by actors.g4.
 */
cancellableActor
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 36. CHANNEL INTEGRATION
 * ============================================================================
 *
 * Channel declaration/send/receive syntax remains owned by channels.g4.
 *
 * This production only marks an operation as cancellation-aware.
 */
cancellableChannelOperation
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 37. SYNCHRONIZATION INTEGRATION
 * ============================================================================
 *
 * synchronization.g4 already owns cancellation options associated with waits
 * and acquisition.
 *
 * This grammar must not duplicate those synchronization productions.
 */
cancellableSynchronizationOperation
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 38. PARALLEL INTEGRATION
 * ============================================================================
 *
 * Parallelism remains an execution-intent concern.
 *
 * Cancellation can apply to an entire parallel computation without specifying
 * how many physical workers execute it.
 */
cancellableParallelOperation
    : CANCELLABLE expression
    ;


/*
 * ============================================================================
 * 39. PROPAGATION SET
 * ============================================================================
 *
 * A cancellation request may be associated with an extensible set of semantic
 * targets.
 *
 * No fixed target count is permitted.
 */
cancellationTargetSet
    : LBRACKET cancellationTargetList? RBRACKET
    ;


cancellationTargetList
    : cancellationTarget
      (COMMA cancellationTarget)*
    ;


/*
 * ============================================================================
 * 40. MULTI-TARGET CANCELLATION
 * ============================================================================
 */
cancelTargets
    : CANCEL cancellationTargetSet cancellationRequestOptions? SEMICOLON
    ;


/*
 * ============================================================================
 * 41. CANCELLATION RACE / FIRST-COMPLETION POLICY
 * ============================================================================
 *
 * This is intentionally policy-based.
 *
 * It does not define a fixed executor or scheduling strategy.
 */
cancellationRacePolicy
    : RACE cancellationRaceClause+
    ;


cancellationRaceClause
    : ON_CANCEL blockExpression
    | ON_COMPLETE blockExpression
    | POLICY qualifiedName policyArguments?
    ;


/*
 * ============================================================================
 * 42. CANCELLATION DEADLINE
 * ============================================================================
 *
 * A deadline is an expression.
 *
 * It is therefore not tied to a fixed clock resolution, machine timer,
 * hardware clock, or integer width.
 */
cancellationDeadline
    : DEADLINE expression
    ;


cancellationDeadlineOption
    : cancellationDeadline
    | cancellationPolicy
    | cancellationRequirement
    | cancellationHint
    ;


/*
 * ============================================================================
 * 43. CANCELLATION TIMEOUT
 * ============================================================================
 *
 * Timeout values remain expressions.
 *
 * The canonical duration/type system determines their meaning.
 */
cancellationTimeout
    : TIMEOUT expression
    ;


/*
 * ============================================================================
 * 44. CANCELLATION REQUEST WITH DEADLINE
 * ============================================================================
 */
deadlineCancellationRequest
    : CANCEL cancellationTarget
      cancellationDeadlineOption+
      SEMICOLON
    ;


/*
 * ============================================================================
 * 45. CANCELLATION HANDLER COMPOSITION
 * ============================================================================
 */
cancellationHandlerClause
    : ON_CANCEL blockExpression
    | HANDLE_CANCEL blockExpression
    | CANCEL_FINALLY blockExpression
    ;


/*
 * ============================================================================
 * 46. CANCELLATION-SAFE REGION
 * ============================================================================
 *
 * A cancellation-safe region describes a semantic guarantee/requirement.
 *
 * It does not dictate a particular implementation.
 */
cancellationSafeRegion
    : CANCELLATION_SAFE
      cancellationSafeOptions?
      blockExpression
    ;


cancellationSafeOptions
    : cancellationSafeOption+
    ;


cancellationSafeOption
    : cancellationRequirement
    | cancellationPolicy
    | cancellationHint
    ;


cancellationSafeOption
    : cancellationRequirement
    | cancellationPolicy
    | cancellationHint
    ;


/*
 * ============================================================================
 * 47. CANCELLATION RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource management belongs to the resource/runtime layers.
 *
 * This construct only establishes that cancellation semantics must be
 * considered at the boundary.
 */
cancellationResourceBoundary
    : RESOURCE_BOUNDARY
      cancellationBoundaryOptions?
      blockExpression
    ;


/*
 * ============================================================================
 * 48. CANCELLATION DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed cancellation is represented as intent.
 *
 * Network transport and distributed consistency remain outside this grammar.
 */
distributedCancellationBoundary
    : DISTRIBUTED_BOUNDARY
      cancellationPropagation?
      blockExpression
    ;


/*
 * ============================================================================
 * 49. CANCELLATION EXECUTION POLICY
 * ============================================================================
 *
 * Execution policy remains extensible.
 */
cancellationExecutionPolicy
    : EXECUTION_POLICY qualifiedName policyArguments?
    ;


/*
 * ============================================================================
 * 50. CANCELLATION CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * A program can require cancellation support without naming a provider,
 * machine, device, operating system, or runtime implementation.
 */
cancellationCapabilityRequirement
    : REQUIRE_CANCELLATION
    ;


/*
 * ============================================================================
 * 51. CANCELLATION CAPABILITY QUERY
 * ============================================================================
 */
cancellationCapabilityExpression
    : HAS_CANCELLATION_CAPABILITY
    ;


/*
 * ============================================================================
 * 52. CANCELLATION ERROR / OUTCOME OBSERVATION
 * ============================================================================
 *
 * The actual error/result type belongs to the canonical type system.
 *
 * This rule only establishes syntactic observation of cancellation outcome.
 */
cancellationOutcomeExpression
    : CANCELLATION_OUTCOME cancellationReference
    ;


/*
 * ============================================================================
 * 53. CANCELLATION OUTCOME ASSERTION
 * ============================================================================
 */
cancellationOutcomeAssertion
    : ASSERT CANCELLATION_OUTCOME cancellationReference
      IS cancellationOutcome
      SEMICOLON
    ;


cancellationOutcome
    : CANCELLED
    | NOT_CANCELLED
    | CANCELLATION_REQUESTED
    | CANCELLATION_COMPLETED
    | CANCELLATION_REJECTED
    | CANCELLATION_UNKNOWN
    ;


/*
 * ============================================================================
 * 54. FINAL PUBLIC COMPOSITION RULE
 * ============================================================================
 *
 * Parser composition layers should expose this rule from the concurrency
 * grammar rather than importing individual implementation productions.
 */
cancellation
    : cancellationConstruct
    | cancellationStatement
    | cancellationExpressionRoot
    | cancellationSafeRegion
    | cancellationResourceBoundary
    | distributedCancellationBoundary
    | cancellationLifecycle
    | cancellationRacePolicy
    | cancellationCapabilityRequirement
    | cancellationCapabilityExpression
    ;


/*
 * ============================================================================
 * END OF cancellation.g4
 * ============================================================================
 */