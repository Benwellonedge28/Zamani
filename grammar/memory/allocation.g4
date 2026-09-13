/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/allocation.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Purpose:
 *     Production source grammar for memory-allocation intent.
 *
 * Architectural role:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Core parser / domain parser
 *          |
 *          v
 *     Memory
 *          |
 *          v
 *     Allocation
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     resource analysis      ownership/lifetime
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *       classical   quantum    hardware
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *              lowering / runtime
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - allocation-operation syntax;
 *   - allocation intent;
 *   - allocation target/place syntax;
 *   - allocation type/reference syntax;
 *   - optional allocation policy syntax;
 *   - optional memory-space syntax;
 *   - allocation requirement syntax;
 *   - allocation constraint syntax;
 *   - allocation preference syntax;
 *   - allocation hint syntax;
 *   - allocation initialization syntax;
 *   - allocation lifetime attachment syntax;
 *   - allocation extension syntax;
 *   - syntactic representation of resizable allocations;
 *   - syntactic representation of reservations;
 *   - syntactic representation of capacity-independent allocation intent.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifiers;
 *   - general expressions;
 *   - general types;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime inference;
 *   - lifetime validation;
 *   - physical memory;
 *   - virtual memory;
 *   - physical addresses;
 *   - pointer widths;
 *   - heap implementation;
 *   - stack implementation;
 *   - allocator algorithms;
 *   - garbage collection;
 *   - reference counting;
 *   - NUMA discovery;
 *   - cache discovery;
 *   - GPU-memory discovery;
 *   - FPGA memory discovery;
 *   - quantum-resource allocation;
 *   - qubit allocation;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware selection;
 *   - backend selection;
 *   - runtime allocation;
 *   - resource discovery;
 *   - resource limits;
 *   - canonical classical IR;
 *   - quantum::ir;
 *   - hardware IR;
 *   - HDL IR.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * No machine capacity is encoded here.
 *
 * This grammar MUST NOT define:
 *
 *     MAX_ALLOCATION
 *     MAX_ALLOCATIONS
 *     MAX_MEMORY
 *     MAX_HEAP
 *     MAX_STACK
 *     MAX_REGION
 *     MAX_ADDRESS
 *     MAX_POINTER
 *     MAX_ELEMENTS
 *     MAX_BYTES
 *     MAX_DEVICES
 *     MAX_MEMORY_SPACES
 *
 * It MUST NOT assume:
 *
 *     a fixed pointer width;
 *     a fixed address width;
 *     a fixed heap;
 *     a fixed stack;
 *     a fixed memory bank;
 *     a fixed NUMA topology;
 *     a fixed accelerator count;
 *     a fixed quantum-device size;
 *     a fixed qubit count;
 *     a fixed node count.
 *
 * Resource availability is a downstream concern.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file declares no lexer rules.
 *
 * Existing lexical tokens are reused where available.
 *
 * Open-world memory/allocation names are represented by identifiers instead
 * of an ever-growing list of reserved allocator keywords.
 *
 * ============================================================================
 * MEMORY FOUNDATION INTEGRATION
 * ============================================================================
 *
 * This grammar extends the common memory foundation:
 *
 *     grammar/memory/memory.g4
 *
 * The foundation owns the generic memory-domain vocabulary and establishes
 * the common memory AST/semantic boundary.
 *
 * Allocation specializes that foundation.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * General type syntax remains owned by the canonical type grammar.
 *
 * Allocation MUST consume the repository's canonical type-expression rule
 * rather than redefining types.
 *
 * This prevents allocation syntax from creating a competing type system.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Allocation sizes, predicates, initialization expressions, and policies
 * consume the canonical expression grammar.
 *
 * This grammar does not redefine:
 *
 *     arithmetic;
 *     indexing;
 *     calls;
 *     comparisons;
 *     logical expressions;
 *     conditional expressions;
 *     literals.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     allocate(T)
 *     allocate(T, count)
 *     allocate_in(memory::device, T, count)
 *     reserve(T, count)
 *
 * describe source intent.
 *
 * They do NOT imply:
 *
 *     malloc;
 *     calloc;
 *     stack allocation;
 *     a particular heap;
 *     a particular GPU;
 *     a particular QPU;
 *     a particular memory bank;
 *     a particular address;
 *     a particular allocator.
 *
 * The semantic layer determines the legal realization.
 *
 * ============================================================================
 */

parser grammar Allocation;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable integration rule consumed by the memory domain.
 * ============================================================================
 */

allocationConstruct
    : allocationStatement
    | allocationExpression
    ;

/*
 * ============================================================================
 * STATEMENTS
 * ============================================================================
 */

allocationStatement
    : allocateStatement
    | reserveStatement
    | resizeStatement
    | releaseReservationStatement
    ;

/*
 * ============================================================================
 * ALLOCATION
 * ============================================================================
 *
 * General form:
 *
 *     allocate <allocationType>
 *
 * Optional clauses express intent rather than implementation.
 *
 * Examples:
 *
 *     allocate T;
 *     allocate T count expression;
 *     allocate T in memory::device;
 *     allocate T lifetime 'a;
 *     allocate T with allocationPolicy;
 *
 * No physical resource is selected by this grammar.
 * ============================================================================
 */

allocateStatement
    : allocationKeyword
      allocationType
      allocationCountClause?
      allocationPlacementClause?
      allocationLifetimeClause?
      allocationInitializationClause?
      allocationPolicyClause?
      allocationRequirementsClause?
      allocationConstraintsClause?
      allocationPreferencesClause?
      allocationHintsClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * ALLOCATION EXPRESSION
 * ============================================================================
 */

allocationExpression
    : allocationKeyword
      allocationType
      allocationCountClause?
      allocationPlacementClause?
      allocationLifetimeClause?
      allocationInitializationClause?
      allocationPolicyClause?
      allocationRequirementsClause?
      allocationConstraintsClause?
      allocationPreferencesClause?
      allocationHintsClause?
    ;

/*
 * ============================================================================
 * ALLOCATION KEYWORD
 * ============================================================================
 *
 * `new` is an existing Zamani lexical construct.
 *
 * The open `memory::allocate(...)` family is also supported through the
 * operation form below so that allocation can evolve without reserving
 * arbitrary future global keywords.
 * ============================================================================
 */

allocationKeyword
    : NEW
    | allocationOperationKeyword
    ;

allocationOperationKeyword
    : memoryAllocationName
    ;

/*
 * ============================================================================
 * ALLOCATION TYPE
 * ============================================================================
 *
 * The canonical type system remains authoritative.
 *
 * `typeExpression` is expected to be supplied by the composed parser.
 * ============================================================================
 */

allocationType
    : typeExpression
    ;

/*
 * ============================================================================
 * COUNT / EXTENT
 * ============================================================================
 *
 * Counts are expressions, not fixed-size integer literals.
 *
 * This permits:
 *
 *     runtime-determined counts;
 *     generic counts;
 *     symbolic counts;
 *     dynamically available resources;
 *     target-dependent realization.
 *
 * The grammar does not impose a maximum.
 * ============================================================================
 */

allocationCountClause
    : allocationCountKeyword allocationExtent
    ;

allocationCountKeyword
    : COUNT
    | SIZE
    | LENGTH
    ;

allocationExtent
    : expression
    | STAR
    ;

/*
 * ============================================================================
 * PLACEMENT / MEMORY SPACE
 * ============================================================================
 *
 * This is semantic placement intent.
 *
 * It does not select a physical address or hardware device.
 * ============================================================================
 */

allocationPlacementClause
    : IN memoryPlace
    ;

memoryPlace
    : qualifiedMemoryName
    | identifierMemoryPlace
    ;

/*
 * ============================================================================
 * MEMORY NAMESPACE
 * ============================================================================
 *
 * Qualified names remain open-world.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::device
 *     memory::persistent
 *     memory::remote
 *     memory::future_domain
 *
 * No fixed list of memory technologies is encoded.
 * ============================================================================
 */

qualifiedMemoryName
    : IDENTIFIER DOUBLE_COLON memoryPathTail
    ;

memoryPathTail
    : IDENTIFIER
    | memoryPathTail DOUBLE_COLON IDENTIFIER
    ;

identifierMemoryPlace
    : IDENTIFIER
    ;

/*
 * ============================================================================
 * LIFETIME
 * ============================================================================
 *
 * Lifetime syntax is preserved for semantic analysis.
 *
 * The grammar does not infer lifetime relationships.
 * ============================================================================
 */

allocationLifetimeClause
    : lifetimeKeyword lifetimeName
    ;

lifetimeKeyword
    : LIFETIME
    ;

lifetimeName
    : APOSTROPHE IDENTIFIER
    ;

/*
 * ============================================================================
 * INITIALIZATION
 * ============================================================================
 */

allocationInitializationClause
    : initializationKeyword expression
    ;

initializationKeyword
    : WITH
    | INITIALIZED
    ;

/*
 * ============================================================================
 * RESERVATION
 * ============================================================================
 *
 * Reservation expresses a requirement/intention to make resources available.
 *
 * It does not reserve anything while parsing.
 * ============================================================================
 */

reserveStatement
    : reserveKeyword
      allocationType
      allocationCountClause?
      allocationPlacementClause?
      allocationLifetimeClause?
      allocationRequirementsClause?
      allocationConstraintsClause?
      allocationPreferencesClause?
      allocationHintsClause?
      SEMICOLON
    ;

reserveKeyword
    : RESERVE
    ;

/*
 * ============================================================================
 * RESIZE / GROW
 * ============================================================================
 *
 * Resizing is semantic intent.
 *
 * The runtime may realize this by:
 *
 *     reallocation;
 *     virtual remapping;
 *     segmented storage;
 *     distributed storage;
 *     another implementation.
 *
 * None is selected here.
 * ============================================================================
 */

resizeStatement
    : resizeKeyword allocationTarget resizeExtentClause? SEMICOLON
    ;

resizeKeyword
    : RESIZE
    | GROW
    ;

allocationTarget
    : memoryPlace
    ;

resizeExtentClause
    : allocationCountKeyword allocationExtent
    | TO allocationExtent
    ;

/*
 * ============================================================================
 * RESERVATION RELEASE
 * ============================================================================
 *
 * This does not necessarily mean deallocation.
 *
 * It releases a reservation/requirement where semantically permitted.
 * ============================================================================
 */

releaseReservationStatement
    : releaseReservationKeyword allocationTarget SEMICOLON
    ;

releaseReservationKeyword
    : RELEASE
    ;

/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Policies are symbolic semantic requests.
 *
 * Examples:
 *
 *     with policy_name
 *     with policy::name
 *
 * The grammar does not define allocator algorithms.
 * ============================================================================
 */

allocationPolicyClause
    : WITH POLICY allocationPolicy
    ;

allocationPolicy
    : qualifiedAllocationName
    ;

qualifiedAllocationName
    : IDENTIFIER
    | IDENTIFIER DOUBLE_COLON allocationNameTail
    ;

allocationNameTail
    : IDENTIFIER
    | allocationNameTail DOUBLE_COLON IDENTIFIER
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 * ============================================================================
 */

allocationRequirementsClause
    : REQUIRES allocationRequirementList
    ;

allocationRequirementList
    : allocationRequirement (COMMA allocationRequirement)*
    ;

allocationRequirement
    : expression
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict legal implementation choices.
 * ============================================================================
 */

allocationConstraintsClause
    : CONSTRAINED_BY allocationConstraintList
    ;

allocationConstraintList
    : allocationConstraint (COMMA allocationConstraint)*
    ;

allocationConstraint
    : expression
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory desired properties.
 * ============================================================================
 */

allocationPreferencesClause
    : PREFER allocationPreferenceList
    ;

allocationPreferenceList
    : allocationPreference (COMMA allocationPreference)*
    ;

allocationPreference
    : expression
    ;

/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints must never change program semantics.
 * ============================================================================
 */

allocationHintsClause
    : HINT allocationHintList
    ;

allocationHintList
    : allocationHint (COMMA allocationHint)*
    ;

allocationHint
    : expression
    ;

/*
 * ============================================================================
 * OPEN-WORLD MEMORY OPERATIONS
 * ============================================================================
 *
 * These rules allow allocation syntax to participate in the generic memory
 * namespace without making every future memory technology a reserved keyword.
 *
 * Examples:
 *
 *     memory::allocate(...)
 *     memory::allocate_in(...)
 *     memory::reserve(...)
 *     memory::grow(...)
 *
 * The actual operation meaning is resolved semantically.
 * ============================================================================
 */

memoryAllocationName
    : MEMORY DOUBLE_COLON allocationOperationName
    ;

allocationOperationName
    : ALLOCATE
    | ALLOCATE_IN
    | RESERVE
    | GROW
    ;

/*
 * ============================================================================
 * GENERIC MEMORY ALLOCATION INVOCATION
 * ============================================================================
 *
 * This rule is useful when the composed memory grammar represents memory
 * operations uniformly.
 *
 * It intentionally accepts an extensible argument list.
 * ============================================================================
 */

memoryAllocationInvocation
    : memoryAllocationName LPAREN allocationArgumentList? RPAREN
    ;

allocationArgumentList
    : allocationArgument (COMMA allocationArgument)*
    ;

allocationArgument
    : expression
    | allocationNamedArgument
    ;

allocationNamedArgument
    : IDENTIFIER ASSIGN expression
    ;

/*
 * ============================================================================
 * SEMANTICALLY SAFE EXTENSION POINT
 * ============================================================================
 *
 * Domain extensions may use a qualified operation name without modifying
 * this grammar every time a new memory technology appears.
 * ============================================================================
 */

memoryAllocationExtension
    : qualifiedAllocationName LPAREN allocationArgumentList? RPAREN
    ;