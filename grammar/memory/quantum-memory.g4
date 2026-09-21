/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/quantum-memory.g4
 *
 * STATUS
 * ------
 * CANONICAL QUANTUM-MEMORY SOURCE-GRAMMAR COMPONENT
 *
 * GRAMMAR KIND
 * ------------
 * ANTLR4 parser grammar.
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 Edition
 * Safe Rust only.
 * No unsafe implementation requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL SYNTAX for quantum-memory intent.
 *
 * Quantum memory is deliberately treated as a semantic resource domain rather
 * than as a synonym for a particular simulator representation or physical
 * quantum device.
 *
 * This grammar therefore supports source-level intent concerning:
 *
 *   - logical quantum-memory resources;
 *   - quantum-memory allocation;
 *   - quantum-memory release;
 *   - quantum-memory views;
 *   - quantum-memory slices;
 *   - quantum-memory projections;
 *   - partial-trace intent;
 *   - quantum-memory copies;
 *   - quantum-memory state access;
 *   - quantum-memory state snapshots;
 *   - quantum-memory checkpoints;
 *   - quantum-memory restoration;
 *   - quantum-memory migration;
 *   - quantum-memory persistence;
 *   - quantum/classical companion memory;
 *   - quantum-memory resource requirements;
 *   - quantum-memory capabilities;
 *   - quantum-memory constraints;
 *   - quantum-memory preferences;
 *   - quantum-memory hints;
 *   - quantum-memory layouts;
 *   - quantum-memory representation intent;
 *   - quantum-memory storage-location intent;
 *   - provider-neutral extension points.
 *
 * The grammar deliberately does NOT define:
 *
 *   - quantum gates;
 *   - circuits;
 *   - quantum::ir;
 *   - physical qubit identities;
 *   - routing;
 *   - scheduling;
 *   - QEC algorithms;
 *   - ZQN;
 *   - calibration;
 *   - vendor APIs;
 *   - device discovery;
 *   - physical memory addresses;
 *   - pointer values;
 *   - simulator implementation;
 *   - state-vector allocation;
 *   - tensor-network algorithms;
 *   - GPU allocation algorithms;
 *   - distributed placement algorithms.
 *
 * Those responsibilities belong to downstream semantic/compiler/runtime
 * components.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar is designed for:
 *
 *   Program_Once
 *       ->
 *   Compile_Once
 *       ->
 *   Run_Everywhere
 *       ->
 *   Run_Anywhere
 *       ->
 *   Forever
 *
 * subject to:
 *
 *   - program semantics;
 *   - declared requirements;
 *   - target capabilities;
 *   - available resources;
 *   - implementation-defined operational limits.
 *
 * No language-level maximum is encoded here for:
 *
 *   - qubits;
 *   - logical qubits;
 *   - physical qubits;
 *   - quantum-memory resources;
 *   - amplitudes;
 *   - state dimensions;
 *   - snapshots;
 *   - checkpoints;
 *   - memory regions;
 *   - allocations;
 *   - views;
 *   - slices;
 *   - distributed partitions;
 *   - devices;
 *   - accelerators;
 *   - nodes;
 *   - storage domains.
 *
 * "Infinity" therefore means that the language does not introduce an
 * artificial hardware ceiling. Actual execution remains bounded by available
 * resources and the mathematical/semantic requirements of the program.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         canonical lexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          type system     resources        capabilities
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *               +--------------+--------------+
 *               |                             |
 *               v                             v
 *       classical semantics             quantum semantics
 *                                             |
 *                                             v
 *                                        quantum::ir
 *                                             |
 *                         +-------------------+----------------+
 *                         |                   |                |
 *                         v                   v                v
 *                     optimization         routing        scheduling
 *                                             |
 *                                             v
 *                                      QEC / resilience
 *                                             |
 *                                             v
 *                                            ZQN
 *                                             |
 *                                             v
 *                                            HAL
 *                                             |
 *                                             v
 *                                      target realization
 *
 * Quantum-memory syntax enters this pipeline as memory/resource intent.
 *
 * It does NOT create a second quantum semantic representation.
 *
 * ============================================================================
 * RELATIONSHIP TO EXISTING MEMORY GRAMMAR
 * ============================================================================
 *
 * Canonical generic memory foundation:
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
 *     memorySpecification
 *     expression
 *     typeExpression
 *
 * The generic memory grammar remains the owner of those concepts.
 *
 * ============================================================================
 * RELATIONSHIP TO QUANTUM TYPE GRAMMAR
 * ============================================================================
 *
 * Quantum source types are owned by:
 *
 *     grammar/types/quantum.g4
 *
 * This file MUST NOT redefine:
 *
 *     qubit
 *     logical qubit
 *     quantum<T>
 *     quantum-qualified types
 *     generic type application.
 *
 * A quantum-memory resource may refer to those types through the canonical
 * type-expression boundary where appropriate.
 *
 * ============================================================================
 * RELATIONSHIP TO QUANTUM IR
 * ============================================================================
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This file MUST NOT introduce:
 *
 *     QuantumMemoryIR
 *     QuantumMemoryOperationIR
 *     QuantumMemoryQubitIR
 *     QuantumMemoryGateIR
 *     QuantumMemoryCircuitIR
 *     MemoryQubitId
 *     PhysicalMemoryQubitId
 *
 * Quantum program identity remains owned by quantum::ir.
 *
 * Quantum-memory resource identities remain owned by the existing
 * `src/quantum/memory` subsystem.
 *
 * ============================================================================
 * RELATIONSHIP TO EXISTING RUST QUANTUM-MEMORY SUBSYSTEM
 * ============================================================================
 *
 * The repository already contains a provider-neutral quantum-memory subsystem
 * under:
 *
 *     src/quantum/memory/
 *
 * Important existing ownership includes:
 *
 *     types.rs
 *     budget.rs
 *     lifetime.rs
 *     layout.rs
 *     slice.rs
 *     view.rs
 *     snapshot.rs
 *     checkpoint.rs
 *     persistence.rs
 *     migration.rs
 *     allocator.rs
 *     reservation.rs
 *     limits.rs
 *     state representations
 *     backend/provider abstractions
 *
 * Existing canonical quantum identities remain in:
 *
 *     quantum::ir::QubitId
 *     quantum::ir::PhysicalQubitId
 *     quantum::ir::ClassicalBitId
 *
 * The grammar therefore represents source intent which semantic analysis maps
 * onto these existing contracts.
 *
 * It must not attempt to mirror every Rust enum or struct as a source keyword.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It declares NO lexer rules.
 *
 * The token vocabulary is supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the existing parser-composition architecture.
 *
 * The grammar deliberately avoids introducing new quantum-memory keywords
 * wherever an existing generic memory operation/name mechanism is sufficient.
 *
 * This is important because the repository's lexical architecture explicitly
 * requires a single lexical authority.
 *
 * Domain-specific names remain semantic names rather than an ever-growing
 * reserved-word inventory.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Quantum-memory operations use the generic qualified-operation model from
 * memory.g4.
 *
 * Examples of semantic names that may be resolved downstream include:
 *
 *     memory::quantum::allocate(...)
 *     memory::quantum::release(...)
 *     memory::quantum::view(...)
 *     memory::quantum::slice(...)
 *     memory::quantum::copy(...)
 *     memory::quantum::project(...)
 *     memory::quantum::partial_trace(...)
 *     memory::quantum::snapshot(...)
 *     memory::quantum::checkpoint(...)
 *     memory::quantum::restore(...)
 *     memory::quantum::migrate(...)
 *
 * These names are examples of semantic operation identifiers.
 *
 * The grammar does not require an exhaustive operation catalogue.
 *
 * Consequently a future operation such as:
 *
 *     memory::quantum::future::operation(...)
 *
 * can be parsed without changing this grammar merely because the semantic
 * operation is new.
 *
 * Semantic resolution MUST still reject unknown operations where the language
 * requires a registered operation.
 *
 * Open-world syntax is therefore extensibility, not automatic semantic
 * acceptance.
 *
 * ============================================================================
 * RESOURCE / SEMANTIC DISTINCTION
 * ============================================================================
 *
 * This grammar preserves the difference between:
 *
 *   requirement
 *   constraint
 *   capability
 *   preference
 *   hint
 *   implementation decision
 *
 * Examples:
 *
 *   memory::quantum::require(...)
 *
 * is source intent.
 *
 * It does not mean:
 *
 *   use physical qubit 0
 *
 * or:
 *
 *   use GPU 0
 *
 * or:
 *
 *   allocate exactly one particular provider resource.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * REPRESENTATION NEUTRALITY
 * ============================================================================
 *
 * A quantum-memory resource may ultimately be represented as:
 *
 *   - a state vector;
 *   - a density matrix;
 *   - a stabilizer/tableau;
 *   - a sparse state;
 *   - a tensor network;
 *   - backend-native state;
 *   - an opaque provider-managed state;
 *   - distributed state;
 *   - another future representation.
 *
 * The source language does not choose one merely by using quantum-memory
 * syntax.
 *
 * A source-level representation preference may be expressed as semantic
 * metadata, but the compiler determines whether it can be honored.
 *
 * ============================================================================
 * MEMORY VS QUANTUM STATE
 * ============================================================================
 *
 * Quantum memory has two different concepts that must not be conflated:
 *
 *   1. the RESOURCE containing/managing quantum information;
 *   2. the MATHEMATICAL STATE represented by that resource.
 *
 * For example, a view is not automatically a reduced quantum state.
 *
 * A copy is not automatically a partial trace.
 *
 * A projection is not automatically a partial trace.
 *
 * A partial trace is a mathematical state-reduction operation.
 *
 * The grammar therefore exposes distinct operation names/semantic categories
 * rather than allowing downstream implementations to silently substitute one
 * for another.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A surrounding parser may use:
 *
 *     quantumMemoryConstruct
 *
 * as the canonical domain entry point.
 *
 * This rule accepts only constructs owned by this file.
 *
 * The universal memory constructs remain available through Memory.
 *
 * ============================================================================
 */

parser grammar QuantumMemory;

options {
    tokenVocab = ZamaniLexer;
}

import Memory;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Quantum-memory syntax is intentionally composable.
 *
 * It may occur through:
 *
 *   - declarations;
 *   - operations;
 *   - resource specifications;
 *   - capability specifications;
 *   - representation/storage intent;
 *   - slicing/view specifications;
 *   - persistence/checkpoint intent;
 *   - migration intent.
 *
 * The surrounding Zamani parser determines the syntactic locations where this
 * domain may occur.
 */
quantumMemoryConstruct
    : quantumMemoryDeclaration
    | quantumMemoryOperation
    | quantumMemorySpecification
    | quantumMemoryView
    | quantumMemorySlice
    | quantumMemoryPersistence
    | quantumMemoryMigration
    | quantumMemoryCheckpoint
    | quantumMemoryCapability
    | quantumMemoryRepresentation
    ;


/*
 * ============================================================================
 * 2. QUANTUM-MEMORY DECLARATION
 * ============================================================================
 *
 * A declaration associates an existing source-level memory place with
 * quantum-memory intent.
 *
 * The declaration does NOT introduce a new universal variable/type grammar.
 *
 * Conceptually supported forms include:
 *
 *     quantum memory buffer ...
 *     quantum memory state ...
 *     quantum memory resource ...
 *
 * Exact source vocabulary is resolved through the surrounding declaration
 * grammar and semantic specification.
 *
 * This component deliberately uses the existing memory-domain abstraction
 * rather than introducing physical storage declarations.
 */
quantumMemoryDeclaration
    : memoryPlace
      quantumMemoryDeclarationClause*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. DECLARATION CLAUSES
 * ============================================================================
 *
 * Clauses carry source-level semantic intent.
 */
quantumMemoryDeclarationClause
    : quantumMemoryTypeClause
    | quantumMemorySpaceClause
    | quantumMemoryRegionClause
    | quantumMemoryLifetimeClause
    | quantumMemoryLayoutClause
    | quantumMemoryRepresentationClause
    | quantumMemoryResourceClause
    | quantumMemoryCapabilityClause
    | quantumMemoryPersistenceClause
    | quantumMemoryPolicyClause
    ;


/*
 * ============================================================================
 * 4. TYPE
 * ============================================================================
 *
 * The canonical type grammar remains authoritative.
 *
 * This rule only provides the integration boundary.
 */
quantumMemoryTypeClause
    : COLON typeExpression
    ;


/*
 * ============================================================================
 * 5. MEMORY SPACE
 * ============================================================================
 *
 * A quantum-memory space is symbolic.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::device
 *     memory::distributed
 *     memory::remote
 *     memory::custom::quantum
 *
 * No finite universal catalogue is encoded.
 */
quantumMemorySpaceClause
    : IN memoryQualifiedName
    ;


/*
 * ============================================================================
 * 6. MEMORY REGION
 * ============================================================================
 *
 * A region is semantic organization/lifetime information.
 *
 * It does not imply:
 *
 *     physical memory bank
 *     NUMA node
 *     cache
 *     device
 *     page
 *     address range
 */
quantumMemoryRegionClause
    : IN memoryRegionReference
    ;


/*
 * ============================================================================
 * 7. LIFETIME
 * ============================================================================
 *
 * Lifetime semantics remain owned by the memory subsystem.
 *
 * This rule preserves an explicit lifetime reference where permitted.
 */
quantumMemoryLifetimeClause
    : memoryLifetimeClause
    ;


/*
 * ============================================================================
 * 8. LAYOUT
 * ============================================================================
 *
 * Layout describes logical/storage organization.
 *
 * It does NOT define physical topology.
 *
 * Examples:
 *
 *     contiguous
 *     indexed
 *     sparse
 *     tensor
 *     distributed
 *     provider_native
 *
 * Names remain open-world.
 */
quantumMemoryLayoutClause
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 9. REPRESENTATION
 * ============================================================================
 *
 * A source program may express a representation requirement/preference.
 *
 * The grammar does not enumerate simulator implementations.
 *
 * Semantic examples:
 *
 *     state_vector
 *     density_matrix
 *     stabilizer
 *     sparse
 *     tensor_network
 *     backend_native
 *     extension::representation
 *
 * These remain semantic names.
 */
quantumMemoryRepresentationClause
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 10. RESOURCE INTENT
 * ============================================================================
 *
 * Resource values remain general expressions.
 *
 * This allows:
 *
 *     symbolic qubit counts;
 *     symbolic classical-memory quantities;
 *     runtime-derived quantities;
 *     generic parameters;
 *     computed extents;
 *     resource expressions.
 *
 * No Rust integer width is imposed by the grammar.
 */
quantumMemoryResourceClause
    : memoryResourceSpecification
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 *
 * Capability resolution belongs to semantic/resource analysis.
 *
 * The capability name is open-world and qualified.
 */
quantumMemoryCapabilityClause
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 12. PERSISTENCE
 * ============================================================================
 *
 * Persistence syntax remains compatible with the existing memory persistence
 * subsystem.
 *
 * The actual persistence implementation remains outside the grammar.
 */
quantumMemoryPersistence
    : memoryQualifiedName
      quantumMemoryOptionalArguments
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. MIGRATION
 * ============================================================================
 *
 * Migration describes semantic movement between abstract memory locations or
 * representations.
 *
 * It does NOT define:
 *
 *     DMA;
 *     PCIe;
 *     CXL;
 *     network links;
 *     physical addresses;
 *     device identifiers.
 */
quantumMemoryMigration
    : memoryQualifiedName
      LPAREN quantumMemoryMigrationArguments? RPAREN
      SEMICOLON?
    ;


quantumMemoryMigrationArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


quantumMemoryArgument
    : expression
    | memoryNamedArgument
    ;


/*
 * ============================================================================
 * 14. CHECKPOINT
 * ============================================================================
 *
 * Checkpoint semantics integrate with:
 *
 *     src/quantum/memory/checkpoint.rs
 *
 * The grammar describes intent only.
 */
quantumMemoryCheckpoint
    : memoryQualifiedName
      quantumMemoryOptionalArguments
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. GENERIC QUANTUM-MEMORY OPERATION
 * ============================================================================
 *
 * This is the primary extensibility boundary.
 *
 * An operation consists of:
 *
 *     qualified semantic operation name
 *     optional argument list
 *
 * Examples:
 *
 *     memory::quantum::allocate(...)
 *     memory::quantum::release(...)
 *     memory::quantum::view(...)
 *     memory::quantum::slice(...)
 *     memory::quantum::copy(...)
 *     memory::quantum::project(...)
 *     memory::quantum::partial_trace(...)
 *     memory::quantum::snapshot(...)
 *     memory::quantum::checkpoint(...)
 *     memory::quantum::restore(...)
 *     memory::quantum::migrate(...)
 *
 * The parser does not need a new keyword for every future operation.
 */
quantumMemoryOperation
    : memoryQualifiedName
      LPAREN quantumMemoryOperationArguments? RPAREN
      SEMICOLON?
    ;


quantumMemoryOperationArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 16. OPTIONAL ARGUMENTS
 * ============================================================================
 *
 * This helper exists to avoid duplicating the common optional-argument shape
 * throughout this file.
 */
quantumMemoryOptionalArguments
    : LPAREN quantumMemoryArgumentList? RPAREN
    ;


quantumMemoryArgumentList
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 17. VIEW
 * ============================================================================
 *
 * A quantum-memory view is NON-OWNING semantic access to a selected logical
 * subsystem or memory resource.
 *
 * It must not imply:
 *
 *     copying;
 *     projection;
 *     partial trace;
 *     state reduction.
 *
 * The existing Rust implementation's `MemoryView`/view contracts remain
 * downstream owners of representation-specific behavior.
 */
quantumMemoryView
    : memoryQualifiedName
      LPAREN quantumMemorySelectionArguments? RPAREN
      SEMICOLON?
    ;


quantumMemorySelectionArguments
    : quantumMemorySelectionArgument
      (COMMA quantumMemorySelectionArgument)*
      COMMA?
    ;


quantumMemorySelectionArgument
    : expression
    | memoryNamedArgument
    ;


/*
 * ============================================================================
 * 18. SLICE
 * ============================================================================
 *
 * A slice describes semantic selection.
 *
 * It does not itself determine whether the implementation:
 *
 *     - creates a view;
 *     - copies;
 *     - projects;
 *     - partially traces.
 *
 * Those semantics must be explicit in the operation/category selected by the
 * source program or by the semantic specification.
 *
 * This prevents a generic "slice" operation from silently changing
 * mathematical meaning.
 */
quantumMemorySlice
    : memoryQualifiedName
      LPAREN quantumMemorySliceArguments? RPAREN
      SEMICOLON?
    ;


quantumMemorySliceArguments
    : quantumMemorySliceArgument
      (COMMA quantumMemorySliceArgument)*
      COMMA?
    ;


quantumMemorySliceArgument
    : expression
    | memoryNamedArgument
    ;


/*
 * ============================================================================
 * 19. SLICE TARGET
 * ============================================================================
 *
 * Selection may be expressed symbolically.
 *
 * This permits:
 *
 *     q
 *     q[i]
 *     q[start..end]
 *     selected
 *     computed_selection
 *
 * The grammar does not impose a finite number of selected elements.
 */
quantumMemorySelection
    : expression
    ;


/*
 * ============================================================================
 * 20. QUANTUM-MEMORY SPECIFICATION
 * ============================================================================
 *
 * This is the preferred contract for declarations and operations that need
 * multiple independent memory properties.
 *
 * The generic memory grammar owns the individual semantic categories.
 */
quantumMemorySpecification
    : memorySpecification
    ;


/*
 * ============================================================================
 * 21. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * This rule delegates the actual requirement expression to the canonical
 * memory/resource grammar.
 */
quantumMemoryRequirement
    : memoryRequirement
    ;


/*
 * ============================================================================
 * 22. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts legal realization choices.
 */
quantumMemoryConstraint
    : memoryConstraint
    ;


/*
 * ============================================================================
 * 23. PREFERENCE
 * ============================================================================
 *
 * A preference is advisory unless the semantic contract explicitly says
 * otherwise.
 */
quantumMemoryPreference
    : memoryPreference
    ;


/*
 * ============================================================================
 * 24. HINT
 * ============================================================================
 *
 * A hint must never silently become a correctness requirement.
 */
quantumMemoryHint
    : memoryHint
    ;


/*
 * ============================================================================
 * 25. POLICY
 * ============================================================================
 *
 * Policies are symbolic semantic requests.
 *
 * They do not execute policy logic.
 */
quantumMemoryPolicyClause
    : memoryPolicyClause
    ;


/*
 * ============================================================================
 * 26. QUANTUM-MEMORY ALLOCATION
 * ============================================================================
 *
 * Allocation is represented as a generic semantic operation.
 *
 * The operation name remains open-world so that the semantic registry can
 * evolve independently of this grammar.
 *
 * The operation MUST NOT encode:
 *
 *     physical qubit IDs;
 *     physical addresses;
 *     vendor device IDs;
 *     fixed resource counts.
 */
quantumMemoryAllocation
    : memoryQualifiedName
      LPAREN quantumMemoryAllocationArguments? RPAREN
      SEMICOLON?
    ;


quantumMemoryAllocationArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 27. QUANTUM-MEMORY RELEASE
 * ============================================================================
 *
 * Release/deallocation remains semantic intent.
 *
 * It does not imply immediate physical reclamation.
 */
quantumMemoryRelease
    : memoryQualifiedName
      LPAREN quantumMemoryArgumentList? RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 28. STATE ACCESS
 * ============================================================================
 *
 * This rule is deliberately generic.
 *
 * Quantum state access is not assumed to mean:
 *
 *     dense state vector;
 *     host memory;
 *     byte address;
 *     classical copy.
 */
quantumMemoryStateAccess
    : memoryQualifiedName
      LPAREN quantumMemoryArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * 29. PROJECTION
 * ============================================================================
 *
 * Projection is semantically distinct from partial trace.
 *
 * The semantic layer must determine:
 *
 *     - retained subsystem;
 *     - discarded subsystem;
 *     - basis assignments;
 *     - normalization behavior;
 *     - probability/result reporting.
 *
 * The grammar does not implement any of those calculations.
 */
quantumMemoryProjection
    : memoryQualifiedName
      LPAREN quantumMemoryProjectionArguments? RPAREN
      SEMICOLON?
    ;


quantumMemoryProjectionArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 30. PARTIAL TRACE
 * ============================================================================
 *
 * Partial trace is a mathematical operation.
 *
 * It MUST NOT be implemented as an alias for:
 *
 *     view
 *     copy
 *     projection
 *
 * The semantic representation must preserve its distinct meaning.
 */
quantumMemoryPartialTrace
    : memoryQualifiedName
      LPAREN quantumMemoryPartialTraceArguments? RPAREN
      SEMICOLON?
    ;


quantumMemoryPartialTraceArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 31. COPY
 * ============================================================================
 *
 * Copy semantics indicate independent materialization.
 *
 * Copy does not imply mathematical state reduction.
 */
quantumMemoryCopy
    : memoryQualifiedName
      LPAREN quantumMemoryArgumentList? RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 32. SNAPSHOT
 * ============================================================================
 *
 * Integrates conceptually with:
 *
 *     src/quantum/memory/snapshot.rs
 *
 * A snapshot is a semantic/persistence object.
 *
 * It must not contain raw pointers, process-local addresses, or implicit
 * hardware identifiers.
 */
quantumMemorySnapshot
    : memoryQualifiedName
      LPAREN quantumMemorySnapshotArguments? RPAREN
      SEMICOLON?
    ;


quantumMemorySnapshotArguments
    : quantumMemoryArgument
      (COMMA quantumMemoryArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 33. RESTORE
 * ============================================================================
 *
 * Restore is a semantic request.
 *
 * Whether restoration is possible depends on the target/backend and the
 * snapshot/checkpoint contract.
 */
quantumMemoryRestore
    : memoryQualifiedName
      LPAREN quantumMemoryArgumentList? RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 34. RESOURCE/CAPABILITY CONTRACT
 * ============================================================================
 *
 * A capability is represented symbolically.
 *
 * Examples of semantic capability identifiers include:
 *
 *     capability::quantum_memory
 *     capability::quantum_snapshot
 *     capability::quantum_checkpoint
 *     capability::quantum_state_restore
 *     capability::quantum_state_migration
 *     capability::distributed_quantum_memory
 *
 * The grammar does not reserve those names.
 */
quantumMemoryCapability
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 35. REPRESENTATION CONTRACT
 * ============================================================================
 *
 * Representation requirements/preferences remain symbolic.
 *
 * Examples:
 *
 *     representation::state_vector
 *     representation::density_matrix
 *     representation::stabilizer
 *     representation::sparse
 *     representation::tensor_network
 *     representation::backend_native
 *
 * A semantic resolver determines whether the requested representation exists
 * and whether it is appropriate.
 */
quantumMemoryRepresentation
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 36. QUANTUM-MEMORY RANGE
 * ============================================================================
 *
 * Ranges remain expressions.
 *
 * No fixed index width is assumed.
 *
 * This is particularly important for:
 *
 *     large quantum registers;
 *     symbolic selections;
 *     distributed selections;
 *     future sparse representations.
 */
quantumMemoryRange
    : expression
    ;


/*
 * ============================================================================
 * 37. QUANTUM-MEMORY EXTENT
 * ============================================================================
 *
 * An extent is a semantic expression.
 *
 * It may be:
 *
 *     constant;
 *     generic;
 *     symbolic;
 *     runtime-dependent;
 *     resource-derived.
 *
 * The grammar does not convert it into a fixed Rust integer.
 */
quantumMemoryExtent
    : expression
    ;


/*
 * ============================================================================
 * 38. QUANTUM-MEMORY RESOURCE REQUIREMENT
 * ============================================================================
 *
 * A resource quantity may be represented symbolically.
 *
 * This rule is intentionally generic because resource quantity semantics are
 * owned by the resource subsystem.
 */
quantumMemoryResourceRequirement
    : memoryResourceSpecification
    ;


/*
 * ============================================================================
 * 39. QUANTUM-MEMORY STORAGE LOCATION
 * ============================================================================
 *
 * A storage location is an ABSTRACT memory location.
 *
 * Examples:
 *
 *     memory::host
 *     memory::device
 *     memory::distributed
 *     memory::remote
 *     memory::unified
 *     memory::custom::provider
 *
 * It is not:
 *
 *     physical address 0x...
 *     GPU 0
 *     QPU 0
 *     memory bank 0
 *     NUMA node 0
 */
quantumMemoryStorageLocation
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 40. QUANTUM-MEMORY LIFETIME
 * ============================================================================
 *
 * Delegates lifetime identity to the canonical memory lifetime grammar.
 */
quantumMemoryLifetime
    : memoryLifetime
    ;


/*
 * ============================================================================
 * 41. QUANTUM-MEMORY REGION
 * ============================================================================
 *
 * Delegates region identity to the canonical memory grammar.
 */
quantumMemoryRegion
    : memoryRegion
    ;


/*
 * ============================================================================
 * 42. QUANTUM-MEMORY PLACE
 * ============================================================================
 *
 * Delegates source-level place identity to the canonical memory grammar.
 */
quantumMemoryPlace
    : memoryPlace
    ;


/*
 * ============================================================================
 * 43. QUANTUM-MEMORY TYPE
 * ============================================================================
 *
 * Delegates type identity to the canonical type grammar.
 *
 * No QuantumMemoryType is introduced.
 */
quantumMemoryType
    : typeExpression
    ;


/*
 * ============================================================================
 * 44. QUANTUM-MEMORY EXTENSION
 * ============================================================================
 *
 * Future quantum-memory domains can be introduced through qualified semantic
 * names without modifying the foundational grammar.
 *
 * Examples:
 *
 *     memory::quantum::provider::extension(...)
 *     memory::quantum::future::technology(...)
 *
 * Semantic registration determines whether an extension is legal.
 */
quantumMemoryExtension
    : memoryQualifiedName
      quantumMemoryOptionalArguments
    ;


/*
 * ============================================================================
 * 45. CROSS-DOMAIN RESOURCE CONTRACT
 * ============================================================================
 *
 * Quantum memory may coexist with:
 *
 *     classical memory
 *     accelerator memory
 *     distributed memory
 *     persistent memory
 *     hardware/co-design memory
 *
 * This rule preserves a single generic memory specification rather than
 * creating a second hybrid-memory grammar.
 */
quantumMemoryCrossDomainSpecification
    : memorySpecification
    ;


/*
 * ============================================================================
 * 46. CANONICAL ARGUMENT BOUNDARY
 * ============================================================================
 *
 * All quantum-memory arguments ultimately consume the canonical expression
 * grammar.
 *
 * This prevents quantum-memory from creating a second expression language.
 */
quantumMemoryExpression
    : expression
    ;


/*
 * ============================================================================
 * 47. NO PHYSICAL QUANTUM IDENTITY
 * ============================================================================
 *
 * Deliberately absent:
 *
 *     physicalQubit
 *     physicalQubitId
 *     qpuId
 *     deviceId
 *     memoryBankId
 *
 * Physical identities belong downstream.
 *
 * If source-level logical quantum identity is required, the canonical quantum
 * grammar/AST/IR owns it.
 *
 * ============================================================================
 * 48. NO FIXED STATE REPRESENTATION
 * ============================================================================
 *
 * Deliberately absent:
 *
 *     stateVector[n]
 *     densityMatrix[n]
 *
 * as universal physical allocation constructs.
 *
 * Source programs may express mathematical dimensions through ordinary
 * expressions/types. The semantic layer determines representation and
 * resource realization.
 *
 * ============================================================================
 * 49. NO FIXED QUANTUM-MEMORY LIMIT
 * ============================================================================
 *
 * Deliberately absent:
 *
 *     MAX_QUBITS
 *     MAX_AMPLITUDES
 *     MAX_STATE_SIZE
 *     MAX_SNAPSHOTS
 *     MAX_CHECKPOINTS
 *     MAX_MEMORY
 *     MAX_DISTRIBUTED_PARTITIONS
 *     MAX_DEVICES
 *
 * The grammar therefore remains scalable from tiny programs to programs whose
 * requirements exceed any particular target.
 *
 * ============================================================================
 * 50. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no hardware queries;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime access;
 *     - no randomness;
 *     - no wall-clock dependency.
 *
 * Parsing depends only on:
 *
 *     source tokens;
 *     grammar version;
 *     imported grammar contracts.
 *
 * ============================================================================
 * 51. SAFETY
 * ============================================================================
 *
 * This grammar contains no executable Rust.
 *
 * The generated frontend is required to remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository-wide no-unsafe requirement.
 *
 * This grammar cannot itself introduce Rust `unsafe`.
 *
 * ============================================================================
 * 52. SECURITY
 * ============================================================================
 *
 * Parsing quantum-memory syntax MUST NOT:
 *
 *     - allocate quantum state;
 *     - contact a QPU;
 *     - inspect a device;
 *     - load a provider;
 *     - access credentials;
 *     - execute an operation;
 *     - access the filesystem;
 *     - access the network.
 *
 * External capability evidence is a semantic/runtime concern.
 *
 * ============================================================================
 * 53. SOURCE SPAN / AST CONTRACT
 * ============================================================================
 *
 * Every accepted quantum-memory construct must remain traceable to its source
 * span.
 *
 * The AST must preserve, where applicable:
 *
 *     - operation/path;
 *     - arguments;
 *     - named arguments;
 *     - target place;
 *     - type;
 *     - memory space;
 *     - region;
 *     - lifetime;
 *     - representation intent;
 *     - layout intent;
 *     - resource requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - persistence/checkpoint metadata;
 *     - explicit-vs-default information.
 *
 * The parser does not decide semantic validity.
 *
 * ============================================================================
 * 54. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the operation is registered;
 *     - whether the target exists;
 *     - whether the memory place is valid;
 *     - whether ownership permits the operation;
 *     - whether lifetimes are valid;
 *     - whether the requested representation is compatible;
 *     - whether the requested resource quantities are satisfiable;
 *     - whether capabilities exist;
 *     - whether requirements can be satisfied;
 *     - whether constraints are compatible;
 *     - whether preferences can be honored;
 *     - whether hints are applicable;
 *     - whether a view/copy/projection/partial-trace distinction is preserved;
 *     - whether persistence/restoration is semantically legal;
 *     - whether migration is supported;
 *     - how the intent maps into the canonical semantic representation.
 *
 * ============================================================================
 * 55. QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * Quantum-memory semantics may be consumed alongside quantum operations, but
 * the canonical quantum operation/program boundary remains:
 *
 *     quantum::ir
 *
 * The expected direction is:
 *
 *     Quantum-memory AST
 *          |
 *          v
 *     semantic memory/resource model
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     memory subsystem      quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *              execution/lowering
 *
 * This grammar MUST NOT make quantum-memory a second quantum IR.
 *
 * ============================================================================
 * 56. QEC CONTRACT
 * ============================================================================
 *
 * Quantum error correction remains owned by:
 *
 *     src/quantum/error_correction/
 *
 * Quantum-memory syntax may express generic resource/capability requirements
 * needed by QEC.
 *
 * It MUST NOT define:
 *
 *     - code distance;
 *     - syndrome extraction;
 *     - decoder algorithms;
 *     - physical qubit placement;
 *     - logical-to-physical mapping.
 *
 * Those are downstream semantic/compiler concerns.
 *
 * ============================================================================
 * 57. ZQN CONTRACT
 * ============================================================================
 *
 * ZQN remains responsible for quantum noise/fault semantics.
 *
 * Quantum-memory grammar may preserve source-level resource/capability
 * metadata relevant to resilience.
 *
 * It MUST NOT define:
 *
 *     - noise channels;
 *     - fault distributions;
 *     - leakage models;
 *     - correlated error models;
 *     - decoder behavior.
 *
 * ============================================================================
 * 58. ROUTING CONTRACT
 * ============================================================================
 *
 * Routing owns logical-to-physical mapping.
 *
 * Quantum-memory syntax must never encode physical mapping as a universal
 * source-language construct.
 *
 * The memory subsystem may consume routing results for representation/layout,
 * but this grammar does not perform routing.
 *
 * ============================================================================
 * 59. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling owns execution ordering and timing.
 *
 * Quantum-memory grammar may express resource/lifetime/coherence requirements.
 *
 * It does not select:
 *
 *     cycle numbers;
 *     machine queues;
 *     physical execution slots;
 *     device timelines.
 *
 * ============================================================================
 * 60. HAL CONTRACT
 * ============================================================================
 *
 * HAL provides target-specific:
 *
 *     capabilities;
 *     resources;
 *     state;
 *     health;
 *     calibration;
 *     execution interfaces.
 *
 * The grammar remains independent of HAL.
 *
 * The semantic/compiler direction is:
 *
 *     source intent
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     target capability evaluation
 *          |
 *          v
 *     HAL realization
 *
 * ============================================================================
 * 61. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns actual:
 *
 *     allocation;
 *     reclamation;
 *     migration;
 *     synchronization;
 *     state handling;
 *     provider interaction;
 *     execution.
 *
 * Parsing must never invoke runtime behavior.
 *
 * ============================================================================
 * 62. INTEROPERABILITY
 * ============================================================================
 *
 * Quantum-memory semantics may interoperate with:
 *
 *     OpenQASM
 *     QIR
 *     simulator representations
 *     provider-native state handles
 *     serialization formats
 *
 * but those formats remain interoperability boundaries.
 *
 * They do not become competing Zamani source grammars or semantic authorities.
 *
 * ============================================================================
 * 63. COMPATIBILITY
 * ============================================================================
 *
 * Token names are inherited from the canonical lexer.
 *
 * Introducing a new reserved keyword is therefore NOT required for ordinary
 * quantum-memory extensions.
 *
 * Adding or removing a parser production remains a language compatibility
 * event and must be tracked through:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * Semantic operation names should be versioned through the semantic feature
 * registry rather than by continuously expanding the reserved keyword set.
 *
 * ============================================================================
 * 64. TEST CONTRACT
 * ============================================================================
 *
 * Production conformance requires:
 *
 * POSITIVE TESTS
 * --------------
 *
 *     - generic quantum-memory operation;
 *     - allocation intent;
 *     - release intent;
 *     - view;
 *     - slice;
 *     - copy;
 *     - projection;
 *     - partial trace;
 *     - snapshot;
 *     - checkpoint;
 *     - restore;
 *     - migration;
 *     - representation intent;
 *     - resource requirements;
 *     - capability requirements;
 *     - symbolic extents;
 *     - symbolic qubit counts;
 *     - nested qualified names;
 *     - provider-neutral extensions.
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     - malformed qualified names;
 *     - malformed argument lists;
 *     - malformed ranges;
 *     - invalid delimiters;
 *     - invalid source placement;
 *     - malformed named arguments.
 *
 * Semantic negative tests must additionally cover:
 *
 *     - unknown operation;
 *     - unsatisfied resource requirement;
 *     - unsupported capability;
 *     - invalid lifetime;
 *     - invalid ownership;
 *     - unsupported representation;
 *     - invalid projection assignment;
 *     - invalid partial trace;
 *     - invalid restoration.
 *
 * BOUNDARY TESTS
 * --------------
 *
 *     - zero-sized semantic resources where legal;
 *     - one qubit;
 *     - many qubits;
 *     - symbolic qubit counts;
 *     - empty argument lists;
 *     - one argument;
 *     - many arguments;
 *     - deeply qualified names;
 *     - nested expressions;
 *     - nested memory specifications.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Tests MUST vary N rather than establish a fixed maximum.
 *
 * Examples:
 *
 *     N qubits
 *     N memory elements
 *     N selected qubits
 *     N snapshots
 *     N distributed partitions
 *
 * The purpose is to verify that the grammar contains no artificial universal
 * ceiling.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * The same source and grammar version must produce the same parse structure.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Quantum-memory syntax must coexist with:
 *
 *     classical memory;
 *     accelerator memory;
 *     distributed memory;
 *     persistent memory;
 *     hybrid quantum/classical computation;
 *     HDL/hardware intent;
 *     resource/capability declarations.
 *
 * ============================================================================
 * 65. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no language-level constants for:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_AMPLITUDES
 *     MAX_STATE_SIZE
 *     MAX_MEMORY
 *     MAX_SNAPSHOTS
 *     MAX_CHECKPOINTS
 *     MAX_ALLOCATIONS
 *     MAX_REGIONS
 *     MAX_DEVICES
 *     MAX_QPUS
 *     MAX_GPUS
 *     MAX_NODES
 *     MAX_PARTITIONS
 *
 * It also contains no universal physical identifiers:
 *
 *     q0
 *     q1
 *     gpu0
 *     qpu0
 *     node0
 *     memory_bank0
 *
 * Such names may occur as ordinary source identifiers where valid, but this
 * grammar does not assign them universal hardware meaning.
 *
 * ============================================================================
 * 66. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It composes through `Memory`.
 * [ ] It introduces no competing memory grammar.
 * [ ] It introduces no competing quantum grammar.
 * [ ] It introduces no lexer rules.
 * [ ] It uses the repository's canonical token vocabulary.
 * [ ] It does not require new reserved quantum-memory keywords.
 * [ ] It preserves open-world semantic operation names.
 * [ ] It supports symbolic resource quantities.
 * [ ] It supports arbitrary qualified-name depth.
 * [ ] It supports arbitrary argument cardinality.
 * [ ] It does not encode physical qubit identity.
 * [ ] It does not encode hardware topology.
 * [ ] It does not encode machine-size limits.
 * [ ] It does not encode provider-specific APIs.
 * [ ] It does not introduce a second quantum IR.
 * [ ] It does not redefine QubitId.
 * [ ] It preserves view/copy/projection/partial-trace distinctions.
 * [ ] It integrates with `src/quantum/memory/`.
 * [ ] It integrates with the canonical frontend AST.
 * [ ] It integrates with resource/capability analysis.
 * [ ] It integrates with `quantum::ir`.
 * [ ] It remains independent of routing.
 * [ ] It remains independent of scheduling.
 * [ ] It remains independent of QEC.
 * [ ] It remains independent of ZQN.
 * [ ] It remains independent of HAL.
 * [ ] It remains deterministic.
 * [ ] It requires no unsafe Rust.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] Compatibility tests exist.
 *
 * ============================================================================
 * 67. REQUIRED COMPOSITION INTEGRATION
 * ============================================================================
 *
 * The parser-composition layer should import this grammar through:
 *
 *     Memory
 *          |
 *          +--> QuantumMemory
 *
 * or through the memory-domain dispatcher selected by the repository's final
 * composition architecture.
 *
 * The universal root remains:
 *
 *     grammar/Zamani.g4
 *
 * and:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * remains the parser composition authority.
 *
 * This file must NOT be imported directly by `Zamani.g4`.
 *
 * ============================================================================
 * 68. REQUIRED DOWNSTREAM INTEGRATION
 * ============================================================================
 *
 * The following existing components are the intended consumers:
 *
 *     grammar/memory/memory.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/persistence.g4
 *     grammar/memory/snapshot/checkpoint contracts
 *     grammar/memory/accelerator-memory.g4
 *     grammar/resources/*
 *     grammar/hardware/*
 *     grammar/quantum/*
 *     grammar/hybrid/*
 *     grammar/compile/*
 *     grammar/execution/*
 *
 * Runtime/compiler consumers include:
 *
 *     src/quantum/memory/types.rs
 *     src/quantum/memory/budget.rs
 *     src/quantum/memory/lifetime.rs
 *     src/quantum/memory/layout.rs
 *     src/quantum/memory/slice.rs
 *     src/quantum/memory/view.rs
 *     src/quantum/memory/snapshot.rs
 *     src/quantum/memory/checkpoint.rs
 *     src/quantum/memory/persistence.rs
 *     src/quantum/memory/migration.rs
 *     src/quantum/memory/allocator.rs
 *     src/quantum/memory/reservation.rs
 *     src/quantum/memory/limits.rs
 *     src/quantum/ir/
 *
 * These files own implementation/semantic behavior.
 *
 * ============================================================================
 * 69. FINAL INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 * One Zamani language.
 *
 * INVARIANT 2
 * ----------
 * One canonical memory grammar foundation.
 *
 * INVARIANT 3
 * ----------
 * One canonical quantum semantic boundary: `quantum::ir`.
 *
 * INVARIANT 4
 * ----------
 * Quantum-memory resource identities do not replace quantum::ir identities.
 *
 * INVARIANT 5
 * ----------
 * No physical hardware limit becomes a language limit.
 *
 * INVARIANT 6
 * ----------
 * Requirements, capabilities, constraints, preferences, and hints remain
 * distinct.
 *
 * INVARIANT 7
 * ----------
 * View, copy, projection, and partial trace remain semantically distinct.
 *
 * INVARIANT 8
 * ----------
 * Parsing is independent of runtime/hardware state.
 *
 * INVARIANT 9
 * ----------
 * Future quantum-memory operations can be added through semantic registration
 * without expanding the universal keyword set.
 *
 * INVARIANT 10
 * -----------
 * Backend realization cannot redefine source meaning.
 *
 * INVARIANT 11
 * -----------
 * No unsafe Rust is required by the grammar or its intended frontend.
 *
 * INVARIANT 12
 * -----------
 * The grammar scales with source semantics and available resources rather than
 * with hard-coded machine dimensions.
 *
 * ============================================================================
 * END OF FILE CONTRACT
 * ============================================================================
 */