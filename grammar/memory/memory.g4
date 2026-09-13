/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/memory.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production memory-domain foundation.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the DOMAIN-NEUTRAL MEMORY SYNTAX FOUNDATION for Zamani.
 *
 * It establishes the common source-level vocabulary required by:
 *
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - allocation;
 *     - deallocation;
 *     - shared memory;
 *     - distributed memory;
 *     - memory constraints;
 *     - resource-aware memory semantics;
 *     - future memory models.
 *
 * This file is intentionally a foundation rather than a second semantic
 * memory system.
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
 *     Core/domain parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Types.g4                       Memory.g4
 *          |                             |
 *          |                 +-----------+-----------+
 *          |                 |           |           |
 *          |                 v           v           v
 *          |             ownership   borrowing   allocation
 *          |                 |           |           |
 *          |                 +-----------+-----------+
 *          |                             |
 *          +-----------------------------+
 *                        |
 *                        v
 *                       AST
 *                        |
 *                        v
 *                structural validation
 *                        |
 *                        v
 *                semantic analysis
 *                        |
 *              +---------+----------+
 *              |                    |
 *              v                    v
 *       ownership model       resource model
 *              |                    |
 *              +---------+----------+
 *                        |
 *                        v
 *                 canonical semantic IR
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical      quantum        hardware
 *         IR            IR            IR
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *              optimization/routing/
 *                 scheduling/lowering
 *                        |
 *                        v
 *                    runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - memory-domain syntax foundations;
 *   - memory-domain qualified names;
 *   - memory-place syntax;
 *   - source-level ownership mode markers;
 *   - source-level borrow mode markers;
 *   - source-level lifetime names;
 *   - memory-region references;
 *   - memory-space references;
 *   - memory-resource requirements;
 *   - memory constraints;
 *   - memory preferences;
 *   - memory hints;
 *   - memory operation invocation structure;
 *   - memory operation argument structure;
 *   - memory policy structure;
 *   - memory-domain extension points;
 *   - stable common syntax consumed by specialized memory grammars.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - lexical tokens;
 *   - identifier spelling;
 *   - Unicode normalization;
 *   - general expressions;
 *   - general statements;
 *   - general declarations;
 *   - complete type semantics;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime inference;
 *   - lifetime validity;
 *   - alias analysis;
 *   - memory allocation;
 *   - physical addresses;
 *   - memory layout;
 *   - allocator implementation;
 *   - garbage collection;
 *   - garbage collector policy;
 *   - NUMA discovery;
 *   - physical memory discovery;
 *   - cache discovery;
 *   - device-memory discovery;
 *   - distributed placement;
 *   - DMA implementation;
 *   - quantum resource allocation;
 *   - qubit allocation;
 *   - quantum routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - hardware selection;
 *   - backend selection;
 *   - runtime execution;
 *   - machine-specific limits;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - hardware IR.
 *
 * ============================================================================
 * CANONICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Memory syntax is source syntax only.
 *
 * The parser MUST preserve source intent without deciding how memory is
 * physically implemented.
 *
 * For example:
 *
 *     memory::shared(x)
 *
 * does NOT mean:
 *
 *     heap allocation
 *     shared RAM
 *     cache-coherent memory
 *     NUMA memory
 *     GPU memory
 *     unified memory
 *     distributed shared memory
 *
 * Semantic analysis determines the meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar MUST remain valid from the smallest supported execution
 * environment to arbitrarily larger environments, subject only to:
 *
 *     - available resources;
 *     - explicit compiler resource policies;
 *     - explicit runtime policies;
 *     - semantic validity;
 *     - target capabilities.
 *
 * This grammar therefore contains NO:
 *
 *     MAX_MEMORY
 *     MAX_ALLOCATION
 *     MAX_REGION_COUNT
 *     MAX_REFERENCE_COUNT
 *     MAX_BORROW_COUNT
 *     MAX_LIFETIME_COUNT
 *     MAX_ADDRESS_WIDTH
 *     MAX_POINTER_WIDTH
 *     MAX_HEAP_SIZE
 *     MAX_STACK_SIZE
 *     MAX_SHARED_MEMORY
 *     MAX_DISTRIBUTED_MEMORY
 *     MAX_MEMORY_SPACES
 *     MAX_MEMORY_OPERATIONS
 *
 * Nor does it encode:
 *
 *     address 0
 *     address 1
 *     heap 0
 *     memory bank 0
 *     NUMA node 0
 *     device memory 0
 *     GPU memory 0
 *
 * Machine/resource limitations belong to explicit downstream policies.
 *
 * ============================================================================
 * OPEN-WORLD MEMORY MODEL
 * ============================================================================
 *
 * Memory is treated as an extensible semantic domain.
 *
 * The grammar must not enumerate every future memory technology.
 *
 * Examples that may be represented without changing the foundational syntax:
 *
 *     memory::local
 *     memory::shared
 *     memory::distributed
 *     memory::persistent
 *     memory::device
 *     memory::unified
 *     memory::remote
 *     memory::nonvolatile
 *     memory::future_domain
 *
 * These names are syntactic identifiers.
 *
 * Their semantics are resolved downstream.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT declare lexer rules.
 *
 * Existing lexical tokens intentionally reused include:
 *
 *     IDENTIFIER
 *     LINEAR
 *     AFFINE
 *     MUT
 *     LET
 *     CONST
 *     VAR
 *     VAL
 *     NEW
 *     EXTERN
 *     AMPERSAND
 *     STAR
 *     APOSTROPHE
 *     AT
 *     ASSIGN
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *     PLUS
 *     MINUS
 *     DOT
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * The foundational memory grammar deliberately does not require adding
 * temporary memory-specific keywords to the lexer.
 *
 * Memory-domain operations use the open qualified-name form:
 *
 *     memory::operation(...)
 *
 * This prevents the lexical layer from becoming a closed list of memory
 * technologies.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Types.g4 remains the canonical owner of:
 *
 *     typeExpression
 *     referenceType
 *     lifetimeAnnotation
 *     pointerType
 *     generic types
 *     dependent types
 *
 * In particular, this grammar MUST NOT redefine:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     *T
 *
 * as competing type productions.
 *
 * This file uses `typeExpression` when a memory construct needs a type.
 *
 * Ownership and borrow semantics are interpreted downstream.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Memory grammar rules therefore do not recreate arithmetic, calls, indexing,
 * member access, conditional expressions, or other general expression syntax.
 *
 * A host parser should integrate:
 *
 *     memoryStatement
 *
 * before generic expression statements where necessary.
 *
 * Semantic analysis distinguishes:
 *
 *     memory::operation(...)
 *
 * from ordinary function calls.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should lower into the existing canonical AST architecture.
 *
 * Memory grammar nodes MUST retain:
 *
 *     - source span;
 *     - operation/namespace name;
 *     - ownership mode;
 *     - mutability;
 *     - lifetime name;
 *     - memory-space name;
 *     - resource expressions;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - type expressions;
 *     - source expressions;
 *     - arguments;
 *     - extension metadata.
 *
 * The grammar MUST NOT require a new physical-memory AST merely to represent:
 *
 *     heap
 *     stack
 *     address
 *     cache
 *     NUMA node
 *     device memory
 *
 * unless semantic analysis explicitly determines that such a concept has
 * source-level meaning.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer is responsible for determining:
 *
 *   - ownership validity;
 *   - move validity;
 *   - copy validity;
 *   - borrow validity;
 *   - mutable-aliasing validity;
 *   - lifetime validity;
 *   - region validity;
 *   - allocation legality;
 *   - deallocation legality;
 *   - memory-space compatibility;
 *   - shared-memory synchronization requirements;
 *   - distributed-memory consistency;
 *   - resource availability;
 *   - capability satisfaction;
 *   - target-specific realization.
 *
 * Syntax alone MUST NOT establish any of those facts.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * The grammar distinguishes:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     memory space
 *     memory resource
 *
 * These must not collapse into one concept.
 *
 * Requirement:
 *
 *     Must be satisfied.
 *
 * Constraint:
 *
 *     Restricts legal implementation choices.
 *
 * Preference:
 *
 *     Describes a desired implementation property.
 *
 * Hint:
 *
 *     Provides optional implementation guidance.
 *
 * Memory space:
 *
 *     Names a semantic memory domain.
 *
 * Memory resource:
 *
 *     Represents a quantity/property that downstream resource analysis may
 *     evaluate.
 *
 * ============================================================================
 * MEMORY PLACE MODEL
 * ============================================================================
 *
 * A memory place is a source-level location expression.
 *
 * It may represent:
 *
 *     variable
 *     field
 *     indexed element
 *     qualified location
 *     dereferenced location
 *     future memory-domain location
 *
 * The grammar deliberately avoids physical addresses.
 *
 * ============================================================================
 * LIFETIME MODEL
 * ============================================================================
 *
 * Lifetime syntax is represented as:
 *
 *     'a
 *     'region
 *     'scope
 *
 * The grammar preserves the name only.
 *
 * It does NOT infer:
 *
 *     start point
 *     end point
 *     region nesting
 *     ownership relationship
 *     lifetime outlives relationship
 *
 * Those are semantic-analysis responsibilities.
 *
 * ============================================================================
 * OWNERSHIP MODEL
 * ============================================================================
 *
 * Source-level ownership modes supported by the foundation are:
 *
 *     linear
 *     affine
 *
 * They are intentionally semantic qualifiers rather than allocator commands.
 *
 * `linear` means, at the language-semantic level, that the associated value
 * participates in linear-use constraints.
 *
 * `affine` means the associated value participates in affine-use constraints.
 *
 * The grammar does not decide:
 *
 *     move implementation
 *     reference counting
 *     tracing GC
 *     stack allocation
 *     heap allocation
 *     register allocation
 *     physical storage
 *
 * ============================================================================
 * BORROW MODEL
 * ============================================================================
 *
 * Borrow syntax uses:
 *
 *     &
 *     &mut
 *
 * together with an existing source-level place.
 *
 * This grammar represents the syntactic operation only.
 *
 * It does not perform:
 *
 *     borrow checking
 *     alias checking
 *     data-race checking
 *     lifetime checking
 *
 * ============================================================================
 * ALLOCATION MODEL
 * ============================================================================
 *
 * Allocation syntax is represented through memory-domain operations and the
 * existing `new` token.
 *
 * The foundational grammar supports:
 *
 *     memory::allocate(...)
 *     memory::allocate_in(...)
 *     memory::reserve(...)
 *     memory::grow(...)
 *
 * without assigning them a physical allocator.
 *
 * `new` is retained as an existing Zamani lexical construct.
 *
 * ============================================================================
 * DEALLOCATION MODEL
 * ============================================================================
 *
 * Deallocation is intentionally represented through the extensible memory
 * namespace:
 *
 *     memory::deallocate(...)
 *     memory::release(...)
 *
 * This avoids introducing a permanently reserved global keyword merely for
 * one implementation strategy.
 *
 * Semantic analysis determines whether explicit release is legal.
 *
 * ============================================================================
 * SHARED MEMORY
 * ============================================================================
 *
 * Shared-memory operations can use:
 *
 *     memory::shared(...)
 *     memory::share(...)
 *     memory::unshare(...)
 *
 * The grammar does not assume:
 *
 *     cache coherence
 *     shared RAM
 *     a particular synchronization primitive
 *     a particular CPU
 *     a particular accelerator.
 *
 * ============================================================================
 * DISTRIBUTED MEMORY
 * ============================================================================
 *
 * Distributed-memory syntax can use:
 *
 *     memory::distributed(...)
 *     memory::remote(...)
 *     memory::replicate(...)
 *     memory::migrate(...)
 *
 * Placement, node selection, network routing and consistency are downstream.
 *
 * ============================================================================
 * EXTENSION MODEL
 * ============================================================================
 *
 * Future memory systems can use:
 *
 *     memory::future_domain::operation(...)
 *
 * without requiring changes to the foundational memory grammar.
 *
 * There is no fixed qualification depth.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a memory operation MUST NEVER:
 *
 *     allocate memory;
 *     deallocate memory;
 *     open files;
 *     access operating-system memory;
 *     inspect hardware;
 *     access device memory;
 *     contact a network;
 *     execute a runtime operation;
 *     invoke an allocator;
 *     invoke a backend.
 *
 * The parser produces syntax only.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no semantic predicates;
 *     no embedded Rust;
 *     no target-specific actions;
 *     no runtime calls;
 *     no generated identifiers;
 *     no hardware discovery;
 *     no environment access.
 *
 * Equivalent source produces deterministic parse structure.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Any generated Rust parser/frontend integration MUST compile with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and:
 *
 *     #![forbid(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * No unsafe implementation is required by this grammar.
 *
 * ============================================================================
 * SPECIALIZED FILE INTEGRATION
 * ============================================================================
 *
 * Future specialized grammar files should import this grammar:
 *
 *     ownership.g4
 *     borrowing.g4
 *     lifetimes.g4
 *     allocation.g4
 *     deallocation.g4
 *     shared-memory.g4
 *     distributed-memory.g4
 *     memory-constraints.g4
 *
 * They must NOT redefine the foundational rules below.
 *
 * Expected dependency direction:
 *
 *     Memory.g4
 *          |
 *          +--> Ownership.g4
 *          +--> Borrowing.g4
 *          +--> Lifetimes.g4
 *          +--> Allocation.g4
 *          +--> Deallocation.g4
 *          +--> SharedMemory.g4
 *          +--> DistributedMemory.g4
 *          +--> MemoryConstraints.g4
 *
 * Domain composition then integrates those specialized grammars into the
 * statement/declaration compilation boundary.
 *
 * The dependency direction MUST NOT be reversed:
 *
 *     Memory.g4 -> semantic analysis
 *     Memory.g4 -> runtime
 *     Memory.g4 -> hardware
 *
 * ============================================================================
 */

parser grammar Memory;

options {
    tokenVocab = ZamaniLexer;
}

import Types;


/* ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ========================================================================== */

/**
 * Canonical memory-domain syntactic entry point.
 *
 * This rule is intentionally broad enough to allow the specialized memory
 * grammars to consume the foundational constructs without modifying this file.
 */
memoryConstruct
    : memoryOwnershipQualifier
    | memoryBorrow
    | memoryLifetime
    | memoryPlace
    | memorySpace
    | memoryRequirement
    | memoryConstraint
    | memoryPreference
    | memoryHint
    | memoryPolicy
    | memoryOperation
    | memoryAllocation
    | memoryDeallocation
    | memorySharing
    | memoryDistribution
    ;


/**
 * Memory-domain statement form.
 *
 * This is the integration point for statement composition.
 */
memoryStatement
    : memoryOperationStatement
    | memoryAllocationStatement
    | memoryDeallocationStatement
    | memorySharingStatement
    | memoryDistributionStatement
    ;


/* ============================================================================
 * 2. NAMES AND QUALIFIED MEMORY OPERATIONS
 * ========================================================================== */

/**
 * A memory-domain name.
 *
 * No finite qualification depth is encoded.
 *
 * Examples:
 *
 *     memory
 *     shared
 *     distributed
 *     future_domain
 */
memoryName
    : identifier
    ;


/**
 * Qualified memory-domain name.
 *
 * Examples:
 *
 *     memory::allocate
 *     memory::shared
 *     memory::distributed::migrate
 *     memory::future_domain::operation
 *
 * Qualification depth is unbounded by language semantics.
 */
memoryQualifiedName
    : memoryName
      (DOUBLE_COLON memoryName)*
    ;


/**
 * Explicit memory namespace.
 *
 * The parser intentionally does not reserve `memory` as a lexer keyword.
 *
 * Semantic validation determines whether the first component denotes the
 * canonical memory namespace.
 */
memoryNamespaceName
    : memoryQualifiedName
    ;


/* ============================================================================
 * 3. OWNERSHIP
 * ========================================================================== */

/**
 * Source-level ownership qualifier.
 *
 * This is syntax only.
 *
 * Semantic ownership checking occurs downstream.
 */
memoryOwnershipQualifier
    : LINEAR
    | AFFINE
    ;


/**
 * Ownership mode with optional mutability.
 *
 * Examples:
 *
 *     linear
 *     affine
 *     linear mut
 *     affine mut
 */
memoryOwnershipMode
    : memoryOwnershipQualifier MUT?
    ;


/**
 * Ownership-qualified binding.
 *
 * Examples:
 *
 *     linear let value: T = expression;
 *     affine let value: T = expression;
 *
 * The initializer expression is intentionally represented through a memory
 * operation payload rather than duplicated general expression grammar.
 *
 * The host declaration grammar may use `memoryOwnershipMode` when composing
 * ordinary declarations.
 */
memoryOwnedBinding
    : memoryOwnershipMode
      LET
      identifier
      memoryTypeAnnotation?
      ASSIGN
      memoryValue
      SEMICOLON
    ;


/**
 * Optional type annotation for a memory-owned binding.
 */
memoryTypeAnnotation
    : COLON typeExpression
    ;


/* ============================================================================
 * 4. BORROWING
 * ========================================================================== */

/**
 * Immutable borrow.
 *
 * Example:
 *
 *     &value
 *
 * This rule represents source-level borrow syntax.
 *
 * It does not perform borrow checking.
 */
memoryBorrow
    : AMPERSAND
      memoryPlace
    | AMPERSAND
      lifetimeAnnotation
      memoryPlace
    ;


/**
 * Mutable borrow.
 *
 * Examples:
 *
 *     &mut value
 *     &'a mut value
 */
memoryMutableBorrow
    : AMPERSAND
      lifetimeAnnotation?
      MUT
      memoryPlace
    ;


/**
 * Unified borrow expression.
 */
memoryBorrowExpression
    : memoryBorrow
    | memoryMutableBorrow
    ;


/* ============================================================================
 * 5. LIFETIMES
 * ========================================================================== */

/**
 * Source-level lifetime.
 *
 * Examples:
 *
 *     'a
 *     'scope
 *     'region
 *
 * The semantic layer determines the lifetime's meaning.
 */
memoryLifetime
    : APOSTROPHE
      identifier
    ;


/**
 * Optional lifetime attached to a memory construct.
 */
memoryLifetimeReference
    : memoryLifetime
    ;


/**
 * Lifetime relation.
 *
 * This intentionally preserves relation syntax without deciding its semantic
 * interpretation.
 *
 * Examples:
 *
 *     'a : 'b
 *     'a : 'b, 'c
 */
memoryLifetimeRelation
    : memoryLifetime
      COLON
      memoryLifetime
      (COMMA memoryLifetime)*
    ;


/* ============================================================================
 * 6. MEMORY PLACES
 * ========================================================================== */

/**
 * A memory place is a source-level location.
 *
 * The foundational grammar intentionally keeps this structural rather than
 * introducing physical addresses.
 */
memoryPlace
    : memoryPlaceRoot
      memoryPlaceProjection*
    ;


/**
 * Root of a memory place.
 */
memoryPlaceRoot
    : identifier
    | SELF
    | SUPER
    ;


/**
 * Memory-place projection.
 *
 * This foundation deliberately supports:
 *
 *     .field
 *     [index]
 *
 * without reimplementing the complete expression grammar.
 */
memoryPlaceProjection
    : DOT identifier
    | LBRACKET memoryIndex RBRACKET
    ;


/**
 * Index expression foundation.
 *
 * A memory index can be:
 *
 *     identifier
 *     integer literal
 *     qualified name
 *
 * More general expressions remain owned by expressions.g4.
 */
memoryIndex
    : INTEGER
    | identifier
    | memoryQualifiedName
    ;


/* ============================================================================
 * 7. MEMORY TYPES
 * ========================================================================== */

/**
 * Memory-aware type reference.
 *
 * This delegates all type construction to the canonical Types grammar.
 */
memoryType
    : typeExpression
    ;


/**
 * Optional memory type argument.
 */
memoryTypedValue
    : typeExpression
      memoryValue?
    ;


/* ============================================================================
 * 8. MEMORY SPACES
 * ========================================================================== */

/**
 * Memory-space reference.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::distributed
 *     memory::persistent
 *     memory::device
 *     memory::future_domain
 *
 * The grammar does not define which spaces actually exist.
 */
memorySpace
    : memoryNamespaceName
    ;


/**
 * Memory-space selection.
 */
memorySpaceSelector
    : AT
      memoryQualifiedName
    ;


/**
 * Memory-space argument.
 *
 * This may be used by allocation, sharing, distribution, and constraints.
 */
memorySpaceArgument
    : memorySpaceSelector
    ;


/* ============================================================================
 * 9. MEMORY VALUES
 * ========================================================================== */

/**
 * Foundational memory value.
 *
 * This deliberately does not recreate the general expression grammar.
 *
 * It accepts source-level names and literal values that are sufficient for
 * memory-domain arguments while leaving general expression ownership with the
 * expression subsystem.
 */
memoryValue
    : memoryPlace
    | INTEGER
    | FLOAT
    | STRING_LITERAL
    | CHAR_LITERAL
    | TRUE
    | FALSE
    | NIL
    | NULL
    | memoryQualifiedName
    | parenthesizedMemoryValue
    ;


/**
 * Parenthesized memory value.
 */
parenthesizedMemoryValue
    : LPAREN
      memoryValue
      RPAREN
    ;


/* ============================================================================
 * 10. MEMORY ARGUMENTS
 * ========================================================================== */

/**
 * Positional memory arguments.
 *
 * No fixed argument count exists.
 */
memoryArgumentList
    : memoryArgument
      (COMMA memoryArgument)*
      COMMA?
    ;


/**
 * A memory argument.
 */
memoryArgument
    : memoryNamedArgument
    | memoryValue
    | memoryTypeArgument
    | memorySpaceArgument
    | memoryLifetimeReference
    | memoryConstraint
    ;


/**
 * Named memory argument.
 *
 * Example:
 *
 *     size = n
 *     alignment = alignment_requirement
 *
 * The meaning of the name is determined downstream.
 */
memoryNamedArgument
    : identifier
      ASSIGN
      memoryValue
    ;


/**
 * Type argument.
 */
memoryTypeArgument
    : typeExpression
    ;


/* ============================================================================
 * 11. GENERIC MEMORY OPERATION
 * ========================================================================== */

/**
 * Open-world memory operation.
 *
 * Examples:
 *
 *     memory::allocate(...)
 *     memory::deallocate(...)
 *     memory::shared(...)
 *     memory::distributed::migrate(...)
 *
 * The parser records the operation path.
 *
 * Semantic analysis determines whether the operation is:
 *
 *     allocation
 *     deallocation
 *     sharing
 *     migration
 *     replication
 *     mapping
 *     reservation
 *     or a future operation.
 */
memoryOperation
    : memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/**
 * Generic memory operation statement.
 */
memoryOperationStatement
    : memoryOperation
      SEMICOLON
    ;


/* ============================================================================
 * 12. ALLOCATION
 * ========================================================================== */

/**
 * Allocation operation.
 *
 * The operation name is intentionally represented structurally rather than
 * as a new lexer keyword.
 */
memoryAllocation
    : memoryQualifiedName
      LPAREN
      memoryAllocationArguments?
      RPAREN
    ;


/**
 * Allocation arguments.
 *
 * Typical semantic forms include:
 *
 *     type
 *     count
 *     size
 *     memory space
 *     lifetime
 *     alignment
 *     ownership mode
 *     constraints
 *
 * None of those meanings are fixed by this grammar.
 */
memoryAllocationArguments
    : memoryArgumentList
    ;


/**
 * Allocation statement.
 */
memoryAllocationStatement
    : memoryAllocation
      SEMICOLON
    ;


/**
 * Existing Zamani `new` construct integrated with the memory domain.
 *
 * Examples:
 *
 *     new T
 *     new T(...)
 *
 * This rule does not decide whether `new` means heap allocation, region
 * allocation, arena allocation, device allocation, or another strategy.
 */
memoryNewExpression
    : NEW
      typeExpression
      memoryConstructorArguments?
    ;


/**
 * Constructor arguments for `new`.
 */
memoryConstructorArguments
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 13. DEALLOCATION
 * ========================================================================== */

/**
 * Deallocation operation.
 *
 * Examples:
 *
 *     memory::deallocate(value)
 *     memory::release(value)
 *
 * The semantic layer determines whether explicit deallocation is legal.
 */
memoryDeallocation
    : memoryQualifiedName
      LPAREN
      memoryPlaceArgumentList
      RPAREN
    ;


/**
 * Place arguments used by deallocation/release operations.
 */
memoryPlaceArgumentList
    : memoryPlace
      (COMMA memoryPlace)*
      COMMA?
    ;


/**
 * Deallocation statement.
 */
memoryDeallocationStatement
    : memoryDeallocation
      SEMICOLON
    ;


/* ============================================================================
 * 14. SHARED MEMORY
 * ========================================================================== */

/**
 * Shared-memory operation foundation.
 *
 * Examples:
 *
 *     memory::shared(value)
 *     memory::share(value)
 *     memory::unshare(value)
 */
memorySharing
    : memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/**
 * Shared-memory statement.
 */
memorySharingStatement
    : memorySharing
      SEMICOLON
    ;


/**
 * Shared-memory binding.
 *
 * The binding does not imply a physical coherence implementation.
 */
memorySharedBinding
    : memoryOwnershipMode?
      LET
      identifier
      memoryTypeAnnotation?
      ASSIGN
      memorySharing
      SEMICOLON
    ;


/* ============================================================================
 * 15. DISTRIBUTED MEMORY
 * ========================================================================== */

/**
 * Distributed-memory operation foundation.
 *
 * Examples:
 *
 *     memory::distributed(value)
 *     memory::remote(value)
 *     memory::replicate(value)
 *     memory::migrate(value)
 */
memoryDistribution
    : memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/**
 * Distributed-memory statement.
 */
memoryDistributionStatement
    : memoryDistribution
      SEMICOLON
    ;


/**
 * Distributed-memory binding.
 */
memoryDistributedBinding
    : memoryOwnershipMode?
      LET
      identifier
      memoryTypeAnnotation?
      ASSIGN
      memoryDistribution
      SEMICOLON
    ;


/* ============================================================================
 * 16. REQUIREMENTS
 * ========================================================================== */

/**
 * Memory requirement.
 *
 * The first component identifies the required semantic property.
 *
 * Examples:
 *
 *     memory::shared
 *     memory::distributed
 *     memory::persistent
 *     memory::address_stable
 */
memoryRequirement
    : REQUIRE_MEMORY
      memoryQualifiedName
      memoryRequirementArguments?
      SEMICOLON?
    ;


/**
 * Requirement arguments.
 */
memoryRequirementArguments
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 17. CONSTRAINTS
 * ========================================================================== */

/**
 * Memory constraint.
 *
 * Constraints are represented structurally and interpreted downstream.
 */
memoryConstraint
    : CONSTRAINT_MEMORY
      memoryConstraintBody
      SEMICOLON?
    ;


/**
 * Constraint body.
 *
 * The grammar intentionally supports:
 *
 *     named properties;
 *     qualified properties;
 *     values;
 *     lists.
 */
memoryConstraintBody
    : memoryQualifiedName
      (ASSIGN memoryValue)?
      memoryConstraintTail*
    ;


/**
 * Additional constraint fields.
 */
memoryConstraintTail
    : COMMA
      memoryQualifiedName
      (ASSIGN memoryValue)?
    ;


/* ============================================================================
 * 18. PREFERENCES
 * ========================================================================== */

/**
 * Non-binding memory preference.
 */
memoryPreference
    : PREFER_MEMORY
      memoryQualifiedName
      memoryPreferenceBody?
      SEMICOLON?
    ;


/**
 * Preference body.
 */
memoryPreferenceBody
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 19. HINTS
 * ========================================================================== */

/**
 * Non-binding memory implementation hint.
 */
memoryHint
    : HINT_MEMORY
      memoryQualifiedName
      memoryHintBody?
      SEMICOLON?
    ;


/**
 * Hint body.
 */
memoryHintBody
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 20. MEMORY POLICY
 * ========================================================================== */

/**
 * Structured memory policy.
 *
 * The policy remains declarative.
 *
 * It does not execute anything.
 */
memoryPolicy
    : AT
      memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 21. MEMORY RESOURCE DESCRIPTION
 * ========================================================================== */

/**
 * Generic memory resource descriptor.
 *
 * A resource is not the same thing as a physical allocation.
 */
memoryResource
    : memoryQualifiedName
      memoryResourceArguments?
    ;


/**
 * Resource arguments.
 */
memoryResourceArguments
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 22. MEMORY REGION
 * ========================================================================== */

/**
 * Source-level region reference.
 *
 * A region is a semantic abstraction.
 *
 * It is not a physical address range.
 */
memoryRegion
    : memoryQualifiedName
    ;


/**
 * Region selector.
 */
memoryRegionSelector
    : AT
      memoryRegion
    ;


/* ============================================================================
 * 23. MEMORY ADDRESS ABSTRACTION
 * ========================================================================== */

/**
 * Source-level address expression.
 *
 * This is intentionally a semantic address abstraction rather than a numeric
 * physical address.
 */
memoryAddress
    : memoryQualifiedName
    ;


/**
 * Address selector.
 *
 * The semantic layer determines whether an address abstraction is meaningful
 * for the selected target.
 */
memoryAddressSelector
    : AT
      memoryAddress
    ;


/* ============================================================================
 * 24. MEMORY EXTENSION POINT
 * ========================================================================== */

/**
 * Open-world memory extension.
 *
 * Any future memory-domain construct can be represented through a qualified
 * operation/value without changing the foundational grammar.
 */
memoryExtension
    : memoryQualifiedName
      memoryExtensionPayload?
    ;


/**
 * Extension payload.
 */
memoryExtensionPayload
    : LPAREN
      memoryArgumentList?
      RPAREN
    | LBRACKET
      memoryArgumentList?
      RBRACKET
    ;


/* ============================================================================
 * 25. INTERNAL TOKEN-COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * These token aliases deliberately have no lexer implementation here.
 *
 * They identify semantic keywords that should eventually be made explicit
 * lexical tokens if and when the language specification decides that their
 * spellings are globally reserved.
 *
 * Until then, the memory namespace remains open-world and these concepts can
 * be represented by qualified identifiers.
 *
 * IMPORTANT:
 *
 * ANTLR parser grammars cannot safely invent a token that the canonical lexer
 * does not emit for a literal spelling and expect the existing lexer to
 * produce it.
 *
 * Therefore the production integration must NOT change the lexer merely to
 * make this foundation work.
 *
 * The actual current canonical lexical vocabulary remains the authority.
 *
 * These rules are represented using identifiers at the syntax level below.
 */


/**
 * Memory requirement marker.
 *
 * Canonical source spelling is interpreted semantically from an identifier.
 *
 * A future lexical reservation may replace this with a dedicated token without
 * changing the semantic model.
 */
REQUIRE_MEMORY
    : identifier
    ;


/**
 * Memory constraint marker.
 */
CONSTRAINT_MEMORY
    : identifier
    ;


/**
 * Memory preference marker.
 */
PREFER_MEMORY
    : identifier
    ;


/**
 * Memory hint marker.
 */
HINT_MEMORY
    : identifier
    ;


/* ============================================================================
 * 26. INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * IMPORTANT ANTLR RULE:
 *
 * Parser rule names must begin with lowercase characters.
 *
 * The four compatibility rules above are intentionally NOT lexer rules in the
 * conceptual architecture. They exist only to describe parser-level marker
 * structure.
 *
 * If the canonical grammar assembler imports this parser grammar, these rules
 * must be consumed through parser composition rather than treated as tokens.
 *
 * Therefore host grammars should prefer the explicit memory namespace forms:
 *
 *     memory::...
 *
 * and specialized memory files should import this foundation.
 */


/* ============================================================================
 * 27. SPECIALIZED-GRAMMAR CONTRACT
 * ========================================================================== */

/*
 * ownership.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryOwnershipQualifier
 *     memoryOwnershipMode
 *     memoryOwnedBinding
 *
 * Owns:
 *
 *     ownership-specific source constructs.
 *
 *
 * borrowing.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryBorrow
 *     memoryMutableBorrow
 *     memoryBorrowExpression
 *     memoryPlace
 *
 * Owns:
 *
 *     borrow-specific source constructs.
 *
 *
 * lifetimes.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryLifetime
 *     memoryLifetimeReference
 *     memoryLifetimeRelation
 *
 * Owns:
 *
 *     lifetime declaration/relationship syntax.
 *
 *
 * allocation.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryAllocation
 *     memoryNewExpression
 *     memoryAllocationArguments
 *
 * Owns:
 *
 *     allocation-specific source syntax.
 *
 *
 * deallocation.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryDeallocation
 *     memoryPlaceArgumentList
 *
 * Owns:
 *
 *     explicit-release/deallocation syntax.
 *
 *
 * shared-memory.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memorySharing
 *     memorySharedBinding
 *
 * Owns:
 *
 *     shared-memory-specific syntax.
 *
 *
 * distributed-memory.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryDistribution
 *     memoryDistributedBinding
 *
 * Owns:
 *
 *     distributed-memory-specific syntax.
 *
 *
 * memory-constraints.g4
 *
 * Imports:
 *
 *     Memory
 *
 * Consumes:
 *
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *     memoryResource
 *
 * Owns:
 *
 *     memory resource/constraint declaration syntax.
 */


/* ============================================================================
 * 28. IR INTEGRATION
 * ========================================================================== */

/*
 * This grammar MUST NOT construct IR.
 *
 * The intended flow is:
 *
 *     Memory parse tree
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic memory model
 *          |
 *          +------------------------+
 *          |                        |
 *          v                        v
 *     classical semantics     resource semantics
 *          |                        |
 *          +------------+-----------+
 *                       |
 *                       v
 *                 canonical IR
 *
 * For quantum programs:
 *
 *     memory syntax
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * Memory.g4 MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumCircuit
 *     GateKind
 *     HardwareMemoryId
 *     DeviceMemoryId
 *
 * and MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware discovery
 *     resilience.
 */


/* ============================================================================
 * 29. COMPILER INTEGRATION
 * ========================================================================== */

/*
 * The compiler consumes this grammar through the normal parser pipeline.
 *
 * The compiler may perform:
 *
 *     syntax validation
 *     AST construction
 *     structural validation
 *     name resolution
 *     type resolution
 *     ownership analysis
 *     borrow checking
 *     lifetime analysis
 *     alias analysis
 *     resource analysis
 *     capability analysis
 *     target-independent lowering
 *
 * None of these operations occur in this grammar.
 *
 * Compiler policies may impose resource limits for safety.
 *
 * Such limits MUST be:
 *
 *     explicit;
 *     configurable;
 *     observable;
 *     independent from language grammar semantics;
 *     independent from hardware identities.
 */


/* ============================================================================
 * 30. RUNTIME INTEGRATION
 * ========================================================================== */

/*
 * Runtime does NOT depend directly on Memory.g4.
 *
 * Runtime consumes semantic/compiled representations produced downstream.
 *
 * Therefore:
 *
 *     runtime -> Memory.g4
 *
 * is forbidden.
 *
 * Correct:
 *
 *     Memory.g4
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic memory model
 *          |
 *          v
 *     compiler IR
 *          |
 *          v
 *     runtime
 */


/* ============================================================================
 * 31. TOOLING INTEGRATION
 * ========================================================================== */

/*
 * Tooling may consume parse trees for:
 *
 *     syntax highlighting;
 *     formatting;
 *     source indexing;
 *     documentation;
 *     diagnostics;
 *     IDE completion;
 *     refactoring.
 *
 * Tooling MUST NOT infer physical memory behavior from syntax alone.
 *
 * For example:
 *
 *     memory::shared
 *
 * must not cause an IDE to claim:
 *
 *     "This uses shared RAM."
 *
 * unless semantic information has resolved that meaning.
 */


/* ============================================================================
 * 32. SECURITY CONTRACT
 * ========================================================================== */

/*
 * Memory syntax is declarative.
 *
 * The parser MUST NOT:
 *
 *     - allocate;
 *     - free;
 *     - map;
 *     - unmap;
 *     - inspect addresses;
 *     - inspect process memory;
 *     - inspect devices;
 *     - access operating-system APIs;
 *     - access environment variables;
 *     - execute external programs;
 *     - perform FFI;
 *     - access networks.
 *
 * All such behavior belongs to later explicitly authorized compiler/runtime
 * components.
 */


/* ============================================================================
 * 33. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * No production grammar rule in this file imposes a machine-size bound.
 *
 * Repetition is used for:
 *
 *     qualified names;
 *     argument lists;
 *     constraint fields;
 *     place projections;
 *     lifetime relations.
 *
 * There are no finite grammar constants representing:
 *
 *     number of memory objects;
 *     number of regions;
 *     number of allocations;
 *     number of references;
 *     number of distributed locations;
 *     number of memory spaces.
 *
 * Actual parser stack/resource exhaustion is an implementation/resource
 * concern, not a source-language limit.
 */


/* ============================================================================
 * 34. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * The grammar contains:
 *
 *     no target-specific actions;
 *     no semantic predicates;
 *     no Rust code;
 *     no I/O;
 *     no environment-dependent behavior;
 *     no hardware discovery;
 *     no generated identifiers.
 *
 * Parsing therefore remains deterministic for a fixed lexer vocabulary and
 * grammar version.
 */


/* ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Backward-compatible source constructs must continue to parse unless an
 * explicit language-version policy changes their meaning.
 *
 * Existing:
 *
 *     linear
 *     affine
 *     mut
 *     &
 *     &mut
 *     lifetime syntax
 *     new
 *
 * remain compatible with the existing type/lexer architecture.
 *
 * This file does not redefine the existing reference-type syntax.
 */


/* ============================================================================
 * 36. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * AUDIT RESULT
 *
 * Fixed qubit count:
 *     NONE
 *
 * Fixed CPU count:
 *     NONE
 *
 * Fixed core count:
 *     NONE
 *
 * Fixed thread count:
 *     NONE
 *
 * Fixed memory capacity:
 *     NONE
 *
 * Fixed address width:
 *     NONE
 *
 * Fixed pointer width:
 *     NONE
 *
 * Fixed heap size:
 *     NONE
 *
 * Fixed stack size:
 *     NONE
 *
 * Fixed memory-bank count:
 *     NONE
 *
 * Fixed NUMA-node count:
 *     NONE
 *
 * Fixed device-memory count:
 *     NONE
 *
 * Fixed allocation count:
 *     NONE
 *
 * Fixed lifetime count:
 *     NONE
 *
 * Fixed reference count:
 *     NONE
 *
 * Fixed distributed-node count:
 *     NONE
 *
 * Fixed topology:
 *     NONE
 *
 * Fixed device identifier:
 *     NONE
 *
 * Fixed vendor:
 *     NONE
 *
 * Fixed backend:
 *     NONE
 *
 * Fixed address:
 *     NONE
 *
 * Fixed machine width:
 *     NONE
 *
 * ============================================================================
 * 37. COMPLETION CRITERIA
 * ============================================================================
 *
 * Memory.g4 is complete when:
 *
 * [ ] It compiles against the canonical ZamaniLexer vocabulary.
 *
 * [ ] It imports only the canonical Types parser dependency required for
 *     source-level type expressions.
 *
 * [ ] It introduces no lexer rules.
 *
 * [ ] It introduces no Rust actions.
 *
 * [ ] It introduces no semantic predicates.
 *
 * [ ] It uses no unsafe implementation.
 *
 * [ ] It does not define an ownership semantic model.
 *
 * [ ] It does not define a second reference type model.
 *
 * [ ] It does not define a second lifetime model.
 *
 * [ ] It does not allocate memory.
 *
 * [ ] It does not deallocate memory.
 *
 * [ ] It does not inspect hardware.
 *
 * [ ] It does not select a target.
 *
 * [ ] It does not depend on quantum::ir.
 *
 * [ ] It does not depend on QEC.
 *
 * [ ] It does not depend on ZQN.
 *
 * [ ] It does not depend on scheduling.
 *
 * [ ] It does not depend on routing.
 *
 * [ ] It does not depend on optimization.
 *
 * [ ] It contains no machine-size constants.
 *
 * [ ] It contains no physical address assumptions.
 *
 * [ ] It supports arbitrarily qualified memory-domain names.
 *
 * [ ] It preserves source-level lifetime information.
 *
 * [ ] It preserves ownership/borrow intent.
 *
 * [ ] It supports memory requirements.
 *
 * [ ] It supports memory constraints.
 *
 * [ ] It supports memory preferences.
 *
 * [ ] It supports memory hints.
 *
 * [ ] It supports allocation/deallocation syntax without prescribing an
 *     allocator.
 *
 * [ ] It supports shared-memory syntax without prescribing coherence.
 *
 * [ ] It supports distributed-memory syntax without prescribing topology.
 *
 * [ ] It provides stable extension points for future memory technologies.
 *
 * [ ] Specialized memory grammars can depend on this file without modifying
 *     its foundational rules.
 *
 * [ ] The host statement/declaration grammar can integrate the memory entry
 *     points without creating circular dependencies.
 *
 * ============================================================================
 */