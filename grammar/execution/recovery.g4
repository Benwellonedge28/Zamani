/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/recovery.g4
 *
 * Grammar:
 *     Recovery
 *
 * Status:
 *     Production-ready modular parser grammar
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust, semantic actions, predicates,
 *     filesystem access, networking, hardware access, process execution,
 *     runtime calls, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL RECOVERY INTENT.
 *
 * Recovery syntax expresses what a Zamani program permits, requires, prefers,
 * or describes when execution encounters failure, degradation, interruption,
 * resource loss, capability change, stale execution state, or another
 * resilience condition.
 *
 * This grammar DOES NOT implement recovery.
 *
 * It does not:
 *
 *     - retry an operation;
 *     - restore a checkpoint;
 *     - migrate execution;
 *     - select hardware;
 *     - select a physical qubit;
 *     - perform QEC;
 *     - perform routing;
 *     - perform scheduling;
 *     - diagnose hardware;
 *     - query a provider;
 *     - access a filesystem;
 *     - contact a network;
 *     - invoke a runtime;
 *     - execute a recovery algorithm.
 *
 * It describes portable recovery intent which is interpreted by later
 * semantic, resilience, compiler, runtime, and HAL layers.
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
 *     canonical parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> dependency analysis
 *          +--> portability analysis
 *          +--> recovery-intent validation
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * Recovery belongs to the resilience/execution layer after semantic
 * interpretation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - recovery intent;
 *     - recovery policy declarations;
 *     - recovery action declarations;
 *     - recovery action sequences;
 *     - recovery triggers;
 *     - recovery conditions;
 *     - recovery preconditions;
 *     - recovery postconditions;
 *     - recovery verification intent;
 *     - recovery escalation intent;
 *     - recovery retry intent;
 *     - recovery restart intent;
 *     - recovery resume intent;
 *     - recovery rollback intent;
 *     - recovery migration intent;
 *     - recovery compensation intent;
 *     - recovery adaptation intent;
 *     - recovery degradation intent;
 *     - recovery quarantine intent;
 *     - recovery abort intent;
 *     - recovery budgets;
 *     - recovery backoff intent;
 *     - recovery idempotency intent;
 *     - recovery provenance references;
 *     - recovery checkpoint references;
 *     - recovery policy composition;
 *     - extensible recovery properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier definitions;
 *     - qualified-name definitions;
 *     - general expressions;
 *     - expression precedence;
 *     - general types;
 *     - general declarations;
 *     - general statements;
 *     - checkpoint implementation;
 *     - checkpoint serialization;
 *     - runtime implementation;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware discovery;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - provider APIs;
 *     - vendor APIs;
 *     - recovery algorithms.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This grammar MUST NOT create a second:
 *
 *     - expression grammar;
 *     - type grammar;
 *     - identifier grammar;
 *     - resource grammar;
 *     - capability grammar;
 *     - quantum grammar;
 *     - checkpoint grammar;
 *     - resilience state machine;
 *     - quantum IR;
 *     - recovery implementation.
 *
 * Recovery references concepts owned by other grammar/specification domains.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical lexer:
 *
 *     ZamaniLexer
 *
 * It imports only foundational syntax needed by its public rules:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Recovery-specific concepts remain structurally represented here.
 *
 * Scheduling, placement, checkpointing, hardware, quantum, resource, and
 * capability implementations remain outside this grammar.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar component.
 *
 * It is intended to be imported by the execution-domain composition grammar.
 *
 * Conceptually:
 *
 *     canonical parser
 *          |
 *          v
 *       Execution
 *          |
 *          +--> Runtime
 *          +--> Scheduling
 *          +--> Placement
 *          +--> Recovery
 *
 * The canonical root parser remains the composition authority.
 *
 * Do NOT create a second Zamani parser for recovery.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file does not define lexer rules.
 *
 * The canonical lexer remains authoritative.
 *
 * Recovery words are intentionally represented through IDENTIFIER/contextual
 * syntax wherever the repository lexer does not already provide a stable
 * reserved token.
 *
 * This prevents this leaf grammar from creating a competing keyword registry.
 *
 * If a recovery word is eventually promoted to a reserved keyword, that
 * change belongs to the canonical lexical authority and this grammar can
 * consume the resulting token without changing its semantic ownership.
 *
 * ============================================================================
 * PORTABILITY / POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Recovery syntax MUST describe portable semantic intent.
 *
 * It MUST NOT encode universal machine assumptions.
 *
 * Forbidden as language-level recovery limits:
 *
 *     MAX_RETRIES
 *     MAX_RECOVERY_ATTEMPTS
 *     MAX_INCIDENTS
 *     MAX_CHECKPOINTS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_TIMELINES
 *     MAX_ACTIONS
 *     MAX_PLAN_DEPTH
 *
 * There is no grammar-level finite maximum for:
 *
 *     - recovery actions;
 *     - recovery policies;
 *     - recovery attempts;
 *     - incidents;
 *     - failures;
 *     - checkpoints;
 *     - plans;
 *     - dependencies;
 *     - affected resources;
 *     - execution contexts;
 *     - devices;
 *     - nodes;
 *     - qubits;
 *     - processors;
 *     - accelerators;
 *     - recovery stages.
 *
 * Practical limits are determined by:
 *
 *     - available resources;
 *     - configured policy;
 *     - runtime capacity;
 *     - compiler capacity;
 *     - target capabilities;
 *     - provider constraints;
 *     - explicit program requirements.
 *
 * These are not grammar limits.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Recovery semantics MUST preserve the distinction between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * Requirement:
 *     Necessary for an acceptable realization.
 *
 * Constraint:
 *     Mandatory restriction on realization.
 *
 * Preference:
 *     Advisory optimization intent.
 *
 * Hint:
 *     Weaker advisory information.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * A hint MUST NOT silently become a constraint.
 *
 * ============================================================================
 * RECOVERY SEMANTIC MODEL
 * ============================================================================
 *
 * Recovery is an orchestration concern.
 *
 * Conceptually:
 *
 *     observation
 *          |
 *          v
 *     containment
 *          |
 *          v
 *     diagnosis
 *          |
 *          v
 *     policy evaluation
 *          |
 *          v
 *     planning
 *          |
 *          v
 *     plan validation
 *          |
 *          v
 *     ownership acquisition
 *          |
 *          v
 *     adaptation / recovery
 *          |
 *          v
 *     verification
 *          |
 *          +--> ACCEPT
 *          +--> DEGRADED_ACCEPT
 *          +--> RETRY
 *          +--> RECOVER
 *          +--> ESCALATE
 *          +--> REJECT
 *
 * The grammar represents the intent surrounding this process.
 *
 * It does not execute the process.
 *
 * ============================================================================
 * RECOVERY OUTCOMES
 * ============================================================================
 *
 * The recovery model established elsewhere in the repository defines the
 * canonical outcome vocabulary:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These names are represented contextually here rather than by creating a
 * second lexical authority.
 *
 * ============================================================================
 * RECOVERY ACTIONS
 * ============================================================================
 *
 * Canonical semantic action classes include:
 *
 *     retry
 *     restart
 *     resume
 *     rollback
 *     remap
 *     reroute
 *     reschedule
 *     recompile
 *     reoptimize
 *     change_qec
 *     mitigate
 *     switch_backend
 *     quarantine
 *     compensate
 *     abort
 *
 * The action vocabulary remains extensible.
 *
 * This grammar therefore accepts action names through identifiers rather than
 * permanently freezing the language to today's recovery implementation.
 *
 * ============================================================================
 * RETRY SAFETY
 * ============================================================================
 *
 * Retry MUST NOT imply an unconditional retry loop.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the operation is retryable;
 *     - whether it is idempotent;
 *     - whether duplicate submission is possible;
 *     - whether external side effects are reversible;
 *     - whether execution identity is preserved;
 *     - whether the policy permits retry;
 *     - whether resources permit retry;
 *     - whether verification remains possible.
 *
 * This grammar therefore expresses retry intent and retry policy, not a fixed
 * retry count.
 *
 * ============================================================================
 * UNKNOWN SUBMISSION STATE
 * ============================================================================
 *
 * Recovery syntax MUST be capable of representing an unknown execution state.
 *
 * This is required for cases such as:
 *
 *     submit
 *       |
 *       v
 *     transport failure
 *       |
 *       v
 *     provider state unknown
 *
 * The semantic/runtime layers MUST determine whether the execution identity
 * can be resolved before duplicate submission is attempted.
 *
 * ============================================================================
 * CHECKPOINT / RESTORE
 * ============================================================================
 *
 * Recovery may reference checkpoints.
 *
 * This grammar does NOT assert that arbitrary quantum state can be serialized
 * or restored.
 *
 * Semantic analysis MUST distinguish:
 *
 *     classical checkpoint;
 *     semantic checkpoint;
 *     measurement boundary;
 *     logical quantum checkpoint;
 *     provider-supported quantum state;
 *     reconstructible state;
 *     non-restorable state.
 *
 * A recovery program MUST NOT imply arbitrary quantum-state restoration merely
 * by naming a checkpoint.
 *
 * ============================================================================
 * ROLLBACK
 * ============================================================================
 *
 * Rollback intent means restoration to a semantically valid recovery boundary.
 *
 * It does NOT mean:
 *
 *     reconstruct arbitrary quantum state;
 *     undo irreversible external effects;
 *     erase provenance;
 *     erase the original failure;
 *     bypass verification.
 *
 * Rollback semantics remain owned by the resilience recovery implementation.
 *
 * ============================================================================
 * MIGRATION
 * ============================================================================
 *
 * Migration intent means that execution MAY be realized on another compatible
 * execution context.
 *
 * It does not select:
 *
 *     CPU 0;
 *     GPU 1;
 *     QPU 2;
 *     FPGA 3;
 *     node 7;
 *     physical qubit 12.
 *
 * Target realization belongs to resource/capability/placement/HAL systems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * For quantum programs:
 *
 *     source
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> ZQN
 *       +--> resilience
 *       +--> HAL
 *
 * Recovery grammar MUST NOT introduce:
 *
 *     QuantumRecoveryIR
 *     QuantumRecoveryPlanIR
 *     QuantumRecoveryCircuitIR
 *
 * as competing quantum representations.
 *
 * Recovery intent must map to the canonical semantic model and, where
 * appropriate, reference canonical quantum::ir entities.
 *
 * ============================================================================
 * QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * Recovery may express intent such as:
 *
 *     change_qec
 *     requires fault_tolerance(...)
 *     requires recovery_policy(...)
 *     requires noise_budget(...)
 *
 * But:
 *
 *     QEC owns error-correction algorithms.
 *     ZQN owns fault/noise semantics.
 *     Resilience owns recovery orchestration.
 *
 * No responsibility may be silently transferred between them.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Recovery may apply to:
 *
 *     functions;
 *     tasks;
 *     processes;
 *     asynchronous operations;
 *     data pipelines;
 *     accelerators;
 *     distributed operations;
 *     external calls.
 *
 * Recovery does not define those constructs.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Recovery intent may apply to hardware/co-design execution such as:
 *
 *     degraded accelerator;
 *     failed pipeline stage;
 *     unavailable resource;
 *     timing violation;
 *     thermal degradation;
 *     hardware fault;
 *     verification failure.
 *
 * The grammar does not define physical recovery algorithms.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Recovery can express:
 *
 *     retry;
 *     restart;
 *     failover intent;
 *     migration;
 *     replication intent;
 *     compensation;
 *     checkpoint recovery;
 *     degraded continuation;
 *     escalation.
 *
 * It does not own node placement or distributed transport semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no randomness;
 *     - no runtime access;
 *     - no hardware access;
 *     - no network access;
 *     - no filesystem access;
 *     - no mutable parser-global state.
 *
 * Given the same token stream and grammar version, the parser must produce
 * deterministic syntax recognition.
 *
 * Recovery execution may be nondeterministic because actual resource and
 * failure conditions can change. That is a runtime/semantic concern, not a
 * parser concern.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing recovery intent MUST NOT:
 *
 *     - execute recovery;
 *     - access credentials;
 *     - bypass authorization;
 *     - access hardware;
 *     - contact providers;
 *     - start processes;
 *     - inspect secrets;
 *     - modify files;
 *     - mutate runtime state.
 *
 * Authorization, trust, identity, and policy evaluation belong downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST remains domain-neutral.
 *
 * Conceptual mappings:
 *
 *     recoveryDeclaration
 *         -> RecoveryPolicyDecl
 *
 *     recoveryRule
 *         -> RecoveryRule
 *
 *     recoveryTrigger
 *         -> RecoveryTrigger
 *
 *     recoveryAction
 *         -> RecoveryActionIntent
 *
 *     recoveryCondition
 *         -> RecoveryCondition
 *
 *     recoveryVerification
 *         -> VerificationIntent
 *
 *     recoveryBudget
 *         -> RecoveryBudget
 *
 *     recoveryOutcome
 *         -> RecoveryOutcomeIntent
 *
 * Each AST node MUST preserve:
 *
 *     - source span;
 *     - source ordering;
 *     - original expression/value;
 *     - contextual names;
 *     - modifiers;
 *     - attributes.
 *
 * This grammar does not define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the recovery policy is valid;
 *     - whether referenced execution exists;
 *     - whether actions are supported;
 *     - whether actions are legal for the affected operation;
 *     - whether retry is safe;
 *     - whether rollback is possible;
 *     - whether restoration is semantically meaningful;
 *     - whether migration is compatible;
 *     - whether resource requirements can be satisfied;
 *     - whether required capabilities exist;
 *     - whether policies conflict;
 *     - whether verification is sufficient;
 *     - whether the plan can preserve program semantics;
 *     - whether recovery creates an unsafe side effect;
 *     - whether a plan has become stale;
 *     - whether degraded acceptance is permitted;
 *     - whether escalation is required.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has no direct IR.
 *
 * Recovery intent is lowered into the canonical semantic model.
 *
 * Later resilience components consume that model.
 *
 * For quantum:
 *
 *     recovery intent
 *          |
 *          v
 *     semantic recovery model
 *          |
 *          +--> quantum::ir references
 *          |
 *          v
 *     resilience planning
 *          |
 *          v
 *     adaptation / recovery
 *
 * No competing quantum IR is introduced.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses repetition and nesting rather than finite enumerations.
 *
 * Therefore:
 *
 *     recoveryRule*
 *     recoveryAction*
 *     recoveryProperty*
 *     recoveryCondition*
 *
 * impose no language-level fixed count.
 *
 * Runtime limits are resource/policy dependent.
 *
 * This supports:
 *
 *     one operation;
 *     one qubit;
 *     many operations;
 *     many qubits;
 *     many devices;
 *     distributed systems;
 *     heterogeneous systems;
 *     future execution architectures.
 *
 * ============================================================================
 * FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] ownership is explicit;
 *     [x] dependencies are explicit;
 *     [x] no lexical authority is duplicated;
 *     [x] no expression grammar is duplicated;
 *     [x] no type grammar is duplicated;
 *     [x] no recovery implementation is embedded;
 *     [x] no hard-coded hardware limit exists;
 *     [x] no hard-coded retry count exists;
 *     [x] retry safety can be expressed;
 *     [x] rollback intent can be expressed;
 *     [x] resume intent can be expressed;
 *     [x] restart intent can be expressed;
 *     [x] migration intent can be expressed;
 *     [x] compensation intent can be expressed;
 *     [x] checkpoint references can be expressed;
 *     [x] verification intent can be expressed;
 *     [x] escalation intent can be expressed;
 *     [x] degraded acceptance can be expressed;
 *     [x] resource/capability requirements can be expressed;
 *     [x] conditions remain expression-owned;
 *     [x] extensibility is supported;
 *     [x] AST integration is specified;
 *     [x] semantic integration is specified;
 *     [x] IR integration is specified;
 *     [x] quantum::ir remains canonical;
 *     [x] scheduling remains separate;
 *     [x] placement remains separate;
 *     [x] QEC remains separate;
 *     [x] ZQN remains separate;
 *     [x] runtime remains downstream;
 *     [x] Rust integration requires no unsafe;
 *     [x] positive tests are identifiable;
 *     [x] negative tests are identifiable;
 *     [x] scalability tests are identifiable.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

parser grammar Recovery;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC COMPOSITION
 * ============================================================================
 *
 * These are the stable entry points consumed by the execution grammar.
 */

recoveryDeclaration
    : recoveryPolicyDeclaration
    | recoveryRequirementDeclaration
    ;

recoveryStatement
    : recoveryRequestStatement
    | recoveryActionStatement
    | recoveryVerifyStatement
    | recoveryEscalateStatement
    ;

recoveryExpression
    : recoveryReferenceExpression
    | recoveryPolicyReferenceExpression
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL NAMES
 * ============================================================================
 *
 * Recovery-specific words remain contextual identifiers unless already exposed
 * by the canonical lexer.
 */

recoveryWord
    : identifier
    ;

recoveryName
    : identifier
    ;

recoveryQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. RECOVERY POLICY
 * ============================================================================
 *
 * A policy is a declarative collection of rules.
 */

recoveryPolicyDeclaration
    : recoveryWord
      recoveryName
      recoveryPolicyBody
    ;

recoveryPolicyBody
    : LBRACE
      recoveryPolicyMember*
      RBRACE
    ;

recoveryPolicyMember
    : recoveryRule
    | recoveryPolicyOption
    | recoveryBudget
    | recoveryVerification
    | recoveryRequirement
    | recoveryConstraint
    | recoveryPreference
    | recoveryHint
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 4. RECOVERY RULE
 * ============================================================================
 *
 * A rule associates a trigger/condition with one or more recovery actions.
 *
 * Conceptually:
 *
 *     when <condition> then <actions>
 *
 * Both words remain contextual unless the canonical lexer later reserves them.
 */

recoveryRule
    : recoveryTrigger
      recoveryCondition?
      recoveryActionBlock
      recoveryRuleModifier*
    ;

recoveryTrigger
    : recoveryWord
      recoveryReference?
    ;

recoveryCondition
    : WHEN
      expression
    ;

recoveryActionBlock
    : recoveryWord
      LBRACE
      recoveryAction*
      RBRACE
    | recoveryAction
    ;

recoveryRuleModifier
    : recoveryVerification
    | recoveryBudget
    | recoveryRequirement
    | recoveryConstraint
    | recoveryPreference
    | recoveryHint
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 5. RECOVERY REQUEST
 * ============================================================================
 *
 * A request expresses recovery intent without executing it.
 */

recoveryRequestStatement
    : recoveryWord
      recoveryReference?
      recoveryRequestModifier*
    ;

recoveryRequestModifier
    : recoveryCondition
    | recoveryActionBlock
    | recoveryVerification
    | recoveryBudget
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 6. RECOVERY ACTION
 * ============================================================================
 *
 * The action name is intentionally open-ended.
 *
 * Known semantic actions include:
 *
 *     retry
 *     restart
 *     resume
 *     rollback
 *     remap
 *     reroute
 *     reschedule
 *     recompile
 *     reoptimize
 *     change_qec
 *     mitigate
 *     switch_backend
 *     quarantine
 *     compensate
 *     abort
 *
 * Additional actions can be introduced by validated dialects/capabilities
 * without modifying the universal grammar.
 */

recoveryAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryActionArguments?
      recoveryActionModifier*
    ;

recoveryActionName
    : recoveryName
    ;

recoveryActionTarget
    : recoveryReference
    ;

recoveryActionArguments
    : LPAREN
      argumentList?
      RPAREN
    ;

recoveryActionModifier
    : recoveryCondition
    | recoveryPrecondition
    | recoveryPostcondition
    | recoveryVerification
    | recoveryBudget
    | recoveryRequirement
    | recoveryConstraint
    | recoveryPreference
    | recoveryHint
    | recoveryIdempotency
    | recoverySideEffectPolicy
    | recoveryProvenance
    | recoveryCheckpointReference
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 7. EXPLICIT ACTION FORMS
 * ============================================================================
 *
 * These rules provide stable semantic categories while keeping action names
 * extensible.
 */

recoveryRetryAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryRetryOption*
    ;

recoveryRetryOption
    : recoveryBudget
    | recoveryIdempotency
    | recoverySideEffectPolicy
    | recoveryVerification
    | recoveryCondition
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;

recoveryRestartAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryRestartOption*
    ;

recoveryRestartOption
    : recoveryCheckpointReference
    | recoveryVerification
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;

recoveryResumeAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryResumeOption*
    ;

recoveryResumeOption
    : recoveryCheckpointReference
    | recoveryVerification
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;

recoveryRollbackAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryRollbackOption*
    ;

recoveryRollbackOption
    : recoveryCheckpointReference
    | recoveryVerification
    | recoveryPrecondition
    | recoveryPostcondition
    | recoverySideEffectPolicy
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;

recoveryMigrationAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryMigrationOption*
    ;

recoveryMigrationOption
    : recoveryRequirement
    | recoveryConstraint
    | recoveryPreference
    | recoveryHint
    | recoveryVerification
    | recoveryProvenance
    | recoveryProperty
    ;

recoveryCompensationAction
    : recoveryActionName
      recoveryActionTarget?
      recoveryCompensationOption*
    ;

recoveryCompensationOption
    : recoveryVerification
    | recoveryPrecondition
    | recoveryPostcondition
    | recoverySideEffectPolicy
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 8. ACTION STATEMENT
 * ============================================================================
 */

recoveryActionStatement
    : recoveryAction
    ;


/*
 * ============================================================================
 * 9. VERIFICATION
 * ============================================================================
 *
 * Recovery MUST NOT imply success merely because an action executed.
 *
 * Verification intent therefore remains explicit.
 */

recoveryVerifyStatement
    : recoveryWord
      recoveryReference?
      recoveryVerification
    ;

recoveryVerification
    : recoveryWord
      recoveryVerificationBody?
    ;

recoveryVerificationBody
    : LBRACE
      recoveryVerificationMember*
      RBRACE
    ;

recoveryVerificationMember
    : recoveryAcceptance
    | recoveryRejection
    | recoveryDegradedAcceptance
    | recoveryOutcome
    | recoveryCondition
    | recoveryRequirement
    | recoveryConstraint
    | recoveryProperty
    ;

recoveryAcceptance
    : recoveryWord
      expression?
    ;

recoveryRejection
    : recoveryWord
      expression?
    ;

recoveryDegradedAcceptance
    : recoveryWord
      expression?
    ;

recoveryOutcome
    : recoveryWord
      expression?
    ;


/*
 * ============================================================================
 * 10. ESCALATION
 * ============================================================================
 *
 * Escalation is an explicit semantic outcome.
 */

recoveryEscalateStatement
    : recoveryWord
      recoveryReference?
      recoveryEscalationBody?
    ;

recoveryEscalationBody
    : LBRACE
      recoveryEscalationMember*
      RBRACE
    ;

recoveryEscalationMember
    : recoveryCondition
    | recoveryRequirement
    | recoveryConstraint
    | recoveryVerification
    | recoveryProperty
    ;


/*
 * ============================================================================
 * 11. REQUIREMENTS
 * ============================================================================
 *
 * Requirements remain semantically distinct from preferences and hints.
 */

recoveryRequirementDeclaration
    : REQUIRES
      recoveryRequirement
    ;

recoveryRequirement
    : recoveryRequirementKind
      recoveryRequirementValue
    ;

recoveryRequirementKind
    : recoveryWord
    ;

recoveryRequirementValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 */

recoveryConstraint
    : recoveryWord
      recoveryConstraintValue
    ;

recoveryConstraintValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 13. PREFERENCE
 * ============================================================================
 */

recoveryPreference
    : recoveryWord
      recoveryPreferenceValue
    ;

recoveryPreferenceValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 14. HINT
 * ============================================================================
 */

recoveryHint
    : recoveryWord
      recoveryHintValue?
    ;

recoveryHintValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 15. PRECONDITIONS / POSTCONDITIONS
 * ============================================================================
 */

recoveryPrecondition
    : recoveryWord
      expression
    ;

recoveryPostcondition
    : recoveryWord
      expression
    ;


/*
 * ============================================================================
 * 16. RECOVERY BUDGET
 * ============================================================================
 *
 * A budget is a semantic expression.
 *
 * It MUST NOT imply a universal maximum.
 *
 * Examples of legal semantic concepts:
 *
 *     retry_budget
 *     time_budget
 *     resource_budget
 *     energy_budget
 *     cost_budget
 *
 * The actual value can be supplied by the program, profile, environment,
 * policy, or runtime.
 */

recoveryBudget
    : recoveryWord
      recoveryBudgetBody
    ;

recoveryBudgetBody
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 17. IDEMPOTENCY
 * ============================================================================
 *
 * Retry safety may require explicit idempotency intent.
 */

recoveryIdempotency
    : recoveryWord
      recoveryIdempotencyValue?
    ;

recoveryIdempotencyValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 18. SIDE-EFFECT POLICY
 * ============================================================================
 *
 * Recovery must account for irreversible external effects.
 */

recoverySideEffectPolicy
    : recoveryWord
      recoverySideEffectValue?
    ;

recoverySideEffectValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 19. CHECKPOINT REFERENCE
 * ============================================================================
 *
 * This references checkpoint semantics without implementing checkpointing.
 */

recoveryCheckpointReference
    : recoveryWord
      recoveryReference
    ;

recoveryReference
    : expression
    ;


/*
 * ============================================================================
 * 20. PROVENANCE
 * ============================================================================
 *
 * Recovery actions must remain auditable.
 */

recoveryProvenance
    : recoveryWord
      recoveryProvenanceValue?
    ;

recoveryProvenanceValue
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 21. POLICY REFERENCES
 * ============================================================================
 */

recoveryPolicyReferenceExpression
    : recoveryWord
      recoveryQualifiedName
      recoveryInvocationArguments?
    ;

recoveryInvocationArguments
    : LPAREN
      argumentList?
      RPAREN
    ;

recoveryReferenceExpression
    : recoveryQualifiedName
    ;


/*
 * ============================================================================
 * 22. RECOVERY PROPERTY
 * ============================================================================
 *
 * This is intentionally open-ended.
 *
 * It allows future recovery features to be introduced without making every
 * implementation-specific concept a universal keyword.
 */

recoveryProperty
    : recoveryPropertyName
      (ASSIGN expression)?
    ;

recoveryPropertyName
    : recoveryQualifiedName
    ;


/*
 * ============================================================================
 * 23. OUTCOME REFERENCES
 * ============================================================================
 *
 * The semantic layer maps these contextual names to the canonical outcome
 * vocabulary:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * This grammar deliberately does not create a second outcome enum.
 */

recoveryOutcomeReference
    : recoveryWord
    ;


/*
 * ============================================================================
 * 24. RECOVERY ACTION SEQUENCES
 * ============================================================================
 *
 * Recovery plans may contain arbitrary-length action sequences.
 */

recoveryActionSequence
    : recoveryAction+
    ;


/*
 * ============================================================================
 * 25. RECOVERY POLICY COMPOSITION
 * ============================================================================
 *
 * Policies may be composed without a fixed nesting depth.
 */

recoveryPolicyReference
    : recoveryWord
      recoveryQualifiedName
      recoveryInvocationArguments?
    ;

recoveryPolicyComposition
    : recoveryPolicyReference
      recoveryPolicyCompositionOperator
      recoveryPolicyReference
    ;

recoveryPolicyCompositionOperator
    : recoveryWord
    ;


/*
 * ============================================================================
 * 26. RECOVERY CONDITION GROUP
 * ============================================================================
 */

recoveryConditionGroup
    : LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 27. FAILURE / DEGRADATION REFERENCES
 * ============================================================================
 *
 * Failure classes remain open-ended.
 *
 * Examples include:
 *
 *     timeout
 *     resource_unavailable
 *     capability_loss
 *     provider_failure
 *     transport_failure
 *     calibration_drift
 *     quantum_error
 *     qec_degradation
 *     checkpoint_incompatibility
 *     stale_plan
 *     verification_failure
 *     unknown_submission
 *
 * These are semantic classifications, not an exhaustive grammar enum.
 */

recoveryFailureReference
    : recoveryQualifiedName
    ;

recoveryDegradationReference
    : recoveryQualifiedName
    ;


/*
 * ============================================================================
 * 28. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime owns execution mechanics.
 *
 * Recovery grammar provides runtime with declarative intent only.
 *
 * Runtime MAY consume:
 *
 *     recovery policy;
 *     action intent;
 *     checkpoint reference;
 *     verification intent;
 *     escalation intent;
 *     budget;
 *     idempotency;
 *     side-effect policy;
 *     provenance.
 *
 * Runtime MUST NOT treat parsing as authorization to execute recovery.
 */


/*
 * ============================================================================
 * 29. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Recovery may require:
 *
 *     rescheduling;
 *     ordering;
 *     synchronization;
 *     timing changes.
 *
 * Scheduling remains owned by:
 *
 *     grammar/execution/scheduling.g4
 *
 * Recovery refers to scheduling intent rather than redefining it.
 */


/*
 * ============================================================================
 * 30. PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Migration/remapping may require placement.
 *
 * Placement remains owned by its execution grammar and downstream semantic
 * subsystem.
 *
 * Recovery MUST NOT select physical devices directly.
 */


/*
 * ============================================================================
 * 31. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Recovery requirements can express semantic requirements such as:
 *
 *     requires capability("quantum.measurement")
 *     requires capability("quantum.dynamic_control")
 *     requires resource(qubits >= required_qubits)
 *     requires resource(memory >= required_memory)
 *
 * The recovery grammar does not define capability discovery or resource
 * allocation.
 */


/*
 * ============================================================================
 * 32. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum recovery can reference semantic objects through ordinary names and
 * expressions.
 *
 * It MUST NOT introduce:
 *
 *     physical qubit limits;
 *     physical qubit enumeration;
 *     fixed topology;
 *     fixed gate lists;
 *     fixed QPU counts;
 *     fixed circuit sizes.
 *
 * Example intent:
 *
 *     recover quantum_execution {
 *         remap;
 *         reroute;
 *         reschedule;
 *         verify;
 *     }
 *
 * The actual realization belongs downstream.
 */


/*
 * ============================================================================
 * 33. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed recovery can express:
 *
 *     migration;
 *     retry;
 *     restart;
 *     compensation;
 *     checkpoint restoration;
 *     degraded execution;
 *     escalation.
 *
 * Node discovery and placement remain external to this grammar.
 */


/*
 * ============================================================================
 * 34. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware recovery can reference semantic properties such as:
 *
 *     capability loss;
 *     resource degradation;
 *     timing failure;
 *     thermal condition;
 *     reliability state;
 *     verification failure.
 *
 * Physical implementation remains owned by hardware/HAL layers.
 */


/*
 * ============================================================================
 * 35. ERROR HANDLING CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Runtime recovery failures belong to runtime/resilience.
 *
 * Examples:
 *
 *     parser:
 *         malformed recovery policy
 *
 *     semantic:
 *         rollback target is not restorable
 *
 *     policy:
 *         retry forbidden for non-idempotent operation
 *
 *     capability:
 *         migration target lacks required capability
 *
 *     runtime:
 *         provider rejected recovery action
 *
 * These categories MUST NOT be collapsed into one parser error.
 */


/*
 * ============================================================================
 * 36. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * The repository validation layer should verify:
 *
 *     - grammar reachability;
 *     - no duplicate lexical authority;
 *     - no ambiguous public entry points;
 *     - no accidental finite limits;
 *     - no embedded actions;
 *     - no embedded predicates;
 *     - no hardware identities;
 *     - no duplicate IR concepts;
 *     - AST coverage;
 *     - semantic coverage;
 *     - negative coverage;
 *     - scalability coverage;
 *     - deterministic parsing.
 */


/*
 * ============================================================================
 * 37. REQUIRED POSITIVE TEST CLASSES
 * ============================================================================
 *
 * The grammar test suite should cover at minimum:
 *
 *     recovery policy
 *     recovery rule
 *     recovery action
 *     retry
 *     restart
 *     resume
 *     rollback
 *     migration
 *     compensation
 *     verification
 *     escalation
 *     degraded acceptance
 *     resource requirement
 *     capability requirement
 *     checkpoint reference
 *     idempotency
 *     side-effect policy
 *     provenance
 *     nested policies
 *     qualified action names
 *     vendor/future extension names
 *     arbitrary action arguments
 *     arbitrary recovery properties
 *     quantum recovery intent
 *     classical recovery intent
 *     distributed recovery intent
 *     hardware recovery intent
 */


/*
 * ============================================================================
 * 38. REQUIRED NEGATIVE TEST CLASSES
 * ============================================================================
 *
 * Reject malformed constructs such as:
 *
 *     incomplete recovery policy
 *     incomplete action
 *     malformed argument list
 *     malformed condition
 *     malformed property assignment
 *     malformed checkpoint reference
 *     malformed verification body
 *
 * Semantic tests, rather than parser tests, should reject:
 *
 *     unsafe retry;
 *     impossible rollback;
 *     unavailable capability;
 *     incompatible checkpoint;
 *     stale recovery plan;
 *     unauthorized recovery;
 *     non-reversible side-effect replay.
 */


/*
 * ============================================================================
 * 39. REQUIRED SCALABILITY TEST CLASSES
 * ============================================================================
 *
 * Tests MUST NOT define a maximum.
 *
 * They should verify that syntax supports:
 *
 *     one action;
 *     many actions;
 *     deeply nested policies;
 *     large dependency sets;
 *     large recovery plans;
 *     large distributed execution descriptions;
 *     large quantum execution descriptions;
 *     dynamically sized resource requirements.
 *
 * Resource exhaustion is an implementation concern and MUST NOT become a
 * language-level artificial ceiling.
 */


/*
 * ============================================================================
 * 40. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_RETRIES
 *     MAX_RECOVERY_ATTEMPTS
 *     MAX_ACTIONS
 *     MAX_CHECKPOINTS
 *     MAX_INCIDENTS
 *     MAX_TIMELINES
 *
 * Numeric expressions remain legal where they represent program semantics.
 *
 * Example:
 *
 *     retry_budget(10)
 *
 * is a program/policy value.
 *
 * It MUST NOT be interpreted as:
 *
 *     the maximum retry capacity of Zamani.
 *
 * ============================================================================
 * 41. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * All downstream Zamani recovery implementation MUST target:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST NOT require `unsafe`.
 *
 * Parser generation and AST/semantic integration must remain compatible with
 * the repository's safe-Rust architecture.
 *
 * ============================================================================
 * 42. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Required downstream integration:
 *
 *     1. Execution composition imports Recovery.
 *
 *     2. Canonical Zamani parser reaches recoveryDeclaration,
 *        recoveryStatement, and recoveryExpression through Execution.
 *
 *     3. AST mapping is added to the existing domain-neutral frontend AST.
 *
 *     4. No recovery-specific quantum IR is created.
 *
 *     5. Semantic analysis maps recovery intent to the resilience model.
 *
 *     6. Resource/capability analysis validates requirements.
 *
 *     7. Scheduling consumes reschedule/order/timing intent where applicable.
 *
 *     8. Placement consumes migration/remap intent where applicable.
 *
 *     9. Checkpoint subsystem owns checkpoint creation/restoration.
 *
 *    10. Resilience planning owns plan generation.
 *
 *    11. Resilience recovery owns action execution.
 *
 *    12. Verification owns acceptance decisions.
 *
 *    13. Runtime owns execution mechanics.
 *
 *    14. HAL owns target realization.
 *
 *    15. QEC owns error correction.
 *
 *    16. ZQN owns noise/fault semantics.
 *
 *    17. Validation tests grammar reachability and semantic coverage.
 *
 *    18. Compatibility tests ensure old valid programs remain valid unless
 *        an explicitly versioned language change says otherwise.
 *
 * ============================================================================
 * 43. FINAL OWNERSHIP INVARIANT
 * ============================================================================
 *
 * The invariant for this file is:
 *
 *     recovery.g4
 *          =
 *     WHAT RECOVERY INTENT MEANS AT SOURCE LEVEL
 *
 *     resilience implementation
 *          =
 *     HOW RECOVERY IS PLANNED AND EXECUTED
 *
 *     runtime / HAL
 *          =
 *     HOW THE TARGET ACTUALLY PERFORMS IT
 *
 * This separation is required for POCO-REAF.
 *
 * ============================================================================
 */