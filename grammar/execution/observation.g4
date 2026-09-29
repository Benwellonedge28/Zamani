/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/observation.g4
 *
 * Grammar:
 *     ExecutionObservation
 *
 * Status:
 *     Production-ready source-level observation-intent grammar
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No parser actions.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No runtime execution.
 *     No resource discovery.
 *     No capability discovery.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL OBSERVATION INTENT of Zamani execution.
 *
 * It provides an independent syntax boundary for expressing:
 *
 *     - what execution information is to be observed;
 *     - what computation or execution object is observed;
 *     - observation scope;
 *     - observation properties;
 *     - observation conditions;
 *     - observation triggers;
 *     - observation requirements;
 *     - observation capabilities;
 *     - observation constraints;
 *     - observation preferences;
 *     - observation hints;
 *     - observation context;
 *     - observation correlation;
 *     - observation provenance;
 *     - extensible observation metadata.
 *
 * This file deliberately does NOT define a closed catalog of:
 *
 *     metrics
 *     events
 *     counters
 *     telemetry providers
 *     profiler names
 *     tracing backends
 *     hardware counters
 *     quantum observables
 *     vendor telemetry APIs
 *
 * Such names remain semantic identifiers or expressions.
 *
 * ============================================================================
 * 2. NON-OWNERSHIP
 * ============================================================================
 *
 * This file DOES NOT own:
 *
 *     - telemetry collection;
 *     - tracing implementation;
 *     - profiling implementation;
 *     - logging implementation;
 *     - metrics storage;
 *     - event storage;
 *     - export protocols;
 *     - OpenTelemetry;
 *     - Prometheus;
 *     - vendor profiler APIs;
 *     - runtime APIs;
 *     - scheduler implementation;
 *     - placement implementation;
 *     - resource allocation;
 *     - hardware discovery;
 *     - capability discovery;
 *     - checkpointing;
 *     - recovery;
 *     - resilience;
 *     - ZQN;
 *     - QEC;
 *     - HAL;
 *     - quantum::ir;
 *     - physical device identifiers.
 *
 * Those responsibilities belong to downstream semantic, compiler, runtime,
 * tooling, resource, security, quantum, hardware, and deployment layers.
 *
 * ============================================================================
 * 3. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The required direction is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ExecutionObservation parser grammar
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> observation semantics
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> effect analysis
 *          +--> ownership/privacy analysis
 *          +--> provenance analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                    scheduling / placement
 *                              |
 *                              v
 *                           runtime
 *                              |
 *                              v
 *                    observation realization
 *
 * Observation is therefore metadata/intent surrounding computation.
 *
 * It is NOT a second execution language.
 *
 * ============================================================================
 * 4. POCO-REAF
 * ============================================================================
 *
 * Observation syntax must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A program may express observation intent without knowing:
 *
 *     - which CPU;
 *     - which GPU;
 *     - which FPGA;
 *     - which ASIC;
 *     - which QPU;
 *     - which accelerator;
 *     - which distributed node;
 *     - which runtime;
 *     - which telemetry collector;
 *     - which profiler;
 *     - which storage system;
 *     - which network;
 *     - which hardware counter.
 *
 * For example, source may express:
 *
 *     observe execution;
 *
 *     observe latency;
 *
 *     observe metric("throughput");
 *
 *     observe trace;
 *
 *     observe execution {
 *         requires capability("telemetry");
 *         constraint observation::overhead <= budget;
 *     };
 *
 * Whether the requested observation can actually be realized is determined
 * downstream.
 *
 * ============================================================================
 * 5. SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal observation capacity.
 *
 * It MUST NOT encode:
 *
 *     MAX_OBSERVATIONS
 *     MAX_EVENTS
 *     MAX_METRICS
 *     MAX_SPANS
 *     MAX_ATTRIBUTES
 *     MAX_CONTEXT_ENTRIES
 *     MAX_TRACE_DEPTH
 *     MAX_SAMPLES
 *     MAX_EXPORTERS
 *     MAX_TARGETS
 *     MAX_TIMELINES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QPUS
 *
 * Repetition is represented structurally by ANTLR repetition operators.
 *
 * Any numerical quantity appearing in source is:
 *
 *     - program data;
 *     - an expression;
 *     - a semantic requirement;
 *     - a constraint;
 *     - a preference;
 *     - or a hint.
 *
 * It is never a grammar-wide capacity.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level ceiling.
 *
 * It does not claim that a particular parser, compiler, runtime, or physical
 * target has infinite resources.
 *
 * ============================================================================
 * 6. DEPENDENCY DIRECTION
 * ============================================================================
 *
 * This grammar intentionally depends ONLY on foundational parser grammars:
 *
 *     Names
 *     Expressions
 *
 * It MUST NOT import:
 *
 *     Execution
 *     Observability
 *     ExecutionTimelines
 *     Runtime
 *     Scheduling
 *     Placement
 *     Recovery
 *     Checkpointing
 *     Profiling
 *     Lifecycle
 *     Dispatch
 *     Deployment
 *
 * This is deliberate.
 *
 * The execution composition root consumes this grammar.
 *
 * Therefore:
 *
 *     observation
 *          |
 *          v
 *     execution
 *          |
 *          v
 *     ZamaniParser
 *
 * and NEVER:
 *
 *     execution
 *          |
 *          v
 *     observation
 *          |
 *          v
 *     execution
 *
 * This prevents circular parser dependencies.
 *
 * ============================================================================
 * 7. RELATIONSHIP TO observability.g4
 * ============================================================================
 *
 * `observability.g4` owns broader source-level observability concepts such as:
 *
 *     events
 *     metrics
 *     logs
 *     traces
 *     spans
 *     profiles
 *     sampling
 *     filtering
 *     retention
 *     export
 *     privacy
 *     provenance
 *
 * This file MUST NOT duplicate those detailed observability grammars.
 *
 * Instead, this file owns the smaller execution-level:
 *
 *     "observe <subject> ..."
 *
 * boundary.
 *
 * The semantic layer may associate this observation intent with the broader
 * observability model.
 *
 * This separation allows:
 *
 *     observation.g4
 *         ->
 *     execution observation intent
 *
 * while:
 *
 *     observability.g4
 *         ->
 *     detailed observability intent
 *
 * without creating two semantic observability systems.
 *
 * ============================================================================
 * 8. RELATIONSHIP TO timelines.g4
 * ============================================================================
 *
 * `timelines.g4` remains the sole owner of MTS-specific temporal syntax.
 *
 * In particular:
 *
 *     mts observe <timeline> ...
 *
 * remains owned by `ExecutionTimelines`.
 *
 * This file owns the non-MTS execution observation construct:
 *
 *     observe <subject> ...
 *
 * This distinction is intentional.
 *
 * There must be no duplicate ownership of:
 *
 *     MTS
 *     timeline relationships
 *     timeline branching
 *     timeline merging
 *     timeline rewinding
 *
 * MTS-specific observation is represented by the timeline grammar and
 * semantically associated with this observation model downstream.
 *
 * ============================================================================
 * 9. LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The following lexical concepts are already owned by the canonical lexer:
 *
 *     OBSERVE
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     WITH
 *     WHEN
 *     FROM
 *
 * This file does NOT define lexer tokens.
 *
 * It also does not create tokens for:
 *
 *     metric
 *     latency
 *     throughput
 *     event
 *     trace
 *     sample
 *     profiler
 *     telemetry
 *
 * Those remain identifiers/qualified names unless the canonical lexical
 * specification independently establishes otherwise.
 *
 * ============================================================================
 * 10. PUBLIC ROOT
 * ============================================================================
 *
 * `observationConstruct` is the public entry point supplied to the execution
 * composition grammar.
 *
 * Example:
 *
 *     observe execution;
 *
 *     observe latency;
 *
 *     observe metric("throughput");
 *
 *     observe execution {
 *         requires capability("telemetry");
 *     };
 *
 * ============================================================================
 */

parser grammar ExecutionObservation;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/*
 * ============================================================================
 * 11. PUBLIC OBSERVATION CONSTRUCT
 * ============================================================================
 *
 * The leading `observe` token is the stable lexical anchor.
 *
 * The remainder is deliberately extensible.
 *
 * ============================================================================
 */

observationConstruct
    : OBSERVE observationSubject? observationBody? observationTerminator
    ;


/*
 * ============================================================================
 * 12. OBSERVATION SUBJECT
 * ============================================================================
 *
 * A subject is an ordinary Zamani expression.
 *
 * This permits observation of:
 *
 *     execution
 *     latency
 *     metric("throughput")
 *     trace
 *     task
 *     timeline
 *     result
 *     resource
 *     capability
 *     user-defined semantic objects
 *
 * without introducing a closed observation vocabulary.
 *
 * The expression is interpreted semantically.
 *
 * ============================================================================
 */

observationSubject
    : expression
    ;


/*
 * ============================================================================
 * 13. OBSERVATION BODY
 * ============================================================================
 *
 * A body contains zero or more observation clauses.
 *
 * There is no fixed number of clauses.
 *
 * ============================================================================
 */

observationBody
    : LBRACE observationClause* RBRACE
    ;


/*
 * ============================================================================
 * 14. OBSERVATION CLAUSE
 * ============================================================================
 *
 * Clauses are divided by semantic category.
 *
 * This keeps:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * distinct even though all eventually become execution metadata.
 *
 * ============================================================================
 */

observationClause
    : observationPropertyClause
    | observationRequirementClause
    | observationCapabilityClause
    | observationConstraintClause
    | observationPreferenceClause
    | observationHintClause
    | observationConditionClause
    | observationTriggerClause
    | observationContextClause
    | observationCorrelationClause
    | observationProvenanceClause
    | observationExtensionClause
    ;


/*
 * ============================================================================
 * 15. PROPERTY
 * ============================================================================
 *
 * Generic properties are intentionally open-ended.
 *
 * Examples:
 *
 *     mode: aggregate;
 *
 *     scope: execution;
 *
 *     detail: summary;
 *
 *     observation::mode: isolated;
 *
 *     sampling::policy: adaptive;
 *
 * The semantic layer determines whether a property is known, supported,
 * deprecated, dialect-specific, or invalid.
 *
 * ============================================================================
 */

observationPropertyClause
    : qualifiedName COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 16. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires capability("telemetry");
 *
 *     requires memory >= required_memory;
 *
 *     requires observation_support;
 *
 * The grammar does not determine whether the requirement can be satisfied.
 *
 * ============================================================================
 */

observationRequirementClause
    : REQUIRES expression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CAPABILITY
 * ============================================================================
 *
 * Capability names remain expressions rather than a closed enumeration.
 *
 * Example:
 *
 *     requires capability("distributed.tracing");
 *
 * The semantic layer determines whether the named capability is meaningful
 * under the selected language profile/dialect and whether the target provides
 * it.
 *
 * ============================================================================
 */

observationCapabilityClause
    : capabilityIntroducer expression SEMICOLON
    ;


capabilityIntroducer
    : capabilityKeyword
    | capabilityName
    ;


capabilityKeyword
    : qualifiedName
    ;


capabilityName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 18. CONSTRAINT
 * ============================================================================
 *
 * A constraint describes a semantic condition that realization must respect.
 *
 * Example:
 *
 *     constraint observation::overhead <= budget;
 *
 * ============================================================================
 */

observationConstraintClause
    : CONSTRAINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PREFERENCE
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT silently become a requirement.
 *
 * Example:
 *
 *     prefer observation::detail;
 *
 * ============================================================================
 */

observationPreferenceClause
    : PREFER expression SEMICOLON
    ;


/*
 * ============================================================================
 * 20. HINT
 * ============================================================================
 *
 * A hint is advisory implementation information.
 *
 * It is not semantically mandatory unless another explicit language contract
 * says otherwise.
 *
 * ============================================================================
 */

observationHintClause
    : HINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 21. CONDITION
 * ============================================================================
 *
 * Observation may be conditional.
 *
 * Examples:
 *
 *     when condition;
 *
 *     condition expression;
 *
 * The expression remains ordinary Zamani syntax.
 *
 * ============================================================================
 */

observationConditionClause
    : observationConditionKeyword expression SEMICOLON
    ;


observationConditionKeyword
    : WHEN
    | qualifiedName
    ;


/*
 * ============================================================================
 * 22. TRIGGER
 * ============================================================================
 *
 * Trigger syntax remains open-ended.
 *
 * Examples:
 *
 *     trigger event("completed");
 *
 *     trigger execution.completed;
 *
 * No finite event catalog is encoded here.
 *
 * ============================================================================
 */

observationTriggerClause
    : triggerKeyword expression SEMICOLON
    ;


triggerKeyword
    : qualifiedName
    ;


/*
 * ============================================================================
 * 23. CONTEXT
 * ============================================================================
 *
 * Observation context describes semantic context associated with the
 * observation.
 *
 * Example:
 *
 *     with context;
 *
 *     with execution.context;
 *
 * A context value remains an ordinary expression.
 *
 * This does not allocate or inspect runtime state during parsing.
 *
 * ============================================================================
 */

observationContextClause
    : WITH expression SEMICOLON
    ;


/*
 * ============================================================================
 * 24. CORRELATION
 * ============================================================================
 *
 * Correlation is represented semantically rather than by hard-coding a
 * particular tracing system.
 *
 * Examples:
 *
 *     correlation trace_id;
 *
 *     correlation distributed::context;
 *
 * ============================================================================
 */

observationCorrelationClause
    : correlationKeyword expression SEMICOLON
    ;


correlationKeyword
    : qualifiedName
    ;


/*
 * ============================================================================
 * 25. PROVENANCE
 * ============================================================================
 *
 * Provenance is source-level intent.
 *
 * Actual provenance collection and storage belong downstream.
 *
 * ============================================================================
 */

observationProvenanceClause
    : provenanceKeyword expression SEMICOLON
    ;


provenanceKeyword
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. EXTENSION
 * ============================================================================
 *
 * The extension form allows future observation concepts to be introduced
 * without requiring a new universal lexer keyword for every technology.
 *
 * It is NOT a semantic bypass.
 *
 * Every extension must still pass:
 *
 *     AST validation
 *     semantic validation
 *     capability validation
 *     resource validation
 *     portability validation
 *     dialect/feature validation
 *
 * ============================================================================
 */

observationExtensionClause
    : observationExtensionName observationExtensionValue? SEMICOLON
    ;


observationExtensionName
    : qualifiedName
    ;


observationExtensionValue
    : expression
    | LPAREN expressionList? RPAREN
    | LBRACE observationExtensionItem* RBRACE
    ;


observationExtensionItem
    : qualifiedName COLON expression SEMICOLON
    | qualifiedName ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 27. TERMINATOR
 * ============================================================================
 *
 * Observation constructs require an explicit semicolon at the public
 * construct boundary.
 *
 * This keeps the construct deterministic when composed with other execution
 * declarations.
 *
 * ============================================================================
 */

observationTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 28. AST CONTRACT
 * ============================================================================
 *
 * Every public construct maps into the existing domain-neutral frontend AST.
 *
 * This grammar MUST NOT create:
 *
 *     ObservationRuntimeAst
 *     TelemetryAst
 *     HardwareObservationAst
 *     QuantumObservationAst
 *     MTSObservationAst
 *
 * merely because an observation can concern those domains.
 *
 * The conceptual AST contract is:
 *
 *     Observation
 *         source_span
 *         subject
 *         clauses
 *
 * and:
 *
 *     ObservationClause
 *         kind
 *         value
 *         source_span
 *
 * The exact Rust AST type names are owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's canonical AST equivalent.
 *
 * The parser must preserve all information necessary for that mapping.
 *
 * ============================================================================
 * 29. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Source locations must remain available for:
 *
 *     observationConstruct
 *     observationSubject
 *     observationBody
 *     every observationClause
 *     every expression
 *     every qualified name
 *
 * Source spans are required for:
 *
 *     diagnostics
 *     IDE/LSP tooling
 *     provenance
 *     semantic errors
 *     compatibility diagnostics
 *     runtime correlation
 *
 * This grammar does not invent a second source-span type.
 *
 * ============================================================================
 * 30. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must classify observation constructs into the canonical
 * semantic observation model.
 *
 * At minimum it must distinguish:
 *
 *     observation intent
 *     requirement
 *     capability requirement
 *     constraint
 *     preference
 *     hint
 *     condition
 *     trigger
 *     context
 *     correlation
 *     provenance
 *     extension
 *
 * Parser acceptance MUST NOT imply:
 *
 *     capability availability
 *     resource availability
 *     telemetry availability
 *     runtime support
 *     target support
 *     hardware support
 *
 * ============================================================================
 * 31. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions remain ordinary expressions.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     constraint observation::overhead <= budget;
 *
 * The grammar does not know the actual quantity of:
 *
 *     memory
 *     bandwidth
 *     storage
 *     compute
 *     telemetry capacity
 *     network capacity
 *     runtime capacity
 *
 * Resource feasibility belongs to:
 *
 *     grammar/resources/
 *     semantic resource analysis
 *     compiler planning
 *     runtime
 *     deployment
 *
 * ============================================================================
 * 32. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are open-ended semantic names.
 *
 * Examples include:
 *
 *     capability("telemetry")
 *     capability("distributed.tracing")
 *     capability("quantum.observation")
 *     capability("hardware.counters")
 *
 * This grammar MUST NOT enumerate every current or future capability.
 *
 * A new capability must normally be representable without changing this
 * grammar.
 *
 * ============================================================================
 * 33. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Observation can surround quantum computation.
 *
 * The semantic path remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     observation metadata/intent
 *          |
 *          v
 *     runtime/backend realization
 *
 * This grammar MUST NOT create:
 *
 *     QuantumObservationIR
 *     ObservationQuantumIR
 *     MTSQuantumIR
 *
 * or any competing quantum representation.
 *
 * `quantum::ir` remains the canonical quantum boundary.
 *
 * ============================================================================
 * 34. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Observation may surround:
 *
 *     scalar computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     functions
 *     tasks
 *     distributed work
 *     scientific computation
 *     AI computation
 *
 * No classical-specific observation syntax is required merely because the
 * observed computation is classical.
 *
 * ============================================================================
 * 35. HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * Observation may refer to hardware-related semantic objects through:
 *
 *     expressions
 *     qualified names
 *     capabilities
 *     resources
 *     constraints
 *
 * It MUST NOT encode a universal physical hardware counter vocabulary.
 *
 * Examples such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *
 * remain names unless an explicit downstream target-binding contract gives
 * them meaning.
 *
 * ============================================================================
 * 36. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Observation may span:
 *
 *     processes
 *     tasks
 *     services
 *     nodes
 *     timelines
 *     distributed executions
 *
 * The grammar imposes no fixed node, process, task, service, or observation
 * count.
 *
 * Distributed correlation is semantic intent.
 *
 * Actual distributed trace collection belongs to runtime/tooling layers.
 *
 * ============================================================================
 * 37. MTS INTEGRATION
 * ============================================================================
 *
 * `timelines.g4` owns:
 *
 *     mts observe ...
 *
 * Observation metadata can therefore be associated semantically with a
 * timeline without this grammar taking ownership of timeline syntax.
 *
 * The semantic model may represent:
 *
 *     observation subject
 *     timeline association
 *     causal context
 *     branch identity
 *     correlation metadata
 *
 * without creating an MTS-specific IR.
 *
 * There is no fixed:
 *
 *     timeline count
 *     branch count
 *     observation count
 *     timestamp width
 *     temporal depth
 *
 * ============================================================================
 * 38. OBSERVABILITY INTEGRATION
 * ============================================================================
 *
 * The semantic layer may lower:
 *
 *     observe metric(...)
 *     observe trace
 *     observe event(...)
 *     observe latency
 *
 * into the canonical observability model owned by:
 *
 *     grammar/execution/observability.g4
 *
 * This file does not duplicate:
 *
 *     metric definitions
 *     trace schemas
 *     event schemas
 *     exporter schemas
 *     retention implementation
 *     sampling implementation
 *
 * ============================================================================
 * 39. TRACING INTEGRATION
 * ============================================================================
 *
 * Observation intent may be correlated with tracing.
 *
 * Source spans remain the source-location mechanism.
 *
 * Runtime trace spans are distinct from source spans:
 *
 *     source span
 *         =
 *     location in Zamani source
 *
 *     runtime trace span
 *         =
 *     runtime observation entity
 *
 * The semantic/runtime layers may associate them.
 *
 * This grammar must not conflate the two.
 *
 * ============================================================================
 * 40. PROFILING INTEGRATION
 * ============================================================================
 *
 * Observation may request profiling-related information through open semantic
 * names and expressions.
 *
 * Profiling implementation remains owned by:
 *
 *     profiling.g4
 *     compiler/tooling
 *     runtime
 *
 * Observation must not select a profiler implementation.
 *
 * ============================================================================
 * 41. SCHEDULING / PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Observation may constrain or describe execution properties relevant to:
 *
 *     scheduling
 *     placement
 *     resource planning
 *
 * It does not perform scheduling or placement.
 *
 * Example:
 *
 *     observe execution {
 *         constraint observation::overhead <= budget;
 *         prefer locality;
 *     };
 *
 * The scheduler may consume the semantic constraint.
 *
 * ============================================================================
 * 42. RESILIENCE / RECOVERY INTEGRATION
 * ============================================================================
 *
 * Observation may be associated with:
 *
 *     retries
 *     recovery
 *     resilience
 *     checkpoint events
 *     fault events
 *
 * but this grammar does not implement those mechanisms.
 *
 * The semantic result can be consumed by:
 *
 *     resilience
 *     recovery
 *     checkpointing
 *     runtime
 *
 * ============================================================================
 * 43. SECURITY / PRIVACY INTEGRATION
 * ============================================================================
 *
 * Observation can expose sensitive execution information.
 *
 * Therefore downstream semantic/security analysis must be able to classify:
 *
 *     privacy
 *     provenance
 *     authorization
 *     confidentiality
 *     integrity
 *     data-flow implications
 *
 * The parser does not decide whether an observation is authorized.
 *
 * ============================================================================
 * 44. DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer vocabulary
 *     parser grammar
 *
 * parsing must produce equivalent:
 *
 *     token sequence
 *     parse structure
 *     source spans
 *     diagnostics
 *
 * Observation parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     random state
 *     environment variables
 *     filesystem state
 *     network state
 *     hardware availability
 *     CPU count
 *     GPU count
 *     QPU availability
 *     runtime state
 *
 * ============================================================================
 * 45. NO PARSER-TIME OBSERVATION
 * ============================================================================
 *
 * The parser MUST NOT:
 *
 *     collect telemetry;
 *     query hardware;
 *     inspect runtime counters;
 *     contact a tracing backend;
 *     contact a profiler;
 *     inspect a timeline;
 *     access a checkpoint;
 *     query resource availability.
 *
 * Parsing creates syntax structure only.
 *
 * ============================================================================
 * 46. NO RUNTIME SIDE EFFECTS
 * ============================================================================
 *
 * No semantic action, parser action, or embedded code may:
 *
 *     execute observation;
 *     allocate resources;
 *     mutate runtime state;
 *     create telemetry;
 *     emit events;
 *     open files;
 *     contact networks;
 *     invoke devices.
 *
 * ============================================================================
 * 47. ERROR CLASSIFICATION
 * ============================================================================
 *
 * Parser errors are restricted to malformed syntax.
 *
 * Examples of parser errors:
 *
 *     observe {;
 *     observe execution { requires ; };
 *     observe execution { constraint ; };
 *     observe execution { foo( ; };
 *
 * The following are NOT parser errors:
 *
 *     unavailable capability
 *     insufficient memory
 *     unavailable telemetry backend
 *     unsupported profiler
 *     unsupported hardware counter
 *     unavailable QPU observation capability
 *
 * Those belong to semantic/resource/capability/runtime diagnostics.
 *
 * ============================================================================
 * 48. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover malformed:
 *
 *     missing observe keyword
 *     missing subject where required by the selected form
 *     malformed expression
 *     malformed clause
 *     missing semicolon
 *     unbalanced braces
 *     malformed extension
 *     malformed requirement
 *     malformed constraint
 *     malformed preference
 *     malformed hint
 *     malformed trigger
 *
 * Semantic tests separately cover:
 *
 *     unknown capability
 *     unavailable capability
 *     unsatisfied resource requirement
 *     invalid observation target
 *     invalid privacy policy
 *     unsupported runtime realization
 *
 * These must not be misclassified as syntax failures.
 *
 * ============================================================================
 * 49. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test at minimum:
 *
 *     observe;
 *
 *     observe execution;
 *
 *     observe execution {};
 *
 *     observe metric("throughput");
 *
 *     observe qualified::name;
 *
 *     observe execution {
 *         requires capability("telemetry");
 *     };
 *
 *     observe execution {
 *         requires memory >= required_memory;
 *         constraint observation::overhead <= budget;
 *         prefer observation::detail;
 *         hint placement::locality;
 *     };
 *
 *     observe execution {
 *         when condition;
 *         trigger execution::completed;
 *         with execution::context;
 *     };
 *
 *     observe execution {
 *         custom::extension {
 *             property: value;
 *         };
 *     };
 *
 * ============================================================================
 * 50. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST demonstrate that the grammar remains structurally valid for:
 *
 *     many observation constructs;
 *     many clauses;
 *     deeply nested expressions;
 *     large expression-valued requirements;
 *     large qualified names;
 *     large sets of properties;
 *     large distributed observation specifications;
 *     many timeline associations;
 *     large classical workloads;
 *     large quantum workloads;
 *     large HDL/hardware workloads.
 *
 * Test limits are test-environment limits only.
 *
 * They MUST NOT become language-level observation capacities.
 *
 * ============================================================================
 * 51. HARD-CODING AUDIT
 * ============================================================================
 *
 * The following must NOT occur as grammar semantics:
 *
 *     MAX_OBSERVATIONS
 *     MAX_EVENTS
 *     MAX_METRICS
 *     MAX_SPANS
 *     MAX_TIMELINES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_COUNTERS
 *     MAX_SAMPLES
 *
 * A literal such as:
 *
 *     sample_count = 1024
 *
 * is program data.
 *
 * It does NOT establish:
 *
 *     MAX_SAMPLES = 1024
 *
 * ============================================================================
 * 52. COMPATIBILITY
 * ============================================================================
 *
 * This grammar preserves the canonical existing lexical vocabulary.
 *
 * No new lexer token is required for:
 *
 *     observation kinds
 *     metrics
 *     events
 *     traces
 *     profiles
 *     triggers
 *     correlation identifiers
 *     provenance identifiers
 *     future observation domains
 *
 * This deliberately reduces compatibility pressure on:
 *
 *     grammar/lexer/keywords.g4
 *
 * and:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Existing stable source spellings remain governed by the repository's
 * compatibility policy.
 *
 * ============================================================================
 * 53. INTEROPERABILITY
 * ============================================================================
 *
 * External systems such as:
 *
 *     OpenTelemetry
 *     Prometheus
 *     vendor profilers
 *     tracing systems
 *     hardware performance counters
 *
 * are interoperability/implementation concerns.
 *
 * They MUST NOT become parser-level authorities.
 *
 * A semantic adapter may lower Zamani observation intent to such systems.
 *
 * ============================================================================
 * 54. IR CONTRACT
 * ============================================================================
 *
 * This file introduces NO:
 *
 *     ObservationIR
 *     ExecutionObservationIR
 *     TelemetryIR
 *     RuntimeObservationIR
 *     QuantumObservationIR
 *     MTSObservationIR
 *
 * Observation intent remains part of the canonical semantic execution model
 * or associated semantic metadata.
 *
 * For quantum computation:
 *
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * remains unchanged.
 *
 * ============================================================================
 * 55. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes the semantic observation model after:
 *
 *     parsing
 *     AST construction
 *     semantic validation
 *     capability analysis
 *     resource analysis
 *     portability analysis
 *
 * The compiler may:
 *
 *     preserve observation;
 *     specialize observation;
 *     lower observation to supported runtime facilities;
 *     diagnose unsupported mandatory requirements;
 *     eliminate semantically irrelevant advisory hints.
 *
 * It must not silently discard mandatory observation requirements.
 *
 * ============================================================================
 * 56. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime implementation consumes validated compiled observation intent.
 *
 * Runtime responsibilities may include:
 *
 *     collection
 *     sampling
 *     aggregation
 *     buffering
 *     export
 *     correlation
 *     persistence
 *     resource management
 *
 * None of these occur during parsing.
 *
 * ============================================================================
 * 57. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Its implementation therefore requires no unsafe Rust.
 *
 * Generated and handwritten frontend implementations must remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * without requiring `unsafe`.
 *
 * ============================================================================
 * 58. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * `observation.g4` is complete when:
 *
 * [x] It is independent of `Execution`.
 * [x] It is independent of `Observability`.
 * [x] It is independent of `ExecutionTimelines`.
 * [x] It uses the canonical ZamaniLexer vocabulary.
 * [x] It reuses canonical Names.
 * [x] It reuses canonical Expressions.
 * [x] It introduces no lexer tokens.
 * [x] It introduces no second expression grammar.
 * [x] It introduces no resource grammar.
 * [x] It introduces no capability grammar.
 * [x] It introduces no runtime behavior.
 * [x] It introduces no execution IR.
 * [x] It introduces no quantum IR.
 * [x] It preserves `quantum::ir`.
 * [x] It contains no hardware limits.
 * [x] It contains no timeline limits.
 * [x] It contains no parser-time observation.
 * [x] It requires no unsafe Rust.
 * [x] It supports open-ended future observation concepts.
 * [x] It preserves requirement/constraint/preference/hint distinction.
 * [x] It preserves source-level intent for downstream semantic analysis.
 *
 * External integration remains required in the composition layer:
 *
 * [ ] Add `ExecutionObservation` to the execution composition imports.
 * [ ] Expose `observationConstruct` from `executionDomainDeclaration`
 *     or the appropriate execution-domain entry rule.
 * [ ] Add AST mapping in the canonical frontend AST.
 * [ ] Add semantic observation lowering.
 * [ ] Add positive/negative/boundary/scalability tests.
 * [ ] Resolve the existing `observability.g4 -> Execution` circular dependency
 *     before composing both grammars together.
 *
 * Those integration steps intentionally belong to their owning files and are
 * NOT duplicated here.
 *
 * ============================================================================
 * 59. FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Observation describes WHAT information the program wants associated with
 * execution.
 *
 * It does not describe:
 *
 *     HOW telemetry is collected;
 *     WHERE telemetry is stored;
 *     WHICH backend collects it;
 *     WHICH device generates it;
 *     HOW many resources exist;
 *     HOW many observations are possible.
 *
 * Therefore:
 *
 *     observation syntax
 *          !=
 *     telemetry implementation
 *
 *     source span
 *          !=
 *     runtime trace span
 *
 *     observation intent
 *          !=
 *     hardware capability
 *
 *     observation requirement
 *          !=
 *     resource allocation
 *
 *     parser
 *          !=
 *     runtime
 *
 * The resulting semantic program remains portable across:
 *
 *     atom / embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     edge
 *     future targets
 *
 * subject only to the actual program semantics, implementation capabilities,
 * security policy, and resources available at realization time.
 *
 * ============================================================================
 * END OF observation.g4
 * ============================================================================
 */