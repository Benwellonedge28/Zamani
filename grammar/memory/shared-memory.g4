/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/shared-memory.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Role:
 *     Shared-memory specialization of the canonical Zamani memory grammar.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SHARED-MEMORY INTENT.
 *
 * It does not define:
 *
 *   - physical memory;
 *   - addresses;
 *   - caches;
 *   - NUMA topology;
 *   - CPU topology;
 *   - GPU topology;
 *   - accelerator topology;
 *   - allocation algorithms;
 *   - synchronization implementations;
 *   - schedulers;
 *   - runtime behavior;
 *   - hardware discovery;
 *   - hardware selection;
 *   - backend selection;
 *   - optimization;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR.
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     composed parser
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *       Memory                SharedMemory
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *                    AST
 *                     |
 *                     v
 *              semantic analysis
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      ownership  concurrency  resources
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical   quantum    hardware
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *          optimization / scheduling /
 *          lowering / execution
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - shared-memory construct syntax;
 *   - sharing intent;
 *   - unsharing intent;
 *   - shared access intent;
 *   - visibility intent;
 *   - consistency intent;
 *   - synchronization requirements attached to shared memory;
 *   - shared-memory requirements;
 *   - shared-memory constraints;
 *   - shared-memory preferences;
 *   - shared-memory hints;
 *   - shared-memory extension invocation syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - expressions;
 *   - types;
 *   - memory places;
 *   - generic memory operations;
 *   - ownership;
 *   - borrowing;
 *   - lifetimes;
 *   - allocation;
 *   - deallocation;
 *   - distributed memory;
 *   - tasks;
 *   - threads;
 *   - actors;
 *   - channels;
 *   - locks;
 *   - barriers as concurrency primitives;
 *   - scheduling;
 *   - resource discovery;
 *   - physical placement;
 *   - physical addresses;
 *   - hardware topology;
 *   - quantum semantics;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Canonical memory syntax is owned by:
 *
 *     grammar/memory/memory.g4
 *
 * This grammar therefore imports Memory and reuses its canonical:
 *
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryOperation
 *     memoryPolicy
 *     memoryResource
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *
 * where provided by the composed memory grammar.
 *
 * This file MUST NOT redefine memoryPlace or another competing memory-location
 * grammar.
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * General type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * Concurrency syntax remains owned by:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file defines no lexer rules.
 *
 * Shared-memory technology names are intentionally NOT converted into an
 * exhaustive lexer keyword list.
 *
 * Therefore names such as:
 *
 *     shared
 *     coherent
 *     noncoherent
 *     numa
 *     unified
 *     persistent
 *     transactional
 *     accelerator
 *     process
 *     task
 *     distributed
 *     future
 *
 * remain extensible semantic names unless the canonical lexer already reserves
 * a spelling.
 *
 * This preserves the repository's open-world lexical policy.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no machine-size limit.
 *
 * It MUST NOT encode:
 *
 *     MAX_SHARED_MEMORY
 *     MAX_SHARED_OBJECTS
 *     MAX_SHARED_REGIONS
 *     MAX_SHARERS
 *     MAX_THREADS
 *     MAX_PROCESSES
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY_SIZE
 *     MAX_ADDRESSES
 *
 * It MUST NOT encode:
 *
 *     fixed physical addresses;
 *     fixed memory banks;
 *     fixed NUMA nodes;
 *     fixed device identifiers;
 *     fixed topology;
 *     fixed accelerator counts;
 *     fixed process counts;
 *     fixed thread counts.
 *
 * Resource limits are downstream semantic/resource-policy concerns.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * Shared memory means that a program declares that multiple execution
 * contexts may semantically access the same memory place according to the
 * declared access, visibility, consistency, and synchronization requirements.
 *
 * Shared memory does NOT automatically imply:
 *
 *     cache coherence;
 *     atomicity;
 *     mutual exclusion;
 *     sequential consistency;
 *     immediate visibility;
 *     physical shared RAM;
 *     one CPU;
 *     one GPU;
 *     one accelerator;
 *     one process;
 *     one machine.
 *
 * Those properties are independently represented and checked.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `sharedMemoryConstruct` is the stable integration rule consumed by the
 * composed memory parser.
 *
 * ============================================================================
 */

parser grammar SharedMemory;

options {
    tokenVocab = ZamaniLexer;
}

import Memory;

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A shared-memory construct may be:
 *
 *     - a declaration;
 *     - a sharing operation;
 *     - an unsharing operation;
 *     - a shared-memory access operation;
 *     - a semantic requirement;
 *     - an extension operation.
 *
 * ============================================================================
 */

sharedMemoryConstruct
    : sharedMemoryDeclaration
    | sharedMemoryStatement
    | sharedMemoryExpression
    | sharedMemoryExtension
    ;

/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * A declaration expresses that a memory place participates in a shared-memory
 * semantic domain.
 *
 * Example conceptual forms:
 *
 *     shared x;
 *     shared x with read;
 *     shared x requires coherent;
 *
 * The exact declaration binding/type semantics remain downstream.
 * ============================================================================
 */

sharedMemoryDeclaration
    : sharedMemoryKeyword
      sharedMemoryTarget
      sharedMemoryAccessClause?
      sharedMemoryVisibilityClause?
      sharedMemoryConsistencyClause?
      sharedMemorySynchronizationClause?
      sharedMemoryRequirementsClause?
      sharedMemoryConstraintsClause?
      sharedMemoryPreferencesClause?
      sharedMemoryHintsClause?
      sharedMemoryMetadataClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * STATEMENTS
 * ============================================================================
 */

sharedMemoryStatement
    : shareStatement
    | unshareStatement
    | sharedMemoryAccessStatement
    ;

/*
 * ============================================================================
 * SHARE
 * ============================================================================
 *
 * `share` requests establishment of a sharing relationship.
 *
 * It does NOT allocate physical shared memory and does NOT select a backend.
 * ============================================================================
 */

shareStatement
    : shareKeyword
      sharedMemoryTarget
      sharedMemoryAccessClause?
      sharedMemoryVisibilityClause?
      sharedMemoryConsistencyClause?
      sharedMemorySynchronizationClause?
      sharedMemoryRequirementsClause?
      sharedMemoryConstraintsClause?
      sharedMemoryPreferencesClause?
      sharedMemoryHintsClause?
      sharedMemoryMetadataClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * UNSHARE
 * ============================================================================
 *
 * `unshare` requests termination of a sharing relationship.
 *
 * Whether unsharing is legal is determined by semantic analysis.
 * ============================================================================
 */

unshareStatement
    : unshareKeyword
      sharedMemoryTarget
      sharedMemoryRequirementsClause?
      sharedMemoryConstraintsClause?
      sharedMemoryPreferencesClause?
      sharedMemoryHintsClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * SHARED MEMORY ACCESS
 * ============================================================================
 *
 * Access intent is explicitly represented rather than inferred.
 *
 * This avoids conflating:
 *
 *     sharing
 *
 * with:
 *
 *     read permission;
 *     write permission;
 *     synchronization;
 *     visibility;
 *     consistency.
 *
 * ============================================================================
 */

sharedMemoryAccessStatement
    : sharedAccessKeyword
      sharedMemoryTarget
      sharedMemoryAccessClause?
      sharedMemoryVisibilityClause?
      sharedMemoryConsistencyClause?
      sharedMemorySynchronizationClause?
      sharedMemoryRequirementsClause?
      sharedMemoryConstraintsClause?
      sharedMemoryPreferencesClause?
      sharedMemoryHintsClause?
      sharedMemoryMetadataClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * EXPRESSION FORM
 * ============================================================================
 *
 * Expression form is intentionally represented as a semantic operation rather
 * than as a physical memory access.
 *
 * This allows future execution models to lower the same source semantics to:
 *
 *     CPU memory;
 *     accelerator memory;
 *     unified memory;
 *     distributed shared memory;
 *     heterogeneous memory;
 *     future memory systems.
 * ============================================================================
 */

sharedMemoryExpression
    : sharedMemoryOperation
    ;

/*
 * ============================================================================
 * OPERATION
 * ============================================================================
 *
 * Canonical operation form:
 *
 *     shared::operation(...)
 *
 * More deeply qualified forms remain legal:
 *
 *     shared::domain::operation(...)
 *     memory::shared::domain::operation(...)
 *
 * No fixed qualification depth is imposed.
 * ============================================================================
 */

sharedMemoryOperation
    : sharedMemoryQualifiedOperation
    ;

/*
 * ============================================================================
 * QUALIFIED OPERATION
 * ============================================================================
 */

sharedMemoryQualifiedOperation
    : sharedMemoryQualifiedName
      LPAREN sharedMemoryArgumentList? RPAREN
    ;

/*
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * Open-world semantic names.
 *
 * The grammar deliberately does not enumerate every possible shared-memory
 * technology.
 * ============================================================================
 */

sharedMemoryQualifiedName
    : IDENTIFIER
    | IDENTIFIER DOUBLE_COLON sharedMemoryNameTail
    ;

sharedMemoryNameTail
    : IDENTIFIER
    | sharedMemoryNameTail DOUBLE_COLON IDENTIFIER
    ;

/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Arguments are ordinary expressions or named semantic arguments.
 *
 * The expression grammar remains authoritative.
 * ============================================================================
 */

sharedMemoryArgumentList
    : sharedMemoryArgument
      (COMMA sharedMemoryArgument)*
    ;

sharedMemoryArgument
    : expression
    | sharedMemoryNamedArgument
    ;

sharedMemoryNamedArgument
    : IDENTIFIER ASSIGN expression
    ;

/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Do not redefine memoryPlace here.
 *
 * `memoryPlace` belongs to Memory and is imported from the canonical memory
 * foundation.
 * ============================================================================
 */

sharedMemoryTarget
    : memoryPlace
    ;

/*
 * ============================================================================
 * KEYWORD REPRESENTATION
 * ============================================================================
 *
 * These semantic words are represented as identifiers rather than requiring
 * new lexer tokens.
 *
 * This is intentionally compatible with the open-world lexer architecture.
 * ============================================================================
 */

sharedMemoryKeyword
    : sharedMemoryName
    ;

shareKeyword
    : sharedMemoryName
    ;

unshareKeyword
    : sharedMemoryName
    ;

sharedAccessKeyword
    : sharedMemoryName
    ;

/*
 * ============================================================================
 * SEMANTIC NAME
 * ============================================================================
 *
 * The semantic analyzer resolves whether a particular identifier denotes the
 * corresponding shared-memory operation.
 *
 * The parser does not hard-code the universe of technologies.
 * ============================================================================
 */

sharedMemoryName
    : IDENTIFIER
    ;

/*
 * ============================================================================
 * ACCESS INTENT
 * ============================================================================
 *
 * Supported base access intents:
 *
 *     read
 *     write
 *     read_write
 *
 * Additional future access modes can be represented through the open semantic
 * name form.
 * ============================================================================
 */

sharedMemoryAccessClause
    : accessKeyword sharedMemoryAccessModeList
    ;

accessKeyword
    : IDENTIFIER
    ;

sharedMemoryAccessModeList
    : sharedMemoryAccessMode
      (COMMA sharedMemoryAccessMode)*
    ;

sharedMemoryAccessMode
    : sharedMemorySemanticName
    ;

/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Visibility is separate from synchronization.
 *
 * Examples:
 *
 *     visible
 *     eventually_visible
 *     immediately_visible
 *     domain_visible
 *
 * These are semantic names, not physical guarantees.
 * ============================================================================
 */

sharedMemoryVisibilityClause
    : visibilityKeyword sharedMemorySemanticName
    ;

visibilityKeyword
    : IDENTIFIER
    ;

/*
 * ============================================================================
 * CONSISTENCY
 * ============================================================================
 *
 * Examples:
 *
 *     relaxed
 *     acquire_release
 *     sequential
 *     transactional
 *     domain_specific
 *
 * The grammar preserves the semantic name without deciding implementation.
 * ============================================================================
 */

sharedMemoryConsistencyClause
    : consistencyKeyword sharedMemorySemanticName
    ;

consistencyKeyword
    : IDENTIFIER
    ;

/*
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization is referenced by semantic identity.
 *
 * This grammar does NOT define:
 *
 *     mutex;
 *     semaphore;
 *     barrier;
 *     channel;
 *     event;
 *     scheduler.
 *
 * Those remain concurrency/effect-domain responsibilities.
 * ============================================================================
 */

sharedMemorySynchronizationClause
    : synchronizationKeyword sharedMemorySynchronizationSpec
    ;

synchronizationKeyword
    : IDENTIFIER
    ;

sharedMemorySynchronizationSpec
    : sharedMemorySemanticName
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * It is not a physical resource identifier.
 * ============================================================================
 */

sharedMemoryRequirementsClause
    : REQUIRES sharedMemoryRequirementList
    ;

sharedMemoryRequirementList
    : sharedMemoryRequirement
      (COMMA sharedMemoryRequirement)*
    ;

sharedMemoryRequirement
    : expression
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict the legal implementation space.
 *
 * They are distinct from requirements.
 * ============================================================================
 */

sharedMemoryConstraintsClause
    : sharedMemoryConstraintKeyword sharedMemoryConstraintList
    ;

sharedMemoryConstraintKeyword
    : IDENTIFIER
    ;

sharedMemoryConstraintList
    : sharedMemoryConstraint
      (COMMA sharedMemoryConstraint)*
    ;

sharedMemoryConstraint
    : expression
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are non-binding desired properties.
 * ============================================================================
 */

sharedMemoryPreferencesClause
    : sharedMemoryPreferenceKeyword sharedMemoryPreferenceList
    ;

sharedMemoryPreferenceKeyword
    : IDENTIFIER
    ;

sharedMemoryPreferenceList
    : sharedMemoryPreference
      (COMMA sharedMemoryPreference)*
    ;

sharedMemoryPreference
    : expression
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints must never alter program semantics.
 * ============================================================================
 */

sharedMemoryHintsClause
    : sharedMemoryHintKeyword sharedMemoryHintList
    ;

sharedMemoryHintKeyword
    : IDENTIFIER
    ;

sharedMemoryHintList
    : sharedMemoryHint
      (COMMA sharedMemoryHint)*
    ;

sharedMemoryHint
    : expression
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is deliberately represented as key/value information.
 *
 * It is not interpreted by the parser.
 * ============================================================================
 */

sharedMemoryMetadataClause
    : WITH sharedMemoryMetadataList
    ;

sharedMemoryMetadataList
    : sharedMemoryMetadata
      (COMMA sharedMemoryMetadata)*
    ;

sharedMemoryMetadata
    : IDENTIFIER ASSIGN expression
    ;

/*
 * ============================================================================
 * OPEN-WORLD SEMANTIC NAMES
 * ============================================================================
 *
 * This is the principal extensibility boundary.
 *
 * A semantic name may be:
 *
 *     shared
 *     read
 *     write
 *     read_write
 *     coherent
 *     noncoherent
 *     numa
 *     unified
 *     persistent
 *     transactional
 *     accelerator
 *     process
 *     task
 *     distributed
 *     future_domain
 *
 * without requiring a lexer modification.
 *
 * ============================================================================
 */

sharedMemorySemanticName
    : IDENTIFIER
    | sharedMemoryQualifiedName
    ;

/*
 * ============================================================================
 * EXTENSIONS
 * ============================================================================
 *
 * Extensions are intentionally open-world.
 *
 * Examples:
 *
 *     shared::persistent(...)
 *     shared::transactional(...)
 *     shared::accelerator(...)
 *     shared::distributed(...)
 *     memory::shared::future(...)
 *
 * The parser preserves the operation identity and arguments.
 * Semantic registration determines whether the extension is known, supported,
 * deprecated, experimental, or invalid.
 * ============================================================================
 */

sharedMemoryExtension
    : sharedMemoryQualifiedOperation
    ;

/*
 * ============================================================================
 * COMPOSER INTEGRATION CONTRACT
 * ============================================================================
 *
 * The host memory parser should integrate this grammar through:
 *
 *     sharedMemoryConstruct
 *
 * and MUST NOT copy these rules into another grammar.
 *
 * Recommended composition:
 *
 *     memoryConstruct
 *         : genericMemoryConstruct
 *         | sharedMemoryConstruct
 *         | allocationConstruct
 *         | deallocationConstruct
 *         | ownershipConstruct
 *         | borrowingConstruct
 *         | lifetimeConstruct
 *         | distributedMemoryConstruct
 *         | ...
 *         ;
 *
 * The exact host rule remains owned by the memory composition layer.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST node corresponding to this grammar must preserve:
 *
 *     source span
 *     construct kind
 *     target memory place
 *     operation name
 *     access intent
 *     visibility intent
 *     consistency intent
 *     synchronization intent
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     metadata
 *     extension arguments
 *
 * The parser MUST NOT manufacture:
 *
 *     physical address
 *     physical memory ID
 *     device ID
 *     NUMA node
 *     CPU ID
 *     GPU ID
 *     accelerator ID
 *     cache level
 *     topology location
 *
 * ============================================================================
 * SEMANTIC ANALYSIS CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     ownership checking
 *     borrow checking
 *     lifetime checking
 *     alias analysis
 *     race analysis
 *     access compatibility
 *     visibility validity
 *     consistency validity
 *     synchronization validity
 *     capability checking
 *     resource checking
 *     effect checking
 *     target realization
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Shared-memory AST semantics are lowered by the semantic/compiler pipeline
 * into the repository's canonical representation.
 *
 * This grammar MUST NOT import or depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     hardware HAL
 *     runtime
 *
 * Those systems consume semantic representations downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Shared classical memory may carry:
 *
 *     measurement results
 *     control predicates
 *     parameters
 *     orchestration state
 *     hybrid execution state
 *
 * This grammar does not define quantum state semantics.
 *
 * It MUST NOT:
 *
 *     allocate qubits;
 *     select physical qubits;
 *     select a QPU;
 *     define coupling topology;
 *     define QEC;
 *     define ZQN;
 *     define quantum routing.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization may implement the same source semantics using:
 *
 *     coherent memory
 *     non-coherent memory
 *     unified memory
 *     accelerator memory
 *     NUMA memory
 *     distributed shared memory
 *     future memory technologies
 *
 * Hardware capabilities determine whether the requested semantics can be
 * realized.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Shared-memory syntax does not imply that all sharers exist on one machine.
 *
 * A backend may realize the semantics through:
 *
 *     shared physical memory;
 *     distributed shared memory;
 *     replicated state;
 *     remote memory;
 *     migration;
 *     another valid implementation.
 *
 * Distributed placement and consistency mechanisms remain outside this file.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     allocate memory;
 *     access memory;
 *     access hardware;
 *     inspect the operating system;
 *     contact a network;
 *     invoke an allocator;
 *     invoke a runtime;
 *     execute user code.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no runtime actions;
 *     no environment reads;
 *     no hardware discovery;
 *     no generated identifiers.
 *
 * Parsing is deterministic for a fixed token stream and grammar version.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No grammar-level finite resource bound is imposed.
 *
 * The number of:
 *
 *     shared objects;
 *     accesses;
 *     clauses;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     arguments;
 *     qualified-name components;
 *
 * is bounded only by the parser implementation, source representation, and
 * available execution resources, rather than by an artificial language limit.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces the stable public entry point:
 *
 *     sharedMemoryConstruct
 *
 * Existing callers should integrate through that rule rather than through
 * internal rules.
 *
 * Future additions should prefer:
 *
 *     new semantic names;
 *     new qualified operations;
 *     new optional clauses;
 *
 * over changes that make existing valid programs invalid.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The grammar must be tested with:
 *
 * POSITIVE:
 *
 *     shared x;
 *     share x;
 *     unshare x;
 *     shared_access x;
 *     shared x access read;
 *     shared x access write;
 *     shared x access read_write;
 *     shared x visibility eventually_visible;
 *     shared x consistency relaxed;
 *     shared x synchronization fence;
 *     shared x requires coherent;
 *     shared x requires memory::shared;
 *     shared::persistent(x);
 *     memory::shared::transactional(x);
 *     memory::shared::future_domain::operation(x);
 *
 * OPEN-WORLD:
 *
 *     arbitrary future qualified shared-memory names;
 *     arbitrary future operation names;
 *     arbitrary semantic capability names.
 *
 * NEGATIVE:
 *
 *     missing target;
 *     missing operation arguments where required by the selected extension;
 *     malformed qualification;
 *     malformed argument lists;
 *     malformed metadata;
 *     malformed requirement lists;
 *     malformed constraint lists;
 *     malformed preference lists;
 *     malformed hint lists.
 *
 * BOUNDARY:
 *
 *     very long qualified names;
 *     very large argument lists;
 *     very large requirement lists;
 *     very large constraint lists;
 *     very large preference lists;
 *     very large hint lists;
 *     deeply nested semantic qualification.
 *
 * CROSS-DOMAIN:
 *
 *     classical + shared memory;
 *     quantum + shared classical state;
 *     hybrid quantum/classical;
 *     hardware + shared memory;
 *     accelerator + shared memory;
 *     distributed + shared memory;
 *     AI/data + shared memory.
 *
 * DETERMINISM:
 *
 *     identical token streams produce identical parse trees.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It composes with Memory without redefining memoryPlace.
 *   2. It consumes the canonical ZamaniLexer.
 *   3. It introduces no lexer rules.
 *   4. It introduces no machine-size limits.
 *   5. It introduces no physical hardware assumptions.
 *   6. It does not duplicate the concurrency grammar.
 *   7. It does not duplicate allocation/deallocation grammar.
 *   8. It does not create an IR.
 *   9. It does not depend on quantum::ir.
 *  10. It preserves shared-memory intent in the AST.
 *  11. It supports open-world future shared-memory domains.
 *  12. It remains deterministic.
 *  13. It remains parser-only and side-effect free.
 *  14. Positive, negative, boundary and cross-domain tests pass.
 *  15. Rust integration remains compatible with Rust 1.97/1.97.1 and safe Rust.
 *
 * ============================================================================
 */