/*
 * ============================================================================
 * Zamani — Execution Observability Grammar
 * ============================================================================
 *
 * File:
 *   grammar/execution/observability.g4
 *
 * Role:
 *   Canonical parser grammar for portable observability intent.
 *
 * Language:
 *   Zamani
 *
 * Toolchain:
 *   ANTLR parser grammar
 *   Rust implementation target: Rust 1.97 / 1.97.1
 *   Unsafe Rust: prohibited by the Zamani implementation contract.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file owns ONLY source-level observability intent:
 *
 *   - observation declarations
 *   - observation scopes
 *   - events
 *   - metrics
 *   - logs
 *   - traces
 *   - spans
 *   - profiles
 *   - sampling intent
 *   - filtering intent
 *   - attributes/labels
 *   - correlation/context
 *   - collection policies
 *   - retention intent
 *   - export intent
 *   - privacy/classification intent
 *   - integrity/provenance intent
 *   - resource/capability requirements
 *   - user-defined observability extensions
 *
 * This file DOES NOT own:
 *
 *   - runtime telemetry implementation
 *   - tracing SDKs
 *   - OpenTelemetry implementation
 *   - Prometheus implementation
 *   - logging backends
 *   - storage formats
 *   - databases
 *   - network transports
 *   - exporter implementations
 *   - profiler implementations
 *   - scheduler implementation
 *   - placement implementation
 *   - checkpoint implementation
 *   - recovery implementation
 *   - resilience policy implementation
 *   - physical device topology
 *   - CPU/GPU/QPU/FPGA identifiers
 *   - quantum::ir
 *   - hardware-specific counters
 *
 * Those concerns belong to downstream semantic/runtime/backend layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Observability must preserve:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar describes WHAT should be observed, not HOW or WHERE
 * the observation is physically collected.
 *
 * Examples of portable intent:
 *
 *   observe execution
 *   observe latency
 *   observe metric("throughput")
 *   observe trace
 *   observe events
 *   sample according_to expression
 *   require capability("telemetry")
 *
 * Physical choices such as:
 *
 *   Prometheus
 *   OTLP
 *   local file
 *   remote collector
 *   vendor profiler
 *   GPU performance counter
 *
 * are implementation/backend decisions and must not be required by this
 * grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No universal limits are encoded here.
 *
 * Forbidden grammar-level assumptions include:
 *
 *   MAX_EVENTS
 *   MAX_METRICS
 *   MAX_SPANS
 *   MAX_ATTRIBUTES
 *   MAX_TRACE_DEPTH
 *   MAX_SAMPLES
 *   MAX_LOG_SIZE
 *   MAX_EXPORTERS
 *   MAX_OBSERVERS
 *   MAX_TARGETS
 *
 * Any quantity appearing in a program is program data or a semantic
 * requirement, not a compiler-wide capacity.
 *
 * ============================================================================
 * ARCHITECTURAL SEPARATION
 * ============================================================================
 *
 * Source
 *   ↓
 * Lexer
 *   ↓
 * Parser
 *   ↓
 * Frontend AST
 *   ↓
 * Semantic validation
 *   ↓
 * Canonical semantic model
 *   ↓
 * Compiler / planner
 *   ↓
 * Runtime observability implementation
 *   ↓
 * Actual telemetry system
 *
 * Observability must never create a second execution IR.
 *
 * Quantum observability follows:
 *
 *   quantum source
 *       ↓
 *   quantum::ir
 *       ↓
 *   observability metadata/intent
 *       ↓
 *   runtime/backend realization
 *
 * This grammar must never create another quantum IR.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *   grammar/lexer/
 *   grammar/core/
 *   grammar/types/
 *   grammar/expressions/
 *   grammar/execution/
 *
 * Related execution domains:
 *
 *   runtime.g4
 *   scheduling.g4
 *   placement.g4
 *   resilience.g4
 *   recovery.g4
 *   checkpointing.g4
 *   lifecycle.g4
 *
 * Related cross-domain contracts:
 *
 *   resources/resource.g4
 *   resources/requirements.g4
 *   resources/capabilities.g4
 *   data/persistence.g4
 *   security/policies.g4
 *   compile/profiles.g4
 *   hardware/capabilities.g4
 *   quantum/*
 *   hdl/*
 *
 * Downstream:
 *
 *   frontend AST
 *   semantic analysis
 *   canonical semantic model
 *   compiler planning
 *   runtime
 *   observability implementation
 *
 * ============================================================================
 * OWNERSHIP RULE
 * ============================================================================
 *
 * runtime.g4 MUST NOT redefine the complete observability language.
 *
 * If runtime syntax needs observability, it should delegate to this grammar's
 * observability construct rather than introduce a competing syntax.
 *
 * resilience.g4 owns resilience policy.
 * recovery.g4 owns recovery intent.
 * checkpointing.g4 owns checkpoint/restore intent.
 * scheduling.g4 owns scheduling.
 * placement.g4 owns placement.
 * persistence.g4 owns persistence.
 *
 * Observability merely describes what execution information is requested,
 * exposed, correlated, sampled, retained, or exported.
 *
 * ============================================================================
 */

parser grammar Observability;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Execution;


/*
 * ============================================================================
 * 1. TOP-LEVEL OBSERVABILITY CONSTRUCT
 * ============================================================================
 *
 * The lexical spelling of an observability keyword remains owned by the
 * canonical lexer. The parser uses the shared identifier/name machinery for
 * extensible observability vocabulary so new metric/event/trace names do not
 * require a new parser rule.
 *
 * Semantic validation must distinguish reserved observability constructs from
 * arbitrary user-defined names.
 * ============================================================================
 */

observabilityConstruct
    : observabilityDeclaration
    | observationStatement
    ;


/*
 * ============================================================================
 * 2. DECLARATION
 * ============================================================================
 *
 * An observability declaration defines reusable observation intent.
 *
 * The declaration does not create a runtime collector.
 * ============================================================================
 */

observabilityDeclaration
    : identifier observabilityDeclarationBody
    ;

observabilityDeclarationBody
    : block
    | observabilityClause*
    ;


/*
 * ============================================================================
 * 3. OBSERVATION STATEMENT
 * ============================================================================
 *
 * Generic structure intentionally permits future observability domains without
 * changing the core parser every time a new observation category is added.
 * ============================================================================
 */

observationStatement
    : identifier observationTarget? observationClause*
    ;


/*
 * ============================================================================
 * 4. TARGET / SCOPE
 * ============================================================================
 *
 * Scope identifies WHAT execution entity is being observed.
 *
 * It does not identify a physical machine.
 * ============================================================================
 */

observationTarget
    : observationScope
    | expression
    ;

observationScope
    : identifier
    | qualifiedName
    | observationScopeBlock
    ;

observationScopeBlock
    : LBRACE observationScopeItem* RBRACE
    ;

observationScopeItem
    : observationScopeSelector
    | observationClause
    ;

observationScopeSelector
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 5. OBSERVABILITY CLAUSES
 * ============================================================================
 */

observabilityClause
    : observationKindClause
    | eventClause
    | metricClause
    | logClause
    | traceClause
    | spanClause
    | profileClause
    | samplingClause
    | filterClause
    | attributeClause
    | contextClause
    | correlationClause
    | collectionClause
    | retentionClause
    | exportClause
    | privacyClause
    | classificationClause
    | integrityClause
    | provenanceClause
    | resourceRequirementClause
    | capabilityRequirementClause
    | preferenceClause
    | constraintClause
    | hintClause
    | conditionClause
    | triggerClause
    | customObservabilityClause
    ;


/*
 * ============================================================================
 * 6. OBSERVATION KIND
 * ============================================================================
 *
 * The grammar deliberately avoids an exhaustive closed list.
 *
 * The semantic layer recognizes standard kinds while allowing extensions.
 * ============================================================================
 */

observationKindClause
    : identifier
    | identifier LPAREN expressionList? RPAREN
    ;


/*
 * ============================================================================
 * 7. EVENTS
 * ============================================================================
 */

eventClause
    : identifier eventSpecification?
    ;

eventSpecification
    : LPAREN eventItem* RPAREN
    ;

eventItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 8. METRICS
 * ============================================================================
 *
 * Metrics are semantic objects, not fixed compiler keywords.
 * ============================================================================
 */

metricClause
    : identifier metricSpecification?
    ;

metricSpecification
    : LPAREN metricItem* RPAREN
    ;

metricItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 9. LOGGING
 * ============================================================================
 */

logClause
    : identifier logSpecification?
    ;

logSpecification
    : LPAREN logItem* RPAREN
    ;

logItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 10. DISTRIBUTED TRACING
 * ============================================================================
 */

traceClause
    : identifier traceSpecification?
    ;

traceSpecification
    : LPAREN traceItem* RPAREN
    ;

traceItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 11. SPANS
 * ============================================================================
 */

spanClause
    : identifier spanSpecification?
    ;

spanSpecification
    : LPAREN spanItem* RPAREN
    ;

spanItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 12. PROFILING
 * ============================================================================
 */

profileClause
    : identifier profileSpecification?
    ;

profileSpecification
    : LPAREN profileItem* RPAREN
    ;

profileItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 13. SAMPLING
 * ============================================================================
 *
 * No fixed sampling rate or sample count is imposed.
 * ============================================================================
 */

samplingClause
    : identifier samplingSpecification?
    ;

samplingSpecification
    : LPAREN samplingItem* RPAREN
    ;

samplingItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 14. FILTERING
 * ============================================================================
 */

filterClause
    : identifier filterSpecification?
    ;

filterSpecification
    : LPAREN expression RPAREN
    | LBRACE filterItem* RBRACE
    ;

filterItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 15. ATTRIBUTES / LABELS
 * ============================================================================
 *
 * Attribute cardinality is unbounded by the language.
 * ============================================================================
 */

attributeClause
    : identifier attributeSpecification?
    ;

attributeSpecification
    : LPAREN attributeItem* RPAREN
    | LBRACE attributeItem* RBRACE
    ;

attributeItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 16. CONTEXT
 * ============================================================================
 */

contextClause
    : identifier contextSpecification?
    ;

contextSpecification
    : LPAREN contextItem* RPAREN
    | LBRACE contextItem* RBRACE
    ;

contextItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 17. CORRELATION
 * ============================================================================
 *
 * Correlation may span local, distributed, classical, quantum, or hardware
 * execution domains.
 * ============================================================================
 */

correlationClause
    : identifier correlationSpecification?
    ;

correlationSpecification
    : LPAREN correlationItem* RPAREN
    | LBRACE correlationItem* RBRACE
    ;

correlationItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 18. COLLECTION
 * ============================================================================
 */

collectionClause
    : identifier collectionSpecification?
    ;

collectionSpecification
    : LPAREN collectionItem* RPAREN
    | LBRACE collectionItem* RBRACE
    ;

collectionItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 19. RETENTION
 * ============================================================================
 *
 * Retention is intent.
 *
 * Actual storage ownership belongs to persistence/storage infrastructure.
 * ============================================================================
 */

retentionClause
    : identifier retentionSpecification?
    ;

retentionSpecification
    : LPAREN retentionItem* RPAREN
    | LBRACE retentionItem* RBRACE
    ;

retentionItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 20. EXPORT
 * ============================================================================
 *
 * Export destination is semantic intent.
 *
 * The grammar must not hard-code:
 *
 *   Prometheus
 *   OTLP
 *   Kafka
 *   file systems
 *   vendor APIs
 *
 * as mandatory implementation choices.
 * ============================================================================
 */

exportClause
    : identifier exportSpecification?
    ;

exportSpecification
    : LPAREN exportItem* RPAREN
    | LBRACE exportItem* RBRACE
    ;

exportItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 21. PRIVACY
 * ============================================================================
 */

privacyClause
    : identifier privacySpecification?
    ;

privacySpecification
    : LPAREN privacyItem* RPAREN
    | LBRACE privacyItem* RBRACE
    ;

privacyItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 22. DATA CLASSIFICATION
 * ============================================================================
 */

classificationClause
    : identifier classificationSpecification?
    ;

classificationSpecification
    : LPAREN classificationItem* RPAREN
    | LBRACE classificationItem* RBRACE
    ;

classificationItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 23. INTEGRITY
 * ============================================================================
 */

integrityClause
    : identifier integritySpecification?
    ;

integritySpecification
    : LPAREN integrityItem* RPAREN
    | LBRACE integrityItem* RBRACE
    ;

integrityItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 24. PROVENANCE
 * ============================================================================
 */

provenanceClause
    : identifier provenanceSpecification?
    ;

provenanceSpecification
    : LPAREN provenanceItem* RPAREN
    | LBRACE provenanceItem* RBRACE
    ;

provenanceItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 25. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * These are requirements, not hard-coded capacities.
 *
 * Examples semantically represented downstream:
 *
 *   requires memory >= required_memory
 *   requires capability("telemetry")
 *   requires capability("distributed.tracing")
 *
 * No MAX_* limits are encoded.
 * ============================================================================
 */

resourceRequirementClause
    : identifier resourceRequirementSpecification?
    ;

resourceRequirementSpecification
    : LPAREN resourceRequirementItem* RPAREN
    | LBRACE resourceRequirementItem* RBRACE
    ;

resourceRequirementItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    | comparisonOperator expression
    ;


/*
 * ============================================================================
 * 26. CAPABILITY REQUIREMENTS
 * ============================================================================
 */

capabilityRequirementClause
    : identifier capabilityRequirementSpecification?
    ;

capabilityRequirementSpecification
    : LPAREN expressionList? RPAREN
    | LBRACE capabilityRequirementItem* RBRACE
    ;

capabilityRequirementItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 27. PREFERENCES
 * ============================================================================
 *
 * Preferences must never become mandatory implementation requirements unless
 * semantic analysis explicitly promotes them according to language policy.
 * ============================================================================
 */

preferenceClause
    : identifier preferenceSpecification?
    ;

preferenceSpecification
    : LPAREN preferenceItem* RPAREN
    | LBRACE preferenceItem* RBRACE
    ;

preferenceItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 28. CONSTRAINTS
 * ============================================================================
 */

constraintClause
    : identifier constraintSpecification?
    ;

constraintSpecification
    : LPAREN expression RPAREN
    | LBRACE constraintItem* RBRACE
    ;

constraintItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 29. HINTS
 * ============================================================================
 *
 * Hints are non-binding unless a downstream semantic contract says otherwise.
 * ============================================================================
 */

hintClause
    : identifier hintSpecification?
    ;

hintSpecification
    : LPAREN hintItem* RPAREN
    | LBRACE hintItem* RBRACE
    ;

hintItem
    : identifier
    | qualifiedName
    | ASSIGN expression
    | COLON expression
    ;


/*
 * ============================================================================
 * 30. CONDITIONS
 * ============================================================================
 */

conditionClause
    : identifier conditionSpecification
    ;

conditionSpecification
    : expression
    | LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 31. TRIGGERS
 * ============================================================================
 *
 * Trigger expressions remain semantic expressions rather than a closed
 * enumeration of runtime events.
 * ============================================================================
 */

triggerClause
    : identifier triggerSpecification
    ;

triggerSpecification
    : expression
    | LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 32. EXTENSIBILITY
 * ============================================================================
 *
 * This is the deliberate escape hatch for future observability domains.
 *
 * It must NOT be an escape hatch around semantic validation.
 *
 * Unknown extensions are parsed as syntax and subsequently admitted/rejected
 * by semantic validation according to the dialect/feature registry.
 * ============================================================================
 */

customObservabilityClause
    : identifier
    | qualifiedName
    | identifier LPAREN expressionList? RPAREN
    | qualifiedName LPAREN expressionList? RPAREN
    ;


/*
 * ============================================================================
 * 33. COMPARISON SUPPORT
 * ============================================================================
 *
 * Resource and policy expressions delegate semantic meaning to the common
 * expression/type system.
 * ============================================================================
 */

comparisonOperator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    ;


/*
 * ============================================================================
 * 34. OBSERVABILITY POLICY CONTRACT
 * ============================================================================
 *
 * The semantic layer must classify every clause into one or more of:
 *
 *   observation intent
 *   requirement
 *   capability requirement
 *   constraint
 *   preference
 *   hint
 *   trigger
 *   condition
 *   metadata
 *   privacy policy
 *   retention policy
 *   export intent
 *
 * Parser success MUST NOT imply that the requested runtime capability exists.
 *
 * ============================================================================
 */