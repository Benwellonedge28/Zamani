/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/memory-constraints.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Role:
 *     Memory-domain constraint specialization.
 *
 * Status:
 *     Production-ready grammar module.
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
 * This file defines SOURCE-LEVEL MEMORY CONSTRAINT SYNTAX.
 *
 * It specializes the canonical Zamani constraint system for memory-domain
 * semantics without creating a second constraint language.
 *
 * The grammar describes conditions that memory-related declarations,
 * operations, regions, places, or semantic memory domains must satisfy.
 *
 * Examples of concepts representable through this grammar include:
 *
 *     - required memory properties;
 *     - forbidden memory properties;
 *     - addressability requirements;
 *     - alignment requirements;
 *     - access requirements;
 *     - visibility requirements;
 *     - persistence requirements;
 *     - locality constraints;
 *     - sharing constraints;
 *     - migration constraints;
 *     - residency constraints;
 *     - consistency constraints;
 *     - ordering constraints;
 *     - atomic-access constraints;
 *     - latency constraints;
 *     - bandwidth constraints;
 *     - capacity constraints;
 *     - durability constraints;
 *     - reliability constraints;
 *     - portability constraints;
 *     - resource relationships;
 *     - memory-space compatibility;
 *     - memory capability requirements.
 *
 * The grammar intentionally does NOT decide whether any of these conditions
 * can actually be satisfied.
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
 *     Core parser
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     Expressions                    Constraints
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                    Memory grammar
 *                          |
 *                          v
 *                 memory-constraints.g4
 *                          |
 *                          v
 *                     frontend AST
 *                          |
 *                          v
 *                  semantic analysis
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *          type/effect  resource    capability
 *            analysis    analysis     analysis
 *              |           |           |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                 canonical semantic IR
 *                          |
 *          +---------------+---------------+
 *          |               |               |
 *          v               v               v
 *     classical IR     quantum::ir      hardware IR
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *               optimization / routing /
 *                 scheduling / lowering
 *                          |
 *                          v
 *                       runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - memory-specific constraint composition;
 *   - memory constraint subjects;
 *   - memory constraint predicates;
 *   - memory-specific constraint categories;
 *   - memory constraint lists;
 *   - memory constraint modifiers;
 *   - memory constraint extension points;
 *   - memory-specific constraint syntax;
 *   - source-level memory constraint metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - qualified names;
 *   - general expressions;
 *   - generic constraint expressions;
 *   - requirements;
 *   - capabilities;
 *   - resource declarations;
 *   - target declarations;
 *   - hardware discovery;
 *   - physical addresses;
 *   - physical memory;
 *   - allocation;
 *   - deallocation;
 *   - ownership checking;
 *   - borrowing;
 *   - lifetime analysis;
 *   - shared-memory semantics;
 *   - distributed-memory semantics;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - quantum semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution.
 *
 * ============================================================================
 * CANONICAL DEPENDENCY BOUNDARIES
 * ============================================================================
 *
 * Generic memory syntax is owned by:
 *
 *     grammar/memory/memory.g4
 *
 * Generic constraint syntax is owned by:
 *
 *     grammar/core/constraints.g4
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Names and qualified names are owned by the canonical name subsystem.
 *
 * This file specializes those contracts.
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     constraintExpression
 *     memoryPlace
 *     memorySpace
 *     memoryRequirement
 *     memoryPreference
 *     memoryHint
 *
 * when those rules are already provided by the imported canonical grammars.
 *
 * ============================================================================
 * IMPORTANT DESIGN RULE
 * ============================================================================
 *
 * A memory constraint is NOT a memory requirement.
 *
 * Requirement:
 *
 *     describes something the program needs.
 *
 * Constraint:
 *
 *     describes a condition that a valid realization must satisfy.
 *
 * Preference:
 *
 *     describes a preferred valid realization.
 *
 * Hint:
 *
 *     provides optional implementation guidance.
 *
 * Capability:
 *
 *     describes what an environment can provide.
 *
 * Resource:
 *
 *     describes a realizable computational resource/property.
 *
 * Target:
 *
 *     describes a compilation or execution context.
 *
 * These concepts MUST remain semantically distinct.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar MUST support:
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
 * Memory constraints therefore describe portable semantic conditions rather
 * than temporary properties of one machine.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_MEMORY
 *     MAX_HEAP
 *     MAX_STACK
 *     MAX_ADDRESS
 *     MAX_ALIGNMENT
 *     MAX_MEMORY_SPACES
 *     MAX_MEMORY_REGIONS
 *     MAX_MEMORY_OBJECTS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_CORES
 *
 * It MUST NOT encode:
 *
 *     CPU 0
 *     GPU 0
 *     NUMA node 0
 *     memory bank 0
 *     device memory 0
 *     fixed physical addresses
 *     fixed topology
 *     fixed cache hierarchy
 *     fixed memory capacity
 *
 * Such properties belong to resource discovery, target descriptions,
 * capabilities, compilation context, scheduling, placement, or runtime
 * policy.
 *
 * ============================================================================
 * OPEN-WORLD MEMORY CONSTRAINT MODEL
 * ============================================================================
 *
 * The language must remain extensible as memory technologies evolve.
 *
 * The grammar therefore permits semantic names such as:
 *
 *     memory::persistent
 *     memory::distributed
 *     memory::remote
 *     memory::shared
 *     memory::device
 *     memory::unified
 *     memory::nonvolatile
 *     memory::coherent
 *     memory::transactional
 *     memory::secure
 *     memory::encrypted
 *     future::memory::technology
 *
 * without requiring a grammar modification for every new technology.
 *
 * ============================================================================
 * CONSTRAINT SUBJECT MODEL
 * ============================================================================
 *
 * A memory constraint may apply to:
 *
 *     - a memory place;
 *     - a memory space;
 *     - a memory region;
 *     - a memory operation;
 *     - a memory resource property;
 *     - a memory capability;
 *     - a memory semantic name;
 *     - a generic semantic subject.
 *
 * The parser records the subject.
 *
 * Semantic analysis determines whether the subject is valid.
 *
 * ============================================================================
 * NO PHYSICAL ADDRESS MODEL
 * ============================================================================
 *
 * This file MUST NOT introduce syntax such as:
 *
 *     address 0x1000
 *     address 0x2000
 *     bank 0
 *     node 0
 *     device 1
 *
 * unless an explicitly defined future language feature makes a physical
 * address itself part of the program's semantic meaning.
 *
 * Even in that case, physical addressing must be represented through the
 * hardware/resource/address-space subsystem rather than by this file.
 *
 * ============================================================================
 * NO RESOURCE ALLOCATION
 * ============================================================================
 *
 * A constraint such as:
 *
 *     memory capacity >= required
 *
 * does NOT allocate memory.
 *
 * It only describes a condition that a valid realization must satisfy.
 *
 * Allocation belongs to:
 *
 *     grammar/memory/allocation.g4
 *
 * and downstream allocation semantics.
 *
 * ============================================================================
 * NO PLACEMENT
 * ============================================================================
 *
 * A constraint such as:
 *
 *     memory locality == preferred
 *
 * does not select a NUMA node, GPU, QPU, accelerator, machine, or cluster
 * location.
 *
 * Placement belongs to the resource/hardware/execution systems.
 *
 * ============================================================================
 * NO SCHEDULING
 * ============================================================================
 *
 * Constraints may express timing or latency conditions.
 *
 * They do not schedule operations.
 *
 * For example:
 *
 *     latency <= requirement
 *
 * is a condition.
 *
 * It does not create a schedule.
 *
 * ============================================================================
 * NO DISTRIBUTED MEMORY OWNERSHIP
 * ============================================================================
 *
 * Distributed-memory semantics remain owned by:
 *
 *     grammar/memory/distributed-memory.g4
 *
 * This file may express constraints concerning distributed memory, but it
 * does not define:
 *
 *     migration;
 *     replication;
 *     sharding;
 *     partitioning;
 *     remote execution;
 *     network routing.
 *
 * Those concepts remain in their appropriate domain grammars.
 *
 * ============================================================================
 * NO SHARED MEMORY OWNERSHIP
 * ============================================================================
 *
 * Shared-memory semantics remain owned by:
 *
 *     grammar/memory/shared-memory.g4
 *
 * This file may express constraints such as consistency or visibility
 * conditions, but does not define sharing operations.
 *
 * ============================================================================
 * GENERIC CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * The canonical constraint grammar remains authoritative for:
 *
 *     constraintExpression
 *     constraintOrExpression
 *     constraintAndExpression
 *     constraintNotExpression
 *     constraintPredicate
 *     constraintReference
 *     whereConstraintClause
 *     constraintTypeBound
 *
 * This grammar reuses those structures rather than reproducing them.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expressions remain authoritative.
 *
 * A memory constraint may contain expressions such as:
 *
 *     required_capacity
 *     available_capacity
 *     requested_alignment
 *     latency_budget
 *     bandwidth_requirement
 *
 * but this grammar does not define arithmetic or expression precedence.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `memoryConstraintConstruct` is the stable public rule for composed grammar
 * integration.
 *
 * A root memory grammar may use:
 *
 *     memoryConstraintConstruct
 *
 * where memory-specific constraint syntax is accepted.
 *
 * ============================================================================
 */

parser grammar MemoryConstraints;

options {
    tokenVocab = ZamaniLexer;
}

import Memory, Constraints, Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is intentionally narrow.
 *
 * It allows the composed memory grammar to distinguish a memory-specific
 * constraint construct from ordinary memory operations.
 *
 * ============================================================================
 */

memoryConstraintConstruct
    : memoryConstraintClause
    | memoryConstraintSet
    | memoryConstraintAssertion
    ;


/*
 * ============================================================================
 * CONSTRAINT CLAUSE
 * ============================================================================
 *
 * A clause attaches one or more constraints to a memory semantic object.
 *
 * The generic constraint expression remains owned by Constraints.g4.
 *
 * ============================================================================
 */

memoryConstraintClause
    : memoryConstraintIntroducer
      memoryConstraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINT INTRODUCER
 * ============================================================================
 *
 * `where` is the canonical language-level constraint introducer.
 *
 * The lexer already reserves WHERE.
 *
 * This avoids creating another competing keyword such as:
 *
 *     memory_constraint
 *     memconstraint
 *     constraint_memory
 *
 * ============================================================================
 */

memoryConstraintIntroducer
    : WHERE
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * A memory constraint expression is deliberately based on the canonical
 * constraint-expression system.
 *
 * This rule adds memory-specific subject/predicate forms while preserving
 * generic boolean composition through the canonical constraint grammar.
 *
 * ============================================================================
 */

memoryConstraintExpression
    : memoryConstraintPredicate
    | memoryConstraintGroup
    | memoryConstraintNegation
    | memoryConstraintExpression AND memoryConstraintExpression
    | memoryConstraintExpression OR memoryConstraintExpression
    ;


/*
 * ============================================================================
 * PREDICATE
 * ============================================================================
 *
 * A predicate expresses a relation involving a memory subject.
 *
 * ============================================================================
 */

memoryConstraintPredicate
    : memoryConstraintSubject memoryConstraintRelation memoryConstraintValue
    ;


/*
 * ============================================================================
 * GROUP
 * ============================================================================
 */

memoryConstraintGroup
    : LPAREN memoryConstraintExpression RPAREN
    ;


/*
 * ============================================================================
 * NEGATION
 * ============================================================================
 *
 * Negation is syntactic.
 *
 * Its semantic interpretation remains downstream.
 *
 * ============================================================================
 */

memoryConstraintNegation
    : NOT memoryConstraintExpression
    ;


/*
 * ============================================================================
 * SUBJECT
 * ============================================================================
 *
 * A subject identifies the memory-domain property to which a constraint
 * applies.
 *
 * ============================================================================
 */

memoryConstraintSubject
    : memoryConstraintMemorySubject
    | memoryConstraintResourceSubject
    | memoryConstraintCapabilitySubject
    | memoryConstraintSemanticSubject
    ;


/*
 * ============================================================================
 * MEMORY SUBJECT
 * ============================================================================
 *
 * Reuses the canonical memory-place and memory-space rules.
 *
 * ============================================================================
 */

memoryConstraintMemorySubject
    : memoryPlace
    | memorySpace
    ;


/*
 * ============================================================================
 * RESOURCE SUBJECT
 * ============================================================================
 *
 * Resource identity is represented as an open qualified name.
 *
 * Examples:
 *
 *     resource::memory::capacity
 *     resource::memory::bandwidth
 *     resource::memory::latency
 *     resource::memory::energy
 *     resource::memory::reliability
 *
 * The grammar does not enumerate these names.
 *
 * ============================================================================
 */

memoryConstraintResourceSubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * CAPABILITY SUBJECT
 * ============================================================================
 *
 * Capability names remain open-world.
 *
 * Examples:
 *
 *     capability::memory::atomic
 *     capability::memory::coherent
 *     capability::memory::persistent
 *     capability::memory::remote_access
 *
 * ============================================================================
 */

memoryConstraintCapabilitySubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * GENERIC SEMANTIC SUBJECT
 * ============================================================================
 *
 * This permits future memory-related domains without modifying this grammar.
 *
 * ============================================================================
 */

memoryConstraintSemanticSubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * RELATION
 * ============================================================================
 *
 * The canonical lexer owns the actual operator spellings.
 *
 * Assignment is intentionally excluded.
 *
 * ============================================================================
 */

memoryConstraintRelation
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * VALUE
 * ============================================================================
 *
 * Constraint values are general expressions.
 *
 * The expression grammar remains authoritative.
 *
 * This is critical for POCO-REAF because values such as:
 *
 *     required_capacity
 *     available_capacity
 *     symbolic_limit
 *     runtime_property
 *
 * must not be reduced to fixed literals.
 *
 * ============================================================================
 */

memoryConstraintValue
    : expression
    ;


/*
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * Open-world semantic names.
 *
 * There is no fixed maximum qualification depth.
 *
 * Examples:
 *
 *     memory
 *     memory::capacity
 *     resource::memory::capacity
 *     capability::memory::atomic
 *     future::memory::property
 *
 * ============================================================================
 */

memoryConstraintQualifiedName
    : IDENTIFIER
    | IDENTIFIER DOUBLE_COLON memoryConstraintQualifiedNameTail
    ;


memoryConstraintQualifiedNameTail
    : IDENTIFIER
    | IDENTIFIER DOUBLE_COLON memoryConstraintQualifiedNameTail
    ;


/*
 * ============================================================================
 * CONSTRAINT SET
 * ============================================================================
 *
 * A set allows multiple memory constraints to be expressed together.
 *
 * ============================================================================
 */

memoryConstraintSet
    : memoryConstraintSetIntroducer
      LBRACKET
      memoryConstraintExpressionList?
      RBRACKET
      SEMICOLON
    ;


memoryConstraintSetIntroducer
    : WHERE
    ;


memoryConstraintExpressionList
    : memoryConstraintExpression
      (COMMA memoryConstraintExpression)*
    ;


/*
 * ============================================================================
 * ASSERTION FORM
 * ============================================================================
 *
 * This form provides a direct memory constraint assertion without introducing
 * a new reserved keyword.
 *
 * Example conceptual form:
 *
 *     where memory::capacity >= required;
 *
 * ============================================================================
 */

memoryConstraintAssertion
    : WHERE
      memoryConstraintPredicate
      SEMICOLON
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT CATEGORIES
 * ============================================================================
 *
 * The following rules classify memory constraints semantically.
 *
 * They intentionally use open names rather than hard-coded hardware models.
 *
 * ============================================================================
 */

memoryConstraintCategory
    : memoryCapacityConstraint
    | memoryAlignmentConstraint
    | memoryAccessConstraint
    | memoryVisibilityConstraint
    | memoryConsistencyConstraint
    | memoryOrderingConstraint
    | memoryLocalityConstraint
    | memoryResidencyConstraint
    | memoryPersistenceConstraint
    | memoryDurabilityConstraint
    | memoryBandwidthConstraint
    | memoryLatencyConstraint
    | memoryReliabilityConstraint
    | memorySecurityConstraint
    | memoryAtomicityConstraint
    | memoryAddressabilityConstraint
    | memoryPortabilityConstraint
    | memoryCustomConstraint
    ;


/*
 * ============================================================================
 * CAPACITY
 * ============================================================================
 */

memoryCapacityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * ALIGNMENT
 * ============================================================================
 */

memoryAlignmentConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * ACCESS
 * ============================================================================
 */

memoryAccessConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 */

memoryVisibilityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * CONSISTENCY
 * ============================================================================
 */

memoryConsistencyConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 */

memoryOrderingConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * LOCALITY
 * ============================================================================
 */

memoryLocalityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * RESIDENCY
 * ============================================================================
 */

memoryResidencyConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * PERSISTENCE
 * ============================================================================
 */

memoryPersistenceConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * DURABILITY
 * ============================================================================
 */

memoryDurabilityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * BANDWIDTH
 * ============================================================================
 */

memoryBandwidthConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * LATENCY
 * ============================================================================
 */

memoryLatencyConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * RELIABILITY
 * ============================================================================
 */

memoryReliabilityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 */

memorySecurityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * ATOMICITY
 * ============================================================================
 */

memoryAtomicityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * ADDRESSABILITY
 * ============================================================================
 */

memoryAddressabilityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 */

memoryPortabilityConstraint
    : memoryConstraintCategoryName memoryConstraintRelation expression
    ;


/*
 * ============================================================================
 * CUSTOM / FUTURE CONSTRAINT
 * ============================================================================
 *
 * Future memory properties must not require grammar changes.
 * ============================================================================
 */

memoryCustomConstraint
    : memoryConstraintQualifiedName
      memoryConstraintRelation
      expression
    ;


/*
 * ============================================================================
 * CATEGORY NAME
 * ============================================================================
 *
 * Category names are semantic identifiers.
 *
 * This prevents the grammar from becoming a closed enumeration of memory
 * technologies.
 * ============================================================================
 */

memoryConstraintCategoryName
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT MODIFIER
 * ============================================================================
 *
 * Modifiers describe the interpretation scope of a memory constraint.
 *
 * ============================================================================
 */

memoryConstraintModifier
    : memoryConstraintScopeModifier
    | memoryConstraintStrengthModifier
    | memoryConstraintPortabilityModifier
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 *
 * Examples:
 *
 *     local
 *     region
 *     process
 *     task
 *     distributed
 *     global
 *
 * These are semantic names, not machine identifiers.
 * ============================================================================
 */

memoryConstraintScopeModifier
    : memoryConstraintModifierName
    ;


memoryConstraintStrengthModifier
    : memoryConstraintModifierName
    ;


memoryConstraintPortabilityModifier
    : memoryConstraintModifierName
    ;


memoryConstraintModifierName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT LIST
 * ============================================================================
 *
 * There is intentionally no fixed maximum number of constraints.
 * ============================================================================
 */

memoryConstraintList
    : memoryConstraintExpression
      (COMMA memoryConstraintExpression)*
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT BLOCK
 * ============================================================================
 *
 * A block groups constraints while preserving source ordering.
 *
 * ============================================================================
 */

memoryConstraintBlock
    : LBRACKET
      memoryConstraintList?
      RBRACKET
    ;


/*
 * ============================================================================
 * SEMANTIC EXTENSION
 * ============================================================================
 *
 * Future memory systems can introduce extension constraints without changing
 * the foundational memory grammar.
 *
 * Example conceptual forms:
 *
 *     future::memory::constraint(...)
 *
 * No fixed extension namespace is required.
 * ============================================================================
 */

memoryConstraintExtension
    : memoryConstraintQualifiedName
      LPAREN
      memoryConstraintArgumentList?
      RPAREN
    ;


memoryConstraintArgumentList
    : memoryConstraintArgument
      (COMMA memoryConstraintArgument)*
    ;


memoryConstraintArgument
    : expression
    | memoryConstraintNamedArgument
    ;


memoryConstraintNamedArgument
    : IDENTIFIER ASSIGN expression
    ;


/*
 * ============================================================================
 * MEMORY RESOURCE CONSTRAINT
 * ============================================================================
 *
 * This rule explicitly separates a resource property from the physical
 * resource that eventually satisfies it.
 *
 * ============================================================================
 */

memoryResourceConstraint
    : memoryResourceConstraintSubject
      memoryConstraintRelation
      expression
    ;


memoryResourceConstraintSubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * MEMORY CAPABILITY CONSTRAINT
 * ============================================================================
 */

memoryCapabilityConstraint
    : memoryCapabilityConstraintSubject
      memoryConstraintRelation
      expression
    ;


memoryCapabilityConstraintSubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * MEMORY TARGET CONSTRAINT
 * ============================================================================
 *
 * Target identity remains symbolic.
 *
 * This rule MUST NOT accept physical device identifiers as a special case.
 * ============================================================================
 */

memoryTargetConstraint
    : memoryTargetConstraintSubject
      memoryConstraintRelation
      expression
    ;


memoryTargetConstraintSubject
    : memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * MEMORY PORTABILITY CONSTRAINT
 * ============================================================================
 *
 * Portability is semantic intent.
 *
 * It does not select a target.
 * ============================================================================
 */

memoryPortabilityConstraintExpression
    : memoryConstraintQualifiedName
      memoryConstraintRelation
      expression
    ;


/*
 * ============================================================================
 * MEMORY SCALABILITY CONSTRAINT
 * ============================================================================
 *
 * A scalability constraint can describe a symbolic relationship involving
 * resource-dependent quantities.
 *
 * No fixed machine scale is embedded here.
 * ============================================================================
 */

memoryScalabilityConstraint
    : memoryConstraintQualifiedName
      memoryConstraintRelation
      expression
    ;


/*
 * ============================================================================
 * MEMORY CONSTRAINT METADATA
 * ============================================================================
 *
 * Metadata is preserved for AST/tooling consumers.
 *
 * It does not alter semantic satisfiability by itself.
 * ============================================================================
 */

memoryConstraintMetadata
    : AT memoryConstraintQualifiedName
    ;


/*
 * ============================================================================
 * HARD-CODING SAFETY CONTRACT
 * ============================================================================
 *
 * The following MUST remain absent from this grammar:
 *
 *     MAX_MEMORY
 *     MAX_HEAP_SIZE
 *     MAX_STACK_SIZE
 *     MAX_ADDRESS_WIDTH
 *     MAX_ALIGNMENT
 *     MAX_REGIONS
 *     MAX_MEMORY_SPACES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *
 * The following MUST also remain absent:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     bank0
 *
 * The grammar is scalable because repetition is represented with:
 *
 *     *
 *     +
 *
 * and symbolic expressions rather than finite enumerations.
 *
 * ============================================================================
 * DETERMINISM
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
 *     - no environment-dependent behavior;
 *     - no random behavior.
 *
 * Parsing is therefore deterministic with respect to the token stream.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a memory constraint MUST NOT:
 *
 *     allocate memory;
 *     inspect memory;
 *     access physical addresses;
 *     query devices;
 *     query hardware;
 *     query a network;
 *     contact a runtime;
 *     execute a constraint;
 *     select a backend.
 *
 * All such behavior belongs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     - source span;
 *     - subject;
 *     - relation;
 *     - value expression;
 *     - grouping;
 *     - negation;
 *     - boolean composition;
 *     - qualification;
 *     - metadata;
 *     - source ordering.
 *
 * Conceptually:
 *
 *     MemoryConstraint
 *     {
 *         subject,
 *         relation,
 *         value,
 *         source_span
 *     }
 *
 *     MemoryConstraintGroup
 *     {
 *         constraints,
 *         source_span
 *     }
 *
 *     MemoryConstraintExtension
 *     {
 *         name,
 *         arguments,
 *         source_span
 *     }
 *
 * The exact Rust AST types remain owned by the frontend AST subsystem.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the subject resolves;
 *     - whether the value is valid;
 *     - whether operand types are compatible;
 *     - whether the relation is meaningful;
 *     - whether the constraint is satisfiable;
 *     - whether the constraint conflicts with another constraint;
 *     - whether a referenced capability exists;
 *     - whether a referenced resource exists;
 *     - whether the target can satisfy the constraint;
 *     - whether the constraint is portable;
 *     - whether the constraint requires runtime evaluation;
 *     - whether the constraint can be proven statically.
 *
 * Possible semantic outcomes include:
 *
 *     SATISFIED
 *     UNSATISFIED
 *     UNKNOWN
 *     CONDITIONAL
 *
 * or the repository's canonical equivalent.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Memory constraints must lower into canonical semantic constraint metadata.
 *
 * They MUST NOT lower directly into:
 *
 *     CPU instructions;
 *     GPU instructions;
 *     FPGA placement;
 *     ASIC implementation;
 *     quantum gates;
 *     quantum::ir operations;
 *     network packets;
 *     runtime allocator calls.
 *
 * Target-specific lowering happens only after:
 *
 *     parsing
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic representation
 *       ->
 *     resource/capability analysis
 *       ->
 *     target lowering.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-related memory constraints may reference semantic properties such as:
 *
 *     memory::state_storage
 *     memory::measurement_buffer
 *     memory::classical_feedback
 *     resource::memory::capacity
 *
 * but this grammar MUST NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     quantum circuits;
 *     physical qubits;
 *     QPU topology;
 *     calibration;
 *     QEC;
 *     ZQN.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented memory constraints may reference:
 *
 *     hardware::memory_interface
 *     hardware::memory::latency
 *     hardware::memory::bandwidth
 *     capability::memory::atomic
 *
 * but this grammar does not define:
 *
 *     buses;
 *     pins;
 *     registers;
 *     clocks;
 *     physical addresses;
 *     synthesis;
 *     placement;
 *     routing.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed-memory constraints may reference semantic properties such as:
 *
 *     memory::distribution
 *     memory::replication
 *     memory::consistency
 *     memory::locality
 *     memory::remote_access
 *
 * but distributed execution, node selection, routing, replication algorithms,
 * migration, and fault tolerance remain outside this grammar.
 *
 * ============================================================================
 * SHARED-MEMORY INTEGRATION
 * ============================================================================
 *
 * Shared-memory constraints may describe:
 *
 *     visibility;
 *     consistency;
 *     atomicity;
 *     ordering;
 *     locality;
 *     access mode.
 *
 * Sharing operations themselves remain owned by shared-memory grammar.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis consumes the semantic representation of constraints.
 *
 * It determines whether available resources satisfy them.
 *
 * The grammar MUST NOT query resources itself.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Latency, ordering, bandwidth, and synchronization constraints may become
 * scheduler inputs after semantic analysis.
 *
 * The grammar does not schedule anything.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may use constraints to eliminate invalid transformations.
 *
 * Constraints themselves MUST remain part of semantic provenance.
 *
 * An optimization pass MUST NOT silently delete a constraint merely because
 * the current target happens to satisfy it.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Constraints may be:
 *
 *     statically proven;
 *     dynamically checked;
 *     negotiated against capabilities;
 *     evaluated during deployment;
 *     evaluated at runtime.
 *
 * The grammar does not determine which mechanism is used.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Constraint source information must survive lowering sufficiently to answer:
 *
 *     Which source constraint produced this semantic requirement?
 *
 * This supports:
 *
 *     diagnostics;
 *     reproducibility;
 *     debugging;
 *     verification;
 *     POCO-REAF portability analysis;
 *     compilation provenance.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing valid generic constraint syntax must remain valid.
 *
 * This grammar must not introduce a conflicting interpretation of:
 *
 *     =
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *
 * Assignment remains distinct from equality.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is no grammar-level limit on:
 *
 *     - number of memory constraints;
 *     - number of constraint groups;
 *     - number of boolean terms;
 *     - number of nested groups;
 *     - number of qualified-name components;
 *     - number of memory spaces;
 *     - number of memory regions;
 *     - number of memory objects;
 *     - number of devices;
 *     - number of nodes;
 *     - amount of memory;
 *     - number of qubits;
 *     - number of processors.
 *
 * Practical limits belong to:
 *
 *     parser/runtime resources;
 *     compiler resource policies;
 *     semantic analysis;
 *     target capabilities;
 *     deployment resources.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests for this grammar MUST include:
 *
 * POSITIVE:
 *
 *     where memory::capacity >= required_capacity;
 *     where memory::latency <= latency_budget;
 *     where resource::memory::bandwidth >= required_bandwidth;
 *     where capability::memory::atomic == true;
 *     where memory::persistent == true;
 *
 * BOOLEAN:
 *
 *     where (
 *         memory::capacity >= required_capacity
 *         and memory::latency <= latency_budget
 *     );
 *
 *     where (
 *         memory::locality == preferred
 *         or memory::distributed == true
 *     );
 *
 * NEGATION:
 *
 *     where not memory::volatile == true;
 *
 * SYMBOLIC:
 *
 *     where memory::capacity >= requested_capacity;
 *
 * OPEN WORLD:
 *
 *     where future::memory::new_property == required;
 *
 * QUALIFIED:
 *
 *     where resource::memory::domain::capacity >= requirement;
 *
 * CROSS-DOMAIN:
 *
 *     classical + memory;
 *     quantum + memory;
 *     quantum + classical + memory;
 *     distributed + memory;
 *     HDL + memory;
 *     hardware + memory;
 *     AI + memory;
 *
 * NEGATIVE:
 *
 *     assignment used as equality;
 *     malformed relational expressions;
 *     missing operands;
 *     malformed qualified names;
 *     unterminated groups;
 *     malformed constraint lists;
 *     invalid separator placement.
 *
 * BOUNDARY:
 *
 *     deeply nested constraint groups;
 *     very long qualified names;
 *     large constraint lists;
 *     symbolic resource expressions;
 *     empty optional lists where permitted.
 *
 * DETERMINISM:
 *
 *     identical token streams produce identical parse structures.
 *
 * ROUND TRIP:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     serializer/printer
 *       ->
 *     parser
 *
 * must preserve intended constraint semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Every future modification to this file must classify any discovered limit as:
 *
 *     1. genuine language semantic requirement;
 *     2. target-specific requirement;
 *     3. resource constraint;
 *     4. implementation limitation;
 *     5. accidental hard-coding;
 *     6. test-only limitation;
 *     7. documentation-only limitation.
 *
 * Accidental hard-coding MUST be removed.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It compiles with the repository's ANTLR toolchain.
 *
 *     [ ] It uses the canonical ZamaniLexer.
 *
 *     [ ] It imports the canonical Memory grammar.
 *
 *     [ ] It imports/reuses the canonical constraint system.
 *
 *     [ ] It reuses the canonical expression system.
 *
 *     [ ] It introduces no competing identifier grammar.
 *
 *     [ ] It introduces no competing expression grammar.
 *
 *     [ ] It introduces no competing generic constraint language.
 *
 *     [ ] It contains no machine-size limits.
 *
 *     [ ] It contains no physical device identifiers.
 *
 *     [ ] It contains no physical memory addresses.
 *
 *     [ ] It contains no target-specific allocation logic.
 *
 *     [ ] It contains no runtime actions.
 *
 *     [ ] It contains no embedded Rust.
 *
 *     [ ] It requires no unsafe Rust.
 *
 *     [ ] It preserves source structure and ordering.
 *
 *     [ ] It supports open-world future memory properties.
 *
 *     [ ] It preserves POCO-REAF semantics.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass.
 *
 *     [ ] Hard-coding audit passes.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */