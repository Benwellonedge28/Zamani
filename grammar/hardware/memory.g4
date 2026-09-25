/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/memory.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwareMemoryParser
 *
 * STATUS
 * ------
 * CANONICAL HARDWARE-MEMORY CONTRACT GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL HARDWARE MEMORY CONTRACT.
 *
 * It is the bridge between:
 *
 *     generic memory semantics
 *
 * and:
 *
 *     abstract hardware capabilities/resources.
 *
 * It does NOT replace:
 *
 *     grammar/memory/memory.g4
 *
 * which remains the canonical generic memory-domain foundation.
 *
 * This grammar describes hardware-facing memory intent such as:
 *
 *     capacity
 *     quantity
 *     bandwidth
 *     latency
 *     address-space requirements
 *     capability requirements
 *     resource requirements
 *     portability
 *     scalability
 *     performance
 *     energy
 *     power
 *     reliability
 *     resilience
 *     availability
 *     constraints
 *     preferences
 *     hints
 *     properties
 *     abstract targets
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     1.97
 *     1.97.1
 *
 * Edition:
 *
 *     Rust 2021
 *
 * Safety:
 *
 *     Generated/runtime Rust must use safe Rust.
 *
 *     No unsafe code is required by this grammar.
 *
 * This grammar contains no embedded Rust actions or predicates.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     generic memory semantics       hardware/resource
 *          |                         capability analysis
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                  canonical semantic model
 *                          |
 *                          v
 *                     canonical IR
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        classical     quantum::ir   HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                 optimization/lowering
 *                          |
 *                 routing/scheduling
 *                          |
 *                   resilience/QEC/ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                  target realization
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hardwareMemoryContract
 *     hardwareMemoryBody
 *     hardwareMemoryClause
 *
 *     hardwareMemoryQuantityClause
 *     hardwareMemoryCapacityClause
 *     hardwareMemoryAvailabilityClause
 *
 *     hardwareMemoryBandwidthClause
 *     hardwareMemoryLatencyClause
 *
 *     hardwareMemoryAddressSpaceClause
 *
 *     hardwareMemoryRequirementClause
 *     hardwareMemoryConstraintClause
 *     hardwareMemoryPreferenceClause
 *     hardwareMemoryHintClause
 *
 *     hardwareMemoryCapabilityClause
 *     hardwareMemoryResourceClause
 *     hardwareMemoryTargetClause
 *
 *     hardwareMemoryPortabilityClause
 *     hardwareMemoryScalabilityClause
 *
 *     hardwareMemoryPerformanceClause
 *     hardwareMemoryEnergyClause
 *     hardwareMemoryPowerClause
 *     hardwareMemoryReliabilityClause
 *     hardwareMemoryResilienceClause
 *     hardwareMemoryCostClause
 *
 *     hardwareMemoryPropertyClause
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     generic memory allocation
 *     generic memory deallocation
 *     ownership
 *     borrowing
 *     lifetime semantics
 *     generic memory regions
 *     generic memory spaces
 *     generic address-space semantics
 *     generic references
 *     generic pointers
 *     generic types
 *     expression precedence
 *     identifiers
 *     qualified names
 *     lexical tokens
 *
 * It also does NOT own:
 *
 *     physical memory discovery
 *     physical memory allocation
 *     memory-bank selection
 *     NUMA placement
 *     cache selection
 *     page-table construction
 *     physical addresses
 *     device IDs
 *     GPU IDs
 *     QPU IDs
 *     CPU IDs
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     runtime allocation
 *     device drivers
 *
 * ============================================================================
 * EXISTING REPOSITORY BOUNDARY
 * ============================================================================
 *
 * Generic memory is already owned by:
 *
 *     grammar/memory/memory.g4
 *
 * Existing specialized memory grammars include:
 *
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/deallocation.g4
 *     grammar/memory/regions.g4
 *     grammar/memory/address-spaces.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/accelerator-memory.g4
 *     grammar/memory/quantum-memory.g4
 *     grammar/memory/memory-capabilities.g4
 *
 * This file MUST NOT redefine those generic concepts.
 *
 * Instead:
 *
 *     memory/memory.g4
 *          |
 *          v
 *     generic memory semantics
 *          |
 *          v
 *     hardware/memory.g4
 *          |
 *          v
 *     hardware memory contract
 *
 * ============================================================================
 * HARDWARE TYPE INTEGRATION
 * ============================================================================
 *
 * Hardware memory types are already defined by:
 *
 *     grammar/types/hardware.g4
 *
 * In particular:
 *
 *     hardwareMemoryType
 *
 * remains the authoritative type-level representation.
 *
 * This grammar does NOT create another Memory type system.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Universal resource semantics are owned by:
 *
 *     grammar/resources/resources.g4
 *
 * Hardware-specific resource syntax is owned by:
 *
 *     grammar/hardware/resources.g4
 *
 * This file may reference resources but must not redefine the universal
 * resource model.
 *
 * Therefore:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * remain semantically distinct.
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * Hardware target intent is owned by:
 *
 *     grammar/hardware/targets.g4
 *
 * A target reference in this file is symbolic.
 *
 * It does NOT select:
 *
 *     a physical machine
 *     a physical memory controller
 *     a memory bank
 *     a NUMA node
 *     a GPU
 *     a QPU
 *     a device ID
 *     a physical address
 *
 * ============================================================================
 * OPEN-WORLD MEMORY MODEL
 * ============================================================================
 *
 * Memory technologies are NOT exhaustively enumerated here.
 *
 * Examples of semantic names that may be represented downstream include:
 *
 *     memory::local
 *     memory::shared
 *     memory::distributed
 *     memory::persistent
 *     memory::remote
 *     memory::device
 *     memory::accelerator
 *     memory::quantum
 *     memory::logical
 *     memory::future::technology
 *
 * The grammar does not need to be changed merely because a new memory
 * technology appears.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware memory syntax describes:
 *
 *     WHAT memory behavior is required
 *     WHAT capacity is required
 *     WHAT capability is required
 *     WHAT performance property matters
 *     WHAT constraints must hold
 *     WHAT scaling behavior is expected
 *     WHAT portability properties are required
 *
 * It does NOT prescribe:
 *
 *     WHICH physical memory
 *     WHICH memory bank
 *     WHICH NUMA node
 *     WHICH cache
 *     WHICH physical page
 *     WHICH memory controller
 *     WHICH device
 *     WHICH address
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT encode universal limits such as:
 *
 *     MAX_MEMORY
 *     MAX_MEMORY_SIZE
 *     MAX_MEMORY_BANKS
 *     MAX_MEMORY_REGIONS
 *     MAX_MEMORY_SPACES
 *     MAX_ALLOCATIONS
 *     MAX_BUFFERS
 *     MAX_ADDRESS_WIDTH
 *     MAX_POINTER_WIDTH
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *     MAX_QUANTUM_MEMORY
 *
 * Quantities and capacities are expressions.
 *
 * Therefore all of these are syntactically representable:
 *
 *     capacity = required_memory;
 *     capacity = problem_size * element_size;
 *     quantity = batch_size;
 *     bandwidth >= required_bandwidth;
 *     latency <= allowed_latency;
 *
 * The actual finite limit is determined downstream by:
 *
 *     target capabilities
 *     resource availability
 *     compiler resources
 *     runtime resources
 *     deployment constraints
 *     physical resources
 *
 * These limits are NOT language limits.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTION
 * ============================================================================
 *
 * The following are different semantic concepts:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     availability
 *     capacity
 *     target
 *     resource
 *
 * For example:
 *
 *     requires capacity >= required_memory;
 *
 * is a requirement.
 *
 *     capacity = available_memory;
 *
 * is a property/value supplied by a contract.
 *
 *     prefer bandwidth >= required_bandwidth;
 *
 * is a preference.
 *
 *     hint locality = "near";
 *
 * is advisory.
 *
 * None of these selects a physical memory device.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * All values and predicates use the canonical Zamani expression hierarchy
 * supplied by the composed parser.
 *
 * This file MUST NOT redefine:
 *
 *     arithmetic
 *     comparison
 *     logical operators
 *     calls
 *     indexing
 *     member access
 *     literals
 *     assignment
 *     precedence
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Generic names are supplied by the canonical core/name grammar.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The parser must preserve source locations for:
 *
 *     declaration
 *     clause
 *     property
 *     requirement
 *     expression
 *     resource reference
 *     capability reference
 *     target reference
 *
 * Source spans are consumed by:
 *
 *     diagnostics
 *     IDE/LSP tooling
 *     formatter
 *     semantic analysis
 *     compatibility tooling
 *     provenance
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax information that maps to the existing
 * domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     hardwareMemoryContract
 *         ->
 *     MemoryContract / ResourceContract semantic category
 *
 * depending on the existing AST model.
 *
 * It MUST preserve:
 *
 *     declaration name
 *     optional type
 *     clauses
 *     clause kind
 *     expressions
 *     symbolic names
 *     resource references
 *     capability references
 *     target references
 *     source spans
 *
 * It MUST NOT create:
 *
 *     PhysicalMemoryNode
 *     MemoryBankNode
 *     NUMANode
 *     GPUAllocationNode
 *     QPUAllocationNode
 *     PhysicalAddressNode
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     type checking
 *     expression checking
 *     unit/dimension checking
 *     resource resolution
 *     capability resolution
 *     target compatibility
 *     satisfiability
 *     portability analysis
 *     scalability analysis
 *     memory-space validation
 *     address-space validation
 *     ownership/lifetime interaction
 *
 * The parser does not determine whether a memory requirement can actually
 * be satisfied.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create a hardware-memory IR.
 *
 * The semantic contract lowers into the repository's existing canonical
 * semantic/IR boundaries.
 *
 * Memory intent may contribute to:
 *
 *     classical IR
 *     quantum::ir where quantum semantics require memory interaction
 *     HDL/hardware representations
 *     distributed representations
 *
 * but this grammar must never create a second quantum IR.
 *
 * ============================================================================
 * DOWNSTREAM INTEGRATION
 * ============================================================================
 *
 *     hardwareMemoryContract
 *              |
 *              v
 *        domain-neutral AST
 *              |
 *              v
 *       semantic analysis
 *              |
 *      +-------+---------+
 *      |                 |
 *      v                 v
 * memory analysis   resource/capability
 *                        analysis
 *      |                 |
 *      +-------+---------+
 *              |
 *              v
 *       canonical semantic model
 *              |
 *              v
 *       compiler / optimizer
 *              |
 *      +-------+---------+
 *      |       |         |
 *      v       v         v
 *    memory  placement  scheduling
 *      |       |         |
 *      +-------+---------+
 *              |
 *              v
 *             HAL
 *              |
 *              v
 *       target realization
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source
 *     language version
 *     grammar
 *     lexer
 *
 * It must NOT depend on:
 *
 *     available RAM
 *     CPU count
 *     GPU count
 *     QPU count
 *     machine topology
 *     runtime state
 *     network state
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     environment inspection
 *     hardware discovery
 *     device allocation
 *     runtime execution
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareMemoryParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This rule is the sole hardware-memory contract entry point.
 *
 * `hardware.g4` delegates its memory contract production here.
 */

hardwareMemoryContract
    : K_MEMORY
      hardwareMemoryName?
      hardwareMemoryTypeAnnotation?
      hardwareMemoryBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. MEMORY NAME
 * ============================================================================
 *
 * The name is symbolic.
 *
 * It is not a physical memory identifier.
 */

hardwareMemoryName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 3. MEMORY TYPE
 * ============================================================================
 *
 * Hardware memory type syntax remains owned by the type grammar.
 *
 * This local wrapper only provides the declaration-level association.
 */

hardwareMemoryTypeAnnotation
    : COLON
      hardwareMemoryType
    ;


/*
 * ============================================================================
 * 4. MEMORY BODY
 * ============================================================================
 */

hardwareMemoryBody
    : LBRACE
      hardwareMemoryClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. CLAUSE DISPATCH
 * ============================================================================
 */

hardwareMemoryClause
    : hardwareMemoryQuantityClause
    | hardwareMemoryCapacityClause
    | hardwareMemoryAvailabilityClause

    | hardwareMemoryBandwidthClause
    | hardwareMemoryLatencyClause

    | hardwareMemoryAddressSpaceClause

    | hardwareMemoryRequirementClause
    | hardwareMemoryConstraintClause
    | hardwareMemoryPreferenceClause
    | hardwareMemoryHintClause

    | hardwareMemoryCapabilityClause
    | hardwareMemoryResourceClause
    | hardwareMemoryTargetClause

    | hardwareMemoryPortabilityClause
    | hardwareMemoryScalabilityClause

    | hardwareMemoryPerformanceClause
    | hardwareMemoryEnergyClause
    | hardwareMemoryPowerClause
    | hardwareMemoryReliabilityClause
    | hardwareMemoryResilienceClause
    | hardwareMemoryCostClause

    | hardwareMemoryPropertyClause
    ;


/*
 * ============================================================================
 * 6. QUANTITY
 * ============================================================================
 *
 * Quantity is an arbitrary expression.
 *
 * It may be:
 *
 *     a literal
 *     a variable
 *     a generic parameter
 *     a symbolic expression
 *     a function result
 *     a runtime-dependent expression
 *
 * No finite range is imposed.
 */

hardwareMemoryQuantityClause
    : K_QUANTITY
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. CAPACITY
 * ============================================================================
 *
 * Capacity is a contract value.
 *
 * It is NOT a universal compiler maximum.
 */

hardwareMemoryCapacityClause
    : K_CAPACITY
      hardwareMemoryRelation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. AVAILABILITY
 * ============================================================================
 */

hardwareMemoryAvailabilityClause
    : K_AVAILABILITY
      hardwareMemoryRelation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. BANDWIDTH
 * ============================================================================
 */

hardwareMemoryBandwidthClause
    : K_BANDWIDTH
      hardwareMemoryRelation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. LATENCY
 * ============================================================================
 */

hardwareMemoryLatencyClause
    : K_LATENCY
      hardwareMemoryRelation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ADDRESS SPACE
 * ============================================================================
 *
 * Address-space meaning remains owned by:
 *
 *     grammar/memory/address-spaces.g4
 *
 * This clause expresses an association or requirement only.
 *
 * It does not describe a physical address.
 */

hardwareMemoryAddressSpaceClause
    : K_ADDRESS_SPACE
      hardwareMemoryRelation
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires capacity >= required_memory;
 *     requires bandwidth >= required_bandwidth;
 *     requires capability("memory.addressable");
 *     requires resource memory;
 *
 * Satisfiability is semantic, not syntactic.
 */

hardwareMemoryRequirementClause
    : K_REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CONSTRAINT
 * ============================================================================
 */

hardwareMemoryConstraintClause
    : K_CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PREFERENCE
 * ============================================================================
 */

hardwareMemoryPreferenceClause
    : K_PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. HINT
 * ============================================================================
 */

hardwareMemoryHintClause
    : K_HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. CAPABILITY
 * ============================================================================
 *
 * Capability names remain open-world.
 *
 * Examples:
 *
 *     capability = memory.addressable;
 *     capability = capability("memory.coherent");
 *     capability = capability("memory.persistent");
 *
 * The grammar does not enumerate capabilities.
 */

hardwareMemoryCapabilityClause
    : K_CAPABILITY
      (
          ASSIGN
      )?
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. RESOURCE
 * ============================================================================
 *
 * A resource reference is symbolic.
 *
 * It does not allocate anything.
 */

hardwareMemoryResourceClause
    : K_RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. TARGET
 * ============================================================================
 *
 * The target is an abstract target expression.
 *
 * Physical target selection remains downstream.
 */

hardwareMemoryTargetClause
    : K_TARGET
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PORTABILITY
 * ============================================================================
 */

hardwareMemoryPortabilityClause
    : K_PORTABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. SCALABILITY
 * ============================================================================
 *
 * Scaling is represented symbolically.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *     scalability = workload_size * parallelism;
 *     scalability = available_capacity;
 *     scalability = elastic;
 *
 * No maximum is encoded.
 */

hardwareMemoryScalabilityClause
    : K_SCALABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PERFORMANCE
 * ============================================================================
 */

hardwareMemoryPerformanceClause
    : K_PERFORMANCE
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. ENERGY
 * ============================================================================
 */

hardwareMemoryEnergyClause
    : K_ENERGY
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. POWER
 * ============================================================================
 */

hardwareMemoryPowerClause
    : K_POWER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. RELIABILITY
 * ============================================================================
 */

hardwareMemoryReliabilityClause
    : K_RELIABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. RESILIENCE
 * ============================================================================
 *
 * Resilience state/outcome semantics remain downstream.
 *
 * This grammar merely preserves the expression.
 */

hardwareMemoryResilienceClause
    : K_RESILIENCE
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. COST
 * ============================================================================
 */

hardwareMemoryCostClause
    : K_COST
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. PROPERTY
 * ============================================================================
 *
 * Generic properties provide forward compatibility without introducing a
 * permanent keyword for every future memory technology.
 */

hardwareMemoryPropertyClause
    : K_PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. RELATION
 * ============================================================================
 *
 * The relation is intentionally separate from expression syntax.
 *
 * This prevents this file from redefining expression precedence.
 */

hardwareMemoryRelation
    : EQ
    | NEQ
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 29. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 * [x] Owns hardware-memory contract syntax only.
 * [x] Does not duplicate generic memory semantics.
 * [x] Does not duplicate generic resource semantics.
 * [x] Does not duplicate target declarations.
 * [x] Uses the canonical Zamani lexer.
 * [x] Reuses canonical expression syntax.
 * [x] Reuses canonical qualified-name syntax.
 * [x] Reuses canonical hardware memory type syntax.
 * [x] Has no physical device selection.
 * [x] Has no physical address selection.
 * [x] Has no vendor-specific universal syntax.
 * [x] Has no fixed memory capacity.
 * [x] Has no fixed address width.
 * [x] Has no fixed device count.
 * [x] Has no fixed node count.
 * [x] Has no fixed CPU/GPU/QPU count.
 * [x] Preserves requirement/constraint/preference/hint distinction.
 * [x] Preserves capability/resource/target distinction.
 * [x] Is deterministic.
 * [x] Contains no embedded Rust.
 * [x] Requires no unsafe Rust.
 * [x] Can participate in POCO-REAF.
 *
 * ============================================================================
 */