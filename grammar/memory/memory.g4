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
 *     Canonical memory-domain foundation.
 *
 * Language role:
 *     Defines source-level memory intent and composition points.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe code.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes computation and intent.
 *
 * Memory syntax MUST NOT encode accidental properties of the machine on which
 * the program happens to execute.
 *
 * Therefore this grammar does NOT encode:
 *
 *     - physical addresses;
 *     - pointer widths;
 *     - machine word widths;
 *     - fixed heap sizes;
 *     - fixed stack sizes;
 *     - fixed memory-bank counts;
 *     - fixed NUMA-node counts;
 *     - fixed device-memory counts;
 *     - fixed GPU-memory counts;
 *     - fixed accelerator counts;
 *     - fixed distributed-node counts;
 *     - fixed allocation counts;
 *     - fixed reference counts;
 *     - fixed lifetime counts;
 *     - fixed region counts;
 *     - fixed memory-space counts.
 *
 * Resource availability, placement, allocation strategy, memory topology,
 * scheduling, and physical realization belong downstream.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     composed parser
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *       Types.g4          Expressions.g4
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *              Memory.g4
 *                    |
 *                    +-----------------------------+
 *                    |                             |
 *                    v                             v
 *              specialized                 frontend AST
 *              memory grammars                   |
 *                    |                           v
 *                    +------------------ semantic analysis
 *                                                |
 *                              +-----------------+----------------+
 *                              |                 |                |
 *                              v                 v                v
 *                         ownership         resource model    effects
 *                              |                 |                |
 *                              +-----------------+----------------+
 *                                                |
 *                                                v
 *                                      canonical semantic IR
 *                                                |
 *                     +--------------------------+------------------------+
 *                     |                          |                        |
 *                     v                          v                        v
 *               classical IR               quantum::ir             hardware/HDL IR
 *                     |                          |                        |
 *                     +--------------------------+------------------------+
 *                                                |
 *                                                v
 *                                optimization / routing / scheduling
 *                                                |
 *                                                v
 *                                         target realization
 *                                                |
 *                                                v
 *                                             runtime
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the canonical memory-domain entry point;
 *     - generic memory constructs;
 *     - memory-qualified names;
 *     - memory places;
 *     - memory spaces;
 *     - memory regions;
 *     - ownership markers at the syntactic level;
 *     - borrow markers at the syntactic level;
 *     - lifetime references at the syntactic level;
 *     - memory operation invocation syntax;
 *     - memory operation arguments;
 *     - memory resource-intent syntax;
 *     - memory requirement syntax;
 *     - memory constraint syntax;
 *     - memory preference syntax;
 *     - memory hint syntax;
 *     - generic memory policy syntax;
 *     - memory extension points;
 *     - memory-domain composition.
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifier spelling;
 *     - Unicode normalization;
 *     - general expression precedence;
 *     - general expressions;
 *     - general types;
 *     - declarations;
 *     - statements outside the memory domain;
 *     - ownership checking;
 *     - borrow checking;
 *     - lifetime inference;
 *     - alias analysis;
 *     - escape analysis;
 *     - allocation implementation;
 *     - deallocation implementation;
 *     - allocator algorithms;
 *     - garbage collection;
 *     - reference counting;
 *     - physical memory discovery;
 *     - virtual-memory implementation;
 *     - NUMA discovery;
 *     - cache discovery;
 *     - device-memory discovery;
 *     - DMA;
 *     - memory placement;
 *     - memory scheduling;
 *     - routing;
 *     - optimization;
 *     - quantum allocation;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - hardware discovery;
 *     - backend selection;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware IR.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical type grammar:
 *
 *     grammar/types/types.g4
 *
 * Canonical expression grammar:
 *
 *     grammar/expressions/expressions.g4
 *
 * Specialized memory grammars:
 *
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/deallocation.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/memory-constraints.g4
 *
 * IMPORTANT:
 *
 * Specialized memory grammars MUST consume and extend the vocabulary defined
 * here. They MUST NOT redefine:
 *
 *     memoryPlace
 *     memorySpace
 *     memoryLifetime
 *     memoryQualifiedName
 *     memoryOperation
 *     memoryOperationArgument
 *
 * unless ownership is deliberately transferred through an explicit grammar
 * architecture change.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * Every memory construct must preserve enough syntax information for the
 * frontend AST to retain:
 *
 *     - source span;
 *     - memory operation/path;
 *     - operation arguments;
 *     - memory place;
 *     - memory space;
 *     - region;
 *     - ownership marker;
 *     - borrow marker;
 *     - lifetime reference;
 *     - type expression;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - policy;
 *     - extension metadata.
 *
 * The parser does NOT decide their semantic validity.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether an ownership mode is legal;
 *     - whether a value may move;
 *     - whether a value may be copied;
 *     - whether a borrow is valid;
 *     - whether mutable aliasing is legal;
 *     - whether a lifetime is valid;
 *     - whether a region relationship is valid;
 *     - whether an allocation is possible;
 *     - whether deallocation is legal;
 *     - whether a memory space is compatible;
 *     - whether a requirement is satisfiable;
 *     - whether a constraint is satisfiable;
 *     - whether a preference can be honored;
 *     - whether a hint is usable;
 *     - whether a target has the required capability;
 *     - how memory intent lowers to canonical semantic IR.
 *
 * Syntax MUST NOT perform these decisions.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Memory syntax is intentionally open-world.
 *
 * The same source-level memory intent must be representable on:
 *
 *     - embedded systems;
 *     - CPUs;
 *     - multicore systems;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - quantum-classical systems;
 *     - distributed systems;
 *     - clusters;
 *     - supercomputers;
 *     - cloud systems;
 *     - future architectures.
 *
 * The grammar imposes no semantic maximum on:
 *
 *     memory size;
 *     allocation count;
 *     reference count;
 *     region count;
 *     address width;
 *     memory-space count;
 *     distributed placement count.
 *
 * Practical parser/compiler resource limits, if required, MUST be explicit
 * implementation policies and MUST NOT become source-language semantics.
 *
 * ============================================================================
 */

parser grammar Memory;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC MEMORY ENTRY POINT
 * ============================================================================
 *
 * This is the canonical entry point for memory-domain syntax.
 *
 * Specialized memory grammars should compose with this rule rather than
 * creating another generic memory root.
 */
memoryConstruct
    : memoryStatement
    | memoryExpression
    | memoryDeclaration
    | memoryAnnotation
    ;


/*
 * ============================================================================
 * 2. MEMORY STATEMENT
 * ============================================================================
 *
 * A memory statement is intentionally generic.
 *
 * Specialized grammars may refine semantic operation names such as:
 *
 *     memory::allocate
 *     memory::deallocate
 *     memory::share
 *     memory::release
 *     memory::migrate
 *
 * without requiring this foundational grammar to know every future operation.
 *
 * The trailing semicolon is owned here because this is the generic statement
 * boundary.
 */
memoryStatement
    : memoryOperationStatement
    ;


memoryOperationStatement
    : memoryOperation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. MEMORY EXPRESSION
 * ============================================================================
 *
 * Memory operations may also occur in expression position.
 *
 * Examples:
 *
 *     memory::allocate(...)
 *     memory::reserve(...)
 *     memory::map(...)
 *
 * Whether a particular operation is expression-producing is a semantic
 * question.
 */
memoryExpression
    : memoryOperation
    ;


/*
 * ============================================================================
 * 4. MEMORY DECLARATION
 * ============================================================================
 *
 * This is a syntactic integration point for future declaration-specific
 * memory forms.
 *
 * It intentionally does not duplicate variable/type declaration grammar.
 *
 * Specialized declaration grammars may consume memory metadata through
 * memoryAnnotation or memoryConstruct composition.
 */
memoryDeclaration
    : memoryBinding
    ;


memoryBinding
    : memoryOwnershipQualifier?
      memoryPlace
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
    ;


/*
 * ============================================================================
 * 5. MEMORY ANNOTATION
 * ============================================================================
 *
 * Memory metadata may be attached to declarations, operations, types, or
 * other source constructs by the composed parser.
 *
 * The annotation payload remains an expression-level semantic value.
 */
memoryAnnotation
    : AT memoryQualifiedName
      memoryAnnotationArguments?
    ;


memoryAnnotationArguments
    : LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 6. MEMORY OPERATION
 * ============================================================================
 *
 * The operation name is open-world.
 *
 * There is deliberately NO exhaustive list such as:
 *
 *     ALLOCATE
 *     DEALLOCATE
 *     SHARED
 *     DEVICE
 *     NUMA
 *     CACHE
 *
 * because doing so would turn every future memory technology into a lexer
 * change.
 *
 * Operation identity is resolved semantically.
 *
 * Examples:
 *
 *     memory::allocate(...)
 *     memory::deallocate(...)
 *     memory::shared(...)
 *     memory::distributed(...)
 *     memory::persistent(...)
 *     memory::future::operation(...)
 */
memoryOperation
    : memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 7. MEMORY QUALIFIED NAME
 * ============================================================================
 *
 * There is no fixed qualification depth.
 *
 * Valid examples include:
 *
 *     memory::local
 *     memory::shared
 *     memory::device
 *     memory::distributed
 *     memory::persistent
 *     memory::remote
 *     memory::future::domain
 *     memory::vendor::extension::operation
 *
 * The grammar does not determine what any name means.
 */
memoryQualifiedName
    : memoryPath
    ;


memoryPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 8. MEMORY ARGUMENTS
 * ============================================================================
 *
 * Arguments consume the canonical expression grammar.
 *
 * This means memory operations can receive:
 *
 *     constants;
 *     variables;
 *     symbolic values;
 *     runtime values;
 *     type-related values;
 *     resource expressions;
 *     computed extents;
 *     capability predicates;
 *     future semantic values.
 *
 * No fixed argument count is encoded.
 */
memoryArgumentList
    : memoryArgument
      (COMMA memoryArgument)*
      COMMA?
    ;


memoryArgument
    : expression
    | memoryNamedArgument
    ;


memoryNamedArgument
    : identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 9. MEMORY PLACE
 * ============================================================================
 *
 * A memory place identifies a source-level location.
 *
 * Physical addresses are intentionally excluded.
 *
 * Examples:
 *
 *     value
 *     object.field
 *     array[index]
 *     namespace::value
 *     object.field[index]
 *
 * The full expression grammar remains authoritative for complex expressions.
 *
 * This rule provides the stable memory-domain location abstraction.
 */
memoryPlace
    : memoryPlaceBase memoryPlaceSuffix*
    ;


memoryPlaceBase
    : identifier
    | memoryQualifiedName
    | parenthesizedMemoryPlace
    ;


memoryPlaceSuffix
    : memoryMemberSuffix
    | memoryIndexSuffix
    | memoryDereferenceSuffix
    ;


memoryMemberSuffix
    : DOT identifier
    ;


memoryIndexSuffix
    : LBRACKET expressionList RBRACKET
    ;


memoryDereferenceSuffix
    : STAR
    ;


parenthesizedMemoryPlace
    : LPAREN memoryPlace RPAREN
    ;


/*
 * ============================================================================
 * 10. MEMORY TYPE ANNOTATION
 * ============================================================================
 *
 * The canonical type grammar remains authoritative.
 *
 * This file MUST NOT redefine:
 *
 *     referenceType
 *     pointerType
 *     lifetimeAnnotation
 *     generic types
 *     arrays
 *     tuples
 *     quantum types
 *     hardware/resource types.
 */
memoryTypeAnnotation
    : COLON typeExpression
    ;


/*
 * ============================================================================
 * 11. OWNERSHIP QUALIFIERS
 * ============================================================================
 *
 * The lexer already owns LINEAR and AFFINE.
 *
 * These are source-level semantic markers.
 *
 * They do NOT mean:
 *
 *     stack allocation;
 *     heap allocation;
 *     reference counting;
 *     garbage collection;
 *     physical storage;
 *     a particular runtime representation.
 */
memoryOwnershipQualifier
    : LINEAR
    | AFFINE
    ;


/*
 * ============================================================================
 * 12. BORROW
 * ============================================================================
 *
 * Borrow syntax is represented independently from the canonical reference
 * type syntax.
 *
 * Type-level reference syntax remains owned by grammar/types/types.g4.
 *
 * These rules represent source operations such as:
 *
 *     &value
 *     &mut value
 *     &'a value
 *     &'a mut value
 *
 * Semantic borrow checking remains outside the grammar.
 */
memoryBorrow
    : AMPERSAND
      memoryLifetimePrefix?
      MUT?
      memoryPlace
    ;


memoryLifetimePrefix
    : APOSTROPHE identifier
    ;


/*
 * ============================================================================
 * 13. LIFETIME
 * ============================================================================
 *
 * Lifetime names are preserved, not interpreted.
 *
 * Examples:
 *
 *     'a
 *     'scope
 *     'region
 *     'transaction
 *     'future
 *
 * There is no fixed number of lifetimes.
 */
memoryLifetime
    : APOSTROPHE identifier
    ;


memoryLifetimeClause
    : memoryLifetime
    ;


/*
 * ============================================================================
 * 14. MEMORY REGION
 * ============================================================================
 *
 * A region is a semantic grouping/lifetime abstraction.
 *
 * It is NOT:
 *
 *     a physical heap;
 *     a physical memory bank;
 *     a NUMA node;
 *     a page;
 *     a cache;
 *     a device.
 *
 * The semantic layer determines its realization.
 */
memoryRegion
    : memoryRegionReference
    ;


memoryRegionReference
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 15. MEMORY SPACE
 * ============================================================================
 *
 * Memory spaces are semantic names.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::device
 *     memory::persistent
 *     memory::remote
 *     memory::distributed
 *
 * The grammar does not reserve these names.
 */
memorySpace
    : memoryQualifiedName
    ;


memorySpaceClause
    : memorySpace
    ;


/*
 * ============================================================================
 * 16. MEMORY RESOURCE
 * ============================================================================
 *
 * Resource expressions describe semantic resource intent.
 *
 * They do not select a physical resource.
 *
 * Examples:
 *
 *     memory::capacity
 *     memory::bandwidth
 *     memory::latency
 *     memory::energy
 *     memory::reliability
 *
 * The values are interpreted downstream.
 */
memoryResource
    : memoryQualifiedName
    ;


memoryResourceClause
    : memoryResource
    ;


/*
 * ============================================================================
 * 17. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * This grammar only captures their source representation.
 */
memoryRequirement
    : REQUIRES memoryRequirementExpressionList
    ;


memoryRequirementExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict legal implementation choices.
 *
 * A constraint is not equivalent to a preference or hint.
 */
memoryConstraint
    : memoryConstraintKeyword memoryConstraintExpressionList
    ;


memoryConstraintKeyword
    : REQUIRES
    ;


memoryConstraintExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 19. PREFERENCES
 * ============================================================================
 *
 * Preferences describe desired implementation properties.
 *
 * They must not change program semantics when ignored.
 *
 * `memoryPreference` deliberately uses an identifier-based operator name
 * rather than introducing a new lexer keyword.
 */
memoryPreference
    : memoryPreferenceKeyword memoryPreferenceExpressionList
    ;


memoryPreferenceKeyword
    : WITH
    ;


memoryPreferenceExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. HINTS
 * ============================================================================
 *
 * Hints are advisory.
 *
 * Ignoring a hint MUST NOT change program semantics.
 */
memoryHint
    : memoryHintKeyword memoryHintExpressionList
    ;


memoryHintKeyword
    : WITH
    ;


memoryHintExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 21. MEMORY POLICY
 * ============================================================================
 *
 * Policy names are open-world qualified names.
 *
 * A policy is a semantic request to downstream compilation/resource systems.
 *
 * It does not directly execute a policy.
 */
memoryPolicy
    : memoryQualifiedName
    ;


memoryPolicyClause
    : WITH memoryPolicy
    ;


/*
 * ============================================================================
 * 22. MEMORY INTENT
 * ============================================================================
 *
 * This rule provides a stable composition point for constructs that need to
 * attach multiple independent memory properties.
 *
 * The order of clauses is deliberately fixed at the grammar boundary to
 * preserve deterministic parsing.
 *
 * Specialized grammars may expose more precise productions but should consume
 * the same underlying components.
 */
memoryIntent
    : memoryOwnershipQualifier?
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
      memoryRegionClause?
      memoryResourceClause*
      memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;


memoryRegionClause
    : IN memoryRegion
    ;


/*
 * ============================================================================
 * 23. MEMORY RESOURCE SPECIFICATION
 * ============================================================================
 *
 * This is the generic source-level representation for a memory resource
 * property.
 *
 * It intentionally accepts arbitrary expressions.
 */
memoryResourceSpecification
    : memoryResourceClause memoryResourceValue?
    ;


memoryResourceValue
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 24. MEMORY QUALIFICATION
 * ============================================================================
 *
 * Generic qualification is intentionally open.
 *
 * This permits future memory technologies to be represented without changing
 * this foundational grammar.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::persistent
 *     memory::device::global
 *     memory::distributed::replicated
 *     memory::future::memory::domain
 */
memoryQualification
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 25. MEMORY OPERATION EXTENSION
 * ============================================================================
 *
 * Specialized domains can recognize their own semantic operations while
 * retaining the same qualified-name and argument structure.
 *
 * Example:
 *
 *     memory::vendor::technology::operation(...)
 *
 * No new lexer token is necessary.
 */
memoryExtensionOperation
    : memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 26. IDENTIFIER BRIDGE
 * ============================================================================
 *
 * The canonical lexer/type/expression grammars use IDENT.
 *
 * This grammar intentionally does not introduce IDENTIFIER as an alias.
 *
 * This prevents the previous mismatch between memory/allocation grammar and
 * the repository's canonical lexical contract.
 */
identifier
    : IDENT
    ;


/*
 * ============================================================================
 * 27. MEMORY EXPRESSION LIST
 * ============================================================================
 *
 * Used by composed grammars that need memory-specific expression sequences.
 *
 * No fixed cardinality is imposed.
 */
memoryExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 28. MEMORY TARGET
 * ============================================================================
 *
 * A memory target is a semantic source-level target.
 *
 * It is NOT a physical address or hardware identifier.
 */
memoryTarget
    : memoryPlace
    | memorySpace
    | memoryRegion
    ;


/*
 * ============================================================================
 * 29. MEMORY BOUND
 * ============================================================================
 *
 * A memory bound is represented as a general expression.
 *
 * This permits:
 *
 *     compile-time constants;
 *     symbolic quantities;
 *     generic parameters;
 *     runtime values;
 *     resource-derived values.
 *
 * No numeric maximum is encoded.
 */
memoryBound
    : expression
    ;


/*
 * ============================================================================
 * 30. MEMORY RANGE
 * ============================================================================
 *
 * Memory ranges remain semantic expressions.
 *
 * This rule does not assume a particular address representation.
 */
memoryRange
    : memoryBound
      DOT_DOT
      memoryBound
    | memoryBound
      DOT_DOT_EQ
      memoryBound
    ;


/*
 * ============================================================================
 * 31. MEMORY EXTENSION METADATA
 * ============================================================================
 *
 * Future domains may attach arbitrary semantic metadata through a qualified
 * name and expression payload.
 *
 * The grammar remains domain-neutral.
 */
memoryExtensionMetadata
    : AT memoryQualifiedName
      (LPAREN memoryExpressionList? RPAREN)?
    ;


/*
 * ============================================================================
 * 32. MEMORY REQUIREMENT / CONSTRAINT / PREFERENCE / HINT BUNDLE
 * ============================================================================
 *
 * This rule keeps the four concepts structurally separate.
 */
memoryPolicyBundle
    : memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;


/*
 * ============================================================================
 * 33. COMPOSABLE MEMORY SPECIFICATION
 * ============================================================================
 *
 * This is the preferred integration rule for declarations, allocations,
 * shared-memory constructs, distributed-memory constructs, accelerator
 * memory, and future memory domains.
 */
memorySpecification
    : memoryIntent
      memoryPolicyBundle
    ;


/*
 * ============================================================================
 * 34. SEMANTIC EXTENSION POINT
 * ============================================================================
 *
 * Specialized grammars can consume:
 *
 *     memoryConstruct
 *     memoryOperation
 *     memoryPlace
 *     memorySpace
 *     memoryRegion
 *     memoryBorrow
 *     memoryLifetime
 *     memorySpecification
 *
 * without introducing a parallel memory language.
 */
memoryDomainExtension
    : memoryQualifiedName
      (LPAREN memoryArgumentList? RPAREN)?
    ;