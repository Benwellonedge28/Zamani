/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/execution.g4
 *
 * Grammar:
 *     Execution
 *
 * Status:
 *     Production-ready execution-intent parser grammar
 *
 * Purpose:
 *     Owns source-level execution intent and execution-context composition.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> target resolution
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> hardware HAL
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * This file describes:
 *
 *     WHAT execution is requested.
 *
 * It does NOT describe:
 *
 *     HOW execution is implemented.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular QPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a vendor;
 *     - a device identifier;
 *     - a physical address;
 *     - a fixed topology;
 *     - a fixed number of devices;
 *     - a fixed number of qubits;
 *     - a fixed number of cores;
 *     - a fixed number of threads;
 *     - a fixed amount of memory;
 *     - a fixed network size;
 *     - a fixed queue size;
 *     - a fixed execution duration;
 *     - a fixed schedule;
 *     - a fixed placement;
 *     - a fixed backend;
 *     - a fixed simulator.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Execution syntax is therefore expressed in terms of:
 *
 *     intent
 *     context
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     resources
 *     placement intent
 *     scheduling intent
 *     dispatch intent
 *     lifecycle intent
 *     deployment intent
 *
 * rather than machine-specific implementation details.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution declaration syntax;
 *     - execution regions;
 *     - execution requests;
 *     - execution contexts;
 *     - execution requirements;
 *     - execution constraints;
 *     - execution preferences;
 *     - execution hints;
 *     - execution capability requirements;
 *     - execution resource requirements;
 *     - execution placement intent;
 *     - execution scheduling intent;
 *     - execution dispatch intent;
 *     - execution synchronization intent;
 *     - execution lifecycle intent;
 *     - execution result-binding intent;
 *     - execution failure-policy intent;
 *     - execution retry-policy intent;
 *     - execution deployment intent;
 *     - execution option/property composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - types;
 *     - functions;
 *     - modules;
 *     - quantum operations;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL semantics;
 *     - hardware discovery;
 *     - resource discovery;
 *     - routing algorithms;
 *     - scheduling algorithms;
 *     - optimization algorithms;
 *     - resilience algorithms;
 *     - runtime implementation;
 *     - deployment implementation;
 *     - device communication;
 *     - backend APIs.
 *
 * ============================================================================
 *
 * CANONICAL COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical parser composition layer is responsible for importing this
 * grammar and exposing:
 *
 *     executionDeclaration
 *
 * where execution syntax is legal in the source language.
 *
 * This file MUST NOT define:
 *
 *     lexer grammar ...
 *     grammar Zamani ...
 *     lexer tokens
 *     EOF entry points
 *
 * ============================================================================
 *
 * EXISTING LEXER CONTRACT
 * ============================================================================
 *
 * This grammar intentionally uses only execution-related tokens already
 * established by the canonical lexer where such tokens exist, including:
 *
 *     EXECUTE
 *     DEPLOY
 *     SIMULATE
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *     WITH
 *     IN
 *
 * Open-ended semantic names such as:
 *
 *     target
 *     resource
 *     capability
 *     schedule
 *     dispatch
 *     placement
 *     retry
 *     result
 *
 * remain identifiers unless the canonical lexer later makes them reserved.
 *
 * This avoids forcing an ever-growing closed vocabulary into the lexer.
 *
 * ============================================================================
 *
 * IMPORTANT QUANTUM INTEGRATION
 * ============================================================================
 *
 * `grammar/antlr/Quantum.g4` already defines:
 *
 *     quantumExecutionRegion
 *         : EXECUTE quantumBlock
 *         ;
 *
 * Therefore this file MUST NOT redefine that rule or steal ownership of
 * quantum operation syntax.
 *
 * Quantum execution remains:
 *
 *     quantum source intent
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     execution
 *
 * This grammar supplies generic execution intent around semantic program
 * execution. It does not replace the quantum execution region.
 *
 * ============================================================================
 *
 * COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compilation intent belongs to:
 *
 *     grammar/compile/
 *
 * Execution begins after semantic compilation intent has been resolved.
 *
 * Therefore:
 *
 *     compile != execute
 *
 * and:
 *
 *     compilation target != execution target
 *
 * Execution may consume the result of compilation, but this grammar does not
 * prescribe how compilation is performed.
 *
 * ============================================================================
 *
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling intent is represented here only as source intent.
 *
 * This grammar does NOT implement:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     critical-path scheduling
 *     RCPSP
 *     resource allocation
 *     timing calculation
 *     dependency analysis
 *     pulse scheduling
 *
 * Those belong to:
 *
 *     quantum scheduling
 *     generic scheduling
 *     hardware timing
 *
 * ============================================================================
 *
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Placement/routing requests are represented as intent.
 *
 * This file does not:
 *
 *     - map logical qubits;
 *     - select physical qubits;
 *     - construct topology;
 *     - insert SWAPs;
 *     - choose paths;
 *     - bind hardware locations.
 *
 * ============================================================================
 *
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Retry/failure/recovery syntax expresses policy intent only.
 *
 * Resilience remains responsible for deciding whether and how recovery occurs.
 *
 * This grammar does NOT implement:
 *
 *     retry algorithms;
 *     recovery algorithms;
 *     fault diagnosis;
 *     QEC;
 *     mitigation;
 *     backend switching;
 *     checkpoint reconstruction.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Execution may reference resource and capability expressions.
 *
 * It does not discover or allocate those resources.
 *
 * The semantic/resource layers determine:
 *
 *     whether a requirement is satisfiable;
 *     whether a capability exists;
 *     whether a constraint can be satisfied;
 *     whether a preference can be honored.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no finite parser limits.
 *
 * The grammar contains no:
 *
 *     MAX_DEVICES
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_JOBS
 *     MAX_STAGES
 *     MAX_RETRIES
 *     MAX_DEPLOYMENTS
 *     MAX_ARGUMENTS
 *
 * Repetition is represented structurally through parser repetition.
 *
 * Actual limits belong to:
 *
 *     - resource analysis;
 *     - compiler policy;
 *     - execution policy;
 *     - operating-system limits;
 *     - runtime limits;
 *     - target capabilities;
 *     - explicitly supplied user constraints.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to the canonical token stream.
 *
 * This file contains:
 *
 *     - no semantic actions;
 *     - no runtime calls;
 *     - no device discovery;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no random selection;
 *     - no backend selection;
 *     - no mutable global state.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Compiler/runtime implementation requirements:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     stable Rust
 *     no unsafe Rust
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Execution;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. EXECUTION DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     execute expression;
 *
 *     execute {
 *         ...
 *     }
 *
 *     execute expression with {
 *         ...
 *     };
 *
 * The expression identifies the semantic computation.
 *
 * The execution context describes intent around its realization.
 *
 * Nothing here causes execution during parsing.
 * ============================================================================
 */

executionDeclaration
    : EXECUTE executionRequest executionTerminator?
    ;


/*
 * ============================================================================
 * 2. EXECUTION REQUEST
 * ============================================================================
 */

executionRequest
    : executionSubject
      executionContext?
    ;


/*
 * ============================================================================
 * 3. EXECUTION SUBJECT
 * ============================================================================
 *
 * The subject remains an ordinary expression.
 *
 * This permits:
 *
 *     execute main();
 *     execute circuit;
 *     execute program;
 *     execute expression;
 *     execute pipeline;
 *     execute module::entry;
 *
 * without creating a closed list of executable entities.
 *
 * ============================================================================
 */

executionSubject
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 4. EXECUTION CONTEXT
 * ============================================================================
 *
 * The context is an unordered semantic collection.
 *
 * Its meaning is determined by semantic analysis.
 *
 * The grammar intentionally permits multiple categories without assigning
 * execution priority.
 * ============================================================================
 */

executionContext
    : WITH executionContextBody
    | executionContextBody
    ;


executionContextBody
    : LBRACE executionClause+ RBRACE
    ;


/*
 * ============================================================================
 * 5. EXECUTION CLAUSES
 * ============================================================================
 */

executionClause
    : executionRequirement
    | executionConstraint
    | executionPreference
    | executionHint
    | executionCapability
    | executionResource
    | executionTarget
    | executionPlacement
    | executionSchedule
    | executionDispatch
    | executionSynchronization
    | executionLifecycle
    | executionResult
    | executionFailurePolicy
    | executionRetryPolicy
    | executionOption
    | executionProperty
    ;


/*
 * ============================================================================
 * 6. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * Example:
 *
 *     requires quantum;
 *
 *     requires expression;
 *
 * The grammar does not decide whether the requirement can be satisfied.
 * ============================================================================
 */

executionRequirement
    : REQUIRES executionValue executionClauseTerminator?
    ;


executionValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 7. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict acceptable realizations.
 *
 * They do not select one physical implementation.
 * ============================================================================
 */

executionConstraint
    : identifier executionConstraintOperator expression executionClauseTerminator?
    ;


executionConstraintOperator
    : EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    ;


/*
 * ============================================================================
 * 8. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory guidance.
 *
 * Failure to honor a preference MUST NOT automatically imply semantic
 * execution failure.
 * ============================================================================
 */

executionPreference
    : identifier executionPreferenceValue executionClauseTerminator?
    ;


executionPreferenceValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 9. HINTS
 * ============================================================================
 *
 * Hints are advisory implementation information.
 *
 * A hint must never become a hidden semantic requirement merely because a
 * backend happens to understand it.
 * ============================================================================
 */

executionHint
    : identifier executionHintValue executionClauseTerminator?
    ;


executionHintValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 10. CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * Capability names are open-ended.
 *
 * Examples:
 *
 *     capability quantum;
 *     capability tensor;
 *     capability realtime;
 *     capability distributed;
 *
 * The parser does not enumerate possible future capabilities.
 * ============================================================================
 */

executionCapability
    : identifier executionCapabilityValue executionClauseTerminator?
    ;


executionCapabilityValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 11. RESOURCE INTENT
 * ============================================================================
 *
 * Resource syntax describes semantic requirements.
 *
 * It does not allocate resources.
 *
 * Examples:
 *
 *     resource memory;
 *     resource quantum;
 *     resource accelerator;
 *
 * Resource quantities and limits are expressions.
 *
 * Therefore:
 *
 *     resource qubits >= required
 *
 * is possible without encoding a fixed machine size.
 * ============================================================================
 */

executionResource
    : identifier executionResourceValue executionClauseTerminator?
    ;


executionResourceValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 12. TARGET INTENT
 * ============================================================================
 *
 * A target is an abstract execution target.
 *
 * It may identify:
 *
 *     a target class;
 *     a semantic environment;
 *     a target profile;
 *     a target expression;
 *     a deployment target;
 *
 * It does NOT automatically identify a physical machine.
 *
 * ============================================================================
 */

executionTarget
    : identifier executionTargetValue executionClauseTerminator?
    ;


executionTargetValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 13. PLACEMENT INTENT
 * ============================================================================
 *
 * Placement specifies where execution is preferred or constrained to occur.
 *
 * It does not perform placement.
 *
 * Examples can be represented through open semantic properties:
 *
 *     placement { locality: ...; affinity: ...; }
 *
 * ============================================================================
 */

executionPlacement
    : identifier executionPlacementValue executionClauseTerminator?
    ;


executionPlacementValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 14. SCHEDULING INTENT
 * ============================================================================
 *
 * Scheduling syntax expresses requirements or preferences for temporal
 * realization.
 *
 * It does NOT calculate a schedule.
 *
 * Examples of semantic properties:
 *
 *     scheduling { policy: ...; priority: ...; deadline: ...; }
 *
 * The grammar does not enumerate scheduling algorithms.
 * ============================================================================
 */

executionSchedule
    : identifier executionScheduleValue executionClauseTerminator?
    ;


executionScheduleValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 15. DISPATCH INTENT
 * ============================================================================
 *
 * Dispatch determines semantic intent for handing a compiled/executable
 * representation to an execution environment.
 *
 * Actual dispatch is a runtime concern.
 * ============================================================================
 */

executionDispatch
    : identifier executionDispatchValue executionClauseTerminator?
    ;


executionDispatchValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 16. SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization expresses an ordering or completion requirement.
 *
 * It does not introduce synchronization primitives into the generated target
 * by itself.
 * ============================================================================
 */

executionSynchronization
    : identifier executionSynchronizationValue executionClauseTerminator?
    ;


executionSynchronizationValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 17. LIFECYCLE
 * ============================================================================
 *
 * Lifecycle intent can describe semantic states such as:
 *
 *     start
 *     pause
 *     resume
 *     stop
 *     suspend
 *     terminate
 *
 * These words remain identifiers so future lifecycle states do not require
 * lexer changes.
 * ============================================================================
 */

executionLifecycle
    : identifier executionLifecycleValue executionClauseTerminator?
    ;


executionLifecycleValue
    : expression?
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 18. RESULT BINDING
 * ============================================================================
 *
 * Execution results may be bound to ordinary language expressions through
 * result-binding syntax.
 *
 * The exact result type is determined semantically.
 *
 * This is important for:
 *
 *     classical results;
 *     quantum measurements;
 *     accelerator results;
 *     distributed results;
 *     HDL/hardware observations;
 *     future computational domains.
 * ============================================================================
 */

executionResult
    : identifier executionResultValue executionClauseTerminator?
    ;


executionResultValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 19. FAILURE POLICY
 * ============================================================================
 *
 * Failure policy is declarative intent.
 *
 * It does not implement recovery.
 *
 * This keeps execution grammar separate from resilience.
 * ============================================================================
 */

executionFailurePolicy
    : identifier executionFailurePolicyValue executionClauseTerminator?
    ;


executionFailurePolicyValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 20. RETRY POLICY
 * ============================================================================
 *
 * Retry is deliberately represented as policy data.
 *
 * No finite retry count is encoded by this grammar.
 *
 * A caller may express:
 *
 *     retry { ... }
 *
 * and semantic/resilience layers determine:
 *
 *     whether retry is legal;
 *     how many attempts are allowed;
 *     whether state can be recovered;
 *     whether retry preserves semantics.
 * ============================================================================
 */

executionRetryPolicy
    : identifier executionRetryPolicyValue executionClauseTerminator?
    ;


executionRetryPolicyValue
    : expression
    | executionPropertyBlock
    ;


/*
 * ============================================================================
 * 21. GENERIC EXECUTION OPTION
 * ============================================================================
 *
 * Open-ended options permit future execution capabilities without forcing
 * every future capability into this grammar.
 *
 * Options are data.
 *
 * They are not automatically authoritative.
 * ============================================================================
 */

executionOption
    : identifier
      (
          ASSIGN expression
        | executionPropertyBlock
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 22. GENERIC PROPERTY
 * ============================================================================
 *
 * Generic properties are the principal extensibility mechanism.
 *
 * This is important for POCO-REAF because future execution technologies can
 * introduce semantic properties without requiring a closed list of hardware
 * concepts.
 * ============================================================================
 */

executionProperty
    : identifier
      executionPropertyValue
      executionClauseTerminator?
    ;


executionPropertyValue
    : COLON expression
    | ASSIGN expression
    | executionPropertyBlock
    | expression
    ;


executionPropertyBlock
    : LBRACE executionPropertyEntry+ RBRACE
    ;


executionPropertyEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


executionPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 23. ASYNCHRONOUS EXECUTION
 * ============================================================================
 *
 * The existing ASYNC keyword is reused.
 *
 * Async execution is source intent.
 *
 * It does not prescribe a particular runtime task model.
 * ============================================================================
 */

executionAsyncDeclaration
    : ASYNC EXECUTE executionRequest executionTerminator?
    ;


/*
 * ============================================================================
 * 24. SPAWNED EXECUTION
 * ============================================================================
 *
 * The existing SPAWN keyword is reused.
 *
 * No fixed task/thread/process model is assumed.
 * ============================================================================
 */

executionSpawnDeclaration
    : SPAWN executionRequest executionTerminator?
    ;


/*
 * ============================================================================
 * 25. AWAIT
 * ============================================================================
 *
 * The existing AWAIT keyword expresses waiting for an execution value or
 * completion boundary.
 *
 * Runtime semantics belong downstream.
 * ============================================================================
 */

executionAwaitStatement
    : AWAIT expression executionTerminator?
    ;


/*
 * ============================================================================
 * 26. PARALLEL EXECUTION
 * ============================================================================
 *
 * PARALLEL expresses semantic concurrency/parallelism intent.
 *
 * It does not mean:
 *
 *     one thread;
 *     one core;
 *     one GPU;
 *     one device;
 *     one node.
 *
 * The execution planner determines realization.
 * ============================================================================
 */

executionParallelDeclaration
    : PARALLEL executionParallelSubject executionParallelContext?
      executionTerminator?
    ;


executionParallelSubject
    : expression
    | blockExpression
    ;


executionParallelContext
    : WITH executionPropertyBlock
    ;


/*
 * ============================================================================
 * 27. DEPLOYMENT
 * ============================================================================
 *
 * Deployment is distinct from execution.
 *
 * Deployment describes intent to make an executable computation available in
 * an execution environment.
 *
 * It does not perform deployment.
 *
 * The existing DEPLOY keyword is used.
 * ============================================================================
 */

executionDeploymentDeclaration
    : DEPLOY executionDeploymentSubject executionDeploymentContext?
      executionTerminator?
    ;


executionDeploymentSubject
    : expression
    | blockExpression
    ;


executionDeploymentContext
    : WITH executionPropertyBlock
    ;


/*
 * ============================================================================
 * 28. SIMULATION
 * ============================================================================
 *
 * Simulation is an execution strategy.
 *
 * It is NOT part of the canonical quantum semantics.
 *
 * The existing SIMULATE keyword is reused.
 *
 * The simulator implementation remains downstream.
 * ============================================================================
 */

executionSimulationDeclaration
    : SIMULATE executionSimulationSubject executionSimulationContext?
      executionTerminator?
    ;


executionSimulationSubject
    : expression
    | blockExpression
    ;


executionSimulationContext
    : WITH executionPropertyBlock
    ;


/*
 * ============================================================================
 * 29. EXECUTION PIPELINE
 * ============================================================================
 *
 * This syntax permits an execution request to describe an ordered semantic
 * pipeline without encoding implementation algorithms.
 *
 * Example:
 *
 *     execute program with {
 *         pipeline: [stage_a, stage_b, stage_c];
 *     };
 *
 * The actual lowering and stage execution remain compiler/runtime concerns.
 * ============================================================================
 */

executionPipeline
    : identifier executionPipelineBody executionClauseTerminator?
    ;


executionPipelineBody
    : executionPropertyBlock
    ;


/*
 * ============================================================================
 * 30. EXECUTION BOUNDARY
 * ============================================================================
 *
 * An execution boundary explicitly marks a semantic handoff.
 *
 * It may be used by frontend/semantic tooling to distinguish ordinary source
 * computation from execution intent.
 *
 * The boundary does not perform a runtime operation.
 * ============================================================================
 */

executionBoundary
    : EXECUTE executionBoundarySubject executionTerminator?
    ;


executionBoundarySubject
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 31. EXECUTION HANDLE
 * ============================================================================
 *
 * A handle is represented as an ordinary expression.
 *
 * No runtime handle representation is hard-coded here.
 * ============================================================================
 */

executionHandle
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 32. EXECUTION TERMINATORS
 * ============================================================================
 *
 * Semicolon remains optional at the execution boundary where the canonical
 * source grammar permits expression/block termination.
 *
 * The parser does not impose a global statement-termination policy.
 * ============================================================================
 */

executionTerminator
    : SEMI
    ;


executionClauseTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 33. EXECUTION PROPERTY LIST
 * ============================================================================
 *
 * Named properties remain extensible.
 * ============================================================================
 */

executionPropertyList
    : executionPropertyEntry+
    ;


/*
 * ============================================================================
 * 34. EXECUTION REQUIREMENT BLOCK
 * ============================================================================
 *
 * This dedicated form allows multiple requirements while keeping requirement
 * semantics distinct from preferences and hints.
 * ============================================================================
 */

executionRequirementBlock
    : LBRACE executionRequirementEntry+ RBRACE
    ;


executionRequirementEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 35. EXECUTION CONSTRAINT BLOCK
 * ============================================================================
 */

executionConstraintBlock
    : LBRACE executionConstraintEntry+ RBRACE
    ;


executionConstraintEntry
    : executionPropertyName
      executionConstraintOperator
      expression
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 36. EXECUTION PREFERENCE BLOCK
 * ============================================================================
 */

executionPreferenceBlock
    : LBRACE executionPreferenceEntry+ RBRACE
    ;


executionPreferenceEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 37. EXECUTION HINT BLOCK
 * ============================================================================
 */

executionHintBlock
    : LBRACE executionHintEntry+ RBRACE
    ;


executionHintEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 38. EXECUTION RESOURCE BLOCK
 * ============================================================================
 */

executionResourceBlock
    : LBRACE executionResourceEntry+ RBRACE
    ;


executionResourceEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 39. EXECUTION CAPABILITY BLOCK
 * ============================================================================
 */

executionCapabilityBlock
    : LBRACE executionCapabilityEntry+ RBRACE
    ;


executionCapabilityEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 40. EXECUTION TARGET BLOCK
 * ============================================================================
 */

executionTargetBlock
    : LBRACE executionTargetEntry+ RBRACE
    ;


executionTargetEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 41. EXECUTION PLACEMENT BLOCK
 * ============================================================================
 */

executionPlacementBlock
    : LBRACE executionPlacementEntry+ RBRACE
    ;


executionPlacementEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 42. EXECUTION SCHEDULE BLOCK
 * ============================================================================
 */

executionScheduleBlock
    : LBRACE executionScheduleEntry+ RBRACE
    ;


executionScheduleEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 43. EXECUTION DISPATCH BLOCK
 * ============================================================================
 */

executionDispatchBlock
    : LBRACE executionDispatchEntry+ RBRACE
    ;


executionDispatchEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 44. EXECUTION SYNCHRONIZATION BLOCK
 * ============================================================================
 */

executionSynchronizationBlock
    : LBRACE executionSynchronizationEntry+ RBRACE
    ;


executionSynchronizationEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 45. EXECUTION RESULT BLOCK
 * ============================================================================
 */

executionResultBlock
    : LBRACE executionResultEntry+ RBRACE
    ;


executionResultEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 46. EXECUTION FAILURE BLOCK
 * ============================================================================
 */

executionFailureBlock
    : LBRACE executionFailureEntry+ RBRACE
    ;


executionFailureEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 47. EXECUTION RETRY BLOCK
 * ============================================================================
 */

executionRetryBlock
    : LBRACE executionRetryEntry+ RBRACE
    ;


executionRetryEntry
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 48. EXECUTION CONTEXT PROPERTY COMPOSITION
 * ============================================================================
 *
 * This generic rule is intentionally available to semantic adapters.
 *
 * It allows future execution domains to carry structured information without
 * modifying this grammar every time a new accelerator or execution model is
 * introduced.
 * ============================================================================
 */

executionContextProperty
    : executionPropertyName
      (
          COLON expression
        | ASSIGN expression
        | executionPropertyBlock
      )
      executionClauseTerminator?
    ;


/*
 * ============================================================================
 * 49. EXECUTION CONTEXT LIST
 * ============================================================================
 */

executionContextList
    : executionContextProperty+
    ;


/*
 * ============================================================================
 * 50. OPEN EXECUTION EXTENSION
 * ============================================================================
 *
 * Extension points are named semantic properties rather than hard-coded
 * machine concepts.
 *
 * This is the mechanism that lets future computing substrates participate in
 * execution without changing the fundamental execution model.
 * ============================================================================
 */

executionExtension
    : identifier
      executionExtensionBody
      executionClauseTerminator?
    ;


executionExtensionBody
    : executionPropertyBlock
    | expression
    ;


/*
 * ============================================================================
 * 51. SOURCE-LEVEL EXECUTION POLICY
 * ============================================================================
 *
 * Policy is declarative data.
 *
 * The runtime/resilience/compiler layers interpret it.
 * ============================================================================
 */

executionPolicy
    : identifier
      executionPolicyBody
      executionClauseTerminator?
    ;


executionPolicyBody
    : executionPropertyBlock
    | expression
    ;


/*
 * ============================================================================
 * 52. EXECUTION SESSION INTENT
 * ============================================================================
 *
 * A session is a semantic grouping, not a runtime object definition.
 * ============================================================================
 */

executionSession
    : identifier
      executionSessionBody
      executionClauseTerminator?
    ;


executionSessionBody
    : executionPropertyBlock
    | blockExpression
    ;


/*
 * ============================================================================
 * 53. EXECUTION PROGRAM COMPOSITION
 * ============================================================================
 *
 * This rule allows multiple execution requests to be represented as a
 * structured execution composition.
 *
 * There is no fixed number of execution units.
 * ============================================================================
 */

executionComposition
    : identifier executionCompositionBody
    ;


executionCompositionBody
    : LBRACE executionCompositionEntry+ RBRACE
    ;


executionCompositionEntry
    : executionDeclaration
    | executionAsyncDeclaration
    | executionSpawnDeclaration
    | executionAwaitStatement
    | executionParallelDeclaration
    | executionDeploymentDeclaration
    | executionSimulationDeclaration
    | executionExtension
    ;


/*
 * ============================================================================
 * 54. EXECUTION PROGRAM
 * ============================================================================
 *
 * A complete execution specification may contain arbitrary numbers of
 * execution entries.
 *
 * ============================================================================
 */

executionProgram
    : executionCompositionEntry+
    ;


executionCompositionEntry
    : executionDeclaration
    | executionAsyncDeclaration
    | executionSpawnDeclaration
    | executionAwaitStatement
    | executionParallelDeclaration
    | executionDeploymentDeclaration
    | executionSimulationDeclaration
    | executionExtension
    ;


/*
 * ============================================================================
 * 55. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The AST produced from this grammar MUST preserve:
 *
 *     1. execution subject;
 *     2. execution context;
 *     3. requirement;
 *     4. constraint;
 *     5. preference;
 *     6. hint;
 *     7. capability;
 *     8. resource;
 *     9. target;
 *    10. placement;
 *    11. scheduling;
 *    12. dispatch;
 *    13. synchronization;
 *    14. lifecycle;
 *    15. result;
 *    16. failure policy;
 *    17. retry policy;
 *    18. extension properties.
 *
 * Semantic analysis MUST NOT collapse these categories into a single
 * untyped key/value dictionary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. IR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar does NOT construct IR.
 *
 * Semantic lowering may produce:
 *
 *     classical execution intent
 *     quantum execution intent
 *     HDL execution intent
 *     distributed execution intent
 *     accelerator execution intent
 *
 * Quantum computations MUST ultimately enter the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file MUST NOT create:
 *
 *     QuantumGate
 *     QuantumCircuit
 *     QubitId
 *     PhysicalQubitId
 *     schedule nodes
 *     routing nodes
 *     hardware instructions
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware realization occurs downstream.
 *
 * An execution target expression may refer semantically to:
 *
 *     target classes;
 *     capability sets;
 *     resource requirements;
 *     deployment environments;
 *     hardware descriptions;
 *     runtime-discovered targets.
 *
 * The grammar itself does not resolve any of these.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. RUNTIME INTEGRATION CONTRACT
 * ============================================================================
 *
 * Runtime receives a validated execution plan rather than a parse tree.
 *
 * The runtime is responsible for:
 *
 *     dispatch;
 *     lifecycle;
 *     cancellation;
 *     synchronization;
 *     result collection;
 *     resource interaction;
 *     backend interaction;
 *     runtime failures.
 *
 * None of these operations occur in the parser.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. RESILIENCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Execution failure/retry declarations are inputs to resilience policy.
 *
 * They do not implement:
 *
 *     retry;
 *     rollback;
 *     resume;
 *     remap;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     reoptimize;
 *     backend switching;
 *     quarantine;
 *     abort.
 *
 * Those decisions belong to the resilience subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. SECURITY CONTRACT
 * ============================================================================
 *
 * Execution grammar contains no:
 *
 *     filesystem access;
 *     network access;
 *     process spawning;
 *     device access;
 *     shell execution;
 *     dynamic code execution;
 *     credential access.
 *
 * `spawn` is source-level execution intent only.
 *
 * Security/capability analysis must authorize actual runtime operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The following are intentionally absent:
 *
 *     MAX_EXECUTIONS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_RESOURCES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_JOBS
 *     MAX_PIPELINE_STAGES
 *     MAX_RETRIES
 *     MAX_DEPLOYMENTS
 *
 * The only practical limits are imposed by:
 *
 *     parser/runtime memory;
 *     address space;
 *     compilation resources;
 *     explicit policy;
 *     execution resources;
 *     target capabilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 62. DETERMINISM CONTRACT
 * ============================================================================
 *
 * The parser preserves source order.
 *
 * It does not decide:
 *
 *     which target wins;
 *     which resource wins;
 *     which backend wins;
 *     which schedule wins;
 *     which route wins;
 *     which recovery action wins.
 *
 * Those decisions belong to deterministic policy/planning layers downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 63. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar is additive and open-ended.
 *
 * New execution capabilities should preferably be introduced as:
 *
 *     identifier + structured property/value
 *
 * rather than by expanding a finite keyword inventory.
 *
 * Existing canonical lexer tokens remain authoritative.
 *
 * A future reserved execution keyword must be introduced centrally in:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and then consumed here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 64. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     execute expression;
 *     execute block;
 *     execute expression with { ... };
 *     async execute ...;
 *     spawn ...;
 *     await ...;
 *     parallel ...;
 *     deploy ...;
 *     simulate ...;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     capabilities;
 *     resources;
 *     targets;
 *     placement;
 *     scheduling;
 *     dispatch;
 *     synchronization;
 *     lifecycle;
 *     result binding;
 *     failure policies;
 *     retry policies;
 *     nested property blocks;
 *     open-ended extensions.
 *
 * Negative tests MUST cover:
 *
 *     empty execution context;
 *     malformed property blocks;
 *     malformed constraints;
 *     missing execution subject;
 *     missing required values;
 *     malformed assignment;
 *     malformed nested contexts.
 *
 * Boundary tests MUST cover:
 *
 *     zero execution declarations in optional contexts;
 *     very large numbers of clauses;
 *     very large property blocks;
 *     deeply nested valid contexts;
 *     very large execution programs.
 *
 * Scalability tests MUST verify the grammar contains no source-level limit
 * related to:
 *
 *     qubits;
 *     cores;
 *     threads;
 *     devices;
 *     nodes;
 *     resources;
 *     memory;
 *     execution units.
 *
 * Cross-domain tests MUST cover:
 *
 *     classical + execution;
 *     quantum + execution;
 *     HDL + execution;
 *     distributed + execution;
 *     AI + execution;
 *     hybrid + execution;
 *     quantum + classical + distributed + execution;
 *     quantum + HDL + hardware + execution.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 65. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It compiles against the canonical ZamaniLexer.
 *
 *     [ ] It introduces no lexer rules.
 *
 *     [ ] It introduces no duplicate identifier rule.
 *
 *     [ ] It introduces no duplicate expression rule.
 *
 *     [ ] It introduces no duplicate type rule.
 *
 *     [ ] It introduces no duplicate block rule.
 *
 *     [ ] It introduces no physical hardware assumption.
 *
 *     [ ] It introduces no machine-size limit.
 *
 *     [ ] It introduces no quantum-size limit.
 *
 *     [ ] It introduces no scheduling algorithm.
 *
 *     [ ] It introduces no routing algorithm.
 *
 *     [ ] It introduces no optimization algorithm.
 *
 *     [ ] It introduces no resilience algorithm.
 *
 *     [ ] It does not construct IR.
 *
 *     [ ] It does not execute code.
 *
 *     [ ] It preserves execution intent in the AST.
 *
 *     [ ] It supports arbitrary extensible execution properties.
 *
 *     [ ] It supports classical execution.
 *
 *     [ ] It supports quantum execution integration.
 *
 *     [ ] It supports HDL/hardware execution integration.
 *
 *     [ ] It supports distributed execution integration.
 *
 *     [ ] It supports heterogeneous execution integration.
 *
 *     [ ] It supports asynchronous execution intent.
 *
 *     [ ] It supports parallel execution intent.
 *
 *     [ ] It supports deployment intent.
 *
 *     [ ] It supports simulation intent.
 *
 *     [ ] It supports execution policy.
 *
 *     [ ] It supports failure/retry intent without implementing recovery.
 *
 *     [ ] It remains compatible with POCO-REAF.
 *
 *     [ ] Compiler implementation remains Rust 1.97/1.97.1 compatible.
 *
 *     [ ] Compiler implementation uses no unsafe Rust.
 *
 * ============================================================================
 */