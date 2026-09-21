/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/memory-capabilities.g4
 *
 * STATUS
 * ------
 * Production memory-domain capability grammar.
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust Edition 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines MEMORY-SPECIFIC CAPABILITY INTENT.
 *
 * It connects the canonical capability model to the memory domain without
 * creating a second capability language.
 *
 * A memory capability describes a semantic property that a memory
 * environment, memory space, memory region, memory operation, or memory
 * resource may provide or require.
 *
 * Examples of semantic capabilities include:
 *
 *     memory::addressable
 *     memory::atomic
 *     memory::coherent
 *     memory::consistent
 *     memory::persistent
 *     memory::durable
 *     memory::shared
 *     memory::distributed
 *     memory::remote_access
 *     memory::mapped
 *     memory::unified
 *     memory::managed
 *     memory::transactional
 *     memory::encrypted
 *     memory::protected
 *     memory::recoverable
 *
 * These names are EXAMPLES ONLY.
 *
 * This grammar intentionally does NOT enumerate them.
 *
 * Future memory technologies must be expressible through ordinary
 * capability references without requiring this grammar to be rewritten.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Capability identity is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource-side capability relationships are owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * Generic memory syntax is owned by:
 *
 *     grammar/memory/memory.g4
 *
 * Memory constraints are owned by:
 *
 *     grammar/memory/memory-constraints.g4
 *
 * This file ONLY connects those existing contracts for memory-domain use.
 *
 * It MUST NOT redefine:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     expression
 *     memoryPlace
 *     memorySpace
 *     memoryRegion
 *     memoryOperation
 *     resource capability semantics
 *
 * ============================================================================
 * FUNDAMENTAL SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *     What a memory environment can provide.
 *
 * REQUIREMENT
 *     What the program requires.
 *
 * CONSTRAINT
 *     A condition a valid realization must satisfy.
 *
 * PREFERENCE
 *     A preferred but non-mandatory realization.
 *
 * HINT
 *     Advisory implementation information.
 *
 * AVAILABILITY
 *     A condition describing when a capability is available.
 *
 * ASSERTION
 *     A semantic condition that may be verified.
 *
 * IMPLICATION
 *     A semantic relationship between capabilities.
 *
 * EXCLUSION
 *     A semantic incompatibility relationship.
 *
 * RESOURCE
 *     A realizable computational resource/property.
 *
 * NONE of these constructs select a physical machine by themselves.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar MUST support:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Run Forever
 *
 * Memory capabilities therefore express portable semantic intent.
 *
 * They MUST NOT encode:
 *
 *     MAX_MEMORY
 *     MAX_HEAP
 *     MAX_STACK
 *     MAX_ADDRESS_WIDTH
 *     MAX_MEMORY_SPACES
 *     MAX_MEMORY_REGIONS
 *     MAX_MEMORY_OBJECTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *
 * They MUST NOT encode:
 *
 *     GPU 0
 *     CPU 0
 *     NUMA 0
 *     memory-bank 0
 *     device-memory 0
 *     physical-address 0x...
 *
 * Such information belongs downstream to:
 *
 *     resource discovery
 *     target description
 *     capability resolution
 *     placement
 *     routing
 *     scheduling
 *     hardware abstraction
 *     runtime
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability names are intentionally open-ended.
 *
 * Valid examples include:
 *
 *     memory::atomic
 *     memory::persistent
 *     memory::shared
 *     memory::distributed
 *     memory::device
 *     memory::unified
 *     memory::future::technology
 *     vendor::memory::extension
 *     future::memory::new_architecture
 *
 * This file MUST NOT create closed enumerations such as:
 *
 *     memoryCapability
 *         : ATOMIC
 *         | SHARED
 *         | DEVICE
 *         | ...
 *
 * Such an enumeration would make future memory technologies require grammar
 * modification and would violate the open-world architecture.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The same capability identity may be provided by:
 *
 *     embedded memory
 *     CPU memory
 *     GPU memory
 *     FPGA memory
 *     accelerator memory
 *     persistent storage
 *     distributed memory
 *     quantum-classical infrastructure
 *     simulation environments
 *     future architectures
 *
 * The grammar does not decide which realization supplies it.
 *
 * ============================================================================
 * MEMORY DOMAIN BOUNDARY
 * ============================================================================
 *
 * This grammar may associate capability intent with abstract memory subjects:
 *
 *     memory places
 *     memory spaces
 *     memory regions
 *     memory operations
 *     abstract memory resources
 *
 * It MUST NOT resolve those subjects to:
 *
 *     physical addresses
 *     physical devices
 *     physical banks
 *     physical nodes
 *     physical NUMA domains
 *     physical caches
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                            lexer
 *                              |
 *                              v
 *                         parser layer
 *                              |
 *             +----------------+----------------+
 *             |                                 |
 *             v                                 v
 *      core/capabilities                 memory/memory
 *             |                                 |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                   memory-capabilities.g4
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                              v
 *                      semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        capability         resource         memory
 *        resolution         analysis         analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                  canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical IR     quantum::ir      HDL/hardware IR
 *                              |
 *                              v
 *                  optimization / routing /
 *                    scheduling / lowering
 *                              |
 *                              v
 *                           runtime
 *
 * This file has no dependency on runtime implementation.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Memory capabilities may be used by hybrid or quantum-classical programs.
 *
 * Examples:
 *
 *     memory::persistent
 *     memory::coherent
 *     memory::shared
 *     memory::distributed
 *
 * may influence semantic/resource analysis around a quantum computation.
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     calibration
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted construct must preserve enough information for the
 * frontend AST to retain, conceptually:
 *
 *     - source span;
 *     - capability identity;
 *     - optional capability version requirement;
 *     - intent kind;
 *     - optional memory subject;
 *     - optional resource subject;
 *     - optional condition;
 *     - optional property;
 *     - optional value expression;
 *     - relationship operands;
 *     - annotations/metadata where supported by the canonical composition.
 *
 * The parser MUST NOT:
 *
 *     - resolve the capability;
 *     - discover hardware;
 *     - allocate memory;
 *     - select a target;
 *     - validate capability availability;
 *     - perform ownership checking;
 *     - perform borrow checking;
 *     - lower to IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the capability identity is known;
 *     - whether it is supplied by the relevant environment;
 *     - whether its version requirement is satisfiable;
 *     - whether it applies to the memory subject;
 *     - whether it conflicts with another capability;
 *     - whether requirements are satisfiable;
 *     - whether constraints can be met;
 *     - whether preferences can be honored;
 *     - whether hints are usable;
 *     - whether capability relationships are valid;
 *     - whether resource realization exists;
 *     - whether the capability is compatible with ownership/effects/types;
 *     - how the intent maps into the canonical semantic representation.
 *
 * ============================================================================
 * NO RESOURCE ALLOCATION
 * ============================================================================
 *
 * The following:
 *
 *     requires memory capability memory::persistent;
 *
 * means:
 *
 *     "the realization must provide the requested semantic capability."
 *
 * It does NOT mean:
 *
 *     allocate persistent memory now.
 *
 * Allocation remains owned by:
 *
 *     grammar/memory/allocation.g4
 *
 * and downstream resource/runtime systems.
 *
 * ============================================================================
 * NO PLACEMENT
 * ============================================================================
 *
 * This grammar MUST NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     NUMA node
 *     memory bank
 *     device
 *     cluster node
 *
 * A capability describes a property.
 *
 * Resource and hardware systems determine realization.
 *
 * ============================================================================
 * NO FIXED CAPABILITY LIST
 * ============================================================================
 *
 * There is intentionally no finite list of memory capabilities.
 *
 * Repetition is represented structurally using:
 *
 *     *
 *     +
 *
 * and names are represented using the canonical capability-name grammar.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Capabilities:
 *
 *     canonical capability identity and version syntax.
 *
 * Memory:
 *
 *     canonical memory subjects, memory spaces, places, regions, and
 *     memory-domain constructs.
 *
 * ResourceCapabilities:
 *
 *     canonical resource-side capability relationships.
 *
 * Expressions:
 *
 *     canonical expression syntax.
 *
 * This file does not replace any of those grammars.
 * ============================================================================
 */

parser grammar MemoryCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities, Memory, ResourceCapabilities, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable composition entry point for the memory capability domain.
 *
 * Zero or more independent memory-capability constructs are permitted where
 * the surrounding grammar defines a capability section.
 */
memoryCapabilities
    : memoryCapabilityItem*
    ;


/*
 * ============================================================================
 * 2. ITEM DISPATCH
 * ============================================================================
 *
 * Every construct has a distinct syntactic role.
 */
memoryCapabilityItem
    : memoryCapabilityRequirement
    | memoryCapabilityConstraint
    | memoryCapabilityPreference
    | memoryCapabilityHint
    | memoryCapabilityAvailability
    | memoryCapabilityAssertion
    | memoryCapabilityImplication
    | memoryCapabilityExclusion
    | memoryCapabilityComposition
    | memoryCapabilityPropertyAssertion
    ;


/*
 * ============================================================================
 * 3. MEMORY CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires memory capability memory::persistent;
 *
 *     requires memory capability memory::atomic;
 *
 *     requires memory capability memory::remote_access;
 *
 * The capability identity remains open-world.
 */
memoryCapabilityRequirement
    : K_REQUIRES
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause?
      SEMI
    ;


/*
 * ============================================================================
 * 4. MEMORY CAPABILITY CONSTRAINT
 * ============================================================================
 *
 * A constraint is mandatory.
 *
 * It does not allocate or select resources.
 */
memoryCapabilityConstraint
    : K_CONSTRAINT
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause?
      SEMI
    ;


/*
 * ============================================================================
 * 5. MEMORY CAPABILITY PREFERENCE
 * ============================================================================
 *
 * A preference is advisory to realization/optimization and does not change
 * semantic validity when it cannot be honored.
 */
memoryCapabilityPreference
    : K_PREFERENCE
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause?
      SEMI
    ;


/*
 * ============================================================================
 * 6. MEMORY CAPABILITY HINT
 * ============================================================================
 *
 * A hint is advisory and may be ignored by the compiler/runtime.
 */
memoryCapabilityHint
    : K_HINT
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause?
      SEMI
    ;


/*
 * ============================================================================
 * 7. MEMORY CAPABILITY AVAILABILITY
 * ============================================================================
 *
 * Availability is a condition.
 *
 * It does not perform capability discovery.
 *
 * Example:
 *
 *     memory capability memory::remote_access
 *         available when execution_context.supports_remote_memory;
 *
 * The condition is parsed as an expression and interpreted downstream.
 */
memoryCapabilityAvailability
    : K_MEMORY
      K_CAPABILITY
      capabilityReference
      K_AVAILABLE
      memoryCapabilityAvailabilityCondition
      memoryCapabilitySubjectClause?
      SEMI
    ;


memoryCapabilityAvailabilityCondition
    : K_WHEN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 8. MEMORY CAPABILITY ASSERTION
 * ============================================================================
 *
 * Assertions express semantic conditions to be checked downstream.
 */
memoryCapabilityAssertion
    : K_ASSERT
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause?
      SEMI
    ;


/*
 * ============================================================================
 * 9. MEMORY CAPABILITY IMPLICATION
 * ============================================================================
 *
 * Example:
 *
 *     memory capability memory::coherent
 *         implies memory capability memory::consistent;
 *
 * The relationship is semantic.
 *
 * It does not allocate anything.
 */
memoryCapabilityImplication
    : K_MEMORY
      K_CAPABILITY
      capabilityReference
      K_IMPLIES
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 10. MEMORY CAPABILITY EXCLUSION
 * ============================================================================
 *
 * Example:
 *
 *     memory capability memory::volatile
 *         excludes memory capability memory::persistent;
 *
 * Semantic analysis determines whether the relationship is meaningful.
 */
memoryCapabilityExclusion
    : K_MEMORY
      K_CAPABILITY
      capabilityReference
      K_EXCLUDES
      K_MEMORY
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 11. MEMORY CAPABILITY COMPOSITION
 * ============================================================================
 *
 * A composition groups multiple capability conditions without imposing a
 * finite number of members.
 *
 * Example:
 *
 *     memory capability {
 *         memory::addressable,
 *         memory::atomic,
 *         memory::coherent
 *     };
 *
 * This is semantic grouping, not physical allocation.
 */
memoryCapabilityComposition
    : K_MEMORY
      K_CAPABILITY
      LBRACE
      memoryCapabilityReferenceList?
      RBRACE
      SEMI
    ;


memoryCapabilityReferenceList
    : capabilityReference
      (COMMA capabilityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. MEMORY CAPABILITY SUBJECT
 * ============================================================================
 *
 * A capability may be associated with an abstract memory subject.
 *
 * The subject is deliberately not a physical address or device.
 */
memoryCapabilitySubjectClause
    : K_FOR
      memoryCapabilitySubject
    ;


memoryCapabilitySubject
    : memoryPlace
    | memoryCapabilitySubjectPath
    | memoryCapabilityResourceSubject
    ;


/*
 * ============================================================================
 * 13. MEMORY SUBJECT PATH
 * ============================================================================
 *
 * This provides an open-world semantic path for abstract memory resources
 * without redefining the canonical memory-place grammar.
 *
 * Examples:
 *
 *     memory::shared
 *     memory::persistent
 *     memory::distributed
 *     memory::future::technology
 *
 * The meaning is resolved semantically.
 */
memoryCapabilitySubjectPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. MEMORY RESOURCE SUBJECT
 * ============================================================================
 *
 * A resource subject is intentionally abstract.
 *
 * Examples:
 *
 *     resource::memory
 *     resource::memory::capacity
 *     resource::memory::bandwidth
 *
 * No resource is selected here.
 */
memoryCapabilityResourceSubject
    : K_RESOURCE
      qualifiedName
    ;


/*
 * ============================================================================
 * 15. MEMORY CAPABILITY PROPERTY ASSERTION
 * ============================================================================
 *
 * This construct permits an open-world capability to expose a semantic
 * property without enumerating every possible property in the grammar.
 *
 * Example:
 *
 *     memory capability memory::persistent
 *         property durability >= required_durability;
 *
 * The property name and value are semantic data.
 */
memoryCapabilityPropertyAssertion
    : K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilityPropertyClause
      SEMI
    ;


memoryCapabilityPropertyClause
    : memoryCapabilityPropertyKeyword
      qualifiedName
      memoryCapabilityComparisonOperator
      expression
    ;


memoryCapabilityPropertyKeyword
    : K_PROPERTY
    ;


/*
 * ============================================================================
 * 16. COMPARISON OPERATORS
 * ============================================================================
 *
 * Uses the canonical relational token vocabulary.
 *
 * No arithmetic or expression grammar is redefined here.
 */
memoryCapabilityComparisonOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 17. CAPABILITY SUBJECT PROPERTY ASSERTION
 * ============================================================================
 *
 * This permits a capability property to be attached to an explicit memory
 * subject.
 *
 * Example:
 *
 *     memory capability memory::persistent
 *         for persistent_region
 *         property durability >= required_durability;
 *
 * Subject selection remains abstract.
 */
memoryCapabilitySubjectPropertyAssertion
    : K_MEMORY
      K_CAPABILITY
      capabilityReference
      memoryCapabilitySubjectClause
      memoryCapabilityPropertyClause
      SEMI
    ;


/*
 * ============================================================================
 * 18. BOOLEAN CAPABILITY CONDITION
 * ============================================================================
 *
 * Capability conditions may be represented through the canonical expression
 * grammar where the surrounding resource/semantic grammar permits them.
 *
 * This rule is deliberately a thin integration boundary.
 */
memoryCapabilityCondition
    : expression
    ;


/*
 * ============================================================================
 * 19. OPTIONAL CONDITION
 * ============================================================================
 *
 * Kept separate so future composition can extend the syntax without
 * modifying capability identity rules.
 */
memoryCapabilityConditionClause
    : K_WHEN
      memoryCapabilityCondition
    ;


/*
 * ============================================================================
 * 20. INTEGRATION ADAPTER
 * ============================================================================
 *
 * This adapter allows existing resource-capability constructs to participate
 * in memory-domain composition without redefining their grammar.
 *
 * It is intentionally a delegation rule.
 *
 * IMPORTANT:
 *
 * This does not create a second implementation of:
 *
 *     capabilityReference
 *     resourceCapabilityRequirement
 *     resourceCapabilityConstraint
 *     resourceCapabilityPreference
 *     resourceCapabilityHint
 *
 * It merely provides an explicit composition point.
 */
memoryResourceCapabilityConstruct
    : resourceCapabilityItem
    ;


/*
 * ============================================================================
 * 21. GENERAL CAPABILITY REFERENCE ADAPTER
 * ============================================================================
 *
 * This adapter is useful to surrounding memory grammars that need to consume
 * the canonical capability reference without importing or redefining the
 * capability grammar themselves.
 */
memoryCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 22. CAPABILITY LIST
 * ============================================================================
 *
 * Open-ended list with no machine-scale limit.
 */
memoryCapabilityList
    : memoryCapabilityReference
      (COMMA memoryCapabilityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 23. CAPABILITY RELATIONSHIP
 * ============================================================================
 *
 * General relationship adapter for downstream composition.
 *
 * The actual semantics are owned by capability/resource analysis.
 */
memoryCapabilityRelationship
    : memoryCapabilityReference
      memoryCapabilityRelationshipOperator
      memoryCapabilityReference
    ;


memoryCapabilityRelationshipOperator
    : K_IMPLIES
    | K_EXCLUDES
    ;


/*
 * ============================================================================
 * 24. MEMORY CAPABILITY DECLARATION ADAPTER
 * ============================================================================
 *
 * A capability declaration itself remains owned by Capabilities.g4.
 *
 * This adapter deliberately delegates instead of redefining
 * capabilityDeclaration.
 *
 * Example:
 *
 *     capability memory::persistent;
 *
 * Whether that capability is semantically classified as a memory capability
 * is decided downstream.
 */
memoryCapabilityDeclaration
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * 25. SEMANTIC EXAMPLES
 * ============================================================================
 *
 * The following conceptual forms are supported by this grammar:
 *
 *     requires memory capability memory::persistent;
 *
 *     requires memory capability memory::atomic for buffer;
 *
 *     constraint memory capability memory::coherent for region;
 *
 *     prefer memory capability memory::unified for buffer;
 *
 *     hint memory capability memory::managed for buffer;
 *
 *     memory capability memory::remote_access
 *         available when execution_context.supports_remote_memory;
 *
 *     memory capability memory::coherent
 *         implies memory capability memory::consistent;
 *
 *     memory capability memory::volatile
 *         excludes memory capability memory::persistent;
 *
 *     memory capability {
 *         memory::addressable,
 *         memory::atomic,
 *         memory::coherent
 *     };
 *
 *     memory capability memory::persistent
 *         property durability >= required_durability;
 *
 * These are syntax examples only.
 *
 * They do not imply that the named capabilities are predefined.
 *
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no finite capability enumeration;
 *     no finite resource enumeration;
 *     no fixed memory size;
 *     no fixed address width;
 *     no fixed address;
 *     no fixed memory-space count;
 *     no fixed region count;
 *     no fixed allocation count;
 *     no fixed node count;
 *     no fixed device count;
 *     no fixed accelerator count;
 *     no fixed processor count;
 *     no fixed qubit count;
 *     no fixed topology;
 *     no vendor-specific hardware requirement.
 *
 * Any future memory capability must be representable by a canonical
 * capability reference.
 *
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no semantic predicates;
 *     - has no parser actions;
 *     - performs no I/O;
 *     - performs no hardware discovery;
 *     - performs no resource allocation;
 *     - performs no network access;
 *     - performs no random behavior.
 *
 * Given an identical token stream, parsing is deterministic.
 *
 * ============================================================================
 * 28. SECURITY BOUNDARY
 * ============================================================================
 *
 * A memory capability MUST NOT silently grant:
 *
 *     physical memory access;
 *     DMA;
 *     kernel privilege;
 *     device ownership;
 *     unrestricted remote memory access;
 *     secret-memory access;
 *     privileged address-space access.
 *
 * Capability authorization remains a semantic/security concern.
 *
 * ============================================================================
 * 29. COMPATIBILITY
 * ============================================================================
 *
 * Existing capability syntax remains authoritative.
 *
 * Existing resource capability syntax remains authoritative.
 *
 * This file adds a memory-domain composition layer rather than replacing
 * those systems.
 *
 * If a future language revision changes capability syntax, this grammar must
 * consume the versioned canonical capability rules rather than creating a
 * compatibility fork.
 *
 * ============================================================================
 * 30. AST / IR INTEGRATION
 * ============================================================================
 *
 * The parser output must remain source-oriented.
 *
 * Conceptually:
 *
 *     memory capability
 *          |
 *          v
 *     AST capability intent
 *          |
 *          v
 *     semantic capability resolution
 *          |
 *          +--> memory resource model
 *          +--> capability model
 *          +--> type/effect analysis
 *          +--> ownership analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir where quantum semantics are involved
 *          +--> HDL/hardware representation where applicable
 *          |
 *          v
 *     optimization / scheduling / placement
 *          |
 *          v
 *     target realization
 *
 * This file MUST NOT introduce a memory-specific IR.
 *
 * ============================================================================
 * 31. INTEGRATION WITH EXISTING MEMORY FILES
 * ============================================================================
 *
 * memory.g4
 *     Owns generic memory constructs and memory subjects.
 *
 * ownership.g4
 *     Owns ownership syntax.
 *
 * borrowing.g4
 *     Owns borrow syntax.
 *
 * lifetimes.g4
 *     Owns lifetime syntax.
 *
 * allocation.g4
 *     Owns allocation intent.
 *
 * deallocation.g4
 *     Owns release/deallocation intent.
 *
 * shared-memory.g4
 *     Owns shared-memory semantics at the syntax boundary.
 *
 * distributed-memory.g4
 *     Owns distributed-memory semantics at the syntax boundary.
 *
 * memory-constraints.g4
 *     Owns memory-specific constraints.
 *
 * This file supplies capability composition to those domains.
 *
 * None of them should redefine capability identity.
 *
 * ============================================================================
 * 32. INTEGRATION WITH RESOURCE SYSTEM
 * ============================================================================
 *
 * Resource capabilities remain owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * Memory capability requirements may therefore participate in the same
 * semantic requirement graph as:
 *
 *     compute capabilities;
 *     quantum capabilities;
 *     accelerator capabilities;
 *     networking capabilities;
 *     security capabilities;
 *     storage capabilities;
 *     future capabilities.
 *
 * The grammar does not decide how those capabilities are satisfied.
 *
 * ============================================================================
 * 33. INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware capability grammars may describe target-side capabilities.
 *
 * This file describes source-side memory capability intent.
 *
 * The relationship is:
 *
 *     source memory capability
 *              |
 *              v
 *     capability resolution
 *              |
 *              v
 *     target capability set
 *              |
 *              v
 *     resource feasibility
 *              |
 *              v
 *     placement / scheduling / lowering
 *
 * There is no direct grammar dependency on physical hardware.
 *
 * ============================================================================
 * 34. INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum-memory interaction remains semantic.
 *
 * Example:
 *
 *     requires memory capability memory::coherent;
 *
 * may become relevant to a hybrid program containing:
 *
 *     classical state
 *     quantum operations
 *     measurement results
 *     feed-forward
 *
 * The memory grammar does not lower any operation into quantum::ir.
 *
 * ============================================================================
 * 35. INTEGRATION WITH HDL
 * ============================================================================
 *
 * HDL may describe memory structures and interfaces.
 *
 * Memory capability syntax may describe portable requirements such as:
 *
 *     atomic access
 *     persistence
 *     coherence
 *     visibility
 *     bandwidth
 *     latency
 *
 * Actual BRAM/URAM/SRAM/DRAM/cache/register-file realization remains
 * downstream.
 *
 * ============================================================================
 * 36. INTEGRATION WITH DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Memory capability syntax may express:
 *
 *     distributed access
 *     remote access
 *     consistency
 *     replication support
 *     persistence
 *     fault tolerance
 *
 * It MUST NOT select:
 *
 *     node 0
 *     node 1
 *     fixed cluster size
 *     fixed topology
 *
 * ============================================================================
 * 37. INTEGRATION WITH AI / DATA
 * ============================================================================
 *
 * AI/data programs may require memory capabilities such as:
 *
 *     streaming
 *     persistence
 *     shared access
 *     distributed access
 *     accelerator access
 *
 * Capability names remain open-world.
 *
 * No ML framework becomes part of this grammar.
 *
 * ============================================================================
 * 38. TEST CONTRACT
 * ============================================================================
 *
 * Dedicated tests belong under:
 *
 *     grammar/tests/memory/
 *
 * Recommended fixtures:
 *
 *     memory-capabilities-positive.zm
 *     memory-capabilities-negative.zm
 *     memory-capabilities-boundary.zm
 *     memory-capabilities-scalability.zm
 *     memory-capabilities-cross-domain.zm
 *     memory-capabilities-quantum.zm
 *     memory-capabilities-hdl.zm
 *     memory-capabilities-distributed.zm
 *
 * ============================================================================
 * 39. POSITIVE TEST REQUIREMENTS
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     requires memory capability memory::persistent;
 *     requires memory capability memory::atomic;
 *     requires memory capability memory::coherent for buffer;
 *     constraint memory capability memory::shared for region;
 *     prefer memory capability memory::unified for buffer;
 *     hint memory capability memory::managed for buffer;
 *     memory capability memory::remote_access
 *         available when execution_context.supports_remote_memory;
 *     memory capability memory::coherent
 *         implies memory capability memory::consistent;
 *     memory capability memory::volatile
 *         excludes memory capability memory::persistent;
 *     memory capability { ... };
 *     memory capability memory::persistent
 *         property durability >= required_durability;
 *
 * Tests should also use:
 *
 *     vendor::memory::extension
 *     future::memory::new_architecture
 *
 * to prove open-world extensibility.
 *
 * ============================================================================
 * 40. NEGATIVE TEST REQUIREMENTS
 * ============================================================================
 *
 * Syntax rejection MUST cover:
 *
 *     missing capability reference;
 *     missing semicolon;
 *     malformed qualified name;
 *     malformed version requirement;
 *     malformed subject clause;
 *     malformed capability composition;
 *     missing property operator;
 *     malformed property expression;
 *     incomplete implication;
 *     incomplete exclusion;
 *     malformed availability condition.
 *
 * Semantic-invalid examples must be distinguished from syntactically-invalid
 * examples.
 *
 * ============================================================================
 * 41. BOUNDARY TEST REQUIREMENTS
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     deeply qualified capability names;
 *     long capability lists;
 *     long property expressions;
 *     deeply nested expressions;
 *     many independent memory capability clauses;
 *     combinations of memory and resource capability intent.
 *
 * No artificial machine-scale boundary may be encoded as a language rule.
 *
 * ============================================================================
 * 42. SCALABILITY TEST REQUIREMENTS
 * ============================================================================
 *
 * Generated tests should vary:
 *
 *     number of capability references;
 *     number of memory subjects;
 *     number of regions;
 *     number of resource relationships;
 *     expression complexity;
 *     program size.
 *
 * The grammar must not introduce a semantic maximum for any of these.
 *
 * ============================================================================
 * 43. CROSS-DOMAIN TEST REQUIREMENTS
 * ============================================================================
 *
 * At minimum:
 *
 *     classical + memory capability
 *     quantum + memory capability
 *     hybrid + memory capability
 *     HDL + memory capability
 *     hardware + memory capability
 *     distributed + memory capability
 *     AI + memory capability
 *     data + memory capability
 *     networking + memory capability
 *     security + memory capability
 *     effects + memory capability
 *     resources + memory capability
 *
 * must be represented in the conformance suite.
 *
 * ============================================================================
 * 44. DETERMINISM TEST REQUIREMENTS
 * ============================================================================
 *
 * Identical source must produce equivalent parse structure on repeated runs.
 *
 * The grammar contains no semantic predicates or actions, so determinism is
 * expected from the parser/token stream.
 *
 * ============================================================================
 * 45. ROUND-TRIP REQUIREMENTS
 * ============================================================================
 *
 * Where the repository's AST printer/serializer supports these constructs:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve capability intent.
 *
 * Formatting differences are permitted.
 *
 * Semantic changes are not.
 *
 * ============================================================================
 * 46. RUST INTEGRATION
 * ============================================================================
 *
 * This grammar introduces no embedded Rust actions.
 *
 * Consequently:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * remain implementation requirements rather than grammar dependencies.
 *
 * The Rust frontend must parse the same stable language contract.
 *
 * The compiler implementation MUST NOT use unsafe Rust for this feature.
 *
 * ============================================================================
 * 47. PRODUCTION COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] canonical capability identity is reused;
 * [ ] memory subjects are reused;
 * [ ] resource capability semantics are reused;
 * [ ] no duplicate capability language exists;
 * [ ] no closed capability enumeration exists;
 * [ ] no machine limits are encoded;
 * [ ] no hardware IDs are encoded;
 * [ ] no physical addresses are encoded;
 * [ ] no allocation occurs in grammar;
 * [ ] no scheduling occurs in grammar;
 * [ ] no routing occurs in grammar;
 * [ ] no optimization occurs in grammar;
 * [ ] no quantum IR is created;
 * [ ] quantum::ir remains canonical;
 * [ ] AST mapping is defined;
 * [ ] semantic mapping is defined;
 * [ ] IR mapping is defined downstream;
 * [ ] compiler integration is defined;
 * [ ] runtime integration is defined;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] cross-domain tests exist;
 * [ ] deterministic parsing is verified;
 * [ ] compatibility is versioned;
 * [ ] Rust 1.97/1.97.1 compatibility is maintained;
 * [ ] no unsafe implementation is introduced.
 *
 * ============================================================================
 * 48. FINAL INVARIANT
 * ============================================================================
 *
 * Memory capability syntax MUST preserve this boundary:
 *
 *     SOURCE
 *       |
 *       v
 *     MEMORY CAPABILITY INTENT
 *       |
 *       v
 *     CAPABILITY / RESOURCE ANALYSIS
 *       |
 *       v
 *     CANONICAL SEMANTIC MODEL
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *   classical          quantum::ir
 *       |                  |
 *       +------------------+
 *              |
 *              v
 *       optimization
 *              |
 *              v
 *       routing / scheduling
 *              |
 *              v
 *       hardware abstraction
 *              |
 *              v
 *       runtime realization
 *
 * The grammar describes WHAT memory capability is required or expressed.
 *
 * It does not decide HOW that capability is physically realized.
 *
 * This invariant is mandatory for POCO-REAF.
 *
 * ============================================================================
 */