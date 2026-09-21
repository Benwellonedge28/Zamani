/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/accelerator-memory.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production-ready accelerator-memory grammar contract.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe code.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level MEMORY INTENT for accelerator-oriented
 * execution environments.
 *
 * It covers memory semantics that may be relevant to:
 *
 *     - GPUs;
 *     - FPGAs;
 *     - ASIC accelerators;
 *     - tensor accelerators;
 *     - AI/ML accelerators;
 *     - vector accelerators;
 *     - DSP-like accelerators;
 *     - cryptographic accelerators;
 *     - reconfigurable accelerators;
 *     - heterogeneous compute devices;
 *     - quantum-classical accelerators;
 *     - distributed accelerators;
 *     - future accelerator architectures.
 *
 * It deliberately does NOT describe a particular physical accelerator.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Accelerator memory syntax describes:
 *
 *     WHAT memory behavior is required
 *     WHAT memory capabilities are required
 *     WHAT resource properties are required
 *     WHAT placement/locality is preferred
 *     WHAT transfer/coherence/persistence properties matter
 *
 * It does NOT describe:
 *
 *     WHICH GPU
 *     WHICH accelerator
 *     WHICH device ID
 *     WHICH memory bank
 *     WHICH NUMA node
 *     WHICH DMA engine
 *     WHICH physical address
 *     WHICH cache
 *     WHICH fixed bus
 *     WHICH fixed memory controller
 *     WHICH fixed accelerator count
 *
 * Those decisions belong to:
 *
 *     semantic analysis
 *          ↓
 *     resource/capability analysis
 *          ↓
 *     compiler
 *          ↓
 *     placement
 *          ↓
 *     scheduling
 *          ↓
 *     HAL
 *          ↓
 *     runtime
 *
 * ============================================================================
 *
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *     Zamani source
 *          ↓
 *     lexer
 *          ↓
 *     parser
 *          ↓
 *     domain-neutral AST
 *          ↓
 *     semantic analysis
 *          ↓
 *     resource/capability analysis
 *          ↓
 *     canonical semantic representation
 *          ↓
 *     canonical IR
 *          ↓
 *     optimization
 *          ↓
 *     placement / routing / scheduling
 *          ↓
 *     HAL
 *          ↓
 *     target realization
 *
 * This file owns syntax only.
 *
 * ============================================================================
 *
 * RELATIONSHIP TO EXISTING MEMORY GRAMMAR
 * ============================================================================
 *
 * The canonical foundation remains:
 *
 *     grammar/memory/memory.g4
 *
 * This file specializes that foundation.
 *
 * It MUST NOT redefine:
 *
 *     memoryPlace
 *     memorySpace
 *     memoryRegion
 *     memoryOperation
 *     memoryQualifiedName
 *     memoryArgumentList
 *     memoryLifetime
 *     memoryPolicy
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *     expression
 *     typeExpression
 *
 * unless ownership is deliberately transferred by the grammar architecture.
 *
 * ============================================================================
 *
 * RELATIONSHIP TO OTHER FILES
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/lexer/*
 *     grammar/core/*
 *     grammar/types/*
 *     grammar/expressions/*
 *     grammar/memory/memory.g4
 *     grammar/memory/address-spaces.g4
 *     grammar/memory/regions.g4
 *     grammar/resources/*
 *     grammar/hardware/*
 *
 * Downstream:
 *
 *     frontend AST
 *     semantic analysis
 *     resource analysis
 *     capability analysis
 *     memory analysis
 *     accelerator selection
 *     optimization
 *     placement
 *     scheduling
 *     HAL
 *     runtime
 *
 * This file does not directly depend on any backend implementation.
 *
 * ============================================================================
 *
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_DEVICE_MEMORY
 *     MAX_SHARED_MEMORY
 *     MAX_LOCAL_MEMORY
 *     MAX_MEMORY_BANKS
 *     MAX_DMA_ENGINES
 *     MAX_STREAMS
 *     MAX_QUEUES
 *     MAX_ADDRESS_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_WORKGROUP_SIZE
 *     MAX_WORKGROUPS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * Nor may it encode physical identifiers such as:
 *
 *     gpu0
 *     accelerator0
 *     bank0
 *     dma0
 *     numa0
 *
 * as universal language constructs.
 *
 * A program may contain such strings as ordinary identifiers when appropriate.
 *
 * ============================================================================
 *
 * IMPORTANT SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * The grammar preserves the distinction between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     implementation decision
 *
 * For example:
 *
 *     requires capability("accelerator.memory")
 *
 * is different from:
 *
 *     prefer memory space
 *
 * which is different from:
 *
 *     hint memory reuse
 *
 * and all are different from:
 *
 *     map buffer to physical device memory
 *
 * The latter is a target realization decision and does not belong in the
 * portable accelerator-memory grammar.
 *
 * ============================================================================
 *
 * NO SECOND IR
 * ============================================================================
 *
 * This grammar does NOT introduce:
 *
 *     AcceleratorMemoryIR
 *     GpuMemoryIR
 *     DeviceMemoryIR
 *     AcceleratorBufferIR
 *
 * or any equivalent semantic IR.
 *
 * The syntax is lowered through the repository's existing semantic/IR
 * architecture.
 *
 * ============================================================================
 */

parser grammar AcceleratorMemory;

options {
    tokenVocab = ZamaniLexer;
}

import Memory;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The root rule deliberately accepts a collection of accelerator-memory
 * constructs without imposing a fixed ordering or finite count.
 *
 * The surrounding Zamani composition root decides where this construct is
 * legal in a complete source file.
 */
acceleratorMemoryConstruct
    : acceleratorMemoryDeclaration
    | acceleratorMemoryOperation
    | acceleratorMemoryPolicy
    | acceleratorMemoryRequirement
    | acceleratorMemoryConstraint
    | acceleratorMemoryPreference
    | acceleratorMemoryHint
    | acceleratorMemoryCapability
    ;


/*
 * ============================================================================
 * 2. DECLARATION
 * ============================================================================
 *
 * A declaration associates an existing memory place with accelerator-memory
 * intent.
 *
 * The actual value/type declaration remains owned by the declaration/type
 * grammars.
 *
 * Example conceptual forms:
 *
 *     accelerator memory buffer
 *
 *     accelerator memory buffer : Tensor<T, shape>
 *
 *     accelerator memory buffer in memory::device
 *
 * The exact semantic interpretation is downstream.
 */
acceleratorMemoryDeclaration
    : ACCELERATOR MEMORY memoryPlace
      acceleratorMemoryDeclarationClause*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. DECLARATION CLAUSES
 * ============================================================================
 *
 * Each clause carries intent rather than physical realization.
 */
acceleratorMemoryDeclarationClause
    : acceleratorMemorySpaceClause
    | acceleratorMemoryRegionClause
    | acceleratorMemoryLayoutClause
    | acceleratorMemoryAccessClause
    | acceleratorMemoryCoherenceClause
    | acceleratorMemoryPersistenceClause
    | acceleratorMemoryTransferClause
    | acceleratorMemoryCapacityClause
    | acceleratorMemoryAlignmentClause
    | acceleratorMemoryLifetimeClause
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 4. MEMORY SPACE
 * ============================================================================
 *
 * Accelerator memory spaces are symbolic.
 *
 * Examples:
 *
 *     memory::device
 *     memory::shared
 *     memory::local
 *     memory::managed
 *     memory::unified
 *     memory::remote
 *     memory::custom::accelerator
 *
 * No exhaustive hardware-defined list is used.
 */
acceleratorMemorySpaceClause
    : IN memoryQualifiedName
    | MEMORY IN memoryQualifiedName
    | memorySpaceClause
    ;


/*
 * ============================================================================
 * 5. MEMORY REGION
 * ============================================================================
 *
 * Regions remain semantic abstractions.
 */
acceleratorMemoryRegionClause
    : IN memoryRegionReference
    | REGION memoryRegionReference
    ;


/*
 * ============================================================================
 * 6. LAYOUT
 * ============================================================================
 *
 * Layout describes logical organization, not physical bank layout.
 *
 * Examples:
 *
 *     layout contiguous
 *     layout strided(...)
 *     layout tiled(...)
 *     layout blocked(...)
 *     layout custom(...)
 *
 * The layout identifier is open-world.
 */
acceleratorMemoryLayoutClause
    : LAYOUT acceleratorMemoryLayout
    ;


acceleratorMemoryLayout
    : memoryQualifiedName
      acceleratorMemoryLayoutArguments?
    ;


acceleratorMemoryLayoutArguments
    : LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 7. ACCESS MODE
 * ============================================================================
 *
 * Access semantics are symbolic and open-ended.
 *
 * Examples:
 *
 *     read
 *     write
 *     read_write
 *     atomic
 *     streaming
 *     random
 *     sequential
 *     custom
 *
 * The grammar does not prescribe implementation.
 */
acceleratorMemoryAccessClause
    : ACCESS acceleratorMemoryAccessMode
    ;


acceleratorMemoryAccessMode
    : memoryQualifiedName
    | memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 8. COHERENCE
 * ============================================================================
 *
 * Coherence is a semantic property.
 *
 * The grammar does not assume that every accelerator has cache coherence.
 */
acceleratorMemoryCoherenceClause
    : COHERENCE acceleratorMemoryCoherenceMode
    ;


acceleratorMemoryCoherenceMode
    : memoryQualifiedName
    | memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 9. PERSISTENCE
 * ============================================================================
 *
 * Persistence describes required visibility/lifetime semantics.
 *
 * It does not identify a particular storage technology.
 */
acceleratorMemoryPersistenceClause
    : PERSISTENCE acceleratorMemoryPersistenceMode
    ;


acceleratorMemoryPersistenceMode
    : memoryQualifiedName
    | memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 10. TRANSFER
 * ============================================================================
 *
 * Accelerator programs may require movement of data between abstract memory
 * spaces.
 *
 * The grammar expresses the operation, not the transport implementation.
 *
 * No PCIe/CXL/NVLink/AXI/etc. assumption is made here.
 */
acceleratorMemoryTransferClause
    : TRANSFER acceleratorMemoryTransferIntent
    ;


acceleratorMemoryTransferIntent
    : memoryQualifiedName
      acceleratorMemoryTransferArguments?
    ;


acceleratorMemoryTransferArguments
    : LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 11. CAPACITY
 * ============================================================================
 *
 * Capacity is expressed as a symbolic expression.
 *
 * This is intentionally NOT:
 *
 *     capacity: 24GB
 *
 * as a hardware declaration.
 *
 * A source program may legitimately require a quantity of memory for its
 * algorithm. Whether that requirement can be satisfied is a semantic/resource
 * decision.
 */
acceleratorMemoryCapacityClause
    : CAPACITY expression
    ;


/*
 * ============================================================================
 * 12. ALIGNMENT
 * ============================================================================
 *
 * Alignment is a logical/resource requirement.
 *
 * It does not establish a universal physical address width.
 */
acceleratorMemoryAlignmentClause
    : ALIGNMENT expression
    ;


/*
 * ============================================================================
 * 13. LIFETIME
 * ============================================================================
 *
 * Lifetime remains a semantic concept.
 *
 * It does not represent wall-clock time or a fixed number of cycles.
 */
acceleratorMemoryLifetimeClause
    : LIFETIME memoryLifetime
    ;


/*
 * ============================================================================
 * 14. CAPABILITY
 * ============================================================================
 *
 * Capability identity remains symbolic.
 *
 * The canonical resource/capability grammar remains authoritative for
 * capability semantics.
 */
acceleratorMemoryCapabilityClause
    : CAPABILITY memoryQualifiedName
      acceleratorMemoryCapabilityArguments?
    ;


acceleratorMemoryCapabilityArguments
    : LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 15. OPERATION
 * ============================================================================
 *
 * Accelerator memory operations are open-world.
 *
 * Examples:
 *
 *     accelerator::memory::allocate(...)
 *     accelerator::memory::release(...)
 *     accelerator::memory::prefetch(...)
 *     accelerator::memory::migrate(...)
 *     accelerator::memory::flush(...)
 *     accelerator::memory::invalidate(...)
 *     accelerator::memory::fence(...)
 *     accelerator::memory::custom(...)
 *
 * No closed enumeration is used.
 */
acceleratorMemoryOperation
    : acceleratorMemoryOperationName
      LPAREN memoryArgumentList? RPAREN
      SEMICOLON?
    ;


acceleratorMemoryOperationName
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 16. REQUIREMENT
 * ============================================================================
 *
 * This rule is intentionally separate from constraint/preference/hint.
 *
 * It expresses something required for semantic correctness or valid execution.
 */
acceleratorMemoryRequirement
    : REQUIRES acceleratorMemoryRequirementExpression
      SEMICOLON?
    ;


acceleratorMemoryRequirementExpression
    : acceleratorMemoryPredicate
    | expression
    ;


acceleratorMemoryPredicate
    : acceleratorMemoryPredicateAtom
    | LPAREN acceleratorMemoryPredicate RPAREN
    | acceleratorMemoryPredicate AND acceleratorMemoryPredicate
    | acceleratorMemoryPredicate OR acceleratorMemoryPredicate
    | NOT acceleratorMemoryPredicate
    ;


acceleratorMemoryPredicateAtom
    : acceleratorMemoryCapabilityPredicate
    | acceleratorMemoryPropertyPredicate
    ;


acceleratorMemoryCapabilityPredicate
    : CAPABILITY memoryQualifiedName
    ;


acceleratorMemoryPropertyPredicate
    : memoryQualifiedName
      acceleratorMemoryComparisonOperator
      expression
    ;


acceleratorMemoryComparisonOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 17. CONSTRAINT
 * ============================================================================
 *
 * A constraint limits valid realizations but is not automatically equivalent
 * to a preference.
 */
acceleratorMemoryConstraint
    : CONSTRAINT acceleratorMemoryConstraintExpression
      SEMICOLON?
    ;


acceleratorMemoryConstraintExpression
    : acceleratorMemoryPredicate
    | expression
    ;


/*
 * ============================================================================
 * 18. PREFERENCE
 * ============================================================================
 *
 * Preferences influence downstream realization but do not become correctness
 * requirements unless semantic analysis explicitly defines such behavior.
 */
acceleratorMemoryPreference
    : PREFER acceleratorMemoryPreferenceExpression
      SEMICOLON?
    ;


acceleratorMemoryPreferenceExpression
    : acceleratorMemoryPredicate
    | expression
    ;


/*
 * ============================================================================
 * 19. HINT
 * ============================================================================
 *
 * Hints are non-binding metadata.
 *
 * They must never silently become requirements.
 */
acceleratorMemoryHint
    : HINT acceleratorMemoryHintExpression
      SEMICOLON?
    ;


acceleratorMemoryHintExpression
    : acceleratorMemoryOperation
    | acceleratorMemoryPredicate
    | expression
    ;


/*
 * ============================================================================
 * 20. CAPABILITY STATEMENT
 * ============================================================================
 *
 * This is a specialized accelerator-memory capability expression.
 *
 * It intentionally does not duplicate the resource capability grammar's
 * versioning, registry, or semantic resolution.
 */
acceleratorMemoryCapability
    : CAPABILITY memoryQualifiedName
      acceleratorMemoryCapabilityArguments?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. BUFFER VIEW
 * ============================================================================
 *
 * A logical view can describe how a memory place is interpreted without
 * forcing a physical layout.
 *
 * Examples:
 *
 *     view buffer(...)
 *     view tensor(...)
 *     view slice(...)
 *
 * The view name is open-world.
 */
acceleratorMemoryView
    : VIEW memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 22. REGION/SPACE ASSOCIATION
 * ============================================================================
 *
 * This rule allows an accelerator-memory operation to carry both abstract
 * memory-space and region metadata.
 *
 * It does not create a physical placement model.
 */
acceleratorMemoryLocationIntent
    : acceleratorMemorySpaceClause
      acceleratorMemoryRegionClause?
    | acceleratorMemoryRegionClause
      acceleratorMemorySpaceClause?
    ;


/*
 * ============================================================================
 * 23. TRANSFER POLICY
 * ============================================================================
 *
 * A transfer policy is an abstract policy.
 *
 * It may eventually influence asynchronous copies, staging, migration,
 * prefetching, overlap, or other compiler/runtime decisions.
 */
acceleratorMemoryTransferPolicy
    : TRANSFER POLICY memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 24. REUSE POLICY
 * ============================================================================
 *
 * Reuse is a semantic optimization hint/intent.
 */
acceleratorMemoryReusePolicy
    : REUSE memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 25. LOCALITY POLICY
 * ============================================================================
 *
 * Locality remains abstract.
 *
 * It does not mean:
 *
 *     NUMA node N
 *     GPU bank N
 *     cache level N
 *     physical address range
 */
acceleratorMemoryLocalityPolicy
    : LOCALITY memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 26. SYNCHRONIZATION POLICY
 * ============================================================================
 *
 * Synchronization belongs semantically to concurrency/execution/resource
 * analysis. This rule merely preserves memory-specific intent.
 */
acceleratorMemorySynchronizationPolicy
    : SYNCHRONIZATION memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 27. ASYNC MEMORY INTENT
 * ============================================================================
 *
 * Asynchrony is not tied to a particular queue or stream implementation.
 */
acceleratorMemoryAsyncIntent
    : ASYNC acceleratorMemoryOperation
    ;


/*
 * ============================================================================
 * 28. STREAMING INTENT
 * ============================================================================
 *
 * Streaming is semantic behavior, not a fixed hardware stream count.
 */
acceleratorMemoryStreamingIntent
    : STREAMING memoryPlace
      acceleratorMemoryStreamingClause*
    ;


acceleratorMemoryStreamingClause
    : acceleratorMemoryCapacityClause
    | acceleratorMemoryAccessClause
    | acceleratorMemoryTransferClause
    | acceleratorMemoryReusePolicy
    | acceleratorMemoryLocalityPolicy
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 29. TILING INTENT
 * ============================================================================
 *
 * Tile sizes are expressions.
 *
 * They may therefore depend on program parameters, input sizes, compiler
 * specialization, or resource availability.
 *
 * No fixed maximum tile size is encoded.
 */
acceleratorMemoryTilingIntent
    : TILE
      LPAREN memoryArgumentList? RPAREN
      memoryPlace
    ;


/*
 * ============================================================================
 * 30. PARTITIONING INTENT
 * ============================================================================
 *
 * Partitioning is symbolic.
 *
 * The compiler may map it to physical memory structures later.
 */
acceleratorMemoryPartitionIntent
    : PARTITION
      LPAREN memoryArgumentList? RPAREN
      memoryPlace
    ;


/*
 * ============================================================================
 * 31. MIGRATION INTENT
 * ============================================================================
 *
 * Migration between abstract spaces remains target independent.
 */
acceleratorMemoryMigrationIntent
    : MIGRATE
      memoryPlace
      acceleratorMemoryMigrationClause*
    ;


acceleratorMemoryMigrationClause
    : FROM memoryQualifiedName
    | TO memoryQualifiedName
    | acceleratorMemoryTransferClause
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 32. PREFETCH INTENT
 * ============================================================================
 *
 * Prefetching is an optimization/availability intent.
 *
 * It is not guaranteed to be physically implementable.
 */
acceleratorMemoryPrefetchIntent
    : PREFETCH memoryPlace
      acceleratorMemoryPrefetchClause*
    ;


acceleratorMemoryPrefetchClause
    : IN memoryQualifiedName
    | acceleratorMemoryCapacityClause
    | acceleratorMemoryLocalityPolicy
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 33. FLUSH / INVALIDATE INTENT
 * ============================================================================
 *
 * These remain abstract memory semantics.
 */
acceleratorMemoryVisibilityIntent
    : FLUSH memoryPlace
    | INVALIDATE memoryPlace
    ;


/*
 * ============================================================================
 * 34. FENCE INTENT
 * ============================================================================
 *
 * Ordering semantics belong to the semantic/concurrency layer.
 */
acceleratorMemoryFenceIntent
    : FENCE memoryQualifiedName?
    ;


/*
 * ============================================================================
 * 35. COPY INTENT
 * ============================================================================
 *
 * Copying remains a semantic operation.
 *
 * No direction such as host→GPU is hard-coded.
 */
acceleratorMemoryCopyIntent
    : COPY memoryPlace TO memoryPlace
      acceleratorMemoryCopyClause*
    ;


acceleratorMemoryCopyClause
    : acceleratorMemoryCapacityClause
    | acceleratorMemoryTransferClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemorySynchronizationPolicy
    ;


/*
 * ============================================================================
 * 36. MOVE/MIGRATE INTENT
 * ============================================================================
 *
 * A logical move is distinct from physical migration.
 */
acceleratorMemoryMoveIntent
    : MOVE memoryPlace TO memoryPlace
    ;


/*
 * ============================================================================
 * 37. MAP INTENT
 * ============================================================================
 *
 * Mapping is expressed against an abstract memory space.
 *
 * It must not accept a physical address as a universal memory-language
 * primitive.
 */
acceleratorMemoryMapIntent
    : MAP memoryPlace TO memoryQualifiedName
      acceleratorMemoryMapClause*
    ;


acceleratorMemoryMapClause
    : acceleratorMemoryAccessClause
    | acceleratorMemoryCoherenceClause
    | acceleratorMemoryPersistenceClause
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 38. UNMAP INTENT
 * ============================================================================
 */
acceleratorMemoryUnmapIntent
    : UNMAP memoryPlace
    ;


/*
 * ============================================================================
 * 39. ALLOCATION INTEGRATION
 * ============================================================================
 *
 * Allocation semantics remain owned by grammar/memory/allocation.g4.
 *
 * This rule only provides accelerator-specific metadata around that intent.
 */
acceleratorMemoryAllocationIntent
    : ALLOCATE memoryPlace
      acceleratorMemoryAllocationClause*
    ;


acceleratorMemoryAllocationClause
    : acceleratorMemorySpaceClause
    | acceleratorMemoryRegionClause
    | acceleratorMemoryCapacityClause
    | acceleratorMemoryAlignmentClause
    | acceleratorMemoryLayoutClause
    | acceleratorMemoryAccessClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryLifetimeClause
    ;


/*
 * ============================================================================
 * 40. RELEASE INTEGRATION
 * ============================================================================
 *
 * Release/deallocation semantics remain owned by the generic memory grammar.
 */
acceleratorMemoryReleaseIntent
    : RELEASE memoryPlace
    ;


/*
 * ============================================================================
 * 41. DEVICE-INDEPENDENT MEMORY RESOURCE
 * ============================================================================
 *
 * This allows a program to state that an abstract accelerator memory resource
 * is needed without selecting an actual accelerator.
 *
 * Example conceptual form:
 *
 *     accelerator memory resource
 *         capacity >= required
 *         capability(...)
 *
 * The resource model determines what it means.
 */
acceleratorMemoryResourceIntent
    : ACCELERATOR MEMORY RESOURCE
      acceleratorMemoryResourceClause*
      SEMICOLON?
    ;


acceleratorMemoryResourceClause
    : acceleratorMemoryCapacityClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryConstraint
    | acceleratorMemoryPreference
    | acceleratorMemoryHint
    ;


/*
 * ============================================================================
 * 42. ACCESS PATTERN
 * ============================================================================
 *
 * Access patterns remain symbolic.
 */
acceleratorMemoryAccessPattern
    : PATTERN memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 43. SHARING
 * ============================================================================
 *
 * Sharing remains distinct from physical coherence.
 */
acceleratorMemorySharingIntent
    : SHARE memoryPlace
      acceleratorMemorySharingClause*
    ;


acceleratorMemorySharingClause
    : acceleratorMemoryCoherenceClause
    | acceleratorMemorySynchronizationPolicy
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 44. OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * Ownership remains owned by grammar/memory/ownership.g4.
 *
 * This rule is an integration point only.
 */
acceleratorMemoryOwnershipIntent
    : memoryOwnershipQualifier memoryPlace
    ;


/*
 * ============================================================================
 * 45. BORROWING INTEGRATION
 * ============================================================================
 *
 * Borrow semantics remain owned by grammar/memory/borrowing.g4.
 */
acceleratorMemoryBorrowIntent
    : memoryBorrow
    ;


/*
 * ============================================================================
 * 46. FULL ACCELERATOR MEMORY POLICY
 * ============================================================================
 *
 * This rule is useful as a compositional semantic metadata block.
 *
 * The contents remain declarative.
 */
acceleratorMemoryPolicy
    : ACCELERATOR MEMORY POLICY
      LBRACE
      acceleratorMemoryPolicyItem*
      RBRACE
    ;


acceleratorMemoryPolicyItem
    : acceleratorMemorySpaceClause
    | acceleratorMemoryRegionClause
    | acceleratorMemoryLayoutClause
    | acceleratorMemoryAccessClause
    | acceleratorMemoryCoherenceClause
    | acceleratorMemoryPersistenceClause
    | acceleratorMemoryTransferClause
    | acceleratorMemoryCapacityClause
    | acceleratorMemoryAlignmentClause
    | acceleratorMemoryLifetimeClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryReusePolicy
    | acceleratorMemoryLocalityPolicy
    | acceleratorMemorySynchronizationPolicy
    | acceleratorMemoryTransferPolicy
    | acceleratorMemoryAccessPattern
    ;


/*
 * ============================================================================
 * 47. EXTENSION POINT
 * ============================================================================
 *
 * Future accelerator memory domains may use qualified symbolic operations.
 *
 * Examples:
 *
 *     photonic::memory::...
 *     neuromorphic::memory::...
 *     molecular::memory::...
 *     optical::memory::...
 *     custom::accelerator::memory::...
 *
 * The grammar does not need to change merely because a new namespace is
 * introduced.
 */
acceleratorMemoryExtension
    : memoryQualifiedName
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 48. CROSS-DOMAIN MEMORY INTENT
 * ============================================================================
 *
 * Accelerator memory may participate in:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     AI
 *     distributed
 *     networking
 *     security
 *
 * The syntax remains generic so that those domains can consume the same
 * memory model.
 */
acceleratorMemoryCrossDomainIntent
    : memoryQualifiedName
      memoryPlace?
      LPAREN memoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 49. QUANTUM-CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Quantum memory capabilities may be named here without introducing a
 * quantum-memory IR.
 *
 * Examples:
 *
 *     quantum::memory
 *     quantum::state::storage
 *     quantum::logical::state
 *
 * Their actual meaning is resolved by the quantum semantic layer.
 */
acceleratorQuantumMemoryIntent
    : QUANTUM memoryQualifiedName
      acceleratorQuantumMemoryClause*
    ;


acceleratorQuantumMemoryClause
    : acceleratorMemoryCapacityClause
    | acceleratorMemorySpaceClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryRequirement
    | acceleratorMemoryConstraint
    | acceleratorMemoryPreference
    ;


/*
 * ============================================================================
 * 50. AI / TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor layout and accelerator-memory requirements remain compatible with the
 * canonical tensor/type system.
 *
 * No tensor rank or dimension maximum is encoded here.
 */
acceleratorTensorMemoryIntent
    : TENSOR memoryPlace
      acceleratorTensorMemoryClause*
    ;


acceleratorTensorMemoryClause
    : acceleratorMemoryLayoutClause
    | acceleratorMemoryTilingIntent
    | acceleratorMemoryPartitionIntent
    | acceleratorMemoryCapacityClause
    | acceleratorMemoryAccessPattern
    | acceleratorMemoryReusePolicy
    | acceleratorMemoryCapabilityClause
    ;


/*
 * ============================================================================
 * 51. DISTRIBUTED ACCELERATOR MEMORY
 * ============================================================================
 *
 * Distributed memory semantics remain abstract.
 *
 * No fixed node/device count is permitted.
 */
acceleratorDistributedMemoryIntent
    : DISTRIBUTED MEMORY memoryPlace
      acceleratorDistributedMemoryClause*
    ;


acceleratorDistributedMemoryClause
    : acceleratorMemorySpaceClause
    | acceleratorMemoryRegionClause
    | acceleratorMemoryTransferClause
    | acceleratorMemoryLocalityPolicy
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryRequirement
    | acceleratorMemoryConstraint
    | acceleratorMemoryPreference
    ;


/*
 * ============================================================================
 * 52. HDL / HARDWARE CO-DESIGN INTEGRATION
 * ============================================================================
 *
 * Hardware realization is downstream.
 *
 * This syntax may preserve memory-interface intent needed by HDL/co-design.
 */
acceleratorHardwareMemoryIntent
    : HARDWARE MEMORY memoryPlace
      acceleratorHardwareMemoryClause*
    ;


acceleratorHardwareMemoryClause
    : acceleratorMemoryCapacityClause
    | acceleratorMemoryAlignmentClause
    | acceleratorMemoryLayoutClause
    | acceleratorMemoryAccessClause
    | acceleratorMemoryPersistenceClause
    | acceleratorMemoryCapabilityClause
    | acceleratorMemoryConstraint
    | acceleratorMemoryPreference
    ;


/*
 * ============================================================================
 * 53. PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same accelerator-memory construct must remain syntactically valid when
 * the target changes.
 *
 * Example:
 *
 *     requires capability("accelerator.memory")
 *
 * does not select:
 *
 *     GPU 0
 *
 * or:
 *
 *     FPGA bank 3
 *
 * or:
 *
 *     accelerator 17
 *
 * Target selection is downstream.
 */


/*
 * ============================================================================
 * 54. RESOURCE SCALABILITY CONTRACT
 * ============================================================================
 *
 * Quantities are expressions rather than grammar constants.
 *
 * Valid semantic examples include:
 *
 *     capacity input_size * element_size
 *     capacity required_memory
 *     capacity workload.memory_requirement()
 *
 * This grammar does not evaluate them.
 *
 * There is no fixed upper bound in the grammar.
 */


/*
 * ============================================================================
 * 55. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This parser grammar:
 *
 *     performs no I/O;
 *     performs no hardware discovery;
 *     performs no allocation;
 *     performs no runtime calls;
 *     performs no randomness;
 *     contains no semantic actions;
 *     contains no embedded Rust;
 *     contains no target probing.
 *
 * Given the same token stream and imported grammar set, parsing is
 * deterministic.
 */


/*
 * ============================================================================
 * 56. AST CONTRACT
 * ============================================================================
 *
 * The AST must preserve, where applicable:
 *
 *     source span
 *     operation name
 *     qualified namespace
 *     memory place
 *     memory space
 *     memory region
 *     capacity expression
 *     alignment expression
 *     layout expression
 *     access mode
 *     coherence intent
 *     persistence intent
 *     transfer intent
 *     lifetime
 *     capability identity
 *     requirement/constraint/preference/hint classification
 *     extension metadata
 *
 * The AST must remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     physical GPU IDs
 *     physical accelerator IDs
 *     physical addresses
 *     memory-controller handles
 *     DMA handles
 *     cache handles
 *     scheduler state
 *     runtime allocations
 *     calibration state
 *     backend-specific ownership
 */


/*
 * ============================================================================
 * 57. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     whether a memory operation exists;
 *     whether its arguments have valid types;
 *     whether a memory space exists;
 *     whether a region exists;
 *     whether a capability exists;
 *     whether a capability version is compatible;
 *     whether a requirement can be satisfied;
 *     whether a constraint is satisfiable;
 *     whether a preference can be honored;
 *     whether a hint is applicable;
 *     whether ownership rules are respected;
 *     whether concurrency rules are respected;
 *     whether cross-domain interactions are legal.
 *
 * None of those decisions occur in this grammar.
 */


/*
 * ============================================================================
 * 58. IR CONTRACT
 * ============================================================================
 *
 * Accelerator-memory syntax lowers into the repository's canonical semantic
 * representation and then into the appropriate existing IR.
 *
 * It MUST NOT create a second accelerator-memory IR.
 *
 * For quantum programs:
 *
 *     accelerator memory intent
 *             ↓
 *     semantic analysis
 *             ↓
 *     quantum::ir where quantum semantics apply
 *             ↓
 *     optimization / routing / scheduling / QEC / ZQN
 *
 * For classical/accelerator programs:
 *
 *     accelerator memory intent
 *             ↓
 *     semantic model
 *             ↓
 *     classical/accelerator target IR
 *
 * For HDL:
 *
 *     accelerator memory intent
 *             ↓
 *     hardware/HDL semantic representation
 *             ↓
 *     synthesis/backend lowering
 */


/*
 * ============================================================================
 * 59. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capability information is supplied downstream.
 *
 * This grammar must never assume:
 *
 *     GPU memory capacity
 *     shared-memory capacity
 *     local-memory capacity
 *     bank count
 *     cache levels
 *     cache sizes
 *     bus width
 *     address width
 *     DMA count
 *     accelerator count
 *
 * The HAL/backend determines what realization is possible.
 */


/*
 * ============================================================================
 * 60. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Memory transfer and synchronization intent may influence scheduling.
 *
 * Scheduling itself remains outside this grammar.
 *
 * The grammar must not encode:
 *
 *     cycle 0
 *     cycle 1
 *     queue 0
 *     stream 0
 *
 * as universal accelerator-memory semantics.
 */


/*
 * ============================================================================
 * 61. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * The compiler may use:
 *
 *     layout
 *     access pattern
 *     locality
 *     reuse
 *     transfer
 *     persistence
 *     capacity
 *     alignment
 *
 * to optimize memory behavior.
 *
 * The grammar itself performs none of these optimizations.
 */


/*
 * ============================================================================
 * 62. SECURITY
 * ============================================================================
 *
 * Parsing accelerator-memory syntax must not:
 *
 *     open devices;
 *     map physical memory;
 *     execute DMA;
 *     inspect PCI;
 *     access MMIO;
 *     query accelerators;
 *     access operating-system memory APIs;
 *     execute vendor libraries.
 *
 * Such actions are outside parsing and must be controlled by later layers.
 */


/*
 * ============================================================================
 * 63. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Therefore its generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the repository's safe-Rust requirement.
 *
 * The downstream Rust compiler/frontend must contain no unsafe blocks or
 * unsafe functions introduced for this grammar.
 */


/*
 * ============================================================================
 * 64. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic memory syntax remains owned by:
 *
 *     grammar/memory/memory.g4
 *
 * Existing accelerator/GPU syntax in:
 *
 *     grammar/hardware/gpu.g4
 *
 * must not be silently redefined here.
 *
 * If an existing hardware rule already expresses a concept, this grammar
 * should provide a memory-intent representation and semantic integration
 * rather than creating a duplicate hardware syntax authority.
 *
 * Existing resource/capability syntax remains owned by:
 *
 *     grammar/resources/*
 *     grammar/core/*
 *
 * Existing quantum semantics remain owned by:
 *
 *     src/quantum/...
 *
 * with:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 */


/*
 * ============================================================================
 * 65. NEGATIVE ARCHITECTURAL EXAMPLES
 * ============================================================================
 *
 * The following concepts MUST NOT become grammar-level universal constructs:
 *
 *     gpu0.memory
 *     gpu1.memory
 *     bank0
 *     bank1
 *     dma0
 *     cache0
 *     numa0
 *     32-bit-address
 *     64-bit-address
 *     24GB-memory
 *     96KB-shared-memory
 *     32-banks
 *     8-accelerators
 *
 * A source program may contain arbitrary numeric data as program data, but
 * these are not universal accelerator-memory language limits.
 */


/*
 * ============================================================================
 * 66. EXTENSIBILITY
 * ============================================================================
 *
 * Future accelerator technologies can introduce semantic namespaces without
 * requiring this grammar to be rewritten.
 *
 * Examples:
 *
 *     photonic::memory
 *     optical::memory
 *     neuromorphic::memory
 *     molecular::memory
 *     analog::memory
 *     quantum::memory
 *     biological::memory
 *     future::memory
 *
 * The semantic registry/capability system determines their meaning.
 */


/*
 * ============================================================================
 * 67. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It remains a specialized extension of memory.g4.
 *
 * [ ] It does not redefine generic memory semantics.
 *
 * [ ] It composes with the canonical lexer vocabulary.
 *
 * [ ] It composes with canonical expressions.
 *
 * [ ] It composes with canonical types through memory.g4.
 *
 * [ ] It preserves source-level memory intent.
 *
 * [ ] It supports accelerator memory spaces symbolically.
 *
 * [ ] It supports capacity expressions without fixed maxima.
 *
 * [ ] It supports alignment expressions.
 *
 * [ ] It supports layout intent.
 *
 * [ ] It supports access intent.
 *
 * [ ] It supports coherence intent.
 *
 * [ ] It supports persistence intent.
 *
 * [ ] It supports transfer intent.
 *
 * [ ] It supports locality intent.
 *
 * [ ] It supports reuse intent.
 *
 * [ ] It supports asynchronous intent.
 *
 * [ ] It supports migration.
 *
 * [ ] It supports prefetch.
 *
 * [ ] It supports mapping/unmapping.
 *
 * [ ] It supports distributed accelerator memory.
 *
 * [ ] It supports quantum-classical integration.
 *
 * [ ] It supports AI/tensor integration.
 *
 * [ ] It supports HDL/hardware co-design.
 *
 * [ ] Requirement/constraint/preference/hint remain distinct.
 *
 * [ ] No fixed accelerator count exists.
 *
 * [ ] No fixed memory capacity exists.
 *
 * [ ] No fixed bank count exists.
 *
 * [ ] No fixed address width exists.
 *
 * [ ] No physical device IDs are required.
 *
 * [ ] No vendor-specific transport is required.
 *
 * [ ] No second IR is introduced.
 *
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [ ] No QEC implementation is embedded.
 *
 * [ ] No ZQN implementation is embedded.
 *
 * [ ] No routing implementation is embedded.
 *
 * [ ] No scheduling implementation is embedded.
 *
 * [ ] No hardware discovery occurs during parsing.
 *
 * [ ] No I/O occurs during parsing.
 *
 * [ ] No semantic actions occur in the grammar.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97/1.97.1 integration remains safe Rust.
 */


/*
 * ============================================================================
 * 68. TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive examples should cover at minimum:
 *
 *     accelerator memory buffer;
 *     accelerator memory buffer in memory::device;
 *     capacity dynamic_size;
 *     layout contiguous;
 *     layout tiled(...);
 *     access streaming;
 *     coherence shared;
 *     persistence persistent;
 *     transfer async;
 *     requires capability(...);
 *     constraint ...;
 *     prefer ...;
 *     hint ...;
 *     migrate ...;
 *     prefetch ...;
 *     map ...;
 *     unmap ...;
 *     distributed memory ...;
 *     tensor ...;
 *     quantum ...;
 *
 * Negative examples should reject, at the appropriate grammar/semantic layer:
 *
 *     malformed qualified names;
 *     malformed argument lists;
 *     malformed predicates;
 *     missing operation arguments where required;
 *     invalid delimiters;
 *     malformed capacity expressions;
 *     malformed capability expressions;
 *     malformed policy blocks.
 *
 * Architectural negative tests must ensure that future validation rejects
 * attempts to turn physical hardware topology into mandatory universal
 * accelerator-memory syntax.
 */


/*
 * ============================================================================
 * 69. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must exercise expressions whose values scale with:
 *
 *     input size;
 *     tensor dimensions;
 *     workload;
 *     data volume;
 *     execution context;
 *     available resources.
 *
 * The grammar must not impose a semantic maximum on:
 *
 *     capacity;
 *     number of buffers;
 *     number of memory regions;
 *     number of memory spaces;
 *     number of accelerator operations;
 *     number of transfers;
 *     number of accelerator devices.
 *
 * Any finite CI/parser budget is an implementation resource limit, not a
 * Zamani language limit.
 */


/*
 * ============================================================================
 * 70. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file exists to express:
 *
 *     accelerator-memory INTENT
 *
 * It does not express:
 *
 *     accelerator-memory REALIZATION
 *
 * Therefore:
 *
 *     SOURCE
 *       ↓
 *     MEMORY INTENT
 *       ↓
 *     SEMANTICS
 *       ↓
 *     RESOURCE/CAPABILITY ANALYSIS
 *       ↓
 *     CANONICAL IR
 *       ↓
 *     OPTIMIZATION
 *       ↓
 *     PLACEMENT
 *       ↓
 *     SCHEDULING
 *       ↓
 *     HAL
 *       ↓
 *     ACTUAL ACCELERATOR
 *
 * This preserves the Zamani architectural goal:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * from the smallest accelerator-capable system through arbitrarily large
 * heterogeneous systems, subject to actual available resources rather than
 * artificial grammar limits.
 *
 * ============================================================================
 */