/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* File:
* grammar/execution/runtime.g4
* 
* Grammar:
* Runtime
* 
* Status:
* Production runtime-intent grammar component
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines SOURCE-LEVEL RUNTIME INTENT.
* 
* It describes:
* 
* - runtime environments;
* - runtime entry points;
* - execution lifecycle;
* - runtime control operations;
* - task/activity lifecycle;
* - runtime policies;
* - runtime requirements;
* - runtime capability requirements;
* - runtime resource requirements;
* - runtime adaptation;
* - runtime discovery intent;
* - runtime resilience policy;
* - checkpoint/restore intent;
* - observability;
* - tracing;
* - profiling;
* - determinism policy;
* - isolation policy;
* - consistency policy;
* - fallback policy;
* - resource-exhaustion policy;
* - portability policy;
* - runtime compatibility intent.
* 
* This grammar describes WHAT a program requests or permits.
* 
* It does NOT describe HOW the request is realized.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* ZamaniParser
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      +--> type/effect analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> portability analysis
*      +--> execution-intent validation
*      |
*      v
* canonical semantic model
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      |
*      v
* optimization / lowering
*      |
*      +--> scheduling
*      +--> placement
*      +--> routing
*      +--> resilience
*      +--> QEC
*      +--> ZQN
*      +--> HAL
*      |
*      v
* target realization
*      |
*      v
* runtime
* 
* Runtime grammar MUST NOT bypass this pipeline.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - runtime-specific source syntax;
* - runtime lifecycle intent;
* - runtime execution-control intent;
* - runtime policy composition;
* - runtime observability intent;
* - runtime resilience policy syntax;
* - runtime adaptation intent;
* - runtime portability policy syntax.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical token definitions;
* - identifiers;
* - general expressions;
* - general types;
* - general blocks;
* - general declarations;
* - general statements;
* - resource-language authority;
* - hardware-language authority;
* - scheduling implementation;
* - placement implementation;
* - dispatch implementation;
* - deployment implementation;
* - routing;
* - optimization;
* - quantum::ir;
* - QEC;
* - ZQN;
* - calibration;
* - HAL;
* - vendor APIs;
* - runtime implementation.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* The grammar consumes the canonical lexer:
* 
* tokenVocab = ZamaniLexer
* 
* It imports only foundational parser grammars needed by its public syntax:
* 
* Core
* Types
* Expressions
* 
* Runtime MUST NOT import:
* 
* Execution
* Scheduling
* Placement
* Dispatch
* Deployment
* 
* merely to reuse their concepts.
* 
* This prevents a circular dependency:
* 
* Execution -> Runtime
*      X
* Runtime -> Execution
* 
* The neighboring grammars retain their own ownership.
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* Canonical composition is:
* 
* grammar/antlr/ZamaniParser.g4
*             |
*             v
*         Execution
*             |
*             v
*          Runtime
* 
* Therefore "execution.g4" should import Runtime and expose:
* 
* runtimeDeclaration
* runtimeStatement
* runtimeExpression
* 
* through its execution-domain composition where appropriate.
* 
* The canonical root parser already imports "Execution"; this means Runtime
* becomes reachable without adding Runtime directly to the root parser.
* 
* DO NOT create a second root parser for Runtime.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar produces syntax that semantically maps to generic,
* domain-neutral execution-intent nodes.
* 
* Conceptual mappings:
* 
* runtimeEnvironmentDeclaration
*     -> RuntimeEnvironmentDecl
* 
* runtimeEntryPointDeclaration
*     -> RuntimeEntryPointDecl
* 
* runtimeStatement
*     -> RuntimeOperation / RuntimeControlIntent
* 
* runtimePolicyDeclaration
*     -> RuntimePolicyDecl
* 
* runtimeRequirement
*     -> Requirement / CapabilityRequirement / ResourceRequirement
* 
* runtimeResiliencePolicy
*     -> ResiliencePolicy
* 
* runtimeCheckpointConfiguration
*     -> CheckpointPolicy
* 
* runtimeObservationConfiguration
*     -> ObservationPolicy
* 
* runtimeDeterminismPolicy
*     -> DeterminismPolicy
* 
* runtimePortabilityPolicy
*     -> PortabilityPolicy
* 
* The exact Rust AST type is owned by the frontend AST/semantic layer.
* 
* This file MUST NOT invent a runtime-specific IR.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Runtime syntax is declarative intent.
* 
* Semantic analysis is responsible for determining:
* 
* - whether the referenced computation exists;
* - whether the requested capability exists;
* - whether resource requirements can be satisfied;
* - whether requirements conflict;
* - whether a policy is legal for the computation;
* - whether a requested transition is valid;
* - whether a requested execution mode is compatible with effects;
* - whether determinism is actually achievable;
* - whether checkpointing is supported;
* - whether recovery is semantically valid;
* - whether portability requirements can be met.
* 
* Parsing MUST NOT perform any of those operations.
* 
* ============================================================================
* RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* These concepts remain distinct:
* 
* requirement
* capability
* resource
* constraint
* preference
* hint
* 
* Examples:
* 
* requires capability("quantum.measurement")
* 
* requires resource(qubits >= n)
* 
* requires performance(latency < limit)
* 
* prefer capability("gpu.compute")
* 
* hint placement_strategy
* 
* None of these selects a physical device.
* 
* Physical realization belongs downstream.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* This grammar contains NO universal machine-size limits.
* 
* It MUST NOT impose:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ASICS
* MAX_QPUS
* MAX_NODES
* MAX_DEVICES
* MAX_MEMORY
* MAX_STORAGE
* MAX_REGISTER_WIDTH
* MAX_VECTOR_WIDTH
* MAX_TENSOR_RANK
* MAX_NETWORK_SIZE
* MAX_TIMELINES
* MAX_TASKS
* MAX_PROCESSES
* 
* Repetition and nesting are intentionally unbounded by language semantics.
* 
* Actual finite limits arise only from:
* 
* - available resources;
* - target capabilities;
* - compiler implementation resources;
* - runtime implementation resources;
* - operating-environment limits;
* - explicitly declared program requirements.
* 
* Those are not grammar limits.
* 
* ============================================================================
* TARGET INDEPENDENCE
* ============================================================================
* 
* Runtime syntax MUST NOT encode universal physical identities such as:
* 
* cpu(0)
* gpu(3)
* qpu(2)
* node(7)
* fpga(4)
* qubit(12)
* 
* Physical identifiers MAY exist in a separately defined target-specific
* language/interop surface, but they are not the portable runtime model.
* 
* The portable runtime model expresses:
* 
* capability
* requirement
* resource
* topology requirement
* locality
* migration policy
* placement intent
* 
* The compiler/runtime determines realization.
* 
* ============================================================================
* LEXICAL CONTRACT
* ============================================================================
* 
* The canonical lexer is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This file therefore does NOT define lexer rules.
* 
* Existing reserved lexical tokens such as:
* 
* REQUIRES
* RESOURCE
* CAPABILITY
* PERFORMANCE
* RELIABILITY
* RESILIENCE
* PORTABILITY
* TARGET
* AVAILABILITY
* OBSERVE
* PROFILE
* WITH
* AS
* WHEN
* WHERE
* SPAWN
* AWAIT
* YIELD
* PARALLEL
* 
* are consumed through the canonical token vocabulary where useful.
* 
* Runtime-specific vocabulary that is not currently reserved remains
* contextual source syntax represented by IDENTIFIER.
* 
* This intentionally avoids silently modifying the lexical authority from
* this leaf grammar.
* 
* ============================================================================
* IMPORTANT: NO STRING-LITERAL TOKEN INVENTION
* ============================================================================
* 
* This is a parser grammar, not a combined grammar.
* 
* It therefore does NOT rely on parser string literals such as:
* 
* 'runtime'
* 'execute'
* 'checkpoint'
* 
* because those spellings are not universally guaranteed to have corresponding
* token types in ZamaniLexer.
* 
* Contextual runtime words are represented by identifier and validated by
* semantic analysis until/if they are promoted to reserved lexer keywords.
* 
* This keeps this file independently complete against the existing lexer.
* 
* ============================================================================
  */

/*

* ============================================================================
* GRAMMAR DECLARATION
* ============================================================================
  */

parser grammar Runtime;

options {
tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;

/*

* ============================================================================
* 1. PUBLIC RUNTIME COMPOSITION
* ============================================================================
* 
* These are the stable entry points consumed by Execution.
* ============================================================================
  */

runtimeDeclaration
: runtimeEnvironmentDeclaration
| runtimeEntryPointDeclaration
| runtimePolicyDeclaration
| runtimeRequirementDeclaration
| runtimeCapabilityDeclaration
;

runtimeStatement
: runtimeExecuteStatement
| runtimeStartStatement
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
| runtimeAdaptationStatement
| runtimeDiscoveryStatement
;

runtimeExpression
: runtimeInvocation
| runtimeContextExpression
;

/*

* ============================================================================
* 2. CONTEXTUAL RUNTIME WORDS
* ============================================================================
* 
* The current canonical lexer does not reserve every runtime-specific word.
* 
* These rules deliberately use IDENTIFIER rather than inventing new lexical
* tokens in this file.
* 
* Semantic validation MUST check the exact contextual spelling where required.
* 
* This permits forward-compatible runtime syntax without creating another
* lexer authority.
* ============================================================================
  */

runtimeWord
: identifier
;

runtimeName
: identifier
;

/*

* ============================================================================
* 3. RUNTIME ENVIRONMENT
* ============================================================================
  */

runtimeEnvironmentDeclaration
: runtimeWord
identifier
runtimeEnvironmentBody
;

runtimeEnvironmentBody
: LBRACE
runtimeEnvironmentMember*
RBRACE
;

runtimeEnvironmentMember
: runtimeEnvironmentOption
| runtimeRequirement
| runtimeCapabilityRequirement
| runtimeResourceRequirement
| runtimePolicyReference
| runtimeLifecycleHook
| runtimeCheckpointConfiguration
| runtimeRecoveryConfiguration
| runtimeObservationConfiguration
| runtimeAttribute
;

runtimeEnvironmentOption
: identifier
ASSIGN
expression
;

runtimeAttribute
: identifier
(ASSIGN expression)?
;

/*

* ============================================================================
* 4. ENTRY POINTS
* ============================================================================
  */

runtimeEntryPointDeclaration
: runtimeWord
runtimeWord
identifier
runtimeEntryPointSignature?
runtimeEntryPointBody?
;

runtimeEntryPointSignature
: LPAREN
runtimeParameterList?
RPAREN
runtimeReturnClause?
;

runtimeParameterList
: runtimeParameter
(COMMA runtimeParameter)*
;

runtimeParameter
: identifier
(COLON typeExpression)?
(ASSIGN expression)?
;

runtimeReturnClause
: ARROW
typeExpression
;

runtimeEntryPointBody
: block
;

/*

* ============================================================================
* 5. EXECUTION
* ============================================================================
  */

runtimeExecuteStatement
: runtimeWord
runtimeExecutionTarget?
runtimeExecutionArguments?
runtimeExecutionModifier*
;

runtimeExecutionTarget
: expression
;

runtimeExecutionArguments
: LPAREN
argumentList?
RPAREN
;

runtimeExecutionModifier
: runtimeExecutionPolicy
| runtimeRequirement
| runtimeCapabilityRequirement
| runtimeResourceRequirement
;

runtimeStartStatement
: runtimeWord
runtimeExecutionTarget?
runtimeExecutionModifier*
;

runtimeStopStatement
: runtimeWord
runtimeExecutionTarget?
runtimeTerminationReason?
;

runtimeCancelStatement
: runtimeWord
runtimeExecutionTarget?
runtimeCancellationReason?
;

runtimePauseStatement
: runtimeWord
runtimeExecutionTarget?
;

runtimeResumeStatement
: runtimeWord
runtimeExecutionTarget?
;

runtimeWaitStatement
: runtimeWord
runtimeWaitTarget
runtimeTimeout?
;

runtimeWaitTarget
: expression
;

runtimeTimeout
: runtimeWord
expression
;

runtimeYieldStatement
: YIELD
runtimeYieldValue?
;

runtimeYieldValue
: expression
;

/*

* ============================================================================
* 6. ACTIVITY / TASK LIFECYCLE
* ============================================================================
  */

runtimeSpawnStatement
: SPAWN
runtimeSpawnTarget
runtimeSpawnOption*
;

runtimeSpawnTarget
: expression
;

runtimeSpawnOption
: runtimeExecutionPolicy
| runtimePlacementIntent
| runtimeResourceRequirement
| runtimeCapabilityRequirement
| runtimeLifecyclePolicyReference
;

runtimeJoinStatement
: runtimeWord
runtimeJoinTarget
runtimeJoinOption*
;

runtimeJoinTarget
: expression
;

runtimeJoinOption
: runtimeJoinMode
| runtimeTimeout
;

runtimeJoinMode
: runtimeWord
;

/*

* ============================================================================
* 7. SCHEDULING / PLACEMENT REFERENCES
* ============================================================================
* 
* Scheduling and placement remain owned by:
* 
* grammar/execution/scheduling.g4
* grammar/execution/placement.g4
* 
* This file does NOT redefine their full grammars.
* 
* Runtime carries only references/intents that can be semantically attached
* to runtime operations.
* ============================================================================
  */

runtimeSchedulingReference
: runtimeWord
runtimeName?
runtimeArgumentList?
;

runtimePlacementIntent
: runtimeWord
runtimePlacementClause*
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
: TARGET
expression
;

runtimeLocalityRequirement
: runtimeWord
expression
;

runtimeMobilityRequirement
: runtimeWord
runtimeName?
;

/*

* ============================================================================
* 8. REQUIREMENTS
* ============================================================================
  */

runtimeRequirementDeclaration
: REQUIRES
runtimeRequirement
;

runtimeRequirement
: runtimeCapabilityRequirement
| runtimeResourceRequirement
| runtimePerformanceRequirement
| runtimeReliabilityRequirement
| runtimeTimingRequirement
| runtimeSecurityRequirement
| runtimePersistenceRequirement
| runtimePortabilityRequirement
| runtimeTopologyRequirement
| runtimeAttribute
;

runtimeCapabilityRequirement
: CAPABILITY
LPAREN
expression
RPAREN
;

runtimeResourceRequirement
: RESOURCE
LPAREN
expression
RPAREN
;

runtimePerformanceRequirement
: PERFORMANCE
LPAREN
expression
RPAREN
;

runtimeReliabilityRequirement
: RELIABILITY
LPAREN
expression
RPAREN
;

runtimeTimingRequirement
: runtimeWord
LPAREN
expression
RPAREN
;

runtimeSecurityRequirement
: runtimeWord
LPAREN
expression
RPAREN
;

runtimePersistenceRequirement
: runtimeWord
LPAREN
expression
RPAREN
;

runtimePortabilityRequirement
: PORTABILITY
LPAREN
expression
RPAREN
;

/*

* ============================================================================
* 9. CAPABILITY DECLARATIONS
* ============================================================================
  */

runtimeCapabilityDeclaration
: CAPABILITY
identifier
runtimeCapabilityBody?
;

runtimeCapabilityBody
: LBRACE
runtimeCapabilityMember*
RBRACE
;

runtimeCapabilityMember
: runtimeCapabilityProvides
| runtimeCapabilityRequires
| runtimeAttribute
;

runtimeCapabilityProvides
: runtimeWord
expressionList
;

runtimeCapabilityRequires
: REQUIRES
expressionList
;

/*

* ============================================================================
* 10. RESOURCE POLICY REFERENCES
* ============================================================================
* 
* Full resource semantics remain owned by grammar/resources/.
* ============================================================================
  */

runtimeResourceReference
: RESOURCE
LPAREN
expression
RPAREN
;

runtimePolicyReference
: runtimeWord
identifier
runtimeArgumentList?
;

runtimeLifecyclePolicyReference
: runtimeWord
identifier
runtimeArgumentList?
;

runtimeArgumentList
: LPAREN
argumentList?
RPAREN
;

/*

* ============================================================================
* 11. POLICY DECLARATION
* ============================================================================
  */

runtimePolicyDeclaration
: runtimeWord
identifier
runtimePolicyBody
;

runtimePolicyBody
: LBRACE
runtimePolicyMember*
RBRACE
;

runtimePolicyMember
: runtimeSchedulingReference
| runtimePlacementIntent
| runtimeResiliencePolicy
| runtimePortabilityPolicy
| runtimeExecutionPolicy
| runtimeDeterminismPolicy
| runtimeIsolationPolicy
| runtimeConsistencyPolicy
| runtimeFallbackPolicy
| runtimeAdaptationPolicy
| runtimeCheckpointConfiguration
| runtimeObservationConfiguration
| runtimeAttribute
;

runtimeExecutionPolicy
: runtimeWord
identifier
runtimeArgumentList?
;

/*

* ============================================================================
* 12. LIFECYCLE
* ============================================================================
  */

runtimeLifecycleDeclaration
: runtimeWord
identifier?
runtimeLifecycleBody
;

runtimeLifecycleBody
: LBRACE
runtimeLifecycleMember*
RBRACE
;

runtimeLifecycleMember
: runtimeLifecycleHook
| runtimeLifecyclePolicyReference
| runtimeLifecycleTransition
| runtimeAttribute
;

runtimeLifecycleHook
: runtimeLifecycleEvent
runtimeHookBody
;

runtimeLifecycleEvent
: runtimeWord
;

runtimeHookBody
: block
;

runtimeLifecycleTransition
: runtimeWord
identifier
ARROW
identifier
;

/*

* ============================================================================
* 13. RESILIENCE
* ============================================================================
* 
* Canonical state vocabulary requested by the Zamani execution/resilience
* architecture:
* 
* Unknown
* Healthy
* Degraded
* Unstable
* Unavailable
* Recovering
* Quarantined
* Retired
* 
* Canonical outcomes:
* 
* ACCEPT
* DEGRADED_ACCEPT
* RETRY
* RECOVER
* ESCALATE
* REJECT
* 
* These are semantic values. They do not describe machine capacity.
* ============================================================================
  */

runtimeResiliencePolicy
: RESILIENCE
runtimeResilienceClause*
;

runtimeResilienceClause
: runtimeResilienceStateClause
| runtimeResilienceOutcomeClause
| runtimeRetryPolicy
| runtimeRecoveryPolicy
| runtimeEscalationPolicy
| runtimeQuarantinePolicy
| runtimeCheckpointReference
| runtimeResilienceEventAction
| runtimeAttribute
;

runtimeResilienceStateClause
: runtimeWord
runtimeResilienceState
;

runtimeResilienceOutcomeClause
: runtimeWord
runtimeResilienceOutcome
;

runtimeResilienceState
: runtimeWord
;

runtimeResilienceOutcome
: runtimeWord
;

runtimeResilienceEventAction
: runtimeWord
runtimeWord
(
runtimeResilienceOutcome
| runtimeHookBody
)
;

runtimeRetryPolicy
: runtimeWord
runtimeRetryClause*
;

runtimeRetryClause
: runtimeWord expression
| runtimeAttribute
;

runtimeRecoveryPolicy
: runtimeWord
runtimeRecoveryClause*
;

runtimeRecoveryClause
: runtimeWord expression
| runtimeAttribute
;

runtimeEscalationPolicy
: runtimeWord
expression?
;

runtimeQuarantinePolicy
: runtimeWord
expression?
;

runtimeCheckpointReference
: runtimeWord
expression
;

/*

* ============================================================================
* 14. CHECKPOINT / RESTORE
* ============================================================================
  */

runtimeCheckpointStatement
: runtimeWord
runtimeCheckpointTarget?
runtimeCheckpointOption*
;

runtimeCheckpointTarget
: expression
;

runtimeCheckpointOption
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeCheckpointConfiguration
: runtimeWord
runtimeCheckpointClause*
;

runtimeCheckpointClause
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeRestoreStatement
: runtimeWord
runtimeRestoreSource
runtimeRestoreOption*
;

runtimeRestoreSource
: expression
;

runtimeRestoreOption
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeRecoveryConfiguration
: runtimeWord
runtimeRecoveryClause*
;

/*

* ============================================================================
* 15. OBSERVABILITY
* ============================================================================
  */

runtimeObservationConfiguration
: OBSERVE
runtimeObservationClause*
;

runtimeObservationClause
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeObserveStatement
: OBSERVE
runtimeObservationTarget?
runtimeObservationOption*
;

runtimeObservationTarget
: expression
;

runtimeObservationOption
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeTraceStatement
: runtimeWord
runtimeTraceTarget?
runtimeTraceOption*
;

runtimeTraceTarget
: expression
;

runtimeTraceOption
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

runtimeProfileStatement
: PROFILE
runtimeProfileTarget?
runtimeProfileOption*
;

runtimeProfileTarget
: expression
;

runtimeProfileOption
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

/*

* ============================================================================
* 16. SIGNALING
* ============================================================================
  */

runtimeSignalStatement
: runtimeWord
runtimeSignalTarget
runtimeSignalPayload?
;

runtimeSignalTarget
: expression
;

runtimeSignalPayload
: LPAREN
argumentList?
RPAREN
;

/*

* ============================================================================
* 17. TERMINATION
* ============================================================================
  */

runtimeTerminationReason
: runtimeWord
expression
;

runtimeCancellationReason
: runtimeWord
expression
;

/*

* ============================================================================
* 18. RESOURCE-AWARE EXECUTION
* ============================================================================
  */

runtimeResourceAwareExecution
: runtimeExecuteStatement
runtimeResourceContext?
;

runtimeResourceContext
: WITH
LBRACE
runtimeResourceBinding*
RBRACE
;

runtimeResourceBinding
: identifier
ASSIGN
expression
;

/*

* ============================================================================
* 19. NEGOTIATION / DISCOVERY
* ============================================================================
  */

runtimeNegotiation
: runtimeWord
runtimeNegotiationClause*
;

runtimeNegotiationClause
: runtimeWord
expressionList?
| runtimeAttribute
;

runtimeAvailabilityRequirement
: AVAILABILITY
expression
;

runtimeDiscoveryStatement
: runtimeWord
runtimeDiscoveryTarget?
runtimeDiscoveryClause*
;

runtimeDiscoveryTarget
: expression
;

runtimeDiscoveryClause
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

/*

* ============================================================================
* 20. DYNAMIC ADAPTATION
* ============================================================================
  */

runtimeAdaptationStatement
: runtimeWord
runtimeAdaptationClause*
;

runtimeAdaptationClause
: runtimeWord
expression?
| runtimeAttribute
;

/*

* ============================================================================
* 21. EXECUTION CONTEXT
* ============================================================================
* 
* The dedicated execution-context.g4 owns the complete execution-context
* structure.
* 
* Runtime only provides an expression-level reference so it does not create
* another context language.
* ============================================================================
  */

runtimeContextExpression
: runtimeWord
identifier?
LBRACE
runtimeContextMember*
RBRACE
;

runtimeContextMember
: runtimeContextValue
| runtimeRequirement
| runtimeCapabilityRequirement
| runtimeResourceRequirement
| runtimePolicyReference
| runtimeAttribute
;

runtimeContextValue
: identifier
ASSIGN
expression
;

/*

* ============================================================================
* 22. ASYNCHRONOUS RUNTIME EXECUTION
* ============================================================================
* 
* General async syntax remains owned by the concurrency grammar.
* 
* Runtime only represents runtime-level asynchronous execution intent.
* ============================================================================
  */

runtimeAsyncExecution
: ASYNC
runtimeAsyncTarget?
runtimeAsyncOption*
;

runtimeAsyncTarget
: expression
;

runtimeAsyncOption
: runtimeExecutionPolicy
| runtimePlacementIntent
| runtimeResourceRequirement
| runtimeCapabilityRequirement
| runtimeResiliencePolicy
| runtimeAttribute
;

/*

* ============================================================================
* 23. DETERMINISM
* ============================================================================
  */

runtimeDeterminismPolicy
: runtimeWord
runtimeDeterminismMode
runtimeDeterminismClause*
;

runtimeDeterminismMode
: runtimeWord
;

runtimeDeterminismClause
: runtimeWord
expression?
| runtimeAttribute
;

/*

* ============================================================================
* 24. ISOLATION
* ============================================================================
  */

runtimeIsolationPolicy
: runtimeWord
runtimeIsolationClause*
;

runtimeIsolationClause
: runtimeWord
| runtimeWord expression
| runtimeAttribute
;

/*

* ============================================================================
* 25. CONSISTENCY
* ============================================================================
  */

runtimeConsistencyPolicy
: runtimeWord
runtimeConsistencyMode
runtimeConsistencyClause*
;

runtimeConsistencyMode
: runtimeWord
;

runtimeConsistencyClause
: runtimeWord
expression?
| runtimeAttribute
;

/*

* ============================================================================
* 26. RESOURCE EXHAUSTION
* ============================================================================
  */

runtimeExhaustionPolicy
: runtimeWord
runtimeExhaustionAction
;

runtimeExhaustionAction
: runtimeWord
;

/*

* ============================================================================
* 27. FALLBACK
* ============================================================================
  */

runtimeFallbackPolicy
: runtimeWord
runtimeFallbackClause*
;

runtimeFallbackClause
: runtimeWord
expression?
| runtimeAttribute
;

/*

* ============================================================================
* 28. PORTABILITY
* ============================================================================
  */

runtimePortabilityPolicy
: PORTABILITY
runtimePortabilityClause*
;

runtimePortabilityClause
: runtimeWord
expression?
| runtimeAttribute
;

/*

* ============================================================================
* 29. TARGET CONSTRAINT
* ============================================================================
* 
* This is an intent constraint, NOT physical target selection.
* ============================================================================
  */

runtimeTargetConstraint
: TARGET
expression
runtimeTargetConstraintClause*
;

runtimeTargetConstraintClause
: REQUIRES
expression
| runtimeWord
expression
| runtimeAttribute
;

/*

* ============================================================================
* 30. COMPATIBILITY
* ============================================================================
  */

runtimeCompatibilityPolicy
: runtimeWord
runtimeCompatibilityClause*
;

runtimeCompatibilityClause
: REQUIRES
expression
| runtimeWord
expressionList?
| runtimeAttribute
;

/*

* ============================================================================
* 31. INVOCATION
* ============================================================================
* 
* Generic runtime extension invocation remains an AST/semantic operation.
* 
* It does not execute anything while parsing.
* ============================================================================
  */

runtimeInvocation
: runtimeWord
DOT
identifier
LPAREN
argumentList?
RPAREN
;

/*

* ============================================================================
* 32. COMPLETION / INTEGRATION CONTRACT
* ============================================================================
* 
* This file is complete when all of the following remain true:
* 
* [x] Filename remains grammar/execution/runtime.g4.
* 
* [x] Grammar name remains Runtime.
* 
* [x] Canonical lexer is ZamaniLexer.
* 
* [x] No second lexer is introduced.
* 
* [x] No Rust actions are present.
* 
* [x] No unsafe Rust is required.
* 
* [x] Rust integration remains Rust 2021 / Rust 1.97 / Rust 1.97.1.
* 
* [x] Runtime syntax is target-independent.
* 
* [x] No physical CPU/GPU/FPGA/QPU/device identity is required.
* 
* [x] No universal hardware capacity is encoded.
* 
* [x] No MAX_* machine/resource constants are encoded.
* 
* [x] Runtime does not define scheduling implementation.
* 
* [x] Runtime does not define placement implementation.
* 
* [x] Runtime does not define dispatch implementation.
* 
* [x] Runtime does not define deployment implementation.
* 
* [x] Runtime does not define routing.
* 
* [x] Runtime does not define optimization.
* 
* [x] Runtime does not define QEC.
* 
* [x] Runtime does not define ZQN.
* 
* [x] Runtime does not define HAL.
* 
* [x] Runtime does not define quantum::ir.
* 
* [x] Runtime consumes generic expressions/types/blocks from canonical
* grammar owners.
* 
* [x] Runtime has stable public entry points for Execution.
* 
* [x] Resource/capability/requirement/preference/hint distinctions remain
* semantically recoverable.
* 
* [x] Resilience state/outcome vocabulary remains extensible.
* 
* [x] Runtime lifecycle is represented without fixed finite state capacity.
* 
* [x] Checkpointing is represented as intent rather than storage
* implementation.
* 
* [x] Observability is represented as intent rather than a logging backend.
* 
* [x] Discovery is represented as intent rather than hardware discovery
* during parsing.
* 
* [x] Runtime invocation is syntax only.
* 
* [x] Physical realization remains downstream.
* 
* ============================================================================
* REQUIRED NEIGHBOR INTEGRATION
* ============================================================================
* 
* The following changes belong OUTSIDE this file:
* 
* 1. grammar/execution/execution.g4
* 
* Import Runtime:
* 
*    import Core, ExecutionContext, Runtime;
* 
* and expose Runtime's public entry points through the execution-domain
* composition rules.
* 
* Runtime must NOT import Execution back.
* 
* 2. grammar/antlr/ZamaniParser.g4
* 
* No direct Runtime import is required because the canonical root already
* imports Execution.
* 
* 3. grammar/execution/execution-context.g4
* 
* Remains authoritative for full execution-context syntax.
* 
* 4. grammar/execution/scheduling.g4
* 
* Remains authoritative for scheduling syntax and scheduling semantics.
* 
* 5. grammar/execution/placement.g4
* 
* Remains authoritative for placement intent.
* 
* 6. grammar/execution/dispatch.g4
* 
* Remains authoritative for dispatch intent.
* 
* 7. grammar/execution/synchronization.g4
* 
* Remains authoritative for synchronization intent.
* 
* 8. grammar/execution/runtime-capabilities.g4
* 
* Remains authoritative for runtime capability declarations that are
* broader than the local runtime requirement/reference forms.
* 
* 9. grammar/execution/deployment.g4
* 
* Remains authoritative for deployment intent.
* 
* 10. grammar/resources/
* 
* Remains authoritative for resource semantics.
* 
* 11. grammar/hardware/
* 
* Remains authoritative for hardware capability/resource/topology intent.
* 
* 12. grammar/quantum/
* 
* Remains authoritative for quantum source semantics.
* 
* 13. src/frontend/ast/
* 
* Owns the actual domain-neutral AST contract.
* 
* 14. semantic analysis
* 
* Resolves runtime intent against capabilities/resources/effects/types.
* 
* 15. canonical IR
* 
* Receives execution intent after semantic validation.
* 
* 16. quantum::ir
* 
* Remains the canonical quantum IR boundary.
* 
* 17. runtime implementation
* 
* Consumes verified semantic/IR execution plans.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* Also forbidden as universal runtime semantics:
* 
* cpu(0)
* gpu(0)
* qpu(0)
* node(0)
* device(0)
* fixed_core_count
* fixed_thread_count
* fixed_memory_size
* 
* Program values such as:
* 
* n
* qubits >= n
* memory >= required_memory
* 
* remain valid because they describe program/resource requirements rather than
* compiler-wide limits.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This grammar contains no embedded Rust.
* 
* Consequently it introduces:
* 
* - no unsafe code;
* - no filesystem access;
* - no network access;
* - no process execution;
* - no environment inspection;
* - no hardware probing;
* - no randomness;
* - no runtime calls.
* 
* The generated Rust frontend MUST remain safe Rust.
* 
* Rust baseline:
* 
* edition = 2021
* rust-version = 1.97
* compatible with Rust 1.97.1
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* Runtime grammar answers:
* 
* "What runtime behavior does the program request or permit?"
* 
* It does NOT answer:
* 
* "Which machine executes it?"
* 
* "Which CPU core executes it?"
* 
* "Which GPU executes it?"
* 
* "Which physical qubit is selected?"
* 
* "Which FPGA region is used?"
* 
* "Which node is allocated?"
* 
* "How is the computation scheduled?"
* 
* "How is a quantum circuit routed?"
* 
* "How is QEC performed?"
* 
* "How does ZQN operate?"
* 
* "How does the HAL communicate with hardware?"
* 
* Those decisions remain downstream.
* 
* Therefore the runtime boundary preserves:
* 
* Program Once
*      ->
* Compile Once
*      ->
* Run Everywhere
*      ->
* Run Anywhere
*      ->
* Run Forever
* 
* subject to program semantics and resources actually available at realization
* time, without imposing artificial language-level hardware ceilings.
* 
* ============================================================================
  */