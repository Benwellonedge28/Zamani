/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/synchronization.g4
 *
 * Grammar:
 *     ExecutionSynchronization
 *
 * Status:
 *     Production execution-synchronization intent grammar
 *
 * Purpose:
 *     Defines source-level synchronization intent for execution.
 *
 * ============================================================================
 *
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar describes WHAT execution must synchronize.
 *
 * It does NOT describe HOW synchronization is implemented.
 *
 * Source
 *   |
 *   v
 * Lexer / Parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Semantic analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> Classical IR
 *   +--> quantum::ir
 *   +--> HDL / hardware representation
 *   +--> Distributed representation
 *   |
 *   v
 * Optimization / Routing / Scheduling / Resilience
 *   |
 *   v
 * Hardware abstraction / Runtime / Deployment
 *
 * ============================================================================
 *
 * THIS FILE OWNS
 * ============================================================================
 *
 *   - execution synchronization declarations;
 *   - execution synchronization boundaries;
 *   - completion dependencies;
 *   - visibility dependencies;
 *   - ordering requirements;
 *   - execution dependency relationships;
 *   - synchronization phases;
 *   - synchronization scopes;
 *   - synchronization policies;
 *   - synchronization requirements;
 *   - synchronization constraints;
 *   - synchronization preferences;
 *   - synchronization hints;
 *   - synchronization conditions;
 *   - synchronization joins;
 *   - synchronization waits;
 *   - synchronization signals;
 *   - synchronization points;
 *   - execution-level synchronization composition.
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 *   - mutexes;
 *   - semaphores;
 *   - condition variables;
 *   - atomics;
 *   - channels;
 *   - futures;
 *   - promises;
 *   - actors;
 *   - task creation;
 *   - cancellation tokens;
 *   - memory ownership;
 *   - memory ordering primitives;
 *   - CPU instructions;
 *   - GPU instructions;
 *   - QPU instructions;
 *   - physical qubit synchronization;
 *   - pulse synchronization;
 *   - hardware clock implementation;
 *   - scheduling algorithms;
 *   - routing algorithms;
 *   - resource allocation;
 *   - hardware discovery;
 *   - runtime implementation.
 *
 * Low-level synchronization remains owned by:
 *
 *     grammar/concurrency/synchronization.g4
 *
 * Execution scheduling remains owned by:
 *
 *     grammar/execution/scheduling.g4
 *
 * Execution placement remains owned by:
 *
 *     grammar/execution/placement.g4
 *
 * Execution dispatch remains owned by:
 *
 *     grammar/execution/dispatch.g4
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Synchronization syntax MUST therefore express semantic relationships
 * rather than physical implementation assumptions.
 *
 * This grammar contains no assumptions about:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - accelerator count;
 *     - QPU count;
 *     - qubit count;
 *     - node count;
 *     - memory capacity;
 *     - topology;
 *     - queue capacity;
 *     - clock frequency;
 *     - device identifiers;
 *     - hardware addresses;
 *     - operating systems;
 *     - providers;
 *     - vendors.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no grammar-level maximums.
 *
 * No:
 *
 *     MAX_SYNCHRONIZATION_POINTS
 *     MAX_DEPENDENCIES
 *     MAX_PARTICIPANTS
 *     MAX_PHASES
 *     MAX_EXECUTIONS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_QUBITS
 *
 * Repetition is represented structurally.
 *
 * Actual resource limits belong to:
 *
 *     - semantic analysis;
 *     - resource analysis;
 *     - scheduling;
 *     - runtime policy;
 *     - target capability;
 *     - deployment configuration;
 *     - operating-system limits.
 *
 * ============================================================================
 *
 * CANONICAL COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical root grammar imports/exposes these rules.
 *
 * This file MUST NOT define:
 *
 *     grammar Zamani;
 *     lexer grammar ...;
 *     lexer rules;
 *     EOF entry points;
 *     semantic actions.
 *
 * ============================================================================
 *
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * The following concepts are expected to be supplied by canonical grammar
 * layers:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     block
 *     statement
 *     argumentList
 *
 * This grammar does not recreate those productions.
 *
 * ============================================================================
 *
 * IMPORTANT DOMAIN SEPARATION
 * ============================================================================
 *
 * A synchronization request such as:
 *
 *     after compute_a before compute_b
 *
 * means:
 *
 *     execution of compute_b has a semantic dependency on compute_a.
 *
 * It does NOT mean:
 *
 *     use a mutex;
 *     use a barrier;
 *     use a CPU fence;
 *     use a GPU fence;
 *     use a QPU barrier;
 *     use a network barrier.
 *
 * The implementation layer chooses an equivalent realization.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations remain represented by the canonical quantum frontend and
 * ultimately by quantum::ir.
 *
 * This grammar may express execution synchronization around quantum work:
 *
 *     synchronize quantum_region;
 *
 *     wait quantum_result;
 *
 *     after measurement before classical_control;
 *
 * but it MUST NOT create a quantum IR representation.
 *
 * QEC remains responsible for quantum error correction.
 *
 * ZQN remains responsible for quantum fault/noise semantics.
 *
 * Routing remains responsible for physical realization.
 *
 * Scheduling remains responsible for timing and resource ordering.
 *
 * Resilience remains responsible for recovery decisions.
 *
 * ============================================================================
 *
 * CLASSICAL / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * The synchronization target is intentionally represented by canonical
 * expressions/references rather than a closed domain-specific list.
 *
 * Therefore the same synchronization model can surround:
 *
 *     classical computation
 *     quantum computation
 *     HDL execution
 *     hardware regions
 *     distributed work
 *     accelerator work
 *     heterogeneous pipelines
 *     future execution domains
 *
 * without modifying this grammar.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * No unsafe implementation is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar ExecutionSynchronization;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. ROOT SYNCHRONIZATION CONSTRUCT
 * ============================================================================
 *
 * The root rule intentionally represents execution synchronization as a
 * semantic construct rather than a low-level primitive.
 */

executionSynchronization
    : synchronizationDeclaration
    | synchronizationPoint
    | synchronizationWait
    | synchronizationSignal
    | synchronizationJoin
    | synchronizationScope
    | synchronizationDependency
    | synchronizationPhase
    | synchronizationAssertion
    ;


/*
 * ============================================================================
 * 2. SYNCHRONIZATION DECLARATION
 * ============================================================================
 *
 * Declares a named execution synchronization object.
 *
 * The concrete runtime representation is deliberately unspecified.
 */

synchronizationDeclaration
    : SYNCHRONIZATION synchronizationName
      synchronizationDeclarationBody?
      SEMICOLON
    ;


synchronizationName
    : identifier
    ;


synchronizationDeclarationBody
    : LBRACE synchronizationClause* RBRACE
    ;


/*
 * ============================================================================
 * 3. SYNCHRONIZATION CLAUSES
 * ============================================================================
 */

synchronizationClause
    : synchronizationRequirement
    | synchronizationConstraint
    | synchronizationPreference
    | synchronizationHint
    | synchronizationPolicy
    | synchronizationCondition
    | synchronizationScopeClause
    | synchronizationProperty
    ;


/*
 * ============================================================================
 * 4. SYNCHRONIZATION POINT
 * ============================================================================
 *
 * A synchronization point identifies a semantic point at which execution
 * coordination may be required.
 */

synchronizationPoint
    : SYNC synchronizationReference synchronizationPointOptions? SEMICOLON
    ;


synchronizationPointOptions
    : LBRACE synchronizationPointOption* RBRACE
    ;


synchronizationPointOption
    : synchronizationRequirement
    | synchronizationConstraint
    | synchronizationPreference
    | synchronizationHint
    | synchronizationPolicy
    | synchronizationCondition
    ;


/*
 * ============================================================================
 * 5. WAIT
 * ============================================================================
 *
 * Waiting is execution-level intent.
 *
 * It does not prescribe blocking, polling, spinning, suspension, yielding,
 * event-loop integration, or any particular operating-system mechanism.
 */

synchronizationWait
    : WAIT synchronizationReference
      synchronizationWaitCondition?
      synchronizationWaitOptions?
      SEMICOLON
    ;


synchronizationWaitCondition
    : UNTIL expression
    ;


synchronizationWaitOptions
    : LBRACE synchronizationWaitOption* RBRACE
    ;


synchronizationWaitOption
    : synchronizationTimeout
    | synchronizationCancellation
    | synchronizationCondition
    | synchronizationPolicy
    | synchronizationHint
    ;


/*
 * ============================================================================
 * 6. SIGNAL
 * ============================================================================
 *
 * Signals semantic progress/completion/availability.
 */

synchronizationSignal
    : SIGNAL synchronizationReference
      synchronizationSignalValue?
      SEMICOLON
    ;


synchronizationSignalValue
    : expression
    ;


/*
 * ============================================================================
 * 7. JOIN
 * ============================================================================
 *
 * A join represents semantic completion dependency.
 *
 * The number of dependencies is unbounded by grammar design.
 */

synchronizationJoin
    : JOIN synchronizationReferenceList
      synchronizationJoinOptions?
      SEMICOLON
    ;


synchronizationJoinOptions
    : LBRACE synchronizationJoinOption* RBRACE
    ;


synchronizationJoinOption
    : synchronizationTimeout
    | synchronizationCancellation
    | synchronizationPolicy
    | synchronizationCondition
    | synchronizationHint
    ;


/*
 * ============================================================================
 * 8. SYNCHRONIZATION SCOPE
 * ============================================================================
 *
 * Applies synchronization intent to a structured execution region.
 */

synchronizationScope
    : SYNCHRONIZE synchronizationScopeTarget?
      synchronizationScopeOptions?
      block
    ;


synchronizationScopeTarget
    : synchronizationReference
    ;


synchronizationScopeOptions
    : LBRACE synchronizationScopeOption* RBRACE
    ;


synchronizationScopeOption
    : synchronizationRequirement
    | synchronizationConstraint
    | synchronizationPreference
    | synchronizationHint
    | synchronizationPolicy
    | synchronizationCondition
    ;


/*
 * ============================================================================
 * 9. DEPENDENCIES
 * ============================================================================
 *
 * Expresses semantic ordering without implementing scheduling.
 *
 * Example semantic relationship:
 *
 *     after A before B
 *
 * The scheduler may realize this using whatever mechanism is appropriate.
 */

synchronizationDependency
    : AFTER synchronizationReferenceList
      BEFORE synchronizationReferenceList
      synchronizationDependencyOptions?
      SEMICOLON
    ;


synchronizationDependencyOptions
    : LBRACE synchronizationDependencyOption* RBRACE
    ;


synchronizationDependencyOption
    : synchronizationRequirement
    | synchronizationConstraint
    | synchronizationPreference
    | synchronizationHint
    | synchronizationPolicy
    | synchronizationCondition
    ;


/*
 * ============================================================================
 * 10. PHASES
 * ============================================================================
 *
 * Execution phases describe semantic coordination boundaries.
 *
 * They do not prescribe a scheduling implementation.
 */

synchronizationPhase
    : PHASE synchronizationName
      synchronizationPhaseBody
    ;


synchronizationPhaseBody
    : LBRACE synchronizationPhaseClause* RBRACE
    ;


synchronizationPhaseClause
    : synchronizationPhaseEntry
    | synchronizationPhaseExit
    | synchronizationRequirement
    | synchronizationConstraint
    | synchronizationPreference
    | synchronizationHint
    | synchronizationPolicy
    | synchronizationCondition
    ;


synchronizationPhaseEntry
    : ENTER synchronizationReferenceList? SEMICOLON
    ;


synchronizationPhaseExit
    : EXIT synchronizationReferenceList? SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ASSERTIONS
 * ============================================================================
 *
 * Assertions describe semantic expectations.
 *
 * They do not force a specific runtime mechanism.
 */

synchronizationAssertion
    : ASSERT synchronizationPredicate SEMICOLON
    ;


synchronizationPredicate
    : synchronizationReference synchronizationStatePredicate
    | expression
    ;


synchronizationStatePredicate
    : IS synchronizationState
    ;


synchronizationState
    : READY
    | PENDING
    | ACTIVE
    | COMPLETE
    | FAILED
    | CANCELLED
    | AVAILABLE
    | UNAVAILABLE
    ;


/*
 * ============================================================================
 * 12. REFERENCES
 * ============================================================================
 *
 * References remain open-ended.
 *
 * They may refer to:
 *
 *     execution regions
 *     tasks
 *     computations
 *     quantum regions
 *     classical regions
 *     hardware regions
 *     distributed operations
 *     named synchronization objects
 *
 * The semantic layer determines what a reference denotes.
 */

synchronizationReference
    : qualifiedName
    ;


synchronizationReferenceList
    : synchronizationReference
      (COMMA synchronizationReference)*
    ;


/*
 * ============================================================================
 * 13. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * They must not silently become physical device selections.
 */

synchronizationRequirement
    : REQUIRES synchronizationValue SEMICOLON?
    ;


synchronizationValue
    : expression
    | synchronizationPropertyBlock
    ;


/*
 * ============================================================================
 * 14. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict acceptable realizations.
 */

synchronizationConstraint
    : CONSTRAINT synchronizationConstraintExpression SEMICOLON?
    ;


synchronizationConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * Failure to satisfy a preference does not inherently make the computation
 * semantically invalid.
 */

synchronizationPreference
    : PREFER synchronizationValue SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. HINTS
 * ============================================================================
 *
 * Hints are advisory implementation guidance.
 */

synchronizationHint
    : HINT synchronizationValue SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. POLICIES
 * ============================================================================
 *
 * Policy names are open-ended.
 *
 * New policies therefore do not require grammar changes.
 */

synchronizationPolicy
    : POLICY qualifiedName synchronizationPolicyArguments?
    ;


synchronizationPolicyArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 18. CONDITIONS
 * ============================================================================
 *
 * Conditions are ordinary language expressions.
 */

synchronizationCondition
    : WHEN expression
    ;


/*
 * ============================================================================
 * 19. TIMEOUT
 * ============================================================================
 *
 * Duration/value semantics belong to the canonical type/semantic layer.
 *
 * No unit or maximum is hard-coded here.
 */

synchronizationTimeout
    : TIMEOUT expression
    ;


/*
 * ============================================================================
 * 20. CANCELLATION
 * ============================================================================
 *
 * Cancellation semantics integrate with the canonical cancellation model.
 *
 * This grammar does not define cancellation tokens.
 */

synchronizationCancellation
    : CANCELLABLE
    | NONCANCELLABLE
    ;


/*
 * ============================================================================
 * 21. SCOPE CLAUSE
 * ============================================================================
 */

synchronizationScopeClause
    : SCOPE synchronizationScopeValue
    ;


synchronizationScopeValue
    : expression
    | synchronizationPropertyBlock
    ;


/*
 * ============================================================================
 * 22. PROPERTIES
 * ============================================================================
 *
 * Properties are open-ended semantic metadata.
 */

synchronizationProperty
    : identifier COLON expression synchronizationClauseTerminator?
    ;


synchronizationPropertyBlock
    : LBRACE synchronizationPropertyEntry* RBRACE
    ;


synchronizationPropertyEntry
    : identifier COLON expression synchronizationClauseTerminator?
    ;


/*
 * ============================================================================
 * 23. GENERIC CLAUSE TERMINATION
 * ============================================================================
 *
 * Semicolon remains optional inside structured property blocks where the
 * surrounding grammar supplies an unambiguous boundary.
 */

synchronizationClauseTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 24. EXECUTION-SYNCHRONIZATION COMPOSITION
 * ============================================================================
 *
 * This rule is the principal integration point consumed by:
 *
 *     grammar/execution/execution.g4
 *
 * It deliberately does not import scheduling, placement, dispatch, or
 * concurrency implementation syntax.
 */

executionSynchronizationClause
    : executionSynchronization
    ;


/*
 * ============================================================================
 * 25. END OF FILE
 * ============================================================================
 */