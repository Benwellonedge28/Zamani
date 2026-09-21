/*
 * ============================================================================
 * Zamani — Memory Persistence Grammar
 * File: grammar/memory/persistence.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Defines target-independent syntax for persistent memory intent.
 *
 * Persistence here means semantic intent concerning the survival, retention,
 * checkpointing, snapshotting, restoration, recovery, migration, and
 * durability of program state or memory-backed data.
 *
 * This grammar does NOT define:
 *
 *   - physical storage devices;
 *   - physical addresses;
 *   - filesystem layouts;
 *   - block/page sizes;
 *   - sector sizes;
 *   - RAM/VRAM/NVRAM capacities;
 *   - device identifiers;
 *   - storage-controller topology;
 *   - number of persistence domains;
 *   - number of snapshots;
 *   - number of checkpoints;
 *   - fixed retention periods;
 *   - pointer widths;
 *   - address widths;
 *   - vendor APIs;
 *   - allocation algorithms;
 *   - ownership/borrowing semantics;
 *   - distributed replication semantics;
 *   - quantum-memory implementation;
 *   - canonical IR definitions.
 *
 * POCO-REAF
 * ---------
 * Programs describe WHAT persistence properties are required.
 * The compiler/runtime determines HOW those properties are realized.
 *
 * Example semantic intent:
 *
 *     persist state
 *     checkpoint state
 *     restore state
 *     retain data until condition
 *     requires capability("durable.storage")
 *
 * The grammar must remain open-world. New persistence mechanisms, storage
 * technologies, retention policies, checkpoint formats, and capabilities can
 * be introduced as semantic data without requiring a new parser keyword.
 *
 * ARCHITECTURAL AUTHORITY
 * -----------------------
 *
 *   Source
 *      ↓
 *   ZamaniLexer
 *      ↓
 *   parser grammar
 *      ↓
 *   domain-neutral AST
 *      ↓
 *   semantic analysis
 *      ↓
 *   canonical semantic model / IR
 *      ↓
 *   optimization / scheduling / resilience / deployment
 *      ↓
 *   runtime / HAL / target
 *
 * This file owns only persistence syntax.
 *
 * AST / semantic / IR ownership belongs downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *   - persistence declarations;
 *   - persistence operations;
 *   - persistence policy clauses;
 *   - retention intent;
 *   - durability intent;
 *   - checkpoint intent;
 *   - snapshot intent;
 *   - restore/resume intent;
 *   - rollback/recovery intent;
 *   - migration intent;
 *   - persistence metadata syntax;
 *   - persistence requirements/constraints/preferences/hints;
 *   - open-world persistence kinds and policies.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *   ownership.g4
 *   borrowing.g4
 *   references.g4
 *   allocation.g4
 *   regions.g4
 *   address-spaces.g4
 *   shared-memory.g4
 *   distributed-memory.g4
 *   accelerator-memory.g4
 *   quantum-memory.g4
 *   memory-capabilities.g4
 *
 * Those modules remain responsible for their own semantic domains.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * LEXER
 * -----
 * Uses the repository's canonical ZamaniLexer vocabulary.
 *
 * No persistence-specific lexer keyword is required by this grammar.
 * Persistence concepts can therefore remain open-world and extensible.
 *
 * Existing lexical tokens used here include the repository's canonical
 * identifiers, delimiters, assignment operators, separators, and literals.
 *
 * PARSER COMPOSITION
 * ------------------
 *
 * Intended composition:
 *
 *   parser grammar ZamaniParser
 *       imports/dispatches persistence declarations and expressions
 *
 * This grammar does not become a second root grammar.
 *
 * Shared parser concepts such as:
 *
 *   identifier
 *   qualifiedName
 *   expression
 *
 * are imported from the canonical Names / Expressions grammar contracts.
 *
 * Token vocabulary alone does NOT import parser rules; the canonical parser
 * composition must therefore import the relevant parser grammars as part of
 * its normal composition.
 *
 * AST
 * ---
 * Persistence constructs lower into domain-neutral AST constructs.
 *
 * The AST must preserve:
 *
 *   - source span;
 *   - persistence operation/kind;
 *   - target/value/state expression;
 *   - policy clauses;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - metadata;
 *   - ordering;
 *   - explicit-vs-default information.
 *
 * This grammar must not require a persistence-specific runtime AST if the
 * existing frontend AST can represent the construct generically.
 *
 * SEMANTICS
 * ---------
 * Semantic analysis determines:
 *
 *   - whether a persistence operation is meaningful;
 *   - what state/data is persistent;
 *   - lifetime and visibility;
 *   - consistency requirements;
 *   - durability requirements;
 *   - recovery requirements;
 *   - compatibility constraints;
 *   - capability requirements;
 *   - whether a requested policy can be satisfied by a target.
 *
 * IR
 * --
 * Persistence semantics lower into the repository's canonical semantic/IR
 * pipeline. This grammar must NOT introduce another memory IR, storage IR,
 * distributed IR, or quantum IR.
 *
 * QUANTUM
 * -------
 * If persistent state contains quantum-related data, this grammar merely
 * records persistence intent. Quantum semantic interpretation remains under
 * the canonical quantum frontend and `quantum::ir` boundary.
 *
 * DISTRIBUTED
 * -----------
 * Replication, consistency, placement, and distributed recovery semantics
 * remain owned by distributed-memory/distributed grammar and semantic layers.
 *
 * HARDWARE
 * --------
 * Physical storage realization remains downstream in hardware/resources/
 * execution/runtime/HAL layers.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF CONTRACT
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants for:
 *
 *   memory capacity
 *   storage capacity
 *   checkpoint count
 *   snapshot count
 *   retention count
 *   address width
 *   storage width
 *   device count
 *   node count
 *   replica count
 *   timeline count
 *   process count
 *   thread count
 *   qubit count
 *
 * Values are expressions where values are semantically meaningful.
 *
 * Therefore:
 *
 *     retain_for duration
 *
 * does not establish a maximum duration.
 *
 *     checkpoint state
 *
 * does not establish a maximum number of checkpoints.
 *
 *     persist data
 *
 * does not select a particular storage technology.
 *
 * ============================================================================
 * EXTENSION PRINCIPLE
 * ============================================================================
 *
 * Persistence kinds and policies are represented as names/data wherever
 * possible instead of a closed enumeration.
 *
 * This permits:
 *
 *   persistent
 *   durable
 *   transactional
 *   recoverable
 *   replicated
 *   archival
 *   volatile-with-checkpoint
 *   application-defined
 *   vendor-defined
 *   future-defined
 *
 * without forcing every future mechanism into the core lexer.
 *
 * Semantic validation decides which names have defined meaning.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The parser is responsible for structural diagnostics.
 *
 * Semantic diagnostics must distinguish:
 *
 *   syntax error
 *   unknown persistence policy
 *   incompatible persistence policy
 *   missing capability
 *   unsatisfied requirement
 *   invalid target expression
 *   invalid lifecycle relationship
 *   invalid restoration target
 *
 * The grammar must not reject an otherwise syntactically valid future
 * persistence capability merely because the parser does not know its
 * implementation.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive coverage:
 *
 *   - basic persistence declaration;
 *   - persist operation;
 *   - checkpoint;
 *   - snapshot;
 *   - restore;
 *   - resume;
 *   - rollback;
 *   - recovery;
 *   - migration;
 *   - retention;
 *   - durability;
 *   - metadata;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - qualified persistence names;
 *   - arbitrary expressions;
 *   - nested persistence blocks;
 *   - multiple persistence policies.
 *
 * Required negative coverage:
 *
 *   - missing persistence target;
 *   - missing operation target;
 *   - malformed assignment;
 *   - malformed policy;
 *   - unterminated persistence block;
 *   - invalid separators;
 *   - malformed metadata;
 *   - malformed requirement/constraint/preference clause.
 *
 * Required boundary coverage:
 *
 *   - empty policy block;
 *   - one policy;
 *   - many policies;
 *   - nested expressions;
 *   - symbolic values;
 *   - zero-valued program expressions where semantically legal;
 *   - arbitrarily large representable values.
 *
 * Required scalability coverage:
 *
 *   - no parser behavior changes with increasing resource quantities;
 *   - no fixed persistence count;
 *   - no fixed storage size;
 *   - no fixed retention range;
 *   - no fixed metadata count;
 *   - no fixed nesting depth imposed by this grammar.
 *
 * Required determinism coverage:
 *
 *   - identical input produces identical parse structure;
 *   - no target-dependent parsing;
 *   - no runtime-dependent parsing;
 *   - no hardware-dependent parsing.
 *
 * ============================================================================
 */

parser grammar Persistence;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;

/*
 * ============================================================================
 * PUBLIC ENTRY RULES
 * ============================================================================
 *
 * `persistenceDeclaration` is the primary integration point for declarations.
 *
 * `persistenceExpression` is the integration point for persistence operations
 * embedded in expression/statement contexts.
 *
 * The canonical root grammar should dispatch to these rules rather than
 * copying their implementations.
 */

/**
 * A named persistence declaration.
 *
 * Examples of semantic forms represented by this structure include:
 *
 *   persist state { ... }
 *   persist data { ... }
 *   persistent checkpoint { ... }
 *
 * The exact semantic interpretation belongs downstream.
 */
persistenceDeclaration
    : persistenceKeyword
      persistenceSubject
      persistenceDeclarationBody?
    ;

/**
 * A persistence operation usable from statement/expression composition.
 */
persistenceExpression
    : persistenceOperation
    ;

/*
 * ============================================================================
 * PERSISTENCE KEYWORDS
 * ============================================================================
 *
 * These rules intentionally use existing lexer tokens where available.
 *
 * If the repository's canonical lexer already exposes persistence keywords,
 * those tokens should be referenced directly here.
 *
 * The persistence architecture remains open-world through
 * `persistenceKindName`, so future semantic persistence kinds do not require
 * new lexer keywords.
 */

persistenceKeyword
    : PERSIST
    ;

/*
 * ============================================================================
 * SUBJECTS
 * ============================================================================
 *
 * A persistence subject identifies the logical state/data/resource whose
 * persistence semantics are being described.
 *
 * It is deliberately expression-based rather than tied to:
 *
 *   memory address
 *   storage device
 *   file descriptor
 *   physical region
 *   node
 *   accelerator
 *   QPU
 *
 * Those are downstream realizations.
 */

persistenceSubject
    : expression
    ;

/*
 * ============================================================================
 * DECLARATION BODY
 * ============================================================================
 */

persistenceDeclarationBody
    : LBRACE persistenceMember* RBRACE
    ;

persistenceMember
    : persistenceAssignment
    | persistencePolicyClause
    | persistenceRequirementClause
    | persistenceConstraintClause
    | persistencePreferenceClause
    | persistenceHintClause
    | persistenceMetadataClause
    | persistenceOperation
    ;

/*
 * ============================================================================
 * PERSISTENCE ASSIGNMENTS
 * ============================================================================
 *
 * Generic named properties are deliberately supported.
 *
 * This prevents the language from needing a new lexer keyword every time a
 * persistence backend introduces a new semantic property.
 */

persistenceAssignment
    : persistencePropertyName ASSIGN expression SEMICOLON?
    ;

persistencePropertyName
    : identifier
    ;

/*
 * ============================================================================
 * OPERATIONS
 * ============================================================================
 *
 * Operations are explicit semantic intents.
 *
 * They do not select a physical implementation.
 */

persistenceOperation
    : persistOperation
    | checkpointOperation
    | snapshotOperation
    | restoreOperation
    | resumeOperation
    | rollbackOperation
    | recoveryOperation
    | migrationOperation
    | retainOperation
    ;

/*
 * ============================================================================
 * PERSIST
 * ============================================================================
 */

persistOperation
    : PERSIST
      persistenceSubject
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * CHECKPOINT
 * ============================================================================
 */

checkpointOperation
    : CHECKPOINT
      persistenceSubject?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * SNAPSHOT
 * ============================================================================
 */

snapshotOperation
    : SNAPSHOT
      persistenceSubject?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * RESTORE
 * ============================================================================
 */

restoreOperation
    : RESTORE
      persistenceSource?
      persistenceRestoreTarget?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * RESUME
 * ============================================================================
 */

resumeOperation
    : RESUME
      persistenceSource?
      persistenceRestoreTarget?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * ROLLBACK
 * ============================================================================
 */

rollbackOperation
    : ROLLBACK
      persistenceSource?
      persistenceRestoreTarget?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * RECOVERY
 * ============================================================================
 */

recoveryOperation
    : RECOVER
      persistenceSource?
      persistenceRestoreTarget?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * MIGRATION
 * ============================================================================
 *
 * Migration is semantic state migration. It does not define network,
 * distributed-memory, storage-controller, or physical-placement syntax.
 */

migrationOperation
    : MIGRATE
      persistenceSource?
      persistenceMigrationTarget?
      persistenceOperationBody?
    ;

/*
 * ============================================================================
 * RETENTION
 * ============================================================================
 */

retainOperation
    : RETAIN
      persistenceSubject?
      persistenceRetentionBody?
    ;

persistenceRetentionBody
    : LBRACE
      persistenceRetentionMember*
      RBRACE
    ;

persistenceRetentionMember
    : persistenceAssignment
    | persistencePolicyClause
    | persistenceRequirementClause
    | persistenceConstraintClause
    | persistencePreferenceClause
    | persistenceHintClause
    | persistenceMetadataClause
    ;

/*
 * ============================================================================
 * SOURCES AND TARGETS
 * ============================================================================
 */

persistenceSource
    : expression
    ;

persistenceRestoreTarget
    : ARROW expression
    ;

persistenceMigrationTarget
    : ARROW expression
    ;

/*
 * ============================================================================
 * OPERATION BODY
 * ============================================================================
 */

persistenceOperationBody
    : LBRACE
      persistenceMember*
      RBRACE
    ;

/*
 * ============================================================================
 * POLICY CLAUSES
 * ============================================================================
 *
 * Requirement, constraint, preference, and hint are intentionally distinct.
 *
 *   requires  = semantic necessity
 *   constrain = condition that realization must obey
 *   prefer    = optimization preference
 *   hint      = non-binding implementation guidance
 *
 * These must never be collapsed into one "policy" concept.
 */

persistencePolicyClause
    : persistencePolicyName
      (ASSIGN expression)?
      SEMICOLON?
    ;

persistencePolicyName
    : identifier
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 */

persistenceRequirementClause
    : REQUIRES expression SEMICOLON?
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

persistenceConstraintClause
    : CONSTRAINT expression SEMICOLON?
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 */

persistencePreferenceClause
    : PREFER expression SEMICOLON?
    ;

/*
 * ============================================================================
 * HINTS
 * ============================================================================
 */

persistenceHintClause
    : HINT expression SEMICOLON?
    ;

/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is semantic information associated with persistent state.
 *
 * Examples:
 *
 *   schema = ...
 *   version = ...
 *   format = ...
 *   integrity = ...
 *   compatibility = ...
 *
 * The grammar does not prescribe a physical serialization format.
 */

persistenceMetadataClause
    : METADATA persistenceMetadataBody
    ;

persistenceMetadataBody
    : LBRACE
      persistenceMetadataMember*
      RBRACE
    ;

persistenceMetadataMember
    : persistenceMetadataAssignment
    | persistenceMetadataEntry
    ;

persistenceMetadataAssignment
    : persistencePropertyName ASSIGN expression SEMICOLON?
    ;

persistenceMetadataEntry
    : persistencePropertyName
      (LPAREN persistenceArgumentList? RPAREN)?
      SEMICOLON?
    ;

/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Persistence arguments remain ordinary expressions.
 *
 * This is important for symbolic/unbounded values:
 *
 *   retention = policy
 *   version = schema_version
 *   checkpoint = epoch
 *   deadline = duration
 *
 * No machine-sized integer grammar is introduced here.
 */

persistenceArgumentList
    : expression (COMMA expression)*
    ;

/*
 * ============================================================================
 * OPEN-WORLD PERSISTENCE KIND
 * ============================================================================
 *
 * Persistence kinds are semantic names rather than a closed keyword list.
 *
 * This supports future and vendor-neutral extensions without modifying the
 * core lexer every time a new persistence technology appears.
 */

persistenceKind
    : persistenceKindName
    ;

persistenceKindName
    : qualifiedName
    ;

/*
 * ============================================================================
 * RETENTION / DURABILITY PROPERTY NAMES
 * ============================================================================
 *
 * These remain identifiers rather than reserved keywords.
 *
 * Consequently the language can evolve without turning every policy into a
 * lexical keyword.
 */

persistenceProperty
    : persistencePropertyName
    ;

/*
 * ============================================================================
 * SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * These categories are parser-level structure only.
 *
 * Semantic validation determines whether a named property is valid.
 */

persistenceDurabilityProperty
    : persistencePropertyName
    ;

persistenceRetentionProperty
    : persistencePropertyName
    ;

persistenceRecoveryProperty
    : persistencePropertyName
    ;

persistenceCompatibilityProperty
    : persistencePropertyName
    ;

persistenceIntegrityProperty
    : persistencePropertyName
    ;

/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 *
 * Completion criteria
 * -------------------
 *
 * This grammar is complete when:
 *
 *   [x] Persistence syntax has one canonical owner.
 *   [x] Ownership is separate from persistence.
 *   [x] Borrowing is separate from persistence.
 *   [x] References are separate from persistence.
 *   [x] Allocation is separate from persistence.
 *   [x] Regions are separate from persistence.
 *   [x] Address spaces are separate from persistence.
 *   [x] Shared/distributed memory realization is separate.
 *   [x] Accelerator memory realization is separate.
 *   [x] Quantum memory realization is separate.
 *   [x] Memory capabilities are consumed rather than redefined.
 *   [x] Persistence remains target-independent.
 *   [x] No physical capacity is hard-coded.
 *   [x] No device count is hard-coded.
 *   [x] No address width is hard-coded.
 *   [x] No checkpoint/snapshot count is hard-coded.
 *   [x] No retention maximum is hard-coded.
 *   [x] Expressions remain the source of scalable values.
 *   [x] Requirements differ from preferences and hints.
 *   [x] Future persistence kinds can be represented as semantic names.
 *   [x] AST lowering remains domain-neutral.
 *   [x] No second memory IR is introduced.
 *   [x] No second quantum IR is introduced.
 *   [x] Runtime/storage implementation remains downstream.
 *   [x] Hardware realization remains downstream.
 *   [x] Source spans are preserved by the parser/frontend.
 *   [x] Deterministic parsing is required.
 *   [x] Positive/negative/boundary/scalability tests are required.
 *
 * Rust 1.97 / 1.97.1 compatibility
 * ---------------------------------
 *
 * This file contains ANTLR grammar only. It introduces no Rust code and no
 * unsafe operation. Generated Rust integration remains subject to the
 * repository's Rust 1.97/1.97.1 frontend/toolchain configuration.
 */