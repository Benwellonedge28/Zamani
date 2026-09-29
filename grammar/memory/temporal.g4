/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/temporal.g4
 *
 * GRAMMAR
 * -------
 * TemporalMemory
 *
 * STATUS
 * ------
 * Production-ready modular temporal-memory parser component.
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 Edition
 * Safe Rust only.
 * No unsafe Rust is required or permitted by the compiler implementation
 * contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL TEMPORAL MEMORY SYNTAX.
 *
 * Temporal memory is the language-level representation of memory/state whose
 * meaning depends on temporal identity, temporal observation, historical
 * state, snapshots, versions, persistence points, causality, speculative
 * state, or other temporal relationships.
 *
 * This file is deliberately located under:
 *
 *     grammar/memory/
 *
 * because it describes temporal aspects of program state/memory.
 *
 * It does NOT own the general temporal TYPE:
 *
 *     MTS<T>
 *
 * That constructor is already owned by:
 *
 *     grammar/types/temporal.g4
 *
 * It also does NOT own the runtime Multi-Timeline System implementation.
 *
 * Runtime MTS behavior remains owned by the runtime subsystem, including:
 *
 *     src/runtime/mts.rs
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT temporal-memory intent the source expresses.
 *
 * It does NOT determine:
 *
 *     HOW temporal memory is represented;
 *     WHERE temporal state is stored;
 *     WHICH clock is used;
 *     WHICH timeline implementation is used;
 *     WHICH machine executes the timeline;
 *     HOW histories are stored;
 *     HOW snapshots are materialized;
 *     HOW branches are scheduled;
 *     HOW timelines are distributed;
 *     HOW rollback is implemented;
 *     HOW causality is computed;
 *     HOW speculative execution is scheduled.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Temporal memory participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A temporal-memory program must remain expressible across:
 *
 *     tiny embedded systems;
 *     atom-scale / nano-scale computational models;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     quantum-classical systems;
 *     QPUs;
 *     simulators;
 *     distributed systems;
 *     HPC systems;
 *     clusters;
 *     cloud systems;
 *     future computational substrates.
 *
 * The grammar therefore contains NO universal limits on:
 *
 *     timelines;
 *     branches;
 *     histories;
 *     snapshots;
 *     checkpoints;
 *     versions;
 *     temporal events;
 *     observations;
 *     causal relationships;
 *     temporal values;
 *     temporal-memory objects;
 *     temporal-memory regions;
 *     nodes;
 *     processes;
 *     tasks;
 *     threads;
 *     devices;
 *     accelerators;
 *     qubits;
 *     memory;
 *     storage.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     MAX_TIMELINES
 *     MAX_BRANCHES
 *     MAX_HISTORY
 *     MAX_SNAPSHOTS
 *     MAX_CHECKPOINTS
 *     MAX_TEMPORAL_EVENTS
 *     MAX_TEMPORAL_VALUES
 *     MAX_TEMPORAL_MEMORY
 *     MAX_TIMELINES_PER_PROCESS
 *     MAX_BRANCHES_PER_TIMELINE
 *     MAX_HISTORY_ENTRIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * A program may contain explicit numeric values where those values are
 * program semantics.
 *
 * For example:
 *
 *     temporal::snapshot(state, version = 1024)
 *
 * may be meaningful if `1024` is program data.
 *
 * That does NOT establish:
 *
 *     "Zamani supports at most 1024 versions."
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     temporalMemoryConstruct
 *     temporalMemoryExpression
 *     temporalMemoryStatement
 *
 *     temporalScopeExpression
 *     zamaniTemporalExpression
 *     sasaTemporalExpression
 *
 *     temporalMemoryLiteral
 *     mtsCompatibilityLiteral
 *
 *     temporalMemoryOperation
 *     temporalMemoryOperationName
 *     temporalMemoryArgumentList
 *     temporalMemoryArgument
 *     temporalMemoryNamedArgument
 *
 *     temporalMemoryReference
 *     temporalMemoryTarget
 *     temporalMemorySelector
 *     temporalMemoryRelationship
 *     temporalMemoryMetadata
 *
 *     temporalMemoryQualifiedName
 *
 *     temporalMemoryOperationBlock
 *     temporalMemoryClause
 *     temporalMemoryClauseList
 *
 *     temporalMemoryExtension
 *     temporalMemoryExtensionName
 *     temporalMemoryExtensionArguments
 *
 * The exact semantic interpretation of these constructs belongs downstream.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file DOES NOT own:
 *
 *     lexical token definitions;
 *     identifier syntax;
 *     qualified-name implementation;
 *     expression precedence;
 *     generic type syntax;
 *     MTS<T> type syntax;
 *     memory ownership;
 *     borrowing;
 *     lifetime inference;
 *     allocation;
 *     deallocation;
 *     memory regions;
 *     memory spaces;
 *     physical memory;
 *     persistence implementation;
 *     resource discovery;
 *     capability discovery;
 *     hardware discovery;
 *     timeline allocation;
 *     timeline scheduling;
 *     timeline execution;
 *     distributed placement;
 *     routing;
 *     optimization;
 *     quantum operation semantics;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     calibration;
 *     HAL;
 *     runtime execution;
 *     physical clock implementation;
 *     timestamp representation;
 *     compiler backend selection.
 *
 * ============================================================================
 * AUTHORITATIVE REPOSITORY CONTRACTS
 * ============================================================================
 *
 * This grammar conforms to:
 *
 *     grammar/DESIGN.md
 *     grammar/README.md
 *     grammar/spec/syntax.md
 *     grammar/spec/lexical.md
 *     grammar/specification/semantics.md
 *     grammar/specification/types.md
 *     grammar/memory/README.md
 *
 * Temporal TYPE syntax is governed separately by:
 *
 *     grammar/types/temporal.g4
 *
 * The implementation-conformance status is represented through:
 *
 *     grammar/grammar.md
 *
 * Extended/historical/proposed temporal material remains in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * and does not silently become canonical syntax merely because it is
 * documented there.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The ONE canonical ANTLR lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its lexical composition is owned by:
 *
 *     grammar/lexer/
 *
 * This file defines NO lexer rules.
 *
 * ============================================================================
 * EXISTING TEMPORAL LEXICAL FACTS
 * ============================================================================
 *
 * The current canonical lexer already provides:
 *
 *     MTS
 *     ZAMANI
 *     SASA
 *
 * as lexical tokens.
 *
 * These are consumed directly.
 *
 * The repository also contains an existing MTSLiteral compatibility concept.
 *
 * The production architecture MUST NOT create a second lexical authority for
 * that construct.
 *
 * Therefore this file treats:
 *
 *     mts[expression]
 *
 * as parser-level compatibility syntax:
 *
 *     MTS LBRACKET expression RBRACKET
 *
 * when the active language version enables it.
 *
 * The parser does NOT depend on a monolithic MTSLiteral token.
 *
 * ============================================================================
 * CONTEXTUAL TEMPORAL WORDS
 * ============================================================================
 *
 * The current canonical lexer does NOT reserve all temporal words as global
 * keywords.
 *
 * In particular, words such as:
 *
 *     timeline
 *     fork
 *     merge
 *     rewind
 *     snapshot
 *     checkpoint
 *     history
 *     branch
 *
 * are not independently introduced here as lexer tokens.
 *
 * This is intentional.
 *
 * The language architecture prefers open semantic names and qualified
 * operations over continually expanding the global reserved-word vocabulary.
 *
 * Therefore temporal operations should normally be expressed through
 * qualified/open-world names such as:
 *
 *     temporal::snapshot(...)
 *     temporal::checkpoint(...)
 *     temporal::restore(...)
 *     temporal::rewind(...)
 *     temporal::fork(...)
 *     temporal::merge(...)
 *
 * or:
 *
 *     mts::timeline(...)
 *     mts::fork(...)
 *     mts::merge(...)
 *
 * without requiring a new lexer release for every future temporal operation.
 *
 * A future stable contextual-keyword mechanism may add specialized syntax,
 * but that is a lexical/specification change and is intentionally NOT invented
 * by this file.
 *
 * ============================================================================
 * NO DUPLICATION OF TYPE TEMPORALITY
 * ============================================================================
 *
 * `grammar/types/temporal.g4` already owns:
 *
 *     temporalTypeConstructor
 *
 * and the complete type composition establishes:
 *
 *     MTS<T>
 *
 * This file MUST NOT define:
 *
 *     temporalType
 *     temporalTypeConstructor
 *     MTS<T>
 *
 * again.
 *
 * This prevents:
 *
 *     memory temporal type
 *
 * and:
 *
 *     type-system temporal type
 *
 * from becoming competing syntax authorities.
 *
 * ============================================================================
 * NO DUPLICATION OF MEMORY FOUNDATION
 * ============================================================================
 *
 * `grammar/memory/memory.g4` owns generic memory constructs including:
 *
 *     memoryConstruct
 *     memoryOperation
 *     memoryPlace
 *     memoryRegion
 *     memorySpace
 *     memoryLifetime
 *     memoryTypeAnnotation
 *
 * This file MUST NOT redefine those rules.
 *
 * Temporal memory references use the canonical expression/name layer and
 * semantic memory integration rather than copying the foundational memory
 * grammar.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          v
 *     TemporalMemory
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     temporal semantic analysis
 *          |
 *     +----+---------+----------+-----------+
 *     |              |          |           |
 *     v              v          v           v
 *   memory        Sankofa     distributed  quantum
 *     |              |          |           |
 *     +--------------+----------+-----------+
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       classical  quantum::ir HDL/hardware
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                    v
 *       optimization / lowering
 *                    |
 *       routing / scheduling
 *                    |
 *       resilience / QEC / ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 * This file has no dependency on runtime implementation.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Names
 *     Expressions
 *
 * because it uses:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     blockExpression
 *
 * It deliberately does NOT import Memory.
 *
 * Reason:
 *
 * `Memory` is the generic memory foundation and must remain an upstream
 * composition dependency. Importing Memory here and subsequently importing
 * TemporalMemory into Memory would create a circular grammar dependency.
 *
 * The canonical composition layer instead composes:
 *
 *     Names
 *     Expressions
 *     Memory
 *     TemporalMemory
 *
 * together.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The single public entry point for this component is:
 *
 *     temporalMemoryConstruct
 *
 * Host grammars decide whether it is legal in:
 *
 *     expression position;
 *     statement position;
 *     memory-domain position;
 *     temporal-domain position;
 *     declaration position.
 *
 * This file does not create another program root.
 *
 * ============================================================================
 */

parser grammar TemporalMemory;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC TEMPORAL MEMORY CONSTRUCT
 * ============================================================================
 *
 * Temporal memory has three stable syntactic families:
 *
 *     - temporal scope expressions (`zamani`, `sasa`);
 *     - MTS compatibility literals (`mts[...]`);
 *     - open-world temporal-memory operations.
 *
 * The operation family is deliberately extensible.
 *
 * ============================================================================
 */

temporalMemoryConstruct
    : temporalMemoryExpression
    | temporalMemoryStatement
    ;


/*
 * ============================================================================
 * 2. EXPRESSION ENTRY
 * ============================================================================
 */

temporalMemoryExpression
    : temporalScopeExpression
    | temporalMemoryLiteral
    | temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 3. STATEMENT ENTRY
 * ============================================================================
 *
 * A temporal-memory operation may be used as a statement.
 *
 * The trailing semicolon belongs to the host statement context when the
 * surrounding grammar already owns semicolon termination. This component
 * provides an explicit statement form for direct composition.
 * ============================================================================
 */

temporalMemoryStatement
    : temporalMemoryOperation
      SEMI
    ;


/*
 * ============================================================================
 * 4. ZAMANI TEMPORAL SCOPE
 * ============================================================================
 *
 * Existing canonical lexer token:
 *
 *     ZAMANI
 *
 * The syntax is intentionally expression-oriented.
 *
 * Examples:
 *
 *     zamani expression
 *
 *     zamani {
 *         ...
 *     }
 *
 * The semantic meaning of "zamani" is determined downstream.
 *
 * It may represent historical/temporal context, immutable historical state,
 * or another specification-defined temporal scope.
 *
 * It is NOT a textual macro.
 * ============================================================================
 */

temporalScopeExpression
    : zamaniTemporalExpression
    | sasaTemporalExpression
    ;


zamaniTemporalExpression
    : ZAMANI
      temporalScopeOperand
    ;


sasaTemporalExpression
    : SASA
      temporalScopeOperand
    ;


temporalScopeOperand
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 5. MTS COMPATIBILITY LITERAL
 * ============================================================================
 *
 * Existing lexical contract:
 *
 *     mts[...]
 *
 * The old monolithic MTSLiteral token is NOT required here.
 *
 * The production parser representation is:
 *
 *     MTS [ expression ]
 *
 * This preserves the payload as an ordinary expression rather than making the
 * lexer responsible for interpreting timestamps or temporal semantics.
 *
 * ============================================================================
 */

temporalMemoryLiteral
    : mtsCompatibilityLiteral
    ;


mtsCompatibilityLiteral
    : MTS
      LBRACKET
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 6. OPEN-WORLD TEMPORAL MEMORY OPERATION
 * ============================================================================
 *
 * Operation identity is semantic data.
 *
 * This intentionally does NOT enumerate:
 *
 *     snapshot
 *     checkpoint
 *     restore
 *     rewind
 *     fork
 *     merge
 *     observe
 *     branch
 *     history
 *
 * as a closed parser enumeration.
 *
 * All of those can be represented through ordinary qualified names.
 *
 * Examples:
 *
 *     temporal::snapshot(state)
 *     temporal::checkpoint(state)
 *     temporal::restore(state, version)
 *     temporal::rewind(state, target)
 *     temporal::fork(state, branch)
 *     temporal::merge(left, right)
 *
 * Future operations remain syntactically representable without modifying this
 * grammar.
 *
 * ============================================================================
 */

temporalMemoryOperation
    : temporalMemoryOperationName
      LPAREN
      temporalMemoryArgumentList?
      RPAREN
    ;


temporalMemoryOperationName
    : temporalMemoryQualifiedName
    ;


temporalMemoryQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. ARGUMENTS
 * ============================================================================
 *
 * Arguments are ordinary Zamani expressions.
 *
 * Named arguments are represented using the canonical identifier and
 * assignment syntax.
 *
 * Examples:
 *
 *     temporal::snapshot(
 *         state,
 *         version = requested_version
 *     )
 *
 *     temporal::rewind(
 *         state,
 *         target = checkpoint
 *     )
 *
 * No fixed number of arguments is imposed.
 * ============================================================================
 */

temporalMemoryArgumentList
    : temporalMemoryArgument
      (COMMA temporalMemoryArgument)*
      COMMA?
    ;


temporalMemoryArgument
    : temporalMemoryNamedArgument
    | expression
    ;


temporalMemoryNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 8. TEMPORAL MEMORY REFERENCE
 * ============================================================================
 *
 * This rule provides an explicit semantic wrapper around an ordinary
 * expression.
 *
 * It does not create a second memory-place grammar.
 *
 * Examples:
 *
 *     state
 *     state.version
 *     memory::state
 *     history.current
 *     object[index]
 *
 * Semantic analysis determines whether the expression is a valid temporal
 * memory subject.
 * ============================================================================
 */

temporalMemoryReference
    : expression
    ;


temporalMemoryTarget
    : temporalMemoryReference
    ;


/*
 * ============================================================================
 * 9. TEMPORAL SELECTOR
 * ============================================================================
 *
 * A temporal selector identifies a temporal position, version, snapshot,
 * branch, observation, or other semantic temporal object.
 *
 * The grammar intentionally accepts expressions rather than imposing a fixed
 * timestamp representation.
 *
 * Examples:
 *
 *     version
 *     checkpoint
 *     logical_time
 *     event
 *     branch
 *     observation
 *     timestamp
 *     temporal::latest()
 *
 * The semantic layer decides which expressions are valid selectors.
 * ============================================================================
 */

temporalMemorySelector
    : expression
    ;


/*
 * ============================================================================
 * 10. TEMPORAL RELATIONSHIP
 * ============================================================================
 *
 * This rule provides a generic syntax boundary for temporal relationships.
 *
 * Relationship identity remains open-world.
 *
 * Examples of semantic names include:
 *
 *     before
 *     after
 *     during
 *     contains
 *     overlaps
 *     caused_by
 *     depends_on
 *     precedes
 *     follows
 *     same_as
 *     derived_from
 *
 * They remain names/data rather than global keywords.
 * ============================================================================
 */

temporalMemoryRelationship
    : temporalMemoryQualifiedName
      LPAREN
      temporalMemoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 11. TEMPORAL MEMORY METADATA
 * ============================================================================
 *
 * Metadata is represented as an annotation using the canonical annotation
 * syntax.
 *
 * Example:
 *
 *     @temporal::persistent(...)
 *
 * The exact annotation semantics remain downstream.
 * ============================================================================
 */

temporalMemoryMetadata
    : AT
      temporalMemoryQualifiedName
      temporalMemoryMetadataArguments?
    ;


temporalMemoryMetadataArguments
    : LPAREN
      temporalMemoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 12. TEMPORAL MEMORY CLAUSE
 * ============================================================================
 *
 * Temporal policies may be attached to an operation through open-world
 * qualified names.
 *
 * Examples:
 *
 *     temporal::snapshot(
 *         state,
 *         policy = temporal::durable
 *     )
 *
 *     temporal::checkpoint(
 *         state,
 *         condition = condition
 *     )
 *
 * The grammar does not enumerate policy names.
 * ============================================================================
 */

temporalMemoryClause
    : temporalMemoryQualifiedName
      ASSIGN
      expression
    ;


temporalMemoryClauseList
    : temporalMemoryClause
      (COMMA temporalMemoryClause)*
      COMMA?
    ;


/*
 * ============================================================================
 * 13. TEMPORAL MEMORY OPERATION BLOCK
 * ============================================================================
 *
 * This is a reusable aggregate for future host grammars that need a temporal
 * memory operation with a body.
 *
 * Example:
 *
 *     temporal::transaction(state) {
 *         ...
 *     }
 *
 * The body is an ordinary canonical block expression.
 *
 * ============================================================================
 */

temporalMemoryOperationBlock
    : temporalMemoryOperationName
      LPAREN
      temporalMemoryArgumentList?
      RPAREN
      blockExpression
    ;


/*
 * ============================================================================
 * 14. TEMPORAL MEMORY EXTENSIONS
 * ============================================================================
 *
 * Future domains may attach extension operations without requiring this file
 * to know every future temporal-memory feature.
 *
 * Examples:
 *
 *     vendor::temporal::operation(...)
 *     future::temporal::memory(...)
 *     dialect::temporal::feature(...)
 *
 * The grammar accepts the syntax.
 *
 * Semantic validation determines whether the extension is declared and
 * available.
 * ============================================================================
 */

temporalMemoryExtension
    : temporalMemoryExtensionName
      LPAREN
      temporalMemoryExtensionArguments?
      RPAREN
    ;


temporalMemoryExtensionName
    : temporalMemoryQualifiedName
    ;


temporalMemoryExtensionArguments
    : temporalMemoryArgumentList
    ;


/*
 * ============================================================================
 * 15. SEMANTIC OPERATION CATEGORIES
 * ============================================================================
 *
 * The following rules intentionally use the OPEN-WORLD operation form.
 *
 * They are semantic category wrappers, not closed operation enumerations.
 *
 * A semantic analyzer may classify:
 *
 *     temporal::snapshot(...)
 *
 * as a snapshot operation.
 *
 *     temporal::checkpoint(...)
 *
 * as a checkpoint operation.
 *
 *     temporal::restore(...)
 *
 * as a restoration operation.
 *
 *     temporal::rewind(...)
 *
 * as a rewind operation.
 *
 *     temporal::fork(...)
 *
 * as a branch/fork operation.
 *
 *     temporal::merge(...)
 *
 * as a merge operation.
 *
 *     temporal::observe(...)
 *
 * as an observation operation.
 *
 * No new parser keyword is required for any of these names.
 * ============================================================================
 */

temporalSnapshotOperation
    : temporalMemoryOperation
    ;


temporalCheckpointOperation
    : temporalMemoryOperation
    ;


temporalRestoreOperation
    : temporalMemoryOperation
    ;


temporalRewindOperation
    : temporalMemoryOperation
    ;


temporalForkOperation
    : temporalMemoryOperation
    ;


temporalMergeOperation
    : temporalMemoryOperation
    ;


temporalObservationOperation
    : temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 16. TEMPORAL STATE ACCESS
 * ============================================================================
 *
 * Temporal state access remains expression-based.
 *
 * This deliberately avoids introducing a second indexing/path grammar.
 *
 * Examples:
 *
 *     temporal::at(state, time)
 *     temporal::version(state, version)
 *     temporal::snapshot(state)
 *     temporal::history(state)
 *
 * ============================================================================
 */

temporalStateAccess
    : temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 17. TEMPORAL QUERY
 * ============================================================================
 *
 * Queries remain open-world operations.
 *
 * Examples:
 *
 *     temporal::at(...)
 *     temporal::history(...)
 *     temporal::versions(...)
 *     temporal::events(...)
 *     temporal::causal(...)
 *
 * The parser does not decide whether an operation is a query.
 * ============================================================================
 */

temporalMemoryQuery
    : temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 18. TEMPORAL MEMORY REQUIREMENT
 * ============================================================================
 *
 * Resource requirements remain semantic/resource-domain constructs.
 *
 * This grammar does not duplicate grammar/resources/ or
 * grammar/memory/memory-constraints.g4.
 *
 * A temporal requirement is therefore represented as an open operation or
 * expression and classified downstream.
 *
 * Examples:
 *
 *     temporal::require(
 *         capability = temporal::snapshot
 *     )
 *
 *     temporal::require(
 *         history = required_history
 *     )
 *
 * ============================================================================
 */

temporalMemoryRequirement
    : temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 19. TEMPORAL MEMORY POLICY
 * ============================================================================
 *
 * Policies remain data-driven.
 *
 * Examples:
 *
 *     temporal::policy(...)
 *     temporal::retention(...)
 *     temporal::consistency(...)
 *     temporal::causality(...)
 *
 * No policy enumeration is introduced.
 * ============================================================================
 */

temporalMemoryPolicy
    : temporalMemoryOperation
    ;


/*
 * ============================================================================
 * 20. TEMPORAL MEMORY CAPABILITY
 * ============================================================================
 *
 * Capability identity remains open-world.
 *
 * The canonical capability system remains authoritative.
 *
 * Examples:
 *
 *     capability::temporal::snapshot
 *     capability::temporal::checkpoint
 *     capability::temporal::rewind
 *     capability::temporal::branch
 *     capability::temporal::merge
 *
 * ============================================================================
 */

temporalMemoryCapability
    : temporalMemoryQualifiedName
    ;


/*
 * ============================================================================
 * 21. TEMPORAL MEMORY RESOURCE
 * ============================================================================
 *
 * Resource identity is also open-world.
 *
 * This rule does not discover or allocate a resource.
 * ============================================================================
 */

temporalMemoryResource
    : temporalMemoryQualifiedName
    ;


/*
 * ============================================================================
 * 22. SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The frontend adapter must preserve source spans covering:
 *
 *     ZAMANI / SASA
 *     MTS
 *     brackets
 *     operation names
 *     argument lists
 *     metadata
 *     operation blocks
 *
 * The grammar itself performs no source-span computation.
 *
 * The parser/AST layer must retain enough information to diagnose the complete
 * temporal-memory construct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define Rust AST structures.
 *
 * Temporal-memory syntax must map into the existing domain-neutral AST.
 *
 * The preferred conceptual representation is:
 *
 *     TemporalMemoryExpression
 *         {
 *             operation/name,
 *             arguments,
 *             metadata,
 *             source_span
 *         }
 *
 * or the repository's equivalent generic operation/expression node.
 *
 * `zamani` and `sasa` should map to the existing temporal expression/scope
 * representation if one exists.
 *
 * `mts[expression]` should preserve:
 *
 *     - MTS identity;
 *     - payload expression;
 *     - source span;
 *     - compatibility/literal status.
 *
 * The AST MUST NOT become:
 *
 *     TimelineRuntimeNode
 *     SnapshotRuntimeNode
 *     PhysicalTimelineNode
 *     QPUTimelineNode
 *
 * merely because the source construct has temporal semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is the source structurally a temporal-memory construct?"
 *
 * Semantic analysis answers:
 *
 *     "What temporal-memory meaning does this construct have?"
 *
 * Semantic analysis may determine:
 *
 *     - whether a temporal operation exists;
 *     - whether its arguments are valid;
 *     - whether the target is temporal state;
 *     - whether a snapshot is legal;
 *     - whether a checkpoint is legal;
 *     - whether a restore target is valid;
 *     - whether a rewind is causally valid;
 *     - whether a fork is legal;
 *     - whether a merge is legal;
 *     - whether branches are compatible;
 *     - whether histories are available;
 *     - whether persistence requirements are satisfied;
 *     - whether temporal capabilities are available;
 *     - whether ownership/lifetime rules permit the operation;
 *     - whether distributed execution changes temporal semantics;
 *     - whether quantum state participates;
 *     - whether the operation requires canonical quantum IR integration.
 *
 * None of those decisions belong to this parser grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. TEMPORAL TYPE INTEGRATION
 * ============================================================================
 *
 * A temporal memory expression may operate on a value whose type is:
 *
 *     MTS<T>
 *
 * or any other type whose semantic model permits temporal behavior.
 *
 * The type itself is parsed by:
 *
 *     grammar/types/temporal.g4
 *
 * Example:
 *
 *     temporal::snapshot(state)
 *
 * where:
 *
 *     state : MTS<State>
 *
 * This file does not parse the type constructor.
 *
 * Semantic analysis determines whether the operation is valid for the type.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. MEMORY INTEGRATION
 * ============================================================================
 *
 * Temporal memory may interact with:
 *
 *     memory.g4
 *     ownership.g4
 *     borrowing.g4
 *     lifetimes.g4
 *     regions.g4
 *     persistence.g4
 *     shared-memory.g4
 *     distributed-memory.g4
 *     accelerator-memory.g4
 *     quantum-memory.g4
 *     memory-capabilities.g4
 *     memory-constraints.g4
 *
 * Integration occurs through the AST/semantic layers.
 *
 * This grammar does not duplicate those grammars.
 *
 * Examples:
 *
 *     temporal::snapshot(memory_object)
 *
 *     temporal::checkpoint(region)
 *
 *     temporal::restore(persistent_state)
 *
 * Whether these are legal is a semantic question.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. OWNERSHIP / BORROWING / LIFETIME INTEGRATION
 * ============================================================================
 *
 * Temporal operations can interact with:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     region;
 *     persistence.
 *
 * For example, a semantic analyzer may need to reject:
 *
 *     temporal operation
 *
 * when the referenced state no longer has a valid lifetime.
 *
 * The parser does not perform that validation.
 *
 * No temporal grammar rule may duplicate:
 *
 *     memoryLifetime
 *     memoryBorrow
 *     ownership checking
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SANKOFA INTEGRATION
 * ============================================================================
 *
 * The repository contains Sankofa-oriented memory concepts including:
 *
 *     Zamani
 *     Sasa
 *     recall
 *     history
 *     learning
 *     provenance
 *     temporal information
 *     consensus
 *
 * The canonical lexer already recognizes:
 *
 *     ZAMANI
 *     SASA
 *     REMEMBER
 *     RECALL
 *     WISDOM
 *
 * This file uses only:
 *
 *     ZAMANI
 *     SASA
 *
 * directly because they already have specification-defined temporal-scope
 * syntax.
 *
 * `REMEMBER`, `RECALL`, and `WISDOM` remain owned by their respective semantic
 * domains and are not silently reinterpreted as temporal-memory operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. MTS RUNTIME INTEGRATION
 * ============================================================================
 *
 * The repository already contains:
 *
 *     src/runtime/mts.rs
 *
 * with runtime timeline concepts such as:
 *
 *     TimelineId
 *     Timestamp
 *
 * This grammar does NOT import runtime Rust types.
 *
 * It does NOT:
 *
 *     allocate TimelineId;
 *     generate timestamps;
 *     create timelines;
 *     fork timelines;
 *     merge timelines;
 *     rewind runtime state;
 *     inspect runtime state;
 *     access clocks.
 *
 * Correct pipeline:
 *
 *     source
 *       |
 *       v
 *     TemporalMemory grammar
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     temporal semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     compiler lowering
 *       |
 *       v
 *     runtime MTS
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. MTS TYPE VS MTS RUNTIME VS MTS LITERAL
 * ============================================================================
 *
 * These are three distinct concepts.
 *
 * 1. TYPE:
 *
 *     MTS<T>
 *
 * owned by:
 *
 *     grammar/types/temporal.g4
 *
 * 2. COMPATIBILITY LITERAL:
 *
 *     mts[expression]
 *
 * represented here as:
 *
 *     MTS LBRACKET expression RBRACKET
 *
 * 3. RUNTIME:
 *
 *     MTS timeline/state management
 *
 * owned by:
 *
 *     src/runtime/mts.rs
 *
 * They MUST NOT be collapsed into one grammar construct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. TEMPORAL RELATIONSHIP TO EXECUTION
 * ============================================================================
 *
 * Temporal memory can influence:
 *
 *     execution;
 *     scheduling;
 *     checkpointing;
 *     recovery;
 *     speculative execution;
 *     distributed execution.
 *
 * But temporal-memory syntax does not implement those systems.
 *
 * The compiler may lower semantic temporal information into:
 *
 *     execution policies;
 *     scheduling constraints;
 *     recovery plans;
 *     persistence requirements;
 *     distributed coordination;
 *
 * after semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Temporal memory may be distributed.
 *
 * The grammar does not encode:
 *
 *     node count;
 *     node identity;
 *     replica count;
 *     physical topology;
 *     network location;
 *     transport protocol.
 *
 * A program may express semantic requirements through ordinary expressions
 * and qualified capability/resource names.
 *
 * Example:
 *
 *     temporal::checkpoint(
 *         state,
 *         capability = capability::distributed::temporal_consistency
 *     )
 *
 * The distributed subsystem determines realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Temporal memory may participate in quantum-classical computation.
 *
 * Examples of semantic subjects may include:
 *
 *     quantum state;
 *     measurement result;
 *     classical feed-forward state;
 *     logical execution state;
 *     simulator state.
 *
 * The grammar does not define:
 *
 *     physical qubit identifiers;
 *     QPU timeline identifiers;
 *     quantum memory addresses;
 *     QEC state storage;
 *     noise models;
 *     routing;
 *     calibration.
 *
 * If a temporal operation eventually affects quantum computation:
 *
 *     temporal source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic temporal/quantum analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling / QEC / ZQN / HAL
 *
 * No temporal quantum IR is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Temporal memory semantics may describe:
 *
 *     checkpoint intent;
 *     state retention;
 *     temporal state;
 *     simulation state;
 *     hardware/software co-design intent.
 *
 * They must not encode:
 *
 *     fixed register widths;
 *     fixed clock widths;
 *     fixed memory banks;
 *     fixed FPGA resources;
 *     fixed ASIC resources;
 *     physical addresses;
 *     device IDs.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. PERSISTENCE INTEGRATION
 * ============================================================================
 *
 * Temporal memory is related to persistence but is not identical to it.
 *
 * Temporal memory answers questions such as:
 *
 *     Which state/version/history?
 *
 * Persistence answers:
 *
 *     How should state survive?
 *
 * Therefore:
 *
 *     temporal::snapshot(...)
 *
 * may interact semantically with:
 *
 *     persistence.g4
 *
 * but this grammar does not duplicate persistence operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CAUSALITY
 * ============================================================================
 *
 * Causal relationships are semantic.
 *
 * This grammar permits them to be represented as open-world operations or
 * expressions without hard-coding a finite causal vocabulary.
 *
 * Examples:
 *
 *     temporal::caused_by(...)
 *     temporal::depends_on(...)
 *     temporal::precedes(...)
 *     temporal::follows(...)
 *
 * Semantic analysis determines whether the relationship is meaningful and
 * whether it is satisfiable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. SPECULATIVE EXECUTION
 * ============================================================================
 *
 * Temporal memory may be used to represent speculative state.
 *
 * The grammar does not implement speculative execution.
 *
 * It may express semantic operations such as:
 *
 *     temporal::fork(...)
 *     temporal::checkpoint(...)
 *     temporal::merge(...)
 *
 * The execution subsystem decides how speculation is realized.
 *
 * There is no fixed number of speculative branches.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. OBSERVATION
 * ============================================================================
 *
 * The canonical lexer already contains:
 *
 *     OBSERVE
 *
 * for the broader language observability construct.
 *
 * This file deliberately does not reinterpret:
 *
 *     OBSERVE
 *
 * as temporal-memory observation.
 *
 * Temporal observation can instead be represented through:
 *
 *     temporal::observe(...)
 *
 * when semantic temporal observation is intended.
 *
 * This avoids coupling generic observability to temporal memory.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser errors concern structural invalidity only.
 *
 * Examples:
 *
 *     missing operation name;
 *     missing '(';
 *     malformed argument list;
 *     malformed named argument;
 *     missing ')';
 *     malformed MTS literal;
 *     missing temporal-scope operand.
 *
 * Semantic diagnostics may include:
 *
 *     INVALID_TEMPORAL_MEMORY_TARGET
 *     INVALID_TEMPORAL_OPERATION
 *     INVALID_TEMPORAL_SELECTOR
 *     INVALID_TEMPORAL_RELATIONSHIP
 *     TEMPORAL_STATE_UNAVAILABLE
 *     TEMPORAL_CAPABILITY_UNAVAILABLE
 *     TEMPORAL_HISTORY_UNAVAILABLE
 *     INVALID_TEMPORAL_FORK
 *     INVALID_TEMPORAL_MERGE
 *     INVALID_TEMPORAL_REWIND
 *     CAUSALITY_VIOLATION
 *     TEMPORAL_LIFETIME_VIOLATION
 *     TEMPORAL_OWNERSHIP_VIOLATION
 *     TEMPORAL_RESOURCE_UNSATISFIED
 *
 * These diagnostics belong to semantic/resource analysis rather than this
 * grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no Rust;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no environment-variable inspection;
 *     - no randomness;
 *     - no target-dependent parsing.
 *
 * Identical token streams therefore have identical parser behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SECURITY
 * ============================================================================
 *
 * Parsing temporal memory MUST NOT:
 *
 *     create a timeline;
 *     access a timeline;
 *     allocate memory;
 *     inspect persistent storage;
 *     access physical memory;
 *     read clocks;
 *     query devices;
 *     execute callbacks;
 *     access a network;
 *     invoke a runtime;
 *     modify history;
 *     perform rollback.
 *
 * All such behavior belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     MAX_TIMELINES
 *     MAX_BRANCHES
 *     MAX_HISTORY
 *     MAX_SNAPSHOTS
 *     MAX_CHECKPOINTS
 *     MAX_EVENTS
 *     MAX_VERSIONS
 *     MAX_TIMESTAMP
 *     MAX_TEMPORAL_DEPTH
 *     MAX_TEMPORAL_STORAGE
 *
 * It MUST also remain free of:
 *
 *     timeline0
 *     timeline1
 *     branch0
 *     branch1
 *     node0
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     qubit0
 *
 * as special grammar constructs.
 *
 * Repetition is represented structurally with:
 *
 *     *
 *     +
 *
 * and resource quantities are represented as ordinary expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. SCALABILITY
 * ============================================================================
 *
 * There is no grammar-level limit on:
 *
 *     - number of temporal operations;
 *     - number of arguments;
 *     - number of temporal scopes;
 *     - number of temporal memory subjects;
 *     - number of snapshots;
 *     - number of checkpoints;
 *     - number of histories;
 *     - number of versions;
 *     - number of branches;
 *     - number of timelines;
 *     - number of temporal relationships;
 *     - number of metadata entries;
 *     - number of distributed participants;
 *     - amount of memory;
 *     - number of processors;
 *     - number of accelerators;
 *     - number of qubits.
 *
 * "Unbounded" means no artificial finite language ceiling.
 *
 * It does not claim infinite physical resources.
 *
 * Actual limits are determined by:
 *
 *     program representation;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     explicitly declared resource requirements;
 *     deployment policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPATIBILITY
 * ============================================================================
 *
 * Existing stable syntax must not be silently reinterpreted.
 *
 * In particular:
 *
 *     MTS<T>
 *
 * remains a type construct.
 *
 *     mts[expression]
 *
 * remains compatibility literal syntax where enabled.
 *
 *     zamani expression
 *
 *     sasa expression
 *
 * remain temporal scope syntax.
 *
 * Open-world temporal operation names remain ordinary names.
 *
 * This file does not reserve new global keywords.
 *
 * Any future promotion of:
 *
 *     timeline
 *     fork
 *     merge
 *     rewind
 *     snapshot
 *     checkpoint
 *
 * into reserved/contextual keywords must be handled through the canonical
 * lexical/specification compatibility process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     zamani state;
 *
 *     sasa state;
 *
 *     zamani {
 *         state;
 *     }
 *
 *     sasa {
 *         state;
 *     }
 *
 *     mts[state]
 *
 *     temporal::snapshot(state)
 *
 *     temporal::checkpoint(state)
 *
 *     temporal::restore(state, version)
 *
 *     temporal::rewind(state, target)
 *
 *     temporal::fork(state, branch)
 *
 *     temporal::merge(left, right)
 *
 *     temporal::observe(state)
 *
 *     temporal::history(state)
 *
 *     temporal::versions(state)
 *
 *     temporal::caused_by(state, event)
 *
 *     temporal::snapshot(
 *         state,
 *         version = requested_version,
 *         policy = temporal::durable
 *     )
 *
 * OPEN-WORLD
 * ----------
 *
 *     future::temporal::operation(state)
 *
 *     vendor::temporal::operation(state)
 *
 *     dialect::temporal::operation(state)
 *
 *     application::history::checkpoint(state)
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     temporal::snapshot(classical_state)
 *
 *     temporal::snapshot(quantum_state)
 *
 *     temporal::snapshot(hardware_state)
 *
 *     temporal::checkpoint(distributed_state)
 *
 *     temporal::restore(model_state)
 *
 *     temporal::fork(speculative_state)
 *
 * NEGATIVE STRUCTURAL TESTS
 * -------------------------
 *
 *     temporal::snapshot(
 *
 *     temporal::snapshot(state
 *
 *     temporal::snapshot(, state)
 *
 *     temporal::snapshot(state,)
 *         [depending on canonical trailing-comma policy]
 *
 *     mts[
 *
 *     mts[state
 *
 *     zamani
 *
 *     sasa
 *
 *     temporal::snapshot(state, = version)
 *
 * BOUNDARY TESTS
 * --------------
 *
 *     deeply nested expressions;
 *     large argument lists;
 *     large qualified names;
 *     large metadata sets;
 *     large temporal operation sequences;
 *     symbolic timestamps;
 *     symbolic versions;
 *     symbolic history identifiers;
 *     symbolic branch identifiers.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Verify that:
 *
 *     1
 *     2
 *     N
 *
 * temporal constructs require no grammar modification as N grows.
 *
 * There must be no special grammar behavior for:
 *
 *     1 timeline;
 *     2 timelines;
 *     fixed timeline counts;
 *     fixed branch counts;
 *     fixed history counts.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * The same token stream must produce the same parse structure independent of:
 *
 *     hardware;
 *     operating system;
 *     runtime state;
 *     resource availability;
 *     network state;
 *     clock state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. FRONTEND / AST INTEGRATION
 * ============================================================================
 *
 * The frontend must map this grammar into the existing domain-neutral AST.
 *
 * Required preservation:
 *
 *     source span;
 *     operation identity;
 *     operand order;
 *     named-argument order;
 *     scope kind;
 *     MTS literal payload;
 *     metadata;
 *     syntactic nesting.
 *
 * The parser must not manufacture backend-specific nodes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. SEMANTIC / RESOURCE INTEGRATION
 * ============================================================================
 *
 * After AST construction:
 *
 *     temporal semantics
 *         ->
 *     memory semantics
 *         ->
 *     lifetime/ownership semantics
 *         ->
 *     resource/capability analysis
 *         ->
 *     persistence/distributed analysis
 *         ->
 *     canonical semantic representation
 *
 * Resource requirements must remain separate from:
 *
 *     preferences;
 *     hints;
 *     implementation decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * Temporal memory syntax does not directly select an IR.
 *
 * The correct path is:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic temporal-memory model
 *       ->
 *     canonical semantic representation
 *       ->
 *     appropriate canonical IR
 *
 * If a temporal operation affects quantum computation:
 *
 *     semantic temporal model
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *
 * No:
 *
 *     TemporalIR
 *     TemporalQuantumIR
 *     MTSIR
 *
 * is introduced merely for this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may use temporal-memory semantics for:
 *
 *     checkpoint planning;
 *     speculative execution;
 *     recovery;
 *     persistence planning;
 *     scheduling constraints;
 *     distributed coordination;
 *     state-version management;
 *     optimization legality.
 *
 * The grammar itself performs none of those operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime consumers may include:
 *
 *     src/runtime/mts.rs
 *     memory runtime;
 *     Sankofa runtime;
 *     persistence runtime;
 *     distributed runtime;
 *     execution runtime.
 *
 * The parser must remain completely independent of those implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. RUST 1.97 / SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This .g4 file contains no Rust implementation.
 *
 * Therefore it contains:
 *
 *     no unsafe;
 *     no FFI;
 *     no raw pointers;
 *     no embedded Rust actions;
 *     no runtime callbacks.
 *
 * The Rust frontend generated/consuming this grammar remains subject to:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one parser grammar identity.
 *
 *     [x] It has one public temporalMemoryConstruct entry point.
 *
 *     [x] It uses the canonical ZamaniLexer.
 *
 *     [x] It reuses canonical Names and Expressions.
 *
 *     [x] It does not redefine identifier syntax.
 *
 *     [x] It does not redefine qualified-name syntax.
 *
 *     [x] It does not redefine expression precedence.
 *
 *     [x] It does not redefine MTS<T>.
 *
 *     [x] It does not redefine generic memory rules.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not introduce TIMELINE/FORK/MERGE/REWIND lexer tokens.
 *
 *     [x] It does not depend on MTSLiteral as a monolithic lexer token.
 *
 *     [x] It supports zamani/sasa temporal scopes.
 *
 *     [x] It supports the documented mts[expression] compatibility form.
 *
 *     [x] It supports open-world temporal operations.
 *
 *     [x] It supports qualified future/vendor/dialect temporal operations.
 *
 *     [x] It supports arbitrary expression arguments.
 *
 *     [x] It supports named arguments.
 *
 *     [x] It supports operation blocks.
 *
 *     [x] It imposes no temporal resource ceilings.
 *
 *     [x] It imposes no hardware ceilings.
 *
 *     [x] It performs no runtime work.
 *
 *     [x] It performs no hardware discovery.
 *
 *     [x] It introduces no temporal IR.
 *
 *     [x] It preserves the quantum::ir boundary.
 *
 *     [x] It preserves POCO-REAF.
 *
 *     [x] It contains no embedded Rust.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It defines downstream AST/semantic/IR integration in advance.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Temporal memory grammar describes temporal-memory SOURCE INTENT.
 *
 * It does not implement:
 *
 *     time;
 *     clocks;
 *     timelines;
 *     history;
 *     persistence;
 *     scheduling;
 *     distributed execution;
 *     quantum execution;
 *     hardware.
 *
 * The final architecture remains:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     TemporalMemory parser component
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic temporal-memory analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------+----------------+
 *          |                |                |
 *          v                v                v
 *       memory          quantum::ir     HDL/hardware
 *          |                |                |
 *          +----------------+----------------+
 *                           |
 *                           v
 *                    optimization/lowering
 *                           |
 *                    routing/scheduling
 *                           |
 *                    resilience/QEC/ZQN
 *                           |
 *                          HAL
 *                           |
 *                   target realization
 *
 * No artificial finite language ceiling is introduced by this file.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */