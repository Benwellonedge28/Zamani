/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/dispatch.g4
 *
 * Grammar:
 *     Dispatch
 *
 * Status:
 *     Production-ready source-level dispatch-intent parser grammar
 *
 * Purpose:
 *     Defines the language-level intent for handing an already-described
 *     computation to an execution environment.
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
 *     canonical parser
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
 *     dispatch planning
 *          |
 *          v
 *     runtime / deployment
 *          |
 *          v
 *     execution environment
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Dispatch means:
 *
 *     "request that a semantically valid computation be handed to an
 *      execution environment according to declared execution intent."
 *
 * Dispatch DOES NOT mean:
 *
 *     - choosing a physical device;
 *     - discovering hardware;
 *     - allocating resources;
 *     - routing;
 *     - scheduling;
 *     - compiling;
 *     - optimizing;
 *     - translating quantum::ir;
 *     - executing a quantum gate;
 *     - selecting a vendor API;
 *     - opening a network connection;
 *     - starting a process;
 *     - communicating with hardware.
 *
 * Those responsibilities belong to downstream compiler, runtime, resource,
 * hardware, scheduling, routing, deployment, and resilience subsystems.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Dispatch syntax therefore describes:
 *
 *     intent
 *     target intent
 *     capability intent
 *     resource intent
 *     placement intent
 *     scheduling intent
 *     synchronization intent
 *     lifecycle intent
 *     reliability intent
 *     deployment intent
 *     transport intent
 *     execution policy
 *
 * It MUST NOT permanently encode:
 *
 *     - a machine;
 *     - a device;
 *     - a vendor;
 *     - a physical address;
 *     - a topology;
 *     - a fixed number of devices;
 *     - a fixed number of qubits;
 *     - a fixed number of CPU cores;
 *     - a fixed number of threads;
 *     - a fixed memory size;
 *     - a fixed network size;
 *     - a fixed queue size;
 *     - a fixed accelerator count.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - source-level dispatch declarations;
 *     - dispatch requests;
 *     - dispatch subjects;
 *     - dispatch modes;
 *     - dispatch destinations as abstract expressions;
 *     - dispatch requirements;
 *     - dispatch constraints;
 *     - dispatch preferences;
 *     - dispatch hints;
 *     - dispatch capability requirements;
 *     - dispatch resource intent;
 *     - dispatch placement intent;
 *     - dispatch scheduling intent;
 *     - dispatch synchronization intent;
 *     - dispatch lifecycle intent;
 *     - dispatch failure policy;
 *     - dispatch retry policy;
 *     - dispatch result binding;
 *     - dispatch options;
 *     - dispatch properties;
 *     - dispatch composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - functions;
 *     - modules;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware discovery;
 *     - resource discovery;
 *     - resource allocation;
 *     - routing;
 *     - scheduling algorithms;
 *     - optimization;
 *     - resilience algorithms;
 *     - QEC;
 *     - ZQN;
 *     - backend APIs;
 *     - deployment implementation;
 *     - process management;
 *     - network communication.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical execution grammar should import/compose this grammar and
 * expose:
 *
 *     dispatchDeclaration
 *
 * as the source-level dispatch entry point.
 *
 * Existing generic execution intent in:
 *
 *     grammar/execution/execution.g4
 *
 * should delegate its dispatch-specific syntax to:
 *
 *     dispatchDeclaration
 *
 * rather than maintaining a second implementation of dispatch syntax.
 *
 * The generic execution grammar currently has an `executionDispatch` concept.
 * That concept should become the integration adapter into this grammar.
 *
 * Therefore the intended dependency direction is:
 *
 *     Core
 *       |
 *       +--> ExecutionContext
 *       |
 *       +--> Dispatch
 *                |
 *                v
 *          execution.g4
 *                |
 *                v
 *          canonical parser
 *
 * The dependency MUST NOT be reversed.
 *
 * ============================================================================
 *
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Dispatch syntax produces semantic intent.
 *
 * Downstream processing is responsible for transforming that intent into
 * appropriate runtime actions.
 *
 * Conceptually:
 *
 *     DispatchSyntax
 *          |
 *          v
 *     DispatchIntent
 *          |
 *          +--> capability resolution
 *          +--> resource resolution
 *          +--> target resolution
 *          +--> placement planning
 *          +--> scheduling
 *          +--> routing
 *          +--> resilience policy
 *          |
 *          v
 *     DispatchPlan
 *          |
 *          v
 *     Runtime
 *
 * The grammar MUST NOT construct or select a DispatchPlan.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum execution remains owned by the quantum frontend and canonical
 * quantum IR.
 *
 * This grammar may dispatch a quantum computation, but it does not define:
 *
 *     - gates;
 *     - qubits;
 *     - physical qubits;
 *     - quantum registers;
 *     - quantum topology;
 *     - pulse operations;
 *     - QEC operations;
 *     - noise models;
 *     - ZQN faults;
 *     - quantum routing;
 *     - quantum scheduling.
 *
 * The intended path is:
 *
 *     quantum source
 *          |
 *          v
 *     quantum frontend
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     dispatch planning
 *          |
 *          v
 *     runtime
 *
 * `dispatch.g4` therefore cannot become a second quantum IR.
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computations may be dispatch subjects.
 *
 * The grammar does not redefine classical operations.
 *
 * Example semantic forms include:
 *
 *     dispatch compute();
 *
 *     dispatch matrix_multiply;
 *
 *     dispatch pipeline;
 *
 * The expression grammar remains authoritative for the subject.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware programs may also be dispatch subjects.
 *
 * This grammar does not define:
 *
 *     ports;
 *     wires;
 *     clocks;
 *     registers;
 *     hardware topology;
 *     FPGA fabric;
 *     ASIC implementation;
 *     physical addresses.
 *
 * Hardware semantics remain owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * and their corresponding semantic/compiler layers.
 *
 * ============================================================================
 *
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Dispatch may express distributed execution intent.
 *
 * It does not enumerate:
 *
 *     nodes;
 *     cluster sizes;
 *     fixed topology;
 *     provider names;
 *     fixed communication paths.
 *
 * Distributed execution resolves those properties after semantic analysis.
 *
 * ============================================================================
 *
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Dispatch may express failure/retry/recovery intent.
 *
 * It does NOT implement:
 *
 *     retry;
 *     rollback;
 *     checkpoint reconstruction;
 *     backend switching;
 *     fault diagnosis;
 *     mitigation;
 *     QEC;
 *     recovery algorithms.
 *
 * Resilience consumes the resulting semantic intent.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are no finite machine-size limits in this grammar.
 *
 * In particular there is no:
 *
 *     MAX_DEVICES
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_JOBS
 *     MAX_DISPATCHES
 *     MAX_ARGUMENTS
 *     MAX_PROPERTIES
 *     MAX_RETRIES
 *
 * Repetition is represented structurally through parser repetition.
 *
 * Actual limits belong to:
 *
 *     - parser resource policy;
 *     - compiler policy;
 *     - resource management;
 *     - runtime policy;
 *     - operating-system limits;
 *     - target capabilities;
 *     - explicitly declared user constraints.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no runtime calls;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no random behavior;
 *     - no mutable global state.
 *
 * Given the same canonical token stream, parsing is deterministic.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Downstream Rust implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * No `unsafe` is required or permitted by this grammar's design.
 *
 * ============================================================================
 */

parser grammar Dispatch;

options {
    tokenVocab = ZamaniLexer;
}

import Core;


/*
 * ============================================================================
 * 1. CANONICAL DISPATCH DECLARATION
 * ============================================================================
 *
 * This is the public entry point exported to execution.g4.
 *
 * The subject is an expression so dispatch can operate over any semantic
 * computation represented by the canonical language.
 *
 * Examples:
 *
 *     dispatch main();
 *     dispatch computation;
 *     dispatch pipeline;
 *
 * Optional context and dispatch configuration are attached through explicit
 * dispatch syntax.
 *
 * ============================================================================
 */

dispatchDeclaration
    : dispatchKeyword dispatchRequest dispatchTerminator?
    ;


/*
 * ============================================================================
 * 2. DISPATCH KEYWORD
 * ============================================================================
 *
 * `dispatch` remains an open identifier at the lexical level unless the
 * canonical lexer establishes a dedicated token later.
 *
 * This avoids coupling this grammar to a second lexer vocabulary.
 *
 * Semantic validation should recognize the canonical dispatch declaration
 * position rather than accepting arbitrary identifier text as an execution
 * operation.
 *
 * ============================================================================
 */

dispatchKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 3. DISPATCH REQUEST
 * ============================================================================
 */

dispatchRequest
    : dispatchSubject
      dispatchModifier*
    ;


/*
 * ============================================================================
 * 4. DISPATCH SUBJECT
 * ============================================================================
 *
 * The subject is deliberately expression-based.
 *
 * This permits dispatch of:
 *
 *     functions;
 *     expressions;
 *     values;
 *     pipelines;
 *     classical computations;
 *     quantum computations;
 *     HDL/hardware computations;
 *     distributed computations;
 *     accelerator computations;
 *     future semantic computation forms.
 *
 * No domain-specific subject list is maintained here.
 *
 * ============================================================================
 */

dispatchSubject
    : expression
    ;


/*
 * ============================================================================
 * 5. DISPATCH MODIFIERS
 * ============================================================================
 *
 * Modifiers express dispatch intent.
 *
 * They do not perform dispatch.
 * ============================================================================
 */

dispatchModifier
    : dispatchMode
    | dispatchDestination
    | dispatchContext
    | dispatchRequirement
    | dispatchConstraint
    | dispatchPreference
    | dispatchHint
    | dispatchCapability
    | dispatchResource
    | dispatchPlacement
    | dispatchSchedule
    | dispatchSynchronization
    | dispatchLifecycle
    | dispatchFailurePolicy
    | dispatchRetryPolicy
    | dispatchResultBinding
    | dispatchOption
    | dispatchProperty
    ;


/*
 * ============================================================================
 * 6. DISPATCH MODE
 * ============================================================================
 *
 * Modes are intentionally open-ended.
 *
 * The grammar does not enumerate:
 *
 *     local
 *     remote
 *     cloud
 *     quantum
 *     simulator
 *     accelerator
 *
 * as a closed set.
 *
 * Those are semantic target/capability concepts.
 *
 * ============================================================================
 */

dispatchMode
    : dispatchModeKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchModeKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 7. DISPATCH DESTINATION
 * ============================================================================
 *
 * A destination is an abstract destination expression.
 *
 * It MUST NOT be interpreted by the parser as a physical device ID.
 *
 * Resolution belongs to target/deployment analysis.
 * ============================================================================
 */

dispatchDestination
    : dispatchDestinationKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchDestinationKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 8. DISPATCH CONTEXT
 * ============================================================================
 *
 * Reuses the canonical expression model rather than defining a second
 * expression language.
 *
 * The execution-context grammar can provide richer structured context when
 * composed by the canonical execution grammar.
 *
 * ============================================================================
 */

dispatchContext
    : dispatchContextKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchContextKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Example conceptual forms:
 *
 *     requires quantum;
 *     requires capability_expression;
 *
 * Whether those requirements can be satisfied is NOT a grammar concern.
 *
 * ============================================================================
 */

dispatchRequirement
    : dispatchRequirementKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchRequirementKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 10. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict acceptable realizations.
 *
 * They are not physical allocation commands.
 *
 * ============================================================================
 */

dispatchConstraint
    : dispatchKey dispatchConstraintOperator dispatchValue
      dispatchClauseTerminator?
    ;


dispatchConstraintOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | ASSIGN
    | COLON
    ;


/*
 * ============================================================================
 * 11. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * Failure to honor a preference is not automatically a dispatch failure.
 * ============================================================================
 */

dispatchPreference
    : dispatchPreferenceKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchPreferenceKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 12. HINTS
 * ============================================================================
 *
 * Hints are advisory implementation guidance.
 *
 * They MUST NOT silently become requirements.
 * ============================================================================
 */

dispatchHint
    : dispatchHintKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchHintKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 13. CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * Capabilities are open-ended semantic names.
 *
 * This permits future capabilities without grammar redesign.
 * ============================================================================
 */

dispatchCapability
    : dispatchCapabilityKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchCapabilityKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 14. RESOURCE INTENT
 * ============================================================================
 *
 * Resource intent may express requirements such as:
 *
 *     resource memory_expression;
 *     resource quantum_expression;
 *     resource accelerator_expression;
 *
 * No fixed resource count is encoded.
 *
 * ============================================================================
 */

dispatchResource
    : dispatchResourceKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchResourceKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 15. PLACEMENT INTENT
 * ============================================================================
 *
 * Placement remains intent.
 *
 * Actual placement belongs to routing/resource/deployment layers.
 * ============================================================================
 */

dispatchPlacement
    : dispatchPlacementKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchPlacementKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 16. SCHEDULING INTENT
 * ============================================================================
 *
 * Dispatch may request a scheduling policy or scheduling property.
 *
 * It does not implement scheduling.
 * ============================================================================
 */

dispatchSchedule
    : dispatchScheduleKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchScheduleKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 17. SYNCHRONIZATION INTENT
 * ============================================================================
 *
 * Synchronization describes source-level completion/order requirements.
 *
 * It does not construct runtime synchronization primitives.
 * ============================================================================
 */

dispatchSynchronization
    : dispatchSynchronizationKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchSynchronizationKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 18. LIFECYCLE INTENT
 * ============================================================================
 *
 * Lifecycle describes desired execution lifecycle semantics.
 *
 * Examples include:
 *
 *     start
 *     suspend
 *     resume
 *     cancel
 *     detach
 *     retain
 *
 * These remain semantic properties rather than runtime commands.
 * ============================================================================
 */

dispatchLifecycle
    : dispatchLifecycleKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchLifecycleKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 19. FAILURE POLICY
 * ============================================================================
 *
 * Failure policy expresses what semantic behavior is desired if dispatch or
 * execution cannot proceed normally.
 *
 * Actual diagnosis/recovery is owned by resilience.
 * ============================================================================
 */

dispatchFailurePolicy
    : dispatchFailurePolicyKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchFailurePolicyKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 20. RETRY POLICY
 * ============================================================================
 *
 * Retry is a policy expression, not a hard-coded loop.
 *
 * There is deliberately no MAX_RETRIES grammar limit.
 * ============================================================================
 */

dispatchRetryPolicy
    : dispatchRetryPolicyKeyword dispatchValue dispatchClauseTerminator?
    ;


dispatchRetryPolicyKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 21. RESULT BINDING
 * ============================================================================
 *
 * A dispatch operation may describe where its result is semantically bound.
 *
 * This grammar does not determine result storage or serialization.
 * ============================================================================
 */

dispatchResultBinding
    : dispatchResultKeyword dispatchResultTarget dispatchClauseTerminator?
    ;


dispatchResultKeyword
    : identifier
    ;


dispatchResultTarget
    : expression
    ;


/*
 * ============================================================================
 * 22. GENERIC OPTIONS
 * ============================================================================
 *
 * Options are deliberately open-ended.
 *
 * This allows future execution environments to add semantic options without
 * requiring a fixed machine-specific grammar vocabulary.
 * ============================================================================
 */

dispatchOption
    : dispatchOptionKeyword dispatchOptionValue dispatchClauseTerminator?
    ;


dispatchOptionKeyword
    : identifier
    ;


dispatchOptionValue
    : expression
    | dispatchPropertyBlock
    ;


/*
 * ============================================================================
 * 23. GENERIC PROPERTIES
 * ============================================================================
 *
 * Properties provide an extensibility mechanism.
 *
 * They are data.
 *
 * They do not cause side effects while parsing.
 * ============================================================================
 */

dispatchProperty
    : dispatchKey dispatchPropertyOperator dispatchValue
      dispatchClauseTerminator?
    ;


dispatchPropertyOperator
    : ASSIGN
    | COLON
    ;


dispatchPropertyBlock
    : LBRACE dispatchPropertyEntry+ RBRACE
    ;


dispatchPropertyEntry
    : dispatchKey dispatchPropertyOperator dispatchValue
      dispatchClauseTerminator?
    ;


/*
 * ============================================================================
 * 24. DISPATCH KEY
 * ============================================================================
 *
 * Keys are qualified names.
 *
 * This allows:
 *
 *     quantum::...
 *     hardware::...
 *     distributed::...
 *     runtime::...
 *     vendor::...
 *     future::...
 *
 * without requiring this grammar to enumerate future domains.
 * ============================================================================
 */

dispatchKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 25. DISPATCH VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * This prevents creation of a second expression language.
 * ============================================================================
 */

dispatchValue
    : expression
    | dispatchPropertyBlock
    ;


/*
 * ============================================================================
 * 26. STRUCTURED DISPATCH OBJECT
 * ============================================================================
 *
 * Structured properties permit arbitrary nested execution metadata.
 *
 * Example conceptual form:
 *
 *     placement {
 *         locality: expression;
 *         affinity: expression;
 *     }
 *
 *     scheduling {
 *         policy: expression;
 *     }
 *
 *     resources {
 *         memory: expression;
 *         quantum: expression;
 *     }
 *
 * ============================================================================
 */

dispatchObject
    : LBRACE dispatchObjectEntry+ RBRACE
    ;


dispatchObjectEntry
    : dispatchKey dispatchObjectValue dispatchClauseTerminator?
    ;


dispatchObjectValue
    : expression
    | dispatchObject
    | dispatchList
    ;


/*
 * ============================================================================
 * 27. OPEN-ENDED LIST
 * ============================================================================
 *
 * No finite list size is encoded.
 * ============================================================================
 */

dispatchList
    : LBRACKET dispatchListElements? RBRACKET
    ;


dispatchListElements
    : dispatchValue
      (COMMA dispatchValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 28. NAMED DISPATCH ARGUMENTS
 * ============================================================================
 *
 * Some future execution-domain extensions may need argument-like syntax.
 *
 * This rule does not replace the canonical expression call syntax.
 * ============================================================================
 */

dispatchInvocation
    : dispatchKey
      LPAREN
      dispatchArgumentList?
      RPAREN
    ;


dispatchArgumentList
    : dispatchArgument
      (COMMA dispatchArgument)*
      COMMA?
    ;


dispatchArgument
    : dispatchKey dispatchPropertyOperator dispatchValue
    | dispatchValue
    ;


/*
 * ============================================================================
 * 29. DISPATCH POLICY OBJECT
 * ============================================================================
 *
 * Policies can contain arbitrary semantic properties.
 * ============================================================================
 */

dispatchPolicy
    : dispatchPolicyKeyword
      dispatchPolicyBody
    ;


dispatchPolicyKeyword
    : identifier
    ;


dispatchPolicyBody
    : dispatchObject
    | expression
    ;


/*
 * ============================================================================
 * 30. DISPATCH REQUIREMENT OBJECT
 * ============================================================================
 */

dispatchRequirementObject
    : dispatchRequirementKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 31. DISPATCH CAPABILITY OBJECT
 * ============================================================================
 */

dispatchCapabilityObject
    : dispatchCapabilityKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 32. DISPATCH RESOURCE OBJECT
 * ============================================================================
 */

dispatchResourceObject
    : dispatchResourceKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 33. DISPATCH PLACEMENT OBJECT
 * ============================================================================
 */

dispatchPlacementObject
    : dispatchPlacementKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 34. DISPATCH SCHEDULING OBJECT
 * ============================================================================
 */

dispatchSchedulingObject
    : dispatchScheduleKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 35. DISPATCH SYNCHRONIZATION OBJECT
 * ============================================================================
 */

dispatchSynchronizationObject
    : dispatchSynchronizationKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 36. DISPATCH LIFECYCLE OBJECT
 * ============================================================================
 */

dispatchLifecycleObject
    : dispatchLifecycleKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 37. DISPATCH FAILURE OBJECT
 * ============================================================================
 */

dispatchFailureObject
    : dispatchFailurePolicyKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 38. DISPATCH RETRY OBJECT
 * ============================================================================
 */

dispatchRetryObject
    : dispatchRetryPolicyKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 39. DISPATCH DESTINATION OBJECT
 * ============================================================================
 */

dispatchDestinationObject
    : dispatchDestinationKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 40. DISPATCH MODE OBJECT
 * ============================================================================
 */

dispatchModeObject
    : dispatchModeKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 41. DISPATCH CONTEXT OBJECT
 * ============================================================================
 */

dispatchContextObject
    : dispatchContextKeyword
      dispatchObject
    ;


/*
 * ============================================================================
 * 42. DISPATCH TERM
 * ============================================================================
 *
 * Semicolon is optional at the outer declaration boundary so the enclosing
 * statement grammar can own statement termination where appropriate.
 * ============================================================================
 */

dispatchTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 43. CLAUSE TERMINATOR
 * ============================================================================
 *
 * Individual dispatch properties may be terminated independently.
 * ============================================================================
 */

dispatchClauseTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 44. CANONICAL INTEGRATION ADAPTER
 * ============================================================================
 *
 * This rule is intentionally provided for execution.g4.
 *
 * Existing execution grammar currently exposes a generic executionDispatch
 * concept. The canonical execution grammar should delegate that concept to:
 *
 *     dispatchDeclaration
 *
 * rather than duplicating dispatch syntax.
 *
 * ============================================================================
 */

dispatchExecutionClause
    : dispatchDeclaration
    ;


/*
 * ============================================================================
 * 45. FUTURE DOMAIN EXTENSION POINT
 * ============================================================================
 *
 * A future execution-domain grammar may compose this grammar through the
 * canonical parser rather than modifying this file for every new computing
 * paradigm.
 *
 * Examples:
 *
 *     quantum dispatch
 *     classical dispatch
 *     HDL dispatch
 *     accelerator dispatch
 *     distributed dispatch
 *     AI dispatch
 *     edge dispatch
 *     cloud dispatch
 *     future computing dispatch
 *
 * Domain-specific semantics belong to their owning subsystems.
 * ============================================================================
 */