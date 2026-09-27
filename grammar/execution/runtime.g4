/*
 * Zamani — Execution Runtime Grammar
 *
 * File:
 *   grammar/execution/runtime.g4
 *
 * Status:
 *   Normative execution-domain grammar component.
 *
 * Purpose:
 *   Defines target-independent runtime intent:
 *     - runtime environments
 *     - execution lifecycle
 *     - entry points
 *     - execution policies
 *     - runtime configuration
 *     - scheduling intent
 *     - placement intent
 *     - resilience/recovery intent
 *     - checkpointing
 *     - observability
 *     - tracing
 *     - profiling
 *     - runtime capabilities
 *     - runtime requirements
 *     - lifecycle hooks
 *     - cancellation
 *     - termination
 *
 * Does NOT own:
 *   - lexical tokens
 *   - concrete hardware topology
 *   - physical device identifiers
 *   - compiler optimization
 *   - quantum IR
 *   - QEC implementation
 *   - ZQN implementation
 *   - HAL implementation
 *   - vendor APIs
 *   - fixed resource limits
 *   - runtime implementation
 *
 * Architectural position:
 *
 *   Zamani source
 *       |
 *       v
 *   Zamani.g4
 *       |
 *       v
 *   frontend AST
 *       |
 *       v
 *   semantic analysis
 *       |
 *       +--> resource/capability validation
 *       |
 *       +--> execution intent
 *       |
 *       v
 *   canonical semantic model / IR
 *       |
 *       +--> scheduling
 *       +--> placement
 *       +--> resilience
 *       +--> routing
 *       +--> quantum::ir
 *       +--> HDL/hardware lowering
 *       |
 *       v
 *   target realization
 *
 * Integration:
 *
 *   This grammar is intended to be imported/composed by the canonical
 *   grammar/Zamani.g4 composition root.
 *
 *   It assumes the canonical Zamani lexer vocabulary generated from
 *   Zamani.g4 is available through:
 *
 *       tokenVocab=ZamaniLexer
 *
 *   The exact AST, semantic, and IR types remain owned by the Rust
 *   frontend/compiler implementation. This grammar deliberately does
 *   not introduce a second execution IR.
 *
 * Rust compatibility:
 *   The grammar itself is language-independent ANTLR syntax.
 *   Generated parser integration is intended for the repository's
 *   Rust 2021 / Rust 1.97 / Rust 1.97.1 toolchain.
 *
 * Safety:
 *   No unsafe Rust requirement is introduced by this grammar.
 *
 * Scalability:
 *   No universal maximum for:
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     threads
 *     tasks
 *     processes
 *     nodes
 *     devices
 *     memory
 *     storage
 *     timelines
 *     checkpoints
 *     events
 *     streams
 *
 *   Resource quantities are program requirements or runtime observations,
 *   never compiler-wide capacity constants.
 */

parser grammar Runtime;

options {
    tokenVocab = ZamaniLexer;
}

/* ====================================================================== */
/* ROOT                                                                   */
/* ====================================================================== */

/*
 * Runtime declarations are intentionally composable.
 *
 * The root Zamani.g4 should dispatch into runtimeDeclaration where
 * execution-domain syntax is accepted.
 */
runtimeDeclaration
    : runtimeEnvironmentDeclaration
    | runtimeEntryPointDeclaration
    | runtimePolicyDeclaration
    | runtimeRequirementDeclaration
    | runtimeCapabilityDeclaration
    | runtimeLifecycleDeclaration
    ;

/*
 * Runtime statements may be embedded in ordinary Zamani blocks.
 *
 * Zamani.g4 should dispatch runtimeStatement from the universal statement
 * rule rather than creating a second statement hierarchy.
 */
runtimeStatement
    : runtimeStartStatement
    | runtimeStopStatement
    | runtimeCancelStatement
    | runtimePauseStatement
    | runtimeResumeStatement
    | runtimeCheckpointStatement
    | runtimeRestoreStatement
    | runtimeWaitStatement
    | runtimeYieldStatement
    | runtimeSpawnStatement
    | runtimeJoinStatement
    | runtimeSignalStatement
    | runtimeObserveStatement
    | runtimeTraceStatement
    | runtimeProfileStatement
    | runtimeExecuteStatement
    ;

/* ====================================================================== */
/* RUNTIME ENVIRONMENTS                                                    */
/* ====================================================================== */

runtimeEnvironmentDeclaration
    : 'runtime'
      identifier
      runtimeEnvironmentBody
    ;

runtimeEnvironmentBody
    : '{'
      runtimeEnvironmentMember*
      '}'
    ;

runtimeEnvironmentMember
    : runtimeEnvironmentOption
    | runtimeRequirement
    | runtimeCapabilityRequirement
    | runtimeResourceRequirement
    | runtimePolicy
    | runtimeLifecycleHook
    | runtimeObservationConfiguration
    | runtimeCheckpointConfiguration
    | runtimeRecoveryConfiguration
    ;

runtimeEnvironmentOption
    : identifier
      '='
      expression
    ;

/*
 * The identifier/value model intentionally permits future runtime
 * attributes without changing this grammar for every backend feature.
 */
runtimeAttribute
    : identifier
      ( '=' expression )?
    ;

/* ====================================================================== */
/* ENTRY POINTS                                                            */
/* ====================================================================== */

runtimeEntryPointDeclaration
    : 'entry'
      'runtime'
      identifier
      runtimeEntryPointSignature?
      runtimeEntryPointBody?
    ;

runtimeEntryPointSignature
    : '('
      runtimeParameterList?
      ')'
      runtimeReturnClause?
    ;

runtimeParameterList
    : runtimeParameter
      (',' runtimeParameter)*
    ;

runtimeParameter
    : identifier
      (':' typeExpression)?
      ( '=' expression )?
    ;

runtimeReturnClause
    : '->'
      typeExpression
    ;

runtimeEntryPointBody
    : block
    ;

/*
 * Explicit entry-point properties remain semantic metadata.
 */
runtimeEntryPointModifier
    : 'deterministic'
    | 'reentrant'
    | 'idempotent'
    | 'restartable'
    | 'checkpointable'
    | 'observable'
    | 'traceable'
    | 'profileable'
    ;

/* ====================================================================== */
/* EXECUTION                                                              */
/* ====================================================================== */

runtimeExecuteStatement
    : 'execute'
      runtimeExecutionTarget?
      runtimeExecutionArguments?
      runtimeExecutionModifier*
      block?
    ;

runtimeExecutionTarget
    : expression
    ;

runtimeExecutionArguments
    : '('
      argumentList?
      ')'
    ;

runtimeExecutionModifier
    : runtimeExecutionPolicy
    | runtimeRequirement
    | runtimeCapabilityRequirement
    | runtimeResourceRequirement
    ;

runtimeStartStatement
    : 'start'
      runtimeExecutionTarget?
      runtimeExecutionModifier*
    ;

runtimeStopStatement
    : 'stop'
      runtimeExecutionTarget?
      runtimeTerminationReason?
    ;

runtimeCancelStatement
    : 'cancel'
      runtimeExecutionTarget?
      runtimeCancellationReason?
    ;

runtimePauseStatement
    : 'pause'
      runtimeExecutionTarget?
    ;

runtimeResumeStatement
    : 'resume'
      runtimeExecutionTarget?
    ;

runtimeYieldStatement
    : 'yield'
      runtimeYieldValue?
    ;

runtimeYieldValue
    : expression
    ;

runtimeWaitStatement
    : 'wait'
      runtimeWaitTarget
      runtimeTimeout?
    ;

runtimeWaitTarget
    : expression
    ;

runtimeTimeout
    : 'timeout'
      expression
    ;

/* ====================================================================== */
/* TASK / ACTIVITY LIFECYCLE                                               */
/* ====================================================================== */

runtimeSpawnStatement
    : 'spawn'
      runtimeSpawnTarget
      runtimeSpawnOptions*
    ;

runtimeSpawnTarget
    : expression
    ;

runtimeSpawnOptions
    : runtimeExecutionPolicy
    | runtimePlacementPolicy
    | runtimeResourceRequirement
    | runtimeCapabilityRequirement
    | runtimeLifecyclePolicy
    ;

runtimeJoinStatement
    : 'join'
      runtimeJoinTarget
      runtimeJoinOptions*
    ;

runtimeJoinTarget
    : expression
    ;

runtimeJoinOptions
    : 'all'
    | 'any'
    | 'first'
    | 'successful'
    | runtimeTimeout
    ;

/* ====================================================================== */
/* SCHEDULING INTENT                                                       */
/* ====================================================================== */

/*
 * Scheduling syntax expresses intent.
 *
 * It must never encode an implementation-specific resource count.
 */
runtimeSchedulingPolicy
    : 'schedule'
      runtimeSchedulingMode?
      runtimeSchedulingClause*
    ;

runtimeSchedulingMode
    : 'automatic'
    | 'adaptive'
    | 'dynamic'
    | 'static'
    | 'deterministic'
    | 'priority'
    | 'deadline'
    | 'throughput'
    | 'latency'
    | 'fair'
    | identifier
    ;

runtimeSchedulingClause
    : runtimePriorityClause
    | runtimeDeadlineClause
    | runtimeAffinityClause
    | runtimeDependencyClause
    | runtimeOrderingClause
    | runtimeConcurrencyClause
    | runtimePreemptionClause
    | runtimeBackpressureClause
    | runtimeAttribute
    ;

runtimePriorityClause
    : 'priority'
      expression
    ;

runtimeDeadlineClause
    : 'deadline'
      expression
    ;

runtimeAffinityClause
    : 'affinity'
      expression
    ;

runtimeDependencyClause
    : 'depends'
      'on'
      expressionList
    ;

runtimeOrderingClause
    : 'ordered'
    | 'unordered'
    | 'deterministic'
    ;

runtimeConcurrencyClause
    : 'concurrency'
      expression
    ;

runtimePreemptionClause
    : 'preemptible'
    | 'nonpreemptible'
    ;

runtimeBackpressureClause
    : 'backpressure'
      runtimeBackpressureMode?
    ;

runtimeBackpressureMode
    : 'block'
    | 'drop'
    | 'buffer'
    | 'adapt'
    | identifier
    ;

/*
 * Generic execution policy.
 *
 * Unknown/future policy names remain representable without requiring
 * a grammar release for every runtime implementation.
 */
runtimeExecutionPolicy
    : 'policy'
      identifier
      runtimePolicyArguments?
    ;

runtimePolicyArguments
    : '('
      argumentList?
      ')'
    ;

/* ====================================================================== */
/* PLACEMENT                                                               */
/* ====================================================================== */

runtimePlacementPolicy
    : 'place'
      runtimePlacementTarget?
      runtimePlacementClause*
    ;

runtimePlacementTarget
    : expression
    ;

runtimePlacementClause
    : runtimeCapabilityRequirement
    | runtimeResourceRequirement
    | runtimeTopologyRequirement
    | runtimeLocalityRequirement
    | runtimeMobilityRequirement
    | runtimeAttribute
    ;

runtimeTopologyRequirement
    : 'topology'
      expression
    ;

runtimeLocalityRequirement
    : 'locality'
      expression
    ;

runtimeMobilityRequirement
    : 'mobility'
      (
          'fixed'
        | 'movable'
        | 'migratable'
        | 'adaptive'
        | identifier
      )
    ;

/*
 * There is intentionally no grammar such as:
 *
 *     cpu(0)
 *     gpu(3)
 *     node(7)
 *     qubit(12)
 *
 * as a universal execution model.
 *
 * Physical placement is a downstream realization concern.
 */

/* ====================================================================== */
/* REQUIREMENTS                                                            */
/* ====================================================================== */

runtimeRequirementDeclaration
    : 'requires'
      runtimeRequirement
    ;

runtimeRequirement
    : runtimeCapabilityRequirement
    | runtimeResourceRequirement
    | runtimeTopologyRequirement
    | runtimePerformanceRequirement
    | runtimeReliabilityRequirement
    | runtimeTimingRequirement
    | runtimeSecurityRequirement
    | runtimePersistenceRequirement
    | runtimePortabilityRequirement
    | runtimeAttribute
    ;

runtimeCapabilityRequirement
    : 'capability'
      '('
      expression
      ')'
    ;

runtimeResourceRequirement
    : 'resource'
      '('
      expression
      ')'
    ;

runtimePerformanceRequirement
    : 'performance'
      '('
      expression
      ')'
    ;

runtimeReliabilityRequirement
    : 'reliability'
      '('
      expression
      ')'
    ;

runtimeTimingRequirement
    : 'timing'
      '('
      expression
      ')'
    ;

runtimeSecurityRequirement
    : 'security'
      '('
      expression
      ')'
    ;

runtimePersistenceRequirement
    : 'persistence'
      '('
      expression
      ')'
    ;

runtimePortabilityRequirement
    : 'portability'
      '('
      expression
      ')'
    ;

/* ====================================================================== */
/* CAPABILITIES                                                            */
/* ====================================================================== */

runtimeCapabilityDeclaration
    : 'capability'
      identifier
      runtimeCapabilityBody?
    ;

runtimeCapabilityBody
    : '{'
      runtimeCapabilityMember*
      '}'
    ;

runtimeCapabilityMember
    : runtimeCapabilityProvides
    | runtimeCapabilityRequires
    | runtimeAttribute
    ;

runtimeCapabilityProvides
    : 'provides'
      expressionList
    ;

runtimeCapabilityRequires
    : 'requires'
      expressionList
    ;

/* ====================================================================== */
/* RESOURCE CONTRACTS                                                      */
/* ====================================================================== */

runtimeResourceContract
    : 'resource'
      identifier
      runtimeResourceContractBody?
    ;

runtimeResourceContractBody
    : '{'
      runtimeResourceClause*
      '}'
    ;

runtimeResourceClause
    : 'requires' expression
    | 'prefers' expression
    | 'allows' expression
    | 'forbids' expression
    | runtimeAttribute
    ;

/*
 * Requirement vs preference is intentionally preserved.
 *
 * Requirement:
 *     requires capability("quantum.measurement")
 *
 * Preference:
 *     prefers capability("gpu.compute")
 *
 * Neither statement selects a physical device.
 */

/* ====================================================================== */
/* LIFECYCLE                                                               */
/* ====================================================================== */

runtimeLifecycleDeclaration
    : 'lifecycle'
      identifier?
      runtimeLifecycleBody
    ;

runtimeLifecycleBody
    : '{'
      runtimeLifecycleMember*
      '}'
    ;

runtimeLifecycleMember
    : runtimeLifecycleHook
    | runtimeLifecyclePolicy
    | runtimeLifecycleTransition
    | runtimeAttribute
    ;

runtimeLifecycleHook
    : runtimeLifecycleEvent
      runtimeHookBody
    ;

runtimeLifecycleEvent
    : 'on'
      (
          'create'
        | 'initialize'
        | 'start'
        | 'ready'
        | 'suspend'
        | 'resume'
        | 'checkpoint'
        | 'restore'
        | 'degrade'
        | 'recover'
        | 'quarantine'
        | 'retire'
        | 'stop'
        | 'cancel'
        | 'fail'
        | 'complete'
        | 'destroy'
        | identifier
      )
    ;

runtimeHookBody
    : block
    ;

runtimeLifecyclePolicy
    : 'lifecycle'
      identifier
      runtimePolicyArguments?
    ;

runtimeLifecycleTransition
    : 'transition'
      identifier
      '->'
      identifier
    ;

/* ====================================================================== */
/* STATE                                                                   */
/* ====================================================================== */

runtimeState
    : 'state'
      identifier
      runtimeStateBody?
    ;

runtimeStateBody
    : '{'
      runtimeStateMember*
      '}'
    ;

runtimeStateMember
    : runtimeStateTransition
    | runtimeAttribute
    ;

runtimeStateTransition
    : 'on'
      identifier
      '->'
      identifier
    ;

/*
 * Resilience state vocabulary.
 *
 * These are semantic states, not hardware limits.
 *
 * Unknown
 * Healthy
 * Degraded
 * Unstable
 * Unavailable
 * Recovering
 * Quarantined
 * Retired
 */
runtimeResilienceState
    : 'Unknown'
    | 'Healthy'
    | 'Degraded'
    | 'Unstable'
    | 'Unavailable'
    | 'Recovering'
    | 'Quarantined'
    | 'Retired'
    | identifier
    ;

/*
 * Resilience outcomes.
 *
 * ACCEPT
 * DEGRADED_ACCEPT
 * RETRY
 * RECOVER
 * ESCALATE
 * REJECT
 */
runtimeResilienceOutcome
    : 'ACCEPT'
    | 'DEGRADED_ACCEPT'
    | 'RETRY'
    | 'RECOVER'
    | 'ESCALATE'
    | 'REJECT'
    | identifier
    ;

/* ====================================================================== */
/* RESILIENCE                                                              */
/* ====================================================================== */

runtimeResiliencePolicy
    : 'resilience'
      runtimeResilienceClause*
    ;

runtimeResilienceClause
    : 'state'
      runtimeResilienceState
    | 'outcome'
      runtimeResilienceOutcome
    | 'retry'
      runtimeRetryPolicy?
    | 'recover'
      runtimeRecoveryPolicy?
    | 'escalate'
      expression?
    | 'quarantine'
      expression?
    | 'checkpoint'
      expression?
    | 'timeout'
      expression
    | 'backoff'
      expression
    | 'on'
      runtimeResilienceEvent
      runtimeResilienceAction
    | runtimeAttribute
    ;

runtimeResilienceEvent
    : 'failure'
    | 'timeout'
    | 'degradation'
    | 'unavailability'
    | 'instability'
    | 'corruption'
    | 'disconnect'
    | 'resource_exhaustion'
    | identifier
    ;

runtimeResilienceAction
    : '->'
      runtimeResilienceOutcome
    | runtimeHookBody
    ;

runtimeRetryPolicy
    : runtimeRetryClause*
    ;

runtimeRetryClause
    : 'count'
      expression
    | 'until'
      expression
    | 'backoff'
      expression
    | 'jitter'
      expression
    | 'when'
      expression
    | runtimeAttribute
    ;

runtimeRecoveryPolicy
    : runtimeRecoveryClause*
    ;

runtimeRecoveryClause
    : 'strategy'
      expression
    | 'from'
      expression
    | 'checkpoint'
      expression
    | 'timeout'
      expression
    | 'when'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* CHECKPOINTING                                                           */
/* ====================================================================== */

runtimeCheckpointStatement
    : 'checkpoint'
      runtimeCheckpointTarget?
      runtimeCheckpointOptions*
    ;

runtimeCheckpointTarget
    : expression
    ;

runtimeCheckpointOptions
    : 'as'
      identifier
    | 'persistent'
    | 'ephemeral'
    | 'incremental'
    | 'consistent'
    | 'best_effort'
    | 'on_failure'
    | 'on_completion'
    | 'when'
      expression
    | runtimeAttribute
    ;

runtimeCheckpointConfiguration
    : 'checkpointing'
      runtimeCheckpointClause*
    ;

runtimeCheckpointClause
    : 'enabled'
    | 'disabled'
    | 'persistent'
    | 'ephemeral'
    | 'incremental'
    | 'consistent'
    | 'interval'
      expression
    | 'condition'
      expression
    | 'retention'
      expression
    | 'storage'
      expression
    | runtimeAttribute
    ;

runtimeRestoreStatement
    : 'restore'
      runtimeRestoreSource
      runtimeRestoreOptions*
    ;

runtimeRestoreSource
    : expression
    ;

runtimeRestoreOptions
    : 'validate'
    | 'replay'
    | 'resume'
    | 'recover'
    | 'as'
      identifier
    | runtimeAttribute
    ;

/* ====================================================================== */
/* OBSERVABILITY                                                           */
/* ====================================================================== */

runtimeObservationConfiguration
    : 'observe'
      runtimeObservationClause*
    ;

runtimeObservationClause
    : 'metrics'
    | 'logs'
    | 'events'
    | 'state'
    | 'resources'
    | 'performance'
    | 'health'
    | 'errors'
    | 'traces'
    | 'profiles'
    | 'enabled'
    | 'disabled'
    | 'sampling'
      expression
    | 'filter'
      expression
    | 'sink'
      expression
    | runtimeAttribute
    ;

runtimeObserveStatement
    : 'observe'
      runtimeObservationTarget?
      runtimeObservationOptions*
    ;

runtimeObservationTarget
    : expression
    ;

runtimeObservationOptions
    : 'metrics'
    | 'logs'
    | 'events'
    | 'state'
    | 'resources'
    | 'performance'
    | 'health'
    | 'errors'
    | 'continuous'
    | 'snapshot'
    | 'sampling'
      expression
    | 'where'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* TRACING                                                                 */
/* ====================================================================== */

runtimeTraceStatement
    : 'trace'
      runtimeTraceTarget?
      runtimeTraceOptions*
    ;

runtimeTraceTarget
    : expression
    ;

runtimeTraceOptions
    : 'start'
    | 'stop'
    | 'span'
    | 'event'
    | 'sample'
      expression
    | 'where'
      expression
    | 'sink'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* PROFILING                                                               */
/* ====================================================================== */

runtimeProfileStatement
    : 'profile'
      runtimeProfileTarget?
      runtimeProfileOptions*
    ;

runtimeProfileTarget
    : expression
    ;

runtimeProfileOptions
    : 'start'
    | 'stop'
    | 'sample'
      expression
    | 'metrics'
      expressionList
    | 'where'
      expression
    | 'sink'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* SIGNALING                                                               */
/* ====================================================================== */

runtimeSignalStatement
    : 'signal'
      runtimeSignalTarget
      runtimeSignalPayload?
    ;

runtimeSignalTarget
    : expression
    ;

runtimeSignalPayload
    : '('
      argumentList?
      ')'
    ;

/* ====================================================================== */
/* TERMINATION                                                             */
/* ====================================================================== */

runtimeTerminationReason
    : 'because'
      expression
    ;

runtimeCancellationReason
    : 'because'
      expression
    ;

/* ====================================================================== */
/* PORTABILITY                                                             */
/* ====================================================================== */

/*
 * Runtime portability contract.
 *
 * The program may describe what it requires without naming a physical
 * realization.
 */
runtimePortabilityPolicy
    : 'portable'
      runtimePortabilityClause*
    ;

runtimePortabilityClause
    : 'across'
      expression
    | 'requires'
      expression
    | 'forbids'
      expression
    | 'preserve'
      expressionList
    | 'allow'
      expressionList
    | 'when'
      expression
    | runtimeAttribute
    ;

/*
 * Explicit target binding is represented as an intent/constraint and is
 * therefore still semantically distinguishable from physical realization.
 */
runtimeTargetConstraint
    : 'target'
      expression
      runtimeTargetConstraintClause*
    ;

runtimeTargetConstraintClause
    : 'requires'
      expression
    | 'prefers'
      expression
    | 'allows'
      expression
    | 'forbids'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* RUNTIME POLICIES                                                        */
/* ====================================================================== */

runtimePolicyDeclaration
    : 'policy'
      identifier
      runtimePolicyBody
    ;

runtimePolicyBody
    : '{'
      runtimePolicyMember*
      '}'
    ;

runtimePolicyMember
    : runtimeSchedulingPolicy
    | runtimePlacementPolicy
    | runtimeResiliencePolicy
    | runtimePortabilityPolicy
    | runtimeExecutionPolicy
    | runtimeLifecyclePolicy
    | runtimeCheckpointConfiguration
    | runtimeObservationConfiguration
    | runtimeAttribute
    ;

/* ====================================================================== */
/* RESOURCE-AWARE EXECUTION                                                */
/* ====================================================================== */

runtimeResourceAwareExecution
    : 'execute'
      runtimeResourceContext?
      runtimeExecutionTarget?
      runtimeExecutionModifier*
    ;

runtimeResourceContext
    : 'with'
      '{'
      runtimeResourceBinding*
      '}'
    ;

runtimeResourceBinding
    : identifier
      '='
      expression
    ;

/*
 * Resource availability is queried/negotiated semantically.
 *
 * It does not become a parser-level maximum.
 */
runtimeAvailabilityRequirement
    : 'available'
      expression
    ;

runtimeNegotiation
    : 'negotiate'
      runtimeNegotiationClause*
    ;

runtimeNegotiationClause
    : 'capabilities'
      expressionList
    | 'resources'
      expressionList
    | 'constraints'
      expressionList
    | 'fallback'
      expression
    | 'adapt'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* DYNAMIC ADAPTATION                                                      */
/* ====================================================================== */

runtimeAdaptationPolicy
    : 'adapt'
      runtimeAdaptationClause*
    ;

runtimeAdaptationClause
    : 'when'
      expression
    | 'to'
      expression
    | 'preserve'
      expressionList
    | 'relax'
      expressionList
    | 'tighten'
      expressionList
    | 'migrate'
      expression
    | 'reconfigure'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* EXECUTION CONTEXT                                                       */
/* ====================================================================== */

runtimeContext
    : 'context'
      identifier?
      runtimeContextBody
    ;

runtimeContextBody
    : '{'
      runtimeContextMember*
      '}'
    ;

runtimeContextMember
    : runtimeContextValue
    | runtimeRequirement
    | runtimeCapabilityRequirement
    | runtimeResourceRequirement
    | runtimeSchedulingPolicy
    | runtimePlacementPolicy
    | runtimeResiliencePolicy
    | runtimeObservationConfiguration
    | runtimeCheckpointConfiguration
    | runtimeAttribute
    ;

runtimeContextValue
    : identifier
      '='
      expression
    ;

/* ====================================================================== */
/* ASYNCHRONOUS EXECUTION                                                  */
/* ====================================================================== */

runtimeAsyncExecution
    : 'async'
      runtimeAsyncTarget?
      runtimeAsyncOptions*
    ;

runtimeAsyncTarget
    : expression
    ;

runtimeAsyncOptions
    : runtimeSchedulingPolicy
    | runtimePlacementPolicy
    | runtimeResourceRequirement
    | runtimeCapabilityRequirement
    | runtimeResiliencePolicy
    | runtimeAttribute
    ;

/* ====================================================================== */
/* DETERMINISM                                                             */
/* ====================================================================== */

runtimeDeterminismPolicy
    : 'determinism'
      runtimeDeterminismMode
      runtimeDeterminismClause*
    ;

runtimeDeterminismMode
    : 'required'
    | 'preferred'
    | 'allowed'
    | 'forbidden'
    ;

runtimeDeterminismClause
    : 'ordering'
      (
          'stable'
        | 'explicit'
        | 'deterministic'
        | 'implementation_defined'
        | identifier
      )
    | 'replay'
      (
          'required'
        | 'preferred'
        | 'allowed'
        | 'forbidden'
      )
    | 'seed'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* SECURITY / ISOLATION                                                    */
/* ====================================================================== */

runtimeIsolationPolicy
    : 'isolation'
      runtimeIsolationClause*
    ;

runtimeIsolationClause
    : 'process'
    | 'memory'
    | 'device'
    | 'network'
    | 'capability'
    | 'resource'
    | 'strict'
    | 'relaxed'
    | 'required'
    | 'preferred'
    | runtimeAttribute
    ;

/* ====================================================================== */
/* DATA CONSISTENCY                                                        */
/* ====================================================================== */

runtimeConsistencyPolicy
    : 'consistency'
      runtimeConsistencyMode
      runtimeConsistencyClause*
    ;

runtimeConsistencyMode
    : 'strong'
    | 'eventual'
    | 'causal'
    | 'session'
    | 'snapshot'
    | 'deterministic'
    | identifier
    ;

runtimeConsistencyClause
    : 'scope'
      expression
    | 'boundary'
      expression
    | 'timeout'
      expression
    | 'recovery'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* RESOURCE EXHAUSTION                                                     */
/* ====================================================================== */

runtimeExhaustionPolicy
    : 'on_resource_exhaustion'
      runtimeExhaustionAction
    ;

runtimeExhaustionAction
    : 'wait'
    | 'retry'
    | 'adapt'
    | 'migrate'
    | 'degrade'
    | 'recover'
    | 'escalate'
    | 'reject'
    | 'abort'
    | identifier
    ;

/* ====================================================================== */
/* FALLBACK                                                                */
/* ====================================================================== */

runtimeFallbackPolicy
    : 'fallback'
      runtimeFallbackClause*
    ;

runtimeFallbackClause
    : 'to'
      expression
    | 'when'
      expression
    | 'preserve'
      expressionList
    | 'relax'
      expressionList
    | 'requires'
      expressionList
    | runtimeAttribute
    ;

/* ====================================================================== */
/* RESOURCE / CAPABILITY DISCOVERY                                         */
/* ====================================================================== */

runtimeDiscovery
    : 'discover'
      runtimeDiscoveryTarget?
      runtimeDiscoveryClause*
    ;

runtimeDiscoveryTarget
    : expression
    ;

runtimeDiscoveryClause
    : 'capabilities'
    | 'resources'
    | 'topology'
    | 'health'
    | 'performance'
    | 'availability'
    | 'security'
    | 'version'
    | 'compatibility'
    | 'where'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* VERSION / COMPATIBILITY                                                 */
/* ====================================================================== */

runtimeCompatibilityPolicy
    : 'compatibility'
      runtimeCompatibilityClause*
    ;

runtimeCompatibilityClause
    : 'requires'
      expression
    | 'supports'
      expressionList
    | 'fallback'
      expression
    | 'version'
      expression
    | 'feature'
      expression
    | runtimeAttribute
    ;

/* ====================================================================== */
/* RUNTIME INVOCATION                                                      */
/* ====================================================================== */

runtimeInvocation
    : 'runtime'
      '.'
      identifier
      '('
      argumentList?
      ')'
    ;

/*
 * Generic invocation is deliberately available for runtime extensions
 * while semantic validation remains responsible for determining whether
 * the operation exists and is permitted.
 */

/* ====================================================================== */
/* COMMON EXPRESSION HOOKS                                                 */
/* ====================================================================== */

/*
 * These rules intentionally reference the canonical Zamani expression,
 * type, block, identifier, and argument rules.
 *
 * They must NOT be redefined here.
 *
 * Expected canonical owners:
 *
 *   identifier
 *   expression
 *   expressionList
 *   argumentList
 *   typeExpression
 *   block
 *
 * are owned by core/, expressions/, types/, and statements/.
 *
 * Zamani.g4 composes those rules into this grammar.
 */