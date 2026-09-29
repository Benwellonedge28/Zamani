/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/timelines.g4
 *
 * Grammar:
 *     ExecutionTimelines
 *
 * Status:
 *     Production-ready source-level Multi-Timeline System (MTS) execution
 *     intent grammar.
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
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL EXECUTION INTENT for Multi-Timeline System
 * (MTS) computations.
 *
 * It provides a target-independent syntax for expressing:
 *
 *     - timeline declarations;
 *     - timeline identity;
 *     - timeline relationships;
 *     - parent/child relationships;
 *     - divergence intent;
 *     - speculative execution;
 *     - branch creation;
 *     - timeline observation;
 *     - timeline synchronization;
 *     - timeline joining;
 *     - timeline merging;
 *     - timeline selection;
 *     - temporal checkpoints;
 *     - temporal restoration/rewind intent;
 *     - causal dependencies;
 *     - consistency requirements;
 *     - conflict policies;
 *     - merge policies;
 *     - observation policies;
 *     - speculative policies;
 *     - timeline lifecycle intent;
 *     - timeline resource/capability requirements;
 *     - extensible timeline properties.
 *
 * This grammar DOES NOT implement the MTS runtime.
 *
 * It does NOT:
 *
 *     - allocate timelines;
 *     - store timeline state;
 *     - evaluate timestamps;
 *     - execute speculative branches;
 *     - resolve causal conflicts;
 *     - detect paradoxes;
 *     - merge states;
 *     - restore checkpoints;
 *     - schedule timelines;
 *     - allocate CPUs;
 *     - allocate GPUs;
 *     - allocate QPUs;
 *     - allocate FPGA resources;
 *     - allocate nodes;
 *     - allocate memory;
 *     - perform quantum routing;
 *     - perform QEC;
 *     - perform ZQN;
 *     - discover hardware;
 *     - perform deployment;
 *     - implement runtime policy.
 *
 * Those responsibilities remain downstream.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The complete pipeline is:
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
 *     semantic analysis
 *          |
 *          +--> temporal/causal analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> effect analysis
 *          +--> ownership analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical              quantum::ir          HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                            optimization
 *                                 |
 *                   +-------------+-------------+
 *                   |             |             |
 *                   v             v             v
 *                routing      scheduling     resilience
 *                   |             |             |
 *                   +-------------+-------------+
 *                                 |
 *                           QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                                 v
 *                         target realization
 *                                 |
 *                                 v
 *                              runtime
 *                                 |
 *                                 v
 *                         MTS realization
 *
 * MTS is therefore a semantic/runtime capability.
 *
 * This grammar describes the SOURCE INTENT that allows the downstream MTS
 * runtime/compiler to realize that intent.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     timeline declarations;
 *     timeline references;
 *     timeline relationships;
 *     timeline operations;
 *     branch intent;
 *     speculative execution intent;
 *     observation intent;
 *     synchronization intent at the MTS level;
 *     merge intent;
 *     rewind/restore intent;
 *     causal relationship intent;
 *     consistency intent;
 *     conflict-resolution intent;
 *     timeline lifecycle intent;
 *     timeline-scoped requirements;
 *     timeline-scoped constraints;
 *     timeline-scoped preferences;
 *     timeline-scoped hints;
 *     extensible timeline properties.
 *
 * ============================================================================
 * 4. NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical identifiers;
 *     lexical literals;
 *     generic expressions;
 *     generic types;
 *     generic names;
 *     generic resources;
 *     generic capabilities;
 *     general concurrency;
 *     general synchronization;
 *     execution scheduling;
 *     physical placement;
 *     hardware discovery;
 *     resource allocation;
 *     quantum operations;
 *     quantum state representation;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime implementation;
 *     persistence implementation;
 *     storage implementation;
 *     distributed consensus implementation;
 *     paradox-resolution algorithms.
 *
 * Generic execution synchronization remains owned by:
 *
 *     grammar/execution/synchronization.g4
 *
 * Generic scheduling remains owned by:
 *
 *     grammar/execution/scheduling.g4
 *
 * Generic resources remain owned by:
 *
 *     grammar/resources/
 *
 * Generic capabilities remain owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/capabilities.g4
 *
 * Temporal type syntax remains owned by:
 *
 *     grammar/types/temporal.g4
 *
 * General expression syntax remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * MTS memory/storage semantics remain downstream from the grammar.
 *
 * ============================================================================
 * 5. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT create:
 *
 *     - a second lexer;
 *     - a second identifier grammar;
 *     - a second expression grammar;
 *     - a second type grammar;
 *     - a second resource grammar;
 *     - a second capability grammar;
 *     - a second synchronization grammar;
 *     - an MTS IR;
 *     - a quantum IR;
 *     - a runtime implementation.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * Timeline semantics are attached to the canonical semantic model and
 * execution representation downstream.
 *
 * ============================================================================
 * 6. LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * The existing canonical keyword vocabulary already contains:
 *
 *     MTS
 *     OBSERVE
 *     FROM
 *     WHEN
 *     WITH
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     TARGET
 *     RESOURCE
 *     RESOURCES
 *     ALIGNMENT
 *     CONSISTENCY-related generic vocabulary where available
 *
 * This file MUST NOT add lexer rules.
 *
 * In particular, it deliberately does NOT introduce dedicated lexer tokens
 * for:
 *
 *     TIMELINE
 *     BRANCH
 *     FORK
 *     MERGE
 *     REWIND
 *     RESTORE
 *     SPECULATE
 *     SYNCHRONIZE
 *
 * unless those words are independently promoted into the canonical lexer by
 * the repository's lexical authority.
 *
 * Timeline operation/property names are therefore represented using canonical
 * identifier/name structures wherever possible.
 *
 * This keeps the MTS language open-ended and prevents a future timeline
 * operation from requiring an unnecessary lexer modification.
 *
 * ============================================================================
 * 7. CANONICAL TOKEN VOCABULARY
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer vocabulary.
 *
 * It uses only parser-visible tokens already established by the repository
 * wherever possible:
 *
 *     MTS
 *     OBSERVE
 *     FROM
 *     WHEN
 *     WITH
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     CAPABILITY
 *     TARGET
 *     RESOURCE
 *     RESOURCES
 *
 * and canonical punctuation/operators:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     DOUBLE_COLON
 *     THIN_ARROW
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     GREATER
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * The grammar intentionally uses canonical `identifier` and `qualifiedName`
 * rather than defining a second name system.
 *
 * ============================================================================
 * 8. POCO-REAF
 * ============================================================================
 *
 * Multi-Timeline syntax participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore:
 *
 *     timeline count
 *     branch count
 *     merge count
 *     checkpoint count
 *     causal-edge count
 *     state count
 *     event count
 *     observation count
 *     synchronization count
 *
 * are NOT language-level finite capacities.
 *
 * The grammar uses:
 *
 *     *
 *     +
 *
 * and recursively composable structures.
 *
 * A target with one execution resource may serialize independent timelines.
 *
 * A target with many execution resources may execute them concurrently.
 *
 * A distributed target may distribute them.
 *
 * A quantum/classical target may map them around quantum and classical
 * regions.
 *
 * The source semantics remain independent of that realization.
 *
 * ============================================================================
 * 9. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     MAX_TIMELINES
 *     MAX_BRANCHES
 *     MAX_TIMELINE_DEPTH
 *     MAX_TIMELINE_STATES
 *     MAX_TIMELINE_EVENTS
 *     MAX_CHECKPOINTS
 *     MAX_OBSERVATIONS
 *     MAX_MERGES
 *     MAX_FORKS
 *     MAX_CAUSAL_EDGES
 *     MAX_TIMELINE_RESOURCES
 *     MAX_TIMELINE_NODES
 *     MAX_TIMELINE_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * It also MUST NOT encode:
 *
 *     timeline 0
 *     timeline 1
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 0
 *
 * as universal implementation semantics.
 *
 * A numeric value appearing in source is a PROGRAM VALUE.
 *
 * Example:
 *
 *     checkpoint: every 100;
 *
 * is a program-level value.
 *
 * It does not establish a language-wide checkpoint limit.
 *
 * ============================================================================
 * 10. TEMPORAL SEMANTICS BOUNDARY
 * ============================================================================
 *
 * This grammar preserves temporal expressions structurally.
 *
 * It does NOT decide:
 *
 *     - whether a timestamp is valid;
 *     - whether time is physical or logical;
 *     - whether time is discrete or continuous;
 *     - whether a future state is materializable;
 *     - whether a rewind is causally legal;
 *     - whether two timelines are compatible;
 *     - whether a branch conflicts with another branch.
 *
 * Those decisions belong to semantic analysis and the MTS runtime model.
 *
 * No fixed timestamp width is imposed by this grammar.
 *
 * No fixed clock representation is imposed.
 *
 * ============================================================================
 * 11. CAUSALITY BOUNDARY
 * ============================================================================
 *
 * Timeline relationships are represented syntactically.
 *
 * Causality is validated semantically.
 *
 * Examples of semantic questions:
 *
 *     - Does B actually depend on A?
 *     - Is a rewind permitted?
 *     - Does a merge violate causal ordering?
 *     - Does a branch inherit state or merely reference it?
 *     - Is an observation allowed to affect a speculative branch?
 *     - Is a timeline immutable?
 *     - Is a timeline speculative?
 *     - Can two histories be reconciled?
 *
 * This grammar MUST NOT answer those questions.
 *
 * ============================================================================
 * 12. PARADOX / CONFLICT BOUNDARY
 * ============================================================================
 *
 * A timeline conflict is not a syntax error merely because the runtime cannot
 * resolve it.
 *
 * Syntax validity:
 *
 *     parser
 *
 * Semantic validity:
 *
 *     semantic analysis
 *
 * Causal validity:
 *
 *     temporal/causal analysis
 *
 * Conflict resolution:
 *
 *     MTS runtime/planner
 *
 * The grammar preserves enough structure for these downstream systems.
 *
 * ============================================================================
 * 13. QUANTUM INTEGRATION
 * ============================================================================
 *
 * MTS may surround quantum computation.
 *
 * Example conceptual flow:
 *
 *     timeline branch
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical decision
 *          |
 *          v
 *     merge/continue
 *
 * This grammar MUST NOT represent quantum gates.
 *
 * Quantum operations remain owned by:
 *
 *     grammar/quantum/
 *
 * Quantum semantics lower through:
 *
 *     quantum::ir
 *
 * Timeline metadata is associated with the semantic execution context.
 *
 * There is no:
 *
 *     TimelineQuantumIR
 *     MTSQuantumIR
 *     SpeculativeQuantumIR
 *
 * ============================================================================
 * 14. CLASSICAL / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * Timeline subjects are generic expressions/references.
 *
 * Therefore a timeline may contain or refer to:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware computation;
 *     accelerator computation;
 *     distributed computation;
 *     AI computation;
 *     dataflow;
 *     networking;
 *     future domains.
 *
 * This grammar does not enumerate those domains.
 *
 * ============================================================================
 * 15. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Timelines may eventually be realized across distributed execution.
 *
 * This grammar does not prescribe:
 *
 *     one timeline per node;
 *     one node per branch;
 *     one process per timeline;
 *     one device per timeline.
 *
 * Distributed realization belongs downstream.
 *
 * ============================================================================
 * 16. SYNCHRONIZATION INTEGRATION
 * ============================================================================
 *
 * MTS synchronization expresses temporal/causal relationships.
 *
 * It does not replace:
 *
 *     grammar/execution/synchronization.g4
 *
 * or:
 *
 *     grammar/concurrency/synchronization.g4
 *
 * MTS-specific synchronization can be represented as a timeline operation
 * or property and lowered into the canonical synchronization model.
 *
 * ============================================================================
 * 17. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Timeline order is NOT automatically a physical schedule.
 *
 * For example:
 *
 *     branch A
 *     branch B
 *
 * establishes a semantic relationship.
 *
 * The scheduler determines whether A and B execute:
 *
 *     concurrently;
 *     sequentially;
 *     partially overlapped;
 *     distributed;
 *     deferred;
 *     simulated.
 *
 * This grammar does not select the scheduling realization.
 *
 * ============================================================================
 * 18. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Timeline declarations may carry resource/capability intent.
 *
 * They may express:
 *
 *     requires ...
 *     constraint ...
 *     prefer ...
 *     hint ...
 *
 * The actual resource grammar remains authoritative for resource expressions.
 *
 * The MTS grammar must not duplicate resource arithmetic or capability
 * semantics.
 *
 * ============================================================================
 * 19. AST CONTRACT
 * ============================================================================
 *
 * Every accepted MTS construct must map to a domain-neutral frontend AST.
 *
 * Conceptual shape:
 *
 *     TimelineDeclaration {
 *         name: Name,
 *         parent: Option<NameReference>,
 *         clauses: Vec<TimelineClause>,
 *         source_span: SourceSpan
 *     }
 *
 *     TimelineOperation {
 *         operation: QualifiedName,
 *         target: Option<NameReference>,
 *         arguments: Vec<Expression>,
 *         clauses: Vec<TimelineClause>,
 *         source_span: SourceSpan
 *     }
 *
 *     TimelineClause {
 *         kind: TimelineClauseKind,
 *         key: Option<QualifiedName>,
 *         value: Expression,
 *         source_span: SourceSpan
 *     }
 *
 *     TimelineReference {
 *         name: QualifiedName,
 *         source_span: SourceSpan
 *     }
 *
 *     TimelineRelation {
 *         relation: QualifiedName,
 *         source: TimelineReference,
 *         target: TimelineReference,
 *         source_span: SourceSpan
 *     }
 *
 * These are conceptual contracts only.
 *
 * Actual AST structures belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust structs.
 *
 * ============================================================================
 * 20. AST DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * The AST MUST NOT require:
 *
 *     PhysicalTimelineId
 *     CpuId
 *     GpuId
 *     QpuId
 *     FpgaId
 *     PhysicalQubitId
 *     NodeId
 *     MemoryAddress
 *
 * merely because a timeline construct exists.
 *
 * Symbolic source names remain symbolic until semantic resolution.
 *
 * ============================================================================
 * 21. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether timeline names are valid;
 *     - whether references resolve;
 *     - whether a parent exists;
 *     - whether a branch relationship is legal;
 *     - whether an operation is supported;
 *     - whether temporal expressions have valid types;
 *     - whether causal relationships are valid;
 *     - whether observations are permitted;
 *     - whether speculative execution is permitted;
 *     - whether rewinds are legal;
 *     - whether merges are compatible;
 *     - whether conflict policy is sufficient;
 *     - whether synchronization requirements are satisfiable;
 *     - whether resource requirements are satisfiable;
 *     - whether capabilities are available;
 *     - whether effects/ownership prohibit a timeline operation;
 *     - whether quantum semantics remain compatible with quantum::ir;
 *     - whether distributed realization is possible.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * 22. IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO MTS-specific IR.
 *
 * The lowering path is:
 *
 *     source
 *       |
 *       v
 *     Timeline AST
 *       |
 *       v
 *     semantic timeline intent
 *       |
 *       v
 *     canonical semantic execution model
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical             quantum::ir          HDL/hardware
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                     routing / scheduling
 *                              |
 *                         resilience
 *                              |
 *                         QEC / ZQN
 *                              |
 *                             HAL
 *
 * MTS state-management implementation is downstream of this semantic
 * representation.
 *
 * ============================================================================
 * 23. OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Timeline operations are deliberately represented using an extensible
 * operation name.
 *
 * This avoids making the language a closed list of timeline operations.
 *
 * Conceptually supported operation names include:
 *
 *     timeline
 *     branch
 *     fork
 *     speculate
 *     observe
 *     synchronize
 *     join
 *     merge
 *     rewind
 *     restore
 *     checkpoint
 *     select
 *     continue
 *
 * These names are semantic operation names, not a finite hardware model.
 *
 * A future operation can be introduced through the semantic feature registry
 * without redesigning the entire grammar.
 *
 * ============================================================================
 * 24. OPERATION ARGUMENTS
 * ============================================================================
 *
 * Timeline operations may accept:
 *
 *     positional arguments;
 *     named properties;
 *     nested property blocks;
 *     resource/capability intent;
 *     temporal expressions;
 *     causal references.
 *
 * The number of arguments is not bounded by the grammar.
 *
 * ============================================================================
 * 25. PROPERTY MODEL
 * ============================================================================
 *
 * Timeline properties use qualified names.
 *
 * Example:
 *
 *     causal::policy: strict;
 *
 *     merge::policy: reconcile;
 *
 *     observation::mode: isolated;
 *
 *     speculative::strategy: explore;
 *
 *     consistency::model: causal;
 *
 *     checkpoint::policy: automatic;
 *
 * Unknown properties remain syntactically representable.
 *
 * Semantic analysis decides whether a property is:
 *
 *     stable;
 *     experimental;
 *     dialect-defined;
 *     unsupported;
 *     deprecated;
 *     invalid.
 *
 * ============================================================================
 * 26. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * The grammar keeps these concepts distinct.
 *
 * Requirement:
 *
 *     mandatory realization property.
 *
 * Constraint:
 *
 *     mandatory restriction.
 *
 * Preference:
 *
 *     advisory optimization preference.
 *
 * Hint:
 *
 *     weaker implementation guidance.
 *
 * Example:
 *
 *     requires capability("temporal.speculation");
 *     constraint causal::consistency >= strict;
 *     prefer merge::strategy;
 *     hint placement::locality;
 *
 * The exact resource/capability expression semantics remain downstream.
 *
 * ============================================================================
 * 27. SOURCE ORDER
 * ============================================================================
 *
 * Source order is preserved by the parse tree.
 *
 * Semantic analysis decides whether repeated properties are:
 *
 *     singleton;
 *     repeatable;
 *     mergeable;
 *     mutually exclusive;
 *     order-sensitive.
 *
 * The grammar does not accidentally impose semantic ordering merely because
 * alternatives appear in a particular order.
 *
 * ============================================================================
 * 28. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no randomness;
 *     - no runtime calls;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no mutable global parser state.
 *
 * Given the same token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * 29. SECURITY
 * ============================================================================
 *
 * MTS syntax cannot:
 *
 *     - execute arbitrary code;
 *     - invoke a runtime;
 *     - contact another timeline;
 *     - access another process;
 *     - modify hardware;
 *     - bypass authorization;
 *     - bypass capability checks;
 *     - bypass resource validation;
 *     - bypass security policy.
 *
 * All such behavior belongs downstream and remains subject to semantic and
 * security validation.
 *
 * ============================================================================
 * 30. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation MUST use safe Rust only.
 *
 * No `unsafe` implementation is required or permitted by this grammar's
 * integration contract.
 *
 * ============================================================================
 * 31. ERROR CATEGORIES
 * ============================================================================
 *
 * Syntax diagnostics:
 *
 *     parser.
 *
 * Name-resolution diagnostics:
 *
 *     semantic analysis.
 *
 * Temporal-type diagnostics:
 *
 *     type/semantic analysis.
 *
 * Causal diagnostics:
 *
 *     temporal/causal analysis.
 *
 * Resource diagnostics:
 *
 *     resource analysis.
 *
 * Capability diagnostics:
 *
 *     capability analysis.
 *
 * Target diagnostics:
 *
 *     target resolution.
 *
 * Runtime timeline failures:
 *
 *     runtime/MTS subsystem.
 *
 * A runtime inability to realize a timeline MUST NOT be reported as a
 * syntactic grammar error.
 *
 * ============================================================================
 * 32. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend MUST preserve source spans for:
 *
 *     MTS
 *     operation name
 *     timeline name
 *     references
 *     argument lists
 *     property names
 *     property values
 *     clauses
 *     nested bodies.
 *
 * This is required for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     formatting;
 *     provenance;
 *     debugging;
 *     semantic error reporting.
 *
 * ============================================================================
 * 33. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses recursive/repeated structures rather than fixed capacities.
 *
 * It therefore places no language-level limit on:
 *
 *     timeline declarations;
 *     timeline operations;
 *     branches;
 *     nested branches;
 *     merges;
 *     causal relationships;
 *     observations;
 *     checkpoints;
 *     clauses;
 *     properties;
 *     arguments;
 *     references;
 *     nested property blocks.
 *
 * Practical limits may arise from:
 *
 *     parser memory;
 *     parser stack;
 *     compiler memory;
 *     compiler time;
 *     runtime resources;
 *     storage;
 *     target resources;
 *     deployment policies.
 *
 * Those are implementation/resource limits, not language semantics.
 *
 * ============================================================================
 * 34. BOUNDARY CASES
 * ============================================================================
 *
 * The conformance suite MUST include:
 *
 *     - one timeline;
 *     - many timelines;
 *     - deeply nested timeline relationships;
 *     - long qualified names;
 *     - symbolic timestamps;
 *     - large integer timestamps;
 *     - expression-valued timestamps;
 *     - empty property bodies where permitted;
 *     - large property lists;
 *     - large causal-reference lists;
 *     - nested branches;
 *     - branch-from-branch relationships;
 *     - merge relationships;
 *     - observation relationships;
 *     - checkpoint relationships;
 *     - rewind relationships.
 *
 * The tests must not turn the test fixture size into a language maximum.
 *
 * ============================================================================
 * 35. NEGATIVE CASES
 * ============================================================================
 *
 * Negative tests MUST include:
 *
 *     - missing MTS introducer;
 *     - missing operation name;
 *     - missing timeline name where required;
 *     - missing braces;
 *     - missing colon;
 *     - missing semicolon;
 *     - malformed argument list;
 *     - malformed qualified name;
 *     - malformed relation;
 *     - malformed property;
 *     - malformed reference;
 *     - malformed range;
 *     - malformed temporal expression.
 *
 * Semantic negative tests must separately cover:
 *
 *     - unresolved timeline;
 *     - cyclic causal relationship where prohibited;
 *     - invalid merge;
 *     - illegal rewind;
 *     - incompatible consistency policy;
 *     - unsupported operation;
 *     - unavailable capability;
 *     - unsatisfied resource requirement.
 *
 * These are NOT parser errors unless their syntax itself is malformed.
 *
 * ============================================================================
 * 36. POSITIVE EXAMPLES
 * ============================================================================
 *
 * The following conceptual forms are supported by the grammar:
 *
 *     mts timeline main {
 *         property::mode: normal;
 *     }
 *
 *     mts branch future from main {
 *         property::mode: speculative;
 *     }
 *
 *     mts fork alternative from main {
 *         speculative::strategy: explore;
 *     }
 *
 *     mts merge future with main {
 *         merge::policy: reconcile;
 *     }
 *
 *     mts rewind future to checkpoint;
 *
 *     mts observe future {
 *         observation::mode: isolated;
 *     }
 *
 *     mts synchronize future with main;
 *
 *     mts timeline simulation {
 *         requires capability("temporal.speculation");
 *         constraint causal::consistency >= strict;
 *         prefer merge::strategy;
 *         hint placement::locality;
 *     }
 *
 * The semantic registry determines the meaning of operation/property names.
 *
 * ============================================================================
 * 37. NO PHYSICAL MACHINE ASSUMPTIONS
 * ============================================================================
 *
 * The following are deliberately absent:
 *
 *     cpu IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     physical qubit IDs;
 *     node IDs;
 *     fixed memory banks;
 *     fixed registers;
 *     fixed topology;
 *     fixed timeline capacity.
 *
 * MTS execution can therefore scale across:
 *
 *     embedded systems;
 *     single-core systems;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     simulators;
 *     HPC;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future execution substrates.
 *
 * ============================================================================
 * 38. INTEGRATION WITH execution.g4
 * ============================================================================
 *
 * `grammar/execution/execution.g4` is the execution-domain composition root.
 *
 * It should expose this grammar through a single wrapper:
 *
 *     executionTimelines
 *         : timelineConstruct
 *         ;
 *
 * or an equivalent composition rule.
 *
 * `execution.g4` MUST NOT copy the rules in this file.
 *
 * This file remains the sole owner of MTS timeline syntax.
 *
 * ============================================================================
 * 39. INTEGRATION WITH execution/scheduling.g4
 * ============================================================================
 *
 * Timeline order is semantic temporal intent.
 *
 * Scheduling determines physical execution order.
 *
 * `scheduling.g4` may consume the semantic timeline model but must not
 * duplicate this grammar.
 *
 * ============================================================================
 * 40. INTEGRATION WITH execution/synchronization.g4
 * ============================================================================
 *
 * Timeline relationships such as:
 *
 *     synchronize
 *     wait
 *     join
 *
 * may lower into the canonical execution synchronization model.
 *
 * The timeline grammar remains the source owner of MTS-specific temporal
 * relationships.
 *
 * ============================================================================
 * 41. INTEGRATION WITH execution/recovery.g4
 * ============================================================================
 *
 * Rewind/restore intent may ultimately require recovery.
 *
 * This grammar does not implement recovery.
 *
 * Semantic lowering may translate:
 *
 *     timeline rewind
 *
 * into:
 *
 *     checkpoint selection
 *     recovery intent
 *     state restoration
 *
 * where the language semantics permit it.
 *
 * ============================================================================
 * 42. INTEGRATION WITH execution/checkpointing.g4
 * ============================================================================
 *
 * Timeline checkpoints may reference the canonical checkpoint model.
 *
 * This grammar does not define checkpoint storage.
 *
 * It only preserves the relationship between:
 *
 *     timeline
 *     checkpoint
 *     restore/rewind intent.
 *
 * ============================================================================
 * 43. INTEGRATION WITH memory/
 * ============================================================================
 *
 * Timeline state may be persisted in:
 *
 *     memory;
 *     temporal memory;
 *     persistent storage;
 *     distributed storage;
 *     checkpoint storage.
 *
 * This grammar does not choose the storage representation.
 *
 * ============================================================================
 * 44. INTEGRATION WITH Sankofa
 * ============================================================================
 *
 * MTS and Sankofa may exchange temporal information.
 *
 * This grammar does not make Sankofa memory part of MTS syntax.
 *
 * Sankofa remains the authority for its memory semantics.
 *
 * A semantic adapter may associate:
 *
 *     timeline observation
 *     historical state
 *     provenance
 *     temporal knowledge
 *
 * where explicitly permitted.
 *
 * ============================================================================
 * 45. INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Timeline metadata can surround:
 *
 *     quantum::ir
 *
 * without modifying the quantum IR boundary.
 *
 * Example conceptual lowering:
 *
 *     timeline A
 *         |
 *         +--> quantum computation
 *         |
 *         +--> measurement
 *
 * The quantum computation remains represented by:
 *
 *     quantum::ir
 *
 * Timeline metadata is execution/semantic metadata.
 *
 * ============================================================================
 * 46. INTEGRATION WITH HDL
 * ============================================================================
 *
 * Timeline syntax can surround HDL/hardware computation.
 *
 * It does not define:
 *
 *     clocks;
 *     wires;
 *     registers;
 *     physical timing;
 *     FPGA placement;
 *     ASIC cells.
 *
 * Those remain owned by HDL/hardware semantics.
 *
 * ============================================================================
 * 47. INTEGRATION WITH DISTRIBUTED COMPUTATION
 * ============================================================================
 *
 * Timeline branches may eventually be distributed.
 *
 * The grammar does not require:
 *
 *     one branch = one node.
 *
 * The distributed compiler/runtime decides how semantic timeline work is
 * partitioned.
 *
 * ============================================================================
 * 48. INTEGRATION WITH RESILIENCE
 * ============================================================================
 *
 * Timeline failure handling may interact with:
 *
 *     resilience;
 *     recovery;
 *     checkpointing;
 *     fault tolerance.
 *
 * This grammar expresses intent only.
 *
 * Resilience remains responsible for runtime fault strategy.
 *
 * ============================================================================
 * 49. INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Timeline operations may access state across temporal boundaries.
 *
 * Semantic/security analysis must therefore enforce:
 *
 *     authorization;
 *     capability checks;
 *     provenance;
 *     isolation;
 *     non-interference;
 *     temporal integrity.
 *
 * The parser MUST NOT bypass these controls.
 *
 * ============================================================================
 * 50. INTEGRATION WITH COMPILATION
 * ============================================================================
 *
 * Compilation may:
 *
 *     specialize timeline operations;
 *     eliminate unnecessary branches;
 *     fuse compatible timelines;
 *     serialize branches;
 *     distribute branches;
 *     checkpoint;
 *     lower temporal relationships.
 *
 * Such transformations MUST preserve the language's semantic contract.
 *
 * The grammar does not perform those transformations.
 *
 * ============================================================================
 * 51. INTEGRATION WITH OPTIMIZATION
 * ============================================================================
 *
 * An optimizer may legally transform timeline realization when semantic
 * equivalence is proven.
 *
 * For example:
 *
 *     independent branches
 *
 * may be fused or serialized if the semantic contract permits it.
 *
 * The grammar does not define the optimizer.
 *
 * ============================================================================
 * 52. COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * Existing files MUST NOT be renamed merely because this file is introduced.
 *
 * Existing MTS concepts documented in:
 *
 *     grammar/Zamani-Grammar.md
 *     grammar/grammar.md
 *     grammar/DESIGN.md
 *
 * must be classified according to the repository's feature lifecycle:
 *
 *     stable
 *     proposed
 *     experimental
 *     deprecated
 *     historical
 *     not implemented
 *
 * Presence of this grammar does NOT automatically promote all MTS semantics
 * to stable implementation status.
 *
 * ============================================================================
 * 53. IMPLEMENTATION-CONFORMANCE REQUIREMENT
 * ============================================================================
 *
 * After this grammar is integrated:
 *
 *     grammar/grammar.md
 *
 * must accurately report whether MTS timeline syntax is:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 * depending on the actual frontend implementation.
 *
 * This file MUST NOT be used to falsely claim runtime implementation.
 *
 * ============================================================================
 * 54. TEST INTEGRATION
 * ============================================================================
 *
 * Recommended test layout:
 *
 *     grammar/tests/execution/timelines/
 *
 * with:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *
 * Minimum semantic test families:
 *
 *     declaration
 *     reference
 *     branch
 *     fork
 *     speculate
 *     observation
 *     synchronization
 *     join
 *     merge
 *     rewind
 *     checkpoint
 *     causal relation
 *     consistency
 *     conflict policy
 *     resource requirement
 *     capability requirement
 *     nested timeline
 *     distributed timeline
 *     quantum timeline
 *     hybrid timeline
 *
 * ============================================================================
 * 55. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] MTS is anchored by the existing canonical MTS lexer token.
 *     [x] No second lexer is introduced.
 *     [x] No second name grammar is introduced.
 *     [x] No second expression grammar is introduced.
 *     [x] Timeline operation names remain extensible.
 *     [x] Timeline properties remain extensible.
 *     [x] No universal timeline capacity exists.
 *     [x] No hardware capacity exists.
 *     [x] No physical device selection exists.
 *     [x] No runtime implementation exists.
 *     [x] No scheduler implementation exists.
 *     [x] No synchronization implementation exists.
 *     [x] No MTS-specific IR is introduced.
 *     [x] quantum::ir remains canonical.
 *     [x] Source spans are required downstream.
 *     [x] AST ownership is specified.
 *     [x] Semantic ownership is specified.
 *     [x] IR integration is specified.
 *     [x] Resource/capability integration is specified.
 *     [x] Quantum integration is specified.
 *     [x] HDL integration is specified.
 *     [x] Distributed integration is specified.
 *     [x] Sankofa integration boundary is specified.
 *     [x] Recovery/checkpoint integration is specified.
 *     [x] Scheduling integration is specified.
 *     [x] Security integration is specified.
 *     [x] Compatibility status is specified.
 *     [x] Positive tests are specified.
 *     [x] Negative tests are specified.
 *     [x] Boundary tests are specified.
 *     [x] Scalability tests are specified.
 *     [x] Determinism requirements are specified.
 *     [x] Rust 1.97/1.97.1 compatibility is specified.
 *     [x] No unsafe Rust is required.
 *
 * ============================================================================
 * 56. FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * MTS syntax describes TEMPORAL EXECUTION INTENT.
 *
 * It does not describe:
 *
 *     today's machine;
 *     today's scheduler;
 *     today's storage;
 *     today's QPU;
 *     today's cluster;
 *     today's number of timelines.
 *
 * Therefore:
 *
 *     PROGRAM TEMPORAL SCALE
 *             !=
 *     MACHINE TEMPORAL CAPACITY
 *
 * and:
 *
 *     TIMELINE SYNTAX
 *             !=
 *     TIMELINE ALLOCATION
 *
 * and:
 *
 *     TIMELINE INTENT
 *             !=
 *     TIMELINE RUNTIME
 *
 * The final architecture remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject only to program semantics, implementation capability and actual
 * resources available at realization time.
 *
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar ExecutionTimelines;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions;


/*
 * ============================================================================
 * 57. PUBLIC ROOT
 * ============================================================================
 *
 * A timeline construct is anchored by the existing MTS lexical keyword.
 *
 * The following structural form is intentionally open:
 *
 *     mts <operation> ...
 *
 * This avoids introducing a new lexer keyword for every future MTS operation.
 *
 * ============================================================================
 */

timelineConstruct
    : MTS timelineOperation
    ;


/*
 * ============================================================================
 * 58. TIMELINE OPERATION
 * ============================================================================
 *
 * An operation consists of:
 *
 *     operation name
 *     optional operation target
 *     optional argument list
 *     optional relation
 *     optional body
 *     optional terminator
 *
 * The operation name is an identifier/qualified name rather than a closed
 * parser enumeration.
 *
 * ============================================================================
 */

timelineOperation
    : timelineOperationName
      timelineOperationTarget?
      timelineOperationArguments?
      timelineOperationRelation?
      timelineOperationBody?
      timelineTerminator
    ;


/*
 * ============================================================================
 * 59. OPERATION NAME
 * ============================================================================
 *
 * The operation namespace is open-ended.
 *
 * Examples:
 *
 *     timeline
 *     branch
 *     fork
 *     speculate
 *     merge
 *     rewind
 *     restore
 *     checkpoint
 *     select
 *     synchronize
 *
 * `observe` is included explicitly because the canonical lexer already owns
 * OBSERVE as a keyword.
 *
 * Future operations do not require new lexer tokens.
 */

timelineOperationName
    : qualifiedName
    | OBSERVE
    ;


/*
 * ============================================================================
 * 60. TARGET
 * ============================================================================
 *
 * A target identifies a symbolic timeline or execution object.
 *
 * It is NOT a physical hardware target.
 *
 * ============================================================================
 */

timelineOperationTarget
    : timelineReference
    ;


/*
 * ============================================================================
 * 61. ARGUMENTS
 * ============================================================================
 *
 * Generic expressions are reused.
 *
 * No second expression grammar is created.
 *
 * ============================================================================
 */

timelineOperationArguments
    : LPAREN timelineArgumentList? RPAREN
    ;


timelineArgumentList
    : expression (COMMA expression)*
    ;


/*
 * ============================================================================
 * 62. RELATION
 * ============================================================================
 *
 * Timeline relationships can use canonical names and expressions.
 *
 * Examples:
 *
 *     from main
 *     with parent
 *     after checkpoint
 *
 * The relation name remains open-ended.
 *
 * ============================================================================
 */

timelineOperationRelation
    : timelineRelationKeyword timelineRelationValue
    ;


timelineRelationKeyword
    : FROM
    | WITH
    | WHEN
    | qualifiedName
    ;


timelineRelationValue
    : timelineReference
    | expression
    ;


/*
 * ============================================================================
 * 63. BODY
 * ============================================================================
 *
 * A body is a sequence of timeline clauses.
 *
 * There is no finite clause limit.
 *
 * ============================================================================
 */

timelineOperationBody
    : LBRACE timelineClause* RBRACE
    ;


/*
 * ============================================================================
 * 64. TERMINATOR
 * ============================================================================
 *
 * A timeline operation may either be a declaration/body form or a simple
 * terminated operation.
 *
 * ============================================================================
 */

timelineTerminator
    : SEMICOLON
    |
    ;


/*
 * ============================================================================
 * 65. TIMELINE REFERENCE
 * ============================================================================
 *
 * Timeline references use canonical qualified names.
 *
 * They remain symbolic until semantic resolution.
 *
 * ============================================================================
 */

timelineReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 66. TIMELINE CLAUSES
 * ============================================================================
 */

timelineClause
    : timelineProperty
    | timelineRequirement
    | timelineConstraint
    | timelinePreference
    | timelineHint
    | timelineRelationClause
    | timelineNestedOperation
    ;


/*
 * ============================================================================
 * 67. PROPERTY
 * ============================================================================
 *
 * Generic extensible property:
 *
 *     causal::policy: strict;
 *
 *     merge::policy: reconcile;
 *
 *     observation::mode: isolated;
 *
 * The property name is open-ended.
 *
 * ============================================================================
 */

timelineProperty
    : qualifiedName COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 68. REQUIREMENT
 * ============================================================================
 *
 * The concrete requirement keyword is canonical.
 *
 * The value is an ordinary expression.
 *
 * Resource/capability semantics remain downstream.
 *
 * ============================================================================
 */

timelineRequirement
    : REQUIRES expression SEMICOLON
    ;


/*
 * ============================================================================
 * 69. CONSTRAINT
 * ============================================================================
 */

timelineConstraint
    : CONSTRAINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 70. PREFERENCE
 * ============================================================================
 */

timelinePreference
    : PREFER expression SEMICOLON
    ;


/*
 * ============================================================================
 * 71. HINT
 * ============================================================================
 */

timelineHint
    : HINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 72. RELATION CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     causal::parent: root;
 *
 *     timeline::depends_on: other;
 *
 *     merge::source: branch_a;
 *
 * ============================================================================
 */

timelineRelationClause
    : qualifiedName COLON timelineReference SEMICOLON
    ;


/*
 * ============================================================================
 * 73. NESTED OPERATION
 * ============================================================================
 *
 * Nested timeline constructs use the existing MTS introducer.
 *
 * This permits hierarchical temporal structures without fixed nesting depth.
 *
 * ============================================================================
 */

timelineNestedOperation
    : MTS timelineOperation
    ;