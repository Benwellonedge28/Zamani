/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/inter-memory.g4
 *
 * Grammar:
 *     InterMemory
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     PROPOSED CANONICAL INTER-MEMORY COMPONENT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only; no unsafe Rust
 *
 * Architectural objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level intent for communication and relationships
 * between semantic memory entities.
 *
 * It supports constructs for:
 *
 *     - identifying source and destination memory entities;
 *     - expressing transfer, exchange, synchronization, and sharing intent;
 *     - expressing request/response relationships;
 *     - expressing memory-to-memory links;
 *     - attaching open-ended policies and metadata;
 *     - expressing requirements, constraints, preferences, and hints;
 *     - referring to distributed, persistent, temporal, quantum, classical,
 *       AI, and other memory domains through ordinary Zamani expressions.
 *
 * This file defines syntax only. It does not implement communication,
 * persistence, replication, synchronization, consensus, or memory access.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     interMemoryConstruct
 *     interMemoryStatement
 *     interMemoryExpression
 *     interMemoryDeclaration
 *     interMemoryOperation
 *     interMemoryEndpoint
 *     interMemorySourceClause
 *     interMemoryDestinationClause
 *     interMemoryRelationClause
 *     interMemoryTransferClause
 *     interMemoryExchangeClause
 *     interMemorySynchronizationClause
 *     interMemorySharingClause
 *     interMemoryRequestClause
 *     interMemoryResponseClause
 *     interMemoryPolicyClause
 *     interMemoryRequirementClause
 *     interMemoryConstraintClause
 *     interMemoryPreferenceClause
 *     interMemoryHintClause
 *     interMemoryMetadataClause
 *     interMemoryArgumentList
 *     interMemoryArgument
 *     interMemoryNamedArgument
 *
 * DOES NOT OWN:
 *
 *     lexical definitions;
 *     identifiers or qualified-name syntax;
 *     expression precedence;
 *     general type syntax;
 *     memory allocation or deallocation;
 *     ownership or borrow checking;
 *     persistence implementation;
 *     distributed topology;
 *     network transport;
 *     synchronization algorithms;
 *     consensus algorithms;
 *     cryptographic identity or verification;
 *     provenance storage;
 *     runtime memory;
 *     canonical IR;
 *     quantum::ir;
 *     hardware realization.
 *
 * ============================================================================
 * AUTHORITY AND COMPOSITION
 * ============================================================================
 *
 * Lexer authority:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Universal names:
 *     grammar/core/names.g4
 *
 * Universal expressions:
 *     grammar/expressions/
 *
 * Memory architecture:
 *     grammar/memory/README.md
 *     grammar/memory/memory.g4
 *
 * Related memory components:
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/persistence.g4
 *     grammar/memory/memory-capabilities.g4
 *     grammar/memory/memory-constraints.g4
 *
 * Sankofa integration:
 *     grammar/memory/sankofa.g4
 *     grammar/memory/consensus.g4
 *     grammar/memory/recall.g4
 *     grammar/memory/temporal.g4
 *
 * Canonical parser composition:
 *     grammar/antlr/ZamaniParser.g4
 *
 * This component reuses universal rules supplied by the composed parser.
 * It does not define identifiers, qualified names, expressions, types,
 * tokens, or a competing root grammar.
 *
 * ============================================================================
 * PORTABILITY AND SCALABILITY
 * ============================================================================
 *
 * There are no grammar-level ceilings on:
 *
 *     memory entities;
 *     endpoints;
 *     transfers;
 *     exchanges;
 *     relationships;
 *     participants;
 *     arguments;
 *     metadata entries;
 *     memory domains;
 *     distributed nodes;
 *     devices;
 *     processes;
 *     quantum resources;
 *     data volume.
 *
 * No fixed device, node, memory, participant, or transfer count is encoded.
 * Repetition is represented structurally. Resource feasibility is checked
 * downstream against actual target capabilities and available resources.
 *
 * "Unbounded" means no artificial finite language ceiling. It does not mean
 * infinite physical resources or unlimited compiler/runtime capacity.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve:
 *
 *     construct kind;
 *     operation identity;
 *     source and destination expressions;
 *     endpoint order;
 *     relation/transfer/exchange/synchronization intent;
 *     positional and named arguments;
 *     policy and metadata expressions;
 *     source spans.
 *
 * Do not introduce an InterMemory-specific AST root or backend-specific node.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether endpoints resolve;
 *     whether endpoint types are compatible;
 *     whether the requested relationship is meaningful;
 *     whether access and ownership rules permit the operation;
 *     whether requirements and constraints are valid;
 *     whether requested capabilities are available;
 *     whether the operation is supported by the selected execution model.
 *
 * Runtime and domain subsystems determine how an accepted intent is realized.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar InterMemory;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/* ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ========================================================================== */

/*
 * Entry point for a construct used as a statement.
 *
 * INTER_MEMORY must be a canonical lexer token. It is intentionally not
 * defined locally in this parser grammar.
 */
interMemoryStatement
    : interMemoryConstruct SEMICOLON
    ;

/*
 * Entry point for expression-position use.
 * Whether a particular inter-memory operation is value-producing is a
 * semantic question.
 */
interMemoryExpression
    : interMemoryConstruct
    ;

/*
 * Shared construct entry point for root-parser composition.
 */
interMemoryConstruct
    : INTER_MEMORY interMemoryOperation
    ;


/* ============================================================================
 * 2. OPERATION
 * ========================================================================== */

/*
 * Operation names are open-world. The grammar provides stable structural
 * categories without enumerating protocols, algorithms, vendors, or policies.
 */
interMemoryOperation
    : interMemoryDeclaration
    | interMemoryTransferClause
    | interMemoryExchangeClause
    | interMemorySynchronizationClause
    | interMemorySharingClause
    | interMemoryRequestClause
    | interMemoryResponseClause
    | interMemoryRelationClause
    | interMemoryExtensionOperation
    ;


/* ============================================================================
 * 3. DECLARATION AND ENDPOINTS
 * ========================================================================== */

interMemoryDeclaration
    : identifier
      interMemorySourceClause
      interMemoryDestinationClause
      interMemoryPolicyClause?
      interMemoryRequirementClause?
      interMemoryConstraintClause?
      interMemoryPreferenceClause?
      interMemoryHintClause?
      interMemoryMetadataClause?
    ;

interMemoryEndpoint
    : expression
    ;

interMemorySourceClause
    : FROM interMemoryEndpoint
    ;

interMemoryDestinationClause
    : TO interMemoryEndpoint
    ;


/* ============================================================================
 * 4. RELATIONSHIPS
 * ========================================================================== */

interMemoryRelationClause
    : RELATES
      interMemoryEndpoint
      TO
      interMemoryEndpoint
      interMemoryArgumentList?
    ;


/* ============================================================================
 * 5. TRANSFER AND EXCHANGE INTENT
 * ========================================================================== */

interMemoryTransferClause
    : TRANSFER
      interMemoryEndpoint
      FROM
      interMemoryEndpoint
      interMemoryArgumentList?
    ;

interMemoryExchangeClause
    : EXCHANGE
      interMemoryEndpoint
      WITH
      interMemoryEndpoint
      interMemoryArgumentList?
    ;


/* ============================================================================
 * 6. SYNCHRONIZATION AND SHARING INTENT
 * ========================================================================== */

interMemorySynchronizationClause
    : SYNCHRONIZE
      interMemoryEndpoint
      WITH
      interMemoryEndpoint
      interMemoryArgumentList?
    ;

interMemorySharingClause
    : SHARE
      interMemoryEndpoint
      WITH
      interMemoryEndpoint
      interMemoryArgumentList?
    ;


/* ============================================================================
 * 7. REQUEST / RESPONSE INTENT
 * ========================================================================== */

interMemoryRequestClause
    : REQUEST
      interMemoryEndpoint
      FROM
      interMemoryEndpoint
      interMemoryArgumentList?
    ;

interMemoryResponseClause
    : RESPOND
      TO
      interMemoryEndpoint
      interMemoryArgumentList?
    ;


/* ============================================================================
 * 8. POLICIES, REQUIREMENTS, CONSTRAINTS, PREFERENCES, AND HINTS
 * ============================================================================
 *
 * Each clause carries an ordinary expression. The grammar does not impose a
 * closed vocabulary of policy names or capability names.
 */

interMemoryPolicyClause
    : WITH interMemoryArgumentList
    ;

interMemoryRequirementClause
    : REQUIRES interMemoryArgumentList
    ;

interMemoryConstraintClause
    : CONSTRAINED_BY interMemoryArgumentList
    ;

interMemoryPreferenceClause
    : PREFERS interMemoryArgumentList
    ;

interMemoryHintClause
    : HINTS interMemoryArgumentList
    ;


/* ============================================================================
 * 9. OPEN-WORLD EXTENSION
 * ============================================================================
 *
 * Extensions use ordinary names and expressions. A new protocol or operation
 * does not require a new reserved keyword merely because it is introduced.
 */

interMemoryExtensionOperation
    : qualifiedName
      interMemoryArgumentList?
    ;


/* ============================================================================
 * 10. ARGUMENTS AND METADATA
 * ============================================================================
 */

interMemoryArgumentList
    : LPAREN
      (interMemoryArgument (COMMA interMemoryArgument)* COMMA?)?
      RPAREN
    ;

interMemoryArgument
    : interMemoryNamedArgument
    | expression
    ;

interMemoryNamedArgument
    : identifier
      ASSIGN
      expression
    ;

interMemoryMetadataClause
    : WITH
      LBRACE
      (interMemoryNamedArgument (COMMA interMemoryNamedArgument)* COMMA?)?
      RBRACE
    ;