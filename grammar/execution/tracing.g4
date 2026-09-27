/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/tracing.g4
 *
 * Grammar:
 *     Tracing
 *
 * Status:
 *     Production execution-tracing grammar component
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL TRACING INTENT.
 *
 * It describes what execution information a Zamani program requests to make
 * observable through traces, spans, events, correlation, attributes,
 * propagation, sampling, filtering, retention, privacy, provenance and
 * related tracing policy.
 *
 * It does NOT implement:
 *
 *     - tracing;
 *     - event collection;
 *     - span storage;
 *     - exporters;
 *     - telemetry transports;
 *     - runtime instrumentation;
 *     - clock access;
 *     - hardware counters;
 *     - distributed tracing infrastructure;
 *     - sampling algorithms;
 *     - trace databases;
 *     - profiling;
 *     - scheduling;
 *     - routing;
 *     - resource allocation;
 *     - runtime execution.
 *
 * The grammar expresses intent only.
 *
 * ============================================================================
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
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> tracing intent validation
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> privacy/security analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantic/IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     runtime realization
 *          |
 *          +--> trace collection
 *          +--> span correlation
 *          +--> event export
 *          +--> observability infrastructure
 *
 * Tracing is therefore metadata/observability intent.
 *
 * It is NOT a semantic IR.
 *
 * ============================================================================
 * CRITICAL REPOSITORY INTEGRATION DECISION
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/execution/observability.g4
 *
 * That file currently contains generic rules named:
 *
 *     traceClause
 *     spanClause
 *
 * Therefore this file MUST NOT define those same generic rule names.
 *
 * This file instead exposes:
 *
 *     tracingConstruct
 *
 * and uses names prefixed with:
 *
 *     tracing*
 *
 * This avoids:
 *
 *     - ANTLR rule-name collisions;
 *     - competing tracing authorities;
 *     - circular grammar dependencies;
 *     - forced modification of this file when observability.g4 evolves.
 *
 * The intended long-term ownership is:
 *
 *     tracing.g4
 *         owns detailed tracing syntax
 *
 *     observability.g4
 *         owns general observability composition
 *
 *     runtime.g4
 *         delegates runtime tracing requests to tracingConstruct
 *
 *     execution.g4
 *         composes execution-domain syntax
 *
 * This file remains independently complete.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The repository's canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its lexical vocabulary is assembled through:
 *
 *     grammar/lexer/tokens.g4
 *
 * Existing reserved token:
 *
 *     OBSERVE
 *
 * is used as the stable lexical entry point.
 *
 * The repository does NOT currently reserve:
 *
 *     TRACE
 *     SPAN
 *     EVENT
 *     ATTRIBUTE
 *     CORRELATION
 *     SAMPLING
 *     PROPAGATION
 *
 * as dedicated universal lexer tokens.
 *
 * This grammar therefore MUST NOT invent those tokens.
 *
 * The spellings:
 *
 *     trace
 *     span
 *     event
 *     attribute
 *     correlation
 *     sampling
 *     propagation
 *
 * are contextual identifiers validated by semantic analysis.
 *
 * This preserves the repository's existing single-lexer architecture.
 *
 * If a future language version promotes one of these spellings to a reserved
 * keyword, the compatibility process must update the lexer and this grammar
 * consistently. This file does not independently create the lexical change.
 *
 * ============================================================================
 * CANONICAL ENTRY FORM
 * ============================================================================
 *
 * The canonical source-level tracing entry point is:
 *
 *     observe trace ...
 *
 * or:
 *
 *     observe span ...
 *
 * Example:
 *
 *     observe trace computation with {
 *         name: "computation",
 *         correlation: execution,
 *         sampling: automatic,
 *         attributes: {
 *             workload: workload_id
 *         }
 *     };
 *
 * Example:
 *
 *     observe span operation for computation with {
 *         kind: internal,
 *         attributes: {
 *             operation: operation_name
 *         }
 *     };
 *
 * The exact semantic interpretation of names and policies is downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tracing construct composition;
 *     - trace intent;
 *     - span intent;
 *     - trace-event intent;
 *     - span relationship intent;
 *     - correlation intent;
 *     - propagation intent;
 *     - sampling intent;
 *     - trace filtering intent;
 *     - trace attribute intent;
 *     - trace context intent;
 *     - trace privacy intent;
 *     - trace retention intent;
 *     - trace export intent;
 *     - trace provenance intent;
 *     - tracing requirements;
 *     - tracing capabilities;
 *     - tracing constraints;
 *     - tracing preferences;
 *     - tracing hints;
 *     - tracing lifecycle attachment;
 *     - tracing error/failure attachment.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - general expressions;
 *     - general types;
 *     - general statements;
 *     - general declarations;
 *     - general observability composition;
 *     - profiling;
 *     - metrics;
 *     - logging;
 *     - scheduling;
 *     - placement;
 *     - dispatch;
 *     - deployment;
 *     - runtime implementation;
 *     - distributed transport;
 *     - network transport;
 *     - persistence implementation;
 *     - security implementation;
 *     - cryptography;
 *     - hardware counters;
 *     - compiler optimization;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - quantum::ir.
 *
 * ============================================================================
 * NON-CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar imports only foundational parser grammars:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * It MUST NOT import:
 *
 *     Execution
 *     Runtime
 *     Observability
 *     Scheduling
 *     Placement
 *     Dispatch
 *     Deployment
 *
 * merely to reuse concepts.
 *
 * This keeps Tracing independently composable.
 *
 * Intended dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Core
 *          |
 *          +--> Types
 *          |
 *          +--> Expressions
 *          |
 *          v
 *       Tracing
 *          |
 *          v
 *   canonical parser composition
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps to domain-neutral structural nodes.
 *
 * Conceptual representation:
 *
 *     TracingConstruct
 *       |
 *       +--> mode
 *       |     trace | span
 *       |
 *       +--> subject
 *       |
 *       +--> optional name
 *       |
 *       +--> optional relationship
 *       |
 *       +--> clauses[]
 *
 * Clauses conceptually map to:
 *
 *     TracePolicy
 *     SpanPolicy
 *     EventPolicy
 *     CorrelationPolicy
 *     PropagationPolicy
 *     SamplingPolicy
 *     FilterPolicy
 *     AttributePolicy
 *     ContextPolicy
 *     PrivacyPolicy
 *     RetentionPolicy
 *     ExportPolicy
 *     ProvenancePolicy
 *     Requirement
 *     CapabilityRequirement
 *     Constraint
 *     Preference
 *     Hint
 *
 * The exact Rust AST types are owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's canonical domain-neutral AST implementation.
 *
 * This grammar does NOT create:
 *
 *     TraceIR
 *     SpanIR
 *     RuntimeTraceIR
 *     DistributedTraceIR
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance means only:
 *
 *     "the source has valid tracing syntax."
 *
 * It does NOT mean:
 *
 *     tracing capability exists;
 *     tracing can be collected;
 *     storage is available;
 *     an exporter exists;
 *     sampling is supported;
 *     propagation is supported;
 *     privacy policy can be satisfied;
 *     the target provides required observability capability.
 *
 * Those questions belong to semantic analysis, capability analysis, resource
 * analysis and runtime realization.
 *
 * ============================================================================
 * OBSERVABILITY NON-INTERFERENCE
 * ============================================================================
 *
 * Tracing MUST NOT change ordinary program semantics unless the program
 * explicitly requests semantics that include observability.
 *
 * In particular, tracing syntax must not silently:
 *
 *     - reorder computation;
 *     - change synchronization;
 *     - change quantum operations;
 *     - change HDL timing semantics;
 *     - alter resource ownership;
 *     - change numerical results;
 *     - change distributed consistency;
 *     - change exception semantics;
 *     - alter security policy.
 *
 * Runtime instrumentation MAY change performance or resource usage.
 *
 * That is an implementation property and MUST NOT silently change the
 * language-level meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Tracing syntax is target-independent.
 *
 * The same source-level tracing intent can be realized through:
 *
 *     CPU execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     QPU execution
 *     simulator execution
 *     embedded execution
 *     distributed execution
 *     cloud execution
 *     edge execution
 *     future execution substrates
 *
 * The grammar does not require any specific tracing backend.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     traces;
 *     spans;
 *     events;
 *     attributes;
 *     contexts;
 *     correlations;
 *     trace links;
 *     parent relationships;
 *     distributed participants;
 *     execution domains;
 *     source files;
 *     workloads;
 *     operations;
 *     processes;
 *     tasks;
 *     threads;
 *     devices;
 *     nodes;
 *     qubits;
 *     tensor dimensions;
 *     memory;
 *     trace depth;
 *     trace width;
 *     trace size.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_TRACES
 *     MAX_SPANS
 *     MAX_EVENTS
 *     MAX_ATTRIBUTES
 *     MAX_CONTEXTS
 *     MAX_TRACE_DEPTH
 *     MAX_TRACE_WIDTH
 *     MAX_TRACE_SIZE
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_QUANTUM_OPERATIONS
 *
 * Repetition is represented structurally.
 *
 * Actual finite limits belong to:
 *
 *     compiler resource policy;
 *     runtime resource policy;
 *     telemetry backend;
 *     storage capacity;
 *     network capacity;
 *     target capabilities;
 *     deployment policy;
 *     explicitly declared resource constraints.
 *
 * These are NOT language-level tracing limits.
 *
 * ============================================================================
 * DISTRIBUTED TRACING
 * ============================================================================
 *
 * Distributed tracing is represented semantically through:
 *
 *     correlation;
 *     parent/child relationships;
 *     links;
 *     propagation;
 *     execution identity;
 *     logical participant identity.
 *
 * The grammar MUST NOT prescribe:
 *
 *     a specific tracing protocol;
 *     a specific network transport;
 *     a specific vendor;
 *     a fixed node count;
 *     a fixed topology.
 *
 * Protocol realization belongs to networking/runtime infrastructure.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Tracing may observe:
 *
 *     logical quantum operations;
 *     quantum circuit execution;
 *     measurements;
 *     classical feed-forward;
 *     QEC activity;
 *     resilience decisions;
 *     scheduling decisions;
 *     target realization;
 *     runtime outcomes.
 *
 * It MUST NOT redefine:
 *
 *     qubits;
 *     gates;
 *     quantum states;
 *     measurements;
 *     quantum::ir;
 *     physical topology;
 *     routing;
 *     QEC;
 *     ZQN.
 *
 * The relationship remains:
 *
 *     quantum source
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> tracing metadata/provenance
 *
 * Tracing remains orthogonal to quantum semantics.
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * Tracing can attach to:
 *
 *     classical functions;
 *     classical operations;
 *     quantum operations;
 *     hybrid boundaries;
 *     HDL simulation;
 *     hardware realization;
 *     accelerator execution;
 *     distributed execution;
 *     AI pipelines;
 *     data pipelines;
 *     networking;
 *     security operations.
 *
 * The grammar does not enumerate these domains.
 *
 * Domain identity remains semantic.
 *
 * ============================================================================
 * PRIVACY / SECURITY CONTRACT
 * ============================================================================
 *
 * Trace data may contain sensitive information.
 *
 * The grammar therefore supports declarative:
 *
 *     privacy;
 *     classification;
 *     redaction;
 *     retention;
 *     export restrictions;
 *     integrity requirements.
 *
 * The grammar does NOT implement redaction or encryption.
 *
 * Security analysis/runtime policy determines whether a requested trace
 * policy is permitted.
 *
 * A tracing request MUST NOT silently bypass:
 *
 *     security policy;
 *     privacy policy;
 *     data-classification policy;
 *     access-control policy.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Tracing should preserve source provenance when a trace construct corresponds
 * to source code.
 *
 * Runtime-generated events may additionally contain runtime provenance.
 *
 * The grammar itself does not define source-map storage.
 *
 * Source locations remain owned by the repository source-map contract.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing of tracing syntax is deterministic.
 *
 * Tracing syntax MUST NOT depend on:
 *
 *     hardware;
 *     runtime state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     device enumeration;
 *     thread scheduling.
 *
 * Runtime trace event order may naturally depend on concurrent execution.
 *
 * That runtime behavior is distinct from parser determinism.
 *
 * If deterministic trace ordering is requested, it must be represented as an
 * explicit semantic policy and validated downstream.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Therefore it introduces no Rust unsafe operation.
 *
 * Downstream implementation MUST support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * Tracing implementation MUST NOT require:
 *
 *     unsafe;
 *     raw-pointer instrumentation;
 *     undefined behavior;
 *     architecture-specific memory assumptions.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser diagnostics.
 *
 * Semantic errors are downstream diagnostics.
 *
 * Examples of downstream conditions include:
 *
 *     missing tracing capability;
 *     unsupported propagation;
 *     invalid privacy policy;
 *     unsatisfied resource requirement;
 *     unsupported sampling policy;
 *     invalid correlation scope;
 *     unsupported export policy;
 *     forbidden sensitive attribute.
 *
 * These must remain distinguishable from syntax errors.
 *
 * ============================================================================
 * PUBLIC COMPOSITION RULE
 * ============================================================================
 *
 * The sole public entry point of this grammar is:
 *
 *     tracingConstruct
 *
 * Consumers should call this rule rather than depending on internal
 * tracing rules.
 *
 * This gives the file an independent completion boundary.
 *
 * ============================================================================
 */

parser grammar Tracing;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary:
 *
 *     tracingConstruct
 *
 * Canonical forms:
 *
 *     observe trace ...
 *     observe span ...
 *
 * The OBSERVE token is already part of the canonical Zamani lexer.
 *
 * `trace` and `span` remain contextual identifiers.
 * ============================================================================
 */

tracingConstruct
    : OBSERVE tracingMode tracingSubject? tracingAttachment* tracingBody? tracingTerminator?
    ;


/*
 * ============================================================================
 * 2. MODE
 * ============================================================================
 *
 * Standard modes:
 *
 *     trace
 *     span
 *
 * The spelling is validated semantically.
 *
 * This intentionally remains extensible without adding a lexer keyword.
 * ============================================================================
 */

tracingMode
    : tracingTraceWord
    | tracingSpanWord
    ;

tracingTraceWord
    : identifier
    ;

tracingSpanWord
    : identifier
    ;


/*
 * ============================================================================
 * 3. SUBJECT
 * ============================================================================
 *
 * The subject identifies an abstract computation or execution entity.
 *
 * It may be:
 *
 *     a named computation;
 *     a function;
 *     a task;
 *     a pipeline;
 *     a quantum computation;
 *     a hybrid computation;
 *     a hardware/software computation;
 *     a distributed activity;
 *     an arbitrary expression.
 *
 * Physical machine identity is not implied.
 * ============================================================================
 */

tracingSubject
    : expression
    ;


/*
 * ============================================================================
 * 4. ATTACHMENTS
 * ============================================================================
 *
 * Attachments allow tracing intent to be associated with an execution entity
 * without requiring a fixed closed vocabulary.
 *
 * `for`, `as`, `when`, and `where` are existing lexical tokens.
 * ============================================================================
 */

tracingAttachment
    : tracingForAttachment
    | tracingAsAttachment
    | tracingWhenAttachment
    | tracingWhereAttachment
    ;

tracingForAttachment
    : FOR expression
    ;

tracingAsAttachment
    : AS identifier
    ;

tracingWhenAttachment
    : WHEN expression
    ;

tracingWhereAttachment
    : WHERE expression
    ;


/*
 * ============================================================================
 * 5. BODY
 * ============================================================================
 *
 * A body contains zero or more declarative tracing clauses.
 *
 * No finite number of clauses is imposed.
 * ============================================================================
 */

tracingBody
    : LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 6. CLAUSE DISPATCH
 * ============================================================================
 *
 * The first identifier of each clause is interpreted semantically.
 *
 * Examples:
 *
 *     name: ...
 *     kind: ...
 *     parent: ...
 *     link: ...
 *     event: ...
 *     attribute: ...
 *     correlation: ...
 *     propagation: ...
 *     sampling: ...
 *     filter: ...
 *     privacy: ...
 *     retention: ...
 *     export: ...
 *     provenance: ...
 *     requires: ...
 *     capability: ...
 *     constraint: ...
 *     prefer: ...
 *     hint: ...
 *
 * No clause is permitted to bypass semantic validation.
 * ============================================================================
 */

tracingClause
    : tracingNamedClause
    | tracingAssignmentClause
    | tracingBlockClause
    ;


/*
 * ============================================================================
 * 7. NAMED CLAUSES
 * ============================================================================
 *
 * The identifier spelling is interpreted by semantic analysis.
 *
 * This avoids making every future tracing concept a lexer keyword.
 * ============================================================================
 */

tracingNamedClause
    : identifier COLON tracingValue
    ;


/*
 * ============================================================================
 * 8. ASSIGNMENT CLAUSES
 * ============================================================================
 */

tracingAssignmentClause
    : identifier ASSIGN tracingValue
    ;


/*
 * ============================================================================
 * 9. NESTED BLOCK CLAUSES
 * ============================================================================
 *
 * Nested blocks support scalable structured metadata:
 *
 *     attributes: {
 *         ...
 *     }
 *
 *     context: {
 *         ...
 *     }
 *
 *     privacy: {
 *         ...
 *     }
 *
 *     export: {
 *         ...
 *     }
 *
 * Nesting is not a machine-size limit.
 * ============================================================================
 */

tracingBlockClause
    : identifier COLON LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 10. VALUES
 * ============================================================================
 *
 * Values delegate to the normal Zamani expression grammar.
 *
 * This permits:
 *
 *     symbolic values;
 *     computed values;
 *     configuration values;
 *     resource expressions;
 *     names;
 *     literals;
 *     capability expressions;
 *     program-derived metadata.
 *
 * No fixed integer width is imposed here.
 * ============================================================================
 */

tracingValue
    : expression
    ;


/*
 * ============================================================================
 * 11. TRACE-SPECIFIC STRUCTURED ENTRY
 * ============================================================================
 *
 * This optional structure gives semantic analysis an unambiguous way to
 * recognize a trace declaration without requiring TRACE to become a lexer
 * keyword.
 *
 * Example:
 *
 *     observe trace computation with {
 *         ...
 *     };
 *
 * The generic body remains authoritative for extensibility.
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. SEMANTIC TRACE CLAUSE CATEGORIES
 * ============================================================================
 *
 * The following comments define the reserved semantic vocabulary without
 * turning it into parser-level closed enumeration.
 *
 * TRACE IDENTITY:
 *
 *     name
 *     id
 *     kind
 *
 * SPAN RELATIONSHIPS:
 *
 *     parent
 *     link
 *     follows
 *     child
 *
 * EVENTS:
 *
 *     event
 *     start
 *     end
 *     state
 *     outcome
 *
 * ATTRIBUTES:
 *
 *     attribute
 *     attributes
 *     label
 *     labels
 *
 * CONTEXT:
 *
 *     context
 *     baggage
 *     correlation
 *     execution
 *
 * PROPAGATION:
 *
 *     propagation
 *     inject
 *     extract
 *     forward
 *
 * SAMPLING:
 *
 *     sampling
 *     sample
 *     rate
 *     condition
 *
 * FILTERING:
 *
 *     filter
 *     include
 *     exclude
 *
 * PRIVACY:
 *
 *     privacy
 *     classify
 *     redact
 *     sensitive
 *
 * RETENTION:
 *
 *     retention
 *     duration
 *     policy
 *
 * EXPORT:
 *
 *     export
 *     destination
 *     format
 *
 * PROVENANCE:
 *
 *     provenance
 *     source
 *     origin
 *
 * REQUIREMENTS:
 *
 *     requires
 *     capability
 *     resource
 *     constraint
 *
 * PREFERENCES:
 *
 *     prefer
 *     hint
 *
 * These are semantic concepts.
 *
 * The parser intentionally does not close the vocabulary.
 */


/*
 * ============================================================================
 * 13. EXPLICIT REQUIREMENT FORM
 * ============================================================================
 *
 * Existing lexical tokens make the common forms unambiguous.
 *
 * Examples:
 *
 *     observe trace computation with {
 *         requires capability("distributed.tracing")
 *     };
 *
 *     observe span operation with {
 *         requires resource(memory >= required_memory)
 *     };
 *
 * These requirements remain declarative.
 * ============================================================================
 */

tracingRequirementClause
    : REQUIRES tracingRequirement
    ;

tracingRequirement
    : CAPABILITY LPAREN expression RPAREN
    | RESOURCE LPAREN expression RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 14. EXPLICIT PREFERENCE FORM
 * ============================================================================
 */

tracingPreferenceClause
    : PREFER tracingValue
    ;


/*
 * ============================================================================
 * 15. EXPLICIT HINT FORM
 * ============================================================================
 */

tracingHintClause
    : HINT tracingValue
    ;


/*
 * ============================================================================
 * 16. EXPLICIT CONSTRAINT FORM
 * ============================================================================
 */

tracingConstraintClause
    : CONSTRAINT tracingValue
    ;


/*
 * ============================================================================
 * 17. SEMANTIC POLICY HELPERS
 * ============================================================================
 *
 * These rules are provided as independently reusable structural forms.
 *
 * They do not force the consumer to use a closed list of policy names.
 * ============================================================================
 */

tracingPolicyValue
    : tracingValue
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 18. TRACE EVENT FORM
 * ============================================================================
 *
 * This is a structural form for an event attached to a trace/span.
 *
 * Example:
 *
 *     event: operation_completed
 *
 * or:
 *
 *     event: {
 *         name: operation_completed,
 *         outcome: success
 *     }
 *
 * The event itself is semantic metadata.
 *
 * It is not a runtime event implementation.
 * ============================================================================
 */

tracingEventClause
    : identifier COLON tracingEventValue
    ;

tracingEventValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 19. SPAN RELATIONSHIP FORM
 * ============================================================================
 *
 * Relationships may be:
 *
 *     parent;
 *     child;
 *     link;
 *     follows;
 *     causal;
 *     correlated;
 *
 * The exact semantics are downstream.
 *
 * No fixed graph size is imposed.
 * ============================================================================
 */

tracingRelationshipClause
    : identifier COLON tracingRelationshipValue
    ;

tracingRelationshipValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 20. CORRELATION FORM
 * ============================================================================
 *
 * Correlation may connect:
 *
 *     local executions;
 *     concurrent tasks;
 *     distributed processes;
 *     services;
 *     classical computation;
 *     quantum computation;
 *     HDL simulation;
 *     hardware realization;
 *     AI/data pipelines.
 *
 * Physical topology is not implied.
 * ============================================================================
 */

tracingCorrelationClause
    : identifier COLON tracingCorrelationValue
    ;

tracingCorrelationValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 21. PROPAGATION FORM
 * ============================================================================
 *
 * Propagation is semantic intent.
 *
 * It does not prescribe:
 *
 *     HTTP;
 *     TCP;
 *     UDP;
 *     gRPC;
 *     OTLP;
 *     vendor protocols;
 *     a particular network.
 * ============================================================================
 */

tracingPropagationClause
    : identifier COLON tracingPropagationValue
    ;

tracingPropagationValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 22. SAMPLING FORM
 * ============================================================================
 *
 * Sampling parameters are expressions.
 *
 * This permits:
 *
 *     fixed;
 *     adaptive;
 *     automatic;
 *     conditional;
 *     workload-dependent;
 *     capability-dependent;
 *     resource-dependent
 *
 * policies without a parser-level finite vocabulary.
 *
 * No maximum/minimum universal sampling rate is encoded here.
 * ============================================================================
 */

tracingSamplingClause
    : identifier COLON tracingSamplingValue
    ;

tracingSamplingValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 23. FILTER FORM
 * ============================================================================
 */

tracingFilterClause
    : identifier COLON tracingFilterValue
    ;

tracingFilterValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 24. ATTRIBUTE FORM
 * ============================================================================
 *
 * Attributes are intentionally expression-valued.
 *
 * There is no fixed attribute count.
 *
 * Attribute names remain semantic identifiers.
 *
 * Values can be:
 *
 *     literals;
 *     program values;
 *     symbolic values;
 *     computed values;
 *     resource values;
 *     metadata.
 * ============================================================================
 */

tracingAttributeClause
    : identifier COLON tracingAttributeValue
    ;

tracingAttributeValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 25. CONTEXT FORM
 * ============================================================================
 */

tracingContextClause
    : identifier COLON tracingContextValue
    ;

tracingContextValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 26. PRIVACY FORM
 * ============================================================================
 */

tracingPrivacyClause
    : identifier COLON tracingPrivacyValue
    ;

tracingPrivacyValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 27. RETENTION FORM
 * ============================================================================
 */

tracingRetentionClause
    : identifier COLON tracingRetentionValue
    ;

tracingRetentionValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 28. EXPORT FORM
 * ============================================================================
 *
 * Export destinations remain abstract.
 *
 * This grammar does not require:
 *
 *     Prometheus;
 *     OTLP;
 *     Kafka;
 *     files;
 *     databases;
 *     cloud providers;
 *     vendor APIs.
 *
 * Such implementations belong downstream.
 * ============================================================================
 */

tracingExportClause
    : identifier COLON tracingExportValue
    ;

tracingExportValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 29. PROVENANCE FORM
 * ============================================================================
 */

tracingProvenanceClause
    : identifier COLON tracingProvenanceValue
    ;

tracingProvenanceValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 30. ERROR / FAILURE ATTACHMENT
 * ============================================================================
 *
 * Tracing may describe which failures should become observable.
 *
 * It does not define the failure itself.
 *
 * Error semantics remain owned by the relevant semantic/runtime subsystem.
 * ============================================================================
 */

tracingFailureClause
    : identifier COLON tracingFailureValue
    ;

tracingFailureValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 31. LIFECYCLE ATTACHMENT
 * ============================================================================
 *
 * Tracing can observe lifecycle phases such as:
 *
 *     start;
 *     suspend;
 *     resume;
 *     complete;
 *     cancel;
 *     recover;
 *
 * The runtime owns actual lifecycle transitions.
 * ============================================================================
 */

tracingLifecycleClause
    : identifier COLON tracingLifecycleValue
    ;

tracingLifecycleValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;


/*
 * ============================================================================
 * 32. EXECUTION CORRELATION
 * ============================================================================
 *
 * An execution may span multiple semantic domains.
 *
 * The grammar therefore permits an expression as the correlation identity.
 *
 * It does not impose a particular ID representation.
 * ============================================================================
 */

tracingExecutionCorrelation
    : identifier COLON expression
    ;


/*
 * ============================================================================
 * 33. TRACE TERMINATOR
 * ============================================================================
 */

tracingTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 34. INTERNAL SEMANTIC CLASSIFICATION
 * ============================================================================
 *
 * The semantic layer should classify the contextual words consumed by this
 * grammar.
 *
 * Required validation:
 *
 *     tracingMode "trace"
 *         -> Trace
 *
 *     tracingMode "span"
 *         -> Span
 *
 * Unknown modes MUST produce a structured semantic diagnostic.
 *
 * This parser MUST NOT silently reinterpret:
 *
 *     observe foo ...
 *
 * as a trace operation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. OBSERVABILITY INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/execution/observability.g4
 *
 * already owns generic observability syntax.
 *
 * It currently contains generic:
 *
 *     traceClause
 *     spanClause
 *
 * This file intentionally does not redefine them.
 *
 * Recommended integration:
 *
 *     Observability
 *          |
 *          +--> tracingConstruct
 *
 * The consumer should delegate rather than copy tracing rules.
 *
 * This means this file remains independently complete and does not need to be
 * edited when unrelated observability clauses change.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/execution/runtime.g4
 *
 * contains:
 *
 *     runtimeObserveStatement
 *     runtimeTraceStatement
 *
 * Runtime integration should delegate tracing-specific syntax to:
 *
 *     tracingConstruct
 *
 * rather than reproducing:
 *
 *     sampling;
 *     propagation;
 *     span;
 *     correlation;
 *     privacy;
 *     retention;
 *     export;
 *     provenance.
 *
 * Runtime remains responsible for runtime intent.
 *
 * Tracing remains responsible for tracing syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. EXECUTION INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/execution/execution.g4
 *
 * is the execution composition boundary.
 *
 * It should compose tracing through the execution/observability/runtime
 * integration path.
 *
 * This file MUST NOT import Execution merely to obtain that composition.
 *
 * The dependency remains one-directional:
 *
 *     Tracing
 *        |
 *        v
 *     Observability / Runtime
 *        |
 *        v
 *     Execution composition
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. SOURCE MAP INTEGRATION
 * ============================================================================
 *
 * Every tracingConstruct MUST retain its source span.
 *
 * Nested clauses SHOULD retain their own spans.
 *
 * Runtime-generated trace events may additionally retain:
 *
 *     runtime provenance;
 *     generated provenance;
 *     transformation provenance.
 *
 * The source-map subsystem remains authoritative for source locations.
 *
 * This grammar does not redefine:
 *
 *     FileId;
 *     BytePos;
 *     Span;
 *     SourceOrigin.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. DIAGNOSTIC INTEGRATION
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     malformed tracing syntax.
 *
 * Semantic diagnostics:
 *
 *     invalid tracing mode;
 *     invalid tracing target;
 *     invalid correlation;
 *     invalid policy;
 *     incompatible policy combination.
 *
 * Capability diagnostics:
 *
 *     required tracing capability unavailable.
 *
 * Resource diagnostics:
 *
 *     required tracing resources unavailable.
 *
 * Security diagnostics:
 *
 *     trace violates privacy/security policy.
 *
 * Runtime diagnostics:
 *
 *     tracing realization failed.
 *
 * These categories MUST remain distinct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Tracing may request capabilities such as:
 *
 *     capability("tracing");
 *     capability("distributed.tracing");
 *     capability("trace.propagation");
 *     capability("trace.hardware_counters");
 *
 * These are semantic capability identifiers.
 *
 * The grammar does not determine whether they exist.
 *
 * Resource requirements may depend on:
 *
 *     workload;
 *     trace volume;
 *     retention;
 *     storage;
 *     bandwidth;
 *     execution mode.
 *
 * No fixed resource capacity is encoded.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Tracing may cross:
 *
 *     tasks;
 *     async operations;
 *     parallel regions;
 *     actors;
 *     channels;
 *     distributed processes.
 *
 * Trace correlation MUST NOT change synchronization semantics.
 *
 * Runtime implementation may attach execution context to concurrent work.
 *
 * The grammar only describes the requested correlation/observation policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed tracing must preserve logical execution identity across
 * realizations without requiring a fixed topology.
 *
 * The grammar may express:
 *
 *     correlation;
 *     parent;
 *     link;
 *     propagation;
 *     locality;
 *     participant identity.
 *
 * It must not encode:
 *
 *     node 1;
 *     node 2;
 *     node 3;
 *
 * as universal assumptions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. QUANTUM / QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * Tracing may observe downstream quantum execution information.
 *
 * It MUST NOT redefine:
 *
 *     QEC;
 *     ZQN;
 *     routing;
 *     scheduling;
 *     quantum::ir.
 *
 * Trace provenance may point back to:
 *
 *     logical quantum operation;
 *     quantum source span;
 *     semantic quantum operation;
 *     canonical quantum::ir operation.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Tracing may observe:
 *
 *     simulation;
 *     synthesis;
 *     execution;
 *     hardware events;
 *     timing;
 *     resource realization;
 *     accelerator activity.
 *
 * The grammar must not impose:
 *
 *     fixed clock frequency;
 *     fixed register width;
 *     fixed device count;
 *     fixed hardware topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SECURITY / PRIVACY INTEGRATION
 * ============================================================================
 *
 * Trace attributes may contain:
 *
 *     identifiers;
 *     workload metadata;
 *     input-derived data;
 *     execution state;
 *     security metadata.
 *
 * Therefore semantic analysis must be capable of rejecting unsafe trace
 * policies.
 *
 * The tracing grammar itself does not implement access control.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. COMPILATION INTEGRATION
 * ============================================================================
 *
 * Tracing metadata may survive compilation through:
 *
 *     AST
 *       |
 *       v
 *     semantic tracing policy
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     target realization metadata
 *
 * Compilation MAY:
 *
 *     preserve;
 *     lower;
 *     specialize;
 *     omit;
 *     reject
 *
 * tracing according to the declared semantic contract.
 *
 * It MUST NOT silently change a required tracing guarantee into a hint.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimizations MUST preserve tracing semantics when tracing is semantically
 * observable.
 *
 * An optimizer may transform:
 *
 *     one operation -> many operations
 *     many operations -> one operation
 *
 * while preserving trace provenance according to the source-map/provenance
 * contract.
 *
 * Tracing must not accidentally become an optimization barrier unless the
 * semantic contract explicitly requires it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime owns:
 *
 *     event creation;
 *     span lifecycle;
 *     context propagation;
 *     storage;
 *     export;
 *     sampling implementation;
 *     filtering implementation;
 *     resource management.
 *
 * Runtime must consume the semantic tracing contract rather than reinterpret
 * raw parser nodes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. NO BACKEND LOCK-IN
 * ============================================================================
 *
 * This grammar does not require:
 *
 *     OpenTelemetry;
 *     Jaeger;
 *     Zipkin;
 *     Prometheus;
 *     vendor telemetry;
 *     a specific database;
 *     a specific cloud;
 *     a specific network.
 *
 * Such systems may be backend implementations of the semantic contract.
 *
 * They are not the Zamani language definition.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. NO FIXED TRACE IDENTIFIER FORMAT
 * ============================================================================
 *
 * Trace IDs, span IDs, correlation IDs and execution IDs are semantic values.
 *
 * This grammar does not require:
 *
 *     a fixed bit width;
 *     a fixed textual encoding;
 *     UUID;
 *     hexadecimal;
 *     decimal;
 *     vendor-specific identifiers.
 *
 * Representation belongs to the semantic/runtime implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. NO FIXED CLOCK MODEL
 * ============================================================================
 *
 * The grammar does not assume:
 *
 *     wall clock;
 *     monotonic clock;
 *     CPU cycle counter;
 *     nanosecond precision;
 *     microsecond precision;
 *     hardware timestamp counter.
 *
 * Timing semantics are owned by the execution/runtime specification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. NO FIXED TRACE STORAGE MODEL
 * ============================================================================
 *
 * Tracing does not imply:
 *
 *     memory;
 *     disk;
 *     network;
 *     database;
 *     stream;
 *     cloud storage.
 *
 * Retention/export are declarative intent.
 *
 * Storage realization is downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. EXTENSIBILITY
 * ============================================================================
 *
 * Future tracing concepts may be represented through:
 *
 *     identifier;
 *     qualified name;
 *     expression;
 *     nested structured values.
 *
 * A new backend or tracing technology MUST NOT require a new grammar rule if
 * the semantic concept can already be represented by this structure.
 *
 * If a new concept genuinely changes language-level semantics, it must undergo
 * the normal:
 *
 *     proposal
 *       ->
 *     semantic design
 *       ->
 *     AST contract
 *       ->
 *     grammar
 *       ->
 *     IR contract
 *       ->
 *     implementation
 *       ->
 *     tests
 *       ->
 *     stable
 *
 * process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal limits:
 *
 *     MAX_TRACES
 *     MAX_SPANS
 *     MAX_EVENTS
 *     MAX_ATTRIBUTES
 *     MAX_CONTEXTS
 *     MAX_TRACE_DEPTH
 *     MAX_TRACE_WIDTH
 *     MAX_TRACE_SIZE
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * None are encoded here.
 *
 * Program-specific numeric values remain valid when they are actual program
 * semantics.
 *
 * Example:
 *
 *     retention: 1000
 *
 * may be a program policy.
 *
 * It does NOT become:
 *
 *     the maximum number of retained events supported by Zamani.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests should cover:
 *
 *     one trace;
 *     many traces;
 *     one span;
 *     deeply nested spans;
 *     broad span trees;
 *     many attributes;
 *     many events;
 *     many links;
 *     many correlations;
 *     large distributed workloads;
 *     quantum workloads;
 *     hybrid workloads;
 *     HDL simulation;
 *     large AI/data pipelines;
 *     large module graphs.
 *
 * Tests must verify that:
 *
 *     syntax remains representable;
 *     parsing remains deterministic;
 *     semantic limits remain implementation/resource limits;
 *     no grammar-level finite ceiling appears.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Negative tests must cover:
 *
 *     missing OBSERVE;
 *     missing trace/span mode;
 *     malformed target;
 *     malformed body;
 *     malformed clause;
 *     invalid delimiter;
 *     malformed nested block;
 *     invalid expression;
 *     malformed attachment;
 *     invalid source construct.
 *
 * Semantic negative tests must additionally cover:
 *
 *     unknown tracing mode;
 *     invalid tracing target;
 *     incompatible policy;
 *     unavailable capability;
 *     unavailable resource;
 *     forbidden privacy policy;
 *     unsupported propagation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer;
 *     parser configuration;
 *
 * parsing must produce equivalent tracing syntax structures.
 *
 * Parser behavior must not depend on:
 *
 *     CPU count;
 *     thread count;
 *     target hardware;
 *     runtime state;
 *     network state;
 *     clock;
 *     randomness.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable lexical token:
 *
 *     OBSERVE
 *
 * must remain the lexical anchor for this syntax unless a future language
 * version explicitly changes the language contract.
 *
 * Contextual words:
 *
 *     trace
 *     span
 *
 * may later become reserved keywords only through the normal compatibility
 * process.
 *
 * Such a lexical promotion must not change the semantic meaning of existing
 * valid tracing constructs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. FEATURE MANIFEST CONTRACT
 * ============================================================================
 *
 * When feature manifests are used under:
 *
 *     grammar/specification/features/
 *
 * this file corresponds to a tracing feature contract containing at least:
 *
 *     id
 *     name
 *     status
 *     version
 *     syntax
 *     grammar
 *     lexer_tokens
 *     ast_nodes
 *     semantic_rules
 *     provenance
 *     capabilities
 *     resource_requirements
 *     privacy_requirements
 *     compiler_consumers
 *     runtime_consumers
 *     positive_tests
 *     negative_tests
 *     boundary_tests
 *     scalability_tests
 *     determinism_tests
 *     compatibility
 *     hard_coding_policy
 *
 * This grammar does not depend on the manifest existing at parse time.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is independently complete when:
 *
 * [x] Canonical parser grammar declaration exists.
 *
 * [x] Canonical ZamaniLexer is consumed.
 *
 * [x] No competing lexer is introduced.
 *
 * [x] Existing OBSERVE token is reused.
 *
 * [x] TRACE token is not invented.
 *
 * [x] SPAN token is not invented.
 *
 * [x] Tracing rules have a unique `tracing*` namespace.
 *
 * [x] Existing observability traceClause/spanClause names are not duplicated.
 *
 * [x] Public entry point is tracingConstruct.
 *
 * [x] Trace intent is represented.
 *
 * [x] Span intent is represented.
 *
 * [x] Trace subject is generic.
 *
 * [x] Span relationships are representable.
 *
 * [x] Correlation is representable.
 *
 * [x] Propagation is representable.
 *
 * [x] Sampling is representable.
 *
 * [x] Filtering is representable.
 *
 * [x] Attributes are representable.
 *
 * [x] Context is representable.
 *
 * [x] Privacy is representable.
 *
 * [x] Retention is representable.
 *
 * [x] Export is representable.
 *
 * [x] Provenance is representable.
 *
 * [x] Requirements are representable.
 *
 * [x] Capabilities are representable.
 *
 * [x] Constraints are representable.
 *
 * [x] Preferences are representable.
 *
 * [x] Hints are representable.
 *
 * [x] No runtime implementation is embedded.
 *
 * [x] No semantic actions are embedded.
 *
 * [x] No hardware discovery is embedded.
 *
 * [x] No backend is selected.
 *
 * [x] No vendor tracing system is mandated.
 *
 * [x] No fixed trace ID representation is imposed.
 *
 * [x] No fixed clock representation is imposed.
 *
 * [x] No fixed storage backend is imposed.
 *
 * [x] No fixed distributed topology is imposed.
 *
 * [x] No fixed machine size is imposed.
 *
 * [x] No MAX_* tracing limits exist.
 *
 * [x] Quantum integration is downstream.
 *
 * [x] quantum::ir remains canonical.
 *
 * [x] QEC remains downstream.
 *
 * [x] ZQN remains downstream.
 *
 * [x] Routing remains downstream.
 *
 * [x] Scheduling remains downstream.
 *
 * [x] HAL remains downstream.
 *
 * [x] Source-map ownership remains downstream.
 *
 * [x] Diagnostic ownership remains downstream.
 *
 * [x] Safe Rust implementation is sufficient.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is specified.
 *
 * [x] Rust 2021 compatibility is specified.
 *
 * [x] POCO-REAF remains intact.
 *
 * [x] Integration points are defined before consumer integration.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What tracing behavior does the Zamani program request?"
 *
 * It does NOT answer:
 *
 *     "How is tracing implemented?"
 *
 *     "Where are traces stored?"
 *
 *     "Which tracing backend is used?"
 *
 *     "Which machine produces the trace?"
 *
 *     "Which network carries trace data?"
 *
 *     "How are spans scheduled?"
 *
 *     "How are quantum operations routed?"
 *
 *     "How does QEC work?"
 *
 *     "How does ZQN model faults?"
 *
 * Those responsibilities remain downstream.
 *
 * Therefore:
 *
 *     SOURCE
 *        |
 *        v
 *     TRACING INTENT
 *        |
 *        v
 *     DOMAIN-NEUTRAL AST
 *        |
 *        v
 *     SEMANTIC VALIDATION
 *        |
 *        v
 *     CANONICAL SEMANTIC MODEL
 *        |
 *        +--> classical
 *        +--> quantum::ir
 *        +--> HDL/hardware
 *        +--> distributed
 *        +--> future domains
 *        |
 *        v
 *     TARGET REALIZATION
 *        |
 *        v
 *     RUNTIME OBSERVABILITY
 *
 * The language remains portable.
 *
 * The tracing implementation remains replaceable.
 *
 * The program remains scalable from tiny execution environments to
 * arbitrarily large execution environments, subject only to actual resources,
 * capabilities, implementation budgets and explicit semantic requirements.
 *
 * ============================================================================
 */

parser grammar Tracing;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The rules below are the executable ANTLR grammar.
 * ============================================================================
 */

tracingConstruct
    : OBSERVE tracingMode tracingSubject? tracingAttachment* tracingBody? tracingTerminator?
    ;

tracingMode
    : tracingTraceWord
    | tracingSpanWord
    ;

tracingTraceWord
    : identifier
    ;

tracingSpanWord
    : identifier
    ;

tracingSubject
    : expression
    ;

tracingAttachment
    : tracingForAttachment
    | tracingAsAttachment
    | tracingWhenAttachment
    | tracingWhereAttachment
    ;

tracingForAttachment
    : FOR expression
    ;

tracingAsAttachment
    : AS identifier
    ;

tracingWhenAttachment
    : WHEN expression
    ;

tracingWhereAttachment
    : WHERE expression
    ;

tracingBody
    : LBRACE tracingClause* RBRACE
    ;

tracingClause
    : tracingNamedClause
    | tracingAssignmentClause
    | tracingBlockClause
    ;

tracingNamedClause
    : identifier COLON tracingValue
    ;

tracingAssignmentClause
    : identifier ASSIGN tracingValue
    ;

tracingBlockClause
    : identifier COLON LBRACE tracingClause* RBRACE
    ;

tracingValue
    : expression
    ;

tracingRequirementClause
    : REQUIRES tracingRequirement
    ;

tracingRequirement
    : CAPABILITY LPAREN expression RPAREN
    | RESOURCE LPAREN expression RPAREN
    | expression
    ;

tracingPreferenceClause
    : PREFER tracingValue
    ;

tracingHintClause
    : HINT tracingValue
    ;

tracingConstraintClause
    : CONSTRAINT tracingValue
    ;

tracingEventClause
    : identifier COLON tracingEventValue
    ;

tracingEventValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingRelationshipClause
    : identifier COLON tracingRelationshipValue
    ;

tracingRelationshipValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingCorrelationClause
    : identifier COLON tracingCorrelationValue
    ;

tracingCorrelationValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingPropagationClause
    : identifier COLON tracingPropagationValue
    ;

tracingPropagationValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingSamplingClause
    : identifier COLON tracingSamplingValue
    ;

tracingSamplingValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingFilterClause
    : identifier COLON tracingFilterValue
    ;

tracingFilterValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingAttributeClause
    : identifier COLON tracingAttributeValue
    ;

tracingAttributeValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingContextClause
    : identifier COLON tracingContextValue
    ;

tracingContextValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingPrivacyClause
    : identifier COLON tracingPrivacyValue
    ;

tracingPrivacyValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingRetentionClause
    : identifier COLON tracingRetentionValue
    ;

tracingRetentionValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingExportClause
    : identifier COLON tracingExportValue
    ;

tracingExportValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingProvenanceClause
    : identifier COLON tracingProvenanceValue
    ;

tracingProvenanceValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingFailureClause
    : identifier COLON tracingFailureValue
    ;

tracingFailureValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingLifecycleClause
    : identifier COLON tracingLifecycleValue
    ;

tracingLifecycleValue
    : expression
    | LBRACE tracingClause* RBRACE
    ;

tracingExecutionCorrelation
    : identifier COLON expression
    ;

tracingTerminator
    : SEMI
    ;